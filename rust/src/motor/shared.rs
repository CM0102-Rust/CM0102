use super::*;

/// Port of `exe::cm3_match_engine_ball_out_of_play` with commentary injected: releases the ball and, if out of play, sets the restart (throw-in/goal
/// kick/corner) and side, narrates and returns 1; 0 in play.
///
/// [pt] { 0x6cd490 } (match_eng.cpp) com o comentario injetado: com o lance ja parado (0x8eb2), 1.
/// Solta a bola do dono e, com a bola fora do campo, marca a reposicao (0x8eb2: 2 lateral, 3 ou
/// 4 conforme a linha de fundo e o lado que tocou por ultimo; 0x8eae: o lado que repoe), narra
/// (0x1f76/0x1f75/0x1f77) e devolve 1. Bola em campo, 0.
pub(crate) unsafe fn ball_out_with(this: usize, side_b: u8, call: AddEventCall) -> u8 {
    if byte(this, engine::RESTART_TYPE) != 0 {
        return 1;
    }
    if byte(this, engine::POSSESSION_SIDE) != -1 {
        *((this + engine::POSSESSION_SIDE) as *mut i8) = -1;
    }
    let owner = *((this + engine::BALL_HOLDER) as *const usize);
    if owner != 0 {
        exe::match_player_release_possession(owner);
    }
    let (x, y) = (byte(this, engine::BALL_X), byte(this, engine::BALL_Y));
    let commentator = *((this + engine::COMMENTATOR) as *const usize);
    let (parada, quem, codigo): (u8, u8, u32) = if !(0..9).contains(&x) {
        let r = (side_b == 0) as u8;
        (2, r, 0x1f76)
    } else if y < 0 {
        if side_b == 1 { (4, 0, 0x1f77) } else { (3, 1, 0x1f75) }
    } else if y >= 0xc {
        if side_b == 1 { (3, 0, 0x1f75) } else { (4, 1, 0x1f77) }
    } else {
        return 0;
    };
    *((this + engine::RESTART_TYPE) as *mut u8) = parada;
    *((this + engine::POSSESSION_SIDE) as *mut u8) = quem;
    call(commentator, [codigo, this as u32, u32::MAX, quem as u32, u32::MAX, u32::MAX, 0]);
    1
}

