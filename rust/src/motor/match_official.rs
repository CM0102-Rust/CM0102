use super::*;

/// Initialises the referee object (engine, official, strictness word +8): 5000 without an official; otherwise the first
/// of 7750, 7250, 6750, 6250, 5750 for which the official's word +0x1e beats random(350, 300, 250, 200, 150), or 5250.
///
/// [pt] { 0x6e9470 }: inicia o objeto (motor, pessoa, valor): sem pessoa, 5000; senao o primeiro de
/// 7750, 7250, 6750, 6250, 5750 cuja palavra 0x1e da pessoa passa de sorteio(350, 300, ...), ou 5250.
pub(crate) unsafe extern "thiscall" fn init_strictness(this: usize, engine: u32, person: usize) {
    let person = Staff(person);
    *(this as *mut u32) = engine;
    *((this + 4) as *mut u32) = person.addr() as u32;
    let v = (this + 8) as *mut u16;
    if person.is_null() {
        *v = 0x1388;
        return;
    }
    for (n, valor) in [(0x15e, 0x1e46u16), (0x12c, 0x1c52), (0xfa, 0x1a5e), (0xc8, 0x186a), (0x96, 0x1676)] {
        let r = random(n);
        if (person.second_nation() as i16) as i32 > r {
            *v = valor;
            return;
        }
    }
    *v = 0x1482;
}

/// does the recorded move continue? the ball crossing the move's line ends it (clears)
/// Original: `move_counts` (0x6bab90, thiscall, `al`).
///
/// [pt] { 0x6bab90 } (match_eng.cpp): o lance registrado continua valendo? 0 sem lance; com o dono da
/// bola (0xf582) do outro lado que o jogador do lance, a bola passando a linha do lance (0x475c)
/// encerra o lance (limpa e da 0); nos demais casos, 1.
pub(crate) unsafe extern "thiscall" fn move_counts(this: usize) -> u8 {
    let p = *((this + engine::PENDING_PLAY_OFFENDER) as *const usize);
    if read_i8(this + engine::PENDING_PLAY_KIND) == 0 || p == 0 {
        return 0;
    }
    let l = read_i8(p + 0x27);
    let owner = *((this + engine::BALL_HOLDER) as *const usize);
    if owner == 0 || read_i8(owner + 0x27) as i32 != (l == 0) as i32 {
        return 1;
    }
    let (y, linha) = (read_i8(this + engine::BALL_Y), read_i8(this + engine::PENDING_PLAY_Y));
    if (y > linha && l == 1) || (y < linha && l == 0) {
        clear_move(this);
        return 0;
    }
    1
}

/// Referee decision on whether an incident on player `j` becomes a call: none while a foul is pending or for a
/// goalkeeper in his box; the referee may let it go (random(refereeing +0x28 + 15) == 0, strictness -250), and in the
/// box 3 times in 4; otherwise strictness +175 and, by `codigo` (0x25: 19 in 20; others 1 in 155 when engine `engine::BALL_HEIGHT` >
/// 1), writes 4 to `tipo` and random(player +0x14) + 1 to `valor` and returns 1; else 0.
///
/// [pt] { 0x6e9540 }: decide se o lance vira falta (ou similar) para o jogador: nao com lance valendo, nem
/// com papel 1 na area; a pessoa (0x4) pode anular (sorteio(0x28 + 15) == 0, -250 no valor 0x8);
/// na area, 3 em 4 anula; senao +175 e, pelo codigo (0x25: 1 em 20; outros: 1 em 155 com 0x8ea9 > 1),
/// grava 4 e sorteio(0x14 do jogador) + 1 nas saidas.
pub(crate) unsafe extern "thiscall" fn decide_foul(this: usize, j: usize, code: u16, kind: *mut u8, value: *mut u8) -> u8 {
    let j = MatchPlayer(j);
    let engine = *(this as *const usize);
    if exe::cm3_match_engine_is_foul_pending(engine) != 0 {
        return 0;
    }
    if exe::cm3_match_engine_get_player_position(engine, j) & 1 != 0 && (exe::match_player_is_in_penalty_area(j, side_of_player(j) as u8) as u32) as u8 != 0 {
        return 0;
    }
    let person = *((this + 4) as *const usize);
    let v = (this + 8) as *mut u16;
    if person != 0 && random(read_i8(person + 0x28) as i32 + 0xf) == 0 {
        *v = (*v).wrapping_add(0xff06);
        return 0;
    }
    if (exe::match_player_is_in_penalty_area(j, side_of_player(j) as u8) as u32) as u8 != 0 && random(4) != 0 {
        return 0;
    }
    *v = (*v).wrapping_add(0xaf);
    let mark = if code == 0x25 { random(0x14) != 0 } else { random(0x9b) == 0 && read_i8(engine + engine::BALL_HEIGHT) > 1 };
    if !mark {
        return 0;
    }
    let r = random((j.fouls() as i8) as u8 as i32);
    *value = (r as u8).wrapping_add(1);
    *kind = 4;
    1
}

