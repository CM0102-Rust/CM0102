// ---------- %%LEVA%%a leva: o nome pelos ids, o nome do oficial e o texto da máscara da posição (Database.cpp) ----------

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
        return orig::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(%%I0%%)(nome, sobrenome, comum, texto, tamanho, forma, pessoa);
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
    let o = orig::<unsafe extern "C" fn(i32, i32, i32, usize, u32, u32, i32) -> u32>(%%I0%%);
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
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I1%%);
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
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I1%%);
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
    let o = orig::<unsafe extern "C" fn(u32, usize, u32) -> u8>(%%I2%%);
    if texto == 0 {
        return o(mascara, texto, maiuscula);
    }
    // sem tamanho: os nomes traduzidos cabem em 128
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 128)], || o(mascara, texto, maiuscula),
        || texto_da_mascara_da_posicao(mascara, texto, maiuscula));
    PLACAR.registrar(igual && r == x, || format!("texto_da_mascara_da_posicao({:x}, {}) original={} rust={} {:x?} x {:x?}", mascara, maiuscula as u8, r, x, mo, mr));
    r
}
