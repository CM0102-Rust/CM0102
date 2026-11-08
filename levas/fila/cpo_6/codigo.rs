// ---------- %%LEVA%%a leva: a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id (media.cpp, news.cpp) ----------

unsafe fn cpo6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal (os atributos de 1 a 20).
unsafe fn cpo6_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

/// O tipo do contrato (+0x4f, nibble alto).
unsafe fn contract_squad_status(contrato: usize) -> u8 {
    *((contrato + 0x4f) as *const u8) & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
const STORED_NEWS_SIZE: usize = 0xdf;

/// { 0x735870 } (thiscall, ret 0xc, media.cpp, 3,7 KB): `Media::pick_ai_manager_response(tecnico, jogador, tipo)`: a
/// resposta (1, 2 ou 3) do técnico da máquina a uma história do `tipo` sobre o jogador. Sem técnico, sem a ficha de
/// técnico (+0x69), jogador sem ficha (+0x61), o próprio técnico, sem clube (+0x39, senão a seleção +0x24) ou sem
/// contrato do jogador com ele (0x4cf5f0): 3. A nota é trunc(5 × 0x736720(jogador, clube, contrato, tipo, 3, 0)); sem
/// jogador, a palavra do argumento fica no lugar (o ponteiro do técnico, como no original). Depois, pelo tipo: 0x83
/// pelo tipo de contrato e o conhecimento (0x544410); 0x82 pelas preferências do técnico (0x547840/0x547810) e
/// sorteios; 0x1f pela lista de transferências (0x544520) e o contrato; os outros, antes, pelas preferências com a
/// nota, e depois por famílias de tipos, cada uma com a sua cadeia de sorteios (0x90cfc0); 8/0x25 e 3/4 podem pôr o
/// jogador entre os rejeitados do técnico (0x547af0).
unsafe extern "thiscall" fn pick_ai_manager_response(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let contract_with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let nota_da_historia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x736720usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let listada_pelo_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    if tecnico == 0 || cpo6_d(tecnico + 0x69) == 0 {
        return 3;
    }
    if jogador != 0 && cpo6_d(jogador + 0x61) == 0 {
        return 3;
    }
    if tecnico == jogador {
        return 3;
    }
    let mut clube = cpo6_d(tecnico + 0x39);
    if clube == 0 {
        clube = cpo6_d(tecnico + 0x24);
        if clube == 0 {
            return 3;
        }
    }
    let contrato = contract_with_club(cpo6_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return 3;
    }
    let mut nota = tecnico as i32; // sem jogador: o argumento que ocupa a vaga
    if jogador != 0 {
        nota = match_pl_ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * *(0x968990 as *const f32) as f64);
    }
    let f = || cpo6_d(tecnico + 0x69); // a ficha de técnico
    let ch = |o: usize| chance(f(), *((f() + o) as *const i8)) as i8 as i32; // o conhecimento pelo atributo +o
    let k = tipo as i16;
    if k == 0x83 {
        let t = contract_squad_status(contrato);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && cpo6_c(contrato + 0x3f) > 1 {
            return 1;
        }
        if cpo6_w(cpo6_d(jogador + 0x61) + 0xb) < 0x1482 && ch(0x14) > 0xf {
            return 2;
        }
        if contract_squad_status(contrato) == 0x70 && ch(0x14) > 0xa {
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
                if *((cpo6_d(jogador + 0x61) + 0x44) as *const u8) != 0 {
                    // o registro de 0x4f bytes da pessoa em [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| *((cpo6_d(0xae2d0c).wrapping_add((cpo6_i(jogador) as usize).wrapping_mul(0x4f)) + o) as *const u8);
                    let e = sorteio((2 - (reg(0xb) >> 7) as i32) * 20);
                    if sorteio(reg(0xa) as i32) > e {
                        arg = 1;
                    }
                }
                if sorteio(arg) == 0 && nota < 0xa {
                    return 2;
                }
            }
            if sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x45)) == 0 {
                return 1;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(0x14 - nota);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            crate::comparison::branch("cpo_6 tipos comuns");
            let t = contract_squad_status(contrato);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = entrada(cpo6_d(cpo6_d(0xde1f54) + 0x856), clube, jogador, 0, 0);
                if e != 0 && cpo6_d(e + 0xf) == cpo6_d(jogador) && nota > -10 {
                    crate::comparison::branch("cpo_6 sorteio descartado");
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
                if cpo6_w(cpo6_d(jogador + 0x61) + 0xb) as i32 <= cpo6_w(clube + 0x80) as i32 + 0x1f4 {
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
            let t = contract_squad_status(contrato);
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
            let t = contract_squad_status(contrato);
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
                        let x = sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x25));
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
            let x = sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x25));
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
unsafe extern "thiscall" fn mark_item_read(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    if tecnico == 0 {
        show_error_at(0xa693ec, 0x2358);
        return 0;
    }
    if id == 0 {
        show_error_at(0xa693ec, 0x2360);
        return 0;
    }
    let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        show_error_at(0xa693ec, 0x236b);
        return 0;
    }
    let e = cpo6_d(this + 8) + idx as usize * 12;
    if cpo6_i(e) == -1 {
        return 0;
    }
    let botao: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8 = core::mem::transmute(0x782b70usize);
    let mut v = 0usize;
    for k in 0..0x64usize {
        v = cpo6_d(this).wrapping_add((cpo6_d(e).wrapping_add(k)).wrapping_mul(STORED_NEWS_SIZE));
        if cpo6_d(v + 0xd6) as u32 == id && cpo6_d(v + 0xda) == 0 {
            let mut tratador = 0u32;
            if botao(v, tecnico, 0xde1f64, 0x7d0, &mut tratador) != 2 {
                crate::comparison::branch("cpo_6 noticia lida");
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
        let maquina: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7fffa0usize);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x787140usize);
        let da_vez = cpo6_d(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize);
        let m = maquina(0xb74450, da_vez) as u16;
        for j in 0..0x10i32 {
            let p = cpo6_d(0xae23bc).wrapping_add((cpo6_i(0xae2364) - j - 1).wrapping_mul(0x6e) as usize);
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
unsafe extern "thiscall" fn find_item_by_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    if id == 0 {
        show_error_at(0xa693ec, 0x2252);
        return 0;
    }
    if saida == 0 {
        show_error_at(0xa693ec, 0x225a);
        return 0;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x786010usize);
    let (inicio, n) = if tecnico == 0 {
        (0usize, cpo6_i(this + 4).max(0) as usize)
    } else {
        let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
        if idx < 0 {
            show_error_at(0xa693ec, 0x2268);
            return 0;
        }
        let ini = cpo6_i(cpo6_d(this + 8) + idx as usize * 12);
        if ini == -1 {
            return 0;
        }
        (ini as usize, 0x64)
    };
    for k in 0..n {
        let v = cpo6_d(this).wrapping_add(inicio.wrapping_add(k).wrapping_mul(STORED_NEWS_SIZE));
        if cpo6_d(v + 0xd6) as u32 == id {
            copiar(saida, v);
            return 1;
        }
    }
    0
}

/// Comparador: a resposta sorteia e pode pôr o jogador entre os rejeitados do técnico (0x547af0): lado a lado no
/// estado do gerador e no bloco das preferências do técnico (+0x65, +0x1c..+0x34), a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_pick_ai_manager_response(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%);
    let mut regioes = crate::rng::STATE_REGIONS.to_vec();
    if tecnico != 0 && cpo6_d(tecnico + 0x65) != 0 {
        regioes.push((cpo6_d(tecnico + 0x65) + 0x1c, 0x18));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regioes,
        || o(this, tecnico, jogador, tipo),
        || crate::rng::without_counting(|| pick_ai_manager_response(this, tecnico, jogador, tipo)),
    );
    PLACAR.record(igual && r == n, || format!("pick_ai_manager_response(tipo {:#x}) original={} rust={} {:x?} x {:x?}", tipo as i16, r, n, mo.0, mr.0));
    r
}

/// Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_mark_item_read(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I1%%);
    if tecnico == 0 || id == 0 || cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0 || cpo6_d(this) == 0 {
        return o(this, tecnico, id, sincronizar);
    }
    let regioes = [
        (cpo6_d(this), cpo6_i(this + 4).max(0) as usize * STORED_NEWS_SIZE),
        (cpo6_d(this + 8), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&regioes, || o(this, tecnico, id, sincronizar), || mark_item_read(this, tecnico, id, sincronizar));
    PLACAR.record(igual && r == n, || format!("mark_item_read(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_find_item_by_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I2%%);
    if id == 0 || saida == 0 || (tecnico != 0 && cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0) {
        return o(this, tecnico, id, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(saida, STORED_NEWS_SIZE)], || o(this, tecnico, id, saida), || find_item_by_id(this, tecnico, id, saida));
    PLACAR.record(igual && r == n, || format!("find_item_by_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}
