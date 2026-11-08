use super::*;

// ---------- 240a leva: o adjetivo de nacionalidade pelo idioma e pela forma (index.cpp) ----------

// As tabelas da nation_get_nationality_adjective, geradas do exe por `python engenharia-reversa/adjetivos_nacao.py`
// (rust/src/adjetivos_nacao.rs).
include!("../adjetivos_nacao.rs");

/// the nation's adjective into the text (size >= 50): with gender 0 or outside languages 9, 4, 5, 10 and 6 the nation's
/// name; in those, the form from 0x545a60 and the chains of 937 nations (generated from the exe by adjetivos_nacao.py,
/// texts by address), with the name if the text stays empty (except in languages 4 and 5); returns 1 (0 with a null
/// nation or text or a short size)
/// Original: `nation_get_nationality_adjective` (0x6181d0, index.cpp, cdecl).
///
/// [pt] { 0x6181d0 } (cdecl, index.cpp; 0x6181d0..0x62a67a, o mapa de funções corta em 26.014 bytes):
/// `nation_get_nationality_adjective(nacao, texto, tamanho, genero)`: o adjetivo da nação em `texto`. Zera o texto
/// antes de tudo; nação nula, texto nulo ou tamanho (curto com sinal) < 50: 0. Gênero (byte) 0: o nome da nação
/// (+0x57). Nos idiomas 9, 4, 5, 10 e 6 (0x66f4d0) a forma vem da 0x545a60(gênero, idioma) e as cadeias da tabela dão o
/// texto: em cada cadeia a primeira chave igual ao id (+0) copia a fonte da forma (0..4, as outras pela 6ª coluna; 0
/// não copia, 1 é o nome) e passa à cadeia seguinte; no fim, com o texto vazio, o nome (nos idiomas 4 e 5 não: fica
/// vazio). Nos outros idiomas (o inglês, 0), o nome. Devolve 1. Sem SEH, sem locais, sem sorteio.
pub(crate) unsafe extern "C" fn adjetivo_da_nacao(nacao: usize, texto: usize, tamanho: u32, genero: u32) -> u32 {
    core::ptr::write_volatile(texto as *mut u8, 0); // antes dos testes, como a original (texto nulo: a mesma falha)
    if nacao == 0 || texto == 0 || (tamanho as u16 as i16) < 50 {
        return 0;
    }
    let nome = nacao + nation::NATIONALITY; // o nome da nação
    if genero as u8 != 0 {
        let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
        let i = idioma();
        if let Some(&(_, nome_se_vazio, cadeias)) = ADJETIVOS_DA_NACAO.iter().find(|x| x.0 == i) {
            let forma: unsafe extern "C" fn(u32, i32) -> u32 = core::mem::transmute(FN_GET_ADJECTIVE_FORM_INDEX);
            let f = forma(genero, i) as u8 as usize;
            let id = core::ptr::read_unaligned(nacao as *const i32);
            for cadeia in cadeias {
                if let Some((_, fontes)) = cadeia.iter().find(|e| *(e.0 as *const i32) == id) {
                    match fontes[f.min(5)] { // as formas 5..255 pela última coluna
                        0 => {}
                        1 => dbu1_copiar(texto, nome),
                        t => dbu1_copiar(texto, t as usize),
                    }
                }
            }
            if !nome_se_vazio || *(texto as *const u8) != 0 {
                return 1;
            }
        }
    }
    dbu1_copiar(texto, nome);
    1
}

pub(crate) unsafe extern "C" fn comparar_adjetivo_da_nacao(nacao: usize, texto: usize, tamanho: u32, genero: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_ADJETIVO_DA_NACAO);
    if texto == 0 {
        return o(nacao, texto, tamanho, genero);
    }
    // o texto produzido (até o tamanho dado, de 1 a 128 bytes; abaixo de 50 só o zero do começo)
    let n = ((tamanho as u16 as i16).max(1) as usize).min(128);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, n)], || o(nacao, texto, tamanho, genero),
        || adjetivo_da_nacao(nacao, texto, tamanho, genero));
    PLACAR.registrar(igual && r == x, || format!("adjetivo_da_nacao({:x}, tamanho {}, genero {}) original={} rust={} {:x?} x {:x?}",
        nacao, tamanho as u16 as i16, genero as u8, r, x, mo, mr));
    r
}

// ---------- 238a leva: as chaves fixas do jogo: os ids pela base, as cores e os continentes, e o relato das que faltam (index.cpp) ----------

// As listas da set_key_ids, geradas do exe por `python engenharia-reversa/chaves_fixas.py` (rust/src/chaves_fixas.rs).
include!("../chaves_fixas.rs");

/// Uma cadeia de _stricmp (0x951c50) em linha (cores, continentes): o primeiro texto igual ao `nome` grava `linha` na
/// global dele e a cadeia acaba.
pub(crate) unsafe fn cadeia_das_chaves_dbw(nome: usize, pares: &[(u32, u32)], linha: u32) {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_STRICMP);
    for &(texto, global) in pares {
        if stricmp(nome, texto as usize) == 0 {
            *(global as *mut u32) = linha;
            return;
        }
    }
}

/// Um laço da set_key_ids: `i` de 0 enquanto `i < limite()` (relido a cada volta, com sinal). Devolve se rodou (a
/// original só põe o -1 de volta no edi depois de um laço que rodou).
pub(crate) unsafe fn laco_das_chaves_dbw(limite: impl Fn() -> i32, mut f: impl FnMut(i32)) -> bool {
    let mut i = 0i32;
    while i < limite() {
        f(i);
        i += 1;
    }
    i > 0
}

/// A caixa fatal do relato (index.cpp, `linha`): "v%s %s.%s %d" em 0xae24d0 (0x90d130 com o nome e a extensão do
/// index.cpp), a caixa "Error" (0x5e8290), [0xb67a34] = 0, 0x5cc7d0, 0x5a8f60 e exit(marca). Não volta.
pub(crate) unsafe fn caixa_fatal_das_chaves_dbw(linha: u32, marca: u32) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let antes_de_sair1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let antes_de_sair2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(u32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9d93cc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    // a original não desempilha os argumentos da 0x90d130: são o resto do "v%s %s.%s %d"
    formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    antes_de_sair1();
    antes_de_sair2();
    sair(marca)
}

/// O miolo da set_key_ids depois do -1 (o -1 é de cada entrada). `marca` = o edi na entrada (-1 na original;
/// MARCA_DAS_CHAVES_1080 no 1080, que só vale se nenhum laço rodar); `relatar` = o relato das chaves sem valor (no
/// modo comparando a cópia roda sem ele, para a caixa não aparecer duas vezes). Devolve 1.
/// 1) As cores ([0xae2394] de [0xae23ec] + 4, passo 0x3a): CORES_DAS_CHAVES. 2) Com [0x9b5984] ≠ 0 e
/// CM3_TEMP\eng.lng aberto: 8 contagens (bytes invertidos se 0x90c140(f) = 0xf1fffff1) e as tabelas na ordem do
/// arquivo: clubes c0 × 0x4f (0x6146b0; pela lista [0xae23fc] até [0xae235c], se houver), continentes c1 × 0xba,
/// competições c2 × 0x53 (0x60efd0 com a linha c5 + i), nações c3 × 0x6d (0x60d810), pula c6 × 0x34 e c4 × 0x34,
/// prêmios c7 × 0x4f (0x6115e0), competições c5 × 0x53 (0x60efd0); tabela sem memória: fseek por cima. 3) Senão, pela
/// base: nações (0x60d810), competições de clube até [0xae2378] − 0x7f (0x60efd0), continentes, prêmios (0x6115e0) e
/// clubes (0x6146b0). 4) O relato (RELATO_DAS_CHAVES): a 0x60d780 (a caixa "Unable to find the %s index"); as 7
/// fatais saem do jogo se a nação da chave tiver o byte +0x11c.
pub(crate) unsafe fn chaves_fixas(marca_entrada: u32, relatar: bool) -> u32 {
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const u32);
    let em = |base: u32, i: i32, passo: u32| base.wrapping_add((i as u32).wrapping_mul(passo)) as usize;
    let clube: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_CLUB_KEY_ID);
    let nacao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_NATION_KEY_ID);
    let competicao: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_CLUB_COMP_KEY_ID);
    let premio: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_MATCH_STAFF_COMP_KEY_ID);
    let mut marca = marca_entrada;
    if laco_das_chaves_dbw(|| gi(0xae2394), |i| cadeia_das_chaves_dbw(em(rd(0xae23ec), i, 0x3a) + 4, &CORES_DAS_CHAVES, i as u32)) {
        marca = u32::MAX;
    }
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let f = if gi(0x9b5984) != 0 { abrir(caminho(0x987da0, 0x9b632c), 0x98f928) } else { 0 }; // CM3_TEMP\eng.lng, "rb"
    if f != 0 {
        let versao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_UTILS_READ_FILE_FORMAT_MARKER);
        let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        let pular: unsafe extern "C" fn(usize, u32, i32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
        let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
        let inverter = versao(f) == 0xf1fffff1;
        let mut c = [0i32; 8];
        ler(c.as_mut_ptr() as usize, 0x20, 1, f);
        if inverter {
            for x in c.iter_mut() {
                *x = x.swap_bytes();
            }
        }
        // a tabela de `n` linhas de `passo` bytes: lida e entregue a `f` (o fseek por cima se a memória faltar)
        let tabela = |n: i32, passo: u32, f_: &mut dyn FnMut(u32)| {
            let b = alocar((n as u32).wrapping_mul(passo));
            if b != 0 {
                ler(b, passo, n as u32, f);
                f_(b as u32);
                liberar(b);
            } else {
                pular(f, (n as u32).wrapping_mul(passo), 1);
            }
        };
        tabela(c[0], 0x4f, &mut |b| {
            let mut i = 0i32;
            loop {
                let lista = rd(0xae23fc); // a lista de clubes da base
                if i >= if lista != 0 { gi(0xae235c) } else { c[0] } {
                    break;
                }
                let linha = if lista != 0 { em(b, gi(em(lista, i, 4)), 0x4f) } else { em(b, i, 0x4f) };
                clube(linha, i as u32);
                i += 1;
            }
        });
        tabela(c[1], 0xba, &mut |b| {
            laco_das_chaves_dbw(|| c[1], |i| cadeia_das_chaves_dbw(em(b, i, 0xba), &CONTINENTES_DAS_CHAVES, i as u32)); // continentes do eng.lng
        });
        tabela(c[2], 0x53, &mut |b| {
            laco_das_chaves_dbw(|| c[2], |i| { competicao(em(b, i, 0x53), c[5].wrapping_add(i) as u32); }); // competições do eng.lng depois das c5
        });
        tabela(c[3], 0x6d, &mut |b| {
            laco_das_chaves_dbw(|| c[3], |i| { nacao(em(b, i, 0x6d), i as u32); }); // nações do eng.lng
        });
        // os prêmios: a memória antes de pular as duas tabelas de 0x34
        let b = alocar((c[7] as u32).wrapping_mul(0x4f));
        pular(f, (c[6] as u32).wrapping_mul(0x34), 1);
        pular(f, (c[4] as u32).wrapping_mul(0x34), 1);
        if b != 0 {
            ler(b, 0x4f, c[7] as u32, f);
            laco_das_chaves_dbw(|| c[7], |i| { premio(em(b as u32, i, 0x4f), i as u32); }); // prêmios do eng.lng
            liberar(b);
        } else {
            pular(f, (c[7] as u32).wrapping_mul(0x4f), 1);
        }
        tabela(c[5], 0x53, &mut |b| {
            laco_das_chaves_dbw(|| c[5], |i| { competicao(em(b, i, 0x53), i as u32); }); // competições do eng.lng
        });
        fechar(f);
        marca = u32::MAX;
    } else {
        let mut rodou = laco_das_chaves_dbw(|| gi(0xae2350), |i| { nacao(em(rd(0xae23a8), i, 0x122) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae2378).wrapping_sub(0x7f), |i| { competicao(em(rd(0xae23d0), i, 0x6b) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae234c), |i| {
            let nome = em(rd(0xae23a4), i, 0xc6) + 4;
            cadeia_das_chaves_dbw(nome, &CONTINENTES_DAS_CHAVES, i as u32); // continentes da base
        });
        rodou |= laco_das_chaves_dbw(|| gi(0xae2388), |i| { premio(em(rd(0xae23e0), i, 0x65) + 4, i as u32); });
        rodou |= laco_das_chaves_dbw(|| gi(0xae235c), |i| { clube(em(rd(0xae23b4), i, 0x245) + 4, i as u32); }); // clubes da base
        if rodou {
            marca = u32::MAX;
        }
    }
    if relatar {
        let relato: unsafe extern "C" fn(usize) = core::mem::transmute(FN_REPORT_MISSING_KEY_ID);
        for &(chave, nome, tipo, nacao_da_chave, linha) in RELATO_DAS_CHAVES.iter() {
            if rd(chave as usize) != marca {
                continue;
            }
            if tipo == 2 {
                let n = em(rd(0xae23a8), gi(nacao_da_chave as usize), 0x122);
                if *((n as u32).wrapping_add(0x11c) as *const u8) != 0 {
                    caixa_fatal_das_chaves_dbw(linha as u32, marca);
                }
            }
            relato(nome as usize); // tipos 0 e 1 (o 1 é o corpo da 0x60d780 em linha)
        }
    }
    1
}

/// As globais em -1 na original (MENOS_UM_DAS_CHAVES).
pub(crate) unsafe fn menos_um_das_chaves_dbw() {
    for &g in MENOS_UM_DAS_CHAVES.iter() {
        *(g as *mut i32) = -1;
    }
}

/// As faixas inteiras em -1 no 1080 (MENOS_UM_DAS_CHAVES_1080, o rep stosd).
pub(crate) unsafe fn menos_um_das_chaves_1080_dbw() {
    for &(a, n) in MENOS_UM_DAS_CHAVES_1080.iter() {
        core::ptr::write_bytes(a as *mut u8, 0xff, n as usize * 4);
    }
}

/// the fixed keys (colours, continents, nations, competitions, awards, clubs) set to -1 and then to the database row by
/// name (from eng.lng or from the database); the ones left without a value go to the "Unable to find the %s index" box
/// (7 fatal by nation); in cm0102-1080.exe through the 0x603834 entry, with -1 over the whole ranges
/// Original: `set_key_ids` (0x601a40, index.cpp, cdecl).
///
/// [pt] { 0x601a40 } (cdecl, index.cpp, set_key_ids): na carga da base (0x528540), as chaves fixas do jogo (cores,
/// continentes, nações, competições, prêmios, clubes) recebem a linha da base pelo nome; as que ficam sem valor vão ao
/// relato. A entrada do cm0102.exe: o -1 nas 1.282 globais de MENOS_UM_DAS_CHAVES e o miolo `chaves_fixas`. Devolve 1.
pub(crate) unsafe extern "C" fn definir_chaves_fixas() -> u32 {
    menos_um_das_chaves_dbw();
    chaves_fixas(u32::MAX, true)
}

/// { 0x603834 } (cdecl, só no cm0102-1080.exe): a mesma rotina pela entrada do 1080 (a chamada de 0x52d8fa vai para
/// ela): o -1 nas faixas inteiras 0x9cf178..0x9d0527 e 0xb63c9c..0xb63d03 (também as 4 globais que a original não toca)
/// e o mesmo miolo. Devolve 1.
pub(crate) unsafe extern "C" fn definir_chaves_fixas_1080() -> u32 {
    menos_um_das_chaves_1080_dbw();
    chaves_fixas(MARCA_DAS_CHAVES_1080, true)
}

/// As regiões das chaves fixas: as globais das chaves, as chaves de clube altas e 0xde279c.
pub(crate) const REGIOES_DAS_CHAVES_DBW: [(usize, usize); 3] = [(KEY_COLOUR_BLACK, 0x13b0), (0xb63c9c, 0x68), (0xde279c, 0x104)];

pub(crate) unsafe extern "C" fn comparar_definir_chaves_fixas() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(IDX_DEFINIR_CHAVES_FIXAS);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&REGIOES_DAS_CHAVES_DBW, || o(), || {
        menos_um_das_chaves_dbw();
        chaves_fixas(u32::MAX, false)
    });
    PLACAR.registrar(igual && r == n, || format!("definir_chaves_fixas() original={} rust={} {}", r, n, chaves_diferentes(&mo, &mr)));
    r
}

pub(crate) unsafe extern "C" fn comparar_definir_chaves_fixas_1080() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(IDX_DEFINIR_CHAVES_FIXAS_1080);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&REGIOES_DAS_CHAVES_DBW, || o(), || {
        menos_um_das_chaves_1080_dbw();
        chaves_fixas(MARCA_DAS_CHAVES_1080, false)
    });
    PLACAR.registrar(igual && r == n, || format!("definir_chaves_fixas_1080() original={} rust={} {}", r, n, chaves_diferentes(&mo, &mr)));
    r
}

// ---------- 237a leva: o nome da região e o adjetivo do continente (index.cpp) ----------



/// Os nomes das regiões 1..14 da 0x616f20 (imediatos do código, pelos endereços no exe): (adjetivo, substantivo). A
/// 3 (Caribe) tem textos próprios.
pub(crate) const DBU3_REGIOES: [(usize, usize); 14] = [
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

/// the region name (1..14): from the language 4, 5 and 9 tables with the adjective form, else the translated adjective
/// or noun by form
/// Original: `get_region_name` (0x616f20, index.cpp, cdecl).
///
/// [pt] { 0x616f20 } (cdecl, index.cpp): `get_region_name(regiao, texto, tamanho, forma)`: zera o texto e escreve o nome da
/// região (1..14, byte com sinal). Forma (byte) diferente de 0xff com o idioma (0x66f4d0) 4, 5 ou 9: o texto da
/// tabela do idioma, lida pelo endereço ([0x9d0518 + 4 × (4 × região + f)], [0x9d06d4 + 4 × (5 × região + f)],
/// [0x9d05f8 + 4 × (4 × região + f)], com f a forma do adjetivo 0x545a60, já em Rust, chamada pelo endereço do
/// jogo), copiado. Senão, pelo 0x66f4e0: nos idiomas 2, 4, 5 e 9 o adjetivo com a forma 0xff e o substantivo com as
/// outras; nos demais o contrário; o Caribe: "Caribbean" nos idiomas 2, 4, 5 e 9, senão os textos de região (forma
/// 0xff) ou de regionalidade, com o vazio 0xad9c64. Devolve 1. Texto nulo, região fora da faixa ou tamanho (curto
/// com sinal) < 100: a original (caixa). Na ordem das regiões (0x545dc0, em Rust), na busca de jogadores
/// (player_search, search_screens) e em telas sem módulo (0x7db360, 0x7e13d0, 0x7e29e0).
pub(crate) unsafe extern "C" fn nome_da_regiao(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let r = regiao as u8 as i8;
    if texto == 0 || !(1..=14).contains(&r) || (tamanho as u16 as i16) < 100 {
        return orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_NOME_DA_REGIAO)(regiao, texto, tamanho, forma);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let adjetivo: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_GET_ADJECTIVE_FORM_INDEX);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
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
            dbu1_copiar(texto, read_usize(base.wrapping_add((k as usize).wrapping_mul(4))));
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

pub(crate) unsafe extern "C" fn comparar_nome_da_regiao(regiao: u32, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(IDX_NOME_DA_REGIAO);
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
pub(crate) const DBU3_CONTINENTES_9: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38, 0x9d0d38],
    [0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08, 0x9d0d08],
    [0x9e2120, 0x9e2114, 0x9e2108, 0x9e20fc, 0x9e20fc],
    [0x9d0b94, 0x9d0b84, 0x9d0b70, 0x9d0b5c, 0x9d0b5c],
    [0x9d0b50, 0x9d0b44, 0x9d0b38, 0x9d0b2c, 0x9d0b2c],
    [0x9d0ae4, 0x9d0ad4, 0x9d0ac4, 0x9d0ab4, 0x9d0ab4],
];
pub(crate) const DBU3_CONTINENTES_5: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d1064, 0x9d104c, 0x9d0d44, 0x9d0d38],
    [0x9d0d2c, 0x9d0d2c, 0x9d0d20, 0x9d0d14, 0x9d0d08],
    [0x9e20f4, 0x9e20f4, 0x9e20ec, 0x9e20e0, 0x9e20d4],
    [0x9d08cc, 0x9d08cc, 0x9d08bc, 0x9d08ac, 0x9d089c],
    [0x9d0890, 0x9d0890, 0x9d0884, 0x9d0878, 0x9d086c],
    [0x9d085c, 0x9d085c, 0x9d084c, 0x9d083c, 0x9d082c],
];
pub(crate) const DBU3_CONTINENTES_4: [[usize; 5]; 6] = [
    [0x9d1064, 0x9d1058, 0x9d104c, 0x9d1040, 0x9d1040],
    [0x9d1034, 0x9d1028, 0x9d101c, 0x9d1010, 0x9d1010],
    [0x9e20f4, 0x9e20cc, 0x9e20ec, 0x9e20c4, 0x9e20c4],
    [0x9d0eb0, 0x9d0ea0, 0x9d0e90, 0x9d0e80, 0x9d0e80],
    [0x9d0e74, 0x9d0e68, 0x9d0e5c, 0x9d0e50, 0x9d0e50],
    [0x9d0e10, 0x9d0e00, 0x9d0df0, 0x9d0de0, 0x9d0de0],
];

/// the continent adjective: the record one (+0x23) or, in languages 9, 5 and 4, the text by continent and adjective
/// form
/// Original: `get_continent_adjective` (0x6177f0, index.cpp, cdecl).
///
/// [pt] { 0x6177f0 } (cdecl, index.cpp; o mapa dá 629 bytes, a rotina vai até 0x6181c5: 2518): `get_continent_adjective(
/// continente, texto, tamanho, forma)`: zera o texto e copia o adjetivo do continente. Forma (byte) 0 ou idioma
/// (0x66f4d0) fora de 9, 5 e 4: o adjetivo do próprio registro (+0x23). Senão o texto do idioma pelo continente (o id
/// +0 comparado às chaves [0x9cfa08..0x9cfa18], nessa ordem) e pela forma do adjetivo (0x545a60, já em Rust, pelo
/// endereço do jogo). Devolve 1. Continente ou texto nulos ou tamanho (curto com sinal) < 50: a original (0). Na
/// busca (find_screens), no Menubar, nos jogos do dia (0x717cc0, em Rust) e em 0x7dd400.
pub(crate) unsafe extern "C" fn adjetivo_do_continente(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_ADJETIVO_DO_CONTINENTE)(continente, texto, tamanho, forma);
    }
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let adjetivo: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_GET_ADJECTIVE_FORM_INDEX);
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
            let id = read_usize(continente);
            let linha = [KEY_CONTINENT_AFRICA, KEY_CONTINENT_ASIA, KEY_CONTINENT_EUROPE, KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_OCEANIA].iter().position(|&g| read_usize(g) == id).unwrap_or(5);
            t[linha][if (0..=3).contains(&f) { f as usize } else { 4 }]
        }
    };
    dbu1_copiar(texto, s);
    1
}

