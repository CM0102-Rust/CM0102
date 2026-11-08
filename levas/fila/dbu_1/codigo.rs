// ---------- %%LEVA%%a leva: o nome pelos textos, o nome do cargo e o nome do idioma (Database.cpp) ----------

/// Texto não nulo e não vazio (os `repne scasb` da original).
unsafe fn dbu1_cheio(t: usize) -> bool {
    t != 0 && *(t as *const u8) != 0
}

/// strcpy (o `repne scasb` e os `rep movsd`/`rep movsb` da original).
unsafe fn copy_c_string(destino: usize, origem: usize) {
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
/// fora de 1..7: a original (caixa; as locais de 0x200 bytes só servem a ela). TapeCall pela 0x53e140 (já em Rust, pelo
/// endereço do jogo: o desvio vale também lá), pelas telas do clube (0x45ccf0) e pelo manager_manager.
unsafe extern "C" fn database_format_name_from_strings(nome: usize, sobrenome: usize, comum: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let f = forma as u8;
    if texto == 0 || (tamanho as u16 as i16) < 0x69 || !(1..=7).contains(&f) {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(%%I0%%)(nome, sobrenome, comum, texto, tamanho, forma);
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let (a, b, c) = (nome, sobrenome, comum);
    let inicial = |t: usize| *(t as *const i8) as i32;
    *(texto as *mut u8) = 0;
    let os_dois = dbu1_cheio(a) && dbu1_cheio(b);
    // o que sobra: o primeiro cheio da lista
    let um_de = |lista: &[usize]| {
        if let Some(&t) = lista.iter().find(|&&t| dbu1_cheio(t)) {
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

unsafe extern "C" fn compare_database_format_name_from_strings(nome: usize, sobrenome: usize, comum: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, usize, u32, u32) -> u32>(%%I0%%);
    if texto == 0 || (tamanho as u16 as i16) < 0x69 || !(1..=7).contains(&(forma as u8)) {
        return o(nome, sobrenome, comum, texto, tamanho, forma); // caixa: só a original
    }
    // o texto produzido (até o tamanho dado, no máximo 128 bytes)
    let n = (tamanho as u16 as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, n)], || o(nome, sobrenome, comum, texto, tamanho, forma),
        || database_format_name_from_strings(nome, sobrenome, comum, texto, tamanho, forma));
    SCOREBOARD.record(igual && r == x, || format!("database_format_name_from_strings(forma {}) original={} rust={} {:x?} x {:x?}", forma as u8, r, x, mo, mr));
    r
}

/// Os textos dos cargos 0..0x11 da 0x53ad00 (imediatos do código, pelos endereços no exe): curto minúsculo, curto
/// maiúsculo, inteiro minúsculo, inteiro maiúsculo e o gênero com o idioma 4 (0: não grava). Os curtos levam o
/// vazio 0xad9c64 ("<%s - COMMENT - short name>"); dos inteiros, só o do jogador (11). O 14 não existe (0).
const JOB_TITLE_TEXTS: [(usize, usize, usize, usize, u8); 18] = [
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
unsafe extern "C" fn database_get_job_name(cargo: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let j = cargo as u8 as i8;
    if !(0..=0x11).contains(&j) || j == 14 {
        return 0;
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let (cm, cg, im, ig, g) = JOB_TITLE_TEXTS[j as usize];
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

unsafe extern "C" fn compare_database_get_job_name(cargo: u32, curto: u32, maiuscula: u32, genero: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, usize) -> usize>(%%I1%%);
    // o texto [0xae19b4] e o byte do gênero, se houver
    let regioes = [(0xae19b4usize, 100usize), (genero, 1)];
    let reg = if genero != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(reg, || o(cargo, curto, maiuscula, genero),
        || database_get_job_name(cargo, curto, maiuscula, genero));
    SCOREBOARD.record(igual && r == x, || format!("database_get_job_name({}, {}, {}) original={:x} rust={:x} {:x?} x {:x?}", cargo as u8 as i8, curto, maiuscula, r, x, mo, mr));
    r
}

/// Os nomes dos idiomas 1..64 da 0x53cee0 (imediatos do código, pelos endereços no exe; o 34 repete o albanês),
/// todos com o vazio 0xad9c64 ("<%s - COMMENT - language>").
const LANGUAGE_NAMES: [usize; 64] = [
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
unsafe extern "C" fn database_get_language_name(idioma: u32) -> usize {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let x = idioma as u8 as i8;
    let s = if (1..=64).contains(&x) { LANGUAGE_NAMES[x as usize - 1] } else { 0x9b6b60 }; // Unknown (dbu)
    formatar(0xae21d4, s, 0xad9c64usize);
    0xae21d4
}

unsafe extern "C" fn compare_database_get_language_name(idioma: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I2%%);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae21d4, 100)], || o(idioma), || database_get_language_name(idioma));
    SCOREBOARD.record(igual && r == x, || format!("database_get_language_name({}) original={:x} rust={:x} {:x?} x {:x?}", idioma as u8 as i8, r, x, mo, mr));
    r
}
