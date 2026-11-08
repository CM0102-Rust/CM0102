use super::*;

// ---------- quadragesima leva: bola fora do campo ----------

/// { 0x6cd490 } (match_eng.cpp) com o comentario injetado: com o lance ja parado (0x8eb2), 1.
/// Solta a bola do dono e, com a bola fora do campo, marca a reposicao (0x8eb2: 2 lateral, 3 ou
/// 4 conforme a linha de fundo e o lado que tocou por ultimo; 0x8eae: o lado que repoe), narra
/// (0x1f76/0x1f75/0x1f77) e devolve 1. Bola em campo, 0.
pub(crate) unsafe fn bola_fora_com(this: usize, lado_b: u8, chamar: ChamarComentario) -> u8 {
    if byte(this, 0x8eb2) != 0 {
        return 1;
    }
    if byte(this, 0x8eae) != -1 {
        *((this + 0x8eae) as *mut i8) = -1;
    }
    let dono = *((this + 0xf582) as *const usize);
    if dono != 0 {
        let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_MATCH_PLAYER_RELEASE_POSSESSION);
        soltar(dono);
    }
    let (x, y) = (byte(this, 0x8ea7), byte(this, 0x8ea8));
    let narrador = *((this + 0x8f7a) as *const usize);
    let (parada, quem, codigo): (u8, u8, u32) = if !(0..9).contains(&x) {
        let r = (lado_b == 0) as u8;
        (2, r, 0x1f76)
    } else if y < 0 {
        if lado_b == 1 { (4, 0, 0x1f77) } else { (3, 1, 0x1f75) }
    } else if y >= 0xc {
        if lado_b == 1 { (3, 0, 0x1f75) } else { (4, 1, 0x1f77) }
    } else {
        return 0;
    };
    *((this + 0x8eb2) as *mut u8) = parada;
    *((this + 0x8eae) as *mut u8) = quem;
    chamar(narrador, [codigo, this as u32, u32::MAX, quem as u32, u32::MAX, u32::MAX, 0]);
    1
}

// ---------- centesima vigesima nona leva: a jogada (0x70dfd0) ----------