/// Player `j` attempts the play against the opponent's `forca` (height `altura`, `estilo` which goes to the narration,
/// `forte` in the air): `0x70dfd0` (ret 0x10, `match_pl.cpp`) with the calls that touch things outside the engine
/// injected. Returns 1 if the play resolved on him, 0 if not.
///
/// Injected: `0x6d6f70` narrates, `0x716e10` tires, `0x6cd770` the ball breaks loose, `0x6f0630` control; and those
/// inside the leaving-the-pitch path.
///
/// The side's goalkeeper, the taker (`+0xf582`) and, on the side without the ball, anyone with `+0x10d` zeroed do not
/// play. The attack is a random draw by the engine's `+0x8ea9` (300, 400, 500, 1000, 1800), divided by height + 1. With
/// the ball on his own side (`+0x8eab`) and random draw(attack + 100) == 0: 0x1f5e, −300 in `+0x4782` and the ball
/// breaks loose.
///
/// Otherwise the move and the attribute as in `0x6f5c30`, but by `+0x8ea9` (4: control 1 with the goalkeeper's `+0x11c`
/// or 0x15 with `+0x144`; for non-goalkeepers 0x24 with `+0x162`; 3 by random draw(4) control; otherwise 0x29 with
/// `+0x16c`; the header 0x25 counts double `+0x164`). With `forte` the attribute counts 85%. The score is random
/// draw(attribute) − `forca` / 3 − attack: below −100 he only gains random draw(2) in `+0x2b`; negative, nothing. Hard
/// foul: `forca` > random draw(150) + 200.
///
/// The referee looks at the foul (`0x6cfca0`): 1 in 5, with no hard foul, records it (`0x6bb9a0`) and carries on
/// without punishment; otherwise punishes (`0x6cfd10`, as in `0x6f5c30`, but the penalty only takes 500 from `+0x35`).
///
/// Without punishment: a score of 400 wins (`+0x1e` in `+0x35`, `+0x12`, `+0x2b`; 0x1ffe from the D, 0x21d6 in the air,
/// 0x1f66/0x1f65 with the style; `0x6f0630`; marks the move); otherwise the side's goalkeeper narrates 0x1fe7; from the
/// D with a score above 200, 0x1ffd and the ball breaks loose with no shooter; otherwise 0x21d8/0x1f85/0x1f84 and the
/// ball breaks loose (random draw(3)). In those three: `+0x14` in `+0x35`, `+0x12`, marks the move and random draw(2) +
/// 1 in `+0x2b`.
///
/// [pt] { 0x70dfd0 } (ret 0x10, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x716e10 cansa, 0x6cd770 a bola espirra, 0x6f0630 dominio; e as de dentro da saida de
/// campo): o jogador `j` tenta a jogada contra a `forca` do adversario (altura `altura`, `estilo`
/// que vai para a narracao, `forte` pelo alto). Devolve 1 se a jogada se resolveu nele, 0 se nao.
/// O goleiro do lado, quem cobra (+0xf582) e, do lado sem a bola, quem tem +0x10d zerado nao jogam.
/// O ataque e sorteio pelo +0x8ea9 do motor (300, 400, 500, 1000, 1800), dividido por altura + 1.
/// Com a bola no proprio lado (+0x8eab) por sorteio(ataque + 100) == 0: 0x1f5e, -300 em +0x4782 e
/// a bola espirra. Senao o lance e o atributo como na 0x6f5c30, mas pelo +0x8ea9 (4: domínio 1 com
/// +0x11c do goleiro ou 0x15 com +0x144; fora do goleiro 0x24 com +0x162; 3 por sorteio(4) o
/// domínio; senao 0x29 com +0x16c; a cabecada 0x25 vale o dobro do +0x164). Com `forte` o atributo
/// vale 85%. A nota e sorteio(atributo) - forca / 3 - ataque: abaixo de -100 so ganha sorteio(2)
/// em +0x2b; negativa, nada. Falta forte: forca > sorteio(150) + 200. O arbitro olha a falta
/// (0x6cfca0): 1 em 5, sem falta forte, a registra (0x6bb9a0) e segue sem punicao; senao pune
/// (0x6cfd10, como na 0x6f5c30, mas o penalti so tira 500 do +0x35). Sem punicao: nota de 400 vence
/// (+0x1e em +0x35, +0x12, +0x2b; 0x1ffe da meia-lua, 0x21d6 pelo alto, 0x1f66/0x1f65 com o estilo;
/// 0x6f0630; marca o lance); senao o goleiro do lado narra 0x1fe7; da meia-lua com nota acima de
/// 200, 0x1ffd e a bola espirra sem chutador; senao 0x21d8/0x1f85/0x1f84 e a bola espirra
/// (sorteio(3)). Nesses tres: +0x14 em +0x35, +0x12, marca o lance e sorteio(2) + 1 em +0x2b.
pub(crate) unsafe fn play_with(j: usize, strength: i16, height: f32, style: i8, strong: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let m = *((j + match_player::ENGINE) as *const usize);
    // the commentary’s seventh argument goes in the high part of the code (see `dispatch_tackle_calls`)
    let narrar7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + engine::COMMENTATOR) as *const usize), [code | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| narrar7(call, code, a3, a4, a5, 0);
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let in_the_box = || area_de(j, byte(j, match_player::SIDE) == 1);
    let espirra = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, spread: i32, shooter: usize| {
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [(read_u8(j + match_player::CELL_X) as u32), (read_u8(j + match_player::CELL_Y) as u32), (spread as u8 as u32) | (read_u8(j + match_player::SIDE) as u32) << 8, 0, shooter as u32]);
    };
    if exe::cm3_match_engine_get_goalkeeper(m, byte(j, match_player::SIDE)) == j || *((m + engine::BALL_HOLDER) as *const usize) == j {
        return 0;
    }
    if byte(m, engine::POSSESSION_SIDE) != byte(j, match_player::SIDE) && byte(j, match_player::MOMENT_WORK_RATE) == 0 {
        return 0;
    }
    let mut attack: i32 = match byte(m, engine::BALL_HEIGHT) {
        0 => random(300),
        1 => random(400),
        2 => random(500),
        3 => random(1000),
        4 => random(0x708),
        _ => {
            // simplification: the original routine shows the “Error” box (version, file, line 0x4755); +0x8ea9
            // outside 0..4 does not happen in the game
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            return 0;
        }
    };
    let st = height as f64 + 1.0;
    if st > 0.0 {
        attack = ftol(attack as i16 as f64 / st);
    }
    if byte(m, engine::LAST_POSSESSION_SIDE) == byte(j, match_player::SIDE) {
        if random(attack as i16 as i32 + 100) != 0 {
            return 0;
        }
        crate::comparison::branch("play bola do proprio lado");
        narrate(call, 0x1f5e, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        let r = random(byte(m, engine::BALL_ROLL) as i32);
        espirra(call, r.wrapping_add(1), j);
        return 1;
    }
    // the move and the attribute
    let em_area = in_the_box();
    let choose = || -> (i16, i16) {
        if byte(m, engine::BALL_HEIGHT) == 4 {
            if exe::cm3_match_engine_get_player_position(m, j) == 1 { (*w(j, 0x11c), 1) } else { (*w(j, 0x144), 0x15) }
        } else if exe::cm3_match_engine_get_player_position(m, j) != 1 {
            (*w(j, 0x162), 0x24)
        } else if byte(m, engine::BALL_HEIGHT) == 3 && random(4) != 0 {
            (*w(j, 0x11c), 1)
        } else {
            (*w(j, 0x16c), 0x29)
        }
    };
    let (mut valor, lance) = if em_area && exe::cm3_match_engine_get_player_position(m, j) == 1 {
        choose()
    } else {
        let ficha5c = byte(*((j + match_player::STAFF) as *const usize), 0x5c) as i32;
        if random(em_area as i32 * 500 + 100 + ficha5c * 200) != 0 {
            choose()
        } else {
            crate::comparison::branch("play cabecada");
            ((*w(j, 0x164)).wrapping_shl(1), 0x25)
        }
    };
    if strong != 0 {
        crate::comparison::branch("play forte");
        valor = ftol(valor as f64 * 0.85) as i16;
    }
    let v = random(valor as i32);
    let rating = (v - strength as i32 / 3 - attack) as i16;
    if rating < -100 {
        crate::comparison::branch("play nota muito baixa");
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(2) as u8);
        return 0;
    }
    if rating < 0 {
        return 0;
    }
    let foul_strong = strength as i32 > random(0x96) + 0xc8;
    let (mut cartao, mut grav) = (0u8, 0u8);
    let mut t = exe::cm3_match_engine_referee_check_foul(m, j, lance as u16, &mut cartao, &mut grav);
    if t != 0 {
        if random(5) == 0 && !foul_strong {
            crate::comparison::branch("play falta registrada");
            exe::cm3_match_engine_set_pending_foul(m, lance as u16, j as u32, 0, 0, t, *b(j, match_player::CELL_X), *b(j, match_player::CELL_Y), cartao, grav);
        } else {
            // the foul with a punishment
            crate::comparison::branch("play falta punida");
            let mut weight = 0f32;
            exe::cm3_match_engine_referee_decide_booking(m, j, &mut t, cartao as i8, grav as i8, foul_strong as u8, &mut weight);
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
                crate::comparison::branch("play penalty");
                sound = 0x1f7a;
                *b(m, engine::RESTART_TYPE) = 5;
                *((m + engine::FOULED_PLAYER) as *mut usize) = 0;
                *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_sub(500);
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
                        let club = *((partida + if byte(j, match_player::SIDE) == 0 { 0x20 } else { 0x1c }) as *const usize);
                        let k = *((club + club::NATION) as *const usize);
                        if k != 0 && byte(k, 0x77) == 8 {
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
                let c = exe::cm3_match_engine_pick_restart_taker(m, (byte(j, match_player::SIDE) == 0) as i8, byte(m, engine::RESTART_TYPE), 0);
                if c == 0 {
                    narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                } else {
                    if t == 7 {
                        narrate(call, 0x21e2, (read_u8(c + 0x2e) as u32), zero(j), u32::MAX);
                    } else {
                        narrate(call, sound, u32::MAX, zero(j), u32::MAX);
                    }
                    if sound == 0x1f79 {
                        let far = (byte(c, 0x27) == 1 && byte(c, 0x103) < 4) || (byte(c, 0x27) == 0 && byte(c, 0x103) > 7);
                        if t == 7 {
                            if far && !area_de(c, byte(j, match_player::SIDE) == 1) {
                                narrate(call, 0x21e1, (read_u8(c + 0x2e) as u32), zero(j), u32::MAX);
                            }
                        } else if far {
                            narrate(call, if (2..=6).contains(&byte(j, match_player::CELL_X)) { 0x21e0 } else { 0x21df }, (read_u8(c + 0x2e) as u32), zero(j), u32::MAX);
                        }
                    }
                }
            }
            if special(lance as i8) == 0 {
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
    let half_lua = byte(j, match_player::CELL_X) == 4 && (byte(j, match_player::CELL_Y) == 0 || byte(j, match_player::CELL_Y) == 11);
    if rating >= 400 {
        // venceu
        crate::comparison::branch("play venceu");
        *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x1e);
        *b(j, match_player::BALLS_CONTESTED) = (*b(j, match_player::BALLS_CONTESTED)).wrapping_add(1);
        *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(1);
        if half_lua {
            narrate(call, 0x1ffe, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        } else if strong != 0 {
            narrar7(call, 0x21d6, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style);
        } else if style == 5 {
            narrar7(call, 0x1f66, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, 5);
        } else {
            narrar7(call, 0x1f65, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style);
        }
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, j, [0; 5]);
        *w(j, match_player::ACTION) = lance;
        return 1;
    }
    if exe::cm3_match_engine_get_goalkeeper(m, byte(j, match_player::SIDE)) == j {
        crate::comparison::branch("play goleiro");
        narrate(call, 0x1fe7, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
    } else if half_lua && rating > 200 {
        crate::comparison::branch("play meia-lua");
        narrate(call, 0x1ffd, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX);
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        espirra(call, random(3), 0);
    } else {
        crate::comparison::branch("play perdeu");
        if strong != 0 {
            narrar7(call, 0x21d8, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style);
        } else if style == 5 {
            narrar7(call, 0x1f85, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, 5);
        } else {
            narrar7(call, 0x1f84, (read_u8(j + match_player::COMMENTARY_ID) as u32), (read_u8(j + match_player::SIDE) as u32), u32::MAX, style);
        }
        *((m + engine::MOMENTUM_A) as *mut i32) = (*((m + engine::MOMENTUM_A) as *const i32)).wrapping_sub(300);
        espirra(call, random(3), j);
    }
    *w(j, match_player::MATCH_RATING) = (*w(j, match_player::MATCH_RATING)).wrapping_add(0x14);
    *b(j, match_player::BALLS_CONTESTED) = (*b(j, match_player::BALLS_CONTESTED)).wrapping_add(1);
    *b(j, match_player::BUSY) = (*b(j, match_player::BUSY)).wrapping_add(random(2).wrapping_add(1) as u8);
    *w(j, match_player::ACTION) = lance;
    1
}

pub(crate) use crate::conteiner::{abrir_no_conteiner, buscar_no_conteiner, posicao_no_conteiner};

// simplification: the container opens files, allocates 650 KB per item and reads from disk; without its own comparison,
// validated by the career (Rust x original; the database and the histories pass through here all the time) and
// by the filler save above 2 GB.

/// Port of `exe::match_day_apply_post_match_player_changes` (cdecl, match_day.cpp) with calls injected: post-match updates for both sides' players (news,
/// contract flags, manager ratings). For the two sides of the match record `t` (club at t+0 / t+4; side at t + side x
/// 0x625): the 20 players (rows of 0x49 from +0x1c5; person at -0x35) with a player sheet (+0x61) and from the database
/// (id < [0xae2364]).
///
/// With the match's competition ([[0xadadfc][+0]]+8), the competition's club (+0x57 = match+0x14) and without +0x4c in
/// the record: by contract (`exe::contract_manager_get_contract_for_club`) of type 0x10/0x20/0x30, the test `exe::shortlist_manager_find_club_shortlist_entry`/`exe::human_manager_is_human_club`, rating +3 above 5000 and
/// above 1.1 x +0x18e (or +7 above 0.9 x, or the player sheet's +5 above 0.024 x), +0 up to 6, the side's score not
/// above the other's, random(10) + 5 < rating 1 (`exe::comp_stats_get_player_stat`) and random(10) above the player sheet's +0x45, and rating
/// 0x11 within 0.2 of the competition's average ([0xb74350]+0x10 / +0x18, or 6.8): news item `exe::media_create_player_media_story` type 0x11.
///
/// Also with 0x76d940, +0 from 9 upward, the side's score not below the other's, random(5) + 2 < rating 1 and still no
/// news item 0x34 in the match: with rating 0x11 above random(9) x 0.1 + average + 0.5, 0x744240 (0x34). Then it copies
/// into the player sheet +0x45, +0xb, +9, +0xd (from -4, +3, +5, +7).
///
/// Two clubs of the same nation (+0x53) and -8 in the row: with `get_stat` non-zero, the previous club (+0x1a) of another
/// nation (+0x76), the player sheet's +0xb above 5750, +0x57 above 10, +0x59 below 15, +0x5c or +0x5d below 10 and
/// random(3) == 0 (of the club itself [0x9cf300]) or random(10) == 0: 0x20 in the contract (`set_flags`).
///
/// Finally the manager (person at +0x184): his player sheet (+0x69) gets +0x188/+0x18a/+0x18c in +0xa/+8/+0xc, and the
/// assistant (+0xd3) and the 5 of +0x19f gain a quarter of the difference (between 1 and 10000); the club's +0x80 =
/// +0x18e.
///
/// [pt] { 0x6b6cc0 } (cdecl, match_day.cpp): depois da partida, para os dois lados do registro `t` da
/// partida (clube em t+0 / t+4; lado em t + lado x 0x625): os 20 jogadores (linhas de 0x49 a partir
/// de +0x1c5; pessoa em -0x35) com ficha (+0x61) e da base (id < [0xae2364]). Com a competicao da
/// partida ([[0xadadfc][+0]]+8), o clube da competicao (+0x57 = partida+0x14) e sem +0x4c no
/// registro: por contrato (0x4cf5f0) de tipo 0x10/0x20/0x30, o teste 0x849ce0/0x600a50, a nota +3
/// acima de 5000 e de 1,1 x +0x18e (ou +7 acima de 0,9 x, ou +5 da ficha acima de 0,024 x), +0 ate
/// 6, o placar do lado nao acima do outro, sorteio(10) + 5 < nota 1 (0x4a31a0) e sorteio(10) acima
/// de +0x45 da ficha, e a nota 0x11 a menos de 0,2 da media da competicao ([0xb74350]+0x10 / +0x18,
/// ou 6,8): a noticia 0x72c1f0 tipo 0x11. Com 0x76d940 tambem, +0 de 9 em diante, o placar do lado
/// nao abaixo do outro, sorteio(5) + 2 < nota 1 e ainda sem noticia 0x34 na partida: com a nota
/// 0x11 acima de sorteio(9) x 0,1 + media + 0,5, 0x744240 (0x34). Depois copia para a ficha +0x45,
/// +0xb, +9, +0xd (de -4, +3, +5, +7). Dois clubes da mesma nacao (+0x53) e -8 na linha: com 0x7c1e30
/// nao zero, o clube anterior (+0x1a) de outra nacao (+0x76), +0xb da ficha acima de 5750, +0x57
/// acima de 10, +0x59 abaixo de 15, +0x5c ou +0x5d abaixo de 10 e sorteio(3) == 0 (do proprio clube
/// [0x9cf300]) ou sorteio(10) == 0: o 0x20 no contrato (0x867560). Por fim o tecnico (pessoa em
/// +0x184): a ficha dele (+0x69) recebe +0x188/+0x18a/+0x18c em +0xa/+8/+0xc, e o auxiliar (+0xd3)
/// e os 5 de +0x19f ganham um quarto da diferenca (entre 1 e 10000); +0x80 do clube = +0x18e.
pub(crate) unsafe fn post_match_players_with(partida: usize, t: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let special: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    let average_7c1e30: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let ch = core::cell::RefCell::new(call);
    let register = |r: usize, pa: usize, t: usize| (ch.borrow_mut())(0x5afe00, r, [pa as u32, t as u32, 0, 0, 0]);
    let news = |n: usize, p: usize, c: usize, tp: u32, pa: usize, z: u32| (ch.borrow_mut())(0x72c1f0, n, [p as u32, c as u32, tp, pa as u32, z]);
    let news_34 = |n: usize, p: usize, c: usize, tp: u32, pa: usize| (ch.borrow_mut())(0x744240, n, [p as u32, c as u32, tp, pa as u32, 0]);
    let por_20 = |c: usize, f: u32| (ch.borrow_mut())(0x867560, c, [f, 0, 0, 0, 0]);
    // simplification: 0x7c1e30 (returns floating point) also reaches 0x68d0d0 in 3 levels, stays direct
    let contract = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x4cf5f0, n, [p as u32, c as u32, 0, 0, 0]) as usize;
    let test_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let ub = |a: usize| *(a as *const u8) as i32;
    let club_of = |id: i32| (g(G_CLUB_TABLE) as isize + id as isize * 0x245) as usize;
    let average = || {
        let b = g(G_PLAYER_STATS);
        let d = core::ptr::read_unaligned((b + 0x18) as *const f64);
        if d != 0.0 { core::ptr::read_unaligned((b + 0x10) as *const f64) / d } else { 6.8 }
    };
    let esp = special(g(G_NATIONAL_TEAMS), read(partida + 0x14) as u32);
    if *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 && g(G_FOG_OF_WAR) != 0 {
        register(g(G_FOG_OF_WAR), partida, t);
    }
    let comp = if read(partida) > -1 { g(g(g(G_COMP_OBJECT_TABLE) + read(partida) as usize * 4) + 8) } else { 0 };
    let mut ja_34 = false;
    for lado in 0..2usize {
        let club = club_of(read(t + 4 * lado));
        let ts = t + lado * 0x625;
        let other = t + (lado == 0) as usize * 0x625;
        let scoreboard = |r: usize| ub(r + 0x120) + ub(r + 0x11f);
        for k in 0..20usize {
            let row = ts + 0x1c5 + k * 0x49;
            let id = read(row - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            if g(p + staff::PLAYER) == 0 || (p != 0 && read(p) >= read(G_STAFF_COUNT)) {
                continue;
            }
            if comp != 0 && read(partida + 0x14) == read(club + club::DIVISION) && *((t + 0x4c) as *const u8) == 0 {
                let c = contract(g(G_CONTRACT_MANAGER), p, club);
                let nota1 = exe::comp_stats_get_player_stat(comp, g(p + staff::PLAYER), 1);
                let nota11 = exe::comp_stats_get_player_stat(comp, g(p + staff::PLAYER), 0x11);
                if c != 0 && matches!(*((c + contract::SQUAD_STATUS) as *const u8) & 0xf0, 0x10 | 0x20 | 0x30) {
                    let x = exe::shortlist_manager_find_club_shortlist_entry(g(g(G_TRANSFER_MANAGER) + 0x856), club, p, 0, 0);
                    let passes = test_600a50(club, 0, 1, 0, 0) != 0 || (x != 0 && read(x + 0xf) == read(p));
                    let d = w(ts + 0x18e) as f64;
                    if passes
                        && w(row + 3) > 0x1388
                        && (d * 1.1 < w(row + 3) as f64 || d * 0.9 < w(row + 7) as f64 || d * 0.024 < w(g(p + staff::PLAYER) + 5) as f64)
                        && *(row as *const u8) <= 6
                        && scoreboard(ts) <= scoreboard(other)
                        && ((random(10) + 5) as f64) < nota1
                        && (*((g(p + staff::PLAYER) + 0x45) as *const i8) as i32) < random(10)
                        && average() + 0.2 > nota11
                        && average() - 0.2 < nota11
                    {
                        crate::comparison::branch("jogadores noticia 0x11");
                        news(g(G_MEDIA), p, club, 0x11, partida, 0);
                    }
                }
            }
            if esp != 0 && comp != 0 && *((t + 0x4c) as *const u8) == 0 {
                let nota1 = exe::comp_stats_get_player_stat(comp, g(p + staff::PLAYER), 1);
                if *(row as *const u8) >= 9 && scoreboard(ts) >= scoreboard(other) && ((random(5) + 2) as f64) < nota1 && !ja_34 {
                    let m = average();
                    let nota11 = exe::comp_stats_get_player_stat(comp, g(p + staff::PLAYER), 0x11);
                    let r = random(9);
                    if r as f64 * 0.1 + m + 0.5 < nota11 {
                        crate::comparison::branch("jogadores noticia 0x34");
                        news_34(g(G_MEDIA), p, club, 0x34, partida);
                        ja_34 = true;
                    }
                }
            }
            let f = g(p + staff::PLAYER);
            *((f + 0x45) as *mut u8) = *((row - 4) as *const u8);
            core::ptr::write_unaligned((f + 0xb) as *mut i16, w(row + 3));
            core::ptr::write_unaligned((g(p + staff::PLAYER) + 9) as *mut i16, w(row + 5));
            core::ptr::write_unaligned((g(p + staff::PLAYER) + 0xd) as *mut i16, w(row + 7));
            if read(club_of(read(t)) + 0x53) == read(club_of(read(t + 4)) + 0x53) && *((row - 8) as *const u8) != 0 {
                let v = average_7c1e30(g(G_PLAYER_STATS), g(p + staff::PLAYER), 6, 0xb, 6.5);
                let previous = g(p + staff::NATION);
                let nation = g(club + club::NATION);
                let f = g(p + staff::PLAYER);
                if !(v == 0.0 || v.is_nan())
                    && previous != 0
                    && nation != 0
                    && *((previous + nation::ACTUAL_REGION) as *const u8) != *((nation + nation::ACTUAL_REGION) as *const u8)
                    && f != 0
                    && w(f + 0xb) > 0x1676
                    && byte(p, staff::AMBITION) > 0xa
                    && byte(p, staff::LOYALTY) < 0xf
                    && (byte(p, staff::SPORTSMANSHIP) < 0xa || byte(p, staff::TEMPERAMENT) < 0xa)
                    && ((read(previous) == read(KEY_NATION_FRANCE) && random(3) == 0) || random(10) == 0)
                {
                    let c = contract(g(G_CONTRACT_MANAGER), p, club);
                    if c != 0 && exe::staff_contract_get_flags(c) & 0x20 == 0 {
                        crate::comparison::branch("jogadores marca 0x20");
                        por_20(c, 0x20);
                    }
                }
            }
        }
        // the manager and the assistants
        let id = read(ts + 0x184);
        if id > -1 {
            crate::comparison::branch("jogadores tecnico");
            let manager = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let (mut da, mut db, mut dc) = (0i16, 0i16, 0i16);
            if manager != 0 && g(manager + staff::NON_PLAYER) != 0 {
                let f = g(manager + staff::NON_PLAYER);
                da = w(ts + 0x188).wrapping_sub(w(f + 0xa));
                core::ptr::write_unaligned((f + 0xa) as *mut i16, w(f + 0xa).wrapping_add(da));
                let f = g(manager + staff::NON_PLAYER);
                db = w(ts + 0x18a).wrapping_sub(w(f + 8));
                core::ptr::write_unaligned((f + 8) as *mut i16, w(f + 8).wrapping_add(db));
                let f = g(manager + staff::NON_PLAYER);
                dc = w(ts + 0x18c).wrapping_sub(w(f + 0xc));
                core::ptr::write_unaligned((f + 0xc) as *mut i16, w(f + 0xc).wrapping_add(dc));
            }
            for k in -1i32..5 {
                let aux = if k == -1 { g(club + club::ASSISTANT_MANAGER) } else { g(club + club::COACHES + k as usize * 4) };
                if aux == 0 || aux == manager || g(aux + staff::NON_PLAYER) == 0 {
                    continue;
                }
                for (o, dv) in [(0xa, da), (8, db), (0xc, dc)] {
                    let f = g(aux + staff::NON_PLAYER);
                    let v = w(f + o).wrapping_add(ftol(dv as f64 * 0.25) as i16);
                    core::ptr::write_unaligned((f + o) as *mut i16, v);
                    let f = g(aux + staff::NON_PLAYER);
                    core::ptr::write_unaligned((f + o) as *mut i16, w(f + o).clamp(1, 10000));
                }
            }
        }
        core::ptr::write_unaligned((club + club::REPUTATION) as *mut i16, w(ts + 0x18e));
    }
}

pub(crate) static mut ORIGINAIS: [usize; N] = [0; N];
pub(crate) static SCOREBOARD: Scoreboard = Scoreboard::new("motor");

/// Returns original routine i (the bytes saved before the detour) as a function pointer of type F.
pub(crate) unsafe fn original_fn<F: Copy>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINAIS[i])
}

