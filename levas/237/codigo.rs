// ---------- 237a leva: o nome pelos textos, o nome do cargo e o nome do idioma (Database.cpp) ----------

/// Texto não nulo e não vazio (os `repne scasb` da original).
unsafe fn dbu1_cheio(t: usize) -> bool {
    t != 0 && *(t as *const u8) != 0
}

/// strcpy (o `repne scasb` e os `rep movsd`/`rep movsb` da original).
unsafe fn dbu1_copiar(destino: usize, origem: usize) {
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

/// { 0x53e2e0 } (cdecl, Database.cpp): `database_format_name_from_strings(nome, sobrenome, comum, texto, tamanho,
/// forma)`: o nome da pessoa em `texto` pela forma (byte) a partir dos três textos (nulos ou vazios contam como
/// faltando). Formas 1 a 4: o nome comum, se houver; senão, com nome e sobrenome, 1 "%s %s", 2 "%c.%s", 3 "%s, %s"
/// (sobrenome primeiro), 4 "%s, %c"; senão o sobrenome, senão o nome. 5: o comum, o sobrenome ou o nome. 6: as
/// iniciais ("%c" do comum, "%c %c" do nome e do sobrenome, "%c" do sobrenome ou do nome). 7: "%s %s" primeiro, depois
/// o comum, o sobrenome ou o nome. Sem nada: o texto vazio. Devolve 1. Texto nulo, tamanho (palavra) < 0x69 ou forma
/// fora de 1..7: a original (caixa; as locais de 0x200 bytes só servem a ela). Chamada pela 0x53e140 (já em Rust, pelo
/// endereço do jogo: o desvio vale também lá), pelas telas do clube (0x45ccf0) e pelo manager_manager.
unsafe extern "C" fn nome_pelos_textos(nome: usize, sobrenome: usize, comum: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let f = forma as u8;
    if texto == 0 || (tamanho as u16 as i16) < 0x69 || !(1..=7).contains(&f) {
        return orig::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(761)(nome, sobrenome, comum, texto, tamanho, forma);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let (a, b, c) = (nome, sobrenome, comum);
    let inicial = |t: usize| *(t as *const i8) as i32;
    *(texto as *mut u8) = 0;
    let os_dois = dbu1_cheio(a) && dbu1_cheio(b);
    // o que sobra: o primeiro cheio da lista
    let um_de = |lista: &[usize]| {
        if let Some(&t) = lista.iter().find(|&&t| dbu1_cheio(t)) {
            dbu1_copiar(texto, t);
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
            if dbu1_cheio(c) {
                sprintf(texto, 0x99ec1c, inicial(c)); // "%c"
            } else if os_dois {
                sprintf(texto, 0x9b7418, inicial(a), inicial(b)); // "%c %c"
            } else if dbu1_cheio(b) {
                sprintf(texto, 0x99ec1c, inicial(b));
            } else if dbu1_cheio(a) {
                sprintf(texto, 0x99ec1c, inicial(a));
            }
        }
        5 => um_de(&[c, b, a]),
        _ => {
            if dbu1_cheio(c) {
                dbu1_copiar(texto, c);
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

unsafe extern "C" fn comparar_nome_pelos_textos(nome: usize, sobrenome: usize, comum: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(761);
    if texto == 0 || (tamanho as u16 as i16) < 0x69 || !(1..=7).contains(&(forma as u8)) {
        return o(nome, sobrenome, comum, texto, tamanho, forma); // caixa: só a original
    }
    // o texto produzido (até o tamanho dado, no máximo 128 bytes)
    let n = (tamanho as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(nome, sobrenome, comum, texto, tamanho, forma),
        || nome_pelos_textos(nome, sobrenome, comum, texto, tamanho, forma));
    PLACAR.registrar(igual && r == x, || format!("nome_pelos_textos(forma {}) original={} rust={} {:x?} x {:x?}", forma as u8, r, x, mo, mr));
    r
}

/// Os textos dos cargos 0..0x11 da 0x53ad00 (imediatos do código, pelos endereços no exe): curto minúsculo, curto
/// maiúsculo, inteiro minúsculo, inteiro maiúsculo e o gênero com o idioma 4 (0: não grava). Os curtos levam o
/// vazio 0xad9c64 ("<%s - COMMENT - short name>"); dos inteiros, só o do jogador (11). O 14 não existe (0).
const DBU1_CARGOS: [(usize, usize, usize, usize, u8); 18] = [
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

/// { 0x53ad00 } (cdecl, Database.cpp): `database_get_job_name(cargo, curto, maiuscula, genero)`: o nome traduzido
/// (0x66f4e0) do cargo (byte com sinal, 0..0x11 sem o 14) no texto [0xae19b4], que é o que devolve; curto ou inteiro e
/// minúsculo ou maiúsculo pelos dois inteiros. Com `genero` não nulo e o idioma (0x66f4d0) 4, grava o gênero: 4 para
/// diretor executivo, técnico, auxiliar, técnico dos reservas e olheiro; 1 para os outros; nada para o aposentado.
/// Cargo fora da faixa: 0, sem escrever. Nas telas do clube, de contratos, da pessoa e de busca, na mídia
/// (0x744f80, 0x74bbd0), nos regens (0x7a9650, 0x7aa4c0) e nas transferências (0x8d3520, 0x8dc370).
unsafe extern "C" fn nome_do_cargo(cargo: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let j = cargo as u8 as i8;
    if !(0..=0x11).contains(&j) || j == 14 {
        return 0;
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let (cm, cg, im, ig, g) = DBU1_CARGOS[j as usize];
    if curto != 0 {
        formatar(0xae19b4, if maiuscula != 0 { cg } else { cm }, 0xad9c64usize);
    } else if j == 11 {
        formatar(0xae19b4, if maiuscula != 0 { ig } else { im }, 0xad9c64usize); // "<%s - COMMENT - job name>"
    } else {
        formatar(0xae19b4, if maiuscula != 0 { ig } else { im });
    }
    if g != 0 && genero != 0 && idioma() == 4 {
        *(genero as *mut u8) = g;
    }
    0xae19b4
}

unsafe extern "C" fn comparar_nome_do_cargo(cargo: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(u32, u32, u32, usize) -> usize>(762);
    // o texto [0xae19b4] e o byte do gênero, se houver
    let regioes = [(0xae19b4usize, 100usize), (genero, 1)];
    let reg = if genero != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(cargo, curto, maiuscula, genero),
        || nome_do_cargo(cargo, curto, maiuscula, genero));
    PLACAR.registrar(igual && r == x, || format!("nome_do_cargo({}, {}, {}) original={:x} rust={:x} {:x?} x {:x?}", cargo as u8 as i8, curto, maiuscula, r, x, mo, mr));
    r
}

/// Os nomes dos idiomas 1..64 da 0x53cee0 (imediatos do código, pelos endereços no exe; o 34 repete o albanês),
/// todos com o vazio 0xad9c64 ("<%s - COMMENT - language>").
const DBU1_IDIOMAS: [usize; 64] = [
    0x9b73dc, 0x9b73bc, 0x9b7398, 0x9b7374, 0x9b7350, 0x9b7330, 0x9b7310, 0x9b72ec, // Albanian .. English
    0x9b72c8, 0x9b72a8, 0x9b7288, 0x9b7268, 0x9b7248, 0x9b7224, 0x9b7200, 0x9b71dc, // Finnish .. Japanese
    0x9b71bc, 0x9b719c, 0x9b7178, 0x9b7154, 0x9b7130, 0x9b710c, 0x9b70e8, 0x9b70c4, // Korean .. Spanish
    0x9b70a0, 0x9b707c, 0x9b705c, 0x9b7038, 0x9b7018, 0x9b6fb0, 0x9b6fd4, 0x9b6ff8, // Turkish .. Danish
    0x9b6f8c, 0x9b73dc, 0x9b6f68, 0x9b6f44, 0x9b6f24, 0x9b6f04, 0x9b6ee0, 0x9b6ebc, // Afrikaans, Albanian .. Georgian
    0x9b6e9c, 0x9b6e78, 0x9b6e58, 0x9b6e38, 0x9b6e18, 0x9b6df4, 0x9b6dcc, 0x9b6da8, // Hindi .. Lithuanian
    0x9b6d84, 0x9b6d60, 0x9b6d3c, 0x9b6d18, 0x9b6cf4, 0x9b6cd4, 0x9b6cb0, 0x9b6c90, // Macedonian .. Tajik
    0x9b6c70, 0x9b6c50, 0x9b6c30, 0x9b6c0c, 0x9b6bec, 0x9b6bc8, 0x9b6ba4, 0x9b6b84, // Tamil .. Zulu
];

/// { 0x53cee0 } (cdecl, Database.cpp): `database_get_language_name(idioma)`: o nome traduzido (0x66f4e0) do idioma
/// (byte com sinal, 1..64; fora disso "Unknown") no texto [0xae21d4] (100 bytes), que é o que devolve. Na busca
/// (0x813210) e na tela da pessoa (0x87dd80).
unsafe extern "C" fn nome_do_idioma(idioma: u32) -> usize {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let x = idioma as u8 as i8;
    let s = if (1..=64).contains(&x) { DBU1_IDIOMAS[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)
    formatar(0xae21d4, s, 0xad9c64usize);
    0xae21d4
}

unsafe extern "C" fn comparar_nome_do_idioma(idioma: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(u32) -> usize>(763);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae21d4, 100)], || o(idioma), || nome_do_idioma(idioma));
    PLACAR.registrar(igual && r == x, || format!("nome_do_idioma({}) original={:x} rust={:x} {:x?} x {:x?}", idioma as u8 as i8, r, x, mo, mr));
    r
}

// ---------- 237a leva: o nome da seleção, o nome do atributo do jogador e o texto das posições (Database.cpp) ----------

unsafe fn dbu2_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbu2_vazio(t: usize) -> bool {
    *(t as *const u8) == 0
}

/// strcat (os `repne scasb` e os `rep movsd`/`rep movsb` da original).
unsafe fn dbu2_juntar(destino: usize, origem: usize) {
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

/// { 0x542100 } (cdecl, Database.cpp): `club_get_national_team_name(clube, variante, texto, tamanho, forma, aspas)`:
/// o nome do clube de seleção em `texto` pela forma (byte, 1..4; fora disso não escreve). O nome é o de +0x38 com o
/// gênero +0x52 (byte com sinal), na forma 3 o de +4 com o gênero +0x37. Fora das seleções (id < [0xae235c] −
/// 2 × [0xae2350]): variante 0 copia o nome (forma 4: "Main Squad"); senão "Reserves" (formas 1 e 3), "Reserve" (2),
/// "Reserve Squad" (4). Seleção com variante: "B" ou "'B'" (com `aspas`), na forma 4 "B Squad"/"'B' Squad". Seleção
/// principal (id < clubes − nações): copia o nome (forma 4: "Squad"). As de base: sub-21 ou sub-23 pelo byte
/// [[0xb74318] + 0x36] + id da nação (+0x53), "U21"/"U23" (1), "Under 21"/"Under 23" (2), "Under 21's"/"Under 23's"
/// (3), "Under 21 Squad"/"Under 23 Squad" (4). Os formatos vão ao 0x66f4e0 com (gênero, gênero, nome). Devolve 1.
/// Tamanho < 0x47 ou clube nulo (com forma 1..4): a original (caixa). Nas telas do clube, da seleção, do técnico, na
/// escalação da partida (0x725f70, em Rust), no hall da fama, nas impressões e no Setup.
unsafe extern "C" fn nome_da_selecao(clube: usize, variante: u32, texto: usize, tamanho: i32, forma: u32, aspas: u32) -> u32 {
    let f = forma as u8 as i8;
    if tamanho < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return orig::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(764)(clube, variante, texto, tamanho, forma, aspas);
    }
    if !(1..=4).contains(&f) {
        return 1;
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let id = dbu2_u32(clube) as i32;
    let nacoes = dbu2_u32(0xae2350) as i32;
    let clubes = dbu2_u32(0xae235c) as i32;
    let (nome, g) = if f == 3 { (clube + 4, *((clube + 0x37) as *const i8) as i32) } else { (clube + 0x38, *((clube + 0x52) as *const i8) as i32) };
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
        let nacao = dbu2_u32(dbu2_u32(clube + 0x53));
        let sub23 = *(nacao.wrapping_add(dbu2_u32(dbu2_u32(0xb74318) + 0x36)) as *const u8) != 0;
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
            dbu2_juntar(texto, nome);
        }
    }
    1
}

unsafe extern "C" fn comparar_nome_da_selecao(clube: usize, variante: u32, texto: usize, tamanho: i32, forma: u32, aspas: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize, i32, u32, u32) -> u32>(764);
    let f = forma as u8 as i8;
    if tamanho < 0x47 || ((1..=4).contains(&f) && clube == 0) {
        return o(clube, variante, texto, tamanho, forma, aspas); // caixa: só a original
    }
    let n = (tamanho as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(clube, variante, texto, tamanho, forma, aspas),
        || nome_da_selecao(clube, variante, texto, tamanho, forma, aspas));
    PLACAR.registrar(igual && r == x, || format!("nome_da_selecao({:x}, {}, forma {}, {}) original={} rust={} {:x?} x {:x?}", clube, variante, f, aspas, r, x, mo, mr));
    r
}

/// Os nomes dos 31 atributos de jogador da 0x5428a0 (imediatos do código, pelos endereços no exe): (abreviado, leva o
/// vazio 0xad9c64, inteiro, leva o vazio).
const DBU2_ATRIBUTOS: [(usize, bool, usize, bool); 31] = [
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

/// { 0x5428a0 } (cdecl, Database.cpp; o mapa parte a rotina em 0x542f48, que é só o meio dela: a fronteira real vai
/// até 0x5431e0): `database_get_player_attribute_name(atributo, texto, tamanho, abreviado)`: zera o texto e escreve o
/// nome traduzido (0x66f4e0) do atributo de jogador (1..0x1f, byte com sinal), abreviado (`abreviado` não nulo) ou
/// inteiro; devolve 1. Atributo fora da faixa, texto nulo ou tamanho (curto com sinal) < 100: a original (caixa e 0).
/// Nas telas do clube, de busca, da pessoa e de treino.
unsafe extern "C" fn nome_do_atributo_do_jogador(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=0x1f).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(765)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *(texto as *mut u8) = 0;
    let (c, cv, l, lv) = DBU2_ATRIBUTOS[x as usize - 1];
    let (s, vazio) = if abreviado != 0 { (c, cv) } else { (l, lv) }; // (dbu)
    if vazio {
        formatar(texto, s, 0xad9c64usize);
    } else {
        formatar(texto, s);
    }
    1
}

unsafe extern "C" fn comparar_nome_do_atributo_do_jogador(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(765);
    let x = atributo as u8 as i8;
    if !(1..=0x1f).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return o(atributo, texto, tamanho, abreviado); // caixa: só a original
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 100)], || o(atributo, texto, tamanho, abreviado),
        || nome_do_atributo_do_jogador(atributo, texto, tamanho, abreviado));
    PLACAR.registrar(igual && r == n, || format!("nome_do_atributo_do_jogador({}, {}) original={} rust={} {:x?} x {:x?}", x, abreviado, r, n, mo, mr));
    r
}