pub(crate) unsafe extern "C" fn comparar_adjetivo_do_continente(continente: usize, texto: usize, tamanho: u32, forma: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, u32) -> u32>(IDX_ADJETIVO_DO_CONTINENTE);
    if continente == 0 || texto == 0 || (tamanho as u16 as i16) < 0x32 {
        return o(continente, texto, tamanho, forma);
    }
    // os 50 primeiros bytes (o tamanho mínimo aceito)
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(texto, 50)], || o(continente, texto, tamanho, forma),
        || adjetivo_do_continente(continente, texto, tamanho, forma));
    PLACAR.registrar(igual && r == n, || format!("adjetivo_do_continente({:x}, forma {}) original={} rust={} {:x?} x {:x?}", continente, forma as u8, r, n, mo, mr));
    r
}

// ---------- 227a leva: as chaves fixas de clube e de nação pelo nome e a troca da chave de clube depois do corte da base (index.cpp) ----------

/// As chaves de clube do index.cpp (0x6146b0), na ordem do código: (texto do nome, global que recebe a linha), em 4 cadeias. Vários nomes podem dar na mesma global (grafias).
pub(crate) static CHAVES_DE_CLUBE: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9e1f08, 0x9d00a4), // MLS All Stars East
        (0x9e1ef4, 0x9d00a8), // MLS All Stars West
        (0x9e1ee8, 0x9d00ac), // River Plate
        (0x9e1ed8, 0x9d00b0), // Vélez Sarsfield
        (0x9e1ec8, 0x9d00b4), // Independiente
        (0x9e1ea4, 0x9d00b8), // Independiente Rivadavia de Mendoza
        (0x9e1e90, 0x9d00bc), // Racing de Córdoba
        (0x9e1e74, 0x9d00c0), // Villa Mitre de Bahía Blanca
        (0x9e1e5c, 0x9d00c4), // Argentino de Rosario
        (0x9e1e50, 0x9d00c8), // Temperley
        (0x9e1e48, 0x9d00cc), // Lanús
        (0x9e1e38, 0x9d00d0), // Boca Juniors
        (0x9e1e18, 0x9d00d4), // Gimnasia y Esgrima de La Plata
        (0x9e1e00, 0x9d00d8), // San Lorenzo de Almagro
        (0x9e1dec, 0x9d00dc), // Argentinos Juniors
        (0x9e1ddc, 0x9d00e0), // Rosario Central
        (0x9e1dc8, 0x9d00e4), // Newell's Old Boys
        (0x9e1dbc, 0x9d00e8), // Racing Club
        (0x9e1da8, 0x9d00ec), // Colón de Santa Fé
        (0x9e1d9c, 0x9d00f0), // Platense
        (0x9e1d88, 0x9d00f4), // Ferro Carril Oeste
        (0x9e1d70, 0x9d00f8), // Estudiantes de La Plata
        (0x9e1d54, 0x9d00fc), // Gimnasia y Esgrima de Jujuy
        (0x9e1d40, 0x9d0100), // Unión de Santa Fé
        (0x9e1d38, 0x9d0104), // Huracán
        (0x9e1d24, 0x9d0108), // Talleres de Córdoba
        (0x9e1d10, 0x9d010c), // Belgrano de Córdoba
        (0x9e1d08, 0x9d0110), // Arsenal
        (0x9e1cf8, 0x9d0114), // Crystal Palace
        (0x9e1ce8, 0x9d0118), // Glasgow Celtic
        (0x9e1cd8, 0x9d011c), // Glasgow Rangers
        (0x98e568, 0x9d0120), // Athletic Club de Bilbao
        (0x98e58c, 0x9d0124), // C.D. Alavés
        (0x98e3c8, 0x9d0128), // Real Sociedad C.F.
        (0x98e2c4, 0x9d012c), // Atlético de Madrid
        (0x9e1cc0, 0x9d0130), // Atlético de Madrid B
        (0x9e1ca4, 0x9d0134), // Athletic Club de Bilbao B
        (0x98e540, 0x9d0138), // F.C. Barcelona
        (0x9e1c90, 0x9d013c), // F.C. Barcelona B
        (0x98e34c, 0x9d0140), // Real Valladolid
        (0x9e1c7c, 0x9d0144), // Real Valladolid B
        (0x98e47c, 0x9d0148), // Málaga C.F.
        (0x9e1c6c, 0x9d014c), // Málaga C.F. B
        (0x98e520, 0x9d0150), // Real Betis Balompié
        (0x9e1c54, 0x9d0154), // Real Betis Balompié B
        (0x98e4fc, 0x9d0158), // Real Club Celta de Vigo
        (0x9e1c38, 0x9d015c), // Real Club Celta de Vigo B
        (0x98e4d8, 0x9d0160), // Deportivo de La Coruña
        (0x9e1c1c, 0x9d0164), // Deportivo de La Coruña B
        (0x98e4b8, 0x9d0168), // R.C.D. Espanyol
        (0x9e1c08, 0x9d016c), // R.C.D. Espanyol B
        (0x9e1bf8, 0x9d0170), // C.D. Logroñés
        (0x9e1be8, 0x9d0174), // C.D. Logroñés B
        (0x98e460, 0x9d0178), // R.C.D. Mallorca
        (0x9e1bd4, 0x9d017c), // R.C.D. Mallorca B
        (0x98e438, 0x9d0180), // Club Atlético Osasuna
        (0x9e1bbc, 0x9d0184), // Club Atlético Osasuna B
        (0x98e0fc, 0x9d0188), // Real Oviedo C.F.
        (0x9e1ba8, 0x9d018c), // Real Oviedo C.F. B
        (0x98e3ec, 0x9d0190), // Real Madrid C.F.
        (0x9e1b94, 0x9d0194), // Real Madrid C.F. B
        (0x98e3a8, 0x9d0198), // Sevilla C.F.
        (0x9e1b84, 0x9d019c), // Sevilla C.F. B
        (0x9e1b6c, 0x9d01a0), // Real Sociedad C.F. B
        (0x9e1b54, 0x9d01a4), // Instituto de Córdoba
        (0x9e1b3c, 0x9d01a8), // San Martín de San Juan
        (0x9e1b20, 0x9d01ac), // Gimnasia y Tiro de Salta
        (0x9e1b0c, 0x9d01b0), // Atlético Rafaela
        (0x9e1af4, 0x9d01b4), // San Martín de Tucumán
        (0x9e1ad4, 0x9d01b8), // Almirante Brown de Arrecifes
        (0x9e1ac0, 0x9d01bc), // Atlético Tucumán
        (0x9e1aa8, 0x9d01c0), // Olimpo de Bahía Blanca
        (0x9e1a90, 0x9d01c4), // Huracán de Corrientes
        (0x9e1a84, 0x9d01c8), // Aldosivi
        (0x9e1a6c, 0x9d01cc), // Cipolletti de Río Negro
        (0x9e1a54, 0x9d01d0), // San Martín de Mendoza
        (0x9e1a44, 0x9d01d4), // Douglas Haig
        (0x9e1a14, 0x9d01d8), // Gimnasia y Esgrima de Concepción del Uruguay
        (0x9e19f8, 0x9d01dc), // Juventud Antoniana de Salta
        (0x9e19e0, 0x9d01e0), // Godoy Cruz de Mendoza
        (0x9e19c4, 0x9d01e0), // Godoy Cruz Antonio Tomba
        (0x9e19b8, 0x9d01e4), // Banfield
        (0x9e19ac, 0x9d01e8), // Los Andes
        (0x9e19a4, 0x9d01ec), // Quilmes
        (0x9e1988, 0x9d01f0), // Central Córdoba de Rosario
        (0x9e1974, 0x9d01f4), // Chacarita Juniors
        (0x9e1968, 0x9d01f8), // All Boys
        (0x9e195c, 0x9d01fc), // San Miguel
        (0x9e194c, 0x9d0200), // Nueva Chicago
        (0x9e1938, 0x9d0204), // Defensa y Justicia
        (0x9e1924, 0x9d0208), // Arsenal de Sarandí
        (0x9e190c, 0x9d020c), // Huracán de Tres Arroyos
        (0x9e18f4, 0x9d0210), // Defensores de Belgrano
        (0x9e18e4, 0x9d0214), // Deportivo Morón
        (0x9e18c8, 0x9d0218), // Estudiantes de Buenos Aires
        (0x9e18c0, 0x9d021c), // Almagro
        (0x9e18b8, 0x9d0220), // Atlanta
        (0x9e18a4, 0x9d0224), // Deportivo Español
        (0x9e1898, 0x9d0228), // El Porvenir
        (0x9e1890, 0x9d022c), // Tigre
        (0x9e1880, 0x9d0230), // Middlesbrough
        (0x9e186c, 0x9d0234), // Newcastle United
        (0x9e1860, 0x9d0234), // Newcastle
        (0x9e1854, 0x9d0238), // Sunderland
        (0x9e1848, 0x9d023c), // Aston Villa
        (0x9e1834, 0x9d0240), // Manchester United
        (0x9e1828, 0x9d0244), // Liverpool
    ],
    // cadeia 2
    &[
        (0x9e1820, 0x9d0248), // Everton
        (0x9e180c, 0x9d024c), // Tottenham Hotspur
        (0x9e1804, 0x9d0250), // Chelsea
        (0x98e36c, 0x9d0254), // Valencia C.F.
        (0x9e17f4, 0x9d0258), // Valencia C.F. B
        (0x9e17e8, 0x9d0260), // Southampton
        (0x9e17cc, 0x9d025c), // Brighton and Hove Albion
        (0x9e17c0, 0x9d0264), // AS Monaco
        (0x9e17b8, 0x9d0268), // FC Metz
        (0x9e17a0, 0x9d026c), // Cruzeiro Esporte Clube
        (0x98a46c, 0x9d0270), // São Paulo Futebol Clube
        (0x98a4f0, 0x9d0274), // Sociedade Esportiva Palmeiras
        (0x98a2a8, 0x9d0278), // Sport Club Corinthians Paulista
        (0x9e1780, 0x9d027c), // Gremio Foot Ball Porto Alegre
        (0x9e175c, 0x9d027c), // Grêmio Foot Ball Porto-Alegrense
        (0x98a304, 0x9d027c), // Grêmio Foot-Ball Porto-Alegrense
        (0x9e173c, 0x9d0354), // Clube de Regatas Vasco da Gama
        (0x9e1718, 0x9d0280), // Club Social y Deportivo Colo Colo
        (0x9e1708, 0x9d0284), // Univ.de Chile
        (0x9e16f8, 0x9d0284), // Univ. de Chile
        (0x9e16c4, 0x9d0284), // Corporación de Fútbol de la Universidad de Chile
        (0x9e1698, 0x9d0284), // Club de Fútbol de la Universidad de Chile
        (0x9e1688, 0x9d0288), // Univ.Catolica
        (0x9e1670, 0x9d0288), // Universidad Catolica
        (0x9e164c, 0x9d0288), // Club Deportivo Universidad Católica
        (0x9e1634, 0x9d028c), // Club Nacional de Fútbol
        (0x9e1620, 0x9d028c), // Nacional Montevideo
        (0x9e1618, 0x9d0290), // Olimpia
        (0x9e1608, 0x9d0290), // Club Olimpia
        (0x9e15f8, 0x9d0294), // Cerro Porteño
        (0x9e15e4, 0x9d0294), // Club Cerro Porteño
        (0x9e15c8, 0x9d0358), // Clube de Regatas Flamengo
        (0x98a2e4, 0x9d0358), // Clube de Regatas do Flamengo
        (0x9e15b0, 0x9d0298), // Club Atlético Peñarol
        (0x9e15a8, 0x9d0298), // Penarol
        (0x9e159c, 0x9d029c), // Elgin City
        (0x9e1590, 0x9d02a0), // Peterhead
        (0x9e157c, 0xb63cdc), // 1.FC Kaiserslautern
        (0x9e155c, 0xb63c9c), // 1.FC Kaiserslautern Amateure
        (0x9e1544, 0xb63c9c), // 1.FC Kaiserslautern (A)
        (0x9e1530, 0xb63cbc), // Bayer 04 Leverkusen
        (0x9e1514, 0xb63ca0), // Bayer Leverkusen Amateure
        (0x9e14fc, 0xb63ca0), // Bayer Leverkusen (A)
        (0x9e14e4, 0xb63ca0), // Bayer 04 Leverkusen U21
        (0x9e14d0, 0xb63ce8), // Borussia Dortmund
        (0x9e14b8, 0xb63ce8), // BV Borussia Dortmund
        (0x9e149c, 0xb63cac), // Borussia Dortmund Amateure
        (0x9e1484, 0xb63cac), // Borussia Dortmund (A)
        (0x9e1470, 0xb63cd8), // FC Bayern München
    ],
    // cadeia 3
    &[
        (0x9e1458, 0xb63cd0), // Bayern München Amateure
        (0x9e143c, 0xb63cd0), // FC Bayern München Amateure
        (0x9e142c, 0xb63cb4), // FC St. Pauli
        (0x9e1414, 0xb63cb0), // FC St. Pauli Amateure
        (0x9e1400, 0xb63cb0), // FC St. Pauli (A)
        (0x9e13f0, 0xb63cd4), // Hamburger SV
        (0x9e13d8, 0xb63ca8), // Hamburger SV Amateure
        (0x9e13c4, 0xb63ca8), // Hamburger SV (A)
        (0x9e13b0, 0xb63ce4), // Hertha BSC Berlin
        (0x9e139c, 0xb63cec), // Hertha BSC Amateure
        (0x9e138c, 0xb63cec), // Hertha BSC (A)
        (0x9e137c, 0xb63cf0), // Karlsruher SC
        (0x9e1364, 0xb63cf8), // Karlsruher SC Amateure
        (0x9e1350, 0xb63cf8), // Karlsruher SC (A)
        (0x9e133c, 0xb63cf8), // Karlsruher SC II
        (0x9e1328, 0xb63cc0), // SV Werder Bremen
        (0x9e130c, 0xb63d00), // SV Werder Bremen Amateure
        (0x9e12f4, 0xb63d00), // SV Werder Bremen (A)
        (0x9e12e0, 0xb63cc8), // TSV München 1860
        (0x9e12cc, 0xb63cc8), // TSV 1860 München
        (0x9e12b0, 0xb63ccc), // TSV München 1860 Amateure
        (0x9e1294, 0xb63ccc), // TSV 1860 München Amateure
        (0x9e127c, 0xb63ce0), // Tennis Borussia Berlin
        (0x9e1260, 0xb63cb8), // Tennis Borussia Amateure
        (0x9e124c, 0xb63cb8), // Tennis Borussia (A)
        (0x9e1230, 0xb63cb8), // Tennis Borussia Berlin II
        (0x9e1220, 0xb63cc4), // VfB Stuttgart
        (0x9e1208, 0xb63cf4), // VfB Stuttgart Amateure
        (0x9e11f4, 0xb63cf4), // VfB Stuttgart (A)
        (0x9e11e8, 0xb63cfc), // VfL Bochum
        (0x9e11d4, 0xb63ca4), // VfL Bochum Amateure
        (0x9e11c4, 0xb63ca4), // VfL Bochum (A)
        (0x9e11b0, 0x9d02a4), // Eintracht Frankfurt
        (0x9e1190, 0x9d02a8), // Eintracht Frankfurt Amateure
        (0x9e1178, 0x9d02a8), // Eintracht Frankfurt (A)
        (0x9e1164, 0x9d02ac), // FC Hansa Rostock
        (0x9e1148, 0x9d02b0), // FC Hansa Rostock Amateure
        (0x9e1130, 0x9d02b0), // FC Hansa Rostock (A)
        (0x9e1120, 0x9d02b4), // FC Schalke 04
        (0x9e1108, 0x9d02b8), // FC Schalke 04 Amateure
        (0x9e10f4, 0x9d02b8), // FC Schalke 04 (A)
        (0x9e10e0, 0x9d02bc), // SG Wattenscheid 09
        (0x9e10c4, 0x9d02c0), // SG Wattenscheid 09 Amateure
        (0x9e10ac, 0x9d02c0), // SG Wattenscheid 09 (A)
        (0x9e1094, 0x9d02c0), // SG Wattenscheid 09 II
        (0x9e107c, 0x9d02c4), // Sport Lisboa e Benfica
        (0x9e1060, 0x9d02c8), // Sport Lisboa e Benfica B
        (0x9e1048, 0x9d02cc), // Sporting Clube de Braga
        (0x9e102c, 0x9d02d0), // Sporting Clube de Braga B
        (0x9e1014, 0x9d02d4), // Clube Sport Marítimo
        (0x9e0ffc, 0x9d02d8), // Clube Sport Marítimo B
        (0x9e0fe4, 0x9d02dc), // Futebol Clube do Porto
        (0x9e0fc8, 0x9d02e0), // Futebol Clube do Porto B
        (0x98e250, 0x9d02e4), // S.D. Éibar
        (0x9e0fb8, 0x9d02e8), // Amurrio C.F.
        (0x9e0fa0, 0x9d02ec), // C.D. Aurrerá Vitoria
        (0x9e0f90, 0x9d02f0), // Barakaldo C.F.
        (0x9e0f80, 0x9d02f4), // S.D. Beasaín
        (0x9e0f74, 0x9d02f8), // Bermeo Club
        (0x9e0f64, 0x9d02fc), // Gernika Club
        (0x9e0f58, 0x9d0304), // S.D. Lemona
        (0x9e0f48, 0x9d0308), // Elgoibar C.D.
        (0x9e0f38, 0x9d030c), // Hernani C.D.
        (0x9e0f28, 0x9d030c), // C.D. Hernani
        (0x9e0f1c, 0x9d0310), // Zalla U.C.
        (0x9e0f08, 0x9d0314), // Cultural Durango
        (0x9e0ef8, 0x9d0318), // S.D. Amorebieta
        (0x9e0ee4, 0x9d031c), // Arenas Getxo Bilbao
        (0x9e0ecc, 0x9d0320), // Aurrerá Ondarroa C.D.
        (0x9e0ebc, 0x9d0324), // Baskonia C.D.
        (0x9e0eac, 0x9d0328), // S.D. Éibar B
        (0x9e0e9c, 0x9d032c), // U.D. San Pedro
        (0x9e0e8c, 0x9d032c), // San Pedro U.D.
        (0x9e0e7c, 0x9d0330), // Santurtzi C.D.
        (0x9e0e68, 0x9d0334), // Sestao River Club
        (0x9e0e5c, 0x9d0338), // Tolosa C.F.
        (0x98a664, 0x9d033c), // Coritiba Football Club
        (0x98a4bc, 0x9d0340), // Santos Futebol Clube
        (0x98a4a4, 0x9d0344), // Sport Club do Recife
        (0x9e0e38, 0x9d0348), // Associação Portugesa de Desportos
        (0x9e0e14, 0x9d0348), // Associação Portuguesa de Desportos
        (0x98a70c, 0x9d0350), // Clube Atlético Mineiro
        (0x9e173c, 0x9d0354), // Clube de Regatas Vasco da Gama
        (0x9e15c8, 0x9d0358), // Clube de Regatas Flamengo
        (0x98a7c0, 0x9d035c), // Sport Club Internacional
        (0x98a800, 0x9d0360), // Esporte Clube Vitória
        (0x9e0dfc, 0x9d0364), // Botafogo Futebol Clube
        (0x98a58c, 0x9d0364), // Botafogo de Futebol e Regatas
        (0x98a2c8, 0x9d0368), // Clube Atlético Paranaense
        (0x98a484, 0x9d036c), // Associação Atlética Ponte Preta
        (0x98a7a8, 0x9d0370), // Esporte Clube Juventude
        (0x9e0de4, 0x9d0374), // Guarani Futebol Clube
        (0x98a6a4, 0x9d0378), // Paraná Clube
        (0x9e0dd4, 0x9d037c), // Chicago Fire
        (0x9e0dc4, 0x9d0380), // Miami Fusion FC
        (0x9e0dac, 0x9d0384), // Washington DC United
        (0x9e0d98, 0x9d0388), // Tampa Bay Mutiny
        (0x9e0d80, 0x9d038c), // San Jose Earthquakes
        (0x9e0d70, 0x9d038c), // San Jose Clash
        (0x9e0d58, 0x9d0390), // New England Revolution
        (0x9e0d44, 0x9d0394), // NY/NJ Metrostars
        (0x9e0d30, 0x9d0394), // NY-NJ Metrostars
        (0x9e0d20, 0x9d0394), // NYNJ Metrostars
        (0x9e0d0c, 0x9d0398), // Los Angeles Galaxy
        (0x9e0cf8, 0x9d039c), // Kansas City Wizards
        (0x9e0cec, 0x9d03a0), // Dallas Burn
        (0x9e0cdc, 0x9d03a4), // Columbus Crew
        (0x9e0ccc, 0x9d03a8), // Colorado Rapids
        (0x98e2e8, 0x9d03ac), // Albacete Balompié
        (0x9e0cb8, 0x9d03b0), // Albacete Balompié B
        (0x98e29c, 0x9d03b4), // C.D. Badajoz
        (0x9e0ca8, 0x9d03b8), // C.D. Badajoz B
        (0x98e19c, 0x9d03bc), // C.D. Leganés
        (0x9e0c98, 0x9d03c0), // C.D. Leganés B
        (0x98e38c, 0x9d03c4), // C.D. Tenerife
        (0x9e0c88, 0x9d03c8), // C.D. Tenerife B
        (0x9e0c7c, 0x9d03cc), // C.P. Mérida
        (0x9e0c6c, 0x9d03d0), // C.P. Mérida B
        (0x9e0c54, 0x9d03d0), // C.P. Mérida Promesas
        (0x9e0c40, 0x9d03d4), // Deportivo Alavés B
        (0x98e1f8, 0x9d03d8), // Extremadura C.F.
        (0x9e0c2c, 0x9d03dc), // Extremadura C.F. B
        (0x98e0d4, 0x9d03e0), // Racing Club de Santander
        (0x9e0c10, 0x9d03e4), // Racing Club de Santander B
        (0x98e410, 0x9d03e8), // Rayo Vallecano de Madrid
        (0x9e0bf4, 0x9d03ec), // Rayo Vallecano de Madrid B
        (0x98e160, 0x9d03f0), // Real Murcia C.F.
        (0x9e0be0, 0x9d03f4), // Real Murcia C.F. B
        (0x98e064, 0x9d03f8), // Real Sporting de Gijón
        (0x9e0bc4, 0x9d03fc), // Real Sporting de Gijón B
        (0x98e30c, 0x9d0400), // Real Zaragoza
        (0x9e0bb4, 0x9d0404), // Real Zaragoza B
        (0x9e0ba4, 0x9d0408), // S.D. Compostela
        (0x9e0b90, 0x9d040c), // S.D. Compostela B
        (0x98e498, 0x9d0410), // U.D. Las Palmas
        (0x9e0b7c, 0x9d0414), // U.D. Las Palmas B
        (0x98e08c, 0x9d0418), // U.D. Salamanca
        (0x9e0b68, 0x9d041c), // U.D. Salamanca B
        (0x9e0b5c, 0x9d0420), // Galatasaray
        (0x9e0b4c, 0x9d0420), // Galatasaray SK
        (0x9e0b3c, 0x9d0424), // BJK Besiktas
        (0x9e0b30, 0x9d0424), // Besiktas JK
        (0x9e0b24, 0x9d0428), // Fenerbahçe
        (0x9e0b14, 0x9d0428), // Fenerbahçe SK
        (0x9e0af4, 0x9d042c), // General Paz Juniors de Córdoba
        (0x9e0ae4, 0x9d0430), // Swansea City
        (0x9e0ad4, 0x9d0434), // Cardiff City
        (0x9e0ac4, 0x9d0438), // Merthyr Tydfil
        (0x9e0abc, 0x9d043c), // Wrexham
        (0x9e0aa0, 0x9d0440), // Sporting Clube De Portugal
        (0x9e0a80, 0x9d0444), // Sporting Clube De Portugal B
    ],
    // cadeia 4
    &[
        (0x9e0a6c, 0x9d0448), // Arminia Bielefeld
        (0x9e0a50, 0x9d044c), // Arminia Bielefeld Amateure
        (0x9e0a38, 0x9d044c), // Arminia Bielefeld II
        (0x9e0a24, 0x9d0450), // FC Energie Cottbus
        (0x9e0a08, 0x9d0454), // FC Energie Cottbus Amateure
        (0x9e09f0, 0x9d0454), // FC Energie Cottbus II
        (0x9e09e0, 0x9d0458), // MSV Duisburg
        (0x9e09c8, 0x9d045c), // MSV Duisburg Amateure
        (0x9e09bc, 0x9d0460), // Hannover 96
        (0x9e09a4, 0x9d0464), // Hannover 96 Amateure
        (0x9e0998, 0x9d0468), // 1.FC Köln
        (0x9e0984, 0x9d046c), // 1.FC Köln Amateure
        (0x9e0974, 0x9d046c), // 1.FC Köln U23
        (0x9e0960, 0x9d0470), // Borussia M'gladbach
        (0x9e094c, 0x9d0470), // Borussia Mgladbach
        (0x9e0930, 0x9d0474), // Bor. M'gladbach Amateure
        (0x9e0920, 0x9d0478), // 1.FC Nürnberg
        (0x9e0908, 0x9d047c), // 1.FC Nürnberg Amateure
        (0x9e08f0, 0x9d0480), // Rot-Weiss Oberhausen
        (0x9e08dc, 0x9d0480), // Rot-Weiß Oberhausen
        (0x9e08c4, 0x9d0484), // RW Oberhausen Amateure
        (0x9e08b8, 0x9d0488), // SC Freiburg
        (0x9e08a0, 0x9d048c), // SC Freiburg Amateure
        (0x9e088c, 0x9d0490), // 1.FC Saarbrücken
        (0x9e0870, 0x9d0494), // 1.FC Saarbrücken Amateure
        (0x9e085c, 0x9d0494), // 1.FC Saarbrücken II
        (0x9e084c, 0x9d0498), // SSV Ulm 1846
        (0x9e0834, 0x9d049c), // SSV Ulm 1846 Amateure
        (0x9e0824, 0x9d049c), // SSV Ulm 1846 II
        (0x9e0810, 0x9d04a0), // SpVgg Unterhaching
        (0x9e07f4, 0x9d04a4), // SpVgg Unterhaching Amateure
        (0x9e07dc, 0x9d04a4), // SpVgg Unterhaching II
        (0x9e07d0, 0x9d04a8), // VfB Leipzig
        (0x9e07b8, 0x9d04ac), // VfB Leipzig Amateure
        (0x9e07a8, 0x9d04ac), // VfB Leipzig II
        (0x9e0798, 0x9d04b0), // VfL Wolfsburg
        (0x9e0780, 0x9d04b4), // VfL Wolfsburg Amateure
        (0x9e0770, 0x9d04b8), // C.D. Castellón
        (0x9e075c, 0x9d04bc), // C.D. Castellón B
        (0x98e29c, 0x9d04c0), // C.D. Badajoz
        (0x9e0ca8, 0x9d04c4), // C.D. Badajoz B
        (0x9e1bf8, 0x9d04c8), // C.D. Logroñés
        (0x9e1be8, 0x9d04cc), // C.D. Logroñés B
        (0x9e074c, 0x9d04d0), // C.D. Ourense
        (0x9e073c, 0x9d04d4), // C.D. Ourense B
        (0x98e268, 0x9d04d8), // Córdoba C.F.
        (0x9e072c, 0x9d04dc), // Córdoba C.F. B
        (0x9e0718, 0x9d04e0), // Cultural Leonesa
        (0x9e0704, 0x9d04e4), // Cultural Leonesa B
        (0x98e218, 0x9d04e8), // Elche C.F.
        (0x9e06f4, 0x9d04ec), // Elche C.F. B
        (0x98e180, 0x9d04f0), // Levante U.D.
        (0x9e06e4, 0x9d04f4), // Levante U.D. B
        (0x9e06d0, 0x9d04f8), // Real Unión de Irún
        (0x9e06b8, 0x9d04fc), // Real Unión de Irún B
        (0x98e0ac, 0x9d0500), // Recreativo de Huelva
        (0x9e06a0, 0x9d0504), // Recreativo de Huelva B
        (0x9e068c, 0x9d0508), // Fortuna Düsseldorf
        (0x9e0674, 0x9d050c), // Fortuna Düsseldorf ii
        (0x9e0668, 0x9d0510), // sg hoechst
        (0x9e0658, 0x9d0514), // sg hoechst ii
        (0x9e0648, 0x9d0518), // fsv mainz 05
        (0x9e0638, 0x9d051c), // fsv mainz 05 ii
        (0x9e0624, 0x9d0520), // stuttgarter kickers
        (0x9e060c, 0x9d0524), // stuttgarter kickers ii
    ],
];

