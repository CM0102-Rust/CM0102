use super::*;

/// Initialises the referee object (engine, official, strictness word +8): 5000 without an official; otherwise the first
/// of 7750, 7250, 6750, 6250, 5750 for which the official's word +0x1e beats random(350, 300, 250, 200, 150), or 5250.
///
/// [pt] { 0x6e9470 }: inicia o objeto (motor, pessoa, valor): sem pessoa, 5000; senao o primeiro de
/// 7750, 7250, 6750, 6250, 5750 cuja palavra 0x1e da pessoa passa de sorteio(350, 300, ...), ou 5250.
pub(crate) unsafe extern "thiscall" fn init_strictness(this: usize, motor: u32, pessoa: usize) {
    *(this as *mut u32) = motor;
    *((this + 4) as *mut u32) = pessoa as u32;
    let v = (this + 8) as *mut u16;
    if pessoa == 0 {
        *v = 0x1388;
        return;
    }
    for (n, valor) in [(0x15e, 0x1e46u16), (0x12c, 0x1c52), (0xfa, 0x1a5e), (0xc8, 0x186a), (0x96, 0x1676)] {
        let r = sortear(n);
        if *((pessoa + staff::SECOND_NATION) as *const i16) as i32 > r {
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
    let p = *((this + 0x4761) as *const usize);
    if byte(this, 0x475a) == 0 || p == 0 {
        return 0;
    }
    let l = byte(p, 0x27);
    let dono = *((this + 0xf582) as *const usize);
    if dono == 0 || byte(dono, 0x27) as i32 != (l == 0) as i32 {
        return 1;
    }
    let (y, linha) = (byte(this, 0x8ea8), byte(this, 0x475c));
    if (y > linha && l == 1) || (y < linha && l == 0) {
        clear_move(this);
        return 0;
    }
    1
}

/// Referee decision on whether an incident on player `j` becomes a call: none while a foul is pending or for a
/// goalkeeper in his box; the referee may let it go (random(refereeing +0x28 + 15) == 0, strictness -250), and in the
/// box 3 times in 4; otherwise strictness +175 and, by `codigo` (0x25: 19 in 20; others 1 in 155 when engine +0x8ea9 >
/// 1), writes 4 to `tipo` and random(player +0x14) + 1 to `valor` and returns 1; else 0.
///
/// [pt] { 0x6e9540 }: decide se o lance vira falta (ou similar) para o jogador: nao com lance valendo, nem
/// com papel 1 na area; a pessoa (0x4) pode anular (sorteio(0x28 + 15) == 0, -250 no valor 0x8);
/// na area, 3 em 4 anula; senao +175 e, pelo codigo (0x25: 1 em 20; outros: 1 em 155 com 0x8ea9 > 1),
/// grava 4 e sorteio(0x14 do jogador) + 1 nas saidas.
pub(crate) unsafe extern "thiscall" fn decide_foul(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_FOUL_PENDING);
    if valendo(motor) != 0 {
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let in_the_box: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    if tabela(motor, j) & 1 != 0 && in_the_box(j, lado(j) as u8) as u8 != 0 {
        return 0;
    }
    let pessoa = *((this + 4) as *const usize);
    let v = (this + 8) as *mut u16;
    if pessoa != 0 && sortear(byte(pessoa, 0x28) as i32 + 0xf) == 0 {
        *v = (*v).wrapping_add(0xff06);
        return 0;
    }
    if in_the_box(j, lado(j) as u8) as u8 != 0 && sortear(4) != 0 {
        return 0;
    }
    *v = (*v).wrapping_add(0xaf);
    let marca = if codigo == 0x25 { sortear(0x14) != 0 } else { sortear(0x9b) == 0 && byte(motor, 0x8ea9) > 1 };
    if !marca {
        return 0;
    }
    let r = sortear(byte(j, 0x14) as u8 as i32);
    *valor = (r as u8).wrapping_add(1);
    *tipo = 4;
    1
}

/// Yellow card: the first stores x at +0x25 (type 2, weight 2.0); the second at +0x26 (type 3, weight 3.5).
///
/// [pt] Cartao amarelo: o primeiro grava `x` em 0x25 (0x24 = 1, tipo 2, peso 2,0); o segundo, em 0x26
/// (0x24 + 1, tipo 3, peso 3,5).
pub(crate) unsafe fn give_yellow_card(j: usize, tipo: *mut u8, x: i8, peso: *mut f32) {
    let n = (j + 0x24) as *mut u8;
    if *n != 0 {
        *n = (*n).wrapping_add(1);
        *tipo = 3;
        *peso = 3.5;
        *((j + 0x26) as *mut i8) = x;
    } else {
        *n = 1;
        *tipo = 2;
        *peso = 2.0;
        *((j + 0x25) as *mut i8) = x;
    }
}

/// Reduces the player's disposition for a card, scaled by the engine value 0x8ed0 / 11 when above 45 [inference].
///
/// [pt] Desconto na disposicao pelo cartao: ftol(c x c') ou, com 0x8ed0 / 11 acima de 45, ftol(c'' / (0x8ed0 / 11) x c').
pub(crate) unsafe fn apply_card_penalty(motor: usize, j: usize) {
    let q = (*((motor + 0x8ed0) as *const i16) as i32 / 11) as i16;
    let base = if q > 0x2d { read_f32_at(0x96d64c) as f64 / q as f64 } else { read_f32_at(0x968e70) as f64 }; // 0x96d64c = 45, 0x968e70 = 1
    let d = (j + 0x35) as *mut i16;
    *d = (*d).wrapping_add(match_pl_ftol(base * read_f32_at(0x96d648) as f64) as i16); // 0x96d648 = -225
}

/// Increments the side's foul counter in the engine (0xf5c9 + 54 × side).
///
/// [pt] Contador de faltas do lado no motor (0xf5c9 + 54 x lado).
pub(crate) unsafe fn count_foul(motor: usize, j: usize) {
    let c = (motor as isize + 0xf5c9 + 54 * lado(j) as isize) as *mut u8;
    *c = (*c).wrapping_add(1);
}

/// Calls the match player is_in_penalty_area check for the player.
pub(crate) unsafe fn is_in_penalty_area(j: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    f(j, lado(j) as u8) as u8 != 0
}

/// The referee's punishment for a foul by player `j`: counts the foul; for a type 1 foul, either a straight red (type
/// 4, -1000 disposition), a yellow card by severity against random draws, or only -25; with `flag` (or for the
/// goalkeeper) a draw out of 3000 gives a red above 475 (-750, type 6) or a yellow (-375) above 50 or inside the box.
/// Writes the card type to `tipo` and the weight to `peso`.
///
/// [pt] { 0x6e9630 }: punicao da falta (tipo 1): vermelho direto (-1000, tipo 4), amarelo pela
/// severidade contra sorteios, ou so -25; com `flag` (ou papel 1), sorteio(3000): acima de 475
/// vermelho (-750, tipo 6, 9 no registro), acima de 50 (ou na area) amarelo com -375.
pub(crate) unsafe extern "thiscall" fn punish_foul(this: usize, j: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32) {
    *peso = 1.5;
    let motor = *(this as *const usize);
    count_foul(motor, j);
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    if *tipo != 1 || byte(motor, 0x8eb3) != 0 {
        return;
    }
    let d = (j + 0x35) as *mut i16;
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    if flag == 0 && tabela(motor, j) & 1 == 0 {
        if severidade as i32 > sortear(0xf) + 9 {
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            *peso = 5.0;
        } else if severidade as i32 > sortear(5) + 3 {
            apply_card_penalty(motor, j);
            give_yellow_card(j, tipo, x, peso);
        } else {
            *d = (*d).wrapping_add(-25);
        }
        return;
    }
    let r = sortear(0xbb8) as i16;
    if r > 0x1db {
        *d = (*d).wrapping_add(-750);
        *tipo = 6;
        *peso = 5.0;
        *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
        return;
    }
    if r <= 0x32 && !is_in_penalty_area(j) {
        return;
    }
    *d = (*d).wrapping_add(-375);
    give_yellow_card(j, tipo, x, peso);
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
pub(crate) unsafe extern "thiscall" fn punish_move(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, severidade: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    *peso = 1.5;
    if byte(j, 0x19) < 0 {
        *peso = 10.0;
        return;
    }
    let motor = *(this as *const usize);
    let d = (j + 0x35) as *mut i16;
    match *tipo {
        7 => {
            *peso = 1.25;
            count_foul(motor, j);
        }
        1 if byte(motor, 0x8eb3) == 0 => {
            count_foul(motor, j);
            if flag != 0 {
                let r = sortear(0xbb8) as i16;
                if r > 0x177 {
                    *d = (*d).wrapping_add(-750);
                    *tipo = 5;
                    *peso = 5.0;
                    *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut u8) = 9;
                } else if r > 0x32 || is_in_penalty_area(j) {
                    apply_card_penalty(motor, j);
                    give_yellow_card(j, tipo, x, peso);
                }
            } else if severidade as i32 > sortear(0xf) + 7 {
                *d = (*d).wrapping_add(-1000);
                *tipo = 4;
                *peso = 5.0;
                *((j + if byte(j, 0x24) != 0 { 0x26 } else { 0x25 }) as *mut i8) = x;
                if expulsar != 0 {
                    *((j + 0x16) as *mut u8) = 1;
                }
            } else if severidade as i32 > sortear(6) {
                apply_card_penalty(motor, j);
                give_yellow_card(j, tipo, x, peso);
            } else {
                *d = (*d).wrapping_add(-25);
            }
        }
        _ => return,
    }
    *((j + 0x14) as *mut u8) = (*((j + 0x14) as *const u8)).wrapping_add(1);
    *((outro + 0x15) as *mut u8) = (*((outro + 0x15) as *const u8)).wrapping_add(1);
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
pub(crate) unsafe extern "thiscall" fn look_for_offside(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let motor = *(this as *const usize);
    let valendo: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_FOUL_PENDING);
    if valendo(motor) != 0 {
        return 0;
    }
    let (l0, l1) = (byte(motor, 0x8eaf), byte(motor, 0x8eb0));
    match lado_b {
        0 if !(y > linha && y >= l0) => return 0,
        1 if !(y < linha && y <= l1) => return 0,
        _ => {}
    }
    if passador != 0 {
        let alem: unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_COUNT_PLAYERS_AHEAD_OF_ROW);
        if alem(motor, linha, lado_b, 0) == 0 {
            return 0;
        }
    }
    let v = (this + 8) as *mut i16;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 && sortear(byte(pessoa, 0x29) as i32 + 0x19) == 0 {
        *v = (*v).wrapping_add(-75);
        return 0;
    }
    let in_the_box: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    for k in 0..20 {
        let p = side_player_slot(motor, lado_b, k);
        if byte(p, 0x19) < 0 || p == passador {
            continue;
        }
        if passador != 0 {
            if in_the_box(passador, (lado_b == 0) as u8) as u8 == 0 {
                continue;
            }
            let dist = match_player_distance_to_cell(p, 4, y);
            if ((sortear(4) + 1) as f64) < dist as f64 || dist.is_nan() {
                continue;
            }
        }
        let (py, a) = (byte(p, 0x103) as i32, byte(p, 0x107) as i32);
        let (linha_d, sinal, perto) = match lado_b {
            0 => (l0 as i32, 1, -25i16),
            1 => (l1 as i32, -1, -15i16),
            _ => continue,
        };
        if py == linha_d - 2 * sinal && sortear(6 * a + 500) == 0 {
            *v = (*v).wrapping_add(perto);
            return p;
        }
        if py == linha_d - sinal && sortear(match_pl_ftol((a as f64) * (a as f64) * (a as f64) + read_f64_at(0x968890))) == 0 { // 0x968890 = 25
            *v = (*v).wrapping_add(-15);
            return p;
        }
        if py == linha_d && sortear(3) != 0 && a < sortear(0x12) {
            *v = (*v).wrapping_add(15);
            return p;
        }
        if (py - linha_d) * sinal > 0 && a < sortear(0x64) {
            return p;
        }
    }
    0
}

/// Forwards to the match engine's referee (this + 0xf5b2) the search for an offside player of side `lado_b` relative to
/// the line, row `y` and the passer. Returns the offside player or 0.
///
/// [pt] { 0x6cfd70 } (match_eng.cpp): `look_for_offside` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn engine_look_for_offside(this: usize, lado_b: i8, linha: i8, y: i8, passador: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, usize) -> usize = core::mem::transmute(FN_MATCH_OFFICIAL_FIND_OFFSIDE_PLAYER);
    f(this + 0xf5b2, lado_b, linha, y, passador)
}