/// { 0x70dfd0 } (ret 0x10, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
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
pub(crate) unsafe fn jogada_com(j: usize, forca: i16, altura: f32, estilo: i8, forte: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_PLAYER_POSITION);
    let goleiro_de: unsafe extern "thiscall" fn(usize, i8) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_GET_GOALKEEPER);
    let decidir: unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_CHECK_FOUL);
    let registrar: unsafe extern "thiscall" fn(usize, u16, u32, u32, u8, u8, u8, u8, u8, u8) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SET_PENDING_FOUL);
    let punir: unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_REFEREE_DECIDE_BOOKING);
    let cobrador: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> usize = core::mem::transmute(FN_CM3_MATCH_ENGINE_PICK_RESTART_TAKER);
    let especial: unsafe extern "stdcall" fn(i8) -> u8 = core::mem::transmute(FN_IS_UNDISPUTED_ACTION_CODE);
    let ao_time: unsafe extern "thiscall" fn(usize, usize, f32) = core::mem::transmute(FN_CM3_MATCH_ENGINE_SCALE_SIDE_ANGER_FROM_PLAYER);
    let b = |p: usize, o: usize| (p + o) as *mut u8;
    let w = |p: usize, o: usize| (p + o) as *mut i16;
    let ub = |p: usize, o: usize| *((p + o) as *const u8) as u32;
    let zero = |p: usize| (byte(p, 0x27) == 0) as u32;
    let m = *((j + 0x19e) as *const usize);
    // o setimo argumento da narracao vai no alto do codigo (ver `despachar_dividida`)
    let narrar7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        chamar(0x6d6f70, *((m + 0x8f7a) as *const usize), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| narrar7(chamar, codigo, a3, a4, a5, 0);
    let area_de = |p: usize, lado1: bool| {
        (2..=6).contains(&byte(p, 0x102)) && if lado1 { (10..=11).contains(&byte(p, 0x103)) } else { (0..=1).contains(&byte(p, 0x103)) }
    };
    let na_area = || area_de(j, byte(j, 0x27) == 1);
    let espirra = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, espalhar: i32, chutador: usize| {
        chamar(0x6cd770, m, [ub(j, 0x102), ub(j, 0x103), (espalhar as u8 as u32) | ub(j, 0x27) << 8, 0, chutador as u32]);
    };
    if goleiro_de(m, byte(j, 0x27)) == j || *((m + 0xf582) as *const usize) == j {
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
            // ponytail: a original mostra a caixa "Error" (versao, arquivo, linha 0x4755); +0x8ea9
            // fora de 0..4 nao acontece no jogo
            *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
            return 0;
        }
    };
    let st = altura as f64 + 1.0;
    if st > 0.0 {
        ataque = ftol(ataque as i16 as f64 / st);
    }
    if byte(m, 0x8eab) == byte(j, 0x27) {
        if sortear(ataque as i16 as i32 + 100) != 0 {
            return 0;
        }
        crate::comparacao::ramo("jogada bola do proprio lado");
        narrar(chamar, 0x1f5e, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        let r = sortear(byte(m, 0x8eaa) as i32);
        espirra(chamar, r.wrapping_add(1), j);
        return 1;
    }
    // o lance e o atributo
    let em_area = na_area();
    let escolher = || -> (i16, i16) {
        if byte(m, 0x8ea9) == 4 {
            if tabela(m, j) == 1 { (*w(j, 0x11c), 1) } else { (*w(j, 0x144), 0x15) }
        } else if tabela(m, j) != 1 {
            (*w(j, 0x162), 0x24)
        } else if byte(m, 0x8ea9) == 3 && sortear(4) != 0 {
            (*w(j, 0x11c), 1)
        } else {
            (*w(j, 0x16c), 0x29)
        }
    };
    let (mut valor, lance) = if em_area && tabela(m, j) == 1 {
        escolher()
    } else {
        let ficha5c = byte(*((j + 0x69) as *const usize), 0x5c) as i32;
        if sortear(em_area as i32 * 500 + 100 + ficha5c * 200) != 0 {
            escolher()
        } else {
            crate::comparacao::ramo("jogada cabecada");
            ((*w(j, 0x164)).wrapping_shl(1), 0x25)
        }
    };
    if forte != 0 {
        crate::comparacao::ramo("jogada forte");
        valor = ftol(valor as f64 * 0.85) as i16;
    }
    let v = sortear(valor as i32);
    let nota = (v - forca as i32 / 3 - ataque) as i16;
    if nota < -100 {
        crate::comparacao::ramo("jogada nota muito baixa");
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2) as u8);
        return 0;
    }
    if nota < 0 {
        return 0;
    }
    let falta_forte = forca as i32 > sortear(0x96) + 0xc8;
    let (mut cartao, mut grav) = (0u8, 0u8);
    let mut t = decidir(m, j, lance as u16, &mut cartao, &mut grav);
    if t != 0 {
        if sortear(5) == 0 && !falta_forte {
            crate::comparacao::ramo("jogada falta registrada");
            registrar(m, lance as u16, j as u32, 0, 0, t, *b(j, 0x102), *b(j, 0x103), cartao, grav);
        } else {
            // a falta com punicao
            crate::comparacao::ramo("jogada falta punida");
            let mut peso = 0f32;
            punir(m, j, &mut t, cartao as i8, grav as i8, falta_forte as u8, &mut peso);
            narrar(chamar, 0x1fe8, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
            let cor: i32 = match t {
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
            } else if na_area() {
                if cor == -1 {
                    match sortear(4) {
                        0 => narrar(chamar, 0x20e9, u32::MAX, zero(j), u32::MAX),
                        1 => narrar(chamar, 0x20ec, u32::MAX, zero(j), u32::MAX),
                        _ => {}
                    }
                }
                crate::comparacao::ramo("jogada penalti");
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
                narrar(chamar, 0x1f7a, u32::MAX, zero(j), u32::MAX);
            }
            let expulso = matches!(t, 3..=6);
            if cor > -1 {
                if sortear(4) == 0 {
                    let a = sortear(grav as i8 as i32);
                    if a > sortear(0x14) {
                        let partida = *((m + 0x4792) as *const usize);
                        let clube = *((partida + if byte(j, 0x27) == 0 { 0x20 } else { 0x1c }) as *const usize);
                        let k = *((clube + 0x53) as *const usize);
                        if k != 0 && byte(k, 0x77) == 8 {
                            narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                        }
                    }
                    narrar(chamar, 0x2109, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                }
                narrar(chamar, cor as u32, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
                if expulso {
                    jogador_saiu_com(m, j, 0, chamar);
                }
            }
            if som != 0x1f7a {
                let c = cobrador(m, (byte(j, 0x27) == 0) as i8, byte(m, 0x8eb2), 0);
                if c == 0 {
                    narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                } else {
                    if t == 7 {
                        narrar(chamar, 0x21e2, ub(c, 0x2e), zero(j), u32::MAX);
                    } else {
                        narrar(chamar, som, u32::MAX, zero(j), u32::MAX);
                    }
                    if som == 0x1f79 {
                        let longe = (byte(c, 0x27) == 1 && byte(c, 0x103) < 4) || (byte(c, 0x27) == 0 && byte(c, 0x103) > 7);
                        if t == 7 {
                            if longe && !area_de(c, byte(j, 0x27) == 1) {
                                narrar(chamar, 0x21e1, ub(c, 0x2e), zero(j), u32::MAX);
                            }
                        } else if longe {
                            narrar(chamar, if (2..=6).contains(&byte(j, 0x102)) { 0x21e0 } else { 0x21df }, ub(c, 0x2e), zero(j), u32::MAX);
                        }
                    }
                }
            }
            if especial(lance as i8) == 0 {
                if sortear(9) == 0 {
                    narrar(chamar, 0x20df, u32::MAX, ub(j, 0x27), u32::MAX);
                    if sortear(3) == 0 {
                        narrar(chamar, 0x20e0, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                    if (expulso || som == 0x1f7a) && sortear(2) == 0 {
                        narrar(chamar, 0x20d9, u32::MAX, ub(j, 0x27), u32::MAX);
                    }
                }
                ao_time(m, j, peso);
                chamar(0x716e10, j, [0; 5]);
            }
            return 1;
        }
    }
    let meia_lua = byte(j, 0x102) == 4 && (byte(j, 0x103) == 0 || byte(j, 0x103) == 11);
    if nota >= 400 {
        // venceu
        crate::comparacao::ramo("jogada venceu");
        *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x1e);
        *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
        *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(1);
        if meia_lua {
            narrar(chamar, 0x1ffe, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        } else if forte != 0 {
            narrar7(chamar, 0x21d6, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        } else if estilo == 5 {
            narrar7(chamar, 0x1f66, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 5);
        } else {
            narrar7(chamar, 0x1f65, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        }
        chamar(0x6f0630, j, [0; 5]);
        *w(j, 0x198) = lance;
        return 1;
    }
    if goleiro_de(m, byte(j, 0x27)) == j {
        crate::comparacao::ramo("jogada goleiro");
        narrar(chamar, 0x1fe7, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
    } else if meia_lua && nota > 200 {
        crate::comparacao::ramo("jogada meia-lua");
        narrar(chamar, 0x1ffd, ub(j, 0x2e), ub(j, 0x27), u32::MAX);
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        espirra(chamar, sortear(3), 0);
    } else {
        crate::comparacao::ramo("jogada perdeu");
        if forte != 0 {
            narrar7(chamar, 0x21d8, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        } else if estilo == 5 {
            narrar7(chamar, 0x1f85, ub(j, 0x2e), ub(j, 0x27), u32::MAX, 5);
        } else {
            narrar7(chamar, 0x1f84, ub(j, 0x2e), ub(j, 0x27), u32::MAX, estilo);
        }
        *((m + 0x4782) as *mut i32) = (*((m + 0x4782) as *const i32)).wrapping_sub(300);
        espirra(chamar, sortear(3), j);
    }
    *w(j, 0x35) = (*w(j, 0x35)).wrapping_add(0x14);
    *b(j, 0x12) = (*b(j, 0x12)).wrapping_add(1);
    *b(j, 0x2b) = (*b(j, 0x2b)).wrapping_add(sortear(2).wrapping_add(1) as u8);
    *w(j, 0x198) = lance;
    1
}

// ---------- centesima quinquagesima oitava leva: o save acima de 2 GB (o contêiner, em `conteiner.rs`) ----------

pub(crate) use crate::conteiner::{abrir_no_conteiner, buscar_no_conteiner, posicao_no_conteiner};

// ponytail: o conteiner abre arquivos, aloca 650 KB por item e le do disco; sem comparacao propria,
// validado pela carreira (Rust x original; o banco e os historicos passam por aqui o tempo todo) e
// pelo save de enchimento acima de 2 GB.
pub(crate) unsafe extern "C" fn comparar_abrir_no_conteiner(caminho: usize, nome: *const u8, modo: u32, a4: u32, tipo: u32) -> usize {
    orig::<unsafe extern "C" fn(usize, *const u8, u32, u32, u32) -> usize>(346)(caminho, nome, modo, a4, tipo)
}

pub(crate) unsafe extern "C" fn comparar_buscar_no_conteiner(z: usize, desloc: i32, origem: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, u32) -> i32>(347)(z, desloc, origem)
}

pub(crate) unsafe extern "C" fn comparar_posicao_no_conteiner(z: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize) -> i32>(348)(z)
}

// ---------- centesima trigesima primeira leva: os jogadores no pos-jogo (0x6b6cc0) ----------

/// { 0x6b6cc0 } (cdecl, match_day.cpp): depois da partida, para os dois lados do registro `t` da
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
pub(crate) unsafe fn jogadores_do_pos_jogo_com(partida: usize, t: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let especial: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    let nota: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(FN_COMP_STATS_GET_PLAYER_STAT);
    let teste_849ce0: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let media_7c1e30: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let ch = core::cell::RefCell::new(chamar);
    let registrar = |r: usize, pa: usize, t: usize| (ch.borrow_mut())(0x5afe00, r, [pa as u32, t as u32, 0, 0, 0]);
    let noticia = |n: usize, p: usize, c: usize, tp: u32, pa: usize, z: u32| (ch.borrow_mut())(0x72c1f0, n, [p as u32, c as u32, tp, pa as u32, z]);
    let noticia_34 = |n: usize, p: usize, c: usize, tp: u32, pa: usize| (ch.borrow_mut())(0x744240, n, [p as u32, c as u32, tp, pa as u32, 0]);
    let por_20 = |c: usize, f: u32| (ch.borrow_mut())(0x867560, c, [f, 0, 0, 0, 0]);
    // ponytail: a 0x7c1e30 (devolve ponto flutuante) tambem alcanca a 0x68d0d0 em 3 niveis, fica direta
    let contrato = |n: usize, p: usize, c: usize| (ch.borrow_mut())(0x4cf5f0, n, [p as u32, c as u32, 0, 0, 0]) as usize;
    let teste_600a50 = |c: usize, a: u32, b: u32, d: u32, e: u32| (ch.borrow_mut())(0x600a50, 0, [c as u32, a, b, d, e]) as i32;
    let g = |a: usize| *(a as *const usize);
    let ler = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let ub = |a: usize| *(a as *const u8) as i32;
    let clube_de = |id: i32| (g(0xae23b4) as isize + id as isize * 0x245) as usize;
    let media = || {
        let b = g(0xb74350);
        let d = core::ptr::read_unaligned((b + 0x18) as *const f64);
        if d != 0.0 { core::ptr::read_unaligned((b + 0x10) as *const f64) / d } else { 6.8 }
    };
    let esp = especial(g(0xb74318), ler(partida + 0x14) as u32);
    if *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 && g(0xae2a60) != 0 {
        registrar(g(0xae2a60), partida, t);
    }
    let comp = if ler(partida) > -1 { g(g(g(0xadadfc) + ler(partida) as usize * 4) + 8) } else { 0 };
    let mut ja_34 = false;
    for lado in 0..2usize {
        let clube = clube_de(ler(t + 4 * lado));
        let ts = t + lado * 0x625;
        let outro = t + (lado == 0) as usize * 0x625;
        let placar = |r: usize| ub(r + 0x120) + ub(r + 0x11f);
        for k in 0..20usize {
            let linha = ts + 0x1c5 + k * 0x49;
            let id = ler(linha - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(0xae23bc) as isize + id as isize * 0x6e) as usize;
            if g(p + 0x61) == 0 || (p != 0 && ler(p) >= ler(0xae2364)) {
                continue;
            }
            if comp != 0 && ler(partida + 0x14) == ler(clube + 0x57) && *((t + 0x4c) as *const u8) == 0 {
                let c = contrato(g(0xae1050), p, clube);
                let nota1 = nota(comp, g(p + 0x61), 1);
                let nota11 = nota(comp, g(p + 0x61), 0x11);
                if c != 0 && matches!(*((c + 0x4f) as *const u8) & 0xf0, 0x10 | 0x20 | 0x30) {
                    let x = teste_849ce0(g(g(0xde1f54) + 0x856), clube, p, 0, 0);
                    let passa = teste_600a50(clube, 0, 1, 0, 0) != 0 || (x != 0 && ler(x + 0xf) == ler(p));
                    let d = w(ts + 0x18e) as f64;
                    if passa
                        && w(linha + 3) > 0x1388
                        && (d * 1.1 < w(linha + 3) as f64 || d * 0.9 < w(linha + 7) as f64 || d * 0.024 < w(g(p + 0x61) + 5) as f64)
                        && *(linha as *const u8) <= 6
                        && placar(ts) <= placar(outro)
                        && ((sortear(10) + 5) as f64) < nota1
                        && (*((g(p + 0x61) + 0x45) as *const i8) as i32) < sortear(10)
                        && media() + 0.2 > nota11
                        && media() - 0.2 < nota11
                    {
                        crate::comparacao::ramo("jogadores noticia 0x11");
                        noticia(g(0xb6b7c8), p, clube, 0x11, partida, 0);
                    }
                }
            }
            if esp != 0 && comp != 0 && *((t + 0x4c) as *const u8) == 0 {
                let nota1 = nota(comp, g(p + 0x61), 1);
                if *(linha as *const u8) >= 9 && placar(ts) >= placar(outro) && ((sortear(5) + 2) as f64) < nota1 && !ja_34 {
                    let m = media();
                    let nota11 = nota(comp, g(p + 0x61), 0x11);
                    let r = sortear(9);
                    if r as f64 * 0.1 + m + 0.5 < nota11 {
                        crate::comparacao::ramo("jogadores noticia 0x34");
                        noticia_34(g(0xb6b7c8), p, clube, 0x34, partida);
                        ja_34 = true;
                    }
                }
            }
            let f = g(p + 0x61);
            *((f + 0x45) as *mut u8) = *((linha - 4) as *const u8);
            core::ptr::write_unaligned((f + 0xb) as *mut i16, w(linha + 3));
            core::ptr::write_unaligned((g(p + 0x61) + 9) as *mut i16, w(linha + 5));
            core::ptr::write_unaligned((g(p + 0x61) + 0xd) as *mut i16, w(linha + 7));
            if ler(clube_de(ler(t)) + 0x53) == ler(clube_de(ler(t + 4)) + 0x53) && *((linha - 8) as *const u8) != 0 {
                let v = media_7c1e30(g(0xb74350), g(p + 0x61), 6, 0xb, 6.5);
                let anterior = g(p + 0x1a);
                let nacao = g(clube + 0x53);
                let f = g(p + 0x61);
                if !(v == 0.0 || v.is_nan())
                    && anterior != 0
                    && nacao != 0
                    && *((anterior + 0x76) as *const u8) != *((nacao + 0x76) as *const u8)
                    && f != 0
                    && w(f + 0xb) > 0x1676
                    && byte(p, 0x57) > 0xa
                    && byte(p, 0x59) < 0xf
                    && (byte(p, 0x5c) < 0xa || byte(p, 0x5d) < 0xa)
                    && ((ler(anterior) == ler(KEY_NATION_FRANCE) && sortear(3) == 0) || sortear(10) == 0)
                {
                    let c = contrato(g(0xae1050), p, clube);
                    if c != 0 && marcas(c) & 0x20 == 0 {
                        crate::comparacao::ramo("jogadores marca 0x20");
                        por_20(c, 0x20);
                    }
                }
            }
        }
        // o tecnico e os auxiliares
        let id = ler(ts + 0x184);
        if id > -1 {
            crate::comparacao::ramo("jogadores tecnico");
            let tec = (g(0xae23bc) as isize + id as isize * 0x6e) as usize;
            let (mut da, mut db, mut dc) = (0i16, 0i16, 0i16);
            if tec != 0 && g(tec + 0x69) != 0 {
                let f = g(tec + 0x69);
                da = w(ts + 0x188).wrapping_sub(w(f + 0xa));
                core::ptr::write_unaligned((f + 0xa) as *mut i16, w(f + 0xa).wrapping_add(da));
                let f = g(tec + 0x69);
                db = w(ts + 0x18a).wrapping_sub(w(f + 8));
                core::ptr::write_unaligned((f + 8) as *mut i16, w(f + 8).wrapping_add(db));
                let f = g(tec + 0x69);
                dc = w(ts + 0x18c).wrapping_sub(w(f + 0xc));
                core::ptr::write_unaligned((f + 0xc) as *mut i16, w(f + 0xc).wrapping_add(dc));
            }
            for k in -1i32..5 {
                let aux = if k == -1 { g(clube + 0xd3) } else { g(clube + 0x19f + k as usize * 4) };
                if aux == 0 || aux == tec || g(aux + 0x69) == 0 {
                    continue;
                }
                for (o, dv) in [(0xa, da), (8, db), (0xc, dc)] {
                    let f = g(aux + 0x69);
                    let v = w(f + o).wrapping_add(ftol(dv as f64 * 0.25) as i16);
                    core::ptr::write_unaligned((f + o) as *mut i16, v);
                    let f = g(aux + 0x69);
                    core::ptr::write_unaligned((f + o) as *mut i16, w(f + o).clamp(1, 10000));
                }
            }
        }
        core::ptr::write_unaligned((clube + 0x80) as *mut i16, w(ts + 0x18e));
    }
}

// ---------- comparacao ----------

pub(crate) static mut ORIGINAIS: [usize; N] = [0; N];
pub(crate) static PLACAR: Placar = Placar::novo("motor");

pub(crate) unsafe fn orig<F: Copy>(i: usize) -> F {
    core::mem::transmute_copy(&ORIGINAIS[i])
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_a(this: usize, j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(0)(this, j);
    let n = tabela_a(this, j);
    PLACAR.registrar(r == n, || format!("tabela_a(pos {}, lado {}) original={} rust={}", byte(j, 0x19), byte(j, 0x27), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_tabela_b(this: usize, j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(1)(this, j);
    let n = tabela_b(this, j);
    PLACAR.registrar(r == n, || format!("tabela_b(pos {}, lado {}) original={} rust={}", byte(j, 0x19), byte(j, 0x27), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_diferenca(this: usize, l: u8, v: u8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u8, u8) -> u8>(2)(this, l, v);
    let n = diferenca_lado(this, l, v);
    PLACAR.registrar(r == n, || format!("diferenca_lado({}, {}) original={} rust={}", l, v, r, n));
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_faixa(l: u8, b: i8, c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(u8, i8, i8) -> u8>(3)(l, b, c);
    let n = faixa(l, b, c);
    PLACAR.registrar(r == n, || format!("faixa({}, {}, {}) original={} rust={}", l, b, c, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_por_diferenca(this: usize, a: i8, b: i8, c: i8, d: i8) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u16>(4)(this, a, b, c, d);
    let n = por_diferenca(this, a, b, c, d);
    PLACAR.registrar(r == n, || format!("por_diferenca({}, {}, {}, {}) original={} rust={}", a, b, c, d, r, n));
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_limite(l: u8, v: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(u8, i8) -> u8>(5)(l, v);
    let n = limite(l, v);
    PLACAR.registrar(r == n, || format!("limite({}, {}) original={} rust={}", l, v, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_condicao(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(6)(this);
    let n = condicao_jogador(this);
    PLACAR.registrar(r == n, || format!("condicao_jogador(lado {}, v {}) original={} rust={}", byte(this, 0x27), byte(this, 0x103), r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_do_jogador(j: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u16>(7)(j);
    let n = tabela_do_jogador(j);
    PLACAR.registrar(r == n, || format!("tabela_do_jogador(motor {:#x}) original={} rust={}", *((j + 0x19e) as *const usize), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_diferenca_jogador(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(8)(this, j);
    let n = diferenca_jogador(this, j);
    PLACAR.registrar(r == n, || format!("diferenca_jogador(lado {}, v {}) original={} rust={}", lado(j), byte(j, 0x103), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_byte_do_lado(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(9)(this, j);
    let n = byte_do_lado(this, j);
    PLACAR.registrar(r == n, || format!("byte_do_lado(lado {}) original={} rust={}", lado(j), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_limite_jogador(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(10)(this);
    let n = limite_jogador(this);
    PLACAR.registrar(r == n, || format!("limite_jogador(lado {}, v {}) original={} rust={}", lado(this), byte(this, 0x103), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_faixa_jogador(this: usize, l: u8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u8) -> u32>(11)(this, l);
    let n = faixa_jogador(this, l);
    PLACAR.registrar(r == n, || format!("faixa_jogador({}, {}, {}) original={} rust={}", l, byte(this, 0x102), byte(this, 0x103), r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_byte_da_posicao(this: usize, i: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(12)(this, i);
    let n = byte_da_posicao(this, i);
    PLACAR.registrar(r == n, || format!("byte_da_posicao({}) original={} rust={}", i, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_campo_pela_posicao(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(13)(this);
    let n = campo_pela_posicao(this);
    PLACAR.registrar(r == n, || format!("campo_pela_posicao(lado {}, pos {}) original={} rust={}", lado(this), byte(this, 0x19), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_pode_escolher(this: usize, j: usize, b: i8, c: i8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u32>(14)(this, j, b, c);
    let n = pode_escolher(this, j, b, c);
    PLACAR.registrar(r == n, || format!("pode_escolher({}, {}) original={} rust={}", b, c, r, n));
    r
}
/// Sorteia: original e Rust partem do mesmo estado do gerador e o jogo segue com o da original.
pub(crate) unsafe extern "thiscall" fn comparar_apontar(this: usize, a: i8, b: i8, c: i8, d: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8)>(15);
    let mut regioes = vec![(this + 0x19a, 2)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || f(this, a, b, c, d),
        || crate::aleatorio::sem_contar(|| apontar(this, a, b, c, d)),
    );
    PLACAR.registrar(igual, || format!("apontar({}, {}, {}, {}) original={:x?} rust={:x?}", a, b, c, d, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_desgastar(this: usize, x: f32, codigo: i16) {
    let f = orig::<unsafe extern "thiscall" fn(usize, f32, i16)>(16);
    let regioes = [(this + 0x4d, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, x, codigo), || desgastar(this, x, codigo));
    PLACAR.registrar(igual, || format!("desgastar({}, {}, 0xd5={}) original={:x?} rust={:x?}", x, codigo, f32_em(this + 0xd5), mo, mr));
}
pub(crate) unsafe extern "thiscall" fn comparar_virado_para(this: usize, a: i8, b: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8) -> u8>(17)(this, a, b);
    let n = virado_para(this, a, b);
    PLACAR.registrar(r == n, || format!("virado_para({}, {}, dir {}, 0x10c {}) original={} rust={}", a, b, *((this + 0x19a) as *const i16), byte(this, 0x10c), r, n));
    r
}

// --- gravar e reproduzir: 0x6f5c30 (3,9 KB, sorteia e altera jogadores) chamada por 0x6cd330 ---
//
// Na comparacao a original roda e cada chamada dela a 0x6f5c30 entra na fita (porta abaixo, so
// durante esta comparacao); a copia Rust recebe a resposta e o efeito gravados (a 0x6f5c30 altera os
// proximos jogadores da lista, que a original pula). No autoteste (jogadores sinteticos) a porta
// responde de forma sintetica, sem rodar a rotina real.

pub(crate) static mut PORTA_6F5C30: usize = 0;
pub(crate) static GRAVAR_6F5C30: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static SINTETICO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// 0x6f5c30: sub esp, 0x218
pub(crate) const PROLOGO_6F5C30: Prologo = Prologo { endereco: 0x6f5c30, bytes: &[0x81, 0xec, 0x18, 0x02, 0x00, 0x00] };

pub(crate) unsafe extern "thiscall" fn porta_6f5c30(p: usize, a: u32, b: u32, c: u32, d: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let rodar = || {
        if SINTETICO.load(Relaxed) {
            (p as u32 ^ a ^ b.wrapping_mul(3) ^ c.wrapping_mul(5) ^ d.wrapping_mul(7)) >> 4 & 1
        } else {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u8 = core::mem::transmute(PORTA_6F5C30);
            f(p, a, b, c, d) as u32
        }
    };
    if SINTETICO.load(Relaxed) {
        if GRAVAR_6F5C30.load(Relaxed) && gravando() {
            return chamar_gravando(0x6f5c30, p, [a, b, c, d, 0], rodar) as u8;
        }
        return rodar() as u8;
    }
    // a comparacao da 0x6f5c30 (128a leva): o desvio dela e esta porta. Ela so e chamada pela
    // 0x6cd330 (`procurar_no_lado`), cuja comparacao a grava como unidade: a comparacao roda dentro
    // da unidade (aninhada, com a fita de fora guardada)
    let comparar = || comparar_responder(p, a, b, c, d, &rodar) as u32;
    if GRAVAR_6F5C30.load(Relaxed) && gravando() {
        return chamar_gravando(0x6f5c30, p, [a, b, c, d, 0], comparar) as u8;
    }
    if gravando_no_topo() {
        return rodar() as u8;
    }
    comparar() as u8
}

pub(crate) unsafe fn comparar_responder(p: usize, a: u32, b: u32, c: u32, d: u32, rodar: &dyn Fn() -> u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(reg, || rodar() as u8, |chamar| {
        responder_com(p, a as i16, b as i8, f32::from_bits(c), d as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("responder(tipo {}, forca {}) original={} rust={} {} chamadas: {}", b as i8, a as i16, r, rr, k, detalhe_fita()));
    r
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_procurar(this: usize, lado: i8, grupo: usize, a3: u32, a4: u32, a5: u32, a6: u32, modo: i8) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32, u32, u32, i8) -> u8>(18);
    // A copia Rust tem de ver o estado que a original viu: 0x6f5c30 altera o jogador que recebe
    // (0x2b, 0x19) e pode tira-lo da lista, entao a lista, a quantidade e esses bytes de cada jogador
    // entram nas regioes (a fita os grava depois de cada chamada).
    let lista = (grupo as isize + lado as isize * 44) as *const usize;
    let mut regioes = vec![(this + 0x1a5, 2), ((grupo as isize + lado as isize) as usize + 0x58, 1), (lista as usize, 44)];
    for i in 0..byte((grupo as isize + lado as isize) as usize, 0x58).clamp(0, 11) as usize {
        let p = *lista.add(i);
        regioes.extend([(p + 0x19, 1), (p + 0x2b, 1)]);
    }
    let gravar = GRAVAR_6F5C30.swap(true, Relaxed);
    let (r, n, igual, sequencia, k) = comparar_com_fita(regioes, || f(this, lado, grupo, a3, a4, a5, a6, modo), |chamar| {
        procurar_com(this, lado, grupo, a3, a4, a5, a6, modo, &mut |p, w, x, y, z| {
            chamar(0x6f5c30, p, [w, x, y, z, 0], [u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0]) as u8
        })
    });
    GRAVAR_6F5C30.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && r == n, || {
        format!("procurar_no_lado(lado {}, modo {}, {} chamadas) original={} rust={}: {}", lado, modo, k, r, n, detalhe_fita())
    });
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_alcanca(j: usize, b: i8, c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, i8, i8) -> u8>(19)(j, b, c);
    let n = alcanca(j, b, c);
    PLACAR.registrar(r == n, || format!("alcanca(lado {}, x {}, y {}, {}, {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), b, c, r, n));
    r
}
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_classificar(this: usize, codigo: i16, a: i8, b: i8, c: i8, d: i8, forca: f32, saida: *mut u16) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i16, i8, i8, i8, i8, f32, *mut u16) -> u8>(20);
    let (r, n, igual, mo, mr) =
        crate::comparacao::lado_a_lado(&[(saida as usize, 2)], || f(this, codigo, a, b, c, d, forca, saida), || classificar_lance(this, codigo, a, b, c, d, forca, saida));
    PLACAR.registrar(igual && r == n, || format!("classificar_lance({}, {} {} {} {}, {}) original={} rust={} {:x?} {:x?}", codigo, a, b, c, d, forca, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_distancia_ao_ponto(this: usize, a: i8, b: i8) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8) -> f32>(21)(this, a, b);
    let n = distancia_ao_ponto(this, a, b);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_ao_ponto({}, {}) original={} rust={}", a, b, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_distancia_ao_gol(this: usize) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> f32>(22)(this);
    let n = distancia_ao_gol(this);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_ao_gol(lado {}) original={} rust={}", lado(this), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_mais_perto(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(23)(this, j);
    let n = mais_perto_do_gol(this, j);
    PLACAR.registrar(r == n, || format!("mais_perto_do_gol(lado {}) original={} rust={}", lado(j), r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_na_linha(this: usize, x1: i8, y1: i8, x2: i8, y2: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(24)(this, x1, y1, x2, y2);
    let n = jogadores_na_linha(this, x1, y1, x2, y2);
    PLACAR.registrar(r == n, || format!("jogadores_na_linha({}, {}) -> ({}, {}) original={} rust={}", x1, y1, x2, y2, r, n));
    r
}
/// Sorteia e grava `saida`: mesmo estado do gerador para as duas, o jogo segue com o da original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_consegue(this: usize, a: i8, b: i8, calcular: i8, forca: f32, ajuste: f32, saida: *mut u8) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, f32, f32, *mut u8) -> u8>(25);
    let mut regioes: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        regioes.push((saida as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || f(this, a, b, calcular, forca, ajuste, saida),
        || crate::aleatorio::sem_contar(|| consegue_jogar(this, a, b, calcular, forca, ajuste, saida)),
    );
    PLACAR.registrar(igual && r == n, || format!("consegue_jogar({}, {}, {}, {}, {}) original={} rust={} {:x?} {:x?}", a, b, calcular, forca, ajuste, r, n, mo, mr));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_na_area(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(26)(this);
    let n = na_area(this);
    PLACAR.registrar(r == n, || format!("na_area(lado {}, x {}, y {}) original={} rust={}", lado(this), byte(this, 0x102), byte(this, 0x103), r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_marcas(this: usize, posicao: i8) -> u64 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u64>(27)(this, posicao);
    let n = marcas_da_posicao(this, posicao);
    PLACAR.registrar(r == n, || format!("marcas_da_posicao({}) original={:#x} rust={:#x}", posicao, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_papel_ataque(j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(28)(j);
    let n = papel_no_ataque(j);
    PLACAR.registrar(r == n, || format!("papel_no_ataque(lado {}, pos {}) original={} rust={}", lado(j), byte(j, 0x19), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_papel_defesa(j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(29)(j);
    let n = papel_na_defesa(j);
    PLACAR.registrar(r == n, || format!("papel_na_defesa(lado {}, pos {}) original={} rust={}", lado(j), byte(j, 0x19), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_posicionar(this: usize, j: usize, sx: *mut i8, sy: *mut i8, a4: u32) {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8, u32)>(30);
    let regioes = [(sx as usize, 1), (sy as usize, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, j, sx, sy, a4), || posicionar(this, j, sx, sy, a4));
    PLACAR.registrar(igual, || format!("posicionar(lado {}, pos {}, bola {} {}) original={:x?} rust={:x?}", lado(j), byte(j, 0x19), byte(this, 0x8ea7), byte(this, 0x8ea8), mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_linhas(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(31);
    let regioes = [(this + 0x8eaf, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this), || linhas_defensivas(this));
    PLACAR.registrar(igual, || format!("linhas_defensivas original={:x?} rust={:x?}", mo, mr));
}

/// Sorteia e grava (0x2b, 0x1a2, 0x35): mesmo estado do gerador, o jogo segue com o da original.
pub(crate) unsafe extern "thiscall" fn comparar_atualizar(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(32);
    let mut regioes = vec![(this + 0x2b, 1), (this + 0x1a2, 4), (this + 0x35, 2)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) =
        crate::comparacao::lado_a_lado(&regioes, || f(this), || crate::aleatorio::sem_contar(|| atualizar_jogador(this)));
    PLACAR.registrar(igual, || format!("atualizar_jogador(papel {:#x}) original={:x?} rust={:x?}", papel_word(*((this + 0x19e) as *const usize), this), mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_momento(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(33);
    let mut regioes = vec![(this + 0x104, 10)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this), || crate::aleatorio::sem_contar(|| sortear_momento(this)));
    PLACAR.registrar(igual, || format!("sortear_momento original={:x?} rust={:x?}", mo, mr));
}
pub(crate) unsafe extern "thiscall" fn comparar_posicao_um(this: usize, l: i8) -> usize {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8) -> usize>(34);
    let mut regioes = vec![(this + 0xf59e, 8)];
    for k in 0..40 {
        regioes.push((this + 0x4796 + 8920 * (k / 20) + (k % 20) * 0x1be + 0x19, 2));
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || f(this, l), || manter_posicao_um(this, l));
    PLACAR.registrar(igual && r == n, || format!("manter_posicao_um({}) original={:#x} rust={:#x} memoria igual={}", l, r, n, igual));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_caminho(this: usize, j: usize, ig: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8) -> u8>(35)(this, j, ig);
    let n = caminho_livre(this, j, ig);
    PLACAR.registrar(r == n, || format!("caminho_livre(lado {}, {} {}, {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), ig, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_espaco(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(36)(this, j);
    let n = espaco_a_frente(this, j);
    PLACAR.registrar(r == n, || format!("espaco_a_frente(lado {}, {} {}) original={} rust={}", lado(j), byte(j, 0x102), byte(j, 0x103), r, n));
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_fora_do_meio(v: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i8) -> u8>(37)(v);
    let n = fora_do_meio(v);
    PLACAR.registrar(r == n, || format!("fora_do_meio({}) original={} rust={}", v, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_alem(this: usize, y: i8, l: i8, sg: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8) -> u8>(38)(this, y, l, sg);
    let n = alem_da_linha(this, y, l, sg);
    PLACAR.registrar(r == n, || format!("alem_da_linha({}, {}, {}) original={} rust={}", y, l, sg, r, n));
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_distancia_entre(a: i8, b: i8, c: i8, d: i8) -> f32 {
    let r = orig::<unsafe extern "stdcall" fn(i8, i8, i8, i8) -> f32>(39)(a, b, c, d);
    let n = distancia_entre(a, b, c, d);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("distancia_entre({} {} {} {}) original={} rust={}", a, b, c, d, r, n));
    r
}

comparar_leitor!(comparar_ler_8ea7, ler_8ea7, 40);
comparar_leitor!(comparar_ler_8ea8, ler_8ea8, 41);
comparar_leitor!(comparar_ler_8ea9, ler_8ea9, 42);
comparar_leitor!(comparar_ler_8eaa, ler_8eaa, 43);
comparar_leitor!(comparar_ler_8eb2, ler_8eb2, 44);
comparar_gravador!(comparar_gravar_8ea9, gravar_8ea9, 45, 0x8ea9, u8);
comparar_gravador!(comparar_gravar_8ea7, gravar_8ea7, 46, 0x8ea7, u8);
comparar_gravador!(comparar_gravar_8ea8, gravar_8ea8, 47, 0x8ea8, u8);
comparar_gravador!(comparar_gravar_8eaa, gravar_8eaa, 48, 0x8eaa, u8);
comparar_gravador!(comparar_gravar_8ed8, gravar_8ed8, 49, 0x8ed8, u16);
pub(crate) unsafe extern "thiscall" fn comparar_ler_8f7a(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(50)(this);
    let n = ler_8f7a(this);
    PLACAR.registrar(r == n, || format!("ler_8f7a original={:#x} rust={:#x}", r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_posse(this: usize, v: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8)>(51);
    let regioes = [(this + 0x8eab, 1), ((this as isize + 0x8d46 + v as isize) as usize, 1), (this + 0x8d46, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || f(this, v), || registrar_posse(this, v));
    PLACAR.registrar(igual, || format!("registrar_posse({}) original={:x?} rust={:x?}", v, mo, mr));
}
pub(crate) unsafe extern "thiscall" fn comparar_na_lista(this: usize, alvo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(52)(this, alvo);
    let n = na_lista(this, alvo);
    PLACAR.registrar(r == n, || format!("na_lista(n {}) original={} rust={}", byte(this, 0x101), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_extremos(this: usize) {
    let f = orig::<unsafe extern "thiscall" fn(usize)>(53);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0xf58e, 16)], || f(this), || guardar_extremos(this));
    PLACAR.registrar(igual, || format!("guardar_extremos original={:x?} rust={:x?}", mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_contar_ativos(this: usize, l: i8) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8) -> u8>(54);
    let reg = [((this as isize + 4 + l as isize) as usize, 1)];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, l), || contar_ativos(this, l));
    PLACAR.registrar(igual && r == n, || format!("contar_ativos({}) original={} rust={}", l, r, n));
    r
}
pub(crate) unsafe extern "stdcall" fn comparar_codigo_especial(c: i8) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(i8) -> u8>(55)(c);
    let n = codigo_especial(c);
    PLACAR.registrar(r == n, || format!("codigo_especial({}) original={} rust={}", c, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_atras(this: usize, j: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(56)(this, j);
    let n = atras_do_extremo(this, j);
    PLACAR.registrar(r == n, || format!("atras_do_extremo(lado {}, y {}) original={} rust={}", lado(j), byte(j, 0x103), r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_sem_partida(this: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u8>(57)(this);
    let n = sem_partida(this);
    PLACAR.registrar(r == n, || format!("sem_partida original={} rust={}", r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_pressao_area(this: usize, l: i8) {
    let f = orig::<unsafe extern "thiscall" fn(usize, i8)>(58);
    let mut reg = vec![((this as isize + 6 + l as isize) as usize, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || f(this, l), || crate::aleatorio::sem_contar(|| pressao_na_area(this, l)));
    PLACAR.registrar(igual, || format!("pressao_na_area({}) original={:x?} rust={:x?}", l, mo, mr));
}
pub(crate) unsafe extern "thiscall" fn comparar_mais_proximo(this: usize, j: usize, a: i8, b: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, i8, i8) -> u8>(59)(this, j, a, b);
    let n = mais_proximo_do_ponto(this, j, a, b);
    PLACAR.registrar(r == n, || format!("mais_proximo_do_ponto({}, {}) original={} rust={}", a, b, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_pressao_ponto(this: usize, l: i8, a: i8, b: i8, t: i8) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> u8>(60)(this, l, a, b, t);
    let n = pressao_no_ponto(this, l, a, b, t);
    PLACAR.registrar(r == n, || format!("pressao_no_ponto({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}
pub(crate) unsafe extern "thiscall" fn comparar_menor_distancia(this: usize, l: i8, a: i8, b: i8, t: i8) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8, i8, i8, i8) -> f32>(61)(this, l, a, b, t);
    let n = menor_distancia(this, l, a, b, t);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("menor_distancia({}, {}, {}, {}) original={} rust={}", l, a, b, t, r, n));
    r
}

comparar_repartir!(comparar_repartir_a, repartir_a, 62);
comparar_repartir!(comparar_repartir_b, repartir_b, 63);
comparar_repartir!(comparar_repartir_c, repartir_c, 64);
pub(crate) unsafe extern "thiscall" fn comparar_escolher(this: usize, arg: u32) -> u8 {
    let f = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(65);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || f(this, arg), || crate::aleatorio::sem_contar(|| escolher_por_papel(this, arg)));
    PLACAR.registrar(igual && r == n, || format!("escolher_por_papel({:#x}) original={} rust={}", arg, r as i8, n as i8));
    r
}

comparar_valor!(comparar_direcao, direcao, 66, () -> u16);
comparar_valor!(comparar_x_fora, x_fora_do_meio, 68, () -> u8);
comparar_valor!(comparar_e_o_guardado, e_o_guardado, 69, () -> u8);
comparar_valor!(comparar_lado_do_lance, lado_do_lance, 70, () -> u8);
comparar_valor!(comparar_jogador_no_bloco, jogador_no_bloco, 71, (l: i8, k: i8) -> usize);
comparar_valor!(comparar_tabela_a_1a, tabela_a_1a, 72, (j: usize) -> u16);
comparar_valor!(comparar_momento_marcado, momento_marcado, 73, () -> u32);
comparar_valor!(comparar_estado_do_lado, estado_do_lado, 74, () -> u8);
comparar_valor!(comparar_estado_do_adversario, estado_do_adversario, 75, () -> u8);
comparar_valor!(comparar_perto_do_gol, perto_do_gol, 78, () -> u8);
comparar_valor!(comparar_atras_no_agregado, atras_no_agregado, 322, (lado: i8) -> u8);
comparar_valor!(comparar_companheiro_livre, companheiro_livre, 323, (p: usize) -> usize);

