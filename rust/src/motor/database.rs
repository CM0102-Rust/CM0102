use super::*;

/// does `b` have `a` among its three favourites (+0x9b, +0x9f, +0xa3)? Compared by the result
/// Original: `favourite_of` (0x547790, cdecl).
///
/// [pt] { 0x547790 } (cdecl): `b` tem `a` entre os tres favoritos (+0x9b, +0x9f, +0xa3)? No fechamento,
/// a pessoa e o clube adversario.
pub(crate) unsafe extern "C" fn favourite_of(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    [0x9b, 0x9f, 0xa3].iter().any(|&o| read_usize(b + o) == a) as u32
}

/// does `b` have `a` among its three rejected (+0xa7, +0xab, +0xaf)? Compared by the result
/// Original: `rejected_by` (0x5477d0, cdecl).
///
/// [pt] { 0x5477d0 } (cdecl): `b` tem `a` entre os tres rejeitados (+0xa7, +0xab, +0xaf)?
pub(crate) unsafe extern "C" fn rejected_by(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    [0xa7, 0xab, 0xaf].iter().any(|&o| read_usize(b + o) == a) as u32
}

/// Liga as unidades da fita do fechamento (`exe::update_player_reputation_after_match`, `exe::update_manager_reputation_after_match`, `exe::match_eng_compute_club_reputation` e o conteudo da 0x68d0d0).
pub(crate) static RECORD_WRAP_UP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True when the fechamento call taping is switched on (RECORD_WRAP_UP) and a recording tape is open.
pub(crate) fn is_recording_wrap_up_calls() -> bool {
    RECORD_WRAP_UP.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// The match wrap-up: `0x6d1cc0` (cdecl, `match_eng.cpp`, called by the engine step `0x6b98c0`) with the calls that
/// touch things outside the engine injected.
///
/// Injected: `0x6d26b0` (each player's player sheet), `0x6d3610` and `0x6d3de0` (the reputations), `0x68d0d0` (copying
/// an event into list [`0xb67dc8`]).
///
/// With the day's event list (`+0x478e`) and the match (`+0x4792`, `+0x40` set), it walks the events (`0x68d1b0`):
/// `0x82` goes to the list; `0x1e` (offer?) by the score (`+0x43`/`+0x44`, a draw by the clubs' strength `+0x80` ×
/// 1.2/0.8) and by the manager on the other side (`+0xd3`, `0x544410` drawn against 20 or 5, `+0x14` > 5) marks the
/// event (`+0xe` 1, `+0xf` the manager) or compares it with the strength of the event's club (`+0x80` with two random
/// draws of 1000, or above 7000; on the other side, 1 in 4) and marks it (`+0xe` 0, `+0xf` −1); the others, of the
/// match player: for 6 by the marks `+0x24`/`+0x23`, for the rest with `+0x1b` < 7 by random draw(`+0x17`).
///
/// Then, per side, each player on the pitch (substitutes only if they came on, `+0x1f`) with a player sheet and a
/// contract: `0x6d26b0` with the rating `+0x1b`; with the list: outside national teams and friendlies, not being the
/// manager, a player who went off (`+0x20`) without being substituted back (`+0x22`) and with a contract (`+0x3d`) and
/// reputation (`+0xb` > 4250) may ask to leave (event 0xa, 0x4000000) by random draws of `+0x1a2`, `+0x5d`, stamina
/// (`+0x4d`) and `+0x57`/`+0x5b`; the unhappiness (−`+0x31`, +10 against the rival `+0xb3`/`+0xb7`/`+0xbb` or rejected
/// by the opponent `0x5477d0`, −10 if his favourite `0x547790`, +10/−20 by the preferences `0x547d30`/`0x547d60`)
/// against random draw((`+0x5a` × `+0x5b` × `+0x5d` + 20) × 100) generates event 0xb.
///
/// The average of the ratings by minutes played (90 in the fast engine; `+0x21`/`+0x20`/minute, minus the entry
/// `+0x1f`), −0.25 at home (`+0x18`) or +0.25 away, goes to `0x6d3610` and, outside national teams, `0x6d3de0`. Without
/// minutes, the average stays that of the previous side (for the 1st side it was stack garbage: 0 in the determinized
/// original, like the final 4 bytes of the event built on the stack).
///
/// [pt] { 0x6d1cc0 } (cdecl, match_eng.cpp, chamada pelo passo do motor 0x6b98c0) com as chamadas que
/// mexem fora do motor injetadas (0x6d26b0 a ficha de cada jogador, 0x6d3610 e 0x6d3de0 as
/// reputacoes, 0x68d0d0 a copia de um evento na lista [0xb67dc8]): o fechamento da partida. Com a
/// lista de eventos do dia (+0x478e) e a partida (+0x4792, +0x40 ligado), percorre os eventos (0x68d1b0):
/// 0x82 vai para a lista; 0x1e (proposta?) pelo placar (+0x43/+0x44, empate pela forca +0x80 dos
/// clubes x 1,2/0,8) e pelo tecnico do outro lado (+0xd3, 0x544410 sorteado contra 20 ou 5, +0x14 > 5)
/// marca o evento (+0xe 1, +0xf o tecnico) ou o compara com a forca do clube do evento (+0x80 com dois
/// sorteios de 1000, ou acima de 7000; no outro lado, 1 em 4) e o marca (+0xe 0, +0xf -1); os outros,
/// do jogador da partida: no 6 pelas marcas +0x24/+0x23, nos demais com +0x1b < 7 por sorteio(+0x17).
/// Depois, por lado, cada jogador em campo (reservas so se entraram, +0x1f) com ficha e contrato:
/// 0x6d26b0 com a nota +0x1b; com a lista: fora das selecoes e dos amistosos, sem ser o tecnico, quem
/// saiu (+0x20) sem ser substituido de volta (+0x22) e com contrato (+0x3d) e reputacao (+0xb >
/// 4250) pode pedir para sair (evento 0xa, 0x4000000) por sorteios de +0x1a2, +0x5d, folego (+0x4d) e
/// +0x57/+0x5b; a insatisfacao (-+0x31, +10 contra o rival +0xb3/+0xb7/+0xbb ou rejeitado pelo
/// adversario 0x5477d0, -10 favorito dele 0x547790, +10/-20 pelas preferencias 0x547d30/0x547d60) contra
/// sorteio((+0x5a x +0x5b x +0x5d + 20) x 100) gera o evento 0xb. A media das notas pelos minutos
/// jogados (90 no motor rapido; +0x21/+0x20/minuto, menos a entrada +0x1f) -0,25 em casa (+0x18) ou
/// +0,25 fora vai para 0x6d3610 e, fora das selecoes, 0x6d3de0. Sem minutos, a media fica a do lado
/// anterior (no 1o lado era lixo de pilha: 0 no original determinizado, como os 4 bytes finais do
/// evento montado na pilha).
pub(crate) unsafe fn match_wrap_up_with(m: Engine, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let list_global = || read_usize(G_MATCH_DAY_PLAYER_INCIDENT_LIST);
    // puts a copy of the block (0x19 bytes) in the events list (the content is checked on the tape)
    let mut event = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, p: usize| {
        request_buffers(&[(p, 0x19)], &[]);
        call(FN_LLIST_ADD_TAIL, list_global(), [0, 0x19, 0, 0, 0]);
    };
    let fixture = || m.r#match();
    let person_of = |id: i32| read_usize(G_STAFF_TABLE).wrapping_add(id as usize * staff::SIZE);
    let list = Llist(m.event_list());
    if !list.is_null() && list_global() != 0 && fixture() != 0 && read_i8(fixture() + 0x40) != 0 {
        list.set_cursor(((list.head() as u32)) as usize);
        let mut i: i8 = 0;
        while (i as i16) < read_i16(list.addr()) {
            let e = exe::llist_get_next(list);
            if e == 0 {
                break;
            }
            let kind = read_i16(e + match_incident::KIND);
            crate::comparison::branch("match_wrap_up day event");
            let mark = |value: u8, who: u32| {
                write_u8(e + match_incident::OUTCOME, value);
                write_u32(e + match_incident::RESPONDER_ID, who);
            };
            let pt = fixture();
            let club_ev = || read_usize(G_CLUB_TABLE).wrapping_add(read_usize(e + match_incident::CLUB_ID) * club::SIZE);
            let injury_word_0c = || read_i16(read_usize((person_of(read_i32(e))) + 0x61) + 0xb);
            if kind == 0x82 {
                event(call, e);
            } else if kind == 0x1e {
                let (c1c, c20) = (read_usize(pt + fixture::HOME_CLUB), read_usize(pt + fixture::AWAY_CLUB));
                if read_i8(e + 0xd) != 0 {
                    let (a, b) = (read_i8(pt + fixture::HOME_GOALS), read_i8(pt + fixture::AWAY_GOALS));
                    let ok = b < a || (b == a && (read_i16(c1c + club::REPUTATION) as f64) < read_i16(c20 + club::REPUTATION) as f64 * read_f64(rdata::F64_0_7));
                    let t = read_usize(c20 + club::ASSISTANT_MANAGER);
                    if ok && t != 0 && read_usize(t + staff::NON_PLAYER) != 0 {
                        let x = read_usize(t + staff::NON_PLAYER);
                        let r1 = random((exe::non_player_attribute_rating(x, (read_i8(x + 0x17) as u8 as u32) as i8) as i8) as i32);
                        if r1 > random(0x14) && read_i8(x + 0x14) > 5 {
                            crate::comparison::branch("match_wrap_up offer accepted");
                            mark(1, read_u32(t));
                            event(call, e);
                        } else {
                            let rep = injury_word_0c();
                            let r = random(1000);
                            let strength = read_i16((club_ev()) + 0x80) as i32 - r + random(1000);
                            if rep as i32 > strength || rep > 7000 {
                                mark(0, u32::MAX);
                                event(call, e);
                            }
                        }
                    }
                } else {
                    let (a, b) = (read_i8(pt + fixture::HOME_GOALS), read_i8(pt + fixture::AWAY_GOALS));
                    let ok = b > a || (b == a && (read_i16(c20 + club::REPUTATION) as f64) < read_i16(c1c + club::REPUTATION) as f64 * read_f64(rdata::F64_0_6));
                    let t = read_usize(c1c + club::ASSISTANT_MANAGER);
                    if ok && t != 0 && read_usize(t + staff::NON_PLAYER) != 0 {
                        let x = read_usize(t + staff::NON_PLAYER);
                        if random((exe::non_player_attribute_rating(x, (read_i8(x + 0x17) as u8 as u32) as i8) as i8) as i32) > 5 && read_i8(x + 0x14) > 5 {
                            crate::comparison::branch("match_wrap_up offer accepted");
                            mark(1, read_u32(t));
                            event(call, e);
                        } else {
                            if injury_word_0c() > read_i16((club_ev()) + 0x80) && random(4) == 0 {
                                mark(0, u32::MAX);
                            }
                            event(call, e);
                        }
                    }
                }
            } else {
                let person = person_of(read_i32(e));
                if person != 0 {
                    // the event’s player on one of the two sides
                    let achar = |side: usize| (0..20usize).find(|&k| {
                        let p = m.addr() + engine::PLAYERS + (k + side * 20) * 0x1be;
                        read_usize(p + match_player::STAFF) == person && read_i8(p + match_player::POSITION) >= 0
                    });
                    let found = achar(0).map(|k| (k, 0)).or_else(|| achar(1).map(|k| (k, 1)));
                    if let Some((k, side)) = found {
                        let p = m.addr() + engine::PLAYERS + (k + side * 20) * 0x1be;
                        let vale = if kind == 6 {
                            crate::comparison::branch("match_wrap_up type 6 event");
                            read_i8(p + match_player::YELLOW_CARDS) == 2 || read_i8(p + match_player::RED_CARDS) != 0
                        } else {
                            read_i8(p + match_player::MATCH_GRADE) < 7 && random(read_i8(p + match_player::MATCH_FORM) as i32) == 0
                        };
                        if vale {
                            crate::comparison::branch("match_wrap_up player event");
                            mark(0, u32::MAX);
                            event(call, e);
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let mut average = 0f64;
    for side in 0..2usize {
        let (mut soma, mut somap) = (0i32, 0i32);
        let pt = fixture();
        let (meu, outro) = if side == 0 { (read_usize(pt + fixture::HOME_CLUB), read_usize(pt + fixture::AWAY_CLUB)) } else { (read_usize(pt + fixture::AWAY_CLUB), read_usize(pt + fixture::HOME_CLUB)) };
        for k in 0..20usize {
            let p = m.addr() + engine::PLAYERS + (k + side * 20) * 0x1be;
            if k > 10 && read_i8(p + match_player::MINUTE_ON) == -1 {
                continue;
            }
            let person = read_usize(p + match_player::STAFF);
            if person != 0 && read_usize(p + match_player::PLAYER) != 0 && *(person as *const i32) < read_i32(G_STAFF_COUNT) {
                call(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH, p, [person as u32, meu as u32, outro as u32, (read_i8(p + match_player::MATCH_GRADE) as f32).to_bits(), read_u8(m.addr()) as u32 | 1 << 8]);
                if list_global() != 0 && exe::database_is_national_team(meu) == 0 && read_i8(pt + fixture::HOME_SIDE_MARK) != 0 {
                    let ct = contract(read_usize(G_CONTRACT_MANAGER), person, meu);
                    if ct != 0
                        && meu != 0
                        && person != read_usize(meu + club::MANAGER)
                        && read_usize(pt + fixture::COMPETITION) != 0
                        && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(pt + fixture::COMPETITION)) == 0
                        && read_i8(p + match_player::MINUTE_OFF) != -1
                        && read_i8(p + match_player::INJURY_MINUTE) == -1
                        && read_i8(ct + 0x3d) > 0
                        && read_i16(read_usize(p + match_player::PLAYER) + 0xb) > 0x109a
                        && fixture() != 0
                        && read_i8(fixture() + 0x40) != 0
                    {
                        let r1 = random(ftol(read_f32(p + match_player::CONFIDENCE) as f64));
                        if r1 > random(100) {
                            let r3 = random(read_i8(person + staff::TEMPERAMENT) as i32);
                            if r3 < random(10) {
                                let f = read_f32(p + match_player::STAMINA);
                                if ((random(0x5dc) + 0x1d4c) as f64) < f as f64 {
                                    let b5b = read_i8(person + staff::PROFESSIONALISM);
                                    if b5b < 6 && read_i8(person + staff::AMBITION) > 10 {
                                        let r6 = random(read_i8(person + staff::AMBITION) as i32);
                                        if random(b5b as i32) < r6 {
                                            crate::comparison::branch("match_wrap_up asks to leave");
                                            let mut ev = [0u8; 0x19];
                                            ev[..4].copy_from_slice(&read_u32(person).to_le_bytes());
                                            ev[4..8].copy_from_slice(&read_u32(meu).to_le_bytes());
                                            ev[8..12].copy_from_slice(&0x4000000u32.to_le_bytes());
                                            ev[0xd] = side as u8;
                                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                                            ev[0x13..0x15].copy_from_slice(&10u16.to_le_bytes());
                                            event(call, ev.as_ptr() as usize);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // a insatisfacao
                    let mut x = -(read_i16(p + 0x31) as i32);
                    if [0xb3, 0xb7, 0xbb].iter().any(|&o| read_usize(meu + o) == outro) {
                        x += 10;
                    }
                    if exe::club_dislikes_staff(person, outro) != 0 {
                        x += 10;
                    } else if exe::club_favours_staff(person, outro) != 0 {
                        x -= 10;
                    }
                    if exe::staff_dislikes_club(person, outro) != 0 {
                        x += 10;
                    } else if exe::staff_favours_club(person, outro) != 0 {
                        x -= 20;
                    }
                    let (a, b, c) = (read_i8(person + staff::PRESSURE) as i32, read_i8(person + staff::PROFESSIONALISM), read_i8(person + staff::TEMPERAMENT));
                    if b < 6 && c < 10 {
                        let r1 = random((a * b as i32 * c as i32 + 0x14) * 100);
                        if r1 < random(x + 0x32) {
                            crate::comparison::branch("match_wrap_up unhappy");
                            let mut ev = [0u8; 0x19];
                            ev[..4].copy_from_slice(&read_u32(person).to_le_bytes());
                            ev[4..8].copy_from_slice(&read_u32(meu).to_le_bytes());
                            ev[0xd] = side as u8;
                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());
                            event(call, ev.as_ptr() as usize);
                        }
                    }
                }
            }
            // the minutes played
            let minute = (m.clock() as i32 / 11) as i8;
            let v: i8 = if k >= 11 {
                let c = read_i8(p + match_player::MINUTE_ON);
                if c == -1 {
                    continue;
                }
                let end = if read_i8(p + match_player::MINUTE_LEFT_PITCH) > -1 { read_i8(p + match_player::MINUTE_LEFT_PITCH) } else if read_i8(p + match_player::MINUTE_OFF) > -1 { read_i8(p + match_player::MINUTE_OFF) } else { minute };
                end.wrapping_sub(c)
            } else if read_u8(m.addr()) != 0 {
                0x5a
            } else if read_i8(p + match_player::MINUTE_LEFT_PITCH) > -1 {
                read_i8(p + match_player::MINUTE_LEFT_PITCH)
            } else if read_i8(p + match_player::MINUTE_OFF) > -1 {
                read_i8(p + match_player::MINUTE_OFF)
            } else {
                minute
            };
            if v == 0 {
                continue;
            }
            soma += v as i32;
            somap += read_i8(p + match_player::MATCH_GRADE) as i32 * v as i32;
        }
        if soma != 0 {
            let q = somap as f64 / soma as f64;
            average = if read_usize(pt + fixture::STADIUM) == read_usize(meu + club::STADIUM) { q - read_f64(rdata::F64_0_1) } else { q + read_f64(rdata::F64_0_2) };
        }
        crate::comparison::branch(if soma != 0 { "match_wrap_up media do lado" } else { "match_wrap_up lado sem minutos" });
        let value = (average as f32).to_bits();
        let alt = read_u8(m.addr()) as u32;
        call(FN_UPDATE_MANAGER_REPUTATION_AFTER_MATCH, m.addr() + engine::MATCH_MANAGERS + side * 0xd0, [meu as u32, outro as u32, pt as u32, value, alt | 1 << 8]);
        if exe::database_is_national_team(meu) == 0 {
            call(FN_MATCH_ENG_COMPUTE_CLUB_REPUTATION, m.addr(), [side as u32, value, alt | 1 << 8, 0, 0]);
        }
    }
}

/// Real dispatcher for the calls injected into fechamento: player/manager reputation updates (`exe::update_player_reputation_after_match`, `exe::update_manager_reputation_after_match`),
/// club reputation (`exe::match_eng_compute_club_reputation`) or adding the requested block to the incident list (0x68d0d0).
///
/// [pt] Despacho real das chamadas injetadas do fechamento.
pub(crate) unsafe fn dispatch_wrap_up_call(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    let m = |p: MatchPlayer| p.engine();
    match routine {
        0x6d26b0 => {
            let f: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH);
            f(this, a[0] as usize, a[1] as usize, a[2] as usize, read_usize((m(MatchPlayer(this))).addr() + 0x4792), f32::from_bits(a[3]), a[4] & 0xff, (a[4] >> 8) & 0xff);
        }
        0x6d3610 => {
            let f: unsafe extern "C" fn(usize, usize, usize, usize, f32, u8, u8) = core::mem::transmute(FN_UPDATE_MANAGER_REPUTATION_AFTER_MATCH);
            f(this, a[0] as usize, a[1] as usize, a[2] as usize, f32::from_bits(a[3]), a[4] as u8, (a[4] >> 8) as u8);
        }
        0x6d3de0 => {
            let f: unsafe extern "C" fn(usize, i8, f32, u8, u8) = core::mem::transmute(FN_MATCH_ENG_COMPUTE_CLUB_REPUTATION);
            f(this, a[0] as u8 as i8, f32::from_bits(a[1]), a[2] as u8, (a[2] >> 8) as u8);
        }
        _ => {
            // 0x68d0d0: the block requested last (request_buffers) is the one that goes to the list
            let p = REQUESTED_BUFFERS.lock().unwrap_or_else(|e| e.into_inner()).take().map_or(0, |v| v.0[0].0);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
            f(this, p, a[1]);
        }
    }
    0
}

/// Zeroes 8 KB of stack just below the caller so the original routine called next runs with zeroed uninitialised locals
/// during comparison.
///
/// [pt] Zera 8 KB da pilha logo abaixo (a rotina original chamada em seguida ocupa essa area): na
/// comparacao, a original determinizada roda com as variaveis sem valor em zero, como no desvio.
#[inline(never)]
pub(crate) fn zero_stack_below() {
    let mut a = [0u8; 0x2000];
    std::hint::black_box(&mut a);
}

/// Memory regions compared for fechamento: RNG state, the match engine, the day's incident list header/cursor and
/// each 0x19-byte incident in it.
///
/// [pt] As regioes do fechamento: o sorteio, o motor, a lista de eventos do dia (cabecalho e cursor) e os
/// eventos dela (+0xe e +0xf mudam).
pub(crate) unsafe fn wrap_up_regions(m: Engine) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let list = m.event_list();
    if list != 0 {
        reg.push((list, 0xe));
        let mut in_the = read_usize(list + 2);
        for _ in 0..read_i16(list).max(0) {
            if in_the == 0 {
                break;
            }
            let e = read_usize(in_the);
            if e != 0 {
                reg.push((e, 0x19));
            }
            in_the = read_usize(in_the + 6);
        }
    }
    reg
}

/// Returns 1 when the given type byte is not 11, else 0 [inference: a competition/club type filter].
///
/// [pt] { 0x53b610 } (Database.cpp): o tipo nao e 11?
pub(crate) unsafe extern "C" fn type_not_11(kind: i8) -> u32 {
    (kind != 11) as u32
}

/// Returns 1 if type code `tipo` is 11, 12, 13 or 15, else 0 [inference: a class of club or competition types].
///
/// [pt] { 0x53b620 } (Database.cpp): o tipo e 11, 12, 13 ou 15?
pub(crate) unsafe extern "C" fn type_11_to_15(kind: i8) -> u32 {
    matches!(kind, 11 | 12 | 13 | 15) as u32
}

/// Class of a national side for the national-teams blocks: 1 when `flag` is set, otherwise 2 for a nation's second team
/// and 0 for any other club.
///
/// [pt] { 0x76d780 } (stdcall, national_teams.cpp): com `flag`, 1; sem ela, 2 para a segunda selecao
/// (0x5409b0) e 0 para as outras.
pub(crate) unsafe extern "stdcall" fn national_side_class(club: usize, flag: u32) -> u8 {
    let and_second = exe::club_is_national_team(club) != 0;
    if flag != 0 {
        1
    } else if and_second {
        2
    } else {
        0
    }
}

/// Pointer to the per-person flag byte: [this+4] + staff id x 110 + k x 20 + 0x45.
///
/// [pt] O byte das marcas por pessoa: [this+4] + id x 110 + k x 20 + 0x45.
pub(crate) unsafe fn person_flag_byte(this: usize, person: usize, k: i8) -> *mut u8 {
    read_usize(this + 4)
        .wrapping_add((read_i32(person) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
        .wrapping_add(0x45) as *mut u8
}

/// Returns flag `k` of the person in the national-teams table (`this`; byte at [this+4] + id x 110 + k x 20 + 0x45).
///
/// [pt] { 0x76d7b0 } (ret 8, national_teams.cpp): a marca `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn read_person_mark(this: usize, person: usize, k: i8) -> u8 {
    *person_flag_byte(this, person, k)
}

/// Clears flag `k` of the person in the national-teams table (`this`; byte at [this+4] + id x 110 + k x 20 + 0x45).
///
/// [pt] { 0x76d7e0 } (ret 8, national_teams.cpp): zera a marca `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn clear_person_mark(this: usize, person: usize, k: i8) {
    *person_flag_byte(this, person, k) = 0;
}

/// the first record with +0 zero in table 0xae23f8 (60 bytes, 0xae23a0 records), cached at 0x9b59b4; with none, a
/// random one
/// Original: `free_record` (0x53a5b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a5b0 } (Database.cpp): o registro livre (+0 zero) da tabela 0xae23f8 (60 bytes,
/// 0xae23a0 registros), guardado em 0x9b59b4 na primeira vez; sem nenhum, um sorteado.
pub(crate) unsafe extern "C" fn free_record() -> usize {
    let base = || read_usize(G_COMMON_NAME_TABLE);
    let mut i = read_i32(G_EMPTY_COMMON_NAME_INDEX);
    if i == -1 {
        let n = read_i32(G_COMMON_NAME_COUNT);
        match (0..n.max(0)).find(|&k| read_u8(base().wrapping_add(k as usize * 60)) == 0) {
            Some(k) => {
                write_i32(G_EMPTY_COMMON_NAME_INDEX, k);
                return base().wrapping_add(k as usize * 60);
            }
            None => i = random(n),
        }
    }
    base().wrapping_add((i as usize).wrapping_mul(60))
}

/// Pairs of globals with the ids of clubs A and B (Spanish B teams and others), in the order of the original's tests
/// (`exe::club_get_linked_b_team`); extracted from the disassembly.
///
/// [pt] Pares de globais com os ids dos clubes A e B (times B da Espanha e outros), na ordem dos testes
/// do original (0x540a50); extraidos da desmontagem.
pub(crate) const A_B_CLUB_PAIRS: [(usize, usize); 82] = [
    (0x9d012c, 0x9d0130), (0x9d0120, 0x9d0134), (0x9d0138, 0x9d013c), (0x9d0140, 0x9d0144), (0x9d0148, 0x9d014c), (0x9d0150, 0x9d0154),
    (0x9d0158, 0x9d015c), (0x9d0160, 0x9d0164), (0x9d0168, 0x9d016c), (0x9d0170, 0x9d0174), (0x9d0178, 0x9d017c), (0x9d0180, 0x9d0184),
    (0x9d0188, 0x9d018c), (0x9d0190, 0x9d0194), (0x9d0198, 0x9d019c), (G_KEY_CLUB_REAL_SOCIEDAD_C_F, G_KEY_CLUB_REAL_SOCIEDAD_C_F_B), (0x9d0254, 0x9d0258), (0x9d02e4, 0x9d0328),
    (0x9d03ac, 0x9d03b0), (0x9d03b4, 0x9d03b8), (0x9d03bc, 0x9d03c0), (0x9d03c4, 0x9d03c8), (0x9d0124, 0x9d03d4), (0x9d03d8, 0x9d03dc),
    (0x9d03e0, 0x9d03e4), (0x9d03e8, 0x9d03ec), (0x9d03f0, 0x9d03f4), (0x9d03f8, 0x9d03fc), (0x9d0400, 0x9d0404), (0x9d0408, 0x9d040c),
    (0x9d0410, 0x9d0414), (0x9d0418, 0x9d041c), (0xb63cdc, 0xb63c9c), (0xb63cbc, 0xb63ca0), (0xb63ce8, 0xb63cac), (0xb63cd8, 0xb63cd0),
    (0xb63cb4, 0xb63cb0), (0xb63cd4, 0xb63ca8), (0xb63ce4, 0xb63cec), (0xb63cf0, 0xb63cf8), (0xb63cc0, 0xb63d00), (0xb63cc8, 0xb63ccc),
    (0xb63ce0, 0xb63cb8), (0xb63cc4, 0xb63cf4), (0xb63cfc, 0xb63ca4), (0x9d02a4, 0x9d02a8), (0x9d02ac, 0x9d02b0), (0x9d02b4, 0x9d02b8),
    (0x9d02bc, 0x9d02c0), (0x9d02c4, 0x9d02c8), (0x9d02cc, 0x9d02d0), (0x9d02d4, 0x9d02d8), (0x9d02dc, 0x9d02e0), (0x9d0440, 0x9d0444),
    (0x9d0448, 0x9d044c), (0x9d0450, 0x9d0454), (0x9d0458, 0x9d045c), (0x9d0460, 0x9d0464), (0x9d0468, 0x9d046c), (0x9d0470, 0x9d0474),
    (0x9d0478, 0x9d047c), (0x9d0480, 0x9d0484), (0x9d0488, 0x9d048c), (0x9d0490, 0x9d0494), (0x9d0498, 0x9d049c), (0x9d04a0, 0x9d04a4),
    (0x9d04a8, 0x9d04ac), (0x9d04b0, 0x9d04b4), (0x9d04b8, 0x9d04bc), (0x9d04c0, 0x9d04c4), (0x9d04c8, 0x9d04cc), (0x9d04d0, 0x9d04d4),
    (0x9d04d8, 0x9d04dc), (0x9d04e0, 0x9d04e4), (0x9d04e8, 0x9d04ec), (0x9d04f0, 0x9d04f4), (0x9d04f8, 0x9d04fc), (0x9d0500, 0x9d0504),
    (0x9d0508, 0x9d050c), (0x9d0510, 0x9d0514), (0x9d0518, 0x9d051c), (0x9d0520, 0x9d0524),
];
pub(crate) const CLUB_PAIRS_B_A: [(usize, usize); 82] = [
    (0x9d0130, 0x9d012c), (0x9d0134, 0x9d0120), (0x9d013c, 0x9d0138), (0x9d0144, 0x9d0140), (0x9d014c, 0x9d0148), (0x9d0154, 0x9d0150),
    (0x9d015c, 0x9d0158), (0x9d0164, 0x9d0160), (0x9d016c, 0x9d0168), (0x9d0174, 0x9d0170), (0x9d017c, 0x9d0178), (0x9d0184, 0x9d0180),
    (0x9d018c, 0x9d0188), (0x9d0194, 0x9d0190), (0x9d019c, 0x9d0198), (G_KEY_CLUB_REAL_SOCIEDAD_C_F_B, G_KEY_CLUB_REAL_SOCIEDAD_C_F), (0x9d0258, 0x9d0254), (0x9d0328, 0x9d02e4),
    (0x9d03b0, 0x9d03ac), (0x9d03b8, 0x9d03b4), (0x9d03c0, 0x9d03bc), (0x9d03c8, 0x9d03c4), (0x9d03d4, 0x9d0124), (0x9d03dc, 0x9d03d8),
    (0x9d03e4, 0x9d03e0), (0x9d03ec, 0x9d03e8), (0x9d03f4, 0x9d03f0), (0x9d03fc, 0x9d03f8), (0x9d0404, 0x9d0400), (0x9d040c, 0x9d0408),
    (0x9d0414, 0x9d0410), (0x9d041c, 0x9d0418), (0xb63c9c, 0xb63cdc), (0xb63ca0, 0xb63cbc), (0xb63cac, 0xb63ce8), (0xb63cd0, 0xb63cd8),
    (0xb63cb0, 0xb63cb4), (0xb63ca8, 0xb63cd4), (0xb63cec, 0xb63ce4), (0xb63cf8, 0xb63cf0), (0xb63d00, 0xb63cc0), (0xb63ccc, 0xb63cc8),
    (0xb63cb8, 0xb63ce0), (0xb63cf4, 0xb63cc4), (0xb63ca4, 0xb63cfc), (0x9d02a8, 0x9d02a4), (0x9d02b0, 0x9d02ac), (0x9d02b8, 0x9d02b4),
    (0x9d02c0, 0x9d02bc), (0x9d02c8, 0x9d02c4), (0x9d02d0, 0x9d02cc), (0x9d02d8, 0x9d02d4), (0x9d02e0, 0x9d02dc), (0x9d0444, 0x9d0440),
    (0x9d044c, 0x9d0448), (0x9d0454, 0x9d0450), (0x9d045c, 0x9d0458), (0x9d0464, 0x9d0460), (0x9d046c, 0x9d0468), (0x9d0474, 0x9d0470),
    (0x9d047c, 0x9d0478), (0x9d0484, 0x9d0480), (0x9d048c, 0x9d0488), (0x9d0494, 0x9d0490), (0x9d049c, 0x9d0498), (0x9d04a4, 0x9d04a0),
    (0x9d04ac, 0x9d04a8), (0x9d04b4, 0x9d04b0), (0x9d04bc, 0x9d04b8), (0x9d04c4, 0x9d04c0), (0x9d04cc, 0x9d04c8), (0x9d04d4, 0x9d04d0),
    (0x9d04dc, 0x9d04d8), (0x9d04e4, 0x9d04e0), (0x9d04ec, 0x9d04e8), (0x9d04f4, 0x9d04f0), (0x9d04fc, 0x9d04f8), (0x9d0504, 0x9d0500),
    (0x9d050c, 0x9d0508), (0x9d0514, 0x9d0510), (0x9d051c, 0x9d0518), (0x9d0524, 0x9d0520),
];

/// Returns the linked club of `clube` (A team <-> B team), only for clubs of Spain, Germany or Portugal (and, with
/// `ativo`, only if the club's linked-club flag +0x244 is set). For an A club sets `*eh_a` = 1 and returns its B club;
/// for a B club sets `*eh_a` = 0 and returns its A club; a missing partner clears +0x244 and returns 0. Null club:
/// falls back to the original.
///
/// [pt] { 0x540a50 } (Database.cpp): o clube parceiro (time A <-> time B). So para clubes de nacao com
/// id em 0x9cf488, 0x9cf314 ou 0x9cf434 (e, com `ativo`, com a marca +0x244). Clube A: `eh_a` = 1 e
/// o parceiro e o B do primeiro par que casa (aceito se o id dele for diferente de -1); clube B:
/// `eh_a` = 0 e o parceiro e o A (aceito se o id for >= 0). Parceiro recusado: zera +0x244 e 0.
/// Devolve o registro do parceiro (vetor 0xae23b4, 581 bytes cada). Clube nulo: a original.
pub(crate) unsafe extern "C" fn partner_club(club: usize, eh_a: *mut u32, active: u32) -> usize {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(IDX_PARTNER_CLUB)(club.addr(), eh_a, active);
    }
    let nation = club.nation().addr();
    if nation == 0 || (active != 0 && (club.has_linked_club() as i8) == 0) {
        return 0;
    }
    let g = |a: usize| read_u32(a);
    let n = read_u32(nation);
    if n != g(KEY_NATION_SPAIN) && n != g(KEY_NATION_GERMANY) && n != g(KEY_NATION_PORTUGAL) {
        return 0;
    }
    let id = read_u32(club.addr());
    let partner = if let Some(&(_, b)) = A_B_CLUB_PAIRS.iter().find(|&&(a, _)| g(a) == id) {
        if !eh_a.is_null() {
            *eh_a = 1;
        }
        let v = g(b) as i32;
        (v != -1).then_some(v)
    } else if let Some(&(_, a)) = CLUB_PAIRS_B_A.iter().find(|&&(b, _)| g(b) == id) {
        if !eh_a.is_null() {
            *eh_a = 0;
        }
        let v = g(a) as i32;
        (v > -1).then_some(v)
    } else {
        return 0;
    };
    match partner {
        Some(v) => read_usize(G_CLUB_TABLE).wrapping_add((v as usize).wrapping_mul(581)),
        None => {
            club.set_has_linked_club(0);
            0
        }
    }
}

/// Match engine: asks the engine's referee (+0xf5b2) to judge a foul of player `j` on `outro` (MatchOfficial foul
/// decision `exe::match_official_judge_tackle`), passing the foul type, who committed it and the out-parameters for card, severity and advantage.
/// Returns the referee's decision.
///
/// [pt] { 0x6cfc60 } (match_eng.cpp): a decisao da falta (0x6e9820) com o arbitro do motor (+0xf5b2).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn foul_in_engine(
    m: usize, j: usize, other: usize, kind: i8, who: i8, card: *mut u8, grav: *mut i8, advantage: *mut u8,
) -> u8 {
    let m = Engine(m);
    exe::match_official_judge_tackle(m.addr() + engine::REFEREE, j, other, kind, who, card, grav, advantage)
}

/// Returns holiday option `i` (byte +0x9b23 + i) of the human manager record for the person (or the current human
/// manager). Without a person or out of range, falls back to the original (error box).
///
/// [pt] { 0x5fea50 } (human_manager.cpp): byte +0x9b23 + `i` do registro do tecnico humano da pessoa (ou
/// do atual). Sem pessoa ou fora da faixa: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn manager_9b23(this: usize, i: i8, person: usize) -> u8 {
    match person_and_human_record(this, person) {
        Some((_, r)) => *((r as isize + 0x9b23 + i as isize) as *const u8),
        None => original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(IDX_MANAGER_9B23)(this, i, person),
    }
}

/// Is the club among the person's favourite clubs (preference slots +0x04, +0x08, +0x0c)? Returns 1 or 0.
///
/// [pt] { 0x547d60 }: `pessoa` gosta do clube (preferencias +0x04, +0x08, +0x0c)?
pub(crate) unsafe extern "C" fn likes_club(person: usize, club: usize) -> u32 {
    is_in_preferences(club, Staff(person), 4)
}

/// Is the club among the person's disliked clubs (preference slots +0x10, +0x14, +0x18)? Returns 1 or 0.
///
/// [pt] { 0x547d30 }: `pessoa` nao gosta do clube (preferencias +0x10, +0x14, +0x18)?
pub(crate) unsafe extern "C" fn dislikes_club(person: usize, club: usize) -> u32 {
    is_in_preferences(club, Staff(person), 0x10)
}

/// Club record of a nation's national team (or B team) at the end of the club table (581 bytes each), counted back
/// `nacoes_antes` nations; falls back to the original when tables or nation are missing.
///
/// [pt] Clube da selecao da nacao: as selecoes ficam no fim do vetor de clubes (0xae23b4, 581 bytes
/// cada), a principal `deslocamento` = 2 nacoes antes do fim e a B uma nacao antes. Sem nacao,
/// sem clubes ou sem 0xae23a8: o original mostra a caixa de erro (Database.cpp) e zera 0xb67a34;
/// esse caminho fica com ele.
pub(crate) unsafe fn national_team_club(nation: usize, nations_before: i32, idx: usize) -> usize {
    let base = read_usize(G_CLUB_TABLE);
    if nation == 0 || base == 0 || read_usize(G_NATION_TABLE) == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(idx)(nation);
    }
    let i = read_i32(nation)
        .wrapping_sub(nations_before.wrapping_mul(read_i32(G_NATION_COUNT)))
        .wrapping_add(read_i32(G_CLUB_COUNT));
    base.wrapping_add((i as usize).wrapping_mul(581))
}

/// Returns the club record of nation `nacao`'s main national team (stored near the end of the club table).
///
/// [pt] { 0x53b3d0 } (Database.cpp): o clube da selecao principal da nacao.
pub(crate) unsafe extern "C" fn main_national_team(nation: usize) -> usize {
    national_team_club(nation, 2, 175)
}

/// Returns the club record of the nation's B national team (the national sides sit at the end of the club table).
///
/// [pt] { 0x53b4f0 } (Database.cpp): o clube da selecao B da nacao.
pub(crate) unsafe extern "C" fn b_national_team(nation: usize) -> usize {
    national_team_club(nation, 1, 176)
}

/// Match engine: checks whether the ball is out of play; if so releases it, sets the restart (throw-in, goal kick or
/// corner, +0x8eb2) and the side taking it (+0x8eae), adds the commentary and returns 1; ball in play returns 0.
/// `lado_b` is the side that last touched it. Wrapper of `ball_out_with` with the real commentary call.
///
/// [pt] { 0x6cd490 } (match_eng.cpp): bola fora do campo (ver `ball_out_with`).
pub(crate) unsafe extern "thiscall" fn ball_out(this: usize, side_b: u8) -> u8 {
    ball_out_with(Engine(this), side_b, &mut |t, a| add_match_event(t, a))
}

/// Returns 1 if the club belongs to the second set of national teams (id at least club count minus nation count)
/// [inference: the under-21/B national sides]. A null club goes to the original routine, whose error path changes the
/// career.
///
/// [pt] { 0x5409b0 } (Database.cpp): o clube e do segundo conjunto de selecoes (id a partir de total de
/// clubes 0xae235c menos nacoes 0xae2350)? Clube nulo: o original mostra uma caixa de erro e zera
/// 0xb67a34 (aqui, sem a caixa).
pub(crate) unsafe extern "C" fn is_second_national_team(club: usize) -> u32 {
    if club == 0 {
        // the original’s error path (message and box) has effects that change the career: the game
        // real one calls with a null club. Uses the original itself in that case.
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_SECOND_NATIONAL_TEAM)(0);
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT));
    (read_i32(club) >= limit) as u32
}

/// Aggregate score of a two-legged tie for side `lado_b` (`this` = match engine): writes into `casa`/`fora` this
/// match's goals (+0xf5bd, +0xf5f3) plus the first-leg goals from the fixture (+0x4b/+0x4c, 0xff = no first leg, sets
/// `ida`), sets `fase3` when the round variant is 3, and on a level aggregate (with optional `bonus` goal) decides on
/// away goals (`classificado`). Returns 1 if qualified on away goals, else home minus away (byte).
///
/// [pt] { 0x6ce310 } (match_eng.cpp): placar agregado de um mata-mata. `casa`/`fora` recebem os gols
/// desta partida (0xf5bd do lado 0, 0xf5f3 do lado 1; trocados com `lado` = 1), somados aos do jogo
/// de ida guardados no objeto da partida (+0x4b, +0x4c; 0xff = sem ida, e `ida` fica 1 com ida).
/// `fase3`: o algarismo das unidades de +0x30 e 3. Com o agregado empatado (mais 1 de bonus para o
/// lado de `bonus`), sem o bit 4 de +0x38 e (com o bit 0x4000) so depois do tempo 0x3de: decide pelos
/// gols fora (`classificado`). Devolve 1 classificado, senao casa - fora (byte).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn aggregate_score(this: usize, side_b: i8, home: *mut i8, away: *mut i8, qualified: *mut u8, first_leg: *mut u8, fase3: *mut u8, bonus: u8) -> u8 {
    let this = Engine(this);
    *home = 0;
    *away = 0;
    *qualified = 0;
    *first_leg = 0;
    *fase3 = 0;
    let (este, outro) = if side_b == 0 { (home, away) } else { (away, home) };
    let (bonus_este, bonus_outro): (i8, i8) = match (side_b == 0, bonus != 0) {
        (true, true) => (1, 0),
        (false, true) => (0, 1),
        _ => (0, 0),
    };
    *este = this.score();
    *outro = this.score_side1();
    let fixture = Fixture(this.r#match());
    if exe::comp_util_get_round_variant(fixture.round_name() as i32) == 3 {
        *fase3 = 1;
    }
    let first_leg_home = fixture.home_aggregate();
    if first_leg_home != -1 {
        *este = (*este).wrapping_add(first_leg_home);
        *outro = (*outro).wrapping_add(fixture.away_aggregate());
        *first_leg = 1;
        if *este as i32 + bonus_este as i32 == *outro as i32 + bonus_outro as i32 {
            let f = fixture.tie_rules();
            let team_ok = f & 0x4000 == 0 || this.period_end() > 0x3de;
            if f & 4 == 0 && team_ok {
                let goals_away = (this.score_side1() as u8) as i32;
                if side_b == 0 {
                    if goals_away + (bonus_este as i32) < first_leg_home as i32 {
                        *qualified = 1;
                    }
                } else if goals_away + (bonus_outro as i32) > first_leg_home as i32 {
                    *qualified = 1;
                }
            }
        }
    }
    if *qualified != 0 {
        1
    } else {
        (*home as u8).wrapping_sub(*away as u8)
    }
}

/// 1 if `alvo` is one of the three people at offset `base` of the person's STAFF_PREFERENCES record, else 0 (0 on any
/// null pointer).
///
/// [pt] Preferencias (`CM3_STAFF_PREFERENCES`, ponteiro em +0x65 da pessoa): `alvo` esta entre as tres
/// pessoas em `base`, +4, +8? (0 com qualquer ponteiro nulo.)
pub(crate) unsafe fn is_in_preferences(target: usize, person: Staff, base: usize) -> u32 {
    if target == 0 || person.is_null() {
        return 0;
    }
    let pref = person.staff_preferences();
    if pref == 0 {
        return 0;
    }
    (0..3).any(|k| read_usize(pref + base + 4 * k) == target) as u32
}

/// Does `pessoa` like `alvo`? True if `alvo` is one of the three favoured people in the person's preferences record
/// (+0x1c, +0x20, +0x24). Returns 1 or 0.
///
/// [pt] { 0x547810 }: `pessoa` gosta de `alvo` (preferencias +0x1c, +0x20, +0x24)?
pub(crate) unsafe extern "C" fn likes(target: usize, person: usize) -> u32 {
    is_in_preferences(target, Staff(person), 0x1c)
}

/// Does `pessoa` dislike `alvo`? True if `alvo` is one of the three disliked people in the person's preferences record
/// (+0x28, +0x2c, +0x30). Returns 1 or 0.
///
/// [pt] { 0x547840 }: `pessoa` nao gosta de `alvo` (preferencias +0x28, +0x2c, +0x30)?
pub(crate) unsafe extern "C" fn dislikes(target: usize, person: usize) -> u32 {
    is_in_preferences(target, Staff(person), 0x28)
}

/// Formation: stores the value pointed to by `ficha` (or -1 with none) in the per-position table (+0x5bf + 4 x
/// position) and clears the changed flag (+0x187f). Returns 1.
///
/// [pt] { 0x5b5910 }: grava na tabela por posicao (+0x5bf + 4 x posicao) o valor apontado por `ficha`
/// (-1 sem ficha) e zera +0x187f. Devolve 1.
pub(crate) unsafe extern "thiscall" fn mark_position(this: usize, position: i8, sheet: *const u32) -> u32 {
    let target = (this as isize + 0x5bf + 4 * position as isize) as *mut u32;
    *target = if sheet.is_null() { u32::MAX } else { *sheet };
    write_u32(this + 0x187f, 0);
    1
}

/// Reads the signed 16-bit value of cell k of a 3-byte-per-cell attribute vector (as built by `database_player_initialisation`).
///
/// [pt] Uma célula (3 bytes: palavra com sinal + peso) dos vetores de atributos da 0x535a80.
pub(crate) fn attr_cell_value(v: &[u8; 189], k: isize) -> i16 {
    let i = (k * 3) as usize;
    i16::from_le_bytes([v[i], v[i + 1]])
}

/// Writes the signed 16-bit value of cell k of an attribute vector.
pub(crate) fn set_attr_cell_value(v: &mut [u8; 189], k: isize, x: i16) {
    let i = (k * 3) as usize;
    v[i..i + 2].copy_from_slice(&x.to_le_bytes());
}

/// Reads the weight byte of cell k of an attribute vector.
pub(crate) fn attr_cell_weight(v: &[u8; 189], k: isize) -> u8 {
    v[(k * 3) as usize + 2]
}

/// Weight that triggers a re-roll: 2 when cell 0 (goalkeeper) is at least 15, else 1.
///
/// [pt] O peso que manda sortear de novo: 2 com a célula 0 (goleiro) a partir de 15, senão 1.
pub(crate) fn reroll_weight(v: &[u8; 189]) -> u8 {
    if attr_cell_value(v, 0) >= 0xf { 2 } else { 1 }
}

/// Fills the 63 attribute-vector cells with terminators (value 0xff81, weight 0x81) as `database_player_initialisation` prepares them.
///
/// [pt] Os 63 terminadores (palavra 0xff81, peso 0x81), como a 0x535a80 prepara os vetores.
pub(crate) unsafe fn clear_attr_vector(v: usize) {
    for k in 0..63 {
        write_u16(v + 3 * k, 0xff81);
        write_u8(v + 3 * k + 2, 0x81);
    }
}

/// Age of a person at a date as `database_player_initialisation` computes it: date year byte minus birth year byte (+0x12), minus 1 if the
/// birth day-of-year (+0x10) is later.
///
/// [pt] A idade da 0x535a80: byte do ano da data − byte do ano de nascimento (+0x12), menos 1 se o dia do ano de
/// nascimento (+0x10) passa o da data.
pub(crate) unsafe fn age_at_date(p: Staff, data: usize) -> i8 {
    let a = read_u8(data + 2).wrapping_sub((p.date_of_birth_year() as u8));
    let d = p.date_of_birth() as i32 - read_i16(data) as i32;
    (if d > 0 { a.wrapping_sub(1) } else { a }) as i8
}

/// Free-agent (no club) reduction used by `database_player_initialisation`: x - x*0.25 when byte +0x22 is set, else x - x*0.6, truncated
/// [inference: constants at 0x9689e0/0x968e48].
///
/// [pt] O ajuste do livre (sem clube) da 0x535a80: x − x × 0,25 (com +0x22) ou x − x × 0,6, truncado.
pub(crate) unsafe fn free_agent_adjust(p: Staff, x: i16) -> i16 {
    let x = x as f64;
    let k = if p.int_apps() != 0 { 0x9689e0 } else { 0x968e48 };
    ftol(x - x * read_f64(k)) as i16
}

/// True if a <= x <= b.
pub(crate) fn in_range(x: i16, a: i16, b: i16) -> bool {
    x >= a && x <= b
}

/// `database_player_initialisation(data)`: prepares every person of the new game. Called once at creation (create_game
/// `exe::create_game`), with the "Player Initialisation" bar (0x66f4e0, `exe::screenmanager_send_network_status_message` at every 1% of the people).
///
/// For each person: common name (+0xc = −2: the row stored in [0x9b59b4], the first empty one or a random one), first
/// and second name (−2: `database_pick_first_name_for_nation`, `database_pick_second_name_for_nation`), the dates (+0x3e joined club, +0x46, +0x31), the 8 person attributes
/// (+0x56..+0x5d, outside 1..20: drawn at random), the adjustment by the nation's region (+0x76), the missing nation
/// (national team, club or drawn).
///
/// For the player (+0x61): the abilities (+0xb, +9, +0xd, in that order, each from the one that already exists), the
/// potential (+7: −2/−1/0 rarely, by draws of 2000/750/100) and the current one (+5), the age (birth drawn when outside
/// 14..50), the positions and sides in the vector (`database_player_to_attribute_vector`), and the attributes outside 1..20 from a template: the
/// most similar player among 250 drawn at random (global vectors 0xae2044/0xae2114, flag 0x9b59b8) or, without a
/// template, by ability.
///
/// For the non-player (+0x69): the reputations (×50, +0xc/+0xa/+8), +4/+6, +0x13, the template among 250
/// (0xae18bc/0xae2238, 0x9b59b9), the job (+0x3d 8/9/10), the famous names (`exe::staff_name_equals`: +0x43) and the missing job. At
/// the end of each person, the age (+0x18).
///
/// Then the second pass: the ability-relative vectors (×10 − ability, ±200, /2) and the normalisations (`staff_normalise_player_physical_attributes`,
/// `staff_normalise_player_attributes`, `staff_normalise_non_player_attributes`).
///
/// The stack vectors (0x54, 0x290) are only cleared on entry: what one person leaves carries over to the next, as in
/// the original; the best score ([esp+0x40], −31073 on entry) too. No people: the error box and the end, through the
/// original. The local at [esp+0x114] (1, zeroed with a club of reputation 0) is never read.
/// Original: `database_player_initialisation` (0x535a80, Database.cpp, cdecl).
///
/// [pt] { 0x535a80 } (cdecl, Database.cpp): `database_player_initialisation(data)`, uma vez na criação (create_game
/// 0x828110), com a barra "Player Initialisation" (0x66f4e0, 0x802f80 a cada 1% das pessoas). Para cada pessoa: nome
/// comum (+0xc = −2: a linha guardada em [0x9b59b4], a primeira vazia ou uma sorteada), primeiro e segundo nome (−2:
/// 0x539c60, 0x53a1b0), as datas (+0x3e entrada no clube, +0x46, +0x31), os 8 atributos de pessoa (+0x56..+0x5d, fora
/// de 1..20: sorteados), o ajuste pela região da nação (+0x76), a nação que falta (seleção, clube ou sorteada). Do
/// jogador (+0x61): as habilidades (+0xb, +9, +0xd, nesta ordem, cada uma da que já existe), o potencial (+7: −2/−1/0
/// raros pelos sorteios 2000/750/100) e a atual (+5), a idade (nascimento sorteado fora de 14..50), as posições e os
/// lados no vetor (0x53a610), e os atributos fora de 1..20 por um molde: o jogador mais parecido entre 250 sorteados
/// (vetores globais 0xae2044/0xae2114, bandeira 0x9b59b8) ou, sem molde, pela habilidade. Do não jogador (+0x69): as
/// reputações (×50, +0xc/+0xa/+8), +4/+6, +0x13, o molde entre 250 (0xae18bc/0xae2238, 0x9b59b9), o cargo (+0x3d 8/9/10),
/// os nomes famosos (0x84b8b0: +0x43) e o cargo que falta. No fim de cada pessoa a idade (+0x18). Depois, a segunda
/// volta: os vetores relativos à habilidade (×10 − habilidade, ±200, /2) e as normalizações (0x538ec0, 0x538bc0,
/// 0x539490). Os vetores da pilha (0x54, 0x290) só são limpos na entrada: o que uma pessoa deixa passa à seguinte, como
/// na original; o melhor escore ([esp+0x40], −31073 na entrada) também. Sem pessoas: a caixa e o fim, pela original.
/// A local de [esp+0x114] (1, zerada com clube de reputação 0) nunca é lida.
pub(crate) unsafe extern "C" fn database_player_initialisation(data: usize) {
    if read_i32(G_STAFF_COUNT) == 0 || read_usize(G_STAFF_TABLE) == 0 {
        return original_fn::<unsafe extern "C" fn(usize)>(IDX_DATABASE_PLAYER_INITIALISATION)(data);
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let w = |a: usize| read_i16(a);
    let signed_byte_at = |a: usize| read_i8(a);
    let b = |a: usize| read_u8(a);
    let wd = |a: usize, v: usize| write_usize(a, v);
    let ww = |a: usize, v: i16| write_i16(a, v);
    let wb = |a: usize, v: u8| write_u8(a, v);
    // the random roll as a byte (al) and as a word (ax)
    let s8 = |n: i32| exe::utils_random(n) as i8;
    let s16 = |n: i32| exe::utils_random(n) as i16;
    let mut lixo = [[0u32; 2]; 12]; // the stack-passed dates (nobody reads them afterwards)
    let l = |k: usize, t: &mut [[u32; 2]; 12]| t[k].as_mut_ptr() as usize;

    let cent = gi(G_STAFF_COUNT) / 100;
    text(0xde1f64, 0x9b6414);
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, 0xde1f64, 0, 100, -1);
    let mut v1 = [0u8; 189];
    let mut v2 = [0u8; 189];
    clear_attr_vector(v1.as_mut_ptr() as usize);
    clear_attr_vector(v2.as_mut_ptr() as usize);
    let (mut barra, mut e0c, mut e10) = (0i8, 0i32, -0x7961i32);
    let (mut e19, mut e05, mut e11, mut e1a) = (0u8, 0u8, 0u8, 0u8);

    let mut i = 0i32;
    let mut p = rd(G_STAFF_TABLE);
    while i < gi(G_STAFF_COUNT) {
        // o nome comum
        if read_i32(p + staff::COMMON_NAME) == -2 {
            let mut k = gi(G_EMPTY_COMMON_NAME_INDEX);
            if k == -1 {
                let n = gi(G_COMMON_NAME_COUNT);
                let base = rd(G_COMMON_NAME_TABLE);
                let mut j = 0i32;
                let mut found = false;
                while j < n {
                    if b(base.wrapping_add((j as usize).wrapping_mul(0x3c))) == 0 {
                        write_i32(G_EMPTY_COMMON_NAME_INDEX, j);
                        found = true;
                        break;
                    }
                    j += 1;
                }
                k = if found { j } else { exe::utils_random(n) };
            }
            wd(p + staff::COMMON_NAME, rd(G_COMMON_NAME_TABLE).wrapping_add((k as usize).wrapping_mul(0x3c)));
        }
        if read_i32(p + staff::FIRST_NAME) == -2 {
            wd(p + staff::FIRST_NAME, exe::database_pick_first_name_for_nation(read_usize(p + staff::NATION)));
        }
        if read_i32(p + staff::SECOND_NAME) == -2 {
            wd(p + staff::SECOND_NAME, exe::database_pick_second_name_for_nation(read_usize(p + staff::NATION)));
        }
        // the joining of the club (+0x3e)
        if read_usize(p + staff::CLUB_JOB) == 0 {
            exe::date_operator_3d(p + staff::DATE_JOINED_CLUB, l(0, &mut lixo), data);
        }
        let year = w(p + 0x40) as u16;
        if year < 0x7b2 {
            if read_usize(p + staff::CLUB_JOB) != 0 {
                exe::date_set(p + staff::DATE_JOINED_CLUB, 1, 0, 0x7cc, -1);
            } else {
                ww(p + staff::DATE_JOINED_CLUB, w(data));
                ww(p + 0x40, w(data + 2));
            }
        } else if year == 0x7cc && read_i16(p + staff::DATE_JOINED_CLUB) == 1 && read_usize(p + staff::CLUB_JOB) != 0 {
            let nation = rd(read_usize(p + staff::CLUB_JOB) + 0x53);
            if nation != 0 && gi(read_usize(nation + nation::CONTINENT)) == gi(KEY_CONTINENT_EUROPE) && exe::database_is_staff_eu_national(p, nation) == 0 {
                let a = w(data + 2);
                let r = exe::utils_random(2);
                let d = exe::new_date(l(1, &mut lixo), 1, (a as i32 - r - 1) as u32);
                exe::date_operator_3d(p + staff::DATE_JOINED_CLUB, l(2, &mut lixo), d);
            }
        }
        if read_usize(p + staff::CLUB_JOB) != 0 && rd(read_usize(p + staff::CLUB_JOB) + 0x53) == 0 {
            wd(p + staff::CLUB_JOB, 0);
        }
        let d = exe::date_sub_weeks(G_GAME_START_DATE, (l(3, &mut lixo)) as *mut [u8; 8], 5);
        let (dd, da, pa) = (w(d), w(d + 2) as u16, w(p + 0x40) as u16);
        if da < pa || (da == pa && dd < read_i16(p + staff::DATE_JOINED_CLUB)) {
            let d = exe::date_sub_weeks(G_GAME_START_DATE, (l(4, &mut lixo)) as *mut [u8; 8], 5);
            exe::date_operator_3d(p + staff::DATE_JOINED_CLUB, l(5, &mut lixo), d);
        }
        for &(o, k) in &[(0x46usize, 6usize), (0x31, 8)] {
            let a = w(p + o + 2) as u16;
            if a < 0x7cf || a > 0x7da {
                let d = exe::new_date(l(k, &mut lixo), 0x1f, 0x76c);
                if w(d + 2) != w(p + o + 2) || w(d) != w(p + o) {
                    let d = exe::new_date(l(k + 1, &mut lixo), 0x1f, 0x76c);
                    exe::date_operator_3d(p + o, l(10, &mut lixo), d);
                }
            }
        }
        // the bar, every 1%
        if e0c < cent {
            e0c += 1;
        } else {
            e0c = 0;
            barra = barra.wrapping_add(1);
            text(0xde1f64, 0x9b6414);
            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, 0xde1f64, barra as i32, 100, -1);
        }
        // person attributes: −1 draws 5..20; outside 1..20, the smaller (min) or the average of two draws
        for &(o, min) in &[(0x56usize, true), (0x57, false), (0x58, false), (0x59, true), (0x5a, false), (0x5b, false), (0x5c, true)] {
            let v = signed_byte_at(p + o);
            if v < 1 || v > 20 {
                let n = if v == -1 {
                    s8(0x10).wrapping_add(5)
                } else {
                    let a = s8(0x14).wrapping_add(1);
                    let c = s8(0x14).wrapping_add(1);
                    if min {
                        if a <= c { a } else { c }
                    } else {
                        ((c as i32 + a as i32) / 2) as i8
                    }
                };
                wb(p + o, n as u8);
            }
        }
        let v = read_i8(p + staff::TEMPERAMENT);
        if v < 1 || v > 20 {
            let n = if v == -1 {
                s8(0xb).wrapping_add(10)
            } else {
                let a = s8(0xb).wrapping_add(5);
                let c = s8(0xb).wrapping_add(5);
                ((c as i32 + a as i32) / 2) as i8
            };
            wb(p + staff::TEMPERAMENT, n as u8);
        }
        // the adjustment by the nation’s region (+0x76)
        let nation = read_usize(p + staff::NATION);
        if nation != 0 {
            let minus = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_sub(exe::utils_random(n) as u8));
            let more = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_add(exe::utils_random(n) as u8));
            let t = read_i8(nation + nation::ACTUAL_REGION) as i32;
            if (t - 4) as u32 <= 9 {
                match t {
                    6 | 8 => minus(0x5c, 10),
                    4 | 0xb | 0xc | 0xd => {
                        if t != 0xb && t != 0xd {
                            minus(0x5c, 0xf);
                            minus(0x5d, 0xf);
                        }
                        if t != 0xd {
                            more(0x5c, 8);
                            more(0x5d, 8);
                        }
                        minus(0x5c, 5);
                        minus(0x5d, 5);
                    }
                    _ => {}
                }
            }
            for o in [0x5cusize, 0x5d] {
                let v = signed_byte_at(p + o);
                wb(p + o, if v < 1 { 1 } else if v > 20 { 20 } else { v as u8 });
            }
        }
        let t79 = rd(G_STAFF_STATE_TABLE).wrapping_add((i as usize).wrapping_mul(0x4f)) + 0xb;
        wb(t79, b(t79) & 0xf0);
        if read_usize(p + staff::NATION) == 0 {
            let s = read_usize(p + staff::NATIONAL_JOB);
            let c = read_usize(p + staff::CLUB_JOB);
            if s != 0 && read_usize(s + club::NATION) != 0 {
                wd(p + staff::NATION, read_usize(s + club::NATION));
            } else if c != 0 && read_usize(c + club::NATION) != 0 {
                wd(p + staff::NATION, read_usize(c + club::NATION));
            } else {
                let r = exe::utils_random(gi(G_NATION_COUNT));
                wd(p + staff::NATION, rd(G_NATION_TABLE).wrapping_add((r as usize).wrapping_mul(0x122)));
            }
        }

        let j = read_usize(p + staff::PLAYER);
        if j != 0 {
            // ---- the player: abilities (+0xb, +9, +0xd), potential (+7) and current (+5) ----
            let ca0 = read_i16(j + player_sheet::CURRENT_ABILITY);
            let mut e24 = read_i16(j + player_sheet::POTENTIAL_ABILITY);
            if exe::utils_random(2000) == 0 {
                e24 = -2;
            } else if exe::utils_random(0x2ee) == 0 {
                e24 = -1;
            } else if exe::utils_random(100) == 0 {
                e24 = 0;
            }
            let mut e34 = read_i16(j + player_sheet::WORLD_REPUTATION);
            let mut e2c = read_i16(j + player_sheet::HOME_REPUTATION);
            let mut e20 = read_i16(j + player_sheet::CURRENT_REPUTATION);
            wb(j + player_sheet::PLAYER_MORALE, 10);
            let nation = read_usize(p + staff::NATION);
            let mut rn = read_i8(nation + nation::REPUTATION);
            if rn == 0 {
                format(0xde1f64, 0x9b63f0, read_i16(nation + nation::REPUTATION) as i32);
                rn = s8(10).wrapping_add(1);
            }
            let rc: i8 = if read_usize(p + staff::CLUB_JOB) == 0 {
                s8(2).wrapping_add(1)
            } else {
                let v = signed_byte_at(read_usize(p + staff::CLUB_JOB) + 0x80);
                if v != 0 { v } else { s8(4).wrapping_add(1) }
            };
            let e30 = rc as i32;
            let di = ((((e30 * 3 + rn as i32) >> 2) * 10) as i16).min(0x50);
            if !in_range(e20, 1, 200) {
                let c = read_usize(p + staff::CLUB_JOB);
                let mut x: i16 = if in_range(e2c, 1, 200) && c != 0 && read_usize(p + staff::NATION) == read_usize(c + club::NATION) {
                    let r = s16(10);
                    let y = e2c.wrapping_sub(r).wrapping_add(s16(10));
                    if y < 1 { 1 } else if y > 200 { 200 } else { y }
                } else if in_range(e34, 1, 200) {
                    if e34 < 0xb4 { e34.wrapping_sub(s16(10)).wrapping_add(0x14) } else { s16(10).wrapping_add(0xaa) }
                } else if e24 == -1 && ca0 > 100 {
                    s16(0x28).wrapping_add(0x3c)
                } else if in_range(ca0, 1, 200) {
                    if ca0 > 0x4b { ca0.wrapping_sub(s16(0x32)) } else { s16(0x19).wrapping_add(1) }
                } else {
                    s16(di as i32).wrapping_add(1).min(0x19)
                };
                if read_usize(p + staff::CLUB_JOB) == 0 {
                    x = free_agent_adjust(Staff(p), x);
                }
                e20 = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            if !in_range(e2c, 1, 200) {
                let mut x: i16 = if in_range(e20, 1, 200) {
                    if e20 < 10 {
                        s16(5).wrapping_add(1)
                    } else if e20 > 0xbe {
                        s16(10).wrapping_add(0xb9)
                    } else {
                        let y = e20.wrapping_sub(s16(10));
                        y.wrapping_add(s16(10))
                    }
                } else if in_range(e34, 1, 200) {
                    if e34 > 0xb4 { s16(10).wrapping_add(0xb4) } else { e34.wrapping_sub(s16(10)).wrapping_add(0x14) }
                } else if e24 == -1 {
                    s16(0x14).wrapping_add(0x50)
                } else if in_range(ca0, 1, 200) {
                    if ca0 > 0x32 { ca0.wrapping_sub(s16(0x32)) } else { s16(0x19).wrapping_add(1) }
                } else {
                    s16(di as i32).wrapping_add(1).min(0x19)
                };
                if read_usize(p + staff::CLUB_JOB) == 0 {
                    x = free_agent_adjust(Staff(p), x);
                }
                e2c = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            if !in_range(e34, 1, 200) {
                let mut x: i16 = if in_range(e2c, 1, 200) {
                    if e2c > 100 { ftol(e2c as f64 * read_f64(rdata::F64_0_25)) as i16 } else { s16(3).wrapping_add(1) }
                } else if in_range(e20, 1, 200) {
                    ftol(e20 as f64 * read_f64(if e20 < 0xaf { 0x968f20 } else { 0x9689e0 })) as i16
                } else if in_range(ca0, 1, 200) {
                    let t = ftol(ca0 as f64 * read_f64(rdata::F64_0_25));
                    let r = exe::utils_random(t);
                    s16(r).wrapping_add(1)
                } else {
                    ((di as i32 / 3) as i16).min(5)
                };
                if read_usize(p + staff::CLUB_JOB) == 0 {
                    x = free_agent_adjust(Staff(p), x);
                }
                e34 = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            let c = read_usize(p + staff::CLUB_JOB);
            if c == 0 || read_usize(c + club::NATION) == read_usize(p + staff::NATION) {
                e2c = e20;
            }
            // the age (birth rolled outside 14..50)
            let mut age = age_at_date(Staff(p), data);
            if age < 0xe || age > 0x32 {
                let a18 = read_i16(p + staff::AGE);
                let year: u32 = if a18 > 0x76c { a18 as u16 as u32 } else if read_usize(p + staff::NON_PLAYER) != 0 { 0x7af } else { 0x7bc };
                let month = exe::utils_random(0xb);
                let day = exe::utils_random(0x1b) + 1;
                exe::date_set(p + staff::DATE_OF_BIRTH, day as u32, month as u32, year, -1);
                age = age_at_date(Staff(p), data);
            }
            ww(p + staff::AGE, age as i16);
            // o potencial (+7)
            let ceiling = |mut s: i16, ca0: i16| {
                if ca0 > s {
                    s = ca0;
                }
                if s < 10 { 10 } else if s > 200 { 200 } else { s }
            };
            if e24 == -2 {
                let mut s = if exe::utils_random(10) != 0 { s16(0x51).wrapping_add(0x78) } else { s16(0x29).wrapping_add(0xa0) };
                if exe::utils_random(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0xa5 {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = ceiling(s, ca0);
            } else if e24 == -1
                || (e24 > 0x8c && (age as i32) < exe::utils_random(0x1e) && ca0 < 0xaa && exe::utils_random(0x271a - e30) == 0)
            {
                let mut s = if exe::utils_random(10) != 0 { s16(0x51).wrapping_add(0x50) } else { s16(0x51).wrapping_add(0x78) };
                if exe::utils_random(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0x8c {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = ceiling(s, ca0);
            } else if !in_range(e24, 1, 200) || (age < 0x19 && exe::utils_random(0x19) == 0) {
                let mut k: i8 = if exe::utils_random(100) == 0 {
                    s8(0x15)
                } else if exe::utils_random(2) == 0 {
                    s8(6).wrapping_add(6)
                } else if exe::utils_random(2000) == 0 {
                    s8(0x14).wrapping_add(1)
                } else {
                    s8(7).wrapping_add(4)
                };
                if k < 1 || k > 20 {
                    k = s8(10).wrapping_add(3);
                }
                let r = s16(0x3c);
                let s = r.wrapping_add(w((0x9b5988isize + k as isize * 2) as usize).wrapping_sub(0x1e));
                e24 = ceiling(s, ca0);
            }
            // a atual (+5)
            let mut ca = ca0;
            if !in_range(ca0, 1, 200) {
                let k = s8(10).wrapping_add(0x14);
                let pa = e24 as i32;
                let a = s16(pa);
                let a = if a < 5 { 5 } else { a as i32 };
                let r = exe::utils_random(((pa >> 2) * a) / 200);
                let y = (10000 - e30) / 200;
                let c16 = (r + y * 2 + 3) as i16;
                let f = ((k as i32) - (age as i32)).abs().max(3);
                let v = pa - (((c16 as i32 * f) / 5) as i16 as i32);
                ca = if v > 200 { 200 } else if (v as i16) >= 1 { v as i16 } else { 1 };
                if ca == 1 && exe::utils_random(10) != 0 {
                    ca = s16(pa / 5).wrapping_add(1);
                }
            }
            if read_usize(p + staff::CLUB_JOB) == 0 && (read_u8(p + staff::INT_APPS) == 0 || age_at_date(Staff(p), data) > 0x20) {
                let r = exe::utils_random(6); // the original stores ca in [0x20] only for the fild
                let f = ftol(ca as f64 * read_f64(rdata::F64_NEG_0_1));
                ca = ca.wrapping_add(r.wrapping_mul(f) as i16);
                ca = if ca < 1 { 1 } else if ca > 200 { 200 } else { ca };
            }
            if ca > e24 {
                ca = e24.wrapping_sub(s16(0x28));
                if ca < 1 {
                    ca = s16(5).wrapping_add(1);
                }
            }
            ww(j + player_sheet::CURRENT_ABILITY, ca);
            ww(j + player_sheet::POTENTIAL_ABILITY, e24);
            ww(j + player_sheet::CURRENT_REPUTATION, e20.wrapping_mul(50));
            ww(j + player_sheet::HOME_REPUTATION, e2c.wrapping_mul(50));
            ww(j + player_sheet::WORLD_REPUTATION, e34.wrapping_mul(50));
            wb(rd(G_STAFF_STATE_TABLE).wrapping_add((gi(p) as usize).wrapping_mul(0x4f)) + 9, 0x9c);
            // ---- the positions (cells 0..8) and the sides (9..11) ----
            exe::database_player_to_attribute_vector(j, p, v1.as_mut_ptr() as usize, &mut e19, &mut e05, &mut e11);
            if attr_cell_value(&v1, 0) > 0xf && attr_cell_value(&v1, 0) < 0x14 {
                set_attr_cell_value(&mut v1, 0, 0x14);
                for k in [1isize, 2, 3, 4, 5, 6, 10, 11, 9] {
                    set_attr_cell_value(&mut v1, k, 1);
                }
            }
            let rf = || (s8(6).wrapping_add(0xf)) as i16;
            // (a, c) = the starting values of cells 5 and 4 at the end of the positions
            let mut ac: Option<(i16, i16)> = None;
            let mut best: i8 = -1;
            let mut tem20 = false;
            for k in 0..8isize {
                let a = attr_cell_value(&v1, k);
                if a == 0x14 {
                    tem20 = true;
                    break;
                }
                if a > 5 && a < 0x14 && (best == -1 || a > attr_cell_value(&v1, best as isize)) {
                    best = k as i8;
                }
            }
            if !tem20 {
                if best > -1 {
                    set_attr_cell_value(&mut v1, best as isize, 0x14);
                } else if exe::utils_random(0x14) == 0 {
                    set_attr_cell_value(&mut v1, 0, 0x14);
                    for k in [1isize, 2, 3, 5, 6, 10, 11, 9] {
                        set_attr_cell_value(&mut v1, k, 1);
                    }
                    ac = Some((1, 1));
                } else {
                    let r = s16(0xb6);
                    // each draw band: the drawn cells (15..20, in this order) and those that become 20
                    let (sorteadas, vinte): (&[isize], &[isize]) = if r < 1 {
                        (&[], &[1])
                    } else if r < 7 {
                        (&[1], &[2])
                    } else if r < 0xb {
                        (&[1, 3], &[2])
                    } else if r < 0xf {
                        (&[1, 7], &[2])
                    } else if r < 0x10 {
                        (&[1, 3, 4], &[2])
                    } else if r < 0x11 {
                        (&[1, 7, 4], &[2])
                    } else if r < 0x25 {
                        (&[], &[2])
                    } else if r < 0x31 {
                        (&[3], &[2])
                    } else if r < 0x3d {
                        (&[7], &[2])
                    } else if r < 0x49 {
                        (&[7, 4], &[2])
                    } else if r < 0x55 {
                        (&[3, 4], &[2])
                    } else if r < 0x56 {
                        (&[6], &[2])
                    } else if r < 0x65 {
                        (&[3], &[4])
                    } else if r < 0x67 {
                        (&[3, 5], &[4])
                    } else if r < 0x6a {
                        (&[7, 5], &[4])
                    } else if r < 0x7e {
                        (&[], &[4])
                    } else if r < 0x8c {
                        (&[5], &[4])
                    } else if r < 0x8e {
                        (&[6, 5], &[4])
                    } else if r < 0x8f {
                        (&[6], &[4])
                    } else if r < 0x95 {
                        (&[], &[4, 5])
                    } else if r < 0x9b {
                        (&[6], &[5])
                    } else if r < 0xa1 {
                        (&[5], &[6])
                    } else {
                        (&[], &[6])
                    };
                    for &k in vinte {
                        set_attr_cell_value(&mut v1, k, 0x14);
                    }
                    for &k in sorteadas {
                        let x = rf();
                        set_attr_cell_value(&mut v1, k, x);
                    }
                }
            }
            if ac.is_none() {
                // the sides: right (9), left (10), center (11)
                if attr_cell_value(&v1, 0) != 0x14 {
                    let mut sides = true;
                    if attr_cell_value(&v1, 1) >= 0xf {
                        set_attr_cell_value(&mut v1, 11, 0x14);
                    } else if attr_cell_value(&v1, 7) >= 0xf && attr_cell_value(&v1, 9) < 0xf && attr_cell_value(&v1, 10) < 0xf {
                        if exe::utils_random(10) >= 3 {
                            set_attr_cell_value(&mut v1, 9, 0x14);
                            sides = false;
                        } else {
                            set_attr_cell_value(&mut v1, 10, 0x14);
                        }
                    }
                    while sides {
                        if attr_cell_value(&v1, 9) >= 0x14 || attr_cell_value(&v1, 10) >= 0x14 || attr_cell_value(&v1, 11) >= 0x14 {
                            break;
                        }
                        if exe::utils_random(0xb) < 4 || exe::utils_random(attr_cell_value(&v1, 11) as i32) > 5 {
                            set_attr_cell_value(&mut v1, 11, 0x14);
                        }
                        if exe::utils_random(0xb) > 8 || exe::utils_random(attr_cell_value(&v1, 10) as i32) > 5 {
                            set_attr_cell_value(&mut v1, 10, 0x14);
                        }
                        if exe::utils_random(0xb) < 4 || exe::utils_random(attr_cell_value(&v1, 9) as i32) > 5 {
                            set_attr_cell_value(&mut v1, 9, 0x14);
                            break;
                        }
                    }
                }
                let a = attr_cell_value(&v1, 5);
                let mut c = attr_cell_value(&v1, 4);
                if a >= 0xf && c < 0xf && attr_cell_value(&v1, 11) < 0xf {
                    c = a;
                }
                ac = Some((a, c));
            }
            let (a, c) = ac.unwrap();
            let m = (a as i32 / 2).max(c as i32);
            set_attr_cell_value(&mut v1, 4, m as i8 as i16);
            let m = (attr_cell_value(&v1, 1) as i32 / 2).max(attr_cell_value(&v1, 2) as i32);
            set_attr_cell_value(&mut v1, 2, m as i8 as i16);
            if attr_cell_value(&v1, 6) >= 0xf && attr_cell_value(&v1, 11) < 0xf {
                let x = s8(0xb).wrapping_add(10);
                let y = s8(7).wrapping_add(0xe) as i16;
                let v5 = attr_cell_value(&v1, 5);
                set_attr_cell_value(&mut v1, 5, (if y > v5 { y } else { v5 }) as i8 as i16);
                let v4 = attr_cell_value(&v1, 4);
                set_attr_cell_value(&mut v1, 4, (if x as i16 > v4 { x as i16 } else { v4 }) as i8 as i16);
            }
            // cells 12 and 13
            let one_twenty = || (s8(exe::utils_random(0x14)).wrapping_add(1)) as i16;
            let mut a = attr_cell_value(&v1, 12);
            let mut c = attr_cell_value(&v1, 13);
            let mut check_c = true;
            if a == 0x14 || c == 0x14 {
            } else if in_range(a, 1, 20) {
                if in_range(c, 1, 20) && c <= a {
                    a = 0x14;
                    set_attr_cell_value(&mut v1, 12, a);
                } else {
                    c = 0x14;
                    set_attr_cell_value(&mut v1, 13, c);
                    check_c = false;
                }
            } else if in_range(c, 1, 20) {
                a = 0x14;
                set_attr_cell_value(&mut v1, 12, a);
            } else if attr_cell_value(&v1, 10) > attr_cell_value(&v1, 9) {
                set_attr_cell_value(&mut v1, 12, 0x14);
                c = one_twenty();
                a = attr_cell_value(&v1, 12);
                set_attr_cell_value(&mut v1, 13, c);
            } else {
                set_attr_cell_value(&mut v1, 13, 0x14);
                let n = one_twenty();
                c = attr_cell_value(&v1, 13);
                set_attr_cell_value(&mut v1, 12, n);
                a = n;
            }
            if check_c && !in_range(c, 1, 20) {
                c = one_twenty();
                a = attr_cell_value(&v1, 12);
                set_attr_cell_value(&mut v1, 13, c);
            }
            if !in_range(a, 1, 20) {
                let n = one_twenty();
                c = attr_cell_value(&v1, 13);
                set_attr_cell_value(&mut v1, 12, n);
                a = n;
            }
            if a != 0x14 && c != 0x14 {
                if exe::utils_random(4) != 0 {
                    a = attr_cell_value(&v1, 12);
                    set_attr_cell_value(&mut v1, 13, 0x14);
                } else {
                    a = 0x14;
                    set_attr_cell_value(&mut v1, 12, a);
                }
            }
            // ---- attributes outside 1..20 (from cell 12 on): by the template ----
            let mut away = false;
            if a != -0x7f {
                let mut k = 12isize;
                loop {
                    let x = attr_cell_value(&v1, k);
                    if x < 1 || x > 20 {
                        away = true;
                        break;
                    }
                    k += 1;
                    if attr_cell_value(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if away {
                let mut molde = 0usize;
                let mut count = 0xfai32;
                loop {
                    let r = exe::utils_random(gi(G_STAFF_COUNT));
                    let q = rd(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(110));
                    let cj = read_usize(q + staff::PLAYER);
                    if cj != 0 && in_range(w(cj + 5), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b8) != 0 {
                            clear_attr_vector(G_PLAYER_INIT_ATTRIBUTE_VECTOR_A);
                            clear_attr_vector(G_PLAYER_INIT_ATTRIBUTE_VECTOR_B);
                            wb(0x9b59b8, 0);
                        }
                        exe::database_player_to_attribute_vector(j, p, G_PLAYER_INIT_ATTRIBUTE_VECTOR_A, &mut e1a, core::ptr::null_mut(), core::ptr::null_mut());
                        exe::database_player_to_attribute_vector(cj, q, G_PLAYER_INIT_ATTRIBUTE_VECTOR_B, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                        let dif = (read_i16(j + player_sheet::CURRENT_ABILITY) as i32 - w(cj + 5) as i32).abs();
                        let mut esc = ftol(dif as f64 * read_f64(rdata::F64_2_5));
                        let g0 = w(G_PLAYER_INIT_ATTRIBUTE_VECTOR_A);
                        let comparar = if g0 == 0x14 {
                            if w(G_PLAYER_INIT_ATTRIBUTE_VECTOR_B) == g0 {
                                esc = 0;
                                false
                            } else {
                                true
                            }
                        } else {
                            g0 != -0x7f
                        };
                        if comparar {
                            let mut o = 0isize;
                            loop {
                                let dv = w((0xae2114 + o) as usize);
                                if dv < 1 || dv > 20 {
                                    esc += 100;
                                } else {
                                    let sv = w((0xae2044 + o) as usize);
                                    if in_range(sv, 1, 20) {
                                        if e35 == 0 {
                                            esc += (g0 as i32 - w(G_PLAYER_INIT_ATTRIBUTE_VECTOR_B) as i32).abs() * 1000;
                                        } else if e35 < e1a as i8 {
                                            esc += (sv as i32 - dv as i32).abs() * 25;
                                        } else {
                                            esc += (sv as i32 - dv as i32).abs() * 16;
                                        }
                                    }
                                }
                                e35 = e35.wrapping_add(1);
                                o = e35 as isize * 3;
                                if w((0xae2044 + o) as usize) == -0x7f {
                                    break;
                                }
                            }
                        }
                        if esc < e10 || molde == 0 {
                            molde = cj;
                            e10 = esc;
                        }
                    }
                    count -= 1; // the 250 drawn
                    if count == 0 {
                        break;
                    }
                }
                let start = e19 as i8 as isize;
                if molde == 0 {
                    let mut k = start;
                    while attr_cell_value(&v1, k) != -0x7f {
                        let x = attr_cell_value(&v1, k);
                        if x < 1 || x > 20 {
                            let r = s8(7);
                            let c = r.wrapping_add(((read_i16(j + player_sheet::CURRENT_ABILITY) as i32 / 10) as i8).wrapping_sub(3));
                            let n = if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 };
                            set_attr_cell_value(&mut v1, k, n);
                            if attr_cell_weight(&v1, k) == reroll_weight(&v1) {
                                set_attr_cell_value(&mut v1, k, s16(5).wrapping_add(1));
                            }
                        }
                        k += 1;
                    }
                } else {
                    exe::database_player_to_attribute_vector(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                    let mut hj = read_i16(j + player_sheet::CURRENT_ABILITY);
                    if !in_range(hj, 1, 200) {
                        hj = s16(100).wrapping_add(1);
                    }
                    let mut hm = w(molde + 5);
                    if !in_range(hm, 1, 200) {
                        hm = hj;
                    }
                    let mut k = start;
                    while attr_cell_value(&v1, k) != -0x7f {
                        let x = attr_cell_value(&v1, k);
                        if in_range(x, 1, 20) {
                            if attr_cell_weight(&v1, k) == reroll_weight(&v1) {
                                set_attr_cell_value(&mut v1, k, (x as i32 - 10).max(1) as i8 as i16);
                            }
                        } else {
                            let t = attr_cell_value(&v2, k);
                            let n = if t > 0 && t < 0x15 {
                                let mut c: i8 = if k < e11 as i8 as isize {
                                    (((hj as i32 - hm as i32) / 10) as i8).wrapping_add(t as u8 as i8)
                                } else {
                                    let c = t as u8 as i8;
                                    if c <= 5 {
                                        c.wrapping_add(s8(5))
                                    } else if c >= 0xf {
                                        c.wrapping_sub(s8(4))
                                    } else {
                                        c
                                    }
                                };
                                c = c.wrapping_add(s8(6).wrapping_sub(3));
                                if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 }
                            } else {
                                let r = s8(0xb);
                                let c = r.wrapping_add(((read_i16(j + player_sheet::CURRENT_ABILITY) as i32 / 10) as i8).wrapping_sub(5));
                                if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 }
                            };
                            set_attr_cell_value(&mut v1, k, n);
                            let o = k * 3;
                            if o == 0x9c || o == 0x9f {
                                let x = attr_cell_value(&v1, k);
                                if x < 0xc {
                                    set_attr_cell_value(&mut v1, k, x.wrapping_add(s8(0xd - x as i32) as i16));
                                }
                            } else if (o == 0x6c || o == 0x6f || o == 0x78) && age < 0x16 && attr_cell_value(&v1, k) > 0xf {
                                let x = attr_cell_value(&v1, k).wrapping_sub(s8(5) as i16);
                                set_attr_cell_value(&mut v1, k, x);
                            }
                            if attr_cell_weight(&v1, k) == reroll_weight(&v1) {
                                set_attr_cell_value(&mut v1, k, s16(10).wrapping_add(1));
                            }
                        }
                        k += 1;
                    }
                }
            }
            (exe::database_attribute_vector_to_player(j, p, v1.as_mut_ptr() as usize) as u32);
            if read_u8(j + player_sheet::GOALKEEPER) == 0x14 && read_i8(j + player_sheet::JUMPING) < 8 {
                wb(j + player_sheet::JUMPING, (exe::utils_random(6) as u8).wrapping_add(5));
            }
        } else {
            // ---- non-player only: an age outside 29..75 randomizes the birth date ----
            let age = age_at_date(Staff(p), data);
            if age < 0x1d || age > 0x4b {
                let a18 = read_i16(p + staff::AGE);
                let year: u32 = if a18 > 0x76c && a18 < 0x7af { a18 as u16 as u32 } else { (exe::utils_random(0x1a) + 0x794) as u32 };
                let month = exe::utils_random(0xb);
                let day = exe::utils_random(0x1b) + 1;
                exe::date_set(p + staff::DATE_OF_BIRTH, day as u32, month as u32, year, -1);
            }
            ww(p + staff::AGE, age_at_date(Staff(p), data) as i16);
        }

        let nj = NonPlayer(read_usize(p + staff::NON_PLAYER));
        if !nj.is_null() {
            // ---- the non-player: the reputations (+0xc, +0xa, +8) ----
            let a = nj.world_reputation().wrapping_mul(50);
            let di = nj.current_reputation().wrapping_mul(50);
            let bp = nj.home_reputation().wrapping_mul(50);
            ww(nj.addr() + non_player::CURRENT_REPUTATION, di);
            ww(nj.addr() + non_player::WORLD_REPUTATION, a);
            ww(nj.addr() + non_player::HOME_REPUTATION, bp);
            let rep = |x: usize| w(x + 0x80);
            let vezes = |x: usize, k: usize| ftol(rep(x) as f64 * read_f64(k)) as i16;
            if !in_range(a, 1, 10000) {
                let c = read_usize(p + staff::CLUB_JOB);
                let s = read_usize(p + staff::NATIONAL_JOB);
                ww(nj.addr() + non_player::WORLD_REPUTATION, if c != 0 && rep(c) > 0x1676 {
                    vezes(c, 0x969510)
                } else if s != 0 {
                    vezes(s, 0x968968)
                } else {
                    1
                });
            }
            let x = nj.world_reputation();
            let bx = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj.addr() + non_player::WORLD_REPUTATION, bx);
            if !in_range(di, 1, 10000) {
                let c = read_usize(p + staff::CLUB_JOB);
                let s = read_usize(p + staff::NATIONAL_JOB);
                ww(nj.addr() + non_player::CURRENT_REPUTATION, if in_range(bp, 1, 10000) {
                    bp
                } else if in_range(bx, 1, 10000) {
                    bx
                } else if c != 0 {
                    rep(c)
                } else if s != 0 {
                    vezes(s, 0x968978)
                } else {
                    1
                });
            }
            let x = nj.current_reputation();
            let di2 = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj.addr() + non_player::CURRENT_REPUTATION, di2);
            if !in_range(bp, 1, 10000) {
                let c = read_usize(p + staff::CLUB_JOB);
                let s = read_usize(p + staff::NATIONAL_JOB);
                ww(nj.addr() + non_player::HOME_REPUTATION, if in_range(di2, 1, 10000) {
                    di2
                } else if in_range(bx, 1, 10000) {
                    bx
                } else if c != 0 {
                    vezes(c, 0x9689d8)
                } else if s != 0 {
                    vezes(s, 0x968978)
                } else {
                    1
                });
            }
            let x = nj.home_reputation();
            ww(nj.addr() + non_player::HOME_REPUTATION, if x < 1 { 1 } else if x > 10000 { 10000 } else { x });
            let c = read_usize(p + staff::CLUB_JOB);
            if c == 0 || read_usize(c + club::NATION) == read_usize(p + staff::NATION) {
                ww(nj.addr() + non_player::HOME_REPUTATION, di2);
            }
            if !in_range(nj.current_ability(), 1, 200) {
                let c = read_usize(p + staff::CLUB_JOB);
                let s = read_usize(p + staff::NATIONAL_JOB);
                ww(nj.addr() + non_player::CURRENT_ABILITY, if c != 0 {
                    (rep(c) as i32 / 50) as i16
                } else if s != 0 {
                    (rep(s) as i32 / 50) as i16
                } else {
                    s16(100).wrapping_add(1)
                });
            }
            if !in_range((nj.potential_ability() as i16), 1, 200) {
                let r = s16(200).wrapping_add(1);
                let x = nj.current_ability();
                ww(nj.addr() + non_player::POTENTIAL_ABILITY, if x > r { x } else { r });
            }
            let nation = read_usize(p + staff::NATION);
            if nation != 0 && (nj.directness() as u8) == 0 {
                let id = gi(nation);
                let home = [KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_WALES, KEY_NATION_NORWAY].iter().any(|&k| id == gi(k));
                let v = if home {
                    let x = 0xdu8.wrapping_sub(exe::utils_random(0xe) as u8);
                    x.wrapping_add(exe::utils_random(8) as u8)
                } else {
                    let x = 5u8.wrapping_sub(exe::utils_random(5) as u8);
                    x.wrapping_add(exe::utils_random(7) as u8)
                };
                wb(nj.addr() + non_player::DIRECTNESS, v);
            }
            // ---- attributes outside 1..20: from the template ----
            exe::database_non_player_to_attribute_vector(nj, p, v1.as_mut_ptr() as usize, core::ptr::null_mut());
            let mut away = false;
            if attr_cell_value(&v1, 0) != -0x7f {
                let mut k = 0isize;
                loop {
                    let x = attr_cell_value(&v1, k);
                    if x < 1 || x > 20 {
                        away = true;
                        break;
                    }
                    k += 1;
                    if attr_cell_value(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if away {
                let c = read_usize(p + staff::CLUB_JOB);
                if read_u8(p + staff::JOB_FOR_CLUB) == 5 && c != 0 && rep(c) > 0x186a {
                    // the big club’s coach
                    let e_360 = || read_usize(p + staff::NATION) != 0 && gi(read_usize(p + staff::NATION)) == gi(KEY_NATION_ITALY);
                    let fora20 = |o: usize| signed_byte_at(nj.addr() + o) < 1 || signed_byte_at(nj.addr() + o) > 20;
                    if fora20(non_player::DIRECTNESS) {
                        wb(nj.addr() + non_player::DIRECTNESS, (if e_360() { s8(0x10).wrapping_add(1) } else { s8(0xe).wrapping_add(7) }) as u8);
                    }
                    if fora20(non_player::ATTACKING) {
                        let c = read_usize(p + staff::CLUB_JOB);
                        wb(nj.addr() + non_player::ATTACKING, (if rep(c) > 0x1e46 {
                            if e_360() { s8(4).wrapping_add(0xd) } else { s8(8).wrapping_add(0xd) }
                        } else {
                            s8(9).wrapping_add(0xc)
                        }) as u8);
                    }
                    if fora20(non_player::MOTIVATING) {
                        wb(nj.addr() + non_player::MOTIVATING, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(non_player::PRESSING) {
                        wb(nj.addr() + non_player::PRESSING, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(non_player::TACTICS) {
                        let c = read_usize(p + staff::CLUB_JOB);
                        wb(nj.addr() + non_player::TACTICS, (if rep(c) > 0x1e46 {
                            if e_360() { s8(7).wrapping_add(0xe) } else { s8(9).wrapping_add(0xc) }
                        } else {
                            s8(0xb).wrapping_add(0xa)
                        }) as u8);
                    }
                }
                let mut molde = 0usize;
                let mut best = e10;
                let mut count = 0xfai32;
                loop {
                    let r = exe::utils_random(gi(G_STAFF_COUNT));
                    let q = rd(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(110));
                    let cn = read_usize(q + staff::NON_PLAYER);
                    if cn != 0 && in_range(w(cn + 4), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b9) != 0 {
                            clear_attr_vector(G_NON_PLAYER_ATTRIBUTE_VECTOR);
                            clear_attr_vector(0xae2238);
                            wb(0x9b59b9, 0);
                        }
                        exe::database_non_player_to_attribute_vector(nj, p, G_NON_PLAYER_ATTRIBUTE_VECTOR, core::ptr::null_mut());
                        exe::database_non_player_to_attribute_vector(cn, q, 0xae2238, core::ptr::null_mut());
                        let dif = (nj.current_ability() as i32 - w(cn + 4) as i32).abs();
                        let mut esc = ftol(dif as f64 * read_f64(rdata::F64_2_5));
                        if w(G_NON_PLAYER_ATTRIBUTE_VECTOR) != -0x7f {
                            let mut o = 0isize;
                            loop {
                                let dv = w((0xae2238 + o) as usize);
                                if dv < 1 || dv > 20 {
                                    esc += 100;
                                } else {
                                    let sv = w((0xae18bc + o) as usize);
                                    if in_range(sv, 1, 20) {
                                        esc += (sv as i32 - dv as i32).abs() * 16;
                                    }
                                }
                                e35 = e35.wrapping_add(1);
                                o = e35 as isize * 3;
                                if w((0xae18bc + o) as usize) == -0x7f {
                                    break;
                                }
                            }
                        }
                        if esc < best || molde == 0 {
                            molde = cn;
                            best = esc;
                        }
                    }
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                }
                e10 = best;
                if molde != 0 {
                    exe::database_non_player_to_attribute_vector(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut());
                    let mut hn = nj.current_ability();
                    if !in_range(hn, 1, 200) {
                        hn = s16(100).wrapping_add(1);
                    }
                    let mut hm = w(molde + 4);
                    if !in_range(hm, 1, 200) {
                        hm = hn;
                    }
                    if attr_cell_value(&v1, 0) != -0x7f {
                        let mut k = 0isize;
                        loop {
                            let x = attr_cell_value(&v1, k);
                            if x < 1 || x > 20 {
                                let t = attr_cell_value(&v2, k);
                                let c = if in_range(t, 1, 20) {
                                    let d = (((hn as i32 - hm as i32) / 10) as i8).wrapping_add(t as u8 as i8);
                                    d.wrapping_add(s8(6).wrapping_sub(3))
                                } else {
                                    let r = s8(6);
                                    r.wrapping_add(((nj.current_ability() as i32 / 10) as i8).wrapping_sub(3))
                                };
                                set_attr_cell_value(&mut v1, k, if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 });
                            }
                            k += 1;
                            if attr_cell_value(&v1, k) == -0x7f {
                                break;
                            }
                        }
                    }
                } else if attr_cell_value(&v1, 0) != -0x7f {
                    let mut k = 0isize;
                    loop {
                        let x = attr_cell_value(&v1, k);
                        if x < 1 || x > 20 {
                            let r = s8(7);
                            let c = r.wrapping_add(((nj.current_ability() as i32 / 10) as i8).wrapping_sub(3));
                            set_attr_cell_value(&mut v1, k, if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 });
                        }
                        k += 1;
                        if attr_cell_value(&v1, k) == -0x7f {
                            break;
                        }
                    }
                }
            }
            (exe::database_attribute_vector_to_non_player(nj, p, v1.as_mut_ptr() as usize) as u32);
            // ---- o cargo: olheiro (9), fisioterapeuta (8), treinador (10) ----
            let ate = |o: usize, max: i8, n: i32, base: u8| {
                if signed_byte_at(nj.addr() + o) > max {
                    wb(nj.addr() + o, (exe::utils_random(n) as u8).wrapping_add(base));
                }
            };
            let job = read_u8(p + staff::JOB_FOR_CLUB);
            if job == 9 && exe::utils_random(100) != 0 {
                let a17 = nj.judgement();
                wb(nj.addr() + non_player::BUSINESS, 1);
                wb(nj.addr() + non_player::PHYSIOTHERAPY, 1);
                if a17 < 5 {
                    wb(nj.addr() + non_player::JUDGEMENT, (exe::utils_random(5) as u8).wrapping_add(4));
                }
                if nj.judging_potential() < 4 {
                    wb(nj.addr() + non_player::JUDGING_POTENTIAL, (exe::utils_random(5) as u8).wrapping_add(3));
                }
                ate(0x10, 8, 8, 1);
                ate(0x11, 8, 8, 1);
                ate(0x12, 5, 5, 1);
                ate(0x13, 5, 5, 1);
                ate(0x1a, 5, 5, 1);
            } else if job == 8 && exe::utils_random(100) != 0 {
                wb(nj.addr() + non_player::BUSINESS, 1);
                wb(nj.addr() + non_player::PHYSIOTHERAPY, 1);
            } else if job == 0xa && exe::utils_random(100) != 0 {
                let a1e = (nj.physiotherapy() as i8);
                wb(nj.addr() + non_player::BUSINESS, 1);
                if a1e < 5 {
                    wb(nj.addr() + non_player::PHYSIOTHERAPY, (exe::utils_random(10) as u8).wrapping_add(4));
                }
                for o in [0x10usize, 0x11, 0x12, 0x13, 0x17, 0x18, 0x1a] {
                    ate(o, 5, 5, 1);
                }
            }
            // ---- the famous names (+0x43) ----
            let name = |s: usize| exe::staff_name_equals(p, s) != 0;
            if name(0x9b63e0) {
                wb(nj.addr() + non_player::FORMATION, 3);
                wb(nj.addr() + non_player::DIRECTNESS, 7);
                wb(nj.addr() + non_player::ATTACKING, 0x11);
            } else if name(0x9b63d0) || name(0x9b63c4) || name(0x9b63b4) || name(0x9b63a8) {
                wb(nj.addr() + non_player::FORMATION, 3);
            } else if name(0x9b6398) || name(0x9b6388) || name(0x9b6378) || name(0x9b6368) || name(0x9b635c) {
                wb(nj.addr() + non_player::FORMATION, 2);
            } else if name(0x9b634c) {
                wb(nj.addr() + non_player::FORMATION, 6);
            } else {
                let x = nj.formation();
                if x == 0 || x >= 10 {
                    let r = exe::utils_random(0x1a);
                    if (r as u32) <= 0x19 {
                        const TABELA: [u8; 26] = [0, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8];
                        wb(nj.addr() + non_player::FORMATION, TABELA[r as usize]);
                    }
                }
            }
            for k in 0..8 {
                wd(nj.addr() + non_player::GOALKEEPER + 4 * k, 0);
            }
            // ---- the missing role ----
            if read_u8(p + staff::JOB_FOR_CLUB) == 0 && read_u8(p + staff::JOB_FOR_NATION) == 0 {
                let (c21, c1b, a10, a11) = (nj.tactics(), (nj.motivating() as i8), nj.coaching(), nj.coaching_gks());
                wb(p + staff::JOB_FOR_CLUB, if read_usize(p + staff::PLAYER) != 0 {
                    0xb
                } else if c21 > 10 && c1b > 10 {
                    5
                } else if (a10 > 6 || a11 > 6) && c1b > 10 && c21 > 10 {
                    6
                } else if a10 <= 6 && a11 <= 6 {
                    9 + ((nj.physiotherapy() as i8) > 10) as u8
                } else {
                    8
                });
            }
        }
        ww(p + staff::AGE, age_at_date(Staff(p), data) as i16);
        i += 1;
        p += 110;
    }

    // ---- the second pass: the ability-relative vectors and the normalizations ----
    let mut v3 = [0u8; 189];
    let mut q = rd(G_STAFF_TABLE);
    let mut k = 0i32;
    while k < gi(G_STAFF_COUNT) {
        let jj = read_usize(q + staff::PLAYER);
        let nn = read_usize(q + staff::NON_PLAYER);
        if jj != 0 {
            clear_attr_vector(v3.as_mut_ptr() as usize);
            let (mut fim_l, mut fim_p, mut fim_q) = (0u8, 0u8, 0u8);
            exe::database_player_to_attribute_vector(jj, q, v3.as_mut_ptr() as usize, &mut fim_l, &mut fim_p, &mut fim_q);
            let a = fim_l.wrapping_add(2) as i8;
            let f = fim_q as i8;
            if a < f {
                for c in a as isize..f as isize {
                    let x = attr_cell_value(&v3, c).wrapping_mul(10).wrapping_sub(w(jj + 5));
                    let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                    set_attr_cell_value(&mut v3, c, (x as i32 / 2) as i16);
                }
            }
            (exe::database_attribute_vector_to_player(jj, q, v3.as_mut_ptr() as usize) as u32);
            exe::staff_normalise_player_physical_attributes(q);
            exe::staff_normalise_player_attributes(q);
        }
        if nn != 0 {
            clear_attr_vector(v3.as_mut_ptr() as usize);
            let mut end = 0u8;
            exe::database_non_player_to_attribute_vector(nn, q, v3.as_mut_ptr() as usize, &mut end);
            let mut c = end as i8;
            while attr_cell_value(&v3, c as isize) != -0x7f {
                let x = attr_cell_value(&v3, c as isize).wrapping_mul(10).wrapping_sub(w(nn + 4));
                let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                set_attr_cell_value(&mut v3, c as isize, (x as i32 / 2) as i16);
                c = c.wrapping_add(1);
            }
            (exe::database_attribute_vector_to_non_player(nn, q, v3.as_mut_ptr() as usize) as u32);
            exe::staff_normalise_non_player_attributes(q);
        }
        k += 1;
        q += 110;
    }
}

/// strcpy: copies a null-terminated string from origem to destino.
///
/// [pt] strcpy (o `repne scasb` e os `rep movsd`/`rep movsb` da original).
pub(crate) unsafe fn copy_c_string(target: usize, source: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(source + k);
        write_u8(target + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// `database_format_name_from_strings(first, surname, common, text, size, style)`: writes the person's name into `text`
/// by style (byte) from the three strings (null or empty ones count as missing).
///
/// Styles 1 to 4: the common name, if any; otherwise, with first name and surname, 1 "%s %s", 2 "%c.%s", 3 "%s, %s"
/// (surname first), 4 "%s, %c"; otherwise the surname, otherwise the first name. 5: the common name, the surname or the
/// first name. 6: the initials ("%c" of the common name, "%c %c" of the first name and surname, "%c" of the surname or
/// the first name). 7: "%s %s" first, then the common name, the surname or the first name. With nothing: the empty
/// text. Returns 1.
///
/// Null text, size (word) < 0x69 or style outside 1..7: the original (error box; the 0x200-byte locals serve only it).
/// TapeCall through `database_format_name_from_ids` (already in Rust, through the game's address: the hook applies there too), through the
/// club screens (0x45ccf0) and through manager_manager.
/// Original: `database_format_name_from_strings` (0x53e2e0, Database.cpp, cdecl).
///
/// [pt] { 0x53e2e0 } (cdecl, Database.cpp): `database_format_name_from_strings(nome, sobrenome, comum, texto, tamanho,
/// forma)`: o nome da pessoa em `texto` pela forma (byte) a partir dos três textos (nulos ou vazios contam como
/// faltando). Formas 1 a 4: o nome comum, se houver; senão, com nome e sobrenome, 1 "%s %s", 2 "%c.%s", 3 "%s, %s"
/// (sobrenome primeiro), 4 "%s, %c"; senão o sobrenome, senão o nome. 5: o comum, o sobrenome ou o nome. 6: as
/// iniciais ("%c" do comum, "%c %c" do nome e do sobrenome, "%c" do sobrenome ou do nome). 7: "%s %s" primeiro, depois
/// o comum, o sobrenome ou o nome. Sem nada: o texto vazio. Devolve 1. Texto nulo, tamanho (palavra) < 0x69 ou forma
/// fora de 1..7: a original (caixa; as locais de 0x200 bytes só servem a ela). TapeCall pela 0x53e140 (já em Rust, pelo
/// endereço do jogo: o desvio vale também lá), pelas telas do clube (0x45ccf0) e pelo manager_manager.
pub(crate) unsafe extern "C" fn database_format_name_from_strings(name: usize, surname: usize, common: usize, text: usize, size: u32, form: u32) -> u32 {
    let f = form as u8;
    if text == 0 || (size as u16 as i16) < 0x69 || !(1..=7).contains(&f) {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_STRINGS)(name, surname, common, text, size, form);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (a, b, c) = (name, surname, common);
    let initial = |t: usize| read_i8(t) as i32;
    write_u8(text, 0);
    let two = is_nonempty_text(a) && is_nonempty_text(b);
    // what remains: the first full one in the list
    let one_of = |list: &[usize]| {
        if let Some(&t) = list.iter().find(|&&t| is_nonempty_text(t)) {
            copy_c_string(text, t);
        }
    };
    match f {
        7 => {
            if two {
                sprintf(text, 0x98f0b8, a, b); // "%s %s"
            } else {
                one_of(&[c, b, a]);
            }
        }
        6 => {
            if is_nonempty_text(c) {
                sprintf(text, 0x99ec1c, initial(c)); // "%c"
            } else if two {
                sprintf(text, 0x9b7418, initial(a), initial(b)); // "%c %c"
            } else if is_nonempty_text(b) {
                sprintf(text, 0x99ec1c, initial(b));
            } else if is_nonempty_text(a) {
                sprintf(text, 0x99ec1c, initial(a));
            }
        }
        5 => one_of(&[c, b, a]),
        _ => {
            if is_nonempty_text(c) {
                copy_c_string(text, c);
            } else if two {
                match f {
                    1 => { sprintf(text, 0x98f0b8, a, b); } // "%s %s" (dbu)
                    2 => { sprintf(text, 0x9b7410, initial(a), b); } // "%c.%s"
                    3 => { sprintf(text, 0x98f6ac, b, a); } // "%s, %s"
                    _ => { sprintf(text, 0x9b7408, b, initial(a)); } // "%s, %c"
                }
            } else {
                one_of(&[b, a]);
            }
        }
    }
    1
}

/// The texts of job roles 0..0x11 of `database_get_job_name` (code immediates, through the addresses in the exe): short lowercase,
/// short uppercase, full lowercase, full uppercase and the gender with language 4 (0: not written).
///
/// The short ones use the empty text 0xad9c64 ("<%s - COMMENT - short name>"); of the full ones, only the player's
/// (11). Role 14 does not exist (0).
///
/// [pt] Os textos dos cargos 0..0x11 da 0x53ad00 (imediatos do código, pelos endereços no exe): curto minúsculo, curto
/// maiúsculo, inteiro minúsculo, inteiro maiúsculo e o gênero com o idioma 4 (0: não grava). Os curtos levam o
/// vazio 0xad9c64 ("<%s - COMMENT - short name>"); dos inteiros, só o do jogador (11). O 14 não existe (0).
pub(crate) const JOB_TITLE_TEXTS: [(usize, usize, usize, usize, u8); 18] = [
    (0x9b6b10, 0x9b6b38, 0x9b6af8, 0x9b6b04, 1), // unemployed
    (0x9b6ab0, 0x9b6ad4, 0x9b6aa4, 0x9b1234, 1), // chairman
    (0x9b6a54, 0x9b6a7c, 0x9b6a2c, 0x9b6a40, 4), // managing director
    (0x9b6780, 0x9b67a8, 0x9b6760, 0x9b6770, 1), // general manager
    (0x9b69dc, 0x9b6a04, 0x9b69ac, 0x9b69c4, 1), // director of football
    (0x9b6964, 0x9b6988, 0x9b695c, 0x9b6440, 4), // manager (dbu)
    (0x9b690c, 0x9b6934, 0x9b68f8, 0x9b642c, 4), // assistant manager
    (0x9b68a8, 0x9b68d0, 0x9b6878, 0x9b6890, 4), // reserve team manager
    (0x9b6830, 0x9b6854, 0x9b6828, 0x9b6448, 1), // coach
    (0x9b67e0, 0x9b6804, 0x9b67d0, 0x9b67d8, 4), // scout
    (0x9b6718, 0x9b673c, 0x9b6708, 0x9b6710, 1), // physio
    (0x9b66b8, 0x9b66e0, 0x9b6678, 0x9b6698, 1), // player
    (0x9b6628, 0x9b6650, 0x9b6608, 0x9b6618, 1), // player/manager
    (0x9b65b0, 0x9b65dc, 0x9b6578, 0x9b6594, 1), // player/assistant manager
    (0, 0, 0, 0, 0),
    (0x9b6530, 0x9b6554, 0x9b6510, 0x9b6520, 1), // player/coach
    (0x9b64c8, 0x9b64ec, 0x9b64a8, 0x9b64b8, 0), // retired player (no gender)
    (0x9b6460, 0x9b6484, 0x9b6450, 0x9b6450, 1), // pundit, “Media Pundit” in both
];

/// `database_get_job_name(job, short, upper, gender)`: the translated name (0x66f4e0) of the job (signed byte, 0..0x11
/// except 14) in the text [0xae19b4], which is what it returns; short or full and lower or upper case by the two ints.
///
/// With `gender` not null and the language (`exe::langlib_get_language`) 4, writes the gender: 4 for chief executive, manager, assistant,
/// reserve team manager and scout; 1 for the others; nothing for the retired. Job out of range: 0, without writing.
///
/// Used in the club, contract, person and search screens, in the media (0x744f80, 0x74bbd0), in the regens (0x7a9650,
/// 0x7aa4c0) and in transfers (0x8d3520, 0x8dc370).
/// Original: `database_get_job_name` (0x53ad00, Database.cpp, cdecl).
///
/// [pt] { 0x53ad00 } (cdecl, Database.cpp): `database_get_job_name(cargo, curto, maiuscula, genero)`: o nome traduzido
/// (0x66f4e0) do cargo (byte com sinal, 0..0x11 sem o 14) no texto [0xae19b4], que é o que devolve; curto ou inteiro e
/// minúsculo ou maiúsculo pelos dois inteiros. Com `genero` não nulo e o idioma (0x66f4d0) 4, grava o gênero: 4 para
/// diretor executivo, técnico, auxiliar, técnico dos reservas e olheiro; 1 para os outros; nada para o aposentado.
/// Cargo fora da faixa: 0, sem escrever. Nas telas do clube, de contratos, da pessoa e de busca, na mídia
/// (0x744f80, 0x74bbd0), nos regens (0x7a9650, 0x7aa4c0) e nas transferências (0x8d3520, 0x8dc370).
pub(crate) unsafe extern "C" fn database_get_job_name(job: u32, short: u32, capital: u32, gender: usize) -> usize {
    let j = job as u8 as i8;
    if !(0..=0x11).contains(&j) || j == 14 {
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (cm, cg, im, ig, g) = JOB_TITLE_TEXTS[j as usize];
    if short != 0 {
        format(G_JOB_NAME_BUFFER, if capital != 0 { cg } else { cm }, 0xad9c64usize);
    } else if j == 11 {
        format(G_JOB_NAME_BUFFER, if capital != 0 { ig } else { im }, 0xad9c64usize); // "<%s - COMMENT - job name>"
    } else {
        format(G_JOB_NAME_BUFFER, if capital != 0 { ig } else { im });
    }
    if g != 0 && gender != 0 && exe::langlib_get_language() == 4 {
        write_u8(gender, g);
    }
    G_JOB_NAME_BUFFER
}

/// The names of languages 1..64 of `database_get_language_name` (code immediates, by their addresses in the exe; 34 repeats Albanian), all
/// with the empty 0xad9c64 ("<%s - COMMENT - language>").
///
/// [pt] Os nomes dos idiomas 1..64 da 0x53cee0 (imediatos do código, pelos endereços no exe; o 34 repete o albanês),
/// todos com o vazio 0xad9c64 ("<%s - COMMENT - language>").
pub(crate) const LANGUAGE_NAMES: [usize; 64] = [
    0x9b73dc, 0x9b73bc, 0x9b7398, 0x9b7374, 0x9b7350, 0x9b7330, 0x9b7310, 0x9b72ec, // Albanian .. English
    0x9b72c8, 0x9b72a8, 0x9b7288, 0x9b7268, 0x9b7248, 0x9b7224, 0x9b7200, 0x9b71dc, // Finnish .. Japanese
    0x9b71bc, 0x9b719c, 0x9b7178, 0x9b7154, 0x9b7130, 0x9b710c, 0x9b70e8, 0x9b70c4, // Korean .. Spanish
    0x9b70a0, 0x9b707c, 0x9b705c, 0x9b7038, 0x9b7018, 0x9b6fb0, 0x9b6fd4, 0x9b6ff8, // Turkish .. Danish
    0x9b6f8c, 0x9b73dc, 0x9b6f68, 0x9b6f44, 0x9b6f24, 0x9b6f04, 0x9b6ee0, 0x9b6ebc, // Afrikaans, Albanian .. Georgian
    0x9b6e9c, 0x9b6e78, 0x9b6e58, 0x9b6e38, 0x9b6e18, 0x9b6df4, 0x9b6dcc, 0x9b6da8, // Hindi .. Lithuanian
    0x9b6d84, 0x9b6d60, 0x9b6d3c, 0x9b6d18, 0x9b6cf4, 0x9b6cd4, 0x9b6cb0, 0x9b6c90, // Macedonian .. Tajik
    0x9b6c70, 0x9b6c50, 0x9b6c30, 0x9b6c0c, 0x9b6bec, 0x9b6bc8, 0x9b6ba4, 0x9b6b84, // Tamil .. Zulu
];

/// `database_get_language_name(language)`: the translated name (0x66f4e0) of the language (signed byte, 1..64;
/// otherwise "Unknown") in the text [0xae21d4] (100 bytes), which is what it returns.
///
/// Used in the search (0x813210) and in the person screen (0x87dd80).
/// Original: `database_get_language_name` (0x53cee0, Database.cpp, cdecl).
///
/// [pt] { 0x53cee0 } (cdecl, Database.cpp): `database_get_language_name(idioma)`: o nome traduzido (0x66f4e0) do idioma
/// (byte com sinal, 1..64; fora disso "Unknown") no texto [0xae21d4] (100 bytes), que é o que devolve. Na busca
/// (0x813210) e na tela da pessoa (0x87dd80).
pub(crate) unsafe extern "C" fn database_get_language_name(language: u32) -> usize {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let x = language as u8 as i8;
    let s = if (1..=64).contains(&x) { LANGUAGE_NAMES[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)
    format(0xae21d4, s, 0xad9c64usize);
    0xae21d4
}

/// True if the text's first byte is 0.
pub(crate) unsafe fn is_empty_text(t: usize) -> bool {
    read_u8(t) == 0
}

/// strcat: appends a null-terminated string to destino.
///
/// [pt] strcat (os `repne scasb` e os `rep movsd`/`rep movsb` da original).
pub(crate) unsafe fn append_c_string(target: usize, source: usize) {
    let mut d = target;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut k = 0;
    loop {
        let c = read_u8(source + k);
        write_u8(d + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// `club_get_national_team_name(club, variant, text, size, style, quotes)`: the national team club name in `text` by
/// style (byte, 1..4; otherwise does not write).
///
/// The name is the one at +0x38 with gender +0x52 (signed byte); in style 3, the one at +4 with gender +0x37. Outside
/// the national teams (id < [0xae235c] − 2 × [0xae2350]): variant 0 copies the name (style 4: "Main Squad"); otherwise
/// "Reserves" (styles 1 and 3), "Reserve" (2), "Reserve Squad" (4). National team with variant: "B" or "'B'" (with
/// `quotes`), in style 4 "B Squad"/"'B' Squad". Main national team (id < clubs − nations): copies the name (style 4:
/// "Squad"). The youth ones: under-21 or under-23 by the byte [[0xb74318] + 0x36] + the nation's id (+0x53),
/// "U21"/"U23" (1), "Under 21"/"Under 23" (2), "Under 21's"/"Under 23's" (3), "Under 21 Squad"/"Under 23 Squad" (4).
/// The formats go to 0x66f4e0 with (gender, gender, name). Returns 1.
///
/// Size < 0x47 or null club (with style 1..4): the original (error box). Used in the club, national team and manager
/// screens, in the match line-up (`match_screens_open_team_selection`, in Rust), in the hall of fame, in the printouts and in Setup.
/// Original: `club_get_national_team_name` (0x542100, Database.cpp, cdecl).
///
/// [pt] { 0x542100 } (cdecl, Database.cpp): `club_get_national_team_name(clube, variante, texto, tamanho, forma, aspas)`:
/// o nome do clube de seleção em `texto` pela forma (byte, 1..4; fora disso não escreve). O nome é o de +0x38 com o
/// gênero +0x52 (byte com sinal), na forma 3 o de +4 com o gênero +0x37. Fora das seleções (id < [0xae235c] −
/// 2 × [0xae2350]): variante 0 copia o nome (forma 4: "Main Squad"); senão "Reserves" (formas 1 e 3), "Reserve" (2),
/// "Reserve Squad" (4). Seleção com variante: "B" ou "'B'" (com `aspas`), na forma 4 "B Squad"/"'B' Squad". Seleção
/// principal (id < clubes − nações): copia o nome (forma 4: "Squad"). As de base: sub-21 ou sub-23 pelo byte
/// [[0xb74318] + 0x36] + id da nação (+0x53), "U21"/"U23" (1), "Under 21"/"Under 23" (2), "Under 21's"/"Under 23's"
/// (3), "Under 21 Squad"/"Under 23 Squad" (4). Os formatos vão ao 0x66f4e0 com (gênero, gênero, nome). Devolve 1.
/// Tamanho < 0x47 ou clube nulo (com forma 1..4): a original (caixa). Nas telas do clube, da seleção, do técnico, na
/// escalação da partida (0x725f70, em Rust), no hall da fama, nas impressões e no Setup.
pub(crate) unsafe extern "C" fn club_get_national_team_name(club: usize, variant: u32, text: usize, size: i32, form: u32, aspas: u32) -> u32 {
    let club = Club(club);
    let f = form as u8 as i8;
    if size < 0x47 || ((1..=4).contains(&f) && club.is_null()) {
        return original_fn::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(IDX_CLUB_GET_NATIONAL_TEAM_NAME)(club.addr(), variant, text, size, form, aspas);
    }
    if !(1..=4).contains(&f) {
        return 1;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let id = read_usize(club.addr()) as i32;
    let nations = read_usize(G_NATION_COUNT) as i32;
    let clubs = read_usize(G_CLUB_COUNT) as i32;
    let (nome, g) = if f == 3 { (club.addr() + 4, club.gender_name() as i32) } else { (club.addr() + club::SHORT_NAME, (club.short_gender_name() as i8) as i32) };
    let formato = if id < clubs.wrapping_sub(nations.wrapping_mul(2)) {
        if variant == 0 {
            if f == 4 { Some(0x9b7634) } else { None } // Main Squad
        } else {
            Some([0x9b784c, 0x9b77ac, 0x9b784c, 0x9b770c][f as usize - 1]) // Reserves, Reserve, Reserves, Reserve Squad
        }
    } else if variant != 0 {
        Some(match (f == 4, aspas != 0) {
            (false, false) => 0x9b7814, // B
            (false, true) => 0x9b7830,  // 'B'
            (true, false) => 0x9b76c8,  // B Squad
            (true, true) => 0x9b76e8,   // 'B' Squad
        })
    } else if id < clubs.wrapping_sub(nations) {
        if f == 4 { Some(0x9b7658) } else { None } // Squad
    } else {
        let nation = read_usize(club.nation().addr());
        let sub23 = read_u8(nation.wrapping_add(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36))) != 0;
        Some(match (f, sub23) {
            (1, false) => 0x9b7734, // U21
            (1, true) => 0x9b7750,  // U23
            (2, false) => 0x9b776c, // Under 21 (dbu)
            (2, true) => 0x9b778c,  // Under 23
            (3, false) => 0x9b77cc, // Under 21's
            (3, true) => 0x9b77f0,  // Under 23's
            (_, false) => 0x9b7678, // Under 21 Squad
            (_, true) => 0x9b76a0,  // Under 23 Squad
        })
    };
    match formato {
        Some(s) => {
            format(text, s, g, g, nome);
        }
        None => {
            write_u8(text, 0);
            append_c_string(text, nome);
        }
    }
    1
}

/// Os nomes dos 31 atributos de jogador da `database_get_player_attribute_name` (imediatos do código, pelos endereços no exe): (abreviado, leva o
/// vazio 0xad9c64, inteiro, leva o vazio).
pub(crate) const PLAYER_ATTRIBUTE_NAMES: [(usize, bool, usize, bool); 31] = [
    (0x9b7b60, false, 0x9b7b50, false), // Acc / Acceleration
    (0x9b7af4, false, 0x9b7ae8, false), // Agg / Aggression
    (0x9b7ae4, false, 0x9b7adc, false), // Agi / Agility
    (0x9b7ad8, false, 0x9b7ac8, false), // Ant / Anticipation
    (0x9b7ac4, false, 0x9b7a98, true),  // Bal / Balance
    (0x9b7a94, false, 0x9b7a8c, false), // Bra / Bravery
    (0x9b7a88, false, 0x9b7a7c, false), // Cre / Creativity
    (0x9b7a78, false, 0x9b7a6c, false), // Cro / Crossing
    (0x9b7b30, true, 0x9b7b24, false),  // Dec / Decisions
    (0x9b7a68, false, 0x9b7a58, false), // Det / Determination
    (0x9b7a54, false, 0x9b7a48, false), // Dri / Dribbling
    (0x9b78d4, true, 0x9b78c8, false),  // Fin / Finishing
    (0x9b7a44, false, 0x9b7a3c, false), // Fla / Flair
    (0x9b7a38, false, 0x9b7a2c, false), // Han / Handling
    (0x9b7a28, false, 0x9b7a20, false), // Hea / Heading
    (0x98ea64, false, 0x9b7a14, false), // Inf / Influence
    (0x9b7a10, false, 0x9b7a08, false), // Jum / Jumping
    (0x9b7b04, true, 0x9b7af8, false),  // Lon / Long Shots
    (0x9b79d8, true, 0x9b79b0, true),   // Mar / Marking
    (0x9b7984, true, 0x9b7974, false),  // Off / Off The Ball
    (0x9b7970, false, 0x9b7968, false), // Pac / Pace
    (0x9b7964, false, 0x9b795c, false), // Pas / Passing
    (0x9b7958, false, 0x9b794c, false), // Pos / Positioning
    (0x9b792c, true, 0x9b7920, false),  // Ref / Reflexes
    (0x9b7900, true, 0x9b78f4, false),  // Set / Set Pieces
    (0x9b78c4, false, 0x9b78bc, false), // Sta / Stamina
    (0x9b78b8, false, 0x9b78ac, false), // Str / Strength
    (0x9b78a8, false, 0x9b789c, false), // Tac / Tackling
    (0x9b7898, false, 0x9b788c, false), // Tea / Teamwork
    (0x9b7888, false, 0x9b787c, false), // Tec / Technique
    (0x9b7878, false, 0x9b786c, false), // Wor / Work Rate
];

/// `database_get_player_attribute_name(attribute, text, size, abbreviated)`: clears the text and writes the translated
/// name (0x66f4e0) of the player attribute (1..0x1f, signed byte), abbreviated (`abbreviated` not null) or full;
/// returns 1.
///
/// The map splits the routine at 0x542f48, which is just its middle: the real boundary goes up to `staff_get_player_attribute`. Attribute
/// out of range, null text or size (signed short) < 100: the original (error box and 0). Used in the club, search,
/// person and training screens.
/// Original: `database_get_player_attribute_name` (0x5428a0, Database.cpp, cdecl).
///
/// [pt] { 0x5428a0 } (cdecl, Database.cpp; o mapa parte a rotina em 0x542f48, que é só o meio dela: a fronteira real vai
/// até 0x5431e0): `database_get_player_attribute_name(atributo, texto, tamanho, abreviado)`: zera o texto e escreve o
/// nome traduzido (0x66f4e0) do atributo de jogador (1..0x1f, byte com sinal), abreviado (`abreviado` não nulo) ou
/// inteiro; devolve 1. Atributo fora da faixa, texto nulo ou tamanho (curto com sinal) < 100: a original (caixa e 0).
/// Nas telas do clube, de busca, da pessoa e de treino.
pub(crate) unsafe extern "C" fn database_get_player_attribute_name(attribute: u32, text: usize, size: u32, abbreviated: u32) -> u32 {
    let x = attribute as u8 as i8;
    if !(1..=0x1f).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_PLAYER_ATTRIBUTE_NAME)(attribute, text, size, abbreviated);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    write_u8(text, 0);
    let (c, cv, l, lv) = PLAYER_ATTRIBUTE_NAMES[x as usize - 1];
    let (s, vazio) = if abbreviated != 0 { (c, cv) } else { (l, lv) }; // (dbu)
    if vazio {
        format(text, s, 0xad9c64usize);
    } else {
        format(text, s);
    }
    1
}

/// Appends one position to the position text built by `database_format_player_positions`: short or long form (translated via the global buffer
/// 0xde1f64) with a "/" separator, or a single letter only while the text is empty.
///
/// [pt] Uma posição no texto da 0x53ee50: forma 1 o abreviado (com o vazio 0xad9c64), 2 e 3 o inteiro, com a barra antes
/// quando o texto já tem algo; as outras formas, a letra só com o texto ainda vazio. O traduzido passa pelo texto
/// global [0xde1f64] (0x66f4e0), como na original.
pub(crate) unsafe fn append_position_text(buf: usize, f: u8, short: usize, long: usize, long_empty: bool, letra: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match f {
        1 => {
            if !is_empty_text(buf) {
                append_c_string(buf, 0x9b7630); // "/"
            }
            format(0xde1f64, short, 0xad9c64usize);
            append_c_string(buf, 0xde1f64);
        }
        2 | 3 => {
            if !is_empty_text(buf) {
                append_c_string(buf, 0x9b7630);
            }
            if long_empty {
                format(0xde1f64, long, 0xad9c64usize);
            } else {
                format(0xde1f64, long);
            }
            append_c_string(buf, 0xde1f64);
        }
        _ => {
            if is_empty_text(buf) {
                append_c_string(buf, letra);
            }
        }
    }
}

/// As posições da `database_format_player_positions` (abreviado, inteiro, o inteiro leva o vazio, letra), imediatos do código.
pub(crate) const POSITION_GOALKEEPER: (usize, usize, bool, usize) = (0x9b7610, 0x9b7600, false, 0x9b760c); // GK, Goalkeeper, A
pub(crate) const POSITION_SWEEPER: (usize, usize, bool, usize) = (0x9b75e4, 0x9b75dc, false, 0x98b3f0); // SW, Sweeper, B
pub(crate) const POSITION_DEFENDER: (usize, usize, bool, usize) = (0x9b75c0, 0x9b75b0, false, 0x9b75bc); // D, Defender, C
pub(crate) const POSITION_DEFENSIVE_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7588, 0x9b756c, false, 0x9b7584); // DM, Defensive Midfielder, D
pub(crate) const POSITION_ATTACKING_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7544, 0x9b7528, false, 0x9b7540); // AM, Attacking Midfielder, F
pub(crate) const POSITION_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7508, 0x9b74f8, false, 0x9b7504); // M, Midfielder, E
pub(crate) const POSITION_STRIKER: (usize, usize, bool, usize) = (0x9b74dc, 0x9b74d0, false, 0x9b74d8); // S, Striker, H
pub(crate) const POSITION_FORWARD: (usize, usize, bool, usize) = (0x9b74b4, 0x9b7484, true, 0x9b74b0); // F, Forward<%s>, G

/// `database_format_player_positions(player, text, size, style)`: the text of the player's positions (ratings
/// +0xf..+0x1a, signed bytes) by style (byte): 1 abbreviated ("D/DM RC"), 2 full with the sides in parentheses, 3 full
/// without the sides, 4 and others the letters.
///
/// The line: with the minimum rating going from 15 down to 10 (stopping when something is found): goalkeeper (+0xf),
/// sweeper (+0x10), defender (+0x11), defensive midfielder (+0x12) or attacking midfielder (+0x14 and +0x13) or
/// midfielder (+0x13), and striker (+0x15; "forward" if also +0x14, +0x17, +0x18 or +0x1a). Goalkeeper at the rating:
/// only the line. Nothing down to 10: attacking midfielder, and with +0x14 ≥ 15 the original WRITES 15 into the
/// player's +0x13 (so does the copy).
///
/// The sides (rating ≥ 15): right (+0x17), left (+0x18), centre (+0x19); style 1 abbreviated together, 2 full with a
/// slash, 4 the letters A, B, C only with empty sides. End: style 2 "%s (%s)", the others "%s %s" or just the line.
/// Returns 1.
///
/// The translated texts go through the global [0xde1f64] (the same use as the original). Null text or player or size
/// (signed short) < 30: the original (error box). The locals (line and sides, 2 KB each) are written before being read.
/// Used in the club, search, person, tactics and training screens and in the printouts.
/// Original: `database_format_player_positions` (0x53ee50, Database.cpp, cdecl).
///
/// [pt] { 0x53ee50 } (cdecl, Database.cpp): `database_format_player_positions(jogador, texto, tamanho, forma)`: o texto das
/// posições do jogador (as notas +0xf..+0x1a, bytes com sinal) pela forma (byte): 1 abreviado ("D/DM RC"), 2 inteiro
/// com os lados entre parênteses, 3 inteiro sem os lados, 4 e outras as letras. A linha: com a nota mínima de 15 a 10
/// (para quando achar algo): goleiro (+0xf), líbero (+0x10), defensor (+0x11), volante (+0x12) ou meia-atacante (+0x14
/// e +0x13) ou meia (+0x13), e atacante (+0x15; "avante" se também +0x14, +0x17, +0x18 ou +0x1a). Goleiro na nota:
/// só a linha. Nada até 10: meia-atacante, e com +0x14 ≥ 15 a original GRAVA 15 em +0x13 do jogador (a cópia também).
/// Os lados (nota ≥ 15): direito (+0x17), esquerdo (+0x18), centro (+0x19); forma 1 abreviados juntos, 2 inteiros com
/// barra, 4 as letras A, B, C só com os lados vazios. Fim: forma 2 "%s (%s)", as outras "%s %s" ou só a linha.
/// Devolve 1. Os textos traduzidos passam pelo global [0xde1f64] (o mesmo uso da original). Texto ou jogador nulos ou
/// tamanho (curto com sinal) < 30: a original (caixa). As locais (linha e lados, 2 KB cada) são gravadas antes de lidas.
/// Nas telas do clube, de busca, da pessoa, de tática e de treino e nas impressões.
pub(crate) unsafe extern "C" fn database_format_player_positions(player: usize, text: usize, size: u32, form: u32) -> u32 {
    let player = PlayerSheet(player);
    if text == 0 || player.is_null() || (size as u16 as i16) < 0x1e {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_PLAYER_POSITIONS)(player.addr(), text, size, form);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    write_u8(text, 0);
    let f = form as u8;
    let mut row = [0u8; 0x7d0];
    let mut sides = [0u8; 0x7c0];
    let buf = row.as_mut_ptr() as usize;
    let pos = |p: (usize, usize, bool, usize)| append_position_text(buf, f, p.0, p.1, p.2, p.3);
    let mut minimum: u8 = 0xf;
    loop {
        write_u8(buf, 0);
        let t = minimum as i32;
        if (player.goalkeeper() as i32) >= t {
            pos(POSITION_GOALKEEPER);
        }
        if (player.sweeper() as i32) >= t {
            pos(POSITION_SWEEPER);
        }
        if (player.defender() as i32) >= t {
            pos(POSITION_DEFENDER);
        }
        if (player.defensive_midfielder() as i32) >= t {
            pos(POSITION_DEFENSIVE_MIDFIELDER);
        } else if (player.attacking_midfielder() as i32) >= t && (player.midfielder() as i32) >= t {
            pos(POSITION_ATTACKING_MIDFIELDER);
        } else if (player.midfielder() as i32) >= t {
            pos(POSITION_MIDFIELDER);
        }
        if (player.attacker() as i32) >= t {
            if (player.attacking_midfielder() as i32) >= t || (player.right_side() as i32) >= t || (player.left_side() as i32) >= t || (player.free_role() as i32) >= t {
                pos(POSITION_FORWARD);
            } else {
                pos(POSITION_STRIKER);
            }
        }
        if (player.goalkeeper() as i32) >= t {
            // goalkeeper: only the row (strcpy in the original; the text may be [0xde1f64] itself)
            write_u8(text, 0);
            append_c_string(text, buf);
            return 1;
        }
        minimum = minimum.wrapping_sub(1);
        if !is_empty_text(buf) || minimum < 0xa {
            break;
        }
    }
    if is_empty_text(buf) {
        if (player.attacking_midfielder() as i32) >= 0xf {
            player.set_midfielder((0xf) as i8); // the original writes to the player
        }
        if f == 2 || f == 3 {
            format(0xde1f64, POSITION_ATTACKING_MIDFIELDER.1);
        } else {
            format(0xde1f64, POSITION_ATTACKING_MIDFIELDER.0, 0xad9c64usize);
        }
        append_c_string(buf, 0xde1f64);
    }
    let side = sides.as_mut_ptr() as usize;
    let mut volta: u8 = 0xf;
    loop {
        write_u8(side, 0);
        for (o, curto, longo, letra) in [(0x17, 0x9b746c, 0x9b7464, 0x9b760c), (0x18, 0x9b744c, 0x9b7444, 0x98b3f0), (0x19, 0x9b7428, 0x9b7420, 0x9b75bc)] {
            if (read_i8(player.addr() + o) as i32) < 0xf {
                continue;
            }
            match f {
                1 => {
                    format(0xde1f64, curto, 0xad9c64usize); // R, L, C
                    append_c_string(side, 0xde1f64);
                }
                2 => {
                    if !is_empty_text(side) {
                        append_c_string(side, 0x9b7630);
                    }
                    format(0xde1f64, longo); // Right, Left, Centre
                    append_c_string(side, 0xde1f64);
                }
                4 => {
                    if is_empty_text(side) {
                        append_c_string(side, letra);
                    }
                }
                _ => {}
            }
        }
        // the original repeats the loop (from 15 to 10) while the sides stay empty; the ratings are the same
        volta = volta.wrapping_sub(1);
        if !is_empty_text(side) || volta < 0xa {
            break;
        }
    }
    if f == 2 && !is_empty_text(side) {
        sprintf(text, 0x9a3d8c, buf, side); // "%s (%s)" (dbu)
    } else if !is_empty_text(side) {
        sprintf(text, 0x98f0b8, buf, side); // "%s %s"
    } else {
        write_u8(text, 0); // strcpy in the original (the text may be [0xde1f64] itself)
        append_c_string(text, buf);
    }
    1
}

/// Text pointer is non-null and the text is not empty.
///
/// [pt] Texto não nulo e não vazio (os `repne scasb` da original).
pub(crate) unsafe fn is_nonempty_text(t: usize) -> bool {
    t != 0 && *(t as *const u8) != 0
}

/// `database_format_name_from_ids(first, surname, common, text, size, style, person)`: looks up the name records (60
/// bytes) by index in tables [0xae23f0], [0xae23f4] and [0xae23f8] (−1 or less: null) and builds the text through
/// `database_format_name_from_strings` (with −1 at the end), which is what it returns.
///
/// With `person` > −1 and [0xb63c98]: if a human gave the person a nickname ([0xae23bc] + 110 × person; `has_nickname`) and
/// it matches the names (`database_staff_matches_names`), the nickname (`exe::human_manager_get_nickname`) replaces the common name.
///
/// Null text or size (word) < 0x69: the original (error box). The 0x200-byte locals serve only the box message. Called
/// through the game's address. ~126 callers (news, screens, contracts).
/// Original: `database_format_name_from_ids` (0x53e140, Database.cpp, cdecl).
///
/// [pt] { 0x53e140 } (cdecl, Database.cpp): `database_format_name_from_ids(nome, sobrenome, comum, texto, tamanho, forma,
/// pessoa)`: os registros dos nomes (60 bytes) pelos índices nas tabelas [0xae23f0], [0xae23f4] e [0xae23f8] (−1 ou
/// menos: nulo) e o texto pela 0x53e2e0 (com −1 no fim), que é o que devolve. Com `pessoa` > −1 e [0xb63c98]: se um
/// humano deu apelido à pessoa ([0xae23bc] + 110 × pessoa; 0x600740) e ela confere com os nomes (0x53e990), o apelido
/// (0x6005e0) no lugar do nome comum. Texto nulo ou tamanho (palavra) < 0x69: a original (caixa). As locais de 0x200
/// bytes só servem à mensagem da caixa. As chamadas pelo endereço do jogo. ~126 chamadores (notícias, telas, contratos).
pub(crate) unsafe extern "C" fn database_format_name_from_ids(name: i32, surname: i32, common: i32, text: usize, size: u32, form: u32, person: i32) -> u32 {
    if text == 0 || (size as i16) < 0x69 {
        return original_fn::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_IDS)(name, surname, common, text, size, form, person);
    }
    let record = |table: usize, i: i32| if i > -1 { read_usize(table).wrapping_add((i as usize).wrapping_mul(60)) } else { 0 };
    let (n, s) = (record(G_FIRST_NAME_TABLE, name), record(G_SECOND_NAME_TABLE, surname));
    let apelidos = read_usize(G_HUMAN_MANAGER);
    let mut third = None;
    if person > -1 && apelidos != 0 {
        let p = read_usize(G_STAFF_TABLE).wrapping_add((person as usize).wrapping_mul(110));
        if exe::human_manager_has_nickname(apelidos, p, 0) != 0 && exe::database_staff_matches_names(p, (name) as u32, (surname) as u32, (common) as u32, (0) as usize) != 0 {
            third = Some(exe::human_manager_get_nickname(apelidos, p, 0));
        }
    }
    let c = third.unwrap_or_else(|| record(G_COMMON_NAME_TABLE, common)); // o nome comum
    exe::database_format_name_from_strings(n, s, c, text, size, form, -1)
}

/// `database_format_official_name(official, text, size, style)`: the official's name (first name +4, surname +8) in
/// `text` by style (byte): 1 "%s %s", 2 "%c.%s", 3 "%s, %s" (surname first), 4 "%s, %c", 5 the surname only, 6 "%c %c"
/// (the initials).
///
/// With one of the two missing: the surname (or the first name); in style 6, whichever initial exists. Official of
/// nation +0x16 with id equal to [0x9cf488]: surname only, in any style. Returns 1.
///
/// Null official or text, size (word) < 0x69 or style outside 1..6: the original (error box). Used in the events and
/// the match panel (`event_text`, `commentate_on_panel`, `draw_match_overview_panel`).
/// Original: `database_format_official_name` (0x53eab0, Database.cpp, cdecl).
///
/// [pt] { 0x53eab0 } (cdecl, Database.cpp): `database_format_official_name(oficial, texto, tamanho, forma)`: o nome do
/// oficial (nome +4, sobrenome +8) em `texto` pela forma (byte): 1 "%s %s", 2 "%c.%s", 3 "%s, %s" (sobrenome
/// primeiro), 4 "%s, %c", 5 só o sobrenome, 6 "%c %c" (as iniciais). Faltando um dos dois: o sobrenome (ou o nome);
/// na forma 6, a inicial que houver. Oficial da nação +0x16 com id igual a [0x9cf488]: só o sobrenome, em qualquer
/// forma. Devolve 1. Oficial ou texto nulos, tamanho (palavra) < 0x69 ou forma fora de 1..6: a original (caixa). Nos
/// eventos e no painel da partida (0x6d8d10, 0x6d7b00, 0x71c270).
pub(crate) unsafe extern "C" fn database_format_official_name(oficial: usize, text: usize, size: u32, form: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_OFFICIAL_NAME);
    if oficial == 0 || text == 0 || (size as i16) < 0x69 {
        return o(oficial, text, size, form);
    }
    let (a, b) = (read_usize(oficial + 4), read_usize(oficial + 8));
    let nation = read_usize(oficial + official::NATION);
    let so_surname = nation != 0 && read_usize(nation) == read_usize(KEY_NATION_SPAIN);
    let f = form as u8;
    if !so_surname && !(1..=6).contains(&f) {
        return o(oficial, text, size, form);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let initial = |t: usize| read_i8(t) as i32;
    let copy = |t: usize| {
        let mut k = 0;
        loop {
            let c = read_u8(t + k);
            write_u8(text + k, c);
            if c == 0 {
                break;
            }
            k += 1;
        }
    };
    write_u8(text, 0);
    // what is left when one of the two is missing: the surname, otherwise the first name
    let um_so = || {
        if is_nonempty_text(b) {
            copy(b);
        } else if is_nonempty_text(a) {
            copy(a);
        }
    };
    let two = is_nonempty_text(a) && is_nonempty_text(b);
    if so_surname || f == 5 || (!two && f != 6) {
        um_so();
    } else {
        match f {
            1 => { sprintf(text, 0x98f0b8, a, b); } // "%s %s"
            2 => { sprintf(text, 0x9b7410, initial(a), b); } // "%c.%s"
            3 => { sprintf(text, 0x98f6ac, b, a); } // "%s, %s"
            4 => { sprintf(text, 0x9b7408, b, initial(a)); } // "%s, %c"
            _ => {
                if two {
                    sprintf(text, 0x9b7418, initial(a), initial(b)); // "%c %c"
                } else if is_nonempty_text(b) {
                    sprintf(text, 0x99ec1c, initial(b)); // "%c"
                } else if is_nonempty_text(a) {
                    sprintf(text, 0x99ec1c, initial(a));
                }
            }
        }
    }
    1
}

/// `position_mask_get_text(mask, text, upper)`: the translated position name of the mask (0x66f4e0 with text 0xad9c64;
/// the capitalised form if byte `upper` is not 0), and the gender by language.
///
/// In order: goalkeeper (1), support player (0x260/0x160/0x460), striker (0x40 with 0x700), wing back (0x88/0x808),
/// left winger (0x90/0xa0), right winger (0x810/0x820), central midfielder and attacking midfielder (0x100/0x200/0x400
/// with 0x18 or 0x20), centre back (0x204/0x104/0x404), left back (0x84), right back (0x804), sweeper (2), substitute
/// (0) and defender.
///
/// Returns the gender byte by language (`exe::langlib_get_language`): 2 gives 2 or 5, 5 gives 1, 4 gives 1 or 4, the others 0 (substitute:
/// always 0). Null text: 0. Used in the media (0x72d270) and in the person screen (0x87dd80).
/// Original: `position_mask_get_text` (0x5482f0, Database.cpp, cdecl).
///
/// [pt] { 0x5482f0 } (cdecl, Database.cpp): `position_mask_get_text(mascara, texto, maiuscula)`: o nome da posição da
/// máscara (0x66f4e0 com o texto 0xad9c64; a forma com maiúscula se o byte `maiuscula` não é 0), na ordem: goleiro
/// (1), jogador de apoio (0x260/0x160/0x460), atacante (0x40 com 0x700), ala (0x88/0x808), ponta esquerda (0x90/0xa0),
/// ponta direita (0x810/0x820), meia central e meia-atacante (0x100/0x200/0x400 com 0x18 ou 0x20), zagueiro central
/// (0x204/0x104/0x404), lateral esquerdo (0x84), lateral direito (0x804), líbero (2), reserva (0) e defensor. Devolve o
/// byte do gênero pelo idioma (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4, os outros 0 (reserva: sempre 0). Texto
/// nulo: 0. Na mídia (0x72d270) e na tela da pessoa (0x87dd80).
pub(crate) unsafe extern "C" fn position_mask_get_text(mask: u32, text: usize, capital: u32) -> u8 {
    if text == 0 {
        return 0;
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = exe::langlib_get_language();
    let m = mask;
    let has = |x: u32| m & x == x;
    let middle = m & 0x700 != 0; // 0x100, 0x200 or 0x400
    // (lowercase, uppercase), gender with language 2 and with 4
    let (t, g2, g4): ((usize, usize), u8, u8) = if m & 1 != 0 {
        ((0x9b841c, 0x9b83f0), 2, 1) // goalkeeper
    } else if has(0x260) || has(0x160) || has(0x460) {
        ((0x9b7f60, 0x9b7f30), 5, 4) // support player
    } else if m & 0x40 != 0 && has(0x700) {
        ((0x9b83c4, 0x9b8398), 5, 4) // striker
    } else if has(0x88) || has(0x808) {
        ((0x9b7fbc, 0x9b7f90), 5, 1) // wing back
    } else if has(0x90) || has(0xa0) {
        ((0x9b8018, 0x9b7fe8), 5, 4) // left winger
    } else if has(0x810) || has(0x820) {
        ((0x9b8078, 0x9b8048), 5, 4) // right winger
    } else if middle && m & 0x18 != 0 {
        ((0x9b80dc, 0x9b80a8), 2, 1) // central midfielder
    } else if middle && m & 0x20 != 0 {
        ((0x9b8360, 0x9b8328), 2, 1) // attacking midfielder
    } else if has(0x204) || has(0x104) || has(0x404) {
        ((0x9b8140, 0x9b8110), 2, 1) // centre back
    } else if has(0x84) {
        ((0x9b82fc, 0x9b82d0), 2, 1) // left back
    } else if has(0x804) {
        ((0x9b82a4, 0x9b8278), 2, 1) // right back
    } else if m & 2 != 0 {
        ((0x9b824c, 0x9b8220), 2, 1) // sweeper
    } else if m == 0 {
        ((0x9b81f4, 0x9b81c8), 0, 0) // substitute (gender stays 0)
    } else {
        ((0x9b819c, 0x9b8170), 2, 1) // defender
    };
    format(text, if capital as u8 != 0 { t.1 } else { t.0 }, 0xad9c64);
    if m == 0 {
        return 0;
    }
    match i {
        2 => g2,
        5 => 1,
        4 => g4,
        _ => 0,
    }
}

/// `write_position_name(mask, text, size)`: the translated position name (0x66f4e0; each one writes over the previous;
/// the last one written wins), counting the positions.
///
/// 1 goalkeeper, 2 sweeper, 4 centre back (with 0x200 without 0x80/0x800), defender (with 0x200) or full back, 8 wing
/// back (without 4 and 0x200) or defensive midfielder (with 0x200), 0x10 midfielder, 0x20 with none before (counts 1)
/// attacking midfielder (0x200), winger (0x80 and 0x800), left winger (0x80) or right winger, 0x40 with 0x200 striker;
/// more than 3: "utility player"; none: "player".
///
/// Returns the gender byte of the last one by language (`exe::langlib_get_language`): 2 gives 2 or 5, 5 gives 1, 4 gives 1 or 4; the
/// others do not change it (starts at 0). Size < 30: the original (error box). Used in the media, contracts, player
/// search, scouts and transfers (0x8dc370).
/// Original: `write_position_name` (0x5461b0, Database.cpp, cdecl).
///
/// [pt] { 0x5461b0 } (cdecl, Database.cpp): `write_position_name(mascara, texto, tamanho)`: o nome da posição (0x66f4e0;
/// cada um escreve por cima do anterior) contando as posições: 1 goleiro, 2 líbero, 4 zagueiro central (com 0x200 sem
/// 0x80/0x800), defensor (com 0x200) ou lateral, 8 lateral ofensivo (sem 4 e 0x200) ou volante (com 0x200), 0x10 meia,
/// 0x20 sem nenhuma antes (conta 1) meia-atacante (0x200), ponta (0x80 e 0x800), ponta esquerda (0x80) ou direita, 0x40
/// com 0x200 atacante; mais de 3: "utility player"; nenhuma: "player". Devolve o byte do gênero do último pelo idioma
/// (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4; os outros não mudam (começa em 0). Tamanho < 30: a original (caixa).
/// Na mídia, nos contratos, na busca de jogadores, nos olheiros e nas transferências (0x8dc370).
pub(crate) unsafe extern "C" fn write_position_name(mask: u32, text: usize, size: i32) -> u8 {
    if size < 0x1e {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(IDX_WRITE_POSITION_NAME)(mask, text, size);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = exe::langlib_get_language();
    let m = mask;
    let mut g = 0u8;
    // the text and the gender with language 2 and with 4 (5 always gives 1)
    let mut write = |s: usize, a: u8, c: u8| {
        format(text, s);
        match i {
            2 => g = a,
            5 => g = 1,
            4 => g = c,
            _ => {}
        }
    };
    let mut n = 0i8;
    if m & 1 != 0 {
        n = 1;
        write(0x9b7d5c, 2, 1); // goalkeeper
    }
    if m & 2 != 0 {
        n += 1;
        write(0x9b7d54, 2, 1); // sweeper
    }
    if m & 4 != 0 {
        n += 1;
        if m & 0x200 != 0 {
            if m & 0x80 == 0 && m & 0x800 == 0 {
                write(0x9b7d40, 2, 1); // central defender
            } else {
                write(0x9b7d28, 2, 1); // defender
            }
        } else {
            write(0x9b7d34, 5, 1); // full back
        }
    }
    if m & 8 != 0 {
        n += 1;
        if m & 4 == 0 && m & 0x200 == 0 {
            write(0x9b7d14, 5, 1); // attacking full back
        } else if m & 0x200 != 0 {
            write(0x9b7cfc, 2, 1); // defensive midfielder
        }
    }
    if m & 0x10 != 0 {
        n += 1;
        write(0x9b7cf0, 2, 1); // midfielder
    }
    if m & 0x20 != 0 && n == 0 {
        n = 1;
        if m & 0x200 != 0 {
            write(0x9b7cd8, 2, 1); // attacking midfielder
        } else if m & 0x80 != 0 {
            write(if m & 0x800 != 0 { 0x9b7cd0 } else { 0x9b7cc4 }, 5, 4); // winger, left winger
        } else {
            write(0x9b7cb4, 5, 4); // right winger
        }
    }
    if m & 0x40 != 0 && m & 0x200 != 0 {
        n += 1;
        write(0x9b7cac, 5, 4); // forward
    }
    if n > 3 {
        write(0x9b7c9c, 2, 1); // utility player
    }
    if n == 0 {
        write(0x9b7c94, 2, 1); // player
    }
    g
}

/// `write_player_description(person, position, strengths, text, size, gender)`: the player description (0x66f4e0) by
/// strengths (those of `staff_get_player_strength_flags`) and by the position mask.
///
/// Forward (strengths 0x40): lethal finisher, finisher or pacy goalscorer (by the sum index + strengths % 3), poacher,
/// finisher or goalscorer (% 4), dynamic, powerful, strong, creative, exciting or skilful striker. Midfielder (position
/// 0x10): with 0x40000 "inspirational" and on top "free-scoring" (returns 0); playmaker, ball winner, dynamic,
/// hard-working, reliable, pacy. Strengths 2 (without 4): pacy or exciting full back; 4: pacy or technical defender.
///
/// Otherwise, by age (+0x18 of the person [0xae23bc] + 110 × index): veteran above index % 3 + 33 (without strength 1),
/// rookie below index % 3 + 18 with +0x22 null and the sum % 3 = 0.
///
/// Returns the gender: with language 4, 1, 3 or 4 depending on the text; otherwise the byte `gender` (1 with language
/// 5, which the original also writes into the argument itself; the copy does not write). Size < 30: the original (error
/// box).
/// Original: `write_player_description` (0x5465a0, Database.cpp, cdecl).
///
/// [pt] { 0x5465a0 } (cdecl, Database.cpp): `write_player_description(pessoa, posicao, qualidades, texto, tamanho,
/// genero)`: a descrição do jogador (0x66f4e0) pelas qualidades (as da 0x546ab0) e pela máscara da posição. Atacante
/// (qualidades 0x40): matador letal, matador ou goleador veloz (pela soma índice + qualidades % 3), oportunista,
/// matador ou artilheiro (% 4), atacante dinâmico, de força, forte, criativo, empolgante, habilidoso. Meia (posição
/// 0x10): com 0x40000 "inspirational" e por cima "free-scoring" (devolve 0); armador, ladrão de bola, dinâmico,
/// esforçado, confiável, veloz. Qualidades 2 (sem 4): lateral veloz ou empolgante; 4: defensor veloz ou técnico. Senão,
/// pela idade (+0x18 da pessoa [0xae23bc] + 110 × índice): veterano acima de índice % 3 + 33 (sem a qualidade 1),
/// revelação abaixo de índice % 3 + 18 com +0x22 nulo e a soma % 3 = 0. Devolve o gênero: com o idioma 4, 1, 3 ou 4
/// conforme o texto; senão o byte `genero` (1 com o idioma 5, que a original grava também no próprio argumento; a
/// cópia não grava). Tamanho < 30: a original (caixa).
pub(crate) unsafe extern "C" fn write_player_description(person: i32, position: u32, qualidades: u32, text: usize, size: i32, gender: u32) -> u8 {
    if size < 0x1e {
        return original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(IDX_WRITE_PLAYER_DESCRIPTION)(person, position, qualidades, text, size, gender);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = exe::langlib_get_language();
    let g = if i == 5 { 1 } else { gender as u8 };
    let q = qualidades;
    let has = |m: u32| q & m == m;
    // the text and the gender with language 4
    let write = |s: usize, com_4: u8| {
        format(text, s);
        if i == 4 { com_4 } else { g }
    };
    let sum = person.wrapping_add(q as i32);
    if q & 0x40 != 0 {
        if has(0x2c0000) {
            return write(0x9b7f20, 1); // deadly attacker
        }
        if has(0x240000) {
            return write(if sum % 3 == 1 { 0x9b7f10 } else { 0x9b7efc }, 1); // pacey hitman, pacey goalscorer
        }
        if has(0xc0000) {
            return write(0x9b7ef0, 4); // goalpoacher
        }
        if has(0x40000) {
            if sum % 4 == 1 {
                return write(0x9b7ee8, 1); // hitman
            }
            return write(0x9b7ed4, 3); // prolific striker
        }
        for (m, s) in [(0x102000, 0x9b7ec4), (0x200800, 0x9b7eb0), (0x800, 0x9b7e9c), (0x1200, 0x9b7e88), (0x10000, 0x9b7e74)] {
            if has(m) {
                return write(s, 4); // dynamic, powerhouse, powerful, creative, exciting forward
            }
        }
        if has(0x1000) {
            return write(0x9b7e60, 1); // skilful attacker
        }
        return g;
    }
    if position & 0x10 != 0 {
        if has(0x40000) {
            if has(0x800200) {
                format(text, 0x9b7e44); // inspirational midfielder
            }
            format(text, 0x9b7e2c); // free-scoring midfielder
            return 0;
        }
        if q & 0x200 != 0 && position & 0x800 == 0 && position & 0x80 == 0 {
            return write(0x9b7e20, 1); // playmaker
        }
        for (m, s) in [(0x800000, 0x9b7e14), (0x102100, 0x9b7e00), (0x2000, 0x9b7de8), (0x8000, 0x9b7dd4), (0x200000, 0x9b7dc0)] {
            if has(m) {
                return write(s, 1); // ball-winner, dynamic, hard working, reliable, pacey midfielder
            }
        }
        return g;
    }
    if q & 2 != 0 && q & 4 == 0 {
        if has(0x200000) {
            return write(0x9b7db0, 1); // pacey full-back
        }
        if has(0x10000) {
            return write(0x9b7d9c, 1); // exciting full-back
        }
        return g;
    }
    if q & 4 != 0 {
        if has(0x200000) {
            return write(0x9b7d8c, 1); // pacey defender
        }
        if has(0x1200) {
            return write(0x9b7d78, 1); // cultured defender
        }
        return g;
    }
    if person <= -1 {
        return g;
    }
    let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((person as usize).wrapping_mul(110)));
    let age = ((p.age() as i8) as i32);
    if q & 1 == 0 && age > person % 3 + 0x21 {
        return write(0x9b7d70, 4); // veteran
    }
    if sum % 3 != 0 || q & 1 != 0 {
        return g;
    }
    if age < person % 3 + 0x12 && p.int_apps() == 0 {
        return write(0x9b7d68, 4); // rookie
    }
    g
}

/// Score of colleague `o` as a favourite of `pessoa` at `clube` (filters and score of `staff_is_best_favoured_candidate`/`staff_pick_favoured_team_mate`: contract at
/// the club, reputation, not leaving, stats), or None if he does not qualify.
///
/// [pt] A nota do colega `o` como favorito da `pessoa` no `clube` (os filtros e a nota da 0x548810, iguais aos da 0x548da0),
/// ou `None` se ele não entra. As chamadas pelo endereço do jogo, na ordem da original.
pub(crate) unsafe fn favourite_colleague_score(person: Staff, club: Club, o: usize) -> Option<i32> {
    let o = Staff(o);
    if o.player().is_null() {
        return None;
    }
    let id = read_usize(person.addr()) as i32;
    let oid = read_usize(o.addr()) as i32;
    if oid >= read_usize(G_STAFF_COUNT) as i32 || o == person || oid.wrapping_add(id) % 3 == 0 {
        return None;
    }
    let pref = person.staff_preferences();
    if pref != 0 && (0..3).any(|j| read_usize(pref + preferences::STAFF_DISLIKED_STAFF1 + 4 * j) == o.addr()) {
        return None; // rejected by the person
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), o, club));
    if contract.is_null() {
        return None;
    }
    let rep = read_i16_as_i32(o.player().addr() + 0xb);
    if rep <= 0x109a || rep <= (club.reputation() as i32) - 0x2ee {
        return None;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), o, 0) != 0 || read_i16_as_i32(o.player().addr() + 5) <= 100 {
        return None;
    }
    let l0 = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), o.player(), 6, 0x11, 6.5) as f32 as f64; // fstp dword
    let m1 = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), o.player(), 6, 1, 6.5);
    let data = o.player();
    let mut n = (data.current_reputation() as i32);
    if m1 > read_f32(rdata::F32_5) as f64 {
        let g = read_usize(G_PLAYER_STATS);
        let ratio = || {
            let d = read_f64(g + player_stats_fields::RATED_COUNT);
            if d == 0.0 || d.is_nan() { read_f64(rdata::F64_6_8) } else { read_f64(g + player_stats_fields::RATING_SUM) / d }
        };
        // fcompp: C0 (below or unordered) adds
        if !(ratio() + read_f64(rdata::F64_0_2) >= l0) {
            n += 500;
        } else if !(ratio() + read_f64(rdata::F64_0_1) >= l0) {
            n += 250;
        }
    }
    if pref != 0 && (0..3).any(|j| read_usize(pref + preferences::STAFF_FAVOURITE_STAFF1 + 4 * j) == o.addr()) {
        n += 1250; // already favorite
    }
    let c = (data.consistency() as i32);
    if c == 20 {
        n += 75;
    } else if c > 15 {
        n += 50;
    }
    let a = (o.professionalism() as i32);
    if a > 15 {
        n += 250;
    } else if a < 6 {
        n -= 100;
    }
    n += id.wrapping_add(oid) % 500; // desempate pelos ids (dbs)
    match contract.squad_status() >> 4 {
        1 => n += 750,
        2 => n += 300,
        3 => n += 150,
        _ => {}
    }
    let nation = person.nation();
    if nation != club.nation() && o.nation() == nation {
        if (contract.grievances() as usize) & 0x20020 != 0 {
            n += 1750;
        } else if exe::date_days_since(G_CURRENT_DATE, (person.date_joined_club()) as u32, (person.date_joined_club_leap_year()) as u32) < 350 && ((person.age() as i8) as i32) < 24 {
            n += 250;
        } else {
            n += 125;
        }
    }
    Some(n)
}

/// `staff_is_best_favoured_candidate(person, club, candidate)`: 1 if the candidate passes the favoured team mate
/// filters (those of `staff_pick_favoured_team_mate` without the position mask one) and none of the 50 in the club's squad (+0xd7) has a
/// strictly higher score (the `staff_pick_favoured_team_mate` score); 0 otherwise.
///
/// First, the person's own filters: [0xb74350], id % 6 = r: +0x59 ≥ r + 6, +0x57 ≤ 15 − r, age (`exe::date_days_since`) ≥ id % 28 +
/// 14. The original stores the candidate's score in the argument `person` itself and a float in `candidate` (written
/// before being read); the copy does not. Only `al`. Used in the media (`process_transfer_squad_reactions`).
/// Original: `staff_is_best_favoured_candidate` (0x548810, Database.cpp, cdecl).
///
/// [pt] { 0x548810 } (cdecl, Database.cpp): `staff_is_best_favoured_candidate(pessoa, clube, candidato)`: 1 se o candidato
/// passa nos filtros da escolha do companheiro favorito (os da 0x548da0 sem o da máscara da posição) e nenhum dos 50 do
/// elenco do clube (+0xd7) tem nota estritamente maior; 0 se não. Antes, os da pessoa: [0xb74350], id % 6 = r: +0x59 ≥
/// r + 6, +0x57 ≤ 15 − r, idade (0x54cdb0) ≥ id % 28 + 14. A original guarda a nota do candidato no próprio argumento
/// `pessoa` e um float no `candidato` (gravados antes de lidos); a cópia não. Só `al`. Na mídia (0x758200).
pub(crate) unsafe extern "C" fn staff_is_best_favoured_candidate(person: usize, club: usize, candidate: usize) -> u8 {
    let person = Staff(person);
    let club = Club(club);
    if person.is_null() || club.is_null() || candidate == 0 || read_usize(G_PLAYER_STATS) == 0 {
        return 0;
    }
    let age: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let id = read_usize(person.addr()) as i32;
    let r6 = id % 6;
    if (person.loyalty() as i32) < r6 + 6 || (person.ambition() as i32) > 15 - r6 {
        return 0;
    }
    if age(G_CURRENT_DATE, person.date_joined_club(), person.date_joined_club_leap_year()) < id % 28 + 14 {
        return 0;
    }
    let target = match favourite_colleague_score(person, club, candidate) {
        Some(n) => n,
        None => return 0,
    };
    for k in 0..50 {
        let o = club.squad(k);
        if o == 0 {
            continue;
        }
        if let Some(n) = favourite_colleague_score(person, club, o) {
            if n > target {
                return 0; // there is a better one
            }
        }
    }
    1
}

/// Player stat average (PLAYERSTATS get_stat on [0xb74350]) for the person's player record, given stat type and
/// attribute, weight 6.5.
///
/// [pt] A 0x7c1e30 das notas ([0xb74350]) do jogador (+0x61) da pessoa: `media(p, tipo, atributo)`, peso 6,5.
pub(crate) unsafe fn player_stat_average(person: Staff, kind: u32, attribute: u32) -> f64 {
    exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), person.player().addr(), kind, attribute, 6.5)
}

/// Computed player attribute (`staff_compute_player_attribute`) of the person's player, as a signed byte.
///
/// [pt] O atributo calculado (0x543540) do jogador da pessoa, como byte com sinal.
pub(crate) unsafe fn computed_player_attribute(person: Staff, attribute: u32) -> i32 {
    exe::staff_compute_player_attribute(person, person.player(), attribute) as i8 as i32
}

/// Bit mask of the positions where the player is natural (rating 20) - goalkeeper, wing-backs, defenders, midfielders -
/// plus `soma` when the ten position ratings sum above 110.
///
/// [pt] Os bits das posições em que o jogador é natural (20), comuns às duas: goleiro (+0xf), lateral (+0x11, com +0x19
/// ou com +0x17/+0x18), zagueiros (+0x12/+0x13 sem +0x17/+0x18 a 15), meias (+0x14, +0x15). `bits` = os valores
/// de cada caso na ordem; mais o da soma das dez posições (+0x10..+0x19) acima de 110.
pub(crate) unsafe fn natural_position_bits(j: usize, bits: [u32; 7], sum: u32) -> u32 {
    let b = |k: usize| signed_byte_at(j, k);
    let mut f = 0;
    if b(0xf) == 20 {
        f |= bits[0];
    }
    if b(0x11) == 20 {
        if b(0x19) == 20 {
            f += bits[1];
        }
        if b(0x17) == 20 || b(0x18) == 20 {
            f += bits[2];
        }
    }
    if b(0x12) == 20 && b(0x18) < 15 && b(0x17) < 15 {
        f += bits[3];
    }
    if b(0x13) == 20 && b(0x18) < 15 && b(0x17) < 15 {
        f += bits[4];
    }
    if b(0x14) == 20 {
        f += bits[5];
    }
    if b(0x15) == 20 {
        f += bits[6];
    }
    if (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += sum;
    }
    f
}

/// `staff_get_player_strength_flags(person, other)`: the strength bits of the player (+0x61) for the media, scouts,
/// search and transfers; `other` only needs to be non-null.
///
/// Natural positions (1, 4, 2, 8, 0x10, 0x20, 0x40), 0x1000000 if the nibble of [0xae2d0c] + 79 × id + 0xb passes a
/// draw(3) and `get_future_transfer` ([0xde1f54]) gives 0, 0x80 by the sum of positions; then 19 draws (`exe::utils_random`) against the
/// player's bytes, attributes (`staff_compute_player_attribute`) and averages (`get_stat` on [0xb74350], weight 6.5, x87 in double precision with
/// the `fstp dword` in between): 0x100 (+0x3c), 0x200 (+0x37 plus the adjustment of averages 0x16/0xd/5/1), 0x800000
/// (average 0xe above 8), 0x400 (+0x39, +0x3e and average 0xe or attribute 0x17 with averages 7 and 0x16), 0x800,
/// 0x1000, 0x2000 (+0x2e, +0x40, +0x44), 0x4000, 0x20000, 0x100000, 0x200000, 0x400000 (attributes 4, 0xd, 0x14, 0x15,
/// 1), 0x8000 (+0x21), 0x10000 (attribute 0xb and average 0xf), 0x40000 (reputation and averages 6/2 and 6/1), 0x80000
/// (averages 0x15, 0x10, 2 and 1).
///
/// The original stores floats in the arguments themselves (written before being read); the copy does not. Nulls: 0.
/// Original: `staff_get_player_strength_flags` (0x546ab0, Database.cpp, cdecl).
///
/// [pt] { 0x546ab0 } (cdecl): `staff_get_player_strength_flags(pessoa, outro)`: os bits das qualidades do jogador (+0x61)
/// para a mídia, olheiros, busca e transferências; `outro` só precisa não ser nulo. Posições naturais (1, 4, 2, 8, 0x10,
/// 0x20, 0x40), 0x1000000 se o nibble de [0xae2d0c] + 79 × id + 0xb passa um sorteio(3) e a 0x8cca10 ([0xde1f54]) dá 0,
/// 0x80 pela soma das posições; depois 19 sorteios (0x90cfc0) contra bytes do jogador, atributos (0x543540) e médias
/// (0x7c1e30 em [0xb74350], peso 6,5, x87 em precisão dupla com os `fstp dword` no meio): 0x100 (+0x3c), 0x200 (+0x37
/// mais o ajuste das médias 0x16/0xd/5/1), 0x800000 (média 0xe acima de 8), 0x400 (+0x39, +0x3e e a média 0xe ou o
/// atributo 0x17 com as médias 7 e 0x16), 0x800, 0x1000, 0x2000 (+0x2e, +0x40, +0x44), 0x4000, 0x20000, 0x100000,
/// 0x200000, 0x400000 (atributos 4, 0xd, 0x14, 0x15, 1), 0x8000 (+0x21), 0x10000 (atributo 0xb e a média 0xf), 0x40000
/// (reputação e as médias 6/2 e 6/1), 0x80000 (médias 0x15, 0x10, 2 e 1). A original guarda floats nos próprios
/// argumentos (gravados antes de lidos); a cópia não. Nulos: 0.
pub(crate) unsafe extern "C" fn staff_get_player_strength_flags(person: usize, other: usize) -> u32 {
    let person = Staff(person);
    if other == 0 || person.is_null() {
        return 0;
    }
    let j = || person.player();
    let b = |k: usize| signed_byte_at(j().addr(), k);
    let m = |kind: u32, attribute: u32| player_stat_average(person, kind, attribute);
    let mut f = natural_position_bits(j().addr(), [1, 4, 2, 8, 0x10, 0x20, 0x40], 0);
    let r = random(3);
    let id = read_usize(person.addr()) as u32;
    let nibble = (read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79) as usize).wrapping_add(0xb)) & 0xf) as i32;
    if nibble > r && read_usize(G_TRANSFER_MANAGER) != 0 && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0) == 0 {
        f += 0x1000000;
    }
    if !j().is_null() && (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += 0x80;
    }
    if b(0x3c) > random(3) + 15 {
        f += 0x100;
    }
    let mut adjust = 0i32;
    if m(2, 0x16) > read_f32(rdata::F32_180) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            adjust = ftol(a as f64 - read_f64(rdata::F64_65));
        }
        let x = (m(2, 5) * read_f32(rdata::F32_50) as f64) as f32; // fstp dword;
        adjust = adjust.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37).wrapping_add(adjust) > random(50) {
        f += 0x200;
    }
    let v14 = m(2, 0xe);
    let v14f = v14 as f32; // fst dword
    if v14 > read_f64(rdata::F64_8) {
        f += 0x800000;
    }
    if b(0x39) > random(50) && b(0x3e) > random(50) {
        let mut yes = false;
        if v14f as f64 > read_f64(rdata::F64_0) {
            let r = random(10) + 5;
            yes = ftol(v14f as f64) as i8 as i32 > r;
        }
        if !yes && computed_player_attribute(person, 0x17) > random(5) + 15 {
            let x = m(2, 7) * read_f64(rdata::F64_150); // fstp qword;
            yes = m(2, 0x16) > x;
        }
        if yes {
            f += 0x400;
        }
    }
    for (k, bit) in [(0x2e, 0x800), (0x40, 0x1000), (0x44, 0x2000)] {
        if b(k) > random(5) + 15 {
            f += bit;
        }
    }
    for (a, bit) in [(4, 0x4000), (0xd, 0x20000), (0x14, 0x100000), (0x15, 0x200000), (1, 0x400000)] {
        if computed_player_attribute(person, a) > random(5) + 15 {
            f += bit;
        }
    }
    if b(0x21) > random(3) + 16 {
        f += 0x8000;
    }
    if computed_player_attribute(person, 0xb) > random(4) + 14 {
        let x = m(2, 0xf); // fstp qword
        let r = random(20);
        if !(r as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_7_5) >= x) {
            f += 0x10000;
        }
    }
    let v = m(2, 0x16);
    let limit = 1400 - read_i16_as_i32(j().addr() + 0xb) / 10;
    if !(limit as f64 >= v) {
        let a = m(6, 2) as f32;
        let c = m(6, 1) as f32;
        let r = random(3);
        if !((r as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_0_5)) * c as f64 >= a as f64) {
            f += 0x40000;
        }
    }
    if !(m(2, 0x15) >= read_f64(rdata::F64_3)) && m(2, 0x10) > read_f64(rdata::F64_50) {
        let x = m(2, 2) * read_f64(rdata::F64_3); // fstp qword;
        if m(2, 1) > x {
            f += 0x80000;
        }
    }
    f
}

/// `staff_get_player_weakness_flags(person, club)`: the weakness bits of the player (+0x61) for the media.
///
/// Natural positions (0x20, 0x10, 0x40, 0x80, 0x100, 0x200), 0x800000 by the sum of positions; then the draws
/// (`exe::utils_random`, some with n = −50) against bytes of the person and of the player, attributes (`staff_compute_player_attribute`), averages
/// (`get_stat`, weight 6.5) and the word `exe::injury_get_fatigue` ([0xb63d28]): 1 (the person's +0x5d and averages 0xb, 0xa and 1), 2
/// (+0x3c against a draw of 4, 7, 8 or 11 by +0x34 and `exe::injury_get_fatigue`), 4 (starter, slot 0..10 of `find_selection_slot` on [0xde1d08],
/// with the `off_match_score` score of the `exe::formation_get_slot_position` slot below 150 + draw(40)), 0x400, 0x800, 8, 0x1000, 0x2000, 0x4000, 0x8000,
/// 0x10000, 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000.
///
/// The original stores bytes and floats in the arguments themselves (written before being read; the slot passed to
/// `exe::formation_get_slot_position` is the `club` argument with the slot byte over it, and the copy builds it the same way); the copy does not
/// write into them. Nulls: 0. Used in the media (`exe::media_create_player_media_story`, 0x487c70).
/// Original: `staff_get_player_weakness_flags` (0x5470d0, Database.cpp, cdecl).
///
/// [pt] { 0x5470d0 } (cdecl): `staff_get_player_weakness_flags(pessoa, clube)`: os bits dos defeitos do jogador (+0x61) para
/// a mídia. Posições naturais (0x20, 0x10, 0x40, 0x80, 0x100, 0x200), 0x800000 pela soma das posições; depois os
/// sorteios (0x90cfc0, alguns com n = −50) contra bytes da pessoa e do jogador, atributos (0x543540), médias (0x7c1e30,
/// peso 6,5) e a palavra 0x62c3d0 ([0xb63d28]): 1 (+0x5d da pessoa e as médias 0xb, 0xa e 1), 2 (+0x3c contra um
/// sorteio de 4, 7, 8 ou 11 pelas +0x34 e 0x62c3d0), 4 (titular, vaga 0..10 da 0x8972c0 em [0xde1d08], com a nota
/// 0x6e6340 da casa 0x5b2ae0 abaixo de 150 + sorteio(40)), 0x400, 0x800, 8, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
/// 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000. A original guarda bytes e floats nos próprios argumentos
/// (gravados antes de lidos; a vaga passada à 0x5b2ae0 é o argumento `clube` com o byte da vaga por cima, e a cópia a
/// monta igual); a cópia não grava neles. Nulos: 0. Na mídia (0x72c1f0, 0x487c70).
pub(crate) unsafe extern "C" fn staff_get_player_weakness_flags(person: usize, club: usize) -> u32 {
    let person = Staff(person);
    if club == 0 || person.is_null() {
        return 0;
    }
    let j = || person.player();
    let b = |k: usize| signed_byte_at(j().addr(), k);
    let m = |kind: u32, attribute: u32| player_stat_average(person, kind, attribute);
    let mut f = natural_position_bits(j().addr(), [0, 0x20, 0x10, 0x40, 0x80, 0x100, 0x200], 0x800000);
    if (person.temperament() as i32) < random(4) + 5 && (b(0x1c) > 15 || b(0x25) < 6) {
        let a = (m(2, 0xb) * read_f32(rdata::F32_3) as f64) as f32; // fstp dword;
        let s = (m(2, 0xa) + a as f64) as f32; // fstp dword
        if !(m(2, 1) * read_f32(rdata::F32_0_5) as f64 >= s as f64) {
            f += 1;
        }
    }
    let mut n = 4;
    let w = (exe::injury_get_fatigue(read_usize(G_INJURY), (read_usize(person.addr()) as i32) as i32 as u32) as i32) as i16 as i32;
    if w > random(1000) + 500 {
        n = 8;
    }
    if b(0x34) < random(12) {
        n += 3;
    }
    if b(0x3c) < random(n) + 8 && b(0xf) < 20 {
        f += 2;
    }
    let tactic = read_usize(G_TACTICS);
    let slot = exe::tactics_find_selection_slot(tactic, club, person, 0);
    if (slot as i8) > -1 && (slot as i8) < 11 {
        let formation = exe::tactics_get_club_tactic(tactic, club, 0);
        let home = exe::formation_get_slot_position(formation, (club as u32 & !0xff) | slot as u32, 0); // the argument with the slot byte
        let x = exe::match_man_score_player_for_selection(person, home as u32, 0x800, 0, -1, 0);
        if x < random(40) + 150 {
            f += 4;
        }
    }
    let mut adjust = 0i32;
    if m(2, 0x16) > read_f32(rdata::F32_180) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            adjust = 70 - a;
        }
        let x = (m(2, 5) * read_f32(rdata::F32_NEG_50) as f64) as f32; // fstp dword;
        adjust = adjust.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37) < random(-50).wrapping_add(adjust) {
        f += 0x400;
    }
    if b(0x39) < random(-50) && b(0x3e) < random(-50) {
        let v = m(2, 0xe);
        let vf = v as f32; // fst dword
        if v > read_f64(rdata::F64_0) {
            let r = random(10);
            if (ftol(vf as f64) as i8 as i32) < r {
                f += 0x800;
            }
        }
    }
    let bd = read_usize(G_INJURY);
    if bd != 0 {
        let w = (exe::injury_get_fatigue(bd, (read_usize(person.addr()) as i32) as i32 as u32) as i32) as i16 as i32;
        if w > random(500) + 750 {
            f += 8;
        }
    }
    if b(0x2e) < random(15) {
        f += 0x1000;
    }
    if b(0xf) < 20 { // the outfield players
        if b(0x40) < random(10) {
            f += 0x2000;
        }
        if b(0x44) < random(12) {
            f += 0x4000;
        }
    }
    if b(0x1e) < random(-50) {
        f += 0x8000;
    }
    if b(0xf) < 20 && b(0x25) + b(0x1c) > random(20) + 25 {
        f += 0x10000;
    }
    if b(0x21) < 12 - random(10) {
        f += 0x20000;
    }
    if b(0x11) < 15 && computed_player_attribute(person, 0xb) > random(4) + 14 {
        let v = m(2, 0xf);
        let vf = v as f32; // fst dword
        if v > read_f64(rdata::F64_0) && computed_player_attribute(person, 0xb) as f64 * read_f64(rdata::F64_0_28) > vf as f64 {
            f += 0x40000;
        }
    }
    if b(0x28) > random(5) + 13 {
        f += 0x80000;
    }
    if m(2, 0x16) > read_f32(rdata::F32_180) as f64 {
        let a = computed_player_attribute(person, 0xc);
        let t = (m(2, 0x10) + a as f64 - read_f32(rdata::F32_10) as f64) as f32; // fstp dword;
        if (random(20) + 20) as f64 > t as f64 {
            let x = m(6, 2); // fstp qword
            if m(6, 1) * read_f64(rdata::F64_0_35) > x {
                f += 0x100000;
            }
        }
    }
    if !(m(2, 0x15) >= read_f64(rdata::F64_3)) && m(2, 0x10) > read_f64(rdata::F64_50) {
        f += 0x200000;
    }
    if computed_player_attribute(person, 0x14) < 7 || computed_player_attribute(person, 0x14) < read_i16_as_i32(j().addr() + 5) / 15 {
        f += 0x400000;
    }
    f
}

/// Side-by-side comparison of the original and the copy, with the random draw state; on divergence, re-runs once from
/// the same state and restores the first run's state.
///
/// `off_match_score` (via `exe::injury_get_player_injury`) draws a random number only on the player's first query and caches it: with that cache cold,
/// the original draws an extra number and the copy, which runs afterwards, does not, so the following draws come out
/// different. So, on divergence, it redoes the comparison once from the same state (now warm, the original without
/// counting) and restores the random draw state of the first run, which is the one the game continues with. Returns
/// both results, whether they are equal, and a diagnostic.
///
/// [pt] O lado a lado das duas, com o estado do sorteio. A 0x6e6340 (pela 0x62ed70) sorteia só na primeira consulta do
/// jogador e guarda: com essa memória fria a original sorteia a mais e a cópia, que roda depois, não, e os sorteios
/// seguintes saem diferentes. Então, divergindo, refaz uma vez a comparação a partir do mesmo estado (agora quente, a
/// original sem contar) e devolve o estado do sorteio da primeira, a que o jogo segue.
pub(crate) unsafe fn compare_with_rng_retry(o: impl Fn() -> u32, n: impl Fn() -> u32) -> (u32, u32, bool, String) {
    let before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let (r, x, igual, _, _) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(),
        || crate::rng::without_counting(|| n()));
    if igual && r == x {
        return (r, x, true, String::new());
    }
    let after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    before.write_back();
    let (r2, x2, igual2, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || crate::rng::without_counting(|| o()),
        || crate::rng::without_counting(|| n()));
    after.write_back();
    (r, x, igual2 && r2 == x2, format!("1a {:x} x {:x}; 2a {:x} x {:x} {:x?} x {:x?}", r, x, r2, x2, mo, mr))
}

/// The B team linked to `clube` (club_get_linked_b_team `exe::club_get_linked_b_team` at the game's address, with `ativo` = 1): the
/// partner's record or 0; `eh_a` gets 1 (the club is the A team) or 0 (the club is the B team) when there is a partner.
///
/// When refused, `exe::club_get_linked_b_team` clears the club's mark +0x244: in the comparison the original routine comes first and the
/// copy finds the mark already cleared (also 0).
///
/// [pt] O time B ligado ao `clube` (club_get_linked_b_team 0x540a50 pelo endereço do jogo, com `ativo` = 1): o registro
/// do parceiro ou 0; `eh_a` recebe 1 (o clube é o A) ou 0 (o clube é o B) quando há parceiro. Recusado, a 0x540a50
/// zera a marca +0x244 do clube: na comparação a original vem antes e a cópia acha a marca já zerada (também 0).
pub(crate) unsafe fn database_linked_b_team(club: usize, eh_a: *mut u32) -> usize {
    exe::club_get_linked_b_team(club, eh_a, 1)
}

/// `database_count_club_physios(club)`: how many of the 3 physios (+0x1cf) of the club and of its linked B team
/// (`exe::club_get_linked_b_team`) are valid people (not null, id +0 < [0xae2364], signed). Only `al`.
///
/// Null club: the original (error box and 0). Used in the person screens (staff_screens, 0x8740c8 and 0x87412b).
/// Original: `database_count_club_physios` (0x53c9b0, Database.cpp, cdecl).
///
/// [pt] { 0x53c9b0 } (cdecl, Database.cpp): `database_count_club_physios(clube)`: quantos dos 3 fisioterapeutas
/// (+0x1cf) do clube e do time B ligado (0x540a50) são pessoas válidas (não nulas, id +0 < [0xae2364] com sinal).
/// Só `al`. Clube nulo: a original (caixa e 0). Nas telas da pessoa (staff_screens, 0x8740c8 e 0x87412b).
pub(crate) unsafe extern "C" fn database_count_club_physios(club: usize) -> u8 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_PHYSIOS)(club);
    }
    let validos = |c: Club| {
        (0..3).filter(|k| {
            let p = read_usize(c.addr() + club::PHYSIOS + 4 * k);
            p != 0 && *(p as *const i32) < read_i32(G_STAFF_COUNT)
        }).count() as u8
    };
    let b = database_linked_b_team(club, core::ptr::null_mut());
    validos(Club(club)) + if b != 0 { validos(Club(b)) } else { 0 } // those of the club and of the B team (dbt)
}

/// `club_is_b_team(club)`: 1 if the club has a partner (`exe::club_get_linked_b_team` with `active`) and is its B team (`is_a` = 0),
/// otherwise 0 (the whole eax).
///
/// The local `is_a` is only read when there is a partner, and then `exe::club_get_linked_b_team` has written it. Null club: the original
/// (error box and 0). Used in the person screen (staff_screens, 0x87272d).
/// Original: `club_is_b_team` (0x5448c0, Database.cpp, cdecl).
///
/// [pt] { 0x5448c0 } (cdecl, Database.cpp): `club_is_b_team(clube)`: 1 se o clube tem parceiro (0x540a50 com `ativo`) e
/// é o B dele (`eh_a` = 0), senão 0 (o eax inteiro). A local `eh_a` só é lida com parceiro, e então a 0x540a50 a
/// escreveu. Clube nulo: a original (caixa e 0). Na tela da pessoa (staff_screens, 0x87272d).
pub(crate) unsafe extern "C" fn club_is_b_team(club: usize) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_B_TEAM)(club);
    }
    let mut eh_a = 0u32;
    (database_linked_b_team(club, &mut eh_a) != 0 && eh_a == 0) as u32 // is the B (dbt)
}

/// `club_collect_non_playing_staff(club, out, max)`: puts into `out` the people outside the club's squad and then those
/// of the linked B team (`exe::club_get_linked_b_team` with `active`), in job order: the 6 from +0xbf (manager, assistant, ...) and the 15
/// from +0x19f (coaches, scouts, physios), the non-null ones, up to `max` (signed byte); returns how many in `al`.
///
/// Null club: the original (error box and 0). Used in the info tab of the club screen (club_screen_draw_info_tab
/// 0x460590, 0x4616fa).
/// Original: `club_collect_non_playing_staff` (0x544c70, Database.cpp, cdecl).
///
/// [pt] { 0x544c70 } (cdecl, Database.cpp): `club_collect_non_playing_staff(clube, saida, max)`: põe em `saida` as
/// pessoas fora do elenco do clube e depois as do time B ligado (0x540a50 com `ativo`), na ordem dos cargos: os 6
/// de +0xbf (técnico, auxiliar, ...) e os 15 de +0x19f (treinadores, olheiros, fisioterapeutas), as não nulas, até
/// `max` (byte com sinal); devolve quantas em `al`. Clube nulo: a original (caixa e 0). Na aba de informações da
/// tela do clube (club_screen_draw_info_tab 0x460590, 0x4616fa).
pub(crate) unsafe extern "C" fn club_collect_non_playing_staff(club: usize, out: *mut usize, max: u32) -> u8 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(IDX_CLUB_COLLECT_NON_PLAYING_STAFF)(club, out, max);
    }
    let max = max as i8;
    let mut n = 0i8;
    let b = database_linked_b_team(club, core::ptr::null_mut());
    for c in [club, b] {
        if c == 0 {
            continue;
        }
        for d in (0..6).map(|k| 0xbf + 4 * k).chain((0..15).map(|k| 0x19f + 4 * k)) {
            let p = read_usize(c + d);
            if p != 0 && n < max {
                *out.add(n as usize) = p; // (dbt)
                n += 1;
            }
        }
    }
    n as u8
}

/// `staff_is_player_attribute_visible(viewer, person, attribute)`: 1 if the viewer sees the person's attribute (signed
/// byte); in `al`.
///
/// Visible without any check: `viewer` or `person` null, no fog table ([0xae2a60]), fog turned off ([0x9cc270] = 0), no
/// record for the viewer (fog_of_war `exe::fog_of_war_get_manager_map`, thiscall on the table) or knowledge (`exe::fog_of_war_map_get_knowledge_level`, thiscall on the record)
/// 3. Knowledge 1 or 2: attributes 1, 0x11, 0x15, 0x1a and 0x1b always; the others by the remainder of the person's id
/// by 4 (`m`): the shifted attribute (0, 1, 2 or 3 depending on `m`) divisible by 4 (knowledge 1) or by 2 (knowledge
/// 2), or the next one (m = 0) / the same one (m ≠ 0) divisible by 3. Other knowledge: 0. Only `al`.
///
/// `exe::fog_of_war_map_get_knowledge_level` stores the last club found in [0xae1080] (the same value in both). Used in the search and filter screens
/// (search_screens, SearchFilters 0x807450), the person screen and the club screen.
/// Original: `staff_is_player_attribute_visible` (0x543280, Database.cpp, cdecl).
///
/// [pt] { 0x543280 } (cdecl, Database.cpp): `staff_is_player_attribute_visible(quem_ve, pessoa, atributo)`: 1 se quem vê
/// enxerga o atributo (byte com sinal) da pessoa. Visível sem conta: `quem_ve` ou `pessoa` nulos, sem a tabela da
/// névoa ([0xae2a60]), névoa desligada ([0x9cc270] = 0), sem a ficha de quem vê (fog_of_war 0x5afde0, thiscall na
/// tabela) ou conhecimento (0x5b02f0, thiscall na ficha) 3. Conhecimento 1 ou 2: os atributos 1, 0x11, 0x15, 0x1a e
/// 0x1b sempre; os outros pelo resto do id da pessoa por 4 (`m`): o atributo deslocado (0, 1, 2 ou 3 conforme `m`)
/// divisível por 4 (conhecimento 1) ou por 2 (conhecimento 2), ou o seguinte (m = 0) / o mesmo (m ≠ 0) divisível
/// por 3. Outro conhecimento: 0. Só `al`. A 0x5b02f0 guarda o último clube achado em [0xae1080] (o mesmo valor
/// nas duas). Nas telas de busca e filtros (search_screens, SearchFilters 0x807450), da pessoa e do clube.
pub(crate) unsafe extern "C" fn staff_is_player_attribute_visible(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let table = read_usize(G_FOG_OF_WAR);
    if who_ve == 0 || person == 0 || table == 0 || read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 {
        return 1;
    }
    let f = exe::fog_of_war_get_manager_map(table, who_ve);
    if f == 0 {
        return 1;
    }
    let k = exe::fog_of_war_map_get_knowledge_level(f, person);
    if k == 3 {
        return 1;
    }
    if k != 1 && k != 2 {
        return 0;
    }
    let x = attribute as u8 as i8;
    if matches!(x, 1 | 0x11 | 0x15 | 0x1a | 0x1b) {
        return 1;
    }
    let x = x as i32;
    let divisor = if k == 2 { 2 } else { 4 }; // (dbt)
    let (a, b) = match read_i32(person) % 4 {
        0 => (x, x + 1),
        1 => (x + 1, x + 1),
        2 => (x + 2, x + 2),
        _ => (x + 3, x + 3),
    };
    (a % divisor == 0 || b % 3 == 0) as u8
}

/// `staff_get_player_attribute_for_viewer(viewer, person, attribute)`: the attribute (1 to 0x1f, signed byte) of the
/// person's player record (+0x61) as the viewer sees it.
///
/// With the fog on ([0x9cc270]), a viewer who has a record (`exe::fog_of_war_get_manager_map`) and does not see the attribute (`staff_is_player_attribute_visible`, through
/// the game's address) gets 0xf9 (-7, "unknown"); otherwise the value of staff_compute_player_attribute (`staff_compute_player_attribute`).
/// Only `al`. Null person or attribute out of range: the original (error box and 0xff). Used in the player comparison
/// and the person profile (staff_screens 0x877470, 0x879ac0, 0x87a754).
/// Original: `staff_get_player_attribute_for_viewer` (0x543440, Database.cpp, cdecl).
///
/// [pt] { 0x543440 } (cdecl, Database.cpp): `staff_get_player_attribute_for_viewer(quem_ve, pessoa, atributo)`: o
/// atributo (1 a 0x1f, byte com sinal) do jogador (+0x61) da pessoa como quem vê o enxerga: com a névoa ligada
/// ([0x9cc270]), quem tem ficha (0x5afde0) e não vê o atributo (0x543280, pelo endereço do jogo) recebe 0xf9 (-7,
/// "desconhecido"); senão o valor de staff_compute_player_attribute (0x543540). Só `al`. Pessoa nula ou atributo fora
/// da faixa: a original (caixa e 0xff). Na comparação de jogadores e no perfil da pessoa (staff_screens 0x877470,
/// 0x879ac0, 0x87a754).
pub(crate) unsafe extern "C" fn staff_get_player_attribute_for_viewer(who_ve: usize, person: usize, attribute: u32) -> u8 {
    let person = Staff(person);
    let x = attribute as u8 as i8;
    if person.is_null() || !(1..=0x1f).contains(&x) {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE_FOR_VIEWER)(who_ve, person.addr(), attribute);
    }
    let player = person.player();
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
        if exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), who_ve) != 0 && exe::staff_is_player_attribute_visible(who_ve, person, attribute) == 0 {
            return 0xf9; // desconhecido (dbt)
        }
    }
    (exe::staff_compute_player_attribute(person, player, attribute) as u32) as u8
}

/// The names of the 12 manager attributes (abbreviated, full) of `database_get_non_player_attribute_name`, through the addresses of the texts in the
/// exe. The abbreviated 8th (man management) is a format with the text of [0xad9c64].
///
/// [pt] Os nomes dos 12 atributos de técnico (abreviado, inteiro) da 0x543b10, pelos endereços dos textos no exe; o 8º
/// (gestão de pessoas) abreviado é um formato com o texto de [0xad9c64].
pub(crate) const STAFF_ATTRIBUTE_NAMES: [(usize, usize); 12] = [
    (0x9b7c90, 0x9b7c80), // Ada / Adaptability
    (0x9b7c7c, 0x9b7c64), // CGk / Coaching Goalkeepers
    (0x9b7c60, 0x9b7c44), // Coa / Coaching Outfield Players
    (0x9b7a68, 0x9b7a58), // Det / Determination
    (0x9b7c40, 0x9b7c28), // Jud / Judging Player Ability
    (0x9b7c24, 0x9b7c08), // JPt / Judging Player Potential
    (0x9b7c04, 0x9b7bf0), // Dis / Level of Discipline
    (0x9b7bcc, 0x9b7bbc), // Man<%s - COMMENT - man management> / Man Management
    (0x9b7bb8, 0x9b7bac), // Mot / Motivating
    (0x9b7ba8, 0x9b7b98), // Phy / Physiotherapy
    (0x9b7b94, 0x9b7b80), // Tct / Tactical Knowledge
    (0x9b7b7c, 0x9b7b64), // Yng / Working With Youngsters
];

/// `database_get_non_player_attribute_name(attribute, text, size, abbreviated)`: writes into `text` the name of the
/// manager attribute (1 to 12, signed byte), abbreviated or full, from the translated Langlib text (0x66f4e0, through
/// the game's address); returns 1.
///
/// Attribute out of range, null text or size (signed short) below 100: the original (error box and 0). Used in the
/// search screens (search_screens) and the person profile (staff_screens 0x87ad97).
/// Original: `database_get_non_player_attribute_name` (0x543b10, Database.cpp, cdecl).
///
/// [pt] { 0x543b10 } (cdecl, Database.cpp): `database_get_non_player_attribute_name(atributo, texto, tamanho, abreviado)`:
/// escreve em `texto` o nome do atributo de técnico (1 a 12, byte com sinal), abreviado ou inteiro, pelo texto
/// traduzido do Langlib (0x66f4e0, pelo endereço do jogo); devolve 1. Atributo fora da faixa, texto nulo ou tamanho
/// (curto com sinal) abaixo de 100: a original (caixa e 0). Nas telas de busca (search_screens) e no perfil da
/// pessoa (staff_screens 0x87ad97).
pub(crate) unsafe extern "C" fn database_get_non_player_attribute_name(attribute: u32, text: usize, size: u32, abbreviated: u32) -> u32 {
    let x = attribute as u8 as i8;
    if !(1..=12).contains(&x) || text == 0 || (size as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_NON_PLAYER_ATTRIBUTE_NAME)(attribute, text, size, abbreviated);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    write_u8(text, 0);
    let (curto, longo) = STAFF_ATTRIBUTE_NAMES[x as usize - 1];
    if abbreviated == 0 {
        format(text, longo);
    } else if x == 8 {
        format(text, curto, 0xad9c64usize);
    } else {
        format(text, curto); // (dbt)
    }
    1
}

/// `database_compare_staff_by_surname(a, b)`: qsort comparison of person pointers by the style-4 name ("surname, first
/// name", `database_format_staff_name` into 0x7d0 bytes) through the text comparison `exe::utils_string_compare` (case-insensitive, `compare_text_nocase`).
///
/// A null person counts as the empty text (the original zeroes the first byte of both locals beforehand). Used in the
/// contract offer screen (contract_screens 0x4de5d0).
/// Original: `database_compare_staff_by_surname` (0x53ff60, Database.cpp, cdecl).
///
/// [pt] { 0x53ff60 } (cdecl, Database.cpp): `database_compare_staff_by_surname(a, b)`: comparação do qsort de ponteiros
/// de pessoa pelo nome no estilo 4 ("sobrenome, nome", 0x53dba0 em 0x7d0 bytes) na comparação de textos 0x90c1d0
/// (sem maiúsculas, `compare_text_nocase`); pessoa nula vale o texto vazio (a original zera o primeiro byte das duas
/// locais antes). Na tela de oferta de contrato (contract_screens 0x4de5d0).
pub(crate) unsafe extern "C" fn database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (read_usize(a), read_usize(b));
    if pa != 0 {
        exe::database_format_staff_name(pa, na.as_mut_ptr() as usize, 0x7d0, 4); // "sobrenome, nome" (dbt)
    }
    if pb != 0 {
        exe::database_format_staff_name(pb, nb.as_mut_ptr() as usize, 0x7d0, 4);
    }
    compare_text_nocase(na.as_ptr(), nb.as_ptr())
}

/// `database_compare_clubs_by_reputation(a, b)`: qsort comparison of club pointers by reputation (+0x80, signed short;
/// null club = 0), highest first: `a`'s lower: 1; higher: -1; equal: 0.
///
/// (0x540020, already in Rust, is the same order over ids.) Used in the club's teams menu (0x482070) and the Arrange
/// Friendlies tabs (find_screens 0x5a1480, 0x5a1e10, 0x5a2250).
/// Original: `database_compare_clubs_by_reputation` (0x53ffe0, Database.cpp, cdecl).
///
/// [pt] { 0x53ffe0 } (cdecl, Database.cpp): `database_compare_clubs_by_reputation(a, b)`: comparação do qsort de
/// ponteiros de clube pela reputação (+0x80, curto com sinal; clube nulo = 0), a maior primeiro: a de `a` menor: 1;
/// maior: -1; iguais: 0. (A 0x540020, já em Rust, é a mesma ordem sobre ids.) No menu dos times do clube
/// (0x482070) e nas abas de Escolher Amistosos (find_screens 0x5a1480, 0x5a1e10, 0x5a2250).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let injury_word_0c = |p: usize| {
        let c = Club(read_usize(p));
        if c.is_null() { 0 } else { c.reputation() }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // the one with lower reputation afterwards (dbt)
    } else if ra > rb {
        -1
    } else {
        0
    }
}

/// `database_club_nation_is_eec(club)`: 1 if the club's nation (+0x53) is not null and is in the EEC (+0x7f = 2),
/// otherwise 0 (the whole eax).
///
/// The club is not checked (the original reads its +0x53). Used in the info tab of the club screen (club_screens
/// 0x462070, the foreigners).
/// Original: `database_club_nation_is_eec` (0x53d7d0, Database.cpp, cdecl).
///
/// [pt] { 0x53d7d0 } (cdecl, Database.cpp): `database_club_nation_is_eec(clube)`: 1 se a nação do clube (+0x53) não é
/// nula e está na CEE (+0x7f = 2), senão 0 (o eax inteiro). O clube não é conferido (a original lê o +0x53 dele).
/// Na aba de informações da tela do clube (club_screens 0x462070, os estrangeiros).
pub(crate) unsafe extern "C" fn database_club_nation_is_eec(club: usize) -> u32 {
    let club = Club(club);
    let nation = club.nation();
    (!nation.is_null() && nation.group_membership() == 2) as u32 // in the EEC (dbt)
}

/// `get_regions_sorted_by_name(language)`: fills the global array of the 14 regions (0xae2104) with 1..14, stores
/// `language` (byte) in [0xae232b] and sorts it with the CRT qsort (`exe::crt_qsort`) using the comparison `compare_region_names` (through
/// the game's address); returns the array.
///
/// Used in the search screens (search_screens 0x80970c, 0x814b7e, 0x815d5e: the region filter).
/// Original: `get_regions_sorted_by_name` (0x545dc0, Database.cpp, cdecl).
///
/// [pt] { 0x545dc0 } (cdecl, Database.cpp): `get_regions_sorted_by_name(idioma)`: enche o vetor global de 14 regiões
/// (0xae2104) com 1..14, guarda o `idioma` (byte) em [0xae232b] e o ordena pelo qsort da CRT (0x945333) com a
/// comparação 0x545e00 (pelo endereço do jogo); devolve o vetor. Nas telas de busca (search_screens 0x80970c,
/// 0x814b7e, 0x815d5e: o filtro de região).
pub(crate) unsafe extern "C" fn get_regions_sorted_by_name(language: u32) -> usize {
    for k in 0..14u8 {
        write_u8(G_REGIONS_SORTED_BY_NAME + k as usize, k + 1);
    }
    write_u8(G_REGION_NAME_FORM, language as u8);
    exe::crt_qsort(G_REGIONS_SORTED_BY_NAME, 14, 1, 0x545e00); // (dbt)
    G_REGIONS_SORTED_BY_NAME
}

/// `compare_region_names(a, b)`: qsort comparison of region bytes by name (`get_region_name`, from index.cpp, into 0xc8 bytes,
/// in the language of [0xae232b]) through the text comparison `exe::utils_string_compare` (case-insensitive); null `a`: 1; null `b`: -1.
///
/// `get_region_name` zeroes the first byte of the text before anything else (the locals are never read unset). Called by
/// `get_regions_sorted_by_name`.
/// Original: `compare_region_names` (0x545e00, Database.cpp, cdecl).
///
/// [pt] { 0x545e00 } (cdecl, Database.cpp): `compare_region_names(a, b)`: comparação do qsort de bytes de região pelo
/// nome (0x616f20, do index.cpp, em 0xc8 bytes, no idioma de [0xae232b]) na comparação de textos 0x90c1d0 (sem
/// maiúsculas); `a` nulo: 1; `b` nulo: -1. A 0x616f20 zera o primeiro byte do texto antes de tudo (as locais nunca
/// são lidas sem valor). Pela 0x545dc0.
pub(crate) unsafe extern "C" fn compare_region_names(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let language = read_u8(G_REGION_NAME_FORM) as u32;
    let mut na = [0u8; 0xc8];
    let mut nb = [0u8; 0xc8];
    exe::get_region_name(read_u8(a) as u32, na.as_mut_ptr() as usize, 0xc8, language);
    exe::get_region_name(read_u8(b) as u32, nb.as_mut_ptr() as usize, 0xc8, language);
    compare_text_nocase(na.as_ptr(), nb.as_ptr()) // (dbt)
}

/// `staff_get_opinion_link_staff(person, target)`: the person at the other end of the person's opinion link, or 0.
///
/// In the person's record ([0xae2d0c] + id × 79; null person or id >= [0xae2364]: 0), with a link (+0x27 ≠ -1): if the
/// target is +0x37, the person of index +0x27 ([0xae23bc] + 110 × index); otherwise, if the target is +0x4b, the one of
/// index +0x3b; otherwise 0. Used in the transfers tab of the person profile (staff_screens 0x87dd80).
/// Original: `staff_get_opinion_link_staff` (0x549dd0, Database.cpp, cdecl).
///
/// [pt] { 0x549dd0 } (cdecl, Database.cpp): `staff_get_opinion_link_staff(pessoa, alvo)`: o outro lado do laço de opinião
/// da pessoa: no registro da pessoa ([0xae2d0c] + id × 79; pessoa nula ou id >= [0xae2364]: 0), com laço (+0x27 ≠
/// -1), se o alvo é o +0x37: a pessoa de índice +0x27 ([0xae23bc] + 110 × índice); senão, se o alvo é o +0x4b: a de
/// índice +0x3b; senão 0. Na aba de transferências do perfil da pessoa (staff_screens 0x87dd80).
pub(crate) unsafe extern "C" fn staff_get_opinion_link_staff(person: usize, target: u32) -> usize {
    if person == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let reg = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79));
    let read = |d: usize| read_u32(reg + d);
    let persons = read_usize(G_STAFF_TABLE);
    if read(0x27) == u32::MAX {
        return 0;
    }
    let index = if read(0x37) == target {
        read(0x27)
    } else if read(0x4b) == target {
        read(0x3b) // (dbt)
    } else {
        return 0;
    };
    persons.wrapping_add((index as usize).wrapping_mul(110))
}

/// True if the three-dword list at `lista` contains `quem`.
///
/// [pt] A lista de três dwords em `lista` tem `quem`?
pub(crate) unsafe fn list3_contains(list: usize, who: usize) -> bool {
    (0..3).any(|k| read_usize(list + 4 * k) == who)
}

/// Core of the add-to-preferences routines: if already present do nothing; if in the opposite list remove it from
/// there; else put it in the first free slot, or with `forcar` replace a random slot (rand 3).
///
/// [pt] O miolo das rotinas de acrescentar às preferências: já na lista (`ja`), nada; na lista `outra` (o oposto: favoritos
/// x rejeitados), tira de lá a primeira ocorrência e para; senão entra na primeira vaga de `lista`; sem vaga, só com
/// `forcar`: sorteia (0x90cfc0, 3) a vaga que troca (0, 1 ou 2).
pub(crate) unsafe fn database_add_to_preference_list(list: usize, other: usize, who: usize, ja: bool, force: u32) {
    if ja {
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(other + 4 * k) == who) {
        write_usize(other + 4 * k, 0);
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(list + 4 * k) == 0) {
        write_usize(list + 4 * k, who);
        return;
    }
    if force == 0 {
        return;
    }
    let k = random(3);
    if (0..3).contains(&k) {
        write_usize(list + 4 * k as usize, who);
    }
}

/// Regions compared for the preference routines: RNG state plus the 24 bytes of favourite/disliked people at `base` (if
/// non-null).
///
/// [pt] O estado do sorteio e as 6 pessoas (favoritas e rejeitadas) a partir de `base` (o clube +0x9b ou as preferências
/// +0x1c); `base` nulo: só o sorteio.
pub(crate) unsafe fn preference_regions(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if base != 0 {
        reg.push((base, 24));
    }
    reg
}

/// puts the person among the three favoured staff of the club (+0x9b..+0xa3): already favoured, nothing; disliked
/// (+0xa7..+0xaf), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot
/// (3)
/// Original: `club_add_favoured_staff` (0x547870, Database.cpp, cdecl).
///
/// [pt] { 0x547870 } (cdecl, Database.cpp): `club_add_favoured_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// favoritas do clube (+0x9b/+0x9f/+0xa3): já favorita, nada; rejeitada (+0xa7..+0xaf), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Clube ou pessoa nulos: nada. Nos registros de clubes
/// (0x445f45) e de pessoas (0x86dc30).
pub(crate) unsafe extern "C" fn club_add_favoured_staff(club: usize, person: usize, force: u32) {
    let club = Club(club);
    if club.is_null() || person == 0 {
        return;
    }
    database_add_to_preference_list(club.addr() + club::FAV_STAFF1, club.addr() + club::DIS_STAFF1, person, list3_contains(club.addr() + club::FAV_STAFF1, person), force) // the club’s favourites
}

/// puts the person among the three disliked staff of the club (+0xa7..+0xaf): already disliked, nothing; favoured
/// (+0x9b..+0xa3), stops being; otherwise the first free slot; no free slot and the third argument set, a random slot
/// (3)
/// Original: `club_add_disliked_staff` (0x547940, Database.cpp, cdecl).
///
/// [pt] { 0x547940 } (cdecl, Database.cpp): `club_add_disliked_staff(clube, pessoa, forcar)`: põe a pessoa entre as três
/// rejeitadas do clube (+0xa7/+0xab/+0xaf): já rejeitada, nada; favorita (+0x9b..+0xa3), deixa de ser e para; senão a
/// primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Nas audiências das disputas (0x569ed0), na mídia
/// (0x755340) e em 0x7daf50 (do motor de partidas).
pub(crate) unsafe extern "C" fn club_add_disliked_staff(club: usize, person: usize, force: u32) {
    let club = Club(club);
    if club.is_null() || person == 0 {
        return;
    }
    database_add_to_preference_list(club.addr() + club::DIS_STAFF1, club.addr() + club::FAV_STAFF1, person, list3_contains(club.addr() + club::DIS_STAFF1, person), force) // the club’s rejected ones
}

/// puts the other person among the three favoured staff of the person (preferences +0x1c..+0x24): already favoured,
/// nothing; disliked (+0x28..+0x30), stops being; otherwise the first free slot; no free slot and the third argument
/// set, a random slot (3)
/// Original: `staff_add_favoured_staff` (0x547a10, Database.cpp, cdecl).
///
/// [pt] { 0x547a10 } (cdecl, Database.cpp): `staff_add_favoured_staff(pessoa, outra, forcar)`: põe `outra` entre as três
/// pessoas favoritas da pessoa (preferências +0x65: +0x1c/+0x20/+0x24): já favorita, nada; rejeitada (+0x28..+0x30),
/// deixa de ser e para; senão a primeira vaga; sem vaga, com `forcar`, a vaga sorteada (3). Pessoa, preferências ou
/// outra nulas: nada. Nas relações do elenco (0x5d6d10, diária), registros de clubes (0x445f45), volta da lesão
/// (0x630ee0), mídia e outras.
pub(crate) unsafe extern "C" fn staff_add_favoured_staff(person: usize, other: usize, force: u32) {
    let person = Staff(person);
    if person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || other == 0 {
        return;
    }
    database_add_to_preference_list(pref + preferences::STAFF_FAVOURITE_STAFF1, pref + preferences::STAFF_DISLIKED_STAFF1, other, list3_contains(pref + preferences::STAFF_FAVOURITE_STAFF1, other), force) // the person’s favorites
}

/// `staff_add_disliked_club(person, club, force)`: puts the club among the person's three disliked clubs (preferences
/// +0x65: +0x10/+0x14/+0x18).
///
/// The club already dislikes the person (+0xa7..+0xaf): nothing; the club is a favoured one (+4..+0xc): it stops being
/// one and the routine stops; otherwise the first free slot (without checking whether it is already among the disliked,
/// as in the original); with no free slot, with `force`, a random slot (`exe::utils_random`, 3). Null person, preferences or club:
/// nothing.
///
/// Used in the daily person update (game_daily_staff_update 0x5ce2d0, two calls), `exe::fine_fine_player`, MANAGER_MANAGER (0x69c030)
/// and others.
/// Original: `staff_add_disliked_club` (0x547e70, Database.cpp, cdecl).
///
/// [pt] { 0x547e70 } (cdecl, Database.cpp): `staff_add_disliked_club(pessoa, clube, forcar)`: põe o clube entre os três
/// rejeitados da pessoa (preferências +0x65: +0x10/+0x14/+0x18). O clube já rejeita a pessoa (+0xa7..+0xaf): nada;
/// o clube é favorito (+4..+0xc): deixa de ser e para; senão a primeira vaga (sem conferir se já está entre os
/// rejeitados, como a original); sem vaga, com `forcar`, a vaga sorteada (0x90cfc0, 3). Pessoa, preferências ou clube
/// nulos: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0, duas chamadas), 0x5a5980, MANAGER_MANAGER
/// (0x69c030) e outras.
pub(crate) unsafe extern "C" fn staff_add_disliked_club(person: usize, club: usize, force: u32) {
    let person = Staff(person);
    let club = Club(club);
    if person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || club.is_null() {
        return;
    }
    if (0..3).any(|k| read_usize(club.addr() + club::DIS_STAFF1 + 4 * k) == person.addr()) {
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(pref + preferences::STAFF_FAVOURITE_CLUBS1 + 4 * k) == club.addr()) {
        write_usize(pref + preferences::STAFF_FAVOURITE_CLUBS1 + 4 * k, 0);
        return;
    }
    let list = pref + preferences::STAFF_DISLIKED_CLUBS1; // the rejected clubs
    if let Some(k) = (0..3).find(|&k| read_usize(list + 4 * k) == 0) {
        write_usize(list + 4 * k, club.addr());
        return;
    }
    if force == 0 {
        return;
    }
    let k = random(3);
    if (0..3).contains(&k) {
        write_usize(list + 4 * k as usize, club.addr());
    }
}

/// `staff_fade_staff_relationships(person)`: each of the three favoured people (preferences +0x1c..+0x24) present
/// leaves when a draw (`exe::utils_random`) of (the person's +0x59, signed) + 15 gives 0; each of the three disliked ones
/// (+0x28..+0x30), with a draw of 35 − (+0x5d).
///
/// One draw per occupied slot, in order. Null person or preferences: nothing. Used in the daily person update
/// (game_daily_staff_update 0x5ce2d0).
/// Original: `staff_fade_staff_relationships` (0x547f90, Database.cpp, cdecl).
///
/// [pt] { 0x547f90 } (cdecl, Database.cpp): `staff_fade_staff_relationships(pessoa)`: cada uma das três pessoas favoritas
/// (preferências +0x1c..+0x24) presente sai com o sorteio (0x90cfc0) de (+0x59 da pessoa, com sinal) + 15 dando 0;
/// cada uma das três rejeitadas (+0x28..+0x30), com o de 35 − (+0x5d). Um sorteio por vaga ocupada, na ordem.
/// Pessoa ou preferências nulas: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0).
pub(crate) unsafe extern "C" fn staff_fade_staff_relationships(person: usize) {
    let person = Staff(person);
    if person.is_null() || person.staff_preferences() == 0 {
        return;
    }
    let pref = person.staff_preferences();
    for k in 0..3 {
        let v = pref + preferences::STAFF_FAVOURITE_STAFF1 + 4 * k;
        if read_usize(v) != 0 && random(person.loyalty() as i32 + 15) == 0 { // the favorites
            write_usize(v, 0);
        }
    }
    for k in 0..3 {
        let v = pref + preferences::STAFF_DISLIKED_STAFF1 + 4 * k;
        if read_usize(v) != 0 && random(35 - person.temperament() as i32) == 0 { // the rejected ones
            write_usize(v, 0);
        }
    }
}

/// `staff_get_disliked_staff(person, out, n)`: zeroes the byte `*n` (always, before checking the person) and copies to
/// `out` the disliked people present (preferences +0x28/+0x2c/+0x30), in order, counting in `*n`.
///
/// Null person, preferences or output: only the zero. Returns `n` (the original's eax); callers read only the byte.
/// Used in the daily person update (0x5ce2d0, two calls), MANAGER_MANAGER (0x693850), the media and the person screens.
/// Original: `staff_get_disliked_staff` (0x5480f0, Database.cpp, cdecl).
///
/// [pt] { 0x5480f0 } (cdecl, Database.cpp): `staff_get_disliked_staff(pessoa, saida, n)`: zera o byte `*n` (sempre, antes
/// de conferir a pessoa) e copia para `saida` as pessoas rejeitadas presentes (preferências +0x28/+0x2c/+0x30), na
/// ordem, contando em `*n`. Pessoa, preferências ou saída nulas: só o zero. Devolve `n` (o eax da original); os
/// chamadores leem só o byte. No diário das pessoas (0x5ce2d0, duas chamadas), MANAGER_MANAGER (0x693850), mídia e
/// telas de pessoas.
pub(crate) unsafe extern "C" fn staff_get_disliked_staff(person: usize, out: usize, n: usize) -> usize {
    let person = Staff(person);
    let count = n as *mut i8;
    *count = 0;
    if person.is_null() || person.staff_preferences() == 0 || out == 0 {
        return n;
    }
    let pref = person.staff_preferences();
    for k in 0..3 {
        let p = read_usize(pref + preferences::STAFF_DISLIKED_STAFF1 + 4 * k); // the rejected ones
        if p != 0 {
            *((out as isize + 4 * *count as isize) as *mut usize) = p;
            *count = (*count).wrapping_add(1);
        }
    }
    n
}

/// `staff_get_natural_position_mask(person)`: the mask of the positions where the player (person +0x61) is natural
/// (20).
///
/// No person or player: 0; goalkeeper (+0xf) 20: 1. Otherwise, by the player's signed bytes: +0x18 (0x80; 0x180 with
/// +0x19 > 10), +0x17 (0x800; plus 0x400 with +0x19 > 10), +0x19 (0x700), +0x10 (2), +0x11 (4), +0x12 (0x18), +0x13
/// (0x38), +0x14 (0x30), +0x15 (0x40; plus 0x700 if none of +0x10..+0x14 is 20). With none: the first position from
/// +0x10 to +0x14 at 15 or more gives 2, 4, 0x18, 0x38 or 0x30; +0x15 at 15 or more gives 0x740 (0x40 if any of
/// +0x10..+0x14 is 20, which cannot happen there); otherwise 0x10.
///
/// The original stores three bytes in a local (written before being read) and one in the argument itself; the copy does
/// not. TapeCall at 37 points: the favoured team mate pick (`staff_pick_favoured_team_mate`, in the daily person update), the match engine
/// (`prepare_match`), the media (`warn_squad_of_position_rival`/`process_transfer_squad_reactions`, `exe::media_create_player_media_story`), awards (0x768cf0) and the person screens.
/// Original: `staff_get_natural_position_mask` (0x5481a0, Database.cpp, cdecl).
///
/// [pt] { 0x5481a0 } (cdecl, Database.cpp): `staff_get_natural_position_mask(pessoa)`: a máscara das posições em que o
/// jogador (pessoa +0x61) é natural (20). Sem pessoa ou jogador: 0; goleiro (+0xf) 20: 1. Senão, pelos bytes com sinal
/// do jogador: +0x18 (0x80; 0x180 com +0x19 > 10), +0x17 (0x800; mais 0x400 com +0x19 > 10), +0x19 (0x700), +0x10 (2),
/// +0x11 (4), +0x12 (0x18), +0x13 (0x38), +0x14 (0x30), +0x15 (0x40; mais 0x700 se nenhum de +0x10..+0x14 é 20). Sem
/// nenhum: a primeira posição de +0x10 a +0x14 com 15 ou mais dá 2, 4, 0x18, 0x38 ou 0x30; +0x15 com 15 ou mais dá
/// 0x740 (0x40 se algum de +0x10..+0x14 é 20, o que ali não acontece); senão 0x10. A original guarda três bytes numa
/// local (gravados antes de lidos) e um no próprio argumento; a cópia não. TapeCall em 37 pontos: escolha do companheiro
/// favorito (0x548da0, no diário das pessoas), motor de partidas (0x6b7f20), mídia (0x758010/0x758200, 0x72c1f0),
/// prêmios (0x768cf0) e telas de pessoas.
pub(crate) unsafe extern "C" fn staff_get_natural_position_mask(person: usize) -> u32 {
    let person = Staff(person);
    if person.is_null() {
        return 0;
    }
    let j = person.player();
    if j.is_null() {
        return 0;
    }
    let b = |k: usize| read_i8(j.addr() + k);
    if b(0xf) == 20 {
        return 1;
    }
    let mut m = 0u32;
    if b(0x18) == 20 {
        m = if b(0x19) > 10 { 0x180 } else { 0x80 };
    }
    if b(0x17) == 20 {
        m |= if b(0x19) > 10 { 0xc00 } else { 0x800 };
    }
    if b(0x19) == 20 {
        m |= 0x700;
    }
    for (k, bit) in [(0x10, 2), (0x11, 4), (0x12, 0x18), (0x13, 0x38), (0x14, 0x30)] {
        if b(k) == 20 {
            m |= bit; // the full-back, the centre-back...
        }
    }
    let algum_20 = (0x10..=0x14).any(|k| b(k) == 20);
    if b(0x15) == 20 {
        m |= 0x40;
        if !algum_20 {
            m |= 0x700;
        }
    }
    if m != 0 {
        return m;
    }
    for (k, r) in [(0x10, 2), (0x11, 4), (0x12, 0x18), (0x13, 0x38), (0x14, 0x30)] {
        if b(k) >= 15 {
            return r;
        }
    }
    if b(0x15) >= 15 {
        return if algum_20 { 0x40 } else { 0x740 };
    }
    0x10
}

/// `club_count_squad_with_position_mask(club, mask)`: how many of the 50 in the squad (+0xd7) with a player record
/// (+0x61) have a natural position mask (`staff_get_natural_position_mask`, through the game's address) equal to `mask`.
///
/// Only `al` (the three callers, `warn_squad_of_position_rival` and `process_transfer_squad_reactions`, read the byte). Used in the media.
/// Original: `club_count_squad_with_position_mask` (0x548150, Database.cpp, cdecl).
///
/// [pt] { 0x548150 } (cdecl, Database.cpp): `club_count_squad_with_position_mask(clube, mascara)`: quantos dos 50 do elenco
/// (+0xd7) com jogador (+0x61) têm a máscara da posição natural (0x5481a0, pelo endereço do jogo) igual a `mascara`.
/// Só `al` (os três chamadores, 0x758010 e 0x758200, leem o byte). Na mídia.
pub(crate) unsafe extern "C" fn club_count_squad_with_position_mask(club: usize, mask: u32) -> u8 {
    let club = Club(club);
    let mut n = 0u8;
    for k in 0..50 {
        let p = Staff(club.squad(k));
        if !p.is_null() && !p.player().is_null() && exe::staff_get_natural_position_mask(p) == mask { // the equal ones
            n = n.wrapping_add(1);
        }
    }
    n
}

/// `staff_pick_favoured_team_mate(person, club)`: the squad mate the person picks as favoured, or 0; the one with the
/// highest score.
///
/// Needs the person, the club and [0xb74350]; id % 6 = r: +0x59 ≥ r + 6, +0x57 ≤ 15 − r and age (`exe::date_days_since` on 0xae2c90
/// with +0x3e/+0x42) ≥ id % 28 + 14.
///
/// Each of the 50 (+0xd7) with a player record, id below [0xae2364], not the person herself, (summed ids) % 3 ≠ 0, not
/// disliked by the person (+0x28..+0x30), with +0x5a ≥ 11 or a mask (`staff_get_natural_position_mask`) different from the person's, with a
/// contract (`exe::contract_manager_get_contract_for_club` on [0xae1050]), reputation (the player's +0xb) above 4250 and above (the club's +0x80) − 750,
/// outside `get_future_transfer` ([0xde1f54], 0) and with the player's +5 above 100 gets a score: the reputation; +500/+250 if the
/// average `get_stat`(6, 17, 6.5), rounded to float, beats the ratio of [0xb74350] (+0x10/+0x18, or 6.8) + 0.2/+0.1, when
/// `get_stat`(6, 1, 6.5) is above 5; +1250 if already favoured; +75/+50 by the player's +0x21 (20 / above 15); +250/−100
/// by +0x5b (above 15 / below 6); + (summed ids) % 500; +750/+300/+150 by the contract level (+0x4f >> 4 = 1/2/3); a
/// compatriot (+0x1a) of a person from outside the club's country (+0x53): +1750 with contract +0x45 & 0x20020,
/// otherwise +250 (age below 350 and +0x18 below 24) or +125.
///
/// The one with the highest score is kept (strictly above −10000 and above the previous one). Calls go through the
/// game's address. All locals are written before being read (the original uses the argument itself as a cursor). Used
/// in the daily person update (game_daily_staff_update 0x5ce2d0) and the person screen (0x87dd80).
/// Original: `staff_pick_favoured_team_mate` (0x548da0, Database.cpp, cdecl).
///
/// [pt] { 0x548da0 } (cdecl, Database.cpp): `staff_pick_favoured_team_mate(pessoa, clube)`: o colega do elenco do clube
/// que a pessoa escolhe para favorito, ou 0. Precisa da pessoa, do clube e de [0xb74350]; id % 6 = r: +0x59 ≥ r + 6,
/// +0x57 ≤ 15 − r e a idade (0x54cdb0 em 0xae2c90 com +0x3e/+0x42) ≥ id % 28 + 14. Cada um dos 50 (+0xd7) com jogador,
/// id abaixo de [0xae2364], não a própria pessoa, (ids somados) % 3 ≠ 0, não rejeitado pela pessoa (+0x28..+0x30), com
/// +0x5a ≥ 11 ou máscara (0x5481a0) diferente da da pessoa, com contrato (0x4cf5f0 em [0xae1050]), reputação (+0xb do
/// jogador) acima de 4250 e de (+0x80 do clube) − 750, fora da 0x8cca10 ([0xde1f54], 0) e com +5 do jogador acima de
/// 100 leva nota: a reputação; +500/+250 se a média 0x7c1e30(6, 17, 6,5), arredondada a float, passa a razão de
/// [0xb74350] (+0x10/+0x18, ou 6,8) + 0,2/+0,1, quando a 0x7c1e30(6, 1, 6,5) passa de 5; +1250 se já favorito; +75/+50
/// por +0x21 do jogador (20 / acima de 15); +250/−100 por +0x5b (acima de 15 / abaixo de 6); + (ids somados) % 500;
/// +750/+300/+150 pelo nível do contrato (+0x4f >> 4 = 1/2/3); compatriota (+0x1a) de pessoa de fora do país do clube
/// (+0x53): +1750 com o contrato +0x45 & 0x20020, senão +250 (idade abaixo de 350 e +0x18 abaixo de 24) ou +125. Fica o
/// de nota mais alta (estritamente acima de −10000 e do anterior). As chamadas pelo endereço do jogo. Locais todas
/// gravadas antes de lidas (a original usa o próprio argumento como cursor). No diário das pessoas
/// (game_daily_staff_update 0x5ce2d0) e na tela da pessoa (0x87dd80).
pub(crate) unsafe extern "C" fn staff_pick_favoured_team_mate(person: usize, club: usize) -> usize {
    let person = Staff(person);
    let club = Club(club);
    if person.is_null() || club.is_null() || read_usize(G_PLAYER_STATS) == 0 {
        return 0;
    }
    let age: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let id = read_usize(person.addr()) as i32;
    let r6 = id % 6;
    if (person.loyalty() as i32) < r6 + 6 || person.ambition() as i32 > 15 - r6 {
        return 0;
    }
    if age(G_CURRENT_DATE, person.date_joined_club(), person.date_joined_club_leap_year()) < id % 28 + 14 {
        return 0;
    }
    let (mut melhor, mut nota_melhor) = (0usize, -10000i32);
    for k in 0..50 {
        let o = Staff(club.squad(k));
        if o.is_null() || o.player().is_null() {
            continue;
        }
        let oid = read_usize(o.addr()) as i32;
        if oid >= read_usize(G_STAFF_COUNT) as i32 || o == person || oid.wrapping_add(id) % 3 == 0 {
            continue;
        }
        let pref = person.staff_preferences();
        if pref != 0 && (0..3).any(|j| read_usize(pref + preferences::STAFF_DISLIKED_STAFF1 + 4 * j) == o.addr()) {
            continue;
        }
        if o.pressure() < 11 && exe::staff_get_natural_position_mask(o) == exe::staff_get_natural_position_mask(person) {
            continue;
        }
        let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), o, club));
        if contract.is_null() {
            continue;
        }
        let rep = read_i16(o.player().addr() + 0xb) as i32;
        if rep <= 0x109a || rep <= club.reputation() as i32 - 0x2ee {
            continue;
        }
        if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), o, 0) != 0 || read_i16(o.player().addr() + 5) <= 100 {
            continue;
        }
        let l0 = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), o.player(), 6, 0x11, 6.5) as f32 as f64; // fstp dword
        let m1 = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), o.player(), 6, 1, 6.5);
        let data = o.player();
        let mut rating = data.current_reputation() as i32;
        if m1 > read_f32(rdata::F32_5) as f64 {
            let g = read_usize(G_PLAYER_STATS);
            let ratio = || {
                let d = read_f64(g + player_stats_fields::RATED_COUNT);
                if d == 0.0 || d.is_nan() { read_f64(rdata::F64_6_8) } else { read_f64(g + player_stats_fields::RATING_SUM) / d }
            };
            // fcompp: C0 (below or unordered) adds
            if !(ratio() + read_f64(rdata::F64_0_2) >= l0) {
                rating += 500;
            } else if !(ratio() + read_f64(rdata::F64_0_1) >= l0) {
                rating += 250;
            }
        }
        if pref != 0 && (0..3).any(|j| read_usize(pref + preferences::STAFF_FAVOURITE_STAFF1 + 4 * j) == o.addr()) {
            rating += 1250; // already favorite
        }
        let c = data.consistency();
        if c == 20 {
            rating += 75;
        } else if c > 15 {
            rating += 50;
        }
        let a = o.professionalism();
        if a > 15 {
            rating += 250;
        } else if a < 6 {
            rating -= 100;
        }
        rating += oid.wrapping_add(id) % 500; // o desempate pelos ids
        match contract.squad_status() >> 4 {
            1 => rating += 750,
            2 => rating += 300,
            3 => rating += 150,
            _ => {}
        }
        let nation = person.nation();
        if nation != club.nation() && o.nation() == nation {
            if (contract.grievances() as usize) & 0x20020 != 0 {
                rating += 1750;
            } else if age(G_CURRENT_DATE, person.date_joined_club(), person.date_joined_club_leap_year()) < 350 && (person.age() as i8) < 24 {
                rating += 250;
            } else {
                rating += 125;
            }
        }
        if rating > nota_melhor {
            melhor = o.addr();
            nota_melhor = rating;
        }
    }
    melhor
}

/// Id (+0) of the club's nation (+0x53); None if club or nation is null.
///
/// [pt] O id (+0) da nação do `clube` (+0x53); clube ou nação nulos: None.
pub(crate) unsafe fn database_club_nation_id(club: Club) -> Option<u32> {
    if club.is_null() {
        return None;
    }
    let nation = club.nation().addr();
    if nation == 0 {
        return None;
    }
    Some(read_u32(nation))
}

/// `club_nation_is_not_argentina(club)`: 1 if the club's nation (+0x53) does not have the id of the Argentina key
/// 0x9cf218; null club or nation: 0.
///
/// The answer goes in `al` and the rest of eax is that of the id (`setne al` after `mov eax, [eax]`), as in the
/// original. In the career: discipline (0x55e300), the daily manager update (manager_manager 0x6ab800) and the news
/// (0x782f70).
/// Original: `club_nation_is_not_argentina` (0x5487b0, Database.cpp, cdecl).
///
/// [pt] { 0x5487b0 } (cdecl, Database.cpp): `club_nation_is_not_argentina(clube)`: 1 se a nação do clube não tem o id
/// da chave Argentina 0x9cf218; clube ou nação nulos: 0. A resposta vai em `al` e o resto do eax é o do id (`setne
/// al` depois do `mov eax, [eax]`), como na original. Na carreira: a disciplina (0x55e300), o diário dos técnicos
/// (manager_manager 0x6ab800) e as notícias (0x782f70).
pub(crate) unsafe extern "C" fn club_nation_is_not_argentina(club: usize) -> u32 {
    match database_club_nation_id(Club(club)) {
        None => 0,
        Some(id) => (id & 0xffff_ff00) | (id != read_u32(KEY_NATION_ARGENTINA)) as u32, // a Argentina
    }
}

/// `club_nation_not_argentina_france_croatia(club)`: 1 if the club's nation does not have the id of Argentina 0x9cf218,
/// France 0x9cf300 or Croatia 0x9cf2ac; null club or nation: 0.
///
/// In `al`; the rest of eax is that of the id. In the career, through the daily manager update (manager_manager
/// 0x6ab800).
/// Original: `club_nation_not_argentina_france_croatia` (0x5487d0, Database.cpp, cdecl).
///
/// [pt] { 0x5487d0 } (cdecl, Database.cpp): `club_nation_not_argentina_france_croatia(clube)`: 1 se a nação do clube não
/// tem o id de Argentina 0x9cf218, França 0x9cf300 nem Croácia 0x9cf2ac; clube ou nação nulos: 0. Em `al`, o resto
/// do eax é o do id. Na carreira, pelo diário dos técnicos (manager_manager 0x6ab800).
pub(crate) unsafe extern "C" fn club_nation_not_argentina_france_croatia(club: usize) -> u32 {
    match database_club_nation_id(Club(club)) {
        None => 0,
        Some(id) => {
            let keys = [KEY_NATION_ARGENTINA, KEY_NATION_FRANCE, KEY_NATION_CROATIA]; // Argentina, France, Croatia
            (id & 0xffff_ff00) | (!keys.iter().any(|&k| read_u32(k) == id)) as u32
        }
    }
}

/// 1 if the nation's id matches the id of any of the given KEY_* nations, else 0.
///
/// [pt] 1 se o id (+0) da `nacao` (não nula; a original não confere) é o de uma das `chaves`, senão 0 (o eax inteiro).
pub(crate) unsafe fn nation_is_one_of(nation: usize, keys: &[usize]) -> u32 {
    let id = read_u32(nation);
    keys.iter().any(|&k| read_u32(k) == id) as u32
}

/// `database_is_melanesian_nation(nation)`: 1 if the nation has the id of one of the keys 0x9cf41c, 0x9cf478, 0x9cf530,
/// 0x9cf2f8, 0x9cf500 (the Melanesian ones), otherwise 0.
///
/// Used in the Oceania cups (oceania_club_champ `oceania_club_champ_select_qualified_clubs`, oceania_nations 0x799e30).
/// Original: `database_is_melanesian_nation` (0x53d900, Database.cpp, cdecl).
///
/// [pt] { 0x53d900 } (cdecl, Database.cpp): `database_is_melanesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (as melanésias), senão 0. Nas copas da Oceania
/// (oceania_club_champ 0x798220, oceania_nations 0x799e30).
pub(crate) unsafe extern "C" fn database_is_melanesian_nation(nation: usize) -> u32 {
    nation_is_one_of(nation, &[KEY_NATION_PAPUA_NEW_GUINEA, KEY_NATION_SOLOMON_ISLANDS, KEY_NATION_NEW_CALEDONIA, KEY_NATION_FIJI, KEY_NATION_VANUATU]) // Melanesian
}

/// `database_is_polynesian_nation(nation)`: 1 if the nation has the id of one of the keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4,
/// 0x9cf2a4, 0x9cf514 (the Polynesian ones), otherwise 0 (the nation is not checked).
///
/// Used in the Oceania cups (oceania_club_champ `oceania_club_champ_select_qualified_clubs`, oceania_nations 0x799e30).
/// Original: `database_is_polynesian_nation` (0x53d940, Database.cpp, cdecl).
///
/// [pt] { 0x53d940 } (cdecl, Database.cpp): `database_is_polynesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (as polinésias), senão 0 (a nação não é conferida). Nas
/// copas da Oceania (oceania_club_champ 0x798220, oceania_nations 0x799e30).
pub(crate) unsafe extern "C" fn database_is_polynesian_nation(nation: usize) -> u32 {
    let id = read_u32(nation);
    let keys = [KEY_NATION_TONGA, KEY_NATION_AMERICAN_SAMOA, KEY_NATION_TAHITI, KEY_NATION_COOK_ISLANDS, KEY_NATION_WESTERN_SAMOA]; // Polynesian
    keys.iter().any(|&k| read_u32(k) == id) as u32
}

/// `database_get_coaching_job_name(job, gender)`: the job name (signed byte) in the fixed text 0xae1980, through the
/// jump table 0x53acc8: 5 and 12 "Manager", 6 and 13 "Assistant Manager", 8 and 15 "Coach" (translated by 0x66f4e0);
/// the others, empty text.
///
/// With `gender` not null and the language (`exe::langlib_get_language`) 4, writes into it 4 (manager, assistant) or 1 (coach). Returns
/// 0xae1980. Used in the regeneration (player_regen 0x7a9650).
/// Original: `database_get_coaching_job_name` (0x53ac40, Database.cpp, cdecl).
///
/// [pt] { 0x53ac40 } (cdecl, Database.cpp): `database_get_coaching_job_name(cargo, genero)`: o nome do cargo (byte com
/// sinal) no texto fixo 0xae1980, pela tabela de saltos 0x53acc8: 5 e 12 "Manager", 6 e 13 "Assistant Manager",
/// 8 e 15 "Coach" (traduzidos pela 0x66f4e0); os demais, texto vazio. Com `genero` não nulo e o idioma (0x66f4d0)
/// 4, grava nele 4 (técnico, auxiliar) ou 1 (treinador). Devolve 0xae1980. Na regeneração (player_regen 0x7a9650).
pub(crate) unsafe extern "C" fn database_get_coaching_job_name(job: u32, gender: usize) -> usize {
    let (texto, marca) = match (job as i8 as i32).wrapping_sub(5) {
        0 | 7 => (0x9b6440usize, 4u8), // "Manager"
        1 | 8 => (0x9b642c, 4),        // "Assistant Manager"
        3 | 10 => (0x9b6448, 1),       // "Coach"
        _ => {
            write_u8(G_COACHING_JOB_NAME_BUFFER, 0);
            return G_COACHING_JOB_NAME_BUFFER;
        }
    };
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    format(G_COACHING_JOB_NAME_BUFFER, texto);
    if gender != 0 && exe::langlib_get_language() == 4 {
        write_u8(gender, marca);
    }
    G_COACHING_JOB_NAME_BUFFER
}

/// `database_format_years_between_dates(end, start, text, size)`: the full years between the dates (+0 day of year, +2
/// year, words): year(end) − year(start), minus 1 if the start day is later (signed); written into `text` with "%d"
/// (0x98806c) as an unsigned word (a negative becomes 65535). Returns 1.
///
/// Null text, size < 5 or one of the dates null: the original (error box and 0). Used in the club and competition
/// records screens (0x44aa20, 0x4a5690, 0x4a7f50).
/// Original: `database_format_years_between_dates` (0x540340, Database.cpp, cdecl).
///
/// [pt] { 0x540340 } (cdecl, Database.cpp): `database_format_years_between_dates(fim, inicio, texto, tamanho)`: os anos
/// completos entre as datas (+0 dia do ano, +2 ano, palavras): ano(fim) − ano(início), menos 1 se o dia do início é
/// maior (com sinal); escrito em `texto` com "%d" (0x98806c) como palavra sem sinal (negativo vira 65535). Devolve 1.
/// Texto nulo, tamanho < 5 ou uma das datas nula: a original (caixa e 0). Nas telas de recordes do clube e das
/// competições (0x44aa20, 0x4a5690, 0x4a7f50).
pub(crate) unsafe extern "C" fn database_format_years_between_dates(end: usize, start: usize, text: usize, size: i32) -> u32 {
    if text == 0 || size < 5 || end == 0 || start == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(IDX_DATABASE_FORMAT_YEARS_BETWEEN_DATES)(end, start, text, size);
    }
    let mut years = read_i16(end + 2).wrapping_sub(read_i16(start + 2));
    if read_i16(start + 0) > read_i16(end + 0) {
        years = years.wrapping_sub(1); // the birthday has not come yet
    }
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    fmt(text, 0x98806c, years as u16 as u32);
    1
}

/// Case-insensitive string compare (Utils string compare `exe::utils_string_compare` with (a, b, 1, 0)) used by the Database.cpp sorts.
///
/// [pt] A comparação de textos sem maiúsculas 0x90c1d0(a, b, 1, 0) que as ordens do Database.cpp usam.
pub(crate) unsafe fn compare_text_nocase(a: *const u8, b: *const u8) -> i32 {
    exe::utils_string_compare(a, b, 1, 0)
}

/// `database_compare_clubs_by_short_name(a, b)`: qsort comparison of club pointers by short name (+0x38) through the
/// text comparison `exe::utils_string_compare` (case-insensitive); a null club counts as the empty text.
///
/// The original copies both names to the stack before comparing; the copy compares in place (`exe::utils_string_compare` only reads). Used
/// in the club and search screens, in Setup (0x822260) and in the UEFA seeding (0x906410).
/// Original: `database_compare_clubs_by_short_name` (0x540080, Database.cpp, cdecl).
///
/// [pt] { 0x540080 } (cdecl, Database.cpp): `database_compare_clubs_by_short_name(a, b)`: comparação do qsort de
/// ponteiros de clube pelo nome curto (+0x38) na comparação de textos 0x90c1d0 (sem maiúsculas); clube nulo vale o
/// texto vazio. A original copia os dois nomes para a pilha antes de comparar; a cópia compara no lugar (a 0x90c1d0
/// só lê). Nas telas de clubes e de busca, no Setup (0x822260) e na semeadura da UEFA (0x906410).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_short_name(a: usize, b: usize) -> i32 {
    let name = |p: usize| {
        let c = Club(read_usize(p));
        if c.is_null() { b"\0".as_ptr() } else { (c.addr() + club::SHORT_NAME) as *const u8 } // o nome curto
    };
    compare_text_nocase(name(a), name(b))
}

/// `database_compare_staff_by_full_name(a, b)`: qsort comparison of person pointers by the style-1 name ("first name
/// surname", `database_format_staff_name` into 0x7d0 bytes) through the text comparison `exe::utils_string_compare` (case-insensitive); a null person counts
/// as the empty text.
///
/// Used in the menu (0x7612c0) and the transfer manager (0x8e55f0).
/// Original: `database_compare_staff_by_full_name` (0x540270, Database.cpp, cdecl).
///
/// [pt] { 0x540270 } (cdecl, Database.cpp): `database_compare_staff_by_full_name(a, b)`: comparação do qsort de
/// ponteiros de pessoa pelo nome no estilo 1 ("nome sobrenome", 0x53dba0 em 0x7d0 bytes) na comparação de textos
/// 0x90c1d0; pessoa nula vale o texto vazio. No menu (0x7612c0) e no gerente de transferências (0x8e55f0).
pub(crate) unsafe extern "C" fn database_compare_staff_by_full_name(a: usize, b: usize) -> i32 {
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (read_usize(a), read_usize(b));
    if pa != 0 {
        exe::database_format_staff_name(pa, na.as_mut_ptr() as usize, 0x7d0, 1); // "nome sobrenome"
    }
    if pb != 0 {
        exe::database_format_staff_name(pb, nb.as_mut_ptr() as usize, 0x7d0, 1);
    }
    compare_text_nocase(na.as_ptr(), nb.as_ptr())
}

/// `database_staff_matches_names(person, first, surname, common, date)`: 1 if the person, who must have a player record
/// (+0x61) or a non-player record (+0x69), matches: each non-null name (+4, +8, +0xc) has the requested id (+0x33) and,
/// with `date`, the birth date (+0x10 day, +0x12 year) is the same (`exe::date_operator_3d_3d`); otherwise 0.
///
/// Null person: the original (error box and 0). Used by screens, regeneration and transfers (~137 callers).
/// Original: `database_staff_matches_names` (0x53e990, Database.cpp, cdecl).
///
/// [pt] { 0x53e990 } (cdecl, Database.cpp): `database_staff_matches_names(pessoa, nome, sobrenome, comum, data)`: 1 se a
/// pessoa, que tem de ter registro de jogador (+0x61) ou de não jogador (+0x69), confere: cada nome não nulo (+4, +8,
/// +0xc) tem o id (+0x33) pedido e, com `data`, a data de nascimento (+0x10 dia, +0x12 ano) é a mesma (0x4a7f20);
/// senão 0. Pessoa nula: a original (caixa e 0). Telas, regeneração e transferências (~137 chamadores).
pub(crate) unsafe extern "C" fn database_staff_matches_names(person: usize, name: u32, surname: u32, common: u32, data: usize) -> u32 {
    let person = Staff(person);
    if person.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(IDX_DATABASE_STAFF_MATCHES_NAMES)(person.addr(), name, surname, common, data);
    }
    if person.player().is_null() && person.non_player().is_null() {
        return 0;
    }
    for (d, id) in [(4, name), (8, surname), (0xc, common)] {
        let n = read_usize(person.addr() + d);
        if n != 0 && read_u32(n + 0x33) != id {
            return 0; // the name does not match
        }
    }
    if data != 0 {
        if person.date_of_birth_year() != read_u16(data + 2) || (person.date_of_birth() as u16) != read_u16(data + 0) {
            return 0; // another date of birth
        }
    }
    1
}

/// Orders two clubs by the f64 coordinate at offset d of their stadium's (+0x69) city (+0x38): -1, 0 or 1; a missing
/// city uses [0x9689a0] for ca and 0 for cb.
///
/// [pt] A ordem dos clubes `ca` e `cb` (não nulos) pela coordenada f64 em `d` da cidade (+0x38) do estádio (+0x69); sem
/// estádio ou sem cidade, a de `ca` vale [0x9689a0] e a de `cb` vale 0. −1 se a de `ca` é menor (ou NaN: o `fcom`
/// com C0), 1 se maior, 0 se igual.
pub(crate) unsafe fn order_by_city_coordinate(ca: usize, cb: usize, d: usize) -> i32 {
    let city = |c: Club| {
        let e = c.stadium().addr();
        if e == 0 { 0 } else { read_usize(e + stadium::CITY) }
    };
    let (xa, xb) = (city(Club(ca)), city(Club(cb)));
    let x = if xa != 0 { read_f64(xa + d) } else { read_f64(rdata::F64_0) };
    let y = if xb != 0 { read_f64(xb + d) } else { 0.0 };
    if !(x >= y) {
        -1
    } else if x > y {
        1
    } else {
        0
    }
}

/// `database_compare_clubs_by_latitude(a, b)`: qsort comparison of club pointers by the latitude (+0x23, f64) of the
/// city (+0x38) of the stadium (+0x69), lowest first; no city: 0.
///
/// Null club: the original (error box and 0). Used in the cups drawn by region (eng_auto_cup `select_section_clubs`, cro_a1, fin_prm,
/// ita_ser_a, nor_prm, por_prm, spa_first, swe_cup).
/// Original: `database_compare_clubs_by_latitude` (0x53fd40, Database.cpp, cdecl).
///
/// [pt] { 0x53fd40 } (cdecl, Database.cpp): `database_compare_clubs_by_latitude(a, b)`: comparação do qsort de ponteiros
/// de clube pela latitude (+0x23, f64) da cidade do estádio, a menor primeiro. Clube nulo: a original (caixa e 0).
/// Nas copas sorteadas por região (eng_auto_cup 0x56b730, cro_a1, fin_prm, ita_ser_a, nor_prm, por_prm, spa_first,
/// swe_cup).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_usize(a), read_usize(b));
    if ca == 0 || cb == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LATITUDE)(a, b);
    }
    order_by_city_coordinate(ca, cb, 0x23) // a latitude
}

/// the same comparison by longitude (+0x2b, f64)
/// Original: `database_compare_clubs_by_longitude` (0x53fe50, Database.cpp, cdecl).
///
/// [pt] { 0x53fe50 } (cdecl, Database.cpp): `database_compare_clubs_by_longitude(a, b)`: a mesma comparação pela
/// longitude (+0x2b, f64). Clube nulo: a original (caixa e 0). Nas copas sorteadas por região (bel_first, hol_cup,
/// ita_c_cup, spa_first, tur_first).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_longitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_usize(a), read_usize(b));
    if ca == 0 || cb == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LONGITUDE)(a, b);
    }
    order_by_city_coordinate(ca, cb, 0x2b) // a longitude
}

/// Counts how many of the 50 squad members (+0xd7) count for the three squad-strength queries by the /65 limit.
///
/// A member counts when: non-null person with a player record (+0x61) of `posicao`, no serious injury (INJURY `get_player_injury_status`
/// on [0xb63d28] ≠ 2), id (+0) < [0xae2364] (signed) and current ability (+5 of the player, signed short) above
/// `limite`.
///
/// The order of the tests is the original routine's (`get_player_injury_status` is only called for those of the position). 8-bit counter
/// (at most 50).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) contam para as três consultas da força do elenco pelo limite /65: pessoa não
/// nula com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364]
/// (com sinal) e habilidade atual (+5 do jogador, curto com sinal) acima do `limite`. A ordem dos testes é a da
/// original (a 0x62c400 só é chamada para os da posição). Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_above_ability(club: Club, limit: i32, position: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| read_usize(a);
    let mut n = 0u8;
    for k in 0..50 {
        let p = Staff(rd(club.addr() + club::SQUAD + 4 * k));
        if p.is_null() {
            continue;
        }
        let j = p.player();
        if j.is_null() || !position(j.addr()) || exe::injury_get_player_injury_status(rd(G_INJURY), p) == 2 {
            continue;
        }
        if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if read_i16(j.addr() + 5) as i32 > limit {
            n += 1;
        }
    }
    n
}

/// Ability threshold from club reputation: reputation/65 + 40, clamped to 40..teto.
///
/// [pt] O limite de habilidade das três: reputação do clube (+0x80, curto com sinal) / 65 + 40, entre 40 e `teto`.
pub(crate) unsafe fn squad_ability_threshold(club: Club, ceiling: i32) -> i32 {
    (club.reputation() as i32 / 65 + 40).clamp(40, ceiling)
}

/// The club's average per game of stat byte `campo` (goals per game) in its competition (+0x57).
///
/// The club's stats (COMP_STATS `exe::comp_stats_find_club_entry`, thiscall on +8 of the competition [[0xadadfc] + 4·id]); `None` with no
/// competition, no stats or no games (+4); otherwise byte `campo` / games (unsigned integers).
///
/// `exe::comp_stats_find_club_entry` sorts the competition table on the first call (mark +0x18) and stores the competition in [0xadae10]: in the
/// comparison the original comes first and the copy finds the table already sorted.
///
/// [pt] A média de gols por jogo do clube na competição (+0x57): a estatística do clube (COMP_STATS 0x4a4850, thiscall
/// no +8 da competição [[0xadadfc] + 4·id]); `None` sem competição, sem estatística ou sem jogos (+4); senão o byte
/// `campo` / jogos (inteiros sem sinal). A 0x4a4850 ordena a tabela da competição na primeira chamada (marca +0x18)
/// e guarda a competição em [0xadae10]: na comparação a original vem antes e a cópia acha a tabela já ordenada.
pub(crate) unsafe fn club_goals_per_game(club: Club, field: usize) -> Option<i32> {
    let rd = |a: usize| read_usize(a);
    let comp = club.division().addr();
    if comp == 0 {
        return None;
    }
    let c = rd(rd(G_COMP_OBJECT_TABLE).wrapping_add(rd(comp).wrapping_mul(4)));
    if c == 0 || rd(c + 8) == 0 {
        return None;
    }
    let e = exe::comp_stats_find_club_entry(rd(c + 8), club);
    if e == 0 {
        return None;
    }
    let games = read_u8(e + 4) as i32;
    if games == 0 {
        return None;
    }
    Some(read_u8(e + field) as i32 / games)
}

/// `club_has_enough_defenders(club)`: whether the club has enough good defenders. Null club: 0.
///
/// If the club concedes more than 1.35 goals a game ([0x969588]; goals conceded +0x6a of the statistics / games): 0.
/// Otherwise 1 when more than 2 defenders (position +0x11 = 20) without serious injury are above the ability limit
/// (reputation/65 + 40, from 40 to 145). Only `al`. Only by the scout (PLAYER_SEARCH::run_scout_day `exe::player_search_run_scout_day`, report on
/// a club).
/// Original: `club_has_enough_defenders` (0x549100, Database.cpp, cdecl).
///
/// [pt] { 0x549100 } (cdecl, Database.cpp): `club_has_enough_defenders(clube)`: clube nulo: 0. Se o clube toma mais de
/// 1,35 gol por jogo ([0x969588]; gols sofridos +0x6a da estatística / jogos): 0. Senão, 1 se mais de 2 zagueiros
/// (posição +0x11 = 20) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day
/// 0x7b3e20, relatório de um clube).
pub(crate) unsafe extern "C" fn club_has_enough_defenders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(Club(club), 0x6a) {
        if v as f64 > read_f64(0x969588) {
            return 0; // a defesa toma gols demais
        }
    }
    let limit = squad_ability_threshold(Club(club), 0x91);
    (count_squad_above_ability(Club(club), limit, |j| read_u8(j + player_sheet::DEFENDER) == 20) > 2) as u8 // zagueiros (dbo)
}

/// 1 when more than 2 midfielders (+0x14, +0x12 or +0x13 = 20) without serious injury are above the limit
/// (reputation/65 + 40, 40 to 145)
/// Original: `club_has_enough_midfielders` (0x549200, Database.cpp, cdecl).
///
/// [pt] { 0x549200 } (cdecl, Database.cpp): `club_has_enough_midfielders(clube)`: clube nulo: 0; senão 1 se mais de 2
/// meias (posição +0x14, +0x12 ou +0x13 = 20, nessa ordem) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_enough_midfielders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    let limit = squad_ability_threshold(Club(club), 0x91);
    let half = |j: usize| read_u8(j + player_sheet::ATTACKING_MIDFIELDER) == 20 || read_u8(j + player_sheet::DEFENSIVE_MIDFIELDER) == 20 || read_u8(j + player_sheet::MIDFIELDER) == 20;
    (count_squad_above_ability(Club(club), limit, half) > 2) as u8 // meias (dbo)
}

/// club scoring fewer than 1.5 goals a game: 0; otherwise 1 when more than 1 striker (+0x15 = 20) without serious
/// injury is above the limit (reputation/65 + 40, 40 to 150)
/// Original: `club_has_enough_strikers` (0x5492c0, Database.cpp, cdecl).
///
/// [pt] { 0x5492c0 } (cdecl, Database.cpp): `club_has_enough_strikers(clube)`: clube nulo: 0. Se o clube faz menos de
/// 1,5 gol por jogo ([0x968940]; gols marcados +0x69 da estatística / jogos): 0. Senão, 1 se mais de 1 atacante
/// (posição +0x15 = 20) passa do limite /65 (teto 150). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_enough_strikers(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(Club(club), 0x69) {
        if (v as f64) < read_f64(0x968940) {
            return 0; // o ataque faz poucos gols
        }
    }
    let limit = squad_ability_threshold(Club(club), 0x96);
    (count_squad_above_ability(Club(club), limit, |j| read_u8(j + player_sheet::ATTACKER) == 20) > 1) as u8 // atacantes (dbo)
}

/// How many of the 50 in the squad (+0xd7) count: a non-null person with a player (+0x61) of `posicao`, not badly
/// injured (INJURY `get_player_injury_status` on [0xb63d28] ≠ 2), id (+0) < [0xae2364] (signed), current ability (the player's +5, signed
/// short) above `limite`, and the `extra` predicate (the player's attributes).
///
/// Like count_squad_above_ability but also requiring `extra`. In the original's order (`get_player_injury_status` only for those of the
/// position). 8-bit counter (at most 50).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) contam: pessoa não nula com jogador (+0x61) da `posicao`, sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5 do jogador, curto com
/// sinal) acima do `limite` e o `extra` (os atributos do jogador). Na ordem da original (a 0x62c400 só para os da
/// posição). Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_above_ability_with(club: Club, limit: i32, position: impl Fn(usize) -> bool, extra: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| read_usize(a);
    let mut n = 0u8;
    for k in 0..50 {
        let p = Staff(rd(club.addr() + club::SQUAD + 4 * k));
        if p.is_null() {
            continue;
        }
        let j = p.player();
        if j.is_null() || !position(j.addr()) || exe::injury_get_player_injury_status(rd(G_INJURY), p) == 2 {
            continue;
        }
        if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if read_i16(j.addr() + 5) as i32 > limit && extra(j.addr()) {
            n += 1;
        }
    }
    n
}

/// Club reputation (+0x80, signed 16-bit).
///
/// [pt] A reputação do clube (+0x80, curto com sinal).
pub(crate) unsafe fn club_reputation(club: Club) -> i32 {
    club.reputation() as i32
}

/// `club_has_enough_hard_workers(club)`: null club: 0; otherwise 1 when more than 4 players (any position) without
/// serious injury are above the ability limit reputation/65 + 40 (from 40 to 140) with attribute +0x44 (signed byte;
/// the same one `club_has_ball_winning_midfielders` uses) above 14.
///
/// Only `al`. Only by the scout (PLAYER_SEARCH::run_scout_day `exe::player_search_run_scout_day`, report on a club).
/// Original: `club_has_enough_hard_workers` (0x5493c0, Database.cpp, cdecl).
///
/// [pt] { 0x5493c0 } (cdecl, Database.cpp): `club_has_enough_hard_workers(clube)`: clube nulo: 0; senão 1 se mais de 4
/// jogadores (qualquer posição) passam do limite reputação/65 + 40 (de 40 a 140) com o atributo +0x44 (byte com
/// sinal; o mesmo que a 0x549850 usa) acima de 14. Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20,
/// relatório de um clube).
pub(crate) unsafe extern "C" fn club_has_enough_hard_workers(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    let limit = (club_reputation(Club(club)) / 65 + 40).clamp(40, 0x8c);
    let n = count_squad_above_ability_with(Club(club), limit, |_| true, |j| read_i8(j + player_sheet::WORK_RATE) > 14);
    (n > 4) as u8 // esforcados (dbo)
}

/// `club_has_fit_goalkeeper(club)`: 1 as soon as one of the 50 in the squad (+0xd7) is a person with a player record
/// (+0x61) who is a goalkeeper (+0xf = 20), without serious injury (`get_player_injury_status` ≠ 2) and with id < [0xae2364]; none or
/// null club: 0.
///
/// No ability limit. Only `al`. Only by the scout.
/// Original: `club_has_fit_goalkeeper` (0x549470, Database.cpp, cdecl).
///
/// [pt] { 0x549470 } (cdecl, Database.cpp): `club_has_fit_goalkeeper(clube)`: 1 assim que um dos 50 do elenco (+0xd7) é
/// pessoa com jogador (+0x61) goleiro (+0xf = 20), sem lesão grave (0x62c400 ≠ 2) e id < [0xae2364]; nenhum ou clube
/// nulo: 0. Sem limite de habilidade. Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_fit_goalkeeper(club: usize) -> u8 {
    let club = Club(club);
    if club.is_null() {
        return 0;
    }
    let rd = |a: usize| read_usize(a);
    for k in 0..50 {
        let p = Staff(rd(club.addr() + club::SQUAD + 4 * k));
        if p.is_null() {
            continue;
        }
        let j = p.player();
        if j.is_null() || (j.goalkeeper() as u8) != 20 || exe::injury_get_player_injury_status(rd(G_INJURY), p) == 2 {
            continue;
        }
        if read_i32(p.addr()) < read_i32(G_STAFF_COUNT) {
            return 1; // goalkeeper fit to play (dbo)
        }
    }
    0
}

/// 1 when more than 1 midfielder without serious injury is above the limit (reputation/75 + 25, 40 to 135) with
/// attributes +0x44 and +0x3e above reputation/500 + 5 (at most 14)
/// Original: `club_has_ball_winning_midfielders` (0x549850, Database.cpp, cdecl).
///
/// [pt] { 0x549850 } (cdecl, Database.cpp): `club_has_ball_winning_midfielders(clube)`: clube nulo: 0; senão 1 se mais
/// de 1 meia (posição +0x14, +0x12 ou +0x13 = 20) sem lesão grave passa do limite reputação/75 + 25 (de 40 a 135)
/// com os atributos +0x44 e +0x3e (bytes com sinal) acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo
/// olheiro.
pub(crate) unsafe extern "C" fn club_has_ball_winning_midfielders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    let rep = club_reputation(Club(club));
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(14);
    let half = |j: usize| read_u8(j + player_sheet::ATTACKING_MIDFIELDER) == 20 || read_u8(j + player_sheet::DEFENSIVE_MIDFIELDER) == 20 || read_u8(j + player_sheet::MIDFIELDER) == 20;
    let n = count_squad_above_ability_with(Club(club), limit, half, |j| read_i8(j + player_sheet::WORK_RATE) as i32 > a && *((j + player_sheet::TACKLING) as *const i8) as i32 > a);
    (n > 1) as u8 // midfielders who steal the ball (dbo)
}

/// Counts how many of the 50 squad members (+0xd7) are natural at position `posicao` and good enough for the club's
/// reputation.
///
/// A member counts when: person with a player record (+0x61) at `posicao` (byte = 20), no serious injury (INJURY
/// `get_player_injury_status` on [0xb63d28] ≠ 2), id (+0) < [0xae2364] (signed), current ability (+5, signed short) above reputation/75 +
/// 25 (from 40 to 135) and `atributo` (signed byte) above reputation/500 + 5 (at most `teto`). Reputation: the club's
/// +0x80, signed short.
///
/// In the original routine's order. 8-bit counter (at most 50).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) na `posicao` (byte = 20), sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal)
/// acima de reputação/75 + 25 (de 40 a 135) e o `atributo` (byte com sinal) acima de reputação/500 + 5 (no máximo
/// `teto`). Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_natural_at_position(club: Club, position: usize, attribute: usize, ceiling: i32) -> u8 {
    let rd = |a: usize| read_usize(a);
    let rep = club.reputation() as i32;
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(ceiling);
    let mut n = 0u8;
    for k in 0..50 {
        let p = Staff(rd(club.addr() + club::SQUAD + 4 * k));
        if p.is_null() {
            continue;
        }
        let j = p.player();
        if j.is_null() || read_u8(j.addr() + position) != 20 || exe::injury_get_player_injury_status(rd(G_INJURY), p) == 2 {
            continue;
        }
        if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if read_i16(j.addr() + 5) as i32 > limit && *((j.addr() + attribute) as *const i8) as i32 > a {
            n += 1;
        }
    }
    n
}

/// 1 when at most 1 striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to
/// 135) with attribute +0x2e above reputation/500 + 5 (at most 13)
/// Original: `club_lacks_aerial_strikers` (0x5494d0, Database.cpp, cdecl).
///
/// [pt] { 0x5494d0 } (cdecl, Database.cpp): `club_lacks_aerial_strikers(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// atacante (posição +0x15) passa dos limites com o atributo +0x2e (o jogo pelo alto) acima de reputação/500 + 5 (no
/// máximo 13). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de um clube).
pub(crate) unsafe extern "C" fn club_lacks_aerial_strikers(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    (count_natural_at_position(Club(club), 0x15, 0x2e, 13) <= 1) as u8 // forwards in the air (dbo)
}

/// the same with defenders (+0x11 = 20) and attribute +0x2e (at most 13)
/// Original: `club_lacks_aerial_defenders` (0x5495b0, Database.cpp, cdecl).
///
/// [pt] { 0x5495b0 } (cdecl, Database.cpp): `club_lacks_aerial_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x2e acima de reputação/500 + 5 (no máximo 13). Só
/// `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_lacks_aerial_defenders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    (count_natural_at_position(Club(club), 0x11, 0x2e, 13) <= 1) as u8 // centre-backs in the air (dbo)
}

/// `club_lacks_fast_defenders(club)`: null club: 0; otherwise 1 when at most 1 defender (position +0x11 = 20) passes
/// the limits (as in the previous checks) with attribute +0x36 (pace) above reputation/500 + 5 (at most 10).
///
/// Only `al`. Only by the scout.
/// Original: `club_lacks_fast_defenders` (0x549690, Database.cpp, cdecl).
///
/// [pt] { 0x549690 } (cdecl, Database.cpp): `club_lacks_fast_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x36 (a velocidade) acima de reputação/500 + 5 (no
/// máximo 10). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_lacks_fast_defenders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    (count_natural_at_position(Club(club), 0x11, 0x36, 10) <= 1) as u8 // zagueiros rapidos (dbo)
}

/// How many of the 50 in the squad (+0xd7) are a person with a player (+0x61) of `posicao`, not badly injured (INJURY
/// `get_player_injury_status` on [0xb63d28] ≠ 2), id (+0) < [0xae2364] (signed), current ability (+5, signed short) above reputation/75 +
/// 25 (from 40 to 135), and pass `extra` with the attribute threshold reputation/500 + 5 (at most `teto`).
///
/// Reputation: the club's +0x80, signed short. In the original's order. 8-bit counter (at most 50).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400
/// em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal) acima de
/// reputação/75 + 25 (de 40 a 135) e o `extra` com o limite de atributo reputação/500 + 5 (no máximo `teto`).
/// Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_by_rep_thresholds(club: Club, ceiling: i32, position: impl Fn(usize) -> bool, extra: impl Fn(usize, i32) -> bool) -> u8 {
    let rd = |a: usize| read_usize(a);
    let rep = club.reputation() as i32;
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(ceiling);
    let mut n = 0u8;
    for k in 0..50 {
        let p = Staff(rd(club.addr() + club::SQUAD + 4 * k));
        if p.is_null() {
            continue;
        }
        let j = p.player();
        if j.is_null() || !position(j.addr()) || exe::injury_get_player_injury_status(rd(G_INJURY), p) == 2 {
            continue;
        }
        if read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        if read_i16(j.addr() + 5) as i32 > limit && extra(j.addr(), a) {
            n += 1;
        }
    }
    n
}

/// 1 when some striker (+0x15 = 20) without serious injury is above the ability limit (reputation/75 + 25, 40 to 135)
/// with attribute +0x36 above reputation/500 + 5 (at most 17)
/// Original: `club_has_fast_striker` (0x549770, Database.cpp, cdecl).
///
/// [pt] { 0x549770 } (cdecl, Database.cpp): `club_has_fast_striker(clube)`: clube nulo: 0; senão 1 se algum atacante
/// (posição +0x15 = 20) passa dos limites com o atributo +0x36 (a velocidade, byte com sinal) acima de
/// reputação/500 + 5 (no máximo 17). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de
/// um clube).
pub(crate) unsafe extern "C" fn club_has_fast_striker(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    let n = count_squad_by_rep_thresholds(Club(club), 17, |j| read_u8(j + player_sheet::ATTACKER) == 20, |j, a| read_i8(j + player_sheet::PLAYER_PACE) as i32 > a);
    (n > 0) as u8 // atacante rapido (dbo)
}

/// `club_has_two_creative_midfielders(club)`: null club: 0; otherwise 1 when more than 1 midfielder (position +0x14,
/// +0x12 or +0x13 = 20) without serious injury passes the limits with attributes +0x43 and +0x37 (signed bytes) above
/// reputation/500 + 5 (at most 14).
///
/// Only `al`. Only by the scout.
/// Original: `club_has_two_creative_midfielders` (0x549940, Database.cpp, cdecl).
///
/// [pt] { 0x549940 } (cdecl, Database.cpp): `club_has_two_creative_midfielders(clube)`: clube nulo: 0; senão 1 se mais de
/// 1 meia (posição +0x14, +0x12 ou +0x13 = 20) passa dos limites com os atributos +0x43 e +0x37 (bytes com sinal)
/// acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_two_creative_midfielders(club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    let half = |j: usize| read_u8(j + player_sheet::ATTACKING_MIDFIELDER) == 20 || read_u8(j + player_sheet::DEFENSIVE_MIDFIELDER) == 20 || read_u8(j + player_sheet::MIDFIELDER) == 20;
    let n = count_squad_by_rep_thresholds(Club(club), 14, half, |j, a| read_i8(j + player_sheet::VISION) as i32 > a && *((j + player_sheet::PASSING) as *const i8) as i32 > a);
    (n > 1) as u8 // meias criativos (dbo)
}

/// `club_squad_strength_band(club, other)`: the squad strength band, 0..6. Null club: 3.
///
/// Gathers those in the squad (+0xd7) with a player record (+0x61) and id < [0xae2364] (read once) into a local array
/// of 50, sorts it with the CRT qsort (`exe::crt_qsort`) using the comparison `compare_staff_by_player_ability_desc` (through the game's address) and sums
/// the current ability (the player's +5) of the first min(n − 1, 16). With nothing summed: by reputation (+0x80; >
/// 7250: 6, > 5750: 5, > 4750: 4, > 3250: 3, > 2250: 2, otherwise 1); otherwise by the average (> 155: 6, > 135: 5, >
/// 110: 4, > 85: 3, > 60: 2, > 40: 1, otherwise 0).
///
/// With `other` (the scout's club): its reputation > 7250 takes off 2 if the club's is < 4250 and the band > 2, and 1
/// more if the club's is < 7250 and the band > 0; otherwise, its reputation < 4250 adds 1 if the band < 5. In the same
/// competition (+0x57 equal), a reputation gap (16 bits) > 1250 takes off 1 (band > 0, byte +4 of the table [0xae2d14]
/// of 6 bytes per id: the other's ≥ 0 and the club's ≤ 0); < 1250 adds 1 (band < 5, the club's ≥ 0 and the other's ≤
/// 0). At the end, between 0 and 6. Only `al`. Only by the scout.
/// Original: `club_squad_strength_band` (0x549a30, Database.cpp, cdecl).
///
/// [pt] { 0x549a30 } (cdecl, Database.cpp): `club_squad_strength_band(clube, outro)`: a faixa de força do elenco, 0..6.
/// Clube nulo: 3. Junta os do elenco (+0xd7) com jogador (+0x61) e id < [0xae2364] (lido uma vez) num vetor local de
/// 50, ordena pelo qsort da CRT (0x945333) com a comparação 0x549c30 (pelo endereço do jogo) e soma a habilidade atual
/// (+5 do jogador) dos primeiros min(n − 1, 16). Sem nenhum somado: pela reputação (+0x80; > 7250: 6, > 5750: 5,
/// > 4750: 4, > 3250: 3, > 2250: 2, senão 1); senão pela média (> 155: 6, > 135: 5, > 110: 4, > 85: 3, > 60: 2,
/// > 40: 1, senão 0). Com `outro` (o clube do olheiro): reputação dele > 7250 tira 2 se a do clube < 4250 e a faixa
/// > 2, e mais 1 se a do clube < 7250 e a faixa > 0; senão, reputação dele < 4250 soma 1 se a faixa < 5. Na mesma
/// competição (+0x57 igual), a diferença de reputação (16 bits) > 1250 tira 1 (faixa > 0, byte +4 da tabela
/// [0xae2d14] de 6 bytes por id: do outro ≥ 0 e do clube ≤ 0); < 1250 soma 1 (faixa < 5, do clube ≥ 0 e do outro
/// ≤ 0). No fim, entre 0 e 6. Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_squad_strength_band(club: usize, other: usize) -> u8 {
    let other = Club(other);
    let club = Club(club);
    if club.is_null() {
        return 3;
    }
    let rd = |a: usize| read_usize(a);
    let rd16 = |a: usize| read_i16(a);
    let id = |a: usize| read_i32(a);
    let total = read_i32(G_STAFF_COUNT);
    let mut v = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let p = rd(club.addr() + club::SQUAD + 4 * k);
        if p != 0 && read_usize(p + staff::PLAYER) != 0 && id(p) < total {
            v[n] = p;
            n += 1;
        }
    }
    exe::crt_qsort(v.as_mut_ptr() as usize, n as u32, 4, 0x549c30); // the best first (dbo)
    let m = (n as i32 - 1).clamp(0, 16) as usize;
    let mut sum = 0i32;
    for &p in &v[..m] {
        sum = sum.wrapping_add(rd16(read_usize(p + staff::PLAYER) + 5) as i32);
    }
    let rep = club.reputation();
    let mut f: i8 = if m == 0 {
        match rep {
            r if r > 0x1c52 => 6,
            r if r > 0x1676 => 5,
            r if r > 0x128e => 4,
            r if r > 0xcb2 => 3,
            r => (r > 0x8ca) as i8 + 1,
        }
    } else {
        match sum / m as i32 {
            x if x > 0x9b => 6,
            x if x > 0x87 => 5,
            x if x > 0x6e => 4,
            x if x > 0x55 => 3,
            x if x > 0x3c => 2,
            x => (x > 0x28) as i8,
        }
    };
    if !other.is_null() {
        let ro = other.reputation();
        if ro > 0x1c52 {
            if rep < 0x109a && f > 2 {
                f -= 2;
            }
            if rep < 0x1c52 && f > 0 {
                f -= 1;
            }
        } else if ro < 0x109a && f < 5 {
            f += 1; // the scout of a small club sees the stronger club (dbo)
        }
        if other.division() == club.division() {
            let d = ro.wrapping_sub(rep);
            let table = rd(G_CLUB_STATE_TABLE);
            let mark = |c: usize| read_i8(table.wrapping_add((id(c).wrapping_mul(6) as usize).wrapping_add(4)));
            if d > 0x4e2 && f > 0 && mark(other.addr()) >= 0 && mark(club.addr()) <= 0 {
                f -= 1;
            } else if d < 0x4e2 && f < 5 && mark(club.addr()) >= 0 && mark(other.addr()) <= 0 {
                f += 1;
            }
        }
    }
    f.clamp(0, 6) as u8
}

/// Club of the person's contract as `club_reputation_for_staff` finds it (contract via `exe::contract_manager_get_contract`, stored in G_CONTRACT_LOOKUP_RESULT; club
/// id at contract +4); 0 without contract.
///
/// [pt] O clube do contrato da `pessoa` como a 0x545e90 o acha: o contrato pela 0x4cf5a0 (this = [0xae1050]), gravado
/// em [0xae1080] como na original; com contrato, o clube de id +4 na tabela [0xae23b4] (581 bytes); sem: 0.
pub(crate) unsafe fn database_club_of_contract(person: Staff) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if c.is_null() {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(581))
}

/// +500 if the club is one of the person's three favourite clubs, -500 if one of the three disliked, else 0.
///
/// [pt] +500 se o `clube` está entre os três clubes favoritos da pessoa (preferências +4/+8/+0xc), −500 se entre os três
/// rejeitados (+0x10/+0x14/+0x18), senão 0; sem preferências: 0.
pub(crate) unsafe fn club_preference_bonus(club: Club, person: Staff) -> i32 {
    let pref = person.staff_preferences();
    if pref == 0 {
        return 0;
    }
    if (0..3).any(|k| read_usize(pref + preferences::STAFF_FAVOURITE_CLUBS1 + 4 * k) == club.addr()) {
        500
    } else if (0..3).any(|k| read_usize(pref + preferences::STAFF_DISLIKED_CLUBS1 + 4 * k) == club.addr()) {
        -500
    } else {
        0
    }
}

/// `club_reputation_for_staff(club, person)`: the club's reputation (+0x80) as seen by the person.
///
/// No division (+0x57): the reputation. With the division's league ([0xadadfc][id]) and the division's reputation
/// (+0x69) × 500 above the club's: the position (`exe::comp_stats_find_club_entry` on the league's table +8; no row: 0) scaled to 20 clubs
/// (+0x3e) gives the weight from table 0x9b59c0; with fewer than 20 games (+4 of the row) it is mixed with the weight
/// of the previous position (+0x5f, if +0x5b = +0x57; otherwise 1.3) × 0.05; if the person plays (+0x61) and the
/// contract is with the club, adds the average `get_stat`(data, 6, 20, weight) and divides by 2, and gets ±500 from the
/// preferences (favoured or disliked club); the weight becomes x = 14p − 18 between 0 and 10 and the result is ((10 −
/// x) × reputation + x × division × 500 (16 bits)) / 10 truncated, plus the ±500. Otherwise: the reputation ±500 (a
/// person who plays, with a contract at the club).
///
/// Callers read only `ax`; the rest of eax follows the original (the register left over on each path). Writes
/// [0xae1080] (the contract found). Used in the daily person update (game_daily_staff_update 0x5ce2d0, two calls).
/// Original: `club_reputation_for_staff` (0x545e90, Database.cpp, cdecl).
///
/// [pt] { 0x545e90 } (cdecl, Database.cpp): `club_reputation_for_staff(clube, pessoa)`: a reputação do clube (+0x80) vista
/// pela pessoa. Sem divisão (+0x57): a reputação. Com a liga da divisão ([0xadadfc][id]) e a reputação da divisão
/// (+0x69) × 500 acima da do clube: a posição (0x4a4850 na tabela +8 da liga; sem linha: 0) escalada a 20 clubes
/// (+0x3e) dá o peso da tabela 0x9b59c0; com menos de 20 jogos (+4 da linha) mistura com o peso da posição anterior
/// (+0x5f, se +0x5b = +0x57; senão 1,3) × 0,05; se a pessoa joga (+0x61) e o contrato é com o clube, soma a média
/// 0x7c1e30(dados, 6, 20, peso) e divide por 2, e ganha ±500 pelas preferências; o peso vira x = 14p − 18 entre 0 e
/// 10 e o resultado é ((10 − x) × reputação + x × divisão × 500 (16 bits)) / 10 truncado, mais o ±500. Senão: a
/// reputação ±500 (pessoa que joga, com contrato no clube). Os chamadores leem só `ax`; o resto do eax segue a
/// original (o registro que sobrou em cada caminho). Grava [0xae1080] (o contrato achado). No diário das pessoas
/// (game_daily_staff_update 0x5ce2d0, duas chamadas).
pub(crate) unsafe extern "C" fn club_reputation_for_staff(club: usize, person: usize) -> u32 {
    let person = Staff(person);
    let club = Club(club);
    let rep = club.reputation();
    let comp = club.division();
    if comp.is_null() {
        return rep as u16 as u32;
    }
    let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp.addr()).wrapping_mul(4))));
    let division = comp.reputation();
    let high = (division as i32).wrapping_mul(500);
    if !league.is_null() && high > rep as i32 {
        let e = exe::comp_stats_find_club_entry(league.comp_stats(), club);
        if e == 0 {
            return 0;
        }
        let n = league.club_count() as i32;
        let weight = |k: i32| read_f64(0x9b59c0 + 8 * k.clamp(0, 19) as usize);
        let mut position = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32 - 1;
        if n != 0 && n != 20 {
            position = position * 20 / n;
        }
        let mut v = weight(position);
        let games = read_u8(e + 4) as i32;
        if games < 20 {
            let before = (club.last_position() as i8) as i32;
            let w = if club.last_division() != comp {
                read_f64(rdata::F64_1_3)
            } else if n != 0 && n != 20 {
                weight((before * 20 - 20) / n)
            } else if (1..21).contains(&before) {
                weight(before - 1)
            } else {
                read_f64(rdata::F64_1_3)
            };
            v = ((20 - games) as f64 * w + games as f64 * v) * read_f64(rdata::F64_0_05);
        }
        let mut bonus = 0;
        if !person.is_null() && !person.player().is_null() && database_club_of_contract(person) == club.addr() {
            let m = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), person.player(), 6, 0x14, v as f32);
            v = (m + v) * read_f64(rdata::F64_0_5);
            bonus = club_preference_bonus(club, person);
        }
        let mut x = v * read_f64(rdata::F64_14) - read_f64(rdata::F64_18); // o peso entre 0 e 10;
        let (baixo, teto) = (read_f64(rdata::F64_0), read_f64(rdata::F64_10));
        if baixo > x {
            x = baixo;
        } else if !(teto >= x) {
            x = teto; // fcomp: above the ceiling or NaN
        }
        let cx = (division as i32).wrapping_mul(500) as i16 as f64; // imul cx, cx, 0x1f4: 16 bits
        let r = ((teto - x) * rep as f64 + cx * x) * read_f64(rdata::F64_0_1);
        return (ftol(r) as u32).wrapping_add(bonus as u32);
    }
    // no league, or the club already at the division’s level: reputation ±500; the high part of eax is leftover
    let mut eax = if league.is_null() { comp.addr() } else { high as usize };
    let mut bonus = 0;
    if !person.is_null() {
        eax = person.player().addr();
        if eax != 0 {
            eax = database_club_of_contract(person);
            if eax == club.addr() {
                eax = person.staff_preferences();
                if eax != 0 {
                    bonus = club_preference_bonus(club, person); // the person’s clubs
                }
            }
        }
    }
    ((eax as u32 & 0xffff_0000) | rep as u16 as u32).wrapping_add(bonus as u32)
}

/// How many of the three people in the preference list at `base` belong (+0x39) to `clube`; 0 on null inputs.
///
/// [pt] Quantas das três pessoas da lista de preferências em `base` (+0x65 da pessoa) têm o clube (+0x39) igual a `clube`;
/// pessoa, preferências ou clube nulos: 0. Só `al`.
pub(crate) unsafe fn count_preferred_people_at_club(person: Staff, club: usize, base: usize) -> u8 {
    if person.is_null() {
        return 0;
    }
    let pref = person.staff_preferences();
    if pref == 0 || club == 0 {
        return 0;
    }
    let mut n = 0u8;
    for k in 0..3 {
        let p = Staff(read_usize(pref + base + 4 * k));
        if !p.is_null() && p.club_job() == Club(club) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// how many of the three disliked staff (preferences +0x28..+0x30) are at the club (+0x39)
/// Original: `staff_count_disliked_staff_at_club` (0x547c90, Database.cpp, cdecl).
///
/// [pt] { 0x547c90 } (cdecl, Database.cpp): `staff_count_disliked_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// que a pessoa rejeita (preferências +0x28/+0x2c/+0x30) estão no clube. No diário das pessoas (0x5ce2d0).
pub(crate) unsafe extern "C" fn staff_count_disliked_staff_at_club(person: usize, club: usize) -> u8 {
    count_preferred_people_at_club(Staff(person), club, 0x28) // the rejected ones
}

/// how many of the three favoured staff (preferences +0x1c..+0x24) are at the club (+0x39)
/// Original: `staff_count_favoured_staff_at_club` (0x547ce0, Database.cpp, cdecl).
///
/// [pt] { 0x547ce0 } (cdecl, Database.cpp): `staff_count_favoured_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24) estão no clube. No diário das pessoas (0x5ce2d0).
pub(crate) unsafe extern "C" fn staff_count_favoured_staff_at_club(person: usize, club: usize) -> u8 {
    count_preferred_people_at_club(Staff(person), club, 0x1c) // the favorites
}

/// Zeroes the first of three dword slots that holds `alvo`.
///
/// [pt] Zera a primeira das três posições (`reg`, +4, +8) que tem `alvo` (a ordem da original: achou, zera e sai).
pub(crate) unsafe fn remove_from_list3(reg: usize, target: usize) {
    for k in 0..3 {
        let a = (reg + 4 * k) as *mut usize;
        if core::ptr::read_unaligned(a) == target {
            core::ptr::write_unaligned(a, 0);
            return;
        }
    }
}

/// The person's STAFF_PREFERENCES pointer (+0x65), or 0 for a null person.
///
/// [pt] As preferências (+0x65) da pessoa não nula; pessoa nula: 0 (o eax que as 0x547c10/0x547c50/0x547f50 devolvem).
pub(crate) unsafe fn staff_preferences(person: Staff) -> usize {
    if person.is_null() {
        0
    } else {
        person.staff_preferences()
    }
}

/// `staff_remove_favoured_club(person, club)`: removes the club from the person's three favoured clubs (preferences
/// +4/+8/+0xc, the first one that matches).
///
/// Null person, preferences or club: nothing. Returns the original's eax (the preferences; null person: 0). Used in the
/// daily person update (0x5ce2d0), when the attachment to the club ends.
/// Original: `staff_remove_favoured_club` (0x547f50, Database.cpp, cdecl).
///
/// [pt] { 0x547f50 } (cdecl, Database.cpp): `staff_remove_favoured_club(pessoa, clube)`: tira o clube dos três clubes
/// favoritos da pessoa (preferências +4/+8/+0xc, o primeiro que bate). Pessoa, preferências ou clube nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). No diário das pessoas (0x5ce2d0), quando o
/// apego ao clube acaba.
pub(crate) unsafe extern "C" fn staff_remove_favoured_club(person: usize, club: usize) -> usize {
    let pref = staff_preferences(Staff(person));
    if pref != 0 && club != 0 {
        remove_from_list3(pref + 4, club); // the favorite clubs
    }
    pref
}

/// `club_remove_favoured_staff(club, person)`: removes the person from the club's three favoured people
/// (+0x9b/+0x9f/+0xa3, the first one that matches).
///
/// Null club or person: nothing. Returns the club (the original's eax). Used in the dispute hearings
/// (DISPUTE::process_hearings 0x569ed0).
/// Original: `club_remove_favoured_staff` (0x547bd0, Database.cpp, cdecl).
///
/// [pt] { 0x547bd0 } (cdecl, Database.cpp): `club_remove_favoured_staff(clube, pessoa)`: tira a pessoa das três pessoas
/// favoritas do clube (+0x9b/+0x9f/+0xa3, o primeiro que bate). Clube ou pessoa nulos: nada. Devolve o clube (o eax
/// da original). Nas audiências das disputas (DISPUTE::process_hearings 0x569ed0).
pub(crate) unsafe extern "C" fn club_remove_favoured_staff(club: usize, person: usize) -> usize {
    let club = Club(club);
    if !club.is_null() && person != 0 {
        remove_from_list3(club.addr() + club::FAV_STAFF1, person); // the club’s favorites
    }
    club.addr()
}

/// `staff_remove_favoured_staff(person, target)`: removes the target from the person's three favoured people
/// (preferences +0x1c/+0x20/+0x24, the first one that matches).
///
/// Null person, preferences or target: nothing. Returns the original's eax (the preferences; null person: 0). Used in
/// disputes (0x569ed0), squad relationships (game_update_squad_relationships `game_update_squad_relationships`, daily), the return from injury
/// (`post_recovery_news`), MANAGER_MANAGER (0x697a7a) and the media (0x75a340).
/// Original: `staff_remove_favoured_staff` (0x547c10, Database.cpp, cdecl).
///
/// [pt] { 0x547c10 } (cdecl, Database.cpp): `staff_remove_favoured_staff(pessoa, alvo)`: tira o alvo das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24, o primeiro que bate). Pessoa, preferências ou alvo nulos:
/// nada. Devolve o eax da original (as preferências; pessoa nula: 0). Nas disputas (0x569ed0), nas relações do
/// elenco (game_update_squad_relationships 0x5d6d10, diária), na volta da lesão (0x630ee0), no MANAGER_MANAGER
/// (0x697a7a) e na mídia (0x75a340).
pub(crate) unsafe extern "C" fn staff_remove_favoured_staff(person: usize, target: usize) -> usize {
    let pref = staff_preferences(Staff(person));
    if pref != 0 && target != 0 {
        remove_from_list3(pref + preferences::STAFF_FAVOURITE_STAFF1, target); // the favorite people
    }
    pref
}

/// `staff_remove_disliked_staff(person, target)`: clears the first of the person's three disliked people (preferences
/// +0x28/+0x2c/+0x30) that is the target.
///
/// Null person, preferences or target: nothing. Returns the original's eax (the preferences; null person: 0). Used in
/// squad relationships (game_update_squad_relationships `game_update_squad_relationships`, daily), the return from injury (`post_recovery_news`) and the
/// media (0x7414ac, 0x75a340).
/// Original: `staff_remove_disliked_staff` (0x547c50, Database.cpp, cdecl).
///
/// [pt] { 0x547c50 } (cdecl, Database.cpp): `staff_remove_disliked_staff(pessoa, alvo)`: zera a primeira das três pessoas
/// rejeitadas da pessoa (preferências +0x28/+0x2c/+0x30) que é o alvo. Pessoa, preferências ou alvo nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). Nas relações do elenco (game_update_squad_relationships
/// 0x5d6d10, diária), na volta da lesão (0x630ee0) e na mídia (0x7414ac, 0x75a340).
pub(crate) unsafe extern "C" fn staff_remove_disliked_staff(person: usize, target: usize) -> usize {
    let person = Staff(person);
    if person.is_null() {
        return 0;
    }
    let pref = person.staff_preferences();
    if pref != 0 && target != 0 {
        let reg = pref + preferences::STAFF_DISLIKED_STAFF1; // the rejected people
        for k in 0..3 {
            let a = (reg + 4 * k) as *mut usize;
            if core::ptr::read_unaligned(a) == target {
                core::ptr::write_unaligned(a, 0);
                break;
            }
        }
    }
    pref
}

/// `staff_get_favoured_staff(person, array, n)`: zeroes the byte `n` and, with person, preferences and array not null,
/// copies the non-null favoured people (preferences +0x1c, +0x20, +0x24, in that order) to `array[n]`, adding 1 to `n`
/// for each (the first goes straight into `array[0]`).
///
/// Returns `n` (the original's eax). Used in MANAGER_MANAGER: the board's opinion of the squad (0x693850) and its
/// reaction to the match (`board_named_players_after_match`).
/// Original: `staff_get_favoured_staff` (0x548090, Database.cpp, cdecl).
///
/// [pt] { 0x548090 } (cdecl, Database.cpp): `staff_get_favoured_staff(pessoa, vetor, n)`: zera o byte `n` e, com pessoa,
/// preferências e vetor não nulos, copia as pessoas favoritas não nulas (preferências +0x1c, +0x20, +0x24, nessa
/// ordem) para `vetor[n]`, somando 1 a `n` a cada uma (a primeira vai direto em `vetor[0]`). Devolve `n` (o eax da
/// original). No MANAGER_MANAGER: a opinião da diretoria sobre o elenco (0x693850) e a reação dela à partida
/// (0x6aaee0).
pub(crate) unsafe extern "C" fn staff_get_favoured_staff(person: usize, array: usize, n: *mut i8) -> *mut i8 {
    *n = 0;
    if person == 0 {
        return n;
    }
    let pref = |p: Staff| p.staff_preferences();
    if pref(Staff(person)) == 0 || array == 0 {
        return n;
    }
    for (k, d) in [0x1cusize, 0x20, 0x24].into_iter().enumerate() { // the three favorites
        let p = core::ptr::read_unaligned((pref(Staff(person)) + d) as *const usize);
        if p != 0 {
            let i = if k == 0 { 0 } else { *n as isize };
            core::ptr::write_unaligned((array as isize + 4 * i) as *mut usize, p);
            *n = (*n).wrapping_add(1);
        }
    }
    n
}

/// `club_is_in_season(club)`: null club, 0; otherwise what `key_nation_club_in_season` `key_nation_club_in_season`(club, null, null)
/// returns (1 = today is inside the season of the club's nation), the whole eax (the default contract `exe::staff_contract_default_staff_contract` tests
/// eax).
///
/// At creation: the people's default contract (CONTRACT_MANAGER::initialise_all_staff_contracts → 0x85e960 → `exe::staff_contract_default_staff_contract`)
/// and the default wage of the initial lists (0x83a1b0 → `exe::staff_contract_calc_unset_default_wage`).
/// Original: `club_is_in_season` (0x547770, Database.cpp, cdecl).
///
/// [pt] { 0x547770 } (cdecl, Database.cpp): `club_is_in_season(clube)`: clube nulo, 0; senão o que
/// `key_nation_club_in_season` 0x669bd0(clube, nulo, nulo) devolve (1 = hoje dentro da temporada da nação do clube),
/// o eax inteiro (o contrato padrão 0x864550 testa o eax). Na criação: o contrato padrão das pessoas
/// (CONTRACT_MANAGER::initialise_all_staff_contracts → 0x85e960 → 0x864550) e o salário padrão das listas iniciais
/// (0x83a1b0 → 0x862720).
pub(crate) unsafe extern "C" fn club_is_in_season(club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    exe::key_nation_club_in_season(club, 0, 0) // the season of the club’s nation
}

/// `club_is_in_major_nation(club)`: 1 if the club's nation (+0x53) has the id (+0) of one of the index.cpp keys England
/// 0x9cf2e4, France 0x9cf300, Germany 0x9cf314, Italy 0x9cf360, Spain 0x9cf488 or Holland 0x9cf338; null club or
/// nation: 0.
///
/// Only `al` (the rest of eax is the nation's id). At creation, through the default contract (`exe::staff_contract_default_staff_contract`) and the default
/// wage (`exe::staff_contract_calc_unset_default_wage`).
/// Original: `club_is_in_major_nation` (0x549e50, Database.cpp, cdecl).
///
/// [pt] { 0x549e50 } (cdecl, Database.cpp): `club_is_in_major_nation(clube)`: 1 se a nação do clube (+0x53) tem o id
/// (+0) de uma das chaves Inglaterra 0x9cf2e4, França 0x9cf300, Alemanha 0x9cf314, Itália 0x9cf360, Espanha 0x9cf488 ou
/// Holanda 0x9cf338; clube ou nação nulos: 0. Só `al` (o resto do eax é o id da nação). Na criação, pelo contrato
/// padrão (0x864550) e pelo salário padrão (0x862720).
pub(crate) unsafe extern "C" fn club_is_in_major_nation(club: usize) -> u8 {
    let club = Club(club);
    if club.is_null() {
        return 0;
    }
    let nation = club.nation().addr();
    if nation == 0 {
        return 0;
    }
    let id = read_i32(nation);
    let keys = [KEY_NATION_ENGLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_HOLLAND]; // Netherlands (club)
    keys.iter().any(|&k| read_i32(k) == id) as u8
}

/// `nation_is_major_nation(nation)`: the same six-key test as `club_is_in_major_nation` on the id (+0) of the nation itself; null: 0.
///
/// Only `al`. In the career, through the youth regeneration (PLAYER_REGEN::regenerate_as_youth_player `regenerate_as_youth_player`).
/// Original: `nation_is_major_nation` (0x549ea0, Database.cpp, cdecl).
///
/// [pt] { 0x549ea0 } (cdecl, Database.cpp): `nation_is_major_nation(nacao)`: o mesmo teste das seis chaves de 0x549e50
/// sobre o id (+0) da própria nação; nula: 0. Só `al`. Na carreira, pela regeneração dos jovens
/// (PLAYER_REGEN::regenerate_as_youth_player 0x7abac0).
pub(crate) unsafe extern "C" fn nation_is_major_nation(nation: usize) -> u8 {
    if nation == 0 {
        return 0;
    }
    let id = read_i32(nation);
    let keys = [KEY_NATION_ENGLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_HOLLAND]; // Holland (nation)
    keys.iter().any(|&k| read_i32(k) == id) as u8
}

/// The person's 79-byte record in the staff state table (G_STAFF_STATE_TABLE) holding the two relationship links
/// (+0x27/+0x37, +0x3b/+0x4b); None for a null person or id >= staff count.
///
/// [pt] O registro de contrato (79 bytes, tabela [0xae2d0c]) da `pessoa` como as 0x549c90/0x549cf0/0x549d60 o acham:
/// pessoa nula ou id (+0) ≥ [0xae2364] (comparação com sinal: id negativo passa, como na original): nenhum.
/// Os dois vínculos ficam em +0x27/+0x37 e +0x3b/+0x4b (id da outra pessoa, valor), −1 = vazio.
pub(crate) unsafe fn database_staff_state_record(person: usize) -> Option<usize> {
    if person == 0 {
        return None;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)))
}

/// Reads an i32 at offset d of a staff state record.
pub(crate) unsafe fn read_state_i32(c: usize, d: usize) -> i32 {
    read_i32(c.wrapping_add(d))
}

/// Writes an i32 at offset d of a staff state record.
pub(crate) unsafe fn write_state_i32(c: usize, d: usize, v: i32) {
    write_i32(c.wrapping_add(d), v)
}

/// `staff_clear_opinion_links(person)`: −1 in both links (id and value) of the person's contract (+0x27, +0x37, +0x3b,
/// +0x4b); null person or id ≥ [0xae2364]: nothing.
///
/// At creation, for each generated contract (0x85e960, through CONTRACT_MANAGER::initialise_all_staff_contracts); in
/// the career, in offers and complaints.
/// Original: `staff_clear_opinion_links` (0x549c90, Database.cpp, cdecl).
///
/// [pt] { 0x549c90 } (cdecl, Database.cpp): `staff_clear_opinion_links(pessoa)`: −1 nos dois vínculos do contrato da
/// pessoa (+0x27, +0x37, +0x3b, +0x4b). Na criação, a cada contrato gerado (0x85e960, por
/// CONTRACT_MANAGER::initialise_all_staff_contracts); na carreira, nas propostas e nas queixas.
pub(crate) unsafe extern "C" fn staff_clear_opinion_links(person: usize) {
    if let Some(c) = database_staff_state_record(person) {
        for k in [0usize, 0x14] {
            write_state_i32(c, k + 0x27, -1);
            write_state_i32(c, k + 0x37, -1); // the value of the empty link
        }
    }
}

/// `staff_add_opinion_link(person, other, value)`: with both people valid (not null, id < [0xae2364]), writes the id of
/// `other` and the `value` into the first link if it is empty (+0x27 = −1), otherwise into the second (+0x3b/+0x4b,
/// overwriting whatever is there).
///
/// In the career, through the contract events (`exe::staff_contract_apply_event_flags`).
/// Original: `staff_add_opinion_link` (0x549cf0, Database.cpp, cdecl).
///
/// [pt] { 0x549cf0 } (cdecl, Database.cpp): `staff_add_opinion_link(pessoa, outra, valor)`: com as duas pessoas válidas
/// (não nulas, id < [0xae2364]), grava o id da `outra` e o `valor` no primeiro vínculo se ele está vazio (+0x27 = −1),
/// senão no segundo (+0x3b/+0x4b, por cima do que houver). Na carreira, pelos eventos do contrato (0x867700).
pub(crate) unsafe extern "C" fn staff_add_opinion_link(person: usize, other: usize, value: i32) {
    let Some(c) = database_staff_state_record(person) else { return };
    if other == 0 {
        return;
    }
    let id = read_i32(other);
    if id >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if read_state_i32(c, 0x27) == -1 {
        write_state_i32(c, 0x27, id);
        write_state_i32(c, 0x37, value);
    } else {
        write_state_i32(c, 0x3b, id); // the second link
        write_state_i32(c, 0x4b, value);
    }
}

/// `staff_remove_opinion_link(person, value)`: empties (−1 in the id and the value) the first link with that `value`
/// (+0x37) or, if not, the second (+0x4b); none with the value: nothing.
///
/// In the career, when the complaint passes (`check_transfer_request_withdrawal`, `exe::staff_contract_clear_grievances`).
/// Original: `staff_remove_opinion_link` (0x549d60, Database.cpp, cdecl).
///
/// [pt] { 0x549d60 } (cdecl, Database.cpp): `staff_remove_opinion_link(pessoa, valor)`: esvazia (−1 no id e no valor) o
/// primeiro vínculo com esse `valor` (+0x37) ou, se não, o segundo (+0x4b); nenhum com o valor: nada. Na carreira,
/// quando a queixa passa (0x867240, 0x8674e0).
pub(crate) unsafe extern "C" fn staff_remove_opinion_link(person: usize, value: i32) {
    let Some(c) = database_staff_state_record(person) else { return };
    if read_state_i32(c, 0x37) == value {
        write_state_i32(c, 0x27, -1);
        write_state_i32(c, 0x37, -1);
    } else if read_state_i32(c, 0x4b) == value {
        write_state_i32(c, 0x3b, -1); // empties the second link
        write_state_i32(c, 0x4b, -1);
    }
}

/// `database_get_club_assistant(club)`: the club's assistant (+0xd3); without one, that of the partner A/B team
/// (`exe::club_get_linked_b_team` with null and 1, which zeroes the club's flag +0x244 when it rejects the partner); none: 0.
///
/// Null club: the original ("Error" box and 0). In the career, through the coach distribution (`update_coach_assignments`, in hirings and
/// transfers), the regeneration (`process_regen_queue`) and the screens.
/// Original: `database_get_club_assistant` (0x53cb80, Database.cpp, cdecl).
///
/// [pt] { 0x53cb80 } (cdecl, Database.cpp): `database_get_club_assistant(clube)`: o auxiliar (+0xd3) do clube; sem ele, o
/// do time parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro); nenhum: 0.
/// Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos treinadores (0x8b2880, nas
/// contratações e transferências), pela regeneração (0x7a65d0) e pelas telas.
pub(crate) unsafe extern "C" fn database_get_club_assistant(club: usize) -> usize {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_ASSISTANT)(club.addr());
    }
    let rd = |a: usize| read_usize(a);
    let a = club.assistant_manager();
    if a != 0 {
        return a;
    }
    let p = Club(exe::club_get_linked_b_team(club, (0) as *mut u32, 1));
    if p.is_null() {
        return 0;
    }
    p.assistant_manager() // the partner’s assistant
}

/// `database_get_club_coaches(club, array)`: the club's 5 coaches (+0x19f) into `array`.
///
/// Without an A/B partner (`exe::club_get_linked_b_team` with null and 1): the 5 slots as they are. With a partner: the club's non-null ones
/// and then the partner's, up to 5; the rest zeroed only when the two together do not fill the 5. 8-bit counter in the
/// original (at most 5). Returns 1. Null club: the original ("Error" box and 0). In the career, through the coach
/// distribution (`update_coach_assignments`).
/// Original: `database_get_club_coaches` (0x53cc30, Database.cpp, cdecl).
///
/// [pt] { 0x53cc30 } (cdecl, Database.cpp): `database_get_club_coaches(clube, vetor)`: os 5 treinadores (+0x19f) do clube no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): as 5 casas como estão. Com parceiro: os não nulos do clube e
/// depois os do parceiro até 5; o resto zerado só quando os dois juntos não enchem as 5. Contador de 8 bits na
/// original (no máximo 5). Devolve 1. Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos
/// treinadores (0x8b2880).
pub(crate) unsafe extern "C" fn database_get_club_coaches(club: usize, array: *mut usize) -> u32 {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_COACHES)(club.addr(), array);
    }
    let partner: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let rd = |a: usize| read_usize(a);
    let p = Club(partner(club.addr(), 0, 1));
    if p.is_null() {
        for j in 0..5 {
            core::ptr::write_unaligned(array.add(j), rd(club.addr() + club::COACHES + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..5 {
        let t = rd(club.addr() + club::COACHES + 4 * j);
        if t != 0 {
            core::ptr::write_unaligned(array.add(n), t);
            n += 1;
        }
    }
    for j in 0..5 {
        if n >= 5 {
            return 1;
        }
        let t = rd(p.addr() + club::COACHES + 4 * j); // the partner’s coaches
        if t != 0 {
            core::ptr::write_unaligned(array.add(n), t);
            n += 1;
        }
    }
    for k in n..5 {
        core::ptr::write_unaligned(array.add(k), 0);
    }
    1
}

/// `database_count_club_coaches(club)`: how many of the club's 5 coaches (+0x19f) are valid people (not null, id (+0) <
/// [0xae2364], signed) plus those of the partner A/B team (`exe::club_get_linked_b_team` with null and 1, always called).
///
/// Only `al` (the rest of eax is the partner's). Null club: the original ("Error" box and 0). In the career, through
/// the regeneration queue (PLAYER_REGEN::process_regen_queue `process_regen_queue`).
/// Original: `database_count_club_coaches` (0x53c8d0, Database.cpp, cdecl).
///
/// [pt] { 0x53c8d0 } (cdecl, Database.cpp): `database_count_club_coaches(clube)`: quantos dos 5 treinadores (+0x19f) do
/// clube são pessoas válidas (não nulas, id (+0) < [0xae2364], com sinal) mais os do time parceiro A/B (0x540a50 com
/// nulo e 1, sempre chamada). Só `al` (o resto do eax é o do parceiro). Clube nulo: a original (caixa "Error" e 0).
/// Na carreira, pela fila da regeneração (PLAYER_REGEN::process_regen_queue 0x7a65d0).
pub(crate) unsafe extern "C" fn database_count_club_coaches(club: usize) -> u8 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_COACHES)(club);
    }
    let contar = |c: Club| {
        let total = read_i32(G_STAFF_COUNT);
        (0..5).filter(|&j| {
            let t = read_usize(c.addr() + club::COACHES + 4 * j);
            t != 0 && *(t as *const i32) < total // valid coach
        }).count() as u8
    };
    let mut n = contar(Club(club));
    let p = exe::club_get_linked_b_team(club, (0) as *mut u32, 1);
    if p != 0 {
        n = n.wrapping_add(contar(Club(p))); // and the partner’s
    }
    n
}

/// `nation_get_english_three_letter_name(nation)`: the nation's three-letter code, or its English one from eng.lng when
/// another language is loaded.
///
/// Language ([0x9b5984]) 0 or 1: its own code (+0x53). Otherwise the eng.lng table ([0xae2408]): nations in 109-byte
/// rows from +0x2c, by id (without checking the count), the code at +0x4f of the row. SEH frame only on first use.
/// Original: `nation_get_english_three_letter_name` (0x5454c0, Database.cpp, cdecl).
///
/// [pt] { 0x5454c0 } (cdecl, Database.cpp): `nation_get_english_three_letter_name(nação)`: idioma ([0x9b5984]) 0 ou
/// 1: a própria sigla (+0x53). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de
/// +0x2c, pelo id (sem conferir a quantidade), a sigla em +0x4f da linha. Quadro SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn nation_get_english_three_letter_name(nation: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return nation.wrapping_add(0x53); // nation abbreviation
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: the first use allocates (new 0x40) and loads eng.lng (0x544fc0) under the SEH frame: the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_THREE_LETTER_NAME)(nation);
    }
    let id = read_i32(nation);
    read_usize(t + english_name_tables::NATION_ROWS).wrapping_add((id as usize).wrapping_mul(109)).wrapping_add(0x4f) // abbreviation in the row
}

/// the competition's name, or its English one from eng.lng (83-byte rows: club ones by id, nation ones by id minus the
/// club count) when another language is loaded
/// Original: `comp_get_english_name` (0x5455a0, Database.cpp, cdecl).
///
/// [pt] { 0x5455a0 } (cdecl, Database.cpp): `comp_get_english_name(competição)`: idioma 0 ou 1: o próprio nome (+4).
/// Senão a tabela do eng.lng, linhas de 83 bytes: id ≥ [t+0x14] (quantas de clube): a de nação `id − [t+0x14]`
/// em [t+0x28]; senão a de clube `id` em [t+0x38] (comparação com sinal). Quadro SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn comp_get_english_name(comp: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return comp.wrapping_add(4); // competition name
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: first use (allocates and loads eng.lng under the SEH frame): the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_COMP_GET_ENGLISH_NAME)(comp);
    }
    let id = read_i32(comp);
    let of_club = read_i32(t + english_name_tables::CLUB_COMP_COUNT);
    if id >= of_club {
        read_usize(t + english_name_tables::NATION_COMP_ROWS).wrapping_add((id.wrapping_sub(of_club) as usize).wrapping_mul(83)) // nation competition
    } else {
        read_usize(t + english_name_tables::CLUB_COMP_ROWS).wrapping_add((id as usize).wrapping_mul(83)) // club competition
    }
}

/// the staff competition's name, or its English one from eng.lng (79-byte rows, by id) when another language is loaded
/// Original: `staff_comp_get_english_name` (0x5456a0, Database.cpp, cdecl).
///
/// [pt] { 0x5456a0 } (cdecl, Database.cpp): `staff_comp_get_english_name(competição de pessoal)`: idioma 0 ou 1: o
/// próprio nome (+4). Senão a tabela do eng.lng, linhas de 79 bytes em [t+0x3c], pelo id. SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn staff_comp_get_english_name(comp: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return comp.wrapping_add(4); // staff competition name
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: first use (allocates and loads eng.lng under the SEH frame): the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STAFF_COMP_GET_ENGLISH_NAME)(comp);
    }
    let id = read_i32(comp);
    read_usize(t + english_name_tables::STAFF_COMP_ROWS).wrapping_add((id as usize).wrapping_mul(79)) // staff competition in the table
}

/// Self-test (CM_AUTOTESTE=1 or =banco, comparison mode): neither the career nor the save goes through the eng.lng path
/// (the test game is in English).
///
/// 512 cases with language 0 to 13 and a fake eng.lng table (never null: the first use would load the real file), club
/// counts 0 to 9, ids −2 to 13; the globals are restored at the end.
///
/// [pt] Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação): nem a carreira nem o save passam pelo caminho do eng.lng
/// (o jogo dos testes está em inglês). 512 casos com o idioma de 0 a 13 e uma tabela do eng.lng falsa (nunca
/// nula: o primeiro uso carregaria o arquivo de verdade), quantas de clube de 0 a 9, ids de −2 a 13; as globais
/// voltam no fim.
pub unsafe fn autoteste_dbm_1() {
    let (t0, d0) = SCOREBOARD.counts();
    let mut s = TestRng(0x4442_4d31_6964_696f);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // the eng.lng table (new 0x40): +0 .. +0x1c counts, +0x20 .. +0x3c rows
    let mut registro = [0u8; 0x60];
    for _ in 0..512 {
        let z = s.next_u64();
        *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = (z % 14) as u32;
        tabela[5] = ((z >> 4) % 10) as usize;
        for k in 8..16 {
            tabela[k] = 0x1000_0000 * (k - 7) + ((z >> (8 + k)) as usize & 0xffff0);
        }
        *(G_ENGLISH_NAME_TABLES as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        let r = registro.as_ptr() as usize;
        compare_nation_get_english_three_letter_name(r);
        compare_comp_get_english_name(r);
        compare_staff_comp_get_english_name(r);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = SCOREBOARD.counts();
    linha(&format!("database language (English names 1): SELFTEST {} cases, {} divergences", t1 - t0, d1 - d0));
}

/// `stadium_get_english_name(stadium)`: the stadium's name, or its English one from eng.lng when another language is
/// loaded.
///
/// Language ([0x9b5984]) 0 or 1: its own name (+4). Otherwise the eng.lng table ([0xae2408]), 52-byte rows at [t+0x34],
/// by id. SEH frame only on first use (new 0x40 and the load 0x544fc0), which stays with the original.
/// Original: `stadium_get_english_name` (0x545990, Database.cpp, cdecl).
///
/// [pt] { 0x545990 } (cdecl, Database.cpp): `stadium_get_english_name(estádio)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]), linhas de 52 bytes em [t+0x34], pelo id. Quadro SEH só no
/// primeiro uso (new 0x40 e a carga 0x544fc0), que fica com a original.
pub(crate) unsafe extern "C" fn stadium_get_english_name(stadium: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return stadium.wrapping_add(4); // stadium name
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: first use (allocates and loads eng.lng under the SEH frame): the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STADIUM_GET_ENGLISH_NAME)(stadium);
    }
    let id = read_i32(stadium);
    read_usize(t + english_name_tables::STADIUM_ROWS).wrapping_add((id as usize).wrapping_mul(52)) // stadium in the table
}

/// Os nomes dos `.lng` no .rdata do exe por idioma: 0, 1, 11 e 12 "eng.lng"; 2 fra, 3 ger, 4 ita, 5 esp, 6 dan,
/// 7 hol, 8 nor, 9 por, 10 swe; os outros (13, negativos...) nenhum.
pub(crate) const LANGUAGE_FILES: [usize; 13] = [0x9b632c, 0x9b632c, 0x9b633c, 0x9b6334, 0x9b6324, 0x9b630c, 0x9b62fc,
    0x9b6344, 0x9b6314, 0x9b631c, 0x9b6304, 0x9b632c, 0x9b632c];

/// builds CM3_TEMP\<language>.lng (0, 1, 11 and 12: eng.lng; 2 to 10: fra, ger, ita, esp, dan, hol, nor, por, swe) and
/// tells whether it exists; any other language, 0
/// Original: `database_language_file_exists` (0x535930, Database.cpp, cdecl).
///
/// [pt] { 0x535930 } (cdecl, Database.cpp): `database_language_file_exists(idioma)`: o caminho `CM3_TEMP\<.lng>`
/// (0x90bde0, no buffer 0xde279c) e se existe (0x90c120: `_access(.., 0) == 0`, 1 ou 0); idioma sem `.lng`, 0
/// sem montar o caminho. TapeCall pelo `load_database` (0x5286e4, o idioma das preferências) e na criação.
pub(crate) unsafe extern "C" fn database_language_file_exists(language: i32) -> i32 {
    match LANGUAGE_FILES.get(language as u32 as usize) {
        Some(&nome) => exe::utils_file_exists(exe::utils_build_file_path(0x987da0, nome)), // "CM3_TEMP"
        None => 0,
    }
}

/// Calls `soltar` on each non-null of the 8 eng.lng table rows (+0x20..+0x3c) of the table.
///
/// [pt] As 8 linhas da tabela do eng.lng (+0x20 .. +0x3c), na ordem, cada uma não nula entregue a `soltar`.
pub(crate) unsafe fn for_each_english_table(t: usize, mut soltar: impl FnMut(usize)) {
    for k in 0..8 {
        let p = read_usize(t + english_name_tables::CLUB_ROWS + 4 * k); // table row in English
        if p != 0 {
            soltar(p);
        }
    }
}

/// `eng_lng_free_tables(table)` (thiscall): `free` `exe::crt_free` on each of the 8 non-null rows, from +0x20 to +0x3c.
///
/// The pointers stay; the callers, `load_database` 0x528503 and the language switch 0x533aa6, delete the table with
/// `delete` `exe::crt_operator_delete` and zero [0xae2408].
/// Original: `eng_lng_free_tables` (0x544f30, Database.cpp, thiscall).
///
/// [pt] { 0x544f30 } (thiscall, Database.cpp): `eng_lng_free_tables(tabela)`: `free` 0x9452ca de cada uma das 8 linhas
/// não nulas, de +0x20 a +0x3c (os ponteiros ficam; quem chama, `load_database` 0x528503 e a troca de idioma
/// 0x533aa6, apaga a tabela com `delete` 0x944c94 e zera [0xae2408]).
pub(crate) unsafe extern "thiscall" fn eng_lng_free_tables(t: usize) {
    for_each_english_table(t, |p| exe::crt_free(p));
}

/// Comparador: soltar duas vezes soltaria os mesmos blocos duas vezes.
// simplification: original only (like those of the 232nd); the copy is proved by autoteste_dbm_2 (the order of the loose blocks).

/// Self-test (CM_AUTOTESTE=1 or =banco, comparison mode).
///
/// (1) The stadium: 256 cases with language 0 to 13 and a fake eng.lng table (never null), ids −2 to 13.
///
/// (2) The language's `.lng`: languages −3 to 16 against the original (answer and built path).
///
/// (3) Freeing the tables: 64 fake tables with null or non-null rows chosen at random; the copy hands them to a list
/// (without freeing anything), which must be the non-null ones from +0x20 to +0x3c, in the original's order (the
/// original really frees them, so the two cannot both run).
///
/// The globals are restored at the end.
///
/// [pt] Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação). (1) O estádio: 256 casos com o idioma de 0 a 13 e uma
/// tabela do eng.lng falsa (nunca nula), ids de −2 a 13. (2) O `.lng` do idioma: idiomas de −3 a 16 contra a
/// original (resposta e caminho montado). (3) Soltar as tabelas: 64 tabelas falsas com linhas nulas ou não
/// sorteadas; a cópia entrega a uma lista (sem soltar nada), que tem de ser as não nulas de +0x20 a +0x3c, na
/// ordem da original (a original solta de verdade e não dá para rodar as duas). As globais voltam no fim.
pub unsafe fn autoteste_dbm_2() {
    let (t0, d0) = SCOREBOARD.counts();
    let mut s = TestRng(0x4442_4d32_6573_7461);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.next_u64();
        *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = (z % 14) as u32;
        tabela[13] = 0x3000_0000 + ((z >> 8) as usize & 0xffff0);
        *(G_ENGLISH_NAME_TABLES as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        compare_stadium_get_english_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    for idioma in -3..=16 {
        compare_database_language_file_exists(idioma);
    }
    for _ in 0..64 {
        let z = s.next_u64();
        for k in 0..8 {
            tabela[8 + k] = if (z >> k) & 1 == 0 { 0 } else { 0x1000_0000 * (k + 1) + ((z >> (8 + k)) as usize & 0xffff0) };
        }
        let esperado: Vec<usize> = tabela[8..].iter().copied().filter(|&p| p != 0).collect();
        let mut soltos = Vec::new();
        for_each_english_table(tabela.as_ptr() as usize, |p| soltos.push(p));
        SCOREBOARD.record(soltos == esperado, || format!("eng_lng_free_tables: soltos {:x?} esperados {:x?}", soltos, esperado));
    }
    let (t1, d1) = SCOREBOARD.counts();
    linha(&format!("database language (stadium, .lng, release): SELFTEST {} cases, {} divergences", t1 - t0, d1 - d0));
}

/// Shows the Database.cpp error box ("v%s %s.%s %d" with the line) and clears G_MOUSE_LEFT_BUTTON_DOWN; the game
/// continues.
///
/// [pt] A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
pub(crate) unsafe fn database_error_box(row: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9b5ac8, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Database.cpp error box followed by fatal cleanup and exit(-1).
///
/// [pt] A caixa do Database.cpp seguida do fim do jogo (0x5cc7d0, 0x5a8f60 e exit(−1)), como a original sem memória.
pub(crate) unsafe fn database_fatal_error(row: u32) -> ! {
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    database_error_box(row);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// True for a club of England (KEY_NATION_ENGLAND) with reputation (+0x80) at most 500, which `database_generate_extra_non_playing_staff` and `database_trim_club_table`
/// treat separately.
///
/// [pt] Clube da nação da chave 0x9cf2e4 com reputação (+0x80, 16 bits com sinal) até 500: 0x527240 e 0x5267c0 o tratam à parte.
pub(crate) unsafe fn is_weak_english_club(c: Club) -> bool {
    let nation = c.nation().addr();
    nation != 0 && read_i32(nation) == read_i32(KEY_NATION_ENGLAND) && c.reputation() <= 500
}

/// Initial check of `database_generate_extra_non_playing_staff`: some person with job 8 (+0x3d) and a club (+0x39) that is not in that club's 5 coach
/// slots (+0x19f) [inference: job 8 = coach].
///
/// [pt] A conferência inicial da 0x527240: alguma pessoa de cargo 8 (+0x3d) com clube (+0x39) que não a tem nas 5 casas +0x19f.
pub(crate) unsafe fn any_coach_missing_from_club() -> bool {
    let mut i = 0i32;
    while i < read_i32(G_STAFF_COUNT) {
        let s = Staff(read_usize(G_STAFF_TABLE) + i as usize * 110);
        let c = s.club_job();
        if s.job_for_club() == 8 && !c.is_null() && !(0..5).any(|j| c.coaches(j) == s.addr()) {
            return true;
        }
        i += 1;
    }
    false
}

/// Fills an extra staff person (row staff_count - generated - 17): first and second names for the nation and a common
/// name (stored row, first empty common name, or a random one); returns the record.
///
/// [pt] Uma pessoa do pessoal extra da 0x527240: a linha [0xae2364] − gerados − 17 (das reservas, de trás para a frente),
/// primeiro e segundo nome da nação (0x539c60, 0x53a1b0) e o nome comum: a linha guardada em [0x9b59b4] ou, sem ela
/// (−1), a primeira linha de nome comum vazia (guardada) ou, sem nenhuma, uma sorteada entre todas (sem guardar).
pub(crate) unsafe fn make_extra_staff(nation: usize, gerados: i32) -> usize {
    let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - gerados - 0x11) as usize * 110));
    s.set_first_name(exe::database_pick_first_name_for_nation(nation));
    s.set_second_name(exe::database_pick_second_name_for_nation(nation));
    let common = |k: i32| read_usize(G_COMMON_NAME_TABLE).wrapping_add((k as usize).wrapping_mul(0x3c));
    let stored = read_i32(G_EMPTY_COMMON_NAME_INDEX);
    let name = if stored != -1 {
        common(stored)
    } else {
        let n = read_i32(G_COMMON_NAME_COUNT);
        match (0..n).find(|&j| read_u8(common(j)) == 0) {
            Some(j) => {
                write_i32(G_EMPTY_COMMON_NAME_INDEX, j);
                common(j)
            }
            None => common(exe::utils_random(n)),
        }
    };
    s.set_common_name(name);
    s.addr()
}

/// Sets the extra person's birth date (+0x10): year 1940 + rand(26), month rand(11), day rand(27)+1, drawn in that
/// order.
///
/// [pt] A data de nascimento da pessoa extra (+0x10, 0x54a130): ano 1940 + exe::utils_random(26), mês exe::utils_random(11), dia exe::utils_random(27) + 1,
/// sorteados nessa ordem.
pub(crate) unsafe fn set_extra_staff_birth_date(s: Staff) {
    let year = exe::utils_random(0x1a) + 0x794; // 1940 + up to 25
    let month = exe::utils_random(0xb);
    let day = exe::utils_random(0x1b) + 1;
    exe::date_set(s.addr() + staff::DATE_OF_BIRTH, day as u32, month as u32, year as u32, -1);
}

/// `database_generate_extra_non_playing_staff()`: fills coaches, scouts and physios of the clubs of chosen nations by
/// reputation and creates unattached managers for clubs without one. Called once at creation (by 0x524d70 at the start
/// of the trims).
///
/// Checks the people with job 8 (box 0x62f on the first one outside the club's slots, and carries on). Then, in each
/// club that is not a national team (id < [0xae235c] − 2 × [0xae2350]), is not a weak club of key 0x9cf2e4, has a
/// chosen nation (+0x11c) and is not the A team of a pair (`exe::club_get_linked_b_team` with 1): tops up coaches (+0x19f, 5, job 8), scouts
/// (+0x1b3, 7, job 9) and physios (+0x1cf, 3, job 10) up to (reputation + 500) × slots / 10000, between 1 and the
/// number of slots, with people from the reserve rows (make_extra_staff, nation +0x1a, +0x5f = 1, random birth date,
/// names of the nation) placed in the first free slot.
///
/// Finally, each club without a manager (+0xcf) that is a national team or (with nation, division +0x57, chosen nation
/// and reputation ≠ 0) or with reputation > 7500, except the weak ones of 0x9cf2e4, gets a manager (job 5, +0x28 = 5)
/// with no club and no national team (he does not go into the slot).
///
/// Over 23,000: box (0x65f, 0x6aa, 0x6f5, 0x734) and 0. Writes "generated %ld extra non playing staff" to 0xde1f64 and
/// returns 1. The pair local is only read when `exe::club_get_linked_b_team` returns a partner.
/// Original: `database_generate_extra_non_playing_staff` (0x527240, Database.cpp, cdecl).
///
/// [pt] { 0x527240 } (cdecl, Database.cpp): `database_generate_extra_non_playing_staff()`, uma vez na criação (chamada pela
/// 0x524d70 no começo dos cortes). Confere as pessoas de cargo 8 (caixa 0x62f na primeira fora das casas do clube, e
/// segue). Depois, em cada clube que não é seleção (id < [0xae235c] − 2 × [0xae2350]), não é fraco da chave 0x9cf2e4,
/// tem nação escolhida (+0x11c) e não é time A de um par (0x540a50 com 1): completa técnicos (+0x19f, 5, cargo 8),
/// olheiros (+0x1b3, 7, cargo 9) e fisioterapeutas (+0x1cf, 3, cargo 10) até (reputação + 500) × casas / 10000, entre 1
/// e o número de casas, com pessoas das reservas (make_extra_staff, nação +0x1a, +0x5f = 1, nascimento sorteado) postas
/// na primeira casa livre. Por fim, cada clube sem treinador (+0xcf) que seja seleção ou (com nação, divisão +0x57, nação
/// escolhida e reputação ≠ 0) ou reputação > 7500, fora os fracos da 0x9cf2e4, ganha um treinador (cargo 5, +0x28 = 5)
/// sem clube e sem seleção (não vai para a casa). Mais de 23.000: caixa (0x65f, 0x6aa, 0x6f5, 0x734) e 0. Grava "generated
/// %ld extra non playing staff" em 0xde1f64 e devolve 1. A local do par só é lida quando 0x540a50 devolve parceiro.
pub(crate) unsafe extern "C" fn database_generate_extra_non_playing_staff() -> u32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let rep = |c: Club| c.reputation() as i32;
    let byte = |a: usize| read_u8(a);
    if any_coach_missing_from_club() {
        database_error_box(0x62f);
    }
    let national_team = |c: usize| gi(c) >= gi(G_CLUB_COUNT) - 2 * gi(G_NATION_COUNT);
    let mut gerados = 0i32;
    for &(d, k, cargo, linha) in &[(0x19fusize, 5usize, 8u8, 0x65fu32), (0x1b3, 7, 9, 0x6aa), (0x1cf, 3, 10, 0x6f5)] {
        let mut i = 0i32;
        while i < gi(G_CLUB_COUNT) {
            let c = rd(G_CLUB_TABLE) + i as usize * 581;
            i += 1;
            if is_weak_english_club(Club(c)) || national_team(c) {
                continue;
            }
            let nation = read_usize(c + club::NATION);
            if nation == 0 || read_u8(nation + nation::LEAGUE_SELECTED) == 0 {
                continue;
            }
            let mut eh_a = 0u32;
            if exe::club_get_linked_b_team(c, &mut eh_a, 1) != 0 && eh_a != 0 {
                continue;
            }
            let mut has = (0..k).filter(|&j| rd(c + d + 4 * j) != 0).count() as i32;
            let target = ((rep(Club(c)) + 500) * k as i32 / 10000).clamp(1, k as i32); // the commission by reputation
            while has < target {
                has += 1;
                if gerados >= 23000 {
                    database_error_box(linha);
                    return 0;
                }
                let s = make_extra_staff(read_usize(c + club::NATION), gerados);
                write_usize(s + staff::NATION, read_usize(c + club::NATION));
                write_u8(s + staff::CLASSIFICATION, 1);
                set_extra_staff_birth_date(Staff(s));
                if let Some(j) = (0..k).find(|&j| rd(c + d + 4 * j) == 0) {
                    write_usize(c + d + 4 * j, s);
                    write_usize(s + staff::CLUB_JOB, c);
                    write_u8(s + staff::JOB_FOR_CLUB, cargo);
                }
                gerados += 1;
            }
        }
    }
    let mut i = 0i32;
    while i < gi(G_CLUB_COUNT) {
        let c = rd(G_CLUB_TABLE) + i as usize * 581;
        i += 1;
        if is_weak_english_club(Club(c)) {
            continue;
        }
        let nation = read_usize(c + club::NATION);
        let vale = national_team(c) || (nation != 0 && read_usize(c + club::DIVISION) != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && rep(Club(c)) != 0) || rep(Club(c)) > 7500;
        if !vale || read_usize(c + club::MANAGER) != 0 {
            continue;
        }
        if gerados >= 23000 {
            database_error_box(0x734);
            return 0;
        }
        let s = make_extra_staff(nation, gerados);
        write_u8(s + staff::CLASSIFICATION, 1);
        write_u8(s + staff::JOB_FOR_CLUB, 5); // coach without a club
        write_usize(s + staff::CLUB_JOB, 0);
        write_usize(s + staff::NATIONAL_JOB, 0);
        write_u8(s + staff::JOB_FOR_NATION, 5);
        set_extra_staff_birth_date(Staff(s));
        gerados += 1;
    }
    format(0xde1f64, 0x9b5b10, gerados);
    1
}

/// `database_trim_club_table(size)`: trims the club table by the database size. Called once at creation (create_game).
/// Small = size 1 or 2.
///
/// Per nation (calloc of 0xd0 bytes per nation: 50 ids, count +0xc8, index of the lowest +0xcc), the clubs with the
/// highest reputation (+0x80, signed 16 bits): up to 20 (15 in the small one); a nation not chosen (+0x11c) and outside
/// the 15 nation keys: up to 5 (size 2) or 11 (size 1). Also kept: the clubs of people (+0x39, +0x24), those of the
/// competition histories (+0xa, +0xe, +0x12, in range) and, outside the small one, those of the history of people with
/// a chosen nation; the national teams; the club keys (`remap_club_key_id` receives each id with the new row and returns 1 if it
/// is a key); and those of chosen nations, except (in the small one) the weak ones of 0x9cf2e4.
///
/// Swaps the club pointers (club +0xb3/+0xb7/+0xbb, person +0x39/+0x24, preferences +4..+0x18, people history +0xa,
/// competition history +0xa..+0x16) for the new ones (null for those removed), builds [0xae23fc] (new row → old id,
/// malloc), compacts the table (id = new row), reallocs in place (another address: box 0x4ca and the end) and writes
/// "Club table has %d entries" to 0xde1f64.
///
/// Without [0xae23a8] (box) or out of memory on new: the original; out of memory on the callocs: box and the end, as
/// the original.
/// Original: `database_trim_club_table` (0x5267c0, Database.cpp, cdecl).
///
/// [pt] { 0x5267c0 } (cdecl, Database.cpp): `database_trim_club_table(tamanho)`, uma vez na criação (create_game). Pequena =
/// tamanho 1 ou 2. Por nação (calloc de 0xd0 bytes por nação: 50 ids, quantidade +0xc8, índice do menor +0xcc), os
/// clubes de maior reputação (+0x80, 16 bits com sinal): até 20 (15 na pequena); nação não escolhida (+0x11c) e fora das
/// 15 chaves de nação: até 5 (tamanho 2) ou 11 (tamanho 1). Ficam também os clubes das pessoas (+0x39, +0x24), os dos
/// históricos de competição (+0xa, +0xe, +0x12, na faixa) e, fora da pequena, os do histórico de pessoas com nação
/// escolhida; as seleções; as chaves de clube (0x60ba80 recebe cada id com a linha nova e devolve 1 se é chave); e os
/// de nação escolhida, menos (na pequena) os fracos da 0x9cf2e4. Troca os ponteiros de clube (clube +0xb3/+0xb7/+0xbb,
/// pessoa +0x39/+0x24, preferências +4..+0x18, histórico de pessoas +0xa, histórico de competições +0xa..+0x16) pelos
/// novos (nulo o que sai), monta [0xae23fc] (linha nova → id antigo, malloc), compacta a tabela (id = linha nova),
/// realloc no lugar (outro endereço: caixa 0x4ca e fim) e grava "Club table has %d entries" em 0xde1f64. Sem [0xae23a8]
/// (caixa) ou sem memória no new: a original; sem memória nos calloc: caixa e fim, como a original.
pub(crate) unsafe extern "C" fn database_trim_club_table(size: u32) {
    if read_usize(G_NATION_TABLE) == 0 {
        return original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_TRIM_CLUB_TABLE)(size);
    }
    crate::comparison::branch("creation trim the club table");
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let wi = |a: usize, v: i32| write_i32(a, v);
    let wr = |a: usize, v: usize| write_usize(a, v);
    let rep = |c: Club| c.reputation();
    let rep_id = |id: i32| rep(Club(rd(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(581))));
    let byte = |a: usize| read_u8(a);
    let new = exe::crt_operator_new((gi(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if new == 0 {
        return;
    }
    let t = size as u8;
    let pequena = t == 2 || t == 1;
    let marked = exe::crt_calloc(gi(G_CLUB_COUNT) as u32, 4);
    if marked == 0 {
        database_fatal_error(0x513);
    }
    let mark = |id: i32| wi(marked.wrapping_add((id as usize).wrapping_mul(4)), 1);
    let last = rd(G_CLUB_TABLE).wrapping_add((gi(G_CLUB_COUNT) as usize).wrapping_mul(581)).wrapping_sub(581);
    let nations = exe::crt_calloc(gi(G_NATION_COUNT) as u32, 0xd0);
    if nations == 0 {
        database_fatal_error(0x51b);
    }
    let limit = if pequena { 15 } else { 20 }; // clubs per nation
    let keys = [KEY_NATION_ENGLAND, KEY_NATION_ITALY, KEY_NATION_GERMANY, KEY_NATION_SPAIN, KEY_NATION_FRANCE, KEY_NATION_NORWAY, KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA, KEY_NATION_DENMARK, KEY_NATION_BELGIUM,
        KEY_NATION_HOLLAND, KEY_NATION_JAPAN, KEY_NATION_PORTUGAL, KEY_NATION_SCOTLAND, KEY_NATION_SWEDEN];
    let mut c = rd(G_CLUB_TABLE);
    while c <= last {
        let nation = read_usize(c + club::NATION);
        if nation != 0 {
            let id = gi(nation);
            let l = nations.wrapping_add((id as usize).wrapping_mul(0xd0));
            let cont = read_i32(l + nation_club_bucket::CLUB_COUNT);
            let strong = read_u8(nation + nation::LEAGUE_SELECTED) != 0 || keys.iter().any(|&k| id == gi(k));
            let enters = strong || match t {
                2 => cont <= 4,
                1 => cont <= 10,
                _ => true,
            };
            if enters {
                if cont < limit {
                    wi(l + cont as usize * 4, gi(c));
                    if cont == 0 || rep(Club(c)) < rep_id(gi(l + read_i32(l + nation_club_bucket::WEAKEST_SLOT) as usize * 4)) {
                        wi(l + nation_club_bucket::WEAKEST_SLOT, cont);
                    }
                    wi(l + nation_club_bucket::CLUB_COUNT, cont + 1);
                } else if rep(Club(c)) > rep_id(gi(l + read_i32(l + nation_club_bucket::WEAKEST_SLOT) as usize * 4)) {
                    wi(l + read_i32(l + nation_club_bucket::WEAKEST_SLOT) as usize * 4, gi(c));
                    for j in 0..cont {
                        if j == 0 || rep_id(gi(l + j as usize * 4)) < rep_id(gi(l + read_i32(l + nation_club_bucket::WEAKEST_SLOT) as usize * 4)) {
                            wi(l + nation_club_bucket::WEAKEST_SLOT, j);
                        }
                    }
                }
            }
        }
        c += 581;
    }
    let mut k = 0i32;
    while k < gi(G_NATION_COUNT) {
        let l = nations + k as usize * 0xd0;
        for j in 0..gi(l + nation_club_bucket::CLUB_COUNT) {
            mark(gi(l + j as usize * 4));
        }
        k += 1;
    }
    exe::crt_free(nations);
    let mut s = rd(G_STAFF_TABLE);
    let end = s.wrapping_add((gi(G_STAFF_COUNT) as usize).wrapping_mul(110)).wrapping_sub(110);
    while s <= end {
        for &d in &[0x39usize, 0x24] { // the person’s club and national team stay
            let p = rd(s + d);
            if p != 0 {
                mark(gi(p));
            }
        }
        s += 110;
    }
    let mut h = rd(G_CLUB_COMP_HISTORY_TABLE);
    let end = h.wrapping_add((gi(G_CLUB_COMP_HISTORY_COUNT) as usize).wrapping_mul(0x1a)).wrapping_sub(0x1a);
    while h <= end {
        for &d in &[0xausize, 0xe, 0x12] {
            let p = rd(h + d);
            if p != 0 {
                let id = gi(p);
                if id < gi(G_CLUB_COUNT) && id >= 0 {
                    mark(id);
                }
            }
        }
        h += 0x1a;
    }
    if !pequena {
        let mut h = rd(G_STAFF_HISTORY_TABLE);
        let end = h.wrapping_add((gi(G_STAFF_HISTORY_COUNT) as usize).wrapping_mul(0x11)).wrapping_sub(0x11);
        while h <= end {
            let club = Club(rd(h + 0xa));
            let q = Staff(rd(h + 4));
            if !club.is_null() && !q.is_null() {
                let nq = q.nation();
                let nc = club.nation();
                if (!nq.is_null() && nq.league_selected() != 0) || (!nc.is_null() && nc.league_selected() != 0) {
                    mark(gi(club.addr()));
                }
            }
            h += 0x11;
        }
    }
    let table = rd(G_CLUB_TABLE);
    let mut target = table;
    let mut c = table;
    while c <= last {
        let id = gi(c);
        let stays = if exe::remap_club_key_id(id, (target.wrapping_sub(table) as i32) / 581) != 0
            || gi(marked.wrapping_add((id as usize).wrapping_mul(4))) != 0
            || id >= gi(G_CLUB_COUNT) - 2 * gi(G_NATION_COUNT)
        {
            true
        } else {
            let nation = read_usize(c + club::NATION);
            !(nation == 0 || read_u8(nation + nation::LEAGUE_SELECTED) == 0 || (gi(nation) == gi(KEY_NATION_ENGLAND) && rep(Club(c)) <= 500 && pequena))
        };
        let home = new.wrapping_add((id as usize).wrapping_mul(4));
        if stays {
            wr(home, target);
            target += 581;
        } else {
            wr(home, 0);
        }
        c += 581;
    }
    exe::crt_free(marked);
    // the club pointers by the new ones (the old id is at +0 of the target, still in the old place)
    let swap = |field: usize| {
        let p = rd(field);
        wr(field, if p == 0 { 0 } else { rd(new.wrapping_add((gi(p) as usize).wrapping_mul(4))) });
    };
    for &(cont, base, tam, campos) in &[
        (0xae235cusize, G_CLUB_TABLE, 581usize, &[0xb3usize, 0xb7, 0xbb][..]),
        (G_STAFF_COUNT, G_STAFF_TABLE, 110, &[0x39, 0x24][..]),
        (0xae2370, G_PREFERENCES_TABLE, 0x34, &[4, 8, 0xc, 0x10, 0x14, 0x18][..]),
        (G_STAFF_HISTORY_COUNT, G_STAFF_HISTORY_TABLE, 0x11, &[0xa][..]),
        (G_CLUB_COMP_HISTORY_COUNT, G_CLUB_COMP_HISTORY_TABLE, 0x1a, &[0xa, 0xe, 0x12, 0x16][..]),
    ] {
        let mut i = 0i32;
        while i < gi(cont) {
            for &d in campos {
                swap(rd(base) + i as usize * tam + d);
            }
            i += 1;
        }
    }
    let n = gi(G_CLUB_COUNT);
    let ficam = (0..n).filter(|&i| rd(new + i as usize * 4) != 0).count();
    let ids = exe::crt_malloc((ficam as u32) << 2);
    write_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE, ids);
    if ids == 0 {
        exe::crt_operator_delete(new);
        return;
    }
    let mut row = 0i32;
    for i in 0..n {
        let d = rd(new + i as usize * 4);
        if d != 0 {
            // forward over the table itself (destination ≤ source), like the original’s rep movsd
            core::ptr::copy((rd(G_CLUB_TABLE) + i as usize * 581) as *const u8, d as *mut u8, 581);
            wi(rd(G_CLUB_ENGLISH_NAME_INDEX_TABLE) + row as usize * 4, i);
            wi(d, row);
            row += 1;
        }
    }
    write_i32(G_CLUB_COUNT, row);
    let r = exe::crt_realloc(rd(G_CLUB_TABLE), (row as u32).wrapping_mul(581));
    write_usize(G_NAT_CLUB_TABLE, rd(G_CLUB_TABLE).wrapping_add(((row - gi(0xae2360)) as usize).wrapping_mul(581)));
    if r != rd(G_CLUB_TABLE) {
        database_fatal_error(0x4ca);
    }
    exe::crt_operator_delete(new);
    format(0xde1f64, 0x9b5af4, gi(G_CLUB_COUNT));
}

/// `database_prune_comp_history_table()`: called once at creation. Compacts the club competition history ([0xae23d4],
/// [0xae237c] rows of 26 bytes): a row stays if it has a competition (+4) whose nation (+0x5d) is null or chosen
/// (+0x11c), copied forward within the table itself.
///
/// realloc (`exe::crt_realloc`) to the new size, which becomes [0xae23d4] (without checking the address); [0xae237c] = rows;
/// [0xae23dc] (the national teams part) = table + (rows − [0xae2384]) × 26; qsort (`exe::crt_qsort`) with `database_compare_comp_history_rows`.
/// Original: `database_prune_comp_history_table` (0x539ae0, Database.cpp, cdecl).
///
/// [pt] { 0x539ae0 } (cdecl, Database.cpp): `database_prune_comp_history_table()`, uma vez na criação. Compacta o histórico
/// de competições de clube ([0xae23d4], [0xae237c] linhas de 26 bytes): fica a linha com competição (+4) cuja nação
/// (+0x5d) é nula ou escolhida (+0x11c), copiada para a frente na própria tabela. realloc (0x945667) ao tamanho novo,
/// que vira [0xae23d4] (sem conferir o endereço); [0xae237c] = linhas; [0xae23dc] (a parte das seleções) = tabela +
/// (linhas − [0xae2384]) × 26; qsort (0x945333) com 0x540780.
pub(crate) unsafe extern "C" fn database_prune_comp_history_table() {
    crate::comparison::branch("creation prune the competition history");
    let mut k = 0usize;
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COMP_HISTORY_COUNT) {
        let l = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a;
        let comp = Competition(read_usize(l + 4));
        if !comp.is_null() {
            let nation = comp.nation();
            if nation.is_null() || nation.league_selected() != 0 { // a linha fica
                core::ptr::copy(l as *const u8, (read_usize(G_CLUB_COMP_HISTORY_TABLE) + k * 0x1a) as *mut u8, 0x1a);
                k += 1;
            }
        }
        i += 1;
    }
    let t = exe::crt_realloc(read_usize(G_CLUB_COMP_HISTORY_TABLE), (k * 0x1a) as u32);
    let national_teams = (k as i32).wrapping_sub(read_i32(G_NATION_COMP_HISTORY_COUNT));
    write_usize(G_CLUB_COMP_HISTORY_TABLE, t);
    write_i32(G_CLUB_COMP_HISTORY_COUNT, k as i32);
    write_usize(G_NATION_COMP_HISTORY_TABLE, t.wrapping_add((national_teams as usize).wrapping_mul(0x1a)));
    exe::crt_qsort(t, k as u32, 0x1a, 0x540780);
}

/// First differing byte between the original's and the port's memory snapshots: (address, original, port).
///
/// [pt] O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
pub(crate) fn first_memory_difference(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}

// simplification: compacts and reallocates the club table, allocates [0xae23fc] and swaps the club keys; does not run twice
// times. Original only; the copy takes the branch and is proved by the save proof (the .qst has the trimmed table).

// simplification: compacts and reallocates the competition history; it does not run twice. Original only; the copy is proved
// by the save proof (the pruned history goes into the .qst).

/// As tabelas do banco: (ponteiro, quantidade, bytes por linha).
pub(crate) const DATABASE_TABLES: [(usize, usize, usize); 12] = [
    (G_CLUB_TABLE, G_CLUB_COUNT, 581), (G_NATION_TABLE, G_NATION_COUNT, 0x122), (G_OFFICIAL_TABLE, G_OFFICIAL_COUNT, 0x2b), (G_STADIUM_TABLE, G_STADIUM_COUNT, 0x4e),
    (G_PREFERENCES_TABLE, 0xae2370, 0x34), (G_NON_PLAYER_TABLE, G_NON_PLAYER_COUNT, 0x44), (G_STAFF_TABLE, G_STAFF_COUNT, 0x6e), (G_STAFF_COMP_TABLE, 0xae2388, 0x65),
    (G_STAFF_HISTORY_TABLE, G_STAFF_HISTORY_COUNT, 0x11), (G_CITY_TABLE, G_CITY_COUNT, 0x38), (G_CLUB_COMP_HISTORY_TABLE, G_CLUB_COMP_HISTORY_COUNT, 0x1a), (G_CLUB_COMP_TABLE, G_CLUB_COMP_COUNT, 0x6b),
];

/// Os campos de pessoa que, fora da faixa, abrem a caixa na `resolve_database_indexes`: (campo, teto, tabela, bytes, texto, linha).
pub(crate) const PERSON_RANGE_CHECK_FIELDS: [(usize, usize, usize, usize, usize, u32); 3] = [
    (0x61, G_PLAYER_DATA_COUNT, G_PLAYER_DATA_TABLE, 0x46, 0x9b60b4, 0x18f6), // player
    (0x65, 0xae2370, G_PREFERENCES_TABLE, 0x34, 0x9b6088, 0x1907), // preferences
    (0x69, G_NON_PLAYER_COUNT, G_NON_PLAYER_TABLE, 0x44, 0x9b605c, 0x1918), // non-player
];

/// `resolve_database_indexes(lenient)`: replaces every id field of the 12 tables with the record pointer (table + id ×
/// bytes). Called at load (load_database, with 0) and at save (save_database, after writing).
///
/// In the original's order and with its rules: negative → null; some with a "greater than the count" ceiling (an id
/// equal to the count passes), others "from the count on", others with no ceiling. Club: nation, divisions, stadiums,
/// colours, people and clubs (+0x53..+0xbb, inclusive ceiling; a log line in 0xde1f64 for high nation, last division
/// and stadium); squad +0xd7 (50; with `lenient` = 0, from [0xae2364] on it becomes null, with a log line above
/// [0xae2364] + 0x52d), scouts, coaches, physios, directors, manager, chairman, assistant and the 20 selected (+0x1e0),
/// with log lines. Then nations, referees, stadiums, preferences, non-players, people (player, preferences and
/// non-player out of range: log line, box and null), person competitions, people history, cities, competition history
/// and club competitions.
///
/// Defects of the original, copied: the club's high last division (+0x5b) zeroes +0x60 (which later becomes competition
/// 0) and stays raw; the high neighbouring stadium (+0x48) zeroes the city (+0x38) and stays raw. Does not read the
/// target records nor allocate.
/// Original: `resolve_database_indexes` (0x5315a0, Database.cpp, cdecl).
///
/// [pt] { 0x5315a0 } (cdecl, Database.cpp): `resolve_database_indexes(leniente)`, na carga (load_database, com 0) e na
/// gravação (save_database, depois de gravar). Troca cada campo de id pelo ponteiro ao registro (tabela + id × bytes),
/// na ordem da original e com as regras dela: negativo → nulo; alguns com teto "maior que a quantidade" (o id igual à
/// quantidade passa), outros "a partir da quantidade", outros sem teto. Clube: nação, divisões, estádios, cores, pessoas
/// e clubes (+0x53..+0xbb, teto inclusivo; registro em 0xde1f64 nos altos de nação, última divisão e estádio); elenco
/// +0xd7 (50; com `leniente` = 0, a partir de [0xae2364] vira nulo, com registro acima de [0xae2364] + 0x52d), olheiros,
/// técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar e os 20 escalados (+0x1e0), com registro.
/// Depois nações, árbitros, estádios, preferências, não jogadores, pessoas (jogador, preferências e não jogador fora da
/// faixa: registro, caixa e nulo), competições de pessoa, histórico de pessoas, cidades, histórico de competições e
/// competições de clube. Erros do original, copiados: a última divisão alta do clube (+0x5b) zera +0x60 (que depois vira
/// a competição 0) e fica crua; o estádio vizinho alto (+0x48) zera a cidade (+0x38) e fica cru. Não lê os registros
/// alvo nem aloca.
pub(crate) unsafe extern "C" fn resolve_database_indexes(leniente: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let wr = |a: usize, v: usize| write_usize(a, v);
    let target = |base: usize, v: i32, size: usize| rd(base).wrapping_add((v as usize).wrapping_mul(size));
    // no cap: negative → null
    let a = |f: usize, base: usize, size: usize| {
        let v = gi(f);
        wr(f, if v < 0 { 0 } else { target(base, v, size) });
    };
    // inclusive ceiling: negative → null; above [ceiling] → Some(v), the caller decides; otherwise the record
    let b = |f: usize, ceiling: usize, base: usize, size: usize| -> Option<i32> {
        let v = gi(f);
        if v < 0 {
            wr(f, 0);
            None
        } else if v > gi(ceiling) {
            Some(v)
        } else {
            wr(f, target(base, v, size));
            None
        }
    };
    // exclusive ceiling: from [teto] up → Some(v), the caller decides; negative → null; otherwise the record
    let c = |f: usize, ceiling: usize, base: usize, size: usize| -> Option<i32> {
        let v = gi(f);
        if v >= gi(ceiling) {
            Some(v)
        } else {
            wr(f, if v < 0 { 0 } else { target(base, v, size) });
            None
        }
    };
    let rows = |count: usize, base: usize, size: usize, f: &dyn Fn(usize, i32)| {
        let mut i = 0i32;
        while i < gi(count) {
            f(rd(base) + i as usize * size, i);
            i += 1;
        }
    };
    rows(G_CLUB_COUNT, G_CLUB_TABLE, 581, &|e, i| {
        if let Some(v) = b(e + 0x53, G_NATION_COUNT, G_NATION_TABLE, 0x122) {
            format(0xde1f64, 0x9b62b0, v);
            wr(e + 0x53, 0);
        }
        if b(e + 0x57, G_CLUB_COMP_COUNT, G_CLUB_COMP_TABLE, 0x6b).is_some() {
            wr(e + 0x57, 0);
        }
        if let Some(v) = b(e + 0x5b, G_CLUB_COMP_COUNT, G_CLUB_COMP_TABLE, 0x6b) {
            format(0xde1f64, 0x9b6274, v);
            wr(e + 0x60, 0); // bug in the original: zeroes +0x60 and +0x5b stays raw
        }
        if b(e + 0x60, G_CLUB_COMP_COUNT, G_CLUB_COMP_TABLE, 0x6b).is_some() {
            wr(e + 0x60, 0);
        }
        if let Some(v) = b(e + 0x69, G_STADIUM_COUNT, G_STADIUM_TABLE, 0x4e) {
            format(0xde1f64, 0x9b6240, v);
            wr(e + 0x69, 0);
        }
        for &(d, ceiling, base, size) in &[
            (0x6eusize, 0xae2358usize, G_STADIUM_TABLE, 0x4eusize),
            (0x83, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x8b, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x93, 0xae2394, G_COLOUR_TABLE, 0x3a),
            (0x87, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x8f, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x97, 0xae2394, G_COLOUR_TABLE, 0x3a),
            (0x9b, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e), (0x9f, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e), (0xa3, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e),
            (0xa7, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e), (0xab, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e), (0xaf, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e),
            (0xb3, G_CLUB_COUNT, G_CLUB_TABLE, 581), (0xb7, G_CLUB_COUNT, G_CLUB_TABLE, 581), (0xbb, G_CLUB_COUNT, G_CLUB_TABLE, 581),
        ] {
            if b(e + d, ceiling, base, size).is_some() {
                wr(e + d, 0);
            }
        }
        for j in 0..50 {
            let f = e + 0xd7 + 4 * j;
            let v = gi(f);
            if leniente == 0 && v >= gi(G_STAFF_COUNT) { // the squad without the extra rows
                if v > gi(G_STAFF_COUNT) + 0x52d {
                    format(0xde1f64, 0x9b6218, i, v);
                }
                wr(f, 0);
            } else {
                wr(f, if v < 0 { 0 } else { target(G_STAFF_TABLE, v, 0x6e) });
            }
        }
        // scouts, coaches, physios, directors, coach, chairman, assistant, the 20 selected
        for &(d, k, texto) in &[(0x1b3usize, 7usize, 0x9b61f4usize), (0x19f, 5, 0x9b61d0), (0x1cf, 3, 0x9b61ac), (0xc3, 3, 0x9b6188),
            (0xcf, 1, 0x9b6164), (0xbf, 1, 0x9b6140), (0xd3, 1, 0x9b6110), (0x1e0, 20, 0x9b60dc)]
        {
            for j in 0..k {
                let f = e + d + 4 * j;
                if let Some(v) = c(f, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e) {
                    format(0xde1f64, texto, i, v);
                    wr(f, 0);
                }
            }
        }
    });
    rows(G_NATION_COUNT, G_NATION_TABLE, 0x122, &|e, _| {
        for &(d, base, size) in &[(0x71usize, G_CONTINENT_TABLE, 0xc6usize), (0x7a, G_CITY_TABLE, 0x38), (0x80, G_STADIUM_TABLE, 0x4e),
            (0x90, G_COLOUR_TABLE, 0x3a), (0x98, G_COLOUR_TABLE, 0x3a), (0xa0, G_COLOUR_TABLE, 0x3a), (0x94, G_COLOUR_TABLE, 0x3a),
            (0x9c, G_COLOUR_TABLE, 0x3a), (0xa4, G_COLOUR_TABLE, 0x3a), (0x110, G_NATION_TABLE, 0x122), (0x114, G_NATION_TABLE, 0x122),
            (0x118, G_NATION_TABLE, 0x122)]
        {
            a(e + d, base, size);
        }
    });
    rows(G_OFFICIAL_COUNT, G_OFFICIAL_TABLE, 0x2b, &|e, _| {
        for &(d, base, size) in &[(4usize, G_FIRST_NAME_TABLE, 0x3cusize), (8, G_SECOND_NAME_TABLE, 0x3c), (0x16, G_NATION_TABLE, 0x122), (0x1a, G_CITY_TABLE, 0x38)] {
            a(e + d, base, size);
        }
    });
    rows(G_STADIUM_COUNT, G_STADIUM_TABLE, 0x4e, &|e, _| {
        if b(e + 0x38, G_CITY_COUNT, G_CITY_TABLE, 0x38).is_some() {
            wr(e + 0x38, 0);
        }
        if b(e + 0x48, G_STADIUM_COUNT, G_STADIUM_TABLE, 0x4e).is_some() {
            wr(e + 0x38, 0); // bug in the original: clears the city and +0x48 is left raw
        }
    });
    rows(0xae2370, G_PREFERENCES_TABLE, 0x34, &|e, _| {
        for d in (4..=0x18).step_by(4) {
            a(e + d, G_CLUB_TABLE, 581);
        }
        for d in (0x1c..=0x30).step_by(4) {
            a(e + d, G_STAFF_TABLE, 0x6e);
        }
    });
    rows(G_NON_PLAYER_COUNT, G_NON_PLAYER_TABLE, 0x44, &|e, _| {
        for d in (0x23..=0x3f).step_by(4) {
            if c(e + d, G_STAFF_COUNT, G_STAFF_TABLE, 0x6e).is_some() {
                wr(e + d, 0);
            }
        }
    });
    rows(G_STAFF_COUNT, G_STAFF_TABLE, 0x6e, &|e, i| {
        for &(d, base, size) in &[(4usize, G_FIRST_NAME_TABLE, 0x3cusize), (8, G_SECOND_NAME_TABLE, 0x3c), (0xc, G_COMMON_NAME_TABLE, 0x3c),
            (0x1a, G_NATION_TABLE, 0x122), (0x39, G_CLUB_TABLE, 581), (0x1e, G_NATION_TABLE, 0x122), (0x24, G_CLUB_TABLE, 581)]
        {
            a(e + d, base, size);
        }
        for &(d, ceiling, base, size, texto, linha) in PERSON_RANGE_CHECK_FIELDS.iter() {
            if let Some(v) = c(e + d, ceiling, base, size) {
                format(0xde1f64, texto, i, v);
                database_error_box(linha);
                wr(e + d, 0);
            }
        }
    });
    rows(0xae2388, G_STAFF_COMP_TABLE, 0x65, &|e, _| {
        for &(d, ceiling, base, size) in &[(0x53usize, 0xae234cusize, G_CONTINENT_TABLE, 0xc6usize), (0x57, G_NATION_COUNT, G_NATION_TABLE, 0x122),
            (0x5b, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x5f, 0xae2394, G_COLOUR_TABLE, 0x3a)]
        {
            if c(e + d, ceiling, base, size).is_some() {
                wr(e + d, 0);
            }
        }
    });
    rows(G_STAFF_HISTORY_COUNT, G_STAFF_HISTORY_TABLE, 0x11, &|e, _| {
        a(e + 4, G_STAFF_TABLE, 0x6e);
        a(e + 0xa, G_CLUB_TABLE, 581);
    });
    rows(G_CITY_COUNT, G_CITY_TABLE, 0x38, &|e, _| {
        a(e + 0x1f, G_NATION_TABLE, 0x122); // the city’s nation
    });
    rows(G_CLUB_COMP_HISTORY_COUNT, G_CLUB_COMP_HISTORY_TABLE, 0x1a, &|e, _| {
        a(e + 4, G_CLUB_COMP_TABLE, 0x6b);
        a(e + 0xa, G_CLUB_TABLE, 581);
        a(e + 0xe, G_CLUB_TABLE, 581);
        for &d in &[0x12usize, 0x16] {
            if b(e + d, G_CLUB_COUNT, G_CLUB_TABLE, 581).is_some() {
                wr(e + d, 0);
            }
        }
    });
    rows(G_CLUB_COMP_COUNT, G_CLUB_COMP_TABLE, 0x6b, &|e, _| {
        for &(d, base, size) in &[(0x5dusize, G_NATION_TABLE, 0x122usize), (0x59, G_CONTINENT_TABLE, 0xc6), (0x61, G_COLOUR_TABLE, 0x3a), (0x65, G_COLOUR_TABLE, 0x3a)] {
            a(e + d, base, size);
        }
    });
}

/// `database_sort_clubs_by_reputation(rebuild)`: called at creation, in load_game_data `load_game_data` (vector missing from
/// the save) and in the daily loop 0x5cccd0.
///
/// With `rebuild` (byte) ≠ 0, the vector [0xae2d1c] receives the ids (+0) of the [0xae235c] clubs in table order; then
/// qsort (`exe::crt_qsort`) of the [0xae235c] ids with 0x540020 (by reputation).
/// Original: `database_sort_clubs_by_reputation` (0x53db40, Database.cpp, cdecl).
///
/// [pt] { 0x53db40 } (cdecl, Database.cpp): `database_sort_clubs_by_reputation(refazer)`, na criação, no load_game_data
/// 0x82a9c0 (vetor ausente do save) e no laço diário 0x5cccd0. Com `refazer` (byte) ≠ 0, o vetor [0xae2d1c] recebe os
/// ids (+0) dos [0xae235c] clubes na ordem da tabela; depois qsort (0x945333) dos [0xae235c] ids com 0x540020 (pela
/// reputação).
pub(crate) unsafe extern "C" fn database_sort_clubs_by_reputation(redo: u32) {
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    if redo as u8 != 0 {
        let mut i = 0i32;
        while i < gi(G_CLUB_COUNT) {
            write_i32(rd(G_CLUB_REPUTATION_ORDER) + i as usize * 4, gi(rd(G_CLUB_TABLE) + i as usize * 581));
            i += 1;
        }
    }
    exe::crt_qsort(rd(G_CLUB_REPUTATION_ORDER), gi(G_CLUB_COUNT) as u32, 4, 0x540020); // all clubs by reputation
}

/// Frees a database table with GlobalFree when the database uses the shared block (G_DATABASE_SHARED_ALLOC_MODE), else
/// with CRT free.
///
/// [pt] Solta uma tabela do banco como 0x527ee0/0x527f20: `GlobalFree` (IAT 0x9670f4) com o banco no bloco
/// compartilhado ([0xae2340] ≠ 0), senão `free` 0x9452ca.
pub(crate) unsafe fn free_database_table(table: usize) {
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
        let global_free: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(0x9670f4));
        global_free(table);
    } else {
        exe::crt_free(table);
    }
}

/// `database_free_staff_history_table()`: with the people history table ([0xae23cc], 17 bytes) present, frees it
/// (0x9670f4, GlobalFree in the shared database, or `exe::crt_free`, free, by the database mode) and zeroes the pointer and
/// the count [0xae2374].
///
/// TapeCall through PlayerStats::PlayerStats (0x7c0ccd): the history moves to `player stats history.tmp`. No stack
/// frame.
/// Original: `database_free_staff_history_table` (0x527ee0, Database.cpp, cdecl).
///
/// [pt] { 0x527ee0 } (cdecl, Database.cpp): `database_free_staff_history_table()`: com a tabela do histórico de
/// pessoas ([0xae23cc], 17 bytes) presente, solta (0x9670f4 ou 0x9452ca, pelo modo do banco) e zera o ponteiro
/// e a contagem [0xae2374]. TapeCall por PlayerStats::PlayerStats (0x7c0ccd): o histórico passa a viver no
/// `player stats history.tmp`. Sem quadro de pilha.
pub(crate) unsafe extern "C" fn database_free_staff_history_table() {
    let table = read_usize(G_STAFF_HISTORY_TABLE);
    if table == 0 {
        return;
    }
    crate::comparison::branch("database free people history");
    free_database_table(table);
    write_usize(G_STAFF_HISTORY_TABLE, 0);
    write_u32(G_STAFF_HISTORY_COUNT, 0);
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// simplification: original only; the proof is the .qst (the table comes from the save) and the copy’s branch in Rust mode.

/// the same for the award (staff competition) history
/// Original: `database_free_staff_comp_history_table` (0x527f20, Database.cpp, cdecl).
///
/// [pt] { 0x527f20 } (cdecl, Database.cpp): `database_free_staff_comp_history_table()`: o mesmo para o histórico
/// de prêmios ([0xae23e4], 58 bytes; contagem [0xae238c]). TapeCall por AWARD_MANAGER::AWARD_MANAGER (0x413360).
pub(crate) unsafe extern "C" fn database_free_staff_comp_history_table() {
    let table = read_usize(G_STAFF_COMP_HISTORY_TABLE);
    if table == 0 {
        return;
    }
    crate::comparison::branch("database free award history");
    free_database_table(table);
    write_usize(G_STAFF_COMP_HISTORY_TABLE, 0);
    write_u32(G_STAFF_COMP_HISTORY_COUNT, 0);
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// simplification: original only; the proof is the .qst (the table comes from the save) and the copy’s branch in Rust mode.

/// `database_grow_comp_history_table(n)`: `realloc` `exe::crt_realloc` of the competition history ([0xae23d4], 26 bytes) to
/// [0xae237c] + `n` rows; out of memory returns 0 and nothing changes.
///
/// Otherwise adds `n` to the count, stores the block, rebuilds the pointer to the national teams part [0xae23dc] (from
/// row [0xae2384]) and clears the `n` new rows (+0 = the row's index, the rest zero); returns 1. TapeCall through
/// comp_util_add_history_record (0x4afd2c), at the end of a competition. No stack frame.
/// Original: `database_grow_comp_history_table` (0x527f60, Database.cpp, cdecl).
///
/// [pt] { 0x527f60 } (cdecl, Database.cpp): `database_grow_comp_history_table(n)`: `realloc` 0x945667 do histórico de
/// competições ([0xae23d4], 26 bytes) para [0xae237c] + `n` linhas; sem memória devolve 0 e nada muda. Senão soma
/// `n` à contagem, guarda o bloco, refaz o ponteiro da parte das seleções [0xae23dc] (a partir da linha [0xae2384])
/// e limpa as `n` linhas novas (+0 = o índice da linha, o resto zero); devolve 1. TapeCall por
/// comp_util_add_history_record (0x4afd2c), no fim de competição. Sem quadro de pilha.
pub(crate) unsafe extern "C" fn database_grow_comp_history_table(n: i32) -> u32 {
    crate::comparison::branch("database grow competition history");
    let total = (*(G_CLUB_COMP_HISTORY_COUNT as *const i32)).wrapping_add(n);
    let new = exe::crt_realloc(*(G_CLUB_COMP_HISTORY_TABLE as *const usize), (total as u32).wrapping_mul(26));
    if new == 0 {
        return 0;
    }
    *(G_CLUB_COMP_HISTORY_COUNT as *mut i32) = total;
    *(G_CLUB_COMP_HISTORY_TABLE as *mut usize) = new;
    let national_teams = total.wrapping_sub(*(G_NATION_COMP_HISTORY_COUNT as *const i32));
    *(G_NATION_COMP_HISTORY_TABLE as *mut usize) = new.wrapping_add((national_teams as usize).wrapping_mul(26));
    for k in 1..=n {
        let i = total.wrapping_sub(k);
        let row = new.wrapping_add((i as usize).wrapping_mul(26)) as *mut u8;
        core::ptr::write_bytes(row, 0, 26);
        core::ptr::write_unaligned(row as *mut i32, i); // the new row stores its own index
    }
    1
}

/// Comparer note: the `realloc` cannot be undone (growing twice would leave the table with 2n extra rows).
///
/// [pt] Comparador: o `realloc` não se desfaz (crescer duas vezes deixaria a tabela com 2n linhas a mais).
// simplification: original only; the proof is the .qst (the table goes into the save) and the copy’s branch in Rust mode.

/// qsort comparison of the club id vector (`database_sort_clubs_by_reputation`) by reputation, highest first.
///
/// By the reputation (+0x80, signed word) of the club ([0xae23b4] + id × 581; id −1 or lower = reputation 0): `a`'s
/// lower than `b`'s: 1; higher: −1; equal: 0.
/// Original: `qsort comparator (club ids by reputation)` (0x540020, Database.cpp, cdecl).
///
/// [pt] { 0x540020 } (cdecl, Database.cpp): comparação do qsort do vetor de ids de clube (0x53db40): pela reputação
/// (+0x80, palavra com sinal) do clube ([0xae23b4] + id × 581; id −1 ou menor = reputação 0), a maior primeiro:
/// a de `a` menor que a de `b`: 1; maior: −1; iguais: 0.
pub(crate) unsafe extern "C" fn qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let clubs = read_usize(G_CLUB_TABLE);
    let injury_word_0c = |p: usize| {
        let id = read_i32(p);
        if id > -1 { read_i16(clubs.wrapping_add((id as usize).wrapping_mul(581)).wrapping_add(0x80)) } else { 0 }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // the lowest reputation afterwards
    } else if ra > rb {
        -1
    } else {
        0
    }
}

/// qsort comparison of squads (the trim of the joint A+B squad at `database_trim_senior_and_b_team_squads` and the MLS all-star selections at
/// `exe::usamlsallstars_select_squads`): person pointers.
///
/// A null person goes last (a null and b not: 1; a not and b null: −1; both: 0); then by the player's word +0xb (person
/// +0x61; no player, 0), highest first (b − a).
/// Original: `qsort comparator (squad by player +0xb)` (0x5402f0, Database.cpp, cdecl).
///
/// [pt] { 0x5402f0 } (cdecl, Database.cpp): comparação do qsort dos elencos (corte do conjunto A+B em 0x541e60 e as
/// seleções da MLS em 0x90a980): ponteiros de pessoa; nula vai para o fim (a nula e b não: 1; a não e b nula: −1;
/// as duas: 0); depois pela palavra +0xb do jogador (pessoa +0x61; sem jogador, 0), a maior primeiro (b − a).
pub(crate) unsafe extern "C" fn qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let pa = read_usize(a);
    let pb = read_usize(b);
    if pa == 0 {
        return (pb != 0) as i32;
    }
    if pb == 0 {
        return -1;
    }
    let ability = |p: Staff| {
        let j = p.player();
        if !j.is_null() { j.current_reputation() } else { 0 }
    };
    ability(Staff(pb)) as i32 - ability(Staff(pa)) as i32 // the largest first
}

/// qsort comparison of the squad in the team strength (club_squad_strength_band `club_squad_strength_band`): person pointers.
///
/// No person or no player record (+0x61) in `a`: 1 (even if `b` is the same); in `b`: −1; then by current ability
/// (player +5, signed word) and, when tied, by the word +0xb, highest first (b − a).
/// Original: `compare_staff_by_player_ability_desc` (0x549c30, Database.cpp, cdecl).
///
/// [pt] { 0x549c30 } (cdecl, Database.cpp): comparação do qsort do elenco na força do time (club_squad_strength_band
/// 0x549a30): ponteiros de pessoa; sem pessoa ou sem jogador (+0x61) em `a`: 1 (mesmo com `b` igual); em `b`:
/// −1; depois pela habilidade atual (jogador +5, palavra com sinal) e, empatada, pela palavra +0xb, a maior
/// primeiro (b − a).
pub(crate) unsafe extern "C" fn compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let player = |p: usize| {
        let s = Staff(read_usize(p));
        if s.is_null() { 0 } else { s.player().addr() }
    };
    let ja = PlayerSheet(player(a));
    if ja.is_null() {
        return 1;
    }
    let jb = PlayerSheet(player(b));
    if jb.is_null() {
        return -1;
    }
    if (ja.current_ability() as i32) != (jb.current_ability() as i32) {
        (jb.current_ability() as i32) - (ja.current_ability() as i32) // the highest current ability first
    } else {
        (jb.current_reputation() as i32) - (ja.current_reputation() as i32)
    }
}

/// Calls TRAINING_MANAGER remove_or_restore_training_progress on G_TRAINING_MANAGER for the person (devolver 0 removes,
/// 1 restores); nothing without a manager.
///
/// [pt] TRAINING_MANAGER::remove_or_restore_training_progress (0x8b3640, thiscall com o gerente [0xde1f44]): tira
/// (`devolver` = 0) ou devolve (1) o progresso do treino dos atributos da `pessoa`. Sem gerente, nada.
pub(crate) unsafe fn remove_or_restore_training(person: Staff, devolver: u32) {
    let manager = read_usize(G_TRAINING_MANAGER);
    if manager != 0 {
        exe::training_manager_remove_or_restore_training_progress(manager, person, devolver);
    }
}

/// Subtracts the mean of the player's signed bytes +0x2a and +0x3a from both (common tail of `staff_normalise_player_attributes`/`staff_normalise_player_physical_attributes`).
///
/// [pt] O fim comum de 0x538bc0 e 0x538ec0 com o jogador +0xf alto: tira de +0x2a e de +0x3a a média dos dois
/// (bytes com sinal, divisão truncada).
pub(crate) unsafe fn subtract_mean_2a_3a(player: PlayerSheet) {
    let (a, b) = ((player.addr() + player_sheet::HANDLING) as *mut u8, (player.addr() + player_sheet::REFLEXES) as *mut u8);
    let average = ((*a as i8 as i32 + *b as i8 as i32) / 2) as u8;
    *a = (*a).wrapping_sub(average);
    *b = (*b).wrapping_sub(average);
}

/// The shared rebalancing of `staff_normalise_player_attributes` and `staff_normalise_player_physical_attributes` over the player's attribute vector (`database_player_to_attribute_vector`: signed word + weight
/// per cell), on cells from `inicio` (end of the sides + 2) up to before `fim` (end of the weighted ones), excluding
/// those in `pular`.
///
/// Two passes: (1) limit 30, +5 per round, up to 12 cells with |value| ≤ limit (marked); the one with the highest value
/// outside the limit is also marked; (2) adds `alvo` − average to the marked ones (clamped to −125..125); in the 1st
/// pass, with the player's +0xf < 15, the unmarked positive ones go into the excess (cells 0x11 and 0x12 via
/// `excesso`), the rest are marked; with cell 0 < 15, those of weight 1 become −120; (3) with more than one in the
/// excess and sum > 85, takes (sum − 85) / count from the unmarked ones (minimum 0).
///
/// The original's frame is not read uninitialised: the cells of the range all come from `database_player_to_attribute_vector`.
///
/// [pt] O reequilíbrio comum de 0x538bc0 e 0x538ec0 sobre o vetor de atributos do jogador (0x53a610: palavra com
/// sinal + peso por célula), nas células de `inicio` (fim dos lados + 2) até antes de `fim` (fim dos
/// ponderados), sem as de `pular`; duas passadas: (1) limite 30, +5 por volta, até 12 células com |valor| ≤
/// limite (marcadas); a de maior valor fora do limite também é marcada; (2) soma `alvo` − média às marcadas
/// (presas em −125..125); na 1ª passada, com o jogador +0xf < 15, as não marcadas positivas entram no
/// excesso (as células 0x11 e 0x12 por `excesso`), as demais são marcadas; com a célula 0 < 15, as de peso 1
/// viram −120; (3) com mais de uma no excesso e soma > 85, tira (soma − 85) / quantas das não marcadas
/// (mínimo 0). O quadro da original não é lido sem valor: as células do intervalo vêm todas de 0x53a610.
pub(crate) unsafe fn rebalance_attr_vector(v: *mut u8, start: i8, end: i8, player: PlayerSheet, target: i32, skip: &[i8],
                                        excesso: impl Fn(i16) -> i32) {
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let weight = |i: i8| *v.add(3 * i as usize + 2);
    let away = |c: i8| skip.contains(&c);
    let jovem = player.goalkeeper() < 0xf;
    let mut mark = [0u32; 64];
    for step_towards in 0..2 {
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limit = 0x1ei32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if start < end {
                let mut c = start;
                while c < end {
                    if !away(c) {
                        let x = w(c);
                        if (x as i32).abs() <= limit as i16 as i32 {
                            total = total.wrapping_add(x as i32);
                            n = n.wrapping_add(1);
                            mark[c as usize] = 1;
                        } else {
                            mark[c as usize] = 0;
                            if melhor == -1 || x > w(melhor) {
                                melhor = c;
                            }
                        }
                    }
                    c += 1;
                }
                if n >= 12 {
                    break;
                }
            }
            // simplification: with inicio >= fim (or fewer than 12 cells) the original runs forever; the copy too
            limit += 5;
        }
        if melhor > -1 {
            mark[melhor as usize] = 1;
        }
        let delta = target.wrapping_sub(total / n as i32) as i16;
        let mut c = start;
        while c < end {
            if !away(c) && c != melhor {
                if mark[c as usize] == 1 {
                    pw(c, w(c).wrapping_add(delta).clamp(-0x7d, 0x7d));
                } else if step_towards == 0 && w(c) > 0 && jovem {
                    quantas += 1;
                    soma = soma.wrapping_add(if c == 0x11 || c == 0x12 { excesso(w(c)) } else { w(c) as i32 });
                } else {
                    mark[c as usize] = 1;
                }
                if w(0) < 0xf && weight(c) == 1 {
                    pw(c, -0x78);
                }
            }
            c += 1;
        }
        if quantas > 1 && soma > 0x55 {
            let remove = ((soma - 0x55) / quantas) as i16;
            let mut c = start;
            while c < end {
                if !away(c) && mark[c as usize] != 1 {
                    let x = w(c).wrapping_sub(remove);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
            }
        }
    }
}

/// Fills the player's attribute vector (`database_player_to_attribute_vector`) and returns the group ends (sides end, weighted end).
///
/// [pt] O vetor de atributos do `jogador` (0x53a610) com os fins de grupo (lados, posições, ponderados).
pub(crate) unsafe fn player_attr_vector(player: PlayerSheet, person: Staff, v: *mut u8) -> (u8, u8) {
    let (mut lados, mut posicoes, mut ponderados) = (0u8, 0u8, 0u8);
    exe::database_player_to_attribute_vector(player, person, v as usize, &mut lados, &mut posicoes, &mut ponderados);
    (lados, ponderados)
}

/// `staff_normalise_player_physical_attributes(person)`: lowers the physical attributes above the current ability and
/// rebalances the attribute vector.
///
/// With the player record (person +0x61): removes the training progress (`remove_or_restore_training_progress`); cut = (ability +5 / 2 −
/// ftol(weighted sum of the physical attributes × [0x969528]) + 0x37) / 2, the sum with the .rdata constants (+0x36,
/// +0x2f, +0x2e, +0x1b, +0x3f, +0x40, the lower of +0x3b/+0x30, plus +0x44); cut < −14: adds max(cut / 15, −2) to
/// +0x1b, +0x36, +0x40 and +0x44 (at least 1); target = cut − 20 − min((+0x36 × +0x1b) / 2 − ability, 25) when it
/// exceeds the ability, clamped to −40..0; vector (0xff81/0x81) through `database_player_to_attribute_vector`, `rebalance_attr_vector` without cells
/// 0x18 and 0x1e (0x11/0x12 count double), back through `database_vector_to_player`; player +0xf = 20: removes the averages of +0x2a/+0x3a
/// and the third of +0x1e, +0x24, +0x39 (pair and trio averages); restores the training progress.
/// Original: `staff_normalise_player_physical_attributes` (0x538ec0, Database.cpp, cdecl).
///
/// [pt] { 0x538ec0 } (cdecl, Database.cpp): `staff_normalise_player_physical_attributes(pessoa)`: com o jogador
/// (pessoa +0x61), tira o progresso do treino (0x8b3640); corte = (habilidade +5 / 2 − ftol(soma ponderada dos
/// físicos × [0x969528]) + 0x37) / 2, a soma com as constantes do .rdata (+0x36, +0x2f, +0x2e, +0x1b, +0x3f,
/// +0x40, o menor de +0x3b/+0x30, mais +0x44); corte < −14: soma max(corte / 15, −2) a +0x1b, +0x36, +0x40 e
/// +0x44 (no mínimo 1); alvo = corte − 20 − min((+0x36 × +0x1b) / 2 − habilidade, 25) quando passa da
/// habilidade, preso em −40..0; vetor (0xff81/0x81) pelo 0x53a610, `rebalance_attr_vector` sem as
/// células 0x18 e 0x1e (0x11/0x12 contam em dobro), volta pelo 0x53aa10; jogador +0xf = 20: tira as médias
/// de +0x2a/+0x3a e o terço de +0x1e, +0x24, +0x39; devolve o progresso do treino.
pub(crate) unsafe extern "C" fn staff_normalise_player_physical_attributes(person: usize) {
    let person = Staff(person);
    if person.is_null() {
        return;
    }
    let player = person.player();
    if player.is_null() {
        return;
    }
    remove_or_restore_training(person, 0);
    let b = |d: usize| read_i8(player.addr() + d) as i32;
    let f = |x: i32, k: usize| ftol(x as f64 * read_f64(k));
    let smaller = b(player_sheet::RIGHT_FOOT).min(b(player_sheet::LEFT_FOOT));
    let t = f(b(player_sheet::PLAYER_PACE), 0x968940) - f(b(player_sheet::LEADERSHIP), 0x969548) - f(b(player_sheet::JUMPING), 0x969540) - f(b(player_sheet::ACCELERATION), 0x969538)
        + (f(b(player_sheet::TEAMWORK), 0x9689b0) - f(b(player_sheet::TECHNIQUE), 0x969548) - f(smaller, 0x969530)) + b(player_sheet::WORK_RATE);
    let ability = player.current_ability();
    let cut = (ability as i32 / 2 - f(t, 0x969528) + 0x37) / 2;
    if cut < -14 {
        let d = (cut / 15).max(-2);
        for off in [0x1b, 0x36, 0x40, 0x44] {
            let x = d + b(off);
            write_u8(player.addr() + off, if x < 1 { 1 } else { x as u8 });
        }
    }
    let mut s = cut - 0x14;
    let produto = ((b(player_sheet::PLAYER_PACE) * b(player_sheet::ACCELERATION)) / 2) as i16;
    if produto > ability {
        s -= (produto as i32 - ability as i32).min(0x19);
    }
    let target = if s < -40 { -40 } else if s > 0 { 0 } else { s }; // physical target
    let mut array = [0u8; 63 * 3];
    for c in array.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = array.as_mut_ptr();
    let (lados, ponderados) = player_attr_vector(player, person, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, player, target, &[0x1e, 0x18],
        |x| 2 * x as i32);
    (exe::database_attribute_vector_to_player(player, person, v as usize) as u32);
    if player.goalkeeper() == 0x14 {
        subtract_mean_2a_3a(player);
        let third = ((b(player_sheet::ANTICIPATION) + b(player_sheet::DECISIONS) + b(player_sheet::POSITIONING)) / 3) as u8;
        for off in [0x1e, 0x39, 0x24] {
            write_u8(player.addr() + off, read_u8(player.addr() + off).wrapping_sub(third));
        }
    }
    remove_or_restore_training(person, 1);
}

/// A second name (60-byte row in [0xae23f4]) by weight.
///
/// A nation with more than 10 second names (`nation_name_index` +0x1c): 3 draws in its range (from +4), the one with
/// the highest weight is kept (+0x3b, signed; starts at −1, and row 0 if none passes). No nation or up to 10: a draw
/// over the whole table ([0xae239c], at most 0x7fff). Only the CRT rand(). TapeCall through
/// OfficialsManager::add_random_officials (0x79c6b4), when creating the referees of a new game.
/// Original: `database_pick_second_name_by_weight` (0x53a0e0, Database.cpp, cdecl).
///
/// [pt] { 0x53a0e0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) pelo peso. Nação com
/// mais de 10 segundos nomes (`nation_name_index` +0x1c): 3 sorteios na faixa dela (a partir de +4),
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com até
/// 10: sorteio na tabela inteira ([0xae239c], no máximo 0x7fff). Só o rand() da CRT. TapeCall por
/// OfficialsManager::add_random_officials (0x79c6b4), na criação dos árbitros do jogo novo.
pub(crate) unsafe extern "C" fn database_pick_second_name_by_weight(nation: usize) -> usize {
    let row = |i: i32| read_usize(G_SECOND_NAME_TABLE).wrapping_add((i as usize).wrapping_mul(60));
    if nation != 0 {
        let e = nation_name_index(nation);
        let how_many = read_i32(e + nation_name_index::SECOND_NAME_COUNT);
        if how_many > 0xa {
            let start = read_i32(e + nation_name_index::SECOND_NAME_START);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 weighted draws
                let i = scaled_crt_rand(how_many).wrapping_add(start);
                let weight = read_i8(row(i) + 0x3b);
                if weight > melhor {
                    melhor = weight;
                    escolhido = i;
                }
            }
            return row(escolhido); // the second name with the highest weight
        }
    }
    row(scaled_crt_rand(name_count_capped(0xae239c)))
}

/// `staff_normalise_player_attributes(person)`: rebalances the player's attribute vector with target −20.
///
/// With the player record (person +0x61): removes the training progress (`remove_or_restore_training_progress`), builds the vector through `database_player_to_attribute_vector`
/// (without pre-filling: the cells used all come from it), `rebalance_attr_vector` with target −20 in all cells of the
/// range (12 cells within the limit, excess above 85 spread; 0x11/0x12 count ftol(value × [0x968f98])), back through
/// `database_vector_to_player`; player +0xf ≥ 15: removes the averages of +0x2a/+0x3a; restores the training progress.
///
/// Runs at creation (after `staff_normalise_player_physical_attributes`), at load and in training.
/// Original: `staff_normalise_player_attributes` (0x538bc0, Database.cpp, cdecl).
///
/// [pt] { 0x538bc0 } (cdecl, Database.cpp): `staff_normalise_player_attributes(pessoa)`: com o jogador (pessoa
/// +0x61), tira o progresso do treino (0x8b3640), monta o vetor pelo 0x53a610 (sem pré-encher: as células
/// usadas vêm todas dele), `rebalance_attr_vector` com alvo −20 em todas as células do intervalo
/// (0x11/0x12 contam ftol(valor × [0x968f98])), volta pelo 0x53aa10; jogador +0xf ≥ 15: tira as médias de
/// +0x2a/+0x3a; devolve o progresso do treino. Roda na criação (depois de 0x538ec0) e na carga e no treino.
pub(crate) unsafe extern "C" fn staff_normalise_player_attributes(person: usize) {
    let person = Staff(person);
    if person.is_null() {
        return;
    }
    let player = person.player();
    if player.is_null() {
        return;
    }
    remove_or_restore_training(person, 0);
    let mut array = [0u8; 63 * 3];
    let v = array.as_mut_ptr();
    let (lados, ponderados) = player_attr_vector(player, person, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, player, -20, &[],
        |x| ftol(x as f64 * read_f64(rdata::F64_1_75)));
    (exe::database_attribute_vector_to_player(player, person, v as usize) as u32);
    if player.goalkeeper() >= 0xf {
        subtract_mean_2a_3a(player);
    }
    remove_or_restore_training(person, 1);
}

/// Competition record (107 bytes) in club_comp.dat / nation_comp.dat version 1 (104 in the file): `(deslocamento,
/// tamanho, quantos)` in file order.
///
/// Gaps at +0x37 and +0x52; size 0 = the reputation in one byte, u16 at +0x69.
///
/// [pt] Competição (107 bytes) no club_comp.dat / nation_comp.dat versão 1 (104 no arquivo): `(deslocamento,
/// tamanho, quantos)` na ordem do arquivo; buracos em +0x37 e +0x52; tamanho 0 = a reputação num byte, u16 em +0x69.
pub(crate) const COMPETITION_V1_LAYOUT: [(usize, u32, usize); 7] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 1), (0x57, 1, 2), (0x59, 4, 4), (0x69, 0, 1)];

/// Comparer: with `fread` (use_zip = 0), the original routine reads, the file is rewound (`fseek` `exe::crt_fseek` to the
/// position from `ftell` `exe::crt_ftell`) and the Rust routine reads again; it compares the records, the answer and where the
/// file stopped.
///
/// [pt] Comparador: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à posição do `ftell`
/// 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo parou.
// simplification: with zread (the container) original only; the save is version 2 and does not get here.

/// the nation's name, or its English one from eng.lng (109-byte rows, by id) when another language is loaded
/// Original: `nation_get_english_name` (0x5453f0, Database.cpp, cdecl).
///
/// [pt] { 0x5453f0 } (cdecl, Database.cpp): `nation_get_english_name(nação)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de +0x2c, pelo id
/// (sem conferir a quantidade).
pub(crate) unsafe extern "C" fn nation_get_english_name(nation: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return nation.wrapping_add(4); // language 0 or 1 (nation)
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: the first use allocates (new 0x40) and loads eng.lng (0x544fc0) under the SEH frame: the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_NAME)(nation);
    }
    let id = read_i32(nation);
    read_usize(t + english_name_tables::NATION_ROWS).wrapping_add((id as usize).wrapping_mul(109))
}

/// `club_get_english_short_name(club)`: the club's short name, or its English one from eng.lng when another language is
/// loaded.
///
/// Language ([0x9b5984]) 0 or 1: its own short name (+0x38). Otherwise the eng.lng table ([0xae2408], 79-byte rows from
/// +0x20, the short name at +0x34 of the row): through the id map [0xae23fc] if present, otherwise by id if < the count
/// (+0), otherwise 0.
/// Original: `club_get_english_short_name` (0x545770, Database.cpp, cdecl).
///
/// [pt] { 0x545770 } (cdecl, Database.cpp): `club_get_english_short_name(clube)`: idioma ([0x9b5984]) 0 ou 1: o
/// próprio nome curto (+0x38). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20, o
/// nome curto em +0x34 da linha): pelo mapa de ids [0xae23fc] se houver, senão pelo id se < quantos (+0),
/// senão 0.
pub(crate) unsafe extern "C" fn club_get_english_short_name(club: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return club.wrapping_add(0x38);
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: the first use allocates and loads eng.lng under the SEH frame: the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_SHORT_NAME)(club);
    }
    let rows = read_usize(t + english_name_tables::CLUB_ROWS);
    let id = read_i32(club);
    let map = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    let i = if map != 0 {
        read_i32(map.wrapping_add((id as usize).wrapping_mul(4)))
    } else if id < read_i32(t) {
        id
    } else {
        return 0;
    };
    rows.wrapping_add((i as usize).wrapping_mul(79)).wrapping_add(0x34) // short name in the row
}

/// Self-test (`CM_AUTOTESTE=1`, comparison mode) of three routines that neither the career nor the save goes through
/// (the database is version 2; the test game is in English).
///
/// (1) The competitions converter: 64 cases with fixed seeds, from 1 to 6 records, a temporary file
/// (`autoteste_banco_v1c.tmp` in the game folder, deleted at the end) with random bytes (1 in 4 all zeros), whole (1 in
/// 3) or cut at a random point, with the buffer starting random.
///
/// (2) The English names: 256 cases with language 0, 1, 2, 3 or 13 and a fake `eng.lng` table (never null: the first
/// use would load the real file), with and without the id map, ids from −2 to 11 (in the map, from 0 to 7); the globals
/// are restored at the end.
///
/// [pt] Autoteste (CM_AUTOTESTE=1, modo comparação): nem a carreira nem o save passam por estas três (a base é
/// versão 2; o jogo dos testes está em inglês). (1) O conversor das competições: 64 casos de sementes fixas,
/// de 1 a 6 registros, arquivo temporário (`autoteste_banco_v1c.tmp` na pasta do jogo, apagado no fim) com
/// bytes sorteados (1 em 4 só zeros), inteiro (1 em 3) ou cortado num ponto sorteado, buffer começando
/// sorteado. (2) Os nomes em inglês: 256 casos com o idioma 0, 1, 2, 3 ou 13 e uma tabela do eng.lng falsa
/// (nunca nula: o primeiro uso carregaria o arquivo de verdade), com e sem o mapa de ids, ids de −2 a 11 (no
/// mapa, de 0 a 7); as globais voltam no fim.
pub unsafe fn autoteste_dbj_2() {
    const ARQ: &str = "autoteste_banco_v1c.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let (t0, d0) = SCOREBOARD.counts();
    let mut s = TestRng(0x5631_4442_4a32);
    let v1: usize = COMPETITION_V1_LAYOUT.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum();
    for caso in 0..64u64 {
        let n = 1 + (s.next_u64() % 6) as usize;
        let total = n * v1;
        let corte = if caso % 3 == 0 { total } else { (s.next_u64() % (total as u64 + 1)) as usize };
        let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.next_u64() as u8 }).collect();
        if std::fs::write(ARQ, &bytes).is_err() {
            continue;
        }
        let f = fopen(b"autoteste_banco_v1c.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
        if f == 0 {
            continue;
        }
        let mut buf: Vec<u8> = (0..n * 107).map(|_| s.next_u64() as u8).collect();
        compare_db_files_read_comps_v1(f, 0, buf.as_mut_ptr() as usize, n as u32);
        fclose(f);
    }
    let _ = std::fs::remove_file(ARQ);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES, G_CLUB_ENGLISH_NAME_INDEX_TABLE];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // the eng.lng table (new 0x40): +0 how many clubs, +0x20 clubs, +0x2c nations
    let mut mapa = [0i32; 8];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.next_u64();
        *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = [0u32, 1, 2, 3, 13][(z % 5) as usize];
        tabela[0] = ((z >> 4) % 10) as usize;
        tabela[8] = 0x1000_0000 + ((z >> 8) as usize & 0xffff0);
        tabela[11] = 0x2000_0000 + ((z >> 24) as usize & 0xffff0);
        *(G_ENGLISH_NAME_TABLES as *mut usize) = tabela.as_mut_ptr() as usize;
        for (k, m) in mapa.iter_mut().enumerate() {
            *m = ((z >> (40 + k)) % 9) as i32 - 1;
        }
        let com_mapa = (z >> 50) % 2 == 0;
        *(G_CLUB_ENGLISH_NAME_INDEX_TABLE as *mut usize) = if com_mapa { mapa.as_ptr() as usize } else { 0 };
        let id = if com_mapa { ((z >> 52) % 8) as i32 } else { ((z >> 52) % 14) as i32 - 2 };
        *(registro.as_mut_ptr() as *mut i32) = id;
        compare_nation_get_english_name(registro.as_ptr() as usize);
        compare_club_get_english_short_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = SCOREBOARD.counts();
    linha(&format!("database v1 and English names: SELFTEST {} cases, {} divergences", t1 - t0, d1 - d0));
}

/// `database_trim_senior_and_b_team_squads()`: flags every club that has a partner team and trims each joint senior+B
/// squad to 48. Called once at creation (create_game 0x8282f1).
///
/// In each club, the flag +0x244 = has a partner (`exe::club_get_linked_b_team` with 0) and the partner has a nation (+0x53). In the A team
/// (`is_a`): the joint A+B squad (`database_collect_club_squad` with double: 100 pointers) in reputation order (qsort `exe::crt_qsort` with
/// 0x5402f0); from the back to the front, while the joint squad has more than 48, removes the person from her club
/// (`database_remove_staff_from_club` with the club +0x39, no date, detaching the person, not as a player), skipping the goalkeepers (player
/// +0x61 with +0xf ≥ 15, signed byte) while 3 or fewer remain.
///
/// Signed 8-bit counters, like the original's. No draw; the local `is_a` is only read when `exe::club_get_linked_b_team` returns a partner
/// (and then it was written).
/// Original: `database_trim_senior_and_b_team_squads` (0x541e60, Database.cpp, cdecl).
///
/// [pt] { 0x541e60 } (cdecl, Database.cpp): `database_trim_senior_and_b_team_squads()`, uma vez na criação (create_game
/// 0x8282f1). Em cada clube, a marca +0x244 = tem parceiro (0x540a50 com 0) e o parceiro tem nação (+0x53). No time
/// A (`eh_a`): o elenco conjunto A+B (0x541f90 com dobro: 100 ponteiros) em ordem de reputação (qsort 0x945333 com
/// 0x5402f0); de trás para a frente, enquanto o conjunto tiver mais de 48, tira a pessoa do clube dela (0x53c190 com
/// o clube +0x39, sem data, desligando a pessoa, não como jogador), pulando os goleiros (jogador +0x61 com +0xf ≥ 15,
/// byte com sinal) enquanto restarem 3 ou menos. Contadores de 8 bits com sinal, como os da original. Sem sorteio;
/// a local `eh_a` só é lida quando 0x540a50 devolve parceiro (e então ela foi gravada).
pub(crate) unsafe extern "C" fn database_trim_senior_and_b_team_squads() {
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let rd = |a: usize| read_usize(a);
    let goalkeeper = |q: Staff| {
        let j = PlayerSheet(read_usize(q.addr() + staff::PLAYER));
        !j.is_null() && j.goalkeeper() >= 0xf
    };
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let mut eh_a = 0u32;
        let p = Club(partner(club, &mut eh_a, 0));
        if p.is_null() || p.nation().addr() == 0 {
            write_u8(club + club::HAS_LINKED_CLUB, 0);
        } else {
            write_u8(club + club::HAS_LINKED_CLUB, 1);
            if eh_a != 0 {
                let mut v = [0usize; 100];
                exe::database_collect_club_squad(club, v.as_mut_ptr(), 1);
                exe::crt_qsort(v.as_mut_ptr() as usize, 100, 4, 0x5402f0);
                let (mut total, mut goleiros) = (0i8, 0i8);
                for &q in v.iter() {
                    if q != 0 {
                        total += 1;
                        if goalkeeper(Staff(q)) {
                            goleiros += 1;
                        }
                    }
                }
                let mut k = 99i8;
                while k >= 0 && total > 48 { // the combined squad at 48
                    let q = v[k as usize];
                    if q != 0 {
                        let g = goalkeeper(Staff(q));
                        if !(g && goleiros < 3) {
                            if g {
                                goleiros -= 1;
                            }
                            exe::database_remove_staff_from_club(read_usize(q + staff::CLUB_JOB), q, 0, 1, 0);
                            total -= 1;
                        }
                    }
                    k -= 1;
                }
            }
        }
        i += 1;
        club += club::SIZE;
    }
}

/// `database_merge_b_team_staff()`: for every senior team with a B team, trims the B squad and moves the B backroom
/// staff to the senior club. Called once at creation (create_game 0x8283d9).
///
/// For each A team with a B partner (`exe::club_get_linked_b_team` with 1, which requires the +0x244 flag from `database_trim_senior_and_b_team_squads`): counts the valid
/// people (non-null pointer and id < [0xae2364]) of the A squad (+0xd7, 50), sorts the B squad (qsort with 0x540130)
/// and removes from B those beyond 50 in the joint total.
///
/// Then, in slots +0x19f (5, job 8), +0x1b3 (7, job 9) and +0x1cf (3, job 10): each valid person of B leaves it
/// (`database_remove_staff_from_club`) and, while A has room, joins A (`database_add_staff_to_club` with the game date 0xae2c90, writing into the person). Manager
/// (+0xcf): with both, B's just leaves; only B's: leaves and joins A with his job (+0x3d, read after leaving; the
/// original pushes the byte with garbage above it, and `database_add_staff_to_club` uses only the byte), no date. Assistant (+0xd3): with
/// both, B's leaves. No draw.
/// Original: `database_merge_b_team_staff` (0x544970, Database.cpp, cdecl).
///
/// [pt] { 0x544970 } (cdecl, Database.cpp): `database_merge_b_team_staff()`, uma vez na criação (create_game 0x8283d9).
/// Para cada time A com parceiro B (0x540a50 com 1, que exige a marca +0x244 de 0x541e60): conta as pessoas válidas
/// (ponteiro não nulo e id < [0xae2364]) do elenco do A (+0xd7, 50), ordena o elenco do B (qsort com 0x540130) e tira
/// do B as que passarem de 50 no conjunto. Depois, nas casas +0x19f (5, cargo 8), +0x1b3 (7, cargo 9) e +0x1cf (3,
/// cargo 10): cada pessoa válida do B sai dele (0x53c190) e, enquanto o A tiver vaga, entra no A (0x53bd10 com a data
/// do jogo 0xae2c90, gravando na pessoa). Treinador (+0xcf): com os dois, o do B só sai; só o do B: sai e entra no A
/// com o cargo dele (+0x3d, lido depois da saída; a original empurra o byte com lixo em cima, e a 0x53bd10 só usa o
/// byte), sem data. Auxiliar (+0xd3): com os dois, o do B sai. Sem sorteio.
pub(crate) unsafe extern "C" fn database_merge_b_team_staff() {
    let rd = |a: usize| read_usize(a);
    let valid = |p: usize| p != 0 && *(p as *const i32) < read_i32(G_STAFF_COUNT);
    let contar = |c: usize, d: usize, k: usize| (0..k).filter(|&j| valid(rd(c + d + 4 * j))).count() as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let mut eh_a = 0u32;
        let b = Club(exe::club_get_linked_b_team(club, &mut eh_a, 1));
        if !b.is_null() && eh_a != 0 {
            let mut n = contar(club, 0xd7, 50);
            exe::crt_qsort(b.addr() + club::SQUAD, 50, 4, 0x540130);
            for j in 0..50 {
                let p = rd(b.addr() + club::SQUAD + 4 * j);
                if valid(p) {
                    n += 1;
                    if n > 50 {
                        exe::database_remove_staff_from_club(b, p, 0, 1, 0);
                    }
                }
            }
            for &(d, k, cargo) in &[(0x19fusize, 5usize, 8u32), (0x1b3, 7, 9), (0x1cf, 3, 10)] {
                let mut n = contar(club, d, k);
                for j in 0..k {
                    let p = rd(b.addr() + d + 4 * j);
                    if valid(p) {
                        exe::database_remove_staff_from_club(b, p, 0, 1, 0);
                        if n < k as i32 { // the B team’s staff go to the A team
                            n += 1;
                            exe::database_add_staff_to_club(club, p, cargo, G_CURRENT_DATE, 1, 0);
                        }
                    }
                }
            }
            let tb = b.manager();
            if read_usize(club + club::MANAGER) != 0 && !tb.is_null() {
                exe::database_remove_staff_from_club(b, tb, 0, 1, 0);
            } else if !tb.is_null() {
                exe::database_remove_staff_from_club(b, tb, 0, 1, 0);
                exe::database_add_staff_to_club(club, tb, tb.job_for_club() as u32, 0, 1, 0);
            }
            if read_usize(club + club::ASSISTANT_MANAGER) != 0 && b.assistant_manager() != 0 {
                exe::database_remove_staff_from_club(b, b.assistant_manager(), 0, 1, 0);
            }
        }
        i += 1;
        club += club::SIZE;
    }
}

/// `database_build_nation_name_index()`: allocates and builds the per-nation index of the first, second and common name
/// tables. Called at creation (create_game 0x82836c) and when loading the game.
///
/// Allocates (malloc `exe::crt_malloc`) [0xae2350] entries of 0x24 bytes in [0xae2338]: first row (+0, +4, +8 = −1), weight
/// total (+0xc, +0x10, +0x14 = 0) and count (+0x18, +0x1c, +0x20 = 0) of the first, second and common names. Walks the
/// second names ([0xae23f4], [0xae239c]), the first names ([0xae23f0], [0xae2398]) and the common names ([0xae23f8],
/// [0xae23a0]), 60-byte rows with the nation at +0x37 (0..[0xae2350]) and the weight at +0x3b (signed byte):
/// consecutive rows of the same nation form a group, written into the entry of the nation's id (+0) when the nation
/// changes.
///
/// Defect of the original, copied: the last group of each table is not written. Out of memory: the original.
/// Original: `database_build_nation_name_index` (0x539750, Database.cpp, cdecl).
///
/// [pt] { 0x539750 } (cdecl, Database.cpp): `database_build_nation_name_index()`, na criação (create_game 0x82836c) e na
/// carga do jogo. Aloca (malloc 0x944e46) [0xae2350] entradas de 0x24 bytes em [0xae2338]: primeira linha (+0, +4,
/// +8 = −1), soma dos pesos (+0xc, +0x10, +0x14 = 0) e quantidade (+0x18, +0x1c, +0x20 = 0) dos primeiros, segundos
/// e nomes comuns. Percorre os segundos nomes ([0xae23f4], [0xae239c]), os primeiros ([0xae23f0], [0xae2398]) e os
/// comuns ([0xae23f8], [0xae23a0]), linhas de 60 bytes com a nação em +0x37 (0..[0xae2350]) e o peso em +0x3b
/// (byte com sinal): linhas seguidas da mesma nação formam um grupo, gravado na entrada do id (+0) da nação quando
/// a nação muda. Erro do original, copiado: o último grupo de cada tabela não é gravado. Sem memória: a original.
pub(crate) unsafe extern "C" fn database_build_nation_name_index() {
    let nations = read_i32(G_NATION_COUNT);
    let t = exe::crt_malloc((nations as u32).wrapping_mul(0x24));
    write_usize(G_NATION_NAME_INDEX_TABLE, t);
    if t == 0 {
        // simplification: the original asks again and, out of memory, opens the message box and ends the game
        return original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_BUILD_NATION_NAME_INDEX)();
    }
    fill_nation_name_index(t);
}

/// Body of `database_build_nation_name_index`: resets the per-nation name index table and, for the first/second/common name tables, records each
/// nation's first row, weight sum and count.
///
/// [pt] O corpo da 0x539750 sobre a tabela `t`, já em [0xae2338]. O comparador passa um buffer do Rust: montar a cópia
/// com o malloc do jogo deslocava a heap e mudava a impressão das partidas no jogo real comparando (232a, sonda).
pub(crate) unsafe fn fill_nation_name_index(t: usize) {
    let mut k = 0i32;
    while k < read_i32(G_NATION_COUNT) {
        let e = t + k as usize * 0x24;
        for &(d, v) in &[(0usize, -1i32), (4, -1), (8, -1), (0xc, 0), (0x10, 0), (0x14, 0), (0x18, 0), (0x1c, 0), (0x20, 0)] {
            write_i32(e + d, v);
        }
        k += 1;
    }
    // (count, table, +first row, +sum of weights, +count), in the original’s order
    for &(qtd, tabela, d_inicio, d_soma, d_quantos) in &[
        (0xae239cusize, G_SECOND_NAME_TABLE, 4usize, 0x10usize, 0x1cusize),
        (0xae2398, G_FIRST_NAME_TABLE, 0, 0xc, 0x18),
        (G_COMMON_NAME_COUNT, G_COMMON_NAME_TABLE, 8, 0x14, 0x20),
    ] {
        let (mut atual, mut inicio, mut soma, mut quantos) = (0usize, -1i32, 0i32, 0i32);
        let mut l = 0i32;
        while l < read_i32(qtd) {
            let row = read_usize(tabela) + l as usize * 0x3c;
            let nation = read_i32(row + 0x37);
            if nation > -1 && nation < read_i32(G_NATION_COUNT) {
                let record = read_usize(G_NATION_TABLE).wrapping_add((nation as usize).wrapping_mul(nation::SIZE));
                let weight = read_i8(row + 0x3b) as i32;
                if atual == record {
                    soma = soma.wrapping_add(weight);
                    quantos += 1;
                } else {
                    if atual != 0 {
                        let id = read_i32(atual);
                        let e = read_usize(G_NATION_NAME_INDEX_TABLE).wrapping_add(id.wrapping_mul(0x24) as usize);
                        write_i32(e + d_soma, soma);
                        write_i32(e + d_inicio, inicio);
                        write_i32(e + d_quantos, quantos); // group size
                    }
                    inicio = l;
                    quantos = 1;
                    atual = record;
                    soma = weight;
                }
            }
            l += 1;
        }
    }
}

/// The regions compared by the comparers of `database_trim_senior_and_b_team_squads` and `database_merge_b_team_staff`.
///
/// The whole club table (581 bytes per row: everyone's +0x244 mark and the pairs' slots) and the persons (110 bytes) in
/// the slots of the A and B clubs of the `exe::club_get_linked_b_team` pairs, the only ones the two routines remove, add or switch off
/// (`database_remove_staff_from_club` and `database_add_staff_to_club` only touch the club and the person).
///
/// [pt] As regiões dos comparadores de 0x541e60 e 0x544970: a tabela de clubes inteira (581 bytes por linha: a marca
/// +0x244 de todos e as casas dos pares) e as pessoas (110 bytes) nas casas dos clubes A e B dos pares de 0x540a50,
/// as únicas que as duas tiram, põem ou desligam (0x53c190 e 0x53bd10 só mexem no clube e na pessoa).
pub(crate) unsafe fn ab_team_regions() -> Vec<(usize, usize)> {
    let table = read_usize(G_CLUB_TABLE);
    let n = read_i32(G_CLUB_COUNT);
    let mut regions = vec![(table, n.max(0) as usize * 581)];
    let mut persons = Vec::new();
    for &(a, b) in A_B_CLUB_PAIRS.iter() {
        for g in [a, b] {
            let row = read_i32(g);
            if row < 0 || row >= n {
                continue;
            }
            let c = table + row as usize * 581;
            for &(d, k) in &[(0xbfusize, 1usize), (0xc3, 3), (0xcf, 1), (0xd3, 1), (0xd7, 50), (0x19f, 5), (0x1b3, 7), (0x1cf, 3)] {
                for j in 0..k {
                    let p = read_usize(c + d + 4 * j);
                    if p != 0 {
                        persons.push(p);
                    }
                }
            }
        }
    }
    persons.sort_unstable();
    persons.dedup();
    regions.extend(persons.into_iter().map(|p| (p, 110)));
    regions
}

/// `database_remove_staff_from_club(club, person, date, detach, as_player)`: removes the person from the club slot her
/// job points to; 0 if she was not in the slot. Called at creation (the trims of `database_trim_senior_and_b_team_squads`, `database_merge_b_team_staff` and 0x524d70) and
/// in the career (transfers, sackings).
///
/// National team = club id ≥ [0xae235c] − 2 × [0xae2350]. With `date`: its 8 bytes go into the person (+0x29 for a
/// national team, otherwise +0x3e; `exe::date_operator_3d` copies +2, +0 and +4 and repeats into a local nobody reads), before looking
/// at the job. Job = person +0x28 (national team) or +0x3d (byte); with `as_player`, 11, 12, 13 and 15 become 11 and
/// the others 0.
///
/// Slots (jump table 0x53c54c): 0 none; 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf and, for the senior national team (id <
/// [0xae235c] − [0xae2350]), the under-21 side's +0xcf (`exe::database_get_national_u21_team` with the nation +0x53) if it is the same person; 6, 7
/// +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 squad +0xd7 (50); 12 +0xcf and the squad; 13 +0xd3 and the
/// squad; 15 +0x19f and the squad; 14, above 15 or negative: 0.
///
/// The slot that has the person (the first) is zeroed; person not in the slot: 0 (in 12, 13 and 15 the first slot,
/// already zeroed, stays so). With `detach`: clears the club in the person (+0x24 for a national team, +0x39 for a
/// club). Returns 1. Null club: 0. Null person: the original ("Error" box and 0).
/// Original: `database_remove_staff_from_club` (0x53c190, Database.cpp, cdecl).
///
/// [pt] { 0x53c190 } (cdecl, Database.cpp): `database_remove_staff_from_club(clube, pessoa, data, desligar, como_jogador)`.
/// Na criação (os cortes de 0x541e60, 0x544970 e 0x524d70) e na carreira (transferências, demissões). Seleção = id do
/// clube ≥ [0xae235c] − 2 × [0xae2350]. Com `data`: os 8 bytes dela vão para a pessoa (+0x29 na seleção, senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê), antes de olhar o cargo. Cargo = pessoa +0x28 (seleção)
/// ou +0x3d (byte); com `como_jogador`, 11, 12, 13 e 15 viram 11 e os outros 0. Casas (tabela de salto 0x53c54c): 0
/// nenhuma; 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf e, na seleção principal (id < [0xae235c] − [0xae2350]), o +0xcf da sub-21
/// (0x53b4f0 com a nação +0x53) se for a mesma pessoa; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco
/// +0xd7 (50); 12 +0xcf e o elenco; 13 +0xd3 e o elenco; 15 +0x19f e o elenco; 14, acima de 15 ou negativo: 0. A casa
/// que tem a pessoa (a primeira) é zerada; pessoa fora da casa: 0 (nas 12, 13 e 15 a primeira casa já zerada fica).
/// Com `desligar`: zera o clube na pessoa (+0x24 na seleção, +0x39 no clube). Devolve 1. Clube nulo: 0. Pessoa nula: a
/// original (caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_remove_staff_from_club(club: usize, person: usize, data: usize, disable: u32, how_player: u32) -> u32 {
    let person = Staff(person);
    if club == 0 {
        return 0;
    }
    if person.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_REMOVE_STAFF_FROM_CLUB)(club, person.addr(), data, disable, how_player);
    }
    let gi = |a: usize| read_i32(a);
    let national_team = || gi(club) >= gi(G_CLUB_COUNT).wrapping_sub(gi(G_NATION_COUNT).wrapping_mul(2));
    if data != 0 {
        let d = if national_team() { person.addr() + staff::DATE_JOINED_NATION } else { person.addr() + staff::DATE_JOINED_CLUB };
        write_u16(d + 2, read_u16(data + 2));
        write_u16(d, read_u16(data));
        write_u32(d + 4, read_u32(data + 4));
    }
    let mut job = if national_team() { person.job_for_nation() } else { person.job_for_club() };
    if how_player != 0 {
        job = if matches!(job, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    let clear = |d: usize| write_usize(club + d, 0);
    // the first of the `k` slots starting at +d that holds the person, cleared; false if none does
    let remove_of_home = |d: usize, k: usize| match (0..k).find(|&j| read_usize(club + (d + 4 * j)) == person.addr()) {
        Some(j) => {
            clear(d + 4 * j);
            true
        }
        None => false,
    };
    let tirou = match job {
        0 => true,
        1 => remove_of_home(0xbf, 1),
        2..=4 => remove_of_home(0xc3, 3),
        5 => {
            let ok = remove_of_home(0xcf, 1);
            let id = gi(club);
            let n = gi(G_CLUB_COUNT);
            if ok && id >= n.wrapping_sub(gi(G_NATION_COUNT).wrapping_mul(2)) && id < n.wrapping_sub(gi(G_NATION_COUNT)) {
                let s = Club(exe::database_get_national_u21_team(read_usize(club + 0x53)));
                if s.manager() == person {
                    write_usize(s.addr() + club::MANAGER, 0); // the U21 coach
                }
            }
            ok
        }
        6 | 7 => remove_of_home(0xd3, 1),
        8 => remove_of_home(0x19f, 5),
        9 => remove_of_home(0x1b3, 7),
        10 => remove_of_home(0x1cf, 3),
        11 => remove_of_home(0xd7, 50),
        12 => remove_of_home(0xcf, 1) && remove_of_home(0xd7, 50),
        13 => remove_of_home(0xd3, 1) && remove_of_home(0xd7, 50),
        15 => remove_of_home(0x19f, 5) && remove_of_home(0xd7, 50),
        _ => false,
    };
    if !tirou {
        return 0;
    }
    if disable != 0 {
        if national_team() {
            person.set_national_job(Club(0));
        } else {
            person.set_club_job(Club(0)); // detaches the person from the club
        }
    }
    1
}

/// `database_get_club_scouts(club, array)`: the 7 scouts (+0x1b3) of `club` into `array`.
///
/// Without an A/B partner (`exe::club_get_linked_b_team` with null and 1, which zeroes the club's flag +0x244 when it rejects the partner):
/// the 7 slots as they are. With a partner: the club's non-null ones and then the partner's, up to 7; the rest zeroed
/// only when the two together do not fill the 7. 8-bit counter in the original (at most 7). Returns 1. Null club: the
/// original ("Error" box and 0). In the career, through `sync_scout_lists` (the scouts' lists, in transfers and hirings).
/// Original: `database_get_club_scouts` (0x53cd60, Database.cpp, cdecl).
///
/// [pt] { 0x53cd60 } (cdecl, Database.cpp): `database_get_club_scouts(clube, vetor)`: os 7 olheiros (+0x1b3) do `clube` no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro): as 7
/// casas como estão. Com parceiro: os não nulos do clube e depois os do parceiro até 7; o resto zerado só quando os
/// dois juntos não enchem as 7. Contador de 8 bits na original (no máximo 7). Devolve 1. Clube nulo: a original
/// (caixa "Error" e 0). Na carreira, pela 0x7f7270 (as listas dos olheiros, nas transferências e contratações).
pub(crate) unsafe extern "C" fn database_get_club_scouts(club: usize, array: *mut usize) -> u32 {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_SCOUTS)(club.addr(), array);
    }
    let rd = |a: usize| read_usize(a);
    let p = Club(exe::club_get_linked_b_team(club.addr(), core::ptr::null_mut(), 1));
    if p.is_null() {
        for j in 0..7 {
            core::ptr::write_unaligned(array.add(j), rd(club.addr() + club::SCOUTS + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..7 {
        let o = rd(club.addr() + club::SCOUTS + 4 * j);
        if o != 0 {
            core::ptr::write_unaligned(array.add(n), o);
            n += 1;
        }
    }
    for j in 0..7 {
        if n >= 7 {
            return 1;
        }
        let o = rd(p.addr() + club::SCOUTS + 4 * j); // the partner’s scouts
        if o != 0 {
            core::ptr::write_unaligned(array.add(n), o);
            n += 1;
        }
    }
    for k in n..7 {
        core::ptr::write_unaligned(array.add(k), 0);
    }
    1
}

/// As formas do adjetivo de `get_adjective_form_index` (tabelas de salto 0x545b2c/0x545b34, 0x545b58/0x545b60, 0x545b84,
/// 0x545c10, 0x545c9c e 0x545d28), índice 1..35. Os idiomas 3, 6, 7, 8 e 10 dão a mesma linha.
pub(crate) const ADJECTIVE_FORMS_COMMON: [u8; 35] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub(crate) const ADJECTIVE_FORMS_LANGUAGE_4: [u8; 35] = [
    2, 3, 0, 2, 2, 3, 0, 0, 2, 0, 0, 0, 1, 1, 0, 1, 2, 3, 0, 3, 0, 0, 0, 2, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0,
];
pub(crate) const ADJECTIVE_FORMS_LANG5: [u8; 35] = [
    2, 3, 2, 2, 1, 3, 2, 1, 2, 2, 3, 1, 3, 3, 1, 3, 2, 3, 2, 4, 1, 1, 1, 2, 2, 1, 3, 2, 1, 1, 1, 1, 3, 2, 2,
];
pub(crate) const ADJECTIVE_FORMS_9: [u8; 35] = [
    1, 2, 1, 1, 0, 1, 1, 1, 1, 1, 2, 0, 2, 3, 1, 2, 2, 3, 1, 3, 0, 0, 0, 1, 1, 0, 2, 1, 0, 0, 0, 0, 1, 1, 0,
];

/// the adjective form (0..4) for index 1..35 in the inflecting languages 3..10 (jump tables); any other language or
/// index: 0xff
/// Original: `get_adjective_form_index` (0x545a60, Database.cpp, cdecl).
///
/// [pt] { 0x545a60 } (cdecl, Database.cpp): a forma (0..4) do adjetivo para o `indice` (byte com sinal,
/// 1..35) no `idioma` (0x66f4d0; os chamadores só chamam nos idiomas que flexionam). Idioma sem tabela
/// ou índice fora: 0xff. Sem chamadas e sem caixa.
pub(crate) unsafe extern "C" fn get_adjective_form_index(index: u32, language: u32) -> u8 {
    let row = match language {
        3 | 6 | 7 | 8 | 10 => &ADJECTIVE_FORMS_COMMON,
        4 => &ADJECTIVE_FORMS_LANGUAGE_4,
        5 => &ADJECTIVE_FORMS_LANG5,
        9 => &ADJECTIVE_FORMS_9,
        _ => return 0xff,
    };
    let i = (index as u8 as i8 as i32).wrapping_sub(1) as u32; // the original’s ja compares unsigned
    if i > 0x22 {
        return 0xff;
    }
    row[i as usize]
}

/// the person's coaching attribute 1..12: `staff_compute_non_player_attribute` with their non-player record (+0x69)
/// Original: `staff_get_non_player_attribute` (0x543f30, Database.cpp, cdecl).
///
/// [pt] { 0x543f30 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`: 0x543fd0 com o
/// não jogador dela (+0x69) e o `atributo` inteiro como veio. Pessoa nula ou atributo (byte com sinal)
/// fora de 1..12: a original (caixa "Error" e −1).
pub(crate) unsafe extern "C" fn staff_get_non_player_attribute(person: usize, attribute: u32) -> u8 {
    let person = Staff(person);
    let a = attribute as u8 as i8;
    if person.is_null() || !(1..=12).contains(&a) {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_NON_PLAYER_ATTRIBUTE)(person.addr(), attribute);
    }
    let not_player = person.non_player();
    exe::staff_compute_non_player_attribute(person, not_player, attribute)
}

/// the 1-to-20 rating of 2 × value + current ability × 0.1 (`player_attribute_rating` with the ability scaled down)
/// Original: `player_attribute_rating_with_scaled_ability` (0x5442f0, Database.cpp, cdecl).
///
/// [pt] { 0x5442f0 } (cdecl, Database.cpp): a nota de 1 a 20 (`attribute_rating`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador` × 0,1 ([0x967880]); a de 0x544380 com a habilidade
/// reduzida (os goleiros de 0x543540). Termina no `_ftol`.
pub(crate) unsafe extern "C" fn player_attribute_rating_with_scaled_ability(player: usize, value: u32) -> u32 {
    let ability = read_i16(player + 5) as f64;
    let v = (value as u8 as i8) as f64;
    attribute_rating(ability * read_f64(rdata::F64_0_1) + (v + v)) as u32
}

/// A first name (60-byte row in [0xae23f0]) by weight.
///
/// A nation with more than 15 first names (`nation_name_index` +0x18): 3 draws in its range, the one with the highest
/// weight is kept (+0x3b, signed; starts at −1, and row 0 if none passes). No nation or up to 15: a draw over the whole
/// table ([0xae2398], at most 0x7fff). Only the CRT rand().
/// Original: `database_pick_first_name_by_weight` (0x539b90, Database.cpp, cdecl).
///
/// [pt] { 0x539b90 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) pelo peso.
/// Nação com mais de 15 primeiros nomes (`nation_name_index` +0x18): 3 sorteios na faixa dela,
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com
/// até 15: sorteio na tabela inteira ([0xae2398], no máximo 0x7fff). Só o rand() da CRT.
pub(crate) unsafe extern "C" fn database_pick_first_name_by_weight(nation: usize) -> usize {
    let row = |i: i32| read_usize(G_FIRST_NAME_TABLE).wrapping_add((i as usize).wrapping_mul(60));
    if nation != 0 {
        let e = nation_name_index(nation);
        let how_many = read_i32(e + nation_name_index::FIRST_NAME_COUNT);
        if how_many > 0xf {
            let start = read_i32(e);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 weighted draws
                let i = scaled_crt_rand(how_many).wrapping_add(start);
                let weight = read_i8(row(i) + 0x3b);
                if weight > melhor {
                    melhor = weight;
                    escolhido = i;
                }
            }
            return row(escolhido);
        }
    }
    row(scaled_crt_rand(name_count_capped(0xae2398)))
}

/// The colours of `competition` in pixels (`exe::graphics_rgb_to_pixel` with bytes +0x37, +0x38, +0x39 of the colour and format 0):
/// `foreground` and `background`, which start with [0xae3184] and [0xaebe04].
///
/// The colours come from the +0x5d record (its +0x90 and +0x94) or, without it, from the competition itself (+0x61 and
/// +0x65); a null colour leaves the default. Returns 1. Null competition: the original (defaults, "Error" box and 0).
/// Original: `database_get_comp_colours` (0x53b7c0, Database.cpp, cdecl).
///
/// [pt] { 0x53b7c0 } (cdecl, Database.cpp): as cores da `competicao` em pixels (0x5e4800 com os bytes +0x37,
/// +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que começam com [0xae3184] e [0xaebe04]. As
/// cores vêm do registro +0x5d (+0x90 e +0x94 dele) ou, sem ele, da própria competição (+0x61 e +0x65);
/// cor nula deixa o padrão. Devolve 1. Competição nula: a original (padrões, caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_comp_colours(competition: usize, front: *mut u16, bottom: *mut u16) -> u32 {
    let competition = Competition(competition);
    if competition.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_COMP_COLOURS)(competition.addr(), front, bottom);
    }
    // simplification: the original pushes dwords with only the low byte written (the rest is stack garbage);
    // 0x5e4800 does & 0xff on all three, so the extended bytes give the same pixel (as in 0x53b640)
    core::ptr::write_unaligned(front, read_u16(G_GUI_COLOUR_PALE_GREY));
    core::ptr::write_unaligned(bottom, read_u16(G_GUI_COLOUR_DARK_BLUE));
    let owner = competition.nation();
    let (a, b) = if !owner.is_null() { (owner.foreground_colour1(), owner.background_colour1()) } else { (competition.foreground_colour(), competition.background_colour()) };
    let pixel = |c: usize| {
        exe::graphics_rgb_to_pixel((read_u8(c + 0x37) as u32), (read_u8(c + 0x38) as u32), (read_u8(c + 0x39) as u32), (0) as u32) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(front, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(bottom, pixel(b));
    }
    1
}

/// True if any of the n club slots at clube+d is empty or already holds the person.
///
/// [pt] Há vaga para a `pessoa` nas `n` casas a partir de `clube + d`: alguma vazia ou já com ela.
pub(crate) unsafe fn has_slot_for(club: usize, d: usize, n: usize, person: usize) -> bool {
    (0..n).any(|i| {
        let v = read_usize(club + d + 4 * i);
        v == 0 || v == person
    })
}

/// 1 if `club` has room for `person` in `job` (signed byte, jump table 0x53bcc8): an empty slot or one already theirs.
///
/// 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 squad +0xd7 (50); 12 and
/// 13: +0xcf / +0xd3 free or hers and room in the squad; 15: room in +0x19f and in the squad. Null club or person, job
/// 14 or outside 1..15: the original (error box and 0).
/// Original: `database_club_has_room_for_job` (0x53b9a0, Database.cpp, cdecl).
///
/// [pt] { 0x53b9a0 } (cdecl, Database.cpp): 1 se o `clube` tem vaga para a `pessoa` no `cargo` (byte com
/// sinal, tabela de salto 0x53bcc8): 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3
/// (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 e 13: +0xcf / +0xd3 livre ou dela e vaga no elenco; 15:
/// vaga em +0x19f e no elenco. Clube ou pessoa nulos, cargo 14 ou fora de 1..15: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_club_has_room_for_job(club: usize, job: u32, person: usize) -> u32 {
    let c = job as u8 as i8;
    if club == 0 || person == 0 || !(1..=15).contains(&c) || c == 14 {
        return original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_DATABASE_CLUB_HAS_ROOM_FOR_JOB)(club, job, person);
    }
    let slot = |d: usize, n: usize| has_slot_for(club, d, n, person);
    let has = match c {
        1 => slot(0xbf, 1),
        2..=4 => slot(0xc3, 3),
        5 => slot(0xcf, 1),
        6 | 7 => slot(0xd3, 1),
        8 => slot(0x19f, 5),
        9 => slot(0x1b3, 7),
        10 => slot(0x1cf, 3),
        11 => slot(0xd7, 50),
        12 => slot(0xcf, 1) && slot(0xd7, 50),
        13 => slot(0xd3, 1) && slot(0xd7, 50),
        _ => slot(0x19f, 5) && slot(0xd7, 50), // 15
    };
    has as u32
}

/// Puts the person in the first empty of the n club slots at clube+d; false if none.
///
/// [pt] Põe a `pessoa` na primeira das `n` casas vazias a partir de `clube + d`; false se não há.
pub(crate) unsafe fn fill_club_slot(club: usize, d: usize, n: usize, person: Staff) -> bool {
    let home = |i: usize| (club + d + 4 * i) as *mut usize;
    match (0..n).find(|&i| core::ptr::read_unaligned(home(i)) == 0) {
        Some(i) => {
            core::ptr::write_unaligned(home(i), person.addr());
            true
        }
        None => false,
    }
}

/// Puts `person` at `club` in `job` (byte, jump table 0x53c14c), copies the date and writes club and job into the
/// person.
///
/// A person in the humans' range (id ≥ [0xae2364] − 16) only in job 5. With `date`: its 8 bytes go into the person
/// (+0x29 if the club is a national team, id ≥ [0xae235c] − 2 × [0xae2350]; otherwise +0x3e; `exe::date_operator_3d` copies +2, +0 and
/// +4 and repeats into a local nobody reads). With `reduce`: jobs 11, 12, 13 and 15 become 11, the others 0. A player
/// (+0x61) with current ability (+5) > 85 at a club without a human manager (`exe::human_manager_is_human_club` with 0, 1, 0, 0): 6 becomes 13
/// and 8 becomes 15.
///
/// Slots: 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 squad +0xd7 (50);
/// 12 +0xcf and the squad (no room in the squad: 0 with +0xcf already written); 13 +0xd3 and the squad (no room in the
/// squad, stays 6); 15 +0x19f and the squad (no room in the squad, stays 8); 0 none; 14 and outside: 0.
///
/// With `write`: club and job into the person (+0x24/+0x28 national team, +0x39/+0x3d club). Returns 1. Null club or
/// person: the original ("Error" box and 0).
/// Original: `database_add_staff_to_club` (0x53bd10, Database.cpp, cdecl).
///
/// [pt] { 0x53bd10 } (cdecl, Database.cpp): põe a `pessoa` no `clube` no `cargo` (byte, tabela de salto
/// 0x53c14c). Pessoa da faixa dos humanos (id ≥ [0xae2364] − 16) só no cargo 5. Com `data`: as 8 bytes
/// dela vão para a pessoa (+0x29 se o clube é seleção, id ≥ [0xae235c] − 2 × [0xae2350]; senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê). Com `reduzir`: cargos 11, 12, 13 e 15
/// viram 11, os outros 0. Jogador (+0x61) com habilidade atual (+5) > 85 num clube sem técnico humano
/// (0x600a50 com 0, 1, 0, 0): 6 vira 13 e 8 vira 15. Casas: 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3;
/// 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 +0xcf e o elenco (sem elenco: 0
/// com o +0xcf já gravado); 13 +0xd3 e o elenco (sem elenco fica 6); 15 +0x19f e o elenco (sem elenco
/// fica 8); 0 nenhuma; 14 e fora: 0. Com `gravar`: clube e cargo na pessoa (+0x24/+0x28 seleção,
/// +0x39/+0x3d clube). Devolve 1. Clube ou pessoa nulos: a original (caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_add_staff_to_club(club: usize, person: usize, job: u32, data: usize, write: u32, reduzir: u32) -> u32 {
    let person = Staff(person);
    if club == 0 || person.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(IDX_DATABASE_ADD_STAFF_TO_CLUB)(club, person.addr(), job, data, write, reduzir);
    }
    let gi = |a: usize| read_i32(a);
    let national_team = || gi(club) >= gi(G_CLUB_COUNT).wrapping_sub(gi(G_NATION_COUNT).wrapping_mul(2));
    if gi(person.addr()) >= gi(G_STAFF_COUNT).wrapping_sub(16) && job as u8 != 5 {
        return 0;
    }
    if data != 0 {
        let d = if national_team() { person.addr() + staff::DATE_JOINED_NATION } else { person.addr() + staff::DATE_JOINED_CLUB };
        write_u16(d + 2, read_u16(data + 2));
        write_u16(d, read_u16(data));
        write_u32(d + 4, read_u32(data + 4));
    }
    let mut c = job as u8;
    if reduzir != 0 {
        c = if matches!(c, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    if !person.player().is_null() {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
            let player = person.player();
            if read_i16(player.addr() + 5) > 0x55 {
                if c == 6 {
                    c = 0xd;
                } else if c == 8 {
                    c = 0xf;
                }
            }
        }
    }
    let ocupar = |d: usize, n: usize| fill_club_slot(club, d, n, person);
    let unica = |d: usize| {
        let p = (club + d) as *mut usize;
        if core::ptr::read_unaligned(p) != 0 {
            return false;
        }
        core::ptr::write_unaligned(p, person.addr());
        true
    };
    let ok = match c as i8 {
        0 => true,
        1 => unica(0xbf),
        2..=4 => ocupar(0xc3, 3),
        5 => unica(0xcf),
        6 | 7 => unica(0xd3),
        8 => ocupar(0x19f, 5),
        9 => ocupar(0x1b3, 7),
        10 => ocupar(0x1cf, 3),
        11 => ocupar(0xd7, 50),
        12 => unica(0xcf) && ocupar(0xd7, 50),
        13 => {
            if !unica(0xd3) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 6; // no room in the squad: stays only in cell +0xd3
            }
            true
        }
        15 => {
            if !ocupar(0x19f, 5) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 8; // no squad slot: stays only at +0x19f
            }
            true
        }
        _ => false, // 14 and outside 0..15
    };
    if !ok {
        return 0;
    }
    if write != 0 {
        if national_team() {
            person.set_national_job(Club(club));
            person.set_job_for_nation(c);
        } else {
            person.set_club_job(Club(club));
            person.set_job_for_club(c); // club and club role
        }
    }
    1
}

/// Person pointer in squad slot i (+0xd7) of the club.
///
/// [pt] Um dos 50 lugares do elenco (+0xd7, ponteiros de pessoa) do `clube`.
pub(crate) unsafe fn squad_slot(club: Club, i: usize) -> usize {
    read_usize(club.addr() + club::SQUAD + 4 * i)
}

/// Person is non-null and its id is below the staff count.
///
/// [pt] A pessoa do elenco conta: não nula e com id abaixo de [0xae2364] (com sinal).
pub(crate) unsafe fn is_valid_squad_person(p: usize) -> bool {
    p != 0 && *(p as *const i32) < read_i32(G_STAFF_COUNT)
}

/// How many valid people are in the squad of `club` (byte).
///
/// With an A/B partner (`exe::club_get_linked_b_team` with 1): `mode` 2 and an A club counts only the partner; otherwise counts the club
/// and, except for `mode` 1 with a B club, also the partner. Without a partner and with a reserve team (`database_club_has_reserve_squad`): only
/// the people with the `mode` bits in the squad flags (+0x5e), if `mode` is not 0.
///
/// With `filter`, someone who has a loan contract record (`contract_record`) and a club (+0x39) other than `club` only counts if
/// the contract's club (`exe::contract_manager_get_contract`, stored in [0xae1080]; +4 = id) is also another one (the loanee filter). Null club:
/// the original (error box).
/// Original: `database_count_club_squad` (0x53c590, Database.cpp, cdecl).
///
/// [pt] { 0x53c590 } (cdecl, Database.cpp): quantas pessoas válidas há no elenco do `clube` (byte). Com
/// parceiro A/B (0x540a50 com 1): `modo` 2 e clube A conta só o parceiro; senão conta o clube e, fora
/// `modo` 1 com clube B, também o parceiro. Sem parceiro e com time reserva (0x542740): só as pessoas
/// com bits `modo` nas marcas de elenco (+0x5e), se `modo` não for 0. Com `filtro`, quem tem ficha
/// de contrato (0x4cf6d0) e clube (+0x39) diferente do `clube` só conta se o clube do contrato
/// (0x4cf5a0, guardado em [0xae1080]; +4 = id) também for outro. Clube nulo: a original (caixa).
pub(crate) unsafe extern "C" fn database_count_club_squad(club: usize, mode: u32, filter: u32) -> u8 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(IDX_DATABASE_COUNT_CLUB_SQUAD)(club, mode, filter);
    }
    let mode = mode as u8;
    let count = |c: usize, marks: u8| -> u8 {
        let mut n = 0u8;
        for i in 0..50 {
            let p = squad_slot(Club(c), i);
            if !is_valid_squad_person(p) || (marks != 0 && *((p + staff::PLAYING_SQUAD) as *const u8) & marks == 0) {
                continue;
            }
            if filter != 0 && exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0
                && read_usize(p + staff::CLUB_JOB) != club {
                let k = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
                write_usize(G_CONTRACT_LOOKUP_RESULT, k.addr());
                let owner = if k.is_null() {
                    0
                } else {
                    read_usize(G_CLUB_TABLE).wrapping_add(((k.club() as u32) as usize).wrapping_mul(581))
                };
                if owner == club {
                    continue;
                }
            }
            n = n.wrapping_add(1);
        }
        n
    };
    // simplification: the original passes the address of a local; 0x540a50 only returns a partner after writing it
    let mut eh_a = 0u32;
    let p = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    if p == 0 {
        if exe::database_club_has_reserve_squad(club) != 0 {
            return count(club, mode);
        }
        return count(club, 0);
    }
    if mode == 2 && eh_a != 0 {
        return count(p, 0);
    }
    let n = count(club, 0);
    if mode == 1 && eh_a == 0 {
        return n;
    }
    n.wrapping_add(count(p, 0))
}

/// Copies the squad of `club` (50 pointers at +0xd7) into `array`.
///
/// Without an A/B partner (`exe::club_get_linked_b_team` with null and 1): the 50 as they are and, with `double`, 50 more zeros. With a
/// partner: only the valid people of the club and then those of the partner, up to the limit (100 with `double`,
/// otherwise 50), the rest up to the limit zeroed. Returns 1. Null club: the original (error box and 0).
/// Original: `database_collect_club_squad` (0x541f90, Database.cpp, cdecl).
///
/// [pt] { 0x541f90 } (cdecl, Database.cpp): copia o elenco do `clube` (50 ponteiros em +0xd7) para o
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): os 50 como estão e, com `dobro`, mais 50 zeros.
/// Com parceiro: só as pessoas válidas do clube e depois as do parceiro até o limite (100 com
/// `dobro`, senão 50), o resto até o limite zerado. Devolve 1. Clube nulo: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_collect_club_squad(club: usize, array: *mut usize, dobro: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(IDX_DATABASE_COLLECT_CLUB_SQUAD)(club, array, dobro);
    }
    let limit: i8 = if dobro != 0 { 100 } else { 50 };
    let p = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    if p == 0 {
        for i in 0..50 {
            core::ptr::write_unaligned(array.add(i), squad_slot(Club(club), i));
        }
        if dobro != 0 {
            for i in 50..100 {
                core::ptr::write_unaligned(array.add(i), 0);
            }
        }
        return 1;
    }
    let mut n: i8 = 0;
    for i in 0..50 {
        let q = squad_slot(Club(club), i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(array.add(n as usize), q);
            n += 1;
        }
    }
    for i in 0..50 {
        if n >= limit {
            break;
        }
        let q = squad_slot(Club(p), i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(array.add(n as usize), q);
            n += 1;
        }
    }
    for k in n..limit {
        core::ptr::write_unaligned(array.add(k as usize), 0);
    }
    1
}

/// Whether `club` can be managed. Needs a nation (+0x53).
///
/// National teams (id ≥ [0xae235c] − 2 × [0xae2350], signed): only the senior ones (id < [0xae235c] − [0xae2350]) of a
/// nation with +0x88 > 150. Clubs: with a competition (+0x57), not the B team of a pair (`exe::club_get_linked_b_team` with 1), with the
/// competition in the vector [[0xadadfc] + id × 4] (active) and the nation with bit 2 of +0x11c (flagged).
/// Original: `club_can_be_managed` (0x544820, Database.cpp, cdecl).
///
/// [pt] { 0x544820 } (cdecl, Database.cpp): o `clube` pode ser dirigido? Precisa de nação (+0x53). Seleções
/// (id ≥ [0xae235c] − 2 × [0xae2350], com sinal): só as principais (id < [0xae235c] − [0xae2350]) de
/// nação com +0x88 > 150. Clubes: com competição (+0x57), que não sejam o time B de um par (0x540a50
/// com 1), com a competição no vetor [[0xadadfc] + id × 4] e a nação com o bit 2 de +0x11c.
pub(crate) unsafe extern "C" fn club_can_be_managed(club: usize) -> u8 {
    if club == 0 || read_usize(club + 0x53) == 0 {
        return 0;
    }
    let id = *(club as *const i32);
    let (n, total) = (*(G_NATION_COUNT as *const i32), *(G_CLUB_COUNT as *const i32));
    if id >= total.wrapping_sub(n.wrapping_add(n)) {
        return (id < total.wrapping_sub(n) && core::ptr::read_unaligned((read_usize(club + 0x53) + 0x88) as *const i32) > 0x96) as u8;
    }
    if read_usize(club + 0x57) == 0 {
        return 0;
    }
    // simplification: the original uses the argument’s own place as output; it is only read with a partner, and by then it has been written
    let mut eh_a = 0u32;
    if exe::club_get_linked_b_team(club, &mut eh_a, 1) != 0 && eh_a == 0 {
        return 0; // time B
    }
    let table = *(G_COMP_OBJECT_TABLE as *const usize);
    if table == 0 {
        return 0;
    }
    let comp = *(read_usize(club + 0x57) as *const i32);
    if *((table as isize).wrapping_add((comp as isize).wrapping_mul(4)) as *const u32) == 0 {
        return 0;
    }
    (*((read_usize(club + 0x53) + 0x11c) as *const u8) & 2 != 0) as u8
}

/// The inverse of `database_non_player_to_attribute_vector`: the low byte of each of the 21 three-byte cells of `vector` goes back to `non_player` (the
/// coaching attributes +0x0e..+0x22) in the order `NON_PLAYER_ATTRIBUTE_ORDER`.
///
/// The person is not used. eax on exit: the vector with the last copied byte in the low byte (callers do not use it).
/// Original: `database_vector_to_non_player` (0x53a980, Database.cpp, cdecl).
///
/// [pt] { 0x53a980 } (cdecl, Database.cpp): o inverso de 0x53a8b0: o byte de baixo de cada uma das 21
/// células de 3 bytes do `vetor` volta ao `nao_jogador` na ordem `NON_PLAYER_ATTRIBUTE_ORDER`. A pessoa não
/// é usada. eax na saída: o vetor com o último byte copiado no byte de baixo (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_vector_to_non_player(not_player: usize, person: usize, array: usize) -> u32 {
    let mut last = 0u8;
    for (k, &d) in NON_PLAYER_ATTRIBUTE_ORDER.iter().enumerate() {
        last = read_u8(array + 3 * k);
        write_u8(not_player + d, last);
    }
    (array as u32 & 0xffff_ff00) | last as u32
}

/// The inverse of `database_player_to_attribute_vector`: the low byte of cells 0..53 of `vector` goes back to `player` at the offsets of
/// `ATTRIBUTE_VECTOR_CELLS`, and that of cell 54 to `person` +0x58.
///
/// eax on exit: the vector with the last byte in the low byte (callers do not use it).
/// Original: `database_vector_to_player` (0x53aa10, Database.cpp, cdecl).
///
/// [pt] { 0x53aa10 } (cdecl, Database.cpp): o inverso de 0x53a610: o byte de baixo das células 0..53 do
/// `vetor` volta ao `jogador` nos deslocamentos de `ATTRIBUTE_VECTOR_CELLS` e o da célula 54 à `pessoa`
/// +0x58. eax na saída: o vetor com o último byte no byte de baixo (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_vector_to_player(player: usize, person: usize, array: usize) -> u32 {
    let person = Staff(person);
    for (i, &(d, _)) in ATTRIBUTE_VECTOR_CELLS.iter().enumerate() {
        write_u8(player + d as usize, read_u8(array + 3 * i));
    }
    let last = read_u8(array + 3 * 54);
    person.set_determination((last) as i8);
    (array as u32 & 0xffff_ff00) | last as u32
}

/// Rebalances the attributes of the non-player (person +0x69) in two passes.
///
/// Builds the vector (63 cells 0xff81/0x81, filled by `database_non_player_to_attribute_vector`, which returns the cut 13) and, twice, starting from the
/// cut: (1) with limit 40, +10 per round, up to 6 cells with |value| ≤ limit (marked); cell 19 outside the limit is
/// also marked; (2) adds −5 − mean to the marked ones (clamped to −125..125), pulling the mean of the near-zero cells
/// to −5; in the 1st pass the unmarked positive ones go into the excess sum, the others are marked; with job +0x3d or
/// +0x28 = 10, the cells stay ≤ −80 and cell 19 ≥ 0; (3) in the 1st pass, with sum > 28, takes (sum − 28) / count off
/// the unmarked ones (minimum 0).
///
/// At the end, `database_vector_to_non_player` writes back. Null person or non-player: nothing.
/// Original: `staff_normalise_non_player_attributes` (0x539490, Database.cpp, cdecl).
///
/// [pt] { 0x539490 } (cdecl, Database.cpp): reequilibra os atributos do não jogador (pessoa +0x69). Monta
/// o vetor (63 células 0xff81/0x81, preenchidas por 0x53a8b0, que devolve o corte 13) e, duas vezes,
/// a partir do corte: (1) com limite 40, +10 por volta, até 6 células com |valor| ≤ limite (marcadas);
/// a célula 19 fora do limite também é marcada; (2) soma −5 − média às marcadas (presas em −125..125);
/// na 1ª passada as não marcadas positivas entram na soma de excesso, as demais são marcadas; com
/// cargo +0x3d ou +0x28 = 10, as células ficam ≤ −80 e a 19 ≥ 0; (3) na 1ª passada, com soma > 28,
/// tira (soma − 28) / quantas das não marcadas (mínimo 0). No fim, 0x53a980 grava de volta. Pessoa
/// ou não jogador nulos: nada.
pub(crate) unsafe extern "C" fn staff_normalise_non_player_attributes(person: usize) {
    let person = Staff(person);
    if person.is_null() {
        return;
    }
    let nj = person.non_player();
    if nj.is_null() {
        return;
    }
    let mut array = [0u8; 63 * 3];
    for c in array.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = array.as_mut_ptr();
    let mut end = 0u8;
    exe::database_non_player_to_attribute_vector(nj, person, v as usize, &mut end);
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let end = end as i8;
    let mut mark = [0u32; 64];
    let job_10 = person.job_for_club() == 10 || person.job_for_nation() == 10;
    for step_towards in 0..2 {
        let bp = w(end);
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limit = 0x28i32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if bp != -0x7f {
                let mut c = end;
                loop {
                    let x = w(c) as i32;
                    if x.abs() <= limit as i16 as i32 {
                        total += x;
                        n += 1;
                        mark[c as usize] = 1;
                    } else {
                        mark[c as usize] = 0;
                        if c == 0x13 && (melhor == -1 || w(0x13) > w(melhor)) {
                            melhor = 0x13;
                        }
                    }
                    c += 1;
                    if w(c) == -0x7f {
                        break;
                    }
                }
                if n >= 6 {
                    break;
                }
            }
            // simplification: with the cut cell = −127 the original loops forever; the copy too
            limit += 10;
        }
        if melhor > -1 {
            mark[melhor as usize] = 1;
        }
        let delta = (-5i32).wrapping_sub(total / n as i32) as i16;
        if bp != -0x7f {
            let mut c = end;
            loop {
                if c != melhor {
                    if mark[c as usize] == 1 {
                        let x = w(c).wrapping_add(delta);
                        pw(c, if x < -0x7d { -0x7d } else if x > 0x7d { 0x7d } else { x });
                    } else if step_towards == 0 && w(c) > 0 {
                        quantas += 1;
                        soma += w(c) as i32;
                    } else {
                        mark[c as usize] = 1;
                    }
                    if job_10 {
                        if c != 0x13 {
                            pw(c, w(c).min(-0x50));
                        } else {
                            pw(0x13, w(0x13).max(0));
                        }
                    }
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
        if w(end) != -0x7f {
            let mut c = end;
            while quantas != 0 && soma > 0x1c {
                if mark[c as usize] != 1 {
                    let x = w(c).wrapping_sub(((soma - 0x1c) / quantas) as i16);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
    }
    (exe::database_attribute_vector_to_non_player(nj, person, v as usize) as u32);
}

/// The fixed colours (index.cpp keys at 0x9cf178..0x9cf1fc, ids) and the family of each one, in the order of the
/// comparisons in `database_get_colour_family`.
///
/// [pt] As cores fixas (chaves do index.cpp em 0x9cf178..0x9cf1fc, ids) e a família de cada uma, na ordem
/// das comparações de 0x540520.
pub(crate) const COLOUR_FAMILIES: [(usize, u8); 34] = [
    (KEY_COLOUR_BROWN_1, 10), (KEY_COLOUR_BROWN_2, 10), (KEY_COLOUR_BLACK, 1), (KEY_COLOUR_BLUE_5, 1), (KEY_COLOUR_WHITE, 2), (KEY_COLOUR_GREY_1, 3), (KEY_COLOUR_GREY_2, 3),
    (KEY_COLOUR_GREY_3, 3), (KEY_COLOUR_RED_1, 8), (KEY_COLOUR_RED_2, 8), (KEY_COLOUR_RED_3, 8), (KEY_COLOUR_RED_4, 8), (KEY_COLOUR_ORANGE_1, 9), (KEY_COLOUR_ORANGE_2, 9),
    (KEY_COLOUR_GOLD, 4), (KEY_COLOUR_YELLOW_1, 4), (KEY_COLOUR_YELLOW_2, 4), (KEY_COLOUR_AMBER, 4), (KEY_COLOUR_GREEN_1, 5), (KEY_COLOUR_GREEN_2, 5), (KEY_COLOUR_GREEN_3, 5),
    (KEY_COLOUR_GREEN_4, 5), (KEY_COLOUR_GREEN_5, 5), (KEY_COLOUR_BLUE_1, 6), (KEY_COLOUR_BLUE_2, 6), (KEY_COLOUR_BLUE_3, 6), (KEY_COLOUR_BLUE_4, 6), (KEY_COLOUR_CYAN_1, 6),
    (KEY_COLOUR_CYAN_2, 6), (KEY_COLOUR_PURPLE_1, 7), (KEY_COLOUR_PURPLE_2, 7), (KEY_COLOUR_PURPLE_3, 7), (KEY_COLOUR_PURPLE_4, 7), (KEY_COLOUR_PINK, 7),
];

/// Colour family of a colour record via COLOUR_FAMILIES; None if its id matches no key.
///
/// [pt] A família da `cor` pela tabela; None se o id não está em nenhuma chave (caixa na original).
pub(crate) unsafe fn colour_family(colour: usize) -> Option<u8> {
    let id = read_u32(colour);
    COLOUR_FAMILIES.iter().find(|&&(a, _)| read_u32(a) == id).map(|&(_, f)| f)
}

/// the colour's family (1..10) from the index.cpp colour keys
/// Original: `database_get_colour_family` (0x540520, Database.cpp, cdecl).
///
/// [pt] { 0x540520 } (cdecl, Database.cpp): a família (1..10) da `cor` pelo id (+0), comparado com as chaves
/// de cor em `COLOUR_FAMILIES` (a primeira que casa). Cor fora da lista: a original (caixa "Error",
/// [0xb67a34] = 0 e 2).
pub(crate) unsafe extern "C" fn database_get_colour_family(colour: usize) -> u8 {
    match colour_family(colour) {
        Some(f) => f,
        None => original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_GET_COLOUR_FAMILY)(colour),
    }
}

/// whether two colours clash: same family or the pairs 8-9, 8-7, 6-7 and family 6 against the colour keyed at
/// [0x9cf1dc]
/// Original: `database_colours_clash` (0x540490, Database.cpp, cdecl).
///
/// [pt] { 0x540490 } (cdecl, Database.cpp): as cores `a` e `b` se confundem? Pelas famílias (0x540520, `a`
/// primeiro): iguais; 8 com 9; 8 com 7; 6 com 7, salvo quando a da família 6 está contra a cor com o id
/// da chave [0x9cf1dc] (exceção: não se confundem).
pub(crate) unsafe extern "C" fn database_colours_clash(a: usize, b: usize) -> u8 {
    let fa = exe::database_get_colour_family(a);
    let fb = exe::database_get_colour_family(b);
    let par = |x: u8, y: u8| (fa == x && fb == y) || (fb == x && fa == y);
    let key = read_u32(KEY_COLOUR_PURPLE_3);
    if fa == fb || par(8, 9) || par(8, 7) {
        return 1;
    }
    // family 6 against the key colour is an exception: they are not confused (the original exits with 0 before the 6/7 pair)
    if (fa == 6 && *(b as *const u32) == key) || (fb == 6 && *(a as *const u32) == key) {
        return 0;
    }
    par(6, 7) as u8
}

/// The coaching `attribute` (1..12) of `person`, from her `non_player` record (+0x69).
///
/// Direct: 1 person +0x56, 4 person +0x58, 7 non-player +0x14, 12 +0x22. The others (2 +0x11, 3 +0x10, 5 +0x17, 6
/// +0x18, 8 +0x19, 9 +0x1b, 10 +0x1e, 11 +0x21) through the 1-to-20 formula of `exe::non_player_attribute_rating`: x = (word +4 + 2 × byte) ×
/// 0.05 + 10; (x + 0.1 x²) / 3 + 0.5, clamped to 1..20 and truncated.
///
/// Null person or non-player, or attribute outside 1..12: the original (error box and −1).
/// Original: `staff_compute_non_player_attribute` (0x543fd0, Database.cpp, cdecl).
///
/// [pt] { 0x543fd0 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`, do `nao_jogador`
/// (+0x69 dela). Diretos: 1 pessoa +0x56, 4 pessoa +0x58, 7 não jogador +0x14, 12 +0x22. Os outros
/// (2 +0x11, 3 +0x10, 5 +0x17, 6 +0x18, 8 +0x19, 9 +0x1b, 10 +0x1e, 11 +0x21) pela fórmula de 0x544410:
/// x = (palavra +4 + 2 × byte) × 0,05 + 10; (x + 0,1 x²) / 3 + 0,5, preso em 1..20 e truncado. Pessoa
/// ou não jogador nulos ou atributo fora de 1..12: a original (caixa e −1).
pub(crate) unsafe extern "C" fn staff_compute_non_player_attribute(person: usize, not_player: usize, attribute: u32) -> u8 {
    let not_player = NonPlayer(not_player);
    let person = Staff(person);
    let a = attribute as u8 as i8;
    if person.is_null() || !(1..=12).contains(&a) || not_player.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_NON_PLAYER_ATTRIBUTE)(person.addr(), not_player.addr(), attribute);
    }
    let formula = |d: usize| {
        let b = read_i8(not_player.addr() + d) as f64;
        let x = (not_player.current_ability() as f64 + (b + b)) * 0.05 + 10.0;
        let y = (x * x * 0.1 + x) * read_f64(rdata::F64_0_3333333333) + 0.5;
        ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y }) as u8
    };
    match a {
        1 => (person.adaptability() as u8),
        2 => formula(0x11),
        3 => formula(0x10),
        4 => (person.determination() as u8),
        5 => formula(0x17),
        6 => formula(0x18),
        7 => not_player.discipline(),
        8 => formula(0x19),
        9 => formula(0x1b),
        10 => formula(0x1e),
        11 => formula(0x21),
        _ => (not_player.youngsters() as u8),
    }
}

/// CRT rand() scaled to n: rand * n / 0x8000 (64-bit arithmetic, low 32 bits).
///
/// [pt] rand() da CRT (0x946a04) escalado a `n`: rand × n / 0x8000. As rotinas de nomes fazem a conta em
/// 64 bits (0x945e40 multiplica, 0x945e80 divide com sinal) e ficam com os 32 bits de baixo.
pub(crate) unsafe fn scaled_crt_rand(n: i32) -> i32 {
    ((exe::crt_rand() as i64 * n as i64) / 0x8000) as i32
}

/// Name count in the global at `a` (first or second names), capped at 0x7fff.
///
/// [pt] O total de nomes da global `a` ([0xae2398] primeiros, [0xae239c] segundos), no máximo 0x7fff.
pub(crate) unsafe fn name_count_capped(a: usize) -> i32 {
    let t = read_i32(a);
    if t > 0x7fff { 0x7fff } else { t }
}

/// Nation's entry in the name index table ([0xae2338] + id x 36: first rows and counts of first/second names).
///
/// [pt] O índice de nomes da nação `n` (id em +0): [0xae2338] + id × 36 (+0 primeiro nome de onde começam os
/// primeiros nomes, +4 o dos segundos, +0x18 quantos primeiros, +0x1c quantos segundos).
pub(crate) unsafe fn nation_name_index(n: usize) -> usize {
    read_usize(G_NATION_NAME_INDEX_TABLE).wrapping_add((read_i32(n) as usize).wrapping_mul(36))
}

/// The nation that lends names to each language group (nation +0x77, 1..32): jump tables 0x53a060 and 0x53a528
/// (identical). Group 17 (0) draws at random between [0x9cf484] and [0x9cf404].
///
/// [pt] A nação que empresta nomes a cada grupo de língua (nação +0x77, 1..32): tabelas de salto 0x53a060 e
/// 0x53a528 (iguais). O grupo 17 (0) sorteia entre [0x9cf484] e [0x9cf404].
pub(crate) const LANGUAGE_GROUP_NATIONS: [usize; 32] = [
    KEY_NATION_ALBANIA, KEY_NATION_SAUDI_ARABIA, KEY_NATION_RUSSIA, KEY_NATION_CHINA, KEY_NATION_CROATIA, KEY_NATION_CZECH_REPUBLIC, KEY_NATION_HOLLAND, KEY_NATION_ENGLAND, KEY_NATION_FINLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY,
    KEY_NATION_GREECE, KEY_NATION_ISRAEL, KEY_NATION_HUNGARY, KEY_NATION_ITALY, KEY_NATION_JAPAN, 0, KEY_NATION_POLAND, KEY_NATION_PORTUGAL, KEY_NATION_ROMANIA, KEY_NATION_RUSSIA, KEY_NATION_NORWAY,
    KEY_NATION_CROATIA, KEY_NATION_SPAIN, KEY_NATION_TURKEY, KEY_NATION_BURMA, KEY_NATION_MALAYSIA, KEY_NATION_PERU, KEY_NATION_SLOVAKIA, KEY_NATION_SWEDEN, KEY_NATION_ICELAND, KEY_NATION_DENMARK,
];

/// For a nation with few names, picks a nation of the same language group with enough names; returns (count, first row)
/// or None to draw from the whole table.
///
/// This is the shared stretch of `database_pick_first_name_for_nation` (0x539df0..) and `database_pick_second_name_for_nation` (0x53a2be..) for a nation with up to 100 names and
/// a language group: the group's nation (table) if it has ≥ random(20) + 15 names in `conta`; otherwise up to 40
/// randomly drawn nations ([0xae2350], 0x122 bytes each at [0xae23a8]) until one of the same group with more than 5
/// first names (+0x18, in both cases) is found.
///
/// Returns (how many in `conta`, first in `inicio`) if the chosen nation has ≥ 5 names; otherwise `None` (draw from the
/// whole table).
///
/// [pt] O trecho comum de 0x539c60 (0x539df0..) e 0x53a1b0 (0x53a2be..) para nação com até 100 nomes e
/// grupo de língua: a nação do grupo (tabela) se ela tem ≥ sorteio(20) + 15 nomes em `conta`; senão até
/// 40 nações sorteadas ([0xae2350], 0x122 bytes cada em [0xae23a8]) até achar uma do mesmo grupo com
/// mais de 5 primeiros nomes (+0x18, nos dois casos). Devolve (quantos em `conta`, primeiro em `inicio`)
/// se a nação escolhida tem ≥ 5 nomes; senão `None` (sorteio na tabela inteira).
pub(crate) unsafe fn name_donor_nation(nation: Nation, count: usize, start: usize) -> Option<(i32, i32)> {
    let g = |a: usize| read_i32(a);
    let nations = read_usize(G_NATION_TABLE);
    let nation_of = |id: i32| nations.wrapping_add((id as usize).wrapping_mul(nation::SIZE));
    let group = nation.first_language();
    let k = (group as i32).wrapping_sub(1) as u32;
    let mut esi = 0usize;
    let mut direct = false;
    if k <= 0x1f {
        let id = if k == 16 {
            if scaled_crt_rand(2) != 0 { g(KEY_NATION_SOUTH_KOREA) } else { g(KEY_NATION_NORTH_KOREA) }
        } else {
            g(LANGUAGE_GROUP_NATIONS[k as usize])
        };
        esi = nation_of(id);
        if esi != 0 {
            let minimum = scaled_crt_rand(0x14).wrapping_add(0xf);
            direct = g(nation_name_index(esi) + count) >= minimum;
        }
    }
    if !direct {
        for _ in 0..40 {
            esi = nation_of(scaled_crt_rand(g(G_NATION_COUNT)));
            if read_i8(esi + nation::FIRST_LANGUAGE) == group && g(nation_name_index(esi) + 0x18) > 5 {
                break;
            }
        }
        if esi == 0 {
            return None;
        }
    }
    let e = nation_name_index(esi);
    let how_many = g(e + count);
    if how_many < 5 {
        return None;
    }
    Some((how_many, g(e + start)))
}

/// Name accepted by `database_pick_first_name_for_nation`'s draw: null/empty yes; a single letter or one containing '.' no.
///
/// [pt] Nome aceito no sorteio de 0x539c60: nulo ou vazio sim; uma letra só ou com ponto não.
pub(crate) unsafe fn is_acceptable_name(name: usize) -> bool {
    if name == 0 || read_u8(name) == 0 {
        return true;
    }
    let s = core::ffi::CStr::from_ptr(name as *const core::ffi::c_char).to_bytes();
    s.len() != 1 && !s.contains(&b'.')
}

/// A first name (60-byte row in [0xae23f0]) for `nation`.
///
/// No nation: a draw over the whole table. A nation with more than 100 first names: 20 draws in its range, the one with
/// the highest weight (+0x3b, signed) among the accepted ones (`is_acceptable_name`) is kept. With up to 100: language
/// group 0 draws over the whole table, but returns the row of the SECOND names table ([0xae23f4], as in the original);
/// otherwise borrows from `name_donor_nation` (the language group). Only the CRT rand() (not `exe::utils_random`).
/// Original: `database_pick_first_name_for_nation` (0x539c60, Database.cpp, cdecl).
///
/// [pt] { 0x539c60 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) para a `nacao`.
/// Sem nação: sorteio na tabela inteira. Nação com mais de 100 primeiros nomes: 20 sorteios na faixa
/// dela, fica o de maior peso (+0x3b, com sinal) entre os aceitos (`is_acceptable_name`). Com até 100: grupo de
/// língua 0 sorteia na tabela inteira, mas devolve a linha da tabela dos SEGUNDOS nomes ([0xae23f4],
/// assim na original); senão `name_donor_nation`. Só o rand() da CRT (não o 0x90cfc0).
pub(crate) unsafe extern "C" fn database_pick_first_name_for_nation(nation: usize) -> usize {
    let nation = Nation(nation);
    let row = |t: usize, i: i32| read_usize(t).wrapping_add((i as usize).wrapping_mul(60));
    if nation.is_null() {
        return row(G_FIRST_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398)));
    }
    let e = nation_name_index(nation.addr());
    let how_many = read_i32(e + nation_name_index::FIRST_NAME_COUNT);
    if how_many > 100 {
        let start = read_i32(e);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 rolls for the first name
            let i = scaled_crt_rand(how_many).wrapping_add(start);
            let name = row(G_FIRST_NAME_TABLE, i);
            let weight = read_i8(name + 0x3b);
            if weight > melhor && is_acceptable_name(name) {
                melhor = weight;
                escolhido = i;
            }
        }
        return row(G_FIRST_NAME_TABLE, escolhido);
    }
    if (nation.first_language() as u8) == 0 {
        return row(G_SECOND_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398)));
    }
    match name_donor_nation(nation, 0x18, 0) {
        Some((n, start)) => row(G_FIRST_NAME_TABLE, scaled_crt_rand(n).wrapping_add(start)),
        None => row(G_FIRST_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398))),
    }
}

/// draws a second name for the nation, the same way
/// Original: `database_pick_second_name_for_nation` (0x53a1b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a1b0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) para a `nacao`.
/// Como `database_pick_first_name_for_nation` com os segundos nomes (+4, +0x1c, [0xae239c]), sem filtrar os nomes
/// no sorteio de 20 (só o maior peso).
pub(crate) unsafe extern "C" fn database_pick_second_name_for_nation(nation: usize) -> usize {
    let nation = Nation(nation);
    let row = |i: i32| read_usize(G_SECOND_NAME_TABLE).wrapping_add((i as usize).wrapping_mul(60));
    if nation.is_null() {
        return row(scaled_crt_rand(name_count_capped(0xae239c)));
    }
    let e = nation_name_index(nation.addr());
    let how_many = read_i32(e + nation_name_index::SECOND_NAME_COUNT);
    if how_many > 100 {
        let start = read_i32(e + nation_name_index::SECOND_NAME_START);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 random rolls of the second name
            let i = scaled_crt_rand(how_many).wrapping_add(start);
            let weight = read_i8(row(i) + 0x3b);
            if weight > melhor {
                melhor = weight;
                escolhido = i;
            }
        }
        return row(escolhido);
    }
    if (nation.first_language() as u8) == 0 {
        return row(scaled_crt_rand(name_count_capped(0xae239c)));
    }
    match name_donor_nation(nation, 0x1c, 4) {
        Some((n, start)) => row(scaled_crt_rand(n).wrapping_add(start)),
        None => row(scaled_crt_rand(name_count_capped(0xae239c))),
    }
}

/// Text pointer is non-null and the text is not empty.
///
/// [pt] Texto não nulo e não vazio.
pub(crate) unsafe fn has_text(p: usize) -> bool {
    p != 0 && *(p as *const u8) != 0
}

/// strcpy of the formatted name into destino.
///
/// [pt] strcpy (a original copia com rep movsd/movsb o texto e o zero final).
pub(crate) unsafe fn copy_formatted_name(target: usize, source: usize) {
    let mut i = 0;
    loop {
        let b = read_u8(source + i);
        write_u8(target + i, b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Writes the name of `person` into `dest` in `style` (byte, 1..7), with first name (+4), surname (+8) and common name
/// (+0xc, or the nickname given by the human: `has_nickname`/`exe::human_manager_get_nickname` on [0xb63c98]).
///
/// 1 "first surname", 2 "F.surname", 3 "surname, first", 4 "surname, F", 5 a single name, 6 initials, 7 like 1 without
/// preferring the common name; always with fallbacks (whatever there is). Returns 1.
///
/// Null person or destination, size (i16) < 0x69 or style outside 1..7: the original (error box and 0).
/// Original: `database_format_staff_name` (0x53dba0, Database.cpp, cdecl).
///
/// [pt] { 0x53dba0 } (cdecl, Database.cpp): o nome da `pessoa` em `destino` no `estilo` (byte, 1..7), com
/// nome (+4), sobrenome (+8) e nome comum (+0xc, ou o apelido dado pelo humano: 0x600740/0x6005e0 em
/// [0xb63c98]). 1 "nome sobrenome", 2 "N.sobrenome", 3 "sobrenome, nome", 4 "sobrenome, N", 5 só um
/// nome, 6 iniciais, 7 como o 1 sem preferir o comum; sempre com recuos (o que houver). Devolve 1.
/// Pessoa ou destino nulos, tamanho (i16) < 0x69 ou estilo fora de 1..7: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_format_staff_name(person: usize, target: usize, size: u32, style: u32) -> u32 {
    let e = style as u8;
    if person == 0 || target == 0 || (size as i16) < 0x69 || !(1..=7).contains(&e) {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_STAFF_NAME)(person, target, size, style);
    }
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (nome, sobrenome) = (read_usize(person + 4), read_usize(person + 8));
    let apelidos = read_usize(G_HUMAN_MANAGER);
    let common = if apelidos != 0 && exe::human_manager_has_nickname(apelidos, person, (0) as u32) != 0 { exe::human_manager_get_nickname(apelidos, person, (0) as u32) } else { read_usize(person + 0xc) };
    write_u8(target, 0);
    let (c, n, s) = (has_text(common), has_text(nome), has_text(sobrenome));
    let letra = |p: usize| read_i8(p) as i32;
    let copy = |p: usize| copy_formatted_name(target, p);
    // the original’s fallbacks test the common name again after the surname: no effect (it was already empty)
    match e {
        1 | 7 => {
            if e == 1 && c {
                copy(common);
            } else if n && s {
                fmt(target, 0x98f0b8, nome, sobrenome); // "%s %s"
            } else if s {
                copy(sobrenome);
            } else if c {
                copy(common);
            } else if n {
                copy(nome);
            }
        }
        2 | 3 | 4 => {
            if c {
                copy(common);
            } else if n && s {
                match e {
                    2 => fmt(target, 0x9b7410, letra(nome), sobrenome), // "%c.%s"
                    3 => fmt(target, 0x98f6ac, sobrenome, nome),        // "%s, %s"
                    _ => fmt(target, 0x9b7408, sobrenome, letra(nome)), // "%s, %c"
                };
            } else if s {
                copy(sobrenome);
            } else if n {
                copy(nome);
            }
        }
        5 => {
            if c {
                copy(common);
            } else if s {
                copy(sobrenome);
            } else if n {
                copy(nome);
            }
        }
        _ => {
            // 6: iniciais
            if c {
                fmt(target, 0x99ec1c, letra(common)); // "%c"
            } else if n && s {
                fmt(target, 0x9b7400, letra(nome), letra(sobrenome)); // "%c%c"
            } else if s {
                fmt(target, 0x99ec1c, letra(sobrenome));
            } else if n {
                fmt(target, 0x99ec1c, letra(nome));
            }
        }
    }
    1
}

/// The colours of `kit` (byte: 1 home +0x83/+0x87, 2 away +0x8b/+0x8f, 3 third +0x93/+0x97; 2 and 3 with one of the two
/// colours missing: the home ones swapped) of `club` as screen pixels (`exe::graphics_rgb_to_pixel` with bytes +0x37, +0x38, +0x39 of the
/// colour and format 0): `foreground` and `background`, which start with [0xaebdf0] and [0xae3184].
///
/// Any other kit: only the defaults. Returns 1. Null club: the original (defaults, "Error" box and 0).
/// Original: `database_get_club_kit_colours` (0x53b640, Database.cpp, cdecl).
///
/// [pt] { 0x53b640 } (cdecl, Database.cpp): as cores do `uniforme` (byte: 1 casa +0x83/+0x87, 2 fora
/// +0x8b/+0x8f, 3 terceiro +0x93/+0x97; 2 e 3 sem uma das duas cores: as de casa trocadas) do `clube`
/// em pixels (0x5e4800 com os bytes +0x37, +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que
/// começam com [0xaebdf0] e [0xae3184]. Outro uniforme: só os padrões. Devolve 1. Clube nulo: a
/// original (padrões, caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_club_kit_colours(club: usize, kit: u32, front: *mut u16, bottom: *mut u16) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_CLUB_KIT_COLOURS)(club, kit, front, bottom);
    }
    // simplification: the original pushes dwords with only the low byte written (the rest is stack garbage);
    // 0x5e4800 does & 0xff on all three, so the extended bytes give the same pixel
    core::ptr::write_unaligned(front, read_u16(G_GUI_COLOUR_BLUE));
    core::ptr::write_unaligned(bottom, read_u16(G_GUI_COLOUR_PALE_GREY));
    let (a, b) = match kit as u8 {
        1 => (read_usize(club + 0x83), read_usize(club + 0x87)), // casa
        2 | 3 => {
            let d = if kit as u8 == 2 { 0x8b } else { 0x93 };
            let (a, b) = (read_usize(club + d), read_usize(club + (d + 4)));
            if a == 0 || b == 0 { (read_usize(club + 0x87), read_usize(club + 0x83)) } else { (a, b) }
        }
        _ => return 1,
    };
    let pixel = |c: usize| {
        exe::graphics_rgb_to_pixel((read_u8(c + 0x37) as u32), (read_u8(c + 0x38) as u32), (read_u8(c + 0x39) as u32), (0) as u32) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(front, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(bottom, pixel(b));
    }
    1
}

/// The manager (+0xcf) of `club`; without one, that of the A/B partner (`exe::club_get_linked_b_team` with 0 and 1); without either, an
/// ordinary club gives 0 and a national team (id ≥ [0xae235c] − [0xae2350]) repeats with its nation's national team
/// (`exe::database_get_national_team` with +0x53: the under-21 side reaches the senior one).
///
/// Null club (at the start or coming from `exe::database_get_national_team`): the original with null ("Error" box and 0).
/// Original: `database_get_club_manager` (0x53ca90, Database.cpp, cdecl).
///
/// [pt] { 0x53ca90 } (cdecl, Database.cpp): o técnico (+0xcf) do `clube`; sem ele, o do parceiro A/B
/// (0x540a50 com 0 e 1); sem os dois, clube comum dá 0 e seleção (id ≥ [0xae235c] − [0xae2350]) repete
/// com a seleção da nação dela (0x53b3d0 com +0x53, a sub-21 chega na principal). Clube nulo (no começo
/// ou vindo da 0x53b3d0): a original com nulo (caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_club_manager(club: usize) -> usize {
    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let manager = |c: Club| c.manager().addr();
    let mut c = club;
    loop {
        if c == 0 {
            return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_MANAGER)(0);
        }
        let t = manager(Club(c));
        if t != 0 {
            return t; // the club’s manager
        }
        let p = partner(c, 0, 1);
        if p != 0 && manager(Club(p)) != 0 {
            return manager(Club(p));
        }
        if read_i32(c) < read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT)) {
            return 0;
        }
        c = exe::database_get_national_team(read_usize(c + club::NATION));
    }
}

/// The name of `club` in English: its own name, or its English one from eng.lng when another language is loaded.
///
/// Language ([0x9b5984]) 0 or 1: its own name (+4). Otherwise the eng.lng table ([0xae2408], 79-byte rows from +0x20):
/// through the id map [0xae23fc] if present, otherwise by id if < the count (+0), otherwise 0.
/// Original: `club_get_english_name` (0x545880, Database.cpp, cdecl).
///
/// [pt] { 0x545880 } (cdecl, Database.cpp): o nome do `clube` em inglês. Idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20): pelo mapa
/// de ids [0xae23fc] se houver, senão pelo id se < quantos (+0), senão 0.
pub(crate) unsafe extern "C" fn club_get_english_name(club: usize) -> usize {
    let language = read_u32(G_DATABASE_NAMES_LANGUAGE);
    if language == 0 || language == 1 {
        return club.wrapping_add(4); // language 0 or 1
    }
    let t = read_usize(G_ENGLISH_NAME_TABLES);
    if t == 0 {
        // simplification: the first use allocates (new 0x40) and loads eng.lng (0x544fc0) under the SEH frame: the original routine
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_NAME)(club);
    }
    let rows = read_usize(t + english_name_tables::CLUB_ROWS);
    let id = read_i32(club);
    let map = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    if map != 0 {
        let i = read_i32(map.wrapping_add((id as usize).wrapping_mul(4)));
        return rows.wrapping_add((i as usize).wrapping_mul(79));
    }
    if id < read_i32(t) {
        rows.wrapping_add((id as usize).wrapping_mul(79))
    } else {
        0
    }
}

/// Adds `delta` (signed byte) to the morale (+0x45) of the player record (person +0x61), damped near the ends; returns
/// whether it changed.
///
/// Going up above 10, if random(21) < morale + delta, adds only random(delta + 1); going down below 10, if
/// random((player +0x2f + person +0x5a + person +0x58) / 3) > morale + delta, adds random(delta − 1). Clamps to 1..20
/// (signed byte); with `limit` and delta > 0, the link with the club (`exe::contract_manager_get_contract_for_club`, person +0x39) with +0x45 not null
/// prevents going up. Returns 1 if the morale changed. Delta 0: 0, without reading anything.
/// Original: `staff_adjust_morale` (0x544710, Database.cpp, cdecl).
///
/// [pt] { 0x544710 } (cdecl, Database.cpp): soma `delta` (byte com sinal) ao moral (+0x45) do jogador
/// (pessoa +0x61), amortecido perto das pontas: subindo acima de 10, se exe::utils_random(21) < moral + delta
/// soma só exe::utils_random(delta + 1); descendo abaixo de 10, se exe::utils_random((jogador +0x2f + pessoa +0x5a +
/// pessoa +0x58) / 3) > moral + delta soma exe::utils_random(delta − 1). Prende em 1..20 (byte com sinal); com
/// `limitar` e delta > 0, o vínculo com o clube (0x4cf5f0, pessoa +0x39) com +0x45 não nulo impede
/// subir. Devolve 1 se o moral mudou. Delta 0: 0, sem ler nada.
pub(crate) unsafe extern "C" fn staff_adjust_morale(person: usize, delta: u32, limitar: u32) -> u32 {
    let person = Staff(person);
    let d = delta as u8 as i8;
    if d == 0 {
        return 0;
    }
    let player = person.player();
    let morale = player.player_morale();
    let sum = morale as i32 + d as i32;
    let simples = morale.wrapping_add(d);
    let mut new = if d > 0 {
        if sum > 10 && exe::utils_random(0x15) < sum { morale.wrapping_add(exe::utils_random(d as i32 + 1) as i8) } else { simples }
    } else if sum < 10 {
        let b = |a: usize| read_i8(a) as i32;
        let ceiling = (b(player.addr() + player_sheet::LEADERSHIP) + b(person.addr() + staff::PRESSURE) + b(person.addr() + staff::DETERMINATION)) / 3;
        if exe::utils_random(ceiling) > sum { morale.wrapping_add(exe::utils_random(d as i32 - 1) as i8) } else { simples }
    } else {
        simples
    };
    new = if new < 1 { 1 } else if new > 20 { 20 } else { new };
    if limitar != 0 && d > 0 {
        let v = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, person.club_job());
        if v != 0 && read_u32(v + 0x45) != 0 && new >= morale {
            new = morale;
        }
    }
    player.set_player_morale(new);
    (new != morale) as u32
}

/// The non-player's bytes (+0x0e..+0x22) in the order of the 21 cells of `database_non_player_to_attribute_vector`'s array (13 before the cut, 8 after).
///
/// [pt] Os bytes do não jogador (+0x0e..+0x22) na ordem das 21 células do vetor de 0x53a8b0 (13 antes do
/// corte, 8 depois).
pub(crate) const NON_PLAYER_ATTRIBUTE_ORDER: [usize; 21] = [
    0x0e, 0x0f, 0x12, 0x13, 0x14, 0x15, 0x16, 0x1a, 0x1c, 0x1d, 0x1f, 0x20, 0x22, 0x11, 0x10, 0x17, 0x18, 0x19, 0x1b,
    0x1e, 0x21,
];

/// Widens (movsx) the attributes of `non_player` (coaching attributes +0x0e..+0x22) into the low 16 bits of the 21
/// three-byte cells of `vector` (the 3rd byte of each stays as it is) and writes 13 (the split) into `end` if not null.
///
/// The person is not used. eax on exit: the high 16 bits of the non-player pointer and the last widened attribute
/// (callers do not use it).
/// Original: `database_non_player_to_attribute_vector` (0x53a8b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a8b0 } (cdecl, Database.cpp): estende (movsx) os atributos do `nao_jogador` para os 16 bits de
/// baixo das 21 células de 3 bytes do `vetor` (o 3º byte de cada uma fica como está) e grava 13 (o
/// corte) em `fim` se não nulo. A pessoa não é usada. eax na saída: os 16 bits altos do ponteiro do não
/// jogador e o último atributo estendido (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_non_player_to_attribute_vector(not_player: usize, person: usize, array: usize, end: *mut u8) -> u32 {
    let mut last = 0u16;
    for (k, &d) in NON_PLAYER_ATTRIBUTE_ORDER.iter().enumerate() {
        if k == 13 && !end.is_null() {
            *end = 0xd;
        }
        last = read_i8(not_player + d) as i16 as u16;
        write_u16(array + 3 * k, last);
    }
    (not_player as u32 & 0xffff_0000) | last as u32
}

/// Sets `squad` (byte) in the person's squad flags (+0x5e): with club bits (& 3) keeps the national team ones (& 0xc),
/// otherwise keeps the club ones; then adds.
///
/// The original writes twice (the mask and the result). Returns the person (eax).
/// Original: `staff_set_squad_flags` (0x540980, Database.cpp, cdecl).
///
/// [pt] { 0x540980 } (cdecl, Database.cpp): o `elenco` (byte) nas marcas de elenco da pessoa (+0x5e): com
/// bits de clube (& 3) mantém os de seleção (& 0xc), senão mantém os de clube; depois acrescenta. A
/// original grava duas vezes (a máscara e o resultado). Devolve a pessoa (eax).
pub(crate) unsafe extern "C" fn staff_set_squad_flags(person: usize, squad: u32) -> usize {
    let person = Staff(person);
    let e = squad as u8;
    let p = (person.addr() + staff::PLAYING_SQUAD) as *mut u8;
    let mantem = if e & 3 != 0 { *p & 0xc } else { *p & 3 };
    *p = mantem;
    *p = mantem | e;
    person.addr()
}

/// qsort comparison of the competition history rows.
///
/// A row without a competition (+4) goes last (a without and b with: 1; a with and b without: −1; both without: 0);
/// then by the competition id (a − b); within the same one, by season (+8, word) from newest to oldest (b − a), unless
/// one of them is −1 (0).
/// Original: `database_compare_comp_history_rows` (0x540780, Database.cpp, cdecl).
///
/// [pt] { 0x540780 } (cdecl, Database.cpp): comparação do qsort das linhas do histórico de competições.
/// Linha sem competição (+4) vai para o fim (a sem e b com: 1; a com e b sem: −1; as duas sem: 0);
/// depois pelo id da competição (a − b); na mesma, pela temporada (+8, palavra) da mais nova para a
/// mais velha (b − a), a não ser que uma delas seja −1 (0).
pub(crate) unsafe extern "C" fn database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let ca = read_usize(a + 4);
    if ca == 0 {
        return (read_usize(b + 4) != 0) as i32;
    }
    let cb = read_usize(b + 4);
    if cb == 0 {
        return -1;
    }
    let d = read_i32(ca).wrapping_sub(read_i32(cb));
    if d != 0 {
        return d; // by competition id
    }
    let ta = read_i16(a + 8);
    let tb = read_i16(b + 8);
    if ta == -1 || tb == -1 {
        return 0;
    }
    tb as i32 - ta as i32 // the newest season first
}

/// As 54 primeiras células do vetor de atributos (`database_player_to_attribute_vector`), na ordem em que a original grava:
/// (deslocamento no registro de jogador, peso). Peso 0 = o zero do começo.
pub(crate) const ATTRIBUTE_VECTOR_CELLS: [(u8, u8); 54] = [
    (0x0f, 0), (0x10, 0), (0x11, 0), (0x12, 0), (0x13, 0), (0x14, 0), (0x15, 0), (0x16, 0), (0x1a, 0),
    (0x17, 0), (0x18, 0), (0x19, 0),
    (0x30, 0), (0x3b, 0), (0x1e, 0), (0x23, 2), (0x24, 0), (0x26, 2), (0x27, 2), (0x2a, 1), (0x2b, 2),
    (0x31, 2), (0x32, 2), (0x33, 2), (0x35, 0), (0x37, 2), (0x38, 2), (0x39, 0), (0x3a, 0), (0x3e, 2),
    (0x41, 0), (0x43, 2),
    (0x28, 2), (0x22, 2), (0x29, 0), (0x40, 2), (0x3d, 0), (0x3c, 0), (0x36, 0), (0x2e, 0), (0x2f, 0),
    (0x25, 0), (0x1f, 0), (0x20, 0), (0x21, 0), (0x1b, 2), (0x1c, 0), (0x1d, 0), (0x2c, 0), (0x2d, 0),
    (0x42, 0), (0x3f, 2), (0x44, 2), (0x34, 0),
];

/// Widens the bytes of the `player` record into a vector of 63 three-byte cells (signed word + weight).
///
/// Zeroes the 63 weights; cells 0..53 by the table `ATTRIBUTE_VECTOR_CELLS` (positions 0..8, sides 9..11, then the
/// attributes, weight 2 on those that count and 1 on +0x2a); cell 54 is byte +0x58 of `person`. The group ends go to
/// the non-null outputs: positions 9, sides 0xc, weighted 0x20. Returns the vector (eax).
/// Original: `database_player_to_attribute_vector` (0x53a610, Database.cpp, cdecl).
///
/// [pt] { 0x53a610 } (cdecl, Database.cpp): estende os bytes do registro de `jogador` num vetor de 63
/// células de 3 bytes (palavra com sinal + peso): zera os 63 pesos; células 0..53 pela tabela
/// `ATTRIBUTE_VECTOR_CELLS` (posições 0..8, lados 9..11, depois os atributos, peso 2 nos que contam e 1 no
/// +0x2a); a célula 54 é o byte +0x58 da `pessoa`. Os fins de grupo vão às saídas não nulas:
/// posições 9, lados 0xc, ponderados 0x20. Devolve o vetor (eax).
pub(crate) unsafe extern "C" fn database_player_to_attribute_vector(player: usize, person: usize, array: usize, end_of_sides: *mut u8,
                                                   end_of_positions: *mut u8, end_of_the_weighted: *mut u8) -> usize {
    let person = Staff(person);
    for i in 0..0x3f {
        write_u8(array + 3 * i + 2, 0);
    }
    for (i, &(d, peso)) in ATTRIBUTE_VECTOR_CELLS.iter().enumerate() {
        write_i16(array + 3 * i, read_i8(player + d as usize) as i16);
        write_u8(array + 3 * i + 2, peso);
        match i {
            8 if !end_of_positions.is_null() => *end_of_positions = 9,
            11 if !end_of_sides.is_null() => *end_of_sides = 0xc,
            31 if !end_of_the_weighted.is_null() => *end_of_the_weighted = 0x20,
            _ => {}
        }
    }
    write_i16(array + 3 * 54, person.determination() as i16);
    array
}

/// whether the nation is England, Scotland, Ireland, Wales or Northern Ireland
/// Original: `nation_is_uk_or_ireland` (0x5446d0, Database.cpp, cdecl).
///
/// [pt] { 0x5446d0 } (cdecl, Database.cpp): 1 se a `nacao` (não nula) é Inglaterra, Escócia, Irlanda, País
/// de Gales ou Irlanda do Norte (a regra das nações da casa).
pub(crate) unsafe extern "C" fn nation_is_uk_or_ireland(nation: usize) -> u32 {
    let g = |a: usize| read_u32(a);
    let id = g(nation);
    (id == g(KEY_NATION_ENGLAND) || id == g(KEY_NATION_SCOTLAND) || id == g(KEY_NATION_REPUBLIC_OF_IRELAND) || id == g(KEY_NATION_WALES) || id == g(KEY_NATION_NORTHERN_IRELAND)) as u32
}

/// Os 23 clubes bascos de `database_is_basque_club` (ponteiros fixos da área 0x9d00a4..), na ordem da original.
pub(crate) const BASQUE_CLUBS: [usize; 23] = [
    0x9d0120, 0x9d0134, 0x9d02e4, 0x9d02e8, 0x9d02ec, 0x9d02f0, 0x9d02f4, 0x9d02f8, 0x9d02fc, 0x9d0304, 0x9d0308,
    0x9d030c, 0x9d0310, 0x9d0314, 0x9d0318, 0x9d031c, 0x9d0320, 0x9d0324, 0x9d0328, 0x9d032c, 0x9d0330, 0x9d0334,
    0x9d0338,
];

/// whether the club is Spanish and one of the 23 Basque clubs named in the code
/// Original: `database_is_basque_club` (0x53d7f0, Database.cpp, cdecl).
///
/// [pt] { 0x53d7f0 } (cdecl, Database.cpp): 1 se o `clube` é da Espanha ([0x9cf488], nação +0x53) e o id
/// dele é um dos 23 clubes bascos (`BASQUE_CLUBS`). Clube ou nação nulos: 0.
pub(crate) unsafe extern "C" fn database_is_basque_club(club: usize) -> u32 {
    let club = Club(club);
    let g = |a: usize| read_u32(a);
    if club.is_null() {
        return 0;
    }
    let nation = club.nation().addr();
    if nation == 0 || g(nation) != g(KEY_NATION_SPAIN) {
        return 0; // outside Spain
    }
    let id = g(club.addr());
    BASQUE_CLUBS.iter().any(|&a| id == g(a)) as u32
}

/// Contract link (`exe::contract_manager_get_contract_for_club`) of the person with the club; with no club given, uses the club of the person's contract
/// (stored in G_CONTRACT_LOOKUP_RESULT); returns (link, club) or (0, 0).
///
/// [pt] O vínculo (0x4cf5f0) da `pessoa` (não nula) com o `clube` e o clube usado: sem clube, o do contrato
/// (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
/// Sem contrato ou clube: (0, 0). Comum a 0x5444a0 e 0x544590 (e 0x544520, 0x544600).
pub(crate) unsafe fn contract_link_for_club(person: usize, club: usize) -> (usize, usize) {
    let mut club = club;
    if club == 0 {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        if c.is_null() {
            return (0, 0);
        }
        let id = (c.club() as u32) as usize;
        club = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(581));
        if club == 0 {
            return (0, 0);
        }
    }
    (exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club), club)
}

/// whether the person's link with the club (or the contract club) is transfer listed by the club; job 0xf at a club
/// without a human manager does not count
/// Original: `staff_is_transfer_listed` (0x5444a0, Database.cpp, cdecl).
///
/// [pt] { 0x5444a0 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de transferências pelo clube (vínculo +0x4e, bit 1), a não ser que o clube
/// não seja de técnico humano (0x600a50 com 1, 1, 0, 0) e a função no clube da pessoa (+0x3d) seja 0xf.
pub(crate) unsafe extern "C" fn staff_is_transfer_listed(person: usize, club: usize) -> u32 {
    let person = Staff(person);
    if person.is_null() {
        return 0;
    }
    let (v, club) = contract_link_for_club(person.addr(), club);
    if v == 0 || read_u8(v + contract::TRANSFER_STATUS) & 1 == 0 {
        return 0;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && person.job_for_club() == 0xf {
        return 0;
    }
    1
}

/// whether the person's link with the club (or the contract club) is loan listed
/// Original: `staff_is_loan_listed` (0x544590, Database.cpp, cdecl).
///
/// [pt] { 0x544590 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de empréstimos (vínculo +0x4e, bit 2).
pub(crate) unsafe extern "C" fn staff_is_loan_listed(person: usize, club: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let (v, _) = contract_link_for_club(person, club);
    (v != 0 && *((v + contract::TRANSFER_STATUS) as *const u8) & 2 != 0) as u32
}

/// 1 if the link (`exe::contract_manager_get_contract_for_club`) of `person` with `club` is transfer listed at request (link +0x4e, bit 8).
///
/// Without a club, the contract's club (`exe::contract_manager_get_contract`, stored in [0xae1080]; contract +4 = id in table [0xae23b4], 581 bytes
/// per club).
/// Original: `staff_is_listed_by_request` (0x544600, Database.cpp, cdecl).
///
/// [pt] { 0x544600 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências a pedido (vínculo +0x4e, bit 8). Sem clube, o do contrato (0x4cf5a0,
/// guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
pub(crate) unsafe extern "C" fn staff_is_listed_by_request(person: usize, club: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let mut club = club;
    if club == 0 {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        if c.is_null() {
            return 0;
        }
        let id = (c.club() as u32) as usize;
        club = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(581));
        if club == 0 {
            return 0;
        }
    }
    let v = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
    (!v.is_null() && v.transfer_status() & 8 != 0) as u32
}

/// the 1 to 20 rating of 2 × the attribute byte + the player's current ability
/// Original: `player_attribute_rating` (0x544380, Database.cpp, cdecl).
///
/// [pt] { 0x544380 } (cdecl, Database.cpp): a nota de 1 a 20 (`attribute_rating`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador`. Termina no `_ftol` (eax inteiro, 1..20).
pub(crate) unsafe extern "C" fn player_attribute_rating(player: usize, value: u32) -> u32 {
    let ability = read_i16(player + 5) as i32;
    attribute_rating((ability + 2 * (value as u8 as i8 as i32)) as f64) as u32
}

/// 1 if `club` may have a reserve side.
///
/// Not: national teams (id ≥ [0xae235c] − [0xae2350]), clubs with an A/B partner (`exe::club_get_linked_b_team` with null and 1); in the
/// range of the second national sides (id ≥ [0xae235c] − 2 × [0xae2350]), nation Spain, Norway, [0x9cf360], Brazil or
/// Argentina, or continent (+0x71) [0x9cfa18] or [0x9cfa08]; in the others, nation Spain, Germany or Portugal. Null
/// club: "Error" box and 0 (the original).
/// Original: `database_club_has_reserve_squad` (0x542740, Database.cpp, cdecl).
///
/// [pt] { 0x542740 } (cdecl, Database.cpp): 1 se o `clube` pode ter time reserva. Não: seleções (id ≥
/// [0xae235c] − [0xae2350]), clubes com parceiro A/B (0x540a50 com nulo e 1); na faixa das segundas
/// seleções (id ≥ [0xae235c] − 2 × [0xae2350]), nação Espanha, Noruega, [0x9cf360], Brasil ou
/// Argentina ou continente (+0x71) [0x9cfa18] ou [0x9cfa08]; nos demais, nação Espanha, Alemanha ou
/// Portugal. Clube nulo: caixa "Error" e 0 (a original).
pub(crate) unsafe extern "C" fn database_club_has_reserve_squad(club: usize) -> u32 {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_HAS_RESERVE_SQUAD)(club.addr());
    }
    let partner: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let g = |a: usize| read_u32(a);
    let gi = |a: usize| read_i32(a);
    let id = gi(club.addr());
    if id >= gi(G_CLUB_COUNT).wrapping_sub(gi(G_NATION_COUNT)) {
        return 0; // national team
    }
    if partner(club.addr(), 0, 1) != 0 {
        return 0;
    }
    let nation = club.nation();
    if nation.is_null() {
        return 1;
    }
    let n = g(nation.addr());
    if id >= gi(G_CLUB_COUNT).wrapping_sub(gi(G_NATION_COUNT).wrapping_mul(2)) {
        if [KEY_NATION_SPAIN, KEY_NATION_NORWAY, KEY_NATION_ITALY, KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA].iter().any(|&a| n == g(a)) {
            return 0;
        }
        let continent = nation.continent();
        if continent != 0 && (g(continent) == g(KEY_CONTINENT_OCEANIA) || g(continent) == g(KEY_CONTINENT_AFRICA)) {
            return 0;
        }
        return 1;
    }
    if n == g(KEY_NATION_SPAIN) || n == g(KEY_NATION_GERMANY) || n == g(KEY_NATION_PORTUGAL) {
        return 0; // Espanha, Alemanha, Portugal
    }
    1 // pode
}

/// How far `viewer` stands from `person`, which picks the reputation.
///
/// 0 = same club or clubs of the same nation (+0x39, club +0x53) → the home one; 2 = same nation (+0x1a) or not foreign
/// in it (`database_is_staff_foreign`) → the current one; 3 = not foreign in the nation of the person's club, or player reputation (+0x61,
/// +9) + reputation of the viewer's club (+0x80) ≥ 14000 → the average; 4 (no viewer, viewer among the last 16 ids =
/// human managers, no data or no club) → the world one.
///
/// Only al is returned (callers use al).
/// Original: `database_get_reputation_scope` (0x5408c0, Database.cpp, cdecl).
///
/// [pt] { 0x5408c0 } (cdecl, Database.cpp): de que distância `quem_ve` olha a `pessoa`, o que escolhe a
/// reputação: 0 = mesmo clube ou clubes da mesma nação (+0x39, clube +0x53) → a nacional; 2 = mesma
/// nação (+0x1a) ou não estrangeiro nela (0x53d980) → a atual; 3 = não estrangeiro na nação do clube
/// da pessoa, ou reputação de jogador (+0x61, +9) + reputação do clube de quem vê (+0x80) ≥ 14000 →
/// a média; 4 (sem quem vê, quem vê entre os 16 últimos ids = técnicos humanos, sem dados ou sem
/// clube) → a mundial. Só o al é devolvido (quem chama usa al).
pub(crate) unsafe extern "C" fn database_get_reputation_scope(person: usize, who_ve: usize) -> u8 {
    let who_ve = Staff(who_ve);
    let person = Staff(person);
    if who_ve.is_null() {
        return 4;
    }
    let club = person.club_job();
    if !club.is_null() {
        if club == who_ve.club_job() {
            return 0;
        }
        let other = who_ve.club_job();
        if !other.is_null() && club.nation() == other.nation() {
            return 0;
        }
    }
    let nation = person.nation();
    if nation == who_ve.nation() {
        return 2; // same nation
    }
    if exe::database_is_staff_foreign(who_ve, nation, 1) == 0 {
        return 2;
    }
    let club = person.club_job();
    if !club.is_null() && exe::database_is_staff_foreign(who_ve, club.nation().addr(), 1) == 0 {
        return 3;
    }
    if read_i32(who_ve.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(16) {
        return 4;
    }
    let player = person.player();
    if player.is_null() {
        return 4;
    }
    let club_of_who_ve = who_ve.club_job();
    if club_of_who_ve.is_null() {
        return 4;
    }
    let sum = read_i16(player.addr() + 9) as i32 + club_of_who_ve.reputation() as i32;
    if sum >= 0x36b0 {
        3
    } else {
        4
    }
}

/// The reputation of `person` as seen by `viewer`, by the scope (`database_get_reputation_scope`): from the player data (+0x61: current +9,
/// home +0xb, world +0xd) or, with `non_player`, from the non-player data (+0x69: +8, +0xa, +0xc).
///
/// 4 → world, 2 → current, 0 → home, 3 → half the world + half the current, other → half the home + half the current
/// (truncating divisions). Only ax is returned.
/// Original: `database_get_staff_reputation_for_viewer` (0x5407e0, Database.cpp, cdecl).
///
/// [pt] { 0x5407e0 } (cdecl, Database.cpp): a reputação da `pessoa` vista por `quem_ve`, pelo alcance
/// (0x5408c0): dos dados de jogador (+0x61: atual +9, nacional +0xb, mundial +0xd) ou, com
/// `nao_jogador`, dos de não jogador (+0x69: +8, +0xa, +0xc). 4 → mundial, 2 → atual, 0 → nacional,
/// 3 → metade da mundial + metade da atual, outro → metade da nacional + metade da atual (divisões
/// truncadas). Só o ax é devolvido.
pub(crate) unsafe extern "C" fn database_get_staff_reputation_for_viewer(person: usize, who_ve: usize, not_player: u32) -> u16 {
    let person = Staff(person);
    let e = exe::database_get_reputation_scope(person, who_ve);
    let (dados, atual) = if not_player != 0 {
        (person.non_player().addr(), 8)
    } else {
        (person.player().addr(), 9)
    };
    // r(0) atual, r(2) nacional, r(4) mundial
    let r = |d: usize| read_i16(dados + atual + d) as i32;
    (match e {
        4 => r(4),
        2 => r(0), // escopo 2: a atual
        0 => r(2),
        3 => r(4) / 2 + r(0) / 2,
        _ => r(2) / 2 + r(0) / 2,
    }) as u16
}

/// 1 if `person` is foreign in `nation` (null: 1).
///
/// Not foreign with the nation as first (+0x1a) or second nationality (+0x1e). With `groups`, also not foreign: among
/// England, Scotland, Wales, Northern Ireland and Ireland (British Isles); between Australia and [0x9cf3f4]; in
/// [0x9cf360] with the nation [0x9cf4f0]; and in Portugal someone who is not from the EU (`database_is_staff_eu_national`) with a contract
/// (`exe::contract_manager_get_contract`) that started up to 1/1/1998 (Date `exe::date_date_549f70`; contract +0x25 day, +0x27 year).
/// Original: `database_is_staff_foreign` (0x53d980, Database.cpp, cdecl).
///
/// [pt] { 0x53d980 } (cdecl, Database.cpp): 1 se a `pessoa` é estrangeira na `nacao` (nula: 1). Não é com
/// a nação na primeira (+0x1a) ou segunda nacionalidade (+0x1e). Com `grupos`, também não é: entre
/// Inglaterra, Escócia, País de Gales, Irlanda do Norte e Irlanda; entre Austrália e [0x9cf3f4];
/// em [0x9cf360] com a nação [0x9cf4f0]; e em Portugal quem não é da UE (0x53d740) com contrato
/// (0x4cf5a0) que começou até 1/1/1998 (Date 0x549f70; contrato +0x25 dia, +0x27 ano).
pub(crate) unsafe extern "C" fn database_is_staff_foreign(person: usize, nation: usize, groups: u32) -> u32 {
    let person = Staff(person);
    let g = |a: usize| read_u32(a);
    if nation == 0 {
        return 1;
    }
    let n1 = person.nation();
    if n1 == Nation(nation) {
        return 0;
    }
    let n2 = person.second_nation();
    if n2 == Nation(nation) {
        return 0;
    }
    if groups == 0 {
        return 1;
    }
    let id = g(nation);
    // England, Scotland, Wales, Northern Ireland and Ireland
    let britanica = |x: u32| x == g(KEY_NATION_ENGLAND) || x == g(KEY_NATION_SCOTLAND) || x == g(KEY_NATION_WALES) || x == g(KEY_NATION_REPUBLIC_OF_IRELAND) || x == g(KEY_NATION_NORTHERN_IRELAND);
    if britanica(id) {
        if britanica(g(n1.addr())) {
            return 0;
        }
        if !n2.is_null() && britanica(g(n2.addr())) {
            return 0;
        }
    }
    if !n1.is_null() {
        let (x, y) = (g(KEY_NATION_AUSTRALIA), g(KEY_NATION_NEW_ZEALAND)); // Australia and its pair
        if (id == x || id == y) && (g(n1.addr()) == x || g(n1.addr()) == y) {
            return 0;
        }
    }
    if id == g(KEY_NATION_ITALY) && !n1.is_null() && g(n1.addr()) == g(KEY_NATION_UKRAINE) {
        return 0;
    }
    if id == g(KEY_NATION_PORTUGAL) && exe::database_is_staff_eu_national(person, 0) == 0 {
        let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
        if !contract.is_null() {
            let mut d = [0u16; 4];
            let data = exe::date_date_549f70(&mut d, 1, 0, 0x7ce, 0xffff_ffff) as *const u16; // 1/1/1998
            let (dia, ano) = (core::ptr::read_unaligned(data), core::ptr::read_unaligned(data.add(1)));
            let year_of_contract = contract.date_started_year();
            if ano > year_of_contract {
                return 0;
            }
            if ano == year_of_contract && dia as i16 > (contract.date_started() as i16) {
                return 0;
            }
        }
    }
    1 // estrangeira
}

/// The age of `person` in whole years on `date` (word +0 day of year, +2 year): year − birth year (+0x12), minus 1 if
/// the birth day (+0x10) has not come yet.
///
/// Null person: "Error" box and 0xff (the original). Only ax is returned (callers use al).
/// Original: `database_get_staff_age` (0x53ab90, Database.cpp, cdecl).
///
/// [pt] { 0x53ab90 } (cdecl, Database.cpp): a idade da `pessoa` em anos completos na `data` (palavra +0 dia
/// do ano, +2 ano): ano − ano de nascimento (+0x12), menos 1 se o dia de nascimento (+0x10) ainda não
/// chegou. Pessoa nula: caixa "Error" e 0xff (a original). Só o ax é devolvido (quem chama usa al).
pub(crate) unsafe extern "C" fn database_get_staff_age(person: usize, data: *const u16) -> u16 {
    let person = Staff(person);
    if person.is_null() {
        return original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(IDX_DATABASE_GET_STAFF_AGE)(person.addr(), data);
    }
    let day_birth = person.date_of_birth() as i32;
    let years = core::ptr::read_unaligned(data.add(1)).wrapping_sub(person.date_of_birth_year());
    if day_birth - core::ptr::read_unaligned(data as *const i16) as i32 > 0 {
        years.wrapping_sub(1) // has not had a birthday yet
    } else {
        years
    }
}

/// whether the nation has free transfers: new FIFA rules, Brazil, or Europe except Poland, Turkey, Croatia and Russia
/// Original: `nation_allows_free_transfers` (0x544670, Database.cpp, cdecl).
///
/// [pt] { 0x544670 } (cdecl, Database.cpp): 1 se a `nacao` tem transferência livre: com as regras novas da
/// FIFA ([0xae233c] = 1), no Brasil ou numa nação da Europa (continente +0x71, [0x9cfa10]) fora de
/// Polônia, Turquia, Croácia e Rússia.
pub(crate) unsafe extern "C" fn nation_allows_free_transfers(nation: usize) -> u32 {
    let nation = Nation(nation);
    let g = |a: usize| read_u32(a);
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 {
        return 1;
    }
    let id = g(nation.addr());
    if id == g(KEY_NATION_BRAZIL) {
        return 1; // Brasil
    }
    let continent = nation.continent();
    if continent == 0 {
        return 0;
    }
    if id == g(KEY_NATION_POLAND) || id == g(KEY_NATION_TURKEY) || id == g(KEY_NATION_CROATIA) || id == g(KEY_NATION_RUSSIA) {
        return 0;
    }
    (g(continent) == g(KEY_CONTINENT_EUROPE)) as u32 // Europa
}

/// 1 if `person` counts as an EU national for a club of `club_nation`.
///
/// Nationality (+0x1a) or second nationality (+0x1e) with the EU mark (nation +0x7f = 2) or Switzerland ([0x9cf4ac]);
/// in Germany, any European (nation +0x71 = the continent [0x9cfa10] in table [0xae23a4], 198 bytes); in [0x9cf360],
/// everyone; in Denmark, [0x9cf348], Norway and [0x9cf3a0].
/// Original: `database_is_staff_eu_national` (0x53d740, Database.cpp, cdecl).
///
/// [pt] { 0x53d740 } (cdecl, Database.cpp): 1 se a `pessoa` conta como da UE para um clube da
/// `nacao_do_clube`: nacionalidade (+0x1a) ou segunda (+0x1e) com a marca de UE (nação +0x7f = 2) ou
/// Suíça ([0x9cf4ac]); na Alemanha, qualquer europeu (nação +0x71 = o continente [0x9cfa10] na tabela
/// [0xae23a4], 198 bytes); em [0x9cf360], todos; na Dinamarca, [0x9cf348], Noruega e [0x9cf3a0].
pub(crate) unsafe extern "C" fn database_is_staff_eu_national(person: usize, nation_of_club: usize) -> u32 {
    let person = Staff(person);
    let g = |a: usize| read_u32(a);
    let n = person.nation();
    if n.group_membership() == 2 { return 1; } // EU nationality
    let id = g(n.addr());
    if id == g(KEY_NATION_SWITZERLAND) {
        return 1;
    }
    let n2 = person.second_nation();
    if !n2.is_null() && n2.group_membership() == 2 {
        return 1;
    }
    if nation_of_club == 0 {
        return 0;
    }
    let c = g(nation_of_club);
    if c == g(KEY_NATION_GERMANY) {
        let europa = read_usize(G_CONTINENT_TABLE).wrapping_add((g(KEY_CONTINENT_EUROPE) as usize).wrapping_mul(0xc6));
        if n.continent() == europa {
            return 1;
        }
    }
    if c == g(KEY_NATION_ITALY) {
        return 1;
    }
    if c == g(KEY_NATION_DENMARK) && (id == g(KEY_NATION_ICELAND) || id == g(KEY_NATION_NORWAY) || id == g(KEY_NATION_LIECHTENSTEIN)) {
        return 1;
    }
    0
}

/// 1 if the link (`exe::contract_manager_get_contract_for_club`) of `person` with `club` is transfer listed, by the club or at request (link +0x4e, bits 1
/// and 8).
///
/// Without a club, the contract's club (`exe::contract_manager_get_contract`, stored in [0xae1080]; contract +4 = club id in table [0xae23b4], 581
/// bytes per club).
/// Original: `staff_is_transfer_listed_or_requested` (0x544520, Database.cpp, cdecl).
///
/// [pt] { 0x544520 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências, pelo clube ou a pedido (vínculo +0x4e, bits 1 e 8). Sem clube, o do
/// contrato (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id do clube na tabela [0xae23b4], 581
/// bytes por clube).
pub(crate) unsafe extern "C" fn staff_is_transfer_listed_or_requested(person: usize, club: usize) -> u32 {
    if person == 0 {
        return 0;
    }
    let mut club = club;
    if club == 0 {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        if c.is_null() {
            return 0;
        }
        let id = (c.club() as u32) as usize;
        club = read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(581));
        if club == 0 {
            return 0;
        }
    }
    let v = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
    (!v.is_null() && v.transfer_status() & 9 != 0) as u32
}

/// 1-20 rating of a relative attribute value: x = v*0.05 + 10; (x^2*0.1 + x)/3 + 0.5, clamped 1..20, truncated.
///
/// [pt] A nota de 1 a 20 de um atributo relativo (de 0x543540, 0x544380 e 0x5442f0): x = v × 0,05 + 10;
/// (x² × 0,1 + x) × 1/3 + 0,5, entre 1 e 20, truncada (`_ftol`).
pub(crate) unsafe fn attribute_rating(v: f64) -> u8 {
    let x = v * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_10);
    let z = (x * x * read_f64(rdata::F64_0_1) + x) * read_f64(rdata::F64_0_3333333333) + read_f64(rdata::F64_0_5); // arredonda;
    let (minimo, maximo) = (read_f64(rdata::F64_1), read_f64(rdata::F64_20));
    ftol(if z > minimo {
        if z < maximo {
            z
        } else {
            maximo
        }
    } else {
        minimo
    }) as u8
}

/// The player `attribute` (1..31) as 1 to 20.
///
/// Some are the byte of the `player` record; the relative ones give the rating (`attribute_rating`) of 2 × byte +
/// current ability (+5), or of 2 × byte + ability × 0.1 depending on the goalkeeper (+0xf: = 20 or < 15); 10 is byte
/// +0x58 of `person`; some go to `player_attribute_rating` (or `player_attribute_rating_with_scaled_ability`). No person, no player or attribute out of range: "Error" box
/// and 0xff (the original). Only al is returned.
/// Original: `staff_compute_player_attribute` (0x543540, Database.cpp, cdecl).
///
/// [pt] { 0x543540 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador de 1 a 20. Uns são o byte do
/// registro de `jogador`; os relativos dão a nota (`attribute_rating`) de 2 × byte + habilidade atual
/// (+5), ou de 2 × byte + habilidade × 0,1 conforme o goleiro (+0xf: = 20 ou < 15); o 10 é o byte
/// +0x58 da `pessoa`; uns vão a 0x544380 (ou 0x5442f0). Sem pessoa, sem jogador ou atributo fora:
/// caixa "Error" e 0xff (a original). Só o al é devolvido.
pub(crate) unsafe extern "C" fn staff_compute_player_attribute(person: usize, player: usize, attribute: u32) -> u8 {
    let person = Staff(person);
    let a = attribute as u8 as i8;
    if person.is_null() || !(1..=0x1f).contains(&a) || player == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_PLAYER_ATTRIBUTE)(person.addr(), player, attribute);
    }
    // simplification: the original pushes the whole register with the byte at the bottom; both only read the byte
    let byte = |d: usize| read_i8(player + d) as u8;
    let ability = read_i16(player + 5) as i32;
    let rating = |d: usize| attribute_rating((2 * read_i8(player + d) as i32 + ability) as f64);
    let rating_h = |d: usize| attribute_rating(ability as f64 * read_f64(rdata::F64_0_1) + (2 * read_i8(player + d) as i32) as f64);
    let goalkeeper = read_i8(player + 0xf);
    match a {
        1 => byte(0x1b),
        2 => byte(0x1c),
        3 => byte(0x1d),
        4 => rating(0x1e),
        5 => byte(0x1f),
        6 => byte(0x20),
        7 => if goalkeeper == 20 { rating_h(0x43) } else { rating(0x43) },
        8 => if goalkeeper == 20 { rating_h(0x23) } else { rating(0x23) },
        9 => rating(0x24),
        10 => (person.determination() as u8),
        11 => if goalkeeper == 20 { rating_h(0x26) } else { rating(0x26) },
        12 => if goalkeeper == 20 { exe::player_attribute_rating_with_scaled_ability(player, byte(0x27) as u32) } else { exe::player_attribute_rating(player, byte(0x27) as u32) },
        13 => byte(0x28),
        14 => if goalkeeper < 15 { rating_h(0x2a) } else { rating(0x2a) },
        15 => rating(0x2b),
        16 => byte(0x2f),
        17 => byte(0x2e),
        18 => exe::player_attribute_rating(player, byte(0x31) as u32),
        19 => if goalkeeper == 20 { rating_h(0x32) } else { rating(0x32) },
        20 => if goalkeeper == 20 { rating_h(0x33) } else { rating(0x33) },
        21 => byte(0x36),
        22 => rating(0x37),
        23 => rating(0x39),
        24 => if goalkeeper < 15 { rating_h(0x3a) } else { exe::player_attribute_rating(player, byte(0x3a) as u32) },
        25 => byte(0x29),
        26 => byte(0x3c),
        27 => byte(0x3d),
        28 => exe::player_attribute_rating(player, byte(0x3e) as u32),
        29 => byte(0x3f),
        30 => byte(0x40),
        _ => byte(0x44),
    }
}

/// a player attribute of the person, from the person's player data (`staff_compute_player_attribute`)
/// Original: `staff_get_player_attribute` (0x5431e0, Database.cpp, cdecl).
///
/// [pt] { 0x5431e0 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador da `pessoa`: 0x543540 com os
/// dados de jogador (+0x61). Sem pessoa ou atributo fora: caixa "Error" e 0xff (a original).
pub(crate) unsafe extern "C" fn staff_get_player_attribute(person: usize, attribute: u32) -> u8 {
    let person = Staff(person);
    let a = attribute as u8 as i8;
    if person.is_null() || !(1..=0x1f).contains(&a) {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE)(person.addr(), attribute);
    }
    // the original routine pushes the whole argument, as it came
    exe::staff_compute_player_attribute(person, person.player(), attribute)
}

/// `save_database_for_game(save, a4, type)`: step 1 of `save_game_data`: `save_database(save, 1, a4, type)` (`exe::save_database`,
/// into the container), the database tables, returning its result.
///
/// Was turned off (DESLIGADAS) until 2026-10-31, when the test criterion changed and the list was emptied: save_database writes into index.dat 408 bytes of unset locals (pointer garbage,
/// from +0x5cb); called from this copy it runs on the private stack and the garbage written changes (proof of the save
/// of the 219th batch). It came back with the new criterion.
/// Original: `save_database_for_game` (0x524d30, Database.cpp, cdecl).
///
/// [pt] { 0x524d30 } (cdecl, Database.cpp): `save_database_for_game(save, a4, tipo)`: o passo 1 do
/// `save_game_data`: `save_database(save, 1, a4, tipo)` (0x52dad0, para o contêiner), devolvendo a
/// resposta dela. Ficou desligada (DESLIGADAS) até 31/10/2026, quando o critério dos testes mudou e a lista ficou vazia: o save_database grava no index.dat 408 bytes de
/// locais sem valor (lixo de ponteiro, a partir de +0x5cb); chamado desta cópia ele roda na pilha
/// particular e o lixo gravado muda (prova do save da 219a). Voltou com o critério novo.
pub(crate) unsafe extern "C" fn save_database_for_game(save: usize, a4: u32, kind: u32) -> u32 {
    crate::comparison::branch("save write database");
    exe::save_database(save, 1, a4, kind)
}

// simplification: writes the database tables into the container; it does not run twice. Original only; the copy
// takes the branch and is proven by the save proof.

/// load_game_data step 2: load_database(save, shared, 1), the database read from the container
/// Original: `load_database_from_stream` (0x524d10, Database.cpp, cdecl).
///
/// [pt] { 0x524d10 } (cdecl, Database.cpp): `load_database_from_stream(save, compartilhado)`: o passo 2
/// do `load_game_data`: `load_database(save, compartilhado, 1)` (0x528540, do contêiner), devolvendo
/// a resposta dela (1 = certo).
pub(crate) unsafe extern "C" fn load_database_from_stream(save: usize, shared: u32) -> u32 {
    crate::comparison::branch("save load database");
    exe::load_database(save, shared, 1)
}

// simplification: loads the whole database from the container (allocates the tables); does not run twice. Only the
// original; the copy carries the branch (the load step of the save proof exercises it).

// simplification: destroys all subsystems and deletes the *.tmp; does not run twice. Original only; the
// the copy takes the branch and runs in Rust mode (career start, end of quick start).

/// reads global 0x9b5984
/// Original: `database_global_9b5984` (0x535920, Database.cpp, cdecl).
///
/// [pt] { 0x535920 } (Database.cpp): a global 0x9b5984.
pub(crate) unsafe extern "C" fn database_global_9b5984() -> u32 {
    read_u32(G_DATABASE_NAMES_LANGUAGE)
}

