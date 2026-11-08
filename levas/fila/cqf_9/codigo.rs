// ---------- %%LEVA%%a leva: a lesão pelo nome do player_setup.cfg; a notícia da volta aos treinos e o que ela acerta (o tratamento, as férias e a moral do jogador, a relação com o técnico, a fuga) e o nome da categoria da lesão (player_regen.cpp, injury.cpp) ----------

/// Os nomes das lesões do player_setup.cfg (a comparação, os caracteres comparados, o texto no .rdata e o tipo), na
/// ordem da original; o último, ARTHRITIS (8 caracteres, 0xa6b3a8), dá 0x4c, e nenhum, 0x4b.
const INJURY_NAMES: [(u32, usize, u8); 75] = [(4, 0xa6b840, 0), (3, 0xa6b83c, 1), (5, 0xa6b834, 2), (10, 0xa6b824, 3), (10, 0xa6b80c, 4), (10, 0xa6b7fc, 5), (10, 0xa6b7ec, 6), (10, 0xa6b7dc, 7), (10, 0xa6b7d0, 8), (10, 0xa6b7c0, 9), (10, 0xa6b7b4, 10), (15, 0xa6b7a0, 11), (15, 0xa6b788, 12), (10, 0xa6b778, 80), (10, 0xa6b768, 13), (10, 0xa6b758, 14), (10, 0xa6b74c, 15), (10, 0xa6b73c, 16), (10, 0xa6b730, 17), (10, 0xa6b71c, 18), (10, 0xa6b70c, 19), (10, 0xa6b6fc, 20), (10, 0xa6b6ec, 21), (10, 0xa6b6dc, 22), (10, 0xa6b6c8, 23), (10, 0xa6b6b4, 24), (10, 0xa6b6a0, 25), (10, 0xa6b690, 26), (10, 0xa6b678, 27), (10, 0xa6b664, 28), (10, 0xa6b654, 29), (10, 0xa6b644, 30), (10, 0xa6b634, 31), (10, 0xa6b628, 32), (10, 0xa6b618, 33), (10, 0xa6b608, 34), (10, 0xa6b5f8, 35), (10, 0xa6b5e4, 36), (5, 0xa6b5d8, 37), (10, 0xa6b5c8, 38), (10, 0xa6b5bc, 39), (10, 0xa6b5ac, 40), (10, 0xa6b5a0, 41), (14, 0xa6b590, 42), (10, 0xa6b584, 43), (10, 0xa6b574, 44), (10, 0xa6b564, 45), (10, 0xa6b554, 46), (5, 0xa6b548, 47), (10, 0xa6b538, 48), (10, 0xa6b52c, 49), (10, 0xa6b51c, 50), (10, 0xa6b50c, 52), (15, 0xa6b4fc, 53), (5, 0xa6b4f0, 54), (13, 0xa6b4e0, 55), (10, 0xa6b4d0, 56), (10, 0xa6b4c4, 57), (10, 0xa6b4b0, 58), (10, 0xa6b49c, 59), (10, 0xa6b48c, 60), (10, 0xa6b47c, 61), (10, 0xa6b46c, 62), (10, 0xa6b458, 63), (10, 0xa6b448, 64), (10, 0xa6b438, 65), (5, 0xa6b42c, 66), (10, 0xa6b420, 67), (5, 0xa6b414, 68), (5, 0xa6b408, 69), (13, 0xa6b3f8, 70), (10, 0xa6b3e8, 71), (19, 0xa6b3d4, 72), (10, 0xa6b3c0, 73), (10, 0xa6b3b4, 74)];

/// strncmp (0x946fa0, cdecl).
unsafe fn strncmp(a: usize, b: usize, n: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x946fa0usize);
    f(a, b, n)
}

/// { 0x7a32e0 } (cdecl, player_regen.cpp): `player_setup_injury_from_name(nome)`: o tipo da lesão (0..0x4c) pelo nome
/// da linha de lesão do player_setup.cfg (strncmp com os 76 nomes, cada um pelo seu tamanho, na ordem; o primeiro que
/// bate); nenhum: 0x4b. Pela aplicação das linhas (0x7a3a10) na criação do jogo. A devolução vale pelo al (a original
/// deixa o resto do eax do strncmp).
unsafe extern "C" fn player_setup_injury_from_name(nome: usize) -> u32 {
    for &(n, texto, tipo) in INJURY_NAMES.iter() {
        if strncmp(nome, texto, n) == 0 {
            crate::comparison::branch("cqf_9 lesao pelo nome");
            return tipo as u32; // cqf9: o tipo da lesão pelo nome
        }
    }
    if strncmp(nome, 0xa6b3a8, 8) == 0 { 0x4c } else { 0x4b }
}