// --- gravar e reproduzir: 0x6f5c30 (3,9 KB, sorteia e altera jogadores) chamada por 0x6cd330 ---
//
// In comparison mode the original runs and each of its calls to 0x6f5c30 goes onto the tape (port below, only
// during this comparison); the Rust copy receives the recorded return value and effect (0x6f5c30 changes the
// next players in the list, which the original skips). In the self-test (synthetic players) the port
// responds synthetically, without running the real routine.

pub(crate) static mut MATCH_PLAYER_RESPOND_TRAMPOLINE: usize = 0;
pub(crate) static RECORD_6F5C30: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static SYNTHETIC: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// 0x6f5c30: sub esp, 0x218
pub(crate) const PROLOGUE_6F5C30: Prologue = Prologue { address: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] };

/// Hook entry for `exe::match_player_try_intercept_ball`: synthetic result in self-test, tape recording, or the comparer around the original.
pub(crate) unsafe extern "thiscall" fn respond_hook(p: usize, a: u32, b: u32, c: u32, d: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let rodar = || {
        if SYNTHETIC.load(Relaxed) {
            (p as u32 ^ a ^ b.wrapping_mul(3) ^ c.wrapping_mul(5) ^ d.wrapping_mul(7)) >> 4 & 1
        } else {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(MATCH_PLAYER_RESPOND_TRAMPOLINE);
            f(p, a, b, c, d) as u32
        }
    };
    if SYNTHETIC.load(Relaxed) {
        if RECORD_6F5C30.load(Relaxed) && is_recording() {
            return call_recording(FN_MATCH_PLAYER_TRY_INTERCEPT_BALL, p, [a, b, c, d, 0], rodar) as u8;
        }
        return rodar() as u8;
    }
    // the comparison of 0x6f5c30 (128th batch): its hook is this port. It is only called by
    // 0x6cd330 (`search_on_side`), whose comparison records it as a unit: the comparison runs inside
    // of the unit (nested, with the outer tape saved)
    let comparar = || compare_respond(p, a, b, c, d, &rodar) as u32;
    if RECORD_6F5C30.load(Relaxed) && is_recording() {
        return call_recording(FN_MATCH_PLAYER_TRY_INTERCEPT_BALL, p, [a, b, c, d, 0], comparar) as u8;
    }
    if is_recording_top_level() {
        return rodar() as u8;
    }
    comparar() as u8
}