/// Uma posição no texto da 0x53ee50: forma 1 o abreviado (com o vazio 0xad9c64), 2 e 3 o inteiro, com a barra antes
/// quando o texto já tem algo; as outras formas, a letra só com o texto ainda vazio. O traduzido passa pelo texto
/// global [0xde1f64] (0x66f4e0), como na original.
unsafe fn dbu2_posicao(buf: usize, f: u8, curto: usize, longo: usize, longo_vazio: bool, letra: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    match f {
        1 => {
            if !dbu2_vazio(buf) {
                dbu2_juntar(buf, 0x9b7630); // "/"
            }
            formatar(0xde1f64, curto, 0xad9c64usize);
            dbu2_juntar(buf, 0xde1f64);
        }
        2 | 3 => {
            if !dbu2_vazio(buf) {
                dbu2_juntar(buf, 0x9b7630);
            }
            if longo_vazio {
                formatar(0xde1f64, longo, 0xad9c64usize);
            } else {
                formatar(0xde1f64, longo);
            }
            dbu2_juntar(buf, 0xde1f64);
        }
        _ => {
            if dbu2_vazio(buf) {
                dbu2_juntar(buf, letra);
            }
        }
    }
}

/// As posições da 0x53ee50 (abreviado, inteiro, o inteiro leva o vazio, letra), imediatos do código.
const DBU2_GOLEIRO: (usize, usize, bool, usize) = (0x9b7610, 0x9b7600, false, 0x9b760c); // GK, Goalkeeper, A
const DBU2_LIBERO: (usize, usize, bool, usize) = (0x9b75e4, 0x9b75dc, false, 0x98b3f0); // SW, Sweeper, B
const DBU2_DEFENSOR: (usize, usize, bool, usize) = (0x9b75c0, 0x9b75b0, false, 0x9b75bc); // D, Defender, C
const DBU2_VOLANTE: (usize, usize, bool, usize) = (0x9b7588, 0x9b756c, false, 0x9b7584); // DM, Defensive Midfielder, D
const DBU2_MEIA_ATACANTE: (usize, usize, bool, usize) = (0x9b7544, 0x9b7528, false, 0x9b7540); // AM, Attacking Midfielder, F
const DBU2_MEIA: (usize, usize, bool, usize) = (0x9b7508, 0x9b74f8, false, 0x9b7504); // M, Midfielder, E
const DBU2_ATACANTE: (usize, usize, bool, usize) = (0x9b74dc, 0x9b74d0, false, 0x9b74d8); // S, Striker, H
const DBU2_AVANTE: (usize, usize, bool, usize) = (0x9b74b4, 0x9b7484, true, 0x9b74b0); // F, Forward<%s>, G

