// ---------- %%LEVA%%a leva: o nome da região e o adjetivo do continente (index.cpp) ----------

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
const REGION_NAMES: [(usize, usize); 14] = [
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
unsafe extern "C" fn get_region_name(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let r = regiao as u8 as i8;
    if texto == 0 || !(1..=14).contains(&r) || (tamanho as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I0%%)(regiao, texto, tamanho, forma);
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
        let (adj, nome) = REGION_NAMES[r as usize - 1];
        // nos idiomas do grupo o adjetivo vem com a forma 0xff; nos outros, com as outras formas
        let usa_adjetivo = if grupo { fb == 0xff } else { fb != 0xff }; // (dbu)
        formatar(texto, if usa_adjetivo { adj } else { nome });
    }
    1
}

unsafe extern "C" fn comparar_get_region_name(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I0%%);
    let r = regiao as u8 as i8;
    if texto == 0 || !(1..=14).contains(&r) || (tamanho as u16 as i16) < 100 {
        return o(regiao, texto, tamanho, forma); // caixa: só a original
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito)
    let (x, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, 100)], || o(regiao, texto, tamanho, forma),
        || get_region_name(regiao, texto, tamanho, forma));
    PLACAR.record(igual && x == n, || format!("get_region_name({}, forma {}) original={} rust={} {:x?} x {:x?}", r, forma as u8, x, n, mo, mr));
    x
}

/// Os adjetivos dos continentes da 0x6177f0 por idioma (imediatos do código, pelos endereços no exe; levantados
/// seguindo os desvios da original): linhas África [0x9cfa08], Ásia [0x9cfa0c], Europa [0x9cfa10], América do Norte
/// [0x9cfa14], Oceania [0x9cfa18] e os outros; colunas a forma do adjetivo 0, 1, 2, 3 e as outras.
const CONTINENT_ADJECTIVES: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38, 0x9d0d38],
    [0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08, 0x9d0d08],
    [0x9e2120, 0x9e2114, 0x9e2108, 0x9e20fc, 0x9e20fc],
    [0x9d0b94, 0x9d0b84, 0x9d0b70, 0x9d0b5c, 0x9d0b5c],
    [0x9d0b50, 0x9d0b44, 0x9d0b38, 0x9d0b2c, 0x9d0b2c],
    [0x9d0ae4, 0x9d0ad4, 0x9d0ac4, 0x9d0ab4, 0x9d0ab4],
];
const CONTINENT_ADJECTIVES_LANG5: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38],
    [0x9d0d2c, 0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08],
    [0x9e20f4, 0x9e20f4, 0x9e20ec, 0x9e20e0, 0x9e20d4],
    [0x9d08cc, 0x9d08cc, 0x9d08bc, 0x9d08ac, 0x9d089c],
    [0x9d0890, 0x9d0890, 0x9d0884, 0x9d0878, 0x9d086c],
    [0x9d085c, 0x9d085c, 0x9d084c, 0x9d083c, 0x9d082c],
];
const CONTINENT_NAMES_LANGUAGE_4: [[usize; 5]; 6] = [
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
unsafe extern "C" fn get_continent_adjective(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I1%%)(continente, texto, tamanho, forma);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let adjetivo: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x545a60usize);
    *(texto as *mut u8) = 0;
    let proprio = continente + 0x23; // o adjetivo do registro (dbu)
    let tabela = if forma as u8 == 0 {
        None
    } else {
        match idioma() {
            9 => Some((&CONTINENT_ADJECTIVES, 9)),
            5 => Some((&CONTINENT_ADJECTIVES_LANG5, 5)),
            4 => Some((&CONTINENT_NAMES_LANGUAGE_4, 4)),
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

unsafe extern "C" fn comparar_get_continent_adjective(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(%%I1%%);
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return o(continente, texto, tamanho, forma);
    }
    // os 50 primeiros bytes (o tamanho mínimo aceito)
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, 50)], || o(continente, texto, tamanho, forma),
        || get_continent_adjective(continente, texto, tamanho, forma));
    PLACAR.record(igual && r == n, || format!("get_continent_adjective({:x}, forma {}) original={} rust={} {:x?} x {:x?}", continente, forma as u8, r, n, mo, mr));
    r
}