compare_reader!(compare_ler_8ea7, ler_8ea7, IDX_LER_8EA7);
compare_reader!(compare_ler_8ea8, ler_8ea8, IDX_LER_8EA8);
compare_reader!(compare_ler_8ea9, ler_8ea9, IDX_LER_8EA9);
compare_reader!(compare_ler_8eaa, ler_8eaa, IDX_LER_8EAA);
compare_reader!(compare_ler_8eb2, ler_8eb2, IDX_LER_8EB2);
compare_recorder!(compare_gravar_8ea9, gravar_8ea9, IDX_GRAVAR_8EA9, 0x8ea9, u8);
compare_recorder!(compare_gravar_8ea7, gravar_8ea7, IDX_GRAVAR_8EA7, 0x8ea7, u8);
compare_recorder!(compare_gravar_8ea8, gravar_8ea8, IDX_GRAVAR_8EA8, 0x8ea8, u8);
compare_recorder!(compare_gravar_8eaa, gravar_8eaa, IDX_GRAVAR_8EAA, 0x8eaa, u8);
compare_recorder!(compare_write_8ed8, write_8ed8, IDX_WRITE_8ED8, 0x8ed8, u16);

compare_split!(compare_split_a, split_a, IDX_SPLIT_A);
compare_split!(compare_split_b, split_b, IDX_SPLIT_B);
compare_split!(compare_split_c, split_c, IDX_SPLIT_C);