/// As chaves de nação do index.cpp (0x60d810), na ordem do código: (texto do nome, global que recebe a linha), em 2 cadeias.
pub(crate) static CHAVES_DE_NACAO: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9d9e30, KEY_NATION_AFGHANISTAN as u32), // Afghanistan
        (0x9d9e28, KEY_NATION_ALBANIA as u32), // Albania
        (0x9d9e20, KEY_NATION_ALGERIA as u32), // Algeria
        (0x9d9e18, KEY_NATION_ANDORRA as u32), // Andorra
        (0x9d9e10, KEY_NATION_ANGOLA as u32), // Angola
        (0x9d9dfc, KEY_NATION_ANTIGUA_BARBUDA as u32), // Antigua & Barbuda
        (0x9b1698, KEY_NATION_ARGENTINA as u32), // Argentina
        (0x9d9df4, KEY_NATION_ARMENIA as u32), // Armenia
        (0x9d9dec, KEY_NATION_ARUBA as u32), // Aruba
        (0x9b167c, KEY_NATION_AUSTRALIA as u32), // Australia
        (0x9b1664, KEY_NATION_AUSTRIA as u32), // Austria
        (0x9d9de0, KEY_NATION_AZERBAIJAN as u32), // Azerbaijan
        (0x9d9dd4, KEY_NATION_THE_BAHAMAS as u32), // The Bahamas
        (0x9d9dcc, KEY_NATION_THE_BAHAMAS as u32), // Bahamas
        (0x9d9dc4, KEY_NATION_BAHRAIN as u32), // Bahrain
        (0x9d9db8, KEY_NATION_BANGLADESH as u32), // Bangladesh
        (0x9d9dac, KEY_NATION_BARBADOS as u32), // Barbados
        (0x9d9da4, KEY_NATION_BELARUS as u32), // Belarus
        (0x9b164c, KEY_NATION_BELGIUM as u32), // Belgium
        (0x9d9d9c, KEY_NATION_BELIZE as u32), // Belize
        (0x9d9d94, KEY_NATION_BENIN as u32), // Benin
        (0x9d9d8c, KEY_NATION_BERMUDA as u32), // Bermuda
        (0x9d9d84, KEY_NATION_BOLIVIA as u32), // Bolivia
        (0x9d9d70, KEY_NATION_BOSNIA_HERZEGOVINA as u32), // Bosnia-Herzegovina
        (0x9d9d64, KEY_NATION_BOTSWANA as u32), // Botswana
        (0x9b1620, KEY_NATION_BRAZIL as u32), // Brazil
        (0x9d9d50, KEY_NATION_BRUNEI_DARUSSALAM as u32), // Brunei Darussalam
        (0x9d9d44, KEY_NATION_BULGARIA as u32), // Bulgaria
        (0x9d9d34, KEY_NATION_BURKINO_FASO as u32), // Burkino Faso
        (0x9d9d24, KEY_NATION_BURKINO_FASO as u32), // Burkina Faso
        (0x9d9d1c, KEY_NATION_BURUNDI as u32), // Burundi
        (0x9d9cfc, KEY_NATION_COMMONWEALTH_OF_SOVIET_STATES as u32), // Commonwealth of Soviet States
        (0x9d9cf0, KEY_NATION_CAMEROON as u32), // Cameroon
        (0x9d9ce8, KEY_NATION_CANADA as u32), // Canada
        (0x9d9cd4, KEY_NATION_CAPE_VERDE_ISLANDS as u32), // Cape Verde Islands
        (0x9d9cc4, KEY_NATION_CAYMAN_ISLANDS as u32), // Cayman Islands
        (0x9d9ca8, KEY_NATION_CENTRAL_AFRICAN_REPUBLIC as u32), // Central African Republic
        (0x9d9ca0, KEY_NATION_CHAD as u32), // Chad
        (0x9d9c98, KEY_NATION_CHILE as u32), // Chile
        (0x9b1608, KEY_NATION_CHINA as u32), // China
        (0x9d9c8c, KEY_NATION_CHINA as u32), // China PR
        (0x9d9c80, KEY_NATION_COLOMBIA as u32), // Colombia
        (0x9d9c74, KEY_NATION_THE_CONGO as u32), // The Congo
        (0x9d9c64, KEY_NATION_COOK_ISLANDS as u32), // Cook Islands
        (0x9d9c58, KEY_NATION_COSTA_RICA as u32), // Costa Rica
        (0x9b15f4, KEY_NATION_CROATIA as u32), // Croatia
        (0x9d9c50, KEY_NATION_CUBA as u32), // Cuba
        (0x9d9c48, KEY_NATION_CYPRUS as u32), // Cyprus
        (0x9b15d0, KEY_NATION_CZECH_REPUBLIC as u32), // Czech Republic
        (0x9d9c38, KEY_NATION_CZECHOSLOVAKIA as u32), // Czechoslovakia
        (0x9b15b8, KEY_NATION_DENMARK as u32), // Denmark
        (0x9d9c2c, KEY_NATION_DJIBOUTI as u32), // Djibouti
        (0x9d9c20, KEY_NATION_DOMINICA as u32), // Dominica
        (0x9d9c0c, KEY_NATION_DOMINICAN_REPUBLIC as u32), // Dominican Republic
        (0x9d9bfc, KEY_NATION_EAST_GERMANY as u32), // East Germany
        (0x9d9bf4, KEY_NATION_ECUADOR as u32), // Ecuador
        (0x9d9bec, KEY_NATION_EGYPT as u32), // Egypt
        (0x9d9bd8, KEY_NATION_REPUBLIC_OF_IRELAND as u32), // Republic Of Ireland
        (0x9d9bcc, KEY_NATION_EL_SALVADOR as u32), // El Salvador
        (0x9d9bc4, KEY_NATION_ENGLAND as u32), // England
        (0x9d9bb0, KEY_NATION_EQUATORIAL_GUINEA as u32), // Equatorial Guinea
        (0x9d9ba8, KEY_NATION_ESTONIA as u32), // Estonia
        (0x9d9b9c, KEY_NATION_ETHIOPIA as u32), // Ethiopia
        (0x9d9b8c, KEY_NATION_FAROE_ISLANDS as u32), // Faroe Islands
        (0x9d9b84, KEY_NATION_FIJI as u32), // Fiji
        (0x9b159c, KEY_NATION_FINLAND as u32), // Finland
        (0x9b1584, KEY_NATION_FRANCE as u32), // France
        (0x9d9b70, KEY_NATION_FYR_OF_MACEDONIA as u32), // FYR of Macedonia
        (0x9d9b68, KEY_NATION_GABON as u32), // Gabon
        (0x9d9b5c, KEY_NATION_THE_GAMBIA as u32), // The Gambia
        (0x9d9b54, KEY_NATION_GEORGIA as u32), // Georgia
        (0x9b1568, KEY_NATION_GERMANY as u32), // Germany
        (0x9d9b4c, KEY_NATION_GHANA as u32), // Ghana
        (0x9b1550, KEY_NATION_GREECE as u32), // Greece
        (0x9d9b44, KEY_NATION_GRENADA as u32), // Grenada
        (0x9d9b38, KEY_NATION_GUATEMALA as u32), // Guatemala
        (0x9d9b30, KEY_NATION_GUINEA as u32), // Guinea
        (0x9d9b20, KEY_NATION_GUINEA_BISSAU as u32), // Guinea-Bissau
        (0x9d9b18, KEY_NATION_GUYANA as u32), // Guyana
        (0x9d9b10, KEY_NATION_HAITI as u32), // Haiti
        (0x9b1534, KEY_NATION_HOLLAND as u32), // Holland
        (0x9d9b04, KEY_NATION_HONDURAS as u32), // Honduras
        (0x9d9af8, KEY_NATION_HONG_KONG as u32), // Hong Kong
        (0x9d9af0, KEY_NATION_HUNGARY as u32), // Hungary
        (0x9d9ae8, KEY_NATION_ICELAND as u32), // Iceland
        (0x9d9ae0, KEY_NATION_INDIA as u32), // India
        (0x9d9ad4, KEY_NATION_INDONESIA as u32), // Indonesia
        (0x9d9acc, KEY_NATION_IRAN as u32), // Iran
        (0x9d9ac4, KEY_NATION_IRAQ as u32), // Iraq
        (0x9d9abc, KEY_NATION_ISRAEL as u32), // Israel
        (0x9b150c, KEY_NATION_ITALY as u32), // Italy
        (0x9d9ab0, KEY_NATION_IVORY_COAST as u32), // Ivory Coast
        (0x9d9aa8, KEY_NATION_JAMAICA as u32), // Jamaica
        (0x9b14ec, KEY_NATION_JAPAN as u32), // Japan
        (0x9d9aa0, KEY_NATION_JORDAN as u32), // Jordan
        (0x9d9a94, KEY_NATION_KAMPUCHEA as u32), // Kampuchea
        (0x9d9a88, KEY_NATION_KAMPUCHEA as u32), // Cambodia
        (0x9d9a7c, KEY_NATION_KAZAKHSTAN as u32), // Kazakhstan
        (0x9d9a74, KEY_NATION_KENYA as u32), // Kenya
        (0x9d9a6c, KEY_NATION_KUWAIT as u32), // Kuwait
        (0x9d9a60, KEY_NATION_KYRGYZSTAN as u32), // Kyrgyzstan
        (0x9d9a58, KEY_NATION_LAOS as u32), // Laos
        (0x9d9a50, KEY_NATION_LATVIA as u32), // Latvia
        (0x9d9a48, KEY_NATION_LEBANON as u32), // Lebanon
        (0x9d9a40, KEY_NATION_LESOTHO as u32), // Lesotho
        (0x9d9a38, KEY_NATION_LIBERIA as u32), // Liberia
        (0x9d9a30, KEY_NATION_LIBYA as u32), // Libya
        (0x9d9a20, KEY_NATION_LIECHTENSTEIN as u32), // Liechtenstein
        (0x9d9a14, KEY_NATION_LITHUANIA as u32), // Lithuania
        (0x9d9a08, KEY_NATION_LUXEMBOURG as u32), // Luxembourg
        (0x9d9a00, KEY_NATION_MACAU as u32), // Macau
        (0x9d99f4, KEY_NATION_MADAGASCAR as u32), // Madagascar
        (0x9d99ec, KEY_NATION_MALAWI as u32), // Malawi
        (0x9d99e0, KEY_NATION_MALAYSIA as u32), // Malaysia
        (0x9d99d4, KEY_NATION_MALDIVES as u32), // Maldives
        (0x9d99cc, KEY_NATION_MALI as u32), // Mali
        (0x9d99c4, KEY_NATION_MALTA as u32), // Malta
        (0x9d99b8, KEY_NATION_MAURITANIA as u32), // Mauritania
        (0x9d99ac, KEY_NATION_MAURITIUS as u32), // Mauritius
        (0x9b14bc, KEY_NATION_MEXICO as u32), // Mexico
        (0x9d99a4, KEY_NATION_MOLDOVA as u32), // Moldova
        (0x9d999c, KEY_NATION_MOROCCO as u32), // Morocco
        (0x9d9990, KEY_NATION_MOZAMBIQUE as u32), // Mozambique
        (0x9d9988, KEY_NATION_MYANMAR as u32), // Myanmar
        (0x9d9978, KEY_NATION_MYANMAR as u32), // Myanmar (Burma)
        (0x9b1494, KEY_NATION_NORTHERN_IRELAND as u32), // Northern Ireland
        (0x9d9970, KEY_NATION_NAMIBIA as u32), // Namibia
    ],
    // cadeia 2
    &[
        (0x9d9968, KEY_NATION_NEPAL as u32), // Nepal
        (0x9d9950, KEY_NATION_NETHERLANDS_ANTILLES as u32), // Netherlands Antilles
        (0x9d9944, KEY_NATION_NEW_ZEALAND as u32), // New Zealand
        (0x9d9938, KEY_NATION_NICARAGUA as u32), // Nicaragua
        (0x9d9930, KEY_NATION_NIGER as u32), // Niger
        (0x9d9928, KEY_NATION_NIGERIA as u32), // Nigeria
        (0x9d991c, KEY_NATION_NORTH_KOREA as u32), // North Korea
        (0x9b147c, KEY_NATION_NORWAY as u32), // Norway
        (0x9d9914, KEY_NATION_OMAN as u32), // Oman
        (0x9d9908, KEY_NATION_PAKISTAN as u32), // Pakistan
        (0x9d98fc, KEY_NATION_PALESTINE as u32), // Palestine
        (0x9d98f4, KEY_NATION_PANAMA as u32), // Panama
        (0x9d98e0, KEY_NATION_PAPUA_NEW_GUINEA as u32), // Papua New Guinea
        (0x9d98d4, KEY_NATION_PARAGUAY as u32), // Paraguay
        (0x9d98c8, KEY_NATION_PAYS_BASQUE as u32), // Pays Basque
        (0x9d98c0, KEY_NATION_PERU as u32), // Peru
        (0x9d98b0, KEY_NATION_THE_PHILIPPINES as u32), // The Philippines
        (0x9b1450, KEY_NATION_POLAND as u32), // Poland
        (0x9b1430, KEY_NATION_PORTUGAL as u32), // Portugal
        (0x9d98a4, KEY_NATION_PUERTO_RICO as u32), // Puerto Rico
        (0x9d989c, KEY_NATION_QATAR as u32), // Qatar
        (0x9d987c, KEY_NATION_DEMOCRATIC_REPUBLIC_OF_CONGO as u32), // Democratic Republic of Congo
        (0x9d9874, KEY_NATION_ROMANIA as u32), // Romania
        (0x9b13ec, KEY_NATION_RUSSIA as u32), // Russia
        (0x9d986c, KEY_NATION_RWANDA as u32), // Rwanda
        (0x9d9860, KEY_NATION_SAN_MARINO as u32), // San Marino
        (0x9d984c, KEY_NATION_SAO_TOME_PRINCIPE as u32), // Sao Tome & Principe
        (0x9d9838, KEY_NATION_SAO_TOME_PRINCIPE as u32), // São Tomé & Principe
        (0x9d9828, KEY_NATION_SAUDI_ARABIA as u32), // Saudi Arabia
        (0x9b13bc, KEY_NATION_SCOTLAND as u32), // Scotland
        (0x9d9820, KEY_NATION_SENEGAL as u32), // Senegal
        (0x9d9814, KEY_NATION_SEYCHELLES as u32), // Seychelles
        (0x9d9804, KEY_NATION_SIERRA_LEONE as u32), // Sierra Leone
        (0x9d97f8, KEY_NATION_SINGAPORE as u32), // Singapore
        (0x9d97ec, KEY_NATION_SLOVAKIA as u32), // Slovakia
        (0x9d97e0, KEY_NATION_SLOVENIA as u32), // Slovenia
        (0x9d97d0, KEY_NATION_SOLOMON_ISLANDS as u32), // Solomon Islands
        (0x9d97c8, KEY_NATION_SOMALIA as u32), // Somalia
        (0x9b1384, KEY_NATION_SOUTH_AFRICA as u32), // South Africa
        (0x9b14d0, KEY_NATION_SOUTH_KOREA as u32), // South Korea
        (0x9b136c, KEY_NATION_SPAIN as u32), // Spain
        (0x9d97bc, KEY_NATION_SRI_LANKA as u32), // Sri Lanka
        (0x9d97a8, KEY_NATION_ST_KITTS_NEVIS as u32), // St Kitts & Nevis
        (0x9d979c, KEY_NATION_SAINT_LUCIA as u32), // Saint Lucia
        (0x9d9780, KEY_NATION_ST_VINCENT_THE_GRENADINES as u32), // St Vincent & The Grenadines
        (0x9d9778, KEY_NATION_SUDAN as u32), // Sudan
        (0x9d9770, KEY_NATION_SURINAM as u32), // Surinam
        (0x9d9764, KEY_NATION_SWAZILAND as u32), // Swaziland
        (0x9b1334, KEY_NATION_SWEDEN as u32), // Sweden
        (0x9b1304, KEY_NATION_SWITZERLAND as u32), // Switzerland
        (0x9d975c, KEY_NATION_SYRIA as u32), // Syria
        (0x9d9754, KEY_NATION_TAHITI as u32), // Tahiti
        (0x9d974c, KEY_NATION_TAIWAN as u32), // Taiwan
        (0x9d973c, KEY_NATION_TAIWAN as u32), // Chinese Taipei
        (0x9d9730, KEY_NATION_TAJIKISTAN as u32), // Tajikistan
        (0x9d9724, KEY_NATION_TANZANIA as u32), // Tanzania
        (0x9d9718, KEY_NATION_THAILAND as u32), // Thailand
        (0x9d9710, KEY_NATION_TIMOR as u32), // Timor
        (0x9d9708, KEY_NATION_TOGO as u32), // Togo
        (0x9d9700, KEY_NATION_TONGA as u32), // Tonga
        (0x9d96ec, KEY_NATION_TRINIDAD_TOBAGO as u32), // Trinidad & Tobago
        (0x9d96e4, KEY_NATION_TUNISIA as u32), // Tunisia
        (0x9b12ec, KEY_NATION_TURKEY as u32), // Turkey
        (0x9d96d4, KEY_NATION_TURKMENISTAN as u32), // Turkmenistan
        (0x9d96bc, KEY_NATION_UNITED_ARAB_EMIRATES as u32), // United Arab Emirates
        (0x9d96ac, KEY_NATION_SOVIET_UNION as u32), // Soviet Union
        (0x9d96a4, KEY_NATION_UGANDA as u32), // Uganda
        (0x9d969c, KEY_NATION_UKRAINE as u32), // Ukraine
        (0x9d9694, KEY_NATION_URUGUAY as u32), // Uruguay
        (0x9d9684, KEY_NATION_UNITED_STATES as u32), // United States
        (0x9d9678, KEY_NATION_UZBEKISTAN as u32), // Uzbekistan
        (0x9d9670, KEY_NATION_VANUATU as u32), // Vanuatu
        (0x9d9664, KEY_NATION_VENEZUELA as u32), // Venezuela
        (0x9d965c, KEY_NATION_VIETNAM as u32), // Vietnam
        (0x9d964c, KEY_NATION_WEST_GERMANY as u32), // West Germany
        (0x9b12c0, KEY_NATION_WALES as u32), // Wales
        (0x9d963c, KEY_NATION_WESTERN_SAMOA as u32), // Western Samoa
        (0x9d9634, KEY_NATION_WESTERN_SAMOA as u32), // Samoa
        (0x9d962c, KEY_NATION_YEMEN as u32), // Yemen
        (0x9b12a4, KEY_NATION_YUGOSLAVIA as u32), // Yugoslavia
        (0x9d9624, KEY_NATION_ZAIRE as u32), // Zaire
        (0x9d961c, KEY_NATION_ZAMBIA as u32), // Zambia
        (0x9d9610, KEY_NATION_ZIMBABWE as u32), // Zimbabwe
        (0x9d9600, KEY_NATION_AMERICAN_SAMOA as u32), // American Samoa
        (0x9d95f0, KEY_NATION_NEW_CALEDONIA as u32), // New Caledonia
        (0x9d95d8, KEY_NATION_BRITISH_VIRGIN_ISLANDS as u32), // British Virgin Islands
        (0x9d95bc, KEY_NATION_TURKS_AND_CAICOS_ISLANDS as u32), // Turks and Caicos Islands
        (0x9d95a8, KEY_NATION_US_VIRGIN_ISLANDS as u32), // US Virgin Islands
        (0x9d959c, KEY_NATION_ANGUILLA as u32), // Anguilla
        (0x9d9590, KEY_NATION_MONTSERRAT as u32), // Montserrat
    ],
];