/// Yellow card: the first stores x at +0x25 (type 2, weight 2.0); the second at +0x26 (type 3, weight 3.5).
///
/// [pt] Cartao amarelo: o primeiro grava `x` em 0x25 (0x24 = 1, tipo 2, peso 2,0); o segundo, em 0x26
/// (0x24 + 1, tipo 3, peso 3,5).
pub(crate) unsafe fn give_yellow_card(j: MatchPlayer, kind: *mut u8, x: i8, weight: *mut f32) {
    let n = (j.addr() + match_player::YELLOW_CARDS) as *mut u8;
    if *n != 0 {
        *n = (*n).wrapping_add(1);
        *kind = 3;
        *weight = 3.5;
        j.set_second_card_code(x);
    } else {
        *n = 1;
        *kind = 2;
        *weight = 2.0;
        j.set_first_card_code(x);
    }
}

/// Reduces the player's disposition for a card, scaled by the engine value 0x8ed0 / 11 when above 45 [inference].
///
/// [pt] Desconto na disposicao pelo cartao: ftol(c x c') ou, com 0x8ed0 / 11 acima de 45, ftol(c'' / (0x8ed0 / 11) x c').
pub(crate) unsafe fn apply_card_penalty(engine: usize, j: MatchPlayer) {
    let q = (*((engine + engine::CLOCK) as *const i16) as i32 / 11) as i16;
    let base = if q > 0x2d { read_f32(rdata::F32_45) as f64 / q as f64 } else { read_f32(rdata::F32_1) as f64 };
    let d = (j.addr() + match_player::MATCH_RATING) as *mut i16;
    *d = (*d).wrapping_add(ftol(base * read_f32(rdata::F32_NEG_225) as f64) as i16);
}

/// Increments the side's foul counter in the engine (0xf5c9 + 54 × side).
///
/// [pt] Contador de faltas do lado no motor (0xf5c9 + 54 x lado).
pub(crate) unsafe fn count_foul(engine: usize, j: MatchPlayer) {
    let c = (engine as isize + 0xf5c9 + 54 * side_of_player(MatchPlayer(j.addr())) as isize) as *mut u8;
    *c = (*c).wrapping_add(1);
}

/// Calls the match player is_in_penalty_area check for the player.
pub(crate) unsafe fn is_in_penalty_area(j: MatchPlayer) -> bool {
    (exe::match_player_is_in_penalty_area(j, side_of_player(MatchPlayer(j.addr())) as u8) as u32) as u8 != 0
}

