// ---------- 251a leva: a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id (media.cpp, news.cpp) ----------


unsafe fn cpo6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


/// O byte com sinal (os atributos de 1 a 20).
unsafe fn cpo6_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

/// O tipo do contrato (+0x4f, nibble alto).
unsafe fn cpo6_tipo(contrato: usize) -> u8 {
    *((contrato + 0x4f) as *const u8) & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
const CPO6_NOTICIA: usize = 0xdf;

/// { 0x735870 } (thiscall, ret 0xc, media.cpp, 3,7 KB): `Media::pick_ai_manager_response(tecnico, jogador, tipo)`: a
/// resposta (1, 2 ou 3) do técnico da máquina a uma história do `tipo` sobre o jogador. Sem técnico, sem a ficha de
/// técnico (+0x69), jogador sem ficha (+0x61), o próprio técnico, sem clube (+0x39, senão a seleção +0x24) ou sem
/// contrato do jogador com ele (0x4cf5f0): 3. A nota é trunc(5 × 0x736720(jogador, clube, contrato, tipo, 3, 0)); sem
/// jogador, a palavra do argumento fica no lugar (o ponteiro do técnico, como no original). Depois, pelo tipo: 0x83
/// pelo tipo de contrato e o conhecimento (0x544410); 0x82 pelas preferências do técnico (0x547840/0x547810) e
/// sorteios; 0x1f pela lista de transferências (0x544520) e o contrato; os outros, antes, pelas preferências com a
/// nota, e depois por famílias de tipos, cada uma com a sua cadeia de sorteios (0x90cfc0); 8/0x25 e 3/4 podem pôr o
/// jogador entre os rejeitados do técnico (0x547af0).
unsafe extern "thiscall" fn resposta_do_tecnico_da_maquina(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let contrato_com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let nota_da_historia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_MEDIA_RATE_PLAYER_FORM_FOR_STORY);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let listada_pelo_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    if tecnico == 0 || read_usize(tecnico + 0x69) == 0 {
        return 3;
    }
    if jogador != 0 && read_usize(jogador + 0x61) == 0 {
        return 3;
    }
    if tecnico == jogador {
        return 3;
    }
    let mut clube = read_usize(tecnico + 0x39);
    if clube == 0 {
        clube = read_usize(tecnico + 0x24);
        if clube == 0 {
            return 3;
        }
    }
    let contrato = contrato_com_clube(read_usize(0xae1050), jogador, clube);
    if contrato == 0 {
        return 3;
    }
    let mut nota = tecnico as i32; // sem jogador: o argumento que ocupa a vaga
    if jogador != 0 {
        nota = ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * *(0x968990 as *const f32) as f64);
    }
    let f = || read_usize(tecnico + 0x69); // a ficha de técnico
    let ch = |o: usize| chance(f(), *((f() + o) as *const i8)) as i8 as i32; // o conhecimento pelo atributo +o
    let k = tipo as i16;
    if k == 0x83 {
        let t = cpo6_tipo(contrato);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && cpo6_c(contrato + 0x3f) > 1 {
            return 1;
        }
        if read_i16(read_usize(jogador + 0x61) + 0xb) < 0x1482 && ch(0x14) > 0xf {
            return 2;
        }
        if cpo6_tipo(contrato) == 0x70 && ch(0x14) > 0xa {
            return 2;
        }
        return if sorteio(ch(0x19)) != 0 { 1 } else { 3 };
    }
    if k == 0x82 {
        if nao_gosta(jogador, tecnico) != 0 {
            let a = cpo6_c(tecnico + 0x5b);
            if a < 0xf && sorteio(a) == 0 {
                let x = sorteio(ch(0x19));
                let y = sorteio(0x14);
                if x < y {
                    return 2;
                }
            }
            let a = cpo6_c(tecnico + 0x5b);
            if a <= 0xe || sorteio(a) <= 10 {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0x14);
            return if x <= y { 3 } else { 1 };
        }
        if gosta(jogador, tecnico) != 0 {
            let x = sorteio(cpo6_c(tecnico + 0x5b));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0xf);
            return if x <= y { 3 } else { 1 };
        }
        if cpo6_c(tecnico + 0x5d) < 6 {
            let y = sorteio(0x14);
            if cpo6_c(tecnico + 0x5d) < y {
                return 1;
            }
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        if x > y {
            return 1;
        }
        if cpo6_c(tecnico + 0x5c) <= 0xf {
            return 3;
        }
        let y = sorteio(10);
        return if cpo6_c(tecnico + 0x5c) <= y { 3 } else { 2 };
    }
    if k == 0x1f {
        if listada(jogador, clube) != 0 {
            return 2;
        }
        let b = cpo6_c(contrato + 0x3d);
        if b <= sorteio(100) {
            return 3;
        }
        if criterio(contrato) != 0 || *((contrato + 0x49) as *const u8) != 0 {
            return 3;
        }
        return if sorteio(3) != 0 { 1 } else { 3 };
    }
    // as preferências do técnico com a nota
    if nao_gosta(jogador, tecnico) != 0 && nota < 5 {
        if sorteio(cpo6_c(tecnico + 0x5b)) != 0 {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 2 };
    }
    if gosta(jogador, tecnico) != 0 && nota > -5 {
        let x = sorteio(cpo6_c(tecnico + 0x5b));
        let y = sorteio(10);
        if x >= y {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 1 };
    }
    // a soma do conhecimento (+0x19), de +0x1d da ficha e de +0x59 do técnico
    let soma = || ch(0x19) + cpo6_c(f() + 0x1d) + cpo6_c(tecnico + 0x59);
    match k as i32 {
        0x3c | 0x3d => {
            let x = sorteio(ch(0x1b));
            let y = sorteio(0x17);
            if x > y {
                let x = sorteio(cpo6_c(f() + 0x1d));
                let y = sorteio(10);
                if x < y {
                    return 2;
                }
            }
            if sorteio(cpo6_c(tecnico + 0x5a)) == 0 {
                return 1;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(0x28);
            if x > y {
                return 1;
            }
            let x = sorteio(cpo6_c(f() + 0x1d));
            let y = sorteio(0x32);
            if x <= y { 3 } else { 1 }
        }
        0x28..=0x3b => {
            let x = sorteio(cpo6_c(jogador + 0x5b));
            let y = sorteio(0x14);
            if x > y {
                let a = sorteio(ch(0x19));
                let b = sorteio(0x14 - nota);
                if a > b {
                    return 1;
                }
                if sorteio(ch(0x19)) != 0 {
                    return 3;
                }
                return if nota >= 0xa { 3 } else { 2 };
            }
            let a = sorteio(ch(0x19));
            let b = sorteio(10);
            if a > b {
                let mut arg = 0;
                if *((read_usize(jogador + 0x61) + 0x44) as *const u8) != 0 {
                    // o registro de 0x4f bytes da pessoa em [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| *((read_usize(0xae2d0c).wrapping_add((cpo6_i(jogador) as usize).wrapping_mul(0x4f)) + o) as *const u8);
                    let e = sorteio((2 - (reg(0xb) >> 7) as i32) * 20);
                    if sorteio(reg(0xa) as i32) > e {
                        arg = 1;
                    }
                }
                if sorteio(arg) == 0 && nota < 0xa {
                    return 2;
                }
            }
            if sorteio(cpo6_c(read_usize(jogador + 0x61) + 0x45)) == 0 {
                return 1;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(0x14 - nota);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            crate::comparacao::ramo("cpo_6 tipos comuns");
            let t = cpo6_tipo(contrato);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = entrada(read_usize(read_usize(0xde1f54) + 0x856), clube, jogador, 0, 0);
                if e != 0 && read_usize(e + 0xf) == read_usize(jogador) && nota > -10 {
                    crate::comparacao::ramo("cpo_6 sorteio descartado");
                    sorteio(cpo6_c(tecnico + 0x59)); // o original descarta o resultado
                }
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(10);
            if x < y {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y {
                        let x = sorteio(cpo6_c(tecnico + 0x59));
                        let y = sorteio(10);
                        if x < y {
                            let x = sorteio(nota);
                            let y = sorteio(-5);
                            if x < y {
                                return 2;
                            }
                        }
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x > y { 1 } else { 3 }
        }
        8 | 0x25 => {
            if sorteio(cpo6_c(f() + 0x1d) * 2) == 0 {
                rejeitar(tecnico, jogador, 1);
            }
            if ch(0x19) < 0xa {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y { 3 } else { 1 }
        }
        1 => {
            if sorteio(ch(0x19)) == 0 {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(5);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x1e);
            if x <= y { 3 } else { 1 }
        }
        0x1e | 0x20 => {
            let x = sorteio(cpo6_c(tecnico + 0x5d));
            let y = sorteio(0x14);
            if x < y {
                let x = sorteio(cpo6_c(f() + 0x1d));
                let y = sorteio(0xf);
                if x < y {
                    return 2;
                }
            }
            let x = sorteio(cpo6_c(f() + 0x1d) + cpo6_c(tecnico + 0x59));
            let y = sorteio(0x96);
            if x > y {
                if read_i16(read_usize(jogador + 0x61) + 0xb) as i32 <= read_i16(clube + 0x80) as i32 + 0x1f4 {
                    return 2;
                }
                let x = sorteio(cpo6_c(f() + 0x14));
                let y = sorteio(5);
                return if x >= y { 3 } else { 1 };
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(5);
            if x < y {
                return 1;
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0x14);
            if x <= y { 3 } else { 2 }
        }
        0x63 | 0x62 | 0x5d | 0x3f | 0x5f | 0x60 | 0x43 | 0x5c | 0x5a | 0x44 | 0x5b | 0x42 | 0x61 | 0x40 | 0x41 => {
            let mut base: i32 = match k {
                0x5d => 5,
                0x61 => 10,
                0x62 => -7,
                _ => 0,
            };
            if sorteio(cpo6_c(tecnico + 0x5a)) == 0 {
                base += sorteio(10);
                base -= sorteio(10);
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0xf - base);
            if x > y {
                return 2;
            }
            let x = sorteio(cpo6_c(tecnico + 0x57));
            let y = sorteio(base + 0xf);
            if x <= y { 3 } else { 1 }
        }
        3 | 4 => {
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0x32);
            if x > y && sorteio(cpo6_c(f() + 0x1d)) == 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x < y {
                    rejeitar(tecnico, jogador, 1);
                    return 2;
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0 && t != 0x70 && t != 0x80 && listada_pelo_clube(jogador, 0) == 0 {
                let x = sorteio(cpo6_c(tecnico + 0x59));
                let y = sorteio(6);
                if x > y {
                    let x = sorteio(ch(0x19));
                    let y = sorteio(6);
                    if x > y {
                        let x = sorteio(nota);
                        let y = sorteio(-5);
                        if x > y {
                            return 1;
                        }
                    }
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0x70 && t != 0x40 {
                return 3;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(cpo6_c(tecnico + 0x5b));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        5 => {
            if sorteio(cpo6_c(tecnico + 0x59)) != 0 && sorteio(ch(0x19)) != 0 && sorteio(cpo6_c(jogador + 0x5b)) != 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x > y {
                    return 1;
                }
            }
            if sorteio(ch(0x19)) != 0 {
                return 3;
            }
            let x = sorteio(cpo6_c(jogador + 0x5a));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        6 => {
            if sorteio(cpo6_c(tecnico + 0x59)) != 0 {
                let x = sorteio(cpo6_c(f() + 0x1f));
                let y = sorteio(10);
                if x > y {
                    let x = sorteio(cpo6_c(jogador + 0x5b));
                    let y = sorteio(5);
                    if x > y {
                        let x = sorteio(cpo6_c(read_usize(jogador + 0x61) + 0x25));
                        let y = sorteio(0x1e);
                        if x < y {
                            let x = sorteio(nota);
                            let y = sorteio(-5);
                            if x > y {
                                return 1;
                            }
                        }
                    }
                }
            }
            if sorteio(ch(0x19)) != 0 || sorteio(cpo6_c(f() + 0x1f)) != 0 {
                return 3;
            }
            let x = sorteio(cpo6_c(read_usize(jogador + 0x61) + 0x25));
            let y = sorteio(0x32);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        _ => 3,
    }
}

/// { 0x787140 } (thiscall, ret 0xc, news.cpp): `News::mark_item_read(tecnico, id, sincronizar)`: no anel de 100 vagas
/// do humano (News +8, pelo id − [0xae2364] + 16), a primeira vaga com o `id` ainda não lida (+0xda = 0) cujo botão
/// (0x782b70 em 0xde1f64) não pede resposta (2) fica lida e sai da conta (+8 da entrada). Sem anel: 0. Senão (mesmo
/// sem achar), com `sincronizar`, repete (sem sincronizar) para os outros humanos (+0x5f = 1) da mesma máquina da rede
/// (0x7fffa0) que o humano da vez; devolve 1. Sem técnico, sem id ou técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2358);
        return 0;
    }
    if id == 0 {
        aviso_em(0xa693ec, 0x2360);
        return 0;
    }
    let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, 0x236b);
        return 0;
    }
    let e = read_usize(this + 8) + idx as usize * 12;
    if cpo6_i(e) == -1 {
        return 0;
    }
    let botao: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8 = core::mem::transmute(FN_NEWSITEM_GET_ACTION_BUTTON);
    let mut v = 0usize;
    for k in 0..0x64usize {
        v = read_usize(this).wrapping_add((read_usize(e).wrapping_add(k)).wrapping_mul(CPO6_NOTICIA));
        if read_usize(v + 0xd6) as u32 == id && read_usize(v + 0xda) == 0 {
            let mut tratador = 0u32;
            if botao(v, tecnico, 0xde1f64, 0x7d0, &mut tratador) != 2 {
                crate::comparacao::ramo("cpo_6 noticia lida");
                core::ptr::write_unaligned((v + 0xda) as *mut u32, 1);
                core::ptr::write_unaligned((e + 8) as *mut i32, cpo6_i(e + 8) - 1);
                break;
            }
        }
    }
    if v == 0 {
        return 0;
    }
    if sincronizar != 0 {
        let maquina: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_FIND_MANAGER_CHANNEL);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_MARK_ITEM_READ);
        let da_vez = read_usize(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize);
        let m = maquina(0xb74450, da_vez) as u16;
        for j in 0..0x10i32 {
            let p = read_usize(0xae23bc).wrapping_add((cpo6_i(0xae2364) - j - 1).wrapping_mul(0x6e) as usize);
            if *((p + 0x5f) as *const u8) == 1 && p != tecnico && maquina(0xb74450, p) as u16 == m {
                marcar(this, p, id, 0);
            }
        }
    }
    1
}

/// { 0x786b70 } (thiscall, ret 0xc, news.cpp): `News::find_item_by_id(tecnico, id, saida)`: copia (0x786010) em `saida`
/// a notícia com o `id`: com técnico, entre as 100 vagas do anel dele (sem anel: 0); sem técnico, entre todas as vagas
/// (News +4). Achou: 1; senão 0. Sem id, sem saída ou técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    if id == 0 {
        aviso_em(0xa693ec, 0x2252);
        return 0;
    }
    if saida == 0 {
        aviso_em(0xa693ec, 0x225a);
        return 0;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_NEWSITEM_OPERATOR_3D);
    let (inicio, n) = if tecnico == 0 {
        (0usize, cpo6_i(this + 4).max(0) as usize)
    } else {
        let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
        if idx < 0 {
            aviso_em(0xa693ec, 0x2268);
            return 0;
        }
        let ini = cpo6_i(read_usize(this + 8) + idx as usize * 12);
        if ini == -1 {
            return 0;
        }
        (ini as usize, 0x64)
    };
    for k in 0..n {
        let v = read_usize(this).wrapping_add(inicio.wrapping_add(k).wrapping_mul(CPO6_NOTICIA));
        if read_usize(v + 0xd6) as u32 == id {
            copiar(saida, v);
            return 1;
        }
    }
    0
}

/// Comparador: a resposta sorteia e pode pôr o jogador entre os rejeitados do técnico (0x547af0): lado a lado no
/// estado do gerador e no bloco das preferências do técnico (+0x65, +0x1c..+0x34), a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_resposta_do_tecnico_da_maquina(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(982);
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if tecnico != 0 && read_usize(tecnico + 0x65) != 0 {
        regioes.push((read_usize(tecnico + 0x65) + 0x1c, 0x18));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, tecnico, jogador, tipo),
        || crate::aleatorio::sem_contar(|| resposta_do_tecnico_da_maquina(this, tecnico, jogador, tipo)),
    );
    PLACAR.registrar(igual && r == n, || format!("resposta_do_tecnico_da_maquina(tipo {:#x}) original={} rust={} {:x?} x {:x?}", tipo as i16, r, n, mo.0, mr.0));
    r
}

/// Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(983);
    if tecnico == 0 || id == 0 || cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0 || read_usize(this) == 0 {
        return o(this, tecnico, id, sincronizar);
    }
    let regioes = [
        (read_usize(this), cpo6_i(this + 4).max(0) as usize * CPO6_NOTICIA),
        (read_usize(this + 8), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico, id, sincronizar), || marcar_noticia_lida(this, tecnico, id, sincronizar));
    PLACAR.registrar(igual && r == n, || format!("marcar_noticia_lida(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(984);
    if id == 0 || saida == 0 || (tecnico != 0 && cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0) {
        return o(this, tecnico, id, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, CPO6_NOTICIA)], || o(this, tecnico, id, saida), || achar_noticia_pelo_id(this, tecnico, id, saida));
    PLACAR.registrar(igual && r == n, || format!("achar_noticia_pelo_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}

// ---------- 250a leva: a entrega das notícias: pelo alcance, ao técnico do clube e na caixa do técnico humano (news.cpp) ----------


unsafe fn cpn1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpn1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +0xd6 id, +0xda livre/lida, +0xde alcance).
const CPN1_NOTICIA: usize = 0xdf;

/// A entrada do técnico humano na tabela das caixas (News +8, 12 bytes: início do anel, a próxima vaga, a conta):
/// índice = id da pessoa − [0xae2364] + 16 (os humanos são as 16 últimas pessoas). Negativo: não é humano.
unsafe fn cpn1_indice_humano(tecnico: usize) -> i32 {
    cpn1_i(tecnico).wrapping_sub(cpn1_i(0xae2364)).wrapping_add(0x10)
}

/// { 0x786090 } (thiscall, ret 0x18, news.cpp): `News::send_by_scope(noticia, clube, competicao, nacao, continente,
/// alcance)`: o alcance (char) escolhe a entrega: 0 o técnico do clube (0x7861e0), 1 os da divisão (0x7862d0, pela
/// competição), 2 os do país (0x7863f0, pela nação), 3 os do continente (0x786640), 4 todos (0x786780), 5 os da região
/// da competição (0x786860). Fora disso, a caixa "Error" e 0. Devolve o que a entrega devolve.
unsafe extern "thiscall" fn noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let dois: unsafe extern "thiscall" fn(usize, usize, usize) -> u32;
    let alvo = match alcance as u8 as i8 {
        0 => {
            crate::comparacao::ramo("cpn_1 alcance do clube");
            dois = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
            clube // o técnico do clube
        }
        1 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
            competicao
        }
        2 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
            nacao
        }
        3 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
            continente
        }
        4 => {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
            return todos(this, noticia);
        }
        5 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_COMP_REGION);
            competicao
        }
        _ => {
            aviso_em(0xa693ec, 0x1fd3);
            return 0;
        }
    };
    dois(this, noticia, alvo)
}