/// Writes the card code in the player's record: +0x25 when the card total is 1 (or ≤ 1), else +0x26 when non-zero.
///
/// [pt] Grava o codigo do cartao no registro do jogador: 0x25 com (0x23 + 0x24) == 1 (ou <= 1, `ate_um`),
/// 0x26 com outro total diferente de zero.
pub(crate) unsafe fn record_card_code(j: usize, codigo: u8, ate_um: bool) {
    let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
    if t == 1 || (ate_um && t <= 1) {
        *((j + 0x25) as *mut u8) = codigo;
    } else if t != 0 {
        *((j + 0x26) as *mut u8) = codigo;
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
pub(crate) unsafe extern "thiscall" fn card_by_severity(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let mut nota = sortear(byte(j, 0x14) as u8 as i32) as u8;
    let pessoa = *((this + 4) as *const usize);
    if pessoa != 0 {
        let a = byte(pessoa, 0x25);
        nota = nota.wrapping_add(if a > 18 { 2 } else if a > 12 { 1 } else if a > 6 { 0 } else if a > 2 { 0xff } else { 0xfe });
    }
    let (soma, codigo) = match gravidade {
        1 | 6 => (4, 5),
        2 => (10, 8),
        3 => (7, 1),
        4 | 5 => (100, 12),
        _ => (0, j as u8),
    };
    nota = nota.wrapping_add(soma);
    if byte(j, 0x24) != 0 && (gravidade == 1 || gravidade == 6) && sortear(byte(pessoa, 0x24) as i32) != 0 {
        *tipo = 9;
        return;
    }
    let ativo = byte(j, 0x19) >= 0 && flag == 0;
    if gravidade != 1 && nota as i8 as i32 > sortear(0xf) + 7 {
        if ativo {
            let d = (j + 0x35) as *mut i16;
            *d = (*d).wrapping_add(-1000);
            *tipo = 4;
            let t = byte(j, 0x23) as i32 + byte(j, 0x24) as i32;
            *((j + if t == 0 { 0x25 } else { 0x26 }) as *mut u8) = codigo;
        } else if codigo == 12 || codigo == 8 {
            record_card_code(j, codigo, true);
        }
        return;
    }
    let r = sortear(3 * byte(j, 0x24) as i32 + 5);
    if nota as i8 as i32 > r + 2 {
        if !ativo {
            return;
        }
        apply_card_penalty(*(this as *const usize), j);
        let n = (j + 0x24) as *mut u8;
        if *n != 0 {
            *n = (*n).wrapping_add(1);
            *tipo = 3;
        } else {
            *n = 1;
            *tipo = 2;
        }
        record_card_code(j, codigo, false);
    } else if ativo {
        *tipo = 9;
    }
}

/// Forwards to the match referee's card decision (referee object at engine + 0xf5b2) for player `j` with severity
/// `gravidade` and `flag`, writing the card type into `tipo`.
///
/// [pt] { 0x6cfd40 } (match_eng.cpp): `card_by_severity` do arbitro do motor (this + 0xf5b2).
pub(crate) unsafe extern "thiscall" fn engine_card_by_severity(this: usize, j: usize, tipo: *mut u8, gravidade: u8, flag: u8) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut u8, u8, u8) = core::mem::transmute(FN_MATCH_OFFICIAL_DECIDE_CARD);
    f(this + 0xf5b2, j, tipo, gravidade, flag)
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
    outro: usize,
    tipo: i8,
    quem: i8,
    cartao: *mut u8,
    grav: *mut i8,
    vantagem: *mut u8,
) -> u8 {
    *grav = 0;
    let mut res = 0u8;
    let m = *(this as *const usize);
    if m == 0 {
        return 0;
    }
    let parado: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_FOUL_PENDING);
    if parado(m) != 0 {
        return 0;
    }
    let paciencia = (this + 8) as *mut u16;
    let arb = *((this + 4) as *const usize);
    if arb != 0 && sortear(byte(arb, 0x28) as i32 + 0xf) == 0 {
        *paciencia = (*paciencia).wrapping_add(0xff06);
        return 0;
    }
    let minuto = (*((m + 0x8ed0) as *const i16) as i32 / 11) as i8;
    *paciencia = (*paciencia).wrapping_add(0xaf);
    let faixa_l: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_IS_IN_PENALTY_AREA);
    let dura = |t: i8| t == 0x2a || t == 0xc || t == 0x15;
    if arb != 0 && dura(tipo) && faixa_l(j, 0) == 0 && faixa_l(j, 1) == 0 && sortear(50 - byte(arb, 0x24) as i32) == 0 {
        *paciencia = (*paciencia).wrapping_add(0x19);
        return 0;
    }
    if (faixa_l(j, 0) != 0 || faixa_l(j, 1) != 0) && (dura(tipo) || tipo == 0x2e) && sortear(3) != 0 {
        return 0;
    }
    // vantagem: (20, ou 5 na faixa do proprio lado) x (+0x5c da ficha de `outro`)^1,5 / (1 ou 3)
    let a = if faixa_l(j, byte(j, 0x27)) != 0 { 5.0 } else { 20.0 };
    let b = byte(*((outro + 0x69) as *const usize), 0x5c) as f64;
    let div = if *((outro + 0x1a6) as *const usize) != j { 1.0 } else { 3.0 };
    *vantagem = (sortear(match_pl_ftol(a * game_pow(b, read_f64_at(0x968940)) / div)) == 0) as u8; // 0x968940 = 1.5
    let palavra = |p: usize, d: i16| *((p + 0x31) as *mut i16) = (*((p + 0x31) as *const i16)).wrapping_add(d);
    if *vantagem != 0 {
        palavra(outro, -4);
    }
    let mut fator = 1.0f64;
    let mut rigor = 0i8;
    if arb != 0 {
        let sev = byte(arb, 0x25);
        fator = if sev > 10 { sev as f64 * read_f64_at(0x967880) } else { (sev as i32 - 10) as f64 * read_f64_at(0x967888) + read_f64_at(0x967890) }; // 0x967880 = 0.1, 0x967888 = 0.05, 0x967890 = 1
        let lado_j = byte(j, 0x27);
        let modo = *((m as isize + lado_j as isize + 0x1ad) as *const i8) as f64;
        fator += modo / (byte(arb, 0x27) as f64 * read_f64_at(0x968968) + read_f64_at(0x968930)); // 0x968968 = 0.5, 0x968930 = 3
        if byte(j, 0x24) != 0 {
            let x = sortear(21 - sev as i32);
            *grav = (*grav).wrapping_add((x / -5) as i8);
        }
        if *vantagem != 0 {
            let y = sortear(byte(arb, 0x28) as i32);
            fator *= read_f64_at(0x968940) - y as f64 * read_f64_at(0x967888); // 0x968940 = 1.5, 0x967888 = 0.05
        }
        rigor = match sev {
            20 => 2,
            18..=i8::MAX => 1,
            10..=19 => 0,
            5..=9 => -1,
            _ => (sev > 2) as i8 - 3,
        };
        let placar = *((m + 0xf5c9 + 54 * lado_j as usize) as *const u8) as i32;
        if placar > byte(arb, 0x27) as i32 + 5 {
            *paciencia = (*paciencia).wrapping_add(0xff6a);
            rigor = rigor.wrapping_add(1);
            let z = sortear(4);
            fator *= (read_f64_at(0x969000) - z as f64) * read_f64_at(0x967880) + read_f64_at(0x967890); // 0x969000 = 6, 0x967880 = 0.1, 0x967890 = 1
        }
    }
    let partida = *((m + 0x4792) as *const usize);
    if partida != 0 {
        let comp = *((partida + 0x14) as *const usize);
        let amistoso = *(G_FRIENDLY as *const usize);
        if comp != 0 && amistoso != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
            if f(amistoso, comp) != 0 {
                rigor = rigor.wrapping_sub(1);
            }
        }
    }
    if read_f64_at(0x967880) > fator { // 0x967880 = 0.1
        fator = 0.1;
    } else if read_f64_at(0x9688b0) < fator || fator.is_nan() { // 0x9688b0 = 10
        fator = 10.0;
    }
    let dureza = *((j + 0x14) as *const u8) as i32;
    let somar = |v: i32| *grav = (*grav).wrapping_add(v as i8);
    let pelo_minuto = |limit: i8| {
        if minuto < limit {
            *grav = sortear(*grav as i32) as i8;
        }
    };
    // ajuste final pela ficha de `outro` (+0x5c): 1 -> `um`; >= 6 -> +1 abaixo de 10; senao `senao`
    let pela_ficha = |um: i32, senao: i32| {
        if *vantagem != 0 && outro != 0 {
            let c = byte(*((outro + 0x69) as *const usize), 0x5c);
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
    let teto = |v: f64, k: f64| if v > k { v } else { k };
    match tipo {
        0x2a => {
            palavra(j, -4);
            if sortear(match_pl_ftol(fator * read_f64_at(0x969000))) != 0 { // 0x969000 = 6
                res = 1;
                somar(sortear(dureza) + 4);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2b => {
            palavra(j, -3);
            if sortear(match_pl_ftol(fator * read_f64_at(0x969558))) != 0 { // 0x969558 = 14
                res = 7;
                somar(sortear(dureza) + 3);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x2c => {
            let v = teto(fator * read_f64_at(0x9688b0), read_f64_at(0x968ff8)); // 0x9688b0 = 10, 0x968ff8 = 50
            palavra(j, -12);
            if sortear(match_pl_ftol(v)) != 0 {
                res = 1;
                somar(sortear(dureza + 20) + rigor as i32 + 9);
                *cartao = 10;
                pela_ficha(3, 2);
            }
        }
        0x2d => {
            palavra(j, -4);
            if sortear(match_pl_ftol(fator * read_f64_at(0x968ea8))) != 0 { // 0x968ea8 = 20
                if faixa_l(j, byte(j, 0x27)) == 0 || sortear(0x12) == 0 {
                    res = 1;
                    somar(sortear(dureza) + rigor as i32 + 4);
                    *cartao = 7;
                }
                pelo_minuto(6);
            }
        }
        0x2e => {
            palavra(j, -1);
            let r = sortear(match_pl_ftol(fator * read_f64_at(0x969000))); // 0x969000 = 6
            if r != 0 || (outro != 0 && player_table(outro) == 1 && sortear(100) != 0) {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        0x2f | 0x60 => {
            let (v, d) = if tipo == 0x2f {
                (teto(fator * read_f64_at(0x9688b0), read_f64_at(0x9688a8)), -9) // 0x9688b0 = 10, 0x9688a8 = 100
            } else {
                (fator * read_f64_at(0x968ea8), -25) // 0x968ea8 = 20
            };
            palavra(j, d);
            if sortear(match_pl_ftol(v)) != 0 {
                res = 1;
                let y = sortear(if tipo == 0x2f { dureza } else { dureza + 20 });
                somar(y + rigor as i32 + 9);
                *cartao = 5;
                pela_ficha(5, 3);
            }
        }
        0x5f => {
            palavra(j, -12);
            if sortear(match_pl_ftol(fator * read_f64_at(0x9688b0))) != 0 { // 0x9688b0 = 10
                res = 1;
                somar(sortear(dureza + 7) + rigor as i32 + 3);
                *cartao = 5;
            }
        }
        0x61 => {
            palavra(j, -25);
            if sortear(match_pl_ftol(fator + fator)) != 0 {
                res = 1;
                somar(sortear(dureza) + rigor as i32 + 20);
                *cartao = 5;
            }
        }
        0x30 | 0x31 | 0x32 => {
            let (k, d, extra, mais) = match tipo {
                0x30 => (0x968878, -100, 0, 20),
                0x31 => (0x968908, -200, 10, 20),
                _ => (0x968908, -60, 0, 18),
            };
            palavra(j, d);
            if sortear(match_pl_ftol(fator * read_f64_at(k))) != 0 {
                res = 1;
                somar(sortear(dureza + extra) + rigor as i32 + mais);
                *cartao = 11;
            }
        }
        0x13 | 0x21 | 0x22 => {
            let segue = sortear(match_pl_ftol(read_f64_at(0x96d520) / fator)) == 0 // 0x96d520 = 75
                || (quem == 0x6c && sortear(if *vantagem != 0 { 2 } else { 6 }) == 0);
            if segue && (faixa_l(j, byte(j, 0x27)) == 0 || sortear(match_pl_ftol(read_f64_at(0x969550) / fator)) == 0) { // 0x969550 = 18
                res = 1;
                let a = sortear(dureza);
                let b = sortear(5);
                somar(a + b + rigor as i32);
                *cartao = 7;
                pela_ficha(5, 3);
            }
        }
        0x23 | 0x0c | 0x24 => {
            if sortear(match_pl_ftol(read_f64_at(0x968f50) / fator)) == 0 { // 0x968f50 = 150
                res = if tipo == 0x0c { 7 } else { 1 };
                somar(sortear(dureza) + 1);
                *cartao = 4;
                if tipo == 0x24 {
                    pela_ficha(3, 2);
                }
            }
        }
        0x15 | 0x38 => {
            let r = sortear(match_pl_ftol(read_f64_at(0x968f50) / fator)); // 0x968f50 = 150
            if r == 0 || (outro != 0 && player_table(outro) == 1 && sortear(5) == 0) {
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(15);
            }
        }
        0x29 => {
            if sortear(match_pl_ftol(read_f64_at(0x9688a8) / fator)) == 0 { // 0x9688a8 = 100
                res = 1;
                somar(sortear(dureza) + 2);
                *cartao = 7;
                pelo_minuto(10);
            }
        }
        0x01 => {
            if sortear(match_pl_ftol(read_f64_at(0x968910) / fator)) == 0 { // 0x968910 = 10000
                res = 1;
                somar(sortear(dureza) + 1);
                *cartao = 4;
                pelo_minuto(10);
            }
        }
        _ => {}
    }
    let aplica: unsafe extern "thiscall" fn(usize, i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    if aplica(m, tipo) != 0 {
        let multiplicar: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(FN_MATCH_PLAYER_SCALE_ANGER);
        let teste: unsafe extern "thiscall" fn(usize, i8, usize) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_PASSES_ANGER_CHECK);
        multiplicar(outro, (*grav as f64 * read_f64_at(0x968918) + read_f64_at(0x967890)) as f32); // 0x968918 = 0.2, 0x967890 = 1
        if teste(outro, 1, 0) != 0 {
            *((outro + 0xff) as *mut u8) = (*((outro + 0xff) as *const u8)).wrapping_add(1);
            if teste(outro, 1, 0) != 0 {
                *((outro + 0x1a6) as *mut usize) = j;
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