/// As globais de clube na ordem em que 0x60ba80 as compara (as mesmas 313 de CHAVES_DE_CLUBE).
pub(crate) static CHAVES_DE_CLUBE_NA_TROCA: &[u32] = &[
    0x9d00a4, // MLS All Stars East
    0x9d00a8, // MLS All Stars West
    0x9d00ac, // River Plate
    0x9d00b0, // Vélez Sarsfield
    0x9d00b4, // Independiente
    0x9d00b8, // Independiente Rivadavia de Mendoza
    0x9d00bc, // Racing de Córdoba
    0x9d00c0, // Villa Mitre de Bahía Blanca
    0x9d00c4, // Argentino de Rosario
    0x9d00c8, // Temperley
    0x9d00cc, // Lanús
    0x9d00d0, // Boca Juniors
    0x9d00d4, // Gimnasia y Esgrima de La Plata
    0x9d00d8, // San Lorenzo de Almagro
    0x9d00dc, // Argentinos Juniors
    0x9d00e0, // Rosario Central
    0x9d00e4, // Newell's Old Boys
    0x9d00e8, // Racing Club
    0x9d00ec, // Colón de Santa Fé
    0x9d00f0, // Platense
    0x9d00f4, // Ferro Carril Oeste
    0x9d00f8, // Estudiantes de La Plata
    0x9d00fc, // Gimnasia y Esgrima de Jujuy
    0x9d0100, // Unión de Santa Fé
    0x9d0104, // Huracán
    0x9d0108, // Talleres de Córdoba
    0x9d010c, // Belgrano de Córdoba
    0x9d0110, // Arsenal
    0x9d0114, // Crystal Palace
    0x9d0118, // Glasgow Celtic
    0x9d011c, // Glasgow Rangers
    0x9d0120, // Athletic Club de Bilbao
    0x9d0124, // C.D. Alavés
    0x9d0128, // Real Sociedad C.F.
    0x9d012c, // Atlético de Madrid
    0x9d0130, // Atlético de Madrid B
    0x9d0134, // Athletic Club de Bilbao B
    0x9d0138, // F.C. Barcelona
    0x9d013c, // F.C. Barcelona B
    0x9d0140, // Real Valladolid
    0x9d0144, // Real Valladolid B
    0x9d0148, // Málaga C.F.
    0x9d014c, // Málaga C.F. B
    0x9d0150, // Real Betis Balompié
    0x9d0154, // Real Betis Balompié B
    0x9d0158, // Real Club Celta de Vigo
    0x9d015c, // Real Club Celta de Vigo B
    0x9d0160, // Deportivo de La Coruña
    0x9d0164, // Deportivo de La Coruña B
    0x9d0168, // R.C.D. Espanyol
    0x9d016c, // R.C.D. Espanyol B
    0x9d0170, // C.D. Logroñés
    0x9d0174, // C.D. Logroñés B
    0x9d0178, // R.C.D. Mallorca
    0x9d017c, // R.C.D. Mallorca B
    0x9d0180, // Club Atlético Osasuna
    0x9d0184, // Club Atlético Osasuna B
    0x9d0188, // Real Oviedo C.F.
    0x9d018c, // Real Oviedo C.F. B
    0x9d0190, // Real Madrid C.F.
    0x9d0194, // Real Madrid C.F. B
    0x9d0198, // Sevilla C.F.
    0x9d019c, // Sevilla C.F. B
    0x9d01a0, // Real Sociedad C.F. B
    0x9d01a4, // Instituto de Córdoba
    0x9d01a8, // San Martín de San Juan
    0x9d01ac, // Gimnasia y Tiro de Salta
    0x9d01b0, // Atlético Rafaela
    0x9d01b4, // San Martín de Tucumán
    0x9d01b8, // Almirante Brown de Arrecifes
    0x9d01bc, // Atlético Tucumán
    0x9d01c0, // Olimpo de Bahía Blanca
    0x9d01c4, // Huracán de Corrientes
    0x9d01c8, // Aldosivi
    0x9d01cc, // Cipolletti de Río Negro
    0x9d01d0, // San Martín de Mendoza
    0x9d01d4, // Douglas Haig
    0x9d01d8, // Gimnasia y Esgrima de Concepción del Uruguay
    0x9d01dc, // Juventud Antoniana de Salta
    0x9d01e0, // Godoy Cruz de Mendoza
    0x9d01e4, // Banfield
    0x9d01e8, // Los Andes
    0x9d01ec, // Quilmes
    0x9d01f0, // Central Córdoba de Rosario
    0x9d01f4, // Chacarita Juniors
    0x9d01f8, // All Boys
    0x9d01fc, // San Miguel
    0x9d0200, // Nueva Chicago
    0x9d0204, // Defensa y Justicia
    0x9d0208, // Arsenal de Sarandí
    0x9d020c, // Huracán de Tres Arroyos
    0x9d0210, // Defensores de Belgrano
    0x9d0214, // Deportivo Morón
    0x9d0218, // Estudiantes de Buenos Aires
    0x9d021c, // Almagro
    0x9d0220, // Atlanta
    0x9d0224, // Deportivo Español
    0x9d0228, // El Porvenir
    0x9d022c, // Tigre
    0x9d0230, // Middlesbrough
    0x9d0234, // Newcastle United
    0x9d0238, // Sunderland
    0x9d023c, // Aston Villa
    0x9d0240, // Manchester United
    0x9d0244, // Liverpool
    0x9d0248, // Everton
    0x9d024c, // Tottenham Hotspur
    0x9d0250, // Chelsea
    0x9d0254, // Valencia C.F.
    0x9d0258, // Valencia C.F. B
    0x9d025c, // Brighton and Hove Albion
    0x9d0260, // Southampton
    0x9d0264, // AS Monaco
    0x9d0268, // FC Metz
    0x9d026c, // Cruzeiro Esporte Clube
    0x9d0270, // São Paulo Futebol Clube
    0x9d0274, // Sociedade Esportiva Palmeiras
    0x9d0278, // Sport Club Corinthians Paulista
    0x9d027c, // Gremio Foot Ball Porto Alegre
    0x9d0354, // Clube de Regatas Vasco da Gama
    0x9d0280, // Club Social y Deportivo Colo Colo
    0x9d0284, // Univ.de Chile
    0x9d0288, // Univ.Catolica
    0x9d028c, // Club Nacional de Fútbol
    0x9d0290, // Olimpia
    0x9d0294, // Cerro Porteño
    0x9d0298, // Club Atlético Peñarol
    0x9d029c, // Elgin City
    0x9d02a0, // Peterhead
    0xb63cdc, // 1.FC Kaiserslautern
    0xb63c9c, // 1.FC Kaiserslautern Amateure
    0xb63cbc, // Bayer 04 Leverkusen
    0xb63ca0, // Bayer Leverkusen Amateure
    0xb63ce8, // Borussia Dortmund
    0xb63cac, // Borussia Dortmund Amateure
    0xb63cd8, // FC Bayern München
    0xb63cd0, // Bayern München Amateure
    0xb63cb4, // FC St. Pauli
    0xb63cb0, // FC St. Pauli Amateure
    0xb63cd4, // Hamburger SV
    0xb63ca8, // Hamburger SV Amateure
    0xb63ce4, // Hertha BSC Berlin
    0xb63cec, // Hertha BSC Amateure
    0xb63cf0, // Karlsruher SC
    0xb63cf8, // Karlsruher SC Amateure
    0xb63cc0, // SV Werder Bremen
    0xb63d00, // SV Werder Bremen Amateure
    0xb63cc8, // TSV München 1860
    0xb63ccc, // TSV München 1860 Amateure
    0xb63ce0, // Tennis Borussia Berlin
    0xb63cb8, // Tennis Borussia Amateure
    0xb63cc4, // VfB Stuttgart
    0xb63cf4, // VfB Stuttgart Amateure
    0xb63cfc, // VfL Bochum
    0xb63ca4, // VfL Bochum Amateure
    0x9d02a4, // Eintracht Frankfurt
    0x9d02a8, // Eintracht Frankfurt Amateure
    0x9d02ac, // FC Hansa Rostock
    0x9d02b0, // FC Hansa Rostock Amateure
    0x9d02b4, // FC Schalke 04
    0x9d02b8, // FC Schalke 04 Amateure
    0x9d02bc, // SG Wattenscheid 09
    0x9d02c0, // SG Wattenscheid 09 Amateure
    0x9d02c4, // Sport Lisboa e Benfica
    0x9d02c8, // Sport Lisboa e Benfica B
    0x9d02cc, // Sporting Clube de Braga
    0x9d02d0, // Sporting Clube de Braga B
    0x9d02d4, // Clube Sport Marítimo
    0x9d02d8, // Clube Sport Marítimo B
    0x9d02dc, // Futebol Clube do Porto
    0x9d02e0, // Futebol Clube do Porto B
    0x9d02e4, // S.D. Éibar
    0x9d02e8, // Amurrio C.F.
    0x9d02ec, // C.D. Aurrerá Vitoria
    0x9d02f0, // Barakaldo C.F.
    0x9d02f4, // S.D. Beasaín
    0x9d02f8, // Bermeo Club
    0x9d02fc, // Gernika Club
    0x9d0304, // S.D. Lemona
    0x9d0308, // Elgoibar C.D.
    0x9d030c, // Hernani C.D.
    0x9d0310, // Zalla U.C.
    0x9d0314, // Cultural Durango
    0x9d0318, // S.D. Amorebieta
    0x9d031c, // Arenas Getxo Bilbao
    0x9d0320, // Aurrerá Ondarroa C.D.
    0x9d0324, // Baskonia C.D.
    0x9d0328, // S.D. Éibar B
    0x9d032c, // U.D. San Pedro
    0x9d0330, // Santurtzi C.D.
    0x9d0334, // Sestao River Club
    0x9d0338, // Tolosa C.F.
    0x9d033c, // Coritiba Football Club
    0x9d0340, // Santos Futebol Clube
    0x9d0344, // Sport Club do Recife
    0x9d0348, // Associação Portugesa de Desportos
    0x9d0350, // Clube Atlético Mineiro
    0x9d0358, // Clube de Regatas Flamengo
    0x9d035c, // Sport Club Internacional
    0x9d0360, // Esporte Clube Vitória
    0x9d0364, // Botafogo Futebol Clube
    0x9d0368, // Clube Atlético Paranaense
    0x9d036c, // Associação Atlética Ponte Preta
    0x9d0370, // Esporte Clube Juventude
    0x9d0374, // Guarani Futebol Clube
    0x9d0378, // Paraná Clube
    0x9d037c, // Chicago Fire
    0x9d0380, // Miami Fusion FC
    0x9d0384, // Washington DC United
    0x9d0388, // Tampa Bay Mutiny
    0x9d038c, // San Jose Earthquakes
    0x9d0390, // New England Revolution
    0x9d0394, // NY/NJ Metrostars
    0x9d0398, // Los Angeles Galaxy
    0x9d039c, // Kansas City Wizards
    0x9d03a0, // Dallas Burn
    0x9d03a4, // Columbus Crew
    0x9d03a8, // Colorado Rapids
    0x9d03ac, // Albacete Balompié
    0x9d03b0, // Albacete Balompié B
    0x9d03b4, // C.D. Badajoz
    0x9d03b8, // C.D. Badajoz B
    0x9d03bc, // C.D. Leganés
    0x9d03c0, // C.D. Leganés B
    0x9d03c4, // C.D. Tenerife
    0x9d03c8, // C.D. Tenerife B
    0x9d03cc, // C.P. Mérida
    0x9d03d0, // C.P. Mérida B
    0x9d03d4, // Deportivo Alavés B
    0x9d03d8, // Extremadura C.F.
    0x9d03dc, // Extremadura C.F. B
    0x9d03e0, // Racing Club de Santander
    0x9d03e4, // Racing Club de Santander B
    0x9d03e8, // Rayo Vallecano de Madrid
    0x9d03ec, // Rayo Vallecano de Madrid B
    0x9d03f0, // Real Murcia C.F.
    0x9d03f4, // Real Murcia C.F. B
    0x9d03f8, // Real Sporting de Gijón
    0x9d03fc, // Real Sporting de Gijón B
    0x9d0400, // Real Zaragoza
    0x9d0404, // Real Zaragoza B
    0x9d0408, // S.D. Compostela
    0x9d040c, // S.D. Compostela B
    0x9d0410, // U.D. Las Palmas
    0x9d0414, // U.D. Las Palmas B
    0x9d0418, // U.D. Salamanca
    0x9d041c, // U.D. Salamanca B
    0x9d0420, // Galatasaray
    0x9d0424, // BJK Besiktas
    0x9d0428, // Fenerbahçe
    0x9d042c, // General Paz Juniors de Córdoba
    0x9d0430, // Swansea City
    0x9d0434, // Cardiff City
    0x9d0438, // Merthyr Tydfil
    0x9d043c, // Wrexham
    0x9d0440, // Sporting Clube De Portugal
    0x9d0444, // Sporting Clube De Portugal B
    0x9d0448, // Arminia Bielefeld
    0x9d044c, // Arminia Bielefeld Amateure
    0x9d0450, // FC Energie Cottbus
    0x9d0454, // FC Energie Cottbus Amateure
    0x9d0458, // MSV Duisburg
    0x9d045c, // MSV Duisburg Amateure
    0x9d0460, // Hannover 96
    0x9d0464, // Hannover 96 Amateure
    0x9d0468, // 1.FC Köln
    0x9d046c, // 1.FC Köln Amateure
    0x9d0470, // Borussia M'gladbach
    0x9d0474, // Bor. M'gladbach Amateure
    0x9d0478, // 1.FC Nürnberg
    0x9d047c, // 1.FC Nürnberg Amateure
    0x9d0480, // Rot-Weiss Oberhausen
    0x9d0484, // RW Oberhausen Amateure
    0x9d0488, // SC Freiburg
    0x9d048c, // SC Freiburg Amateure
    0x9d0490, // 1.FC Saarbrücken
    0x9d0494, // 1.FC Saarbrücken Amateure
    0x9d0498, // SSV Ulm 1846
    0x9d049c, // SSV Ulm 1846 Amateure
    0x9d04a0, // SpVgg Unterhaching
    0x9d04a4, // SpVgg Unterhaching Amateure
    0x9d04a8, // VfB Leipzig
    0x9d04ac, // VfB Leipzig Amateure
    0x9d04b0, // VfL Wolfsburg
    0x9d04b4, // VfL Wolfsburg Amateure
    0x9d04b8, // C.D. Castellón
    0x9d04bc, // C.D. Castellón B
    0x9d04c0, // C.D. Badajoz
    0x9d04c4, // C.D. Badajoz B
    0x9d04c8, // C.D. Logroñés
    0x9d04cc, // C.D. Logroñés B
    0x9d04d0, // C.D. Ourense
    0x9d04d4, // C.D. Ourense B
    0x9d04d8, // Córdoba C.F.
    0x9d04dc, // Córdoba C.F. B
    0x9d04e0, // Cultural Leonesa
    0x9d04e4, // Cultural Leonesa B
    0x9d04e8, // Elche C.F.
    0x9d04ec, // Elche C.F. B
    0x9d04f0, // Levante U.D.
    0x9d04f4, // Levante U.D. B
    0x9d04f8, // Real Unión de Irún
    0x9d04fc, // Real Unión de Irún B
    0x9d0500, // Recreativo de Huelva
    0x9d0504, // Recreativo de Huelva B
    0x9d0508, // Fortuna Düsseldorf
    0x9d050c, // Fortuna Düsseldorf ii
    0x9d0510, // sg hoechst
    0x9d0514, // sg hoechst ii
    0x9d0518, // fsv mainz 05
    0x9d051c, // fsv mainz 05 ii
    0x9d0520, // stuttgarter kickers
    0x9d0524, // stuttgarter kickers ii
];

/// matches the database club name with the 313 fixed club keys (372 names in 4 chains, _stricmp): the first hit of each
/// chain stores the row in the key global
/// Original: `match_club_key_id` (0x6146b0, index.cpp, cdecl).
///
/// [pt] { 0x6146b0 } (cdecl, index.cpp, match_club_key_id): casa o nome de clube da base com as chaves fixas
/// do jogo (CHAVES_DE_CLUBE): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
pub(crate) unsafe extern "C" fn casar_chave_de_clube(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_STRICMP);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_CLUBE.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de clube
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de clube)
}

/// after the club table trim, the first club key holding the old id gets the new one (returns 1; none: 0)
/// Original: `remap_club_key_id` (0x60ba80, index.cpp, cdecl).
///
/// [pt] { 0x60ba80 } (cdecl, index.cpp, remap_club_key_id): depois do corte da tabela de clubes pelo
/// tamanho da base (0x5267c0), a primeira chave de clube (na ordem de CHAVES_DE_CLUBE_NA_TROCA) que
/// guarda o id `antigo` passa a guardar `novo` e devolve 1; nenhuma: 0.
pub(crate) unsafe extern "C" fn remapear_chave_de_clube(antigo: u32, novo: u32) -> u32 {
    for &g in CHAVES_DE_CLUBE_NA_TROCA.iter() {
        if *(g as *const u32) == antigo {
            *(g as *mut u32) = novo; // chave remapeada
            return 1;
        }
    }
    0
}

/// matches the database nation name with the 209 fixed nation keys (217 names in 2 chains, _stricmp)
/// Original: `match_nation_key_id` (0x60d810, index.cpp, cdecl).
///
/// [pt] { 0x60d810 } (cdecl, index.cpp, match_nation_key_id): casa o nome de nação da base com as chaves fixas
/// do jogo (CHAVES_DE_NACAO): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
pub(crate) unsafe extern "C" fn casar_chave_de_nacao(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_STRICMP);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_NACAO.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de nação
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de nação)
}

