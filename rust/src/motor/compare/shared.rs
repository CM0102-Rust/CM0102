use super::*;

// The comparers of the routines in ../shared.rs.
// [pt] Os comparadores das rotinas de ../shared.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `abrir_no_conteiner`.
pub(crate) unsafe extern "C" fn compare_abrir_no_conteiner(path: usize, name: *const u8, mode: u32, a4: u32, kind: u32) -> usize {
    original_fn::<unsafe extern "C" fn(usize, *const u8, u32, u32, u32) -> usize>(IDX_ABRIR_NO_CONTEINER)(path, name, mode, a4, kind)
}

/// Comparer for `buscar_no_conteiner`.
pub(crate) unsafe extern "C" fn compare_buscar_no_conteiner(z: usize, desloc: i32, source: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, u32) -> i32>(IDX_BUSCAR_NO_CONTEINER)(z, desloc, source)
}

/// Comparer for `posicao_no_conteiner`.
pub(crate) unsafe extern "C" fn compare_posicao_no_conteiner(z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_POSICAO_NO_CONTEINER)(z)
}

/// Comparer for `table_a`.
pub(crate) unsafe extern "thiscall" fn compare_table_a(this: usize, j: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u16>(IDX_TABLE_A)(this, j);
    let n = table_a(this, j);
    SCOREBOARD.record(r == n, || format!("table_a(pos {}, lado {}) original={} rust={}", read_i8(j + match_player::POSITION), read_i8(j + match_player::SIDE), r, n));
    r
}

/// Comparer for `table_b`.
pub(crate) unsafe extern "thiscall" fn compare_table_b(this: usize, j: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u16>(IDX_TABLE_B)(this, j);
    let n = table_b(this, j);
    SCOREBOARD.record(r == n, || format!("table_b(pos {}, lado {}) original={} rust={}", read_i8(j + match_player::POSITION), read_i8(j + match_player::SIDE), r, n));
    r
}

/// Comparer for `diferenca`.
pub(crate) unsafe extern "thiscall" fn compare_diferenca(this: usize, l: u8, v: u8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u8, u8) -> u8>(IDX_SIDE_DIFFERENCE)(this, l, v);
    let n = side_difference(this, l, v);
    SCOREBOARD.record(r == n, || format!("side_difference({}, {}) original={} rust={}", l, v, r, n));
    r
}

/// Comparer for `band`.
pub(crate) unsafe extern "stdcall" fn compare_band(l: u8, b: i8, c: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u8, i8, i8) -> u8>(IDX_BAND)(l, b, c);
    let n = band(l, b, c);
    SCOREBOARD.record(r == n, || format!("band({}, {}, {}) original={} rust={}", l, b, c, r, n));
    r
}

/// Comparer for `by_difference`.
pub(crate) unsafe extern "thiscall" fn compare_by_difference(this: usize, a: i8, b: i8, c: i8, d: i8) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16>(IDX_BY_DIFFERENCE)(this, a, b, c, d);
    let n = by_difference(this, a, b, c, d);
    SCOREBOARD.record(r == n, || format!("by_difference({}, {}, {}, {}) original={} rust={}", a, b, c, d, r, n));
    r
}

/// Comparer for `limit`.
pub(crate) unsafe extern "stdcall" fn compare_limit(l: u8, v: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u8, i8) -> u8>(IDX_LIMIT)(l, v);
    let n = limit(l, v);
    SCOREBOARD.record(r == n, || format!("limit({}, {}) original={} rust={}", l, v, r, n));
    r
}

