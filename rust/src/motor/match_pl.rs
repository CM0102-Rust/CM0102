use super::*;

//
// The game’s x87 runs in double precision (Windows default): each operation rounds to 53 bits,
// as f64 in SSE2. What matters to reproduce is where the original rounds to float (32 bits):
// `fstp`+`fld` compares the already rounded value; `fst` followed by a comparison compares the 53-bit one.
// The constants are read from the game’s own .rdata.

/// Drains the match player's stamina (+0x4d, float, never below 0): code -1 subtracts `x` times a factor by the current
/// band (12, 10, 8 or 6 / 11); other codes subtract (200 - 8.5 x float +0xd5) times a per-code factor.
///
/// [pt] { 0x6f56c0 } (match_pl.cpp): desgaste do jogador, float em 0x4d (nunca abaixo de zero).
/// codigo -1: `x` x (12, 10, 8 ou 6 pela faixa do valor atual: acima de 9500, 8750, 7750) / 11.
/// Outros codigos: (200 - 8,5 x float de 0xd5) x fator do codigo (switch do original).
pub(crate) unsafe extern "thiscall" fn tire_fatigue(this: usize, x: f32, code: i16) {
    let field = (this + match_player::STAMINA) as *mut f32;
    let v = *field;
    let zero = read_f32(rdata::F32_0) as f64;
    if code == -1 {
        let k = if v > read_f32(rdata::F32_9500) {
            0x96d690
        } else if v > read_f32(rdata::F32_8750) {
            0x968a78
        } else if v > read_f32(rdata::F32_7750) {
            0x96d784
        } else {
            0x969084
        };
        let t = x as f64 * read_f32(k) as f64 * read_f32(rdata::F32_0_09090909362) as f64;
        *field = (v as f64 - t) as f32;
        if (*field as f64) < zero {
            *field = 0.0; // mov dword ptr [ecx + 0x4d], 0
        }
    } else {
        let factor = match code {
            19 | 33 => 0x96d770,
            21 | 36 | 46 => 0x96d778,
            _ => 0x96d768,
        };
        let t = (read_f64(rdata::F64_200) - read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64 * read_f64(rdata::F64_8_5)) * read_f64(factor);
        let r = v as f64 - t;
        *field = r as f32;
        if r < zero {
            *field = 0.0;
        }
    }
}

/// Game CRT _ftol (0x945640): truncates f64 to i32 (low half of i64); out of range or NaN gives 0 like x87 fistp.
///
/// [pt] `_ftol` { 0x945640 }: trunca para inteiro (64 bits; o jogo usa a parte baixa). Fora da faixa
/// (infinito, NaN, |x| >= 2^63) o `fistp` do x87 grava o inteiro indefinido, de parte baixa 0.
pub(crate) fn ftol(x: f64) -> i32 {
    if x.is_finite() && x.abs() < 9.223372036854775808e18 {
        x as i64 as i32
    } else {
        0
    }
}

/// Is the player facing cell (a, b)? Compares his direction (+0x19a) with the table angle to the target, with a
/// tolerance of +0x10c x 2.25 + 45 degrees. Returns 1 or 0 (0 for his own cell).
///
/// [pt] { 0x6fa140 } (match_pl.cpp): o jogador esta virado para (a, b)? Compara a direcao dele (0x19a)
/// com o angulo da tabela, com tolerancia de 0x10c x 2,25 + 45 graus.
pub(crate) unsafe extern "thiscall" fn facing(this: usize, a: i8, b: i8) -> u8 {
    let (dl, dc) = (a.wrapping_sub(read_i8(this + match_player::CELL_X)), b.wrapping_sub(read_i8(this + match_player::CELL_Y)));
    if dl == 0 && dc == 0 {
        return 0;
    }
    let dir = *((this + match_player::FACING) as *const i16);
    let target = *((GLOBAL_TABLE as isize + 2 * (dl as isize * 24 + dc as isize)) as *const i16);
    if dir == target {
        return 1;
    }
    let tolerancia = ftol(read_i8(this + match_player::MOMENT_AGILITY) as f64 * read_f64(rdata::F64_2_25) + read_f64(rdata::F64_45)) as i16;
    let mut d = (dir as i32 - target as i32).abs();
    if d as i16 > 0xb4 {
        d = 0x168 - d;
    }
    ((d as i16) < tolerancia) as u8
}

/// Cell of the match-engine distance table at 0xa15e44 (9 per row) for row/column deltas, abs then signed low byte like
/// the original.
///
/// [pt] Celula da tabela de distancias (0xa15e44, 9 por linha). Diferencas com o mesmo calculo do
/// original: valor absoluto em 32 bits e depois o byte baixo com sinal.
pub(crate) unsafe fn distance_table_cell(dl: i32, dc: i32) -> f32 {
    let row = dl.abs() as i8 as isize;
    let col = dc.abs() as i8 as isize;
    *((0xa15e44 + 4 * (row * 9 + col)) as *const f32)
}

/// Match player: the distance (from the distance table) from the player's cell (+0x102, +0x103) to cell (a, b).
///
/// [pt] { 0x6f09b0 } (match_pl.cpp): distancia (tabela) do jogador ao ponto (a, b).
pub(crate) unsafe extern "thiscall" fn distance_to_point(this: usize, a: i8, b: i8) -> f32 {
    distance_table_cell(read_i8(this + match_player::CELL_Y) as i32 - b as i32, read_i8(this + match_player::CELL_X) as i32 - a as i32)
}

/// Distance from the player to the goal he attacks (column 4; row 11 for side 0, row 0 for the other).
/// Original: `distance_to_goal` (0x6fa1e0, thiscall, `float` in st0).
///
/// [pt] { 0x6fa1e0 } (match_pl.cpp): distancia ao gol que o jogador ataca (coluna 4; linha 11 do lado 0, 0 do outro).
pub(crate) unsafe extern "thiscall" fn distance_to_goal(this: usize) -> f32 {
    let target = if side_of_player(this) == 0 { 0xb } else { 0 };
    distance_table_cell(read_i8(this + match_player::CELL_Y) as i32 - target, read_i8(this + match_player::CELL_X) as i32 - 4)
}

/// Match engine: for player `j`, returns 0 if he is in his own half or his cell is marked, or if an active opposing
/// outfield player is at least as close to goal as his limit (his distance to goal + 0.5, + 1.0 outside the box); 2 if
/// a team-mate is strictly closer; otherwise 1.
///
/// [pt] { 0x6cb780 } (match_eng.cpp): 0 se o jogador esta impedido de seguir (0x70dfa0, marca do
/// motor na casa dele) ou se algum adversario ativo esta a distancia do gol <= limite; 2 se algum
/// colega ativo esta estritamente mais perto; senao 1. Limite: distancia dele ao gol + 0,5, e mais
/// 1,0 fora da area (arredondado para float a cada passo, como o `fstp` do original). Os blocos
/// de 20 jogadores de cada lado ficam em motor + 0x4796 + 8920 x lado, de 0x1be em 0x1be.
pub(crate) unsafe extern "thiscall" fn closest_to_goal(this: usize, j: usize) -> u8 {
    if exe::match_player_is_in_own_half(j) != 0 {
        return 0;
    }
    let s = side_of_player(j);
    let other = (s == 0) as isize;
    let home = this as isize + other + 2 * 45 * (read_i8(j + match_player::CELL_Y) as isize + 12 * read_i8(j + match_player::CELL_X) as isize) + 0x21b6;
    if *(home as *const u8) != 0 {
        return 0;
    }
    let mut limit = (exe::match_player_distance_to_opponent_goal(j) as f64 + read_f64(rdata::F64_0_5)) as f32;
    if exe::match_player_can_see_opponent_goal(j) == 0 {
        limit = (limit as f64 + read_f64(rdata::F64_1)) as f32;
    }
    let row: i8 = if s == 0 { 0xb } else { 0 };
    let block = |l: isize| (this as isize + 8920 * l + 0x4796) as usize;
    for k in 0..20 {
        let p = block(other) + k * 0x1be;
        if read_i8(p + match_player::POSITION) >= 0 && exe::match_player_get_position(p) != 1 && !(exe::match_player_distance_to_cell(p, 4, row) > limit) {
            return 0; // C0|C3: less, equal or unordered
        }
    }
    for k in 0..20 {
        let p = block(s as isize) + k * 0x1be;
        if read_i8(p + match_player::POSITION) >= 0 && exe::match_player_get_position(p) != 1 && !(exe::match_player_distance_to_cell(p, 4, row) >= limit) {
            return 2; // C0: less than or unordered
        }
    }
    1
}

/// Returns the 64-bit tactical instruction mask of position `posicao` ([this + 0x188b + 8 x position]).
///
/// [pt] { 0x5b5270 }: marcas taticas (64 bits) de uma posicao: [this + 0x188b + 8 x posicao].
pub(crate) unsafe extern "thiscall" fn position_marks(this: usize, position: i8) -> u64 {
    *((this as isize + 0x188b + 8 * position as isize) as *const u64)
}

/// Calls `exe::formation_get_player_instructions` on the player's side tactics block (engine `engine::TACTICS` + 6371 x side) for the player's position byte
/// +0x19; returns the 64-bit instruction flags.
///
/// [pt] Marcas da posicao do jogador no bloco do lado dele (0x91e2 + 6371 x lado), por 0x5b5270.
pub(crate) unsafe fn player_position_instructions(j: usize) -> u64 {
    let block = (*((j + match_player::ENGINE) as *const usize) as isize + 0x91e2 + 6371 * side_of_player(j) as isize) as usize;
    exe::formation_get_player_instructions(block, read_i8(j + match_player::POSITION))
}

/// True when the player has an engine and its position word (FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION) is 1 [inference:
/// 1 = goalkeeper].
///
/// [pt] `table_a` do motor do jogador e 1 (pelo endereco do jogo); sem motor, nao.
pub(crate) unsafe fn is_goalkeeper_role(j: usize) -> bool {
    let engine = *((j + match_player::ENGINE) as *const usize);
    engine != 0 && exe::cm3_match_engine_get_player_position(engine, j) == 1
}

/// 1 (0x20000); 0 if `table_a` = 1; 2 (0x40000000); 0
/// Original: `attacking_role` (0x716270, the player's marks (block 0x91e2 + 6371 × side), thiscall, `al`).
///
/// [pt] { 0x716270 } (match_pl.cpp): papel no ataque pelas marcas da posicao: 1 (marca 0x20000), 0 se
/// `table_a` = 1, 2 (marca 0x40000000) ou 0.
pub(crate) unsafe extern "thiscall" fn attacking_role(j: usize) -> u8 {
    if player_position_instructions(j) & 0x20000 != 0 {
        return 1;
    }
    if is_goalkeeper_role(j) {
        return 0;
    }
    if player_position_instructions(j) & 0x4000_0000 != 0 { 2 } else { 0 }
}

/// 0 if `table_a` = 1; 3 (0x80000000); 4 (0x80000); 0
/// Original: `defending_role` (0x716310, thiscall, `al`).
///
/// [pt] { 0x716310 } (match_pl.cpp): papel na defesa: 0 se `table_a` = 1; 3 (marca 0x80000000),
/// 4 (marca 0x80000) ou 0.
pub(crate) unsafe extern "thiscall" fn defending_role(j: usize) -> u8 {
    if is_goalkeeper_role(j) {
        return 0;
    }
    let m = player_position_instructions(j);
    if m & 0x8000_0000 != 0 {
        3
    } else if m & 0x80000 != 0 {
        4
    } else {
        0
    }
}

/// Works out where player `j` should stand (writes x and y): from the ball's cell, his side and whether his team is
/// attacking, it reads the target cell from the side's formation table; during set pieces (`engine::SET_PIECE_KIND` > 0) some roles keep
/// their current place; otherwise the side's tactical flags move him a step forward or back or make him follow the
/// ball's line.
///
/// [pt] { 0x6c2be0 } (match_eng.cpp): onde o jogador deve se posicionar (saidas x e y). Pela bola
/// (0x8ea7/0x8ea8, limitada a 9 x 12), pelo lado e se o time dele esta atacando, le a casa na
/// tabela de formacao do lado (x no meio byte baixo, y no alto). Com 0x8eb1 > 0, a posicao atual
/// quando o papel (ataque ou defesa) manda ficar, e tudo limitado a grade. Senao, ajustes pelas
/// marcas do lado (0x9766 + 6371 x lado): um passo a frente ou atras, e acompanhar a linha da bola.
pub(crate) unsafe extern "thiscall" fn position_place_players(this: usize, j: usize, out_x: *mut i8, out_y: *mut i8, _a4: u32) {
    let ball_x = read_i8(this + engine::BALL_X);
    let ball_y = read_i8(this + engine::BALL_Y);
    let bx = if ball_x < 0 { 0 } else if ball_x >= 9 { 8 } else { ball_x };
    let by = if ball_y < 0 { 0 } else if ball_y >= 12 { 11 } else { ball_y };
    let lado1 = (side_of_player(j) == 1) as i8;
    let s = side_of_player(j);
    let posse = read_i8(this + engine::POSSESSION_SIDE);
    // attacking: his team has possession (or the player stored in 0xf57a is on his side), or the
    // ball is at the edge of the opponent’s area with 0x8ea9 set
    let atacando: i8 = if posse as i32 == (s == 0) as i32 {
        0
    } else if s == posse {
        1
    } else {
        let stored = *((this + engine::SET_PIECE_TAKER) as *const usize);
        if stored != 0 && read_i8(stored + match_player::SIDE) == s {
            1
        } else {
            let band_y = if s == 0 { 10..=11 } else { 0..=1 };
            ((2..=6).contains(&ball_x) && band_y.contains(&ball_y) && read_i8(this + engine::BALL_HEIGHT) != -1) as i8
        }
    };
    let formation = |cel: usize| -> u8 {
        let table = *((this + engine::FORMATION_TABLE + 4 * lado1 as usize) as *const usize);
        let e = 9 * (atacando as isize + 2 * read_i8(j + match_player::POSITION) as isize) + bx as isize;
        *((table as isize + 12 * e + cel as isize) as *const u8)
    };
    let limit_y = |v: i8| if v < 0 { 0 } else if v >= 12 { 11 } else { v };
    let limit_x = |v: i8| if v < 0 { 0 } else if v >= 9 { 8 } else { v };
    if read_i8(this + engine::SET_PIECE_KIND) > 0 {
        let role = if atacando != 0 { call_player_byte_routine(0x716270, j) } else { call_player_byte_routine(0x716310, j) };
        if role != 0 {
            *out_x = read_i8(j + match_player::CELL_X);
            *out_y = read_i8(j + match_player::CELL_Y);
        } else {
            let c = formation(by as usize);
            *out_x = (c & 0xf) as i8;
            *out_y = (c >> 4) as i8;
        }
        *out_y = limit_y(*out_y);
        *out_x = limit_x(*out_x);
        return;
    }
    let c = formation(by as usize);
    *out_x = (c & 0xf) as i8;
    *out_y = (c >> 4) as i8;
    if exe::match_player_get_position(j) == 1 {
        return;
    }
    let marks = (this as isize + 6371 * lado1 as isize + 0x9766) as *const u32;
    let parado = read_i8(this + (lado1 as usize + 6)) == 0;
    let step_towards = |for_front: bool| {
        let d: i8 = if for_front { 1 } else { -1 };
        *out_y = limit_y((*out_y).wrapping_add(d));
        *out_x = limit_x(*out_x);
    };
    let m = *marks;
    if m & 0x1000 != 0 && parado {
        step_towards(lado1 != 0);
    } else if m & 0x10000 != 0 && atacando != 0 {
        step_towards(lado1 == 0);
    } else if m & 0x8000 != 0 && atacando == 0 {
        step_towards(lado1 != 0);
    }
    if *marks & 0x4000 == 0 {
        return;
    }
    if atacando != 0 && parado {
        if lado1 != 0 {
            let v = (*out_y).wrapping_add(2);
            *out_y = if v > 0xb { 0xb } else { v };
        } else {
            let v = (*out_y).wrapping_add(-2);
            *out_y = if v < 0 { 0 } else { v };
        }
        return;
    }
    let row = ball_y as i32;
    let current = *out_y as i32;
    let v = if lado1 != 0 {
        let v = (row + 2).max(current) as i8;
        let v = if v >= 9 { v } else { 9 };
        let v = (v as i32).min(row + 4) as i8;
        if v > 0xb { 0xb } else { v }
    } else {
        let v = (row - 2).min(current) as i8;
        let v = if v <= 2 { v } else { 2 };
        let v = (v as i32).max(row - 4) as i8;
        if v < 0 { 0 } else { v }
    };
    *out_y = v;
}

/// side 1, largest y (0x103) among the active ones starting from 6, plus 1, at 0x8eaf; side 0, smallest y starting from
/// 5, minus 1, at 0x8eb0 (the fields read by `side_difference` and `player_difference`). Active: position >= 0 and
/// `player_table` != 1. The original has both 20-player loops unrolled (1.7 KB)
/// Original: `defensive_lines` (0x6c3aa0, offside, thiscall).
///
/// [pt] { 0x6c3aa0 } (match_eng.cpp): as duas linhas defensivas (impedimento). Lado 1: maior y (0x103)
/// entre os ativos, partindo de 6, mais 1, em 0x8eaf. Lado 0: menor y, partindo de 5, menos 1, em
/// 0x8eb0. Ativo: posicao >= 0 e `player_table` (pelo endereco) diferente de 1. O original tem
/// os dois lacos de 20 jogadores desenrolados.
pub(crate) unsafe extern "thiscall" fn defensive_lines(this: usize) {
    let ativos = |side: usize| {
        (0..20).map(move |k| (this + engine::PLAYERS + 8920 * side + k * 0x1be) as usize).filter(move |&p| read_i8(p + match_player::POSITION) >= 0 && exe::match_player_get_position(p) != 1)
    };
    let greater = ativos(1).fold(6i8, |m, p| m.max(read_i8(p + match_player::CELL_Y)));
    *((this + engine::OFFSIDE_LINE) as *mut i8) = greater.wrapping_add(1);
    let smaller = ativos(0).fold(5i8, |m, p| m.min(read_i8(p + match_player::CELL_Y)));
    *((this + engine::OFFSIDE_LINE_SIDE1) as *mut i8) = smaller.wrapping_sub(1);
}

/// ranges by the attributes and, by the tactical role (bits 1, 2, 4, 8, 0x10, 0x20, 0x40 of `table_a`; "strong" = bits
/// 0x880 of the whole word), draws from 1/75 to 1/1000, floating-point ratios between attributes and the side's state
/// (0xf5bd/0xf5f3). Transcribed block by block, with the original's addresses in the comments; the shared endings
/// became local functions
/// Original: `update_player` (0x6f4aa0, 2.8 KB, 908 instructions. Counter 0x2b (decreases down to 0); float 0x1a2 (× 0.85, floor 2 × (25 − the person's attribute 0x5d), ceiling 100); word 0x35 (disposition, ~4,000 to ~8,750), thiscall).
///
/// [pt] { 0x6f4aa0 } (match_pl.cpp): atualiza, a cada lance, o contador 0x2b (decresce ate 0), o float
/// 0x1a2 (x 0,85, com piso 2 x (25 - atributo 0x5d da pessoa) e teto 100) e a palavra 0x35
/// (disposicao): faixas pelos atributos e, pelo papel tatico (bits de `table_a`), sorteios de
/// 1/75 a 1/1000, razoes em ponto flutuante entre atributos e o estado do lado (0xf5bd/0xf5f3).
/// Transcrita bloco a bloco (enderecos do original nos comentarios); os finais compartilhados
/// viram as funcoes locais `ate`.
pub(crate) unsafe extern "thiscall" fn update_player(this: usize) {
    let w35 = (this + match_player::MATCH_RATING) as *mut i16;
    // 0x6f4aa8: contador 0x2b
    let c = read_i8(this + match_player::BUSY);
    if c > 0 {
        *((this + match_player::BUSY) as *mut i8) = c - 1;
    } else if c < 0 {
        *((this + match_player::BUSY) as *mut i8) = 0;
    }
    // 0x6f4abc: float 0x1a2
    let f1a2 = (this + match_player::CONFIDENCE) as *mut f32;
    let mut st = *f1a2 as f64 * read_f32(rdata::F32_0_8500000238) as f64;
    *f1a2 = st as f32;
    let person = Staff(*((this + match_player::STAFF) as *const usize));
    let floor = ((0x19 - person.temperament() as i32) * 2) as f32;
    if !(st > floor as f64) {
        st = floor as f64;
    }
    *f1a2 = st as f32;
    if st > read_f64(rdata::F64_100) {
        *f1a2 = f32::from_bits(0x42c80000);
    }
    // 0x6f4b18
    if (read_u8(this + match_player::ASSISTS) as i32 + read_u8(this + match_player::GOALS) as i32) < 2 && read_u8(this + match_player::KEY_TACKLES) < 7 && read_u8(this + 3) < 4 && read_u8(this + match_player::DRIBBLES) < 6 {
        let edi = *w35 as i32;
        if edi > random(0x5dc) + 0x203a {
            *w35 = ftol((edi - 0x203a) as f64 * read_f64(rdata::F64_0_96) + read_f64(rdata::F64_8250)) as i16;
        }
    }
    // 0x6f4b78
    let a1 = read_u8(this + match_player::PASSES) as i32;
    if a1 > 0x32 {
        let mut cx = (*w35 as i32).max((a1 + 20 * read_u8(this + 3) as i32 + 0x113) * 20) as i16;
        *w35 = cx;
        let a2 = read_u8(this + match_player::PASSES_COMPLETED) as i32;
        if a2 != 0 {
            let ratio = a1 as f64 / a2 as f64;
            if ratio > read_f64(rdata::F64_0_75) && ratio * read_f32(rdata::F32_1000) as f64 + read_f32(rdata::F32_5850) as f64 > cx as f64 {
                cx = cx.wrapping_add(5);
                *w35 = cx;
            }
        }
    }
    // 0x6f4bfd / 0x6f4c2b
    let d10 = read_u8(this + match_player::ASSISTS) as i32;
    if d10 != 0 {
        *w35 = (425 * d10 + 0x1770).max(*w35 as i32) as i16;
    }
    let dc = read_u8(this + match_player::GOALS) as i32;
    if dc != 0 && dc + d10 > 1 {
        *w35 = (375 * (dc + d10) + 0x1964).max(*w35 as i32) as i16;
    }
    // finais compartilhados
    let ate = move |lim: i16, step_towards: i16| {
        let ax = *w35;
        if ax < lim {
            *w35 = ax + step_towards;
        }
    };
    let above = move |lim: i16, step_towards: i16| {
        let ax = *w35;
        if ax > lim {
            *w35 = ax - step_towards;
        }
    };
    let mais5_ou_1 = move || {
        let ax = *w35;
        if ax >= 0x1676 {
            if ax < 0x1a5e {
                *w35 = ax + 1;
            }
        } else {
            *w35 = ax + 5;
        }
    };
    let sum_5c = move |c: i32| {
        if (*w35 as i32) < c * 500 + 0x1a5e {
            *w35 = (*w35).wrapping_add((5 * c) as i16);
        }
    };
    let quiet = || read_u8(this + match_player::KEY_TACKLES) == 0 && read_u8(this + match_player::KEY_CHALLENGES_WON) == 0 && read_u8(this + 3) == 0 && read_u8(this + match_player::ASSISTS) == 0;
    let engine = *((this + match_player::ENGINE) as *const usize);
    let role = if engine != 0 { exe::cm3_match_engine_get_player_position(engine, this) as u8 } else { 0 };
    let state_side = || {
        if side_of_player(this) != 0 { *((engine + engine::SCORE) as *const i8) } else { *((engine + engine::SCORE_SIDE1) as *const i8) }
    };
    let byte_side = |engine: usize| *((engine + 54 * (side_of_player(this) == 0) as usize + 0xf5bf) as *const u8) as i32;
    if role & 1 != 0 {
        // 0x6f4c7e
        let f = read_u8(this + match_player::GOALS_CONCEDED) as i32;
        if random(f * f) == 0 && *w35 < 0x1cb6 {
            *w35 += 1;
        }
        let a1f = read_i8(this + match_player::MINUTE_ON);
        if a1f != -1 && a1f > 0x1e {
            if read_u8(this + match_player::GOALS_CONCEDED) == 0 {
                ate(0x1b58, 1);
            }
            return;
        }
        // 0x6f4cc7
        let b = byte_side(engine);
        *w35 = ((b + 0xe) * 500).min(*w35 as i32) as i16;
        let cf = read_u8(this + match_player::GOALS_CONCEDED);
        if cf == 0 {
            *w35 = ((b + 0x14) * 300).max(*w35 as i32) as i16;
            let lim = (if (b as u8) < 2 { b } else { 2 } + 0x1c) * 0xfa;
            if (*w35 as i32) < lim {
                *w35 += 1;
            }
            return;
        }
        // 0x6f4d5f
        if *w35 <= 0x109a {
            return;
        }
        if cf > 1 && random(cf as i32) != 0 {
            if read_u8(this + match_player::GOALS_CONCEDED) > 2 && *w35 > 0x1e46 {
                *w35 = 0x1e46;
                return;
            }
            let ax = *w35;
            *w35 = if ax > 0x222e {
                ax - 0x19
            } else if ax > 0x1c52 {
                ax - read_u8(this + match_player::GOALS_CONCEDED) as i16
            } else {
                ax - 1
            };
            return;
        }
        // 0x6f4dcc
        if (*w35 as i32) <= (byte_side(engine) + 0x19) * 0xfa {
            return;
        }
        let q = *((engine + engine::CLOCK) as *const i16) as i32 / 11;
        if random(0x5a - q) == 0 {
            *w35 -= 1;
        }
        return;
    }
    if role & 2 != 0 {
        // 0x6f4e53
        if random(0x4b) == 0 {
            *w35 -= 1;
        }
        if quiet() {
            above(0x1d4c, 10);
        }
        if state_side() != 0 {
            return;
        }
        ate(0x1676, 1);
        return;
    }
    if role & 4 != 0 {
        // 0x6f4ee4
        if random(0x4b) == 0 {
            *w35 -= 1;
        }
        let (a4, a7) = (read_u8(this + match_player::TACKLES) as i32, read_u8(this + match_player::CHALLENGES) as i32);
        if a4 + a7 > 9 {
            let mut aa = ((read_u8(this + match_player::TACKLES_WON) as i32 + read_u8(this + match_player::CHALLENGES_WON) as i32) as f64 / (a4 + a7) as f64) as f32;
            let s96 = read_u8(this + match_player::KEY_CHALLENGES_WON) as i32 + read_u8(this + match_player::KEY_TACKLES) as i32;
            if s96 != 0 {
                aa = (s96 as f64 * read_f64(rdata::F64_0_025) + aa as f64) as f32;
            }
            let adjust = |aa: f32, num: i32, den: i32, factor_strong: usize| -> f32 {
                let st = num as f64 / den as f64;
                let stored = st as f32;
                if st < read_f64(rdata::F64_0_65) || st.is_nan() {
                    let k = if engine_player_role(engine, this) & 0x880 != 0 { factor_strong } else { 0x9689e0 };
                    (aa as f64 - (stored as f64 - read_f64(rdata::F64_0_65)) * read_f64(k)) as f32
                } else {
                    aa
                }
            };
            if a4 > 4 {
                aa = adjust(aa, read_u8(this + match_player::TACKLES_WON) as i32, a4, 0x968918);
            }
            if a7 > 4 {
                aa = adjust(aa, read_u8(this + match_player::CHALLENGES_WON) as i32, a7, 0x967880);
            }
            // 0x6f502a
            let v = state_side() as i16;
            let mut cx = 0x1di16.wrapping_sub(v).wrapping_mul(0xfa);
            if read_u8(this + match_player::GOALS) != 0 {
                cx = cx.wrapping_add((read_u8(this + match_player::GOALS) as i16).wrapping_mul(0x1f4));
            }
            if read_u8(this + match_player::ASSISTS) != 0 {
                cx = cx.wrapping_add((read_u8(this + match_player::ASSISTS) as i16).wrapping_mul(0xfa));
            }
            let a = aa as f64;
            if a < read_f64(rdata::F64_0_625) || a.is_nan() {
                if *w35 > cx {
                    *w35 = cx;
                }
            } else if a > read_f64(rdata::F64_0_825) && *w35 < cx {
                *w35 = cx;
            }
        }
        // 0x6f50b7
        let parado6 = read_u8(this + match_player::KEY_TACKLES) == 0;
        if quiet() {
            above(0x1d4c, 10);
        }
        let v = state_side();
        let sem_nada = parado6 && read_u8(this + match_player::KEY_CHALLENGES_WON) == 0 && read_u8(this + 3) == 0 && read_u8(this + match_player::ASSISTS) == 0 && read_u8(this + match_player::GOALS) == 0;
        if v == 0 {
            if sem_nada {
                let a2 = read_u8(this + match_player::PASSES_COMPLETED) as i32;
                if a2 <= 0x14 {
                    ate(0x1676, 1);
                    return;
                }
                let r = read_u8(this + match_player::PASSES) as i32 as f64 / a2 as f64;
                if !(r > read_f64(rdata::F64_0_75)) {
                    ate(0x1676, 1);
                    return;
                }
            }
            mais5_ou_1();
            return;
        }
        // 0x6f5195
        if v == 1 {
            if sem_nada {
                above(0x1c52, 1);
            } else {
                ate(0x186a, 1);
            }
            return;
        }
        // 0x6f51f9
        let lim = (0x1fi16.wrapping_sub(v as i16).wrapping_mul(0xfa)).max(0x1a5e);
        if *w35 <= lim {
            return;
        }
        let strong = engine_player_role(engine, this) & 0x880 != 0;
        if strong && !sem_nada {
            *w35 -= 1;
        } else if strong || !sem_nada {
            *w35 = (*w35).wrapping_sub(v as i16);
        } else {
            *w35 = (*w35).wrapping_sub((v as i16).wrapping_mul(5));
        }
        return;
    }
    if role & 8 != 0 {
        // 0x6f5332
        if random(0x1f4) == 0 {
            *w35 -= 1;
        }
        let three = read_u8(this + match_player::KEY_TACKLES) == 0 && read_u8(this + match_player::KEY_CHALLENGES_WON) == 0 && read_u8(this + 3) == 0;
        if three {
            above(0x1d4c, 10);
        }
        if state_side() != 0 {
            return;
        }
        if three && read_u8(this + match_player::ASSISTS) == 0 {
            ate(0x186a, 1);
        } else {
            mais5_ou_1();
        }
        return;
    }
    if role & 0x10 != 0 {
        // 0x6f540f
        if random(0x3e8) == 0 {
            *w35 -= 1;
        }
        if read_u8(this + match_player::GOALS) != 0 || read_u8(this + match_player::ASSISTS) != 0 {
            return;
        }
        if read_u8(this + 3) != 0 || read_u8(this + match_player::SHOTS_ON_TARGET) != 0 {
            above(0x1c52, 5);
        } else {
            above(0x1c52, 10);
        }
        return;
    }
    if role & 0x20 != 0 {
        // 0x6f5469
        if random(0x1f4) == 0 {
            *w35 -= 1;
        }
        let c = read_u8(this + match_player::GOALS);
        if c != 0 {
            sum_5c(c as i32);
        } else if read_u8(this + match_player::ASSISTS) != 0 {
            // 0x6f54ba: al = 0
        } else if read_u8(this + 3) != 0 || read_u8(this + match_player::SHOTS_ON_TARGET) != 0 {
            above(0x1c52, 5);
        } else {
            above(0x1c52, 10);
        }
        return;
    }
    if role & 0x40 != 0 {
        // 0x6f5515
        if random(0x96) == 0 {
            *w35 -= 1;
        }
        let c = read_u8(this + match_player::GOALS);
        if c != 0 {
            sum_5c(c as i32);
        } else if read_u8(this + match_player::ASSISTS) != 0 || read_u8(this + match_player::SHOTS_ON_TARGET) != 0 {
            // 0x6f557d: al = 0
        } else if read_u8(this + 3) != 0 || read_u8(this + match_player::KEY_CHALLENGES_WON) != 0 {
            above(0x1e46, 5);
        } else {
            above(0x1c52, 10);
        }
    }
}

reader!(ler_8ea7, 0x8ea7); // { 0x700320 } bola x
reader!(ler_8ea8, 0x8ea8); // { 0x700330 } bola y
reader!(ler_8ea9, 0x8ea9); // { 0x700340 }
reader!(ler_8eaa, 0x8eaa); // { 0x700380 }
reader!(ler_8eb2, 0x8eb2); // { 0x700400 }
recorder!(gravar_8ea9, 0x8ea9); // { 0x700350 }
recorder!(gravar_8ea7, 0x8ea7); // { 0x700360 }
recorder!(gravar_8ea8, 0x8ea8); // { 0x700370 }
recorder!(gravar_8eaa, 0x8eaa); // { 0x700390 }

/// Stores the engine word `engine::BALL_DIRECTION` [inference: the ball's direction angle].
///
/// [pt] { 0x7003a0 }: grava a palavra 0x8ed8.
pub(crate) unsafe extern "thiscall" fn write_8ed8(this: usize, v: u16) {
    *((this + engine::BALL_DIRECTION) as *mut u16) = v;
}

/// Match engine: returns the 32-bit field `engine::COMMENTATOR`.
///
/// [pt] { 0x700410 }: le o ponteiro 0x8f7a.
pub(crate) unsafe extern "thiscall" fn read_8f7a(this: usize) -> u32 {
    *((this + engine::COMMENTATOR) as *const u32)
}

/// Records side `v` as having possession (engine `engine::LAST_POSSESSION_SIDE`); unless -1, adds 1 to that side's counter (`engine::POSSESSION_COUNTER` + side)
/// and takes 10 from the other side's counter if positive.
///
/// [pt] { 0x7003b0 } (match_pl.cpp): registra o lado com a posse (0x8eab). Exceto -1, soma 1 ao
/// contador do lado (0x8d46 + lado) e tira 10 do contador do outro (0x8d46 + (lado == 0)), se positivo.
pub(crate) unsafe extern "thiscall" fn record_possession(this: usize, v: i8) {
    *((this + engine::LAST_POSSESSION_SIDE) as *mut i8) = v;
    if v == -1 {
        return;
    }
    let my = (this as isize + 0x8d46 + v as isize) as *mut i8;
    *my = (*my).wrapping_add(1);
    let other = (this + engine::POSSESSION_COUNTER + (v == 0) as usize) as *mut i8;
    if *other > 0 {
        *other = (*other).wrapping_sub(10);
    }
}

/// Is the pointer in the match player's list (+0x1ae, count at +0x101)? Returns 1 or 0.
///
/// [pt] { 0x714c60 } (match_pl.cpp): o ponteiro esta na lista do objeto (0x1ae, quantidade em 0x101)?
pub(crate) unsafe extern "thiscall" fn in_list(this: usize, target: usize) -> u8 {
    let n = *((this + match_player::MARKER_COUNT) as *const i8);
    (0..n.max(0) as usize).any(|k| *((this + match_player::MARKERS + 4 * k) as *const usize) == target) as u8
}

/// Stores the extreme players of each side among the active ones (position >= 0 and, in 0..=10, `table_a` different
/// from 1): side 0, smallest y at `0xf58e` and largest at `0xf596`; side 1, largest y at `0xf592` and smallest at
/// `0xf59a`. With no candidate the previous value stays.
///
/// Convention: `match_eng.cpp`.
/// Original: `store_extremes` (0x6cd660, thiscall).
///
/// [pt] { 0x6cd660 } (match_eng.cpp): os jogadores extremos de cada lado entre os ativos (posicao >= 0 e,
/// em 0..=10, `table_a` diferente de 1): lado 0, menor y em 0xf58e e maior em 0xf596; lado 1,
/// maior y em 0xf592 e menor em 0xf59a. Sem candidato, o valor anterior fica.
pub(crate) unsafe extern "thiscall" fn store_extremes(this: usize) {
    for (lado_b, campo_menor, campo_maior) in [(0usize, 0xf58eusize, 0xf596usize), (1, 0xf59a, 0xf592)] {
        let (mut menor, mut maior) = (12i8, -1i8);
        for k in 0..20 {
            let p = this + engine::PLAYERS + 8920 * lado_b + k * 0x1be;
            let pos = read_i8(p + match_player::POSITION);
            if pos < 0 || (pos <= 10 && position_table_entry(this, p, pos) == 1) {
                continue;
            }
            let y = read_i8(p + match_player::CELL_Y);
            if y < menor {
                menor = y;
                *((this + campo_menor) as *mut usize) = p;
            }
            if y > maior {
                maior = y;
                *((this + campo_maior) as *mut usize) = p;
            }
        }
    }
}

/// the player's word 0x19a
/// Original: `direction` (0x6f4a30, thiscall, `ax`).
///
/// [pt] { 0x6f4a30 } (match_pl.cpp): direcao do jogador (palavra 0x19a).
pub(crate) unsafe extern "thiscall" fn direction(this: usize) -> u16 {
    *((this + match_player::FACING) as *const u16)
}

/// Clears the side's stored player pointer (engine `engine::GOALKEEPER` for side 0, `engine::GOALKEEPER_SIDE1` otherwise).
///
/// [pt] { 0x6ce0f0 } (match_eng.cpp): zera o guardado do lado (0xf59e para o lado 0, senao 0xf5a2).
pub(crate) unsafe extern "thiscall" fn clear_stored(this: usize, side_b: u8) {
    *((this + if side_b == 0 { 0xf59e } else { 0xf5a2 }) as *mut u32) = 0;
}

/// x (0x102) outside 2..=6
/// Original: `x_off_centre` (0x70df20, thiscall, `al`).
///
/// [pt] { 0x70df20 } (match_pl.cpp): 1 com o x do jogador (0x102) fora de 2..=6.
pub(crate) unsafe extern "thiscall" fn x_off_centre(this: usize) -> u8 {
    !(2..=6).contains(&read_i8(this + match_player::CELL_X)) as u8
}

/// player == motor[0xf582]
/// Original: `is_the_stored` (0x7103c0, thiscall, `al`).
///
/// [pt] { 0x7103c0 } (match_pl.cpp): o jogador e o guardado em 0xf582 do motor dele.
pub(crate) unsafe extern "thiscall" fn is_the_stored(this: usize) -> u8 {
    let engine = *((this + match_player::ENGINE) as *const usize);
    (this == *((engine + engine::BALL_HOLDER) as *const usize)) as u8
}

/// 0xff with no move; otherwise the move's player is on side 0
/// Original: `side_of_move` (0x6bba50, thiscall, `al`).
///
/// [pt] { 0x6bba50 } (match_eng.cpp): lado 0 do jogador em 0x4761 (1 se lado 0, senao 0); 0xff sem
/// lance (0x475a zerado) ou sem jogador.
pub(crate) unsafe extern "thiscall" fn side_of_move(this: usize) -> u8 {
    let p = *((this + engine::PENDING_PLAY_OFFENDER) as *const usize);
    if read_i8(this + engine::PENDING_PLAY_KIND) == 0 || p == 0 {
        return 0xff;
    }
    (read_i8(p + match_player::SIDE) == 0) as u8
}

/// Returns the address of player `k` of side `lado_b` in the match engine's player block (`engine::PLAYERS`, 0x1be bytes per
/// player, 20 per side).
///
/// [pt] { 0x6c2fe0 } (match_eng.cpp): endereco do jogador `k` do bloco do `lado`.
pub(crate) unsafe extern "thiscall" fn player_in_block(this: usize, side_b: i8, k: i8) -> usize {
    (this as isize + 0x4796 + 0x1be * (side_b as isize * 20 + k as isize)) as usize
}

/// Match engine: the role code of player `jogador`'s reserve position (+0x1a, 0..10) from his side's formation table
/// (`engine::POSITION_TABLE` + 4 x side); 0 if out of range.
///
/// [pt] { 0x6c3920 } (match_eng.cpp): como `table_a`, pela posicao em 0x1a.
pub(crate) unsafe extern "thiscall" fn table_a_1a(this: usize, player: usize) -> u16 {
    let position = read_i8(player + match_player::PREVIOUS_POSITION);
    if !(0..=10).contains(&position) {
        return 0;
    }
    let table = *((this as isize + 0x8ebc + read_i8(player + match_player::SIDE) as isize * 4) as *const usize) as *const u16;
    *table.add(position as usize)
}

/// 0x8ed4 outside 0x1ef/0x3de and bit 0x80 of the match's byte 0x38
/// Original: `marked_moment` (0x6d4250, thiscall, `eax`).
///
/// [pt] { 0x6d4250 } (match_eng.cpp): 1 com o momento (0x8ed4) fora de 0x1ef e 0x3de e o bit 0x80 do
/// byte 0x38 da partida (0x4792) ligado.
pub(crate) unsafe extern "thiscall" fn marked_moment(this: usize) -> u32 {
    let m = *((this + engine::PERIOD_END) as *const u16);
    let partida = *((this + engine::MATCH) as *const usize);
    (m != 0x1ef && m != 0x3de && read_i8(partida + fixture::TIE_RULES) as u8 & 0x80 != 0) as u32
}

/// Returns the state byte of the match player's own side (engine `engine::SCORE` for side 0, `engine::SCORE_SIDE1` otherwise).
///
/// [pt] { 0x6f4a40 } (match_pl.cpp): estado do proprio lado (lado 0: 0xf5bd; outros: 0xf5f3).
pub(crate) unsafe extern "thiscall" fn side_state(this: usize) -> u8 {
    let engine = *((this + match_player::ENGINE) as *const usize);
    read_i8(engine + (if side_of_player(this) != 0 { 0xf5f3 } else { 0xf5bd })) as u8
}

/// Match player: returns the state byte of the other side (engine `engine::SCORE` for side 0, `engine::SCORE_SIDE1` for side 1) [inference:
/// team mentality/state].
///
/// [pt] { 0x6f4a70 } (match_pl.cpp): estado do outro lado.
pub(crate) unsafe extern "thiscall" fn opponent_state(this: usize) -> u8 {
    let engine = *((this + match_player::ENGINE) as *const usize);
    read_i8(engine + (if side_of_player(this) != 0 { 0xf5bd } else { 0xf5f3 })) as u8
}

/// Writes the match player's word +0x19c, clamped to -200..200.
///
/// [pt] { 0x70a890 } (match_pl.cpp): grava 0x19c limitado a -200..=200.
pub(crate) unsafe extern "thiscall" fn write_19c(this: usize, v: i16) {
    *((this + match_player::BALL_CONTROL) as *mut i16) = v.clamp(-200, 200);
}

/// Returns 1 if move code `c` is a set-piece code (12, 13, 21, 37, 46 or 59).
///
/// [pt] { 0x70a8c0 } (match_pl.cpp, sem `this`): codigos 12, 13, 21, 37, 46 e 59.
pub(crate) unsafe extern "stdcall" fn set_piece_code(c: i16) -> u8 {
    matches!(c, 12 | 13 | 21 | 37 | 46 | 59) as u8
}

/// y < 4; side 0: y > 7
/// Original: `near_goal` (0x70df70, side 1, thiscall, `al`).
///
/// [pt] { 0x70df70 } (match_pl.cpp): perto do gol adversario (lado 1: y < 4; lado 0: y > 7).
pub(crate) unsafe extern "thiscall" fn near_goal(this: usize) -> u8 {
    let (l, y) = (side_of_player(this), read_i8(this + match_player::CELL_Y));
    ((l == 1 && y < 4) || (l == 0 && y > 7)) as u8
}

/// 0x475a..=0x4769
/// Original: `clear_move` (0x6bba10, thiscall).
///
/// [pt] { 0x6bba10 } (match_eng.cpp): limpa o lance (0x475a..=0x4769).
pub(crate) unsafe extern "thiscall" fn clear_move(this: usize) {
    *((this + engine::PENDING_PLAY_CODE) as *mut u16) = 0xffff;
    *((this + engine::PENDING_PLAY_OFFENDER) as *mut u32) = 0;
    *((this + engine::PENDING_PLAY_VICTIM) as *mut u32) = 0;
    *((this + engine::PENDING_PLAY_SERIOUS) as *mut u8) = 0;
    *((this + engine::PENDING_PLAY_KIND) as *mut u8) = 0;
    for o in 0x475b..=0x475e {
        *((this + o) as *mut u8) = 0xff;
    }
}

/// Adds `delta` to the disposition word (+0x35) of every active player of side `lado_b`.
///
/// [pt] { 0x6cf710 } (match_eng.cpp): soma `delta` a disposicao (0x35) de todos os ativos do lado.
pub(crate) unsafe extern "thiscall" fn add_disposition(this: usize, side_b: i8, delta: i16) {
    for k in 0..20 {
        let p = side_player_slot(this, side_b, k);
        if read_i8(p + match_player::POSITION) >= 0 {
            let w = (p + match_player::MATCH_RATING) as *mut i16;
            *w = (*w).wrapping_add(delta);
        }
    }
}

/// word 0x1bc
/// Original: `read_1bc` (0x717400, thiscall, `ax`).
///
/// [pt] { 0x717400 }: palavra 0x1bc do jogador.
pub(crate) unsafe extern "thiscall" fn read_1bc(this: usize) -> u16 {
    *((this + 0x1bc) as *const u16)
}

/// Adds `v` to the match player's busy counter (+0x2b).
///
/// [pt] { 0x7160d0 }: soma ao byte 0x2b do jogador.
pub(crate) unsafe extern "thiscall" fn add_2b(this: usize, v: u8) {
    let b = (this + match_player::BUSY) as *mut u8;
    *b = (*b).wrapping_add(v);
}

/// Match player: writes byte +0x2b.
///
/// [pt] { 0x7160e0 }: grava o byte 0x2b do jogador.
pub(crate) unsafe extern "thiscall" fn write_2b(this: usize, v: u8) {
    *((this + match_player::BUSY) as *mut u8) = v;
}

/// Calls a thiscall (engine, player) routine by game address using the player's engine (+0x19e); None without engine.
///
/// [pt] Chama uma rotina (motor, jogador) do motor do jogador pelo endereco do jogo; sem motor, None.
pub(crate) unsafe fn call_engine_routine<R>(j: usize, routine: usize) -> Option<R> {
    let engine = *((j + match_player::ENGINE) as *const usize);
    if engine == 0 {
        return None;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> R = core::mem::transmute(routine);
    Some(f(engine, j))
}

/// Match player: the role code of his reserve position (+0x1a) from his engine's formation table (`table_a_1a`), or 0
/// without an engine.
///
/// [pt] { 0x717430 }: `table_a_1a` (0x6c3920) do motor do jogador, ou 0.
pub(crate) unsafe extern "thiscall" fn player_table_a_1a(this: usize) -> u16 {
    call_engine_routine(this, 0x6c3920).unwrap_or(0)
}

/// Returns table_b (`table_b`) of the player's match engine for this player, or 0 without an engine.
///
/// [pt] { 0x717450 }: `table_b` (0x6c38f0) do motor do jogador, ou 0.
pub(crate) unsafe extern "thiscall" fn player_table_b(this: usize) -> u16 {
    call_engine_routine(this, 0x6c38f0).unwrap_or(0)
}

/// Returns the match engine's player_difference (`player_difference`) for this match player, or 0xff when there is no engine.
///
/// [pt] { 0x715f30 }: `player_difference` (0x6c4130) do motor do jogador, ou 0xff.
pub(crate) unsafe extern "thiscall" fn difference_of_player(this: usize) -> u8 {
    call_engine_routine(this, 0x6c4130).unwrap_or(0xff)
}

/// already in Rust, not hooked; now hooked (prologue with a relative `jne` rebuilt in the trampoline)
/// Original: `ball_in_attack` (0x714d00, match_pl.cpp, thiscall).
///
/// [pt] { 0x714d00 } (o prologo curto termina num jne relativo, refeito no trampolim: RELATIVAS em
/// detours.rs): bola (y em 0x8ea8) no campo de ataque do lado (lado 1: y >= 6; outros: y < 6).
pub(crate) unsafe extern "thiscall" fn ball_in_attack(this: usize) -> u8 {
    let y = read_i8(*((this + match_player::ENGINE) as *const usize) + 0x8ea8);
    (if side_of_player(this) == 1 { y >= 6 } else { y < 6 }) as u8
}

/// `exe::formation_get_player_instructions` of the position
/// Original: `player_marks_rot` (0x7163c0, thiscall, `edx:eax`).
///
/// [pt] { 0x7163c0 }: marcas taticas (0x5b5270) da posicao do jogador. Devolve edx:eax inteiro.
pub(crate) unsafe extern "thiscall" fn player_marks_rot(this: usize) -> u64 {
    player_position_instructions(this)
}

/// byte 6 + side; otherwise bit 0x800000 of the marks
/// Original: `mark_800000` (0x7163f0, bit 0x1000 of the block, thiscall, `al`).
///
/// [pt] { 0x7163f0 }: com o bit 0x1000 do bloco do lado (0x9766), o byte 6 + lado do motor; senao, o
/// bit 0x800000 das marcas da posicao.
pub(crate) unsafe extern "thiscall" fn mark_800000(this: usize) -> u8 {
    let l = side_of_player(this) as isize;
    let engine = *((this + match_player::ENGINE) as *const usize) as isize;
    let block = engine + 6371 * l;
    if *((block + engine::TACTICS_FLAGS as isize) as *const u32) & 0x1000 != 0 {
        return *((engine + 6 + l) as *const u8);
    }
    (player_position_instructions(this) as u32 & 0x800000 != 0) as u8
}

/// y < 3; side 1: y > 8
/// Original: `near_own_goal` (0x70def0, side 0, thiscall, `al`).
///
/// [pt] { 0x70def0 }: perto do proprio gol (lado 0: y < 3; lado 1: y > 8).
pub(crate) unsafe extern "thiscall" fn near_own_goal(this: usize) -> u8 {
    let (l, y) = (side_of_player(this), read_i8(this + match_player::CELL_Y));
    ((l == 0 && y < 3) || (l == 1 && y > 8)) as u8
}

/// 0x880 in `table_a` and `table_b`
/// Original: `role_880` (0x717c70, thiscall, `al`).
///
/// [pt] { 0x717c70 }: papel 0x880 em `table_a` e em `table_b` do jogador (sem motor, 0).
pub(crate) unsafe extern "thiscall" fn role_880(this: usize) -> u8 {
    let a: u16 = call_engine_routine(this, 0x6c38c0).unwrap_or(0);
    if a & 0x880 == 0 {
        return 0;
    }
    let b: u16 = call_engine_routine(this, 0x6c38f0).unwrap_or(0);
    (b & 0x880 != 0) as u8
}

/// zeroes the owner and the move state
/// Original: `release_ball` (0x6f0970, thiscall).
///
/// [pt] { 0x6f0970 }: solta a bola: zera 0x4786, 0xf582 e 0x1a7 do motor (0x8eae = 0xff) e as palavras
/// 0x29 e 0x19c do jogador. O motor e relido a cada gravacao.
pub(crate) unsafe extern "thiscall" fn release_ball(this: usize) {
    let engine = || *((this + match_player::ENGINE) as *const usize);
    *((engine() + 0x4786) as *mut u32) = 0;
    *((engine() + 0x8eae) as *mut u8) = 0xff;
    *((engine() + 0xf582) as *mut u32) = 0;
    *((engine() + 0x1a7) as *mut u8) = 0;
    *((this + 0x29) as *mut u16) = 0;
    *((this + match_player::BALL_CONTROL) as *mut u16) = 0;
}

/// Returns the match player (among the 20 starting at [this + 0x23]) whose player record (+0x69) is `ficha`, or 0.
///
/// [pt] { 0x6e1cd0 }: o jogador (dos 20 a partir de [this + 0x23]) com a ficha dada, ou 0.
pub(crate) unsafe extern "thiscall" fn player_by_record(this: usize, sheet: usize) -> usize {
    let base = *((this + match_manager::PLAYERS) as *const usize);
    (0..20).map(|k| base + 0x1be * k).find(|&p| *((p + match_player::STAFF) as *const usize) == sheet).unwrap_or(0)
}

/// Returns the first byte of the 18-byte record for commentary code `c` (0x1f40..0x21e4, bit 15 ignored) in the table
/// at 0xb68788; 0 outside the range.
///
/// [pt] { 0x6d5cd0 } (sem `this`): byte da tabela de 18 bytes por codigo em 0xb68788 (codigos
/// 0x1f40..0x21e4, bit 15 ignorado); fora, 0.
pub(crate) unsafe extern "stdcall" fn code_byte(c: u32) -> i32 {
    let i = ((c & 0x7fff) as i32 - 0x1f40) as i16;
    if !(0..0x2a5).contains(&i) {
        return 0;
    }
    *((0xb68788 + 18 * i as usize) as *const i8) as i32
}

/// Match player: a value adjusted by role and by the person's attributes (+0x6d): for a goalkeeper, or when 2 x byte
/// +0x10a >= random(21), just `base`; otherwise the average of three parts, each `base` or a fixed value per side when
/// one of the person's attributes (+0x18, +0x19, +0x17) beats random(41) and the role allows it [inference: preferred
/// flank/column].
///
/// [pt] { 0x715f50 }: valor ajustado pelo papel e pela pessoa (0x6d). Papel 1, ou 2 x 0x10a >=
/// sorteio(21): o proprio `base`. Senao soma tres parcelas (cada uma `base`, ou um valor fixo por
/// lado quando o atributo da pessoa vence sorteio(41) e o papel nao o impede) e divide por 3.
pub(crate) unsafe extern "thiscall" fn adjusted_rating(this: usize, base: i8) -> i32 {
    let role = || call_engine_routine::<u16>(this, 0x6c38c0).unwrap_or(0);
    if role() & 1 != 0 {
        return base as u8 as i32;
    }
    if (read_i8(this + match_player::MOMENT_POSITIONING) as i32) * 2 >= random(0x15) {
        return base as u8 as i32;
    }
    let person = *((this + match_player::PLAYER) as *const usize);
    let wins = |o: usize| {
        let r = random(read_i8(person + o) as i32);
        r > random(0x29)
    };
    let lado0 = side_of_player(this) == 0;
    let mut v: i8 = if role() & 0x80 == 0 && wins(0x18) { if lado0 { 1 } else { 7 } } else { base };
    v = v.wrapping_add(if role() & 0x700 == 0 && wins(0x19) { 4 } else { base });
    v = v.wrapping_add(if role() & 0x800 == 0 && wins(0x17) { if lado0 { 7 } else { 1 } } else { base });
    v as i32 / 3
}

/// Adjusts the match player's morale (byte +0x17) by `delta`, with random draws near the midpoint 10, a floor from
/// disposition (+0x35 >= 7500) and the range 1..20. Returns 1 if it changed.
///
/// [pt] { 0x7172d0 }: ajusta o byte 0x17 do jogador por `delta` (com sorteios perto do limite 10), com
/// piso pela disposicao (0x35 >= 7500) e faixa 1..=20; devolve se mudou. Com `delta` 0 e 0x477a
/// do motor em -1, so age se a disposicao passar de 7500 e o byte estiver abaixo de 10.
pub(crate) unsafe extern "thiscall" fn adjust_17(this: usize, delta: i8) -> u8 {
    let disp = *((this + match_player::MATCH_RATING) as *const i16);
    if delta == 0 {
        let engine = *((this + match_player::ENGINE) as *const usize);
        if *((engine + engine::ADJUSTED_PLAYER_ID) as *const i32) == -1 && !(disp > 0x1d4c && read_i8(this + match_player::MATCH_FORM) < 10) {
            return 0;
        }
    }
    let v = read_i8(this + match_player::MATCH_FORM);
    let t = v as i32 + delta as i32;
    let mut a = if delta > 0 {
        if t <= 10 || random(0x15) >= t { v.wrapping_add(delta) } else { (random(delta as i32 + 1) as i8).wrapping_add(v) }
    } else if delta < 0 {
        if t >= 10 || random(0xb) <= t { v.wrapping_add(delta) } else { (random(delta as i32 - 1) as i8).wrapping_add(v) }
    } else {
        v
    };
    let disp = *((this + match_player::MATCH_RATING) as *const i16);
    if disp >= 0x1d4c {
        let x = a as f64;
        let y = disp as f64 * read_f64(rdata::F64_0_004) - read_f64(rdata::F64_20_5);
        a = ftol(if x > y { x } else { y }) as i8;
    }
    let a = a.clamp(1, 20);
    *((this + match_player::MATCH_FORM) as *mut i8) = a;
    (a != v) as u8
}

/// The player takes the ball: releases the previous owner's, credits the pass (counter and disposition by zone),
/// records possession and the ball on his square.
///
/// It releases the previous owner's ball (`0xf582`, the same writes as `release_ball`), clears the play state, credits
/// the pass (`0xf576`, same side: counter `0x2` and disposition + `0x6c`/`0x60`/`0x96`/`0x78` according to role and
/// zone), records the new owner, the ball on his square, the side in possession (`0x1a5` + side) and the possession
/// count (`0x8d46` + side). The engine is re-read on every access, as in the original.
/// Original: `take_ball` (0x6f0630, the player takes the ball, thiscall).
///
/// [pt] { 0x6f0630 } (match_pl.cpp): o jogador pega a bola. Solta a do dono anterior (0xf582, mesmas
/// gravacoes de `release_ball`), limpa o estado do lance, credita o passe (0xf576, do mesmo lado:
/// contador 0x2 e disposicao + 0x6c/0x60/0x96/0x78 conforme papel e zona), registra o novo dono,
/// a bola na casa dele, o lado com a posse (0x1a5 + lado) e a contagem de posse (0x8d46 + lado).
/// O motor e relido a cada acesso, como no original.
pub(crate) unsafe extern "thiscall" fn take_ball(this: usize) {
    let m = || *((this + match_player::ENGINE) as *const usize);
    let b = |o: usize| (m() + o) as *mut u8;
    let w = |o: usize| (m() + o) as *mut u32;
    if m() == 0 {
        return;
    }
    let old = *w(engine::BALL_HOLDER) as usize;
    if old != 0 {
        release_ball(old);
    }
    *b(0x1a4) = 0;
    *b(engine::SHOT_PENDING) = 0;
    *b(engine::SHOT_REBOUND) = 0;
    *w(engine::MOMENTUM_B) = 0;
    let table_a = || {
        exe::cm3_match_engine_get_player_position(m(), this)
    };
    let passe = *w(engine::PASSER) as usize;
    if passe != 0 && side_of_player(this) == side_of_player(passe) {
        *((passe + 2) as *mut u8) = (*((passe + 2) as *const u8)).wrapping_add(1);
        let (l, y) = (side_of_player(this), read_i8(this + match_player::CELL_Y));
        let sum: i16 = if table_a() == 1 {
            0x6c
        } else if (l == 1 && y >= 6) || (l == 0 && y < 6) {
            0x60
        } else if (l == 1 && y < 3) || (l == 0 && y > 8) {
            0x96
        } else {
            0x78
        };
        let disp = (*w(engine::PASSER) as usize + 0x35) as *mut i16;
        *disp = (*disp).wrapping_add(sum);
        *w(engine::MOMENTUM_A) = (*w(engine::MOMENTUM_A)).wrapping_add(0xf0);
        let previous = *w(engine::PASSER);
        *w(engine::PASS_RECEIVER) = this as u32;
        *w(engine::ASSIST_CANDIDATE) = previous;
    }
    *w(engine::PASSER) = 0;
    *w(engine::SET_PIECE_TAKER) = 0;
    *w(engine::BALL_RECEIVER) = 0;
    if table_a() == 1 {
        let (l, x, y) = (side_of_player(this), read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y));
        let area = if l == 1 { 10..=11 } else { 0..=1 };
        if (2..=6).contains(&x) && area.contains(&y) {
            let mm = m() as isize;
            if *((mm + engine::SIDE_PLAYED_BALL as isize + l as isize) as *const u8) == 0 {
                *((this + match_player::FACING) as *mut u16) = if l != 0 { 0xb4 } else { 0 };
                *((mm + engine::BALL_HEIGHT as isize) as *mut u8) = 0xff;
            }
        }
    }
    let (l, y) = (side_of_player(this), read_i8(this + match_player::CELL_Y));
    if (l == 1 && y >= 6) || (l == 0 && y < 6) {
        *b(engine::SET_PIECE_KIND) = 0;
    }
    *((m() as isize + 0x1a5 + side_of_player(this) as isize) as *mut u8) = 1;
    *((m() + 0x1a5 + (side_of_player(this) == 0) as usize) as *mut u8) = 0;
    *b(engine::BALL_X) = read_i8(this + match_player::CELL_X) as u8;
    *b(engine::BALL_Y) = read_i8(this + match_player::CELL_Y) as u8;
    *b(engine::POSSESSION_SIDE) = side_of_player(this) as u8;
    *w(engine::BALL_HOLDER) = this as u32;
    *((m() + 0x8ed8) as *mut u16) = *((this + match_player::FACING) as *const u16);
    *b(engine::BALL_ROLL) = 0;
    let mm = m();
    if read_i8(mm + engine::LAST_POSSESSION_SIDE) != side_of_player(this) {
        *((mm + engine::MOMENTUM_A) as *mut u32) = 0;
    }
    let l = side_of_player(this);
    let mm = m();
    *((mm + engine::LAST_POSSESSION_SIDE) as *mut i8) = l;
    if l != -1 {
        let c = (mm as isize + 0x8d46 + l as isize) as *mut u8;
        *c = (*c).wrapping_add(1);
        let o = (mm + engine::POSSESSION_COUNTER + (l == 0) as usize) as *mut i8;
        if *o > 0 {
            *o -= 10;
        }
    }
    *((this + 0x29) as *mut u16) = 0;
    *((this + match_player::BALL_CONTROL) as *mut u16) = 0;
}

/// Defence random roll: if random(goalkeeper word +0x3b) beats random(285), random(b x 0.7) + b x 0.3, else random(b).
///
/// [pt] Sorteio da defesa: se sorteio(palavra 0x3b do goleiro) vence sorteio(285), sorteio(b x 0,7) +
/// b x 0,3; senao sorteio(b).
pub(crate) unsafe fn defence_roll(j: usize, b: i16) -> i32 {
    let r = random(*((j + match_player::EFFECTIVE_ABILITY) as *const i16) as i32);
    if r > random(0x11d) {
        let x = b as f64;
        let t = random(ftol(x * read_f64(rdata::F64_0_7)));
        ftol(t as f64 + x * read_f64(rdata::F64_0_3))
    } else {
        random(b as i32)
    }
}

/// Save value of the goalkeeper (`this`, role 1, +0x2b <= 1) against a shot to (a, b) with code `codigo` (`a3` picks
/// the base word +0x16a or +0x168, + 220), adjusted by distance to the ball; writes `saida` (1 wrong side, 3 out of the
/// area, 2 out of reach, 4 for codes 26/27) and `rebote` when the final roll fails. Returns the save value, 0 when not
/// a goalkeeper.
///
/// [pt] { 0x700420 } (match_pl.cpp): valor da defesa do goleiro (papel 1, 0x2b <= 1) contra o lance
/// para (a, b) com `codigo`. Base pela palavra 0x16a (com `a3`) ou 0x168, + 220; a distancia a bola
/// (tabela 12 x 9) entra no fim. `saida`: 1 fora do lado, 3 fora da area, 2 sem alcance, 4 nos
/// codigos 26/27; `rebote` quando o sorteio final perde. Devolve ftol(base / 6 + v x c + c').
pub(crate) unsafe extern "thiscall" fn goalkeeper_save(this: usize, a: i8, b: i8, a3: u8, code: i8, out: *mut u8, rebote: *mut u8) -> u16 {
    *out = 0;
    *rebote = 0;
    let w = *((this + if a3 != 0 { 0x16a } else { 0x168 }) as *const i16);
    let base = (w as u16).wrapping_add(0xdc) as i16;
    let engine = *((this + match_player::ENGINE) as *const usize);
    let dy = (read_i8(this + match_player::CELL_Y) as i32 - read_i8(engine + engine::BALL_Y) as i32).abs() as i8 as isize;
    let dx = (read_i8(this + match_player::CELL_X) as i32 - read_i8(engine + engine::BALL_X) as i32).abs() as i8 as isize;
    let dist = read_f32((0xa15e44 + 4 * (dy * 9 + dx)) as usize) as f64;
    if read_i8(this + match_player::BUSY) > 1 || engine == 0 {
        return 0;
    }
    if exe::cm3_match_engine_get_player_position(engine, this) != 1 {
        return 0;
    }
    let far = |e: i32| if dist > read_f64(rdata::F64_2_1) { e + 10 - ftol(*((this + match_player::EFFECTIVE_ABILITY) as *const i16) as f64 * read_f64(rdata::F64_NEG_0_35)) } else { e };
    let near = |e: i32| if !(dist >= read_f64(rdata::F64_1)) { e + 100 } else { e };
    let e: i32 = if code == 0x33 {
        let b0 = base as i32;
        let v = random(b0);
        random(b0 - v)
    } else {
        let (l, x, y) = (side_of_player(this), read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y));
        let area = if l == 1 { 10..=11 } else { 0..=1 };
        if (l == 1 && y < b) || (l == 0 && y > b) {
            *out = 1;
            0
        } else if !(2..=6).contains(&x) || !area.contains(&y) {
            *out = 3;
            0
        } else {
            if exe::match_player_is_goal_side_of_cell(this, a, b) == 0 {
                *out = 2;
                defence_roll(this, base) - 200
            } else if y == 11 || y == 0 {
                match code {
                    26 => near(defence_roll(this, base) + 75),
                    27 => near(defence_roll(this, base) + 125),
                    _ => far(defence_roll(this, base)),
                }
            } else {
                match code {
                    3 | 5 => defence_roll(this, base.wrapping_sub(150)),
                    26 => {
                        *out = 4;
                        near(defence_roll(this, base.wrapping_sub(150)))
                    }
                    27 => {
                        *out = 4;
                        near(defence_roll(this, base.wrapping_sub(200)))
                    }
                    _ => far(defence_roll(this, base)),
                }
            }
        }
    };
    let engine = *((this + match_player::ENGINE) as *const usize);
    let r1 = random(ftol(read_f32(this + match_player::REFLEXES) as f64 * 2.0 + *((this + match_player::GOALS_CONCEDED) as *const u8) as f64 + read_f64(rdata::F64_10)));
    let r2 = random(ftol((read_f64(rdata::F64_2) - dist) * read_i8(engine + engine::BALL_ROLL) as f64 * read_f64(rdata::F64_10)));
    let v = if r1 > r2 {
        e as i16
    } else {
        *rebote = 1;
        defence_roll(this, e as i16) as i16
    };
    ftol((w as i32 / 6) as f64 + v as f64 * read_f64(rdata::F64_0_5714285714) + read_f64(rdata::F64_20)) as u16
}

/// Address of grid cell (x, y) in the engine occupancy grid (90 bytes per cell from `engine::GRID`).
///
/// [pt] Casa (x, y) da grade de ocupacao do motor: 90 bytes por casa, a partir de 0x215e; por lado,
/// lista de 11 ponteiros (44 bytes) e a quantidade em 0x58 + lado.
pub(crate) fn occupancy_grid_cell(engine: usize, x: i8, y: i8) -> isize {
    engine as isize + 0x215e + 90 * (12 * x as isize + y as isize)
}

/// Moves the match player to grid cell (x, y): removes him from his side's list in the old cell (the last one takes his
/// slot +0x28), appends him to the new cell's list, updates his position (+0x102/+0x103) and, if he has the ball, the
/// ball's position. Nothing happens when already there.
///
/// [pt] { 0x6f46b0 } (match_pl.cpp): move o jogador para (x, y): tira da lista do lado na casa antiga
/// (o ultimo da lista ocupa a vaga dele, 0x28) e poe no fim da lista da casa nova; atualiza a bola
/// se ele for o dono. Na mesma casa, nada. Estados impossiveis do original (lista vazia ao tirar,
/// mais de 11 ao por) mostram erro (e, no segundo, encerram); aqui, o primeiro so volta e o segundo
/// encerra como o original.
pub(crate) unsafe extern "thiscall" fn move_to(this: usize, x: i8, y: i8) {
    if x == read_i8(this + match_player::CELL_X) && y == read_i8(this + match_player::CELL_Y) {
        return;
    }
    let engine = *((this + match_player::ENGINE) as *const usize);
    let list = |home: isize, l: i8, i: isize| (home + 4 * (11 * l as isize + i)) as *mut usize;
    let how_many = |home: isize, l: i8| (home + 0x58 + l as isize) as *mut i8;
    let slot = (this + match_player::CELL_SLOT) as *mut i8;
    let new = occupancy_grid_cell(engine, x, y);
    if *slot > -1 {
        let old = occupancy_grid_cell(engine, read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y));
        let l = side_of_player(this);
        let n = *how_many(old, l);
        if n == 0 {
            return;
        }
        *list(old, l, *slot as isize) = *list(old, l, n as isize - 1);
        *how_many(old, l) = n.wrapping_sub(1);
        let v = *slot;
        let movido = *list(old, l, v as isize);
        if movido != 0 {
            *((movido + match_player::CELL_SLOT) as *mut i8) = v;
        }
        let l = side_of_player(this);
        *slot = *how_many(new, l);
        *how_many(new, l) = (*how_many(new, l)).wrapping_add(1);
        *list(new, side_of_player(this), *slot as isize) = this;
    } else {
        let l = side_of_player(this);
        *slot = *how_many(new, l);
        *how_many(new, l) = (*how_many(new, l)).wrapping_add(1);
        if *how_many(new, side_of_player(this)) > 11 {
            exe::crt_exit(-1);
        }
        *list(new, l, *slot as isize) = this;
    }
    *((this + match_player::CELL_X) as *mut i8) = x;
    *((this + match_player::CELL_Y) as *mut i8) = y;
    let engine = *((this + match_player::ENGINE) as *const usize);
    if this == *((engine + engine::BALL_HOLDER) as *const usize) {
        *((engine + engine::BALL_X) as *mut i8) = x;
        *((*((this + match_player::ENGINE) as *const usize) + 0x8ea8) as *mut i8) = read_i8(this + match_player::CELL_Y);
    }
}

/// Is the player inside his own penalty area (x 2..=6, y 10..=11 for side 1 else 0..=1)?
///
/// [pt] O jogador do lado esta na propria area (x 2..=6; lado 1: y 10..=11; senao y 0..=1)?
pub(crate) unsafe fn is_in_own_box(j: usize) -> bool {
    let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let area = if side_of_player(j) == 1 { 10..=11 } else { 0..=1 };
    (2..=6).contains(&x) && area.contains(&y)
}

/// Computes how well the player controls move `codigo`: his per-move skill word minus a fitness-based draw (+0x4d), the
/// difficulty of a loose ball's direction, the holder's +0x19c and, for some moves, 20 x (engine `engine::BALL_ROLL` - 1)^3. Far
/// from goal the move may fail outright (-20); a goalkeeper in his own box switches the move code, and weak moves
/// 19/33/34 may become 45. Sets the outputs `a`, `b` by the final code and returns the value.
///
/// [pt] { 0x70d690 } (match_pl.cpp): valor do dominio do lance `codigo` (palavra do jogador por codigo,
/// menos o sorteio pela condicao fisica 0x4d, a dificuldade pela direcao da bola sem dono, a
/// palavra 0x19c do dono e, em alguns codigos, 20 x (0x8eaa - 1)^3). Fora da area, sem o codigo
/// 0x15/0x25, a distancia ao gol pode fazer perder na hora (-20). O goleiro na propria area troca
/// o codigo (41, ou 1 com 0x8ea9 == 4); 19/33/34 com valor baixo viram 45 em metade das vezes.
/// As saidas (`a`, `b`) dependem do codigo final.
pub(crate) unsafe extern "thiscall" fn control_of_move(this: usize, code: *mut i8, a: *mut u8, b: *mut u8) -> u16 {
    let engine = *((this + match_player::ENGINE) as *const usize);
    let c8ea9 = read_i8(engine + engine::BALL_HEIGHT);
    *a = 0;
    *b = 0;
    let c0 = *code;
    if c0 != 0x25 && c0 != 0x15 && !is_in_own_box(this) {
        let (x, y) = (read_i8(this + match_player::CELL_X) as i32, read_i8(this + match_player::CELL_Y) as i32);
        let dy = (if side_of_player(this) == 1 { y - 0xb } else { y }).abs() as i8 as isize;
        let dx = (x - 4).abs() as i8 as isize;
        let t = read_f32((0xa15e44 + 4 * (dy * 9 + dx)) as usize) as f64;
        let r1 = random(ftol(read_f32(this + match_player::AGGRESSION) as f64 + read_f32(rdata::F32_2) as f64));
        if r1 < random(ftol(t + t)) {
            *b = 0;
            *a = 0;
            return 0xffec;
        }
    }
    let engine = *((this + match_player::ENGINE) as *const usize);
    let owner = *((engine + engine::BALL_HOLDER) as *const usize);
    if owner == 0 {
        let c = *code;
        let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
        if c != 0x15 && c != 0x25 && special(c) == 0 {
            exe::crt_exit(-1);
        }
    }
    let bp: u16 = if owner == this { *((owner + match_player::BALL_CONTROL) as *const u16) } else { 0 };
    let c = 10000 - ftol(read_f32(this + match_player::STAMINA) as f64) as i16 as i32;
    let r = random(c.wrapping_mul(c) / 50000) as u16;
    let mut dif: u16 = 0;
    if owner == 0 {
        let engine = *((this + match_player::ENGINE) as *const usize);
        let mut dd = (*((this + match_player::FACING) as *const i16) as i32 - *((engine + engine::BALL_DIRECTION) as *const i16) as i32).abs();
        if dd as i16 > 0xb4 {
            dd = 0x168 - dd;
        }
        let e = (0xb4 - dd) * 2;
        let f = (5 - read_i8(engine + engine::BALL_HEIGHT) as i32).max(1);
        dif = random(((e as i16 as i32) / (f as i16 as i32)) as i16 as i32) as u16;
    }
    let engine = *((this + match_player::ENGINE) as *const usize);
    let g = (read_i8(engine + engine::BALL_ROLL) as u8).wrapping_sub(1) as i8 as i16 as u16;
    let cubo = g.wrapping_mul(g).wrapping_mul(g).wrapping_mul(20);
    let si: u16 = if exe::cm3_match_engine_get_player_position(engine, this) == 1 && is_in_own_box(this) {
        let new: i8 = if c8ea9 != 4 { 0x29 } else { 1 };
        *code = new;
        read_u16(this + (0x11a + 2 * new as usize)).wrapping_sub(r).wrapping_sub(bp)
    } else {
        let minus = |v: u16| v.wrapping_sub(r).wrapping_sub(bp);
        let ou_45 = |v: u16| {
            if (v as i16) < 2 && random(2) != 0 {
                *code = 0x2d;
                minus(read_u16(this + 0x174))
            } else {
                v
            }
        };
        match *code {
            12 => read_u16(this + 0x132).wrapping_sub(cubo).wrapping_sub(r).wrapping_sub(dif).wrapping_sub(bp),
            21 => read_u16(this + 0x144).wrapping_sub(cubo).wrapping_sub(r).wrapping_sub(dif).wrapping_sub(bp),
            19 => ou_45(minus(read_u16(this + 0x140))),
            33 => ou_45(minus(read_u16(this + 0x15c))),
            34 => ou_45(minus(read_u16(this + 0x15e))),
            37 => read_u16(this + 0x164).wrapping_sub(bp),
            42 => minus(read_u16(this + 0x16e)),
            43 => minus(read_u16(this + 0x170)),
            44 => minus(read_u16(this + 0x172)),
            45 => minus(read_u16(this + 0x174)),
            46 => minus(read_u16(this + 0x176)),
            47 => minus(read_u16(this + 0x178)),
            49 => read_u16(this + 0x17c).wrapping_sub(bp),
            50 => read_u16(this + 0x17e).wrapping_sub(bp),
            _ => 0,
        }
    };
    let (sa, signed_byte_at): (u8, u8) = match *code {
        1 | 41 => {
            let r = random(2) as u8;
            (1, r.wrapping_add(2))
        }
        12 => (0, 0),
        19 | 33 | 45 => (2, 2),
        21 => (1, 2),
        34 => (random(2) as u8 + 1, 2),
        36 | 37 | 43 => (0, 2),
        42 | 46 => (random(2) as u8, 2),
        44 => (5, 2),
        47 => (random(4) as u8 + 1, 2),
        49 => (random(4) as u8 + 1, 0),
        50 => (random(2) as u8 + 1, 2),
        _ => (0, 0),
    };
    *b = signed_byte_at;
    *a = sa;
    si
}

/// TapeCall a 0x6d6f70 (enfileirar um evento de comentario): (this, 7 argumentos).
pub(crate) type AddEventCall<'a> = &'a mut dyn FnMut(usize, [u32; 7]);

/// Calls MATCH_EVENTS::add_event (0x6d6f70) with seven arguments.
///
/// [pt] 0x6d6f70 pelo endereco do jogo.
pub(crate) unsafe fn add_match_event(this: usize, a: [u32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_MATCH_EVENTS_ADD_EVENT);
    f(this, a[0], a[1], a[2], a[3], a[4], a[5], a[6])
}

/// 0x7002f0 with injected call: commentary event with two fields set to -1.
///
/// [pt] { 0x7002f0 } (match_pl.cpp) com a chamada injetada: comentario com dois campos -1.
pub(crate) unsafe fn commentary_two_blank(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, call: AddEventCall) {
    call(this, [a1, a2, a3, a4, u32::MAX, u32::MAX, a5]);
}

/// Queues a match commentary event (MATCH_EVENTS::add_event) with arguments a1..a5 and two fields set to -1.
///
/// [pt] { 0x7002f0 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com dois campos -1.
pub(crate) unsafe extern "thiscall" fn commentary_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    commentary_two_blank(this, a1, a2, a3, a4, a5, &mut |t, a| add_match_event(t, a))
}

/// 0x70a860 with injected call: commentary event with three fields set to -1.
///
/// [pt] { 0x70a860 } (match_pl.cpp) com a chamada injetada: comentario com tres campos -1.
pub(crate) unsafe fn commentary_three_blank(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, call: AddEventCall) {
    call(this, [a1, a2, u32::MAX, a3, u32::MAX, u32::MAX, a4]);
}

/// Queues a match commentary event with the given fields and three fields set to -1.
///
/// [pt] { 0x70a860 } (match_pl.cpp): enfileira um comentario (0x6d6f70) com tres campos -1.
pub(crate) unsafe extern "thiscall" fn commentary_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    commentary_three_blank(this, a1, a2, a3, a4, &mut |t, a| add_match_event(t, a))
}

/// Referee punishes the challenger's foul on the ball holder (`exe::cm3_match_engine_referee_decide_foul_punishment`): commentary, card, set piece 7, penalty or free
/// kick, sending off, taker, team anger, fatigue; challenger loses 25 stamina. Calls out of the engine go through the
/// injected closure.
///
/// [pt] A falta do desafiante `e` sobre `j` (tipo da 0x6cfc60, lance `lance`) punida pelo arbitro
/// (0x6cfcd0), como na dividida (0x70ec80) e no chute (0x6f6c00): narracao pelo lance, cartao, bola
/// parada 7, penalti (5) ou falta (6), expulsao, cobrador, as narracoes de sempre, o desgaste do time
/// e o cansaco; no fim `e` perde 25 de folego. `ramos`: penalti, expulsao, cobrador, vantagem.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn punish_challenger_foul_injected(m: usize, e: usize, j: usize, mut kind: u8, card: u8, grav: i8, risky: i8, advantage: u8, lance: i8, ajustar_no_penalti: bool, ramos: &[&'static str; 4], call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    let par = |p: usize| (read_u8(p + match_player::COMMENTARY_ID) as u32) | (read_u8(p + match_player::SIDE) as u32) << 8;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let in_the_box = |p: usize, lado1: bool| {
        (2..=6).contains(&read_i8(p + match_player::CELL_X)) && if lado1 { (10..=11).contains(&read_i8(p + match_player::CELL_Y)) } else { (0..=1).contains(&read_i8(p + match_player::CELL_Y)) }
    };
    let mut weight = 0f32;
    exe::cm3_match_engine_referee_decide_foul_punishment(m, e, j, &mut kind, card as i8, grav, risky as u8, &mut weight, advantage);
    narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
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
    } else if in_the_box(j, read_i8(j + match_player::SIDE) == 0) {
        if colour == -1 {
            match random(4) {
                0 => narrate(call, 0x20e9, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX),
                1 => narrate(call, 0x20ec, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX),
                _ => {}
            }
        }
        crate::comparison::branch(ramos[0]);
        sound = 0x1f7a;
        *b(m, engine::RESTART_TYPE) = 5;
        *((m + engine::FOULED_PLAYER) as *mut usize) = j;
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(500);
        // in the shot (0x6f6c00) the penalty still changes the scorer’s +0x17
        if ajustar_no_penalti && random(read_i8(*((e + match_player::STAFF) as *const usize) + 0x5a) as i32) == 0 {
            let r = random(10);
            if exe::match_player_adjust_morale(e, (r as u8).wrapping_neg() as i8) != 0 {
                exe::match_player_calc_ratings(e, 0);
            }
        }
    } else {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 6;
    }
    *b(m, engine::BALL_X) = *b(j, match_player::CELL_X);
    *b(m, engine::BALL_Y) = *b(j, match_player::CELL_Y);
    *b(m, engine::POSSESSION_SIDE) = *b(j, match_player::SIDE);
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    if sound == 0x1f7a {
        narrate(call, 0x1f7a, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
    }
    let sent_off = matches!(kind, 3..=6);
    if colour > -1 {
        if (lance == 0x2c || lance == 0x2f) && advantage != 0 {
            narrate(call, if lance == 0x2f { 0x21da } else { 0x21d9 }, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), par(e));
            if sent_off || sound == 0x1f7a {
                *b(e, match_player::SENT_OFF) = 1;
            }
        }
        if random(4) == 0 {
            let x = random(grav as i32);
            if x > random(0x14) {
                let t = *((m + engine::MATCH) as *const usize);
                let club = Club(*((t + if read_i8(j + match_player::SIDE) != 0 { 0x20 } else { 0x1c }) as *const usize));
                let k = club.nation().addr();
                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrate(call, 0x2109, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        }
        narrate(call, colour as u32, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        if sent_off {
            crate::comparison::branch(ramos[1]);
            player_left_pitch_with(m, e, 0, call);
        }
    }
    if sound != 0x1f7a {
        crate::comparison::branch(ramos[2]);
        let c = exe::cm3_match_engine_pick_restart_taker(m, read_i8(j + match_player::SIDE), read_i8(m + engine::RESTART_TYPE), 0);
        if c == 0 {
            narrate(call, sound, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        } else {
            if kind == 7 {
                narrate(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            } else {
                narrate(call, sound, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            }
            // the set piece far from the opponent’s goal (side 1: row < 4; side 0: > 7)
            let far = (read_i8(j + match_player::SIDE) == 1 && read_i8(j + match_player::CELL_Y) < 4) || (read_i8(j + match_player::SIDE) == 0 && read_i8(j + match_player::CELL_Y) > 7);
            if sound == 0x1f79 && advantage == 0 && far {
                if kind == 7 {
                    if !in_the_box(j, read_i8(j + match_player::SIDE) == 0) {
                        narrate(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                    }
                } else if (2..=6).contains(&read_i8(j + match_player::CELL_X)) {
                    narrate(call, 0x21e0, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                } else {
                    narrate(call, 0x21df, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                }
            }
        }
    }
    if special(lance) == 0 {
        let extra = sent_off || sound == 0x1f7a;
        if advantage != 0 {
            crate::comparison::branch(ramos[3]);
            narrate(call, 0x20dd, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            if kind == 4 || kind == 5 {
                *b(e, match_player::SENT_OFF) = 1;
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(j), u32::MAX);
            }
        } else if random(9) == 0 {
            narrate(call, 0x20df, u32::MAX, zero(j), u32::MAX);
            if random(3) == 0 {
                narrate(call, 0x20e0, u32::MAX, zero(j), u32::MAX);
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(j), u32::MAX);
            }
        }
        exe::cm3_match_engine_scale_side_anger_from_player(m, e, weight);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e, [0; 5]);
    }
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x19);
}

/// Commentary code for the foul by contest action code (tables 0x71026c/0x710328 of 0x70ec80).
///
/// [pt] Narracao da falta pelo lance em disputa (tabelas 0x71026c e 0x710328 da 0x70ec80).
pub(crate) fn foul_commentary_code(lance: i8) -> u32 {
    match lance {
        1 => 0x1ff1,
        0xc => 0x1ff4,
        0x13 | 0x21 => 0x1fee,
        0x15 => 0x1ff0,
        0x22 => 0x1ff2,
        0x29 => 0x1fef,
        0x2a => 0x1feb,
        0x2b => 0x1fec,
        0x2c => 0x1fed,
        0x2d => 0x1fea,
        0x2e => 0x1ff3,
        _ => 0x1fe9,
    }
}

/// Port of 0x70ec80 (ret 4, match_pl.cpp), the tackle against `j`, the ball holder, with the calls that touch things
/// outside the engine injected (0x6d6f70 commentates, 0x716e10 tires, 0x716c90 the incident of whoever lost the ball,
/// `take_ball` control, `exe::cm3_match_engine_move_ball_to_cell` the ball spills; and those inside the injury, the defence and leaving the pitch).
///
/// First of all it draws at random by stamina (+0x4d); it only continues if `j` has the ball (`engine::BALL_HOLDER`). With `forte`,
/// the opposing side gains 2.25 (`exe::cm3_match_engine_scale_side_anger`) and `j`'s side +10 possession. The attack is random(+0x19c - stamina +
/// +0x13a - random(`engine::BALL_HEIGHT` x 50)); the challenger comes from `exe::cm3_match_engine_get_opposition_challenge` (incident 0x10) with the rating, the incident and
/// the risk, and the injury in the tackle (0x6cfe10) comes right after. Without a challenger, or with the attack above
/// the rating, `j` gets past (see the end).
///
/// Otherwise, with the rating below the attack and random(5), the contest is clean; otherwise the referee decides
/// (`exe::cm3_match_engine_referee_judge_tackle`: foul, card, severity, advantage; 0x2168 in the area with `engine::LIVE_MATCH`). A foul with 1 in 30 without risk only
/// records the incident (`exe::cm3_match_engine_set_pending_foul`); otherwise it punishes (`exe::cm3_match_engine_referee_decide_foul_punishment`), commentates the foul by incident, picks the card
/// (0x1ff6 to 0x1ff9 or 0x210a by severity), marks set piece 7, a penalty (5, with 0x20e9/0x20ec without a card, -500
/// in the challenger's +0x35) or a free kick (6); commentates the card (0x21d9/0x21da on incident 0x2c/0x2f with
/// advantage, 1 in 4 0x2109 and 0x20f5), a sending-off removes the challenger (0x6cda20); without a penalty it picks
/// the taker (`exe::cm3_match_engine_pick_restart_taker`) and commentates the kick (0x21df to 0x21e2 by position); then 0x20dd/0x20df/0x20e0/0x20d9, the
/// challenger's team is worn down (`exe::cm3_match_engine_scale_side_anger_from_player`), he tires and loses 25 stamina.
///
/// Clean contest: the challenger marks the incident (+0x198), counts (+4), loses 59 or 70 stamina (`table_a`); with the
/// attack above the rating, `j` gets past; otherwise, in his own area by random(2), it commentates the foul and 0x20e8
/// to 0x20eb; otherwise the incident (0x70d430 on incident 1, which may end it;
/// 0x1f61/0x1f62/0x1f64/0x2150/0x2164/0x1f5d). The challenger controls (`take_ball`) with +0x19c = rating - attack - 200
/// (+-200); below the incident's limit (400, 250 or 100) the ball spills (`exe::cm3_match_engine_move_ball_to_cell`) and, without a set piece, he stays
/// on the ground (+0x2b) or, with a penalty given, `j` suffers (0x716c90); otherwise he turns for the incident
/// (`exe::match_player_set_facing_direction`), counts (+5) and, after minute 55/65, may commentate 0x21cd/0x21ce by the success average. He gains
/// stamina (0xc8 or 0x78, doubled outside tables 2/4) and `j` adds the challenge's first byte to +0x2b; 1 in 7
/// commentates 0x1f41.
///
/// When `j` gets past: outside the area, random(+0x31) < -10 makes `j` grow (+0x1a2 x 1.05, minimum (25 - +0x5d) x 2,
/// maximum 100, 0x211d); it counts possession (`engine::LAST_POSSESSION_SIDE`, `engine::POSSESSION_COUNTER`), +0x19c gains attack - rating (+-200), the challenger
/// adds the second byte; outside set pieces and incident 0x10 it commentates 0x2133 (with `forte`), 0x216d (with a
/// challenger), 0x21de or 0x1f40, and zeroes `engine::BALL_HEIGHT`.
///
/// [pt] { 0x70ec80 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x716c90 o lance de quem perdeu a bola, 0x6f0630 dominio, 0x6cd770 a bola
/// espirra; e as de dentro da lesao, da defesa e da saida de campo): a dividida contra `j`, dono da
/// bola. Antes de tudo sorteia pelo folego (+0x4d); so segue se `j` tem a bola (+0xf582). Com
/// `forte`, o lado adversario ganha 2,25 (0x6bb950) e o de `j` +10 de posse. O ataque e
/// sorteio(+0x19c - folego + +0x13a - sorteio(+0x8ea9 x 50)); o desafiante sai da 0x6c87e0 (lance
/// 0x10) com a nota, o lance e o risco, e a lesao na dividida (0x6cfe10) vem logo. Sem desafiante,
/// ou com o ataque acima da nota, `j` passa (ver o fim). Senao, com a nota abaixo do ataque e
/// sorteio(5), a disputa e limpa; senao o arbitro decide (0x6cfc60: falta, cartao, gravidade,
/// vantagem; 0x2168 na area com +0x8ebb). Falta com 1 em 30 sem risco so registra o lance
/// (0x6bb9a0); senao pune (0x6cfcd0), narra a falta pelo lance, escolhe o cartao (0x1ff6 a 0x1ff9 ou
/// 0x210a pela gravidade), marca bola parada 7, penalti (5, com 0x20e9/0x20ec sem cartao, -500 em
/// +0x35 do desafiante) ou falta (6); narra o cartao (0x21d9/0x21da no lance 0x2c/0x2f com vantagem,
/// 1 em 4 0x2109 e 0x20f5), a expulsao tira o desafiante (0x6cda20); sem penalti escolhe o cobrador
/// (0x6ceb90) e narra a cobranca (0x21df a 0x21e2 pela posicao); depois 0x20dd/0x20df/0x20e0/0x20d9,
/// o time do desafiante se desgasta (0x6bb8d0), ele cansa e perde 25 de folego. Disputa limpa: o
/// desafiante marca o lance (+0x198), conta (+4), perde 59 ou 70 de folego (0x6c38c0); com o ataque
/// acima da nota, `j` passa; senao, na propria area por sorteio(2), narra a falta e 0x20e8 a 0x20eb;
/// senao o lance (0x70d430 no lance 1, que pode encerrar; 0x1f61/0x1f62/0x1f64/0x2150/0x2164/0x1f5d).
/// O desafiante domina (0x6f0630) com +0x19c = nota - ataque - 200 (+-200); abaixo do limite do lance
/// (400, 250 ou 100) a bola espirra (0x6cd770) e, sem bola parada, ele fica no chao (+0x2b) ou, com
/// penalti marcado, `j` sofre (0x716c90); senao vira para o lance (0x6f4920), conta (+5) e, depois do
/// minuto 55/65, pode narrar 0x21cd/0x21ce pela media de acertos. Ganha folego (0xc8 ou 0x78, dobrado
/// fora das tabelas 2/4) e `j` soma o primeiro byte do desafio a +0x2b; 1 em 7 narra 0x1f41. Quando
/// `j` passa: fora da area, sorteio(+0x31) < -10 faz `j` crescer (+0x1a2 x 1,05, minimo
/// (25 - +0x5d) x 2, maximo 100, 0x211d); conta a posse (+0x8eab, +0x8d46), +0x19c ganha ataque -
/// nota (+-200), o desafiante soma o segundo byte; fora de bola parada e do lance 0x10 narra 0x2133
/// (com `forte`), 0x216d (com desafiante), 0x21de ou 0x1f40, e zera +0x8ea9.
pub(crate) unsafe fn tackle_injected(j: usize, strong: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let m = *((j + match_player::ENGINE) as *const usize);
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    let par = |p: usize| (read_u8(p + match_player::COMMENTARY_ID) as u32) | (read_u8(p + match_player::SIDE) as u32) << 8;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    // in the area of whoever defends against `p`’s side (columns 2 to 6, rows 10-11 or 0-1)
    let in_the_box = |p: usize, lado1: bool| {
        (2..=6).contains(&read_i8(p + match_player::CELL_X)) && if lado1 { (10..=11).contains(&read_i8(p + match_player::CELL_Y)) } else { (0..=1).contains(&read_i8(p + match_player::CELL_Y)) }
    };
    let d = 10000 - ftol(read_f32(j + match_player::STAMINA) as f64) as i16 as i32;
    let folego = random(d.wrapping_mul(d) / 50000);
    let (mut a, mut b2, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let mut advantage = 0u8;
    let mut penalty = false;
    if j != *((m + engine::BALL_HOLDER) as *const usize) {
        return;
    }
    if strong != 0 {
        exe::cm3_match_engine_scale_side_anger(m, (read_i8(j + match_player::SIDE) == 0) as i8, 2.25);
        let p = (m as isize + read_i8(j + match_player::SIDE) as isize + 0x8d46) as *mut u8;
        *p = (*p).wrapping_add(10);
    }
    let base = *w(j, 0x13a) as i32;
    let r = random(read_i8(m + engine::BALL_HEIGHT) as i32 * 50);
    let attack = random(*w(j, match_player::BALL_CONTROL) as i32 - folego as i16 as i32 + base - r);
    let at = attack as i16;
    let e = exe::cm3_match_engine_get_opposition_challenge(m, j, read_i8(j + match_player::SIDE), 0x10, &mut a, &mut b2, &mut nota, &mut lance, &mut arriscado, 0);
    tackle_injury_with(m, j, 0x10, e, lance as i16, 0, call);
    let passes = 'contest: {
        if e == 0 {
            crate::comparison::branch("tackle no challenger");
            break 'contest true;
        }
        let mut kind = 0u8;
        let (mut cartao, mut grav) = (0u8, 0i8);
        if !(nota < at && random(5) != 0) {
            kind = exe::cm3_match_engine_referee_judge_tackle(m, e, j, lance, -1, &mut cartao, &mut grav, &mut advantage);
            if read_i8(m + engine::LIVE_MATCH) != 0 && in_the_box(e, read_i8(j + match_player::SIDE) == 0)
                && (special(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
                && random(3) != 0
            {
                narrate(call, 0x2168, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
            }
            if kind != 0 {
                if random(0x1e) == 0 && arriscado == 0 {
                    crate::comparison::branch("tackle only records");
                    exe::cm3_match_engine_set_pending_foul(m, lance as i16 as u16, e as u32, j as u32, 0, kind, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav as u8);
                } else {
                    // the foul with a punishment
                    crate::comparison::branch("tackle foul punished");
                    punish_challenger_foul_injected(m, e, j, kind, cartao, grav, arriscado, advantage, lance, false, &["tackle penalty", "tackle expulsao", "tackle pick_restart_taker", "tackle vantagem"], call);
                    return;
                }
            }
        }
        if kind == 0 && advantage != 0 {
            penalty = in_the_box(j, read_i8(j + match_player::SIDE) == 0) || arriscado != 0;
        }
        crate::comparison::branch("tackle clean contest");
        // a disputa limpa
        *w(e, match_player::ACTION) = lance as i16;
        *b(e, 4) = (*b(e, 4)).wrapping_add(1);
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(if exe::cm3_match_engine_get_player_position(m, e) & 0x30 != 0 && exe::cm3_match_engine_get_player_position(m, e) & 0x880 != 0 { 0x3b } else { 0x46 });
        if at > nota {
            crate::comparison::branch("tackle clean pass");
            break 'contest true;
        }
        let narrated = (special(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
            && in_the_box(e, read_i8(e + match_player::SIDE) == 1)
            && random(2) == 0;
        if narrated {
            crate::comparison::branch("tackle narrates foul in the box");
            narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
            let code = match random(0x32) {
                1 => 0x20ea,
                0 | 2 | 4 | 6 | 8 | 10 => 0x20eb,
                _ => 0x20e8,
            };
            narrate(call, code, u32::MAX, zero(e), u32::MAX);
        } else {
            let code = match lance {
                1 => {
                    crate::comparison::branch("tackle save");
                    let strength = nota.wrapping_sub(at);
                    let r = save_shot_injected(e, strength, &mut |rotina, this, x| {
                        if rotina == 0x6d6f70 {
                            // the defense narrates with the fifth field whole and the sixth -1
                            call(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                        } else {
                            call(rotina, this, x)
                        }
                    });
                    if r == 0 {
                        return;
                    }
                    0
                }
                0x13 | 0x21 | 0x2d => 0x1f62,
                0x15 | 0x2e => {
                    let r = random(6);
                    if ((r as f64 + read_f32(e + match_player::JUMPING) as f64) + 3.0) < read_f32(j + match_player::JUMPING) as f64 { 0x2164 } else { 0x1f5d }
                }
                0x22 | 0x2c => 0x2150,
                0x29 => 0x1f61,
                0x2a | 0x2b => 0x1f64,
                _ => 0,
            };
            if code != 0 {
                narrate(call, code, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
            }
        }
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, e, [0; 5]);
        *w(e, match_player::BALL_CONTROL) = nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200);
        let mut controlled = false;
        let limit = match lance {
            0x13 => 400,
            0x21 | 0x2d => 250,
            _ => 100,
        };
        if *w(e, match_player::BALL_CONTROL) < limit && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 {
            crate::comparison::branch("tackle ball breaks loose");
            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(j + match_player::CELL_X) as u32), (read_u8(j + match_player::CELL_Y) as u32), 1 | zero(j) << 8, 0, 0]);
            if read_i8(m + engine::RESTART_TYPE) == 0 {
                let mut k = 3;
                k -= (read_i8(e + match_player::MOMENT_WORK_RATE) as i32 > random(10)) as i32;
                k -= (read_i8(e + match_player::MOMENT_AGILITY) as i32 > random(10)) as i32;
                *b(e, match_player::BUSY) = random(k) as u8;
                controlled = true;
            } else if penalty {
                crate::comparison::branch("tackle 716c90");
                call(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, j, [0; 5]);
            }
        } else {
            if (0..4).contains(&read_i8(m + engine::BALL_HEIGHT)) {
                *b(m, engine::BALL_HEIGHT) = 0;
            }
            let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
            if read_i8(e + match_player::SIDE) == 0 && read_i8(m + engine::BALL_HEIGHT) != -1 {
                exe::match_player_set_facing_direction(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                exe::match_player_set_facing_direction(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
            let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
            let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
            let average = if read_i8(e + contas.0) != 0 { ((read_u8(e + contas.1) as u32) as f64 / (read_u8(e + contas.0) as u32) as f64) as f32 } else { 0.0 };
            if minute > (random(0x14) + sorteio) as u8 as i8 {
                let goals = read_i8(m + (if read_i8(e + match_player::SIDE) != 0 { 0xf5bd } else { 0xf5f3 }));
                let attempts = if lance == 0x15 { (read_u8(e + match_player::CHALLENGES_WON) as u32) > 3 && read_i8(e + 9) != 0 } else { (read_u8(e + match_player::KEY_TACKLES) as u32) > 1 };
                if goals == 0 && average as f64 > 0.725 && attempts && *w(e, match_player::MATCH_RATING) > 0x1a5e && read_i8(e + match_player::TACKLE_RATE_NARRATED) == 0 {
                    crate::comparison::branch("tackle success average");
                    narrate(call, if lance == 0x15 { 0x21ce } else { 0x21cd }, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
                    *b(e, match_player::TACKLE_RATE_NARRATED) = 1;
                }
            }
        }
        *b(e, 5) = (*b(e, 5)).wrapping_add(1);
        let mm = *((e + match_player::ENGINE) as *const usize);
        let t1 = if mm != 0 { exe::cm3_match_engine_get_player_position(mm, e) } else { 0 };
        let far = t1 & 4 == 0 && (if mm != 0 { exe::cm3_match_engine_get_player_position(mm, e) } else { 0 }) & 2 == 0;
        let gain: i16 = if controlled { 0x78 } else { 0xc8 };
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(if far { gain << 1 } else { gain });
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(a);
        if controlled && random(7) == 0 {
            narrate(call, 0x1f41, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        }
        return;
    };
    let _ = passes;
    // `j` passa
    if !in_the_box(j, read_i8(j + match_player::SIDE) == 0) && random(*w(j, 0x31) as i32) < -10 {
        crate::comparison::branch("tackle grows");
        let p = read_f32(j + match_player::CONFIDENCE) as f64 * 1.0499999523162842;
        *((j + match_player::CONFIDENCE) as *mut f32) = p as f32;
        let minimum = ((25 - read_i8(*((j + match_player::STAFF) as *const usize) + 0x5d) as i32) * 2) as f32;
        let v = if p > minimum as f64 { p } else { minimum as f64 };
        *((j + match_player::CONFIDENCE) as *mut f32) = if v > 100.0 { 100.0 } else { v as f32 };
        narrate(call, 0x211d, u32::MAX, zero(j), u32::MAX);
    }
    let side = read_i8(j + match_player::SIDE);
    *b(m, engine::LAST_POSSESSION_SIDE) = side as u8;
    if side != -1 {
        let p = (m as isize + side as isize + 0x8d46) as *mut u8;
        *p = (*p).wrapping_add(1);
        let q = b(m, 0x8d46 + (side == 0) as usize);
        if (*q as i8) > 0 {
            *q = (*q).wrapping_sub(10);
        }
    }
    *w(j, match_player::BALL_CONTROL) = (*w(j, match_player::BALL_CONTROL)).wrapping_add(at.wrapping_sub(nota)).clamp(-200, 200);
    if e != 0 {
        *b(e, match_player::BUSY) = (*b(e, match_player::BUSY)).wrapping_add(b2);
    }
    if read_i8(m + engine::BALL_HEIGHT) == -1 || *w(j, match_player::ACTION) == 0x10 {
        return;
    }
    if strong != 0 {
        narrate(call, 0x2133, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
    } else if e != 0 {
        narrate(call, 0x216d, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), (read_u8(e + match_player::COMMENTARY_ID) as u32) | zero(j) << 8);
    } else {
        let t = if m != 0 { exe::cm3_match_engine_get_player_position(m, j) } else { 0 };
        let near = (side == 1 && read_i8(j + match_player::CELL_Y) < 3) || (side == 0 && read_i8(j + match_player::CELL_Y) > 8);
        if t & 1 == 0 && !near && exe::cm3_match_engine_count_players_near_cell(m, side, read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y), 1) == 0 {
            narrate(call, 0x21de, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        } else {
            narrate(call, 0x1f40, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        }
    }
    *b(m, engine::BALL_HEIGHT) = 0;
}

/// Rust-mode dispatcher: forwards the injected calls of the tackle (commentary, fatigue, ball moves, injury...) to the
/// real game routines.
///
/// [pt] Despacho real das chamadas injetadas da dividida (e das rotinas que ela leva junto).
pub(crate) unsafe fn dispatch_tackle_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        // code | a7 << 16 (a7 signed), a5 | a6 << 8
        0x6d6f70 => add_match_event(this, [a[0] & 0xffff, a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, (a[0] >> 16) as u8 as i8 as u32]),
        0x716e10 | 0x6e1090 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(routine);
            f(this, a[0]);
        }
        0x716c90 | 0x6f0630 | 0x6e1aa0 | 0x7103e0 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(routine);
            f(this);
        }
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            return f(this, a[0] as u8) as u32;
        }
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, 1, a[4] as usize);
        }
        0x62ed70 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
            return f(this, a[0] as usize) as u32;
        }
        0x6d0980 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CM3_MATCH_ENGINE_INJURE_PLAYER);
            f(this, a[0], a[1], a[2] as usize, a[3] as usize);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32 = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
            return f(this, a[0] as usize, a[1] as u8 as i8, a[2]);
        }
    }
    0
}

/// Hook entry for 0x70ec80 (tackle_injected): the tackle contest on ball holder `j` (`forte` = a hard tackle): decides
/// who wins, fouls and penalties, ball deflection, stamina changes, possession and commentary.
///
/// [pt] { 0x70ec80 } (ret 4, match_pl.cpp): ver `tackle_injected`.
pub(crate) unsafe extern "thiscall" fn tackle(j: usize, strong: u32) {
    tackle_injected(j, strong as u8, &mut |rotina, this, a| dispatch_tackle_calls(rotina, this, a))
}

/// Stack probe: fills 64 KB of stack below with 0xa5, so an original routine called later that reads garbage
/// is exposed.
///
/// [pt] Sonda: enche 4 KB da pilha abaixo com 0xa5 (o lixo que uma rotina original chamada depois leria).
#[inline(never)]
pub(crate) fn poison_stack() {
    let mut a = [0xa5u8; 65536];
    std::hint::black_box(&mut a);
}

/// The foul by `j` (type `t` from `exe::cm3_match_engine_referee_check_foul`, incident `codigo`) punished by the referee (`exe::cm3_match_engine_referee_decide_booking`) on the shot
/// (`shot`), identical to the stretch in `exe::match_player_try_intercept_ball`; returns whether it was a penalty.
///
/// `exe::match_player_try_intercept_ball` keeps its own inline copy: calling this function from there changes the career on 2002-01-12 without
/// changing the logic (see PENDENCIAS).
///
/// It does: 0x1fe8, card, set piece 7, penalty (5, −500 at +0x35 and, by random(the player sheet's +0x5a), an
/// adjustment of +0x17 and the performance `exe::match_player_calc_ratings`) or free kick (6), sending-off, taker (`exe::cm3_match_engine_pick_restart_taker`) and the
/// commentaries 0x21df to 0x21e2, and outside the special incidents 0x20df/0x20e0/0x20d9, the team's wear (`exe::cm3_match_engine_scale_side_anger_from_player`)
/// and fatigue.
///
/// [pt] A falta de `j` (tipo `t` da 0x6cfca0, lance `codigo`) punida pelo arbitro (0x6cfd10) no chute
/// (0x6f6c00), igual ao trecho da 0x6f5c30. A 0x6f5c30 guarda a sua copia em linha: chamar esta
/// funcao de la muda a carreira em 2002-01-12 sem mudar a logica (ver PENDENCIAS): 0x1fe8, cartao, bola parada 7, penalti (5, -500 em +0x35 e, por
/// sorteio(+0x5a da ficha), ajuste do +0x17 e desempenho 0x6eecb0) ou falta (6), expulsao, cobrador
/// (0x6ceb90) e as narracoes 0x21df a 0x21e2, e fora dos lances especiais 0x20df/0x20e0/0x20d9, o
/// desgaste do time (0x6bb8d0) e o cansaco. Devolve se foi penalti.
pub(crate) unsafe fn punish_contester_foul_injected(j: usize, mut t: u8, card: u8, grav: u8, code: i16, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> bool {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let m = *((j + match_player::ENGINE) as *const usize);
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&read_i8(p + match_player::CELL_X)) && if lado1 { (10..=11).contains(&read_i8(p + match_player::CELL_Y)) } else { (0..=1).contains(&read_i8(p + match_player::CELL_Y)) }
    };
    let mut weight = 0f32;
    exe::cm3_match_engine_referee_decide_booking(m, j, &mut t, card as i8, grav as i8, 0, &mut weight);
    narrate(call, 0x1fe8, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
    let colour: i32 = match t {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        6 => 0x1ff5,
        _ => {
            if grav as i8 as i32 > random(7) + 2 { 0x210a } else { -1 }
        }
    };
    let sound: u32;
    if t == 7 {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 7;
    } else if area_de(j, read_i8(j + match_player::SIDE) == 1) {
        if colour == -1 {
            match random(4) {
                0 => narrate(call, 0x20e9, u32::MAX, zero(j), u32::MAX),
                1 => narrate(call, 0x20ec, u32::MAX, zero(j), u32::MAX),
                _ => {}
            }
        }
        sound = 0x1f7a;
        *b(m, engine::RESTART_TYPE) = 5;
        *((m + engine::FOULED_PLAYER) as *mut usize) = 0;
        *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(500);
        if random(read_i8(*((j + match_player::STAFF) as *const usize) + 0x5a) as i32) == 0 {
            let r = random(10);
            if exe::match_player_adjust_morale(j, (r as u8).wrapping_neg() as i8) != 0 {
                exe::match_player_calc_ratings(j, 0);
            }
        }
    } else {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 6;
    }
    *b(m, engine::BALL_X) = *b(j, match_player::CELL_X);
    *b(m, engine::BALL_Y) = *b(j, match_player::CELL_Y);
    *b(m, engine::POSSESSION_SIDE) = zero(j) as u8;
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    if sound == 0x1f7a {
        narrate(call, 0x1f7a, u32::MAX, zero(j), u32::MAX);
    }
    let sent_off = matches!(t, 3..=6);
    if colour > -1 {
        if random(4) == 0 {
            let a = random(grav as i8 as i32);
            if a > random(0x14) {
                let partida = *((m + engine::MATCH) as *const usize);
                let club = Club(*((partida + if read_i8(j + match_player::SIDE) == 0 { 0x20 } else { 0x1c }) as *const usize));
                let k = club.nation().addr();
                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrate(call, 0x2109, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        }
        narrate(call, colour as u32, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        if sent_off {
            player_left_pitch_with(m, j, 0, call);
        }
    }
    if sound != 0x1f7a {
        let c = exe::cm3_match_engine_pick_restart_taker(m, (read_i8(j + match_player::SIDE) == 0) as i8, read_i8(m + engine::RESTART_TYPE), 0);
        if c == 0 {
            narrate(call, sound, u32::MAX, zero(j), u32::MAX);
        } else {
            if t == 7 {
                narrate(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
            } else {
                narrate(call, sound, u32::MAX, zero(j), u32::MAX);
            }
            if sound == 0x1f79 {
                // the taker far from the goal he attacks (side 1: row < 4; side 0: > 7)
                let far = (read_i8(c + match_player::SIDE) == 1 && read_i8(c + match_player::CELL_Y) < 4) || (read_i8(c + match_player::SIDE) == 0 && read_i8(c + match_player::CELL_Y) > 7);
                if t == 7 {
                    if far && !area_de(c, read_i8(j + match_player::SIDE) == 1) {
                        narrate(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                    }
                } else if far {
                    narrate(call, if (2..=6).contains(&read_i8(j + match_player::CELL_X)) { 0x21e0 } else { 0x21df }, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                }
            }
        }
    }
    if special(code as i8) == 0 {
        if random(9) == 0 {
            narrate(call, 0x20df, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            if random(3) == 0 {
                narrate(call, 0x20e0, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            }
            if (sent_off || sound == 0x1f7a) && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            }
        }
        exe::cm3_match_engine_scale_side_anger_from_player(m, j, weight);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [0; 5]);
    }
    sound == 0x1f7a
}

/// Player `j` contests the incoming ball with `forca` (move `tipo` 0..5, height `p4`): `0x6f5c30` (ret 0x10,
/// `match_pl.cpp`) with the calls that touch things outside the engine injected. Returns 1 if the play resolved on him
/// (he controlled it, was fouled, the ball broke loose), 0 if not.
///
/// Injected: `0x6d6f70` narrates, `0x716e10` tires, `0x6cd770` the ball breaks loose, `0x6f0630` control; and those
/// inside the leaving-the-pitch path.
///
/// The taker (`+0xf576`) does not contest; on the side without the ball, with `+0x10d` × 2 < random draw(6) and the row
/// outside the limit (`0x6cb930`), neither. The attack is random draw(2000 − `+0xa5` × 40) for the goalkeeper in his
/// own area, otherwise by type (500, 600, 700, 1000, 1800; type 5 is random draw(500000) + 200 and becomes 4); with
/// `p4` < 1.5 types 4 and 5 do not contest; with height, attack × (`+0x8eaa` + 1) / (`p4` + 1).
///
/// With the ball on his own side (`+0x8eab`) and random draw(attack + 100) == 0: 0x1f5e, −300 in `+0x4782` and the ball
/// breaks loose. Otherwise the move and the attribute: in his own area the goalkeeper (or, outside the area and by
/// random draw, the header 0x25 with `+0x164`) chooses between control (1, `+0x11c`), 0x15 (`+0x144`), 0x24 (`+0x162`)
/// and 0x29 (`+0x16c`); the score is random draw(attribute) − `forca` / 2 − attack: below −100 he only gains random
/// draw(2) in `+0x2b`; below 150 nothing.
///
/// The referee looks at the foul (`0x6cfca0`): 1 in 10 records it (`0x6bb9a0`), otherwise punishes (`0x6cfd10`) with
/// 0x1fe8, card, set piece 7, penalty (5, −500 in `+0x35` and, by random draw(`+0x5a` of the player sheet), adjustment
/// of `+0x17` and performance `0x6eecb0`) or foul (6), sending-off, taker (`0x6ceb90`) and the narrations 0x21df to
/// 0x21e2, and outside the special moves 0x20df/0x20e0/0x20d9, the team's wear (`0x6bb8d0`) and the fatigue.
///
/// Without punishment: the limit is 1500 − `p4`^2 × 100 + `+0x8eaa` × 100 (minimum 160); with the score within it, `p4`
/// >= 1 and above `+0x8eaa` + 1, the player controls the ball (`+0x2b`, 0x1fe2 for the goalkeeper in the area or
/// 0x1f5f, `0x6f0630`, marks the move, 0x21cd/0x21ce by the success average); otherwise he loses it (0x20d4 for the
/// goalkeeper in the area, or 0x213e/0x1f71) and the ball breaks loose (`0x6cd770`). With `conta`, +1 in `+0x12` and
/// stamina (`+0x1c` or +10).
///
/// [pt] { 0x6f5c30 } (ret 0x10, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x6cd770 a bola espirra, 0x6f0630 dominio; e as de dentro da saida de
/// campo): o jogador `j` disputa a bola que chega com `forca` (lance `tipo` 0..5, altura `p4`).
/// Devolve 1 se a jogada se resolveu nele (dominou, sofreu falta, a bola espirrou), 0 se nao.
/// Quem cobra (+0xf576) nao disputa; do lado sem a bola, com +0x10d x 2 < sorteio(6) e a linha
/// fora do limite (0x6cb930), tambem nao. O ataque e sorteio(2000 - +0xa5 x 40) para o goleiro na
/// propria area, senao pelo tipo (500, 600, 700, 1000, 1800; o 5 vale sorteio(500000) + 200 e vira
/// 4); com `p4` < 1,5 os tipos 4 e 5 nao disputam; com altura, ataque x (+0x8eaa + 1) / (p4 + 1). Com
/// a bola no proprio lado (+0x8eab) por sorteio(ataque + 100) == 0: 0x1f5e, -300 em +0x4782 e a bola
/// espirra. Senao o lance e o atributo: na propria area o goleiro (ou, sem area e por sorteio, a
/// cabecada 0x25 com +0x164) escolhe entre domínio (1, +0x11c), 0x15 (+0x144), 0x24 (+0x162) e 0x29
/// (+0x16c); a nota e sorteio(atributo) - forca / 2 - ataque: abaixo de -100 so ganha sorteio(2) em
/// +0x2b; abaixo de 150 nada. O arbitro olha a falta (0x6cfca0): 1 em 10 a registra (0x6bb9a0),
/// senao pune (0x6cfd10) com 0x1fe8, cartao, bola parada 7, penalti (5, -500 em +0x35 e, por
/// sorteio(+0x5a da ficha), ajuste do +0x17 e desempenho 0x6eecb0) ou falta (6), expulsao, cobrador
/// (0x6ceb90) e as narracoes 0x21df a 0x21e2, e fora dos lances especiais 0x20df/0x20e0/0x20d9, o
/// desgaste do time (0x6bb8d0) e o cansaco. Sem punicao: o limite e 1500 - p4^2 x 100 + +0x8eaa x 100
/// (minimo 160); com a nota nele, p4 >= 1 e acima de +0x8eaa + 1, o jogador domina (+0x2b, 0x1fe2 do
/// goleiro na area ou 0x1f5f, 0x6f0630, marca o lance, 0x21cd/0x21ce pela media de acertos); senao
/// perde (0x20d4 do goleiro na area, ou 0x213e/0x1f71) e a bola espirra (0x6cd770). Com `conta`, +1
/// em +0x12 e folego (+0x1c ou +10).
pub(crate) unsafe fn contest_ball_injected(j: usize, strength: i16, kind_in: i8, p4: f32, count: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let m = *((j + match_player::ENGINE) as *const usize);
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    // in the area that `p`’s side defends (side 1: rows 10-11; the others: 0-1), columns 2 to 6
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&read_i8(p + match_player::CELL_X)) && if lado1 { (10..=11).contains(&read_i8(p + match_player::CELL_Y)) } else { (0..=1).contains(&read_i8(p + match_player::CELL_Y)) }
    };
    let in_the_box = || area_de(j, read_i8(j + match_player::SIDE) == 1);
    if *((m + engine::PASSER) as *const usize) == j {
        return 0;
    }
    if read_i8(m + engine::POSSESSION_SIDE) != read_i8(j + match_player::SIDE) {
        let c = read_i8(j + match_player::MOMENT_WORK_RATE) as i32;
        if c * 2 < random(6) && exe::is_in_attacking_third((read_i8(j + match_player::SIDE) == 0) as u8, read_i8(j + match_player::CELL_Y)) == 0 {
            return 0;
        }
    }
    let mut kind = kind_in;
    let mut attack: i32 = if exe::cm3_match_engine_get_player_position(m, j) == 1 && in_the_box() {
        crate::comparison::branch("respond goalkeeper in the box");
        random(ftol(2000.0 - read_f32(j + match_player::HANDLING) as f64 * 40.0))
    } else {
        match kind {
            0 => random(500),
            1 => random(600),
            2 => random(700),
            3 => random(1000),
            4 => random(0x708),
            5 => {
                crate::comparison::branch("respond type 5");
                kind = 4;
                random(500000) + 200
            }
            _ => {
                // simplification: the original routine shows the “Error” box (version, file, line 0x17e2); type
                // outside 0..5 does not happen in the game
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                return 0;
            }
        }
    };
    if (p4 as f64) < 1.5 && kind >= 4 {
        return 0;
    }
    let st = p4 as f64 + 1.0;
    if st > 0.0 {
        attack = ftol(((read_i8(m + engine::BALL_ROLL) as i32 + 1) * attack as i16 as i32) as f64 / st);
    }
    let espirra = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, inside: u32, shooter: usize| {
        let r = random(read_i8(m + engine::BALL_ROLL) as i32);
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(j + match_player::CELL_X) as u32), (read_u8(j + match_player::CELL_Y) as u32), (r.wrapping_add(1) as u8 as u32) | (read_u8(j + match_player::SIDE) as u32) << 8, inside, shooter as u32]);
    };
    if read_i8(m + engine::LAST_POSSESSION_SIDE) == read_i8(j + match_player::SIDE) {
        if random(attack as i16 as i32 + 100) != 0 {
            return 0;
        }
        crate::comparison::branch("respond ball on own side");
        narrate(call, 0x1f5e, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        espirra(call, 0, j);
        return 1;
    }
    // the play and the attribute that contests it
    let em_area = in_the_box();
    let choose = |kind: i8| -> (i16, i16) {
        if kind == 4 {
            if exe::cm3_match_engine_get_player_position(m, j) == 1 { (*w(j, 0x11c), 1) } else { (*w(j, 0x144), 0x15) }
        } else if exe::cm3_match_engine_get_player_position(m, j) != 1 {
            (*w(j, 0x162), 0x24)
        } else if kind == 3 && random(4) != 0 {
            (*w(j, 0x11c), 1)
        } else {
            (*w(j, 0x16c), 0x29)
        }
    };
    let (valor, code) = if em_area && exe::cm3_match_engine_get_player_position(m, j) == 1 {
        choose(kind)
    } else {
        let ficha5c = read_i8(*((j + match_player::STAFF) as *const usize) + 0x5c) as i32;
        if random(em_area as i32 * 500 + 100 + ficha5c * 200) != 0 { choose(kind) } else { (*w(j, 0x164), 0x25) }
    };
    let v = random(valor as i32);
    let rating = (v - strength as i32 / 2 - attack) as i16;
    if rating < -100 {
        crate::comparison::branch("respond very low rating");
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(2) as u8);
        return 0;
    }
    if rating < 0x96 {
        return 0;
    }
    let (mut cartao, mut grav) = (0u8, 0u8);
    let mut t = exe::cm3_match_engine_referee_check_foul(m, j, code as u16, &mut cartao, &mut grav);
    if t != 0 {
        if random(10) == 0 {
            crate::comparison::branch("respond foul recorded");
            exe::cm3_match_engine_set_pending_foul(m, code as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);
        } else {
            // the foul with a punishment
            crate::comparison::branch("respond foul punished");
            let mut weight = 0f32;
            exe::cm3_match_engine_referee_decide_booking(m, j, &mut t, cartao as i8, grav as i8, 0, &mut weight);
            narrate(call, 0x1fe8, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            let colour: i32 = match t {
                2 => 0x1ff9,
                3 => 0x1ff7,
                4 => 0x1ff8,
                6 => 0x1ff5,
                _ => {
                    if grav as i8 as i32 > random(7) + 2 { 0x210a } else { -1 }
                }
            };
            let sound: u32;
            if t == 7 {
                sound = 0x1f79;
                *b(m, engine::RESTART_TYPE) = 7;
            } else if in_the_box() {
                if colour == -1 {
                    match random(4) {
                        0 => narrate(call, 0x20e9, u32::MAX, zero(j), u32::MAX),
                        1 => narrate(call, 0x20ec, u32::MAX, zero(j), u32::MAX),
                        _ => {}
                    }
                }
                crate::comparison::branch("respond penalty");
                sound = 0x1f7a;
                *b(m, engine::RESTART_TYPE) = 5;
                *((m + engine::FOULED_PLAYER) as *mut usize) = 0;
                *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(500);
                if random(read_i8(*((j + match_player::STAFF) as *const usize) + 0x5a) as i32) == 0 {
                    let r = random(10);
                    if exe::match_player_adjust_morale(j, (r as u8).wrapping_neg() as i8) != 0 {
                        exe::match_player_calc_ratings(j, 0);
                    }
                }
            } else {
                sound = 0x1f79;
                *b(m, engine::RESTART_TYPE) = 6;
            }
            *b(m, engine::BALL_X) = *b(j, match_player::CELL_X);
            *b(m, engine::BALL_Y) = *b(j, match_player::CELL_Y);
            *b(m, engine::POSSESSION_SIDE) = zero(j) as u8;
            *((m + engine::BALL_HOLDER) as *mut u32) = 0;
            if sound == 0x1f7a {
                narrate(call, 0x1f7a, u32::MAX, zero(j), u32::MAX);
            }
            let sent_off = matches!(t, 3..=6);
            if colour > -1 {
                if random(4) == 0 {
                    let a = random(grav as i8 as i32);
                    if a > random(0x14) {
                        let partida = *((m + engine::MATCH) as *const usize);
                        let club = Club(*((partida + if read_i8(j + match_player::SIDE) == 0 { 0x20 } else { 0x1c }) as *const usize));
                        let k = club.nation().addr();
                        if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                            narrate(call, 0x20f5, 0, 0, u32::MAX);
                        }
                    }
                    narrate(call, 0x2109, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                }
                narrate(call, colour as u32, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                if sent_off {
                    player_left_pitch_with(m, j, 0, call);
                }
            }
            if sound != 0x1f7a {
                let c = exe::cm3_match_engine_pick_restart_taker(m, (read_i8(j + match_player::SIDE) == 0) as i8, read_i8(m + engine::RESTART_TYPE), 0);
                if c == 0 {
                    narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                } else {
                    if t == 7 {
                        narrate(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                    } else {
                        narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                    }
                    if sound == 0x1f79 {
                        // the taker far from the goal he attacks (side 1: row < 4; side 0: > 7)
                        let far = (read_i8(c + match_player::SIDE) == 1 && read_i8(c + match_player::CELL_Y) < 4) || (read_i8(c + match_player::SIDE) == 0 && read_i8(c + match_player::CELL_Y) > 7);
                        if t == 7 {
                            if far && !area_de(c, read_i8(j + match_player::SIDE) == 1) {
                                narrate(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                            }
                        } else if far {
                            narrate(call, if (2..=6).contains(&read_i8(j + match_player::CELL_X)) { 0x21e0 } else { 0x21df }, (read_u8(c + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                        }
                    }
                }
            }
            if special(code as i8) == 0 {
                if random(9) == 0 {
                    narrate(call, 0x20df, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                    if random(3) == 0 {
                        narrate(call, 0x20e0, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                    }
                    if (sent_off || sound == 0x1f7a) && random(2) == 0 {
                        narrate(call, 0x20d9, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                    }
                }
                exe::cm3_match_engine_scale_side_anger_from_player(m, j, weight);
                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [0; 5]);
            }
            return 1;
        }
    }
    let p = p4 as f64;
    let mut lim = ftol((1500.0 - p * p * 100.0) + (read_i8(m + engine::BALL_ROLL) as i32 * 100) as f64) as i16;
    if lim < 0xa0 {
        lim = 0xa0;
    }
    if !(rating < lim) && !(p < 1.0) && ((read_i8(m + engine::BALL_ROLL) as i32 + 1) as f64) < p {
        // dominou
        crate::comparison::branch("respond controlled");
        if count != 0 {
            *b(j, match_player::BALLS_CONTESTED) = (*b(j, match_player::BALLS_CONTESTED)).wrapping_add(1);
            *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1c);
        }
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(1);
        if exe::cm3_match_engine_get_player_position(m, j) == 1 && in_the_box() {
            narrate(call, 0x1fe2, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        } else if read_i8(m + engine::BALL_HEIGHT) < 4 {
            narrate(call, 0x1f5f, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        }
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, j, [0; 5]);
        *w(j, match_player::ACTION) = code;
        let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
        let (contas, sorteio) = if code == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let average = if read_i8(j + contas.0) != 0 { ((read_u8(j + contas.1) as u32) as f64 / (read_u8(j + contas.0) as u32) as f64) as f32 } else { 0.0 };
        if minute > (random(0x14) + sorteio) as u8 as i8 {
            let goals = read_i8(m + (if read_i8(j + match_player::SIDE) != 0 { 0xf5bd } else { 0xf5f3 }));
            let attempts = if code == 0x15 { (read_u8(j + match_player::CHALLENGES_WON) as u32) > 3 && read_i8(j + 9) != 0 } else { (read_u8(j + match_player::KEY_TACKLES) as u32) > 1 };
            if goals == 0 && average as f64 > 0.725 && attempts && *w(j, match_player::MATCH_RATING) > 0x1a5e && read_i8(j + match_player::TACKLE_RATE_NARRATED) == 0 {
                crate::comparison::branch("respond success average");
                narrate(call, if code == 0x15 { 0x21ce } else { 0x21cd }, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), (read_u8(j + match_player::COMMENTARY_ID) as u32) | (read_u8(j + match_player::SIDE) as u32) << 8);
                *b(j, match_player::TACKLE_RATE_NARRATED) = 1;
            }
        }
        return 1;
    }
    // perdeu: a bola espirra
    if count != 0 {
        *b(j, match_player::BALLS_CONTESTED) = (*b(j, match_player::BALLS_CONTESTED)).wrapping_add(1);
        *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(10);
    }
    let (inside, shooter) = if exe::cm3_match_engine_get_player_position(m, j) == 1 && in_the_box() {
        crate::comparison::branch("respond goalkeeper lost");
        narrate(call, 0x20d4, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        if random(10) != 0 {
            (1, 0)
        } else if random(200) != 0 {
            (0, 0)
        } else {
            (0, j)
        }
    } else {
        crate::comparison::branch("respond lost");
        narrate(call, if kind <= 2 { 0x213e } else { 0x1f71 }, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        (0, j)
    };
    espirra(call, inside, shooter);
    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(2) as u8);
    *w(j, match_player::ACTION) = code;
    1
}

/// The match player contests the ball (tackle, interception or challenge with strength `forca`): may lead to a foul and
/// its punishment, winning the ball or losing it; thin wrapper over contest_ball_injected with the real calls. Returns
/// its result.
///
/// [pt] { 0x6f5c30 } (ret 0x10, match_pl.cpp): ver `contest_ball_injected`.
pub(crate) unsafe extern "thiscall" fn respond(j: usize, strength: u32, kind: u32, p4: u32, count: u32) -> u8 {
    contest_ball_injected(j, strength as i16, kind as i8, f32::from_bits(p4), count as u8, &mut |rotina, this, a| dispatch_tackle_calls(rotina, this, a))
}

/// { `shot` } (ret 0xc, match_pl.cpp, called only by `engine_loop`) with the calls that touch things outside the match
/// engine injected (0x6d6f70 commentates, 0x716e10 tires, `take_ball` ball control, `exe::cm3_match_engine_move_ball_to_cell` the ball deflects, 0x716c90,
/// `decide_player` the next action; and those inside the injury, the save and leaving the pitch): player `j` shoots with
/// `forca` aiming at (x, y) (x = −1: the set piece `engine::BALL_X`/`engine::BALL_Y`).
///
/// The chance by distance to goal (side 0 from table 0xa15e44, side 1 via `exe::match_player_distance_to_cell`); stamina (random draw((10000 −
/// +0x4d)^2 / 50000)); the angle of the direction (+0x19a) against `engine::BALL_DIRECTION` weighted by the action (`engine::BALL_HEIGHT`); the
/// strength loses 20 × (`engine::BALL_ROLL` − 1)^3 (and more from the engine's byte 1). With an angle up to 90 the player turns
/// (`exe::match_player_set_facing_direction`) towards goal (by +0x10c × +0x105, chance below 2 or `exe::cm3_match_engine_is_path_ahead_clear`) or towards the target.
///
/// The action and the attribute: the header 0x25 (+0x164), or by `engine::BALL_HEIGHT`: 4+ outside the attacking area with strength
/// and a clear path (`exe::cm3_match_engine_is_clear_ahead_of_cell`) 0xc ((+0x10c − 20) × 10 + +0x132, `engine::BALL_HEIGHT` becomes 2), otherwise 0x2e (+0x176) or 0x15
/// (+0x144) by +0x1a2 and +0xff against +0x24; 2 and 3: 0x3b (+0x190) near goal (`exe::match_player_is_cell_in_view`, chance below 1.41 and random
/// draws), otherwise 0xc (+0x132); the others 0xd (+0x134). The rating is random draw(attribute) − random draw(stamina)
/// − random draw(angle) + random draw(strength); `exe::match_player_drain_stamina` tires.
///
/// The referee looks at the shooter's foul (`exe::cm3_match_engine_referee_check_foul`): by random draw(chance × 5 + 2) it is only recorded (`exe::cm3_match_engine_set_pending_foul`),
/// otherwise punished as in `exe::match_player_try_intercept_ball`. The challenger comes from `exe::cm3_match_engine_get_opposition_challenge` (injury 0x6cfe10 right away); he (rating >=
/// the shot's, or 1 in 5) and the shooter (shot rating higher, or 1 in 5) may commit a foul (`exe::cm3_match_engine_referee_judge_tackle`: recorded 1 in 30
/// / 1 in 10 or punished, the challenger's as in the 50-50 and with the +0x17 adjustment on a penalty).
///
/// With the challenger's rating, he keeps the ball: foul commentated in the area, goalkeeper save (0x70d430) or the
/// action's commentary, `take_ball`, +0x19c, the ball deflects below the action's limit, counts and stamina (+0x35,
/// 0x21cd/0x21ce, +0x17) and 1 in 7 0x1f41. Otherwise the shot goes on: `take_ball`, `engine::MOMENTUM_A`/`engine::MOMENTUM_B` lose rating × 2/11;
/// with a header (`engine::BALL_HEIGHT` 4) 0x2164/0x21cb/0x214f/0x1f5d, counts and `decide_player`; otherwise 0x216e (on 0xc) or the
/// long-range shot commentaries (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c gains rating − the challenger's − 200 (±200)
/// and by random draw(+0x19c) against random draw(200) `decide_player` (with +0x1a7 and +100 possession).
///
/// Outside the attacking area, random draw(+0x31) < −10 makes the player grow in confidence (0x211d).
///
/// [pt] { 0x6f6c00 } (ret 0xc, match_pl.cpp, chamada so pela 0x6b98c0) com as chamadas que mexem fora do
/// motor injetadas (0x6d6f70 narra, 0x716e10 cansa, 0x6f0630 dominio, 0x6cd770 a bola espirra,
/// 0x716c90, 0x7103e0 o lance seguinte; e as de dentro da lesao, da defesa e da saida de campo): o
/// jogador `j` chuta com `forca` mirando (x, y) (x = -1: a bola parada +0x8ea7/+0x8ea8).
/// A chance pela distancia ao gol (lado 0 pela tabela 0xa15e44, lado 1 pela 0x6f09b0); o folego
/// (sorteio((10000 - +0x4d)^2 / 50000)); o angulo da direcao (+0x19a) contra +0x8ed8 pesado pelo lance
/// (+0x8ea9); a forca perde 20 x (+0x8eaa - 1)^3 (e mais pelo byte 1 do motor). Com angulo ate 90 o
/// jogador vira (0x6f4920) para o gol (por +0x10c x +0x105, chance abaixo de 2 ou 0x6bd5b0) ou para o
/// alvo. O lance e o atributo: a cabecada 0x25 (+0x164), ou pelo +0x8ea9: 4+ fora da area de ataque
/// com forca e caminho livre (0x6c41b0) o 0xc ((+0x10c - 20) x 10 + +0x132, +0x8ea9 vira 2), senao
/// 0x2e (+0x176) ou 0x15 (+0x144) pelo +0x1a2 e +0xff contra +0x24; 2 e 3 o 0x3b (+0x190) perto do gol
/// (0x6f9f90, chance abaixo de 1,41 e sorteios), senao 0xc (+0x132); os outros 0xd (+0x134). A nota e
/// sorteio(atributo) - sorteio(folego) - sorteio(angulo) + sorteio(forca); 0x6f56c0 cansa. O arbitro
/// olha a falta do chutador (0x6cfca0): por sorteio(chance x 5 + 2) so registra (0x6bb9a0), senao pune
/// como a 0x6f5c30. O desafiante sai da 0x6c87e0 (lesao 0x6cfe10 logo); ele (nota >= a do chute ou 1
/// em 5) e o chutador (nota do chute maior ou 1 em 5) podem fazer falta (0x6cfc60: registrada 1 em 30
/// / 1 em 10 ou punida, a do desafiante como na dividida e com ajuste do +0x17 no penalti). Com a nota
/// do desafiante, ele fica com a bola: falta narrada na area, defesa do goleiro (0x70d430) ou a
/// narracao do lance, 0x6f0630, +0x19c, a bola espirra abaixo do limite do lance, contas e folego
/// (+0x35, 0x21cd/0x21ce, +0x17) e 1 em 7 0x1f41. Senao o chute segue: 0x6f0630, +0x4782/+0x4786
/// perdem a nota x 2/11; de cabeca (+0x8ea9 4) 0x2164/0x21cb/0x214f/0x1f5d, contas e 0x7103e0; senao
/// 0x216e (no 0xc) ou as narracoes do chute de longe (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c ganha a
/// nota - a do desafiante - 200 (+-200) e por sorteio(+0x19c) contra sorteio(200) 0x7103e0 (com
/// +0x1a7 e +100 de posse). Fora da area de ataque, sorteio(+0x31) < -10 faz o jogador crescer (0x211d).
pub(crate) unsafe fn shoot_injected(j: usize, strength_in: i16, x_in: i8, y_in: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let zero = |p: usize| (read_i8(p + match_player::SIDE) == 0) as u32;
    let par = |p: usize| (read_u8(p + match_player::COMMENTARY_ID) as u32) | (read_u8(p + match_player::SIDE) as u32) << 8;
    let more = |p: *mut i16, v: i16| *p = (*p).wrapping_add(v);
    let um = |p: *mut u8| *p = (*p).wrapping_add(1);
    let m = *((j + match_player::ENGINE) as *const usize);
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, a3, a4, a5]);
    };
    // in columns 2 to 6, rows 10-11 (`lado1`) or 0-1
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&read_i8(p + match_player::CELL_X)) && if lado1 { (10..=11).contains(&read_i8(p + match_player::CELL_Y)) } else { (0..=1).contains(&read_i8(p + match_player::CELL_Y)) }
    };
    // the challenger far from his goal by the table and behind the wide player, or the shooter unable to shoot
    let count_shot = |e: usize| {
        let dy = read_i8(e + match_player::CELL_Y) as i32 - if read_i8(e + match_player::SIDE) != 0 { 11 } else { 0 };
        !(distance_table_cell_yx(dy, read_i8(e + match_player::CELL_X) as i32 - 4) as f64 > 1.0 && exe::cm3_match_engine_is_behind_own_back_line(*((e + match_player::ENGINE) as *const usize), e) == 0) && can_shoot_from_here(j)
    };
    let (jx, jy) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let chance = if read_i8(j + match_player::SIDE) == 0 { distance_table_cell_yx(jy as i32 - 11, jx as i32 - 4) } else { exe::match_player_distance_to_cell(j, 4, 0) };
    let mut kind = read_i8(m + engine::BALL_HEIGHT);
    let (x, y) = if x_in == -1 { (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y)) } else { (x_in, y_in) };
    let d = 10000 - ftol(read_f32(j + match_player::STAMINA) as f64) as i16 as i32;
    let folego = random(d.wrapping_mul(d) / 50000);
    let mut ang = (*w(j, match_player::FACING) as i32 - *w(m, engine::BALL_DIRECTION) as i32).abs();
    if ang as i16 > 0xb4 {
        ang = 0x168 - ang;
    }
    let q = (((0xb4 - ang) * 2) as i16 as i32) / ((5 - kind as i32).max(1) as i16 as i32);
    let angulo = if (ang as i16) < 0x5a { (11 - ang as i16 as i32 / 10) * q } else { q * 2 };
    let c = (*b(m, engine::BALL_ROLL)).wrapping_sub(1) as i8 as i16;
    let mut strength = strength_in.wrapping_sub(c.wrapping_mul(c).wrapping_mul(c).wrapping_mul(20));
    if c != 0 {
        match read_i8(m + 1) {
            2 => strength = strength.wrapping_sub(((random(3) + 1) as i16).wrapping_mul(c)),
            0x20 => strength = strength.wrapping_sub(((random(8) + 3) as i16).wrapping_mul(c)),
            _ => {}
        }
    }
    let owner = *((m + engine::SET_PIECE_TAKER) as *const usize);
    // turns towards goal or towards the target
    if (ang as i16) <= 0x5a {
        let r = random(200);
        let to_goal = read_i8(j + match_player::MOMENT_AGILITY) as i32 * read_i8(j + match_player::MOMENT_ANTICIPATION) as i32 > r || (chance as f64) < 2.0 || exe::cm3_match_engine_is_path_ahead_clear(m, j, 0) != 0;
        if to_goal {
            if read_i8(j + match_player::SIDE) == 0 {
                exe::match_player_set_facing_direction(j, jx, jy, 4, if jy < 9 { jy + 3 } else { 11 });
            } else {
                exe::match_player_set_facing_direction(j, jx, jy, 4, if jy > 3 { jy - 3 } else { 0 });
            }
        } else if read_i8(m + engine::BALL_HEIGHT) < 4 {
            exe::match_player_set_facing_direction(j, jx, jy, x, y);
        }
    }
    // the move and the attribute
    let mut doze = false;
    let in_own = area_de(j, read_i8(j + match_player::SIDE) == 1);
    let cabeca = !(in_own && exe::cm3_match_engine_get_player_position(m, j) == 1)
        && random(in_own as i32 * 500 + 100 + read_i8(*((j + match_player::STAFF) as *const usize) + 0x5c) as i32 * 200) == 0;
    let (attr, lance): (i16, i8) = if cabeca {
        crate::comparison::branch("shot header");
        (*w(j, 0x164), 0x25)
    } else if kind >= 4 {
        if !area_de(j, read_i8(j + match_player::SIDE) == 0)
            && strength as i32 > (20 - read_i8(j + match_player::MOMENT_AGILITY) as i32) * 7
            && exe::cm3_match_engine_is_clear_ahead_of_cell(m, read_i8(j + match_player::SIDE), jx, jy) != 0
        {
            crate::comparison::branch("shot 0xc from distance");
            doze = true;
            *b(m, engine::BALL_HEIGHT) = 2;
            kind = read_i8(m + engine::BALL_HEIGHT);
            ((read_i8(j + match_player::MOMENT_AGILITY) as i16 - 20).wrapping_mul(10).wrapping_add(*w(j, 0x132)), 0xc)
        } else {
            let v = ftol(((read_f32(j + match_player::CONFIDENCE) as f64 * read_f64(rdata::F64_0_01) + 0.75) * read_i8(j + match_player::ANGER) as f64 + 5.0) * 0.5) as i8;
            let r1 = random(read_i8(j + match_player::YELLOW_CARDS) as i32 * 80 + in_own as i32 * 1500 + 0x46);
            if random(v as i32) > r1 { (*w(j, 0x176), 0x2e) } else { (*w(j, 0x144), 0x15) }
        }
    } else if kind != 3 && kind != 2 {
        (*w(j, 0x134), 0xd)
    } else {
        let ok = can_shoot_from_here(j)
            && (chance as f64) < 1.41
            && {
                let r1 = random(*w(j, 0x132) as i32);
                random(*w(j, 0x190) as i32) > r1
            }
            && random(ftol(read_f32(j + match_player::FINISHING) as f64)) >= random(5);
        if ok {
            crate::comparison::branch("shot 0x3b");
            (*w(j, 0x190), 0x3b)
        } else {
            (*w(j, 0x132), 0xc)
        }
    };
    let s1 = random(attr as i32);
    let s2 = random(folego as i16 as i32);
    let s3 = random(angulo as i16 as i32);
    let rating_shot = (s1 - s2 - s3 + random(strength as i32)) as i16;
    exe::match_player_drain_stamina(j, -1.0, lance as i16);
    // the shooter’s foul
    let (mut cartao, mut grav) = (0u8, 0u8);
    let t = exe::cm3_match_engine_referee_check_foul(m, j, lance as i16 as u16, &mut cartao, &mut grav);
    if t != 0 {
        if random(ftol(chance as f64 * 5.0 + 2.0)) == 0 {
            crate::comparison::branch("shot foul recorded");
            exe::cm3_match_engine_set_pending_foul(m, lance as i16 as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);
        } else {
            crate::comparison::branch("shot foul punished");
            punish_contester_foul_injected(j, t, cartao, grav, lance as i16, call);
            more(w(j, match_player::MATCH_RATING), -0x19);
            return;
        }
    }
    // o desafiante
    *((m + engine::BALL_HOLDER) as *mut usize) = j;
    let (mut a, mut b2, mut nota, mut ld, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = exe::cm3_match_engine_get_opposition_challenge(m, j, read_i8(j + match_player::SIDE), lance, &mut a, &mut b2, &mut nota, &mut ld, &mut arriscado, 0);
    tackle_injury_with(m, j, lance as i16, e, ld as i16, 0, call);
    *((m + engine::BALL_HOLDER) as *mut usize) = 0;
    let gr = &mut grav as *mut u8 as *mut i8;
    'contest: {
        if e == 0 {
            crate::comparison::branch("shot no challenger");
            break 'contest;
        }
        let mut advantage = 0u8;
        let mut checar_chutador = true;
        // the challenger’s foul
        if nota >= rating_shot || random(5) == 0 {
            let td = exe::cm3_match_engine_referee_judge_tackle(m, e, j, ld, -1, &mut cartao, gr, &mut advantage);
            if read_i8(m + engine::LIVE_MATCH) != 0 && area_de(e, read_i8(j + match_player::SIDE) == 0)
                && (special(ld) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
                && random(3) != 0
            {
                narrate(call, 0x2168, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
            }
            if td != 0 {
                if random(0x1e) == 0 && arriscado == 0 {
                    crate::comparison::branch("shot challenger recorded");
                    exe::cm3_match_engine_set_pending_foul(m, ld as i16 as u16, e as u32, j as u32, 0, td, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);
                    checar_chutador = false;
                } else {
                    crate::comparison::branch("shot challenger punished");
                    punish_challenger_foul_injected(m, e, j, td, cartao, grav as i8, arriscado, advantage, ld, true, &["shot penalty do desafiante", "shot expulsao do desafiante", "shot pick_restart_taker do desafiante", "shot vantagem do desafiante"], call);
                    return;
                }
            }
        }
        // the taker’s foul on the challenger
        if checar_chutador && (rating_shot > nota || random(5) == 0) {
            let mut tj = exe::cm3_match_engine_referee_judge_tackle(m, j, e, lance, -1, &mut cartao, gr, &mut advantage);
            if read_i8(m + engine::LIVE_MATCH) != 0 && area_de(j, read_i8(j + match_player::SIDE) == 1)
                && (special(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
                && random(3) != 0
            {
                narrate(call, 0x2168, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            }
            if tj != 0 {
                if random(10) == 0 {
                    crate::comparison::branch("shot shooter recorded");
                    exe::cm3_match_engine_set_pending_foul(m, lance as i16 as u16, j as u32, e as u32, 0, tj, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);
                } else {
                    crate::comparison::branch("shot shooter punished");
                    let mut weight = 0f32;
                    exe::cm3_match_engine_referee_decide_foul_punishment(m, j, e, &mut tj, cartao as i8, grav as i8, 0, &mut weight, advantage);
                    narrate(call, if lance == 0x38 { 0x1ff3 } else { foul_commentary_code(lance) }, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), par(e));
                    let colour: i32 = match tj {
                        2 => 0x1ff9,
                        3 => 0x1ff7,
                        4 => 0x1ff8,
                        5 => 0x1ff6,
                        _ => {
                            if grav as i8 as i32 > random(7) + 2 { 0x210a } else { -1 }
                        }
                    };
                    let sound: u32;
                    if tj == 7 {
                        sound = 0x1f79;
                        *b(m, engine::RESTART_TYPE) = 7;
                    } else if area_de(j, read_i8(j + match_player::SIDE) == 1) {
                        if colour == -1 {
                            match random(4) {
                                0 => narrate(call, 0x20e9, u32::MAX, zero(j), u32::MAX),
                                1 => narrate(call, 0x20ec, u32::MAX, zero(j), u32::MAX),
                                _ => {}
                            }
                        }
                        crate::comparison::branch("shot shooter penalty");
                        sound = 0x1f7a;
                        *b(m, engine::RESTART_TYPE) = 5;
                        *((m + engine::FOULED_PLAYER) as *mut usize) = e;
                        more(w(j, match_player::MATCH_RATING), -500);
                        if random(read_i8(*((j + match_player::STAFF) as *const usize) + 0x5a) as i32) == 0 {
                            let r = random(10);
                            if exe::match_player_adjust_morale(j, (r as u8).wrapping_neg() as i8) != 0 {
                                exe::match_player_calc_ratings(j, 0);
                            }
                        }
                    } else {
                        sound = 0x1f79;
                        *b(m, engine::RESTART_TYPE) = 6;
                    }
                    *b(m, engine::BALL_X) = *b(j, match_player::CELL_X);
                    *b(m, engine::BALL_Y) = *b(j, match_player::CELL_Y);
                    *b(m, engine::POSSESSION_SIDE) = zero(j) as u8;
                    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
                    if sound == 0x1f7a {
                        narrate(call, 0x1f7a, u32::MAX, zero(j), u32::MAX);
                    }
                    let sent_off = matches!(tj, 3..=6);
                    if colour > -1 {
                        if random(4) == 0 {
                            let g = random(grav as i8 as i32);
                            if g > random(0x14) {
                                let partida = *((m + engine::MATCH) as *const usize);
                                let club = Club(*((partida + if read_i8(j + match_player::SIDE) == 0 { 0x20 } else { 0x1c }) as *const usize));
                                let k = club.nation().addr();
                                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                                }
                            }
                            narrate(call, 0x2109, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                        }
                        narrate(call, colour as u32, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                        if sent_off {
                            crate::comparison::branch("shot shooter sent off");
                            player_left_pitch_with(m, j, 0, call);
                        }
                    }
                    if sound != 0x1f7a {
                        let cb = exe::cm3_match_engine_pick_restart_taker(m, zero(j) as i8, read_i8(m + engine::RESTART_TYPE), 0);
                        if cb == 0 {
                            narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                        } else {
                            if tj == 7 {
                                narrate(call, 0x21e2, (read_u8(cb + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                            } else {
                                narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                            }
                            if sound == 0x1f79 && advantage == 0 {
                                // the taker far from the goal he attacks (side 1: row < 4; side 0: > 7)
                                let far = (read_i8(cb + match_player::SIDE) == 1 && read_i8(cb + match_player::CELL_Y) < 4) || (read_i8(cb + match_player::SIDE) == 0 && read_i8(cb + match_player::CELL_Y) > 7);
                                if tj == 7 {
                                    if far && !area_de(cb, read_i8(j + match_player::SIDE) == 1) {
                                        narrate(call, 0x21e1, (read_u8(cb + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                                    }
                                } else if far {
                                    narrate(call, if (2..=6).contains(&read_i8(j + match_player::CELL_X)) { 0x21e0 } else { 0x21df }, (read_u8(cb + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                                }
                            }
                        }
                    }
                    if special(lance) == 0 {
                        let extra = sent_off || sound == 0x1f7a;
                        if advantage != 0 {
                            crate::comparison::branch("shot shooter advantage");
                            narrate(call, 0x20dd, (read_u8(e + match_player::COMMENTARY_ID) as u32), zero(j), u32::MAX);
                            if matches!(tj, 4..=6) {
                                *b(e, match_player::SENT_OFF) = 1;
                            }
                            if extra && random(3) != 0 {
                                narrate(call, 0x20d9, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                            }
                        } else if random(9) == 0 {
                            narrate(call, 0x20df, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                            if random(3) == 0 {
                                narrate(call, 0x20e0, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                            }
                            if extra && random(2) != 0 {
                                narrate(call, 0x20d9, u32::MAX, (read_u8(j + match_player::SIDE) as u32), u32::MAX);
                            }
                        }
                        exe::cm3_match_engine_scale_side_anger_from_player(m, j, weight);
                        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [0; 5]);
                    }
                    more(w(j, match_player::MATCH_RATING), -0x19);
                    return;
                }
            }
        }
        let penalty = advantage != 0 && (area_de(j, read_i8(j + match_player::SIDE) == 0) || arriscado != 0);
        *w(e, match_player::ACTION) = ld as i16;
        if kind != 4 {
            um(b(e, 4));
            more(w(e, match_player::MATCH_RATING), if exe::cm3_match_engine_get_player_position(m, e) & 0x30 != 0 && exe::cm3_match_engine_get_player_position(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
        }
        if rating_shot > nota {
            break 'contest;
        }
        // the challenger keeps the ball
        crate::comparison::branch("shot challenger wins");
        let narrated = (special(ld) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
            && area_de(e, read_i8(e + match_player::SIDE) == 1)
            && random(2) == 0;
        if narrated {
            crate::comparison::branch("shot challenger narrates foul in the box");
            narrate(call, foul_commentary_code(ld), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
            if kind < 4 {
                let code = match random(0x32) {
                    1 => 0x20ea,
                    0 | 2 | 5 | 6 | 9 => 0x20eb,
                    _ => 0x20e8,
                };
                narrate(call, code, u32::MAX, zero(e), u32::MAX);
            }
        } else {
            let code = match ld {
                1 => {
                    crate::comparison::branch("shot goalkeeper save");
                    if save_shot_injected(e, nota.wrapping_sub(rating_shot), call) == 0 {
                        return;
                    }
                    0
                }
                0x13 | 0x21 | 0x2d => 0x1f62,
                0x15 | 0x2e => {
                    let r = random(6);
                    if ((r as f64 + read_f32(e + match_player::JUMPING) as f64) + 3.0) < read_f32(j + match_player::JUMPING) as f64 { 0x2164 } else { 0x1f5d }
                }
                0x22 | 0x2c => 0x2150,
                0x29 => 0x1f61,
                0x2a | 0x2b => 0x1f64,
                _ => 0,
            };
            if code != 0 {
                narrate(call, code, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
            }
        }
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, e, [0; 5]);
        *w(e, match_player::BALL_CONTROL) = nota.wrapping_sub(rating_shot).wrapping_sub(200).clamp(-200, 200);
        let mut controlled = false;
        let limit = match ld {
            0x13 => 400,
            0x21 | 0x2d => 250,
            _ => 100,
        };
        if *w(e, match_player::BALL_CONTROL) < limit && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 {
            crate::comparison::branch("shot ball breaks loose");
            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(j + match_player::CELL_X) as u32), (read_u8(j + match_player::CELL_Y) as u32), 1 | zero(j) << 8, 0, 0]);
            if read_i8(m + engine::RESTART_TYPE) == 0 {
                let mut k = 3;
                k -= (read_i8(e + match_player::MOMENT_WORK_RATE) as i32 > random(10)) as i32;
                k -= (read_i8(e + match_player::MOMENT_AGILITY) as i32 > random(10)) as i32;
                *b(e, match_player::BUSY) = random(k) as u8;
                controlled = true;
            } else if penalty {
                crate::comparison::branch("shot 716c90");
                call(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, j, [0; 5]);
            }
        } else {
            if (0..4).contains(&read_i8(m + engine::BALL_HEIGHT)) {
                *b(m, engine::BALL_HEIGHT) = 0;
            }
            let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
            if read_i8(e + match_player::SIDE) == 0 && read_i8(m + engine::BALL_HEIGHT) != -1 {
                exe::match_player_set_facing_direction(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
            } else {
                exe::match_player_set_facing_direction(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
            }
            let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
            let (contas, sorteio) = if ld == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
            let average = if read_i8(e + contas.0) != 0 { ((read_u8(e + contas.1) as u32) as f64 / (read_u8(e + contas.0) as u32) as f64) as f32 } else { 0.0 };
            if minute > (random(0x14) + sorteio) as u8 as i8 {
                let me = *((e + match_player::ENGINE) as *const usize);
                let goals = read_i8(me + (if read_i8(e + match_player::SIDE) != 0 { 0xf5bd } else { 0xf5f3 }));
                let attempts = if ld == 0x15 { (read_u8(e + match_player::CHALLENGES_WON) as u32) > 3 && read_i8(e + 9) != 0 } else { (read_u8(e + match_player::KEY_TACKLES) as u32) > 1 };
                if goals == 0 && average as f64 > 0.725 && attempts && *w(e, match_player::MATCH_RATING) > 0x1a5e && read_i8(e + match_player::TACKLE_RATE_NARRATED) == 0 {
                    crate::comparison::branch("shot success average");
                    narrate(call, if ld == 0x15 { 0x21ce } else { 0x21cd }, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(j));
                    *b(e, match_player::TACKLE_RATE_NARRATED) = 1;
                }
            }
        }
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(a);
        let gain: i16 = if controlled { 0x78 } else { 0xc8 };
        more(w(e, match_player::MATCH_RATING), gain);
        if kind != 4 {
            um(b(e, 5));
            let me = *((e + match_player::ENGINE) as *const usize);
            let t1 = if me != 0 { exe::cm3_match_engine_get_player_position(me, e) } else { 0 };
            let far = t1 & 4 == 0 && (if me != 0 { exe::cm3_match_engine_get_player_position(me, e) } else { 0 }) & 2 == 0;
            more(w(e, match_player::MATCH_RATING), if far { gain.wrapping_mul(2) } else { gain });
            if count_shot(e) {
                more(w(e, match_player::MATCH_RATING), 300);
                um(b(e, 6));
                if random(read_i8(e + match_player::MATCH_FORM) as i32 * 30 + 0x46) == 0 {
                    let r = random(3);
                    if exe::match_player_adjust_morale(e, r as i8) != 0 {
                        exe::match_player_calc_ratings(e, 0);
                    }
                }
            }
        } else {
            crate::comparison::branch("shot challenger wins in the air");
            um(b(e, 7));
            um(b(e, 8));
            *w(e, match_player::ACTION) = 0x7b;
            more(w(e, match_player::MATCH_RATING), 0x28);
            more(w(j, match_player::MATCH_RATING), -10);
            um(b(j, 7));
            if count_shot(e) {
                more(w(e, match_player::MATCH_RATING), 300);
                um(b(e, 9));
                if random(read_i8(e + match_player::MATCH_FORM) as i32 * 30 + 0x46) == 0 {
                    let r = random(2);
                    if exe::match_player_adjust_morale(e, r as i8) != 0 {
                        exe::match_player_calc_ratings(e, 0);
                    }
                }
            }
        }
        if controlled && random(7) == 0 {
            narrate(call, 0x1f41, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        }
        return;
    }
    // o chute segue
    crate::comparison::branch("shot continues");
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, j, [0; 5]);
    more(w(j, match_player::MATCH_RATING), 4);
    *w(j, match_player::ACTION) = lance as i16;
    let v = ftol(rating_shot as f64 * read_f64(rdata::F64_NEG_0_1818181818));
    let posse = |o: usize| *((m + o) as *mut i32) = (*((m + o) as *const i32)).wrapping_sub(v);
    posse(engine::MOMENTUM_A);
    posse(engine::MOMENTUM_B);
    if e != 0 {
        *b(e, match_player::BUSY) = (*b(e, match_player::BUSY)).wrapping_add(b2);
    }
    let add_19c = || *w(j, match_player::BALL_CONTROL) = (*w(j, match_player::BALL_CONTROL)).wrapping_add(rating_shot.wrapping_sub(nota).wrapping_sub(200)).clamp(-200, 200);
    if kind == 4 {
        if e != 0 {
            crate::comparison::branch("shot header with challenger");
            let (pj, pe) = (*((j + match_player::PLAYER) as *const usize), *((e + match_player::PLAYER) as *const usize));
            let me = *((e + match_player::ENGINE) as *const usize);
            let code = 'n: {
                if pj != 0 {
                    if pe != 0 {
                        let r = random(3);
                        if r + read_i8(pj + player_sheet::JUMPING) as i32 + 3 < read_i8(pe + player_sheet::JUMPING) as i32 {
                            break 'n 0x2164;
                        }
                    }
                    if pe != 0 && !(me != 0 && exe::cm3_match_engine_get_player_position(me, e) == 1) {
                        let r = random(3);
                        if read_i8(pj + player_sheet::JUMPING) as i32 > r + read_i8(pe + player_sheet::JUMPING) as i32 + 5 {
                            break 'n 0x21cb;
                        }
                    }
                }
                if me != 0 && exe::cm3_match_engine_get_player_position(me, e) == 1 && random(5) != 0 { 0x214f } else { 0x1f5d }
            };
            narrate(call, code, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), par(e));
            *w(j, match_player::ACTION) = 0x7b;
            um(b(e, 7));
            let t1 = if me != 0 { exe::cm3_match_engine_get_player_position(me, e) } else { 0 };
            more(w(e, match_player::MATCH_RATING), if t1 != 1 { -10 } else { -600 });
            more(w(j, match_player::MATCH_RATING), 0x28);
            um(b(j, 7));
            um(b(j, 8));
            if count_shot(e) {
                more(w(j, match_player::MATCH_RATING), 300);
                um(b(j, 9));
                if random(read_i8(e + match_player::MATCH_FORM) as i32 * 30 + 0x46) == 0 {
                    let r = random(2);
                    if exe::match_player_adjust_morale(e, r as i8) != 0 {
                        exe::match_player_calc_ratings(e, 0);
                    }
                }
            }
        }
        add_19c();
        call(FN_MATCH_PLAYER_TAKE_TURN, j, [0; 5]);
    } else {
        if doze {
            narrate(call, 0x216e, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        } else {
            let (jy, jx) = (read_i8(j + match_player::CELL_Y), read_i8(j + match_player::CELL_X));
            if owner == j && kind < 2 && ((jy != 0 && jy != 11) || (4 - jx as i32).abs() <= 1) {
                crate::comparison::branch("shot narrates from distance");
                let side = read_i8(j + match_player::SIDE);
                let code = if strength < -500 {
                    0x21cc
                } else if ((side == 0 && jy < 3) || (side == 1 && jy > 8)) && random(2) != 0 {
                    0x21db
                } else if ((side == 1 && read_i8(j + match_player::CELL_Y) >= 6) || (side == 0 && read_i8(j + match_player::CELL_Y) < 6)) && random(2) != 0 {
                    0x21dc
                } else {
                    0x1f47
                };
                narrate(call, code, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
            }
            add_19c();
        }
        let r = random(*w(j, match_player::BALL_CONTROL) as i32);
        if r >= random(200) {
            if kind < 4 {
                *b(m, engine::SHOT_PENDING) = 1;
                *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_add(100);
                *((m + engine::MOMENTUM_B) as *mut i32) = (*((m + engine::MOMENTUM_B) as *const i32)).wrapping_add(100);
            }
            call(FN_MATCH_PLAYER_TAKE_TURN, j, [0; 5]);
        }
    }
    // outside the attacking area the player may push forward
    if !area_de(j, read_i8(j + match_player::SIDE) == 0) && random(*w(j, 0x31) as i32) < -10 {
        crate::comparison::branch("shot grows");
        let p = read_f32(j + match_player::CONFIDENCE) as f64 * 1.0499999523162842;
        *((j + match_player::CONFIDENCE) as *mut f32) = p as f32;
        let minimum = ((25 - read_i8(*((j + match_player::STAFF) as *const usize) + 0x5d) as i32) * 2) as f32;
        let v = if p > minimum as f64 { p } else { minimum as f64 };
        *((j + match_player::CONFIDENCE) as *mut f32) = if v > 100.0 { 100.0 } else { v as f32 };
        narrate(call, 0x211d, u32::MAX, zero(j), u32::MAX);
    }
}

/// the chance by distance to goal (table 0xa15e44 on side 0, `exe::match_player_distance_to_cell` on side 1), stamina, the direction angle (+0x19a
/// against `engine::BALL_DIRECTION`) weighted by the play `engine::BALL_HEIGHT`, power minus 20 × (`engine::BALL_ROLL` − 1)³ (and more by the engine's byte 1);
/// with an angle up to 90 the player turns to the goal or to the target (`exe::match_player_set_facing_direction`). The play and the attribute: header
/// 0x25 (+0x164); with `engine::BALL_HEIGHT` 4+ the long-range 0xc (power and clear path `exe::cm3_match_engine_is_clear_ahead_of_cell`, `engine::BALL_HEIGHT` becomes 2), 0x2e (+0x176)
/// or 0x15 (+0x144); with 2/3 the 0x3b (+0x190) near goal, else 0xc (+0x132); the others 0xd (+0x134). Score =
/// draw(attribute) − draw(stamina) − draw(angle) + draw(power). Three possible fouls: the shooter's before (`exe::cm3_match_engine_referee_check_foul`;
/// recorded by draw(chance × 5 + 2), else punished as in `exe::match_player_try_intercept_ball`), the challenger's (`exe::cm3_match_engine_get_opposition_challenge`, injury 0x6cfe10;
/// punished as in the tackle, plus the +0x17 adjustment on a penalty) and the shooter's on him (narration by the play
/// table, penalty given to the challenger). A challenger with the higher score takes the ball (foul narrated in the
/// box, keeper save 0x70d430 or the play's narration; the ball spills below the limit; counts, stamina, +0x17; 1 in 7
/// 0x1f41). Otherwise the shot goes on: `engine::MOMENTUM_A`/`engine::MOMENTUM_B` lose score × 2/11; headed 0x2164/0x21cb/0x214f/0x1f5d and
/// `decide_player`; else 0x216e or the long-range narrations (0x21cc, 0x21db, 0x21dc, 0x1f47), +0x19c gains the difference
/// (±200) and by draw `decide_player` (with +0x1a7 and +100 possession); outside the attacking box the player may grow
/// (0x211d). The tackle's punished foul became a shared function (`punish_challenger_foul_injected`); the shooter's
/// (`punish_contester_foul_injected`) repeats `exe::match_player_try_intercept_ball`'s, which keeps its inline copy (see COMO_CONTINUAR). Compared
/// through the tape
/// Original: `shot` (0x6f6c00, match_pl.cpp (12.7 KB, called only by 0x6b98c0 with power +0x8ed6 and target +0x1c8/+0x1c9), thiscall, ret 0xc).
///
/// [pt] { 0x6f6c00 } (ret 0xc, match_pl.cpp): ver `shoot_injected`.
pub(crate) unsafe extern "thiscall" fn shot(j: usize, strength: u32, x: u32, y: u32) {
    shoot_injected(j, strength as i16, x as i8, y as i8, &mut |rotina, this, a| dispatch_tackle_calls(rotina, this, a))
}

/// `0x70c040` (ret 4, match_pl.cpp) with the calls that change things outside the engine injected (0x6d6f70 commentary,
/// `exe::cm3_match_engine_move_ball_to_cell` set piece, 0x716e10 and `take_ball`; and those of the foul `apply_foul`): the outcome of player `p`'s shot with
/// strength `f` (0 counts as 1).
///
/// The quality (random(1000) − random(2000)) × (`engine::BALL_ROLL` + 1) / f against the goalkeeper's defence (`exe::cm3_match_engine_get_goalkeeper`, 0x700420,
/// with rebound and coming out) gives the type: 1 on target, 2..11 wide (only the score `exe::cm3_match_engine_get_goal_margin`), 12/13 wide with a
/// commented goal kick (0x1fca/0x1fcb, set piece 2).
///
/// On target, the shot's share of the sum with the defence: above 71 it is a goal (penalty shoot-out `side_of_move`, golden
/// goal `marked_moment`, the goalkeeper concedes; commentaries by score and time, `exe::cm3_match_engine_scale_side_anger` on the side; the score `engine::SCORE` of
/// the other side, the crowd commentaries), above 61 a save for a corner, above 51 a save that goes over the line
/// (`engine::BALL_Y`, set piece 3 and 0x1f75), otherwise a save (`take_ball`); on saves, the goalkeeper's highlight (`player_highlight`)
/// chooses the commentary.
///
/// [pt] { 0x70c040 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x6cd770 bola parada, 0x716e10 e 0x6f0630; e as da falta 0x6bac40): o desfecho do chute de
/// `p` com a forca `f` (0 vale 1). A qualidade (sorteio(1000) - sorteio(2000)) x (+0x8eaa + 1) / f
/// contra a defesa do goleiro (0x6ce110, 0x700420, com rebote e saida) da o tipo: 1 no alvo, 2..11
/// para fora (so o placar 0x6ce310), 12/13 para fora com tiro de meta narrado (0x1fca/0x1fcb, bola
/// parada 2). No alvo, a parte do chute na soma com a defesa: acima de 71 e gol (disputa de penaltis
/// 0x6bba50, gol de ouro 0x6d4250, o goleiro sofre; narracoes pelo placar e o tempo, 0x6bb950 no
/// lado; o placar +0xf5bd do outro lado, as narracoes da torcida), acima de 61 defesa com escanteio,
/// acima de 51 defesa que sai pela linha (+0x8ea8, bola parada 3 e 0x1f75), senao defesa (0x6f0630);
/// nas defesas, o destaque do goleiro (0x717470) escolhe a narracao.
pub(crate) unsafe fn shot_outcome_injected(p: usize, f_in: f32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + match_player::ENGINE) as *const usize);
    let bm = |o: usize| (m + o) as *mut u8;
    let side = read_i8(p + match_player::SIDE);
    let other = (side == 0) as u8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, x: u32, y: u8, a7: i16| shot_commentary(call, m, code, x, y, a7);
    let tabela1 = |q: usize| read_usize(q + match_player::ENGINE) != 0 && player_table(q) == 1;
    let add = |q: usize, d: i16| {
        let w = (q + match_player::MATCH_RATING) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let more_2b = |q: usize, d: u8| {
        let b = (q + match_player::BUSY) as *mut u8;
        *b = (*b).wrapping_add(d);
    };
    let f = if f_in as f64 == read_f64(rdata::F64_0) { 1.0f32 } else { f_in };
    let factor = read_i8(m + engine::BALL_ROLL);
    let r1 = random(0x7d0);
    let r2 = random(0x3e8);
    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 1) as f64 / f as f64);
    let g = hold_position_one(m, side);
    if g == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, f32)>(IDX_SHOT_OUTCOME)(p, f_in);
    }
    let mut b18 = 0u8;
    if read_i8(m + engine::BALL_HEIGHT) > 3 {
        b18 = 1;
    }
    let (mut b13, mut b16) = (0u8, 0u8);
    let defesa = if g != p {
        crate::comparison::branch("outcome goalkeeper saves");
        goalkeeper_save(g, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y), b18, -1, &mut b16, &mut b13)
    } else {
        // simplification: with the goalkeeper himself shooting, the original reads b13/b16 without a value (zeroed in the
        // original determinizado)
        0
    };
    let (mut b12, mut b14) = (0u8, 0u8);
    if b13 == 0 {
        if defesa as i16 > 0x177 {
            b12 = 1;
        } else if defesa as i16 > 0x104 {
            b14 = 1;
        }
    }
    let q = qualidade as i16;
    let kind: u8 = if q > 0xf0 {
        1
    } else if q < 10 {
        if b18 != 0 { if random(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let d = 0xf0 - q as i32;
        if d > 0x32 {
            if b18 != 0 { if random(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if d > 0xa {
            8 + (b18 != 0) as u8
        } else if d > 6 {
            0xa + (b18 != 0) as u8
        } else {
            0xc + (b18 != 0) as u8
        }
    };
    let (mut b10, mut b11, mut b15, mut b23, mut b17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    aggregate_score(m, other as i8, &mut b10, &mut b11, &mut b15, &mut b23, &mut b17, 1);
    if kind != 1 {
        match kind {
            0xc => narrate(call, 0x1fca, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, other, 0),
            0xd => narrate(call, 0x1fcb, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, other, 0),
            _ => {}
        }
        if !(0xc..=0xd).contains(&kind) {
            crate::comparison::branch("outcome wide");
            return;
        }
        crate::comparison::branch("outcome goal kick");
        if b13 != 0 && b16 == 0 && tabela1(g) {
            narrate(call, 0x20d5, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
        }
        narrate(call, 0x211b, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32, 2 | (side as u8 as u32) << 8, 1, 0]);
        if tabela1(g) {
            more_2b(g, random(3) as u8);
        }
        *bm(engine::SHOT_REBOUND) = 1;
        return;
    }
    // on target
    let k = defesa as i16 as i32;
    let part = if k + q as i32 == 0 { 0 } else { (q as i32 * 100) / (k + q as i32) };
    let part = part as i16;
    if part <= 0x47 {
        if b14 != 0 && tabela1(g) {
            crate::comparison::branch("outcome medium save");
            add(g, 0x15e);
            if random(read_i8(g + match_player::MATCH_FORM) as i32 * 20 + 0x32) == 0 {
                let r = random(4);
                if adjust_17(g, r as i8) != 0 {
                    match_performance(g, 0);
                }
            }
        } else if b12 != 0 && tabela1(g) {
            crate::comparison::branch("outcome hard save");
            add(g, 0x258);
            if random((read_i8(g + match_player::MATCH_FORM) as i32 + 1) * 20) == 0 {
                let r = random(7);
                if adjust_17(g, r as i8) != 0 {
                    match_performance(g, 0);
                }
            }
        }
    }
    if part > 0x47 {
        shot_outcome_goal(m, p, g, side, other, b10, b11, b12, b13, b14, b15, b16, b17, call);
        return;
    }
    let codes = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
    let highlight = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, abertura: u32, common: u32, quieta: u32| {
        let (g2e, g27) = (read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8);
        if b12 != 0 {
            narrate(call, abertura, g2e, g27, 0);
            if let Some(&c) = codes.iter().find(|&&c| player_highlight(g, c) != 0) {
                crate::comparison::branch("outcome goalkeeper highlight");
                narrate(call, c, g2e, g27, 0);
            }
        } else {
            narrate(call, if b14 != 0 { common } else { quieta }, g2e, g27, 0);
        }
    };
    if part > 0x3d {
        crate::comparison::branch("outcome corner");
        highlight(call, 0x2111, 0x210f, 0x1fe6);
        add(g, 0x8c);
        more_2b(g, (random(3) + 1) as u8);
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32, (side as u8 as u32) << 8, 0, 0]);
        narrate(call, 0x211b, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);
        *bm(engine::SHOT_REBOUND) = 1;
    } else if part > 0x33 {
        crate::comparison::branch("outcome byline");
        highlight(call, 0x2111, 0x210f, 0x1fe4);
        if side == 1 {
            *bm(engine::BALL_Y) = 0xff;
        } else if side == 0 {
            *bm(engine::BALL_Y) = 0xc;
        }
        *bm(engine::POSSESSION_SIDE) = other;
        *((m + engine::BALL_HOLDER) as *mut u32) = 0;
        narrate(call, 0x211b, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);
        *bm(engine::RESTART_TYPE) = 3;
        narrate(call, 0x1f75, u32::MAX, other, 0);
    } else {
        crate::comparison::branch("outcome save");
        let r = random(3);
        add(g, 0xc8);
        more_2b(g, r as u8);
        highlight(call, 0x2112, 0x2110, 0x1fe5);
        narrate(call, 0x211b, read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8, 0);
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, g, [0; 5]);
    }
}

/// Goal branch of the shot outcome (shot share above 71): golden goal, keeper stats, scoring, commentary and crowd.
///
/// [pt] O gol do desfecho (parte do chute acima de 71): ver `shot_outcome_injected`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn shot_outcome_goal(m: usize, p: usize, g: usize, side: i8, other: u8, mut b10: i8, b11: i8, b12: u8, b13: u8, b14: u8, b15: u8, b16: u8, b17: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let _ = (b12, b14);
    let bm = |o: usize| (m + o) as *mut u8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, x: u32, y: u8, a7: i16| shot_commentary(call, m, code, x, y, a7);
    let tabela1 = |q: usize| read_usize(q + match_player::ENGINE) != 0 && player_table(q) == 1;
    let add = |q: usize, d: i16| {
        let w = (q + match_player::MATCH_RATING) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let minute = || *((m + engine::CLOCK) as *const i16) as i32 / 11;
    let tarde = |d: i32| (0x55..0x5a).contains(&d) || d >= 0x73;
    let (p2e, p27) = (read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8);
    crate::comparison::branch("outcome goal");
    if side_of_move(m) as i8 == -1 {
        if marked_moment(m) as u8 != 0 {
            crate::comparison::branch("outcome golden goal");
            *bm(engine::GOLDEN_GOAL_SCORED) = 1;
        }
        if tabela1(g) {
            *((g + match_player::GOALS_CONCEDED) as *mut u8) = read_i8(g + match_player::GOALS_CONCEDED).wrapping_add(1) as u8;
            let sheet = Staff(read_usize(g + match_player::STAFF));
            if random(sheet.determination() as i32 + 1) != 0 && random(sheet.pressure() as i32 + 1) != 0 {
                let s = read_i8(g + match_player::GOALS_CONCEDED) as u8 as i32;
                let r = random(s * s) as u8;
                let pf = read_i8(p + match_player::GOALS_CONCEDED) as u8;
                let v = if ((r as i8) as i32) < pf as i32 { pf } else { r };
                *((g + match_player::FRUSTRATION) as *mut u8) = (read_i8(g + match_player::FRUSTRATION) as u8).wrapping_add(v);
                match_performance(g, 0);
            }
            add(g, -500);
            if b13 != 0 {
                add(g, -600);
            }
        }
    }
    if tabela1(g) && g != p && b13 == 0 && random(0x32) == 0 && b16 == 0 {
        narrate(call, 0x20d6, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
    }
    narrate(call, 0x1fff, p2e, other, 0);
    let penalties = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, y: u8| {
        crate::comparison::branch("outcome penalty shootout goal");
        narrate(call, 0x20ed, u32::MAX, y, 0);
        let bl = side_of_move(m);
        apply_pending_foul_with(m, call);
        let k = if bl as i8 == side {
            narrate(call, 0x20df, u32::MAX, p27, 0);
            if random((b11 as i32) - (b10 as i32) + 3) > 0 {
                narrate(call, 0x20d9, u32::MAX, p27, 0);
            }
            3.0f32
        } else {
            if (b10 == b11 || (b10 as i32) - 1 == b11 as i32) && random(0x28) == 0 {
                narrate(call, 0x20d9, u32::MAX, p27, 0);
            }
            f32::from_bits(0x3fb9999a)
        };
        apply_to_team(m, p, k);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, p, [0; 5]);
    };
    if random(3) != 0 && side_of_move(m) as i8 > -1 {
        penalties(call, other);
        return;
    }
    let nos_penaltis = side_of_move(m) as i8 > -1;
    narrate(call, if nos_penaltis { 0x20fa } else { 0x1fcd }, p2e, p27, 0);
    if side_of_move(m) as i8 > -1 {
        penalties(call, p27);
        return;
    }
    if tabela1(g) && g != p && b16 != 0 {
        narrate(call, 0x1fe1, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, b16 as i8 as i16);
    }
    b10 = b10.wrapping_add(1);
    let k_side = |k: u32| apply_to_side(m, side, f32::from_bits(k));
    if *bm(engine::GOLDEN_GOAL_SCORED) != 0 {
        narrate(call, 0x1fd2, u32::MAX, other, 0);
    } else if b17 != 0 {
        goal_crowd_reaction(call, m, side, other, b15, &narrate);
    } else {
        let dif = b10 as i32 - b11 as i32;
        let t = minute();
        let fase2 = *((m + engine::PERIOD_END) as *const i16) == 0x3de;
        if dif == 2 && ((t >= 0x46 && fase2) || t >= 0x6a) {
            narrate(call, 0x1fd0, u32::MAX, other, 0);
            k_side(0x3fc66666);
        } else if (dif == 1 || b15 != 0) && ((t >= 0x50 && fase2) || t >= 0x6e) {
            narrate(call, 0x1fd1, u32::MAX, other, 0);
            k_side(0x3fb9999a);
        } else if b10 == b11 && b15 == 0 && t >= 0x46 {
            crate::comparison::branch("outcome late equaliser");
            let a = count_active(m, other as i8) as i8;
            let b = count_active(m, side) as i8;
            let bl = if a < b { read_i8(m + (4 + other as usize)) } else { -1 };
            let d = minute() as i16 as i32;
            let (code, k) = if bl == -1 {
                if tarde(d) { (0x1fd8, 0x3fb9999a) } else { (0x1fd6, 0x3faccccd) }
            } else if tarde(d) {
                (0x1fd9, 0x3fd33333)
            } else {
                (0x1fd7, 0x3fb33333)
            };
            k_side(k);
            narrate(call, code, u32::MAX, other, bl as i16);
        } else if dif < 0 && dif > -3 && (t >= 0x50 || (t >= 0x46 && b10 == 1) || (t >= 0x3c && b10 == 1 && 1 - b11 as i32 == -2)) {
            narrate(call, if tarde(t as i16 as i32) { 0x1fdb } else { 0x1fda }, u32::MAX, other, 0);
        } else {
            goal_crowd_reaction(call, m, side, other, b15, &narrate);
        }
    }
    if tabela1(g) && g != p && b13 != 0 && b16 == 0 {
        narrate(call, 0x20d5, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
    }
    apply_to_team(m, p, f32::from_bits(0x3fd9999a));
    let goals = bm(0xf5bd + other as usize * 0x36);
    *goals = (*goals).wrapping_add(1);
    let r = random(5);
    let club = || {
        let pt = read_usize(m + engine::MATCH);
        read_usize((if side != 0 { pt + 0x1c } else { pt + 0x20 }) + 0)
    };
    let manager_8 = || {
        let t = read_usize((club()) + 0x53);
        t != 0 && read_i8(t + 0x77) == 8
    };
    let mut done = false;
    if (b10 as i32 - b11 as i32) > r + 2 && random(7) == 0 && manager_8() {
        narrate(call, 0x2117, u32::MAX, other, 0);
        done = true;
    }
    if !done && (b10 == b11 || b10 as i32 - 1 == b11 as i32 || b15 != 0) {
        let r1 = random(minute());
        if r1 > random(0x78) && manager_8() {
            narrate(call, 0x2116, u32::MAX, other, 0);
        }
    }
    *bm(engine::POSSESSION_SIDE) = side as u8;
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *bm(engine::RESTART_TYPE) = 8;
    *bm(engine::KICKOFF_ANNOUNCED) = 0;
}

/// Shared end of goal commentary: crowd event 0x1fdc and side anger x1.45 before minute 80, else just x1.25.
///
/// [pt] O fim comum das narracoes do gol: a torcida (0x1fdc antes dos 80 com +0x15, fator 1,45) ou
/// so o fator 1,25 no lado.
pub(crate) unsafe fn goal_crowd_reaction(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, side: i8, other: u8, b15: u8, narrate: &dyn Fn(&mut dyn FnMut(usize, usize, [u32; 5]) -> u32, u32, u32, u8, i16)) {
    let t = *((m + engine::CLOCK) as *const i16) as i32 / 11;
    if b15 != 0 && t < 0x50 {
        narrate(call, 0x1fdc, u32::MAX, other, 0);
        apply_to_side(m, side, f32::from_bits(0x3fb9999a));
    } else {
        apply_to_side(m, side, f32::from_bits(0x3fa00000));
    }
}

/// Sends a shot commentary event, packing the 16-bit a7 argument into the high half of the code.
///
/// [pt] A narracao dos chutes: o ultimo argumento (a7) vai em 16 bits no alto do codigo (contagens de gols
/// em palavra; a fita confere o byte baixo).
pub(crate) unsafe fn shot_commentary(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, code: u32, x: u32, y: u8, a7: i16) {
    call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code | (a7 as u16 as u32) << 16, m as u32, x, y as u32, u32::MAX]);
}

/// Rust-mode dispatcher for the shot's injected calls (commentary unpacks a7 as 16 bits), else as the tackle.
pub(crate) unsafe fn dispatch_shot_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    if routine == 0x6d6f70 {
        add_match_event(this, [a[0] & 0xffff, a[1], a[2], a[3], a[4] & 0xff, (a[4] >> 8) & 0xff, (a[0] >> 16) as u16 as i16 as i32 as u32]);
        return 0;
    }
    dispatch_tackle_calls(routine, this, a)
}

/// the same structure against the keeper of the shooter's own side (the goal goes to the other side, commentary and
/// side factor mirrored). Determinized original. Compared through the tape
/// Original: `shot_outcome` (0x70c040, match_pl.cpp (5.1 KB), thiscall, ret 4).
///
/// [pt] { 0x70c040 }: ver `shot_outcome_injected`.
pub(crate) unsafe extern "thiscall" fn shot_outcome(p: usize, f: f32) {
    shot_outcome_injected(p, f, &mut |r, t, a| dispatch_shot_calls(r, t, a))
}

/// Comparer for `shot_outcome`: runs original vs Rust shot outcome over a call tape with masks, registers the result.
pub(crate) unsafe fn compare_shot_outcome_inner(p: usize, f: f32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, f32)>(IDX_SHOT_OUTCOME);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || {
        zero_stack_below();
        o(p, f)
    }, |chamar| {
        shot_outcome_injected(p, f, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
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
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia, || format!("shot_outcome {} chamadas: {}", k, tape_detail()));
}

/// `0x70a910` (ret 4, match_pl.cpp) with the calls that change things outside the engine injected (as in `shot_outcome`):
/// player `p`'s shot on goal with strength `f` (0 counts as 1), against the other side's goalkeeper (`exe::cm3_match_engine_get_goalkeeper`,
/// 0x700420), with shot stats, goal counters, hat-trick and career marks.
///
/// Same type as in `shot_outcome`; outside the penalty shoot-out it counts the shot (+0xa) and the shot on target (+0xb);
/// wide, it comments 0x1fc6..0x1fcb by type (8..13) and, with 12/13, the goal kick (0x211c).
///
/// Goal: +0xc, performance +0x89 and the five goal counts (+0x3d..+0x45, words); the score commentaries, the hat-trick
/// (0x2011/0x1fd5) and the career marks (0x200a..0x2012 and the highlight 0x21b6, with the count as the last argument);
/// the side's score `engine::SCORE`.
///
/// Saves as in `shot_outcome` (0x211c instead of 0x211b).
///
/// [pt] { 0x70a910 } (ret 4, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (como na
/// 0x70c040): o chute a gol de `p` com a forca `f` (0 vale 1), contra o goleiro do outro lado
/// (0x6ce110, 0x700420). Mesmo tipo da 0x70c040; fora da disputa de penaltis conta o chute (+0xa) e o
/// chute no alvo (+0xb); para fora narra 0x1fc6..0x1fcb pelo tipo (8..13) e, com 12/13, o tiro de
/// meta (0x211c). Gol: +0xc, atuacao +0x89 e as cinco contagens de gols (+0x3d..+0x45, palavras); as
/// narracoes do placar, o hat-trick (0x2011/0x1fd5) e as marcas da carreira (0x200a..0x2012 e o
/// destaque 0x21b6, com a contagem como ultimo argumento); o placar +0xf5bd do lado. As defesas como
/// na 0x70c040 (0x211c no lugar de 0x211b).
pub(crate) unsafe fn shot_on_goal_injected(p: usize, f_in: f32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + match_player::ENGINE) as *const usize);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let bm = |o: usize| (m + o) as *mut u8;
    let side = read_i8(p + match_player::SIDE);
    let other = (side == 0) as u8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, x: u32, y: u8, a7: i16| shot_commentary(call, m, code, x, y, a7);
    let tabela1 = |q: usize| u(q, match_player::ENGINE) != 0 && player_table(q) == 1;
    let add = |q: usize, d: i16| {
        let w = (q + match_player::MATCH_RATING) as *mut i16;
        *w = (*w).wrapping_add(d);
    };
    let more = |q: usize, o: usize| {
        let b = (q + o) as *mut u8;
        *b = (*b).wrapping_add(1);
    };
    let more_2b = |q: usize, d: u8| {
        let b = (q + match_player::BUSY) as *mut u8;
        *b = (*b).wrapping_add(d);
    };
    let (p2e, p27) = (read_i8(p + match_player::COMMENTARY_ID) as u8 as u32, side as u8);
    let f = if f_in as f64 == read_f64(rdata::F64_0) { 1.0f32 } else { f_in };
    let factor = read_i8(m + engine::BALL_ROLL);
    let r1 = random(0x7d0);
    let r2 = random(0x3e8);
    let qualidade = ftol(r2.wrapping_sub(r1).wrapping_mul(factor as i32 + 1) as f64 / f as f64);
    let g = hold_position_one(m, other as i8);
    if g == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, f32)>(IDX_SHOT_ON_GOAL)(p, f_in);
    }
    let b18 = (read_i8(m + engine::BALL_HEIGHT) > 3) as u8;
    let (mut b13, mut b15) = (0u8, 0u8);
    let defesa = goalkeeper_save(g, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y), b18, -1, &mut b15, &mut b13);
    let (mut b12, mut b14) = (0u8, 0u8);
    if b13 == 0 {
        if defesa as i16 > 0x177 {
            b12 = 1;
        } else if defesa as i16 > 0x104 {
            b14 = 1;
        }
    }
    let q = qualidade as i16;
    let kind: u8 = if q > 0xf0 {
        1
    } else if q < 10 {
        if b18 != 0 { if random(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let d = 0xf0 - q as i32;
        if d > 0x32 {
            if b18 != 0 { if random(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if d > 0xa {
            8 + (b18 != 0) as u8
        } else if d > 6 {
            0xa + (b18 != 0) as u8
        } else {
            0xc + (b18 != 0) as u8
        }
    };
    if side_of_move(m) as i8 == -1 {
        more(p, 0xa);
        if kind == 1 {
            more(p, 0xb);
        }
    }
    let (mut b10, mut b11, mut b16, mut b23, mut b17) = (0i8, 0i8, 0u8, 0u8, 0u8);
    aggregate_score(m, side, &mut b10, &mut b11, &mut b16, &mut b23, &mut b17, 1);
    if kind != 1 {
        if (8..=0xd).contains(&kind) {
            narrate(call, 0x1fc6 + (kind as u32 - 8), p2e, p27, 0);
        }
        if !(0xc..=0xd).contains(&kind) {
            crate::comparison::branch("shot on target wide");
            return;
        }
        crate::comparison::branch("shot on target goal kick");
        if b13 != 0 && b15 == 0 && tabela1(g) {
            narrate(call, 0x20d5, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
        }
        narrate(call, 0x211c, p2e, p27, 0);
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32, 2 | (p27 as u32) << 8, 1, 0]);
        if tabela1(g) {
            more_2b(g, random(3) as u8);
        }
        *bm(engine::SHOT_REBOUND) = 1;
        return;
    }
    let k = defesa as i16 as i32;
    let part = if k + q as i32 == 0 { 0 } else { (q as i32 * 100) / (k + q as i32) } as i16;
    if part <= 0x47 {
        if b14 != 0 && tabela1(g) {
            crate::comparison::branch("shot on target medium save");
            add(g, 0x15e);
            if random(read_i8(g + match_player::MATCH_FORM) as i32 * 20 + 0x32) == 0 {
                let r = random(4);
                if adjust_17(g, r as i8) != 0 {
                    match_performance(g, 0);
                }
            }
        } else if b12 != 0 && tabela1(g) {
            crate::comparison::branch("shot on target hard save");
            add(g, 0x258);
            if random((read_i8(g + match_player::MATCH_FORM) as i32 + 1) * 20) == 0 {
                let r = random(7);
                if adjust_17(g, r as i8) != 0 {
                    match_performance(g, 0);
                }
            }
        }
        let codes = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
        let highlight = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, abertura: u32, common: u32, quieta: u32| {
            let (g2e, g27) = (read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8);
            if b12 != 0 {
                narrate(call, abertura, g2e, g27, 0);
                if let Some(&c) = codes.iter().find(|&&c| player_highlight(g, c) != 0) {
                    crate::comparison::branch("shot on target goalkeeper highlight");
                    narrate(call, c, g2e, g27, 0);
                }
            } else {
                narrate(call, if b14 != 0 { common } else { quieta }, g2e, g27, 0);
            }
        };
        if part > 0x3d {
            crate::comparison::branch("shot on target corner");
            highlight(call, 0x2111, 0x210f, 0x1fe6);
            narrate(call, 0x211c, p2e, p27, 0);
            add(g, 0x8c);
            more_2b(g, (random(3) + 1) as u8);
            call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [read_i8(m + engine::BALL_X) as u8 as u32, read_i8(m + engine::BALL_Y) as u8 as u32, (other as u32) << 8, 0, 0]);
            *bm(engine::SHOT_REBOUND) = 1;
        } else if part > 0x33 {
            crate::comparison::branch("shot on target byline");
            highlight(call, 0x2111, 0x210f, 0x1fe4);
            if side == 1 {
                *bm(engine::BALL_Y) = 0xff;
            } else if side == 0 {
                *bm(engine::BALL_Y) = 0xc;
            }
            *bm(engine::POSSESSION_SIDE) = p27;
            *((m + engine::BALL_HOLDER) as *mut u32) = 0;
            narrate(call, 0x211c, p2e, p27, 0);
            *bm(engine::RESTART_TYPE) = 3;
            narrate(call, 0x1f75, u32::MAX, p27, 0);
        } else {
            crate::comparison::branch("shot on target save");
            let r = random(3);
            add(g, 0xc8);
            more_2b(g, r as u8);
            highlight(call, 0x2112, 0x2110, 0x1fe5);
            narrate(call, 0x211c, p2e, p27, 0);
            call(FN_MATCH_PLAYER_TAKE_POSSESSION, g, [0; 5]);
        }
        return;
    }
    // o gol
    crate::comparison::branch("shot on target goal");
    let w = |o: usize| (p + o) as *mut i16;
    if side_of_move(m) as i8 == -1 {
        if marked_moment(m) as u8 != 0 {
            crate::comparison::branch("shot on target golden goal");
            *bm(engine::GOLDEN_GOAL_SCORED) = 1;
        }
        more(p, 0xc);
        add(p, 0x89);
        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
            *w(o) = (*w(o)).wrapping_add(1);
        }
        if tabela1(g) {
            more(g, 0xf);
            let sheet = Staff(u(g, match_player::STAFF));
            if random(sheet.determination() as i32 + 1) != 0 && random(sheet.pressure() as i32 + 1) != 0 {
                let s = read_i8(g + match_player::GOALS_CONCEDED) as u8 as i32;
                let r = random(s * s) as u8;
                let pf = read_i8(p + match_player::GOALS_CONCEDED) as u8;
                let v = if ((r as i8) as i32) < pf as i32 { pf } else { r };
                *((g + match_player::FRUSTRATION) as *mut u8) = (read_i8(g + match_player::FRUSTRATION) as u8).wrapping_add(v);
                match_performance(g, 0);
            }
            add(g, -500);
            if b13 != 0 {
                add(g, -600);
            }
        }
    }
    if b13 == 0 && random(0x32) == 0 && b15 == 0 && tabela1(g) {
        narrate(call, 0x20d6, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
    }
    narrate(call, 0x1fff, p2e, p27, 0);
    // the penalty shootout: the foul (0x6bac40) and the taker’s commentary
    let penalties = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        crate::comparison::branch("shot on target penalty shootout");
        narrate(call, 0x20ed, u32::MAX, p27, 0);
        let bl = side_of_move(m);
        apply_pending_foul_with(m, call);
        let k = if bl as i8 == side {
            narrate(call, 0x20df, u32::MAX, p27, 0);
            if random((b11 as i32) - (b10 as i32) + 3) > 0 {
                narrate(call, 0x20d9, u32::MAX, p27, 0);
            }
            3.0f32
        } else {
            if (b10 == b11 || (b10 as i32) - 1 == b11 as i32) && random(0x28) == 0 {
                narrate(call, 0x20d9, u32::MAX, p27, 0);
            }
            f32::from_bits(0x3fb9999a)
        };
        apply_to_team(m, p, k);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, p, [0; 5]);
    };
    if random(3) != 0 && side_of_move(m) as i8 > -1 {
        penalties(call);
        return;
    }
    narrate(call, if side_of_move(m) as i8 > -1 { 0x2115 } else { 0x2114 }, p2e, p27, 0);
    if random(3) != 0 && side_of_move(m) as i8 > -1 {
        penalties(call);
        return;
    }
    if read_i8(p + match_player::GOALS) == 3 {
        crate::comparison::branch("shot on target hat-trick");
        narrate(call, if read_i8(p + 0x30) != 0 { 0x2011 } else { 0x1fd5 }, p2e, p27, 0);
    }
    if side_of_move(m) as i8 > -1 {
        penalties(call);
        return;
    }
    // the marks: the last argument and the count
    let c = |o: usize| *w(o);
    let redondo = |v: i16, d: i16| v >= d && v % d == 0;
    let mark: Option<(u32, i16)> = if read_i8(p + 0x30) != 0 && read_i8(p + match_player::GOALS) == 1 {
        Some((if player_highlight(p, 0x21b6) != 0 { 0x21b6 } else { 0x2010 }, c(0x3d)))
    } else if c(0x3d) == 1 {
        Some((0x200f, 1))
    } else if c(0x3f) == 1 {
        Some((0x200b, 1))
    } else if c(0x43) == 1 {
        Some((0x2012, 1))
    } else if redondo(c(0x3f), 10) {
        Some((0x200b, c(0x3f)))
    } else if c(0x3d) == 0x32 || redondo(c(0x3d), 100) {
        Some((0x200a, c(0x3d)))
    } else if c(0x41) != c(0x3d) && (c(0x41) == 0x32 || redondo(c(0x41), 100)) {
        Some((0x200c, c(0x41)))
    } else if c(0x43) != c(0x41) && redondo(c(0x43), 100) {
        Some((0x200d, c(0x43)))
    } else if c(0x45) == 1 || redondo(c(0x45), 5) {
        Some((0x200e, c(0x45)))
    } else {
        None
    };
    if let Some((code, n)) = mark {
        crate::comparison::branch("shot on target mark");
        narrate(call, code, p2e, p27, n);
    }
    if b15 != 0 && tabela1(g) {
        narrate(call, 0x1fe1, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, b15 as i8 as i16);
    }
    b10 = b10.wrapping_add(1);
    let minute = || *((m + engine::CLOCK) as *const i16) as i32 / 11;
    let tarde = |d: i32| (0x55..0x5a).contains(&d) || d >= 0x73;
    let k_other = |k: u32| apply_to_side(m, other as i8, f32::from_bits(k));
    let goal_crowd_reaction = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if b16 != 0 && minute() < 0x50 {
            narrate(call, 0x1fdc, u32::MAX, p27, 0);
            k_other(0x3fb9999a);
        } else {
            k_other(0x3fa00000);
        }
    };
    if *bm(engine::GOLDEN_GOAL_SCORED) != 0 {
        narrate(call, 0x1fd2, u32::MAX, p27, 0);
    } else if b17 != 0 {
        goal_crowd_reaction(call);
    } else {
        let dif = b10 as i32 - b11 as i32;
        let t = minute();
        let fase2 = *((m + engine::PERIOD_END) as *const i16) == 0x3de;
        if dif == 2 && ((t >= 0x46 && fase2) || t >= 0x6a) {
            narrate(call, 0x1fd0, u32::MAX, p27, 0);
            k_other(0x3fc66666);
        } else if (dif == 1 || b16 != 0) && ((t >= 0x50 && fase2) || t >= 0x6e) {
            narrate(call, 0x1fd1, u32::MAX, p27, 0);
            k_other(0x3fb9999a);
        } else if b10 == b11 && b16 == 0 && t >= 0x46 {
            crate::comparison::branch("shot on target late equaliser");
            let a = count_active(m, other as i8) as i8;
            let b = count_active(m, side) as i8;
            let bl = if b < a { read_i8(m + ((4 + side as isize) as usize)) } else { -1 };
            let d = minute() as i16 as i32;
            let (code, k) = if bl == -1 {
                if tarde(d) { (0x1fd8, 0x3fb9999a) } else { (0x1fd6, 0x3faccccd) }
            } else if tarde(d) {
                (0x1fd9, 0x3fd33333)
            } else {
                (0x1fd7, 0x3fb33333)
            };
            k_other(k);
            narrate(call, code, u32::MAX, p27, bl as i16);
        } else if dif < 0 && dif > -3 && (t >= 0x50 || (t >= 0x46 && b10 == 1) || (t >= 0x3c && b10 == 1 && 1 - b11 as i32 == -2)) {
            narrate(call, if tarde(t as i16 as i32) { 0x1fdb } else { 0x1fda }, u32::MAX, p27, 0);
        } else {
            goal_crowd_reaction(call);
        }
    }
    if b13 != 0 && b15 == 0 && tabela1(g) {
        narrate(call, 0x20d5, read_i8(g + match_player::COMMENTARY_ID) as u8 as u32, read_i8(g + match_player::SIDE) as u8, 0);
    }
    apply_to_team(m, p, f32::from_bits(0x3dcccccd));
    let goals = bm((0xf5bd + side as isize * 0x36) as usize);
    *goals = (*goals).wrapping_add(1);
    let r = random(5);
    let manager_8 = || {
        let pt = u(m, engine::MATCH);
        let t = u(u(pt, if side == 0 { 0x1c } else { 0x20 }), 0x53);
        t != 0 && read_i8(t + 0x77) == 8
    };
    let mut done = false;
    if (b10 as i32 - b11 as i32) > r + 2 && random(7) == 0 && manager_8() {
        narrate(call, 0x2117, u32::MAX, p27, 0);
        done = true;
    }
    if !done && (b10 == b11 || b10 as i32 - 1 == b11 as i32 || b16 != 0) {
        let r1 = random(minute());
        if r1 > random(0x78) && manager_8() {
            narrate(call, 0x2116, u32::MAX, p27, 0);
        }
    }
    let _ = b23;
    *bm(engine::POSSESSION_SIDE) = other;
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *bm(engine::RESTART_TYPE) = 8;
    *bm(engine::KICKOFF_ANNOUNCED) = 0;
}

/// `p`'s shot against the other side's keeper (`exe::cm3_match_engine_get_goalkeeper`, save 0x700420). Quality from strength and draws gives the
/// type: wide (0x1fc6..0x1fcb, goal kick), on target; on target, the shot's share against the save decides goal (player
/// counts, hat-trick, career milestones 0x200a..0x2012, score and time commentary, penalty shoot-out
/// `side_of_move`/`apply_foul`), corner, byline or save (keeper highlight `player_highlight`). Compared through the tape (commentary, set
/// piece, 0x716e10, `take_ball` and the foul's calls as units)
/// Original: `shot_on_goal` (0x70a910, match_pl.cpp (5.9 KB), thiscall, ret 4).
///
/// [pt] { 0x70a910 }: ver `shot_on_goal_injected`.
pub(crate) unsafe extern "thiscall" fn shot_on_goal(p: usize, f: f32) {
    shot_on_goal_injected(p, f, &mut |r, t, a| dispatch_shot_calls(r, t, a))
}

/// Unidades da fita do movimento: a 0x68d0d0 recebe o evento 0x82 montado na pilha (confere o conteudo).
pub(crate) static RECORD_MOVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True while the player-move tape flag is set and a tape is recording.
pub(crate) fn is_recording_move() -> bool {
    RECORD_MOVE.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// Is (x, y) in the penalty area the side attacks (cols 2..6; rows 10-11 for side 0, 0-1 for side 1)?
///
/// [pt] A area atacada pelo lado (colunas 2 a 6; linhas 10-11 para o lado 0, 0-1 para o lado 1).
pub(crate) fn is_in_attacked_box(side: i8, x: i8, y: i8) -> bool {
    (2..=6).contains(&x) && if side == 0 { (10..=11).contains(&y) } else { (0..=1).contains(&y) }
}

/// Players marked by p that can still be chosen (`can_choose`) on his cell move away (`move_off_ball`) unless +0x107 x +0x104
/// beats random(900)+50.
///
/// [pt] Os jogadores marcados por `p` (+0x101 deles em +0x1ae) que ainda podem ser escolhidos (0x6bce20)
/// na casa dele saem do lugar (0x714d40) se +0x107 x +0x104 nao passar de sorteio(900) + 50.
pub(crate) unsafe fn release_marked_players(m: usize, p: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mut i = 0i8;
    while i < read_i8(p + match_player::MARKER_COUNT) {
        let q = *((p + match_player::MARKERS + i as usize * 4) as *const usize);
        if can_choose(m, q, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) as u8 != 0 {
            let a = read_i8(p + match_player::MOMENT_MOVEMENT) as i32;
            let b = read_i8(p + match_player::MOMENT_PACE) as i32;
            if a * b <= random(900) + 0x32 {
                call(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE, q, [0; 5]);
            }
        }
        i = i.wrapping_add(1);
    }
}

/// One-cell step from `de` towards `para` (or stay).
///
/// [pt] O passo de uma casa em direcao ao alvo.
pub(crate) fn step_towards(de: i8, para: i8) -> i8 {
    if de < para {
        de + 1
    } else if de > para {
        de - 1
    } else {
        de
    }
}

/// Player `p` moves one cell (two, if fast) towards (x, y) with action `codigo` (0xf, 0x11, 0x12...): `0x6f09f0` (ret
/// 0x14, `match_pl.cpp`) with the calls that touch things outside the engine injected.
///
/// Injected: `0x6d6f70` narrates, `0x714d40` releases the marked players, `0x6f0630` control, `0x716e10`/`0x716c90`,
/// `0x6cd770` set piece, `0x6cd490` ball out, `0x68d0d0` the event; and those of the injury, the foul and leaving the
/// pitch.
///
/// Already at the target (other than 0xf and 0x100) it only releases the marked players. Without the ball: injury in
/// the challenge (`0x6cfe10`) and, if not facing the target (`0x6fa140`), it only turns (`0x6f4920`, `+0x198` = 0x6f).
///
/// With the ball: the attack from stamina, `+0x19c` and the action, the narration of the approach (0x2156..0x2162), the
/// challenger (`0x6c87e0`) with the foul (`0x6cfc60`/`0x6cfcd0`, as in the challenge, with event 0x82) or the clean
/// contest; if the challenger wins, his move (`0x70d430` in 1) and the end as in the challenge; getting past him,
/// possession, `+0x19c`, the move counts and the action's narrations (0x1f48/0x1f49; 0x2009, 0x20d1..0x20d3, 0x1f4a..
/// in 0x11 and 0x12); losing control (`+0x1d` or random draw), 0x1f42.
///
/// Then it moves on the grid (`+0x215e`, counts and slots per side), the furthest forward (`+0xf596`/`+0xf59a`),
/// releases the marked players, wears (`0x6f56c0`) by distance, by role and by the position marks (`0x5b5270`), and the
/// ball: with it, the ball follows (`+0x8ea7`/`+0x8ea8`, outside `0x6cd490`); without it, if the ball is loose he
/// controls it (0x1f46/0x2113/0x1f60). Finally he turns towards a third of the way to the ball.
///
/// [pt] { 0x6f09f0 } (ret 0x14, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x714d40 os marcados saem, 0x6f0630 dominio, 0x716e10/0x716c90, 0x6cd770 bola parada,
/// 0x6cd490 bola fora, 0x68d0d0 o evento; e as da lesao, da falta e da saida de campo): o jogador `p`
/// anda uma casa (duas, se for rapido) para (x, y) com a acao `codigo` (0xf, 0x11, 0x12...). Ja no
/// alvo (fora de 0xf e 0x100) so solta os marcados. Sem a bola: lesao na dividida (0x6cfe10) e, sem
/// estar virado para o alvo (0x6fa140), so vira (0x6f4920, +0x198 = 0x6f). Com a bola: o ataque pelo
/// folego, +0x19c e a acao, a narracao da aproximacao (0x2156..0x2162), o desafiante (0x6c87e0) com a
/// falta (0x6cfc60/0x6cfcd0, como na dividida, com o evento 0x82) ou a disputa limpa; vencendo o
/// desafiante, o lance dele (0x70d430 no 1) e o fim como na dividida; passando, a posse, +0x19c,
/// as contas do lance e as narracoes da acao (0x1f48/0x1f49; 0x2009, 0x20d1..0x20d3, 0x1f4a.. no 0x11
/// e 0x12); perdendo o controle (+0x1d ou sorteio), 0x1f42. Depois anda na grade (+0x215e, contas e
/// vagas por lado), o mais adiantado (+0xf596/+0xf59a), solta os marcados, desgasta (0x6f56c0) pela
/// distancia, pelo papel e pelas marcas da posicao (0x5b5270), e a bola: com ela acompanha
/// (+0x8ea7/+0x8ea8, fora 0x6cd490); sem ela, na bola livre a domina (0x1f46/0x2113/0x1f60). Por
/// fim vira para um terco do caminho ate a bola.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn player_move_injected(p: usize, x: i8, y: i8, code: i16, carries_in: u8, mode: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + match_player::ENGINE) as *const usize);
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let side = read_i8(p + match_player::SIDE);
    let other = (side == 0) as u8;
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let tabela1 = |q: usize| read_usize(q + match_player::ENGINE) != 0 && player_table(q) == 1;
    let add = |q: usize, d: i16| *w(q, match_player::MATCH_RATING) = (*w(q, match_player::MATCH_RATING)).wrapping_add(d);
    let fixar_19c = |q: usize, v: i16| *w(q, match_player::BALL_CONTROL) = v.clamp(-200, 200);
    let mut carries = carries_in;
    let (ox, oy) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
    *b(p, match_player::BUSY) = (*b(p, match_player::BUSY)).wrapping_add(1);
    let mut perdeu = false;
    if x == ox && y == oy && code != 0xf && code != 0x100 {
        release_marked_players(m, p, call);
        return;
    }
    let with_ball = || p == read_usize(m + engine::BALL_HOLDER);
    if !with_ball() {
        tackle_injury_with(m, p, code, 0, -1, 1, call);
    }
    if !with_ball() && facing(p, x, y) == 0 && read_i8(p + match_player::MOMENT_AGILITY) == 0 && read_i8(p + match_player::MOMENT_POSITIONING) == 0 && read_i8(p + match_player::MOMENT_ANTICIPATION) == 0 {
        crate::comparison::branch("movement only turns");
        aim_set_facing_direction(p, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y), x, y);
        release_marked_players(m, p, call);
        *w(p, match_player::ACTION) = 0x6f;
        return;
    }
    let mut nx = step_towards(ox, x);
    let mut ny = step_towards(oy, y);
    if !with_ball() {
        let c = read_i8(p + match_player::MOMENT_PACE) as i32;
        if c * c > random(1000) {
            nx = step_towards(nx, x);
            ny = step_towards(ny, y);
        }
    }
    'movimento: {
        if !with_ball() {
            break 'movimento;
        }
        crate::comparison::branch("movement with the ball");
        // o ataque
        let d = 10000 - ftol(read_f32(p + match_player::STAMINA) as f64) as i16 as i32;
        let folego = random(d.wrapping_mul(d) / 50000) as i16;
        if carries != 0 {
            let v = ftol(read_f32(p + match_player::FINISHING) as f64 + read_f64(rdata::F64_20)) as i16;
            fixar_19c(p, (*w(p, match_player::BALL_CONTROL)).wrapping_add(v));
        }
        let base = *w(p, match code {
            0xf => 0x138,
            0x12 => 0x13e,
            _ => 0x13c,
        }) as i32;
        let mut at = random(base - folego as i32 + *w(p, match_player::BALL_CONTROL) as i32);
        match read_i8(m + 1) {
            1 | 2 => at += -1 - random(3),
            8 => at += -2 - random(5),
            0x20 => at += -3 - random(8),
            _ => {}
        }
        let c = read_i8(m + engine::BALL_HEIGHT) as i32;
        at -= random(c * c * c * 50);
        at += random(*w(p, 0x29) as i32 / 10 - 0x1e);
        let (mut a, mut b2, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
        let e = opposition_challenge(m, p, side, code as i8, &mut a, &mut b2, &mut nota, &mut lance, &mut arriscado, 0);
        if e != 0 {
            tackle_injury_with(m, p, code, e, lance as i16, 0, call);
        }
        // the commentary of the approach
        let action = *w(p, match_player::ACTION);
        if !(0xf..=0x12).contains(&action) && !tabela1(p) && random(0x14) == 0 {
            let value = || {
                let t = if side == 0 {
                    let i = ((read_i8(p + match_player::CELL_Y) as i32 - 0xb).abs() as i8 as i32) * 9 + (read_i8(p + match_player::CELL_X) as i32 - 4).abs() as i8 as i32;
                    read_f32(G_CELL_DISTANCE_TABLE + i as usize * 4)
                } else {
                    distance_to_point(p, 4, 0)
                };
                ftol(t as f64 * read_f32(rdata::F32_9) as f64 + read_f64(rdata::F64_2_5)) as i8
            };
            let v = value();
            let n = if (v as i32 / 5) * 5 < 1 { 1i8 } else { (((value() as i32 / 5) as u8).wrapping_mul(5)) as i8 };
            let (px, py) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
            let corner_own = (side == 0 && px < 2) || (side == 1 && px > 6);
            let corner_other = (side == 0 && px > 6) || (side == 1 && px < 2);
            let bottom = (side == 0 && py > 7) || (side == 1 && py < 4);
            let middle = (side == 0 && (4..=7).contains(&py)) || (side == 1 && (4..=7).contains(&py));
            let code_n: Option<u32> = if bottom {
                if corner_own {
                    Some(0x2160)
                } else if corner_other {
                    Some(0x2162)
                } else if is_in_attacked_box(side, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if !middle && (side == 0 || side == 1) {
                // behind (side 0: < 4; side 1: > 7)
                if corner_own {
                    Some(0x215a)
                } else if corner_other {
                    Some(0x215c)
                } else {
                    Some(0x215b)
                }
            } else if (side == 0 && px < 2) || (side == 1 && px > 6) {
                Some(0x215d)
            } else if corner_other {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && random(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = code_n {
                crate::comparison::branch("movement narrated approach");
                narrate(call, cn, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX, n);
            }
        }
        let at16 = at as i16;
        let mut penalty = false;
        let mut advantage = 0u8;
        if e != 0 {
            if !tabela1(e) {
                narrate(call, 0x1f4d, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), par(e), 0);
            }
            let mut kind = 0u8;
            if !(nota < at16 && random(5) != 0) {
                let (mut cartao, mut grav) = (0u8, 0i8);
                kind = foul_in_engine(m, e, p, lance, -1, &mut cartao, &mut grav, &mut advantage);
                if read_i8(m + engine::LIVE_MATCH) != 0
                    && is_in_attacked_box(side, read_i8(e + match_player::CELL_X), read_i8(e + match_player::CELL_Y))
                    && (special_code(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
                    && random(3) != 0
                {
                    narrate(call, 0x2168, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX, 0);
                }
                if kind != 0 {
                    if random(0x1e) != 0 || arriscado != 0 {
                        crate::comparison::branch("movement foul punished");
                        punish_move_foul_injected(m, e, p, kind, cartao, grav, arriscado, advantage, lance, call);
                        return;
                    }
                    crate::comparison::branch("movement foul only recorded");
                    record_move(m, lance as i16 as u16, e as u32, p as u32, 0, kind, *b(p, match_player::CELL_X), *b(p, match_player::CELL_Y), cartao, grav as u8);
                }
            }
            if kind == 0 && advantage != 0 {
                penalty = is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) || arriscado != 0;
            }
            // a disputa limpa
            *w(e, match_player::ACTION) = lance as i16;
            *b(e, 4) = (*b(e, 4)).wrapping_add(1);
            let t = player_table(e);
            add(e, if t & 0x30 != 0 && player_table(e) & 0x880 != 0 { -0x3b } else { -0x46 });
        }
        if read_i8(p + match_player::INJURY_PENDING) != 0 {
            perdeu = true;
        } else if mode != 4 && at16 < 1 {
            let r = random(ftol(read_f32(p + match_player::TECHNIQUE) as f64 * read_f32(rdata::F32_5) as f64 + read_f32(rdata::F32_75) as f64));
            if r == 0 && (e == 0 || nota < at16) {
                perdeu = true;
            }
        }
        if perdeu {
            crate::comparison::branch("movement loses the ball");
            *((m + engine::MOMENTUM_B) as *mut u32) = 0;
            *b(m, engine::POSSESSION_SIDE) = 0xff;
            *((m + engine::BALL_HOLDER) as *mut u32) = 0;
            *b(m, engine::SHOT_PENDING) = 0;
            *w(p, 0x29) = 0;
            *w(p, match_player::BALL_CONTROL) = 0;
            let r = random(2);
            add(p, -0x96);
            *b(p, match_player::BUSY) = (r as u8).wrapping_add(1);
            narrate(call, 0x1f42, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX, 0);
            if random((read_i8(read_usize(p + match_player::STAFF) + 0x5a) as i32 + 1) * 0x14) == 0 {
                let r = random(3);
                if adjust_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                    match_performance(p, 0);
                }
            }
            break 'movimento;
        }
        if e != 0 && at16 <= nota {
            crate::comparison::branch("movement challenger wins");
            challenger_wins_injected(m, p, e, lance, nota, at16, advantage, penalty, a, carries, call);
            return;
        }
        // `p` passa
        crate::comparison::branch("movement passes");
        if !is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) && random(*w(p, 0x31) as i32) < -10 {
            crate::comparison::branch("movement grows");
            let v = read_f32(p + match_player::CONFIDENCE) as f64 * read_f32(rdata::F32_1_049999952) as f64;
            *((p + match_player::CONFIDENCE) as *mut f32) = v as f32;
            let minimum = ((25 - read_i8(read_usize(p + match_player::STAFF) + 0x5d) as i32) * 2) as f32;
            let v = if v > minimum as f64 { v } else { minimum as f64 };
            *((p + match_player::CONFIDENCE) as *mut f32) = v as f32;
            if v > read_f64(rdata::F64_100) {
                *((p + match_player::CONFIDENCE) as *mut f32) = 100.0;
            }
            narrate(call, 0x211d, u32::MAX, other as u32, u32::MAX, 0);
        }
        *b(m, engine::LAST_POSSESSION_SIDE) = side as u8;
        if side != -1 {
            let q = (m as isize + side as isize + 0x8d46) as *mut u8;
            *q = (*q).wrapping_add(1);
            let o2 = b(m, 0x8d46 + other as usize);
            if (*o2 as i8) > 0 {
                *o2 = (*o2).wrapping_sub(10);
            }
        }
        let q = (m as isize + side as isize + 0x8d46) as *mut u8;
        *q = (*q).wrapping_add(5);
        let limite_nota = (nota as i32 + 0x32).max(0xfa);
        let v = (at16 as i32 - limite_nota) as f64 + (read_f32(p + match_player::TECHNIQUE) as f64 * read_f32(p + match_player::DRIBBLING) as f64);
        let delta = if (read_f32(rdata::F32_0) as f64) < v { 0 } else { ftol(v) as i16 };
        fixar_19c(p, (*w(p, match_player::BALL_CONTROL)).wrapping_add(delta));
        let home = (m as isize + other as isize + 0x21b6 + ((read_i8(p + match_player::CELL_Y) as isize) + read_i8(p + match_player::CELL_X) as isize * 0xc) * 0x5a) as *const u8;
        if *home == 0 {
            fixar_19c(p, (*w(p, match_player::BALL_CONTROL)).wrapping_add(0x32));
        }
        let sum = |k: u32| {
            *((m + engine::MOMENTUM_A) as *mut u32) = (*((m + engine::MOMENTUM_A) as *const u32)).wrapping_add(k);
            *((m + engine::MOMENTUM_B) as *mut u32) = (*((m + engine::MOMENTUM_B) as *const u32)).wrapping_add(k);
        };
        if e == 0 {
            add(p, 9);
            sum(0x24);
        } else if code == 0xf {
            *b(e, match_player::BUSY) = (*b(e, match_player::BUSY)).wrapping_add(b2);
            add(p, 0x4b);
            *b(p, match_player::DRIBBLES) = (*b(p, match_player::DRIBBLES)).wrapping_add(1);
            sum(300);
        } else {
            let k = if (b2 as i8) > 2 { b2 } else { 2 };
            *b(e, match_player::BUSY) = (*b(e, match_player::BUSY)).wrapping_add(k);
            if tabela1(e) {
                *b(m, 0x1a4) = 1;
                sum(200);
            } else {
                sum(400);
            }
            add(p, 100);
            *b(p, match_player::DRIBBLES) = (*b(p, match_player::DRIBBLES)).wrapping_add(1);
            if random((read_i8(p + match_player::MATCH_FORM) as i32 + 1) * 0x14) == 0 {
                let r = random(5);
                if adjust_17(p, r as i8) != 0 {
                    match_performance(p, 0);
                }
            }
        }
        *b(m, engine::BALL_HEIGHT) = 0;
        move_action_commentary_injected(m, p, e, code, nx, ny, carries, mode, call);
        let _ = &mut carries;
    }
    grid_step_injected(m, p, ox, oy, nx, ny, perdeu, call);
}

/// Commentary for the action of a player passing with the ball (codes 0xf, 0x11, 0x12) by area, flank, marker and
/// keeper.
///
/// [pt] As narracoes da acao de quem passa com a bola (0xf: 0x1f48/0x1f49; 0x11 e 0x12: pela area, pelo
/// meio, pelas pontas, pelo marcado e pelo goleiro).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn move_action_commentary_injected(m: usize, p: usize, e: usize, code: i16, x: i8, y: i8, carries_in: u8, mode: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let side = read_i8(p + match_player::SIDE);
    let other = (side == 0) as u8;
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, a5]);
    };
    let tabela1 = |q: usize| read_usize(q + match_player::ENGINE) != 0 && player_table(q) == 1;
    let (px, py) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
    let mut carries = carries_in;
    let defesa_400 = || *((m + engine::TACTICS_FLAGS + other as usize * 0x18e3) as *const u32) & 0x400 != 0;
    // the commentary with the challenger (0x2173 by +0x71, 0x1f4a; with the goalkeeper 0x1f55/0x2158)
    let com_desafiante = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        let cod = if !tabela1(e) {
            let v = read_f32(p + match_player::ACCELERATION);
            let strong = v > read_f32(rdata::F32_15) || (v as f64 - read_f32(e + match_player::ACCELERATION) as f64) > read_f32(rdata::F32_10) as f64;
            if strong && random(ftol(read_f32(p + match_player::DRIBBLING) as f64)) < random(0x1e) { 0x2173 } else { 0x1f4a }
        } else if is_in_attacked_box(side, px, py) {
            0x1f55
        } else {
            0x2158
        };
        narrate(call, cod, par(e));
    };
    // the tree without a challenger, starting from the target
    let sem_desafiante = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if is_in_attacked_box(side, x, y) && !is_in_attacked_box(side, px, py) {
            narrate(call, 0x1f50, u32::MAX);
            return;
        }
        if off_centre(x) != 0 && ((side == 0 && y == 0xb && py < 0xb) || (side == 1 && y == 0 && py > 0)) {
            narrate(call, 0x2156, u32::MAX);
            return;
        }
        if (side == 0 && y == 6 && py < 6) || (side == 1 && y == 5 && py > 5) {
            narrate(call, 0x1f51, u32::MAX);
            return;
        }
        if (side == 0 && py < 6 && py > y) || (side == 1 && py >= 6 && y > py) {
            narrate(call, 0x1f54, u32::MAX);
            return;
        }
        if ((side == 1 && py < 3) || (side == 0 && py > 8)) && (px < 2 || px > 6) && off_centre(x) == 0 {
            narrate(call, 0x1f4e, u32::MAX);
            return;
        }
        if off_centre(x) != 0 && ((side == 0 && x < px && x < 2) || (side == 1 && px < x && x > 6)) {
            narrate(call, 0x1f52, u32::MAX);
            return;
        }
        if off_centre(x) != 0 && ((side == 0 && px < x && x > 6) || (side == 1 && x < px && x < 2)) {
            narrate(call, 0x1f53, u32::MAX);
            return;
        }
        let q = read_usize(p + match_player::MARKERS);
        if q != 0 && read_i8(q + match_player::CELL_X) == px && read_i8(q + match_player::CELL_Y) == py {
            let c = read_i8(q + match_player::MOMENT_POSITIONING) as i32;
            if random(3) < c && read_i8(q + match_player::BUSY) < 2 {
                crate::comparison::branch("movement narrates the marker");
                narrate(call, 0x2172, (read_u8(q + match_player::COMMENTARY_ID) as u32) | (other as u32) << 8);
                return;
            }
        }
        let action = *((p + match_player::ACTION) as *const i16);
        if action == 0x12 || action == 0x11 {
            narrate(call, 0x1f4c, u32::MAX);
        } else if is_in_attacked_box(side, px, py) {
            narrate(call, 0x20d1, u32::MAX);
        } else if limit(other, py) != 0 {
            narrate(call, 0x20d3, u32::MAX);
        } else {
            narrate(call, 0x20d2, u32::MAX);
        }
    };
    match code {
        0xf => {
            if e == 0 {
                narrate(call, 0x1f48, u32::MAX);
            } else {
                narrate(call, 0x1f49, par(e));
            }
        }
        0x11 => {
            if is_in_attacked_box(side, px, py) || (side == 1 && py > 5) || (side == 0 && py < 6) {
                carries = 0;
            }
            if mode == 4 {
                crate::comparison::branch("movement 0x11 mode 4");
                narrate(call, if *((p + match_player::ACTION) as *const i16) != 0xf { 0x2009 } else { 0x20d2 }, u32::MAX);
            } else if e == 0 {
                if carries == 0 || !(smallest_distance(m, other as i8, px, py, 1) as f64 > read_f64(rdata::F64_1_5)) {
                    sem_desafiante(call);
                } else {
                    narrate(call, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            } else {
                com_desafiante(call);
                if carries != 0 && smallest_distance(m, other as i8, px, py, 1) as f64 > read_f64(rdata::F64_1_5) {
                    narrate(call, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            }
        }
        0x12 => {
            if is_in_attacked_box(side, px, py) || mode == 4 {
                carries = 0;
            }
            if e != 0 {
                com_desafiante(call);
                if carries != 0
                    && ((side == 1 && py < 3) || (side == 0 && py > 8))
                    && smallest_distance(m, other as i8, px, py, 1) as f64 > read_f64(rdata::F64_1_5)
                {
                    if defesa_400() && random(2) == 0 {
                        narrate(call, 0x20de, u32::MAX);
                    } else {
                        narrate(call, 0x1f45, u32::MAX);
                    }
                }
            } else {
                sem_desafiante(call);
            }
        }
        _ => {
            // the original shows the error box (”unknown action”) and clears 0xb67a34
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
        }
    }
}

/// The challenger e wins the contest in a move (like the tackle, with other commentary and p's fatigue).
///
/// [pt] O desafiante `e` vence a disputa (como na dividida, com outras narracoes e o desgaste de `p`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn challenger_wins_injected(m: usize, p: usize, e: usize, lance: i8, rating: i16, at: i16, advantage: u8, penalty: bool, a: u8, carries: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let zero = |q: usize| (read_i8(q + match_player::SIDE) == 0) as u32;
    let side = read_i8(p + match_player::SIDE);
    let narrated = (special_code(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
        && is_in_attacked_box((read_i8(e + match_player::SIDE) == 0) as i8, read_i8(e + match_player::CELL_X), read_i8(e + match_player::CELL_Y))
        && random(2) == 0;
    if narrated {
        crate::comparison::branch("movement narrates foul in the box");
        narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
        let code = match random(0x32) {
            1 => 0x20ea,
            0 | 2 | 4 | 6 | 8 | 10 => 0x20eb,
            _ => 0x20e8,
        };
        narrate(call, code, u32::MAX, zero(e), u32::MAX);
    } else {
        let code = match lance {
            1 => {
                crate::comparison::branch("movement save");
                if save_shot_injected(e, rating.wrapping_sub(at), &mut |rotina, this, x| {
                    if rotina == 0x6d6f70 {
                        call(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                    } else {
                        call(rotina, this, x)
                    }
                }) == 0
                {
                    return;
                }
                0
            }
            0x13 | 0x21 | 0x2d => 0x1f62,
            0x22 | 0x2c => 0x2150,
            0x29 => 0x1f61,
            0x2a | 0x2b => 0x1f64,
            _ => 0,
        };
        if code != 0 {
            narrate(call, code, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
        }
    }
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, e, [0; 5]);
    // `p` loses: more in his own area
    if is_in_attacked_box((side == 0) as i8, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) {
        crate::comparison::branch("movement loses in the box");
        *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_sub(0xfa);
        if random(read_i8(read_usize(p + match_player::STAFF) + 0x5a) as i32 * 0x1e + 0x14) == 0 {
            let r = random(3);
            if adjust_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                match_performance(p, 0);
            }
        }
    } else if (side == 0 && read_i8(p + match_player::CELL_Y) < 3) || (side == 1 && read_i8(p + match_player::CELL_Y) > 8) {
        *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_sub(100);
    } else {
        *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_sub(0x14);
    }
    *w(e, match_player::BALL_CONTROL) = rating.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200);
    let mut controlled = false;
    let limit = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, match_player::BALL_CONTROL) < limit && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 {
        crate::comparison::branch("movement ball breaks loose");
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(p + match_player::CELL_X) as u32), (read_u8(p + match_player::CELL_Y) as u32), 1 | zero(p) << 8, 0, 0]);
        if read_i8(m + engine::RESTART_TYPE) == 0 {
            let mut k = 3;
            k -= (read_i8(e + match_player::MOMENT_WORK_RATE) as i32 > random(10)) as i32;
            k -= (read_i8(e + match_player::MOMENT_AGILITY) as i32 > random(10)) as i32;
            *b(e, match_player::BUSY) = random(k) as u8;
            controlled = true;
        } else if penalty {
            call(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, p, [0; 5]);
        }
    } else {
        if (0..4).contains(&read_i8(m + engine::BALL_HEIGHT)) {
            *b(m, engine::BALL_HEIGHT) = 0;
        }
        let (x, y) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
        if read_i8(e + match_player::SIDE) == 0 && read_i8(m + engine::BALL_HEIGHT) != -1 {
            aim_set_facing_direction(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            aim_set_facing_direction(e, x, y, 4, if y < 4 { 0 } else { y - 3 });
        }
        let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let average = if read_i8(e + contas.0) != 0 { ((read_u8(e + contas.1) as u32) as f64 / (read_u8(e + contas.0) as u32) as f64) as f32 } else { 0.0 };
        if minute > (random(0x14) + sorteio) as u8 as i8 {
            let goals = read_i8(m + (if read_i8(e + match_player::SIDE) != 0 { 0xf5bd } else { 0xf5f3 }));
            let attempts = if lance == 0x15 { (read_u8(e + match_player::CHALLENGES_WON) as u32) > 3 && read_i8(e + 9) != 0 } else { (read_u8(e + match_player::KEY_TACKLES) as u32) > 1 };
            if goals == 0 && average as f64 > read_f64(rdata::F64_0_725) && attempts && *w(e, match_player::MATCH_RATING) > 0x1a5e && read_i8(e + match_player::TACKLE_RATE_NARRATED) == 0 {
                crate::comparison::branch("movement success average");
                narrate(call, if lance == 0x15 { 0x21ce } else { 0x21cd }, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
                *b(e, match_player::TACKLE_RATE_NARRATED) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let mm = read_usize(e + match_player::ENGINE);
    let t = |_: ()| if mm != 0 { player_table(e) } else { 0 };
    let far = t(()) & 4 == 0 && t(()) & 2 == 0;
    let mut gain: i16 = if controlled { 0x78 } else { 0xc8 };
    if far {
        gain <<= 1;
    }
    let py = read_i8(p + match_player::CELL_Y);
    let in_row = match side {
        0 | 1 => py < 3 || py > 8,
        _ => false,
    };
    if !in_row {
        gain = ftol(gain as f64 * read_f64(rdata::F64_0_85)) as i16;
    }
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(gain);
    let target = if read_i8(e + match_player::SIDE) != 0 { 0xb } else { 0 };
    let i = ((read_i8(e + match_player::CELL_Y) as i32 - target).abs() as i8 as i32) * 9 + (read_i8(e + match_player::CELL_X) as i32 - 4).abs() as i8 as i32;
    let near = !(read_f32(G_CELL_DISTANCE_TABLE + i as usize * 4) as f64 > read_f64(rdata::F64_1));
    if carries != 0 || near || behind_the_extreme(mm, e) != 0 {
        crate::comparison::branch("movement challenger bonus");
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(300);
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
        if random(read_i8(e + match_player::MATCH_FORM) as i32 * 0x1e + 0x32) == 0 {
            let r = random(5);
            if adjust_17(e, r as i8) != 0 {
                match_performance(e, 0);
            }
        }
    }
    *b(p, match_player::BUSY) = (*b(p, match_player::BUSY)).wrapping_add(a);
    if controlled && random(7) == 0 {
        narrate(call, 0x1f41, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
    }
}

/// Punishes e's foul on p during a move (like punish_challenger_foul_injected, with event 0x82 and the set-piece mark
/// at the end).
///
/// [pt] A falta de `e` sobre `p` punida no movimento (como `punish_challenger_foul_injected`, com o evento 0x82 e a
/// marca da bola parada no fim).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn punish_move_foul_injected(m: usize, e: usize, p: usize, mut kind: u8, card: u8, grav: i8, risky: i8, advantage: u8, lance: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let zero = |q: usize| (read_i8(q + match_player::SIDE) == 0) as u32;
    let side = read_i8(p + match_player::SIDE);
    let mut weight = 0f32;
    engine_punish_move(m, e, p, &mut kind, card as i8, grav, risky as u8, &mut weight, advantage);
    narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
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
        crate::comparison::branch("movement penalty");
        if colour as i16 == -1 {
            match random(4) {
                0 => narrate(call, 0x20e9, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX),
                1 => narrate(call, 0x20ec, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX),
                _ => {}
            }
        }
        sound = 0x1f7a;
        *b(m, engine::RESTART_TYPE) = 5;
        *((m + engine::FOULED_PLAYER) as *mut usize) = p;
        *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(500);
        if random(read_i8(read_usize(e + match_player::STAFF) + 0x5a) as i32 + 6) == 0 {
            let r = random(8);
            if adjust_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                match_performance(e, 0);
            }
        }
        narrate(call, 0x1f7a, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
    } else {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 6;
    }
    let sent_off = matches!(kind, 3..=6);
    if colour as i16 > -1 {
        // event 0x82 (card 4/5 for reasons 5, 10, 11 and the player’s history)
        let v = ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS + 0), read_usize(e + match_player::PLAYER), 0xb, 2, 6.5)) as i8;
        let base = *(G_STAFF_COUNT as *const i32);
        let list = if m != 0 { read_usize(m + engine::EVENT_LIST) } else { 0 };
        if list != 0
            && (kind == 5 || kind == 4)
            && (card == 0xa || card == 5 || card == 0xb)
            && read_usize(e + match_player::STAFF) != 0
            && *(read_usize(e + match_player::STAFF) as *const i32) < base
            && (read_usize(p + match_player::STAFF) == 0 || *(read_usize(p + match_player::STAFF) as *const i32) < base)
            && read_usize(e + match_player::CLUB) != 0
            && exe::human_manager_is_human_club(read_usize(e + match_player::CLUB), 0, 1, 0, 0) == 0
            && (read_i8(read_usize(e + match_player::STAFF) + 0x5b) as i32) < v as i32 + 10
        {
            let person = read_usize(e + match_player::PLAYER);
            let k = 0xfu8.wrapping_sub(v as u8);
            if read_i8(person + player_sheet::AGGRESSION) > (random(5) as u8).wrapping_add(k) as i8
                && read_i8(person + player_sheet::DIRTINESS) > (random(5) as u8).wrapping_add(k) as i8
                && *((person + player_sheet::CURRENT_REPUTATION) as *const i16) > (random(0xdac) + 3000) as i16
                && v as i32 > random(3) + 1
            {
                crate::comparison::branch("movement event 0x82");
                let mut ev = [0u8; 0x19];
                ev[0..4].copy_from_slice(&*(read_usize(e + match_player::STAFF) as *const [u8; 4]));
                ev[4..8].copy_from_slice(&*(read_usize(e + match_player::CLUB) as *const [u8; 4]));
                ev[0xd] = side as u8;
                ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                ev[0x13..0x15].copy_from_slice(&0x82u16.to_le_bytes());
                ev[0x15..0x19].copy_from_slice(&*(read_usize(p + match_player::STAFF) as *const [u8; 4]));
                request_buffers(&[(ev.as_ptr() as usize, 0x19)], &[]);
                call(FN_LLIST_ADD_TAIL, list, [0, 0x19, 0, 0, 0]);
            }
        }
        if (lance == 0x2c || lance == 0x2f) && advantage != 0 {
            narrate(call, if lance == 0x2f { 0x21da } else { 0x21d9 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), par(e));
            if sent_off || sound == 0x1f7a {
                *b(e, match_player::SENT_OFF) = 1;
            }
        }
        if random(4) == 0 {
            let x = random(grav as i32);
            if x > random(0x14) {
                let t = read_usize(m + engine::MATCH);
                let club = Club(read_usize(t + (if read_i8(e + match_player::SIDE) == 0 { 0x20 } else { 0x1c })));
                let k = club.nation().addr();
                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrate(call, 0x2109, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        }
        narrate(call, colour as u32, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        if sent_off {
            crate::comparison::branch("movement sending off");
            player_left_pitch_with(m, e, 0, call);
        }
    }
    if sound != 0x1f7a {
        let c = choose_taker(m, side, read_i8(m + engine::RESTART_TYPE), 0);
        if c == 0 {
            narrate(call, sound, 0xff, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
        } else {
            if kind == 7 {
                narrate(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            } else {
                narrate(call, sound, 0xff, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            }
            let far = (side == 1 && read_i8(p + match_player::CELL_Y) < 4) || (side == 0 && read_i8(p + match_player::CELL_Y) > 7);
            if sound == 0x1f79 && advantage == 0 && far {
                if kind == 7 {
                    if !is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) {
                        narrate(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                    }
                } else if (2..=6).contains(&read_i8(p + match_player::CELL_X)) {
                    narrate(call, 0x21e0, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                } else {
                    narrate(call, 0x21df, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                }
            }
        }
    }
    if special_code(lance) == 0 {
        let extra = sent_off || sound == 0x1f7a;
        if advantage != 0 {
            narrate(call, 0x20dd, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            if kind == 4 || kind == 5 {
                *b(e, match_player::SENT_OFF) = 1;
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        } else if random(9) == 0 {
            narrate(call, 0x20df, u32::MAX, zero(p), u32::MAX);
            if random(3) == 0 {
                narrate(call, 0x20e0, u32::MAX, zero(p), u32::MAX);
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        }
        apply_to_team(m, e, weight);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e, [0; 5]);
    }
    *b(m, engine::BALL_X) = *b(p, match_player::CELL_X);
    *b(m, engine::BALL_Y) = *b(p, match_player::CELL_Y);
    *b(m, engine::POSSESSION_SIDE) = *b(p, match_player::SIDE);
    *((m + engine::BALL_HOLDER) as *mut u32) = 0;
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x19);
}

/// Grid step and aftermath of a move: cell slots (`engine::GRID`), furthest-forward player, marked players, fatigue, ball and
/// facing.
///
/// [pt] O passo na grade e o que vem depois: vagas da casa (+0x215e, 0x5a bytes por casa, 11 vagas por
/// lado e a conta em +0x58), o mais adiantado, os marcados, o desgaste, a bola e a direcao.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn grid_step_injected(m: usize, p: usize, ox: i8, oy: i8, nx: i8, ny: i8, perdeu: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, u32::MAX]);
    };
    let side = read_i8(p + match_player::SIDE);
    let home = |x: i8, y: i8| (m as isize + 0x215e + (y as isize + x as isize * 0xc) * 0x5a) as usize;
    let slot = |c: usize, k: i8| (c as isize + (side as isize * 0xb + k as isize) * 4) as *mut usize;
    let count = |c: usize| (c as isize + side as isize + 0x58) as *mut i8;
    if read_i8(p + match_player::CELL_X) != nx || read_i8(p + match_player::CELL_Y) != ny {
        let new = home(nx, ny);
        if read_i8(p + match_player::CELL_SLOT) < 0 {
            *b(p, match_player::CELL_SLOT) = *count(new) as u8;
            *count(new) = (*count(new)).wrapping_add(1);
            *slot(new, read_i8(p + match_player::CELL_SLOT)) = p;
        } else {
            let old = home(read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
            *slot(old, read_i8(p + match_player::CELL_SLOT)) = *slot(old, (*count(old)).wrapping_sub(1));
            *count(old) = (*count(old)).wrapping_sub(1);
            let q = *slot(old, read_i8(p + match_player::CELL_SLOT));
            if q != 0 {
                *b(q, match_player::CELL_SLOT) = *b(p, match_player::CELL_SLOT);
            }
            *b(p, match_player::CELL_SLOT) = *count(new) as u8;
            *count(new) = (*count(new)).wrapping_add(1);
            *slot(new, read_i8(p + match_player::CELL_SLOT)) = p;
        }
    }
    *b(p, match_player::CELL_X) = nx as u8;
    *b(p, match_player::CELL_Y) = ny as u8;
    if side == 0 {
        let q = read_usize(m + engine::FURTHEST_FORWARD);
        let qy = if q == 0 { -1 } else { read_i8(q + match_player::CELL_Y) };
        if ny > qy {
            *((m + engine::FURTHEST_FORWARD) as *mut usize) = p;
        }
    } else if side == 1 {
        let q = read_usize(m + engine::FURTHEST_FORWARD_SIDE1);
        let qy = if q == 0 { 9 } else { read_i8(q + match_player::CELL_Y) };
        if ny < qy {
            *((m + engine::FURTHEST_FORWARD_SIDE1) as *mut usize) = p;
        }
    }
    release_marked_players(m, p, call);
    if read_i8(p + match_player::POSITION) >= 0 {
        let mut k: f32 = match read_i8(m + 1) {
            1 => 1.4,
            2 => 1.2,
            4 | 0x10 => 1.05,
            8 => 1.45,
            0x20 => 1.15,
            0x40 => 1.1,
            _ => 1.0,
        };
        match read_i8(m + engine::WEATHER) {
            3 => k = (k as f64 * read_f32(rdata::F32_1_049999952) as f64) as f32,
            4 => k = (k as f64 * read_f32(rdata::F32_1_100000024) as f64) as f32,
            _ => {}
        }
        let dy = (read_i8(p + match_player::CELL_Y) as i32 - oy as i32).abs() as i8 as i32;
        let dx = (read_i8(p + match_player::CELL_X) as i32 - ox as i32).abs() as i8 as i32;
        let t = read_f32(G_CELL_DISTANCE_TABLE + (dy * 9 + dx) as usize * 4) as f64;
        let d5 = read_f32(p + match_player::STAMINA_ATTRIBUTE) as f64;
        let f = (read_f64(rdata::F64_5) - (d5 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_15)) * read_f64(rdata::F64_0_1666666667)) + read_f64(rdata::F64_2);
        k = (t * f * k as f64) as f32;
        let role_b = if m != 0 { table_b(m, p) } else { 0 };
        let role = if m != 0 { player_table(p) } else { 0 };
        if role_b != role {
            let class = |t: u16| -> i8 {
                if t & 1 != 0 {
                    0
                } else if t & 4 != 0 {
                    1
                } else if t & 8 != 0 {
                    2
                } else if t & 0x10 != 0 {
                    3
                } else if t & 0x20 != 0 {
                    4
                } else if t & 0x40 != 0 {
                    5
                } else {
                    3
                }
            };
            let a = class(if m != 0 { player_table(p) } else { 0 });
            let c = class(if m != 0 { table_b(m, p) } else { 0 });
            let kk = if (a as i32 - c as i32).abs() as u8 as i8 > 1 {
                0x968928
            } else if (if m != 0 { player_table(p) } else { 0 }) & 4 == 0 && (if m != 0 { table_b(m, p) } else { 0 }) & 4 == 0 {
                0x968978
            } else {
                0x968980
            };
            k = ((read_f64(kk) - d5 * read_f64(rdata::F64_0_005)) * k as f64) as f32;
        }
        let tactic = m + engine::TACTICS + side as usize * 0x18e3;
        let marks = position_marks(tactic, read_i8(p + match_player::POSITION));
        if marks & 0x8000 != 0 || (position_marks(tactic, read_i8(p + match_player::POSITION)) & 0x2000 != 0 && *((m + side as usize * 0x18e3 + 0x9766) as *const u32) & 0x100 != 0) {
            k = ((read_f64(rdata::F64_1_55) - d5 * read_f64(rdata::F64_0_025)) * k as f64) as f32;
        }
        tire_fatigue(p, k, -1);
    }
    if p == read_usize(m + engine::BALL_HOLDER) || perdeu {
        *b(m, engine::BALL_X) = *b(p, match_player::CELL_X);
        *b(m, engine::BALL_Y) = *b(p, match_player::CELL_Y);
        let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
        if !(bx >= 0 && by >= 0 && by < 0xc && bx < 9) {
            crate::comparison::branch("movement ball out");
            call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [side as u8 as u32, 0, 0, 0, 0]);
        }
    } else if read_i8(m + engine::BALL_HEIGHT) < 5
        && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0
        && read_i8(m + engine::POSSESSION_SIDE) == -1
        && read_i8(m + engine::BALL_X) == read_i8(p + match_player::CELL_X)
        && read_i8(m + engine::BALL_Y) == read_i8(p + match_player::CELL_Y)
        && p != read_usize(m + engine::SET_PIECE_TAKER)
        && p != read_usize(m + engine::PASSER)
    {
        let q = read_usize(m + engine::SET_PIECE_TAKER);
        if q != 0 {
            if read_i8(q + match_player::SIDE) != side {
                crate::comparison::branch("movement steals the loose ball");
                narrate(call, 0x1f60, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
                call(FN_MATCH_PLAYER_TAKE_POSSESSION, p, [0; 5]);
                *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(0x1c);
            }
        } else {
            crate::comparison::branch("movement controls the loose ball");
            aim_set_facing_direction(p, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y), 4, if side != 0 { 0 } else { 0xb });
            let (px, py) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
            let goal = read_i8(m + engine::SHOT_REBOUND) != 0 && (3..=5).contains(&px) && ((side == 0 && py >= 10) || (side == 1 && py <= 1));
            narrate(call, if goal { 0x2113 } else { 0x1f46 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32);
            *b(m, engine::BALL_HEIGHT) = 0;
            call(FN_MATCH_PLAYER_TAKE_POSSESSION, p, [0; 5]);
            *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(4);
        }
    }
    // turns a third of the way toward the ball
    if p == read_usize(m + engine::BALL_HOLDER) {
        return;
    }
    let ball_y = read_i8(m + engine::BALL_Y) as i32;
    let q = read_usize(m + engine::SET_PIECE_TAKER);
    let owner = read_i8(m + engine::POSSESSION_SIDE);
    let near: bool = if side == 0 {
        if owner == 1 {
            true
        } else if p == q {
            false
        } else if owner != 0 && (q == 0 || read_i8(q + match_player::SIDE) != 0) {
            read_i8(p + match_player::CELL_Y) < 6
        } else {
            false
        }
    } else if owner == 0 {
        false
    } else {
        !(p != q && owner != side && (q == 0 || read_i8(q + match_player::SIDE) != side) && side == 1 && read_i8(p + match_player::CELL_Y) >= 6)
    };
    let dy = if near { ball_y + ny as i32 } else { ball_y + ny as i32 + 0xb };
    let dx = read_i8(m + engine::BALL_X) as i32 + 4 + nx as i32;
    aim_set_facing_direction(p, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y), (dx / 3) as i8, (dy / 3) as i8);
}

/// Rust-mode dispatcher for move calls: marked player off-ball move (`move_off_ball`), event 0x82 (0x68d0d0), else as the
/// tackle.
///
/// [pt] Despacho real das chamadas injetadas do movimento: as da dividida, o marcado que sai (0x714d40)
/// e o evento 0x82 (0x68d0d0, o bloco pedido por ultimo).
pub(crate) unsafe fn dispatch_move_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        0x714d40 => {
            exe::match_player_choose_defensive_off_ball_move(this);
            0
        }
        0x68d0d0 => dispatch_injury_list_call(routine, this, a),
        _ => dispatch_tackle_calls(routine, this, a),
    }
}

/// the player moves one cell (two, if fast) towards (x, y) with the given action. Without the ball, the tackle injury
/// and, if not facing the target, only turns. With the ball: the attack from stamina and action, the approach
/// commentary, the challenger (foul with event 0x82, clean contest), possession and the action commentary; then the
/// grid, the furthest forward, the marked players released, wear from the position's marks and the ball (follows it or
/// takes the loose one). Compared through the tape (commentary, `move_off_ball`, control, set piece, ball out, the event and
/// the injury and foul calls as units)
/// Original: `player_move` (0x6f09f0, match_pl.cpp (9 KB), thiscall, ret 0x14).
///
/// [pt] { 0x6f09f0 }: ver `player_move_injected`.
pub(crate) unsafe extern "thiscall" fn player_move(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    player_move_injected(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |r, t, a| dispatch_move_calls(r, t, a));
    0
}

/// Comparer for `player_move`: original vs Rust player move over a call tape with masks; registers and returns the result.
pub(crate) unsafe fn compare_player_move(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(IDX_PLAYER_MOVE);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let mov = RECORD_MOVE.swap(true, Relaxed);
    let (r, _, igual, sequencia, k) = compare_com_fita(reg, || o(p, a1, a2, a3, a4, a5), |chamar| {
        player_move_injected(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x6cd490 => [0xff, 0, 0, 0, 0],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        });
        0
    });
    RECORD_MOVE.store(mov, Relaxed);
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia, || format!("move_to_cell(acao {:#x}) {} chamadas: {}", a3 as u16, k, tape_detail()));
    r
}

/// Comparer for `shot_on_goal`: original vs Rust shot on goal over a call tape with masks.
pub(crate) unsafe fn compare_shot_on_goal_inner(p: usize, f: f32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, f32)>(IDX_SHOT_ON_GOAL);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(p, f), |chamar| {
        shot_on_goal_injected(p, f, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
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
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia, || format!("shot_on_goal {} chamadas: {}", k, tape_detail()));
}

/// Units of the dribbling tape: the copy of 0x68d0d0 goes to the list on the stack (a different address in the Rust
/// copy: it writes without the `this`) and `shoot` (the shot) is a unit.
///
/// [pt] Unidades da fita da conducao: a copia de 0x68d0d0 vai para a lista na pilha (endereco diferente
/// na copia Rust: grava sem o `this`) e a 0x702050 (o chute) entra como unidade.
pub(crate) static RECORD_BALL_CARRY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Is row y in the side's attacking half (side 1: below 6; side 0: 6 and up; other sides: yes)?
///
/// [pt] Na metade de ataque do lado (lado 1: linhas abaixo de 6; lado 0: de 6 em diante; outro lado, sim).
pub(crate) fn is_in_attacking_half(side: i8, y: i8) -> bool {
    !((side == 1 && y >= 6) || (side == 0 && y < 6))
}

/// Is row y in the side's defending half (sides 0 and 1 only)?
///
/// [pt] Na metade de defesa do lado (o contrario da de ataque, so para os lados 0 e 1).
pub(crate) fn is_in_defending_half(side: i8, y: i8) -> bool {
    (side == 1 && y >= 6) || (side == 0 && y < 6)
}

/// Is (x, y) in the penalty area the side defends?
///
/// [pt] Na propria area (a que o lado defende; lado 1: linhas 10-11, os outros 0-1).
pub(crate) fn is_in_defended_box(side: i8, x: i8, y: i8) -> bool {
    is_in_attacked_box((side != 1) as i8, x, y)
}

/// `0x7109f0` (match_pl.cpp, ~13 KB) with the calls that change things outside the engine injected (0x6fa250 the pass,
/// 0x6fa490 the long ball, 0x70ec80 the tackle, `player_move` the movement, `shoot` the shot, 0x6d6f70 commentary, 0x68d0d0
/// the copy in the pressure list): the decision of player `p` with the ball; returns 1 only when dribbling.
///
/// Set pieces (`engine::RESTART_TYPE`): 2 and 3 (corners, 0x36/0x34), 4 (throw-in, 0x5a), 6 and 7 (free kicks, 0x37) pass to the
/// chosen team-mate (0x6c6a30) or to a randomly drawn square; 5 (penalty, 0x33) and 6 near the goal (0x35) shoot
/// (`exe::match_player_get_shot_score` + `shoot`).
///
/// In open play, it adds up the ratings for shooting (+0x10), passing (+0x14) and dribbling (+0x44) by the distance to
/// goal, the pressure (0x6bcbd0), the score, the goalkeeper, the position marks and the clock; it may play a long ball
/// (0x6fa490), tackle (0x70ec80), shoot (`shoot`), pass (0x6fa250) or dribble (`player_move`) to the chosen square
/// (`exe::cm3_match_engine_choose_move_cell`).
///
/// Mode 4: the goalkeeper in the penalty shoot-out. Returns 1 only when dribbling.
///
/// [pt] { 0x7109f0 } (match_pl.cpp, ~13 KB) com as chamadas que mexem fora do motor injetadas (0x6fa250
/// o passe, 0x6fa490 o lancamento, 0x70ec80 a dividida, 0x6f09f0 o movimento, 0x702050 o chute,
/// 0x6d6f70 narra, 0x68d0d0 a copia na lista da pressao): a decisao do jogador `p` com a bola.
/// Bola parada (+0x8eb2): 2 e 3 (escanteios, 0x36/0x34), 4 (lateral, 0x5a), 6 e 7 (faltas, 0x37) passam
/// para o companheiro escolhido (0x6c6a30) ou para uma casa sorteada; 5 (penalti, 0x33) e 6 perto do
/// gol (0x35) chutam (0x700bd0 + 0x702050). Em jogo, soma as notas do chute (+0x10), do passe
/// (+0x14) e da conducao (+0x44) pela distancia ao gol, a pressao (0x6bcbd0), o placar, o goleiro,
/// as marcas da posicao e o relogio; pode lancar (0x6fa490), dividir (0x70ec80), chutar (0x702050),
/// passar (0x6fa250) ou conduzir (0x6f09f0) para a casa escolhida (0x6bce80). Modo 4: o goleiro na
/// disputa de penaltis. Devolve 1 so quando conduz.
#[allow(clippy::cognitive_complexity)]
pub(crate) unsafe fn dribble_injected(p: usize, mode: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let fl = |o: usize| read_f32(p + o) as f64;
    let mut volta = 0u8;
    let m = read_usize(p + match_player::ENGINE);
    let side = read_i8(p + match_player::SIDE);
    let x0 = || read_i8(p + match_player::CELL_X);
    let y0 = || read_i8(p + match_player::CELL_Y);
    let f: f32 = if side == 0 { distance_table_cell_yx(y0() as i32 - 0xb, x0() as i32 - 4) } else { distance_to_point(p, 4, 0) };
    let ff = f as f64;
    let (mut l14, mut l44) = (0i32, 0i32);
    let mut l10: i32;
    let mut l90 = 0f32;
    let mut l70 = 0i16;
    let mut l88: i8 = -1;
    let mut l64 = 0u16;
    let (mut l5f, mut l37) = (false, false);
    let mut la4 = 0i32;
    // the pressure list (0x68d020: counter and head/tail); only the Rust-mode copy fills it
    let mut list = [0u8; 14];
    let la = list.as_mut_ptr() as usize;
    let pass_ball = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, kind: u32, r: usize, moveu: u8, x: i8, y: i8| {
        *w(p, match_player::ACTION) = kind as i16;
        call(FN_MATCH_PLAYER_PLAY_PASS_TO, p, [kind, r as u32, moveu as u32, x as u8 as u32, y as u8 as u32]);
    };
    let shoot = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, kind: u32, v: i32, sound: u16, a4: u32, with_list: bool| {
        call(FN_MATCH_PLAYER_TAKE_SHOT, p, [kind, v as u32, sound as u32, a4, if with_list { la as u32 } else { 0 }]);
    };
    'fim: {
        let c = read_i8(m + engine::RESTART_TYPE);
        let (mut kind, mut moveu, mut x, mut y, mut best) = (0i8, 0u8, 0i8, 0i8, 0i32);
        let choose = |request: i8, kind: &mut i8, moveu: &mut u8, x: &mut i8, y: &mut i8, best: &mut i32| {
            choose_teammate(m, read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y), kind, moveu, x, y, best, request)
        };
        match c {
            2 => {
                *b(m, engine::RESTART_TYPE) = 0;
                let r = choose(0x36, &mut kind, &mut moveu, &mut x, &mut y, &mut best);
                if r == 0 {
                    crate::comparison::branch("dribble corner without teammate");
                    let d: i8 = if side != 1 { 1 } else { -1 };
                    y = read_i8(m + engine::BALL_Y).wrapping_add(d);
                    if y < 0 {
                        y = 0;
                    } else if y > 0xb {
                        y = 0xb;
                    }
                    x = if read_i8(m + engine::BALL_X) == 0 { random(4) as i8 } else { 8 - random(4) as i8 };
                }
                pass_ball(call, 0x36, r, moveu, x, y);
                break 'fim;
            }
            3 => {
                *b(m, engine::RESTART_TYPE) = 0;
                let r = choose(0x34, &mut kind, &mut moveu, &mut x, &mut y, &mut best);
                if r == 0 {
                    x = 5 - random(3) as i8;
                    if side == 1 {
                        y = 0;
                    } else if side == 0 {
                        y = 0xb;
                    }
                }
                pass_ball(call, 0x34, r, moveu, x, y);
                break 'fim;
            }
            4 => {
                *b(m, engine::RESTART_TYPE) = 0;
                let mut r = choose(0x5a, &mut kind, &mut moveu, &mut x, &mut y, &mut best);
                if r == 0 || best < 2000 {
                    crate::comparison::branch("dribble throw_in drawn");
                    x = 7 - random(7) as i8;
                    y = if side == 0 { random(3) as i8 + 6 } else { 5 - random(3) as i8 };
                    r = 0;
                }
                pass_ball(call, 0x5a, r, moveu, x, y);
                break 'fim;
            }
            5 => {
                *b(m, engine::RESTART_TYPE) = 0;
                crate::comparison::branch("dribble penalty");
                let mut t: i8 = 0x33;
                let v = shot_value(p, &mut t, &mut l64) as i16 as i32;
                let a4 = if random(100) == 0 { 5 } else { 1 };
                shoot(call, 0x33, v, l64, a4, false);
                break 'fim;
            }
            6 | 7 => {
                *b(m, engine::RESTART_TYPE) = 0;
                if c == 6 && (2..=6).contains(&x0()) && ff < read_f64(rdata::F64_3_1) {
                    let bl = read_i8(p + match_player::MOMENT_FLAIR) as i32;
                    let n = (random(ftol(fl(match_player::FREE_KICKS) + read_f32(rdata::F32_20) as f64)) + bl) / 3;
                    if (n as f64) > ff {
                        crate::comparison::branch("dribble direct free kick");
                        *b(m, engine::SET_PIECE_KIND) = 6;
                        let mut t: i8 = 0x35;
                        let v = shot_value(p, &mut t, &mut l64) as i16 as i32;
                        let a4 = if random(100) == 0 { 6 } else { 1 };
                        shoot(call, 0x35, v, l64, a4, false);
                        break 'fim;
                    }
                }
                let mut r = choose(0x37, &mut kind, &mut moveu, &mut x, &mut y, &mut best);
                if r == 0 || best < 2000 {
                    crate::comparison::branch("dribble foul in the box");
                    x = 4;
                    y = if side == 0 { read_i8(m + engine::OFFSIDE_LINE).wrapping_sub(2) } else { read_i8(m + engine::OFFSIDE_LINE_SIDE1).wrapping_add(2) };
                    r = 0;
                }
                pass_ball(call, 0x37, r, moveu, x, y);
                break 'fim;
            }
            8 => *b(m, engine::RESTART_TYPE) = 0,
            _ => {}
        }
        // in play: the target (cell of 0x6c2be0) and the ratings
        let (l74, l36): (i8, i8);
        if mode == 4 {
            l74 = 4;
            l36 = if side != 0 { 0 } else { 0xb };
        } else {
            let (mut sx, mut sy) = (0i8, 0i8);
            position_place_players(m, p, &mut sx, &mut sy, 1);
            l74 = adjusted_rating(p, sx) as i8;
            l36 = sy;
            if (side == 0 && y0() > l36) || (side == 1 && y0() < l36) {
                l90 = distance_table_cell_yx(y0() as i32 - l36 as i32, x0() as i32 - l74 as i32);
            }
        }
        let l53 = beyond_the_line(m, y0(), side, 0);
        let mut l27: i8 = 0;
        let l68 = closest_to_goal(m, p);
        let mut l60 = smallest_distance(m, (side == 0) as i8, x0(), y0(), 1);
        let l2f = pressure_at_point(m, (side == 0) as i8, x0(), y0(), 1);
        let mut l49: i8 = 0;
        let mut l4c = false;
        let g = hold_position_one(m, (side == 0) as i8);
        let (mut home, mut fora, mut cl, mut ida, mut f3) = (0i8, 0i8, 0u8, 0u8, 0u8);
        aggregate_score(m, side, &mut home, &mut fora, &mut cl, &mut ida, &mut f3, 0);
        let mut l30 = 0i32;
        let mut l58 = 0i32;
        let (mut l8c, mut l54, mut l94, mut l98) = (0i8, 0u8, 0i8, 0i8);
        if mode == 4 {
            *w(p, 0x29) = 0x15e;
        } else {
            if read_i8(m + engine::BALL_HEIGHT) == -1 {
                *w(p, 0x29) = 200;
            } else {
                l49 = pressure_on_player_with(m, p, &mut l70, la, &mut |l, q| {
                    call(FN_LLIST_ADD_TAIL, l, [q as u32, 0x1be, 0, 0, 0]);
                }) as i8;
                if l49 == 0 {
                    if l60 as f64 > read_f64(rdata::F64_3_5) {
                        l60 = 3.5;
                    }
                    *w(p, 0x29) = ftol(l60 as f64 * read_f32(rdata::F32_60) as f64 + read_f32(rdata::F32_57) as f64) as i16;
                } else {
                    if l70 < 0 {
                        l70 = 0;
                    }
                    let s = l70 as i32;
                    let k = (l49 as i32).wrapping_mul(s).wrapping_mul(s);
                    l58 = ftol(-(k as f64 * read_f64(rdata::F64_0_75)));
                    if l58 < -500 {
                        l58 = random((l58 + 500) / 3) - 500;
                        if l58 < -2000 {
                            l58 = -2000;
                        }
                    } else if l58 > 32000 {
                        l58 = 32000;
                    }
                    *w(p, 0x29) = ftol(l58 as f64 * read_f64(rdata::F64_0_75)) as i16;
                }
            }
            if is_in_attacking_half(side, y0()) {
                l27 = players_on_path(m, x0(), y0(), 4, if side != 0 { 0 } else { 0xb }) as i8;
            }
            let sum = if is_in_attacked_box(side, x0(), y0()) { 2 } else { 4 };
            let mut l28 = 0i32;
            l58 = choose_teammate(m, x0(), y0(), &mut l8c, &mut l54, &mut l94, &mut l98, &mut l28, -1) as i32;
            l30 = (l49 as i32 + l27 as i32 + sum).wrapping_mul(l28) / 5;
        }
        *w(p, 0x29) = (*w(p, 0x29)).wrapping_add(0x46);
        let flags = *((m + engine::TACTICS_FLAGS + (side == 0) as usize * 0x18e3) as *const u32);
        if flags & 0x400 != 0 && read_i8(p + match_player::MOMENT_DECISIONS) as i32 > random(5) && !is_in_attacked_box(side, x0(), y0()) && !is_in_defended_box(side, x0(), y0()) && is_in_attacking_half(side, y0()) {
            crate::comparison::branch("dribble instruction to push forward");
            l14 = 4000 - l30;
            l44 = 2000 - l30 / 2;
        }
        // the shot rating
        if read_i8(m + engine::BALL_HEIGHT) == 4 {
            if ff < read_f64(rdata::F64_1) || ((y0() == 0 || y0() == 0xb) && (x0() as i32 - 4).abs() == 1) {
                l10 = ftol(fl(match_player::HEADING) * read_f32(rdata::F32_75) as f64 + read_f32(rdata::F32_2500) as f64 - (400 * l27 as i32) as f64);
            } else {
                l10 = -5000;
            }
        } else if ff <= read_f64(rdata::F64_1_5) {
            l10 = ftol(fl(match_player::FINISHING) * read_f32(rdata::F32_100) as f64 + read_f32(rdata::F32_3000) as f64 - (400 * l27 as i32) as f64 + *w(p, 0x29) as f64 * read_f64(rdata::F64_1_2));
        } else {
            let bl = read_i8(p + match_player::MOMENT_FLAIR) as i32;
            let n = bl / 7 + random(3) + 3;
            if (n as f64) <= ff || (mode == 4 && bl <= random(0x32)) {
                l10 = -500;
            } else {
                crate::comparison::branch("dribble long-range shot");
                let v = shot_value(p, &mut l88, &mut l64) as i16 as i32;
                let k = read_i8(p + match_player::MOMENT_DECISIONS) as i32 * 4;
                let r1 = random(ftol(ff * read_f32(rdata::F32_75) as f64 - k as f64));
                let t2 = ftol(k as f64 - ff * read_f32(rdata::F32_75) as f64);
                la4 = random(-(r1.wrapping_add(t2))).wrapping_add(v);
                l37 = true;
                if v > 0 {
                    let div = if read_i8(p + match_player::MOMENT_FLAIR) < 1 { 1 } else { read_i8(p + match_player::MOMENT_FLAIR) as i32 };
                    l10 = ftol((v * v) as f64 * read_f64(rdata::F64_0_3333333333) - ff * ff * read_f32(rdata::F32_4000) as f64 / div as f64 - (700 * l27 as i32) as f64);
                } else {
                    l10 = ((read_i8(p + match_player::MOMENT_VISION) as i32 + read_i8(p + match_player::MOMENT_DECISIONS) as i32) * 4 + 2) * v;
                }
            }
        }
        // o placar
        let dif = fora as i32 - home as i32;
        if read_i8(m + (0x1ab + side as usize)) == 5 {
            l10 = l10.wrapping_add(dif * 3000);
            l30 = l30.wrapping_mul(2);
            if !(m != 0 && table_a(m, p) == 1) && random((dif * 3 + read_i8(read_usize(p + match_player::STAFF) + 0x5b) as i32) * 2) < 1 {
                crate::comparison::branch("dribble knockout loses stamina");
                *b(p, match_player::FRUSTRATION) = (*b(p, match_player::FRUSTRATION)).wrapping_sub(1);
            }
        } else if (fora as i32) < home as i32 - 2 {
            l10 = l10.wrapping_add((dif + 2) * 3000);
            l30 = l30.wrapping_mul(2);
        }
        if read_i8(m + engine::BALL_HEIGHT) < 4 && ff > read_f32(rdata::F32_0) as f64 {
            if l68 == 2 {
                let pw = game_pow(ftol(ff) as f64, read_f64(rdata::F64_3));
                let q = fl(match_player::DRIBBLING);
                let r = random(4) + 1;
                let k = ftol((q * q + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5)) as i16 as i32;
                l14 = l14.wrapping_add(ftol((r * k) as f64 * pw));
            }
            if distance_table_cell_yx(y0() as i32 - read_i8(g + match_player::CELL_Y) as i32, x0() as i32 - read_i8(g + match_player::CELL_X) as i32) as f64 == read_f64(rdata::F64_0)
                && read_i8(m + (0x21b6 + (y0() as isize + x0() as isize * 12) as usize * 0x5a + (side == 0) as usize)) == 1
            {
                crate::comparison::branch("dribble one on one");
                let r1 = random(ftol(fl(match_player::FINISHING) * read_f32(rdata::F32_10) as f64));
                let r2 = random(ftol(fl(match_player::DRIBBLING) * read_f32(rdata::F32_10) as f64));
                l14 = l14.wrapping_add(ftol((r2 - r1) as f64 * (ff * read_f32(rdata::F32_50) as f64 + read_f32(rdata::F32_150) as f64)));
                l4c = true;
            }
        }
        // the end of the match: the challenge to waste time
        if l68 == 0 && l30 < 3000 && mode != 4 && (*w(m, engine::CLOCK) as i32 / 11) > random(0x14) + 0x46 && !is_in_attacked_box(side, x0(), y0()) && read_i8(p + match_player::MOMENT_DECISIONS) != 0 && read_i8(m + engine::BALL_HEIGHT) < 3 {
            let e = random(read_i8(read_usize(p + match_player::STAFF) + 0x5c) as i32);
            if e < random(10) && side_under_control(m, side) != 0 {
                let r = aggregate_score(m, side, &mut home, &mut fora, &mut cl, &mut ida, &mut f3, 1) as i8;
                if r > 0 && r < 3 {
                    if !(2..=6).contains(&x0()) && ((side == 1 && y0() < 3) || (side == 0 && y0() > 8)) {
                        crate::comparison::branch("dribble holds the ball");
                        call(FN_MATCH_PLAYER_HOLD_BALL, p, [1, 0, 0, 0, 0]);
                        *w(p, match_player::ACTION) = 0x10;
                        // see the end of 0x7139cb
                        break 'fim;
                    }
                    if is_in_attacking_half(side, y0()) && *w(p, 0x29) > -0x14 && l10 < 0 {
                        crate::comparison::branch("dribble holds the result");
                        l5f = true;
                        l14 += 4000;
                    }
                }
            }
        }
        if ff < read_f64(rdata::F64_3) && l10 > 0 {
            l44 = l44.wrapping_add(l10 / -5);
        }
        if m != 0 && table_a(m, p) == 1 {
            if read_i8(p + match_player::MOMENT_POSITIONING) != 0 {
                l14 = l14.wrapping_add(ftol(fl(match_player::FLAIR) * read_f32(rdata::F32_250) as f64 - read_f32(rdata::F32_5000) as f64));
            }
            if is_in_defending_half(side, y0()) {
                l10 = l10.wrapping_sub(25000);
            }
        }
        // the goalkeeper (or whoever holds position 1)
        if g != 0 && is_in_attacking_half(side, y0()) {
            let t = if read_usize(g + match_player::ENGINE) != 0 { table_a(read_usize(g + match_player::ENGINE), g) } else { 0 };
            if t & 1 == 0 {
                l10 = l10.wrapping_add(5000);
            } else if read_i8(g + match_player::BUSY) != 0 && read_i8(p + match_player::MOMENT_VISION) as i32 + 5 > random(ftol(ff * read_f32(rdata::F32_20) as f64)) {
                l10 = l10.wrapping_add(1500);
            }
            let (gx, gy) = (read_i8(g + match_player::CELL_X), read_i8(g + match_player::CELL_Y));
            if !is_in_attacked_box(side, gx, gy) {
                if read_i8(p + match_player::MOMENT_VISION) as i32 + 5 > random(ftol(ff)) {
                    l10 = l10.wrapping_sub(ftol(fl(match_player::DRIBBLING) * read_f32(rdata::F32_100) as f64 - read_f32(rdata::F32_2500) as f64));
                }
            } else if gx == 4 && gy == if read_i8(g + match_player::SIDE) == 1 { 0xb } else { 0 } {
                if reaches_the_point(g, x0(), y0()) == 0 && read_i8(p + match_player::MOMENT_VISION) as i32 > random(ftol(ff * read_f32(rdata::F32_10) as f64)) {
                    crate::comparison::branch("dribble keeper in goal");
                    l10 = l10.wrapping_sub(ftol(fl(match_player::DRIBBLING) * read_f32(rdata::F32_40) as f64 - read_f32(rdata::F32_1000) as f64));
                }
            } else {
                let e17 = read_i8(p + match_player::MATCH_FORM) as i32;
                let r = random(ftol(ff * ff * read_f32(rdata::F32_3) as f64));
                if read_i8(p + match_player::MOMENT_VISION) as i32 + e17 - 10 > r {
                    crate::comparison::branch("dribble keeper off his line");
                    l10 = l10.wrapping_sub(ftol(fl(match_player::DRIBBLING) * read_f32(rdata::F32_40) as f64 - read_f32(rdata::F32_1500) as f64));
                }
                if x0() == gx && y0() == gy {
                    l14 = l14.wrapping_add(ftol((e17 as f64 + fl(match_player::DRIBBLING)) * read_f32(rdata::F32_100) as f64 - read_f32(rdata::F32_3000) as f64));
                }
            }
        }
        if read_i8(m + engine::BALL_HEIGHT) == 4 && ff > read_f64(rdata::F64_1) {
            l10 = l10.wrapping_add(ftol(ff * ff * read_f64(rdata::F64_NEG_750)));
        } else if (4..=6).contains(&x0()) && ((side == 1 && y0() == 0) || (side == 0 && y0() == 0xb)) {
            l10 = l10.wrapping_add(5000 - (x0() as i32 - 4).abs() * 2000);
        }
        if l58 == 0 && l53 == 0 {
            l30 -= 2000;
        }
        // lancar
        if limit((side == 0) as u8, y0()) != 0 && read_i8(m + engine::BALL_HEIGHT) != -1 && read_i8(p + match_player::MOMENT_DECISIONS) != 0 && l30 < 4000 - *w(m, engine::CLOCK) as i32 && side_under_control(m, side) != 0 {
            crate::comparison::branch("dribble long ball");
            call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 1, 0, 0, 0]);
            break 'fim;
        }
        let near = (is_in_defended_box(side, x0(), y0()) && l60 as f64 <= read_f64(rdata::F64_1_5)) || (limit((side == 0) as u8, y0()) != 0 && (l60 as f64) < read_f64(rdata::F64_1));
        if near {
            let r1 = random(ftol(read_f32(rdata::F32_3000) as f64 - l60 as f64 * read_f32(rdata::F32_1000) as f64));
            if r1 > random(l30 + 2000) && read_i8(m + engine::BALL_HEIGHT) != -1 && mode != 4 {
                crate::comparison::branch("dribble clearance");
                call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        // the target (the middle of the attacked goal) and the goal mouth
        let ty: i8 = if side == 0 { 0xb } else { 0 };
        let na_boca = || y0() == ty && (3..=5).contains(&x0());
        let consegue = || can_play(p, 4, ty, 0, 0.0, 0.0, core::ptr::null_mut()) != 0;
        if (side_difference(m, side as u8, y0() as u8) as i8) < 0 && (clear_path(m, p, 0) == 0 || l53 != 0 || table_a(m, p) == 1) {
            if is_in_attacked_box(side, x0(), y0()) {
                l44 -= 1000;
            }
            if is_in_defended_box(side, x0(), y0()) {
                l44 -= 1000;
                l14 -= 1200;
            } else if is_in_defending_half(side, y0()) && !na_boca() && !consegue() {
                l14 = l14.wrapping_add((-5 - l70 as i32) * 100);
            }
        } else {
            l44 -= 5000;
            l14 = l14.wrapping_add(ftol(ff * read_f32(rdata::F32_1500) as f64 - read_f32(rdata::F32_1500) as f64));
            if mode == 4 {
                l14 = l14.wrapping_add(ftol(ff * read_f32(rdata::F32_2000) as f64 - read_f32(rdata::F32_1500) as f64));
            }
            if is_in_defending_half(side, y0()) {
                l10 -= 3000;
            }
            if is_in_attacked_box(side, x0(), y0()) {
                l44 -= 1000;
                l10 += 1000;
            }
        }
        if table_a(m, p) == 1 || is_in_defended_box(side, x0(), y0()) {
            l44 = l44.wrapping_add(ftol(l60 as f64 * read_f32(rdata::F32_500) as f64 - read_f32(rdata::F32_1200) as f64));
        } else {
            l14 = l14.wrapping_add(ftol((fl(match_player::DRIBBLING) - read_f32(rdata::F32_20) as f64 + l60 as f64 * read_f64(rdata::F64_7_5)) * read_f64(rdata::F64_120)));
        }
        if !na_boca() && !consegue() && is_in_attacking_half(side, y0()) {
            l14 = l14.wrapping_add(2000 - l30);
        }
        // the square to carry the ball to (0x6bce80)
        let (mut sx, mut sy) = (l74, l36);
        let mut l43 = choose_square(m, p, &mut sx, &mut sy, (l68 != 0) as u32 + 2) as i8;
        let bottom = (side == 1 && y0() < 3) || (side == 0 && y0() > 8);
        let ta = |q: usize| if read_usize(q + match_player::ENGINE) != 0 { table_a(read_usize(q + match_player::ENGINE), q) } else { 0 };
        let tb = |q: usize| if read_usize(q + match_player::ENGINE) != 0 { table_b(read_usize(q + match_player::ENGINE), q) } else { 0 };
        if l43 != 0 {
            if mode == 4 {
                sx = sx.clamp(3, 5);
            }
            l44 -= 3000;
            if !(2..=6).contains(&x0()) && !bottom && !is_in_defending_half(side, y0()) {
                l14 += 1250;
                l10 -= 500;
            } else if bottom && !is_in_attacked_box(side, x0(), y0()) {
                l14 += 1000;
                l10 = l10.wrapping_add((read_i8(p + match_player::MOMENT_FLAIR) as i32 - 20) * 100);
            }
        } else if (2..=6).contains(&x0()) || bottom {
            l14 -= 3000;
        } else {
            let k = (read_i8(p + match_player::MOMENT_FLAIR) as f64 + fl(match_player::DRIBBLING)) * read_f32(rdata::F32_10) as f64;
            if table_a(m, p) & 0x38 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !((side == 0 && y0() < 3) || (side == 1 && y0() > 8)) {
                crate::comparison::branch("dribble target man");
                l14 = l14.wrapping_add(ftol(k));
            } else if table_a(m, p) & 0x38 != 0 && table_a(m, p) & 0x880 != 0 {
                l14 = l14.wrapping_add(ftol(k - read_f32(rdata::F32_250) as f64));
            } else {
                l14 -= 3000;
            }
            l10 -= 650;
        }
        // the shot from outside the box
        let in_the_box = is_in_attacked_box(side, x0(), y0());
        if (2..=6).contains(&x0()) && !in_the_box && read_i8(m + engine::BALL_HEIGHT) != 4 {
            let t = table_a(m, p);
            let bl = read_i8(p + match_player::MOMENT_FLAIR) as i32;
            let r = random(ftol((l27 as i32 + 1) as f64 * (ff + read_f32(rdata::F32_1) as f64) * ff * ff * ff * read_f32(rdata::F32_200) as f64));
            l10 = l10.wrapping_add(if t & 0x40 != 0 { bl * 500 } else { bl * 325 } - r);
        } else if in_the_box {
            l10 = l10.wrapping_add(ftol((read_i8(p + match_player::MOMENT_FLAIR) as f64 + fl(match_player::FINISHING) - fl(match_player::TEAMWORK)) * (read_f32(rdata::F32_20) as f64 - ff * read_f32(rdata::F32_4) as f64)));
        } else if is_in_attacking_half(side, y0()) {
            l10 = l10.wrapping_sub(ftol(ff * read_f32(rdata::F32_300) as f64 - (read_i8(p + match_player::MOMENT_FLAIR) as i32 * 75) as f64));
        }
        let marks = |q: usize| position_marks(m + engine::TACTICS + read_i8(q + match_player::SIDE) as isize as usize * 0x18e3, read_i8(q + match_player::POSITION));
        'rating: {
            if random(ftol(fl(match_player::TEAMWORK))) == 0 && !(table_a(m, p) & 0x40 == 0 && random(4) != 0) {
                crate::comparison::branch("dribble stamina");
                if !(is_in_defending_half(side, y0()) && (read_i8(p + match_player::MOMENT_DECISIONS) != 0 || read_i8(p + match_player::MOMENT_VISION) != 0)) {
                    let bl = is_in_defended_box(side, x0(), y0()) as i32;
                    let pw = game_pow(fl(match_player::DRIBBLING), read_f64(rdata::F64_3));
                    l14 = l14.wrapping_add(ftol(pw + (1000 - bl * 1000) as f64));
                }
                if ff >= read_f64(rdata::F64_2) && ff < read_f64(rdata::F64_4_5) && (2..=6).contains(&x0()) {
                    if read_i8(p + match_player::MOMENT_FLAIR) != 0 {
                        let r = random(ftol(fl(match_player::LONG_SHOTS)));
                        l10 = l10.wrapping_add(*w(p, 0x29) as i32 * 2 + r * 400 + 2500);
                    }
                } else if in_the_box {
                    l10 = l10.wrapping_add(ftol((read_f32(rdata::F32_20) as f64 - ff * read_f32(rdata::F32_4) as f64) * (read_i8(p + match_player::MOMENT_FLAIR) as f64 + fl(match_player::FINISHING))));
                }
                break 'rating;
            }
            if mode == 4 {
                break 'rating;
            }
            if marks(p) & 0x2000000 != 0 && !is_in_defended_box(side, x0(), y0()) && l49 < 2 && ff >= read_f64(rdata::F64_2) {
                let follows = if l70 != 0 && is_in_defending_half(side, y0()) { (random(read_i8(p + match_player::MOMENT_DECISIONS) as i32 * 2 + 20) as f64) < ff } else { true };
                if follows {
                    crate::comparison::branch("dribble playmaker");
                    let k = (read_i8(p + match_player::DRIBBLES) as u8 as i32 * 30 + *w(p, 0x29) as i32 + 400) * 10 - l30;
                    l14 = l14.wrapping_add(ftol(k as f64 + read_i8(p + match_player::MOMENT_FLAIR) as f64 * fl(match_player::DRIBBLING) * fl(match_player::TEAMWORK)));
                    if !(2..=6).contains(&x0()) && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !bottom && marks(p) & 0x4_0000_0000 != 0 {
                        l14 += 500;
                        l30 -= 250;
                    }
                }
            }
            if marks(p) & 0x8000000 != 0 && is_in_attacking_half(side, y0()) && !na_boca() && !consegue() {
                l44 += 3500;
            }
            if marks(p) & 0x20000000 != 0 && ff >= read_f64(rdata::F64_2) && ff < read_f64(rdata::F64_4_5) {
                let r = random(ftol(fl(match_player::LONG_SHOTS)));
                l10 = l10.wrapping_add((*w(p, 0x29) as i32 + r * 200) * 2);
            }
        }
        // the swerving move and the pressure
        let c = read_i8(m + engine::BALL_HEIGHT);
        if c < 2 || (ff > read_f64(rdata::F64_1_41) && c < 4) {
            let mut out = 0u16;
            let r = classify_move(p, 0x16, x0(), y0(), 4, if side == 1 { 0 } else { 0xb }, (ff + read_f64(rdata::F64_1)) as f32, &mut out);
            if r != 2 && r != read_i8(p + match_player::PREFERRED_FOOT) as u8 && l68 == 0 {
                l10 = l10.wrapping_sub(random(read_i8(p + 0x2d) as i32 * 300));
            }
        }
        if ff < read_f64(rdata::F64_5) {
            let s = l70 as i32;
            l10 = l10.wrapping_sub(ftol(s.wrapping_mul(s).wrapping_mul(s) as f64 / (read_f64(rdata::F64_10) - ff)));
        }
        if read_i8(m + engine::BALL_HEIGHT) == 3 {
            l10 -= 2000;
        }
        let e = (l43 as i32 - l2f as i8 as i32) * 100;
        let t = ftol((read_f32(rdata::F32_400) as f64 - fl(match_player::FLAIR) * read_f32(rdata::F32_15) as f64) * l90 as f64 * l90 as f64);
        l44 = l44.wrapping_add(e);
        l14 = l14.wrapping_add(e - t);
        let bl = read_i8(p + match_player::MOMENT_FLAIR) as i32;
        if !(ff < read_f64(rdata::F64_1)) && bl * bl * bl * bl > random(ftol(ff * read_f32(rdata::F32_10000) as f64)) {
            if !is_in_defended_box(side, x0(), y0()) && ff >= read_f64(rdata::F64_2) && l49 < 2 && !(is_in_defending_half(side, y0()) && (read_i8(p + match_player::MOMENT_DECISIONS) != 0 || read_i8(p + match_player::MOMENT_VISION) != 0)) {
                let pw = game_pow(fl(match_player::DRIBBLING), read_f64(rdata::F64_3));
                l14 = l14.wrapping_add(random(ftol(pw + (*w(p, 0x29) as i32 * 3) as f64 + (10000 - l30) as f64)));
            }
            if read_i8(m + engine::BALL_HEIGHT) < 4 && l27 < 2 && (ff < read_f64(rdata::F64_5) || (ff < read_f64(rdata::F64_7) && random(500) == 0)) {
                crate::comparison::branch("dribble placed shot");
                let mut r = random(ftol(ff * read_f32(rdata::F32_10) as f64)) as i8;
                if r < 10 {
                    r = 10;
                }
                l10 = l10.wrapping_add(random(400000 / r as i32));
            }
            l44 -= 10000;
        } else if !is_in_defended_box(side, x0(), y0()) {
            let e = random(ftol(read_i8(p + match_player::MOMENT_DECISIONS) as f64 * read_f64(rdata::F64_30)));
            if e > random(200) {
                // the opponents on the same square
                let home = m + engine::GRID + (y0() as isize + x0() as isize * 12) as usize * 90;
                let other = (side == 0) as usize;
                let mut i = 0i8;
                while i < *((home + other + 0x58) as *const i8) {
                    let q = *((home + other * 44 + i as usize * 4) as *const usize);
                    if read_i8(q + match_player::YELLOW_CARDS) != 0 {
                        l14 = l14.wrapping_add(random(ftol((read_i8(p + match_player::MATCH_FORM) as i32 * 100) as f64 + fl(match_player::DRIBBLING) * read_f32(rdata::F32_125) as f64)));
                    }
                    i += 1;
                }
            }
        }
        // the dribbling discount (the original’s ebx)
        let k = ftol(game_pow(read_i8(m + engine::BALL_HEIGHT) as f64, read_f64(rdata::F64_3)) * read_f64(rdata::F64_NEG_200));
        let mut nc = (-(*w(p, 0x29) as i32) - *w(p, match_player::BALL_CONTROL) as i32 * 10) * 10;
        nc = nc.wrapping_sub(ftol((read_f32(rdata::F32_10000) as f64 - fl(match_player::STAMINA)) * read_f32(rdata::F32_NEG_0_200000003) as f64)).wrapping_sub(k);
        if l58 != 0 && l30 > 0 && matches!(l8c as u8, 0x5e | 8 | 7 | 6) {
            nc = nc.wrapping_add(l30);
        } else if marks(p) & 0x4_0000_0000 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 {
            if y0() == 0 || y0() == 0xb {
                nc += 500;
            } else if (2..=6).contains(&x0()) {
                // nada
            } else if bottom {
                l14 += 2000;
            } else if ff < read_f64(rdata::F64_5) {
                l14 += 6000;
            }
        }
        if !l4c && l10 > 2000 && *w(p, 0x29) > -0x32 {
            nc = nc.wrapping_add(random(l10));
        }
        if read_i8(p + match_player::MOMENT_DECISIONS) == 0 {
            nc = random(nc);
        }
        if is_in_defending_half(side, y0()) && nc < 0 {
            nc = 0;
        }
        l14 = l14.wrapping_sub(nc);
        // on the byline, away from the middle: crosses (or keeps the set play)
        if off_centre(x0()) != 0 && !(m != 0 && read_i8(m + engine::BALL_HEIGHT) >= 3) && ((side == 0 && y0() == 0xb) || (side == 1 && y0() == 0)) {
            if l58 != 0 && matches!(l8c, 8 | 7 | 6) {
                l30 += 1500;
                if marks(p) & 0x4_0000_0000 != 0 {
                    l30 += 1500;
                }
            } else if random(10) != 0 {
                crate::comparison::branch("dribble crosses from the byline");
                call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        let k = (20 - read_i8(p + match_player::MOMENT_DECISIONS) as i32) * 50;
        l30 = l30.wrapping_add(random(k));
        l14 = l14.wrapping_add(random(k));
        let l44s = l44.wrapping_add(random(k));
        let mut rating = l10.wrapping_add(random(k));
        // a acao: 1 passa, 2 divide, 3 chuta, 4 conduz
        let mut action = 3u8;
        if rating < l14 && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 && !(x0() == 4 && ((side == 0 && y0() == 0xb) || (side == 1 && y0() == 0))) {
            rating = l14;
            action = 4;
        }
        if rating < l44s && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 && clear_path(m, p, 1) == 0 && !(*w(p, match_player::ACTION) == 0x10 && read_i8(m + (0x8d46 + side as usize)) >= 0xc) {
            rating = l44s;
            action = 2;
        }
        'action: {
            if *w(p, match_player::ACTION) != 0x10 && read_i8(m + engine::BALL_HEIGHT) == -1 {
                action = 2;
                break 'action;
            }
            if rating < l30 && mode != 4 {
                if l58 != 0 {
                    action = 1;
                    break 'action;
                }
                if (l53 != 0 && !bottom) || read_i8(m + engine::BALL_HEIGHT) == 4 {
                    crate::comparison::branch("dribble clears without teammate");
                    call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                action = if random(if *w(p, match_player::ACTION) != 0x10 { 2 } else { 10 }) != 0 { 4 } else { 2 };
            }
            if action == 3 {
                if !na_boca() && !consegue() {
                    match read_i8(m + engine::BALL_HEIGHT) {
                        3 => {
                            if !(read_i8(p + match_player::MOMENT_FLAIR) as i32 > random(0xf) && ff < read_f64(rdata::F64_2_1)) {
                                action = 2;
                                break 'action;
                            }
                        }
                        4 => {
                            action = 1;
                            break 'action;
                        }
                        c => {
                            action = if c == -1 { 1 } else { 4 };
                            break 'action;
                        }
                    }
                }
                let c = read_i8(m + engine::BALL_HEIGHT);
                if (c == 4 && !(ff < read_f64(rdata::F64_2))) || c == -1 {
                    action = 1;
                    break 'action;
                }
                if !is_in_defending_half(side, y0()) && ((2..=6).contains(&x0()) || read_i8(p + match_player::MOMENT_FLAIR) as i32 >= random(0x78)) {
                    break 'action;
                }
                if mode == 4 {
                    break 'action;
                }
                if l58 != 0 {
                    action = 1;
                    break 'action;
                }
                if (m != 0 && read_i8(m + engine::BALL_HEIGHT) >= 3) || l60 as f64 <= read_f64(rdata::F64_1_5) {
                    crate::comparison::branch("dribble clears from distance");
                    call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                if *w(p, match_player::ACTION) == 2 {
                    action = 4;
                } else {
                    let bl = read_i8(p + match_player::MOMENT_FLAIR) as i32;
                    let e = random(ftol(fl(match_player::DRIBBLING)) as i16 as i32) + bl;
                    action = if e < random(0x32) { 2 } else { 4 };
                }
            } else if action == 4 && (read_i8(m + engine::BALL_HEIGHT) == 4 || read_i8(m + engine::BALL_HEIGHT) == -1) {
                action = 1;
            }
        }
        if *w(p, match_player::ACTION) == 0x3b {
            action = 3;
        }
        if mode == 4 && (action == 1 || action == 2 || l43 == 0) {
            action = 3;
        }
        let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
            call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [0x1f44, m as u32, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, u32::MAX]);
            *w(p, match_player::MATCH_RATING) = (*w(p, match_player::MATCH_RATING)).wrapping_add(0x19);
        };
        match action {
            1 => {
                if l58 == 0 {
                    crate::comparison::branch("dribble long ball without teammate");
                    *w(p, match_player::ACTION) = 9;
                    call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                } else {
                    crate::comparison::branch("dribble passes");
                    *w(p, match_player::ACTION) = 0x6c;
                    call(FN_MATCH_PLAYER_PLAY_PASS_TO, p, [l8c as u8 as u32, l58 as u32, l54 as u32, l94 as u8 as u32, l98 as u8 as u32]);
                }
            }
            2 => {
                crate::comparison::branch("dribble 50-50 challenge");
                call(FN_MATCH_PLAYER_HOLD_BALL, p, [0; 5]);
                *w(p, match_player::ACTION) = 0x10;
            }
            3 => {
                if mode == 4 {
                    crate::comparison::branch("dribble keeper stays");
                    volta = 1;
                    break 'fim;
                }
                crate::comparison::branch("dribble shoots");
                if read_i8(m + engine::SHOT_PENDING) != 0 && *w(p, match_player::ACTION) != 0x3b && read_i8(m + engine::BALL_HEIGHT) == 0 {
                    narrate(call);
                }
                let v = if l37 {
                    let v = shot_value(p, &mut l88, &mut l64) as i16 as i32;
                    v / 2 + la4 / 2
                } else {
                    shot_value(p, &mut l88, &mut l64) as i16 as i32
                };
                let r = check_move(m);
                shoot(call, l88 as u8 as u32, v, l64, r as u32, true);
            }
            _ => {
                // dribble
                let (mut cx, mut cy) = (sx, sy);
                if l5f {
                    crate::comparison::branch("dribble holds in the corner");
                    cx = if l74 > 4 || (l74 == 4 && random(2) == 0) { 8 } else { 0 };
                    cy = ty;
                    l43 = 1;
                }
                if !na_boca() && !consegue() && (l43 == 0 || mode == 4) {
                    crate::comparison::branch("dribble turns towards goal");
                    let d = if side == 0 {
                        if y0() < 9 { y0() + 3 } else { 0xb }
                    } else if y0() > 3 {
                        y0() - 3
                    } else {
                        0
                    };
                    aim_set_facing_direction(p, x0(), y0(), 4, d);
                    call(FN_MATCH_PLAYER_MOVE_TO, p, [x0() as u8 as u32, y0() as u8 as u32, 0xf, 0, 0]);
                    *w(p, match_player::ACTION) = 0xf;
                    break 'fim;
                }
                if l43 == 0 && mode != 4 && read_i8(p + match_player::MOMENT_FLAIR) == 0 {
                    if random(10) == 0 || reaches(p, 4, ty) != 0 {
                        crate::comparison::branch("dribble no way out long ball");
                        *w(p, match_player::ACTION) = 9;
                        call(FN_MATCH_PLAYER_PLAY_BALL_INTO_AREA, p, [read_i8(m + engine::BALL_HEIGHT) as u8 as u32, 0, 0, 0, 0]);
                    } else {
                        crate::comparison::branch("dribble no way out challenges");
                        call(FN_MATCH_PLAYER_HOLD_BALL, p, [0; 5]);
                        *w(p, match_player::ACTION) = 0x10;
                    }
                    break 'fim;
                }
                let nx = cx.clamp(0, 8);
                let ny = cy.clamp(0, 0xb);
                if (side == 0 && nx >= x0()) || (side == 1 && nx <= x0()) {
                    aim_set_facing_direction(p, x0(), y0(), ((nx as i32 + 4) / 2) as i8, ((ny as i32 / 2) as i8).wrapping_add(if side != 0 { 0 } else { 6 }));
                } else {
                    aim_set_facing_direction(p, x0(), y0(), nx, ny);
                }
                if read_i8(m + engine::SHOT_PENDING) != 0 {
                    narrate(call);
                }
                let a = *w(p, match_player::ACTION);
                if a == 0x12 || a == 0x11 {
                    call(FN_MATCH_PLAYER_MOVE_TO, p, [nx as u8 as u32, ny as u8 as u32, 0x12, l68 as u32, mode as u32]);
                    *w(p, match_player::ACTION) = 0x12;
                } else {
                    let q = read_usize(m + engine::ASSIST_CANDIDATE);
                    if l68 != 0 && p == read_usize(m + engine::PASS_RECEIVER) && q != 0 && *w(p, 0x29) > -0x64 && *w(q, match_player::ACTION) != 0x34 && *w(q, match_player::ACTION) != 0x37 {
                        crate::comparison::branch("dribble one-two returned");
                        *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                        *w(q, match_player::MATCH_RATING) = (*w(q, match_player::MATCH_RATING)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0x113) as i16);
                        let t = (m + engine::MOMENTUM_A) as *mut i32;
                        *t = (*t).wrapping_add(0x113);
                    }
                    call(FN_MATCH_PLAYER_MOVE_TO, p, [nx as u8 as u32, ny as u8 as u32, 0x11, l68 as u32, mode as u32]);
                    *w(p, match_player::ACTION) = 0x11;
                }
            }
        }
    }
    // the list’s destructor (0x68d040): in the comparison’s Rust copy it stays empty
    exe::drop_llist(la);
    volta
}

/// Address recorded on the tape for the pressure list in the dribble stack frame: 0 while recording the dribble with n
/// = 0x1be, else the address.
///
/// [pt] A lista da pressao na pilha da conducao entra na fita sem o endereco (o da original e o da
/// copia Rust diferem); o resto da copia (o bloco do jogador, 0x1be bytes) e conferido. Vale nas
/// duas fases da comparacao (na da copia Rust a fita ja foi tirada).
pub(crate) fn taped_pressure_list_addr(l: usize, n: u32) -> usize {
    if RECORD_BALL_CARRY.load(std::sync::atomic::Ordering::Relaxed) && n == 0x1be {
        0
    } else {
        l
    }
}

/// Rust-mode dispatcher for dribble calls (pass, long ball, tackle, move, shot with list, list copy), else as the
/// tackle.
///
/// [pt] Modo Rust: as chamadas da conducao vao para o jogo (passe, lancamento, dividida e movimento pelos
/// proprios desvios; o chute com a lista: a cabeca e o endereco).
pub(crate) unsafe fn dispatch_dribble_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        0x6fa250 | 0x6f09f0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(routine);
            f(this, a[0], a[1], a[2], a[3], a[4])
        }
        0x6fa490 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(routine);
            f(this, a[0], a[1]);
            0
        }
        0x70ec80 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(routine);
            f(this, a[0]);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(routine);
            let l = a[4] as usize;
            f(this, a[0], a[1], a[2], a[3], if l != 0 { *(l as *const u32) } else { 0 }, l as u32);
            0
        }
        0x68d0d0 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(routine);
            f(this, a[0] as usize, a[1]);
            0
        }
        _ => dispatch_tackle_calls(routine, this, a),
    }
}

/// the decision of the player on the ball. Set pieces: corners, throw-in and free kicks pass (0x6c6a30 picks the
/// team-mate, otherwise a drawn cell); penalty and free kick near goal shoot (`exe::match_player_get_shot_score` + `shoot`). In play, shot, pass
/// and dribble scores from the distance to goal, the pressure (0x6bcbd0, with list), the score, the keeper, the
/// position's marks and the clock; long ball (0x6fa490), dividida (0x70ec80), shot, pass (0x6fa250) or dribble
/// (`player_move`) to `exe::cm3_match_engine_choose_move_cell`'s cell. Mode 4: the keeper in the shoot-out (returns 1 when he stays). Determinized original.
/// Compared through the tape
/// Original: `dribble` (0x7109f0, match_pl.cpp (13 KB), thiscall, ret 4).
///
/// [pt] { 0x7109f0 } (ret 4, match_pl.cpp): ver `dribble_injected`.
pub(crate) unsafe extern "thiscall" fn dribble(p: usize, mode: u32) -> u32 {
    dribble_injected(p, mode as u8, &mut |r, t, a| dispatch_dribble_calls(r, t, a)) as u32
}

/// Comparer for `dribble`: original vs Rust dribble over a call tape, nesting the pass/long ball/tackle ports; registers
/// and returns the result.
pub(crate) unsafe fn compare_dribble_inner(p: usize, mode: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DRIBBLE);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let cond = RECORD_BALL_CARRY.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = compare_com_fita(
        reg,
        || {
            zero_stack_below();
            o(p, mode) & 0xff
        },
        |chamar| {
            dribble_injected(p, mode as u8, &mut |rotina, this, args| match rotina {
                // pass, long ball and challenge: their units go on the dribbling tape
                0x6fa250 => {
                    pass_ball_injected(this, args[0] as u8, args[1] as usize, args[3], args[4], &mut |r, t, a| {
                        let mask = match r {
                            0x6fad40 => [0xff, 0xff, 0xff, u32::MAX, u32::MAX],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x7103e0 => [0; 5],
                            _ => [u32::MAX; 5],
                        };
                        chamar(r, t, a, mask)
                    });
                    0
                }
                0x6fa490 => {
                    long_ball_injected(this, args[0] as i8, args[1] as u8, &mut |r, t, a| {
                        let mask = match r {
                            0x6fad40 => [0xff, 0xff, u32::MAX, u32::MAX, 0xff],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd490 => [0xff, 0, 0, 0, 0],
                            _ => [u32::MAX; 5],
                        };
                        // the port of 0x6fad40 writes (x, y, type, receiver, column); the target row is not included
                        let a = if r == 0x6fad40 { [a[0], a[1], a[2], 0, a[3]] } else { a };
                        chamar(r, t, a, mask)
                    });
                    0
                }
                0x70ec80 => {
                    tackle_injected(this, args[0] as u8, &mut |r, t, a| {
                        let mask = match r {
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                            0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                            0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                            0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                            0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                            _ => [0xff, 0, 0, 0, 0],
                        };
                        chamar(r, t, a, mask)
                    });
                    0
                }
                0x68d0d0 => chamar(rotina, taped_pressure_list_addr(this, args[1]), args, [u32::MAX, u32::MAX, 0, 0, 0]),
                _ => {
                    let mask = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6f09f0 => [0xff, 0xff, 0xffff, 0xff, 0xff],
                        // the head of the pressure list: only the Rust-mode copy fills it
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        _ => [u32::MAX; 5],
                    };
                    chamar(rotina, this, args, mask)
                }
            }) as u32
        },
    );
    RECORD_BALL_CARRY.store(cond, Relaxed);
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia && r == rr, || format!("dribble(modo {}) original={} rust={} {} chamadas: {}", mode as u8, r, rr, k, tape_detail()));
    r
}

/// Base pass rating adjusted by position markers (cases 6..=8 and 0x5e of `execute_pass`): byline bonuses depending on human
/// club, else a third of the roll off.
///
/// [pt] A nota base do passe pelos marcadores da posicao (casos 6..=8 e 0x5e de 0x6fad40): na linha de
/// fundo, com as marcas 0x880 nas duas tabelas (e 0x38 + a marca 4 da posicao), +0x4b/+0x3e/+0x2d
/// conforme o clube humano (0x600a50); senao +0x19 no fundo ou o desconto de um terco do sorteio.
pub(crate) unsafe fn pass_rating_by_position(p: usize, base: i16, l20: i32, l14: i32) -> i16 {
    let m = *((p + match_player::ENGINE) as *const usize);
    let ta = || if m != 0 { table_a(m, p) } else { 0 };
    let tb = || if m != 0 { table_b(m, p) } else { 0 };
    let club = *((p + match_player::CLUB) as *const usize);
    let dirigido = || club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
    let y = read_i8(p + match_player::CELL_Y);
    let marks = || position_marks(m + engine::TACTICS + read_i8(p + match_player::SIDE) as isize as usize * 0x18e3, read_i8(p + match_player::POSITION));
    let bottom = y == 0 || y == 0xb;
    let w = base.wrapping_sub(l20 as i16);
    if bottom && ta() & 0x880 != 0 && tb() & 0x880 != 0 && ta() & 0x38 != 0 && marks() & 0x4_0000_0000 != 0 {
        return w.wrapping_add(if dirigido() { 0x4b } else { 0x3e });
    }
    if bottom && ta() & 0x880 != 0 && tb() & 0x880 != 0 {
        return w.wrapping_add(if dirigido() { 0x3e } else { 0x2d });
    }
    let side = read_i8(p + match_player::SIDE);
    if bottom || (((side == 1 && y < 3) || (side == 0 && y > 8)) && ta() & 0x880 != 0) {
        return w.wrapping_add(0x19);
    }
    base.wrapping_sub((l14 as i16 as i32 / 3) as i16).wrapping_sub(l20 as i16)
}

/// The challenger took the ball during the pass execution (`execute_pass`, from 0x6fc0e5 to 0x6fc988).
///
/// The clean contest counts for him (+0x198 = incident, +4, -0x3b/-0x46 in +0x35), the commentary (in his area, 1 in 2:
/// the foul's and 0x20e8/0x20ea/0x20eb; otherwise by incident, with the goalkeeper's save 0x70d430 on incident 1), he
/// controls (`take_ball`, +0x19c = rating - 400 in -200..=200), the ball spills (`exe::cm3_match_engine_move_ball_to_cell`) below the incident's limit, or
/// the success average is commentated; his gains and the passer's +0x2b.
///
/// [pt] O desafiante tirou a bola na execucao do passe (0x6fad40, de 0x6fc0e5 a 0x6fc988): a disputa
/// limpa conta para ele (+0x198 = lance, +4, -0x3b/-0x46 em +0x35), a narracao (na area dele, 1 em
/// 2: a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa do goleiro 0x70d430 no lance
/// 1), ele domina (0x6f0630, +0x19c = nota - 400 em -200..=200), a bola espirra (0x6cd770) abaixo do
/// limite do lance ou a media de acertos e narrada; os ganhos dele e o +0x2b do passador.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn pass_intercepted_injected(m: usize, p: usize, e: usize, lance: i8, rating: i16, a: u8, advantage: u8, penalty: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let zero = |q: usize| (read_i8(q + match_player::SIDE) == 0) as u32;
    // a disputa limpa
    *w(e, match_player::ACTION) = lance as i16;
    *b(e, 4) = (*b(e, 4)).wrapping_add(1);
    let t = table_a(m, e);
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(if t & 0x30 != 0 && table_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
    let narrated = (special_code(lance) != 0 || advantage != 0 || random(*w(e, 0x31) as i32) < -10)
        && is_in_attacked_box((read_i8(e + match_player::SIDE) != 1) as i8, read_i8(e + match_player::CELL_X), read_i8(e + match_player::CELL_Y))
        && random(2) == 0;
    if narrated {
        crate::comparison::branch("pass tackle narrated in the box");
        narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
        let code = match random(0x32) {
            1 => 0x20ea,
            0 | 2..=5 => 0x20eb,
            _ => 0x20e8,
        };
        narrate(call, code, u32::MAX, zero(e), u32::MAX);
    } else {
        let code = match lance {
            1 => {
                crate::comparison::branch("pass goalkeeper tackle");
                if save_shot_injected(e, (rating as i32 - 200) as i16, &mut |rotina, this, x| {
                    if rotina == 0x6d6f70 {
                        call(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                    } else {
                        call(rotina, this, x)
                    }
                }) == 0
                {
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
            narrate(call, code, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
        }
    }
    call(FN_MATCH_PLAYER_TAKE_POSSESSION, e, [0; 5]);
    *w(e, match_player::BALL_CONTROL) = ((rating as i32 - 400) as i16).clamp(-200, 200);
    let mut controlled = false;
    let limit = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, match_player::BALL_CONTROL) < limit && read_i8(m + engine::BALL_HEIGHT) != 4 && read_i8(m + engine::BALL_HEIGHT) != -1 {
        crate::comparison::branch("pass ball breaks loose");
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(p + match_player::CELL_X) as u32), (read_u8(p + match_player::CELL_Y) as u32), 1 | zero(p) << 8, 0, 0]);
        if read_i8(m + engine::RESTART_TYPE) == 0 {
            let mut k = 3;
            k -= (read_i8(e + match_player::MOMENT_WORK_RATE) as i32 > random(10)) as i32;
            k -= (read_i8(e + match_player::MOMENT_AGILITY) as i32 > random(10)) as i32;
            *b(e, match_player::BUSY) = random(k) as u8;
            controlled = true;
        } else if penalty {
            call(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, p, [0; 5]);
        }
    } else if read_i8(m + engine::BALL_HEIGHT) < 4 {
        if read_i8(m + engine::BALL_HEIGHT) > -1 {
            *b(m, engine::BALL_HEIGHT) = 0;
        }
        let (x, y) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
        if read_i8(e + match_player::SIDE) == 0 && read_i8(m + engine::BALL_HEIGHT) != -1 {
            aim_set_facing_direction(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            aim_set_facing_direction(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
        let minute = (*w(m, engine::CLOCK) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let average = if read_i8(e + contas.0) != 0 { ((read_u8(e + contas.1) as u32) as f64 / (read_u8(e + contas.0) as u32) as f64) as f32 } else { 0.0 };
        if minute > (random(0x14) as u8).wrapping_add(sorteio) as i8 {
            let me = read_usize(e + match_player::ENGINE);
            let goals = read_i8(me + (if read_i8(e + match_player::SIDE) != 0 { 0xf5bd } else { 0xf5f3 }));
            let attempts = if lance == 0x15 { (read_u8(e + match_player::CHALLENGES_WON) as u32) > 3 && read_i8(e + 9) != 0 } else { (read_u8(e + match_player::KEY_TACKLES) as u32) > 1 };
            if goals == 0 && average as f64 > read_f64(rdata::F64_0_725) && attempts && *w(e, match_player::MATCH_RATING) > 0x1a5e && read_i8(e + match_player::TACKLE_RATE_NARRATED) == 0 {
                crate::comparison::branch("pass success average");
                narrate(call, if lance == 0x15 { 0x21ce } else { 0x21cd }, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
                *b(e, match_player::TACKLE_RATE_NARRATED) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = read_usize(e + match_player::ENGINE);
    let t = || if me != 0 { table_a(me, e) } else { 0 };
    let gain: i16 = if controlled { 0x78 } else { 0xc8 };
    let far = t() & 4 == 0 && t() & 2 == 0;
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_add(if far { gain << 1 } else { gain });
    *b(p, match_player::BUSY) = (*b(p, match_player::BUSY)).wrapping_add(a);
    let target = if read_i8(e + match_player::SIDE) != 0 { 0xb } else { 0 };
    let near = !(distance_table_cell_yx(read_i8(e + match_player::CELL_Y) as i32 - target, read_i8(e + match_player::CELL_X) as i32 - 4) as f64 > read_f64(rdata::F64_1));
    if (near || behind_the_extreme(me, e) != 0) && *w(p, 0x29) > 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if controlled && random(7) == 0 {
        narrate(call, 0x1f41, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
    }
}

/// Challenger's foul during pass execution (`execute_pass`): like punish_move_foul_injected but set piece to the passer first
/// and event 0x82 independent of the card.
///
/// [pt] A falta do desafiante na execucao do passe (0x6fad40, de 0x6fb678 a 0x6fc08f): como
/// `punish_move_foul_injected`, mas a bola parada fica com o passador antes das narracoes, o penalti
/// sorteia pela pessoa (+0x5a, sem o +6) e 10, e o evento 0x82 nao depende do cartao.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn pass_foul_injected(m: usize, e: usize, p: usize, mut kind: u8, card: u8, grav: i8, risky: i8, advantage: u8, lance: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| (read_u8(q + match_player::COMMENTARY_ID) as u32) | (read_u8(q + match_player::SIDE) as u32) << 8;
    let zero = |q: usize| (read_i8(q + match_player::SIDE) == 0) as u32;
    let side = read_i8(p + match_player::SIDE);
    let mut weight = 0f32;
    engine_punish_move(m, e, p, &mut kind, card as i8, grav, risky as u8, &mut weight, advantage);
    narrate(call, foul_commentary_code(lance), (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), par(p));
    let colour: i32 = match kind {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > random(7) + 2 { 0x210a } else { -1 }
        }
    };
    // event 0x82 (card 4/5 for reasons 5, 10, 11 and the player’s history)
    let v = ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS + 0), read_usize(e + match_player::PLAYER), 0xb, 2, 6.5)) as i8;
    let base = *(G_STAFF_COUNT as *const i32);
    let list = if m != 0 { read_usize(m + engine::EVENT_LIST) } else { 0 };
    if list != 0
        && (kind == 5 || kind == 4)
        && (card == 0xa || card == 5 || card == 0xb)
        && read_usize(e + match_player::STAFF) != 0
        && *(read_usize(e + match_player::STAFF) as *const i32) < base
        && (read_usize(p + match_player::STAFF) == 0 || *(read_usize(p + match_player::STAFF) as *const i32) < base)
        && read_usize(e + match_player::CLUB) != 0
        && exe::human_manager_is_human_club(read_usize(e + match_player::CLUB), 0, 1, 0, 0) == 0
        && (read_i8(read_usize(e + match_player::STAFF) + 0x5b) as i32) < v as i32 + 10
    {
        let person = read_usize(e + match_player::PLAYER);
        let k = 0xfu8.wrapping_sub(v as u8);
        if read_i8(person + player_sheet::AGGRESSION) > (random(5) as u8).wrapping_add(k) as i8
            && read_i8(person + player_sheet::DIRTINESS) > (random(5) as u8).wrapping_add(k) as i8
            && *((person + player_sheet::CURRENT_REPUTATION) as *const i16) > (random(0xdac) + 3000) as i16
            && v as i32 > random(3) + 1
        {
            crate::comparison::branch("pass event 0x82");
            let mut ev = [0u8; 0x19];
            ev[0..4].copy_from_slice(&*(read_usize(e + match_player::STAFF) as *const [u8; 4]));
            ev[4..8].copy_from_slice(&*(read_usize(e + match_player::CLUB) as *const [u8; 4]));
            ev[0xd] = side as u8;
            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
            ev[0x13..0x15].copy_from_slice(&0x82u16.to_le_bytes());
            ev[0x15..0x19].copy_from_slice(&*(read_usize(p + match_player::STAFF) as *const [u8; 4]));
            request_buffers(&[(ev.as_ptr() as usize, 0x19)], &[]);
            call(FN_LLIST_ADD_TAIL, list, [0, 0x19, 0, 0, 0]);
        }
    }
    let sound: u32;
    if kind == 7 {
        sound = 0x1f79;
        *b(m, engine::RESTART_TYPE) = 7;
    } else if is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) {
        crate::comparison::branch("pass penalty");
        if colour as i16 == -1 {
            match random(4) {
                0 => narrate(call, 0x20e9, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX),
                1 => narrate(call, 0x20ec, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX),
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
        narrate(call, 0x1f7a, u32::MAX, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
    }
    let sent_off = matches!(kind, 3..=6);
    if colour as i16 > -1 {
        if (lance == 0x2c || lance == 0x2f) && advantage != 0 {
            narrate(call, if lance == 0x2f { 0x21da } else { 0x21d9 }, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), par(e));
            if sent_off || sound == 0x1f7a {
                *b(e, match_player::SENT_OFF) = 1;
            }
        }
        if random(4) == 0 {
            let x = random(grav as i32);
            if x > random(0x14) {
                let t = read_usize(m + engine::MATCH);
                let club = Club(read_usize(t + (if read_i8(e + match_player::SIDE) == 0 { 0x20 } else { 0x1c })));
                let k = club.nation().addr();
                if k != 0 && read_i8(k + nation::FIRST_LANGUAGE) == 8 {
                    narrate(call, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrate(call, 0x2109, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        }
        narrate(call, colour as u32, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), u32::MAX);
        if sent_off {
            crate::comparison::branch("pass sending off");
            player_left_pitch_with(m, e, 0, call);
        }
    }
    if sound != 0x1f7a {
        let c = choose_taker(m, side, read_i8(m + engine::RESTART_TYPE), 0);
        if c == 0 {
            narrate(call, sound, 0xff, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
        } else {
            if kind == 7 {
                narrate(call, 0x21e2, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            } else {
                narrate(call, sound, 0xff, (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            }
            let far = (side == 1 && read_i8(p + match_player::CELL_Y) < 4) || (side == 0 && read_i8(p + match_player::CELL_Y) > 7);
            if sound == 0x1f79 && advantage == 0 && far {
                if kind == 7 {
                    if !is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) {
                        narrate(call, 0x21e1, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                    }
                } else if !(2..=6).contains(&read_i8(p + match_player::CELL_X)) {
                    narrate(call, 0x21df, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                } else {
                    narrate(call, 0x21e0, (read_u8(c + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
                }
            }
        }
    }
    if special_code(lance) == 0 {
        let extra = sent_off || sound == 0x1f7a;
        if advantage != 0 {
            narrate(call, 0x20dd, (read_u8(p + match_player::COMMENTARY_ID) as u32), (read_u8(p + match_player::SIDE) as u32), u32::MAX);
            if kind == 4 || kind == 5 {
                *b(e, match_player::SENT_OFF) = 1;
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        } else if random(9) == 0 {
            narrate(call, 0x20df, u32::MAX, zero(p), u32::MAX);
            if random(3) == 0 {
                narrate(call, 0x20e0, u32::MAX, zero(p), u32::MAX);
            }
            if extra && random(2) == 0 {
                narrate(call, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        }
        apply_to_team(m, e, weight);
        call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, e, [0; 5]);
    }
    *w(e, match_player::MATCH_RATING) = (*w(e, match_player::MATCH_RATING)).wrapping_sub(0x19);
}

/// Picks one of seven commentary codes by the pass direction bits (0x10, 0x20, 0x40, 2, 4, 1, 8), else the default.
///
/// [pt] Os oito codigos de narracao pela direcao do passe (`bl` de 0x6fad40: 0x10 pela faixa, 0x20/0x40
/// para a ponta, 2/4 de lado, 1 para frente, 8 para tras), na ordem 0x10, 0x20, 0x40, 2, 4, 1, 8.
pub(crate) fn commentary_by_direction(bl: u8, c: [u32; 7], default: u32) -> u32 {
    for (k, f) in [0x10u8, 0x20, 0x40, 2, 4, 1, 8].iter().enumerate() {
        if bl & f != 0 {
            return c[k];
        }
    }
    default
}

/// { `execute_pass` } (ret 0x1c, match_pl.cpp, ~22 KB) with the calls that touch things outside the match engine injected:
/// executes the pass of `p` (type 6..=0x5e) from (x, y) to (tx, ty), with `receptor` (or 0) and the output flag (1 when
/// the pass does not reach the receiver); returns 1 if the ball stays in play.
///
/// The pass rating from the type and the distance; the challenger (`exe::cm3_match_engine_get_opposition_challenge`) with the foul (as in the 50-50) or the
/// tackle; without him, stamina, the inaccurate pass (the target drawn by the factor of `engine::WEATHER`), the cost at +0x35,
/// the approach commentary, the pass-type commentary (by direction, by the output and by the engine's +0x1a7), the next
/// set piece (`engine::BALL_HEIGHT`, `engine::BALL_ROLL`) and the trace (0x6ccb60): from the table, a shot (`shot_outcome`/`shot_on_goal`) or the ball in
/// transit (`engine::BALL_TRANSIT_TICKS`..`engine::TRANSIT_ORIGIN_Y`, `engine::BALL_RECEIVER`), and ball out (`exe::cm3_match_engine_ball_out_of_play`).
///
/// [pt] { 0x6fad40 } (ret 0x1c, match_pl.cpp, ~22 KB) com as chamadas que mexem fora do motor injetadas:
/// a execucao do passe de `p` (tipo 6..=0x5e) de (x, y) para (tx, ty), com `receptor` (ou 0) e a
/// saida (1 quando o passe nao chega ao receptor). A nota do passe pelo tipo e pela distancia; o
/// desafiante (0x6c87e0) com a falta (como na dividida) ou o desarme; sem ele, o folego, o passe
/// impreciso (o alvo sorteado pelo fator de +0x4756), o custo em +0x35, a narracao da aproximacao, a
/// narracao do tipo de passe (pela direcao, pela saida e por +0x1a7 do motor), a bola parada seguinte
/// (+0x8ea9, +0x8eaa) e o tracado (0x6ccb60): pela tabela, chute (0x70c040/0x70a910) ou a bola em
/// transito (+0x1c5..+0x1c9, +0x477e) e bola fora (0x6cd490). Devolve 1 se a bola segue em jogo.
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
pub(crate) unsafe fn execute_pass_injected(p: usize, x: i8, y: i8, kind_in: i8, out: *mut u8, receiver: usize, tx_in: i8, ty_in: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let m = read_usize(p + match_player::ENGINE);
    let side = read_i8(p + match_player::SIDE);
    let (mut tipo, mut tx, mut ty) = (kind_in, tx_in, ty_in);
    // commentary with seven arguments (a5 | a6 << 8 and a7 << 16 in the code)
    let narrar7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    };
    // the stamina and the distance
    let r0 = random({
        let v = 10000 - ftol(read_f32(p + match_player::STAMINA) as f64) as i16 as i32;
        v * v / 50000
    }) as i16 as i32;
    let mut d = distance_between(x, y, tx, ty);
    let k = if (read_f32(p + match_player::PASSING) as f64) < read_f32(rdata::F32_1) as f64 { read_f32(rdata::F32_1) as f64 } else { read_f32(p + match_player::PASSING) as f64 };
    let mut l14 = random(ftol(d as f64 * d as f64 * d as f64 * d as f64 * read_f32(rdata::F32_4) as f64 / k));
    if receiver == 0 {
        l14 = random(l14 as i16 as i32);
    }
    if (d as f64) < read_f64(rdata::F64_2) && tipo != 0x3d && tipo != 0x5c {
        l14 = l14.wrapping_add(ftol(read_f32(rdata::F32_250) as f64 - d as f64 * read_f32(rdata::F32_125) as f64));
    }
    let five = |v: f64| ((ftol(v * read_f32(rdata::F32_9) as f64 + read_f64(rdata::F64_2_5)) as i8 as i32 / 5) * 5) as i8;
    let mut l12: i8 = 1;
    let c5 = five(d as f64);
    if c5 as i32 >= 1 {
        l12 = c5;
    }
    if tipo != 0x36 && tipo != 0x5c && tipo != 0x3d {
        *b(m, 0x1a5 + side as isize as usize) = 1;
    }
    let mut out_play = 0u16;
    classify_move(p, tipo as i16, x, y, tx, ty, (d as f64 + read_f64(rdata::F64_1)) as f32, &mut out_play);
    let l20 = random(out_play as i16 as i32);
    // the pass rating by type
    let mut l13 = false;
    let wp = |o: usize| *w(p, o);
    let mut bp: i16 = match tipo {
        6..=8 => pass_rating_by_position(p, wp(0x11a + tipo as usize * 2), l20, l14),
        0x5e => pass_rating_by_position(p, wp(0x12a), l20, l14),
        0x34 | 0x37 => {
            l13 = true;
            wp(0x182).wrapping_sub((l14 as i16 as i32 / 5) as i16).wrapping_add(300)
        }
        0x36 => {
            l13 = true;
            wp(0x186).wrapping_sub((l14 as i16).wrapping_add(l14 as i16))
        }
        0x5a => {
            l13 = true;
            wp(0x156).wrapping_sub(l14 as i16).wrapping_add(300)
        }
        0x5b | 0x5c => wp(0x194).wrapping_sub(l14 as i16),
        0x5d => wp(0x130).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
        _ => wp(0x11a + (tipo as isize * 2) as usize).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
    };
    if read_i8(m + engine::BALL_HEIGHT) == 5 && d as f64 > read_f64(rdata::F64_2_1) {
        let (n, c) = match read_i8(m + engine::WEATHER) {
            1 => (7, 0x9689d0),
            2 => (5, 0x96af30),
            3 => (3, 0x9689d8),
            4 => (3, 0x96a678),
            _ => (0, 0),
        };
        if n != 0 && random(n) == 0 {
            crate::comparison::branch("pass penalty shootout contest");
            bp = bp.wrapping_mul(ftol(bp as f64 * read_f64(c)) as i16);
        }
    }
    let r1 = random(r0 as i16 as i32);
    // the attack rating: only the low 16 bits are read later
    let l44 = wp(0x19c).wrapping_sub(r1 as i16).wrapping_add(random(bp as i32) as i16).wrapping_add(wp(0x29));
    // o desafiante
    let (mut a, mut bb, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if read_i8(m + engine::BALL_HEIGHT) == 4 || l13 { 0 } else { opposition_challenge(m, p, side, 0x6c, &mut a, &mut bb, &mut nota, &mut lance, &mut arriscado, 0) };
    tackle_injury_with(m, p, 0x6c, e, lance as i16, 0, call);
    if e != 0 && nota as i32 >= wp(0x19c) as i32 + 200 {
        let (mut cartao, mut grav, mut vantagem) = (0u8, 0i8, 0u8);
        let kind_foul = foul_in_engine(m, e, p, lance, 0x6c, &mut cartao, &mut grav, &mut vantagem);
        if read_i8(m + engine::LIVE_MATCH) != 0
            && is_in_attacked_box(side, read_i8(e + match_player::CELL_X), read_i8(e + match_player::CELL_Y))
            && (special_code(lance) != 0 || vantagem != 0 || random(*w(e, 0x31) as i32) < -10)
            && random(3) != 0
        {
            narrar7(call, 0x2168, (read_u8(e + match_player::COMMENTARY_ID) as u32), (read_u8(e + match_player::SIDE) as u32), 0xff, 0xff, 0);
        }
        if kind_foul != 0 {
            if random(10) == 0 && arriscado == 0 {
                crate::comparison::branch("pass foul only recorded");
                record_move(m, lance as i16 as u16, e as u32, p as u32, 0, kind_foul, *b(p, match_player::CELL_X), *b(p, match_player::CELL_Y), cartao, grav as u8);
            } else {
                crate::comparison::branch("pass foul punished");
                pass_foul_injected(m, e, p, kind_foul, cartao, grav, arriscado, vantagem, lance, call);
                return 0;
            }
        }
        // the incident record skips the penalty calculation (0x6fb667 goes straight to 0x6fc0e5)
        let penalty = kind_foul == 0 && vantagem != 0 && (is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) || arriscado != 0);
        crate::comparison::branch("pass tackled");
        pass_intercepted_injected(m, p, e, lance, nota, a, vantagem, penalty, call);
        return 0;
    }
    // no challenger: the stamina of the passer
    if !is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) && random(*w(p, 0x31) as i32) < -10 {
        crate::comparison::branch("pass out of breath");
        let person = Staff(read_usize(p + match_player::STAFF));
        let mut v = read_f32(p + match_player::CONFIDENCE) as f64 * read_f32(rdata::F32_1_049999952) as f64;
        *((p + match_player::CONFIDENCE) as *mut f32) = v as f32;
        let lim = ((25 - person.temperament() as i32) * 2) as f32;
        if !(v > lim as f64) {
            v = lim as f64;
        }
        *((p + match_player::CONFIDENCE) as *mut f32) = v as f32;
        if v > read_f64(rdata::F64_100) {
            *((p + match_player::CONFIDENCE) as *mut f32) = 100.0;
        }
        narrar7(call, 0x211d, u32::MAX, (side == 0) as u32, 0xff, 0xff, 0);
        if random(person.pressure() as i32 * 20 + 10) == 0 {
            let r = random(3);
            if adjust_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                match_performance(p, 0);
            }
        }
    }
    *((m + engine::PASSER) as *mut usize) = p;
    *w(m, engine::PASS_TYPE) = tipo as i16;
    *b(p, 1) = (*b(p, 1)).wrapping_add(1);
    if random(read_i8(read_usize(p + match_player::STAFF) + 0x5a) as i32 * 20 + 10) == 0 {
        let r = random(3);
        if adjust_17(p, (r as u8).wrapping_neg() as i8) != 0 {
            match_performance(p, 0);
        }
    }
    let mut l24 = 0i32;
    if read_usize(m + engine::BALL_HOLDER) != 0 {
        l24 = read_i8(m + engine::SHOT_PENDING) as i32;
        *((m + engine::MOMENTUM_B) as *mut usize) = 0;
        *b(m, engine::POSSESSION_SIDE) = 0xff;
        *((m + engine::BALL_HOLDER) as *mut usize) = 0;
        *b(m, engine::SHOT_PENDING) = 0;
        *w(p, 0x29) = 0;
        *w(p, match_player::BALL_CONTROL) = 0;
    }
    *w(m, engine::PASS_POWER) = l44.wrapping_sub(nota);
    *b(m, engine::PASS_TARGET_X) = x as u8;
    *b(m, engine::PASS_TARGET_Y) = y as u8;
    // the pass accuracy
    let mut factor = (read_i8(m + engine::BALL_HEIGHT) as f64 * read_f64(rdata::F64_0_3) + read_f64(rdata::F64_1)) as f32;
    let c = match read_i8(m + engine::WEATHER) {
        1 => 0x96d7bc,
        2 => 0x96d3d4,
        3 => 0x96d7b8,
        4 => 0x96d7b4,
        _ => 0,
    };
    if c != 0 {
        factor = (factor as f64 * read_f32(c) as f64) as f32;
    }
    let r = random(ftol(factor as f64 * factor as f64 * read_f32(rdata::F32_10) as f64));
    if (l44 as i32 - nota as i32) < r {
        crate::comparison::branch("pass inaccurate");
        let e8 = ftol((factor as f64 * d as f64 + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5)) as i8 as i32;
        let s1 = random(e8);
        let s2 = random(e8);
        let nx = (tx as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8;
        let ny = if m != 0 && table_a(m, p) == 1 && random((wp(0x29) as i32 + 400) * 50) != 0 {
            ty
        } else {
            let s1 = random(e8);
            let s2 = random(e8);
            (ty as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8
        };
        if nx != tx || ny != ty {
            if receiver != 0 {
                if m != 0 && table_a(m, p) == 1 {
                    *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(0x32);
                } else if is_in_defending_half(side, y) && is_in_defending_half(side, ny) {
                    *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(0x6e);
                }
                *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 0x5e | 6) { 0xf } else { 0x64 });
            }
            ty = ny;
            tx = nx;
            d = distance_between(x, y, nx, ny);
        }
    }
    // the cost of the pass
    if receiver != 0 {
        if *out != 0 && limit(side as u8, ty) != 0 {
            *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(if m != 0 && table_a(m, p) == 1 { 0xd } else { 0x14 });
        } else {
            if m != 0 && table_a(m, p) == 1 {
                *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(0x42);
            } else if is_in_defending_half(side, y) && is_in_defending_half(side, ty) {
                *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(0x64);
            }
            *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 6) { 0x14 } else { 0x48 });
        }
    } else {
        *w(p, match_player::MATCH_RATING) = wp(0x35).wrapping_sub(0x24);
    }
    // the approach commentary (where he takes the move)
    let action = wp(0x198);
    let setpiece = matches!(tipo, 6 | 7 | 8 | 0x5e);
    if !matches!(action, 0x12 | 0x11 | 0x10 | 0xf) && tipo != 8 && !setpiece && !l13 {
        let r = random(0x14);
        if l12 as i32 >= 30 - r && read_i8(m + engine::BALL_HEIGHT) < 3 && !(m != 0 && table_a(m, p) == 1) && random(3) == 0 {
            let t = if side == 0 { distance_table_cell_yx(read_i8(p + match_player::CELL_Y) as i32 - 0xb, read_i8(p + match_player::CELL_X) as i32 - 4) } else { distance_to_point(p, 4, 0) };
            let c5 = five(t as f64);
            let n5: i8 = if (c5 as i32) < 1 { 1 } else { c5 };
            let (px, py) = (read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y));
            let bottom = (side == 0 && py > 7) || (side == 1 && py < 4);
            let recuado = (side == 0 && py < 4) || (side == 1 && py > 7);
            let corner_own = (side == 0 && px < 2) || (side == 1 && px > 6);
            let corner_other = (side == 0 && px > 6) || (side == 1 && px < 2);
            let code = if bottom {
                if corner_own {
                    Some(0x2160)
                } else if corner_other {
                    Some(0x2162)
                } else if is_in_attacked_box(side, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if recuado {
                Some(if corner_own { 0x215a } else if corner_other { 0x215c } else { 0x215b })
            } else if (side == 0 && px < 2) || (side == 1 && px > 6) {
                Some(0x215d)
            } else if corner_other {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && random(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = code {
                crate::comparison::branch("pass narrated approach");
                narrar7(call, cn, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, 0xff, 0xff, n5);
            }
        }
    }
    // the pass direction and the commentary by type
    let mut l54: i32 = -1;
    let rp = |o: usize| if receiver != 0 { (read_u8(receiver + o) as u32) } else { 0 };
    let (bl, al): (u8, u8);
    if setpiece {
        let mut l13b = 0u8;
        if (side == 0 && x < 3) || (side == 1 && x > 5) {
            l13b = 1;
        } else if (side == 1 && x < 3) || (side == 0 && x > 5) {
            l13b = 2;
        }
        let dif = ((x as i32 - 4).abs() as u8).wrapping_sub((x as i32 - tx as i32).abs() as u8) as i8;
        bl = l13b;
        al = if dif > 0 { l13b | 8 } else if dif < 0 { l13b | 4 } else { l13b };
    } else {
        let mut f = 0u8;
        if receiver != 0 && read_usize(receiver + match_player::ENGINE) != 0 && table_a(read_usize(receiver + match_player::ENGINE), receiver) == 1 {
            f = 0x80;
        }
        if (side == 0 && ty > y) || (side == 1 && ty < y) {
            f |= 1;
        } else if (side == 0 && ty < y) || (side == 1 && ty > y) {
            f |= 8;
        }
        let dy = (ty as i32 - y as i32).abs();
        if dy < 2 {
            let dx = (tx as i32 - x as i32).abs();
            if ((side == 0 && tx < x) || (side == 1 && tx > x)) && dy <= dx {
                f |= 2;
            } else if ((side == 0 && tx > x) || (side == 1 && tx < x)) && dy <= dx {
                f |= 4;
            }
        }
        if !is_in_attacked_box(side, read_i8(p + match_player::CELL_X), read_i8(p + match_player::CELL_Y)) && band((side == 0) as u8, tx, ty) != 0 {
            f |= 0x10;
        }
        let long = (side == 1 && read_i8(p + match_player::CELL_Y) < 3) || (side == 0 && read_i8(p + match_player::CELL_Y) > 8) || (side == 0 && ty as i32 > y as i32 + 3) || (side == 1 && (ty as i32) < y as i32 - 3);
        if long {
            if off_centre(tx) != 0 && ((side == 0 && tx < read_i8(p + match_player::CELL_X) && tx < 2) || (side == 1 && tx > read_i8(p + match_player::CELL_X) && tx > 6)) {
                f |= 0x20;
            } else if off_centre(tx) != 0 && ((side == 0 && tx > read_i8(p + match_player::CELL_X) && tx > 6) || (side == 1 && tx < read_i8(p + match_player::CELL_X) && tx < 2)) {
                f |= 0x40;
            }
        }
        if side == 0 && tx == 4 && ty == 0xc && (d as f64) < read_f64(rdata::F64_2) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if side == 1 && tx == 4 && ty == -1 && (d as f64) < read_f64(rdata::F64_2) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if read_i8(m + engine::BALL_HEIGHT) < 4 && tipo != 0x5d && tipo != 0x5c && tx == 4 && (ty == -1 || ty == 0xc) {
            l54 = 0x211f;
        }
        bl = f;
        al = l13 as u8;
    }
    let n = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| narrar7(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, 0xff, 0xff, l12);
    let nr = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a7: i8| narrar7(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, rp(match_player::COMMENTARY_ID), rp(match_player::SIDE), a7);
    let group = |out_v: u8, l24: i32, c: [[u32; 7]; 4]| -> [u32; 7] {
        match (out_v != 0, l24 != 0) {
            (false, false) => c[0],
            (false, true) => c[1],
            (true, false) => c[2],
            (true, true) => c[3],
        }
    };
    let of_side = |al: u8, c: [u32; 3]| if al & 8 != 0 { c[0] } else if al & 4 != 0 { c[1] } else { c[2] };
    match tipo {
        9 => {
            if (ty >= 0xc && side == 1) || (ty < 0 && side == 0) {
                n(call, if tx != 4 && random(4) != 0 { 0x210e } else { 0x211a });
            } else {
                n(call, if (exe::match_player_is_in_penalty_area(p, side as u8) as u8) == 0 { 0x2157 } else { 0x1f70 });
            }
        }
        0x3e => n(call, 0x2134),
        0x5b => {
            if (0..0xc).contains(&ty) {
                n(call, 0x1f6f);
            } else {
                n(call, if tx != 4 && random(4) != 0 { 0x210d } else { 0x2119 });
            }
        }
        0x5d => n(call, commentary_by_direction(bl, [0x20b2, 0x20b3, 0x20b4, 0x20af, 0x20b0, 0x20ae, 0x20b1], 0x1f72)),
        6 | 8 | 0x5e => {
            let code = if al & 1 != 0 {
                if l24 != 0 { of_side(al, [0x20c3, 0x20c1, 0x20bf]) } else { of_side(al, [0x20bd, 0x20bb, 0x20b9]) }
            } else if al & 2 != 0 {
                if l24 != 0 { of_side(al, [0x20c4, 0x20c2, 0x20c0]) } else { of_side(al, [0x20be, 0x20bc, 0x20ba]) }
            } else {
                0x1f73
            };
            narrar7(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, 0xff, 0xff, 0);
        }
        7 => {
            let code = if al & 1 != 0 {
                if l24 != 0 { of_side(al, [0x20cf, 0x20cd, 0x20cb]) } else { of_side(al, [0x20c9, 0x20c7, 0x20c5]) }
            } else if al & 2 != 0 {
                if l24 != 0 { of_side(al, [0x20d0, 0x20ce, 0x20cc]) } else { of_side(al, [0x20ca, 0x20c8, 0x20c6]) }
            } else {
                0x21e4
            };
            narrar7(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, 0xff, 0xff, 0);
        }
        0x5c => n(call, 0x1f71),
        0x3d => {
            let rr = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| narrar7(call, code, (read_u8(p + match_player::COMMENTARY_ID) as u32), side as u8 as u32, rp(match_player::COMMENTARY_ID), rp(match_player::SIDE), 0);
            if bl & 0x80 != 0 && bl & 1 == 0 {
                rr(call, 0x2148);
            } else if band((side == 0) as u8, tx, ty) != 0 {
                rr(call, 0x21d5);
            } else if bl & 2 != 0 {
                rr(call, 0x20b6);
            } else if bl & 4 != 0 {
                rr(call, 0x20b7);
            } else if bl & 1 != 0 {
                rr(call, 0x20b5);
            } else {
                rr(call, if bl & 8 != 0 { 0x20b8 } else { 0x1f6d });
            }
        }
        0x34 => {
            if (d as f64) < read_f64(rdata::F64_3) {
                n(call, 0x2130);
            } else {
                n(call, if band((side == 0) as u8, tx, ty) == 0 { 0x2131 } else { 0x1f80 });
            }
        }
        0x5a => n(call, 0x1f7e),
        0x37 => n(call, if off_centre(tx) != 0 { 0x21bc } else { 0x1f7d }),
        0x36 => {
            if receiver != 0 {
                nr(call, 0x1f7b, l12);
            } else {
                n(call, 0x1f7c);
            }
        }
        0x1f => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(call, 0x214a, 0);
            } else {
                let c = if read_i8(m + engine::BALL_HEIGHT) == 0 {
                    group(*out, l24, [
                        [0x2026, 0x2027, 0x2028, 0x2023, 0x2024, 0x2022, 0x2025],
                        [0x202d, 0x202e, 0x202f, 0x202a, 0x202b, 0x2029, 0x202c],
                        [0x2034, 0x2035, 0x2036, 0x2031, 0x2032, 0x2030, 0x2033],
                        [0x203b, 0x203c, 0x203d, 0x2038, 0x2039, 0x2037, 0x203a],
                    ])
                } else {
                    group(*out, l24, [
                        [0x2042, 0x2043, 0x2044, 0x203f, 0x2040, 0x203e, 0x2041],
                        [0x2049, 0x204a, 0x204b, 0x2046, 0x2047, 0x2045, 0x2048],
                        [0x2050, 0x2051, 0x2052, 0x204d, 0x204e, 0x204c, 0x204f],
                        [0x2057, 0x2058, 0x2059, 0x2054, 0x2055, 0x2053, 0x2056],
                    ])
                };
                nr(call, commentary_by_direction(bl, c, 0x1f67), l12);
            }
        }
        0xb => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(call, 0x214a, 0);
            } else if (exe::match_player_is_in_penalty_area(receiver, (side == 0) as u8) as u8) != 0 && (exe::match_player_is_in_penalty_area(p, (side == 0) as u8) as u8) != 0 && (d as f64) < read_f64(rdata::F64_2) {
                nr(call, 0x2021, l12);
            } else {
                let c = group(*out, l24, [
                    [0x205e, 0x205f, 0x2060, 0x205b, 0x205c, 0x205a, 0x205d],
                    [0x2065, 0x2066, 0x2067, 0x2062, 0x2063, 0x2061, 0x2064],
                    [0x206c, 0x206d, 0x206e, 0x2069, 0x206a, 0x2068, 0x206b],
                    [0x2073, 0x2074, 0x2075, 0x2070, 0x2071, 0x206f, 0x2072],
                ]);
                nr(call, commentary_by_direction(bl, c, 0x1f69), l12);
            }
        }
        0x1e => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(call, 0x214c, 0);
            } else {
                let c = group(*out, l24, [
                    [0x2096, 0x2097, 0x2098, 0x2093, 0x2094, 0x2092, 0x2095],
                    [0x209d, 0x209e, 0x209f, 0x209a, 0x209b, 0x2099, 0x209c],
                    [0x20a4, 0x20a5, 0x20a6, 0x20a1, 0x20a2, 0x20a0, 0x20a3],
                    [0x20ab, 0x20ac, 0x20ad, 0x20a8, 0x20a9, 0x20a7, 0x20aa],
                ]);
                nr(call, commentary_by_direction(bl, c, 0x1f6a), l12);
            }
        }
        0x3c => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(call, 0x214c, 0);
            } else {
                let c = group(*out, l24, [
                    [0x207a, 0x207b, 0x207c, 0x2077, 0x2078, 0x2076, 0x2079],
                    [0x2081, 0x2082, 0x2083, 0x207e, 0x207f, 0x207d, 0x2080],
                    [0x2088, 0x2089, 0x208a, 0x2085, 0x2086, 0x2084, 0x2087],
                    [0x208f, 0x2090, 0x2091, 0x208c, 0x208d, 0x208b, 0x208e],
                ]);
                nr(call, commentary_by_direction(bl, c, 0x1f6b), l12);
            }
        }
        _ => {
            // simplification: the original stops the game (”MATCH_PLAYER::pass() - unknown pass”); the callers only use the 16 types above
            linha(&format!("motor: execute_pass: tipo desconhecido {:#x}", tipo));
            std::process::abort();
        }
    }
    if l54 as i16 > -1 {
        n(call, l54 as u32);
    }
    // the next set piece (+0x8ea9) and its time (+0x8eaa)
    let (v, t): (i8, i8) = match tipo {
        0x34 | 0x37 => {
            let v = if (d as f64) < read_f64(rdata::F64_3) && band((side == 0) as u8, tx, ty) == 0 {
                0
            } else if random(0x28) != 0 {
                4
            } else {
                random(3) as i8
            };
            (v, random(3) as i8 + 1)
        }
        0x1e | 8 => (4, random(3) as i8 + 1),
        0x3c | 0x1f | 0x20 => {
            let r = if random(3) != 0 { random(4) } else { 0 };
            (4 - r as i8, random(2) as i8 + 1)
        }
        0xb | 7 => ((random(7) == 0) as i8, random(2) as i8),
        9 | 0x3e | 0x5a => {
            let r = if random(0x23) != 0 { 0 } else { random(2) };
            (4 - r as i8, random(2) as i8 + 2)
        }
        _ => {
            let v = if random(3) != 0 { 2 - random(2) as i8 } else { 4 - random(4) as i8 };
            (v, random(2) as i8)
        }
    };
    *b(m, engine::BALL_HEIGHT) = v as u8;
    *b(m, engine::BALL_ROLL) = t as u8;
    let mut bl2 = read_i8(m + engine::BALL_HEIGHT);
    let cai = (*out != 0 && bl2 > 0 && random(3) == 0) || (bl2 < 4 && wp(0x19c) as i32 > random(200));
    if cai {
        bl2 = bl2.wrapping_sub(random(2) as i8);
        *b(m, engine::BALL_HEIGHT) = bl2 as u8;
    }
    let cy = ty.clamp(0, 0xb);
    let cx = tx.clamp(0, 8);
    *w(m, engine::BALL_DIRECTION) = by_difference(m, x, y, cx, cy) as i16;
    // 0x7003b0: the side of the pass and the possession count
    *b(m, engine::LAST_POSSESSION_SIDE) = side as u8;
    if side != -1 {
        *b(m, 0x8d46 + side as isize as usize) = (*b(m, 0x8d46 + side as isize as usize)).wrapping_add(1);
        let o = (m + engine::POSSESSION_COUNTER + (side == 0) as usize) as *mut i8;
        if *o > 0 {
            *o = (*o).wrapping_sub(10);
        }
    }
    if receiver != 0 && (tx != read_i8(receiver + match_player::CELL_X) || ty != read_i8(receiver + match_player::CELL_Y)) {
        *out = 1;
    }
    let strength = if l44 > -100 { l44 as i32 } else { -100 };
    let r = call(FN_CM3_MATCH_ENGINE_CHECK_PASS_INTERCEPTED_ON_PATH, m, [x as u8 as u32 | (y as u8 as u32) << 8 | (tx as u8 as u32) << 16 | (ty as u8 as u32) << 24, tipo as u8 as u32, strength as u32, 1, 0]) as u8;
    let away = |m: usize| !((0..9).contains(&read_i8(m + engine::BALL_X)) && (0..0xc).contains(&read_i8(m + engine::BALL_Y)));
    if r != 0 {
        if read_i8(m + engine::RESTART_TYPE) != 0 {
            return 0;
        }
        if away(m) {
            crate::comparison::branch("pass traced goes out");
            call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [(side == 0) as u32, 0, 0, 0, 0]);
        }
        return 0;
    }
    if tx == 4 && (ty == 0xc || ty == -1) {
        crate::comparison::branch("pass becomes shot");
        let contra = (ty == -1) == (side == 0);
        call(if contra { 0x70c040 } else { 0x70a910 }, p, [d.to_bits(), 0, 0, 0, 0]);
        if read_i8(m + engine::RESTART_TYPE) != 0 {
            return 0;
        }
    }
    let v = read_i8(m + engine::BALL_ROLL) as i32;
    let k = if (v as f64 + read_f64(rdata::F64_0_2)) < read_f64(rdata::F64_1) { read_f64(rdata::F64_1) } else { read_i8(m + engine::BALL_ROLL) as f64 + read_f64(rdata::F64_0_2) };
    *b(m, engine::BALL_TRANSIT_TICKS) = ftol((d as f64 + d as f64) / k + read_f64(rdata::F64_0_5)) as u8;
    *b(m, engine::TRANSIT_TARGET_X) = tx as u8;
    *b(m, engine::TRANSIT_TARGET_Y) = ty as u8;
    *b(m, engine::TRANSIT_ORIGIN_X) = *b(p, match_player::CELL_X);
    *b(m, engine::TRANSIT_ORIGIN_Y) = *b(p, match_player::CELL_Y);
    *((m + engine::BALL_RECEIVER) as *mut usize) = receiver;
    *b(m, engine::BALL_X) = tx as u8;
    *b(m, engine::BALL_Y) = ty as u8;
    if away(m) {
        crate::comparison::branch("pass in transit goes out");
        call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, m, [side as u8 as u32, 0, 0, 0, 0]);
    }
    (read_i8(m + engine::RESTART_TYPE) == 0) as u8
}

/// Unidades da fita do passe: o evento 0x82 vai para a lista montado na pilha (confere o conteudo).
pub(crate) static RECORD_PASS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True while the pass tape flag is set and a tape is recording.
pub(crate) fn is_recording_pass() -> bool {
    RECORD_PASS.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// Rust-mode dispatcher for pass calls (trace 0x6ccb60 with packed coords, shots with distance bits), else as the move.
///
/// [pt] Modo Rust: as chamadas da execucao do passe vao para o jogo (o tracado com as coordenadas num
/// dword, os chutes com a distancia em bits; o resto como na dividida e no movimento).
pub(crate) unsafe fn dispatch_pass_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        0x6ccb60 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, i8, u32, u32) -> u8 = core::mem::transmute(routine);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, (a[0] >> 16) as u8 as i8, (a[0] >> 24) as u8 as i8, a[1] as u8 as i8, a[2], a[3]) as u32
        }
        0x70c040 | 0x70a910 => {
            let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(routine);
            f(this, f32::from_bits(a[0]));
            0
        }
        _ => dispatch_move_calls(routine, this, a),
    }
}

/// `p`'s pass (type 6..=0x5e) from (x, y) to (tx, ty), with receiver and the output (1 when it does not reach him).
/// Score by type and distance; challenger with foul or dispossession; without him, stamina, inaccurate pass by
/// `engine::WEATHER`'s factor, cost in +0x35, approach and pass-type commentary, the next set piece (`engine::BALL_HEIGHT`/`engine::BALL_ROLL`) and the
/// trace (0x6ccb60): shot (`shot_outcome`/`shot_on_goal`) or ball in transit (`engine::BALL_TRANSIT_TICKS`..`engine::TRANSIT_ORIGIN_Y`, `engine::BALL_RECEIVER`). Compared through the tape,
/// with comparison inside the unit
/// Original: `execute_pass` (0x6fad40, match_pl.cpp (22 KB), thiscall, ret 0x1c).
///
/// [pt] { 0x6fad40 } (ret 0x1c, match_pl.cpp): ver `execute_pass_injected`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn execute_pass(p: usize, x: u32, y: u32, kind: u32, out: *mut u8, receiver: usize, tx: u32, ty: u32) -> u8 {
    execute_pass_injected(p, x as u8 as i8, y as u8 as i8, kind as u8 as i8, out, receiver, tx as u8 as i8, ty as u8 as i8, &mut |r, t, a| dispatch_pass_calls(r, t, a))
}

/// Comparer for `execute_pass`: original vs Rust pass execution over a call tape with masks; registers and returns the
/// result.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn compare_execute_pass_inner(p: usize, x: u32, y: u32, kind: u32, out: *mut u8, receiver: usize, tx: u32, ty: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u8, usize, u32, u32) -> u8>(IDX_EXECUTE_PASS);
    let m = *((p + match_player::ENGINE) as *const usize);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4), (out as usize, 1)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + fixture::HOME_CLUB) as *const usize), *((t + fixture::AWAY_CLUB) as *const usize)] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, Relaxed);
    let a7 = RECORD_A7.swap(true, Relaxed);
    let passe = RECORD_PASS.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = compare_com_fita(
        reg,
        || o(p, x, y, kind, out, receiver, tx, ty) as u32,
        |chamar| {
            execute_pass_injected(p, x as u8 as i8, y as u8 as i8, kind as u8 as i8, out, receiver, tx as u8 as i8, ty as u8 as i8, &mut |rotina, this, args| {
                let mask = match rotina {
                    0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                    0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                    0x6cd490 => [0xff, 0, 0, 0, 0],
                    0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                    0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                    0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                    0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                    0x6ccb60 => [u32::MAX, 0xff, u32::MAX, u32::MAX, 0],
                    0x70c040 | 0x70a910 => [u32::MAX, 0, 0, 0, 0],
                    0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                    _ => [0xff, 0, 0, 0, 0],
                };
                chamar(rotina, this, args, mask)
            }) as u32
        },
    );
    RECORD_PASS.store(passe, Relaxed);
    RECORD_A7.store(a7, Relaxed);
    RECORD_716.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia && (r as u8) == (rr as u8), || format!("execute_pass(tipo {:#x}) original={} rust={} {} chamadas: {}", kind as u8, r as u8, rr as u8, k, tape_detail()));
    r as u8
}

/// Port of `exe::match_day_process_player_incidents` (cdecl, match_day.cpp): after the match, walks the event list in [0xb67dc8] (cursor reset to the
/// start; `next_list_item` gives the next) and handles those of the two clubs of `partida` (+0x1c, +0x20) that are not out
/// (`exe::database_is_national_team`), through injected calls.
///
/// Event: +0 person, +4 club, +8 mark, +0x13 type, +0x15 other person. Type 0x82: 0x759610 with the two persons.
///
/// 0x13: with the contract (`exe::contract_manager_get_contract_for_club`), without a mark it clears 0x40 (`exe::staff_contract_clear_flags`) and 0x2000000 (`exe::staff_contract_clear_grievances`); with a mark,
/// if 0x40 is already set and +0x5b < 6, or by random(+0x5b) == 0, the news item `exe::media_create_player_media_story` and 0x2000000 (`exe::staff_contract_apply_event_flags`);
/// otherwise it sets 0x40 (`set_flags`).
///
/// 0x0a: sets 0x4000000 and, with a player sheet (+0x61), if random(+0x5d) < random(5), `has_story` or the contract with
/// more than 0x4000000, and the player sheet's +0xb > random(2000) + 4500, the news item. 0x0b: 0x7daf50.
///
/// 0x1e: by random(+0x5a) < random(10) 0x40000; 0x733a60; with `exe::human_manager_is_human_club` zero and the manager (+0xcf) with player sheet
/// +0x14 > 15, `exe::staff_add_disliked_staff` and -50 in word +0xa of the club's row in [[0xb67a44]] (rows of 0x49 bytes); otherwise -75 in
/// +0xa and +0xc (between 1 and 10000).
///
/// The others: by random(+0x5a) < random(15, 8 or 6 by age +0x18 and +0x5b) 0x40000, and the news item with the type.
///
/// [pt] { 0x6b74a0 } (cdecl, match_day.cpp): depois da partida, percorre a lista de eventos em
/// [0xb67dc8] (cursor reposto no inicio; 0x68d1b0 da o proximo) e trata os dos dois clubes da
/// `partida` (+0x1c, +0x20) que nao estao fora (0x53b900). Evento: +0 pessoa, +4 clube, +8 marca,
/// +0x13 tipo, +0x15 outra pessoa. Tipo 0x82: 0x759610 com as duas pessoas. 0x13: com o contrato
/// (0x4cf5f0), sem marca tira o 0x40 (0x8676d0) e o 0x2000000 (0x8674e0); com marca, se o 0x40 ja
/// esta e +0x5b < 6, ou por sorteio(+0x5b) == 0, a noticia 0x72c1f0 e o 0x2000000 (0x867700); senao
/// poe o 0x40 (0x867560). 0x0a: poe o 0x4000000 e, com ficha (+0x61), se sorteio(+0x5d) <
/// sorteio(5), 0x735840 ou o contrato com mais que o 0x4000000, e +0xb da ficha > sorteio(2000) +
/// 4500, a noticia. 0x0b: 0x7daf50. 0x1e: por sorteio(+0x5a) < sorteio(10) o 0x40000; 0x733a60;
/// com 0x600a50 zero e o tecnico (+0xcf) de ficha +0x14 > 15, 0x547af0 e -50 na palavra +0xa da linha
/// do clube em [[0xb67a44]] (linhas de 0x49 bytes); senao -75 em +0xa e +0xc (entre 1 e 10000).
/// Os outros: por sorteio(+0x5a) < sorteio(15, 8 ou 6 pela idade +0x18 e +0x5b) o 0x40000, e a
/// noticia com o tipo.
pub(crate) unsafe fn post_match_events_injected(partida: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    // those that touch the database go through `chamar` (in Rust mode the game routine; in comparison, the tape)
    let ch = core::cell::RefCell::new(call);
    let duas = |n: usize, a: usize, b: usize, p: usize| (ch.borrow_mut())(0x759610, n, [a as u32, b as u32, p as u32, 0, 0]);
    let remove_40 = |c: usize, f: u32| (ch.borrow_mut())(0x8676d0, c, [f, 0, 0, 0, 0]);
    let remove = |c: usize, f: u32| (ch.borrow_mut())(0x8674e0, c, [f, 0, 0, 0, 0]);
    let por_40 = |c: usize, f: u32| (ch.borrow_mut())(0x867560, c, [f, 0, 0, 0, 0]);
    let por = |c: usize, f: u32, z: u32| (ch.borrow_mut())(0x867700, c, [f, z, 0, 0, 0]);
    let news = |n: usize, p: usize, c: usize, t: u32, pa: usize, z: u32| (ch.borrow_mut())(0x72c1f0, n, [p as u32, c as u32, t, pa as u32, z]);
    let kind_0b = |p: usize, c: usize, pa: usize| (ch.borrow_mut())(0x7daf50, 0, [p as u32, c as u32, pa as u32, 0, 0]);
    let kind_1e = |n: usize, ev: usize, pa: usize| (ch.borrow_mut())(0x733a60, n, [ev as u32, pa as u32, 0, 0, 0]);
    let reserve = |t: usize, p: usize, um: u32| (ch.borrow_mut())(0x547af0, 0, [t as u32, p as u32, um, 0, 0]);
    // the queries that reach 0x68d0d0 (allocates) also go through `chamar`
    let away = |c: usize| (ch.borrow_mut())(0x53b900, 0, [c as u32, 0, 0, 0, 0]) as i32;
    let contract = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x4cf5f0, n, [p as u32, c as u32, 0, 0, 0]) as usize;
    let test_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let list = || *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    let noticias = || *(G_MEDIA as *const usize);
    let read = |p: usize| core::ptr::read_unaligned(p as *const i32);
    let person = |id: i32| (*(G_STAFF_TABLE as *const usize) as isize + id as isize * 0x6e) as usize;
    let c45 = |c: usize| core::ptr::read_unaligned((c + 0x45) as *const u32);
    // the word +o of the club’s row in [[0xb67a44]] minus `menos`, between 1 and 10000
    let adjust = |club: usize, o: usize, minus: i16| {
        let row = (**(G_MANAGER_MANAGER as *const *const usize) as isize + read(club) as isize * 0x49) as usize;
        let w = (row + o) as *mut i16;
        let v = core::ptr::read_unaligned(w).wrapping_sub(minus);
        core::ptr::write_unaligned(w, v.clamp(1, 10000));
    };
    if list() == 0 {
        return;
    }
    core::ptr::write_unaligned((list() + 0xa) as *mut u32, core::ptr::read_unaligned((list() + 2) as *const u32));
    let mut k = 0i32;
    while k < core::ptr::read_unaligned(list() as *const i16) as i32 {
        let ev = exe::llist_get_next(list());
        if ev == 0 {
            return;
        }
        k += 1;
        let club = (*(G_CLUB_TABLE as *const usize) as isize + read(ev + 4) as isize * 0x245) as usize;
        if (club != *((partida + fixture::HOME_CLUB) as *const usize) && club != *((partida + fixture::AWAY_CLUB) as *const usize)) || away(club) != 0 {
            continue;
        }
        let kind = core::ptr::read_unaligned((ev + 0x13) as *const u16);
        match kind {
            0x82 => {
                crate::comparison::branch("events 0x82");
                duas(noticias(), person(read(ev)), person(read(ev + 0x15)), partida);
            }
            0x0b => {
                crate::comparison::branch("events 0x0b");
                kind_0b(person(read(ev)), club, partida);
            }
            _ => {
                let p = person(read(ev));
                let c = contract(*(G_CONTRACT_MANAGER as *const usize), p, club);
                if c == 0 {
                    continue;
                }
                match kind {
                    0x13 if read(ev + 8) == 0 => {
                        crate::comparison::branch("events 0x13 no mark");
                        if exe::staff_contract_get_flags(c) & 0x40 != 0 {
                            remove_40(c, 0x40);
                        }
                        if c45(c) & 0x2000000 != 0 {
                            remove(c, 0x2000000);
                        }
                    }
                    0x13 => {
                        let ja = exe::staff_contract_get_flags(c) & 0x40 != 0 && read_i8(p + staff::PROFESSIONALISM) < 6;
                        if !ja && random(read_i8(p + staff::PROFESSIONALISM) as i32) != 0 {
                            crate::comparison::branch("events 0x13 mark");
                            if exe::staff_contract_get_flags(c) & 0x40 == 0 {
                                por_40(c, 0x40);
                            }
                        } else {
                            crate::comparison::branch("events 0x13 news");
                            news(noticias(), p, club, 0x13, partida, 0);
                            if c45(c) & 0x2000000 == 0 {
                                por(c, 0x2000000, 0);
                            }
                        }
                    }
                    0x0a => {
                        crate::comparison::branch("events 0x0a");
                        por(c, 0x4000000, 0);
                        if *((p + staff::PLAYER) as *const usize) == 0 {
                            continue;
                        }
                        let a = random(read_i8(p + staff::TEMPERAMENT) as i32);
                        let b = random(5);
                        if a < b || exe::media_has_story_for_player(noticias(), p) != 0 || c45(c) != 0x4000000 {
                            let r = random(2000) + 0x1194;
                            let sheet = *((p + staff::PLAYER) as *const usize);
                            if core::ptr::read_unaligned((sheet + player_sheet::CURRENT_REPUTATION) as *const i16) as i32 > r {
                                crate::comparison::branch("events 0x0a news");
                                news(noticias(), p, club, 0xa, partida, 0);
                            }
                        }
                    }
                    0x1e => {
                        crate::comparison::branch("events 0x1e");
                        let a = random(read_i8(p + staff::PRESSURE) as i32);
                        if a < random(10) {
                            por(c, 0x40000, 0);
                        }
                        kind_1e(noticias(), ev, partida);
                        let r = test_600a50(club, 1, 1, 0, 0);
                        let t = *((club + club::MANAGER) as *const usize);
                        if r == 0 && t != 0 && *((t + staff::NON_PLAYER) as *const usize) != 0 && read_i8(*((t + staff::NON_PLAYER) as *const usize) + 0x14) > 0xf {
                            crate::comparison::branch("events 0x1e manager");
                            reserve(t, person(read(ev)), 1);
                            if club != 0 {
                                adjust(club, 0xa, 0x32);
                            }
                        } else if club != 0 {
                            crate::comparison::branch("events 0x1e no manager");
                            adjust(club, 0xa, 0x4b);
                            adjust(club, 0xc, 0x4b);
                        }
                    }
                    _ => {
                        crate::comparison::branch("events others");
                        let age = read_i8(p + staff::YEAR_OF_BIRTH);
                        let a = random(read_i8(p + staff::PRESSURE) as i32);
                        let b = if age < 0x18 && read_i8(p + staff::PROFESSIONALISM) < 0xf {
                            random(0xf)
                        } else if age < 0x20 {
                            random(8)
                        } else {
                            random(6)
                        };
                        if a < b {
                            por(c, 0x40000, 0);
                        }
                        news(noticias(), p, club, kind as u32, partida, 0);
                    }
                }
            }
        }
    }
}

/// A player challenges for the ball (tackle) with strength `forca`, height `altura`, style `estilo` and `forte`: rolls
/// the contest, possible foul and referee decision, and the outcome (win, ball deflects); calls play_with with the
/// game's real calls. Returns its result.
///
/// [pt] { 0x70dfd0 } (ret 0x10, match_pl.cpp): ver `play_with`.
pub(crate) unsafe extern "thiscall" fn play(j: usize, strength: u32, height: u32, style: u32, strong: u32) -> u8 {
    play_with(j, strength as i16, f32::from_bits(height), style as i8, strong as u8, &mut |rotina, this, a| dispatch_tackle_calls(rotina, this, a))
}

/// `0x6f5810` (ret 4, match_pl.cpp): recovers the match player's stamina (+0x4d) or, with `teto`, sets its ceiling
/// (+0x51), without random draws.
///
/// Without player data (+0x6d) nothing changes. The factor is 1, or 1.4 − 0.0004 × injury (`exe::injury_get_fatigue`) above 1000.
///
/// Without `teto`: on the pitch (0x6c38c0 = 1) it adds 10 × (the player sheet's natural fitness +0x34 + 15) × factor;
/// otherwise, off the bench (+0x19 >= 0), it adds 12 × fitness + 325 (with +0x1f = −1) or the same 10 × (fitness + 15);
/// stamina stops at 10000.
///
/// With `teto`: +0x51 = stamina + (12 × fitness + 85) × factor, with the base multiplied by 5.56 × +0x20 − 255.76 when
/// +0x20 (or +0x21 if the former is −1) is between 46 and 64; without both, on the bench the ceiling is the stamina.
/// Out of play (+0x1c zero) and with a person, if the injury (`get_condition`, up to 6500) exceeds the ceiling, the ceiling
/// becomes 6500.
///
/// Finally the ceiling adds +0x59 and stops at 10000.
///
/// [pt] { 0x6f5810 } (ret 4, match_pl.cpp): recupera o folego (+0x4d) e o teto dele (+0x51), sem sortear.
/// Sem ficha (+0x6d) nada muda. O fator e 1, ou 1,4 - 0,0004 x lesao (0x62c3d0) acima de 1000.
/// Sem `teto`: em campo (0x6c38c0 = 1) soma 10 x (ritmo +0x34 da ficha + 15) x fator; senao, fora
/// do banco (+0x19 >= 0), soma 12 x ritmo + 325 (com +0x1f = -1) ou o mesmo 10 x (ritmo + 15); o
/// folego para em 10000. Com `teto`: +0x51 = folego + (12 x ritmo + 85) x fator, com a base vezes
/// 5,56 x +0x20 - 255,76 quando +0x20 (ou +0x21 se aquele for -1) fica entre 46 e 64; sem os dois,
/// no banco o teto e o folego. Fora de jogo (+0x1c zero) e com pessoa, se a lesao (0x62c590,
/// ate 6500) passar do teto, o teto vira 6500. Por fim o teto soma +0x59 e para em 10000.
pub(crate) unsafe extern "thiscall" fn recover_stamina(j: usize, ceiling: u8) {
    let injury: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_INJURY_GET_FATIGUE);
    let f = |o: usize| (j + o) as *mut f32;
    let sheet = *((j + match_player::PLAYER) as *const usize);
    if sheet == 0 {
        return;
    }
    let injuries = *((G_INJURY) as *const usize);
    let person = *((j + match_player::STAFF) as *const usize);
    let mut factor = 1.0f64;
    let v = injury(injuries, *(person as *const u32));
    if v > 1000 {
        factor = read_f64(rdata::F64_1_4) - v as f64 * read_f64(rdata::F64_0_0004);
    }
    let ritmo = read_i8(sheet + player_sheet::NATURAL_FITNESS) as i32;
    if ceiling == 0 {
        let m = *((j + match_player::ENGINE) as *const usize);
        let base = if m != 0 && exe::cm3_match_engine_get_player_position(m, j) == 1 {
            Some((ritmo + 15) * 10)
        } else if read_i8(j + match_player::POSITION) < 0 {
            None
        } else if read_i8(j + match_player::MINUTE_ON) == -1 {
            Some(ritmo * 12 + 0x145)
        } else {
            Some((ritmo + 15) * 10)
        };
        if let Some(base) = base {
            let x = base as f64 * factor + *f(match_player::STAMINA) as f64;
            *f(match_player::STAMINA) = x as f32;
            if x > read_f64(rdata::F64_10000) {
                *f(match_player::STAMINA) = 10000.0;
            }
        }
    } else {
        let mark = if read_i8(j + match_player::MINUTE_OFF) >= 0 { read_i8(j + match_player::MINUTE_OFF) } else { read_i8(j + match_player::MINUTE_LEFT_PITCH) };
        let k = ritmo * 12 + 0x55;
        if mark < 0 && read_i8(j + match_player::POSITION) < 0 {
            *f(match_player::STAMINA_CEILING) = *f(match_player::STAMINA);
        } else {
            let v = if mark > 0x2d && mark < 0x41 { (mark as f64 * read_f64(rdata::F64_5_56) - read_f64(rdata::F64_255_76)) * k as f64 } else { k as f64 };
            *f(match_player::STAMINA_CEILING) = (v * factor + *f(match_player::STAMINA) as f64) as f32;
            if read_i8(j + match_player::INJURY_FORCES_OFF) == 0 && person != 0 {
                let l = exe::injury_get_condition(injuries, person, 0).min(0x1964);
                if l as f32 > *f(match_player::STAMINA_CEILING) {
                    *f(match_player::STAMINA_CEILING) = 6500.0;
                }
            }
        }
    }
    let x = *f(match_player::STAMINA_BOOST) as f64 + *f(match_player::STAMINA_CEILING) as f64;
    *f(match_player::STAMINA_CEILING) = x as f32;
    if x > read_f64(rdata::F64_10000) {
        *f(match_player::STAMINA_CEILING) = 10000.0;
    }
}

/// { 0x70d430 } (ret 4, match_pl.cpp) with the calls that touch the game injected (commentary 0x6d6f70, deflected ball
/// `exe::cm3_match_engine_move_ball_to_cell`): the goalkeeper tries to save the shot of strength `forca`; returns 1 if not saved, 0 if saved.
///
/// Above 1000, or if random draw(player sheet +0x21 + +0xa5) is not below random draw(20), he does not save (0x1fe2)
/// and returns 1. Otherwise he saves and returns 0: with +0x106 above random draw(8) he parries (0x1fe3, +0x2b adds
/// random draw(2), the ball deflects 3 squares), otherwise he blocks (0x216f, +0x2b adds random draw(3), 1 square). The
/// ball stays in play (inside) with random draw(10), otherwise it may go out; the goalkeeper is the goal-line shooter
/// only if random draw(200) gives zero.
///
/// When blocking, he tires (+0x35 − 250) and, with random draw(30 × +0x5a of player sheet +0x69 + 20) zero, he loses
/// random draw(3) at +0x17 (`exe::match_player_adjust_morale`) and his performance is recomputed (`exe::match_player_calc_ratings`).
///
/// [pt] { 0x70d430 } (ret 4, match_pl.cpp) com as chamadas que mexem no jogo injetadas (comentarios
/// 0x6d6f70, bola desviada 0x6cd770): o goleiro tenta defender o chute de `forca`. Acima de 1000, ou
/// se sorteio(ficha +0x21 + +0xa5) nao ficar abaixo de sorteio(20), nao defende (0x1fe2) e devolve 1.
/// Senao defende e devolve 0: com +0x106 acima de sorteio(8) espalma (0x1fe3, +0x2b soma sorteio(2),
/// a bola espirra 3 casas), senao rebate (0x216f, +0x2b soma sorteio(3), 1 casa). A bola fica em jogo
/// (dentro) com sorteio(10), senao pode sair; o goleiro so e o chutador da linha de fundo se
/// sorteio(200) der zero. Rebatendo, cansa (+0x35 - 250) e, com sorteio(30 x +0x5a da ficha +0x69 +
/// 20) zero, perde sorteio(3) em +0x17 (0x7172d0) e o desempenho e refeito (0x6eecb0).
pub(crate) unsafe fn save_shot_injected(j: usize, strength: i16, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u32 {
    let m = *((j + match_player::ENGINE) as *const usize);
    let dir = read_i8(j + match_player::SIDE);
    let narra = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| {
        c(0x6d6f70, *((m + engine::COMMENTATOR) as *const usize), [code, m as u32, read_i8(j + match_player::COMMENTARY_ID) as u8 as u32, dir as u8 as u32, u32::MAX]);
    };
    let sheet = *((j + match_player::PLAYER) as *const usize);
    let pega = strength <= 1000 && {
        let n = ftol(read_i8(sheet + player_sheet::CONSISTENCY) as f64 + *((j + match_player::HANDLING) as *const f32) as f64);
        random(n) < random(0x14)
    };
    if !pega {
        narra(call, 0x1fe2);
        return 1;
    }
    let espalma = read_i8(j + match_player::MOMENT_DECISIONS) as i32 > random(8);
    narra(call, if espalma { 0x1fe3 } else { 0x216f });
    let b2 = (j + match_player::BUSY) as *mut i8;
    *b2 = (*b2).wrapping_add(random(if espalma { 2 } else { 3 }) as i8);
    let (dentro, chutador) = if random(10) != 0 { (1u32, 0usize) } else { (0, if random(0xc8) != 0 { 0 } else { j }) };
    let spread = if espalma { 3 } else { 1 };
    call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [read_i8(j + match_player::CELL_X) as u8 as u32, read_i8(j + match_player::CELL_Y) as u8 as u32, spread | (dir as u8 as u32) << 8, dentro, chutador as u32]);
    if espalma {
        return 0;
    }
    let w35 = (j + match_player::MATCH_RATING) as *mut u16;
    *w35 = (*w35).wrapping_add(0xff06);
    let v = *((*((j + match_player::STAFF) as *const usize) + 0x5a) as *const i8) as i32;
    if random(v * 30 + 0x14) == 0 {
        let r = random(3) as i8;
        if exe::match_player_adjust_morale(j, r.wrapping_neg()) != 0 {
            exe::match_player_calc_ratings(j, 0);
        }
    }
    0
}

/// Match player `j` (the goalkeeper) tries to save a shot of strength `forca`: returns 1 if not saved, 0 if saved
/// (parried or pushed away, possibly losing condition or attributes); the deflected ball moves with tiredness flag 1.
/// Wrapper of `save_shot_injected` with the real outside calls.
///
/// [pt] { 0x70d430 } (ret 4, match_pl.cpp): ver `save_shot_injected`. A bola desviada vai com `cansa` 1.
pub(crate) unsafe extern "thiscall" fn save_shot(j: usize, strength: i16) -> u32 {
    save_shot_injected(j, strength, &mut |rotina, this, a| match rotina {
        0x6cd770 => {
            exe::cm3_match_engine_move_ball_to_cell(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, 1, a[4] as usize);
            0
        }
        _ => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

/// The long ball (hoof, cross, long pass): `0x6fa490` (ret 8, `match_pl.cpp`) with the calls that touch the game
/// injected (`0x6fad40` executes, `0x6cd490` ball out, commentary `0x6d6f70`); the target is chosen by facing and area,
/// then the pass is executed, with offside and ball out.
///
/// The player's facing (`+0x19a`, degrees) marks the sides the ball goes to (f12, f13, f14, f15). Near the opponent's
/// goal (distance < c) and outside the centre, without `modo` 4: cross (0x5e) to row 0..1 or 10..11 and column 3..5. In
/// his own half: mode 4 = 0x5c (mark ± random draw(2) and 1 towards the facing side), otherwise 0x5d (± random draw(4)
/// and 2); in the opponent's half: mode 4 = 0x5b (± random draw(2) and 2), otherwise 0x3e (ball in play, `+0x8ea9` =
/// −1) or 9 (± random draw(5) and 4/3). Outside mode 4, the row drops back 2 towards his own side when it does not go
/// there.
///
/// Without `refinar` (`+0x10b` or random draw(3)) and far from his own goal, the target goes halfway between the wing
/// (column 1 or 7, by facing) and the area (the nearer row up to 5, or 3 in mode 4). Executes (`0x6fad40`, type,
/// target); offside as in `0x6fa250`; ball off the grid without `+0x8eb2`: `0x6cd490`.
///
/// [pt] { 0x6fa490 } (ret 8, match_pl.cpp) com as chamadas que mexem no jogo injetadas (0x6fad40
/// executa, 0x6cd490 bola fora, comentario 0x6d6f70): o lancamento (chutao, cruzamento, bola longa).
/// A direcao do jogador (+0x19a, graus) marca os lados para onde a bola vai (f12, f13, f14, f15). Perto
/// do gol adversario (distancia < c) e fora do miolo, sem `modo` 4: cruzamento (0x5e) para a linha 0..1
/// ou 10..11 e coluna 3..5. No proprio campo: modo 4 = 0x5c (marca +- sorteio(2) e 1 pelo lado da
/// direcao), senao 0x5d (+- sorteio(4) e 2); no campo adversario: modo 4 = 0x5b (+- sorteio(2) e 2),
/// senao 0x3e (bola em jogo, +0x8ea9 = -1) ou 9 (+- sorteio(5) e 4/3). Fora o modo 4, a linha recua 2
/// para o proprio lado quando nao vai para la. Sem `refinar` (+0x10b ou sorteio(3)) e longe do proprio
/// gol, o alvo vai a meio caminho da ponta (coluna 1 ou 7, pela direcao) e da area (linha mais perto
/// a ate 5, ou 3 no modo 4). Executa (0x6fad40, tipo, alvo); impedimento como 0x6fa250; bola fora da
/// grade sem +0x8eb2: 0x6cd490.
pub(crate) unsafe fn long_ball_injected(j: usize, mode: i8, refinar: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = || *((j + match_player::ENGINE) as *const usize);
    let b = |p: usize, o: usize| (p + o) as *mut i8;
    let s = |n: i32| random(n) as i8;
    let dir = read_i8(j + match_player::SIDE);
    let ang = *((j + match_player::FACING) as *const i16);
    // the sides where the ball goes, by the player’s direction
    let (mut f12, mut f13, mut f14, mut f15) = (false, false, false, false);
    if !(0x32..=0x136).contains(&ang) {
        if dir == 1 {
            if ang <= 0x136 {
                f13 = true;
            } else {
                f12 = true;
            }
        } else {
            f14 = true;
        }
    }
    if ang > 0x28 && ang < 0x8c {
        f13 = true;
    }
    if ang > 0x82 && ang < 0xe6 {
        if dir == 0 {
            if ang > 0xb4 {
                f12 = true;
            } else {
                f13 = true;
            }
        } else {
            f15 = true;
        }
    }
    if !f12 && ang > 0xdc && ang < 0x140 {
        f12 = true;
    }
    let d = if dir == 0 {
        let lin = (read_i8(j + match_player::CELL_Y) as i32 - 11).abs() as i8 as isize;
        let col = (read_i8(j + match_player::CELL_X) as i32 - 4).abs() as i8 as isize;
        read_f32((0xa15e44 + 4 * (lin * 9 + col)) as usize)
    } else {
        exe::match_player_distance_to_cell(j, 4, 0)
    };
    let (bx, by) = (|| read_i8(m() + 0x8ea7), || read_i8(m() + 0x8ea8));
    let kind: u32;
    let (mut tx, mut ty): (i8, i8);
    // the column and the row according to the sides: (roll offset, step)
    let mira = |base: i8, for_minus: bool, for_more: bool, n: i32, step_towards: i8, n_middle: i32| -> i8 {
        if for_minus {
            base.wrapping_sub(s(n)).wrapping_sub(step_towards)
        } else if for_more {
            s(n).wrapping_add(base).wrapping_add(step_towards)
        } else {
            let r = s(n_middle);
            base.wrapping_sub(r).wrapping_add(s(n_middle))
        }
    };
    let x = read_i8(j + match_player::CELL_X);
    // fcomp + test ah,1: near the goal when the distance is smaller (NaN counts as smaller)
    let refina;
    if !(d as f64 >= read_f64(rdata::F64_5)) && !(2..=6).contains(&x) && mode != 4 {
        kind = 0x5e;
        ty = if dir == 0 { 11i8.wrapping_sub(s(2)) } else { s(2) };
        tx = s(3).wrapping_add(3);
        refina = mode != -1;
    } else {
        let y = read_i8(j + match_player::CELL_Y);
        // dir 1 attacks toward row 0: y >= 6 is its half (and likewise with dir 0 and y < 6)
        let field_own = (dir == 1 && y >= 6) || (dir == 0 && y < 6);
        if !field_own {
            // from the middle forward
            if mode == 4 {
                kind = 0x5c;
                ty = mira(by(), f15, f14, 2, 1, 2);
                tx = mira(bx(), f12, f13, 2, 1, 2);
                refina = true;
            } else {
                kind = 0x5d;
                ty = mira(by(), f15, f14, 4, 2, 3);
                tx = mira(bx(), f12, f13, 4, 2, 3);
                if dir == 0 {
                    if !f14 {
                        ty = ty.wrapping_add(2);
                    }
                } else if !f15 {
                    ty = ty.wrapping_sub(2);
                }
                refina = mode != -1;
            }
        } else if mode == 4 {
            kind = 0x5b;
            ty = mira(by(), f15, f14, 2, 2, 2);
            tx = mira(bx(), f12, f13, 2, 2, 2);
            refina = true;
        } else {
            kind = if read_i8(m() + 0x8ea9) != -1 { 9 } else { 0x3e };
            ty = mira(by(), f15, f14, 5, 4, 3);
            tx = mira(bx(), f12, f13, 5, 3, 3);
            if dir == 0 {
                if !f14 {
                    ty = ty.wrapping_add(2);
                }
            } else if !f15 {
                ty = ty.wrapping_sub(2);
            }
            refina = mode != -1;
        }
    }
    if refina {
        let y = read_i8(j + match_player::CELL_Y);
        let follows = refinar != 0 || ((b(j, match_player::MOMENT_VISION).read() != 0 || s(3) != 0) && !(dir == 0 && y < 2) && !(dir == 1 && y > 9));
        if follows {
            let ponta: i8 = if f12 {
                1
            } else if !f13 && s(2) != 0 {
                1
            } else {
                7
            };
            let mut row: i8;
            if dir == 0 {
                row = 10;
                loop {
                    let e = row as i32 - y as i32;
                    if e > 5 || (mode == 4 && e > 3) {
                        row -= 1;
                    } else {
                        break;
                    }
                }
            } else {
                row = 1;
                loop {
                    let e = y as i32 - row as i32;
                    if e > 5 || (mode == 4 && e > 3) {
                        row += 1;
                    } else {
                        break;
                    }
                }
            }
            tx = ((ponta as i32 + tx as i32) / 2) as i8;
            ty = ((row as i32 + ty as i32) / 2) as i8;
        }
    }
    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(1);
    // (x, y, type, column and row of the target): the receiver is zero
    let r = call(FN_MATCH_PLAYER_PASS, j, [x as u8 as u32, read_i8(j + match_player::CELL_Y) as u8 as u32, kind, tx as u8 as u32, ty as u8 as u32]);
    if r as u8 == 0 {
        return;
    }
    if kind as u16 != 0x36 {
        let q = exe::cm3_match_engine_referee_check_offside(m(), dir, read_i8(j + match_player::CELL_Y), ty, 0);
        if q != 0 {
            let lin = if read_i8(q + match_player::SIDE) == 0 { read_i8(q + match_player::CELL_Y) as i32 - 11 } else { read_i8(q + match_player::CELL_Y) as i32 };
            let col = (read_i8(q + match_player::CELL_X) as i32 - 4).abs() as i8 as isize;
            let lin = lin.abs() as i8 as isize;
            let d = read_f32((0xa15e44 + 4 * (lin * 9 + col)) as usize) as f64;
            if random(ftol(d * d + d * d)) != 0 {
                let mm = m();
                call(FN_MATCH_EVENTS_ADD_EVENT, *((mm + engine::COMMENTATOR) as *const usize), [0x1f78, mm as u32, read_i8(q + match_player::COMMENTARY_ID) as u8 as u32, dir as u8 as u32, u32::MAX]);
                let count = (m() as isize + 0xf5ca + 54 * dir as isize) as *mut u8;
                *count = (*count).wrapping_add(1);
                *b(q, match_player::OFFSIDES) = (*b(q, match_player::OFFSIDES)).wrapping_add(1);
                *b(j, 1) = (*b(j, 1)).wrapping_sub(1);
                *b(m(), 0x8ea7) = read_i8(q + match_player::CELL_X);
                *b(m(), 0x8ea8) = read_i8(q + match_player::CELL_Y);
                *b(m(), 0x8eb2) = 7;
                *b(m(), 0x8eae) = (dir == 0) as i8;
                *((m() + 0xf582) as *mut usize) = 0;
                return;
            }
            exe::cm3_match_engine_set_pending_foul(m(), 0xffff, q as u32, 0, 0, 8, read_i8(q + match_player::CELL_X) as u8, read_i8(q + match_player::CELL_Y) as u8, 0xff, 0xff);
        }
    }
    let mm = m();
    if read_i8(mm + engine::RESTART_TYPE) != 0 {
        return;
    }
    if !(0..9).contains(&read_i8(mm + engine::BALL_X)) || !(0..12).contains(&read_i8(mm + engine::BALL_Y)) {
        call(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY, mm, [read_i8(j + match_player::SIDE) as u8 as u32, 0, 0, 0, 0]);
    }
}

/// Match player `j` plays a long ball or cross depending on `modo` and his position, aiming at a target cell (refined
/// unless `refinar`), executes the pass and checks offside and ball out of play. Wrapper of `long_ball_injected` with
/// the real outside calls.
///
/// [pt] { 0x6fa490 } (ret 8, match_pl.cpp): ver `long_ball_injected`.
pub(crate) unsafe extern "thiscall" fn long_ball(j: usize, mode: i8, refinar: u8) {
    long_ball_injected(j, mode, refinar, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u8, usize, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_PASS);
            let mut out = 1u8;
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut out, 0, a[3], a[4]);
            r as u32 | ((out as u32) << 8)
        }
        0x6cd490 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_BALL_OUT_OF_PLAY);
            f(this, a[0] as u8) as u32
        }
        _ => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

/// `0x6fa250` (ret 0x14, match_pl.cpp) with the calls that change the game injected (`chamar(rotina, this,
/// argumentos)`; `execute_pass` returns the answer in the low byte and the output it writes into argument 3 in the next
/// byte): the player's pass (+0x2b goes up), with offside check and commentary, possession and the receiver's immediate
/// decision.
///
/// It executes (`execute_pass` with his square, `tipo`, the receiver and arguments 4 and 5); without success, nothing.
///
/// Outside type 0x36, it looks for the offside player (`exe::cm3_match_engine_referee_check_offside` on the line of the 5th argument): random(2 × d1 × d2,
/// his distance to the opposing goal) zero and offside: comments (0x1f78), counts (`engine::OFFSIDES` + 54 × side), the offside
/// player marks +0x13, the passer loses 1 at +1, the ball goes to his square for the other side (`engine::RESTART_TYPE` = 7) and
/// nobody has it. Otherwise it records the incident (`exe::cm3_match_engine_set_pending_foul`, 8).
///
/// Without `engine::RESTART_TYPE` and with the pass's output: the receiver and the striker (`engine::SET_PIECE_TAKER`), the side counts possession
/// (`engine::POSSESSION_COUNTER`: his goes up, the other's drops by 10) and the receiver decides immediately (`decide_player`) if random(80) does
/// not exceed +0x105 × +0x107.
///
/// [pt] { 0x6fa250 } (ret 0x14, match_pl.cpp) com as chamadas que mexem no jogo injetadas
/// (`chamar(rotina, this, argumentos)`; a 0x6fad40 devolve a resposta no byte baixo e a saida que
/// ela grava no argumento 3 no byte seguinte): o passe do jogador (+0x2b sobe). Executa (0x6fad40 com
/// a casa dele, `tipo`, o receptor e os argumentos 4 e 5); sem sucesso, nada. Fora do tipo 0x36,
/// procura o impedido (0x6cfd70 na linha 5o argumento): sorteio(2 x d1 x d2, a distancia dele ao gol
/// adversario) zero e impedimento: narra (0x1f78), conta (+0xf5ca + 54 x lado), o impedido marca +0x13,
/// o passador perde 1 em +1, a bola vai a casa dele com o outro lado (+0x8eb2 = 7) e ninguem a tem.
/// Senao registra o lance (0x6bb9a0, 8). Sem +0x8eb2 e com a saida do passe: o receptor e o batedor
/// (+0xf57a), o lado conta a posse (+0x8d46: o dele sobe, o outro cai 10) e o receptor decide ja
/// (0x7103e0) se sorteio(80) nao passar de +0x105 x +0x107.
pub(crate) unsafe fn pass_ball_injected(j: usize, kind: u8, receiver: usize, a4: u32, a5: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = || *((j + match_player::ENGINE) as *const usize);
    let b = |p: usize, o: usize| (p + o) as *mut i8;
    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(1);
    let dir = read_i8(j + match_player::SIDE);
    let r = call(FN_MATCH_PLAYER_PASS, j, [read_i8(j + match_player::CELL_X) as u8 as u32, read_i8(j + match_player::CELL_Y) as u8 as u32, kind as u32, receiver as u32, a4]);
    if r as u8 == 0 {
        return;
    }
    let full = (r >> 8) as u8;
    if kind != 0x36 {
        let q = exe::cm3_match_engine_referee_check_offside(m(), dir, read_i8(j + match_player::CELL_Y), a5 as i8, 0);
        if q != 0 {
            let goal = if read_i8(q + match_player::SIDE) == 0 { 11 } else { 0 };
            let d1 = exe::match_player_distance_to_cell(q, 4, goal);
            let d2 = exe::match_player_distance_to_cell(q, 4, goal);
            let n = ftol(d2 as f64 * d1 as f64 + d2 as f64 * d1 as f64);
            if random(n) != 0 {
                let mm = m();
                call(FN_MATCH_EVENTS_ADD_EVENT, *((mm + engine::COMMENTATOR) as *const usize), [0x1f78, mm as u32, read_i8(q + match_player::COMMENTARY_ID) as u8 as u32, dir as u8 as u32, u32::MAX]);
                let count = (m() as isize + 0xf5ca + 54 * dir as isize) as *mut u8;
                *count = (*count).wrapping_add(1);
                *b(q, match_player::OFFSIDES) = (*b(q, match_player::OFFSIDES)).wrapping_add(1);
                *b(j, 1) = (*b(j, 1)).wrapping_sub(1);
                *b(m(), 0x8ea7) = read_i8(q + match_player::CELL_X);
                *b(m(), 0x8ea8) = read_i8(q + match_player::CELL_Y);
                *b(m(), 0x8eb2) = 7;
                *b(m(), 0x8eae) = (dir == 0) as i8;
                *((m() + 0xf582) as *mut usize) = 0;
                return;
            }
            exe::cm3_match_engine_set_pending_foul(m(), 0xffff, q as u32, 0, 0, 8, read_i8(q + match_player::CELL_X) as u8, read_i8(q + match_player::CELL_Y) as u8, 0xff, 0xff);
        }
    }
    let mm = m();
    if read_i8(mm + engine::RESTART_TYPE) != 0 || full == 0 {
        return;
    }
    *((mm + engine::SET_PIECE_TAKER) as *mut usize) = receiver;
    let dir = read_i8(j + match_player::SIDE);
    *b(mm, engine::LAST_POSSESSION_SIDE) = dir;
    if dir != -1 {
        let my = (mm as isize + 0x8d46 + dir as isize) as *mut i8;
        *my = (*my).wrapping_add(1);
        let other = (mm as isize + 0x8d46 + (dir == 0) as isize) as *mut i8;
        if *other > 0 {
            *other = (*other).wrapping_sub(10);
        }
    }
    if receiver != 0 && read_i8(receiver + match_player::BUSY) == 0 {
        let r = random(0x50);
        if read_i8(receiver + match_player::MOMENT_ANTICIPATION) as i32 * read_i8(receiver + match_player::MOMENT_MOVEMENT) as i32 > r {
            call(FN_MATCH_PLAYER_TAKE_TURN, receiver, [0; 5]);
        }
    }
}

/// Hook entry for 0x6fa250 (pass_ball_injected): player `j` passes to `receptor` (`execute_pass`), with the offside check and
/// commentary, possession count and the receiver's immediate decision (`decide_player`); `a3` is the in/out argument of
/// `execute_pass`.
///
/// [pt] { 0x6fa250 } (ret 0x14, match_pl.cpp): ver `pass_ball_injected`. O 3o argumento e a saida da 0x6fad40.
pub(crate) unsafe extern "thiscall" fn pass_ball(j: usize, kind: u8, receiver: usize, mut a3: u32, a4: u32, a5: u32) {
    pass_ball_injected(j, kind, receiver, a4, a5, &mut |rotina, this, a| match rotina {
        0x6fad40 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32, usize, u32, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_PASS);
            let r = f(this, a[0] as i8, a[1] as i8, a[2], &mut a3, a[3] as usize, a[4], a5);
            r as u32 | ((a3 as u8 as u32) << 8)
        }
        0x7103e0 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_TURN);
            f(this);
            0
        }
        _ => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
    })
}

/// { `decide_player` } (match_pl.cpp) with the calls that touch the game injected (`chamar(rotina, this, argumentos)`:
/// `dribble` dribble, `exe::match_player_choose_attacking_off_ball_move` and `move_off_ball` the off-ball decisions): the player's decision.
///
/// Without the ball and without mark 1 of `table_a`: he "switches off" (0x67) if random draw(10) + +0x4d × c does not
/// exceed random draw(65). With the ball on the other side (`engine::POSSESSION_SIDE` = opposite side), or on his side but outside the
/// band or with the ball out of play, or on the other side with no taker on his side: random draw(20) zero and without
/// +0x10d he also switches off, tiring (+0x35 − 15, +0x2b = 1, +0x4d + 5..9). With the side in situation 5 (+0x1ab),
/// far from goal according to the table (or `exe::match_player_distance_to_cell` on side 1), random draw(12) and +0x10d against random draw(10)
/// switch him off (without the −15). With +0x2b, only the marked players.
///
/// Before each outcome, the players marked by him (+0x1ae, +0x101) that can be chosen (`can_choose`) move (`move_off_ball`) if
/// +0x107 × +0x104 does not exceed random draw(900) + 50. The ball holder dribbles (`dribble`) and may decide again
/// (random draw(+0x7d) against random draw(5)); the others go to `exe::match_player_choose_attacking_off_ball_move` (ball with his side, within the band, in
/// play) or to `move_off_ball`.
///
/// [pt] { 0x7103e0 } (match_pl.cpp) com as chamadas que mexem no jogo injetadas (`chamar(rotina, this,
/// argumentos)`: 0x7109f0 conduzir, 0x713fc0 e 0x714d40 as decisoes sem a bola): a decisao do jogador.
/// Sem a bola e sem a marca 1 de `table_a`: "desliga" (0x67) se sorteio(10) + +0x4d x c nao passar do
/// sorteio(65). Com a bola do outro lado (+0x8eae = lado oposto), ou do lado dele fora da faixa ou com
/// a bola fora de jogo, ou do outro lado sem batedor do lado dele: sorteio(20) zero e sem +0x10d tambem
/// desliga, cansando (+0x35 - 15, +0x2b = 1, +0x4d + 5..9). Com o lado na situacao 5 (+0x1ab), longe do
/// gol pela tabela (ou 0x6f09b0 no lado 1) o sorteio(12) e +0x10d contra sorteio(10) desligam (sem o
/// -15). Com +0x2b, so os marcados. Antes de cada desfecho, os marcados por ele (+0x1ae, +0x101) que
/// podem ser escolhidos (0x6bce20) se mexem (0x714d40) se +0x107 x +0x104 nao passar de sorteio(900) +
/// 50. O dono da bola conduz (0x7109f0) e pode decidir de novo (sorteio(+0x7d) contra sorteio(5));
/// os outros vao a 0x713fc0 (bola com o lado dele, na faixa, em jogo) ou a 0x714d40.
pub(crate) unsafe fn player_decide_injected(j: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let w = |o: usize| (j + o) as *mut u16;
    let b = |o: usize| read_i8(j + o);
    let u = |a: usize| *(a as *const usize);
    // the players marked by him move (0x714d40) when the random roll allows
    let marked = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        let mut k = 0i8;
        while k < b(0x101) {
            let p = u(j + match_player::MARKERS + 4 * k as usize);
            if (exe::cm3_match_engine_can_marker_challenge(u(j + match_player::ENGINE), p, b(0x102), b(0x103)) as u32) as u8 != 0 {
                let prod = b(0x107) as i32 * b(0x104) as i32;
                if prod <= random(0x384) + 0x32 {
                    c(0x714d40, p, [0; 5]);
                }
            }
            k += 1;
        }
    };
    let desliga = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, cansa_35: bool| {
        marked(c);
        if cansa_35 {
            *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(0xfff1);
        }
        *w(match_player::ACTION) = 0x67;
        if b(0x2b) == 0 {
            *((j + match_player::BUSY) as *mut u8) = 1;
        }
        let r = random(5) + 5;
        let f = (j + match_player::STAMINA) as *mut f32;
        *f = (r as f64 + *f as f64) as f32;
    };
    loop {
        let m = u(j + match_player::ENGINE);
        let ta = || if m == 0 { 0 } else { table_a(m, j) };
        let dir = b(0x27);
        let and_owner = j == u(m + engine::BALL_HOLDER);
        if !and_owner && ta() & 1 == 0 {
            let v4d = *((j + match_player::STAMINA) as *const f32);
            let r1 = random(0x41);
            let r2 = random(10);
            // fcomp + test ah,0x41: continues only if above the random roll (NaN switches it off)
            if !(r2 as f64 + v4d as f64 * read_f64(rdata::F64_0_0078125) > r1 as f64) {
                marked(call);
                *w(match_player::ACTION) = 0x67;
                return;
            }
        }
        if !and_owner && ta() & 1 == 0 {
            let ae = read_i8(m + engine::POSSESSION_SIDE);
            let pressa = if ae as i32 == (dir == 0) as i32 {
                true
            } else if dir == ae {
                false
            } else {
                let bat = u(m + engine::SET_PIECE_TAKER);
                !(bat != 0 && read_i8(bat + match_player::SIDE) == dir) && (exe::is_in_own_penalty_area((dir == 0) as u8, read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y)) == 0 || read_i8(m + engine::BALL_HEIGHT) == -1)
            };
            if pressa && random(0x14) == 0 && b(0x10d) == 0 {
                desliga(call, true);
                return;
            }
        }
        if !and_owner && *((m as isize + 0x1ab + dir as isize) as *const i8) == 5 && ta() & 1 == 0 {
            let d = if dir == 0 {
                let row = (b(0x103) as i32 - 11).abs() as i8 as isize;
                let col = (b(0x102) as i32 - 4).abs() as i8 as isize;
                read_f32((0xa15e44 + 4 * (row * 9 + col)) as usize)
            } else {
                exe::match_player_distance_to_cell(j, 4, 0)
            };
            let r = random(0xc);
            // fcomp + test ah,0x41: only far (draw greater than the distance, NaN not)
            if r as f64 > d as f64 && (b(0x10d) as i32) < random(10) {
                desliga(call, false);
                return;
            }
        }
        if b(0x2b) != 0 && !and_owner {
            marked(call);
            *w(match_player::ACTION) = 0x67;
            return;
        }
        if !and_owner {
            let ae = read_i8(m + engine::POSSESSION_SIDE);
            let with_ball = if ae as i32 == (dir == 0) as i32 {
                false
            } else if dir == ae {
                true
            } else {
                let bat = u(m + engine::SET_PIECE_TAKER);
                (bat != 0 && read_i8(bat + match_player::SIDE) == dir) || (exe::is_in_own_penalty_area((dir == 0) as u8, read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y)) != 0 && read_i8(m + engine::BALL_HEIGHT) != -1)
            };
            if with_ball {
                call(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE, j, [1, 0, 0, 0, 0]);
            } else {
                call(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE, j, [0; 5]);
            }
            return;
        }
        marked(call);
        call(FN_MATCH_PLAYER_DECIDE_BALL_ACTION, j, [0; 5]);
        *((u(j + match_player::ENGINE) + 0x1a7) as *mut u8) = 0;
        let m = u(j + match_player::ENGINE);
        if j == u(m + engine::BALL_HOLDER) || *w(match_player::ACTION) == 0x6d || b(0x2b) != 0 || read_i8(m + engine::RESTART_TYPE) != 0 || b(0x107) == 0 {
            return;
        }
        let r = random(ftol(*((j + match_player::ANTICIPATION) as *const f32) as f64));
        if r <= random(5) {
            return;
        }
    }
}

/// off the ball, "switches off" (0x67) when draw(10) + +0x4d·c does not beat draw(65); with the ball out of the side's
/// reach, draw(20) zero and no +0x10d switches off tiring (+0x35 − 15, +0x2b = 1, +0x4d + 5..9); with the side in
/// situation 5 (+0x1ab) and far from goal, switches off without the −15; with +0x2b, idle. Before each outcome the
/// marked players (+0x1ae) move (`move_off_ball`) by draw(900) against +0x107·+0x104. The ball holder dribbles (`dribble`, new
/// tape port) and may decide again (draw(+0x7d) against draw(5)); the others go to `exe::match_player_choose_attacking_off_ball_move` (ball with the side, in the
/// band, in play) or `move_off_ball`
/// Original: `decide_player` (0x7103e0, match_pl.cpp, thiscall).
///
/// [pt] { 0x7103e0 } (match_pl.cpp): ver `player_decide_injected`.
pub(crate) unsafe extern "thiscall" fn decide_player(j: usize) {
    player_decide_injected(j, &mut |rotina, this, _a| match rotina {
        0x713fc0 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE);
            f(this, 1);
            0
        }
        0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE);
            f(this);
            0
        }
        _ => {
            (exe::match_player_decide_ball_action(this, 0) as u32)
        }
    })
}

/// Port of `move_off_ball` (match_pl.cpp, ~4.7 KB) with the calls that touch the game injected (`chamar(rotina, this,
/// argumentos)`: `player_move`, 0x6f46b0, `exe::match_player_drop_back_one_row`, `take_ball` and the commentary 0x6d6f70): the movement of a player without
/// the ball outside the attacking band (0x7109df; in the band, with the ball in play, it is `exe::match_player_choose_attacking_off_ball_move`). Action in
/// +0x198.
///
/// Set piece: the taker (`engine::SET_PIECE_TAKER`) or whoever the side designates (`side_byte`, without `engine::BALL_TRANSIT_TICKS`) goes to the spot (0x100).
/// Ball out of play: formation (`exe::cm3_match_engine_get_reference_tactic_cell`, 0x68).
///
/// Run in behind (0x7a): whoever has a good player sheet (+0x105 x +0x106 x +0x10a against random(25)), with someone on
/// the ball and behind the ball, runs beyond the line (`engine::OFFSIDE_LINE_SIDE1` + 2 or `engine::OFFSIDE_LINE` - 2) if the stored goalkeeper
/// (`engine::FURTHEST_FORWARD_SIDE1`/`engine::FURTHEST_FORWARD`) is not ahead: right on the line (mark 0x400 of the block) or one line short of it by random(100)
/// against +0x106 x +0x10b.
///
/// Without `table_a` = 1: support for the ball owner on the other side (`side_difference`) and set piece; pressing roles (0x700
/// with 0x38) run to the ball by random draw (+0xdd and distance). Marking someone (+0x1aa, 0x105): follows his cell
/// (like `exe::match_player_choose_attacking_off_ball_move`) or lets go (position -5, -3..-1). Otherwise the pressing: threshold 1.5 (or ~2 by +0x10d) reduced
/// without +0x106/+0x10a and by the random draw against +0x1a2; near the ball it presses (target = ball) by marks
/// 0x8000/0x2000/0x4000 or a random draw on the other side's possession, otherwise the cell from `exe::cm3_match_engine_get_player_target_cell`.
///
/// Goalkeeper (`table_a` = 1): on the ball he catches it and it is commentated (0x1f46, 0x103), near it he comes out to
/// the ball (0x103), otherwise his cell (0x71).
///
/// The others: deep with the ball and the opposing goalkeeper out (0x78), move up a line (`exe::match_player_drop_back_one_row`, 0x6b), under the
/// ball's side go to it (0x100; on it, take it and commentate 0x1f46 or 0x2113, or 0x104), in the opposing area with a
/// lone marker they stay (0x104); otherwise their cell (0x68, or 0x100 when pressing; already in it 0x76).
///
/// [pt] { 0x714d40 } (match_pl.cpp, ~4,7 KB) com as chamadas que mexem no jogo injetadas
/// (`chamar(rotina, this, argumentos)`: 0x6f09f0, 0x6f46b0, 0x6f5b80, 0x6f0630 e o comentario
/// 0x6d6f70): o movimento do jogador sem a bola fora da faixa de ataque (0x7109df; na faixa, com a
/// bola em jogo, e 0x713fc0). Acao em +0x198.
/// Bola parada: o batedor (+0xf57a) ou quem o lado aponta (0x6cfda0, sem +0x1c5) vai a marca (0x100).
/// Bola fora de jogo: formacao (0x6c2a40, 0x68). Infiltracao (0x7a): quem tem a ficha boa (+0x105 x
/// +0x106 x +0x10a contra sorteio(25)), com alguem com a bola e atras da bola, corre para alem da linha
/// (+0x8eb0 + 2 ou +0x8eaf - 2) se o goleiro guardado (+0xf59a/+0xf596) nao estiver a frente: logo na
/// linha (marca 0x400 do bloco) ou a uma linha dela por sorteio(100) contra +0x106 x +0x10b.
/// Sem `table_a` = 1: apoio ao dono da bola do outro lado (0x6c4180) e bola parada; papeis de
/// pressao (0x700 com 0x38) correm para a bola por sorteio (+0xdd e distancia). Marcando alguem
/// (+0x1aa, 0x105): segue a casa dele (como 0x713fc0) ou larga (posicao -5, -3..-1). Senao a pressao:
/// limiar 1,5 (ou ~2 por +0x10d) reduzido sem +0x106/+0x10a e pelo sorteio contra +0x1a2; perto da bola
/// pressiona (alvo = bola) pelas marcas 0x8000/0x2000/0x4000 ou sorteio da posse do outro lado,
/// senao a casa de 0x6c2be0. Goleiro (`table_a` = 1): na bola pega e narra (0x1f46, 0x103), perto
/// sai a bola (0x103), senao a casa (0x71). Os outros: no fundo com a bola e o goleiro adversario
/// fora (0x78), avancam uma linha (0x6f5b80, 0x6b), sob o lado da bola vao a ela (0x100; nela,
/// pegam e narram 0x1f46 ou 0x2113, ou 0x104), na area adversaria com um marcador sozinho ficam
/// (0x104); fora isso a casa (0x68, ou 0x100 pressionando; ja nela 0x76).
pub(crate) unsafe fn off_ball_move_injected(j: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((j + match_player::ENGINE) as *const usize);
    let dir = read_i8(j + match_player::SIDE);
    let block = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let u = |a: usize| *(a as *const usize);
    let b = |o: usize| read_i8(j + o);
    let ta = || if m == 0 { 0 } else { table_a(m, j) };
    let rep = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, x: i8, y: i8, a: i32| {
        c(0x6f09f0, j, [x as u8 as u32, y as u8 as u32, a as u32, 0, 0]);
    };
    // table distance between the player and (x, y)
    let dist_table = |x: i8, y: i8| {
        let lin = (read_i8(j + match_player::CELL_Y) as i32 - y as i32).abs() as i8 as isize;
        let col = (read_i8(j + match_player::CELL_X) as i32 - x as i32).abs() as i8 as isize;
        read_f32((0xa15e44 + 4 * (lin * 9 + col)) as usize)
    };
    // picks up the dead ball on its square: points, narrates and picks up (0x6f0630)
    let pegar_e_narrar = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32| {
        c(0x6d6f70, u(m + engine::COMMENTATOR), [code, m as u32, b(0x2e) as u8 as u32, dir as u8 as u32, u32::MAX]);
        c(0x6f0630, j, [0; 5]);
    };
    let (ae, a9) = (read_i8(m + engine::POSSESSION_SIDE), read_i8(m + engine::BALL_HEIGHT));
    let mark = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        rep(c, read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y), 0x100);
        *w(match_player::ACTION) = 0x100;
    };
    if ae == -1 {
        let batedor = u(m + engine::SET_PIECE_TAKER);
        let vai = if batedor == j { random(0x14).wrapping_add(b(0x107) as i32) != 0 } else { batedor == 0 && exe::cm3_match_engine_is_player_nearest_to_ball(m, j) != 0 && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0 };
        if vai {
            mark(call);
            return;
        }
    }
    let ousado = exe::formation_get_marking_type(block, b(0x19)) & 4 != 0;
    *w(match_player::ACTION) = 0xffff;
    let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
    let t = ta();
    if a9 == -1 && ae != -1 {
        let (mut x, mut y) = (0u8, 0u8);
        exe::cm3_match_engine_get_reference_tactic_cell(m, j, &mut x, &mut y);
        *w(match_player::ACTION) = 0x68;
        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, j, [x as u32, y as u32, 0, 0, 0]);
        return;
    }
    // run beyond the offside line (0x7a)
    {
        let (x, y) = (b(0x102), b(0x103));
        let r = random(0x19);
        let sheet = b(0x105) as i32 * b(0x106) as i32 * b(0x10a) as i32;
        let base = sheet > r && u(m + engine::BALL_HOLDER) != 0 && t != 1;
        let corre = |c: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, row: i8, alem: i8| {
            *w(match_player::ACTION) = 0x7a;
            rep(c, x, row.wrapping_add(alem), -1);
        };
        if dir == 0 {
            if base && by > y && y < 6 {
                let gk = u(m + engine::FURTHEST_FORWARD_SIDE1);
                let gy = |gk: usize| if gk != 0 { read_i8(gk + match_player::CELL_Y) } else { 9 };
                let l = read_i8(m + engine::OFFSIDE_LINE_SIDE1);
                if gy(gk) <= l {
                    let d = y as i32 - l as i32;
                    if *((block + tactics_fields::FLAGS) as *const u32) & 0x400 != 0 && d < 2 && d > -2 {
                        corre(call, l, 2);
                        return;
                    }
                    let r2 = random(100);
                    if b(0x106) as i32 * b(0x10b) as i32 > r2 && y as i32 - read_i8(m + engine::OFFSIDE_LINE_SIDE1) as i32 == 1 && gy(u(m + engine::FURTHEST_FORWARD_SIDE1)) as i32 + 1 <= read_i8(m + engine::OFFSIDE_LINE_SIDE1) as i32 {
                        corre(call, read_i8(m + engine::OFFSIDE_LINE_SIDE1), 2);
                        return;
                    }
                }
            }
        } else if base && by < y && y > 5 {
            let gk = u(m + engine::FURTHEST_FORWARD);
            let gy = |gk: usize| if gk != 0 { read_i8(gk + match_player::CELL_Y) } else { -1 };
            let l = read_i8(m + engine::OFFSIDE_LINE);
            if gy(gk) >= l {
                let d = l as i32 - y as i32;
                if *((block + tactics_fields::FLAGS) as *const u32) & 0x400 != 0 && d < 2 && d > -2 {
                    corre(call, l, -2);
                    return;
                }
                let r2 = random(100);
                if b(0x106) as i32 * b(0x10b) as i32 > r2 && read_i8(m + engine::OFFSIDE_LINE) as i32 - y as i32 == 1 && gy(u(m + engine::FURTHEST_FORWARD)) as i32 - 1 >= read_i8(m + engine::OFFSIDE_LINE) as i32 {
                    corre(call, read_i8(m + engine::OFFSIDE_LINE), -2);
                    return;
                }
            }
        }
    }
    let (x, y) = (b(0x102), b(0x103));
    if t != 1 {
        let owner = u(m + engine::BALL_HOLDER);
        let apoio = ae == -1 || (owner != 0 && read_i8(owner + match_player::SIDE) != dir && exe::cm3_match_engine_get_offside_margin_for_row(m, (dir == 0) as u8, read_i8(owner + match_player::CELL_Y) as u8) as i8 >= 0);
        let batedor = u(m + engine::SET_PIECE_TAKER);
        if apoio && !(batedor != 0 && read_i8(batedor + match_player::SIDE) == dir) {
            if exe::cm3_match_engine_is_player_nearest_to_ball(m, j) != 0 {
                mark(call);
                return;
            }
            if ta() & 0x700 != 0 && ta() & 0x38 != 0 {
                let d = dist_table(bx, by);
                let r1 = random(ftol(*((j + match_player::TACKLING) as *const f32) as f64));
                let v = r1 + b(0x10d) as i32;
                if v > random(ftol(d as f64 * read_f32(rdata::F32_8) as f64)) {
                    rep(call, bx, by, 0x100);
                    *w(match_player::ACTION) = 0x100;
                    return;
                }
            }
        }
    }
    // marcacao individual (+0x1aa)
    let marked = u(j + match_player::MARKING_TARGET);
    if marked != 0 {
        let larga_a_vez = *((block + 0x585) as *const u8) & 4 != 0 && u(m + engine::BALL_HOLDER) != 0 && {
            if dir == 0 && y < 6 {
                !((by as i32 - 1) < y as i32 || read_i8(m + engine::OFFSIDE_LINE) <= read_i8(marked + match_player::CELL_Y))
            } else if dir == 1 {
                y <= 5 || (by as i32 + 1 <= y as i32 && read_i8(m + engine::OFFSIDE_LINE_SIDE1) < read_i8(marked + match_player::CELL_Y))
            } else {
                true
            }
        };
        if !larga_a_vez {
            let pm = read_i8(marked + match_player::POSITION);
            if pm == -5 || (pm > -4 && pm < 0) {
                *((j + match_player::MARKING_TARGET) as *mut usize) = 0;
            } else {
                *w(match_player::ACTION) = 0x105;
                if b(0x105) != 0 && u(m + engine::SET_PIECE_TAKER) == marked {
                    rep(call, bx, by, -1);
                    return;
                }
                let (mut tx, mut ty) = (read_i8(marked + match_player::CELL_X), read_i8(marked + match_player::CELL_Y));
                if exe::is_in_attacking_third((dir == 0) as u8, ty) == 0 && !ousado {
                    let r = random(0x15);
                    // fcomp + test ah,0x41: only steps forward if the roll is greater (not NaN)
                    if r as f64 > *((marked + match_player::JUMPING) as *const f32) as f64 {
                        if dir == 0 && ty > 0 {
                            ty -= 1;
                        } else if dir == 1 && ty < 11 {
                            ty += 1;
                        }
                        if tx <= 2 {
                            tx += 1;
                        } else if tx >= 6 {
                            tx -= 1;
                        }
                    }
                }
                rep(call, tx, ty, -1);
                return;
            }
        }
    }
    // pressure: target on the ball, or the square from 0x6c2be0
    let (mut px, mut py) = (0i8, 0i8);
    let mut pressiona = false;
    if ae != -1 && t & 1 == 0 {
        let threshold: f64 = if random(ftol(*((j + match_player::TEAMWORK) as *const f32) as f64 + read_f64(rdata::F64_0_5))) == 0 {
            read_f64(rdata::F64_0)
        } else {
            let mut f = f64::from_bits(0x3ff8_0000_0000_0000);
            if b(0x10d) as i32 > random(10) {
                f = f64::from_bits(0x3fff_eb85_1eb8_51ec);
            }
            if b(0x106) == 0 {
                f *= read_f64(rdata::F64_1_33);
            }
            if b(0x10a) == 0 {
                f *= read_f64(rdata::F64_1_33);
            }
            let a2 = *((j + match_player::CONFIDENCE) as *const f32);
            let r3 = random(0x96) + 0x1e;
            if !(r3 as f64 >= a2 as f64) {
                f * read_f64(rdata::F64_1_33)
            } else {
                f
            }
        };
        // fcomp st(1) + test ah,1: close when the distance is smaller (NaN counts as smaller)
        if !(dist_table(bx, by) as f64 >= threshold) {
            let mk = exe::formation_get_player_instructions(block, b(0x19));
            let est = *((block + tactics_fields::FLAGS) as *const u32);
            pressiona = mk & 0x8000 != 0
                || (exe::formation_get_player_instructions(block, b(0x19)) & 0x2000 != 0 && est & 0x100 != 0)
                || est & 0x4000 != 0
                || (ta() & 2 == 0 && ta() & 4 == 0 && {
                    let r4 = random(*((m as isize + 0x8d46 + (dir == 0) as isize) as *const i8) as i32);
                    r4 > random(25) && exe::cm3_match_engine_is_behind_own_back_line(m, j) == 0
                });
        }
    }
    if pressiona {
        (px, py) = (bx, by);
    } else {
        exe::cm3_match_engine_get_player_target_cell(m, j, &mut px, &mut py, 1);
        px = (exe::match_player_bias_target_column(j, px) as i32) as i8;
    }
    let v_goalkeeper = if dir != 0 { 0 } else { 11 };
    if t == 1 {
        if bx == x && by == y {
            if ae == -1 && a9 < 5 && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0 {
                exe::match_player_set_facing_direction(j, x, y, 4, v_goalkeeper);
                pegar_e_narrar(call, 0x1f46);
                *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(4);
                *w(match_player::ACTION) = 0x103;
            }
            return;
        }
        // test ah,0x41 / je: continues when the distance does not exceed the limit (NaN also continues)
        if exe::cm3_match_engine_is_player_nearest_to_ball(m, j) != 0 && !(dist_table(bx, by) as f64 > read_f64(rdata::F64_1_5)) {
            if exe::is_in_own_penalty_area(dir as u8, bx, by) != 0 && !(ae != -1 && (b(0x106) != 0 || b(0x10a) != 0)) {
                *w(match_player::ACTION) = 0x103;
                rep(call, bx, by, 0x100);
                return;
            }
            if ae == -1 && (exe::cm3_match_engine_min_outfield_distance_to_cell(m, (dir == 0) as i8, bx, by, 1) as f64 > read_f64(rdata::F64_2_5) || (b(0x106) == 0 && b(0x10a) == 0)) {
                if (0..=8).contains(&bx) && (0..=11).contains(&by) {
                    *w(match_player::ACTION) = 0x103;
                    rep(call, bx, by, 0x100);
                }
                return;
            }
        }
        *w(match_player::ACTION) = 0x71;
        rep(call, px, py, -1);
        return;
    }
    let in_bottom = (2..=6).contains(&x) && if dir == 1 { (10..=11).contains(&y) } else { (0..=1).contains(&y) };
    if ae != -1 && in_bottom && exe::is_in_own_penalty_area(dir as u8, bx, by) != 0 {
        let q = exe::cm3_match_engine_get_goalkeeper(m, dir);
        if q == 0 {
            // the original shows the error box and quits the game (0x5cc7d0, 0x5a8f60, exit(-1))
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
        if read_i8(q + match_player::BUSY) > 1 {
            rep(call, 4, if dir != 0 { 11 } else { 0 }, -1);
            *w(match_player::ACTION) = 0x78;
            return;
        }
    }
    if exe::cm3_match_engine_get_offside_margin(m, j) as i8 >= 0 && b(0x107) != 0 {
        call(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW, j, [0, 0, 0, 0, 0]);
        *w(match_player::ACTION) = 0x6b;
        return;
    }
    let vai_ball = exe::cm3_match_engine_is_player_nearest_to_ball(m, j) != 0
        && if ae == -1 {
            (dir == 1 && y >= 6) || (dir == 0 && y < 6) || (b(0x10d) != 0 && !(exe::get_cell_distance(bx, by, px, py) as f64 >= read_f32(rdata::F32_2) as f64))
        } else {
            b(0x10d) != 0
        };
    if vai_ball {
        if !(0..=8).contains(&bx) || !(0..=11).contains(&by) {
            *w(match_player::ACTION) = if pressiona { 0x100 } else { 0x68 };
            rep(call, px, py, 0x100);
            return;
        }
        if bx != x || by != y {
            *w(match_player::ACTION) = 0x100;
            rep(call, bx, by, 0x100);
            return;
        }
        if ae == -1 && a9 < 5 && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0 {
            *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(4);
            *w(match_player::ACTION) = 0x100;
            exe::match_player_set_facing_direction(j, x, y, 4, v_goalkeeper);
            let near_goal = read_i8(m + engine::SHOT_REBOUND) != 0 && (3..=5).contains(&x) && ((dir == 0 && y >= 10) || (dir == 1 && y <= 1));
            pegar_e_narrar(call, if near_goal { 0x2113 } else { 0x1f46 });
        } else {
            *w(match_player::ACTION) = 0x104;
        }
        return;
    }
    if in_bottom {
        let home = (m as isize + 0x21b6 + 90 * (12 * x as isize + y as isize)) as usize;
        let other = *((home as isize + (dir == 0) as isize) as *const u8);
        let my = *((home as isize + dir as isize) as *const u8);
        if other != 0 && my == 1 && b(0x10a) != 0 {
            *w(match_player::ACTION) = 0x104;
            return;
        }
    }
    if px == x && py == y {
        *w(match_player::ACTION) = if pressiona { 0x100 } else { 0x76 };
        return;
    }
    *w(match_player::ACTION) = if pressiona { 0x100 } else { 0x68 };
    rep(call, px, py, 0x100);
}

/// set piece (spot, 0x100), formation (0x68), run beyond the offside line (0x7a, by record +0x105/+0x106/+0x10a/+0x10b
/// and the kept goalkeeper), support and pressing (0x100), man-marking (+0x1aa, 0x105, or drop), pressing threshold
/// (1.5 or ~2, reduced without +0x106/+0x10a and by the draw against +0x1a2). Goalkeeper: picks up and comments
/// (0x1f46, 0x103), comes out (0x103) or returns (0x71). The others: deep (0x78), advance (0x6b), go to the ball
/// (0x100; on it picks up and comments 0x1f46/0x2113, or 0x104), stay in the opponent's area (0x104), cell (0x68/0x76)
/// Original: `move_off_ball` (0x714d40, match_pl.cpp, ~4.7 KB (outside the attacking band; inside it, with the ball in play, it is 0x713fc0), thiscall).
///
/// [pt] { 0x714d40 } (match_pl.cpp): ver `off_ball_move_injected`.
pub(crate) unsafe extern "thiscall" fn move_off_ball(j: usize) {
    off_ball_move_injected(j, &mut |rotina, this, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
            f(this, 0)
        }
        0x6f0630 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_TAKE_POSSESSION);
            f(this);
            0
        }
        0x6d6f70 => {
            add_match_event(this, [a[0], a[1], a[2], a[3], a[4], u32::MAX, 0]);
            0
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(this, a[0] as i8, a[1] as i8, a[2] as i32, 0, 0)
        }
    })
}

/// A player without the ball chooses the action (`+0x198`) and the target cell: `0x713fc0` (ret 4, `match_pl.cpp`) with
/// the calls that touch the game injected (`0x6f09f0` reposition, `0x6f46b0` move, `0x6f5b80` advance).
///
/// Ball out of play (`+0x8ea9` and `+0x8eae` = −1, but not both together): goes to the formation cell (`0x6c2a40`,
/// 0x68). Set piece: the taker (`+0xf57a`) goes to the spot (0x100, unless random draw(20) + 0x107 = 0); without a
/// taker, the one the side points to (`0x6cfda0`, without `+0x1c5`) too.
///
/// In open play: near the opponent's area and outside the centre (or far from the ball by the table, > 2), or with the
/// position mark 0x200000 (random draw by player sheet `+0x1a` against random draw(5)), tries to move up into a free
/// cell (`0x6bce80`, 0x6a, `+0x35` goes up 7) if random draw(20 with mark 0x40 of `table_a`, otherwise 50) < (1 − 0x35
/// × c) × 0x101 × 0x107.
///
/// Otherwise it starts from the cell of `0x6c2be0` (x by `0x715f50`): wingers (position mark 34) go to the corner by
/// the ball (0x80 or 0x800 of `table_a`); the row goes to the byline (half the x and row 0 or 11) if the ball on the
/// side (`+6`) or mark 0x800000 wins the random draw against 0x107, or if the row allows it (`0x6cb700`, 0x109); the
/// row moves by the side's difference (`0x6c4180`). Marking someone (`+0x1aa`, 0x105): follows his cell, one step ahead
/// if random draw(21) exceeds his 0xad (outside the band, `0x6cb930`, or with position mark 4, not).
///
/// Otherwise advances one row (`0x6f5b80`, 0x6b) by the player's difference (`0x6c4130`); already in the cell: stays
/// (0x76) or looks for a free cell (0x6a/0x69); outside it, by the random draw of 0xe1 + c against 0x109, looks for a
/// free cell or goes (0x68). The original routine shows an error box if the marked player is out (−5/−1).
///
/// [pt] { 0x713fc0 } (ret 4, match_pl.cpp) com as chamadas que mexem no jogo injetadas (0x6f09f0
/// reposicionar, 0x6f46b0 mover, 0x6f5b80 avancar): o jogador sem a bola escolhe a acao (+0x198) e a
/// casa. Bola fora de jogo (+0x8ea9 e +0x8eae = -1 juntos nao): vai a casa da formacao (0x6c2a40, 0x68).
/// Bola parada: quem bate (+0xf57a) vai a marca (0x100, salvo sorteio(20) + 0x107 = 0); sem batedor,
/// quem o lado aponta (0x6cfda0, sem +0x1c5) tambem. No jogo: perto da area adversaria e fora do miolo
/// (ou longe da bola pela tabela, > 2), ou com a marca 0x200000 da posicao (sorteio pela ficha +0x1a
/// contra sorteio(5)), tenta subir para uma casa livre (0x6bce80, 0x6a, +0x35 sobe 7) se sorteio(20
/// com a marca 0x40 de `table_a`, senao 50) < (1 - 0x35 x c) x 0x101 x 0x107. Senao parte da casa de
/// 0x6c2be0 (x por 0x715f50): pontas (marca 34 da posicao) vao ao canto pela bola (0x80 ou 0x800 de
/// `table_a`); a linha vai ao fundo (metade do x e linha 0 ou 11) se a bola do lado (+6) ou a marca
/// 0x800000 sortearem contra 0x107, ou se a linha permitir (0x6cb700, 0x109); a linha anda pela
/// diferenca do lado (0x6c4180). Marcando alguem (+0x1aa, 0x105): segue a casa dele, um passo a frente
/// se o sorteio(21) passar do 0xad dele (fora da faixa, 0x6cb930, ou com a marca 4 da posicao, nao).
/// Senao avanca uma linha (0x6f5b80, 0x6b) pela diferenca do jogador (0x6c4130); ja na casa: fica (0x76)
/// ou procura casa livre (0x6a/0x69); fora dela, pelo sorteio de 0xe1 + c contra 0x109, procura casa
/// livre ou vai (0x68). A original mostra uma caixa de erro se o marcado estiver fora (-5/-1).
pub(crate) unsafe fn off_ball_decide_injected(j: usize, call: &mut dyn FnMut(usize, [u32; 5]) -> u32) {
    let m = *((j + match_player::ENGINE) as *const usize);
    let dir = read_i8(j + match_player::SIDE);
    let block = (m as isize + 0x91e2 + 6371 * dir as isize) as usize;
    let w = |o: usize| (j + o) as *mut u16;
    let ta = || if m == 0 { 0 } else { table_a(m, j) };
    let reposicionar = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, x: i8, y: i8, a: i32| {
        c(0x6f09f0, [x as u8 as u32, y as u8 as u32, a as u32, 0, 0]);
    };
    // looks for a free cell (0x6bce80 with `flags`) and moves to it with action `acao`
    let subir = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, flags: u32, action: u16, sum_35: bool| -> bool {
        let (mut x, mut y) = (0i8, 0i8);
        if exe::cm3_match_engine_choose_move_cell(m, j, &mut x, &mut y, flags) == 0 {
            return false;
        }
        if sum_35 {
            *w(match_player::MATCH_RATING) = (*w(match_player::MATCH_RATING)).wrapping_add(7);
        }
        *w(match_player::ACTION) = action;
        reposicionar(c, x, y, -1);
        true
    };
    let ousado = exe::formation_get_marking_type(block, read_i8(j + match_player::POSITION)) & 4 != 0;
    *w(match_player::ACTION) = 0xffff;
    let (a9, ae) = (read_i8(m + engine::BALL_HEIGHT), read_i8(m + engine::POSSESSION_SIDE));
    if a9 == -1 && ae != -1 {
        let (mut x, mut y) = (0u8, 0u8);
        exe::cm3_match_engine_get_reference_tactic_cell(m, j, &mut x, &mut y);
        *w(match_player::ACTION) = 0x68;
        call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, [x as u32, y as u32, 0, 0, 0]);
        return;
    }
    if ae == -1 {
        let batedor = *((m + engine::SET_PIECE_TAKER) as *const usize);
        let vai_mark = if batedor == j {
            random(0x14).wrapping_add(read_i8(j + match_player::MOMENT_MOVEMENT) as i32) != 0
        } else {
            batedor == 0 && exe::cm3_match_engine_is_player_nearest_to_ball(m, j) != 0 && read_i8(m + engine::BALL_TRANSIT_TICKS) == 0
        };
        if vai_mark {
            reposicionar(call, read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y), 0x100);
            *w(match_player::ACTION) = 0x100;
            return;
        }
    }
    // tries to move up to a free cell: random roll(20 or 50) < (1 - 0x35 x c) x 0x101 x 0x107
    let arrancar = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32| -> bool {
        let t = read_i8(j + match_player::MOMENT_MOVEMENT);
        let n = if ta() & 0x40 != 0 { 20 } else { 50 };
        let chance = ((read_f64(rdata::F64_2_5) - *((j + match_player::MATCH_RATING) as *const i16) as f64 * read_f64(rdata::F64_0_0002)) * (read_i8(j + match_player::MARKER_COUNT) as i32 * t as i32) as f64) as f32;
        let r = random(n);
        !(r as f64 >= chance as f64) && subir(c, 4, 0x6a, true)
    };
    let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let in_attack = (dir == 1 && y < 3) || (dir == 0 && y > 8);
    let mut decidido = false;
    let mut testar_c = true;
    if in_attack {
        let far = (2..=6).contains(&x) || {
            let row = (y as i32 - read_i8(m + engine::BALL_Y) as i32).abs() as i8 as isize;
            let col = (x as i32 - read_i8(m + engine::BALL_X) as i32).abs() as i8 as isize;
            // fcomp + test ah,0x41: only passes if the distance is greater than 2 (NaN does not)
            read_f32((0xa15e44 + 4 * (row * 9 + col)) as usize) as f64 > read_f64(rdata::F64_2)
        };
        if far {
            decidido = arrancar(call);
            testar_c = false;
        }
    }
    if decidido {
        return;
    }
    if testar_c {
        if exe::formation_get_player_instructions(block, read_i8(j + match_player::POSITION)) & 0x200000 != 0 {
            let r1 = random(*((*((j + match_player::PLAYER) as *const usize) + 0x1a) as *const i8) as i32);
            if r1 >= random(5) && subir(call, 0xc, 0x6a, false) {
                return;
            }
        }
        if arrancar(call) {
            return;
        }
    }
    let (mut ax, mut ay) = (0i8, 0i8);
    exe::cm3_match_engine_get_player_target_cell(m, j, &mut ax, &mut ay, 1);
    ax = (exe::match_player_bias_target_column(j, ax) as i32) as i8;
    if (exe::formation_get_player_instructions(block, read_i8(j + match_player::POSITION)) >> 32) & 4 != 0 {
        let (bx, by) = (read_i8(m + engine::BALL_X), read_i8(m + engine::BALL_Y));
        let t = ta();
        let canto = t & 0x78 != 0;
        if dir == 0 {
            if t & 0x80 != 0 && canto {
                if bx < 2 || (bx < 6 && !(10..=11).contains(&by)) {
                    (ax, ay) = (0, 11);
                }
            } else if t & 0x800 != 0 && canto && bx < 2 {
                (ax, ay) = (8, 11);
            }
        } else if t & 0x80 != 0 && canto {
            if bx < 2 {
                (ax, ay) = (8, 0);
            }
        } else if t & 0x800 != 0 && canto && (bx < 2 || (bx < 6 && !(0..=1).contains(&by))) {
            (ax, ay) = (0, 0);
        }
    }
    let n: i32 = {
        let t = ta();
        if t & 0x200 == 0 {
            0x10
        } else if t & 8 != 0 {
            0x12
        } else if t & 4 != 0 {
            0x14
        } else {
            0
        }
    };
    let ball_of_side = *((m as isize + 6 + dir as isize) as *const i8) != 0;
    let draws = if *((block + tactics_fields::FLAGS) as *const u32) & 0x1000 != 0 { ball_of_side } else { exe::formation_get_player_instructions(block, read_i8(j + match_player::POSITION)) & 0x800000 != 0 };
    let to_bottom = (draws && read_i8(j + match_player::MOMENT_MOVEMENT) as i32 > random(n)) || {
        let yj = read_i8(j + match_player::CELL_Y);
        let near = (dir == 0 && read_i8(m + engine::BALL_Y) > yj && yj > 7) || (dir == 1 && read_i8(m + engine::BALL_Y) < yj && yj < 4);
        near && read_i8(j + match_player::MOMENT_FLAIR) != 0 && {
            let v = exe::cm3_match_engine_count_players_ahead_of_row(m, yj, dir, 1) as i8 as i32;
            v < random(3) + 1
        }
    };
    if to_bottom {
        ay = if dir != 0 { 0 } else { 11 };
        ax = ((ax as i32 + 4) / 2) as i8;
    }
    if read_i8(j + match_player::MOMENT_MOVEMENT) != 0 {
        let d = exe::cm3_match_engine_get_offside_margin_for_row(m, dir as u8, ay as u8) as i8;
        if d >= 0 {
            ay = if dir == 0 { ay.wrapping_sub(d) } else { ay.wrapping_add(d) };
        }
    }
    let t = ta();
    let marked = *((j + match_player::MARKING_TARGET) as *const usize);
    if marked != 0 {
        let pm = read_i8(marked + match_player::POSITION);
        // position -5 or -1: the original shows the error box and goes on to the player’s difference
        if pm != -5 && pm != -1 {
            *w(match_player::ACTION) = 0x105;
            let (mut tx, mut ty) = (read_i8(marked + match_player::CELL_X), read_i8(marked + match_player::CELL_Y));
            if read_i8(j + match_player::MOMENT_POSITIONING) == 0 {
                // without 0x10a it does not follow the marked player: falls to the player’s difference (with 0x198 = 0x105)
            } else {
                if exe::is_in_attacking_third((dir == 0) as u8, ty) == 0 && !ousado {
                    let r = random(0x15);
                    // fcomp + test ah,0x41: only steps forward if the roll is greater (not NaN)
                    if r as f64 > *((marked + match_player::JUMPING) as *const f32) as f64 {
                        if dir == 0 && ty > 0 {
                            ty -= 1;
                        } else if dir == 1 && ty < 11 {
                            ty += 1;
                        }
                        if tx <= 2 {
                            tx += 1;
                        } else if tx >= 6 {
                            tx -= 1;
                        }
                    }
                }
                reposicionar(call, tx, ty, -1);
                return;
            }
        }
    }
    let d = exe::cm3_match_engine_get_offside_margin(m, j) as i8;
    if d >= 0 {
        let r = random(d as i32 * 5).wrapping_add(read_i8(j + match_player::MOMENT_MOVEMENT) as i32);
        if r > random(3) {
            *w(match_player::ACTION) = 0x6b;
            call(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW, [0, 0, 0, 0, 0]);
            return;
        }
    }
    let free = |c: &mut dyn FnMut(usize, [u32; 5]) -> u32, subir: &dyn Fn(&mut dyn FnMut(usize, [u32; 5]) -> u32, u32, u16, bool) -> bool| -> bool {
        (read_i8(j + match_player::MOMENT_MOVEMENT) != 0 && subir(c, 4, 0x6a, true)) || (read_i8(j + match_player::MARKER_COUNT) != 0 && t & 6 == 0 && read_i8(j + match_player::MOMENT_MOVEMENT) != 0 && subir(c, 4, 0x69, true))
    };
    if read_i8(j + match_player::CELL_X) == ax && read_i8(j + match_player::CELL_Y) == ay {
        let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
        let stays = (t == 1 && read_i8(j + match_player::MOMENT_POSITIONING) != 0) || read_i8(j + match_player::MOMENT_POSITIONING) != 0 || exe::cm3_match_engine_is_behind_own_back_line(m, j) != 0;
        if stays || !free(call, &subir) {
            *w(match_player::ACTION) = 0x76;
            reposicionar(call, x, y, -1);
        }
        return;
    }
    let n = ftol(*((j + match_player::TEAMWORK) as *const f32) as f64 + read_f32(rdata::F32_10) as f64);
    if read_i8(j + match_player::MOMENT_FLAIR) as i32 > random(n) && free(call, &subir) {
        return;
    }
    *w(match_player::ACTION) = 0x68;
    reposicionar(call, ax, ay, -1);
}

/// Decides an off-ball player's movement (formation cell, set-piece mark, runs forward, wing or full-back runs,
/// following his marked man, dropping back); calls off_ball_decide_injected with the game's move routines. The argument
/// is not read.
///
/// [pt] { 0x713fc0 } (ret 4, match_pl.cpp): ver `off_ball_decide_injected`. O argumento nao e lido.
pub(crate) unsafe extern "thiscall" fn decide_off_ball(j: usize, _a: u32) {
    off_ball_decide_injected(j, &mut |rotina, a| match rotina {
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(j, a[0] as i8, a[1] as i8);
            0
        }
        0x6f5b80 => {
            let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW);
            f(j, 0)
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(j, a[0] as i8, a[1] as i8, a[2] as i32, 0, 0)
        }
    })
}

/// `0x6f5b80` (ret 4, match_pl.cpp) with the calls injected: the player moves one row towards his side (+0x27 = 1 goes
/// up to 11, otherwise down to 0).
///
/// With `direto`, he goes (0x6f46b0) to the current column on the ball's row (the engine's `engine::OFFSIDE_LINE_SIDE1` + 1 or `engine::OFFSIDE_LINE` − 1);
/// otherwise he repositions (`player_move`) on the new row with −1, 0, 0.
///
/// Returns the call's `eax` (the callers ignore it).
///
/// [pt] { 0x6f5b80 } (ret 4, match_pl.cpp) com as chamadas injetadas: o jogador anda uma linha para o
/// lado dele (+0x27 = 1 sobe ate 11, senao desce ate 0). Com `direto`, vai (0x6f46b0) para a coluna
/// atual na linha da bola (+0x8eb0 + 1 ou +0x8eaf - 1 do motor); senao reposiciona (0x6f09f0) na
/// nova linha com -1, 0, 0. Devolve o `eax` da chamada (os chamadores ignoram).
pub(crate) unsafe fn advance_row_injected(this: usize, direct: u8, call: &mut dyn FnMut(usize, [u32; 5]) -> u32) -> u32 {
    let x = read_i8(this + match_player::CELL_X);
    let mut y = read_i8(this + match_player::CELL_Y);
    let engine = *((this + match_player::ENGINE) as *const usize);
    let sobe = read_i8(this + match_player::SIDE) == 1;
    if sobe {
        if y < 11 {
            y += 1;
        }
    } else if y > 0 {
        y -= 1;
    }
    if direct != 0 {
        let row = if sobe { read_i8(engine + engine::OFFSIDE_LINE_SIDE1).wrapping_add(1) } else { read_i8(engine + engine::OFFSIDE_LINE).wrapping_sub(1) };
        return call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, [x as u8 as u32, row as u8 as u32, 0, 0, 0]);
    }
    call(FN_MATCH_PLAYER_MOVE_TO, [x as u8 as u32, y as u8 as u32, u32::MAX, 0, 0])
}

/// Hook entry for `exe::match_player_drop_back_one_row` (advance_row_injected): the player moves one row towards his side's attack, straight to the
/// ball row when `direto` (0x6f46b0), otherwise by repositioning (`player_move`); returns the callee's eax (ignored by
/// callers).
///
/// [pt] { 0x6f5b80 } (ret 4, match_pl.cpp): ver `advance_row_injected`.
pub(crate) unsafe extern "thiscall" fn advance_row(this: usize, direct: u8) -> u32 {
    advance_row_injected(this, direct, &mut |rotina, a| {
        if rotina == 0x6f46b0 {
            let f: unsafe extern "thiscall" fn(usize, i8, i8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL);
            f(this, a[0] as i8, a[1] as i8)
        } else {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_MOVE_TO);
            f(this, a[0] as i8, a[1] as i8, -1, 0, 0)
        }
    })
}

/// `draw_moment`; 1 in 11: +0xff drops 1 above (+0x100/2 + record +0x25 + 10)/4, +0x4d adds +0x59 (which clears if 1 or
/// halves), above 50 in +1 subtracts (+1 − 50)^1.25/4 (up to 25); +0x4d within 1..10000 (prologue with a relative
/// `call`)
/// Original: `evolve_moment` (0x6f55c0, match_pl.cpp, thiscall).
///
/// [pt] { 0x6f55c0 } (match_pl.cpp): sorteia o momento (0x6f44a0); 1 em 11 vezes ainda: o byte +0xff
/// cai 1 se passar de (+0x100 / 2 + 10 + ficha +0x25) / 4; +0x4d soma +0x59 (que zera se era 1 e
/// senao cai a metade); acima de 50 em +1, tira (+1 - 50)^1,25 / 4 (no maximo 25); +0x4d fica em
/// 1..10000.
pub(crate) unsafe extern "thiscall" fn evolve_moment(this: usize) {
    exe::match_player_roll_current_ratings(this);
    if random(11) != 0 {
        return;
    }
    let ff = (this + match_player::ANGER) as *mut i8;
    let h = (read_i8(this + match_player::FRUSTRATION) as i32 / 2 + read_i8(*((this + match_player::PLAYER) as *const usize) + 0x25) as i32 + 10) / 4;
    if *ff as i32 > h {
        *ff = (*ff).wrapping_sub(1);
    }
    let f = |o: usize| (this + o) as *mut f32;
    *f(match_player::STAMINA) = (*f(match_player::STAMINA) as f64 + *f(match_player::STAMINA_BOOST) as f64) as f32;
    if *(f(match_player::STAMINA_BOOST) as *const u32) == 0x3f80_0000 {
        *(f(match_player::STAMINA_BOOST) as *mut u32) = 0;
    } else {
        *f(match_player::STAMINA_BOOST) = (*f(match_player::STAMINA_BOOST) as f64 * read_f64(rdata::F64_0_5)) as f32;
    }
    let b1 = *((this + match_player::PASSES) as *const u8);
    if b1 > 50 {
        let v = game_pow((b1 as i32 - 50) as f64, read_f64(rdata::F64_1_25)) * read_f32(rdata::F32_0_25) as f64;
        // fcom + test ah,0x41: only passes 25 when greater (NaN stays)
        let v = if v > read_f64(rdata::F64_25) { read_f32(rdata::F32_25) as f64 } else { v };
        *f(match_player::STAMINA) = (*f(match_player::STAMINA) as f64 - v) as f32;
    }
    if !(*f(match_player::STAMINA) as f64 >= read_f64(rdata::F64_1)) {
        *f(match_player::STAMINA) = 1.0;
    } else if *f(match_player::STAMINA) as f64 > read_f64(rdata::F64_10000) {
        *f(match_player::STAMINA) = 10000.0;
    }
}

/// Does the match interest any of the 16 human managers? Decided by competition: friendlies and reserve games only via
/// flagged national teams, B/U21 internationals never, friendly competitions when a human's club takes part, Intertoto
/// only in round 150, national competitions when a human's club is from that nation, and international ones via the
/// managers' national teams or continent. Returns 1 or 0.
///
/// [pt] { 0x6b61e0 } (match_day.cpp): a partida interessa a algum dos 16 tecnicos humanos (as ultimas
/// pessoas, tipo +0x5f = 1)? Pela competicao (+0x14): as duas de 0x9cf668/0x9cf684 so pelas selecoes
/// dos clubes (+0x11c da nacao); as de 0x9cf794/0x9cf798 nunca; amistoso (0x5c6710 de 0xae2c74): o
/// clube do tecnico (+0x39) esta nela (0x48db80); a de 0x9cf6f8 so na rodada 150 (0x4b0870 de +0x30);
/// com nacao (+0x5d): clube do tecnico da mesma nacao e um dos dois clubes fora da divisao dela;
/// senao pelas selecoes dos tecnicos ou pela competicao continental do grupo (0xae23a4, 0xc6 bytes).
pub(crate) unsafe extern "C" fn match_of_interest(p: usize) -> u32 {
    let g = |a: usize| *(a as *const i32);
    let comp = || *((p + fixture::COMPETITION) as *const usize);
    let id = *(comp() as *const i32);
    let of_group: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let object = || *((*(G_COMP_OBJECT_TABLE as *const usize)).wrapping_add((*(comp() as *const i32) as usize).wrapping_mul(4)) as *const usize);
    let manager = |s: i32| (*(G_STAFF_TABLE as *const usize)).wrapping_add(((g(G_STAFF_COUNT) - s - 1) as usize).wrapping_mul(110));
    let u = |a: usize| *(a as *const usize);
    if id == g(KEY_CLUB_COMP_FRIENDLY) || id == g(KEY_CLUB_COMP_RESERVE) {
        let sel = |o: usize| exe::database_is_national_team(u(p + o)) != 0 && *((u(u(p + o) + 0x53) + 0x11c) as *const u8) != 0;
        return (sel(fixture::HOME_CLUB) || sel(fixture::AWAY_CLUB)) as u32;
    }
    if id == g(KEY_CLUB_COMP_B_INTERNATIONAL) || id == g(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
        return 0;
    }
    if exe::friendly_is_friendly_competition(u(G_FRIENDLY), comp()) != 0 {
        for s in 0..16 {
            let t = manager(s);
            if read_i8(t + staff::CLASSIFICATION) == 1 && u(t + staff::CLUB_JOB) != 0 && exe::comp_includes_club(object(), u(t + staff::CLUB_JOB)) != 0 {
                return 1;
            }
        }
        return 0;
    }
    if id == g(KEY_CLUB_COMP_INTER_TOTO_CUP) && (exe::comp_util_get_base_round_id((*((p + fixture::ROUND_NAME) as *const u16) as i32) as u32) as i32) as i16 != 150 {
        return 0;
    }
    if u(comp() + 0x5d) != 0 {
        if u(G_RESERVE_COMP) != 0 && of_group(comp()) != 0 {
            return 0;
        }
        for s in 0..16 {
            let t = manager(s);
            if read_i8(t + staff::CLASSIFICATION) == 1 && u(t + staff::CLUB_JOB) != 0 && u(u(t + staff::CLUB_JOB) + 0x53) == u(comp() + 0x5d) {
                let c1c = Club(u(p + fixture::HOME_CLUB));
                let cm = comp();
                if u(c1c.addr() + club::DIVISION) != cm && u(c1c.addr() + club::RESERVE_DIVISION) != cm {
                    return 1;
                }
                let x = u(t + staff::CLUB_JOB);
                return (u(x + club::DIVISION) == cm || u(x + club::RESERVE_DIVISION) == cm) as u32;
            }
        }
        return 0;
    }
    let mut edi = u(comp() + 0x59);
    if edi == 0 {
        let k = if id == g(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) || id == g(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS) {
            Some(g(KEY_CONTINENT_AFRICA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_ASIA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_NORTH_AMERICA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION) || id == g(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) || id == g(KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING) {
            Some(g(KEY_CONTINENT_EUROPE))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
            Some(g(KEY_CONTINENT_OCEANIA))
        } else if id == g(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT) || id == g(KEY_CLUB_COMP_COPA_AMERICA) {
            Some(g(KEY_CONTINENT_SOUTH_AMERICA))
        } else {
            None
        };
        if let Some(k) = k {
            edi = (*(G_CONTINENT_TABLE as *const usize)).wrapping_add((k as usize).wrapping_mul(0xc6));
        }
        if edi == 0 && id != g(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
            return 1;
        }
    }
    for s in 0..16 {
        let t = manager(s);
        if read_i8(t + staff::CLASSIFICATION) != 1 {
            continue;
        }
        if exe::comp_includes_club(object(), exe::database_get_national_team(u(t + staff::NATION))) != 0 {
            return 1;
        }
        let x = u(t + staff::CLUB_JOB);
        if x != 0 {
            let xn = u(x + club::NATION);
            if u(xn + nation::CONTINENT) == edi || u(u(t + staff::NATION) + 0x71) == edi {
                return 1;
            }
            if exe::comp_includes_club(object(), exe::database_get_national_team(xn)) != 0 {
                return 1;
            }
        }
    }
    0
}

/// Computes the player's in-match performance: recalculates match attributes, the positional penalty word +0x37
/// (tactical tables A and B against the position ratings +0x10..+0x1a of his player data; with `marcar`, counts on the
/// engine side byte +0xaf), the bytes +0x10e..+0x117 and the derived words +0x11a..+0x196, then rolls the moment
/// attributes.
///
/// [pt] { 0x6eecb0 } (ret 4, match_pl.cpp, ~6,4 KB): desempenho do jogador na partida. Atualiza os
/// atributos (0x6ec010), calcula a palavra +0x37 (posicao nas tabelas taticas A e B contra os
/// bytes de +0x10 a +0x1a da ficha; com `marcar`, conta no byte +0xaf do lado do motor), os bytes
/// +0x10e..+0x117 e as palavras derivadas de +0x11a a +0x196 (63 delas ajustadas por +0x37 numa
/// tabela de casos) e sorteia os atributos do momento (0x6f44a0).
pub(crate) unsafe extern "thiscall" fn match_performance(this: usize, marcar: u8) {
    let q = |a: usize| read_f64(a);
    let f = |a: usize| read_f32(a) as f64;
    let engine = || *((this + match_player::ENGINE) as *const usize);
    let sheet = || *((this + match_player::PLAYER) as *const usize);
    let fb = |o: usize| (read_i8((sheet()) + o) as i32);
    let ta = || if engine() != 0 { table_a(engine(), this) } else { 0 };
    let tb = || if engine() != 0 { table_b(engine(), this) } else { 0 };
    let p37 = (this + 0x37) as *mut i16;
    let soma37 = move |v: i32| *p37 = (*p37).wrapping_add(v as i16);
    let word = |o: usize, v: i32| *((this + o) as *mut i16) = v as i16;
    exe::match_player_calc_match_attributes(this);
    let bl: i8 = if (read_i8((sheet()) + 0xf) as i32) > 15 { 20 } else { ftol((read_i8((sheet()) + 0xf) as i32) as f64 * q(0x969040) + q(0x9688a0)) as i8 };
    *p37 = 0;
    // the position in table A against the attributes at +0x10..+0x1a
    let greater_13_14 = || core::cmp::max(fb(0x14) - 5, fb(0x13));
    let x: Option<f64> = if ta() & 1 != 0 {
        soma37((bl as i16 as i32) * 2 - 40);
        None
    } else if ta() & 2 != 0 {
        Some((20 - fb(0x10)) as f64)
    } else if ta() & 4 != 0 {
        Some((20 - fb(0x11)) as f64)
    } else if ta() & 8 != 0 {
        if ta() & 0x800 == 0 && ta() & 0x80 == 0 {
            Some((20 - fb(0x12)) as f64)
        } else {
            let b_880 = || tb() & 0x800 != 0 || tb() & 0x80 != 0;
            if b_880() && (tb() & 8 != 0 || tb() & 0x10 != 0 || tb() & 0x20 != 0 || tb() & 0x40 != 0) {
                Some((20 - fb(0x16)) as f64)
            } else if b_880() && (tb() & 4 != 0 || tb() & 2 != 0) {
                Some((20 - fb(0x11)) as f64)
            } else {
                Some((20 - fb(0x12)) as f64)
            }
        }
    } else if ta() & 0x10 != 0 {
        Some((20 - greater_13_14()) as f64)
    } else if ta() & 0x20 != 0 {
        if ta() & 0x800 != 0 || ta() & 0x80 != 0 {
            Some((20 - fb(0x14)) as f64)
        } else {
            let side = read_i8(this + match_player::SIDE) as isize;
            let m = exe::formation_get_player_instructions((engine() as isize + 0x91e2 + 0x18e3 * side) as usize, read_i8(this + match_player::POSITION));
            let v = if m as u32 & 0x200000 != 0 { fb(0x1a) * fb(0x14) } else { fb(0x15) * fb(0x14) };
            Some(q(0x968ea8) - v as f64 * q(0x967888) + q(0x968968))
        }
    } else if ta() & 0x40 != 0 {
        Some((20 - fb(0x15)) as f64)
    } else {
        None
    };
    if let Some(x) = x {
        soma37(ftol(x * q(0x96d740)) as i16 as i32);
    }
    if ta() & 0x80 != 0 {
        soma37(fb(0x18) - 20);
    } else if ta() & 0x100 != 0 {
        soma37(core::cmp::max(fb(0x19), fb(0x18) / 2) - 20);
    } else if ta() & 0x200 != 0 {
        soma37(fb(0x19) - 20);
    } else if ta() & 0x400 != 0 {
        soma37(core::cmp::max(fb(0x19), fb(0x17) / 2) - 20);
    } else if ta() & 0x800 != 0 {
        soma37(fb(0x17) - 20);
    }
    if ta() & 0x1000 != 0 {
        soma37(fb(0x1a) - 20);
    }
    // table B, when it differs from A
    if tb() != ta() {
        let tres_quartos = |v: i32| soma37(ftol(v as f64 * q(0x969530)) as i16 as i32);
        let b_altas = || tb() & 0x100 != 0 || tb() & 0x200 != 0 || tb() & 0x400 != 0;
        let mark = || (engine() as isize + 0xaf + read_i8(this + match_player::SIDE) as isize) as *mut u8;
        if tb() & 2 != 0 {
            tres_quartos(20 - fb(0x10));
        } else if tb() & 4 != 0 {
            tres_quartos(20 - fb(0x11));
        } else if tb() & 8 != 0 {
            tres_quartos(20 - fb(0x12));
        } else if tb() & 0x10 != 0 {
            tres_quartos(20 - greater_13_14());
        } else if tb() & 0x20 != 0 {
            tres_quartos(20 - fb(0x14));
            if marcar != 0 && ta() & 0x40 == 0 && ta() & 0x20 == 0 && b_altas() {
                *mark() = (*mark()).wrapping_add(1);
            }
        } else if tb() & 0x40 != 0 {
            tres_quartos(20 - fb(0x15));
            if marcar != 0 && ta() & 0x40 == 0 && b_altas() {
                *mark() = (*mark()).wrapping_add(2);
            }
        }
    }
    let middle = |v: i32| soma37(v / 2 - 10);
    if tb() & 0x80 != 0 {
        middle(fb(0x18));
    } else if tb() & 0x100 != 0 {
        middle(core::cmp::max(fb(0x19), fb(0x18) / 2));
    } else if tb() & 0x200 != 0 {
        middle(fb(0x19));
    } else if tb() & 0x400 != 0 {
        middle(core::cmp::max(fb(0x19), fb(0x17) / 2));
    } else if tb() & 0x800 != 0 {
        middle(fb(0x17));
    }
    *p37 = ftol((q(0x96d730) - (read_f32(this + match_player::VERSATILITY) as f64) * q(0x96d738)) * *p37 as f64) as i16;
    let w37 = move || *p37 as i32;

    // bytes +0x10e..+0x117 (no minimo 2)
    let min2 = |v: f64, two: f64| if v > two { v } else { two };
    let byte_em = |o: usize, v: f64| *((this + o) as *mut u8) = ftol(v) as u8;
    let d3 = || (w37() / 3) as f64;
    byte_em(0x10e, min2((read_f32(this + match_player::PACE) as f64) + f(0x96d228), f(0x9689a8)));
    byte_em(0x110, min2(d3() + (read_f32(this + match_player::POSITIONING) as f64) + f(0x96d228), f(0x9689a8)));
    byte_em(0x111, min2(d3() + (read_f32(this + match_player::MOVEMENT) as f64) * q(0x9689d8) + q(0x9688a0), q(0x968950)));
    byte_em(0x112, min2(d3() + (read_f32(this + match_player::MARKING) as f64) + f(0x96d228), f(0x9689a8)));
    byte_em(0x113, min2(game_pow(read_i8(this + match_player::MATCH_FORM) as f64 + (read_f32(this + match_player::FLAIR) as f64) - f(0x968a78), q(0x968950)) * q(0x967888), q(0x968950)));
    byte_em(0x114, min2(d3() + (read_f32(this + match_player::VISION) as f64) * q(0x9689b0) + q(0x9688a0), q(0x968950)));
    byte_em(0x115, min2(d3() + (read_f32(this + match_player::ANTICIPATION) as f64) + f(0x968990), f(0x9689a8)));
    byte_em(0x116, min2((read_f32(this + match_player::WORK_RATE) as f64) + f(0x9689a8), f(0x9689a8)));
    byte_em(0x117, min2((read_f32(this + match_player::AGILITY) as f64) + f(0x968990), f(0x9689a8)));
    byte_em(0x10f, min2((read_f32(this + match_player::DECISIONS) as f64) + f(0x9689a8), f(0x9689a8)));

    // palavras derivadas: T = x*10 + 0,5; Q = (x^2 + 1)/2; D = 2x*10 + 0,5 (truncados)
    let t = |o: usize| ftol((read_f32(this + o) as f64) * q(0x9688b0) + q(0x968968)) as i16 as i32;
    let qd = |o: usize| ftol(((read_f32(this + o) as f64) * (read_f32(this + o) as f64) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let d = |o: usize| ftol(((read_f32(this + o) as f64) + (read_f32(this + o) as f64)) * q(0x9688b0) + q(0x968968)) as i16 as i32;
    let blq = ftol(((bl as i32 as f64) * (bl as i32 as f64) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let v = (blq + t(match_player::HANDLING) + t(match_player::POSITIONING) + t(match_player::JUMPING) + t(match_player::DECISIONS) + t(match_player::ANTICIPATION)) / 6;
    word(0x11a, v);
    word(0x11c, v);
    word(0x11e, t(match_player::HEADING));
    word(0x120, t(match_player::HEADING));
    word(0x122, (t(match_player::BALANCE) + t(match_player::HEADING)) / 2);
    word(0x124, (qd(match_player::AGILITY) + qd(match_player::FLAIR) + qd(match_player::TECHNIQUE) + qd(match_player::BALANCE) + 100) / 5);
    word(0x18c, t(match_player::HEADING));
    word(0x190, (qd(match_player::BRAVERY) + qd(match_player::AGILITY)) / 2);
    word(0x18e, (d(match_player::FINISHING) + 300) / 5);
    word(0x126, t(match_player::CROSSING));
    word(0x128, t(match_player::CROSSING));
    word(0x12a, t(match_player::CROSSING));
    word(0x12c, (t(match_player::PASSING) + 200) / 2);
    word(0x196, (t(match_player::PASSING) + 200) / 2);
    word(0x12e, qd(match_player::PASSING));
    word(0x130, (d(match_player::PASSING) + 200) / 3);
    word(0x132, qd(match_player::TECHNIQUE));
    word(0x134, t(match_player::TECHNIQUE));
    word(0x136, (qd(match_player::TECHNIQUE) + t(match_player::TECHNIQUE)) / 2);
    word(0x138, (t(match_player::TECHNIQUE) + t(match_player::AGILITY) + t(match_player::BALANCE) + t(match_player::STRENGTH)) / 4);
    word(0x13a, (t(match_player::TECHNIQUE) + t(match_player::BALANCE) + t(match_player::STRENGTH)) / 3);
    word(0x13c, ftol((qd(match_player::TECHNIQUE) + d(match_player::TECHNIQUE) + t(match_player::ACCELERATION)) as f64 * q(0x96d728)) as i32);
    word(0x13e, ftol((d(match_player::PACE) + d(match_player::DRIBBLING) + qd(match_player::DRIBBLING) + t(match_player::TECHNIQUE)) as f64 * q(0x96d720)) as i32);
    word(0x142, (t(match_player::AGILITY) + t(match_player::BALANCE)) / 2);
    word(0x146, t(match_player::FINISHING));
    word(0x148, t(match_player::FINISHING));
    // the square of +0xe5 stays on the x87 stack between the following calculations
    let e5q = (read_f32(this + match_player::TECHNIQUE) as f64) * (read_f32(this + match_player::TECHNIQUE) as f64);
    let a = ftol((e5q + q(0x967890)) * q(0x968968)) as i16 as i32;
    let t99 = t(match_player::FINISHING);
    word(0x14a, (a + t99) / 2);
    word(0x14c, (t(match_player::TECHNIQUE) + t99) / 2);
    let bq = ftol(((e5q + e5q) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let d99 = d(match_player::FINISHING);
    let tc5 = t(match_player::PASSING);
    word(0x150, (bq + d99 + tc5) / 5);
    word(0x14e, (bq + d99 + tc5) / 5);
    word(0x152, (qd(match_player::FLAIR) + d99 + bq + tc5) / 6);
    word(0x154, (t(match_player::STRENGTH) + t99) / 2);
    word(0x156, qd(match_player::PASSING));
    word(0x192, qd(match_player::PASSING));
    word(0x158, (a + tc5) / 2);
    word(0x15a, (a + tc5) / 2);
    word(0x194, t(match_player::HEADING));
    let ddq = (read_f32(this + match_player::TACKLING) as f64) * (read_f32(this + match_player::TACKLING) as f64);
    let c = ftol(((ddq + ddq) + q(0x967890)) * q(0x968968)) as i16 as i32;
    let t81 = t(match_player::BALANCE);
    let t7d = t(match_player::ANTICIPATION);
    word(0x15c, (t81 + t7d + c) / 4);
    word(0x140, (qd(match_player::PACE) + t81 + t7d + c) / 5);
    word(0x15e, ftol((ddq + q(0x967890)) * q(0x968968)) as i32);
    word(0x160, ftol(qd(match_player::BRAVERY) as f64 * q(0x96d718)) as i32);
    let t79 = t(match_player::AGILITY);
    word(0x162, ftol((t7d + t79) as f64 * q(0x968e00)) as i32);
    word(0x166, (qd(match_player::STRENGTH) + t81) / 2);
    let bq2 = ftol(((bl as i32 * bl as i32) as f64 + q(0x967890)) * q(0x968968)) as i16 as i32;
    let ba5 = bq2 + t(match_player::HANDLING);
    word(0x168, (ba5 + t79) / 3);
    word(0x16a, (ba5 + t79) / 3);
    word(0x16c, (t(match_player::BRAVERY) + ba5) / 3);
    word(0x16e, 200);
    let dd9 = d(match_player::STRENGTH);
    word(0x178, 200);
    word(0x172, 400);
    word(0x174, 400);
    word(0x170, (dd9 + 400) / 2);
    word(0x176, (dd9 + 400) / 2);
    word(0x17a, 200);
    word(0x17c, 200);
    word(0x17e, 200);
    word(0x180, (t(match_player::PENALTIES) + 200) / 2);
    word(0x182, qd(match_player::CORNERS));
    word(0x184, qd(match_player::FREE_KICKS));
    word(0x144, qd(match_player::JUMPING));
    word(0x186, t(match_player::THROW_INS));
    word(0x188, qd(match_player::FREE_KICKS));
    word(0x164, 200);
    word(0x18a, (qd(match_player::BALANCE) + qd(match_player::STRENGTH)) / 2);

    // the 63 words of +0x11a adjusted by +0x37 (the original’s case table)
    const CASOS: [u8; 63] = [
        0, 0, 1, 2, 2, 2, 3, 3, 3, 1, 3, 3, 1, 1, 1, 4, 4, 4, 4, 1, 5, 6, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
        3, 1, 1, 1, 1, 1, 1, 7, 7, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 5, 2, 5, 5, 5, 2, 1, 6, 3, 3, 1,
    ];
    for (k, &caso) in CASOS.iter().enumerate() {
        let p = (this + match_player::MOVE_SKILL + 2 * k) as *mut i16;
        let w = *p as f64;
        let w37f = w37();
        let v = match caso {
            0 => ((w37f + 25) * 10) as f64 + w * q(0x968e18),
            1 => ((w37f + 45) * 5) as f64 + w * q(0x968968),
            2 => ((w37f * 3) as f64 + f(0x96d710)) + w * q(0x96d708),
            3 => w * q(0x96d700) + w37f as f64 * q(0x968e18) + q(0x96d520),
            4 => w * q(0x968ec8) + w37f as f64 * q(0x968e18) + q(0x96d520),
            5 => w * q(0x96d640) + q(0x968ff8),
            6 => w * q(0x968e38),
            _ => ((w37f + 30) * 7) as f64 + w * q(0x96a6e8),
        };
        *p = ftol(v) as i16;
    }
    exe::match_player_roll_current_ratings(this);
}

/// recomputes word +0x3b and the 34 f32 fields from +0x71 to +0xf5 of the player: the designated player's rating
/// (`player_rating`, slot `exe::tactics_find_captain_slot`), age, days alive, exe::contract_manager_get_contract_for_club(`exe::contract_manager_get_contract_for_club`, +0x45 bits, +0x10/+0x14 history), morale, tactical
/// tables A/B (`table_a`/`table_b`), formation (`side_centre_backs_without_conflict`/`side_slots_without_conflict`), managers and preferences
/// (`exe::staff_is_favoured_by`/`exe::staff_is_disliked_by`/`exe::staff_dislikes_club`/`exe::staff_favours_club`), powers (0x945ff0) and random draws. Only the player record changes
/// Original: `update_attributes` (0x6ec010, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ec010 } (match_pl.cpp, ~11 KB): atributos do jogador na partida (campos f32 de +0x7d a
/// +0xf5 e a palavra +0x3b), pela nota do designado, idade, contrato, moral, cansaco e taticas.
pub(crate) unsafe extern "thiscall" fn update_attributes(this: usize) {
    let q = |a: usize| read_f64(a);
    let f = |a: usize| read_f32(a) as f64;
    let wr = |o: usize, v: f64| *((this + o) as *mut f32) = v as f32;
    let person = || *((this + match_player::STAFF) as *const usize);
    let sheet = || *((this + match_player::PLAYER) as *const usize);
    let engine = || *((this + match_player::ENGINE) as *const usize);
    let ta = || if engine() != 0 { table_a(engine(), this) } else { 0 };
    let tb = || if engine() != 0 { table_b(engine(), this) } else { 0 };
    let n_persons = || *(G_STAFF_COUNT as *const i32);
    let club_side = || {
        let t = *((engine() + 0x4792) as *const usize);
        *((t + if read_i8(this + match_player::SIDE) != 0 { 0x20 } else { 0x1c }) as *const usize)
    };
    let rating: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_MATCH_PLAYER_CALC_INFLUENCE_SCORE);
    let rating = |j: usize| rating(j) as i8 as i32;
    let designado = |m: usize| {
        let t = *((m + engine::MATCH) as *const usize);
        let l = read_i8(this + match_player::SIDE);
        let (c, fl) = if l != 0 { (*((t + fixture::AWAY_CLUB) as *const usize), (read_i8(t + fixture::AWAY_SIDE_MARK) == 0) as u32) } else { (*((t + fixture::HOME_CLUB) as *const usize), (read_i8(t + fixture::HOME_SIDE_MARK) == 0) as u32) };
        let k = exe::tactics_find_captain_slot(*(G_TACTICS as *const usize), c, fl, 0) as i32;
        (m as isize + 0x4796 + 0x1be * (k + 20 * l as i32) as isize) as usize
    };

    let (mut l28, mut l18, mut l24, mut l1c, mut l14) = (0f32, 0f32, 0f32, 0f32, 0f32);
    let l12 = read_i8(person() + 0x18) as i32;

    // the designated player (marked slot of the side’s tactic)
    let j = designado(engine());
    if j != 0 && *((j + match_player::PLAYER) as *const usize) != 0 {
        l18 = ((rating(j) as f64 - q(0x968e08)) * q(0x96d6f8)) as f32;
        let j2 = designado(engine());
        let fj = *((j + match_player::PLAYER) as *const usize);
        let x = ((read_i8(fj + player_sheet::POSITIONING) as i32) as f64 + (read_i8(fj + player_sheet::POSITIONING) as i32) as f64 + (read_i16((*((j2 + match_player::PLAYER) as *const usize)) + 5) as i32) as f64) * q(0x967880);
        let l34 = if x < q(0x9689a0) { 0.0 } else if !(x <= q(0x968ea8)) { 20.0 } else { x };
        l28 = (rating(j) as f64 * l34 * q(0x968ef0)) as f32;
        let mj = *((j + match_player::ENGINE) as *const usize);
        l24 = if mj != 0 && table_a(mj, j) == 1 {
            (rating(j) - 20) as f64 * q(0x968968)
        } else {
            ((rating(j) * (read_i8((*((j + match_player::PLAYER) as *const usize)) + 0x44) as i32)) as f64 * q(0x967888) - q(0x968e08)) * q(0x968968)
        } as f32;
        let age = (read_i8((*((j + match_player::STAFF) as *const usize)) + 0x18) as i32);
        let m = |v: f32, c: usize| (v as f64 * f(c)) as f32;
        if age < 18 {
            l18 = m(l18, 0x968f18);
            l24 = m(l24, 0x96d248);
            l28 = m(l28, 0x96d248);
        } else if age < 21 {
            l18 = m(l18, 0x96d248);
            l24 = m(l24, 0x96d364);
            l28 = m(l28, 0x96d364);
        } else if age < 24 {
            l18 = m(l18, 0x96a32c);
            l24 = m(l24, 0x96a32c);
            l28 = m(l28, 0x96a32c);
        } else if age > 30 && age < 36 {
            l18 = m(l18, 0x96d3b8);
        }
        let pj = *((j + match_player::STAFF) as *const usize);
        let read = |o: usize| core::ptr::read_unaligned((pj + o) as *const u32);
        if exe::date_days_since(G_CURRENT_DATE, read(staff::DATE_JOINED_CLUB), read(0x42)) < (50 - (read_i8(pj + staff::LOYALTY) as i32)) * 10 {
            l18 = m(l18, 0x96a32c);
        }
        if (read_i8(pj + staff::PROFESSIONALISM) as i32) < 10 {
            l18 = (((read_i8(pj + staff::PROFESSIONALISM) as i32) as f64 * f(0x96bac8) + f(0x96a32c)) * l18 as f64) as f32;
        }
    }
    let l2c = if exe::database_is_national_team(club_side()) == 0 {
        exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person(), club_side())
    } else {
        0
    };
    let side = read_i8(this + match_player::SIDE) as i32;
    l18 = ((((read_i16((engine()) + ((0x1ce + 2 * side) as usize)) as i32) - 100) as f64 * q(0x96d260) + q(0x967890)) * l18 as f64) as f32;
    let p = person();
    let greater = || core::cmp::max(read_i8(p + staff::PRESSURE), read_i8(p + staff::DETERMINATION)) as i32;
    let w33 = (read_i16(this + 0x33) as i32);
    let mut bl: i8 = if w33 > 10 { (w33 / 4) as i8 } else { (((10 - w33) * (greater() - 10)) / 10) as i8 };
    let w31 = (read_i16(this + 0x31) as i32);
    let dl = if w31 < 0 { ((greater() - 10) * w31) / -10 } else { core::cmp::min((((read_i8(p + staff::PROFESSIONALISM) as i32) - 10) * w31) / 10, 10) };
    bl = bl.wrapping_add(dl as i8);
    bl = if ta() == 1 { bl.clamp(-5, 15) } else { bl.clamp(-10, 10) };
    let b1ad = read_i8(engine() + ((0x1ad + side) as usize));
    if b1ad != 0 {
        let t = *((engine() + 0x4792) as *const usize);
        let al = b1ad as i32;
        let base = |c: usize| {
            (al as f64 + q(0x9688b0)) * f(0x96d490) + q(c) - ((read_i8(p + staff::PROFESSIONALISM) as i32) - 10) as f64 * q(0x96a7a8)
                - ((read_i8((sheet()) + 0x2c) as i32) - 10) as f64 * q(0x9689e0) - l18 as f64 * q(0x968968)
        };
        let x = if t != 0 && (read_i8(t + fixture::CATEGORY) == 8 || read_i8(t + fixture::CATEGORY) == 10) {
            base(0x968e38)
        } else {
            let mut s = f(0x96d6f4);
            if read_i8(engine() + 0) == 0 {
                let wt = (read_i16((*((t + fixture::COMPETITION) as *const usize)) + 0x69) as i32);
                if wt > 12 {
                    s = (wt - 10) as f64 * f(0x96d6f0) + f(0x96d6f4);
                }
            }
            base(0x968e18) / s
        };
        l1c = (((al / 2) as f64 + q(0x968e18) + x) * 2.0) as f32;
        if ta() == 1 {
            l1c = (l1c as f64 * f(0x96d6ec)) as f32;
        }
        if !(l1c as f64 >= q(0x967890)) {
            l1c = 1.0;
        }
    }
    let l30 = (read_i8((sheet()) + 0x21) as i32) as f32;
    let tabela79 = |id: i32| *((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(0xa) as *const u8) as i32;
    let teto30 = |v: i32| if v as f64 > q(0x968dd8) { v as f64 } else { q(0x968dd8) };
    if p != 0 && *(p as *const i32) >= n_persons() {
        let v = (read_i16((sheet()) + 5) as i32) as f64 + l18 as f64 * q(0x968f98) - l1c as f64;
        *((this + match_player::EFFECTIVE_ABILITY) as *mut i16) = ftol(v) as i16;
        let h = (exe::database_get_contract_rating_modifier(p, club_side()) as i8 as i32) / 2;
        let a = h as f64 + (l30 as f64 - l1c as f64);
        if (read_i8(this + 0x118) as f64) < a {
            l14 = (read_i8(this + 0x119) as f64 * teto30((read_i16(this + match_player::EFFECTIVE_ABILITY) as i32)) * f(0x96d6e8)) as f32;
        }
    } else {
        let id = *(p as *const i32);
        let v = (read_i16((sheet()) + 5) as i32) as f64 + l18 as f64 * q(0x968f98) - l1c as f64 - tabela79(id) as f64 + (read_i8(this + match_player::FRUSTRATION) as i32 / 2) as f64;
        *((this + match_player::EFFECTIVE_ABILITY) as *mut i16) = ftol(v) as i16;
        let h = (exe::database_get_contract_rating_modifier(p, club_side()) as i8 as i32) / 2;
        let h17 = read_i8(this + match_player::MATCH_FORM) as i32 / 2 - 5;
        let a = (h as f64 + h17 as f64) + (l30 as f64 - l1c as f64) + bl as f64 + read_i8(this + match_player::FRUSTRATION) as f64;
        if (read_i8(this + 0x118) as f64) > a {
            l14 = (read_i8(this + 0x119) as f64 * teto30((read_i16(this + match_player::EFFECTIVE_ABILITY) as i32)) * f(0x96d6e8)) as f32;
        }
    }
    if exe::database_is_national_team(*((this + match_player::CLUB) as *const usize)) != 0 && ta() != 1 {
        l14 = (l14 as f64 + f(0x968990)) as f32;
    }
    *((this + match_player::EFFECTIVE_ABILITY) as *mut i16) = ftol(((read_i8(this + match_player::MATCH_FORM) as i32 - 10) * read_i8(this + 0x118) as i32) as f64 * q(0x969020) + (read_i16(this + match_player::EFFECTIVE_ABILITY) as i32) as f64) as i16;
    // youth bonus (+0x14 of the tactics frame, cell +0x69) in L14
    {
        let t = *((engine() + 0x4792) as *const usize);
        if t != 0 {
            let t14 = *((t + fixture::COMPETITION) as *const usize);
            if t14 != 0 && read_i8(t + fixture::CATEGORY) != 6 {
                let ax = (read_i16(t14 + competition::REPUTATION) as i32);
                let pv = person();
                if ax != 0 && !(pv != 0 && *(pv as *const i32) >= n_persons()) && (read_i8(pv + staff::YEAR_OF_BIRTH) as i32) < 24 {
                    let age = read_i8(pv + staff::YEAR_OF_BIRTH) as i32;
                    let dx = ax.wrapping_mul(500) as i16 as i32;
                    let mut e = (read_i8(pv + staff::PRESSURE) as i32).wrapping_mul(150);
                    if read_i8(sheet() + 0xf) != 20 {
                        if age > 21 {
                            e += 750;
                        } else if age > 18 {
                            e += 250;
                        }
                    }
                    let ecx = e as i16 as i32;
                    let d = dx - ecx;
                    if d > 4500 && (read_i16((sheet()) + 0xd) as i32) < d {
                        let edi = (read_i16((sheet()) + 0xd) as i32) - dx + ecx;
                        let add = if ta() == 1 {
                            if edi <= 2500 { f(0x96b008) } else { f(0x968990) }
                        } else if edi > 2500 {
                            f(0x969078)
                        } else if edi > 1250 {
                            f(0x968990)
                        } else {
                            f(0x96b008)
                        };
                        l14 = (l14 as f64 + add) as f32;
                    }
                }
            }
        }
    }
    let p3b = (this + match_player::EFFECTIVE_ABILITY) as *mut i16;
    if ta() == 1 {
        let v = ftol((read_i16((sheet()) + 5) as i32) as f64 * q(0x969040)) as i16;
        if *p3b < v {
            *p3b = v;
        }
        let pv = person();
        if (pv == 0 || *(pv as *const i32) < n_persons()) && *p3b < 8 {
            *p3b = 8;
        }
    }
    let lim = if ta() == 1 { ((read_i16((sheet()) + 7) as i32) as i16).wrapping_add(10) } else { (read_i16((sheet()) + 7) as i32) as i16 };
    if *p3b > lim {
        *p3b = lim;
    }
    *p3b = ftol(*p3b as f64 * q(0x968968) + q(0x96d6e0)) as i16;
    let maior150 = *p3b > 150;
    let w3 = || *p3b as f64;
    let zera = |o: usize| *((this + o) as *mut u32) = 0;
    // fst + comparison with the value not yet rounded
    let guarda = |o: usize, v: f64| {
        wr(o, v);
        // fcomp + test ah,1: NaN counts as less (C0 set), as in the original
        if !(v >= q(0x9689a0)) {
            zera(o);
        }
    };
    let pow = |o: usize, threshold: f64, e: f64| wr(o, game_pow((read_f32(this + o) as f64) - threshold, e) + threshold);
    let jx = |l: i32| *((engine() as isize + 0x904d + 208 * l as isize) as *const usize);
    let of_bench = || {
        // the side’s manager (+0x904d) and his person, when the club is not human
        if exe::human_manager_is_human_club(club_side(), 0, (1) as u32, (0) as u32, 0) != 0 {
            return 0;
        }
        let x = jx(read_i8(this + match_player::SIDE) as i32);
        if x == 0 { 0 } else { *((x + 0x69) as *const usize) }
    };
    let par_880 = || ta() & 0x880 != 0 && tb() & 0x880 != 0;
    let form_b20 = || exe::cm3_match_engine_side_has_no_wide_defenders(engine(), (read_i8(this + match_player::SIDE) == 0) as u8) != 0 && par_880() && (ta() & 0x60 != 0 || tb() & 0x60 != 0);
    let form_af0 = || exe::cm3_match_engine_side_has_no_wide_midfield_or_attack(engine(), read_i8(this + match_player::SIDE) as u8) != 0 && (ta() & 0x78 != 0 || tb() & 0x78 != 0);
    let a_20_0c = || ta() & 0x20 != 0 && ta() & 0xc != 0;
    let fb = |o: usize| (read_i8((sheet()) + o) as i32) as f64;
    let l12f = l12;

    // +0x7d
    let e1e = if l12f > 36 { fb(0x1e) * q(0x969040) } else { fb(0x1e) };
    let l4c = l14 as f64 * q(0x9689b0);
    wr(0x7d, ((e1e + e1e) + w3() - l4c) * q(0x967880));
    let pj = of_bench();
    if pj != 0 {
        let k = exe::non_player_attribute_rating(pj, read_i8(pj + 0x21)) as i8 as i32 - 13;
        wr(0x7d, k as f64 * q(0x967880) + (read_f32(this + match_player::ANTICIPATION) as f64));
    } else {
        wr(0x7d, (read_f32(this + match_player::ANTICIPATION) as f64) + q(0x96d6d8));
    }
    if !((read_f32(this + match_player::ANTICIPATION) as f64) >= q(0x9689a0)) {
        zera(match_player::ANTICIPATION);
    }
    if a_20_0c() {
        wr(0x7d, (read_f32(this + match_player::ANTICIPATION) as f64) * q(0x9689d0));
    }
    wr(0x7d, (read_f32(this + match_player::ANTICIPATION) as f64) * q(0x968968) + q(0x9688a0));
    // +0x8d
    let l3c = l14 as f64 * q(0x968e48);
    wr(0x8d, (fb(0x23) + fb(0x23) + w3() - l3c) * q(0x967880));
    if form_b20() {
        wr(0x8d, (read_f32(this + match_player::CROSSING) as f64) * q(0x968fa0));
    }
    if !((read_f32(this + match_player::CROSSING) as f64) >= q(0x9689a0)) {
        zera(match_player::CROSSING);
    }
    if (read_f32(this + match_player::CROSSING) as f64) > q(0x969508) && maior150 {
        pow(0x8d, q(0x969508), q(0x968928));
    }
    // +0x91, +0x95, +0x99
    guarda(0x91, (fb(0x24) + fb(0x24) + w3() - l14 as f64) * q(0x967880));
    let l54 = l14 as f64 * q(0x969510);
    wr(0x95, (fb(0x26) + fb(0x26) + w3() - l54) * q(0x967880));
    if a_20_0c() {
        wr(0x95, (read_f32(this + match_player::DRIBBLING) as f64) * q(0x9689d0));
    }
    if !((read_f32(this + match_player::DRIBBLING) as f64) >= q(0x9689a0)) {
        zera(match_player::DRIBBLING);
    }
    let l44 = l14 as f64 * q(0x968f80);
    guarda(0x99, (fb(0x27) + fb(0x27) + w3() - l44) * q(0x967880));
    if par_880() {
        wr(0x99, (read_f32(this + match_player::FINISHING) as f64) * q(0x96a720));
    }
    if a_20_0c() {
        wr(0x99, (read_f32(this + match_player::FINISHING) as f64) * q(0x9689d0));
    }
    if (read_f32(this + match_player::FINISHING) as f64) > q(0x969508) && maior150 && exe::database_is_national_team(*((this + match_player::CLUB) as *const usize)) == 0 {
        pow(0x99, q(0x969508), q(0x968928));
    }
    // +0xa5, +0xa9, +0xb1, +0xb5
    guarda(0xa5, (fb(0x2a) + fb(0x2a) + w3() - l14 as f64) * q(0x967880));
    guarda(0xa9, (fb(0x2b) + fb(0x2b) + w3() - l3c) * q(0x967880));
    if (read_f32(this + match_player::HEADING) as f64) > q(0x969550) && maior150 {
        pow(0xa9, q(0x969550), q(0x968978));
    }
    guarda(0xb1, (fb(0x31) + fb(0x31) + w3() - l14 as f64) * q(0x967880));
    let l5c = l14 as f64 * q(0x968968);
    guarda(0xb5, (fb(0x32) + fb(0x32) + w3() - l5c) * q(0x967880));
    let t1c = (game_pow((read_f32(this + match_player::MARKING) as f64), q(0x96d6d0)) * q(0x967888) + q(0x967890)) as f32;
    wr(0xb5, t1c as f64);
    if t1c as f64 > q(0x969508) && maior150 {
        wr(0xb5, game_pow(t1c as f64 - q(0x969508), q(0x968978)) + q(0x969508));
    }
    // L18 by byte +0x33
    let b33 = fb(0x33);
    l18 = (if l12f > 36 {
        b33 * q(0x969040)
    } else if ta() & 0x40 != 0 {
        b33
    } else if ta() & 0x20 != 0 {
        b33 * q(0x9689d0)
    } else if ta() & 0x18 != 0 {
        if read_i8(sheet() + 0x12) == 20 { b33 * q(0x969520) } else { b33 * q(0x9689d8) }
    } else {
        b33 * q(0x9689b0)
    }) as f32;
    if par_880() {
        if ta() & 0x30 != 0 {
            l18 = (l18 as f64 * q(0x96d6c8)) as f32;
        } else if ta() & 8 != 0 || tb() & 8 != 0 {
            l18 = (l18 as f64 * q(0x968fa0)) as f32;
        }
    }
    if form_b20() {
        l18 = (l18 as f64 * q(0x968fa0)) as f32;
    }
    if form_af0() {
        l18 = (l18 as f64 * q(0x96af30)) as f32;
    }
    // +0xb9
    wr(0xb9, (l18 as f64 + l18 as f64 + w3() - l4c) * q(0x967880));
    let pj = of_bench();
    if pj != 0 {
        let m = ((read_i8(pj + 0xe) as i32) as f64 * f(0x96d6c4) + f(0x96a340)) as f32;
        let k = exe::non_player_attribute_rating(pj, read_i8(pj + 0x21)) as i8 as i32 - 13;
        wr(0xb9, k as f64 * m as f64 + (read_f32(this + match_player::MOVEMENT) as f64));
    } else {
        wr(0xb9, (read_f32(this + match_player::MOVEMENT) as f64) + q(0x968fd0));
    }
    guarda(0xb9, (read_f32(this + match_player::MOVEMENT) as f64) * f(0x96d320) + f(0x96d6c0));
    // +0xbd, +0xc5, +0xc9
    guarda(0xbd, (fb(0x35) + fb(0x35) + w3() - l14 as f64) * q(0x967880));
    guarda(0xc5, (w3() + (fb(0x37) + fb(0x37)) - l54) * q(0x967880));
    if form_b20() {
        wr(0xc5, (read_f32(this + match_player::PASSING) as f64) * q(0x96d6b8));
    }
    if (read_f32(this + match_player::PASSING) as f64) > q(0x969550) && maior150 {
        pow(0xc5, q(0x969550), q(0x968928));
    }
    guarda(0xc9, (fb(0x38) + fb(0x38) + w3() - l5c) * q(0x967880));
    // L1c by byte +0x39
    let b39 = fb(0x39);
    l1c = (if l12f > 36 { b39 * q(0x969040) } else { b39 }) as f32;
    if par_880() {
        if ta() & 4 != 0 {
            l1c = (l1c as f64 * q(0x96d6c8)) as f32;
        } else if ta() & 8 != 0 {
            l1c = (l1c as f64 * q(0x96d6b0)) as f32;
        } else if ta() & 0x10 != 0 || tb() & 0xc != 0 {
            // the original routine uses L18 here
            l1c = (l18 as f64 * q(0x968fa0)) as f32;
        }
    }
    if ta() == 1 {
        if l12f < 21 || (read_i16((sheet()) + 0xd) as i32) < 3750 {
            l1c = (l1c as f64 * q(0x96af30)) as f32;
        } else if l12f < 23 {
            l1c = (l1c as f64 * q(0x96a6d8)) as f32;
        }
    }
    if (ta() & 4 != 0 || tb() & 4 != 0) && tb() != ta() {
        let pos = |x: u16| -> i32 {
            if x & 1 != 0 {
                0
            } else if x & 4 != 0 {
                1
            } else if x & 8 != 0 {
                2
            } else if x & 0x10 != 0 {
                3
            } else if x & 0x20 != 0 {
                4
            } else if x & 0x40 != 0 {
                5
            } else {
                3
            }
        };
        let d = (pos(ta()) - pos(tb())).abs();
        l1c = (l1c as f64 * if d as i8 > 1 { q(0x96a678) } else { q(0x9689d0) }) as f32;
    }
    // +0xcd
    wr(0xcd, (l1c as f64 + l1c as f64 + w3() - l3c + l28 as f64) * q(0x967880));
    let pj = of_bench();
    if pj != 0 {
        l28 = (f(0x968f18) - (read_i8(pj + 0xe) as i32) as f64 * f(0x96d6c4)) as f32;
        let k = exe::non_player_attribute_rating(pj, read_i8(pj + 0x21)) as i8 as i32 - 13;
        wr(0xcd, k as f64 * l28 as f64 + (read_f32(this + match_player::POSITIONING) as f64));
    } else {
        wr(0xcd, (read_f32(this + match_player::POSITIONING) as f64) + q(0x968fd0));
    }
    guarda(0xcd, game_pow((read_f32(this + match_player::POSITIONING) as f64), q(0x96d6d0)) * q(0x967888) + q(0x967890));
    if (read_f32(this + match_player::POSITIONING) as f64) > q(0x969508) && maior150 && ta() != 1 {
        pow(0xcd, q(0x969508), q(0x968978));
    }
    // L28 by byte +0x3a; +0xd1, +0xdd, +0xe9
    let b3a = fb(0x3a);
    l28 = (if l12f > 39 { b3a * q(0x9689b0) } else if l12f > 37 { b3a * q(0x9689d8) } else { b3a }) as f32;
    guarda(0xd1, (l28 as f64 + l28 as f64 + w3() - l14 as f64) * q(0x967880));
    guarda(0xdd, (fb(0x3e) + fb(0x3e) + w3() - l44) * q(0x967880));
    guarda(0xe9, (fb(0x41) + fb(0x41) + w3()) * q(0x967880));
    // L28 by byte +0x43 (and word +5); +0xf1
    let b43 = (read_i8((sheet()) + 0x43) as i32);
    if b43 > 10 {
        let w5 = (read_i16((sheet()) + 5) as i32);
        if w5 < 80 {
            l28 = ((b43 - 10) as f64 * q(0x969040) + q(0x9688b0)) as f32;
        } else if w5 < 125 {
            l28 = ((b43 - 10) as f64 * q(0x969520) + q(0x9688b0)) as f32;
        }
    } else {
        l28 = b43 as f32;
    }
    let x = if form_af0() { l28 as f64 * q(0x96af30) } else { l28 as f64 };
    guarda(0xf1, (x + x + w3() - l44) * q(0x967880));
    if (read_f32(this + match_player::VISION) as f64) > q(0x96d4e8) && maior150 {
        pow(0xf1, q(0x96d4e8), q(0x968978));
    }
    // +0x71 and the like (record bytes), limited by the engine type (+1: 1, 8, 32)
    let m1 = read_i8(engine() + 1) as i32;
    let s = read_i8(sheet() + 0x44).wrapping_add(read_i8(sheet() + 0x3d));
    let mut cl = read_i8(sheet() + 0x1b);
    if m1 == 1 || m1 == 8 || m1 == 32 {
        if s == 40 {
            if cl == 20 {
                cl = 19;
            }
        } else if s > 30 {
            cl = cl.min(18);
        } else if s > 25 {
            cl = cl.min(17);
        } else if s < 15 {
            cl = cl.min(16);
        } else {
            cl = cl.min(15);
        }
    }
    let v71 = cl as f64 * q(0x96a670) + q(0x96d6a8);
    wr(0x71, v71);
    if v71 > q(0x96d6a0) {
        wr(0x71, game_pow(v71 as f32 as f64 - q(0x96d6a0), q(0x968928)) + q(0x96d6a0));
    }
    let b1d = fb(0x1d);
    let v1d = if l12f > 39 { b1d * q(0x9689b0) } else if l12f > 37 { b1d * q(0x9689d8) } else { b1d };
    wr(0x79, v1d * q(0x968968) + q(0x96d6a8));
    wr(0x75, fb(0x1c) * q(0x968968));
    wr(0x81, fb(0x1f) * q(0x968968) + q(0x96d6a8));
    wr(0x85, fb(0x20) * q(0x968968) + q(0x96d6a8));
    wr(0x89, fb(0x22) * q(0x968968) + q(0x96d6a8));
    let b28 = fb(0x28);
    let v28 = if l12f > 36 { b28 * q(0x968968) } else if l12f > 34 { b28 * q(0x969040) } else { b28 };
    wr(0x9d, v28 - l14 as f64 * f(0x96afc4));
    wr(0xa1, fb(0x29) * q(0x96d698) + q(0x96d6a8));
    wr(0xad, fb(0x2e) - l14 as f64 * f(0x96d694));
    let mut al = read_i8(sheet() + 0x36);
    if m1 == 32 {
        if s == 40 {
            if al == 20 {
                al = 19;
            }
        } else if s > 35 {
            al = al.min(18);
        } else if s > 30 {
            al = al.min(17);
        } else if s < 20 {
            al = al.min(16);
        } else {
            al = al.min(14);
        }
    } else if m1 == 1 || m1 == 8 {
        if s == 40 {
            if al == 20 {
                al = 19;
            }
        } else if s > 30 {
            al = al.min(18);
        } else if s > 25 {
            al = al.min(17);
        } else if s < 15 {
            al = al.min(16);
        } else {
            al = al.min(15);
        }
    }
    let vc1 = al as f64 * q(0x96a670) + q(0x96d6a8);
    wr(0xc1, vc1);
    if vc1 > q(0x96d6a0) {
        wr(0xc1, game_pow(vc1 as f32 as f64 - f(0x96d690), q(0x968928)) + q(0x96d6a0));
    }
    wr(0xd9, fb(0x3d) * q(0x968968) + q(0x96d6a8));
    let sum_3c = if l12f > 38 {
        q(0x968ec8)
    } else if l12f > 35 {
        q(0x96d688)
    } else if l12f > 33 {
        q(0x96d680)
    } else if l12f < 20 {
        q(0x96d678)
    } else if l12f < 24 {
        q(0x96d670)
    } else if l12f < 30 {
        q(0x968eb0)
    } else {
        q(0x968988)
    };
    wr(0xd5, fb(0x3c) * q(0x96a5d0) + sum_3c);
    match m1 {
        1 => wr(0xd5, (read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64) - q(0x968968)),
        8 => wr(0xd5, (read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64) - q(0x969040)),
        32 => wr(0xd5, (read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64) - q(0x9689e0)),
        _ => {}
    }
    let t = *((engine() + 0x4792) as *const usize);
    if l12f < 32 && t != 0 {
        let c1c = Club(*((t + fixture::HOME_CLUB) as *const usize));
        if !c1c.is_null() {
            let c57 = c1c.division().addr();
            if c57 != 0 && *((t + fixture::COMPETITION) as *const usize) == c57 {
                let ax = (read_i16(c57 + competition::REPUTATION) as i32);
                if ax < 5 {
                    wr(0xd5, (read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64) + q(0x968928));
                } else if ax < 9 {
                    wr(0xd5, (read_f32(this + match_player::STAMINA_ATTRIBUTE) as f64) + q(0x967890));
                }
            }
        }
    }
    wr(0xe1, fb(0x3f));
    let mut a40 = read_i8(sheet() + 0x40);
    match m1 {
        1 => a40 = if a40 == 20 { 19 } else { a40.min(18) },
        8 => a40 = a40.min(17),
        32 => a40 = a40.min(15),
        _ => {}
    }
    wr(0xe5, a40 as f64);
    wr(0xed, fb(0x42));
    if ta() != 1 && t != 0 {
        let t14 = *((t + fixture::COMPETITION) as *const usize);
        if t14 != 0 && *(t14 as *const u32) == *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const u32) {
            let pn = *((person() + 0x1a) as *const usize);
            if pn != 0 {
                let id = *(pn as *const u32);
                let g = |a: usize| *(a as *const u32);
                // the original only spares the nation equal to the three globals at the same time
                if !(id == g(KEY_NATION_UNITED_STATES) && id == g(KEY_NATION_CANADA) && id == g(KEY_NATION_MEXICO)) {
                    wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) * f(0x96a32c));
                }
            }
            wr(0x9d, (read_f32(this + match_player::FLAIR) as f64) * f(0x96d66c));
            wr(0xf1, (read_f32(this + match_player::VISION) as f64) * f(0x96d66c));
            wr(0xb9, (read_f32(this + match_player::MOVEMENT) as f64) * f(0x96a32c));
        }
    }
    if ta() != 1 {
        let fi = sheet();
        if read_i8(fi + player_sheet::GOALKEEPER) == 20 && read_i8(fi + player_sheet::DEFENDER) < 10 && read_i8(fi + player_sheet::DEFENSIVE_MIDFIELDER) < 10 && read_i8(fi + player_sheet::MIDFIELDER) < 15 && read_i8(fi + player_sheet::ATTACKING_MIDFIELDER) < 10 && read_i8(fi + player_sheet::ATTACKER) < 10 {
            for (o, c) in [(0x9d, 0x96d424), (0xf1, 0x96d22c), (0x95, 0x96d22c), (0x91, 0x96d22c), (0x7d, 0x96d22c), (0xad, 0x96d364), (0xa9, 0x96d248), (0xcd, 0x968f18), (0xc5, 0x968f18), (0xdd, 0x96d248), (0xb9, 0x96d248)] {
                wr(o, (read_f32(this + o) as f64) * f(c));
            }
        }
    }
    // +0xf5
    let b44 = fb(0x44);
    let v44 = if l12f < 36 { b44 } else { b44 * q(0x969040) };
    let h = ((read_i8((person()) + 0x5b) as i32) / 2 + 10) as f64;
    wr(0xf5, (read_i8(this + match_player::FRUSTRATION) as f64 + v44) - l14 as f64 / h + l24 as f64 * q(0x968968));
    let p = person();
    if p != 0 && *(p as *const i32) >= n_persons() {
        return;
    }
    let mul = |o: usize, c: usize| wr(o, (read_f32(this + o) as f64) * f(c));
    if tabela79(*(p as *const i32)) > 8 && (read_i8(p + staff::PROFESSIONALISM) as i32) < 15 {
        for (o, c) in [(0x7d, 0x96b014), (0xf5, 0x96b014), (0xf1, 0x96a32c), (0x9d, 0x96a32c), (0x95, 0x96a32c), (0xc1, 0x96b014)] {
            mul(o, c);
        }
    }
    if l2c != 0 {
        let c = l2c;
        let c45 = || core::ptr::read_unaligned((c + contract::GRIEVANCES) as *const u32);
        if c45() & 0x40000 != 0 && (read_i8(p + staff::PRESSURE) as i32) + l12f < random(50) {
            for (o, k) in [(0xf5, 0x96b014), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96b014), (0x91, 0x96b014), (0xc1, 0x96b014)] {
                mul(o, k);
            }
        }
        let pa61 = *((p + staff::PLAYER) as *const usize);
        if pa61 != 0 && read_i8(c + contract::MORALE) < -25 && read_i8(pa61 + player_sheet::WORK_RATE) < 15 && (read_i8(p + staff::PROFESSIONALISM) as i32) < 10 && (read_i16(pa61 + player_sheet::CURRENT_REPUTATION) as i32) > 3750 && (read_i16(pa61 + player_sheet::CURRENT_REPUTATION) as i32) > (read_i16((club_side()) + 0x80) as i32) + 250 {
            mul(0xf5, 0x96b014);
            mul(0x7d, 0x96a32c);
            if read_i8(c + contract::MORALE) < -75 {
                for (o, k) in [(0xf1, 0x96a32c), (0x9d, 0x96a32c), (0x95, 0x96b014), (0x99, 0x96a32c), (0xc1, 0x96b014)] {
                    mul(o, k);
                }
            }
        }
        if read_i8(c + contract::GRIEVANCES) as u8 & 0x17 != 0 {
            if (read_i8(p + staff::PROFESSIONALISM) as i32) > 15 && (read_i8(p + staff::AMBITION) as i32) > 15 {
                mul(0xf5, 0x96a33c);
            } else if (read_i8(p + staff::PROFESSIONALISM) as i32) < 5 {
                mul(0xf5, 0x96a32c);
            }
        }
        if c45() & 0x2088 != 0 {
            let w0b = (read_i16((*((p + staff::PLAYER) as *const usize)) + 0xb) as i32);
            if (w0b > 4250 && (read_i8(p + staff::PROFESSIONALISM) as i32) < 10) || (read_i8(p + staff::PROFESSIONALISM) as i32) < 5 {
                mul(0xf5, 0x96a32c);
            }
        }
        if c45() & 0x38da00 != 0 {
            if (read_i8(p + staff::PROFESSIONALISM) as i32) == 1 {
                mul(0xf5, 0x96d38c);
            } else if (read_i8(p + staff::PROFESSIONALISM) as i32) < 6 {
                mul(0xf5, 0x96b014);
            }
        }
        let fraco = || (read_i8(p + staff::PRESSURE) as i32) < 5 && (read_i8(p + staff::PROFESSIONALISM) as i32) < 6 && (read_i8(p + staff::TEMPERAMENT) as i32) < 10 && (read_i16((*((p + staff::PLAYER) as *const usize)) + 0xd) as i32) > 5250;
        let strong = [(0xf5, 0x96d38c), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96a32c), (0x91, 0x96d38c), (0xc1, 0x96a32c)];
        let sorte = [(0xf5, 0x96b014), (0x91, 0x96b014), (0x9d, 0x96a32c), (0xf1, 0x96a32c), (0x95, 0x96a32c), (0xc1, 0x96b014)];
        if c45() & 0x20000 != 0 {
            if fraco() {
                for (o, k) in strong {
                    mul(o, k);
                }
            } else if (read_i8(p + staff::PRESSURE) as i32) < random(20) {
                for (o, k) in sorte {
                    mul(o, k);
                }
            }
        }
        if read_i8(c + contract::GRIEVANCES) as u8 & 0x40 != 0 {
            if fraco() {
                for (o, k) in strong {
                    mul(o, k);
                }
            } else if (read_i8(p + staff::PRESSURE) as i32) < random(25) {
                for (o, k) in sorte {
                    mul(o, k);
                }
            }
        }
    }
    // the person’s preferences (clubs of the opponent and of his own side, managers, designated)
    let t = *((engine() + 0x4792) as *const usize);
    if read_i8(t + fixture::HOME_SIDE_MARK) == 0 {
        return;
    }
    let minus = |o: usize, k: i32, c: usize| wr(o, (read_f32(this + o) as f64) - k as f64 * f(c));
    let club_other = || {
        let t = *((engine() + 0x4792) as *const usize);
        *((t + if read_i8(this + match_player::SIDE) == 0 { 0x20 } else { 0x1c }) as *const usize)
    };
    let k25 = || 25 - (read_i8((person()) + 0x5b) as i32);
    let k50 = || 50 - (read_i8((person()) + 0x5b) as i32) - (read_i8((person()) + 0x59) as i32);
    if exe::staff_dislikes_club(person(), club_other()) != 0 {
        wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) + f(0x968990));
        wr(0x75, k25() as f64 * f(0x96d668) + (read_f32(this + match_player::AGGRESSION) as f64));
    } else if exe::staff_favours_club(person(), club_other()) != 0 {
        minus(0xf5, k50(), 0x96afc4);
        minus(0x75, k50(), 0x96afc4);
    }
    let other = (read_i8(this + match_player::SIDE) == 0) as i32;
    if exe::staff_is_disliked_by(jx(other), person()) != 0 {
        wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) + f(0x968990));
        wr(0x75, k25() as f64 * f(0x96d668) + (read_f32(this + match_player::AGGRESSION) as f64));
    } else if exe::staff_is_favoured_by(jx(other), person()) != 0 {
        minus(0xf5, k50(), 0x96afc4);
        minus(0x75, k50(), 0x96afc4);
    }
    if exe::staff_favours_club(person(), club_side()) != 0 {
        wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) + f(0x968990));
    } else if exe::staff_dislikes_club(person(), club_side()) != 0 {
        minus(0xf5, k25(), 0x96d668);
    }
    let own = read_i8(this + match_player::SIDE) as i32;
    if exe::staff_is_disliked_by(jx(own), person()) != 0 {
        minus(0xf5, k25(), 0x96d668);
        minus(0xe1, k25(), 0x96d668);
    } else if exe::staff_is_favoured_by(jx(own), person()) != 0 {
        wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) + f(0x968990));
        wr(0xe1, (read_f32(this + match_player::TEAMWORK) as f64) + f(0x968990));
    }
    let pd = |m: usize| *((designado(m) + 0x69) as *const usize);
    if exe::staff_is_disliked_by(pd(engine()), person()) != 0 {
        minus(0xf5, k25(), 0x96d668);
        minus(0xe1, k25(), 0x96d668);
    } else if exe::staff_is_favoured_by(pd(engine()), person()) != 0 {
        wr(0xf5, (read_f32(this + match_player::WORK_RATE) as f64) + f(0x968990));
        wr(0xe1, (read_f32(this + match_player::TEAMWORK) as f64) + f(0x968990));
    }
    // contract history (+0x10 and +0x14 over +0xc matches)
    if l2c != 0 {
        let c = l2c;
        let c0c = *((c + contract::WAGE) as *const i32);
        for (o, tira) in [(0x10usize, true), (0x14, false)] {
            let e = *((c + o) as *const i32);
            if e > -1 && c0c > 0 {
                let r = e as f64 * f(0x96d664) / c0c as f64;
                if r > q(0x96afa8) {
                    let d = k25() as f64 * r * f(0x96d660);
                    wr(0xe1, if tira { (read_f32(this + match_player::TEAMWORK) as f64) - d } else { d + (read_f32(this + match_player::TEAMWORK) as f64) });
                }
            }
        }
    }
}

/// n = contract level (`exe::database_get_contract_rating_modifier`); form = (word +0xd of the record − club word +0x80) × 0.002 + 1.5, clamped to −4..4; c
/// = contract byte +0x44 (`exe::contract_manager_get_contract_for_club`) / 75 + 0.5, adjusted by type (+0x4f >> 4: −10, +2.5, +2, −3, −15, −20, −50); x =
/// byte +0x22 × 75 for a national side, otherwise days alive (`exe::date_days_since`); y = 1 + (x − 1000) × 0.000125 above 1000.
/// Rating = byte +0x2f × y − (n² + 5) × 0.1 + form + c + 0.5, truncated
/// Original: `player_rating` (0x6ebd70, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ebd70 } (match_pl.cpp): nota do jogador na partida, pelo clube do lado dele (motor +0x4792,
/// +0x1c/+0x20). n = nivel do contrato (0x53ce90, byte com sinal); forma = (palavra +0xd da ficha
/// +0x6d - palavra +0x80 do clube) x 0,002 + 1,5, limitada a -4..4; c = byte +0x44 do contrato
/// (0x4cf5f0) x (1/75) + 0,5, ajustado pelo tipo (+0x4f >> 4); x = byte +0x22 da pessoa x 75 na
/// selecao, senao os dias de vida (0x54cdb0); y = 1 + (x - 1000) x 0,000125 acima de 1000.
/// Resultado: byte +0x2f da ficha x y - (n^2 + 5) x 0,1 + forma + c + 0,5, truncado.
pub(crate) unsafe extern "thiscall" fn player_rating(this: usize) -> i32 {
    let club = || {
        let t = *((*((this + match_player::ENGINE) as *const usize) + 0x4792) as *const usize);
        *((t + if read_i8(this + match_player::SIDE) == 0 { 0x1c } else { 0x20 }) as *const usize)
    };
    let person = Staff(*((this + match_player::STAFF) as *const usize));
    let sheet = *((this + match_player::PLAYER) as *const usize);
    let n = exe::database_get_contract_rating_modifier(person, club()) as f64;
    let mut form = (*((sheet + player_sheet::WORLD_REPUTATION) as *const i16) as i32 - *((club() + 0x80) as *const i16) as i32) as f64 * 0.002 + 1.5;
    if form < -4.0 {
        form = -4.0;
    } else if form > 4.0 {
        form = 4.0;
    }
    let r = exe::contract_manager_get_contract_for_club(*(G_CONTRACT_MANAGER as *const usize), person, club());
    let mut c = 0.0;
    if r != 0 {
        c = *((r + 0x44) as *const i8) as f64 * 0.013333333333333334 + 0.5;
        c = match read_i8(r + contract::SQUAD_STATUS) as u8 >> 4 {
            0 | 5 => c - 10.0,
            1 => c + 2.5,
            2 => c + 2.0,
            4 => c - 3.0,
            6 => c - 15.0,
            7 => c - 20.0,
            8 => c - 50.0,
            _ => c,
        };
    }
    let x = if exe::database_is_national_team(club()) != 0 {
        (person.int_apps() as i8) as u8 as f64 * 75.0
    } else {
        let read = |o: usize| core::ptr::read_unaligned((person.addr() + o) as *const u32);
        exe::date_days_since(G_CURRENT_DATE, read(staff::DATE_JOINED_CLUB), read(0x42)) as f64
    };
    let y = if x > 1000.0 { (x - 1000.0) * 0.000125 + 1.0 } else { 1.0 };
    let e = *((sheet + player_sheet::LEADERSHIP) as *const i8) as f64;
    ftol(e * y - (n * n + 5.0) * 0.1 + form + c + 0.5)
}

/// Match player: a test by physical condition (+0x1a2): code 1 passes if random(110 - condition) < limit; codes 2..5 if
/// random(K / condition^2) < limit with K = 400,000, 2, 4 and 8 million; the limit is 8 when `alvo` is the player
/// stored at +0x1a6, else 1. Other codes: 0. Returns 1/0.
///
/// [pt] { 0x7164b0 } (match_pl.cpp): teste pela condicao fisica (0x1a2). v = condicao^2 (truncado);
/// limite 8 se `alvo` for o guardado em 0x1a6, senao 1. Codigo 1: sorteio(110 - condicao) < limite;
/// 2 a 5: sorteio(K / v) < limite, K = 400 mil, 2, 4 e 8 milhoes (mais condicao, mais chance).
/// Outros codigos: 0.
pub(crate) unsafe extern "thiscall" fn condition_test(this: usize, code: i8, target: usize) -> u8 {
    let limit: i32 = if target != 0 && target == *((this + match_player::LAST_FOULED_BY) as *const usize) { 8 } else { 1 };
    let c = read_f32(this + match_player::CONFIDENCE) as f64;
    let v = ftol(game_pow(c, read_f64(rdata::F64_2))) as f64;
    let n = match code {
        1 => 0x6e - ftol(c) as i8 as i32,
        2 => ftol(read_f64(rdata::F64_400000) / v),
        3 => ftol(read_f64(rdata::F64_2000000) / v),
        4 => ftol(read_f64(rdata::F64_4000000) / v),
        5 => ftol(read_f64(rdata::F64_8000000) / v),
        _ => return 0,
    };
    (random(n) < limit) as u8
}

/// Calls the game's CRT pow (0x945ff0, _CIpow) via x87 so the result is bit-identical to the original.
///
/// [pt] pow da CRT do jogo (0x945ff0, `_CIpow`: base em st1, expoente em st0), para o resultado ser
/// identico bit a bit ao do original.
pub(crate) unsafe fn game_pow(base: f64, expoente: f64) -> f64 {
    let mut v = [base, expoente, 0.0];
    core::arch::asm!(
        "fld qword ptr [edi]",
        "fld qword ptr [edi + 8]",
        "call {f}",
        "fstp qword ptr [edi + 16]",
        // edi: a chamada preserva (eax/ecx/edx nao)
        in("edi") v.as_mut_ptr(),
        f = in(reg) 0x945ff0usize,
        clobber_abi("C"),
    );
    v[2]
}

/// Value of the 12 x 9 distance table (0xa15e44) for |dy| and |dx| (each truncated to a byte).
///
/// [pt] Valor da tabela 12 x 9 (0xa15e44) para as distancias |dy| e |dx| (cada uma truncada a byte).
pub(crate) unsafe fn distance_table_cell_yx(dy: i32, dx: i32) -> f32 {
    read_f32((0xa15e44 + 4 * ((dy.abs() as i8 as isize) * 9 + dx.abs() as i8 as isize)) as usize)
}

/// Can the player shoot from his cell (goal centre in view via `exe::match_player_is_cell_in_view`, or in the central six-yard area)?
///
/// [pt] O jogador pode bater dali (0x6f9f90 para o centro do gol, salvo na pequena area central)?
pub(crate) unsafe fn can_shoot_from_here(j: usize) -> bool {
    let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
    let bottom: i8 = if side_of_player(j) == 0 { 0xb } else { 0 };
    if y == bottom && (3..=5).contains(&x) {
        return true;
    }
    exe::match_player_is_cell_in_view(j, 4, bottom, 0, 0.0, 0.0, core::ptr::null_mut()) != 0
}

/// Role word (table_a) of the player via `table_a` on his own engine; None without engine.
///
/// [pt] `table_a` do jogador por 0x6c38c0 (o motor do proprio jogador; sem motor, None).
pub(crate) unsafe fn player_role_word(j: usize) -> Option<u16> {
    call_engine_routine::<u16>(j, 0x6c38c0)
}

/// MATCH_PLAYER::get_shot_score: chooses the shot type (when `*tipo` < 0) from the situation and attributes, computes
/// the shot value from per-type words (+0x11a + 2 x type), condition, the opposing goalkeeper, angle and distance to
/// goal, stores the quality part at +0x39 and the commentary code in `som`. Returns the value.
///
/// [pt] { 0x700bd0 } (match_pl.cpp, `MATCH_PLAYER::get_shot_score`): escolhe o tipo do chute (quando
/// `tipo` < 0) pela situacao e pelos atributos (palavras 0x146..0x154), calcula o valor (palavras
/// por tipo em 0x11a + 2 x tipo, condicao fisica, goleiro adversario, angulo, distancia ao gol com
/// pow 8 e 7) e grava em 0x39 a parte "de qualidade" e em `som` o comentario. Devolve o valor.
pub(crate) unsafe extern "thiscall" fn shot_value(this: usize, kind: *mut i8, sound: *mut u16) -> u16 {
    let j = this;
    let w = |o: usize| *((j + o) as *const i16);
    let q39 = (j + match_player::SHOT_QUALITY) as *mut u16;
    let minus_mil = |kind: *mut i8, sound: *mut u16| {
        *kind = 0x16;
        *sound = 0x1f81;
        0xfc18u16
    };
    let gk = exe::cm3_match_engine_get_goalkeeper(*((j + match_player::ENGINE) as *const usize), (side_of_player(j) == 0) as i8);
    let mut b13 = 0xfbu8.wrapping_sub(read_i8(j + match_player::MOMENT_VISION) as u8).wrapping_mul(2) as i8;
    let mut angulo_fechado = false;
    let gy = read_i8(gk + match_player::CELL_Y);
    if gy != 0xb && gy != 0 && player_role_word(gk) == Some(1) {
        b13 = (read_i8(j + match_player::MOMENT_VISION) as u8).wrapping_add(5).wrapping_mul(2) as i8;
    }
    let dgk = distance_table_cell_yx(read_i8(j + match_player::CELL_Y) as i32 - gy as i32, read_i8(j + match_player::CELL_X) as i32 - read_i8(gk + match_player::CELL_X) as i32);
    if !(dgk >= read_f32(rdata::F32_1)) && player_role_word(gk) == Some(1) {
        b13 = (b13 as u8).wrapping_sub(read_i8(j + match_player::MOMENT_VISION) as u8) as i8;
    }
    let (x, y) = (read_i8(j + match_player::CELL_X) as i32, read_i8(j + match_player::CELL_Y) as i32);
    let dist = if side_of_player(j) == 0 { distance_table_cell_yx(y - 0xb, x - 4) } else { distance_table_cell_yx(y, x - 4) } as f64;
    if *kind == 0x33 {
        *sound = 0x1f7f;
        return (random(w(0x180) as i32 * 5) + 0x96) as u16;
    }
    // choice of type
    'escolha: {
        if *kind == 0x35 {
            break 'escolha;
        }
        if w(match_player::ACTION) == 0x3b {
            *kind = 0x39;
            break 'escolha;
        }
        let engine = *((j + match_player::ENGINE) as *const usize);
        if can_shoot_from_here(j) && read_i8(engine + engine::BALL_HEIGHT) < 3 {
            let r = random(3);
            let l = side_of_player(j);
            let reach = read_i8(j + match_player::MOMENT_VISION) as i32 > r
                || (l == 0 && read_i8(j + match_player::CELL_Y) > read_i8(gk + match_player::CELL_Y))
                || (l == 1 && read_i8(j + match_player::CELL_Y) < read_i8(gk + match_player::CELL_Y));
            if reach {
                let in_the_box = {
                    let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
                    let area = if l == 0 { 10..=11 } else { 0..=1 };
                    (2..=6).contains(&x) && area.contains(&y)
                };
                let cobertura = if read_i8(gk + match_player::BUSY) > 1 && in_the_box {
                    true
                } else if player_role_word(gk) != Some(1) {
                    true
                } else {
                    let (gy, gx) = (read_i8(gk + match_player::CELL_Y) as i32, read_i8(gk + match_player::CELL_X) as i32);
                    let t1 = if side_of_player(gk) == 1 { distance_table_cell_yx(gy - 0xb, gx - 4) } else { distance_table_cell_yx(gy, gx - 4) };
                    // goalkeeper far from his own goal (>= 2) and from the attacker (>= 1): direct cover;
                    // otherwise, random(0x29) against random(2000) and distance to goal < 1.5
                    let longe_da_meta = t1 as f64 >= read_f64(rdata::F64_2);
                    let direta = longe_da_meta && {
                        let t2 = distance_table_cell_yx(gy - read_i8(j + match_player::CELL_Y) as i32, gx - read_i8(j + match_player::CELL_X) as i32);
                        t2 as f64 >= read_f64(rdata::F64_1)
                    };
                    direta || {
                        let r1 = random(w(0x29) as i32);
                        r1 > random(0x7d0) && !(dist >= read_f64(rdata::F64_1_5))
                    }
                };
                if cobertura {
                    *kind = 0x3a;
                    break 'escolha;
                }
            }
        }
        if *kind > -1 {
            break 'escolha;
        }
        let greater = |v: &mut i16, r: i32, t: i8, kind: *mut i8| {
            if r as i16 > *v {
                *v = r as i16;
                *kind = t;
            }
        };
        match read_i8(engine + engine::BALL_HEIGHT) {
            0 => {
                if !can_shoot_from_here(j) {
                    return minus_mil(kind, sound);
                }
                let mut v = random(w(0x146) as i32) as i16;
                *kind = 0x16;
                greater(&mut v, random(w(0x14e) as i32 + 20 * b13 as i32), 0x1a, kind);
                if dist >= read_f64(rdata::F64_1_5) {
                    greater(&mut v, random(w(0x152) as i32), 0x1c, kind);
                }
                let a = random(0x1f4);
                let b = random(0x96);
                greater(&mut v, random(w(0x154) as i32 - a + b), 0x1d, kind);
                if random(w(0x148) as i32) as i16 > v {
                    *kind = 0x17;
                }
            }
            1 => {
                if !can_shoot_from_here(j) {
                    return minus_mil(kind, sound);
                }
                let mut v = random(w(0x150) as i32 + 22 * b13 as i32) as i16;
                *kind = 0x1b;
                let a = random(0x1f4);
                let b = random(0x96);
                greater(&mut v, random(w(0x154) as i32 - a + b), 0x1d, kind);
                greater(&mut v, random(w(0x146) as i32), 0x16, kind);
                if random(w(0x14c) as i32) as i16 > v {
                    *kind = 0x19;
                }
            }
            2 | 3 => {
                if !can_shoot_from_here(j) {
                    if read_i8(*((j + match_player::ENGINE) as *const usize) + 0x8ea9) != 3 {
                        return minus_mil(kind, sound);
                    }
                    *kind = 5;
                    break 'escolha;
                }
                let v = random(w(0x150) as i32 + 22 * b13 as i32) as i16;
                *kind = 0x1b;
                if random(w(0x14a) as i32) as i16 > v {
                    *kind = 0x18;
                }
            }
            4 => {
                *kind = if can_shoot_from_here(j) { 3 } else { 5 };
            }
            _ => return minus_mil(kind, sound),
        }
    }
    // valor
    let t = *kind;
    let mut e: i32;
    let match_condition = || {
        let q = ftol(read_f32(j + match_player::STAMINA) as f64) as i16 as i32;
        let c = 10000 - q;
        random(c.wrapping_mul(c) / 50000)
    };
    if t == -1 {
        // tipo desconhecido: o original mostra um erro antes
        e = random(0x32);
        *q39 = 0;
    } else if t == 0x3a {
        let r0 = match_condition() as i16 as i32;
        let rr = r0 / 5;
        let r1 = random(rr);
        let base = w(0x29) as i32 / 2 - r1;
        let r2 = random(w(match_player::BALL_CONTROL) as i32 / 5);
        *q39 = (base + r2 + 0xd2) as u16;
        let mut v = *((j + match_player::MOVE_SKILL + 2 * t as usize) as *const u16);
        v = v.wrapping_add((w(match_player::BALL_CONTROL) as i32 / 5) as u16).wrapping_sub(rr as u16).wrapping_add(w(0x29) as u16);
        let a = random(10);
        let b = random(10);
        e = v as i32 + (a - b);
    } else {
        let r0 = match_condition();
        if *kind == 0x35 {
            *q39 = 0;
        } else {
            *q39 = ((w(0x29) as i32 / 2) as u16).wrapping_add(w(match_player::BALL_CONTROL) as u16).wrapping_sub(r0 as u16);
        }
        let t = *kind;
        let mut bp = *((j + match_player::MOVE_SKILL + 2 * t as isize as usize) as *const u16);
        bp = bp.wrapping_sub(random(r0 as i16 as i32) as u16);
        bp = bp.wrapping_add(random(w(match_player::BALL_CONTROL) as i32) as u16);
        let engine = *((j + match_player::ENGINE) as *const usize);
        let c = read_i8(engine + engine::BALL_HEIGHT);
        let atributos = |bp: &mut u16| {
            for o in [0x79, 0x81, 0xe5] {
                let a = read_f32(j + o);
                if !(a >= read_f32(rdata::F32_15)) {
                    let v = ftol(a as f64 * read_f32(rdata::F32_667) as f64 - read_f32(rdata::F32_10000) as f64);
                    *bp = bp.wrapping_add(random(v) as u16);
                }
            }
        };
        if t == 5 && c == 4 {
            *q39 = (*q39).wrapping_add(0xfed4);
            atributos(&mut bp);
            bp = bp.wrapping_add((read_i8(j + match_player::MOMENT_FLAIR) as i16 as u16).wrapping_mul(30)).wrapping_sub(400);
        } else if t == 5 && c == 3 {
            *q39 = (*q39).wrapping_add(0xff06);
            atributos(&mut bp);
            bp = bp.wrapping_add((read_i8(j + match_player::MOMENT_FLAIR) as i16 as u16).wrapping_sub(15).wrapping_mul(20));
        } else if t == 0x39 {
            let v = (read_f64(rdata::F64_200) - read_f32(j + match_player::AGILITY) as f64 * read_f64(rdata::F64_7_5)) * (3 - c as i32) as f64;
            let r = random(ftol(v));
            bp = bp.wrapping_sub(r as u16);
            *q39 = (*q39).wrapping_sub(r as u16);
        } else if c == 3 {
            let r = random(0x190);
            bp = bp.wrapping_sub(r as u16);
            *q39 = (*q39).wrapping_sub(r as u16);
        }
        let r1 = random(w(0x29) as i32);
        e = r1 + random(bp as i16 as i32);
    }
    // angle (outside the central columns)
    let x = read_i8(j + match_player::CELL_X);
    if !(3..=5).contains(&x) {
        let l = side_of_player(j);
        let mut a = if l == 0 {
            (12 - read_i8(j + match_player::CELL_Y) as i32) as f64 / ((4 - x as i32) as f64).abs()
        } else if l == 1 {
            (read_i8(j + match_player::CELL_Y) as i32 + 1) as f64 / ((4 - x as i32) as f64).abs()
        } else {
            read_f32(rdata::F32_0) as f64
        };
        a *= a;
        if a < read_f64(rdata::F64_0_3) {
            angulo_fechado = true;
        }
        a = read_f32(rdata::F32_740) as f64 - a * read_f32(rdata::F32_2160) as f64;
        let minimum = read_f32(rdata::F32_25) as f64;
        if minimum > a {
            a = minimum;
        }
        let mut k = ftol(a) as u16;
        if *kind == 0x3a {
            k = (k as i16 as i32 / 3) as u16;
        }
        let e5 = read_f32(j + match_player::TECHNIQUE) as f64;
        let f = if !(e5 >= 1.0) { 1.0 } else { e5 };
        let m = k as i16 as i32;
        e -= random(ftol(m as f64 * read_f64(rdata::F64_5) / f));
        *q39 = (*q39).wrapping_add((-(m / 2)) as u16);
    }
    // long shot (outside the area, distance > 2.2): variation by the engine’s wind/pitch
    let mut far = false;
    if dist > read_f64(rdata::F64_2_2) {
        let (x, y) = (read_i8(j + match_player::CELL_X), read_i8(j + match_player::CELL_Y));
        let area = if side_of_player(j) == 0 { 10..=11 } else { 0..=1 };
        if !((2..=6).contains(&x) && area.contains(&y)) {
            far = true;
            let engine = *((j + match_player::ENGINE) as *const usize);
            if engine != 0 {
                match *kind {
                    0x17 | 0x1a | 0x1b | 0x1d => match read_i8(engine + engine::WEATHER) {
                        2 => {
                            let a = random(5);
                            e += random(2) - a;
                        }
                        3 => {
                            let a = random(10);
                            e += random(5) - a;
                        }
                        4 => {
                            let a = random(15);
                            e += random(10) - a;
                        }
                        _ => {}
                    },
                    0x16 => match read_i8(engine + 1) {
                        2 => e += random(4),
                        0x20 => {
                            let a = random(5);
                            e += a - random(5);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    if player_role_word(gk) != Some(1) {
        e += random(0x1f4);
        *q39 = (*q39).wrapping_add(0xfa);
    }
    // distancia ao gol
    let t = *kind;
    let x = read_i8(j + match_player::CELL_X);
    let canto = (read_i8(j + match_player::CELL_Y) == 0 || read_i8(j + match_player::CELL_Y) == 0xb) && (x as i32 - 4).abs() == 1;
    if (t == 3 || t == 0x39) && dist >= read_f64(rdata::F64_1) && !canto {
        let mut v = ftol(game_pow(dist + read_f64(rdata::F64_2), read_f64(rdata::F64_8)) * read_f64(rdata::F64_0_3333333333));
        if v > 0x3a98 {
            v = 0x3a98;
        }
        if (v as i16) < 0 {
            v = 0x3a98;
        }
        if *kind == 0x3a {
            v = (v as i16 as i32) / 8;
        }
        let m = v as i16 as i32;
        e -= random(m);
        *q39 = (*q39).wrapping_add((-(m / 2)) as u16);
    } else if dist == 0.0 {
        e += 0xc8;
        *q39 = (*q39).wrapping_add(0xc8);
    } else if dist == 1.0 {
        e += 0x64;
        *q39 = (*q39).wrapping_add(0x64);
    } else if dist > read_f64(rdata::F64_1_5) {
        let mut c = ftol(game_pow(dist * read_f64(rdata::F64_0_6) + read_f64(rdata::F64_1_5), read_f64(rdata::F64_7)));
        if c > 0x7530 {
            c = 0x7530;
        }
        let (k, mut f) = match *kind {
            0x3a => (c / 20, (read_i8(j + match_player::MOMENT_FLAIR) as f64 + read_f32(j + match_player::LONG_SHOTS) as f64) * read_f64(rdata::F64_0_025)),
            0x35 => (c / 15, read_f32(j + match_player::FREE_KICKS) as f64 * read_f64(rdata::F64_0_05)),
            _ => (c / 2, (read_i8(j + match_player::MOMENT_FLAIR) as f64 + read_f32(j + match_player::LONG_SHOTS) as f64) * read_f64(rdata::F64_0_025)),
        };
        f = read_f64(rdata::F64_1) - f;
        if f < read_f64(rdata::F64_0_2) {
            f = read_f64(rdata::F64_0_2);
        }
        *q39 = (*q39).wrapping_add((-(k / 2)) as u16);
        let mut r = ftol(k as f64 * f);
        if r > 0x7530 {
            r = 0x7530;
        }
        e -= random(r);
    }
    // comentario
    let fl = far as u16;
    *sound = match *kind {
        3 => if angulo_fechado { 0x1fb8 } else { 0x1f92 + 0x18 * fl },
        5 => if angulo_fechado { 0x1fb7 } else { 0x1f91 + 0x18 * fl },
        0x16 => if angulo_fechado { 0x1fb0 } else { 0x1f86 + 0x1c * fl },
        0x17 => if angulo_fechado { 0x1fbb } else { 0x1f99 + 0x14 * fl },
        0x18 => if angulo_fechado { 0x1fb9 } else { 0x1f93 + 0x18 * fl },
        0x19 => if angulo_fechado { 0x1fba } else { 0x1f96 + 0x16 * fl },
        0x1a => if angulo_fechado { 0x1fb1 } else { 0x1f89 + 0x1a * fl },
        0x1b => if angulo_fechado { 0x1fb3 } else { 0x1f8b + 0x1a * fl },
        0x1c => if angulo_fechado { 0x1fb5 } else { 0x1f8d + 0x1a * fl },
        0x1d => if angulo_fechado { 0x1fb6 } else { 0x1f8e + 0x1a * fl },
        0x35 => {
            if angulo_fechado {
                0x1fbd
            } else if far {
                0x1faf
            } else if random(3) != 0 {
                0x1fbe
            } else {
                0x21bd
            }
        }
        0x39 => if angulo_fechado { 0x1fbc } else { 0x1f9c + 0x12 * fl },
        0x3a => 0x1f9d,
        _ => 0x1f81,
    };
    e as u16
}

/// Role bits word of the player from the engine via FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION (0 without engine).
///
/// [pt] `table_a` do motor do jogador (0 sem motor), pelo endereco do jogo: os bits do papel tatico.
pub(crate) unsafe fn engine_player_role(engine: usize, j: usize) -> u16 {
    if engine == 0 {
        return 0;
    }
    exe::cm3_match_engine_get_player_position(engine, j)
}

/// Calls a thiscall(player) -> u8 routine at the given game address.
pub(crate) unsafe fn call_player_byte_routine(endereco: usize, j: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(endereco);
    f(j)
}

/// the opponent the player man-marks, or 0. Needs mark 4 on the position's square; looks among the active opponents for
/// the record designated for the position. With mark 0x10: if the opposing formation has a centre forward (bit
/// 0x10000000 of +0x187b) or the other side's byte +0xaf is above 1, a player with 0x106 does not mark; and, if the
/// marked player has tables A and B different (arrow to another square) and B without the attack mark 0x40, a player
/// with 0x106 does not mark either
/// Original: `designated_marked` (0x7160f0, match_pl.cpp, thiscall).
///
/// [pt] { 0x7160f0 } (match_pl.cpp): o adversario que o jogador marca individualmente, ou 0. Precisa
/// da marca 4 na casa da posicao (0x5b57c0, formacao do lado em motor + 0x91e2); procura entre os
/// adversarios ativos a ficha designada para a posicao (0x5b58e0). Com a marca 0x10 ainda: se a
/// formacao adversaria tem o bit 0x10000000 em +0x187b ou o byte +0xaf do outro lado passa de 1, o
/// jogador com 0x106 nao marca; e, se o marcado tem as tabelas A e B diferentes e sem a marca 0x40
/// na B, o jogador com 0x106 tambem nao marca.
pub(crate) unsafe extern "thiscall" fn designated_marked(this: usize) -> usize {
    let engine = *((this + match_player::ENGINE) as *const usize);
    let side = read_i8(this + match_player::SIDE);
    let formation = (engine as isize + 0x91e2 + 0x18e3 * side as isize) as usize;
    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 4 == 0 {
        return 0;
    }
    let sheet = exe::formation_get_marking_target(formation, read_i8(this + match_player::POSITION));
    let other = (side == 0) as i8;
    let target = match (0..20).map(|k| side_player_slot(engine, other, k)).find(|&q| read_i8(q + match_player::POSITION) >= 0 && *((q + match_player::STAFF) as *const usize) == sheet) {
        Some(q) => q,
        None => return 0,
    };
    if exe::formation_get_marking_type(formation, read_i8(this + match_player::POSITION)) & 0x10 == 0 {
        return target;
    }
    let o = other as usize;
    let fechada = *((engine + 0xaa5d + 0x18e3 * o) as *const u32) & 0x1000_0000 != 0 || read_i8(engine + o + 0xaf) > 1;
    if fechada && read_i8(this + match_player::MOMENT_DECISIONS) != 0 {
        return 0;
    }
    let m_target = *((target + match_player::ENGINE) as *const usize);
    let b = if m_target != 0 { table_b(m_target, target) } else { 0 };
    let a = if m_target != 0 { table_a(m_target, target) } else { 0 };
    if b == a || b & 0x40 != 0 || read_i8(this + match_player::MOMENT_DECISIONS) == 0 {
        target
    } else {
        0
    }
}

/// the player's side (+0x27)
/// Original: `player_side` (0x714d30, match_pl.cpp, thiscall).
///
/// [pt] { 0x714d30 } (thiscall, match_pl.cpp): o lado do jogador `j` (+0x27).
pub(crate) unsafe extern "thiscall" fn player_side(j: usize) -> u8 {
    *((j + match_player::SIDE) as *const u8)
}

/// `p`'s shot (type, value, commentary, mode 1 match / 2 no challenger / 3 shoot-out / 4 set-piece kick / 5-6 repeat,
/// the pressure teammates list). Challenger (`exe::cm3_match_engine_get_opposition_challenge`) with foul or dispossession; commentary; offside; interception
/// (0x6cc540); the outcome class; goal (can be disallowed), save, post or wide, with the commentary. Compared through
/// the tape, with comparison inside the unit
/// Original: `shoot` (0x702050, match_pl.cpp (35 KB), thiscall, ret 0x18).
///
/// [pt] { 0x702050 } (ret 0x18, match_pl.cpp): ver `shoot_with`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn shoot(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    shoot_with(p, a1, a2, a3, a4, a5, a6 as usize, &mut |r, t, a| dispatch_shot_call(r, t, a))
}

/// the match player destructor (zeroes +0x19e)
/// Original: `destroy_match_player` (0x6ebd60, match_pl.cpp, thiscall).
///
/// [pt] { 0x6ebd60 } (thiscall, match_pl.cpp): o destrutor do jogador da partida: zera o ponteiro do motor
/// (+0x19e).
pub(crate) unsafe extern "thiscall" fn destroy_match_player(j: usize) {
    core::ptr::write_unaligned((j + match_player::ENGINE) as *mut u32, 0);
}

/// the match player constructor (clears the fields, with initial values for some)
/// Original: `construct_match_player` (0x6eada0, match_pl.cpp, thiscall).
///
/// [pt] { 0x6eada0 } (thiscall, match_pl.cpp): o construtor do jogador da partida (0x1bc bytes): zera os
/// campos, com −1 em +0, +0x28, +0x2b e +0x5d/+0x61/+0x65, 0xfb em +0x19, 6 em +0x1b e 0x17d4 em +0x35.
/// Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construct_match_player(j: usize) -> usize {
    let b = |o: usize, v: u8| *((j + o) as *mut u8) = v;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((j + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((j + o) as *mut u32, v);
    for o in [0x19e, 0x69, 0x6d, 0x55, 0x1a6, 0x1a2, 0x1aa, 0x1ae, 0x1b2, 0x1b6, 0x4d, 0xf9] {
        d(o, 0);
    }
    for o in [0x5d, 0x61, 0x65] {
        d(o, u32::MAX);
    }
    for o in [0x37, 0x29, 0x19c, 0x1ba] {
        w(o, 0);
    }
    for o in [0x27, 0x102, 0x103, 0x25, 0x26, 0x1c, 0xff, 0x1a, 0x101, 0xfe, 1, 0x24, 0x23, 0xfd] {
        b(o, 0);
    }
    for o in 2..=0x16 {
        b(o, 0);
    }
    b(0x19, 0xfb);
    b(0, 0xff);
    b(0x2b, 0xff);
    b(0x28, 0xff);
    b(0x1b, 6);
    w(0x35, 0x17d4);
    j
}

/// Returns 1 if the player is in his own half (side 1 with row +0x103 >= 6, side 0 with row < 6), else 0 [inference:
/// meaning taken from the attack directions].
/// Original: `player_condition` (0x70dfa0, match_pl.cpp).
///
/// [pt] { 0x70dfa0 } (match_pl.cpp)
pub(crate) unsafe extern "thiscall" fn player_condition(this: usize) -> u8 {
    let (lado, v) = (read_i8(this + match_player::SIDE), read_i8(this + match_player::CELL_Y));
    (if lado == 1 && v >= 6 { true } else { lado == 0 && v < 6 }) as u8
}

/// `table_a` of the player's engine (0x19e), called through the game's address
/// Original: `player_table` (0x717410, thiscall).
///
/// [pt] { 0x717410 } (match_pl.cpp): `table_a` do motor do jogador (campo 0x19e), ou 0 sem motor.
/// Chama 0x6c38c0 pelo endereco do jogo (Rust ou original, conforme o modo).
pub(crate) unsafe extern "thiscall" fn player_table(player: usize) -> u16 {
    let engine = *((player + match_player::ENGINE) as *const usize);
    if engine == 0 {
        return 0;
    }
    exe::cm3_match_engine_get_player_position(engine, player)
}

/// Is the match player in his attacking end (side 1 with y < 3, side 0 with y > 8)? Returns 1 or 0.
/// Original: `player_limit` (0x70df40, match_pl.cpp).
///
/// [pt] { 0x70df40 } (match_pl.cpp): como `limite`, com o lado e o valor do proprio jogador.
pub(crate) unsafe extern "thiscall" fn player_limit(this: usize) -> u8 {
    let (s, v) = (read_i8(this + match_player::SIDE), read_i8(this + match_player::CELL_Y));
    (if s == 1 && v < 3 { true } else { s == 0 && v > 8 }) as u8
}

/// Is the match player inside the goal-area band (x 2..6) at the end of the pitch belonging to side `lado` (y 10..11
/// for side 1, 0..1 otherwise)? Returns 1 or 0.
/// Original: `player_band` (0x6f5b20, match_pl.cpp).
///
/// [pt] { 0x6f5b20 } (match_pl.cpp): como `faixa`, com os campos 0x102/0x103 do jogador. Devolve eax inteiro.
pub(crate) unsafe extern "thiscall" fn player_band(this: usize, side: u8) -> u32 {
    let (b, c) = (read_i8(this + match_player::CELL_X), read_i8(this + match_player::CELL_Y));
    let target = if side == 1 { 10..=11 } else { 0..=1 };
    ((2..=6).contains(&b) && target.contains(&c)) as u32
}

/// 0x91e2 + 6371 × side
/// Original: `field_by_position` (0x6f4600, the side's block in the engine, thiscall, `al`).
///
/// [pt] { 0x6f4600 } (match_pl.cpp): pela marca da posicao do jogador (bit 2 do byte de 0x5b57c0 no
/// bloco do lado dele dentro do motor: 0x91e2 + 6371 x lado), devolve o campo 0x10a ou o 0x108.
pub(crate) unsafe extern "thiscall" fn field_by_position(this: usize) -> u8 {
    let engine = *((this + match_player::ENGINE) as *const usize);
    let block = (engine as isize + 0x91e2 + 6371 * side_of_player(this) as isize) as usize;
    if exe::formation_get_marking_type(block, read_i8(this + match_player::POSITION)) & 2 != 0 { read_i8(this + match_player::MOMENT_POSITIONING) as u8 } else { read_i8(this + match_player::MOMENT_MARKING) as u8 }
}

/// Sets the match player's facing direction in degrees (+0x19a, 0..=359) from cell (a, b) towards (c, d) (same cell:
/// random, except two fixed goal cases 0 and 180), plus a random deviation of up to +-30 when +0x10c x +0x105 <
/// random(120), plus twice the difference of two player-data attributes (+0x3b - +0x30).
/// Original: `aim (set facing direction)` (0x6f4920, match_pl.cpp).
///
/// [pt] { 0x6f4920 } (match_pl.cpp): direcao do jogador em graus, campo 0x19a (0..=359), de (a, b)
/// para (c, d). Mesmo ponto: sorteia 0..359 (salvo dois casos fixos: 0 e 180 graus). Depois
/// um desvio sorteado (+-30) quando 0x10c x 0x105 < sorteio(120), e a diferenca de dois
/// atributos do registro da pessoa (0x3b - 0x30, vezes 2).
pub(crate) unsafe extern "thiscall" fn aim_set_facing_direction(this: usize, a: i8, b: i8, c: i8, d: i8) {
    let dir = (this + match_player::FACING) as *mut u16;
    let (dl, dc) = (c.wrapping_sub(a), d.wrapping_sub(b));
    *dir = if dl == 0 && dc == 0 {
        match (d, c) {
            (11, 4) => 0,
            (0, 4) => 180,
            _ => random(0x168) as u16,
        }
    } else {
        *((GLOBAL_TABLE as isize + 2 * (dl as isize * 24 + dc as isize)) as *const u16)
    };
    let (x, y) = (read_i8(this + match_player::MOMENT_AGILITY) as i32, read_i8(this + match_player::MOMENT_ANTICIPATION) as i32);
    if x * y < random(0x78) {
        let p = random(0x1e);
        let q = random(0x1e);
        *dir = dir.read().wrapping_add(p.wrapping_sub(q) as u16);
    }
    let person = *((this + match_player::PLAYER) as *const usize);
    let adjust = (read_i8(person + player_sheet::RIGHT_FOOT) as i16).wrapping_sub(read_i8(person + player_sheet::LEFT_FOOT) as i16).wrapping_mul(2);
    *dir = dir.read().wrapping_add(adjust as u16);
    let v = *dir as i16;
    if v < 0 {
        *dir = v.wrapping_add(0x168) as u16;
    } else if v > 0x167 {
        *dir = v.wrapping_sub(0x168) as u16;
    }
}