/// As globais (dword) que ficaram diferentes entre a original e a cópia: "[end] original x rust".
pub(crate) fn chaves_diferentes(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> String {
    let mut s = String::new();
    for ((a, x), (_, y)) in mo.0.iter().zip(mr.0.iter()) {
        for i in (0..x.len()).step_by(4) {
            if x[i..i + 4] != y[i..i + 4] {
                let v = |b: &[u8]| i32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
                s += &format!("[{:x}] {} x {} ", a + i, v(x), v(y));
            }
        }
    }
    s
}

pub(crate) unsafe extern "C" fn comparar_casar_chave_de_clube(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_CASAR_CHAVE_DE_CLUBE);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_clube(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_clube(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

pub(crate) unsafe extern "C" fn comparar_remapear_chave_de_clube(antigo: u32, novo: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_REMAPEAR_CHAVE_DE_CLUBE);
    let reg: [(usize, usize); 2] = [(0x9d00a4, 0x484), (0xb63c9c, 0x68)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(antigo, novo), || remapear_chave_de_clube(antigo, novo));
    PLACAR.registrar(igual && r == n, || format!("remapear_chave_de_clube({}, {}) original={} rust={} {}", antigo as i32, novo as i32, r, n, chaves_diferentes(&mo, &mr)));
    r
}

pub(crate) unsafe extern "C" fn comparar_casar_chave_de_nacao(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_CASAR_CHAVE_DE_NACAO);
    let reg: [(usize, usize); 1] = [(KEY_NATION_AFGHANISTAN, 0x348)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_nacao(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_nacao(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

// ---------- 227a leva: as chaves fixas das competições de clube e das competições e prêmios de pessoa pelo nome (index.cpp) ----------

/// As chaves de competição de clube do index.cpp (0x60efd0), na ordem do código: (texto do nome longo, global que recebe a linha), em 3 cadeias; (0, global) = mais uma global gravada no mesmo acerto.
pub(crate) static CHAVES_DE_COMPETICAO_DE_CLUBE: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9dc200, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION as u32), // French Lower Division
        (0x9dc1e8, KEY_CLUB_COMP_GERMAN_FIRST_DIVISION as u32), // German First Division
        (0x9dc1d0, KEY_CLUB_COMP_GERMAN_SECOND_DIVISION as u32), // German Second Division
        (0x9dc1a8, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_WEST_SOUTHWEST as u32), // German Regional Division West/Southwest
        (0x9dc188, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_EAST as u32), // German Regional Division East
        (0x9dc168, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH as u32), // German Regional Division North
        (0x9dc148, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH as u32), // German Regional Division South
        (0x9dc138, KEY_CLUB_COMP_GERMAN_REGIONAL as u32), // German Regional
        (0x9dc11c, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION as u32), // Dutch KPN-Telecompetition
        (0x9dc104, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION as u32), // Dutch KPN Eredivisie
        (0x9dc0ec, KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION as u32), // Dutch Premier Division
        (0x9dc0d8, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION as u32), // Dutch Toto Division
        (0x9dc0c0, KEY_CLUB_COMP_DUTCH_TOTO_DIVISION as u32), // Dutch First Division
        (0x9dc0b0, KEY_CLUB_COMP_ITALIAN_SERIE_A as u32), // Italian Serie A
        (0x9dc0a0, KEY_CLUB_COMP_ITALIAN_SERIE_B as u32), // Italian Serie B
        (0x9dc08c, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A as u32), // Italian Serie C1/A
        (0x9dc078, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B as u32), // Italian Serie C1/B
        (0x9dc064, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A as u32), // Italian Serie C2/A
        (0x9dc050, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B as u32), // Italian Serie C2/B
        (0x9dc03c, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C as u32), // Italian Serie C2/C
        (0x9dc02c, KEY_CLUB_COMP_ITALIAN_SERIE_D as u32), // Italian Serie D
        (0x9dc014, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as u32), // American Major League
        (0x9dc000, KEY_CLUB_COMP_AMERICAN_A_LEAGUE as u32), // American A-League
        (0x9dbff0, KEY_CLUB_COMP_AMERICAN_USISL as u32), // American USISL
        (0x9dbfd8, KEY_CLUB_COMP_AMERICAN_USISL as u32), // American D3-Pro League
        (0x9dbfc0, KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION as u32), // Belgian First Division
        (0x9dbfa8, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION as u32), // Belgian Second Division
        (0x9dbf8c, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A as u32), // Belgian Third Division A
        (0x9dbf70, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B as u32), // Belgian Third Division B
        (0x9dbf58, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION as u32), // Danish Premier Division
        (0x9dbf40, KEY_CLUB_COMP_DANISH_FIRST_DIVISION as u32), // Danish First Division
        (0x9dbf28, KEY_CLUB_COMP_DANISH_SECOND_DIVISION as u32), // Danish Second Division
        (0x9dbf0c, KEY_CLUB_COMP_DANISH_KVALIFIKATIONSRAEKKEN as u32), // Danish Kvalifikationsrækken
        (0x9dbef0, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION as u32), // English Premier Division
        (0x9dbed8, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION as u32), // English First Division
        (0x9dbec0, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION as u32), // English Second Division
        (0x9dbea8, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION as u32), // English Third Division
        (0x9dbe90, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION as u32), // French First Division
        (0x9dbe78, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION as u32), // French Second Division
        (0x9dbe68, KEY_CLUB_COMP_FRENCH_NATIONAL as u32), // French National
        (0x9dbe5c, KEY_CLUB_COMP_FRENCH_CFA as u32), // French CFA
        (0x9dbe4c, KEY_CLUB_COMP_FRENCH_CFA as u32), // French CFA - A
        (0x9dbe30, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION as u32), // Scottish Premier Division
        (0x9dbe18, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION as u32), // Scottish First Division
        (0x9dbdfc, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION as u32), // Scottish Second Division
        (0x9dbde4, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION as u32), // Scottish Third Division
        (0x9dbdc8, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION as u32), // Swedish Premier Division
        (0x9dbdb0, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION as u32), // Swedish First Division
        (0x9dbd90, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION as u32), // Swedish First Division North
        (0x9dbd68, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND as u32), // Swedish Second Division South Gotaland
        (0x9dbd48, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH as u32), // Swedish Second Division North
        (0x9dbd20, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND as u32), // Swedish Second Division East Svealand
        (0x9dbcf8, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND as u32), // Swedish Second Division East Gotaland
        (0x9dbcd0, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND as u32), // Swedish Second Division West Svealand
        (0x9dbca8, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND as u32), // Swedish Second Division West Gotaland
        (0x9dbc8c, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION as u32), // Portuguese Premier Division
        (0x9dbc70, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION as u32), // Portuguese Premier League
        (0x9dbc54, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION as u32), // Portuguese Second Division
        (0x9dbc38, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION as u32), // Portuguese Second League
        (0x9dbc14, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH as u32), // Portuguese Second Division B North
        (0x9dbbec, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL as u32), // Portuguese Second Division B Central
        (0x9dbbc8, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH as u32), // Portuguese Second Division B South
        (0x9dbbac, KEY_CLUB_COMP_PORTUGUESE_THIRD_DIVISION as u32), // Portuguese Third Division
        (0x9dbb94, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION as u32), // Spanish First Division
        (0x9dbb7c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION as u32), // Spanish Second Division
        (0x9dbb60, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1 as u32), // Spanish Second Division B1
        (0x9dbb44, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2 as u32), // Spanish Second Division B2
        (0x9dbb28, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3 as u32), // Spanish Second Division B3
        (0x9dbb0c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4 as u32), // Spanish Second Division B4
        (0x9dbae8, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES as u32), // South American Copa Libertadores
        (0x9dbacc, KEY_CLUB_COMP_SOUTH_AMERICAN_SUPER_CUP as u32), // South American Super Cup
        (0x9dbab0, KEY_CLUB_COMP_SOUTH_AMERICAN_CONMEBOL_CUP as u32), // South American CONMEBOL Cup
        (0x9dba98, KEY_CLUB_COMP_SOUTH_AMERICAN_RECOPA as u32), // South American Recopa
        (0x9dba84, KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP as u32), // Asian Champions Cup
        (0x9dba6c, KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP as u32), // Asian Club Championship
        (0x9dba54, KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP as u32), // Asian Cup Winners Cup
        (0x9dba3c, KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP as u32), // Asian Cup Winners' Cup
        (0x9dba20, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION as u32), // Argentine Premier Division
        (0x9dba04, KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION as u32), // Argentine Second Division
        (0x9db9f0, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1 as u32), // Japanese J-League 1
        (0x9db9dc, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1 as u32), // Japanese League 1
        (0x9db9c4, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP as u32), // Japanese Emperor's Cup
        (0x9db9ac, KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP as u32), // Japanese Rising Sun Cup
        (0x9db98c, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP as u32), // Japanese Yamazaki Nabisco Cup
        (0x9db97c, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP as u32), // Japanese Cup
        (0x9db96c, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP as u32), // Japanese  Cup
        (0x9db964, KEY_CLUB_COMP_JAPANESE_YAMAZAKI_NABISCO_CUP as u32), // J Cup
        (0x9db958, KEY_CLUB_COMP_FRIENDLY as u32), // Friendly
        (0x9db93c, KEY_CLUB_COMP_JAPANESE_XEROX_SUPER_CUP as u32), // Japanese Xerox Super Cup
        (0x9db928, KEY_CLUB_COMP_JAPANESE_XEROX_SUPER_CUP as u32), // Japanese Super Cup
        (0x9db910, KEY_CLUB_COMP_JAPANESE_XEROX_SUPER_CUP as u32), // Japanese Winners Cup
        (0x9db8f4, KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE as u32), // Japanese Football League
        (0x9db8e0, KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE as u32), // Japanese Non-League
        (0x9db8c4, KEY_CLUB_COMP_JAPANESE_UNIVERSITY_LEAGUE as u32), // Japanese University League
        (0x9db8a0, KEY_CLUB_COMP_JAPANESE_ADMINISTRATIVE_DIVISION as u32), // Japanese Administrative Division
        (0x9db884, KEY_CLUB_COMP_ARGENTINE_METROPOLITAN_ZONE as u32), // Argentine Metropolitan Zone
        (0x9db86c, KEY_CLUB_COMP_ARGENTINE_INTERIOR_ZONE as u32), // Argentine Interior Zone
        (0x9b5b40, KEY_CLUB_COMP_RESERVE as u32), // Reserve
        (0x9db850, KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION as u32), // Argentine Third Division
        (0x9db838, KEY_CLUB_COMP_CONCACAF_MERCONORTE_CUP as u32), // CONCACAF Merconorte Cup
        (0x9db828, KEY_CLUB_COMP_CONCACAF_MERCONORTE_CUP as u32), // Merconorte Cup
        (0x9db810, KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS as u32), // American MLS All-Stars
        (0x9db7fc, KEY_CLUB_COMP_GERMAN_LEAGUE_CUP as u32), // German League Cup
        (0x9db7dc, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_MERCOSUR as u32), // South American Copa Mercosur
        (0x9db7c8, KEY_CLUB_COMP_ENGLISH_CONFERENCE as u32), // English Conference
        (0x9db7b4, KEY_CLUB_COMP_ENGLISH_FA_TROPHY as u32), // English FA Trophy
        (0x9db79c, KEY_CLUB_COMP_ENGLISH_CONFERENCE_CUP as u32), // English Conference Cup
        (0x9db784, KEY_CLUB_COMP_FRENCH_CHAMPIONS_TROPHY as u32), // French Champions Trophy
        (0x9db76c, KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION as u32), // Swedish Lower Division
        (0x9db754, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION as u32), // Spanish Lower Division
        (0x9db73c, KEY_CLUB_COMP_PORTUGUESE_SUPER_CUP as u32), // Portuguese Super Cup
        (0x9db728, KEY_CLUB_COMP_A_PREMIER_DIVISION as u32), // A Premier Division
        (0x9db70c, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION as u32), // Norwegian Premier Division
        (0x9db6f0, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION as u32), // Norwegian First Division
        (0x9db6cc, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_1 as u32), // Norwegian Second Division Group 1
        (0x9db6a8, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_2 as u32), // Norwegian Second Division Group 2
        (0x9db684, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_3 as u32), // Norwegian Second Division Group 3
        (0x9db660, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_4 as u32), // Norwegian Second Division Group 4
        (0x9db63c, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_5 as u32), // Norwegian Second Division Group 5
        (0x9db618, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_6 as u32), // Norwegian Second Division Group 6
        (0x9db5f4, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_7 as u32), // Norwegian Second Division Group 7
        (0x9db5d0, KEY_CLUB_COMP_NORWEGIAN_SECOND_DIVISION_GROUP_8 as u32), // Norwegian Second Division Group 8
    ],
    // cadeia 2
    &[
        (0x9db5c0, KEY_CLUB_COMP_WORLD_CLUB_CUP as u32), // World Club Cup
        (0x9db5a8, KEY_CLUB_COMP_WORLD_CLUB_CUP as u32), // Intercontinental Cup
        (0x9db588, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as u32), // UEFA European Champions League
        (0x9db570, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as u32), // UEFA Champions League
        (0x9db558, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as u32), // European Champions Cup
        (0x9db53c, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP as u32), // European Cup Winners Cup
        (0x9db530, KEY_CLUB_COMP_UEFA_CUP as u32), // UEFA Cup
        (0x9db51c, KEY_CLUB_COMP_EUROPEAN_SUPER_CUP as u32), // European Super Cup
        (0x9db50c, KEY_CLUB_COMP_INTER_TOTO_CUP as u32), // Inter-toto Cup
        (0x9db4f8, KEY_CLUB_COMP_BELGIAN_SUPER_CUP as u32), // Belgian Super Cup
        (0x9db4ec, KEY_CLUB_COMP_BELGIAN_CUP as u32), // Belgian Cup
        (0x9db4d8, KEY_CLUB_COMP_BELGIAN_LEAGUE_CUP as u32), // Belgian League Cup
        (0x9db4cc, KEY_CLUB_COMP_DANISH_CUP as u32), // Danish Cup
        (0x9db4c0, KEY_CLUB_COMP_DANISH_CUP as u32), // Dong Cup
        (0x9db4b4, KEY_CLUB_COMP_FRENCH_CUP as u32), // French Cup
        (0x9db4a0, KEY_CLUB_COMP_FRENCH_LEAGUE_CUP as u32), // French League Cup
        (0x9db494, KEY_CLUB_COMP_GERMAN_CUP as u32), // German Cup
        (0x9db480, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP as u32), // Dutch Amstel Cup
        (0x9db474, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP as u32), // Dutch Cup
        (0x9db468, KEY_CLUB_COMP_ITALIAN_CUP as u32), // Italian Cup
        (0x9db454, KEY_CLUB_COMP_ITALIAN_CUP as u32), // Italian T.I.M. Cup
        (0x9db440, KEY_CLUB_COMP_ITALIAN_SERIE_C_CUP as u32), // Italian Serie C Cup
        (0x9db42c, KEY_CLUB_COMP_ITALIAN_SUPER_CUP as u32), // Italian Super Cup
        (0x9db420, KEY_CLUB_COMP_US_OPEN_CUP as u32), // US Open Cup
        (0x9db408, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP as u32), // CONCACAF Champions Cup
        (0x9db3ec, KEY_CLUB_COMP_CONCACAF_CUP_WINNERS_CUP as u32), // CONCACAF Cup Winners Cup
        (0x9db3dc, KEY_CLUB_COMP_NORWEGIAN_CUP as u32), // Norwegian Cup
        (0x9db3c0, KEY_CLUB_COMP_NORWEGIAN_THIRD_DIVISION as u32), // Norwegian Third Division
        (0x9db3b0, KEY_CLUB_COMP_PORTUGUESE_CUP as u32), // Portuguese Cup
        (0x9db3a4, KEY_CLUB_COMP_SPANISH_CUP as u32), // Spanish Cup
        (0x9db390, KEY_CLUB_COMP_SPANISH_SUPER_CUP as u32), // Spanish Super Cup
        (0x9db384, KEY_CLUB_COMP_SWEDISH_CUP as u32), // Swedish Cup
        (0x9db374, KEY_CLUB_COMP_ENGLISH_FA_CUP as u32), // English FA Cup
        (0x9db360, KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP as u32), // English League Cup
        (0x9db348, KEY_CLUB_COMP_ENGLISH_CHARITY_SHIELD as u32), // English Charity Shield
        (0x9db32c, KEY_CLUB_COMP_ENGLISH_AUTOGLASS_TROPHY as u32), // English Autoglass Trophy
        (0x9db318, KEY_CLUB_COMP_ENGLISH_AUTOGLASS_TROPHY as u32), // English Vans Trophy
        (0x9db308, KEY_CLUB_COMP_SCOTTISH_CUP as u32), // Scottish Cup
        (0x9db2f4, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CUP as u32), // Scottish League Cup
        (0x9db2e0, KEY_CLUB_COMP_A_LOWER_DIVISION as u32), // A Lower Division
        (0x9db2cc, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2 as u32), // Japanese J-League 2
        (0x9db2b8, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2 as u32), // Japanese League 2
        (0x9db290, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as u32), // World Cup Oceania Qualifying Section
        (0x9db268, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION as u32), // World Cup CONCACAF Qualifying Section
        (0x9db244, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION as u32), // World Cup Asian Qualifying Section
        (0x9db218, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT as u32), // World Cup South American Qualifying Section
        (0x9db1f0, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION as u32), // World Cup African Qualifying Section
        (0x9db1c8, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION as u32), // World Cup European Qualifying Section
        (0x9db1a4, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING as u32), // European Championship Qualifying
        (0x9db18c, KEY_CLUB_COMP_ASIAN_CUP_QUALIFYING as u32), // Asian Cup Qualifying
        (0x9db164, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES as u32), // Football Tournament of the Asian Games
        (0x9db144, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES as u32), // Asian Games Football Tournament
        (0x9db138, KEY_CLUB_COMP_ASIAN_CUP as u32), // Asian Cup
        (0x9db124, KEY_CLUB_COMP_B_INTERNATIONAL as u32), // 'B' International
        (0x9db10c, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL as u32), // Under 21 International
        (0x9db0fc, KEY_CLUB_COMP_FIFA_WORLD_CUP as u32), // FIFA World Cup
        (0x9db0d0, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES as u32), // Football Tournament of the Olympic Games
        (0x9db0c0, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES as u32), // Olympic Games
        (0x9db0a0, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP as u32), // European Football Championship
        (0x9db090, KEY_CLUB_COMP_COPA_AMERICA as u32), // Copa America
        (0x9db078, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS as u32), // African Cup of Nations
        (0x9db068, KEY_CLUB_COMP_DUTCH_SUPER_CUP as u32), // Dutch Super Cup
        (0x9db04c, KEY_CLUB_COMP_DUTCH_SUPER_CUP as u32), // Dutch Johan Cruyff Schaal
        (0x9db030, KEY_CLUB_COMP_DUTCH_SUPER_CUP as u32), // Dutch Johan Cruijff Schaal
        (0x9db010, KEY_CLUB_COMP_SCOTTISH_LEAGUE_CHALLENGE_CUP as u32), // Scottish League Challenge Cup
        (0x9daff4, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B as u32), // Spanish Second Division B
        (0x9dafd4, KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP as u32), // FIFA Club World Championship
        (0x9dafb4, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B as u32), // Portuguese Second Division B
        (0x9dbc14, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B as u32), // Portuguese Second Division B North
        (0x9daf9c, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION as u32), // Belgian Third Division
        (0x9dbf8c, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION as u32), // Belgian Third Division A
        (0x9daf84, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION as u32), // Swedish Second Division
        (0x9dbd48, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION as u32), // Swedish Second Division North
        (0x9daf68, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP as u32), // Oceania Club Championship
        (0x9daf50, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP as u32), // Oceania Champions Cup
        (0x9daf40, KEY_CLUB_COMP_ASIAN_SUPER_CUP as u32), // Asian Super Cup
        (0x9daf24, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A as u32), // Belgian Fourth Division  A
        (0x9daf08, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B as u32), // Belgian Fourth Division  B
        (0x9daeec, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_C as u32), // Belgian Fourth Division  C
        (0x9daed0, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_D as u32), // Belgian Fourth Division  D
        (0x9daec4, KEY_CLUB_COMP_FINNISH_CUP as u32), // Finnish Cup
        (0x9daea8, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION as u32), // Finnish Premier Division
        (0x9dae88, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH as u32), // Finnish First Division North
        (0x9dae68, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH as u32), // Finnish First Division South
        (0x9dae50, KEY_CLUB_COMP_FINNISH_LOWER_DIVISION as u32), // Finnish Lower Division
        (0x9dae38, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION as u32), // Finnish First Division
        (0x9dae18, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH as u32), // Finnish Second Division North
        (0x9dadf8, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_WEST as u32), // Finnish Second Division West
        (0x9dadd8, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_SOUTH as u32), // Finnish Second Division South
        (0x9dadb8, KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_EAST as u32), // Finnish Second Division East
        (0x9dada0, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION as u32), // Irish Premier Division
        (0x9dad88, KEY_CLUB_COMP_IRISH_FIRST_DIVISION as u32), // Irish First Division
        (0x9dad78, KEY_CLUB_COMP_IRISH_GROUP_A as u32), // Irish Group A
        (0x9dad68, KEY_CLUB_COMP_IRISH_GROUP_B as u32), // Irish Group B
        (0x9dad58, KEY_CLUB_COMP_IRISH_GROUP_C as u32), // Irish Group C
        (0x9dad48, KEY_CLUB_COMP_IRISH_GROUP_D as u32), // Irish Group D
        (0x9dad38, KEY_CLUB_COMP_IRISH_GROUP_E as u32), // Irish Group E
        (0x9dad20, KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_A as u32), // Northern Irish Group A
        (0x9dad08, KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_B as u32), // Northern Irish Group B
        (0x9dacf0, KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_C as u32), // Northern Irish Group C
        (0x9dacd8, KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_D as u32), // Northern Irish Group D
        (0x9dacc4, KEY_CLUB_COMP_IRISH_LEAGUE_CUP as u32), // Irish League Cup
        (0x9daca8, KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP as u32), // Irish Senior Challenge Cup
        (0x9dac98, KEY_CLUB_COMP_IRISH_SUPER_CUP as u32), // Irish Super Cup
        (0x9dac78, KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE as u32), // Irish Connacht Senior League
        (0x9dac5c, KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE as u32), // Irish Ulster Senior League
        (0x9dac40, KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE as u32), // Irish Munster Senior League
        (0x9dac20, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE as u32), // Irish Leinster Senior League
        (0x9dabf8, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE as u32), // Irish Leinster Senior League Premier
        (0x9dabe0, KEY_CLUB_COMP_POLISH_FIRST_DIVISION as u32), // Polish First Division
        (0x9dabc8, KEY_CLUB_COMP_POLISH_SECOND_DIVISION as u32), // Polish Second Division
        (0x9dabb4, KEY_CLUB_COMP_POLISH_LEAGUE_CUP as u32), // Polish League Cup
        (0x9daba4, KEY_CLUB_COMP_POLISH_FA_CUP as u32), // Polish FA Cup
        (0x9dab8c, KEY_CLUB_COMP_POLISH_LOWER_DIVISION as u32), // Polish Lower Division
        (0x9dab74, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION as u32), // greek premier division
        (0x9dab58, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION as u32), // Greek National A Division
        (0x9dab40, KEY_CLUB_COMP_GREEK_SECOND_DIVISION as u32), // greek second division
        (0x9dab24, KEY_CLUB_COMP_GREEK_SECOND_DIVISION as u32), // Greek National B Division
        (0x9dab0c, KEY_CLUB_COMP_GREEK_LOWER_DIVISION as u32), // greek lower division
        (0x9dab00, KEY_CLUB_COMP_GREEK_CUP as u32), // greek cup
        (0x9daae8, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF87C as u32), // croatian first division
        (0, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4 as u32), // (também grava, mesmo acerto)
        (0x9daad0, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF87C as u32), // croatian a1 division
        (0, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4 as u32), // (também grava, mesmo acerto)
        (0x9daab4, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION as u32), // croatian second division
        (0x9daa94, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_CENTRAL as u32), // croatian third division central
        (0x9daa74, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_NORTH as u32), // croatian third division north
        (0x9daa54, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_SOUTH as u32), // croatian third division south
        (0x9daa34, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_EAST as u32), // croatian third division east
        (0x9daa14, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_WEST as u32), // croatian third division west
        (0x9daa04, KEY_CLUB_COMP_CROATIAN_CUP as u32), // croatian cup
        (0x9da9e0, KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE as u32), // australian national soccer league
        (0x9da9cc, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP as u32), // oceania nations cup
        (0x9da9ac, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH as u32), // croatian second division north
        (0x9da990, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH as u32), // croatian second division a
        (0x9da978, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH as u32), // croatian a2/a division
        (0x9da958, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH as u32), // croatian second division south
        (0x9da93c, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH as u32), // croatian second division b
        (0x9da924, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH as u32), // croatian a2/b division
        (0x9da900, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION as u32), // northern ireland premier division
        (0x9da8e0, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION as u32), // irish league premier division
        (0x9da8b8, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION as u32), // northern irish league premier division
        (0x9da898, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION as u32), // northern ireland first division
        (0x9da87c, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION as u32), // irish league first division
        (0x9da854, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION as u32), // northern irish league first division
        (0x9da834, KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION as u32), // northern ireland lower division
        (0x9da818, KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION as u32), // irish league lower division
        (0x9da7f0, KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION as u32), // northern irish league lower division
        (0x9da7d0, KEY_CLUB_COMP_NORTHERN_IRISH_CHARITY_SHIELD as u32), // northern irish charity shield
        (0x9da7bc, KEY_CLUB_COMP_NORTHERN_IRISH_CUP as u32), // northern irish cup
        (0x9da7b0, KEY_CLUB_COMP_NORTHERN_IRISH_CUP as u32), // irish cup
        (0x9da798, KEY_CLUB_COMP_NORTHERN_IRISH_GOLD_CUP as u32), // northern irish gold cup
        (0x9da77c, KEY_CLUB_COMP_NORTHERN_IRISH_LEAGUE_CUP as u32), // northern irish league cup
    ],
    // cadeia 3
    &[
        (0x9da764, KEY_CLUB_COMP_TURKISH_LOWER_DIVISION as u32), // turkish lower division
        (0x9da748, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION as u32), // turkish premier division
        (0x9da728, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A as u32), // turkish 2. division category a
        (0x9da708, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B as u32), // turkish 2. division category b
        (0x9da6e4, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1 as u32), // turkish 2. division category b g1
        (0x9da6c0, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2 as u32), // turkish 2. division category b g2
        (0x9da69c, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3 as u32), // turkish 2. division category b g3
        (0x9da678, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4 as u32), // turkish 2. division category b g4
        (0x9da654, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5 as u32), // turkish 2. division category b g5
        (0x9da644, KEY_CLUB_COMP_TURKISH_FA_CUP as u32), // turkish fa cup
        (0x9da618, KEY_CLUB_COMP_TURKISH_FOOTBALL_WRITERS_ASSOCIATION_CUP as u32), // turkish football writers' association cup
        (0x9da5f0, KEY_CLUB_COMP_TURKISH_FOOTBALL_WRITERS_ASSOCIATION_CUP as u32), // turkish sports writers' association cup
        (0x9da5d4, KEY_CLUB_COMP_TURKISH_2_DIVISION_GROUP_1 as u32), // turkish 2. division group 1
        (0x9da5b8, KEY_CLUB_COMP_TURKISH_2_DIVISION_GROUP_2 as u32), // turkish 2. division group 2
        (0x9da59c, KEY_CLUB_COMP_TURKISH_2_DIVISION_GROUP_3 as u32), // turkish 2. division group 3
        (0x9da580, KEY_CLUB_COMP_TURKISH_2_DIVISION_GROUP_4 as u32), // turkish 2. division group 4
        (0x9da564, KEY_CLUB_COMP_TURKISH_2_DIVISION_GROUP_5 as u32), // turkish 2. division group 5
        (0x9da544, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_WEST as u32), // russian second division - west
        (0x9da520, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_CENTRAL as u32), // russian second division - central
        (0x9da500, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH as u32), // russian second division - south
        (0x9da4e0, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_VOLGA as u32), // russian second division - volga
        (0x9da4c0, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_URAL as u32), // russian second division - ural
        (0x9da4a0, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_EAST as u32), // russian second division - east
        (0x9da488, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION as u32), // russian first division
        (0x9da46c, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION as u32), // russian premier division
        (0x9da460, KEY_CLUB_COMP_RUSSIAN_CUP as u32), // russian cup
        (0x9da448, KEY_CLUB_COMP_ITALIAN_C1_SUPER_CUP as u32), // italian c1 super cup
        (0x9da438, KEY_CLUB_COMP_GREEK_SUPER_CUP as u32), // greek super cup
        (0x9da424, KEY_CLUB_COMP_CROATIAN_SUPER_CUP as u32), // croatian super cup
        (0x9da410, KEY_CLUB_COMP_POLISH_SUPER_CUP as u32), // polish super cup
        (0x9da3f8, KEY_CLUB_COMP_IRISH_PRESIDENTS_CUP as u32), // irish presidents cup
        (0x9da3e0, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION as u32), // welsh premier division
        (0x9da3cc, KEY_CLUB_COMP_WELSH_LEAGUE_CUP as u32), // welsh league cup
        (0x9da3c0, KEY_CLUB_COMP_WELSH_CUP as u32), // welsh cup
        (0x9da3ac, KEY_CLUB_COMP_WELSH_PREMIER_CUP as u32), // welsh premier cup
        (0x9da398, KEY_CLUB_COMP_INTER_AMERICAN_CUP as u32), // inter-american cup
        (0x9da380, KEY_CLUB_COMP_WELSH_LOWER_DIVISION as u32), // welsh lower division
        (0x9da36c, KEY_CLUB_COMP_CONFEDERATIONS_CUP as u32), // confederations cup
        (0x9da34c, KEY_CLUB_COMP_BRAZILIAN_BAHIA_LOWER_DIVISION as u32), // Brazilian Bahia Lower Division
        (0x9da328, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_LOWER_DIVISION as u32), // Brazilian Central Lower Division
        (0x9da300, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_LOWER_DIVISION as u32), // Brazilian Santa Catarina Lower Division
        (0x9da2dc, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_LOWER_DIVISION as u32), // Brazilian Pernambuco Lower Division
        (0x9da2bc, KEY_CLUB_COMP_BRAZILIAN_NORTH_LOWER_DIVISION as u32), // Brazilian North Lower Division
        (0x9da294, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_LOWER_DIVISION as u32), // Brazilian Rio de Janeiro Lower Division
        (0x9da270, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_LOWER_DIVISION as u32), // Brazilian Northeast Lower Division
        (0x9da250, KEY_CLUB_COMP_BRAZILIAN_GOIAS_LOWER_DIVISION as u32), // Brazilian Goiás Lower Division
        (0x9da228, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_LOWER_DIVISION as u32), // Brazilian Minas Gerais Lower Division
        (0x9da208, KEY_CLUB_COMP_BRAZILIAN_PARANA_LOWER_DIVISION as u32), // Brazilian Paraná Lower Division
        (0x9da1e8, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_LOWER_DIVISION as u32), // Brazilian Gaúcho Lower Division
        (0x9da1c4, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP as u32), // Brazilian North State Championship
        (0x9da19c, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP as u32), // Brazilian Central State Championship
        (0x9da178, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_LOWER_DIVISION as u32), // Brazilian São Paulo Lower Division
        (0x9da158, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_REGIONAL_CUP as u32), // Brazilian Central Regional Cup
        (0x9da130, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP as u32), // Brazilian Northeast State Championship
        (0x9da110, KEY_CLUB_COMP_BRAZILIAN_NORTH_REGIONAL_CUP as u32), // Brazilian North Regional Cup
        (0x9da0ec, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION as u32), // Brazilian National Third Division
        (0x9da0c0, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS as u32), // Brazilian Santa Catarina State Championship
        (0x9da09c, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP as u32), // Brazilian Goiás State Championship
        (0x9da084, KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP as u32), // Brazilian Champions Cup
        (0x9da05c, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP as u32), // Brazilian Pernambuco State Championship
        (0x9da038, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP as u32), // Brazilian Bahia State Championship
        (0x9da014, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_REGIONAL_CUP as u32), // Brazilian Northeast Regional Cup
        (0x9d9ff0, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP as u32), // Brazilian Gaúcho State Championship
        (0x9d9fcc, KEY_CLUB_COMP_BRAZILIAN_SOUTH_MINAS_REGIONAL_CUP as u32), // Brazilian South-Minas Regional Cup
        (0x9d9fa8, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP as u32), // Brazilian Paraná State Championship
        (0x9d9f7c, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI as u32), // Brazilian Minas Gerais State Championship
        (0x9d9f58, KEY_CLUB_COMP_BRAZILIAN_RIO_SAO_PAULO_TOURNEMENT as u32), // Brazilian Rio-São Paulo Tournement
        (0x9d9f2c, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS as u32), // Brazilian Rio de Janeiro State Championship
        (0x9d9f08, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION as u32), // Brazilian National Second Division
        (0x9d9ee0, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP as u32), // Brazilian São Paulo State Championship
        (0x9d9ed0, KEY_CLUB_COMP_BRAZILIAN_CUP as u32), // Brazilian Cup
        (0x9d9eac, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION as u32), // Brazilian National First Division
        (0x9d9e90, KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_CUP as u32), // Irish Munster Senior Cup
        (0x9d9e74, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_CUP as u32), // Irish Leinster Senior Cup
        (0x9d9e60, KEY_CLUB_COMP_KOREAN_SUPER_CUP as u32), // Korean Super Cup
        (0x9d9e4c, KEY_CLUB_COMP_KOREAN_LEAGUE_CUP as u32), // Korean League Cup
        (0x9ce524, KEY_CLUB_COMP_KOREAN_LEAGUE as u32), // Korean League
        (0x9d9e3c, KEY_CLUB_COMP_KOREAN_FA_CUP as u32), // Korean FA Cup
    ],
];

/// As chaves de competição e prêmio de pessoa do index.cpp (0x6115e0), na ordem do código: (texto do nome, global que recebe a linha), em 5 cadeias.
pub(crate) static CHAVES_DE_COMPETICAO_DE_PESSOA: &[&[(u32, u32)]] = &[
    // cadeia 1
    &[
        (0x9e05ec, KEY_STAFF_COMP_FIFA_WORLD_PLAYER_OF_THE_YEAR as u32), // FIFA World Player of the Year
        (0x9e05cc, KEY_STAFF_COMP_WORLD_FOOTBALLER_OF_THE_YEAR as u32), // World Footballer of the Year
        (0x9e05ac, KEY_STAFF_COMP_EUROPEAN_FOOTBALLER_OF_THE_YEAR as u32), // European Footballer of the Year
        (0x9e0584, KEY_STAFF_COMP_SOUTH_AMERICAN_FOOTBALLER_OF_THE_YEAR as u32), // South American Footballer of the Year
        (0x9e0564, KEY_STAFF_COMP_AFRICAN_FOOTBALLER_OF_THE_YEAR as u32), // African Footballer of the Year
        (0x9e0540, KEY_STAFF_COMP_ENGLISH_PLAYERS_PLAYER_OF_THE_YEAR as u32), // English Players Player of the Year
        (0x9e0514, KEY_STAFF_COMP_ENGLISH_PLAYERS_YOUNG_PLAYER_OF_THE_YEAR as u32), // English Players Young Player of the Year
        (0x9e04ec, KEY_STAFF_COMP_ENGLISH_PLAYERS_PREMIER_DIVISION_SELECT as u32), // English Players Premier Division Select
        (0x9e04c4, KEY_STAFF_COMP_ENGLISH_PLAYERS_FIRST_DIVISION_SELECT as u32), // English Players First Division Select
        (0x9e049c, KEY_STAFF_COMP_ENGLISH_PLAYERS_SECOND_DIVISION_SELECT as u32), // English Players Second Division Select
        (0x9e0474, KEY_STAFF_COMP_ENGLISH_PLAYERS_THIRD_DIVISION_SELECT as u32), // English Players Third Division Select
        (0x9e0444, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_PLAYER_OF_THE_M as u32), // English Premier Division Player of the Month
        (0x9e0418, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_PLAYER_OF_THE_MON as u32), // English First Division Player of the Month
        (0x9e03ec, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_PLAYER_OF_THE_MO as u32), // English Second Division Player of the Month
        (0x9e03c0, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_PLAYER_OF_THE_MON as u32), // English Third Division Player of the Month
        (0x9e038c, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_YOUNG_PLAYER_OF as u32), // English Premier Division Young Player of the Month
        (0x9e0358, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_YOUNG_PLAYER_OF_T as u32), // English First Division Young Player of the Month
        (0x9e0324, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_YOUNG_PLAYER_OF as u32), // English Second Division Young Player of the Month
        (0x9e02f0, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_YOUNG_PLAYER_OF_T as u32), // English Third Division Young Player of the Month
        (0x9e02c4, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // English Premier Division Team of the Week
        (0x9e029c, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // English First Division Team of the Week
        (0x9e0270, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // English Second Division Team of the Week
        (0x9e0248, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_TEAM_OF_THE_WEEK as u32), // English Third Division Team of the Week
        (0x9e022c, KEY_STAFF_COMP_SCOTTISH_PLAYER_OF_THE_YEAR as u32), // Scottish Player of the Year
        (0x9e0208, KEY_STAFF_COMP_SCOTTISH_YOUNG_PLAYER_OF_THE_YEAR as u32), // Scottish Young Player of the Year
        (0x9e01d8, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_PLAYER_OF_THE as u32), // Scottish Premier Division Player of the Month
        (0x9e01ac, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_PLAYER_OF_THE_MO as u32), // Scottish First Division Player of the Month
        (0x9e017c, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_PLAYER_OF_THE_M as u32), // Scottish Second Division Player of the Month
        (0x9e0150, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_PLAYER_OF_THE_MO as u32), // Scottish Third Division Player of the Month
        (0x9e0124, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_TEAM_OF_THE_WE as u32), // Scottish Premier Division Team of the Week
        (0x9e00f8, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Scottish First Division Team of the Week
        (0x9e00cc, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_TEAM_OF_THE_WEE as u32), // Scottish Second Division Team of the Week
        (0x9e00a0, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_TEAM_OF_THE_WEEK as u32), // Scottish Third Division Team of the Week
        (0x9e008c, KEY_STAFF_COMP_BELGIAN_GOLDEN_BOOT as u32), // Belgian Golden Boot
        (0x9e0060, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_PLAYER_OF_THE_YEA as u32), // Belgian First Division Player of the Year
        (0x9e0030, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_GOALKEEPER_OF_THE as u32), // Belgian First Division Goalkeeper of the Year
        (0x9e0004, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_PLAYER_OF_THE_YE as u32), // Belgian Second Division Player of the Year
        (0x9dffe8, KEY_STAFF_COMP_DANISH_PLAYER_OF_THE_YEAR as u32), // Danish Player of the Year
        (0x9dffbc, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_PLAYER_OF_THE_YE as u32), // Danish Premier Division Player of the Year
        (0x9dff88, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_YOUNG_PLAYER_OF as u32), // Danish Premier Division Young Player of the Year
        (0x9dff5c, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR as u32), // Danish First Division Player of the Year
        (0x9dff2c, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_YOUNG_PLAYER_OF_TH as u32), // Danish First Division Young Player of the Year
        (0x9dfefc, KEY_STAFF_COMP_DANISH_KVALIFIK_PLAYER_OF_THE_YEAR as u32), // Danish Kvalifikationsrækken Player of the Year
        (0x9dfed0, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_PLAYER_OF_THE_YEA as u32), // Danish Second Division Player of the Year
        (0x9dfe9c, KEY_STAFF_COMP_DANISH_KVALIFIK_YOUNG_PLAYER_OF_YR as u32), // Danish Kvalifikationsrækken Young Player of Year
        (0x9dfe70, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_YOUNG_PLAYER_OF_Y as u32), // Danish Second Division Young Player of Year
        (0x9dfe54, KEY_STAFF_COMP_FRENCH_PLAYER_OF_THE_YEAR as u32), // French Player of the Year
        (0x9dfe30, KEY_STAFF_COMP_FRENCH_FOREIGN_PLAYER_OF_THE_YEAR as u32), // French Foreign Player of the Year
        (0x9dfe0c, KEY_STAFF_COMP_FRENCH_UNDER_23_PLAYER_OF_THE_YEAR as u32), // French Under-23 Player of the Year
        (0x9dfde4, KEY_STAFF_COMP_FRENCH_MOST_POPULAR_PLAYER_OF_THE_YEAR as u32), // French Most Popular Player of the Year
        (0x9dfdb8, KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR as u32), // French Most Unpopular Player of the Year
        (0x9dfd84, KEY_STAFF_COMP_FRENCH_PLAYERS_FIRST_DIVISION_PLAYER_OF as u32), // French Players First Division Player of the Year
        (0x9dfd58, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR as u32), // French First Division Player of the Year
        (0x9dfd28, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_GOALKEEPER_OF_THE as u32), // French First Division Goalkeeper of the Year
        (0x9dfcf4, KEY_STAFF_COMP_FRENCH_PLAYERS_SECOND_DIVISION_PLAYER_OF as u32), // French Players Second Division Player of the Year
        (0x9dfcc8, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_PLAYER_OF_THE_YEA as u32), // French Second Division Player of the Year
        (0x9dfc98, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_GOALKEEPER_OF_THE as u32), // French Second Division Goalkeeper of the Year
        (0x9dfc7c, KEY_STAFF_COMP_GERMAN_PLAYER_OF_THE_YEAR as u32), // German Player of the Year
        (0x9dfc50, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_PLAYER_OF_THE_MONT as u32), // German First Division Player of the Month
        (0x9dfc24, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_PLAYER_OF_THE_MON as u32), // German Second Division Player of the Month
        (0x9dfc04, KEY_STAFF_COMP_DUTCH_GOALKEEPER_OF_THE_YEAR as u32), // Dutch Goalkeeper of the Year
        (0x9dfbe4, KEY_STAFF_COMP_DUTCH_YOUNG_PLAYER_OF_THE_YEAR as u32), // Dutch Young Player of the Year
        (0x9dfbb8, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_PLAYER_OF_THE_YEA as u32), // Dutch Premier Division Player of the Year
        (0x9dfb90, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_TOP_GOALSCORER as u32), // Dutch Premier Division Top Goalscorer
        (0x9dfb6c, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_GOLDEN_BOOT as u32), // Dutch Premier Division Golden Boot
        (0x9dfb44, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR as u32), // Dutch First Division Player of the Year
        (0x9dfb20, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_TOP_GOALSCORER as u32), // Dutch First Division Top Goalscorer
        (0x9dfafc, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_GOLDEN_BOOT as u32), // Dutch First Division Golden Boot
        (0x9dfad0, KEY_STAFF_COMP_ITALIAN_SERIE_A_YOUNG_PLAYER_OF_THE_YEAR as u32), // Italian Serie A Young Player of the Year
        (0x9dfaac, KEY_STAFF_COMP_ITALIAN_SERIE_A_PLAYER_OF_THE_YEAR as u32), // Italian Serie A Player of the Year
        (0x9dfa80, KEY_STAFF_COMP_ITALIAN_SERIE_A_ITALIAN_PLAYER_OF_THE_YE as u32), // Italian Serie A Italian Player of the Year
        (0x9dfa54, KEY_STAFF_COMP_ITALIAN_SERIE_A_FOREIGN_PLAYER_OF_THE_YE as u32), // Italian Serie A Foreign Player of the Year
        (0x9dfa2c, KEY_STAFF_COMP_ITALIAN_SERIE_A_DEFENDER_OF_THE_YEAR as u32), // Italian Serie A Defender of the Year
        (0x9dfa04, KEY_STAFF_COMP_ITALIAN_SERIE_A_GOALKEEPER_OF_THE_YEAR as u32), // Italian Serie A Goalkeeper of the Year
        (0x9df9e0, KEY_STAFF_COMP_ITALIAN_SERIE_B_PLAYER_OF_THE_YEAR as u32), // Italian Serie B Player of the Year
        (0x9df9b8, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_PLAYER_OF_THE_YEAR as u32), // Italian Serie C1/A Player of the Year
        (0x9df990, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_PLAYER_OF_THE_YEAR as u32), // Italian Serie C1/B Player of the Year
        (0x9df970, KEY_STAFF_COMP_NORWEGIAN_PLAYER_OF_THE_YEAR as u32), // Norwegian Player of the Year
        (0x9df94c, KEY_STAFF_COMP_NORWEGIAN_GOALKEEPER_OF_THE_YEAR as u32), // Norwegian Goalkeeper of the Year
        (0x9df92c, KEY_STAFF_COMP_NORWEGIAN_DEFENDER_OF_THE_YEAR as u32), // Norwegian Defender of the Year
        (0x9df908, KEY_STAFF_COMP_NORWEGIAN_MIDFIELDER_OF_THE_YEAR as u32), // Norwegian Midfielder of the Year
        (0x9df8e8, KEY_STAFF_COMP_NORWEGIAN_ATTACKER_OF_THE_YEAR as u32), // Norwegian Attacker of the Year
        (0x9df8bc, KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_TEAM_OF_THE_W as u32), // Norwegian Premier Division Team of the Week
        (0x9df890, KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_TEAM_OF_THE_WEE as u32), // Norwegian First Division Team of the Week
        (0x9df85c, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_MOST_MAN_OF as u32), // Portuguese Premier Division Most Man of the Match
        (0x9df82c, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_MOST_MAN_OF as u32), // Portuguese Premier League Most Man of the Match
        (0x9df800, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TOP_GOALSCOR as u32), // Portuguese Premier Division Top Goalscorer
        (0x9df7d4, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TOP_GOALSCOR as u32), // Portuguese Premier League Top Goalscorer
        (0x9df7a4, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_PLAYER_OF_THE as u32), // Portuguese Second Division Player of the Year
        (0x9df778, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_PLAYER_OF_THE as u32), // Portuguese Second League Player of the Year
        (0x9df748, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_GOALKEEPER_OF_THE as u32), // Spanish First Division Goalkeeper of the Year
        (0x9df72c, KEY_STAFF_COMP_SPANISH_PLAYER_OF_THE_YEAR as u32), // Spanish Player of the Year
        (0x9df704, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_TOP_GOALSCORER as u32), // Spanish First Division Top Goalscorer
        (0x9df6e8, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR as u32), // Swedish Player of the Year
        (0x9df6c0, KEY_STAFF_COMP_SWEDISH_PREMIER_DIVISION_TOP_GOALSCORER as u32), // Swedish Premier Division Top Goalscorer
        (0x9df6a0, KEY_STAFF_COMP_SWEDISH_FANS_PLAYER_OF_THE_YEAR as u32), // Swedish Fans Player of the Year
        (0x9df684, KEY_STAFF_COMP_MLS_GOALKEEPER_OF_THE_YEAR as u32), // MLS Goalkeeper of the Year
        (0x9df668, KEY_STAFF_COMP_MLS_DEFENDER_OF_THE_YEAR as u32), // MLS Defender of the Year
        (0x9df650, KEY_STAFF_COMP_MLS_ROOKIE_OF_THE_YEAR as u32), // MLS Rookie of the Year
        (0x9df638, KEY_STAFF_COMP_MLS_SCORING_CHAMPION as u32), // MLS Scoring Champion
        (0x9df624, KEY_STAFF_COMP_MLS_ALL_PRO_TEAM as u32), // MLS All-Pro Team
        (0x9df60c, KEY_STAFF_COMP_MLS_PLAYER_OF_THE_MONTH as u32), // MLS Player of the Month
        (0x9df5e0, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_TEAM_OF_THE_YE as u32), // Scottish Premier Division Team of the Year
        (0x9df5b4, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // Scottish First Division Team of the Year
        (0x9df588, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_TEAM_OF_THE_YEA as u32), // Scottish Second Division Team of the Year
        (0x9df55c, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_TEAM_OF_THE_YEAR as u32), // Scottish Third Division Team of the Year
        (0x9df544, KEY_STAFF_COMP_SCOTTISH_TOP_GOALSCORER as u32), // Scottish Top Goalscorer
        (0x9df514, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_YOUNG_PLAYER_O as u32), // Scottish Premier Division Young Player of Month
        (0x9df4e4, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_YOUNG_PLAYER_OF as u32), // Scottish First Division Young Player of Month
        (0x9df4b4, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_YOUNG_PLAYER_OF as u32), // Scottish Second Division Young Player of Month
        (0x9df484, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_YOUNG_PLAYER_OF as u32), // Scottish Third Division Young Player of Month
        (0x9df468, KEY_STAFF_COMP_BELGIAN_MANAGER_OF_THE_YEAR as u32), // Belgian Manager of the Year
        (0x9df438, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFBD4 as u32), // English Premier Division Manager of the Year
        (0x9df408, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFBD8 as u32), // English Premier Division Manager of the Month
        (0x9df3dc, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_MANAGER_OF_THE_YE as u32), // English First Division Manager of the Year
        (0x9df3b0, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_MANAGER_OF_THE_MO as u32), // English First Division Manager of the Month
        (0x9df384, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_MANAGER_OF_THE_Y as u32), // English Second Division Manager of the Year
        (0x9df354, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_MANAGER_OF_THE_M as u32), // English Second Division Manager of the Month
        (0x9df328, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_MANAGER_OF_THE_YE as u32), // English Third Division Manager of the Year
        (0x9df2fc, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_MANAGER_OF_THE_MO as u32), // English Third Division Manager of the Month
        (0x9df2e0, KEY_STAFF_COMP_FRENCH_MANAGER_OF_THE_YEAR as u32), // French Manager of the Year
        (0x9df2b4, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_MANAGER_OF_THE_YEA as u32), // German First Division Manager of the Year
        (0x9df288, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_MANAGER_OF_THE_YE as u32), // German Second Division Manager of the Year
        (0x9df26c, KEY_STAFF_COMP_DUTCH_MANAGER_OF_THE_YEAR as u32), // Dutch Manager of the Year
        (0x9df248, KEY_STAFF_COMP_ITALIAN_SERIE_A_MANAGER_OF_THE_YEAR as u32), // Italian Serie A Manager of the Year
    ],
    // cadeia 2
    &[
        (0x9df224, KEY_STAFF_COMP_ITALIAN_SERIE_B_MANAGER_OF_THE_YEAR as u32), // Italian Serie B Manager of the Year
        (0x9df1fc, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_MANAGER_OF_THE_YEAR as u32), // Italian Serie C1/A Manager of the Year
        (0x9df1d4, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_MANAGER_OF_THE_YEAR as u32), // Italian Serie C1/B Manager of the Year
        (0x9df1a4, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFC14 as u32), // Scottish Premier Division Manager of the Year
        (0x9df174, KEY_STAFF_COMP_SCOTTISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFC18 as u32), // Scottish Premier Division Manager of the Month
        (0x9df148, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_MANAGER_OF_THE_Y as u32), // Scottish First Division Manager of the Year
        (0x9df118, KEY_STAFF_COMP_SCOTTISH_FIRST_DIVISION_MANAGER_OF_THE_M as u32), // Scottish First Division Manager of the Month
        (0x9df0e8, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_MANAGER_OF_THE_9CFC24 as u32), // Scottish Second Division Manager of the Year
        (0x9df0b8, KEY_STAFF_COMP_SCOTTISH_SECOND_DIVISION_MANAGER_OF_THE_9CFC28 as u32), // Scottish Second Division Manager of the Month
        (0x9df08c, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_MANAGER_OF_THE_Y as u32), // Scottish Third Division Manager of the Year
        (0x9df05c, KEY_STAFF_COMP_SCOTTISH_THIRD_DIVISION_MANAGER_OF_THE_M as u32), // Scottish Third Division Manager of the Month
        (0x9df040, KEY_STAFF_COMP_SPANISH_MANAGER_OF_THE_YEAR as u32), // Spanish Manager of the Year
        (0x9df024, KEY_STAFF_COMP_SWEDISH_MANAGER_OF_THE_YEAR as u32), // Swedish Manager of the Year
        (0x9df00c, KEY_STAFF_COMP_MLS_MANAGER_OF_THE_YEAR as u32), // MLS Manager of the Year
        (0x9defe8, KEY_STAFF_COMP_BELGIAN_YOUNG_PLAYER_OF_THE_YEAR as u32), // Belgian Young Player of the Year
        (0x9defb8, KEY_STAFF_COMP_ARGENTINIAN_PREMIER_DIVISION_TEAM_OF_THE as u32), // Argentinian Premier Division Team of the Week
        (0x9def88, KEY_STAFF_COMP_ARGENTINIAN_SECOND_DIVISION_TEAM_OF_THE as u32), // Argentinian Second Division Team of the Week
        (0x9def6c, KEY_STAFF_COMP_JAPANESE_PLAYER_OF_THE_YEAR as u32), // Japanese Player of the Year
        (0x9def54, KEY_STAFF_COMP_JAPANESE_TOP_GOALSCORER as u32), // Japanese Top Goalscorer
        (0x9def30, KEY_STAFF_COMP_JAPANESE_YOUNG_PLAYER_OF_THE_YEAR as u32), // Japanese Young Player of the Year
        (0x9def10, KEY_STAFF_COMP_JAPANESE_MANAGER_OF_THE_YEAR as u32), // Japanese Manager of the Year
        (0x9deee8, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_YEAR as u32), // Japanese J-League 1 Team of the Year
        (0x9deec4, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_YEAR as u32), // Japanese League 1 Team of the Year
        (0x9deeac, KEY_STAFF_COMP_WORLD_CUP_GOLDEN_BOOT as u32), // World Cup Golden Boot
        (0x9dee94, KEY_STAFF_COMP_WORLD_CUP_DREAM_TEAM as u32), // World Cup Dream Team
        (0x9dee74, KEY_STAFF_COMP_GERMAN_GOALKEEPER_OF_THE_YEAR as u32), // German Goalkeeper of the Year
        (0x9dee58, KEY_STAFF_COMP_GERMAN_SWEEPER_OF_THE_YEAR as u32), // German Sweeper of the Year
        (0x9dee3c, KEY_STAFF_COMP_GERMAN_DEFENDER_OF_THE_YEAR as u32), // German Defender of the Year
        (0x9dee1c, KEY_STAFF_COMP_GERMAN_MIDFIELDER_OF_THE_YEAR as u32), // German Midfielder of the Year
        (0x9dee00, KEY_STAFF_COMP_GERMAN_STRIKER_OF_THE_YEAR as u32), // German Striker of the Year
        (0x9dedd8, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_TOP_GOALSCORER as u32), // German First Division Top Goalscorer
        (0x9dedb8, KEY_STAFF_COMP_ENGLISH_FOOTBALLER_OF_THE_YEAR as u32), // English Footballer of the Year
        (0x9ded90, KEY_STAFF_COMP_ENGLISH_CONFERENCE_MANAGER_OF_THE_MONTH as u32), // English Conference Manager of the Month
        (0x9ded6c, KEY_STAFF_COMP_ENGLISH_PLAYERS_CONFERENCE_SELECT as u32), // English Players Conference Select
        (0x9ded44, KEY_STAFF_COMP_ENGLISH_CONFERENCE_MANAGER_OF_THE_YEAR as u32), // English Conference Manager of the Year
        (0x9ded1c, KEY_STAFF_COMP_ENGLISH_CONFERENCE_PLAYER_OF_THE_MONTH as u32), // English Conference Player of the Month
        (0x9decf8, KEY_STAFF_COMP_ENGLISH_CONFERENCE_TEAM_OF_THE_WEEK as u32), // English Conference Team of the Week
        (0x9decc8, KEY_STAFF_COMP_ENGLISH_CONFERENCE_YOUNG_PLAYER_OF_THE_M as u32), // English Conference Young Player of the Month
        (0x9dec98, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFC9C as u32), // Portuguese Premier Division Team of the Year
        (0x9dec6c, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFC9C as u32), // Portuguese Premier League Team of the Year
        (0x9dec40, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_Y as u32), // Portuguese Second Division Team of the Year
        (0x9dec14, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_Y as u32), // Portuguese Second League Team of the Year
        (0x9debe4, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFCA4 as u32), // Portuguese Premier Division Team of the Week
        (0x9debb8, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFCA4 as u32), // Portuguese Premier League Team of the Week
        (0x9deb8c, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_W as u32), // Portuguese Second Division Team of the Week
        (0x9deb60, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_W as u32), // Portuguese Second League Team of the Week
        (0x9deb38, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_WEEK as u32), // Japanese J-League 1 Team of the Week
        (0x9deb14, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_WEEK as u32), // Japanese League 1 Team of the Week
        (0x9deaec, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_WEEK as u32), // Japanese J-League 2 Team of the Week
        (0x9deac8, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_WEEK as u32), // Japanese League 2 Team of the Week
        (0x9dea9c, KEY_STAFF_COMP_SWEDISH_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // Swedish Premier Division Team of the Week
        (0x9dea74, KEY_STAFF_COMP_SWEDISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Swedish First Division Team of the Week
        (0x9dea44, KEY_STAFF_COMP_SWEDISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Swedish First Division North Team of the Week
        (0x9dea1c, KEY_STAFF_COMP_SPANISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Spanish First Division Team of the Week
        (0x9de9f0, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // Spanish Second Division Team of the Week
        (0x9de9cc, KEY_STAFF_COMP_ITALIAN_SERIE_A_TEAM_OF_THE_WEEK as u32), // Italian Serie A Team of the Week
        (0x9de9a8, KEY_STAFF_COMP_ITALIAN_SERIE_B_TEAM_OF_THE_WEEK as u32), // Italian Serie B Team of the Week
        (0x9de984, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TEAM_OF_THE_WEEK as u32), // Italian Serie C1/A Team of the Week
        (0x9de960, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TEAM_OF_THE_WEEK as u32), // Italian Serie C1/B Team of the Week
        (0x9de938, KEY_STAFF_COMP_DUTCH_PREMIER_DIVISION_TEAM_OF_THE_WEEK as u32), // Dutch Premier Division Team of the Week
        (0x9de910, KEY_STAFF_COMP_DUTCH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Dutch First Division Team of the Week
        (0x9de8e8, KEY_STAFF_COMP_GERMAN_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // German First Division Team of the Week
        (0x9de8c0, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // German Second Division Team of the Week
        (0x9de898, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // French First Division Team of the Week
        (0x9de878, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // French First Team of the Week
        (0x9de850, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // French Second Division Team of the Week
        (0x9de824, KEY_STAFF_COMP_DANISH_PREMIER_DIVISION_TEAM_OF_THE_WEEK as u32), // Danish Premier Division Team of the Week
        (0x9de7fc, KEY_STAFF_COMP_DANISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Danish First Division Team of the Week
        (0x9de7d4, KEY_STAFF_COMP_DANISH_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // Danish Second Division Team of the Week
        (0x9de7ac, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Belgian First Division Team of the Week
        (0x9de780, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // Belgian Second Division Team of the Week
        (0x9de750, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_PLAYER_OF_TH as u32), // Portuguese Premier Division Player of the Year
        (0x9de720, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_PLAYER_OF_TH as u32), // Portuguese Premier League Player of the Year
        (0x9de6ec, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_MOST_MAN_OF_T as u32), // Portuguese Second Division Most Man of the Match
        (0x9de6bc, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_MOST_MAN_OF_T as u32), // Portuguese Second League Most Man of the Match
        (0x9de690, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TOP_GOALSCORE as u32), // Portuguese Second Division Top Goalscorer
        (0x9de668, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TOP_GOALSCORE as u32), // Portuguese Second League Top Goalscorer
        (0x9de640, KEY_STAFF_COMP_GERMAN_SECOND_DIVISION_TOP_GOALSCORER as u32), // German Second Division Top Goalscorer
        (0x9de620, KEY_STAFF_COMP_SWEDISH_GOALKEEPER_OF_THE_YEAR as u32), // Swedish Goalkeeper of the Year
        (0x9de600, KEY_STAFF_COMP_SWEDISH_DEFENDER_OF_THE_YEAR as u32), // Swedish Defender of the Year
        (0x9de5e4, KEY_STAFF_COMP_SWEDISH_FORWARD_OF_THE_YEAR as u32), // Swedish Forward of the Year
        (0x9de5c8, KEY_STAFF_COMP_SWEDISH_ROOKIE_OF_THE_YEAR as u32), // Swedish Rookie of the Year
        (0x9de5a8, KEY_STAFF_COMP_SWEDISH_MIDFIELDER_OF_THE_YEAR as u32), // Swedish Midfielder of the Year
        (0x9de580, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_MANAGER_OF_THE_YEAR as u32), // Italian Serie C2/A Manager of the Year
        (0x9de558, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_PLAYER_OF_THE_YEAR as u32), // Italian Serie C2/A Player of the Year
        (0x9de534, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TEAM_OF_THE_WEEK as u32), // Italian Serie C2/A Team of the Week
        (0x9de50c, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_MANAGER_OF_THE_YEAR as u32), // Italian Serie C2/B Manager of the Year
        (0x9de4e4, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_PLAYER_OF_THE_YEAR as u32), // Italian Serie C2/B Player of the Year
        (0x9de4bc, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_YEAR as u32), // Japanese J-League 2 Team of the Year
        (0x9de498, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_YEAR as u32), // Japanese League 2 Team of the Year
        (0x9de468, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_GOALKEEPER_OF_TH as u32), // Spanish Second Division Goalkeeper of the Year
        (0x9de440, KEY_STAFF_COMP_SPANISH_SECOND_DIVISION_TOP_GOALSCORER as u32), // Spanish Second Division Top Goalscorer
        (0x9de41c, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TEAM_OF_THE_WEEK as u32), // Italian Serie C2/B Team of the Week
        (0x9de3f4, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_MANAGER_OF_THE_YEAR as u32), // Italian Serie C2/C Manager of the Year
        (0x9de3cc, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_PLAYER_OF_THE_YEAR as u32), // Italian Serie C2/C Player of the Year
        (0x9de3a8, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TEAM_OF_THE_WEEK as u32), // Italian Serie C2/C Team of the Week
        (0x9de380, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_TOP_GOALSCORER as u32), // Belgian First Division Top Goalscorer
        (0x9de350, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_YOUNG_PLAYER_OF_T as u32), // Belgian First Division Young Player of the Year
        (0x9de320, KEY_STAFF_COMP_BELGIAN_FIRST_DIVISION_YOUNG_PLAYER_OF_T as u32), // Belgian First Division Revelation of the Year
        (0x9de2f8, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_TOP_GOALSCORER as u32), // Belgian Second Division Top Goalscorer
        (0x9de2d0, KEY_STAFF_COMP_BELGIAN_THIRD_DIVISION_TEAM_OF_THE_WEEK as u32), // Belgian Third Division Team of the Week
        (0x9de2a4, KEY_STAFF_COMP_BELGIAN_THIRD_DIVISION_PLAYER_OF_THE_YEA as u32), // Belgian Third Division Player of the Year
        (0x9de27c, KEY_STAFF_COMP_BELGIAN_THIRD_DIVISION_TOP_GOALSCORER as u32), // Belgian Third Division Top Goalscorer
        (0x9de25c, KEY_STAFF_COMP_EUROPEAN_GOALKEEPER_OF_THE_YEAR as u32), // European Goalkeeper of the Year
        (0x9de23c, KEY_STAFF_COMP_EUROPEAN_DEFENDER_OF_THE_YEAR as u32), // European Defender of the Year
        (0x9de21c, KEY_STAFF_COMP_EUROPEAN_MIDFIELDER_OF_THE_YEAR as u32), // European Midfielder of the Year
        (0x9de1fc, KEY_STAFF_COMP_EUROPEAN_STRIKER_OF_THE_YEAR as u32), // European Striker of the Year
    ],
    // cadeia 3
    &[
        (0x9de1d8, KEY_STAFF_COMP_ITALIAN_SERIE_A_TEAM_OF_THE_YEAR as u32), // Italian Serie A Team of the Year
        (0x9de1b4, KEY_STAFF_COMP_ITALIAN_SERIE_B_TEAM_OF_THE_YEAR as u32), // Italian Serie B Team of the Year
        (0x9de190, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TEAM_OF_THE_YEAR as u32), // Italian Serie C1/A Team of the Year
        (0x9de16c, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TEAM_OF_THE_YEAR as u32), // Italian Serie C1/B Team of the Year
        (0x9de148, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TEAM_OF_THE_YEAR as u32), // Italian Serie C2/A Team of the Year
        (0x9de124, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TEAM_OF_THE_YEAR as u32), // Italian Serie C2/B Team of the Year
        (0x9de100, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TEAM_OF_THE_YEAR as u32), // Italian Serie C2/C Team of the Year
        (0x9de0e0, KEY_STAFF_COMP_ITALIAN_SERIE_A_TOP_GOALSCORER as u32), // Italian Serie A Top Goalscorer
        (0x9de0c0, KEY_STAFF_COMP_ITALIAN_SERIE_B_TOP_GOALSCORER as u32), // Italian Serie B Top Goalscorer
    ],
    // cadeia 4
    &[
        (0x9de09c, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TOP_GOALSCORER as u32), // Italian Serie C1/A Top Goalscorer
        (0x9de078, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TOP_GOALSCORER as u32), // Italian Serie C1/B Top Goalscorer
        (0x9de054, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TOP_GOALSCORER as u32), // Italian Serie C2/A Top Goalscorer
        (0x9de030, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TOP_GOALSCORER as u32), // Italian Serie C2/B Top Goalscorer
        (0x9de00c, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TOP_GOALSCORER as u32), // Italian Serie C2/C Top Goalscorer
        (0x9ddfdc, KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_MANAGER_OF_TH as u32), // Norwegian Premier Division Manager of the Month
        (0x9ddfac, KEY_STAFF_COMP_NORWEGIAN_PREMIER_DIVISION_PLAYER_OF_THE as u32), // Norwegian Premier Division Player of the Month
        (0x9ddf7c, KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_MANAGER_OF_THE as u32), // Norwegian First Division Manager of the Month
        (0x9ddf4c, KEY_STAFF_COMP_NORWEGIAN_FIRST_DIVISION_PLAYER_OF_THE_M as u32), // Norwegian First Division Player of the Month
        (0x9ddf1c, KEY_STAFF_COMP_PORTUGUESE_PREMIER_LEAGUE_MANAGER_OF_THE as u32), // Portuguese Premier League Manager of the Year
        (0x9ddeec, KEY_STAFF_COMP_PORTUGUESE_SECOND_LEAGUE_MANAGER_OF_THE as u32), // Portuguese Second League Manager of the Year
        (0x9dded0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR as u32), // Finnish Player of the Year
        (0x9ddeac, KEY_STAFF_COMP_FINNISH_YOUNG_PLAYER_OF_THE_YEAR as u32), // Finnish Young Player of the Year
        (0x9dde8c, KEY_STAFF_COMP_FINNISH_GOALKEEPER_OF_THE_YEAR as u32), // Finnish Goalkeeper of the Year
        (0x9dde70, KEY_STAFF_COMP_FINNISH_MANAGER_OF_THE_YEAR as u32), // Finnish Manager of the Year
        (0x9dde48, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TOP_GOALSCORER as u32), // Finnish Premier Division Top Goalscorer
        (0x9dde18, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_PLAYER_OF_THE_M as u32), // Finnish Premier Division Player of the Month
        (0x9dddec, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_YEA as u32), // Finnish Premier Division Team of the Year
        (0x9dddc0, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // Finnish Premier Division Team of the Week
        (0x9ddd98, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TOP_GOALSCORER as u32), // Finnish First Division Top Goalscorer
        (0x9ddd70, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // Finnish First Division Team of the Year
        (0x9ddd48, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Finnish First Division Team of the Week
        (0x9ddd2c, KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_YEAR as u32), // Irish Player Of The Year
        (0x9ddd0c, KEY_STAFF_COMP_IRISH_YOUNG_PLAYER_OF_THE_YEAR as u32), // Irish Young Player Of The Year
        (0x9ddce4, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_WEEK as u32), // Irish Premier Division Team Of The Week
        (0x9ddcb8, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_PLAYER_OF_THE_YEA as u32), // Irish Premier Division Player Of The Year
        (0x9ddc88, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_YOUNG_PLAYER_OF_T as u32), // Irish Premier Division Young Player Of The Year
        (0x9ddc60, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_YEAR as u32), // Irish Premier Division Team Of The Year
        (0x9ddc38, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // Irish First Division Team Of The Week
        (0x9ddc10, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR as u32), // Irish First Division Player Of The Year
        (0x9ddbe0, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_YOUNG_PLAYER_OF_THE as u32), // Irish First Division Young Player Of The Year
        (0x9ddbb8, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // Irish First Division Team Of The Year
        (0x9ddb8c, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_YE as u32), // Irish Premier Division Manager of the Year
        (0x9ddb60, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_MO as u32), // Irish Premier Division Manager of the Month
        (0x9ddb34, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_YEAR as u32), // Irish First Division Manager of the Year
        (0x9ddb08, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_MONT as u32), // Irish First Division Manager of the Month
        (0x9ddae0, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // polish first division team of the week
        (0x9ddab4, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR as u32), // polish first division player of the year
        (0x9dda84, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_YOUNG_PLAYER_OF_TH as u32), // polish first division young player of the YEAR
        (0x9dda58, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_YEA as u32), // polish first division manager of the year
        (0x9dda2c, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_MON as u32), // polish first division manager of the month
        (0x9dda04, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // polish second division team of the week
        (0x9dd9d8, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_PLAYER_OF_THE_YEA as u32), // polish second division player of the year
        (0x9dd9a8, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_YOUNG_PLAYER_OF_T as u32), // polish second division young player of the year
        (0x9dd97c, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_YE as u32), // polish second division manager of the year
        (0x9dd950, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_MO as u32), // polish second division manager of the month
        (0x9dd924, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_PLAYER_OF_THE_YE as u32), // Croatian first division player of the year
        (0x9dd8f8, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // croatian first division team of the week
        (0x9dd8d0, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_TOP_GOALSCORER as u32), // croatian first division top goalscorer
        (0x9dd8a0, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_PLAYER_OF_THE as u32), // croatian second division a player of the year
        (0x9dd874, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_TEAM_OF_THE_W as u32), // croatian second division a team of the week
        (0x9dd848, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_TOP_GOALSCORE as u32), // croatian second division a top goalscorer
        (0x9dd818, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_PLAYER_OF_THE as u32), // croatian second division b player of the year
        (0x9dd7ec, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_TEAM_OF_THE_W as u32), // croatian second division b team of the week
        (0x9dd7c0, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_TOP_GOALSCORE as u32), // croatian second division b top goalscorer
        (0x9dd7a0, KEY_STAFF_COMP_AUSTRALIAN_NSL_TEAM_OF_THE_WEEK as u32), // australian nsl team of the week
        (0x9dd780, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_MONTH as u32), // australian manager of the month
        (0x9dd75c, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_MONTH as u32), // australian nsl manager of the month
        (0x9dd73c, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_MONTH as u32), // australian player of the month
        (0x9dd718, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_MONTH as u32), // australian nsl player of the month
        (0x9dd6ec, KEY_STAFF_COMP_AUSTRALIAN_NSL_YOUNG_PLAYER_OF_THE_MONTH as u32), // australian nsl young player of the month
        (0x9dd6c4, KEY_STAFF_COMP_AUSTRALIAN_NSL_YOUNG_PLAYER_OF_THE_MONTH as u32), // australian young player of the month
        (0x9dd6a4, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_YEAR as u32), // australian player of the year
        (0x9dd680, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_YEAR as u32), // australian nsl player of the year
        (0x9dd658, KEY_STAFF_COMP_AUSTRALIAN_NSL_U21_PLAYER_OF_THE_YEAR as u32), // australian nsl u21 player of the year
        (0x9dd630, KEY_STAFF_COMP_AUSTRALIAN_NSL_U21_PLAYER_OF_THE_YEAR as u32), // australian nsl young player of the year
        (0x9dd60c, KEY_STAFF_COMP_AUSTRALIAN_GOALKEEPER_OF_THE_YEAR as u32), // australian goalkeeper of the year
        (0x9dd5e4, KEY_STAFF_COMP_AUSTRALIAN_GOALKEEPER_OF_THE_YEAR as u32), // australian nsl goalkeeper of the year
        (0x9dd5c8, KEY_STAFF_COMP_AUSTRALIAN_TOP_GOALSCORER as u32), // australian top goalscorer
        (0x9dd5a4, KEY_STAFF_COMP_AUSTRALIAN_TOP_GOALSCORER as u32), // australian nsl leading goalscorer
        (0x9dd584, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_YEAR as u32), // australian manager of the year
        (0x9dd560, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_YEAR as u32), // australian nsl manager of the year
        (0x9dd544, KEY_STAFF_COMP_GREEK_PLAYER_OF_THE_YEAR as u32), // greek player of the year
        (0x9dd520, KEY_STAFF_COMP_GREEK_FOREIGN_PLAYER_OF_THE_YEAR as u32), // greek foreign player of the year
        (0x9dd4f8, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_TEAM_OF_THE_WEEK as u32), // greek premier division team of the week
        (0x9dd4cc, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_PLAYER_OF_THE_YEA as u32), // greek premier division player of the year
        (0x9dd4a0, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_MANAGER_OF_THE_YE as u32), // greek premier division manager of the year
        (0x9dd478, KEY_STAFF_COMP_GREEK_PREMIER_DIVISION_TOP_GOALSCORER as u32), // greek premier division top goalscorer
        (0x9dd450, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_TEAM_OF_THE_WEEK as u32), // greek second division team of the week
        (0x9dd424, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_PLAYER_OF_THE_YEAR as u32), // greek second division player of the year
        (0x9dd3f8, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_MANAGER_OF_THE_YEA as u32), // greek second division manager of the year
        (0x9dd3d0, KEY_STAFF_COMP_GREEK_SECOND_DIVISION_TOP_GOALSCORER as u32), // greek second division top goalscorer
        (0x9dd3b4, KEY_STAFF_COMP_TURKISH_PLAYER_OF_THE_YEAR as u32), // turkish player of the year
        (0x9dd398, KEY_STAFF_COMP_TURKISH_MANAGER_OF_THE_YEAR as u32), // turkish manager of the year
        (0x9dd36c, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // turkish premier division team of the week
        (0x9dd340, KEY_STAFF_COMP_TURKISH_PREMIER_DIVISION_TEAM_OF_THE_YEA as u32), // turkish premier division team of the year
        (0x9dd318, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // turkish first division team of the week
        (0x9dd2f0, KEY_STAFF_COMP_TURKISH_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // turkish first division team of the year
        (0x9dd2d4, KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR as u32), // russian player of the year
        (0x9dd2a8, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // russian premier division team of the week
        (0x9dd27c, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_YEA as u32), // russian premier division team of the year
        (0x9dd24c, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_PLAYER_OF_THE_M as u32), // russian premier division player of the month
        (0x9dd224, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // russian first division team of the week
        (0x9dd1fc, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // russian first division team of the year
        (0x9dd1d0, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_PLAYER_OF_THE_MON as u32), // russian first division player of the month
        (0x9dd1b0, KEY_STAFF_COMP_IRISH_PERSONALITY_OF_THE_YEAR as u32), // Irish Personality of the Year
        (0x9dd194, KEY_STAFF_COMP_N_IRISH_PLAYER_OF_THE_YEAR as u32), // n. irish player of the year
        (0x9dd168, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_TEAM_OF_THE_WEE as u32), // n. irish premier division team of the week
        (0x9dd138, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_PLAYER_OF_THE_Y as u32), // n. irish premier division player of the year
        (0x9dd104, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_YOUNG_PLAYER_OF as u32), // n. irish premier division young player of the year
        (0x9dd0d8, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_TOP_GOALSCORER as u32), // n. irish premier division top goalscorer
        (0x9dd0a8, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFF14 as u32), // n. irish premier division manager of the year
        (0x9dd078, KEY_STAFF_COMP_N_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFF18 as u32), // n. irish premier division manager of the month
        (0x9dd04c, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_TEAM_OF_THE_WEEK as u32), // n. irish first division team of the week
        (0x9dd020, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_PLAYER_OF_THE_YEA as u32), // n. irish first division player of the year
        (0x9dcfec, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_YOUNG_PLAYER_OF_T as u32), // n. irish first division young player of the year
        (0x9dcfc4, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_TOP_GOALSCORER as u32), // n. irish first division top goalscorer
        (0x9dcf98, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_MANAGER_OF_THE_YE as u32), // n. irish first division manager of the year
        (0x9dcf68, KEY_STAFF_COMP_N_IRISH_FIRST_DIVISION_MANAGER_OF_THE_MO as u32), // n. irish first division manager of the month
        (0x9dcf4c, KEY_STAFF_COMP_IRISH_LEAGUE_GOLDEN_BOOT as u32), // irish league golden boot
        (0x9dcf30, KEY_STAFF_COMP_IRISH_LEAGUE_BRONZE_BOOT as u32), // irish league bronze boot
        (0x9dcf00, KEY_STAFF_COMP_BELGIAN_SECOND_DIVISION_REVELATION_OF_TH as u32), // belgian second division revelation of the year
        (0x9dcee8, KEY_STAFF_COMP_WELSH_TEAM_OF_THE_WEEK as u32), // welsh team of the week
        (0x9dcecc, KEY_STAFF_COMP_WELSH_PLAYER_OF_THE_YEAR as u32), // welsh player of the year
        (0x9dceac, KEY_STAFF_COMP_WELSH_YOUNG_PLAYER_OF_THE_YEAR as u32), // welsh young player of the year
        (0x9dce94, KEY_STAFF_COMP_WELSH_TOP_GOALSCORER as u32), // welsh top goalscorer
        (0x9dce78, KEY_STAFF_COMP_WELSH_MANAGER_OF_THE_YEAR as u32), // welsh manager of the year
        (0x9dce5c, KEY_STAFF_COMP_WELSH_MANAGER_OF_THE_MONTH as u32), // welsh manager of the month
        (0x9dce34, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_YEAR as u32), // french first division team of the year
        (0x9dce0c, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_YEAR as u32), // french second division team of the year
        (0x9dcde8, KEY_STAFF_COMP_FRENCH_NATIONAL_PLAYER_OF_THE_YEAR as u32), // french national player of the year
        (0x9dcdc0, KEY_STAFF_COMP_FRENCH_NATIONAL_GOALKEEPER_OF_THE_YEAR as u32), // french national goalkeeper of the year
        (0x9dcd9c, KEY_STAFF_COMP_FRENCH_NATIONAL_TEAM_OF_THE_WEEK as u32), // french national team of the week
        (0x9dcd78, KEY_STAFF_COMP_FRENCH_NATIONAL_TEAM_OF_THE_YEAR as u32), // french national team of the year
        (0x9dcd5c, KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_MONTH as u32), // irish player of the month
        (0x9dcd30, KEY_STAFF_COMP_FRENCH_PLAYERS_NATIONAL_PLAYER_OF_THE_YE as u32), // french players national player of the year
        (0x9dcd14, KEY_STAFF_COMP_OCEANIA_PLAYER_OF_THE_YEAR as u32), // oceania player of the year
        (0x9dcce8, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_PLAYER_OF_THE_MON as u32), // finnish first division player of the month
        (0x9dccb8, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_MANAGER_OF_THE as u32), // finnish premier division manager of the month
        (0x9dcc8c, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_MANAGER_OF_THE_MO as u32), // finnish first division manager of the month
        (0x9dcc60, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_PLAYER_OF_THE_YEA as u32), // Brazilian 1st Division Player of the Year
    ],
    // cadeia 5
    &[
        (0x9dcc38, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TEAM_OF_THE_WEEK as u32), // Brazilian 1st Division Team of the Week
        (0x9dcc10, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TEAM_OF_THE_YEAR as u32), // Brazilian 1st Division Team of the Year
        (0x9dcbe8, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TOP_GOALSCORER as u32), // Brazilian 1st Division Top Goalscorer
        (0x9dcbb8, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_YOUNG_PLAYER_OF_T as u32), // Brazilian 1st Division Young Player of the Year
        (0x9dcb8c, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_PLAYER_OF_THE_YEA as u32), // Brazilian 2nd Division Player of the Year
        (0x9dcb64, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TEAM_OF_THE_WEEK as u32), // Brazilian 2nd Division Team of the Week
        (0x9dcb3c, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TEAM_OF_THE_YEAR as u32), // Brazilian 2nd Division Team of the Year
        (0x9dcb14, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TOP_GOALSCORER as u32), // Brazilian 2nd Division Top Goalscorer
        (0x9dcae4, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_YOUNG_PLAYER_OF_T as u32), // Brazilian 2nd Division Young Player of the Year
        (0x9dcab8, KEY_STAFF_COMP_BRAZILIAN_3RD_DIVISION_PLAYER_OF_THE_YEA as u32), // Brazilian 3rd Division Player of the Year
        (0x9dca90, KEY_STAFF_COMP_BRAZILIAN_3RD_DIVISION_TEAM_OF_THE_WEEK as u32), // Brazilian 3rd Division Team of the Week
        (0x9dca68, KEY_STAFF_COMP_BRAZILIAN_3RD_DIVISION_TEAM_OF_THE_YEAR as u32), // Brazilian 3rd Division Team of the Year
        (0x9dca40, KEY_STAFF_COMP_BRAZILIAN_3RD_DIVISION_TOP_GOALSCORER as u32), // Brazilian 3rd Division Top Goalscorer
        (0x9dca10, KEY_STAFF_COMP_BRAZILIAN_3RD_DIVISION_YOUNG_PLAYER_OF_T as u32), // Brazilian 3rd Division Young Player of the Year
        (0x9dc9f0, KEY_STAFF_COMP_BRAZILIAN_BAHIA_BEST_GOALKEEPER as u32), // Brazilian Bahia Best Goalkeeper
        (0x9dc9d4, KEY_STAFF_COMP_BRAZILIAN_BAHIA_BEST_PLAYER as u32), // Brazilian Bahia Best Player
        (0x9dc9b0, KEY_STAFF_COMP_BRAZILIAN_BAHIA_TEAM_OF_THE_WEEK as u32), // Brazilian Bahia Team of the Week
        (0x9dc990, KEY_STAFF_COMP_BRAZILIAN_BAHIA_TOP_GOALSCORER as u32), // Brazilian Bahia Top Goalscorer
        (0x9dc96c, KEY_STAFF_COMP_BRAZILIAN_CENTRAL_BEST_GOALKEEPER as u32), // Brazilian Central Best Goalkeeper
        (0x9dc94c, KEY_STAFF_COMP_BRAZILIAN_CENTRAL_BEST_PLAYER as u32), // Brazilian Central Best Player
        (0x9dc928, KEY_STAFF_COMP_BRAZILIAN_CENTRAL_TEAM_OF_THE_WEEK as u32), // Brazilian Central Team of the Week
        (0x9dc904, KEY_STAFF_COMP_BRAZILIAN_CENTRAL_TOP_GOALSCORER as u32), // Brazilian Central Top Goalscorer
        (0x9dc8e0, KEY_STAFF_COMP_BRAZILIAN_GAUCHO_BEST_GOALKEEPER as u32), // Brazilian Gaúcho Best Goalkeeper
        (0x9dc8c0, KEY_STAFF_COMP_BRAZILIAN_GAUCHO_BEST_PLAYER as u32), // Brazilian Gaúcho Best Player
        (0x9dc89c, KEY_STAFF_COMP_BRAZILIAN_GAUCHO_TEAM_OF_THE_WEEK as u32), // Brazilian Gaúcho Team of the Week
        (0x9dc87c, KEY_STAFF_COMP_BRAZILIAN_GAUCHO_TOP_GOALSCORER as u32), // Brazilian Gaúcho Top Goalscorer
        (0x9dc85c, KEY_STAFF_COMP_BRAZILIAN_GOIAS_BEST_GOALKEEPER as u32), // Brazilian Goiás Best Goalkeeper
        (0x9dc840, KEY_STAFF_COMP_BRAZILIAN_GOIAS_BEST_PLAYER as u32), // Brazilian Goiás Best Player
        (0x9dc81c, KEY_STAFF_COMP_BRAZILIAN_GOIAS_TEAM_OF_THE_WEEK as u32), // Brazilian Goiás Team of the Week
        (0x9dc7fc, 0x9d0000), // Brazilian Goiás Top Goalscorer
        (0x9dc7d4, 0x9d0004), // Brazilian Minas Gerais Best Goalkeeper
        (0x9dc7b0, 0x9d0008), // Brazilian Minas Gerais Best Player
        (0x9dc788, 0x9d000c), // Brazilian Minas Gerais Team of the Week
        (0x9dc760, 0x9d0010), // Brazilian Minas Gerais Top Goalscorer
        (0x9dc740, 0x9d0014), // Brazilian North Best Goalkeeper
        (0x9dc724, 0x9d0018), // Brazilian North Best Player
        (0x9dc700, 0x9d001c), // Brazilian North Team of the Week
        (0x9dc6e0, 0x9d0020), // Brazilian North Top Goalscorer
        (0x9dc6bc, 0x9d0024), // Brazilian Northeast Best Goalkeeper
        (0x9dc69c, 0x9d0028), // Brazilian Northeast Best Player
        (0x9dc674, 0x9d002c), // Brazilian Northeast Team of the Week
        (0x9dc650, 0x9d0030), // Brazilian Northeast Top Goalscorer
        (0x9dc62c, 0x9d0034), // Brazilian Paraná Best Goalkeeper
        (0x9dc60c, 0x9d0038), // Brazilian Paraná Best Player
        (0x9dc5e8, 0x9d003c), // Brazilian Paraná Team of the Week
        (0x9dc5c8, 0x9d0040), // Brazilian Paraná Top Goalscorer
        (0x9dc5a0, 0x9d0044), // Brazilian Pernambuco Best Goalkeeper
        (0x9dc57c, 0x9d0048), // Brazilian Pernambuco Best Player
        (0x9dc554, 0x9d004c), // Brazilian Pernambuco Team of the Week
        (0x9dc530, 0x9d0050), // Brazilian Pernambuco Top Goalscorer
        (0x9dc504, 0x9d0054), // Brazilian Rio de Janeiro Best Goalkeeper
        (0x9dc4dc, 0x9d0058), // Brazilian Rio de Janeiro Best Player
        (0x9dc4b0, 0x9d005c), // Brazilian Rio de Janeiro Team of the Week
        (0x9dc488, 0x9d0060), // Brazilian Rio de Janeiro Top Goalscorer
        (0x9dc45c, 0x9d0064), // Brazilian Santa Catarina Best Goalkeeper
        (0x9dc434, 0x9d0068), // Brazilian Santa Catarina Best Player
        (0x9dc408, 0x9d006c), // Brazilian Santa Catarina Team of the Week
        (0x9dc3e0, 0x9d0070), // Brazilian Santa Catarina Top Goalscorer
        (0x9dc3bc, 0x9d0074), // Brazilian São Paulo Best Goalkeeper
        (0x9dc39c, 0x9d0078), // Brazilian São Paulo Best Player
        (0x9dc374, 0x9d007c), // Brazilian São Paulo Team of the Week
        (0x9dc350, 0x9d0080), // Brazilian São Paulo Top Goalscorer
        (0x9dc32c, 0x9d0084), // South Korean Best 11 Of The Year
        (0x9dc304, 0x9d0088), // South Korean Goalkeeper Of The Month
        (0x9dc2e0, 0x9d008c), // South Korean Manager Of The Year
        (0x9dc2b0, 0x9d0090), // South Korean Most Assisted Player Of The Year
        (0x9dc28c, 0x9d0094), // South Korean Player Of The Month
        (0x9dc26c, 0x9d0098), // South Korean Player Of The Year
        (0x9dc240, 0x9d009c), // South Korean Top Goal Scorer Of The Year
        (0x9dc218, 0x9d00a0), // South Korean Young Player Of The Year
    ],
];

/// matches the club competition long name with the 301 fixed keys (351 names in 3 chains, _stricmp; the two Croatian
/// divisions also store 0x9cf8a4)
/// Original: `match_club_comp_key_id` (0x60efd0, index.cpp, cdecl).
///
/// [pt] { 0x60efd0 } (cdecl, index.cpp, match_club_comp_key_id): casa o nome de competição de clube da base com as chaves fixas
/// do jogo (CHAVES_DE_COMPETICAO_DE_CLUBE): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
pub(crate) unsafe extern "C" fn casar_chave_de_competicao_de_clube(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_STRICMP);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_COMPETICAO_DE_CLUBE.iter() {
        let mut i = 0;
        while i < cadeia.len() {
            let (texto, global) = cadeia[i];
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de competição de clube
                // as gravações extras do mesmo acerto (texto 0)
                while i + 1 < cadeia.len() && cadeia[i + 1].0 == 0 {
                    i += 1;
                    *(cadeia[i].1 as *mut u32) = linha;
                }
                break;
            }
            i += 1;
            while i < cadeia.len() && cadeia[i].0 == 0 {
                i += 1;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de competição de clube)
}

/// matches the staff competition or award name with the 417 fixed keys (442 names in 5 chains, _stricmp)
/// Original: `match_staff_comp_key_id` (0x6115e0, index.cpp, cdecl).
///
/// [pt] { 0x6115e0 } (cdecl, index.cpp, match_staff_comp_key_id): casa o nome de competição ou prêmio de pessoa da base com as chaves fixas
/// do jogo (CHAVES_DE_COMPETICAO_DE_PESSOA): em cada cadeia, o primeiro texto igual ao nome (_stricmp 0x951c50, sem
/// distinguir maiúsculas) grava a `linha` na global da chave e a cadeia acaba; sem acerto, segue
/// para a próxima. Devolve o eax da original: o resultado do último _stricmp (0 se a última cadeia
/// acertou).
pub(crate) unsafe extern "C" fn casar_chave_de_competicao_de_pessoa(nome: usize, linha: u32) -> u32 {
    let stricmp: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_STRICMP);
    let mut r = 0i32;
    for cadeia in CHAVES_DE_COMPETICAO_DE_PESSOA.iter() {
        for &(texto, global) in cadeia.iter() {
            r = stricmp(nome, texto as usize);
            if r == 0 {
                *(global as *mut u32) = linha; // chave de competição de pessoa
                break;
            }
        }
    }
    r as u32 // o eax do último _stricmp (chave de competição de pessoa)
}


pub(crate) unsafe extern "C" fn comparar_casar_chave_de_competicao_de_clube(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_CASAR_CHAVE_DE_COMPETICAO_DE_CLUBE);
    let reg: [(usize, usize); 1] = [(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION, 0x4c0)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_competicao_de_clube(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_competicao_de_clube(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

pub(crate) unsafe extern "C" fn comparar_casar_chave_de_competicao_de_pessoa(nome: usize, linha: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, u32) -> u32>(IDX_CASAR_CHAVE_DE_COMPETICAO_DE_PESSOA);
    let reg: [(usize, usize); 1] = [(KEY_STAFF_COMP_FIFA_WORLD_PLAYER_OF_THE_YEAR, 0x684)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(nome, linha), || casar_chave_de_competicao_de_pessoa(nome, linha));
    PLACAR.registrar(igual && r == n, || {
        let t = std::ffi::CStr::from_ptr(nome as *const std::os::raw::c_char).to_string_lossy();
        format!("casar_chave_de_competicao_de_pessoa(\"{}\", {}) original={} rust={} {}", t, linha, r as i32, n as i32, chaves_diferentes(&mo, &mr))
    });
    r
}