/// Comparador: a devolução (o al; só lê).
unsafe extern "C" fn comparar_player_setup_injury_from_name(nome: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(nome);
    let n = player_setup_injury_from_name(nome);
    PLACAR.record(r as u8 == n as u8, || format!("player_setup_injury_from_name({:x}) original={:x} rust={:x}", nome, r as u8, n as u8));
    r
}

unsafe fn cqf9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf9_sb(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqf9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O ensaio do comparador: a cópia roda sem a entrega da notícia (0x786090, que aloca) e para antes da fuga (0x419840,
/// que põe na lista e manda notícia), guardando a decisão.
static mut RECOVERY_NEWS_DRY_RUN: bool = false;
static mut RECOVERY_NEWS_WOULD_GO_AWOL: bool = false;

/// O registro de forma do jogador na tabela das lesões ([this] + 31 × id).
unsafe fn physical_record_of_player(this: usize, p: usize) -> usize {
    cqf9_d(this).wrapping_add((*(p as *const i32) as isize * 31) as usize)
}

/// Gosta/não gosta do técnico (0x547810/0x547840) e tirar dos favoritos/rejeitados (0x547c10/0x547c50), cdecl.
unsafe fn call_preference_routine(rotina: usize, p: usize, tec: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(rotina);
    f(p, tec)
}

/// Acrescentar aos favoritos (0x547a10) ou aos rejeitados (0x547af0), cdecl(pessoa, outra, 1).
unsafe fn injury_add_to_preference_list(rotina: usize, p: usize, tec: usize) {
    let f: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(rotina);
    f(p, tec, 1);
}

/// O técnico gostar mais: tira dos rejeitados e, com sorteio(+0x59) acima de 5, põe nos favoritos.
unsafe fn like_manager_more(p: usize, clube: usize) {
    if cqf9_d(clube + 0xcf) != 0 && call_preference_routine(0x547840, p, cqf9_d(clube + 0xcf)) != 0 {
        call_preference_routine(0x547c50, p, cqf9_d(clube + 0xcf));
    }
    if cqf9_d(clube + 0xcf) != 0 && cqf9_sortear(cqf9_sb(p + 0x59) as i32) > 5 {
        injury_add_to_preference_list(0x547a10, p, cqf9_d(clube + 0xcf));
    }
}

/// { 0x630ee0 } (thiscall, ret 0x14, injury.cpp, quadro SEH da notícia local): `Injury::post_recovery_news(jogador,
/// estágio, tipo, forçar, dias)`, pelas voltas aos treinos (0x62e010) e pelo fim da lesão (0x62e700). Sem jogador:
/// nada; sem forçar, só com clube de técnico humano (0x7875e0). A notícia 2001 (0x77bbe0) com o estágio, o tipo, o id,
/// os três nomes, o nascimento, o ano e os dias (0x785790, dados 0..8). Tipo de categoria 12 (a tabela de 13 bytes em
/// 0x9e7ad9): sorteio(32000) e: tipo 'Q' (férias): com contrato (0x4cf5a0), ficha e clube, cada queixa do
/// contrato (+0x45: 0x20000, 0x20, 0x40) cai (0x8674e0) com sorteio(+0x5b) > sorteio(25); sem nenhuma cair, sorteio(+0x5b)
/// 0 leva à chance de fuga (com o gerente das faltas [0xad9c6c] e +0x59 abaixo de 12, sorteio(+0x59) < sorteio(5): a
/// fuga 0x419840 com 4 e a notícia não sai); senão a felicidade do contrato (+0x44) sobe 15 (−100..100) e, com marca
/// caída, o técnico gosta mais (resultado 2); depois sorteio(+0x5b + +0x59, − 5 com valor +0xb da ficha acima de 7250) <
/// sorteio(20) e o sorteio(4) acerta +0x57, +0x5d, +0x44 da ficha e +0x59. Estágio 0, tipo 'M' (tratamento): 0 dá
/// resultado 2 (forma +0x12 = 0xff, +0x13 = 0); abaixo de 5, resultado 1 (+0x13 = 0); senão resultado 0 e, com +0x5d
/// abaixo de 5, o técnico pode perder a simpatia (0x547810/0x547c10, sorteio(+0x5d) 0: rejeitado 0x547af0); +0x59 (ou
/// +0x57) acima de 10: o técnico gosta mais. Estágio 1, tipos 'O' e 'N': o resultado pelo sorteio, a forma e a marca
/// +0xc do registro de 79 bytes ([0xae2d0c]) cai. Dados 9 (resultado) e 10; a entrega pelo alcance `forcar` (0x786090:
/// o clube, a nação do clube ou da pessoa e o continente dela).
unsafe extern "thiscall" fn post_recovery_news(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    if p == 0 {
        return;
    }
    if forcar as u8 == 0 {
        let clube = cqf9_d(p + 0x39);
        if clube == 0 {
            return;
        }
        let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
        if humano(clube) == 0 {
            return;
        }
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x7d1, 0);
    let est = estagio as u8 as i8;
    let tp = tipo as u8 as i8;
    dado(n, 0, est as i32 as u32);
    dado(n, 1, tp as i32 as u32);
    dado(n, 2, cqf9_d(p) as u32);
    dado(n, 3, cqf9_d(cqf9_d(p + 4) + 0x33) as u32);
    dado(n, 4, cqf9_d(cqf9_d(p + 8) + 0x33) as u32);
    dado(n, 5, cqf9_d(cqf9_d(p + 0xc) + 0x33) as u32);
    dado(n, 6, *((p + 0x10) as *const i16) as i32 as u32);
    dado(n, 7, *((p + 0x12) as *const u16) as u32);
    dado(n, 8, dias as u16 as i16 as i32 as u32);
    let clube = cqf9_d(p + 0x39);
    let mut res: i8 = -1;
    let mut f1: i8 = 0;
    if *((0x9e7ad9 + tp as isize * 13) as *const u8) == 0xc {
        let s = cqf9_sortear(32000) as i16;
        if tp == 0x51 {
            res = 0;
            let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contract_of(cqf9_d(0xae1050), p);
            if c == 0 || cqf9_d(p + 0x61) == 0 || clube == 0 {
                f1 = 1;
            } else {
                let tirar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
                let mut caiu = false;
                for marca in [0x20000u32, 0x20] {
                    if cqf9_d(c + 0x45) as u32 & marca != 0 {
                        let a = cqf9_sortear(cqf9_sb(p + 0x5b) as i32);
                        let b = cqf9_sortear(0x19);
                        if a > b {
                            tirar(c, marca);
                            caiu = true;
                        }
                    }
                }
                let mut feliz = caiu;
                if cqf9_d(c + 0x45) as u32 & 0x40 != 0 {
                    let a = cqf9_sortear(cqf9_sb(p + 0x5b) as i32);
                    let b = cqf9_sortear(0x19);
                    if a > b {
                        tirar(c, 0x40);
                        caiu = true;
                        feliz = true;
                    }
                }
                if !feliz && cqf9_sortear(cqf9_sb(p + 0x5b) as i32) != 0 {
                    feliz = true;
                }
                let mut seguir = true;
                if feliz {
                    let v = cqf9_sb(c + 0x44) as i32 + 0xf; // cqf9: a felicidade sobe 15
                    crate::comparison::branch("cqf_9 felicidade pelas ferias");
                    *((c + 0x44) as *mut i8) = if v > 100 { 100 } else if v < -100 { -100 } else { v as i8 };
                }
                if feliz && caiu {
                    res = 2;
                    like_manager_more(p, clube);
                } else {
                    if cqf9_d(0xad9c6c) != 0 && cqf9_sb(p + 0x59) < 0xc {
                        let a = cqf9_sortear(cqf9_sb(p + 0x59) as i32);
                        let b = cqf9_sortear(5);
                        if a < b {
                            if RECOVERY_NEWS_DRY_RUN {
                                RECOVERY_NEWS_WOULD_GO_AWOL = true;
                                return;
                            }
                            let fugir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x419840usize);
                            fugir(cqf9_d(0xad9c6c), p, clube, 4);
                            return;
                        }
                    }
                    res = 0;
                    seguir = true;
                }
                if seguir {
                    let mut x = (cqf9_sb(p + 0x5b) as i16).wrapping_add(cqf9_sb(p + 0x59) as i16);
                    f1 = 1;
                    if *((cqf9_d(p + 0x61) + 0xb) as *const i16) > 0x1c52 {
                        x = x.wrapping_sub(5);
                    }
                    let a = cqf9_sortear(x as i32);
                    let b = cqf9_sortear(0x14);
                    if a < b {
                        let r = cqf9_sortear(4);
                        let f = cqf9_d(p + 0x61);
                        let subir_57 = |p: usize| {
                            if cqf9_sb(p + 0x57) < 0x14 {
                                *((p + 0x57) as *mut i8) += 1;
                            }
                        };
                        let descer_5d_44 = |p: usize, f: usize| {
                            if cqf9_sb(p + 0x5d) > 1 {
                                *((p + 0x5d) as *mut i8) -= 1;
                            }
                            if cqf9_sb(f + 0x44) > 1 {
                                *((f + 0x44) as *mut i8) -= 1;
                            }
                        };
                        crate::comparison::branch("cqf_9 acerto pelas ferias");
                        if r == 0 {
                            subir_57(p); // cqf9: o sorteio 0 sobe +0x57
                            descer_5d_44(p, f);
                        } else if r == 1 {
                            subir_57(p);
                        } else {
                            descer_5d_44(p, f);
                        }
                        if r == 0 || r == 1 {
                            if cqf9_sb(p + 0x59) > 1 {
                                *((p + 0x59) as *mut i8) -= 1;
                            }
                        }
                    }
                }
            }
        } else if est == 0 {
            if tp != 0x4d {
                res = -1;
            } else {
                let forma = physical_record_of_player(this, p);
                let mut v: Option<i8> = None;
                crate::comparison::branch("cqf_9 tratamento");
                if s == 0 { // cqf9: o tratamento dá certo
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                    v = Some(cqf9_sb(p + 0x59));
                } else if s < 5 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                    if cqf9_sb(p + 0x59) > 0xf {
                        v = Some(cqf9_sb(p + 0x57));
                    }
                } else {
                    res = 0;
                    if cqf9_sb(p + 0x5d) < 5 {
                        f1 = -1;
                        if clube != 0 {
                            if cqf9_d(clube + 0xcf) != 0 && call_preference_routine(0x547810, p, cqf9_d(clube + 0xcf)) != 0 {
                                call_preference_routine(0x547c10, p, cqf9_d(clube + 0xcf));
                            }
                            if cqf9_d(clube + 0xcf) != 0 && cqf9_sortear(cqf9_sb(p + 0x5d) as i32) == 0 {
                                injury_add_to_preference_list(0x547af0, p, cqf9_d(clube + 0xcf));
                            }
                        }
                    }
                }
                if let Some(v) = v {
                    if v > 10 {
                        f1 = 1;
                        if clube != 0 {
                            like_manager_more(p, clube);
                        }
                    }
                }
            }
        } else {
            let forma = physical_record_of_player(this, p);
            crate::comparison::branch("cqf_9 volta da lesao");
            if tp == 0x4f {
                if s < 30000 { // cqf9: a volta 'O' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if cqf9_sortear(5) != 0 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 3;
                    let a = forma + 0x13;
                    let r = cqf9_sortear(4);
                    *(a as *mut u8) = (*(a as *const u8)).wrapping_add((r as u8).wrapping_add(1));
                }
            } else if tp == 0x4e {
                if s < 0x733c { // cqf9: a volta 'N' completa
                    res = 2;
                    *((forma + 0x12) as *mut u8) = 0xff;
                    *((forma + 0x13) as *mut u8) = 0;
                } else if s < 30000 {
                    res = 1;
                    *((forma + 0x13) as *mut u8) = 0;
                } else {
                    res = 0;
                }
            }
            let e = cqf9_d(0xae2d0c).wrapping_add((*(p as *const i32) as isize * 0x4f + 0xc) as usize);
            *(e as *mut u8) &= 0xfe; // cqf9: a marca +0xc cai
        }
    }
    dado(n, 9, res as i32 as u32);
    dado(n, 10, f1 as i32 as u32);
    if RECOVERY_NEWS_DRY_RUN {
        return;
    }
    let nacao = if clube != 0 { cqf9_d(clube + 0x53) } else { cqf9_d(p + 0x1a) };
    let continente = if nacao != 0 { cqf9_d(nacao + 0x71) } else { 0 };
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x786090usize);
    entregar(cqf9_d(0xb7431c), n, clube, 0, nacao, continente, forcar);
}

