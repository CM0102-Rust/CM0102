// ---------- 236a leva: o nome pelos ids, o nome do oficial e o texto da máscara da posição (Database.cpp) ----------

unsafe fn dbs1_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Texto não nulo e não vazio (os `repne scasb` da original).
unsafe fn dbs1_cheio(t: usize) -> bool {
    t != 0 && *(t as *const u8) != 0
}

/// { 0x53e140 } (cdecl, Database.cpp): `database_format_name_from_ids(nome, sobrenome, comum, texto, tamanho, forma,
/// pessoa)`: os registros dos nomes (60 bytes) pelos índices nas tabelas [0xae23f0], [0xae23f4] e [0xae23f8] (−1 ou
/// menos: nulo) e o texto pela 0x53e2e0 (com −1 no fim), que é o que devolve. Com `pessoa` > −1 e [0xb63c98]: se um
/// humano deu apelido à pessoa ([0xae23bc] + 110 × pessoa; 0x600740) e ela confere com os nomes (0x53e990), o apelido
/// (0x6005e0) no lugar do nome comum. Texto nulo ou tamanho (palavra) < 0x69: a original (caixa). As locais de 0x200
/// bytes só servem à mensagem da caixa. As chamadas pelo endereço do jogo. ~126 chamadores (notícias, telas, contratos).
unsafe extern "C" fn nome_por_ids(nome: i32, sobrenome: i32, comum: i32, texto: usize, tamanho: u32, forma: u32, pessoa: i32) -> u32 {
    if texto == 0 || (tamanho as i16) < 0x69 {
        return orig::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(741)(nome, sobrenome, comum, texto, tamanho, forma, pessoa);
    }
    let deu_apelido: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x600740usize);
    let confere: unsafe extern "C" fn(usize, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x53e990usize);
    let apelido: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x6005e0usize);
    let formatar: unsafe extern "C" fn(usize, usize, usize, usize, u32, u32, i32) -> u32 = core::mem::transmute(0x53e2e0usize);
    let registro = |tabela: usize, i: i32| if i > -1 { dbs1_u32(tabela).wrapping_add((i as usize).wrapping_mul(60)) } else { 0 };
    let (n, s) = (registro(0xae23f0, nome), registro(0xae23f4, sobrenome));
    let apelidos = dbs1_u32(0xb63c98);
    let mut terceiro = None;
    if pessoa > -1 && apelidos != 0 {
        let p = dbs1_u32(0xae23bc).wrapping_add((pessoa as usize).wrapping_mul(110));
        if deu_apelido(apelidos, p, 0) != 0 && confere(p, nome, sobrenome, comum, 0) != 0 {
            terceiro = Some(apelido(apelidos, p, 0));
        }
    }
    let c = terceiro.unwrap_or_else(|| registro(0xae23f8, comum)); // o nome comum
    formatar(n, s, c, texto, tamanho, forma, -1)
}

unsafe extern "C" fn comparar_nome_por_ids(nome: i32, sobrenome: i32, comum: i32, texto: usize, tamanho: u32, forma: u32, pessoa: i32) -> u32 {
    let o = orig::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(741);
    if texto == 0 || (tamanho as i16) < 0x69 {
        return o(nome, sobrenome, comum, texto, tamanho, forma, pessoa); // caixa: só a original
    }
    // o texto produzido (até o tamanho dado, no máximo 128 bytes)
    let n = (tamanho as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(nome, sobrenome, comum, texto, tamanho, forma, pessoa),
        || nome_por_ids(nome, sobrenome, comum, texto, tamanho, forma, pessoa));
    PLACAR.registrar(igual && r == x, || format!("nome_por_ids({}, {}, {}, forma {}, pessoa {}) original={} rust={} {:x?} x {:x?}", nome, sobrenome, comum, forma, pessoa, r, x, mo, mr));
    r
}

/// { 0x53eab0 } (cdecl, Database.cpp): `database_format_official_name(oficial, texto, tamanho, forma)`: o nome do
/// oficial (nome +4, sobrenome +8) em `texto` pela forma (byte): 1 "%s %s", 2 "%c.%s", 3 "%s, %s" (sobrenome
/// primeiro), 4 "%s, %c", 5 só o sobrenome, 6 "%c %c" (as iniciais). Faltando um dos dois: o sobrenome (ou o nome);
/// na forma 6, a inicial que houver. Oficial da nação +0x16 com id igual a [0x9cf488]: só o sobrenome, em qualquer
/// forma. Devolve 1. Oficial ou texto nulos, tamanho (palavra) < 0x69 ou forma fora de 1..6: a original (caixa). Nos
/// eventos e no painel da partida (0x6d8d10, 0x6d7b00, 0x71c270).
unsafe extern "C" fn nome_oficial(oficial: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(742);
    if oficial == 0 || texto == 0 || (tamanho as i16) < 0x69 {
        return o(oficial, texto, tamanho, forma);
    }
    let (a, b) = (dbs1_u32(oficial + 4), dbs1_u32(oficial + 8));
    let nacao = dbs1_u32(oficial + 0x16);
    let so_sobrenome = nacao != 0 && dbs1_u32(nacao) == dbs1_u32(0x9cf488);
    let f = forma as u8;
    if !so_sobrenome && !(1..=6).contains(&f) {
        return o(oficial, texto, tamanho, forma);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
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
        if dbs1_cheio(b) {
            copiar(b);
        } else if dbs1_cheio(a) {
            copiar(a);
        }
    };
    let os_dois = dbs1_cheio(a) && dbs1_cheio(b);
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
                } else if dbs1_cheio(b) {
                    sprintf(texto, 0x99ec1c, inicial(b)); // "%c"
                } else if dbs1_cheio(a) {
                    sprintf(texto, 0x99ec1c, inicial(a));
                }
            }
        }
    }
    1
}

unsafe extern "C" fn comparar_nome_oficial(oficial: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(742);
    if oficial == 0 || texto == 0 || (tamanho as i16) < 0x69 {
        return o(oficial, texto, tamanho, forma); // caixa: só a original
    }
    let n = (tamanho as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(oficial, texto, tamanho, forma),
        || nome_oficial(oficial, texto, tamanho, forma));
    PLACAR.registrar(igual && r == x, || format!("nome_oficial({:x}, forma {}) original={} rust={} {:x?} x {:x?}", oficial, forma as u8, r, x, mo, mr));
    r
}

