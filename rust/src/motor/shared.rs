use super::*;

/// Port of 0x6cd490 with commentary injected: releases the ball and, if out of play, sets the restart (throw-in/goal
/// kick/corner) and side, narrates and returns 1; 0 in play.
///
/// [pt] { 0x6cd490 } (match_eng.cpp) com o comentario injetado: com o lance ja parado (0x8eb2), 1.
/// Solta a bola do dono e, com a bola fora do campo, marca a reposicao (0x8eb2: 2 lateral, 3 ou
/// 4 conforme a linha de fundo e o lado que tocou por ultimo; 0x8eae: o lado que repoe), narra
/// (0x1f76/0x1f75/0x1f77) e devolve 1. Bola em campo, 0.
pub(crate) unsafe fn ball_out_with(this: usize, side_b: u8, call: AddEventCall) -> u8 {
    if byte(this, 0x8eb2) != 0 {
        return 1;
    }
    if byte(this, 0x8eae) != -1 {
        *((this + 0x8eae) as *mut i8) = -1;
    }
    let owner = *((this + 0xf582) as *const usize);
    if owner != 0 {
        let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
        soltar(owner);
    }
    let (x, y) = (byte(this, 0x8ea7), byte(this, 0x8ea8));
    let commentator = *((this + 0x8f7a) as *const usize);
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
    *((this + 0x8eb2) as *mut u8) = parada;
    *((this + 0x8eae) as *mut u8) = quem;
    call(commentator, [codigo, this as u32, u32::MAX, quem as u32, u32::MAX, u32::MAX, 0]);
    1
}