/// { 0x7861e0 } (thiscall, ret 8, news.cpp): `News::send_to_club_manager(noticia, clube)`: marca o alcance da notícia
/// como clube (+0xde = 0) e a entrega (0x785d40) ao técnico do clube (+0xcf) se ele for humano (id entre as 16 últimas
/// pessoas). Sem notícia ou sem clube: a caixa "Error" e 0; sem técnico ou técnico da máquina: 0.
unsafe extern "thiscall" fn noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x2026);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa693ec, 0x202e);
        return 0;
    }
    let tecnico = read_usize(clube + 0xcf);
    *((noticia + 0xde) as *mut u8) = 0;
    if tecnico == 0 {
        return 0;
    }
    if cpn1_i(tecnico) < cpn1_i(0xae2364).wrapping_sub(0x10) { // a máquina
        return 0;
    }
    crate::comparacao::ramo("cpn_1 noticia ao tecnico humano");
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    entregar(this, noticia, tecnico, 0)
}

/// { 0x785d40 } (thiscall, ret 0xc, news.cpp): `News::deliver_to_manager(noticia, tecnico, _)`: grava se a notícia é
/// importante para ele (+4, 0x784eb0) e zera +0xda; acha a entrada do humano (News +8); na primeira notícia dele, o anel
/// de 100 vagas no fim do vetor das notícias (News +0, conta de vagas em +4, realloc 0x945667; sem memória, caixa e 0)
/// com as vagas zeradas (+0xd6 = 0, +0xda = 1). Notícia sem id (+0xd6) recebe o próximo (News +0x10). Se a vaga anterior
/// tem o mesmo id, não repete (devolve 1). Senão copia a notícia (0x786010) na próxima vaga (a vaga ainda não lida, +0xda
/// = 0, sai da conta), avança o anel e, com todos de férias ([0xdd6bec]) ou o técnico de férias (0x5fd210), dá a
/// resposta automática (0x784100) pelo clube (+0x39) e pela seleção (+0x24). Soma 1 na conta e devolve 1.
unsafe extern "thiscall" fn entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, _livre: u32) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x1f3f);
        return 0;
    }
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x1f47);
        return 0;
    }
    let importante: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWSITEM_IS_IMPORTANT_FOR);
    *((noticia + 4) as *mut u8) = importante(noticia, tecnico) as u8;
    cpn1_por(noticia + 0xda, 0);
    let idx = cpn1_indice_humano(tecnico);
    if idx < 0 {
        aviso_em(0xa693ec, 0x1f58);
        return 0;
    }
    let e = read_usize(this + 8) + idx as usize * 12;
    if cpn1_i(e) == -1 {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        cpn1_por(e, read_usize(this + 4));
        cpn1_por(e + 4, read_usize(this + 4));
        let total = read_usize(this + 4).wrapping_add(0x64);
        cpn1_por(this + 4, total);
        let p = realocar(read_usize(this), total.wrapping_mul(CPN1_NOTICIA) as u32);
        cpn1_por(this, p);
        if p == 0 {
            aviso_em(0xa693ec, 0x1f6d);
            return 0;
        }
        for k in 0..0x64usize {
            let v = read_usize(this) + (read_usize(e) + k).wrapping_mul(CPN1_NOTICIA);
            cpn1_por(v + 0xd6, 0);
            cpn1_por(v + 0xda, 1);
        }
    }
    if read_usize(noticia + 0xd6) == 0 {
        let id = read_usize(this + 0x10).wrapping_add(1);
        cpn1_por(this + 0x10, id);
        cpn1_por(noticia + 0xd6, id);
    }
    let atual = cpn1_i(e + 4);
    let mut anterior = atual.wrapping_sub(1);
    if anterior < cpn1_i(e) {
        anterior = cpn1_i(e) + 0x63;
    }
    let vaga = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
    if read_usize(vaga(anterior) + 0xd6) == read_usize(noticia + 0xd6) {
        return 1; // a mesma notícia de novo
    }
    if read_usize(vaga(atual) + 0xda) == 0 {
        cpn1_por(e + 8, read_usize(e + 8).wrapping_sub(1));
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWSITEM_OPERATOR_3D);
    copiar(vaga(atual), noticia);
    let atual = cpn1_i(e + 4);
    let inicio = cpn1_i(e);
    cpn1_por(e + 4, atual.wrapping_add(1) as usize);
    if atual.wrapping_add(1) == inicio.wrapping_add(0x64) {
        cpn1_por(e + 4, inicio as usize);
    }
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    if *(0xdd6bec as *const i16) != 0 || ferias(read_usize(0xb63c98), tecnico) != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWSITEM_AUTO_RESPOND);
        crate::comparacao::ramo("cpn_1 resposta automatica");
        responder(vaga(atual), tecnico, read_usize(tecnico + 0x39)); // o clube
        let selecao = read_usize(tecnico + 0x24);
        if selecao != 0 {
            responder(vaga(atual), tecnico, selecao);
        }
    }
    cpn1_por(e + 8, read_usize(e + 8).wrapping_add(1));
    1
}

/// Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
unsafe extern "thiscall" fn comparar_noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(946)(this, noticia, clube, competicao, nacao, continente, alcance);
    let fora = !(0..=5).contains(&(alcance as u8 as i8));
    PLACAR.registrar(!fora || r == 0, || format!("noticia_pelo_alcance alcance {} devolveu {}", alcance as u8 as i8, r));
    r
}

/// Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
unsafe extern "thiscall" fn comparar_noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(947);
    if noticia == 0 || clube == 0 {
        return o(this, noticia, clube);
    }
    let tecnico = read_usize(clube + 0xcf);
    let humano = tecnico != 0 && cpn1_i(tecnico) >= cpn1_i(0xae2364).wrapping_sub(0x10);
    let r = o(this, noticia, clube);
    let ok = *((noticia + 0xde) as *const u8) == 0 && (humano || r == 0) && r <= 1;
    PLACAR.registrar(ok, || format!("noticia_ao_tecnico_do_clube(clube {:x}, técnico {:x}, humano {}) devolveu {}", clube, tecnico, humano, r));
    r
}

/// Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
unsafe extern "thiscall" fn comparar_entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, livre: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(948);
    if noticia == 0 || tecnico == 0 || cpn1_indice_humano(tecnico) < 0 {
        return o(this, noticia, tecnico, livre);
    }
    let idx = cpn1_indice_humano(tecnico) as usize;
    let e = read_usize(this + 8) + idx * 12;
    // o plano
    let novo = cpn1_i(e) == -1;
    let (inicio, atual, conta) = if novo { (cpn1_i(this + 4), cpn1_i(this + 4), cpn1_i(e + 8)) } else { (cpn1_i(e), cpn1_i(e + 4), cpn1_i(e + 8)) };
    let contador = read_usize(this + 0x10);
    let (id, contador_depois) = if read_usize(noticia + 0xd6) == 0 { (contador.wrapping_add(1), contador.wrapping_add(1)) } else { (read_usize(noticia + 0xd6), contador) };
    let mut anterior = atual - 1;
    if anterior < inicio {
        anterior = inicio + 0x63;
    }
    let (repetida, livre_atual) = if novo {
        (id == 0, true)
    } else {
        let v = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
        (read_usize(v(anterior) + 0xd6) == id, read_usize(v(atual) + 0xda) != 0)
    };
    let (prox, conta_depois) = if repetida {
        (atual, conta)
    } else {
        let p = if atual + 1 == inicio + 0x64 { inicio } else { atual + 1 };
        (p, conta + if livre_atual { 1 } else { 0 })
    };
    // com o técnico de férias a resposta automática (0x784100) pode entregar outra notícia ao mesmo humano: o anel anda
    // mais vagas e a conta junto (achado em 31/10/2026, quando a rotina saiu das DESLIGADAS)
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let responde = !repetida && (*(0xdd6bec as *const i16) != 0 || ferias(read_usize(0xb63c98), tecnico) != 0);
    let r = o(this, noticia, tecnico, livre);
    if r == 0 {
        PLACAR.registrar(novo, || format!("entregar_noticia_ao_tecnico devolveu 0 sem anel novo")); // só sem memória
        return r;
    }
    let extra = if responde { (cpn1_i(e + 4) - prox).rem_euclid(0x64) } else { 0 };
    let vaga = read_usize(this).wrapping_add((atual as usize).wrapping_mul(CPN1_NOTICIA));
    let ok = r == 1 && cpn1_i(e) == inicio && cpn1_i(e + 4) == if extra == 0 { prox } else { cpn1_i(e + 4) }
        && cpn1_i(e + 8) >= conta_depois && cpn1_i(e + 8) <= conta_depois + extra
        && (read_usize(this + 0x10) == contador_depois || (extra > 0 && read_usize(this + 0x10) > contador_depois))
        && read_usize(noticia + 0xd6) == id && read_usize(noticia + 0xda) == 0
        && (repetida || read_usize(vaga + 0xd6) == id);
    PLACAR.registrar(ok, || format!("entregar_noticia_ao_tecnico(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, read_usize(noticia + 0xd6), id, cpn1_i(e), cpn1_i(e + 4), cpn1_i(e + 8), inicio, prox, conta_depois, repetida));
    r
}

// ---------- 250a leva: a caixa do técnico humano: responder a todas as notícias, se há notícia pendente e o texto num campo da notícia (news.cpp) ----------


unsafe fn cpn2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +5 os 50 campos de 4 bytes, +0xcc fim do texto, +0xd6 id,
/// +0xda lida).
const CPN2_NOTICIA: usize = 0xdf;

/// A entrada do humano na tabela das caixas (News +8, 12 bytes: início do anel, próxima vaga, conta), ou None (com a
/// caixa "Error" da `linha`) se a pessoa não está entre as 16 últimas.
unsafe fn cpn2_entrada(this: usize, tecnico: usize, linha: u32) -> Option<usize> {
    let idx = cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, linha);
        return None;
    }
    Some(read_usize(this + 8) + idx as usize * 12)
}

/// A vaga anterior a `k` no anel que começa em `inicio` (100 vagas).
fn cpn2_volta(k: i32, inicio: i32) -> i32 {
    if k < inicio { inicio + 0x63 } else { k }
}

/// { 0x7874b0 } (thiscall, ret 8, news.cpp): `News::auto_respond_all(tecnico, clube)`: da notícia mais nova para a mais
/// antiga do anel do humano, até a primeira vaga vazia (+0xd6 = 0), dá a resposta automática (0x784100) pelo `clube`.
/// Sem técnico ou técnico da máquina: a caixa "Error" e 0; sem anel ou no fim: 1. Chamada ao sair de férias, ao ser
/// demitido e ao ser contratado.
unsafe extern "thiscall" fn responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2460);
        return 0;
    }
    let e = match cpn2_entrada(this, tecnico, 0x246b) {
        Some(e) => e,
        None => return 0,
    };
    if cpn2_i(e) == -1 {
        return 1;
    }
    let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWSITEM_AUTO_RESPOND);
    let mut k = cpn2_i(e + 4) - 1;
    loop {
        k = cpn2_volta(k, cpn2_i(e));
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if read_usize(vaga + 0xd6) == 0 {
            return 1;
        }
        crate::comparacao::ramo("cpn_2 responder todas");
        responder(vaga, tecnico, clube); // a resposta automática
        k -= 1;
        if k == cpn2_i(e + 4) - 1 {
            return 1;
        }
    }
}