/// { 0x53ee50 } (cdecl, Database.cpp): `database_format_player_positions(jogador, texto, tamanho, forma)`: o texto das
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
unsafe extern "C" fn texto_das_posicoes(jogador: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if texto == 0 || jogador == 0 || (tamanho as u16 as i16) < 0x1e {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(766)(jogador, texto, tamanho, forma);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    *(texto as *mut u8) = 0;
    let f = forma as u8;
    let nota = |o: usize| *((jogador + o) as *const i8) as i32;
    let mut linha = [0u8; 0x7d0];
    let mut lados = [0u8; 0x7c0];
    let buf = linha.as_mut_ptr() as usize;
    let pos = |p: (usize, usize, bool, usize)| dbu2_posicao(buf, f, p.0, p.1, p.2, p.3);
    let mut minimo: u8 = 0xf;
    loop {
        *(buf as *mut u8) = 0;
        let t = minimo as i32;
        if nota(0xf) >= t {
            pos(DBU2_GOLEIRO);
        }
        if nota(0x10) >= t {
            pos(DBU2_LIBERO);
        }
        if nota(0x11) >= t {
            pos(DBU2_DEFENSOR);
        }
        if nota(0x12) >= t {
            pos(DBU2_VOLANTE);
        } else if nota(0x14) >= t && nota(0x13) >= t {
            pos(DBU2_MEIA_ATACANTE);
        } else if nota(0x13) >= t {
            pos(DBU2_MEIA);
        }
        if nota(0x15) >= t {
            if nota(0x14) >= t || nota(0x17) >= t || nota(0x18) >= t || nota(0x1a) >= t {
                pos(DBU2_AVANTE);
            } else {
                pos(DBU2_ATACANTE);
            }
        }
        if nota(0xf) >= t {
            // goleiro: só a linha
            dbu2_juntar(texto, buf);
            return 1;
        }
        minimo = minimo.wrapping_sub(1);
        if !dbu2_vazio(buf) || minimo < 0xa {
            break;
        }
    }
    if dbu2_vazio(buf) {
        if nota(0x14) >= 0xf {
            *((jogador + 0x13) as *mut u8) = 0xf; // a original grava no jogador
        }
        if f == 2 || f == 3 {
            formatar(0xde1f64, DBU2_MEIA_ATACANTE.1);
        } else {
            formatar(0xde1f64, DBU2_MEIA_ATACANTE.0, 0xad9c64usize);
        }
        dbu2_juntar(buf, 0xde1f64);
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
                    dbu2_juntar(lado, 0xde1f64);
                }
                2 => {
                    if !dbu2_vazio(lado) {
                        dbu2_juntar(lado, 0x9b7630);
                    }
                    formatar(0xde1f64, longo); // Right, Left, Centre
                    dbu2_juntar(lado, 0xde1f64);
                }
                4 => {
                    if dbu2_vazio(lado) {
                        dbu2_juntar(lado, letra);
                    }
                }
                _ => {}
            }
        }
        // a original repete a volta (de 15 a 10) enquanto os lados ficam vazios; as notas são as mesmas
        volta = volta.wrapping_sub(1);
        if !dbu2_vazio(lado) || volta < 0xa {
            break;
        }
    }
    if f == 2 && !dbu2_vazio(lado) {
        sprintf(texto, 0x9a3d8c, buf, lado); // "%s (%s)" (dbu)
    } else if !dbu2_vazio(lado) {
        sprintf(texto, 0x98f0b8, buf, lado); // "%s %s"
    } else {
        dbu2_juntar(texto, buf);
    }
    1
}