compare_value!(compare_direction, direction, IDX_DIRECTION, () -> u16);
compare_value!(compare_x_fora, x_off_centre, IDX_X_OFF_CENTRE, () -> u8);
compare_value!(compare_is_the_stored, is_the_stored, IDX_IS_THE_STORED, () -> u8);
compare_value!(compare_side_of_move, side_of_move, IDX_SIDE_OF_MOVE, () -> u8);
compare_value!(compare_player_in_block, player_in_block, IDX_PLAYER_IN_BLOCK, (l: i8, k: i8) -> usize);
compare_value!(compare_table_a_1a, table_a_1a, IDX_TABLE_A_1A, (j: usize) -> u16);
compare_value!(compare_marked_moment, marked_moment, IDX_MARKED_MOMENT, () -> u32);
compare_value!(compare_side_state, side_state, IDX_SIDE_STATE, () -> u8);
compare_value!(compare_opponent_state, opponent_state, IDX_OPPONENT_STATE, () -> u8);
compare_value!(compare_near_goal, near_goal, IDX_NEAR_GOAL, () -> u8);
compare_value!(compare_behind_on_aggregate, behind_on_aggregate, IDX_BEHIND_ON_AGGREGATE, (lado: i8) -> u8);
compare_value!(compare_free_teammate, free_teammate, IDX_FREE_TEAMMATE, (p: usize) -> usize);