/// { 0x787350 } (thiscall, ret 8, news.cpp): `News::has_urgent_items(tecnico, so_urgentes)`: sem `so_urgentes`, se há
/// notícia não lida (0x786fe0 > 0). Com ele, da mais nova para a mais antiga do anel do humano, até a primeira vaga vazia:
/// 1 se uma pede resposta (o texto 0x782b70 em 0xde1f64 devolve 2) ou é importante (+4 = 1) e não lida (+0xda = 0).
/// Técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    if so_urgentes == 0 {
        let nao_lidas: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_NEWS_COUNT_UNREAD);
        return (nao_lidas(this, tecnico, 0) > 0) as u32;
    }
    let e = match cpn2_entrada(this, tecnico, 0x241e) {
        Some(e) => e,
        None => return 0,
    };
    if cpn2_i(e) == -1 {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_ACTION_BUTTON);
    let mut k = cpn2_i(e + 4) - 1;
    loop {
        k = cpn2_volta(k, cpn2_i(e));
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if read_usize(vaga + 0xd6) == 0 {
            return 0;
        }
        let mut saida = 0u32;
        if texto(vaga, tecnico, 0xde1f64, 0x7d0, &mut saida) as u8 == 2 {
            return 1; // pede resposta
        }
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if *((vaga + 4) as *const u8) == 1 && read_usize(vaga + 0xda) == 0 {
            return 1; // importante e não lida
        }
        k -= 1;
        if k == cpn2_i(e + 4) - 1 {
            return 0;
        }
    }
}

/// { 0x7858c0 } (thiscall, ret 0xc, news.cpp): `NewsItem::set_slot_string(campo, texto, proximo)`: copia o `texto` nos
/// campos da notícia a partir do `campo` (strncpy 0x9466d0 em +5 + 4·campo, até 200 − 4·campo bytes), termina a área
/// (+0xcc = 0) e, com `proximo`, grava o primeiro campo livre depois do texto (o tamanho com o zero arredondado para
/// 4, em campos, mais o `campo`; no máximo 50). Campo fora de 0..49 ou sem texto: a caixa "Error" e 0; senão 1.
unsafe extern "thiscall" fn gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let campo = campo as u8 as i8;
    if !(0..0x32).contains(&campo) {
        aviso_em(0xa693ec, 0x1e8d);
        return 0;
    }
    if texto.is_null() {
        aviso_em(0xa693ec, 0x1e95);
        return 0;
    }
    let copiar: unsafe extern "C" fn(usize, *const u8, u32) -> usize = core::mem::transmute(FN_9466D0);
    let d = 4 * campo as i32;
    copiar(this + 5 + d as usize, texto, (0xc8 - d) as u32);
    *((this + 0xcc) as *mut u8) = 0;
    if !proximo.is_null() {
        let mut n = 0i32;
        while *texto.add(n as usize) != 0 {
            n += 1;
        }
        let quartos = ((n + 1 + 3) & !3) / 4; // o tamanho com o zero, em campos de 4 bytes
        let mut p = (quartos as u8).wrapping_add(campo as u8);
        crate::comparacao::ramo("cpn_2 proximo campo do texto");
        if p as i8 > 0x32 {
            p = 0x32;
        }
        *proximo = p; // o próximo campo livre
    }
    1
}

/// Comparador: a original responde (convites, propostas, emprego): roda só ela e confere a devolução (0 só sem técnico
/// ou com técnico da máquina; senão 1).
unsafe extern "thiscall" fn comparar_responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    let humano = tecnico != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) >= 0;
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(949)(this, tecnico, clube);
    PLACAR.registrar(r == humano as u32, || format!("responder_todas_as_noticias(técnico {:x}, humano {}) devolveu {}", tecnico, humano, r));
    r
}

/// Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
unsafe extern "thiscall" fn comparar_tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(950);
    if so_urgentes != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico, so_urgentes), || tecnico_tem_noticia_pendente(this, tecnico, so_urgentes));
    PLACAR.registrar(igual && r == n, || format!("tecnico_tem_noticia_pendente(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
unsafe extern "thiscall" fn comparar_gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(951);
    if !(0..0x32).contains(&(campo as u8 as i8)) || texto.is_null() {
        return o(this, campo, texto, proximo); // a caixa "Error"
    }
    let mut reg = vec![(this + 5, 0xc8usize)];
    if !proximo.is_null() {
        reg.push((proximo as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, campo, texto, proximo), || gravar_texto_na_noticia(this, campo, texto, proximo));
    PLACAR.registrar(igual && r == n, || format!("gravar_texto_na_noticia(campo {}) {:x?} x {:x?}", campo as u8 as i8, mo, mr));
    r
}

// ---------- 247a leva: as notícias para os técnicos humanos de um continente e da região da competição, e se a notícia é importante para a pessoa ----------


/// { 0x786640 } (thiscall, ret 8, news.cpp): `News::send_to_continent_managers(noticia, continente)`: alcance do
/// continente (+0xde = 3) e a entrega (0x785d40 com 0) a cada técnico humano ativo cujo clube (+0x39) tem a nação
/// (+0x53) no `continente` (+0x71) ou cuja nação (+0x1a) está nele. Devolve 1, ou 0 na primeira recusada. Notícia ou
/// continente nulos: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    if noticia == 0 || continente == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(895)(this, noticia, continente); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 3, |p| {
        let c = read_usize(p + 0x39);
        if c != 0 {
            let n = read_usize(c + 0x53);
            if n != 0 && read_usize(n + 0x71) == continente { // o continente da nação do clube
                crate::comparacao::ramo("cpk_5 continente pelo clube");
                return true;
            }
        }
        let s = read_usize(p + 0x1a);
        s != 0 && read_usize(s + 0x71) == continente // o continente da nação da pessoa
    }) as u32
}

/// { 0x786860 } (thiscall, ret 8, news.cpp): `News::send_to_comp_region(noticia, comp)`: com a nação da competição
/// (+0x5d), para os técnicos da nação (0x786520); senão, com o continente (+0x59), para os do continente (0x786640);
/// senão para todos (0x786780). Devolve o que a entrega devolve. Notícia ou competição nulas: a original (caixa
/// "Error").
unsafe extern "thiscall" fn noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    if noticia == 0 || comp == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(896)(this, noticia, comp); // caixa: só a original
    }
    let nacao = read_usize(comp + 0x5d);
    if nacao != 0 {
        crate::comparacao::ramo("cpk_5 noticia da nacao da competicao");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_CLUB_MANAGERS);
        return f(this, noticia, nacao);
    }
    let continente = read_usize(comp + 0x59);
    if continente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
        return f(this, noticia, continente);
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    f(this, noticia)
}

/// O técnico (+0xcf) do clube cujo id está no dado da notícia em `o` ([0xae23b4], 0x245 bytes por clube).
unsafe fn cpk5_tecnico_do_clube(noticia: usize, o: usize) -> usize {
    read_usize(read_usize(0xae23b4).wrapping_add(read_usize(noticia + o).wrapping_mul(0x245)) + 0xcf)
}