unsafe extern "C" fn comparar_texto_das_posicoes(jogador: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(766);
    if texto == 0 || jogador == 0 || (tamanho as u16 as i16) < 0x1e {
        return o(jogador, texto, tamanho, forma); // caixa: só a original
    }
    // o texto, as notas do jogador (a original pode gravar +0x13) e o começo do texto global [0xde1f64]
    let n = (tamanho as u16 as usize).min(128);
    let reg = [(texto, n), (jogador + 0xf, 12), (0xde1f64, 64)];
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(jogador, texto, tamanho, forma),
        || texto_das_posicoes(jogador, texto, tamanho, forma));
    PLACAR.registrar(igual && r == x, || format!("texto_das_posicoes({:x}, forma {}) original={} rust={} {:x?} x {:x?}", jogador, forma as u8, r, x, mo, mr));
    r
}

// ---------- 237a leva: o nome da região e o adjetivo do continente (index.cpp) ----------

unsafe fn dbu3_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// strcpy (o `repne scasb` e os `rep movsd`/`rep movsb` da original).
unsafe fn dbu3_copiar(destino: usize, origem: usize) {
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

/// Os nomes das regiões 1..14 da 0x616f20 (imediatos do código, pelos endereços no exe): (adjetivo, substantivo). A
/// 3 (Caribe) tem textos próprios.
const DBU3_REGIOES: [(usize, usize); 14] = [
    (0x9e20bc, 0x9d9458), // African / Africa
    (0x9e20b4, 0x9d9450), // Asian / Asia
    (0, 0),               // Caribbean (à parte)
    (0x9e2040, 0x9e1f84), // Central American / Central America
    (0x9e202c, 0x9e1f74), // Central European / Central Europe
    (0x9e2018, 0x9e1f64), // Eastern European / Eastern Europe
    (0x9e2008, 0x9e1f58), // Middle Eastern / Middle East
    (0x9e1ff8, 0x9e1f48), // North African / North Africa
    (0x9e1fe8, 0x9d9438), // North American / North America
    (0x9e1fe0, 0x9d9430), // Oceanic / Oceania
    (0x9e1fd0, 0x9e1f3c), // Scandinavian / Scandinavia
    (0x9e1fc0, 0x9d9420), // South American / South America
    (0x9e1fac, 0x9e1f2c), // Southern European / Southern Europe
    (0x9e1fa0, 0x9e1f1c), // UK & Irish / UK & Ireland
];

/// { 0x616f20 } (cdecl, index.cpp): `get_region_name(regiao, texto, tamanho, forma)`: zera o texto e escreve o nome da
/// região (1..14, byte com sinal). Forma (byte) diferente de 0xff com o idioma (0x66f4d0) 4, 5 ou 9: o texto da
/// tabela do idioma, lida pelo endereço ([0x9d0518 + 4 × (4 × região + f)], [0x9d06d4 + 4 × (5 × região + f)],
/// [0x9d05f8 + 4 × (4 × região + f)], com f a forma do adjetivo 0x545a60, já em Rust, chamada pelo endereço do
/// jogo), copiado. Senão, pelo 0x66f4e0: nos idiomas 2, 4, 5 e 9 o adjetivo com a forma 0xff e o substantivo com as
/// outras; nos demais o contrário; o Caribe: "Caribbean" nos idiomas 2, 4, 5 e 9, senão os textos de região (forma
/// 0xff) ou de regionalidade, com o vazio 0xad9c64. Devolve 1. Texto nulo, região fora da faixa ou tamanho (curto
/// com sinal) < 100: a original (caixa). Na ordem das regiões (0x545dc0, em Rust), na busca de jogadores
/// (player_search, search_screens) e em telas sem módulo (0x7db360, 0x7e13d0, 0x7e29e0).
unsafe extern "C" fn nome_da_regiao(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let r = regiao as u8 as i8;
    if texto == 0 || !(1..=14).contains(&r) || (tamanho as u16 as i16) < 100 {
        return orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(767)(regiao, texto, tamanho, forma);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let adjetivo: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x545a60usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *(texto as *mut u8) = 0;
    let r = r as i32;
    let fb = forma as u8;
    let i = idioma(); // a original relê [0x9eb2dc] a cada pergunta (o mesmo valor)
    if fb != 0xff {
        // (tabela, linha) do idioma: as tabelas do exe, lidas pelo endereço
        let tabela = match i {
            4 => Some((0x9d0518usize, 4)),
            5 => Some((0x9d06d4usize, 5)),
            9 => Some((0x9d05f8usize, 4)),
            _ => None,
        };
        if let Some((base, linha)) = tabela {
            let f = adjetivo(forma, i as u32) as i8 as i32;
            let k = r.wrapping_mul(linha).wrapping_add(f);
            dbu3_copiar(texto, dbu3_u32(base.wrapping_add((k as usize).wrapping_mul(4))));
            return 1;
        }
    }
    let grupo = i == 5 || i == 2 || i == 4 || i == 9;
    if r == 3 {
        if grupo {
            formatar(texto, 0x9e1f94); // Caribbean
        } else if fb == 0xff {
            formatar(texto, 0x9e2054, 0xad9c64usize); // Caribbean<%s COMMENT Region as in Europe>
        } else {
            formatar(texto, 0x9e2080, 0xad9c64usize); // Caribbean<%s COMMENT Regionality as in European>
        }
    } else {
        let (adj, nome) = DBU3_REGIOES[r as usize - 1];
        // nos idiomas do grupo o adjetivo vem com a forma 0xff; nos outros, com as outras formas
        let usa_adjetivo = if grupo { fb == 0xff } else { fb != 0xff }; // (dbu)
        formatar(texto, if usa_adjetivo { adj } else { nome });
    }
    1
}

unsafe extern "C" fn comparar_nome_da_regiao(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(767);
    let r = regiao as u8 as i8;
    if texto == 0 || !(1..=14).contains(&r) || (tamanho as u16 as i16) < 100 {
        return o(regiao, texto, tamanho, forma); // caixa: só a original
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito)
    let (x, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 100)], || o(regiao, texto, tamanho, forma),
        || nome_da_regiao(regiao, texto, tamanho, forma));
    PLACAR.registrar(igual && x == n, || format!("nome_da_regiao({}, forma {}) original={} rust={} {:x?} x {:x?}", r, forma as u8, x, n, mo, mr));
    x
}