/// The referee's punishment for a foul by player `j`: counts the foul; for a type 1 foul, either a straight red (type
/// 4, -1000 disposition), a yellow card by severity against random draws, or only -25; with `flag` (or for the
/// goalkeeper) a draw out of 3000 gives a red above 475 (-750, type 6) or a yellow (-375) above 50 or inside the box.
/// Writes the card type to `tipo` and the weight to `peso`.
///
/// [pt] { 0x6e9630 }: punicao da falta (tipo 1): vermelho direto (-1000, tipo 4), amarelo pela
/// severidade contra sorteios, ou so -25; com `flag` (ou papel 1), sorteio(3000): acima de 475
/// vermelho (-750, tipo 6, 9 no registro), acima de 50 (ou na area) amarelo com -375.
pub(crate) unsafe extern "thiscall" fn punish_foul(this: usize, j: usize, kind: *mut u8, x: i8, severidade: i8, flag: u8, weight: *mut f32) {
    let j = MatchPlayer(j);
    *weight = 1.5;
    let engine = *(this as *const usize);
    count_foul(engine, j);
    j.set_fouls((j.fouls()).wrapping_add(1));
    if *kind != 1 || read_i8(engine + engine::SHOOTOUT_STATE) != 0 {
        return;
    }
    let d = (j.addr() + match_player::MATCH_RATING) as *mut i16;
    if flag == 0 && exe::cm3_match_engine_get_player_position(engine, j) & 1 == 0 {
        if severidade as i32 > random(0xf) + 9 {
            *d = (*d).wrapping_add(-1000);
            *kind = 4;
            *weight = 5.0;
        } else if severidade as i32 > random(5) + 3 {
            apply_card_penalty(engine, j);
            give_yellow_card(j, kind, x, weight);
        } else {
            *d = (*d).wrapping_add(-25);
        }
        return;
    }
    let r = random(0xbb8) as i16;
    if r > 0x1db {
        *d = (*d).wrapping_add(-750);
        *kind = 6;
        *weight = 5.0;
        *((j.addr() + if j.yellow_cards() != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
        return;
    }
    if r <= 0x32 && !is_in_penalty_area(j) {
        return;
    }
    *d = (*d).wrapping_add(-375);
    give_yellow_card(j, kind, x, weight);
}

/// Referee: decides the punishment for a move of type `*tipo` by player `j` on `outro` (like `punish_foul`): off-pitch
/// player weight 10; type 7 weight 1.25; with `flag`, a red (type 5) if random(3000) > 375, else a yellow (above 50 or
/// in the box); without `flag`, a red by `severidade` (with `expulsar`, flag +0x16), a yellow against random(6), or
/// disposition -25. Counts the foul on the player (+0x14) and the fouled count on `outro` (+0x15).
///
/// [pt] { 0x6ea4e0 }: como `punish_foul`, para outro tipo de lance: inativo, peso 10; tipo 7, peso
/// 1,25; com `flag`, sorteio(3000) acima de 375 e vermelho (tipo 5), senao amarelo com desconto
/// (acima de 50, ou na area); sem `flag`, vermelho pela severidade (marca 0x16 com `expulsar`),
/// amarelo contra sorteio(6) ou -25. Conta a falta no jogador e 0x15 no outro.
pub(crate) unsafe extern "thiscall" fn punish_move(this: usize, j: usize, other: usize, kind: *mut u8, x: i8, severidade: i8, flag: u8, weight: *mut f32, send_off: u8) {
    let j = MatchPlayer(j);
    *weight = 1.5;
    if j.position() < 0 {
        *weight = 10.0;
        return;
    }
    let engine = *(this as *const usize);
    let d = (j.addr() + match_player::MATCH_RATING) as *mut i16;
    match *kind {
        7 => {
            *weight = 1.25;
            count_foul(engine, j);
        }
        1 if read_i8(engine + engine::SHOOTOUT_STATE) == 0 => {
            count_foul(engine, j);
            if flag != 0 {
                let r = random(0xbb8) as i16;
                if r > 0x177 {
                    *d = (*d).wrapping_add(-750);
                    *kind = 5;
                    *weight = 5.0;
                    *((j.addr() + if j.yellow_cards() != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
                } else if r > 0x32 || is_in_penalty_area(j) {
                    apply_card_penalty(engine, j);
                    give_yellow_card(j, kind, x, weight);
                }
            } else if severidade as i32 > random(0xf) + 7 {
                *d = (*d).wrapping_add(-1000);
                *kind = 4;
                *weight = 5.0;
                *((j.addr() + if j.yellow_cards() != 0 { 0x26 } else { 0x25 }) as *mut i8) = x;
                if send_off != 0 {
                    j.set_sent_off(1);
                }
            } else if severidade as i32 > random(6) {
                apply_card_penalty(engine, j);
                give_yellow_card(j, kind, x, weight);
            } else {
                *d = (*d).wrapping_add(-25);
            }
        }
        _ => return,
    }
    j.set_fouls((j.fouls()).wrapping_add(1));
    *((other + 0x15) as *mut u8) = (*((other + 0x15) as *const u8)).wrapping_add(1);
}

/// Referee: looks for an attacker of side `lado_b` caught offside on a ball to row `y` (passing row `linha`). Returns
/// the player (adjusting the referee's value at +8) or 0; nothing if a foul is pending or `y` is not past the line and
/// the last defensive line; the referee may let it go by a roll on his person; per player, rolls against his +0x107
/// depending on how far past the line he is.
///
/// [pt] { 0x6ea760 } (match_official.cpp): procura um atacante do `lado` em impedimento no lance para a
/// casa de `y` (linha do passe em `linha`). Devolve o jogador (ajustando o valor do arbitro em 0x8)
/// ou 0. Sem lance valendo; `y` alem da linha e nao atras da ultima linha defensiva (0x8eaf para o
/// lado 0, 0x8eb0 para o 1); com `passador`, ele na area do outro lado e jogadores alem da linha.
/// A pessoa do arbitro pode deixar passar (sorteio(0x29 + 25) == 0, -75). Por jogador, conforme a
/// distancia a linha (2, 1, 0 casas ou alem), sorteios contra o byte 0x107.
pub(crate) unsafe extern "thiscall" fn look_for_offside(this: usize, side_b: i8, row: i8, y: i8, passador: usize) -> usize {
    let engine = *(this as *const usize);
    if exe::cm3_match_engine_is_foul_pending(engine) != 0 {
        return 0;
    }
    let (l0, l1) = (read_i8(engine + engine::OFFSIDE_LINE), read_i8(engine + engine::OFFSIDE_LINE_SIDE1));
    match side_b {
        0 if !(y > row && y >= l0) => return 0,
        1 if !(y < row && y <= l1) => return 0,
        _ => {}
    }
    if passador != 0 {
        if exe::cm3_match_engine_count_players_ahead_of_row(engine, row, side_b, 0) == 0 {
            return 0;
        }
    }
    let v = (this + 8) as *mut i16;
    let person = *((this + 4) as *const usize);
    if person != 0 && random(read_i8(person + 0x29) as i32 + 0x19) == 0 {
        *v = (*v).wrapping_add(-75);
        return 0;
    }
    for k in 0..20 {
        let p = side_player_slot(engine, side_b, k);
        if read_i8(p + 0x19) < 0 || p == passador {
            continue;
        }
        if passador != 0 {
            if (exe::match_player_is_in_penalty_area(passador, (side_b == 0) as u8) as u32) as u8 == 0 {
                continue;
            }
            let dist = match_player_distance_to_cell(p, 4, y);
            if ((random(4) + 1) as f64) < dist as f64 || dist.is_nan() {
                continue;
            }
        }
        let (py, a) = (read_i8(p + 0x103) as i32, read_i8(p + 0x107) as i32);
        let (linha_d, sinal, perto) = match side_b {
            0 => (l0 as i32, 1, -25i16),
            1 => (l1 as i32, -1, -15i16),
            _ => continue,
        };
        if py == linha_d - 2 * sinal && random(6 * a + 500) == 0 {
            *v = (*v).wrapping_add(perto);
            return p;
        }
        if py == linha_d - sinal && random(ftol((a as f64) * (a as f64) * (a as f64) + read_f64(rdata::F64_25))) == 0 {
            *v = (*v).wrapping_add(-15);
            return p;
        }
        if py == linha_d && random(3) != 0 && a < random(0x12) {
            *v = (*v).wrapping_add(15);
            return p;
        }
        if (py - linha_d) * sinal > 0 && a < random(0x64) {
            return p;
        }
    }
    0
}

/// Forwards to the match engine's referee (this + 0xf5b2) the search for an offside player of side `lado_b` relative to
/// the line, row `y` and the passer. Returns the offside player or 0.
///
/// [pt] { 0x6cfd70 } (match_eng.cpp): `look_for_offside` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn engine_look_for_offside(this: usize, side_b: i8, row: i8, y: i8, passador: usize) -> usize {
    exe::match_official_find_offside_player(this + engine::REFEREE, side_b, row, y, passador)
}

/// Writes the card code in the player's record: +0x25 when the card total is 1 (or ≤ 1), else +0x26 when non-zero.
///
/// [pt] Grava o codigo do cartao no registro do jogador: 0x25 com (0x23 + 0x24) == 1 (ou <= 1, `ate_um`),
/// 0x26 com outro total diferente de zero.
pub(crate) unsafe fn record_card_code(j: MatchPlayer, code: u8, ate_um: bool) {
    let t = j.red_cards() as i32 + j.yellow_cards() as i32;
    if t == 1 || (ate_um && t <= 1) {
        j.set_first_card_code((code) as i8);
    } else if t != 0 {
        j.set_second_card_code((code) as i8);
    }
}

/// Referee: decides the card for a move of severity `gravidade` (1..6) by player `j`: random(foul count +0x14) adjusted
/// by the referee's attribute (+0x25) and the severity; a possible second yellow; a red (disposition -1000, `*tipo` =
/// 4) against random(15) + 7; else a yellow (`*tipo` 2 or 3) against random(3 x yellows + 5) + 2, or just type 9 (no
/// card). Records the card code on the player.
///
/// [pt] { 0x6eab60 } (match_official.cpp): cartao pela gravidade do lance (1..6): sorteio(0x14 do
/// jogador) ajustado pela pessoa do arbitro (0x25) e pela gravidade; segundo amarelo possivel com
/// 0x24; vermelho (-1000, tipo 4) contra sorteio(15) + 7; senao amarelo (desconto, tipo 2/3) contra
/// sorteio(3 x 0x24 + 5) + 2, ou so tipo 9. O codigo do registro fica no lugar do argumento `j`
/// (sem gravidade conhecida, o byte baixo do ponteiro).
pub(crate) unsafe extern "thiscall" fn card_by_severity(this: usize, j: usize, kind: *mut u8, severity: u8, flag: u8) {
    let j = MatchPlayer(j);
    let mut rating = random((j.fouls() as i8) as u8 as i32) as u8;
    let person = *((this + 4) as *const usize);
    if person != 0 {
        let a = read_i8(person + 0x25);
        rating = rating.wrapping_add(if a > 18 { 2 } else if a > 12 { 1 } else if a > 6 { 0 } else if a > 2 { 0xff } else { 0xfe });
    }
    let (soma, codigo) = match severity {
        1 | 6 => (4, 5),
        2 => (10, 8),
        3 => (7, 1),
        4 | 5 => (100, 12),
        _ => (0, j.addr() as u8),
    };
    rating = rating.wrapping_add(soma);
    if j.yellow_cards() != 0 && (severity == 1 || severity == 6) && random(read_i8(person + 0x24) as i32) != 0 {
        *kind = 9;
        return;
    }
    let active = j.position() >= 0 && flag == 0;
    if severity != 1 && rating as i8 as i32 > random(0xf) + 7 {
        if active {
            let d = (j.addr() + match_player::MATCH_RATING) as *mut i16;
            *d = (*d).wrapping_add(-1000);
            *kind = 4;
            let t = j.red_cards() as i32 + j.yellow_cards() as i32;
            *((j.addr() + if t == 0 { 0x25 } else { 0x26 }) as *mut u8) = codigo;
        } else if codigo == 12 || codigo == 8 {
            record_card_code(j, codigo, true);
        }
        return;
    }
    let r = random(3 * j.yellow_cards() as i32 + 5);
    if rating as i8 as i32 > r + 2 {
        if !active {
            return;
        }
        apply_card_penalty(*(this as *const usize), j);
        let n = (j.addr() + match_player::YELLOW_CARDS) as *mut u8;
        if *n != 0 {
            *n = (*n).wrapping_add(1);
            *kind = 3;
        } else {
            *n = 1;
            *kind = 2;
        }
        record_card_code(j, codigo, false);
    } else if active {
        *kind = 9;
    }
}

/// Forwards to the match referee's card decision (referee object at engine + 0xf5b2) for player `j` with severity
/// `gravidade` and `flag`, writing the card type into `tipo`.
///
/// [pt] { 0x6cfd40 } (match_eng.cpp): `card_by_severity` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn engine_card_by_severity(this: usize, j: usize, kind: *mut u8, severity: u8, flag: u8) {
    exe::match_official_decide_card(this + engine::REFEREE, j, kind, severity, flag)
}

/// Referee: judges foul type `tipo` by player `j` on `outro`. Returns 1 or 7 (card given; 0 = nothing) and writes the
/// card (4, 5, 7, 10, 11), the severity and the `vantagem` (advantage played) flag. `this`: +0 engine, +4 referee (or
/// 0), +8 patience; the strictness factor depends on the referee's attributes, the side's mode, the advantage and the
/// score difference, clamped to 0.1..10; severity rises with the player's foul count and falls with the minute.
///
/// [pt] { 0x6e9820 } (match_man.cpp): o arbitro decide a falta `tipo` do jogador `j` sobre `outro`.
/// Devolve 1 ou 7 (cartao dado; 0 = nada) e escreve o cartao (4, 5, 7, 10, 11), a gravidade e a
/// marca `vantagem`. `this`: +0 motor, +4 arbitro (ou 0), +8 paciencia (palavra). Fator de rigor
/// (dobra, 1,0 sem arbitro): pela severidade (+0x25), pelo modo do lado (+0x1ad do motor) e o
/// estilo (+0x27), pela vantagem e pela diferenca de placar (0xf5c9 + 54 x lado), limitado a
/// 0,1..10. Cada tipo de falta sorteia contra o fator; depois a gravidade sobe pela dureza do
/// jogador (+0x14) e cai pelo minuto (0x8ed0 / 11). Com 0x6cb650, aplica a gravidade em `outro`
/// (0x716450, 0x7164b0) e guarda `j` em +0x1a6 dele.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn foul_decision(
    this: usize,
    j: usize,
    other: usize,
    kind: i8,
    who: i8,
    card: *mut u8,
    grav: *mut i8,
    advantage: *mut u8,
) -> u8 {
    let j = MatchPlayer(j);
    let other = MatchPlayer(other);
    *grav = 0;
    let mut res = 0u8;
    let m = *(this as *const usize);
    if m == 0 {
        return 0;
    }
    if exe::cm3_match_engine_is_foul_pending(m) != 0 {
        return 0;
    }
    let paciencia = (this + 8) as *mut u16;
    let arb = *((this + 4) as *const usize);
    if arb != 0 && random(read_i8(arb + 0x28) as i32 + 0xf) == 0 {
        *paciencia = (*paciencia).wrapping_add(0xff06);
        return 0;
    }
    let minute = (*((m + engine::CLOCK) as *const i16) as i32 / 11) as i8;
    *paciencia = (*paciencia).wrapping_add(0xaf);
    let dura = |t: i8| t == 0x2a || t == 0xc || t == 0x15;
    if arb != 0 && dura(kind) && ((exe::match_player_is_in_penalty_area(j, (0) as u8) as u32) as u8) == 0 && ((exe::match_player_is_in_penalty_area(j, (1) as u8) as u32) as u8) == 0 && random(50 - read_i8(arb + 0x24) as i32) == 0 {
        *paciencia = (*paciencia).wrapping_add(0x19);
        return 0;
    }
    if (((exe::match_player_is_in_penalty_area(j, (0) as u8) as u32) as u8) != 0 || ((exe::match_player_is_in_penalty_area(j, (1) as u8) as u32) as u8) != 0) && (dura(kind) || kind == 0x2e) && random(3) != 0 {
        return 0;
    }
    // advantage: (20, or 5 in the band of its own side) x (+0x5c of `outro`’s profile)^1.5 / (1 or 3)
    let a = if ((exe::match_player_is_in_penalty_area(j, (j.side()) as u8) as u32) as u8) != 0 { 5.0 } else { 20.0 };
    let b = read_i8(other.staff() + 0x5c) as f64;
    let div = if other.last_fouled_by() != j.addr() { 1.0 } else { 3.0 };
    *advantage = (random(ftol(a * game_pow(b, read_f64(rdata::F64_1_5)) / div)) == 0) as u8;
    let word = |p: usize, d: i16| *((p + 0x31) as *mut i16) = (*((p + 0x31) as *const i16)).wrapping_add(d);
    if *advantage != 0 {
        word(other.addr(), -4);
    }
    let mut factor = 1.0f64;
    let mut rigor = 0i8;
    if arb != 0 {
        let sev = read_i8(arb + 0x25);
        factor = if sev > 10 { sev as f64 * read_f64(rdata::F64_0_1) } else { (sev as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1) };
        let side_j = j.side();
        let mode = *((m as isize + side_j as isize + 0x1ad) as *const i8) as f64;
        factor += mode / (read_i8(arb + 0x27) as f64 * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_3));
        if j.yellow_cards() != 0 {
            let x = random(21 - sev as i32);
            *grav = (*grav).wrapping_add((x / -5) as i8);
        }
        if *advantage != 0 {
            let y = random(read_i8(arb + 0x28) as i32);
            factor *= read_f64(rdata::F64_1_5) - y as f64 * read_f64(rdata::F64_0_05);
        }
        rigor = match sev {
            20 => 2,
            18..=i8::MAX => 1,
            10..=19 => 0,
            5..=9 => -1,
            _ => (sev > 2) as i8 - 3,
        };
        let scoreboard = *((m + engine::SIDE_FOUL_TALLY + 54 * side_j as usize) as *const u8) as i32;
        if scoreboard > read_i8(arb + 0x27) as i32 + 5 {
            *paciencia = (*paciencia).wrapping_add(0xff6a);
            rigor = rigor.wrapping_add(1);
            let z = random(4);
            factor *= (read_f64(rdata::F64_6) - z as f64) * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_1);
        }
    }
    let partida = *((m + engine::MATCH) as *const usize);
    if partida != 0 {
        let comp = *((partida + 0x14) as *const usize);
        let friendly = *(G_FRIENDLY as *const usize);
        if comp != 0 && friendly != 0 {
            if exe::friendly_is_friendly_competition(friendly, comp) != 0 {
                rigor = rigor.wrapping_sub(1);
            }
        }
    }
    if read_f64(rdata::F64_0_1) > factor {
        factor = 0.1;
    } else if read_f64(rdata::F64_10) < factor || factor.is_nan() {
        factor = 10.0;
    }
    let dureza = j.fouls() as i32;
    let add = |v: i32| *grav = (*grav).wrapping_add(v as i8);
    let by_minute = |limit: i8| {
        if minute < limit {
            *grav = random(*grav as i32) as i8;
        }
    };
    // final adjustment from `outro`’s record (+0x5c): 1 -> `um`; >= 6 -> +1 below 10; otherwise `senao`
    let by_sheet = |um: i32, senao: i32| {
        if *advantage != 0 && !other.is_null() {
            let c = read_i8(other.staff() + 0x5c);
            if c == 1 {
                *grav = (*grav).wrapping_add(um as i8);
            } else if c >= 6 {
                if c < 10 {
                    *grav = (*grav).wrapping_add(1);
                }
            } else {
                *grav = (*grav).wrapping_add(senao as i8);
            }
        }
    };
    let ceiling = |v: f64, k: f64| if v > k { v } else { k };
    match kind {
        0x2a => {
            word(j.addr(), -4);
            if random(ftol(factor * read_f64(rdata::F64_6))) != 0 {
                res = 1;
                add(random(dureza) + 4);
                *card = 4;
                by_minute(10);
            }
        }
        0x2b => {
            word(j.addr(), -3);
            if random(ftol(factor * read_f64(rdata::F64_14))) != 0 {
                res = 7;
                add(random(dureza) + 3);
                *card = 4;
                by_minute(15);
            }
        }
        0x2c => {
            let v = ceiling(factor * read_f64(rdata::F64_10), read_f64(rdata::F64_50));
            word(j.addr(), -12);
            if random(ftol(v)) != 0 {
                res = 1;
                add(random(dureza + 20) + rigor as i32 + 9);
                *card = 10;
                by_sheet(3, 2);
            }
        }
        0x2d => {
            word(j.addr(), -4);
            if random(ftol(factor * read_f64(rdata::F64_20))) != 0 {
                if ((exe::match_player_is_in_penalty_area(j, (j.side()) as u8) as u32) as u8) == 0 || random(0x12) == 0 {
                    res = 1;
                    add(random(dureza) + rigor as i32 + 4);
                    *card = 7;
                }
                by_minute(6);
            }
        }
        0x2e => {
            word(j.addr(), -1);
            let r = random(ftol(factor * read_f64(rdata::F64_6)));
            if r != 0 || (!other.is_null() && player_table(other.addr()) == 1 && random(100) != 0) {
                res = 1;
                add(random(dureza) + rigor as i32 + 1);
                *card = 4;
                by_minute(10);
            }
        }
        0x2f | 0x60 => {
            let (v, d) = if kind == 0x2f {
                (ceiling(factor * read_f64(rdata::F64_10), read_f64(rdata::F64_100)), -9)
            } else {
                (factor * read_f64(rdata::F64_20), -25)
            };
            word(j.addr(), d);
            if random(ftol(v)) != 0 {
                res = 1;
                let y = random(if kind == 0x2f { dureza } else { dureza + 20 });
                add(y + rigor as i32 + 9);
                *card = 5;
                by_sheet(5, 3);
            }
        }
        0x5f => {
            word(j.addr(), -12);
            if random(ftol(factor * read_f64(rdata::F64_10))) != 0 {
                res = 1;
                add(random(dureza + 7) + rigor as i32 + 3);
                *card = 5;
            }
        }
        0x61 => {
            word(j.addr(), -25);
            if random(ftol(factor + factor)) != 0 {
                res = 1;
                add(random(dureza) + rigor as i32 + 20);
                *card = 5;
            }
        }
        0x30 | 0x31 | 0x32 => {
            let (k, d, extra, mais) = match kind {
                0x30 => (0x968878, -100, 0, 20),
                0x31 => (0x968908, -200, 10, 20),
                _ => (0x968908, -60, 0, 18),
            };
            word(j.addr(), d);
            if random(ftol(factor * read_f64(k))) != 0 {
                res = 1;
                add(random(dureza + extra) + rigor as i32 + mais);
                *card = 11;
            }
        }
        0x13 | 0x21 | 0x22 => {
            let follows = random(ftol(read_f64(rdata::F64_75) / factor)) == 0
                || (who == 0x6c && random(if *advantage != 0 { 2 } else { 6 }) == 0);
            if follows && (((exe::match_player_is_in_penalty_area(j, (j.side()) as u8) as u32) as u8) == 0 || random(ftol(read_f64(rdata::F64_18) / factor)) == 0) {
                res = 1;
                let a = random(dureza);
                let b = random(5);
                add(a + b + rigor as i32);
                *card = 7;
                by_sheet(5, 3);
            }
        }
        0x23 | 0x0c | 0x24 => {
            if random(ftol(read_f64(rdata::F64_150) / factor)) == 0 {
                res = if kind == 0x0c { 7 } else { 1 };
                add(random(dureza) + 1);
                *card = 4;
                if kind == 0x24 {
                    by_sheet(3, 2);
                }
            }
        }
        0x15 | 0x38 => {
            let r = random(ftol(read_f64(rdata::F64_150) / factor));
            if r == 0 || (!other.is_null() && player_table(other.addr()) == 1 && random(5) == 0) {
                res = 1;
                add(random(dureza) + 1);
                *card = 4;
                by_minute(15);
            }
        }
        0x29 => {
            if random(ftol(read_f64(rdata::F64_100) / factor)) == 0 {
                res = 1;
                add(random(dureza) + 2);
                *card = 7;
                by_minute(10);
            }
        }
        0x01 => {
            if random(ftol(read_f64(rdata::F64_10000) / factor)) == 0 {
                res = 1;
                add(random(dureza) + 1);
                *card = 4;
                by_minute(10);
            }
        }
        _ => {}
    }
    if exe::is_undisputed_action_code(m, kind) != 0 {
        exe::match_player_scale_anger(other, (*grav as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_1)) as f32);
        if exe::match_player_passes_anger_check(other, 1, 0) != 0 {
            other.set_anger((((other.anger() as u8)).wrapping_add(1)) as i8);
            if exe::match_player_passes_anger_check(other, 1, 0) != 0 {
                other.set_last_fouled_by(j.addr());
            }
        }
    }
    res
}

/// the referee constructor (zeroes +0 and +4)
/// Original: `construct_referee` (0x6e9460, match_official.cpp, thiscall).
///
/// [pt] { 0x6e9460 } (thiscall, match_official.cpp): o construtor do árbitro: zera o motor (+0) e o oficial
/// (+4). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construct_referee(a: usize) -> usize {
    core::ptr::write_unaligned(a as *mut u32, 0);
    core::ptr::write_unaligned((a + 4) as *mut u32, 0);
    a
}