/// { 0x784eb0 } (thiscall, ret 4, news.cpp): `NewsItem::is_important_for(pessoa)`: a notícia é importante para a
/// pessoa (o byte +4 que a entrega 0x785d40 grava)? Pela categoria (0x785670) e o tipo (+0): categoria 2, os tipos 7 a
/// 0xb, 0x27 e 0x28; categoria 4, 0xbb8, 0xbb9 e 0xbc4; categoria 5, uma lista fixa, 0xfa1 se a pessoa é o técnico do
/// clube do dado 5 (+0x19) e 0xfb6 se é o do dado 4 (+0x15) ou do 5; categoria 6, 0x1770 se o dado 0 (+5) é um dos
/// 16 técnicos humanos; categoria 1, 0x1388; o resto, 0. A original chama a 0x785670 até seis vezes (só lê): a cópia,
/// uma. Pessoa nula ou tipo fora de 1..0x1790 (caixa da 0x785670): a original.
unsafe extern "thiscall" fn noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let t = read_usize(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(897)(noticia, pessoa); // caixa: só a original
    }
    let categoria: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
    let r = match categoria(noticia) as u8 {
        2 => matches!(t, 7..=0xb | 0x27 | 0x28),
        4 => matches!(t, 0xbb8 | 0xbb9 | 0xbc4),
        5 => match t {
            0xfa0 | 0xfa4 | 0xfad | 0xfae | 0xfaf | 0xfb0 | 0xfb2 | 0xfb4 | 0xfb7 | 0xfb8 | 0xfbb | 0xfbf | 0xfc0 => true,
            0xfa1 => {
                crate::comparacao::ramo("cpk_5 importante oferta pelo clube");
                cpk5_tecnico_do_clube(noticia, 0x19) == pessoa // o técnico do clube da oferta
            }
            0xfb6 => cpk5_tecnico_do_clube(noticia, 0x15) == pessoa || cpk5_tecnico_do_clube(noticia, 0x19) == pessoa,
            _ => false,
        },
        6 => t == 0x1770 && read_usize(noticia + 5) as i32 >= (read_usize(0xae2364) as i32).wrapping_sub(0x10),
        1 => t == 0x1388,
        _ => false, // 3 e 7: nenhum
    };
    r as u8
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_continente");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(895)(this, noticia, continente)
}

unsafe extern "thiscall" fn comparar_noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_a_regiao_da_competicao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(896)(this, noticia, comp)
}

/// Comparador: pelo valor (a 0x785670 só lê).
unsafe extern "thiscall" fn comparar_noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(897)(noticia, pessoa);
    let t = read_usize(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = noticia_importante_para_a_pessoa(noticia, pessoa);
    PLACAR.registrar(r == n, || format!("noticia_importante_para_a_pessoa(tipo {:#x}, {:x}) original={} rust={}", t, pessoa, r, n));
    r
}

// ---------- 247a leva: o texto de um dado da notícia, a notícia do clube em apuro financeiro e se os olheiros do clube cobrem a nação ----------



/// { 0x785a20 } (thiscall, ret 0xc, news.cpp): `NewsItem::get_slot_string(dado, destino, n)`: copia o texto guardado a
/// partir do dado `dado` (+5 + 4 × dado) para `destino` como o strncpy (0x9466d0: até o zero, completando com zeros
/// até `n`) e zera o último byte (`destino[n − 1]`). Devolve 1. Dado fora de 0..49, destino nulo ou `n` < 1: a
/// original (caixa "Error" e 0).
unsafe extern "thiscall" fn texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(898)(noticia, dado, destino, n); // caixa: só a original
    }
    crate::comparacao::ramo("cpk_6 texto do dado");
    let origem = (noticia + 5 + 4 * k as usize) as *const u8; // o dado
    let mut fim = false;
    for i in 0..n as usize {
        let c = if fim { 0 } else { *origem.add(i) };
        fim |= c == 0;
        *destino.add(i) = c;
    }
    *destino.add(n as usize - 1) = 0;
    1
}

/// { 0x74ae60 } (thiscall, ret 4, media.cpp, quadro SEH sem destrutor): `Media::post_financial_trouble_news(clube)`:
/// com o clube, a nação (+0x53), a ficha (+0x69, com +0x38) e a divisão (+0x57), monta a notícia 0x1790 numa local
/// (0x77bbe0 com 0) com os dados 0 = id do clube, 1 = reputação (+0x80) e 2 = sorteio(2) (0x90cfc0); clube de
/// reputação 9000 ou mais numa divisão com +0x69 acima de 12: para todos os técnicos (0x786780); senão para os da
/// nação (0x7863f0). A notícia local tem 0xe0 bytes (a entrega copia só até +0xde).
unsafe extern "thiscall" fn noticia_de_apuro_financeiro(_this: usize, clube: usize) {
    if clube == 0 || read_usize(clube + 0x53) == 0 {
        return;
    }
    let ficha = read_usize(clube + 0x69);
    if ficha == 0 || read_usize(ficha + 0x38) == 0 || read_usize(clube + 0x57) == 0 {
        return;
    }
    crate::comparacao::ramo("cpk_6 apuro financeiro");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    construir(n, 0x1790, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, read_i16(clube + 0x80) as i32 as u32);
    let v = sorteio(2); // o sorteio da notícia
    gravar(n, 2, v as u32);
    let noticias = read_usize(0xb7431c);
    let divisao = read_usize(clube + 0x57);
    if read_i16(clube + 0x80) >= 9000 && divisao != 0 && read_i16(divisao + 0x69) > 12 {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        todos(noticias, n);
    } else {
        let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
        pais(noticias, n, read_usize(clube + 0x53));
    }
}

/// O clube que a 0x7f9190 usa: o `clube`, ou o do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]] + 0x39).
unsafe fn cpk6_clube(clube: usize) -> usize {
    if clube != 0 {
        return clube;
    }
    read_usize(read_usize(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize) + 0x39)
}

/// { 0x7f9190 } (thiscall, ret 8, scout_manager.cpp; `this` não é lido): `SCOUT_MANAGER::can_club_scout_nation(nacao,
/// clube)`: o clube (ou o do humano da vez) não cobre a nação só se tem as finanças (0x5a0590 em [0xae2a38]) com saldo
/// de 64 bits abaixo de 1.000.000, reputação (+0x80) abaixo de 5000 e a região (+0x76) da nação dele (+0x53) diferente
/// da região da `nacao`: 0. Senão 1. Sem clube nenhum: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn olheiros_cobrem_a_nacao(this: usize, nacao: usize, clube: usize) -> u32 {
    let c = cpk6_clube(clube);
    if c == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(900)(this, nacao, clube); // caixa: só a original
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = financas_do_clube(read_usize(0xae2a38), c);
    if f != 0 && core::ptr::read_unaligned(f as *const i64) < 1_000_000 && read_i16(c + 0x80) < 5000 {
        if *((read_usize(c + 0x53) + 0x76) as *const u8) != *((nacao + 0x76) as *const u8) { // fora da região
            crate::comparacao::ramo("cpk_6 olheiro fora da regiao");
            return 0;
        }
    }
    1
}