/// Os adjetivos dos continentes da 0x6177f0 por idioma (imediatos do código, pelos endereços no exe; levantados
/// seguindo os desvios da original): linhas África [0x9cfa08], Ásia [0x9cfa0c], Europa [0x9cfa10], América do Norte
/// [0x9cfa14], Oceania [0x9cfa18] e os outros; colunas a forma do adjetivo 0, 1, 2, 3 e as outras.
const DBU3_CONTINENTES_9: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38, 0x9d0d38],
    [0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08, 0x9d0d08],
    [0x9e2120, 0x9e2114, 0x9e2108, 0x9e20fc, 0x9e20fc],
    [0x9d0b94, 0x9d0b84, 0x9d0b70, 0x9d0b5c, 0x9d0b5c],
    [0x9d0b50, 0x9d0b44, 0x9d0b38, 0x9d0b2c, 0x9d0b2c],
    [0x9d0ae4, 0x9d0ad4, 0x9d0ac4, 0x9d0ab4, 0x9d0ab4],
];
const DBU3_CONTINENTES_5: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38],
    [0x9d0d2c, 0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08],
    [0x9e20f4, 0x9e20f4, 0x9e20ec, 0x9e20e0, 0x9e20d4],
    [0x9d08cc, 0x9d08cc, 0x9d08bc, 0x9d08ac, 0x9d089c],
    [0x9d0890, 0x9d0890, 0x9d0884, 0x9d0878, 0x9d086c],
    [0x9d085c, 0x9d085c, 0x9d084c, 0x9d083c, 0x9d082c],
];
const DBU3_CONTINENTES_4: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d1058, 0x9d104c, 0x9d1040, 0x9d1040],
    [0x9d1034, 0x9d1028, 0x9d101c, 0x9d1010, 0x9d1010],
    [0x9e20f4, 0x9e20cc, 0x9e20ec, 0x9e20c4, 0x9e20c4],
    [0x9d0eb0, 0x9d0ea0, 0x9d0e90, 0x9d0e80, 0x9d0e80],
    [0x9d0e74, 0x9d0e68, 0x9d0e5c, 0x9d0e50, 0x9d0e50],
    [0x9d0e10, 0x9d0e00, 0x9d0df0, 0x9d0de0, 0x9d0de0],
];