/// { 0x5482f0 } (cdecl, Database.cpp): `position_mask_get_text(mascara, texto, maiuscula)`: o nome da posição da
/// máscara (0x66f4e0 com o texto 0xad9c64; a forma com maiúscula se o byte `maiuscula` não é 0), na ordem: goleiro
/// (1), jogador de apoio (0x260/0x160/0x460), atacante (0x40 com 0x700), ala (0x88/0x808), ponta esquerda (0x90/0xa0),
/// ponta direita (0x810/0x820), meia central e meia-atacante (0x100/0x200/0x400 com 0x18 ou 0x20), zagueiro central
/// (0x204/0x104/0x404), lateral esquerdo (0x84), lateral direito (0x804), líbero (2), reserva (0) e defensor. Devolve o
/// byte do gênero pelo idioma (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4, os outros 0 (reserva: sempre 0). Texto
/// nulo: 0. Na mídia (0x72d270) e na tela da pessoa (0x87dd80).
unsafe extern "C" fn texto_da_mascara_da_posicao(mascara: u32, texto: usize, maiuscula: u32) -> u8 {
    if texto == 0 {
        return 0;
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
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

unsafe extern "C" fn comparar_texto_da_mascara_da_posicao(mascara: u32, texto: usize, maiuscula: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32) -> u8>(743);
    if texto == 0 {
        return o(mascara, texto, maiuscula);
    }
    // sem tamanho: os nomes traduzidos cabem em 128
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 128)], || o(mascara, texto, maiuscula),
        || texto_da_mascara_da_posicao(mascara, texto, maiuscula));
    PLACAR.registrar(igual && r == x, || format!("texto_da_mascara_da_posicao({:x}, {}) original={} rust={} {:x?} x {:x?}", mascara, maiuscula as u8, r, x, mo, mr));
    r
}

// ---------- 236a leva: o nome da posição, a descrição do jogador e o melhor candidato a favorito (Database.cpp) ----------