/// Comparador: lado a lado pelos `n` bytes do destino.
unsafe extern "thiscall" fn comparar_texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(898);
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return o(noticia, dado, destino, n);
    }
    let reg = [(destino as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(noticia, dado, destino, n), || texto_do_dado_da_noticia(noticia, dado, destino, n));
    PLACAR.registrar(igual && r == m, || format!("texto_do_dado_da_noticia({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}

/// Comparador: a notícia sorteia (0x90cfc0) e vai para as caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_de_apuro_financeiro(this: usize, clube: usize) {
    crate::comparacao::ramo("noticia_de_apuro_financeiro");
    orig::<unsafe extern "thiscall" fn(usize, usize)>(899)(this, clube)
}

/// Comparador: pelo valor (a 0x5a0590 só procura).
unsafe extern "thiscall" fn comparar_olheiros_cobrem_a_nacao(this: usize, nacao: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(900)(this, nacao, clube);
    if cpk6_clube(clube) == 0 {
        return r;
    }
    let n = olheiros_cobrem_a_nacao(this, nacao, clube);
    PLACAR.registrar(r == n, || format!("olheiros_cobrem_a_nacao({:x}, {:x}) original={} rust={}", nacao, clube, r, n));
    r
}

// ---------- 246a leva: as notícias para os técnicos humanos: para todos, para os da divisão e para os da nação (clube ou seleção) ----------


/// O técnico humano `i` (0 a 15: os últimos 16 de [0xae23bc], 0x6e bytes), se ativo (+0x5f = 1).
unsafe fn cpj3_humano_ativo(i: usize) -> Option<usize> {
    let p = read_usize(0xae23bc).wrapping_add(read_usize(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
    (*((p + 0x5f) as *const u8) == 1).then_some(p)
}

/// Entrega a notícia a cada técnico humano ativo que `passa` (0x785d40 com 0), com o alcance `alcance` em +0xde.
/// Devolve se todas as entregas foram aceitas (a primeira recusada encerra).
unsafe fn cpj3_entregar(this: usize, noticia: usize, alcance: u8, passa: impl Fn(usize) -> bool) -> bool {
    *((noticia + 0xde) as *mut u8) = alcance;
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    for i in 0..0x10 {
        if let Some(p) = cpj3_humano_ativo(i) {
            if passa(p) && entregar(this, noticia, p, 0) == 0 {
                return false; // a entrega recusada encerra
            }
        }
    }
    true
}

/// { 0x786780 } (thiscall, ret 4, news.cpp): `News::send_to_all_managers(noticia)`: alcance de todos (+0xde = 4) e a
/// entrega (0x785d40) a cada técnico humano ativo. Devolve 1, ou 0 na primeira recusada. Notícia nula: a original
/// (caixa "Error").
unsafe extern "thiscall" fn noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    if noticia == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(870)(this, noticia); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 4, |_| true) as u32
}

/// { 0x7862d0 } (thiscall, ret 8, news.cpp): `News::send_to_division_managers(noticia, divisao)`: alcance da divisão
/// (+0xde = 1) e a entrega a cada técnico humano ativo com clube (+0x39) na `divisao` (+0x57). Devolve 1, ou 0 na
/// primeira recusada. Notícia ou divisão nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    if noticia == 0 || divisao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(871)(this, noticia, divisao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 1, |p| {
        let c = read_usize(p + 0x39);
        c != 0 && read_usize(c + 0x57) == divisao // a divisão do clube
    }) as u32
}

/// { 0x7863f0 } (thiscall, ret 8, news.cpp): `News::send_to_nation_managers(noticia, nacao)`: alcance da nação (+0xde
/// = 2) e a entrega a cada técnico humano ativo cujo clube (+0x39) ou seleção (+0x24) é da `nacao` (+0x53). Devolve 0.
/// Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(872)(this, noticia, nacao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 2, |p| {
        let (c, s) = (read_usize(p + 0x39), read_usize(p + 0x24));
        (c != 0 && read_usize(c + 0x53) == nacao) || (s != 0 && read_usize(s + 0x53) == nacao) // o clube ou a seleção
    });
    0
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_todos_os_tecnicos");
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(870)(this, noticia)
}

unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_divisao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(871)(this, noticia, divisao)
}

unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_pais");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(872)(this, noticia, nacao)
}

// ---------- 245a leva: a notícia para os técnicos humanos de uma nação, a missão do olheiro da busca e se a copa terminou ----------


/// { 0x786520 } (thiscall, ret 8, news.cpp): `News::send_to_nation_club_managers(noticia, nacao)`: a notícia fica com
/// o alcance da nação (+0xde = 2) e vai (0x785d40 com 0) para cada um dos 16 técnicos humanos (os últimos 16 de
/// [0xae23bc], 0x6e bytes) ativos (+0x5f = 1) com clube (+0x39) da `nacao` (+0x53); a primeira entrega recusada (0)
/// encerra. Devolve 0. Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(861)(this, noticia, nacao); // caixa: só a original
    }
    *((noticia + 0xde) as *mut u8) = 2; // o alcance da nação
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    for i in 0..0x10usize {
        let p = read_usize(0xae23bc).wrapping_add(read_usize(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) != 1 {
            continue;
        }
        let clube = read_usize(p + 0x39);
        if clube == 0 || read_usize(clube + 0x53) != nacao {
            continue;
        }
        if entregar(this, noticia, p, 0) == 0 {
            break; // a entrega recusada encerra
        }
    }
    0
}

/// O olheiro da busca confere (o tipo +0xa é 4, o índice +0x180 > −1 e a pessoa ainda confere com os nomes e a data
/// guardados pela 0x53e990)? Devolve a pessoa, ou nenhuma (a original mostra a caixa).
unsafe fn cpi6_olheiro(this: usize) -> Option<usize> {
    let i = read_usize(this + 0x180) as i32;
    if *((this + 0xa) as *const u8) != 4 || i <= -1 {
        return None;
    }
    let p = read_usize(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e));
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    if confere(p, read_usize(this + 0x174), read_usize(this + 0x178), read_usize(this + 0x17c), this + 0x184) == 0 || p == 0 {
        return None;
    }
    Some(p)
}

/// { 0x7aefd0 } (thiscall, ret 0x18, player_search.cpp): `PLAYER_SEARCH::get_scout_assignment(clube, nacao, regiao,
/// comp, base, adversario)`: com o olheiro conferido, se alguma das missões está marcada (clube +0x190, nação +0x18c,
/// região +0x194 byte, competição +0x195 acima de −1; base +0x199, adversário +0x19d não nulos), copia as seis para
/// as saídas e devolve 1; nenhuma: 0. Olheiro que não confere: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn missao_do_olheiro(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    if cpi6_olheiro(this).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(862)(this, clube, nacao, regiao, comp, base, adversario); // caixa: só a original
    }
    let marcada = read_usize(this + 0x190) as i32 > -1
        || read_usize(this + 0x18c) as i32 > -1
        || *((this + 0x194) as *const i8) > -1
        || read_usize(this + 0x195) as i32 > -1 // a competição
        || read_usize(this + 0x199) != 0
        || read_usize(this + 0x19d) != 0;
    if !marcada {
        return 0;
    }
    clube.write_unaligned(read_usize(this + 0x190) as u32);
    nacao.write_unaligned(read_usize(this + 0x18c) as u32);
    *regiao = *((this + 0x194) as *const u8);
    comp.write_unaligned(read_usize(this + 0x195) as u32);
    base.write_unaligned(read_usize(this + 0x199) as u32);
    adversario.write_unaligned(read_usize(this + 0x19d) as u32);
    1
}

/// { 0x521e00 } (thiscall, ret 8, Cup.cpp): `Cup::check_complete(fases, _)`: 0 se a copa não começou (+0xb1). Com
/// `fases` = 1 e sem fase marcada (+0x44 = 0xff): cada fase de 0 até a atual (+0x30, vetor +0xc) tem de estar completa
/// (método +0x10 com 0, 1) e a atual tem de ser a última (+0x2c − 1); senão 0. Fora isso, 1.
unsafe extern "thiscall" fn copa_completa(this: usize, fases: u32, _x: u32) -> u8 {
    if *((this + 0xb1) as *const u8) == 0 {
        return 0;
    }
    if fases != 1 || *((this + 0x44) as *const u8) != 0xff {
        return 1;
    }
    let mut i = 0i32;
    while i <= read_usize(this + 0x30) as i32 {
        let fase = read_usize(read_usize(this + 0xc).wrapping_add(i as usize * 4));
        let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(fase) + 0x10));
        if completa(fase, 0, 1) == 0 {
            return 0; // uma fase incompleta
        }
        i += 1;
    }
    (read_usize(this + 0x30) as i32 >= (read_usize(this + 0x2c) as i32).wrapping_sub(1)) as u8
}

/// Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_nacao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(861)(this, noticia, nacao)
}

/// Comparador: lado a lado pelas seis saídas (a 0x53e990 só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_missao_do_olheiro(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(862);
    if cpi6_olheiro(this).is_none() {
        return o(this, clube, nacao, regiao, comp, base, adversario);
    }
    let reg = [(clube as usize, 4), (nacao as usize, 4), (regiao as usize, 1), (comp as usize, 4), (base as usize, 4), (adversario as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, clube, nacao, regiao, comp, base, adversario),
        || missao_do_olheiro(this, clube, nacao, regiao, comp, base, adversario));
    PLACAR.registrar(igual && r == n, || format!("missao_do_olheiro original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o método das fases só lê).
unsafe extern "thiscall" fn comparar_copa_completa(this: usize, fases: u32, x: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(863)(this, fases, x);
    let n = copa_completa(this, fases, x);
    PLACAR.registrar(r == n, || format!("copa_completa({:x}, {}) original={} rust={}", this, fases, r, n));
    r
}

// ---------- 213a leva: as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano (news.cpp) ----------

/// A entrada (12 bytes) da caixa de notícias do técnico humano: [noticias+8] + 12 × (id − [0xae2364] +
/// 0x10); None quando o índice fica negativo (a original abre a caixa de erro).
unsafe fn entrada_da_caixa_do_tecnico(noticias: usize, tecnico: usize) -> Option<usize> {
    let i = (*(tecnico as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some((*((noticias + 8) as *const usize)).wrapping_add((i as usize).wrapping_mul(12)))
}

/// A notícia `k` da caixa que começa em `inicio`: [noticias] + 0xdf × (inicio + k).
unsafe fn noticia_da_caixa(noticias: usize, inicio: i32, k: i32) -> usize {
    (*(noticias as *const usize)).wrapping_add((inicio.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// { 0x786fe0 } (thiscall, ret 8, news.cpp): as notícias não lidas do técnico humano. Sem categoria (0):
/// o total da entrada (+8, negativo vira 0). Com categoria: das 100 notícias da caixa (início na entrada
/// +0; −1 = vazia, 0), as ocupadas (+0xd6) da categoria (0x785670) ainda não lidas (+0xda = 0). Técnico
/// nulo ou fora da faixa: a original (caixa de erro).
unsafe extern "thiscall" fn noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let entrada = match if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) } {
        Some(e) => e,
        None => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(528)(noticias, tecnico, categoria),
    };
    let total = (entrada + 8) as *mut i32;
    if *total < 0 {
        *total = 0;
    }
    if categoria as u8 == 0 {
        return *total; // total da caixa
    }
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return 0;
    }
    let categoria_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
    let mut conta = 0;
    for k in 0..100 {
        let n = noticia_da_caixa(noticias, inicio, k);
        if core::ptr::read_unaligned((n + 0xd6) as *const u32) != 0
            && categoria_de(n) as u8 == categoria as u8
            && core::ptr::read_unaligned((n + 0xda) as *const u32) == 0
        {
            conta += 1;
        }
    }
    conta
}

/// Onde a 0x786d30 grava: Err nos casos da caixa de erro (técnico nulo, id 0, vaga fora de 0..49, cópia
/// de outro id, índice negativo, id fora das 100 da caixa); Ok(None) com a caixa vazia (início −1);
/// Ok(Some(notícia)) achada.
unsafe fn alvo_da_marca(noticias: usize, tecnico: usize, id: u32, vaga: i32, copia: usize) -> Result<Option<usize>, ()> {
    if tecnico == 0 || id == 0 || !(0..0x32).contains(&vaga) {
        return Err(());
    }
    if copia != 0 && core::ptr::read_unaligned((copia + 0xd6) as *const u32) != id {
        return Err(());
    }
    let entrada = entrada_da_caixa_do_tecnico(noticias, tecnico).ok_or(())?;
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return Ok(None);
    }
    (0..100)
        .map(|k| noticia_da_caixa(noticias, inicio, k))
        .find(|&n| core::ptr::read_unaligned((n + 0xd6) as *const u32) == id)
        .map(Some)
        .ok_or(())
}

/// { 0x786d30 } (thiscall, ret 0x14, news.cpp): grava `valor` na vaga `vaga` (dword +5 + 4 × vaga) da
/// notícia `id` (+0xd6) da caixa do técnico humano, primeiro na `copia` (se houver) e depois na da caixa;
/// 1 se achou. Caixa vazia: só a cópia, 0. Casos de erro: a original.
unsafe extern "thiscall" fn gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let alvo = match alvo_da_marca(noticias, tecnico, id, vaga, copia) {
        Ok(a) => a,
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(529)(noticias, tecnico, id, vaga, valor, copia),
    };
    if copia != 0 {
        core::ptr::write_unaligned((copia + 5 + 4 * vaga as usize) as *mut u32, valor);
    }
    match alvo {
        Some(n) => {
            core::ptr::write_unaligned((n + 5 + 4 * vaga as usize) as *mut u32, valor); // marca na caixa
            1
        }
        None => 0,
    }
}

/// { 0x7875e0 } (stdcall, ret 4, news.cpp): 1 se um humano dirige o `clube` (0x600a50 com férias
/// ignoradas); antes de devolver, consulta as férias (0x5fd210 em [0xb63c98]) do técnico do clube
/// (0x53ca90), sem usar a resposta. Clube nulo: 0.
unsafe extern "stdcall" fn clube_de_tecnico_humano(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let r = humano(clube, 1, 1, 0, 0);
    if r == 0 {
        return r;
    }
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t = tecnico(clube);
    ferias(*(0xb63c98 as *const usize), t);
    1 // humano
}

unsafe extern "thiscall" fn comparar_noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(528);
    let entrada = if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) };
    let Some(entrada) = entrada else {
        return o(noticias, tecnico, categoria);
    };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(entrada + 8, 4)], || o(noticias, tecnico, categoria),
        || noticias_nao_lidas(noticias, tecnico, categoria));
    PLACAR.registrar(igual && r == n, || format!("noticias_nao_lidas(categoria {}) original={} rust={} {:x?} x {:x?}", categoria as u8, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(529);
    let Ok(alvo) = alvo_da_marca(noticias, tecnico, id, vaga, copia) else {
        return o(noticias, tecnico, id, vaga, valor, copia);
    };
    let mut regioes = vec![];
    if copia != 0 {
        regioes.push((copia + 5 + 4 * vaga as usize, 4));
    }
    if let Some(a) = alvo {
        regioes.push((a + 5 + 4 * vaga as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(noticias, tecnico, id, vaga, valor, copia),
        || gravar_marca_da_noticia(noticias, tecnico, id, vaga, valor, copia));
    PLACAR.registrar(igual && r == n, || format!("gravar_marca_da_noticia(vaga {}) original={} rust={} {:x?} x {:x?}", vaga, r, n, mo, mr));
    r
}

unsafe extern "stdcall" fn comparar_clube_de_tecnico_humano(clube: usize) -> u32 {
    // só consultas: as duas rodam de novo sem efeito
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(530)(clube);
    let n = clube_de_tecnico_humano(clube);
    PLACAR.registrar(r == n, || format!("clube_de_tecnico_humano original={} rust={}", r, n));
    r
}