/// { 0x6177f0 } (cdecl, index.cpp; o mapa dá 629 bytes, a rotina vai até 0x6181c5: 2518): `get_continent_adjective(
/// continente, texto, tamanho, forma)`: zera o texto e copia o adjetivo do continente. Forma (byte) 0 ou idioma
/// (0x66f4d0) fora de 9, 5 e 4: o adjetivo do próprio registro (+0x23). Senão o texto do idioma pelo continente (o id
/// +0 comparado às chaves [0x9cfa08..0x9cfa18], nessa ordem) e pela forma do adjetivo (0x545a60, já em Rust, pelo
/// endereço do jogo). Devolve 1. Continente ou texto nulos ou tamanho (curto com sinal) < 50: a original (0). Na
/// busca (find_screens), no Menubar, nos jogos do dia (0x717cc0, em Rust) e em 0x7dd400.
unsafe extern "C" fn adjetivo_do_continente(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(768)(continente, texto, tamanho, forma);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let adjetivo: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x545a60usize);
    *(texto as *mut u8) = 0;
    let proprio = continente + 0x23; // o adjetivo do registro (dbu)
    let tabela = if forma as u8 == 0 {
        None
    } else {
        match idioma() {
            9 => Some((&DBU3_CONTINENTES_9, 9)),
            5 => Some((&DBU3_CONTINENTES_5, 5)),
            4 => Some((&DBU3_CONTINENTES_4, 4)),
            _ => None,
        }
    };
    let s = match tabela {
        None => proprio,
        Some((t, i)) => {
            let f = adjetivo(forma, i) as i8;
            let id = dbu3_u32(continente);
            let linha = [0x9cfa08usize, 0x9cfa0c, 0x9cfa10, 0x9cfa14, 0x9cfa18].iter().position(|&g| dbu3_u32(g) == id).unwrap_or(5);
            t[linha][if (0..=3).contains(&f) { f as usize } else { 4 }]
        }
    };
    dbu3_copiar(texto, s);
    1
}

unsafe extern "C" fn comparar_adjetivo_do_continente(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(768);
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return o(continente, texto, tamanho, forma);
    }
    // os 50 primeiros bytes (o tamanho mínimo aceito)
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 50)], || o(continente, texto, tamanho, forma),
        || adjetivo_do_continente(continente, texto, tamanho, forma));
    PLACAR.registrar(igual && r == n, || format!("adjetivo_do_continente({:x}, forma {}) original={} rust={} {:x?} x {:x?}", continente, forma as u8, r, n, mo, mr));
    r
}
