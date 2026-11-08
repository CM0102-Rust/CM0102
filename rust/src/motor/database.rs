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
    [0x9b, 0x9f, 0xa3].iter().any(|&o| *((b + o) as *const usize) == a) as u32
}

/// does `b` have `a` among its three rejected (+0xa7, +0xab, +0xaf)? Compared by the result
/// Original: `rejected_by` (0x5477d0, cdecl).
///
/// [pt] { 0x5477d0 } (cdecl): `b` tem `a` entre os tres rejeitados (+0xa7, +0xab, +0xaf)?
pub(crate) unsafe extern "C" fn rejected_by(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    [0xa7, 0xab, 0xaf].iter().any(|&o| *((b + o) as *const usize) == a) as u32
}

/// Liga as unidades da fita do fechamento (0x6d26b0, 0x6d3610, 0x6d3de0 e o conteudo da 0x68d0d0).
pub(crate) static RECORD_WRAP_UP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True when the fechamento call taping is switched on (RECORD_WRAP_UP) and a recording tape is open.
pub(crate) fn is_recording_wrap_up_calls() -> bool {
    RECORD_WRAP_UP.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// Body of fechamento (0x6d1cc0) with the out-of-engine calls (player/manager/club reputation updates, incident list
/// copy) routed through a callback: processes the day's match incidents, transfer-request and unhappiness events and
/// post-match reputations.
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
pub(crate) unsafe fn match_wrap_up_with(m: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let conhecer: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let d = |p: usize, o: usize| *((p + o) as *const usize);
    let w = |p: usize, o: usize| *((p + o) as *const i16);
    let lista_global = || *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    // poe uma copia do bloco (0x19 bytes) na lista de eventos (o conteudo e conferido na fita)
    let mut evento = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, p: usize| {
        request_buffers(&[(p, 0x19)], &[]);
        chamar(FN_LLIST_ADD_TAIL, lista_global(), [0, 0x19, 0, 0, 0]);
    };
    let partida = || d(m, 0x4792);
    let pessoa_de = |id: i32| (*(G_STAFF_TABLE as *const usize)).wrapping_add(id as usize * staff::SIZE);
    let lista = d(m, 0x478e);
    if lista != 0 && lista_global() != 0 && partida() != 0 && byte(partida(), 0x40) != 0 {
        *((lista + 0xa) as *mut u32) = core::ptr::read_unaligned((lista + 2) as *const u32);
        let mut i: i8 = 0;
        while (i as i16) < *(lista as *const i16) {
            let e = proximo(lista);
            if e == 0 {
                break;
            }
            let tipo = w(e, 0x13);
            crate::comparacao::ramo("match_wrap_up evento do dia");
            let marcar = |valor: u8, quem: u32| {
                *((e + 0xe) as *mut u8) = valor;
                core::ptr::write_unaligned((e + 0xf) as *mut u32, quem);
            };
            let pt = partida();
            let clube_ev = || (*(G_CLUB_TABLE as *const usize)).wrapping_add(d(e, 4) * club::SIZE);
            let injury_word_0c = || w(d(pessoa_de(*(e as *const i32)), 0x61), 0xb);
            if tipo == 0x82 {
                evento(chamar, e);
            } else if tipo == 0x1e {
                let (c1c, c20) = (d(pt, 0x1c), d(pt, 0x20));
                if byte(e, 0xd) != 0 {
                    let (a, b) = (byte(pt, 0x43), byte(pt, 0x44));
                    let ok = b < a || (b == a && (w(c1c, 0x80) as f64) < w(c20, 0x80) as f64 * read_f64_at(0x968e40)); // 0x968e40 = 0.7
                    let t = d(c20, 0xd3);
                    if ok && t != 0 && d(t, 0x69) != 0 {
                        let x = d(t, 0x69);
                        let r1 = sortear(conhecer(x, byte(x, 0x17) as u8 as u32) as i32);
                        if r1 > sortear(0x14) && byte(x, 0x14) > 5 {
                            crate::comparacao::ramo("match_wrap_up proposta aceita");
                            marcar(1, *(t as *const u32));
                            evento(chamar, e);
                        } else {
                            let rep = injury_word_0c();
                            let r = sortear(1000);
                            let forca = w(clube_ev(), 0x80) as i32 - r + sortear(1000);
                            if rep as i32 > forca || rep > 7000 {
                                marcar(0, u32::MAX);
                                evento(chamar, e);
                            }
                        }
                    }
                } else {
                    let (a, b) = (byte(pt, 0x43), byte(pt, 0x44));
                    let ok = b > a || (b == a && (w(c20, 0x80) as f64) < w(c1c, 0x80) as f64 * read_f64_at(0x968e48)); // 0x968e48 = 0.6
                    let t = d(c1c, 0xd3);
                    if ok && t != 0 && d(t, 0x69) != 0 {
                        let x = d(t, 0x69);
                        if sortear(conhecer(x, byte(x, 0x17) as u8 as u32) as i32) > 5 && byte(x, 0x14) > 5 {
                            crate::comparacao::ramo("match_wrap_up proposta aceita");
                            marcar(1, *(t as *const u32));
                            evento(chamar, e);
                        } else {
                            if injury_word_0c() > w(clube_ev(), 0x80) && sortear(4) == 0 {
                                marcar(0, u32::MAX);
                            }
                            evento(chamar, e);
                        }
                    }
                }
            } else {
                let pessoa = pessoa_de(*(e as *const i32));
                if pessoa != 0 {
                    // o jogador do evento num dos dois lados
                    let achar = |lado: usize| (0..20usize).find(|&k| {
                        let p = m + 0x4796 + (k + lado * 20) * 0x1be;
                        d(p, 0x69) == pessoa && byte(p, 0x19) >= 0
                    });
                    let achado = achar(0).map(|k| (k, 0)).or_else(|| achar(1).map(|k| (k, 1)));
                    if let Some((k, lado)) = achado {
                        let p = m + 0x4796 + (k + lado * 20) * 0x1be;
                        let vale = if tipo == 6 {
                            crate::comparacao::ramo("match_wrap_up evento tipo 6");
                            byte(p, 0x24) == 2 || byte(p, 0x23) != 0
                        } else {
                            byte(p, 0x1b) < 7 && sortear(byte(p, 0x17) as i32) == 0
                        };
                        if vale {
                            crate::comparacao::ramo("match_wrap_up evento do jogador");
                            marcar(0, u32::MAX);
                            evento(chamar, e);
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let mut media = 0f64;
    for lado in 0..2usize {
        let (mut soma, mut somap) = (0i32, 0i32);
        let pt = partida();
        let (meu, outro) = if lado == 0 { (d(pt, 0x1c), d(pt, 0x20)) } else { (d(pt, 0x20), d(pt, 0x1c)) };
        for k in 0..20usize {
            let p = m + 0x4796 + (k + lado * 20) * 0x1be;
            if k > 10 && byte(p, 0x1f) == -1 {
                continue;
            }
            let pessoa = d(p, 0x69);
            if pessoa != 0 && d(p, 0x6d) != 0 && *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) {
                chamar(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH, p, [pessoa as u32, meu as u32, outro as u32, (byte(p, 0x1b) as f32).to_bits(), *(m as *const u8) as u32 | 1 << 8]);
                if lista_global() != 0 && selecao(meu) == 0 && byte(pt, 0x40) != 0 {
                    let ct = contrato(*(G_CONTRACT_MANAGER as *const usize), pessoa, meu);
                    if ct != 0
                        && meu != 0
                        && pessoa != d(meu, 0xcf)
                        && d(pt, 0x14) != 0
                        && amistoso(*(G_FRIENDLY as *const usize), d(pt, 0x14)) == 0
                        && byte(p, 0x20) != -1
                        && byte(p, 0x22) == -1
                        && byte(ct, 0x3d) > 0
                        && w(d(p, 0x6d), 0xb) > 0x109a
                        && partida() != 0
                        && byte(partida(), 0x40) != 0
                    {
                        let r1 = sortear(match_pl_ftol(read_f32_at(p + 0x1a2) as f64));
                        if r1 > sortear(100) {
                            let r3 = sortear(byte(pessoa, 0x5d) as i32);
                            if r3 < sortear(10) {
                                let f = read_f32_at(p + 0x4d);
                                if ((sortear(0x5dc) + 0x1d4c) as f64) < f as f64 {
                                    let b5b = byte(pessoa, 0x5b);
                                    if b5b < 6 && byte(pessoa, 0x57) > 10 {
                                        let r6 = sortear(byte(pessoa, 0x57) as i32);
                                        if sortear(b5b as i32) < r6 {
                                            crate::comparacao::ramo("match_wrap_up pede para sair");
                                            let mut ev = [0u8; 0x19];
                                            ev[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
                                            ev[4..8].copy_from_slice(&(*(meu as *const u32)).to_le_bytes());
                                            ev[8..12].copy_from_slice(&0x4000000u32.to_le_bytes());
                                            ev[0xd] = lado as u8;
                                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                                            ev[0x13..0x15].copy_from_slice(&10u16.to_le_bytes());
                                            evento(chamar, ev.as_ptr() as usize);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // a insatisfacao
                    let mut x = -(w(p, 0x31) as i32);
                    if [0xb3, 0xb7, 0xbb].iter().any(|&o| d(meu, o) == outro) {
                        x += 10;
                    }
                    if rejeitado(pessoa, outro) != 0 {
                        x += 10;
                    } else if favorito(pessoa, outro) != 0 {
                        x -= 10;
                    }
                    if nao_gosta(pessoa, outro) != 0 {
                        x += 10;
                    } else if gosta(pessoa, outro) != 0 {
                        x -= 20;
                    }
                    let (a, b, c) = (byte(pessoa, 0x5a) as i32, byte(pessoa, 0x5b), byte(pessoa, 0x5d));
                    if b < 6 && c < 10 {
                        let r1 = sortear((a * b as i32 * c as i32 + 0x14) * 100);
                        if r1 < sortear(x + 0x32) {
                            crate::comparacao::ramo("match_wrap_up insatisfeito");
                            let mut ev = [0u8; 0x19];
                            ev[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
                            ev[4..8].copy_from_slice(&(*(meu as *const u32)).to_le_bytes());
                            ev[0xd] = lado as u8;
                            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
                            ev[0x13..0x15].copy_from_slice(&11u16.to_le_bytes());
                            evento(chamar, ev.as_ptr() as usize);
                        }
                    }
                }
            }
            // os minutos jogados
            let minuto = (w(m, 0x8ed0) as i32 / 11) as i8;
            let v: i8 = if k >= 11 {
                let c = byte(p, 0x1f);
                if c == -1 {
                    continue;
                }
                let fim = if byte(p, 0x21) > -1 { byte(p, 0x21) } else if byte(p, 0x20) > -1 { byte(p, 0x20) } else { minuto };
                fim.wrapping_sub(c)
            } else if *(m as *const u8) != 0 {
                0x5a
            } else if byte(p, 0x21) > -1 {
                byte(p, 0x21)
            } else if byte(p, 0x20) > -1 {
                byte(p, 0x20)
            } else {
                minuto
            };
            if v == 0 {
                continue;
            }
            soma += v as i32;
            somap += byte(p, 0x1b) as i32 * v as i32;
        }
        if soma != 0 {
            let q = somap as f64 / soma as f64;
            media = if d(pt, 0x18) == d(meu, 0x69) { q - read_f64_at(0x967880) } else { q + read_f64_at(0x968918) }; // 0x967880 = 0.1, 0x968918 = 0.2
        }
        crate::comparacao::ramo(if soma != 0 { "match_wrap_up media do lado" } else { "match_wrap_up lado sem minutos" });
        let valor = (media as f32).to_bits();
        let alt = *(m as *const u8) as u32;
        chamar(FN_UPDATE_MANAGER_REPUTATION_AFTER_MATCH, m + 0x9032 + lado * 0xd0, [meu as u32, outro as u32, pt as u32, valor, alt | 1 << 8]);
        if selecao(meu) == 0 {
            chamar(FN_MATCH_ENG_COMPUTE_CLUB_REPUTATION, m, [lado as u32, valor, alt | 1 << 8, 0, 0]);
        }
    }
}

/// Real dispatcher for the calls injected into fechamento: player/manager reputation updates (0x6d26b0, 0x6d3610),
/// club reputation (0x6d3de0) or adding the requested block to the incident list (0x68d0d0).
///
/// [pt] Despacho real das chamadas injetadas do fechamento.
pub(crate) unsafe fn dispatch_wrap_up_call(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    let m = |p: usize| *((p + 0x19e) as *const usize);
    match rotina {
        0x6d26b0 => {
            let f: unsafe extern "C" fn(usize, usize, usize, usize, usize, f32, u32, u32) = core::mem::transmute(FN_UPDATE_PLAYER_REPUTATION_AFTER_MATCH);
            f(this, a[0] as usize, a[1] as usize, a[2] as usize, *((m(this) + 0x4792) as *const usize), f32::from_bits(a[3]), a[4] & 0xff, (a[4] >> 8) & 0xff);
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
            // 0x68d0d0: o bloco pedido por ultimo (request_buffers) e o que vai para a lista
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
pub(crate) unsafe fn wrap_up_regions(m: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4)]);
    let lista = *((m + 0x478e) as *const usize);
    if lista != 0 {
        reg.push((lista, 0xe));
        let mut no = core::ptr::read_unaligned((lista + 2) as *const usize);
        for _ in 0..(*(lista as *const i16)).max(0) {
            if no == 0 {
                break;
            }
            let e = *(no as *const usize);
            if e != 0 {
                reg.push((e, 0x19));
            }
            no = core::ptr::read_unaligned((no + 6) as *const usize);
        }
    }
    reg
}



/// Returns 1 when the given type byte is not 11, else 0 [inference: a competition/club type filter].
///
/// [pt] { 0x53b610 } (Database.cpp): o tipo nao e 11?
pub(crate) unsafe extern "C" fn type_not_11(tipo: i8) -> u32 {
    (tipo != 11) as u32
}

/// Returns 1 if type code `tipo` is 11, 12, 13 or 15, else 0 [inference: a class of club or competition types].
///
/// [pt] { 0x53b620 } (Database.cpp): o tipo e 11, 12, 13 ou 15?
pub(crate) unsafe extern "C" fn type_11_to_15(tipo: i8) -> u32 {
    matches!(tipo, 11 | 12 | 13 | 15) as u32
}

/// Class of a national side for the national-teams blocks: 1 when `flag` is set, otherwise 2 for a nation's second team
/// and 0 for any other club.
///
/// [pt] { 0x76d780 } (stdcall, national_teams.cpp): com `flag`, 1; sem ela, 2 para a segunda selecao
/// (0x5409b0) e 0 para as outras.
pub(crate) unsafe extern "stdcall" fn national_side_class(clube: usize, flag: u32) -> u8 {
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let e_segunda = segunda(clube) != 0;
    if flag != 0 {
        1
    } else if e_segunda {
        2
    } else {
        0
    }
}

/// Pointer to the per-person flag byte: [this+4] + staff id x 110 + k x 20 + 0x45.
///
/// [pt] O byte das marcas por pessoa: [this+4] + id x 110 + k x 20 + 0x45.
pub(crate) unsafe fn person_flag_byte(this: usize, pessoa: usize, k: i8) -> *mut u8 {
    (*((this + 4) as *const usize))
        .wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
        .wrapping_add(0x45) as *mut u8
}

/// Returns flag `k` of the person in the national-teams table (`this`; byte at [this+4] + id x 110 + k x 20 + 0x45).
///
/// [pt] { 0x76d7b0 } (ret 8, national_teams.cpp): a marca `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn read_person_mark(this: usize, pessoa: usize, k: i8) -> u8 {
    *person_flag_byte(this, pessoa, k)
}

/// Clears flag `k` of the person in the national-teams table (`this`; byte at [this+4] + id x 110 + k x 20 + 0x45).
///
/// [pt] { 0x76d7e0 } (ret 8, national_teams.cpp): zera a marca `k` da pessoa.
pub(crate) unsafe extern "thiscall" fn clear_person_mark(this: usize, pessoa: usize, k: i8) {
    *person_flag_byte(this, pessoa, k) = 0;
}

/// the first record with +0 zero in table 0xae23f8 (60 bytes, 0xae23a0 records), cached at 0x9b59b4; with none, a
/// random one
/// Original: `free_record` (0x53a5b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a5b0 } (Database.cpp): o registro livre (+0 zero) da tabela 0xae23f8 (60 bytes,
/// 0xae23a0 registros), guardado em 0x9b59b4 na primeira vez; sem nenhum, um sorteado.
pub(crate) unsafe extern "C" fn free_record() -> usize {
    let base = || *(G_COMMON_NAME_TABLE as *const usize);
    let mut i = *(G_EMPTY_COMMON_NAME_INDEX as *const i32);
    if i == -1 {
        let n = *(G_COMMON_NAME_COUNT as *const i32);
        match (0..n.max(0)).find(|&k| *(base().wrapping_add(k as usize * 60) as *const u8) == 0) {
            Some(k) => {
                *(G_EMPTY_COMMON_NAME_INDEX as *mut i32) = k;
                return base().wrapping_add(k as usize * 60);
            }
            None => i = sortear(n),
        }
    }
    base().wrapping_add((i as usize).wrapping_mul(60))
}

/// Pares de globais com os ids dos clubes A e B (times B da Espanha e outros), na ordem dos testes
/// do original (0x540a50); extraidos da desmontagem.
pub(crate) const A_B_CLUB_PAIRS: [(usize, usize); 82] = [
    (0x9d012c, 0x9d0130), (0x9d0120, 0x9d0134), (0x9d0138, 0x9d013c), (0x9d0140, 0x9d0144), (0x9d0148, 0x9d014c), (0x9d0150, 0x9d0154),
    (0x9d0158, 0x9d015c), (0x9d0160, 0x9d0164), (0x9d0168, 0x9d016c), (0x9d0170, 0x9d0174), (0x9d0178, 0x9d017c), (0x9d0180, 0x9d0184),
    (0x9d0188, 0x9d018c), (0x9d0190, 0x9d0194), (0x9d0198, 0x9d019c), (0x9d0128, 0x9d01a0), (0x9d0254, 0x9d0258), (0x9d02e4, 0x9d0328),
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
    (0x9d018c, 0x9d0188), (0x9d0194, 0x9d0190), (0x9d019c, 0x9d0198), (0x9d01a0, 0x9d0128), (0x9d0258, 0x9d0254), (0x9d0328, 0x9d02e4),
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
pub(crate) unsafe extern "C" fn partner_club(clube: usize, eh_a: *mut u32, ativo: u32) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut u32, u32) -> usize>(IDX_PARTNER_CLUB)(clube, eh_a, ativo);
    }
    let nacao = *((clube + club::NATION) as *const usize);
    if nacao == 0 || (ativo != 0 && byte(clube, 0x244) == 0) {
        return 0;
    }
    let g = |a: usize| *(a as *const u32);
    let n = *(nacao as *const u32);
    if n != g(KEY_NATION_SPAIN) && n != g(KEY_NATION_GERMANY) && n != g(KEY_NATION_PORTUGAL) {
        return 0;
    }
    let id = *(clube as *const u32);
    let parceiro = if let Some(&(_, b)) = A_B_CLUB_PAIRS.iter().find(|&&(a, _)| g(a) == id) {
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
    match parceiro {
        Some(v) => (*(G_CLUB_TABLE as *const usize)).wrapping_add((v as usize).wrapping_mul(581)),
        None => {
            *((clube + club::HAS_LINKED_CLUB) as *mut u8) = 0;
            0
        }
    }
}

/// Match engine: asks the engine's referee (+0xf5b2) to judge a foul of player `j` on `outro` (MatchOfficial foul
/// decision 0x6e9820), passing the foul type, who committed it and the out-parameters for card, severity and advantage.
/// Returns the referee's decision.
///
/// [pt] { 0x6cfc60 } (match_eng.cpp): a decisao da falta (0x6e9820) com o arbitro do motor (+0xf5b2).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn foul_in_engine(
    m: usize, j: usize, outro: usize, tipo: i8, quem: i8, cartao: *mut u8, grav: *mut i8, vantagem: *mut u8,
) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8 = core::mem::transmute(FN_MATCH_OFFICIAL_JUDGE_TACKLE);
    f(m + 0xf5b2, j, outro, tipo, quem, cartao, grav, vantagem)
}

/// Returns holiday option `i` (byte +0x9b23 + i) of the human manager record for the person (or the current human
/// manager). Without a person or out of range, falls back to the original (error box).
///
/// [pt] { 0x5fea50 } (human_manager.cpp): byte +0x9b23 + `i` do registro do tecnico humano da pessoa (ou
/// do atual). Sem pessoa ou fora da faixa: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn manager_9b23(this: usize, i: i8, pessoa: usize) -> u8 {
    match person_and_human_record(this, pessoa) {
        Some((_, r)) => *((r as isize + 0x9b23 + i as isize) as *const u8),
        None => original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(IDX_MANAGER_9B23)(this, i, pessoa),
    }
}

/// Is the club among the person's favourite clubs (preference slots +0x04, +0x08, +0x0c)? Returns 1 or 0.
///
/// [pt] { 0x547d60 }: `pessoa` gosta do clube (preferencias +0x04, +0x08, +0x0c)?
pub(crate) unsafe extern "C" fn likes_club(pessoa: usize, clube: usize) -> u32 {
    is_in_preferences(clube, pessoa, 4)
}

/// Is the club among the person's disliked clubs (preference slots +0x10, +0x14, +0x18)? Returns 1 or 0.
///
/// [pt] { 0x547d30 }: `pessoa` nao gosta do clube (preferencias +0x10, +0x14, +0x18)?
pub(crate) unsafe extern "C" fn dislikes_club(pessoa: usize, clube: usize) -> u32 {
    is_in_preferences(clube, pessoa, 0x10)
}

/// Club record of a nation's national team (or B team) at the end of the club table (581 bytes each), counted back
/// `nacoes_antes` nations; falls back to the original when tables or nation are missing.
///
/// [pt] Clube da selecao da nacao: as selecoes ficam no fim do vetor de clubes (0xae23b4, 581 bytes
/// cada), a principal `deslocamento` = 2 nacoes antes do fim e a B uma nacao antes. Sem nacao,
/// sem clubes ou sem 0xae23a8: o original mostra a caixa de erro (Database.cpp) e zera 0xb67a34;
/// esse caminho fica com ele.
pub(crate) unsafe fn national_team_club(nacao: usize, nacoes_antes: i32, idx: usize) -> usize {
    let base = *(G_CLUB_TABLE as *const usize);
    if nacao == 0 || base == 0 || *(G_NATION_TABLE as *const usize) == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(idx)(nacao);
    }
    let i = (*(nacao as *const i32))
        .wrapping_sub(nacoes_antes.wrapping_mul(*(G_NATION_COUNT as *const i32)))
        .wrapping_add(*(G_CLUB_COUNT as *const i32));
    base.wrapping_add((i as usize).wrapping_mul(581))
}

/// Returns the club record of nation `nacao`'s main national team (stored near the end of the club table).
///
/// [pt] { 0x53b3d0 } (Database.cpp): o clube da selecao principal da nacao.
pub(crate) unsafe extern "C" fn main_national_team(nacao: usize) -> usize {
    national_team_club(nacao, 2, 175)
}

/// Returns the club record of the nation's B national team (the national sides sit at the end of the club table).
///
/// [pt] { 0x53b4f0 } (Database.cpp): o clube da selecao B da nacao.
pub(crate) unsafe extern "C" fn b_national_team(nacao: usize) -> usize {
    national_team_club(nacao, 1, 176)
}

/// Match engine: checks whether the ball is out of play; if so releases it, sets the restart (throw-in, goal kick or
/// corner, +0x8eb2) and the side taking it (+0x8eae), adds the commentary and returns 1; ball in play returns 0.
/// `lado_b` is the side that last touched it. Wrapper of `ball_out_with` with the real commentary call.
///
/// [pt] { 0x6cd490 } (match_eng.cpp): bola fora do campo (ver `ball_out_with`).
pub(crate) unsafe extern "thiscall" fn ball_out(this: usize, lado_b: u8) -> u8 {
    ball_out_with(this, lado_b, &mut |t, a| add_match_event(t, a))
}

/// Returns 1 if the club belongs to the second set of national teams (id at least club count minus nation count)
/// [inference: the under-21/B national sides]. A null club goes to the original routine, whose error path changes the
/// career.
///
/// [pt] { 0x5409b0 } (Database.cpp): o clube e do segundo conjunto de selecoes (id a partir de total de
/// clubes 0xae235c menos nacoes 0xae2350)? Clube nulo: o original mostra uma caixa de erro e zera
/// 0xb67a34 (aqui, sem a caixa).
pub(crate) unsafe extern "C" fn is_second_national_team(clube: usize) -> u32 {
    if clube == 0 {
        // o caminho de erro do original (mensagem e caixa) tem efeitos que mudam a carreira: o jogo
        // real chama com clube nulo. Usa a propria original nesse caso.
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_SECOND_NATIONAL_TEAM)(0);
    }
    let limit = (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32));
    (*(clube as *const i32) >= limit) as u32
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
pub(crate) unsafe extern "thiscall" fn aggregate_score(this: usize, lado_b: i8, casa: *mut i8, fora: *mut i8, classificado: *mut u8, ida: *mut u8, fase3: *mut u8, bonus: u8) -> u8 {
    *casa = 0;
    *fora = 0;
    *classificado = 0;
    *ida = 0;
    *fase3 = 0;
    let (este, outro) = if lado_b == 0 { (casa, fora) } else { (fora, casa) };
    let (bonus_este, bonus_outro): (i8, i8) = match (lado_b == 0, bonus != 0) {
        (true, true) => (1, 0),
        (false, true) => (0, 1),
        _ => (0, 0),
    };
    *este = byte(this, 0xf5bd);
    *outro = byte(this, 0xf5f3);
    let partida = *((this + 0x4792) as *const usize);
    let unidades_: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_ROUND_VARIANT);
    if unidades_(*((partida + 0x30) as *const i16) as i32) == 3 {
        *fase3 = 1;
    }
    let ida_casa = byte(partida, 0x4b);
    if ida_casa != -1 {
        *este = (*este).wrapping_add(ida_casa);
        *outro = (*outro).wrapping_add(byte(partida, 0x4c));
        *ida = 1;
        if *este as i32 + bonus_este as i32 == *outro as i32 + bonus_outro as i32 {
            let f = *((partida + 0x38) as *const u16);
            let tempo_ok = f & 0x4000 == 0 || *((this + 0x8ed4) as *const i16) > 0x3de;
            if f & 4 == 0 && tempo_ok {
                let gols_fora = *((this + 0xf5f3) as *const u8) as i32;
                if lado_b == 0 {
                    if gols_fora + (bonus_este as i32) < ida_casa as i32 {
                        *classificado = 1;
                    }
                } else if gols_fora + (bonus_outro as i32) > ida_casa as i32 {
                    *classificado = 1;
                }
            }
        }
    }
    if *classificado != 0 {
        1
    } else {
        (*casa as u8).wrapping_sub(*fora as u8)
    }
}

/// 1 if `alvo` is one of the three people at offset `base` of the person's STAFF_PREFERENCES record, else 0 (0 on any
/// null pointer).
///
/// [pt] Preferencias (`CM3_STAFF_PREFERENCES`, ponteiro em +0x65 da pessoa): `alvo` esta entre as tres
/// pessoas em `base`, +4, +8? (0 com qualquer ponteiro nulo.)
pub(crate) unsafe fn is_in_preferences(alvo: usize, pessoa: usize, base: usize) -> u32 {
    if alvo == 0 || pessoa == 0 {
        return 0;
    }
    let pref = *((pessoa + staff::STAFF_PREFERENCES) as *const usize);
    if pref == 0 {
        return 0;
    }
    (0..3).any(|k| *((pref + base + 4 * k) as *const usize) == alvo) as u32
}

/// Does `pessoa` like `alvo`? True if `alvo` is one of the three favoured people in the person's preferences record
/// (+0x1c, +0x20, +0x24). Returns 1 or 0.
///
/// [pt] { 0x547810 }: `pessoa` gosta de `alvo` (preferencias +0x1c, +0x20, +0x24)?
pub(crate) unsafe extern "C" fn likes(alvo: usize, pessoa: usize) -> u32 {
    is_in_preferences(alvo, pessoa, 0x1c)
}

/// Does `pessoa` dislike `alvo`? True if `alvo` is one of the three disliked people in the person's preferences record
/// (+0x28, +0x2c, +0x30). Returns 1 or 0.
///
/// [pt] { 0x547840 }: `pessoa` nao gosta de `alvo` (preferencias +0x28, +0x2c, +0x30)?
pub(crate) unsafe extern "C" fn dislikes(alvo: usize, pessoa: usize) -> u32 {
    is_in_preferences(alvo, pessoa, 0x28)
}

/// Formation: stores the value pointed to by `ficha` (or -1 with none) in the per-position table (+0x5bf + 4 x
/// position) and clears the changed flag (+0x187f). Returns 1.
///
/// [pt] { 0x5b5910 }: grava na tabela por posicao (+0x5bf + 4 x posicao) o valor apontado por `ficha`
/// (-1 sem ficha) e zera +0x187f. Devolve 1.
pub(crate) unsafe extern "thiscall" fn mark_position(this: usize, posicao: i8, ficha: *const u32) -> u32 {
    let destino = (this as isize + 0x5bf + 4 * posicao as isize) as *mut u32;
    *destino = if ficha.is_null() { u32::MAX } else { *ficha };
    *((this + 0x187f) as *mut u32) = 0;
    1
}

/// Reads the signed 16-bit value of cell k of a 3-byte-per-cell attribute vector (as built by 0x535a80).
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

/// Fills the 63 attribute-vector cells with terminators (value 0xff81, weight 0x81) as 0x535a80 prepares them.
///
/// [pt] Os 63 terminadores (palavra 0xff81, peso 0x81), como a 0x535a80 prepara os vetores.
pub(crate) unsafe fn clear_attr_vector(v: usize) {
    for k in 0..63 {
        core::ptr::write_unaligned((v + 3 * k) as *mut u16, 0xff81);
        *((v + 3 * k + 2) as *mut u8) = 0x81;
    }
}

/// Age of a person at a date as 0x535a80 computes it: date year byte minus birth year byte (+0x12), minus 1 if the
/// birth day-of-year (+0x10) is later.
///
/// [pt] A idade da 0x535a80: byte do ano da data − byte do ano de nascimento (+0x12), menos 1 se o dia do ano de
/// nascimento (+0x10) passa o da data.
pub(crate) unsafe fn age_at_date(p: usize, data: usize) -> i8 {
    let a = (*((data + 2) as *const u8)).wrapping_sub(*((p + 0x12) as *const u8));
    let d = core::ptr::read_unaligned((p + 0x10) as *const i16) as i32 - core::ptr::read_unaligned(data as *const i16) as i32;
    (if d > 0 { a.wrapping_sub(1) } else { a }) as i8
}

/// Free-agent (no club) reduction used by 0x535a80: x - x*0.25 when byte +0x22 is set, else x - x*0.6, truncated
/// [inference: constants at 0x9689e0/0x968e48].
///
/// [pt] O ajuste do livre (sem clube) da 0x535a80: x − x × 0,25 (com +0x22) ou x − x × 0,6, truncado.
pub(crate) unsafe fn free_agent_adjust(p: usize, x: i16) -> i16 {
    let x = x as f64;
    let k = if *((p + 0x22) as *const u8) != 0 { 0x9689e0 } else { 0x968e48 };
    match_pl_ftol(x - x * read_f64_at(k)) as i16
}

/// True if a <= x <= b.
pub(crate) fn in_range(x: i16, a: i16, b: i16) -> bool {
    x >= a && x <= b
}

/// prepares every person of the new game: missing names, dates, staff attributes, nation; for the player the abilities,
/// potential, current ability, age, positions and missing attributes (from the most similar of 250 random players); for
/// the non-player the reputations, attributes (template among 250), job and the famous names; then the ability-relative
/// vectors and the normalisations; "Player Initialisation" bar
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
    if *(G_STAFF_COUNT as *const i32) == 0 || *(G_STAFF_TABLE as *const usize) == 0 {
        return original_fn::<unsafe extern "C" fn(usize)>(IDX_DATABASE_PLAYER_INITIALISATION)(data);
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_SET);
    let semanas_antes: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let vet_j: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize = core::mem::transmute(FN_DATABASE_PLAYER_TO_ATTRIBUTE_VECTOR);
    let vet_nj: unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32 = core::mem::transmute(FN_DATABASE_NON_PLAYER_TO_ATTRIBUTE_VECTOR);
    let de_vet_j: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_ATTRIBUTE_VECTOR_TO_PLAYER);
    let de_vet_nj: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_ATTRIBUTE_VECTOR_TO_NON_PLAYER);
    let nome_e: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_STAFF_NAME_EQUALS);
    let norm_fisicos: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_PHYSICAL_ATTRIBUTES);
    let norm_jogador: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES);
    let norm_nao_jogador: unsafe extern "C" fn(usize) = core::mem::transmute(FN_STAFF_NORMALISE_NON_PLAYER_ATTRIBUTES);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let w = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let signed_byte_at = |a: usize| *(a as *const i8);
    let b = |a: usize| *(a as *const u8);
    let wd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let ww = |a: usize, v: i16| core::ptr::write_unaligned(a as *mut i16, v);
    let wb = |a: usize, v: u8| *(a as *mut u8) = v;
    // o sorteio em byte (al) e em palavra (ax)
    let s8 = |n: i32| sorteio(n) as i8;
    let s16 = |n: i32| sorteio(n) as i16;
    let mut lixo = [[0u32; 2]; 12]; // as datas de passagem da pilha (ninguém as lê depois)
    let l = |k: usize, t: &mut [[u32; 2]; 12]| t[k].as_mut_ptr() as usize;

    let cent = gi(0xae2364) / 100;
    texto(0xde1f64, 0x9b6414);
    mostrar(G_SCREEN_MANAGER, 0xde1f64, 0, 100, -1);
    let mut v1 = [0u8; 189];
    let mut v2 = [0u8; 189];
    clear_attr_vector(v1.as_mut_ptr() as usize);
    clear_attr_vector(v2.as_mut_ptr() as usize);
    let (mut barra, mut e0c, mut e10) = (0i8, 0i32, -0x7961i32);
    let (mut e19, mut e05, mut e11, mut e1a) = (0u8, 0u8, 0u8, 0u8);

    let mut i = 0i32;
    let mut p = rd(G_STAFF_TABLE);
    while i < gi(0xae2364) {
        // o nome comum
        if gi(p + 0xc) == -2 {
            let mut k = gi(0x9b59b4);
            if k == -1 {
                let n = gi(0xae23a0);
                let base = rd(G_COMMON_NAME_TABLE);
                let mut j = 0i32;
                let mut achou = false;
                while j < n {
                    if b(base.wrapping_add((j as usize).wrapping_mul(0x3c))) == 0 {
                        *(G_EMPTY_COMMON_NAME_INDEX as *mut i32) = j;
                        achou = true;
                        break;
                    }
                    j += 1;
                }
                k = if achou { j } else { sorteio(n) };
            }
            wd(p + 0xc, rd(G_COMMON_NAME_TABLE).wrapping_add((k as usize).wrapping_mul(0x3c)));
        }
        if gi(p + 4) == -2 {
            wd(p + 4, primeiro(rd(p + 0x1a)));
        }
        if gi(p + 8) == -2 {
            wd(p + 8, segundo(rd(p + 0x1a)));
        }
        // a entrada no clube (+0x3e)
        if rd(p + 0x39) == 0 {
            copiar(p + 0x3e, l(0, &mut lixo), data);
        }
        let ano = w(p + 0x40) as u16;
        if ano < 0x7b2 {
            if rd(p + 0x39) != 0 {
                acertar(p + 0x3e, 1, 0, 0x7cc, -1);
            } else {
                ww(p + 0x3e, w(data));
                ww(p + 0x40, w(data + 2));
            }
        } else if ano == 0x7cc && w(p + 0x3e) == 1 && rd(p + 0x39) != 0 {
            let nac = rd(rd(p + 0x39) + 0x53);
            if nac != 0 && gi(rd(nac + 0x71)) == gi(KEY_CONTINENT_EUROPE) && comunitaria(p, nac) == 0 {
                let a = w(data + 2);
                let r = sorteio(2);
                let d = data_nova(l(1, &mut lixo), 1, (a as i32 - r - 1) as u32);
                copiar(p + 0x3e, l(2, &mut lixo), d);
            }
        }
        if rd(p + 0x39) != 0 && rd(rd(p + 0x39) + 0x53) == 0 {
            wd(p + 0x39, 0);
        }
        let d = semanas_antes(G_GAME_START_DATE, l(3, &mut lixo), 5);
        let (dd, da, pa) = (w(d), w(d + 2) as u16, w(p + 0x40) as u16);
        if da < pa || (da == pa && dd < w(p + 0x3e)) {
            let d = semanas_antes(G_GAME_START_DATE, l(4, &mut lixo), 5);
            copiar(p + 0x3e, l(5, &mut lixo), d);
        }
        for &(o, k) in &[(0x46usize, 6usize), (0x31, 8)] {
            let a = w(p + o + 2) as u16;
            if a < 0x7cf || a > 0x7da {
                let d = data_nova(l(k, &mut lixo), 0x1f, 0x76c);
                if w(d + 2) != w(p + o + 2) || w(d) != w(p + o) {
                    let d = data_nova(l(k + 1, &mut lixo), 0x1f, 0x76c);
                    copiar(p + o, l(10, &mut lixo), d);
                }
            }
        }
        // a barra, a cada 1%
        if e0c < cent {
            e0c += 1;
        } else {
            e0c = 0;
            barra = barra.wrapping_add(1);
            texto(0xde1f64, 0x9b6414);
            mostrar(G_SCREEN_MANAGER, 0xde1f64, barra as i32, 100, -1);
        }
        // os atributos de pessoa: −1 sorteia 5..20; fora de 1..20, o menor (min) ou a média de dois sorteios
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
        let v = signed_byte_at(p + 0x5d);
        if v < 1 || v > 20 {
            let n = if v == -1 {
                s8(0xb).wrapping_add(10)
            } else {
                let a = s8(0xb).wrapping_add(5);
                let c = s8(0xb).wrapping_add(5);
                ((c as i32 + a as i32) / 2) as i8
            };
            wb(p + 0x5d, n as u8);
        }
        // o ajuste pela região da nação (+0x76)
        let nac = rd(p + 0x1a);
        if nac != 0 {
            let menos = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_sub(sorteio(n) as u8));
            let mais = |o: usize, n: i32| wb(p + o, b(p + o).wrapping_add(sorteio(n) as u8));
            let t = signed_byte_at(nac + 0x76) as i32;
            if (t - 4) as u32 <= 9 {
                match t {
                    6 | 8 => menos(0x5c, 10),
                    4 | 0xb | 0xc | 0xd => {
                        if t != 0xb && t != 0xd {
                            menos(0x5c, 0xf);
                            menos(0x5d, 0xf);
                        }
                        if t != 0xd {
                            mais(0x5c, 8);
                            mais(0x5d, 8);
                        }
                        menos(0x5c, 5);
                        menos(0x5d, 5);
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
        if rd(p + 0x1a) == 0 {
            let s = rd(p + 0x24);
            let c = rd(p + 0x39);
            if s != 0 && rd(s + 0x53) != 0 {
                wd(p + 0x1a, rd(s + 0x53));
            } else if c != 0 && rd(c + 0x53) != 0 {
                wd(p + 0x1a, rd(c + 0x53));
            } else {
                let r = sorteio(gi(0xae2350));
                wd(p + 0x1a, rd(G_NATION_TABLE).wrapping_add((r as usize).wrapping_mul(0x122)));
            }
        }

        let j = rd(p + 0x61);
        if j != 0 {
            // ---- o jogador: habilidades (+0xb, +9, +0xd), potencial (+7) e atual (+5) ----
            let ca0 = w(j + 5);
            let mut e24 = w(j + 7);
            if sorteio(2000) == 0 {
                e24 = -2;
            } else if sorteio(0x2ee) == 0 {
                e24 = -1;
            } else if sorteio(100) == 0 {
                e24 = 0;
            }
            let mut e34 = w(j + 0xd);
            let mut e2c = w(j + 9);
            let mut e20 = w(j + 0xb);
            wb(j + 0x45, 10);
            let nac = rd(p + 0x1a);
            let mut rn = signed_byte_at(nac + 0x8e);
            if rn == 0 {
                formatar(0xde1f64, 0x9b63f0, w(nac + 0x8e) as i32);
                rn = s8(10).wrapping_add(1);
            }
            let rc: i8 = if rd(p + 0x39) == 0 {
                s8(2).wrapping_add(1)
            } else {
                let v = signed_byte_at(rd(p + 0x39) + 0x80);
                if v != 0 { v } else { s8(4).wrapping_add(1) }
            };
            let e30 = rc as i32;
            let di = ((((e30 * 3 + rn as i32) >> 2) * 10) as i16).min(0x50);
            if !in_range(e20, 1, 200) {
                let c = rd(p + 0x39);
                let mut x: i16 = if in_range(e2c, 1, 200) && c != 0 && rd(p + 0x1a) == rd(c + 0x53) {
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
                if rd(p + 0x39) == 0 {
                    x = free_agent_adjust(p, x);
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
                if rd(p + 0x39) == 0 {
                    x = free_agent_adjust(p, x);
                }
                e2c = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            if !in_range(e34, 1, 200) {
                let mut x: i16 = if in_range(e2c, 1, 200) {
                    if e2c > 100 { match_pl_ftol(e2c as f64 * read_f64_at(0x9689e0)) as i16 } else { s16(3).wrapping_add(1) } // 0x9689e0 = 0.25
                } else if in_range(e20, 1, 200) {
                    match_pl_ftol(e20 as f64 * read_f64_at(if e20 < 0xaf { 0x968f20 } else { 0x9689e0 })) as i16
                } else if in_range(ca0, 1, 200) {
                    let t = match_pl_ftol(ca0 as f64 * read_f64_at(0x9689e0)); // 0x9689e0 = 0.25
                    let r = sorteio(t);
                    s16(r).wrapping_add(1)
                } else {
                    ((di as i32 / 3) as i16).min(5)
                };
                if rd(p + 0x39) == 0 {
                    x = free_agent_adjust(p, x);
                }
                e34 = if x < 1 { 1 } else if x > 200 { 200 } else { x };
            }
            let c = rd(p + 0x39);
            if c == 0 || rd(c + 0x53) == rd(p + 0x1a) {
                e2c = e20;
            }
            // a idade (nascimento sorteado fora de 14..50)
            let mut idade = age_at_date(p, data);
            if idade < 0xe || idade > 0x32 {
                let a18 = w(p + 0x18);
                let ano: u32 = if a18 > 0x76c { a18 as u16 as u32 } else if rd(p + 0x69) != 0 { 0x7af } else { 0x7bc };
                let mes = sorteio(0xb);
                let dia = sorteio(0x1b) + 1;
                acertar(p + 0x10, dia as u32, mes as u32, ano, -1);
                idade = age_at_date(p, data);
            }
            ww(p + 0x18, idade as i16);
            // o potencial (+7)
            let teto = |mut s: i16, ca0: i16| {
                if ca0 > s {
                    s = ca0;
                }
                if s < 10 { 10 } else if s > 200 { 200 } else { s }
            };
            if e24 == -2 {
                let mut s = if sorteio(10) != 0 { s16(0x51).wrapping_add(0x78) } else { s16(0x29).wrapping_add(0xa0) };
                if sorteio(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0xa5 {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = teto(s, ca0);
            } else if e24 == -1
                || (e24 > 0x8c && (idade as i32) < sorteio(0x1e) && ca0 < 0xaa && sorteio(0x271a - e30) == 0)
            {
                let mut s = if sorteio(10) != 0 { s16(0x51).wrapping_add(0x50) } else { s16(0x51).wrapping_add(0x78) };
                if sorteio(0x14) == 0 {
                    s = s.wrapping_sub(s16(0x32));
                } else if s > 0x8c {
                    s = s.wrapping_sub(s16(0x1e));
                }
                e24 = teto(s, ca0);
            } else if !in_range(e24, 1, 200) || (idade < 0x19 && sorteio(0x19) == 0) {
                let mut k: i8 = if sorteio(100) == 0 {
                    s8(0x15)
                } else if sorteio(2) == 0 {
                    s8(6).wrapping_add(6)
                } else if sorteio(2000) == 0 {
                    s8(0x14).wrapping_add(1)
                } else {
                    s8(7).wrapping_add(4)
                };
                if k < 1 || k > 20 {
                    k = s8(10).wrapping_add(3);
                }
                let r = s16(0x3c);
                let s = r.wrapping_add(w((0x9b5988isize + k as isize * 2) as usize).wrapping_sub(0x1e));
                e24 = teto(s, ca0);
            }
            // a atual (+5)
            let mut ca = ca0;
            if !in_range(ca0, 1, 200) {
                let k = s8(10).wrapping_add(0x14);
                let pa = e24 as i32;
                let a = s16(pa);
                let a = if a < 5 { 5 } else { a as i32 };
                let r = sorteio(((pa >> 2) * a) / 200);
                let y = (10000 - e30) / 200;
                let c16 = (r + y * 2 + 3) as i16;
                let f = ((k as i32) - (idade as i32)).abs().max(3);
                let v = pa - (((c16 as i32 * f) / 5) as i16 as i32);
                ca = if v > 200 { 200 } else if (v as i16) >= 1 { v as i16 } else { 1 };
                if ca == 1 && sorteio(10) != 0 {
                    ca = s16(pa / 5).wrapping_add(1);
                }
            }
            if rd(p + 0x39) == 0 && (b(p + 0x22) == 0 || age_at_date(p, data) > 0x20) {
                let r = sorteio(6); // a original guarda ca em [0x20] só para o fild
                let f = match_pl_ftol(ca as f64 * read_f64_at(0x969518)); // 0x969518 = -0.1
                ca = ca.wrapping_add(r.wrapping_mul(f) as i16);
                ca = if ca < 1 { 1 } else if ca > 200 { 200 } else { ca };
            }
            if ca > e24 {
                ca = e24.wrapping_sub(s16(0x28));
                if ca < 1 {
                    ca = s16(5).wrapping_add(1);
                }
            }
            ww(j + 5, ca);
            ww(j + 7, e24);
            ww(j + 0xb, e20.wrapping_mul(50));
            ww(j + 9, e2c.wrapping_mul(50));
            ww(j + 0xd, e34.wrapping_mul(50));
            wb(rd(G_STAFF_STATE_TABLE).wrapping_add((gi(p) as usize).wrapping_mul(0x4f)) + 9, 0x9c);
            // ---- as posições (células 0..8) e os lados (9..11) ----
            vet_j(j, p, v1.as_mut_ptr() as usize, &mut e19, &mut e05, &mut e11);
            if attr_cell_value(&v1, 0) > 0xf && attr_cell_value(&v1, 0) < 0x14 {
                set_attr_cell_value(&mut v1, 0, 0x14);
                for k in [1isize, 2, 3, 4, 5, 6, 10, 11, 9] {
                    set_attr_cell_value(&mut v1, k, 1);
                }
            }
            let rf = || (s8(6).wrapping_add(0xf)) as i16;
            // (a, c) = os valores de partida das células 5 e 4 no fim das posições
            let mut ac: Option<(i16, i16)> = None;
            let mut melhor: i8 = -1;
            let mut tem20 = false;
            for k in 0..8isize {
                let a = attr_cell_value(&v1, k);
                if a == 0x14 {
                    tem20 = true;
                    break;
                }
                if a > 5 && a < 0x14 && (melhor == -1 || a > attr_cell_value(&v1, melhor as isize)) {
                    melhor = k as i8;
                }
            }
            if !tem20 {
                if melhor > -1 {
                    set_attr_cell_value(&mut v1, melhor as isize, 0x14);
                } else if sorteio(0x14) == 0 {
                    set_attr_cell_value(&mut v1, 0, 0x14);
                    for k in [1isize, 2, 3, 5, 6, 10, 11, 9] {
                        set_attr_cell_value(&mut v1, k, 1);
                    }
                    ac = Some((1, 1));
                } else {
                    let r = s16(0xb6);
                    // cada faixa do sorteio: as células sorteadas (15..20, nesta ordem) e as que viram 20
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
                // os lados: direito (9), esquerdo (10), centro (11)
                if attr_cell_value(&v1, 0) != 0x14 {
                    let mut lados = true;
                    if attr_cell_value(&v1, 1) >= 0xf {
                        set_attr_cell_value(&mut v1, 11, 0x14);
                    } else if attr_cell_value(&v1, 7) >= 0xf && attr_cell_value(&v1, 9) < 0xf && attr_cell_value(&v1, 10) < 0xf {
                        if sorteio(10) >= 3 {
                            set_attr_cell_value(&mut v1, 9, 0x14);
                            lados = false;
                        } else {
                            set_attr_cell_value(&mut v1, 10, 0x14);
                        }
                    }
                    while lados {
                        if attr_cell_value(&v1, 9) >= 0x14 || attr_cell_value(&v1, 10) >= 0x14 || attr_cell_value(&v1, 11) >= 0x14 {
                            break;
                        }
                        if sorteio(0xb) < 4 || sorteio(attr_cell_value(&v1, 11) as i32) > 5 {
                            set_attr_cell_value(&mut v1, 11, 0x14);
                        }
                        if sorteio(0xb) > 8 || sorteio(attr_cell_value(&v1, 10) as i32) > 5 {
                            set_attr_cell_value(&mut v1, 10, 0x14);
                        }
                        if sorteio(0xb) < 4 || sorteio(attr_cell_value(&v1, 9) as i32) > 5 {
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
            // as células 12 e 13
            let um_a_vinte = || (s8(sorteio(0x14)).wrapping_add(1)) as i16;
            let mut a = attr_cell_value(&v1, 12);
            let mut c = attr_cell_value(&v1, 13);
            let mut conferir_c = true;
            if a == 0x14 || c == 0x14 {
            } else if in_range(a, 1, 20) {
                if in_range(c, 1, 20) && c <= a {
                    a = 0x14;
                    set_attr_cell_value(&mut v1, 12, a);
                } else {
                    c = 0x14;
                    set_attr_cell_value(&mut v1, 13, c);
                    conferir_c = false;
                }
            } else if in_range(c, 1, 20) {
                a = 0x14;
                set_attr_cell_value(&mut v1, 12, a);
            } else if attr_cell_value(&v1, 10) > attr_cell_value(&v1, 9) {
                set_attr_cell_value(&mut v1, 12, 0x14);
                c = um_a_vinte();
                a = attr_cell_value(&v1, 12);
                set_attr_cell_value(&mut v1, 13, c);
            } else {
                set_attr_cell_value(&mut v1, 13, 0x14);
                let n = um_a_vinte();
                c = attr_cell_value(&v1, 13);
                set_attr_cell_value(&mut v1, 12, n);
                a = n;
            }
            if conferir_c && !in_range(c, 1, 20) {
                c = um_a_vinte();
                a = attr_cell_value(&v1, 12);
                set_attr_cell_value(&mut v1, 13, c);
            }
            if !in_range(a, 1, 20) {
                let n = um_a_vinte();
                c = attr_cell_value(&v1, 13);
                set_attr_cell_value(&mut v1, 12, n);
                a = n;
            }
            if a != 0x14 && c != 0x14 {
                if sorteio(4) != 0 {
                    a = attr_cell_value(&v1, 12);
                    set_attr_cell_value(&mut v1, 13, 0x14);
                } else {
                    a = 0x14;
                    set_attr_cell_value(&mut v1, 12, a);
                }
            }
            // ---- os atributos fora de 1..20 (da célula 12 em diante): pelo molde ----
            let mut fora = false;
            if a != -0x7f {
                let mut k = 12isize;
                loop {
                    let x = attr_cell_value(&v1, k);
                    if x < 1 || x > 20 {
                        fora = true;
                        break;
                    }
                    k += 1;
                    if attr_cell_value(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if fora {
                let mut molde = 0usize;
                let mut cont = 0xfai32;
                loop {
                    let r = sorteio(gi(0xae2364));
                    let q = rd(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(110));
                    let cj = rd(q + 0x61);
                    if cj != 0 && in_range(w(cj + 5), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b8) != 0 {
                            clear_attr_vector(0xae2044);
                            clear_attr_vector(0xae2114);
                            wb(0x9b59b8, 0);
                        }
                        vet_j(j, p, 0xae2044, &mut e1a, core::ptr::null_mut(), core::ptr::null_mut());
                        vet_j(cj, q, 0xae2114, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                        let dif = (w(j + 5) as i32 - w(cj + 5) as i32).abs();
                        let mut esc = match_pl_ftol(dif as f64 * read_f64_at(0x968e18)); // 0x968e18 = 2.5
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
                    cont -= 1; // os 250 sorteados
                    if cont == 0 {
                        break;
                    }
                }
                let ini = e19 as i8 as isize;
                if molde == 0 {
                    let mut k = ini;
                    while attr_cell_value(&v1, k) != -0x7f {
                        let x = attr_cell_value(&v1, k);
                        if x < 1 || x > 20 {
                            let r = s8(7);
                            let c = r.wrapping_add(((w(j + 5) as i32 / 10) as i8).wrapping_sub(3));
                            let n = if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 };
                            set_attr_cell_value(&mut v1, k, n);
                            if attr_cell_weight(&v1, k) == reroll_weight(&v1) {
                                set_attr_cell_value(&mut v1, k, s16(5).wrapping_add(1));
                            }
                        }
                        k += 1;
                    }
                } else {
                    vet_j(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
                    let mut hj = w(j + 5);
                    if !in_range(hj, 1, 200) {
                        hj = s16(100).wrapping_add(1);
                    }
                    let mut hm = w(molde + 5);
                    if !in_range(hm, 1, 200) {
                        hm = hj;
                    }
                    let mut k = ini;
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
                                let c = r.wrapping_add(((w(j + 5) as i32 / 10) as i8).wrapping_sub(5));
                                if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 }
                            };
                            set_attr_cell_value(&mut v1, k, n);
                            let o = k * 3;
                            if o == 0x9c || o == 0x9f {
                                let x = attr_cell_value(&v1, k);
                                if x < 0xc {
                                    set_attr_cell_value(&mut v1, k, x.wrapping_add(s8(0xd - x as i32) as i16));
                                }
                            } else if (o == 0x6c || o == 0x6f || o == 0x78) && idade < 0x16 && attr_cell_value(&v1, k) > 0xf {
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
            de_vet_j(j, p, v1.as_mut_ptr() as usize);
            if b(j + 0xf) == 0x14 && signed_byte_at(j + 0x2e) < 8 {
                wb(j + 0x2e, (sorteio(6) as u8).wrapping_add(5));
            }
        } else {
            // ---- só não jogador: a idade fora de 29..75 sorteia o nascimento ----
            let idade = age_at_date(p, data);
            if idade < 0x1d || idade > 0x4b {
                let a18 = w(p + 0x18);
                let ano: u32 = if a18 > 0x76c && a18 < 0x7af { a18 as u16 as u32 } else { (sorteio(0x1a) + 0x794) as u32 };
                let mes = sorteio(0xb);
                let dia = sorteio(0x1b) + 1;
                acertar(p + 0x10, dia as u32, mes as u32, ano, -1);
            }
            ww(p + 0x18, age_at_date(p, data) as i16);
        }

        let nj = rd(p + 0x69);
        if nj != 0 {
            // ---- o não jogador: as reputações (+0xc, +0xa, +8) ----
            let a = w(nj + 0xc).wrapping_mul(50);
            let di = w(nj + 0xa).wrapping_mul(50);
            let bp = w(nj + 8).wrapping_mul(50);
            ww(nj + 0xa, di);
            ww(nj + 0xc, a);
            ww(nj + 8, bp);
            let rep = |x: usize| w(x + 0x80);
            let vezes = |x: usize, k: usize| match_pl_ftol(rep(x) as f64 * read_f64_at(k)) as i16;
            if !in_range(a, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 0xc, if c != 0 && rep(c) > 0x1676 {
                    vezes(c, 0x969510)
                } else if s != 0 {
                    vezes(s, 0x968968)
                } else {
                    1
                });
            }
            let x = w(nj + 0xc);
            let bx = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj + 0xc, bx);
            if !in_range(di, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 0xa, if in_range(bp, 1, 10000) {
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
            let x = w(nj + 0xa);
            let di2 = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            ww(nj + 0xa, di2);
            if !in_range(bp, 1, 10000) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 8, if in_range(di2, 1, 10000) {
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
            let x = w(nj + 8);
            ww(nj + 8, if x < 1 { 1 } else if x > 10000 { 10000 } else { x });
            let c = rd(p + 0x39);
            if c == 0 || rd(c + 0x53) == rd(p + 0x1a) {
                ww(nj + 8, di2);
            }
            if !in_range(w(nj + 4), 1, 200) {
                let c = rd(p + 0x39);
                let s = rd(p + 0x24);
                ww(nj + 4, if c != 0 {
                    (rep(c) as i32 / 50) as i16
                } else if s != 0 {
                    (rep(s) as i32 / 50) as i16
                } else {
                    s16(100).wrapping_add(1)
                });
            }
            if !in_range(w(nj + 6), 1, 200) {
                let r = s16(200).wrapping_add(1);
                let x = w(nj + 4);
                ww(nj + 6, if x > r { x } else { r });
            }
            let nac = rd(p + 0x1a);
            if nac != 0 && b(nj + 0x13) == 0 {
                let id = gi(nac);
                let casa = [KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_WALES, KEY_NATION_NORWAY].iter().any(|&k| id == gi(k));
                let v = if casa {
                    let x = 0xdu8.wrapping_sub(sorteio(0xe) as u8);
                    x.wrapping_add(sorteio(8) as u8)
                } else {
                    let x = 5u8.wrapping_sub(sorteio(5) as u8);
                    x.wrapping_add(sorteio(7) as u8)
                };
                wb(nj + 0x13, v);
            }
            // ---- os atributos fora de 1..20: pelo molde ----
            vet_nj(nj, p, v1.as_mut_ptr() as usize, core::ptr::null_mut());
            let mut fora = false;
            if attr_cell_value(&v1, 0) != -0x7f {
                let mut k = 0isize;
                loop {
                    let x = attr_cell_value(&v1, k);
                    if x < 1 || x > 20 {
                        fora = true;
                        break;
                    }
                    k += 1;
                    if attr_cell_value(&v1, k) == -0x7f {
                        break;
                    }
                }
            }
            if fora {
                let c = rd(p + 0x39);
                if b(p + 0x3d) == 5 && c != 0 && rep(c) > 0x186a {
                    // o treinador de clube grande
                    let e_360 = || rd(p + 0x1a) != 0 && gi(rd(p + 0x1a)) == gi(KEY_NATION_ITALY);
                    let fora20 = |o: usize| signed_byte_at(nj + o) < 1 || signed_byte_at(nj + o) > 20;
                    if fora20(0x13) {
                        wb(nj + 0x13, (if e_360() { s8(0x10).wrapping_add(1) } else { s8(0xe).wrapping_add(7) }) as u8);
                    }
                    if fora20(0xe) {
                        let c = rd(p + 0x39);
                        wb(nj + 0xe, (if rep(c) > 0x1e46 {
                            if e_360() { s8(4).wrapping_add(0xd) } else { s8(8).wrapping_add(0xd) }
                        } else {
                            s8(9).wrapping_add(0xc)
                        }) as u8);
                    }
                    if fora20(0x1b) {
                        wb(nj + 0x1b, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(0x1f) {
                        wb(nj + 0x1f, s8(0xd).wrapping_add(8) as u8);
                    }
                    if fora20(0x21) {
                        let c = rd(p + 0x39);
                        wb(nj + 0x21, (if rep(c) > 0x1e46 {
                            if e_360() { s8(7).wrapping_add(0xe) } else { s8(9).wrapping_add(0xc) }
                        } else {
                            s8(0xb).wrapping_add(0xa)
                        }) as u8);
                    }
                }
                let mut molde = 0usize;
                let mut melhor = e10;
                let mut cont = 0xfai32;
                loop {
                    let r = sorteio(gi(0xae2364));
                    let q = rd(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(110));
                    let cn = rd(q + 0x69);
                    if cn != 0 && in_range(w(cn + 4), 1, 200) {
                        let mut e35 = 0i8;
                        if b(0x9b59b9) != 0 {
                            clear_attr_vector(0xae18bc);
                            clear_attr_vector(0xae2238);
                            wb(0x9b59b9, 0);
                        }
                        vet_nj(nj, p, 0xae18bc, core::ptr::null_mut());
                        vet_nj(cn, q, 0xae2238, core::ptr::null_mut());
                        let dif = (w(nj + 4) as i32 - w(cn + 4) as i32).abs();
                        let mut esc = match_pl_ftol(dif as f64 * read_f64_at(0x968e18)); // 0x968e18 = 2.5
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
                        if esc < melhor || molde == 0 {
                            molde = cn;
                            melhor = esc;
                        }
                    }
                    cont -= 1;
                    if cont == 0 {
                        break;
                    }
                }
                e10 = melhor;
                if molde != 0 {
                    vet_nj(molde, p, v2.as_mut_ptr() as usize, core::ptr::null_mut());
                    let mut hn = w(nj + 4);
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
                                    r.wrapping_add(((w(nj + 4) as i32 / 10) as i8).wrapping_sub(3))
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
                            let c = r.wrapping_add(((w(nj + 4) as i32 / 10) as i8).wrapping_sub(3));
                            set_attr_cell_value(&mut v1, k, if c < 1 { s8(8).wrapping_add(1) as i16 } else if c > 20 { 0x14 } else { c as i16 });
                        }
                        k += 1;
                        if attr_cell_value(&v1, k) == -0x7f {
                            break;
                        }
                    }
                }
            }
            de_vet_nj(nj, p, v1.as_mut_ptr() as usize);
            // ---- o cargo: olheiro (9), fisioterapeuta (8), treinador (10) ----
            let ate = |o: usize, max: i8, n: i32, base: u8| {
                if signed_byte_at(nj + o) > max {
                    wb(nj + o, (sorteio(n) as u8).wrapping_add(base));
                }
            };
            let cargo = b(p + 0x3d);
            if cargo == 9 && sorteio(100) != 0 {
                let a17 = signed_byte_at(nj + 0x17);
                wb(nj + 0xf, 1);
                wb(nj + 0x1e, 1);
                if a17 < 5 {
                    wb(nj + 0x17, (sorteio(5) as u8).wrapping_add(4));
                }
                if signed_byte_at(nj + 0x18) < 4 {
                    wb(nj + 0x18, (sorteio(5) as u8).wrapping_add(3));
                }
                ate(0x10, 8, 8, 1);
                ate(0x11, 8, 8, 1);
                ate(0x12, 5, 5, 1);
                ate(0x13, 5, 5, 1);
                ate(0x1a, 5, 5, 1);
            } else if cargo == 8 && sorteio(100) != 0 {
                wb(nj + 0xf, 1);
                wb(nj + 0x1e, 1);
            } else if cargo == 0xa && sorteio(100) != 0 {
                let a1e = signed_byte_at(nj + 0x1e);
                wb(nj + 0xf, 1);
                if a1e < 5 {
                    wb(nj + 0x1e, (sorteio(10) as u8).wrapping_add(4));
                }
                for o in [0x10usize, 0x11, 0x12, 0x13, 0x17, 0x18, 0x1a] {
                    ate(o, 5, 5, 1);
                }
            }
            // ---- os nomes famosos (+0x43) ----
            let nome = |s: usize| nome_e(p, s) != 0;
            if nome(0x9b63e0) {
                wb(nj + 0x43, 3);
                wb(nj + 0x13, 7);
                wb(nj + 0xe, 0x11);
            } else if nome(0x9b63d0) || nome(0x9b63c4) || nome(0x9b63b4) || nome(0x9b63a8) {
                wb(nj + 0x43, 3);
            } else if nome(0x9b6398) || nome(0x9b6388) || nome(0x9b6378) || nome(0x9b6368) || nome(0x9b635c) {
                wb(nj + 0x43, 2);
            } else if nome(0x9b634c) {
                wb(nj + 0x43, 6);
            } else {
                let x = signed_byte_at(nj + 0x43);
                if x == 0 || x >= 10 {
                    let r = sorteio(0x1a);
                    if (r as u32) <= 0x19 {
                        const TABELA: [u8; 26] = [0, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 8];
                        wb(nj + 0x43, TABELA[r as usize]);
                    }
                }
            }
            for k in 0..8 {
                wd(nj + 0x23 + 4 * k, 0);
            }
            // ---- o cargo que falta ----
            if b(p + 0x3d) == 0 && b(p + 0x28) == 0 {
                let (c21, c1b, a10, a11) = (signed_byte_at(nj + 0x21), signed_byte_at(nj + 0x1b), signed_byte_at(nj + 0x10), signed_byte_at(nj + 0x11));
                wb(p + 0x3d, if rd(p + 0x61) != 0 {
                    0xb
                } else if c21 > 10 && c1b > 10 {
                    5
                } else if (a10 > 6 || a11 > 6) && c1b > 10 && c21 > 10 {
                    6
                } else if a10 <= 6 && a11 <= 6 {
                    9 + (signed_byte_at(nj + 0x1e) > 10) as u8
                } else {
                    8
                });
            }
        }
        ww(p + 0x18, age_at_date(p, data) as i16);
        i += 1;
        p += 110;
    }

    // ---- a segunda volta: os vetores relativos à habilidade e as normalizações ----
    let mut v3 = [0u8; 189];
    let mut q = rd(G_STAFF_TABLE);
    let mut k = 0i32;
    while k < gi(0xae2364) {
        let jj = rd(q + 0x61);
        let nn = rd(q + 0x69);
        if jj != 0 {
            clear_attr_vector(v3.as_mut_ptr() as usize);
            let (mut fim_l, mut fim_p, mut fim_q) = (0u8, 0u8, 0u8);
            vet_j(jj, q, v3.as_mut_ptr() as usize, &mut fim_l, &mut fim_p, &mut fim_q);
            let a = fim_l.wrapping_add(2) as i8;
            let f = fim_q as i8;
            if a < f {
                for c in a as isize..f as isize {
                    let x = attr_cell_value(&v3, c).wrapping_mul(10).wrapping_sub(w(jj + 5));
                    let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                    set_attr_cell_value(&mut v3, c, (x as i32 / 2) as i16);
                }
            }
            de_vet_j(jj, q, v3.as_mut_ptr() as usize);
            norm_fisicos(q);
            norm_jogador(q);
        }
        if nn != 0 {
            clear_attr_vector(v3.as_mut_ptr() as usize);
            let mut fim = 0u8;
            vet_nj(nn, q, v3.as_mut_ptr() as usize, &mut fim);
            let mut c = fim as i8;
            while attr_cell_value(&v3, c as isize) != -0x7f {
                let x = attr_cell_value(&v3, c as isize).wrapping_mul(10).wrapping_sub(w(nn + 4));
                let x = if x > 200 { 200 } else if x < -200 { -200 } else { x };
                set_attr_cell_value(&mut v3, c as isize, (x as i32 / 2) as i16);
                c = c.wrapping_add(1);
            }
            de_vet_nj(nn, q, v3.as_mut_ptr() as usize);
            norm_nao_jogador(q);
        }
        k += 1;
        q += 110;
    }
}



/// strcpy: copies a null-terminated string from origem to destino.
///
/// [pt] strcpy (o `repne scasb` e os `rep movsd`/`rep movsb` da original).
pub(crate) unsafe fn copy_c_string(destino: usize, origem: usize) {
    let mut k = 0;
    loop {
        let c = *((origem + k) as *const u8);
        *((destino + k) as *mut u8) = c;
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// the person name by format (1..7) from the first, second and common names: "%s %s", "%c.%s", "%s, %s", "%s, %c", one
/// of them or the initials
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
pub(crate) unsafe extern "C" fn database_format_name_from_strings(nome: usize, sobrenome: usize, comum: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let f = forma as u8;
    if texto == 0 || (tamanho as u16 as i16) < 0x69 || !(1..=7).contains(&f) {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_STRINGS)(nome, sobrenome, comum, texto, tamanho, forma);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (a, b, c) = (nome, sobrenome, comum);
    let inicial = |t: usize| *(t as *const i8) as i32;
    *(texto as *mut u8) = 0;
    let os_dois = is_nonempty_text(a) && is_nonempty_text(b);
    // o que sobra: o primeiro cheio da lista
    let um_de = |lista: &[usize]| {
        if let Some(&t) = lista.iter().find(|&&t| is_nonempty_text(t)) {
            copy_c_string(texto, t);
        }
    };
    match f {
        7 => {
            if os_dois {
                sprintf(texto, 0x98f0b8, a, b); // "%s %s"
            } else {
                um_de(&[c, b, a]);
            }
        }
        6 => {
            if is_nonempty_text(c) {
                sprintf(texto, 0x99ec1c, inicial(c)); // "%c"
            } else if os_dois {
                sprintf(texto, 0x9b7418, inicial(a), inicial(b)); // "%c %c"
            } else if is_nonempty_text(b) {
                sprintf(texto, 0x99ec1c, inicial(b));
            } else if is_nonempty_text(a) {
                sprintf(texto, 0x99ec1c, inicial(a));
            }
        }
        5 => um_de(&[c, b, a]),
        _ => {
            if is_nonempty_text(c) {
                copy_c_string(texto, c);
            } else if os_dois {
                match f {
                    1 => { sprintf(texto, 0x98f0b8, a, b); } // "%s %s" (dbu)
                    2 => { sprintf(texto, 0x9b7410, inicial(a), b); } // "%c.%s"
                    3 => { sprintf(texto, 0x98f6ac, b, a); } // "%s, %s"
                    _ => { sprintf(texto, 0x9b7408, b, inicial(a)); } // "%s, %c"
                }
            } else {
                um_de(&[b, a]);
            }
        }
    }
    1
}


/// Os textos dos cargos 0..0x11 da 0x53ad00 (imediatos do código, pelos endereços no exe): curto minúsculo, curto
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
    (0x9b64c8, 0x9b64ec, 0x9b64a8, 0x9b64b8, 0), // retired player (sem gênero)
    (0x9b6460, 0x9b6484, 0x9b6450, 0x9b6450, 1), // pundit, "Media Pundit" nas duas
];

/// the translated job name (0..0x11), short or full, lower or upper case, in [0xae19b4]; the gender with language 4
/// Original: `database_get_job_name` (0x53ad00, Database.cpp, cdecl).
///
/// [pt] { 0x53ad00 } (cdecl, Database.cpp): `database_get_job_name(cargo, curto, maiuscula, genero)`: o nome traduzido
/// (0x66f4e0) do cargo (byte com sinal, 0..0x11 sem o 14) no texto [0xae19b4], que é o que devolve; curto ou inteiro e
/// minúsculo ou maiúsculo pelos dois inteiros. Com `genero` não nulo e o idioma (0x66f4d0) 4, grava o gênero: 4 para
/// diretor executivo, técnico, auxiliar, técnico dos reservas e olheiro; 1 para os outros; nada para o aposentado.
/// Cargo fora da faixa: 0, sem escrever. Nas telas do clube, de contratos, da pessoa e de busca, na mídia
/// (0x744f80, 0x74bbd0), nos regens (0x7a9650, 0x7aa4c0) e nas transferências (0x8d3520, 0x8dc370).
pub(crate) unsafe extern "C" fn database_get_job_name(cargo: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let j = cargo as u8 as i8;
    if !(0..=0x11).contains(&j) || j == 14 {
        return 0;
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (cm, cg, im, ig, g) = JOB_TITLE_TEXTS[j as usize];
    if curto != 0 {
        formatar(G_JOB_NAME_BUFFER, if maiuscula != 0 { cg } else { cm }, 0xad9c64usize);
    } else if j == 11 {
        formatar(G_JOB_NAME_BUFFER, if maiuscula != 0 { ig } else { im }, 0xad9c64usize); // "<%s - COMMENT - job name>"
    } else {
        formatar(G_JOB_NAME_BUFFER, if maiuscula != 0 { ig } else { im });
    }
    if g != 0 && genero != 0 && idioma() == 4 {
        *(genero as *mut u8) = g;
    }
    G_JOB_NAME_BUFFER
}


/// Os nomes dos idiomas 1..64 da 0x53cee0 (imediatos do código, pelos endereços no exe; o 34 repete o albanês),
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

/// the translated language name (1..64, else "Unknown") in [0xae21d4]
/// Original: `database_get_language_name` (0x53cee0, Database.cpp, cdecl).
///
/// [pt] { 0x53cee0 } (cdecl, Database.cpp): `database_get_language_name(idioma)`: o nome traduzido (0x66f4e0) do idioma
/// (byte com sinal, 1..64; fora disso "Unknown") no texto [0xae21d4] (100 bytes), que é o que devolve. Na busca
/// (0x813210) e na tela da pessoa (0x87dd80).
pub(crate) unsafe extern "C" fn database_get_language_name(idioma: u32) -> usize {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let x = idioma as u8 as i8;
    let s = if (1..=64).contains(&x) { LANGUAGE_NAMES[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)
    formatar(0xae21d4, s, 0xad9c64usize);
    0xae21d4
}



/// True if the text's first byte is 0.
pub(crate) unsafe fn is_empty_text(t: usize) -> bool {
    *(t as *const u8) == 0
}

/// strcat: appends a null-terminated string to destino.
///
/// [pt] strcat (os `repne scasb` e os `rep movsd`/`rep movsb` da original).
pub(crate) unsafe fn append_c_string(destino: usize, origem: usize) {
    let mut d = destino;
    while *(d as *const u8) != 0 {
        d += 1;
    }
    let mut k = 0;
    loop {
        let c = *((origem + k) as *const u8);
        *((d + k) as *mut u8) = c;
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// the national team club name by style (1..4): the name, "Reserves", "B"/"'B'", "U21"/"Under 21"/"Under 21's"/"Under
/// 21 Squad" (or 23) and "Squad"/"Main Squad"
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
pub(crate) unsafe extern "C" fn club_get_national_team_name(clube: usize, variante: u32, texto: usize, tamanho: i32, forma: u32, aspas: u32) -> u32 {
    let f = forma as u8 as i8;
    if tamanho < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return original_fn::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(IDX_CLUB_GET_NATIONAL_TEAM_NAME)(clube, variante, texto, tamanho, forma, aspas);
    }
    if !(1..=4).contains(&f) {
        return 1;
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let id = read_usize(clube) as i32;
    let nacoes = read_usize(G_NATION_COUNT) as i32;
    let clubes = read_usize(G_CLUB_COUNT) as i32;
    let (nome, g) = if f == 3 { (clube + 4, *((clube + club::GENDER_NAME) as *const i8) as i32) } else { (clube + club::SHORT_NAME, *((clube + club::SHORT_GENDER_NAME) as *const i8) as i32) };
    let formato = if id < clubes.wrapping_sub(nacoes.wrapping_mul(2)) {
        if variante == 0 {
            if f == 4 { Some(0x9b7634) } else { None } // Main Squad
        } else {
            Some([0x9b784c, 0x9b77ac, 0x9b784c, 0x9b770c][f as usize - 1]) // Reserves, Reserve, Reserves, Reserve Squad
        }
    } else if variante != 0 {
        Some(match (f == 4, aspas != 0) {
            (false, false) => 0x9b7814, // B
            (false, true) => 0x9b7830,  // 'B'
            (true, false) => 0x9b76c8,  // B Squad
            (true, true) => 0x9b76e8,   // 'B' Squad
        })
    } else if id < clubes.wrapping_sub(nacoes) {
        if f == 4 { Some(0x9b7658) } else { None } // Squad
    } else {
        let nacao = read_usize(read_usize(clube + club::NATION));
        let sub23 = *(nacao.wrapping_add(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x36)) as *const u8) != 0;
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
            formatar(texto, s, g, g, nome);
        }
        None => {
            *(texto as *mut u8) = 0;
            append_c_string(texto, nome);
        }
    }
    1
}


/// Os nomes dos 31 atributos de jogador da 0x5428a0 (imediatos do código, pelos endereços no exe): (abreviado, leva o
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

/// the translated name of the 31 player attributes, short or full; returns 1
/// Original: `database_get_player_attribute_name` (0x5428a0, Database.cpp, cdecl).
///
/// [pt] { 0x5428a0 } (cdecl, Database.cpp; o mapa parte a rotina em 0x542f48, que é só o meio dela: a fronteira real vai
/// até 0x5431e0): `database_get_player_attribute_name(atributo, texto, tamanho, abreviado)`: zera o texto e escreve o
/// nome traduzido (0x66f4e0) do atributo de jogador (1..0x1f, byte com sinal), abreviado (`abreviado` não nulo) ou
/// inteiro; devolve 1. Atributo fora da faixa, texto nulo ou tamanho (curto com sinal) < 100: a original (caixa e 0).
/// Nas telas do clube, de busca, da pessoa e de treino.
pub(crate) unsafe extern "C" fn database_get_player_attribute_name(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=0x1f).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_PLAYER_ATTRIBUTE_NAME)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *(texto as *mut u8) = 0;
    let (c, cv, l, lv) = PLAYER_ATTRIBUTE_NAMES[x as usize - 1];
    let (s, vazio) = if abreviado != 0 { (c, cv) } else { (l, lv) }; // (dbu)
    if vazio {
        formatar(texto, s, 0xad9c64usize);
    } else {
        formatar(texto, s);
    }
    1
}


/// Appends one position to the position text built by 0x53ee50: short or long form (translated via the global buffer
/// 0xde1f64) with a "/" separator, or a single letter only while the text is empty.
///
/// [pt] Uma posição no texto da 0x53ee50: forma 1 o abreviado (com o vazio 0xad9c64), 2 e 3 o inteiro, com a barra antes
/// quando o texto já tem algo; as outras formas, a letra só com o texto ainda vazio. O traduzido passa pelo texto
/// global [0xde1f64] (0x66f4e0), como na original.
pub(crate) unsafe fn append_position_text(buf: usize, f: u8, curto: usize, longo: usize, longo_vazio: bool, letra: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match f {
        1 => {
            if !is_empty_text(buf) {
                append_c_string(buf, 0x9b7630); // "/"
            }
            formatar(0xde1f64, curto, 0xad9c64usize);
            append_c_string(buf, 0xde1f64);
        }
        2 | 3 => {
            if !is_empty_text(buf) {
                append_c_string(buf, 0x9b7630);
            }
            if longo_vazio {
                formatar(0xde1f64, longo, 0xad9c64usize);
            } else {
                formatar(0xde1f64, longo);
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

/// As posições da 0x53ee50 (abreviado, inteiro, o inteiro leva o vazio, letra), imediatos do código.
pub(crate) const POSITION_GOALKEEPER: (usize, usize, bool, usize) = (0x9b7610, 0x9b7600, false, 0x9b760c); // GK, Goalkeeper, A
pub(crate) const POSITION_SWEEPER: (usize, usize, bool, usize) = (0x9b75e4, 0x9b75dc, false, 0x98b3f0); // SW, Sweeper, B
pub(crate) const POSITION_DEFENDER: (usize, usize, bool, usize) = (0x9b75c0, 0x9b75b0, false, 0x9b75bc); // D, Defender, C
pub(crate) const POSITION_DEFENSIVE_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7588, 0x9b756c, false, 0x9b7584); // DM, Defensive Midfielder, D
pub(crate) const POSITION_ATTACKING_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7544, 0x9b7528, false, 0x9b7540); // AM, Attacking Midfielder, F
pub(crate) const POSITION_MIDFIELDER: (usize, usize, bool, usize) = (0x9b7508, 0x9b74f8, false, 0x9b7504); // M, Midfielder, E
pub(crate) const POSITION_STRIKER: (usize, usize, bool, usize) = (0x9b74dc, 0x9b74d0, false, 0x9b74d8); // S, Striker, H
pub(crate) const POSITION_FORWARD: (usize, usize, bool, usize) = (0x9b74b4, 0x9b7484, true, 0x9b74b0); // F, Forward<%s>, G

/// the positions text from the ratings (15 to 10) and the sides (15), short ("D/DM RC"), full or as letters
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
pub(crate) unsafe extern "C" fn database_format_player_positions(jogador: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if texto == 0 || jogador == 0 || (tamanho as u16 as i16) < 0x1e {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_PLAYER_POSITIONS)(jogador, texto, tamanho, forma);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    *(texto as *mut u8) = 0;
    let f = forma as u8;
    let nota = |o: usize| *((jogador + o) as *const i8) as i32;
    let mut linha = [0u8; 0x7d0];
    let mut lados = [0u8; 0x7c0];
    let buf = linha.as_mut_ptr() as usize;
    let pos = |p: (usize, usize, bool, usize)| append_position_text(buf, f, p.0, p.1, p.2, p.3);
    let mut minimo: u8 = 0xf;
    loop {
        *(buf as *mut u8) = 0;
        let t = minimo as i32;
        if nota(0xf) >= t {
            pos(POSITION_GOALKEEPER);
        }
        if nota(0x10) >= t {
            pos(POSITION_SWEEPER);
        }
        if nota(0x11) >= t {
            pos(POSITION_DEFENDER);
        }
        if nota(0x12) >= t {
            pos(POSITION_DEFENSIVE_MIDFIELDER);
        } else if nota(0x14) >= t && nota(0x13) >= t {
            pos(POSITION_ATTACKING_MIDFIELDER);
        } else if nota(0x13) >= t {
            pos(POSITION_MIDFIELDER);
        }
        if nota(0x15) >= t {
            if nota(0x14) >= t || nota(0x17) >= t || nota(0x18) >= t || nota(0x1a) >= t {
                pos(POSITION_FORWARD);
            } else {
                pos(POSITION_STRIKER);
            }
        }
        if nota(0xf) >= t {
            // goleiro: só a linha (strcpy na original; o texto pode ser o próprio [0xde1f64])
            *(texto as *mut u8) = 0;
            append_c_string(texto, buf);
            return 1;
        }
        minimo = minimo.wrapping_sub(1);
        if !is_empty_text(buf) || minimo < 0xa {
            break;
        }
    }
    if is_empty_text(buf) {
        if nota(0x14) >= 0xf {
            *((jogador + 0x13) as *mut u8) = 0xf; // a original grava no jogador
        }
        if f == 2 || f == 3 {
            formatar(0xde1f64, POSITION_ATTACKING_MIDFIELDER.1);
        } else {
            formatar(0xde1f64, POSITION_ATTACKING_MIDFIELDER.0, 0xad9c64usize);
        }
        append_c_string(buf, 0xde1f64);
    }
    let lado = lados.as_mut_ptr() as usize;
    let mut volta: u8 = 0xf;
    loop {
        *(lado as *mut u8) = 0;
        for (o, curto, longo, letra) in [(0x17, 0x9b746c, 0x9b7464, 0x9b760c), (0x18, 0x9b744c, 0x9b7444, 0x98b3f0), (0x19, 0x9b7428, 0x9b7420, 0x9b75bc)] {
            if nota(o) < 0xf {
                continue;
            }
            match f {
                1 => {
                    formatar(0xde1f64, curto, 0xad9c64usize); // R, L, C
                    append_c_string(lado, 0xde1f64);
                }
                2 => {
                    if !is_empty_text(lado) {
                        append_c_string(lado, 0x9b7630);
                    }
                    formatar(0xde1f64, longo); // Right, Left, Centre
                    append_c_string(lado, 0xde1f64);
                }
                4 => {
                    if is_empty_text(lado) {
                        append_c_string(lado, letra);
                    }
                }
                _ => {}
            }
        }
        // a original repete a volta (de 15 a 10) enquanto os lados ficam vazios; as notas são as mesmas
        volta = volta.wrapping_sub(1);
        if !is_empty_text(lado) || volta < 0xa {
            break;
        }
    }
    if f == 2 && !is_empty_text(lado) {
        sprintf(texto, 0x9a3d8c, buf, lado); // "%s (%s)" (dbu)
    } else if !is_empty_text(lado) {
        sprintf(texto, 0x98f0b8, buf, lado); // "%s %s"
    } else {
        *(texto as *mut u8) = 0; // strcpy na original (o texto pode ser o próprio [0xde1f64])
        append_c_string(texto, buf);
    }
    1
}



/// Text pointer is non-null and the text is not empty.
///
/// [pt] Texto não nulo e não vazio (os `repne scasb` da original).
pub(crate) unsafe fn is_nonempty_text(t: usize) -> bool {
    t != 0 && *(t as *const u8) != 0
}

/// the first, second and common name records by index ([0xae23f0], [0xae23f4], [0xae23f8]; the nickname given by a
/// human instead of the common name) and the text by 0x53e2e0
/// Original: `database_format_name_from_ids` (0x53e140, Database.cpp, cdecl).
///
/// [pt] { 0x53e140 } (cdecl, Database.cpp): `database_format_name_from_ids(nome, sobrenome, comum, texto, tamanho, forma,
/// pessoa)`: os registros dos nomes (60 bytes) pelos índices nas tabelas [0xae23f0], [0xae23f4] e [0xae23f8] (−1 ou
/// menos: nulo) e o texto pela 0x53e2e0 (com −1 no fim), que é o que devolve. Com `pessoa` > −1 e [0xb63c98]: se um
/// humano deu apelido à pessoa ([0xae23bc] + 110 × pessoa; 0x600740) e ela confere com os nomes (0x53e990), o apelido
/// (0x6005e0) no lugar do nome comum. Texto nulo ou tamanho (palavra) < 0x69: a original (caixa). As locais de 0x200
/// bytes só servem à mensagem da caixa. As chamadas pelo endereço do jogo. ~126 chamadores (notícias, telas, contratos).
pub(crate) unsafe extern "C" fn database_format_name_from_ids(nome: i32, sobrenome: i32, comum: i32, texto: usize, tamanho: u32, forma: u32, pessoa: i32) -> u32 {
    if texto == 0 || (tamanho as i16) < 0x69 {
        return original_fn::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(IDX_DATABASE_FORMAT_NAME_FROM_IDS)(nome, sobrenome, comum, texto, tamanho, forma, pessoa);
    }
    let deu_apelido: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_HAS_NICKNAME);
    let confere: unsafe extern "C" fn(usize, i32, i32, i32, u32) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    let apelido: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_NICKNAME);
    let formatar: unsafe extern "C" fn(usize, usize, usize, usize, u32, u32, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_NAME_FROM_STRINGS);
    let registro = |tabela: usize, i: i32| if i > -1 { read_usize(tabela).wrapping_add((i as usize).wrapping_mul(60)) } else { 0 };
    let (n, s) = (registro(G_FIRST_NAME_TABLE, nome), registro(G_SECOND_NAME_TABLE, sobrenome));
    let apelidos = read_usize(G_HUMAN_MANAGER);
    let mut terceiro = None;
    if pessoa > -1 && apelidos != 0 {
        let p = read_usize(G_STAFF_TABLE).wrapping_add((pessoa as usize).wrapping_mul(110));
        if deu_apelido(apelidos, p, 0) != 0 && confere(p, nome, sobrenome, comum, 0) != 0 {
            terceiro = Some(apelido(apelidos, p, 0));
        }
    }
    let c = terceiro.unwrap_or_else(|| registro(G_COMMON_NAME_TABLE, comum)); // o nome comum
    formatar(n, s, c, texto, tamanho, forma, -1)
}


/// the official name by format: 1 "%s %s", 2 "%c.%s", 3 "%s, %s", 4 "%s, %c", 5 the surname, 6 the initials; nation
/// [0x9cf488]: surname only
/// Original: `database_format_official_name` (0x53eab0, Database.cpp, cdecl).
///
/// [pt] { 0x53eab0 } (cdecl, Database.cpp): `database_format_official_name(oficial, texto, tamanho, forma)`: o nome do
/// oficial (nome +4, sobrenome +8) em `texto` pela forma (byte): 1 "%s %s", 2 "%c.%s", 3 "%s, %s" (sobrenome
/// primeiro), 4 "%s, %c", 5 só o sobrenome, 6 "%c %c" (as iniciais). Faltando um dos dois: o sobrenome (ou o nome);
/// na forma 6, a inicial que houver. Oficial da nação +0x16 com id igual a [0x9cf488]: só o sobrenome, em qualquer
/// forma. Devolve 1. Oficial ou texto nulos, tamanho (palavra) < 0x69 ou forma fora de 1..6: a original (caixa). Nos
/// eventos e no painel da partida (0x6d8d10, 0x6d7b00, 0x71c270).
pub(crate) unsafe extern "C" fn database_format_official_name(oficial: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_OFFICIAL_NAME);
    if oficial == 0 || texto == 0 || (tamanho as i16) < 0x69 {
        return o(oficial, texto, tamanho, forma);
    }
    let (a, b) = (read_usize(oficial + 4), read_usize(oficial + 8));
    let nacao = read_usize(oficial + 0x16);
    let so_sobrenome = nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_SPAIN);
    let f = forma as u8;
    if !so_sobrenome && !(1..=6).contains(&f) {
        return o(oficial, texto, tamanho, forma);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let inicial = |t: usize| *(t as *const i8) as i32;
    let copiar = |t: usize| {
        let mut k = 0;
        loop {
            let c = *((t + k) as *const u8);
            *((texto + k) as *mut u8) = c;
            if c == 0 {
                break;
            }
            k += 1;
        }
    };
    *(texto as *mut u8) = 0;
    // o que sobra quando falta um dos dois: o sobrenome, senão o nome
    let um_so = || {
        if is_nonempty_text(b) {
            copiar(b);
        } else if is_nonempty_text(a) {
            copiar(a);
        }
    };
    let os_dois = is_nonempty_text(a) && is_nonempty_text(b);
    if so_sobrenome || f == 5 || (!os_dois && f != 6) {
        um_so();
    } else {
        match f {
            1 => { sprintf(texto, 0x98f0b8, a, b); } // "%s %s"
            2 => { sprintf(texto, 0x9b7410, inicial(a), b); } // "%c.%s"
            3 => { sprintf(texto, 0x98f6ac, b, a); } // "%s, %s"
            4 => { sprintf(texto, 0x9b7408, b, inicial(a)); } // "%s, %c"
            _ => {
                if os_dois {
                    sprintf(texto, 0x9b7418, inicial(a), inicial(b)); // "%c %c"
                } else if is_nonempty_text(b) {
                    sprintf(texto, 0x99ec1c, inicial(b)); // "%c"
                } else if is_nonempty_text(a) {
                    sprintf(texto, 0x99ec1c, inicial(a));
                }
            }
        }
    }
    1
}


/// the translated position name of the mask (goalkeeper, support, striker, wing back, wingers, midfielders, centre
/// back, full backs, sweeper, substitute, defender) and the gender by language
/// Original: `position_mask_get_text` (0x5482f0, Database.cpp, cdecl).
///
/// [pt] { 0x5482f0 } (cdecl, Database.cpp): `position_mask_get_text(mascara, texto, maiuscula)`: o nome da posição da
/// máscara (0x66f4e0 com o texto 0xad9c64; a forma com maiúscula se o byte `maiuscula` não é 0), na ordem: goleiro
/// (1), jogador de apoio (0x260/0x160/0x460), atacante (0x40 com 0x700), ala (0x88/0x808), ponta esquerda (0x90/0xa0),
/// ponta direita (0x810/0x820), meia central e meia-atacante (0x100/0x200/0x400 com 0x18 ou 0x20), zagueiro central
/// (0x204/0x104/0x404), lateral esquerdo (0x84), lateral direito (0x804), líbero (2), reserva (0) e defensor. Devolve o
/// byte do gênero pelo idioma (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4, os outros 0 (reserva: sempre 0). Texto
/// nulo: 0. Na mídia (0x72d270) e na tela da pessoa (0x87dd80).
pub(crate) unsafe extern "C" fn position_mask_get_text(mascara: u32, texto: usize, maiuscula: u32) -> u8 {
    if texto == 0 {
        return 0;
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = idioma();
    let m = mascara;
    let tem = |x: u32| m & x == x;
    let meio = m & 0x700 != 0; // 0x100, 0x200 ou 0x400
    // (minúscula, maiúscula), gênero com o idioma 2 e com o 4
    let (t, g2, g4): ((usize, usize), u8, u8) = if m & 1 != 0 {
        ((0x9b841c, 0x9b83f0), 2, 1) // goalkeeper
    } else if tem(0x260) || tem(0x160) || tem(0x460) {
        ((0x9b7f60, 0x9b7f30), 5, 4) // support player
    } else if m & 0x40 != 0 && tem(0x700) {
        ((0x9b83c4, 0x9b8398), 5, 4) // striker
    } else if tem(0x88) || tem(0x808) {
        ((0x9b7fbc, 0x9b7f90), 5, 1) // wing back
    } else if tem(0x90) || tem(0xa0) {
        ((0x9b8018, 0x9b7fe8), 5, 4) // left winger
    } else if tem(0x810) || tem(0x820) {
        ((0x9b8078, 0x9b8048), 5, 4) // right winger
    } else if meio && m & 0x18 != 0 {
        ((0x9b80dc, 0x9b80a8), 2, 1) // central midfielder
    } else if meio && m & 0x20 != 0 {
        ((0x9b8360, 0x9b8328), 2, 1) // attacking midfielder
    } else if tem(0x204) || tem(0x104) || tem(0x404) {
        ((0x9b8140, 0x9b8110), 2, 1) // centre back
    } else if tem(0x84) {
        ((0x9b82fc, 0x9b82d0), 2, 1) // left back
    } else if tem(0x804) {
        ((0x9b82a4, 0x9b8278), 2, 1) // right back
    } else if m & 2 != 0 {
        ((0x9b824c, 0x9b8220), 2, 1) // sweeper
    } else if m == 0 {
        ((0x9b81f4, 0x9b81c8), 0, 0) // substitute (o gênero fica 0)
    } else {
        ((0x9b819c, 0x9b8170), 2, 1) // defender
    };
    formatar(texto, if maiuscula as u8 != 0 { t.1 } else { t.0 }, 0xad9c64);
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





/// the translated position name by mask (the last one written wins; more than 3 positions "utility player", none
/// "player") and the gender by language
/// Original: `write_position_name` (0x5461b0, Database.cpp, cdecl).
///
/// [pt] { 0x5461b0 } (cdecl, Database.cpp): `write_position_name(mascara, texto, tamanho)`: o nome da posição (0x66f4e0;
/// cada um escreve por cima do anterior) contando as posições: 1 goleiro, 2 líbero, 4 zagueiro central (com 0x200 sem
/// 0x80/0x800), defensor (com 0x200) ou lateral, 8 lateral ofensivo (sem 4 e 0x200) ou volante (com 0x200), 0x10 meia,
/// 0x20 sem nenhuma antes (conta 1) meia-atacante (0x200), ponta (0x80 e 0x800), ponta esquerda (0x80) ou direita, 0x40
/// com 0x200 atacante; mais de 3: "utility player"; nenhuma: "player". Devolve o byte do gênero do último pelo idioma
/// (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4; os outros não mudam (começa em 0). Tamanho < 30: a original (caixa).
/// Na mídia, nos contratos, na busca de jogadores, nos olheiros e nas transferências (0x8dc370).
pub(crate) unsafe extern "C" fn write_position_name(mascara: u32, texto: usize, tamanho: i32) -> u8 {
    if tamanho < 0x1e {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u8>(IDX_WRITE_POSITION_NAME)(mascara, texto, tamanho);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = idioma();
    let m = mascara;
    let mut g = 0u8;
    // o texto e o gênero com o idioma 2 e com o 4 (o 5 dá sempre 1)
    let mut escrever = |s: usize, a: u8, c: u8| {
        formatar(texto, s);
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
        escrever(0x9b7d5c, 2, 1); // goalkeeper
    }
    if m & 2 != 0 {
        n += 1;
        escrever(0x9b7d54, 2, 1); // sweeper
    }
    if m & 4 != 0 {
        n += 1;
        if m & 0x200 != 0 {
            if m & 0x80 == 0 && m & 0x800 == 0 {
                escrever(0x9b7d40, 2, 1); // central defender
            } else {
                escrever(0x9b7d28, 2, 1); // defender
            }
        } else {
            escrever(0x9b7d34, 5, 1); // full back
        }
    }
    if m & 8 != 0 {
        n += 1;
        if m & 4 == 0 && m & 0x200 == 0 {
            escrever(0x9b7d14, 5, 1); // attacking full back
        } else if m & 0x200 != 0 {
            escrever(0x9b7cfc, 2, 1); // defensive midfielder
        }
    }
    if m & 0x10 != 0 {
        n += 1;
        escrever(0x9b7cf0, 2, 1); // midfielder
    }
    if m & 0x20 != 0 && n == 0 {
        n = 1;
        if m & 0x200 != 0 {
            escrever(0x9b7cd8, 2, 1); // attacking midfielder
        } else if m & 0x80 != 0 {
            escrever(if m & 0x800 != 0 { 0x9b7cd0 } else { 0x9b7cc4 }, 5, 4); // winger, left winger
        } else {
            escrever(0x9b7cb4, 5, 4); // right winger
        }
    }
    if m & 0x40 != 0 && m & 0x200 != 0 {
        n += 1;
        escrever(0x9b7cac, 5, 4); // forward
    }
    if n > 3 {
        escrever(0x9b7c9c, 2, 1); // utility player
    }
    if n == 0 {
        escrever(0x9b7c94, 2, 1); // player
    }
    g
}


/// the player description by strengths (0x546ab0) and position: forward, midfielder, full back, defender, veteran or
/// rookie by age; the gender by language
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
pub(crate) unsafe extern "C" fn write_player_description(pessoa: i32, posicao: u32, qualidades: u32, texto: usize, tamanho: i32, genero: u32) -> u8 {
    if tamanho < 0x1e {
        return original_fn::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(IDX_WRITE_PLAYER_DESCRIPTION)(pessoa, posicao, qualidades, texto, tamanho, genero);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let i = idioma();
    let g = if i == 5 { 1 } else { genero as u8 };
    let q = qualidades;
    let tem = |m: u32| q & m == m;
    // o texto e o gênero com o idioma 4
    let escrever = |s: usize, com_4: u8| {
        formatar(texto, s);
        if i == 4 { com_4 } else { g }
    };
    let soma = pessoa.wrapping_add(q as i32);
    if q & 0x40 != 0 {
        if tem(0x2c0000) {
            return escrever(0x9b7f20, 1); // deadly attacker
        }
        if tem(0x240000) {
            return escrever(if soma % 3 == 1 { 0x9b7f10 } else { 0x9b7efc }, 1); // pacey hitman, pacey goalscorer
        }
        if tem(0xc0000) {
            return escrever(0x9b7ef0, 4); // goalpoacher
        }
        if tem(0x40000) {
            if soma % 4 == 1 {
                return escrever(0x9b7ee8, 1); // hitman
            }
            return escrever(0x9b7ed4, 3); // prolific striker
        }
        for (m, s) in [(0x102000, 0x9b7ec4), (0x200800, 0x9b7eb0), (0x800, 0x9b7e9c), (0x1200, 0x9b7e88), (0x10000, 0x9b7e74)] {
            if tem(m) {
                return escrever(s, 4); // dynamic, powerhouse, powerful, creative, exciting forward
            }
        }
        if tem(0x1000) {
            return escrever(0x9b7e60, 1); // skilful attacker
        }
        return g;
    }
    if posicao & 0x10 != 0 {
        if tem(0x40000) {
            if tem(0x800200) {
                formatar(texto, 0x9b7e44); // inspirational midfielder
            }
            formatar(texto, 0x9b7e2c); // free-scoring midfielder
            return 0;
        }
        if q & 0x200 != 0 && posicao & 0x800 == 0 && posicao & 0x80 == 0 {
            return escrever(0x9b7e20, 1); // playmaker
        }
        for (m, s) in [(0x800000, 0x9b7e14), (0x102100, 0x9b7e00), (0x2000, 0x9b7de8), (0x8000, 0x9b7dd4), (0x200000, 0x9b7dc0)] {
            if tem(m) {
                return escrever(s, 1); // ball-winner, dynamic, hard working, reliable, pacey midfielder
            }
        }
        return g;
    }
    if q & 2 != 0 && q & 4 == 0 {
        if tem(0x200000) {
            return escrever(0x9b7db0, 1); // pacey full-back
        }
        if tem(0x10000) {
            return escrever(0x9b7d9c, 1); // exciting full-back
        }
        return g;
    }
    if q & 4 != 0 {
        if tem(0x200000) {
            return escrever(0x9b7d8c, 1); // pacey defender
        }
        if tem(0x1200) {
            return escrever(0x9b7d78, 1); // cultured defender
        }
        return g;
    }
    if pessoa <= -1 {
        return g;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((pessoa as usize).wrapping_mul(110));
    let idade = read_i8_as_i32(p + 0x18);
    if q & 1 == 0 && idade > pessoa % 3 + 0x21 {
        return escrever(0x9b7d70, 4); // veteran
    }
    if soma % 3 != 0 || q & 1 != 0 {
        return g;
    }
    if idade < pessoa % 3 + 0x12 && *((p + 0x22) as *const u8) == 0 {
        return escrever(0x9b7d68, 4); // rookie
    }
    g
}


/// Score of colleague `o` as a favourite of `pessoa` at `clube` (filters and score of 0x548810/0x548da0: contract at
/// the club, reputation, not leaving, stats), or None if he does not qualify.
///
/// [pt] A nota do colega `o` como favorito da `pessoa` no `clube` (os filtros e a nota da 0x548810, iguais aos da 0x548da0),
/// ou `None` se ele não entra. As chamadas pelo endereço do jogo, na ordem da original.
pub(crate) unsafe fn favourite_colleague_score(pessoa: usize, clube: usize, o: usize) -> Option<i32> {
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    if read_usize(o + 0x61) == 0 {
        return None;
    }
    let id = read_usize(pessoa) as i32;
    let oid = read_usize(o) as i32;
    if oid >= read_usize(G_STAFF_COUNT) as i32 || o == pessoa || oid.wrapping_add(id) % 3 == 0 {
        return None;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref != 0 && (0..3).any(|j| read_usize(pref + 0x28 + 4 * j) == o) {
        return None; // rejeitado pela pessoa
    }
    let contrato = contract_of(read_usize(G_CONTRACT_MANAGER), o, clube);
    if contrato == 0 {
        return None;
    }
    let rep = read_i16_as_i32(read_usize(o + 0x61) + 0xb);
    if rep <= 0x109a || rep <= read_i16_as_i32(clube + club::REPUTATION) - 0x2ee {
        return None;
    }
    if fora(read_usize(G_TRANSFER_MANAGER), o, 0) != 0 || read_i16_as_i32(read_usize(o + 0x61) + 5) <= 100 {
        return None;
    }
    let l0 = media(read_usize(G_PLAYER_STATS), read_usize(o + 0x61), 6, 0x11, 6.5) as f32 as f64; // fstp dword
    let m1 = media(read_usize(G_PLAYER_STATS), read_usize(o + 0x61), 6, 1, 6.5);
    let dados = read_usize(o + 0x61);
    let mut n = read_i16_as_i32(dados + 0xb);
    if m1 > read_f32_at(0x968990) as f64 { // 0x968990 = 5
        let g = read_usize(G_PLAYER_STATS);
        let razao = || {
            let d = read_f64_at(g + 0x18);
            if d == 0.0 || d.is_nan() { read_f64_at(0x968998) } else { read_f64_at(g + 0x10) / d } // 0x968998 = 6.8
        };
        // fcompp: C0 (abaixo ou sem ordem) soma
        if !(razao() + read_f64_at(0x968918) >= l0) { // 0x968918 = 0.2
            n += 500;
        } else if !(razao() + read_f64_at(0x967880) >= l0) { // 0x967880 = 0.1
            n += 250;
        }
    }
    if pref != 0 && (0..3).any(|j| read_usize(pref + 0x1c + 4 * j) == o) {
        n += 1250; // já favorito
    }
    let c = read_i8_as_i32(dados + 0x21);
    if c == 20 {
        n += 75;
    } else if c > 15 {
        n += 50;
    }
    let a = read_i8_as_i32(o + 0x5b);
    if a > 15 {
        n += 250;
    } else if a < 6 {
        n -= 100;
    }
    n += id.wrapping_add(oid) % 500; // desempate pelos ids (dbs)
    match *((contrato + contract::SQUAD_STATUS) as *const u8) >> 4 {
        1 => n += 750,
        2 => n += 300,
        3 => n += 150,
        _ => {}
    }
    let nacao = read_usize(pessoa + staff::NATION);
    if nacao != read_usize(clube + club::NATION) && read_usize(o + 0x1a) == nacao {
        if read_usize(contrato + 0x45) & 0x20020 != 0 {
            n += 1750;
        } else if idade(G_CURRENT_DATE, read_usize(pessoa + staff::DATE_JOINED_CLUB), read_usize(pessoa + 0x42)) < 350 && read_i8_as_i32(pessoa + staff::YEAR_OF_BIRTH) < 24 {
            n += 250;
        } else {
            n += 125;
        }
    }
    Some(n)
}

/// 1 when the candidate passes the favoured team mate filters and none of the 50 in the squad has a higher score (the
/// 0x548da0 score)
/// Original: `staff_is_best_favoured_candidate` (0x548810, Database.cpp, cdecl).
///
/// [pt] { 0x548810 } (cdecl, Database.cpp): `staff_is_best_favoured_candidate(pessoa, clube, candidato)`: 1 se o candidato
/// passa nos filtros da escolha do companheiro favorito (os da 0x548da0 sem o da máscara da posição) e nenhum dos 50 do
/// elenco do clube (+0xd7) tem nota estritamente maior; 0 se não. Antes, os da pessoa: [0xb74350], id % 6 = r: +0x59 ≥
/// r + 6, +0x57 ≤ 15 − r, idade (0x54cdb0) ≥ id % 28 + 14. A original guarda a nota do candidato no próprio argumento
/// `pessoa` e um float no `candidato` (gravados antes de lidos); a cópia não. Só `al`. Na mídia (0x758200).
pub(crate) unsafe extern "C" fn staff_is_best_favoured_candidate(pessoa: usize, clube: usize, candidato: usize) -> u8 {
    if pessoa == 0 || clube == 0 || candidato == 0 || read_usize(G_PLAYER_STATS) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let id = read_usize(pessoa) as i32;
    let r6 = id % 6;
    if read_i8_as_i32(pessoa + staff::LOYALITY) < r6 + 6 || read_i8_as_i32(pessoa + staff::AMBITION) > 15 - r6 {
        return 0;
    }
    if idade(G_CURRENT_DATE, read_usize(pessoa + staff::DATE_JOINED_CLUB), read_usize(pessoa + 0x42)) < id % 28 + 14 {
        return 0;
    }
    let alvo = match favourite_colleague_score(pessoa, clube, candidato) {
        Some(n) => n,
        None => return 0,
    };
    for k in 0..50 {
        let o = read_usize(clube + club::SQUAD + 4 * k);
        if o == 0 {
            continue;
        }
        if let Some(n) = favourite_colleague_score(pessoa, clube, o) {
            if n > alvo {
                return 0; // há um melhor
            }
        }
    }
    1
}




/// Player stat average (PLAYERSTATS get_stat on [0xb74350]) for the person's player record, given stat type and
/// attribute, weight 6.5.
///
/// [pt] A 0x7c1e30 das notas ([0xb74350]) do jogador (+0x61) da pessoa: `media(p, tipo, atributo)`, peso 6,5.
pub(crate) unsafe fn player_stat_average(pessoa: usize, tipo: u32, atributo: u32) -> f64 {
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    media(read_usize(G_PLAYER_STATS), read_usize(pessoa + staff::PLAYER), tipo, atributo, 6.5)
}

/// Computed player attribute (0x543540) of the person's player, as a signed byte.
///
/// [pt] O atributo calculado (0x543540) do jogador da pessoa, como byte com sinal.
pub(crate) unsafe fn computed_player_attribute(pessoa: usize, atributo: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_COMPUTE_PLAYER_ATTRIBUTE);
    f(pessoa, read_usize(pessoa + staff::PLAYER), atributo) as i8 as i32
}

/// Bit mask of the positions where the player is natural (rating 20) - goalkeeper, wing-backs, defenders, midfielders -
/// plus `soma` when the ten position ratings sum above 110.
///
/// [pt] Os bits das posições em que o jogador é natural (20), comuns às duas: goleiro (+0xf), lateral (+0x11, com +0x19
/// ou com +0x17/+0x18), zagueiros (+0x12/+0x13 sem +0x17/+0x18 a 15), meias (+0x14, +0x15). `bits` = os valores
/// de cada caso na ordem; mais o da soma das dez posições (+0x10..+0x19) acima de 110.
pub(crate) unsafe fn natural_position_bits(j: usize, bits: [u32; 7], soma: u32) -> u32 {
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
        f += soma;
    }
    f
}

/// the player strength bits: natural positions, the [0xae2d0c] nibble and 19 draws against bytes, attributes (0x543540)
/// and averages (0x7c1e30)
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
pub(crate) unsafe extern "C" fn staff_get_player_strength_flags(pessoa: usize, outro: usize) -> u32 {
    if outro == 0 || pessoa == 0 {
        return 0;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let j = || read_usize(pessoa + staff::PLAYER);
    let b = |k: usize| signed_byte_at(j(), k);
    let m = |tipo: u32, atributo: u32| player_stat_average(pessoa, tipo, atributo);
    let mut f = natural_position_bits(j(), [1, 4, 2, 8, 0x10, 0x20, 0x40], 0);
    let r = sortear(3);
    let id = read_usize(pessoa) as u32;
    let nibble = (*(read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(79) as usize).wrapping_add(0xb) as *const u8) & 0xf) as i32;
    if nibble > r && read_usize(G_TRANSFER_MANAGER) != 0 && fora(read_usize(G_TRANSFER_MANAGER), pessoa, 0) == 0 {
        f += 0x1000000;
    }
    if j() != 0 && (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += 0x80;
    }
    if b(0x3c) > sortear(3) + 15 {
        f += 0x100;
    }
    let mut ajuste = 0i32;
    if m(2, 0x16) > read_f32_at(0x969570) as f64 { // 0x969570 = 180
        let a = match_pl_ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = match_pl_ftol(a as f64 - read_f64_at(0x969568)); // 0x969568 = 65
        }
        let x = (m(2, 5) * read_f32_at(0x969560) as f64) as f32; // fstp dword; 0x969560 = 50
        ajuste = ajuste.wrapping_add(match_pl_ftol(x as f64 / m(2, 1)));
    }
    if b(0x37).wrapping_add(ajuste) > sortear(50) {
        f += 0x200;
    }
    let v14 = m(2, 0xe);
    let v14f = v14 as f32; // fst dword
    if v14 > read_f64_at(0x968f10) { // 0x968f10 = 8
        f += 0x800000;
    }
    if b(0x39) > sortear(50) && b(0x3e) > sortear(50) {
        let mut sim = false;
        if v14f as f64 > read_f64_at(0x9689a0) { // 0x9689a0 = 0
            let r = sortear(10) + 5;
            sim = match_pl_ftol(v14f as f64) as i8 as i32 > r;
        }
        if !sim && computed_player_attribute(pessoa, 0x17) > sortear(5) + 15 {
            let x = m(2, 7) * read_f64_at(0x968f50); // fstp qword; 0x968f50 = 150
            sim = m(2, 0x16) > x;
        }
        if sim {
            f += 0x400;
        }
    }
    for (k, bit) in [(0x2e, 0x800), (0x40, 0x1000), (0x44, 0x2000)] {
        if b(k) > sortear(5) + 15 {
            f += bit;
        }
    }
    for (a, bit) in [(4, 0x4000), (0xd, 0x20000), (0x14, 0x100000), (0x15, 0x200000), (1, 0x400000)] {
        if computed_player_attribute(pessoa, a) > sortear(5) + 15 {
            f += bit;
        }
    }
    if b(0x21) > sortear(3) + 16 {
        f += 0x8000;
    }
    if computed_player_attribute(pessoa, 0xb) > sortear(4) + 14 {
        let x = m(2, 0xf); // fstp qword
        let r = sortear(20);
        if !(r as f64 * read_f64_at(0x967880) + read_f64_at(0x968eb0) >= x) { // 0x967880 = 0.1, 0x968eb0 = 7.5
            f += 0x10000;
        }
    }
    let v = m(2, 0x16);
    let limit = 1400 - read_i16_as_i32(j() + 0xb) / 10;
    if !(limit as f64 >= v) {
        let a = m(6, 2) as f32;
        let c = m(6, 1) as f32;
        let r = sortear(3);
        if !((r as f64 * read_f64_at(0x967880) + read_f64_at(0x968968)) * c as f64 >= a as f64) { // 0x967880 = 0.1, 0x968968 = 0.5
            f += 0x40000;
        }
    }
    if !(m(2, 0x15) >= read_f64_at(0x968930)) && m(2, 0x10) > read_f64_at(0x968ff8) { // 0x968930 = 3, 0x968ff8 = 50
        let x = m(2, 2) * read_f64_at(0x968930); // fstp qword; 0x968930 = 3
        if m(2, 1) > x {
            f += 0x80000;
        }
    }
    f
}

/// the player weakness bits: natural positions and draws against bytes, attributes, averages, the 0x62c3d0 word and the
/// starter score (0x8972c0, 0x5b2ae0, 0x6e6340)
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
pub(crate) unsafe extern "C" fn staff_get_player_weakness_flags(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 || pessoa == 0 {
        return 0;
    }
    let palavra: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_INJURY_GET_FATIGUE);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TACTICS_FIND_SELECTION_SLOT);
    let formacao_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTIC);
    let casa_de: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_SELECTION);
    let j = || read_usize(pessoa + staff::PLAYER);
    let b = |k: usize| signed_byte_at(j(), k);
    let m = |tipo: u32, atributo: u32| player_stat_average(pessoa, tipo, atributo);
    let mut f = natural_position_bits(j(), [0, 0x20, 0x10, 0x40, 0x80, 0x100, 0x200], 0x800000);
    if (*((pessoa + staff::TEMPERAMENT) as *const i8) as i32) < sortear(4) + 5 && (b(0x1c) > 15 || b(0x25) < 6) {
        let a = (m(2, 0xb) * read_f32_at(0x969584) as f64) as f32; // fstp dword; 0x969584 = 3
        let s = (m(2, 0xa) + a as f64) as f32; // fstp dword
        if !(m(2, 1) * read_f32_at(0x968f18) as f64 >= s as f64) { // 0x968f18 = 0.5
            f += 1;
        }
    }
    let mut n = 4;
    let w = palavra(read_usize(G_INJURY), read_usize(pessoa) as i32) as i16 as i32;
    if w > sortear(1000) + 500 {
        n = 8;
    }
    if b(0x34) < sortear(12) {
        n += 3;
    }
    if b(0x3c) < sortear(n) + 8 && b(0xf) < 20 {
        f += 2;
    }
    let tatica = read_usize(G_TACTICS);
    let vaga = vaga_de(tatica, clube, pessoa, 0);
    if (vaga as i8) > -1 && (vaga as i8) < 11 {
        let formacao = formacao_de(tatica, clube, 0);
        let casa = casa_de(formacao, (clube as u32 & !0xff) | vaga as u32, 0); // o argumento com o byte da vaga
        let x = nota(pessoa, casa as u32, 0x800, 0, -1, 0);
        if x < sortear(40) + 150 {
            f += 4;
        }
    }
    let mut ajuste = 0i32;
    if m(2, 0x16) > read_f32_at(0x969570) as f64 { // 0x969570 = 180
        let a = match_pl_ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = 70 - a;
        }
        let x = (m(2, 5) * read_f32_at(0x969580) as f64) as f32; // fstp dword; 0x969580 = -50
        ajuste = ajuste.wrapping_add(match_pl_ftol(x as f64 / m(2, 1)));
    }
    if b(0x37) < sortear(-50).wrapping_add(ajuste) {
        f += 0x400;
    }
    if b(0x39) < sortear(-50) && b(0x3e) < sortear(-50) {
        let v = m(2, 0xe);
        let vf = v as f32; // fst dword
        if v > read_f64_at(0x9689a0) { // 0x9689a0 = 0
            let r = sortear(10);
            if (match_pl_ftol(vf as f64) as i8 as i32) < r {
                f += 0x800;
            }
        }
    }
    let bd = read_usize(G_INJURY);
    if bd != 0 {
        let w = palavra(bd, read_usize(pessoa) as i32) as i16 as i32;
        if w > sortear(500) + 750 {
            f += 8;
        }
    }
    if b(0x2e) < sortear(15) {
        f += 0x1000;
    }
    if b(0xf) < 20 { // os de linha
        if b(0x40) < sortear(10) {
            f += 0x2000;
        }
        if b(0x44) < sortear(12) {
            f += 0x4000;
        }
    }
    if b(0x1e) < sortear(-50) {
        f += 0x8000;
    }
    if b(0xf) < 20 && b(0x25) + b(0x1c) > sortear(20) + 25 {
        f += 0x10000;
    }
    if b(0x21) < 12 - sortear(10) {
        f += 0x20000;
    }
    if b(0x11) < 15 && computed_player_attribute(pessoa, 0xb) > sortear(4) + 14 {
        let v = m(2, 0xf);
        let vf = v as f32; // fst dword
        if v > read_f64_at(0x9689a0) && computed_player_attribute(pessoa, 0xb) as f64 * read_f64_at(0x969578) > vf as f64 { // 0x9689a0 = 0, 0x969578 = 0.28
            f += 0x40000;
        }
    }
    if b(0x28) > sortear(5) + 13 {
        f += 0x80000;
    }
    if m(2, 0x16) > read_f32_at(0x969570) as f64 { // 0x969570 = 180
        let a = computed_player_attribute(pessoa, 0xc);
        let t = (m(2, 0x10) + a as f64 - read_f32_at(0x968a78) as f64) as f32; // fstp dword; 0x968a78 = 10
        if (sortear(20) + 20) as f64 > t as f64 {
            let x = m(6, 2); // fstp qword
            if m(6, 1) * read_f64_at(0x969510) > x { // 0x969510 = 0.35
                f += 0x100000;
            }
        }
    }
    if !(m(2, 0x15) >= read_f64_at(0x968930)) && m(2, 0x10) > read_f64_at(0x968ff8) { // 0x968930 = 3, 0x968ff8 = 50
        f += 0x200000;
    }
    if computed_player_attribute(pessoa, 0x14) < 7 || computed_player_attribute(pessoa, 0x14) < read_i16_as_i32(j() + 5) / 15 {
        f += 0x400000;
    }
    f
}

/// Side-by-side comparison of original and port that, on divergence, re-runs once from the same RNG state (cache now
/// warm) and restores the first run's RNG state; returns both results, equality and a diagnostic.
///
/// [pt] O lado a lado das duas, com o estado do sorteio. A 0x6e6340 (pela 0x62ed70) sorteia só na primeira consulta do
/// jogador e guarda: com essa memória fria a original sorteia a mais e a cópia, que roda depois, não, e os sorteios
/// seguintes saem diferentes. Então, divergindo, refaz uma vez a comparação a partir do mesmo estado (agora quente, a
/// original sem contar) e devolve o estado do sorteio da primeira, a que o jogo segue.
pub(crate) unsafe fn compare_with_rng_retry(o: impl Fn() -> u32, n: impl Fn() -> u32) -> (u32, u32, bool, String) {
    let antes = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, x, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(),
        || crate::aleatorio::sem_contar(|| n()));
    if igual && r == x {
        return (r, x, true, String::new());
    }
    let depois = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let (r2, x2, igual2, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || crate::aleatorio::sem_contar(|| o()),
        || crate::aleatorio::sem_contar(|| n()));
    depois.escrever();
    (r, x, igual2 && r2 == x2, format!("1a {:x} x {:x}; 2a {:x} x {:x} {:x?} x {:x?}", r, x, r2, x2, mo, mr))
}



/// Club linked to `clube` as A/B team (club_get_linked_b_team 0x540a50, active=1); sets eh_a to 1 if the club is the A
/// team; 0 if none.
///
/// [pt] O time B ligado ao `clube` (club_get_linked_b_team 0x540a50 pelo endereço do jogo, com `ativo` = 1): o registro
/// do parceiro ou 0; `eh_a` recebe 1 (o clube é o A) ou 0 (o clube é o B) quando há parceiro. Recusado, a 0x540a50
/// zera a marca +0x244 do clube: na comparação a original vem antes e a cópia acha a marca já zerada (também 0).
pub(crate) unsafe fn database_linked_b_team(clube: usize, eh_a: *mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    parceiro(clube, eh_a, 1)
}

/// how many of the 3 physios (+0x1cf) of the club and of its linked B team are valid people; in `al`
/// Original: `database_count_club_physios` (0x53c9b0, Database.cpp, cdecl).
///
/// [pt] { 0x53c9b0 } (cdecl, Database.cpp): `database_count_club_physios(clube)`: quantos dos 3 fisioterapeutas
/// (+0x1cf) do clube e do time B ligado (0x540a50) são pessoas válidas (não nulas, id +0 < [0xae2364] com sinal).
/// Só `al`. Clube nulo: a original (caixa e 0). Nas telas da pessoa (staff_screens, 0x8740c8 e 0x87412b).
pub(crate) unsafe extern "C" fn database_count_club_physios(clube: usize) -> u8 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_PHYSIOS)(clube);
    }
    let validos = |c: usize| {
        (0..3).filter(|k| {
            let p = core::ptr::read_unaligned((c + 0x1cf + 4 * k) as *const usize);
            p != 0 && *(p as *const i32) < *(G_STAFF_COUNT as *const i32)
        }).count() as u8
    };
    let b = database_linked_b_team(clube, core::ptr::null_mut());
    validos(clube) + if b != 0 { validos(b) } else { 0 } // os do clube e os do time B (dbt)
}


/// 1 when the club has a partner (0x540a50 with the flag) and is its B team
/// Original: `club_is_b_team` (0x5448c0, Database.cpp, cdecl).
///
/// [pt] { 0x5448c0 } (cdecl, Database.cpp): `club_is_b_team(clube)`: 1 se o clube tem parceiro (0x540a50 com `ativo`) e
/// é o B dele (`eh_a` = 0), senão 0 (o eax inteiro). A local `eh_a` só é lida com parceiro, e então a 0x540a50 a
/// escreveu. Clube nulo: a original (caixa e 0). Na tela da pessoa (staff_screens, 0x87272d).
pub(crate) unsafe extern "C" fn club_is_b_team(clube: usize) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_CLUB_IS_B_TEAM)(clube);
    }
    let mut eh_a = 0u32;
    (database_linked_b_team(clube, &mut eh_a) != 0 && eh_a == 0) as u32 // é o B (dbt)
}


/// the non-playing staff (6 from +0xbf, 15 from +0x19f) of the club and its B team into an array, up to the maximum;
/// the count in `al`
/// Original: `club_collect_non_playing_staff` (0x544c70, Database.cpp, cdecl).
///
/// [pt] { 0x544c70 } (cdecl, Database.cpp): `club_collect_non_playing_staff(clube, saida, max)`: põe em `saida` as
/// pessoas fora do elenco do clube e depois as do time B ligado (0x540a50 com `ativo`), na ordem dos cargos: os 6
/// de +0xbf (técnico, auxiliar, ...) e os 15 de +0x19f (treinadores, olheiros, fisioterapeutas), as não nulas, até
/// `max` (byte com sinal); devolve quantas em `al`. Clube nulo: a original (caixa e 0). Na aba de informações da
/// tela do clube (club_screen_draw_info_tab 0x460590, 0x4616fa).
pub(crate) unsafe extern "C" fn club_collect_non_playing_staff(clube: usize, saida: *mut usize, max: u32) -> u8 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(IDX_CLUB_COLLECT_NON_PLAYING_STAFF)(clube, saida, max);
    }
    let max = max as i8;
    let mut n = 0i8;
    let b = database_linked_b_team(clube, core::ptr::null_mut());
    for c in [clube, b] {
        if c == 0 {
            continue;
        }
        for d in (0..6).map(|k| 0xbf + 4 * k).chain((0..15).map(|k| 0x19f + 4 * k)) {
            let p = core::ptr::read_unaligned((c + d) as *const usize);
            if p != 0 && n < max {
                *saida.add(n as usize) = p; // (dbt)
                n += 1;
            }
        }
    }
    n as u8
}


/// 1 when the viewer sees the attribute: no fog or knowledge 3, always; knowledge 1 or 2, by the attribute and the id
/// modulo 4 (divisible by 4 or 2, or by 3); in `al`
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
pub(crate) unsafe extern "C" fn staff_is_player_attribute_visible(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let tabela = *(G_FOG_OF_WAR as *const usize);
    if quem_ve == 0 || pessoa == 0 || tabela == 0 || *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) == 0 {
        return 1;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FOG_OF_WAR_GET_MANAGER_MAP);
    let conhecimento: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_FOG_OF_WAR_MAP_GET_KNOWLEDGE_LEVEL);
    let f = ficha(tabela, quem_ve);
    if f == 0 {
        return 1;
    }
    let k = conhecimento(f, pessoa);
    if k == 3 {
        return 1;
    }
    if k != 1 && k != 2 {
        return 0;
    }
    let x = atributo as u8 as i8;
    if matches!(x, 1 | 0x11 | 0x15 | 0x1a | 0x1b) {
        return 1;
    }
    let x = x as i32;
    let divisor = if k == 2 { 2 } else { 4 }; // (dbt)
    let (a, b) = match *(pessoa as *const i32) % 4 {
        0 => (x, x + 1),
        1 => (x + 1, x + 1),
        2 => (x + 2, x + 2),
        _ => (x + 3, x + 3),
    };
    (a % divisor == 0 || b % 3 == 0) as u8
}


/// the player attribute (0x543540) or 0xf9 (unknown) when the fog hides it from the viewer; in `al`
/// Original: `staff_get_player_attribute_for_viewer` (0x543440, Database.cpp, cdecl).
///
/// [pt] { 0x543440 } (cdecl, Database.cpp): `staff_get_player_attribute_for_viewer(quem_ve, pessoa, atributo)`: o
/// atributo (1 a 0x1f, byte com sinal) do jogador (+0x61) da pessoa como quem vê o enxerga: com a névoa ligada
/// ([0x9cc270]), quem tem ficha (0x5afde0) e não vê o atributo (0x543280, pelo endereço do jogo) recebe 0xf9 (-7,
/// "desconhecido"); senão o valor de staff_compute_player_attribute (0x543540). Só `al`. Pessoa nula ou atributo fora
/// da faixa: a original (caixa e 0xff). Na comparação de jogadores e no perfil da pessoa (staff_screens 0x877470,
/// 0x879ac0, 0x87a754).
pub(crate) unsafe extern "C" fn staff_get_player_attribute_for_viewer(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let x = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&x) {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE_FOR_VIEWER)(quem_ve, pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_STAFF_COMPUTE_PLAYER_ATTRIBUTE);
    let jogador = core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize);
    if *(G_ATTRIBUTE_MASKING_ENABLED as *const u8) != 0 {
        let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FOG_OF_WAR_GET_MANAGER_MAP);
        let visivel: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_IS_PLAYER_ATTRIBUTE_VISIBLE);
        if ficha(*(G_FOG_OF_WAR as *const usize), quem_ve) != 0 && visivel(quem_ve, pessoa, atributo) == 0 {
            return 0xf9; // desconhecido (dbt)
        }
    }
    calcular(pessoa, jogador, atributo) as u8
}


/// Os nomes dos 12 atributos de técnico (abreviado, inteiro) da 0x543b10, pelos endereços dos textos no exe; o 8º
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

/// the translated name (0x66f4e0) of the 12 non-player attributes, short or full; returns 1
/// Original: `database_get_non_player_attribute_name` (0x543b10, Database.cpp, cdecl).
///
/// [pt] { 0x543b10 } (cdecl, Database.cpp): `database_get_non_player_attribute_name(atributo, texto, tamanho, abreviado)`:
/// escreve em `texto` o nome do atributo de técnico (1 a 12, byte com sinal), abreviado ou inteiro, pelo texto
/// traduzido do Langlib (0x66f4e0, pelo endereço do jogo); devolve 1. Atributo fora da faixa, texto nulo ou tamanho
/// (curto com sinal) abaixo de 100: a original (caixa e 0). Nas telas de busca (search_screens) e no perfil da
/// pessoa (staff_screens 0x87ad97).
pub(crate) unsafe extern "C" fn database_get_non_player_attribute_name(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=12).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_DATABASE_GET_NON_PLAYER_ATTRIBUTE_NAME)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *(texto as *mut u8) = 0;
    let (curto, longo) = STAFF_ATTRIBUTE_NAMES[x as usize - 1];
    if abreviado == 0 {
        formatar(texto, longo);
    } else if x == 8 {
        formatar(texto, curto, 0xad9c64usize);
    } else {
        formatar(texto, curto); // (dbt)
    }
    1
}


/// qsort comparison of staff by style-4 name (0x53dba0), case-insensitive; null person: empty text
/// Original: `database_compare_staff_by_surname` (0x53ff60, Database.cpp, cdecl).
///
/// [pt] { 0x53ff60 } (cdecl, Database.cpp): `database_compare_staff_by_surname(a, b)`: comparação do qsort de ponteiros
/// de pessoa pelo nome no estilo 4 ("sobrenome, nome", 0x53dba0 em 0x7d0 bytes) na comparação de textos 0x90c1d0
/// (sem maiúsculas, `compare_text_nocase`); pessoa nula vale o texto vazio (a original zera o primeiro byte das duas
/// locais antes). Na tela de oferta de contrato (contract_screens 0x4de5d0).
pub(crate) unsafe extern "C" fn database_compare_staff_by_surname(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 4); // "sobrenome, nome" (dbt)
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 4);
    }
    compare_text_nocase(na.as_ptr(), nb.as_ptr())
}


/// qsort comparison of club pointers by reputation (+0x80), highest first; null club: 0
/// Original: `database_compare_clubs_by_reputation` (0x53ffe0, Database.cpp, cdecl).
///
/// [pt] { 0x53ffe0 } (cdecl, Database.cpp): `database_compare_clubs_by_reputation(a, b)`: comparação do qsort de
/// ponteiros de clube pela reputação (+0x80, curto com sinal; clube nulo = 0), a maior primeiro: a de `a` menor: 1;
/// maior: -1; iguais: 0. (A 0x540020, já em Rust, é a mesma ordem sobre ids.) No menu dos times do clube
/// (0x482070) e nas abas de Escolher Amistosos (find_screens 0x5a1480, 0x5a1e10, 0x5a2250).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_reputation(a: usize, b: usize) -> i32 {
    let injury_word_0c = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { 0 } else { core::ptr::read_unaligned((c + 0x80) as *const i16) }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // a de menor reputação depois (dbt)
    } else if ra > rb {
        -1
    } else {
        0
    }
}


/// 1 when the club nation (+0x53) is in the EEC (+0x7f = 2)
/// Original: `database_club_nation_is_eec` (0x53d7d0, Database.cpp, cdecl).
///
/// [pt] { 0x53d7d0 } (cdecl, Database.cpp): `database_club_nation_is_eec(clube)`: 1 se a nação do clube (+0x53) não é
/// nula e está na CEE (+0x7f = 2), senão 0 (o eax inteiro). O clube não é conferido (a original lê o +0x53 dele).
/// Na aba de informações da tela do clube (club_screens 0x462070, os estrangeiros).
pub(crate) unsafe extern "C" fn database_club_nation_is_eec(clube: usize) -> u32 {
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    (nacao != 0 && *((nacao + nation::GROUP_MEMBERSHIP) as *const u8) == 2) as u32 // na CEE (dbt)
}


/// the global array of the 14 regions (0xae2104) sorted by name in the given language (qsort 0x945333 with 0x545e00)
/// Original: `get_regions_sorted_by_name` (0x545dc0, Database.cpp, cdecl).
///
/// [pt] { 0x545dc0 } (cdecl, Database.cpp): `get_regions_sorted_by_name(idioma)`: enche o vetor global de 14 regiões
/// (0xae2104) com 1..14, guarda o `idioma` (byte) em [0xae232b] e o ordena pelo qsort da CRT (0x945333) com a
/// comparação 0x545e00 (pelo endereço do jogo); devolve o vetor. Nas telas de busca (search_screens 0x80970c,
/// 0x814b7e, 0x815d5e: o filtro de região).
pub(crate) unsafe extern "C" fn get_regions_sorted_by_name(idioma: u32) -> usize {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    for k in 0..14u8 {
        *((G_REGIONS_SORTED_BY_NAME + k as usize) as *mut u8) = k + 1;
    }
    *(G_REGION_NAME_FORM as *mut u8) = idioma as u8;
    qsort(G_REGIONS_SORTED_BY_NAME, 14, 1, 0x545e00); // (dbt)
    G_REGIONS_SORTED_BY_NAME
}


/// qsort comparison of regions by name (0x616f20), case-insensitive; null a: 1; null b: -1
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
    let nome: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_GET_REGION_NAME);
    let idioma = *(G_REGION_NAME_FORM as *const u8) as u32;
    let mut na = [0u8; 0xc8];
    let mut nb = [0u8; 0xc8];
    nome(*(a as *const u8) as u32, na.as_mut_ptr() as usize, 0xc8, idioma);
    nome(*(b as *const u8) as u32, nb.as_mut_ptr() as usize, 0xc8, idioma);
    compare_text_nocase(na.as_ptr(), nb.as_ptr()) // (dbt)
}


/// the person at the other end of the opinion link (record [0xae2d0c] + id × 79: +0x27/+0x37 or +0x3b/+0x4b), or 0
/// Original: `staff_get_opinion_link_staff` (0x549dd0, Database.cpp, cdecl).
///
/// [pt] { 0x549dd0 } (cdecl, Database.cpp): `staff_get_opinion_link_staff(pessoa, alvo)`: o outro lado do laço de opinião
/// da pessoa: no registro da pessoa ([0xae2d0c] + id × 79; pessoa nula ou id >= [0xae2364]: 0), com laço (+0x27 ≠
/// -1), se o alvo é o +0x37: a pessoa de índice +0x27 ([0xae23bc] + 110 × índice); senão, se o alvo é o +0x4b: a de
/// índice +0x3b; senão 0. Na aba de transferências do perfil da pessoa (staff_screens 0x87dd80).
pub(crate) unsafe extern "C" fn staff_get_opinion_link_staff(pessoa: usize, alvo: u32) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let reg = (*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let ler = |d: usize| core::ptr::read_unaligned((reg + d) as *const u32);
    let pessoas = *(G_STAFF_TABLE as *const usize);
    if ler(0x27) == u32::MAX {
        return 0;
    }
    let indice = if ler(0x37) == alvo {
        ler(0x27)
    } else if ler(0x4b) == alvo {
        ler(0x3b) // (dbt)
    } else {
        return 0;
    };
    pessoas.wrapping_add((indice as usize).wrapping_mul(110))
}



/// True if the three-dword list at `lista` contains `quem`.
///
/// [pt] A lista de três dwords em `lista` tem `quem`?
pub(crate) unsafe fn list3_contains(lista: usize, quem: usize) -> bool {
    (0..3).any(|k| read_usize(lista + 4 * k) == quem)
}

/// Core of the add-to-preferences routines: if already present do nothing; if in the opposite list remove it from
/// there; else put it in the first free slot, or with `forcar` replace a random slot (rand 3).
///
/// [pt] O miolo das rotinas de acrescentar às preferências: já na lista (`ja`), nada; na lista `outra` (o oposto: favoritos
/// x rejeitados), tira de lá a primeira ocorrência e para; senão entra na primeira vaga de `lista`; sem vaga, só com
/// `forcar`: sorteia (0x90cfc0, 3) a vaga que troca (0, 1 ou 2).
pub(crate) unsafe fn database_add_to_preference_list(lista: usize, outra: usize, quem: usize, ja: bool, forcar: u32) {
    if ja {
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(outra + 4 * k) == quem) {
        *((outra + 4 * k) as *mut usize) = 0;
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = quem;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = sortear(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = quem;
    }
}

/// Regions compared for the preference routines: RNG state plus the 24 bytes of favourite/disliked people at `base` (if
/// non-null).
///
/// [pt] O estado do sorteio e as 6 pessoas (favoritas e rejeitadas) a partir de `base` (o clube +0x9b ou as preferências
/// +0x1c); `base` nulo: só o sorteio.
pub(crate) unsafe fn preference_regions(base: usize) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
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
pub(crate) unsafe extern "C" fn club_add_favoured_staff(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    database_add_to_preference_list(clube + club::FAV_STAFF1, clube + club::DIS_STAFF1, pessoa, list3_contains(clube + club::FAV_STAFF1, pessoa), forcar) // as favoritas do clube
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
pub(crate) unsafe extern "C" fn club_add_disliked_staff(clube: usize, pessoa: usize, forcar: u32) {
    if clube == 0 || pessoa == 0 {
        return;
    }
    database_add_to_preference_list(clube + club::DIS_STAFF1, clube + club::FAV_STAFF1, pessoa, list3_contains(clube + club::DIS_STAFF1, pessoa), forcar) // as rejeitadas do clube
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
pub(crate) unsafe extern "C" fn staff_add_favoured_staff(pessoa: usize, outra: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref == 0 || outra == 0 {
        return;
    }
    database_add_to_preference_list(pref + 0x1c, pref + 0x28, outra, list3_contains(pref + 0x1c, outra), forcar) // as favoritas da pessoa
}




/// puts the club among the three disliked clubs of the person (preferences +0x10..+0x18): the club already dislikes the
/// person (+0xa7..+0xaf), nothing; favoured (+4..+0xc), stops being; otherwise the first free slot; no free slot and
/// the third argument set, a random slot (3)
/// Original: `staff_add_disliked_club` (0x547e70, Database.cpp, cdecl).
///
/// [pt] { 0x547e70 } (cdecl, Database.cpp): `staff_add_disliked_club(pessoa, clube, forcar)`: põe o clube entre os três
/// rejeitados da pessoa (preferências +0x65: +0x10/+0x14/+0x18). O clube já rejeita a pessoa (+0xa7..+0xaf): nada;
/// o clube é favorito (+4..+0xc): deixa de ser e para; senão a primeira vaga (sem conferir se já está entre os
/// rejeitados, como a original); sem vaga, com `forcar`, a vaga sorteada (0x90cfc0, 3). Pessoa, preferências ou clube
/// nulos: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0, duas chamadas), 0x5a5980, MANAGER_MANAGER
/// (0x69c030) e outras.
pub(crate) unsafe extern "C" fn staff_add_disliked_club(pessoa: usize, clube: usize, forcar: u32) {
    if pessoa == 0 {
        return;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref == 0 || clube == 0 {
        return;
    }
    if (0..3).any(|k| read_usize(clube + club::DIS_STAFF1 + 4 * k) == pessoa) {
        return;
    }
    if let Some(k) = (0..3).find(|&k| read_usize(pref + 4 + 4 * k) == clube) {
        *((pref + 4 + 4 * k) as *mut usize) = 0;
        return;
    }
    let lista = pref + 0x10; // os clubes rejeitados
    if let Some(k) = (0..3).find(|&k| read_usize(lista + 4 * k) == 0) {
        *((lista + 4 * k) as *mut usize) = clube;
        return;
    }
    if forcar == 0 {
        return;
    }
    let k = sortear(3);
    if (0..3).contains(&k) {
        *((lista + 4 * k as usize) as *mut usize) = clube;
    }
}


/// each favoured person (preferences +0x1c..+0x24) leaves when a random draw of (+0x59) + 15 gives 0; each disliked one
/// (+0x28..+0x30), with 35 − (+0x5d)
/// Original: `staff_fade_staff_relationships` (0x547f90, Database.cpp, cdecl).
///
/// [pt] { 0x547f90 } (cdecl, Database.cpp): `staff_fade_staff_relationships(pessoa)`: cada uma das três pessoas favoritas
/// (preferências +0x1c..+0x24) presente sai com o sorteio (0x90cfc0) de (+0x59 da pessoa, com sinal) + 15 dando 0;
/// cada uma das três rejeitadas (+0x28..+0x30), com o de 35 − (+0x5d). Um sorteio por vaga ocupada, na ordem.
/// Pessoa ou preferências nulas: nada. No diário das pessoas (game_daily_staff_update 0x5ce2d0).
pub(crate) unsafe extern "C" fn staff_fade_staff_relationships(pessoa: usize) {
    if pessoa == 0 || read_usize(pessoa + staff::STAFF_PREFERENCES) == 0 {
        return;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    for k in 0..3 {
        let v = pref + 0x1c + 4 * k;
        if read_usize(v) != 0 && sortear(*((pessoa + staff::LOYALITY) as *const i8) as i32 + 15) == 0 { // as favoritas
            *(v as *mut usize) = 0;
        }
    }
    for k in 0..3 {
        let v = pref + 0x28 + 4 * k;
        if read_usize(v) != 0 && sortear(35 - *((pessoa + staff::TEMPERAMENT) as *const i8) as i32) == 0 { // as rejeitadas
            *(v as *mut usize) = 0;
        }
    }
}


/// copies the present disliked staff (preferences +0x28..+0x30) to the output, counting in the byte of the third
/// argument (always cleared)
/// Original: `staff_get_disliked_staff` (0x5480f0, Database.cpp, cdecl).
///
/// [pt] { 0x5480f0 } (cdecl, Database.cpp): `staff_get_disliked_staff(pessoa, saida, n)`: zera o byte `*n` (sempre, antes
/// de conferir a pessoa) e copia para `saida` as pessoas rejeitadas presentes (preferências +0x28/+0x2c/+0x30), na
/// ordem, contando em `*n`. Pessoa, preferências ou saída nulas: só o zero. Devolve `n` (o eax da original); os
/// chamadores leem só o byte. No diário das pessoas (0x5ce2d0, duas chamadas), MANAGER_MANAGER (0x693850), mídia e
/// telas de pessoas.
pub(crate) unsafe extern "C" fn staff_get_disliked_staff(pessoa: usize, saida: usize, n: usize) -> usize {
    let cont = n as *mut i8;
    *cont = 0;
    if pessoa == 0 || read_usize(pessoa + staff::STAFF_PREFERENCES) == 0 || saida == 0 {
        return n;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    for k in 0..3 {
        let p = read_usize(pref + 0x28 + 4 * k); // as rejeitadas
        if p != 0 {
            *((saida as isize + 4 * *cont as isize) as *mut usize) = p;
            *cont = (*cont).wrapping_add(1);
        }
    }
    n
}





/// the mask of the positions where the player is natural (20): goalkeeper 1; full backs, centre backs, midfielders,
/// forwards (bits 2..0xc00); with none, the first position at 15 or more; otherwise 0x10
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
pub(crate) unsafe extern "C" fn staff_get_natural_position_mask(pessoa: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let j = read_usize(pessoa + staff::PLAYER);
    if j == 0 {
        return 0;
    }
    let b = |k: usize| read_i8(j + k);
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
            m |= bit; // o lateral, o zagueiro...
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


/// how many of the 50 in the squad (+0xd7) with a player record have the given natural position mask
/// Original: `club_count_squad_with_position_mask` (0x548150, Database.cpp, cdecl).
///
/// [pt] { 0x548150 } (cdecl, Database.cpp): `club_count_squad_with_position_mask(clube, mascara)`: quantos dos 50 do elenco
/// (+0xd7) com jogador (+0x61) têm a máscara da posição natural (0x5481a0, pelo endereço do jogo) igual a `mascara`.
/// Só `al` (os três chamadores, 0x758010 e 0x758200, leem o byte). Na mídia.
pub(crate) unsafe extern "C" fn club_count_squad_with_position_mask(clube: usize, mascara: u32) -> u8 {
    let mascara_de: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    let mut n = 0u8;
    for k in 0..50 {
        let p = read_usize(clube + club::SQUAD + 4 * k);
        if p != 0 && read_usize(p + 0x61) != 0 && mascara_de(p) == mascara { // os iguais
            n = n.wrapping_add(1);
        }
    }
    n
}


/// the squad mate the person picks as favoured: filters by the person (age, +0x57, +0x59) and by the mate (disliked,
/// position, contract, reputation, 0x8cca10), score by reputation, the 0x7c1e30 average, already favoured, ids,
/// contract level and nationality; the highest score
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
pub(crate) unsafe extern "C" fn staff_pick_favoured_team_mate(pessoa: usize, clube: usize) -> usize {
    if pessoa == 0 || clube == 0 || read_usize(G_PLAYER_STATS) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let mascara_de: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_GET_NATURAL_POSITION_MASK);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let id = read_usize(pessoa) as i32;
    let r6 = id % 6;
    if (read_i8(pessoa + staff::LOYALITY) as i32) < r6 + 6 || read_i8(pessoa + staff::AMBITION) as i32 > 15 - r6 {
        return 0;
    }
    if idade(G_CURRENT_DATE, read_usize(pessoa + staff::DATE_JOINED_CLUB), read_usize(pessoa + 0x42)) < id % 28 + 14 {
        return 0;
    }
    let (mut melhor, mut nota_melhor) = (0usize, -10000i32);
    for k in 0..50 {
        let o = read_usize(clube + club::SQUAD + 4 * k);
        if o == 0 || read_usize(o + 0x61) == 0 {
            continue;
        }
        let oid = read_usize(o) as i32;
        if oid >= read_usize(G_STAFF_COUNT) as i32 || o == pessoa || oid.wrapping_add(id) % 3 == 0 {
            continue;
        }
        let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
        if pref != 0 && (0..3).any(|j| read_usize(pref + 0x28 + 4 * j) == o) {
            continue;
        }
        if read_i8(o + 0x5a) < 11 && mascara_de(o) == mascara_de(pessoa) {
            continue;
        }
        let contrato = contract_of(read_usize(G_CONTRACT_MANAGER), o, clube);
        if contrato == 0 {
            continue;
        }
        let rep = read_i16(read_usize(o + 0x61) + 0xb) as i32;
        if rep <= 0x109a || rep <= read_i16(clube + club::REPUTATION) as i32 - 0x2ee {
            continue;
        }
        if fora(read_usize(G_TRANSFER_MANAGER), o, 0) != 0 || read_i16(read_usize(o + 0x61) + 5) <= 100 {
            continue;
        }
        let l0 = media(read_usize(G_PLAYER_STATS), read_usize(o + 0x61), 6, 0x11, 6.5) as f32 as f64; // fstp dword
        let m1 = media(read_usize(G_PLAYER_STATS), read_usize(o + 0x61), 6, 1, 6.5);
        let dados = read_usize(o + 0x61);
        let mut nota = read_i16(dados + 0xb) as i32;
        if m1 > read_f32_at(0x968990) as f64 { // 0x968990 = 5
            let g = read_usize(G_PLAYER_STATS);
            let razao = || {
                let d = read_f64_at(g + 0x18);
                if d == 0.0 || d.is_nan() { read_f64_at(0x968998) } else { read_f64_at(g + 0x10) / d } // 0x968998 = 6.8
            };
            // fcompp: C0 (abaixo ou sem ordem) soma
            if !(razao() + read_f64_at(0x968918) >= l0) { // 0x968918 = 0.2
                nota += 500;
            } else if !(razao() + read_f64_at(0x967880) >= l0) { // 0x967880 = 0.1
                nota += 250;
            }
        }
        if pref != 0 && (0..3).any(|j| read_usize(pref + 0x1c + 4 * j) == o) {
            nota += 1250; // já favorito
        }
        let c = read_i8(dados + 0x21);
        if c == 20 {
            nota += 75;
        } else if c > 15 {
            nota += 50;
        }
        let a = read_i8(o + 0x5b);
        if a > 15 {
            nota += 250;
        } else if a < 6 {
            nota -= 100;
        }
        nota += oid.wrapping_add(id) % 500; // o desempate pelos ids
        match *((contrato + contract::SQUAD_STATUS) as *const u8) >> 4 {
            1 => nota += 750,
            2 => nota += 300,
            3 => nota += 150,
            _ => {}
        }
        let nacao = read_usize(pessoa + staff::NATION);
        if nacao != read_usize(clube + club::NATION) && read_usize(o + 0x1a) == nacao {
            if read_usize(contrato + 0x45) & 0x20020 != 0 {
                nota += 1750;
            } else if idade(G_CURRENT_DATE, read_usize(pessoa + staff::DATE_JOINED_CLUB), read_usize(pessoa + 0x42)) < 350 && read_i8(pessoa + staff::YEAR_OF_BIRTH) < 24 {
                nota += 250;
            } else {
                nota += 125;
            }
        }
        if nota > nota_melhor {
            melhor = o;
            nota_melhor = nota;
        }
    }
    melhor
}


/// Id (+0) of the club's nation (+0x53); None if club or nation is null.
///
/// [pt] O id (+0) da nação do `clube` (+0x53); clube ou nação nulos: None.
pub(crate) unsafe fn database_club_nation_id(clube: usize) -> Option<u32> {
    if clube == 0 {
        return None;
    }
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    if nacao == 0 {
        return None;
    }
    Some(*(nacao as *const u32))
}

/// 1 when the club nation (+0x53) is not Argentina (key 0x9cf218); null club or nation: 0; the rest of eax comes from
/// the id
/// Original: `club_nation_is_not_argentina` (0x5487b0, Database.cpp, cdecl).
///
/// [pt] { 0x5487b0 } (cdecl, Database.cpp): `club_nation_is_not_argentina(clube)`: 1 se a nação do clube não tem o id
/// da chave Argentina 0x9cf218; clube ou nação nulos: 0. A resposta vai em `al` e o resto do eax é o do id (`setne
/// al` depois do `mov eax, [eax]`), como na original. Na carreira: a disciplina (0x55e300), o diário dos técnicos
/// (manager_manager 0x6ab800) e as notícias (0x782f70).
pub(crate) unsafe extern "C" fn club_nation_is_not_argentina(clube: usize) -> u32 {
    match database_club_nation_id(clube) {
        None => 0,
        Some(id) => (id & 0xffff_ff00) | (id != *(KEY_NATION_ARGENTINA as *const u32)) as u32, // a Argentina
    }
}


/// 1 when the club nation is not Argentina, France or Croatia (keys 0x9cf218, 0x9cf300, 0x9cf2ac); null: 0
/// Original: `club_nation_not_argentina_france_croatia` (0x5487d0, Database.cpp, cdecl).
///
/// [pt] { 0x5487d0 } (cdecl, Database.cpp): `club_nation_not_argentina_france_croatia(clube)`: 1 se a nação do clube não
/// tem o id de Argentina 0x9cf218, França 0x9cf300 nem Croácia 0x9cf2ac; clube ou nação nulos: 0. Em `al`, o resto
/// do eax é o do id. Na carreira, pelo diário dos técnicos (manager_manager 0x6ab800).
pub(crate) unsafe extern "C" fn club_nation_not_argentina_france_croatia(clube: usize) -> u32 {
    match database_club_nation_id(clube) {
        None => 0,
        Some(id) => {
            let chaves = [KEY_NATION_ARGENTINA, KEY_NATION_FRANCE, KEY_NATION_CROATIA]; // Argentina, França, Croácia
            (id & 0xffff_ff00) | (!chaves.iter().any(|&k| *(k as *const u32) == id)) as u32
        }
    }
}


/// 1 if the nation's id matches the id of any of the given KEY_* nations, else 0.
///
/// [pt] 1 se o id (+0) da `nacao` (não nula; a original não confere) é o de uma das `chaves`, senão 0 (o eax inteiro).
pub(crate) unsafe fn nation_is_one_of(nacao: usize, chaves: &[usize]) -> u32 {
    let id = *(nacao as *const u32);
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}

/// 1 when the nation is one of the five Melanesian ones (keys 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500)
/// Original: `database_is_melanesian_nation` (0x53d900, Database.cpp, cdecl).
///
/// [pt] { 0x53d900 } (cdecl, Database.cpp): `database_is_melanesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (as melanésias), senão 0. Nas copas da Oceania
/// (oceania_club_champ 0x798220, oceania_nations 0x799e30).
pub(crate) unsafe extern "C" fn database_is_melanesian_nation(nacao: usize) -> u32 {
    nation_is_one_of(nacao, &[KEY_NATION_PAPUA_NEW_GUINEA, KEY_NATION_SOLOMON_ISLANDS, KEY_NATION_NEW_CALEDONIA, KEY_NATION_FIJI, KEY_NATION_VANUATU]) // melanésias
}


/// 1 when the nation is one of the five Polynesian ones (keys 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514)
/// Original: `database_is_polynesian_nation` (0x53d940, Database.cpp, cdecl).
///
/// [pt] { 0x53d940 } (cdecl, Database.cpp): `database_is_polynesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (as polinésias), senão 0 (a nação não é conferida). Nas
/// copas da Oceania (oceania_club_champ 0x798220, oceania_nations 0x799e30).
pub(crate) unsafe extern "C" fn database_is_polynesian_nation(nacao: usize) -> u32 {
    let id = *(nacao as *const u32);
    let chaves = [KEY_NATION_TONGA, KEY_NATION_AMERICAN_SAMOA, KEY_NATION_TAHITI, KEY_NATION_COOK_ISLANDS, KEY_NATION_WESTERN_SAMOA]; // polinésias
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}


/// the job name (5/12 manager, 6/13 assistant, 8/15 coach; others: empty) translated into 0xae1980; with language 4
/// writes the gender (4 or 1) into the second argument
/// Original: `database_get_coaching_job_name` (0x53ac40, Database.cpp, cdecl).
///
/// [pt] { 0x53ac40 } (cdecl, Database.cpp): `database_get_coaching_job_name(cargo, genero)`: o nome do cargo (byte com
/// sinal) no texto fixo 0xae1980, pela tabela de saltos 0x53acc8: 5 e 12 "Manager", 6 e 13 "Assistant Manager",
/// 8 e 15 "Coach" (traduzidos pela 0x66f4e0); os demais, texto vazio. Com `genero` não nulo e o idioma (0x66f4d0)
/// 4, grava nele 4 (técnico, auxiliar) ou 1 (treinador). Devolve 0xae1980. Na regeneração (player_regen 0x7a9650).
pub(crate) unsafe extern "C" fn database_get_coaching_job_name(cargo: u32, genero: usize) -> usize {
    let (texto, marca) = match (cargo as i8 as i32).wrapping_sub(5) {
        0 | 7 => (0x9b6440usize, 4u8), // "Manager"
        1 | 8 => (0x9b642c, 4),        // "Assistant Manager"
        3 | 10 => (0x9b6448, 1),       // "Coach"
        _ => {
            *(G_COACHING_JOB_NAME_BUFFER as *mut u8) = 0;
            return G_COACHING_JOB_NAME_BUFFER;
        }
    };
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    formatar(G_COACHING_JOB_NAME_BUFFER, texto);
    if genero != 0 && idioma() == 4 {
        *(genero as *mut u8) = marca;
    }
    G_COACHING_JOB_NAME_BUFFER
}


/// the full years between the dates (end year minus start year, minus 1 when the start day is later) as "%d" text of an
/// unsigned word; returns 1
/// Original: `database_format_years_between_dates` (0x540340, Database.cpp, cdecl).
///
/// [pt] { 0x540340 } (cdecl, Database.cpp): `database_format_years_between_dates(fim, inicio, texto, tamanho)`: os anos
/// completos entre as datas (+0 dia do ano, +2 ano, palavras): ano(fim) − ano(início), menos 1 se o dia do início é
/// maior (com sinal); escrito em `texto` com "%d" (0x98806c) como palavra sem sinal (negativo vira 65535). Devolve 1.
/// Texto nulo, tamanho < 5 ou uma das datas nula: a original (caixa e 0). Nas telas de recordes do clube e das
/// competições (0x44aa20, 0x4a5690, 0x4a7f50).
pub(crate) unsafe extern "C" fn database_format_years_between_dates(fim: usize, inicio: usize, texto: usize, tamanho: i32) -> u32 {
    if texto == 0 || tamanho < 5 || fim == 0 || inicio == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(IDX_DATABASE_FORMAT_YEARS_BETWEEN_DATES)(fim, inicio, texto, tamanho);
    }
    let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const i16);
    let mut anos = w(fim, 2).wrapping_sub(w(inicio, 2));
    if w(inicio, 0) > w(fim, 0) {
        anos = anos.wrapping_sub(1); // o aniversário ainda não chegou
    }
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    fmt(texto, 0x98806c, anos as u16 as u32);
    1
}


/// Case-insensitive string compare (Utils string compare 0x90c1d0 with (a, b, 1, 0)) used by the Database.cpp sorts.
///
/// [pt] A comparação de textos sem maiúsculas 0x90c1d0(a, b, 1, 0) que as ordens do Database.cpp usam.
pub(crate) unsafe fn compare_text_nocase(a: *const u8, b: *const u8) -> i32 {
    let textos: unsafe extern "C" fn(*const u8, *const u8, i32, u32) -> i32 = core::mem::transmute(FN_UTILS_STRING_COMPARE);
    textos(a, b, 1, 0)
}

/// qsort comparison of clubs by short name (+0x38), case-insensitive (0x90c1d0); null club: empty text
/// Original: `database_compare_clubs_by_short_name` (0x540080, Database.cpp, cdecl).
///
/// [pt] { 0x540080 } (cdecl, Database.cpp): `database_compare_clubs_by_short_name(a, b)`: comparação do qsort de
/// ponteiros de clube pelo nome curto (+0x38) na comparação de textos 0x90c1d0 (sem maiúsculas); clube nulo vale o
/// texto vazio. A original copia os dois nomes para a pilha antes de comparar; a cópia compara no lugar (a 0x90c1d0
/// só lê). Nas telas de clubes e de busca, no Setup (0x822260) e na semeadura da UEFA (0x906410).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_short_name(a: usize, b: usize) -> i32 {
    let nome = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { b"\0".as_ptr() } else { (c + 0x38) as *const u8 } // o nome curto
    };
    compare_text_nocase(nome(a), nome(b))
}


/// qsort comparison of staff by style-1 name (0x53dba0), case-insensitive; null person: empty text
/// Original: `database_compare_staff_by_full_name` (0x540270, Database.cpp, cdecl).
///
/// [pt] { 0x540270 } (cdecl, Database.cpp): `database_compare_staff_by_full_name(a, b)`: comparação do qsort de
/// ponteiros de pessoa pelo nome no estilo 1 ("nome sobrenome", 0x53dba0 em 0x7d0 bytes) na comparação de textos
/// 0x90c1d0; pessoa nula vale o texto vazio. No menu (0x7612c0) e no gerente de transferências (0x8e55f0).
pub(crate) unsafe extern "C" fn database_compare_staff_by_full_name(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 1); // "nome sobrenome"
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 1);
    }
    compare_text_nocase(na.as_ptr(), nb.as_ptr())
}


/// 1 when the person (with a player or non-player record) has the requested first, second and common name ids (on the
/// non-null ones) and, with a date, the same birth date
/// Original: `database_staff_matches_names` (0x53e990, Database.cpp, cdecl).
///
/// [pt] { 0x53e990 } (cdecl, Database.cpp): `database_staff_matches_names(pessoa, nome, sobrenome, comum, data)`: 1 se a
/// pessoa, que tem de ter registro de jogador (+0x61) ou de não jogador (+0x69), confere: cada nome não nulo (+4, +8,
/// +0xc) tem o id (+0x33) pedido e, com `data`, a data de nascimento (+0x10 dia, +0x12 ano) é a mesma (0x4a7f20);
/// senão 0. Pessoa nula: a original (caixa e 0). Telas, regeneração e transferências (~137 chamadores).
pub(crate) unsafe extern "C" fn database_staff_matches_names(pessoa: usize, nome: u32, sobrenome: u32, comum: u32, data: usize) -> u32 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32>(IDX_DATABASE_STAFF_MATCHES_NAMES)(pessoa, nome, sobrenome, comum, data);
    }
    let ler = |d: usize| core::ptr::read_unaligned((pessoa + d) as *const usize);
    if ler(0x61) == 0 && ler(0x69) == 0 {
        return 0;
    }
    for (d, id) in [(4, nome), (8, sobrenome), (0xc, comum)] {
        let n = ler(d);
        if n != 0 && core::ptr::read_unaligned((n + 0x33) as *const u32) != id {
            return 0; // o nome não confere
        }
    }
    if data != 0 {
        let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const u16);
        if w(pessoa, 0x12) != w(data, 2) || w(pessoa, 0x10) != w(data, 0) {
            return 0; // outra data de nascimento
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
    let cidade = |c: usize| {
        let e = core::ptr::read_unaligned((c + 0x69) as *const usize);
        if e == 0 { 0 } else { core::ptr::read_unaligned((e + 0x38) as *const usize) }
    };
    let (xa, xb) = (cidade(ca), cidade(cb));
    let x = if xa != 0 { core::ptr::read_unaligned((xa + d) as *const f64) } else { read_f64_at(0x9689a0) }; // 0x9689a0 = 0
    let y = if xb != 0 { core::ptr::read_unaligned((xb + d) as *const f64) } else { 0.0 };
    if !(x >= y) {
        -1
    } else if x > y {
        1
    } else {
        0
    }
}

/// qsort comparison of clubs by the latitude (+0x23, f64) of the city (+0x38) of the stadium (+0x69), lowest first; no
/// city: 0
/// Original: `database_compare_clubs_by_latitude` (0x53fd40, Database.cpp, cdecl).
///
/// [pt] { 0x53fd40 } (cdecl, Database.cpp): `database_compare_clubs_by_latitude(a, b)`: comparação do qsort de ponteiros
/// de clube pela latitude (+0x23, f64) da cidade do estádio, a menor primeiro. Clube nulo: a original (caixa e 0).
/// Nas copas sorteadas por região (eng_auto_cup 0x56b730, cro_a1, fin_prm, ita_ser_a, nor_prm, por_prm, spa_first,
/// swe_cup).
pub(crate) unsafe extern "C" fn database_compare_clubs_by_latitude(a: usize, b: usize) -> i32 {
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
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
    let (ca, cb) = (*(a as *const usize), *(b as *const usize));
    if ca == 0 || cb == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_DATABASE_COMPARE_CLUBS_BY_LONGITUDE)(a, b);
    }
    order_by_city_coordinate(ca, cb, 0x2b) // a longitude
}


/// Counts squad members (+0xd7, 50 slots) with a player record matching `posicao`, not badly injured, valid id and
/// current ability (+5) above `limit`.
///
/// [pt] Quantos dos 50 do elenco (+0xd7) contam para as três consultas da força do elenco pelo limite /65: pessoa não
/// nula com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364]
/// (com sinal) e habilidade atual (+5 do jogador, curto com sinal) acima do `limite`. A ordem dos testes é a da
/// original (a 0x62c400 só é chamada para os da posição). Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_above_ability(clube: usize, limit: i32, posicao: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(G_INJURY), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit {
            n += 1;
        }
    }
    n
}

/// Ability threshold from club reputation: reputation/65 + 40, clamped to 40..teto.
///
/// [pt] O limite de habilidade das três: reputação do clube (+0x80, curto com sinal) / 65 + 40, entre 40 e `teto`.
pub(crate) unsafe fn squad_ability_threshold(clube: usize, teto: i32) -> i32 {
    (core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32 / 65 + 40).clamp(40, teto)
}

/// Club's per-game average of stat byte `campo` in its division (COMP_STATS entry found via 0x4a4850): byte / games
/// played; None without competition, stats or games.
///
/// [pt] A média de gols por jogo do clube na competição (+0x57): a estatística do clube (COMP_STATS 0x4a4850, thiscall
/// no +8 da competição [[0xadadfc] + 4·id]); `None` sem competição, sem estatística ou sem jogos (+4); senão o byte
/// `campo` / jogos (inteiros sem sinal). A 0x4a4850 ordena a tabela da competição na primeira chamada (marca +0x18)
/// e guarda a competição em [0xadae10]: na comparação a original vem antes e a cópia acha a tabela já ordenada.
pub(crate) unsafe fn club_goals_per_game(clube: usize, campo: usize) -> Option<i32> {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let comp = rd(clube + club::DIVISION);
    if comp == 0 {
        return None;
    }
    let c = rd(rd(G_COMP_OBJECT_TABLE).wrapping_add(rd(comp).wrapping_mul(4)));
    if c == 0 || rd(c + 8) == 0 {
        return None;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let e = estatistica(rd(c + 8), clube);
    if e == 0 {
        return None;
    }
    let jogos = *((e + 4) as *const u8) as i32;
    if jogos == 0 {
        return None;
    }
    Some(*((e + campo) as *const u8) as i32 / jogos)
}

/// club conceding more than 1.35 goals a game: 0; otherwise 1 when more than 2 defenders (+0x11 = 20) without serious
/// injury are above the ability limit (reputation/65 + 40, 40 to 145)
/// Original: `club_has_enough_defenders` (0x549100, Database.cpp, cdecl).
///
/// [pt] { 0x549100 } (cdecl, Database.cpp): `club_has_enough_defenders(clube)`: clube nulo: 0. Se o clube toma mais de
/// 1,35 gol por jogo ([0x969588]; gols sofridos +0x6a da estatística / jogos): 0. Senão, 1 se mais de 2 zagueiros
/// (posição +0x11 = 20) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day
/// 0x7b3e20, relatório de um clube).
pub(crate) unsafe extern "C" fn club_has_enough_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(clube, 0x6a) {
        if v as f64 > *(0x969588 as *const f64) {
            return 0; // a defesa toma gols demais
        }
    }
    let limit = squad_ability_threshold(clube, 0x91);
    (count_squad_above_ability(clube, limit, |j| *((j + 0x11) as *const u8) == 20) > 2) as u8 // zagueiros (dbo)
}


/// 1 when more than 2 midfielders (+0x14, +0x12 or +0x13 = 20) without serious injury are above the limit
/// (reputation/65 + 40, 40 to 145)
/// Original: `club_has_enough_midfielders` (0x549200, Database.cpp, cdecl).
///
/// [pt] { 0x549200 } (cdecl, Database.cpp): `club_has_enough_midfielders(clube)`: clube nulo: 0; senão 1 se mais de 2
/// meias (posição +0x14, +0x12 ou +0x13 = 20, nessa ordem) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_enough_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limit = squad_ability_threshold(clube, 0x91);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    (count_squad_above_ability(clube, limit, meia) > 2) as u8 // meias (dbo)
}


/// club scoring fewer than 1.5 goals a game: 0; otherwise 1 when more than 1 striker (+0x15 = 20) without serious
/// injury is above the limit (reputation/65 + 40, 40 to 150)
/// Original: `club_has_enough_strikers` (0x5492c0, Database.cpp, cdecl).
///
/// [pt] { 0x5492c0 } (cdecl, Database.cpp): `club_has_enough_strikers(clube)`: clube nulo: 0. Se o clube faz menos de
/// 1,5 gol por jogo ([0x968940]; gols marcados +0x69 da estatística / jogos): 0. Senão, 1 se mais de 1 atacante
/// (posição +0x15 = 20) passa do limite /65 (teto 150). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_enough_strikers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(clube, 0x69) {
        if (v as f64) < *(0x968940 as *const f64) {
            return 0; // o ataque faz poucos gols
        }
    }
    let limit = squad_ability_threshold(clube, 0x96);
    (count_squad_above_ability(clube, limit, |j| *((j + 0x15) as *const u8) == 20) > 1) as u8 // atacantes (dbo)
}


/// Like count_squad_above_ability but also requiring the `extra` predicate on the player record.
///
/// [pt] Quantos dos 50 do elenco (+0xd7) contam: pessoa não nula com jogador (+0x61) da `posicao`, sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5 do jogador, curto com
/// sinal) acima do `limite` e o `extra` (os atributos do jogador). Na ordem da original (a 0x62c400 só para os da
/// posição). Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_above_ability_with(clube: usize, limit: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(G_INJURY), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && extra(j) {
            n += 1;
        }
    }
    n
}

/// Club reputation (+0x80, signed 16-bit).
///
/// [pt] A reputação do clube (+0x80, curto com sinal).
pub(crate) unsafe fn club_reputation(clube: usize) -> i32 {
    core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32
}

/// 1 when more than 4 players without serious injury are above the ability limit (reputation/65 + 40, 40 to 140) with
/// attribute +0x44 above 14
/// Original: `club_has_enough_hard_workers` (0x5493c0, Database.cpp, cdecl).
///
/// [pt] { 0x5493c0 } (cdecl, Database.cpp): `club_has_enough_hard_workers(clube)`: clube nulo: 0; senão 1 se mais de 4
/// jogadores (qualquer posição) passam do limite reputação/65 + 40 (de 40 a 140) com o atributo +0x44 (byte com
/// sinal; o mesmo que a 0x549850 usa) acima de 14. Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20,
/// relatório de um clube).
pub(crate) unsafe extern "C" fn club_has_enough_hard_workers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limit = (club_reputation(clube) / 65 + 40).clamp(40, 0x8c);
    let n = count_squad_above_ability_with(clube, limit, |_| true, |j| *((j + 0x44) as *const i8) > 14);
    (n > 4) as u8 // esforcados (dbo)
}


/// 1 when some goalkeeper (+0xf = 20) in the squad has no serious injury and a valid id
/// Original: `club_has_fit_goalkeeper` (0x549470, Database.cpp, cdecl).
///
/// [pt] { 0x549470 } (cdecl, Database.cpp): `club_has_fit_goalkeeper(clube)`: 1 assim que um dos 50 do elenco (+0xd7) é
/// pessoa com jogador (+0x61) goleiro (+0xf = 20), sem lesão grave (0x62c400 ≠ 2) e id < [0xae2364]; nenhum ou clube
/// nulo: 0. Sem limite de habilidade. Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_fit_goalkeeper(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + 0xf) as *const u8) != 20 || lesao(rd(G_INJURY), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) < *(G_STAFF_COUNT as *const i32) {
            return 1; // goleiro em condicoes (dbo)
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
pub(crate) unsafe extern "C" fn club_has_ball_winning_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rep = club_reputation(clube);
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(14);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = count_squad_above_ability_with(clube, limit, meia, |j| *((j + 0x44) as *const i8) as i32 > a && *((j + 0x3e) as *const i8) as i32 > a);
    (n > 1) as u8 // meias que roubam (dbo)
}


/// Counts squad members natural (byte 20) at position offset `posicao`, not badly injured, valid id, ability above
/// rep/75+25 (40..135) and attribute above rep/500+5 (max teto).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) na `posicao` (byte = 20), sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal)
/// acima de reputação/75 + 25 (de 40 a 135) e o `atributo` (byte com sinal) acima de reputação/500 + 5 (no máximo
/// `teto`). Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_natural_at_position(clube: usize, posicao: usize, atributo: usize, teto: i32) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let rep = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(teto);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + posicao) as *const u8) != 20 || lesao(rd(G_INJURY), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && *((j + atributo) as *const i8) as i32 > a {
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
pub(crate) unsafe extern "C" fn club_lacks_aerial_strikers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x15, 0x2e, 13) <= 1) as u8 // atacantes pelo alto (dbo)
}


/// the same with defenders (+0x11 = 20) and attribute +0x2e (at most 13)
/// Original: `club_lacks_aerial_defenders` (0x5495b0, Database.cpp, cdecl).
///
/// [pt] { 0x5495b0 } (cdecl, Database.cpp): `club_lacks_aerial_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x2e acima de reputação/500 + 5 (no máximo 13). Só
/// `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_lacks_aerial_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x11, 0x2e, 13) <= 1) as u8 // zagueiros pelo alto (dbo)
}


/// the same with defenders (+0x11 = 20) and attribute +0x36 (at most 10)
/// Original: `club_lacks_fast_defenders` (0x549690, Database.cpp, cdecl).
///
/// [pt] { 0x549690 } (cdecl, Database.cpp): `club_lacks_fast_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x36 (a velocidade) acima de reputação/500 + 5 (no
/// máximo 10). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_lacks_fast_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x11, 0x36, 10) <= 1) as u8 // zagueiros rapidos (dbo)
}


/// Counts squad members matching `posicao`, not badly injured, valid id, ability above rep/75+25 (40..135) and passing
/// `extra` with the attribute threshold rep/500+5 (max teto).
///
/// [pt] Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400
/// em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal) acima de
/// reputação/75 + 25 (de 40 a 135) e o `extra` com o limite de atributo reputação/500 + 5 (no máximo `teto`).
/// Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
pub(crate) unsafe fn count_squad_by_rep_thresholds(clube: usize, teto: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize, i32) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let rep = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(teto);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(G_INJURY), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(G_STAFF_COUNT as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && extra(j, a) {
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
pub(crate) unsafe extern "C" fn club_has_fast_striker(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let n = count_squad_by_rep_thresholds(clube, 17, |j| *((j + 0x15) as *const u8) == 20, |j, a| *((j + 0x36) as *const i8) as i32 > a);
    (n > 0) as u8 // atacante rapido (dbo)
}


/// 1 when more than 1 midfielder without serious injury is above the limit with attributes +0x43 and +0x37 above
/// reputation/500 + 5 (at most 14)
/// Original: `club_has_two_creative_midfielders` (0x549940, Database.cpp, cdecl).
///
/// [pt] { 0x549940 } (cdecl, Database.cpp): `club_has_two_creative_midfielders(clube)`: clube nulo: 0; senão 1 se mais de
/// 1 meia (posição +0x14, +0x12 ou +0x13 = 20) passa dos limites com os atributos +0x43 e +0x37 (bytes com sinal)
/// acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo olheiro.
pub(crate) unsafe extern "C" fn club_has_two_creative_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = count_squad_by_rep_thresholds(clube, 14, meia, |j, a| *((j + 0x43) as *const i8) as i32 > a && *((j + 0x37) as *const i8) as i32 > a);
    (n > 1) as u8 // meias criativos (dbo)
}


/// the 0..6 band from the average ability of the best in the squad (up to 16, qsort with 0x549c30) or, without them,
/// from the reputation; adjusted by the scout club reputation and, in the same competition, by the reputation gap and
/// the [0xae2d14] table; null club: 3
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
pub(crate) unsafe extern "C" fn club_squad_strength_band(clube: usize, outro: usize) -> u8 {
    if clube == 0 {
        return 3;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd16 = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let id = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let total = *(G_STAFF_COUNT as *const i32);
    let mut v = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let p = rd(clube + club::SQUAD + 4 * k);
        if p != 0 && rd(p + 0x61) != 0 && id(p) < total {
            v[n] = p;
            n += 1;
        }
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    ordenar(v.as_mut_ptr() as usize, n as u32, 4, 0x549c30); // os melhores primeiro (dbo)
    let m = (n as i32 - 1).clamp(0, 16) as usize;
    let mut soma = 0i32;
    for &p in &v[..m] {
        soma = soma.wrapping_add(rd16(rd(p + 0x61) + 5) as i32);
    }
    let rep = rd16(clube + club::REPUTATION);
    let mut f: i8 = if m == 0 {
        match rep {
            r if r > 0x1c52 => 6,
            r if r > 0x1676 => 5,
            r if r > 0x128e => 4,
            r if r > 0xcb2 => 3,
            r => (r > 0x8ca) as i8 + 1,
        }
    } else {
        match soma / m as i32 {
            x if x > 0x9b => 6,
            x if x > 0x87 => 5,
            x if x > 0x6e => 4,
            x if x > 0x55 => 3,
            x if x > 0x3c => 2,
            x => (x > 0x28) as i8,
        }
    };
    if outro != 0 {
        let ro = rd16(outro + 0x80);
        if ro > 0x1c52 {
            if rep < 0x109a && f > 2 {
                f -= 2;
            }
            if rep < 0x1c52 && f > 0 {
                f -= 1;
            }
        } else if ro < 0x109a && f < 5 {
            f += 1; // o olheiro de clube pequeno ve o clube mais forte (dbo)
        }
        if rd(outro + 0x57) == rd(clube + club::DIVISION) {
            let d = ro.wrapping_sub(rep);
            let tab = rd(G_CLUB_STATE_TABLE);
            let marca = |c: usize| *(tab.wrapping_add((id(c).wrapping_mul(6) as usize).wrapping_add(4)) as *const i8);
            if d > 0x4e2 && f > 0 && marca(outro) >= 0 && marca(clube) <= 0 {
                f -= 1;
            } else if d < 0x4e2 && f < 5 && marca(clube) >= 0 && marca(outro) <= 0 {
                f += 1;
            }
        }
    }
    f.clamp(0, 6) as u8
}




/// Club of the person's contract as 0x545e90 finds it (contract via 0x4cf5a0, stored in G_CONTRACT_LOOKUP_RESULT; club
/// id at contract +4); 0 without contract.
///
/// [pt] O clube do contrato da `pessoa` como a 0x545e90 o acha: o contrato pela 0x4cf5a0 (this = [0xae1050]), gravado
/// em [0xae1080] como na original; com contrato, o clube de id +4 na tabela [0xae23b4] (581 bytes); sem: 0.
pub(crate) unsafe fn database_club_of_contract(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(581))
}

/// +500 if the club is one of the person's three favourite clubs, -500 if one of the three disliked, else 0.
///
/// [pt] +500 se o `clube` está entre os três clubes favoritos da pessoa (preferências +4/+8/+0xc), −500 se entre os três
/// rejeitados (+0x10/+0x14/+0x18), senão 0; sem preferências: 0.
pub(crate) unsafe fn club_preference_bonus(clube: usize, pessoa: usize) -> i32 {
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref == 0 {
        return 0;
    }
    if (0..3).any(|k| read_usize(pref + 4 + 4 * k) == clube) {
        500
    } else if (0..3).any(|k| read_usize(pref + 0x10 + 4 * k) == clube) {
        -500
    } else {
        0
    }
}

/// the club reputation (+0x80) as seen by the person: with a division stronger than the club, weighs it by the league
/// position (table 0x9b59c0, mixed with last season before 20 games) and, for someone playing at the club, by the
/// 0x7c1e30 average; ±500 when the club is a favoured or disliked one; writes [0xae1080]
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
pub(crate) unsafe extern "C" fn club_reputation_for_staff(clube: usize, pessoa: usize) -> u32 {
    let rep = read_i16(clube + club::REPUTATION);
    let comp = read_usize(clube + club::DIVISION);
    if comp == 0 {
        return rep as u16 as u32;
    }
    let liga = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    let divisao = read_i16(comp + competition::CLUB_COMP_REPUTATION);
    let alto = (divisao as i32).wrapping_mul(500);
    if liga != 0 && alto > rep as i32 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
        let e = busca(read_usize(liga + 8), clube);
        if e == 0 {
            return 0;
        }
        let n = read_i16(liga + 0x3e) as i32;
        let peso = |k: i32| read_f64_at(0x9b59c0 + 8 * k.clamp(0, 19) as usize);
        let mut posicao = *((e + 0x73) as *const u8) as i32 - 1;
        if n != 0 && n != 20 {
            posicao = posicao * 20 / n;
        }
        let mut v = peso(posicao);
        let jogos = *((e + 4) as *const u8) as i32;
        if jogos < 20 {
            let antes = *((clube + club::LAST_POSITION) as *const i8) as i32;
            let w = if read_usize(clube + club::LAST_DIVISION) != comp {
                read_f64_at(0x968de8) // 0x968de8 = 1.3
            } else if n != 0 && n != 20 {
                peso((antes * 20 - 20) / n)
            } else if (1..21).contains(&antes) {
                peso(antes - 1)
            } else {
                read_f64_at(0x968de8) // 0x968de8 = 1.3
            };
            v = ((20 - jogos) as f64 * w + jogos as f64 * v) * read_f64_at(0x967888); // 0x967888 = 0.05
        }
        let mut bonus = 0;
        if pessoa != 0 && read_usize(pessoa + staff::PLAYER) != 0 && database_club_of_contract(pessoa) == clube {
            let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
            let m = media(read_usize(G_PLAYER_STATS), read_usize(pessoa + staff::PLAYER), 6, 0x14, v as f32);
            v = (m + v) * read_f64_at(0x968968); // 0x968968 = 0.5
            bonus = club_preference_bonus(clube, pessoa);
        }
        let mut x = v * read_f64_at(0x969558) - read_f64_at(0x969550); // o peso entre 0 e 10; 0x969558 = 14, 0x969550 = 18
        let (baixo, teto) = (read_f64_at(0x9689a0), read_f64_at(0x9688b0)); // 0x9689a0 = 0, 0x9688b0 = 10
        if baixo > x {
            x = baixo;
        } else if !(teto >= x) {
            x = teto; // fcomp: acima do teto ou NaN
        }
        let cx = (divisao as i32).wrapping_mul(500) as i16 as f64; // imul cx, cx, 0x1f4: 16 bits
        let r = ((teto - x) * rep as f64 + cx * x) * read_f64_at(0x967880); // 0x967880 = 0.1
        return (match_pl_ftol(r) as u32).wrapping_add(bonus as u32);
    }
    // sem liga, ou o clube já à altura da divisão: a reputação ±500; a parte alta do eax é o que sobrou
    let mut eax = if liga == 0 { comp } else { alto as usize };
    let mut bonus = 0;
    if pessoa != 0 {
        eax = read_usize(pessoa + staff::PLAYER);
        if eax != 0 {
            eax = database_club_of_contract(pessoa);
            if eax == clube {
                eax = read_usize(pessoa + staff::STAFF_PREFERENCES);
                if eax != 0 {
                    bonus = club_preference_bonus(clube, pessoa); // os clubes da pessoa
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
pub(crate) unsafe fn count_preferred_people_at_club(pessoa: usize, clube: usize, base: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let pref = read_usize(pessoa + staff::STAFF_PREFERENCES);
    if pref == 0 || clube == 0 {
        return 0;
    }
    let mut n = 0u8;
    for k in 0..3 {
        let p = read_usize(pref + base + 4 * k);
        if p != 0 && read_usize(p + 0x39) == clube {
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
pub(crate) unsafe extern "C" fn staff_count_disliked_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    count_preferred_people_at_club(pessoa, clube, 0x28) // os rejeitados
}


/// how many of the three favoured staff (preferences +0x1c..+0x24) are at the club (+0x39)
/// Original: `staff_count_favoured_staff_at_club` (0x547ce0, Database.cpp, cdecl).
///
/// [pt] { 0x547ce0 } (cdecl, Database.cpp): `staff_count_favoured_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24) estão no clube. No diário das pessoas (0x5ce2d0).
pub(crate) unsafe extern "C" fn staff_count_favoured_staff_at_club(pessoa: usize, clube: usize) -> u8 {
    count_preferred_people_at_club(pessoa, clube, 0x1c) // os favoritos
}


/// Zeroes the first of three dword slots that holds `alvo`.
///
/// [pt] Zera a primeira das três posições (`reg`, +4, +8) que tem `alvo` (a ordem da original: achou, zera e sai).
pub(crate) unsafe fn remove_from_list3(reg: usize, alvo: usize) {
    for k in 0..3 {
        let a = (reg + 4 * k) as *mut usize;
        if core::ptr::read_unaligned(a) == alvo {
            core::ptr::write_unaligned(a, 0);
            return;
        }
    }
}

/// The person's STAFF_PREFERENCES pointer (+0x65), or 0 for a null person.
///
/// [pt] As preferências (+0x65) da pessoa não nula; pessoa nula: 0 (o eax que as 0x547c10/0x547c50/0x547f50 devolvem).
pub(crate) unsafe fn staff_preferences(pessoa: usize) -> usize {
    if pessoa == 0 {
        0
    } else {
        core::ptr::read_unaligned((pessoa + staff::STAFF_PREFERENCES) as *const usize)
    }
}

/// clears the first of the three favoured clubs of the person (preferences +4..+0xc) that is the club
/// Original: `staff_remove_favoured_club` (0x547f50, Database.cpp, cdecl).
///
/// [pt] { 0x547f50 } (cdecl, Database.cpp): `staff_remove_favoured_club(pessoa, clube)`: tira o clube dos três clubes
/// favoritos da pessoa (preferências +4/+8/+0xc, o primeiro que bate). Pessoa, preferências ou clube nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). No diário das pessoas (0x5ce2d0), quando o
/// apego ao clube acaba.
pub(crate) unsafe extern "C" fn staff_remove_favoured_club(pessoa: usize, clube: usize) -> usize {
    let pref = staff_preferences(pessoa);
    if pref != 0 && clube != 0 {
        remove_from_list3(pref + 4, clube); // os clubes favoritos
    }
    pref
}


/// clears the first of the three favoured staff of the club (+0x9b..+0xa3) that is the person
/// Original: `club_remove_favoured_staff` (0x547bd0, Database.cpp, cdecl).
///
/// [pt] { 0x547bd0 } (cdecl, Database.cpp): `club_remove_favoured_staff(clube, pessoa)`: tira a pessoa das três pessoas
/// favoritas do clube (+0x9b/+0x9f/+0xa3, o primeiro que bate). Clube ou pessoa nulos: nada. Devolve o clube (o eax
/// da original). Nas audiências das disputas (DISPUTE::process_hearings 0x569ed0).
pub(crate) unsafe extern "C" fn club_remove_favoured_staff(clube: usize, pessoa: usize) -> usize {
    if clube != 0 && pessoa != 0 {
        remove_from_list3(clube + club::FAV_STAFF1, pessoa); // os favoritos do clube
    }
    clube
}


/// clears the first of the three favoured staff of the person (preferences +0x1c..+0x24) that is the target
/// Original: `staff_remove_favoured_staff` (0x547c10, Database.cpp, cdecl).
///
/// [pt] { 0x547c10 } (cdecl, Database.cpp): `staff_remove_favoured_staff(pessoa, alvo)`: tira o alvo das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24, o primeiro que bate). Pessoa, preferências ou alvo nulos:
/// nada. Devolve o eax da original (as preferências; pessoa nula: 0). Nas disputas (0x569ed0), nas relações do
/// elenco (game_update_squad_relationships 0x5d6d10, diária), na volta da lesão (0x630ee0), no MANAGER_MANAGER
/// (0x697a7a) e na mídia (0x75a340).
pub(crate) unsafe extern "C" fn staff_remove_favoured_staff(pessoa: usize, alvo: usize) -> usize {
    let pref = staff_preferences(pessoa);
    if pref != 0 && alvo != 0 {
        remove_from_list3(pref + 0x1c, alvo); // as pessoas favoritas
    }
    pref
}


/// clears the first of the three disliked staff of the person (preferences +0x28..+0x30) that is the target
/// Original: `staff_remove_disliked_staff` (0x547c50, Database.cpp, cdecl).
///
/// [pt] { 0x547c50 } (cdecl, Database.cpp): `staff_remove_disliked_staff(pessoa, alvo)`: zera a primeira das três pessoas
/// rejeitadas da pessoa (preferências +0x28/+0x2c/+0x30) que é o alvo. Pessoa, preferências ou alvo nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). Nas relações do elenco (game_update_squad_relationships
/// 0x5d6d10, diária), na volta da lesão (0x630ee0) e na mídia (0x7414ac, 0x75a340).
pub(crate) unsafe extern "C" fn staff_remove_disliked_staff(pessoa: usize, alvo: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let pref = core::ptr::read_unaligned((pessoa + staff::STAFF_PREFERENCES) as *const usize);
    if pref != 0 && alvo != 0 {
        let reg = pref + 0x28; // as pessoas rejeitadas
        for k in 0..3 {
            let a = (reg + 4 * k) as *mut usize;
            if core::ptr::read_unaligned(a) == alvo {
                core::ptr::write_unaligned(a, 0);
                break;
            }
        }
    }
    pref
}


/// clears the counter and copies the non-null favoured staff (preferences +0x1c..+0x24) into the array, counting them
/// Original: `staff_get_favoured_staff` (0x548090, Database.cpp, cdecl).
///
/// [pt] { 0x548090 } (cdecl, Database.cpp): `staff_get_favoured_staff(pessoa, vetor, n)`: zera o byte `n` e, com pessoa,
/// preferências e vetor não nulos, copia as pessoas favoritas não nulas (preferências +0x1c, +0x20, +0x24, nessa
/// ordem) para `vetor[n]`, somando 1 a `n` a cada uma (a primeira vai direto em `vetor[0]`). Devolve `n` (o eax da
/// original). No MANAGER_MANAGER: a opinião da diretoria sobre o elenco (0x693850) e a reação dela à partida
/// (0x6aaee0).
pub(crate) unsafe extern "C" fn staff_get_favoured_staff(pessoa: usize, vetor: usize, n: *mut i8) -> *mut i8 {
    *n = 0;
    if pessoa == 0 {
        return n;
    }
    let pref = |p: usize| core::ptr::read_unaligned((p + 0x65) as *const usize);
    if pref(pessoa) == 0 || vetor == 0 {
        return n;
    }
    for (k, d) in [0x1cusize, 0x20, 0x24].into_iter().enumerate() { // as três favoritas
        let p = core::ptr::read_unaligned((pref(pessoa) + d) as *const usize);
        if p != 0 {
            let i = if k == 0 { 0 } else { *n as isize };
            core::ptr::write_unaligned((vetor as isize + 4 * i) as *mut usize, p);
            *n = (*n).wrapping_add(1);
        }
    }
    n
}


/// null club: 0; otherwise key_nation_club_in_season 0x669bd0(club, null, null): 1 when today is inside the season of
/// the club nation (the whole eax)
/// Original: `club_is_in_season` (0x547770, Database.cpp, cdecl).
///
/// [pt] { 0x547770 } (cdecl, Database.cpp): `club_is_in_season(clube)`: clube nulo, 0; senão o que
/// `key_nation_club_in_season` 0x669bd0(clube, nulo, nulo) devolve (1 = hoje dentro da temporada da nação do clube),
/// o eax inteiro (o contrato padrão 0x864550 testa o eax). Na criação: o contrato padrão das pessoas
/// (CONTRACT_MANAGER::initialise_all_staff_contracts → 0x85e960 → 0x864550) e o salário padrão das listas iniciais
/// (0x83a1b0 → 0x862720).
pub(crate) unsafe extern "C" fn club_is_in_season(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
    temporada(clube, 0, 0) // a temporada da nação do clube
}


/// 1 when the club nation (+0x53) is England, France, Germany, Italy, Spain or Holland (the index.cpp keys); null club
/// or nation: 0
/// Original: `club_is_in_major_nation` (0x549e50, Database.cpp, cdecl).
///
/// [pt] { 0x549e50 } (cdecl, Database.cpp): `club_is_in_major_nation(clube)`: 1 se a nação do clube (+0x53) tem o id
/// (+0) de uma das chaves Inglaterra 0x9cf2e4, França 0x9cf300, Alemanha 0x9cf314, Itália 0x9cf360, Espanha 0x9cf488 ou
/// Holanda 0x9cf338; clube ou nação nulos: 0. Só `al` (o resto do eax é o id da nação). Na criação, pelo contrato
/// padrão (0x864550) e pelo salário padrão (0x862720).
pub(crate) unsafe extern "C" fn club_is_in_major_nation(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [KEY_NATION_ENGLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_HOLLAND]; // Holanda (clube)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}


/// the same six-key test on the nation itself; null: 0
/// Original: `nation_is_major_nation` (0x549ea0, Database.cpp, cdecl).
///
/// [pt] { 0x549ea0 } (cdecl, Database.cpp): `nation_is_major_nation(nacao)`: o mesmo teste das seis chaves de 0x549e50
/// sobre o id (+0) da própria nação; nula: 0. Só `al`. Na carreira, pela regeneração dos jovens
/// (PLAYER_REGEN::regenerate_as_youth_player 0x7abac0).
pub(crate) unsafe extern "C" fn nation_is_major_nation(nacao: usize) -> u8 {
    if nacao == 0 {
        return 0;
    }
    let id = *(nacao as *const i32);
    let chaves = [KEY_NATION_ENGLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_HOLLAND]; // Holanda (nação)
    chaves.iter().any(|&k| *(k as *const i32) == id) as u8
}


/// The person's 79-byte record in the staff state table (G_STAFF_STATE_TABLE) holding the two relationship links
/// (+0x27/+0x37, +0x3b/+0x4b); None for a null person or id >= staff count.
///
/// [pt] O registro de contrato (79 bytes, tabela [0xae2d0c]) da `pessoa` como as 0x549c90/0x549cf0/0x549d60 o acham:
/// pessoa nula ou id (+0) ≥ [0xae2364] (comparação com sinal: id negativo passa, como na original): nenhum.
/// Os dois vínculos ficam em +0x27/+0x37 e +0x3b/+0x4b (id da outra pessoa, valor), −1 = vazio.
pub(crate) unsafe fn database_staff_state_record(pessoa: usize) -> Option<usize> {
    if pessoa == 0 {
        return None;
    }
    let id = *(pessoa as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return None;
    }
    Some((*(G_STAFF_STATE_TABLE as *const usize)).wrapping_add((id as usize).wrapping_mul(79)))
}

/// Reads an i32 at offset d of a staff state record.
pub(crate) unsafe fn read_state_i32(c: usize, d: usize) -> i32 {
    core::ptr::read_unaligned(c.wrapping_add(d) as *const i32)
}

/// Writes an i32 at offset d of a staff state record.
pub(crate) unsafe fn write_state_i32(c: usize, d: usize, v: i32) {
    core::ptr::write_unaligned(c.wrapping_add(d) as *mut i32, v)
}

/// −1 in both links (id and value) of the person contract; null person or id ≥ [0xae2364]: nothing
/// Original: `staff_clear_opinion_links` (0x549c90, Database.cpp, cdecl).
///
/// [pt] { 0x549c90 } (cdecl, Database.cpp): `staff_clear_opinion_links(pessoa)`: −1 nos dois vínculos do contrato da
/// pessoa (+0x27, +0x37, +0x3b, +0x4b). Na criação, a cada contrato gerado (0x85e960, por
/// CONTRACT_MANAGER::initialise_all_staff_contracts); na carreira, nas propostas e nas queixas.
pub(crate) unsafe extern "C" fn staff_clear_opinion_links(pessoa: usize) {
    if let Some(c) = database_staff_state_record(pessoa) {
        for k in [0usize, 0x14] {
            write_state_i32(c, k + 0x27, -1);
            write_state_i32(c, k + 0x37, -1); // o valor do vínculo vazio
        }
    }
}


/// writes the other person id and the value into the first empty link (+0x27 = −1) or else into the second,
/// overwriting; both persons must be valid
/// Original: `staff_add_opinion_link` (0x549cf0, Database.cpp, cdecl).
///
/// [pt] { 0x549cf0 } (cdecl, Database.cpp): `staff_add_opinion_link(pessoa, outra, valor)`: com as duas pessoas válidas
/// (não nulas, id < [0xae2364]), grava o id da `outra` e o `valor` no primeiro vínculo se ele está vazio (+0x27 = −1),
/// senão no segundo (+0x3b/+0x4b, por cima do que houver). Na carreira, pelos eventos do contrato (0x867700).
pub(crate) unsafe extern "C" fn staff_add_opinion_link(pessoa: usize, outra: usize, valor: i32) {
    let Some(c) = database_staff_state_record(pessoa) else { return };
    if outra == 0 {
        return;
    }
    let id = *(outra as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return;
    }
    if read_state_i32(c, 0x27) == -1 {
        write_state_i32(c, 0x27, id);
        write_state_i32(c, 0x37, valor);
    } else {
        write_state_i32(c, 0x3b, id); // o segundo vínculo
        write_state_i32(c, 0x4b, valor);
    }
}


/// empties the first link with the given value or, if not, the second
/// Original: `staff_remove_opinion_link` (0x549d60, Database.cpp, cdecl).
///
/// [pt] { 0x549d60 } (cdecl, Database.cpp): `staff_remove_opinion_link(pessoa, valor)`: esvazia (−1 no id e no valor) o
/// primeiro vínculo com esse `valor` (+0x37) ou, se não, o segundo (+0x4b); nenhum com o valor: nada. Na carreira,
/// quando a queixa passa (0x867240, 0x8674e0).
pub(crate) unsafe extern "C" fn staff_remove_opinion_link(pessoa: usize, valor: i32) {
    let Some(c) = database_staff_state_record(pessoa) else { return };
    if read_state_i32(c, 0x37) == valor {
        write_state_i32(c, 0x27, -1);
        write_state_i32(c, 0x37, -1);
    } else if read_state_i32(c, 0x4b) == valor {
        write_state_i32(c, 0x3b, -1); // esvazia o segundo vínculo
        write_state_i32(c, 0x4b, -1);
    }
}


/// the club assistant (+0xd3); without one, the partner (senior/B) team one; none: 0
/// Original: `database_get_club_assistant` (0x53cb80, Database.cpp, cdecl).
///
/// [pt] { 0x53cb80 } (cdecl, Database.cpp): `database_get_club_assistant(clube)`: o auxiliar (+0xd3) do clube; sem ele, o
/// do time parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro); nenhum: 0.
/// Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos treinadores (0x8b2880, nas
/// contratações e transferências), pela regeneração (0x7a65d0) e pelas telas.
pub(crate) unsafe extern "C" fn database_get_club_assistant(clube: usize) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_ASSISTANT)(clube);
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let a = rd(clube + club::ASSISTANT_MANAGER);
    if a != 0 {
        return a;
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        return 0;
    }
    rd(p + 0xd3) // o auxiliar do parceiro
}


/// copies the 5 club coaches into an array; with a partner team (senior/B), merges both up to 5 and clears the rest
/// Original: `database_get_club_coaches` (0x53cc30, Database.cpp, cdecl).
///
/// [pt] { 0x53cc30 } (cdecl, Database.cpp): `database_get_club_coaches(clube, vetor)`: os 5 treinadores (+0x19f) do clube no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): as 5 casas como estão. Com parceiro: os não nulos do clube e
/// depois os do parceiro até 5; o resto zerado só quando os dois juntos não enchem as 5. Contador de 8 bits na
/// original (no máximo 5). Devolve 1. Clube nulo: a original (caixa "Error" e 0). Na carreira, pela distribuição dos
/// treinadores (0x8b2880).
pub(crate) unsafe extern "C" fn database_get_club_coaches(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_COACHES)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, 0, 1);
    if p == 0 {
        for j in 0..5 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + club::COACHES + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..5 {
        let t = rd(clube + club::COACHES + 4 * j);
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for j in 0..5 {
        if n >= 5 {
            return 1;
        }
        let t = rd(p + 0x19f + 4 * j); // os treinadores do parceiro
        if t != 0 {
            core::ptr::write_unaligned(vetor.add(n), t);
            n += 1;
        }
    }
    for k in n..5 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}


/// how many valid coaches (non-null, id < [0xae2364]) the club and the partner team have together
/// Original: `database_count_club_coaches` (0x53c8d0, Database.cpp, cdecl).
///
/// [pt] { 0x53c8d0 } (cdecl, Database.cpp): `database_count_club_coaches(clube)`: quantos dos 5 treinadores (+0x19f) do
/// clube são pessoas válidas (não nulas, id (+0) < [0xae2364], com sinal) mais os do time parceiro A/B (0x540a50 com
/// nulo e 1, sempre chamada). Só `al` (o resto do eax é o do parceiro). Clube nulo: a original (caixa "Error" e 0).
/// Na carreira, pela fila da regeneração (PLAYER_REGEN::process_regen_queue 0x7a65d0).
pub(crate) unsafe extern "C" fn database_count_club_coaches(clube: usize) -> u8 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_COUNT_CLUB_COACHES)(clube);
    }
    let contar = |c: usize| {
        let total = *(G_STAFF_COUNT as *const i32);
        (0..5).filter(|&j| {
            let t = core::ptr::read_unaligned((c + 0x19f + 4 * j) as *const usize);
            t != 0 && *(t as *const i32) < total // treinador válido
        }).count() as u8
    };
    let mut n = contar(clube);
    let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let p = parceiro(clube, 0, 1);
    if p != 0 {
        n = n.wrapping_add(contar(p)); // e os do parceiro
    }
    n
}



/// the nation's three-letter code, or its English one from eng.lng (109-byte rows, by id, code at +0x4f) when another
/// language is loaded
/// Original: `nation_get_english_three_letter_name` (0x5454c0, Database.cpp, cdecl).
///
/// [pt] { 0x5454c0 } (cdecl, Database.cpp): `nation_get_english_three_letter_name(nação)`: idioma ([0x9b5984]) 0 ou
/// 1: a própria sigla (+0x53). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de
/// +0x2c, pelo id (sem conferir a quantidade), a sigla em +0x4f da linha. Quadro SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn nation_get_english_three_letter_name(nacao: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(0x53); // sigla da nação
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_THREE_LETTER_NAME)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109)).wrapping_add(0x4f) // sigla na linha
}


/// the competition's name, or its English one from eng.lng (83-byte rows: club ones by id, nation ones by id minus the
/// club count) when another language is loaded
/// Original: `comp_get_english_name` (0x5455a0, Database.cpp, cdecl).
///
/// [pt] { 0x5455a0 } (cdecl, Database.cpp): `comp_get_english_name(competição)`: idioma 0 ou 1: o próprio nome (+4).
/// Senão a tabela do eng.lng, linhas de 83 bytes: id ≥ [t+0x14] (quantas de clube): a de nação `id − [t+0x14]`
/// em [t+0x28]; senão a de clube `id` em [t+0x38] (comparação com sinal). Quadro SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn comp_get_english_name(comp: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_COMP_GET_ENGLISH_NAME)(comp);
    }
    let id = *(comp as *const i32);
    let de_clube = *((t + 0x14) as *const i32);
    if id >= de_clube {
        (*((t + 0x28) as *const usize)).wrapping_add((id.wrapping_sub(de_clube) as usize).wrapping_mul(83)) // competição de nação
    } else {
        (*((t + 0x38) as *const usize)).wrapping_add((id as usize).wrapping_mul(83)) // competição de clube
    }
}


/// the staff competition's name, or its English one from eng.lng (79-byte rows, by id) when another language is loaded
/// Original: `staff_comp_get_english_name` (0x5456a0, Database.cpp, cdecl).
///
/// [pt] { 0x5456a0 } (cdecl, Database.cpp): `staff_comp_get_english_name(competição de pessoal)`: idioma 0 ou 1: o
/// próprio nome (+4). Senão a tabela do eng.lng, linhas de 79 bytes em [t+0x3c], pelo id. SEH só no primeiro uso.
pub(crate) unsafe extern "C" fn staff_comp_get_english_name(comp: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return comp.wrapping_add(4); // nome da competição de pessoal
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STAFF_COMP_GET_ENGLISH_NAME)(comp);
    }
    let id = *(comp as *const i32);
    (*((t + 0x3c) as *const usize)).wrapping_add((id as usize).wrapping_mul(79)) // competição de pessoal na tabela
}


/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação): nem a carreira nem o save passam pelo caminho do eng.lng
/// (o jogo dos testes está em inglês). 512 casos com o idioma de 0 a 13 e uma tabela do eng.lng falsa (nunca
/// nula: o primeiro uso carregaria o arquivo de verdade), quantas de clube de 0 a 9, ids de −2 a 13; as globais
/// voltam no fim.
pub unsafe fn autoteste_dbm_1() {
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x4442_4d31_6964_696f);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 .. +0x1c quantos, +0x20 .. +0x3c linhas
    let mut registro = [0u8; 0x60];
    for _ in 0..512 {
        let z = s.proximo();
        *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = (z % 14) as u32;
        tabela[5] = ((z >> 4) % 10) as usize;
        for k in 8..16 {
            tabela[k] = 0x1000_0000 * (k - 7) + ((z >> (8 + k)) as usize & 0xffff0);
        }
        *(G_ENGLISH_NAME_TABLES as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        let r = registro.as_ptr() as usize;
        comparar_nation_get_english_three_letter_name(r);
        comparar_comp_get_english_name(r);
        comparar_staff_comp_get_english_name(r);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco idioma (nomes em inglês 1): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

/// the stadium's name, or its English one from eng.lng (52-byte rows, by id) when another language is loaded
/// Original: `stadium_get_english_name` (0x545990, Database.cpp, cdecl).
///
/// [pt] { 0x545990 } (cdecl, Database.cpp): `stadium_get_english_name(estádio)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]), linhas de 52 bytes em [t+0x34], pelo id. Quadro SEH só no
/// primeiro uso (new 0x40 e a carga 0x544fc0), que fica com a original.
pub(crate) unsafe extern "C" fn stadium_get_english_name(estadio: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return estadio.wrapping_add(4); // nome do estádio
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: primeiro uso (aloca e carrega o eng.lng sob o quadro SEH): a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_STADIUM_GET_ENGLISH_NAME)(estadio);
    }
    let id = *(estadio as *const i32);
    (*((t + 0x34) as *const usize)).wrapping_add((id as usize).wrapping_mul(52)) // estádio na tabela
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
pub(crate) unsafe extern "C" fn database_language_file_exists(idioma: i32) -> i32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_UTILS_FILE_EXISTS);
    match LANGUAGE_FILES.get(idioma as u32 as usize) {
        Some(&nome) => existe(caminho(0x987da0, nome)), // "CM3_TEMP"
        None => 0,
    }
}


/// Calls `soltar` on each non-null of the 8 eng.lng table rows (+0x20..+0x3c) of the table.
///
/// [pt] As 8 linhas da tabela do eng.lng (+0x20 .. +0x3c), na ordem, cada uma não nula entregue a `soltar`.
pub(crate) unsafe fn for_each_english_table(t: usize, mut soltar: impl FnMut(usize)) {
    for k in 0..8 {
        let p = *((t + 0x20 + 4 * k) as *const usize); // linha da tabela em inglês
        if p != 0 {
            soltar(p);
        }
    }
}

/// frees the 8 non-null rows of the eng.lng table, +0x20 to +0x3c, without clearing the pointers
/// Original: `eng_lng_free_tables` (0x544f30, Database.cpp, thiscall).
///
/// [pt] { 0x544f30 } (thiscall, Database.cpp): `eng_lng_free_tables(tabela)`: `free` 0x9452ca de cada uma das 8 linhas
/// não nulas, de +0x20 a +0x3c (os ponteiros ficam; quem chama, `load_database` 0x528503 e a troca de idioma
/// 0x533aa6, apaga a tabela com `delete` 0x944c94 e zera [0xae2408]).
pub(crate) unsafe extern "thiscall" fn eng_lng_free_tables(t: usize) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    for_each_english_table(t, |p| free(p));
}

/// Comparador: soltar duas vezes soltaria os mesmos blocos duas vezes.
// simplification: só a original (como as da 232a); a cópia é provada pelo autoteste_dbm_2 (a ordem dos blocos soltos).

/// Autoteste (CM_AUTOTESTE=1 ou =banco, modo comparação). (1) O estádio: 256 casos com o idioma de 0 a 13 e uma
/// tabela do eng.lng falsa (nunca nula), ids de −2 a 13. (2) O `.lng` do idioma: idiomas de −3 a 16 contra a
/// original (resposta e caminho montado). (3) Soltar as tabelas: 64 tabelas falsas com linhas nulas ou não
/// sorteadas; a cópia entrega a uma lista (sem soltar nada), que tem de ser as não nulas de +0x20 a +0x3c, na
/// ordem da original (a original solta de verdade e não dá para rodar as duas). As globais voltam no fim.
pub unsafe fn autoteste_dbm_2() {
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x4442_4d32_6573_7461);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
        *(G_DATABASE_NAMES_LANGUAGE as *mut u32) = (z % 14) as u32;
        tabela[13] = 0x3000_0000 + ((z >> 8) as usize & 0xffff0);
        *(G_ENGLISH_NAME_TABLES as *mut usize) = tabela.as_mut_ptr() as usize;
        *(registro.as_mut_ptr() as *mut i32) = ((z >> 32) % 16) as i32 - 2;
        comparar_stadium_get_english_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    for idioma in -3..=16 {
        comparar_database_language_file_exists(idioma);
    }
    for _ in 0..64 {
        let z = s.proximo();
        for k in 0..8 {
            tabela[8 + k] = if (z >> k) & 1 == 0 { 0 } else { 0x1000_0000 * (k + 1) + ((z >> (8 + k)) as usize & 0xffff0) };
        }
        let esperado: Vec<usize> = tabela[8..].iter().copied().filter(|&p| p != 0).collect();
        let mut soltos = Vec::new();
        for_each_english_table(tabela.as_ptr() as usize, |p| soltos.push(p));
        PLACAR.registrar(soltos == esperado, || format!("eng_lng_free_tables: soltos {:x?} esperados {:x?}", soltos, esperado));
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco idioma (estádio, .lng, soltar): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

/// Shows the Database.cpp error box ("v%s %s.%s %d" with the line) and clears G_MOUSE_LEFT_BUTTON_DOWN; the game
/// continues.
///
/// [pt] A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
pub(crate) unsafe fn database_error_box(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b5ac8, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
}

/// Database.cpp error box followed by fatal cleanup and exit(-1).
///
/// [pt] A caixa do Database.cpp seguida do fim do jogo (0x5cc7d0, 0x5a8f60 e exit(−1)), como a original sem memória.
pub(crate) unsafe fn database_fatal_error(linha: u32) -> ! {
    let antes: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let depois: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    database_error_box(linha);
    antes();
    depois();
    sair(-1)
}



/// True for a club of England (KEY_NATION_ENGLAND) with reputation (+0x80) at most 500, which 0x527240 and 0x5267c0
/// treat separately.
///
/// [pt] Clube da nação da chave 0x9cf2e4 com reputação (+0x80, 16 bits com sinal) até 500: 0x527240 e 0x5267c0 o tratam à parte.
pub(crate) unsafe fn is_weak_english_club(c: usize) -> bool {
    let nac = read_usize(c + 0x53);
    nac != 0 && read_i32(nac) == read_i32(KEY_NATION_ENGLAND) && core::ptr::read_unaligned((c + 0x80) as *const i16) <= 500
}

/// Initial check of 0x527240: some person with job 8 (+0x3d) and a club (+0x39) that is not in that club's 5 coach
/// slots (+0x19f) [inference: job 8 = coach].
///
/// [pt] A conferência inicial da 0x527240: alguma pessoa de cargo 8 (+0x3d) com clube (+0x39) que não a tem nas 5 casas +0x19f.
pub(crate) unsafe fn any_coach_missing_from_club() -> bool {
    let mut i = 0i32;
    while i < read_i32(G_STAFF_COUNT) {
        let s = read_usize(G_STAFF_TABLE) + i as usize * 110;
        let c = read_usize(s + 0x39);
        if *((s + 0x3d) as *const u8) == 8 && c != 0 && !(0..5).any(|j| read_usize(c + 0x19f + 4 * j) == s) {
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
pub(crate) unsafe fn make_extra_staff(nacao: usize, gerados: i32) -> usize {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_FIRST_NAME_FOR_NATION);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_PICK_SECOND_NAME_FOR_NATION);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let s = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(G_STAFF_COUNT) - gerados - 0x11) as usize * 110);
    core::ptr::write_unaligned((s + 4) as *mut usize, primeiro(nacao));
    core::ptr::write_unaligned((s + 8) as *mut usize, segundo(nacao));
    let comum = |k: i32| read_usize(G_COMMON_NAME_TABLE).wrapping_add((k as usize).wrapping_mul(0x3c));
    let guardada = read_i32(G_EMPTY_COMMON_NAME_INDEX);
    let nome = if guardada != -1 {
        comum(guardada)
    } else {
        let n = read_i32(G_COMMON_NAME_COUNT);
        match (0..n).find(|&j| *(comum(j) as *const u8) == 0) {
            Some(j) => {
                *(G_EMPTY_COMMON_NAME_INDEX as *mut i32) = j;
                comum(j)
            }
            None => comum(sorteio(n)),
        }
    };
    core::ptr::write_unaligned((s + 0xc) as *mut usize, nome);
    s
}

/// Sets the extra person's birth date (+0x10): year 1940 + rand(26), month rand(11), day rand(27)+1, drawn in that
/// order.
///
/// [pt] A data de nascimento da pessoa extra (+0x10, 0x54a130): ano 1940 + sorteio(26), mês sorteio(11), dia sorteio(27) + 1,
/// sorteados nessa ordem.
pub(crate) unsafe fn set_extra_staff_birth_date(s: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_SET);
    let ano = sorteio(0x1a) + 0x794; // 1940 + até 25
    let mes = sorteio(0xb);
    let dia = sorteio(0x1b) + 1;
    acertar_data(s + 0x10, dia as u32, mes as u32, ano as u32, -1);
}

/// fills coaches, scouts and physios of the clubs of chosen nations by reputation (reserve rows, names of the nation,
/// random birth date) and creates unattached managers for clubs without one; over 23,000: box and 0
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
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16) as i32;
    let byte = |a: usize| *(a as *const u8);
    if any_coach_missing_from_club() {
        database_error_box(0x62f);
    }
    let selecao = |c: usize| gi(c) >= gi(0xae235c) - 2 * gi(0xae2350);
    let mut gerados = 0i32;
    for &(d, k, cargo, linha) in &[(0x19fusize, 5usize, 8u8, 0x65fu32), (0x1b3, 7, 9, 0x6aa), (0x1cf, 3, 10, 0x6f5)] {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            let c = rd(G_CLUB_TABLE) + i as usize * 581;
            i += 1;
            if is_weak_english_club(c) || selecao(c) {
                continue;
            }
            let nac = rd(c + 0x53);
            if nac == 0 || byte(nac + 0x11c) == 0 {
                continue;
            }
            let mut eh_a = 0u32;
            if parceiro(c, &mut eh_a, 1) != 0 && eh_a != 0 {
                continue;
            }
            let mut tem = (0..k).filter(|&j| rd(c + d + 4 * j) != 0).count() as i32;
            let alvo = ((rep(c) + 500) * k as i32 / 10000).clamp(1, k as i32); // a comissão pela reputação
            while tem < alvo {
                tem += 1;
                if gerados >= 23000 {
                    database_error_box(linha);
                    return 0;
                }
                let s = make_extra_staff(rd(c + 0x53), gerados);
                core::ptr::write_unaligned((s + 0x1a) as *mut usize, rd(c + 0x53));
                *((s + 0x5f) as *mut u8) = 1;
                set_extra_staff_birth_date(s);
                if let Some(j) = (0..k).find(|&j| rd(c + d + 4 * j) == 0) {
                    core::ptr::write_unaligned((c + d + 4 * j) as *mut usize, s);
                    core::ptr::write_unaligned((s + 0x39) as *mut usize, c);
                    *((s + 0x3d) as *mut u8) = cargo;
                }
                gerados += 1;
            }
        }
    }
    let mut i = 0i32;
    while i < gi(0xae235c) {
        let c = rd(G_CLUB_TABLE) + i as usize * 581;
        i += 1;
        if is_weak_english_club(c) {
            continue;
        }
        let nac = rd(c + 0x53);
        let vale = selecao(c) || (nac != 0 && rd(c + 0x57) != 0 && byte(nac + 0x11c) != 0 && rep(c) != 0) || rep(c) > 7500;
        if !vale || rd(c + 0xcf) != 0 {
            continue;
        }
        if gerados >= 23000 {
            database_error_box(0x734);
            return 0;
        }
        let s = make_extra_staff(nac, gerados);
        *((s + 0x5f) as *mut u8) = 1;
        *((s + 0x3d) as *mut u8) = 5; // treinador sem clube
        core::ptr::write_unaligned((s + 0x39) as *mut usize, 0);
        core::ptr::write_unaligned((s + 0x24) as *mut usize, 0);
        *((s + 0x28) as *mut u8) = 5;
        set_extra_staff_birth_date(s);
        gerados += 1;
    }
    formatar(0xde1f64, 0x9b5b10, gerados);
    1
}

/// trims the club table by the database size: the best per nation, those referred by staff and histories, national
/// teams, keys and clubs of chosen nations; swaps the pointers, compacts, builds [0xae23fc] and reallocates in place
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
pub(crate) unsafe extern "C" fn database_trim_club_table(tamanho: u32) {
    if *(G_NATION_TABLE as *const usize) == 0 {
        return original_fn::<unsafe extern "C" fn(u32)>(IDX_DATABASE_TRIM_CLUB_TABLE)(tamanho);
    }
    crate::comparacao::ramo("criação cortar a tabela de clubes");
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let remapear: unsafe extern "C" fn(i32, i32) -> u32 = core::mem::transmute(FN_REMAP_CLUB_KEY_ID);
    let gi = |a: usize| read_i32(a);
    let rd = |a: usize| read_usize(a);
    let wi = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
    let rep_id = |id: i32| rep(rd(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(581)));
    let byte = |a: usize| *(a as *const u8);
    let novos = novo((gi(0xae235c) as u32).wrapping_mul(4));
    if novos == 0 {
        return;
    }
    let t = tamanho as u8;
    let pequena = t == 2 || t == 1;
    let marcados = calloc(gi(0xae235c) as u32, 4);
    if marcados == 0 {
        database_fatal_error(0x513);
    }
    let marcar = |id: i32| wi(marcados.wrapping_add((id as usize).wrapping_mul(4)), 1);
    let ultimo = rd(G_CLUB_TABLE).wrapping_add((gi(0xae235c) as usize).wrapping_mul(581)).wrapping_sub(581);
    let nacoes = calloc(gi(0xae2350) as u32, 0xd0);
    if nacoes == 0 {
        database_fatal_error(0x51b);
    }
    let limit = if pequena { 15 } else { 20 }; // clubes por nação
    let chaves = [KEY_NATION_ENGLAND, KEY_NATION_ITALY, KEY_NATION_GERMANY, KEY_NATION_SPAIN, KEY_NATION_FRANCE, KEY_NATION_NORWAY, KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA, KEY_NATION_DENMARK, KEY_NATION_BELGIUM,
        KEY_NATION_HOLLAND, KEY_NATION_JAPAN, KEY_NATION_PORTUGAL, KEY_NATION_SCOTLAND, KEY_NATION_SWEDEN];
    let mut c = rd(G_CLUB_TABLE);
    while c <= ultimo {
        let nac = rd(c + 0x53);
        if nac != 0 {
            let id = gi(nac);
            let l = nacoes.wrapping_add((id as usize).wrapping_mul(0xd0));
            let cont = gi(l + 0xc8);
            let forte = byte(nac + 0x11c) != 0 || chaves.iter().any(|&k| id == gi(k));
            let entra = forte || match t {
                2 => cont <= 4,
                1 => cont <= 10,
                _ => true,
            };
            if entra {
                if cont < limit {
                    wi(l + cont as usize * 4, gi(c));
                    if cont == 0 || rep(c) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                        wi(l + 0xcc, cont);
                    }
                    wi(l + 0xc8, cont + 1);
                } else if rep(c) > rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                    wi(l + gi(l + 0xcc) as usize * 4, gi(c));
                    for j in 0..cont {
                        if j == 0 || rep_id(gi(l + j as usize * 4)) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                            wi(l + 0xcc, j);
                        }
                    }
                }
            }
        }
        c += 581;
    }
    let mut k = 0i32;
    while k < gi(0xae2350) {
        let l = nacoes + k as usize * 0xd0;
        for j in 0..gi(l + 0xc8) {
            marcar(gi(l + j as usize * 4));
        }
        k += 1;
    }
    livre(nacoes);
    let mut s = rd(G_STAFF_TABLE);
    let fim = s.wrapping_add((gi(0xae2364) as usize).wrapping_mul(110)).wrapping_sub(110);
    while s <= fim {
        for &d in &[0x39usize, 0x24] { // o clube e a seleção da pessoa ficam
            let p = rd(s + d);
            if p != 0 {
                marcar(gi(p));
            }
        }
        s += 110;
    }
    let mut h = rd(G_CLUB_COMP_HISTORY_TABLE);
    let fim = h.wrapping_add((gi(0xae237c) as usize).wrapping_mul(0x1a)).wrapping_sub(0x1a);
    while h <= fim {
        for &d in &[0xausize, 0xe, 0x12] {
            let p = rd(h + d);
            if p != 0 {
                let id = gi(p);
                if id < gi(0xae235c) && id >= 0 {
                    marcar(id);
                }
            }
        }
        h += 0x1a;
    }
    if !pequena {
        let mut h = rd(G_STAFF_HISTORY_TABLE);
        let fim = h.wrapping_add((gi(0xae2374) as usize).wrapping_mul(0x11)).wrapping_sub(0x11);
        while h <= fim {
            let clube = rd(h + 0xa);
            let q = rd(h + 4);
            if clube != 0 && q != 0 {
                let nq = rd(q + 0x1a);
                let nc = rd(clube + club::NATION);
                if (nq != 0 && byte(nq + 0x11c) != 0) || (nc != 0 && byte(nc + 0x11c) != 0) {
                    marcar(gi(clube));
                }
            }
            h += 0x11;
        }
    }
    let tabela = rd(G_CLUB_TABLE);
    let mut destino = tabela;
    let mut c = tabela;
    while c <= ultimo {
        let id = gi(c);
        let fica = if remapear(id, (destino.wrapping_sub(tabela) as i32) / 581) != 0
            || gi(marcados.wrapping_add((id as usize).wrapping_mul(4))) != 0
            || id >= gi(0xae235c) - 2 * gi(0xae2350)
        {
            true
        } else {
            let nac = rd(c + 0x53);
            !(nac == 0 || byte(nac + 0x11c) == 0 || (gi(nac) == gi(KEY_NATION_ENGLAND) && rep(c) <= 500 && pequena))
        };
        let casa = novos.wrapping_add((id as usize).wrapping_mul(4));
        if fica {
            wr(casa, destino);
            destino += 581;
        } else {
            wr(casa, 0);
        }
        c += 581;
    }
    livre(marcados);
    // os ponteiros de clube pelos novos (o id antigo está em +0 do alvo, ainda no lugar velho)
    let trocar = |campo: usize| {
        let p = rd(campo);
        wr(campo, if p == 0 { 0 } else { rd(novos.wrapping_add((gi(p) as usize).wrapping_mul(4))) });
    };
    for &(cont, base, tam, campos) in &[
        (0xae235cusize, G_CLUB_TABLE, 581usize, &[0xb3usize, 0xb7, 0xbb][..]),
        (0xae2364, G_STAFF_TABLE, 110, &[0x39, 0x24][..]),
        (0xae2370, G_PREFERENCES_TABLE, 0x34, &[4, 8, 0xc, 0x10, 0x14, 0x18][..]),
        (0xae2374, G_STAFF_HISTORY_TABLE, 0x11, &[0xa][..]),
        (0xae237c, G_CLUB_COMP_HISTORY_TABLE, 0x1a, &[0xa, 0xe, 0x12, 0x16][..]),
    ] {
        let mut i = 0i32;
        while i < gi(cont) {
            for &d in campos {
                trocar(rd(base) + i as usize * tam + d);
            }
            i += 1;
        }
    }
    let n = gi(0xae235c);
    let ficam = (0..n).filter(|&i| rd(novos + i as usize * 4) != 0).count();
    let ids = alocar((ficam as u32) << 2);
    *(G_CLUB_ENGLISH_NAME_INDEX_TABLE as *mut usize) = ids;
    if ids == 0 {
        apagar(novos);
        return;
    }
    let mut linha = 0i32;
    for i in 0..n {
        let d = rd(novos + i as usize * 4);
        if d != 0 {
            // para a frente sobre a própria tabela (destino ≤ origem), como o rep movsd da original
            core::ptr::copy((rd(G_CLUB_TABLE) + i as usize * 581) as *const u8, d as *mut u8, 581);
            wi(rd(G_CLUB_ENGLISH_NAME_INDEX_TABLE) + linha as usize * 4, i);
            wi(d, linha);
            linha += 1;
        }
    }
    *(G_CLUB_COUNT as *mut i32) = linha;
    let r = realocar(rd(G_CLUB_TABLE), (linha as u32).wrapping_mul(581));
    *(G_NAT_CLUB_TABLE as *mut usize) = rd(G_CLUB_TABLE).wrapping_add(((linha - gi(0xae2360)) as usize).wrapping_mul(581));
    if r != rd(G_CLUB_TABLE) {
        database_fatal_error(0x4ca);
    }
    apagar(novos);
    formatar(0xde1f64, 0x9b5af4, gi(0xae235c));
}

/// keeps in the club competition history only the rows of competitions with no nation or a chosen one, reallocates,
/// sets the national part and sorts (qsort with 0x540780)
/// Original: `database_prune_comp_history_table` (0x539ae0, Database.cpp, cdecl).
///
/// [pt] { 0x539ae0 } (cdecl, Database.cpp): `database_prune_comp_history_table()`, uma vez na criação. Compacta o histórico
/// de competições de clube ([0xae23d4], [0xae237c] linhas de 26 bytes): fica a linha com competição (+4) cuja nação
/// (+0x5d) é nula ou escolhida (+0x11c), copiada para a frente na própria tabela. realloc (0x945667) ao tamanho novo,
/// que vira [0xae23d4] (sem conferir o endereço); [0xae237c] = linhas; [0xae23dc] (a parte das seleções) = tabela +
/// (linhas − [0xae2384]) × 26; qsort (0x945333) com 0x540780.
pub(crate) unsafe extern "C" fn database_prune_comp_history_table() {
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    crate::comparacao::ramo("criação podar o histórico de competições");
    let mut k = 0usize;
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COMP_HISTORY_COUNT) {
        let l = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * 0x1a;
        let comp = read_usize(l + 4);
        if comp != 0 {
            let nac = read_usize(comp + competition::CLUB_COMP_NATION);
            if nac == 0 || *((nac + 0x11c) as *const u8) != 0 { // a linha fica
                core::ptr::copy(l as *const u8, (read_usize(G_CLUB_COMP_HISTORY_TABLE) + k * 0x1a) as *mut u8, 0x1a);
                k += 1;
            }
        }
        i += 1;
    }
    let t = realocar(read_usize(G_CLUB_COMP_HISTORY_TABLE), (k * 0x1a) as u32);
    let selecoes = (k as i32).wrapping_sub(read_i32(G_NATION_COMP_HISTORY_COUNT));
    *(G_CLUB_COMP_HISTORY_TABLE as *mut usize) = t;
    *(G_CLUB_COMP_HISTORY_COUNT as *mut i32) = k as i32;
    *(G_NATION_COMP_HISTORY_TABLE as *mut usize) = t.wrapping_add((selecoes as usize).wrapping_mul(0x1a));
    ordenar(t, k as u32, 0x1a, 0x540780);
}

/// First differing byte between the original's and the port's memory snapshots: (address, original, port).
///
/// [pt] O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
pub(crate) fn first_memory_difference(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}


// simplification: compacta e realoca a tabela de clubes, aloca [0xae23fc] e troca as chaves de clube; não roda duas
// vezes. Só a original; a cópia leva o ramo e é provada pela prova do save (o .qst tem a tabela cortada).

// simplification: compacta e realoca o histórico de competições; não roda duas vezes. Só a original; a cópia é provada
// pela prova do save (o histórico podado vai para o .qst).


/// As tabelas do banco: (ponteiro, quantidade, bytes por linha).
pub(crate) const DATABASE_TABLES: [(usize, usize, usize); 12] = [
    (G_CLUB_TABLE, 0xae235c, 581), (G_NATION_TABLE, 0xae2350, 0x122), (G_OFFICIAL_TABLE, 0xae2390, 0x2b), (G_STADIUM_TABLE, 0xae2358, 0x4e),
    (G_PREFERENCES_TABLE, 0xae2370, 0x34), (G_NON_PLAYER_TABLE, 0xae2368, 0x44), (G_STAFF_TABLE, 0xae2364, 0x6e), (G_STAFF_COMP_TABLE, 0xae2388, 0x65),
    (G_STAFF_HISTORY_TABLE, 0xae2374, 0x11), (G_CITY_TABLE, 0xae2354, 0x38), (G_CLUB_COMP_HISTORY_TABLE, 0xae237c, 0x1a), (G_CLUB_COMP_TABLE, 0xae2378, 0x6b),
];

/// Os campos de pessoa que, fora da faixa, abrem a caixa na 0x5315a0: (campo, teto, tabela, bytes, texto, linha).
pub(crate) const PERSON_RANGE_CHECK_FIELDS: [(usize, usize, usize, usize, usize, u32); 3] = [
    (0x61, 0xae236c, G_PLAYER_DATA_TABLE, 0x46, 0x9b60b4, 0x18f6), // jogador
    (0x65, 0xae2370, G_PREFERENCES_TABLE, 0x34, 0x9b6088, 0x1907), // preferências
    (0x69, 0xae2368, G_NON_PLAYER_TABLE, 0x44, 0x9b605c, 0x1918), // não jogador
];

/// replaces every id field of the 12 tables with the record pointer, with the limits, the 0xde1f64 log lines, the boxes
/// for staff with bad player, preferences or non-player data and the original bugs (club +0x5b clears +0x60; stadium
/// +0x48 clears the city)
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
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let alvo = |base: usize, v: i32, tam: usize| rd(base).wrapping_add((v as usize).wrapping_mul(tam));
    // sem teto: negativo → nulo
    let a = |f: usize, base: usize, tam: usize| {
        let v = gi(f);
        wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
    };
    // teto inclusivo: negativo → nulo; acima de [teto] → Some(v), quem chama decide; senão o registro
    let b = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v < 0 {
            wr(f, 0);
            None
        } else if v > gi(teto) {
            Some(v)
        } else {
            wr(f, alvo(base, v, tam));
            None
        }
    };
    // teto exclusivo: a partir de [teto] → Some(v), quem chama decide; negativo → nulo; senão o registro
    let c = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v >= gi(teto) {
            Some(v)
        } else {
            wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
            None
        }
    };
    let linhas = |cont: usize, base: usize, tam: usize, f: &dyn Fn(usize, i32)| {
        let mut i = 0i32;
        while i < gi(cont) {
            f(rd(base) + i as usize * tam, i);
            i += 1;
        }
    };
    linhas(0xae235c, G_CLUB_TABLE, 581, &|e, i| {
        if let Some(v) = b(e + 0x53, 0xae2350, G_NATION_TABLE, 0x122) {
            formatar(0xde1f64, 0x9b62b0, v);
            wr(e + 0x53, 0);
        }
        if b(e + 0x57, 0xae2378, G_CLUB_COMP_TABLE, 0x6b).is_some() {
            wr(e + 0x57, 0);
        }
        if let Some(v) = b(e + 0x5b, 0xae2378, G_CLUB_COMP_TABLE, 0x6b) {
            formatar(0xde1f64, 0x9b6274, v);
            wr(e + 0x60, 0); // erro do original: zera +0x60 e +0x5b fica cru
        }
        if b(e + 0x60, 0xae2378, G_CLUB_COMP_TABLE, 0x6b).is_some() {
            wr(e + 0x60, 0);
        }
        if let Some(v) = b(e + 0x69, 0xae2358, G_STADIUM_TABLE, 0x4e) {
            formatar(0xde1f64, 0x9b6240, v);
            wr(e + 0x69, 0);
        }
        for &(d, teto, base, tam) in &[
            (0x6eusize, 0xae2358usize, G_STADIUM_TABLE, 0x4eusize),
            (0x83, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x8b, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x93, 0xae2394, G_COLOUR_TABLE, 0x3a),
            (0x87, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x8f, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x97, 0xae2394, G_COLOUR_TABLE, 0x3a),
            (0x9b, 0xae2364, G_STAFF_TABLE, 0x6e), (0x9f, 0xae2364, G_STAFF_TABLE, 0x6e), (0xa3, 0xae2364, G_STAFF_TABLE, 0x6e),
            (0xa7, 0xae2364, G_STAFF_TABLE, 0x6e), (0xab, 0xae2364, G_STAFF_TABLE, 0x6e), (0xaf, 0xae2364, G_STAFF_TABLE, 0x6e),
            (0xb3, 0xae235c, G_CLUB_TABLE, 581), (0xb7, 0xae235c, G_CLUB_TABLE, 581), (0xbb, 0xae235c, G_CLUB_TABLE, 581),
        ] {
            if b(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
        for j in 0..50 {
            let f = e + 0xd7 + 4 * j;
            let v = gi(f);
            if leniente == 0 && v >= gi(0xae2364) { // o elenco sem as linhas a mais
                if v > gi(0xae2364) + 0x52d {
                    formatar(0xde1f64, 0x9b6218, i, v);
                }
                wr(f, 0);
            } else {
                wr(f, if v < 0 { 0 } else { alvo(G_STAFF_TABLE, v, 0x6e) });
            }
        }
        // olheiros, técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar, os 20 escalados
        for &(d, k, texto) in &[(0x1b3usize, 7usize, 0x9b61f4usize), (0x19f, 5, 0x9b61d0), (0x1cf, 3, 0x9b61ac), (0xc3, 3, 0x9b6188),
            (0xcf, 1, 0x9b6164), (0xbf, 1, 0x9b6140), (0xd3, 1, 0x9b6110), (0x1e0, 20, 0x9b60dc)]
        {
            for j in 0..k {
                let f = e + d + 4 * j;
                if let Some(v) = c(f, 0xae2364, G_STAFF_TABLE, 0x6e) {
                    formatar(0xde1f64, texto, i, v);
                    wr(f, 0);
                }
            }
        }
    });
    linhas(0xae2350, G_NATION_TABLE, 0x122, &|e, _| {
        for &(d, base, tam) in &[(0x71usize, G_CONTINENT_TABLE, 0xc6usize), (0x7a, G_CITY_TABLE, 0x38), (0x80, G_STADIUM_TABLE, 0x4e),
            (0x90, G_COLOUR_TABLE, 0x3a), (0x98, G_COLOUR_TABLE, 0x3a), (0xa0, G_COLOUR_TABLE, 0x3a), (0x94, G_COLOUR_TABLE, 0x3a),
            (0x9c, G_COLOUR_TABLE, 0x3a), (0xa4, G_COLOUR_TABLE, 0x3a), (0x110, G_NATION_TABLE, 0x122), (0x114, G_NATION_TABLE, 0x122),
            (0x118, G_NATION_TABLE, 0x122)]
        {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2390, G_OFFICIAL_TABLE, 0x2b, &|e, _| {
        for &(d, base, tam) in &[(4usize, G_FIRST_NAME_TABLE, 0x3cusize), (8, G_SECOND_NAME_TABLE, 0x3c), (0x16, G_NATION_TABLE, 0x122), (0x1a, G_CITY_TABLE, 0x38)] {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2358, G_STADIUM_TABLE, 0x4e, &|e, _| {
        if b(e + 0x38, 0xae2354, G_CITY_TABLE, 0x38).is_some() {
            wr(e + 0x38, 0);
        }
        if b(e + 0x48, 0xae2358, G_STADIUM_TABLE, 0x4e).is_some() {
            wr(e + 0x38, 0); // erro do original: zera a cidade e +0x48 fica cru
        }
    });
    linhas(0xae2370, G_PREFERENCES_TABLE, 0x34, &|e, _| {
        for d in (4..=0x18).step_by(4) {
            a(e + d, G_CLUB_TABLE, 581);
        }
        for d in (0x1c..=0x30).step_by(4) {
            a(e + d, G_STAFF_TABLE, 0x6e);
        }
    });
    linhas(0xae2368, G_NON_PLAYER_TABLE, 0x44, &|e, _| {
        for d in (0x23..=0x3f).step_by(4) {
            if c(e + d, 0xae2364, G_STAFF_TABLE, 0x6e).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2364, G_STAFF_TABLE, 0x6e, &|e, i| {
        for &(d, base, tam) in &[(4usize, G_FIRST_NAME_TABLE, 0x3cusize), (8, G_SECOND_NAME_TABLE, 0x3c), (0xc, G_COMMON_NAME_TABLE, 0x3c),
            (0x1a, G_NATION_TABLE, 0x122), (0x39, G_CLUB_TABLE, 581), (0x1e, G_NATION_TABLE, 0x122), (0x24, G_CLUB_TABLE, 581)]
        {
            a(e + d, base, tam);
        }
        for &(d, teto, base, tam, texto, linha) in PERSON_RANGE_CHECK_FIELDS.iter() {
            if let Some(v) = c(e + d, teto, base, tam) {
                formatar(0xde1f64, texto, i, v);
                database_error_box(linha);
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2388, G_STAFF_COMP_TABLE, 0x65, &|e, _| {
        for &(d, teto, base, tam) in &[(0x53usize, 0xae234cusize, G_CONTINENT_TABLE, 0xc6usize), (0x57, 0xae2350, G_NATION_TABLE, 0x122),
            (0x5b, 0xae2394, G_COLOUR_TABLE, 0x3a), (0x5f, 0xae2394, G_COLOUR_TABLE, 0x3a)]
        {
            if c(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2374, G_STAFF_HISTORY_TABLE, 0x11, &|e, _| {
        a(e + 4, G_STAFF_TABLE, 0x6e);
        a(e + 0xa, G_CLUB_TABLE, 581);
    });
    linhas(0xae2354, G_CITY_TABLE, 0x38, &|e, _| {
        a(e + 0x1f, G_NATION_TABLE, 0x122); // a nação da cidade
    });
    linhas(0xae237c, G_CLUB_COMP_HISTORY_TABLE, 0x1a, &|e, _| {
        a(e + 4, G_CLUB_COMP_TABLE, 0x6b);
        a(e + 0xa, G_CLUB_TABLE, 581);
        a(e + 0xe, G_CLUB_TABLE, 581);
        for &d in &[0x12usize, 0x16] {
            if b(e + d, 0xae235c, G_CLUB_TABLE, 581).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2378, G_CLUB_COMP_TABLE, 0x6b, &|e, _| {
        for &(d, base, tam) in &[(0x5dusize, G_NATION_TABLE, 0x122usize), (0x59, G_CONTINENT_TABLE, 0xc6), (0x61, G_COLOUR_TABLE, 0x3a), (0x65, G_COLOUR_TABLE, 0x3a)] {
            a(e + d, base, tam);
        }
    });
}

/// with rebuild, fills the vector [0xae2d1c] with the club ids; then sorts it by reputation (qsort with 0x540020)
/// Original: `database_sort_clubs_by_reputation` (0x53db40, Database.cpp, cdecl).
///
/// [pt] { 0x53db40 } (cdecl, Database.cpp): `database_sort_clubs_by_reputation(refazer)`, na criação, no load_game_data
/// 0x82a9c0 (vetor ausente do save) e no laço diário 0x5cccd0. Com `refazer` (byte) ≠ 0, o vetor [0xae2d1c] recebe os
/// ids (+0) dos [0xae235c] clubes na ordem da tabela; depois qsort (0x945333) dos [0xae235c] ids com 0x540020 (pela
/// reputação).
pub(crate) unsafe extern "C" fn database_sort_clubs_by_reputation(refazer: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    if refazer as u8 != 0 {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            *((rd(G_CLUB_REPUTATION_ORDER) + i as usize * 4) as *mut i32) = gi(rd(G_CLUB_TABLE) + i as usize * 581);
            i += 1;
        }
    }
    ordenar(rd(G_CLUB_REPUTATION_ORDER), gi(0xae235c) as u32, 4, 0x540020); // todos os clubes pela reputação
}




/// Frees a database table with GlobalFree when the database uses the shared block (G_DATABASE_SHARED_ALLOC_MODE), else
/// with CRT free.
///
/// [pt] Solta uma tabela do banco como 0x527ee0/0x527f20: `GlobalFree` (IAT 0x9670f4) com o banco no bloco
/// compartilhado ([0xae2340] ≠ 0), senão `free` 0x9452ca.
pub(crate) unsafe fn free_database_table(tabela: usize) {
    if *(G_DATABASE_SHARED_ALLOC_MODE as *const u32) != 0 {
        let global_free: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(*(0x9670f4 as *const usize));
        global_free(tabela);
    } else {
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        free(tabela);
    }
}

/// with the staff history table present, frees it (GlobalFree in the shared database, free otherwise) and zeroes
/// pointer and count
/// Original: `database_free_staff_history_table` (0x527ee0, Database.cpp, cdecl).
///
/// [pt] { 0x527ee0 } (cdecl, Database.cpp): `database_free_staff_history_table()`: com a tabela do histórico de
/// pessoas ([0xae23cc], 17 bytes) presente, solta (0x9670f4 ou 0x9452ca, pelo modo do banco) e zera o ponteiro
/// e a contagem [0xae2374]. TapeCall por PlayerStats::PlayerStats (0x7c0ccd): o histórico passa a viver no
/// `player stats history.tmp`. Sem quadro de pilha.
pub(crate) unsafe extern "C" fn database_free_staff_history_table() {
    let tabela = *(G_STAFF_HISTORY_TABLE as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparacao::ramo("banco liberar histórico de pessoas");
    free_database_table(tabela);
    *(G_STAFF_HISTORY_TABLE as *mut usize) = 0;
    *(G_STAFF_HISTORY_COUNT as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// simplification: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.

/// the same for the award (staff competition) history
/// Original: `database_free_staff_comp_history_table` (0x527f20, Database.cpp, cdecl).
///
/// [pt] { 0x527f20 } (cdecl, Database.cpp): `database_free_staff_comp_history_table()`: o mesmo para o histórico
/// de prêmios ([0xae23e4], 58 bytes; contagem [0xae238c]). TapeCall por AWARD_MANAGER::AWARD_MANAGER (0x413360).
pub(crate) unsafe extern "C" fn database_free_staff_comp_history_table() {
    let tabela = *(G_STAFF_COMP_HISTORY_TABLE as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparacao::ramo("banco liberar histórico de prêmios");
    free_database_table(tabela);
    *(G_STAFF_COMP_HISTORY_TABLE as *mut usize) = 0;
    *(G_STAFF_COMP_HISTORY_COUNT as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// simplification: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.

/// reallocs the competition history to n more rows (out of memory: 0); count, national part pointer and the new rows
/// with their own index at +0 and the rest zero; 1
/// Original: `database_grow_comp_history_table` (0x527f60, Database.cpp, cdecl).
///
/// [pt] { 0x527f60 } (cdecl, Database.cpp): `database_grow_comp_history_table(n)`: `realloc` 0x945667 do histórico de
/// competições ([0xae23d4], 26 bytes) para [0xae237c] + `n` linhas; sem memória devolve 0 e nada muda. Senão soma
/// `n` à contagem, guarda o bloco, refaz o ponteiro da parte das seleções [0xae23dc] (a partir da linha [0xae2384])
/// e limpa as `n` linhas novas (+0 = o índice da linha, o resto zero); devolve 1. TapeCall por
/// comp_util_add_history_record (0x4afd2c), no fim de competição. Sem quadro de pilha.
pub(crate) unsafe extern "C" fn database_grow_comp_history_table(n: i32) -> u32 {
    crate::comparacao::ramo("banco crescer histórico de competições");
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let total = (*(G_CLUB_COMP_HISTORY_COUNT as *const i32)).wrapping_add(n);
    let nova = realloc(*(G_CLUB_COMP_HISTORY_TABLE as *const usize), (total as u32).wrapping_mul(26));
    if nova == 0 {
        return 0;
    }
    *(G_CLUB_COMP_HISTORY_COUNT as *mut i32) = total;
    *(G_CLUB_COMP_HISTORY_TABLE as *mut usize) = nova;
    let selecoes = total.wrapping_sub(*(G_NATION_COMP_HISTORY_COUNT as *const i32));
    *(G_NATION_COMP_HISTORY_TABLE as *mut usize) = nova.wrapping_add((selecoes as usize).wrapping_mul(26));
    for k in 1..=n {
        let i = total.wrapping_sub(k);
        let linha = nova.wrapping_add((i as usize).wrapping_mul(26)) as *mut u8;
        core::ptr::write_bytes(linha, 0, 26);
        core::ptr::write_unaligned(linha as *mut i32, i); // a linha nova guarda o próprio índice
    }
    1
}

/// Comparador: o `realloc` não se desfaz (crescer duas vezes deixaria a tabela com 2n linhas a mais).
// simplification: só a original; a prova é o .qst (a tabela vai para o save) e o ramo da cópia no modo Rust.


/// qsort comparison of club ids by reputation (+0x80), highest first; negative id = 0
/// Original: `qsort comparator (club ids by reputation)` (0x540020, Database.cpp, cdecl).
///
/// [pt] { 0x540020 } (cdecl, Database.cpp): comparação do qsort do vetor de ids de clube (0x53db40): pela reputação
/// (+0x80, palavra com sinal) do clube ([0xae23b4] + id × 581; id −1 ou menor = reputação 0), a maior primeiro:
/// a de `a` menor que a de `b`: 1; maior: −1; iguais: 0.
pub(crate) unsafe extern "C" fn qsort_comparator_club_ids_by_reputation(a: usize, b: usize) -> i32 {
    let clubes = *(G_CLUB_TABLE as *const usize);
    let injury_word_0c = |p: usize| {
        let id = *(p as *const i32);
        if id > -1 { core::ptr::read_unaligned(clubes.wrapping_add((id as usize).wrapping_mul(581)).wrapping_add(0x80) as *const i16) } else { 0 }
    };
    let (ra, rb) = (injury_word_0c(a), injury_word_0c(b));
    if ra < rb {
        1 // a de menor reputação depois
    } else if ra > rb {
        -1
    } else {
        0
    }
}


/// qsort comparison of squads: null staff last, then the player word +0xb, highest first
/// Original: `qsort comparator (squad by player +0xb)` (0x5402f0, Database.cpp, cdecl).
///
/// [pt] { 0x5402f0 } (cdecl, Database.cpp): comparação do qsort dos elencos (corte do conjunto A+B em 0x541e60 e as
/// seleções da MLS em 0x90a980): ponteiros de pessoa; nula vai para o fim (a nula e b não: 1; a não e b nula: −1;
/// as duas: 0); depois pela palavra +0xb do jogador (pessoa +0x61; sem jogador, 0), a maior primeiro (b − a).
pub(crate) unsafe extern "C" fn qsort_comparator_squad_by_player_0xb(a: usize, b: usize) -> i32 {
    let pa = *(a as *const usize);
    let pb = *(b as *const usize);
    if pa == 0 {
        return (pb != 0) as i32;
    }
    if pb == 0 {
        return -1;
    }
    let habilidade = |p: usize| {
        let j = core::ptr::read_unaligned((p + 0x61) as *const usize);
        if j != 0 { core::ptr::read_unaligned((j + 0xb) as *const i16) } else { 0 }
    };
    habilidade(pb) as i32 - habilidade(pa) as i32 // a maior primeiro
}


/// qsort comparison of a squad: no player last, then current ability (+5) and, when tied, the word +0xb, highest first
/// Original: `compare_staff_by_player_ability_desc` (0x549c30, Database.cpp, cdecl).
///
/// [pt] { 0x549c30 } (cdecl, Database.cpp): comparação do qsort do elenco na força do time (club_squad_strength_band
/// 0x549a30): ponteiros de pessoa; sem pessoa ou sem jogador (+0x61) em `a`: 1 (mesmo com `b` igual); em `b`:
/// −1; depois pela habilidade atual (jogador +5, palavra com sinal) e, empatada, pela palavra +0xb, a maior
/// primeiro (b − a).
pub(crate) unsafe extern "C" fn compare_staff_by_player_ability_desc(a: usize, b: usize) -> i32 {
    let jogador = |p: usize| {
        let s = *(p as *const usize);
        if s == 0 { 0 } else { core::ptr::read_unaligned((s + 0x61) as *const usize) }
    };
    let ja = jogador(a);
    if ja == 0 {
        return 1;
    }
    let jb = jogador(b);
    if jb == 0 {
        return -1;
    }
    let palavra = |j: usize, d: usize| core::ptr::read_unaligned((j + d) as *const i16) as i32;
    if palavra(ja, 5) != palavra(jb, 5) {
        palavra(jb, 5) - palavra(ja, 5) // a habilidade atual maior primeiro
    } else {
        palavra(jb, 0xb) - palavra(ja, 0xb)
    }
}


/// Calls TRAINING_MANAGER remove_or_restore_training_progress on G_TRAINING_MANAGER for the person (devolver 0 removes,
/// 1 restores); nothing without a manager.
///
/// [pt] TRAINING_MANAGER::remove_or_restore_training_progress (0x8b3640, thiscall com o gerente [0xde1f44]): tira
/// (`devolver` = 0) ou devolve (1) o progresso do treino dos atributos da `pessoa`. Sem gerente, nada.
pub(crate) unsafe fn remove_or_restore_training(pessoa: usize, devolver: u32) {
    let gerente = *(G_TRAINING_MANAGER as *const usize);
    if gerente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRAINING_MANAGER_REMOVE_OR_RESTORE_TRAINING_PROGRESS);
        f(gerente, pessoa, devolver);
    }
}

/// Subtracts the mean of the player's signed bytes +0x2a and +0x3a from both (common tail of 0x538bc0/0x538ec0).
///
/// [pt] O fim comum de 0x538bc0 e 0x538ec0 com o jogador +0xf alto: tira de +0x2a e de +0x3a a média dos dois
/// (bytes com sinal, divisão truncada).
pub(crate) unsafe fn subtract_mean_2a_3a(jogador: usize) {
    let (a, b) = ((jogador + 0x2a) as *mut u8, (jogador + 0x3a) as *mut u8);
    let media = ((*a as i8 as i32 + *b as i8 as i32) / 2) as u8;
    *a = (*a).wrapping_sub(media);
    *b = (*b).wrapping_sub(media);
}

/// Shared rebalancing of 0x538bc0/0x538ec0 over the player's attribute vector cells [inicio, fim) minus `pular`: marks
/// cells within a growing limit, shifts them toward `alvo`, handles young players' excess and caps the total.
///
/// [pt] O reequilíbrio comum de 0x538bc0 e 0x538ec0 sobre o vetor de atributos do jogador (0x53a610: palavra com
/// sinal + peso por célula), nas células de `inicio` (fim dos lados + 2) até antes de `fim` (fim dos
/// ponderados), sem as de `pular`; duas passadas: (1) limite 30, +5 por volta, até 12 células com |valor| ≤
/// limite (marcadas); a de maior valor fora do limite também é marcada; (2) soma `alvo` − média às marcadas
/// (presas em −125..125); na 1ª passada, com o jogador +0xf < 15, as não marcadas positivas entram no
/// excesso (as células 0x11 e 0x12 por `excesso`), as demais são marcadas; com a célula 0 < 15, as de peso 1
/// viram −120; (3) com mais de uma no excesso e soma > 85, tira (soma − 85) / quantas das não marcadas
/// (mínimo 0). O quadro da original não é lido sem valor: as células do intervalo vêm todas de 0x53a610.
pub(crate) unsafe fn rebalance_attr_vector(v: *mut u8, inicio: i8, fim: i8, jogador: usize, alvo: i32, pular: &[i8],
                                        excesso: impl Fn(i16) -> i32) {
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let peso = |i: i8| *v.add(3 * i as usize + 2);
    let fora = |c: i8| pular.contains(&c);
    let jovem = *((jogador + 0xf) as *const i8) < 0xf;
    let mut marca = [0u32; 64];
    for step_towards in 0..2 {
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limit = 0x1ei32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if inicio < fim {
                let mut c = inicio;
                while c < fim {
                    if !fora(c) {
                        let x = w(c);
                        if (x as i32).abs() <= limit as i16 as i32 {
                            total = total.wrapping_add(x as i32);
                            n = n.wrapping_add(1);
                            marca[c as usize] = 1;
                        } else {
                            marca[c as usize] = 0;
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
            // simplification: com inicio >= fim (ou menos de 12 células) o original roda para sempre; a cópia também
            limit += 5;
        }
        if melhor > -1 {
            marca[melhor as usize] = 1;
        }
        let delta = alvo.wrapping_sub(total / n as i32) as i16;
        let mut c = inicio;
        while c < fim {
            if !fora(c) && c != melhor {
                if marca[c as usize] == 1 {
                    pw(c, w(c).wrapping_add(delta).clamp(-0x7d, 0x7d));
                } else if step_towards == 0 && w(c) > 0 && jovem {
                    quantas += 1;
                    soma = soma.wrapping_add(if c == 0x11 || c == 0x12 { excesso(w(c)) } else { w(c) as i32 });
                } else {
                    marca[c as usize] = 1;
                }
                if w(0) < 0xf && peso(c) == 1 {
                    pw(c, -0x78);
                }
            }
            c += 1;
        }
        if quantas > 1 && soma > 0x55 {
            let tirar = ((soma - 0x55) / quantas) as i16;
            let mut c = inicio;
            while c < fim {
                if !fora(c) && marca[c as usize] != 1 {
                    let x = w(c).wrapping_sub(tirar);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
            }
        }
    }
}

/// Fills the player's attribute vector (0x53a610) and returns the group ends (sides end, weighted end).
///
/// [pt] O vetor de atributos do `jogador` (0x53a610) com os fins de grupo (lados, posições, ponderados).
pub(crate) unsafe fn player_attr_vector(jogador: usize, pessoa: usize, v: *mut u8) -> (u8, u8) {
    let preencher: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize = core::mem::transmute(FN_DATABASE_PLAYER_TO_ATTRIBUTE_VECTOR);
    let (mut lados, mut posicoes, mut ponderados) = (0u8, 0u8, 0u8);
    preencher(jogador, pessoa, v as usize, &mut lados, &mut posicoes, &mut ponderados);
    (lados, ponderados)
}

/// lowers the physical attributes above the current ability (cut by the weighted sum with the .rdata constants) and
/// rebalances the attribute vector without cells 0x18 and 0x1e, target −40 to 0; with +0xf = 20 removes pair and trio
/// averages; removes and restores the training progress
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
pub(crate) unsafe extern "C" fn staff_normalise_player_physical_attributes(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize);
    if jogador == 0 {
        return;
    }
    remove_or_restore_training(pessoa, 0);
    let b = |d: usize| *((jogador + d) as *const i8) as i32;
    let f = |x: i32, k: usize| match_pl_ftol(x as f64 * read_f64_at(k));
    let menor = b(0x3b).min(b(0x30));
    let t = f(b(0x36), 0x968940) - f(b(0x2f), 0x969548) - f(b(0x2e), 0x969540) - f(b(0x1b), 0x969538)
        + (f(b(0x3f), 0x9689b0) - f(b(0x40), 0x969548) - f(menor, 0x969530)) + b(0x44);
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16);
    let corte = (habilidade as i32 / 2 - f(t, 0x969528) + 0x37) / 2;
    if corte < -14 {
        let d = (corte / 15).max(-2);
        for off in [0x1b, 0x36, 0x40, 0x44] {
            let x = d + b(off);
            *((jogador + off) as *mut u8) = if x < 1 { 1 } else { x as u8 };
        }
    }
    let mut s = corte - 0x14;
    let produto = ((b(0x36) * b(0x1b)) / 2) as i16;
    if produto > habilidade {
        s -= (produto as i32 - habilidade as i32).min(0x19);
    }
    let alvo = if s < -40 { -40 } else if s > 0 { 0 } else { s }; // alvo físico
    let mut vetor = [0u8; 63 * 3];
    for c in vetor.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = vetor.as_mut_ptr();
    let (lados, ponderados) = player_attr_vector(jogador, pessoa, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, alvo, &[0x1e, 0x18],
        |x| 2 * x as i32);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_ATTRIBUTE_VECTOR_TO_PLAYER);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) == 0x14 {
        subtract_mean_2a_3a(jogador);
        let terco = ((b(0x1e) + b(0x24) + b(0x39)) / 3) as u8;
        for off in [0x1e, 0x39, 0x24] {
            *((jogador + off) as *mut u8) = (*((jogador + off) as *const u8)).wrapping_sub(terco);
        }
    }
    remove_or_restore_training(pessoa, 1);
}



/// a second name: with the nation having more than 10, the heaviest of 3 draws in its range; otherwise a draw over the
/// whole table
/// Original: `database_pick_second_name_by_weight` (0x53a0e0, Database.cpp, cdecl).
///
/// [pt] { 0x53a0e0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) pelo peso. Nação com
/// mais de 10 segundos nomes (`nation_name_index` +0x1c): 3 sorteios na faixa dela (a partir de +4),
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com até
/// 10: sorteio na tabela inteira ([0xae239c], no máximo 0x7fff). Só o rand() da CRT. TapeCall por
/// OfficialsManager::add_random_officials (0x79c6b4), na criação dos árbitros do jogo novo.
pub(crate) unsafe extern "C" fn database_pick_second_name_by_weight(nacao: usize) -> usize {
    let linha = |i: i32| (*(G_SECOND_NAME_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = nation_name_index(nacao);
        let quantos = *((e + 0x1c) as *const i32);
        if quantos > 0xa {
            let inicio = *((e + 4) as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = scaled_crt_rand(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido); // o segundo nome de maior peso
        }
    }
    linha(scaled_crt_rand(name_count_capped(0xae239c)))
}


/// rebalances the player attribute vector with target −20 (12 cells within the limit, excess above 85 spread); with
/// +0xf ≥ 15 removes the +0x2a/+0x3a average; removes and restores the training progress
/// Original: `staff_normalise_player_attributes` (0x538bc0, Database.cpp, cdecl).
///
/// [pt] { 0x538bc0 } (cdecl, Database.cpp): `staff_normalise_player_attributes(pessoa)`: com o jogador (pessoa
/// +0x61), tira o progresso do treino (0x8b3640), monta o vetor pelo 0x53a610 (sem pré-encher: as células
/// usadas vêm todas dele), `rebalance_attr_vector` com alvo −20 em todas as células do intervalo
/// (0x11/0x12 contam ftol(valor × [0x968f98])), volta pelo 0x53aa10; jogador +0xf ≥ 15: tira as médias de
/// +0x2a/+0x3a; devolve o progresso do treino. Roda na criação (depois de 0x538ec0) e na carga e no treino.
pub(crate) unsafe extern "C" fn staff_normalise_player_attributes(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize);
    if jogador == 0 {
        return;
    }
    remove_or_restore_training(pessoa, 0);
    let mut vetor = [0u8; 63 * 3];
    let v = vetor.as_mut_ptr();
    let (lados, ponderados) = player_attr_vector(jogador, pessoa, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, -20, &[],
        |x| match_pl_ftol(x as f64 * read_f64_at(0x968f98))); // 0x968f98 = 1.75
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_ATTRIBUTE_VECTOR_TO_PLAYER);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) >= 0xf {
        subtract_mean_2a_3a(jogador);
    }
    remove_or_restore_training(pessoa, 1);
}


/// Competição (107 bytes) no club_comp.dat / nation_comp.dat versão 1 (104 no arquivo): `(deslocamento,
/// tamanho, quantos)` na ordem do arquivo; buracos em +0x37 e +0x52; tamanho 0 = a reputação num byte, u16 em +0x69.
pub(crate) const COMPETITION_V1_LAYOUT: [(usize, u32, usize); 7] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 1), (0x57, 1, 2), (0x59, 4, 4), (0x69, 0, 1)];


/// Comparador: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à posição do `ftell`
/// 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo parou.
// simplification: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.

/// the nation's name, or its English one from eng.lng (109-byte rows, by id) when another language is loaded
/// Original: `nation_get_english_name` (0x5453f0, Database.cpp, cdecl).
///
/// [pt] { 0x5453f0 } (cdecl, Database.cpp): `nation_get_english_name(nação)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de +0x2c, pelo id
/// (sem conferir a quantidade).
pub(crate) unsafe extern "C" fn nation_get_english_name(nacao: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(4); // idioma 0 ou 1 (nação)
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_NATION_GET_ENGLISH_NAME)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109))
}


/// the club's short name, or its English one from eng.lng (79-byte rows, through the id map or by id) when another
/// language is loaded
/// Original: `club_get_english_short_name` (0x545770, Database.cpp, cdecl).
///
/// [pt] { 0x545770 } (cdecl, Database.cpp): `club_get_english_short_name(clube)`: idioma ([0x9b5984]) 0 ou 1: o
/// próprio nome curto (+0x38). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20, o
/// nome curto em +0x34 da linha): pelo mapa de ids [0xae23fc] se houver, senão pelo id se < quantos (+0),
/// senão 0.
pub(crate) unsafe extern "C" fn club_get_english_short_name(clube: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(0x38);
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: o primeiro uso aloca e carrega o eng.lng sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_SHORT_NAME)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(G_CLUB_ENGLISH_NAME_INDEX_TABLE as *const usize);
    let i = if mapa != 0 {
        *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32)
    } else if id < *(t as *const i32) {
        id
    } else {
        return 0;
    };
    linhas.wrapping_add((i as usize).wrapping_mul(79)).wrapping_add(0x34) // nome curto na linha
}


/// Autoteste (CM_AUTOTESTE=1, modo comparação): nem a carreira nem o save passam por estas três (a base é
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
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x5631_4442_4a32);
    let v1: usize = COMPETITION_V1_LAYOUT.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum();
    for caso in 0..64u64 {
        let n = 1 + (s.proximo() % 6) as usize;
        let total = n * v1;
        let corte = if caso % 3 == 0 { total } else { (s.proximo() % (total as u64 + 1)) as usize };
        let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.proximo() as u8 }).collect();
        if std::fs::write(ARQ, &bytes).is_err() {
            continue;
        }
        let f = fopen(b"autoteste_banco_v1c.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
        if f == 0 {
            continue;
        }
        let mut buf: Vec<u8> = (0..n * 107).map(|_| s.proximo() as u8).collect();
        comparar_db_files_read_comps_v1(f, 0, buf.as_mut_ptr() as usize, n as u32);
        fclose(f);
    }
    let _ = std::fs::remove_file(ARQ);
    let globais = [0x9b5984usize, G_ENGLISH_NAME_TABLES, G_CLUB_ENGLISH_NAME_INDEX_TABLE];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 quantos clubes, +0x20 clubes, +0x2c nações
    let mut mapa = [0i32; 8];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
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
        comparar_nation_get_english_name(registro.as_ptr() as usize);
        comparar_club_get_english_short_name(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco v1 e nomes em inglês: AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

/// flags every club that has a partner team (+0x244) and, for every senior team, trims the joint senior+B squad to 48
/// by reputation, from the bottom, keeping 3 goalkeepers
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
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let goleiro = |q: usize| {
        let j = rd(q + 0x61);
        j != 0 && *((j + 0xf) as *const i8) >= 0xf
    };
    let mut clube = *(G_CLUB_TABLE as *const usize);
    let mut i = 0i32;
    while i < *(G_CLUB_COUNT as *const i32) {
        let mut eh_a = 0u32;
        let p = parceiro(clube, &mut eh_a, 0);
        if p == 0 || rd(p + 0x53) == 0 {
            *((clube + club::HAS_LINKED_CLUB) as *mut u8) = 0;
        } else {
            *((clube + club::HAS_LINKED_CLUB) as *mut u8) = 1;
            if eh_a != 0 {
                let mut v = [0usize; 100];
                reunir(clube, v.as_mut_ptr(), 1);
                ordenar(v.as_mut_ptr() as usize, 100, 4, 0x5402f0);
                let (mut total, mut goleiros) = (0i8, 0i8);
                for &q in v.iter() {
                    if q != 0 {
                        total += 1;
                        if goleiro(q) {
                            goleiros += 1;
                        }
                    }
                }
                let mut k = 99i8;
                while k >= 0 && total > 48 { // o elenco conjunto a 48
                    let q = v[k as usize];
                    if q != 0 {
                        let g = goleiro(q);
                        if !(g && goleiros < 3) {
                            if g {
                                goleiros -= 1;
                            }
                            tirar(rd(q + 0x39), q, 0, 1, 0);
                            total -= 1;
                        }
                    }
                    k -= 1;
                }
            }
        }
        i += 1;
        clube += 0x245;
    }
}

/// for every senior team with a B team: trims the B squad to what fits in 50 with the senior one and moves the B
/// backroom staff (coaches, scouts, physios and the manager when the senior team has none) to the senior club
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
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let valida = |p: usize| p != 0 && *(p as *const i32) < *(G_STAFF_COUNT as *const i32);
    let contar = |c: usize, d: usize, k: usize| (0..k).filter(|&j| valida(rd(c + d + 4 * j))).count() as i32;
    let mut clube = *(G_CLUB_TABLE as *const usize);
    let mut i = 0i32;
    while i < *(G_CLUB_COUNT as *const i32) {
        let mut eh_a = 0u32;
        let b = parceiro(clube, &mut eh_a, 1);
        if b != 0 && eh_a != 0 {
            let mut n = contar(clube, 0xd7, 50);
            ordenar(b + 0xd7, 50, 4, 0x540130);
            for j in 0..50 {
                let p = rd(b + 0xd7 + 4 * j);
                if valida(p) {
                    n += 1;
                    if n > 50 {
                        tirar(b, p, 0, 1, 0);
                    }
                }
            }
            for &(d, k, cargo) in &[(0x19fusize, 5usize, 8u32), (0x1b3, 7, 9), (0x1cf, 3, 10)] {
                let mut n = contar(clube, d, k);
                for j in 0..k {
                    let p = rd(b + d + 4 * j);
                    if valida(p) {
                        tirar(b, p, 0, 1, 0);
                        if n < k as i32 { // a comissão do B vai para o A
                            n += 1;
                            incluir(clube, p, cargo, G_CURRENT_DATE, 1, 0);
                        }
                    }
                }
            }
            let tb = rd(b + 0xcf);
            if rd(clube + club::MANAGER) != 0 && tb != 0 {
                tirar(b, tb, 0, 1, 0);
            } else if tb != 0 {
                tirar(b, tb, 0, 1, 0);
                incluir(clube, tb, *((tb + 0x3d) as *const u8) as u32, 0, 1, 0);
            }
            if rd(clube + club::ASSISTANT_MANAGER) != 0 && rd(b + 0xd3) != 0 {
                tirar(b, rd(b + 0xd3), 0, 1, 0);
            }
        }
        i += 1;
        clube += 0x245;
    }
}

/// allocates and builds the per-nation index of the first, second and common name tables (first row, weight total and
/// count of every group); the last group of each table is left unwritten, as in the original
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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let nacoes = *(G_NATION_COUNT as *const i32);
    let t = alocar((nacoes as u32).wrapping_mul(0x24));
    *(G_NATION_NAME_INDEX_TABLE as *mut usize) = t;
    if t == 0 {
        // simplification: a original pede de novo e, sem memória, abre a caixa e encerra o jogo
        return original_fn::<unsafe extern "C" fn()>(IDX_DATABASE_BUILD_NATION_NAME_INDEX)();
    }
    fill_nation_name_index(t);
}

/// Body of 0x539750: resets the per-nation name index table and, for the first/second/common name tables, records each
/// nation's first row, weight sum and count.
///
/// [pt] O corpo da 0x539750 sobre a tabela `t`, já em [0xae2338]. O comparador passa um buffer do Rust: montar a cópia
/// com o malloc do jogo deslocava a heap e mudava a impressão das partidas no jogo real comparando (232a, sonda).
pub(crate) unsafe fn fill_nation_name_index(t: usize) {
    let mut k = 0i32;
    while k < *(G_NATION_COUNT as *const i32) {
        let e = t + k as usize * 0x24;
        for &(d, v) in &[(0usize, -1i32), (4, -1), (8, -1), (0xc, 0), (0x10, 0), (0x14, 0), (0x18, 0), (0x1c, 0), (0x20, 0)] {
            *((e + d) as *mut i32) = v;
        }
        k += 1;
    }
    // (quantidade, tabela, +primeira linha, +soma dos pesos, +quantidade), na ordem da original
    for &(qtd, tabela, d_inicio, d_soma, d_quantos) in &[
        (0xae239cusize, G_SECOND_NAME_TABLE, 4usize, 0x10usize, 0x1cusize),
        (0xae2398, G_FIRST_NAME_TABLE, 0, 0xc, 0x18),
        (0xae23a0, G_COMMON_NAME_TABLE, 8, 0x14, 0x20),
    ] {
        let (mut atual, mut inicio, mut soma, mut quantos) = (0usize, -1i32, 0i32, 0i32);
        let mut l = 0i32;
        while l < *(qtd as *const i32) {
            let linha = *(tabela as *const usize) + l as usize * 0x3c;
            let nacao = core::ptr::read_unaligned((linha + 0x37) as *const i32);
            if nacao > -1 && nacao < *(G_NATION_COUNT as *const i32) {
                let registro = (*(G_NATION_TABLE as *const usize)).wrapping_add((nacao as usize).wrapping_mul(nation::SIZE));
                let peso = *((linha + 0x3b) as *const i8) as i32;
                if atual == registro {
                    soma = soma.wrapping_add(peso);
                    quantos += 1;
                } else {
                    if atual != 0 {
                        let id = *(atual as *const i32);
                        let e = (*(G_NATION_NAME_INDEX_TABLE as *const usize)).wrapping_add(id.wrapping_mul(0x24) as usize);
                        *((e + d_soma) as *mut i32) = soma;
                        *((e + d_inicio) as *mut i32) = inicio;
                        *((e + d_quantos) as *mut i32) = quantos; // quantidade do grupo
                    }
                    inicio = l;
                    quantos = 1;
                    atual = registro;
                    soma = peso;
                }
            }
            l += 1;
        }
    }
}

/// Regions compared for 0x541e60/0x544970: the whole club table plus the 110-byte staff records in the staff slots of
/// every A/B club pair.
///
/// [pt] As regiões dos comparadores de 0x541e60 e 0x544970: a tabela de clubes inteira (581 bytes por linha: a marca
/// +0x244 de todos e as casas dos pares) e as pessoas (110 bytes) nas casas dos clubes A e B dos pares de 0x540a50,
/// as únicas que as duas tiram, põem ou desligam (0x53c190 e 0x53bd10 só mexem no clube e na pessoa).
pub(crate) unsafe fn ab_team_regions() -> Vec<(usize, usize)> {
    let tabela = *(G_CLUB_TABLE as *const usize);
    let n = *(G_CLUB_COUNT as *const i32);
    let mut regioes = vec![(tabela, n.max(0) as usize * 581)];
    let mut pessoas = Vec::new();
    for &(a, b) in A_B_CLUB_PAIRS.iter() {
        for g in [a, b] {
            let linha = *(g as *const i32);
            if linha < 0 || linha >= n {
                continue;
            }
            let c = tabela + linha as usize * 581;
            for &(d, k) in &[(0xbfusize, 1usize), (0xc3, 3), (0xcf, 1), (0xd3, 1), (0xd7, 50), (0x19f, 5), (0x1b3, 7), (0x1cf, 3)] {
                for j in 0..k {
                    let p = core::ptr::read_unaligned((c + d + 4 * j) as *const usize);
                    if p != 0 {
                        pessoas.push(p);
                    }
                }
            }
        }
    }
    pessoas.sort_unstable();
    pessoas.dedup();
    regioes.extend(pessoas.into_iter().map(|p| (p, 110)));
    regioes
}





/// removes the person from the club slot her job points to (writing the date into her, when given, and, for the senior
/// national team manager, from the under-21 side too) and, if asked, clears the club in the person; 0 if she was not in
/// the slot
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
pub(crate) unsafe extern "C" fn database_remove_staff_from_club(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    if clube == 0 {
        return 0;
    }
    if pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(IDX_DATABASE_REMOVE_STAFF_FROM_CLUB)(clube, pessoa, data, desligar, como_jogador);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if data != 0 {
        let d = if selecao() { pessoa + staff::DATE_JOINED_NATION } else { pessoa + staff::DATE_JOINED_CLUB };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut cargo = if selecao() { *((pessoa + staff::JOB_FOR_NATION) as *const u8) } else { *((pessoa + staff::JOB_FOR_CLUB) as *const u8) };
    if como_jogador != 0 {
        cargo = if matches!(cargo, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    let rd = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let zerar = |d: usize| core::ptr::write_unaligned((clube + d) as *mut usize, 0);
    // a primeira das `k` casas a partir de +d que tem a pessoa, zerada; false se nenhuma tem
    let tirar_da_casa = |d: usize, k: usize| match (0..k).find(|&j| rd(d + 4 * j) == pessoa) {
        Some(j) => {
            zerar(d + 4 * j);
            true
        }
        None => false,
    };
    let tirou = match cargo {
        0 => true,
        1 => tirar_da_casa(0xbf, 1),
        2..=4 => tirar_da_casa(0xc3, 3),
        5 => {
            let ok = tirar_da_casa(0xcf, 1);
            let id = gi(clube);
            let n = gi(0xae235c);
            if ok && id >= n.wrapping_sub(gi(0xae2350).wrapping_mul(2)) && id < n.wrapping_sub(gi(0xae2350)) {
                let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
                let s = sub21(rd(0x53));
                if core::ptr::read_unaligned((s + 0xcf) as *const usize) == pessoa {
                    core::ptr::write_unaligned((s + 0xcf) as *mut usize, 0); // o treinador da sub-21
                }
            }
            ok
        }
        6 | 7 => tirar_da_casa(0xd3, 1),
        8 => tirar_da_casa(0x19f, 5),
        9 => tirar_da_casa(0x1b3, 7),
        10 => tirar_da_casa(0x1cf, 3),
        11 => tirar_da_casa(0xd7, 50),
        12 => tirar_da_casa(0xcf, 1) && tirar_da_casa(0xd7, 50),
        13 => tirar_da_casa(0xd3, 1) && tirar_da_casa(0xd7, 50),
        15 => tirar_da_casa(0x19f, 5) && tirar_da_casa(0xd7, 50),
        _ => false,
    };
    if !tirou {
        return 0;
    }
    if desligar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + staff::NATIONAL_JOB) as *mut usize, 0);
        } else {
            core::ptr::write_unaligned((pessoa + staff::CLUB_JOB) as *mut usize, 0); // desliga a pessoa do clube
        }
    }
    1
}

/// copies the 7 club scouts into an array; with a partner team (senior/B), merges both up to 7 and clears the rest
/// Original: `database_get_club_scouts` (0x53cd60, Database.cpp, cdecl).
///
/// [pt] { 0x53cd60 } (cdecl, Database.cpp): `database_get_club_scouts(clube, vetor)`: os 7 olheiros (+0x1b3) do `clube` no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro): as 7
/// casas como estão. Com parceiro: os não nulos do clube e depois os do parceiro até 7; o resto zerado só quando os
/// dois juntos não enchem as 7. Contador de 8 bits na original (no máximo 7). Devolve 1. Clube nulo: a original
/// (caixa "Error" e 0). Na carreira, pela 0x7f7270 (as listas dos olheiros, nas transferências e contratações).
pub(crate) unsafe extern "C" fn database_get_club_scouts(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize) -> u32>(IDX_DATABASE_GET_CLUB_SCOUTS)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for j in 0..7 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + club::SCOUTS + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..7 {
        let o = rd(clube + club::SCOUTS + 4 * j);
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for j in 0..7 {
        if n >= 7 {
            return 1;
        }
        let o = rd(p + 0x1b3 + 4 * j); // os olheiros do parceiro
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for k in n..7 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}



/// As formas do adjetivo de 0x545a60 (tabelas de salto 0x545b2c/0x545b34, 0x545b58/0x545b60, 0x545b84,
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
pub(crate) unsafe extern "C" fn get_adjective_form_index(indice: u32, idioma: u32) -> u8 {
    let linha = match idioma {
        3 | 6 | 7 | 8 | 10 => &ADJECTIVE_FORMS_COMMON,
        4 => &ADJECTIVE_FORMS_LANGUAGE_4,
        5 => &ADJECTIVE_FORMS_LANG5,
        9 => &ADJECTIVE_FORMS_9,
        _ => return 0xff,
    };
    let i = (indice as u8 as i8 as i32).wrapping_sub(1) as u32; // o ja da original compara sem sinal
    if i > 0x22 {
        return 0xff;
    }
    linha[i as usize]
}

/// the person's coaching attribute 1..12: 0x543fd0 with their non-player record (+0x69)
/// Original: `staff_get_non_player_attribute` (0x543f30, Database.cpp, cdecl).
///
/// [pt] { 0x543f30 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`: 0x543fd0 com o
/// não jogador dela (+0x69) e o `atributo` inteiro como veio. Pessoa nula ou atributo (byte com sinal)
/// fora de 1..12: a original (caixa "Error" e −1).
pub(crate) unsafe extern "C" fn staff_get_non_player_attribute(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_NON_PLAYER_ATTRIBUTE)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_COMPUTE_NON_PLAYER_ATTRIBUTE);
    let nao_jogador = core::ptr::read_unaligned((pessoa + staff::NON_PLAYER) as *const usize);
    calcular(pessoa, nao_jogador, atributo)
}

/// the 1-to-20 rating of 2 × value + current ability × 0.1 (0x544380 with the ability scaled down)
/// Original: `player_attribute_rating_with_scaled_ability` (0x5442f0, Database.cpp, cdecl).
///
/// [pt] { 0x5442f0 } (cdecl, Database.cpp): a nota de 1 a 20 (`attribute_rating`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador` × 0,1 ([0x967880]); a de 0x544380 com a habilidade
/// reduzida (os goleiros de 0x543540). Termina no `_ftol`.
pub(crate) unsafe extern "C" fn player_attribute_rating_with_scaled_ability(jogador: usize, valor: u32) -> u32 {
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as f64;
    let v = (valor as u8 as i8) as f64;
    attribute_rating(habilidade * read_f64_at(0x967880) + (v + v)) as u32 // 0x967880 = 0.1
}




/// a first name: nation with more than 15 names, the heaviest of 3 draws in its range; otherwise a draw over the whole
/// table
/// Original: `database_pick_first_name_by_weight` (0x539b90, Database.cpp, cdecl).
///
/// [pt] { 0x539b90 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) pelo peso.
/// Nação com mais de 15 primeiros nomes (`nation_name_index` +0x18): 3 sorteios na faixa dela,
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com
/// até 15: sorteio na tabela inteira ([0xae2398], no máximo 0x7fff). Só o rand() da CRT.
pub(crate) unsafe extern "C" fn database_pick_first_name_by_weight(nacao: usize) -> usize {
    let linha = |i: i32| (*(G_FIRST_NAME_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = nation_name_index(nacao);
        let quantos = *((e + 0x18) as *const i32);
        if quantos > 0xf {
            let inicio = *(e as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = scaled_crt_rand(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido);
        }
    }
    linha(scaled_crt_rand(name_count_capped(0xae2398)))
}

/// the competition's colours (foreground and background) in pixels, from the +0x5d record or the competition itself
/// Original: `database_get_comp_colours` (0x53b7c0, Database.cpp, cdecl).
///
/// [pt] { 0x53b7c0 } (cdecl, Database.cpp): as cores da `competicao` em pixels (0x5e4800 com os bytes +0x37,
/// +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que começam com [0xae3184] e [0xaebe04]. As
/// cores vêm do registro +0x5d (+0x90 e +0x94 dele) ou, sem ele, da própria competição (+0x61 e +0x65);
/// cor nula deixa o padrão. Devolve 1. Competição nula: a original (padrões, caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_comp_colours(competicao: usize, frente: *mut u16, fundo: *mut u16) -> u32 {
    if competicao == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_COMP_COLOURS)(competicao, frente, fundo);
    }
    // simplification: a original empurra dwords com só o byte de baixo escrito (o resto é lixo de pilha);
    // a 0x5e4800 faz & 0xff nos três, então os bytes estendidos dão o mesmo pixel (como em 0x53b640)
    let cor: unsafe extern "C" fn(u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    core::ptr::write_unaligned(frente, *(G_GUI_COLOUR_PALE_GREY as *const u16));
    core::ptr::write_unaligned(fundo, *(G_GUI_COLOUR_DARK_BLUE as *const u16));
    let r = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let dono = r(competicao, 0x5d);
    let (a, b) = if dono != 0 { (r(dono, 0x90), r(dono, 0x94)) } else { (r(competicao, 0x61), r(competicao, 0x65)) };
    let pixel = |c: usize| {
        let byte = |d: usize| *((c + d) as *const u8) as u32;
        cor(byte(0x37), byte(0x38), byte(0x39), 0) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(frente, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(fundo, pixel(b));
    }
    1
}

/// True if any of the n club slots at clube+d is empty or already holds the person.
///
/// [pt] Há vaga para a `pessoa` nas `n` casas a partir de `clube + d`: alguma vazia ou já com ela.
pub(crate) unsafe fn has_slot_for(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    (0..n).any(|i| {
        let v = core::ptr::read_unaligned((clube + d + 4 * i) as *const usize);
        v == 0 || v == pessoa
    })
}

/// whether the club has room for the person in job 1..15 (an empty slot or one already theirs; 12, 13 and 15 also in
/// the squad)
/// Original: `database_club_has_room_for_job` (0x53b9a0, Database.cpp, cdecl).
///
/// [pt] { 0x53b9a0 } (cdecl, Database.cpp): 1 se o `clube` tem vaga para a `pessoa` no `cargo` (byte com
/// sinal, tabela de salto 0x53bcc8): 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3
/// (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 e 13: +0xcf / +0xd3 livre ou dela e vaga no elenco; 15:
/// vaga em +0x19f e no elenco. Clube ou pessoa nulos, cargo 14 ou fora de 1..15: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_club_has_room_for_job(clube: usize, cargo: u32, pessoa: usize) -> u32 {
    let c = cargo as u8 as i8;
    if clube == 0 || pessoa == 0 || !(1..=15).contains(&c) || c == 14 {
        return original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(IDX_DATABASE_CLUB_HAS_ROOM_FOR_JOB)(clube, cargo, pessoa);
    }
    let vaga = |d: usize, n: usize| has_slot_for(clube, d, n, pessoa);
    let tem = match c {
        1 => vaga(0xbf, 1),
        2..=4 => vaga(0xc3, 3),
        5 => vaga(0xcf, 1),
        6 | 7 => vaga(0xd3, 1),
        8 => vaga(0x19f, 5),
        9 => vaga(0x1b3, 7),
        10 => vaga(0x1cf, 3),
        11 => vaga(0xd7, 50),
        12 => vaga(0xcf, 1) && vaga(0xd7, 50),
        13 => vaga(0xd3, 1) && vaga(0xd7, 50),
        _ => vaga(0x19f, 5) && vaga(0xd7, 50), // 15
    };
    tem as u32
}




/// Puts the person in the first empty of the n club slots at clube+d; false if none.
///
/// [pt] Põe a `pessoa` na primeira das `n` casas vazias a partir de `clube + d`; false se não há.
pub(crate) unsafe fn fill_club_slot(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    let casa = |i: usize| (clube + d + 4 * i) as *mut usize;
    match (0..n).find(|&i| core::ptr::read_unaligned(casa(i)) == 0) {
        Some(i) => {
            core::ptr::write_unaligned(casa(i), pessoa);
            true
        }
        None => false,
    }
}

/// puts the person in the job slot at the club (jobs 1..15; a player above 85 without a human manager moves from 6/8 to
/// 13/15), copies the date and writes club and job into the person
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
pub(crate) unsafe extern "C" fn database_add_staff_to_club(clube: usize, pessoa: usize, cargo: u32, data: usize, gravar: u32, reduzir: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(IDX_DATABASE_ADD_STAFF_TO_CLUB)(clube, pessoa, cargo, data, gravar, reduzir);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if gi(pessoa) >= gi(0xae2364).wrapping_sub(16) && cargo as u8 != 5 {
        return 0;
    }
    if data != 0 {
        let d = if selecao() { pessoa + staff::DATE_JOINED_NATION } else { pessoa + staff::DATE_JOINED_CLUB };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut c = cargo as u8;
    if reduzir != 0 {
        c = if matches!(c, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    if core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) == 0 {
            let jogador = core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize);
            if core::ptr::read_unaligned((jogador + 5) as *const i16) > 0x55 {
                if c == 6 {
                    c = 0xd;
                } else if c == 8 {
                    c = 0xf;
                }
            }
        }
    }
    let ocupar = |d: usize, n: usize| fill_club_slot(clube, d, n, pessoa);
    let unica = |d: usize| {
        let p = (clube + d) as *mut usize;
        if core::ptr::read_unaligned(p) != 0 {
            return false;
        }
        core::ptr::write_unaligned(p, pessoa);
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
                c = 6; // sem vaga no elenco: fica só na casa +0xd3
            }
            true
        }
        15 => {
            if !ocupar(0x19f, 5) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 8; // sem vaga no elenco: fica só em +0x19f
            }
            true
        }
        _ => false, // 14 e fora de 0..15
    };
    if !ok {
        return 0;
    }
    if gravar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + staff::NATIONAL_JOB) as *mut usize, clube);
            *((pessoa + staff::JOB_FOR_NATION) as *mut u8) = c;
        } else {
            core::ptr::write_unaligned((pessoa + staff::CLUB_JOB) as *mut usize, clube);
            *((pessoa + staff::JOB_FOR_CLUB) as *mut u8) = c; // clube e cargo do clube
        }
    }
    1
}


/// Person pointer in squad slot i (+0xd7) of the club.
///
/// [pt] Um dos 50 lugares do elenco (+0xd7, ponteiros de pessoa) do `clube`.
pub(crate) unsafe fn squad_slot(clube: usize, i: usize) -> usize {
    core::ptr::read_unaligned((clube + club::SQUAD + 4 * i) as *const usize)
}

/// Person is non-null and its id is below the staff count.
///
/// [pt] A pessoa do elenco conta: não nula e com id abaixo de [0xae2364] (com sinal).
pub(crate) unsafe fn is_valid_squad_person(p: usize) -> bool {
    p != 0 && *(p as *const i32) < *(G_STAFF_COUNT as *const i32)
}

/// how many valid people are in the club's squad (with the A/B partner, the reserve squad flags and the loanee filter)
/// Original: `database_count_club_squad` (0x53c590, Database.cpp, cdecl).
///
/// [pt] { 0x53c590 } (cdecl, Database.cpp): quantas pessoas válidas há no elenco do `clube` (byte). Com
/// parceiro A/B (0x540a50 com 1): `modo` 2 e clube A conta só o parceiro; senão conta o clube e, fora
/// `modo` 1 com clube B, também o parceiro. Sem parceiro e com time reserva (0x542740): só as pessoas
/// com bits `modo` nas marcas de elenco (+0x5e), se `modo` não for 0. Com `filtro`, quem tem ficha
/// de contrato (0x4cf6d0) e clube (+0x39) diferente do `clube` só conta se o clube do contrato
/// (0x4cf5a0, guardado em [0xae1080]; +4 = id) também for outro. Clube nulo: a original (caixa).
pub(crate) unsafe extern "C" fn database_count_club_squad(clube: usize, modo: u32, filtro: u32) -> u8 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u8>(IDX_DATABASE_COUNT_CLUB_SQUAD)(clube, modo, filtro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let tem_reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_CLUB_HAS_RESERVE_SQUAD);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let modo = modo as u8;
    let contar = |c: usize, marcas: u8| -> u8 {
        let mut n = 0u8;
        for i in 0..50 {
            let p = squad_slot(c, i);
            if !is_valid_squad_person(p) || (marcas != 0 && *((p + 0x5e) as *const u8) & marcas == 0) {
                continue;
            }
            if filtro != 0 && ficha(*(G_CONTRACT_MANAGER as *const usize), p) != 0
                && core::ptr::read_unaligned((p + 0x39) as *const usize) != clube {
                let k = contrato(*(G_CONTRACT_MANAGER as *const usize), p);
                *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = k;
                let dono = if k == 0 {
                    0
                } else {
                    (*(G_CLUB_TABLE as *const usize)).wrapping_add((core::ptr::read_unaligned((k + 4) as *const u32) as usize).wrapping_mul(581))
                };
                if dono == clube {
                    continue;
                }
            }
            n = n.wrapping_add(1);
        }
        n
    };
    // simplification: o original passa o endereço de um local; 0x540a50 só devolve parceiro depois de gravá-lo
    let mut eh_a = 0u32;
    let p = parceiro(clube, &mut eh_a, 1);
    if p == 0 {
        if tem_reservas(clube) != 0 {
            return contar(clube, modo);
        }
        return contar(clube, 0);
    }
    if modo == 2 && eh_a != 0 {
        return contar(p, 0);
    }
    let n = contar(clube, 0);
    if modo == 1 && eh_a == 0 {
        return n;
    }
    n.wrapping_add(contar(p, 0))
}

/// copies the club's squad (and the A/B partner's, up to 50 or 100) into a vector, padding with zeros
/// Original: `database_collect_club_squad` (0x541f90, Database.cpp, cdecl).
///
/// [pt] { 0x541f90 } (cdecl, Database.cpp): copia o elenco do `clube` (50 ponteiros em +0xd7) para o
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1): os 50 como estão e, com `dobro`, mais 50 zeros.
/// Com parceiro: só as pessoas válidas do clube e depois as do parceiro até o limite (100 com
/// `dobro`, senão 50), o resto até o limite zerado. Devolve 1. Clube nulo: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_collect_club_squad(clube: usize, vetor: *mut usize, dobro: u32) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *mut usize, u32) -> u32>(IDX_DATABASE_COLLECT_CLUB_SQUAD)(clube, vetor, dobro);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let limit: i8 = if dobro != 0 { 100 } else { 50 };
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for i in 0..50 {
            core::ptr::write_unaligned(vetor.add(i), squad_slot(clube, i));
        }
        if dobro != 0 {
            for i in 50..100 {
                core::ptr::write_unaligned(vetor.add(i), 0);
            }
        }
        return 1;
    }
    let mut n: i8 = 0;
    for i in 0..50 {
        let q = squad_slot(clube, i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for i in 0..50 {
        if n >= limit {
            break;
        }
        let q = squad_slot(p, i);
        if is_valid_squad_person(q) {
            core::ptr::write_unaligned(vetor.add(n as usize), q);
            n += 1;
        }
    }
    for k in n..limit {
        core::ptr::write_unaligned(vetor.add(k as usize), 0);
    }
    1
}

/// whether the club can be managed: senior national team of a nation with +0x88 > 150, or a club that is not a B team,
/// with an active competition and a flagged nation
/// Original: `club_can_be_managed` (0x544820, Database.cpp, cdecl).
///
/// [pt] { 0x544820 } (cdecl, Database.cpp): o `clube` pode ser dirigido? Precisa de nação (+0x53). Seleções
/// (id ≥ [0xae235c] − 2 × [0xae2350], com sinal): só as principais (id < [0xae235c] − [0xae2350]) de
/// nação com +0x88 > 150. Clubes: com competição (+0x57), que não sejam o time B de um par (0x540a50
/// com 1), com a competição no vetor [[0xadadfc] + id × 4] e a nação com o bit 2 de +0x11c.
pub(crate) unsafe extern "C" fn club_can_be_managed(clube: usize) -> u8 {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let u = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    if clube == 0 || u(0x53) == 0 {
        return 0;
    }
    let id = *(clube as *const i32);
    let (n, total) = (*(G_NATION_COUNT as *const i32), *(G_CLUB_COUNT as *const i32));
    if id >= total.wrapping_sub(n.wrapping_add(n)) {
        return (id < total.wrapping_sub(n) && core::ptr::read_unaligned((u(0x53) + 0x88) as *const i32) > 0x96) as u8;
    }
    if u(0x57) == 0 {
        return 0;
    }
    // simplification: o original usa o lugar do próprio argumento como saída; só é lido com parceiro, e aí já foi gravado
    let mut eh_a = 0u32;
    if parceiro(clube, &mut eh_a, 1) != 0 && eh_a == 0 {
        return 0; // time B
    }
    let tabela = *(G_COMP_OBJECT_TABLE as *const usize);
    if tabela == 0 {
        return 0;
    }
    let comp = *(u(0x57) as *const i32);
    if *((tabela as isize).wrapping_add((comp as isize).wrapping_mul(4)) as *const u32) == 0 {
        return 0;
    }
    (*((u(0x53) + 0x11c) as *const u8) & 2 != 0) as u8
}




/// writes the low byte of the vector's 21 cells back to the non-player's coaching attributes (+0x0e..+0x22)
/// Original: `database_vector_to_non_player` (0x53a980, Database.cpp, cdecl).
///
/// [pt] { 0x53a980 } (cdecl, Database.cpp): o inverso de 0x53a8b0: o byte de baixo de cada uma das 21
/// células de 3 bytes do `vetor` volta ao `nao_jogador` na ordem `NON_PLAYER_ATTRIBUTE_ORDER`. A pessoa não
/// é usada. eax na saída: o vetor com o último byte copiado no byte de baixo (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_vector_to_non_player(nao_jogador: usize, _pessoa: usize, vetor: usize) -> u32 {
    let mut ultimo = 0u8;
    for (k, &d) in NON_PLAYER_ATTRIBUTE_ORDER.iter().enumerate() {
        ultimo = *((vetor + 3 * k) as *const u8);
        *((nao_jogador + d) as *mut u8) = ultimo;
    }
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// writes the low byte of the vector's cells 0..53 back to the player's attributes and cell 54 to staff +0x58
/// Original: `database_vector_to_player` (0x53aa10, Database.cpp, cdecl).
///
/// [pt] { 0x53aa10 } (cdecl, Database.cpp): o inverso de 0x53a610: o byte de baixo das células 0..53 do
/// `vetor` volta ao `jogador` nos deslocamentos de `ATTRIBUTE_VECTOR_CELLS` e o da célula 54 à `pessoa`
/// +0x58. eax na saída: o vetor com o último byte no byte de baixo (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_vector_to_player(jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    for (i, &(d, _)) in ATTRIBUTE_VECTOR_CELLS.iter().enumerate() {
        *((jogador + d as usize) as *mut u8) = *((vetor + 3 * i) as *const u8);
    }
    let ultimo = *((vetor + 3 * 54) as *const u8);
    *((pessoa + staff::DETERMINATION) as *mut u8) = ultimo;
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// rebalances the non-player's attributes in two passes: mean of the near-zero cells pulled to −5, positive excess
/// trimmed, cap −80 for job 10
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
pub(crate) unsafe extern "C" fn staff_normalise_non_player_attributes(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let nj = core::ptr::read_unaligned((pessoa + staff::NON_PLAYER) as *const usize);
    if nj == 0 {
        return;
    }
    let preencher: unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32 = core::mem::transmute(FN_DATABASE_NON_PLAYER_TO_ATTRIBUTE_VECTOR);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_ATTRIBUTE_VECTOR_TO_NON_PLAYER);
    let mut vetor = [0u8; 63 * 3];
    for c in vetor.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = vetor.as_mut_ptr();
    let mut fim = 0u8;
    preencher(nj, pessoa, v as usize, &mut fim);
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let fim = fim as i8;
    let mut marca = [0u32; 64];
    let cargo_10 = *((pessoa + staff::JOB_FOR_CLUB) as *const u8) == 10 || *((pessoa + staff::JOB_FOR_NATION) as *const u8) == 10;
    for step_towards in 0..2 {
        let bp = w(fim);
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limit = 0x28i32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if bp != -0x7f {
                let mut c = fim;
                loop {
                    let x = w(c) as i32;
                    if x.abs() <= limit as i16 as i32 {
                        total += x;
                        n += 1;
                        marca[c as usize] = 1;
                    } else {
                        marca[c as usize] = 0;
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
            // simplification: com a célula do corte = −127 o original roda para sempre; a cópia também
            limit += 10;
        }
        if melhor > -1 {
            marca[melhor as usize] = 1;
        }
        let delta = (-5i32).wrapping_sub(total / n as i32) as i16;
        if bp != -0x7f {
            let mut c = fim;
            loop {
                if c != melhor {
                    if marca[c as usize] == 1 {
                        let x = w(c).wrapping_add(delta);
                        pw(c, if x < -0x7d { -0x7d } else if x > 0x7d { 0x7d } else { x });
                    } else if step_towards == 0 && w(c) > 0 {
                        quantas += 1;
                        soma += w(c) as i32;
                    } else {
                        marca[c as usize] = 1;
                    }
                    if cargo_10 {
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
        if w(fim) != -0x7f {
            let mut c = fim;
            while quantas != 0 && soma > 0x1c {
                if marca[c as usize] != 1 {
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
    devolver(nj, pessoa, v as usize);
}




/// As cores fixas (chaves do index.cpp em 0x9cf178..0x9cf1fc, ids) e a família de cada uma, na ordem
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
pub(crate) unsafe fn colour_family(cor: usize) -> Option<u8> {
    let id = *(cor as *const u32);
    COLOUR_FAMILIES.iter().find(|&&(a, _)| *(a as *const u32) == id).map(|&(_, f)| f)
}

/// the colour's family (1..10) from the index.cpp colour keys
/// Original: `database_get_colour_family` (0x540520, Database.cpp, cdecl).
///
/// [pt] { 0x540520 } (cdecl, Database.cpp): a família (1..10) da `cor` pelo id (+0), comparado com as chaves
/// de cor em `COLOUR_FAMILIES` (a primeira que casa). Cor fora da lista: a original (caixa "Error",
/// [0xb67a34] = 0 e 2).
pub(crate) unsafe extern "C" fn database_get_colour_family(cor: usize) -> u8 {
    match colour_family(cor) {
        Some(f) => f,
        None => original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_DATABASE_GET_COLOUR_FAMILY)(cor),
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
    let familia: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_COLOUR_FAMILY);
    let fa = familia(a);
    let fb = familia(b);
    let par = |x: u8, y: u8| (fa == x && fb == y) || (fb == x && fa == y);
    let chave = *(KEY_COLOUR_PURPLE_3 as *const u32);
    if fa == fb || par(8, 9) || par(8, 7) {
        return 1;
    }
    // a família 6 contra a cor da chave é exceção: não se confundem (a original sai com 0 antes do par 6/7)
    if (fa == 6 && *(b as *const u32) == chave) || (fb == 6 && *(a as *const u32) == chave) {
        return 0;
    }
    par(6, 7) as u8
}

/// the coaching attribute 1..12: four straight from the record, eight through the 1-to-20 formula with the non-player
/// word +4
/// Original: `staff_compute_non_player_attribute` (0x543fd0, Database.cpp, cdecl).
///
/// [pt] { 0x543fd0 } (cdecl, Database.cpp): o `atributo` (1..12) de treinador da `pessoa`, do `nao_jogador`
/// (+0x69 dela). Diretos: 1 pessoa +0x56, 4 pessoa +0x58, 7 não jogador +0x14, 12 +0x22. Os outros
/// (2 +0x11, 3 +0x10, 5 +0x17, 6 +0x18, 8 +0x19, 9 +0x1b, 10 +0x1e, 11 +0x21) pela fórmula de 0x544410:
/// x = (palavra +4 + 2 × byte) × 0,05 + 10; (x + 0,1 x²) / 3 + 0,5, preso em 1..20 e truncado. Pessoa
/// ou não jogador nulos ou atributo fora de 1..12: a original (caixa e −1).
pub(crate) unsafe extern "C" fn staff_compute_non_player_attribute(pessoa: usize, nao_jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=12).contains(&a) || nao_jogador == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_NON_PLAYER_ATTRIBUTE)(pessoa, nao_jogador, atributo);
    }
    let byte = |p: usize, d: usize| *((p + d) as *const u8);
    let formula = |d: usize| {
        let b = *((nao_jogador + d) as *const i8) as f64;
        let x = (*((nao_jogador + 4) as *const i16) as f64 + (b + b)) * 0.05 + 10.0;
        let y = (x * x * 0.1 + x) * read_f64_at(0x968a80) + 0.5; // 0x968a80 = 0.3333333333
        match_pl_ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y }) as u8
    };
    match a {
        1 => byte(pessoa, 0x56),
        2 => formula(0x11),
        3 => formula(0x10),
        4 => byte(pessoa, 0x58),
        5 => formula(0x17),
        6 => formula(0x18),
        7 => byte(nao_jogador, 0x14),
        8 => formula(0x19),
        9 => formula(0x1b),
        10 => formula(0x1e),
        11 => formula(0x21),
        _ => byte(nao_jogador, 0x22),
    }
}




/// CRT rand() scaled to n: rand * n / 0x8000 (64-bit arithmetic, low 32 bits).
///
/// [pt] rand() da CRT (0x946a04) escalado a `n`: rand × n / 0x8000. As rotinas de nomes fazem a conta em
/// 64 bits (0x945e40 multiplica, 0x945e80 divide com sinal) e ficam com os 32 bits de baixo.
pub(crate) unsafe fn scaled_crt_rand(n: i32) -> i32 {
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_CRT_RAND);
    ((rand() as i64 * n as i64) / 0x8000) as i32
}

/// Name count in the global at `a` (first or second names), capped at 0x7fff.
///
/// [pt] O total de nomes da global `a` ([0xae2398] primeiros, [0xae239c] segundos), no máximo 0x7fff.
pub(crate) unsafe fn name_count_capped(a: usize) -> i32 {
    let t = *(a as *const i32);
    if t > 0x7fff { 0x7fff } else { t }
}

/// Nation's entry in the name index table ([0xae2338] + id x 36: first rows and counts of first/second names).
///
/// [pt] O índice de nomes da nação `n` (id em +0): [0xae2338] + id × 36 (+0 primeiro nome de onde começam os
/// primeiros nomes, +4 o dos segundos, +0x18 quantos primeiros, +0x1c quantos segundos).
pub(crate) unsafe fn nation_name_index(n: usize) -> usize {
    (*(G_NATION_NAME_INDEX_TABLE as *const usize)).wrapping_add((*(n as *const i32) as usize).wrapping_mul(36))
}

/// A nação que empresta nomes a cada grupo de língua (nação +0x77, 1..32): tabelas de salto 0x53a060 e
/// 0x53a528 (iguais). O grupo 17 (0) sorteia entre [0x9cf484] e [0x9cf404].
pub(crate) const LANGUAGE_GROUP_NATIONS: [usize; 32] = [
    KEY_NATION_ALBANIA, KEY_NATION_SAUDI_ARABIA, KEY_NATION_RUSSIA, KEY_NATION_CHINA, KEY_NATION_CROATIA, KEY_NATION_CZECH_REPUBLIC, KEY_NATION_HOLLAND, KEY_NATION_ENGLAND, KEY_NATION_FINLAND, KEY_NATION_FRANCE, KEY_NATION_GERMANY,
    KEY_NATION_GREECE, KEY_NATION_ISRAEL, KEY_NATION_HUNGARY, KEY_NATION_ITALY, KEY_NATION_JAPAN, 0, KEY_NATION_POLAND, KEY_NATION_PORTUGAL, KEY_NATION_ROMANIA, KEY_NATION_RUSSIA, KEY_NATION_NORWAY,
    KEY_NATION_CROATIA, KEY_NATION_SPAIN, KEY_NATION_TURKEY, KEY_NATION_BURMA, KEY_NATION_MALAYSIA, KEY_NATION_PERU, KEY_NATION_SLOVAKIA, KEY_NATION_SWEDEN, KEY_NATION_ICELAND, KEY_NATION_DENMARK,
];

/// For a nation with few names: picks a nation of the same language group (table or up to 40 random tries) with enough
/// names; returns (count, first row) or None to draw from the whole table.
///
/// [pt] O trecho comum de 0x539c60 (0x539df0..) e 0x53a1b0 (0x53a2be..) para nação com até 100 nomes e
/// grupo de língua: a nação do grupo (tabela) se ela tem ≥ sorteio(20) + 15 nomes em `conta`; senão até
/// 40 nações sorteadas ([0xae2350], 0x122 bytes cada em [0xae23a8]) até achar uma do mesmo grupo com
/// mais de 5 primeiros nomes (+0x18, nos dois casos). Devolve (quantos em `conta`, primeiro em `inicio`)
/// se a nação escolhida tem ≥ 5 nomes; senão `None` (sorteio na tabela inteira).
pub(crate) unsafe fn name_donor_nation(nacao: usize, conta: usize, inicio: usize) -> Option<(i32, i32)> {
    let g = |a: usize| *(a as *const i32);
    let nacoes = *(G_NATION_TABLE as *const usize);
    let nacao_de = |id: i32| nacoes.wrapping_add((id as usize).wrapping_mul(nation::SIZE));
    let grupo = *((nacao + nation::FIRST_LANGUAGE) as *const i8);
    let k = (grupo as i32).wrapping_sub(1) as u32;
    let mut esi = 0usize;
    let mut direto = false;
    if k <= 0x1f {
        let id = if k == 16 {
            if scaled_crt_rand(2) != 0 { g(KEY_NATION_SOUTH_KOREA) } else { g(KEY_NATION_NORTH_KOREA) }
        } else {
            g(LANGUAGE_GROUP_NATIONS[k as usize])
        };
        esi = nacao_de(id);
        if esi != 0 {
            let minimo = scaled_crt_rand(0x14).wrapping_add(0xf);
            direto = g(nation_name_index(esi) + conta) >= minimo;
        }
    }
    if !direto {
        for _ in 0..40 {
            esi = nacao_de(scaled_crt_rand(g(0xae2350)));
            if *((esi + 0x77) as *const i8) == grupo && g(nation_name_index(esi) + 0x18) > 5 {
                break;
            }
        }
        if esi == 0 {
            return None;
        }
    }
    let e = nation_name_index(esi);
    let quantos = g(e + conta);
    if quantos < 5 {
        return None;
    }
    Some((quantos, g(e + inicio)))
}

/// Name accepted by 0x539c60's draw: null/empty yes; a single letter or one containing '.' no.
///
/// [pt] Nome aceito no sorteio de 0x539c60: nulo ou vazio sim; uma letra só ou com ponto não.
pub(crate) unsafe fn is_acceptable_name(nome: usize) -> bool {
    if nome == 0 || *(nome as *const u8) == 0 {
        return true;
    }
    let s = core::ffi::CStr::from_ptr(nome as *const core::ffi::c_char).to_bytes();
    s.len() != 1 && !s.contains(&b'.')
}

/// draws a first name for the nation (20 draws by weight when it has over 100; otherwise borrows from the language
/// group)
/// Original: `database_pick_first_name_for_nation` (0x539c60, Database.cpp, cdecl).
///
/// [pt] { 0x539c60 } (cdecl, Database.cpp): um primeiro nome (linha de 60 bytes em [0xae23f0]) para a `nacao`.
/// Sem nação: sorteio na tabela inteira. Nação com mais de 100 primeiros nomes: 20 sorteios na faixa
/// dela, fica o de maior peso (+0x3b, com sinal) entre os aceitos (`is_acceptable_name`). Com até 100: grupo de
/// língua 0 sorteia na tabela inteira, mas devolve a linha da tabela dos SEGUNDOS nomes ([0xae23f4],
/// assim na original); senão `name_donor_nation`. Só o rand() da CRT (não o 0x90cfc0).
pub(crate) unsafe extern "C" fn database_pick_first_name_for_nation(nacao: usize) -> usize {
    let linha = |t: usize, i: i32| (*(t as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao == 0 {
        return linha(G_FIRST_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398)));
    }
    let e = nation_name_index(nacao);
    let quantos = *((e + 0x18) as *const i32);
    if quantos > 100 {
        let inicio = *(e as *const i32);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 sorteios do primeiro nome
            let i = scaled_crt_rand(quantos).wrapping_add(inicio);
            let nome = linha(G_FIRST_NAME_TABLE, i);
            let peso = *((nome + 0x3b) as *const i8);
            if peso > melhor && is_acceptable_name(nome) {
                melhor = peso;
                escolhido = i;
            }
        }
        return linha(G_FIRST_NAME_TABLE, escolhido);
    }
    if *((nacao + nation::FIRST_LANGUAGE) as *const u8) == 0 {
        return linha(G_SECOND_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398)));
    }
    match name_donor_nation(nacao, 0x18, 0) {
        Some((n, inicio)) => linha(G_FIRST_NAME_TABLE, scaled_crt_rand(n).wrapping_add(inicio)),
        None => linha(G_FIRST_NAME_TABLE, scaled_crt_rand(name_count_capped(0xae2398))),
    }
}

/// draws a second name for the nation, the same way
/// Original: `database_pick_second_name_for_nation` (0x53a1b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a1b0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) para a `nacao`.
/// Como `database_pick_first_name_for_nation` com os segundos nomes (+4, +0x1c, [0xae239c]), sem filtrar os nomes
/// no sorteio de 20 (só o maior peso).
pub(crate) unsafe extern "C" fn database_pick_second_name_for_nation(nacao: usize) -> usize {
    let linha = |i: i32| (*(G_SECOND_NAME_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao == 0 {
        return linha(scaled_crt_rand(name_count_capped(0xae239c)));
    }
    let e = nation_name_index(nacao);
    let quantos = *((e + 0x1c) as *const i32);
    if quantos > 100 {
        let inicio = *((e + 4) as *const i32);
        let (mut melhor, mut escolhido) = (-1i8, 0i32);
        for _ in 0..20 { // 20 sorteios do segundo nome
            let i = scaled_crt_rand(quantos).wrapping_add(inicio);
            let peso = *((linha(i) + 0x3b) as *const i8);
            if peso > melhor {
                melhor = peso;
                escolhido = i;
            }
        }
        return linha(escolhido);
    }
    if *((nacao + nation::FIRST_LANGUAGE) as *const u8) == 0 {
        return linha(scaled_crt_rand(name_count_capped(0xae239c)));
    }
    match name_donor_nation(nacao, 0x1c, 4) {
        Some((n, inicio)) => linha(scaled_crt_rand(n).wrapping_add(inicio)),
        None => linha(scaled_crt_rand(name_count_capped(0xae239c))),
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
pub(crate) unsafe fn copy_formatted_name(destino: usize, origem: usize) {
    let mut i = 0;
    loop {
        let b = *((origem + i) as *const u8);
        *((destino + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// writes the person's name in one of 7 styles (first last, F.last, last, first, initials...), preferring the common
/// name or nickname
/// Original: `database_format_staff_name` (0x53dba0, Database.cpp, cdecl).
///
/// [pt] { 0x53dba0 } (cdecl, Database.cpp): o nome da `pessoa` em `destino` no `estilo` (byte, 1..7), com
/// nome (+4), sobrenome (+8) e nome comum (+0xc, ou o apelido dado pelo humano: 0x600740/0x6005e0 em
/// [0xb63c98]). 1 "nome sobrenome", 2 "N.sobrenome", 3 "sobrenome, nome", 4 "sobrenome, N", 5 só um
/// nome, 6 iniciais, 7 como o 1 sem preferir o comum; sempre com recuos (o que houver). Devolve 1.
/// Pessoa ou destino nulos, tamanho (i16) < 0x69 ou estilo fora de 1..7: a original (caixa e 0).
pub(crate) unsafe extern "C" fn database_format_staff_name(pessoa: usize, destino: usize, tamanho: u32, estilo: u32) -> u32 {
    let e = estilo as u8;
    if pessoa == 0 || destino == 0 || (tamanho as i16) < 0x69 || !(1..=7).contains(&e) {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_DATABASE_FORMAT_STAFF_NAME)(pessoa, destino, tamanho, estilo);
    }
    let deu_apelido: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_HAS_NICKNAME);
    let apelido: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_NICKNAME);
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let ler = |d: usize| core::ptr::read_unaligned((pessoa + d) as *const usize);
    let (nome, sobrenome) = (ler(4), ler(8));
    let apelidos = *(G_HUMAN_MANAGER as *const usize);
    let comum = if apelidos != 0 && deu_apelido(apelidos, pessoa, 0) != 0 { apelido(apelidos, pessoa, 0) } else { ler(0xc) };
    *(destino as *mut u8) = 0;
    let (c, n, s) = (has_text(comum), has_text(nome), has_text(sobrenome));
    let letra = |p: usize| *(p as *const i8) as i32;
    let copiar = |p: usize| copy_formatted_name(destino, p);
    // os recuos da original voltam a testar o comum depois do sobrenome: sem efeito (já era vazio)
    match e {
        1 | 7 => {
            if e == 1 && c {
                copiar(comum);
            } else if n && s {
                fmt(destino, 0x98f0b8, nome, sobrenome); // "%s %s"
            } else if s {
                copiar(sobrenome);
            } else if c {
                copiar(comum);
            } else if n {
                copiar(nome);
            }
        }
        2 | 3 | 4 => {
            if c {
                copiar(comum);
            } else if n && s {
                match e {
                    2 => fmt(destino, 0x9b7410, letra(nome), sobrenome), // "%c.%s"
                    3 => fmt(destino, 0x98f6ac, sobrenome, nome),        // "%s, %s"
                    _ => fmt(destino, 0x9b7408, sobrenome, letra(nome)), // "%s, %c"
                };
            } else if s {
                copiar(sobrenome);
            } else if n {
                copiar(nome);
            }
        }
        5 => {
            if c {
                copiar(comum);
            } else if s {
                copiar(sobrenome);
            } else if n {
                copiar(nome);
            }
        }
        _ => {
            // 6: iniciais
            if c {
                fmt(destino, 0x99ec1c, letra(comum)); // "%c"
            } else if n && s {
                fmt(destino, 0x9b7400, letra(nome), letra(sobrenome)); // "%c%c"
            } else if s {
                fmt(destino, 0x99ec1c, letra(sobrenome));
            } else if n {
                fmt(destino, 0x99ec1c, letra(nome));
            }
        }
    }
    1
}




/// the club's home, away or third kit colours as screen pixels, with defaults when missing
/// Original: `database_get_club_kit_colours` (0x53b640, Database.cpp, cdecl).
///
/// [pt] { 0x53b640 } (cdecl, Database.cpp): as cores do `uniforme` (byte: 1 casa +0x83/+0x87, 2 fora
/// +0x8b/+0x8f, 3 terceiro +0x93/+0x97; 2 e 3 sem uma das duas cores: as de casa trocadas) do `clube`
/// em pixels (0x5e4800 com os bytes +0x37, +0x38, +0x39 da cor e formato 0): `frente` e `fundo`, que
/// começam com [0xaebdf0] e [0xae3184]. Outro uniforme: só os padrões. Devolve 1. Clube nulo: a
/// original (padrões, caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_club_kit_colours(clube: usize, kit: u32, frente: *mut u16, fundo: *mut u16) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize, u32, *mut u16, *mut u16) -> u32>(IDX_DATABASE_GET_CLUB_KIT_COLOURS)(clube, kit, frente, fundo);
    }
    // simplification: a original empurra dwords com só o byte de baixo escrito (o resto é lixo de pilha);
    // a 0x5e4800 faz & 0xff nos três, então os bytes estendidos dão o mesmo pixel
    let cor: unsafe extern "C" fn(u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    core::ptr::write_unaligned(frente, *(G_GUI_COLOUR_BLUE as *const u16));
    core::ptr::write_unaligned(fundo, *(G_GUI_COLOUR_PALE_GREY as *const u16));
    let r = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let (a, b) = match kit as u8 {
        1 => (r(0x83), r(0x87)), // casa
        2 | 3 => {
            let d = if kit as u8 == 2 { 0x8b } else { 0x93 };
            let (a, b) = (r(d), r(d + 4));
            if a == 0 || b == 0 { (r(0x87), r(0x83)) } else { (a, b) }
        }
        _ => return 1,
    };
    let pixel = |c: usize| {
        let byte = |d: usize| *((c + d) as *const u8) as u32;
        cor(byte(0x37), byte(0x38), byte(0x39), 0) as u16
    };
    if a != 0 {
        core::ptr::write_unaligned(frente, pixel(a));
    }
    if b != 0 {
        core::ptr::write_unaligned(fundo, pixel(b));
    }
    1
}

/// the manager of the club, of its A/B partner or, for an under-21 side, the senior national team's
/// Original: `database_get_club_manager` (0x53ca90, Database.cpp, cdecl).
///
/// [pt] { 0x53ca90 } (cdecl, Database.cpp): o técnico (+0xcf) do `clube`; sem ele, o do parceiro A/B
/// (0x540a50 com 0 e 1); sem os dois, clube comum dá 0 e seleção (id ≥ [0xae235c] − [0xae2350]) repete
/// com a seleção da nação dela (0x53b3d0 com +0x53, a sub-21 chega na principal). Clube nulo (no começo
/// ou vindo da 0x53b3d0): a original com nulo (caixa "Error" e 0).
pub(crate) unsafe extern "C" fn database_get_club_manager(clube: usize) -> usize {
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let selecao_da_nacao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let tecnico = |c: usize| core::ptr::read_unaligned((c + 0xcf) as *const usize);
    let mut c = clube;
    loop {
        if c == 0 {
            return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_DATABASE_GET_CLUB_MANAGER)(0);
        }
        let t = tecnico(c);
        if t != 0 {
            return t; // o técnico do clube
        }
        let p = parceiro(c, 0, 1);
        if p != 0 && tecnico(p) != 0 {
            return tecnico(p);
        }
        if *(c as *const i32) < (*(G_CLUB_COUNT as *const i32)).wrapping_sub(*(G_NATION_COUNT as *const i32)) {
            return 0;
        }
        c = selecao_da_nacao(core::ptr::read_unaligned((c + 0x53) as *const usize));
    }
}

/// the club's name, or its English one from eng.lng when another language is loaded
/// Original: `club_get_english_name` (0x545880, Database.cpp, cdecl).
///
/// [pt] { 0x545880 } (cdecl, Database.cpp): o nome do `clube` em inglês. Idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20): pelo mapa
/// de ids [0xae23fc] se houver, senão pelo id se < quantos (+0), senão 0.
pub(crate) unsafe extern "C" fn club_get_english_name(clube: usize) -> usize {
    let idioma = *(G_DATABASE_NAMES_LANGUAGE as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(4); // idioma 0 ou 1
    }
    let t = *(G_ENGLISH_NAME_TABLES as *const usize);
    if t == 0 {
        // simplification: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_CLUB_GET_ENGLISH_NAME)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(G_CLUB_ENGLISH_NAME_INDEX_TABLE as *const usize);
    if mapa != 0 {
        let i = *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32);
        return linhas.wrapping_add((i as usize).wrapping_mul(79));
    }
    if id < *(t as *const i32) {
        linhas.wrapping_add((id as usize).wrapping_mul(79))
    } else {
        0
    }
}




/// adds a delta to the player's morale, damped by a draw near the ends, clamped to 1..20; returns whether it changed
/// Original: `staff_adjust_morale` (0x544710, Database.cpp, cdecl).
///
/// [pt] { 0x544710 } (cdecl, Database.cpp): soma `delta` (byte com sinal) ao moral (+0x45) do jogador
/// (pessoa +0x61), amortecido perto das pontas: subindo acima de 10, se aleatorio(21) < moral + delta
/// soma só aleatorio(delta + 1); descendo abaixo de 10, se aleatorio((jogador +0x2f + pessoa +0x5a +
/// pessoa +0x58) / 3) > moral + delta soma aleatorio(delta − 1). Prende em 1..20 (byte com sinal); com
/// `limitar` e delta > 0, o vínculo com o clube (0x4cf5f0, pessoa +0x39) com +0x45 não nulo impede
/// subir. Devolve 1 se o moral mudou. Delta 0: 0, sem ler nada.
pub(crate) unsafe extern "C" fn staff_adjust_morale(pessoa: usize, delta: u32, limitar: u32) -> u32 {
    let aleatorio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let d = delta as u8 as i8;
    if d == 0 {
        return 0;
    }
    let jogador = core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize);
    let moral = *((jogador + 0x45) as *const i8);
    let soma = moral as i32 + d as i32;
    let simples = moral.wrapping_add(d);
    let mut novo = if d > 0 {
        if soma > 10 && aleatorio(0x15) < soma { moral.wrapping_add(aleatorio(d as i32 + 1) as i8) } else { simples }
    } else if soma < 10 {
        let b = |a: usize| *(a as *const i8) as i32;
        let teto = (b(jogador + 0x2f) + b(pessoa + staff::PRESSURE) + b(pessoa + staff::DETERMINATION)) / 3;
        if aleatorio(teto) > soma { moral.wrapping_add(aleatorio(d as i32 - 1) as i8) } else { simples }
    } else {
        simples
    };
    novo = if novo < 1 { 1 } else if novo > 20 { 20 } else { novo };
    if limitar != 0 && d > 0 {
        let v = contract_bond(*(G_CONTRACT_MANAGER as *const usize), pessoa, core::ptr::read_unaligned((pessoa + staff::CLUB_JOB) as *const usize));
        if v != 0 && core::ptr::read_unaligned((v + 0x45) as *const u32) != 0 && novo >= moral {
            novo = moral;
        }
    }
    *((jogador + 0x45) as *mut i8) = novo;
    (novo != moral) as u32
}

/// Os bytes do não jogador (+0x0e..+0x22) na ordem das 21 células do vetor de 0x53a8b0 (13 antes do
/// corte, 8 depois).
pub(crate) const NON_PLAYER_ATTRIBUTE_ORDER: [usize; 21] = [
    0x0e, 0x0f, 0x12, 0x13, 0x14, 0x15, 0x16, 0x1a, 0x1c, 0x1d, 0x1f, 0x20, 0x22, 0x11, 0x10, 0x17, 0x18, 0x19, 0x1b,
    0x1e, 0x21,
];

/// widens the non-player's coaching attributes (+0x0e..+0x22) into a 21-cell vector of 3 bytes, with the split at 13
/// Original: `database_non_player_to_attribute_vector` (0x53a8b0, Database.cpp, cdecl).
///
/// [pt] { 0x53a8b0 } (cdecl, Database.cpp): estende (movsx) os atributos do `nao_jogador` para os 16 bits de
/// baixo das 21 células de 3 bytes do `vetor` (o 3º byte de cada uma fica como está) e grava 13 (o
/// corte) em `fim` se não nulo. A pessoa não é usada. eax na saída: os 16 bits altos do ponteiro do não
/// jogador e o último atributo estendido (os chamadores não usam).
pub(crate) unsafe extern "C" fn database_non_player_to_attribute_vector(nao_jogador: usize, _pessoa: usize, vetor: usize, fim: *mut u8) -> u32 {
    let mut ultimo = 0u16;
    for (k, &d) in NON_PLAYER_ATTRIBUTE_ORDER.iter().enumerate() {
        if k == 13 && !fim.is_null() {
            *fim = 0xd;
        }
        ultimo = *((nao_jogador + d) as *const i8) as i16 as u16;
        core::ptr::write_unaligned((vetor + 3 * k) as *mut u16, ultimo);
    }
    (nao_jogador as u32 & 0xffff_0000) | ultimo as u32
}

/// sets the person's club or national squad bits, keeping the other pair
/// Original: `staff_set_squad_flags` (0x540980, Database.cpp, cdecl).
///
/// [pt] { 0x540980 } (cdecl, Database.cpp): o `elenco` (byte) nas marcas de elenco da pessoa (+0x5e): com
/// bits de clube (& 3) mantém os de seleção (& 0xc), senão mantém os de clube; depois acrescenta. A
/// original grava duas vezes (a máscara e o resultado). Devolve a pessoa (eax).
pub(crate) unsafe extern "C" fn staff_set_squad_flags(pessoa: usize, elenco: u32) -> usize {
    let e = elenco as u8;
    let p = (pessoa + staff::PLAYING_SQUAD) as *mut u8;
    let mantem = if e & 3 != 0 { *p & 0xc } else { *p & 3 };
    *p = mantem;
    *p = mantem | e;
    pessoa
}




/// qsort comparison of the competition history: rows without a competition last, by competition id and by season,
/// newest first
/// Original: `database_compare_comp_history_rows` (0x540780, Database.cpp, cdecl).
///
/// [pt] { 0x540780 } (cdecl, Database.cpp): comparação do qsort das linhas do histórico de competições.
/// Linha sem competição (+4) vai para o fim (a sem e b com: 1; a com e b sem: −1; as duas sem: 0);
/// depois pelo id da competição (a − b); na mesma, pela temporada (+8, palavra) da mais nova para a
/// mais velha (b − a), a não ser que uma delas seja −1 (0).
pub(crate) unsafe extern "C" fn database_compare_comp_history_rows(a: usize, b: usize) -> i32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let ca = ler(a, 4);
    if ca == 0 {
        return (ler(b, 4) != 0) as i32;
    }
    let cb = ler(b, 4);
    if cb == 0 {
        return -1;
    }
    let d = (*(ca as *const i32)).wrapping_sub(*(cb as *const i32));
    if d != 0 {
        return d; // pelo id da competição
    }
    let ta = core::ptr::read_unaligned((a + 8) as *const i16);
    let tb = core::ptr::read_unaligned((b + 8) as *const i16);
    if ta == -1 || tb == -1 {
        return 0;
    }
    tb as i32 - ta as i32 // a temporada mais nova primeiro
}

/// As 54 primeiras células do vetor de atributos (0x53a610), na ordem em que a original grava:
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

/// widens the player's positions, sides and attributes into a 63 cell vector (value and weight) and returns the group
/// ends (9, 0xc, 0x20)
/// Original: `database_player_to_attribute_vector` (0x53a610, Database.cpp, cdecl).
///
/// [pt] { 0x53a610 } (cdecl, Database.cpp): estende os bytes do registro de `jogador` num vetor de 63
/// células de 3 bytes (palavra com sinal + peso): zera os 63 pesos; células 0..53 pela tabela
/// `ATTRIBUTE_VECTOR_CELLS` (posições 0..8, lados 9..11, depois os atributos, peso 2 nos que contam e 1 no
/// +0x2a); a célula 54 é o byte +0x58 da `pessoa`. Os fins de grupo vão às saídas não nulas:
/// posições 9, lados 0xc, ponderados 0x20. Devolve o vetor (eax).
pub(crate) unsafe extern "C" fn database_player_to_attribute_vector(jogador: usize, pessoa: usize, vetor: usize, fim_dos_lados: *mut u8,
                                                   fim_das_posicoes: *mut u8, fim_dos_ponderados: *mut u8) -> usize {
    for i in 0..0x3f {
        *((vetor + 3 * i + 2) as *mut u8) = 0;
    }
    for (i, &(d, peso)) in ATTRIBUTE_VECTOR_CELLS.iter().enumerate() {
        core::ptr::write_unaligned((vetor + 3 * i) as *mut i16, *((jogador + d as usize) as *const i8) as i16);
        *((vetor + 3 * i + 2) as *mut u8) = peso;
        match i {
            8 if !fim_das_posicoes.is_null() => *fim_das_posicoes = 9,
            11 if !fim_dos_lados.is_null() => *fim_dos_lados = 0xc,
            31 if !fim_dos_ponderados.is_null() => *fim_dos_ponderados = 0x20,
            _ => {}
        }
    }
    core::ptr::write_unaligned((vetor + 3 * 54) as *mut i16, *((pessoa + staff::DETERMINATION) as *const i8) as i16);
    vetor
}

/// whether the nation is England, Scotland, Ireland, Wales or Northern Ireland
/// Original: `nation_is_uk_or_ireland` (0x5446d0, Database.cpp, cdecl).
///
/// [pt] { 0x5446d0 } (cdecl, Database.cpp): 1 se a `nacao` (não nula) é Inglaterra, Escócia, Irlanda, País
/// de Gales ou Irlanda do Norte (a regra das nações da casa).
pub(crate) unsafe extern "C" fn nation_is_uk_or_ireland(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    let id = g(nacao);
    (id == g(KEY_NATION_ENGLAND) || id == g(KEY_NATION_SCOTLAND) || id == g(KEY_NATION_REPUBLIC_OF_IRELAND) || id == g(KEY_NATION_WALES) || id == g(KEY_NATION_NORTHERN_IRELAND)) as u32
}




/// Os 23 clubes bascos de 0x53d7f0 (ponteiros fixos da área 0x9d00a4..), na ordem da original.
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
pub(crate) unsafe extern "C" fn database_is_basque_club(clube: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if clube == 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    if nacao == 0 || g(nacao) != g(KEY_NATION_SPAIN) {
        return 0; // fora da Espanha
    }
    let id = g(clube);
    BASQUE_CLUBS.iter().any(|&a| id == g(a)) as u32
}

/// Contract link (0x4cf5f0) of the person with the club; with no club given, uses the club of the person's contract
/// (stored in G_CONTRACT_LOOKUP_RESULT); returns (link, club) or (0, 0).
///
/// [pt] O vínculo (0x4cf5f0) da `pessoa` (não nula) com o `clube` e o clube usado: sem clube, o do contrato
/// (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
/// Sem contrato ou clube: (0, 0). Comum a 0x5444a0 e 0x544590 (e 0x544520, 0x544600).
pub(crate) unsafe fn contract_link_for_club(pessoa: usize, clube: usize) -> (usize, usize) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mut clube = clube;
    if clube == 0 {
        let c = contract_of(*(G_CONTRACT_MANAGER as *const usize), pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        if c == 0 {
            return (0, 0);
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return (0, 0);
        }
    }
    (contract_bond(*(G_CONTRACT_MANAGER as *const usize), pessoa, clube), clube)
}

/// whether the person's link with the club (or the contract club) is transfer listed by the club; job 0xf at a club
/// without a human manager does not count
/// Original: `staff_is_transfer_listed` (0x5444a0, Database.cpp, cdecl).
///
/// [pt] { 0x5444a0 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de transferências pelo clube (vínculo +0x4e, bit 1), a não ser que o clube
/// não seja de técnico humano (0x600a50 com 1, 1, 0, 0) e a função no clube da pessoa (+0x3d) seja 0xf.
pub(crate) unsafe extern "C" fn staff_is_transfer_listed(pessoa: usize, clube: usize) -> u32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if pessoa == 0 {
        return 0;
    }
    let (v, clube) = contract_link_for_club(pessoa, clube);
    if v == 0 || *((v + 0x4e) as *const u8) & 1 == 0 {
        return 0;
    }
    if humano(clube, 1, 1, 0, 0) == 0 && *((pessoa + staff::JOB_FOR_CLUB) as *const u8) == 0xf {
        return 0;
    }
    1
}

/// whether the person's link with the club (or the contract club) is loan listed
/// Original: `staff_is_loan_listed` (0x544590, Database.cpp, cdecl).
///
/// [pt] { 0x544590 } (cdecl, Database.cpp): 1 se o vínculo da `pessoa` com o `clube` (sem clube, o do
/// contrato) está na lista de empréstimos (vínculo +0x4e, bit 2).
pub(crate) unsafe extern "C" fn staff_is_loan_listed(pessoa: usize, clube: usize) -> u32 {
    if pessoa == 0 {
        return 0;
    }
    let (v, _) = contract_link_for_club(pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 2 != 0) as u32
}




/// whether the person's link with the club (or the contract club) is transfer listed at request
/// Original: `staff_is_listed_by_request` (0x544600, Database.cpp, cdecl).
///
/// [pt] { 0x544600 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências a pedido (vínculo +0x4e, bit 8). Sem clube, o do contrato (0x4cf5a0,
/// guardado em [0xae1080]; contrato +4 = id na tabela [0xae23b4], 581 bytes por clube).
pub(crate) unsafe extern "C" fn staff_is_listed_by_request(pessoa: usize, clube: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    if pessoa == 0 {
        return 0;
    }
    let mut clube = clube;
    if clube == 0 {
        let c = contract_of(*(G_CONTRACT_MANAGER as *const usize), pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        if c == 0 {
            return 0;
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return 0;
        }
    }
    let v = contract_bond(*(G_CONTRACT_MANAGER as *const usize), pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 8 != 0) as u32
}

/// the 1 to 20 rating of 2 × the attribute byte + the player's current ability
/// Original: `player_attribute_rating` (0x544380, Database.cpp, cdecl).
///
/// [pt] { 0x544380 } (cdecl, Database.cpp): a nota de 1 a 20 (`attribute_rating`) de 2 × `valor` (byte com
/// sinal) + habilidade atual (+5) do `jogador`. Termina no `_ftol` (eax inteiro, 1..20).
pub(crate) unsafe extern "C" fn player_attribute_rating(jogador: usize, valor: u32) -> u32 {
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    attribute_rating((habilidade + 2 * (valor as u8 as i8 as i32)) as f64) as u32
}

/// whether the club may have a reserve side: not national teams, A/B pairs, Spanish, German and Portuguese clubs and
/// some second national sides
/// Original: `database_club_has_reserve_squad` (0x542740, Database.cpp, cdecl).
///
/// [pt] { 0x542740 } (cdecl, Database.cpp): 1 se o `clube` pode ter time reserva. Não: seleções (id ≥
/// [0xae235c] − [0xae2350]), clubes com parceiro A/B (0x540a50 com nulo e 1); na faixa das segundas
/// seleções (id ≥ [0xae235c] − 2 × [0xae2350]), nação Espanha, Noruega, [0x9cf360], Brasil ou
/// Argentina ou continente (+0x71) [0x9cfa18] ou [0x9cfa08]; nos demais, nação Espanha, Alemanha ou
/// Portugal. Clube nulo: caixa "Error" e 0 (a original).
pub(crate) unsafe extern "C" fn database_club_has_reserve_squad(clube: usize) -> u32 {
    if clube == 0 {
        return original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_DATABASE_CLUB_HAS_RESERVE_SQUAD)(clube);
    }
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let g = |a: usize| *(a as *const u32);
    let gi = |a: usize| *(a as *const i32);
    let id = gi(clube);
    if id >= gi(0xae235c).wrapping_sub(gi(0xae2350)) {
        return 0; // seleção
    }
    if parceiro(clube, 0, 1) != 0 {
        return 0;
    }
    let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
    if nacao == 0 {
        return 1;
    }
    let n = g(nacao);
    if id >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2)) {
        if [KEY_NATION_SPAIN, KEY_NATION_NORWAY, KEY_NATION_ITALY, KEY_NATION_BRAZIL, KEY_NATION_ARGENTINA].iter().any(|&a| n == g(a)) {
            return 0;
        }
        let continente = core::ptr::read_unaligned((nacao + nation::CONTINENT) as *const usize);
        if continente != 0 && (g(continente) == g(KEY_CONTINENT_OCEANIA) || g(continente) == g(KEY_CONTINENT_AFRICA)) {
            return 0;
        }
        return 1;
    }
    if n == g(KEY_NATION_SPAIN) || n == g(KEY_NATION_GERMANY) || n == g(KEY_NATION_PORTUGAL) {
        return 0; // Espanha, Alemanha, Portugal
    }
    1 // pode
}




/// how far the viewer stands from the person (0 same club or league, 2 same nation, 3 club nation or high reputation, 4
/// world): picks the reputation
/// Original: `database_get_reputation_scope` (0x5408c0, Database.cpp, cdecl).
///
/// [pt] { 0x5408c0 } (cdecl, Database.cpp): de que distância `quem_ve` olha a `pessoa`, o que escolhe a
/// reputação: 0 = mesmo clube ou clubes da mesma nação (+0x39, clube +0x53) → a nacional; 2 = mesma
/// nação (+0x1a) ou não estrangeiro nela (0x53d980) → a atual; 3 = não estrangeiro na nação do clube
/// da pessoa, ou reputação de jogador (+0x61, +9) + reputação do clube de quem vê (+0x80) ≥ 14000 →
/// a média; 4 (sem quem vê, quem vê entre os 16 últimos ids = técnicos humanos, sem dados ou sem
/// clube) → a mundial. Só o al é devolvido (quem chama usa al).
pub(crate) unsafe extern "C" fn database_get_reputation_scope(pessoa: usize, quem_ve: usize) -> u8 {
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    if quem_ve == 0 {
        return 4;
    }
    let clube = ler(pessoa, 0x39);
    if clube != 0 {
        if clube == ler(quem_ve, 0x39) {
            return 0;
        }
        let outro = ler(quem_ve, 0x39);
        if outro != 0 && ler(clube, 0x53) == ler(outro, 0x53) {
            return 0;
        }
    }
    let nacao = ler(pessoa, 0x1a);
    if nacao == ler(quem_ve, 0x1a) {
        return 2; // mesma nação
    }
    if estrangeira(quem_ve, nacao, 1) == 0 {
        return 2;
    }
    let clube = ler(pessoa, 0x39);
    if clube != 0 && estrangeira(quem_ve, ler(clube, 0x53), 1) == 0 {
        return 3;
    }
    if *(quem_ve as *const i32) >= (*(G_STAFF_COUNT as *const i32)).wrapping_sub(16) {
        return 4;
    }
    let jogador = ler(pessoa, 0x61);
    if jogador == 0 {
        return 4;
    }
    let clube_de_quem_ve = ler(quem_ve, 0x39);
    if clube_de_quem_ve == 0 {
        return 4;
    }
    let soma = core::ptr::read_unaligned((jogador + 9) as *const i16) as i32 + core::ptr::read_unaligned((clube_de_quem_ve + club::REPUTATION) as *const i16) as i32;
    if soma >= 0x36b0 {
        3
    } else {
        4
    }
}

/// the person's reputation as seen by a viewer: current, home, world or a blend, by the scope; player or non-player
/// data
/// Original: `database_get_staff_reputation_for_viewer` (0x5407e0, Database.cpp, cdecl).
///
/// [pt] { 0x5407e0 } (cdecl, Database.cpp): a reputação da `pessoa` vista por `quem_ve`, pelo alcance
/// (0x5408c0): dos dados de jogador (+0x61: atual +9, nacional +0xb, mundial +0xd) ou, com
/// `nao_jogador`, dos de não jogador (+0x69: +8, +0xa, +0xc). 4 → mundial, 2 → atual, 0 → nacional,
/// 3 → metade da mundial + metade da atual, outro → metade da nacional + metade da atual (divisões
/// truncadas). Só o ax é devolvido.
pub(crate) unsafe extern "C" fn database_get_staff_reputation_for_viewer(pessoa: usize, quem_ve: usize, nao_jogador: u32) -> u16 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_REPUTATION_SCOPE);
    let e = escopo(pessoa, quem_ve);
    let (dados, atual) = if nao_jogador != 0 {
        (core::ptr::read_unaligned((pessoa + staff::NON_PLAYER) as *const usize), 8)
    } else {
        (core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize), 9)
    };
    // r(0) atual, r(2) nacional, r(4) mundial
    let r = |d: usize| core::ptr::read_unaligned((dados + atual + d) as *const i16) as i32;
    (match e {
        4 => r(4),
        2 => r(0), // escopo 2: a atual
        0 => r(2),
        3 => r(4) / 2 + r(0) / 2,
        _ => r(2) / 2 + r(0) / 2,
    }) as u16
}

/// whether the person is foreign in the nation: nationalities, groups (British Isles, Australia, [0x9cf360]) and the
/// Portugal rule for contracts up to 1998
/// Original: `database_is_staff_foreign` (0x53d980, Database.cpp, cdecl).
///
/// [pt] { 0x53d980 } (cdecl, Database.cpp): 1 se a `pessoa` é estrangeira na `nacao` (nula: 1). Não é com
/// a nação na primeira (+0x1a) ou segunda nacionalidade (+0x1e). Com `grupos`, também não é: entre
/// Inglaterra, Escócia, País de Gales, Irlanda do Norte e Irlanda; entre Austrália e [0x9cf3f4];
/// em [0x9cf360] com a nação [0x9cf4f0]; e em Portugal quem não é da UE (0x53d740) com contrato
/// (0x4cf5a0) que começou até 1/1/1998 (Date 0x549f70; contrato +0x25 dia, +0x27 ano).
pub(crate) unsafe extern "C" fn database_is_staff_foreign(pessoa: usize, nacao: usize, grupos: u32) -> u32 {
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let construir: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let g = |a: usize| *(a as *const u32);
    if nacao == 0 {
        return 1;
    }
    let n1 = ler(pessoa, 0x1a);
    if n1 == nacao {
        return 0;
    }
    let n2 = ler(pessoa, 0x1e);
    if n2 == nacao {
        return 0;
    }
    if grupos == 0 {
        return 1;
    }
    let id = g(nacao);
    // Inglaterra, Escócia, País de Gales, Irlanda do Norte e Irlanda
    let britanica = |x: u32| x == g(KEY_NATION_ENGLAND) || x == g(KEY_NATION_SCOTLAND) || x == g(KEY_NATION_WALES) || x == g(KEY_NATION_REPUBLIC_OF_IRELAND) || x == g(KEY_NATION_NORTHERN_IRELAND);
    if britanica(id) {
        if britanica(g(n1)) {
            return 0;
        }
        if n2 != 0 && britanica(g(n2)) {
            return 0;
        }
    }
    if n1 != 0 {
        let (x, y) = (g(KEY_NATION_AUSTRALIA), g(KEY_NATION_NEW_ZEALAND)); // Austrália e o par dela
        if (id == x || id == y) && (g(n1) == x || g(n1) == y) {
            return 0;
        }
    }
    if id == g(KEY_NATION_ITALY) && n1 != 0 && g(n1) == g(KEY_NATION_UKRAINE) {
        return 0;
    }
    if id == g(KEY_NATION_PORTUGAL) && comunitaria(pessoa, 0) == 0 {
        let contrato = contract_of(*(G_CONTRACT_MANAGER as *const usize), pessoa);
        if contrato != 0 {
            let mut d = [0u16; 4];
            let data = construir(&mut d, 1, 0, 0x7ce, 0xffff_ffff) as *const u16; // 1/1/1998
            let (dia, ano) = (core::ptr::read_unaligned(data), core::ptr::read_unaligned(data.add(1)));
            let ano_do_contrato = core::ptr::read_unaligned((contrato + 0x27) as *const u16);
            if ano > ano_do_contrato {
                return 0;
            }
            if ano == ano_do_contrato && dia as i16 > core::ptr::read_unaligned((contrato + contract::DATE_STARTED) as *const i16) {
                return 0;
            }
        }
    }
    1 // estrangeira
}




/// the person's age in whole years on a date, from the birth date
/// Original: `database_get_staff_age` (0x53ab90, Database.cpp, cdecl).
///
/// [pt] { 0x53ab90 } (cdecl, Database.cpp): a idade da `pessoa` em anos completos na `data` (palavra +0 dia
/// do ano, +2 ano): ano − ano de nascimento (+0x12), menos 1 se o dia de nascimento (+0x10) ainda não
/// chegou. Pessoa nula: caixa "Error" e 0xff (a original). Só o ax é devolvido (quem chama usa al).
pub(crate) unsafe extern "C" fn database_get_staff_age(pessoa: usize, data: *const u16) -> u16 {
    if pessoa == 0 {
        return original_fn::<unsafe extern "C" fn(usize, *const u16) -> u16>(IDX_DATABASE_GET_STAFF_AGE)(pessoa, data);
    }
    let dia_nasc = core::ptr::read_unaligned((pessoa + staff::DATE_OF_BIRTH) as *const i16) as i32;
    let anos = core::ptr::read_unaligned(data.add(1)).wrapping_sub(core::ptr::read_unaligned((pessoa + 0x12) as *const u16));
    if dia_nasc - core::ptr::read_unaligned(data as *const i16) as i32 > 0 {
        anos.wrapping_sub(1) // ainda não fez aniversário
    } else {
        anos
    }
}

/// whether the nation has free transfers: new FIFA rules, Brazil, or Europe except Poland, Turkey, Croatia and Russia
/// Original: `nation_allows_free_transfers` (0x544670, Database.cpp, cdecl).
///
/// [pt] { 0x544670 } (cdecl, Database.cpp): 1 se a `nacao` tem transferência livre: com as regras novas da
/// FIFA ([0xae233c] = 1), no Brasil ou numa nação da Europa (continente +0x71, [0x9cfa10]) fora de
/// Polônia, Turquia, Croácia e Rússia.
pub(crate) unsafe extern "C" fn nation_allows_free_transfers(nacao: usize) -> u32 {
    let g = |a: usize| *(a as *const u32);
    if *(G_FIFA_TRANSFER_RULES_IN_FORCE as *const u8) == 1 {
        return 1;
    }
    let id = g(nacao);
    if id == g(KEY_NATION_BRAZIL) {
        return 1; // Brasil
    }
    let continente = core::ptr::read_unaligned((nacao + nation::CONTINENT) as *const usize);
    if continente == 0 {
        return 0;
    }
    if id == g(KEY_NATION_POLAND) || id == g(KEY_NATION_TURKEY) || id == g(KEY_NATION_CROATIA) || id == g(KEY_NATION_RUSSIA) {
        return 0;
    }
    (g(continente) == g(KEY_CONTINENT_EUROPE)) as u32 // Europa
}

/// whether the person counts as an EU national for a club of the nation: the nationality EU mark, Switzerland and the
/// Germany, [0x9cf360] and Denmark exceptions
/// Original: `database_is_staff_eu_national` (0x53d740, Database.cpp, cdecl).
///
/// [pt] { 0x53d740 } (cdecl, Database.cpp): 1 se a `pessoa` conta como da UE para um clube da
/// `nacao_do_clube`: nacionalidade (+0x1a) ou segunda (+0x1e) com a marca de UE (nação +0x7f = 2) ou
/// Suíça ([0x9cf4ac]); na Alemanha, qualquer europeu (nação +0x71 = o continente [0x9cfa10] na tabela
/// [0xae23a4], 198 bytes); em [0x9cf360], todos; na Dinamarca, [0x9cf348], Noruega e [0x9cf3a0].
pub(crate) unsafe extern "C" fn database_is_staff_eu_national(pessoa: usize, nacao_do_clube: usize) -> u32 {
    let ler = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const usize);
    let g = |a: usize| *(a as *const u32);
    let n = ler(pessoa, 0x1a);
    if *((n + 0x7f) as *const u8) == 2 { return 1; } // nacionalidade da UE
    let id = g(n);
    if id == g(KEY_NATION_SWITZERLAND) {
        return 1;
    }
    let n2 = ler(pessoa, 0x1e);
    if n2 != 0 && *((n2 + 0x7f) as *const u8) == 2 {
        return 1;
    }
    if nacao_do_clube == 0 {
        return 0;
    }
    let c = g(nacao_do_clube);
    if c == g(KEY_NATION_GERMANY) {
        let europa = (*(G_CONTINENT_TABLE as *const usize)).wrapping_add((g(KEY_CONTINENT_EUROPE) as usize).wrapping_mul(0xc6));
        if ler(n, 0x71) == europa {
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




/// whether the person's link with the club (or the contract club) is transfer listed, by the club or at request
/// Original: `staff_is_transfer_listed_or_requested` (0x544520, Database.cpp, cdecl).
///
/// [pt] { 0x544520 } (cdecl, Database.cpp): 1 se o vínculo (0x4cf5f0) da `pessoa` com o `clube` está na
/// lista de transferências, pelo clube ou a pedido (vínculo +0x4e, bits 1 e 8). Sem clube, o do
/// contrato (0x4cf5a0, guardado em [0xae1080]; contrato +4 = id do clube na tabela [0xae23b4], 581
/// bytes por clube).
pub(crate) unsafe extern "C" fn staff_is_transfer_listed_or_requested(pessoa: usize, clube: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let contract_bond: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    if pessoa == 0 {
        return 0;
    }
    let mut clube = clube;
    if clube == 0 {
        let c = contract_of(*(G_CONTRACT_MANAGER as *const usize), pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        if c == 0 {
            return 0;
        }
        let id = core::ptr::read_unaligned((c + 4) as *const u32) as usize;
        clube = (*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(581));
        if clube == 0 {
            return 0;
        }
    }
    let v = contract_bond(*(G_CONTRACT_MANAGER as *const usize), pessoa, clube);
    (v != 0 && *((v + 0x4e) as *const u8) & 9 != 0) as u32
}

/// 1-20 rating of a relative attribute value: x = v*0.05 + 10; (x^2*0.1 + x)/3 + 0.5, clamped 1..20, truncated.
///
/// [pt] A nota de 1 a 20 de um atributo relativo (de 0x543540, 0x544380 e 0x5442f0): x = v × 0,05 + 10;
/// (x² × 0,1 + x) × 1/3 + 0,5, entre 1 e 20, truncada (`_ftol`).
pub(crate) unsafe fn attribute_rating(v: f64) -> u8 {
    let x = v * read_f64_at(0x967888) + read_f64_at(0x9688b0); // 0x967888 = 0.05, 0x9688b0 = 10
    let z = (x * x * read_f64_at(0x967880) + x) * read_f64_at(0x968a80) + read_f64_at(0x968968); // arredonda; 0x967880 = 0.1, 0x968a80 = 0.3333333333, 0x968968 = 0.5
    let (minimo, maximo) = (read_f64_at(0x967890), read_f64_at(0x968ea8)); // 0x967890 = 1, 0x968ea8 = 20
    match_pl_ftol(if z > minimo {
        if z < maximo {
            z
        } else {
            maximo
        }
    } else {
        minimo
    }) as u8
}

/// a player attribute (1 to 31) as 1 to 20: the record byte or, for relative ones, the rating of 2 × byte + current
/// ability
/// Original: `staff_compute_player_attribute` (0x543540, Database.cpp, cdecl).
///
/// [pt] { 0x543540 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador de 1 a 20. Uns são o byte do
/// registro de `jogador`; os relativos dão a nota (`attribute_rating`) de 2 × byte + habilidade atual
/// (+5), ou de 2 × byte + habilidade × 0,1 conforme o goleiro (+0xf: = 20 ou < 15); o 10 é o byte
/// +0x58 da `pessoa`; uns vão a 0x544380 (ou 0x5442f0). Sem pessoa, sem jogador ou atributo fora:
/// caixa "Error" e 0xff (a original). Só o al é devolvido.
pub(crate) unsafe extern "C" fn staff_compute_player_attribute(pessoa: usize, jogador: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) || jogador == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(IDX_STAFF_COMPUTE_PLAYER_ATTRIBUTE)(pessoa, jogador, atributo);
    }
    // simplification: a original empurra o registrador inteiro com o byte embaixo; as duas só leem o byte
    let relativo: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_PLAYER_ATTRIBUTE_RATING);
    let relativo_goleiro: unsafe extern "C" fn(usize, u32) -> u8 = core::mem::transmute(FN_PLAYER_ATTRIBUTE_RATING_WITH_SCALED_ABILITY);
    let b = |d: usize| *((jogador + d) as *const i8);
    let byte = |d: usize| b(d) as u8;
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16) as i32;
    let nota = |d: usize| attribute_rating((2 * b(d) as i32 + habilidade) as f64);
    let nota_h = |d: usize| attribute_rating(habilidade as f64 * read_f64_at(0x967880) + (2 * b(d) as i32) as f64); // 0x967880 = 0.1
    let goleiro = b(0xf);
    match a {
        1 => byte(0x1b),
        2 => byte(0x1c),
        3 => byte(0x1d),
        4 => nota(0x1e),
        5 => byte(0x1f),
        6 => byte(0x20),
        7 => if goleiro == 20 { nota_h(0x43) } else { nota(0x43) },
        8 => if goleiro == 20 { nota_h(0x23) } else { nota(0x23) },
        9 => nota(0x24),
        10 => *((pessoa + staff::DETERMINATION) as *const u8),
        11 => if goleiro == 20 { nota_h(0x26) } else { nota(0x26) },
        12 => if goleiro == 20 { relativo_goleiro(jogador, byte(0x27) as u32) } else { relativo(jogador, byte(0x27) as u32) },
        13 => byte(0x28),
        14 => if goleiro < 15 { nota_h(0x2a) } else { nota(0x2a) },
        15 => nota(0x2b),
        16 => byte(0x2f),
        17 => byte(0x2e),
        18 => relativo(jogador, byte(0x31) as u32),
        19 => if goleiro == 20 { nota_h(0x32) } else { nota(0x32) },
        20 => if goleiro == 20 { nota_h(0x33) } else { nota(0x33) },
        21 => byte(0x36),
        22 => nota(0x37),
        23 => nota(0x39),
        24 => if goleiro < 15 { nota_h(0x3a) } else { relativo(jogador, byte(0x3a) as u32) },
        25 => byte(0x29),
        26 => byte(0x3c),
        27 => byte(0x3d),
        28 => relativo(jogador, byte(0x3e) as u32),
        29 => byte(0x3f),
        30 => byte(0x40),
        _ => byte(0x44),
    }
}

/// a player attribute of the person, from the person's player data (0x543540)
/// Original: `staff_get_player_attribute` (0x5431e0, Database.cpp, cdecl).
///
/// [pt] { 0x5431e0 } (cdecl, Database.cpp): o `atributo` (1..31) de jogador da `pessoa`: 0x543540 com os
/// dados de jogador (+0x61). Sem pessoa ou atributo fora: caixa "Error" e 0xff (a original).
pub(crate) unsafe extern "C" fn staff_get_player_attribute(pessoa: usize, atributo: u32) -> u8 {
    let a = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&a) {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> u8>(IDX_STAFF_GET_PLAYER_ATTRIBUTE)(pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_COMPUTE_PLAYER_ATTRIBUTE);
    // a original empurra o argumento inteiro, como veio
    calcular(pessoa, core::ptr::read_unaligned((pessoa + staff::PLAYER) as *const usize), atributo)
}




/// save_game_data step 1: save_database(save, 1, a4, type), the database tables into the container
/// Original: `save_database_for_game` (0x524d30, Database.cpp, cdecl).
///
/// [pt] { 0x524d30 } (cdecl, Database.cpp): `save_database_for_game(save, a4, tipo)`: o passo 1 do
/// `save_game_data`: `save_database(save, 1, a4, tipo)` (0x52dad0, para o contêiner), devolvendo a
/// resposta dela. DESLIGADA (DESLIGADAS no motor.rs): o save_database grava no index.dat 408 bytes de
/// locais sem valor (lixo de ponteiro, a partir de +0x5cb); chamado desta cópia ele roda na pilha
/// particular e o lixo gravado muda (prova do save da 219a). Volta quando a cópia reproduzir o resíduo.
pub(crate) unsafe extern "C" fn save_database_for_game(save: usize, a4: u32, tipo: u32) -> u32 {
    let gravar_banco: unsafe extern "C" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SAVE_DATABASE);
    crate::comparacao::ramo("save gravar banco");
    gravar_banco(save, 1, a4, tipo)
}

// simplification: grava as tabelas do banco no contêiner; não roda duas vezes. Só a original; a cópia
// leva o ramo e é provada pela prova do save.

/// load_game_data step 2: load_database(save, shared, 1), the database read from the container
/// Original: `load_database_from_stream` (0x524d10, Database.cpp, cdecl).
///
/// [pt] { 0x524d10 } (cdecl, Database.cpp): `load_database_from_stream(save, compartilhado)`: o passo 2
/// do `load_game_data`: `load_database(save, compartilhado, 1)` (0x528540, do contêiner), devolvendo
/// a resposta dela (1 = certo).
pub(crate) unsafe extern "C" fn load_database_from_stream(save: usize, compartilhado: u32) -> u32 {
    let carregar_banco: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_LOAD_DATABASE);
    crate::comparacao::ramo("save carregar banco");
    carregar_banco(save, compartilhado, 1)
}

// simplification: carrega o banco inteiro do contêiner (aloca as tabelas); não roda duas vezes. Só a
// original; a cópia leva o ramo (a etapa de carregar da prova do save a exercita).


// simplification: destrói todos os subsistemas e apaga os *.tmp; não roda duas vezes. Só a original; a
// cópia leva o ramo e roda no modo Rust (abertura da carreira, fim do início rápido).

/// reads global 0x9b5984
/// Original: `database_global_9b5984` (0x535920, Database.cpp, cdecl).
///
/// [pt] { 0x535920 } (Database.cpp): a global 0x9b5984.
pub(crate) unsafe extern "C" fn database_global_9b5984() -> u32 {
    *(G_DATABASE_NAMES_LANGUAGE as *const u32)
}


