unsafe fn dbs2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbs2_i8(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn dbs2_i16(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

/// { 0x5461b0 } (cdecl, Database.cpp): `write_position_name(mascara, texto, tamanho)`: o nome da posição (0x66f4e0;
/// cada um escreve por cima do anterior) contando as posições: 1 goleiro, 2 líbero, 4 zagueiro central (com 0x200 sem
/// 0x80/0x800), defensor (com 0x200) ou lateral, 8 lateral ofensivo (sem 4 e 0x200) ou volante (com 0x200), 0x10 meia,
/// 0x20 sem nenhuma antes (conta 1) meia-atacante (0x200), ponta (0x80 e 0x800), ponta esquerda (0x80) ou direita, 0x40
/// com 0x200 atacante; mais de 3: "utility player"; nenhuma: "player". Devolve o byte do gênero do último pelo idioma
/// (0x66f4d0): 2 dá 2 ou 5, 5 dá 1, 4 dá 1 ou 4; os outros não mudam (começa em 0). Tamanho < 30: a original (caixa).
/// Na mídia, nos contratos, na busca de jogadores, nos olheiros e nas transferências (0x8dc370).
unsafe extern "C" fn nome_da_posicao(mascara: u32, texto: usize, tamanho: i32) -> u8 {
    if tamanho < 0x1e {
        return orig::<unsafe extern "C" fn(u32, usize, i32) -> u8>(744)(mascara, texto, tamanho);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
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

unsafe extern "C" fn comparar_nome_da_posicao(mascara: u32, texto: usize, tamanho: i32) -> u8 {
    let o = orig::<unsafe extern "C" fn(u32, usize, i32) -> u8>(744);
    if tamanho < 0x1e {
        return o(mascara, texto, tamanho); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(mascara, texto, tamanho),
        || nome_da_posicao(mascara, texto, tamanho));
    PLACAR.registrar(igual && r == x, || format!("nome_da_posicao({:x}) original={} rust={} {:x?} x {:x?}", mascara, r, x, mo, mr));
    r
}

/// { 0x5465a0 } (cdecl, Database.cpp): `write_player_description(pessoa, posicao, qualidades, texto, tamanho,
/// genero)`: a descrição do jogador (0x66f4e0) pelas qualidades (as da 0x546ab0) e pela máscara da posição. Atacante
/// (qualidades 0x40): matador letal, matador ou goleador veloz (pela soma índice + qualidades % 3), oportunista,
/// matador ou artilheiro (% 4), atacante dinâmico, de força, forte, criativo, empolgante, habilidoso. Meia (posição
/// 0x10): com 0x40000 "inspirational" e por cima "free-scoring" (devolve 0); armador, ladrão de bola, dinâmico,
/// esforçado, confiável, veloz. Qualidades 2 (sem 4): lateral veloz ou empolgante; 4: defensor veloz ou técnico. Senão,
/// pela idade (+0x18 da pessoa [0xae23bc] + 110 × índice): veterano acima de índice % 3 + 33 (sem a qualidade 1),
/// revelação abaixo de índice % 3 + 18 com +0x22 nulo e a soma % 3 = 0. Devolve o gênero: com o idioma 4, 1, 3 ou 4
/// conforme o texto; senão o byte `genero` (1 com o idioma 5, que a original grava também no próprio argumento; a
/// cópia não grava). Tamanho < 30: a original (caixa).
unsafe extern "C" fn descricao_do_jogador(pessoa: i32, posicao: u32, qualidades: u32, texto: usize, tamanho: i32, genero: u32) -> u8 {
    if tamanho < 0x1e {
        return orig::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(745)(pessoa, posicao, qualidades, texto, tamanho, genero);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
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
    let p = dbs2_u32(0xae23bc).wrapping_add((pessoa as usize).wrapping_mul(110));
    let idade = dbs2_i8(p + 0x18);
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

unsafe extern "C" fn comparar_descricao_do_jogador(pessoa: i32, posicao: u32, qualidades: u32, texto: usize, tamanho: i32, genero: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(i32, u32, u32, usize, i32, u32) -> u8>(745);
    if tamanho < 0x1e {
        return o(pessoa, posicao, qualidades, texto, tamanho, genero); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(pessoa, posicao, qualidades, texto, tamanho, genero),
        || descricao_do_jogador(pessoa, posicao, qualidades, texto, tamanho, genero));
    PLACAR.registrar(igual && r == x, || format!("descricao_do_jogador({}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, posicao, qualidades, r, x, mo, mr));
    r
}

/// A nota do colega `o` como favorito da `pessoa` no `clube` (os filtros e a nota da 0x548810, iguais aos da 0x548da0),
/// ou `None` se ele não entra. As chamadas pelo endereço do jogo, na ordem da original.
unsafe fn dbs2_nota(pessoa: usize, clube: usize, o: usize) -> Option<i32> {
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    if dbs2_u32(o + 0x61) == 0 {
        return None;
    }
    let id = dbs2_u32(pessoa) as i32;
    let oid = dbs2_u32(o) as i32;
    if oid >= dbs2_u32(0xae2364) as i32 || o == pessoa || oid.wrapping_add(id) % 3 == 0 {
        return None;
    }
    let pref = dbs2_u32(pessoa + 0x65);
    if pref != 0 && (0..3).any(|j| dbs2_u32(pref + 0x28 + 4 * j) == o) {
        return None; // rejeitado pela pessoa
    }
    let contrato = contrato_de(dbs2_u32(0xae1050), o, clube);
    if contrato == 0 {
        return None;
    }
    let rep = dbs2_i16(dbs2_u32(o + 0x61) + 0xb);
    if rep <= 0x109a || rep <= dbs2_i16(clube + 0x80) - 0x2ee {
        return None;
    }
    if fora(dbs2_u32(0xde1f54), o, 0) != 0 || dbs2_i16(dbs2_u32(o + 0x61) + 5) <= 100 {
        return None;
    }
    let l0 = media(dbs2_u32(0xb74350), dbs2_u32(o + 0x61), 6, 0x11, 6.5) as f32 as f64; // fstp dword
    let m1 = media(dbs2_u32(0xb74350), dbs2_u32(o + 0x61), 6, 1, 6.5);
    let dados = dbs2_u32(o + 0x61);
    let mut n = dbs2_i16(dados + 0xb);
    if m1 > f32_em(0x968990) as f64 {
        let g = dbs2_u32(0xb74350);
        let razao = || {
            let d = f64_em(g + 0x18);
            if d == 0.0 || d.is_nan() { f64_em(0x968998) } else { f64_em(g + 0x10) / d }
        };
        // fcompp: C0 (abaixo ou sem ordem) soma
        if !(razao() + f64_em(0x968918) >= l0) {
            n += 500;
        } else if !(razao() + f64_em(0x967880) >= l0) {
            n += 250;
        }
    }
    if pref != 0 && (0..3).any(|j| dbs2_u32(pref + 0x1c + 4 * j) == o) {
        n += 1250; // já favorito
    }
    let c = dbs2_i8(dados + 0x21);
    if c == 20 {
        n += 75;
    } else if c > 15 {
        n += 50;
    }
    let a = dbs2_i8(o + 0x5b);
    if a > 15 {
        n += 250;
    } else if a < 6 {
        n -= 100;
    }
    n += id.wrapping_add(oid) % 500; // desempate pelos ids (dbs)
    match *((contrato + 0x4f) as *const u8) >> 4 {
        1 => n += 750,
        2 => n += 300,
        3 => n += 150,
        _ => {}
    }
    let nacao = dbs2_u32(pessoa + 0x1a);
    if nacao != dbs2_u32(clube + 0x53) && dbs2_u32(o + 0x1a) == nacao {
        if dbs2_u32(contrato + 0x45) & 0x20020 != 0 {
            n += 1750;
        } else if idade(0xae2c90, dbs2_u32(pessoa + 0x3e), dbs2_u32(pessoa + 0x42)) < 350 && dbs2_i8(pessoa + 0x18) < 24 {
            n += 250;
        } else {
            n += 125;
        }
    }
    Some(n)
}

/// { 0x548810 } (cdecl, Database.cpp): `staff_is_best_favoured_candidate(pessoa, clube, candidato)`: 1 se o candidato
/// passa nos filtros da escolha do companheiro favorito (os da 0x548da0 sem o da máscara da posição) e nenhum dos 50 do
/// elenco do clube (+0xd7) tem nota estritamente maior; 0 se não. Antes, os da pessoa: [0xb74350], id % 6 = r: +0x59 ≥
/// r + 6, +0x57 ≤ 15 − r, idade (0x54cdb0) ≥ id % 28 + 14. A original guarda a nota do candidato no próprio argumento
/// `pessoa` e um float no `candidato` (gravados antes de lidos); a cópia não. Só `al`. Na mídia (0x758200).
unsafe extern "C" fn melhor_candidato_a_favorito(pessoa: usize, clube: usize, candidato: usize) -> u8 {
    if pessoa == 0 || clube == 0 || candidato == 0 || dbs2_u32(0xb74350) == 0 {
        return 0;
    }
    let idade: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let id = dbs2_u32(pessoa) as i32;
    let r6 = id % 6;
    if dbs2_i8(pessoa + 0x59) < r6 + 6 || dbs2_i8(pessoa + 0x57) > 15 - r6 {
        return 0;
    }
    if idade(0xae2c90, dbs2_u32(pessoa + 0x3e), dbs2_u32(pessoa + 0x42)) < id % 28 + 14 {
        return 0;
    }
    let alvo = match dbs2_nota(pessoa, clube, candidato) {
        Some(n) => n,
        None => return 0,
    };
    for k in 0..50 {
        let o = dbs2_u32(clube + 0xd7 + 4 * k);
        if o == 0 {
            continue;
        }
        if let Some(n) = dbs2_nota(pessoa, clube, o) {
            if n > alvo {
                return 0; // há um melhor
            }
        }
    }
    1
}

unsafe extern "C" fn comparar_melhor_candidato_a_favorito(pessoa: usize, clube: usize, candidato: usize) -> u8 {
    // as chamadas (contrato, média, 0x8cca10) vão pelo endereço do jogo: o estado do sorteio volta entre os lados e a
    // cópia não conta, se alguma delas sortear
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u8>(746);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa, clube, candidato),
        || crate::aleatorio::sem_contar(|| melhor_candidato_a_favorito(pessoa, clube, candidato)));
    PLACAR.registrar(igual && r == x, || format!("melhor_candidato_a_favorito({:x}, {:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, candidato, r, x, mo, mr));
    r
}

// ---------- 236a leva: as qualidades e os defeitos do jogador (Database.cpp) ----------

unsafe fn dbs3_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbs3_i16(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

/// A 0x7c1e30 das notas ([0xb74350]) do jogador (+0x61) da pessoa: `media(p, tipo, atributo)`, peso 6,5.
unsafe fn dbs3_media(pessoa: usize, tipo: u32, atributo: u32) -> f64 {
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    media(dbs3_u32(0xb74350), dbs3_u32(pessoa + 0x61), tipo, atributo, 6.5)
}

/// O atributo calculado (0x543540) do jogador da pessoa, como byte com sinal.
unsafe fn dbs3_atributo(pessoa: usize, atributo: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543540usize);
    f(pessoa, dbs3_u32(pessoa + 0x61), atributo) as i8 as i32
}

/// Os bits das posições em que o jogador é natural (20), comuns às duas: goleiro (+0xf), lateral (+0x11, com +0x19
/// ou com +0x17/+0x18), zagueiros (+0x12/+0x13 sem +0x17/+0x18 a 15), meias (+0x14, +0x15). `bits` = os valores
/// de cada caso na ordem; mais o da soma das dez posições (+0x10..+0x19) acima de 110.
unsafe fn dbs3_posicoes(j: usize, bits: [u32; 7], soma: u32) -> u32 {
    let b = |k: usize| sb(j, k);
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

/// { 0x546ab0 } (cdecl): `staff_get_player_strength_flags(pessoa, outro)`: os bits das qualidades do jogador (+0x61)
/// para a mídia, olheiros, busca e transferências; `outro` só precisa não ser nulo. Posições naturais (1, 4, 2, 8, 0x10,
/// 0x20, 0x40), 0x1000000 se o nibble de [0xae2d0c] + 79 × id + 0xb passa um sorteio(3) e a 0x8cca10 ([0xde1f54]) dá 0,
/// 0x80 pela soma das posições; depois 19 sorteios (0x90cfc0) contra bytes do jogador, atributos (0x543540) e médias
/// (0x7c1e30 em [0xb74350], peso 6,5, x87 em precisão dupla com os `fstp dword` no meio): 0x100 (+0x3c), 0x200 (+0x37
/// mais o ajuste das médias 0x16/0xd/5/1), 0x800000 (média 0xe acima de 8), 0x400 (+0x39, +0x3e e a média 0xe ou o
/// atributo 0x17 com as médias 7 e 0x16), 0x800, 0x1000, 0x2000 (+0x2e, +0x40, +0x44), 0x4000, 0x20000, 0x100000,
/// 0x200000, 0x400000 (atributos 4, 0xd, 0x14, 0x15, 1), 0x8000 (+0x21), 0x10000 (atributo 0xb e a média 0xf), 0x40000
/// (reputação e as médias 6/2 e 6/1), 0x80000 (médias 0x15, 0x10, 2 e 1). A original guarda floats nos próprios
/// argumentos (gravados antes de lidos); a cópia não. Nulos: 0.
unsafe extern "C" fn forcas_do_jogador(pessoa: usize, outro: usize) -> u32 {
    if outro == 0 || pessoa == 0 {
        return 0;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    let j = || dbs3_u32(pessoa + 0x61);
    let b = |k: usize| sb(j(), k);
    let m = |tipo: u32, atributo: u32| dbs3_media(pessoa, tipo, atributo);
    let mut f = dbs3_posicoes(j(), [1, 4, 2, 8, 0x10, 0x20, 0x40], 0);
    let r = sortear(3);
    let id = dbs3_u32(pessoa) as u32;
    let nibble = (*(dbs3_u32(0xae2d0c).wrapping_add(id.wrapping_mul(79) as usize).wrapping_add(0xb) as *const u8) & 0xf) as i32;
    if nibble > r && dbs3_u32(0xde1f54) != 0 && fora(dbs3_u32(0xde1f54), pessoa, 0) == 0 {
        f += 0x1000000;
    }
    if j() != 0 && (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += 0x80;
    }
    if b(0x3c) > sortear(3) + 15 {
        f += 0x100;
    }
    let mut ajuste = 0i32;
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = ftol(a as f64 - f64_em(0x969568));
        }
        let x = (m(2, 5) * f32_em(0x969560) as f64) as f32; // fstp dword
        ajuste = ajuste.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37).wrapping_add(ajuste) > sortear(50) {
        f += 0x200;
    }
    let v14 = m(2, 0xe);
    let v14f = v14 as f32; // fst dword
    if v14 > f64_em(0x968f10) {
        f += 0x800000;
    }
    if b(0x39) > sortear(50) && b(0x3e) > sortear(50) {
        let mut sim = false;
        if v14f as f64 > f64_em(0x9689a0) {
            let r = sortear(10) + 5;
            sim = ftol(v14f as f64) as i8 as i32 > r;
        }
        if !sim && dbs3_atributo(pessoa, 0x17) > sortear(5) + 15 {
            let x = m(2, 7) * f64_em(0x968f50); // fstp qword
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
        if dbs3_atributo(pessoa, a) > sortear(5) + 15 {
            f += bit;
        }
    }
    if b(0x21) > sortear(3) + 16 {
        f += 0x8000;
    }
    if dbs3_atributo(pessoa, 0xb) > sortear(4) + 14 {
        let x = m(2, 0xf); // fstp qword
        let r = sortear(20);
        if !(r as f64 * f64_em(0x967880) + f64_em(0x968eb0) >= x) {
            f += 0x10000;
        }
    }
    let v = m(2, 0x16);
    let limite = 1400 - dbs3_i16(j() + 0xb) / 10;
    if !(limite as f64 >= v) {
        let a = m(6, 2) as f32;
        let c = m(6, 1) as f32;
        let r = sortear(3);
        if !((r as f64 * f64_em(0x967880) + f64_em(0x968968)) * c as f64 >= a as f64) {
            f += 0x40000;
        }
    }
    if !(m(2, 0x15) >= f64_em(0x968930)) && m(2, 0x10) > f64_em(0x968ff8) {
        let x = m(2, 2) * f64_em(0x968930); // fstp qword
        if m(2, 1) > x {
            f += 0x80000;
        }
    }
    f
}

/// { 0x5470d0 } (cdecl): `staff_get_player_weakness_flags(pessoa, clube)`: os bits dos defeitos do jogador (+0x61) para
/// a mídia. Posições naturais (0x20, 0x10, 0x40, 0x80, 0x100, 0x200), 0x800000 pela soma das posições; depois os
/// sorteios (0x90cfc0, alguns com n = −50) contra bytes da pessoa e do jogador, atributos (0x543540), médias (0x7c1e30,
/// peso 6,5) e a palavra 0x62c3d0 ([0xb63d28]): 1 (+0x5d da pessoa e as médias 0xb, 0xa e 1), 2 (+0x3c contra um
/// sorteio de 4, 7, 8 ou 11 pelas +0x34 e 0x62c3d0), 4 (titular, vaga 0..10 da 0x8972c0 em [0xde1d08], com a nota
/// 0x6e6340 da casa 0x5b2ae0 abaixo de 150 + sorteio(40)), 0x400, 0x800, 8, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
/// 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000. A original guarda bytes e floats nos próprios argumentos
/// (gravados antes de lidos; a vaga passada à 0x5b2ae0 é o argumento `clube` com o byte da vaga por cima, e a cópia a
/// monta igual); a cópia não grava neles. Nulos: 0. Na mídia (0x72c1f0, 0x487c70).
unsafe extern "C" fn fraquezas_do_jogador(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 || pessoa == 0 {
        return 0;
    }
    let palavra: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x62c3d0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(0x8972c0usize);
    let formacao_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x896660usize);
    let casa_de: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    let j = || dbs3_u32(pessoa + 0x61);
    let b = |k: usize| sb(j(), k);
    let m = |tipo: u32, atributo: u32| dbs3_media(pessoa, tipo, atributo);
    let mut f = dbs3_posicoes(j(), [0, 0x20, 0x10, 0x40, 0x80, 0x100, 0x200], 0x800000);
    if (*((pessoa + 0x5d) as *const i8) as i32) < sortear(4) + 5 && (b(0x1c) > 15 || b(0x25) < 6) {
        let a = (m(2, 0xb) * f32_em(0x969584) as f64) as f32; // fstp dword
        let s = (m(2, 0xa) + a as f64) as f32; // fstp dword
        if !(m(2, 1) * f32_em(0x968f18) as f64 >= s as f64) {
            f += 1;
        }
    }
    let mut n = 4;
    let w = palavra(dbs3_u32(0xb63d28), dbs3_u32(pessoa) as i32) as i16 as i32;
    if w > sortear(1000) + 500 {
        n = 8;
    }
    if b(0x34) < sortear(12) {
        n += 3;
    }
    if b(0x3c) < sortear(n) + 8 && b(0xf) < 20 {
        f += 2;
    }
    let tatica = dbs3_u32(0xde1d08);
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
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = 70 - a;
        }
        let x = (m(2, 5) * f32_em(0x969580) as f64) as f32; // fstp dword
        ajuste = ajuste.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37) < sortear(-50).wrapping_add(ajuste) {
        f += 0x400;
    }
    if b(0x39) < sortear(-50) && b(0x3e) < sortear(-50) {
        let v = m(2, 0xe);
        let vf = v as f32; // fst dword
        if v > f64_em(0x9689a0) {
            let r = sortear(10);
            if (ftol(vf as f64) as i8 as i32) < r {
                f += 0x800;
            }
        }
    }
    let bd = dbs3_u32(0xb63d28);
    if bd != 0 {
        let w = palavra(bd, dbs3_u32(pessoa) as i32) as i16 as i32;
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
    if b(0x11) < 15 && dbs3_atributo(pessoa, 0xb) > sortear(4) + 14 {
        let v = m(2, 0xf);
        let vf = v as f32; // fst dword
        if v > f64_em(0x9689a0) && dbs3_atributo(pessoa, 0xb) as f64 * f64_em(0x969578) > vf as f64 {
            f += 0x40000;
        }
    }
    if b(0x28) > sortear(5) + 13 {
        f += 0x80000;
    }
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = dbs3_atributo(pessoa, 0xc);
        let t = (m(2, 0x10) + a as f64 - f32_em(0x968a78) as f64) as f32; // fstp dword
        if (sortear(20) + 20) as f64 > t as f64 {
            let x = m(6, 2); // fstp qword
            if m(6, 1) * f64_em(0x969510) > x {
                f += 0x100000;
            }
        }
    }
    if !(m(2, 0x15) >= f64_em(0x968930)) && m(2, 0x10) > f64_em(0x968ff8) {
        f += 0x200000;
    }
    if dbs3_atributo(pessoa, 0x14) < 7 || dbs3_atributo(pessoa, 0x14) < dbs3_i16(j() + 5) / 15 {
        f += 0x400000;
    }
    f
}

/// O lado a lado das duas, com o estado do sorteio. A 0x6e6340 (pela 0x62ed70) sorteia só na primeira consulta do
/// jogador e guarda: com essa memória fria a original sorteia a mais e a cópia, que roda depois, não, e os sorteios
/// seguintes saem diferentes. Então, divergindo, refaz uma vez a comparação a partir do mesmo estado (agora quente, a
/// original sem contar) e devolve o estado do sorteio da primeira, a que o jogo segue.
unsafe fn dbs3_comparar(o: impl Fn() -> u32, n: impl Fn() -> u32) -> (u32, u32, bool, String) {
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

unsafe extern "C" fn comparar_forcas_do_jogador(pessoa: usize, outro: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(747);
    let (r, _, igual, d) = dbs3_comparar(|| o(pessoa, outro), || forcas_do_jogador(pessoa, outro));
    PLACAR.registrar(igual, || format!("forcas_do_jogador({:x}, {:x}) {}", pessoa, outro, d));
    r
}

unsafe extern "C" fn comparar_fraquezas_do_jogador(pessoa: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(748);
    let (r, _, igual, d) = dbs3_comparar(|| o(pessoa, clube), || fraquezas_do_jogador(pessoa, clube));
    PLACAR.registrar(igual, || format!("fraquezas_do_jogador({:x}, {:x}) {}", pessoa, clube, d));
    r
}

// ---------- 236a leva: o clube e o seu time B: os fisioterapeutas, se é time B e as pessoas fora do elenco (Database.cpp) ----------

/// O time B ligado ao `clube` (club_get_linked_b_team 0x540a50 pelo endereço do jogo, com `ativo` = 1): o registro
/// do parceiro ou 0; `eh_a` recebe 1 (o clube é o A) ou 0 (o clube é o B) quando há parceiro. Recusado, a 0x540a50
/// zera a marca +0x244 do clube: na comparação a original vem antes e a cópia acha a marca já zerada (também 0).
unsafe fn dbt1_parceiro(clube: usize, eh_a: *mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    parceiro(clube, eh_a, 1)
}

/// { 0x53c9b0 } (cdecl, Database.cpp): `database_count_club_physios(clube)`: quantos dos 3 fisioterapeutas
/// (+0x1cf) do clube e do time B ligado (0x540a50) são pessoas válidas (não nulas, id +0 < [0xae2364] com sinal).
/// Só `al`. Clube nulo: a original (caixa e 0). Nas telas da pessoa (staff_screens, 0x8740c8 e 0x87412b).
unsafe extern "C" fn contar_fisioterapeutas(clube: usize) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u8>(749)(clube);
    }
    let validos = |c: usize| {
        (0..3).filter(|k| {
            let p = core::ptr::read_unaligned((c + 0x1cf + 4 * k) as *const usize);
            p != 0 && *(p as *const i32) < *(0xae2364 as *const i32)
        }).count() as u8
    };
    let b = dbt1_parceiro(clube, core::ptr::null_mut());
    validos(clube) + if b != 0 { validos(b) } else { 0 } // os do clube e os do time B (dbt)
}

unsafe extern "C" fn comparar_contar_fisioterapeutas(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(749)(clube);
    if clube == 0 {
        return r; // caixa: só a original
    }
    let n = contar_fisioterapeutas(clube);
    PLACAR.registrar(r == n, || format!("contar_fisioterapeutas({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5448c0 } (cdecl, Database.cpp): `club_is_b_team(clube)`: 1 se o clube tem parceiro (0x540a50 com `ativo`) e
/// é o B dele (`eh_a` = 0), senão 0 (o eax inteiro). A local `eh_a` só é lida com parceiro, e então a 0x540a50 a
/// escreveu. Clube nulo: a original (caixa e 0). Na tela da pessoa (staff_screens, 0x87272d).
unsafe extern "C" fn clube_e_time_b(clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(750)(clube);
    }
    let mut eh_a = 0u32;
    (dbt1_parceiro(clube, &mut eh_a) != 0 && eh_a == 0) as u32 // é o B (dbt)
}

unsafe extern "C" fn comparar_clube_e_time_b(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(750)(clube);
    if clube == 0 {
        return r;
    }
    let n = clube_e_time_b(clube);
    PLACAR.registrar(r == n, || format!("clube_e_time_b({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x544c70 } (cdecl, Database.cpp): `club_collect_non_playing_staff(clube, saida, max)`: põe em `saida` as
/// pessoas fora do elenco do clube e depois as do time B ligado (0x540a50 com `ativo`), na ordem dos cargos: os 6
/// de +0xbf (técnico, auxiliar, ...) e os 15 de +0x19f (treinadores, olheiros, fisioterapeutas), as não nulas, até
/// `max` (byte com sinal); devolve quantas em `al`. Clube nulo: a original (caixa e 0). Na aba de informações da
/// tela do clube (club_screen_draw_info_tab 0x460590, 0x4616fa).
unsafe extern "C" fn pessoas_fora_do_elenco(clube: usize, saida: *mut usize, max: u32) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(751)(clube, saida, max);
    }
    let max = max as i8;
    let mut n = 0i8;
    let b = dbt1_parceiro(clube, core::ptr::null_mut());
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

unsafe extern "C" fn comparar_pessoas_fora_do_elenco(clube: usize, saida: *mut usize, max: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(751);
    if clube == 0 {
        return o(clube, saida, max);
    }
    let reg = [(saida as usize, 4 * (max as i8).max(0) as usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, saida, max), || pessoas_fora_do_elenco(clube, saida, max));
    PLACAR.registrar(igual && r == n, || format!("pessoas_fora_do_elenco({:x}, {:x}, {}) original={} rust={} {:x?} x {:x?}", clube, saida as usize, max as i8, r, n, mo, mr));
    r
}

// ---------- 236a leva: os atributos vistos por quem olha: se o atributo do jogador aparece, o valor dele para quem olha e o nome do atributo de técnico (Database.cpp) ----------

/// { 0x543280 } (cdecl, Database.cpp): `staff_is_player_attribute_visible(quem_ve, pessoa, atributo)`: 1 se quem vê
/// enxerga o atributo (byte com sinal) da pessoa. Visível sem conta: `quem_ve` ou `pessoa` nulos, sem a tabela da
/// névoa ([0xae2a60]), névoa desligada ([0x9cc270] = 0), sem a ficha de quem vê (fog_of_war 0x5afde0, thiscall na
/// tabela) ou conhecimento (0x5b02f0, thiscall na ficha) 3. Conhecimento 1 ou 2: os atributos 1, 0x11, 0x15, 0x1a e
/// 0x1b sempre; os outros pelo resto do id da pessoa por 4 (`m`): o atributo deslocado (0, 1, 2 ou 3 conforme `m`)
/// divisível por 4 (conhecimento 1) ou por 2 (conhecimento 2), ou o seguinte (m = 0) / o mesmo (m ≠ 0) divisível
/// por 3. Outro conhecimento: 0. Só `al`. A 0x5b02f0 guarda o último clube achado em [0xae1080] (o mesmo valor
/// nas duas). Nas telas de busca e filtros (search_screens, SearchFilters 0x807450), da pessoa e do clube.
unsafe extern "C" fn atributo_visivel(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let tabela = *(0xae2a60 as *const usize);
    if quem_ve == 0 || pessoa == 0 || tabela == 0 || *(0x9cc270 as *const u8) == 0 {
        return 1;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecimento: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x5b02f0usize);
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

unsafe extern "C" fn comparar_atributo_visivel(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(752)(quem_ve, pessoa, atributo);
    let n = atributo_visivel(quem_ve, pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_visivel({:x}, {:x}, {}) original={} rust={}", quem_ve, pessoa, atributo as u8 as i8, r, n));
    r
}

/// { 0x543440 } (cdecl, Database.cpp): `staff_get_player_attribute_for_viewer(quem_ve, pessoa, atributo)`: o
/// atributo (1 a 0x1f, byte com sinal) do jogador (+0x61) da pessoa como quem vê o enxerga: com a névoa ligada
/// ([0x9cc270]), quem tem ficha (0x5afde0) e não vê o atributo (0x543280, pelo endereço do jogo) recebe 0xf9 (-7,
/// "desconhecido"); senão o valor de staff_compute_player_attribute (0x543540). Só `al`. Pessoa nula ou atributo fora
/// da faixa: a original (caixa e 0xff). Na comparação de jogadores e no perfil da pessoa (staff_screens 0x877470,
/// 0x879ac0, 0x87a754).
unsafe extern "C" fn atributo_para_quem_ve(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let x = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&x) {
        return orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(753)(quem_ve, pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x543540usize);
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if *(0x9cc270 as *const u8) != 0 {
        let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
        let visivel: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543280usize);
        if ficha(*(0xae2a60 as *const usize), quem_ve) != 0 && visivel(quem_ve, pessoa, atributo) == 0 {
            return 0xf9; // desconhecido (dbt)
        }
    }
    calcular(pessoa, jogador, atributo) as u8
}

unsafe extern "C" fn comparar_atributo_para_quem_ve(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize, u32) -> u8>(753)(quem_ve, pessoa, atributo);
    let x = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&x) {
        return r; // caixa: só a original
    }
    let n = atributo_para_quem_ve(quem_ve, pessoa, atributo);
    PLACAR.registrar(r == n, || format!("atributo_para_quem_ve({:x}, {:x}, {}) original={} rust={}", quem_ve, pessoa, x, r, n));
    r
}

/// Os nomes dos 12 atributos de técnico (abreviado, inteiro) da 0x543b10, pelos endereços dos textos no exe; o 8º
/// (gestão de pessoas) abreviado é um formato com o texto de [0xad9c64].
const DBT2_NOMES: [(usize, usize); 12] = [
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

/// { 0x543b10 } (cdecl, Database.cpp): `database_get_non_player_attribute_name(atributo, texto, tamanho, abreviado)`:
/// escreve em `texto` o nome do atributo de técnico (1 a 12, byte com sinal), abreviado ou inteiro, pelo texto
/// traduzido do Langlib (0x66f4e0, pelo endereço do jogo); devolve 1. Atributo fora da faixa, texto nulo ou tamanho
/// (curto com sinal) abaixo de 100: a original (caixa e 0). Nas telas de busca (search_screens) e no perfil da
/// pessoa (staff_screens 0x87ad97).
unsafe extern "C" fn nome_do_atributo_de_tecnico(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=12).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(754)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *(texto as *mut u8) = 0;
    let (curto, longo) = DBT2_NOMES[x as usize - 1];
    if abreviado == 0 {
        formatar(texto, longo);
    } else if x == 8 {
        formatar(texto, curto, 0xad9c64usize);
    } else {
        formatar(texto, curto); // (dbt)
    }
    1
}

unsafe extern "C" fn comparar_nome_do_atributo_de_tecnico(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(754);
    let x = atributo as u8 as i8;
    if !(1..=12).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return o(atributo, texto, tamanho, abreviado);
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let reg = [(texto, 100)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(atributo, texto, tamanho, abreviado),
        || nome_do_atributo_de_tecnico(atributo, texto, tamanho, abreviado));
    PLACAR.registrar(igual && r == n, || format!("nome_do_atributo_de_tecnico({}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}", x, texto, tamanho as u16 as i16, abreviado, r, n, mo, mr));
    r
}

// ---------- 236a leva: as ordens das pessoas pelo sobrenome e dos clubes pela reputação, e o clube da CEE (Database.cpp) ----------

/// { 0x53ff60 } (cdecl, Database.cpp): `database_compare_staff_by_surname(a, b)`: comparação do qsort de ponteiros
/// de pessoa pelo nome no estilo 4 ("sobrenome, nome", 0x53dba0 em 0x7d0 bytes) na comparação de textos 0x90c1d0
/// (sem maiúsculas, `dbr_comparar_textos`); pessoa nula vale o texto vazio (a original zera o primeiro byte das duas
/// locais antes). Na tela de oferta de contrato (contract_screens 0x4de5d0).
unsafe extern "C" fn ordem_das_pessoas_pelo_sobrenome(a: usize, b: usize) -> i32 {
    let formatar: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let mut na = [0u8; 0x7d0];
    let mut nb = [0u8; 0x7d0];
    let (pa, pb) = (*(a as *const usize), *(b as *const usize));
    if pa != 0 {
        formatar(pa, na.as_mut_ptr() as usize, 0x7d0, 4); // "sobrenome, nome" (dbt)
    }
    if pb != 0 {
        formatar(pb, nb.as_mut_ptr() as usize, 0x7d0, 4);
    }
    dbr_comparar_textos(na.as_ptr(), nb.as_ptr())
}

unsafe extern "C" fn comparar_ordem_das_pessoas_pelo_sobrenome(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(755)(a, b);
    let n = ordem_das_pessoas_pelo_sobrenome(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_pessoas_pelo_sobrenome({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53ffe0 } (cdecl, Database.cpp): `database_compare_clubs_by_reputation(a, b)`: comparação do qsort de
/// ponteiros de clube pela reputação (+0x80, curto com sinal; clube nulo = 0), a maior primeiro: a de `a` menor: 1;
/// maior: -1; iguais: 0. (A 0x540020, já em Rust, é a mesma ordem sobre ids.) No menu dos times do clube
/// (0x482070) e nas abas de Escolher Amistosos (find_screens 0x5a1480, 0x5a1e10, 0x5a2250).
unsafe extern "C" fn ordem_dos_ponteiros_de_clube_pela_reputacao(a: usize, b: usize) -> i32 {
    let reputacao = |p: usize| {
        let c = *(p as *const usize);
        if c == 0 { 0 } else { core::ptr::read_unaligned((c + 0x80) as *const i16) }
    };
    let (ra, rb) = (reputacao(a), reputacao(b));
    if ra < rb {
        1 // a de menor reputação depois (dbt)
    } else if ra > rb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn comparar_ordem_dos_ponteiros_de_clube_pela_reputacao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(756)(a, b);
    let n = ordem_dos_ponteiros_de_clube_pela_reputacao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_ponteiros_de_clube_pela_reputacao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x53d7d0 } (cdecl, Database.cpp): `database_club_nation_is_eec(clube)`: 1 se a nação do clube (+0x53) não é
/// nula e está na CEE (+0x7f = 2), senão 0 (o eax inteiro). O clube não é conferido (a original lê o +0x53 dele).
/// Na aba de informações da tela do clube (club_screens 0x462070, os estrangeiros).
unsafe extern "C" fn nacao_do_clube_na_cee(clube: usize) -> u32 {
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    (nacao != 0 && *((nacao + 0x7f) as *const u8) == 2) as u32 // na CEE (dbt)
}

unsafe extern "C" fn comparar_nacao_do_clube_na_cee(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(757)(clube);
    let n = nacao_do_clube_na_cee(clube);
    PLACAR.registrar(r == n, || format!("nacao_do_clube_na_cee({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 236a leva: as regiões em ordem de nome e a pessoa de um laço de opinião (Database.cpp) ----------

/// { 0x545dc0 } (cdecl, Database.cpp): `get_regions_sorted_by_name(idioma)`: enche o vetor global de 14 regiões
/// (0xae2104) com 1..14, guarda o `idioma` (byte) em [0xae232b] e o ordena pelo qsort da CRT (0x945333) com a
/// comparação 0x545e00 (pelo endereço do jogo); devolve o vetor. Nas telas de busca (search_screens 0x80970c,
/// 0x814b7e, 0x815d5e: o filtro de região).
unsafe extern "C" fn regioes_pelo_nome(idioma: u32) -> usize {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    for k in 0..14u8 {
        *((0xae2104 + k as usize) as *mut u8) = k + 1;
    }
    *(0xae232b as *mut u8) = idioma as u8;
    qsort(0xae2104, 14, 1, 0x545e00); // (dbt)
    0xae2104
}

unsafe extern "C" fn comparar_regioes_pelo_nome(idioma: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(u32) -> usize>(758);
    let reg = [(0xae2104, 14), (0xae232b, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(idioma), || regioes_pelo_nome(idioma));
    PLACAR.registrar(igual && r == n, || format!("regioes_pelo_nome({}) original={:x} rust={:x} {:x?} x {:x?}", idioma as u8, r, n, mo, mr));
    r
}

/// { 0x545e00 } (cdecl, Database.cpp): `compare_region_names(a, b)`: comparação do qsort de bytes de região pelo
/// nome (0x616f20, do index.cpp, em 0xc8 bytes, no idioma de [0xae232b]) na comparação de textos 0x90c1d0 (sem
/// maiúsculas); `a` nulo: 1; `b` nulo: -1. A 0x616f20 zera o primeiro byte do texto antes de tudo (as locais nunca
/// são lidas sem valor). Pela 0x545dc0.
unsafe extern "C" fn ordem_das_regioes_pelo_nome(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let nome: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x616f20usize);
    let idioma = *(0xae232b as *const u8) as u32;
    let mut na = [0u8; 0xc8];
    let mut nb = [0u8; 0xc8];
    nome(*(a as *const u8) as u32, na.as_mut_ptr() as usize, 0xc8, idioma);
    nome(*(b as *const u8) as u32, nb.as_mut_ptr() as usize, 0xc8, idioma);
    dbr_comparar_textos(na.as_ptr(), nb.as_ptr()) // (dbt)
}

unsafe extern "C" fn comparar_ordem_das_regioes_pelo_nome(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(759)(a, b);
    let n = ordem_das_regioes_pelo_nome(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_regioes_pelo_nome({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x549dd0 } (cdecl, Database.cpp): `staff_get_opinion_link_staff(pessoa, alvo)`: o outro lado do laço de opinião
/// da pessoa: no registro da pessoa ([0xae2d0c] + id × 79; pessoa nula ou id >= [0xae2364]: 0), com laço (+0x27 ≠
/// -1), se o alvo é o +0x37: a pessoa de índice +0x27 ([0xae23bc] + 110 × índice); senão, se o alvo é o +0x4b: a de
/// índice +0x3b; senão 0. Na aba de transferências do perfil da pessoa (staff_screens 0x87dd80).
unsafe extern "C" fn pessoa_da_opiniao(pessoa: usize, alvo: u32) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = *(pessoa as *const i32);
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let reg = (*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79));
    let ler = |d: usize| core::ptr::read_unaligned((reg + d) as *const u32);
    let pessoas = *(0xae23bc as *const usize);
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

unsafe extern "C" fn comparar_pessoa_da_opiniao(pessoa: usize, alvo: u32) -> usize {
    let r = orig::<unsafe extern "C" fn(usize, u32) -> usize>(760)(pessoa, alvo);
    let n = pessoa_da_opiniao(pessoa, alvo);
    PLACAR.registrar(r == n, || format!("pessoa_da_opiniao({:x}, {:x}) original={:x} rust={:x}", pessoa, alvo, r, n));
    r
}