/// Port of 0x70dfd0 with external calls injected: player j contests a play against the opponent's strength; returns 1
/// if resolved in his favour.
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
pub(crate) unsafe fn play_with(j: usize, strength: i16, altura: f32, estilo: i8, strong: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let table: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let goalkeeper_of: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let decidir: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_FOUL);
    let register: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let punir: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_BOOKING);
    let pick_restart_taker: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let special: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let to_team: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let m = *((j + 0x19e) as *const usize);
    // o setimo argumento da narracao vai no alto do codigo (ver `dispatch_tackle_calls`)
    let narrar7 = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        call(FN_MATCH_EVENTS_ADD_EVENT, *((m + 0x8f7a) as *const usize), [code | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, a3: u32, a4: u32, a5: u32| narrar7(call, code, a3, a4, a5, 0);
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let in_the_box = || area_de(j, byte(j, 0x27) == 1);
    let espirra = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, espalhar: i32, chutador: usize| {
        call(FN_CM3_MATCH_ENGINE_MOVE_BALL_TO_CELL, m, [ub(j, 0x102), ub(j, 0x103), (espalhar as u8 as u32) | ub(j, 0x27) << 8, 0, chutador as u32]);
    };
    if goalkeeper_of(m, byte(j, 0x27)) == j || *((m + 0xf582) as *const usize) == j {
        return 0;
    }
    if byte(m, 0x8eae) != byte(j, 0x27) && byte(j, 0x10d) == 0 {
        return 0;
    }
    let mut ataque: i32 = match byte(m, 0x8ea9) {
        0 => sortear(300),
        1 => sortear(400),
        2 => sortear(500),
        3 => sortear(1000),
        4 => sortear(0x708),
        _ => {
            // simplification: a original mostra a caixa "Error" (versao, arquivo, linha 0x4755); +0x8ea9
            // fora de 0..4 nao acontece no jogo
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            return 0;
        }
    };
    let st = altura as f64 + 1.0;
    if st > 0.0 {
        ataque = match_pl_ftol(ataque as i16 as f64 / st);
    }
    if byte(m, 0x8eab) == byte(j, 0x27) {
        if sortear(ataque as i16 as i32 + 100) != 0 {
            return 0;
        }
        crate::comparacao::ramo("play bola do proprio lado");
        narrate(call, 0x1f5e, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        let r = sortear(byte(m, 0x8eaa) as i32);
        espirra(call, r.wrapping_add(1), j);
        return 1;
    }
    // o lance e o atributo
    let em_area = in_the_box();
    let escolher = || -> (i16, i16) {
        if byte(m, 0x8ea9) == 4 {
            if table(m, j) == 1 { (*w(j, 0x11c), 1) } else { (*w(j, 0x144), 0x15) }
        } else if table(m, j) != 1 {
            (*w(j, 0x162), 0x24)
        } else if byte(m, 0x8ea9) == 3 && sortear(4) != 0 {
            (*w(j, 0x11c), 1)
        } else {
            (*w(j, 0x16c), 0x29)
        }
    };
    let (mut valor, lance) = if em_area && table(m, j) == 1 {
        escolher()
    } else {
        let ficha5c = byte(*((j + 0x69) as *const usize), 0x5c) as i32;
        if sortear(em_area as i32 * 500 + 100 + ficha5c * 200) != 0 {
            escolher()
        } else {
            crate::comparacao::ramo("play cabecada");
            ((*w(j, 0x164)).wrapping_shl(1), 0x25)
        }
    };
    if strong != 0 {
        crate::comparacao::ramo("play forte");
        valor = match_pl_ftol(valor as f64 * 0.85) as i16;
    }
    let v = sortear(valor as i32);
    let rating = (v - strength as i32 / 3 - ataque) as i16;
    if rating < -100 {
        crate::comparacao::ramo("play nota muito baixa");
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2) as u8);
        return 0;
    }
    if rating < 0 {
        return 0;
    }
    let foul_strong = strength as i32 > sortear(0x96) + 0xc8;
    let (mut cartao, mut grav) = (0u8, 0u8);
    let mut t = decidir(m, j, lance as u16, &mut cartao, &mut grav);
    if t != 0 {
        if sortear(5) == 0 && !foul_strong {
            crate::comparacao::ramo("play falta registrada");
            register(m, lance as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);
        } else {
            // a falta com punicao
            crate::comparacao::ramo("play falta punida");
            let mut weight = 0f32;
            punir(m, j, &mut t, cartao as i8, grav as i8, foul_strong as u8, &mut weight);
            narrate(call, 0x1fe8, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            let colour: i32 = match t {
                2 => 0x1ff9,
                3 => 0x1ff7,
                4 => 0x1ff8,
                6 => 0x1ff5,
                _ => {
                    if grav as i8 as i32 > sortear(7) + 2 { 0x210a } else { -1 }
                }
            };
            let som: u32;
            if t == 7 {
                som = 0x1f79;
                *b(m, 0x8eb2) = 7;
            } else if in_the_box() {
                if colour == -1 {
                    match sortear(4) {
                        0 => narrate(call, 0x20e9, u32::MAX, zero(j), u32::MAX),
                        1 => narrate(call, 0x20ec, u32::MAX, zero(j), u32::MAX),
                        _ => {}
                    }
                }
                crate::comparacao::ramo("play penalty");
                som = 0x1f7a;
                *b(m, 0x8eb2) = 5;
                *((m + 0x478a) as *mut usize) = 0;
                *w(j, 0x35) = (*w(j, 0x35)).wrapping_sub(500);
            } else {
                som = 0x1f79;
                *b(m, 0x8eb2) = 6;
            }
            *b(m, 0x8ea7) = *b(j, 0x102);
            *b(m, 0x8ea8) = *b(j, 0x103);
            *b(m, 0x8eae) = zero(j) as u8;
            *((m + 0xf582) as *mut u32) = 0;
            if som == 0x1f7a {
                narrate(call, 0x1f7a, u32::MAX, zero(j), u32::MAX);
            }
            let expulso = matches!(t, 3..=6);
            if colour > -1 {
                if sortear(4) == 0 {
                    let a = sortear(grav as i8 as i32);
                    if a > sortear(0x14) {
                        let partida = *((m + 0x4792) as *const usize);
                        let clube = *((partida + if byte(j, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize);
                        let k = *((clube + club::NATION) as *const usize);
                        if k != 0 && byte(k, 0x77) == 8 {
                            narrate(call, 0x20f5, 0, 0, u32::MAX);
                        }
                    }
                    narrate(call, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                }
                narrate(call, colour as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                if expulso {
                    player_left_pitch_with(m, j, 0, call);
                }
            }
            if som != 0x1f7a {
                let c = pick_restart_taker(m, (byte(j, 0x27) == 0) as i8, byte(m, 0x8eb2), 0);
                if c == 0 {
                    narrate(call, som, u32::MAX, zero(j), u32::MAX);
                } else {
                    if t == 7 {
                        narrate(call, 0x21e2, ub(c, 0x2e), zero(j), u32::MAX);
                    } else {
                        narrate(call, som, u32::MAX, zero(j), u32::MAX);
                    }
                    if som == 0x1f79 {
                        let far = (byte(c, 0x27) == 1 && byte(c, 0x103) < 4) || (byte(c, 0x27) == 0 && byte(c, 0x103) > 7);
                        if t == 7 {
                            if far && !area_de(c, byte(j, 0x27) == 1) {
                                narrate(call, 0x21e1, ub(c, 0x2e), zero(j), u32::MAX);
                            }
                        } else if far {
                            narrate(call, if (2..=6).contains(&byte(j, 0x102)) { 0x21e0 } else { 0x21df }, ub(c, 0x2e), zero(j), u32::MAX);
                        }
                    }
                }
            }
            if special(lance as i8) == 0 {
                if sortear(9) == 0 {
                    narrate(call, 0x20df, u32::MAX, ub(j, 0x27), u32::MAX);
                    if sortear(3) == 0 {
                        narrate(call, 0x20e0, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                    if (expulso || som == 0x1f7a) && sortear(2) == 0 {
                        narrate(call, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                }
                to_team(m, j, weight);
                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, [0; 5]);
            }
            return 1;
        }
    }
    let meia_lua = byte(j, 0x102) == 4 && (byte(j, 0x103) == 0 || byte(j, 0x103) == 11);
    if rating >= 400 {
        // venceu
        crate::comparacao::ramo("play venceu");
        *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1e);
        *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);
        if meia_lua {
            narrate(call, 0x1ffe, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        } else if strong != 0 {
            narrar7(call, 0x21d6, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        } else if estilo == 5 {
            narrar7(call, 0x1f66, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 5);
        } else {
            narrar7(call, 0x1f65, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        }
        call(FN_MATCH_PLAYER_TAKE_POSSESSION, j, [0; 5]);
        *w(j, 0x198) = lance;
        return 1;
    }
    if goalkeeper_of(m, byte(j, 0x27)) == j {
        crate::comparacao::ramo("play goleiro");
        narrate(call, 0x1fe7, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
    } else if meia_lua && rating > 200 {
        crate::comparacao::ramo("play meia-lua");
        narrate(call, 0x1ffd, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        espirra(call, sortear(3), 0);
    } else {
        crate::comparacao::ramo("play perdeu");
        if strong != 0 {
            narrar7(call, 0x21d8, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        } else if estilo == 5 {
            narrar7(call, 0x1f85, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 5);
        } else {
            narrar7(call, 0x1f84, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        }
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        espirra(call, sortear(3), j);
    }
    *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x14);
    *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2).wrapping_add(1) as u8);
    *w(j, 0x198) = lance;
    1
}

pub(crate) use crate::conteiner::{abrir_no_conteiner, buscar_no_conteiner, posicao_no_conteiner};

// simplification: o conteiner abre arquivos, aloca 650 KB por item e le do disco; sem comparacao propria,
// validado pela carreira (Rust x original; o banco e os historicos passam por aqui o tempo todo) e
// pelo save de enchimento acima de 2 GB.



/// Port of 0x6b6cc0 with calls injected: post-match updates for both sides' players (news, contract flags, manager
/// ratings).
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
    let rating: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(FN_COMP_STATS_GET_PLAYER_STAT);
    let teste_849ce0: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let average_7c1e30: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let marks: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let ch = core::cell::RefCell::new(call);
    let register = |r: usize, pa: usize, t: usize| (ch.borrow_mut())(0x5afe00, r, [pa as u32, t as u32, 0, 0, 0]);
    let noticia = |n: usize, p: usize, c: usize, tp: u32, pa: usize, z: u32| (ch.borrow_mut())(0x72c1f0, n, [p as u32, c as u32, tp, pa as u32, z]);
    let news_34 = |n: usize, p: usize, c: usize, tp: u32, pa: usize| (ch.borrow_mut())(0x744240, n, [p as u32, c as u32, tp, pa as u32, 0]);
    let por_20 = |c: usize, f: u32| (ch.borrow_mut())(0x867560, c, [f, 0, 0, 0, 0]);
    // simplification: a 0x7c1e30 (devolve ponto flutuante) tambem alcanca a 0x68d0d0 em 3 niveis, fica direta
    let contrato = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x4cf5f0, n, [p as u32, c as u32, 0, 0, 0]) as usize;
    let teste_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let ub = |a: usize| *(a as *const u8) as i32;
    let club_of = |id: i32| (g(0xae23b4) as isize + id as isize * 0x245) as usize;
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
        let clube = club_of(read(t + 4 * lado));
        let ts = t + lado * 0x625;
        let other = t + (lado == 0) as usize * 0x625;
        let placar = |r: usize| ub(r + 0x120) + ub(r + 0x11f);
        for k in 0..20usize {
            let row = ts + 0x1c5 + k * 0x49;
            let id = read(row - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            if g(p + 0x61) == 0 || (p != 0 && read(p) >= read(0xae2364)) {
                continue;
            }
            if comp != 0 && read(partida + 0x14) == read(clube + club::DIVISION) && *((t + 0x4c) as *const u8) == 0 {
                let c = contrato(g(G_CONTRACT_MANAGER), p, clube);
                let nota1 = rating(comp, g(p + 0x61), 1);
                let nota11 = rating(comp, g(p + 0x61), 0x11);
                if c != 0 && matches!(*((c + 0x4f) as *const u8) & 0xf0, 0x10 | 0x20 | 0x30) {
                    let x = teste_849ce0(g(g(G_TRANSFER_MANAGER) + 0x856), clube, p, 0, 0);
                    let passa = teste_600a50(clube, 0, 1, 0, 0) != 0 || (x != 0 && read(x + 0xf) == read(p));
                    let d = w(ts + 0x18e) as f64;
                    if passa
                        && w(row + 3) > 0x1388
                        && (d * 1.1 < w(row + 3) as f64 || d * 0.9 < w(row + 7) as f64 || d * 0.024 < w(g(p + 0x61) + 5) as f64)
                        && *(row as *const u8) <= 6
                        && placar(ts) <= placar(other)
                        && ((sortear(10) + 5) as f64) < nota1
                        && (*((g(p + 0x61) + 0x45) as *const i8) as i32) < sortear(10)
                        && average() + 0.2 > nota11
                        && average() - 0.2 < nota11
                    {
                        crate::comparacao::ramo("jogadores noticia 0x11");
                        noticia(g(G_MEDIA), p, clube, 0x11, partida, 0);
                    }
                }
            }
            if esp != 0 && comp != 0 && *((t + 0x4c) as *const u8) == 0 {
                let nota1 = rating(comp, g(p + 0x61), 1);
                if *(row as *const u8) >= 9 && placar(ts) >= placar(other) && ((sortear(5) + 2) as f64) < nota1 && !ja_34 {
                    let m = average();
                    let nota11 = rating(comp, g(p + 0x61), 0x11);
                    let r = sortear(9);
                    if r as f64 * 0.1 + m + 0.5 < nota11 {
                        crate::comparacao::ramo("jogadores noticia 0x34");
                        news_34(g(G_MEDIA), p, clube, 0x34, partida);
                        ja_34 = true;
                    }
                }
            }
            let f = g(p + 0x61);
            *((f + 0x45) as *mut u8) = *((row - 4) as *const u8);
            core::ptr::write_unaligned((f + 0xb) as *mut i16, w(row + 3));
            core::ptr::write_unaligned((g(p + 0x61) + 9) as *mut i16, w(row + 5));
            core::ptr::write_unaligned((g(p + 0x61) + 0xd) as *mut i16, w(row + 7));
            if read(club_of(read(t)) + 0x53) == read(club_of(read(t + 4)) + 0x53) && *((row - 8) as *const u8) != 0 {
                let v = average_7c1e30(g(G_PLAYER_STATS), g(p + 0x61), 6, 0xb, 6.5);
                let previous = g(p + 0x1a);
                let nacao = g(clube + club::NATION);
                let f = g(p + 0x61);
                if !(v == 0.0 || v.is_nan())
                    && previous != 0
                    && nacao != 0
                    && *((previous + 0x76) as *const u8) != *((nacao + nation::ACTUAL_REGION) as *const u8)
                    && f != 0
                    && w(f + 0xb) > 0x1676
                    && byte(p, 0x57) > 0xa
                    && byte(p, 0x59) < 0xf
                    && (byte(p, 0x5c) < 0xa || byte(p, 0x5d) < 0xa)
                    && ((read(previous) == read(KEY_NATION_FRANCE) && sortear(3) == 0) || sortear(10) == 0)
                {
                    let c = contrato(g(G_CONTRACT_MANAGER), p, clube);
                    if c != 0 && marks(c) & 0x20 == 0 {
                        crate::comparacao::ramo("jogadores marca 0x20");
                        por_20(c, 0x20);
                    }
                }
            }
        }
        // o tecnico e os auxiliares
        let id = read(ts + 0x184);
        if id > -1 {
            crate::comparacao::ramo("jogadores tecnico");
            let manager = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let (mut da, mut db, mut dc) = (0i16, 0i16, 0i16);
            if manager != 0 && g(manager + 0x69) != 0 {
                let f = g(manager + 0x69);
                da = w(ts + 0x188).wrapping_sub(w(f + 0xa));
                core::ptr::write_unaligned((f + 0xa) as *mut i16, w(f + 0xa).wrapping_add(da));
                let f = g(manager + 0x69);
                db = w(ts + 0x18a).wrapping_sub(w(f + 8));
                core::ptr::write_unaligned((f + 8) as *mut i16, w(f + 8).wrapping_add(db));
                let f = g(manager + 0x69);
                dc = w(ts + 0x18c).wrapping_sub(w(f + 0xc));
                core::ptr::write_unaligned((f + 0xc) as *mut i16, w(f + 0xc).wrapping_add(dc));
            }
            for k in -1i32..5 {
                let aux = if k == -1 { g(clube + club::ASSISTANT_MANAGER) } else { g(clube + club::COACHES + k as usize * 4) };
                if aux == 0 || aux == manager || g(aux + 0x69) == 0 {
                    continue;
                }
                for (o, dv) in [(0xa, da), (8, db), (0xc, dc)] {
                    let f = g(aux + 0x69);
                    let v = w(f + o).wrapping_add(match_pl_ftol(dv as f64 * 0.25) as i16);
                    core::ptr::write_unaligned((f + o) as *mut i16, v);
                    let f = g(aux + 0x69);
                    core::ptr::write_unaligned((f + o) as *mut i16, w(f + o).clamp(1, 10000));
                }
            }
        }
        core::ptr::write_unaligned((clube + club::REPUTATION) as *mut i16, w(ts + 0x18e));
    }
}

pub(crate) static mut ORIGINAIS: [usize; N] = [0; N];
pub(crate) static PLACAR: Placar = Placar::novo("motor");

/// Returns original routine i (the bytes saved before the detour) as a function pointer of type F.
pub(crate) unsafe fn original_fn<F: Copy>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINAIS[i])
}





// --- gravar e reproduzir: 0x6f5c30 (3,9 KB, sorteia e altera jogadores) chamada por 0x6cd330 ---
//
// Na comparacao a original roda e cada chamada dela a 0x6f5c30 entra na fita (porta abaixo, so
// durante esta comparacao); a copia Rust recebe a resposta e o efeito gravados (a 0x6f5c30 altera os
// proximos jogadores da lista, que a original pula). No autoteste (jogadores sinteticos) a porta
// responde de forma sintetica, sem rodar a rotina real.

pub(crate) static mut MATCH_PLAYER_RESPOND_TRAMPOLINE: usize = 0;
pub(crate) static RECORD_6F5C30: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static SYNTHETIC: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// 0x6f5c30: sub esp, 0x218
pub(crate) const PROLOGUE_6F5C30: Prologo = Prologo { endereco: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] };

/// Hook entry for 0x6f5c30: synthetic result in self-test, tape recording, or the comparer around the original.
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
    // a comparacao da 0x6f5c30 (128a leva): o desvio dela e esta porta. Ela so e chamada pela
    // 0x6cd330 (`search_on_side`), cuja comparacao a grava como unidade: a comparacao roda dentro
    // da unidade (aninhada, com a fita de fora guardada)
    let comparar = || comparar_respond(p, a, b, c, d, &rodar) as u32;
    if RECORD_6F5C30.load(Relaxed) && is_recording() {
        return call_recording(FN_MATCH_PLAYER_TRY_INTERCEPT_BALL, p, [a, b, c, d, 0], comparar) as u8;
    }
    if is_recording_top_level() {
        return rodar() as u8;
    }
    comparar() as u8
}









comparar_leitor!(comparar_ler_8ea7, ler_8ea7, IDX_LER_8EA7);
comparar_leitor!(comparar_ler_8ea8, ler_8ea8, IDX_LER_8EA8);
comparar_leitor!(comparar_ler_8ea9, ler_8ea9, IDX_LER_8EA9);
comparar_leitor!(comparar_ler_8eaa, ler_8eaa, IDX_LER_8EAA);
comparar_leitor!(comparar_ler_8eb2, ler_8eb2, IDX_LER_8EB2);
comparar_gravador!(comparar_gravar_8ea9, gravar_8ea9, IDX_GRAVAR_8EA9, 0x8ea9, u8);
comparar_gravador!(comparar_gravar_8ea7, gravar_8ea7, IDX_GRAVAR_8EA7, 0x8ea7, u8);
comparar_gravador!(comparar_gravar_8ea8, gravar_8ea8, IDX_GRAVAR_8EA8, 0x8ea8, u8);
comparar_gravador!(comparar_gravar_8eaa, gravar_8eaa, IDX_GRAVAR_8EAA, 0x8eaa, u8);
comparar_gravador!(comparar_write_8ed8, write_8ed8, IDX_WRITE_8ED8, 0x8ed8, u16);


comparar_repartir!(comparar_split_a, split_a, IDX_SPLIT_A);
comparar_repartir!(comparar_split_b, split_b, IDX_SPLIT_B);
comparar_repartir!(comparar_split_c, split_c, IDX_SPLIT_C);

comparar_valor!(comparar_direction, direction, IDX_DIRECTION, () -> u16);
comparar_valor!(comparar_x_fora, x_off_centre, IDX_X_OFF_CENTRE, () -> u8);
comparar_valor!(comparar_is_the_stored, is_the_stored, IDX_IS_THE_STORED, () -> u8);
comparar_valor!(comparar_side_of_move, side_of_move, IDX_SIDE_OF_MOVE, () -> u8);
comparar_valor!(comparar_player_in_block, player_in_block, IDX_PLAYER_IN_BLOCK, (l: i8, k: i8) -> usize);
comparar_valor!(comparar_table_a_1a, table_a_1a, IDX_TABLE_A_1A, (j: usize) -> u16);
comparar_valor!(comparar_marked_moment, marked_moment, IDX_MARKED_MOMENT, () -> u32);
comparar_valor!(comparar_side_state, side_state, IDX_SIDE_STATE, () -> u8);
comparar_valor!(comparar_opponent_state, opponent_state, IDX_OPPONENT_STATE, () -> u8);
comparar_valor!(comparar_near_goal, near_goal, IDX_NEAR_GOAL, () -> u8);
comparar_valor!(comparar_behind_on_aggregate, behind_on_aggregate, IDX_BEHIND_ON_AGGREGATE, (lado: i8) -> u8);
comparar_valor!(comparar_free_teammate, free_teammate, IDX_FREE_TEAMMATE, (p: usize) -> usize);