/// Comparer for `condicao`.
pub(crate) unsafe extern "thiscall" fn compare_condicao(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_PLAYER_CONDITION)(this);
    let n = player_condition(this);
    SCOREBOARD.record(r == n, || format!("player_condition(lado {}, v {}) original={} rust={}", read_i8(this + match_player::SIDE), read_i8(this + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `player_table`.
pub(crate) unsafe extern "thiscall" fn compare_player_table(j: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(IDX_PLAYER_TABLE)(j);
    let n = player_table(j);
    SCOREBOARD.record(r == n, || format!("player_table(motor {:#x}) original={} rust={}", *((j + match_player::ENGINE) as *const usize), r, n));
    r
}

/// Comparer for `player_difference`.
pub(crate) unsafe extern "thiscall" fn compare_player_difference(this: usize, j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_PLAYER_DIFFERENCE)(this, j);
    let n = player_difference(this, j);
    SCOREBOARD.record(r == n, || format!("player_difference(lado {}, v {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `side_byte`.
pub(crate) unsafe extern "thiscall" fn compare_side_byte(this: usize, j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_SIDE_BYTE)(this, j);
    let n = side_byte(this, j);
    SCOREBOARD.record(r == n, || format!("side_byte(lado {}) original={} rust={}", side_of_player(j), r, n));
    r
}

/// Comparer for `player_limit`.
pub(crate) unsafe extern "thiscall" fn compare_player_limit(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_PLAYER_LIMIT)(this);
    let n = player_limit(this);
    SCOREBOARD.record(r == n, || format!("player_limit(lado {}, v {}) original={} rust={}", side_of_player(this), read_i8(this + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `player_band`.
pub(crate) unsafe extern "thiscall" fn compare_player_band(this: usize, l: u8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> u32>(IDX_PLAYER_BAND)(this, l);
    let n = player_band(this, l);
    SCOREBOARD.record(r == n, || format!("player_band({}, {}, {}) original={} rust={}", l, read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `position_byte`.
pub(crate) unsafe extern "thiscall" fn compare_position_byte(this: usize, i: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_POSITION_BYTE)(this, i);
    let n = position_byte(this, i);
    SCOREBOARD.record(r == n, || format!("position_byte({}) original={} rust={}", i, r, n));
    r
}

/// Comparer for `field_by_position`.
pub(crate) unsafe extern "thiscall" fn compare_field_by_position(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_FIELD_BY_POSITION)(this);
    let n = field_by_position(this);
    SCOREBOARD.record(r == n, || format!("field_by_position(lado {}, pos {}) original={} rust={}", side_of_player(this), read_i8(this + match_player::POSITION), r, n));
    r
}

/// Comparer for `can_choose`.
pub(crate) unsafe extern "thiscall" fn compare_can_choose(this: usize, j: usize, b: i8, c: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u32>(IDX_CAN_CHOOSE)(this, j, b, c);
    let n = can_choose(this, j, b, c);
    SCOREBOARD.record(r == n, || format!("can_choose({}, {}) original={} rust={}", b, c, r, n));
    r
}

/// Comparer for `aim_set_facing_direction`.
///
/// [pt] Sorteia: original e Rust partem do mesmo estado do gerador e o jogo segue com o da original.
pub(crate) unsafe extern "thiscall" fn compare_aim_set_facing_direction(this: usize, a: i8, b: i8, c: i8, d: i8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8)>(IDX_AIM_SET_FACING_DIRECTION);
    let mut regions = vec![(this + match_player::FACING, 2)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || f(this, a, b, c, d),
        || crate::rng::without_counting(|| aim_set_facing_direction(this, a, b, c, d)),
    );
    SCOREBOARD.record(igual, || format!("aim_set_facing_direction({}, {}, {}, {}) original={:x?} rust={:x?}", a, b, c, d, mo, mr));
}

/// Comparer for `tire_fatigue`.
pub(crate) unsafe extern "thiscall" fn compare_tire_fatigue(this: usize, x: f32, code: i16) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, f32, i16)>(IDX_TIRE_FATIGUE);
    let regions = [(this + match_player::STAMINA, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || f(this, x, code), || tire_fatigue(this, x, code));
    SCOREBOARD.record(igual, || format!("tire_fatigue({}, {}, 0xd5={}) original={:x?} rust={:x?}", x, code, read_f32(this + match_player::STAMINA_ATTRIBUTE), mo, mr));
}

/// Comparer for `facing`.
pub(crate) unsafe extern "thiscall" fn compare_facing(this: usize, a: i8, b: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8) -> u8>(IDX_FACING)(this, a, b);
    let n = facing(this, a, b);
    SCOREBOARD.record(r == n, || format!("facing({}, {}, dir {}, 0x10c {}) original={} rust={}", a, b, *((this + match_player::FACING) as *const i16), read_i8(this + match_player::MOMENT_AGILITY), r, n));
    r
}

/// Comparer for `respond`.
pub(crate) unsafe fn compare_respond(p: usize, a: u32, b: u32, c: u32, d: u32, rodar: &dyn Fn() -> u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = compare_com_fita(reg, || rodar() as u8, |chamar| {
        contest_ball_injected(p, a as i16, b as i8, f32::from_bits(c), d as u8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia && r == rr, || format!("respond(tipo {}, forca {}) original={} rust={} {} chamadas: {}", b as i8, a as i16, r, rr, k, tape_detail()));
    r
}

/// Comparer for `procurar`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_procurar(this: usize, side: i8, group: usize, a3: u32, a4: u32, a5: u32, a6: u32, mode: i8) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8>(IDX_SEARCH_ON_SIDE);
    // The Rust copy must see the state the original saw: 0x6f5c30 changes the receiving player
    // (0x2b, 0x19) and may remove it from the list, so the list, the count and these bytes of each player
    // go into the regions (the tape records them after each call).
    let list = (group as isize + side as isize * 44) as *const usize;
    let mut regions = vec![(this + engine::SIDE_PLAYED_BALL, 2), ((group as isize + side as isize) as usize + 0x58, 1), (list as usize, 44)];
    for i in 0..read_i8((group as isize + side as isize) as usize + 0x58).clamp(0, 11) as usize {
        let p = *list.add(i);
        regions.extend([(p + match_player::POSITION, 1), (p + match_player::BUSY, 1)]);
    }
    let write = RECORD_6F5C30.swap(true, Relaxed);
    let (r, n, igual, sequencia, k) = compare_com_fita(regions, || f(this, side, group, a3, a4, a5, a6, mode), |chamar| {
        search_cell_side_with(this, side, group, a3, a4, a5, a6, mode, &mut |p, w, x, y, z| {
            chamar(FN_MATCH_PLAYER_TRY_INTERCEPT_BALL, p, [w, x, y, z, 0], [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0]) as u8
        })
    });
    RECORD_6F5C30.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia && r == n, || {
        format!("search_on_side(lado {}, modo {}, {} chamadas) original={} rust={}: {}", side, mode, k, r, n, tape_detail())
    });
    r
}

/// Comparer for `reaches`.
pub(crate) unsafe extern "stdcall" fn compare_reaches(j: usize, b: i8, c: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, i8, i8) -> u8>(IDX_REACHES)(j, b, c);
    let n = reaches(j, b, c);
    SCOREBOARD.record(r == n, || format!("reaches(lado {}, x {}, y {}, {}, {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y), b, c, r, n));
    r
}

/// Comparer for `classificar`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_classificar(this: usize, code: i16, a: i8, b: i8, c: i8, d: i8, strength: f32, out: *mut u16) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i16, i8, i8, i8, i8, f32, *mut u16) -> u8>(IDX_CLASSIFY_MOVE);
    let (r, n, igual, mo, mr) =
        crate::comparison::side_by_side(&[(out as usize, 2)], || f(this, code, a, b, c, d, strength, out), || classify_move(this, code, a, b, c, d, strength, out));
    SCOREBOARD.record(igual && r == n, || format!("classify_move({}, {} {} {} {}, {}) original={} rust={} {:x?} {:x?}", code, a, b, c, d, strength, r, n, mo, mr));
    r
}

/// Comparer for `distance_to_point`.
pub(crate) unsafe extern "thiscall" fn compare_distance_to_point(this: usize, a: i8, b: i8) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8) -> f32>(IDX_DISTANCE_TO_POINT)(this, a, b);
    let n = distance_to_point(this, a, b);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("distance_to_point({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `distance_to_goal`.
pub(crate) unsafe extern "thiscall" fn compare_distance_to_goal(this: usize) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> f32>(IDX_DISTANCE_TO_GOAL)(this);
    let n = distance_to_goal(this);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("distance_to_goal(lado {}) original={} rust={}", side_of_player(this), r, n));
    r
}

/// Comparer for `mais_perto`.
pub(crate) unsafe extern "thiscall" fn compare_mais_perto(this: usize, j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_CLOSEST_TO_GOAL)(this, j);
    let n = closest_to_goal(this, j);
    SCOREBOARD.record(r == n, || format!("closest_to_goal(lado {}) original={} rust={}", side_of_player(j), r, n));
    r
}

/// Comparer for `na_linha`.
pub(crate) unsafe extern "thiscall" fn compare_na_linha(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(IDX_PLAYERS_IN_LINE)(this, x1, y1, x2, y2);
    let n = players_in_line(this, x1, y1, x2, y2);
    SCOREBOARD.record(r == n, || format!("players_in_line({}, {}) -> ({}, {}) original={} rust={}", x1, y1, x2, y2, r, n));
    r
}

/// Comparer for `consegue`.
///
/// [pt] Sorteia e grava `saida`: mesmo estado do gerador para as duas, o jogo segue com o da original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_consegue(this: usize, a: i8, b: i8, calcular: i8, strength: f32, adjust: f32, out: *mut u8) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8>(IDX_CAN_PLAY);
    let mut regions: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    if !out.is_null() {
        regions.push((out as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || f(this, a, b, calcular, strength, adjust, out),
        || crate::rng::without_counting(|| can_play(this, a, b, calcular, strength, adjust, out)),
    );
    SCOREBOARD.record(igual && r == n, || format!("can_play({}, {}, {}, {}, {}) original={} rust={} {:x?} {:x?}", a, b, calcular, strength, adjust, r, n, mo, mr));
    r
}

/// Comparer for `in_the_box`.
pub(crate) unsafe extern "thiscall" fn compare_in_the_box(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_IN_THE_BOX)(this);
    let n = in_the_box(this);
    SCOREBOARD.record(r == n, || format!("in_the_box(lado {}, x {}, y {}) original={} rust={}", side_of_player(this), read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `marcas`.
pub(crate) unsafe extern "thiscall" fn compare_marcas(this: usize, position: i8) -> u64 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u64>(IDX_POSITION_MARKS)(this, position);
    let n = position_marks(this, position);
    SCOREBOARD.record(r == n, || format!("position_marks({}) original={:#x} rust={:#x}", position, r, n));
    r
}

/// Comparer for `papel_ataque`.
pub(crate) unsafe extern "thiscall" fn compare_papel_ataque(j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_ATTACKING_ROLE)(j);
    let n = attacking_role(j);
    SCOREBOARD.record(r == n, || format!("attacking_role(lado {}, pos {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::POSITION), r, n));
    r
}

/// Comparer for `papel_defesa`.
pub(crate) unsafe extern "thiscall" fn compare_papel_defesa(j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_DEFENDING_ROLE)(j);
    let n = defending_role(j);
    SCOREBOARD.record(r == n, || format!("defending_role(lado {}, pos {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::POSITION), r, n));
    r
}

/// Comparer for `position_place_players`.
pub(crate) unsafe extern "thiscall" fn compare_position_place_players(this: usize, j: usize, sx: *mut i8, sy: *mut i8, a4: u32) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32)>(IDX_POSITION_PLACE_PLAYERS);
    let regions = [(sx as usize, 1), (sy as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || f(this, j, sx, sy, a4), || position_place_players(this, j, sx, sy, a4));
    SCOREBOARD.record(igual, || format!("position_place_players(lado {}, pos {}, bola {} {}) original={:x?} rust={:x?}", side_of_player(j), read_i8(j + match_player::POSITION), read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y), mo, mr));
}

/// Comparer for `linhas`.
pub(crate) unsafe extern "thiscall" fn compare_linhas(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DEFENSIVE_LINES);
    let regions = [(this + engine::OFFSIDE_LINE, 2)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || f(this), || defensive_lines(this));
    SCOREBOARD.record(igual, || format!("defensive_lines original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `atualizar`.
///
/// [pt] Sorteia e grava (0x2b, 0x1a2, 0x35): mesmo estado do gerador, o jogo segue com o da original.
pub(crate) unsafe extern "thiscall" fn compare_atualizar(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_PLAYER);
    let mut regions = vec![(this + match_player::BUSY, 1), (this + match_player::CONFIDENCE, 4), (this + match_player::MATCH_RATING, 2)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) =
        crate::comparison::side_by_side(&regions, || f(this), || crate::rng::without_counting(|| update_player(this)));
    SCOREBOARD.record(igual, || format!("update_player(papel {:#x}) original={:x?} rust={:x?}", engine_player_role(*((this + match_player::ENGINE) as *const usize), this), mo, mr));
}

/// Comparer for `momento`.
pub(crate) unsafe extern "thiscall" fn compare_momento(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DRAW_MOMENT);
    let mut regions = vec![(this + match_player::MOMENT_PACE, 10)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || f(this), || crate::rng::without_counting(|| draw_moment(this)));
    SCOREBOARD.record(igual, || format!("draw_moment original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `posicao_um`.
pub(crate) unsafe extern "thiscall" fn compare_posicao_um(this: usize, l: i8) -> usize {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> usize>(IDX_HOLD_POSITION_ONE);
    let mut regions = vec![(this + engine::GOALKEEPER, 8)];
    for k in 0..40 {
        regions.push((this + engine::PLAYERS + 8920 * (k / 20) + (k % 20) * 0x1be + 0x19, 2));
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&regions, || f(this, l), || hold_position_one(this, l));
    SCOREBOARD.record(igual && r == n, || format!("hold_position_one({}) original={:#x} rust={:#x} memoria igual={}", l, r, n, igual));
    r
}

/// Comparer for `caminho`.
pub(crate) unsafe extern "thiscall" fn compare_caminho(this: usize, j: usize, ig: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> u8>(IDX_CLEAR_PATH)(this, j, ig);
    let n = clear_path(this, j, ig);
    SCOREBOARD.record(r == n, || format!("clear_path(lado {}, {} {}, {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y), ig, r, n));
    r
}

/// Comparer for `espaco`.
pub(crate) unsafe extern "thiscall" fn compare_espaco(this: usize, j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_SPACE_AHEAD)(this, j);
    let n = space_ahead(this, j);
    SCOREBOARD.record(r == n, || format!("space_ahead(lado {}, {} {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `off_centre`.
pub(crate) unsafe extern "stdcall" fn compare_off_centre(v: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(i8) -> u8>(IDX_OFF_CENTRE)(v);
    let n = off_centre(v);
    SCOREBOARD.record(r == n, || format!("off_centre({}) original={} rust={}", v, r, n));
    r
}

/// Comparer for `alem`.
pub(crate) unsafe extern "thiscall" fn compare_alem(this: usize, y: i8, l: i8, sg: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8>(IDX_BEYOND_THE_LINE)(this, y, l, sg);
    let n = beyond_the_line(this, y, l, sg);
    SCOREBOARD.record(r == n, || format!("beyond_the_line({}, {}, {}) original={} rust={}", y, l, sg, r, n));
    r
}

/// Comparer for `distance_between`.
pub(crate) unsafe extern "stdcall" fn compare_distance_between(a: i8, b: i8, c: i8, d: i8) -> f32 {
    let r = original_fn::<unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32>(IDX_DISTANCE_BETWEEN)(a, b, c, d);
    let n = distance_between(a, b, c, d);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("distance_between({} {} {} {}) original={} rust={}", a, b, c, d, r, n));
    r
}

/// Comparer for `read_8f7a`.
pub(crate) unsafe extern "thiscall" fn compare_read_8f7a(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_READ_8F7A)(this);
    let n = read_8f7a(this);
    SCOREBOARD.record(r == n, || format!("read_8f7a original={:#x} rust={:#x}", r, n));
    r
}

/// Comparer for `posse`.
pub(crate) unsafe extern "thiscall" fn compare_posse(this: usize, v: i8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_RECORD_POSSESSION);
    let regions = [(this + engine::LAST_POSSESSION_SIDE, 1), ((this as isize + 0x8d46 + v as isize) as usize, 1), (this + engine::POSSESSION_COUNTER, 2)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || f(this, v), || record_possession(this, v));
    SCOREBOARD.record(igual, || format!("record_possession({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `in_list`.
pub(crate) unsafe extern "thiscall" fn compare_in_list(this: usize, target: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IN_LIST)(this, target);
    let n = in_list(this, target);
    SCOREBOARD.record(r == n, || format!("in_list(n {}) original={} rust={}", read_i8(this + match_player::MARKER_COUNT), r, n));
    r
}

/// Comparer for `extremos`.
pub(crate) unsafe extern "thiscall" fn compare_extremos(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_STORE_EXTREMES);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + engine::LOWEST_Y_PLAYER, 16)], || f(this), || store_extremes(this));
    SCOREBOARD.record(igual, || format!("store_extremes original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `count_active`.
pub(crate) unsafe extern "thiscall" fn compare_count_active(this: usize, l: i8) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_COUNT_ACTIVE);
    let reg = [((this as isize + 4 + l as isize) as usize, 1)];
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || f(this, l), || count_active(this, l));
    SCOREBOARD.record(igual && r == n, || format!("count_active({}) original={} rust={}", l, r, n));
    r
}

/// Comparer for `special_code`.
pub(crate) unsafe extern "stdcall" fn compare_special_code(c: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(i8) -> u8>(IDX_SPECIAL_CODE)(c);
    let n = special_code(c);
    SCOREBOARD.record(r == n, || format!("special_code({}) original={} rust={}", c, r, n));
    r
}

/// Comparer for `atras`.
pub(crate) unsafe extern "thiscall" fn compare_atras(this: usize, j: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_BEHIND_THE_EXTREME)(this, j);
    let n = behind_the_extreme(this, j);
    SCOREBOARD.record(r == n, || format!("behind_the_extreme(lado {}, y {}) original={} rust={}", side_of_player(j), read_i8(j + match_player::CELL_Y), r, n));
    r
}

/// Comparer for `no_match`.
pub(crate) unsafe extern "thiscall" fn compare_no_match(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_NO_MATCH)(this);
    let n = no_match(this);
    SCOREBOARD.record(r == n, || format!("no_match original={} rust={}", r, n));
    r
}

/// Comparer for `pressao_area`.
pub(crate) unsafe extern "thiscall" fn compare_pressao_area(this: usize, l: i8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_PRESSURE_IN_BOX);
    let mut reg = vec![((this as isize + 6 + l as isize) as usize, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || f(this, l), || crate::rng::without_counting(|| pressure_in_box(this, l)));
    SCOREBOARD.record(igual, || format!("pressure_in_box({}) original={:x?} rust={:x?}", l, mo, mr));
}

/// Comparer for `mais_proximo`.
pub(crate) unsafe extern "thiscall" fn compare_mais_proximo(this: usize, j: usize, a: i8, b: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u8>(IDX_NEAREST_TO_POINT)(this, j, a, b);
    let n = nearest_to_point(this, j, a, b);
    SCOREBOARD.record(r == n, || format!("nearest_to_point({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `pressao_ponto`.
pub(crate) unsafe extern "thiscall" fn compare_pressao_ponto(this: usize, l: i8, a: i8, b: i8, t: i8) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(IDX_PRESSURE_AT_POINT)(this, l, a, b, t);
    let n = pressure_at_point(this, l, a, b, t);
    SCOREBOARD.record(r == n, || format!("pressure_at_point({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}

/// Comparer for `smallest_distance`.
pub(crate) unsafe extern "thiscall" fn compare_smallest_distance(this: usize, l: i8, a: i8, b: i8, t: i8) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> f32>(IDX_SMALLEST_DISTANCE)(this, l, a, b, t);
    let n = smallest_distance(this, l, a, b, t);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("smallest_distance({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}

/// Comparer for `escolher`.
pub(crate) unsafe extern "thiscall" fn compare_escolher(this: usize, arg: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(IDX_CHOOSE_BY_ROLE);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || f(this, arg), || crate::rng::without_counting(|| choose_by_role(this, arg)));
    SCOREBOARD.record(igual && r == n, || format!("choose_by_role({:#x}) original={} rust={}", arg, r as i8, n as i8));
    r
}
