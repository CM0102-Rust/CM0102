use super::*;

/// Looks through side `lado`'s players in grid cell group `grupo`: in mode 2 it only counts the able ones; otherwise it
/// asks each one to try to intercept the ball and, at the first success, clears engine +0x1a5/+0x1a6 and returns 1.
/// Returns the count (mode 2) or 0.
///
/// [pt] { 0x6cd330 } (match_eng.cpp): percorre os jogadores do lado `lado` no grupo `grupo` (quantidade
/// em grupo[0x58 + lado], ponteiros em grupo + lado x 44). Para cada um ativo (0x2b <= 1, posicao
/// >= 0): com `modo` 2 so conta; senao pergunta a 0x6f5c30 (pelo endereco do jogo) e, ao primeiro
/// sim, zera 0x1a5/0x1a6 deste objeto e devolve 1. Devolve a contagem (so cresce no modo 2).
pub(crate) unsafe extern "thiscall" fn search_on_side(this: usize, side: i8, group: usize, a3: u32, a4: u32, a5: u32, a6: u32, mode: i8) -> u8 {
    search_cell_side_with(this, side, group, a3, a4, a5, a6, mode, &mut |p, w, x, y, z| exe::match_player_try_intercept_ball(p, w, x, y, z))
}

/// search_on_side with an injected callback: walks the side's player list in a grid cell group (+0x58 count); mode 2
/// only counts able players, else calls f on each until one succeeds, then clears engine +0x1a5/+0x1a6 and returns 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn search_cell_side_with(
    this: usize, side: i8, group: usize, a3: u32, a4: u32, a5: u32, a6: u32, mode: i8,
    f: &mut dyn FnMut(usize, u32, u32, u32, u32) -> u8,
) -> u8 {
    let list = (group as isize + side as isize * 44) as *const usize;
    let mut count = 0u8;
    let mut i = 0;
    while i < read_i8((group as isize + side as isize) as usize + 0x58) as i32 {
        let p = *list.add(i as usize);
        if read_i8(p + match_player::BUSY) <= 1 && read_i8(p + match_player::POSITION) >= 0 {
            if mode == 2 {
                count = count.wrapping_add(1);
            } else if f(p, a4, a5, a3, a6) != 0 {
                *((this + 0x1a6) as *mut u8) = 0;
                *((this + engine::SIDE_PLAYED_BALL) as *mut u8) = 0;
                return 1;
            }
        }
        i += 1;
    }
    count
}

/// Is player `j` in position to reach cell (b, c) inside the opponents' penalty area (by his side)? Coordinate-band
/// checks and finally the distance table value (12 x 9 floats at 0xa15e44; 1000 outside) must exceed 1.5. Returns 1 or
/// 0.
///
/// [pt] { 0x6cb9b0 } (match_eng.cpp): o jogador (pelo lado dele) esta em posicao de alcancar (b, c)?
/// Faixas de coordenadas e, no fim, o valor da tabela de distancias (float, 12 x 9 em 0xa15e44;
/// fora dela, 1000) tem de passar de 1,5.
pub(crate) unsafe extern "stdcall" fn reaches(j: usize, b: i8, c: i8) -> u8 {
    let s = side_of_player(j);
    let band_c = if s == 0 { 10..=11 } else { 0..=1 };
    if !(2..=6).contains(&b) || !band_c.contains(&c) {
        return 0;
    }
    let x = read_i8(j + match_player::CELL_X);
    if x == 4 {
        return 0;
    }
    let y = read_i8(j + match_player::CELL_Y);
    let middle = x < 6 && x > 2;
    if s == 0 {
        if (middle && c < y) || y < 9 {
            return 0;
        }
    } else if (middle && c > y) || y > 2 {
        return 0;
    }
    if b <= 2 || b >= 6 || (x as i32 - b as i32).abs() <= 1 {
        return 0;
    }
    let col = (b as i32 - x as i32).abs() as i8;
    let row = (c as i32 - y as i32).abs() as i8;
    let v = if (0..=8).contains(&col) && (0..=11).contains(&row) {
        read_f32(G_CELL_DISTANCE_TABLE + 4 * (row as usize * 9 + col as usize))
    } else {
        read_f32(rdata::F32_1000)
    };
    (v as f64 > read_f64(rdata::F64_1_5)) as u8
}

/// Codigos de lance que `exe::match_player_choose_kicking_foot` ignora (devolve 2).
pub(crate) const IGNORED_MOVES: [i16; 11] = [0x5b, 0x5c, 0x3d, 0x36, 0x34, 0x37, 0x5a, 0x33, 0x35, 0x39, 3];

/// Classifies a move by the angle between the player's facing direction (+0x19a) and the target cell: within the
/// tolerance 90 - (18 x `forca` - +0xe5) it returns the player's byte +0x2c; outside it returns 0 or 1 by side and
/// writes into `saida` a penalty of (20 - attribute) x 12 using the person's attribute +0x30 or +0x3b. Some move codes
/// are ignored and return 2.
///
/// [pt] { 0x713e90 } (match_pl.cpp): classifica um lance pelo angulo entre a direcao do jogador (0x19a)
/// e o alvo (0x6cb6c0 do motor, pelo endereco do jogo). Dentro da tolerancia
/// 90 - (18 x `forca` - float de 0xe5): devolve o byte 0x2c. Fora: 0 (um lado) ou 1 (outro), e
/// grava em `saida` (20 - atributo da pessoa) x 12 (0x30 ou 0x3b do registro).
pub(crate) unsafe extern "thiscall" fn classify_move(this: usize, code: i16, a: i8, b: i8, c: i8, d: i8, strength: f32, out: *mut u16) -> u8 {
    *out = 0;
    if IGNORED_MOVES.contains(&code) {
        return 2;
    }
    let dx = exe::cm3_match_engine_get_direction_between_cells(*((this + match_player::ENGINE) as *const usize), a, b, c, d).wrapping_sub(*((this + match_player::FACING) as *const u16)) as i16;
    let sign: i16 = if dx >= 0 { 1 } else { -1 };
    let mut ang = sign.wrapping_mul(dx);
    if ang > 0xb4 {
        ang = (0x168i32 - ang as i32) as i16;
    }
    let assinado = sign.wrapping_mul(ang);
    let tol = ftol(read_f32(rdata::F32_90) as f64 - (strength as f64 * read_f32(rdata::F32_18) as f64 - read_f32(this + match_player::TECHNIQUE) as f64));
    let (lim, menos) = (tol as i16, (tol.wrapping_neg()) as i16);
    let person = *((this + match_player::PLAYER) as *const usize);
    let (r, attr) = if assinado > lim {
        (0, read_i8(person + player_sheet::LEFT_FOOT))
    } else if assinado < menos {
        (1, read_i8(person + player_sheet::RIGHT_FOOT))
    } else {
        return read_i8(this + match_player::PREFERRED_FOOT) as u8;
    };
    *out = ((0x14i16.wrapping_sub(attr as i16)).wrapping_mul(12)) as u16;
    r
}

/// Counts the players of the attacking side in grid cell (x, y) through the exe's search_on_side
/// (FN_CM3_MATCH_ENGINE_TRY_CELL_INTERCEPTION) in mode 2.
///
/// [pt] Jogadores numa casa (x, y) da grade do motor, pelo `search_on_side` em modo 2 (so conta),
/// chamado pelo endereco do jogo. O 3o argumento e um float (bits), que o modo 2 nao usa.
pub(crate) unsafe fn count_players_in_cell(engine: usize, x: i8, y: i8, value: f32) -> u8 {
    let group = (engine as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
    let side = (read_i8(engine + engine::POSSESSION_SIDE) == 0) as i8;
    exe::cm3_match_engine_try_cell_interception(engine, side, group, value.to_bits(), u32::MAX, u32::MAX, u32::MAX, 2)
}

/// Distance from the 12x9 float table at 0xa15e44 for (|dx|, |dy|); 1000.0 outside the table.
///
/// [pt] Distancia da tabela para (|dx|, |dy|) dentro dela (coluna 0..=8, linha 0..=11); fora, 1000.
pub(crate) unsafe fn grid_distance_or_1000(dx: i32, dy: i32) -> f32 {
    let (col, lin) = (dx.abs() as i8, dy.abs() as i8);
    if (0..=8).contains(&col) && (0..=11).contains(&lin) {
        read_f32(G_CELL_DISTANCE_TABLE + 4 * (lin as usize * 9 + col as usize))
    } else {
        f32::from_bits(0x447a0000)
    }
}

/// Counts the players standing on the grid cells along the line from (x1, y1) to (x2, y2) (Bresenham-style walk on the
/// 9 x 12 grid, stopping at the first cell off the pitch). Returns the count as a byte.
///
/// [pt] { 0x6cbc00 } (match_eng.cpp): quantos jogadores ha nas casas da linha de (x1, y1) a (x2, y2),
/// percorrida ao estilo de Bresenham. Grade de 9 x 12 casas; a contagem para na primeira casa fora.
/// Vertical/horizontal: do menor ao maior, sem a ultima casa. Diagonais: as duas pontas incluidas,
/// a partir da de menor x (rasa) ou menor y (ingreme). Contagem em byte (soma com estouro).
pub(crate) unsafe extern "thiscall" fn players_in_line(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    walk_line_cells(x1, y1, x2, y2, false, 8, &|x, y, v| count_players_in_cell(this, x, y, v))
}

/// Match engine: counts the opponents that may block a shot along the line of cells (x1, y1)-(x2, y2), both ends
/// included on straight lines (cell check `exe::cm3_match_engine_try_cell_shot_block` in mode 2). Returns the count.
///
/// [pt] { 0x6cc070 } (match_eng.cpp): como 0x6cbc00, contando pela 0x6cd3d0 (modo 2) e com as duas
/// pontas nas linhas retas (na horizontal, x ate 11).
pub(crate) unsafe extern "thiscall" fn players_on_path(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let side = (read_i8(this + engine::POSSESSION_SIDE) == 0) as i8;
    walk_line_cells(x1, y1, x2, y2, true, 11, &|x, y, v| {
        let group = (this as isize + 2 * 45 * (y as isize + 12 * x as isize) + 0x215e) as usize;
        exe::cm3_match_engine_try_cell_shot_block(this, side, group, v.to_bits(), 0, 2, 6, 0)
    })
}

/// Walks the grid cells on the line (x1,y1)-(x2,y2) as `exe::cm3_match_engine_count_interceptors_on_line`/0x6cc070 do (straight lines and Bresenham diagonals),
/// summing `contar` for each cell; stops at the pitch edge.
///
/// [pt] O percurso de 0x6cbc00 e 0x6cc070: `ate_o_fim` inclui a ultima casa nas linhas retas e a
/// horizontal aceita x ate `x_horizontal`; `contar` da os jogadores de cada casa.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn walk_line_cells(x1: i8, y1: i8, x2: i8, y2: i8, until_end: bool, x_horizontal: i8, count: &dyn Fn(i8, i8, f32) -> u8) -> u8 {
    let dx = (x2 as i32 - x1 as i32).abs() as i8;
    let dy = (y2 as i32 - y1 as i32).abs() as i8;
    let away = |x: i8, y: i8| !(0..=8).contains(&x) || !(0..=11).contains(&y);
    let mut n = 0u8;
    if dx == 0 {
        let (inicio, fim) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        let mut y = inicio;
        while y < fim || (until_end && y == fim) {
            if away(x1, y) {
                return n;
            }
            n = n.wrapping_add(count(x1, y, (y as i32 - inicio as i32) as f32));
            y = y.wrapping_add(1);
        }
        return n;
    }
    if dy == 0 {
        let (inicio, fim) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let mut x = inicio;
        while x < fim || (until_end && x == fim) {
            if !(0..=x_horizontal).contains(&x) || !(0..=11).contains(&y1) {
                return n;
            }
            n = n.wrapping_add(count(x, y1, (x as i32 - inicio as i32) as f32));
            x = x.wrapping_add(1);
        }
        return n;
    }
    // diagonals: the points swap when the main axis comes reversed
    let ingreme = dx < dy;
    let swap = if ingreme { y1 > y2 } else { x1 > x2 };
    let (xa, ya, xb, yb) = if swap { (x2, y2, x1, y1) } else { (x1, y1, x2, y2) };
    let mut error: i8 = 0;
    if !ingreme {
        let step_towards: i8 = if yb as i32 - ya as i32 >= 0 { 1 } else { -1 };
        let mut y = ya;
        let mut x = xa;
        loop {
            if error >= dx {
                error = error.wrapping_sub(dx);
                y = y.wrapping_add(step_towards);
            }
            if away(x, y) {
                return n;
            }
            n = n.wrapping_add(count(x, y, grid_distance_or_1000(x as i32 - xa as i32, y as i32 - ya as i32)));
            error = error.wrapping_add(dy);
            x = x.wrapping_add(1);
            if x > xb {
                return n;
            }
        }
    }
    let step_towards: i8 = if xb as i32 - xa as i32 >= 0 { 1 } else { -1 };
    let mut x = xa;
    let mut y = ya;
    loop {
        if error >= dy {
            error = error.wrapping_sub(dy);
            x = x.wrapping_add(step_towards);
        }
        if away(x, y) {
            return n;
        }
        n = n.wrapping_add(count(x, y, grid_distance_or_1000(x as i32 - xa as i32, y as i32 - ya as i32)));
        error = error.wrapping_add(dx);
        y = y.wrapping_add(1);
        if y > yb {
            return n;
        }
    }
}

/// Can the match player play the ball to cell (a, b) with his current facing (+0x19a)? 1 if already facing the target
/// exactly; otherwise the angle must be within a tolerance (75, or with `calcular` one computed from his skill +0x10b,
/// `forca`, distance and `ajuste`, clamped to 30..=135). With `saida`, writes how many opponents are on the line and
/// returns 1 only if +0x10b beats random(count^2).
///
/// [pt] { 0x6f9f90 } (match_pl.cpp): o jogador consegue jogar a bola para (a, b) com a direcao atual?
/// 1 se ja esta virado exatamente para o alvo. Senao, tolerancia de angulo: 75 (sem `calcular`) ou
/// ftol(10 x 0x10b ... ) - ftol(...), limitada a 30..=135. Fora dela, 0. Com `saida`, grava quantos
/// jogadores ha na linha (0x6cbc00) e sorteia: 1 se 0x10b >= sorteio(c x c).
pub(crate) unsafe extern "thiscall" fn can_play(this: usize, a: i8, b: i8, calcular: i8, strength: f32, adjust: f32, out: *mut u8) -> u8 {
    let target = *((GLOBAL_TABLE as isize + 2 * (a.wrapping_sub(read_i8(this + match_player::CELL_X)) as isize * 24 + b.wrapping_sub(read_i8(this + match_player::CELL_Y)) as isize)) as *const i16);
    let dir = *((this + match_player::FACING) as *const i16);
    if dir == target {
        return 1;
    }
    let tolerancia: i16 = if calcular == 0 {
        0x4b
    } else {
        let f: unsafe extern "stdcall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_CAN_RUN_INTO_CENTRAL_BOX);
        let v = read_i8(this + match_player::MOMENT_VISION) as i32;
        let (base, k) = if f(this, a, b) != 0 { (v * 4, 0x96d7b0) } else { (v * 5, 0x96d664) };
        let t1 = ftol((base as f64 + strength as f64 * read_f32(rdata::F32_10) as f64) + read_f32(k) as f64);
        let d = distance_table_cell(read_i8(this + match_player::CELL_Y) as i32 - b as i32, read_i8(this + match_player::CELL_X) as i32 - a as i32);
        let t2 = ftol((d as f64 - adjust as f64) * read_f32(rdata::F32_20) as f64 - read_f32(rdata::F32_50) as f64);
        let t = t1.wrapping_sub(t2) as i16;
        t.clamp(0x1e, 0x87)
    };
    let mut ang = (dir as i32 - target as i32).abs();
    if ang as i16 > 0xb4 {
        ang = 0x168 - ang;
    }
    if ang as i16 >= tolerancia {
        return 0;
    }
    if out.is_null() {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_INTERCEPTORS_ON_LINE);
    let c = f(*((this + match_player::ENGINE) as *const usize), read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y), a, b);
    *out = c;
    let c = c as i8 as i32;
    (read_i8(this + match_player::MOMENT_VISION) as i32 >= random(c * c)) as u8
}

/// goal line and columns 3..5; otherwise `exe::match_player_is_cell_in_view` facing the middle of the goal, with no calculation and no output
/// (does not draw)
/// Original: `in_the_box` (0x6f4650, thiscall, `al`).
///
/// [pt] { 0x6f4650 } (match_pl.cpp): na entrada da area (linha do gol, colunas 3..=5)? Senao,
/// 0x6f9f90 (pelo endereco) virado para o meio do gol, sem calculo nem saida (nao sorteia).
pub(crate) unsafe extern "thiscall" fn in_the_box(this: usize) -> u8 {
    let row: i8 = if side_of_player(this) == 0 { 0xb } else { 0 };
    if read_i8(this + match_player::CELL_Y) == row && (3..=5).contains(&read_i8(this + match_player::CELL_X)) {
        return 1;
    }
    exe::match_player_is_cell_in_view(this, 4, row, 0, 0.0, 0.0, core::ptr::null_mut())
}

/// the moment's attributes (0x104 to 0x10d), several "best of two draws"; 15 draws in the original's order
/// Original: `draw_moment` (0x6f44a0, thiscall).
///
/// [pt] { 0x6f44a0 } (match_pl.cpp): atributos do momento, sorteados no inicio do lance. Varios sao
/// "o melhor de dois sorteios". A ordem dos 15 sorteios e a do original.
pub(crate) unsafe extern "thiscall" fn draw_moment(this: usize) {
    let grava = |o: usize, v: i32| *((this + o) as *mut u8) = v as u8;
    let best_of_two = |n: i32| {
        let a = random(n);
        let b = random(n);
        if b > a { b } else { a }
    };
    let base = ftol(*((this + match_player::STAMINA) as *const f32) as f64 * read_f32(rdata::F32_0_002499999944) as f64) as i16 as i32;
    grava(0x104, best_of_two(base + (read_i8(this + match_player::BASE_PACE) as i32) - 0x19));
    grava(0x10a, best_of_two((read_i8(this + match_player::BASE_POSITIONING) as i32)));
    grava(0x107, random((read_i8(this + match_player::BASE_MOVEMENT) as i32)));
    grava(0x108, random((read_i8(this + match_player::BASE_MARKING) as i32)));
    grava(0x109, random((read_i8(this + match_player::BASE_FLAIR) as i32)));
    grava(0x10b, best_of_two((read_i8(this + match_player::BASE_VISION) as i32)));
    grava(0x106, best_of_two((read_i8(this + match_player::BASE_DECISIONS) as i32)));
    grava(0x105, random((read_i8(this + match_player::BASE_ANTICIPATION) as i32)));
    grava(0x10d, random((read_i8(this + match_player::BASE_WORK_RATE) as i32)));
    grava(0x10c, random((read_i8(this + match_player::BASE_AGILITY) as i32)));
}

/// Reads the u16 position-table entry [engine + 0x8ebc + 4 x side][position] for player p, without range check.
///
/// [pt] `table_a` direta: [motor + 0x8ebc + 4 x lado do jogador][posicao] (sem a checagem de faixa).
pub(crate) unsafe fn position_table_entry(engine: usize, p: usize, position: i8) -> u16 {
    let t = *((engine as isize + 0x8ebc + 4 * side_of_player(p) as isize) as *const usize) as *const u16;
    *t.offset(position as isize)
}

/// Match engine: keeps track of each side's player in position 1 (the goalkeeper) at `engine::GOALKEEPER` / `engine::GOALKEEPER_SIDE1`: drops one who
/// left the pitch, otherwise finds the first player whose position role is 1, or else promotes the first active player,
/// swapping with the reserve who held position 1. Returns the one for side `lado_pedido`.
///
/// [pt] { 0x6ce110 } (match_eng.cpp): mantem o jogador de posicao 1 de cada lado (0xf59e: bloco 0;
/// 0xf5a2: bloco 1). Perdido (posicao < 0) zera; sem ninguem, o primeiro com `tabela` 1 na posicao;
/// se ainda nao houver, promove o primeiro ativo, trocando com quem guardava a posicao 1 em
/// reserva (0x1a, com posicao -2 e 0x20 = -1). Devolve o do lado pedido.
pub(crate) unsafe extern "thiscall" fn hold_position_one(this: usize, side_request: i8) -> usize {
    for (campo, bloco) in [(0xf59eusize, 0x4796usize), (0xf5a2, 0x6a6e)] {
        let stored = (this + campo) as *mut usize;
        if *stored != 0 && read_i8(*stored + 0x19) < 0 {
            *stored = 0;
        }
        if *stored != 0 {
            continue;
        }
        let mut reserve = 0usize;
        for k in 0..20 {
            let p = this + bloco + k * 0x1be;
            let pos = read_i8(p + match_player::POSITION);
            if *stored == 0 && (0..=10).contains(&pos) && position_table_entry(this, p, pos) == 1 {
                *stored = p;
            }
            let r = read_i8(p + match_player::PREVIOUS_POSITION);
            if (0..=10).contains(&r) && position_table_entry(this, p, r) == 1 && read_i8(p + match_player::POSITION) == -2 && read_i8(p + match_player::MINUTE_OFF) == -1 {
                reserve = p;
            }
        }
        if *stored != 0 {
            continue;
        }
        let Some(p) = (0..20).map(|k| this + bloco + k * 0x1be).find(|&p| read_i8(p + match_player::POSITION) >= 0) else { continue };
        let mut new = 0i8;
        if reserve != 0 {
            new = read_i8(reserve + match_player::PREVIOUS_POSITION);
            *((reserve + match_player::PREVIOUS_POSITION) as *mut i8) = read_i8(p + match_player::POSITION);
        }
        *stored = p;
        if position_table_entry(this, p, new) == 1 {
            *((p + match_player::POSITION) as *mut i8) = new;
        }
    }
    *((this + if side_request == 1 { 0xf5a2 } else { 0xf59e }) as *const usize)
}

/// Match engine: tells whether player `j` has a clear path forward: 0 if his cell is marked by the opponents (unless
/// `ignorar_casa`) or an active opposing outfield player ahead of him is in the same column; otherwise 1.
///
/// [pt] { 0x6bd5b0 } (match_eng.cpp): caminho livre para avancar? 0 se a casa dele estiver marcada (sem
/// `ignorar_casa`) ou se algum adversario ativo a frente estiver na mesma coluna; senao 1.
pub(crate) unsafe extern "thiscall" fn clear_path(this: usize, j: usize, ignore_home: i8) -> u8 {
    let s = side_of_player(j);
    let other = (s == 0) as isize;
    let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    if ignore_home == 0 && *((this as isize + other + 90 * (y as isize + 12 * x as isize) + 0x21b6) as *const u8) != 0 {
        return 0;
    }
    for k in 0..20 {
        let p = (this as isize + 0x4796 + 8920 * other) as usize + k * 0x1be;
        if read_i8(p + match_player::POSITION) < 0 || exe::match_player_get_position(p) == 1 {
            continue;
        }
        let front = if s == 1 { y > read_i8(p + match_player::CELL_Y) } else { s == 0 && y < read_i8(p + match_player::CELL_Y) };
        let px = read_i8(p + match_player::CELL_X) as i32;
        if front && x as i32 + 1 > px && (x as i32 - 1) < px {
            return 0;
        }
    }
    1
}

/// Returns 1 if there is free space in the row ahead of player `j` (towards his attack), checking the opposing side's
/// occupancy grids: diagonal x + 1, then x - 1, then straight ahead; 0 when the row ahead is off the pitch or all
/// taken.
///
/// [pt] { 0x6cbad0 } (match_eng.cpp): ha espaco livre na casa a frente (diagonais primeiro)? Fora do
/// campo, 0. Casas marcadas na grade do lado oposto: x + 1 (se x < 8), x - 1 (se x > 0), x.
pub(crate) unsafe extern "thiscall" fn space_ahead(this: usize, j: usize) -> u8 {
    let x = read_i8(j + match_player::CELL_X);
    let other = (side_of_player(j) == 0) as isize;
    let y = if side_of_player(j) == 1 {
        let y = read_i8(j + match_player::CELL_Y).wrapping_sub(1);
        if y < 0 {
            return 0;
        }
        y
    } else {
        let y = read_i8(j + match_player::CELL_Y).wrapping_add(1);
        if y > 0xb {
            return 0;
        }
        y
    };
    let home = |base: isize| *((this as isize + other + 90 * (y as isize + 12 * x as isize) + base) as *const u8);
    if x < 8 && home(0x25ee) == 0 {
        return 1;
    }
    if x > 0 && home(0x1d7e) == 0 {
        return 1;
    }
    (home(0x21b6) == 0) as u8
}

/// Returns 1 if column `v` is outside 2..=6 (outside the penalty-area width).
///
/// [pt] { 0x6cb910 } (match_eng.cpp): 1 fora da faixa 2..=6.
pub(crate) unsafe extern "stdcall" fn off_centre(v: i8) -> u8 {
    !(2..=6).contains(&v) as u8
}

/// Counts the active players of side `lado_b` beyond grid row `y` (side 1: row lower than y; side 0: higher; other
/// sides: none), optionally skipping the player stored at engine `engine::BALL_HOLDER`. Returns the count.
///
/// [pt] { 0x6cb700 } (match_eng.cpp): quantos jogadores ativos do `lado` estao alem da linha `y` (lado
/// 1: y menor; lado 0: y maior; outros lados: nenhum), opcionalmente sem o guardado em 0xf582.
pub(crate) unsafe extern "thiscall" fn beyond_the_line(this: usize, y: i8, side_b: i8, without_stored: i8) -> u8 {
    let mut n = 0u8;
    for k in 0..20 {
        let p = (this as isize + 0x4796 + 8920 * side_b as isize) as usize + k * 0x1be;
        if read_i8(p + match_player::POSITION) < 0 || (without_stored != 0 && p == *((this + engine::BALL_HOLDER) as *const usize)) {
            continue;
        }
        let py = read_i8(p + match_player::CELL_Y);
        if (side_b == 1 && py < y) || (side_b == 0 && py > y) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// Grid distance between cells (a, b) and (c, d) from the distance table; 1000 when the offset falls outside the table.
///
/// [pt] { 0x6cc4e0 } (match_eng.cpp): distancia (tabela) entre (a, b) e (c, d); fora da tabela, 1000.
pub(crate) unsafe extern "stdcall" fn distance_between(a: i8, b: i8, c: i8, d: i8) -> f32 {
    grid_distance_or_1000(a as i32 - c as i32, b as i32 - d as i32)
}

/// Address of player slot k (0x1be bytes each) in the side's 20-player block at engine + 0x4796 + 8920 x side.
///
/// [pt] Bloco de 20 jogadores do lado no motor (0x4796 + 8920 x lado), de 0x1be em 0x1be.
pub(crate) fn side_player_slot(engine: usize, side_b: i8, k: usize) -> usize {
    (engine as isize + 0x4796 + 8920 * side_b as isize) as usize + k * 0x1be
}

/// Calls MATCH_PLAYER::get_position (FN_MATCH_PLAYER_GET_POSITION) for player p.
pub(crate) unsafe fn match_player_position(p: usize) -> u16 {
    exe::match_player_get_position(p)
}

/// Calls MATCH_PLAYER distance_to_cell (FN_MATCH_PLAYER_DISTANCE_TO_CELL) for player p and cell (a, b).
pub(crate) unsafe fn match_player_distance_to_cell(p: usize, a: i8, b: i8) -> f32 {
    exe::match_player_distance_to_cell(p, a, b)
}

/// Counts the side's active players (position +0x19 >= 0) among its 20, stores the count in engine byte +4 + side and
/// returns it.
///
/// [pt] { 0x6cbbb0 } (match_eng.cpp): quantos ativos (posicao >= 0) o lado tem; guarda em motor[4 + lado].
pub(crate) unsafe extern "thiscall" fn count_active(this: usize, side_b: i8) -> u8 {
    let n = (0..20).filter(|&k| read_i8(side_player_slot(this, side_b, k) + 0x19) >= 0).count() as u8;
    *((this as isize + 4 + side_b as isize) as *mut u8) = n;
    n
}

/// Returns 1 for certain match-event codes (37, 42..=50, 96, 97), following the original switch tables read from the
/// exe, else 0.
///
/// [pt] { 0x6cb650 } (match_eng.cpp): 1 para alguns codigos de lance (37, 42..=50, 96, 97), pelo switch
/// do original (tabelas lidas do exe: indices em 0x6cb680, destinos em 0x6cb678).
pub(crate) unsafe extern "stdcall" fn special_code(code: i8) -> u8 {
    let i = code as i32 - 0x25;
    if !(0..=0x3c).contains(&i) {
        return 0;
    }
    let case = *((0x6cb680 + i as usize) as *const u8) as usize;
    (*((0x6cb678 + 4 * case) as *const u32) == 0x6cb66c) as u8
}

/// Is player `j` behind the stored extreme player of his side (`this` = match engine)? Side 0: his y (+0x103) <= the y
/// of the stored lowest player (`engine::LOWEST_Y_PLAYER`; 12 without one); side 1: y >= the y of the stored highest (`engine::LOWEST_Y_PLAYER_SIDE1`; -1 without
/// one). Returns 1 or 0.
///
/// [pt] { 0x6cd5f0 } (match_eng.cpp): o jogador esta atras do extremo guardado do lado dele?
/// Lado 0: y <= y do menor (0xf58e; sem ele, 12). Outros: y >= y do maior (0xf592; sem ele, -1).
pub(crate) unsafe extern "thiscall" fn behind_the_extreme(this: usize, j: usize) -> u8 {
    let y = read_i8(j + match_player::CELL_Y);
    if side_of_player(j) == 0 {
        let p = *((this + engine::LOWEST_Y_PLAYER) as *const usize);
        (y <= if p != 0 { read_i8(p + match_player::CELL_Y) } else { 0xc }) as u8
    } else {
        let p = *((this + engine::LOWEST_Y_PLAYER_SIDE1) as *const usize);
        (y >= if p != 0 { read_i8(p + match_player::CELL_Y) } else { -1 }) as u8
    }
}

/// `*[0x4792] == −2`
/// Original: `no_match` (0x6d4280, thiscall, `al`).
///
/// [pt] { 0x6d4280 } (match_eng.cpp): o ponteiro em 0x4792 aponta para -2?
pub(crate) unsafe extern "thiscall" fn no_match(this: usize) -> u8 {
    (**((this + engine::MATCH) as *const *const i32) == -2) as u8
}

/// Match engine: decides whether side `lado_b`'s penalty area is under pressure: sums the opponents in the six rows
/// nearest its goal (closer rows weigh 6..1) and sets engine flag +6 + side when the sum exceeds random(15) + 30.
///
/// [pt] { 0x6bd6b0 } (match_eng.cpp): pressao sobre a area do lado. Soma os adversarios nas linhas
/// perto do gol (grade de contagem, lado oposto), pesando mais as mais proximas (6..1), e marca
/// motor[6 + lado] se a soma (16 bits) passar de sorteio(15) + 30.
pub(crate) unsafe extern "thiscall" fn pressure_in_box(this: usize, side_b: i8) {
    let mark = (this as isize + 6 + side_b as isize) as *mut u8;
    *mark = 0;
    let mut sum: i32 = 0;
    let cell = |base: usize, column: usize| *((base + column * 0x438) as *const i8) as i16 as i32;
    if side_b == 0 {
        for linha in (0..=5i32).rev() {
            let base = this + 0x2379 - (5 - linha) as usize * 0x5a;
            for c in 0..9 {
                sum = sum.wrapping_add((cell(base, c) * (6 - linha)) as i16 as i32);
            }
        }
    } else {
        for linha in 6..12i32 {
            let base = this + 0x23d2 + (linha - 6) as usize * 0x5a;
            for c in 0..9 {
                sum = sum.wrapping_add((cell(base, c) * (linha - 5)) as i16 as i32);
            }
        }
    }
    let limit = random(0xf) + 0x1e;
    if sum as i16 as i32 > limit {
        *mark = 1;
    }
}

/// Is player `j` the closest of his available team-mates (active, not himself, +0x2b = 0) to cell (a, b)? Returns 0 if
/// anyone is strictly closer (or equally close when `j` is the goalkeeper), 1 otherwise; an outfield player already on
/// the cell returns 1 at once.
///
/// [pt] { 0x6c3010 } (match_eng.cpp): o jogador e o mais proximo de (a, b) entre os colegas disponiveis
/// (ativos, diferentes dele, 0x2b = 0)? 0 se algum estiver estritamente mais perto (ou na mesma
/// distancia, quando ele tem `tabela` 1). Na propria casa (distancia 0) e sem `tabela` 1: 1 direto.
pub(crate) unsafe extern "thiscall" fn nearest_to_point(this: usize, j: usize, a: i8, b: i8) -> u8 {
    let d = match_player_distance_to_cell(j, a, b);
    let zero = read_f64(rdata::F64_0);
    if (d as f64 == zero || (d as f64).is_nan()) && match_player_position(j) != 1 {
        return 1;
    }
    let s = side_of_player(j);
    for k in 0..20 {
        let p = side_player_slot(this, s, k);
        if read_i8(p + match_player::POSITION) < 0 || p == j || read_i8(p + match_player::BUSY) != 0 {
            continue;
        }
        let dp = match_player_distance_to_cell(p, a, b);
        if !(dp >= d) {
            return 0; // C0: less than or unordered
        }
        if match_player_position(j) == 1 && (dp == d || dp.is_nan()) {
            return 0;
        }
    }
    1
}

/// Pressure of side `lado_b` on cell (a, b): each active outfield player (only free ones, +0x2b = 0, unless `todos`)
/// adds 3 when on the cell and 1 when within distance 1.5. Returns the total.
///
/// [pt] { 0x6c3a00 } (match_eng.cpp): pressao no ponto (a, b): para cada jogador ativo do lado (sem
/// `tabela` 1; 0x2b = 0 salvo `todos`), +3 na propria casa (distancia 0), +1 a distancia <= 1,5.
pub(crate) unsafe extern "thiscall" fn pressure_at_point(this: usize, side_b: i8, a: i8, b: i8, all: i8) -> u8 {
    let mut n = 0u8;
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) < 0 || match_player_position(p) == 1 || (all == 0 && read_i8(p + match_player::BUSY) != 0) {
            continue;
        }
        let d = match_player_distance_to_cell(p, a, b) as f64;
        if d == read_f64(rdata::F64_0) || d.is_nan() {
            n = n.wrapping_add(3);
        } else if !(d > read_f64(rdata::F64_1_5)) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// Match engine: the smallest distance from cell (a, b) to an active outfield player of side `lado_b` (skipping those
/// with +0x2b set unless `todos`), starting from 5000; 0 if one of them is on that cell.
///
/// [pt] { 0x6c3950 } (match_eng.cpp): menor distancia ao ponto (a, b) entre os jogadores do lado (bit 0
/// de `tabela` zerado; 0x2b = 0 salvo `todos`), partindo de 5000; 0 se algum estiver na casa.
pub(crate) unsafe extern "thiscall" fn smallest_distance(this: usize, side_b: i8, a: i8, b: i8, all: i8) -> f32 {
    let mut smaller = f32::from_bits(0x459c4000);
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) < 0 || match_player_position(p) & 1 != 0 || (all == 0 && read_i8(p + match_player::BUSY) != 0) {
            continue;
        }
        let d = match_player_distance_to_cell(p, a, b);
        if d as f64 == read_f64(rdata::F64_0) || d.is_nan() {
            return read_f32(rdata::F32_0);
        }
        if d < smaller {
            smaller = d;
        }
    }
    smaller
}

/// O que cada bit do papel faz com o delta: nada (pula o jogador), soma ftol(delta x fator) ou soma o delta.
#[derive(Clone, Copy)]
pub(crate) enum RoleBitEffect {
    Pula,
    Fator(usize),
    Inteiro,
    /// delta + delta (fadd st0, st0)
    Dobro,
}

/// Adds a share of `delta` to the disposition word (+0x35) of each active player of a side, chosen by the first role
/// bit of the player's position (Skip, Factor, Whole, Double).
///
/// [pt] Soma a cada jogador ativo do lado uma parte de `delta` na palavra 0x35 (disposicao), pelo
/// primeiro bit do papel (`tabela`, ordem 1, 2, 4, 8, 0x10, 0x20, 0x40); sem bit, nada.
pub(crate) unsafe fn spread_disposition_delta(this: usize, side_b: i8, delta: i16, partes: [RoleBitEffect; 7]) {
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let role = match_player_position(p) as u8;
        let Some(i) = (0..7).find(|&i| role & (1 << i) != 0) else { continue };
        let w = (p + match_player::MATCH_RATING) as *mut i16;
        match partes[i] {
            RoleBitEffect::Pula => {}
            RoleBitEffect::Fator(k) => *w = (*w).wrapping_add(ftol(delta as f64 * read_f64(k)) as i16),
            RoleBitEffect::Inteiro => *w = (*w).wrapping_add(delta),
            RoleBitEffect::Dobro => *w = (*w).wrapping_add(ftol(delta as f64 + delta as f64) as i16),
        }
    }
}

/// Spreads a disposition change `delta` over side `lado_b`'s active players (word +0x35), with a share factor per role
/// from the first table; the goalkeeper is skipped.
///
/// [pt] { 0x6c06b0 } (match_eng.cpp): reparte a disposicao (papel 1 fica de fora).
pub(crate) unsafe extern "thiscall" fn split_a(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    spread_disposition_delta(this, side_b, delta, [Pula, Fator(0x968918), Fator(0x968f20), Fator(0x96d328), Fator(0x969040), Fator(0x968978), Fator(0x968f88)]);
}

/// Match engine: spreads `delta` over the disposition (+0x35) of side `lado_b`'s active players, with a factor per role
/// (the role 4 players get the whole delta).
///
/// [pt] { 0x6c07a0 } (match_eng.cpp): reparte a disposicao (papel 4 recebe o delta inteiro).
pub(crate) unsafe extern "thiscall" fn split_b(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    spread_disposition_delta(this, side_b, delta, [Fator(0x96d330), Fator(0x9689b8), Inteiro, Fator(0x969520), Fator(0x968e00), Fator(0x968f20), Fator(0x967888)]);
}

/// Spreads a disposition change `delta` over the side's active players (word +0x35) using the third table of role
/// factors.
///
/// [pt] { 0x6c0a70 } (match_eng.cpp): reparte a disposicao (terceira tabela de fatores).
pub(crate) unsafe extern "thiscall" fn split_c(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    spread_disposition_delta(this, side_b, delta, [Fator(0x96d330), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x968968), Fator(0x969510), Fator(0x968918)]);
}

/// Match engine: picks a player of side `argumento` (index 0..19, -1 if none) by a draw weighted by role: random(25) x
/// a factor (0.1 for defenders, 0.2, 0.5, 0.75, 1.0 or 1.05 for forwards), skipping goalkeepers and players off the
/// pitch; ties keep the first. Like the original, a player with no role bit reuses the previous factor (the first one
/// comes from the raw argument bits).
///
/// [pt] { 0x6c2750 } (match_eng.cpp): escolhe um jogador do lado (indice 0..19, -1 sem candidato) por
/// sorteio ponderado pelo papel: maior sorteio(25) x fator (0,1 / 0,1 / 0,2 / 0,5 / 0,75 / 1,05 ou
/// 1,0). O original guarda o fator no lugar do argumento na pilha: jogador sem bit de papel usa o
/// fator anterior, e o primeiro, os bits da palavra inteira empilhada pelo chamador (por isso u32).
pub(crate) unsafe extern "thiscall" fn choose_by_role(this: usize, argumento: u32) -> u8 {
    let side_b = argumento as i8;
    let mut factor = f32::from_bits(argumento);
    let mut best = 0.0f32;
    let mut chosen: i8 = -1;
    for k in 0..20u8 {
        let p = side_player_slot(this, side_b, k as usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let role = match_player_position(p);
        if role & 1 != 0 {
            continue;
        }
        let bits: u32 = if role & 2 != 0 || role & 4 != 0 {
            0x3dcccccd
        } else if role & 8 != 0 {
            0x3e4ccccd
        } else if role & 0x10 != 0 {
            0x3f000000
        } else if role & 0x20 != 0 {
            0x3f400000
        } else if role & 0x40 != 0 {
            if role & 0x700 != 0 { 0x3f866666 } else { 0x3f800000 }
        } else {
            factor.to_bits()
        };
        factor = f32::from_bits(bits);
        let v = random(0x19) as f64 * factor as f64;
        if chosen == -1 || v > best as f64 {
            best = v as f32;
            chosen = k as i8;
        }
    }
    chosen as u8
}

/// Spreads a disposition change `delta` over the active players of side `lado_b` by role (goalkeeper skipped; fourth
/// factor table).
///
/// [pt] { 0x6c0890 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quarta tabela de fatores).
pub(crate) unsafe extern "thiscall" fn split_d(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    let f = Fator(0x969040);
    spread_disposition_delta(this, side_b, delta, [Pula, f, f, f, f, f, Fator(0x968940)]);
}

/// Spreads a disposition change `delta` over side `lado_b`'s active players (word +0x35), with a share factor per role
/// from the fifth table; the goalkeeper is skipped.
///
/// [pt] { 0x6c0980 } (match_eng.cpp): reparte a disposicao (papel 1 de fora; quinta tabela).
pub(crate) unsafe extern "thiscall" fn split_e(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    spread_disposition_delta(this, side_b, delta, [Pula, Fator(0x967880), Fator(0x967880), Fator(0x96d340), Fator(0x96d338), Fator(0x9689d0), Fator(0x968940)]);
}

/// Match engine: spreads `delta` over the disposition (+0x35) of side `lado_b`'s active players, with a factor per role
/// (the role 1 players get double).
///
/// [pt] { 0x6c0b70 } (match_eng.cpp): reparte a disposicao (papel 1 recebe o dobro; sexta tabela).
pub(crate) unsafe extern "thiscall" fn split_f(this: usize, side_b: i8, delta: i16) {
    use RoleBitEffect::*;
    spread_disposition_delta(this, side_b, delta, [Dobro, Fator(0x9689e0), Fator(0x968918), Fator(0x968f20), Fator(0x967888), Fator(0x968a98), Fator(0x968a98)]);
}

/// Match engine: records the current move (code, player, other player and six flag bytes at `engine::PENDING_PLAY_KIND`..`engine::PENDING_PLAY_SERIOUS`) unless a
/// move with a player is already recorded.
///
/// [pt] { 0x6bb9a0 } (match_eng.cpp): registra o lance (0x475a..=0x4769) se nao houver jogador nele.
pub(crate) unsafe extern "thiscall" fn record_move(this: usize, code: u16, player: u32, other: u32, b4769: u8, b475a: u8, b475b: u8, b475c: u8, b475d: u8, b475e: u8) {
    if *((this + engine::PENDING_PLAY_OFFENDER) as *const u32) != 0 {
        return;
    }
    *((this + engine::PENDING_PLAY_CODE) as *mut u16) = code;
    *((this + engine::PENDING_PLAY_OFFENDER) as *mut u32) = player;
    *((this + engine::PENDING_PLAY_VICTIM) as *mut u32) = other;
    for (o, v) in [(0x4769, b4769), (0x475a, b475a), (0x475b, b475b), (0x475c, b475c), (0x475d, b475d), (0x475e, b475e)] {
        *((this + o) as *mut u8) = v;
    }
}

/// Removes pointer `alvo` from the player's list (+0x1ae, count at +0x101) by moving the last item into its place;
/// returns the new count (the old count if not found, 0 if the list is empty).
///
/// [pt] { 0x714ca0 } (match_pl.cpp): tira o ponteiro da lista (0x1ae, quantidade em 0x101) pondo o
/// ultimo no lugar; devolve a nova quantidade (sem achar, a quantidade; lista vazia, 0).
pub(crate) unsafe extern "thiscall" fn remove_from_list(this: usize, target: usize) -> u8 {
    let n = read_i8(this + match_player::MARKER_COUNT);
    if n <= 0 {
        return 0;
    }
    let item = |k: i8| (this + match_player::MARKERS + 4 * k as usize) as *mut usize;
    for k in 0..n {
        if *item(k) == target {
            *item(k) = *item(n - 1);
            *((this + match_player::MARKER_COUNT) as *mut i8) = n - 1;
            return (n - 1) as u8;
        }
    }
    n as u8
}

/// Puts the player on the pitch at cell (8, 6) and appends him to his side's on-pitch list (`engine::ON_PITCH_PLAYERS`, 11 per side, count
/// at `engine::ON_PITCH_COUNT` + side), storing his order in +0x28; returns the new count.
///
/// [pt] { 0x6cdd00 } (match_eng.cpp): poe o jogador no meio do campo (8, 6) e na lista do lado
/// (0x453a, 11 por lado; quantidade em 0x4592 + lado), guardando a ordem em 0x28.
pub(crate) unsafe extern "thiscall" fn take_the_field(this: usize, player: usize) -> u8 {
    let l = side_of_player(player) as isize;
    *((player + match_player::CELL_X) as *mut i8) = 8;
    *((player + match_player::CELL_Y) as *mut i8) = 6;
    let count = (this as isize + 0x4592 + l) as *mut i8;
    *((this as isize + 0x453a + 4 * (*count as isize + 11 * l)) as *mut usize) = player;
    // reread after writing: the slot may land on the count itself (side outside 0/1)
    *((player + match_player::CELL_SLOT) as *mut i8) = *count;
    let n = (*count).wrapping_add(1);
    *count = n;
    n as u8
}

/// 12 × 9 float table (0xa15e44) ≤ 1.0, otherwise `behind_the_extreme`
/// Original: `in_danger` (0x70d630, thiscall, `al`).
///
/// [pt] { 0x70d630 } (match_pl.cpp): perigo pela tabela de floats 12 x 9 em 0xa15e44 (distancia em y a
/// propria linha de fundo, em x ao centro): valor <= 1,0 (ou NaN) da 1; senao, `behind_the_extreme`.
pub(crate) unsafe extern "thiscall" fn in_danger(this: usize) -> u8 {
    let bottom = if side_of_player(this) != 0 { 11 } else { 0 };
    let dy = (read_i8(this + match_player::CELL_Y) as i32 - bottom).abs() as i8 as isize;
    let dx = (read_i8(this + match_player::CELL_X) as i32 - 4).abs() as i8 as isize;
    let v = read_f32((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    if !(v as f64 > read_f64(rdata::F64_1)) {
        return 1;
    }
    (exe::cm3_match_engine_is_behind_own_back_line(*((this + match_player::ENGINE) as *const usize), this) != 0) as u8
}

/// 1/2 according to the move's side and area; draw(200) decides whether to clear
/// Original: `check_move` (0x6bba80, thiscall, `al`).
///
/// [pt] { 0x6bba80 } (match_eng.cpp): confere o lance registrado. 1 sem lance; 2 se o jogador do lance
/// e do lado em 0x8eae, se o lance (codigo diferente de 8) esta na faixa 2..=6 x area do lado, ou
/// se sorteio(200) der 0; senao limpa o lance e da 1.
pub(crate) unsafe extern "thiscall" fn check_move(this: usize) -> u8 {
    let code = read_i8(this + engine::PENDING_PLAY_KIND);
    let p = *((this + engine::PENDING_PLAY_OFFENDER) as *const usize);
    if code == 0 || p == 0 {
        return 1;
    }
    let side_of_move = read_i8(this + engine::POSSESSION_SIDE);
    if read_i8(p + match_player::SIDE) == side_of_move {
        return 2;
    }
    if code != 8 {
        let area = if side_of_move == 0 { 10..=11 } else { 0..=1 };
        if (2..=6).contains(&read_i8(this + engine::PENDING_PLAY_X)) && area.contains(&read_i8(this + engine::PENDING_PLAY_Y)) {
            return 2;
        }
    }
    if random(200) == 0 {
        return 2;
    }
    clear_move(this);
    1
}

/// Returns 1 if cell (x, y) is in side `lado_b`'s useful area and no active opponent stands on it or, for outfield
/// opponents, in the neighbouring columns between the cell and the goal; else 0.
///
/// [pt] { 0x6c41b0 } (match_eng.cpp): 0 se (x, y) esta fora do campo util do lado (lado 0: y > 8;
/// outros: y < 3) ou se um adversario ativo esta na casa, ou (sem `tabela` 1) na mesma coluna
/// entre o ponto e o gol (lado 1: y maior que o dele; lado 0: menor); senao 1.
pub(crate) unsafe extern "thiscall" fn clear_path_to(this: usize, side_b: i8, x: i8, y: i8) -> u8 {
    if (side_b != 0 && y < 3) || (side_b == 0 && y > 8) {
        return 0;
    }
    for k in 0..20 {
        let p = side_player_slot(this, (side_b == 0) as i8, k);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        if x == read_i8(p + match_player::CELL_X) && y == read_i8(p + match_player::CELL_Y) {
            return 0;
        }
        if match_player_position(p) == 1 {
            continue;
        }
        let alem = match side_b {
            1 => y > read_i8(p + match_player::CELL_Y),
            0 => y < read_i8(p + match_player::CELL_Y),
            _ => false,
        };
        let (px, cx) = (read_i8(p + match_player::CELL_X) as i32, x as i32);
        if alem && cx + 1 > px && cx - 1 < px {
            return 0;
        }
    }
    1
}

/// Can the match player reach cell (x, y)? 1 if he is already on it; otherwise compares the direction from the point to
/// the centre of his goal line with the direction from the point to him (must differ by less than +0x10c + 10) and
/// checks the point's distance to the goal line against the 12 x 9 distance table (0xa15e44).
///
/// [pt] { 0x713d90 } (match_pl.cpp): o jogador alcanca (x, y)? Na propria casa, 1. Senao compara as
/// palavras de `by_difference` do ponto ao centro da linha de fundo e do ponto ao jogador (diferenca
/// abaixo de 0x10c + 10) e a distancia do ponto a linha de fundo com a tabela 12 x 9 (0xa15e44).
pub(crate) unsafe extern "thiscall" fn reaches_the_point(this: usize, x: i8, y: i8) -> u8 {
    let (jx, jy) = (read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y));
    if x == jx && y == jy {
        return 1;
    }
    let bottom: i8 = if side_of_player(this) != 0 { 11 } else { 0 };
    let engine = *((this + match_player::ENGINE) as *const usize);
    let a = exe::cm3_match_engine_get_direction_between_cells(engine, x, y, 4, bottom) as i16 as i32;
    let b = exe::cm3_match_engine_get_direction_between_cells(engine, x, y, jx, jy) as i16 as i32;
    if (a - b).abs() >= read_i8(this + match_player::MOMENT_AGILITY) as i32 + 10 {
        return 0;
    }
    let dx = (jx as i32 - x as i32).abs() as i8 as isize;
    let dy = (jy as i32 - y as i32).abs() as i8 as isize;
    let t = read_f32((0xa15e44 + 4 * (dy * 9 + dx)) as usize);
    (exe::get_cell_distance(x, y, 4, bottom) >= t) as u8
}

/// Chooses the match's standout player: 16-bit score (disposition + 125 × (0x10 + 2 × 0x0c) + 50 × 0x03 + 35 × 0x06)
/// and ftol(disposition × factor); stores the record's id at 0x477a.
///
/// The standout among the first (byte 0x4d of the fixture & 0xf) + 11 of each side (inactive ones, with 0x20 = 0xff,
/// left out): score (16 bits) = disposition 0x35 + 125 × (0x10 + 2 × 0x0c) + 50 × 0x03 + 35 × 0x06; replaces only with
/// a higher score and ftol(disposition × factor) not lower. Stores the first dword of the player sheet (0x69) at
/// 0x477a.
/// Original: `choose_standout` (0x6d0f80, among the (match byte 0x4d & 0xf) + 11 of each side, thiscall).
///
/// [pt] { 0x6d0f80 } (match_eng.cpp): o destaque entre os (byte 0x4d da partida & 0xf) + 11 primeiros
/// de cada lado (inativos com 0x20 = 0xff de fora): pontuacao (16 bits) = disposicao 0x35 +
/// 125 x (0x10 + 2 x 0x0c) + 50 x 0x03 + 35 x 0x06; troca so com pontuacao maior e
/// ftol(disposicao x fator) nao menor. Guarda o primeiro dword da ficha (0x69) em 0x477a.
pub(crate) unsafe extern "thiscall" fn choose_standout(this: usize) {
    let partida = Fixture(*((this + engine::MATCH) as *const usize));
    let n = ((partida.squad_rules() as i8) & 0xf) + 0xb;
    let (mut melhor, mut pontos, mut valor) = (0usize, 0i16, 0i16);
    for l in 0..2usize {
        for k in 0..n.max(0) as usize {
            let p = this + engine::PLAYERS + 0x22d8 * l + 0x1be * k;
            if read_i8(p + match_player::POSITION) < 0 && read_i8(p + match_player::MINUTE_OFF) == -1 {
                continue;
            }
            let disp = *((p + match_player::MATCH_RATING) as *const i16);
            let v = ftol(disp as f64 * read_f64(rdata::F64_0_001)) as i16;
            let u = |o: usize| read_i8(p + o) as u8 as i16;
            let pt = disp
                .wrapping_add(125i16.wrapping_mul(u(0x10).wrapping_add(2 * u(0x0c))))
                .wrapping_add(50i16.wrapping_mul(u(0x03)))
                .wrapping_add(35i16.wrapping_mul(u(0x06)));
            if melhor == 0 || (pt > pontos && v >= valor) {
                melhor = p;
                pontos = pt;
                valor = v;
            }
        }
    }
    if melhor != 0 {
        *((this + engine::ADJUSTED_PLAYER_ID) as *mut u32) = **((melhor + match_player::STAFF) as *const *const u32);
    }
}

/// Picks a player of a side (index 0..19, 0xff if none) by a role-weighted draw: highest random(25) x factor, the
/// factor set by the role bits of his tactical position (1.2/1.2/0.65/0.5/0.35; role 0x40: 0.25 or 0.15) and scaled by
/// work rate (+0x44), positioning (+0x39) and tackling (+0x3e). As in the original, the factor reuses the argument slot
/// and carries over to players without a role bit.
///
/// [pt] { 0x6c2880 } (match_eng.cpp): como `choose_by_role`, com outros fatores (1,2 / 1,2 / 0,65 /
/// 0,5 / 0,35; papel 0x40: 0,25 com 0x700, senao 0,15) ajustados pelos atributos da pessoa (0x6d:
/// 0x44, 0x39 e 0x3e). O fator, inclusive os ajustes, fica no lugar do argumento e passa adiante.
pub(crate) unsafe extern "thiscall" fn choose_by_role_b(this: usize, argumento: u32) -> u8 {
    let side_b = argumento as i8;
    let mut factor = f32::from_bits(argumento);
    let mut best = 0.0f32;
    let mut chosen: u8 = 0xff;
    for k in 0..20u8 {
        let p = side_player_slot(this, side_b, k as usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let role = match_player_position(p);
        if role & 1 != 0 {
            continue;
        }
        let bits = if role & 6 != 0 {
            Some(0x3f99999a)
        } else if role & 8 != 0 {
            Some(0x3f266666)
        } else if role & 0x10 != 0 {
            Some(0x3f000000)
        } else if role & 0x20 != 0 {
            Some(0x3eb33333)
        } else if role & 0x40 != 0 {
            Some(if role & 0x700 != 0 { 0x3e800000 } else { 0x3e19999a })
        } else {
            None
        };
        if let Some(b) = bits {
            factor = f32::from_bits(b);
        }
        let person = *((p + match_player::PLAYER) as *const usize);
        if person != 0 {
            let a = read_i8(person + player_sheet::WORK_RATE);
            if a > 15 {
                factor *= read_f32(rdata::F32_1_100000024);
            } else if a > 10 {
                factor *= read_f32(rdata::F32_1_049999952);
            } else if a < 5 {
                factor *= read_f32(rdata::F32_0_9499999881);
            }
            for o in [0x39, 0x3e] {
                let a = read_i8(person + o);
                if a > 15 {
                    factor *= read_f32(rdata::F32_1_100000024);
                } else if a < 5 {
                    factor *= read_f32(rdata::F32_0_9499999881);
                }
            }
        }
        let v = random(0x19) as f64 * factor as f64;
        if chosen == 0xff || v > best as f64 {
            best = v as f32;
            chosen = k;
        }
    }
    chosen
}

/// Writes into `x`/`y` the formation cell of `jogador` (`this` = match engine), read from his side's formation table
/// (`engine::FORMATION_TABLE`, 216 bytes per position; low and high nibble of one byte), using a different field depending on whether his
/// side has the ball (`engine::POSSESSION_SIDE`), the other side has it, or nobody does.
///
/// [pt] { 0x6c2a40 } (match_eng.cpp): casa (x, y) da formacao do jogador (tabela 0xf56e do lado, 216
/// bytes por posicao; nibble baixo e alto de um byte): campo 0x9f/0xa4 quando o lado em 0x8eae
/// (0 ou 1) e o do jogador; 0x30/0x3b quando e o outro; 0x35 com 0x8eae fora de 0/1. O byte e
/// relido para o segundo nibble, depois de gravar o primeiro.
pub(crate) unsafe extern "thiscall" fn formation_square(this: usize, player: usize, x: *mut u8, y: *mut u8) {
    let l = read_i8(this + engine::POSSESSION_SIDE);
    let jl = read_i8(player + match_player::SIDE);
    let field = match l {
        0 if jl == 0 => 0x9f,
        0 => 0x30,
        1 if jl == 1 => 0xa4,
        1 => 0x3b,
        _ => 0x35,
    };
    let read = || {
        let table = *((this as isize + 0xf56e + 4 * read_i8(player + match_player::SIDE) as isize) as *const isize);
        *((table + 216 * read_i8(player + match_player::POSITION) as isize + field) as *const u8)
    };
    *x = read() & 0xf;
    *y = read() >> 4;
}

/// Writes ftol(a / total x 100) and its complement to 100 into the engine bytes pa and pb; 50/50 when total is not
/// positive.
///
/// [pt] Grava ftol(a / (a + b) x 100) e o complemento a 100 nos bytes `pa` e `pb`; 50 e 50 sem total positivo.
pub(crate) unsafe fn write_percent_pair(this: usize, a: i32, total: i32, pa: isize, pb: isize) {
    let (x, y) = if total > 0 {
        let v = ftol(a as f64 / total as f64 * read_f32(rdata::F32_100) as f64) as u8;
        (v, 100u8.wrapping_sub(v))
    } else {
        (50, 50)
    };
    *((this as isize + pa) as *mut u8) = x;
    *((this as isize + pb) as *mut u8) = y;
}

/// Computes the match percentages (possession and similar stats) for index `i` and the totals 3 and 4, from the
/// counters in the engine's six tables at +0x8f02..+0x8f66, writing each complementary pair of percentages.
///
/// [pt] { 0x6ce9c0 } (match_eng.cpp): percentuais da partida (posse e afins) para os indices `i`, 3 e 4,
/// a partir das contagens em 0x8f02..0x8f66 (seis tabelas de dwords).
pub(crate) unsafe extern "thiscall" fn compute_percentages(this: usize, i: i8) {
    for s in [i as isize, 3, 4] {
        let c = |o: isize| *((this as isize + o + 4 * s) as *const i32);
        write_percent_pair(this, c(0x8f52), c(0x8f52).wrapping_add(c(0x8f16)), 0xf5dd + s, 0xf609 + s);
        write_percent_pair(this, c(0x8f2a), c(0x8f2a).wrapping_add(c(0x8f3e)), 0xf5d8 + s, 0xf60e + s);
        write_percent_pair(this, c(0x8f66), c(0x8f66).wrapping_add(c(0x8f02)), 0xf613 + s, 0xf5d3 + s);
        let a = c(0x8f02).wrapping_add(c(0x8f2a)).wrapping_add(c(0x8f52));
        let total = a.wrapping_add(c(0x8f66)).wrapping_add(c(0x8f3e)).wrapping_add(c(0x8f16));
        write_percent_pair(this, a, total, 0xf5ce + s, 0xf604 + s);
    }
}

/// Third weighted choice of a side's player by role (`argumento` = side byte / float factor, `modo`): for each active
/// outfield player, a role weight (0.4 to 1.1) adjusted by his player data and a roll, times a random score from his
/// float match attributes (+0xc9 with `modo`, else various weighted sums); returns the slot (0..19) of the highest
/// score, 0xff with none, and leaves the factor in the argument.
///
/// [pt] { 0x6c2050 } (match_eng.cpp): terceira escolha ponderada pelo papel (0,4 / 0,45 / 0,5 / 0,75 /
/// 0,85; papel 0x40: 1,1 com 0x700, senao 1,0), ajustada pela pessoa (palavra 5 e sorteio contra o
/// byte 0x21). O sorteio base vem de atributos em float do jogador: com `modo`, 0xc9; senao, 1 em
/// 7, 0xa1 + c; 2 em 3 (dos restantes), uma soma ponderada de cinco; o resto, de tres. Nos dois
/// primeiros o fator nao entra. Acima de um limite, perde sorteio(5). O fator fica no argumento.
pub(crate) unsafe extern "thiscall" fn choose_by_role_c(this: usize, argumento: u32, mode: u32) -> u8 {
    let side_b = argumento as i8;
    let mut factor = f32::from_bits(argumento);
    let mut best = 0.0f32;
    let mut chosen: u8 = 0xff;
    for k in 0..20u8 {
        let p = side_player_slot(this, side_b, k as usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let role = match_player_position(p);
        if role & 1 != 0 {
            continue;
        }
        let bits = if role & 2 != 0 {
            Some(0x3ecccccd)
        } else if role & 4 != 0 {
            Some(0x3ee66666)
        } else if role & 8 != 0 {
            Some(0x3f000000)
        } else if role & 0x10 != 0 {
            Some(0x3f400000)
        } else if role & 0x20 != 0 {
            Some(0x3f59999a)
        } else if role & 0x40 != 0 {
            Some(if role & 0x700 != 0 { 0x3f8ccccd } else { 0x3f800000 })
        } else {
            None
        };
        if let Some(b) = bits {
            factor = f32::from_bits(b);
        }
        let person = *((p + match_player::PLAYER) as *const usize);
        if person != 0 {
            let w = *((person + player_sheet::CURRENT_ABILITY) as *const i16);
            factor = if w > 0x8c {
                factor * read_f32(rdata::F32_1_399999976)
            } else if w < 0x3c {
                factor * read_f32(rdata::F32_0_6000000238)
            } else {
                (w as f64 * read_f64(rdata::F64_0_01) * factor as f64) as f32
            };
            let r = random(read_i8(person + player_sheet::CONSISTENCY) as i32);
            if r < random(0x14) {
                factor *= read_f32(rdata::F32_0_6999999881);
            }
        }
        let f = |o: usize| read_f32(p + o) as f64;
        let mut v = if mode as u8 != 0 {
            random(ftol(f(match_player::PENALTIES))) as f64
        } else if random(7) == 0 {
            random(ftol(f(match_player::FREE_KICKS) + read_f32(rdata::F32_15) as f64)) as f64
        } else if random(3) != 0 {
            let x = f(match_player::FLAIR) * read_f64(rdata::F64_0_1) + f(match_player::DRIBBLING) * read_f64(rdata::F64_0_45) + f(match_player::LONG_SHOTS) * read_f64(rdata::F64_0_35) + f(match_player::ANTICIPATION) * read_f64(rdata::F64_0_35) + f(match_player::FINISHING) + read_f64(rdata::F64_18);
            random(ftol(x)) as f64 * factor as f64
        } else {
            let x = f(match_player::STRENGTH) * read_f64(rdata::F64_0_25) + f(match_player::JUMPING) * read_f64(rdata::F64_0_75) + f(match_player::HEADING) + read_f32(rdata::F32_10) as f64;
            random(ftol(x)) as f64 * factor as f64
        };
        if v > read_f32(rdata::F32_25) as f64 {
            let v32 = v as f32;
            v = v32 as f64 - random(5) as f64;
        }
        if chosen == 0xff || v > best as f64 {
            best = v as f32;
            chosen = k;
        }
    }
    chosen
}

/// Multiplies the match player's condition (+0x1a2, float) by `fator`, with a floor of 2 x (25 - the person byte +0x5d)
/// and set to 100 when the product exceeds a ceiling.
///
/// [pt] { 0x716450 } (match_pl.cpp): multiplica a condicao (0x1a2) pelo fator; o produto (em double) fica
/// no minimo em 2 x (25 - byte 0x5d da ficha 0x69) e, passando de um teto, vira 100.
pub(crate) unsafe extern "thiscall" fn apply_factor(this: usize, factor: f32) {
    let c = (this + match_player::CONFIDENCE) as *mut f32;
    let produto = factor as f64 * *c as f64;
    *c = produto as f32;
    let minimum = ((25 - read_i8(*((this + match_player::STAFF) as *const usize) + 0x5d) as i32) * 2) as f32;
    let v = if produto > minimum as f64 { produto } else { minimum as f64 };
    *c = v as f32;
    if v > read_f64(rdata::F64_100) {
        *c = 100.0;
    }
}

/// Calls MATCH_PLAYER scale_anger (`exe::match_player_scale_anger`, FN_MATCH_PLAYER_SCALE_ANGER) for player p with a factor.
///
/// [pt] { 0x716450 } pelo endereco do jogo (Rust ou original, conforme o modo).
pub(crate) unsafe fn match_player_scale_anger(p: usize, factor: f32) {
    exe::match_player_scale_anger(p, factor)
}

/// Match player: a value from his physical condition (+0x1a2) and byte +0xff; when `alvo` is the player stored at
/// +0x1a6 [inference: the last one who fouled him] it uses its own formula with (byte +0xff + 5). Returns the truncated
/// value.
///
/// [pt] { 0x7165b0 } (match_pl.cpp): valor pela condicao e pelo byte 0xff; com `alvo` igual ao guardado
/// em 0x1a6, formula propria (0xff + 5).
pub(crate) unsafe extern "thiscall" fn value_by_condition(this: usize, target: usize) -> i32 {
    let base = read_f32(this + match_player::CONFIDENCE) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_75);
    let b = read_i8(this + match_player::ANGER) as i32;
    if target != 0 && target == *((this + match_player::LAST_FOULED_BY) as *const usize) {
        ftol(base * (b + 5) as f64 * read_f64(rdata::F64_2_5))
    } else {
        ftol((base * b as f64 + read_f64(rdata::F64_5)) * read_f64(rdata::F64_0_5))
    }
}

/// Match engine: applies factor `fator` to player `jogador` and a derived factor to his team-mates who have a player
/// record (on the pitch: (f + c) x c'; off it: f x c + c') through the match player's anger scaling [inference].
///
/// [pt] { 0x6bb8d0 } (match_eng.cpp): aplica o fator ao jogador e um derivado aos colegas com ficha
/// (ativos: (f + c) x c'; inativos: f x c + c').
pub(crate) unsafe extern "thiscall" fn apply_to_team(this: usize, player: usize, factor: f32) {
    for k in 0..20 {
        let p = side_player_slot(this, side_of_player(player), k);
        if *((p + match_player::STAFF) as *const usize) == 0 {
            continue;
        }
        let f = if p == player {
            factor
        } else if read_i8(p + match_player::POSITION) >= 0 {
            ((factor as f64 + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5)) as f32
        } else {
            (factor as f64 * read_f64(rdata::F64_0_1666666667) + read_f64(rdata::F64_0_83)) as f32
        };
        match_player_scale_anger(p, f);
    }
}

/// Applies the factor to every player of side `lado_b` who has a person record (+0x69), scaling each one's
/// anger/aggression via match_player_scale_anger.
///
/// [pt] { 0x6bb950 } (match_eng.cpp): aplica o fator a todos os jogadores com ficha do lado.
pub(crate) unsafe extern "thiscall" fn apply_to_side(this: usize, side_b: i8, factor: f32) {
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if *((p + match_player::STAFF) as *const usize) != 0 {
            match_player_scale_anger(p, factor);
        }
    }
}

/// Picks a player of side `lado_b`: each active player draws a random number up to his foul-chance value + 5 (+25 in
/// mode 2), weighted 0.2 for the goalkeeper and 1.0 otherwise; the highest wins. Returns the player's slot (0..19) or
/// 0xff.
///
/// [pt] { 0x6c2310 } (match_eng.cpp): escolha pelo valor da condicao: maior sorteio(valor + 5, ou + 25
/// no modo 2) x fator (0,2 com papel 1, senao 1,0).
pub(crate) unsafe extern "thiscall" fn choose_by_condition(this: usize, side_b: i8, mode: u32) -> u8 {
    let mut best = 0.0f32;
    let mut chosen: u8 = 0xff;
    for k in 0..20u8 {
        let p = side_player_slot(this, side_b, k as usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let factor = if match_player_position(p) & 1 != 0 { f32::from_bits(0x3e4ccccd) } else { 1.0 };
        let v = exe::match_player_get_foul_chance(p, 0) as i8 as i32 + if mode as u8 == 2 { 0x19 } else { 5 };
        let v = random(v) as f64 * factor as f64;
        if chosen == 0xff || v > best as f64 {
            best = v as f32;
            chosen = k;
        }
    }
    chosen
}

/// Forwards to the match referee's decide_foul (referee object at engine + 0xf5b2) for player `j` and code `codigo`,
/// writing the foul type and value; returns its result.
///
/// [pt] { 0x6cfca0 } (match_eng.cpp): `decide_foul` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn engine_decide_foul(this: usize, j: usize, code: u16, kind: *mut u8, value: *mut u8) -> u8 {
    exe::match_official_check_foul(this + engine::REFEREE, j, code, kind, value)
}

/// Match engine: asks the engine's referee (`engine::REFEREE`) to decide the punishment for a foul by `j` on `outro` (card type,
/// severity, weight and sending-off flag), via MatchOfficial::punish_move.
///
/// [pt] { 0x6cfcd0 } (match_eng.cpp): `punish_move` do arbitro do motor.
pub(crate) unsafe extern "thiscall" fn engine_punish_move(this: usize, j: usize, other: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32, send_off: u8) {
    exe::match_official_decide_foul_punishment(this + engine::REFEREE, j, other, kind, x, sev, flag, weight, send_off)
}

/// Forwards the foul punishment to the match engine's referee (this + 0xf5b2), i.e. calls punish_foul there.
///
/// [pt] { 0x6cfd10 } (match_eng.cpp): `punish_foul` do arbitro do motor.
pub(crate) unsafe extern "thiscall" fn engine_punish_foul(this: usize, j: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32) {
    exe::match_official_decide_booking(this + engine::REFEREE, j, kind, x, sev, flag, weight)
}

/// Chooses which player of the side takes the shot: each active outfield player gets a weight by role (from 0.15 for
/// defenders to 1.05 for forwards), adjusted by his rating (+5) and attributes, and a random draw based on his
/// attributes; with `modo` a different draw is used. The best draw wins, but 1 time in 5 the choice is discarded
/// (0xff). Returns the slot or 0xff.
///
/// [pt] { 0x6c23f0 } (match_eng.cpp): quinta escolha ponderada pelo papel (0,15 / 0,2 / 0,5 ou 0,3 /
/// 0,65 ou 0,8 / 1,05 / 0,95 ou 0,85), ajustada pela pessoa (palavra 5, bytes 0x37 e 0x43; sem
/// `modo`, a soma 0x40 + 0x36 + 0x26 + 0x1b; sorteio contra 0x21). Sorteio base: com `modo`,
/// 0xa1 - 0x99 - 0xad + c (sem fator); senao 4 em 5, 0xf1 + 0xc5 + 0x8d + c, ou 0xa9 + c. No fim,
/// 1 em 5 descarta a escolha (0xff). O fator fica no argumento e passa adiante.
pub(crate) unsafe extern "thiscall" fn choose_by_role_e(this: usize, argumento: u32, mode: u32) -> u8 {
    let side_b = argumento as i8;
    let mut factor = f32::from_bits(argumento);
    let mut best = 0.0f32;
    let mut chosen: u8 = 0xff;
    for k in 0..20u8 {
        let p = side_player_slot(this, side_b, k as usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let role = match_player_position(p);
        if role & 1 != 0 {
            continue;
        }
        let strong = role & 0x880 != 0;
        let bits = if role & 2 != 0 {
            Some(0x3e19999a)
        } else if role & 4 != 0 {
            Some(0x3e4ccccd)
        } else if role & 8 != 0 {
            Some(if strong { 0x3f000000 } else { 0x3e99999a })
        } else if role & 0x10 != 0 {
            Some(if strong { 0x3f266666 } else { 0x3f4ccccd })
        } else if role & 0x20 != 0 {
            Some(0x3f866666)
        } else if role & 0x40 != 0 {
            Some(if role & 0x700 != 0 { 0x3f733333 } else { 0x3f59999a })
        } else {
            None
        };
        if let Some(b) = bits {
            factor = f32::from_bits(b);
        }
        let person = *((p + match_player::PLAYER) as *const usize);
        if person != 0 {
            let w = *((person + player_sheet::CURRENT_ABILITY) as *const i16);
            factor = if w > 0x8c {
                factor * read_f32(rdata::F32_1_399999976)
            } else if w < 0x3c {
                factor * read_f32(rdata::F32_0_6000000238)
            } else {
                (w as f64 * read_f64(rdata::F64_0_01) * factor as f64) as f32
            };
            for o in [0x37, 0x43] {
                let a = read_i8(person + o);
                if a > 15 {
                    factor *= read_f32(rdata::F32_1_100000024);
                } else if a < 6 {
                    factor *= read_f32(rdata::F32_0_8000000119);
                } else if a < 10 {
                    factor *= read_f32(rdata::F32_0_9499999881);
                }
            }
            if mode as u8 == 0 {
                let t = [0x40, 0x36, 0x26, 0x1b].iter().fold(0u8, |t, &o| t.wrapping_add(read_i8(person + o) as u8)) as i8;
                if t > 0x4b {
                    factor *= read_f32(rdata::F32_1_25);
                } else if t > 0x3c {
                    factor *= read_f32(rdata::F32_1_049999952);
                } else if t < 0x19 {
                    factor = (factor as f64 * read_f64(rdata::F64_0_75)) as f32;
                } else if t < 0x28 {
                    factor *= read_f32(rdata::F32_0_9499999881);
                }
            }
            let r = random(read_i8(person + player_sheet::CONSISTENCY) as i32);
            if r < random(0x14) {
                factor *= read_f32(rdata::F32_0_75);
            }
        }
        let f = |o: usize| read_f32(p + o) as f64;
        let v = if mode as u8 != 0 {
            random(ftol(f(match_player::FREE_KICKS) - f(match_player::FINISHING) - f(match_player::JUMPING) + read_f32(rdata::F32_50) as f64)) as f64
        } else if random(5) != 0 {
            random(ftol(f(match_player::VISION) + f(match_player::PASSING) + f(match_player::CROSSING) + read_f32(rdata::F32_5) as f64)) as f64 * factor as f64
        } else {
            random(ftol(f(match_player::HEADING) + read_f32(rdata::F32_5) as f64)) as f64 * factor as f64
        };
        if chosen == 0xff || v > best as f64 {
            best = v as f32;
            chosen = k;
        }
    }
    if random(5) != 0 { chosen } else { 0xff }
}

/// Por zona da bola (y <= 3, < 8, o resto): contadores do lado 0 e do lado 1 (por periodo, total,
/// total da janela, janela de bytes) e as palavras da zona (por periodo, total, total da janela).
pub(crate) const POSSESSION_ZONES: [([usize; 4], [usize; 4], [usize; 3]); 3] = [
    ([0x8f52, 0x8f5e, 0x8f62, 0x8e2d], [0x8f16, 0x8f22, 0x8f26, 0x8d88], [0x1f6, 0x1fc, 0x1fe]),
    ([0x8f2a, 0x8f36, 0x8f3a, 0x8dbf], [0x8f3e, 0x8f4a, 0x8f4e, 0x8df6], [0x200, 0x206, 0x208]),
    ([0x8f02, 0x8f0e, 0x8f12, 0x8d51], [0x8f66, 0x8f72, 0x8f76, 0x8e64], [0x20a, 0x210, 0x212]),
];

/// Counts one possession tick for side `lado_b` in the ball's zone (by ball row: defence, midfield, attack), per period
/// (first half, second half, extra time, from engine `engine::PERIOD_END`). With `janela`, also keeps rolling totals over the last
/// 55 ticks (circular index `engine::POSSESSION_WINDOW_INDEX`), removing the outgoing tick; without it, side -1 is not counted.
///
/// [pt] { 0x6cf750 } (match_eng.cpp): conta um instante de posse do `lado` na zona da bola, por periodo
/// (0x8ed4: 0x1ef, 0x3de ou outro). Com `janela`, mantem tambem os totais dos ultimos 55 instantes
/// (janela circular em 0x8e9b): tira o que sai, poe o que entra. Sem janela, lado -1 nao conta.
pub(crate) unsafe extern "thiscall" fn possession_statistic(this: usize, side_b: i8, window: u8) {
    let w16 = |o: isize| (this as isize + o) as *mut u16;
    let w32 = |o: isize| (this as isize + o) as *mut i32;
    let b8 = |o: isize| (this as isize + o) as *mut i8;
    let mais16 = |o: isize| *w16(o) = (*w16(o)).wrapping_add(1);
    let mais32 = |o: isize| *w32(o) = (*w32(o)).wrapping_add(1);
    let per = match *w16(0x8ed4) {
        0x1ef => 0,
        0x3de => 1,
        _ => 2,
    };
    mais16(0x214 + 2 * per);
    mais16(0x21a);
    let y = read_i8(this + engine::BALL_Y);
    let z = if y <= 3 { 0 } else if y < 8 { 1 } else { 2 };
    let (l0, l1, pal) = POSSESSION_ZONES[z];
    let contadores = match side_b {
        0 => Some(l0),
        1 => Some(l1),
        _ => None,
    };
    if window == 0 {
        if side_b == -1 {
            return;
        }
        if let Some(c) = contadores {
            mais32(c[0] as isize + 4 * per);
            mais32(c[1] as isize);
        }
        mais16(pal[0] as isize + 2 * per);
        mais16(pal[1] as isize);
        return;
    }
    let i = read_i8(this + engine::POSSESSION_WINDOW_INDEX) as isize;
    for (o, b) in [(0x1fe, 0xa), (0x208, 0x41), (0x212, 0x78)] {
        *w16(o) = (*w16(o)).wrapping_sub(*b8(b + i) as i16 as u16);
    }
    let jan = [(0x8f12, 0x8d51), (0x8f26, 0x8d88), (0x8f3a, 0x8dbf), (0x8f4e, 0x8df6), (0x8f62, 0x8e2d), (0x8f76, 0x8e64)];
    for (o, b) in jan {
        *w32(o) = (*w32(o)).wrapping_sub(*b8(b + i) as i32);
    }
    for (_, b) in jan {
        *b8(b + i) = 0;
    }
    if let Some(c) = contadores {
        mais32(c[0] as isize + 4 * per);
        mais32(c[1] as isize);
        mais32(c[2] as isize);
        *b8(c[3] as isize + i) = 1;
    }
    mais16(pal[0] as isize + 2 * per);
    mais16(pal[1] as isize);
    mais16(pal[2] as isize);
    for (k, b) in [0xa, 0x41, 0x78].into_iter().enumerate() {
        *b8(b + i) = (k == z) as i8;
    }
    let n = (i as u8).wrapping_add(1) as i8;
    *b8(0x8e9b) = if n >= 0x37 { 0 } else { n };
}

/// Builds each side's summary at engine + 0xf5c0 + 0x36 x side: each player's 1..10 match rating (+0x1b from
/// disposition +0x35) and sums of several attribute bytes and words.
///
/// [pt] Resumo de cada lado (0xf5c0 + 0x36 x lado): nota de cada um dos 20 jogadores do bloco
/// (0x1b = (disposicao + 500) / 1000, em 1..=10) e somas de atributos (bytes e palavras).
pub(crate) unsafe fn summarize_sides(engine: usize) {
    for lado_b in 0..2usize {
        let c = engine + engine::SIDE_SUMMARY + 0x36 * lado_b;
        let b8 = |o: isize| (c as isize + o) as *mut u8;
        let w16 = |o: usize| (c + o) as *mut u16;
        for o in [2, 0, 1, -2, -1] {
            *b8(o) = 0;
        }
        for o in (0x22..=0x30).step_by(2) {
            *w16(o) = 0;
        }
        *b8(0xb) = 0;
        *b8(0xc) = 0;
        for k in 0..20 {
            let p = engine + engine::PLAYERS + 0x22d8 * lado_b + 0x1be * k;
            let rating = (p + match_player::MATCH_GRADE) as *mut i8;
            *rating = ((*((p + match_player::MATCH_RATING) as *const i16) as i32 + 500) / 1000) as i8;
            if *rating > 10 {
                *rating = 10;
            }
            if *rating < 1 {
                *rating = 1;
            }
            let u = |o: usize| *((p + o) as *const u8);
            for (dst, src) in [(2isize, 0xf), (0, 0xd), (1, 0xe), (-2, 0xa), (-1, 0xb)] {
                *b8(dst) = (*b8(dst)).wrapping_add(u(src));
            }
            for (dst, src) in [(0x22, 1), (0x24, 2), (0x26, 4), (0x28, 5), (0x2a, 7), (0x2c, 8), (0x2e, 9), (0x30, 0x12)] {
                *w16(dst) = (*w16(dst)).wrapping_add(u(src) as u16);
            }
            *b8(0xb) = (*b8(0xb)).wrapping_add(u(match_player::RED_CARDS));
            *b8(0xc) = (*b8(0xc)).wrapping_add(u(match_player::YELLOW_CARDS));
        }
    }
}

/// Match summary at time `momento` (`this` = match engine): at 0 or 1 rolls the tempo values (`engine::TEMPO_ROLL_A`, `engine::TEMPO_ROLL_B` in
/// 3..=9; `engine::TEMPO_GOALS_ROLL` from a roll plus the goals, at least 10) and summarizes both sides; at 0x3de (full time) only
/// summarizes; always recomputes the possession percentages for the current period (`engine::PERIOD_END`).
///
/// [pt] { 0x6ce460 } (match_eng.cpp): resumo da partida no `momento`: 0 e 1 sorteiam o ritmo (0xf628 e
/// 0xf62c: 3..=9; 0xf630: sorteio(5) + estados dos lados, minimo 10, senao novo sorteio) e
/// resumem os lados; 0x3de so resume. Sempre recalcula os percentuais do periodo (0x8ed4).
pub(crate) unsafe extern "thiscall" fn match_summary(this: usize, momento: i16) {
    let per = match *((this + engine::PERIOD_END) as *const u16) {
        0x1ef => 0i8,
        0x3de => 1,
        _ => 2,
    };
    match momento {
        0 | 1 => {
            let f = |o: usize| (this + o) as *mut f32;
            *f(engine::TEMPO_ROLL_A) = ((random(7) as u8).wrapping_add(3) as i8) as f32;
            *f(engine::TEMPO_ROLL_B) = ((random(7) as u8).wrapping_add(3) as i8) as f32;
            let estados = |r: i32| r + *((this + engine::SCORE) as *const u8) as i32 + *((this + engine::SCORE_SIDE1) as *const u8) as i32;
            let t = estados(random(5));
            let t = if t < 10 { 10 } else { estados(random(5)) };
            *f(engine::TEMPO_GOALS_ROLL) = (t as u8 as i8) as f32;
            summarize_sides(this);
        }
        0x3de => summarize_sides(this),
        _ => {}
    }
    exe::cm3_match_engine_compute_possession_percentages(this, per);
}

/// Returns the active player of side `lado_b` closest to cell (a, b) (excluding goalkeepers, and without `todos` those
/// with +0x2b > 1); zero distance returns immediately; 0 if none.
///
/// [pt] { 0x6c30e0 } (match_eng.cpp, laco desenrolado 20 vezes no original): o jogador ativo do `lado`
/// mais perto de (a, b) pela tabela (`f6f09b0`), sem `tabela` 1 e, sem `todos`, so com 0x2b <= 1.
/// Distancia zero devolve na hora; partindo de 5000, o menor estritamente. Sem candidato, 0.
pub(crate) unsafe extern "thiscall" fn closest_on_side(this: usize, side_b: i8, a: i8, b: i8, all: u8) -> usize {
    let mut best = 5000.0f32;
    let mut chosen = 0;
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) < 0 || match_player_position(p) == 1 || (all == 0 && read_i8(p + match_player::BUSY) > 1) {
            continue;
        }
        let d = match_player_distance_to_cell(p, a, b);
        if d == 0.0 || d.is_nan() {
            return p;
        }
        if d < best {
            best = d;
            chosen = p;
        }
    }
    chosen
}

/// Scores grid cell (x, y) for player j in choose_square (turn arc, tactics table, blockers, side bias...); None when
/// the cell is not eligible.
///
/// [pt] Pontua a casa (x, y) para o jogador em `choose_square`; None quando a casa nem entra.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn score_cell_for_player(engine: usize, j: usize, x: i8, y: i8, v: f32, d: i8, flags: u8, target: (i8, i8)) -> Option<f64> {
    let (jx, jy) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let flag2 = flags & 2 != 0;
    if x < 0 || x >= 9 || (jx == x && jy == y && flag2) {
        return None;
    }
    if flag2 {
        if exe::match_player_is_cell_within_turn_arc(j, x, y) == 0 {
            return None;
        }
    }
    let t = table_by_position(engine, j, 0x8ebc);
    if t as u8 & 0x30 != 0 && t & 0x880 != 0 && (2..=6).contains(&x) && !(2..=6).contains(&jx) && flags & 1 == 0 {
        return None;
    }
    let home = occupancy_grid_cell(engine, x, y);
    let opp = (side_of_player(j) == 0) as isize;
    let mut bl: i8 = 0;
    let mut base = v;
    if flag2 {
        bl = *((home + 0x58 + opp) as *const i8);
        if bl != 0 {
            let person = *((j + match_player::PLAYER) as *const usize);
            let r = random(read_i8(person + player_sheet::DRIBBLING) as i32);
            if r > random(3) {
                bl = (bl as i32 - 2).max(0) as i8;
            }
        }
        base = (v as f64 - (bl as f64 - read_f64(rdata::F64_0_5))) as f32;
        let l = side_of_player(j);
        let ajusta = match read_i8(j + match_player::PREFERRED_FOOT) {
            1 => if l == 0 { x < jx || (x == jx && x < 4) } else { x > jx || (x == jx && x > 4) },
            0 => if l == 0 { x > jx || (x == jx && x > 4) } else { x < jx || (x == jx && x < 4) },
            _ => false,
        };
        if ajusta {
            base = (base as f64 - read_i8(j + 0x2d) as f64 * read_f64(rdata::F64_0_125)) as f32;
        }
    } else {
        let n = *((home + 0x58 + opp) as *const i8);
        for i in 0..n.max(0) as isize {
            let p = *((home + 4 * (11 * opp + i)) as *const usize);
            if read_i8(p + match_player::POSITION) >= 0 && read_i8(p + match_player::BUSY) < 2 {
                bl = bl.wrapping_add(1);
            }
        }
    }
    let mut s = if bl != 0 && !flag2 {
        base as f64 - 2.0 * (bl as f64 * bl as f64)
    } else {
        let mut best = 5000.0f32;
        let mut zero = false;
        for k in 0..20 {
            let p = side_player_slot(engine, opp as i8, k);
            if read_i8(p + match_player::POSITION) < 0 || match_player_position(p) & 1 != 0 {
                continue;
            }
            let dist = match_player_distance_to_cell(p, x, y);
            if dist == 0.0 || dist.is_nan() {
                zero = true;
                break;
            }
            if dist < best {
                best = dist;
            }
        }
        let m = if zero { read_f32(rdata::F32_0) } else { best } as f64;
        m * m + base as f64
    };
    let (l, yj) = (side_of_player(j), read_i8(j + match_player::CELL_Y));
    let mut volta = false;
    if (l == 0 && y < yj) || (l == 1 && y > yj) {
        if flags & 8 == 0 {
            s -= read_f64(rdata::F64_4_5);
            volta = true;
        }
    } else {
        s += if y == jy { read_f64(rdata::F64_0_5) } else { read_f64(rdata::F64_4) };
    }
    if d > 0 {
        s -= (d as i32 * 4) as f64;
    }
    if volta {
        return Some(s);
    }
    if (!flag2 || flags & 1 != 0) && !(3..=8).contains(&y) && x < 6 && x > 2 {
        s += read_f64(rdata::F64_3_5);
    }
    if x == 4 || (x > 2 && jx < 4 && jx >= x) || (x < 6 && jx > 4 && jx <= x) {
        s += read_f64(rdata::F64_3);
    }
    let (a, b) = (jx as i32 - 4, x as i32 - 4);
    if !flag2 {
        if jx > 4 && a < b {
            s -= (3 * a) as f64;
        } else if jx < 4 && a > b {
            s -= (3 * (4 - jx as i32)) as f64;
        }
    } else {
        if !(2..=6).contains(&jx) && !(2..=6).contains(&x) && flags & 1 == 0 {
            s += read_f32(j + match_player::TEAMWORK) as f64 * read_f64(rdata::F64_0_25);
        }
        if jx >= 4 && a < b {
            s -= a as f64 * read_f64(rdata::F64_2_5);
        } else if jx <= 4 && a > b {
            s -= (4 - jx as i32) as f64 * read_f64(rdata::F64_2_5);
        }
        if target == (x, y) {
            s += read_f32(rdata::F32_2_5) as f64;
        }
    }
    Some(s)
}

/// Match engine: chooses for player `j` the neighbouring cell (3 x 3 around his position +0x102/+0x103) with the best
/// score above 7.5, weighing opponents in or near the cell, retreat, the offside line (flag 4), the central band and
/// the previous target (`*sx`, `*sy` with flag 2). Writes the cell (or -1) to `*sx`/`*sy` and returns whether one was
/// found; without flag 2 a player lacking +0x107 and +0x105 does not move (returns 0).
///
/// [pt] { 0x6bce80 } (match_eng.cpp): escolhe a casa vizinha (3 x 3 em volta do jogador) com a maior
/// pontuacao, partindo de 7,5 (empate fica com a ultima): adversarios na casa ou o mais perto
/// dela, recuo, linha de impedimento (flag 4), faixa central e o alvo anterior (`x`, `y` na entrada,
/// com flag 2). Grava a casa (ou -1) e devolve se achou. Sem flag 2, o jogador sem 0x107 nem 0x105
/// nao escolhe (devolve 0 sem gravar); com 0x105, um sorteio de -5..5 entra na base.
pub(crate) unsafe extern "thiscall" fn choose_square(this: usize, j: usize, sx: *mut i8, sy: *mut i8, flags: u32) -> u8 {
    let engine = this;
    let fl = flags as u8;
    let (jx, jy) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let mut v = 0.0f32;
    if fl & 2 == 0 && read_i8(j + match_player::MOMENT_MOVEMENT) == 0 {
        if read_i8(j + match_player::MOMENT_ANTICIPATION) == 0 {
            return 0;
        }
        let r = random(6);
        v = (r - random(6)) as f32;
    }
    let target = (*sx, *sy);
    let (mut bx, mut by) = (-1i8, -1i8);
    let mut best = 7.5f32;
    let (y0, y1) = (jy.wrapping_sub(1), jy.wrapping_add(1));
    let mut y = y0;
    while y0 <= y1 {
        'linha: {
            if !(0..12).contains(&y) {
                break 'linha;
            }
            let mut d: i8 = -10;
            if fl & 4 != 0 {
                let l = side_of_player(j);
                d = if read_i8(j + match_player::MOMENT_MOVEMENT) < 2 {
                    let base = if l == 1 { read_i8(engine + engine::OFFSIDE_LINE_SIDE1).wrapping_sub(y) } else { y.wrapping_sub(read_i8(engine + engine::OFFSIDE_LINE)) };
                    let r1 = random(3) as i8;
                    let r2 = random(3) as i8;
                    base.wrapping_sub(r1).wrapping_add(r2)
                } else if l == 1 {
                    read_i8(engine + engine::OFFSIDE_LINE_SIDE1).wrapping_sub(y)
                } else {
                    y.wrapping_sub(read_i8(engine + engine::OFFSIDE_LINE))
                };
                if read_i8(j + match_player::MOMENT_DECISIONS) != 0 {
                    let i = (side_of_player(j) == 0) as usize;
                    if *((engine + 6371 * i + 0x9766) as *const u32) & 0x400 != 0 {
                        d = d.wrapping_add(1);
                    }
                }
                if d > 2 {
                    break 'linha;
                }
            }
            let (x0, x1) = (jx.wrapping_sub(1), jx.wrapping_add(1));
            let mut x = x0;
            while x0 <= x1 {
                if let Some(s) = score_cell_for_player(engine, j, x, y, v, d, fl, target) {
                    if !(s < best as f64) && !s.is_nan() {
                        bx = x;
                        by = y;
                        best = s as f32;
                    }
                }
                x = x.wrapping_add(1);
                if x > x1 {
                    break;
                }
            }
        }
        y = y.wrapping_add(1);
        if y > y1 {
            break;
        }
    }
    *sy = by;
    *sx = bx;
    (bx > -1) as u8
}

/// TapeCall a `search_on_side` (`exe::cm3_match_engine_try_cell_interception`) feita pelo tracado: (lado, casa, distancia, a4, tipo, a6).
pub(crate) type SearchOnSideCall<'a> = &'a mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8;

/// `0x6ccb60` with the call to `0x6cd330` injected: walks the cells of the line from (x0, y0) to (x1, y1) looking for a
/// player.
///
/// Uses the major axis with accumulated error, Bresenham-style; the final cell is only included if the line is a single
/// point. In each cell it asks `search_on_side` for a player of a randomly drawn side and then of the other, with the
/// distance from the origin (12 × 9 table; outside it, 1000). The first hit returns 1; leaving the pitch or reaching
/// the end returns 0. The type (by `codigo`) is passed along.
///
/// [pt] { 0x6ccb60 } com a chamada a 0x6cd330 injetada: percorre as casas da linha de (x0, y0) a
/// (x1, y1) (eixo maior, erro acumulado como Bresenham; a casa final so entra se a linha for um
/// ponto) e, em cada uma, pergunta a `search_on_side` por um jogador de um lado sorteado e
/// depois do outro, com a distancia desde a origem (tabela 12 x 9; fora, 1000). Primeiro que
/// achar, 1; sair do campo ou chegar ao fim, 0. O tipo (por `codigo`) vai junto.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn trace_line_interception_with(engine: usize, x0: i8, y0: i8, x1: i8, y1: i8, code: i8, a6: u32, a7: u32, call: SearchOnSideCall) -> u8 {
    let kind: u8 = match code {
        9 | 62 | 90 => {
            let r = random(*(&a6 as *const u32 as *const i16) as i32 + 0xbb8);
            (r != 0) as u8 + 4
        }
        31 | 54 | 60 | 61 => random(4) as u8 + 1,
        6 | 8 | 30 | 52 | 55 | 91 | 92 | 94 => 4,
        _ => 0,
    };
    let ddx = (x1 as i32 - x0 as i32).abs() as i8;
    let ddy = (y1 as i32 - y0 as i32).abs() as i8;
    let sign = |v: i32| if v >= 0 { 1i8 } else { -1 };
    // (is the x axis the major one?, major-axis step, minor step, major delta, minor delta)
    let (eixo_x, step_towards, passo_menor) = if ddx >= ddy {
        if x0 < x1 { (true, 1i8, sign(y1 as i32 - y0 as i32)) } else { (true, -1, sign(y1 as i32 - y0 as i32)) }
    } else if y0 < y1 {
        (false, 1, sign(x1 as i32 - x0 as i32))
    } else {
        (false, -1, sign(x1 as i32 - x0 as i32))
    };
    let (dmaior, dmenor) = if eixo_x { (ddx, ddy) } else { (ddy, ddx) };
    let (mut x, mut y) = (x0, y0);
    let mut error: i8 = 0;
    let away = |x: i8, y: i8| !(0..=8).contains(&x) || !(0..=11).contains(&y);
    loop {
        if away(x, y) {
            return 0;
        }
        if error >= dmaior {
            error = error.wrapping_sub(dmaior);
            if eixo_x { y = y.wrapping_add(passo_menor) } else { x = x.wrapping_add(passo_menor) }
        }
        let target = x == x1 && y == y1 && (ddx != 0 || ddy != 0);
        if !target {
            if away(x, y) {
                return 0;
            }
            let home = occupancy_grid_cell(engine, x, y) as usize;
            let r = random(2) as i8;
            let ex = (x as i32 - x0 as i32).abs() as i8;
            let ey = (y as i32 - y0 as i32).abs() as i8;
            let dist = if (0..=8).contains(&ex) && (0..=11).contains(&ey) { *((G_CELL_DISTANCE_TABLE + 4 * (ey as usize * 9 + ex as usize)) as *const u32) } else { 0x447a0000 };
            if call(r, home, dist, a6, kind as u32, a7) != 0 || call((r == 0) as i8, home, dist, a6, kind as u32, a7) != 0 {
                return 1;
            }
        }
        error = error.wrapping_add(dmenor);
        if eixo_x {
            x = x.wrapping_add(step_towards);
            if (step_towards > 0 && x > x1) || (step_towards < 0 && x < x1) {
                return 0;
            }
        } else {
            y = y.wrapping_add(step_towards);
            if (step_towards > 0 && y > y1) || (step_towards < 0 && y < y1) {
                return 0;
            }
        }
    }
}

/// Traces a move (pass or shot) from cell (x0, y0) to (x1, y1), checking each cell on the way for a player of either
/// side who intercepts it. Returns 1 on the first interception, 0 if the ball leaves the pitch or reaches the end.
///
/// [pt] { 0x6ccb60 } (match_eng.cpp): tracado do lance de (x0, y0) a (x1, y1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn trace_move(this: usize, x0: i8, y0: i8, x1: i8, y1: i8, code: i8, a6: u32, a7: u32) -> u8 {
    trace_line_interception_with(this, x0, y0, x1, y1, code, a6, a7, &mut |l, c, d, a, t, b| exe::cm3_match_engine_try_cell_interception(this, l, c, d, a, t, b, 1))
}

/// { `apply_substitutions` } (match_eng.cpp) with the calls that touch things outside the match engine injected (0x6d6f70
/// commentates, `exe::match_man_make_substitutions` and `redo_team` for the staff, 0x897760 writes the captain): applies the marked substitutions
/// (+0x1d) of both sides.
///
/// Whoever is on the pitch (or −3) and marked: with +0x1c he goes off (on the pitch: commentary 0x1f56, `exe::cm3_match_engine_remove_player_from_pitch` with
/// −1 or −2 depending on the side's `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT`, +0x1a = position, +0x19 = −2; with −3: commentary 0x1f57 and `exe::cm3_match_engine_remove_player_from_pitch` with
/// 0) and the minute is written at +0x22 (at most the end of the period); without +0x1c, commentary 0x1f58 by random
/// draw(4) or with +0x55. With +0x55, commentary 0x20f6 (both players and each one's side, `player_side`).
///
/// If there are marked players, except in matches with a human manager who does not delegate (as in 0x6cda20): with
/// `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT` the staff substitutes (`exe::match_man_make_substitutions`); otherwise the team is rebuilt (`redo_team`), and if the captain (`exe::tactics_find_captain_slot`)
/// has left the pitch the best-rated player (`best_rated`) becomes captain (0x897760), `engine::GOALKEEPER`/`engine::GOALKEEPER_SIDE1` are zeroed and the
/// team is recomposed (`exe::cm3_match_engine_get_goalkeeper`). It stores the substitution minute (`engine::LAST_SUBSTITUTION_TICK` + 2 × side), rebuilds the side's strength
/// (`exe::cm3_match_engine_recalc_side_score`) and the marking (`individual_marking`). At the end it zeroes `engine::SUBSTITUTION_PENDING`.
///
/// On the tape, the commentary's 6th argument goes in byte 1 of the 5th (0x6d6f70 reads both as bytes; only byte 0 is
/// checked).
///
/// [pt] { 0x6d0b60 } (match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70 narra,
/// 0x6e1090 e 0x6e1aa0 da comissao, 0x897760 grava o capitao): aplica as trocas marcadas (+0x1d) dos
/// dois lados. Quem esta em campo (ou -3) e marcado: com +0x1c sai (em campo: narra 0x1f56, 0x6cdd50
/// com -1 ou -2 conforme +0x9049 do lado, +0x1a = posicao, +0x19 = -2; com -3: narra 0x1f57 e 0x6cdd50
/// com 0) e grava o minuto em +0x22 (no maximo o fim do periodo); sem +0x1c narra 0x1f58 por sorteio(4)
/// ou com +0x55. Com +0x55 narra 0x20f6 (os dois e o lado de cada um, 0x714d30). Havendo marcados, fora
/// dos jogos com tecnico humano que nao delega (como na 0x6cda20): com +0x9049 a comissao substitui
/// (0x6e1090); senao refaz o time (0x6e1aa0), e se o capitao (0x897590) saiu de campo o mais bem
/// avaliado (0x6e1a40) vira capitao (0x897760), zera +0xf59e/+0xf5a2 e recompoe (0x6ce110). Guarda o
/// minuto da troca (+0x8ecc + 2 x lado), refaz a forca do lado (0x6bbb60) e a marcacao (0x6bbf60). No
/// fim zera +0x8eb5. Na fita, o 6o argumento da narracao vai no byte 1 do 5o (a 0x6d6f70 le os dois
/// como bytes; so o byte 0 e conferido).
pub(crate) unsafe fn apply_substitutions_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    for lado in 0..2i8 {
        let lu = lado as usize;
        let mut marked = 0u8;
        for k in 0..20 {
            let j = m + engine::PLAYERS + 0x22d8 * lu + 0x1be * k;
            let e = read_i8(j + match_player::POSITION);
            if !(e >= 0 || e == -3) || read_i8(j + match_player::INJURY_PENDING) == 0 {
                continue;
            }
            marked = marked.wrapping_add(1);
            let name = |p: usize| *((p + 0x2e) as *const u8) as u32;
            if read_i8(j + match_player::INJURY_FORCES_OFF) != 0 {
                if e >= 0 {
                    let a = name(j);
                    call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [0x1f56, m as u32, a, exe::match_player_get_side(j) as u32, u32::MAX]);
                    exe::cm3_match_engine_remove_player_from_pitch(m, j, if read_i8(m + (0x9049 + 0xd0 * lu)) != 0 { -1 } else { -2 });
                    *((j + match_player::PREVIOUS_POSITION) as *mut i8) = read_i8(j + match_player::POSITION);
                    *((j + match_player::POSITION) as *mut i8) = -2;
                } else {
                    let a = name(j);
                    call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [0x1f57, m as u32, a, exe::match_player_get_side(j) as u32, u32::MAX]);
                    exe::cm3_match_engine_remove_player_from_pitch(m, j, 0);
                }
                *((j + match_player::INJURY_MINUTE) as *mut i8) = minute_in_period(m);
            } else if random(4) == 0 || read_usize(j + match_player::INJURY_CULPRIT) != 0 {
                let a = name(j);
                call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [0x1f58, m as u32, a, exe::match_player_get_side(j) as u32, u32::MAX]);
            }
            let p55 = read_usize(j + match_player::INJURY_CULPRIT);
            if p55 != 0 {
                let (a5, a3) = (name(p55), name(j));
                let a6 = exe::match_player_get_side(p55) as u32;
                let a4 = exe::match_player_get_side(j) as u32;
                call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [0x20f6, m as u32, a3, a4, a5 | a6 << 8]);
            }
            *((j + match_player::INJURY_PENDING) as *mut u8) = 0;
        }
        if marked == 0 {
            continue;
        }
        let base = m + 0xd0 * lu;
        let t = Fixture(read_usize(m + engine::MATCH));
        let club_of = |l: i8| if l == 0 { t.home_club() } else { t.away_club() };
        let p = read_usize(base + engine::SIDE0_MANAGER);
        let q = *(G_STAFF_COUNT as *const i32);
        if p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && read_i8(p + 0x5f) != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER + 0), p) == 0 {
            if read_i8(base + engine::SIDE0_MANAGER_TACTIC_SLOT) == 0 || exe::database_is_national_team(club_of(lado)) != 0 || exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER + 0), read_usize(base + engine::SIDE0_MANAGER)) != 0 {
                continue;
            }
        }
        let staff = base + engine::MATCH_MANAGERS;
        if read_i8(base + engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT) != 0 {
            call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [0, 0, 0, 0, 0]);
        } else {
            call(FN_MATCH_MAN_REASSIGN_POSITIONS, staff, [0, 0, 0, 0, 0]);
            let main = if lado != 0 { t.away_side_mark() } else { (t.home_side_mark() as i8) } == 0;
            let v = exe::tactics_find_captain_slot(read_usize(G_TACTICS + 0), club_of(lado), main as u32, 0);
            let jv = (m as isize + 0x4796 + 0x1be * (v as isize + 20 * lado as isize)) as usize;
            if read_i8(jv + match_player::POSITION) < 0 {
                let main = if lado != 0 { t.away_side_mark() } else { (t.home_side_mark() as i8) } == 0;
                let k = exe::match_man_find_captain_slot(staff);
                call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, read_usize(G_TACTICS + 0), [club_of(lado) as u32, k as u8 as u32, main as u32, 0, 0]);
            }
            *((m + if lado == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
            exe::cm3_match_engine_get_goalkeeper(m, lado);
        }
        *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16);
        exe::cm3_match_engine_recalc_side_score(m, lado);
        exe::cm3_match_engine_assign_marking_targets(m);
    }
    *((m + engine::SUBSTITUTION_PENDING) as *mut u8) = 0;
}

/// applies both sides' marked substitutions (+0x1d). With +0x1c the player goes off (on the pitch: narrates 0x1f56,
/// `exe::cm3_match_engine_remove_player_from_pitch` with −1 or −2 by `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT`, +0x1a = position and +0x19 = −2; with −3: narrates 0x1f57 and `exe::cm3_match_engine_remove_player_from_pitch` with 0)
/// and the minute goes to +0x22; without +0x1c it narrates 0x1f58 by draw(4) or with +0x55; with +0x55 it narrates
/// 0x20f6 with each one's side (`player_side`). With marked players and outside matches with a human manager who doesn't
/// delegate: with `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT` the staff substitute (`exe::match_man_make_substitutions`); otherwise they redo the team (`redo_team`), give the armband
/// (0x897760, the best rated from `best_rated`) if the captain (`exe::tactics_find_captain_slot`) went off, clear `engine::GOALKEEPER`/`engine::GOALKEEPER_SIDE1` and recompose
/// (`exe::cm3_match_engine_get_goalkeeper`). Stores the minute (`engine::LAST_SUBSTITUTION_TICK`), redoes the side's strength (`exe::cm3_match_engine_recalc_side_score`) and the marking (`individual_marking`); at the
/// end clears `engine::SUBSTITUTION_PENDING`. Compared through the tape (narration, staff and captain as units); the 6th argument of narration
/// 0x20f6 goes in byte 1 of the 5th (0x6d6f70 reads both as bytes)
/// Original: `apply_substitutions` (0x6d0b60, match_eng.cpp (~1 KB), thiscall).
///
/// [pt] { 0x6d0b60 } (match_eng.cpp): ver `apply_substitutions_with`.
pub(crate) unsafe extern "thiscall" fn apply_substitutions(m: usize) {
    apply_substitutions_with(m, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => add_match_event(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

/// `0x6d1210` (ret 4, match_eng.cpp) with the calls that change things outside the engine injected: applies the pending
/// incidents (+0x1a3 entries from +0xb1): referee punishment, cards, commentary, team anger, sendings-off and the AI's
/// resulting team changes.
///
/// The injected calls: 0x6d6f70 commentary; inside 0x6cda20 and at the end, `redo_team`/`exe::match_man_make_substitutions` of the staff and 0x897760
/// of the captain.
///
/// On the 1st call (`engine::PUNISHMENT_STATE` = 0) it only prepares `primeiro` (0x716630 with 0) and sets `engine::PUNISHMENT_STATE` = 1 if there are
/// pending ones.
///
/// Then it applies each of the +0x1a3 pending incidents (11 bytes from +0xb1: player, other, severity, code, x): the
/// referee's punishment (`exe::match_official_decide_foul_punishment`, type 1) gives the card; 2 comments 0x1ff9, 3 and 4 (sending-off) comment
/// 0x1ff7/0x1ff8 and mark `engine::SIDE_SENT_OFF` of the side (goalkeeper: table 1); without a card, severity above random(7) + 2 with
/// the player on the pitch comments 0x210a. When commenting, 0x20f5 (random(severity) > random(20)) and 0x2109 come
/// first.
///
/// Outside codes 0x25, 0x2a..0x32, 0x60 and 0x61: by random(9) it comments 0x20df (and 0x20e0 by random(3); types 3 to
/// 6 mark +0x16 and 0x20d9 by random(7)), passes the punishment's weight to the side's players (`exe::match_player_scale_anger`: the player
/// himself, those on the pitch (p + 1) / 2, those off it p / 6 + 0.83) and 0x716e10 (1 on a sending-off); a sending-off
/// removes the player (0x6cda20).
///
/// At the end, for each marked side and outside matches with a human manager who does not delegate: with `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT` the
/// staff change the goalkeeper (`exe::match_man_make_substitutions` with 1), otherwise rebuild the team (`redo_team`), give the armband to the
/// best-rated player (`best_rated`/0x897760), zero `engine::GOALKEEPER`/`engine::GOALKEEPER_SIDE1` and recompose (`exe::cm3_match_engine_get_goalkeeper`); it stores the minute and
/// recomputes strength and marking.
///
/// Ends with +0x1a3 = 0 and `engine::PUNISHMENT_STATE` = 2.
///
/// [pt] { 0x6d1210 } (ret 4, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra; dentro da 0x6cda20 e no fim, 0x6e1aa0/0x6e1090 da comissao e 0x897760 do capitao): na 1a
/// chamada (+0x8eb4 = 0) so prepara `primeiro` (0x716630 com 0) e marca +0x8eb4 = 1 se houver
/// pendentes. Depois aplica cada um dos +0x1a3 lances pendentes (11 bytes a partir de +0xb1: jogador,
/// outro, severidade, codigo, x): a punicao do arbitro (0x6ea4e0, tipo 1) da o cartao; 2 narra
/// 0x1ff9, 3 e 4 (expulsao) narram 0x1ff7/0x1ff8 e marcam +0x1cb do lado (goleiro: tabela 1); sem
/// cartao, severidade acima de sorteio(7) + 2 com o jogador em campo narra 0x210a. Narrando, antes vem
/// 0x20f5 (sorteio(severidade) > sorteio(20)) e 0x2109. Fora dos codigos 0x25, 0x2a..0x32, 0x60 e 0x61:
/// por sorteio(9) narra 0x20df (e 0x20e0 por sorteio(3); tipos 3 a 6 marcam +0x16 e 0x20d9 por
/// sorteio(7)), passa o peso da punicao aos do lado (0x716450: o proprio, os em campo (p + 1) / 2, os
/// de fora p / 6 + 0,83) e 0x716e10 (1 na expulsao); expulsao tira o jogador (0x6cda20). No fim, para
/// cada lado marcado e fora dos jogos com tecnico humano que nao delega: com +0x9049 a comissao troca o
/// goleiro (0x6e1090 com 1), senao refaz o time (0x6e1aa0), da a braçadeira ao mais bem avaliado
/// (0x6e1a40/0x897760), zera +0xf59e/+0xf5a2 e recompoe (0x6ce110); guarda o minuto e refaz a forca e
/// a marcacao. Termina com +0x1a3 = 0 e +0x8eb4 = 2.
pub(crate) unsafe fn apply_pending_punishments_with(m: usize, first: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    if read_i8(m + engine::PUNISHMENT_STATE) == 0 {
        call(FN_MATCH_PLAYER_START_FIGHT, first, [0; 5]);
        if read_i8(m + engine::PENDING_INCIDENT_COUNT) != 0 {
            *((m + engine::PUNISHMENT_STATE) as *mut u8) = 1;
        }
        return;
    }
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, u32::MAX]);
    };
    let mut k = 0i8;
    while k < read_i8(m + engine::PENDING_INCIDENT_COUNT) {
        let e = (m as isize + 11 * k as isize) as usize;
        let j = read_usize(e + engine::PENDING_INCIDENTS);
        let table = exe::match_player_get_position(j);
        let mut kind = 1u8;
        let mut weight = 0f32;
        exe::match_official_decide_foul_punishment(m + engine::REFEREE, j, read_usize(e + engine::PENDING_INCIDENT_VICTIM), &mut kind, read_i8(e + engine::PENDING_INCIDENT_REASON), read_i8(e + engine::PENDING_INCIDENT_SEVERITY), 0, &mut weight, 0);
        let mark_side = || *((m as isize + read_i8(j + match_player::SIDE) as isize + 0x1cb) as *mut u8) = (table == 1) as u8;
        let code: i32 = match kind as i8 {
            2 => 0x1ff9,
            3 => {
                mark_side();
                0x1ff7
            }
            4 => {
                mark_side();
                0x1ff8
            }
            _ => {
                let r = random(7) + 2;
                if read_i8(e + engine::PENDING_INCIDENT_SEVERITY) as i32 > r && read_i8(j + match_player::POSITION) >= 0 { 0x210a } else { -1 }
            }
        };
        if code != -1 {
            let a = random(read_i8(e + engine::PENDING_INCIDENT_SEVERITY) as i32);
            if a > random(20) {
                narrate(call, 0x20f5, 0, (read_i8(j + match_player::SIDE) == 0) as u32);
            }
            narrate(call, 0x2109, *((j + match_player::COMMENTARY_ID) as *const u8) as u32, *((j + match_player::SIDE) as *const u8) as u32);
            narrate(call, code as u32, *((j + match_player::COMMENTARY_ID) as *const u8) as u32, *((j + match_player::SIDE) as *const u8) as u32);
        }
        let v = (read_i8(e + engine::PENDING_INCIDENT_PLAY) as i32 - 0x25) as u32;
        let skips = v <= 0x3c && matches!(v, 0 | 5..=13 | 0x3b | 0x3c);
        let sent_off = kind == 4 || kind == 3;
        if !skips && code != -1 {
            if random(9) == 0 {
                narrate(call, 0x20df, u32::MAX, *((j + match_player::SIDE) as *const u8) as u32);
                if random(3) == 0 {
                    narrate(call, 0x20e0, u32::MAX, *((j + match_player::SIDE) as *const u8) as u32);
                }
                if matches!(kind, 3..=6) {
                    *((j + match_player::SENT_OFF) as *mut u8) = 1;
                    if random(7) == 0 {
                        narrate(call, 0x20d9, u32::MAX, *((j + match_player::SIDE) as *const u8) as u32);
                    }
                }
            }
            let block = m + engine::PLAYERS + 0x22d8 * read_i8(j + match_player::SIDE) as isize as usize;
            for i in 0..20 {
                let p = block + 0x1be * i;
                if read_usize(p + 0x69) == 0 {
                    continue;
                }
                let f = if p == j {
                    weight
                } else if read_i8(p + 0x19) < 0 {
                    (weight as f64 * 0.16666666666666666 + 0.83) as f32
                } else {
                    ((weight as f64 + 1.0) * 0.5) as f32
                };
                exe::match_player_scale_anger(p, f);
            }
            if sent_off {
                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [1, 0, 0, 0, 0]);
                player_left_pitch_with(m, j, 0, call);
            } else {
                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [0; 5]);
            }
        } else if sent_off {
            player_left_pitch_with(m, j, 0, call);
        }
        k = k.wrapping_add(1);
    }
    *((m + engine::PENDING_INCIDENT_COUNT) as *mut u8) = 0;
    for lado in 0..2i8 {
        let lu = lado as usize;
        if read_i8(m + (0x1cb + lu)) != 0 {
            let base = m + 0xd0 * lu;
            let t = Fixture(read_usize(m + engine::MATCH));
            let club_of = |l: i8| if l == 0 { t.home_club() } else { t.away_club() };
            let p = read_usize(base + engine::SIDE0_MANAGER);
            let q = *(G_STAFF_COUNT as *const i32);
            let skips = p != 0 && *(p as *const i32) >= q - 0x10 && *(p as *const i32) < q && read_i8(p + 0x5f) != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER + 0), p) == 0
                && (read_i8(base + engine::SIDE0_MANAGER_TACTIC_SLOT) == 0 || exe::database_is_national_team(club_of(lado)) != 0 || exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER + 0), read_usize(base + engine::SIDE0_MANAGER)) != 0);
            if !skips {
                let staff = base + engine::MATCH_MANAGERS;
                if read_i8(base + engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT) != 0 {
                    call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [1, 0, 0, 0, 0]);
                } else {
                    call(FN_MATCH_MAN_REASSIGN_POSITIONS, staff, [0, 0, 0, 0, 0]);
                    let main = if lado != 0 { t.away_side_mark() } else { (t.home_side_mark() as i8) } == 0;
                    let k = exe::match_man_find_captain_slot(staff);
                    call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, read_usize(G_TACTICS + 0), [club_of(lado) as u32, k as u8 as u32, main as u32, 0, 0]);
                    *((m + if lado == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
                    exe::cm3_match_engine_get_goalkeeper(m, lado);
                }
                *((m as isize + lado as isize * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16);
                exe::cm3_match_engine_recalc_side_score(m, lado);
                exe::cm3_match_engine_assign_marking_targets(m);
            }
        }
        *((m + engine::SIDE_SENT_OFF + lu) as *mut u8) = 0;
    }
    *((m + engine::PUNISHMENT_STATE) as *mut u8) = 2;
}

/// { 0x6d1210 } (ret 4, match_eng.cpp): ver `apply_pending_punishments_with`.
// Ports of 0x716630 (prepares) and 0x716e10 (tires): both narrate internally. Tape units only in
// comparison of punishments (0x716e10 also runs inside many other routines).
pub(crate) static RECORD_716: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static mut START_FIGHT_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_716630: Prologue = Prologue { address: 0x716630, bytes: &[0x83, 0xec, 0x10, 0x53, 0x55] };
pub(crate) static mut REACT_TO_FOUL_DECISION_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_716E10: Prologue = Prologue { address: 0x716e10, bytes: &[0x83, 0xec, 0x08, 0x53, 0x56] };
pub(crate) static mut HANDLE_SUSPECTED_DIVE_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_716C90: Prologue = Prologue { address: 0x716c90, bytes: &[0x51, 0x56, 0x8b, 0xf1, 0x57] };

/// Shared gate for the 0x716xxx MATCH_PLAYER calls: runs the original directly, or records it on the open tape as one
/// unit when RECORD_716 is set.
pub(crate) unsafe fn gate_match_player_call(routine: usize, target: usize, p: usize, a1: u32) {
    let o: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(target);
    if !(RECORD_716.load(std::sync::atomic::Ordering::Relaxed) && is_recording()) {
        return o(p, a1);
    }
    call_recording(routine, p, [a1, 0, 0, 0, 0], || {
        o(p, a1);
        0
    });
}
/// Gate for MATCH_PLAYER::start_fight (0x716630): records it on the comparison tape when recording, else just calls it.
pub(crate) unsafe extern "thiscall" fn gate_start_fight(p: usize, a1: u32) {
    gate_match_player_call(0x716630, START_FIGHT_TRAMPOLINE, p, a1)
}
/// Gate for MATCH_PLAYER::react_to_foul_decision (0x716e10): records it on the comparison tape when recording, else
/// just calls it.
pub(crate) unsafe extern "thiscall" fn gate_react_to_foul_decision(p: usize, a1: u32) {
    gate_match_player_call(0x716e10, REACT_TO_FOUL_DECISION_TRAMPOLINE, p, a1)
}
/// Gate for MATCH_PLAYER::handle_suspected_dive (0x716c90): records it on the comparison tape when recording, else just
/// calls it.
pub(crate) unsafe extern "thiscall" fn gate_handle_suspected_dive(p: usize) {
    let o: unsafe extern "thiscall" fn(usize) = core::mem::transmute(HANDLE_SUSPECTED_DIVE_TRAMPOLINE);
    if !(RECORD_716.load(std::sync::atomic::Ordering::Relaxed) && is_recording()) {
        return o(p);
    }
    call_recording(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, p, [0; 5], || {
        o(p);
        0
    });
}

/// Hook entry for apply_pending_punishments_with: applies each pending incident (+0x1a3 entries of 11 bytes from
/// +0xb1): the referee's punishment gives cards, commentary is added, the discipline weight spreads to the side and
/// sent-off players leave the pitch; then marked sides rebuild the team, captain and marking. Ends with the pending
/// count zeroed and `engine::PUNISHMENT_STATE` = 2.
pub(crate) unsafe extern "thiscall" fn punishments(m: usize, first: usize) {
    apply_pending_punishments_with(m, first, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => add_match_event(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x716630 | 0x716e10 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
                f(this, a[0]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

/// `0x6be710` (ret 4, `match_eng.cpp`) with the calls that touch things outside the engine injected (`0x6d6f70`
/// narrates, `0x702050` takes the kick, `0x7109f0` moves the goalkeeper): one step of the penalty shootout by state
/// `+0x8eb3`.
///
/// States 1 and 4 set up (zero the move, save and silence the commentator in `+0x8ea4`, five kicks per side in
/// `+0x8ea1`/`+0x8ea2`, side 1 or the drawn one starts, `0x6be520`) and go to 2 and 5.
///
/// States 2 and 5: with no kicks left comes sudden death (one each, `+0x8ea3`); the taker is the next one in the side's
/// list (`+0x1af` + 11 × side, `+0x8e9f` counts), the goalkeeper is the other side's (`0x6ce110`); it marks who took
/// the kick (`+0x8eda`); the strength comes from random draw(`+0x35` of the player sheet) and the kicks remaining, and
/// grows (× 2.25 with narration 0x2005 when the side needs to score, × 0.75 with 0x2006 when it can decide). With
/// `modo` 3 the kick (`0x700bd0` and `0x702050`) is taken directly; otherwise it positions the taker, the goalkeeper
/// and the others (`0x6f46b0`, `0x7160e0`), the kick direction by strength, and the goalkeeper reacts (`0x714d40`, up
/// to 4 times `0x7109f0`) before the kick. It passes the turn and, once the score is decided (`+0xf5bc` × `+0xf5f2`
/// with the kicks remaining), goes to 3 (mode 4) or 6.
///
/// States 3 and 6 narrate the result (0x2004), write the shootout score into the match (`+0x49`/`+0x4a`) and give the
/// commentator back.
///
/// [pt] { 0x6be710 } (ret 4, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x702050 cobra, 0x7109f0 move o goleiro): um passo da disputa de penaltis pelo estado
/// +0x8eb3. 1 e 4 preparam (zeram o lance, guardam e calam o narrador em +0x8ea4, cinco cobrancas
/// por lado em +0x8ea1/+0x8ea2, comeca o lado 1 ou o sorteado, 0x6be520) e vao para 2 e 5. 2 e 5: sem
/// cobrancas sobrando vem a morte subita (uma de cada, +0x8ea3); o batedor e o da vez na lista do lado
/// (+0x1af + 11 x lado, +0x8e9f conta), o goleiro e o do outro lado (0x6ce110); marca quem bateu
/// (+0x8eda), a forca sai de sorteio(+0x35 da ficha) e das cobrancas que faltam, e cresce (x2,25 com
/// narracao 0x2005 quando o lado precisa marcar, x0,75 com 0x2006 quando pode decidir). Com `modo` 3
/// o chute (0x700bd0 e 0x702050) sai direto; senao posiciona batedor, goleiro e os outros (0x6f46b0,
/// 0x7160e0), a direcao do chute pela forca, e o goleiro reage (0x714d40, ate 4 vezes 0x7109f0) antes
/// do chute. Passa a vez e, decidido o placar (+0xf5bc x +0xf5f2 com as cobrancas que faltam), vai
/// para 3 (modo 4) ou 6. 3 e 6 narram o resultado (0x2004), gravam o placar dos penaltis na partida
/// (+0x49/+0x4a) e devolvem o narrador.
pub(crate) unsafe fn penalty_shootout_step_with(m: usize, mode: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |o: usize| (m + o) as *mut u8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, u32::MAX]);
    };
    match read_i8(m + engine::SHOOTOUT_STATE) {
        1 | 4 => {
            let commentator = read_usize(m + engine::COMMENTATOR) as *mut u8;
            *((m + engine::PENDING_PLAY_CODE) as *mut u16) = 0xffff;
            *((m + engine::PENDING_PLAY_OFFENDER) as *mut u32) = 0;
            *((m + engine::PENDING_PLAY_VICTIM) as *mut u32) = 0;
            *b(engine::PENDING_PLAY_SERIOUS) = 0;
            *b(engine::PENDING_PLAY_KIND) = 0;
            for o in 0x475b..=0x475e {
                *b(o) = 0xff;
            }
            *b(engine::SAVED_COMMENTATOR_FLAG) = *commentator;
            *commentator = 0;
            *b(engine::SHOOTOUT_KICKING_SIDE) = if read_i8(m + engine::SHOOTOUT_STATE) == 1 { 1 } else { random(2) as u8 };
            *b(engine::BALL_TRANSIT_TICKS) = 0;
            for o in [0x477e, 0xf5aa, 0xf5a6, 0xf576, 0xf57a, 0x91de] {
                *((m + o) as *mut u32) = 0;
            }
            *b(engine::BALL_ROLL) = 0;
            *b(engine::BALL_HEIGHT) = 0;
            *b(engine::PASS_TARGET_Y) = 0xff;
            *b(engine::PASS_TARGET_X) = 0xff;
            *((m + engine::PASS_POWER) as *mut u16) = 0;
            *b(engine::SHOOTOUT_KICKS_LEFT_SIDE1) = 5;
            *b(engine::SHOOTOUT_KICKS_LEFT) = 5;
            *b(0x8ea0) = 0;
            *b(engine::SHOOTOUT_KICKS_TAKEN) = 0;
            let next = if read_i8(m + engine::SHOOTOUT_STATE) == 1 { 2 } else { 5 };
            exe::cm3_match_engine_choose_shootout_takers(m);
            *b(engine::SHOOTOUT_SUDDEN_DEATH) = 0;
            *b(engine::SHOOTOUT_STATE) = next;
        }
        2 | 5 => {
            *b(engine::RESTART_TYPE) = 0;
            if read_i8(m + engine::SHOOTOUT_KICKS_LEFT) == 0 && read_i8(m + engine::SHOOTOUT_KICKS_LEFT_SIDE1) == 0 {
                *b(engine::SHOOTOUT_KICKS_LEFT_SIDE1) = 1;
                *b(engine::SHOOTOUT_KICKS_LEFT) = 1;
                *b(engine::SHOOTOUT_SUDDEN_DEATH) = 1;
            }
            let side = read_i8(m + engine::SHOOTOUT_KICKING_SIDE);
            let lu = side as isize;
            let team = read_i8(m + ((0x8e9f + lu) as usize)) as i32 % 11;
            let who = read_i8(m + ((0x1af + 11 * lu + team as isize) as usize)) as isize;
            let j = (m as isize + 0x4796 + 0x1be * (who + 20 * lu)) as usize;
            let goal = exe::cm3_match_engine_get_goalkeeper(m, (side == 0) as i8);
            *b((0x8e9f + lu) as usize) = read_i8(m + ((0x8e9f + lu) as usize)).wrapping_add(1) as u8;
            exe::match_player_set_action_delay(j, 0);
            exe::match_player_set_action_delay(goal, 0);
            *((m as isize + read_i8(j + 0) as isize + 20 * lu + 0x8eda) as *mut u8) = 1;
            call(FN_MATCH_PLAYER_TAKE_POSSESSION, j, [0; 5]);
            *b(engine::BALL_HEIGHT) = 0;
            let restam = read_i8(m + ((0x8ea1 + lu) as usize)) as i32;
            let r = random(read_i8(*((j + match_player::PLAYER) as *const usize) + 0x35) as i32);
            let mut strength = (0xc8 - r.wrapping_mul(restam + 4)) << 1;
            let goals = |l: isize| *((m as isize + 0xf5bc + 0x36 * l) as *const u8) as i32;
            let other = (side == 0) as isize;
            let (meus, deles) = (goals(lu), goals(other));
            let factor = if deles > meus + restam - 1 {
                if read_i8(m + engine::SHOOTOUT_SUDDEN_DEATH) == 0 {
                    narrate(call, 0x2005, *((j + match_player::COMMENTARY_ID) as *const u8) as u32, side as u8 as u32);
                }
                Some(2.25)
            } else if meus + 1 > read_i8(m + ((0x8ea1 + other) as usize)) as i32 + deles {
                if read_i8(m + engine::SHOOTOUT_SUDDEN_DEATH) == 0 {
                    narrate(call, 0x2006, *((j + match_player::COMMENTARY_ID) as *const u8) as u32, side as u8 as u32);
                }
                Some(0.75)
            } else {
                None
            };
            if let Some(f) = factor {
                strength = strength.wrapping_add(random(ftol(strength as f64 * f)));
            }
            let (mut tipo, mut som) = (0x33i8, 0u16);
            if mode as u8 == 3 {
                let v = exe::match_player_get_shot_score(j, &mut tipo, &mut som) as i16 as i32;
                let r = random(strength);
                call(FN_MATCH_PLAYER_TAKE_SHOT, j, [tipo as u8 as u32, v.wrapping_sub(r) as u32, som as u32, 3, 0]);
            } else {
                let (dir, a_j, a_g, a_o) = if side == 0 { (0u16, 7u32, (4u32, 0xbu32), (0u32, 0u32)) } else { (0xb4, 4, (4, 0), (8, 0xb)) };
                *((j + match_player::FACING) as *mut u16) = dir;
                let mut v = 4u8;
                if read_i8(j + 0x2d) > 3 {
                    let s2 = random(2) as u8;
                    v = if (read_i8(j + match_player::PREFERRED_FOOT) == 0) == (side == 0) { 4u8.wrapping_sub(s2) } else { s2.wrapping_add(4) };
                }
                call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, j, [v as u32, a_j, 0, 0, 0]);
                call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, goal, [a_g.0, a_g.1, 0, 0, 0]);
                let base = m + engine::PLAYERS;
                for k in 0..20 {
                    for p in [base + 0x1be * k, base + 0x22d8 + 0x1be * k] {
                        if p != j && p != goal && read_i8(p + match_player::POSITION) >= 0 {
                            call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, p, [a_o.0, a_o.1, 0, 0, 0]);
                            exe::match_player_set_action_delay(p, 10);
                        }
                    }
                }
                let c = (0xc8 - random(strength)) as i16;
                *((j + match_player::BALL_CONTROL) as *mut i16) = c.clamp(-0xc8, 0xc8);
                let mut defendeu = 0u8;
                let mut vezes = 0i8;
                loop {
                    call(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE, goal, [0; 5]);
                    let active = read_i8(m + engine::RESTART_TYPE) == 0 && *b(engine::BALL_HEIGHT) != 0xff;
                    if active {
                        defendeu = call(FN_MATCH_PLAYER_DECIDE_BALL_ACTION, j, [4, 0, 0, 0, 0]) as u8;
                    }
                    exe::match_player_add_action_delay(j, 0xff);
                    exe::match_player_add_action_delay(goal, 0xff);
                    if !(read_i8(m + engine::RESTART_TYPE) == 0 && *b(engine::BALL_HEIGHT) != 0xff) {
                        break;
                    }
                    let n = vezes;
                    vezes = vezes.wrapping_add(1);
                    if n >= 4 || defendeu != 0 || read_usize(m + engine::BALL_HOLDER) != j {
                        break;
                    }
                }
                if read_usize(m + engine::BALL_HOLDER) == j && read_i8(m + engine::RESTART_TYPE) == 0 && *b(engine::BALL_HEIGHT) != 0xff {
                    tipo = -1;
                    let v = exe::match_player_get_shot_score(j, &mut tipo, &mut som) as i16 as i32;
                    call(FN_MATCH_PLAYER_TAKE_SHOT, j, [tipo as u8 as u32, v as u32, som as u32, 4, 0]);
                }
            }
            *b((0x8ea1 + lu) as usize) = read_i8(m + ((0x8ea1 + lu) as usize)).wrapping_sub(1) as u8;
            *b(engine::SHOOTOUT_KICKING_SIDE) = (side == 0) as u8;
            let (g0, g1) = (*b(engine::SIDE_STATS) as i32, *b(0xf5f2) as i32);
            if g0 > read_i8(m + engine::SHOOTOUT_KICKS_LEFT_SIDE1) as i32 + g1 || g1 > read_i8(m + engine::SHOOTOUT_KICKS_LEFT) as i32 + g0 {
                *b(engine::SHOOTOUT_STATE) = if mode as u8 == 4 { 3 } else { 6 };
            }
        }
        3 | 6 => {
            narrate(call, 0x2004, u32::MAX, (*b(engine::SIDE_STATS) <= *b(0xf5f2)) as u32);
            let game = Fixture(read_usize(m + engine::MATCH));
            game.set_home_penalties((*b(engine::SIDE_STATS)) as i8);
            game.set_away_penalties((*b(0xf5f2)) as i8);
            *b(engine::SHOOTOUT_STATE) = 0;
            *(read_usize(m + engine::COMMENTATOR) as *mut u8) = *b(engine::SAVED_COMMENTATOR_FLAG);
        }
        _ => {}
    }
}

/// One step of the penalty shoot-out in mode `modo` (taker and keeper chosen and positioned, shot power and keeper
/// reaction, score and end of shoot-out); thin wrapper over penalty_shootout_step_with with the real calls.
///
/// [pt] { 0x6be710 } (ret 4, match_eng.cpp): ver `penalty_shootout_step_with`.
pub(crate) unsafe extern "thiscall" fn penalties(m: usize, mode: u32) {
    penalty_shootout_step_with(m, mode, &mut |rotina, this, a| match rotina {
        0x6d6f70 => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_SHOT);
            f(this, a[0], a[1], a[2], a[3], 0, 0);
            0
        }
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0], a[1]);
            0
        }
        0x6f0630 | 0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
            f(this);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_DECIDE_BALL_ACTION);
            f(this, a[0]) as u32
        }
    })
}

/// `apply_foul` (match_eng.cpp) with the calls that touch things outside the engine injected (0x6d6f70 commentates,
/// 0x716e10 tires): applies the pending incident at `engine::PENDING_PLAY_KIND`, clears it and returns 1; 0 if none or discarded.
///
/// Fields: type; `engine::PENDING_PLAY_OFFENDER` who committed it, `engine::PENDING_PLAY_VICTIM` who suffered it, `engine::PENDING_PLAY_X`/`engine::PENDING_PLAY_Y` where, `engine::PENDING_PLAY_REASON`/`engine::PENDING_PLAY_SEVERITY`/`engine::PENDING_PLAY_SERIOUS` for
/// the punishment, `engine::PENDING_PLAY_CODE` the code. No type returns 0. With a set piece in progress (`engine::RESTART_TYPE`) and the incident from
/// the side that already had the advantage (`engine::POSSESSION_SIDE`) behind the marked position (`engine::BALL_Y`), it discards the incident and
/// returns 0; otherwise commentates 0x20f0.
///
/// Type 8: commentates 0x20ee, counts on the side's score (`engine::OFFSIDES`) and on the player (+0x13) and marks set piece 7.
/// Without a victim, the foul punishment (`exe::match_official_decide_booking`) and commentary 0x20fb; with one, the incident punishment (`exe::match_official_decide_foul_punishment`)
/// and commentary by code `engine::PENDING_PLAY_CODE` (0x20fc to 0x2107).
///
/// The returned type picks the card (0x1ff9 yellow, 0x1ff7/0x1ff8 red, 0x1ff5 or 0x1ff6 depending on the case, 0x210a
/// by severity against random(7) + 2). Type 7 and set piece 7; near the opposing area (`engine::PENDING_PLAY_X` from 2 to 6 and `engine::PENDING_PLAY_Y`
/// in the goal band) a penalty (5, 0x1f7a, -500 in +0x35 of the victim or the offender); otherwise a free kick (6,
/// 0x1f79).
///
/// With a card: 1 in 4 commentates 0x2109 (and 0x20f5 by severity), commentates the card, and a sending-off removes the
/// player (0x6cda20). Outside the set-piece codes, 1 in 9 commentates 0x20df (1 in 3 also 0x20e0) and, with a
/// sending-off or penalty, marks +0x16 of the offender (1 in 7 commentates 0x20d9); the offender's team is worn down by
/// the punishment's weight (`exe::match_player_scale_anger`: the offender himself by the weight, those outside p / 6 + 0.83, the others (p +
/// 1) / 2) and the offender tires (0x716e10). Clears the incident and returns 1.
///
/// [pt] { 0x6bac40 } (match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70 narra,
/// 0x716e10 cansa): aplica o lance pendente em +0x475a (tipo; +0x4761 quem fez, +0x4765 quem sofreu,
/// +0x475b/+0x475c onde, +0x475d/+0x475e/+0x4769 para a punicao, +0x475f o codigo). Sem tipo devolve
/// 0. Com bola parada em andamento (+0x8eb2) e o lance do lado que ja tinha a vantagem (+0x8eae)
/// atras da posicao marcada (+0x8ea8), descarta o lance e devolve 0; senao narra 0x20f0. Tipo 8:
/// narra 0x20ee, conta no placar do lado (+0xf5ca) e no jogador (+0x13) e marca a bola parada 7. Sem
/// quem sofreu, a punicao da falta (0x6e9630) e a narracao 0x20fb; com ele, a punicao do lance
/// (0x6ea4e0) e a narracao pelo codigo +0x475f (0x20fc a 0x2107). O tipo devolvido escolhe o cartao
/// (0x1ff9 amarelo, 0x1ff7/0x1ff8 vermelho, 0x1ff5 ou 0x1ff6 conforme o caso, 0x210a pela severidade
/// contra sorteio(7) + 2). Tipo 7 e bola parada 7; perto da area adversaria (+0x475b de 2 a 6 e
/// +0x475c na faixa do gol) e penalti (5, 0x1f7a, -500 em +0x35 de quem sofreu ou fez); senao falta
/// (6, 0x1f79). Com cartao: 1 em 4 narra 0x2109 (e 0x20f5 pela severidade), narra o cartao e a
/// expulsao tira o jogador (0x6cda20). Fora dos codigos de bola parada, 1 em 9 narra 0x20df (1 em 3
/// tambem 0x20e0) e, com expulsao ou penalti, marca +0x16 de quem fez (1 em 7 narra 0x20d9); o time
/// de quem fez se desgasta pelo peso da punicao (0x716450: o proprio pelo peso, os de fora p / 6 +
/// 0,83, os outros (p + 1) / 2) e quem fez cansa (0x716e10). Limpa o lance e devolve 1.
pub(crate) unsafe fn apply_pending_foul_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |o: usize| (m + o) as *mut u8;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, a5]);
    };
    let clear = || {
        *((m + engine::PENDING_PLAY_CODE) as *mut u16) = 0xffff;
        *((m + engine::PENDING_PLAY_OFFENDER) as *mut usize) = 0;
        *((m + engine::PENDING_PLAY_VICTIM) as *mut usize) = 0;
        *b(engine::PENDING_PLAY_SERIOUS) = 0;
        *b(engine::PENDING_PLAY_KIND) = 0;
        for o in 0x475b..=0x475e {
            *b(o) = 0xff;
        }
    };
    if *b(engine::PENDING_PLAY_KIND) == 0 {
        return 0;
    }
    if *b(engine::RESTART_TYPE) != 0 {
        let side = read_i8(read_usize(m + engine::PENDING_PLAY_OFFENDER) + 0x27);
        if read_i8(m + engine::POSSESSION_SIDE) as i32 == (side == 0) as i32 {
            let (marcada, aqui) = (read_i8(m + engine::BALL_Y), read_i8(m + engine::PENDING_PLAY_Y));
            if (aqui < marcada && side == 1) || (marcada < aqui && side == 0) {
                clear();
                return 0;
            }
        }
        narrate(call, 0x20f0, u32::MAX, side as u8 as u32, u32::MAX);
    }
    let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
    if *b(engine::PENDING_PLAY_KIND) == 8 {
        narrate(call, 0x20ee, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
        let scoreboard = (m as isize + read_i8(j + match_player::SIDE) as isize * 0x36 + 0xf5ca) as *mut u8;
        *scoreboard = (*scoreboard).wrapping_add(1);
        *((j + match_player::OFFSIDES) as *mut u8) = (read_u8(j + match_player::OFFSIDES) as u32).wrapping_add(1) as u8;
        *b(engine::BALL_X) = *b(engine::PENDING_PLAY_X);
        *b(engine::BALL_Y) = *b(engine::PENDING_PLAY_Y);
        *b(engine::RESTART_TYPE) = 7;
        *b(engine::POSSESSION_SIDE) = zero(j) as u8;
        *((m + engine::BALL_HOLDER) as *mut u32) = 0;
        clear();
        return 1;
    }
    if j == 0 {
        clear();
        return 1;
    }
    let sofreu = read_usize(m + engine::PENDING_PLAY_VICTIM) != 0;
    let mut weight = 0f32;
    if !sofreu {
        exe::match_official_decide_booking(m + engine::REFEREE, j, b(engine::PENDING_PLAY_KIND), read_i8(m + engine::PENDING_PLAY_REASON), read_i8(m + engine::PENDING_PLAY_SEVERITY), *b(engine::PENDING_PLAY_SERIOUS), &mut weight);
        let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
        narrate(call, 0x20fb, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
    } else {
        exe::match_official_decide_foul_punishment(m + engine::REFEREE, j, read_usize(m + engine::PENDING_PLAY_VICTIM), b(engine::PENDING_PLAY_KIND), read_i8(m + engine::PENDING_PLAY_REASON), read_i8(m + engine::PENDING_PLAY_SEVERITY), *b(engine::PENDING_PLAY_SERIOUS), &mut weight, 0);
        let code = match *((m + engine::PENDING_PLAY_CODE) as *const i16) {
            1 => 0x2104,
            0xc => 0x2107,
            0x13 | 0x21 => 0x2101,
            0x15 => 0x2103,
            0x22 => 0x2105,
            0x29 => 0x2102,
            0x2a => 0x20fe,
            0x2b => 0x20ff,
            0x2c => 0x2100,
            0x2d => 0x20fd,
            0x2e => 0x2106,
            _ => 0x20fc,
        };
        let (j, v) = (read_usize(m + engine::PENDING_PLAY_OFFENDER), read_usize(m + engine::PENDING_PLAY_VICTIM));
        narrate(call, code, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), (read_u8(v + match_player::COMMENTARY_ID) as u32) | (read_u8(v + match_player::SIDE) as u32) << 8);
    }
    let kind = *b(engine::PENDING_PLAY_KIND) as i8;
    let card: i32 = match kind {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        6 if !sofreu => 0x1ff5,
        5 if sofreu => 0x1ff6,
        _ => {
            if read_i8(m + engine::PENDING_PLAY_SEVERITY) as i32 > random(7) + 2 { 0x210a } else { -1 }
        }
    };
    let sound: u32;
    if *b(engine::PENDING_PLAY_KIND) == 7 {
        sound = 0x1f79;
        *b(engine::RESTART_TYPE) = 7;
    } else {
        let (x, y) = (read_i8(m + engine::PENDING_PLAY_X), read_i8(m + engine::PENDING_PLAY_Y));
        let area = (2..=6).contains(&x) && if read_i8(read_usize(m + engine::PENDING_PLAY_OFFENDER) + 0x27) == 1 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
        if area {
            *b(engine::RESTART_TYPE) = 5;
            let who = if sofreu { read_usize(m + engine::PENDING_PLAY_VICTIM) } else { read_usize(m + engine::PENDING_PLAY_OFFENDER) };
            *((m + engine::FOULED_PLAYER) as *mut usize) = if sofreu { who } else { 0 };
            let d = (who + match_player::MATCH_RATING) as *mut i16;
            *d = (*d).wrapping_sub(500);
            sound = 0x1f7a;
        } else {
            sound = 0x1f79;
            *b(engine::RESTART_TYPE) = 6;
        }
    }
    *b(engine::BALL_Y) = *b(engine::PENDING_PLAY_Y);
    *b(engine::BALL_X) = *b(engine::PENDING_PLAY_X);
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *b(engine::POSSESSION_SIDE) = zero(read_usize(m + engine::PENDING_PLAY_OFFENDER)) as u8;
    if sound == 0x1f7a {
        let a4 = if sofreu { (read_u8((read_usize(m + engine::PENDING_PLAY_VICTIM)) + 0x27) as u32) } else { zero(read_usize(m + engine::PENDING_PLAY_OFFENDER)) };
        narrate(call, 0x1f7a, u32::MAX, a4, u32::MAX);
    }
    if card >= 0 {
        if random(4) == 0 {
            let a = random(read_i8(m + engine::PENDING_PLAY_SEVERITY) as i32);
            if a > random(0x14) {
                narrate(call, 0x20f5, 0, 0, u32::MAX);
            }
            let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
            narrate(call, 0x2109, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        }
        let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
        narrate(call, card as u32, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        if matches!(*b(engine::PENDING_PLAY_KIND), 3..=6) {
            player_left_pitch_with(m, read_usize(m + engine::PENDING_PLAY_OFFENDER), 0, call);
        }
    }
    if sound != 0x1f7a {
        let a4 = if sofreu { (read_u8((read_usize(m + engine::PENDING_PLAY_VICTIM)) + 0x27) as u32) } else { zero(read_usize(m + engine::PENDING_PLAY_OFFENDER)) };
        narrate(call, sound, u32::MAX, a4, u32::MAX);
    }
    // the set-piece codes (0x25, 0x2a to 0x32, 0x60 and 0x61) stop here
    let k = (read_i8(m + engine::PENDING_PLAY_CODE) as i32 - 0x25) as u32;
    if k <= 0x3c && matches!(k, 0 | 5..=13 | 0x3b | 0x3c) {
        clear();
        return 1;
    }
    // without the fouled player, the raw side of the offender; with him, ”lado == 0” (0x20df by the fouled player’s side)
    let side_of = |p: usize| if sofreu { zero(p) } else { (read_u8(p + match_player::SIDE) as u32) };
    if random(9) == 0 {
        narrate(call, 0x20df, u32::MAX, side_of(if sofreu { read_usize(m + engine::PENDING_PLAY_VICTIM) } else { read_usize(m + engine::PENDING_PLAY_OFFENDER) }), u32::MAX);
        if random(3) == 0 {
            narrate(call, 0x20e0, u32::MAX, side_of(read_usize(m + engine::PENDING_PLAY_OFFENDER)), u32::MAX);
        }
        if matches!(*b(engine::PENDING_PLAY_KIND), 3..=6) || sound == 0x1f7a {
            *((read_usize(m + engine::PENDING_PLAY_OFFENDER) + 0x16) as *mut u8) = 1;
            if random(7) == 0 {
                narrate(call, 0x20d9, u32::MAX, side_of(read_usize(m + engine::PENDING_PLAY_OFFENDER)), u32::MAX);
            }
        }
    }
    let j = read_usize(m + engine::PENDING_PLAY_OFFENDER);
    let block = m + engine::PLAYERS + 0x22d8 * read_i8(j + match_player::SIDE) as isize as usize;
    for i in 0..20 {
        let p = block + 0x1be * i;
        if *((p + match_player::STAFF) as *const usize) == 0 {
            continue;
        }
        let f = if p == j {
            weight
        } else if read_i8(p + match_player::POSITION) < 0 {
            (weight as f64 * 0.16666666666666666 + 0.83) as f32
        } else {
            ((weight as f64 + 1.0) * 0.5) as f32
        };
        exe::match_player_scale_anger(p, f);
    }
    call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, read_usize(m + engine::PENDING_PLAY_OFFENDER), [0; 5]);
    clear();
    1
}

/// applies the pending incident in `engine::PENDING_PLAY_KIND` (type; `engine::PENDING_PLAY_OFFENDER` who committed it, `engine::PENDING_PLAY_VICTIM` who suffered it, `engine::PENDING_PLAY_X`/`engine::PENDING_PLAY_Y`
/// where, `engine::PENDING_PLAY_CODE` the code). With no type it returns 0; with a set piece under way and the incident behind the marked
/// position (`engine::BALL_Y`) of the side that had the advantage, it discards it and returns 0 (otherwise it narrates 0x20f0).
/// Type 8: narrates 0x20ee, counts in the side's score (`engine::OFFSIDES`) and the player's (+0x13), set piece 7. With no victim:
/// foul punishment (`exe::match_official_decide_booking`) and 0x20fb; with one: incident punishment (`exe::match_official_decide_foul_punishment`) and narration by code (0x20fc to
/// 0x2107). The returned type picks the card (0x1ff5 to 0x1ff9, or 0x210a by severity); near the opponent's box it
/// becomes a penalty (`engine::RESTART_TYPE` = 5, 0x1f7a, −500 in +0x35), otherwise a free kick (6, 0x1f79). With a card, 1 in 4
/// narrates 0x2109 (and 0x20f5), narrates the card and a sending-off takes the player off (0x6cda20). Outside the
/// set-piece codes: narrations 0x20df/0x20e0/0x20d9 by draw, the offender's team wears down by the punishment weight
/// (`exe::match_player_scale_anger`) and the offender tires (0x716e10). Clears the incident and returns 1. Compared through the tape:
/// narration and tiring (0x716e10) are units
/// Original: `apply_foul` (0x6bac40, match_eng.cpp (~3.2 KB), thiscall, returns al).
///
/// [pt] { 0x6bac40 } (match_eng.cpp): ver `apply_pending_foul_with`.
pub(crate) unsafe extern "thiscall" fn apply_foul(m: usize) -> u8 {
    apply_pending_foul_with(m, &mut |rotina, this, a| {
        match rotina {
            0x6d6f70 => add_match_event(this, [a[0], a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, 0]),
            0x716e10 | 0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
                f(this, a[0]);
            }
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

/// { 0x6c9cd0 } (ret 8, match_eng.cpp) with the calls that touch things outside the match engine injected (0x6d6f70
/// commentates, 0x716e10 tires, `exe::cm3_match_engine_ball_out_of_play` ball out, `take_ball` ball control, `decide_player` the winner's next action; and those
/// inside the injury, the save and leaving the pitch): the loose ball at square (x, y) contested by the challengers of
/// both sides.
///
/// Challengers come from `exe::cm3_match_engine_get_opposition_challenge` (action 0x15, the rating corrected by the angle of the direction against `engine::BALL_DIRECTION`).
/// Off the pitch: ball out (`exe::cm3_match_engine_ball_out_of_play`). With no challenger, it stores the square and returns 0. The one with the higher
/// rating wins; the injury in the 50-50 (0x6cfe10) comes right away.
///
/// With both, the referee looks at the winner's foul (`exe::match_official_judge_tackle`, +4 of stamina before; 0x2168 in the area with `engine::LIVE_MATCH`)
/// and then the loser's; with no foul, `exe::match_official_check_foul` (action "playing on") may give one. A foul given goes to `engine::PENDING_PLAY_KIND`
/// (pending action) when random draw(distance to goal × 10 + 2) == 0 with no risk, or is punished at once (`exe::match_official_decide_foul_punishment` /
/// `exe::match_official_decide_booking`): commentary by action (0x1fe8 to 0x1ff4), card, set piece 7, penalty (5) or free kick (6), sending-off
/// (0x6cda20), the usual commentary, the offending side wears down (`exe::match_player_scale_anger`) and the offender tires; the special
/// actions come before the wear.
///
/// With no foul, the clean contest: in action 1 the goalkeeper save (0x70d430, which may end it); with both, commentary
/// by the reading (+0xad: 0x2164, 0x214f or 0x1f5d), it counts attempts (+7, +8, +9 with danger) and changes stamina.
/// The winner marks the action, turns towards the play (`exe::match_player_set_facing_direction`), controls the ball (`take_ball`) with +0x19c = rating −
/// the other's rating − 225 (±200), the loser adds the challenge's second byte to +0x2b (`exe::match_player_add_action_delay`) and the winner
/// carries on (`decide_player`). Returns the winner.
///
/// The mark is set in the advantage before `exe::match_official_judge_tackle` (which only writes 0 or 1).
///
/// [pt] { 0x6c9cd0 } (ret 8, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x6cd490 bola fora, 0x6f0630 dominio, 0x7103e0 o lance de quem ganhou; e
/// as de dentro da lesao, da defesa e da saida de campo): a bola solta na casa (x, y) disputada pelos
/// desafiantes dos dois lados (0x6c87e0, lance 0x15, a nota corrigida pelo angulo da direcao contra
/// +0x8ed8). Fora do campo e bola fora (0x6cd490). Sem desafiante, guarda a casa e devolve 0. O de
/// nota maior ganha; a lesao na dividida (0x6cfe10) vem logo. Com os dois, o arbitro olha a falta do
/// vencedor (0x6e9820, +4 de folego antes; 0x2168 na area com +0x8ebb) e depois a do perdedor; sem
/// falta, a 0x6e9540 (lance "valendo") pode marcar uma. A falta marcada vai para +0x475a (lance
/// pendente) por sorteio(distancia ao gol x 10 + 2) == 0 sem risco, ou e punida na hora (0x6ea4e0 /
/// 0x6e9630): narracao pelo lance (0x1fe8 a 0x1ff4), cartao, bola parada 7, penalti (5) ou falta (6),
/// expulsao (0x6cda20), as narracoes de sempre, o time de quem fez se desgasta (0x716450) e quem fez
/// cansa; os lances especiais saem antes do desgaste. Sem falta, a disputa limpa: no lance 1 a
/// defesa (0x70d430, que pode encerrar); com os dois, narra pela leitura (+0xad: 0x2164, 0x214f ou
/// 0x1f5d), conta tentativas (+7, +8, +9 com perigo) e mexe no folego. O vencedor marca o lance,
/// vira para a jogada (0x6f4920), domina (0x6f0630) com +0x19c = nota - nota do outro - 225
/// (+-200), o perdedor soma o segundo byte do desafio a +0x2b (0x7160d0) e o vencedor segue (0x7103e0).
/// Devolve o vencedor.
/// Marca posta na vantagem antes da 0x6e9820 (que so escreve 0 ou 1).
pub(crate) const NO_ANSWER_MARK: u8 = 0xa5;
/// A disputa em andamento leu a vantagem sem resposta do juiz (lixo da pilha na original).
pub(crate) static ADVANTAGE_UNANSWERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// Divergencias da disputa explicadas pelo lixo da pilha (contadas a parte, nao como divergencia).
pub(crate) static CONTEST_STACK_GARBAGE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Contest for the ball at (x, y) with outside calls injected: picks the challengers of both sides, judges tackle and
/// foul through the referee, narrates and applies the outcome; returns the winner [inference].
pub(crate) unsafe fn tackle_contest_with(m: usize, x: i8, y: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> usize {
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    let special = |l: i8| {
        let k = (l as i32 - 0x25) as u32;
        k <= 0x3c && matches!(k, 0 | 5..=13 | 0x3b | 0x3c)
    };
    if x < 0 || y < 0 || x >= 9 || y >= 12 {
        crate::comparison::branch("contest out of play");
        call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [(read_u8(m + engine::LAST_POSSESSION_SIDE) as u32), 0, 0, 0, 0]);
        return 0;
    }
    // the challengers of both sides; the rating (word, zero-extended) corrected by the angle
    let side_of = |side: i8, a: &mut u8, bb: &mut u8, lance: &mut i8, arr: &mut i8| -> (usize, i32) {
        let mut rating = 0i16;
        let e = exe::cm3_match_engine_get_opposition_challenge(m, 0, side, 0x15, a, bb, &mut rating, lance, arr, 0);
        let mut v = rating as u16 as i32;
        if e != 0 {
            let mut d = (exe::match_player_get_facing_angle(e) as i16 as i32 - *w(m, engine::BALL_DIRECTION) as i32).abs();
            if d as i16 > 0xb4 {
                d = 0x168 - d;
            }
            v = v + d * 2 - 0x168;
        }
        (e, v)
    };
    let (mut a1, mut b1, mut lance1, mut arr1) = (0u8, 0u8, -1i8, 0i8);
    let (mut a0, mut b0, mut lance0, mut arr0) = (0u8, 0u8, -1i8, 0i8);
    let (e1, nota1) = side_of(1, &mut a1, &mut b1, &mut lance1, &mut arr1);
    let (e0, nota0) = side_of(0, &mut a0, &mut b0, &mut lance0, &mut arr0);
    let (wv, l, lance_w, lance_l, nota_w, nota_l, arr_w, arr_l, b_w) = if nota1 as i16 > nota0 as i16 {
        (e1, e0, lance1, lance0, nota1, nota0, arr1, arr0, b1)
    } else {
        (e0, e1, lance0, lance1, nota0, nota1, arr0, arr1, b0)
    };
    if wv == 0 {
        crate::comparison::branch("contest without challenger");
        *b(m, engine::BALL_X) = x as u8;
        *b(m, engine::BALL_Y) = y as u8;
        *((m + engine::PASSER) as *mut u32) = 0;
        return 0;
    }
    tackle_injury_with(m, wv, lance_w as i16, l, lance_l as i16, 0, call);
    let arb = m + engine::REFEREE;
    let (mut card, mut grav, mut vantagem) = (0u8, 0i8, 0u8);
    let mut kind: u8;
    let mut weight = 0f32;
    // the awarded foul stays pending (+0x475a) if there is none yet
    let pendente = |kind: u8, lance: i8, who: usize, other: usize, card: u8, grav: i8| {
        crate::comparison::branch("contest pending foul");
        if *((m + engine::PENDING_PLAY_OFFENDER) as *const usize) == 0 {
            *w(m, engine::PENDING_PLAY_CODE) = lance as i16;
            *b(m, engine::PENDING_PLAY_KIND) = kind;
            *b(m, engine::PENDING_PLAY_X) = x as u8;
            *((m + engine::PENDING_PLAY_OFFENDER) as *mut usize) = who;
            *((m + engine::PENDING_PLAY_VICTIM) as *mut usize) = other;
            *b(m, engine::PENDING_PLAY_SERIOUS) = 0;
            *b(m, engine::PENDING_PLAY_Y) = y as u8;
            *b(m, engine::PENDING_PLAY_REASON) = card;
            *b(m, engine::PENDING_PLAY_SEVERITY) = grav as u8;
        }
    };
    let punish_now = |d: usize| random(ftol(exe::match_player_distance_to_opponent_goal(d) as f64 * 10.0 + 2.0)) != 0;
    // mode 1: foul by the winner on the loser; 2: by the loser on the winner; 3: only one challenger
    let mut punido: Option<(u8, usize, usize)> = None;
    'faltas: {
        if l != 0 {
            *w(wv, match_player::MATCH_RATING) = (*w(wv, match_player::MATCH_RATING)).wrapping_add(4);
            // 0x6e9820 may exit before writing the advantage (foul in the area, impatient referee): the
            // the original then reads what was left on the stack; the copy uses 0 (Lucas’s decision, 14/10/2026)
            vantagem = NO_ANSWER_MARK;
            kind = exe::match_official_judge_tackle(arb, wv, l, lance_w, -1, &mut card, &mut grav, &mut vantagem);
            if vantagem == NO_ANSWER_MARK {
                crate::comparison::branch("contest advantage without reply");
                ADVANTAGE_UNANSWERED.store(true, std::sync::atomic::Ordering::Relaxed);
                vantagem = 0;
            }
            if read_i8(m + engine::LIVE_MATCH) != 0 && (exe::match_player_is_in_penalty_area(wv, read_i8(wv + match_player::SIDE) as u8) as u32) as u8 != 0
                && (special(lance_w) || vantagem != 0 || random(*w(wv, 0x31) as i32) < -10)
                && random(3) != 0
            {
                narrate(call, 0x2168, (read_u8(wv + match_player::COMMENTARY_ID) as u32), (read_u8(wv + match_player::SIDE) as u32), u32::MAX);
            }
            if kind != 0 {
                crate::comparison::branch("contest foul by the winner");
                if punish_now(wv) || arr_w != 0 {
                    exe::match_official_decide_foul_punishment(arb, wv, l, &mut kind, card as i8, grav, arr_w as u8, &mut weight, 0);
                    if kind != 0 {
                        punido = Some((1, wv, l));
                        break 'faltas;
                    }
                } else {
                    pendente(kind, lance_w, wv, l, card, grav);
                }
            } else {
                kind = exe::match_official_check_foul(arb, wv, lance_w as i16 as u16, &mut card, &mut grav as *mut i8 as *mut u8);
                if kind != 0 {
                    if random(10) != 0 {
                        exe::match_official_decide_booking(arb, wv, &mut kind, card as i8, grav, 0, &mut weight);
                        if kind != 0 {
                            punido = Some((1, wv, l));
                            break 'faltas;
                        }
                    } else {
                        pendente(kind, lance_w, wv, 0, card, grav);
                    }
                }
            }
            kind = exe::match_official_judge_tackle(arb, l, wv, lance_l, -1, &mut card, &mut grav, &mut vantagem);
            if kind != 0 {
                crate::comparison::branch("contest foul by the loser");
                if punish_now(l) || arr_l != 0 {
                    exe::match_official_decide_foul_punishment(arb, l, wv, &mut kind, card as i8, grav, arr_l as u8, &mut weight, 0);
                    if kind != 0 {
                        punido = Some((2, l, wv));
                    }
                } else {
                    pendente(kind, lance_l, l, wv, card, grav);
                }
            } else {
                kind = exe::match_official_check_foul(arb, l, lance_l as i16 as u16, &mut card, &mut grav as *mut i8 as *mut u8);
                if kind != 0 {
                    if random(10) != 0 {
                        exe::match_official_decide_booking(arb, l, &mut kind, card as i8, grav, 0, &mut weight);
                        if kind != 0 {
                            punido = Some((2, l, wv));
                        }
                    } else {
                        pendente(kind, lance_l, l, 0, card, grav);
                    }
                }
            }
        } else {
            crate::comparison::branch("contest only one challenger");
            kind = exe::match_official_check_foul(arb, wv, lance_w as i16 as u16, &mut card, &mut grav as *mut i8 as *mut u8);
            if kind != 0 {
                if random(10) != 0 {
                    exe::match_official_decide_booking(arb, wv, &mut kind, card as i8, grav, 0, &mut weight);
                    if kind != 0 {
                        punido = Some((3, wv, 0));
                    }
                } else {
                    pendente(kind, lance_w, wv, 0, card, grav);
                }
            }
        }
    }
    if let Some((modo, f, v)) = punido {
        crate::comparison::branch(["", "contest punida modo 1", "contest punida modo 2", "contest punida modo 3"][modo as usize]);
        let lance = if modo == 2 { lance_l } else { lance_w };
        // the foul commentary by the move of whoever committed it
        let code = if modo == 3 {
            0x1fe8
        } else {
            match lance {
                1 => 0x1ff1,
                0xc => 0x1ff4,
                0x15 => 0x1ff0,
                0x25 => 0x1fe8,
                0x2e => 0x1ff3,
                _ => 0x1fe9,
            }
        };
        if code == 0x1fe8 {
            narrate(call, code, (read_u8(f + match_player::COMMENTARY_ID) as u32), (read_u8(f + match_player::SIDE) as u32), u32::MAX);
        } else {
            narrate(call, code, (read_u8(f + match_player::COMMENTARY_ID) as u32), (read_u8(f + match_player::SIDE) as u32), (read_u8(v + match_player::COMMENTARY_ID) as u32) | (read_u8(v + match_player::SIDE) as u32) << 8);
        }
        let colour: i32 = match kind {
            2 => 0x1ff9,
            3 => 0x1ff7,
            4 => 0x1ff8,
            5 if modo != 3 => 0x1ff6,
            6 => 0x1ff5,
            _ => {
                if grav as i32 > random(7) + 2 { 0x210a } else { -1 }
            }
        };
        // the narrated side: in mode 1 that of who conceded (raw); in the others, ”who scored is on side 0”
        let lado_narrado = if modo == 1 { (read_u8(v + match_player::SIDE) as u32) } else { zero(f) };
        let sound: u32;
        if kind == 7 {
            sound = 0x1f79;
            *b(m, engine::RESTART_TYPE) = 7;
        } else if (exe::match_player_is_in_penalty_area(f, read_i8(f + match_player::SIDE) as u8) as u32) as u8 != 0 {
            if colour == -1 {
                match random(4) {
                    0 => narrate(call, 0x20e9, u32::MAX, lado_narrado, u32::MAX),
                    1 => narrate(call, 0x20ec, u32::MAX, lado_narrado, u32::MAX),
                    _ => {}
                }
            }
            crate::comparison::branch("contest penalty");
            sound = 0x1f7a;
            *b(m, engine::RESTART_TYPE) = 5;
            *((m + engine::FOULED_PLAYER) as *mut usize) = v;
            *w(f, match_player::MATCH_RATING) = (*w(f, match_player::MATCH_RATING)).wrapping_sub(500);
        } else {
            sound = 0x1f79;
            *b(m, engine::RESTART_TYPE) = 6;
        }
        *b(m, engine::BALL_X) = x as u8;
        *b(m, engine::BALL_Y) = y as u8;
        *b(m, engine::POSSESSION_SIDE) = zero(f) as u8;
        *((m + engine::BALL_HOLDER) as *mut u32) = 0;
        if sound == 0x1f7a {
            narrate(call, 0x1f7a, u32::MAX, lado_narrado, u32::MAX);
        }
        let sent_off = matches!(kind, 3..=6);
        if colour > -1 {
            if random(4) == 0 {
                let a = random(grav as i32);
                if a > random(0x14) {
                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                }
                narrate(call, 0x2109, (read_u8(f + match_player::COMMENTARY_ID) as u32), (read_u8(f + match_player::SIDE) as u32), u32::MAX);
            }
            narrate(call, colour as u32, (read_u8(f + match_player::COMMENTARY_ID) as u32), (read_u8(f + match_player::SIDE) as u32), u32::MAX);
            if sent_off {
                crate::comparison::branch("contest sending off");
                player_left_pitch_with(m, f, 0, call);
            }
        }
        if sound != 0x1f7a {
            narrate(call, sound, u32::MAX, lado_narrado, u32::MAX);
        }
        if special(lance) {
            crate::comparison::branch("contest special play punished");
            return 0;
        }
        // in mode 1 the extra commentaries look at who conceded (”side 0”) and mark his +0x16
        let (alvo, lado_extra) = if modo == 1 { (v, zero(v)) } else { (f, (read_u8(f + match_player::SIDE) as u32)) };
        if random(9) == 0 {
            narrate(call, 0x20df, u32::MAX, lado_extra, u32::MAX);
            if random(3) == 0 {
                narrate(call, 0x20e0, u32::MAX, lado_extra, u32::MAX);
            }
            if sent_off || sound == 0x1f7a {
                *b(alvo, match_player::SENT_OFF) = 1;
                if random(7) == 0 {
                    narrate(call, 0x20d9, u32::MAX, lado_extra, u32::MAX);
                }
            }
        }
        let block = m + engine::PLAYERS + 0x22d8 * read_i8(f + match_player::SIDE) as isize as usize;
        for i in 0..20 {
            let p = block + 0x1be * i;
            if *((p + match_player::STAFF) as *const usize) == 0 {
                continue;
            }
            let factor = if p == f {
                weight
            } else if read_i8(p + match_player::POSITION) < 0 {
                (weight as f64 * 0.16666666666666666 + 0.83) as f32
            } else {
                ((weight as f64 + 1.0) * 0.5) as f32
            };
            exe::match_player_scale_anger(p, factor);
        }
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, f, [0; 5]);
        return 0;
    }
    // a disputa limpa
    if lance_w == 1 {
        crate::comparison::branch("contest defence");
        let strength = if l == 0 { nota_w + 0x11 } else { nota_w - nota_l };
        let r = save_shot_injected(wv, strength as i16, &mut |rotina, this, a| {
            if rotina == 0x6d6f70 {
                // the defense narrates with the fifth field whole and the sixth -1
                call(rotina, this, [a[0], a[1], a[2], a[3], (a[4] & 0xff) | 0xff00])
            } else {
                call(rotina, this, a)
            }
        });
        if r == 0 {
            return 0;
        }
    } else if l != 0 {
        let r = random(6);
        let code = if ((r as f64 + read_f32(wv + match_player::JUMPING) as f64) + 3.0) < read_f32(l + match_player::JUMPING) as f64 {
            0x2164
        } else if exe::match_player_get_position(l) == 1 && random(5) != 0 {
            0x214f
        } else {
            0x1f5d
        };
        crate::comparison::branch(match code { 0x2164 => "contest leitura 2164", 0x214f => "contest leitura 214f", _ => "contest leitura 1f5d" });
        narrate(call, code, (read_u8(wv + match_player::COMMENTARY_ID) as u32), (read_u8(wv + match_player::SIDE) as u32), (read_u8(l + match_player::COMMENTARY_ID) as u32) | (read_u8(l + match_player::SIDE) as u32) << 8);
        *b(wv, 7) = (*b(wv, 7)).wrapping_add(1);
        *b(wv, 8) = (*b(wv, 8)).wrapping_add(1);
        *w(wv, match_player::MATCH_RATING) = (*w(wv, match_player::MATCH_RATING)).wrapping_add(0x28);
        *b(l, 7) = (*b(l, 7)).wrapping_add(1);
        *w(l, match_player::MATCH_RATING) = (*w(l, match_player::MATCH_RATING)).wrapping_add(if exe::match_player_get_position(l) == 1 { -600 } else { -10 });
        if exe::match_player_is_last_defender(wv) != 0 {
            *b(wv, 9) = (*b(wv, 9)).wrapping_add(1);
            *w(wv, match_player::MATCH_RATING) = (*w(wv, match_player::MATCH_RATING)).wrapping_add(300);
        }
    }
    *w(wv, match_player::ACTION) = lance_w as i16;
    let k = random(200);
    let far = read_i8(wv + match_player::MOMENT_AGILITY) as i32 * read_i8(wv + match_player::MOMENT_ANTICIPATION) as i32 > k
        || (exe::match_player_distance_to_opponent_goal(wv) as f64) < 2.0
        || exe::cm3_match_engine_is_path_ahead_clear(m, wv, 0) != 0;
    if !far {
        crate::comparison::branch("contest points to the marked play");
        exe::match_player_set_facing_direction(wv, x, y, read_i8(m + engine::TRANSIT_ORIGIN_X), read_i8(m + engine::TRANSIT_ORIGIN_Y));
    } else if read_i8(wv + match_player::SIDE) == 0 {
        exe::match_player_set_facing_direction(wv, x, y, 4, if y < 9 { y + 3 } else { 11 });
    } else {
        exe::match_player_set_facing_direction(wv, x, y, 4, if y > 3 { y - 3 } else { 0 });
    }
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, wv, [0; 5]);
    *w(wv, match_player::BALL_CONTROL) = (nota_w - nota_l - 225) as i16;
    *w(wv, match_player::BALL_CONTROL) = (*w(wv, match_player::BALL_CONTROL)).clamp(-200, 200);
    if l != 0 {
        exe::match_player_add_action_delay(l, b_w);
    }
    call(FN_MATCH_PLAYER_TAKE_TURN, wv, [0; 5]);
    let _ = (a0, a1);
    wv
}

/// Match engine: the contest for the ball at cell (x, y): picks the challengers from both sides, judges tackle and foul
/// through the referee, narrates and applies the outcome; returns the winner [inference]. Wrapper of
/// `tackle_contest_with` with the real outside calls.
///
/// [pt] { 0x6c9cd0 } (ret 8, match_eng.cpp): ver `tackle_contest_with`.
pub(crate) unsafe extern "thiscall" fn contest(m: usize, x: u32, y: u32) -> usize {
    tackle_contest_with(m, x as i8, y as i8, &mut |rotina, this, a| dispatch_tackle_calls(rotina, this, a))
}

/// As bolas paradas que a `restart_play` despacha por `engine::RESTART_TYPE` (2 lateral, 3 escanteio, 4 saida, 5
/// penalti, 6/7 falta, 8 pontape inicial). Na comparacao do reinicio elas sao unidades da fita
/// (`RECORD_RESTART`): por dentro reposicionam jogadores (`player_move`), que a fita grava.
pub(crate) const SET_PIECES: [usize; 6] = [0x6c4290, 0x6c4470, 0x6c4ec0, 0x6c50d0, 0x6c4820, 0x6c51e0];
pub(crate) static RECORD_RESTART: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True inside the restart comparison with the tape open (set pieces are recorded as one unit).
///
/// [pt] Dentro da comparacao do reinicio, com a fita aberta: as bolas paradas se gravam como unidade.
pub(crate) fn is_recording_restart() -> bool {
    RECORD_RESTART.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// `0x6c54a0` (thiscall, `match_eng.cpp`) with the calls that touch things outside injected (`0x6d6f70` narrates,
/// `0x6e1090` substitutes, the set pieces): the restart of play.
///
/// It zeroes the move state (`+0x1c5`, `+0x4782`.., `+0x8ea9`/`+0x8eaa`, `+0xf576`.., the pending move at `+0x475a`..)
/// and, without `+0x1a8`:
///
/// - the 15 minutes since each side's last change (`+0x1d2`/`+0x1d4`; with `+0x8eb2` = 8, both and −1000 in
/// `+0x476a`/`+0x476e`);
///
/// - after 60 minutes, 1 in 6, the first player out of stamina (`+0x4d` < 6750, `+0x10d` zeroed) is narrated (0x21cf);
///
/// - each side's mentality (`0x6df460` at `+0x9032` + 0xd0 × side): if it changed (or at the start) it is saved in
/// `+0x1ab`, with the previous one;
///
/// - mentality 3/4 of the side with the ball (`+0x8eae`): random draw(10) of added time into `+0x8ed0` and half into
/// `+0x8eb7`; the other side, in mentality 1, tires (`0x716450` with 1 + n/9);
///
/// - each side's AI substitution (`0x6e1090`; without a human, or a human with `+0x904b` outside a national team and
/// without `0x5fd480`): minute > random draw(50) + 10 and 55 minutes of play since the last one, then `0x6bbb60` and
/// `0x6bbf60`;
///
/// - near the end of each half (`+0x8ed4` = 495 or 990) the narration 0x21c1 with the added time;
///
/// - the tactics of each side (without a human) that changed mentality, by the marks `0x5b50d0` in the object at
/// `+0x91e2` + 0x18e3 × side, according to the mentality (1 to 5), `+0xe`/`+0x13` of the manager (`+0x9051`), the
/// formation (`+0x905d`: 3-4-3 and 3-5-2 Attacking are already attacking) and `+0x476a`; the narrations 0x2135 to
/// 0x213d (`0x6ce310` decides 0x213b/0x213d), `0x6d10a0` and the cells' mark 0x400000 (`0x5b5290`);
///
/// - with `+0x8ebb` (narrated match), outside corners and penalties, and for free kicks only near goal (table
/// `0xa15e44` >= 5): the crowd chants by score (`0x6ce310`), by the fixed clubs ([`0x9d0230`]..), by nation (`+0x76`:
/// 4/12, 13, 14) or the "Ruud Gullit" one (`0x84b8b0`);
///
/// - and the set piece by `+0x8eb2`.
///
/// [pt] { 0x6c54a0 } (thiscall, match_eng.cpp) com as chamadas que mexem fora injetadas (0x6d6f70 narra,
/// 0x6e1090 substitui, as bolas paradas): o reinicio do lance. Zera o estado do lance (+0x1c5,
/// +0x4782.., +0x8ea9/+0x8eaa, +0xf576.., o lance pendente em +0x475a..) e, sem +0x1a8:
/// - os 15 minutos desde a ultima mudanca de cada lado (+0x1d2/+0x1d4; com +0x8eb2 = 8, os dois e
///   -1000 em +0x476a/+0x476e);
/// - depois dos 60 minutos, 1 em 6, o primeiro jogador sem folego (+0x4d < 6750, +0x10d zerado) e
///   narrado (0x21cf);
/// - a postura de cada lado (0x6df460 em +0x9032 + 0xd0 x lado): mudou (ou no inicio) e guardada em
///   +0x1ab, com a anterior;
/// - postura 3/4 do lado com a bola (+0x8eae): sorteio(10) de acrescimo em +0x8ed0 e metade em
///   +0x8eb7; o outro lado, na postura 1, cansa (0x716450 com 1 + n/9);
/// - a substituicao da IA de cada lado (0x6e1090; sem humano, ou humano com +0x904b fora de selecao
///   e sem 0x5fd480): minuto > sorteio(50) + 10 e 55 de jogo desde a ultima, depois 0x6bbb60 e
///   0x6bbf60;
/// - perto do fim de cada tempo (+0x8ed4 = 495 ou 990) a narracao 0x21c1 com o acrescimo;
/// - a tatica de cada lado (sem humano) que mudou de postura, pelas marcas 0x5b50d0 no objeto em
///   +0x91e2 + 0x18e3 x lado, conforme a postura (1 a 5), o +0xe/+0x13 do tecnico (+0x9051), a
///   formacao (+0x905d: 3-4-3 e 3-5-2 Attacking ja sao de ataque) e +0x476a; as narracoes 0x2135 a
///   0x213d (0x6ce310 decide 0x213b/0x213d), 0x6d10a0 e a marca 0x400000 das casas (0x5b5290);
/// - com +0x8ebb (partida narrada), fora do escanteio e do penalti, e na falta so perto do gol
///   (tabela 0xa15e44 >= 5): os cantos da torcida pelo placar (0x6ce310), pelos clubes fixos
///   ([0x9d0230]..), pela nacao (+0x76: 4/12, 13, 14) ou o "Ruud Gullit" (0x84b8b0);
/// - e a bola parada por +0x8eb2.
pub(crate) unsafe fn restart_play_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut i16;
    let d = |o: usize| (m + o) as *mut u32;
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let narr = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        call(FN_MATCH_EVENTS_ADD_EVENT, g(m + engine::COMMENTATOR), [code | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let player = |side: usize, k: usize| m + engine::PLAYERS + side * 0x22d8 + k * 0x1be;
    // the state of the play
    *b(engine::BALL_TRANSIT_TICKS) = 0;
    *d(engine::MOMENTUM_A) = 0;
    *d(engine::MOMENTUM_B) = 0;
    *d(engine::BALL_RECEIVER) = 0;
    *b(engine::BALL_ROLL) = 0;
    *b(engine::BALL_HEIGHT) = 0;
    *d(engine::PASS_RECEIVER) = 0;
    *d(engine::ASSIST_CANDIDATE) = 0;
    *d(engine::PASSER) = 0;
    let mut changed = [false; 2];
    let mut old = [0u8; 2];
    *d(engine::SET_PIECE_TAKER) = 0;
    *w(engine::PASS_POWER) = 0;
    *d(engine::ACTION_FIRST_PLAYER) = 0;
    *b(engine::SHOT_REBOUND) = 0;
    *d(engine::PENDING_PLAY_OFFENDER) = 0;
    *d(engine::PENDING_PLAY_VICTIM) = 0;
    *b(engine::PENDING_PLAY_SERIOUS) = 0;
    *b(engine::PENDING_PLAY_KIND) = 0;
    *b(engine::PASS_TARGET_Y) = 0xff;
    *b(engine::PASS_TARGET_X) = 0xff;
    *w(engine::PENDING_PLAY_CODE) = -1;
    *b(engine::PENDING_PLAY_X) = 0xff;
    *b(engine::PENDING_PLAY_Y) = 0xff;
    *b(engine::PENDING_PLAY_REASON) = 0xff;
    *b(engine::PENDING_PLAY_SEVERITY) = 0xff;
    if *b(engine::GOLDEN_GOAL_SCORED) != 0 {
        return;
    }
    let quinze: [bool; 2] = if *b(engine::RESTART_TYPE) == 8 {
        *d(engine::STANCE_FACTOR) = 0xc47a0000;
        *d(0x476e) = 0xc47a0000;
        [true, true]
    } else {
        let t = *w(engine::CLOCK) as i32;
        [t - *w(engine::LAST_STANCE_CALL_TICK) as i32 > 0xa5, t - *w(0x1d4) as i32 > 0xa5]
    };
    let minute = (*w(engine::CLOCK) as i32 / 11) as i8;
    // the first player out of stamina
    if minute > 0x3c && random(6) == 0 {
        crate::comparison::branch("restart out of breath");
        'achou: for side in 0..2 {
            for k in 0..20 {
                let j = player(side, k);
                if read_i8(j + match_player::POSITION) >= 0 && read_f32(j + match_player::STAMINA) < 6750.0 && read_i8(j + match_player::MOMENT_WORK_RATE) == 0 {
                    let l = exe::match_player_get_side(j) as u32;
                    narr(call, 0x21cf, *((j + match_player::COMMENTARY_ID) as *const u8) as u32, l, u32::MAX, 0);
                    break 'achou;
                }
            }
        }
    }
    // each side’s mentality
    for side in 0..2usize {
        if *b(engine::RESTART_TYPE) == 5 || *b(engine::GOLDEN_GOAL_SCORED) != 0 {
            break;
        }
        let new = exe::match_man_choose_mentality(m + engine::MATCH_MANAGERS + side * 0xd0);
        let velha = *b(0x1ab + side);
        if new != velha || *w(engine::CLOCK) <= 1 {
            changed[side] = true;
            old[side] = velha;
            *b(0x1ab + side) = new;
        }
    }
    // the stoppage time (stance 3/4 of the side with the ball) and the other side’s fatigue
    if *b(engine::POSSESSION_SIDE) != 0xff {
        let mode = *b((0x1ab + *(b(engine::POSSESSION_SIDE) as *const i8) as isize) as usize);
        if mode == 3 || mode == 4 {
            crate::comparison::branch("restart stoppage time");
            let r = random(10) as i8 as i32;
            *w(engine::CLOCK) = (*w(engine::CLOCK)).wrapping_add(r as i16);
            let r2 = random(r / 2);
            *b(engine::STOPPAGE_TIME) = (*b(engine::STOPPAGE_TIME)).wrapping_add(r2 as u8);
            let other = (*b(engine::POSSESSION_SIDE) == 0) as usize;
            if *b(0x1ab + other) == 1 {
                for k in 0..20 {
                    let j = player(other, k);
                    if g(j + match_player::STAFF) != 0 {
                        exe::match_player_scale_anger(j, (r as f64 * read_f64(rdata::F64_0_1111111111) + 1.0) as f32);
                    }
                }
            }
        }
    }
    // the real human on the side (or the AI): +0x904d + 0xd0 x side
    let humano_de_verdade = |side: usize| -> bool {
        let p = Staff(g(m + engine::SIDE0_MANAGER + side * 0xd0));
        !p.is_null()
            && read(p.addr()) >= read(G_STAFF_COUNT) - 0x10
            && read(p.addr()) < read(G_STAFF_COUNT)
            && p.classification() != 0
            && (exe::human_manager_is_on_holiday(g(G_HUMAN_MANAGER), p) as i32) == 0
    };
    let club_of = |side: usize| g(g(m + engine::MATCH) + if side == 0 { 0x1c } else { 0x20 });
    // the AI substitution, side 0 and side 1
    for side in 0..2usize {
        let ia = if humano_de_verdade(side) {
            *b(0x904b + side * 0xd0) != 0 && (exe::database_is_national_team(club_of(side)) as i32) == 0 && (exe::human_manager_controls_reserve_team(g(G_HUMAN_MANAGER), g(m + engine::SIDE0_MANAGER + side * 0xd0)) as i32) == 0
        } else {
            true
        };
        let last = 0x8ecc + 2 * side;
        if ia && (minute as i32) > random(0x32) + 10 && *w(engine::CLOCK) as i32 - *w(last) as i32 > 0x37 {
            crate::comparison::branch("restart AI substitution");
            call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, m + engine::MATCH_MANAGERS + side * 0xd0, [0; 5]);
            *w(last) = *w(engine::CLOCK);
            exe::cm3_match_engine_recalc_side_score(m, side as i8);
            exe::cm3_match_engine_assign_marking_targets(m);
        }
        if side == 0 {
            // near the end of each half, the narrated stoppage time
            let end = match *w(engine::PERIOD_END) {
                0x1ef if minute > 0x27 && minute < 0x2e => Some(0x8d48),
                0x3de if minute > 0x54 && minute < 0x5b => Some(0x8d49),
                _ => None,
            };
            if let Some(f) = end {
                if *b(f) == 0 {
                    crate::comparison::branch("restart stoppage time narrated");
                    let q = *(b(engine::STOPPAGE_TIME) as *const i8) as i32 / 11;
                    let v = if q <= 2 { 2 } else { q as i8 };
                    narr(call, 0x21c1, *b(engine::POSSESSION_SIDE) as u32, 0, 0, v);
                    *b(f) = 1;
                }
            }
        }
    }
    // the AI tactic by stance
    for side in 0..2usize {
        let blk = m + side * 0xd0;
        if humano_de_verdade(side) {
            if *((blk + engine::SIDE0_MANAGER_TACTIC_SLOT) as *const u8) == 0 || (exe::database_is_national_team(club_of(side)) as i32) != 0 || (exe::human_manager_controls_reserve_team(g(G_HUMAN_MANAGER), g(blk + engine::SIDE0_MANAGER)) as i32) != 0 {
                continue;
            }
        }
        if !changed[side] {
            continue;
        }
        let formation = {
            let n = g(blk + engine::SIDE0_MANAGER_TACTICS);
            let n = if n == 0 { 0xa15934 } else { n };
            std::ffi::CStr::from_ptr(n as *const core::ffi::c_char).to_bytes()
        };
        let attack = formation != b"3-4-3" && formation != b"3-5-2 Attacking";
        let tobj = m + engine::TACTICS + side * 0x18e3;
        let manager = g(blk + engine::SIDE0_MANAGER_SHEET);
        let tb = |o: usize| *((manager + o) as *const i8);
        let lim = |base: i32| if *(b(0x1ad + side) as *const i8) <= 0 { base } else { base + 3 };
        let la = side as u32;
        let casas = || {
            for k in 0..20 {
                let c = read_i8(player(side, k) + 0x19);
                if c >= 0 {
                    exe::formation_set_player_instruction(tobj, c, 0x400000);
                }
            }
        };
        let quinze_l = quinze[side];
        match *b(0x1ab + side) {
            1 => {
                crate::comparison::branch("restart stance 1");
                let f = if (manager != 0 && read_f32(m + (0x476a + 4 * side)) < 2.5 && tb(non_player::ATTACKING) <= 6) || !attack { 0x20000 } else { 0x10000 };
                exe::formation_set_team_instruction(tobj, f);
                exe::formation_set_team_instruction(tobj, 0x800);
                exe::formation_set_team_instruction(tobj, 0x2000);
                if *w(engine::CLOCK) <= 1 {
                    narr(call, 0x2137, u32::MAX, la, u32::MAX, 0);
                } else if quinze_l {
                    narr(call, 0x2139, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * side) = *w(engine::CLOCK);
                }
                exe::cm3_match_engine_set_instruction_for_top_rated_players(m, side as i8);
            }
            2 => {
                crate::comparison::branch("restart stance 2");
                if manager == 0 || (!(tb(non_player::ATTACKING) as i32 > lim(0xf) && attack) && (tb(non_player::ATTACKING) > 5 || *b(0x1ad + side) == 0 || read_f32(m + (0x476a + 4 * side)) as f64 > 1.0)) {
                    exe::formation_set_team_instruction(tobj, 0x20000);
                    exe::formation_set_team_instruction(tobj, 0x800);
                } else if tb(non_player::ATTACKING) as i32 > lim(0xf) && attack {
                    exe::formation_set_team_instruction(tobj, 0x10000);
                    exe::formation_set_team_instruction(tobj, 0x800);
                } else {
                    // NaN at +0x476a falls here, like the original’s fcomp
                    exe::formation_set_team_instruction(tobj, 0x8000);
                    exe::formation_set_team_instruction(tobj, if (tb(non_player::DIRECTNESS) as i32) < lim(4) { 0x1000 } else { 0x800 });
                }
                exe::formation_set_team_instruction(tobj, 0x2000);
                if *w(engine::CLOCK) > 1 {
                    if matches!(old[side], 3..=5) {
                        narr(call, 0x2138, u32::MAX, la, u32::MAX, 0);
                    } else if old[side] == 1 && quinze_l {
                        narr(call, 0x213a, u32::MAX, la, u32::MAX, 0);
                        *w(0x1d2 + 2 * side) = *w(engine::CLOCK);
                    }
                }
                casas();
            }
            mode @ (3 | 4) => {
                if mode == 3 {
                    crate::comparison::branch("restart stance 3");
                    if manager != 0 {
                        exe::formation_set_team_instruction(tobj, if tb(non_player::ATTACKING) > 0x10 { 0x20000 } else { 0x8000 });
                        exe::formation_set_team_instruction(tobj, if (tb(non_player::DIRECTNESS) as i32) < lim(10) { 0x1000 } else { 0x800 });
                    } else {
                        exe::formation_set_team_instruction(tobj, 0x8000);
                        exe::formation_set_team_instruction(tobj, 0x1000);
                    }
                    exe::formation_set_team_instruction(tobj, 0x2000);
                } else {
                    crate::comparison::branch("restart stance 4");
                    exe::formation_set_team_instruction(tobj, if manager != 0 && tb(non_player::ATTACKING) > 0x12 { 0x20000 } else { 0x8000 });
                    exe::formation_set_team_instruction(tobj, 0x800);
                    exe::formation_set_team_instruction(tobj, 0x4000);
                }
                if *w(engine::CLOCK) <= 1 {
                    narr(call, if mode == 3 { 0x2136 } else { 0x2135 }, u32::MAX, la, u32::MAX, 0);
                } else if (old[side] == 2 || old[side] == 1) && quinze_l {
                    let (mut c1, mut c2, mut c3, mut c4, mut c5) = (0i8, 0i8, 0u8, 0u8, 0u8);
                    let r = (exe::cm3_match_engine_get_goal_margin(m, side as i8, &mut c1, &mut c2, &mut c3, &mut c4, &mut c5, 0) as u8);
                    narr(call, if (r as i8) > 0 { 0x213b } else { 0x213d }, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * side) = *w(engine::CLOCK);
                }
                casas();
            }
            5 => {
                crate::comparison::branch("restart stance 5");
                if manager != 0 {
                    let e = tb(non_player::ATTACKING);
                    exe::formation_set_team_instruction(tobj, if e > 0x10 && attack { 0x10000 } else if e > 0xc { 0x20000 } else { 0x8000 });
                    exe::formation_set_team_instruction(tobj, if (tb(non_player::DIRECTNESS) as i32) < lim(10) { 0x1000 } else { 0x800 });
                } else {
                    exe::formation_set_team_instruction(tobj, 0x20000);
                    exe::formation_set_team_instruction(tobj, 0x1000);
                }
                exe::formation_set_team_instruction(tobj, 0x2000);
                if quinze_l {
                    narr(call, 0x213c, u32::MAX, la, u32::MAX, 0);
                    *w(0x1d2 + 2 * side) = *w(engine::CLOCK);
                }
                casas();
            }
            _ => {}
        }
    }
    // the crowd chants (narrated match)
    'cantos: {
        let mode = *b(engine::RESTART_TYPE);
        if *b(engine::LIVE_MATCH) == 0 || mode == 3 || mode == 5 {
            break 'cantos;
        }
        if mode == 6 {
            // on a foul, only near the goal (chance from table 0xa15e44 >= 5)
            let chance = |x: i32, y: i32| -> f32 {
                let (x, y) = (x as i8, y as i8);
                if (0..=8).contains(&x) && (0..=11).contains(&y) { read_f32(G_CELL_DISTANCE_TABLE + (y as usize * 9 + x as usize) * 4) } else { 1000.0 }
            };
            let x = ((*(b(engine::BALL_X) as *const i8) as i32) - 4).abs();
            let ycol = *(b(engine::BALL_Y) as *const i8) as i32;
            let side = *b(engine::POSSESSION_SIDE);
            if side == 0 && (chance(x, (ycol - 0xb).abs()) as f64) < 5.0 {
                break 'cantos;
            }
            if side == 1 && (chance(x, ycol.abs()) as f64) < 5.0 {
                break 'cantos;
            }
        }
        let (mut o1, mut o2, mut o3, mut o4, mut o5) = (0i8, 0i8, 0u8, 0u8, 0u8);
        (exe::cm3_match_engine_get_goal_margin(m, 0, &mut o1, &mut o2, &mut o3, &mut o4, &mut o5, 0) as u8);
        let (v1, v2, empate) = (o1 as i32, o2 as i32, o3);
        let canto = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, side: u32| {
            crate::comparison::branch("restart goal_crowd_reaction chant");
            narr(call, code, u32::MAX, side, u32::MAX, 0);
        };
        // a club’s corners (`lado_clube` 0 home, 1 away): the other side’s with +0x77 = 8, the fixed ones
        // and those of the nation
        let cantos_de = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, side_club: usize| {
            let other = club_of(1 - side_club);
            if exe::manager_manager_is_club_in_financial_trouble(g(G_MANAGER_MANAGER), other) != 0 && *((g(other + 0x53) + 0x77) as *const u8) == 8 {
                canto(call, 0x2129, (1 - side_club) as u32);
                return;
            }
            let club = Club(club_of(side_club));
            for (fixo, code) in [(0x9d0230usize, 0x2127u32), (0x9d0234, 0x2128), (0x9d0238, 0x2163), (0x9d0268, 0x2166), (0x9d023c, 0x212b), (0x9d0240, 0x212c), (G_KEY_CLUB_LIVERPOOL, 0x212d)] {
                if random(2) == 0 && read(club.addr()) == read(fixo) {
                    canto(call, code, side_club as u32);
                    return;
                }
            }
            match *((g(club.addr() + club::NATION) + 0x76) as *const i8) {
                4 | 12 => {
                    let r = random(2);
                    canto(call, if r == 0 { 0x216b } else { 0x216a }, side_club as u32);
                }
                13 => {
                    let r = random(2);
                    canto(call, if r == 0 { 0x2167 } else { 0x216a }, side_club as u32);
                }
                14 => canto(call, 0x2118, side_club as u32),
                _ => {}
            }
        };
        // A: losing (or drawn without qualification) by random draw(10), the “Ruud Gullit”
        if (v1 < v2 || (v1 == v2 && empate == 0)) && random(10) == 0 && exe::staff_name_equals(g(m + engine::SIDE0_MANAGER), 0x9b63a8) != 0 {
            canto(call, 0x212a, 1);
            break 'cantos;
        }
        // B/C: the away club’s supporters
        let r = random(v2 - v1 + 1);
        let away = r != 0 && (v1 < v2 || (v1 == v2 && empate == 0));
        if away || random(v1 - v2 + 5) == 0 {
            cantos_de(call, 1);
            break 'cantos;
        }
        // E/F: the home crowd
        let r = random(v1 - v2 + 1);
        let home = r != 0 && (v1 > v2 || (v1 == v2 && empate != 0));
        if home || random(v2 - v1 + 5) == 0 {
            cantos_de(call, 0);
        }
    }
    // a bola parada
    match *b(engine::RESTART_TYPE) {
        1 => {}
        2..=8 => {
            let k = match *b(engine::RESTART_TYPE) {
                2 => 0,
                3 => 1,
                4 => 2,
                5 => 3,
                6 | 7 => 4,
                _ => 5,
            };
            call(SET_PIECES[k], m, [0; 5]);
        }
        _ => {
            // simplification: the original shows the ”Error” box (match_eng.cpp, line 0x29ab) and exits the game
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        }
    }
}

/// Rust-mode dispatch of the restart's recorded calls: set pieces go to the game address, the rest as in the tackle
/// dispatch.
///
/// [pt] O despacho do reinicio no modo Rust: narracao e substituicao como na dividida; as bolas paradas
/// pelo endereco do jogo.
pub(crate) unsafe fn dispatch_restart_call(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    if SET_PIECES.contains(&routine) {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(routine);
        f(this);
        return 0;
    }
    dispatch_tackle_calls(routine, this, a)
}

/// resets the play state (`engine::BALL_TRANSIT_TICKS`, `engine::MOMENTUM_A`.., `engine::BALL_HEIGHT`/`engine::BALL_ROLL`, `engine::PASSER`.., the pending play `engine::PENDING_PLAY_KIND`..) and, without
/// +0x1a8: the 15 minutes since each side's last change (`engine::LAST_STANCE_CALL_TICK`/+0x1d4); after 60 minutes, 1 in 6, the first player out
/// of breath (+0x4d < 6750) narrated (0x21cf); each side's posture (`stance`) stored in +0x1ab with the previous one;
/// with posture 3/4 for the side with the ball, draw(10) of stoppage time (`engine::CLOCK`, half in `engine::STOPPAGE_TIME`) and the other side
/// tiring in posture 1 (`exe::match_player_scale_anger`); the AI substitution (`exe::match_man_make_substitutions` on `engine::MATCH_MANAGERS` + 0xd0 × side; no human, or an automatic
/// human) with minute > draw(50) + 10 and 55 of play since the last, `exe::cm3_match_engine_recalc_side_score` and `individual_marking`; the narrated stoppage time
/// (0x21c1) near the end of each half; the AI tactics by posture (`set_formation_flag` marks on object `engine::TACTICS` + 0x18e3 × side, by
/// the manager's +0xe/+0x13, the formation `engine::SIDE0_MANAGER_TACTICS` and `engine::STANCE_FACTOR`; narrations 0x2135 to 0x213d, `exe::cm3_match_engine_set_instruction_for_top_rated_players`, the squares'
/// 0x400000 mark); with `engine::LIVE_MATCH` the crowd chants (score `exe::cm3_match_engine_get_goal_margin`, fixed clubs [0x9d0230].., nation +0x76, "Ruud Gullit"
/// `exe::staff_name_equals`); and the set piece by `engine::RESTART_TYPE` (throw-in, corner, kick-off, penalty, free kick, kick-off). Ghidra's
/// pseudocode gets postures 2 and 3 wrong ("previous 3/4/5" and "previous 1/2" become "== true"): ported from the
/// disassembly. Compared through the tape (set pieces are units with `RECORD_RESTART`, since they reposition players
/// through `player_move`)
/// Original: `restart_play` (0x6c54a0, match_eng.cpp (3.2 KB, called by 0x6b98c0), thiscall).
///
/// [pt] { 0x6c54a0 } (thiscall, match_eng.cpp): ver `restart_play_with`.
pub(crate) unsafe extern "thiscall" fn restart_play(m: usize) {
    restart_play_with(m, &mut |r, t, a| dispatch_restart_call(r, t, a))
}

// Loop calls by address: each goes through its own hook (Rust copy or comparer).
/// Calls the thiscall function at address a with `this` and no arguments.
pub(crate) unsafe fn call_thiscall0(a: usize, this: usize) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(a)(this)
}
/// Calls the thiscall function at address a with `this` and 1 argument.
pub(crate) unsafe fn call_thiscall1(a: usize, this: usize, x: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(a)(this, x)
}
/// Calls the thiscall function at address a with `this` and 2 arguments.
pub(crate) unsafe fn call_thiscall2(a: usize, this: usize, x: u32, y: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(a)(this, x, y)
}
/// Calls the thiscall function at address a with `this` and 3 arguments.
pub(crate) unsafe fn call_thiscall3(a: usize, this: usize, x: u32, y: u32, z: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(a)(this, x, y, z)
}
/// Calls the thiscall function at address a with `this` and 4 arguments.
pub(crate) unsafe fn call_thiscall4(a: usize, this: usize, x: u32, y: u32, z: u32, w: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(a)(this, x, y, z, w)
}

/// End of match from the loop: runs fechamento (post-match reputations) and restores the saved tactics (`store_instructions`) of
/// each human-managed club whose manager lacks the `exe::human_manager_controls_reserve_team` flag.
///
/// [pt] Fim da partida pelo laco: o fechamento (0x6d1cc0) e, para cada clube que um humano dirige
/// (0x600a50) sem o registro +0x9a05 do tecnico (0x5fd480), as instrucoes guardadas voltam (0x896fe0).
pub(crate) unsafe fn finish_match_for_clubs(m: usize) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    exe::match_eng_update_post_match_reputations(m);
    for (o, marca) in [(0x1c, 0x40), (0x20, 0x41)] {
        let t = u(m, engine::MATCH);
        if read_i8(t + marca) == 0 && exe::human_manager_is_human_club(u(t, o), 1, 1, 0, 0) != 0 && call_thiscall1(FN_HUMAN_MANAGER_CONTROLS_RESERVE_TEAM, u(G_HUMAN_MANAGER, 0), u(u(t, o), 0xcf) as u32) == 0 {
            call_thiscall4(FN_TACTICS_CLEAR_OR_RESTORE_SELECTION, u(G_TACTICS, 0), u(t, o) as u32, 1, 0, 0);
        }
    }
}

/// Stores each side's player nearest the ball (0x6c30e0, engine `engine::NEAREST_TO_BALL`/`engine::NEAREST_TO_BALL_SIDE1`) and whether each side's goalkeeper is
/// nearest (0x6c3010, `engine::GOALKEEPER_NEAREST`/`engine::GOALKEEPER_NEAREST_SIDE1`).
///
/// [pt] Os mais perto da bola de cada lado (0x6c30e0, so dentro da area quando a bola esta nela) e se o
/// guarda da posicao 1 e o mais proximo dela (0x6ce110 + 0x6c3010).
pub(crate) unsafe fn update_nearest_to_ball(m: usize) {
    let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
    let a0 = (2..=6).contains(&bx) && (0..=1).contains(&by);
    *((m + engine::NEAREST_TO_BALL) as *mut usize) = call_thiscall4(FN_CM3_MATCH_ENGINE_FIND_NEAREST_OUTFIELD_PLAYER, m, 0, bx as u8 as u32, by as u8 as u32, !a0 as u32) as usize;
    let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
    let a1 = (2..=6).contains(&bx) && (10..=11).contains(&by);
    *((m + engine::NEAREST_TO_BALL_SIDE1) as *mut usize) = call_thiscall4(FN_CM3_MATCH_ENGINE_FIND_NEAREST_OUTFIELD_PLAYER, m, 1, bx as u8 as u32, by as u8 as u32, !a1 as u32) as usize;
    for (lado, campo) in [(0u32, 0x8eac), (1, 0x8ead)] {
        let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
        let g = call_thiscall1(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER, m, lado);
        *((m + campo) as *mut u8) = call_thiscall3(FN_CM3_MATCH_ENGINE_IS_CLOSEST_TEAMMATE_TO_CELL, m, g, bx as u8 as u32, by as u8 as u32) as u8;
    }
}

/// The match engine loop: `n` match instants. Match already decided: quick simulation, standout, closing, humans'
/// instructions restored, referee rating (2). Each instant: punishments, shoot-out, kick-off, drawn substitution,
/// marked incident, restart and taker; on the clock, abandonment (8), period (4) and full time (2 or 4); then
/// possession, each player's time, lines, nearest to the ball, marking and each active player's decision (`decide_player`),
/// with the pending shot and the contest. Validated by the career.
///
/// Ret 4: the engine loop, `n` instants of the match. Match already decided (+0): quick simulation (`quick_simulation`), standout
/// (`choose_standout`), closing, the humans' instructions restored and the referee rating (`referee_grade`); returns 2. Otherwise
/// clears the player sheets and the pointers of those who left and, at each instant: pending punishments (0x6d1210),
/// penalty shoot-out (0x6be710; at the end, 2), the narrated kick-off (0x212f/0x1f74), the randomly drawn substitution,
/// the marked incident (`apply_foul`/`clear_move`), the restart (`restart_play`) and the set-piece taker (`exe::cm3_match_engine_pick_restart_taker`). On the clock
/// (`engine::CLOCK`): end by abandonment (8), the period (`period_completed`: 4 at the breaks; end of the match with the aggregate score,
/// `record_tactic_result` and 2 or 4). Ball in play (`roll_ball`) or stopped adds stoppage time; otherwise possession (0x6cf750), each
/// player's time (`update_player`), the lines (`defensive_lines`, `store_extremes`, `player_order`), those nearest to the ball, the pressure in the
/// areas (0x6bd6b0), the marking (`designated_marked`/0x6bc030) and each active player's decision (`decide_player`), with the pending
/// shot (`shot`) and the contest (0x6c9cd0). Returns 1 at the end of the `n` instants.
/// Original: `engine_loop` (0x6b98c0, match_eng.cpp (4.8 KB), thiscall, ret 4).
///
/// [pt] { 0x6b98c0 } (ret 4, match_eng.cpp): o laco do motor, `n` instantes da partida. Partida ja
/// decidida (+0): simulacao rapida (0x6beeb0), destaque (0x6d0f80), fechamento, instrucoes dos
/// humanos de volta e nota do arbitro (0x6d4a20); devolve 2. Senao limpa as fichas e os ponteiros
/// de quem saiu e, a cada instante: punicoes pendentes (0x6d1210), disputa de penaltis (0x6be710;
/// no fim, 2), o pontape inicial narrado (0x212f/0x1f74), a substituicao sorteada, o lance marcado
/// (0x6bac40/0x6bba10), o reinicio (0x6c54a0) e o cobrador da bola parada (0x6ceb90). No relogio
/// (+0x8ed0): fim por abandono (8), o periodo (0x6bd930: 4 nos intervalos; fim da partida com o
/// placar agregado, 0x896290 e 2 ou 4). Bola rolando (0x6bbe40) ou parada somam acrescimos; senao
/// a posse (0x6cf750), o tempo de cada jogador (0x6f4aa0), as linhas (0x6c3aa0, 0x6cd660, 0x6bd760),
/// os mais perto da bola, a pressao nas areas (0x6bd6b0), as marcacoes (0x7160f0/0x6bc030) e a
/// decisao de cada jogador ativo (0x7103e0), com o chute pendente (0x6f6c00) e a disputa (0x6c9cd0).
/// Devolve 1 ao fim dos `n` instantes.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe extern "thiscall" fn engine_loop(m: usize, n_in: u32) -> u32 {
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut i16;
    let narrate = |code: u32, a4: u32| {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_EVENTS_ADD_EVENT);
        f(read_usize(m + engine::COMMENTATOR), code, m as u32, u32::MAX, a4, u32::MAX, u32::MAX, 0);
    };
    let acrescimos = || {
        let r = random(10) as i8;
        *w(engine::CLOCK) = (*w(engine::CLOCK)).wrapping_add(r as i16);
        let v = random(ftol(r as f64 * read_f64(rdata::F64_0_8)));
        *b(engine::STOPPAGE_TIME) = (*b(engine::STOPPAGE_TIME)).wrapping_add(v as u8);
    };
    let mut n = n_in as u8 as i8;
    if read_i8(m + 0) != 0 {
        call_thiscall0(FN_CM3_MATCH_ENGINE_QUICK_SIMULATE_MATCH, m);
        call_thiscall0(FN_CM3_MATCH_ENGINE_PICK_MAN_OF_THE_MATCH, m);
        finish_match_for_clubs(m);
        call_thiscall0(FN_CM3_MATCH_ENGINE_RECORD_REFEREE_RATING, m);
        return 2;
    }
    for side in 0..2 {
        for k in 0..20 {
            let f = m + engine::PLAYERS + (side * 20 + k) * 0x1be;
            if read_i8(f + match_player::POSITION) >= 0 && *((f + match_player::ENGINE) as *const u32) == 0 {
                *((f + match_player::POSITION) as *mut u8) = 0xfb;
            }
        }
    }
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        let q = read_usize(m + o);
        if q != 0 && *((q + match_player::ENGINE) as *const usize) == 0 {
            *((m + o) as *mut usize) = 0;
        }
    }
    'instante: while n != 0 {
        crate::extension::instant(m);
        if read_i8(m + engine::PUNISHMENT_STATE) == 1 {
            call_thiscall1(FN_CM3_MATCH_ENGINE_PROCESS_QUEUED_FOULS, m, 0);
            n = n.wrapping_sub(1);
            continue;
        }
        match read_i8(m + engine::SHOOTOUT_STATE) {
            1..=3 => {
                call_thiscall1(FN_CM3_MATCH_ENGINE_RUN_PENALTY_SHOOTOUT_STEP, m, 4);
                if read_i8(m + engine::SHOOTOUT_STATE) == 0 {
                    crate::comparison::branch("loop penalties decided");
                    finish_match_for_clubs(m);
                    call_thiscall1(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, *w(engine::PERIOD_END) as u16 as u32);
                    call_thiscall0(FN_CM3_MATCH_ENGINE_RECORD_REFEREE_RATING, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            4..=6 => {
                call_thiscall1(FN_CM3_MATCH_ENGINE_RUN_PENALTY_SHOOTOUT_STEP, m, 3);
                if read_i8(m + engine::SHOOTOUT_STATE) == 0 {
                    crate::comparison::branch("loop penalties decided without summary");
                    finish_match_for_clubs(m);
                    call_thiscall0(FN_CM3_MATCH_ENGINE_RECORD_REFEREE_RATING, m);
                    return 2;
                }
                n = n.wrapping_sub(1);
                continue;
            }
            _ => {}
        }
        if read_i8(m + engine::KICKOFF_ANNOUNCED) == 0 && read_i8(m + engine::RESTART_TYPE) == 8 && read_i8(m + engine::GOLDEN_GOAL_SCORED) == 0 {
            narrate(if *w(engine::CLOCK) > 1 { 0x1f74 } else { 0x212f }, read_i8(m + engine::POSSESSION_SIDE) as u8 as u32);
            *b(engine::KICKOFF_ANNOUNCED) = 1;
            continue;
        }
        if read_i8(m + engine::RESTART_TYPE) != 0 && read_i8(m + engine::PUNISHMENT_STATE) == 0 && read_i8(m + engine::GOLDEN_GOAL_SCORED) == 0 {
            // the random substitution: a player on the pitch, up to 20 attempts (0x7164b0 releases those off it)
            let mut attempts = 0i8;
            let sheet = |a: i8, c: i8| m + engine::PLAYERS + ((a as i32 * 20 + c as i32) * 0x1be) as usize;
            let (a, c) = loop {
                let a = random(2) as i8;
                let c = random(0x14) as i8;
                attempts += 1;
                if read_i8(sheet(a, c) + 0x19) >= 0 {
                    break (a, c);
                }
                if attempts >= 0x14 {
                    break (a, c);
                }
                if call_thiscall2(FN_MATCH_PLAYER_PASSES_ANGER_CHECK, sheet(a, c), 1, 0) as u8 != 0 {
                    break (a, c);
                }
            };
            if attempts < 0x14 && read_i8(sheet(a, c) + 0x19) >= 0 {
                call_thiscall1(FN_CM3_MATCH_ENGINE_PROCESS_QUEUED_FOULS, m, sheet(a, c) as u32);
                if read_i8(m + engine::PUNISHMENT_STATE) != 0 {
                    continue;
                }
            }
        }
        if read_i8(m + engine::PUNISHMENT_STATE) == 2 {
            *b(engine::PUNISHMENT_STATE) = 0;
        }
        // o lance marcado
        let q = read_usize(m + engine::PENDING_PLAY_OFFENDER);
        if read_i8(m + engine::PENDING_PLAY_KIND) != 0 && q != 0 {
            let cl = read_i8(q + match_player::SIDE);
            if read_i8(m + engine::RESTART_TYPE) != 0 {
                let d = read_usize(m + engine::BALL_HOLDER);
                let (by, ly) = (read_i8(m + engine::BALL_Y), read_i8(m + engine::PENDING_PLAY_Y));
                if d != 0 && read_i8(d + match_player::SIDE) as i32 == (cl == 0) as i32 && ((by > ly && cl == 1) || (by < ly && cl == 0)) {
                    crate::comparison::branch("loop move discarded");
                    call_thiscall0(FN_CM3_MATCH_ENGINE_CLEAR_PENDING_FOUL, m);
                } else if random(5) == 0 && call_thiscall0(FN_CM3_MATCH_ENGINE_AWARD_PENDING_FOUL, m) as u8 != 0 {
                    continue;
                }
            } else if random(7) == 0 {
                let v = if read_i8(m + engine::PENDING_PLAY_KIND) != 0 && read_usize(m + engine::PENDING_PLAY_OFFENDER) != 0 { (read_i8(read_usize(m + engine::PENDING_PLAY_OFFENDER) + 0x27) == 0) as u32 } else { 0xff };
                narrate(0x20ef, v);
                if call_thiscall0(FN_CM3_MATCH_ENGINE_AWARD_PENDING_FOUL, m) as u8 != 0 {
                    continue;
                }
            }
        }
        if read_i8(m + engine::GOLDEN_GOAL_SCORED) == 0 {
            if read_i8(m + engine::RESTART_TYPE) != 0 {
                call_thiscall0(FN_CM3_MATCH_ENGINE_HANDLE_STOPPAGE, m);
            }
            let d = read_usize(m + engine::BALL_HOLDER);
            if read_i8(m + engine::GOLDEN_GOAL_SCORED) == 0 && read_i8(m + engine::RESTART_TYPE) != 0 && (d == 0 || read_i8(d + match_player::POSITION) < 0) {
                crate::comparison::branch("loop set piece pick_restart_taker");
                let c = call_thiscall3(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER, m, read_i8(m + engine::POSSESSION_SIDE) as u8 as u32, read_i8(m + engine::RESTART_TYPE) as u8 as u32, 0) as usize;
                *((m + engine::BALL_HOLDER) as *mut usize) = c;
                call_thiscall2(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, c, read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32);
                *((read_usize(m + engine::BALL_HOLDER) + 0x19c) as *mut i16) = 200;
                call_thiscall0(FN_MATCH_PLAYER_TAKE_POSSESSION, read_usize(m + engine::BALL_HOLDER));
                let d = if read_i8(m + engine::POSSESSION_SIDE) != 0 { 1 } else { 0xa };
                call_thiscall4(FN_MATCH_PLAYER_SET_FACING_DIRECTION, read_usize(m + engine::BALL_HOLDER), read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32, 4, d);
            }
        }
        *w(engine::SUB_TICK) = (*w(engine::SUB_TICK)).wrapping_add(1);
        if *w(engine::SUB_TICK) >= 1 || read_i8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
            n = n.wrapping_sub(1);
            let limit = read_i8(m + engine::ABANDON_MINUTE);
            if limit != -1 && (limit as i16).wrapping_mul(11) < *w(engine::CLOCK) {
                crate::comparison::branch("loop abandonment");
                *b(engine::SCORE) = 0xfd;
                *b(engine::SCORE_SIDE1) = 0xfd;
                *((read_usize(m + engine::MATCH) + 0x43) as *mut u8) = 0xfd;
                *b(engine::RESTART_TYPE) = 0;
                narrate(0x217b, 0);
                return 8;
            }
            if read_i8(m + engine::LIVE_MATCH) != 0 {
                call_thiscall1(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, 0x3de);
            }
            *w(engine::CLOCK) = (*w(engine::CLOCK)).wrapping_add(1);
            *w(engine::SUB_TICK) = 0;
            match call_thiscall0(FN_CM3_MATCH_ENGINE_PERIOD_COMPLETED, m) as i8 {
                0 => {
                    for base in [0x4796, 0x6a6e] {
                        for k in 0..20 {
                            let f = m + base + k * 0x1be;
                            if read_i8(f + match_player::POSITION) >= 0 {
                                call_thiscall0(FN_MATCH_PLAYER_UPDATE_RATINGS_AND_CONDITION, f);
                            }
                        }
                    }
                }
                1..=3 => return 4,
                4 => {
                    // end of the match
                    call_thiscall0(FN_CM3_MATCH_ENGINE_PICK_MAN_OF_THE_MATCH, m);
                    let t = Fixture(read_usize(m + engine::MATCH));
                    let tb = |o: usize| (t.addr() + o) as *mut u8;
                    let ag = t.home_aggregate();
                    if ag != -1 {
                        if t.home_goals_extra_time() != -1 {
                            *tb(fixture::HOME_AGGREGATE) = (*tb(fixture::HOME_GOALS_EXTRA_TIME)).wrapping_add(ag as u8);
                            *tb(fixture::AWAY_AGGREGATE) = (*tb(fixture::AWAY_AGGREGATE)).wrapping_add(*tb(fixture::AWAY_GOALS_EXTRA_TIME));
                        } else {
                            *tb(fixture::HOME_AGGREGATE) = (*tb(fixture::HOME_GOALS)).wrapping_add(ag as u8);
                            *tb(fixture::AWAY_AGGREGATE) = (*tb(fixture::AWAY_AGGREGATE)).wrapping_add(*tb(fixture::AWAY_GOALS));
                        }
                    }
                    if *(t.addr() as *const i32) != -2 {
                        call_thiscall1(FN_TACTICS_RECORD_TACTIC_RESULT, *(G_TACTICS as *const usize), t.addr() as u32);
                    }
                    let p = read_i8(m + engine::SHOOTOUT_STATE);
                    if p != 0 {
                        crate::comparison::branch("loop end for penalties");
                        *w(engine::CLOCK) = (*w(engine::CLOCK) as i32 / 30 * 30) as i16;
                        narrate(if p == 1 { 0x2003 } else { 0x2002 }, 0);
                        return 4;
                    }
                    finish_match_for_clubs(m);
                    call_thiscall0(FN_CM3_MATCH_ENGINE_RECORD_REFEREE_RATING, m);
                    return 2;
                }
                r => {
                    // simplification: the original stops the game with ”Error” (match_eng.cpp, line 0x776); 0x6bd930 only returns 0..=4
                    linha(&format!("motor: engine_loop: periodo devolveu {}", r));
                    std::process::abort();
                }
            }
        }
        // ball in play or dead: only stoppage time
        if read_i8(m + engine::RESTART_TYPE) == 0 && {
            call_thiscall0(FN_CM3_MATCH_ENGINE_MOVE_LOOSE_BALL, m);
            read_i8(m + engine::RESTART_TYPE) != 0
        } {
            acrescimos();
            continue;
        }
        if read_usize(m + engine::PASS_RECEIVER) != read_usize(m + engine::BALL_HOLDER) && read_usize(m + engine::BALL_HOLDER) != 0 {
            *((m + engine::PASS_RECEIVER) as *mut usize) = 0;
            *((m + engine::ASSIST_CANDIDATE) as *mut usize) = 0;
        }
        call_thiscall2(FN_CM3_MATCH_ENGINE_RECORD_POSSESSION_TICK, m, read_i8(m + engine::POSSESSION_SIDE) as u8 as u32, read_i8(m + engine::LIVE_MATCH) as u8 as u32);
        for base in [0x4796, 0x6a6e] {
            for k in 0..20 {
                let f = m + base + k * 0x1be;
                if read_i8(f + match_player::POSITION) >= 0 {
                    call_thiscall0(FN_MATCH_PLAYER_UPDATE_MATCH_RATING, f);
                }
            }
        }
        call_thiscall0(FN_CM3_MATCH_ENGINE_UPDATE_OFFSIDE_LINES, m);
        call_thiscall0(FN_CM3_MATCH_ENGINE_UPDATE_DEFENSIVE_LINES, m);
        call_thiscall0(FN_CM3_MATCH_ENGINE_BUILD_ACTION_ORDER, m);
        *((m + engine::ACTION_FIRST_PLAYER) as *mut usize) = 0;
        update_nearest_to_ball(m);
        *b(2) = 0xff;
        *b(3) = 0xff;
        let (mut vx, mut vy) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
        if *((m + engine::TACTICS_FLAGS) as *const u32) & 0x1000 != 0 {
            call_thiscall1(FN_CM3_MATCH_ENGINE_UPDATE_SIDE_PRESSURE_FLAG, m, 0);
        }
        if *((m + engine::TACTICS_FLAGS_SIDE1) as *const u32) & 0x1000 != 0 {
            call_thiscall1(FN_CM3_MATCH_ENGINE_UPDATE_SIDE_PRESSURE_FLAG, m, 1);
        }
        let list = |i: i32| *((m + engine::ACTIVE_LIST + i as usize * 4) as *const usize);
        let mut i = 0i8;
        while i < read_i8(m + engine::ACTIVE_LIST_COUNT) {
            let q = list(i as i32);
            *((q + match_player::ACTION_ORDER) as *mut i8) = i;
            *((q + match_player::MARKER_COUNT) as *mut u8) = 0;
            *((q + match_player::MARKING_TARGET) as *mut usize) = 0;
            i += 1;
        }
        // the markings
        let mut i = 0i8;
        while i < read_i8(m + engine::ACTIVE_LIST_COUNT) {
            let q = list(i as i32);
            i += 1;
            let side = read_i8(q + match_player::SIDE);
            if call_thiscall1(FN_FORMATION_GET_MARKING_TYPE, m + engine::TACTICS + side as isize as usize * 0x18e3, read_i8(q + match_player::POSITION) as u8 as u32) & 1 != 0 {
                continue;
            }
            let mut target = call_thiscall0(FN_MATCH_PLAYER_FIND_MAN_MARKING_TARGET, q) as usize;
            if target == 0 {
                if call_thiscall0(FN_MATCH_PLAYER_GET_POSITION, q) as u16 == 1 || q == read_usize(m + engine::BALL_HOLDER) {
                    continue;
                }
                let table = || call_thiscall0(FN_MATCH_PLAYER_GET_POSITION, q);
                let by = read_i8(m + engine::BALL_Y);
                let mark = if *((m + engine::TACTICS_FLAGS + side as isize as usize * 0x18e3) as *const u32) & 0x4000 != 0 {
                    true
                } else if read_i8(m + engine::POSSESSION_SIDE) == side && if side == 0 { by >= 3 } else { by <= 8 } {
                    false
                } else if (table() & 4 != 0 || table() & 2 != 0) && call_thiscall0(FN_MATCH_PLAYER_IS_BALL_IN_OWN_HALF, q) as u8 != 0 {
                    true
                } else {
                    let bottom = if table() & 8 == 0 && table() & 0x10 == 0 {
                        false
                    } else if side == 0 {
                        by < 3
                    } else {
                        by > 8
                    };
                    bottom || is_in_attacked_box((side != 1) as i8, read_i8(m + engine::BALL_X), by)
                };
                if !mark {
                    continue;
                }
                target = call_thiscall1(FN_CM3_MATCH_ENGINE_CHOOSE_MARKING_TARGET, m, q as u32) as usize;
                if target == 0 {
                    continue;
                }
            }
            *((q + match_player::MARKING_TARGET) as *mut usize) = target;
            let k = read_i8(target + match_player::MARKER_COUNT);
            if k < 3 {
                *((target + match_player::MARKERS + k as usize * 4) as *mut usize) = q;
                *((target + match_player::MARKER_COUNT) as *mut u8) = (k + 1) as u8;
            }
        }
        // the decision of each active player
        let mut cansou = false;
        let mut i = 0i32;
        while i < read_i8(m + engine::ACTIVE_LIST_COUNT) as i32 {
            let q = list(i);
            if *((q + match_player::MARKING_TARGET) as *const usize) == 0 || read_i8(m + engine::POSSESSION_SIDE) == read_i8(q + match_player::SIDE) {
                if !cansou || read_i8(q + match_player::MOMENT_PACE) as i32 > random(10) {
                    call_thiscall0(FN_MATCH_PLAYER_TAKE_TURN, q);
                }
                let q = list(i);
                let action = *((q + match_player::ACTION) as *const i16);
                if action == 0x11 || action == 0x12 {
                    let v = read_f32(q + if action == 0x11 { 0x71 } else { 0xc1 }) as f64;
                    if random(ftol(read_f32(rdata::F32_2010) as f64 - v * read_f32(rdata::F32_100) as f64)) == 0 {
                        cansou = true;
                    }
                }
                if read_i8(m + engine::RESTART_TYPE) != 0 || read_i8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
                    acrescimos();
                    if read_i8(m + engine::SUBSTITUTION_PENDING) != 0 {
                        call_thiscall0(FN_CM3_MATCH_ENGINE_PROCESS_INJURED_PLAYERS, m);
                    }
                    continue 'instante;
                }
                // the pending shot and the marked contest
                let mut done = false;
                if read_i8(m + engine::BALL_TRANSIT_TICKS) != 0 {
                    *b(engine::BALL_TRANSIT_TICKS) = (*b(engine::BALL_TRANSIT_TICKS)).wrapping_sub(1);
                    if read_i8(m + engine::BALL_TRANSIT_TICKS) == 0 && read_i8(m + engine::BALL_HEIGHT) == 4 && read_usize(m + engine::BALL_RECEIVER) == 0 {
                        crate::comparison::branch("loop contest scheduled");
                        call_thiscall2(FN_CM3_MATCH_ENGINE_RESOLVE_BALL_CHALLENGE, m, read_i8(m + engine::TRANSIT_TARGET_X) as u8 as u32, read_i8(m + engine::TRANSIT_TARGET_Y) as u8 as u32);
                        done = true;
                    }
                }
                let c = read_usize(m + engine::BALL_RECEIVER);
                if !done && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0 && c != 0 {
                    let shoot = |c: usize| call_thiscall3(FN_MATCH_PLAYER_RECEIVE_BALL, c, *w(engine::PASS_POWER) as u16 as u32, read_i8(m + engine::TRANSIT_ORIGIN_X) as u8 as u32, read_i8(m + engine::TRANSIT_ORIGIN_Y) as u8 as u32);
                    if read_i8(c + match_player::CELL_X) == read_i8(m + engine::BALL_X) && read_i8(c + match_player::CELL_Y) == read_i8(m + engine::BALL_Y) {
                        shoot(c);
                    } else if c == read_usize(m + engine::SET_PIECE_TAKER) && read_i8(c + match_player::BUSY) == 0 && read_i8(c + match_player::MOMENT_ANTICIPATION) != 0 && read_i8(c + match_player::MOMENT_MOVEMENT) != 0 {
                        crate::comparison::branch("loop taker approaches");
                        *b(engine::BALL_TRANSIT_TICKS) = 1;
                        call_thiscall1(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE, c, 1);
                        *b(engine::BALL_TRANSIT_TICKS) = 0;
                        let c2 = read_usize(m + engine::BALL_RECEIVER);
                        if c2 == c && read_i8(c2 + match_player::CELL_X) == read_i8(m + engine::BALL_X) && read_i8(c2 + match_player::CELL_Y) == read_i8(m + engine::BALL_Y) {
                            shoot(c2);
                        }
                    }
                }
                if read_i8(m + engine::RESTART_TYPE) != 0 || read_i8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
                    acrescimos();
                    continue 'instante;
                }
                if read_usize(m + engine::ACTION_FIRST_PLAYER) != 0 {
                    continue 'instante;
                }
                let q = list(i);
                let action = *((q + match_player::ACTION) as *const i16);
                if (action == 0x104 || action == 0x76) && i < 0x16 {
                    crate::comparison::branch("loop enters the list");
                    let k = read_i8(m + engine::ACTIVE_LIST_COUNT);
                    *((m + engine::ACTIVE_LIST + k as isize as usize * 4) as *mut usize) = q;
                    *b(engine::ACTIVE_LIST_COUNT) = (k + 1) as u8;
                }
                call_thiscall0(FN_CM3_MATCH_ENGINE_UPDATE_OFFSIDE_LINES, m);
                if vx != read_i8(m + engine::BALL_X) || vy != read_i8(m + engine::BALL_Y) {
                    update_nearest_to_ball(m);
                    vx = read_i8(m + engine::BALL_X);
                    vy = read_i8(m + engine::BALL_Y);
                }
            }
            i += 1;
        }
    }
    1
}

/// the side is losing on aggregate (`exe::cm3_match_engine_get_goal_margin` without bonus) or, in a first leg with no third phase and no qualified
/// side, level
/// Original: `behind_on_aggregate` (0x6d5950, match_eng.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d5950 } (ret 4, match_eng.cpp): o lado `lado` esta atras no agregado (0x6ce310 sem bonus): perde
/// no placar somado ou, no jogo de ida sem terceira fase nem classificado, empata.
pub(crate) unsafe extern "thiscall" fn behind_on_aggregate(m: usize, side: i8) -> u8 {
    let (mut casa, mut fora, mut classificado, mut ida, mut fase3) = (0i8, 0i8, 0u8, 0u8, 0u8);
    aggregate_score(m, side, &mut casa, &mut fora, &mut classificado, &mut ida, &mut fase3, 0);
    if casa < fora {
        return 1;
    }
    (ida != 0 && fase3 == 0 && classificado == 0 && casa <= fora) as u8
}

/// the first active teammate in the deep attack (`player_limit`), central (`x_off_centre`) and unpressed (0x6bcbd0)
/// Original: `free_teammate` (0x6d5a70, match_eng.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d5a70 } (ret 4, match_eng.cpp): o primeiro companheiro de `p` (os 20 do lado dele, ativos) no
/// ataque fundo (0x70df40), no meio (0x70df20) e sem ninguem pressionando (0x6bcbd0 sem lista), ou 0.
pub(crate) unsafe extern "thiscall" fn free_teammate(m: usize, p: usize) -> usize {
    let l = side_of_player(p);
    for k in 0..20 {
        let j = side_player_slot(m, l, k);
        if read_i8(j + match_player::POSITION) < 0 || player_limit(j) == 0 || x_off_centre(j) != 0 {
            continue;
        }
        let mut sum = 0i16;
        if pressure_on_player_with(m, j, &mut sum, 0, &mut |_, _| {}) == 0 {
            return j;
        }
    }
    0
}

/// Os locais do chute (`shoot`) que o desfecho (gol, defesa, rebote) le: pelo deslocamento na pilha.
pub(crate) struct ShotLocals {
    pub(crate) p: usize,
    pub(crate) m: usize,
    pub(crate) g: usize,
    pub(crate) side: i8,
    pub(crate) kind: u8,
    pub(crate) a1: u32,
    pub(crate) v: i32,
    pub(crate) v0: u32,
    pub(crate) sound: u32,
    pub(crate) mode: u8,
    pub(crate) count: i8,
    pub(crate) list: usize,
    pub(crate) d: f32,
    pub(crate) l10: u8,
    pub(crate) l11: u8,
    pub(crate) l14: u8,
    pub(crate) l16: i8,
    pub(crate) l17: u8,
    pub(crate) l18: bool,
    pub(crate) l19: bool,
    pub(crate) l1a: bool,
    pub(crate) l1b: i8,
    pub(crate) l20: u8,
    pub(crate) l25: u8,
    pub(crate) l28: i8,
    pub(crate) l38_before: i16,
    pub(crate) l40: i16,
    pub(crate) l50: i8,
    pub(crate) l54: i8,
    pub(crate) l58: i16,
    pub(crate) l5c: i32,
    pub(crate) l60: i32,
    pub(crate) l64: i32,
    pub(crate) l68: u32,
    pub(crate) e: usize,
    pub(crate) l8c: i16,
    pub(crate) l12: i8,
    pub(crate) l13: i8,
    pub(crate) l15: u8,
    pub(crate) l26: u8,
    pub(crate) l4e: u8,
    pub(crate) l4f: u8,
}

/// Sends a 7-argument commentary call (0x6d6f70) through the tape, packing code | a7 << 16 and a5 | a6 << 8.
///
/// [pt] Narracao com sete argumentos pela fita (codigo | a7 << 16, a5 | a6 << 8).
pub(crate) fn narrate_packed7(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, code: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8) {
    unsafe {
        let narr = *((m + engine::COMMENTATOR) as *const usize);
        call(FN_MATCH_EVENTS_ADD_EVENT, narr, [(code & 0xffff) | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    }
}

/// `shoot` (MATCH_PLAYER shoot) with outside calls injected: the full shot of player p by type, value, commentary and
/// mode (play, no challenger, shootout, set piece, retake).
///
/// [pt] { 0x702050 } (ret 0x18, match_pl.cpp, ~35 KB) com as chamadas que mexem fora do motor injetadas:
/// o chute de `p` (tipo a1, valor a2, narracao a3, modo a4: 1 jogo, 2 sem desafiante, 3 penaltis, 4
/// cobranca, 5/6 repetir; a5/a6 a lista dos companheiros da pressao).
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
pub(crate) unsafe fn shoot_with(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let m = read_usize(p + match_player::ENGINE);
    let lado = read_i8(p + match_player::SIDE);
    let tipo = a1 as u8;
    let mut v = a2 as i32;
    let mut som = a3;
    let mut modo = a4 as u8;
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| narrate_packed7(chamar, m, codigo, a3, a4, a5, a6, a7);
    // 0x7002f0: dois campos -1; 0x70a860: tres
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrate_packed7(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let l60 = *dw(m, engine::MOMENTUM_B);
    let l5c = *dw(m, engine::MOMENTUM_A);
    let alvo_y = if lado == 0 { 0xb } else { 0 };
    let d = distance_to_point(p, 4, alvo_y);
    let (l58, l50, l54) = (*w(p, match_player::SHOT_QUALITY), read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
    let l8c = *w(p, match_player::ACTION);
    let mut l64: i32 = 0xf0;
    let v0 = a2;
    let mut l28: i8 = 6;
    let l25 = how_many_mark(m, p);
    let minuto = (*w(m, engine::CLOCK) as i32 / 11) as i16;
    let l16 = minuto as i8;
    *b(m, engine::LAST_SHOT_SIDE) = lado as u8;
    *w(m, engine::LAST_SHOT_MINUTE) = minuto;
    let l18 = !((*dw(m, engine::MOMENTUM_B) as f64) < read_f64(rdata::F64_1680));
    let l1b = ((ftol(d as f64 * read_f32(rdata::F32_9) as f64 + read_f64(rdata::F64_2_5)) as i8 as i32 / 5) * 5) as i8;
    if tipo == 0x39 || tipo == 3 {
        l28 = 5;
    }
    if tipo == 0x33 {
        l64 = 0x78;
    }
    let d2 = distance_to_point(p, 4, alvo_y);
    let mut l7c = 0u16;
    let mut l10 = classify_move(p, tipo as i8 as i16, l50, l54, 4, if lado == 1 { 0 } else { 0xb }, (d2 as f64 + read_f64(rdata::F64_1)) as f32, &mut l7c);
    *w(p, match_player::SHOT_QUALITY) = (*w(p, match_player::SHOT_QUALITY)).wrapping_add(-(l7c as i16 as i32 / 2) as i16);
    let r = random(l7c as i16 as i32);
    v = v.wrapping_sub(r);
    // whoever made the pass gets the shot assist
    let mut l68 = 0u32;
    let q = read_usize(m + engine::ASSIST_CANDIDATE);
    if p == read_usize(m + engine::PASS_RECEIVER) && q != 0 && !matches!(*w(p, match_player::ACTION), 0x12 | 0x11) && (*w(p, 0x29) > -100 || v as i16 > 100) && modo == 1 {
        if !matches!(*w(q, match_player::ACTION), 0x34 | 0x37) {
            crate::comparison::branch("shot assist");
            *b(q, 3) = (*b(q, 3)).wrapping_add(1);
            *w(q, match_player::MATCH_RATING) = (*w(q, match_player::MATCH_RATING)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
            *dw(m, engine::MOMENTUM_A) = (*dw(m, engine::MOMENTUM_A)).wrapping_add(0x113);
        }
        l68 = 1;
    }
    let g = hold_position_one(m, (lado == 0) as i8);
    if g == 0 {
        // simplification: the original stops the game (match_pl.cpp line 0x2e0b); without a goalkeeper there is no shot
        linha("motor: shoot: sem goleiro");
        std::process::abort();
    }
    let l1c: u8 = match tipo {
        0x16 | 0x39 => 0,
        0x17 | 0x1a | 0x1b => 1,
        0x18 | 0x19 | 0x1c => (random(3) != 0) as u8,
        0x3a => (random(4) == 0) as u8,
        _ => random(2) as u8,
    };
    let (mut l20, mut l14) = (0u8, 0u8);
    let mut l38 = goalkeeper_save(g, l50, l54, l1c, tipo as i8, &mut l20, &mut l14) as i16 as i32;
    let (mut l19, mut l1a) = (false, false);
    if l14 == 0 {
        if l38 as i16 > 0x177 {
            l1a = true;
        } else if l38 as i16 > 0x104 {
            l19 = true;
        }
    }
    // o desafiante
    let (mut l4c, mut l4d, mut l70, mut l30, mut l84) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if read_i8(m + engine::BALL_HEIGHT) != 4 && modo != 4 && tipo != 0x33 && tipo != 0x35 {
        opposition_challenge(m, p, lado, 0x6d, &mut l4c, &mut l4d, &mut l70, &mut l30, &mut l84, l20)
    } else {
        0
    };
    tackle_injury_with(m, p, 0x6d, e, l30 as i16, 0, chamar);
    if e != 0 {
        let mut penalty = false;
        let mut l44 = 0u8;
        'contest: {
            if l70 < *w(p, match_player::BALL_CONTROL) {
                // no foul: the clean contest
            } else {
                let (mut l88, mut l74) = (0u8, 0i8);
                let mut l2c = foul_in_engine(m, e, p, l30, -1, &mut l88, &mut l74, &mut l44);
                if read_i8(m + engine::LIVE_MATCH) != 0
                    && is_in_attacked_box(lado, read_i8(e + match_player::CELL_X), read_i8(e + match_player::CELL_Y))
                    && (special_code(l30) != 0 || l44 != 0 || random(*w(e, 0x31) as i32) < -10)
                    && random(3) != 0
                {
                    n7(chamar, 0x2168, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), 0xff, 0xff, 0);
                }
                if l2c != 0 {
                    if random(0x32) == 0 && modo != 4 && l84 == 0 {
                        crate::comparison::branch("shot foul only recorded");
                        record_move(m, l30 as i16 as u16, e as u32, p as u32, 0, l2c, l50 as u8, l54 as u8, l88, l74 as u8);
                        modo = 2;
                        break 'contest;
                    }
                    crate::comparison::branch("shot foul punished");
                    let mut peso = 0f32;
                    engine_punish_move(m, e, p, &mut l2c, l88 as i8, l74, l84 as u8, &mut peso, l44);
                    n7(chamar, foul_commentary_code(l30), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), 0);
                    if modo == 4 {
                        // on the kick, the foul becomes a free kick: the shot again (type 0x33)
                        crate::comparison::branch("shot foul on the kick");
                        n7(chamar, 0x1f7a, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
                        let mut t = 0x33i8;
                        let mut s = som as u16;
                        let val = shot_value(p, &mut t, &mut s);
                        let novo_som = (som & 0xffff_0000) | s as u32;
                        chamar(FN_MATCH_PLAYER_TAKE_SHOT, p, [(a1 & 0xffff_ff00) | t as u8 as u32, val as i16 as i32 as u32, novo_som, 4, 0]);
                        return;
                    }
                    shot_foul_with(m, e, p, l2c, l74, l44, l30, peso, chamar);
                    return;
                }
            }
            penalty = l44 != 0 && (player_band(p, (lado == 0) as u8) as u8 != 0 || l84 != 0);
        }
        // a disputa limpa (0x702675)
        *w(e, match_player::ACTION) = l30 as i16;
        *b(e, 4) = (*b(e, 4)).wrapping_add(1);
        let t = table_a(m, e);
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(if t & 0x30 != 0 && table_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
        if v as i16 > l70 {
            crate::comparison::branch("shot gets past the challenger");
            *b(e, match_player::BUSY) = (*b(e, match_player::BUSY)).wrapping_add(l4d);
        } else {
            crate::comparison::branch("shot tackled");
            shot_tackled_with(m, p, e, l30, l70, v, l4c, l44, penalty, chamar);
            return;
        }
    } else if !matches!(tipo, 0x33 | 0x35 | 3 | 0x39) && modo != 4 {
        // no challenger: the approach commentary
        if player_band(p, (lado == 0) as u8) as u8 != 0 && a5 as u8 == 0 && set_piece_code(*w(p, match_player::ACTION)) != 0 {
            na(chamar, 0x1f43, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0);
        } else if random(10) == 0 && (0x14..=0x23).contains(&l1b) && player_band(p, (lado == 0) as u8) as u8 == 0 && read_i8(p + match_player::CELL_X) < 6 && read_i8(p + match_player::CELL_X) > 2 {
            na(chamar, 0x2161, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, l1b);
        }
    }
    // 0x703634: the shot commentary
    if l10 == read_i8(p + match_player::PREFERRED_FOOT) as u8 && random(10) != 0 {
        l10 = 0xff;
    }
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    if tipo == 3 {
        if a5 as u8 == 0 && e == 0 {
            na(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 7);
        } else {
            som = shot_commentary_by_class(som, l10);
            na(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0);
        }
    } else if tipo == 0x1b || tipo == 0x1a {
        if tg() != 1 {
            na(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0);
        } else if l20 == 4 && distance_to_point(p, read_i8(g + match_player::CELL_X), read_i8(g + match_player::CELL_Y)) as f64 > read_f64(rdata::F64_1_5) {
            crate::comparison::branch("shot goalkeeper off his line");
            n7(chamar, 0x1fe0, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, (read_u8(g + match_player::COMMENTARY_ID) as u32), (lado == 0) as u32, 0);
            n7(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0);
            l20 = 0;
        } else {
            let novo = match som as u16 as i16 as i32 {
                0x1f89 => Some(0x1f8a),
                0x1f8b => Some(0x1f8c),
                0x1fa3 => Some(0x1fa4),
                0x1fa5 => Some(0x1fa6),
                0x1fb1 => Some(0x1fb2),
                0x1fb3 => Some(0x1fb4),
                _ => None,
            };
            if let Some(c) = novo {
                som = c;
            }
            l20 = 0;
            n7(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0);
        }
    } else {
        som = shot_commentary_by_class(som, l10);
        na(chamar, som, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0);
    }
    // 0x703883: o impedimento
    *w(p, match_player::ACTION) = 0x6d;
    if tipo != 0x33 && tipo != 0x35 && modo == 1 {
        let o = engine_look_for_offside(m, lado, read_i8(p + match_player::CELL_Y), if lado == 0 { 0xb } else { 0 }, p);
        if o != 0 {
            if random(500) != 0 {
                crate::comparison::branch("shot offside recorded");
                record_move(m, 0xffff, o as u32, 0, 0, 8, *b(o, match_player::CELL_X), *b(o, match_player::CELL_Y), 0xff, 0xff);
                modo = 2;
            } else {
                crate::comparison::branch("shot offside");
                na(chamar, 0x1f78, (read_u8(o + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0);
                let k = m + engine::OFFSIDES + (lado as isize * 0x36) as usize;
                *b(k, 0) = (*b(k, 0)).wrapping_add(1);
                *b(o, match_player::OFFSIDES) = (*b(o, match_player::OFFSIDES)).wrapping_add(1);
                *b(m, engine::BALL_X) = *b(o, match_player::CELL_X);
                *b(m, engine::BALL_Y) = *b(o, match_player::CELL_Y);
                *b(m, engine::RESTART_TYPE) = 7;
                *b(m, engine::POSSESSION_SIDE) = (lado == 0) as u8;
                *((m + engine::BALL_HOLDER) as *mut usize) = 0;
                return;
            }
        }
    }
    *b(m, engine::POSSESSION_SIDE) = 0xff;
    *b(m, engine::SHOT_PENDING) = 0;
    let l40 = *w(p, 0x29);
    *w(p, 0x29) = 0;
    *w(p, match_player::BALL_CONTROL) = 0;
    let mut l11 = 0u8;
    if tipo != 0x33 && modo != 4 {
        let forca = if (v as i16) < 0 { 0 } else { v as i16 as i32 };
        l11 = chamar(FN_CM3_MATCH_ENGINE_CHECK_SHOT_BLOCKED_ON_PATH, m, [l50 as u8 as u32 | (l54 as u8 as u32) << 8, forca as u32, (lado == 0) as u32, l28 as u8 as u32, (tipo == 0x35) as u32]) as u8;
        if read_i8(m + engine::RESTART_TYPE) != 0 {
            return;
        }
    }
    if read_usize(m + engine::BALL_HOLDER) == p {
        *((m + engine::BALL_HOLDER) as *mut usize) = 0;
    }
    if l11 == 2 {
        // the shot hit someone on the way
        crate::comparison::branch("shot blocked");
        let q = read_usize(m + engine::SHOT_BLOCKER);
        if l28 == 5 {
            na(chamar, 0x1fd4, (read_u8(q + match_player::COMMENTARY_ID) as u32), (read_u8(q + match_player::SIDE) as u32), 5);
        } else {
            na(chamar, 0x1fd3, (read_u8(q + match_player::COMMENTARY_ID) as u32), (read_u8(q + match_player::SIDE) as u32), l28);
        }
        let s1 = random(2000);
        let s2 = random(1000);
        v = v.wrapping_add(s2.wrapping_sub(s1));
        *w(p, match_player::SHOT_QUALITY) = 0;
    }
    // the outcome class
    let tem = l1c != 0;
    l10 = if v as i16 > l64 as i16 {
        if random(v as i16 as i32) < 10 { 0xe + tem as u8 } else { 1 }
    } else if (v as i16) < 10 {
        if tem { if random(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let dif = l64 as i16 as i32 - v as i16 as i32;
        if dif > 0x3c {
            if tem { if random(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if dif > 0x19 {
            8 + tem as u8
        } else if dif > 0xe {
            0xa + tem as u8
        } else {
            0xc + tem as u8
        }
    };
    let no_alvo = matches!(l10, 1 | 0xe | 0xf);
    match modo {
        3 | 4 => {
            let k = m + 0x1a9 + lado as isize as usize;
            *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        }
        1 => {
            if tipo == 0x33 {
                *b(p, match_player::PENALTIES_TAKEN) = (*b(p, match_player::PENALTIES_TAKEN)).wrapping_add(1);
            }
            let q = read_usize(m + engine::SHOT_BLOCKER);
            if tipo == 0x33 || q == 0 || (read_i8(q + match_player::CELL_X) == 4 && (read_i8(q + match_player::CELL_Y) == 0 || read_i8(q + match_player::CELL_Y) == 0xb)) {
                *b(p, match_player::SHOTS) = (*b(p, match_player::SHOTS)).wrapping_add(1);
                *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(if tipo != 0x33 { -150 } else { -1000 });
                if no_alvo {
                    *b(p, match_player::SHOTS_ON_TARGET) = (*b(p, match_player::SHOTS_ON_TARGET)).wrapping_add(1);
                    let k: i32 = if tipo != 0x33 { 0xdc } else { 400 };
                    if l10 == 0xe || l10 == 0xf {
                        *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(ftol(k as f64 * read_f64(rdata::F64_0_5)) as i16);
                    } else {
                        *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(k as i16);
                    }
                }
            }
            if !no_alvo {
                let di = ((*w(p, match_player::SHOT_QUALITY)).wrapping_sub(v as i16).wrapping_add(0xf0)).clamp(0, 200);
                *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_sub(di);
                if random(read_i8(read_usize(p + match_player::STAFF) + 0x5a) as i32) == 0 && adjust_17(p, (-(di as i32 / 50)) as i8) != 0 {
                    match_performance(p, 0);
                }
            }
        }
        _ => {}
    }
    if l11 == 1 {
        // the ball stopped on the way
        let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
        if bx < 0 || by >= 0xc || (bx >= 9 && by < 0) {
            chamar(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [(lado == 0) as u32, 0, 0, 0, 0]);
        }
        *b(m, engine::SHOT_REBOUND) = 1;
        return;
    }
    if lado == 1 {
        *b(m, engine::BALL_X) = 4;
        *b(m, engine::BALL_Y) = 0;
    } else if lado == 0 {
        *b(m, engine::BALL_X) = 4;
        *b(m, engine::BALL_Y) = 0xb;
    }
    l64 = l1b as i32;
    if l1b as i32 > random(0xf) + 0x19 {
        // long shot: everyone decides again with dead ball 5
        crate::comparison::branch("shot from distance");
        let guardado = *b(m, engine::BALL_HEIGHT);
        *b(m, engine::BALL_HEIGHT) = 5;
        for l in 0..2i8 {
            for k in 0..20 {
                let j = side_player_slot(m, l, k);
                if (0..=10).contains(&read_i8(j + match_player::POSITION)) && read_i8(j + match_player::BUSY) == 0 && j != p {
                    chamar(FN_MATCH_PLAYER_TAKE_TURN, j, [0; 5]);
                }
            }
        }
        *b(m, engine::BALL_HEIGHT) = guardado;
    }
    if l20 != 0 {
        if modo == 1 && tg() == 1 {
            *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_sub(800);
            if random(read_i8(read_usize(g + match_player::STAFF) + 0x5a) as i32 * 20 + 10) == 0 {
                let r = random(3);
                adjust_17(g, (r as u8).wrapping_neg() as i8);
            }
            match_performance(g, 0);
        }
        if (lado == 0 && read_i8(p + match_player::CELL_X) > read_i8(g + match_player::CELL_X)) || (lado == 1 && read_i8(p + match_player::CELL_X) < read_i8(g + match_player::CELL_X)) {
            v = random(400).wrapping_add(v).wrapping_add(0xbe);
        } else {
            v = v.wrapping_add(0xbe);
        }
    }
    if read_usize(m + engine::SHOT_BLOCKER) == g {
        l38 = 0;
    }
    if tipo == 0x39 {
        *b(p, match_player::BUSY) = (*b(p, match_player::BUSY)).wrapping_add(2);
    }
    let (mut l12, mut l13, mut l26, mut l4e, mut l4f) = (0i8, 0i8, 0u8, 0u8, 0u8);
    aggregate_score(m, lado, &mut l12, &mut l13, &mut l26, &mut l4e, &mut l4f, 1);
    let l38_antes = l38 as i16;
    l38 = (l38 as i16 as i32) - if l14 != 0 { 100 } else { 0 } - *w(p, match_player::SHOT_QUALITY) as i32 + v as i16 as i32 - 0x64;
    let fator = match tipo {
        5 => read_f32(rdata::F32_5_5),
        0x17 | 0x1c => read_f32(rdata::F32_1_5),
        0x18 => read_f32(rdata::F32_2_5),
        0x19 => read_f32(rdata::F32_1_75),
        0x1a | 0x39 => read_f32(rdata::F32_2_25),
        0x1b => read_f32(rdata::F32_2),
        0x35 => read_f32(rdata::F32_3),
        0x3a => read_f32(rdata::F32_0_5),
        _ => read_f32(rdata::F32_1),
    } as f64;
    let ganho = if l38 < 0 { l38 as f64 / fator } else { l38 as f64 * fator };
    *dw(m, engine::MOMENTUM_B) = (*dw(m, engine::MOMENTUM_B)).wrapping_add(ftol(ganho));
    let c = ShotLocals {
        p, m, g, side: lado, kind: tipo, a1, v, v0, sound: som, mode: modo, count: a5 as u8 as i8, list: a6, d, l10, l11, l14, l16,
        l17: 0, l18, l19, l1a, l1b, l20, l25, l28, l38_before: l38_antes, l40, l50, l54, l58, l5c, l60, l64, l68,
        e, l8c, l12, l13, l15: 0, l26, l4e, l4f,
    };
    if no_alvo {
        shot_on_target_with(c, chamar);
        return;
    }
    shot_off_target_with(&c, chamar);
}

/// Maps a shot commentary code to its class variant (0x1f86, 0x1f8e, 0x1f93, 0x1f96, 0x1f99 become the next code, +1
/// when class is non-zero); class 0xff keeps it.
///
/// [pt] A narracao do chute pela classe (0x7037ee): os codigos 0x1f86, 0x1f8e, 0x1f93, 0x1f96 e 0x1f99
/// passam ao seguinte (+1 com a classe diferente de zero); a classe 0xff (a do lance repetido) nao muda.
pub(crate) fn shot_commentary_by_class(sound: u32, class: u8) -> u32 {
    if class == 0xff {
        return sound;
    }
    let k = (class != 0) as u32;
    match sound as u16 as i16 as i32 {
        0x1f86 => 0x1f87 + k,
        0x1f8e => 0x1f8f + k,
        0x1f93 => 0x1f94 + k,
        0x1f96 => 0x1f97 + k,
        0x1f99 => 0x1f9a + k,
        _ => sound,
    }
}

/// The challenger's foul on the shot (part of `shoot`): penalty or free kick, card, narration and disposition loss for
/// the fouled player.
///
/// [pt] A falta do desafiante no chute (0x702050, de 0x702934 a 0x702f1a): como a do passe, sem o evento
/// 0x82 e sem as narracoes do lance 0x2c/0x2f; o perto do gol vem de 0x70df70.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn shot_foul_with(m: usize, e: usize, p: usize, kind: u8, grav: i8, advantage: u8, lance: i8, weight: f32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let side = read_i8(p + match_player::SIDE);
    let n7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32| narrate_packed7(call, m, code, a3, a4, 0xff, 0xff, 0);
    let na = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32| narrate_packed7(call, m, code, a3, a4, 0xff, 0xff, 0);
    let nb = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a4: u32| narrate_packed7(call, m, code, 0xff, a4, 0xff, 0xff, 0);
    let colour: i32 = match kind {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > random(7) + 2 { 0x210a } else { -1 }
        }
    };
    let sound: u32;
    if kind == 7 {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 7;
    } else if is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) {
        crate::comparison::branch("shot penalty");
        if colour as i16 == -1 {
            match random(4) {
                0 => n7(call, 0x20e9, 0xff, side as u8 as u32),
                1 => n7(call, 0x20ec, 0xff, side as u8 as u32),
                _ => {}
            }
        }
        sound = 0x1f7a;
        *b(m, engine::RESTART_TYPE) = 5;
        *((m + engine::FOULED_PLAYER) as *mut usize) = p;
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(500);
        if random(read_i8(read_usize(e + match_player::STAFF) + 0x5a) as i32) == 0 {
            let r = random(10);
            if adjust_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                match_performance(e, 0);
            }
        }
    } else {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 6;
    }
    *b(m, engine::BALL_X) = *b(p, match_player::CELL_X);
    *b(m, engine::BALL_Y) = *b(p, match_player::CELL_Y);
    *b(m, engine::POSSESSION_SIDE) = *b(p, match_player::SIDE);
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    if sound == 0x1f7a {
        n7(call, 0x1f7a, 0xff, side as u8 as u32);
    }
    let sent_off = matches!(kind, 3..=6);
    if colour as i16 > -1 {
        if random(4) == 0 {
            let x = random(grav as i32);
            if x > random(0x14) {
                let t = read_usize(m + engine::MATCH);
                let club = Club(read_usize(t + (if read_i8(e + match_player::SIDE) == 0 { 0x20 } else { 0x1c })));
                let k = club.nation().addr();
                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                    n7(call, 0x20f5, 0, 0);
                }
            }
            n7(call, 0x2109, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32));
        }
        n7(call, colour as u32, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32));
        if sent_off {
            crate::comparison::branch("shot sending off");
            player_left_pitch_with(m, e, 0, call);
        }
    }
    if sound != 0x1f7a {
        let c = choose_taker(m, side, read_i8(m + engine::RESTART_TYPE), 0);
        if c == 0 {
            n7(call, sound, 0xff, side as u8 as u32);
        } else {
            if kind == 7 {
                n7(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
            } else {
                n7(call, sound, 0xff, side as u8 as u32);
            }
            if sound == 0x1f79 && advantage == 0 {
                if kind == 7 {
                    if near_goal(p) != 0 && player_band(p, (side == 0) as u8) as u8 == 0 {
                        n7(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
                    }
                } else if off_centre(read_i8(p + match_player::CELL_X)) != 0 && near_goal(p) != 0 {
                    na(call, 0x21df, (read_u8(c + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
                } else if near_goal(p) != 0 {
                    na(call, 0x21e0, (read_u8(c + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
                }
            }
        }
    }
    if special_code(lance) == 0 {
        let extra = sent_off || sound == 0x1f7a;
        let zero = (side == 0) as u32;
        if advantage != 0 {
            na(call, 0x20dd, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
            if kind == 4 || kind == 5 {
                *b(e, match_player::SENT_OFF) = 1;
            }
            if extra && random(2) == 0 {
                nb(call, 0x20d9, zero);
            }
        } else if random(9) == 0 {
            nb(call, 0x20df, zero);
            if random(3) == 0 {
                nb(call, 0x20e0, zero);
            }
            if extra && random(2) == 0 {
                nb(call, 0x20d9, zero);
            }
        }
        apply_to_team(m, e, weight);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e, [0; 5]);
    }
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x19);
}

/// The challenger won the ball on the shot (part of `shoot`, from 0x702f35 to 0x70353b): commentary, control or ball
/// deflection, and the shooter's delay (+0x2b) and gains.
///
/// The commentary (in his own area, 1 in 2, the foul one and 0x20e8/0x20ea/0x20eb; otherwise by incident, with the save
/// 0x70d430 on incident 1); he controls it (`take_ball`, 0x70a890 with rating − shot − 200), the ball deflects below the
/// incident's threshold (`exe::cm3_match_engine_move_ball_to_cell`) or he aims for the side of the pitch; then the gains and the +0x2b of the player who
/// shot.
///
/// [pt] O desafiante tirou a bola no chute (0x702050, de 0x702f35 a 0x70353b): a narracao (na area dele,
/// 1 em 2, a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa 0x70d430 no lance 1),
/// ele domina (0x6f0630, 0x70a890 com nota - chute - 200), a bola espirra abaixo do limite do lance
/// (0x6cd770) ou ele aponta para o lado do campo; os ganhos e o +0x2b de quem chutou.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn shot_tackled_with(m: usize, p: usize, e: usize, lance: i8, rating: i16, v: i32, a: u8, advantage: u8, penalty: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let side = read_i8(p + match_player::SIDE);
    let n7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrate_packed7(call, m, code, a3, a4, a5, a6, 0);
    let nb = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a4: u32| narrate_packed7(call, m, code, 0xff, a4, 0xff, 0xff, 0);
    let narrated = (special_code(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
        && player_band(e, read_i8(e + match_player::SIDE) as u8) as u8 != 0
        && random(2) == 0;
    if narrated {
        crate::comparison::branch("shot tackle narrated in the box");
        n7(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32));
        let zero = (read_i8(e + match_player::SIDE) == 0) as u32;
        match random(0x32) {
            0 => nb(call, 0x20eb, zero),
            1 => nb(call, 0x20ea, zero),
            _ => nb(call, 0x20e8, zero),
        }
    } else {
        let code = match lance {
            1 => {
                crate::comparison::branch("shot goalkeeper tackle");
                if save_shot_injected(e, (rating as i32).wrapping_sub(v) as i16, call) == 0 {
                    return;
                }
                0
            }
            0x13 | 0x21 | 0x2d => 0x1f62,
            0x22 | 0x2c => 0x2150,
            0x29 => 0x1f61,
            0x2a | 0x2b => 0x1f64,
            0x15 | 0x2e => {
                let r = random(6);
                if (r as f64 + read_f32(e + match_player::JUMPING) as f64 + read_f32(rdata::F32_3) as f64) < read_f32(p + match_player::JUMPING) as f64 { 0x2164 } else { 0x1f5d }
            }
            _ => 0,
        };
        if code != 0 {
            n7(call, code, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32));
        }
    }
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, e, [0; 5]);
    write_19c(e, (rating as i32).wrapping_sub(v).wrapping_sub(200) as i16);
    let mut controlled = false;
    let limit: i16 = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, match_player::BALL_CONTROL) < limit && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 {
        crate::comparison::branch("shot ball breaks loose");
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(p + match_player::CELL_X) as u32), (read_u8(p + match_player::CELL_Y) as u32), 1 | ((side == 0) as u32) << 8, 0, 0]);
        if read_i8(m + engine::RESTART_TYPE) == 0 {
            let mut k = 3;
            let (c10d, c10c) = (read_i8(e + match_player::MOMENT_WORK_RATE), read_i8(e + match_player::MOMENT_AGILITY));
            k -= (c10d as i32 > random(10)) as i32;
            k -= (c10c as i32 > random(10)) as i32;
            *b(e, match_player::BUSY) = random(k) as u8;
            controlled = true;
        } else if penalty {
            call(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, p, [0; 5]);
        }
    } else {
        if read_i8(m + engine::BALL_HEIGHT) < 4 && read_i8(m + engine::BALL_HEIGHT) > -1 {
            *b(m, engine::BALL_HEIGHT) = 0;
        }
        let (x, y) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
        if read_i8(e + match_player::SIDE) == 0 && read_i8(m + engine::BALL_HEIGHT) != -1 {
            aim_set_facing_direction(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            aim_set_facing_direction(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = read_usize(e + match_player::ENGINE);
    let t = || if me != 0 { table_a(me, e) } else { 0 };
    let gain: i16 = if controlled { 0x78 } else { 0xc8 };
    let far = t() & 4 == 0 && t() & 2 == 0;
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(if far { gain << 1 } else { gain });
    *b(p, match_player::BUSY) = (*b(p, match_player::BUSY)).wrapping_add(a);
    if player_band(p, (side == 0) as u8) as u8 != 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if controlled && random(7) == 0 {
        narrate_packed7(call, m, 0x1f41, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), 0xff, 0xff, 0);
    }
}

/// True if the player's record name (+0x69 -> +8) is "Shearer" (his special goal celebration).
///
/// [pt] O nome (+0x69 -> +8) da ficha e "Shearer" (0xa16110): a comemoracao dele.
pub(crate) unsafe fn is_shearer(p: usize) -> bool {
    let sheet = Staff(*((p + match_player::STAFF) as *const usize));
    let name = *((sheet.addr() + staff::SECOND_NAME) as *const *const u8);
    std::ffi::CStr::from_ptr(name as *const std::ffi::c_char).to_bytes() == b"Shearer"
}

/// Narrates the first of the given highlight codes that MATCH_PLAYER highlight (`player_highlight`) accepts for player q; returns
/// whether one was narrated.
///
/// [pt] Os destaques do jogador (0x717470), na ordem: o primeiro que valer e narrado com (q.2e, a4, 0).
pub(crate) unsafe fn narrate_first_highlight(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, q: usize, a4: u32, codes: &[u32]) -> bool {
    for &c in codes {
        if player_highlight(q, c) != 0 {
            narrate_packed7(call, m, c, read_i8(q + match_player::COMMENTARY_ID) as u8 as u32, a4, 0xff, 0xff, 0);
            return true;
        }
    }
    false
}

pub(crate) const HIGHLIGHTS_P: [u32; 4] = [0x219e, 0x21b1, 0x21b2, 0x21b7];
pub(crate) const HIGHLIGHTS_R: [u32; 6] = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
pub(crate) const GOAL_HIGHLIGHTS: [u32; 7] = [0x219f, 0x21a0, 0x21a1, 0x21b0, 0x21b3, 0x21b4, 0x21b5];

/// The shot on target (end of `shoot`): goal by the shot/(save + shot) ratio, else the goalkeeper's save; ratings and
/// narration.
///
/// [pt] O chute no alvo (0x702050 de 0x706cba ao fim): a defesa do goleiro ou o gol.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn shot_on_target_with(mut c: ShotLocals, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrate_packed7(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    let ajustar = |q: usize, delta: i32| {
        if adjust_17(q, delta as i8) != 0 {
            match_performance(q, 0);
        }
    };
    // the goal by the ratio shot / (save + shot)
    let l6c = (c.l38_before as i32).wrapping_add(c.v as i16 as i32);
    let mut razao: f32;
    let gol = if l6c != 0 {
        razao = ((c.v as i16 as i32).wrapping_mul(100) as f64 / l6c as f64) as f32;
        if m != 0 && !(razao as f64 > read_f32(rdata::F32_68) as f64) && matches!(read_i8(m + 1), 2 | 8) {
            razao = (razao as f64 + read_f32(rdata::F32_2) as f64) as f32;
        }
        razao as f64 > read_f32(rdata::F32_71) as f64
    } else {
        razao = 0.0;
        false
    };
    if !gol {
        if c.l19 && tg() == 1 {
            *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_add(0x15e);
            if random(read_i8(g + match_player::MATCH_FORM) as i32 * 20 + 0x32) == 0 {
                let r = random(4);
                ajustar(g, r);
            }
        } else if c.l1a && tg() == 1 {
            *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_add(0x258);
            if random((read_i8(g + match_player::MATCH_FORM) as i32 + 1) * 20) == 0 {
                let r = random(7);
                ajustar(g, r);
            }
        }
        if !(razao as f64 > read_f32(rdata::F32_71) as f64) {
            goalkeeper_save_with(c, razao, chamar);
            return;
        }
    }
    // o gol (0x706dfd)
    crate::comparison::branch("shot goal");
    let modo = c.mode;
    if modo == 1 {
        if side_of_move(m) as i8 > -1 {
            clear_move(m);
        }
        let k = read_usize(m + engine::FOULED_PLAYER);
        if c.kind == 0x33 && k != 0 && k != p {
            *b(k, match_player::ASSISTS) = (*b(k, match_player::ASSISTS)).wrapping_add(1);
        }
        let base = m + (lado as isize * 0x36) as usize;
        match read_i8(m + engine::SET_PIECE_KIND) {
            -1 | 7 => *b(base, engine::FREE_KICK_PLAY_GOALS) = (*b(base, engine::FREE_KICK_PLAY_GOALS)).wrapping_add(1),
            3 => *b(base, engine::CORNER_GOALS) = (*b(base, engine::CORNER_GOALS)).wrapping_add(1),
            6 => *b(base, engine::DIRECT_FREE_KICK_GOALS) = (*b(base, engine::DIRECT_FREE_KICK_GOALS)).wrapping_add(1),
            _ => {}
        }
        if marked_moment(m) as u8 != 0 {
            *b(m, engine::GOLDEN_GOAL_SCORED) = 1;
        }
        if c.kind == 0x33 {
            *b(p, match_player::PENALTIES_SCORED) = (*b(p, match_player::PENALTIES_SCORED)).wrapping_add(1);
        }
    } else if modo == 3 || modo == 4 {
        shootout_goal_with(&c, chamar);
        goal_stands_with(&mut c, chamar);
        return;
    }
    if c.l11 == 0 {
        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
            *w(p, o) = (*w(p, o)).wrapping_add(1);
        }
        *b(p, match_player::GOALS) = (*b(p, match_player::GOALS)).wrapping_add(1);
        c.l17 = 1;
    }
    if modo == 1 {
        if c.l11 == 0 {
            if c.kind == 0x33 {
                *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(0x2bc);
            } else {
                *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(0x28a);
                if random((read_i8(p + match_player::MATCH_FORM) as i32 + 1) * 20) == 0 {
                    let r = random(8);
                    ajustar(p, r);
                }
            }
            let piso = ((read_u8(p + match_player::GOALS) as u32) as i32 + 5) * 1000;
            if (*w(p, match_player::MATCH_RATING) as i32) <= piso {
                *w(p, match_player::MATCH_RATING) = piso as i16;
            }
        } else {
            *b(p, match_player::ASSISTS) = (*b(p, match_player::ASSISTS)).wrapping_add(1);
            *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(0x113);
            if random(read_i8(p + match_player::MATCH_FORM) as i32 * 20 + 0x32) == 0 {
                let r = random(4);
                ajustar(p, r);
            }
            let piso = ftol(((read_u8(p + match_player::ASSISTS) as u32) as f64 * read_f32(rdata::F32_0_5) as f64 + read_f64(rdata::F64_4_5)) * read_f64(rdata::F64_1000)) as i16;
            if *w(p, match_player::MATCH_RATING) <= piso {
                *w(p, match_player::MATCH_RATING) = piso;
            }
        }
        if tg() == 1 {
            // the goalkeeper conceded the goal
            *b(g, match_player::GOALS_CONCEDED) = (*b(g, match_player::GOALS_CONCEDED)).wrapping_add(1);
            let ficha = Staff(read_usize(g + match_player::STAFF));
            if random(ficha.determination() as i32 + 1) != 0 && random(ficha.pressure() as i32 + 1) != 0 {
                let r = random((read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32 * (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32) as i8;
                let soma = if (r as i32) < (read_u8(p + match_player::GOALS_CONCEDED) as u32) as i32 { *b(p, match_player::GOALS_CONCEDED) } else { r as u8 };
                *b(g, match_player::FRUSTRATION) = (*b(g, match_player::FRUSTRATION)).wrapping_add(soma);
                match_performance(g, 0);
            }
            if c.kind != 0x33 {
                *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_sub(500);
            }
            let di = *w(g, match_player::MATCH_RATING);
            if c.l14 != 0 {
                let mut v = (c.d as f64 + read_f64(rdata::F64_0_25)) * read_f64(rdata::F64_NEG_600) * read_f64(rdata::F64_0_5);
                if !(v > read_f64(rdata::F64_NEG_1000)) {
                    v = read_f64(rdata::F64_NEG_1000);
                }
                *w(g, match_player::MATCH_RATING) = (ftol(v) as i16).wrapping_add(di);
            }
        }
        let q = read_usize(m + engine::ASSIST_CANDIDATE);
        if p == read_usize(m + engine::PASS_RECEIVER) && q != 0 {
            *b(q, match_player::ASSISTS) = (*b(q, match_player::ASSISTS)).wrapping_add(1);
            *w(q, match_player::MATCH_RATING) = (*w(q, match_player::MATCH_RATING)).wrapping_add(0x113);
            if c.l68 == 0 && !matches!(*w(q, match_player::ACTION), 0x34 | 0x37) {
                *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                *w(q, match_player::MATCH_RATING) = (*w(q, match_player::MATCH_RATING)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
                *dw(m, engine::MOMENTUM_A) = (*dw(m, engine::MOMENTUM_A)).wrapping_add(0x113);
            }
        }
    }
    // 0x70717c: the goal commentary
    if c.kind != 0x33 && c.kind != 0x3a && c.l14 == 0 && random(0x32) == 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, if c.kind == 0x1a { 0x21d2 } else { 0x20d6 }, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0);
    }
    na(chamar, 0x1fff, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
    if c.kind == 0x33 {
        na(chamar, if modo == 1 { 0x1fcf } else { 0x20f8 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
    } else {
        *dw(m, engine::MOMENTUM_A) = (*dw(m, engine::MOMENTUM_A)).wrapping_sub(*dw(m, engine::MOMENTUM_B));
        narrate_goal_with(&mut c, chamar);
    }
    goal_stands_with(&mut c, chamar);
}

/// Goal commentary by context (part of `shoot`): comeback, aggregate, Shearer celebration, marker, assist and
/// goalkeeper.
///
/// [pt] As narracoes do gol pelo momento (0x70724d a 0x707cc6): a virada no placar (+0x4782, +0x4786), o
/// gol de quem esta atras no agregado, a comemoracao do Shearer, quem marcava, a assistencia e o
/// goleiro.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn narrate_goal_with(c: &mut ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let na = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a7: i8| narrate_packed7(call, m, code, a3, a4, 0xff, 0xff, a7);
    let n7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrate_packed7(call, m, code, a3, a4, a5, a6, 0);
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    let adjust = |q: usize, delta: i32| {
        if adjust_17(q, delta as i8) != 0 {
            match_performance(q, 0);
        }
    };
    let quem_bloqueou = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| {
        let q = read_usize(m + engine::SHOT_BLOCKER);
        na(call, code, (read_u8(q + match_player::COMMENTARY_ID) as u32), (read_u8(q + match_player::SIDE) as u32), 0);
    };
    let atras = || behind_on_aggregate(m, lado) != 0 && {
        let r = random(10);
        read_i8(read_usize(p + match_player::STAFF) + 0x58) > (r as u8).wrapping_add(0xe) as i8
    };
    let l4786 = *dw(m, engine::MOMENTUM_B);
    let l4782 = || *dw(m, engine::MOMENTUM_A) as f64;
    if side_of_move(m) as i8 > -1 || c.mode == 6 || c.mode == 5 {
        // 0x707c86
        if c.l11 == 0 {
            na(call, 0x20f9, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        } else {
            quem_bloqueou(call, 0x20fa);
        }
        return;
    }
    if c.l18 && c.l11 == 0 {
        na(call, 0x201a, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        if atras() {
            na(call, 0x21be, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
        return;
    }
    if l4786 > 0x578 {
        let code = if l4782() > read_f64(rdata::F64_1680) {
            0x2013
        } else if l4782() > read_f64(rdata::F64_1080) {
            0x2015
        } else {
            0x201b
        };
        na(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        let r = random(8);
        adjust(p, r);
        if atras() {
            na(call, 0x21be, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
        return;
    }
    if c.l18 && c.l11 != 0 {
        na(call, 0x201c, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        quem_bloqueou(call, if c.mode == 1 { 0x1fcd } else { 0x20fa });
        let r = random(6);
        adjust(p, r);
        return;
    }
    if l4786 > 0x384 {
        na(call, if l4782() > read_f64(rdata::F64_1080) { 0x2016 } else { 0x2018 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        let r = random(6);
        adjust(p, r);
        return;
    }
    if l4782() > read_f64(rdata::F64_1680) {
        na(call, 0x2019, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        let r = random(4);
        adjust(p, r);
        return;
    }
    if c.mode != 1 {
        // 0x707bcb
        if c.l11 != 0 {
            quem_bloqueou(call, 0x20fa);
            return;
        }
        na(call, 0x20f9, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        if read_i8(m + engine::LIVE_MATCH) != 0 && random(8) == 0 && is_shearer(p) {
            na(call, 0x216c, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
        return;
    }
    if c.l11 != 0 {
        quem_bloqueou(call, 0x1fcd);
        return;
    }
    na(call, 0x1fcc, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
    if read_i8(m + engine::LIVE_MATCH) != 0 && random(3) != 0 && is_shearer(p) {
        crate::comparison::branch("shot Shearer goal");
        na(call, 0x216c, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
    }
    // quem marcava o artilheiro
    let markers = how_many_mark(m, p);
    c.l16 = markers as i8;
    if c.kind != 0x35 && markers != 0 {
        let mk = who_marks(m, p);
        if mk == 0 {
            return;
        }
        let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
        if c.count == 0 {
            if c.e != 0 {
                return;
            }
            if markers == 1 {
                if *w(mk, match_player::ACTION) != 0x6e && *w(mk, match_player::MATCH_RATING) < 0x1a5e && *w(p, match_player::MATCH_RATING) > 0x1a5e && minute > 0xf {
                    n7(call, 0x21c2, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(mk + match_player::COMMENTARY_ID) as u32), (read_u8(mk + match_player::SIDE) as u32));
                }
            } else if read_i8(p + match_player::MATCH_GRADE) as i32 > 0x1c52 {
                n7(call, 0x21c3, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(mk + match_player::COMMENTARY_ID) as u32), (read_u8(mk + match_player::SIDE) as u32));
            }
            return;
        }
        let (pe, me) = (read_usize(p + match_player::PLAYER), read_usize(mk + match_player::PLAYER));
        if c.e == 0 || pe == 0 || me == 0 || *w(p, match_player::MATCH_RATING) <= 0x1a5e || *w(mk, match_player::MATCH_RATING) >= 0x1c52 || mk != c.e {
            return;
        }
        if (c.kind == 3 || c.kind == 0x39) && read_i8(pe + player_sheet::JUMPING) > 10 && read_i8(pe + player_sheet::JUMPING) as i32 - read_i8(me + player_sheet::JUMPING) as i32 > 3 {
            n7(call, 0x21c4, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(mk + match_player::COMMENTARY_ID) as u32), (read_u8(mk + match_player::SIDE) as u32));
        } else if matches!(c.l8c, 0x70 | 0x6a | 0x69 | 0x12) && read_i8(pe + player_sheet::PLAYER_PACE) > 10 && read_i8(pe + player_sheet::PLAYER_PACE) as i32 - read_i8(me + player_sheet::PLAYER_PACE) as i32 > 3 {
            n7(call, 0x21c5, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(mk + match_player::COMMENTARY_ID) as u32), (read_u8(mk + match_player::SIDE) as u32));
        } else if minute > 0x3c && opponent_state(p) == 0 && side_state(p) == 1 {
            let k = m + (lado as isize * 0x36) as usize;
            if (read_u8(k + 0xf5fe) as u32) > 4 && (read_u8(k + 0xf5d2) as u32) > 0x41 && (read_u8(k + 0xf5d1) as u32) > 0x37 {
                n7(call, 0x21d7, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp);
            }
        }
        return;
    }
    // 0x707840: the scorer himself, the assist or the goalkeeper
    let tp = if read_usize(p + match_player::ENGINE) != 0 { table_a(read_usize(p + match_player::ENGINE), p) } else { 0 };
    if tp & 0x30 != 0 && role_880(p) == 0 && *w(p, match_player::MATCH_RATING) > 0x1a5e && (read_u8(p + match_player::SHOTS_ON_TARGET) as u32) > 1 {
        na(call, 0x21c7, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        return;
    }
    let q = read_usize(m + engine::ASSIST_CANDIDATE);
    if p == read_usize(m + engine::PASS_RECEIVER) && q != 0 {
        let tq = || if read_usize(q + match_player::ENGINE) != 0 { table_a(read_usize(q + match_player::ENGINE), q) } else { 0 };
        let nq = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| na(call, code, (read_u8(q + match_player::COMMENTARY_ID) as u32), (read_u8(q + match_player::SIDE) as u32), 0);
        if role_880(q) != 0 && tq() & 0x38 != 0 && matches!(*w(m, engine::PASS_TYPE), 0x5e | 6 | 7 | 8) {
            if *w(q, match_player::MATCH_RATING) <= 0x1a5e {
                return;
            }
            if (read_u8(q + 3) as u32) > 1 && (read_u8(q + match_player::ASSISTS) as u32) > 1 && (read_u8(q + match_player::PASSES) as u32) > 0xa {
                nq(call, 0x21c8);
            } else if (read_u8(q + match_player::PASSES) as u32) > 0xa && (read_u8(q + 3) as u32) > 1 && read_i8(q + match_player::MOMENT_VISION) != 0 {
                nq(call, 0x21ca);
            }
            return;
        }
        if role_880(q) == 0 && tq() & 0x30 != 0 && (read_u8(q + match_player::PASSES_COMPLETED) as u32) > 0xa && *w(q, match_player::MATCH_RATING) > 0x1a5e && (read_u8(q + 3) as u32) > 1 && (read_u8(q + match_player::PASSES) as u32) > 0xa && random(2) != 0 && read_i8(q + match_player::MOMENT_VISION) != 0 {
            nq(call, 0x21c9);
        }
        return;
    }
    if c.l14 != 0 {
        return;
    }
    let ng = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| na(call, code, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0);
    if c.l20 == 0 && tg() == 1 {
        let r = (random(10) as u8).wrapping_add(5) as i8;
        if read_i8(read_usize(g + match_player::PLAYER) + 0x1c) as i32 + (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32 > r as i32 {
            let f = (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32;
            let r2 = (random(10) as u8).wrapping_add(5) as i8;
            if read_i8(read_usize(g + match_player::STAFF) + 0x57) as i32 + f > r2 as i32 && random(f) != 0 {
                let r3 = (random(10) as u8).wrapping_add(1) as i8;
                if read_i8(p + match_player::FRUSTRATION) as i32 + (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32 > r3 as i32 {
                    ng(call, 0x21d3);
                    return;
                }
            }
        }
    }
    if c.l20 != 0 || tg() != 1 {
        return;
    }
    if read_i8(read_usize(g + match_player::PLAYER) + 0x1c) >= 0xa || read_i8(read_usize(g + match_player::STAFF) + 0x57) as i32 >= (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32 + 0xa {
        return;
    }
    let f = (read_u8(g + match_player::GOALS_CONCEDED) as u32) as i32;
    let r = random(5);
    if read_i8(read_usize(g + match_player::STAFF) + 0x5a) as i32 >= r + f * 2 + 4 || f <= 1 || random(f) == 0 {
        return;
    }
    let r2 = (random(5) as u8).wrapping_add(1) as i8;
    if read_i8(p + match_player::FRUSTRATION) >= r2 {
        return;
    }
    ng(call, 0x21d4);
}

/// Retake (modes 5 and 6): restores engine `engine::MOMENTUM_B`/`engine::MOMENTUM_A` and the shooter's cell and +0x39, optionally undoes the
/// goal, narrates 0x20f7 and shoots again in mode 1.
///
/// [pt] Repetir o lance (modos 5 e 6): o motor volta aos +0x4786/+0x4782 de antes, narra 0x20f7, o
/// chutador volta a casa e ao +0x39 de antes, domina e chuta de novo no modo 1 (0x704a57 e iguais).
pub(crate) unsafe fn retake_shot_with(c: &ShotLocals, apply: bool, desfazer: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m) = (c.p, c.m);
    *((m + engine::MOMENTUM_B) as *mut i32) = c.l60;
    *((m + engine::MOMENTUM_A) as *mut i32) = c.l5c;
    if apply {
        apply_to_team(m, p, f32::from_bits(0x3fb9999a));
    }
    narrate_packed7(call, m, 0x20f7, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, c.side as u8 as u32, 0xff, 0xff, 0);
    *((p + match_player::SHOT_QUALITY) as *mut i16) = c.l58;
    if desfazer {
        undo_goal_stats(p);
    }
    call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, p, [c.l50 as u8 as u32, c.l54 as u8 as u32, 0, 0, 0]);
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, p, [0; 5]);
    call(FN_MATCH_PLAYER_TAKE_SHOT, p, [c.a1, c.v0, c.sound, 1, 0]);
}

/// Decrements the goal counters of player p (+0x3d..+0x45 words and the +0xc byte) when a goal is disallowed or
/// retaken.
///
/// [pt] Desfaz as contas do gol (+0x3d..+0x45 e +0xc) quando ele e anulado ou repetido.
pub(crate) unsafe fn undo_goal_stats(p: usize) {
    for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
        let w = (p + o) as *mut i16;
        *w = (*w).wrapping_sub(1);
    }
    let b = (p + 0xc) as *mut u8;
    *b = (*b).wrapping_sub(1);
}

/// Narrates the teammates in the shot's pressure list (0x2154, 0x2155 or 0x2132) by a random draw, then their
/// highlights; returns false if the draw skips it.
///
/// [pt] Os companheiros na lista da pressao (a5 = quantos, a6 a lista): com sorteio(n x n + k), 1 narra
/// 0x2154 com o primeiro, 2 (1 em 2) 0x2155 com os dois, senao 0x2132; depois os destaques deles.
/// Com `assistencia`, a de quem marcava (0x21c6) entra como na 0x704d4b.
pub(crate) unsafe fn narrate_pressure_teammates_with(c: &ShotLocals, k: i32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> bool {
    let (p, m, lado) = (c.p, c.m, c.side);
    let n = c.count as i32;
    if random(n * n + k) == 0 {
        return false;
    }
    let zero = (lado == 0) as u32;
    let reiniciar = || core::ptr::write_unaligned((c.list + 0xa) as *mut u32, core::ptr::read_unaligned((c.list + 2) as *const u32));
    if n == 1 {
        reiniciar();
        let r1 = next_list_item(c.list) as usize;
        narrate_packed7(call, m, 0x2154, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, 0);
        narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R);
    } else if n == 2 && random(2) != 0 {
        reiniciar();
        let r1 = next_list_item(c.list) as usize;
        let r2 = next_list_item(c.list) as usize;
        narrate_packed7(call, m, 0x2155, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, (read_u8(r2 + match_player::COMMENTARY_ID) as u32), zero, 0);
        if !narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R) {
            narrate_first_highlight(call, m, r2, zero, &HIGHLIGHTS_R);
        }
    } else {
        narrate_packed7(call, m, 0x2132, (read_u8(p + match_player::COMMENTARY_ID) as u32), lado as u8 as u32, 0xff, 0xff, 0);
    }
    true
}

/// True when the 0x21c6 "assist against the marker" commentary applies: the marker (0x6d5a10) equals teammate r and the
/// shooter's goals, shots, minute and condition fit.
///
/// [pt] A assistencia contra quem marcava (0x21c6): o marcador (0x6d5a10) e o companheiro da lista, no
/// jogo (modo 1, nao 0x33/0x35), sem gols nem chutes demais do artilheiro, nos minutos e folegos.
pub(crate) unsafe fn is_assist_against_marker(c: &ShotLocals, r: usize) -> bool {
    let (p, m) = (c.p, c.m);
    let mk = who_marks(m, p);
    mk != 0
        && c.kind != 0x33
        && c.kind != 0x35
        && c.mode == 1
        && read_i8(p + match_player::GOALS) == 0
        && read_i8(p + match_player::ASSISTS) == 0
        && read_i8(p + 3) == 0
        && (read_i8(p + match_player::SHOTS) as u8) < 5
        && (read_i8(p + match_player::SHOTS_ON_TARGET) as u8) < 2
        && c.l16 > (random(0x14) as u8).wrapping_add(0x41) as i8
        && read_i16(p + match_player::MATCH_RATING) < 0x1c52
        && read_i16(mk + match_player::MATCH_RATING) > 0x1a5e
        && (read_i8(p + match_player::FRUSTRATION) < 1 || random(read_i8(p + match_player::FRUSTRATION) as i32) == 0)
        && r == mk
}

/// The goalkeeper's save (part of `shoot`): by ratio he holds, tips for a corner or parries; narration, keeper gains
/// and the ball.
///
/// [pt] A defesa do goleiro (0x708e84 ao fim): pela razao, ele segura (ate 51), espalma para escanteio
/// (ate 61) ou rebate; as narracoes, os ganhos dele e a bola.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn goalkeeper_save_with(mut c: ShotLocals, razao: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrate_packed7(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let ng = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrate_packed7(chamar, m, codigo, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0xff, 0xff, 0);
    let modo = c.mode;
    let classe = if razao as f64 > read_f32(rdata::F32_61) as f64 {
        1
    } else if razao as f64 > read_f32(rdata::F32_51) as f64 {
        2
    } else {
        3
    };
    crate::comparison::branch(match classe {
        1 => "shot defesa rebatida",
        2 => "shot defesa corner",
        _ => "shot defesa segura",
    });
    if classe == 3 {
        *b(g, match_player::BUSY) = (*b(g, match_player::BUSY)).wrapping_add(random(3) as u8);
        *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_add(if c.kind != 0x33 { 200 } else { 1300 });
    }
    // the start of the commentary: set piece, penalties or free kick
    let mut a_2122 = false;
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        if modo == 3 {
            a_2122 = true;
        } else {
            na(chamar, 0x2174, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
    } else if c.kind == 0x33 {
        if modo == 1 {
            na(chamar, 0x1fce, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
        a_2122 = true;
    }
    if a_2122 {
        c.l15 = 1;
        ng(chamar, 0x2122);
    } else {
        let tipo_1a = c.kind == 0x1a && (classe != 3 || c.l20 == 0);
        if c.l1a {
            ng(chamar, if classe == 3 { 0x2112 } else { 0x2111 });
            narrate_first_highlight(chamar, m, g, (read_u8(g + match_player::SIDE) as u32), &HIGHLIGHTS_R);
        } else if c.l19 {
            ng(chamar, if classe == 3 && tipo_1a { 0x21d0 } else if classe == 3 { 0x2110 } else { 0x210f });
        } else {
            match classe {
                1 => ng(chamar, 0x1fe6),
                2 => ng(chamar, if c.kind == 0x1a { 0x21d0 } else { 0x1fe4 }),
                _ => {
                    ng(chamar, if tipo_1a { 0x21d0 } else { 0x1fe5 });
                    narrate_first_highlight(chamar, m, p, lp, &HIGHLIGHTS_P);
                }
            }
        }
    }
    let alto = (*((m + engine::MOMENTUM_A) as *const i32) as f64) > read_f64(rdata::F64_1680);
    if classe == 1 {
        // rebatida (0x70909d)
        if modo == 5 || modo == 6 {
            retake_shot_with(&c, false, false, chamar);
            return;
        }
        if modo != 2 {
            *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_add(if c.kind != 0x33 { 140 } else { 1000 });
        }
        if modo != 1 && modo != 2 {
            return;
        }
        *b(g, match_player::BUSY) = (*b(g, match_player::BUSY)).wrapping_add((random(3) + 1) as u8);
        chamar(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(m + engine::BALL_X) as u32), (read_u8(m + engine::BALL_Y) as u32), ((lado == 0) as u32) << 8, ((c.kind == 0x33) as u32) << 8, 0]);
        *b(m, engine::SHOT_REBOUND) = 1;
        if c.l15 != 0 || read_i8(m + engine::RESTART_TYPE) == 0 || modo == 4 {
            return;
        }
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        } else if !c.l18 && c.l11 == 0 && c.l1b >= 0x19 && modo == 1 {
            na(chamar, 0x1f9e, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, c.l64 as i8);
        }
        return;
    }
    for codigo in [0x21a1, 0x21a2] {
        if player_highlight(p, codigo) != 0 {
            na(chamar, codigo, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
    }
    if classe == 2 {
        // for a corner (0x70950f)
        if c.l15 == 0 {
            if alto || (c.l18 && c.l11 == 0) {
                na(chamar, 0x2020, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
            }
            if !matches!(modo, 5 | 6 | 2) {
                *w(g, match_player::MATCH_RATING) = (*w(g, match_player::MATCH_RATING)).wrapping_add(if c.kind != 0x33 { 120 } else { 1000 });
            }
        }
        if modo == 4 || modo == 3 {
            return;
        }
        if modo == 5 || modo == 6 {
            retake_shot_with(&c, false, false, chamar);
            return;
        }
        corner_from_shot(&c, chamar);
        return;
    }
    // segura (0x709afb)
    if c.l15 == 0 {
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        } else {
            let limit = 0x12c + if c.l1a { 0x190 } else { 0 } + if c.l19 { 0xc8 } else { 0 };
            if *w(p, match_player::SHOT_QUALITY) as i32 > limit || c.kind == 0x3a {
                na(chamar, 0x1fdd, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, read_i8(p + match_player::GOALS));
            }
            if c.l40 < -0x87 && c.kind != 0x3a {
                narrate_pressure_teammates_with(&c, 3, chamar);
            }
        }
    }
    if modo == 5 || modo == 6 {
        retake_shot_with(&c, false, false, chamar);
        return;
    }
    chamar(FN_MATCH_PLAYER_TAKE_POSSESSION, g, [0; 5]);
}

/// Corner to the opponent after a shot: ball on the shooter's goal line (`engine::BALL_Y`), restart 3 with the shooter's side,
/// narration 0x1f75; aborts on an invalid side.
///
/// [pt] Escanteio para o adversario (0x70961e): a bola na linha de fundo do lado de quem chutou, a
/// reposicao 3 com a bola do lado dele e a narracao 0x1f75.
pub(crate) unsafe fn corner_from_shot(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.side);
    let b = |o: usize| (m + o) as *mut u8;
    match lado {
        1 => *b(engine::BALL_Y) = 0xff,
        0 => *b(engine::BALL_Y) = 0xc,
        _ => {
            // simplification: the original stops the game (match_pl.cpp); the side is always 0 or 1
            linha("motor: shoot: lado invalido no corner");
            std::process::abort();
        }
    }
    *b(engine::POSSESSION_SIDE) = *((p + match_player::SIDE) as *const u8);
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *b(engine::RESTART_TYPE) = 3;
    narrate_packed7(call, m, 0x1f75, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
}

/// Goal in a shootout or set-piece kick: +1 on the side's count (`engine::SIDE_STATS` + 0x36 x side), narration 0x2000 and, in
/// shootouts, 0x212e.
///
/// [pt] O gol nos penaltis e na cobranca (0x707db2): +1 na conta do lado (+0xf5bc), 0x2000 e, nos
/// penaltis, 0x212e.
pub(crate) unsafe fn shootout_goal_with(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.side);
    let k = (m + engine::SIDE_STATS + (lado as isize * 0x36) as usize) as *mut u8;
    *k = (*k).wrapping_add(1);
    narrate_packed7(call, m, 0x2000, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    if c.mode == 3 {
        narrate_packed7(call, m, 0x212e, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    }
}

/// Disallowed goal (0x20ed): applies the pending incident (`apply_foul`), narration, team anger factor, shooter tired
/// (0x716e10) and goal stats undone.
///
/// [pt] O gol anulado (0x20ed): o lance marcado antes (0x6bba50) e aplicado (0x6bac40). Contra o proprio
/// lado, 0x20df, talvez 0x20d9 e fator 3; senao, empatado ou um a frente, 1 em 40 0x20d9, fator
/// 1,45. Quem chutou leva o fator (0x6bb8d0) e cansa (0x716e10); as contas do gol sao desfeitas.
pub(crate) unsafe fn disallowed_goal_with(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.side);
    let nb = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| narrate_packed7(call, m, code, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
    crate::comparison::branch("shot goal disallowed");
    nb(call, 0x20ed);
    let l = side_of_move(m) as i8;
    apply_pending_foul_with(m, call);
    let factor: u32;
    if l == lado {
        nb(call, 0x20df);
        if random(c.l13 as i32 - c.l12 as i32 + 3) > 0 {
            nb(call, 0x20d9);
        }
        factor = 0x40400000;
    } else {
        if (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32) && random(0x28) == 0 {
            nb(call, 0x20d9);
        }
        factor = 0x3fb9999a;
    }
    apply_to_team(m, p, f32::from_bits(factor));
    call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, p, [0; 5]);
    if c.l17 != 0 {
        undo_goal_stats(p);
    }
}

/// After the goal commentary: possible disallowance, milestone and record commentary, the score, comeback and end state
/// (`engine::RESTART_TYPE` = 8).
///
/// [pt] Depois das narracoes do gol (0x707cc7): o gol pode ser anulado (1 em 3 a cada passo, com lance
/// marcado), as narracoes de marca e recorde, o placar, a virada e o fim (+0x8eb2 = 8).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn goal_stands_with(c: &mut ShotLocals, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let modo = c.mode;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrate_packed7(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrate_packed7(chamar, m, codigo, 0xff, lp, 0xff, 0xff, a7);
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    let anular = |c: &ShotLocals, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| -> bool {
        if random(3) != 0 && side_of_move(m) as i8 > -1 {
            disallowed_goal_with(c, chamar);
            return true;
        }
        false
    };
    if modo != 3 && modo != 4 {
        match c.l10 {
            0xe => na(chamar, 0x2175, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0),
            0xf => na(chamar, 0x2176, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0),
            _ => {}
        }
    }
    if anular(c, chamar) {
        return;
    }
    let minuto = *((m + engine::CLOCK) as *const i16) as i32 / 11;
    'final_: {
        if modo == 3 || modo == 4 {
            apply_to_side(m, (lado == 0) as i8, f32::from_bits(0x3fa00000));
            break 'final_;
        }
        if c.l11 == 0 && read_i8(p + match_player::GOALS) == 3 {
            crate::comparison::branch("shot three goals");
            na(chamar, if read_i8(p + 0x30) != 0 { 0x2011 } else { 0x1fd5 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0);
        }
        if modo == 5 || modo == 6 {
            retake_shot_with(c, true, c.l17 != 0, chamar);
            return;
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 && c.l1b >= 0x14 {
            let codigo = if c.l1b > 0x28 { 0x1fa1 } else if c.l1b > 0x19 { 0x1fa0 } else { 0x1f9f };
            na(chamar, codigo, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, c.l64 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 {
            // the top scorer’s marks
            if read_i8(p + 0x30) != 0 && read_i8(p + match_player::GOALS) == 1 {
                let codigo = if player_highlight(p, 0x21b6) != 0 { 0x21b6 } else { 0x2010 };
                na(chamar, codigo, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, read_i16(p + match_player::GOAL_COUNTS) as i8);
            } else {
                let (g3d, g3f, g41, g43, g45) = (read_i16(p + match_player::GOAL_COUNTS), read_i16(p + 0x3f), read_i16(p + 0x41), read_i16(p + 0x43), read_i16(p + 0x45));
                let redondo = |v: i16| v == 0x32 || (v >= 100 && v % 100 == 0);
                let marca = if g3d == 1 {
                    Some((0x200f, 1))
                } else if g3f == 1 {
                    Some((0x200b, 1))
                } else if g43 == 1 {
                    Some((0x2012, 1))
                } else if g3f >= 10 && g3f % 10 == 0 {
                    Some((0x200b, g3f))
                } else if redondo(g3d) {
                    Some((0x200a, g3d))
                } else if g41 != g3d && redondo(g41) {
                    Some((0x200c, g41))
                } else if g43 != g41 && g43 >= 100 && g43 % 100 == 0 {
                    Some((0x200d, g43))
                } else if g45 == 1 || (g45 >= 5 && g45 % 5 == 0) {
                    Some((0x200e, g45))
                } else {
                    None
                };
                if let Some((codigo, v)) = marca {
                    crate::comparison::branch("shot top scorer mark");
                    na(chamar, codigo, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, v as i8);
                }
            }
            narrate_first_highlight(chamar, m, p, lp, &GOAL_HIGHLIGHTS);
        }
        if anular(c, chamar) {
            return;
        }
        if read_i8(m + 0x1a4) == 0 && tg() == 1 && c.l20 != 0 {
            na(chamar, 0x1fe1, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), c.l20 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        // the goal stands (0x70847e): the comeback and the moments
        c.l12 = c.l12.wrapping_add(1);
        let tarde = (0x55..0x5a).contains(&(minuto as i16)) || minuto as i16 >= 0x73;
        let campeonato = read_i16(m + engine::PERIOD_END) == 0x3de;
        let dif = c.l12 as i32 - c.l13 as i32;
        let ao_lado = |fator: u32| apply_to_side(m, (lado == 0) as i8, f32::from_bits(fator));
        if read_i8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
            nb(chamar, 0x1fd2, 0);
            break 'final_;
        }
        if c.l4f == 0 {
            if dif == 2 && ((minuto >= 0x46 && campeonato) || minuto >= 0x6a) {
                nb(chamar, 0x1fd0, 0);
                ao_lado(0x3fc66666);
                break 'final_;
            }
            if (dif == 1 || c.l26 != 0) && ((minuto >= 0x50 && campeonato) || minuto >= 0x6e) {
                nb(chamar, 0x1fd1, 0);
                ao_lado(0x3fb9999a);
                break 'final_;
            }
            if c.l12 == c.l13 && c.l26 == 0 && minuto >= 0x46 && c.l4e == 0 {
                crate::comparison::branch("shot equaliser at the end");
                let a0 = count_active(m, (lado == 0) as i8) as i8;
                let a1 = count_active(m, lado) as i8;
                let bl: i8 = if a1 < a0 { read_i8(m + (4 + lado as isize as usize)) } else { -1 };
                let (codigo, fator) = match (bl == -1, tarde) {
                    (true, true) => (0x1fd8, 0x3fb9999a),
                    (true, false) => (0x1fd6, 0x3faccccd),
                    (false, true) => (0x1fd9, 0x3fd33333),
                    (false, false) => (0x1fd7, 0x3fb33333),
                };
                ao_lado(fator);
                nb(chamar, codigo, bl);
                break 'final_;
            }
            if dif < 0 && dif > -3 && (minuto >= 0x50 || (minuto >= 0x46 && c.l12 == 1) || (minuto >= 0x3c && c.l12 == 1 && 1 - c.l13 as i32 == -2)) {
                nb(chamar, if tarde { 0x1fdb } else { 0x1fda }, 0);
                break 'final_;
            }
        }
        if c.l26 != 0 && minuto < 0x50 {
            nb(chamar, 0x1fdc, 0);
            ao_lado(0x3fb9999a);
        }
    }
    // 0x708844: o lance marcado ainda anula
    if side_of_move(m) as i8 > -1 {
        disallowed_goal_with(c, chamar);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, 0x20d5, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0);
    }
    if modo == 1 {
        crate::comparison::branch("shot valid goal");
        apply_to_team(m, p, f32::from_bits(0x3ca3d70a));
        add_disposition(m, lado, 0x96);
        add_disposition(m, (lado == 0) as i8, -0x87);
        let k = m + engine::SCORE + (lado as isize * 0x36) as usize;
        *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        // the score commentary (0x6d4b70 stays in the original)
        let zero = (lado == 0) as u32;
        let mut l3c = 0u32;
        for (codigo, lado_c, com) in [
            (0x21a3, lp, false),
            (0x21a4, lp, false),
            (0x21a5, lp, false),
            (0x21a6, zero, false),
            (0x21a7, zero, false),
            (0x21a8, zero, false),
            (0x21a9, zero, false),
            (0x21aa, lp, false),
            (0x21ab, lp, true),
            (0x21ac, zero, true),
            (0x21ad, lp, true),
            (0x21ae, zero, true),
            (0x21af, zero, true),
        ] {
            if exe::cm3_match_engine_is_context_commentary_allowed(m, lado_c, codigo, if com { &mut l3c } else { core::ptr::null_mut() }) != 0 {
                narrate_packed7(chamar, m, codigo, 0xff, lado_c, 0xff, 0xff, if com { l3c as i8 } else { 0 });
                break;
            }
        }
        let clube_8 = || {
            let t = read_usize(m + engine::MATCH);
            let clube = Club(read_usize(t + (if lado != 0 { 0x20 } else { 0x1c })));
            let k = clube.nation().addr();
            k != 0 && read_i8(k + 0x77) == 8
        };
        let r = random(5);
        let mut feito = false;
        if c.l12 as i32 - c.l13 as i32 > r + 2 && random(2) == 0 && clube_8() {
            nb(chamar, 0x2117, 0);
            feito = true;
        }
        if !feito && (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32 || c.l26 != 0) {
            let x = random(minuto);
            if x > random(0x32) && clube_8() {
                nb(chamar, 0x2116, 0);
            }
        }
    }
    *b(m, engine::POSSESSION_SIDE) = (lado == 0) as u8;
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *b(m, engine::RESTART_TYPE) = 8;
    *b(m, engine::KICKOFF_ANNOUNCED) = 0;
}

/// The shot that missed (part of `shoot`): blocked for a corner, hit the post (ball loose) or wide (goal kick), with
/// commentary by class and context.
///
/// [pt] O chute que nao foi no alvo (0x7040cb a 0x706cb7): bloqueado (escanteio), na trave (a bola
/// espirra) ou para fora (tiro de meta); as narracoes pela classe e pelo momento.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn shot_off_target_with(c: &ShotLocals, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let modo = c.mode;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrate_packed7(chamar, m, codigo, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0xff, 0xff, a7);
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    if c.l11 == 2 {
        // the shot hit someone and went out (0x7040d6): corner
        crate::comparison::branch("shot blocked for corner");
        if c.l28 == 5 {
            na(chamar, 0x1f83, 5);
        } else {
            na(chamar, 0x1f82, c.l28);
        }
        if modo == 4 || modo == 3 {
            na(chamar, 0x2008, 0);
        } else if c.kind == 0x33 && modo == 1 {
            na(chamar, 0x1fce, 0);
        }
        if modo == 5 || modo == 6 {
            match lado {
                1 => *b(m, engine::BALL_Y) = 0xff,
                0 => *b(m, engine::BALL_Y) = 0xc,
                _ => {}
            }
            retake_shot_with(c, false, false, chamar);
            return;
        }
        corner_from_shot(c, chamar);
        return;
    }
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, 0);
        if modo == 4 {
            na(chamar, 0x2174, 0);
        }
    } else if c.kind == 0x33 {
        na(chamar, 0x1fce, 0);
    }
    let goleiro = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.kind != 0x33 && c.kind != 0x3a && c.l14 == 0 && random(0x32) == 0 && c.l20 == 0 && tg() == 1 {
            narrate_packed7(chamar, m, if c.kind == 0x1a { 0x21d2 } else { 0x20d6 }, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0xff, 0xff, 0);
        }
    };
    if c.kind == 0x33 {
        match c.l10 {
            2 | 5 | 8 | 3 | 4 | 6 | 7 | 9 => {
                na(chamar, if matches!(c.l10, 2 | 5 | 8) { 0x2126 } else { 0x2125 }, c.l28);
                narrate_first_highlight(chamar, m, p, lp, &HIGHLIGHTS_P);
            }
            0xa | 0xc => na(chamar, 0x2123, c.l28),
            0xb | 0xd => na(chamar, 0x2124, c.l28),
            _ => {}
        }
    } else {
        match c.l10 {
            2 => {
                na(chamar, 0x1fbf, 0);
                narrate_first_highlight(chamar, m, p, lp, &HIGHLIGHTS_P);
            }
            3 => {
                if c.l28 == 5 {
                    na(chamar, 0x1fc1, 5);
                } else {
                    na(chamar, 0x1fc0, c.l28);
                }
                narrate_first_highlight(chamar, m, p, lp, &HIGHLIGHTS_P);
            }
            4 => {
                na(chamar, 0x1fc2, 0);
                narrate_first_highlight(chamar, m, p, lp, &HIGHLIGHTS_P);
            }
            5 => na(chamar, 0x1fc3, 0),
            6 => na(chamar, 0x1fc4, 0),
            7 => na(chamar, 0x1fc5, 0),
            8..=0xd => {
                goleiro(chamar);
                na(chamar, 0x1fc6 + (c.l10 as u32 - 8), 0);
            }
            _ => {}
        }
    }
    if modo != 1 && modo != 2 {
        if modo == 5 || modo == 6 {
            retake_shot_with(c, false, false, chamar);
        }
        return;
    }
    match c.l10 {
        0xc | 0xd => {
            // off the post (0x704b27): the ball ricochets
            crate::comparison::branch("shot hits the post");
            if c.l18 {
                if c.l11 == 0 {
                    na(chamar, 0x2020, 0);
                }
            } else if c.l11 == 0 && c.l1b >= 0x19 && read_i16(p + match_player::SHOT_QUALITY) < 0x32 {
                na(chamar, 0x1f9e, c.l64 as i8);
            }
            if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
                narrate_packed7(chamar, m, 0x20d5, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0xff, 0xff, 0);
            }
            let l = if c.l11 != 0 { (lado == 0) as u32 } else { lp };
            chamar(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(m + engine::BALL_X) as u32), (read_u8(m + engine::BALL_Y) as u32), 2 | l << 8, 1 | ((c.kind == 0x33) as u32) << 8, 0]);
            if tg() == 1 {
                *b(g, match_player::BUSY) = (*b(g, match_player::BUSY)).wrapping_add(random(3) as u8);
            }
            *b(m, engine::SHOT_REBOUND) = 1;
        }
        2..=4 => {
            crate::comparison::branch("shot weak and wide");
            if c.kind != 0x33 {
                narrate_weak_miss_with(c, chamar);
            }
            for codigo in [0x21a1, 0x21a2] {
                if player_highlight(p, codigo) != 0 {
                    na(chamar, codigo, 0);
                }
            }
            goal_kick_from_shot(c, chamar);
        }
        _ => {
            crate::comparison::branch("shot wide");
            if c.kind != 0x33 {
                narrate_miss_with(c, chamar);
            }
            goal_kick_from_shot(c, chamar);
        }
    }
}

/// Ball out over the goal line after a shot: puts the ball on the other line and calls ball-out (`exe::cm3_match_engine_ball_out_of_play`).
///
/// [pt] A bola saiu pela linha de fundo (0x705d15 e 0x706c5d): a linha do outro lado e a bola fora.
pub(crate) unsafe fn goal_kick_from_shot(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = c.m;
    match c.side {
        1 => *((m + engine::BALL_Y) as *mut u8) = 0xff,
        0 => *((m + engine::BALL_Y) as *mut u8) = 0xc,
        _ => {}
    }
    call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [c.side as u8 as u32, 0, 0, 0, 0]);
}

/// Commentary for a weak shot wide (classes 2 to 4, 0x704c6d..0x705cb2).
///
/// [pt] As narracoes do chute fraco para fora (classes 2 a 4, 0x704c6d a 0x705cb2).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn narrate_weak_miss_with(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.side);
    let lp = lado as u8 as u32;
    let zero = (lado == 0) as u32;
    let na = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a7: i8| narrate_packed7(call, m, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0xff, 0xff, a7);
    let n_21c6 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, r: usize| narrate_packed7(call, m, 0x21c6, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(r + match_player::COMMENTARY_ID) as u32), (read_u8(r + match_player::SIDE) as u32), 0);
    let reiniciar = || core::ptr::write_unaligned((c.list + 0xa) as *mut u32, core::ptr::read_unaligned((c.list + 2) as *const u32));
    if c.l18 && c.l11 == 0 {
        let q = free_teammate(m, p);
        if q != 0 && read_i8(read_usize(p + match_player::PLAYER) + 0x3f) < 10 && {
            let r = random(6);
            read_i8(read_usize(p + match_player::STAFF) + 0x5b) < (r as u8).wrapping_add(10) as i8
        } && read_i8(read_usize(q + match_player::STAFF) + 0x5b) < 10
            && random(read_i8(read_usize(q + match_player::STAFF) + 0x5b) as i32) == 0
        {
            crate::comparison::branch("shot teammate free");
            narrate_packed7(call, m, 0x21dd, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(q + match_player::COMMENTARY_ID) as u32), (read_u8(q + match_player::SIDE) as u32), 0);
        } else {
            na(call, 0x201e, 0);
        }
        return;
    }
    if (*((m + engine::MOMENTUM_A) as *const i32) as f64) > read_f64(rdata::F64_1680) {
        if c.l40 >= -0x87 {
            if c.kind == 0x3a && c.count == 0 {
                na(call, 0x219a, 0);
            }
        } else if c.kind != 0x3a {
            let n = c.count as i32;
            if random(n * n + 4) != 0 {
                if n == 1 {
                    reiniciar();
                    let r1 = next_list_item(c.list) as usize;
                    if c.l25 != 0 {
                        if is_assist_against_marker(c, r1) {
                            n_21c6(call, r1);
                        }
                    } else {
                        narrate_packed7(call, m, 0x2154, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, 0);
                    }
                    narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R);
                } else if n == 2 {
                    if random(2) == 0 {
                        na(call, 0x2132, 0);
                    } else {
                        reiniciar();
                        let r1 = next_list_item(c.list) as usize;
                        let r2 = next_list_item(c.list) as usize;
                        if c.l25 != 0 {
                            if r2 != 0 && is_assist_against_marker(c, r2) {
                                n_21c6(call, r2);
                            }
                        } else {
                            narrate_packed7(call, m, 0x2155, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, (read_u8(r2 + match_player::COMMENTARY_ID) as u32), zero, 0);
                        }
                        if !narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R) {
                            narrate_first_highlight(call, m, r2, zero, &HIGHLIGHTS_R);
                        }
                    }
                } else {
                    na(call, 0x2132, 0);
                }
            }
        }
        na(call, 0x201f, 0);
        return;
    }
    let x39 = *((p + match_player::SHOT_QUALITY) as *const i16);
    if x39 > 0x15e {
        if read_i8(p + match_player::GOALS) == 2 && (*((m + engine::CLOCK) as *const i16) as i32 / 11) > 0x32 {
            na(call, 0x1fdf, 2);
        } else {
            na(call, 0x1fde, read_i8(p + match_player::GOALS));
            narrate_first_highlight(call, m, p, lp, &HIGHLIGHTS_P);
        }
        return;
    }
    if x39 > 0xfa {
        na(call, 0x1fde, read_i8(p + match_player::GOALS));
        if !narrate_first_highlight(call, m, p, lp, &HIGHLIGHTS_P) && c.l25 != 0 {
            reiniciar();
            let r1 = next_list_item(c.list) as usize;
            if is_assist_against_marker(c, r1) {
                n_21c6(call, r1);
            }
        }
        return;
    }
    if c.l40 >= -0x87 || c.kind == 0x3a {
        return;
    }
    let n = c.count as i32;
    if random(n * n + 4) == 0 {
        return;
    }
    if n == 1 {
        reiniciar();
        let r1 = next_list_item(c.list) as usize;
        narrate_packed7(call, m, 0x2154, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, 0);
        if !narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R) && c.l25 != 0 && is_assist_against_marker(c, r1) {
            n_21c6(call, r1);
        }
    } else if n == 2 && random(2) != 0 {
        reiniciar();
        let r1 = next_list_item(c.list) as usize;
        let r2 = next_list_item(c.list) as usize;
        narrate_packed7(call, m, 0x2155, (read_u8(r1 + match_player::COMMENTARY_ID) as u32), zero, (read_u8(r2 + match_player::COMMENTARY_ID) as u32), zero, 0);
        if !narrate_first_highlight(call, m, r1, zero, &HIGHLIGHTS_R) {
            narrate_first_highlight(call, m, r2, zero, &HIGHLIGHTS_R);
        }
    } else {
        na(call, 0x2132, 0);
    }
}

/// Commentary for a shot wide (classes 5 to 0xb, 0x705d52..0x706c5c).
///
/// [pt] As narracoes do chute para fora (classes 5 a 0xb, 0x705d52 a 0x706c5c).
pub(crate) unsafe fn narrate_miss_with(c: &ShotLocals, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, g, lado) = (c.p, c.m, c.g, c.side);
    let lp = lado as u8 as u32;
    let na = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a7: i8| narrate_packed7(call, m, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), lp, 0xff, 0xff, a7);
    let tg = || if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
    let companheiros = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.l40 < -0x87 && c.kind != 0x3a {
            narrate_pressure_teammates_with(c, 4, call);
        }
    };
    if c.l18 && c.l11 == 0 {
        na(call, 0x201e, 0);
        companheiros(call);
        return;
    }
    if (*((m + engine::MOMENTUM_A) as *const i32) as f64) > read_f64(rdata::F64_1680) {
        companheiros(call);
        na(call, 0x201f, 0);
        return;
    }
    let x39 = *((p + match_player::SHOT_QUALITY) as *const i16);
    let k3a = c.kind == 0x3a;
    if (x39 > 0x15e && !k3a) || (x39 > 0xaf && k3a) {
        na(call, if read_i8(p + match_player::GOALS) != 2 { 0x1fdd } else { 0x1fdf }, read_i8(p + match_player::GOALS));
        narrate_first_highlight(call, m, p, lp, &HIGHLIGHTS_P);
        return;
    }
    if x39 > 0xfa || k3a {
        na(call, 0x1fdd, read_i8(p + match_player::GOALS));
        narrate_first_highlight(call, m, p, lp, &HIGHLIGHTS_P);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 && !matches!(c.l10, 5 | 6 | 7) {
        narrate_packed7(call, m, 0x20d5, (read_u8(g + match_player::COMMENTARY_ID) as u32), (read_u8(g + match_player::SIDE) as u32), 0xff, 0xff, 0);
    }
    companheiros(call);
}

/// Rust-mode dispatch of the shot's recorded calls to the game (interception, ball deflection, re-shoot, move),
/// unpacking the packed arguments.
///
/// [pt] Modo Rust: as chamadas do chute vao para o jogo (a interceptacao com x | y << 8; a bola desviada
/// com dentro | sem cansar << 8; o chute de novo sem a lista).
pub(crate) unsafe fn dispatch_shot_call(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        0x6cc540 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, i8, u32, u32) -> u8 = core::mem::transmute(routine);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, a[1], a[2] as u8 as i8, a[3], a[4]) as u32
        }
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(routine);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, ((a[3] >> 8) as u8 == 0) as u8, a[4] as usize);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(routine);
            f(this, a[0], a[1], a[2], a[3], 0, 0);
            0
        }
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(routine);
            f(this, a[0], a[1]);
            0
        }
        _ => dispatch_move_calls(routine, this, a),
    }
}

/// Comparer for shoot (`shoot`): runs the original and the Rust copy against the call tape over the engine and both
/// clubs' regions and records any difference.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_shoot_tape(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32)>(IDX_SHOOT);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    if a6 != 0 {
        // the teammates list: the cursor (+0xa) moves
        reg.push((a6 as usize, 14));
    }
    let t = Fixture(*((m + engine::MATCH) as *const usize));
    for clube in [t.home_club(), t.away_club()] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(
        reg,
        || {
            o(p, a1, a2, a3, a4, a5, a6);
            0
        },
        |chamar| {
            shoot_with(p, a1, a2, a3, a4, a5, a6 as usize, &mut |rotina, this, args| match rotina {
                // the interception runs inside (its comparer does not write a unit): the plays are the units
                0x6cc540 => {
                    let mut play = |q: usize, a: [u32; 4]| chamar(FN_MATCH_PLAYER_TRY_BLOCK_SHOT, q, [a[0], a[1], a[2], a[3], 0], [u32::MAX, u32::MAX, 0xff, u32::MAX, 0]) as u8;
                    find_interception_with(this, args[0] as u8 as i8, (args[0] >> 8) as u8 as i8, args[1], args[2] as u8 as i8, args[3], args[4], &mut |lado, grupo, v, a3, a5, a6| {
                        try_side_block_with(this, lado, grupo, v, a3, 1, a5, a6, &mut play)
                    }) as u32
                }
                _ => {
                    let mask = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        0x6f46b0 => [0xff, 0xff, 0, 0, 0],
                        0x6cd490 => [0xff, 0, 0, 0, 0],
                        0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                        0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                        0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                        0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                        0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 | 0x7103e0 => [0, 0, 0, 0, 0],
                        _ => [0xff, 0, 0, 0, 0],
                    };
                    chamar(rotina, this, args, mask)
                }
            });
            0
        },
    );
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia, || format!("shoot(tipo {:#x}, modo {}) {} chamadas: {}", a1 as u8, a4 as u8, k, tape_detail()));
}

/// the engine constructor (the player, tactics and tactical line vectors through the runtime vector constructor
/// `exe::crt_vector_constructor_iterator`; the referee strictness `construct_referee`; the empty list)
/// Original: `build_engine` (0x6b7e50, match_eng.cpp, thiscall).
///
/// [pt] { 0x6b7e50 } (match_eng.cpp): o construtor do motor. Os vetores de objetos pelo construtor de
/// vetores do runtime (0x9463af): os 40 jogadores (0x1be bytes, 0x6eada0), as duas taticas (0xd0,
/// 0x5961c0) e as duas copias das linhas taticas (0x18e3, 0x5b27c0); o rigor do arbitro (0x6e9460) e
/// a lista de eventos vazia (+0x478e).
pub(crate) unsafe extern "thiscall" fn build_engine(m: usize) -> usize {
    exe::crt_vector_constructor_iterator(m + engine::PLAYERS, 0x1be, 0x28, 0x6eada0, 0x6ebd60);
    exe::crt_vector_constructor_iterator(m + engine::MATCH_MANAGERS, 0xd0, 2, 0x5961c0, 0x6dbb10);
    exe::crt_vector_constructor_iterator(m + engine::TACTICS, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    exe::crt_vector_constructor_iterator(m + engine::TACTICS_BACKUP, 0x18e3, 2, 0x5b27c0, 0x5a53c0);
    exe::new_match_official(m + engine::REFEREE);
    *((m + engine::EVENT_LIST) as *mut usize) = 0;
    m
}

/// the destructor (its own commentator, the list, the strictness and the vectors)
/// Original: `destroy_engine` (0x6b97a0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6b97a0 } (match_eng.cpp): o destrutor do motor. O narrador proprio (+0x8e9d) e a lista de
/// eventos (+0x478e) sao destruidos e liberados; depois o rigor e os vetores, na ordem inversa.
pub(crate) unsafe extern "thiscall" fn destroy_engine(m: usize) {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let narr = *((m + engine::COMMENTATOR) as *const usize);
    if *((m + engine::OWNS_COMMENTATOR) as *const u8) != 0 && narr != 0 {
        destroy(narr);
        exe::crt_operator_delete(narr);
        *((m + engine::COMMENTATOR) as *mut usize) = 0;
        *((m + engine::OWNS_COMMENTATOR) as *mut u8) = 0;
    }
    let list = *((m + engine::EVENT_LIST) as *const usize);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        *((m + engine::EVENT_LIST) as *mut usize) = 0;
    }
    destroy(m + engine::REFEREE);
    exe::crt_vector_destructor_iterator(m + engine::TACTICS_BACKUP, 0x18e3, 2, 0x5a53c0);
    exe::crt_vector_destructor_iterator(m + engine::TACTICS, 0x18e3, 2, 0x5a53c0);
    exe::crt_vector_destructor_iterator(m + engine::MATCH_MANAGERS, 0xd0, 2, 0x6dbb10);
    exe::crt_vector_destructor_iterator(m + engine::PLAYERS, 0x1be, 0x28, 0x6ebd60);
}

/// Angle in degrees (0..359) of cell (i, j) seen from the centre, for the global table 0xb67c30 used by ball
/// deflection.
///
/// [pt] O angulo (graus, 0..=359) da casa (i, j) vista do centro, para a tabela global 0xb67c30 que a
/// bola desviada usa (+0x8ed8); a original calcula com fpatan, o Rust com atan.
pub(crate) unsafe fn cell_angle_degrees(i: i32, j: i32) -> u16 {
    let graus = read_f64(rdata::F64_57_29577951);
    let mut a: i32 = if i == 0 {
        if j > 0 {
            0
        } else if j < 0 {
            0xb4
        } else {
            return 0;
        }
    } else if i > 0 && j == 0 {
        0x5a
    } else if i < 0 && j == 0 {
        0x10e
    } else if i > 0 {
        if j > 0 {
            ftol((i as f64 / j as f64).abs().atan() * graus)
        } else {
            ftol((j as f64 / i as f64).abs().atan() * graus + read_f64(rdata::F64_90))
        }
    } else if j < 0 {
        ftol((i as f64 / j as f64).abs().atan() * graus + read_f64(rdata::F64_180))
    } else {
        ftol((j as f64 / i as f64).abs().atan() * graus + read_f64(rdata::F64_270))
    };
    if a as i16 > 0x167 {
        a = 0;
    } else if (a as i16) < 0 {
        a += 0x168;
    }
    a as u16
}

/// Builds the 0x19-byte pre-match event (record, club, flags, starter, side, code) and appends it to the engine's event
/// list (`engine::EVENT_LIST`, LLIST::add_tail).
///
/// [pt] Monta e poe na lista de eventos (+0x478e, 0x68d0d0) o evento de pre-jogo de 0x19 bytes: a ficha,
/// o clube, as bandeiras, o titular, o lado (ou a vaga), o codigo e -1 nos campos livres.
pub(crate) unsafe fn push_prematch_event(m: usize, sheet: Staff, club: Club, bandeiras: u32, titular: u8, side: u8, code: u16) {
    let mut ev = [0u8; 0x19];
    ev[0..4].copy_from_slice(&*(sheet.addr() as *const [u8; 4]));
    ev[4..8].copy_from_slice(&*(club.addr() as *const [u8; 4]));
    ev[8..0xc].copy_from_slice(&bandeiras.to_le_bytes());
    ev[0xc] = titular;
    ev[0xd] = side;
    ev[0xe] = 0;
    ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
    ev[0x13..0x15].copy_from_slice(&code.to_le_bytes());
    ev[0x15..0x19].copy_from_slice(&u32::MAX.to_le_bytes());
    exe::llist_add_tail(*((m + engine::EVENT_LIST) as *const usize), ev.as_ptr() as usize, 0x19);
}

/// `0x6b7f20` (ret 0x10, match_eng.cpp, 6 KB): prepares match `jogo` in the engine; returns 0 if the match had already
/// ended.
///
/// It zeroes the state, the weather (`engine::WEATHER`: `get_match_weather`, `get_pitch_condition`, `check_postponement` in the original routine), the kits
/// (0x6d47e0), the referee (0x6e9470) and the pitch conditions (`calculate_match_attendance`, `engine::PITCH_CONDITION`), both squads (`select_side`, with the
/// error box below 11), the commentator (given or new, `construct_commentary_queue`), the strength of the sides and the marking.
///
/// The pre-match event list (`exe::new_llist`) receives the players unhappy with their contract (0x13), another club's interest
/// (0x1e), form (5, 4) and contract expiry (6).
///
/// Without `rapida`, it draws who kicks off (`engine::NEXT_KICKOFF_SIDE`/`engine::POSSESSION_SIDE`) and set piece 8. Returns 0 if the match had already ended
/// (+0x8eba).
///
/// [pt] { 0x6b7f20 } (ret 0x10, match_eng.cpp, 6 KB): prepara a partida `jogo` no motor. Zera o estado,
/// o clima (+0x4756: 0x923010, 0x923970, 0x923630 na original), os uniformes (0x6d47e0), o arbitro
/// (0x6e9470) e as condicoes do gramado (0x85cba0, +0x91d2), os dois elencos (0x6db500, com a caixa de
/// erro abaixo de 11), o narrador (dado ou novo, 0x6d5bf0), a forca dos lados e a marcacao; a lista
/// de eventos de pre-jogo (0x68d020) recebe os jogadores insatisfeitos com o contrato (0x13), o
/// interesse de outro clube (0x1e), a forma (5, 4) e o fim de contrato (6). Sem `rapida`, sorteia
/// quem comeca (+0x8eb6/+0x8eae) e a bola parada 8. Devolve 0 se a partida acabou antes (+0x8eba).
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn prepare_match_with(m: usize, game: usize, fast: u8, commentator: usize, ao_vivo: u8) -> u8 {
    let b = |o: usize| (m + o) as *mut u8;
    let w = |o: usize| (m + o) as *mut u16;
    let dw = |o: usize| (m + o) as *mut u32;
    let clear = |de: usize, ate: usize| core::ptr::write_bytes((m + de) as *mut u8, 0, ate - de);
    *((m + engine::MATCH) as *mut usize) = game;
    *b(engine::SHOOTOUT_STATE) = 0;
    *b(engine::PUNISHMENT_STATE) = 0;
    clear(0x21e, 0x21e + 8000);
    *b(engine::MATCH_RUNNING) = 0;
    *b(engine::PENDING_INCIDENT_COUNT) = 0;
    *b(0x1cc) = 0;
    *b(engine::SIDE_SENT_OFF) = 0;
    *w(engine::SIDE_REPUTATION) = *((read_usize(game + fixture::HOME_CLUB) + 0x80) as *const u16);
    *w(0x1d8) = *((read_usize(game + fixture::AWAY_CLUB) + 0x80) as *const u16);
    *b(engine::SIDE_OUT_OF_POSITION) = 0;
    *b(0xb0) = 0;
    *dw(engine::FURTHEST_FORWARD) = 0;
    *dw(engine::FURTHEST_FORWARD_SIDE1) = 0;
    *w(engine::LAST_SHOT_MINUTE) = 0xffff;
    *b(engine::LAST_SHOT_SIDE) = 0;
    *w(engine::PASS_TYPE) = 0xffff;
    let time = (game + fixture::SQUAD_RULES) as *mut u16;
    if *((game + fixture::SQUAD_RULES) as *const u8) & 0xf > 9 {
        *time = (*time & 0xfff9) | 9;
    }
    if (*((game + fixture::SQUAD_RULES) as *const u8) & 0xf0) as u32 > 0x90 {
        *time = (*time & 0xff9f) | 0x90;
    }
    let clima = *(G_WEATHER as *const usize);
    exe::weather_get_match_weather(clima, m + engine::WEATHER, game);
    *b(1) = exe::weather_get_pitch_condition(clima, m + engine::WEATHER, game);
    let v = exe::weather_check_postponement(clima, game);
    *time = ((((v & 3) as i8 as i16) << 12) as u16) | (*time & 0xcfff);
    if *time & 0x2000 == 0 {
        *time &= 0xcfff;
        *b(engine::ABANDON_MINUTE) = 0xff;
    } else {
        *b(engine::ABANDON_MINUTE) = if random(2) != 0 { (random(0x19) as u8).wrapping_add(0xf) } else { (random(0x1e) as u8).wrapping_add(0x32) };
        if matches!(*b(1), 0 | 2 | 4) {
            *time &= 0xcfff;
            *b(engine::ABANDON_MINUTE) = 0xff;
        }
    }
    *dw(engine::FORMATION_TABLE) = (m + engine::FORMATION_TABLE_DATA) as u32;
    *dw(engine::FORMATION_TABLE_SIDE1) = (m + engine::FORMATION_TABLE_DATA_SIDE1) as u32;
    *dw(engine::POSITION_TABLE) = (m + engine::POSITION_TABLE_DATA) as u32;
    *dw(engine::POSITION_TABLE_SIDE1) = (m + engine::POSITION_TABLE_DATA_SIDE1) as u32;
    *dw(engine::ARROW_TABLE) = (m + engine::ARROW_TABLE_DATA) as u32;
    *dw(engine::ARROW_TABLE_SIDE1) = (m + engine::ARROW_TABLE_DATA_SIDE1) as u32;
    if *(G_MATCH_ZONE_TABLE_INIT_PENDING as *const u8) != 0 {
        for i in -8..9i32 {
            for j in -11..12i32 {
                *((0xb67c30 + ((i * 24 + j) * 2) as isize as usize) as *mut u16) = cell_angle_degrees(i, j);
            }
        }
        *(G_MATCH_ZONE_TABLE_INIT_PENDING as *mut u8) = 0;
    }
    *w(engine::INJURY_COUNT) = 0;
    *b(0x8eba) = 0;
    *w(engine::CLOCK) = 0;
    *b(engine::STOPPAGE_TIME) = 0;
    *b(engine::SUBSTITUTION_PENDING) = 0;
    *w(engine::PERIOD_END) = 0x1ef;
    *w(engine::SUB_TICK) = 0;
    *b(engine::GOLDEN_GOAL_SCORED) = 0;
    *b(0x1a9) = 0;
    *b(0x1aa) = 0;
    *w(engine::LAST_SUBSTITUTION_TICK) = 0;
    *w(0x8ece) = 0;
    *b(4) = 0xb;
    *b(5) = 0xb;
    *w(engine::LAST_STANCE_CALL_TICK) = 0x2710;
    *w(0x1d4) = 0x2710;
    *dw(engine::ADJUSTED_PLAYER_ID) = u32::MAX;
    for o in [0x1a4, 0x1cd, 8, 9, 6, 7, 0x1a5, 0x1a6] {
        *b(o) = 0;
    }
    *dw(engine::MOMENTUM_B) = 0;
    *dw(engine::MOMENTUM_A) = 0;
    *b(engine::SIDE_STANCE) = 0xff;
    *b(0x1ac) = 0xff;
    *dw(engine::STANCE_FACTOR) = 0xc47a0000;
    *dw(0x476e) = 0xc47a0000;
    *b(engine::BALL_TRANSIT_TICKS) = 0;
    *b(engine::TRANSIT_TARGET_X) = 0;
    *b(engine::TRANSIT_TARGET_Y) = 0;
    *dw(engine::BALL_RECEIVER) = 0;
    *b(engine::SET_PIECE_KIND) = 0;
    *b(engine::SHOT_PENDING) = 0;
    *b(engine::STOPPAGE_ANNOUNCED) = 0;
    *b(0x8d49) = 0;
    *w(engine::PENDING_PLAY_CODE) = 0xffff;
    *dw(engine::PENDING_PLAY_OFFENDER) = 0;
    *dw(engine::PENDING_PLAY_VICTIM) = 0;
    *b(engine::PENDING_PLAY_SERIOUS) = 0;
    *b(engine::PENDING_PLAY_KIND) = 0;
    for o in 0x475b..=0x475e {
        *b(o) = 0xff;
    }
    *dw(engine::FOULED_PLAYER) = 0;
    *b(2) = 0xff;
    *b(3) = 0xff;
    clear(0x8f02, 0x8f7a);
    clear(0x1f6, 0x21e);
    clear(0xa, 0xaf);
    clear(0x8d51, 0x8e9b);
    *b(engine::POSSESSION_WINDOW_INDEX) = 0;
    *b(engine::LAST_POSSESSION_SIDE) = 0xff;
    *w(engine::BALL_DIRECTION) = 0xb4;
    *b(engine::BALL_ROLL) = 0;
    *dw(engine::BALL_HOLDER) = 0;
    *b(engine::BALL_HEIGHT) = 0;
    for o in [0xf576, 0xf57a, 0xf57e, 0xf5ae, 0x91de, 0xf586, 0xf58a, 0xf5a6, 0xf5aa, 0xf58e, 0xf592] {
        *dw(o) = 0;
    }
    clear(0x215e, 0x215e + 0x97e * 4);
    *dw(engine::GOALKEEPER) = 0;
    *dw(engine::GOALKEEPER_SIDE1) = 0;
    for l in 0..2usize {
        let e = 0xf5bd + 0x36 * l;
        clear(e - 1, e + 0x10);
        clear(e + 0x11, e + 0x35);
        *b(0x8d46 + l) = 0;
    }
    *dw(engine::TEMPO_ROLL_A) = 0x40c00000;
    *dw(engine::TEMPO_ROLL_B) = 0x40c00000;
    *dw(engine::TEMPO_GOALS_ROLL) = 0x40a00000;
    *b(0) = fast;
    if *(G_MATCH_ENGINE_ENABLED as *const u8) == 0 {
        *b(0) = 1;
        *b(engine::OWNS_COMMENTATOR) = 0;
    }
    *b(engine::LIVE_MATCH) = ao_vivo;
    let (casa, fora) = (read_usize(game + fixture::HOME_CLUB), read_usize(game + fixture::AWAY_CLUB));
    *b(engine::SIDE_KIT) = kit(casa, fora, 1);
    *b(0x8d50) = kit(fora, casa, 0);
    let x2e = *((game + fixture::REFEREE) as *const i16);
    let referee = if *(G_OFFICIALS_MANAGER as *const u32) != 0 && *(game as *const i32) != -2 && x2e > -1 {
        (*(G_OFFICIAL_TABLE as *const u32)).wrapping_add((x2e as i32 * 0x2b) as u32) as usize
    } else {
        0
    };
    init_strictness(m + engine::REFEREE, m as u32, referee);
    let comp = read_usize(game + fixture::COMPETITION);
    if *(game as *const i32) != -2 && *(comp as *const u32) != *(KEY_CLUB_COMP_RESERVE as *const u32) {
        *dw(engine::ATTENDANCE) = exe::stadium_calculate_match_attendance(*(G_STADIUM as *const usize), game, m + engine::PITCH_CONDITION);
    } else {
        *dw(engine::ATTENDANCE) = 0;
        *dw(engine::PITCH_CONDITION) = 0;
        *dw(0x91d6) = 0;
        *dw(0x91da) = 0;
    }
    if *dw(engine::PITCH_CONDITION) == 0 {
        *dw(engine::PITCH_CONDITION) = 100;
        *dw(0x91d6) = 0x5a;
        *dw(0x91da) = 0xa;
    }
    if *(game as *const i32) == -2 {
        *w(0x1ad) = 0;
    } else {
        // pitch wear (+0x1ad) and how much it weighs (+0x1ae)
        let (g2, g6, ga) = (*dw(engine::PITCH_CONDITION) as i32 as f64, *dw(0x91d6) as i32 as f64, *dw(0x91da) as i32 as f64);
        let x = g6 * read_f64(rdata::F64_100) / g2;
        let t = (read_f64(rdata::F64_100) - x) as f32;
        let a = (ga * read_f64(rdata::F64_0_0001) + read_f64(rdata::F64_5)) - x * read_f64(rdata::F64_0_3333333333);
        let a = if !(a > read_f64(rdata::F64_10)) && a < read_f64(rdata::F64_0) { read_f64(rdata::F64_0) } else if a > read_f64(rdata::F64_10) { read_f64(rdata::F64_10) } else { a };
        *b(0x1ad) = ftol(a) as u8;
        let stadium = read_usize(casa + club::STADIUM);
        let base = if stadium == read_usize(game + fixture::STADIUM) || stadium == read_usize(fora + club::STADIUM) { read_f32(rdata::F32_1) } else { read_f32(rdata::F32_0) } as f64;
        let c = (g6 * read_f64(rdata::F64_0_0001) + read_f64(rdata::F64_5)) - t as f64 * read_f64(rdata::F64_0_3333333333);
        let cf = c as f32 as f64;
        let smaller = if c > read_f64(rdata::F64_10) { read_f64(rdata::F64_10) } else { cf };
        let r = if base > smaller { base } else if cf > read_f64(rdata::F64_10) { read_f64(rdata::F64_10) } else { cf };
        *b(0x1ae) = ftol(r) as u8;
    }
    // the squads
    let (mut n_casa, mut n_fora) = (0i8, 0i8);
    let pouco = |n: i8, row: u32, text: usize| {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let join: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let mut data = [0u8; 8];
        let mut quando = [0u8; 200];
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        let j = *((m + engine::MATCH) as *const usize);
        exe::new_date((data.as_mut_ptr()) as *mut u8 as usize, *((j + fixture::DAY) as *const u16) as u32, *((j + fixture::YEAR) as *const u16) as u32);
        exe::date_get_short_weekday_date_string(data.as_mut_ptr(), quando.as_mut_ptr(), 200, 0xff);
        let id = *(j as *const i32);
        let comp = if id == -2 { 0xa158dc } else if id == -1 { 0xa158ec } else { read_usize(j + fixture::COMPETITION) + 0x38 };
        format(0xde1f64, text, n as i32, read_usize(j + fixture::HOME_CLUB) + 0x38, read_usize(j + fixture::AWAY_CLUB) + 0x38, comp, quando.as_ptr());
        exe::crt_strtok(0xa15884, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let s = join(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
        format(0xae24d0, 0x9870e8, s);
        exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    };
    let nc = exe::match_man_init(m + engine::MATCH_MANAGERS, m, read_usize(casa + club::MANAGER), casa + club::SQUAD, m + engine::PLAYERS, 0, casa, fora, game, &mut n_casa);
    if nc < 0xb {
        pouco(nc, 0x264, 0xa158b0);
    }
    let nf = exe::match_man_init(m + engine::MATCH_MANAGERS_SIDE1, m, read_usize(fora + club::MANAGER), fora + club::SQUAD, m + engine::PLAYERS_SIDE1, 1, fora, casa, game, &mut n_fora);
    if nf < 0xb {
        pouco(nf, 0x28c, 0xa15858);
    }
    if (n_fora as i32 + n_casa as i32) < 5 && ao_vivo == 0 {
        *b(0) = 1;
    }
    core::ptr::copy_nonoverlapping((m + engine::TACTICS) as *const u8, (m + engine::TACTICS_BACKUP) as *mut u8, 0xc71 * 4 + 2);
    *w(engine::SIDE_STRENGTH) = 0x6e;
    *w(0x1d0) = 0x6e;
    *dw(engine::COMMENTATOR) = 0;
    if *b(0x8eba) != 0 {
        *b(0) = 1;
        *b(engine::OWNS_COMMENTATOR) = 0;
        return 0;
    }
    // o narrador
    if commentator != 0 {
        *dw(engine::COMMENTATOR) = commentator as u32;
        *b(engine::OWNS_COMMENTATOR) = 0;
    } else {
        let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let p = new(0x2db1);
        *dw(engine::COMMENTATOR) = if p != 0 { exe::new_match_events(p, *b(engine::LIVE_MATCH)) as u32 } else { 0 };
        *b(engine::OWNS_COMMENTATOR) = 1;
        *(*dw(engine::COMMENTATOR) as usize as *mut u8) = 1;
    }
    side_strength(m, 0);
    side_strength(m, 1);
    individual_marking(m);
    *b(engine::MATCH_RUNNING) = 1;
    let slots = *(G_TACTICS as *const usize);
    let titular = |l: usize, k: i8| m + (if l == 0 { 0x4796 } else { 0x6a6e }) + (k as isize * 0x1be) as usize;
    for (l, o_clube, o_marca, o_destino) in [(0usize, 0x1c, 0x40, 0xf5cd), (1, 0x20, 0x41, 0xf603)] {
        let k = marked_slot(slots, read_usize(game + o_clube), (*((game + o_marca) as *const u8) == 0) as u32, 0);
        *b(o_destino) = if titular(l, k) == 0 {
            0xff
        } else {
            let k = marked_slot(slots, read_usize(game + o_clube), (*((game + o_marca) as *const u8) == 0) as u32, 0);
            *(titular(l, k) as *const u8)
        };
    }
    // the list of pre-match events
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    *dw(engine::EVENT_LIST) = list as u32;
    'eventos: {
        if list == 0 || is_national_team(casa) != 0 {
            break 'eventos;
        }
        if *b(0) != 0 {
            return 1;
        }
        let comp = read_usize(game + fixture::COMPETITION);
        if *((game + fixture::HOME_SIDE_MARK) as *const u8) == 0 || comp == 0 {
            break 'eventos;
        }
        let amistosos = *(G_FRIENDLY as *const usize);
        if amistosos != 0 && is_friendly(amistosos, comp) != 0 {
            break 'eventos;
        }
        add_prematch_player_events(m, Fixture(game));
    }
    if *b(0) == 0 {
        if random(2) != 0 {
            *b(engine::NEXT_KICKOFF_SIDE) = 1;
            *b(engine::POSSESSION_SIDE) = 0;
        } else {
            *b(engine::NEXT_KICKOFF_SIDE) = 0;
            *b(engine::POSSESSION_SIDE) = 1;
        }
        *b(engine::RESTART_TYPE) = 8;
        *b(engine::KICKOFF_ANNOUNCED) = 0;
    }
    1
}

/// The pre-match events of each player on the pitch (0x6b8d9b to 0x6b9745).
///
/// The contract (`exe::contract_manager_get_contract_for_club`) and the position marks (`staff_get_natural_position_mask` against `read_1bc`/`exe::match_player_get_secondary_position`) give contract unhappiness (0x13,
/// with points from the player sheet); his reputation against the club's, the other club's interest (0x1e); the form
/// from the average (`get_stat`) and the index of [0xb74350], 5 or 4; the contract ending soon (`exe::date_days_since`), 6.
///
/// [pt] Os eventos de pre-jogo de cada jogador em campo (0x6b8d9b a 0x6b9745): o contrato (0x4cf5f0) e as
/// marcas da posicao (0x5481a0 contra 0x717400/0x717450) dao a insatisfacao (0x13, com pontos pela
/// ficha); a reputacao dele contra a do clube, o interesse (0x1e); a forma pela media (0x7c1e30) e
/// pelo indice de [0xb74350], 5 ou 4; o fim de contrato perto (0x54cdb0), 6.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn add_prematch_player_events(m: usize, game: Fixture) {
    let contratos = *(G_CONTRACT_MANAGER as *const usize);
    let average: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let slots = *(G_TACTICS as *const usize);
    let index = || {
        let t = *(G_PLAYER_STATS as *const usize);
        let d = *((t + 0x18) as *const f64);
        if d == read_f64(rdata::F64_0) { read_f64(rdata::F64_6_8) } else { *((t + 0x10) as *const f64) / d }
    };
    for lado in 0..2i8 {
        let titular_da_vaga = |j: usize| {
            let (c, marca) = if lado == 0 { (game.home_club(), game.home_side_mark()) } else { (game.away_club(), (game.away_side_mark() as u8)) };
            let k = marked_slot(slots, c, (marca == 0) as u32, 0);
            (j == m + engine::PLAYERS + ((k as isize + lado as isize * 20) * 0x1be) as usize) as u8
        };
        for k in 0..20usize {
            let j = side_player_slot(m, lado, k);
            if read_i8(j + match_player::POSITION) < 0 {
                continue;
            }
            let f = Staff(read_usize(j + match_player::STAFF));
            let club = Club(if lado != 0 { game.away_club() } else { game.home_club() });
            if is_national_team(club.addr()) != 0 || club.nation().addr() == 0 {
                break;
            }
            if f.player() == 0 {
                continue;
            }
            let c = contract_with_club(contratos, f.addr(), club.addr());
            if c == 0 {
                continue;
            }
            let a = exe::staff_get_natural_position_mask(f) as u16;
            let bb = read_1bc(j);
            let d = player_table_b(j);
            let flags = contract_flags(c);
            let marked = flags & 0x40 != 0 || *((c + 0x45) as *const u32) & 0x2000000 != 0;
            if marked && a != 0 && ((d & a) == d || (bb & a) != bb) {
                crate::comparison::branch("prematch unhappy");
                push_prematch_event(m, f, club, 0, 0, k as u8, 0x13);
            } else if (f.job_for_club() as i8) != 0xc && a != 0 && (d & a) != d && (bb & a) != bb {
                let s: i16 = if a == 1 {
                    6
                } else if (d | 4) != 0 && read_i8(f.player() + 0x11) < 10 && read_i8(f.player() + 0x12) < 10 {
                    2
                } else if (d | 4) != 0 && (a | 4) != 0 {
                    -1
                } else if (d | 0x10) != 0 && (a | 0x10) != 0 {
                    0
                } else if (d | 0x40) != 0 && (a | 0x40) != 0 {
                    0
                } else {
                    1
                };
                let w = read_i16((f.player()) + 0xb);
                let f5b = f.professionalism();
                let f57 = f.ambition();
                if w > 0xcb2
                    && (w > 0x1d4c || (w.wrapping_add((f57 as i16).wrapping_add(s.wrapping_mul(20)).wrapping_mul(25))) > club.reputation())
                    && f5b < 0xf
                    && f57 > 5
                    && (f.temperament() as i32 + f5b as i32) < 0x19
                {
                    let mut p: i16 = match s {
                        6 => 0x19,
                        2 => 5,
                        0 => -3,
                        _ => 0,
                    };
                    p += if f5b == 1 { 0x14 } else if f5b < 6 { 0xa } else { 5 };
                    if (a | 4) != 0 {
                        p -= 5;
                    }
                    if f57 == 1 {
                        p -= 3;
                    } else if f57 < 6 {
                        p -= 1;
                    }
                    if w > 0x186a {
                        p += if f57 > 0xf && f5b < 6 { 5 } else { 3 };
                    }
                    let f5a = f.pressure();
                    if f5a > 0xa && (f.year_of_birth() as i8) < 0x18 && f57 > 0xf {
                        p += 5;
                    }
                    if f5a == 1 {
                        p -= 0xa;
                    } else if f5a < 6 {
                        p -= 5;
                    }
                    if *((c + 0x45) as *const u32) != 0 {
                        p += 5;
                    } else if contract_flags(c) != 0 {
                        p += 1;
                    } else {
                        p -= 3;
                    }
                    if contract_flags(c) & 0x40 != 0 {
                        p += 5;
                    }
                    if likes_club(f.addr(), club.addr()) != 0 {
                        p -= 3;
                    }
                    if club.manager().addr() != 0 && likes(f.addr(), club.manager().addr()) != 0 {
                        p -= 5;
                    }
                    if p > 5 && (p as i32) > random(0x1e) {
                        crate::comparison::branch("prematch unhappy with position");
                        push_prematch_event(m, f, club, 0x40, 0, k as u8, 0x13);
                    }
                }
            }
            // another club’s interest (0x6b917a)
            let pe = read_usize(j + match_player::PLAYER);
            let r = random(7);
            if (read_i8(pe + player_sheet::LEADERSHIP) as i32) > r + 6 && read_i16(pe + player_sheet::CURRENT_REPUTATION) > 0xea6 && f.professionalism() < 7 && (f.professionalism() as i32) < random(0xf) {
                let x = random(read_i16((read_usize(j + match_player::PLAYER)) + 0xb) as i32);
                if random(club.reputation() as i32) < x && *((c + 0x45) as *const u32) & 0x2f7a0b != 0 {
                    crate::comparison::branch("prematch interest");
                    push_prematch_event(m, f, club, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 0x1e);
                }
            }
            // a forma
            let sheet_61 = f.player();
            let mut form = (f.year_of_birth() as i8);
            if sheet_61 != 0 {
                let x0f = read_i8(sheet_61 + player_sheet::GOALKEEPER);
                if x0f == 0x14 {
                    form = form.wrapping_sub(2);
                } else if read_i8(sheet_61 + player_sheet::DEFENDER) == 0x14 {
                    form = form.wrapping_sub(1);
                } else if read_i8(sheet_61 + player_sheet::ATTACKER) == 0x14 || read_i8(sheet_61 + player_sheet::ATTACKING_MIDFIELDER) == 0x14 {
                    form = form.wrapping_add(1);
                }
                let x3c = read_i8(sheet_61 + player_sheet::STAMINA);
                if x0f != 0x14 && x3c > 0xf {
                    form = form.wrapping_sub(2);
                } else if x3c > 0xa {
                    form = form.wrapping_sub(1);
                } else if x0f != 0x14 && x3c < 6 {
                    form = form.wrapping_add(1);
                }
            }
            let sheet_average = *(G_PLAYER_STATS as *const usize);
            if form < 0x17 {
                let m1 = average(sheet_average, sheet_61, 6, 1, 6.5) as f32;
                let m2 = average(sheet_average, f.player(), 6, 0x11, 6.5) as f32;
                let mut al = f.professionalism().wrapping_add(f.pressure()).wrapping_add((f.year_of_birth() as i8)).wrapping_sub(0x1e);
                if al < 4 {
                    al = 4;
                } else if form < 0x12 && al > 0x19 {
                    al = 0x19;
                    let x3c = read_i8(f.player() + 0x3c);
                    if x3c < 0xa {
                        al = 0x19 - x3c;
                    }
                }
                let f61 = f.player();
                if f61 != 0
                    && (f.int_apps() as i8) == 0
                    && read_i16(f61 + player_sheet::CURRENT_REPUTATION) < 0x1b58
                    && club.reputation() > 0xcb2
                    && (al as f64) < m1 as f64
                    && game.home_side_mark() != 0
                    && random(al as i32) == 0
                    && (m2 as f64) < index()
                {
                    crate::comparison::branch("prematch form down");
                    push_prematch_event(m, f, club, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 5);
                }
            } else if form > 0x20 {
                let m1 = average(sheet_average, sheet_61, 6, 1, 6.5) as f32;
                let m2 = average(sheet_average, f.player(), 6, 0x11, 6.5);
                if m1 as f64 > read_f32(rdata::F32_2) as f64 {
                    let mut z = m1 as f64 * read_f64(rdata::F64_0_1);
                    if !(z < read_f64(rdata::F64_1)) {
                        z = read_f64(rdata::F64_1);
                    }
                    if m2 < index() - read_f64(rdata::F64_1) + z {
                        crate::comparison::branch("prematch form up");
                        push_prematch_event(m, f, club, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 4);
                    }
                }
            }
            // the end of the contract
            if random(f.temperament() as i32) == 0 && read_i8(f.player() + 0x25) > 0xf {
                let d = exe::date_days_since(G_CURRENT_DATE, (f.date_joined_club() as u32), (f.date_joined_club_leap_year() as u32));
                let r = random(200) + 0x32;
                if d > r && form > 0x17 {
                    let g = read_i8(c + 0x3f);
                    if (g as i32) > random(4) + 2 {
                        crate::comparison::branch("prematch end of contract");
                        push_prematch_event(m, f, club, *((c + 0x45) as *const u32), titular_da_vaga(j), lado as u8, 6);
                    }
                }
            }
        }
    }
}

/// clears the state, the weather (`get_match_weather`/`get_pitch_condition`/`check_postponement`), the kits, the referee, the pitch (`calculate_match_attendance`,
/// +0x1ad/+0x1ae), the squads (`select_side`), the commentator (given or new), the side strength, the marking, the angle
/// table and the pre-match event list (unhappiness 0x13, interest 0x1e, form 5/4, contract end 6); draws who kicks off
/// Original: `prepare_match` (0x6b7f20, match_eng.cpp (6 KB), thiscall, ret 0x10).
///
/// [pt] { 0x6b7f20 } (ret 0x10, match_eng.cpp): ver `prepare_match_with`.
pub(crate) unsafe extern "thiscall" fn prepare_match(m: usize, game: usize, fast: u32, commentator: usize, ao_vivo: u32) -> u8 {
    prepare_match_with(m, game, fast as u8, commentator, ao_vivo as u8)
}

// simplification: the creation, the preparation and the end of the engine have no comparison of their own (they allocate the
// commentator and the list, which the tape would have to reproduce); validated by the career (Rust x original),
// like the engine loop. The routines they call are still compared.

/// 0x6cda20 (ret 8, match_eng.cpp) with the calls that touch things outside the engine injected (`redo_team` and `exe::match_man_make_substitutions`
/// of the staff, 0x897760 writes the captain): `j` leaves the pitch (injury or sending-off).
///
/// Without `ja_fora`, adds to +0x23, writes the minute (`engine::CLOCK` / 11, at most the end of the period: 45, 90, 105, 120)
/// in +0x21 and removes him from the grid (`exe::cm3_match_engine_remove_player_from_pitch` with -3). He ends up with +0x19 = -3. In matches with a human
/// manager (the side's `engine::SIDE0_MANAGER`, `exe::human_manager_is_on_holiday`/`exe::human_manager_controls_reserve_team`), it only continues with `engine::SIDE0_MANAGER_TACTIC_SLOT` and outside national teams. With
/// +0x1a3 it stops.
///
/// Goalkeeper (table 1) with the side's `engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT`: the staff swap the goalkeeper (`exe::match_man_make_substitutions` with 1). Otherwise the staff
/// rebuild the team (`redo_team` and `exe::match_man_make_substitutions` with 0); if he was the captain (`exe::tactics_find_captain_slot`), the best rated (`best_rated`) becomes
/// captain (0x897760); it zeroes the side's position-1 player and recomposes it (`exe::cm3_match_engine_get_goalkeeper`).
///
/// Outside matches with a human manager it stores the substitution minute (`engine::LAST_SUBSTITUTION_TICK` + 2 x side). At the end it rebuilds
/// the side's strength (`exe::cm3_match_engine_recalc_side_score`) and the individual marking (`individual_marking`).
///
/// [pt] { 0x6cda20 } (ret 8, match_eng.cpp) com as chamadas que mexem fora do motor injetadas (0x6e1aa0
/// e 0x6e1090 da comissao, 0x897760 grava o capitao): `j` sai de campo (lesao ou expulsao). Sem
/// `ja_fora`, soma +0x23, grava o minuto (+0x8ed0 / 11, no maximo o fim do periodo: 45, 90, 105, 120)
/// em +0x21 e tira da grade (0x6cdd50 com -3). Fica com +0x19 = -3. Nos jogos com tecnico humano
/// (+0x904d do lado, 0x5fd210/0x5fd480), so segue com +0x904b e fora de selecao. Com +0x1a3 para.
/// Goleiro (tabela 1) com +0x9049 do lado: a comissao troca o goleiro (0x6e1090 com 1). Senao a
/// comissao refaz o time (0x6e1aa0 e 0x6e1090 com 0); se ele era o capitao (0x897590), o mais bem
/// avaliado (0x6e1a40) vira capitao (0x897760); zera o jogador de posicao 1 do lado e o recompoe
/// (0x6ce110). Fora dos jogos com tecnico humano guarda o minuto da troca (+0x8ecc + 2 x lado). No
/// fim refaz a forca do lado (0x6bbb60) e a marcacao individual (0x6bbf60).
pub(crate) unsafe fn player_left_pitch_with(m: usize, j: usize, ja_away: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |o: usize| (j + o) as *mut i8;
    if ja_away == 0 {
        *b(match_player::RED_CARDS) = (*b(match_player::RED_CARDS)).wrapping_add(1);
        let min = (*((m + engine::CLOCK) as *const i16) as i32 / 11) as i8;
        *b(match_player::MINUTE_LEFT_PITCH) = min;
        let ceiling = match *((m + engine::PERIOD_END) as *const i16) {
            0x1ef => Some(0x2d),
            0x3de => Some(0x5a),
            0x483 => Some(0x69),
            0x528 => Some(0x78),
            _ => None,
        };
        if let Some(t) = ceiling {
            *b(match_player::MINUTE_LEFT_PITCH) = min.min(t);
        }
        exe::cm3_match_engine_remove_player_from_pitch(m, j, -3);
    }
    let goalkeeper = exe::match_player_get_position(j) == 1;
    *b(match_player::POSITION) = -3;
    let side = read_i8(j + match_player::SIDE);
    let base = (m as isize + side as isize * 0xd0) as usize;
    let t = *((m + engine::MATCH) as *const usize);
    let club_of = |l: i8| if l == 0 { *((t + fixture::HOME_CLUB) as *const usize) } else { *((t + fixture::AWAY_CLUB) as *const usize) };
    if read_i8(m + 0) == 0 {
        let p = Staff(*((base + engine::SIDE0_MANAGER) as *const usize));
        let n = *(G_STAFF_COUNT as *const i32);
        if !p.is_null() && *(p.addr() as *const i32) >= n - 0x10 && *(p.addr() as *const i32) < n && (p.classification() as i8) != 0 && exe::human_manager_is_on_holiday(*(G_HUMAN_MANAGER as *const usize), p) == 0 {
            if read_i8(base + engine::SIDE0_MANAGER_TACTIC_SLOT) == 0 || exe::database_is_national_team(club_of(side)) != 0 || exe::human_manager_controls_reserve_team(*(G_HUMAN_MANAGER as *const usize), p) != 0 {
                return;
            }
        }
    }
    if read_i8(m + engine::PENDING_INCIDENT_COUNT) != 0 {
        return;
    }
    let staff = base + engine::MATCH_MANAGERS;
    let mut store = read_i8(m + 0) == 0;
    if read_i8(base + engine::SIDE0_MANAGER_SUBSTITUTIONS_LEFT) != 0 && goalkeeper {
        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [1, 0, 0, 0, 0]);
    } else if read_i8(m + 0) != 0 {
        store = false;
    } else {
        call(FN_MATCH_MAN_REASSIGN_POSITIONS, staff, [0, 0, 0, 0, 0]);
        call(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, staff, [0, 0, 0, 0, 0]);
        let l = read_i8(j + match_player::SIDE);
        let main = if l != 0 { read_i8(t + fixture::AWAY_SIDE_MARK) } else { read_i8(t + fixture::HOME_SIDE_MARK) } == 0;
        let v = exe::tactics_find_captain_slot(*(G_TACTICS as *const usize), club_of(l), main as u32, 0);
        if j == side_player_slot(m, l, v as isize as usize) {
            let l = read_i8(j + match_player::SIDE);
            let main = if l != 0 { read_i8(t + fixture::AWAY_SIDE_MARK) } else { read_i8(t + fixture::HOME_SIDE_MARK) } == 0;
            let k = exe::match_man_find_captain_slot(m + engine::MATCH_MANAGERS + l as isize as usize * 0xd0);
            call(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, *(G_TACTICS as *const usize), [club_of(l) as u32, k as u8 as u32, main as u32, 0, 0]);
        }
        *((m + if read_i8(j + match_player::SIDE) == 0 { 0xf59e } else { 0xf5a2 }) as *mut usize) = 0;
        exe::cm3_match_engine_get_goalkeeper(m, read_i8(j + match_player::SIDE));
    }
    if store {
        let l = read_i8(j + match_player::SIDE) as isize;
        *((m as isize + l * 2 + 0x8ecc) as *mut i16) = *((m + engine::CLOCK) as *const i16);
    }
    exe::cm3_match_engine_recalc_side_score(m, read_i8(j + match_player::SIDE));
    exe::cm3_match_engine_assign_marking_targets(m);
}

/// Hook entry for 0x6cda20 (player_left_pitch_with): player `j` leaves the pitch (injury or sending-off): records the
/// minute, removes him from the grid and, unless a human manager handles it, the staff replace the goalkeeper or
/// rebuild the team and pick a new captain; finally recomputes the side's strength and man-marking. `ja_fora` skips the
/// minute and grid steps.
///
/// [pt] { 0x6cda20 } (ret 8, match_eng.cpp): ver `player_left_pitch_with`.
pub(crate) unsafe extern "thiscall" fn player_left(m: usize, j: usize, ja_away: u32) {
    player_left_pitch_with(m, j, ja_away as u8, &mut |rotina, this, a| {
        match rotina {
            0x6e1aa0 => {
                let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_MAN_REASSIGN_POSITIONS);
                f(this);
            }
            0x6e1090 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_MAN_MAKE_SUBSTITUTIONS);
                f(this, a[0]);
            }
            _ => {
                let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
            }
        }
        0
    })
}

// Ports of 0x6e1aa0 and 0x6e1090 (the staff redo the team and make the substitution, match_man.cpp): units of the
// fita ate serem portadas.

/// Codigo do lance -> multiplicador do risco de quem tem a bola (tabelas 0x6d08b0/0x6d0884).
pub(crate) const RISK_BY_MOVE: [u8; 0x2f] = [0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 1, 10, 10, 10, 10, 10, 10, 2, 10, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 2, 4, 10, 10, 10, 10, 10, 10, 5, 6, 7, 8, 2, 2, 9];

/// `0x6cfe10` (ret 0x14, match_eng.cpp, ~2.6 KB) with the calls that change the game injected (`exe::injury_get_player_injury` current injury,
/// which first updates it; `injure` applies the injury): a tackle between `a` and `b` (incidents `tipo_a`/`tipo_b`) may
/// injure either.
///
/// Each has a chance (random(random(102)), above 50 redone by random(random(76)) in 2/3) and a risk of 4 (6 if already
/// injured, with the type's severity `exe::injury_get_type_category` and the chance doubled in 24/25), adjusted by stamina (10000 / max(1000,
/// +0x4d/2 + 5000)), by the player sheet (+0x2d, always `a`'s deciding the branch) and by the injury grade (`exe::injury_get_recurring_injury_level`).
///
/// `a` without stamina (< 4000) is injured on the spot (−1). With `direto`, only `a` is at risk: random(50000) below 10
/// × risk injures by severity, or by random(11).
///
/// Otherwise each one is at risk: whoever has the ball adds risk from the other's incident (the other's +0xd9 × table
/// factor), incident 5 divides by +0x79; an injured player picks the type by severity (with the other player, 1..=9;
/// without, 1..=5) or at random, and incident 5 has its own types (6, 9, 10).
///
/// [pt] { 0x6cfe10 } (ret 0x14, match_eng.cpp, ~2,6 KB) com as chamadas que mexem no jogo injetadas
/// (0x62ed70 lesao atual, que antes a atualiza; 0x6d0980 aplica a lesao): a dividida entre `a` e
/// `b` (lances `tipo_a`/`tipo_b`) pode machucar. Cada um tem uma chance (sorteio(sorteio(102)),
/// acima de 50 refeita por sorteio(sorteio(76)) em 2/3) e um risco 4 (6 ja machucado, com a
/// gravidade 0x62ec00 do tipo e a chance dobrada em 24/25), ajustado pelo folego (10000 /
/// max(1000, +0x4d/2 + 5000)), pela ficha (+0x2d, sempre a de `a` decidindo o ramo) e pelo grau
/// da lesao (0x62ece0). `a` sem folego (< 4000) se machuca na hora (-1). Com `direto`, so `a`
/// arrisca: sorteio(50000) abaixo de 10 x risco machuca pela gravidade, ou por sorteio(11).
/// Senao cada um arrisca: quem tem a bola soma o risco pelo lance do outro (+0xd9 do outro x
/// fator da tabela), o lance 5 divide por +0x79; machucado escolhe o tipo pela gravidade (com o
/// outro jogador, 1..=9; sem, 1..=5) ou por sorteio, e o lance 5 tem tipos proprios (6, 9, 10).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn tackle_injury_with(m: usize, a: usize, tipo_a: i16, b: usize, kind_b: i16, direct: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let injuries = *(G_INJURY as *const usize);
    let f4 = |a: usize| read_f32(a) as f64;
    let d8 = |a: usize| read_f64(a);
    let mut chance = [0i8; 2];
    let mut grav = [-1i8; 2];
    let mut f = [4.0f32; 2];
    for c in chance.iter_mut() {
        *c = random(random(0x66)) as i8;
        if *c > 0x32 && random(3) != 0 {
            *c = random(random(0x4c)) as i8;
        }
    }
    for (k, p) in [(0usize, a), (1, b)] {
        if p == 0 {
            continue;
        }
        let person = *((p + match_player::STAFF) as *const usize);
        let e = call(FN_INJURY_GET_PLAYER_INJURY, injuries, [person as u32, 0, 0, 0, 0]) as usize;
        if e != 0 {
            f[k] = 6.0;
            grav[k] = exe::injury_get_type_category(injuries, *(e as *const i8)) as i8;
            if random(0x19) != 0 {
                chance[k] = (chance[k] as u8).wrapping_shl(1) as i8;
            }
        }
        let mut x = (read_f32(p + match_player::STAMINA) as f64) * f4(0x968f18) + f4(0x968db0);
        if !(x > f4(0x96d2dc)) {
            x = f4(0x96d2dc);
        }
        let q = f4(0x96d1b0) / x * f[k] as f64;
        // the branch follows `a`’s record (as in the original), the value a of the player himself
        let branch = read_i8(*((a + match_player::PLAYER) as *const usize) + 0x2d);
        let val = read_i8(*((p + match_player::PLAYER) as *const usize) + 0x2d) as i32;
        let v = if branch > 10 { val as f64 * q * f4(0x96afc4) } else { ((val - 10) as f64 * f4(0x96afc4) + f4(0x968e70)) * q };
        f[k] = v as f32;
        if exe::injury_get_recurring_injury_level(injuries, person) != 0 {
            let mut t = exe::injury_get_recurring_injury_level(injuries, person) as i8 as f64 * d8(0x968968) + d8(0x968940);
            if t > d8(0x968930) {
                t = f4(0x969584);
            }
            f[k] = (t * f[k] as f64) as f32;
        }
    }
    let mut injure = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, kind: u32, chance: u32, p: usize, o: usize| {
        c(0x6d0980, m, [kind, chance, p as u32, o as u32, 0]);
    };
    if a != 0 && (read_f32(a + match_player::STAMINA) as f64) < f4(0x96d40c) {
        injure(call, u32::MAX, 0, a, 0);
        return;
    }
    if direct != 0 && a != 0 {
        if random(50000) >= ftol(f[0] as f64 * f4(0x968a78)) {
            return;
        }
        let kind = if (1..=5).contains(&grav[0]) {
            grav[0] as u32
        } else {
            match random(11) {
                0..=2 => 1,
                3..=5 => 3,
                6 | 7 => 4,
                8 | 9 => 2,
                _ => 5,
            }
        };
        let word = chance[0] as u8 as u32 | (chance[1] as u8 as u32) << 8 | (grav[0] as u8 as u32) << 16 | (grav[1] as u8 as u32) << 24;
        injure(call, kind, word, a, 0);
        return;
    }
    let jog = [a, b];
    let code = [tipo_a, kind_b];
    for i in 0..2 {
        let p = jog[i];
        if p == 0 {
            continue;
        }
        let o = jog[1 - i];
        if p == *((m + engine::BALL_HOLDER) as *const usize) && o != 0 {
            let c = code[1 - i];
            let d9 = (read_f32(o + 0xd9) as f64);
            let r = if (1..=0x2f).contains(&c) { RISK_BY_MOVE[c as usize - 1] } else { 10 };
            match r {
                4 => {
                    f[i] = (d9 * d8(0x968fd0) * f[i] as f64) as f32;
                    chance[i] = ftol(chance[i] as f64 * d8(0x9689c0)) as i8;
                }
                8 | 9 => {
                    let k = if r == 8 { d8(0x96afd8) } else { d8(0x96d450) };
                    f[i] = (d9 * k * f[i] as f64) as f32;
                    chance[i] = (chance[i] as u8).wrapping_shl(1) as i8;
                }
                _ => {
                    let k = [0x96d440, 0x96d430, 0x967880, 0x96d438, 0, 0x968fd0, 0x968a98, 0x96d448, 0, 0, 0x968918][r as usize];
                    f[i] = (d9 * d8(k) * f[i] as f64) as f32;
                }
            }
        }
        if code[i] == 5 {
            let r79 = (read_f32(p + match_player::AGILITY) as f64);
            let x = if r79 > d8(0x968950) { r79 } else { d8(0x968950) };
            f[i] = (f4(0x968a78) / x * f[i] as f64) as f32;
        }
        if random(50000) >= ftol(f[i] as f64 * f4(0x968a78)) {
            continue;
        }
        let ch = chance[i] as u8 as u32;
        let g = grav[i];
        if code[i] == 5 {
            if matches!(g, 6 | 9 | 10) {
                injure(call, g as u32, ch, p, 0);
                return;
            }
            let t = match random(5) {
                0..=2 => 9,
                3 => 6,
                _ => 10,
            };
            injure(call, t, ch, p, 0);
        } else if read_i8(m + engine::BALL_HEIGHT) == 4 && o != 0 {
            injure(call, 0xb, ch, p, o);
        } else if o == 0 {
            if (1..=5).contains(&g) {
                injure(call, g as u32, ch, p, 0);
                return;
            }
            match random(11) {
                1..=3 => injure(call, 3, ch, p, 0),
                4 | 5 => injure(call, 4, ch, p, 0),
                6 | 7 => {
                    injure(call, 2, ch, p, 0);
                    injure(call, 5, ch, p, 0);
                }
                8 => injure(call, 5, ch, p, 0),
                _ => injure(call, 1, ch, p, 0),
            }
        } else {
            if (1..=9).contains(&g) {
                injure(call, g as u32, ch, p, o);
                return;
            }
            let t = random(9) + 1;
            injure(call, t as u32, ch, p, o);
        }
    }
}

/// Decides whether players `a` and/or `b` get injured in a tackle (risk by move type, stamina, injury proneness; with
/// `direto` only `a` risks it) and injures them; thin wrapper over tackle_injury_with with the real calls.
///
/// [pt] { 0x6cfe10 } (ret 0x14, match_eng.cpp): ver `tackle_injury_with`.
pub(crate) unsafe extern "thiscall" fn tackle_injury(m: usize, a: usize, kind: u32, b: usize, kind_b: u32, direct: u32) {
    tackle_injury_with(m, a, kind as u16 as i16, b, kind_b as u16 as i16, direct as u8, &mut |rotina, this, x| match rotina {
        0x62ed70 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
            f(this, x[0] as usize) as u32
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_INJURE_PLAYER);
            f(this, x[0], x[1], x[2] as usize, x[3] as usize);
            0
        }
    })
}

// Ports of 0x62ed70 (current injury: updates the record through 0x62c680) and 0x6d0980 (applies the injury,
// allocating): tape units.
pub(crate) static mut GET_PLAYER_INJURY_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_62ED70: Prologue = Prologue { address: 0x62ed70, bytes: &[0x81, 0xec, 0x00, 0x02, 0x00, 0x00] };

/// Gate for Injury::get_player_injury (`exe::injury_get_player_injury`): records it on the comparison tape when recording, else just calls it.
pub(crate) unsafe extern "thiscall" fn gate_get_player_injury(l: usize, person: usize) -> usize {
    let o: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(GET_PLAYER_INJURY_TRAMPOLINE);
    if !is_recording() {
        return o(l, person);
    }
    call_recording(FN_INJURY_GET_PLAYER_INJURY, l, [person as u32, 0, 0, 0, 0], || o(l, person) as u32) as usize
}

/// Chooses which of the club's three kits (colour pairs +0x83/+0x87, +0x8b/+0x8f, +0x93/+0x97) to wear against `outro`:
/// the first whose second colour does not clash with the opponent's colour (or any kit when `qualquer`). Returns 1, 2
/// or 3; 1 if a pair has a zero colour or none fits, 2 if the 2nd or 3rd kit is missing; 0 (with an error) for a
/// missing club.
///
/// [pt] { 0x6d47e0 } (cdecl, match_eng.cpp): qual dos tres uniformes do `clube` (pares de cores +0x83/+0x87,
/// +0x8b/+0x8f, +0x93/+0x97) usar contra o `outro`. Sem clube ou sem outro, registra o erro e devolve 0.
/// Para cada par, a 0x48ce70 (que devolve 0) decide se troca as cores (0x48ce90). Sem `qualquer`, o
/// par vale se a segunda cor nao choca (0x540490) com a do outro (+0x87 dele, ou a da tabela 0xae23ec
/// pelo indice 0x9cf17c); devolve 1, 2 ou 3 conforme o par, 1 se o par tiver cor zero, 2 se faltar
/// o 2o ou o 3o uniforme, 1 se nenhum servir.
pub(crate) unsafe extern "C" fn kit(club: usize, other: usize, any: u32) -> u8 {
    let other = Club(other);
    let club = Club(club);
    if club.is_null() || other.is_null() {
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        return 0;
    }
    let any = any as u8 != 0;
    let colour_other = if any {
        0
    } else {
        let c = other.back_colour1();
        if c != 0 { c } else { *(G_COLOUR_TABLE as *const usize) + *(KEY_COLOUR_WHITE as *const i32) as usize * 58 }
    };
    let (mut x, mut y) = (0usize, 0usize);
    let mut k = 0u8;
    loop {
        match k {
            0 => (x, y) = (club.fore_colour1(), club.back_colour1()),
            1 | 2 => {
                (x, y) = if k == 1 { (club.fore_colour2(), club.back_colour2()) } else { (club.fore_colour3(), club.back_colour3()) };
                if x == 0 || y == 0 {
                    return 2;
                }
            }
            _ => return 1,
        }
        if exe::shared_return_zero(x, y) != 0 {
            exe::shared_return_one(&mut x, &mut y);
        }
        k += 1;
        if any || (y != 0 && x != 0 && exe::database_colours_clash(y, colour_other) == 0) {
            break;
        }
    }
    if x == 0 || y == 0 { 1 } else { k }
}

/// { `exe::update_player_reputation_after_match` } (cdecl, match_eng.cpp, ~3.9 KB): after a match, develops the three ratings of the person's player sheet
/// (+0x61: +0x9, +0xb, +0xd, up to 10000) from his match rating `nota`.
///
/// The `nivel` comes from the competition (`comp`: −2 of byte 5; with +0x40, the average of the club reputation/500 and
/// the match's +0x69; otherwise +0x69, at least 15 for national teams 0x76d940; up to 20). Each rating converges
/// towards rating + (nivel/2 + 5) × deviation × 50, with the deviation of `nota` from the league average (0xb74350;
/// squared, keeping the sign, when below) and divisors by the rating's age (/1000 + 16) minus the level; with `forte`,
/// the value 250 × nivel × nota / average acts as a floor. Floors A/B/C rise with reputation (+0x80 of `rep`) and
/// rating in big national team or cup games (codes +0x30/+0x32 of the competition).
///
/// The change in rating +0xb gives the morale adjustment (`exe::match_player_adjust_morale` on the match player, or `staff_adjust_morale` on the person) by
/// random draw, up to ±20. Then the ratings are clamped to the band of the potential (+0x5) and of age (250/300/375/500
/// × (2 − rating/10000)), and +0xb pulls the club reputation (`exe::contract_manager_get_contract_for_club`, age +0x18 up to 19, CRT power 3). With `jm`
/// (the match player) the new values are also written to its +0x47/+0x49/+0x4b.
///
/// [pt] { 0x6d26b0 } (cdecl, match_eng.cpp, ~3,9 KB): depois do jogo, as tres notas da ficha da pessoa
/// (+0x61: +0x9, +0xb, +0xd, ate 10000) andam pela `nota` do jogo. O `nivel` vem da competicao
/// (`comp`: -2 da 5; com +0x40 a media da reputacao do clube/500 e do +0x69 do jogo; senao o +0x69,
/// 15 no minimo nas selecoes 0x76d940; ate 20). Cada nota converge para a nota + (nivel/2 + 5) x
/// desvio x 50, com o desvio da `nota` contra a media do campeonato (0xb74350; ao quadrado com o
/// sinal quando abaixo) e divisores pela idade da nota (/1000 + 16) menos o nivel; com `forte`, o
/// valor 250 x nivel x nota / media entra como piso. Pisos A/B/C sobem com reputacao (+0x80 do
/// `rep`) e nota em jogos grandes da selecao ou de copas (codigos +0x30/+0x32 da competicao). A
/// diferenca na nota +0xb da o ajuste do moral (0x7172d0 no jogador do jogo, ou 0x544710 na
/// pessoa) por sorteio, ate +-20. Depois as notas ficam presas a faixa do potencial (+0x5) e da
/// idade (250/300/375/500 x (2 - nota/10000)), e a +0xb puxa a reputacao do clube (0x4cf5f0,
/// idade +0x18 ate 19, potencia 3 da CRT). Com `jm` grava em +0x47/+0x49/+0x4b dele.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn develop_player(jm: usize, person: usize, club: usize, rep: usize, comp: usize, rating: f32, average_b: u32, strong: u32) {
    let comp = Fixture(comp);
    let person = Staff(person);
    let rep = Club(rep);
    let club = Club(club);
    let d8 = |a: usize| read_f64(a);
    let f4 = |a: usize| read_f32(a) as f64;
    let sheet = || person.player();
    let of_game = Competition(comp.competition());
    let injury_word_0c = || rep.reputation() as i32;
    // the competition level
    let mut level: i8 = if *(comp.addr() as *const i32) == -2 {
        5
    } else {
        let n = if (comp.home_side_mark() as i8) != 0 {
            Some(((injury_word_0c() / 500 + of_game.reputation() as i32) / 2) as i8)
        } else if *(G_NATIONAL_TEAMS as *const usize) != 0 && exe::national_teams_is_finals_comp(*(G_NATIONAL_TEAMS as *const usize), of_game) != 0 {
            let v = of_game.reputation();
            if v <= 0xf { None } else { Some(v as i8) }
        } else {
            Some((of_game.reputation() as i8))
        };
        match n {
            None => 15,
            Some(n) => n.min(20),
        }
    };
    let mut factor = rating;
    if jm != 0 && !((rating as f64) < d8(0x968f10)) {
        if read_i8(jm + match_player::GOALS) == 0 {
            factor = (rating as f64 * f4(0x96d48c)) as f32;
        }
        if read_i8(jm + match_player::ASSISTS) == 0 {
            factor = (factor as f64 * f4(0x96d488)) as f32;
        }
        if read_i8(jm + match_player::PENALTIES_TAKEN) != read_i8(jm + match_player::PENALTIES_SCORED) {
            factor = (factor as f64 * f4(0x96d488)) as f32;
        }
        if read_i8(jm + match_player::RED_CARDS) != 0 {
            factor = (factor as f64 * f4(0x96d48c)) as f32;
        }
    }
    // the ”age” of each rating: /1000 + 16, with a minimum
    let age = |o: usize, min: i8| ((read_i16((sheet()) + o) as i32 / 1000) as i8).wrapping_add(0x10).max(min);
    let nivel0 = level as i16;
    let div_b = (age(0xb, 0x16) as i16).wrapping_sub(nivel0);
    let mut grande = false;
    let ja_national_team = exe::database_is_national_team(club) != 0;
    let div_9 = if ja_national_team
        || (*(comp.addr() as *const i32) == -2 && (club.nation().addr() as i32) != 0 && (club.nation().addr() as i32) == (person.nation() as i32))
        || (!of_game.is_null() && (of_game.nation() as i32) == (person.nation() as i32))
        || (club.nation().addr() as i32) == (person.nation() as i32)
    {
        grande = true;
        (age(9, 0x16) as i16).wrapping_sub(nivel0)
    } else {
        let v = (((0xca - nivel0 as i32 * 10) as i16 as i32 + (age(9, 0x16) as i16).wrapping_sub(nivel0) as i32) / 2) as i16;
        v.max(4)
    };
    // floors: (A, B, C) for +0xb, +0x9, +0xd
    let (mut pa, mut pb, mut pc) = (1i32, 1i32, 1i32);
    let floor = |p: f64, flag: bool| -> (i32, i32, i32) {
        let a = ftol(if p < d8(0x96d470) { p } else { d8(0x96d470) });
        let pk = p * d8(0x96a670);
        let c = ftol(if pk < d8(0x96d468) { pk } else { d8(0x96d468) });
        (a, if flag { a } else { c }, c)
    };
    let div_d: i16;
    if exe::database_is_national_team(club) != 0 {
        div_d = (age(0xd, 0x15) as i16).wrapping_sub(nivel0);
        if (comp.category() as i8) == 0xa
            && *(G_NATIONAL_TEAMS as *const usize) != 0
            && exe::national_teams_is_finals_comp(*(G_NATIONAL_TEAMS as *const usize), of_game) != 0
            && of_game.reputation() >= 0x11
            && !((factor as f64) < d8(0x968988))
            && injury_word_0c() > 0x157c
        {
            let c32 = comp.round_prefix();
            let c30 = comp.round_name();
            if (0x41f..=0x426).contains(&c32) || c30 == 0x8c {
                (pa, pb, pc) = floor((injury_word_0c() - 0x1194) as f64 * (factor as f64 - d8(0x968988)), grande);
            } else {
                let lim = match c30 {
                    0x14 => Some((0x1194, 0x96af90, 0x9690b8)),
                    0x78 => Some((0xfa0, 0x96af90, 0x96d480)),
                    0x82 => Some((0xfa0, 0x96af90, 0x968dc8)),
                    0x96 => Some((0xfa0, 0x969000, 0x96d478)),
                    _ => None,
                };
                if let Some((menos, k, teto)) = lim {
                    let p = (injury_word_0c() - menos) as f64 * (factor as f64 - d8(k));
                    let v = ftol(if p < d8(teto) { p } else { d8(teto) });
                    (pa, pb, pc) = (v, v, v);
                }
            }
        }
    } else if (comp.category() as i8) != 8 {
        div_d = (0xca - nivel0 as i32 * 10) as i16;
    } else {
        let c32 = comp.round_prefix();
        let c30 = comp.round_name();
        if (0x43d..=0x448).contains(&c32) {
            div_d = (age(0xd, 0x15) as i16).wrapping_sub(nivel0).wrapping_add(1);
            if !((factor as f64) < d8(0x968f10)) && injury_word_0c() > 0x157c {
                let p = (injury_word_0c() - 0x157c) as f64 * (factor as f64 - d8(0x968f10));
                pa = ftol(p * d8(0x96afd8));
                let c = ftol(p * d8(0x968a80));
                pb = if grande { pa } else { c };
                pc = c;
            }
        } else if matches!(c30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
            let id = age(0xd, 0x15);
            level = (of_game.reputation() as i8);
            div_d = (id as i16).wrapping_sub(level as i16);
            if !((factor as f64) < d8(0x968eb0)) && injury_word_0c() > 0x157c {
                (pa, pb, pc) = floor((injury_word_0c() - 0x1194) as f64 * (factor as f64 - d8(0x968988)), grande);
            }
        } else {
            div_d = (0xca - nivel0 as i32 * 10) as i16;
        }
    }
    // the rating’s deviation from the league average
    let average = || {
        let g = *(G_PLAYER_STATS as *const usize);
        let (soma, n) = if average_b as u8 != 0 { (0x20, 0x28) } else { (0x10, 0x18) };
        if *((g + n) as *const f64) == d8(0x9689a0) { d8(0x968998) } else { *((g + soma) as *const f64) / *((g + n) as *const f64) }
    };
    let mut desvio = rating as f64 - average();
    if desvio < f4(0x968970) {
        desvio *= desvio.abs();
    }
    let st = sheet();
    let (v9, vb, vd) = (read_i16(st + player_sheet::HOME_REPUTATION) as i32, read_i16(st + player_sheet::CURRENT_REPUTATION) as i32, read_i16(st + player_sheet::WORLD_REPUTATION) as i32);
    let step_towards = (level as i32 as f64 * d8(0x968968) + d8(0x9688a0)) * desvio * d8(0x968ff8);
    let mut n9 = ftol(v9 as f64 + step_towards);
    let mut nb = ftol(vb as f64 + step_towards);
    let mut nd = ftol(vd as f64 + step_towards);
    if strong as u8 != 0 {
        let mut c = ftol((level as i32 * 250) as f64 * rating as f64 / average());
        if desvio < f4(0x968970) {
            c = ftol(c as i16 as f64 * rating as f64 / average());
        }
        let c16 = c as i16;
        if c16 > nb as i16 {
            nb = c;
        }
        if c16 > n9 as i16 {
            n9 = c;
        }
        if c16 > nd as i16 {
            nd = c;
        }
    }
    let new_b = ((div_b as i32 - 1) * vb + nb as i16 as i32) / div_b as i32;
    let new_9 = ((div_9 as i32 - 1) * v9 + n9 as i16 as i32) / div_9 as i32;
    let new_d = ((div_d as i32 - 1) * vd + nd as i16 as i32) / div_d as i32;
    // morale by the difference in rating +0xb
    let base = if vb as i16 > 1 { vb } else { 1 };
    let dif = new_b as i16 as i32 - vb;
    let x = ftol(dif as f64 * d8(0x96a6b0) / base as f64 + (rating as f64 + d8(0x967890)) * d8(0x968968)) as i16;
    let mut m: i16 = if x > 0 {
        (random(3) + (x as i32).min(dif)) as i16
    } else if x == 0 {
        0
    } else {
        let v = x as f64 * d8(0x968940);
        let t = if v > dif as f64 { v } else { dif as f64 };
        ftol(t - random(3) as f64) as i16
    };
    if m < 0 {
        let r = random(person.determination() as i32) + 1;
        m = ftol(m as f32 as f64 / (r as f64 * d8(0x9689e0))) as i16;
    }
    let delta = random(m.clamp(-20, 20) as i32);
    if jm != 0 {
        exe::match_player_adjust_morale(jm, delta as i8);
    } else {
        exe::staff_adjust_morale(person, (delta as i8) as u32, 1);
    }
    // the potential and age bands
    let st = sheet();
    let (cb, c9, cd) = (read_i16(st + player_sheet::CURRENT_REPUTATION), read_i16(st + player_sheet::HOME_REPUTATION), read_i16(st + player_sheet::WORLD_REPUTATION));
    let f_b = (d8(0x968950) - cb as f64 * d8(0x96a5c0)) as f32;
    let f_9 = (d8(0x968950) - c9 as f64 * d8(0x96a5c0)) as f32;
    let f_d = (d8(0x968950) - cd as f64 * d8(0x96a5c0)) as f32;
    let lim = (level as i16 as i32 * 300 + 4000) as i16;
    let pot = read_i16(st + player_sheet::CURRENT_ABILITY) as i32;
    let band = |new: i32, current: i16, lo: i32, hi: i32| -> i32 {
        let mx = lim.max(current);
        let v = if (new as i16) < mx { new as i16 } else { mx } as i32;
        if lo > v { lo } else if hi >= v { v } else { hi }
    };
    let preso = |current: i16, f: f32, low: f64, high: f64, t: i32| -> i32 {
        let t = t as i16 as f32 as f64;
        let smaller = current as f32 as f64 - f as f64 * low;
        if smaller > t {
            return ftol(smaller);
        }
        let greater = high * f as f64 + current as f32 as f64;
        ftol(if greater < t { greater } else { t })
    };
    let lo = pot * 50 - 3750;
    let tb = band(new_b, cb, lo, pot * 50 + 2500);
    let mut rb = preso(cb, f_b, f4(0x96d210), f4(0x96d20c), tb);
    let t9 = band(new_9, c9, lo, pot * 50 + 2500);
    let r9 = preso(c9, f_9, f4(0x96d210), f4(0x96d208), t9);
    let td = band(new_d, cd, pot * 50 - 10000, pot * 50 + 1250);
    let rd = preso(cd, f_d, f4(0x96d1a8), (level as i32 * 100) as f64, td);
    // the club’s reputation pulls +0xb
    if exe::database_is_national_team(club) == 0 && level > 10 {
        let e = exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person, club);
        if e != 0 {
            let mut id = (person.year_of_birth() as i8) as i32 as f32;
            if read_f32(rdata::F32_19) < id {
                id = 19.0;
            }
            let k = (game_pow(id as f64 * d8(0x96a578), d8(0x968930)) * d8(0x968968)) as f32 as f64;
            let r = club.reputation() as f64;
            let target = match read_i8(e + contract::SQUAD_STATUS) as u8 >> 4 {
                1 => ftol(r * k * d8(0x968978)),
                2 => ftol(r * k * d8(0x9689c0)),
                3 | 5 => ftol(r * k),
                4 => ftol(r * k * d8(0x9689b0)),
                _ => 0,
            };
            if (rb as i16) < target as i16 {
                rb = ftol(target as i16 as f64 * d8(0x967880) + rb as i16 as f64 * d8(0x9689d8));
            }
        }
    }
    let end = |v: i32, p: i32| -> i16 {
        let v = (v as i16).min(10000);
        if v < p as i16 { p as i16 } else { v }
    };
    let (b, n, d) = (end(rb, pa), end(r9, pb), end(rd, pc));
    if jm != 0 {
        *((jm + match_player::NEW_WORLD_REPUTATION) as *mut i16) = d;
        *((jm + match_player::NEW_CURRENT_REPUTATION) as *mut i16) = b;
        *((jm + match_player::NEW_HOME_REPUTATION) as *mut i16) = n;
    } else {
        let st = sheet();
        *((st + player_sheet::CURRENT_REPUTATION) as *mut i16) = b;
        *((st + player_sheet::HOME_REPUTATION) as *mut i16) = n;
        *((st + player_sheet::WORLD_REPUTATION) as *mut i16) = d;
    }
}

/// `CM3_MATCH_ENGINE::get_opposition_challenge` (`0x6c87e0`, ret 0x24, `match_eng.cpp`, ~5.3 KB): picks which of the
/// active opponents in the ball's cell challenges move `tipo` of `atacante` (or of side `lado`, with no attacker).
///
/// The ball's distance to goal (table `0xa15e44`, 1000 outside it) gives the difficulty `dist`. Each standing opponent
/// (`+0x2b` < 2) gets a weight: 3.5 with `+0x8ea9` = 4, otherwise by the tactical marks (`0x7163c0`, or the side's
/// strategy byte with 0x200) 0.5, 4 or 2; times 0.05 far from goal, 2.25 if he marks the attacker (`+0x1a6`), doubled
/// if he chases him (`+0x1aa`), the random draw by reading of the game (`+0x106`) and the attacker's `+0x19c`, form
/// (`+0x24`, `+0x106`, `+0x75`), stamina (`+0x4d`, outside table 1), 1/4 with the side's strategy 5; at least 0.05.
///
/// Each `tipo` has its own contest: random draw(k / weight) against `+0x10d` + `+0x10a` − `dist` (variants for
/// 0x11/0x12, 0x15 with the 5th power (CRT `pow`) of how many are in the cell, 0x6c with 10.5 − `dist`). The winner
/// receives a code and the alternative moves; offside (`0x6cb780`) and position decide `*arriscado`, the value
/// (`0x7165b0`, `0x5407e0`, `0x6f5b20`) chooses between the moves, and control (`0x70d690`) gives the score. The one
/// with the highest score is written to `*nota`/`*lance` and tires (`0x6f56c0` with −1). Returns that opponent or 0.
///
/// [pt] { 0x6c87e0 } (ret 0x24, match_eng.cpp, `CM3_MATCH_ENGINE::get_opposition_challenge`, ~5,3 KB):
/// quem, entre os adversarios ativos na casa da bola, desafia o lance `tipo` de `atacante` (ou do
/// `lado`, sem atacante). A distancia da bola ao gol (tabela 0xa15e44, 1000 fora dela) da a
/// dificuldade `dist`; cada adversario em pe (+0x2b < 2) tem um peso: 3,5 com +0x8ea9 = 4, senao
/// pelas marcas taticas (0x7163c0, ou o byte de estrategia do lado com 0x200) 0,5, 4 ou 2; vezes
/// 0,05 longe do gol, 2,25 se marca o atacante (+0x1a6), dobrado se o persegue (+0x1aa), o sorteio
/// pela leitura (+0x106) e o +0x19c do atacante, a forma (+0x24, +0x106, +0x75), o folego (+0x4d,
/// fora da tabela 1), 1/4 com a estrategia 5 do lado; no minimo 0,05. Cada `tipo` tem a sua
/// disputa: sorteio(k / peso) contra +0x10d + +0x10a - dist (variantes para 0x11/0x12, 0x15 com a
/// potencia 5 (pow da CRT) de quantos estao na casa, 0x6c com 10,5 - dist). Quem vence recebe um codigo e os
/// lances alternativos; o impedimento (0x6cb780) e a posicao decidem `*arriscado`, o valor
/// (0x7165b0, 0x5407e0, 0x6f5b20) escolhe entre os lances, e o dominio (0x70d690) da a nota. O de
/// maior nota fica em `*nota`/`*lance` e cansa (0x6f56c0 com -1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn opposition_challenge(m: usize, atacante: usize, side: i8, kind: i8, a: *mut u8, b: *mut u8, rating: *mut i16, lance: *mut i8, risky: *mut i8, free: u8) -> usize {
    *risky = 0;
    let side = if atacante != 0 { read_i8(atacante + match_player::SIDE) } else { side };
    let flag = (side == 0) as i32;
    if read_i8(m + engine::BALL_HEIGHT) == -1 && (atacante == *((m + engine::GOALKEEPER) as *const usize) || atacante == *((m + engine::GOALKEEPER_SIDE1) as *const usize)) {
        return 0;
    }
    let bx = read_i8(m + engine::BALL_X) as i32;
    let by = read_i8(m + engine::BALL_Y) as i32;
    let cx = ((bx - 4).abs()) as i8;
    let cy = ((if flag == 1 { by - 11 } else { by }).abs()) as i8;
    let base = if (0..=8).contains(&cx) && (0..=11).contains(&cy) { *((G_CELL_DISTANCE_TABLE + (cy as usize * 9 + cx as usize) * 4) as *const f32) } else { read_f32(rdata::F32_1000) };
    let mut dist = (base as f64 + read_f64(rdata::F64_0_5)) as f32;
    if dist as f64 > read_f64(rdata::F64_1) {
        let x = dist as f64 * read_f64(rdata::F64_0_5);
        dist = (x * x + x + read_f64(rdata::F64_1)) as f32;
    }
    let distf = dist as f64;
    let mut best = 0usize;
    let mut contado = false;
    let mut more_near_r = 0i8;
    let mut colegas_a_frente = 1i8;
    let home = m + engine::GRID + ((by + bx * 12) * 90) as usize;
    let n = || read_i8(home + flag as usize + 0x58) as i32;
    let mut i = 0;
    while i < n() {
        let p = *((home + (flag as usize * 11 + i as usize) * 4) as *const usize);
        i += 1;
        if p == 0 {
            // the original logs the error and ends the game
            exe::crt_exit(-1);
        }
        if read_i8(p + match_player::POSITION) < 0 || read_i8(p + match_player::BUSY) >= 2 {
            continue;
        }
        // the weight stays in the register (double precision) between calculations; the in-memory copy is float
        let mut v: f64 = if read_i8(m + engine::BALL_HEIGHT) == 4 {
            read_f32(rdata::F32_3_5) as f64
        } else if (exe::match_player_get_instructions(p) as u32) & 0x200 != 0 {
            let e = *((m + engine::TACTICS_FLAGS + 6371 * flag as usize) as *const u32);
            if e & 0x20 != 0 {
                read_f32(rdata::F32_0_5) as f64
            } else if e & 0x40 != 0 {
                read_f32(rdata::F32_4) as f64
            } else {
                read_f32(rdata::F32_2) as f64
            }
        } else if (exe::match_player_get_instructions(p) as u32) & 0x800 != 0 {
            read_f32(rdata::F32_0_5) as f64
        } else if (exe::match_player_get_instructions(p) as u32) & 0x1000 != 0 {
            read_f32(rdata::F32_4) as f64
        } else {
            read_f32(rdata::F32_2) as f64
        };
        let mut weight = v as f32;
        if distf > read_f64(rdata::F64_5) {
            v *= read_f32(rdata::F32_0_05000000075) as f64;
            weight = v as f32;
        }
        if atacante != 0 {
            if atacante == *((p + match_player::LAST_FOULED_BY) as *const usize) {
                v *= read_f32(rdata::F32_2_25) as f64;
                weight = v as f32;
            }
            if *((p + match_player::MARKING_TARGET) as *const usize) == atacante {
                v += v;
                weight = v as f32;
            }
            if read_i8(m + engine::BALL_HEIGHT) < 4 {
                let leitura = read_i8(p + match_player::MOMENT_DECISIONS) as i32;
                let r1 = random(0x1e - leitura);
                let r2 = random(0x1e - leitura);
                let k = (read_i16(atacante + match_player::BALL_CONTROL) as i32).wrapping_add(r2.wrapping_sub(r1).wrapping_mul(6)) as i16;
                v = (k as f64 * read_f64(rdata::F64_0_0025) + read_f64(rdata::F64_1)) * weight as f64;
            }
        }
        let form = (read_i8(p + match_player::YELLOW_CARDS) as i32 * read_i8(p + match_player::MOMENT_DECISIONS) as i32) as f64;
        weight = (((read_f64(rdata::F64_0_075) - form * read_f64(rdata::F64_0_0025)) * read_f32(p + match_player::AGGRESSION) as f64 + read_f64(rdata::F64_0_5)) * v) as f32;
        if exe::match_player_get_position(p) != 1 {
            weight = (read_f32(p + match_player::STAMINA) as f64 * read_f64(rdata::F64_1ENEG_08) * read_f32(p + match_player::STAMINA) as f64 * weight as f64) as f32;
        }
        if read_i8(m + flag as usize + 0x1ab) == 5 {
            weight = (weight as f64 * read_f64(rdata::F64_0_25)) as f32;
        }
        if (weight as f64) < read_f64(rdata::F64_0_05) {
            weight = f32::from_bits(0x3d4ccccd);
        }
        let somado = (read_i8(p + match_player::MOMENT_WORK_RATE) as i32 + read_i8(p + match_player::MOMENT_POSITIONING) as i32) as f64;
        // the contest: random roll(k / weight) below the target
        let contest = |k: i32| random(ftol(k as f64 / weight as f64));
        let atacante_t1 = || atacante != 0 && exe::match_player_get_position(atacante) == 1;
        let bit100 = *((p + match_player::ACTION) as *const u16) & 0x100 != 0;
        let letras = || match random(4) {
            0 => (0x2a, 0x2c),
            1 => (0x2d, 0x2c),
            2 => (0x2b, 0x2c),
            _ => (0x2e, 0x2c),
        };
        // (codigo, lance, alternativo)
        let won: Option<(i8, i8, i8)> = match kind {
            0xc..=0xe => {
                let k = if atacante_t1() { 3 } else if bit100 { 10 } else { 20 };
                let target = (somado - distf) as f32;
                if (contest(k) as f32) < target {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    None
                }
            }
            0xf | 0x10 | 0x67 => {
                let k = if atacante_t1() { 3 } else if bit100 { 5 } else { 10 };
                let target = (somado - distf) as f32;
                if (contest(k) as f32) < target {
                    let (l, a2) = letras();
                    Some((0x21, l, a2))
                } else {
                    None
                }
            }
            0x11 | 0x12 => {
                let (perto, longe) = if kind == 0x11 { (12, 8) } else { (8, 4) };
                // None: vence direto; Some(k): disputa
                let k = if exe::match_player_get_position(p) == 1 {
                    if best != 0 || n() > 1 {
                        Some(0xc)
                    } else if flag == 0 {
                        if read_i8(p + match_player::CELL_Y) > 0 || read_i8(p + match_player::CELL_X) != 4 { None } else { Some(2) }
                    } else if read_i8(p + match_player::CELL_Y) < 0xb || read_i8(p + match_player::CELL_X) != 4 {
                        None
                    } else {
                        Some(2)
                    }
                } else if free != 0 {
                    Some(2)
                } else if atacante_t1() {
                    Some(3)
                } else if bit100 {
                    Some(longe)
                } else {
                    Some(perto)
                };
                let wins = match k {
                    None => true,
                    Some(k) => {
                        let target = somado - distf * read_f64(rdata::F64_0_5);
                        ((contest(k) + read_i8(atacante + match_player::MOMENT_PACE) as i32) as f64) < target
                    }
                };
                if wins {
                    let (l, a2) = letras();
                    Some((0x13, l, a2))
                } else {
                    None
                }
            }
            0x15 | 0x25 | 0x2e | 0x38 => {
                let k: Option<i32> = if exe::match_player_get_position(p) == 1 {
                    let how_many = n() as f64;
                    let more = if read_i8(p + match_player::CELL_X) == 4 && matches!(read_i8(p + match_player::CELL_Y), 0 | 0xb) { read_f64(rdata::F64_10) } else { read_f64(rdata::F64_3) };
                    let q5 = game_pow(how_many, read_f64(rdata::F64_5));
                    Some(ftol(q5 * read_i8(p + match_player::MOMENT_DECISIONS) as f64 + more))
                } else {
                    let r = random(if distf < read_f64(rdata::F64_2) { 100 } else { 10 });
                    if r as f64 > distf {
                        None
                    } else if bit100 || atacante_t1() {
                        Some(1)
                    } else {
                        Some(3)
                    }
                };
                let wins = match k {
                    None => true,
                    Some(k) => {
                        let r = random(ftol(read_f32(p + match_player::JUMPING) as f64 + read_f32(p + match_player::JUMPING) as f64)) + read_i8(p + match_player::MOMENT_POSITIONING) as i32;
                        r > random(k as i16 as i32)
                    }
                };
                if wins {
                    let a2 = if random(5) != 0 { -1 } else { 0x2f };
                    Some((0x15, 0x2e, a2))
                } else {
                    None
                }
            }
            0x3b => {
                let k = if atacante_t1() { 3 } else if bit100 { 20 } else { 60 };
                let target = (somado - distf) as f32;
                if (contest(k) as f32) < target { Some((0xc, 0x32, 0x32)) } else { None }
            }
            0x6c => {
                let mut k = if atacante_t1() { 10 } else if bit100 { 500 } else { 1000 };
                let (x, y) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
                if (2..=6).contains(&x) && ((flag == 1 && (10..=11).contains(&y)) || (flag == 0 && (0..=1).contains(&y))) {
                    k += 0x514;
                }
                let target = (read_f64(rdata::F64_10_5) - distf) * read_f32(p + match_player::AGGRESSION) as f64 + somado;
                if (contest(k) as f64) < target && read_i8(m + engine::BALL_HEIGHT) < 4 {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    None
                }
            }
            0x6d => {
                let k = if exe::match_player_get_position(p) == 1 {
                    if (exe::match_player_is_in_penalty_area(p, (flag as u32) as u8) as u32) as u8 == 0 {
                        None
                    } else if read_i8(m + engine::SHOOTOUT_STATE) != 0 {
                        Some(5)
                    } else if free != 0 {
                        Some(6)
                    } else {
                        Some(30)
                    }
                } else if free != 0 {
                    if read_i8(p + match_player::MOMENT_ANTICIPATION) != 0 { None } else { Some(2) }
                } else if bit100 {
                    Some(2)
                } else {
                    Some(4)
                };
                let wins = match k {
                    None => true,
                    Some(k) => {
                        let target = (somado - distf) as f32;
                        (contest(k) as f32) < target
                    }
                };
                if !wins {
                    None
                } else if read_i8(m + engine::BALL_HEIGHT) < 2 {
                    let (l, a2) = letras();
                    Some((0x22, l, a2))
                } else {
                    Some((0xc, 0x2e, 0x2e))
                }
            }
            _ => {
                let error: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(FN_CRT_SPRINTF);
                error(0xde1f64, 0xa15954, kind as i32);
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                None
            }
        };
        let Some((codigo, lance1, mut lance2)) = won else { continue };
        if atacante != 0 && !contado && kind != 0x3b && kind != 0x15 {
            more_near_r = exe::cm3_match_engine_check_clear_run_on_goal(m, atacante) as i8;
            let y = read_i8(atacante + match_player::CELL_Y);
            let l = read_i8(atacante + match_player::SIDE);
            let mut count = 0i8;
            for k in 0..20 {
                let q = m + engine::PLAYERS + l as usize * 0x22d8 + k * 0x1be;
                if read_i8(q + match_player::POSITION) >= 0 && ((l == 1 && read_i8(q + match_player::CELL_Y) < y) || (l == 0 && read_i8(q + match_player::CELL_Y) > y)) {
                    count += 1;
                }
            }
            colegas_a_frente = count;
            contado = true;
        }
        if more_near_r != 0 {
            *risky = 1;
        } else {
            let y = read_i8(p + match_player::CELL_Y);
            let last = if read_i8(p + match_player::SIDE) == 0 {
                let u = *((m + engine::LOWEST_Y_PLAYER) as *const usize);
                y <= if u != 0 { read_i8(u + match_player::CELL_Y) } else { 0xc }
            } else {
                let u = *((m + engine::LOWEST_Y_PLAYER_SIDE1) as *const usize);
                y >= if u != 0 { read_i8(u + match_player::CELL_Y) } else { -1 }
            };
            let follows = if last && random(10) == 0 {
                true
            } else {
                let c = read_i8(p + match_player::POSITION);
                (0..=10).contains(&c) && *((*((m + engine::POSITION_TABLE + read_i8(p + match_player::SIDE) as usize * 4) as *const usize) + c as usize * 2) as *const i16) == 1 && random(2) == 0
            };
            if follows && colegas_a_frente == 0 && (exe::match_player_is_on_wing(p) == 0 || exe::match_player_is_in_own_third(p) == 0) {
                *risky = 1;
            }
        }
        let mut strength = random((exe::match_player_get_foul_chance(p, atacante) as u8) as i8 as i32);
        if atacante != 0 && exe::match_player_is_in_attacking_third(atacante) == 0 && *risky == 0 && read_i8(p + match_player::YELLOW_CARDS) == 0 {
            let r1 = random(ftol(read_f32(p + match_player::AGGRESSION) as f64));
            if r1 > random(0xc) {
                let skips = read_i16(atacante + match_player::MATCH_RATING) as i32 <= random(1000) + 0x1d4c && {
                    let rel = exe::database_get_staff_reputation_for_viewer(*((atacante + match_player::STAFF) as *const usize), *((p + match_player::STAFF) as *const usize), 0) as i32;
                    rel <= random(10000)
                };
                if !skips {
                    strength = strength.wrapping_add(random(10));
                    if read_i8(p + match_player::MOMENT_DECISIONS) != 0 {
                        lance2 = -1;
                    }
                }
            }
        }
        let leitura = read_i8(p + match_player::MOMENT_DECISIONS) as i32;
        let b5c = || read_i8(*((p + match_player::STAFF) as *const usize) + 0x5c) as i32;
        let arr = *risky as i32;
        let chosen: Option<i8> = 'e: {
            if kind == 0x15 {
                let fx = (exe::match_player_is_in_penalty_area(p, (flag as u32) as u8) as u32) as i8 as i32;
                if random(b5c() * 200 + fx * 1500 + 0xfa) == 0 {
                    break 'e Some(0x25);
                }
            }
            let fo = strength as i16 as i32;
            let fx = (exe::match_player_is_in_penalty_area(p, (flag as u32) as u8) as u32) as i8 as i32;
            let lim = ((leitura + 5) * read_i8(p + match_player::YELLOW_CARDS) as i32 + fx * leitura * 14 + leitura * arr * 20) * 25 + 0x14;
            if fo > random(lim) {
                break 'e Some(lance1);
            }
            if lance2 > -1 {
                let fx = (exe::match_player_is_in_penalty_area(p, (flag as u32) as u8) as u32) as i8 as i32;
                let lim = (fx * (leitura + 10) + 10) * 500 + (b5c() + 100) * leitura * arr * 80;
                if fo * fo > random(lim) {
                    break 'e Some(lance2);
                }
            }
            if arr != 0 && exe::match_player_get_position(p) != 1 && !(distf < read_f64(rdata::F64_2)) && read_i8(m + engine::BALL_HEIGHT) != 4 && leitura != 0 {
                break 'e None;
            }
            Some(codigo)
        };
        let Some(mut escolha) = chosen else { continue };
        let mut value_play = random(exe::match_player_get_challenge_score(p, &mut escolha, a, b) as i16 as i32);
        if kind == 0x11 && escolha != 0x29 {
            value_play = value_play.wrapping_add(random(ftol(read_i8(p + match_player::MOMENT_PACE) as f64 * read_f32(p + match_player::ACCELERATION) as f64)));
        } else if kind == 0x12 && escolha != 0x29 {
            let c = read_i8(p + match_player::MOMENT_PACE) as i32;
            value_play = value_play.wrapping_add(c * c);
        }
        let v16 = value_play as i16;
        let accepted = v16 > 0 || (exe::match_player_is_in_penalty_area(p, (read_i8(p + match_player::SIDE) as u8 as u32) as u8) as u32) as u8 != 0 || (leitura == 0 && read_i8(p + match_player::MOMENT_WORK_RATE) as i32 > random(100));
        if !accepted {
            continue;
        }
        if best != 0 && v16 <= *rating {
            continue;
        }
        best = p;
        *lance = escolha;
        *rating = v16;
    }
    if best != 0 {
        exe::match_player_drain_stamina(best, -1.0, *lance as i16);
    }
    best
}

/// 0x6bcbd0 (ret 0xc, match_eng.cpp) with the list copy injected (0x68d0d0), without random draws: sums into `*saida`
/// the pressure of the active opponents in `j`'s cell (the list for that side on the grid) and returns how many weigh
/// (2 per marker).
///
/// Whoever marks `j` (`exe::match_player_is_marked_by`) counts (`field_by_position` + 0x10a - `j`'s +0x19c / 20 - `j`'s +0x107) x 1.7; the others standing
/// (+0x2b >= 0), outside table 1 (`player_table`) and with +0x10a or bit 0x100 of +0x198, count (0x10a - ...) x 1.7 / (+0x2b
/// + 1).
///
/// With `lista`, the two that weigh the most (above 5) go into it as a 0x1be-byte copy.
///
/// [pt] { 0x6bcbd0 } (ret 0xc, match_eng.cpp) com a copia na lista injetada (0x68d0d0), sem sortear:
/// soma em `*saida` a pressao dos adversarios ativos na casa de `j` (a lista do lado de la na grade)
/// e devolve quantos pesam (2 por marcador). Quem marca `j` (0x714c60) vale (0x6f4600 + 0x10a -
/// +0x19c de `j` / 20 - +0x107 de `j`) x 1,7; os outros em pe (+0x2b >= 0), fora da tabela 1
/// (0x717410) e com +0x10a ou o bit 0x100 de +0x198, valem (0x10a - ...) x 1,7 / (+0x2b + 1). Com
/// `lista`, os dois que mais pesam (acima de 5) vao para ela como copia de 0x1be bytes.
pub(crate) unsafe fn pressure_on_player_with(m: usize, j: usize, out: *mut i16, list: usize, copy: &mut dyn FnMut(usize, usize)) -> u8 {
    *out = 0;
    let mut count = 0u8;
    let (mut melhor, mut v_melhor) = (0usize, -1000i32);
    let (mut segundo, mut v_segundo) = (0usize, -1000i32);
    let home = m + engine::GRID + (read_i8(j + match_player::CELL_X) as isize * 12 + read_i8(j + match_player::CELL_Y) as isize).wrapping_mul(90) as usize;
    let side = (read_i8(j + match_player::SIDE) == 0) as usize;
    let n = read_i8(home + side + 0x58) as i32;
    let base = || read_i8(j + match_player::MOMENT_MOVEMENT) as i32 + *((j + match_player::BALL_CONTROL) as *const i16) as i32 / 20;
    let k = read_f64(rdata::F64_1_7);
    for i in 0..n.max(0) as usize {
        let p = *((home + side * 44 + i * 4) as *const usize);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        let mut weight = 0i32;
        if exe::match_player_is_marked_by(j, p) != 0 {
            count = count.wrapping_add(2);
            let v = exe::match_player_get_marking_rating(p) as i8 as i32 + read_i8(p + match_player::MOMENT_POSITIONING) as i32 - base();
            let r = ftol(v as f64 * k);
            *out = (*out).wrapping_add(r as i16);
            weight = r.wrapping_add(r);
        } else if read_i8(p + match_player::BUSY) >= 0 && exe::match_player_get_position(p) != 1 && (read_i8(p + match_player::MOMENT_POSITIONING) != 0 || *((p + match_player::ACTION) as *const u16) & 0x100 != 0) {
            count = count.wrapping_add(1);
            let v = read_i8(p + match_player::MOMENT_POSITIONING) as i32 - base();
            weight = ftol(v as f64 * k / (read_i8(p + match_player::BUSY) as i32 + 1) as f64);
            *out = (*out).wrapping_add(weight as i16);
        }
        if list != 0 && weight as i16 > 5 {
            if weight as i16 > v_melhor as i16 {
                if melhor != 0 {
                    segundo = melhor;
                    v_segundo = v_melhor;
                }
                melhor = p;
                v_melhor = weight;
            } else if weight as i16 > v_segundo as i16 {
                segundo = p;
                v_segundo = weight;
            }
        }
    }
    if list != 0 {
        if melhor != 0 {
            copy(list, melhor);
        }
        if segundo != 0 {
            copy(list, segundo);
        }
    }
    count
}

/// Sums into `*saida` the pressure on player `j` from the active opponents in his cell and returns how many weigh; with
/// `lista`, copies the two heaviest into it. Calls pressure_on_player_with with the game's list append.
///
/// [pt] { 0x6bcbd0 } (ret 0xc, match_eng.cpp): ver `pressure_on_player_with`.
pub(crate) unsafe extern "thiscall" fn pressure(m: usize, j: usize, out: *mut i16, list: usize) -> u8 {
    pressure_on_player_with(m, j, out, list, &mut |l, p| {
        exe::llist_add_tail(l, p, 0x1be);
    })
}

// Port of 0x68d0d0 (puts a copy of a block on the list, allocating): tape unit, for the copy
// Rust not to allocate again.
pub(crate) static mut LLIST_ADD_TAIL_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_68D0D0: Prologue = Prologue { address: 0x68d0d0, bytes: &[0x6a, 0xff, 0x68, 0x56, 0xcb, 0x95, 0x00] };

/// Gate for LLIST::add_tail (0x68d0d0): when recording, records it (by content for stack blocks in wrap-up, injury,
/// movement and pass comparisons), else just calls it.
pub(crate) unsafe extern "thiscall" fn gate_llist_add_tail(l: usize, p: usize, n: u32) {
    let o: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(LLIST_ADD_TAIL_TRAMPOLINE);
    if !is_recording() {
        return o(l, p, n);
    }
    if is_recording_wrap_up_calls() || is_recording_injury() || is_recording_move() || is_recording_pass() {
        // at close (and on injury) the block comes from the stack (different address in the Rust copy): checks the content
        call_recording_buffers(0x68d0d0, l, [0, n, 0, 0, 0], &[(p, n as usize)], &[], || {
            o(l, p, n);
            0
        });
        return;
    }
    call_recording(FN_LLIST_ADD_TAIL, taped_pressure_list_addr(l, n), [p as u32, n, 0, 0, 0], || {
        o(l, p, n);
        0
    });
}

/// `0x6bd930` (match_eng.cpp, `CM3_MATCH_ENGINE::period_completed`) with the calls that change the game injected
/// (commentary 0x6d6f70, standings `exe::cm3_match_engine_post_period_end_events`, summary `exe::cm3_match_engine_rebuild_team_match_stats`): closes the current period when the clock passes it
/// (half time, full time, extra time, penalties); returns the next state.
///
/// It closes the period of `engine::PERIOD_END` when the clock (`engine::CLOCK`) passes its end plus stoppage time (`engine::STOPPAGE_TIME`). With `engine::RESTART_TYPE` =
/// 5, nothing. Period 0 or 1: returns 4.
///
/// 0x1ef (1st half): writes the half-time score to the match (`engine::MATCH`: +0x45/+0x46), comments 0x20f1, recovers stamina
/// and moves to the 2nd half (returns 1).
///
/// 0x3de (2nd half): writes the score (+0x43/+0x44); by the match rule (+0x38: 2 second leg, 4, 0x4000; 1 and 0x20
/// decide by penalties 4 or 1 at `engine::SHOOTOUT_STATE`) and the aggregate (+0x4b, +0x4c), it ends (4, with the standings commented,
/// stamina ceiling and summary) or goes to extra time (comments 0x20f2, returns 2).
///
/// 0x483 (extra time, 1st half): with +0x1a8 it ends; otherwise comments 0x20f3 and moves to the 2nd (3). 0x528: writes
/// +0x47/+0x48 and ends, with penalties when the aggregate after extra time is level.
///
/// Unknown period: logs the error (0x944c9f) and returns 0, as when the clock has not passed.
///
/// [pt] { 0x6bd930 } (match_eng.cpp, `CM3_MATCH_ENGINE::period_completed`) com as chamadas que mexem no
/// jogo injetadas (comentarios 0x6d6f70, classificacao 0x6be120, resumo 0x6ce460): fecha o periodo
/// de +0x8ed4 quando o relogio (+0x8ed0) passa do fim dele mais os acrescimos (+0x8eb7). Com +0x8eb2
/// = 5 nada. Periodo 0 ou 1: devolve 4. 0x1ef (1o tempo): grava o placar do intervalo no jogo
/// (+0x4792: +0x45/+0x46), narra 0x20f1, recupera o folego e passa ao 2o tempo (devolve 1). 0x3de
/// (2o tempo): grava o placar (+0x43/+0x44); pela regra do jogo (+0x38: 2 jogo de volta, 4, 0x4000;
/// 1 e 0x20 decidem por penaltis 4 ou 1 em +0x8eb3) e o agregado (+0x4b, +0x4c), encerra (4, com a
/// classificacao narrada, teto do folego e resumo) ou vai a prorrogacao (narra 0x20f2, devolve 2).
/// 0x483 (prorrogacao, 1o tempo): com +0x1a8 encerra; senao narra 0x20f3 e passa ao 2o (3). 0x528:
/// grava +0x47/+0x48 e encerra, com penaltis quando o agregado da prorrogacao empata. Periodo
/// desconhecido: registra o erro (0x944c9f) e devolve 0, como quando o relogio nao passou.
pub(crate) unsafe fn period_completed_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |o: usize| (m + o) as *mut i8;
    let w = |o: usize| (m + o) as *mut i16;
    if read_i8(m + engine::RESTART_TYPE) == 5 {
        return 0;
    }
    let per = *w(engine::PERIOD_END) as i32;
    let passed = |end: i32| *w(engine::CLOCK) as i32 > read_i8(m + engine::STOPPAGE_TIME) as i32 + end;
    let t = Fixture(*((m + engine::MATCH) as *const usize));
    let tb = |o: usize| (t.addr() + o) as *mut i8;
    let commentator = *((m + engine::COMMENTATOR) as *const usize);
    let fala = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| {
        c(0x6d6f70, commentator, [code, m as u32, u32::MAX, 0, u32::MAX]);
    };
    let classificar = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        c(0x6be120, m, [0, 0, 0, 0, 0]);
        c(0x6be120, m, [1, 0, 0, 0, 0]);
    };
    let recuperar = |ceiling: u8| {
        for base in [0x4796, 0x6a6e] {
            for k in 0..20 {
                exe::match_player_recover_condition_at_break(m + base + k * 0x1be, ceiling);
            }
        }
    };
    let release_owner = || {
        let owner = *((m + engine::BALL_HOLDER) as *const usize);
        if owner != 0 {
            exe::match_player_release_possession(owner);
        }
    };
    // possession at the restart: whoever did not kick off in the 1st half kicks off now
    let swap_out = || {
        let a = read_i8(m + engine::NEXT_KICKOFF_SIDE);
        *b(engine::RESTART_TYPE) = 8;
        *b(engine::KICKOFF_ANNOUNCED) = 0;
        *b(engine::POSSESSION_SIDE) = a;
        *b(engine::NEXT_KICKOFF_SIDE) = (a == 0) as i8;
    };
    match per {
        0x1ef => {
            if !passed(0x1ef) {
                return 0;
            }
            *tb(fixture::HOME_GOALS_HALF_TIME) = read_i8(m + engine::SCORE);
            *tb(fixture::AWAY_GOALS_HALF_TIME) = read_i8(m + engine::SCORE_SIDE1);
            fala(call, 0x20f1);
            *w(engine::CLOCK) = 0x1ef;
            *b(engine::STOPPAGE_TIME) = 0;
            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0x3de, 0, 0, 0, 0]);
            recuperar(0);
            *w(engine::PERIOD_END) = 0x3de;
            *b(engine::RESTART_TYPE) = 8;
            *b(engine::KICKOFF_ANNOUNCED) = 0;
            release_owner();
            swap_out();
            1
        }
        0x3de => {
            if !passed(0x3de) {
                return 0;
            }
            *tb(fixture::HOME_GOALS) = read_i8(m + engine::SCORE);
            *tb(fixture::AWAY_GOALS) = read_i8(m + engine::SCORE_SIDE1);
            let rule = t.tie_rules();
            let cl = t.home_aggregate();
            let empate = || t.home_goals() as i32 + cl as i32 == t.away_aggregate() as i32 + t.away_goals() as i32;
            // ends with the standings narrated before (or after) closing the period
            let encerrar_antes = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                *b(engine::RESTART_TYPE) = 8;
                *b(engine::KICKOFF_ANNOUNCED) = 0;
                *w(engine::PERIOD_END) = 0;
                classificar(c);
                recuperar(1);
                c(0x6ce460, m, [0, 0, 0, 0, 0]);
                4
            };
            if rule & 2 == 0 {
                let decides = cl != -1 && rule & 4 == 0 && t.away_goals() != cl && empate();
                if !decides && rule & 1 != 0 && empate() {
                    *b(engine::SHOOTOUT_STATE) = 4;
                } else if !decides && rule & 0x20 != 0 && empate() {
                    *b(engine::SHOOTOUT_STATE) = 1;
                } else {
                    classificar(call);
                }
                *b(engine::RESTART_TYPE) = 8;
                *b(engine::KICKOFF_ANNOUNCED) = 0;
                recuperar(1);
                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0, 0, 0, 0, 0]);
                *w(engine::PERIOD_END) = 0;
                return 4;
            }
            if cl == -1 {
                if t.home_goals() != t.away_goals() {
                    return encerrar_antes(call);
                }
                fala(call, 0x20f2);
                *w(engine::CLOCK) = 0x3de;
                *b(engine::STOPPAGE_TIME) = 0;
                *w(engine::PERIOD_END) = 0x483;
                release_owner();
                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0x3de, 0, 0, 0, 0]);
                swap_out();
                return 2;
            }
            if !empate() {
                return encerrar_antes(call);
            }
            if rule & 4 == 0 && t.away_goals() != cl && rule & 0x4000 == 0 {
                return encerrar_antes(call);
            }
            fala(call, 0x20f2);
            *w(engine::CLOCK) = 0x3de;
            *b(engine::STOPPAGE_TIME) = 0;
            *w(engine::PERIOD_END) = 0x483;
            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [0x3de, 0, 0, 0, 0]);
            release_owner();
            swap_out();
            2
        }
        0x483 => {
            if read_i8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
                *tb(fixture::HOME_GOALS_EXTRA_TIME) = read_i8(m + engine::SCORE);
                *tb(fixture::AWAY_GOALS_EXTRA_TIME) = read_i8(m + engine::SCORE_SIDE1);
                *b(engine::RESTART_TYPE) = 8;
                *b(engine::KICKOFF_ANNOUNCED) = 0;
                *w(engine::PERIOD_END) = 1;
                classificar(call);
                recuperar(1);
                call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [1, 0, 0, 0, 0]);
                return 4;
            }
            if !passed(0x483) {
                return 0;
            }
            fala(call, 0x20f3);
            *w(engine::CLOCK) = 0x483;
            *b(engine::STOPPAGE_TIME) = 0;
            *w(engine::PERIOD_END) = 0x528;
            *b(engine::RESTART_TYPE) = 8;
            *b(engine::KICKOFF_ANNOUNCED) = 0;
            release_owner();
            swap_out();
            3
        }
        0x528 => {
            if read_i8(m + engine::GOLDEN_GOAL_SCORED) == 0 && !passed(0x528) {
                return 0;
            }
            *tb(fixture::HOME_GOALS_EXTRA_TIME) = read_i8(m + engine::SCORE);
            *tb(fixture::AWAY_GOALS_EXTRA_TIME) = read_i8(m + engine::SCORE_SIDE1);
            let cl = t.home_aggregate();
            let rule = t.tie_rules();
            let empate = || t.home_goals_extra_time() as i32 + cl as i32 == t.away_aggregate() as i32 + t.away_goals_extra_time() as i32;
            let penalties = if cl == -1 {
                rule & 0x21 != 0
            } else if rule & 0x4000 != 0 {
                empate() && t.away_goals_extra_time() == cl && rule & 0x21 != 0
            } else {
                empate() && rule & 0x21 != 0
            };
            let equal = t.home_goals_extra_time() as i32 - t.home_goals() as i32 == t.away_goals_extra_time() as i32 - t.away_goals() as i32;
            if rule & 1 != 0 && equal && penalties {
                *b(engine::SHOOTOUT_STATE) = 4;
            } else if rule & 0x20 != 0 && equal && penalties {
                *b(engine::SHOOTOUT_STATE) = 1;
            } else {
                classificar(call);
            }
            *b(engine::RESTART_TYPE) = 8;
            *b(engine::KICKOFF_ANNOUNCED) = 0;
            *w(engine::PERIOD_END) = 1;
            recuperar(1);
            call(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS, m, [1, 0, 0, 0, 0]);
            4
        }
        0 | 1 => 4,
        _ => {
            let error: unsafe extern "C" fn(usize, usize, i32) = core::mem::transmute(FN_CRT_SPRINTF);
            error(0xde1f64, 0xa158f8, per);
            0
        }
    }
}

/// closes the period in `engine::PERIOD_END` when the clock (`engine::CLOCK`) passes its end plus stoppage (`engine::STOPPAGE_TIME`). 1st half: half-time
/// score into the fixture (`engine::MATCH` +0x45/+0x46), narrates 0x20f1, recovers stamina and moves to the 2nd (1). 2nd half:
/// by the fixture rule (+0x38) and aggregate (+0x4b/+0x4c), ends (4: standings `exe::cm3_match_engine_post_period_end_events`, stamina ceiling, summary
/// `exe::cm3_match_engine_rebuild_team_match_stats`; penalties in `engine::SHOOTOUT_STATE`) or goes to extra time (0x20f2, 2). Extra time: 0x20f3 and 2nd half (3), then ends.
/// Unknown period: logs the error (0x944c9f). Standings and summary become tape units when called directly by the
/// compared routine
/// Original: `period_completed` (0x6bd930, match_eng.cpp (`period_completed`), thiscall).
///
/// [pt] { 0x6bd930 } (match_eng.cpp): ver `period_completed_with`.
pub(crate) unsafe extern "thiscall" fn period_completed(m: usize) -> u8 {
    period_completed_with(m, &mut |rotina, this, a| {
        match rotina {
            0x6be120 => {
                let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_POST_PERIOD_END_EVENTS);
                f(this, a[0]);
            }
            0x6ce460 => {
                let f: unsafe extern "thiscall" fn(usize, i16) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REBUILD_TEAM_MATCH_STATS);
                f(this, a[0] as i16);
            }
            _ => add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]),
        }
        0
    })
}

/// { `exe::cm3_match_engine_move_ball_to_cell` } (ret 0x1c, match_eng.cpp) with the calls that touch the game injected (`shot_on_goal`, `shot_outcome` goal-line
/// actions, `exe::cm3_match_engine_ball_out_of_play` ball out): the ball deflects from (x, y).
///
/// It releases the ball from its holder, so nobody has it (`engine::POSSESSION_SIDE` = −1); `lado` −1 becomes random draw(2) = 0. Without
/// `dentro`, outside the central area and by random draw(4), it goes out for a throw-in (column 9 or −1) with the row ±
/// random draw(espalhar + 1); otherwise it lands at x and y ± random draw(espalhar + 1) (with `dentro`, until the row
/// is within 0..=11). Height (`engine::BALL_ROLL`) 1 + random draw(espalhar + 1), or 0 with no scatter; spin (`engine::BALL_DIRECTION`) from table
/// 0xb67c30 by the deflection (unchanged with no deflection); `engine::BALL_HEIGHT` = random draw(4); the ball stays in the square
/// and `lado` gets the possession count (`engine::POSSESSION_COUNTER`, the other loses 10).
///
/// With `cansa`, each player on the pitch without +0x105 adds random draw(2) to +0x2b (`exe::match_player_add_action_delay`) if random draw(3) is
/// zero. On the goal line (column 4, row −1 or 12) with `chutador`, by random draw(4): `shot_outcome` or `shot_on_goal` depending
/// on his side and the row; then, without `engine::RESTART_TYPE`, ball out (`exe::cm3_match_engine_ball_out_of_play`).
///
/// [pt] { 0x6cd770 } (ret 0x1c, match_eng.cpp) com as chamadas que mexem no jogo injetadas (0x70a910,
/// 0x70c040 lances na linha de fundo, 0x6cd490 bola fora): a bola espirra de (x, y). Solta a bola do
/// dono, ninguem a tem (+0x8eae = -1); `lado` -1 vira sorteio(2) = 0. Sem `dentro`, fora do miolo e
/// por sorteio(4), sai pela lateral (coluna 9 ou -1) com a linha +- sorteio(espalhar + 1); senao cai
/// em x e y +- sorteio(espalhar + 1) (com `dentro`, ate a linha ficar em 0..=11). Altura (+0x8eaa)
/// 1 + sorteio(espalhar + 1), ou 0 sem espalhar; efeito (+0x8ed8) pela tabela 0xb67c30 do desvio (o
/// mesmo sem desvio); +0x8ea9 = sorteio(4); a bola fica na casa e o `lado` conta a posse (+0x8d46, o
/// outro perde 10). Com `cansa`, quem esta em campo sem +0x105 soma sorteio(2) em +0x2b (0x7160d0) se
/// sorteio(3) for zero. Na linha de fundo (coluna 4, linha -1 ou 12) com `chutador`, por sorteio(4):
/// 0x70c040 ou 0x70a910 pelo lado dele e a linha; depois, sem +0x8eb2, a bola fora (0x6cd490).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn deflect_ball_with(this: usize, x: i8, y: i8, spread: i8, mut side: i8, inside: u8, cansa: u8, shooter: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |o: usize| (this + o) as *mut i8;
    let owner = *((this + engine::BALL_HOLDER) as *const usize);
    if owner != 0 {
        exe::match_player_release_possession(owner);
    }
    *b(engine::BALL_X) = x;
    *b(engine::BALL_Y) = y;
    if read_i8(this + engine::POSSESSION_SIDE) != -1 {
        *b(engine::POSSESSION_SIDE) = -1;
    }
    if side == -1 {
        side = (random(2) == 0) as i8;
    }
    let n = spread as i32 + 1;
    let (tx, mut ty): (i8, i8);
    let bx = read_i8(this + engine::BALL_X);
    if inside == 0 && !(2..=6).contains(&bx) && random(4) != 0 {
        tx = if bx >= 2 { 9 } else { -1 };
        let r = random(n) as i8;
        ty = read_i8(this + engine::BALL_Y).wrapping_sub(r).wrapping_add(random(n) as i8);
    } else {
        loop {
            let r = random(n) as i8;
            let x2 = read_i8(this + engine::BALL_X).wrapping_sub(r).wrapping_add(random(n) as i8);
            let r = random(n) as i8;
            ty = read_i8(this + engine::BALL_Y).wrapping_sub(r).wrapping_add(random(n) as i8);
            if inside == 0 || (0..=11).contains(&ty) {
                tx = x2;
                break;
            }
        }
    }
    *b(engine::BALL_ROLL) = if spread == 0 { 0 } else { (random(n) + 1) as i8 };
    let dx = tx.wrapping_sub(read_i8(this + engine::BALL_X));
    let dy = ty.wrapping_sub(read_i8(this + engine::BALL_Y));
    if dx != 0 || dy != 0 {
        let i = dy as isize + 24 * dx as isize;
        *((this + engine::BALL_DIRECTION) as *mut u16) = *((0xb67c30 + 2 * i) as *const u16);
    }
    *b(engine::BALL_HEIGHT) = random(4) as i8;
    *b(engine::BALL_X) = tx;
    *b(engine::BALL_Y) = ty;
    *b(engine::LAST_POSSESSION_SIDE) = side;
    if side != -1 {
        let my = (this as isize + 0x8d46 + side as isize) as *mut i8;
        *my = (*my).wrapping_add(1);
        let other = (this as isize + 0x8d46 + (side == 0) as isize) as *mut i8;
        if *other > 0 {
            *other = (*other).wrapping_sub(10);
        }
    }
    if cansa != 0 {
        for l in 0..2i8 {
            for k in 0..20 {
                let j = side_player_slot(this, l, k);
                if read_i8(j + match_player::POSITION) >= 0 && read_i8(j + match_player::MOMENT_ANTICIPATION) == 0 && random(3) == 0 {
                    exe::match_player_add_action_delay(j, random(2) as u8);
                }
            }
        }
    }
    let bottom = shooter != 0 && read_i8(this + engine::BALL_X) == 4 && matches!(read_i8(this + engine::BALL_Y), -1 | 12);
    if bottom && random(4) != 0 {
        let d = read_i8(shooter + match_player::SIDE);
        let routine = if read_i8(this + engine::BALL_Y) == 12 { if d != 0 { 0x70c040 } else { 0x70a910 } } else if d == 0 { 0x70c040 } else { 0x70a910 };
        call(routine, shooter, [0x3f80_0000, 0, 0, 0, 0]);
        if read_i8(this + engine::RESTART_TYPE) != 0 {
            return;
        }
    }
    call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, this, [side as u8 as u32, 0, 0, 0, 0]);
}

/// The ball deflects from cell (x, y): releases it, scatters it (or out for a throw-in), sets height, spin and
/// possession, optionally tires players, and handles goal-line cases and ball out of play; calls deflect_ball_with with
/// the game's real calls.
///
/// [pt] { 0x6cd770 } (ret 0x1c, match_eng.cpp): ver `deflect_ball_with`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn deflect_ball(this: usize, x: i8, y: i8, spread: i8, side: i8, inside: u8, cansa: u8, shooter: usize) {
    deflect_ball_with(this, x, y, spread, side, inside, cansa, shooter, &mut |rotina, alvo, a| match rotina {
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            f(alvo, a[0] as u8) as u32
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(rotina);
            f(alvo, f32::from_bits(a[0]));
            0
        }
    })
}

// Ports of 0x70a910 and 0x70c040 (plays on the byline, ~6 and 5 KB): tape units.

/// After a move: whether player j (or the ball holder) is past his side's defensive line (engine `engine::OFFSIDE_LINE_SIDE1` / `engine::OFFSIDE_LINE`).
///
/// [pt] Depois de mover para a casa nova: o jogador sem a bola passou da linha defensiva do lado dele
/// (+0x8eb0 menos y no lado 1, y menos +0x8eaf no outro, nao negativo)? Quem tem a bola (+0xf582)
/// conta como passou. Trecho repetido de 0x6c4ec0 e 0x6c4820.
pub(crate) unsafe fn is_past_defensive_line(engine: usize, j: usize) -> bool {
    if j == *((engine + engine::BALL_HOLDER) as *const usize) {
        return true;
    }
    let y = read_i8(j + match_player::CELL_Y);
    let d = if side_of_player(j) == 1 { read_i8(engine + engine::OFFSIDE_LINE_SIDE1).wrapping_sub(y) } else { y.wrapping_sub(read_i8(engine + engine::OFFSIDE_LINE)) };
    d >= 0
}

/// Lines the teams up for a set piece of side `engine::POSSESSION_SIDE`: counter `engine::SET_PIECES` + 54 × side; releases the ball. Ball in the
/// side's own area: formation (as `line_up_kickoff`) and whoever of the other side lands on the spot moves one row. Otherwise:
/// spot clamped to the grid, far (`engine::SET_PIECE_KIND` = 7) or near (−1), base column 4 − draw + draw, base row 3/4 from the spot;
/// near, everyone from the set-piece table (12 rows × 9 columns per position and kicking side); far, by role (`attacking_role`
/// in attack: 1 deep, 2 middle; `defending_role` in defence: 4 deep, 3 middle). Finally the taker (`exe::cm3_match_engine_pick_restart_taker`, type `engine::RESTART_TYPE`) goes
/// to the spot, takes the ball and aims (`exe::match_player_set_facing_direction`). Cell off the grid: fatal error, as in the original.
///
/// Details: counter `engine::SET_PIECES` + 54 × side goes up; `engine::POSSESSION_SIDE` = -1; the ball is released (`release_ball`). Ball in the side's own
/// area (row < 3 on side 0, > 8 on side 1): each player goes to his formation cell (as `line_up_kickoff`), and whoever of the
/// other side lands on the ball's spot moves one row; on the kicking side, whoever passed the line goes back one
/// (`exe::match_player_drop_back_one_row`). Otherwise (spot clamped to 0..=8 × 0..=11): far from goal (`engine::SET_PIECE_KIND` = 7) or near (-1); base column 4 -
/// random(2) + random(2) and base row 3 (near) or 4 (far) from the spot towards the goal. Near: everyone from the
/// side's set-piece table (`engine::FORMATION_TABLE`, 12 rows per column, 9 columns per position and kicking side). Far: by role: in
/// attack (`attacking_role`) 1 goes deep and 2 to the middle, in defence (`defending_role`) 4 deep and 3 to the middle, the others from
/// the table. Middle: x 4 ± random, row 6 (side 1) or 5. Deep: x of the base column ± random and row 0..1 or 10..11
/// next to the attacked or defended goal. Whoever passed the line goes back one (`exe::match_player_drop_back_one_row`). Cell off the grid: the
/// original ends the game (fatal error).
///
/// At the end the taker (`exe::cm3_match_engine_pick_restart_taker` with type `engine::RESTART_TYPE`) goes to the spot, takes the ball (`take_ball`) and aims (`exe::match_player_set_facing_direction`, 4,
/// 1 on side 1 or 10), with +0x19c = 200. Everyone zeroes +0x2b (`exe::match_player_set_action_delay`).
/// Original: `line_up_set_piece` (0x6c4820, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4820 } (match_eng.cpp): arruma os times para a bola parada do lado +0x8eae (contador
/// +0xf5c6 + 54 x lado sobe; +0x8eae = -1; a bola e solta, 0x6f0970).
/// Bola na area do proprio lado (linha < 3 no lado 0, > 8 no lado 1): cada um vai a casa da formacao
/// (como 0x6c4ec0), e quem do outro lado cair na marca da bola sai uma linha; no lado que cobra,
/// quem passou da linha volta uma (0x6f5b80).
/// Senao (marca limitada a 0..=8 x 0..=11): longe do gol (+0x8eb1 = 7) ou perto (-1); coluna base 4 -
/// sorteio(2) + sorteio(2) e linha base a 3 (perto) ou 4 (longe) da marca para o lado do gol. Perto,
/// todos pela tabela de bola parada do lado (+0xf56e, 12 linhas por coluna, 9 colunas por posicao e
/// lado que cobra). Longe, pelo papel: no ataque (0x716270) 1 vai ao fundo e 2 ao meio, na defesa
/// (0x716310) 4 ao fundo e 3 ao meio, os outros pela tabela. Meio: x 4 +- sorteio, linha 6 (lado 1) ou 5.
/// Fundo: x da coluna base +- sorteio e linha 0..1 ou 10..11 junto ao gol atacado ou defendido. Quem
/// passou da linha volta uma (0x6f5b80). Casa fora da grade: a original encerra o jogo (erro fatal).
/// No fim o cobrador (0x6ceb90 com o tipo +0x8eb2) vai a marca, pega a bola (0x6f0630) e aponta
/// (0x6f4920, 4, 1 no lado 1 ou 10), com +0x19c = 200. Todos zeram +0x2b (0x7160e0).
pub(crate) unsafe extern "thiscall" fn line_up_set_piece(this: usize) {
    let b = |o: usize| (this + o) as *mut i8;
    let side_b = read_i8(this + engine::POSSESSION_SIDE);
    let counter = (this as isize + 0xf5c6 + 54 * side_b as isize) as *mut u8;
    *counter = (*counter).wrapping_add(1);
    *b(engine::POSSESSION_SIDE) = -1;
    if read_usize(this + engine::BALL_HOLDER) != 0 {
        exe::match_player_release_possession(read_usize(this + engine::BALL_HOLDER));
    }
    let in_the_box = (side_b == 0 && read_i8(this + engine::BALL_Y) < 3) || (side_b == 1 && read_i8(this + engine::BALL_Y) > 8);
    if in_the_box {
        for l in 0..2i8 {
            let sai = side_b as i32 == l as i32;
            for k in 0..20 {
                let j = side_player_slot(this, l, k);
                let pos = read_i8(j + match_player::POSITION);
                if pos < 0 {
                    continue;
                }
                let table = read_usize(this + (0xf56e + 4 * (read_i8(j + match_player::SIDE) == 1) as usize));
                let home = *((table + pos as usize * 0xd8 + 0xa2) as *const u8);
                let x = (home & 0xf) as i8;
                let mut y = (home >> 4) as i8;
                if x == read_i8(this + engine::BALL_X) && y == read_i8(this + engine::BALL_Y) && !sai {
                    y = if side_b == 0 {
                        if y < 11 { y + 1 } else { y - 1 }
                    } else if y <= 0 {
                        y + 1
                    } else {
                        y - 1
                    };
                }
                move_to_cell(j, x, y);
                exe::cm3_match_engine_update_offside_lines(this);
                if is_past_defensive_line(this, j) && sai {
                    exe::match_player_drop_back_one_row(j, 1);
                }
                exe::match_player_set_action_delay(j, 0);
            }
        }
    } else {
        if read_i8(this + engine::BALL_Y) >= 12 {
            *b(engine::BALL_Y) = 11;
        } else if read_i8(this + engine::BALL_Y) < 0 {
            *b(engine::BALL_Y) = 0;
        }
        if read_i8(this + engine::BALL_X) >= 9 {
            *b(engine::BALL_X) = 8;
        } else if read_i8(this + engine::BALL_X) < 0 {
            *b(engine::BALL_X) = 0;
        }
        let mark = read_i8(this + engine::BALL_Y);
        let far = if side_b == 0 { mark > 6 } else { mark < 5 };
        *b(engine::SET_PIECE_KIND) = if far { 7 } else { -1 };
        let s1 = random(2) as i8;
        let column = 4i8.wrapping_sub(s1).wrapping_add(random(2) as i8);
        let step_towards = if far { 4 } else { 3 };
        let bruta = if side_b == 0 { mark.wrapping_add(step_towards) } else { mark.wrapping_sub(step_towards) };
        let row = if bruta >= 12 { 11 } else if bruta < 0 { 0 } else { bruta };
        for l in 0..2i8 {
            let sai = side_b as i32 == l as i32;
            for k in 0..20 {
                let j = side_player_slot(this, l, k);
                let pos = read_i8(j + match_player::POSITION);
                if pos < 0 {
                    continue;
                }
                let of_table = || {
                    let t = read_usize(this + (0xf56e + 4 * l as usize));
                    let c = *((t as isize + 12 * ((sai as isize + 2 * pos as isize) * 9 + column as isize) + row as isize) as *const u8);
                    ((c & 0xf) as i8, (c >> 4) as i8)
                };
                let (x, y) = if read_i8(this + engine::SET_PIECE_KIND) <= 0 {
                    of_table()
                } else {
                    let (r, fundo, meio) = if sai { (exe::match_player_get_attacking_set_piece_role(j), 1, 2) } else { (exe::match_player_get_defending_set_piece_role(j), 4, 3) };
                    if r == meio {
                        let s1 = random(2) as i8;
                        (4i8.wrapping_sub(s1).wrapping_add(random(2) as i8), if l == 1 { 6 } else { 5 })
                    } else if r == fundo {
                        let s1 = random(2) as i8;
                        let x = column.wrapping_sub(s1).wrapping_add(random(2) as i8);
                        let s3 = random(2) as i8;
                        // by the top goal (row 11) whoever defends side 1 or attacks with side 0
                        (x, if (l == 1) != sai { 11 - s3 } else { s3 })
                    } else {
                        of_table()
                    }
                };
                if !(0..=8).contains(&x) || !(0..=11).contains(&y) {
                    // the original shows the error box and quits the game (0x5cc7d0, 0x5a8f60, exit(-1))
                    exe::game_fatal_cleanup();
                    exe::shared_noop_ret0_5a8f60();
                    exe::crt_exit(-1);
                }
                move_to_cell(j, x, y);
                exe::cm3_match_engine_update_offside_lines(this);
                if is_past_defensive_line(this, j) {
                    exe::match_player_drop_back_one_row(j, 1);
                }
                exe::match_player_set_action_delay(j, 0);
            }
        }
    }
    let who = exe::cm3_match_engine_pick_restart_taker(this, side_b, read_i8(this + engine::RESTART_TYPE), 0);
    *b(engine::BALL_HEIGHT) = 0;
    move_to_cell(who, read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y));
    exe::match_player_take_possession(who);
    exe::match_player_set_facing_direction(who, read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y), 4, if side_b != 0 { 1 } else { 10 });
    *((who + match_player::BALL_CONTROL) as *mut u16) = 200;
}

/// who kicks off (`engine::GOALKEEPER` side 0, centre 4, 0; `engine::GOALKEEPER_SIDE1` side 1, centre 4, 11); releases the ball (`release_ball`); every
/// player on the pitch goes to his formation cell (table `engine::FORMATION_TABLE` of the direction, byte +0xa2 of the position: x in the
/// low nibble, y in the high one), the defensive lines are redone (`defensive_lines`) and, on the kicking-off side, whoever
/// crossed the line steps back one row (`exe::match_player_drop_back_one_row`); +0x2b is cleared; finally the kicker goes to the centre and takes
/// the ball (`take_ball`), +0x19c = 200, +0x19a = 180 off row 0. Without him: the original (error box). Compared with
/// `lado_a_lado` over the engine from 0x1a0, the grid, the 40 players and the 0xf56e pointers
/// Original: `line_up_kickoff` (0x6c4ec0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4ec0 } (match_eng.cpp): arruma os times para a saida de bola do lado +0x8eae. Quem sai
/// jogando: +0xf59e (lado 0, centro 4, 0) ou +0xf5a2 (lado 1, centro 4, 11; outro lado nao mexe no
/// centro). Solta a bola (0x6f0970) de quem a tem; cada jogador em campo (+0x19 >= 0) vai para a casa
/// da formacao (tabela +0xf56e do lado da direcao +0x27, byte +0xa2 da posicao: x no nibble baixo, y no
/// alto), as linhas defensivas sao refeitas (0x6c3aa0) e, no lado que sai jogando, quem nao tem a bola
/// e passou da linha (+0x8eb0 / +0x8eaf) volta uma linha (0x6f5b80 direto); +0x2b zera (0x7160e0). No
/// fim, quem sai jogando vai ao centro (+0x8ea9 = 0) e pega a bola (0x6f0630), com +0x19c = 200 e
/// +0x19a = 180 se o centro nao for a linha 0. Sem ele: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn line_up_kickoff(this: usize) {
    let side_ball = read_i8(this + engine::POSSESSION_SIDE);
    let b = |o: usize| (this + o) as *mut i8;
    let who = if side_ball == 0 { read_usize(this + engine::GOALKEEPER) } else { read_usize(this + engine::GOALKEEPER_SIDE1) };
    if who == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LINE_UP_KICKOFF)(this);
    }
    if side_ball == 0 {
        *b(engine::BALL_X) = 4;
        *b(engine::BALL_Y) = 0;
    } else if side_ball == 1 {
        *b(engine::BALL_X) = 4;
        *b(engine::BALL_Y) = 11;
    }
    if read_usize(this + engine::BALL_HOLDER) != 0 {
        exe::match_player_release_possession(read_usize(this + engine::BALL_HOLDER));
    }
    for l in 0..2i8 {
        let sai_jogando = side_ball as i32 == l as i32;
        for k in 0..20 {
            let j = side_player_slot(this, l, k);
            let pos = read_i8(j + match_player::POSITION);
            if pos < 0 {
                continue;
            }
            let table = read_usize(this + (0xf56e + 4 * (read_i8(j + match_player::SIDE) == 1) as usize));
            let home = *((table + pos as usize * 0xd8 + 0xa2) as *const u8);
            move_to_cell(j, (home & 0xf) as i8, (home >> 4) as i8);
            exe::cm3_match_engine_update_offside_lines(this);
            if is_past_defensive_line(this, j) && sai_jogando {
                exe::match_player_drop_back_one_row(j, 1);
            }
            exe::match_player_set_action_delay(j, 0);
        }
    }
    *b(engine::BALL_HEIGHT) = 0;
    move_to_cell(who, read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y));
    exe::match_player_take_possession(who);
    *((who + match_player::BALL_CONTROL) as *mut u16) = 200;
    *((who + match_player::FACING) as *mut u16) = if read_i8(this + engine::BALL_Y) != 0 { 0xb4 } else { 0 };
}

/// 0x6cc540 (ret 0x18, match_eng.cpp) with the search (`exe::cm3_match_engine_try_cell_shot_block` in mode 1) injected: does anyone intercept on the path
/// from (x, y) to the goal centre (x 4; y 11 with `gol` = 1, otherwise 0)?
///
/// The cells are walked Bresenham-style, from the starting one to the goal's; in each one it draws at random (2) the
/// side that tries first and searches both (table distance from the start, or 1000).
///
/// Returns the first non-zero code; 0 on leaving the grid or arriving. The engine's `engine::SHOT_BLOCKER` is zeroed at the start.
///
/// [pt] { 0x6cc540 } (ret 0x18, match_eng.cpp) com a procura (0x6cd3d0 em modo 1) injetada: alguem
/// intercepta no caminho de (x, y) ate o centro do gol (x 4; y 11 com `gol` = 1, senao 0)? As casas
/// saem ao estilo de Bresenham, da de saida ate a do gol; em cada uma sorteia (2) o lado que tenta
/// primeiro e procura nos dois (distancia da tabela desde a saida, ou 1000). Volta o primeiro codigo
/// diferente de zero; 0 ao sair da grade ou ao chegar. +0xf5ae do motor zera no inicio.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn find_interception_with(this: usize, x: i8, y: i8, a3: u32, goal: i8, a5: u32, a6: u32, procurar: &mut dyn FnMut(i8, usize, u32, u32, u32, u32) -> u8) -> u8 {
    *((this + engine::SHOT_BLOCKER) as *mut usize) = 0;
    let target: i8 = if goal == 1 { 11 } else { 0 };
    let dx = (4 - x as i32).abs() as i8;
    let dy = (target as i32 - y as i32).abs() as i8;
    if dx == 0 && dy == 0 {
        return 0;
    }
    // Some(code) ends (0 outside the grid); None moves on to the next cell
    let mut home = |cx: i8, cy: i8| -> Option<u8> {
        if !(0..=8).contains(&cx) || !(0..=11).contains(&cy) {
            return Some(0);
        }
        let group = (this as isize + 2 * 45 * (cy as isize + 12 * cx as isize) + 0x215e) as usize;
        let v = grid_distance_or_1000(cx as i32 - x as i32, cy as i32 - y as i32).to_bits();
        let moeda = random(2) as i8;
        for lado in [moeda, (moeda == 0) as i8] {
            let r = procurar(lado, group, v, a3, a5, a6);
            if r != 0 {
                return Some(r);
            }
        }
        None
    };
    let (mut cx, mut cy, mut erro) = (x, y, 0i8);
    if dx >= dy {
        let step_towards: i8 = if target as i32 - y as i32 >= 0 { 1 } else { -1 };
        let sobe = x < 4;
        loop {
            if erro >= dx {
                erro = erro.wrapping_sub(dx);
                cy = cy.wrapping_add(step_towards);
            }
            if let Some(r) = home(cx, cy) {
                return r;
            }
            erro = erro.wrapping_add(dy);
            cx = cx.wrapping_add(if sobe { 1 } else { -1 });
            if (sobe && cx > 4) || (!sobe && cx < 4) {
                return 0;
            }
        }
    }
    let step_towards: i8 = if 4 - x as i32 >= 0 { 1 } else { -1 };
    let sobe = y < target;
    loop {
        if erro >= dy {
            erro = erro.wrapping_sub(dy);
            cx = cx.wrapping_add(step_towards);
        }
        if let Some(r) = home(cx, cy) {
            return r;
        }
        erro = erro.wrapping_add(dx);
        cy = cy.wrapping_add(if sobe { 1 } else { -1 });
        if (sobe && cy > target) || (!sobe && cy < target) {
            return 0;
        }
    }
}

/// Does anyone intercept on the path from (x, y) to the goal centre (`gol` picks which goal)? Walks the cells
/// Bresenham-style trying both sides; returns the first non-zero code, else 0. Calls find_interception_with with the
/// game's shot-block search.
///
/// [pt] { 0x6cc540 } (ret 0x18, match_eng.cpp): ver `find_interception_with`.
pub(crate) unsafe extern "thiscall" fn intercept(this: usize, x: i8, y: i8, a3: u32, goal: i8, a5: u32, a6: u32) -> u8 {
    find_interception_with(this, x, y, a3, goal, a5, a6, &mut |lado, grupo, v, a3, a5, a6| exe::cm3_match_engine_try_cell_shot_block(this, lado, grupo, v, a3, 1, a5, a6))
}

/// `exe::cm3_match_engine_try_cell_shot_block` with the play (`exe::match_player_try_block_shot` try_block_shot) injected: walks the side's list in the cell group; mode 2 counts,
/// else each able player tries; on 1 or 2 stores him in engine `engine::SHOT_BLOCKER` and returns the code.
///
/// [pt] { 0x6cd3d0 } (ret 0x1c, match_eng.cpp) com a jogada (0x70dfd0) injetada: percorre a lista do
/// lado no grupo (+0x58 jogadores) atras de quem pode agir (+0x2b ate 1 e +0x19 nao negativo). Com
/// modo 2 so conta. Senao cada um tenta a jogada (argumentos 4, 3, 6 e 7): 1 = conseguiu (+0x12
/// sobe), 2 = outra saida; nos dois casos o jogador fica em +0xf5ae do motor e o codigo volta.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn try_side_block_with(this: usize, side: i8, group: usize, a3: u32, a4: u32, mode: u8, a6: u32, a7: u32, play: &mut dyn FnMut(usize, [u32; 4]) -> u8) -> u8 {
    let list = (group as isize + side as isize * 44) as *const usize;
    let how_many = || read_i8((group as isize + side as isize) as usize + 0x58) as i32;
    let mut n = 0u8;
    let mut k = 0;
    while k < how_many() {
        let p = *list.add(k as usize);
        if read_i8(p + match_player::BUSY) <= 1 && read_i8(p + match_player::POSITION) >= 0 {
            if mode == 2 {
                n = n.wrapping_add(1);
            } else {
                match play(p, [a4, a3, a6, a7]) as i8 {
                    1 => {
                        *((p + match_player::BALLS_CONTESTED) as *mut u8) = (*((p + match_player::BALLS_CONTESTED) as *const u8)).wrapping_add(1);
                        *((this + engine::SHOT_BLOCKER) as *mut usize) = p;
                        return 1;
                    }
                    2 => {
                        *((this + engine::SHOT_BLOCKER) as *mut usize) = p;
                        return 2;
                    }
                    _ => {}
                }
            }
        }
        k += 1;
    }
    n
}

/// Hook entry for `exe::cm3_match_engine_try_cell_shot_block` (try_side_block_with with try_block_shot `exe::match_player_try_block_shot` as the play): walks the side's players in
/// the cell group; mode 2 only counts, otherwise each able player tries to block and on 1 or 2 is stored in engine
/// `engine::SHOT_BLOCKER` and the code is returned.
///
/// [pt] { 0x6cd3d0 } (ret 0x1c, match_eng.cpp): ver `try_side_block_with`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn look_for_play(this: usize, side: i8, group: usize, a3: u32, a4: u32, mode: u8, a6: u32, a7: u32) -> u8 {
    try_side_block_with(this, side, group, a3, a4, mode, a6, a7, &mut |p, a| exe::match_player_try_block_shot(p, a[0], a[1], a[2], a[3]))
}

// Tape of calls with side effects (0x70dfd0 the move, 0x6f09f0 the repositioning, 0x6f46b0 the
// movement). In the comparison of a ported routine that calls them, each call of the original is recorded
// with the answer and the regions requested by the comparer as they ended up; the Rust copy checks the
// sequence and rewrites those regions instead of calling again.
pub(crate) type TapeCall = (usize, usize, [u32; 5], u32, crate::comparison::MemorySnapshot, crate::comparison::MemorySnapshot, (Vec<u8>, Vec<u8>)); // (routine, this, arguments, return value, before, after, (bytes of the input vectors, of the output))
#[allow(clippy::type_complexity)]
pub(crate) static CALL_TAPE: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<TapeCall>)>> = std::sync::Mutex::new(None);

/// Locks and returns the global call tape (recorded regions and calls), recovering from poisoning.
pub(crate) fn tape_lock() -> std::sync::MutexGuard<'static, Option<(Vec<(usize, usize)>, Vec<TapeCall>)>> {
    CALL_TAPE.lock().unwrap_or_else(|e| e.into_inner())
}

/// True when a recording tape is open.
///
/// [pt] Ha uma gravacao aberta (uma comparacao de fora ja grava estas chamadas)?
pub(crate) fn is_recording() -> bool {
    tape_lock().is_some()
}

/// True when a recording (tape) is open and we are at its top level, outside any recorded unit currently running.
///
/// Then the routine is called directly by the one being compared and goes into the tape (or just passes through).
/// Inside a unit, it can be compared on its own (the outer tape is kept aside).
///
/// [pt] Gravacao aberta e no nivel de cima dela (fora de qualquer unidade gravada em andamento): a
/// rotina e chamada direto pela que esta sendo comparada e entra na fita (ou so repassa). Dentro de
/// uma unidade, ela pode ser comparada por conta propria (a fita de fora fica guardada).
pub(crate) fn is_recording_top_level() -> bool {
    is_recording() && RECORDABLE_DEPTH.load(std::sync::atomic::Ordering::Relaxed) == 0
}

/// Quantas chamadas gravaveis estao em andamento: so as de fora (profundidade 0) entram na fita;
/// as que uma delas faz por dentro (`exe::match_player_try_block_shot` e `player_move` movem jogadores) ja estao na foto.
pub(crate) static RECORDABLE_DEPTH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Runs a recordable call and, at top level, pushes it on the open tape with memory snapshots before and after (random
/// draws inside paused from the trace).
///
/// [pt] Roda uma chamada gravavel e guarda-a na fita aberta, com a foto das regioes depois dela.
pub(crate) unsafe fn call_recording(routine: usize, this: usize, a: [u32; 5], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let level = RECORDABLE_DEPTH.fetch_add(1, Relaxed);
    // the state before the call: the Rust replica must reach it (what the routine wrote before
    // call, like the action at +0x198, would be erased by the later snapshot)
    let before = if level == 0 { tape_lock().as_ref().map(|(reg, _)| crate::comparison::MemorySnapshot::read(reg)) } else { None };
    // the random rolls inside the unit do not enter the original’s trace (see `pausar_rastro`)
    let r = if before.is_some() { crate::rng::pause_trace(chamada) } else { chamada() };
    RECORDABLE_DEPTH.fetch_sub(1, Relaxed);
    if let Some(before) = before {
        push_tape_call(routine, this, a, r, before, (Vec::new(), Vec::new()));
    }
    r
}

/// Pushes one call (routine, this, args, result, memory before/after, buffers) on the open tape.
pub(crate) unsafe fn push_tape_call(routine: usize, this: usize, a: [u32; 5], r: u32, before: crate::comparison::MemorySnapshot, vetores: (Vec<u8>, Vec<u8>)) {
    if let Some((reg, f)) = tape_lock().as_mut() {
        let after = crate::comparison::MemorySnapshot::read(reg);
        f.push((routine, this, a, r, before, after, vetores));
    }
}

/// Concatenates the bytes of the (address, size) buffers, skipping null addresses.
///
/// [pt] Os bytes dos vetores (endereco, tamanho), em sequencia.
pub(crate) unsafe fn read_buffers(v: &[(usize, usize)]) -> Vec<u8> {
    let mut b = Vec::new();
    for &(a, n) in v {
        if a != 0 {
            b.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
        }
    }
    b
}

/// Writes consecutive bytes back into the (address, size) buffers, skipping null addresses.
pub(crate) unsafe fn write_buffers(v: &[(usize, usize)], bytes: &[u8]) {
    let mut k = 0;
    for &(a, n) in v {
        if a != 0 && k + n <= bytes.len() {
            core::ptr::copy_nonoverlapping(bytes.as_ptr().add(k), a as *mut u8, n);
            k += n;
        }
    }
}

/// Like `call_recording`, for a unit that reads and writes buffers by pointer (on the caller's stack, outside the
/// regions): records the bytes of the `entradas` (inputs) before and of the `saidas` (outputs) after.
///
/// The Rust copy requests its own (`request_buffers`) before calling: the inputs are checked and the outputs written.
///
/// [pt] Como `call_recording`, para uma unidade que le e escreve vetores por ponteiro (na pilha de quem
/// chama, fora das regioes): grava os bytes das `entradas` antes e das `saidas` depois. A copia Rust
/// pede os seus (`request_buffers`) antes de chamar: as entradas sao conferidas e as saidas escritas.
pub(crate) unsafe fn call_recording_buffers(routine: usize, this: usize, a: [u32; 5], entries: &[(usize, usize)], outs: &[(usize, usize)], chamada: impl FnOnce() -> u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let level = RECORDABLE_DEPTH.fetch_add(1, Relaxed);
    let before = if level == 0 { tape_lock().as_ref().map(|(reg, _)| crate::comparison::MemorySnapshot::read(reg)) } else { None };
    let ent = if before.is_some() { read_buffers(entries) } else { Vec::new() };
    let r = if before.is_some() { crate::rng::pause_trace(chamada) } else { chamada() };
    RECORDABLE_DEPTH.fetch_sub(1, Relaxed);
    if let Some(before) = before {
        let sai = read_buffers(outs);
        push_tape_call(routine, this, a, r, before, (ent, sai));
    }
    r
}

/// Os vetores da proxima unidade pedida pela copia Rust: (entradas, saidas).
#[allow(clippy::type_complexity)]
pub(crate) static REQUESTED_BUFFERS: std::sync::Mutex<Option<(Vec<(usize, usize)>, Vec<(usize, usize)>)>> = std::sync::Mutex::new(None);

/// Stores the input and output buffers the Rust copy asks for before its recorded call (REQUESTED_BUFFERS).
pub(crate) fn request_buffers(entries: &[(usize, usize)], outs: &[(usize, usize)]) {
    *REQUESTED_BUFFERS.lock().unwrap_or_else(|e| e.into_inner()) = Some((entries.to_vec(), outs.to_vec()));
}

/// Runs the original routine recording the calls and the Rust copy replaying them (`reproduzir`).
///
/// Returns both responses, whether the regions came out equal and whether the call sequence matched.
///
/// What diverged in the last tape comparison (recorded call vs requested call, different regions), for the divergence
/// message. It is only built when there is a difference.
///
/// [pt] Roda a original gravando as chamadas e a copia Rust reproduzindo-as (`reproduzir`). Devolve as
/// duas respostas, se as regioes ficaram iguais e se a sequencia de chamadas bateu.
/// O que divergiu na ultima comparacao com fita (chamada gravada x pedida, regioes diferentes),
/// para a mensagem de divergencia. So e montado quando ha diferenca.
pub(crate) static TAPE_DETAIL: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// Returns a copy of the last tape comparison's detail text (TAPE_DETAIL).
pub(crate) fn tape_detail() -> String {
    TAPE_DETAIL.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

// Port of 0x70dfd0 (the move, 7 KB: narrates, rolls random numbers and changes the engine and the player).
pub(crate) static mut TRY_BLOCK_SHOT_TRAMPOLINE: usize = 0;
pub(crate) const PROLOGUE_70DFD0: Prologue = Prologue { address: 0x70dfd0, bytes: &[0x81, 0xec, 0x20, 0x02, 0x00, 0x00] };

/// Gate for MATCH_PLAYER::try_block_shot (`exe::match_player_try_block_shot`): runs its comparer, as a recorded unit when a tape is open.
pub(crate) unsafe extern "thiscall" fn gate_try_block_shot(p: usize, a1: u32, a2: u32, a3: u32, a4: u32) -> u8 {
    let rodar = || {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(TRY_BLOCK_SHOT_TRAMPOLINE);
        f(p, a1, a2, a3, a4) as u32
    };
    // the comparison of 0x70dfd0 (129th batch): its hook and this port. The comparisons of those who
    // calls (0x6cd3d0) record it as a unit: the comparison runs inside the unit (nested)
    let comparar = || compare_play(p, a1, a2, a3, a4, &rodar) as u32;
    if is_recording() {
        return call_recording(FN_MATCH_PLAYER_TRY_BLOCK_SHOT, p, [a1, a2, a3, a4, 0], comparar) as u8;
    }
    comparar() as u8
}

// Port of 0x6f09f0 (repositions the player, 13 KB, draws and changes everything around it).

/// `exe::match_player_try_block_shot` recebe os quatro argumentos inteiros.
pub(crate) const ARG_MASK_70DFD0: [u32; 5] = [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0];
/// `player_move` e 0x6f46b0 leem x e y como bytes (o resto do registro empilhado e lixo).
pub(crate) const XY_ARG_MASK: [u32; 5] = [0xff, 0xff, u32::MAX, u32::MAX, u32::MAX];

/// Computes a side's strength (word `engine::SIDE_STRENGTH` + 2 x side of the match engine, also clearing the side byte +0xaf): sums
/// each active player's influence score, -500 for duplicated formation positions, and penalties when a player with a
/// contract flag, high reputation and lucky rolls underperforms (triple for the tactics' designated player); finally
/// recomputes each active player's ratings.
///
/// [pt] { 0x6bbb60 } (ret 4, match_eng.cpp): a forca do lado (palavra +0x1ce + 2 x lado do motor,
/// recomeca do zero, assim como o byte +0xaf do lado). Para cada um dos 20 ativos: -500 quando o bit
/// da casa (0x717410, 0x5b5a60) ja apareceu (a mascara soma, nao junta); com o contrato (0xae1050:
/// 0x4cf5a0, guardado em 0xae1080; 0x4cf5f0 com o clube dele) e tres sorteios favoraveis (20 contra
/// +0x2f da ficha, 7000 contra a palavra +0xb, 6 contra +0x5b da pessoa), a palavra +0xb acima de
/// +0x80 do clube - 750 e os bits 0x17a0b do contrato, perde 3x a nota (0x6ebd70) se for o designado
/// da tatica (0x897590), senao 2x ou 1x (sorteio pelo +0x5b). Quem nao e o designado soma a nota. No
/// fim, o desempenho de cada ativo (0x6eecb0 com marca).
pub(crate) unsafe extern "thiscall" fn side_strength(this: usize, side: i8) {
    let l = side as isize;
    let strength = (this as isize + 0x1ce + 2 * l) as *mut i16;
    let sum = move |v: i32| *strength = (*strength).wrapping_add(v as i16);
    *strength = 0;
    *((this as isize + 0xaf + l) as *mut u8) = 0;
    let rating: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_CALC_INFLUENCE_SCORE);
    let rating = |j: usize| rating(j) as i8 as i32;
    let designado = || {
        let t = Fixture(*((this + engine::MATCH) as *const usize));
        let (c, fl) = if side != 0 { (t.away_club(), (t.away_side_mark() == 0) as u32) } else { (t.home_club(), ((t.home_side_mark() as i8) == 0) as u32) };
        let k = exe::tactics_find_captain_slot(*(G_TACTICS as *const usize), c, fl, 0) as i32;
        (this as isize + 0x4796 + 0x1be * (k + 20 * side as i32) as isize) as usize
    };
    let mut mask = 0u32;
    for k in 0..20usize {
        let j = side_player_slot(this, side, k);
        if read_i8(j + match_player::POSITION) < 0 {
            continue;
        }
        let bt = exe::formation_position_to_shape_bit((exe::match_player_get_position(j) as u32) as i16);
        if mask & bt != 0 {
            sum(-500);
        }
        mask = mask.wrapping_add(bt);
        let person = Staff(*((j + match_player::STAFF) as *const usize));
        let main = exe::contract_manager_get_contract(*(G_CONTRACT_MANAGER as *const usize), person);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = main;
        let club = Club(if main == 0 { 0 } else { (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((main + 4) as *const i32) as usize).wrapping_mul(581)) });
        let c = exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person, club);
        if c != 0 && !club.is_null() {
            let sheet = *((j + match_player::PLAYER) as *const usize);
            if (read_i8(sheet + player_sheet::LEADERSHIP) as i32) > random(20)
                && (*((sheet + player_sheet::CURRENT_REPUTATION) as *const i16) as i32) > random(7000)
                && (person.professionalism() as i32) < random(6)
                && *((sheet + player_sheet::CURRENT_REPUTATION) as *const i16) > (club.reputation()).wrapping_sub(750)
                && core::ptr::read_unaligned((c + contract::GRIEVANCES) as *const u32) & 0x17a0b != 0
            {
                if j == designado() {
                    sum(-3 * rating(j));
                } else if random(person.professionalism() as i32) == 0 {
                    sum(-2 * rating(j));
                } else {
                    sum(-rating(j));
                }
            }
        }
        if j != designado() {
            sum(rating(j));
        }
    }
    for k in 0..20usize {
        let j = side_player_slot(this, side, k);
        if read_i8(j + match_player::POSITION) >= 0 {
            exe::match_player_calc_ratings(j, 1);
        }
    }
}

/// returns the node's +0 and advances to its +6
/// Original: `next_list_item` (0x68d1b0, cursor at +0xa (unaligned), thiscall).
///
/// [pt] { 0x68d1b0 }: o proximo item da lista (cursor em +0xa: devolve o +0 do no e avanca para o +6), ou 0.
pub(crate) unsafe extern "thiscall" fn next_list_item(this: usize) -> u32 {
    let no = core::ptr::read_unaligned((this + 0xa) as *const usize);
    if no == 0 {
        return 0;
    }
    core::ptr::write_unaligned((this + 0xa) as *mut usize, core::ptr::read_unaligned((no + 6) as *const usize));
    core::ptr::read_unaligned(no as *const u32)
}

/// Returns the signed word +0xc of the person `id`'s 31-byte injury record in the array [this]; 0 when the id is beyond
/// the person count (0xae2364).
///
/// [pt] { 0x62c3d0 } (ret 4, injury.cpp): a palavra com sinal +0xc do registro de 31 bytes da pessoa `id`
/// em [this]; id a partir de 0xae2364: 0.
pub(crate) unsafe extern "thiscall" fn word_0c(this: usize, id: i32) -> i32 {
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    core::ptr::read_unaligned((*(this as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *const i16) as i32
}

/// Returns `v` with the units digit of its low word removed (v - w % 10) when that signed word is below 1000, else `v`
/// unchanged: the round code without its variant.
///
/// [pt] { 0x4b0870 } (comp_util.cpp): `v` menos o resto por 10 da palavra baixa, quando ela (com sinal)
/// e menor que 1000.
pub(crate) unsafe extern "C" fn round_below_1000(v: i32) -> i32 {
    let w = v as i16;
    v.wrapping_sub(if w < 1000 { w as i32 % 10 } else { 0 })
}

/// Is the nation (id at +0) one of the six special pseudo-nations used for club competitions (B internationals,
/// reserves, under-21 internationals, Spanish lower division, French CFA, French lower division)? Returns 1 or 0.
///
/// [pt] { 0x7e6f20 } (stdcall): a nacao (id em +0) e uma das seis de 0x9cf794, 0x9cf684, 0x9cf798,
/// 0x9cf6b0, 0x9cf5d8 e 0x9cf548?
pub(crate) unsafe extern "stdcall" fn group_nation(nation: usize) -> u32 {
    if nation == 0 {
        return 0;
    }
    let id = *(nation as *const u32);
    [KEY_CLUB_COMP_B_INTERNATIONAL, KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&a| *(a as *const u32) == id) as u32
}

/// Counts how many active players of the opposing side man-mark `alvo` (`designated_marked`).
///
/// [pt] { 0x6d59b0 } (ret 4): quantos dos 20 jogadores do outro lado (ativos, +0x19 >= 0) marcam
/// `alvo` individualmente (0x7160f0).
pub(crate) unsafe extern "thiscall" fn how_many_mark(this: usize, target: usize) -> u8 {
    let other = (side_of_player(target) == 0) as i8;
    (0..20).filter(|&k| {
        let j = side_player_slot(this, other, k);
        *((j + match_player::POSITION) as *const i8) >= 0 && exe::match_player_find_man_marking_target(j) == target
    }).count() as u8
}

/// Returns the first active player of the opposing side who man-marks `alvo`, or 0.
///
/// [pt] { 0x6d5a10 } (ret 4): o primeiro dos 20 jogadores do outro lado (ativos) que marca `alvo`, ou 0.
pub(crate) unsafe extern "thiscall" fn who_marks(this: usize, target: usize) -> usize {
    let other = (side_of_player(target) == 0) as i8;
    for k in 0..20 {
        let j = side_player_slot(this, other, k);
        if *((j + match_player::POSITION) as *const i8) >= 0 && exe::match_player_find_man_marking_target(j) == target {
            return j;
        }
    }
    0
}

/// Smallest table distance from an active player of a side (excluding position bit 1) to (x, y), starting at 5000; 0 if
/// one is on it.
///
/// [pt] Menor distancia (tabela) de um jogador ativo do bloco `lado` (fora os de `table_a` com o bit
/// 1) ao ponto (x, y), comecando de 5000; 0 se algum esta a distancia 0.
pub(crate) unsafe fn nearest_active_distance(m: usize, side: usize, x: i8, y: i8) -> f32 {
    let mut smaller = 5000.0f32;
    for k in 0..20 {
        let q = m + engine::PLAYERS + 8920 * side + 0x1be * k;
        if read_i8(q + match_player::POSITION) < 0 || player_table(q) & 1 != 0 {
            continue;
        }
        let d = distance_to_point(q, x, y);
        if d as f64 == read_f64(rdata::F64_0) || d.is_nan() {
            return 0.0;
        }
        if d < smaller || d.is_nan() {
            smaller = d;
        }
    }
    smaller
}

/// Chooses the teammate and the kind of ball (pass, cross, long ball, one-two...) for the ball holder (`engine::BALL_HOLDER`) from
/// cell (bx, by): scores each active teammate on space, formation role, offside line (`engine::OFFSIDE_LINE`/`engine::OFFSIDE_LINE_SIDE1`), pressure,
/// tactical instructions, the manager's playmaker and personal likes/dislikes, then picks the ball type by distance (or
/// tests `pedido`), keeping the best score above `*melhor`. Writes the type, whether the target moves and the target
/// cell; returns the teammate or 0.
///
/// [pt] { 0x6c6a30 } (match_eng.cpp): escolhe o companheiro e a jogada (passe, cruzamento, lancamento,
/// tabela...) para quem esta com a bola (0xf582), a partir da casa (bx, by). Para cada colega ativo
/// soma notas pelo espaco (adversario mais proximo dele e do ponto para onde ele pode se deslocar,
/// 0x6bce80), pelo papel na formacao, pela posicao em relacao a linha de impedimento (0x8eaf/0x8eb0),
/// pelo cerco, pelas marcas taticas (0x7163c0), pela preferencia do tecnico (0x897dc0) e por
/// gostos pessoais (0x547840/0x547810); depois escolhe o tipo de jogada pela distancia ate o alvo
/// (ou testa o tipo `pedido`) e fica com a maior nota acima de `*melhor`. Escreve o tipo, se o alvo
/// se desloca e a casa alvo; devolve o colega (ou 0).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn choose_teammate(
    m: usize,
    bx: i8,
    by: i8,
    kind_out: *mut i8,
    moveu_out: *mut u8,
    x_out: *mut i8,
    y_out: *mut i8,
    best: *mut i32,
    request: i8,
) -> usize {
    type F1 = unsafe extern "thiscall" fn(usize) -> u8;
    let t = |a: usize| core::mem::transmute::<usize, F1>(a);

    let mut res = 0usize;
    *((m + engine::RESTART_TYPE) as *mut u8) = 0;
    let c = *((m + engine::BALL_HOLDER) as *const usize);
    let mut factor_nation = f32::from_bits(0x3e4ccccd);
    let nation = *((*((c + match_player::STAFF) as *const usize) + 0x1a) as *const usize);
    if nation != 0 {
        let id = *(nation as *const u32);
        let g = |a: usize| *(a as *const u32) == id;
        if g(KEY_NATION_ENGLAND) || g(KEY_NATION_SCOTLAND) || g(KEY_NATION_WALES) || g(KEY_NATION_NORTHERN_IRELAND) || g(KEY_NATION_REPUBLIC_OF_IRELAND) || g(KEY_NATION_NORWAY) {
            factor_nation = 1.0;
        } else if g(KEY_NATION_SWEDEN) || g(KEY_NATION_DENMARK) || g(KEY_NATION_GERMANY) {
            factor_nation = f32::from_bits(0x3f19999a);
        }
    }
    let r1 = random(500);
    let base = (*((c + 0x29) as *const i16) as i32 - 1000) * 3 - r1;
    *best = base.wrapping_add(random(500));
    if t(0x70dfa0)(c) == 0 && closest_to_goal(m, c) == 1 && t(0x6f4650)(c) != 0 {
        return 0;
    }
    let ls = read_i8(m + engine::POSSESSION_SIDE);
    let in_the_box = |x: i8, y: i8, s: i8| (2..=6).contains(&x) && if s == 0 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
    let area_atacada = in_the_box(bx, by, ls); // [0x17]
    let area_defendida = in_the_box(bx, by, if ls == 1 { 0 } else { 1 }); // [0x16]
    let adv = (ls == 0) as usize;
    let d_ball = nearest_active_distance(m, adv, read_i8(c + match_player::CELL_X), read_i8(c + match_player::CELL_Y));
    let role_of = |q: usize| -> u16 {
        let pos = read_i8(q + match_player::POSITION);
        if (0..=10).contains(&pos) {
            *((*((m + engine::POSITION_TABLE + 4 * side_of_player(q) as usize) as *const usize) + 2 * pos as usize) as *const u16)
        } else {
            0
        }
    };
    let pf = |x: i32| ftol(x as f64);
    let _ = pf;
    for k in 0..20usize {
        let p = m + engine::PLAYERS + 8920 * ls as usize + 0x1be * k;
        if read_i8(p + match_player::POSITION) < 0 || p == c {
            continue;
        }
        if role_of(c) == 1 && ((ls == 0 && by > read_i8(p + match_player::CELL_Y)) || (ls == 1 && by < read_i8(p + match_player::CELL_Y))) {
            continue;
        }
        let (px, py) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
        let mut d58 = nearest_active_distance(m, adv, px, py);
        let mut s74 = 0u8;
        if request != 0x36 && request != 0x37 && exe::match_player_is_cell_in_view(c, px, py, 1, d_ball, d58, &mut s74) == 0 {
            continue;
        }
        let role = role_of(p);
        let mut v34 = ftol(read_f32(rdata::F32_5000) as f64 - d58 as f64 * d58 as f64 * read_f32(rdata::F32_NEG_50) as f64);
        if t(0x70df40)(p) != 0 {
            v34 = v34.wrapping_sub(1500);
        } else if t(0x70dfa0)(p) != 0 {
            let bottom = if ls == 0 { py < 3 } else { py > 8 };
            v34 = v34.wrapping_add(if bottom { 10000 } else { 3000 });
        }
        let mut v1c: i32 = if role == 1 {
            0
        } else {
            ftol((read_f32(p + match_player::STAMINA) as f64 + *((p + match_player::MATCH_RATING) as *const i16) as f64 - read_f32(rdata::F32_15000) as f64) * read_f32(rdata::F32_0_25) as f64)
        };
        if t(0x70dfa0)(p) == 0 {
            let mp = closest_to_goal(m, p);
            let step_towards = match mp {
                1 => 2000,
                2 => 4000,
                _ => 0,
            };
            if step_towards != 0 {
                v34 = v34.wrapping_sub(step_towards);
                v1c = v1c.wrapping_add(step_towards);
                if (exe::match_player_get_instructions(c) >> 32) & 1 != 0 {
                    v1c = v1c.wrapping_add(step_towards);
                    v34 = v34.wrapping_sub(step_towards);
                }
            } else if (exe::match_player_get_instructions(c) >> 32) & 1 != 0 {
                v1c = v1c.wrapping_add(500);
                v34 = v34.wrapping_sub(500);
            }
        }
        let partida = Fixture(*((m + engine::MATCH) as *const usize));
        let (mut v44, mut v4c, mut v50) = (0i32, 0i32, 0i32);
        let comp = Competition(partida.competition());
        let mut edi: i32 = if !comp.is_null() { (comp.reputation() as i32 - 12) * 25 } else { 0 };
        let mut v30 = edi.wrapping_mul(-4);
        let mut v38 = 0i32;
        let lp = side_of_player(p);
        let (marca_p, clube_p) = if lp != 0 {
            (partida.away_side_mark(), partida.away_club())
        } else {
            ((partida.home_side_mark() as i8), partida.home_club())
        };
        let table = *(G_TACTICS as *const usize);
        let sheet_p = *((p + match_player::STAFF) as *const usize);
        if sheet_p == exe::tactics_get_playmaker(table, clube_p, 0, (marca_p == 0) as u32) {
            v1c = v1c.wrapping_add(1000);
        } else if sheet_p == exe::tactics_get_playmaker(table, clube_p, 1, (marca_p == 0) as u32) {
            let pl0 = exe::tactics_get_playmaker(table, clube_p, 0, (marca_p == 0) as u32);
            let q = exe::match_man_find_match_player(m + engine::MATCH_MANAGERS + 208 * side_of_player(c) as usize, pl0);
            if q == 0 || read_i8(q + match_player::POSITION) < 0 {
                v1c = v1c.wrapping_add(1000);
            }
        } else {
            let a = read_f32(p + match_player::VISION) as f64;
            let r = ftol(a * a * read_f64(rdata::F64_NEG_5));
            edi = edi.wrapping_sub(r);
            v38 = r.wrapping_neg();
        }
        if ((exe::match_player_is_in_penalty_area(p, (lp == 0) as u8) as u32) as u8) != 0 {
            let b = ftol(read_f32(p + match_player::TECHNIQUE) as f64 * read_f32(p + match_player::PACE) as f64 * read_f64(rdata::F64_NEG_5));
            let a = read_f32(p + match_player::FINISHING) as f64;
            let d = ftol(a * a * read_f64(rdata::F64_5)).wrapping_sub(b);
            edi = edi.wrapping_add(d);
            v38 = v38.wrapping_add(d);
        }
        if t(0x70df40)(c) == 0 {
            let mc = exe::match_player_get_instructions(c);
            let level = if mc & 1 != 0 {
                let f = *((m + 6371 * ls as usize + 0x9766) as *const u32);
                if f & 8 != 0 { 3 } else if f & 4 != 0 { 2 } else if f & 2 != 0 { 1 } else { 0 }
            } else if exe::match_player_get_instructions(c) & 0x10 != 0 {
                3
            } else if exe::match_player_get_instructions(c) & 8 != 0 {
                2
            } else if exe::match_player_get_instructions(c) & 4 != 0 {
                1
            } else {
                0
            };
            match level {
                3 => {
                    edi = edi.wrapping_sub(1000);
                    v30 = v30.wrapping_add(1000);
                }
                2 => {
                    edi = edi.wrapping_sub(750);
                    v30 = v30.wrapping_add(500);
                }
                1 => {
                    edi = edi.wrapping_add(250);
                    v30 = v30.wrapping_sub(250);
                }
                _ => {}
            }
        }
        let mut v3c = edi;
        // target offset
        let (mut tx, mut ty, mut moveu) = (px, py, 0u8);
        if role != 1 {
            let prod = read_i8(p + match_player::MOMENT_ANTICIPATION) as i32 * read_i8(p + match_player::MOMENT_PACE) as i32 * read_i8(c + match_player::MOMENT_VISION) as i32 * 38;
            let lim = prod as f64 * read_f64(rdata::F64_0_1);
            let r = random(v34.wrapping_mul(6));
            let (mut x78, mut y77) = (0i8, 0i8);
            if !((r as f64) > lim) && exe::cm3_match_engine_choose_move_cell(m, p, &mut x78, &mut y77, 0) != 0 {
                x78 = x78.clamp(0, 8);
                y77 = y77.clamp(0, 11);
                if !(x78 == px && y77 == py) {
                    tx = x78;
                    ty = y77;
                    moveu = 1;
                    d58 = nearest_active_distance(m, adv, tx, ty);
                    if player_table(p) & 0x40 != 0 {
                        v1c = v1c.wrapping_add(read_i8(p + match_player::MOMENT_PACE) as i32 * read_i8(p + match_player::MOMENT_ANTICIPATION) as i32 * 50);
                    }
                }
            }
        }
        // offside: cells beyond the line (0x8eaf/0x8eb0)
        let mut k7c: i8 = if lp == 1 { read_i8(m + engine::OFFSIDE_LINE_SIDE1).wrapping_sub(py) } else { py.wrapping_sub(read_i8(m + engine::OFFSIDE_LINE)) }.wrapping_add(1);
        if k7c > -2
            && *((m + 6371 * (lp == 0) as usize + 0x9766) as *const u32) & 0x400 != 0
            && read_i8(c + match_player::MOMENT_DECISIONS) != 0
        {
            let b = read_i8(p + match_player::MOMENT_MOVEMENT) as i32;
            if b < random(0x32) {
                k7c = k7c.wrapping_add(1);
            }
        }
        if request == 0x36 {
            k7c = -4;
        } else if k7c >= 0 {
            k7c = k7c.wrapping_mul(k7c).wrapping_add(1);
            let q = k7c as i32;
            let b = read_i8(p + match_player::MOMENT_MOVEMENT) as i32;
            if b < random((q * 5 + 5) * 2) {
                v1c = v1c.wrapping_sub((read_i8(c + match_player::MOMENT_VISION) as i32 * 400 + 500).wrapping_mul(q).wrapping_mul(q));
            }
        }
        let target_area = in_the_box(tx, ty, ls); // [0x11]
        let target_area_def = in_the_box(tx, ty, if ls == 1 { 0 } else { 1 }); // [0x12]
        let (by_i, ty_i) = (by as i32, ty as i32);
        let d60 = by_i - ty_i;
        let d68 = grid_distance_or_1000(bx as i32 - tx as i32, d60);
        // closing in on the target
        let mut n = 0i8;
        for kk in 0..20 {
            let q = m + engine::PLAYERS + 8920 * adv + 0x1be * kk;
            if read_i8(q + match_player::POSITION) < 0 || player_table(q) == 1 {
                continue;
            }
            let d = distance_to_point(q, tx, ty) as f64;
            if d == read_f64(rdata::F64_0) || d.is_nan() {
                n = n.wrapping_add(3);
            } else if !(d > read_f64(rdata::F64_1_5)) {
                n = n.wrapping_add(1);
            }
        }
        let n_i = n as i32;
        v1c = v1c.wrapping_add((5 - n_i) * 400);
        if target_area && n < 3 && k7c < 1 {
            v1c = v1c.wrapping_add(ftol(game_pow((3 - n_i) as f64, read_f64(rdata::F64_3)) * read_f64(rdata::F64_100)));
        }
        let mut s72 = 0u16;
        if read_i8(c + match_player::PREFERRED_FOOT) != exe::match_player_choose_kicking_foot(c, 0x5d, bx, by, tx, ty, d68, &mut s72) as i8 {
            v1c = v1c.wrapping_sub(random(read_i8(c + 0x2d) as i32 * 400));
        }
        if request == 0x34 || request == 0x37 {
            let sum = |extra: f32| {
                read_f32(p + match_player::FLAIR) as f64 * read_f32(rdata::F32_100) as f64 + read_f32(p + match_player::FINISHING) as f64 * read_f32(rdata::F32_50) as f64
                    + read_f32(p + match_player::CROSSING) as f64 * read_f32(rdata::F32_50) as f64
                    + extra as f64
            };
            let val = if target_area {
                Some(read_f32(rdata::F32_4000) as f64 - exe::match_player_distance_to_opponent_goal(p) as f64 * read_f32(rdata::F32_1000) as f64 + read_f32(p + match_player::JUMPING) as f64 * read_f32(rdata::F32_100) as f64)
            } else if (d68 as f64) < read_f64(rdata::F64_3) {
                if n == 0 {
                    Some(sum(read_f32(rdata::F32_3000)))
                } else if (n_i as f64) < read_f64(rdata::F64_3) {
                    Some(sum(read_f32(rdata::F32_2000)))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(v) = val {
                v1c = v1c.wrapping_add(ftol(v));
            }
        }
        if !(d58 as f64 <= read_f64(rdata::F64_1_5) || d58.is_nan()) && !target_area {
            let e = read_i8(c + match_player::MOMENT_DECISIONS) as i32 + 5;
            v30 = v30.wrapping_add(ftol((-700 * e) as f64 / factor_nation as f64));
            v44 = ftol((-500 * e) as f64 / factor_nation as f64);
        } else if moveu == 0 {
            let x = (30 - read_i8(c + match_player::MOMENT_VISION) as i32) as f64 * (read_f32(rdata::F32_10) as f64 - read_f32(p + match_player::JUMPING) as f64) * factor_nation as f64;
            v30 = v30.wrapping_add(ftol(x * read_f32(rdata::F32_NEG_40) as f64));
            v44 = ftol(x * read_f32(rdata::F32_NEG_20) as f64);
        }
        if role == 1 {
            if t(0x70dfa0)(c) == 0 && random(100000) != 0 {
                continue;
            }
            let v = read_i8(c + match_player::MOMENT_DECISIONS) as i32;
            if t(0x70def0)(c) == 0 {
                v1c = v1c.wrapping_add(ftol((-1000 * v) as f64 * (d58 as f64 + read_f64(rdata::F64_0_1))));
            }
            v44 = v44.wrapping_add(-1000 - 2200 * v);
            v30 = v30.wrapping_add(-2000 - 5000 * v);
            v38 = v38.wrapping_add(-1500 * v);
        }
        if role_of(c) == 1 {
            v1c = v1c.wrapping_sub(1000);
        }
        if area_defendida {
            if (d58 as f64) < read_f64(rdata::F64_2) && player_table(c) == 1 && t(0x70dfa0)(p) != 0 {
                v1c = v1c.wrapping_sub(2000);
            }
            if target_area_def {
                v1c = v1c.wrapping_add(ftol(read_i8(c + match_player::MOMENT_DECISIONS) as f64 * d58 as f64 * d58 as f64 * read_f32(rdata::F32_NEG_20000) as f64));
            }
        }
        if target_area_def {
            v1c = v1c.wrapping_sub(ftol((read_i8(c + match_player::MOMENT_DECISIONS) as i32 * 32 + 200) as f64 * d58 as f64));
        }
        let alcanca_r = exe::can_run_into_central_box(c, tx, ty);
        if alcanca_r != 0 {
            let v = read_i8(c + match_player::MOMENT_DECISIONS) as f64;
            let a = read_f32(p + match_player::JUMPING) as f64 * read_f32(rdata::F32_100) as f64;
            let b = read_f32(c + match_player::CROSSING) as f64 * read_f32(rdata::F32_100) as f64;
            v4c = ftol((a + read_f32(rdata::F32_500) as f64) * v * read_f32(rdata::F32_0_1000000015) as f64 + b);
            v50 = ftol((read_f32(rdata::F32_3000) as f64 - a) * v * read_f32(rdata::F32_0_1000000015) as f64 + b);
            if (exe::match_player_get_instructions(c) >> 32) & 4 != 0 {
                let w = ftol(read_f32(c + match_player::TEAMWORK) as f64 * read_f32(rdata::F32_100) as f64 + read_f32(rdata::F32_500) as f64);
                v4c = v4c.wrapping_add(w);
                v50 = v50.wrapping_add(w);
            }
            v30 = v30.wrapping_sub(2000);
            v38 = v38.wrapping_sub(2000);
            v3c = v3c.wrapping_sub(2000);
        }
        if exe::match_player_get_instructions(c) & 0x40 != 0 && ((ls == 0 && px <= 2) || (ls == 1 && px >= 6)) {
            v1c = v1c.wrapping_add(1000);
        }
        if exe::match_player_get_instructions(c) & 0x100 != 0 && ((ls == 0 && px >= 6) || (ls == 1 && px <= 2)) {
            v1c = v1c.wrapping_add(1000);
        }
        if exe::match_player_get_instructions(c) & 0x80 != 0 && t(0x70df20)(p) == 0 && alcanca_r == 0 {
            v1c = v1c.wrapping_add(1000);
        }
        // proximity to the area (the original’s branches at 0x6c7c95..0x6c7d5f)
        {
            let bottom = |y: i8| if ls == 1 { y < 3 } else if ls == 0 { y > 8 } else { false };
            let minus = |v1c: &mut i32, d58: &mut f32, q: i32| {
                *d58 = 0.0;
                *v1c = v1c.wrapping_sub(q);
            };
            if area_atacada && !target_area {
                minus(&mut v1c, &mut d58, 4000);
            } else if !area_atacada && target_area {
                d58 = (d58 as f64 * d58 as f64 + read_f64(rdata::F64_1)) as f32;
            } else {
                #[derive(Clone, Copy)]
                enum L {
                    Cf3,
                    Cce,
                    Cec,
                    Cfc,
                    D20,
                    D25,
                    D43,
                    D4a,
                    Fim,
                }
                let mut l = if bx <= 2 {
                    L::Cf3
                } else if area_atacada {
                    L::Cce
                } else if bottom(by) {
                    if bx < 6 { L::Cfc } else { L::Cce }
                } else {
                    L::Cf3
                };
                loop {
                    l = match l {
                        L::Cce => if bottom(ty) { L::Cec } else { L::Cf3 },
                        L::Cec => if tx < 2 { L::D4a } else { L::Cf3 },
                        L::Cf3 => {
                            if bx >= 6 {
                                L::Fim
                            } else if area_atacada {
                                L::D25
                            } else {
                                L::Cfc
                            }
                        }
                        L::Cfc => if bottom(by) { L::D20 } else { L::Fim },
                        L::D20 => if bx > 2 { L::Fim } else { L::D25 },
                        L::D25 => if bottom(ty) { L::D43 } else { L::Fim },
                        L::D43 => if tx <= 6 { L::Fim } else { L::D4a },
                        L::D4a => {
                            minus(&mut v1c, &mut d58, 2000);
                            L::Fim
                        }
                        L::Fim => break,
                    };
                }
            }
        }
        if ((ls == 0 && by >= ty) || (ls == 1 && by <= ty)) && t(0x70dfa0)(p) != 0 {
            v30 = v30.wrapping_sub(5000);
            v38 = v38.wrapping_sub(3000);
            v44 = v44.wrapping_sub(3000);
        }
        let zera = |v1c: &mut i32, d58: &mut f32, v: f64, k: f32| {
            *d58 = v as f32;
            if v < read_f64(rdata::F64_0) || v.is_nan() {
                *v1c = v1c.wrapping_add(ftol(*d58 as f64 * *d58 as f64 * k as f64));
                *d58 = 0.0;
            }
        };
        if ls == 0 && by > ty {
            let v = d58 as f64 - (d60 as f64 - read_f64(rdata::F64_0_5));
            zera(&mut v1c, &mut d58, v, read_f32(rdata::F32_NEG_500));
        } else if ls == 1 && by < ty {
            let v = d58 as f64 - ((ty_i - by_i) as f64 - read_f64(rdata::F64_0_5));
            zera(&mut v1c, &mut d58, v, read_f32(rdata::F32_NEG_500));
        } else {
            let mut cnt = 0i8;
            for kk in 0..20 {
                let q = m + engine::PLAYERS + 8920 * ls as usize + 0x1be * kk;
                if read_i8(q + match_player::POSITION) >= 0 && ((ls == 1 && read_i8(q + match_player::CELL_Y) < ty) || (ls == 0 && read_i8(q + match_player::CELL_Y) > ty)) {
                    cnt = cnt.wrapping_add(1);
                }
            }
            if cnt == 0 {
                d58 = (d58 as f64 + d58 as f64 + read_f64(rdata::F64_1)) as f32;
            } else if !(2..=6).contains(&tx) && t(0x70dfa0)(p) == 0 && (exe::match_player_get_instructions(p) >> 32) & 4 != 0 {
                d58 = (d58 as f64 * read_f64(rdata::F64_1_5) + read_f64(rdata::F64_1)) as f32;
            }
        }
        if k7c >= 0 {
            let v = d58 as f64 - k7c as f64;
            zera(&mut v1c, &mut d58, v, read_f32(rdata::F32_NEG_200));
        }
        v1c = v1c.wrapping_add(ftol(d58 as f64 * d58 as f64 * read_f32(rdata::F32_100) as f64));
        let near = (ls == 0 && by >= ty) || (ls == 1 && by <= ty);
        let mut c2 = true;
        if near && (d58 as f64) < read_f64(rdata::F64_3_1) && !(if ls == 1 { ty < 3 } else if ls == 0 { ty > 8 } else { false })
            && !(read_i8(c + match_player::MOMENT_DECISIONS) == 0 && read_i8(c + match_player::MOMENT_VISION) == 0)
        {
            c2 = false;
            let s: i16 = if t(0x70def0)(p) != 0 {
                ftol(read_f32(rdata::F32_1500) as f64 - d58 as f64 * read_f32(rdata::F32_375) as f64) as i16
            } else {
                ftol(read_f32(rdata::F32_1000) as f64 - d58 as f64 * read_f32(rdata::F32_250) as f64) as i16
            };
            if ls == 0 {
                v1c = v1c.wrapping_sub((d60 + 1).wrapping_mul(s as i32));
            } else if ls == 1 {
                v1c = v1c.wrapping_sub((ty_i - by_i + 1).wrapping_mul(s as i32));
            }
        }
        if c2 {
            if d58 < read_f32(rdata::F32_2) && t(0x70dfa0)(p) != 0 {
                v1c = v1c.wrapping_sub(1500);
            } else if role != 1 {
                let v10 = read_i8(c + match_player::MOMENT_VISION) as i32 + 10;
                v1c = v1c.wrapping_add(ftol(v10 as f64 * d58 as f64 * d58 as f64));
                let bottom_t = if ls == 1 { ty < 3 } else if ls == 0 { ty > 8 } else { false };
                if bottom_t {
                    if exe::cm3_match_engine_has_free_cell_ahead(m, p) != 0 {
                        v1c = v1c.wrapping_add(v10 * 20);
                    } else if exe::cm3_match_engine_is_path_ahead_clear(m, p, 0) != 0 {
                        v1c = v1c.wrapping_add(v10 * 50);
                    }
                } else if t(0x70dfa0)(p) == 0 {
                    if exe::cm3_match_engine_has_free_cell_ahead(m, p) != 0 {
                        v1c = v1c.wrapping_add(80 + read_i8(c + match_player::MOMENT_VISION) as i32 * 8);
                    } else if exe::cm3_match_engine_is_path_ahead_clear(m, p, 0) != 0 {
                        v1c = v1c.wrapping_add(v10 * 20);
                    }
                }
                if (ls == 0 && ty > by) || (ls == 1 && ty < by) {
                    v1c = v1c.wrapping_add(ftol(v10 as f64 * d58 as f64 * d58 as f64));
                }
            }
        }
        let g = read_i8(p + match_player::BUSY) as i32;
        if g > 1 {
            v1c = v1c.wrapping_add(-50 * (read_i8(c + match_player::MOMENT_VISION) as i32 + 4) * g * g);
        }
        let s74r = random(s74 as i8 as i32) as i8;
        let q = s74r as i32 * s74r as i32;
        v44 = v44.wrapping_sub(155 * q);
        let v3c2 = v3c.wrapping_sub(500 * q);
        v38 = v38.wrapping_sub(155 * q);
        let v30b = v30.wrapping_sub(25 * q);
        let mut base = v1c;
        if random(read_i8(*((c + match_player::STAFF) as *const usize) + 0x5b) as i32 + 1) == 0 {
            let (fp, fc) = (*((p + match_player::STAFF) as *const usize), *((c + match_player::STAFF) as *const usize));
            if exe::staff_is_disliked_by(fp, fc) != 0 {
                continue;
            }
            if exe::staff_is_favoured_by(fp, fc) != 0 {
                base = base.wrapping_add(1000);
            }
        }
        // move type and final rating
        let mut kind: i8 = -1;
        let mut rating: i32 = -100000;
        let lim = read_f32(rdata::F32_5) as f64 - (read_f32(rdata::F32_21) as f64 - read_f32(c + match_player::STRENGTH) as f64) * read_f64(rdata::F64_0_1666666667);
        let d = d68 as f64;
        let entre = |hi: f64, lo: f64| !(d > hi || d.is_nan()) && !(d < lo);
        if request < 0 {
            let pa = read_i8(m + engine::BALL_HEIGHT);
            if pa == 4 {
                if d > read_f32(rdata::F32_2) as f64 {
                    continue;
                }
                kind = 0x3d;
                rating = base;
            } else {
                if !(lim < d) && !(d < read_f32(rdata::F32_4) as f64) {
                    rating = v30b;
                    kind = 0x1e;
                }
                if alcanca_r != 0 {
                    if entre(read_f32(rdata::F32_5) as f64, read_f32(rdata::F32_1) as f64) && v4c > rating {
                        rating = v4c;
                        kind = 8;
                    }
                    if entre(read_f32(rdata::F32_5) as f64, read_f32(rdata::F32_1) as f64) && v50 > rating {
                        rating = v50;
                        kind = 7;
                    }
                }
                if !(d > read_f32(rdata::F32_2) as f64 || d.is_nan()) && v3c2 > rating && pa < 3 {
                    rating = v3c2;
                    kind = 0xb;
                }
                if entre(read_f32(rdata::F32_4) as f64, read_f32(rdata::F32_3) as f64) && v38 > rating {
                    rating = v38;
                    kind = 0x3c;
                }
                if entre(read_f32(rdata::F32_3) as f64, read_f64(rdata::F64_1_5)) && v44 > rating {
                    rating = v44;
                    kind = 0x1f;
                }
                rating = rating.wrapping_add(base);
            }
        } else {
            kind = request;
            let away = -100000i32;
            rating = match request {
                6 | 8 => if entre(read_f32(rdata::F32_5) as f64, read_f32(rdata::F32_1) as f64) { v4c.wrapping_add(base) } else { away },
                7 => if entre(read_f32(rdata::F32_5) as f64, read_f32(rdata::F32_1) as f64) { v50.wrapping_add(base) } else { away },
                0xb => if d > read_f32(rdata::F32_2) as f64 || d.is_nan() { away } else { base.wrapping_add(v3c2) },
                0x1e | 0x5a => if lim < d || d < read_f32(rdata::F32_4) as f64 { away } else { v30b.wrapping_add(base) },
                0x1f => if entre(read_f32(rdata::F32_3) as f64, read_f64(rdata::F64_1_5)) { v44.wrapping_add(base) } else { away },
                0x34 => {
                    if entre(read_f64(rdata::F64_5_5), read_f64(rdata::F64_3)) && tx > 1 && tx < 7 && !(3..=8).contains(&ty) {
                        v38.wrapping_add(base)
                    } else {
                        base.wrapping_add(v3c2)
                    }
                }
                0x36 => {
                    if (read_f32(c + match_player::THROW_INS) as f64 * read_f64(rdata::F64_0_1) + read_f32(rdata::F32_1) as f64) < d {
                        away
                    } else {
                        base
                    }
                }
                0x37 => if lim < d || d < read_f32(rdata::F32_4) as f64 { base.wrapping_add(v3c2) } else { v30b.wrapping_add(base) },
                0x3c => if entre(read_f32(rdata::F32_4) as f64, read_f32(rdata::F32_3) as f64) { v38.wrapping_add(base) } else { away },
                0x3d => if d > read_f32(rdata::F32_2) as f64 || d.is_nan() { away } else { base },
                _ => base,
            };
        }
        if kind >= 0 && rating > *best {
            *best = rating;
            *kind_out = kind;
            res = p;
            *moveu_out = moveu;
            *x_out = tx;
            *y_out = ty;
        }
    }
    res
}

/// the list of active players (+0x19 ≥ 0) at `engine::ACTIVE_LIST`, count at `engine::ACTIVE_LIST_COUNT`. For each slot 0..20 draws (`exe::utils_random`(2)) which
/// side goes first. Sorts with the CRT `qsort` (`exe::crt_qsort`) and the game's own compare (`action_order`, which reads the engine
/// at 0xb67a64)
/// Original: `player_order` (0x6bd760, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bd760 } (match_eng.cpp): a lista dos jogadores ativos (+0x19 >= 0) em +0x8f7e (contagem em
/// +0x8eb8): para cada vaga 0..20 sorteia qual lado entra primeiro (0x90cfc0(2)) e poe o daquele lado
/// e depois o do outro. Ordena com o `qsort` da CRT (0x945333) e a comparacao do proprio jogo
/// (0x6bd840, que le o motor em 0xb67a64).
pub(crate) unsafe extern "thiscall" fn player_order(m: usize) {
    let n = (m + engine::ACTIVE_LIST_COUNT) as *mut i8;
    *n = 0;
    let list = (m + engine::ACTIVE_LIST) as *mut usize;
    for i in 0..20usize {
        let first = random(2) as i8;
        for lado in [(first as isize) as usize, (first == 0) as usize] {
            let p = (m as isize + 0x4796 + 0x1be * (i as isize + 20 * lado as isize)) as usize;
            if read_i8(p + match_player::POSITION) >= 0 {
                *list.offset(*n as isize) = p;
                *n = (*n).wrapping_add(1);
            }
        }
    }
    *(G_ACTION_ORDER_MATCH_ENGINE as *mut usize) = m;
    exe::crt_qsort(m + engine::ACTIVE_LIST, (*n as i32) as u32, (4) as u32, 0x6bd840);
}

/// the order of the 11 takers of each side at +0x1af + 11 × side (−1 first), marking those chosen in each side's 20
/// slots at `engine::SHOOTOUT_TAKEN` (zeroed first). Side's manager (`engine::SIDE0_MANAGER` + 208 × side) human, with +0x5f, without 0x9a01 and (with
/// `engine::SIDE0_MANAGER_TACTIC_SLOT`) national team or with 0x9a05: the order comes from the tactics table (`exe::tactics_get_shootout_taker_slot` on [0xde1d08]). Otherwise,
/// `choose_taker` type 5; with no one, clears the side's marks and repeats the turn
/// Original: `penalty_order` (0x6be520, match_eng.cpp, thiscall).
///
/// [pt] { 0x6be520 } (match_eng.cpp): monta a ordem dos 11 cobradores de penalti de cada lado em
/// +0x1af + 11 x lado (-1 antes), marcando os escolhidos nas 20 casas de cada lado em +0x8eda
/// (zeradas antes). Se o tecnico do lado (+0x904d + 208 x lado) e um dos 16 humanos, tem a marca
/// +0x5f, nao tem 0x9a01 (0x5fd210) e, com +0x904b, e de selecao (0x53b900) ou tem 0x9a05
/// (0x5fd480): a ordem vem da tabela tatica do clube (0x898040 sobre [0xde1d08], com `humano` =
/// byte +0x40/+0x41 da partida igual a 0). Senao, `choose_taker` tipo 5; sem ninguem, zera as
/// marcas do lado e tenta a mesma vez de novo.
pub(crate) unsafe extern "thiscall" fn penalty_order(m: usize) {
    core::ptr::write_bytes((m + engine::SHOOTOUT_TAKEN) as *mut u8, 0, 40);
    core::ptr::write_bytes((m + engine::PENALTY_TAKER_ORDER) as *mut u8, 0xff, 22);
    let h = *(G_HUMAN_MANAGER as *const usize);
    for lado in 0..2i8 {
        let l = lado as usize;
        let mut team = 0i8;
        while team < 11 {
            let manager = Staff(*((m + engine::SIDE0_MANAGER + 208 * l) as *const usize));
            let persons = *(G_STAFF_COUNT as *const i32);
            let by_table = !manager.is_null()
                && (persons.wrapping_sub(16)..persons).contains(&*(manager.addr() as *const i32))
                && (manager.classification() as i8) != 0
                && exe::human_manager_is_on_holiday(h, manager) == 0
                && (read_i8(m + 208 * l + 0x904b) == 0 || {
                    let partida = *((m + engine::MATCH) as *const usize);
                    exe::database_is_national_team(*((partida + if lado == 0 { 0x1c } else { 0x20 }) as *const usize)) != 0
                        || exe::human_manager_controls_reserve_team(h, *((m + engine::SIDE0_MANAGER + 208 * l) as *const usize)) != 0
                });
            let chosen = if by_table {
                let partida = *((m + engine::MATCH) as *const usize);
                let (marca, clube) = if lado == 0 {
                    (read_i8(partida + fixture::HOME_SIDE_MARK), *((partida + fixture::HOME_CLUB) as *const usize))
                } else {
                    (read_i8(partida + fixture::AWAY_SIDE_MARK), *((partida + fixture::AWAY_CLUB) as *const usize))
                };
                Some(exe::tactics_get_shootout_taker_slot(*(G_TACTICS as *const usize), clube, team, (marca == 0) as u32) as i8)
            } else {
                let c = exe::cm3_match_engine_pick_restart_taker(m, lado, 5, m + engine::SHOOTOUT_TAKEN);
                if c == 0 {
                    team = team.wrapping_sub(1);
                    core::ptr::write_bytes((m + engine::SHOOTOUT_TAKEN + 20 * l) as *mut u8, 0, 20);
                    None
                } else {
                    Some(*(c as *const i8))
                }
            };
            if let Some(e) = chosen {
                *((m as isize + 0x1af + 11 * l as isize + team as isize) as *mut i8) = e;
                *((m as isize + 0x8eda + 20 * l as isize + e as isize) as *mut u8) = 1;
            }
            team = team.wrapping_add(1);
        }
    }
}

/// Is side `lado` currently controlled by its decision-maker? Never in match types 1 or 9; for a human-managed club, by
/// flag 0x4000 of the side's tactics block (0x9766 + 6371 x side); otherwise when the side's mode (+0x1ab) is 3 or 4.
/// Returns 1 or 0.
///
/// [pt] { 0x6d42a0 } (match_eng.cpp): o lado esta sob controle de quem decide por ele? Nao nas
/// partidas de tipo +0x3a 1 ou 9. Clube do lado dirigido por humano (0x600a50, `qualquer` = byte
/// +0x40 da partida): pela marca 0x4000 do bloco do lado (0x9766 + 6371 x lado). Senao: o modo do
/// lado em +0x1ab e 3 ou 4.
pub(crate) unsafe extern "thiscall" fn side_under_control(m: usize, side: i8) -> u8 {
    let partida = Fixture(*((m + engine::MATCH) as *const usize));
    let kind = (partida.category() as i8);
    if kind == 1 || kind == 9 {
        return 0;
    }
    let club = *((partida.addr() + if side != 0 { 0x20 } else { 0x1c }) as *const usize);
    let l = side as isize;
    if exe::human_manager_is_human_club(club, 0, ((partida.home_side_mark() as i8) as u8) as u32, (0) as u32, 0) != 0 {
        (*((m as isize + 6371 * l + 0x9766) as *const u32) & 0x4000 != 0) as u8
    } else {
        let mode = *((m as isize + l + 0x1ab) as *const i8);
        (mode == 3 || mode == 4) as u8
    }
}

/// Distance (plus the 0x968990 offset) from (x, y) to the goal that player j's side attacks, from the 0xa15e44 table.
///
/// [pt] Distancia (mais o ajuste 0x968990) do ponto (x, y) ao gol que o lado do jogador `j` ataca
/// (coluna 4; linha 0 para o lado 0, 11 para o lado 1), arredondada para float.
pub(crate) unsafe fn match_eng_distance_to_goal(j: usize, x: i32, y: i32) -> f32 {
    let dl = if side_of_player(j) == 0 { y } else { y - 11 };
    let (c, l) = ((x - 4).wrapping_abs() as i8, dl.wrapping_abs() as i8);
    let d = if (0..=8).contains(&c) && (0..=11).contains(&l) {
        *((0xa15e44 + 4 * (l as isize * 9 + c as isize)) as *const f32)
    } else {
        read_f32(rdata::F32_1000)
    };
    (d as f64 + read_f32(rdata::F32_5) as f64) as f32
}

/// How many opponents of side s surround (x, y): 3 for each at distance zero, 1 for each within 0x968940; inactive and
/// goalkeepers excluded.
///
/// [pt] Quantos adversarios (bloco do lado `s`) cercam o ponto (x, y): 3 para cada um a distancia
/// igual a 0x9689a0, 1 para cada um ate 0x968940. Ficam de fora os inativos e os de `table_a` 1.
pub(crate) unsafe fn surround_count(m: usize, s: usize, x: i8, y: i8) -> i8 {
    let mut n = 0i8;
    for i in 0..20 {
        let p = m + engine::PLAYERS + 8920 * s + 0x1be * i;
        if read_i8(p + match_player::POSITION) < 0 || player_table(p) == 1 {
            continue;
        }
        let d = distance_to_point(p, x, y) as f64;
        if d == read_f64(rdata::F64_0) || d.is_nan() {
            n = n.wrapping_add(3);
        } else if !(d > read_f64(rdata::F64_1_5)) {
            n = n.wrapping_add(1);
        }
    }
    n
}

pub(crate) static mut TABELA65: Vec<u8> = Vec::new();
pub(crate) static mut CLUBES74: Vec<u8> = Vec::new();

/// Fatigue bonus of the draw: ftol(0x96d2d8 - goal distance x 0x968a78).
///
/// [pt] Bonus de fadiga do sorteio: 0x96d2d8 - distancia ao gol x 0x968a78.
pub(crate) unsafe fn goal_distance_bonus(d: f32) -> i32 {
    ftol(read_f32(rdata::F32_225) as f64 - d as f64 * read_f32(rdata::F32_10) as f64)
}

/// Reputation used in the target rating: home if same nation as the player, current if both clubs share a nation, else
/// world; in hundreds minus 50.
///
/// [pt] Reputacao usada na nota do alvo: a de casa se ele e da nacao do jogador, a atual se os dois
/// clubes da partida sao da mesma nacao, senao a mundial; em centenas, menos 50.
pub(crate) unsafe fn target_reputation(j: usize, p: usize, clubs: (usize, usize)) -> i32 {
    let sheet = *((p + match_player::PLAYER) as *const usize);
    let nation = |q: usize| *((*((q + match_player::STAFF) as *const usize) + 0x1a) as *const u32);
    let off = if nation(j) == nation(p) {
        9
    } else if clubs.0 != 0 && clubs.1 != 0 && *((clubs.0 + 0x53) as *const u32) == *((clubs.1 + 0x53) as *const u32) {
        0xb
    } else {
        0xd
    };
    (*((sheet + off) as *const i16) as i32) / 100 - 50
}

/// Chooses the opponent that player `j` will mark, or 0: none if his position has instruction bit 1; the designated
/// man-marking target if active; otherwise scores the ball holder (if near) and the opponents in the 3 x 3 cells around
/// his formation cell (rolls, surrounding players, distance to goal, attributes, role, reputation) and returns the
/// best; the one stored in +0x1aa may be picked at once.
///
/// [pt] { 0x6bc030 } (match_eng.cpp): o adversario que o jogador `j` vai marcar, ou 0. Nao marca se a
/// casa dele tem a marca 1; o marcado designado (0x7160f0) ativo vem antes de tudo. Senao, pela
/// casa (x, y) onde ele deve estar (0x6c2be0), avalia primeiro quem esta com a bola (0xf582, se
/// perto) e depois os adversarios das casas das linhas y-1..y+1 e colunas x-1..x+1 (listas em
/// motor + 0x215e, 90 bytes por casa). Cada um recebe uma nota (sorteios, cerco de adversarios,
/// distancia ao gol, atributos 0x0a/0x0b/0x10/0x35/0x101, papel na formacao, reputacao) e fica o
/// de maior nota (comparada em 16 bits; o primeiro empate fica). O guardado em 0x1aa do jogador
/// pode ser escolhido na hora.
pub(crate) unsafe extern "thiscall" fn who_to_mark(m: usize, j: usize) -> usize {
    let side_j = side_of_player(j) as usize;
    if position_byte(m + engine::TACTICS + 6371 * side_j, read_i8(j + match_player::POSITION) as i8) & 1 != 0 {
        return 0;
    }
    let d = designated_marked(j);
    if d != 0 && read_i8(d + match_player::POSITION) >= 0 {
        return d;
    }
    let s = (side_j == 0) as usize; // the opposing side
    let partida = Fixture(*((m + engine::MATCH) as *const usize));
    let (c1c, c20) = (partida.home_club(), partida.away_club());
    let clubs = if s == 1 { (c1c, c20) } else { (c20, c1c) };
    let (mut x, mut y) = (0i8, 0i8);
    position_place_players(m, j, &mut x, &mut y, 1);
    let bonus = read_i8(j + match_player::MOMENT_POSITIONING);
    let away_of_half = |q: usize| -> bool {
        bonus != 0 && player_condition(q) != 0 && if s == 0 { y >= 6 } else { y < 6 }
    };
    let mut best = 0usize;
    let mut rating_best = 0i16;
    let mut store = |q: usize, rating: i32| {
        if best == 0 || rating as i16 > rating_best {
            best = q;
            rating_best = rating as i16;
        }
    };

    // who has the ball (the ”row” y-2 of the original’s loop)
    let c = *((m + engine::BALL_HOLDER) as *const usize);
    let y0 = y.wrapping_sub(2);
    if y0 <= y.wrapping_add(1) && x.wrapping_sub(1) <= x.wrapping_add(1) && c != 0 {
        let (cx, cy) = (read_i8(c + match_player::CELL_X), read_i8(c + match_player::CELL_Y));
        // the original measures from the player with the ball to `j`’s position
        if !(distance_to_point(c, read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y)) as f64 > read_f64(rdata::F64_2)) {
            let f1c = grid_distance_or_1000(cx as i32 - x as i32, cy as i32 - y as i32);
            let n17 = surround_count(m, s, cx, cy);
            let f18 = match_eng_distance_to_goal(j, cx as i32, cy as i32);
            let v16 = read_i8(c + match_player::MARKER_COUNT);
            if v16 != 3 && !away_of_half(c) {
                let mut a = random(5 * (21 - bonus as i32));
                // the original’s `fld [esp+0x1c]` runs with the roll argument still on the stack: and f18
                if (f18 as f64) < read_f64(rdata::F64_5) && read_i8(c + match_player::MOMENT_FLAIR) != 0 {
                    a = a.wrapping_add(goal_distance_bonus(f18));
                }
                if (random(30) as f64) < read_f32(c + match_player::LONG_SHOTS) as f64 && (random(30) as f64) < read_f32(j + match_player::LONG_SHOTS) as f64 {
                    a = a.wrapping_add(goal_distance_bonus(f18));
                }
                let mut b = a.wrapping_sub(100 * v16 as i32);
                if closest_to_goal(m, c) != 0 {
                    if c == *((j + match_player::MARKING_TARGET) as *const usize) {
                        return c;
                    }
                    b = b.wrapping_add(0xaf);
                } else if c == *((j + match_player::MARKING_TARGET) as *const usize) && random(v16 as i32 + 1) == 0 {
                    return c;
                }
                let mut e = (*((c + match_player::SHOTS_ON_TARGET) as *const u8) as i32 + *((c + match_player::SHOTS) as *const u8) as i32 + 0x14) * 10;
                e = e.wrapping_sub(ftol(f18 as f64 * f18 as f64 - read_f32(rdata::F32_100) as f64));
                let n = n17 as i32;
                e = e.wrapping_sub(n.wrapping_mul(n).wrapping_mul(n));
                e = e.wrapping_add(*((c + match_player::MATCH_RATING) as *const i16) as i32 / 200);
                e = e.wrapping_sub(ftol(*((c + match_player::ASSISTS) as *const u8) as f64 * read_f64(rdata::F64_NEG_7_5)));
                e = e.wrapping_sub(ftol(f1c as f64 * f1c as f64 * read_f32(rdata::F32_50) as f64));
                b = b.wrapping_add(e);
                store(c, b.wrapping_add(target_reputation(j, c, clubs)));
            }
        }
    }

    // the surrounding cells
    let mut r = y.wrapping_sub(1);
    while r <= y.wrapping_add(1) {
        if !(0..=11).contains(&r) {
            r = r.wrapping_add(1);
            continue;
        }
        let mut col = x.wrapping_sub(1);
        while col <= x.wrapping_add(1) {
            if !(0..=8).contains(&col) {
                col = col.wrapping_add(1);
                continue;
            }
            let home = m + engine::GRID + 90 * (r as usize + 12 * col as usize);
            let how_many = |home: usize| read_i8(home + (0x58 + s));
            if how_many(home) == 0 {
                col = col.wrapping_add(1);
                continue;
            }
            let f1c = grid_distance_or_1000(col as i32 - x as i32, r as i32 - y as i32);
            let n14 = surround_count(m, s, col, r);
            let f18 = match_eng_distance_to_goal(j, col as i32, r as i32);
            let mut k = 0i8;
            while k < how_many(home) {
                let q = *((home + 4 * (k as isize + 11 * s as isize) as usize) as *const usize);
                k = k.wrapping_add(1);
                let ponta = *((m + engine::BALL_HOLDER) as *const usize);
                if q == ponta {
                    continue;
                }
                let v16 = read_i8(q + match_player::MARKER_COUNT);
                if v16 == 3 || away_of_half(q) {
                    continue;
                }
                if q != ponta && q != *((m + engine::SET_PIECE_TAKER) as *const usize) {
                    let marks = *((m + engine::TACTICS_FLAGS + 6371 * side_j) as *const u32);
                    let dif = difference_of_player(q) as i8;
                    if if marks & 0x400 != 0 && bonus != 0 { dif >= 0 } else { dif > 0 } {
                        continue;
                    }
                }
                let mut a = random(5 * (21 - bonus as i32));
                if (random(30) as f64) < read_f32(q + match_player::JUMPING) as f64 && (random(30) as f64) < read_f32(j + match_player::JUMPING) as f64 {
                    a = a.wrapping_add(goal_distance_bonus(f18));
                }
                a = a.wrapping_sub(100 * v16 as i32);
                let stored = *((j + match_player::MARKING_TARGET) as *const usize);
                if closest_to_goal(m, q) != 0 {
                    if q == stored {
                        return q;
                    }
                    a = a.wrapping_add(0xaf);
                } else if q == stored && random(v16 as i32 + 1) == 0 {
                    if is_the_stored(q) != 0 || q == *((m + engine::SET_PIECE_TAKER) as *const usize) {
                        return q;
                    }
                    a = a.wrapping_add(0x14);
                }
                let mut e = (*((q + match_player::SHOTS_ON_TARGET) as *const u8) as i32 + *((q + match_player::SHOTS) as *const u8) as i32 + 0xa) * 10;
                e = e.wrapping_sub(ftol(read_f64(rdata::F64_180) - *((q + match_player::MATCH_RATING) as *const i16) as f64 * read_f64(rdata::F64_0_03)));
                e = e.wrapping_sub(ftol(f18 as f64 * f18 as f64 - read_f32(rdata::F32_100) as f64));
                e = e.wrapping_sub(ftol(*((q + match_player::ASSISTS) as *const u8) as f64 * read_f64(rdata::F64_NEG_7_5)));
                let n = n14 as i32;
                e = e.wrapping_sub(n.wrapping_mul(n));
                e = e.wrapping_sub(ftol(f1c as f64 * f1c as f64 * read_f32(rdata::F32_50) as f64));
                let mut rating = a.wrapping_add(e);
                let pos = read_i8(q + match_player::POSITION);
                let role = |base: usize| -> u16 {
                    if (0..=10).contains(&pos) {
                        let t = *((m + base + 4 * side_of_player(q) as usize) as *const usize);
                        *((t + 2 * pos as usize) as *const u16)
                    } else {
                        0
                    }
                };
                let w = role(0x8ebc);
                if w & 0x20 != 0 && w & 0x800 == 0 && w & 0x80 == 0 {
                    let sheet = *((q + match_player::PLAYER) as *const usize);
                    rating = rating.wrapping_add(10 * read_i8(sheet + player_sheet::FREE_ROLE) as i32);
                } else {
                    let fixed = |v: u16| v == 0x140 || v == 0x240 || v == 0x440;
                    if (0..=10).contains(&pos) && !fixed(role(0x8ebc)) && fixed(role(0x8ec4)) {
                        rating = rating.wrapping_add(0x7d);
                    } else if mark_800000(q) != 0 {
                        rating = rating.wrapping_add(0x28);
                    }
                }
                store(q, rating.wrapping_add(target_reputation(j, q, clubs)));
            }
            col = col.wrapping_add(1);
        }
        r = r.wrapping_add(1);
    }
    best
}

/// Calls MATCH_PLAYER move_to_grid_cell for player j and cell (x, y).
pub(crate) unsafe fn move_to_cell(j: usize, x: i8, y: i8) {
    exe::match_player_move_to_grid_cell(j, x, y);
}
/// Calls MATCH_PLAYER set_action_delay(0) for player j (field +0x2b).
pub(crate) unsafe fn clear_action_delay(j: usize) {
    exe::match_player_set_action_delay(j, 0);
}
/// Calls MATCH_PLAYER take_possession for player j.
pub(crate) unsafe fn take_possession(j: usize) {
    exe::match_player_take_possession(j);
}
/// Calls CM3_MATCH_ENGINE cobrador for a side and set-piece type.
pub(crate) unsafe fn pick_restart_taker(this: usize, side: i8, kind: i8) -> usize {
    exe::cm3_match_engine_pick_restart_taker(this, side, kind, 0)
}

/// throw-in for side 0x8eae. Counts (0xf5c8 + 54 × side), puts the ball (0x8ea7, 0x8ea8) inside the pitch, picks the
/// taker (type 2), arranges the 40 players by the formation table (0xf56e/0xf572 by the player's side; index: of the
/// side taking?, position, ball column and row; byte = column + 16 × row) and puts the taker on the ball, facing
/// inwards (90 or 270). With no taker the original shows an error box (no box here) and marks the engine (+0 = 1)
/// Original: `throw_in` (0x6c4290, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4290 } (match_eng.cpp): lateral do lado 0x8eae. Conta (0xf5c8 + 54 x lado), poe a bola
/// (0x8ea7, 0x8ea8) dentro do campo, escolhe o cobrador (tipo 2), arruma os 40 jogadores pela tabela
/// da formacao (0xf56e/0xf572 pelo lado do jogador; indice: do lado que cobra?, posicao, coluna e
/// linha da bola; byte = coluna | linha << 4) e poe o cobrador na bola, virado para dentro (90 ou 270).
/// Sem cobrador o original mostra uma caixa de erro (sem a caixa aqui) e marca o motor (+0 = 1).
pub(crate) unsafe extern "thiscall" fn throw_in(this: usize) {
    let t = read_i8(this + engine::POSSESSION_SIDE);
    let count = (this as isize + 54 * t as isize + 0xf5c8) as *mut u8;
    *count = (*count).wrapping_add(1);
    let a7 = (this + engine::BALL_X) as *mut i8;
    let a8 = (this + engine::BALL_Y) as *mut i8;
    *a7 = (*a7).max(0).min(8);
    *a8 = (*a8).max(0).min(11);
    let c = pick_restart_taker(this, t, 2);
    for l in 0..2i8 {
        let own = (t as i32 == l as i32) as isize;
        for k in 0..20 {
            let j = side_player_slot(this, l, k);
            let pos = read_i8(j + match_player::POSITION);
            if pos < 0 {
                continue;
            }
            let table = *((this + engine::FORMATION_TABLE + 4 * (read_i8(j + match_player::SIDE) == 1) as usize) as *const usize);
            let home = *((table as isize + 12 * ((own + 2 * pos as isize) * 9 + *a7 as isize) + *a8 as isize) as *const u8);
            move_to_cell(j, (home & 0xf) as i8, (home >> 4) as i8);
            clear_action_delay(j);
        }
    }
    if c == 0 {
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        *(this as *mut u8) = 1;
        return;
    }
    move_to_cell(c, *a7, *a8);
    take_possession(c);
    *((c + match_player::BALL_CONTROL) as *mut i16) = 200;
    *((c + match_player::FACING) as *mut i16) = if *a7 != 0 { 270 } else { 90 };
}

/// counts (0xf5c7 + 54 × side), marks the play (0x8eb1 = 3, 0x8eae = −1) and arranges the 40: by the attacking role
/// (`attacking_role`, side taking) or defending role (`defending_role`), some go to drawn squares in the box (column 4 ± 1; row 5/6, or
/// drawn 0..1/10..11), the others by the formation table (+0x3b or +0x30). Off the pitch: fatal error. Taker (type 3)
/// at the flag (column 0 or 8 by a draw against 0x8ea7; row 0 or 11)
/// Original: `corner` (0x6c4470, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c4470 } (match_eng.cpp): escanteio do lado 0x8eae. Conta (0xf5c7 + 54 x lado), marca o
/// lance (0x8eb1 = 3, 0x8eae = -1) e arruma os 40 jogadores: pelo papel no ataque (0x716270, do lado
/// que cobra) ou na defesa (0x716310), alguns vao para casas sorteadas na area (coluna 4 +- 1; linha
/// 5/6, ou 0..1/10..11 sorteada), os outros pela tabela da formacao (0xf56e lado 0, 0xf572 lado 1;
/// +0x3b ou +0x30). Fora do campo: erro fatal. Depois escolhe o cobrador (tipo 3) e o poe na
/// bandeira (coluna 0 ou 8 pelo sorteio contra 0x8ea7; linha 0 ou 11).
pub(crate) unsafe extern "thiscall" fn corner(this: usize) {
    let t = read_i8(this + engine::POSSESSION_SIDE);
    let count = (this as isize + 54 * t as isize + 0xf5c7) as *mut u8;
    *count = (*count).wrapping_add(1);
    *((this + engine::SET_PIECE_KIND) as *mut u8) = 3;
    *((this + engine::POSSESSION_SIDE) as *mut i8) = -1;
    for l in 0..2i8 {
        let own = t as i32 == l as i32;
        for k in 0..20 {
            let j = side_player_slot(this, l, k);
            let pos = read_i8(j + match_player::POSITION);
            if pos < 0 {
                continue;
            }
            // area squares: ’a’ row drawn 0..1, ’b’ 10..11, ’c’ 6, ’d’ 5
            let r = if own { exe::match_player_get_attacking_set_piece_role(j) } else { exe::match_player_get_defending_set_piece_role(j) };
            let area = match (own, l == 1, r) {
                (true, true, 1) | (false, false, 4) => Some('a'),
                (true, false, 1) | (false, true, 4) => Some('b'),
                (true, true, 2) | (false, true, 3) => Some('c'),
                (true, false, 2) | (false, false, 3) => Some('d'),
                _ => None,
            };
            let (x, y) = match area {
                Some(z) => {
                    let mut x = 4i8.wrapping_sub(exe::utils_random(2) as i8);
                    x = x.wrapping_add(exe::utils_random(2) as i8);
                    let y = match z {
                        'a' => exe::utils_random(2) as i8,
                        'b' => 11i8.wrapping_sub(exe::utils_random(2) as i8),
                        'c' => 6,
                        _ => 5,
                    };
                    (x, y)
                }
                None => {
                    let table = *((this + if l == 0 { 0xf56e } else { 0xf572 }) as *const usize);
                    let desl = if own == (l == 0) { 0x3b } else { 0x30 };
                    let home = *((table as isize + 4 * 27 * (own as isize + 2 * pos as isize) + desl) as *const u8);
                    ((home & 0xf) as i8, (home >> 4) as i8)
                }
            };
            if !(0..9).contains(&x) || !(0..12).contains(&y) {
                fatal_exit();
            }
            move_to_cell(j, x, y);
            clear_action_delay(j);
        }
    }
    let c = pick_restart_taker(this, t, 3);
    let r = exe::utils_random(2);
    let x = if read_i8(this + engine::BALL_X) as i32 > 4 - r { 0 } else { 8 };
    move_to_cell(c, x, if t != 0 { 0 } else { 11 });
    take_possession(c);
    *((c + match_player::BALL_CONTROL) as *mut i16) = 200;
    if t == 0 {
        *((c + match_player::FACING) as *mut i16) = if read_i8(this + engine::BALL_X) != 0 { 90 } else { 270 };
    }
}

/// taker (type 5) with the ball on the spot (column 4; row 1 or 10, also in 0x8ea7/0x8ea8); the stored goalkeepers go
/// to goal, those on rows 0..1 go to row 2 and those on 10..11 to row 9
/// Original: `penalty` (0x6c50d0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c50d0 } (match_eng.cpp): penalti do lado 0x8eae. Escolhe o cobrador (tipo 5), da a bola a
/// ele na marca (coluna 4; linha 1 ou 10, tambem em 0x8ea7/0x8ea8) e tira os outros da area: os
/// goleiros guardados (0xf59e, 0xf5a2) vao para o gol (linha 0 / 11), quem esta nas linhas 0..1 vai
/// para a 2 e quem esta nas 10..11 para a 9, na mesma coluna.
pub(crate) unsafe extern "thiscall" fn penalty(this: usize) {
    let t = read_i8(this + engine::POSSESSION_SIDE);
    let c = pick_restart_taker(this, t, 5);
    take_possession(c);
    let row: i8 = if t != 0 { 1 } else { 10 };
    *((this + engine::BALL_X) as *mut i8) = 4;
    *((this + engine::BALL_Y) as *mut i8) = row;
    move_to_cell(c, 4, row);
    for k in 0..20 {
        for l in 0..2i8 {
            let j = side_player_slot(this, l, k);
            if read_i8(j + match_player::POSITION) >= 0 && j != c {
                let goalkeeper = *((this + if l == 0 { 0xf59e } else { 0xf5a2 }) as *const usize);
                let y = read_i8(j + match_player::CELL_Y);
                if j == goalkeeper {
                    move_to_cell(j, 4, if l == 0 { 0 } else { 11 });
                } else if y < 2 {
                    move_to_cell(j, read_i8(j + match_player::CELL_X), 2);
                } else if y > 9 {
                    move_to_cell(j, read_i8(j + match_player::CELL_X), 9);
                }
            }
            clear_action_delay(j);
        }
    }
}

/// The original's fatal error exit without the message: clears the mouse flag, runs the fatal cleanup and exits with
/// -1.
///
/// [pt] Caixa de erro do original (match_eng.cpp, linha) sem a mensagem: zera 0xb67a34, 0x5cc7d0,
/// 0x5a8f60 e exit(-1).
pub(crate) unsafe fn fatal_exit() -> ! {
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// `0x6ceb90` (match_eng.cpp): chooses the side's taker for set piece `tipo` (1..=8; 3 corner, 5 penalty, 6/7 free
/// kick) as the highest-scoring of the 20 players in the block (the first tie stays).
///
/// Excluded: position < 0, or marked in `excluidos` (type 5). Position > 10 or no person (+0x6d): position becomes −1.
/// The guarded goalkeeper (0xf59e/0xf5a2) does not take 1, 2, 3, 8, nor 6/7 far from the opposing area.
///
/// Scores: 1 and 2: attribute (0xdd, 0xe9) − 2 × distance to the ball; 3 (corner): the taker designated in the tactic
/// (`stored_taker` on 0xde1d08; 10000/9000, with a random draw on the opposite side) or 0x89 − 0xad/2, × 0.75 (1.25 if
/// negative) with mark 0x40 of the table; 4: 100 with table = 1; 5 (penalty): the human manager's list (10000 − order,
/// `exe::human_manager_get_squad_number_player`) or 0xc9 by age, random draw of the person's +0x5a and +0xb; 6/7 (free kick): near the goal, the designated
/// taker or the person's set-piece ability (+0x29) and 0x109; far away, 0xa1 − distances; 8: 100 − distance.
///
/// Returns the match player; with nobody: 0 with `excluidos` (penalty shoot-out list), otherwise a fatal error.
///
/// [pt] { 0x6ceb90 } (match_eng.cpp): escolhe o cobrador do lado para a bola parada `tipo` (1..=8),
/// pela maior pontuacao entre os 20 do bloco (o primeiro empate fica). Fora: posicao < 0, ou
/// marcado em `excluidos` (tipo 5). Posicao > 10 ou sem pessoa (+0x6d): posicao vira -1. O goleiro
/// guardado (0xf59e/0xf5a2) nao cobra 1, 2, 3, 8, nem 6/7 longe da area adversaria. Pontuacoes:
/// 1 e 2: atributo (0xdd, 0xe9) - 2 x distancia a bola; 3 (escanteio): o cobrador designado na tatica
/// (0x897990 sobre 0xde1d08; 10000/9000, com sorteio no lado contrario) ou 0x89 - 0xad/2, x 0,75
/// (1,25 se negativo) com a marca 0x40 da tabela; 4: 100 com a tabela = 1; 5 (penalti): a lista do
/// tecnico humano (10000 - ordem, 0x601070) ou 0xc9 pela idade, sorteio de +0x5a e +0xb da pessoa;
/// 6/7 (falta): perto do gol, o designado ou bola parada da pessoa (+0x29) e 0x109; longe,
/// 0xa1 - distancias; 8: 100 - distancia. Sem ninguem: 0 com `excluidos`, senao erro fatal.
pub(crate) unsafe extern "thiscall" fn choose_taker(this: usize, side: i8, kind: i8, excluidos: usize) -> usize {
    let f = |p: usize, o: usize| read_f32(p + o) as f64;
    let partida = read_usize(this + engine::MATCH);
    let (mut melhor, mut melhor_v) = (0usize, -1i16);
    for k in 0..20isize {
        let i = side as isize * 20 + k;
        let j = (this as isize + 0x4796 + 0x1be * i) as usize;
        let (a7, a8) = (read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y));
        if kind == 5 && excluidos != 0 && *((excluidos as isize + i) as *const u8) != 0 {
            continue;
        }
        if read_i8(j + match_player::POSITION) < 0 {
            continue;
        }
        let stored = (side == 0 && j == read_usize(this + engine::GOALKEEPER)) || (side == 1 && j == read_usize(this + engine::GOALKEEPER_SIDE1));
        if stored {
            match kind {
                1 | 2 | 3 | 8 => continue,
                6 | 7 => {
                    let row = if side == 1 { (10..=11).contains(&a8) } else { (0..=1).contains(&a8) };
                    if !((2..=6).contains(&a7) && row) {
                        continue;
                    }
                }
                _ => {}
            }
        }
        if read_i8(j + match_player::POSITION) > 10 || read_usize(j + match_player::PLAYER) == 0 {
            *((j + match_player::POSITION) as *mut i8) = -1;
            continue;
        }
        // designated taker in the player’s club tactic
        let designado = |a: i32, b: i32| {
            let away = read_i8(j + match_player::SIDE) != 0;
            let reserve = (read_i8(partida + (if away { 0x41 } else { 0x40 })) == 0) as i32;
            let club = read_usize(partida + (if away { 0x20 } else { 0x1c }));
            exe::tactics_get_set_piece_taker(*(G_TACTICS as *const usize), club, a, b, reserve) == read_usize(j + match_player::STAFF)
        };
        // near goal: table of ball distances (|a7 - 4|, |a8 - goal line|)
        let table_ball = || {
            let cx = (a7 as i32 - 4).abs() as i8;
            let ax = (a8 as i32 - if side != 0 { 0 } else { 11 }).abs() as i8;
            if (0..=8).contains(&cx) && (0..=11).contains(&ax) {
                distance_table_cell(ax as i32, cx as i32) as f64
            } else {
                read_f32(rdata::F32_1000) as f64
            }
        };
        let attack = if side == 0 { a7 < 4 } else { side == 1 && a7 > 4 };
        let bola_parada = || {
            let pd = read_usize(j + match_player::PLAYER);
            let mut s = (read_i8(pd + player_sheet::FREE_KICKS) as i16).wrapping_mul(5).wrapping_add((read_i8(j + match_player::MOMENT_FLAIR) as i16).wrapping_mul(2));
            if s <= 0 {
                return s;
            }
            let age = read_i8(read_usize(j + match_player::STAFF) + 0x18);
            if age < 20 {
                s = ftol(s as f64 * read_f64(rdata::F64_0_85)) as i16;
            } else if age < 24 {
                s = ftol(s as f64 * read_f64(rdata::F64_0_95)) as i16;
            }
            ftol((*((pd + player_sheet::CURRENT_REPUTATION) as *const i16) as f64 * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_75)) * s as f64) as i16
        };
        let far = || {
            let g1 = exe::match_player_distance_to_cell(j, a7, a8);
            let g2 = exe::match_player_distance_to_cell(j, a7, a8);
            ftol(f(j, match_player::FREE_KICKS) - g2 as f64 * g1 as f64) as i16
        };
        let v: i16 = match kind {
            1 | 2 => ftol(f(j, if kind == 1 { 0xdd } else { 0xe9 }) - 2.0 * exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,
            3 => {
                let points = || {
                    let s = ftol(f(j, match_player::CORNERS) - f(j, match_player::JUMPING) * read_f32(rdata::F32_0_5) as f64) as i16;
                    if table_a(this, j) & 0x40 == 0 {
                        s
                    } else if s > 0 {
                        ftol(s as f64 * read_f64(rdata::F64_0_75)) as i16
                    } else {
                        ftol(s as f64 * read_f64(rdata::F64_1_25)) as i16
                    }
                };
                if attack {
                    if designado(2, 0) { 10000 } else if designado(2, 1) { 9000 } else { points() }
                } else if designado(3, 0) {
                    (10000 - exe::utils_random(1000)) as i16
                } else if designado(3, 1) {
                    (exe::utils_random(500) + 9000) as i16
                } else {
                    points()
                }
            }
            4 => if table_a(this, j) == 1 { 100 } else { 0 },
            5 => {
                let block = (this as isize + 0xd0 * side as isize) as usize;
                let manager = Staff(read_usize(block + engine::SIDE0_MANAGER));
                let n = *(G_STAFF_COUNT as *const i32);
                let list = !manager.is_null()
                    && (n - 16..n).contains(&*(manager.addr() as *const i32))
                    && (manager.classification() as i8) != 0
                    && exe::human_manager_is_on_holiday(*(G_HUMAN_MANAGER as *const usize), manager) == 0
                    && (read_i8(block + engine::SIDE0_MANAGER_TACTIC_SLOT) == 0
                        || is_national_team(read_usize(partida + (if side == 0 { 0x1c } else { 0x20 }))) != 0
                        || exe::human_manager_controls_reserve_team(*(G_HUMAN_MANAGER as *const usize), read_usize(block + engine::SIDE0_MANAGER)) != 0)
                    && excluidos == 0;
                if list {
                    let mut v = ftol(f(j, match_player::PENALTIES) - (read_i8(read_usize(j + match_player::PLAYER) + 0xf) as i32 * 20 - 20) as f64) as i16;
                    for b in 0..50i32 {
                        if exe::human_manager_get_squad_number_player(*(G_HUMAN_MANAGER as *const usize), b, read_usize(block + engine::SIDE0_MANAGER)) == read_usize(j + match_player::STAFF) {
                            v = 10000 - b as i16;
                            break;
                        }
                    }
                    v
                } else if j == read_usize(this + engine::GOALKEEPER) || j == read_usize(this + engine::GOALKEEPER_SIDE1) {
                    -10000
                } else {
                    let mut s = ftol(f(j, match_player::PENALTIES)) as i16;
                    if s > 0 {
                        let sheet = Staff(read_usize(j + match_player::STAFF));
                        let age = (sheet.year_of_birth() as i8);
                        if age < 20 {
                            s = ftol(s as f64 * read_f64(rdata::F64_0_35)) as i16;
                        } else if age < 24 {
                            s = ftol(s as f64 * read_f64(rdata::F64_0_75)) as i16;
                        }
                        let b5a = sheet.pressure();
                        if b5a != 0 && exe::utils_random(b5a as i32) == 0 {
                            s = ftol(s as f64 * read_f64(rdata::F64_0_35)) as i16;
                        }
                        s = ftol((*((read_usize(j + match_player::PLAYER) + 0xb) as *const i16) as f64 * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_75)) * s as f64) as i16;
                    }
                    s
                }
            }
            6 | 7 => {
                if (2..=6).contains(&a7) {
                    if table_ball() < read_f64(rdata::F64_4_1) {
                        let a = if attack { 0 } else { 1 };
                        if designado(a, 0) { 10000 } else if designado(a, 1) { 9000 } else { bola_parada() }
                    } else {
                        far()
                    }
                } else if table_ball() < read_f64(rdata::F64_6_1) {
                    ftol(f(j, match_player::FREE_KICKS) * 2.0 - f(j, match_player::JUMPING) * read_f32(rdata::F32_0_5) as f64) as i16
                } else {
                    far()
                }
            }
            8 => ftol(read_f32(rdata::F32_100) as f64 - exe::match_player_distance_to_cell(j, a7, a8) as f64) as i16,
            _ => fatal_exit(),
        };
        if melhor == 0 || v > melhor_v {
            melhor_v = v;
            melhor = j;
        }
    }
    if melhor == 0 && excluidos == 0 {
        fatal_exit();
    }
    melhor
}

/// Updates a player's three reputations (current +8, home +0xa, world +0xc of his person record) after a match from his
/// rating `valor`, weighted by the competition/opponent level and whether it is a national team, his club or a world
/// competition, with per-match change limits and ceilings; writes the new values into `reg` (home, current, world).
///
/// [pt] { 0x6d3610 } (match_eng.cpp): reputacoes do jogador depois da partida, na mesma conta da do
/// clube (0x6d3de0), para as tres da ficha (+0x69 do jogador): atual (+8), em casa (+0xa) e mundial
/// (+0xc). Grava em `reg` (casa, atual, mundial). Pesos: casa = (reputacao/1000 + 16, minimo 22) -
/// nivel; atual = o mesmo, se o clube e selecao ou o do jogador (+0x1a) e o do clube (+0x53) ou o da
/// competicao (+0x5d), senao a media disso com 202 - 10 x nivel (minimo 4); mundial como a do clube
/// (minimo 21). Faixas por partida: casa -250/+300, atual -250/+375, mundial -500/+100 x nivel (x
/// 2 - reputacao/10000). Tetos: 400 x nivel (/2 sem o caso especial), entre 5000 (mundial 3000) e 10000.
pub(crate) unsafe extern "C" fn player_reputation(reg: usize, my: usize, other: usize, partida: usize, value: f32, alt: u8, bonus: u8) {
    let partida = Fixture(partida);
    let my = Club(my);
    let other = Club(other);
    let p = Staff(*((reg + match_manager::MANAGER) as *const usize));
    if p.is_null() {
        return;
    }
    let st = p.non_player();
    if st == 0 {
        return;
    }
    let comp = Competition(partida.competition());
    let mut level: i8 = if *(partida.addr() as *const i32) == -2 {
        5
    } else {
        let v = if (partida.home_side_mark() as i8) != 0 {
            (((other.reputation() as i32) / 500 + (comp.reputation() as i32)) / 2) as i8
        } else {
            (comp.reputation() as i8)
        };
        v.min(20)
    };
    let ceiling = |rep: i32, minimum: i8| ((rep / 1000) as i8).wrapping_add(16).max(minimum) as i16;
    let (atual, casa, mundo) = ((read_i16(st + non_player::HOME_REPUTATION) as i32), (read_i16(st + non_player::CURRENT_REPUTATION) as i32), (read_i16(st + non_player::WORLD_REPUTATION) as i32));
    let n_home = ceiling(casa, 22).wrapping_sub(level as i16);
    let national_team = is_national_team(my.addr()) != 0;
    let club_p = p.nation();
    let (mut f_atual, mut f_mundo) = (2i32, 2i32);
    // simplification: the original also tests match -2 with a non-null +0x53; the last test (+0x53 equal) already covers it
    let n_current: i16 = if national_team || (!comp.is_null() && comp.nation() == club_p) || my.nation().addr() == club_p {
        f_atual = 1;
        ceiling(atual, 22).wrapping_sub(level as i16)
    } else {
        let x = ((0xca - 10 * level as i32) as i16 as i32 + ceiling(atual, 22).wrapping_sub(level as i16) as i32) / 2;
        if (x as i16) < 4 { 4 } else { x as i16 }
    };
    let w30 = (partida.round_name() as u16);
    let w32 = (partida.round_prefix() as u16);
    let n_world: i16 = if national_team {
        f_mundo = 1;
        ceiling(mundo, 21).wrapping_sub(level as i16)
    } else if (partida.category() as i8) == 8 {
        f_mundo = 1;
        if (0x41f..=0x426).contains(&w32) {
            ceiling(mundo, 21).wrapping_sub(level as i16).wrapping_add(1)
        } else if matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
            level = (comp.reputation() as i8);
            ceiling(mundo, 21).wrapping_sub(level as i16)
        } else {
            (0xca - 10 * level as i32) as i16
        }
    } else {
        (0xca - 10 * level as i32) as i16
    };
    let nv = level as i32;
    let dif = (value as f64 - global_stats_ratio(alt != 0)) * read_f32(rdata::F32_1_299999952) as f64;
    let g = (nv as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_5)) * dif * read_f64(rdata::F64_50);
    let mut new_current = ftol(atual as f64 + g) as i16;
    let mut new_home = ftol(casa as f64 + g) as i16;
    let mut new_world = ftol(mundo as f64 + g) as i16;
    if bonus != 0 {
        let mut t = ftol((nv * 250) as f64 * value as f64 / global_stats_ratio(alt != 0));
        if g < read_f32(rdata::F32_0) as f64 {
            t = ftol(t as i16 as f64 * value as f64 / global_stats_ratio(alt != 0));
        }
        let t = t as i16;
        new_home = new_home.max(t);
        new_current = new_current.max(t);
        new_world = new_world.max(t);
    }
    let limit = (nv * 300 + 4000) as i16;
    let k = |r: i32| (read_f64(rdata::F64_2) - r as f64 * read_f64(rdata::F64_0_0001)) as f32;
    let average = |n: i16, r: i32, new: i16| {
        let n = n as i32;
        (((n - 1).wrapping_mul(r).wrapping_add(new as i32) / n) as i16).min(limit.max(r as i16))
    };
    let band = |r: i32, low: f64, high: f64, m: i16| {
        let (rf, mf, k) = (r as f32 as f64, m as f32 as f64, k(r) as f64);
        let lo = rf - k * low;
        let v = if lo > mf {
            lo
        } else {
            let hi = k * high + rf;
            if hi < mf { hi } else { mf }
        };
        ftol(v) as i16
    };
    let r_home = band(casa, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_300) as f64, average(n_home, casa, new_home));
    let r_current = band(atual, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_375) as f64, average(n_current, atual, new_current));
    let r_world = band(mundo, read_f32(rdata::F32_500) as f64, (nv * 100) as f64, average(n_world, mundo, new_world));
    let c = nv.wrapping_mul(400);
    let cap = |x: i32, floor: i32, r: i32| (x.clamp(floor, 10000) as i16).max(r as i16);
    *(reg as *mut i16) = r_home.min(cap(c, 5000, casa)).max(1);
    *((reg + match_manager::MANAGER_HOME_REPUTATION) as *mut i16) = r_current.min(cap(c / f_atual, 5000, atual)).max(1);
    *((reg + match_manager::MANAGER_WORLD_REPUTATION) as *mut i16) = r_world.min(cap(c / f_mundo, 3000, mundo)).max(1);
}

/// Ratio of the global player stats (G_PLAYER_STATS): +0x20/+0x28 or +0x10/+0x18; zero divisor gives the 0x968998
/// default.
///
/// [pt] Razao das estatisticas globais (0xb74350): +0x20/+0x28 ou +0x10/+0x18; divisor zero da 6,8.
pub(crate) unsafe fn global_stats_ratio(alt: bool) -> f64 {
    let g = *(G_PLAYER_STATS as *const usize);
    let (a, b) = if alt { (0x20, 0x28) } else { (0x10, 0x18) };
    if read_f64(g + b) == read_f64(rdata::F64_0) {
        read_f64(rdata::F64_6_8)
    } else {
        read_f64(g + a) / read_f64(g + b)
    }
}

/// Match engine: updates the reputation of side `lado`'s club after the match (written to `engine::SIDE_REPUTATION` + 2 x side, 1..10000)
/// as a moving average: the level comes from the competition (+0x69, up to 20, or averaged with the opponent's
/// reputation) and the weight from that level or, for national teams and some international competitions, from the
/// club's reputation. `valor` against the global stats ratio gives the gain (squared when negative); `bonus` allows a
/// minimum from the level.
///
/// [pt] { 0x6d3de0 } (match_eng.cpp): reputacao do clube do lado depois da partida, gravada em
/// 0x1d6 + 2 x lado (1..=10000). `nivel` (ate 20) vem da competicao (+0x69), media com a reputacao
/// do adversario /500 quando a partida tem +0x40; 5 na partida -2. O peso `n` da media movel: 202 -
/// 10 x nivel, ou, para selecao (e certos codigos de competicao internacional, tipo 8), reputacao/1000
/// + 16 (no minimo 21) - nivel. `valor` contra a razao global da o ganho (negativo ao quadrado).
pub(crate) unsafe extern "C" fn new_reputation(this: usize, side: i8, value: f32, alt: u8, bonus: u8) {
    let partida = Fixture(*((this + engine::MATCH) as *const usize));
    let (meu, outro) = if side != 0 {
        (partida.away_club(), partida.home_club())
    } else {
        (partida.home_club(), partida.away_club())
    };
    let comp = Competition(partida.competition());
    let rep_de = |c: usize| *((c + club::REPUTATION) as *const i16) as i32;
    let mut level: i8 = if *(partida.addr() as *const i32) == -2 {
        5
    } else {
        let v = if (partida.home_side_mark() as i8) != 0 {
            ((rep_de(outro) / 500 + comp.reputation() as i32) / 2) as i8
        } else {
            (comp.reputation() as i8)
        };
        v.min(20)
    };
    let ceiling = || ((rep_de(meu) / 1000) as i8).wrapping_add(16).max(21) as i16;
    let w30 = (partida.round_name() as u16);
    let w32 = (partida.round_prefix() as u16);
    let n: i16 = if is_national_team(meu) != 0 {
        ceiling().wrapping_sub(level as i16)
    } else if (partida.category() as i8) == 8 && (0x41f..=0x426).contains(&w32) {
        ceiling().wrapping_sub(level as i16).wrapping_add(1)
    } else if (partida.category() as i8) == 8 && matches!(w30, 0x78..=0x7c | 0x82..=0x87 | 0x96..=0x9b) {
        level = (comp.reputation() as i8);
        ceiling().wrapping_sub(level as i16)
    } else {
        (0xca - 10 * level as i32) as i16
    };
    let mut f = (value as f64 - global_stats_ratio(alt != 0)) as f32;
    let zero = read_f32(rdata::F32_0) as f64;
    if (f as f64) < zero {
        f = (f.abs() as f64 * f as f64) as f32;
    }
    let rep = rep_de(meu);
    let nv = level as i32;
    let mut new = ftol((nv as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_5)) * f as f64 * read_f64(rdata::F64_50) + rep as f64) as i16;
    if bonus != 0 {
        let mut t = ftol((nv * 250) as f64 * value as f64 / global_stats_ratio(alt != 0));
        if (f as f64) < zero {
            t = ftol(t as i16 as f64 * value as f64 / global_stats_ratio(alt != 0));
        }
        if t as i16 > new {
            new = t as i16;
        }
    }
    let k = read_f64(rdata::F64_2) - rep as f64 * read_f64(rdata::F64_0_0001);
    let nn = n as i32;
    let mut average = (((nn - 1).wrapping_mul(rep)).wrapping_add(new as i32) / nn) as i16;
    let limit = (nv * 300 + 4000) as i16;
    let maximo = limit.max(rep as i16);
    if average >= maximo {
        average = maximo;
    }
    let delta = ((nv * 5) as f64 * k) as f32;
    let rf = rep as f32;
    let mf = average as f32;
    let low = rf as f64 - delta as f64;
    let v = if low > mf as f64 {
        low
    } else {
        let high = delta as f64 + rf as f64;
        if high < mf as f64 { high } else { mf as f64 }
    };
    let r = (ftol(v) as i16).clamp(1, 10000);
    *((this as isize + 0x1d6 + 2 * side as isize) as *mut i16) = r;
}

/// for each side and active player whose position has marks 0x10 and 4 in the side's table (`position_byte`), finds the direct
/// marker (`exe::cm3_match_engine_find_marking_target`) and stores his record (+0x69) in the per-position table (0x5b5910). Marker −1 reads the "player"
/// before the block, like the original
/// Original: `individual_marking` (0x6bbf60, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bbf60 } (match_eng.cpp): marcacao individual. Para cada lado e jogador ativo cuja posicao
/// tem as marcas 0x10 e 4 na tabela do lado (0x5b57c0), acha o marcador direto (0x6d4330) e grava
/// a ficha dele (0x69) na tabela por posicao (0x5b5910). Marcador -1 le o "jogador" anterior ao
/// bloco, como o original.
pub(crate) unsafe extern "thiscall" fn individual_marking(this: usize) {
    let marker: unsafe extern "thiscall" fn(usize, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_FIND_MARKING_TARGET);
    for l in 0..2i8 {
        let table = this + engine::TACTICS + 0x18e3 * l as usize;
        for k in 0..20i8 {
            let p = side_player_slot(this, l, k as usize);
            let pos = read_i8(p + match_player::POSITION);
            if pos < 0 || exe::formation_get_marking_type(table, pos) & 0x10 == 0 || exe::formation_get_marking_type(table, read_i8(p + match_player::POSITION)) & 4 == 0 {
                continue;
            }
            let mk = marker(this, l, k) as i8;
            let other = (l == 0) as isize;
            let q = (this as isize + 0x4796 + 0x1be * (20 * other + mk as isize)) as usize;
            exe::formation_set_marking_target(table, read_i8(p + match_player::POSITION), *((q + match_player::STAFF) as *const usize) as *const u32);
        }
    }
}

/// Match engine: takes player `j` off the pitch (sending-off or substitution): removes him from his grid cell and
/// undoes his marking links. Unless `modo` = -1, if `modo` = -3 or he is his side's goalkeeper, and he is not a
/// forward, it chooses who takes over his position by score (forwards 70,000, attacking midfielders 50,000, ...; a
/// removed goalkeeper's replacement becomes the side's goalkeeper); a human manager's team only does this automatically
/// under certain settings. No candidate: the game exits with an error, like the original.
///
/// [pt] { 0x6cdd50 } (match_eng.cpp): tira o jogador de campo (expulsao ou substituicao). Sai da lista
/// da casa dele na grade (o ultimo ocupa a vaga), desfaz as marcacoes (o marcado em 0x1aa e a lista
/// 0x1ae dos que o marcam). Com `modo` = -1, so isso. Senao, se `modo` = -3 ou se ele e o guardado
/// em 0xf59e/0xf5a2, e nao e atacante (0x40/0x20): escolhe quem assume a posicao dele (0x19), pela
/// maior pontuacao (atacante 70 mil, meia ofensivo 50 mil, 0x10 15 mil, outros -5 mil; +5 mil pelos
/// lados 0x80/0x800); o primeiro candidato entra sem filtro, os seguintes nao podem ser goleiro nem
/// papel 2 ou 4. Goleiro tirado: o escolhido vira o guardado do lado. Time de tecnico humano (pessoa
/// em 0x904d + 208 x lado entre os 16 tecnicos humanos, com classificacao em 0x5f e 0x5fd210 = 0) so
/// escolhe sozinho com a marca 0x904b, sem ser selecao e sem 0x5fd480.
/// Sem candidato: o original mostra uma caixa de erro e encerra o jogo (aqui, o mesmo encerramento).
pub(crate) unsafe extern "thiscall" fn take_off_pitch(this: usize, j: usize, mode: i8) {
    let position = read_i8(j + match_player::POSITION);
    if position < 0 {
        return;
    }
    let home = occupancy_grid_cell(this, read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y)) as usize;
    let l = read_i8(j + match_player::SIDE) as isize;
    let list = |i: isize| (home as isize + 4 * (11 * l + i)) as *mut usize;
    let n = (home as isize + 0x58 + l) as *mut i8;
    *list(read_i8(j + match_player::CELL_SLOT) as isize) = *list(*n as isize - 1);
    *n = (*n).wrapping_sub(1);
    let idx = read_i8(j + match_player::CELL_SLOT);
    let movido = *list(idx as isize);
    if movido != 0 {
        *((movido + match_player::CELL_SLOT) as *mut i8) = idx;
    }
    let marked = (j + match_player::MARKING_TARGET) as *mut usize;
    if *marked != 0 {
        if exe::match_player_is_marked_by(*marked, j) != 0 {
            exe::match_player_remove_marker(*marked, j);
        }
        *marked = 0;
    }
    if read_i8(j + match_player::MARKER_COUNT) != 0 {
        let mut i: i8 = 0;
        while i < read_i8(j + match_player::MARKER_COUNT) {
            let q = *((j + match_player::MARKERS + 4 * i as usize) as *const usize);
            if *((q + match_player::MARKING_TARGET) as *const usize) == j {
                *((q + match_player::MARKING_TARGET) as *mut usize) = 0;
            }
            i += 1;
        }
        *((j + match_player::MARKER_COUNT) as *mut i8) = 0;
    }
    if mode == -1 {
        return;
    }
    if mode != -3 && j != *((this + engine::GOALKEEPER_SIDE1) as *const usize) && j != *((this + engine::GOALKEEPER) as *const usize) {
        return;
    }
    if match_player_position(j) & 0x40 != 0 || match_player_position(j) & 0x20 != 0 {
        return;
    }
    let side_j = read_i8(j + match_player::SIDE);
    let block_manager = this as isize + 208 * side_j as isize;
    let tec = Staff(*((block_manager + engine::SIDE0_MANAGER as isize) as *const usize));
    if !tec.is_null() {
        let npes = *(G_STAFF_COUNT as *const i32);
        let id = *(tec.addr() as *const i32);
        let manager = *(G_HUMAN_MANAGER as *const usize);
        if id >= npes - 16 && id < npes && (tec.classification() as i8) != 0 && exe::human_manager_is_on_holiday(manager, tec) == 0 {
            if *((block_manager + engine::SIDE0_MANAGER_TACTIC_SLOT as isize) as *const u8) == 0 {
                return;
            }
            let partida = *((this + engine::MATCH) as *const usize);
            let club = *((partida + if side_j == 0 { 0x1c } else { 0x20 }) as *const usize);
            if exe::database_is_national_team(club) != 0 {
                return;
            }
            if exe::human_manager_controls_reserve_team(manager, *((block_manager + engine::SIDE0_MANAGER as isize) as *const usize)) != 0 {
                return;
            }
        }
    }
    let mut best: i32 = -10000;
    let mut chosen = 0usize;
    for k in 0..20usize {
        let q = side_player_slot(this, side_j, k);
        if read_i8(q + match_player::POSITION) < 0 || q == j {
            continue;
        }
        if chosen != 0 && (match_player_position(q) & 1 != 0 || match_player_position(q) & 4 != 0 || match_player_position(q) & 2 != 0) {
            continue;
        }
        let r = match_player_position(q);
        let mut v: i32 = if r & 0x40 != 0 {
            70000
        } else if r & 0x20 != 0 {
            50000
        } else if r & 0x10 != 0 {
            15000
        } else {
            -5000
        };
        if r & 0x80 != 0 || r & 0x800 != 0 {
            v += 5000;
        }
        if v > best || chosen == 0 {
            best = v;
            chosen = q;
        }
    }
    if chosen == 0 {
        // the original shows the error box and exits (0x5cc7d0, 0x5a8f60, exit(-1))
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        exe::game_fatal_cleanup();
        exe::shared_noop_ret0_5a8f60();
        exe::crt_exit(-1);
    }
    *((chosen + match_player::POSITION) as *mut i8) = position;
    if match_player_position(j) == 1 {
        let field = if side_j == 0 { 0xf59e } else { 0xf5a2 };
        *((this + field) as *mut usize) = chosen;
    }
}

/// Match engine: returns the direct marker of player `k` of side `lado_b` (index in the other side's block; 0xff for a
/// goalkeeper or a player off the pitch). First looks for the opponent in the mirrored role (defender vs forward, etc.)
/// and mirrored flank; if none, takes the nearest by the distance table using the mirrored formation position.
///
/// [pt] { 0x6d4330 } (match_eng.cpp): o marcador direto do jogador `k` do lado (indice no bloco do outro
/// lado; -1 para goleiro ou jogador fora). Primeiro procura o papel espelhado: cada setor do jogador
/// exige o oposto (2 ou 4 -> 0x40, 8 -> 0x20, 0x10 -> 0x10, 0x20 -> 8, 0x40 -> 4 ou 2) e um lado
/// espelhado (0x80 <-> 0x800, 0x100 <-> 0x400, 0x200 <-> 0x200). Sem ninguem, o mais perto pela
/// tabela de distancias, com a formacao espelhada (linha e coluna pela mascara, 0x5b73a0).
pub(crate) unsafe extern "thiscall" fn direct_marker(this: usize, side_b: i8, k: i8) -> u8 {
    let p = (this as isize + 0x4796 + 0x1be * (20 * side_b as isize + k as isize)) as usize;
    if read_i8(p + match_player::POSITION) < 0 || match_player_position(p) & 1 != 0 {
        return 0xff;
    }
    let other = (side_b == 0) as i8;
    let rp = match_player_position(p);
    let rpl = rp as u8;
    for k2 in 0..20usize {
        let q = side_player_slot(this, other, k2);
        if read_i8(q + match_player::POSITION) < 0 {
            continue;
        }
        let rq = match_player_position(q);
        let rql = rq as u8;
        if (rpl & 6 != 0 && rql & 0x40 == 0)
            || (rpl & 8 != 0 && rql & 0x20 == 0)
            || (rpl & 0x10 != 0 && rql & 0x10 == 0)
            || (rpl & 0x20 != 0 && rql & 8 == 0)
            || (rpl & 0x40 != 0 && rql & 6 == 0)
        {
            continue;
        }
        let pairs = [(0x80u16, 0x800u16), (0x100, 0x400), (0x200, 0x200), (0x400, 0x100), (0x800, 0x80)];
        if pairs.iter().any(|&(a, b)| rp & a != 0 && rq & b != 0) {
            return k2 as u8;
        }
    }
    // simplification: in the original sector and side start with stack garbage; they only stay without a value with an invalid mask
    let (mut setor, mut lado_m) = (0u16, 0u16);
    exe::formation_split_position_code(rp as u32, &mut setor, &mut lado_m);
    let row_p: i8 = match setor {
        2 => 0,
        4 => 1,
        8 => 2,
        0x10 => 3,
        0x20 => 4,
        0x40 => 5,
        _ => side_b,
    };
    let column_p: i8 = match lado_m {
        0x80 => 1,
        0x100 => 3,
        0x200 => 4,
        0x400 => 5,
        0x800 => 7,
        _ => 0,
    };
    let (mut linha_q, mut coluna_q) = (row_p, k);
    let mut best = f32::from_bits(0x447a0000); // 1000
    let mut chosen: i8 = -1;
    for k2 in 0..20usize {
        let q = side_player_slot(this, other, k2);
        if read_i8(q + match_player::POSITION) < 0 || match_player_position(q) & 1 != 0 {
            continue;
        }
        exe::formation_split_position_code(match_player_position(q) as u32, &mut setor, &mut lado_m);
        linha_q = match setor {
            2 => 6,
            4 => 5,
            8 => 4,
            0x10 => 3,
            0x20 => 2,
            0x40 => 1,
            _ => linha_q,
        };
        coluna_q = match lado_m {
            0x100 => 5,
            0x80 => 7,
            0x200 => 4,
            0x800 => 1,
            0x400 => 3,
            _ => coluna_q,
        };
        let dc = (column_p as i32 - coluna_q as i32).abs() as i8;
        let dr = (row_p as i32 - linha_q as i32).abs() as i8;
        let dist = if (0..=8).contains(&dc) && (0..=11).contains(&dr) {
            read_f32(G_CELL_DISTANCE_TABLE + 4 * (dr as usize * 9 + dc as usize))
        } else {
            read_f32(rdata::F32_1000)
        };
        if chosen == -1 || dist < best {
            best = dist;
            chosen = k2 as i8;
        }
    }
    chosen as u8
}

/// Fills `saida` with fine (27 x 36 grid) positions for drawing the match: the ball cell, and for each side and
/// tactical position the fine x/y, the player's id byte, his role and a presence mark. Each grid cell becomes 3 x 3
/// sub-cells: the ball holder takes the centre, the others draw a random free sub-cell (accepting an occupied one only
/// after 50 tries per occupant); which side places first in each cell is random.
///
/// [pt] { 0x6d1770 } (match_eng.cpp): posicoes finas (grade 27 x 36) de cada jogador para desenhar a
/// partida. `saida`: bola (x, y) e, para cada lado e posicao tatica (6 bytes a partir de
/// saida + 2 + 6 x (posicao + 11 x lado)): x e y finos, byte 0 do jogador, papel (0x717410) e
/// a marca 1. Cada casa da grade vira 3 x 3 subcasas: quem tem a bola fica no centro; os outros
/// sorteiam uma subcasa (nunca o centro da casa de quem tem a bola), aceitando uma ja ocupada
/// so depois de 50 tentativas por ocupante. O lado que comeca em cada casa e sorteado.
/// Saida nula ou jogador sem posicao: o original mostra uma caixa de erro e zera 0xb67a34 (aqui,
/// so zera; nunca ocorre em jogo).
pub(crate) unsafe extern "thiscall" fn fine_positions(this: usize, out: *mut u8) {
    if out.is_null() {
        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    }
    *out = read_i8(this + engine::BALL_X) as u8;
    *out.add(1) = read_i8(this + engine::BALL_Y) as u8;
    for i in 0..22 {
        *out.add(7 + 6 * i) = 0;
    }
    for x in 0..9i8 {
        for y in 0..12i8 {
            let (bx, by) = (3 * x + 1, 3 * y + 1);
            let home = occupancy_grid_cell(this, x, y) as usize;
            let mut ocupada = [0i8; 9];
            let first = random(2) as i8;
            for lado in [first, (first == 0) as i8] {
                let mut k: i8 = 0;
                while k < read_i8(home + (0x58 + lado as usize)) {
                    let p = *((home as isize + 4 * (k as isize + 11 * lado as isize)) as *const usize);
                    let pos = read_i8(p + match_player::POSITION);
                    if pos < 0 {
                        *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                        k += 1;
                        continue;
                    }
                    let owner = *((this + engine::BALL_HOLDER) as *const usize);
                    let (fx, fy) = if p == owner {
                        ocupada[4] = 1;
                        (bx, by)
                    } else {
                        let mut attempts: i32 = 0;
                        loop {
                            let r = random(3) as i8;
                            let c = random(3) as i8;
                            attempts += 1;
                            let owner = *((this + engine::BALL_HOLDER) as *const usize);
                            if owner != 0 && read_i8(owner + match_player::CELL_X) == x && read_i8(owner + match_player::CELL_Y) == y && r == 1 && c == 1 {
                                continue;
                            }
                            let i = (3 * r + c) as usize;
                            if ocupada[i] as i32 > (attempts as i16 as i32) / 50 {
                                continue;
                            }
                            ocupada[i] = 1;
                            break (r + bx - 1, c + by - 1);
                        }
                    };
                    let e = out.offset(6 * (pos as isize + 11 * lado as isize));
                    *e.add(2) = fx as u8;
                    *e.add(3) = fy as u8;
                    *e.add(4) = *(p as *const u8);
                    (e.add(5) as *mut u16).write_unaligned(match_player_position(p));
                    *e.add(7) = 1;
                    k += 1;
                }
            }
        }
    }
}

/// `roll_ball` (match_eng.cpp) with the commentary injected: a loose ball rolling. Only with no owner (0xf582), without
/// 0xf57a and with impulse (0x8eaa).
///
/// Stopped play (0x8eb2): zeroes the impulse and 0x8ea9. Otherwise, with impulse 1 the ball only moves if random(1) >=
/// random(2); impulse <= 0 does not move.
///
/// When it moves, it spends one impulse (and one of 0x8ea9, if any) and takes a step by the direction (0x8ed8,
/// degrees): (220, 320) x - 1; (40, 140) x + 1; (130, 230) y - 1; below 50 or above 310, y + 1. Off the pitch (x
/// outside 0..9 or y outside 0..12): ball out (`exe::cm3_match_engine_ball_out_of_play`) on the side of 0x8eab.
///
/// [pt] { 0x6bbe40 } (match_eng.cpp) com o comentario injetado: bola solta rolando. So sem dono
/// (0xf582), sem 0xf57a e com impulso (0x8eaa). Lance parado (0x8eb2): zera o impulso e 0x8ea9.
/// Senao, com impulso 1 a bola so anda se sorteio(1) >= sorteio(2); impulso <= 0 nao anda. Ao andar,
/// gasta um de impulso (e um de 0x8ea9, se houver) e da um passo pela direcao (0x8ed8, graus):
/// (220, 320) x - 1; (40, 140) x + 1; (130, 230) y - 1; abaixo de 50 ou acima de 310, y + 1.
/// Fora do campo (x fora de 0..9 ou y fora de 0..12): bola fora (0x6cd490) pelo lado de 0x8eab.
pub(crate) unsafe fn roll_loose_ball_with(this: usize, call: AddEventCall) {
    let b = |o: usize| (this + o) as *mut i8;
    if *((this + engine::BALL_HOLDER) as *const usize) != 0 || *((this + engine::SET_PIECE_TAKER) as *const usize) != 0 || *b(engine::BALL_ROLL) == 0 {
        return;
    }
    if *b(engine::RESTART_TYPE) != 0 {
        *b(engine::BALL_ROLL) = 0;
        *b(engine::BALL_HEIGHT) = 0;
        return;
    }
    let v = *b(engine::BALL_ROLL);
    let anda = if v >= 2 {
        true
    } else if v <= 0 {
        false
    } else {
        let a = random(v as i32 * v as i32);
        a >= random(2)
    };
    if anda {
        *b(engine::BALL_ROLL) = v.wrapping_sub(1);
        if *b(engine::BALL_HEIGHT) != 0 {
            *b(engine::BALL_HEIGHT) = (*b(engine::BALL_HEIGHT)).wrapping_sub(1);
        }
        let dir = *((this + engine::BALL_DIRECTION) as *const i16);
        if dir > 0xdc && dir < 0x140 {
            *b(engine::BALL_X) = (*b(engine::BALL_X)).wrapping_sub(1);
        } else if dir > 0x28 && dir < 0x8c {
            *b(engine::BALL_X) = (*b(engine::BALL_X)).wrapping_add(1);
        } else if dir > 0x82 && dir < 0xe6 {
            *b(engine::BALL_Y) = (*b(engine::BALL_Y)).wrapping_sub(1);
        } else if dir < 0x32 || dir > 0x136 {
            *b(engine::BALL_Y) = (*b(engine::BALL_Y)).wrapping_add(1);
        }
    }
    let (x, y) = (*b(engine::BALL_X), *b(engine::BALL_Y));
    if x < 0 || y < 0 || x >= 9 || y >= 12 {
        ball_out_with(this, *b(engine::LAST_POSSESSION_SIDE) as u8, call);
    }
}

/// loose ball (no owner at 0xf582, no 0xf57a) with momentum (0x8eaa). Play stopped: clears the momentum and 0x8ea9.
/// Momentum 1 only moves if draw(1) ≥ draw(2); momentum ≤ 0 does not move. When it moves, it spends one momentum (and
/// one of 0x8ea9) and takes a step along direction 0x8ed8: (220, 320) x − 1, (40, 140) x + 1, (130, 230) y − 1, below
/// 50 or above 310 y + 1. Out of the pitch: ball out (`exe::cm3_match_engine_ball_out_of_play`) for the side in 0x8eab
/// Original: `roll_ball` (0x6bbe40, match_eng.cpp, thiscall).
///
/// [pt] { 0x6bbe40 } (match_eng.cpp): bola rolando (ver `roll_loose_ball_with`).
pub(crate) unsafe extern "thiscall" fn roll_ball(this: usize) {
    roll_loose_ball_with(this, &mut |t, a| add_match_event(t, a))
}

/// who kicks off (0x8eae; outside 0..=1, side 0), ball at the centre (4, 5 or 6); every active player goes to their
/// formation square (table 0xf56e + 0x9e for side 0 or 0xf572 + 0xa6 for side 1, 216 bytes per position; low nibble x,
/// high nibble y, clamped to their own half) through 0x6f46b0 and faces forward (`exe::match_player_set_facing_direction`). On the kicking side, the
/// player closest to the ball (a position > 10 becomes −1) takes the ball (`take_ball`) and faces the square computed from
/// the last formation square used (same point: the low byte of 0x8ed8, sign-extended). With nobody available, it sets
/// byte 0 of the engine
/// Original: `kick_off` (0x6c51e0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6c51e0 } (match_eng.cpp): pontape inicial. Quem sai jogando: 0x8eae (fora de 0..=1, o lado
/// 0); bola no centro (4, 5 ou 6). Todos os ativos vao para a casa da formacao (tabela 0xf56e + 0x9e
/// do lado 0 ou 0xf572 + 0xa6 do lado 1, 216 bytes por posicao; nibble baixo x, alto y, cada um no
/// seu campo) e olham para a frente. Do lado que sai, o mais perto da bola (posicao > 10 vira -1)
/// pega a bola e olha para a casa calculada com a ultima casa da formacao usada (mesmo ponto: o
/// byte baixo de 0x8ed8). Sem ninguem, marca o byte 0 do motor.
pub(crate) unsafe extern "thiscall" fn kick_off(this: usize) {
    let mut sai = read_i8(this + engine::POSSESSION_SIDE);
    if !(0..=1).contains(&sai) {
        sai = 0;
    }
    *((this + engine::POSSESSION_SIDE) as *mut i8) = -1;
    *((this + engine::BALL_X) as *mut i8) = 4;
    *((this + engine::BALL_Y) as *mut i8) = if sai == 0 { 5 } else { 6 };
    // simplification: in the original x and y start with stack garbage; they are only read after some active one
    let (mut x, mut y) = (0i8, 0i8);
    for l in 0..2i8 {
        for k in 0..20 {
            let p = side_player_slot(this, l, k);
            let pos = read_i8(p + match_player::POSITION) as isize;
            if pos < 0 {
                continue;
            }
            let (tabela, campo) = if read_i8(p + match_player::SIDE) == 1 { (0xf572, 0xa6) } else { (0xf56e, 0x9e) };
            let home = *((*((this + tabela) as *const usize) as isize + 216 * pos + campo) as *const u8);
            x = (home & 0xf) as i8;
            y = (home >> 4) as i8;
            if read_i8(p + match_player::SIDE) == 1 {
                if y < 6 {
                    y = 6;
                }
            } else if y >= 6 {
                y = 5;
            }
            exe::match_player_move_to_grid_cell(p, x, y);
            if l == 0 {
                exe::match_player_set_facing_direction(p, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                exe::match_player_set_facing_direction(p, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
        }
    }
    let mut best = f32::from_bits(0x461c4000); // 10000
    let mut chosen = 0usize;
    for k in 0..20 {
        let p = side_player_slot(this, sai, k);
        let pos = read_i8(p + match_player::POSITION);
        if pos < 0 {
            continue;
        }
        if pos > 10 {
            *((p + match_player::POSITION) as *mut i8) = -1;
            continue;
        }
        if *((p + match_player::ENGINE) as *const usize) == 0 {
            continue;
        }
        let d = match_player_distance_to_cell(p, read_i8(this + engine::BALL_X), read_i8(this + engine::BALL_Y));
        if d < best {
            best = d;
            chosen = p;
        }
    }
    if chosen == 0 {
        *(this as *mut u8) = 1;
        return;
    }
    exe::match_player_take_possession(chosen);
    let dx = 4i8.wrapping_sub(x);
    let dy = if sai == 0 { 11i8.wrapping_sub(y) } else { y.wrapping_neg() };
    let v = if dx == 0 && dy == 0 {
        *((this + engine::BALL_DIRECTION) as *const u16)
    } else {
        *((GLOBAL_TABLE as isize + 2 * (dx as isize * 24 + dy as isize)) as *const u16)
    };
    *((chosen + match_player::FACING) as *mut i16) = v as u8 as i8 as i16;
}

/// Multiplies v by the float constant of the first rule (>, <, always) that matches attribute x, optionally rounding to
/// f32 as the original's fld/fmul/fstp.
///
/// [pt] Degrau de multiplicadores pelo atributo `x` (byte com sinal): a primeira regra que vale
/// multiplica pela constante float `k` (0: nenhuma). Regras: ('>', limite), ('<', limite) ou
/// ('*', _) (sempre). `arredondar`: o original guarda em float a cada passo (fld/fmul/fstp).
pub(crate) unsafe fn apply_attribute_step(v: f64, x: i8, rules: &[(char, i8, usize)], arredondar: bool) -> f64 {
    for &(op, lim, k) in rules {
        let vale = match op {
            '>' => x > lim,
            '<' => x < lim,
            _ => true,
        };
        if vale {
            if k == 0 {
                return v;
            }
            let n = v * read_f32(k) as f64;
            return if arredondar { n as f32 as f64 } else { n };
        }
    }
    v
}

/// If random(person attribute 0x21) < random(20), multiplies v by 0x96d248 (rounded to f32).
///
/// [pt] sorteio(atributo 0x21 da pessoa) < sorteio(20): multiplica por 0x96d248.
pub(crate) unsafe fn luck_multiplier(v: f64, person: usize) -> f64 {
    let a = random(read_i8(person + 0x21) as i32);
    if a < random(0x14) {
        (v * read_f32(rdata::F32_0_75) as f64) as f32 as f64
    } else {
        v
    }
}

/// Multiplies a signed engine word by k and stores the truncated result in another engine word.
///
/// [pt] Palavra do motor (com sinal) vezes `k`, truncada de volta na palavra `destino`.
pub(crate) unsafe fn scale_engine_word(this: usize, source: usize, k: f64, target: usize) {
    let v = *((this + source) as *const i16) as f64;
    *((this + target) as *mut i16) = ftol(v * k) as i16;
}

/// Computes a side's team strength in seven words (`engine::TEAM_STRENGTH` ... +0x1f2, step 4, + 2 x side) by summing over the 20
/// players of the block: wear, components combining the player's float attributes adjusted by his player data and
/// rolls, and per tactical role adjustments; then competition, club reputation and side adjustments. A player without
/// person (+0x69) clears the mouse-button global and aborts (the original shows an error box).
///
/// [pt] { 0x6c0c60 } (match_eng.cpp): forca do time por lado em sete palavras (0x1da, 0x1de ... 0x1f2,
/// de 4 em 4, + 2 x lado), somando os 20 jogadores do bloco. Por jogador ativo: desgaste,
/// componentes (combinacoes dos floats do jogador, ajustadas pelos atributos da pessoa e por
/// sorteios) e, pelo papel tatico (bits 1..0x40), ajuste da palavra 0x35 e as parcelas.
/// Depois, ajustes pela competicao, pela reputacao dos clubes e pelo lado.
/// Jogador sem ficha (0x69): o original mostra uma caixa de erro e zera 0xb67a34 (aqui, so isso).
pub(crate) unsafe extern "thiscall" fn team_strength(this: usize, side_b: i8) {
    let w = |o: usize| (this as isize + o as isize + 2 * side_b as isize) as *mut i16;
    for o in (0x1da..=0x1f2).step_by(4) {
        *w(o) = 0;
    }
    let f = |p: usize, o: usize| read_f32(p + o) as f64;
    let (k898, k8a8) = (read_f64(rdata::F64_0_01), read_f64(rdata::F64_100));
    let (cinquenta, meio) = (read_f32(rdata::F32_50) as f64, read_f32(rdata::F32_0_5) as f64);
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) < 0 {
            continue;
        }
        if *((p + match_player::STAFF) as *const usize) == 0 {
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            return;
        }
        let person = *((p + match_player::PLAYER) as *const usize);
        exe::match_player_drain_stamina(p, (30 - read_i8(person + player_sheet::STAMINA) as i32) as f32, -1);
        let role = match_player_position(p) as u8;
        let palavra5 = |pe: usize| *((pe + 5) as *const i16) as f64;
        // S: chute e finalizacao
        let mut s = ((((((f(p, match_player::TACKLING) + f(p, match_player::POSITIONING)) + f(p, match_player::PACE)) + f(p, match_player::ANTICIPATION)) + cinquenta) * meio + read_f32(rdata::F32_48) as f64)
            + f(p, match_player::STAMINA) * k898
            - k8a8) as f32 as f64;
        if person != 0 {
            let t = (read_i8(person + player_sheet::LEADERSHIP) >> 2) as f64 + s;
            let kw = if role & 4 != 0 || role & 2 != 0 {
                0x967880
            } else if role & 8 != 0 {
                0x96d3c0
            } else {
                0x967898
            };
            s = (palavra5(person) * read_f64(kw) + t) as f32 as f64;
            let default = [('>', 15, 0x96d3b8), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            let strong = [('>', 15, 0x96d3b0), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            s = apply_attribute_step(s, read_i8(person + player_sheet::MARKING), &default, true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::STAMINA), &[('>', 15, 0x96d3b8), ('<', 6, 0x96a32c), ('<', 10, 0x96b014)], true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::WORK_RATE), &strong, true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::TACKLING), &default, true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::ANTICIPATION), &default, true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::JUMPING), &strong, true);
            s = apply_attribute_step(s, read_i8(person + player_sheet::HEADING), &strong, true);
            let sum = read_i8(person + player_sheet::PLAYER_PACE).wrapping_add(read_i8(person + player_sheet::ACCELERATION));
            s = apply_attribute_step(s, sum, &[('>', 0x23, 0x96d3b0), ('<', 10, 0x96a32c), ('<', 0x14, 0x96d3b4)], true);
            s = luck_multiplier(s, person);
        }
        // T2
        let mut t2 = ((((((f(p, match_player::VISION) * 2.0 + f(p, match_player::PASSING) * 2.0) + f(p, match_player::FLAIR)) + f(p, match_player::HEADING)) + f(p, match_player::CROSSING) * read_f64(rdata::F64_1_5)) + read_f64(rdata::F64_50))
            * read_f64(rdata::F64_0_5)
            + f(p, match_player::STAMINA) * k898
            + read_f64(rdata::F64_57)
            - k8a8) as f32 as f64;
        if person != 0 {
            let kw = if role & 0x40 != 0 {
                0x967880
            } else if role & 0x20 != 0 {
                0x96d3a0
            } else if role & 0x10 != 0 {
                0x96d398
            } else if role & 8 != 0 {
                0x96a740
            } else {
                0x967898
            };
            t2 = (palavra5(person) * read_f64(kw) + t2) as f32 as f64;
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::WORK_RATE), &[('>', 15, 0x96d3b8), ('<', 6, 0x96d394)], true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::STAMINA), &[('>', 15, 0x96d3b0), ('<', 6, 0x96b014)], true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::ONE_ON_ONES), &[('>', 15, 0x96a33c), ('<', 6, 0x96a32c)], true);
            let three = [('>', 15, 0x96d390), ('<', 6, 0x96d38c), ('<', 10, 0x96d394)];
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::PASSING), &three, true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::VISION), &three, true);
            let sum = read_i8(person + player_sheet::TECHNIQUE).wrapping_add(read_i8(person + player_sheet::PLAYER_PACE)).wrapping_add(read_i8(person + player_sheet::DRIBBLING)).wrapping_add(read_i8(person + player_sheet::ACCELERATION));
            t2 = apply_attribute_step(t2, sum, &[('>', 0x4b, 0x96d388), ('>', 0x41, 0x96d3b8), ('>', 0x32, 0x96d3b0), ('>', 0x23, 0), ('>', 0x19, 0x96d3b4), ('*', 0, 0x96d394)], true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::ANTICIPATION), &[('>', 15, 0x96a33c), ('<', 10, 0x96b014), ('<', 6, 0x96a32c)], true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::MOVEMENT), &[('>', 15, 0x96d3b8), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)], true);
            t2 = apply_attribute_step(t2, read_i8(person + player_sheet::TEAMWORK), &[('>', 15, 0x96d3b0), ('<', 10, 0x96b014), ('<', 5, 0x96a32c)], true);
            t2 = luck_multiplier(t2, person);
        }
        // T3
        let k940 = read_f64(rdata::F64_1_5);
        let soma3 = (((f(p, match_player::DRIBBLING) + f(p, match_player::ACCELERATION)) + f(p, match_player::MOVEMENT) * k940) + f(p, match_player::FINISHING) * 2.0) + f(p, match_player::ANTICIPATION) * k940;
        let mut t3 = ((((soma3 + f(p, match_player::HEADING)) + f(p, match_player::PACE)) + read_f64(rdata::F64_50)) * read_f64(rdata::F64_0_5)
            + f(p, match_player::STAMINA) * k898
            + read_f64(rdata::F64_76)
            - k8a8) as f32 as f64;
        if person != 0 {
            let kw = if role & 0x40 != 0 {
                0x968918
            } else if role & 0x20 != 0 {
                0x96d378
            } else if role & 0x10 != 0 {
                0x96d1e0
            } else if role & 8 != 0 {
                0x96d370
            } else {
                0x967898
            };
            // in st0, without storing as float between the steps
            let mut v = palavra5(person) * read_f64(kw) + t3;
            let strong = [('>', 15, 0x96d3b0), ('<', 6, 0x96b014), ('<', 10, 0x96d3b4)];
            v = apply_attribute_step(v, read_i8(person + player_sheet::WORK_RATE), &strong, false);
            v = apply_attribute_step(v, read_i8(person + player_sheet::STAMINA), &strong, false);
            v = apply_attribute_step(v, read_i8(person + player_sheet::ONE_ON_ONES), &[('>', 15, 0x96d3b8), ('>', 10, 0x96d3b0), ('<', 6, 0x96d394)], false);
            v = apply_attribute_step(v, read_i8(person + player_sheet::LONG_SHOTS), &[('>', 15, 0x96d368), ('>', 10, 0x96d3b0), ('<', 6, 0x96d3b4)], false);
            let sum = read_i8(person + player_sheet::PLAYER_PACE).wrapping_add(read_i8(person + player_sheet::DRIBBLING)).wrapping_add(read_i8(person + player_sheet::ACCELERATION)).wrapping_add(read_i8(person + player_sheet::TECHNIQUE));
            v = apply_attribute_step(v, sum, &[('>', 0x4b, 0x96d388), ('>', 0x41, 0x96d3b8), ('>', 0x32, 0x96d3b0), ('<', 10, 0x96d320), ('<', 0x19, 0x96d364), ('*', 0, 0x96b014)], false);
            t3 = v as f32 as f64;
            t3 = apply_attribute_step(t3, read_i8(person + player_sheet::ANTICIPATION), &[('>', 15, 0x96d388), ('<', 6, 0x96a32c), ('<', 10, 0x96d360)], true);
            t3 = apply_attribute_step(t3, read_i8(person + player_sheet::MOVEMENT), &[('>', 15, 0x96d388), ('>', 10, 0x96d3b0), ('<', 6, 0x96d394)], true);
            t3 = apply_attribute_step(t3, read_i8(person + player_sheet::TECHNIQUE), &[('>', 15, 0x96d3b8)], true);
            t3 = luck_multiplier(t3, person);
        }
        // C1, C4 (stays in the x87, without rounding), C5, C6
        let c1 = ((((f(p, match_player::REFLEXES) + f(p, match_player::POSITIONING)) + f(p, match_player::HANDLING)) + cinquenta) * meio + cinquenta + f(p, match_player::STAMINA) * k898 - k8a8) as f32 as f64;
        let v4d = f(p, match_player::STAMINA);
        let c = exe::match_player_get_foul_chance(p, 0) as i8 as f64;
        let c4 = ((c + f(p, match_player::AGGRESSION)) + cinquenta) * meio + cinquenta + v4d * k898 - k8a8;
        let a = f(p, match_player::JUMPING) + f(p, match_player::POSITIONING);
        let dd = v4d * k898;
        let c5 = (((a + f(p, match_player::HEADING)) + cinquenta) * meio + cinquenta + dd - k8a8) as f32 as f64;
        let c6 = (((a + f(p, match_player::HANDLING)) + cinquenta) * meio + cinquenta + dd - k8a8) as f32 as f64;
        // role: (discount at 0x35, threshold, factors of S, T2, T3, C1, C4, C5, C6; 0 = absent term)
        let (desconto, limiar, ks): (i16, usize, [usize; 7]) = if role & 1 != 0 {
            (3, 0x96d35c, [0x96d330, 0, 0, 1, 0x967880, 0, 0x96d330])
        } else if role & 2 != 0 {
            (13, 0x96d358, [0x9689b8, 0x968918, 0x967880, 0x9689e0, 0x968968, 0x969040, 0x968968])
        } else if role & 4 != 0 {
            (13, 0x96d358, [1, 0x968f20, 0x967880, 0x968918, 0x968968, 0x969040, 0x968968])
        } else if role & 8 != 0 {
            (12, 0x96d354, [0x969520, 0x96d328, 0x96d340, 0x968f20, 0x968968, 0x969040, 0x968968])
        } else if role & 0x10 != 0 {
            (8, 0x96d350, [0x968e00, 0x969040, 0x96d338, 0x967888, 0x968968, 0x969040, 0x968968])
        } else if role & 0x20 != 0 {
            (11, 0x96d34c, [0x968f20, 0x968978, 0x9689d0, 0x968a98, 0x968968, 0x969040, 0x969510])
        } else if role & 0x40 != 0 {
            (0x19, 0x96d348, [0x967888, 0x968f88, 0x968940, 0x968a98, 0x968968, 0x968940, 0x968918])
        } else {
            continue;
        };
        let p35 = (p + match_player::MATCH_RATING) as *mut i16;
        let v = (*p35).wrapping_sub(desconto);
        *p35 = if v as f64 <= read_f32(limiar) as f64 { v.wrapping_add(5) } else { v.wrapping_sub(5) };
        let parcelas = [s, t2, t3, c1, c4, c5, c6];
        for (i, &kk) in ks.iter().enumerate() {
            if kk == 0 {
                continue;
            }
            // 1: no factor (S of role 4); C1 of role 1 counts double
            let x = match (kk, i) {
                (1, 3) => parcelas[3] + parcelas[3],
                (1, _) => parcelas[i],
                _ => parcelas[i] * read_f64(kk),
            };
            *w(0x1da + 4 * i) = (*w(0x1da + 4 * i)).wrapping_add(ftol(x) as i16);
        }
    }
    let partida = Fixture(*((this + engine::MATCH) as *const usize));
    // by competition (reads side 0’s word, writes the side’s one)
    if !partida.is_null() {
        let club = Club(partida.home_club());
        let game = partida.competition();
        if !club.is_null() && game != 0 {
            let comp = club.nation().addr();
            if *((game + competition::NATION) as *const usize) == comp && comp != 0 {
                let id = *(comp as *const u32);
                let g = |a: usize| *(a as *const u32);
                let k = if id == g(KEY_NATION_GERMANY) || id == g(KEY_NATION_SCOTLAND) || id == g(KEY_NATION_BELGIUM) {
                    0x96d390
                } else if id == g(KEY_NATION_SWEDEN) {
                    0x96b010
                } else if id == g(KEY_NATION_NORWAY) || id == g(KEY_NATION_ENGLAND) {
                    0x96d3b8
                } else if id == g(KEY_NATION_HOLLAND) || id == g(KEY_NATION_DENMARK) || id == g(KEY_NATION_PORTUGAL) || id == g(KEY_NATION_FRANCE) {
                    0x96a33c
                } else {
                    0
                };
                if k != 0 {
                    *w(0x1de) = ftol(*((this + engine::TEAM_STRENGTH_2) as *const i16) as f64 * read_f32(k) as f64) as i16;
                }
            }
        }
    }
    *w(0x1de) = ftol(*w(0x1de) as f64 * read_f32(rdata::F32_1_049999952) as f64) as i16;
    *w(0x1e2) = ftol(*w(0x1e2) as f64 * read_f32(rdata::F32_1_019999981) as f64) as i16;
    let game = partida.competition();
    let aplica = if game == 0 {
        true
    } else {
        let who = *(G_FRIENDLY as *const usize);
        if who == 0 {
            false
        } else {
            let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
            f(who, game) != 0
        }
    };
    if aplica {
        *w(0x1de) = ftol(*((this + engine::TEAM_STRENGTH_2) as *const i16) as f64 * read_f32(rdata::F32_1_100000024) as f64) as i16;
    }
    let (casa, fora) = (partida.home_club(), partida.away_club());
    if casa != 0 && fora != 0 && *((casa + club::REPUTATION) as *const i16) < 0x9c4 {
        *w(0x1de) = ftol(*w(0x1de) as f64 * read_f32(rdata::F32_1_039999962) as f64) as i16;
        *w(0x1e2) = ftol(*w(0x1e2) as f64 * read_f32(rdata::F32_1_149999976) as f64) as i16;
    }
    if partida.is_null() || casa == 0 || fora == 0 {
        return;
    }
    let dif = (*((casa + club::REPUTATION) as *const i16)).wrapping_sub(*((fora + club::REPUTATION) as *const i16));
    let (kf88, kafa8) = (read_f64(rdata::F64_1_02), read_f64(rdata::F64_1_01));
    if side_b == 0 {
        if dif > 1000 {
            scale_engine_word(this, 0x1de, kf88, 0x1de);
            scale_engine_word(this, 0x1e2, kf88, 0x1e2);
            scale_engine_word(this, 0x1da, kf88, 0x1da);
        } else if dif > -250 {
            scale_engine_word(this, 0x1de, kf88, 0x1de);
            scale_engine_word(this, 0x1e2, kafa8, 0x1e2);
            scale_engine_word(this, 0x1da, kafa8, 0x1da);
        } else {
            scale_engine_word(this, 0x1da, kf88, 0x1da);
            scale_engine_word(this, 0x1de, read_f64(rdata::F64_0_975), 0x1de);
        }
        let (k1e2, k1da) = if dif > 0x4e2 {
            (kf88, read_f64(rdata::F64_1_05))
        } else if dif > 0x2ee {
            (kf88, read_f64(rdata::F64_1_03))
        } else if dif > 0xfa {
            (kafa8, kf88)
        } else {
            return;
        };
        if read_i8(this + engine::SCORE) == 0 {
            scale_engine_word(this, 0x1e2, k1e2, 0x1e2);
        }
        scale_engine_word(this, 0x1da, k1da, 0x1da);
    } else if side_b == 1 && dif < -750 && read_i8(this + engine::SCORE_SIDE1) == 0 {
        scale_engine_word(this, 0x1e4, kf88, 0x1e4);
        scale_engine_word(this, 0x1dc, kafa8, 0x1dc);
    }
}

/// `centre_backs_without_conflict` on the side's formation
/// Original: `side_centre_backs_without_conflict` (0x6d5b20, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5b20 } (thiscall, ret 4, match_eng.cpp): os zagueiros do lado `lado` sem conflito (0x5b8dc0
/// na formação +0xaac5, ou +0x91e2 com lado 0).
pub(crate) unsafe extern "thiscall" fn side_centre_backs_without_conflict(m: usize, side: i8) -> u8 {
    exe::formation_has_no_wide_defenders(m + if side != 0 { 0xaac5 } else { 0x91e2 })
}

compare_value!(compare_side_centre_backs_without_conflict, side_centre_backs_without_conflict, IDX_SIDE_CENTRE_BACKS_WITHOUT_CONFLICT, (lado: i8) -> u8);

/// 1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al`
/// Original: `ball_goal_end` (0x6d58f0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d58f0 } (thiscall, match_eng.cpp): a bola numa área (casas centrais +0x8ea7 de 2 a 6): 1 na
/// área de cima (linha +0x8ea8 0 ou 1), −1 na de baixo (10 ou 11), senão 0. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn ball_goal_end(m: usize) -> i8 {
    let row = *((m + engine::BALL_Y) as *const i8);
    let column = *((m + engine::BALL_X) as *const i8);
    if (2..=6).contains(&column) {
        if (0..=1).contains(&row) {
            return 1;
        }
        if (10..=11).contains(&row) {
            return -1;
        }
    }
    0
}

/// the abandonment minute × 11 below the clock (16-bit); returns only `al`
/// Original: `match_abandoned` (0x6d5930, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5930 } (thiscall, match_eng.cpp): a partida foi abandonada? O minuto do abandono (+0xf638,
/// −1 sem abandono) × 11 abaixo do relógio (+0x8ed0), em 16 bits. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn match_abandoned(m: usize) -> u8 {
    let minute = *((m + engine::ABANDON_MINUTE) as *const i8);
    if minute == -1 {
        return 0;
    }
    ((minute as i16).wrapping_mul(11) < core::ptr::read_unaligned((m + engine::CLOCK) as *const i16)) as u8
}

/// `slots_without_conflict` on the side's formation
/// Original: `side_slots_without_conflict` (0x6d5af0, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d5af0 } (thiscall, ret 4, match_eng.cpp): as vagas do lado `lado` sem conflito (0x5b8960 na
/// formação +0xaac5, ou +0x91e2 com lado 0).
pub(crate) unsafe extern "thiscall" fn side_slots_without_conflict(m: usize, side: i8) -> u8 {
    exe::formation_has_no_wide_midfield_or_attack(m + if side != 0 { 0xaac5 } else { 0x91e2 })
}

compare_value!(compare_ball_goal_end, ball_goal_end, IDX_BALL_GOAL_END, () -> i8);
compare_value!(compare_match_abandoned, match_abandoned, IDX_MATCH_ABANDONED, () -> u8);
compare_value!(compare_side_slots_without_conflict, side_slots_without_conflict, IDX_SIDE_SLOTS_WITHOUT_CONFLICT, (lado: i8) -> u8);

/// with the day's event list (`engine::EVENT_LIST`) and the match, walks the events (`next_list_item`): 0x82 goes to the global list
/// [0xb67dc8] (0x68d0d0); 0x1e by the score, the clubs' strength and the other side's manager (`exe::non_player_attribute_rating`) marks the
/// event (+0xe, +0xf) or compares it with the club's strength; the others, from the match player (6 by marks
/// +0x24/+0x23; the rest by draw). Then, per side, each player's record (`exe::update_player_reputation_after_match`), the request to leave (event 0xa)
/// and dissatisfaction (event 0xb, rival, favourite/rejected `favourite_of`/`rejected_by`, preferences `exe::staff_dislikes_club`/`exe::staff_favours_club`), and the
/// average rating by minutes (±0.25 home/away) for the reputations (`exe::update_manager_reputation_after_match`, `exe::match_eng_compute_club_reputation`). First routine of the
/// **determinized original**: a side's average with no minutes and 4 bytes of the event built on the stack were
/// garbage; the zeroing at entry leaves them at zero in every mode. Compared through the tape (the record, the
/// reputations and the event copy are units; the event's content is checked)
/// Original: `match_wrap_up` (0x6d1cc0, match_eng.cpp (2.5 KB, called by the engine step 0x6b98c0), cdecl).
///
/// [pt] { 0x6d1cc0 } (cdecl, match_eng.cpp): ver `match_wrap_up_with`.
pub(crate) unsafe extern "C" fn match_wrap_up(m: usize) {
    match_wrap_up_with(m, &mut |r, t, a| dispatch_wrap_up_call(r, t, a))
}

/// the player's injury in the match. With more than 3 injuries in the match (`engine::INJURY_COUNT`) draws may avoid it; more than 6,
/// never. Registers it (`register_injury`), marks the culprit (+0x55, -10 at +0x31), takes stamina by severity (above 50: the
/// square; otherwise a draw), writes +0x59 and reduces +0xd5; with no stamina the player leaves, otherwise stops
/// (`exe::match_player_set_action_delay` with 5 or 3) by the person's strength. Compared through the tape when it runs outside another tape (the
/// injury list and the 0x68d0d0 copy); inside, it is a unit
/// Original: `injure` (0x6d0980, match_eng.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x6d0980 }: ver `injure_with`.
pub(crate) unsafe extern "thiscall" fn injure(m: usize, kind: u32, chance: u32, p: usize, o: usize) {
    injure_with(m, kind, chance, p, o, &mut |r, t, a| dispatch_injury_list_call(r, t, a))
}

/// The ordering of the day's matches (qsort): competition, record fields and the club name.
///
/// The order of the day's matches (0x69-byte records): by competition (+4), then +0x46 (byte), +0x36, +0x38 and +0x34
/// (words) and, when everything ties, by the name of the first club (+0x20 → +0x38) with the text comparison `exe::utils_string_compare`
/// (case-insensitive).
/// Original: `day_match_order` (0x6b6510, match_eng.cpp, cdecl).
///
/// [pt] { 0x6b6510 } (cdecl, qsort): a ordem das partidas do dia (registros de 0x69 bytes): pela
/// competição (+4), depois +0x46 (byte), +0x36, +0x38 e +0x34 (palavras) e, empatando tudo, pelo nome
/// do primeiro clube (+0x20 → +0x38) na comparação de textos 0x90c1d0 (sem maiúsculas).
pub(crate) unsafe extern "C" fn day_match_order(a: usize, b: usize) -> i32 {
    let d = core::ptr::read_unaligned((a + 4) as *const i32).wrapping_sub(core::ptr::read_unaligned((b + 4) as *const i32));
    if d != 0 {
        return d;
    }
    let d = *((a + 0x46) as *const i8) as i32 - *((b + 0x46) as *const i8) as i32;
    if d != 0 {
        return d;
    }
    for o in [0x36, 0x38, 0x34] {
        let d = core::ptr::read_unaligned((a + o) as *const i16) as i32 - core::ptr::read_unaligned((b + o) as *const i16) as i32;
        if d != 0 {
            return d;
        }
    }
    let name = |x: usize| core::ptr::read_unaligned((x + 0x20) as *const usize).wrapping_add(0x38);
    exe::utils_string_compare((name(a)) as *const u8, (name(b)) as *const u8, 1, 0)
}

/// The players' order on the engine's turn (qsort): the marked ones first, then by strength.
///
/// The order of the players on the engine's turn (`[0xb67a64]`): with `engine::RESTART_TYPE` set, the one in `engine::BALL_HOLDER` comes first; then
/// the one in `engine::ACTION_FIRST_PLAYER`; the rest by strength (+0x104 + +0x105) × (+0x3b × 0.005 + 0.5) × +0x4d (f32), truncated, from
/// highest to lowest.
/// Original: `action_order` (0x6bd840, match_eng.cpp, cdecl).
///
/// [pt] { 0x6bd840 } (cdecl, qsort, match_eng.cpp): a ordem dos jogadores na vez do motor ([0xb67a64]):
/// com +0x8eb2 ligado, o de +0xf582 vem primeiro; depois o de +0x91de; os demais pela força
/// (+0x104 + +0x105) × (+0x3b × 0,005 + 0,5) × +0x4d (f32), truncada, do maior para o menor.
pub(crate) unsafe extern "C" fn action_order(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let m = *(G_ACTION_ORDER_MATCH_ENGINE as *const usize);
    if *((m + engine::RESTART_TYPE) as *const u8) != 0 {
        let first = core::ptr::read_unaligned((m + engine::BALL_HOLDER) as *const usize);
        if pa == first {
            return -1;
        }
        if pb == first {
            return 1;
        }
    }
    let c = core::ptr::read_unaligned((m + engine::ACTION_FIRST_PLAYER) as *const usize);
    if c == pa {
        return -1;
    }
    if c == pb {
        return 1;
    }
    let strength = |p: usize| {
        let s = *((p + match_player::MOMENT_ANTICIPATION) as *const i8) as i32 + *((p + match_player::MOMENT_PACE) as *const i8) as i32;
        let w = core::ptr::read_unaligned((p + match_player::EFFECTIVE_ABILITY) as *const i16) as f64;
        ftol(s as f64 * (w * read_f64(rdata::F64_0_005) + read_f64(rdata::F64_0_5)) * core::ptr::read_unaligned((p + match_player::STAMINA) as *const f32) as f64)
    };
    strength(pb).wrapping_sub(strength(pa))
}

/// Decides whether a context commentary (league race, club news) may play now.
///
/// Ret 0xc: may context commentary `event` (0x21a3..=0x21af: the league race, the club's recent news) play now for
/// `side`? Outside the range 8000..=0x21e4, already played (table at +0x21e per side) or without a competition (`engine::MATCH`
/// → +0x14): no. It weighs the aggregate score (`exe::cm3_match_engine_get_goal_margin`), the club's row in the league table (`exe::comp_stats_find_club_entry`: +0x19/+0x41
/// and +0x2d/+0x55), the minute (`engine::CLOCK` / 11), the club's news in the last 10 or 30 days (`has_logged_story` on the media
/// `[0xb6b7c8]`) and the average of the side's 20 players (+0x47cb, step 0x1be, the marked ones at −0x1c); those in the
/// table write the value into `out`. The events in the range outside the 13 cases: yes.
/// Original: `context_commentary` (0x6d4b70, match_eng.cpp, thiscall).
///
/// [pt] { 0x6d4b70 } (thiscall, ret 0xc, match_eng.cpp): o comentário de contexto `evento` (0x21a3..=0x21af:
/// a corrida da liga, as notícias recentes do clube) pode tocar agora para o `lado`? Fora da faixa
/// 8000..=0x21e4, já tocado (tabela de +0x21e por lado) ou sem competição (+0x4792 → +0x14), não. Pesa
/// o placar agregado (0x6ce310), a linha do clube na tabela da liga (0x4a4850: +0x19/+0x41 e
/// +0x2d/+0x55), o minuto (+0x8ed0 / 11), as notícias do clube nos últimos 10 ou 30 dias (0x757da0 na
/// mídia [0xb6b7c8]) e a média dos 20 jogadores do lado (+0x47cb, passo 0x1be, os marcados em −0x1c);
/// os da tabela gravam o valor em `saida`. Os eventos da faixa fora dos 13 casos: sim.
pub(crate) unsafe extern "thiscall" fn context_commentary(m: usize, side: u32, event: u32, out: *mut i32) -> u32 {
    let today = G_CURRENT_DATE;
    let (mut d10, mut d30) = ([0u8; 8], [0u8; 8]);
    exe::date_sub_days(today, (&mut d10) as *mut [u8; 8] as usize, 10);
    exe::date_sub_days(today, (&mut d30) as *mut [u8; 8] as usize, 0x1e);
    let ev = event as u16 as i16 as i32;
    if !(8000..=0x21e4).contains(&ev) {
        return 0;
    }
    let l = side as u8 as i8;
    if *((m as isize + ((ev + l as i32 * 1000) * 4) as isize - 0x7ae2) as *const u32) != 0 {
        return 0;
    }
    let c = Fixture(*((m + engine::MATCH) as *const usize));
    if (c.competition() as u32) == 0 {
        return 0;
    }
    let (mut casa, mut fora, mut classificado, mut ida, mut fase3) = (0i8, 0i8, 0u8, 0u8, 0u8);
    let aggregate = aggregate_score(m, l, &mut casa, &mut fora, &mut classificado, &mut ida, &mut fase3, 0) as i8 as i32;
    let competition = *((*(G_COMP_OBJECT_TABLE as *const usize)) as *const usize).add(*(c.addr() as *const u32) as usize);
    let club = Club(*((c.addr() + if l != 0 { 0x20 } else { 0x1c }) as *const usize));
    let mut row = 0usize;
    if competition != 0 && *((competition + 8) as *const u32) != 0 {
        row = exe::comp_stats_find_club_entry(*((competition + 8) as *const usize), club);
    }
    let in_competition = || {
        let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
        f(competition, club.addr()) != 0
    };
    let news = |kind: u32, data: &[u8; 8]| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, *const [u8; 8]) -> u32 = core::mem::transmute(FN_MEDIA_HAS_LOGGED_STORY);
        let r = f(*(G_MEDIA as *const usize), 0, club.addr(), -1, kind, data) != 0;
        if r {
            crate::comparison::branch("context news");
        }
        r
    };
    // the average (16 bits) of the side’s marked players; None if there are none
    let average = || {
        let (mut s, mut n) = (0i32, 0i32);
        for k in 0..20 {
            let p = (m as isize + l as isize * 0x22d8 + 0x47cb + k * 0x1be) as usize;
            if *((p - 0x1c) as *const i8) < 0 {
                s += core::ptr::read_unaligned(p as *const i16) as i32;
                n += 1;
            }
        }
        if n == 0 { None } else { Some((s / n) as i16) }
    };
    let of_club = || (club.division().addr() as u32) == (c.competition() as u32);
    let of_competition = || club.continental_comp_id() == *(c.addr() as *const u32);
    let minute = core::ptr::read_unaligned((m + engine::CLOCK) as *const i16) as i32 / 11;
    // 0x6d587c / 0x6d5704: the table row (+o1, +o2) against random rolls; writes +o2 to `saida`
    let table = |o1: usize, o2: usize| -> u32 {
        if row == 0 || core::ptr::read_unaligned((row + o1) as *const i32) <= random(3) + 1 {
            return 0;
        }
        let value = core::ptr::read_unaligned((row + o2) as *const i32);
        if value <= random(5) + 8 {
            return 0;
        }
        if !out.is_null() {
            *out = value;
        }
        1
    };
    let until_average = |limit: i16| {
        crate::comparison::branch("context average");
        average().is_some_and(|v| v >= limit) as u32
    };
    let end = || average().map_or(1, |v| (v <= 0x1a2b) as u32);
    let nao_trava = if fase3 as i8 != -3 { 3 } else { 0 };
    match ev {
        0x21a3 => {
            if !of_club() || aggregate <= random(3) || !in_competition() {
                return 0;
            }
            if news(0x61, &d10) && average().is_some_and(|v| v > 8000) {
                return 1;
            }
            if news(0x62, &d10) && average().is_some_and(|v| v > 0x1e46) {
                return 1;
            }
            if !news(99, &d10) {
                return 0;
            }
            until_average(0x1d4d)
        }
        0x21a4 | 0x21a5 => {
            if ev == 0x21a4 {
                if !of_competition() || aggregate <= random(nao_trava) {
                    return 0;
                }
            } else if !of_club() || aggregate <= random(3) {
                return 0;
            }
            let (kind, data) = if ev == 0x21a4 { (0x5c, &d30) } else { (0x5f, &d10) };
            if !in_competition() || !news(kind, data) {
                return 0;
            }
            until_average(0x1d4d)
        }
        0x21a6 | 0x21a7 | 0x21a9 => {
            if !of_club() || random(-3) <= aggregate || minute < 0x1f || !in_competition() {
                return 0;
            }
            if minute == 0x1f {
                crate::comparison::branch("context minute 31");
            }
            let tipos: &[u32] = match ev {
                0x21a6 => &[0x3f],
                0x21a7 => &[0x61, 0x62, 0x41, 99],
                _ => &[0x5f, 0x40],
            };
            if !tipos.iter().any(|&t| news(t, &d10)) {
                return 0;
            }
            end()
        }
        0x21a8 => {
            if !of_competition() || random(-nao_trava) <= aggregate || !in_competition() || !news(0x5c, &d30) {
                return 0;
            }
            end()
        }
        0x21aa => {
            if !of_club() || aggregate <= random(3) || minute < 0x1f || !in_competition() {
                return 0;
            }
            if !news(0x3f, &d10) && !news(0x60, &d10) {
                return 0;
            }
            until_average(0x1b59)
        }
        0x21ab => {
            if !of_club() || aggregate <= random(2) + 1 || minute <= 0x1e {
                return 0;
            }
            table(0x19, 0x41)
        }
        0x21ac | 0x21ad => {
            if !of_club() {
                return 0;
            }
            if ev == 0x21ac {
                if -1 - random(2) <= aggregate {
                    return 0;
                }
            } else if aggregate <= random(3) + 1 {
                return 0;
            }
            if minute < 0x1f {
                return 0;
            }
            table(0x2d, 0x55)
        }
        0x21ae => {
            if !of_club() || random(-3) <= aggregate || minute < 0x1f || (minute > 0x4b && aggregate < -1) {
                return 0;
            }
            table(0x19, 0x41)
        }
        0x21af => {
            if !of_club() {
                return 0;
            }
            match aggregate {
                -1 if minute > 0x50 => table(0x19, 0x41),
                -2 if minute > random(10) + 0x46 => table(0x19, 0x41),
                _ => 0,
            }
        }
        _ => 1,
    }
}

/// the match referee (`engine::REFEREE_ID`, id ≠ −1), except in friendly competitions (`exe::friendly_is_friendly_competition` on [0xae2c74]). Cards (`engine::CARDS` and
/// `engine::CARDS_SIDE1` on both sides) and fouls (`engine::FOULS` and `engine::FOULS_SIDE1`); the grade starts at `engine::REFEREE_RATING` / 1000, loses 2 with more than 2
/// cards, 1 with 2, gains 1 with none; loses 1 with more than 5 fouls, gains 1 with fewer than 3; below 4 becomes 4
/// (except 1 in 500), above 8 becomes 8 (except 1 in 25), and stays within 1 to 10. Records through `record_referee_grade` on
/// [0xb74340] (8-byte record: id, cards, fouls, grade; the 8th byte is stack garbage and is not read). Compared side by
/// side (draws and the referee's record)
/// Original: `referee_grade` (0x6d4a20, match_eng.cpp (336 bytes, called by 0x6b98c0), thiscall).
///
/// [pt] { 0x6d4a20 } (match_eng.cpp, chamada pela 0x6b98c0): a nota do arbitro da partida (+0xf5b6, id
/// != -1), fora das competicoes amistosas (0x5c6710 em [0xae2c74]). Os cartoes (+0xf5cb e +0xf601 dos
/// dois lados) e as faltas (+0xf5cc e +0xf602); a nota parte de +0xf5ba / 1000, perde 2 com mais de 2
/// cartoes, 1 com 2, ganha 1 sem cartao; perde 1 com mais de 5 faltas, ganha 1 com menos de 3. Abaixo
/// de 4 vira 4 (salvo 1 em 500), acima de 8 vira 8 (salvo 1 em 25), e fica entre 1 e 10. Grava pela
/// 0x79ce10 em [0xb74340].
pub(crate) unsafe extern "thiscall" fn referee_grade(m: usize) {
    let referee = *((m + engine::REFEREE_ID) as *const usize);
    if referee == 0 || *(referee as *const i32) == -1 {
        return;
    }
    let partida = Fixture(*((m + engine::MATCH) as *const usize));
    if partida.is_null() {
        return;
    }
    let comp = partida.competition();
    if comp == 0 {
        return;
    }
    let amistosos = *(G_FRIENDLY as *const usize);
    if amistosos != 0 {
        if exe::friendly_is_friendly_competition(amistosos, comp) != 0 {
            crate::comparison::branch("referee rating friendly");
            return;
        }
    }
    let cards = (*((m + engine::CARDS_SIDE1) as *const u8)).wrapping_add(*((m + engine::CARDS) as *const u8)) as i8;
    let faltas = (*((m + engine::FOULS_SIDE1) as *const u8)).wrapping_add(*((m + engine::FOULS) as *const u8)) as i8;
    let mut rating = (*((m + engine::REFEREE_RATING) as *const i16) as i32 / 1000) as i8;
    if cards > 2 {
        rating = rating.wrapping_sub(2);
    } else if cards > 1 {
        rating = rating.wrapping_sub(1);
    } else if cards == 0 {
        rating = rating.wrapping_add(1);
    }
    if faltas > 5 {
        rating = rating.wrapping_sub(1);
    } else if faltas < 3 {
        rating = rating.wrapping_add(1);
    }
    if rating < 4 && random(500) != 0 {
        rating = 4;
    } else if rating > 8 && random(0x19) != 0 {
        rating = 8;
    } else if rating < 1 {
        crate::comparison::branch("referee rating minimum");
        rating = 1;
    } else if rating > 10 {
        crate::comparison::branch("referee rating maximum");
        rating = 10;
    }
    let mut r = [0u8; 8];
    r[..4].copy_from_slice(&(*(referee as *const i32)).to_le_bytes());
    r[4] = cards as u8;
    r[5] = faltas as u8;
    r[6] = rating as u8;
    exe::officialsmanager_record_match_stats(*(G_OFFICIALS_MANAGER as *const usize), r.as_ptr());
}

/// the match without moves, in periods of two rounds. Per period, the team strength (0x6c0c60) with four pairs clamped
/// between half and double, and the side that starts; per round, three attacks with a duel of an active player, the
/// second and third duel (0x6c2880), the wear (0x6c2750), the other side's cards (0x6c2310; the second yellow sends
/// off, 0x6cda20; more than 4 sent off: abandonment 3-0), the score factor, the contest (0x6c06b0/0x6c07a0/0x6c0a70),
/// the shot (0x6c23f0, 0x6c0890/0x6c0980) and the goal (0x6c2050, keepers `player_table`, commentary 0x1fcc/0x1fcf, or 0x1fce
/// on a missed penalty). At minutes 45/90/105/120 the scores (+0x45/+0x43/+0x47), extra time, golden goal and
/// penalties; the aggregate; the home goal; the injuries (`injure`); the ratings (friendly: by the club's reputation).
/// Used to read high bytes of stack dwords with no value (determinized original). Compared through the tape
/// (commentary, injuries and the player-leaving calls are units)
/// Original: `quick_simulation` (0x6beeb0, match_eng.cpp (6 KB, called by the engine step 0x6b98c0), thiscall).
///
/// [pt] { 0x6beeb0 } (match_eng.cpp): ver `quick_simulation_with`.
pub(crate) unsafe extern "thiscall" fn quick_simulation(m: usize) {
    quick_simulation_with(m, &mut |r, t, a| dispatch_tackle_calls(r, t, a))
}

/// Returns the player's entry in the engine's per-side position table (`engine::POSITION_TABLE`, 11 words by tactical position, from the
/// loaded tactic) [inference: the role code of his position; 1 = goalkeeper].
/// Original: `table_a` (0x6c38c0, match_eng.cpp).
///
/// [pt] { 0x6c38c0 } (match_eng.cpp)
pub(crate) unsafe extern "thiscall" fn table_a(this: usize, player: usize) -> u16 {
    table_by_position(this, player, 0x8ebc)
}

/// Returns the player's value in tactical table B (per-side table of 11 words at engine + 0x8ec4), by his position
/// 0..=10; 0 outside.
/// Original: `table_b` (0x6c38f0, match_eng.cpp).
///
/// [pt] { 0x6c38f0 } (match_eng.cpp)
pub(crate) unsafe extern "thiscall" fn table_b(this: usize, player: usize) -> u16 {
    table_by_position(this, player, 0x8ec4)
}

/// Distance of row `v` from side `lado`'s line (side 1: engine `engine::OFFSIDE_LINE_SIDE1` - v; otherwise v - engine `engine::OFFSIDE_LINE`) [inference:
/// the side's defensive line].
/// Original: `side_difference` (0x6c4180, match_eng.cpp).
///
/// [pt] { 0x6c4180 } (match_eng.cpp): diferenca entre um byte do lado e o valor dado.
pub(crate) unsafe extern "thiscall" fn side_difference(this: usize, side: u8, v: u8) -> u8 {
    if side == 1 {
        (read_i8(this + engine::OFFSIDE_LINE_SIDE1) as u8).wrapping_sub(v)
    } else {
        v.wrapping_sub(read_i8(this + engine::OFFSIDE_LINE) as u8)
    }
}

/// Returns 1 if cell (b, c) lies in the penalty-area zone of side `lado`'s own goal: column b in 2..=6 and row c in
/// 10..=11 for side 1 or 0..=1 for side 0.
/// Original: `band` (0x6cb960, match_eng.cpp).
///
/// [pt] { 0x6cb960 } (match_eng.cpp): faixa 2..=6 em `b` e, conforme o lado, 10..=11 ou 0..=1 em `c`.
pub(crate) unsafe extern "stdcall" fn band(side: u8, b: i8, c: i8) -> u8 {
    let target = if side == 1 { 10..=11 } else { 0..=1 };
    ((2..=6).contains(&b) && target.contains(&c)) as u8
}

/// Match engine: returns the direction (angle word from the global direction table) from cell (a, b) to cell (c, d);
/// for the same cell, the current ball direction (`engine::BALL_DIRECTION`).
/// Original: `by_difference` (0x6cb6c0, match_eng.cpp).
///
/// [pt] { 0x6cb6c0 } (match_eng.cpp): palavra pela diferenca de duas coordenadas.
pub(crate) unsafe extern "thiscall" fn by_difference(this: usize, a: i8, b: i8, c: i8, d: i8) -> u16 {
    let (dl, dc) = (c.wrapping_sub(a), d.wrapping_sub(b));
    if dl == 0 && dc == 0 {
        return *((this + engine::BALL_DIRECTION) as *const u16);
    }
    *((GLOBAL_TABLE as isize + 2 * (dl as isize * 24 + dc as isize)) as *const u16)
}

/// Match engine: tells whether row `v` is in the final third near the goal side `lado` attacks (side 1: row < 3; side
/// 0: row > 8) [inference].
/// Original: `limit` (0x6cb930, match_eng.cpp).
///
/// [pt] { 0x6cb930 } (match_eng.cpp)
pub(crate) unsafe extern "stdcall" fn limit(side: u8, v: i8) -> u8 {
    (if side == 1 { v < 3 } else { side == 0 && v > 8 }) as u8
}

/// Match engine: the distance in rows between player `jogador` (+0x103) and his side's line (`engine::OFFSIDE_LINE` / `engine::OFFSIDE_LINE_SIDE1`)
/// [inference: the defensive/offside line]; 0 for the ball holder (`engine::BALL_HOLDER`).
/// Original: `player_difference` (0x6c4130, match_eng.cpp).
///
/// [pt] { 0x6c4130 } (match_eng.cpp): como `side_difference`, com o valor e o lado tirados do jogador;
/// 0 se o jogador for o guardado em 0xf582.
pub(crate) unsafe extern "thiscall" fn player_difference(this: usize, player: usize) -> u8 {
    if player == *((this + engine::BALL_HOLDER) as *const usize) {
        return 0;
    }
    let v = read_i8(player + match_player::CELL_Y) as u8;
    if side_of_player(player) == 1 {
        (read_i8(this + engine::OFFSIDE_LINE_SIDE1) as u8).wrapping_sub(v)
    } else {
        v.wrapping_sub(read_i8(this + engine::OFFSIDE_LINE) as u8)
    }
}

/// Match engine: tells whether player `jogador` is his side's player nearest to the ball (`engine::NEAREST_TO_BALL` / `engine::NEAREST_TO_BALL_SIDE1`); for a
/// goalkeeper, whether the side's goalkeeper is nearest (`engine::GOALKEEPER_NEAREST` / `engine::GOALKEEPER_NEAREST_SIDE1`).
/// Original: `side_byte` (0x6cfda0, match_eng.cpp).
///
/// [pt] { 0x6cfda0 } (match_eng.cpp)
pub(crate) unsafe extern "thiscall" fn side_byte(this: usize, player: usize) -> u8 {
    let s = side_of_player(player);
    if exe::match_player_get_position(player) == 1 {
        if s == 1 {
            return read_i8(this + engine::GOALKEEPER_NEAREST_SIDE1) as u8;
        }
        if s == 0 {
            return read_i8(this + engine::GOALKEEPER_NEAREST) as u8;
        }
    }
    let stored = *((this + if s == 1 { 0xf58a } else { 0xf586 }) as *const usize);
    (player == stored) as u8
}

/// Returns 1 if match player `j` can be chosen: not standing on cell (b, c), busy counter +0x2b below 2, neither the
/// ball holder (engine `engine::BALL_HOLDER`) nor the set-piece taker (`engine::SET_PIECE_TAKER`), a nonzero marking rating (`field_by_position`) and no set piece
/// pending (engine `engine::RESTART_TYPE` zero).
/// Original: `can_choose` (0x6bce20, match_eng.cpp).
///
/// [pt] { 0x6bce20 } (match_eng.cpp): 1 se o jogador pode ser escolhido (fora do ponto (b, c), campo
/// 0x2b abaixo de 2, nao e um dos guardados em 0xf582/0xf57a, 0x6f4600 nao zero e 0x8eb2 zerado).
/// Devolve eax inteiro.
pub(crate) unsafe extern "thiscall" fn can_choose(this: usize, j: usize, b: i8, c: i8) -> u32 {
    if j == 0 || (read_i8(j + match_player::CELL_X) == b && read_i8(j + match_player::CELL_Y) == c) || read_i8(j + match_player::BUSY) >= 2 {
        return 0;
    }
    if j == *((this + engine::BALL_HOLDER) as *const usize) || j == *((this + engine::SET_PIECE_TAKER) as *const usize) {
        return 0;
    }
    (exe::match_player_get_marking_rating(j) != 0 && read_i8(this + engine::RESTART_TYPE) == 0) as u32
}