/// Comparador em ensaio (modelo RARE_FINANCE_EVENT_DRY_RUN): a cópia roda primeiro sem a entrega da notícia e parando antes da fuga,
/// depois a original a partir do mesmo estado; compara o registro do jogador (0x6e), a ficha (0x46), o contrato (0x4f),
/// o registro de forma (31), o byte +0xc do registro de 79 bytes, o registro do técnico do clube e o estado do sorteio.
/// Com a fuga na cópia (a original põe na lista, sorteia e manda notícia): só a original.
unsafe extern "thiscall" fn comparar_post_recovery_news(this: usize, p: usize, estagio: u32, tipo: u32, forcar: u32, dias: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    if p == 0 {
        return o(this, p, estagio, tipo, forcar, dias);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    reg.push((p, 0x6e));
    if cqf9_d(p + 0x61) != 0 {
        reg.push((cqf9_d(p + 0x61), 0x46));
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let antes_1080 = cqf9_d(0xae1080);
    let c = contract_of(cqf9_d(0xae1050), p);
    *(0xae1080 as *mut usize) = antes_1080;
    if c != 0 {
        reg.push((c, 0x4f));
    }
    reg.push((0xae1080, 4));
    reg.push((physical_record_of_player(this, p), 31));
    reg.push((cqf9_d(0xae2d0c).wrapping_add((*(p as *const i32) as isize * 0x4f) as usize), 0x4f));
    let clube = cqf9_d(p + 0x39);
    if clube != 0 && cqf9_d(clube + 0xcf) != 0 {
        reg.push((cqf9_d(clube + 0xcf), 0x6e));
    }
    let antes = crate::comparison::MemorySnapshot::read(&reg);
    RECOVERY_NEWS_DRY_RUN = true;
    RECOVERY_NEWS_WOULD_GO_AWOL = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_recovery_news(this, p, estagio, tipo, forcar, dias)));
    RECOVERY_NEWS_DRY_RUN = false;
    let fugiu = RECOVERY_NEWS_WOULD_GO_AWOL;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    antes.write_back();
    o(this, p, estagio, tipo, forcar, dias);
    if fugiu {
        return;
    }
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    PLACAR.record(mo == mr, || format!("post_recovery_news(pessoa {:x}, estagio {}, tipo {:x}) {:x?} x {:x?}", p, estagio as u8, tipo as u8, mo.0, mr.0));
}

/// { 0x630c50 } (stdcall, ret 0xc, injury.cpp): `injury_get_category_name(categoria, texto, tamanho)`: o nome traduzido
/// (0x66f4e0) da categoria da lesão 0..12 (interna, pé, canela, joelho, coxa, virilha, tronco, mão, braço, ombro,
/// pescoço, cabeça, recuperação); fora disso a caixa (linha 0x10e7). Devolve 2 com o idioma 3 (0x66f4d0), senão 0 (o
/// gênero do nome). Tamanho abaixo de 20: a caixa (linha 0x10d0) e o al zerado.
unsafe extern "stdcall" fn injury_get_category_name(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    if tamanho < 0x14 {
        show_error_at(0x9e7f28, 0x10d0);
        return 0;
    }
    const NOMES: [usize; 13] = [0x9e89ac, 0x9e89a0, 0x9e8994, 0x9e8988, 0x9e8978, 0x9e8968, 0x9e8954, 0x9e8948, 0x9e893c, 0x9e892c, 0x9e8920, 0x9e8914, 0x9e8904];
    let k = categoria as u8 as i8 as i32 as u32;
    if k <= 0xc {
        let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
        traduzir(texto, NOMES[k as usize]);
    } else {
        show_error_at(0x9e7f28, 0x10e7);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    if idioma() == 3 { 2 } else { 0 }
}

/// Comparador: lado a lado no texto (o tamanho, até 0x100) e na devolução; o tamanho curto (a caixa): só a original.
unsafe extern "stdcall" fn comparar_injury_get_category_name(categoria: u32, texto: usize, tamanho: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize, i32) -> u32>(%%I2%%);
    if tamanho < 0x14 || texto == 0 {
        return o(categoria, texto, tamanho);
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&[(texto, (tamanho as usize).min(0x100))], || o(categoria, texto, tamanho), || injury_get_category_name(categoria, texto, tamanho));
    PLACAR.record(igual && r == n, || format!("injury_get_category_name({}) original={} rust={}", categoria as u8 as i8, r, n));
    r
}
