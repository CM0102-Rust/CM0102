use super::*;

// ---------- 217a leva: o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor (match_stats.cpp) ----------

/// checks for the CM01/02 CD and builds the statistics: loads from the save or creates the index, the buckets and the
/// zeroed temporary file
/// Original: `Match_Stats::Match_Stats` (0x729000, match_stats.cpp, thiscall).
///
/// [pt] { 0x729000 } (thiscall, ret 8, match_stats.cpp): o construtor das estatísticas da partida.
/// Confere o CD: entre as units_digit (GetLogicalDriveStringsA, 200) a primeira de CD (GetDriveTypeA 5)
/// com "<raiz>cm0102.exe" (0x4c0aa0; nas outras fecha a busca, 0x4c0c10); sem ela, a caixa "Insert
/// CD" e tenta de novo, para sempre. Com `zip` e a carga do save (0x72aa80) pronto; senão o índice de
/// `n` entradas (+4, [this] com n × 12 bytes; sem memória, erro 0x62 e o jogo fecha), +0x1c = n, os
/// quatro cursores dos baldes (+8: k × n / 4) e o "match_stats_matches.tmp" (CM3_TEMP, "wb") com `n`
/// registros zerados de 0xd7a bytes (erro 0x71 sem abrir, 0x7e se a escrita falha), +0x18 = 0. O
/// registro local nasce e morre pelos iteradores de vetor do runtime (0x9463af/0x9462bb) como na
/// original. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construct_match_stats(estat: usize, zip: usize, n: i32) -> usize {
    crate::comparacao::ramo("estatisticas montar");
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let units_digit: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let tipo: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let procurar: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FILE_FIND_FIRST);
    let fechar_busca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_FILE_FIND_CLOSE);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let carregar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_STATS_LOAD);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let d = |o: usize| (estat + o) as *mut i32;

    let mut registro = [0u8; 0xd7a];
    let r = registro.as_mut_ptr() as usize;
    vetor(r + 0x11f, 0x625, 2, 0x453c30, 0x453c10);

    // o exe de trabalho (cm0102-1080) vem sem a exigência do CD: o jne do tipo da unidade (0x729089)
    // virou nop e o test do achado (0x7290cb, 85 c0) virou xor (31 c0 no cm0102-1080; 33 c0 é a outra
    // codificação); a cópia lê os bytes do próprio exe
    let qualquer_unidade = *(0x729089 as *const u8) == 0x90;
    let sempre_achou = *(0x7290cb as *const u8) != 0x85; // o original tem `test eax, eax` (85 c0); qualquer outra coisa é patch sem CD (xor 31 c0 ou 33 c0)
    'cd: loop {
        // ponytail: o buffer nasce zerado; a original só leria lixo com mais de 200 bytes de units_digit
        // (mais de 50 letras), impossível
        let mut lista = [0u8; 0x100];
        if units_digit(200, lista.as_mut_ptr()) != 0 && lista[0] != 0 {
            let mut i = 0usize;
            loop {
                let raiz = lista.as_ptr() as usize + i;
                if qualquer_unidade || tipo(raiz) == 5 {
                    let mut nome = [0u8; 0x104];
                    let mut achado = [0u8; 0x114];
                    let nm = nome.as_mut_ptr() as usize;
                    formatar(nm, 0x987e00, raiz, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let ach = achado.as_mut_ptr() as usize;
                    if procurar(nm, ach, 0) == 0 || sempre_achou {
                        break 'cd;
                    }
                    fechar_busca(ach);
                }
                // ponytail: a original anda com um i8 (passos de 4): só passaria de 127 com mais de 32 units_digit
                i += 4;
                if lista[i] == 0 {
                    break;
                }
            }
        }
        let mut titulo = [0u8; 0x100];
        let mut msg = [0u8; 0x100];
        texto(titulo.as_mut_ptr() as usize, 0x987df4);
        texto(msg.as_mut_ptr() as usize, 0x987dc4);
        caixa(titulo.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }

    if zip != 0 && carregar(estat, zip) != 0 {
        fim_do_vetor(r + 0x11f, 0x625, 2, 0x453c10);
        return estat;
    }
    *d(4) = n;
    let t = novo((n as u32).wrapping_mul(12));
    *(estat as *mut usize) = t;
    if t == 0 {
        erro_em(0xa17ba0, 0x62);
    }
    *d(0x1c) = n;
    let mut soma = 0i32;
    for k in 0..4 {
        *d(8 + 4 * k) = soma / 4;
        soma = soma.wrapping_add(n);
    }
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x9a7f28);
    if f == 0 {
        aviso_em(0xa17ba0, 0x71);
    } else {
        core::ptr::write_bytes(r as *mut u8, 0, 0xd7a);
        let mut ok = true;
        let mut i = 0;
        while i < n {
            if escrever(r, 0xd7a, 1, f) != 1 {
                aviso_em(0xa17ba0, 0x7e);
                fechar(f);
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            fechar(f);
            *d(0x18) = 0;
        }
    }
    fim_do_vetor(r + 0x11f, 0x625, 2, 0x453c10);
    estat
}

/// frees the record index and clears its pointer and count
/// Original: `Match_Stats::~Match_Stats` (0x7293c0, match_stats.cpp, thiscall).
///
/// [pt] { 0x7293c0 } (thiscall, match_stats.cpp): o destrutor das estatísticas da partida: libera o índice
/// ([this]) e zera o ponteiro e a quantidade (+4).
pub(crate) unsafe extern "thiscall" fn destroy_match_stats(estat: usize) {
    crate::comparacao::ramo("estatisticas desmontar");
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let t = *(estat as *const usize);
    if t != 0 {
        liberar(t);
        *(estat as *mut usize) = 0;
        *((estat + 4) as *mut u32) = 0;
    }
}

// ponytail: o construtor confere o CD (caixa que espera o clique), aloca o índice e recria o arquivo
// temporário; o destrutor libera o índice: rodar duas vezes não dá, então os comparadores só chamam a
// original; as cópias se provam no modo Rust (os ramos: a carreira automática cria o jogo e passa pelo
// construtor).
pub(crate) unsafe extern "thiscall" fn comparar_construct_match_stats(estat: usize, zip: usize, n: i32) -> usize {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> usize>(IDX_CONSTRUCT_MATCH_STATS)(estat, zip, n)
}

pub(crate) unsafe extern "thiscall" fn comparar_destroy_match_stats(estat: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_MATCH_STATS)(estat)
}

// ---------- 212a leva: o registro da partida, a ordem dos eventos e o idioma que o jogador perde (match_stats.cpp, player_stats.cpp) ----------

/// fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation,
/// captain and the 20 players of each side
/// Original: `Match_Stats::fill_match_record` (0x729ff0, match_screens.cpp, thiscall).
///
/// [pt] { 0x729ff0 } (thiscall, ret 0xc, match_stats.cpp; `this` não é usado): preenche o registro de
/// estatística `reg` da partida `m` (jogo em [m+0x4792]): clubes, datas, público (+0x34) e renda
/// (+0x38, 0x59af40 só sem `sem_renda`), o placar por tempo, o estádio, o árbitro, o minuto / 11, o
/// clima (0x923010 em +0xd76), até 50 eventos de 4 bytes em +0x57 (ordenados por 0x72b160, fim 0xff)
/// e, por time (+0x11f + 0x625 × t), as estatísticas, a formação (0x5b2ae0), o capitão (0x897590 com
/// `sem_renda`) e os 20 jogadores de 0x49 bytes (número da camisa por 0x85a760, dando um com
/// 0x85a470 quando falta). Partida ou registro nulos: a original (caixa de erro). Devolve 1.
pub(crate) unsafe extern "thiscall" fn fill_match_record(estat: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    if m == 0 || reg == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_FILL_MATCH_RECORD)(estat, m, reg, sem_renda);
    }
    let b = |p: usize| *(p as *const u8);
    let w = |p: usize| core::ptr::read_unaligned(p as *const u16);
    let d = |p: usize| core::ptr::read_unaligned(p as *const u32);
    let pb = |p: usize, v: u8| *(p as *mut u8) = v;
    let pw = |p: usize, v: u16| core::ptr::write_unaligned(p as *mut u16, v);
    let pd = |p: usize, v: u32| core::ptr::write_unaligned(p as *mut u32, v);
    let data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let atexit: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_ATEXIT);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_GET_MATCH_WEATHER);
    let renda: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FINANCETABLE_CALC_GATE_RECEIPTS);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let capitao: unsafe extern "thiscall" fn(usize, usize, i32, i32) -> i8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let posicao: unsafe extern "thiscall" fn(usize, i32, i32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(FN_SQUAD_MANAGER_GET_SQUAD_NUMBER);
    let dar_numero: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
    let copiar_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    // a data estática (dia 0x29 de 1972) da primeira chamada: a marca antes da construção, como na original
    if b(0xb6b7c4) & 1 == 0 {
        pb(0xb6b7c4, b(0xb6b7c4) | 1);
        data(0xb6b7b8, 0x29, 0x7b4);
        atexit(0x72aa70);
    }
    let jogo = || d(m + 0x4792) as usize;
    pd(reg, d(jogo() + 0xc));
    pd(reg + 4, d(jogo() + 0x10));
    pw(reg + 8, w(jogo() + 0x2a));
    pw(reg + 0xa, w(jogo() + 0x28));
    pd(reg + 0x34, d(m + 0xf634));
    for k in 0..5 {
        pw(reg + 0xc + 2 * k, w(m + 0x1f6 + 2 * k));
        pw(reg + 0x16 + 2 * k, w(m + 0x200 + 2 * k));
        pw(reg + 0x20 + 2 * k, w(m + 0x20a + 2 * k));
        pw(reg + 0x2a + 2 * k, w(m + 0x214 + 2 * k));
    }
    pd(reg + 0x38, 0);
    let estadio = d(m + 0x477a) as i32;
    let p = if estadio > -1 { d((d(0xae23bc) as i32 + estadio * 0x6e + 0x61) as usize) as usize } else { 0 };
    pd(reg + 0x3c, if p != 0 { d(p) } else { u32::MAX });
    let q = d(m + 0xf5b6) as usize;
    pd(reg + 0x40, if q != 0 && d(q) as i32 > -1 { d(q) } else { u32::MAX });
    pw(reg + 0x4d, (w(m + 0x8ed0) as i16 as i32 / 11) as u16);
    pw(reg + 0x4f, w(m + 0x8ed4));
    pw(reg + 0x51, w(m + 0x8ed2));
    pb(reg + 0x4c, b(m));
    pd(reg + 0x53, matches!(b(m + 0x8eb3), 1 | 4) as u32);
    clima(d(0xde2f74) as usize, reg + 0xd76, jogo());
    if sem_renda == 0 {
        pd(reg + 0x38, renda(d(0xae2a38) as usize, jogo(), d(reg + 0x34)));
    }
    // os eventos: código do lance → tipo (1..8); 7 e 8 marcam +0x53 e guardam a ordem no lugar do minuto
    let ev = d(m + 0x8f7a) as usize;
    let mut n: u32 = 0;
    if ev != 0 && (w(ev + 6) as i16) > 0 {
        let mut i: i32 = 0;
        while i < w(ev + 6) as i16 as i32 {
            let e = ev + 0x720 + (i as i16 as i32 * 14) as usize;
            let c = w(e) as i16 as i32;
            let tipo: u8 = match c {
                0x1f56 | 0x1f57 => 6,
                0x1fcc | 0x2013..=0x201b | 0x2114 => 1,
                0x1fcd => 2,
                0x1fce => 4,
                0x1fcf => 3,
                0x1ff5..=0x1ff8 => 5,
                0x2000 => 7,
                0x2008 => 8,
                _ => 0,
            };
            if tipo != 0 && n != 0x32 {
                let especial = tipo >= 7;
                if especial {
                    pd(reg + 0x53, 1);
                }
                let s = reg + 0x57 + 4 * n as usize;
                pb(s, (b(e + 6) != 0) as u8 | ((n as u8).wrapping_add(1) << 1));
                pb(s + 1, b(e + 5));
                pb(s + 2, tipo);
                pb(s + 3, if especial { n as u8 } else { b(e + 4) });
                n += 1;
            }
            i += 1;
        }
        if n > 0 {
            ordenar(reg + 0x57, n, 4, 0x72b160);
            for k in 0..n as usize {
                pb(reg + 0x57 + 4 * k, b(reg + 0x57 + 4 * k) & 1);
            }
        }
    }
    if n < 0x32 {
        pb(reg + 0x57 + 4 * n as usize, 0xff);
    }
    for t in 0..2usize {
        let tm = reg + 0x625 * t + 0x11f;
        let s = m + 0x36 * t + 0xf5bc;
        let id = d(reg + 4 * t) as i32;
        let clube = (d(0xae23b4) as i32 + id * 0x245) as usize;
        let usa = b(jogo() + 0x40 + t) != 0 && usa_numeros(d(0xdd7ef0) as usize, clube, 0) != 0;
        for (o, de) in [(0, 1), (1, 0), (3, 4), (4, 5), (2, 6), (5, 7), (6, 8), (7, 9), (8, 2), (9, 3), (0xa, 0xb),
                        (0xb, 0xa), (0xc, 0xc), (0xd, 0xd), (0xe, 0xe), (0xf, 0x10), (0x10, 0xf)] {
            pb(tm + o, b(s + de));
        }
        for (o, de) in [(0x11, 0x26), (0x13, 0x28), (0x15, 0x2a), (0x17, 0x2c), (0x19, 0x2e), (0x1b, 0x30), (0x1d, 0x34)] {
            pw(tm + o, w(s + de));
        }
        for k in 0..5 {
            pb(tm + 0x1f + k, b(s + 0x12 + k));
            pb(tm + 0x24 + k, b(s + 0x21 + k));
            pb(tm + 0x29 + k, b(s + 0x1c + k));
            pb(tm + 0x2e + k, b(s + 0x17 + k));
        }
        pb(tm + 0x33, b(m + 0x8d4f + t));
        if sem_renda != 0 {
            let fora = (b(jogo() + 0x40 + t) == 0) as i32;
            let c = capitao(d(0xde1d08) as usize, d(jogo() + 0x1c + 4 * t) as usize, fora, 0) as i32;
            pb(tm + 0x34, b((m as isize + (c + t as i32 * 20) as isize * 0x1be + 0x4796) as usize));
        } else {
            pb(tm + 0x34, b(s + 0x11));
        }
        let f = m + 0x18e3 * t;
        for k in 0..11 {
            pw(tm + 0x35 + 2 * k, posicao(f + 0xc3a8, k as i32, 0));
            pw(tm + 0x4b + 2 * k, posicao(f + 0xc3a8, k as i32, 1));
        }
        pd(tm + 0x61, d(f + 0xc92c));
        let g = m + 0xd0 * t;
        let tecnico = d(g + 0x904d) as usize;
        let ultimo = if tecnico != 0 {
            pd(tm + 0x65, d(tecnico));
            pw(tm + 0x69, w(g + 0x9032));
            pw(tm + 0x6b, w(g + 0x9034));
            w(g + 0x9036)
        } else {
            pd(tm + 0x65, u32::MAX);
            pw(tm + 0x69, 0xffff);
            pw(tm + 0x6b, 0xffff);
            0xffff
        };
        pw(tm + 0x6d, ultimo);
        pw(tm + 0x6f, w(m + 2 * t + 0x1d6));
        for k in 0..20usize {
            let x = tm + 0x8d + 0x49 * k;
            let y = m + 0x22d8 * t + 0x47ff + 0x1be * k;
            let gerente = || d(0xdd7ef0) as usize;
            if usa {
                if d(y + 4) == 0 {
                    pb(x, 0);
                } else {
                    let mut v = (numero(gerente(), d(y) as usize, clube) as i32).wrapping_abs() as u8;
                    pb(x, v);
                    if v == 0 {
                        dar_numero(gerente(), clube, d(y) as usize);
                        v = (numero(gerente(), d(y) as usize, clube) as i32).wrapping_abs() as u8;
                        pb(x, v);
                    }
                }
            } else {
                pb(x, if d(y) == 0 { 0 } else { (k as u8) + 1 });
            }
            pd(x - 0x1c, if d(y) != 0 { d(d(y) as usize) } else { u32::MAX });
            pb(x + 1, b(y - 0x50));
            pb(x + 2, b(y - 0x4f));
            if d(x - 0x1c) as i32 <= -1 {
                continue;
            }
            pd(x - 0x18, if d(y + 4) != 0 { d(d(y + 4) as usize) } else { u32::MAX });
            pd(x - 0x14, d(y - 0xc));
            pd(x - 0x10, d(y - 8));
            pd(x - 0xc, d(y - 4));
            let mut copia = [0u8; 8];
            copiar_data(x - 8, copia.as_mut_ptr() as usize, if d(y) != 0 { d(y) as usize + 0x10 } else { 0xb6b7b8 });
            pb(x + 3, b(y - 0x4d));
            // ponytail: fld f32 × fmul f64 (0,01) em 80 bits no x87; em f64 dá o mesmo truncamento (f32 × 0,01 nunca cai a 2^-53 de um inteiro)
            let nota = core::ptr::read_unaligned((y - 0x1c) as *const f32) as f64 * core::ptr::read_unaligned(0x967898 as *const f64);
            pb(x + 4, ftol(nota) as u8);
            for (o, de) in [(5, 0x5d), (6, 0x5a), (7, 0x5c), (8, 0x5b), (9, 0x5f), (0xa, 0x5e), (0xb, 0x59), (0xc, 0x58),
                            (0xd, 0x55), (0xe, 0x54), (0xf, 0x56), (0x10, 0x45), (0x11, 0x46), (0x12, 0x44), (0x13, 0x43),
                            (0x16, 0x4a), (0x17, 0x47), (0x14, 0x49)] {
                pb(x + o, b(y - de));
            }
            pw(x + 0x1a, ftol(core::ptr::read_unaligned((y - 0x18) as *const f32) as f64) as u16);
            pb(x + 0x15, b(y - 0x52));
            pb(x + 0x18, b(y - 0x48));
            pw(x + 0x1c, w(y - 0x22));
            pw(x + 0x1e, w(y - 0x20));
            pw(x + 0x20, w(y - 0x1e));
            pb(x + 0x19, b(y - 0x4e));
            for k2 in 0..9 {
                pb(x + 0x22 + k2, b(y - 0x68 + k2));
            }
            pb(x + 0x2b, b(y - 0x57));
        }
    }
    1
}

/// orders two match events by type group, then by minute and arrival order (qsort comparator)
/// Original: `match_stats_compare_events` (0x72b160, match_screens.cpp, cdecl).
///
/// [pt] { 0x72b160 } (cdecl, match_stats.cpp, comparador do qsort dos eventos de 4 bytes): mesmo grupo
/// (tipos 1..3 entre si, 7..8 entre si, ou o mesmo tipo +2) desempata pelo minuto (+3, sem sinal) e
/// depois pela ordem de chegada (+0 com sinal, metade); grupos diferentes: a diferença dos tipos.
pub(crate) unsafe extern "C" fn match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let (ta, tb) = (*a.add(2), *b.add(2));
    let mesmo = (matches!(ta, 1..=3) && matches!(tb, 1..=3)) || (matches!(ta, 7 | 8) && matches!(tb, 7 | 8));
    if !mesmo {
        let r = ta as i8 as i32 - tb as i8 as i32;
        if r != 0 {
            return r;
        }
    }
    let r = *a.add(3) as i32 - *b.add(3) as i32;
    if r != 0 {
        return r;
    }
    (*a as i8 as i32 >> 1) - (*b as i8 as i32 >> 1)
}


pub(crate) unsafe extern "thiscall" fn comparar_fill_match_record(estat: usize, m: usize, reg: usize, sem_renda: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_FILL_MATCH_RECORD);
    // ponytail: a primeira chamada (data estática e atexit) e os erros só pela original
    if m == 0 || reg == 0 || *(G_MATCH_STATS_FILL_RECORD_STATIC_DATE_GUARD as *const u8) & 1 == 0 {
        return o(estat, m, reg, sem_renda);
    }
    // o número da camisa dado pela original (0x85a470) fica fora das regiões: a cópia já o acha dado
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg, 0xd7a)], || o(estat, m, reg, sem_renda),
        || fill_match_record(estat, m, reg, sem_renda));
    PLACAR.registrar(igual && r == n, || {
        let difs: Vec<usize> = mo.0[0].1.iter().zip(mr.0[0].1.iter()).enumerate().filter(|(_, (a, b))| a != b).map(|(i, _)| i).take(16).collect();
        format!("fill_match_record(sem_renda={}) original={} rust={} bytes diferentes em {:x?}", sem_renda, r, n, difs)
    });
    r
}

pub(crate) unsafe extern "C" fn comparar_match_stats_compare_events(a: *const u8, b: *const u8) -> i32 {
    let r = orig::<unsafe extern "C" fn(*const u8, *const u8) -> i32>(IDX_MATCH_STATS_COMPARE_EVENTS)(a, b);
    let n = match_stats_compare_events(a, b);
    PLACAR.registrar(r == n, || format!("match_stats_compare_events({:x?}, {:x?}) original={} rust={}",
        core::slice::from_raw_parts(a, 4), core::slice::from_raw_parts(b, 4), r, n));
    r
}


// ---------- 209a leva: as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes (match_screens.cpp) ----------

/// Os 25 registros de aba (0xd4 bytes: +4 o título, +0xcc a marca, +0xd0 o valor) que a barra de abas
/// (0x5ed780) lê, construídos (0x4568f0) e destruídos (0x5a53c0) pelos iteradores do runtime
/// (0x9463af/0x9462bb) como na original.
pub(crate) type AbasDaPartida = [u32; 0x14b4 / 4];

pub(crate) unsafe fn construir_abas_da_partida(abas: &mut AbasDaPartida) {
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    vetor(abas.as_mut_ptr() as usize, 0xd4, 0x19, 0x4568f0, 0x5a53c0);
}

pub(crate) unsafe fn destruir_abas_da_partida(abas: &mut AbasDaPartida) {
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    fim_do_vetor(abas.as_mut_ptr() as usize, 0xd4, 0x19, 0x5a53c0);
}

/// O título da aba `k` (texto em +4) e a marca e o valor dela.
pub(crate) unsafe fn titulo_da_aba(abas: &mut AbasDaPartida, k: usize) -> usize {
    abas.as_mut_ptr() as usize + k * 0xd4 + 4
}
pub(crate) unsafe fn marcar_aba(abas: &mut AbasDaPartida, k: usize, marca: u32, valor: u32) {
    abas[k * 53 + 51] = marca;
    abas[k * 53 + 52] = valor;
}

/// Libera a foto da partida do parâmetro 0x1d (0xd7a bytes; o vetor de 2 x 0x625 em +0x11f, 0x453c10)
/// e zera o parâmetro.
pub(crate) unsafe fn liberar_foto_da_partida() {
    if parametro_da_tela(0x1d) != 0 {
        let f = parametro_da_tela(0x1d);
        if f != 0 {
            let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
            let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
            fim_do_vetor(f + 0x11f, 0x625, 2, 0x453c10);
            apagar(f);
        }
        gravar_parametro_da_tela(0x1d, 0);
    }
}

/// O fundo das telas da partida: a foto (0x41c670), senão o da competição (0x41c410), senão o do clube
/// (0x41c250 e 0x41bfb0). Devolve o `eax` da última chamada (a original empilha o byte da cor do
/// kit por cima dele).
pub(crate) unsafe fn fundo_da_partida(foto: usize, competicao: usize, clube: usize) -> usize {
    let pela_foto: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_BACKGROUND_SET_STADIUM_PICTURE);
    let pela_competicao: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_BACKGROUND_SET_COMP_PICTURE);
    let pelo_clube: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_BACKGROUND_SET_CLUB_PICTURE);
    let aplicar: unsafe extern "C" fn() -> usize = core::mem::transmute(FN_BACKGROUND_SET_RANDOM_PICTURE);
    if foto != 0 {
        let r = pela_foto(foto, 0, 1);
        if r != 0 {
            return r;
        }
    }
    let r = pela_competicao(competicao, 0, 1);
    if r != 0 {
        return r;
    }
    pelo_clube(clube, 0, 1);
    aplicar()
}

/// A caixa "Error" de match_screens.cpp na linha `linha` (sem parar o jogo) e [0xb67a34] = 0.
pub(crate) unsafe fn erro_das_telas_da_partida(linha: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa161d4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
}

/// O nome do time no texto 0xde1f64 do cabeçalho da partida: só o nome ("  %s") com `so_o_nome`;
/// senão a seleção B ("'B'", 0x53b900) ou o time reserva ("Res.").
pub(crate) unsafe fn nome_do_time_na_partida(time: usize, so_o_nome: u8) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let b_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if so_o_nome != 0 {
        formatar(0xde1f64, 0x987ff0, time + 0x38);
    } else if b_national_team(time) != 0 {
        let c = *((time + 0x37) as *const i8) as i32;
        texto(0xde1f64, 0xa164a4, c, c, time + 4);
    } else {
        let c = *((time + 0x52) as *const i8) as i32;
        texto(0xde1f64, 0xa16478, c, c, time + 0x38);
    }
}

/// Uma linha da velocidade da narração `valor` na lista `lista`, marcada (0x2002) quando é a atual `v`
/// (a devolução de 0x5fcff0). A original empilha uma das três cores (0xae31aa, 0xaebdec, 0xaebdea, nessa
/// ordem) no `eax` que sobrou de 0x5fcff0 (os 16 bits de cima conhecidos) e as outras em registradores
/// sujos (aqui estendidas); `sujo` diz qual, (marcada, não marcada).
pub(crate) unsafe fn linha_de_velocidade(lista: usize, valor: u16, v: u32, sujo: (usize, usize)) -> usize {
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let marcada = v as u16 == valor;
    let k = if marcada { sujo.0 } else { sujo.1 };
    let enderecos = [0xae31aausize, 0xaebdec, 0xaebdea];
    let cor = |i: usize| if i == k { cor_sobre(v as usize, enderecos[i]) } else { cor_em(enderecos[i]) };
    let marca = if marcada { 0x2002 } else { 2 };
    linha(display_do_humano(), marca, 0, 0, 0, 0, 0, 0, 0x10, cor(0), cor(1), 1, 1, cor(2), 0xde1f64, 0, 0xc, valor as usize, lista)
}

/// builds the live match screen: team headers and score, commentary, clock, control, tactics buttons, commentary
/// speeds, tabs, the selected tab panel, the stadium and the commentary panel
/// Original: `build_match_screen` (0x718e90, match_screens.cpp, cdecl).
///
/// [pt] { 0x718e90 } (cdecl, SEH, match_screens.cpp, ~5 KB): monta a tela da partida ao vivo. Liga a marca
/// +0xea da ficha do humano (0x765480) e libera a foto do parâmetro 0x1d. Modo 1 (parâmetro 1): só a
/// figura de fundo vazia (0x41c0d0). Senão: tira a marca de "saiu" (−911) do humano nos espectadores
/// ([partida+0x5b] +0xc5 + 4h, contando em +0x108); o fundo; os cabeçalhos dos dois times (cores do
/// kit 0x53b640, nome, caixa do clube, placar ou "A" de abandonada) e o agregado ([+0x4f] ≥ 0;
/// somado ao placar fora do modo 2 de [+0x67]); a área da narração (fundo "match.mbr"), a do relógio
/// (minuto [+0x4d] no parâmetro 3), as setas "<<<"/">>>" (ou os símbolos da fonte [0x9cc2c4]), o
/// controle (0x725a70, parâmetro 4) e os botões táticos dos dois times (parâmetros 5 e 6; o do
/// parâmetro 0x15 marcado); com o humano no comando de um dos times (0x600c90) ou sem humano nenhum
/// (0x600a50), as velocidades da narração (0x5fcff0; a atual marcada, o foco [Display+0x12f4ff] e o
/// volume do narrador [+0x2db0]). Por fim a ficha +0 = 3, as abas (Overview, Stats, Zones, Report, as
/// estatísticas dos times, Ratings, Latest Scores e, para liga de verdade sem seleções, League Table),
/// −1 no parâmetro 8, o painel da aba (parâmetro 2), o estádio e, com o parâmetro 8 em −1, o painel de
/// narração (com o lance do parâmetro 7 narrado por 0x6d7b00) no parâmetro 8.
pub(crate) unsafe extern "C" fn build_match_screen() {
    crate::comparacao::ramo("telas montar a partida");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MENOS1: usize = usize::MAX;
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let texto_do_gerente: TextoDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: CaixaDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let cores: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;
    let velocidade = || {
        let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_GET_COMMENTARY_SPEED);
        f(*(G_HUMAN_MANAGER as *const usize), 0)
    };
    let gravar_item = |k: u32, r: usize| gravar_parametro_da_tela(k, r as i16 as isize as usize);

    let mut abas: AbasDaPartida = [0; 0x14b4 / 4];
    construir_abas_da_partida(&mut abas);
    *((ficha(0) + 0xea) as *mut u32) = 1;
    liberar_foto_da_partida();
    let p = parametro_da_tela(0);
    let modo = parametro_da_tela(1) as u8;
    let aba = parametro_da_tela(2) as u8;
    let p15 = parametro_da_tela(0x15);
    parametro_da_tela(0xe);
    if modo == 1 {
        let figura: unsafe extern "C" fn(usize) = core::mem::transmute(FN_BACKGROUND_SET_PICTURE);
        figura(0);
        destruir_abas_da_partida(&mut abas);
        return;
    }
    let espectadores = rd(p + 0x5b);
    let st = rd(p + 0x5f);
    let h = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let marca = (espectadores as isize + 0xc5 + h * 4) as *mut i32;
    if core::ptr::read_unaligned(marca) == -911 {
        core::ptr::write_unaligned(marca, 0);
        let n = (espectadores + 0x108) as *mut u8;
        *n = (*n).wrapping_sub(1);
    }
    fundo_da_partida(rd(p + 0x1c), rd(p + 0x18), rd(p + 0x20));

    // os cabeçalhos: as cores u16 (o resto das variáveis da original é lixo da pilha, aqui 0); o byte do
    // kit vai num registrador sujo (estendido)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    cores(rd(p + 0x20), rb(st + 0x152) as usize, &mut cor_a, &mut cor_b);
    nome_do_time_na_partida(rd(p + 0x20), rb(p + 0x44));
    texto_do_gerente(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 1, rd(p + 0x20));
    let r = if rb(st + 0x11f) == 0xfd { formatar(BUF, 0x9b760c) } else { formatar(BUF, 0x98806c, rb(st + 0x11f) as u32) };
    texto_do_gerente(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    cores(rd(p + 0x24), rb(st + 0x777) as usize, &mut cor_a, &mut cor_b);
    nome_do_time_na_partida(rd(p + 0x24), rb(p + 0x45));
    texto_do_gerente(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 1, rd(p + 0x24));
    let r = if rb(st + 0x11f) == 0xfd { formatar(BUF, 0x9b760c) } else { formatar(BUF, 0x98806c, rb(st + 0x744) as u32) };
    texto_do_gerente(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    let agregado = sx(p + 0x4f);
    if agregado > -1 {
        let (a, b) = if rb(p + 0x67) != 2 {
            (agregado + rb(st + 0x11f) as i32, sx(p + 0x50) + rb(st + 0x744) as i32)
        } else {
            (agregado, sx(p + 0x50))
        };
        texto(BUF, 0xa16454, a, b);
        texto_do_gerente(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }

    // a narração, o relógio e as setas
    // o y2 e o imediato do push em 0x719345: 0x257 no exe 800x600, 0x437 no cm0102-1080 (resolucao.py)
    let y2 = core::ptr::read_unaligned(0x719346 as *const u32) as usize;
    let narracao = lista(display_do_humano(), 0, 0, 0x59, y2, 1, 0, 1, 0, 1, 0, MENOS1);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let fundo = caminho(0x9b5348, 0xa16448);
    linha(display_do_humano(), 0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, fundo, 0, 0, 0, narracao);
    let relogio = lista(display_do_humano(), 5, 0xa, 0x55, 0x24e, 1, 0, 0xd, 0, 1, 0, MENOS1);
    formatar(BUF, 0x98806c, *((st + 0x4d) as *const i16) as i32);
    let r = texto_da_celula(sm, relogio, 0, 0, 0x1061, 0, 0xc, 5, cor_em(0xaebdec), BUF);
    gravar_item(3, r);
    let setas = lista(display_do_humano(), 5, 0xa, 0x55, 0x24e, 2, 0, 0xd, 0, 1, 0, MENOS1);
    if *(G_FONT_MODE as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x28);
        celula(sm, setas, 0, 1, 0x1021, 0, 0xc, 0, cor_em(0xaebdec), BUF, (-2isize) as usize, 0);
    } else {
        // a original empilha o eax com [0x9cc2c4] = −1 em cima da cor
        celula(sm, setas, 0, 1, 0x1021, 0, 0xc, 1, cor_sobre(MENOS1, 0xae3184), 0xa16444, (-2isize) as usize, 0);
    }
    if *(G_FONT_MODE as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x29);
        celula(sm, setas, 1, 1, 0x1021, 0, 0xc, 0, cor_em(0xaebdec), BUF, (-3isize) as usize, 0);
    } else {
        celula(sm, setas, 1, 1, 0x1021, 0, 0xc, 1, cor_em(0xae3184), 0xa16440, (-3isize) as usize, 0);
    }

    // o controle da partida (0x725a70 lê só o byte do modo; a cor da original fica nos 16 bits de baixo
    // da variável que guardava os espectadores)
    let rotulo: unsafe extern "C" fn(u32, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_MATCH_SCREENS_GET_MATCH_CONTROL_LABEL);
    let mut marcas: u32 = 0;
    rotulo(modo as u32, espectadores, BUF, 0x7d0, &mut marcas);
    let r = linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 2, marcas as usize, 0, 0, 0xc, 1, cor_sobre(espectadores, 0xae3184), BUF, 0, 2, 0, relogio);
    gravar_item(4, r);

    // os botões táticos dos dois times (o do parâmetro 0x15 marcado)
    for (k, lado, param) in [(3usize, 0x20usize, 5u32), (4, 0x24, 6)] {
        let t = rd(p + lado);
        let c = sx(t + 0x52);
        texto(BUF, 0xa16428, c, c, t + 0x38);
        let r = if t == p15 {
            // no segundo time a original empilha a cor sobre o ponteiro do time (eax)
            let cor = if k == 4 { cor_sobre(t, 0xae3184) } else { cor_em(0xae3184) };
            celula(sm, relogio, 0, k, 0x1061, 0, 0xc, 1, cor, BUF, 1, t)
        } else {
            celula(sm, relogio, 0, k, 0x1021, 0, 0xc, 1, cor_em(0xae3184), BUF, 1, t)
        };
        gravar_item(param, r);
    }

    // as velocidades da narração
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let mostrar = manda(rd(p + 0x20), 0, 1) != 0
        || manda(rd(p + 0x24), 0, 1) != 0
        || (humano(rd(p + 0x20), 0, 1, 0, 0) == 0 && humano(rd(p + 0x24), 0, 1, 0, 0) == 0);
    if mostrar {
        texto(BUF, 0xa16414);
        let r = linha(display_do_humano(), 0x82, 0, 0, 0, 0, 0, 5, 0x1021, 0, 0, 0xc, 1, cor_sobre(p15, 0xae3184), BUF, 0, 0, 0, relogio);
        let v = lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor_em(0xae31aa), r);
        texto(BUF, 0xa16408);
        let muito_rapida = linha_de_velocidade(v, 0x14, velocidade(), (0, 1));
        texto(BUF, 0xa16400);
        let rapida = linha_de_velocidade(v, 0x37, velocidade(), (2, 0));
        texto(BUF, 0xa163cc, 0xad9c64);
        let media = linha_de_velocidade(v, 0x5a, velocidade(), (1, 2));
        texto(BUF, 0xa163c4);
        let lenta = linha_de_velocidade(v, 0x7d, velocidade(), (0, 1));
        texto(BUF, 0xa163b8);
        let muito_lenta = linha_de_velocidade(v, 0x96, velocidade(), (2, 0));
        // o foco vai para a linha de cima da atual (a muito lenta para a muito rápida e as fora da tabela)
        let foco = match velocidade() as i16 {
            0x37 => muito_rapida,
            0x5a => rapida,
            0x7d => media,
            0x96 => lenta,
            _ => muito_lenta,
        };
        let d = display_do_humano();
        core::ptr::write_unaligned((d + 0x12f4ff) as *mut u16, foco as u16);
        *((d + 0x12f4fe) as *mut u8) = 0x20;
        let narrador = rd(rd(p + 0x57) + 0x8f7a);
        *((narrador + 0x2db0) as *mut u8) = velocidade() as u8;
    }
    *(ficha(0) as *mut u8) = 3;

    // as abas
    texto(titulo_da_aba(&mut abas, 0), 0xa163a8);
    marcar_aba(&mut abas, 0, 3, 1);
    texto(titulo_da_aba(&mut abas, 1), 0xa1639c);
    marcar_aba(&mut abas, 1, 3, 2);
    texto(titulo_da_aba(&mut abas, 2), 0xa1638c);
    marcar_aba(&mut abas, 2, 3, 3);
    texto(titulo_da_aba(&mut abas, 3), 0xa1637c);
    marcar_aba(&mut abas, 3, 3, 4);
    let t1 = rd(p + 0x20);
    texto(titulo_da_aba(&mut abas, 4), 0xa16364, sx(t1 + 0x52), sx(t1 + 0x52), t1 + 0x38);
    marcar_aba(&mut abas, 4, 3, 6);
    texto(titulo_da_aba(&mut abas, 5), 0xa16354);
    marcar_aba(&mut abas, 5, 3, 7);
    texto(titulo_da_aba(&mut abas, 6), 0xa16328, 0xad9c64);
    marcar_aba(&mut abas, 6, 3, 8);
    let mut n: usize = 7;
    // a tabela da liga da partida (a de seleções pelo continente do clube do humano; jogo de copa pela fase)
    let ligas = |k: isize| rd((rd(0xadadfc) as isize + k * 4) as usize);
    let comp = rd(p + 4) as i32 as isize;
    let entrada = ligas(comp);
    let mut liga: usize = 0;
    if entrada != 0 {
        let selecoes = *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const i32) as isize;
        let candidata = if comp == selecoes {
            if sx(p + 0x46) >= 2 {
                None
            } else {
                let vaga = rd((0xb74c26 + h * 0x300) as usize);
                let lado = if rd(rd(p + 0x24) + 0xcf) == vaga { rd(p + 0x24) } else { rd(p + 0x20) };
                let continente: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_USA_MLS_GET_CLUB_DIVISION);
                let selecoes = || *(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE as *const i32) as isize;
                Some(if continente(lado) as u16 == 0x414 {
                    rd(rd(ligas(selecoes()) + 0xc) + 4)
                } else if continente(lado) as u16 == 0x47b {
                    rd(rd(ligas(selecoes()) + 0xc))
                } else {
                    ligas(selecoes())
                })
            }
        } else {
            let fase = *((p + 0x46) as *const i8);
            Some(if fase == -1 { entrada } else { rd((rd(entrada + 0xc) as isize + fase as isize * 4) as usize) })
        };
        if let Some(l) = candidata {
            liga = l;
            let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
            if l != 0 && rb(l + 0x43) == 2 && selecao(*(G_RESERVE_COMP as *const usize), rd(l + 4)) == 0 {
                texto(titulo_da_aba(&mut abas, 7), 0x99c280);
                marcar_aba(&mut abas, 7, 3, 0xa);
                n = 8;
            }
        }
    }
    let t2 = rd(p + 0x24);
    texto(titulo_da_aba(&mut abas, n), 0xa16364, sx(t2 + 0x52), sx(t2 + 0x52), t2 + 0x38);
    marcar_aba(&mut abas, n, 3, 9);
    let (mut topo, mut base) = (0x50i32, 0x221i32);
    let barra: unsafe extern "C" fn(usize, u32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_TAB_BAR);
    let aba = aba as i8 as i32;
    barra(abas.as_mut_ptr() as usize, n as u32 + 1, aba, &mut topo, &mut base, 0);
    gravar_parametro_da_tela(8, MENOS1);
    let painel = |f: usize, topo: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(topo, base)
    };
    let estatisticas_do_time: unsafe extern "C" fn(u32, *mut i32, *mut i32) = core::mem::transmute(FN_DRAW_TEAM_STATS_PANEL);
    match aba {
        1 => painel(0x71c270, &mut topo, &mut base),
        2 => painel(0x71e040, &mut topo, &mut base),
        3 => painel(0x71f500, &mut topo, &mut base),
        4 => painel(0x721000, &mut topo, &mut base),
        6 => estatisticas_do_time(0, &mut topo, &mut base),
        7 => painel(0x722ed0, &mut topo, &mut base),
        8 => painel(0x723d20, &mut topo, &mut base),
        9 => estatisticas_do_time(1, &mut topo, &mut base),
        10 => {
            let tabela: unsafe extern "C" fn(usize, i32, i32) = core::mem::transmute(FN_MATCH_SCREENS_DRAW_LIVE_LEAGUE_TABLE);
            tabela(liga, topo, base);
        }
        _ => {}
    }

    // o estádio ("estádio, cidade"; sem cidade só o nome, sem estádio "Unknown Stadium")
    let estadio = rd(p + 0x1c);
    let r: usize = if estadio != 0 {
        let cidade = rd(estadio + stadium::STADIUM_CITY);
        if cidade != 0 {
            let (a, b) = (sx(estadio + stadium::STADIUM_GENDER_NAME), sx(cidade + city::CITY_GENDER_NAME));
            texto(BUF, 0x9902a4, a, a, b, b, estadio + 4, cidade + 4) as usize
        } else {
            // o strcpy da original deixa eax e ecx em 0
            copiar_para_o_texto(estadio + 4);
            0
        }
    } else {
        texto(BUF, 0x99c1a4) as usize
    };
    texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 0x30, cor_em(0xae317e), 0xc, 3, cor_sobre(r, 0xae3184), BUF, 1);

    // o painel de narração
    if parametro_da_tela(8) as u16 == 0xffff {
        if parametro_da_tela(7) != 0 {
            let narrador = rd(rd(p + 0x57) + 0x8f7a);
            let r = texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_EVENTS_SHOW_EVENT_TEXT);
            narrar(narrador, rd(rd(p + 0x5b) + 1), st, rd(p + 4), r, 0x2b2, 0, 3, 0);
            gravar_item(8, r);
        } else {
            let r = texto_do_gerente(sm, 0x64, 0x22b, 0x316, 0x24e, 1, 0, 0xc, 3, 0, 0, 1);
            gravar_item(8, r);
        }
    }
    destruir_abas_da_partida(&mut abas);
}

/// handles the live match screen: commentary volume and speed, the new event (clock, commentary, modes), watching or
/// not, a team's tactics, switching tabs and each tab's input
/// Original: `handle_match_screen_input` (0x71a240, match_screens.cpp, cdecl).
///
/// [pt] { 0x71a240 } (cdecl, match_screens.cpp): a resposta da tela da partida ao vivo ao item `item` (−1:
/// a passagem do tempo). Em rede ([0xb77c84]) ou com a partida a confirmar no modo 1 sem confirmação:
/// 0; espectadores fechados ([+0x119]): −9; modo 1: confirma pela rede (0x6b5260) e, confirmado, passa
/// ao modo 2 e −11. O item de id 0xc troca o volume do narrador ([+0x2db0]); volume diferente da
/// velocidade do técnico (0x5fcff0) repassa (0x5fd100) e −11. Sem item: com o humano conectado
/// (0x5fd210), os modos 2 e 3 viram 4 (deixa de assistir) e o 5 para a narração (0x802a50, −9); no modo
/// 4 com lance novo (0x6b55c0) lê as marcas do menu da partida (+0: 1 e 2 refazem tudo, 4 o relógio, 8
/// narra o lance (0x6d7b00) e, nos de substituição, marca o time do humano no parâmetro 0x15, 0x20 e
/// 0x40 os modos 3 e 5); a marca de "esperando" (parâmetro 0x24) e o rótulo do controle (0x725a70); sem
/// time marcado, procura o do humano (parâmetros 0x2f/0x30). Com um time a abrir (o marcado, ou o
/// item de id 1): abre a tática dele (0x899a50, −4) se o jogo permite ([+0x8eb2] sem lance), senão
/// remarca os botões (0x800d30) e o parâmetro 0x15. Item de id 2: assistir ou não (modos 2/3/4), 5
/// para (−9); id 3: troca a aba (parâmetro 2). A barra de menu (0x7643d0; −4 marca o humano como
/// saindo, −911) e depois a aba decide (os painéis 0x71d9e0, 0x71e750, 0x720320 (id 4 troca o período
/// do parâmetro 0xa), 0x723350; as respostas 0x722dc0 e 0x725860; a 10 converte o parâmetro 0x26 no
/// 0x25). Por fim −11 com [0xb6b7b0] ou o redesenho (0x802d00) com [0xb6b7a8].
pub(crate) unsafe extern "C" fn handle_match_screen_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond a partida");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let g = |a: usize| *(a as *const u32);
    let por = |a: usize, v: u32| *(a as *mut u32) = v;
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let h = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let margem: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_STYLE);
    let texto_do_item: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let humano_no_time: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let usa: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let indice: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(FN_SCREENMANAGER_FIND_MANAGER_CHANNEL);
    let parar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_SCREENMANAGER_STOP_SOUND);
    let rotulo: unsafe extern "C" fn(u32, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_MATCH_SCREENS_GET_MATCH_CONTROL_LABEL);
    if g(0xb77c84) != 0 {
        return 0;
    }
    let p = parametro_da_tela(0);
    let mut modo = parametro_da_tela(1) as u8;
    let aba = parametro_da_tela(2) as u8;
    let p3 = parametro_da_tela(3);
    let p4 = parametro_da_tela(4);
    let p5 = parametro_da_tela(5);
    let p6 = parametro_da_tela(6);
    let p8 = parametro_da_tela(8);
    let mut p15 = parametro_da_tela(0x15);
    let m = rd(p + 0x5b);
    if rd(m + 0x119) != 0 {
        return -9;
    }
    if modo == 1 {
        if rd(p + 0x57) == 0 {
            let confirmar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MATCH_DAY_START_WATCHED_MATCH);
            confirmar(*(G_MATCH_DAY as *const usize), p);
        }
        if rd(p + 0x57) == 0 {
            return 0;
        }
        gravar_parametro_da_tela(1, 2);
        return -11;
    }
    let pm = rd(p + 0x57);
    let st = rd(p + 0x5f);
    let item16 = item as u16 as i16;
    let registro = |i: i16| (display_do_humano() as isize + i as isize * 0x18c) as usize;
    if id_do_item_ou_padrao(item16) == 0xc {
        let v = if item16 != -1 { rb(registro(item16) + 0xba9a6) } else { 0 };
        *((rd(pm + 0x8f7a) + 0x2db0) as *mut u8) = v;
    }
    let volume = rb(rd(pm + 0x8f7a) + 0x2db0);
    let velocidade: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_GET_COMMENTARY_SPEED);
    if volume as u16 != velocidade(*(G_HUMAN_MANAGER as *const usize), 0) as u16 {
        // a original empilha o ecx com os 16 bits de cima do que sobrou de 0x5fcff0 (aqui estendido)
        let repassar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_HUMAN_MANAGER_SET_COMMENTARY_SPEED);
        repassar(*(G_HUMAN_MANAGER as *const usize), rb(rd(pm + 0x8f7a) + 0x2db0) as u32, 0);
        return -11;
    }
    por(0xb6b7ac, 0);
    por(0xb6b7b0, 0);
    por(0xb6b7a8, 0);
    let espectador = || (m as isize + 0xc5 + h() as isize * 4) as *mut i32;
    let quantos = (m + 0x108) as *mut u8;
    let assistir = || {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), 1);
            *quantos = (*quantos).wrapping_add(1);
        }
    };
    let deixar_de_assistir = || {
        if core::ptr::read_unaligned(espectador()) != 0 {
            core::ptr::write_unaligned(espectador(), 0);
            *quantos = (*quantos).wrapping_sub(1);
        }
    };
    let t_do_menu = |d: usize| {
        let x = rd(pm + 0x4792);
        if x != 0 { rd(x + d) } else { 0 }
    };
    // o time `t` é o do humano da vez (0x600a50 e a vaga dele em 0x7fffa0)? então é o do parâmetro 0x15
    let marcar_se_do_humano = |t: usize, p15: &mut usize| {
        if humano_no_time(t, 0, 1, 0, 0) != 0 && indice(sm, usa(t)) == h() as u16 {
            *p15 = t;
            gravar_parametro_da_tela(0x15, t);
        }
    };

    // o time a abrir (Some) ou seguir para o item
    let alvo: Option<usize> = if item16 == -1 {
        let mut ao_lance = false;
        if rd((0xb74c26 + h() as isize * 0x300) as usize) != 0 {
            let conectado: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            if conectado(*(G_HUMAN_MANAGER as *const usize), 0) != 0 {
                if modo == 2 || modo == 3 {
                    gravar_parametro_da_tela(1, 4);
                    modo = 4;
                    deixar_de_assistir();
                    *((p + 0x67) as *mut u8) = 1;
                    por(0xb6b7b0, 1);
                    ao_lance = true;
                } else if modo == 5 {
                    parar(sm, -1);
                    return -9;
                }
            }
        }
        if !ao_lance && modo == 4 {
            ao_lance = true;
        }
        let lance: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_DAY_STEP_LIVE_MATCH);
        if ao_lance && lance(*(G_MATCH_DAY as *const usize), p) != 0 {
            let marcas = || rb(m);
            if marcas() & 1 != 0 {
                por(0xb6b7b0, 1);
            }
            if marcas() & 2 != 0 {
                por(0xb6b7b0, 1);
            }
            if marcas() & 4 != 0 && g(0xb6b7b0) == 0 {
                let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                formatar(BUF, 0x98806c, *((st + 0x4d) as *const i16) as i32);
                texto_do_item(sm, p3, BUF);
                por(0xb6b7a8, 1);
            }
            if marcas() & 8 != 0 {
                if g(0xb6b7b0) == 0 {
                    let narrador = rd(pm + 0x8f7a);
                    // (largura, quebrar, fonte, modo); o modo da original é um sete em dl sobre um edx sujo
                    let (largura, quebrar, fonte, modo_n) = if aba == 1 && rd(st + 0x53) == 0 { (0x262, 1, 5, 0) } else { (0x2b2, 0, 3, (aba == 5) as u32) };
                    let comp = rd(rd(pm + 0x4792));
                    let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_EVENTS_SHOW_EVENT_TEXT);
                    narrar(narrador, rd(m + 1), st, comp, p8, largura, quebrar, fonte, modo_n);
                    por(0xb6b7a8, 1);
                }
                gravar_parametro_da_tela(7, 1);
                let lance = rd(m + 1);
                let codigo = *(lance as *const u16);
                if matches!(codigo, 0x1ff8 | 0x1ff7 | 0x1ff5 | 0x1ff6 | 0x1f56) {
                    let t = t_do_menu(0x1c);
                    if rd(pm + 0x4792) != 0 && t != 0 && rb(rd(m + 1) + 6) == 0 {
                        marcar_se_do_humano(t, &mut p15);
                    }
                    let t = t_do_menu(0x20);
                    if rd(pm + 0x4792) != 0 && t != 0 && rb(rd(m + 1) + 6) == 1 {
                        marcar_se_do_humano(t, &mut p15);
                    }
                }
            } else if parametro_da_tela(7) != 0 {
                if g(0xb6b7b0) == 0 {
                    margem(sm, p8, 1);
                    texto_do_item(sm, p8, 0xad9c64);
                    por(0xb6b7a8, 1);
                }
                gravar_parametro_da_tela(7, 0);
            }
            if marcas() & 0x20 != 0 {
                gravar_parametro_da_tela(1, 3);
                modo = 3;
                assistir();
                por(0xb6b7b0, 1);
            }
            if marcas() & 0x40 != 0 {
                gravar_parametro_da_tela(1, 5);
                modo = 5;
                por(0xb6b7b0, 1);
            }
            por(0xb6b7ac, 1);
        }
        // a marca de "esperando" (parâmetro 0x24): o humano saiu da partida que outros assistem
        let esperando = *quantos != 0 && core::ptr::read_unaligned(espectador()) == 0;
        if esperando == (parametro_da_tela(0x24) == 0) {
            if g(0xb6b7b0) == 0 {
                let mut cor: u32 = 0;
                rotulo(modo as u32, m, BUF, 0x7d0, &mut cor);
                margem(sm, p4, cor);
                texto_do_item(sm, p4, BUF);
                por(0xb6b7a8, 1);
            }
            gravar_parametro_da_tela(0x24, esperando as usize);
        }
        if p15 == 0 {
            let fase = rb(pm + 0x8eb3);
            if fase == 1 || fase == 4 {
                let t = t_do_menu(0x1c);
                if rd(pm + 0x4792) != 0 && t != 0 && parametro_da_tela(0x2f) == 0 {
                    marcar_se_do_humano(t, &mut p15);
                }
                let t = t_do_menu(0x20);
                if rd(pm + 0x4792) != 0 && t != 0 && parametro_da_tela(0x30) == 0 {
                    marcar_se_do_humano(t, &mut p15);
                }
            }
        }
        if p15 != 0 { Some(p15) } else { None }
    } else if id_do_item(item16) == 1 {
        Some(rd(registro(item16) + 0xba9a6))
    } else {
        None
    };

    if let Some(alvo) = alvo {
        if parametro_da_tela(7) == 0 && rb(pm + 0x8eb2) != 0 {
            // abre a tática do time
            let mut lado: u32 = 0;
            if humano_no_time(alvo, 0, 1, 0, 0) != 0 {
                let vaga = || rd((0xb74c26 + h() as isize * 0x300) as usize);
                if usa(alvo) != vaga() {
                    let v = vaga();
                    let u = usa(alvo);
                    let trocar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SCREENMANAGER_SWAP_MANAGER_KEYS);
                    trocar(sm, v, u);
                }
            }
            if (alvo == rd(p + 0x20) && rb(p + 0x44) == 0) || (alvo == rd(p + 0x24) && rb(p + 0x45) == 0) {
                lado = 1;
            }
            gravar_parametro_da_tela(0x15, 0);
            assistir();
            *((ficha(0) + 0xea) as *mut u32) = 0;
            let tatica: unsafe extern "C" fn(usize, u32, usize, u32, i32, u32) = core::mem::transmute(FN_TACTICS_SCREEN_OPEN);
            let fase = rb(pm + 0x8eb3);
            if (fase == 1 || fase == 4) && humano_no_time(alvo, 0, 1, 0, 0) != 0 && indice(sm, usa(alvo)) == h() as u16 {
                let k = if alvo == t_do_menu(0x1c) { 0x2f } else { 0x30 };
                gravar_parametro_da_tela(k, 1);
                tatica(alvo, lado, pm, 0, -1, 1);
                return -4;
            }
            let estado = rb(p + 0x67);
            if estado == 0xff || estado == 1 || estado == 4 {
                tatica(alvo, lado, pm, 1, -1, 0);
            } else {
                tatica(alvo, lado, pm, 0, -1, 0);
            }
            return -4;
        }
        if alvo != p15 {
            if g(0xb6b7b0) == 0 {
                margem(sm, p5, if t_do_menu(0x1c) == alvo { 0x1061 } else { 0x1021 });
                margem(sm, p6, if t_do_menu(0x20) == alvo { 0x1061 } else { 0x1021 });
                por(0xb6b7a8, 1);
            }
            gravar_parametro_da_tela(0x15, alvo);
        }
    }

    if item16 != -1 {
        if id_do_item(item16) == 2 {
            match modo {
                2 | 3 => {
                    gravar_parametro_da_tela(1, 4);
                    modo = 4;
                    deixar_de_assistir();
                    *((p + 0x67) as *mut u8) = 1;
                    por(0xb6b7b0, 1);
                }
                4 => {
                    if core::ptr::read_unaligned(espectador()) == 0 {
                        core::ptr::write_unaligned(espectador(), 1);
                        *quantos = (*quantos).wrapping_add(1);
                    } else {
                        core::ptr::write_unaligned(espectador(), 0);
                        *quantos = (*quantos).wrapping_sub(1);
                    }
                    por(0xb6b7b0, 1);
                }
                5 => {
                    parar(sm, -1);
                    return -9;
                }
                _ => {}
            }
        }
        if id_do_item(item16) == 3 && rd(registro(item16) + 0xba9a6) != parametro_da_tela(2) {
            gravar_parametro_da_tela(2, rd(registro(item16) + 0xba9a6));
            por(0xb6b7b0, 1);
        }
    }

    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
    if ligacao(item) == -4 {
        if core::ptr::read_unaligned(espectador()) == 0 {
            core::ptr::write_unaligned(espectador(), -911);
            *quantos = (*quantos).wrapping_add(1);
        }
        *((ficha(0) + 0xea) as *mut u32) = 0;
        return -4;
    }
    // a original lê o parâmetro 0 antes de olhar [0xb6b7ac]
    let menu_da_partida = |bit: u8| {
        let q = parametro_da_tela(0);
        g(0xb6b7ac) != 0 && rb(rd(q + 0x5b)) & bit != 0
    };
    let refazer = |f: usize| {
        let r: unsafe extern "C" fn() = core::mem::transmute(f);
        r();
        por(0xb6b7a8, 1);
    };
    match aba as i8 {
        1 => {
            let q = parametro_da_tela(0);
            if g(0xb6b7ac) != 0 {
                let f = rd(q + 0x5b);
                if rb(f) & 0x20 != 0 {
                    por(0xb6b7b0, 1);
                }
                if rb(f) & 0x10 != 0 {
                    por(0xb6b7b0, 1);
                }
                if rb(f) & 4 != 0 {
                    if g(0xb6b7b0) != 0 {
                        return -11;
                    }
                    refazer(0x71d9e0);
                }
            }
        }
        2 => {
            if menu_da_partida(4) {
                if g(0xb6b7b0) != 0 {
                    return -11;
                }
                refazer(0x71e750);
            }
        }
        3 => {
            if menu_da_partida(4) && g(0xb6b7b0) == 0 {
                refazer(0x720320);
            }
            if item16 != -1 && id_do_item(item16) == 4 && rd(registro(item16) + 0xba9a6) != parametro_da_tela(0xa) {
                gravar_parametro_da_tela(0xa, rd(registro(item16) + 0xba9a6));
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        4 => {
            if menu_da_partida(0x20) {
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        6 | 9 => {
            let estatisticas_do_time: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_HANDLE_TEAM_STATS_INPUT);
            let r = estatisticas_do_time(item);
            if r != 0 {
                return r;
            }
        }
        7 => {
            if menu_da_partida(4) {
                if g(0xb6b7b0) != 0 {
                    return -11;
                }
                refazer(0x723350);
            }
        }
        8 => {
            let trocar: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MATCH_SCREENS_HANDLE_LATEST_SCORES_CYCLE);
            let r = trocar(item);
            if r != 0 {
                return r;
            }
        }
        10 => {
            let converter: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_AREA_SCROLL_ROW);
            let v = parametro_da_tela(0x26);
            let r = converter(sm, v);
            gravar_parametro_da_tela(0x25, r);
            if modo != 5 && rb(m) & 4 != 0 {
                por(0xb6b7b0, 1);
                return -11;
            }
        }
        _ => {}
    }
    if g(0xb6b7b0) != 0 {
        return -11;
    }
    if g(0xb6b7a8) != 0 {
        let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_REDRAW_SCREEN);
        redesenhar(sm, 0, 0);
    }
    0
}

/// builds the details screen of a stored match: loads the match record and snapshot from the match stats, the team
/// headers, the 7 tabs and the selected tab panel
/// Original: `build_match_details_screen` (0x71b630, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b630 } (cdecl, SEH, match_screens.cpp): monta a tela de detalhes de uma partida guardada:
/// libera a foto do parâmetro 0x1d; sem o registro (parâmetro 0x1f) lê o da partida do parâmetro 0x1e
/// das estatísticas (0x729e30, 0x4f bytes) e sem a foto (0x20) a monta (0x729600, 0xd7a bytes; erro:
/// caixa "Error" e nada mais); o fundo, os cabeçalhos dos dois times (cores 0x53b640, nome, caixa do
/// clube, placar ou "Match Abandoned"), o agregado ([+0x4b] ≥ 0), a barra de menu, as 7 abas, os
/// botões de voltar e avançar e o painel da aba (parâmetro 2).
pub(crate) unsafe extern "C" fn build_match_details_screen() {
    crate::comparacao::ramo("telas montar os detalhes");
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let texto_do_gerente: TextoDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: CaixaDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let cores: unsafe extern "C" fn(usize, usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rb = |a: usize| *(a as *const u8);
    let sx = |a: usize| *(a as *const i8) as i32;

    let mut abas: AbasDaPartida = [0; 0x14b4 / 4];
    construir_abas_da_partida(&mut abas);
    liberar_foto_da_partida();
    let aba = parametro_da_tela(2) as u8;
    let indice = parametro_da_tela(0x1e);
    let mut reg = parametro_da_tela(0x1f);
    let mut foto = parametro_da_tela(0x20);
    let estatisticas = *(G_MATCH_STATS as *const usize);
    if reg == 0 {
        reg = novo(0x4f);
        let ler: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MATCH_STATS_FIND_RECORD_FIXTURE);
        if ler(estatisticas, indice, reg) == 0 {
            erro_das_telas_da_partida(0x90a);
            apagar(reg);
            destruir_abas_da_partida(&mut abas);
            return;
        }
        gravar_parametro_da_tela(0x1f, reg);
    }
    if foto == 0 {
        let f = novo(0xd7a);
        if f != 0 {
            let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
            vetor(f + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        let montar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MATCH_STATS_READ_MATCH_RECORD);
        if montar(*(G_MATCH_STATS as *const usize), indice, f) == 0 {
            erro_das_telas_da_partida(0x91a);
            if f != 0 {
                let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
                fim_do_vetor(f + 0x11f, 0x625, 2, 0x453c10);
                apagar(f);
            }
            destruir_abas_da_partida(&mut abas);
            return;
        }
        gravar_parametro_da_tela(0x20, f);
        foto = f;
    }
    let r = fundo_da_partida(rd(reg + 0x18), rd(reg + 0x14), rd(reg + 0x1c));
    // as cores u16 (o resto das variáveis da original é lixo da pilha, aqui 0)
    let (mut cor_a, mut cor_b) = (0u32, 0u32);
    cores(rd(reg + 0x1c), (r & !0xff) | rb(foto + 0x152) as usize, &mut cor_a, &mut cor_b);
    formatar(BUF, 0x987ff0, rd(reg + 0x1c) + 0x38);
    texto_do_gerente(sm, 0x64, 0xa, 0x1bc, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x64, 0xa, 0x1bc, 0x46, 1, 0, 1, 7, 0, 0, 2, rd(reg + 0x1c));
    let r = if rb(foto + 0x11f) == 0xfd { texto(BUF, 0xa164c4, 0xad9c64) } else { formatar(BUF, 0x98806c, rb(foto + 0x11f) as u32) };
    texto_do_gerente(sm, 0x18f, 0xf, 0x1b7, 0x41, 0x30, cor_em(0xae3184), 0xc, 6, cor_sobre(r as usize, 0xaebdea), BUF, 1);
    // o byte do kit do segundo time vai num ecx sujo (estendido)
    cores(rd(reg + 0x20), rb(foto + 0x777) as usize, &mut cor_a, &mut cor_b);
    formatar(BUF, 0x987ff0, rd(reg + 0x20) + 0x38);
    texto_do_gerente(sm, 0x1be, 0xa, 0x316, 0x46, 0x30, cor_b as usize, 1, 7, cor_a as usize, BUF, 1);
    caixa(sm, 0x1be, 0xa, 0x316, 0x46, 1, 0, 1, 7, 0, 0, 2, rd(reg + 0x20));
    let r = if rb(foto + 0x11f) == 0xfd { texto(BUF, 0xa164c4, 0xad9c64) } else { formatar(BUF, 0x98806c, rb(foto + 0x744) as u32) };
    texto_do_gerente(sm, 0x2e9, 0xf, 0x311, 0x41, 0x30, cor_sobre(r as usize, 0xae3184), 0xc, 6, cor_em(0xaebdea), BUF, 1);
    let agregado = sx(reg + 0x4b);
    if agregado > -1 {
        texto(BUF, 0xa16454, agregado, sx(reg + 0x4c));
        texto_do_gerente(sm, 0x1c3, 0x37, 0x2e4, 0x41, 1, 0, 0x40, 1, cor_a as usize, BUF, 1);
    }
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_MENUBAR_BUILD);
    menu(4, 0);

    texto(titulo_da_aba(&mut abas, 0), 0xa163a8);
    marcar_aba(&mut abas, 0, 1, 1);
    texto(titulo_da_aba(&mut abas, 1), 0xa1639c);
    marcar_aba(&mut abas, 1, 1, 2);
    texto(titulo_da_aba(&mut abas, 2), 0xa1638c);
    marcar_aba(&mut abas, 2, 1, 3);
    texto(titulo_da_aba(&mut abas, 3), 0xa1637c);
    marcar_aba(&mut abas, 3, 1, 4);
    let t1 = rd(reg + 0x1c);
    texto(titulo_da_aba(&mut abas, 4), 0xa16364, sx(t1 + 0x52), sx(t1 + 0x52), t1 + 0x38);
    marcar_aba(&mut abas, 4, 1, 5);
    texto(titulo_da_aba(&mut abas, 5), 0xa16354);
    marcar_aba(&mut abas, 5, 1, 6);
    let t2 = rd(reg + 0x20);
    texto(titulo_da_aba(&mut abas, 6), 0xa16364, sx(t2 + 0x52), sx(t2 + 0x52), t2 + 0x38);
    marcar_aba(&mut abas, 6, 1, 7);
    let (mut topo, mut base) = (0x50i32, 0x221i32);
    let barra: unsafe extern "C" fn(usize, u32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_TAB_BAR);
    let aba = aba as i8 as i32;
    barra(abas.as_mut_ptr() as usize, 7, aba, &mut topo, &mut base, 1);
    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_FORWARD_UNAVAILABLE);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_BACK_UNAVAILABLE);
    let avancar = (sem_avancar(sm) == 0) as u32;
    let voltar = (sem_voltar(sm, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_BACK_NEXT_BUTTONS);
    botoes(voltar, avancar);
    let painel = |f: usize, topo: *mut i32, base: *mut i32| {
        let g: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(f);
        g(topo, base)
    };
    let estatisticas_do_time: unsafe extern "C" fn(u32, *mut i32, *mut i32) = core::mem::transmute(FN_DRAW_TEAM_STATS_PANEL);
    match aba {
        1 => painel(0x71c270, &mut topo, &mut base),
        2 => painel(0x71e040, &mut topo, &mut base),
        3 => painel(0x71f500, &mut topo, &mut base),
        4 => painel(0x721000, &mut topo, &mut base),
        5 => estatisticas_do_time(0, &mut topo, &mut base),
        6 => painel(0x722ed0, &mut topo, &mut base),
        7 => estatisticas_do_time(1, &mut topo, &mut base),
        _ => {}
    }
    destruir_abas_da_partida(&mut abas);
}

/// As montagens da tela da partida são comparadas pela lista de exibição: as regiões da tela, o bloco
/// de parâmetros sem o 0x14 (a hora que o redesenho dos últimos placares 0x724520 grava na aba 8), as
/// marcas 0xb6b7a8/ac/b0, o texto 0xde1f64 e a posição do buffer de rede ([0xae290c]), mais `extra`.
pub(crate) unsafe fn comparar_montagem_da_partida(nome: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = regioes_da_tela();
    reg.push((0xb6b7a8, 12));
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, || crate::aleatorio::sem_contar(rust));
    PLACAR.registrar(igual, || format!("{}: diferencas em {:?}", nome, diferencas_da_tela(&mo, &mr)));
}

/// A tela da partida é comparada pela lista de exibição, com o que ela grava fora da tela: a ficha do
/// humano (+0 e +0xea), a marca dele nos espectadores e a contagem, o foco das velocidades no `Display`
/// do humano (+0x12f4fe) e o volume do narrador (+0x2db0). Só a original quando há o que não roda duas
/// vezes: a foto do parâmetro 0x1d a liberar, o lance do parâmetro 7 a narrar (som, 0x6d7b00) ou as
/// abas 1 (o resumo também narra), 3 (as zonas alocam a foto) e 10 (a tabela ao vivo).
pub(crate) unsafe extern "C" fn comparar_build_match_screen() {
    let o = orig::<unsafe extern "C" fn()>(IDX_BUILD_MATCH_SCREEN);
    let aba = parametro_da_tela(2) as u8;
    // as abas 2 (estatisticas) e 7 (notas) alocam a copia do estado 0x1d quando ela falta: tambem so a original
    if parametro_da_tela(0x1d) != 0 || parametro_da_tela(7) != 0 || matches!(aba, 1 | 2 | 3 | 7 | 10) {
        // ponytail: liberar, narrar com som e alocar não rodam duas vezes; esses casos se provam no modo Rust
        return o();
    }
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let f = ficha(0);
    let mut extra = Vec::new();
    if display_do_humano() != 0 {
        extra.push((display_do_humano() + 0x12f4fe, 3));
    }
    if f != 0 {
        extra.push((f, 1));
        extra.push((f + 0xea, 4));
    }
    let p = parametro_da_tela(0);
    if p != 0 && parametro_da_tela(1) as u8 != 1 {
        let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
        let espectadores = rd(p + 0x5b);
        extra.push(((espectadores as isize + 0xc5 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as usize, 4));
        extra.push((espectadores + 0x108, 1));
        let pm = rd(p + 0x57);
        if pm != 0 && rd(pm + 0x8f7a) != 0 {
            extra.push((rd(pm + 0x8f7a) + 0x2db0, 1));
        }
    }
    comparar_montagem_da_partida("build_match_screen", &extra, || o(), || build_match_screen());
}

// ponytail: a resposta mexe em telas, menu, rede, narração (som) e tática (não roda duas vezes): no modo
// comparação só a original roda; a cópia se prova no jogo do Lucas em modo Rust (contador "telas ...")
pub(crate) unsafe extern "C" fn comparar_handle_match_screen_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_MATCH_SCREEN_INPUT)(item)
}

/// A tela de detalhes é comparada pela lista de exibição quando o registro e a foto da partida já
/// existem (parâmetros 0x1f e 0x20) e não há foto 0x1d a liberar. Só a original quando eles nascem aqui
/// (alocação) ou nas abas 1 (o resumo narra com som), 3 (as zonas alocam a foto) e 4 (o relatório nasce).
pub(crate) unsafe extern "C" fn comparar_build_match_details_screen() {
    let o = orig::<unsafe extern "C" fn()>(IDX_BUILD_MATCH_DETAILS_SCREEN);
    let aba = parametro_da_tela(2) as u8;
    // as abas 2 (estatisticas) e 6 (notas) alocam a copia do estado 0x1d: tambem so a original
    if parametro_da_tela(0x1d) != 0 || parametro_da_tela(0x1f) == 0 || parametro_da_tela(0x20) == 0 || matches!(aba, 1 | 2 | 3 | 4 | 6) {
        // ponytail: alocar, liberar e narrar não rodam duas vezes; esses casos se provam no modo Rust
        return o();
    }
    comparar_montagem_da_partida("build_match_details_screen", &[], || o(), || build_match_details_screen());
}

// ---------- 209a leva: as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo (match_screens.cpp) ----------

/// redraws the overview possession bar when the share changes (percentages and the colour of the club with more of the
/// ball) and stores the match state in the param 0x1d copy
/// Original: `refresh_possession_bar` (0x71d9e0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71d9e0 } (cdecl, match_screens.cpp): na partida (parâmetro 0), sem contest de pênaltis
/// ([estado + 0x53] = 0), pega as cores dos dois clubes (0x53b640 com os uniformes +0x152 e +0x777) e,
/// se a posse (+0x142) mudou desde a cópia do parâmetro 0x1d (ou sem cópia), redesenha a barra da posse
/// da visão geral: o percentual (50 sem valor ou sem a marca +0x767) em "%d%%" nos itens 3 e 4 a partir
/// do parâmetro 0x21, repartidos de 0x115 a 0x2c9 na altura do parâmetro 0x22, e a cor do clube que tem
/// mais posse no item 5. No fim guarda o estado na cópia (que nasce aqui se não existe).
pub(crate) unsafe extern "C" fn refresh_possession_bar() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let retangulo: unsafe extern "thiscall" fn(usize, u32, i32, i32, i32, i32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_RECT);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_COLOUR);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let p0 = parametro_da_tela(0);
    let copia = parametro_da_tela(0x1d);
    let item = parametro_da_tela(0x21) as u32;
    let altura = parametro_da_tela(0x22) as i32;
    if p0 == 0 {
        return;
    }
    let e = core::ptr::read_unaligned((p0 + 0x5f) as *const usize);
    if core::ptr::read_unaligned((e + 0x53) as *const u32) == 0 {
        // as cores: 0x53b640 grava u16; o resto da variável da original é lixo da pilha (0x801040 só lê a palavra)
        let clube = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add(core::ptr::read_unaligned((e + k) as *const usize).wrapping_mul(0x245));
        let (mut c1, mut c1b, mut c2, mut c2b) = (0u32, 0u32, 0u32, 0u32);
        cores_do_clube(clube(0), *((e + 0x152) as *const u8) as u32, &mut c1, &mut c1b);
        cores_do_clube(clube(4), *((e + 0x777) as *const u8) as u32, &mut c2, &mut c2b);
        let v = *((e + 0x142) as *const u8);
        if copia == 0 || v != *((copia + 0x142) as *const u8) {
            let p = if v == 0 || *((e + 0x767) as *const u8) == 0 { 0x32 } else { v as i32 };
            formatar(BUF, FORMATO_PCT, p);
            let x = p * 0x1b4 / 100 + 0x115;
            retangulo(sm, item.wrapping_add(3), 0x115, altura - 0x23, x, altura - 0x14);
            texto(sm, item.wrapping_add(3), BUF);
            formatar(BUF, FORMATO_PCT, 100 - p);
            retangulo(sm, item.wrapping_add(4), x, altura - 0x23, 0x2c9, altura - 0x14);
            texto(sm, item.wrapping_add(4), BUF);
            fundo(sm, item.wrapping_add(5), if p < 0x32 { c2 } else { c1 });
        }
    }
    guardar_estado_da_partida(copia, e);
}

/// A linha (0x49 bytes) do jogador `idx` do lado `lado` no estado da partida `e` (+0x11f + 0x71).
pub(crate) fn linha_do_jogador_na_partida(e: usize, lado: u8, idx: u8) -> usize {
    (e as isize + lado as i8 as isize * 0x625 + idx as i8 as isize * 0x49) as usize
}

/// A visão geral avisa erro (caixa "Error" de match_screens.cpp e segue)? Na partida, com o período
/// [estado + 0x4f] fora dos seis conhecidos; ou com um lance (até 50, até o lado 0xff) de tipo fora de
/// 1 a 8. Nesses casos a cópia e o comparador rodam a original.
pub(crate) unsafe fn visao_geral_avisa_erro() -> bool {
    let p0 = parametro_da_tela(0);
    let e = if p0 != 0 { core::ptr::read_unaligned((p0 + 0x5f) as *const usize) } else { parametro_da_tela(0x20) };
    if p0 != 0 && ![0x1ef, 0x3de, 0, 0x483, 0x528, 1].contains(&core::ptr::read_unaligned((e + 0x4f) as *const u16)) {
        return true;
    }
    for k in 0..50 {
        let ev = e + 0x57 + 4 * k;
        if *(ev as *const u8) == 0xff {
            break;
        }
        if !(1..=8).contains(&*((ev + 2) as *const u8)) {
            return true;
        }
    }
    false
}

/// draws the match overview: period, possession bar and commentary, date, score with half time/extra time/penalties,
/// competition, scorers and cards, shoot-out, referee, attendance, weather and abandoned match
/// Original: `draw_match_overview_panel` (0x71c270, match_screens.cpp, cdecl).
///
/// [pt] { 0x71c270 } (cdecl, match_screens.cpp, ~6 KB): desenha a visão geral da partida de `*topo` a
/// `*base` (`*base` −= 0x28 logo de saída). Na partida (parâmetro 0; estado em +0x5f): o título do
/// período (pelo [estado + 0x4f], com os nomes do parâmetro 1 = 4 ou os outros) e, sem contest de
/// pênaltis ([estado + 0x53] = 0), a barra da posse (moldura, "Last 5 Mins", as cores dos clubes; o item
/// no parâmetro 0x21, a altura no 0x22; `*base` −= 0x37) e a caixa do comentário (com o parâmetro 7 o
/// narrador escreve o último lance nela, 0x6d7b00; o item no parâmetro 8). Fora dela: o estádio e a
/// cidade do parâmetro 0x1f, com o estado do parâmetro 0x20. Depois: a data, o placar (0x4bf930) com o
/// intervalo, a prorrogação e os pênaltis, a competição (com o atalho 0x7e5 quando não é amistoso nem
/// de seleção), a lista dos autores em duas colunas (gols, gol contra, pênalti perdido, expulsão,
/// lesão; os minutos do mesmo jogador e tipo juntos, quebrando a linha com mais de 10 letras), a
/// contest de pênaltis, o árbitro, o público, o tempo, "Match Abandoned" e a barra da posse (0x71d9e0).
pub(crate) unsafe extern "C" fn draw_match_overview_panel(topo: *mut i32, base: *mut i32) {
    if visao_geral_avisa_erro() {
        return orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_OVERVIEW_PANEL)(topo, base);
    }
    let txt: TextoDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: CaixaDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let largura_do_texto: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_FONT_TEXT_WIDTH);
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_FORMAT_PLAYER_NAME);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_OFFICIAL_NAME);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_DATE_GET_LONG_DATE_STRING);
    let placar: unsafe extern "C" fn(usize, u32, usize, u32) = core::mem::transmute(FN_COMP_UTIL_FORMAT_FIXTURE_ROUND_LABEL);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_FORMAT_DESCRIPTION);
    let narrar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_EVENTS_SHOW_EVENT_TEXT);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let barra: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_POSSESSION_BAR);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let b8 = |a: usize| *(a as *const u8);
    let i8_ = |a: usize| *(a as *const i8) as i32;
    let z = |v: i32| v as usize;
    let item16 = |r: usize| r as u16 as i16 as i32 as usize;

    let mut colunas3: [u8; 4] = [1, 1, 1, 0];
    let mut colunas4: [u8; 4] = [2, 1, 2, 1];
    let p0 = parametro_da_tela(0);
    let periodo = parametro_da_tela(1) as u8;
    let base0 = *base;
    *base = base0 - 0x28;
    let (part, e);
    if p0 != 0 {
        *(BUF as *mut u8) = 0;
        e = u(p0 + 0x5f);
        let w = core::ptr::read_unaligned((e + 0x4f) as *const u16);
        let rotulo = if periodo == 4 {
            match w {
                0x1ef => 0xa167ec,
                0x3de => 0xa167e0,
                0 | 1 => 0xa167b8,
                0x483 => 0xa167a0,
                0x528 => 0xa16788,
                _ => 0,
            }
        } else {
            match w {
                0x1ef => 0xa1677c,
                0x3de => 0xa16770,
                0 | 1 => 0xa167b8,
                0x483 => 0xa1675c,
                0x528 => 0xa16744,
                _ => 0,
            }
        };
        match rotulo {
            0 => aviso_em(0xa161d4, if periodo == 4 { 0x1d49 } else { 0x1d5e }),
            0xa167b8 => {
                texto(BUF, rotulo, TEXTO_VAZIO);
            }
            _ => {
                texto(BUF, rotulo);
            }
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
        *topo += 0x28;
        part = p0 + 4;
        if core::ptr::read_unaligned((e + 0x53) as *const u32) == 0 {
            // a barra da posse
            let b = *base;
            lista(display_do_humano(), 0x6e, z(b - 0x36), 0x30c, z(b), 1, 0, 1, 0, 2, 0, MAX);
            // 0x53b640 grava u16; o resto da variável da original é lixo da pilha (só a palavra vale)
            let clube = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add(u(e + k).wrapping_mul(0x245));
            let (mut c68, mut c58, mut c5c, mut c64) = (0u32, 0u32, 0u32, 0u32);
            cores_do_clube(clube(0), b8(e + 0x152) as u32, &mut c68, &mut c58);
            cores_do_clube(clube(4), b8(e + 0x777) as u32, &mut c5c, &mut c64);
            let b = *base;
            let r = txt(sm, 0x96, z(b - 0x2d), 0x2e4, z(b - 0xa), 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), 0, 1);
            gravar_parametro_da_tela(0x21, item16(r));
            texto(BUF, 0xa16738);
            let b = *base;
            txt(sm, 0xaf, z(b - 0x2d), 0xfa, z(b - 0xa), 1, 0, 0xc, 1, cor_em(0xae3184), BUF, 1);
            let b = *base;
            txt(sm, 0x113, z(b - 0x25), 0x2cb, z(b - 0x12), 0x60, cor_em(0xae317e), 0xc, 1, cor_sobre(z(b), 0xae3184), 0, 1);
            txt(sm, 0x115, z(b - 0x23), 0x1ef, z(b - 0x14), 0x10, c58 as usize, 0xc, 1, c68 as usize, 0, 1);
            let r = txt(sm, 0x1ef, z(b - 0x23), 0x2c9, z(b - 0x14), 0x10, c64 as usize, 0xc, 1, c5c as usize, 0, 1);
            linha(display_do_humano(), 1, 0x1ef, z(b - 0x23), 0x1ef, z(b - 0x20), 0, 0, 0x400, cor_sobre(r, 0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, MAX);
            gravar_parametro_da_tela(0x22, z(*base));
            *base -= 0x37;
            // a caixa do comentário
            let t = *topo;
            let y = (*base - t - 0x46) / 2 + t;
            if parametro_da_tela(7) != 0 {
                crate::comparacao::ramo("telas visao geral com o narrador");
                let rel = u(u(p0 + 0x57) + 0x8f7a);
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                let m = u(p0 + 0x57);
                narrar(rel, u(u(p0 + 0x5b) + 1), e, u(u(m + 0x4792)), r, 0x262, 1, 5, 0);
                gravar_parametro_da_tela(8, item16(r));
            } else {
                let r = txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 1, 0, 0x10c, 5, 0, 0, 1);
                gravar_parametro_da_tela(8, item16(r));
            }
        }
    } else {
        let g = parametro_da_tela(0x1f);
        let st = u(g + 0x18);
        if st != 0 {
            let cidade = u(st + 0x38);
            if cidade != 0 {
                let (a, c) = (i8_(st + 0x37), i8_(cidade + city::CITY_GENDER_NAME));
                texto(BUF, 0x9902a4, a, a, c, c, st + 4, cidade + 4);
            } else {
                copiar_para_o_texto(st + 4);
            }
        } else {
            texto(BUF, 0x99c1a4);
        }
        let t = *topo;
        txt(sm, 0x6e, z(t), 0x30c, z(t + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
        *topo += 0x28;
        e = parametro_da_tela(0x20);
        part = g;
    }

    // a data, o placar e a competição
    let mut data = [0u8; 8];
    data_nova(data.as_mut_ptr() as usize, core::ptr::read_unaligned((part + 0x2a) as *const u16) as u32, core::ptr::read_unaligned((part + 0x28) as *const u16) as u32);
    data_texto(data.as_mut_ptr() as usize, BUF, 0x7d0, 0xff);
    let mut w = largura_do_texto(1, BUF) + 0xf;
    if w <= 0x7d {
        w = 0x7d;
    }
    let t = *topo;
    txt(sm, 0x6e, z(t), z(w + 0x6e), z(t + 0x14), 2, 0, 0xc, 1, cor_em(0xaebde8), BUF, 1);
    placar(part, 3, BUF, 0x7d0);
    let mut aux = [0u8; 0x200];
    let pa = aux.as_mut_ptr() as usize;
    if i8_(part + 0x45) >= 0 {
        texto(pa, 0xa16714, i8_(part + 0x45), i8_(part + 0x46));
        acrescentar_ao_texto(aux.as_ptr());
        if i8_(part + 0x47) >= 0 {
            texto(pa, 0xa166d4, i8_(part + 0x43), i8_(part + 0x44), i8_(part + 0x47), i8_(part + 0x48));
            acrescentar_ao_texto(aux.as_ptr());
        }
        let pens = if b8(part + 0x38) & 0x20 != 0 { 0xa166b0 } else { 0xa1668c };
        if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
            texto(pa, pens, b8(e + 0x120) as u32, b8(e + 0x745) as u32);
            acrescentar_ao_texto(aux.as_ptr());
        } else if i8_(part + 0x49) >= 0 {
            texto(pa, pens, i8_(part + 0x49), i8_(part + 0x4a));
            acrescentar_ao_texto(aux.as_ptr());
        }
    }
    let mut x = largura_do_texto(1, BUF) + 0xf;
    if x <= 0x7d {
        x = 0x7d;
    }
    let comp = u(part + 0x14);
    if amistoso(*(G_FRIENDLY as *const usize), comp) == 0 && selecao(*(G_RESERVE_COMP as *const usize), u(part + 0x14)) == 0 {
        let chave = (((*(u(part + 0x14) as *const u32)) << 11 | (core::ptr::read_unaligned((part + 0x32) as *const u16) as u32 & 0x7ff)) << 11)
            | (core::ptr::read_unaligned((part + 0x30) as *const u16) as u32 & 0x7ff);
        let t = *topo;
        caixa(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, cor_sobre(chave as usize, 0xaebde8), BUF, 0x7e5, chave as usize);
    } else {
        let t = *topo;
        txt(sm, z(0x30c - x), z(t), 0x30c, z(t + 0x14), 2, 0, 0xc, 1, cor_em(0xaebde8), BUF, 1);
    }

    // os autores: duas colunas (casa 0, fora 2), uma linha por jogador e tipo com os minutos ao lado
    *topo += 0x19;
    let (ini, fim) = (*topo, *base);
    let a = altura(4);
    let n = ((fim - ini - 0x10) / a) as u8;
    let lst = lista(display_do_humano(), 0x6e, z(ini), 0x30c, z(fim), 4, colunas4.as_mut_ptr() as usize, n as usize, 0, 2, 0, MAX);
    let mut marcas = [0u32; 50];
    let mut linhas = [0i32; 2];
    let mut lado_da_linha = 0usize;
    let mut col: u32;
    // [esp+0x24] da original: as cores fixas trocam só a palavra de baixo, a do verde (0x5e4800) tudo
    let mut cor = 0u32;
    let palavra = |c: u32, a: usize| (c & 0xffff_0000) | *(a as *const u16) as u32;
    let mut nome = [0u8; 200];
    let mut junto = [0u8; 0x200];
    let pj = junto.as_mut_ptr() as usize;
    let pessoas = G_STAFF_TABLE as *const usize;
    // a célula do nome: com o jogador conferido (0x53e990), a célula com o atalho para a pessoa
    // (0x7dc clube de seleção, 0x7d7 senão); senão só o texto
    let celula_do_nome = |fila: usize, col: u32, cor: u32, lado: u8, f198: u32, f19c: u32, f1a0: u32, l: i32| {
        let pid = core::ptr::read_unaligned((fila + 0x190) as *const i32);
        if pid >= 0 && pid < *(G_STAFF_COUNT as *const i32) && confere((*pessoas).wrapping_add(pid as usize * 110), f198, f19c, f1a0, fila + 0x1a4) != 0 {
            let clube = if lado == 0 { u(e) } else { u(e + 4) } as i32;
            let atalho = if clube >= *(G_CLUB_COUNT as *const i32) - 2 * *(G_NATION_COUNT as *const i32) { 0x7dc } else { 0x7d7 };
            celula(sm, lst, col as usize, z(l), 1, 0, 1, 4, cor as usize, BUF, atalho, (*pessoas).wrapping_add(pid as usize * 110));
        } else {
            texto_da_celula(sm, lst, col as usize, z(l), 1, 0, 1, 4, cor as usize, BUF);
        }
        pid
    };
    for i in 1..=50usize {
        let ev = e + 0x57 + 4 * (i - 1);
        let lado = b8(ev);
        if lado == 0xff {
            break;
        }
        if marcas[i - 1] != 0 {
            continue;
        }
        let idx = b8(ev + 1);
        let fila = linha_do_jogador_na_partida(e, lado, idx);
        let (f198, f19c, f1a0) = (u(fila + 0x198) as u32, u(fila + 0x19c) as u32, u(fila + 0x1a0) as u32);
        let tipo = b8(ev + 2);
        match tipo {
            1..=6 => {
                // o gol contra vai para o outro lado
                let outro = (tipo == 2) != (lado != 0);
                col = if outro { 2 } else { 0 };
                lado_da_linha = outro as usize;
                nome_do_jogador(e, idx as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
                let np = nome.as_ptr();
                match tipo {
                    1 | 3 => {
                        formatar(BUF, 0x987ff0, np);
                        cor = palavra(cor, 0xae3184);
                    }
                    2 => {
                        texto(BUF, 0xa16670, 0x64, np);
                        cor = palavra(cor, 0xaebdd4);
                    }
                    5 => {
                        texto(BUF, 0xa16630, 0x64, np);
                        cor = palavra(cor, 0xae31a8);
                    }
                    _ => {
                        texto(BUF, if tipo == 4 { 0xa16650 } else { 0xa16610 }, 0x64, np);
                        cor = rgb(0, 0xc0, 0, 0);
                    }
                }
            }
            7 | 8 => continue,
            _ => {
                aviso_em(0xa161d4, 0xc04);
                continue;
            }
        }
        let pid = celula_do_nome(fila, col, cor, lado, f198, f19c, f1a0, linhas[lado_da_linha]);
        let minuto = |ev: usize| {
            let m = b8(ev + 3) as u32 + 1;
            if b8(ev + 2) == 3 {
                texto(BUF, 0xa165fc, m);
            } else {
                formatar(BUF, FORMATO_D, m);
            }
        };
        minuto(ev);
        if i < 50 {
            for j in i..50 {
                let evj = e + 0x57 + 4 * j;
                let lj = b8(evj);
                if lj == 0xff {
                    break;
                }
                if marcas[j] != 0 {
                    continue;
                }
                if core::ptr::read_unaligned((linha_do_jogador_na_partida(e, lj, b8(evj + 1)) + 0x190) as *const i32) != pid {
                    continue;
                }
                let tj = b8(evj + 2);
                if tj != tipo && !((tj == 1 || tj == 3) && (tipo == 1 || tipo == 3)) {
                    continue;
                }
                let mj = b8(evj + 3) as u32 + 1;
                if tj == 3 {
                    texto(pj, 0xa165d8, BUF, mj);
                } else {
                    formatar(pj, 0xa165d0, BUF, mj);
                }
                if junto.iter().position(|&c| c == 0).unwrap_or(0) > 10 {
                    acrescentar_ao_texto(0xa165cc as *const u8);
                    let l = linhas[lado_da_linha];
                    texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, cor_em(0xaebdec), BUF);
                    linhas[lado_da_linha] = l + 1;
                    minuto(evj);
                } else {
                    copiar_para_o_texto(pj);
                }
                marcas[j] = 1;
            }
        }
        let l = linhas[lado_da_linha];
        texto_da_celula(sm, lst, col as usize + 1, z(l), 1, 0, 1, 4, cor_em(0xaebdec), BUF);
        linhas[lado_da_linha] = l + 1;
        marcas[i - 1] = 1;
    }

    // a contest de pênaltis
    if core::ptr::read_unaligned((e + 0x53) as *const u32) != 0 {
        let m = if linhas[0] > linhas[1] { linhas[0] } else { linhas[1] };
        linhas[lado_da_linha] = m;
        texto_da_celula(sm, lst, 0, z(m), 1, 0, 1, 3, cor_em(0xaebdec), TEXTO_VAZIO);
        let r = m + 1;
        linhas[lado_da_linha] = r;
        texto(BUF, if b8(part + 0x38) & 0x20 != 0 { 0xa165b8 } else { 0xa165a4 });
        let mut w = largura_do_texto(3, BUF) + 0x73;
        if w >= 0x2f1 {
            w = 0x2f1;
        }
        linhas[lado_da_linha] = r + 1;
        // a original empurra edx = o ponteiro da linha com a cor em dx; 0x55f990 só lê a palavra
        linha(display_do_humano(), 1, 0x73, 0, z(w), 0, 0, z(r), 1, 0, 0, 1, 3, cor_em(0xaebdec), BUF, 0, 0, 0, lst);
        linha(display_do_humano(), 1, 0x73, 0, z(w), 0, 0, z(r + 1), 0xc400, cor_em(0xaebdec), 0, 0, 0, 0, 0, 0, 0, 0, lst);
        linhas[lado_da_linha] = r + 2;
        linhas[0] = r + 2;
        linhas[1] = linhas[lado_da_linha];
        for k in 0..50 {
            let ev = e + 0x57 + 4 * k;
            let lado = b8(ev);
            if lado == 0xff {
                break;
            }
            let idx = b8(ev + 1);
            let fila = linha_do_jogador_na_partida(e, lado, idx);
            let (f198, f19c, f1a0) = (u(fila + 0x198) as u32, u(fila + 0x19c) as u32, u(fila + 0x1a0) as u32);
            let tipo = b8(ev + 2);
            if tipo != 7 && tipo != 8 {
                continue;
            }
            col = if lado != 0 { 2 } else { 0 };
            lado_da_linha = (lado != 0) as usize;
            nome_do_jogador(e, idx as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
            if tipo == 7 {
                texto(BUF, 0xa16588, nome.as_ptr());
                cor = palavra(cor, 0xaebde8);
            } else {
                texto(BUF, 0xa1656c, nome.as_ptr());
                cor = rgb(0, 0xc0, 0, 0);
            }
            let l = linhas[lado_da_linha];
            celula_do_nome(fila, col, cor, lado, f198, f19c, f1a0, l);
            linhas[lado_da_linha] = l + 1;
        }
    }

    // o árbitro, o público e o tempo
    let base23 = base0 - 0x23;
    lista(display_do_humano(), 0x6e, z(base23), 0x30c, z(base0), 1, 0, 1, 0, 2, 0, MAX);
    let l2 = lista(display_do_humano(), 0x78, z(base23), 0x302, z(base0), 3, colunas3.as_mut_ptr() as usize, 1, 0, 1, 0, MAX);
    let arbitro = core::ptr::read_unaligned((part + 0x2e) as *const i16);
    if arbitro > -1 {
        nome_do_arbitro((*(G_OFFICIAL_TABLE as *const usize)).wrapping_add(arbitro as usize * 0x2b), aux.as_mut_ptr(), 0xc8, 1);
        texto(BUF, 0xa16550, aux.as_ptr());
    } else {
        texto(BUF, 0xa1653c);
    }
    texto_da_celula(sm, l2, 0, 0, 1, 0, 1, 2, cor_em(0xaebdec), BUF);
    let publico = core::ptr::read_unaligned((e + 0x34) as *const u32);
    if publico != 0 {
        let w = core::ptr::read_unaligned((e + 0x4f) as *const i16);
        if w >= 0x3de || w == 0 || w == 1 {
            texto(BUF, 0xa1651c, publico);
            texto_da_celula(sm, l2, 1, 0, 1, 0, 0xc, 2, cor_em(0xaebde8), BUF);
        }
    }
    clima(*(G_WEATHER as *const usize), pa, e + 0xd76);
    texto(BUF, 0xa164f8, aux.as_ptr());
    texto_da_celula(sm, l2, 2, 0, 1, 0, 0x40, 2, cor_em(0xaebdec), BUF);
    if b8(e + 0x11f) == 0xfd {
        let t = *topo;
        let y = (*base - t - 0x46) / 2 + t;
        texto(BUF, 0xa164e8);
        txt(sm, 0x8c, z(y), 0x2ee, z(y + 0x46), 0x30, cor_em(0xaebdea), 0xc, 3, cor_em(0xae3184), BUF, 1);
    }
    barra();
}

/// draws the live league table (league, top, bottom) on a copy of the league with the day scores applied (or the draw
/// date notice): title, headers, separators, position, up/down arrow, club and columns
/// Original: `match_screens_draw_live_league_table` (0x7280e0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7280e0 } (cdecl, match_screens.cpp, ~3,4 KB; o mapa diz `(void)`, mas a original lê três
/// argumentos: a liga, `topo` e `base`, por valor): desenha a tabela da liga ao vivo. Faz uma cópia
/// local da liga (League 0x682200 + 0x682380, o temporário destruído com 0x682300) e, na partida com
/// placares ao vivo ([[p0+0x57]+0x8ed0]), aplica nela (0x685f60) os jogos do dia do grupo do parâmetro
/// 0xe da competição dele: a partida do humano com o placar do estado (+0x11f/+0x744, pênaltis
/// +0x120/+0x745), as outras sem resultado ainda com 0 a 0, as que já têm placar pela contagem até o
/// minuto (0x728e90); ordena a cópia. Com a data do sorteio ([liga+0xb6], [+0xb8]+[+0x40]) ainda por
/// vir: só o nome da competição e "The draw will be made on ..." (0x5ed070). Senão: o título, a tabela
/// de 11 colunas (as linhas pelas vagas de acesso e rebaixamento ou pelo espaço), os cabeçalhos, a
/// liga com [+0xad] = parâmetro 0x28 ordenada (0x6827d0) para as setas de posição, uma linha por clube
/// (separadores 0x6400, posição, seta de subida/descida, nome com atalho 0x7d5 e cor do humano, e as
/// colunas do registro pelo parâmetro 0x28), a rolagem (parâmetro 0x25) e o item da tabela no 0x26.
/// Sem liga ou sem o parâmetro 0xe (caixa "Error"): a original.
pub(crate) unsafe extern "C" fn match_screens_draw_live_league_table(liga: usize, topo: i32, base: i32) {
    if liga == 0 || parametro_da_tela(0xe) == 0 {
        return orig::<unsafe extern "C" fn(usize, i32, i32)>(IDX_MATCH_SCREENS_DRAW_LIVE_LEAGUE_TABLE)(liga, topo, base);
    }
    let txt: TextoDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let liga_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let liga_fim: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_DESTRUCTOR_LEAGUE);
    let copiar_liga: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_LEAGUE_COPY_FOR_DISPLAY);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let registro: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let aplicar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize, usize) = core::mem::transmute(FN_LEAGUE_APPLY_RESULT_TO_TABLE);
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let contar: unsafe extern "C" fn(usize, u32, usize) = core::mem::transmute(FN_MATCH_SCREENS_TALLY_LIVE_SCORE);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let data_texto: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_DATE_GET_LONG_DATE_STRING);
    let dia_da_semana: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let idioma: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let genero_do_dia: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(FN_DATE_GET_WEEKDAY_GENDER);
    let competicao_curta: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_COMP_UTIL_GET_PARENT_STAGE_ID);
    let nome_da_competicao: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(FN_COMP_UTIL_GET_STAGE_NAME);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(FN_UTILS_FORMAT_ORDINAL);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let rolar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AREA_SET_SCROLL_ROW);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const MAX: usize = usize::MAX;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let w16 = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let b8 = |a: usize| *(a as *const u8);
    let z = |v: i32| v as usize;

    // o quadro da original ([esp+N] depois dos quatro registros): as variáveis nos mesmos lugares
    // (+0x1c colunas, +0x30 data do sorteio, +0x38 hoje / registro da liga, +0x40 e +0x180 jogos,
    // +0x90 a liga local, +0x1d0 a temporária, +0x3d0 a data por extenso, +0xba0 o título)
    let mut quadro = [0u32; 0x1370 / 4];
    let s = quadro.as_mut_ptr() as usize;
    let gravar8 = |a: usize, v: u8| *(a as *mut u8) = v;
    for (k, &v) in [4u8, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4].iter().enumerate() {
        gravar8(s + 0x1c + k, v);
    }
    let local = s + 0x90;
    liga_nova(local);
    copiar_liga(local, s + 0x1d0, liga);
    liga_fim(s + 0x1d0);
    let p0 = parametro_da_tela(0);
    let pe = parametro_da_tela(0xe);
    *((s + 0x14) as *mut usize) = pe;

    // os jogos do dia do grupo na cópia local
    let m = u(p0 + 0x57);
    if m != 0 && w16(m + 0x8ed0) != 0 {
        let mut k = core::ptr::read_unaligned((pe + 4) as *const i32);
        while k <= core::ptr::read_unaligned((pe + 8) as *const i32) {
            let jogo = u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add((k as usize).wrapping_mul(0x69)) + 4;
            if u(pe) == u(jogo) && b8(jogo + 0x43) != 0xfd {
                if b8(jogo + 0x43) == 0xff {
                    let a = s + 0x40;
                    core::ptr::copy_nonoverlapping(jogo as *const u8, a as *mut u8, 0x4f);
                    if u(a + 0xc) == u(p0 + 0x10) {
                        let e = u(p0 + 0x5f);
                        gravar8(a + 0x43, b8(e + 0x11f));
                        gravar8(a + 0x44, b8(e + 0x744));
                        gravar8(a + 0x4b, 0xff);
                        gravar8(a + 0x4c, 0xff);
                        gravar8(a + 0x49, b8(e + 0x120));
                        gravar8(a + 0x4a, b8(e + 0x745));
                    } else {
                        gravar8(a + 0x43, 0);
                        gravar8(a + 0x44, 0);
                        gravar8(a + 0x4b, 0xff);
                        gravar8(a + 0x4c, 0xff);
                        gravar8(a + 0x49, 0xff);
                        gravar8(a + 0x4a, 0xff);
                    }
                    gravar8(a + 0x46, 0);
                    gravar8(a + 0x45, 0);
                    aplicar(local, a, u(local + 0xb1), u(local + 0x3e) as u32, 1, 0, 0);
                } else {
                    let a = s + 0x180;
                    core::ptr::copy_nonoverlapping(jogo as *const u8, a as *mut u8, 0x4f);
                    contar(p0, k as u32, a);
                    aplicar(local, a, u(local + 0xb1), u(local + 0x3e) as u32, 1, 0, 0);
                }
            }
            k += 1;
        }
    }
    ordenar(local, 0);
    parametro_da_tela(0x27);
    let coluna = parametro_da_tela(0x28);
    parametro_da_tela(0x29);

    // o sorteio ainda por vir
    let sorteio = w16(liga + 0xb6);
    if sorteio != 0xffff {
        data_nova(s + 0x30, sorteio as u32, w16(liga + 0xb8).wrapping_add(w16(liga + 0x40)) as u32);
        core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, (s + 0x38) as *mut u8, 8);
        let (hoje, dia) = (w16(s + 0x3a), w16(s + 0x32));
        if hoje < dia || (hoje == dia && (w16(s + 0x38) as i16) < (w16(s + 0x30) as i16)) {
            let mut y = topo;
            let c = core::ptr::read_unaligned((liga + 0x3a) as *const i16);
            if c != 0 {
                let mut k = competicao_curta(c as i32);
                if k == -1 {
                    k = c as i32;
                }
                nome_da_competicao(BUF, 0x7d0, k as u32, 0);
                txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), BUF, 1);
                y = topo + 0x28;
            }
            let a = altura(3);
            let n = ((base - y - 0x10) / a) as u8;
            data_texto(s + 0x30, s + 0x3d0, 0x7d0, 0xff);
            let d = dia_da_semana(s + 0x30);
            let g = genero_do_dia(idioma(), d);
            texto(BUF, 0x99c2e8, g as u8 as i8 as i32, s + 0x3d0);
            mensagem(0x6e, z(y), 0x30c, z(base), 2, 0, n as usize, 3, cor_em(0xae3184), BUF, 0);
            liga_fim(local);
            return;
        }
    }

    // o título e a tabela
    let titulo = s + 0xba0;
    gravar8(s + 0x3d0, 0);
    let c = w16(liga + 0x3a);
    if c != 0 {
        nome_da_competicao(titulo, 0x7d0, c as u32, 0);
    } else {
        texto(titulo, 0xa17b74);
    }
    txt(sm, 0x6e, z(topo), 0x30c, z(topo + 0x23), 2, 0, 0xc, 3, cor_em(0xaebdec), titulo, 1);
    let y = topo + 0x28;
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let (alto, n);
    if clubes <= 8 && core::ptr::read_unaligned((liga + 0x3c) as *const i16) > 0 && u(u(liga + 4)) != u(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        let mut k = (clubes as u8).wrapping_add(1);
        if b8(liga + 0xbe) != 0 || b8(liga + 0xd9) & 1 != 0 {
            k = k.wrapping_add(1);
        }
        for o in [0xbf, 0xc0, 0xc1] {
            if b8(liga + o) != 0 {
                k = k.wrapping_add(1);
            }
        }
        n = k;
        alto = altura(3) * n as i32 + 0x10;
    } else {
        alto = base - y;
        let a = altura(3);
        n = ((alto - 0x10) / a) as u8;
    }
    let lst = lista(display_do_humano(), 0x6e, z(y), 0x30c, z(alto + y), 0xb, s + 0x1c, n as usize, 0, 2, 0, MAX);
    let cabecalho = |col: usize| {
        texto_da_celula(sm, lst, col, 0, 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), BUF);
    };
    texto(BUF, 0x990b3c);
    cabecalho(4);
    texto(BUF, 0x990b1c, TEXTO_VAZIO);
    cabecalho(5);
    let regra = w16(liga + 0xab) as u8;
    texto(BUF, if regra & 0x20 != 0 { 0x990b18 } else if regra & 1 != 0 { 0x99c24c } else { 0x990b14 });
    cabecalho(6);
    texto(BUF, 0x990af4, TEXTO_VAZIO);
    cabecalho(7);
    texto(BUF, 0x990af0);
    cabecalho(8);
    texto(BUF, 0x990aec);
    cabecalho(9);
    texto(BUF, 0x990ae8);
    cabecalho(0xa);

    // a liga de verdade ordenada pela coluna do parâmetro 0x28, para as setas de posição
    *((liga + 0xad) as *mut usize) = coluna;
    ordenar(liga, 0);
    let mut fila = 1usize;
    let c8 = |o: usize| b8(local + o) as i8 as i32;
    let mut i = 0i32;
    while i < core::ptr::read_unaligned((local + 0x3e) as *const i16) as i32 {
        let total = core::ptr::read_unaligned((local + 0x3e) as *const i16) as i32;
        // os separadores: vagas de cima (+0xbe, ou a 1 com +0xd9 & 1), de playoff (+0xbf), de baixo (+0xc0, +0xc1)
        let (v0, v1, v2, v3) = (c8(0xbe), c8(0xbf), c8(0xc0), c8(0xc1));
        let separador = (|| {
            if v0 == 0 {
                if b8(local + 0xd9) & 1 != 0 && i == 1 {
                    return Some(0xaebdec);
                }
            } else if v0 > 0 && i == v0 {
                return Some(0xaebdec);
            }
            if v1 > 0 && i == v1 + v0 {
                return Some(0xaebdd4);
            }
            if v2 > 0 && i == total - v3 - v2 {
                return Some(0xaebdd4);
            }
            if v3 > 0 && i == total - v3 {
                return Some(0xaebdec);
            }
            None
        })();
        if let Some(a) = separador {
            // [esp+0x14] da original: a cor na palavra de baixo, o parâmetro 0xe na de cima
            *((s + 0x14) as *mut u16) = *(a as *const u16);
            let x2 = if total as i16 <= n as i16 { 0x30a } else { 0x2f4 };
            linha(display_do_humano(), 1, 0x70, 0, x2, 0, 0, fila, 0x6400, u(s + 0x14), 0, 0, 0, 0, 0, 0, 0, 0, lst);
            fila += 1;
        }
        let r = registro(local, i);
        let clube = u(r);
        let nl = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
        if nl > 0 {
            let mut j = 0;
            loop {
                let x = registro(liga, j);
                *((s + 0x38) as *mut usize) = x;
                if clube == u(x) {
                    break;
                }
                j += 1;
                if j >= core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32 {
                    break;
                }
            }
        }
        let pos = por_extenso(b8(r + 4) as i8 as i32 + 1, 3);
        let ret = if core::ptr::read_unaligned((liga + 0x3c) as *const i16) != 0 {
            celula(sm, lst, 0, fila, 0x30, cor_sobre(pos, 0xaebe04), 0xc, 1, cor_em(0xae3184), pos, 7, r)
        } else {
            texto_da_celula(sm, lst, 0, fila, 0x30, cor_sobre(pos, 0xaebe04), 0xc, 1, cor_em(0xae3184), pos)
        };
        let x = u(s + 0x38);
        *(BUF as *mut u8) = 0;
        let (agora, antes) = (b8(r + 4) as i8, b8(x + 4) as i8);
        if agora < antes {
            linha(display_do_humano(), 0x20001, 0, 0, 0, 0, 1, fila, 1, cor_sobre(ret, 0xaebdcc), 0, 0xc, 0, cor_em(0xaebdec), 0, 0, 0, 0, lst);
        } else if agora > antes {
            linha(display_do_humano(), 0x40001, 0, 0, 0, 0, 1, fila, 1, cor_sobre(ret, 0xaebdcc), 0, 0xc, 0, cor_em(0xae31a8), 0, 0, 0, 0, lst);
        }
        let cor = if humano(clube, 0, 1, 0, 0) != 0 { cor_em(0xaebdec) } else { cor_em(0xae3184) };
        celula(sm, lst, 2, fila, 1, 0, 1, 3, cor, clube + club::SHORT_NAME, 0x7d5, clube);
        let d = r + coluna.wrapping_mul(14);
        for (o, col) in [(0x11usize, 4usize), (0x12, 5), (0x13, 6), (0x14, 7)] {
            formatar(BUF, FORMATO_D, b8(d + o) as i8 as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, cor_em(0xae3184), BUF);
        }
        for (o, col) in [(0x15usize, 8usize), (0x17, 9)] {
            formatar(BUF, FORMATO_D, core::ptr::read_unaligned((d + o) as *const i16) as i32);
            texto_da_celula(sm, lst, col, fila, 1, 0, 0xc, 2, cor_em(0xae3184), BUF);
        }
        formatar(BUF, FORMATO_D, core::ptr::read_unaligned((d + 0x19) as *const i16) as i32);
        texto_da_celula(sm, lst, 0xa, fila, 0x30, cor_em(0xae3192), 0xc, 1, cor_em(0xae3184), BUF);
        fila += 1;
        i += 1;
    }

    let rolagem = parametro_da_tela(0x25);
    let disp = display_do_humano();
    let id = lst as u16 as i16;
    if id != -1 {
        rolar(disp.wrapping_add((id as isize * 0xbf1) as usize) + 4, rolagem);
    }
    gravar_parametro_da_tela(0x26, id as i32 as usize);
    let t = u(local + 0xb1);
    if t != 0 {
        liberar_tabela(local, t);
    }
    let v = u(local + 0xba);
    if v != 0 {
        liberar(v);
    }
    liga_fim(local);
}

/// A barra é comparada de verdade quando já existe a cópia do estado (parâmetro 0x1d) e a atualização
/// é local: o `Display` do gerente, os parâmetros, a cópia (0xd7a bytes) e o texto 0xde1f64. Sem a cópia
/// (nasce aqui) ou pela rede: só a original.
pub(crate) unsafe extern "C" fn comparar_refresh_possession_bar() {
    let o = orig::<unsafe extern "C" fn()>(IDX_REFRESH_POSSESSION_BAR);
    let p0 = parametro_da_tela(0);
    let copia = parametro_da_tela(0x1d);
    let mut reg = match regioes_da_atualizacao(false) {
        Some(r) if p0 == 0 || copia != 0 => r,
        // ponytail: a alocação da cópia e o pacote pela rede não rodam duas vezes; provados no modo Rust
        _ => return o(),
    };
    reg.extend(regioes_dos_parametros());
    if copia != 0 {
        reg.push((copia, 0xd7a));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || refresh_possession_bar());
    PLACAR.registrar(igual, || format!("refresh_possession_bar: diferencas em {:?}", diferencas_da_tela(&mo, &mr)));
}

/// A visão geral é comparada de verdade pela lista de exibição, com `*topo`, `*base` e a barra da posse
/// do fim (o `Display` do gerente e a cópia do estado). Só a original: com o aviso de erro, com o
/// narrador (parâmetro 7: escreve o lance e toca o som), na partida sem a cópia do estado (a barra a
/// aloca) e com a atualização pela rede.
pub(crate) unsafe extern "C" fn comparar_draw_match_overview_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_OVERVIEW_PANEL);
    let p0 = parametro_da_tela(0);
    let foto = parametro_da_tela(0x1d);
    let narrador = p0 != 0
        && core::ptr::read_unaligned((core::ptr::read_unaligned((p0 + 0x5f) as *const usize) + 0x53) as *const u32) == 0
        && parametro_da_tela(7) != 0;
    let reg = regioes_da_atualizacao(false);
    if visao_geral_avisa_erro() || narrador || (p0 != 0 && foto == 0) || reg.is_none() {
        // ponytail: aviso, som, alocação e rede não rodam duas vezes; esses caminhos se provam no modo Rust
        return o(topo, base);
    }
    let mut reg = reg.unwrap_or_default();
    reg.extend(regioes_dos_parametros());
    reg.push((0xde1f64, 0x100));
    reg.push((topo as usize, 4));
    reg.push((base as usize, 4));
    if foto != 0 {
        reg.push((foto, 0xd7a));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(topo, base), || draw_match_overview_panel(topo, base));
    PLACAR.registrar(igual, || format!("draw_match_overview_panel: diferencas em {:?}", diferencas_da_tela(&mo, &mr)));
}

/// A tabela ao vivo é comparada de verdade pela lista de exibição, com a liga (0xee bytes: [+0xad]) e a
/// tabela dela ([+0xb1], 59 bytes por clube), que a rotina reordena. As cópias locais nascem e morrem
/// dentro dela. Sem liga ou sem o parâmetro 0xe (caixa "Error"): só a original.
pub(crate) unsafe extern "C" fn comparar_match_screens_draw_live_league_table(liga: usize, topo: i32, base: i32) {
    let o = orig::<unsafe extern "C" fn(usize, i32, i32)>(IDX_MATCH_SCREENS_DRAW_LIVE_LEAGUE_TABLE);
    if liga == 0 || parametro_da_tela(0xe) == 0 {
        return o(liga, topo, base);
    }
    let mut extra = vec![(liga, 0xee)];
    let tabela = core::ptr::read_unaligned((liga + 0xb1) as *const usize);
    let n = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    if tabela != 0 && n > 0 {
        extra.push((tabela, n as usize * 59));
    }
    comparar_pela_lista_com("match_screens_draw_live_league_table", &extra, || o(liga, topo, base), || match_screens_draw_live_league_table(liga, topo, base));
}

// ---------- 207a leva: as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save (match_screens.cpp, match_stats.cpp) ----------

/// strcpy de C (a original copia com `rep movs`).
pub(crate) unsafe fn escalacao_copiar(d: *mut u8, s: *const u8) {
    let mut i = 0;
    loop {
        let c = *s.add(i);
        *d.add(i) = c;
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// strcat de C.
pub(crate) unsafe fn escalacao_juntar(d: *mut u8, s: *const u8) {
    let mut n = 0;
    while *d.add(n) != 0 {
        n += 1;
    }
    escalacao_copiar(d.add(n), s);
}

/// O subtítulo da conferência ("Team Selection" ou "Please Confirm"): o texto traduzido em 0xde1f64 e
/// 0x803f80 no gerente.
pub(crate) unsafe fn titulo_da_escalacao(t: usize) {
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let subtitulo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_SCREEN_SUBTITLE);
    texto(0xde1f64, t);
    subtitulo(GERENTE_DE_TELAS, 0xde1f64, 1);
}

/// Os nomes dos `n` jogadores de `lista` juntos em 0xde1f64 ("a", "a and b", "a, b, and c" pelos
/// textos traduzidos) e, para cada um, a ligação no Text `texto_obj` (0x4ae8a0, tipo 0x7dc na seleção,
/// senão 0x7d7).
pub(crate) unsafe fn nomes_da_escalacao(texto_obj: usize, lista: &[usize], n: i8, clube: usize) {
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let ligacao: unsafe extern "thiscall" fn(usize, *const u8, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let mut nome = [0u8; 0xc8];
    let mut juntos = [0u8; 0x7d0];
    let mut k: i8 = 0;
    while k < n {
        let p = lista[k as usize];
        nome_do(p, nome.as_mut_ptr(), 0xc8, 1);
        if k == 0 {
            escalacao_copiar(juntos.as_mut_ptr(), nome.as_ptr());
        } else {
            let modelo: usize = if k as i32 == n as i32 - 1 { if n == 2 { 0xa17b40 } else { 0xa17b18 } } else { 0xa17af4 };
            texto(juntos.as_mut_ptr() as usize, modelo, 0xde1f64usize, nome.as_ptr() as usize);
        }
        escalacao_copiar(0xde1f64 as *mut u8, juntos.as_ptr());
        let tipo = if selecao(clube) != 0 { 0x7dc } else { 0x7d7 };
        ligacao(texto_obj, nome.as_ptr(), tipo, p);
        k += 1;
    }
}

/// O aviso da conferência: a mensagem `msg` no Text (0x4ae660) numa área (0x5ed200) e o botão "Ok"
/// (0x55f990). `caixa_sobre`/`ok_sobre`: onde a original empurra a cor da caixa / a cor do texto do
/// botão com os 16 bits de cima da devolução da chamada anterior (varia por ramo).
pub(crate) unsafe fn aviso_da_escalacao(texto_obj: usize, msg: *const u8, caixa_sobre: bool, ok_sobre: bool) {
    let definir: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(FN_TEXT_SET_STRING);
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA_FROM_TEXT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let r = definir(texto_obj, msg);
    let cor = if caixa_sobre { cor_sobre(r, 0xae3184) } else { cor_em(0xae3184) };
    area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, cor, texto_obj);
    let r = texto(0xde1f64, 0x990f7c) as usize;
    let cor_do_texto = if ok_sobre { cor_sobre(r, 0xae3184) } else { cor_em(0xae3184) };
    linha(display_do_humano(), 2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, cor_sobre(r, 0xae317e), cor_em(0xae317e), 0xc, 4, cor_do_texto, 0xde1f64, 0, 1, 0, usize::MAX);
}

/// draws the team selection check: warns (with linked names) about players on international duty, on loan, unavailable,
/// ineligible, banned, injured or without a work permit, the bench limit or missing starters; otherwise asks for
/// confirmation (captain, set piece takers, substitutes) with "No" and "Yes"
/// Original: `match_screens_draw_team_selection` (0x7260f0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7260f0 } (cdecl, match_screens.cpp, ~7 KB com os pedaços até 0x727c2e): desenha a conferência
/// da escalação do clube (parâmetro 0) para o jogo (parâmetro 1) e a equipe (parâmetro 2). Passa pelas
/// 20 vagas da tática (0x8968f0): conta starters (< 11), reservas (< 11 + o banco [jogo+0x4d] & 0xf)
/// e os de fora; dos que estão no time, junta os convocados pela seleção (0x7757d0), os emprestados a
/// outro clube (0x4cf6d0/0x4cf5a0, fora da seleção), os inelegíveis com o mesmo motivo do primeiro
/// (0x8cf5e0), os suspensos (0x55d270), os lesionados e os indisponíveis (0x62c400/0x62ed70: lesão de
/// tipo 0x4d a 0x51 no idioma 0 conta como indisponível) e os sem permissão de trabalho (0x8c7560, fora
/// da competição [0x9cf684] e de amistoso). Depois o fundo do clube, o menu, as cores e o título com o
/// nome. O primeiro grupo não vazio, nessa ordem, vira o aviso "Team Selection" com os nomes (ligados)
/// e o botão "Ok"; senão o limite do banco do dia (0x8cf880); senão menos de 11 starters; senão
/// "Please Confirm": o capitão escolhido (0x727e20) quando não há um válido, os cobradores de bola
/// parada fora dos 11, os reservas que faltam, os de fora do grupo, "Proceed to the match ?" e a lista
/// com "No" e "Yes".
pub(crate) unsafe extern "C" fn match_screens_draw_team_selection() {
    crate::comparacao::ramo("telas conferencia da escalacao");
    let criar: unsafe extern "thiscall" fn(*mut usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let destruir: unsafe extern "thiscall" fn(*mut usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let definir: unsafe extern "thiscall" fn(usize, *const u8) -> usize = core::mem::transmute(FN_TEXT_SET_STRING);
    let ligacao: unsafe extern "thiscall" fn(usize, *const u8, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    let escolhido: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let get_player_injury_status: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let sem_permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_BACKGROUND_SET_CLUB_PICTURE);
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_MENUBAR_BUILD);
    let cores: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let nome_da_equipe: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CLUB_GET_NATIONAL_TEAM_NAME);
    let titulo: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SCREENMANAGER_ADD_SCREEN_TITLE);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let limite_do_banco: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    let por_extenso: unsafe extern "C" fn(i32, *mut u8, u32, u32) -> i32 = core::mem::transmute(FN_UTILS_NUMBER_TO_WORDS);
    let vaga_do_capitao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
    let melhor_titular: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT);
    let cobrador: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_TACTICS_GET_SET_PIECE_TAKER);
    let vaga_do_jogador: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_TACTICS_FIND_SELECTION_SLOT);
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let area: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA_FROM_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let g = |a: usize| *(a as *const usize);
    const BUF: usize = 0xde1f64;
    const SEPARADOR: usize = 0x9ed918; // "\n\n"

    let mut obj = [0usize; 2];
    let t = obj.as_mut_ptr();
    let tu = t as usize;
    let mut motivo: u8 = 0;
    criar(t, 0);
    let colunas: [u8; 2] = [3, 1];
    let clube = parametro_da_tela(0);
    let jogo = parametro_da_tela(1);
    let equipe = parametro_da_tela(2);
    let banco = || (*((jogo + 0x4d) as *const u8) & 0xf) as i8;
    let competicao = || core::ptr::read_unaligned((jogo + 0x14) as *const usize);

    let (mut starters, mut reservas, mut fora) = (0i8, 0i8, 0i8);
    let mut convocados = ([0usize; 20], 0i8);
    let mut emprestados = ([0usize; 20], 0i8);
    let mut indisponiveis = ([0usize; 20], 0i8);
    let mut inelegiveis = ([0usize; 20], 0i8);
    let mut suspensos = ([0usize; 20], 0i8);
    let mut lesionados = ([0usize; 20], 0i8);
    let mut sem_visto = ([0usize; 20], 0i8);
    let por = |g: &mut ([usize; 20], i8), p: usize| {
        g.0[g.1 as usize] = p;
        g.1 += 1;
    };
    for vaga in 0..0x14u8 {
        // ponytail: a original empurra a vaga como dword com os 3 bytes de cima sem valor (a variável é um
        // char); a chamada usa o char, aqui vai estendida com zero
        let p = escolhido(g(0xde1d08), clube, vaga as u32, equipe);
        if p == 0 {
            continue;
        }
        let v = vaga as i8;
        if v < 11 {
            starters += 1;
        } else if v < banco() + 0xb {
            reservas += 1;
        } else {
            fora += 1;
        }
        if v >= banco() + 0xb {
            continue;
        }
        if convocado(g(0xb74318), p, 0xae2c90, clube, 0, 0) != 0 {
            por(&mut convocados, p);
        }
        if emprestimo(g(0xae1050), p) != 0
            && contrato(g(0xae1050), p) != 0
            && *((contrato(g(0xae1050), p) + 4) as *const i32) == *(clube as *const i32)
            && selecao(clube) == 0
        {
            por(&mut emprestados, p);
        }
        let m = inelegivel(g(0xde1f54), p, jogo);
        if m != 0 && (inelegiveis.1 == 0 || m == motivo) {
            motivo = m;
            por(&mut inelegiveis, p);
        }
        if suspenso(g(0xae28f0), p, competicao(), 1, 0) != 0 {
            por(&mut suspensos, p);
        }
        let tipo_ruim = |l: usize| l != 0 && (0x4d..=0x51).contains(&*(l as *const i8));
        if get_player_injury_status(g(0xb63d28), p) == 1 {
            let l = lesao(g(0xb63d28), p);
            if tipo_ruim(l) {
                if idioma() == 0 {
                    por(&mut indisponiveis, p);
                } else {
                    por(&mut lesionados, p);
                }
            }
        } else if get_player_injury_status(g(0xb63d28), p) == 2 {
            if idioma() != 0 {
                por(&mut lesionados, p);
            } else if tipo_ruim(lesao(g(0xb63d28), p)) {
                por(&mut indisponiveis, p);
            } else {
                por(&mut lesionados, p);
            }
        }
        if sem_permissao(g(0xde1f54), p, clube, 0) != 0
            && *(competicao() as *const i32) != *(KEY_CLUB_COMP_RESERVE as *const i32)
            && amistoso(g(0xae2c74), competicao()) == 0
        {
            por(&mut sem_visto, p);
        }
    }

    fundo(clube, 0, 0);
    menu(4, 0);
    // 0x53b640 grava u16; o resto da variável da original é lixo da pilha, aqui 0
    let (mut frente, mut fundo_da_cor) = (0u32, 0u32);
    cores(clube, 1, &mut frente, &mut fundo_da_cor);
    nome_da_equipe(clube, equipe, BUF, 0x7d0, 3, 1);
    titulo(GERENTE_DE_TELAS, fundo_da_cor as usize, frente as usize, BUF);

    // ponytail: o motivo de inelegibilidade fora de 1 a 11 deixa a mensagem da original com o que
    // sobrou na pilha; aqui ela começa vazia
    let mut msg = [0u8; 0x2710];
    let m = msg.as_mut_ptr();
    let mu = m as usize;
    let grupo = |g: &([usize; 20], i8), plural: usize, singular: usize, traduzir: bool| {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &g.0, g.1, clube);
        let modelo = if g.1 > 1 { plural } else { singular };
        if traduzir {
            texto(mu, modelo, BUF);
        } else {
            formatar(mu, modelo, BUF);
        }
    };
    if convocados.1 != 0 {
        grupo(&convocados, 0xa17a9c, 0xa17a4c, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if emprestados.1 != 0 {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &emprestados.0, emprestados.1, clube);
        if emprestados.1 > 1 {
            texto(mu, 0xa17a10, BUF);
        } else {
            let c = core::ptr::read_unaligned((emprestados.0[0] + 0x39) as *const usize);
            let genero = *((c + 0x52) as *const i8) as i32;
            texto(mu, 0xa179c8, genero, genero, BUF, c + 0x38);
        }
        aviso_da_escalacao(tu, m, false, true);
    } else if indisponiveis.1 != 0 {
        grupo(&indisponiveis, 0xa1799c, 0xa17970, false);
        aviso_da_escalacao(tu, m, false, false);
    } else if inelegiveis.1 != 0 {
        titulo_da_escalacao(0xa17b64);
        nomes_da_escalacao(tu, &inelegiveis.0, inelegiveis.1, clube);
        const PLURAL: [usize; 11] = [0xa1793c, 0xa17908, 0xa178c0, 0xa17878, 0xa1782c, 0xa177dc, 0xa1778c, 0xa1773c, 0xa176f4, 0xa17694, 0xa1763c];
        const SINGULAR: [usize; 11] = [0xa17610, 0xa175e0, 0xa175a0, 0xa1755c, 0xa17514, 0xa174cc, 0xa17484, 0xa1743c, 0xa173fc, 0xa173a8, 0xa17358];
        if (1..=11).contains(&motivo) {
            let modelo = if inelegiveis.1 > 1 { PLURAL[motivo as usize - 1] } else { SINGULAR[motivo as usize - 1] };
            texto(mu, modelo, BUF);
        }
        aviso_da_escalacao(tu, m, true, false);
    } else if suspensos.1 != 0 {
        grupo(&suspensos, 0xa17328, 0xa17300, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if lesionados.1 != 0 {
        grupo(&lesionados, 0xa172b8, 0xa17278, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if sem_visto.1 != 0 {
        grupo(&sem_visto, 0xa17224, 0xa171d4, true);
        aviso_da_escalacao(tu, m, false, false);
    } else if limite_do_banco(g(0xde1f54), clube, jogo, m, 0, 2, 0) != 0 {
        titulo_da_escalacao(0xa17b64);
        aviso_da_escalacao(tu, m, true, false);
    } else if starters < 11 {
        titulo_da_escalacao(0xa17b64);
        let (mut maximo, mut quantos) = ([0u8; 0xc8], [0u8; 0xc8]);
        por_extenso((banco() + 0xb) as i32, maximo.as_mut_ptr(), 0xc8, 1);
        por_extenso(starters as i32, quantos.as_mut_ptr(), 0xc8, 1);
        match starters {
            0 => texto(mu, 0xa17150, maximo.as_ptr()),
            1 => texto(mu, 0xa170cc, maximo.as_ptr()),
            _ => texto(mu, 0xa17038, quantos.as_ptr(), maximo.as_ptr()),
        };
        aviso_da_escalacao(tu, m, false, true);
    } else {
        titulo_da_escalacao(0x9909b8);
        *m = 0;
        let acrescentar = |s: usize| {
            if *m != 0 {
                escalacao_juntar(m, SEPARADOR as *const u8);
            }
            escalacao_juntar(m, s as *const u8);
        };
        let mut nome = [0u8; 0xc8];
        let mut outro = [0u8; 0xc8];
        // o capitão: sem vaga de capitão (negativa) ou fora dos 11, o melhor titular (0x727e20)
        let c = vaga_do_capitao(g(0xde1d08), clube, equipe, 1) as i8;
        if c < 0 || vaga_do_capitao(g(0xde1d08), clube, equipe, 1) as i8 >= 0xb {
            let s = melhor_titular(clube, equipe) as i8;
            if s > -1 {
                // ponytail: como na vaga acima, os 3 bytes de cima do dword empurrado não têm valor
                let p = escolhido(g(0xde1d08), clube, s as u8 as u32, equipe);
                if p != 0 {
                    nome_do(p, nome.as_mut_ptr(), 0xc8, 1);
                    texto(BUF, 0xa16fec, nome.as_ptr());
                    acrescentar(BUF);
                    let tipo = if selecao(clube) != 0 { 0x7dc } else { 0x7d7 };
                    ligacao(tu, nome.as_ptr(), tipo, p);
                }
            }
        }
        // os cobradores de bola parada (5 papéis × 2 escolhas) que não estão nos 11, sem repetir; a escolha
        // vai no byte de baixo do dword da equipe (a original reaproveita a variável)
        let mut cobradores = ([0usize; 20], 0i8);
        for papel in 0..5usize {
            for escolha in 0..2usize {
                let p = cobrador(g(0xde1d08), clube, papel, (equipe & !0xff) | escolha, equipe);
                if p != 0 {
                    let s = vaga_do_jogador(g(0xde1d08), clube, p, equipe) as i8;
                    if !(0..=10).contains(&s) && !cobradores.0[..cobradores.1 as usize].contains(&p) {
                        por(&mut cobradores, p);
                    }
                }
            }
        }
        if cobradores.1 > 0 {
            nomes_da_escalacao(tu, &cobradores.0, cobradores.1, clube);
            let mut aviso = [0u8; 0x7c0];
            texto(aviso.as_mut_ptr() as usize, if cobradores.1 > 1 { 0xa16f8c } else { 0xa16f34 }, BUF);
            acrescentar(aviso.as_ptr() as usize);
        }
        if reservas < banco() {
            por_extenso(banco() as i32, nome.as_mut_ptr(), 0xc8, 1);
            por_extenso(reservas as i32, outro.as_mut_ptr(), 0xc8, 1);
            match reservas {
                0 => texto(BUF, 0xa16eec, nome.as_ptr()),
                1 => texto(BUF, 0xa16ea4, nome.as_ptr()),
                _ => texto(BUF, 0xa16e50, outro.as_ptr(), nome.as_ptr()),
            };
            acrescentar(BUF);
        }
        if fora > 0 {
            por_extenso(fora as i32, nome.as_mut_ptr(), 0xc8, 1);
            if fora == 1 {
                texto(BUF, 0xa16e14);
            } else {
                texto(BUF, 0xa16dcc, nome.as_ptr());
            }
            acrescentar(BUF);
        }
        texto(BUF, 0xa16db4);
        acrescentar(BUF);
        definir(tu, m);
        area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, cor_em(0xae3184), tu);
        let id = lista(display_do_humano(), 0x64, 0x22b, 0x316, 0x24e, 2, colunas.as_ptr() as usize, 1, 0, 1, 0, usize::MAX);
        let r = texto(BUF, 0x98e8cc) as usize;
        celula(GERENTE_DE_TELAS, id, 0, 0, 0x30, cor_sobre(r, 0xae317e), 0xc, 3, cor_em(0xae3184), BUF, 2, 0);
        texto(BUF, 0x98e8d0);
        celula(GERENTE_DE_TELAS, id, 1, 0, 0x30, cor_em(0xae317e), 0xc, 3, cor_em(0xae3184), BUF, 3, 0);
    }
    destruir(t);
}

/// reads the record index, the four bucket cursors and the counts from match_stats.dat in the saved game
/// Original: `Match_Stats::load` (0x72aa80, match_stats.cpp, thiscall).
///
/// [pt] { 0x72aa80 } (thiscall, ret 4, match_stats.cpp): lê do "match_stats.dat" do save (0x9326e0 no zip
/// `zip`) a quantidade em +4, o índice (12 bytes cada, alocado em [this]), os quatro cursores dos
/// baldes (+8), +0x1c e +0x18. Zip nulo: a original (caixa de erro). Sem o arquivo: erro 0x49e; falha
/// de leitura ou de alocação: erro 0x4a5/0x4ae/0x4b6/0x4be/0x4c6/0x4ce e fecha. Devolve 1 se leu.
pub(crate) unsafe extern "thiscall" fn match_screens_load(estat: usize, zip: usize) -> u32 {
    if zip == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_LOAD)(estat, zip);
    }
    crate::comparacao::ramo("telas estatisticas carregar");
    let abrir: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
    let fechar: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let mut h = abrir(zip, 0xa17bd0, 0, 0x16, 4);
    if h == 0 {
        aviso_em(0xa17ba0, 0x49e);
        return 0;
    }
    let falha = |linha: u32, mut h: usize| {
        aviso_em(0xa17ba0, linha);
        fechar(&mut h);
        0
    };
    let n = (estat + 4) as *mut i32;
    if ler(n as usize, 4, 1, h) != 1 {
        return falha(0x4a5, h);
    }
    let tabela = alocar((*n).wrapping_mul(12) as u32);
    *(estat as *mut usize) = tabela;
    if tabela == 0 {
        return falha(0x4ae, h);
    }
    if ler(tabela, 0xc, *n as u32, h) != *n as u32 {
        return falha(0x4b6, h);
    }
    if ler(estat + 8, 4, 4, h) != 4 {
        return falha(0x4be, h);
    }
    if ler(estat + 0x1c, 4, 1, h) != 1 {
        return falha(0x4c6, h);
    }
    if ler(estat + 0x18, 4, 1, h) != 1 {
        return falha(0x4ce, h);
    }
    fechar(&mut h);
    1
}

/// writes the record index, the four bucket cursors and the counts to match_stats.dat in the saved game
/// Original: `Match_Stats::save` (0x72adf0, match_stats.cpp, thiscall).
///
/// [pt] { 0x72adf0 } (thiscall, ret 4, match_stats.cpp): grava no "match_stats.dat" do save (0x9326e0 em
/// escrita) a quantidade (+4), o índice [this], os quatro cursores (+8), +0x1c e +0x18. Zip nulo: a
/// original (caixa de erro); sem abrir, o jogo para (erro 0x4f1); falha de escrita: erro
/// 0x4f8/0x500/0x508/0x510/0x518 e fecha. Devolve 1 se gravou.
pub(crate) unsafe extern "thiscall" fn match_screens_save(estat: usize, zip: usize) -> u32 {
    if zip == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_SAVE)(estat, zip);
    }
    crate::comparacao::ramo("telas estatisticas salvar");
    let abrir: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
    let fechar: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
    let mut h = abrir(zip, 0xa17bd0, 1, 0x16, 4);
    if h == 0 {
        erro_em(0xa17ba0, 0x4f1);
    }
    let falha = |linha: u32, mut h: usize| {
        aviso_em(0xa17ba0, linha);
        fechar(&mut h);
        0
    };
    let n = (estat + 4) as *const i32;
    if escrever(n as usize, 4, 1, h) != 1 {
        return falha(0x4f8, h);
    }
    if escrever(*(estat as *const usize), 0xc, *n as u32, h) != *n as u32 {
        return falha(0x500, h);
    }
    if escrever(estat + 8, 4, 4, h) != 4 {
        return falha(0x508, h);
    }
    if escrever(estat + 0x1c, 4, 1, h) != 1 {
        return falha(0x510, h);
    }
    if escrever(estat + 0x18, 4, 1, h) != 1 {
        return falha(0x518, h);
    }
    fechar(&mut h);
    1
}

/// A conferência é comparada de verdade pela lista de exibição (o Text local é alocado e liberado
/// dentro de cada execução).
pub(crate) unsafe extern "C" fn comparar_match_screens_draw_team_selection() {
    let o = orig::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_DRAW_TEAM_SELECTION);
    comparar_painel_pela_lista("match_screens_draw_team_selection", || o(), || match_screens_draw_team_selection());
}

// ponytail: carregar e salvar mexem no arquivo do save (e carregar aloca o índice): rodar duas vezes
// leria/gravaria duas vezes, então o comparador só chama a original; as cópias se provam no modo Rust.
pub(crate) unsafe extern "thiscall" fn comparar_match_screens_load(estat: usize, zip: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_LOAD)(estat, zip)
}

pub(crate) unsafe extern "thiscall" fn comparar_match_screens_save(estat: usize, zip: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MATCH_SCREENS_SAVE)(estat, zip)
}

// ---------- 206a leva: as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida (match_stats.cpp) ----------

/// Os registros de estatística da partida têm 0xd7a bytes no "match_stats_matches.tmp" de CM3_TEMP.
pub(crate) const REGISTRO_DA_PARTIDA: u32 = 0xd7a;

/// reads one match statistics record (0xd7a bytes) from match_stats_matches.tmp by index
/// Original: `Match_Stats::read_match_record` (0x729600, match_stats.cpp, thiscall).
///
/// [pt] { 0x729600 } (thiscall, ret 8, match_stats.cpp): lê o registro `k` (0xd7a bytes, na posição
/// k × 0xd7a) do "match_stats_matches.tmp" (CM3_TEMP, "rb") para `saida`. Índice −1: devolve 0 calado;
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro). Sem abrir: erro 0x128; fseek ou
/// fread falham: erro 0x12f/0x137 e fecha. Devolve 1 se leu.
pub(crate) unsafe extern "thiscall" fn read_match_record(estat: usize, k: i32, saida: usize) -> u32 {
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_READ_MATCH_RECORD)(estat, k, saida);
    }
    crate::comparacao::ramo("estatisticas ler registro");
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x98f928);
    if f == 0 {
        aviso_em(0xa17ba0, 0x128);
        return 0;
    }
    if buscar(f, k.wrapping_mul(REGISTRO_DA_PARTIDA as i32), 0) != 0 {
        aviso_em(0xa17ba0, 0x12f);
        fechar(f);
        return 0;
    }
    if ler(saida, REGISTRO_DA_PARTIDA, 1, f) != 1 {
        aviso_em(0xa17ba0, 0x137);
        fechar(f);
        return 0;
    }
    fechar(f);
    1
}

/// fills the record of a finished match and writes it, except for excluded matches
/// Original: `Match_Stats::record_match` (0x7293f0, match_stats.cpp, thiscall).
///
/// [pt] { 0x7293f0 } (thiscall, ret 8, match_stats.cpp): registra a partida `m` que terminou: preenche o
/// registro `r` (0x729ff0, sem pular a renda) e, salvo jogo da competição [0x9cf684] com +0x4c no
/// registro, grava (0x729820) com o jogo [m+0x4792] e os eventos [m+0x8f7a]. Partida ou registro
/// nulos: a original (caixa de erro, devolve 0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn match_screens_record_match(estat: usize, m: usize, r: usize) -> u32 {
    if m == 0 || r == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_MATCH_SCREENS_RECORD_MATCH)(estat, m, r);
    }
    crate::comparacao::ramo("estatisticas registrar partida");
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_MATCH_STATS_FILL_MATCH_RECORD);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_MATCH_STATS_WRITE_MATCH_RECORD);
    preencher(estat, m, r, 0);
    let jogo = core::ptr::read_unaligned((m + 0x4792) as *const usize);
    if *(jogo as *const i32) != *(KEY_CLUB_COMP_RESERVE as *const i32) || *((r + 0x4c) as *const u8) == 0 {
        gravar(estat, jogo, r, core::ptr::read_unaligned((m + 0x8f7a) as *const usize));
    }
    1
}

/// writes the events and the record into the fixture's slot (or the bucket cursor's slot, which wraps around) and
/// updates the index
/// Original: `Match_Stats::write_match_record` (0x729820, match_stats.cpp, thiscall).
///
/// [pt] { 0x729820 } (thiscall, ret 0xc, match_stats.cpp): grava o registro `r` do jogo `jogo` no
/// "match_stats_matches.tmp" ("r+b"): zera os 13 bytes de +0xd69, grava os eventos (0x6d98b0 de
/// `eventos`, humano se um dos clubes +0x1c/+0x20 é de humano, 0x600a50) e escreve o registro na vaga
/// [jogo+8]; sem vaga (−1), na vaga do cursor do balde (0x729bd0) em [this+8+4b], que anda e volta ao
/// começo do balde ao chegar ao fim ((b+1) × [this+0x1c] / 4). No fim, a entrada [jogo+8] do índice
/// (12 bytes em [this]) recebe +0xc, +0x10 e as datas +0x2a/+0x28. Argumento nulo: a original (caixa
/// de erro). Falhas do arquivo: erro 0x172/0x17f/0x18d/0x1a3 (fechando, como a original). Devolve 1.
pub(crate) unsafe extern "thiscall" fn write_match_record(estat: usize, jogo: usize, r: usize, eventos: usize) -> u32 {
    if jogo == 0 || r == 0 || eventos == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD)(estat, jogo, r, eventos);
    }
    crate::comparacao::ramo("estatisticas gravar registro");
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let record_events: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_MATCH_EVENTS_SAVE_EVENT_LOG);
    let balde: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_MATCH_STATS_GET_RECORD_BUCKET);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let d = |o: usize| (estat + o) as *mut i32;
    let j = |o: usize| (jogo + o) as *mut i32;

    let desc = r + 0xd69;
    core::ptr::write_bytes(desc as *mut u8, 0, 13);
    let h = (humano(*((jogo + 0x1c) as *const usize), 1, 1, 0, 0) != 0 || humano(*((jogo + 0x20) as *const usize), 1, 1, 0, 0) != 0) as u32;
    record_events(eventos, desc, h);
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x9c7848);
    if f == 0 {
        aviso_em(0xa17ba0, 0x172);
        return 0;
    }
    let vaga = *j(8);
    if vaga != -1 {
        if buscar(f, vaga.wrapping_mul(REGISTRO_DA_PARTIDA as i32), 0) != 0 {
            aviso_em(0xa17ba0, 0x17f);
            fechar(f);
            return 0;
        }
    } else {
        let b = balde(estat, jogo, r) as i8 as isize;
        let cursor = d(8 + 4 * b as usize);
        if buscar(f, (*cursor).wrapping_mul(REGISTRO_DA_PARTIDA as i32), 0) != 0 {
            aviso_em(0xa17ba0, 0x18d);
            fechar(f);
            return 0;
        }
        *j(8) = *cursor;
        let c = (*cursor).wrapping_add(1);
        *cursor = c;
        let total = *d(0x1c);
        if c == (b as i32 + 1).wrapping_mul(total) / 4 {
            *cursor = c - total / 4;
        }
    }
    if escrever(r, REGISTRO_DA_PARTIDA, 1, f) != 1 {
        aviso_em(0xa17ba0, 0x1a3);
        fechar(f);
        return 0;
    }
    fechar(f);
    let e = (*(estat as *const usize) as isize + *j(8) as isize * 12) as usize;
    *(e as *mut i32) = *j(0xc);
    *((e + 4) as *mut i32) = *j(0x10);
    *((e + 8) as *mut u16) = *((jogo + 0x2a) as *const u16);
    *((e + 0xa) as *mut u16) = *((jogo + 0x28) as *const u16);
    1
}

// ponytail: as três mexem no arquivo temporário (e a gravação também nos eventos e no índice): rodar
// duas vezes gravaria o registro duas vezes, então o comparador só chama a original; as cópias se
// provam no modo Rust (os ramos) e pela carreira com as mutações.
pub(crate) unsafe extern "thiscall" fn comparar_read_match_record(estat: usize, k: i32, saida: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_READ_MATCH_RECORD)(estat, k, saida)
}

pub(crate) unsafe extern "thiscall" fn comparar_match_screens_record_match(estat: usize, m: usize, r: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_MATCH_SCREENS_RECORD_MATCH)(estat, m, r)
}

pub(crate) unsafe extern "thiscall" fn comparar_write_match_record(estat: usize, jogo: usize, r: usize, eventos: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD)(estat, jogo, r, eventos)
}

// ---------- 205a leva: as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores (match_screens.cpp) ----------

/// Cor (16 bits) guardada em `a`. A original empilha registradores com os 16 bits de cima sujos; quem
/// recebe (0x55f990 pelas células, 0x801960/0x801040) só usa a palavra de baixo (ver o painel 0x723d20).
pub(crate) unsafe fn cor_de(a: usize) -> u32 {
    *(a as *const u16) as u32
}

/// As regiões que uma rotina das telas da partida mexe: as da tela, a posição do buffer de rede
/// ([0xae290c]) e, nas atualizações (0x801c70 e companhia, que viram um pacote para o `Display`), o
/// `Display` local do gerente (+0x3844). Atualização que vai pela rede (vaga com [+0x7d4] ≠ 0): `None`
/// (o pacote seria enviado duas vezes).
pub(crate) unsafe fn regioes_da_atualizacao(montagem: bool) -> Option<Vec<(usize, usize)>> {
    let mut r = regioes_da_tela();
    r.push((0xae290c, 4));
    if montagem {
        return Some(r);
    }
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    if core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7d4 + vaga * 0x300) as *const u16) != 0 {
        return None;
    }
    let local = GERENTE_DE_TELAS + 0x3844;
    if !r.iter().any(|x| x.0 == local) {
        r.push((local, 0x12f550));
    }
    Some(r)
}

/// O primeiro byte diferente de cada região (para a mensagem do comparador).
pub(crate) fn diferencas_da_tela(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<String> {
    mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
        let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
        Some(format!("{:#x}+{:#x}", a, k))
    }).collect()
}

/// Acrescenta o texto em `origem` ao fim do texto 0xde1f64 (o strcat da original).
pub(crate) unsafe fn acrescentar_ao_texto(origem: *const u8) {
    let mut fim = 0xde1f64usize;
    while *(fim as *const u8) != 0 {
        fim += 1;
    }
    let mut i = 0;
    loop {
        let b = *origem.add(i);
        *((fim + i) as *mut u8) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Colunas 5 a 24 das estatísticas do time: a cor e o campo (byte da linha do jogador) mostrado.
pub(crate) const CAMPOS_DAS_ESTATISTICAS: [(usize, usize); 20] = [
    (0xaebdec, 0x1ce), (0xaebdec, 0x1cf), (0xaebdec, 0x1d0), (0xaebdd4, 0x1d1), (0xaebdd4, 0x1d2),
    (0xaebdd4, 0x1d3), (0xaebdec, 0x1d4), (0xaebdec, 0x1d5), (0xaebdec, 0x1d6), (0xaebdd4, 0x1b7),
    (0xaebdd4, 0x1b5), (0xaebdd4, 0x1b6), (0xaebdec, 0x1d7), (0xaebdec, 0x1b8), (0xaebdec, 0x1bb),
    (0xaebdd4, 0x1b9), (0xaebdd4, 0x1ba), (0xaebdd4, 0x1b0), (0xaebde8, 0x1c5), (0xaebdec, 0x1b1),
];

/// writes the 20 player rows of the team stats table (22 columns from the param 0x1c table), building the cells or
/// updating what changed
/// Original: `fill_team_stats_rows` (0x721cd0, match_screens.cpp, cdecl).
///
/// [pt] { 0x721cd0 } (cdecl, match_screens.cpp, ~4,2 KB): as 20 linhas de jogador das estatísticas do time
/// do lado [parâmetro 0x1b] (dados: na partida [p0 + 0x5f] e p0 + 4, fora dela os parâmetros 0x20 e
/// 0x1f; linha i em dados + 1573·lado + 73·i, vazia com 0xfb em +0x1ad). As 22 colunas vêm da tabela
/// do parâmetro 0x1c (1: 0xa16178, 2: 0xa1618c): número (ou i + 1), dois marcadores, o nome ("(c)" no
/// capitão; botão 0x7dc/0x7d7 quando a pessoa confere, 0x53e990), o evento (lesão, expulsão, entrada,
/// saída) e os números. Com o parâmetro 0x1d (a foto anterior), célula sem mudança fica sem texto.
/// `modo` ≠ 0 monta as células na lista `area` (0x8040a0 botão, 0x804100 texto; o primeiro item no
/// parâmetro 0x1a se estiver em −1); `modo` 0 atualiza os itens a partir do parâmetro 0x1a (0x801c70,
/// 0x800d30, 0x801960). Parâmetro 0x1c fora de 1 e 2 (a original lê a tabela sem valor): a original.
pub(crate) unsafe extern "C" fn fill_team_stats_rows(modo: u32, area: usize) {
    type T9 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize) -> u32;
    type T11 = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, u32, u32, u32, usize, u32, usize) -> u32;
    let celula: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: T9 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let texto_do_item: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_STYLE);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT_COLOUR);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    let nome_de: unsafe extern "C" fn(u32, u32, u32, *mut u8, u32, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_NAME_FROM_IDS);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    const BUF: usize = 0xde1f64;
    let sm = GERENTE_DE_TELAS;

    let lado = parametro_da_tela(0x1b) as u8 as i8;
    let p0 = parametro_da_tela(0);
    let antes = parametro_da_tela(0x1d);
    let tabela = match parametro_da_tela(0x1c) as u8 {
        1 => 0xa16178usize,
        2 => 0xa1618c,
        _ => return orig::<unsafe extern "C" fn(u32, usize)>(IDX_FILL_TEAM_STATS_ROWS)(modo, area),
    };
    let dados = if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        let d = parametro_da_tela(0x20);
        parametro_da_tela(0x1f);
        d
    };
    let mut item = parametro_da_tela(0x1a) as u32;
    let base_lado = (lado as i32).wrapping_mul(1573);
    // o estado das células passa de uma para a outra, como na original (a coluna só volta a 1 na coluna 0)
    let (mut coluna, mut cor, mut fundo, mut fonte, mut marcas, mut estilo) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for i in 0..20i32 {
        let linha = base_lado.wrapping_add(73 * i) as usize;
        let reg = dados.wrapping_add(linha);
        let b = |o: usize| *((reg + o) as *const u8);
        let bi = |o: usize| *((reg + o) as *const i8);
        for j in 0..22usize {
            let mut botao: (u32, usize) = (0, 0);
            *(BUF as *mut u8) = 0;
            let foto = if antes != 0 { linha.wrapping_add(antes).wrapping_add(0x190) } else { 0 };
            // o campo `o` da linha igual ao da foto anterior
            let igual = |o: usize| *((foto + o - 0x190) as *const u8) == b(o);
            let id = core::ptr::read_unaligned((reg + 0x190) as *const i32);
            if b(0x1ad) == 0xfb {
                continue;
            }
            let codigo = *((tabela + j) as *const u8) as usize;
            if codigo > 0x18 {
                continue;
            }
            match codigo {
                0 => {
                    cor = cor_de(0xae3184);
                    coluna = 1;
                    fonte = 0xc;
                    fundo = if i < 0xb { cor_de(0xaebe04) } else { rgb(0, 0, 0x40, 0) };
                    let n = bi(0x1ac);
                    marcas = 0x30;
                    estilo = 1;
                    formatar(BUF, 0x98806c, if n > 0 { n as i32 } else { i + 1 });
                    // ponytail: a original compara i com ([info + 0x4d] & 0xf) + 0xb, mas os dois ramos dão 0x30
                }
                1 | 2 => {
                    cor = cor_de(0xae317e);
                    marcas = 1;
                    coluna = codigo as u32 + 2;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(foto != 0 && igual(0x1bc) && igual(0x1bd)) {
                        let (a, d) = (b(0x1bc), b(0x1bd));
                        // a cor do cartao vai no fundo da celula ([esp+0x1c] na original), nao no texto
                        if codigo == 1 {
                            if a != 0 {
                                marcas = 0x20030;
                                fundo = cor_de(0xaebdec);
                            } else if d != 0 {
                                marcas = 0x20030;
                                fundo = cor_de(0xae31a8);
                            }
                        } else if a != 0 && d != 0 {
                            fundo = cor_de(0xae31a8);
                            marcas = 0x40030;
                        }
                    }
                }
                3 => {
                    let (n1, n2, n3) = (
                        core::ptr::read_unaligned((reg + 0x198) as *const u32),
                        core::ptr::read_unaligned((reg + 0x19c) as *const u32),
                        core::ptr::read_unaligned((reg + 0x1a0) as *const u32),
                    );
                    marcas = 1;
                    estilo = 2;
                    coluna = 5;
                    fonte = 1;
                    fundo = 0;
                    let mut nome = [0u8; 0xc8];
                    nome_de(n1, n2, n3, nome.as_mut_ptr(), 0xc8, 1, id as u32);
                    let capitao = *((dados.wrapping_add(base_lado as usize) + 0x153) as *const i8) as i32;
                    if i == capitao {
                        texto(BUF, 0xa16a7c, nome.as_ptr());
                    } else {
                        copiar_para_o_texto(nome.as_ptr() as usize);
                    }
                    if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                        let pessoa = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id as usize * 110);
                        if confere(pessoa, n1, n2, n3, reg + 0x1a4) != 0 {
                            let v = if lado == 0 { *(dados as *const i32) } else { *((dados + 4) as *const i32) };
                            let limite = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                            botao = (if v >= limite { 0x7dc } else { 0x7d7 }, pessoa);
                        }
                    }
                    if core::ptr::read_unaligned((reg + 0x194) as *const u32) == core::ptr::read_unaligned((dados + 0x3c) as *const u32) {
                        cor = if *(G_PLAYER_SETUP_COLOURBLIND_FLAG as *const u8) != 0 { cor_de(0xaebdec) } else { cor_de(0xaebde8) };
                    } else if bi(0x1c3) > -1 || bi(0x1c4) > -1 || bi(0x1c0) > -1 {
                        cor = cor_de(0xae317e);
                    } else if i <= 0xa || bi(0x1c2) != -1 {
                        cor = cor_de(0xae3184);
                    } else {
                        cor = cor_de(0xae317e);
                    }
                }
                4 => {
                    cor = cor_de(0xae3154);
                    estilo = 1;
                    coluna = 6;
                    fonte = 1;
                    fundo = 0;
                    marcas = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1)
                        && !(foto != 0 && igual(0x1c3) && igual(0x1c4) && igual(0x1c0) && igual(0x1c2))
                    {
                        if bi(0x1c3) > -1 {
                            texto(BUF, 0xa16a68, bi(0x1c3) as i32 + 1);
                        } else if bi(0x1c4) > -1 {
                            texto(BUF, 0xa16a54, bi(0x1c4) as i32 + 1);
                        } else if bi(0x1c0) > -1 {
                            texto(BUF, 0xa16a40, bi(0x1c0) as i32 + 1);
                        } else if bi(0x1c2) > -1 {
                            texto(BUF, 0xa16a2c, bi(0x1c2) as i32 + 1);
                            cor = cor_de(0xaebdd4);
                        }
                    }
                }
                _ => {
                    let (endereco_da_cor, campo) = CAMPOS_DAS_ESTATISTICAS[codigo - 5];
                    coluna = coluna.wrapping_add(1);
                    cor = cor_de(endereco_da_cor);
                    marcas = 1;
                    fonte = 0xc;
                    fundo = 0;
                    estilo = 1;
                    if !(i >= 0xb && bi(0x1c2) <= -1) && !(foto != 0 && igual(campo) && igual(0x1c2)) {
                        match codigo {
                            22 => {
                                formatar(BUF, 0x98806c, bi(campo) as i32);
                                cor = cor_de(0xaebdde);
                            }
                            24 => {
                                if b(campo) != 0 {
                                    formatar(BUF, 0x98806c, b(campo) as u32);
                                }
                            }
                            _ => {
                                formatar(BUF, 0x98806c, b(campo) as u32);
                            }
                        }
                    }
                }
            }
            if modo != 0 {
                let r = if botao.1 != 0 {
                    celula(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF, botao.0, botao.1)
                } else {
                    texto_da_celula(sm, area, coluna, i + 1, marcas, fundo, fonte, estilo, cor, BUF)
                };
                if item as u16 == 0xffff {
                    gravar_parametro_da_tela(0x1a, r as u16 as i16 as i32 as usize);
                    item = r;
                }
            } else {
                texto_do_item(sm, item, BUF);
                margem(sm, item, marcas);
                cor_do_texto(sm, item, cor);
                item = item.wrapping_add(1);
            }
        }
    }
}

/// rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page
/// footer
/// Original: `refresh_latest_scores_rows` (0x724520, match_screens.cpp, cdecl).
///
/// [pt] { 0x724520 } (cdecl, match_screens.cpp, ~4,9 KB): reescreve as linhas do painel dos últimos placares
/// (parâmetros 0 a partida ou nulo, 0xe a entrada do dia, 0xf quantas linhas, 0x10 o primeiro item, 6
/// itens por linha, 0x11 a página mostrada, 0x12 parado). Limpa as linhas, depois reparte as partidas
/// da entrada ([+4] a [+8], 105 bytes em [[0xb67a50]+0x19]) em páginas (0x13 = quantas): por partida o
/// cabeçalho da fase (0x4bffc0, quando muda), a linha do jogo (times, placar contado pelos eventos,
/// período 0x6b6570) e os autores dos eventos (gol, pênalti, contra, pênalti perdido, expulsão, lesão;
/// minutos do mesmo jogador juntos) e o agregado. Só a página mostrada é escrita (segunda passada).
/// No rodapé "<< Página x/y >>" (ou "Hold") com mais de uma página; no fim a hora no parâmetro 0x14.
pub(crate) unsafe extern "C" fn refresh_latest_scores_rows() {
    let texto_do_item: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_STYLE);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT_COLOUR);
    let cor_de_fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_COLOUR);
    let fonte_do_item: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_FONT);
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_ID);
    let dado: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_DATA);
    let position_place_players: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_RECT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    let nome_da_fase: unsafe extern "C" fn(i32, i32, usize, u32, u32) -> i32 = core::mem::transmute(FN_COMP_UTIL_FORMAT_STAGE_ROUND_NAME);
    let largura_do_texto: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_FONT_TEXT_WIDTH);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let fase: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(FN_MATCH_DAY_GET_PERIOD_RANK);
    let abandono: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_CM3_MATCH_ENGINE_IS_MATCH_ABANDONED);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_FORMAT_PLAYER_NAME);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    const BUF: usize = 0xde1f64;
    let sm = GERENTE_DE_TELAS;
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let i32_em = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let i16_em = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let b = |a: usize| *(a as *const u8);

    let p0 = parametro_da_tela(0);
    let entrada = parametro_da_tela(0xe);
    let n = parametro_da_tela(0xf) as u8 as i32;
    let primeiro = parametro_da_tela(0x10) as u32;
    let pagina = parametro_da_tela(0x11) as u8 as i8;
    let parado = parametro_da_tela(0x12);
    let referencia = if p0 != 0 { p0 } else { u(*(G_MATCH_DAY as *const usize) + 0x34) };
    let item = |linha: i32, desvio: u32| primeiro.wrapping_add((linha as u32).wrapping_mul(6)).wrapping_add(desvio);

    for r in 0..n {
        let it = item(r, 0);
        position_place_players(sm, it, 0, 0, 0, 0);
        margem(sm, it, 1);
        fonte_do_item(sm, it, 3);
        texto_do_item(sm, it, 0xad9c64);
        ligar(sm, it, 0);
        texto_do_item(sm, it + 1, 0xad9c64);
        texto_do_item(sm, it + 2, 0xad9c64);
        texto_do_item(sm, it + 3, 0xad9c64);
        fonte_do_item(sm, it + 4, 3);
        texto_do_item(sm, it + 4, 0xad9c64);
        ligar(sm, it + 4, 0);
        texto_do_item(sm, it + 5, 0xad9c64);
    }
    gravar_parametro_da_tela(0x13, 0);
    let mut paginas: i8 = 0;
    let mut k = i32_em(entrada + 4);
    let fim = i32_em(entrada + 8);
    // os 16 bits de cima de eax no índice do rodapé (`movzx ax` sobre o índice da partida)
    let mut alto = k as u32 & 0xffff0000;
    if k <= fim {
        let mut nova_pagina = true;
        let mut anterior = 0usize;
        let (mut linha, mut folga) = (0i32, 0i32);
        let abandonada = |partida: usize| {
            let e = u(partida + 0x57);
            e != 0 && abandono(e) != 0
        };
        loop {
            let deslocamento = k.wrapping_mul(105) as usize;
            let partida = u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add(deslocamento);
            let m4 = partida + 4;
            let (ebp, extras) = 'refaz: loop {
                if nova_pagina {
                    paginas = paginas.wrapping_add(1);
                    anterior = 0;
                    linha = 0;
                    folga = 0;
                    gravar_parametro_da_tela(0x13, paginas as i32 as usize);
                    nova_pagina = false;
                }
                let (mut g1, mut g2): (u8, u8) = (0, 0);
                if abandonada(partida) {
                    g1 = 0xfd;
                    g2 = 0xfd;
                }
                let passes = if paginas == pagina { 2 } else { 1 };
                let mut resultado = (0i32, 0i32);
                for passe in 1..=passes {
                    let mut ebp = folga + linha;
                    if passe != 2 && ebp > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }
                    let (w32, w30) = (i16_em(m4 + 0x32), i16_em(m4 + 0x30));
                    let mesmo = anterior != 0 && w32 == i16_em(anterior + 0x32) && w30 == i16_em(anterior + 0x30);
                    if !mesmo && !(w32 == 0 && w30 == 0) {
                        if passe == 2 {
                            nome_da_fase(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
                            let mut x = largura_do_texto(3, BUF) + 0x7d;
                            if x >= 0x2f1 {
                                x = 0x2f1;
                            }
                            let comp = u(m4 + 0x14);
                            let it = item(ebp, 0);
                            if amistoso(*(G_FRIENDLY as *const usize), comp) == 0 && selecao(*(G_RESERVE_COMP as *const usize), comp) == 0 {
                                position_place_players(sm, it, 0x7d, 0, x, 0);
                                cor_do_texto(sm, it, cor_de(0xaebdec));
                                texto_do_item(sm, it, BUF);
                                ligar(sm, it, 0x7e5);
                                let chave = ((*(comp as *const u32) << 11 | (w32 as u16 as u32 & 0x7ff)) << 11) | (w30 as u16 as u32 & 0x7ff);
                                dado(sm, it, chave as usize);
                            } else {
                                position_place_players(sm, it, 0x7d, 0, x, 0);
                                cor_do_texto(sm, it, cor_de(0xaebdec));
                                texto_do_item(sm, it, BUF);
                            }
                            ebp += 1;
                            let it = item(ebp, 0);
                            position_place_players(sm, it, 0x7d, 0, x, 0);
                            margem(sm, it, 0xc400);
                            cor_de_fundo(sm, it, cor_de(0xaebdec));
                            ebp += 1;
                        } else {
                            if ebp + 2 > n {
                                nova_pagina = true;
                                continue 'refaz;
                            }
                            ebp += 2;
                        }
                    }
                    if passe == 2 {
                        // a linha do jogo
                        let it = item(ebp, 0);
                        for (lado, desvio) in [(0x1cusize, 0u32), (0x20, 4)] {
                            let time = u(m4 + lado);
                            formatar(BUF, 0x987ff0, time + 0x38);
                            let h = humano(time, 0, 1, 0, 0);
                            cor_do_texto(sm, it + desvio, if h != 0 { cor_de(0xaebdec) } else { cor_de(0xaebde8) });
                            texto_do_item(sm, it + desvio, BUF);
                            ligar(sm, it + desvio, if b(m4 + 0x40 + desvio as usize / 4) != 0 { 0x7d5 } else { 0x7d6 });
                            dado(sm, it + desvio, time);
                            if desvio == 0 {
                                match g1 {
                                    0xfe => texto(BUF, 0x99c9e4, 0xad9c64),
                                    0xfd => texto(BUF, 0x99c9c4, 0xad9c64),
                                    _ => formatar(BUF, 0xa16d1c, g1 as i8 as i32),
                                };
                                texto_do_item(sm, it + 1, BUF);
                                texto_do_item(sm, it + 2, 0x989adc);
                                // a original decide o placar de fora pelo de casa (0xfe/0xfd)
                                match g1 {
                                    0xfe => texto(BUF, 0x99c9a4, 0xad9c64),
                                    0xfd => texto(BUF, 0x99c984, 0xad9c64),
                                    _ => formatar(BUF, 0xa16d14, g2 as i8 as i32),
                                };
                                texto_do_item(sm, it + 3, BUF);
                            }
                        }
                        // o período
                        let rotulo = (|| -> usize {
                            if b(referencia + 0x67) == 0xff {
                                return 0x98ba0c;
                            }
                            let lado_p = b(partida + 0x67);
                            if lado_p == 0xff {
                                return 0x98ba0c;
                            }
                            let er = u(referencia + 0x5f);
                            if er == 0 {
                                return 0x98ba0c;
                            }
                            let ep = u(partida + 0x5f);
                            if ep == 0 {
                                return 0x98ba0c;
                            }
                            let x = fase(*((er + 0x4f) as *const u16) as u32, b(referencia + 0x67) as u32) as i8;
                            let y = fase(*((ep + 0x4f) as *const u16) as u32, lado_p as u32) as i8;
                            let e = if x < y { referencia } else { partida };
                            let cl = b(e + 0x67);
                            let w = || *((u(e + 0x5f) + 0x4f) as *const u16);
                            if cl == 1 && w() == 0x1ef {
                                0xa16d10
                            } else if cl == 4 && w() == 0x3de {
                                0xa16d0c
                            } else if cl == 1 && w() == 0x3de {
                                0xa16d08
                            } else if cl == 2 && w() == 0 {
                                0xa16d04
                            } else if cl == 1 && w() == 0x483 {
                                0xa16cfc
                            } else if cl == 4 && w() == 0x528 {
                                0xa16cf4
                            } else if cl == 1 && w() == 0x528 {
                                0xa16cec
                            } else if cl == 2 && w() == 1 {
                                0xa16ce4
                            } else {
                                0xa16ce0
                            }
                        })();
                        texto(BUF, rotulo);
                        texto_do_item(sm, it + 5, BUF);
                    } else if ebp + 2 > n {
                        nova_pagina = true;
                        continue 'refaz;
                    }

                    // os eventos (até 50, 4 bytes em [partida + 0x5f] + 0x57: lado, jogador, tipo, minuto)
                    let mut usados = [0u32; 0x32];
                    let ev = u(u(*(G_MATCH_DAY as *const usize) + 0x19).wrapping_add(deslocamento) + 0x5f);
                    ebp += 1;
                    let (mut casa, mut fora) = (ebp, ebp);
                    let mut extras = 0i32;
                    let (mut r24, mut cor4c, mut coluna44) = (0i32, 0u32, 0u32);
                    let (mut n98, mut na0, mut na4) = (0u32, 0u32, 0u32);
                    let valido = |minuto: u8| {
                        if b(referencia + 0x67) == 0xff {
                            return false;
                        }
                        let e = u(referencia + 0x5f);
                        e != 0 && (minuto as i16) <= i16_em(e + 0x4d)
                    };
                    let registro = |lado: u8, jogador: u8| {
                        ev.wrapping_add((lado as i8 as i32 * 1573) as usize).wrapping_add((jogador as i8 as i32 * 73) as usize)
                    };
                    let mut j = 0usize;
                    while ev != 0 && j < 0x32 {
                        let e = ev + 4 * j;
                        let lado = b(e + 0x57);
                        if lado == 0xff {
                            break;
                        }
                        if usados[j] != 0 || !valido(b(e + 0x5a)) {
                            j += 1;
                            continue;
                        }
                        let minuto = b(e + 0x5a) as u32;
                        let jogador = b(e + 0x58);
                        if passe == 2 {
                            let reg = registro(lado, jogador);
                            n98 = *((reg + 0x198) as *const u32);
                            na4 = *((reg + 0x1a0) as *const u32);
                            na0 = *((reg + 0x19c) as *const u32);
                        }
                        let tipo = b(e + 0x59);
                        if tipo == 7 || tipo == 8 {
                            j += 1;
                            continue;
                        }
                        if !(1..=6).contains(&tipo) {
                            aviso_em(0xa161d4, 0x1b9c);
                            j += 1;
                            continue;
                        }
                        // o cursor do lado (o gol contra vai para o outro lado)
                        let de_casa = (lado == 0) != (tipo == 2);
                        let r2c;
                        if de_casa {
                            r24 = casa;
                            casa += 1;
                            r2c = casa;
                        } else {
                            r24 = fora;
                            fora += 1;
                            r2c = fora;
                        }
                        if passe == 2 {
                            coluna44 = if de_casa { 0 } else { 2 };
                            let mut nome = [0u8; 0xc8];
                            nome_do_jogador(ev, jogador as i8 as i32, lado as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 2);
                            let (modelo, c) = match tipo {
                                1 => (0xa16be8, cor_de(0xae3184)),
                                3 => (0xa16c0c, cor_de(0xae3184)),
                                2 => (0xa16cb8, cor_de(0xaebdd4)),
                                4 => (0xa16c8c, 0),
                                5 => (0xa16c60, cor_de(0xae31a8)),
                                _ => (0xa16c34, 0),
                            };
                            texto(BUF, modelo, nome.as_ptr(), minuto + 1);
                            cor4c = if tipo == 4 || tipo == 6 { rgb(0, 0xc0, 0, 0) } else { c };
                        } else if tipo <= 3 {
                            if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if de_casa {
                                g1 = g1.wrapping_add(1);
                            } else {
                                g2 = g2.wrapping_add(1);
                            }
                        }
                        // os outros eventos do mesmo jogador e do mesmo tipo (gol e pênalti juntos)
                        let reg = registro(lado, jogador);
                        let id = i32_em(reg + 0x190);
                        let gol = |t: u8| t == 1 || t == 3;
                        for m in (j + 1)..0x32 {
                            let em = ev + 4 * m;
                            let lado_m = b(em + 0x57);
                            if lado_m == 0xff {
                                break;
                            }
                            if usados[m] != 0 || !valido(b(em + 0x5a)) {
                                continue;
                            }
                            let minuto_m = b(em + 0x5a) as u32;
                            if i32_em(registro(lado_m, b(em + 0x58)) + 0x190) != id {
                                continue;
                            }
                            let tm = b(em + 0x59);
                            if tm != tipo && !(gol(tm) && gol(tipo)) {
                                continue;
                            }
                            if passe == 2 {
                                let mut mais = [0u8; 0x100];
                                if tm == 3 {
                                    texto(mais.as_mut_ptr() as usize, 0xa16bd4, minuto_m + 1);
                                } else {
                                    formatar(mais.as_mut_ptr() as usize, 0x990b40, minuto_m + 1);
                                }
                                acrescentar_ao_texto(mais.as_ptr());
                            } else if abandonada(partida) {
                                g1 = 0xfd;
                                g2 = 0xfd;
                            } else if gol(tm) {
                                if lado_m == 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            } else if tm == 2 {
                                if lado_m != 0 { g1 = g1.wrapping_add(1) } else { g2 = g2.wrapping_add(1) }
                            }
                            usados[m] = 1;
                        }
                        if passe == 2 {
                            let it = item(r24, coluna44 * 2);
                            cor_do_texto(sm, it, cor4c);
                            texto_do_item(sm, it, BUF);
                            if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                                let pessoa = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id as usize * 110);
                                if confere(pessoa, n98, na0, na4, reg + 0x1a4) != 0 {
                                    let v = if lado == 0 { i32_em(ev) } else { i32_em(ev + 4) };
                                    let limite = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                                    ligar(sm, it, if v >= limite { 0x7dc } else { 0x7d7 });
                                    dado(sm, it, pessoa);
                                }
                            }
                        } else if r2c > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        usados[j] = 1;
                        if (lado == 0 && casa > fora) || (lado == 1 && fora > casa) {
                            extras += 1;
                        }
                        j += 1;
                    }

                    // o agregado (ida e volta)
                    if b(m4 + 0x4b) as i8 > -1 {
                        let (r, do_fora, prox);
                        if casa <= fora {
                            r = casa;
                            casa += 1;
                            do_fora = false;
                            prox = casa;
                        } else {
                            r = fora;
                            fora += 1;
                            do_fora = true;
                            prox = fora;
                        }
                        if passe == 2 {
                            let ida = u(partida);
                            texto(BUF, 0xa16bb0, b(ida + 0x4b) as i8 as i32 + g1 as i8 as i32, b(ida + 0x4c) as i8 as i32 + g2 as i8 as i32);
                            let it = item(r, if do_fora { 4 } else { 0 });
                            fonte_do_item(sm, it, 2);
                            cor_do_texto(sm, it, cor_de(0xaebdec));
                            texto_do_item(sm, it, BUF);
                        } else if prox > n {
                            nova_pagina = true;
                            continue 'refaz;
                        }
                        if (!do_fora && casa > fora) || (do_fora && fora > casa) {
                            extras += 1;
                        }
                    }
                    resultado = (casa.max(fora), extras);
                }
                break resultado;
            };
            linha = ebp;
            folga = (3 - extras).max(1);
            anterior = m4;
            k = k.wrapping_add(1);
            alto = k as u32 & 0xffff0000;
            if k > fim {
                break;
            }
        }
    }

    let it = (primeiro as usize).wrapping_add(((alto | n as u32) as usize).wrapping_mul(6)) as u32;
    if paginas > 1 {
        margem(sm, it, 0x30);
        if *(G_FONT_MODE as *const i32) != -1 {
            formatar(BUF, 0x99ec1c, 0x2a);
            texto_do_item(sm, it, BUF);
        } else {
            texto_do_item(sm, it, 0x98e7c4);
        }
        let modelo = if parado != 0 { 0xa16b8c } else { 0xa16b68 };
        texto(BUF, modelo, pagina as i32, paginas as i32);
        margem(sm, it + 1, 2);
        texto_do_item(sm, it + 1, BUF);
        margem(sm, it + 2, 0x30);
        if *(G_FONT_MODE as *const i32) != -1 {
            formatar(BUF, 0x99ec1c, 0x2b);
            texto_do_item(sm, it + 2, BUF);
        } else {
            texto_do_item(sm, it + 2, 0x98e7a8);
        }
    } else {
        for d in 0..3 {
            margem(sm, it + d, 1);
            texto_do_item(sm, it + d, 0xad9c64);
        }
    }
    gravar_parametro_da_tela(0x14, agora() as usize);
}

/// in a network game, waits for the other humans of the same fixture or closes the screens, shows the notice and opens
/// the match screen
/// Original: `match_screens_handle_waiting_screen` (0x727fb0, match_screens.cpp, cdecl).
///
/// [pt] { 0x727fb0 } (cdecl, match_screens.cpp): a resposta da tela de espera do jogo em rede. Sem ficha do
/// humano da vez (0x765480) ou sem partida nela (+0xe2): −6. Outro humano (0x7ffd80) não ausente
/// (0x7ffdd0) com a mesma partida: guarda a chave dele em [0xb77c4a] e −6. Senão fecha as telas
/// (0x803d80; bloqueado: −11), mostra "Waiting for other players..." (0x802f80) e abre a tela da
/// partida (0x718aa0) na vaga do humano: −4.
pub(crate) unsafe extern "C" fn match_screens_handle_waiting_screen() -> i32 {
    crate::comparacao::ramo("telas espera dos outros jogadores");
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let humanos: unsafe extern "thiscall" fn(usize, *mut usize, *mut i16, i32) = core::mem::transmute(FN_SCREENMANAGER_GET_CHANNEL_MANAGER_KEYS);
    let ausente: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_MANAGER_AWAY);
    let fechar_tudo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_CLOSE_ALL_SCREENS_803D80);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mostrar: unsafe extern "thiscall" fn(usize, usize, i32, u32, i32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let abrir_partida: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_OPEN_MATCH_SCREEN);
    let sm = GERENTE_DE_TELAS;
    let h = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    let chave_atual = || core::ptr::read_unaligned((0xb74c26 + h() as isize * 0x300) as *const usize);
    if ficha(0) == 0 {
        return -6;
    }
    let partida = core::ptr::read_unaligned((ficha(0) + 0xe2) as *const usize);
    if partida == 0 {
        return -6;
    }
    let mut chaves = [0usize; 16];
    let mut quantos: i16 = 0;
    humanos(sm, chaves.as_mut_ptr(), &mut quantos, h() as i32);
    let mut k = 0usize;
    while (k as i32) < quantos as i32 {
        let c = chaves[k];
        if c != chave_atual() && ausente(sm, c) == 0 && core::ptr::read_unaligned((ficha(c) + 0xe2) as *const usize) == partida {
            *(G_MATCH_WAITING_MANAGER as *mut usize) = c;
            return -6;
        }
        k += 1;
    }
    if fechar_tudo(sm) == 0 {
        return -11;
    }
    texto(0xde1f64, 0xa16d94);
    mostrar(sm, 0xde1f64, 1, 0, h() as i32);
    abrir_partida(chave_atual(), partida);
    -4
}

/// As células são comparadas de verdade: montagem (`modo` ≠ 0) pela lista de exibição, atualização
/// local pelo `Display` do gerente; os parâmetros (o 0x1a) e o texto 0xde1f64 também. Atualização que
/// vai pela rede ou parâmetro 0x1c fora de 1 e 2: só a original.
pub(crate) unsafe extern "C" fn comparar_fill_team_stats_rows(modo: u32, area: usize) {
    let o = orig::<unsafe extern "C" fn(u32, usize)>(IDX_FILL_TEAM_STATS_ROWS);
    let coluna = parametro_da_tela(0x1c) as u8;
    let mut reg = match regioes_da_atualizacao(modo != 0) {
        Some(r) if coluna == 1 || coluna == 2 => r,
        _ => return o(modo, area),
    };
    reg.extend(regioes_dos_parametros());
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(modo, area), || fill_team_stats_rows(modo, area));
    PLACAR.registrar(igual, || format!("fill_team_stats_rows({}, {:#x}): diferencas em {:?}", modo, area, diferencas_da_tela(&mo, &mr)));
}

/// O redesenho é comparado de verdade (atualização local): o `Display` do gerente, os parâmetros (menos
/// o 0x14, a hora que a rotina grava no fim) e o texto 0xde1f64. Pela rede: só a original.
pub(crate) unsafe extern "C" fn comparar_refresh_latest_scores_rows() {
    let o = orig::<unsafe extern "C" fn()>(IDX_REFRESH_LATEST_SCORES_ROWS);
    let mut reg = match regioes_da_atualizacao(false) {
        Some(r) => r,
        None => return o(),
    };
    reg.push((0xb6b7a8, 12));
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || refresh_latest_scores_rows());
    PLACAR.registrar(igual, || format!("refresh_latest_scores_rows: diferencas em {:?}", diferencas_da_tela(&mo, &mr)));
}

// ponytail: a espera fecha as telas, mostra a mensagem e abre a tela da partida (rede); não roda duas
// vezes: no modo comparação só a original (a cópia é provada no modo Rust pelo ramo)
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_waiting_screen() -> i32 {
    orig::<unsafe extern "C" fn() -> i32>(IDX_MATCH_SCREENS_HANDLE_WAITING_SCREEN)()
}

// ---------- 204a leva: as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia (match_screens.cpp) ----------

pub(crate) type LinhaDoDisplay = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type CaixaDoGerente = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type TextoDoGerente = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// A cor (u16) em `a` com os 16 bits de cima de `x`: onde a original troca só o `ax`/`cx`/`dx` de um
/// registrador que tem um valor conhecido (a devolução de uma chamada, uma conta) e o empurra inteiro.
pub(crate) unsafe fn cor_sobre(x: usize, a: usize) -> usize {
    (x & 0xffff_0000) | cor_em(a)
}

/// A mensagem de "não há" dos painéis da partida (0x5ed070 com as linhas que cabem na fonte 3).
pub(crate) unsafe fn mensagem_do_painel(topo: *mut i32, base: *mut i32, texto_id: usize) {
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let a = altura(3);
    let n = ((*base - *topo - 0x10) / a) as u8;
    texto(0xde1f64, texto_id);
    mensagem(0x6e, *topo as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, cor_em(0xae3184), 0xde1f64, 0);
}

/// draws the action zones: the period selector, the pitch split into three zones with both clubs' colour bars, or the
/// no-zones message
/// Original: `draw_action_zones_panel` (0x71f500, match_screens.cpp, cdecl).
///
/// [pt] { 0x71f500 } (cdecl, match_screens.cpp): desenha as zonas de ação de `*topo` a `*base`. Fora da
/// partida (parâmetro 0 nulo) e sem dados ([parâmetro 0x20 + 0x4c] ≠ 0): a mensagem "There are no
/// action zones available.". Senão: a caixa "View" com a lista dos cinco períodos (o item do primeiro no
/// parâmetro 0x23; o texto do período do parâmetro 0xa fica guardado), `*topo` += 0x19, o campo
/// (0x8ac600, que devolve o retângulo), as duas linhas que o dividem em três, o título do período, as
/// três zonas (botão do parâmetro 0xb na primeira), em cada zona o painel com as barras das cores dos
/// dois clubes (0x53b640) e a linha do meio, o painel de baixo igual, o retângulo nos parâmetros 0x16 a
/// 0x19, `*base` no 0xc e os valores (0x720320).
pub(crate) unsafe extern "C" fn draw_action_zones_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das zonas de acao");
    let caixa: CaixaDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let txt: TextoDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let guarda = G_ACTION_ZONES_STATIC_GUARD as *mut u8;
    if *guarda & 1 == 0 {
        *guarda |= 1;
        *(G_ACTION_ZONES_STATIC_COLOUR as *mut u16) = *(G_GUI_COLOUR_YELLOW as *const u16);
    }
    let p0 = parametro_da_tela(0);
    let vista = parametro_da_tela(0xa) as u8;
    let dados = if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        let d = parametro_da_tela(0x20);
        if *((d + 0x4c) as *const u8) != 0 {
            return mensagem_do_painel(topo, base, 0xa16880);
        }
        d
    };

    texto(BUF, 0x988004);
    let t = *topo as usize;
    let c = caixa(sm, 0x6e, t, 0xeb, t + 0x14, 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), BUF, 0, 0);
    let periodos = lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, cor_em(0xae31aa), c);
    // ponytail: com a vista fora de 0 a 4 a original passa o texto de uma variável local sem valor; aqui vazio
    let mut titulo = [0u8; 200];
    for (k, &(rotulo, item)) in [(0xa16738usize, 4u8), (0xa167ec, 0), (0xa167e0, 1), (0xa168a8, 2), (0x98800c, 3)].iter().enumerate() {
        texto(BUF, rotulo);
        let r = linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 0, 0x90, cor_em(0xae31aa), cor_em(0xaebdec), 0x21, 1, cor_em(0xaebdea), BUF, 0, 0, item as usize, periodos);
        if k == 0 {
            gravar_parametro_da_tela(0x23, r as u16 as i16 as i32 as usize);
        }
        if vista == item {
            for i in 0..200 {
                let b = *((BUF + i) as *const u8);
                titulo[i] = b;
                if b == 0 {
                    break;
                }
            }
        }
    }

    *topo += 0x19;
    lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let (mut x0, mut y0, mut x1, mut y1) = (0i32, 0i32, 0i32, 0i32);
    let campo: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) =
        core::mem::transmute(FN_DRAW_PITCH);
    campo(0xb9, *topo + 10, 0x2c1, *base - 0x37, &mut x0, &mut y0, &mut x1, &mut y1, 1, -1, 0x4080_0000);
    let w = x1 - x0;
    let terco = w / 3;
    let dois = w.wrapping_mul(2) / 3;
    let u = |v: i32| v as usize;

    linha(display_do_humano(), 1, u(x0 + terco), u(y0), u(x0 + terco), u(y1), 0, 0, 0x2400, cor_sobre(u(w), 0xb6b7a4), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    linha(display_do_humano(), 1, u(x0 + dois), u(y0), u(x0 + dois), u(y1), 0, 0, 0x2400, cor_sobre(u(w.wrapping_mul(2)), 0xb6b7a4), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    txt(sm, u(x0 + terco + 10), u(y0 + 5), u(x0 + dois - 10), u(y0 + 0x19), 2, 0, 0xc, 2, cor_em(0xae3184), titulo.as_ptr() as usize, 1);
    let r = txt(sm, u(x0), u(y0), u(x0 + terco), u(y1), 1, 0, 0xc, 4, cor_em(0xaebdec), 0, 1);
    gravar_parametro_da_tela(0xb, r as u16 as i16 as i32 as usize);
    txt(sm, u(x0 + terco), u(y0), u(x0 + dois), u(y1), 1, 0, 0xc, 4, cor_em(0xaebdec), 0, 1);
    txt(sm, u(x0 + dois), u(y0), u(x1), u(y1), 1, 0, 0xc, 4, cor_em(0xaebdec), 0, 1);
    txt(sm, u(x0 + dois + 10), u(y0 + 5), u(x1 - 10), u(y0 + 0x19), 1, 0, 0xc, 1, cor_em(0xaebdec), 0, 1);

    // as cores dos dois clubes (0x53b640 grava u16; o resto da variável da original é lixo da pilha, aqui 0)
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let (mut c1a, mut c1b, mut c2a, mut c2b) = (0u32, 0u32, 0u32, 0u32);
    let clubes = *(G_CLUB_TABLE as *const usize);
    let id1 = core::ptr::read_unaligned(dados as *const i32);
    cores_do_clube((clubes as isize + id1 as isize * 0x245) as usize, *((dados + 0x152) as *const u8) as u32, &mut c1a, &mut c1b);
    let id2 = core::ptr::read_unaligned((dados + 4) as *const i32);
    cores_do_clube((clubes as isize + id2 as isize * 0x245) as usize, *((dados + 0x777) as *const u8) as u32, &mut c2a, &mut c2b);

    // uma zona: a moldura, o painel de `l` a `r`, as barras das duas cores e a linha do meio; devolve a da linha
    let zona = |a: (i32, i32), cor_a: usize, l: i32, r: i32, cor_b: (usize, usize)| -> usize {
        txt(sm, u(a.0), u(y1 - 0x19), u(a.1), u(y1 - 5), 0x30, cor_a, 0xc, 1, cor_em(0xae3184), 0, 1);
        txt(sm, u(l), u(y1 - 0x14), u(r), u(y1 - 10), 0x60, cor_b.0, 0xc, 1, cor_b.1, 0, 1);
        let d = r - l;
        let meio = (d * 25 - 100) * 2 / 100 + l + 2;
        txt(sm, u(l + 2), u(y1 - 0x12), u(meio), u(y1 - 12), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
        txt(sm, u(meio), u(y1 - 0x12), u(r - 2), u(y1 - 12), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
        let xm = l + d / 2;
        let sinal = if d < 0 { 0xffff_0000 } else { 0 };
        linha(display_do_humano(), 1, u(xm), u(y1 - 0x12), u(xm), u(y1 - 0x10), 0, 0, 0x400, sinal | cor_em(0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX)
    };
    let r1 = zona((x0 + 5, x0 + terco - 5), cor_em(0xae317e), x0 + 10, x0 + terco - 10, (cor_sobre(u(y1), 0xae317e), cor_em(0xae3184)));
    let r2 = zona((x0 + terco + 5, x0 + dois - 5), cor_sobre(r1, 0xae317e), x0 + terco + 10, x0 + dois - 10, (cor_sobre(u(terco), 0xae317e), cor_sobre(u(w), 0xae3184)));
    zona((x0 + dois + 5, x1 - 5), cor_sobre(r2, 0xae317e), x0 + dois + 10, x1 - 10, (cor_em(0xae317e), cor_sobre(u(dois), 0xae3184)));

    let b = *base;
    let r = txt(sm, 0xb9, u(b - 0x2d), 0x2c1, u(b - 10), 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), 0, 1);
    txt(sm, 0xd2, u(b - 0x25), 0x2a8, u(b - 0x12), 0x60, cor_em(0xae317e), 0xc, 1, cor_sobre(r, 0xae3184), 0, 1);
    txt(sm, 0xd4, u(b - 0x23), 0x1bd, u(b - 0x14), 0x10, c1b as usize, 0xc, 1, c1a as usize, 0, 1);
    let r = txt(sm, 0x1bd, u(b - 0x23), 0x2a6, u(b - 0x14), 0x10, c2b as usize, 0xc, 1, c2a as usize, 0, 1);
    linha(display_do_humano(), 1, 0x1bd, u(b - 0x23), 0x1bd, u(b - 0x20), 0, 0, 0x400, cor_sobre(r, 0xaebdea), 0, 0, 0, 0, 0, 0, 0, 0, usize::MAX);
    gravar_parametro_da_tela(0x16, u(x0));
    gravar_parametro_da_tela(0x17, u(y0));
    gravar_parametro_da_tela(0x18, u(x1));
    gravar_parametro_da_tela(0x19, u(y1));
    gravar_parametro_da_tela(0xc, u(*base));
    let valores: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_ACTION_ZONES_VALUES);
    valores();
}

/// draws the match report: the minutes and the event texts (building the report outside the match), or the no-report
/// message
/// Original: `draw_match_report_panel` (0x721000, match_screens.cpp, cdecl).
///
/// [pt] { 0x721000 } (cdecl, match_screens.cpp): desenha o relatório da partida de `*topo` a `*base`: uma
/// tabela de 2 colunas (1, 5) com uma linha por texto de cada lance do relatório (0x6d8d10, " %s", cor
/// pelo tipo do lance) e o minuto (0x66f4e0 ou, no idioma 2, 0x90c310) na primeira linha de cada lance
/// de minuto novo; completa com linhas vazias até 20. O relatório: na partida, [[p0 + 0x57] + 0x8f7a];
/// fora dela, o do parâmetro 0x2d, que nasce aqui (0x944cf1 de 0x2db1 bytes, 0x6d5bf0, montado pelos
/// dados + 0xd69 em 0x6d9c40; se não monta, é destruído e sai a mensagem). Os lances do código
/// [dados + 0x4f] 0x1ef, 0x3de (minuto ≥ 45), 0x483 (≥ 90) e 0x528 (≥ 105) ficam de fora. Sem dados
/// ([parâmetro 0x20 + 0x4c]) ou sem linhas: "There is no match report available.". Relatório nulo ou
/// sem memória: a original (caixa de erro).
pub(crate) unsafe extern "C" fn draw_match_report_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_REPORT_PANEL);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    const SEM_RELATORIO: usize = 0xa16900;

    let p0 = parametro_da_tela(0);
    let (dados, partida, relatorio);
    if p0 == 0 {
        dados = parametro_da_tela(0x20);
        partida = parametro_da_tela(0x1f);
        if *((dados + 0x4c) as *const u8) != 0 {
            crate::comparacao::ramo("telas painel do relatorio da partida");
            return mensagem_do_painel(topo, base, SEM_RELATORIO);
        }
        if parametro_da_tela(0x2d) == 0 {
            let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
            let memoria = novo(0x2db1);
            if memoria == 0 {
                // ponytail: sem memória a original mostra o erro (e tenta alocar de novo, sem efeito)
                return o(topo, base);
            }
            crate::comparacao::ramo("telas painel do relatorio da partida (montar)");
            // ponytail: o quadro SEH da original só libera a memória se o construtor lançar exceção, o que ele não faz
            let construir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_MATCH_EVENTS);
            let r = construir(memoria, 0);
            gravar_parametro_da_tela(0x2d, r);
            let montar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_EVENTS_LOAD_EVENT_LOG);
            if montar(r, dados + 0xd69) as u8 == 0 {
                mensagem_do_painel(topo, base, SEM_RELATORIO);
                let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
                destruir(r);
                let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
                liberar(r);
                gravar_parametro_da_tela(0x2d, 0);
                return;
            }
            relatorio = r;
        } else {
            relatorio = parametro_da_tela(0x2d);
        }
    } else {
        dados = core::ptr::read_unaligned((parametro_da_tela(0) + 0x5f) as *const usize);
        partida = parametro_da_tela(0) + 4;
        relatorio = core::ptr::read_unaligned((core::ptr::read_unaligned((parametro_da_tela(0) + 0x57) as *const usize) + 0x8f7a) as *const usize);
    }
    if relatorio == 0 {
        return o(topo, base);
    }
    crate::comparacao::ramo("telas painel do relatorio da partida");

    let lances = |r: usize| *((r + 6) as *const i16);
    let mut colunas: [u8; 2] = [1, 5];
    let mut ultimo_minuto: u8 = 0xff;
    let mut linhas: i32 = 0;
    let mut tabela: usize = 0;
    // a cor da linha: as cores u16 trocam só os 16 bits de baixo (os de cima começam com lixo na original)
    let mut cor: usize = 0;
    if lances(relatorio) <= 0 {
        return mensagem_do_painel(topo, base, SEM_RELATORIO);
    }
    let mut i: i16 = 0;
    loop {
        let entrada = if i >= 0 && i < lances(relatorio) { relatorio + 0x720 + i as usize * 14 } else { 0 };
        let codigo = core::ptr::read_unaligned((dados + 0x4f) as *const u16);
        let pular = codigo == 0x1ef
            || (codigo == 0x3de && *((entrada + 4) as *const u8) >= 0x2d)
            || (codigo == 0x483 && ultimo_minuto >= 0x5a)
            || (codigo == 0x528 && ultimo_minuto >= 0x69);
        if !pular {
            let mut primeira: i32 = -1;
            let mut j: u32 = 0;
            loop {
                let id = if partida != 0 { *(partida as *const u32) } else { u32::MAX };
                let texto_do_lance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_BUILD_REPORT_TEXT);
                let t = texto_do_lance(relatorio, entrada, dados, id, j);
                j += 1;
                if t == 0 {
                    break;
                }
                if linhas == 0 {
                    let (b, a) = (*base, *topo);
                    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
                    let n = ((b - a) / altura(2)) as u8;
                    // ponytail: os 3 bytes de cima das linhas são lixo da pilha na original; aqui 0
                    tabela = lista(display_do_humano(), 0x6e, a as usize, 0x30c, b as usize, 2, colunas.as_mut_ptr() as usize, n as usize, 0, 1, 0, usize::MAX);
                }
                let cor_rgb: unsafe extern "C" fn(u32, u32, u32, u32) -> usize = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
                let k = *(entrada as *const i16) as i32;
                let palavra = |a: usize| (cor & 0xffff_0000) | cor_em(a);
                cor = match k {
                    0x1f56 | 0x1f57 | 0x1fce | 0x2008 => cor_rgb(0, 0xc0, 0, 0),
                    0x1fcc | 0x1fcf | 0x2000 | 0x2013..=0x201b | 0x2114 => cor_rgb(0, 0x96, 0xff, 0),
                    0x1fcd => palavra(0xaebdd4),
                    0x1ff5..=0x1ff8 => palavra(0xae31a8),
                    0x1ff9 => palavra(0xaebdec),
                    _ => palavra(0xae3154),
                };
                if primeira == -1 {
                    primeira = linhas;
                }
                formatar(BUF, 0x98ed14, t);
                texto_da_celula(sm, tabela, 1, linhas as usize, 2, 0, 1, 2, cor, BUF);
                linhas += 1;
            }
            if primeira != -1 {
                let minuto = *((entrada + 4) as *const u8);
                let mut k = primeira;
                if ultimo_minuto != minuto {
                    ultimo_minuto = minuto;
                    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
                    if idioma() != 2 {
                        texto(BUF, 0xa168ec, minuto as u32 + 1);
                    } else {
                        let por_extenso: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_UTILS_FORMAT_ORDINAL);
                        formatar(BUF, 0xa168e4, por_extenso(minuto.wrapping_add(1) as u32, 2));
                    }
                    texto_da_celula(sm, tabela, 0, k as usize, 2, 0, 0xc, 2, cor_em(0xaebdec), BUF);
                    k += 1;
                }
                while k < linhas {
                    texto_da_celula(sm, tabela, 0, k as usize, 2, 0, 0xc, 2, cor_em(0xae3184), 0xad9c64);
                    k += 1;
                }
            }
        }
        i = i.wrapping_add(1);
        if i >= lances(relatorio) {
            break;
        }
    }
    if linhas == 0 {
        return mensagem_do_painel(topo, base, SEM_RELATORIO);
    }
    while linhas < 0x14 {
        let r = texto_da_celula(sm, tabela, 0, linhas as usize, 2, 0, 0xc, 3, cor_em(0xae3184), 0xad9c64);
        texto_da_celula(sm, tabela, 1, linhas as usize, 2, 0, 1, 2, cor_sobre(r, 0xae3184), 0xad9c64);
        linhas += 1;
    }
}

/// builds the day fixtures screen: title, the day's competition tree, the rounds and matches table and the scroll to
/// the human's match
/// Original: `build_day_fixtures_screen` (0x717db0, match_screens.cpp, cdecl).
///
/// [pt] { 0x717db0 } (cdecl, match_screens.cpp): monta a tela dos jogos do dia do grupo do parâmetro 1 (+4 a
/// +8 as partidas do dia em [[0xb67a50]+0x19], 105 bytes cada): o fundo e as cores da competição
/// (0x41c410, 0x53b7c0), o título com o nome (Friendlies/Reserves com mais de uma partida) e o selo
/// (fora dos amistosos e das seleções), a barra de menu (0x75db00 com 4), o título "Fixtures"/"Results"
/// (singular ou plural; o dia pela data 0x54d040/0x54d130), com mais de uma rodada ([+0x15] > 1) a
/// árvore dos grupos de competições do dia (cabeçalho do grupo e uma linha por competição), o selo
/// "Competition", a tabela de 9 colunas com o cabeçalho de cada rodada nova e uma linha por partida
/// (0x49db00), os botões de voltar e avançar, a rolagem até a partida do humano (parâmetro 4) ou à do
/// parâmetro 2 e a marca de vista do humano da vez no grupo (+0x14 + 4h).
pub(crate) unsafe extern "C" fn build_day_fixtures_screen() {
    crate::comparacao::ramo("telas montar os jogos do dia");
    let caixa: CaixaDoGerente = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let linha: LinhaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let sem_selo = |c: usize| {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
        amistoso(*(G_FRIENDLY as *const usize), c) == 0 && selecao(*(G_RESERVE_COMP as *const usize), c) == 0
    };
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd32 = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
    let grupo = parametro_da_tela(1);
    let comp = (*(G_CLUB_COMP_TABLE as *const usize) as isize + rd32(grupo) as isize * 107) as usize;
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_BACKGROUND_SET_COMP_PICTURE);
    fundo(comp, 0, 0);
    // as cores da competição (0x53b7c0 grava u16; o resto da variável da original é lixo da pilha, aqui 0)
    let (mut ca, mut cb) = (0u32, 0u32);
    let cores: unsafe extern "C" fn(usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_COMP_COLOURS);
    cores(comp, &mut ca, &mut cb);
    let mais_de_uma = rd32(grupo + 8) > rd32(grupo + 4);
    let chave = *(comp as *const usize);
    if chave == *(KEY_CLUB_COMP_FRIENDLY as *const usize) && mais_de_uma {
        texto(BUF, 0xa1631c);
    } else if chave == *(KEY_CLUB_COMP_RESERVE as *const usize) && mais_de_uma {
        texto(BUF, 0xa16310);
    } else {
        copiar_para_o_texto(comp + 4);
    }
    let titulo: unsafe extern "thiscall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_ADD_SCREEN_TITLE);
    titulo(sm, cb, ca, BUF);
    if sem_selo(comp) {
        linha(display_do_humano(), 6, 0x7d, 0xa, 0x316, 0x46, 0, 0, 1, 0, 0, 0xc, 7, 0, 0, 0, 0x7d0, comp, usize::MAX);
    }
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_MENUBAR_BUILD);
    menu(4, 0);

    // o título do dia; o byte da data vai inteiro (as rotinas só leem o byte)
    let md = rd(0xb67a50);
    let ini = rd32(grupo + 4);
    let jogado = *((rd(rd(md + 0x19) + ini as usize * 105) + 0x43) as *const u8) != 0xff;
    let formato = match (jogado, rd32(grupo + 8) > ini) {
        (false, true) => 0xa162f0usize,
        (false, false) => 0xa162d0,
        (true, true) => 0xa162b0,
        (true, false) => 0xa16290,
    };
    let dia_a: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_TIME_OF_DAY_NAME);
    let dia_b: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATE_GET_TIME_OF_DAY_GENDER);
    let r1 = dia_a(*(rd(0xb67a50) as *const u8) as u32, 1);
    let r2 = dia_b(*(rd(0xb67a50) as *const u8) as u32) as u8 as i8 as i32;
    let r3 = dia_b(*(rd(0xb67a50) as *const u8) as u32) as u8 as i8 as i32;
    texto(BUF, formato, r3, r2, r1);
    let titulo_do_dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_ADD_SCREEN_SUBTITLE);
    titulo_do_dia(sm, BUF, 1);

    // os 3 bytes de cima do número de linhas da tabela: o contador dos grupos que a original deixou na pilha
    let mut resto: usize = 0;
    if rd32(rd(0xb67a50) + 0x15) > 1 {
        let cabeca = caixa(sm, 0x69, 0xf, 0x78, 0x23, 0x30, cb as usize, 0xc, 1, ca as usize, 0, 0, 0);
        let raiz = lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor_em(0xae31aa), cabeca);
        let mut i: i32 = 0;
        loop {
            let md = rd(0xb67a50);
            let n = rd32(md + 9);
            if i >= n {
                break;
            }
            let e = rd(md + 1) + i as usize * 0x18;
            let pai = if n == 1 {
                raiz
            } else {
                let tipo = *(e as *const i32);
                match tipo {
                    0 => {
                        texto(BUF, 0xa16284);
                    }
                    1 => {
                        texto(BUF, 0xa16248, 0xad9c64);
                    }
                    2 => {
                        texto(BUF, 0xa16210, 0xad9c64);
                    }
                    3..=7 => {
                        let mut nome = [0u8; 0x64];
                        let continente: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_GET_CONTINENT_ADJECTIVE);
                        continente(*((0xb63d64 + tipo as isize * 72) as *const usize), nome.as_mut_ptr(), 0x64, 6);
                        copiar_para_o_texto(nome.as_ptr() as usize);
                    }
                    _ => copiar_para_o_texto(*((0xb63d60 + tipo as isize * 72) as *const usize) + 4),
                }
                if *((e + 0x14) as *const u32) != 0 {
                    let r = linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor_sobre(raiz, 0xae31aa), cor_em(0xaebdea), 1, 1, cor_em(0xae3184), BUF, 0, 0, 0, raiz);
                    lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor_em(0xae31aa), r)
                } else {
                    let r = linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor_sobre(raiz, 0xae31aa), cor_em(0xaebdec), 1, 1, cor_em(0xaebdea), BUF, 0, 0, 0, raiz);
                    lista(display_do_humano(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor_sobre(r, 0xae31aa), r)
                }
            };
            let mut k = rd32(e + 4);
            while k <= rd32(e + 8) {
                let m = (rd(rd(0xb67a50) + 0xd) as isize + k as isize * 0x54) as usize;
                let c = (*(G_CLUB_COMP_TABLE as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                nome_da_competicao_no_painel(c, m);
                let v = *((m + 0x10) as *const u32) as usize;
                if v != 0 {
                    linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor_em(0xae31aa), cor_em(0xaebdea), 1, 1, cor_sobre(v, 0xae3184), BUF, 0, 1, m, pai);
                } else {
                    linha(display_do_humano(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor_em(0xae31aa), cor_em(0xaebdec), 1, 1, cor_em(0xaebdea), BUF, 0, 1, m, pai);
                }
                k += 1;
            }
            i += 1;
        }
        resto = i as usize;
    }

    let mut esquerda = 0x91usize;
    if sem_selo(comp) {
        let r = texto(BUF, 0xa16204) as u32 as usize;
        caixa(sm, 0x28f, 0x91, 0x30c, 0xa5, 0x30, cor_sobre(r, 0xae317e), 0xc, 1, cor_em(0xae3184), BUF, 0x7d0, comp);
        esquerda = 0xaa;
    }
    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let n = ((0x207 - esquerda as i32) / altura(3)) as u8;
    let tabela = lista(display_do_humano(), 0x6e, esquerda, 0x30c, 0x217, 9, colunas.as_mut_ptr() as usize, (resto & !0xff) | n as usize, 0, 2, 0, usize::MAX);

    let mut k: i32 = 0;
    let mut alvo: i32 = -1;
    let mut anterior: usize = 0;
    let humano_manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let mut j = rd32(grupo + 4);
    while j <= rd32(grupo + 8) {
        let partidas = rd(rd(0xb67a50) + 0x19);
        let fx = rd(partidas + j as usize * 105);
        let w30 = *((fx + 0x30) as *const i16);
        let w32 = *((fx + 0x32) as *const i16);
        let mesma_rodada = anterior != 0 && w32 == *((anterior + 0x32) as *const i16) && w30 == *((anterior + 0x30) as *const i16);
        if !mesma_rodada && !(w32 == 0 && w30 == 0) {
            if anterior != 0 {
                // a original empurra edx = o ponteiro das partidas com a cor em dx
                texto_da_celula(sm, tabela, 0, k as usize, 1, 0, 1, 1, cor_sobre(partidas, 0xae3184), 0);
                k += 1;
            }
            let rodada: unsafe extern "C" fn(i32, i32, usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_FORMAT_STAGE_ROUND_NAME);
            rodada(w32 as i32, w30 as i32, BUF, 0x7d0, 0);
            let largura_do_texto: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(FN_FONT_TEXT_WIDTH);
            let mut largura = largura_do_texto(3, BUF as *const u8) + 0x73;
            if largura >= 0x2f1 {
                largura = 0x2f1;
            }
            let largura = largura as usize;
            if selecao(*(G_RESERVE_COMP as *const usize), comp) == 0 {
                let base_id = *(rd(fx + 0x14) as *const u32);
                let id = (((base_id << 11) | (w32 as u32 & 0x7ff)) << 11) | (w30 as u32 & 0x7ff);
                linha(display_do_humano(), 2, 0x73, 0, largura, 0, 0, k as usize, 1, 0, 0, 1, 3, cor_sobre(id as usize, 0xaebdec), BUF, 0, 0x7e5, id as usize, tabela);
            } else {
                linha(display_do_humano(), 1, 0x73, 0, largura, 0, 0, k as usize, 1, 0, 0, 1, 3, cor_em(0xaebdec), BUF, 0, 0, 0, tabela);
            }
            linha(display_do_humano(), 1, 0x73, 0, largura, 0, 0, (k + 1) as usize, 0xc400, cor_em(0xaebdec), 0, 0, 0, 0, 0, 0, 0, 0, tabela);
            k += 2;
        }
        let (c1, c2) = (rd(fx + 0x1c), rd(fx + 0x20));
        if humano_manda(c1, 0, 1) != 0 || humano_manda(c2, 0, 1) != 0 {
            alvo = k;
        } else if alvo == -1 && (humano(c1, 0, 1, 0, 0) != 0 || humano(c2, 0, 1, 0, 0) != 0) {
            alvo = k;
        }
        let partida_na_tabela: unsafe extern "C" fn(usize, usize, *mut i32, u32) = core::mem::transmute(FN_COMP_SCREENS_ADD_FIXTURE_ROW);
        partida_na_tabela(fx, tabela, &mut k, 2);
        anterior = fx;
        j += 1;
    }

    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_FORWARD_UNAVAILABLE);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_BACK_UNAVAILABLE);
    let avancar = (sem_avancar(sm) == 0) as u32;
    let voltar = (sem_voltar(sm, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_BACK_NEXT_BUTTONS);
    botoes(voltar, avancar);

    let item = tabela as u16 as i16;
    let rolar = |pos: usize| {
        if item != -1 {
            let ir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AREA_SET_SCROLL_ROW);
            ir((display_do_humano() as isize + item as isize * 3057 + 4) as usize, pos);
        }
    };
    if parametro_da_tela(4) != 0 {
        if alvo > -1 {
            let p = alvo - (n >> 1) as i32;
            rolar(if p <= 0 { 0 } else { p as usize });
        } else {
            rolar(parametro_da_tela(2));
        }
        gravar_parametro_da_tela(3, item as i32 as usize);
        gravar_parametro_da_tela(4, 0);
    } else {
        rolar(parametro_da_tela(2));
        gravar_parametro_da_tela(3, item as i32 as usize);
    }
    *((grupo as isize + 0x14 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as *mut u32) = 1;
}

/// Como `comparar_painel_pela_lista`, com mais regiões (o que a rotina grava fora da tela).
pub(crate) unsafe fn comparar_pela_lista_com(nome: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = regioes_da_tela();
    reg.extend(regioes_dos_parametros());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, || crate::aleatorio::sem_contar(rust));
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

/// As zonas de ação são comparadas de verdade, com o `*topo` (+0x19), a cor fixa 0xb6b7a4 e a guarda
/// 0xb6b7b4 e a foto da partida que os valores (0x720320) comparam e regravam (parâmetro 0x1d, 0xd7a
/// bytes). Sem a foto ainda (o primeiro desenho), os valores a alocam: só a original roda.
pub(crate) unsafe extern "C" fn comparar_draw_action_zones_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_ACTION_ZONES_PANEL);
    let foto = parametro_da_tela(0x1d);
    if foto == 0 {
        // ponytail: a alocação não roda duas vezes; esse desenho se prova no modo Rust
        return o(topo, base);
    }
    let extra = [(topo as usize, 4), (0xb6b7a4, 2), (0xb6b7b4, 1), (foto, 0xd7a)];
    comparar_pela_lista_com("draw_action_zones_panel", &extra, || o(topo, base), || draw_action_zones_panel(topo, base));
}

/// O relatório é comparado de verdade quando já existe (na partida ou no parâmetro 0x2d); quando nasce
/// aqui (alocação e construção) só a original roda.
pub(crate) unsafe extern "C" fn comparar_draw_match_report_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_REPORT_PANEL);
    if parametro_da_tela(0) == 0 && *((parametro_da_tela(0x20) + 0x4c) as *const u8) == 0 && parametro_da_tela(0x2d) == 0 {
        // ponytail: a alocação do relatório não roda duas vezes; esse caminho se prova no modo Rust
        return o(topo, base);
    }
    comparar_painel_pela_lista("draw_match_report_panel", || o(topo, base), || draw_match_report_panel(topo, base));
}

/// A montagem é comparada de verdade, com a marca de vista do humano no grupo (+0x14 + 4h).
pub(crate) unsafe extern "C" fn comparar_build_day_fixtures_screen() {
    let o = orig::<unsafe extern "C" fn()>(IDX_BUILD_DAY_FIXTURES_SCREEN);
    let marca = (parametro_da_tela(1) as isize + 0x14 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as usize;
    comparar_pela_lista_com("build_day_fixtures_screen", &[(marca, 4)], || o(), || build_day_fixtures_screen());
}

// ---------- 203a leva: as telas da partida (11): os valores das estatísticas da partida, das zonas de ação e das notas dos jogadores (match_screens.cpp) ----------

/// Guarda em `copia` (o parâmetro 0x1d) o estado da partida `estado` (0xd7a bytes), como as três
/// rotinas de valores fazem no fim (a cópia é a 0x71dd70 / 0x71df00 em linha): campos de 4, 2 e 1
/// bytes até 0x11f, os dois blocos de time de 0x625 bytes e o fim 0xd69..0xd7a. Sem cópia ainda:
/// aloca (0x944cf1), constrói os dois blocos (0x9463af com 0x453c30) e grava no parâmetro 0x1d.
pub(crate) unsafe fn guardar_estado_da_partida(mut copia: usize, estado: usize) {
    if copia == 0 {
        crate::comparacao::ramo("telas cria a copia do estado da partida");
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
        copia = novo(0xd7a);
        if copia != 0 {
            vetor(copia + 0x11f, 0x625, 2, 0x453c30, 0x453c10);
        }
        gravar_parametro_da_tela(0x1d, copia);
    }
    copiar_campos(copia, estado, 0, 8, 4);
    copiar_campos(copia, estado, 8, 0x34, 2);
    copiar_campos(copia, estado, 0x34, 0x4c, 4);
    copiar_campos(copia, estado, 0x4c, 0x4d, 1);
    copiar_campos(copia, estado, 0x4d, 0x53, 2);
    copiar_campos(copia, estado, 0x53, 0x11f, 4);
    for t in [0x11f, 0x11f + 0x625] {
        let (d, s) = (copia + t, estado + t);
        copiar_campos(d, s, 0, 0x11, 1);
        copiar_campos(d, s, 0x11, 0x1f, 2);
        copiar_campos(d, s, 0x1f, 0x35, 1);
        copiar_campos(d, s, 0x35, 0x61, 2);
        copiar_campos(d, s, 0x61, 0x69, 4);
        copiar_campos(d, s, 0x69, 0x71, 2);
        for j in 0..20 {
            match_player_row_assign(d + 0x71 + j * 0x49, s + 0x71 + j * 0x49);
        }
    }
    copiar_campos(copia, estado, 0xd69, 0xd75, 4);
    copiar_campos(copia, estado, 0xd75, 0xd76, 1);
    copiar_campos(copia, estado, 0xd76, 0xd7a, 4);
}

/// O estado da partida que as rotinas de valores mostram: na partida (parâmetro 0) [p0 + 0x5f], fora
/// dela o parâmetro 0x20 (lido depois dos outros, como a original).
pub(crate) unsafe fn estado_da_partida_na_tela(p0: usize) -> usize {
    if p0 != 0 {
        core::ptr::read_unaligned((p0 + 0x5f) as *const usize)
    } else {
        parametro_da_tela(0x20)
    }
}

pub(crate) const TEXTO_VAZIO: usize = 0xad9c64;
pub(crate) const FORMATO_D: usize = 0x98806c; // "%d"
pub(crate) const FORMATO_PCT: usize = 0x98e8d4; // "%d%%"

/// sends the match stats cells the values that changed since the match state copy (param 0x1d) and updates the copy
/// Original: `refresh_match_stats_values` (0x71e750, match_screens.cpp, cdecl).
///
/// [pt] { 0x71e750 } (cdecl, match_screens.cpp): passa às células das estatísticas da partida (a partir do
/// item do parâmetro 9) os valores do estado que mudaram desde a cópia do parâmetro 0x1d (sem cópia,
/// todos): os bytes dos dois times ("%d"; +0x127.. e +0x74c..), as duas diferenças +0x127 − +0x128 e
/// +0x74c − +0x74d e seis percentuais (num × 100 / den, "%d%%", ou "-" com den 0; o último compara
/// +0x75d mas mostra 100 − o do +0x138, como a original). No fim guarda o estado na cópia.
pub(crate) unsafe extern "C" fn refresh_match_stats_values() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let p0 = parametro_da_tela(0);
    let item = parametro_da_tela(9) as u32;
    let copia = parametro_da_tela(0x1d);
    let e = estado_da_partida_na_tela(p0);
    let u8_ = |b: usize, k: usize| *((b + k) as *const u8) as i32;
    let i16_ = |b: usize, k: usize| core::ptr::read_unaligned((b + k) as *const i16) as i32;
    let byte = |k: usize, d: u32| {
        if copia == 0 || u8_(e, k) != u8_(copia, k) {
            formatar(BUF, FORMATO_D, u8_(e, k));
            texto(sm, item.wrapping_add(d), BUF);
        }
    };
    let diferenca = |a: usize, b: usize, d: u32| {
        if copia == 0 || u8_(e, a) - u8_(e, b) != u8_(copia, a) - u8_(copia, b) {
            formatar(BUF, FORMATO_D, u8_(e, a) - u8_(e, b));
            texto(sm, item.wrapping_add(d), BUF);
        }
    };
    let percentual = |muda: usize, den: usize, num: usize, d: u32, inverter: bool| {
        if copia == 0 || i16_(e, muda) != i16_(copia, muda) {
            if i16_(e, den) == 0 {
                *(BUF as *mut [u8; 2]) = *b"-\0";
            } else {
                let x = i16_(e, num) * 100 / i16_(e, den);
                formatar(BUF, FORMATO_PCT, if inverter { 100 - x } else { x });
            }
            texto(sm, item.wrapping_add(d), BUF);
        }
    };

    byte(0x127, 0);
    byte(0x74c, 2);
    byte(0x128, 3);
    byte(0x74d, 5);
    diferenca(0x127, 0x128, 6);
    diferenca(0x74c, 0x74d, 8);
    for (k, d) in [(0x129, 9), (0x74e, 0xb), (0x12a, 0xc), (0x74f, 0xe), (0x12b, 0xf), (0x750, 0x11), (0x12c, 0x12), (0x751, 0x14), (0x12d, 0x15), (0x752, 0x17)] {
        byte(k, d);
    }
    percentual(0x130, 0x130, 0x132, 0x18, false);
    percentual(0x755, 0x755, 0x757, 0x1a, false);
    percentual(0x134, 0x134, 0x136, 0x1b, false);
    percentual(0x759, 0x759, 0x75b, 0x1d, false);
    percentual(0x138, 0x138, 0x13a, 0x1e, false);
    percentual(0x75d, 0x138, 0x13a, 0x20, true);
    for (k, d) in [(0x12e, 0x21), (0x753, 0x23), (0x12f, 0x24), (0x754, 0x26)] {
        byte(k, d);
    }
    guardar_estado_da_partida(copia, e);
}

/// updates the action zones: markers, possession, ball in play time, the three zone bars and the possession bar in the
/// club colours; updates the state copy
/// Original: `refresh_action_zones_values` (0x720320, match_screens.cpp, cdecl).
///
/// [pt] { 0x720320 } (cdecl, match_screens.cpp): passa às células das zonas de ação os valores que mudaram
/// desde a cópia do parâmetro 0x1d (sem cópia, todos). Os cinco marcadores (parâmetro 0x23 + 0..4;
/// 0x801350 1 e 0x801fc0 4) quando a soma dos bytes +0x767/+0x142, +0x763/+0x13e .. +0x766/+0x141 não é
/// zero (o primeiro só na partida). Pelo lado do parâmetro 0xa (i8): a posse em três (+0xc, +0x16,
/// +0x20, "%d%%" truncado a i8; o terceiro é 100 − os outros) no item do parâmetro 0xb; o tempo de bola
/// em jogo (+0x2a × 60 / 11 segundos, vazio no lado 4); as cores dos dois clubes (0x53b640 com o
/// kit +0x152 e +0x777); as três zonas do campo (+0x143, +0x148, +0x14d; 50 sem valor ou sem a
/// marca +0x768/+0x76d/+0x772) em barras entre os parâmetros 0x16 e 0x18 na altura do 0x19; e a barra
/// da posse (+0x13e/+0x763) de 0xd4 a 0x2a6 na altura do parâmetro 0xc. No fim guarda o estado.
pub(crate) unsafe extern "C" fn refresh_action_zones_values() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let retangulo: unsafe extern "thiscall" fn(usize, u32, i32, i32, i32, i32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_RECT);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_COLOUR);
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT_FLAGS);
    let estilo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_ID);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u8, *mut u8) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let p0 = parametro_da_tela(0);
    let lado = parametro_da_tela(0xa) as u8 as i8 as isize;
    let b = parametro_da_tela(0xb) as u32;
    let altura = parametro_da_tela(0xc) as i32;
    let esq = parametro_da_tela(0x16) as i32;
    let dir = parametro_da_tela(0x18) as i32;
    let base = parametro_da_tela(0x19) as i32;
    let copia = parametro_da_tela(0x1d);
    let marcas = parametro_da_tela(0x23) as u32;
    let e = estado_da_partida_na_tela(p0);
    let u8_ = |x: usize, k: usize| *((x + k) as *const u8) as u32;
    let it = |d: u32| b.wrapping_add(d);

    for (i, a, c) in [(0u32, 0x767, 0x142), (1, 0x763, 0x13e), (2, 0x764, 0x13f), (3, 0x765, 0x140), (4, 0x766, 0x141)] {
        if (i > 0 || p0 != 0) && u8_(e, a) + u8_(e, c) != 0 {
            ligar(sm, marcas.wrapping_add(i), 1);
            estilo(sm, marcas.wrapping_add(i), 4);
        }
    }

    let s = |x: usize, k: isize| core::ptr::read_unaligned((x as isize + lado * 2 + k) as *const i16) as i32;
    if copia == 0 || s(e, 0xc) != s(copia, 0xc) || s(e, 0x16) != s(copia, 0x16) || s(e, 0x20) != s(copia, 0x20) {
        let total = || s(e, 0xc) + s(e, 0x16) + s(e, 0x20);
        let a = if total() == 0 { 0 } else { (s(e, 0xc) * 100 / total()) as i8 as i32 };
        formatar(BUF, FORMATO_PCT, a);
        texto(sm, b, BUF);
        let c = if total() == 0 { 0 } else { (s(e, 0x20) * 100 / total()) as i8 as i32 };
        formatar(BUF, FORMATO_PCT, c);
        texto(sm, it(2), BUF);
        formatar(BUF, FORMATO_PCT, if total() == 0 { 0 } else { 100 - c - a });
        texto(sm, it(1), BUF);
    }
    if copia == 0 || s(e, 0x2a) != s(copia, 0x2a) {
        if lado == 4 {
            *(BUF as *mut u8) = 0;
        } else {
            let x = s(e, 0x2a) * 60 / 11;
            traduzido(BUF, 0xa168b4, x / 60, x % 60);
        }
        texto(sm, it(3), BUF);
    }

    // as cores dos clubes: a primeira saída (+0x3c na pilha da original) é a cor da barra; a segunda
    // saída de cada chamada não é usada. Só os 16 bits de baixo valem (0x801040 lê uma palavra).
    let clube = |k: usize| (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((e + k) as *const usize)).wrapping_mul(0x245));
    let (mut cor1, mut cor2, mut resto1, mut resto2) = ([0u8; 2], [0u8; 2], [0u8; 2], [0u8; 2]);
    cores_do_clube(clube(0), u8_(e, 0x152), cor1.as_mut_ptr(), resto1.as_mut_ptr());
    cores_do_clube(clube(4), u8_(e, 0x777), cor2.as_mut_ptr(), resto2.as_mut_ptr());
    let cor = |pct: u32| u16::from_le_bytes(if pct < 0x32 { cor2 } else { cor1 }) as u32;
    let valor = |campo: isize| *((e as isize + lado + campo) as *const u8);
    let mudou = |campo: isize| copia == 0 || valor(campo) != *((copia as isize + lado + campo) as *const u8);
    let pct = |campo: isize, marca: isize| if valor(campo) == 0 || valor(marca) == 0 { 0x32 } else { valor(campo) as u32 };
    let zona = |campo: isize, marca: isize, ini: i32, fim: i32, d: u32| {
        let p = pct(campo, marca);
        let x = (fim - ini - 4) * p as i32 / 100 + ini + 2;
        retangulo(sm, it(d), ini + 2, base - 0x12, x, base - 0xc);
        retangulo(sm, it(d + 1), x, base - 0x12, fim - 2, base - 0xc);
        fundo(sm, it(d + 2), cor(p));
    };
    if mudou(0x143) {
        zona(0x143, 0x768, esq + 10, (dir - esq) / 3 + esq - 10, 6);
    }
    if mudou(0x148) {
        zona(0x148, 0x76d, (dir - esq) / 3 + esq + 10, (dir - esq) * 2 / 3 + esq - 10, 0xb);
    }
    if mudou(0x14d) {
        zona(0x14d, 0x772, (dir - esq) * 2 / 3 + esq + 10, dir - 10, 0x10);
    }
    if mudou(0x13e) {
        let p = pct(0x13e, 0x763);
        formatar(BUF, FORMATO_PCT, p);
        let w = (p * 0x1d2 / 100) as i32 + 0xd4;
        retangulo(sm, it(0x15), 0xd4, altura - 0x23, w, altura - 0x14);
        texto(sm, it(0x15), BUF);
        formatar(BUF, FORMATO_PCT, 100u32.wrapping_sub(p));
        retangulo(sm, it(0x16), w, altura - 0x23, 0x2a6, altura - 0x14);
        texto(sm, it(0x16), BUF);
        fundo(sm, it(0x17), cor(p));
    }
    guardar_estado_da_partida(copia, e);
}

/// updates the player ratings rows: cards, injury/dismissal/substitution minute, rating, goals and name colour; updates
/// the state copy
/// Original: `refresh_player_ratings_rows` (0x723350, match_screens.cpp, cdecl).
///
/// [pt] { 0x723350 } (cdecl, match_screens.cpp): passa às linhas das notas dos jogadores (7 itens por linha a
/// partir do parâmetro 0xd; 20 linhas por time, a linha só se o item + 6 < 0x4b0, senão pula sem andar)
/// os valores que mudaram desde a cópia do parâmetro 0x1d (sem cópia, todos). Por linha (73 bytes em
/// +0x190): vazia (0xfb em +0x1d) limpa os itens; senão as margens/fundos dos dois cartões (+0x2c e
/// +0x2d), o minuto de lesão, expulsão, saída ou entrada (+0x33, +0x34, +0x30, +0x32, "inj/off/sub/on
/// <%d - minute>", com cor), a nota (+0x35) e os gols (+0x21; vazios para o reserva que não entrou) e a
/// cor do nome (o jogador em +4 igual ao [estado + 0x3c], quem saiu, o reserva que não entrou). No fim
/// guarda o estado.
pub(crate) unsafe extern "C" fn refresh_player_ratings_rows() {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzido: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let texto: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_COLOUR);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_STYLE);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT_COLOUR);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let p0 = parametro_da_tela(0);
    let mut item = parametro_da_tela(0xd) as u32;
    let copia = parametro_da_tela(0x1d);
    let e = estado_da_partida_na_tela(p0);
    let atual = core::ptr::read_unaligned((e + 0x3c) as *const u32);
    for t in 0..2usize {
        for j in 0..20usize {
            if item as i16 as i32 + 6 > 0x4af {
                continue;
            }
            let lin = j * 0x49;
            let i = item.wrapping_add(1);
            let r = e + t * 0x625 + lin + 0x190;
            let c = if copia != 0 { copia + t * 0x625 + lin + 0x190 } else { 0 };
            let er = |k: usize| *((r + k) as *const u8);
            let cr = |k: usize| *((c + k) as *const u8);
            let difere = |ks: &[usize]| c == 0 || ks.iter().any(|&k| er(k) != cr(k));
            if er(0x1d) == 0xfb {
                if c == 0 {
                    margem(sm, i, 1);
                    margem(sm, i + 1, 1);
                    texto(sm, i + 3, TEXTO_VAZIO);
                    texto(sm, i + 4, TEXTO_VAZIO);
                    texto(sm, i + 5, TEXTO_VAZIO);
                }
            } else {
                if difere(&[0x2c, 0x2d]) {
                    let (a, d) = (er(0x2c), er(0x2d));
                    if a != 0 && d != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, cor_em(0xaebdec) as u32);
                        margem(sm, i + 1, 0x40030);
                        fundo(sm, i + 1, cor_em(0xae31a8) as u32);
                    } else if a == 0 && d != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, cor_em(0xae31a8) as u32);
                        margem(sm, i + 1, 1);
                    } else if a != 0 {
                        margem(sm, i, 0x20030);
                        fundo(sm, i, cor_em(0xaebdec) as u32);
                        margem(sm, i + 1, 1);
                    } else {
                        margem(sm, i, 1);
                        margem(sm, i + 1, 1);
                    }
                }
                let m = |k: usize| er(k) as i8;
                if difere(&[0x33, 0x34, 0x30, 0x32]) {
                    let minuto = if m(0x33) >= 0 {
                        Some((0xa16a68usize, m(0x33), 0xae3154usize)) // "inj <%d - minute>"
                    } else if m(0x34) >= 0 {
                        Some((0xa16a54, m(0x34), 0xae3154)) // "off <%d - minute>"
                    } else if m(0x30) >= 0 {
                        Some((0xa16a40, m(0x30), 0xae3154)) // "sub <%d - minute>"
                    } else if m(0x32) >= 0 {
                        Some((0xa16a2c, m(0x32), 0xaebdd4)) // "on <%d - minute>"
                    } else {
                        None
                    };
                    match minuto {
                        Some((f, v, cor)) => {
                            traduzido(BUF, f, v as i32 + 1);
                            cor_do_texto(sm, i + 3, cor_em(cor) as u32);
                            texto(sm, i + 3, BUF);
                        }
                        None => texto(sm, i + 3, TEXTO_VAZIO),
                    }
                }
                if lin >= 0x323 && m(0x32) < 0 {
                    // o reserva que não entrou: só limpa, e só sem cópia
                    if c == 0 {
                        texto(sm, i + 4, TEXTO_VAZIO);
                        texto(sm, i + 5, TEXTO_VAZIO);
                    }
                } else {
                    if difere(&[0x35, 0x32]) {
                        formatar(BUF, FORMATO_D, er(0x35) as i32);
                        texto(sm, i + 4, BUF);
                    }
                    if difere(&[0x21, 0x32]) {
                        if er(0x21) != 0 {
                            formatar(BUF, FORMATO_D, er(0x21) as i32);
                            texto(sm, i + 5, BUF);
                        } else {
                            texto(sm, i + 5, TEXTO_VAZIO);
                        }
                    }
                }
                let cor = if core::ptr::read_unaligned((r + 4) as *const u32) == atual {
                    0xaebde8
                } else if m(0x33) >= 0 || m(0x34) >= 0 || m(0x30) >= 0 || (lin > 0x2da && m(0x32) == -1) {
                    0xae317e
                } else {
                    0xae3184
                };
                if difere(&[0x33, 0x34, 0x30, 0x32]) || atual != core::ptr::read_unaligned((copia + 0x3c) as *const u32) {
                    cor_do_texto(sm, i + 2, cor_em(cor) as u32);
                }
            }
            item = item.wrapping_add(7);
        }
    }
    guardar_estado_da_partida(copia, e);
}

/// As três rotinas são comparadas de verdade quando já existe a cópia do estado (parâmetro 0x1d): as
/// duas atualizam a partir do mesmo estado e o gerente de telas (com a posição do buffer de mensagens
/// em +0x3802, que cada atualização de item zera e preenche), o `Display` da tela atual, o bloco de
/// parâmetros, a cópia do estado (0xd7a bytes) e o texto 0xde1f64 têm de sair iguais. O conteúdo do
/// buffer de mensagens ([gerente + 0x37fe]) não entra (como na leva das listas).
pub(crate) unsafe fn comparar_valores_da_partida(nome: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let copia = parametro_da_tela(0x1d);
    if copia == 0 {
        // ponytail: sem cópia a rotina aloca a cópia (0x944cf1) e a grava no parâmetro 0x1d; não dá
        // para rodar duas vezes. Só a original; a cópia Rust prova esse ramo no modo Rust.
        return original();
    }
    let mut reg = regioes_da_tela();
    reg.extend(regioes_dos_parametros());
    reg.push((copia, 0xd7a));
    reg.push((0xde1f64, 0x100));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, rust);
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

pub(crate) unsafe extern "C" fn comparar_refresh_match_stats_values() {
    let o = orig::<unsafe extern "C" fn()>(IDX_REFRESH_MATCH_STATS_VALUES);
    comparar_valores_da_partida("refresh_match_stats_values", || o(), || refresh_match_stats_values());
}

pub(crate) unsafe extern "C" fn comparar_refresh_action_zones_values() {
    let o = orig::<unsafe extern "C" fn()>(IDX_REFRESH_ACTION_ZONES_VALUES);
    comparar_valores_da_partida("refresh_action_zones_values", || o(), || refresh_action_zones_values());
}

pub(crate) unsafe extern "C" fn comparar_refresh_player_ratings_rows() {
    let o = orig::<unsafe extern "C" fn()>(IDX_REFRESH_PLAYER_RATINGS_ROWS);
    comparar_valores_da_partida("refresh_player_ratings_rows", || o(), || refresh_player_ratings_rows());
}

// ---------- 202a leva: as telas da partida (8): os painéis das estatísticas do time, das notas dos jogadores e das estatísticas da partida (match_screens.cpp) ----------

pub(crate) type CelulaDaLista = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type TextoDaCelulaDaLista = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
pub(crate) type ListaDoDisplay = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;

/// O `Display` do humano da vez ([0xb74c4c + 0x300 x (i16)[0xb77c7a]]).
pub(crate) unsafe fn display_do_humano() -> usize {
    *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize)
}

/// A cor (u16) guardada em `a`, estendida com zero. Onde a original empurra um registrador inteiro com
/// lixo imprevisível nos 16 bits de cima (o que sobrou de uma chamada), vai o valor estendido.
pub(crate) unsafe fn cor_em(a: usize) -> usize {
    *(a as *const u16) as usize
}

/// draws one team's stats table (page header, arrows, column titles) or the no-stats message
/// Original: `draw_team_stats_panel` (0x7217a0, match_screens.cpp, cdecl).
///
/// [pt] { 0x7217a0 } (cdecl, match_screens.cpp): desenha a tabela das estatísticas de um time de `*topo` a
/// `*base`. Fora da partida (parâmetro 0 nulo) e sem estatísticas ([parâmetro 0x20 + 0x4c] ≠ 0): só a
/// mensagem "There are no team stats available." (0x5ed070, as linhas que cabem na fonte 3). Senão: a
/// moldura (0x55fba0), a tabela do cabeçalho de 22 colunas, as setas de página nas colunas 4 e 5 ("<<"
/// e ">>", ou '*' e '+' pela fonte de símbolos [0x9cc2c4] ≠ -1; o item da 4 no parâmetro 0x1a), a seta
/// da página atual (parâmetro 0x1c: 1 ou 2) desligada (0x800580), os títulos das colunas 7 a 0x13 pela
/// tabela da página (0xa16178 / 0xa1618c; o texto em [0xa16118 + 4k], com os títulos de comentário
/// próprios de cada página), a tabela dos valores, o lado `lado` no parâmetro 0x1b, -1 no 0x1a e o
/// preenchimento (0x721cd0).
pub(crate) unsafe extern "C" fn draw_team_stats_panel(lado: u32, topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das estatisticas do time");
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let pagina = parametro_da_tela(0x1c) as u8;
    let mut cabecalho: [u8; 22] = [1, 1, 1, 0x49, 8, 8, 3, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xe, 0];
    let mut valores: [u8; 22] = [0, 0xe, 1, 5, 1, 0x35, 0x15, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 7, 7];
    if parametro_da_tela(0) == 0 && *((parametro_da_tela(0x20) + 0x4c) as *const u8) != 0 {
        let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
        let a = altura(3);
        let n = ((*base - *topo - 0x10) / a) as u8;
        texto(BUF, 0xa16a08);
        let mensagem: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
            core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
        mensagem(0x6e, *topo as usize, 0x30c, *base as usize, 2, 0, n as usize, 3, cor_em(0xae3184), BUF, 0);
        return;
    }

    lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 1, 0, 1, 0, 2, 0, usize::MAX);
    let tabela = lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x16, cabecalho.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    // a seta "<<": com a fonte normal a original empurra eax = [0x9cc2c4] (-1) com a cor em ax
    let r = if *(G_FONT_MODE as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x2a);
        celula(sm, tabela, 4, 0, 0x30, cor_em(0xaebe04), 0xc, 0, cor_em(0xae3184), BUF, 9, 0)
    } else {
        celula(sm, tabela, 4, 0, 0x30, cor_em(0xaebe04), 0xc, 1, 0xffff_0000 | cor_em(0xae3184), 0x98e7c4, 9, 0)
    };
    gravar_parametro_da_tela(0x1a, r as u16 as i16 as i32 as usize);
    if *(G_FONT_MODE as *const i32) != -1 {
        formatar(BUF, 0x99ec1c, 0x2b);
        celula(sm, tabela, 5, 0, 0x30, cor_em(0xaebe04), 0xc, 0, cor_em(0xae3184), BUF, 0xa, 0);
    } else {
        celula(sm, tabela, 5, 0, 0x30, cor_em(0xaebe04), 0xc, 1, cor_em(0xae3184), 0x98e7a8, 0xa, 0);
    }
    // a tabela das colunas da página; numa página fora de 1 e 2 a original lê pelo valor do ponteiro
    // `base` (o lugar do argumento é que recebe a tabela), e nenhum título sai
    let (colunas, seta) = match pagina {
        1 => (0xa16178usize, r),
        2 => (0xa1618c, r.wrapping_add(1)),
        _ => (base as usize, r.wrapping_add(1)),
    };
    let desligar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_FLAGS);
    desligar(sm, seta, 1);
    for c in 7..0x14usize {
        let k = *((colunas + c - 2) as *const u8) as usize;
        let rotulo = *((0xa16118 + k * 4) as *const usize);
        if rotulo == 0 {
            continue;
        }
        let proprio = match pagina {
            1 => match c {
                0xb => Some(0xa16968usize),
                0xe => Some(0xa169e8),
                0x11 => Some(0xa169cc),
                0x12 => Some(0xa169a8),
                0x13 => Some(0xa16988),
                _ => None,
            },
            2 => match c {
                9 => Some(0xa16968usize),
                0xc => Some(0xa169cc),
                0xd => Some(0xa169a8),
                0xe => Some(0xa16944),
                0x10 => Some(0xa16924),
                0x13 => Some(0xa16988),
                _ => None,
            },
            _ => continue,
        };
        match proprio {
            Some(f) => texto(BUF, f, 0xad9c64),
            None => texto(BUF, rotulo, 0),
        };
        texto_da_celula(sm, tabela, c, 0, 0x30, cor_em(0xae317e), 0xc, 1, cor_em(0xae3184), BUF);
    }
    let tabela_dos_valores = lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x16, valores.as_mut_ptr() as usize, 0x15, 0, 1, 0, usize::MAX);
    gravar_parametro_da_tela(0x1b, lado as u8 as i8 as i32 as usize);
    gravar_parametro_da_tela(0x1a, usize::MAX);
    let preencher: unsafe extern "C" fn(u32, usize) = core::mem::transmute(FN_FILL_TEAM_STATS_ROWS);
    preencher(1, tabela_dos_valores);
}

/// draws both teams' player ratings: shirt number, name (button, with the captain) and rating
/// Original: `draw_player_ratings_panel` (0x722ed0, match_screens.cpp, cdecl).
///
/// [pt] { 0x722ed0 } (cdecl, match_screens.cpp): desenha as notas dos jogadores dos dois times de `*topo` a
/// `*base`: uma tabela de 16 colunas (8 por time) e 20 linhas por time. Os dados: na partida (parâmetro
/// 0) [p0 + 0x5f] e a partida p0 + 4; fora dela os parâmetros 0x20 e 0x1f. Por linha `j` do time `i`
/// (registro de 73 bytes em dados + 1573i + 73j + 0x190): o número da camisa (ou j + 1) com fundo por
/// titular/reserva (as 11 primeiras e até 0xb + [partida+0x4d]&0xf; o item da primeira no parâmetro
/// 0xd), dois separadores, o nome (0x53e140; "(c)" no capitão, [dados + 1573i + 0x153]) como botão
/// (0x8040a0, item 0x7dc ou 0x7d7 pelo saldo de [dados] ou [dados+4]) quando a pessoa vale (0x53e990),
/// senão texto; vazio (0xfb em +0x1d) sem nome; e as três células da nota. No fim, 0x723350.
pub(crate) unsafe extern "C" fn draw_player_ratings_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das notas dos jogadores");
    let celula: CelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let cor_reserva: unsafe extern "C" fn(u32, u32, u32, u32) -> usize = core::mem::transmute(FN_GRAPHICS_RGB_TO_PIXEL);
    let nome_da_pessoa: unsafe extern "C" fn(u32, u32, u32, usize, u32, u32, i32) = core::mem::transmute(FN_DATABASE_FORMAT_NAME_FROM_IDS);
    let pessoa_vale: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 16] = [0xe, 1, 5, 1, 0x46, 0x15, 7, 7, 0xe, 1, 5, 1, 0x46, 0x15, 7, 7];
    let p0 = parametro_da_tela(0);
    let (dados, partida) = if p0 != 0 {
        (core::ptr::read_unaligned((p0 + 0x5f) as *const usize), p0 + 4)
    } else {
        let d = parametro_da_tela(0x20);
        (d, parametro_da_tela(0x1f))
    };
    let tabela = lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 0x10, colunas.as_mut_ptr() as usize, 0x14, 0, 2, 0, usize::MAX);
    for i in 0..2usize {
        let col = 8 * i;
        for j in 0..0x14usize {
            let reg = dados + 1573 * i + 73 * j + 0x190;
            let camisa = *((reg + 0x1c) as *const i8);
            formatar(BUF, 0x98806c, if camisa > 0 { camisa as i32 } else { j as i32 + 1 });
            let starters = (*((partida + 0x4d) as *const u8) & 0xf) as usize + 0xb;
            // as 11 primeiras: a original empurra eax = a partida com a cor em ax
            let fundo = if j < 0xb { (partida & 0xffff_0000) | cor_em(0xaebe04) } else { cor_reserva(0, 0, 0x40, 0) };
            let r = if j < starters {
                texto_da_celula(sm, tabela, col, j, 0x30, fundo, 0xc, 1, cor_em(0xae3184), BUF)
            } else {
                texto_da_celula(sm, tabela, col, j, 0x30, fundo, 0x2c, 1, cor_em(0xae3184), 0xad9c64)
            };
            if i == 0 && j == 0 {
                gravar_parametro_da_tela(0xd, r as u16 as i16 as i32 as usize);
            }
            texto_da_celula(sm, tabela, col + 2, j, 1, cor_em(0xaebdec), 0xc, 1, cor_em(0xaebdea), 0);
            texto_da_celula(sm, tabela, col + 2, j, 1, cor_em(0xae31a8), 0xc, 1, cor_em(0xaebdea), 0);
            if *((reg + 0x1d) as *const u8) == 0xfb {
                texto_da_celula(sm, tabela, col + 4, j, 1, 0, 1, 2, cor_em(0xae317e), 0);
            } else {
                let id = core::ptr::read_unaligned(reg as *const i32);
                let a = core::ptr::read_unaligned((reg + 8) as *const u32);
                let b = core::ptr::read_unaligned((reg + 0xc) as *const u32);
                let c = core::ptr::read_unaligned((reg + 0x10) as *const u32);
                let mut nome = [0u8; 0xc8];
                nome_da_pessoa(a, b, c, nome.as_mut_ptr() as usize, 0xc8, 1, id);
                let capitao = *((dados + 1573 * i + 0x153) as *const i8) as i32;
                if j as i32 == capitao {
                    texto(BUF, 0xa16a7c, nome.as_ptr() as usize);
                } else {
                    copiar_para_o_texto(nome.as_ptr() as usize);
                }
                let mut botao = false;
                if id >= 0 && id < *(G_STAFF_COUNT as *const i32) {
                    let pessoa = (*(G_STAFF_TABLE as *const usize) as isize + id as isize * 110) as usize;
                    if pessoa_vale(pessoa, a, b, c, reg + 0x14) != 0 {
                        botao = true;
                        let saldo = if i == 0 { *(dados as *const i32) } else { *((dados + 4) as *const i32) };
                        let limite = (*(G_CLUB_COUNT as *const i32)).wrapping_sub((*(G_NATION_COUNT as *const i32)).wrapping_mul(2));
                        let item = if saldo >= limite { 0x7dc } else { 0x7d7 };
                        celula(sm, tabela, col + 4, j, 1, 0, 1, 2, cor_em(0xae317e), BUF, item, pessoa);
                    }
                }
                if !botao {
                    texto_da_celula(sm, tabela, col + 4, j, 1, 0, 1, 2, cor_em(0xae317e), BUF);
                }
            }
            texto_da_celula(sm, tabela, col + 5, j, 1, 0, 1, 1, cor_em(0xae3154), 0);
            texto_da_celula(sm, tabela, col + 6, j, 1, 0, 0xc, 2, cor_em(0xaebde8), 0);
            texto_da_celula(sm, tabela, col + 7, j, 1, 0, 0xc, 1, cor_em(0xaebdec), 0);
        }
    }
    let preencher: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_PLAYER_RATINGS_ROWS);
    preencher();
}

/// Os rótulos das 13 linhas das estatísticas da partida e as cores deles.
pub(crate) const ROTULOS_DAS_ESTATISTICAS_DA_PARTIDA: [(usize, usize); 13] = [
    (0xa16870, 0xae3184), // Shots On Goal
    (0xa16864, 0xae3184), // On Target
    (0xa16858, 0xae3184), // Off Target
    (0xa16850, 0xae3184), // Corners
    (0xa16844, 0xae3184), // Free Kicks
    (0xa16838, 0xae3184), // Throw-Ins
    (0xa16830, 0xae3184), // Fouls
    (0xa16824, 0xae3184), // Offsides
    (0xa16810, 0xae3184), // Passes Completed
    (0xa16804, 0xae3184), // Tackles Won
    (0xa167f8, 0xae3184), // Headers Won
    (0x99caf4, 0xaebdec), // Yellow Cards
    (0x99cae8, 0xae31a8), // Red Cards
];

/// draws the match stats: 13 labelled rows (shots, corners, fouls, passes, tackles, headers, cards)
/// Original: `draw_match_stats_panel` (0x71e040, match_screens.cpp, cdecl).
///
/// [pt] { 0x71e040 } (cdecl, match_screens.cpp): desenha as estatísticas da partida de `*topo` a `*base`: a
/// tabela de 3 colunas (1, 5, 1) e 13 linhas, cada uma com as células dos dois times (colunas 0 e 2;
/// o item da primeira no parâmetro 9) e o rótulo no meio; depois os valores (0x71e750).
pub(crate) unsafe extern "C" fn draw_match_stats_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel das estatisticas da partida");
    let texto_da_celula: TextoDaCelulaDaLista = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let lista: ListaDoDisplay = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;

    let mut colunas: [u8; 3] = [1, 5, 1];
    let tabela = lista(display_do_humano(), 0x6e, *topo as usize, 0x30c, *base as usize, 3, colunas.as_mut_ptr() as usize, 0xd, 0, 2, 0, usize::MAX);
    for (k, &(rotulo, cor)) in ROTULOS_DAS_ESTATISTICAS_DA_PARTIDA.iter().enumerate() {
        let r = texto_da_celula(sm, tabela, 0, k, 0x30, cor_em(0xaebe04), 0xc, 3, cor_em(0xaebde8), 0);
        if k == 0 {
            gravar_parametro_da_tela(9, r as u16 as i16 as i32 as usize);
        }
        texto(BUF, rotulo);
        texto_da_celula(sm, tabela, 1, k, 1, 0, 0xc, 4, cor_em(cor), BUF);
        texto_da_celula(sm, tabela, 2, k, 0x30, cor_em(0xaebe04), 0xc, 3, cor_em(0xaebde8), 0);
    }
    let preencher: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_MATCH_STATS_VALUES);
    preencher();
}

/// Os três painéis são comparados de verdade: as duas desenham a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual, o bloco de parâmetros com as marcas 0xb6b7a8/ac/b0 e o texto
/// 0xde1f64 têm de sair iguais. A lista (0x55fba0) também escreve num buffer de rede ([0xae2908] na
/// posição [0xae290c]): a posição entra nas regiões (volta para a segunda rodada escrever no mesmo
/// lugar e o tamanho escrito é comparado); o conteúdo desse buffer não.
pub(crate) unsafe fn comparar_painel_pela_lista(nome: &str, original: impl FnOnce(), rust: impl FnOnce()) {
    let mut reg = regioes_da_tela();
    reg.extend(regioes_dos_parametros());
    reg.push((0xde1f64, 0x100));
    reg.push((0xae290c, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, original, rust);
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("{}: diferencas em {:?}", nome, dif)
    });
}

pub(crate) unsafe extern "C" fn comparar_draw_team_stats_panel(lado: u32, topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(u32, *mut i32, *mut i32)>(IDX_DRAW_TEAM_STATS_PANEL);
    comparar_painel_pela_lista("draw_team_stats_panel", || o(lado, topo, base), || draw_team_stats_panel(lado, topo, base));
}

pub(crate) unsafe extern "C" fn comparar_draw_player_ratings_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_PLAYER_RATINGS_PANEL);
    // ponytail: sem a copia do estado (parametro 0x1d) a atualizacao dentro do painel a aloca, e cada
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if parametro_da_tela(0x1d) == 0 {
        return o(topo, base);
    }
    comparar_painel_pela_lista("draw_player_ratings_panel", || o(topo, base), || draw_player_ratings_panel(topo, base));
}

pub(crate) unsafe extern "C" fn comparar_draw_match_stats_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_MATCH_STATS_PANEL);
    // ponytail: sem a copia do estado (parametro 0x1d) a atualizacao dentro do painel a aloca, e cada
    // lado alocaria a sua (enderecos diferentes): so a original; o modo Rust prova esse caminho
    if parametro_da_tela(0x1d) == 0 {
        return o(topo, base);
    }
    comparar_painel_pela_lista("draw_match_stats_panel", || o(topo, base), || draw_match_stats_panel(topo, base));
}

// ---------- 201a leva: as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação (match_screens.cpp) ----------

/// A vaga de tela do humano da vez ([0xb74c26 + 0x300 × humano]).
pub(crate) unsafe fn vaga_de_tela_do_humano() -> usize {
    core::ptr::read_unaligned((0xb74c26 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize)
}

/// O id do item `item` (−1 nenhum: o padrão [0xdd6bde]), como as respostas de match_screens.cpp leem.
pub(crate) unsafe fn id_do_item_ou_padrao(item: i16) -> u16 {
    if item == -1 { *(G_CURRENT_COMMAND_ID as *const u16) } else { id_do_item(item) }
}

/// the match Continue: the pending team selection command (online, the club's free slot or waiting for the others), the
/// network matches and team selection
/// Original: `match_screens_handle_continue_command` (0x725ca0, match_screens.cpp, cdecl).
///
/// [pt] { 0x725ca0 } (cdecl, match_screens.cpp): a resposta ao Continuar da partida (item `item`, −1 nenhum).
/// Em rede ([0xb77c84]): com o comando pendente 1 ([0xdd6be0], zerado com [0xdd6be4]) e a escolha 3,
/// procura na lista de vagas do gerente (0x7ffd80) outra vaga livre (0x7ffdd0) do mesmo clube do humano
/// (0x765480 +0xe2): achou, guarda em [0xb77c4a] e devolve −6; senão, com outros jogadores a esperar
/// (0x803d80), avisa "Waiting for other players..." (0x802f80), abre a tela da partida (0x718aa0) e
/// devolve −4, ou −11. Comando pendente 0x43: abre a escalação (0x725f70) e devolve −4. Depois, só o
/// item de id 0x3e8: com a rede da partida ([0xb67a48]) abre a primeira entrada da lista [0xb67a50]
/// (+0xd, 0x54 bytes; primeiro as de +0x10, depois as de +0xc) que o humano ainda não marcou (0x717cc0,
/// −4) ou devolve −8; sem ela, com ficha e clube, abre a escalação se 0x474e00 negar e devolve −4.
pub(crate) unsafe extern "C" fn match_screens_handle_continue_command(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond ao continuar");
    let g = |a: usize| *(a as *const u32);
    let humano = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let abrir_escalacao: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_SCREENS_OPEN_TEAM_SELECTION);
    let r = ficha(0);
    let clube = if r != 0 { core::ptr::read_unaligned((r + 0xe2) as *const usize) } else { 0 };
    if g(0xb77c84) != 0 {
        let comando = g(0xdd6be0);
        if comando == 1 {
            let escolha = g(0xdd6be4);
            *(G_PENDING_COMMAND_CODE as *mut u32) = 0;
            *(G_PENDING_COMMAND_ARG as *mut u32) = 0;
            if escolha == 3 {
                let listar: unsafe extern "thiscall" fn(usize, *mut usize, *mut i16, u32) = core::mem::transmute(FN_SCREENMANAGER_GET_CHANNEL_MANAGER_KEYS);
                let ocupada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_MANAGER_AWAY);
                let (mut n, mut lista) = (0i16, [0usize; 16]);
                // a original empurra eax inteiro: os 16 bits altos são os da ficha devolvida por 0x765480
                listar(GERENTE_DE_TELAS, lista.as_mut_ptr(), &mut n, (r as u32 & 0xffff_0000) | humano() as u16 as u32);
                let mut i: i8 = 0;
                if n > 0 {
                    loop {
                        let e = *lista.as_ptr().offset(i as isize);
                        if e != vaga_de_tela_do_humano()
                            && ocupada(GERENTE_DE_TELAS, e) == 0
                            && core::ptr::read_unaligned((ficha(e) + 0xe2) as *const usize) == clube
                        {
                            core::ptr::write_unaligned(G_MATCH_WAITING_MANAGER as *mut usize, e);
                            return -6;
                        }
                        i = i.wrapping_add(1);
                        if i as i16 >= n {
                            break;
                        }
                    }
                }
                let esperando: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_CLOSE_ALL_SCREENS_803D80);
                if esperando(GERENTE_DE_TELAS) == 0 {
                    return -11;
                }
                let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                formatar_texto(0xde1f64, 0xa16d94);
                let aviso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SEND_NETWORK_STATUS_MESSAGE);
                // ponytail: a original empurra ecx com 16 bits altos que sobram de 0x66f4e0 (imprevisíveis); aqui zero
                aviso(GERENTE_DE_TELAS, 0xde1f64, 0, 0, humano() as u16 as u32);
                let abrir_partida: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_OPEN_MATCH_SCREEN);
                abrir_partida(vaga_de_tela_do_humano(), clube);
                return -4;
            }
        } else if comando == 0x43 {
            *(G_PENDING_COMMAND_CODE as *mut u32) = 0;
            abrir_escalacao();
            return -4;
        }
    }
    if id_do_item_ou_padrao(item as u16 as i16) != 0x3e8 {
        return 0;
    }
    if g(0xb67a48) != 0 {
        let n = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0x15) as *const i32);
        for modo in 1..=2u8 {
            let base = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0xd) as *const usize);
            let campo = if modo == 1 { 0x10 } else { 0xc };
            let mut i: i8 = 0;
            if n > 0 {
                loop {
                    let e = (base as isize + i as isize * 0x54) as usize;
                    if core::ptr::read_unaligned((e + campo) as *const u32) != 0
                        && core::ptr::read_unaligned((e as isize + humano() as isize * 4 + 0x14) as *const u32) == 0
                    {
                        let abrir: unsafe extern "C" fn(usize, usize) = core::mem::transmute(FN_OPEN_DAY_FIXTURES_SCREEN);
                        abrir(vaga_de_tela_do_humano(), e);
                        return -4;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n {
                        break;
                    }
                }
            }
        }
        return -8;
    }
    if r != 0 && clube != 0 {
        let pronta: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_OPEN_SQUAD_NUMBERS_SCREEN_FOR_MATCH);
        if pronta(r) == 0 {
            abrir_escalacao();
        }
        return -4;
    }
    0
}

/// opens the Team Selection screen for the human's next match and stores the club, the match and the home flag
/// Original: `match_screens_open_team_selection` (0x725f70, match_screens.cpp, cdecl).
///
/// [pt] { 0x725f70 } (cdecl, match_screens.cpp): abre a tela da escalação da próxima partida do humano
/// (0x7fe600 com a montagem 0x7260f0 e a resposta 0x727c30) e grava os parâmetros: 0 o clube (ficha
/// 0x765480 +0xe6), 1 a partida ([ficha+0xe2] + 4) e 2 se o clube joga em casa (+0x1c sem a marca +0x40,
/// ou +0x20 sem a +0x41). Sem ficha, clube ou partida: a caixa de erro (linhas 0x1e1c, 0x1e24, 0x1e2c),
/// como a original (a tela já ficou aberta).
pub(crate) unsafe extern "C" fn match_screens_open_team_selection() {
    crate::comparacao::ramo("telas abrir a escalacao");
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN);
    if abrir(GERENTE_DE_TELAS, 0x7260f0, 0x727c30, 1, 0, 0) == 0 {
        return;
    }
    const FONTE: usize = 0xa161d4; // match_screens.cpp
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_GET_HUMAN_MANAGER_RECORD);
    let r = ficha(0);
    if r == 0 {
        return aviso_em(FONTE, 0x1e1c);
    }
    let clube = core::ptr::read_unaligned((r + 0xe6) as *const usize);
    if clube == 0 {
        return aviso_em(FONTE, 0x1e24);
    }
    let partida = core::ptr::read_unaligned((r + 0xe2) as *const usize).wrapping_add(4);
    if partida == 0 {
        return aviso_em(FONTE, 0x1e2c);
    }
    let d = |o: usize| core::ptr::read_unaligned((partida + o) as *const usize);
    let b = |o: usize| *((partida + o) as *const u8);
    let em_casa = (clube == d(0x1c) && b(0x40) == 0) || (clube == d(0x20) && b(0x41) == 0);
    gravar_parametro_da_tela(0, clube);
    gravar_parametro_da_tela(1, partida);
    gravar_parametro_da_tela(2, em_casa as usize);
}

/// the Team Selection buttons: appoints the captain (the best starter) when missing and stores the pending command; the
/// rest go to the common handler
/// Original: `match_screens_handle_team_selection_input` (0x727c30, match_screens.cpp, cdecl).
///
/// [pt] { 0x727c30 } (cdecl, match_screens.cpp): a resposta da tela da escalação ao item `item` (−1 nenhum).
/// Ids fora de 1, 2 e 3: a resposta comum (0x7615d0 e, se der 0, 0x7643d0) com o argumento inteiro. Id 3
/// (confirmar): se a tática [0xde1d08] não tem capitão (0x897590 com 1 negativo) ou ele não é titular
/// (0x897590 com 0 ≥ 11), nomeia o melhor titular (0x727e20) por 0x897760. Para 1, 2 e 3 deixa o
/// comando pendente 1 ([0xdd6be0]) com a escolha (o id, [0xdd6be4]) e devolve −9.
pub(crate) unsafe extern "C" fn match_screens_handle_team_selection_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond a escalacao");
    let clube = parametro_da_tela(0);
    let p2 = parametro_da_tela(2);
    let item16 = item as u16 as i16;
    let id = id_do_item_ou_padrao(item16);
    if id != 1 && id != 2 && id != 3 {
        let comum: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_COMMAND);
        let r = comum(item);
        if r != 0 {
            return r;
        }
        let outra: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
        return outra(item);
    }
    if id == 3 {
        let capitao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TACTICS_FIND_CAPTAIN_SLOT);
        let tatica = || *(G_TACTICS as *const usize);
        if (capitao(tatica(), clube, p2, 1) as u8 as i8) < 0 || (capitao(tatica(), clube, p2, 0) as u8 as i8) >= 11 {
            let melhor: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT);
            let m = melhor(clube, p2) as u8;
            if (m as i8) > -1 {
                // a original grava o byte por cima do argumento e empurra a palavra inteira
                let nomear: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TACTICS_SET_CAPTAIN_FROM_SLOT);
                nomear(tatica(), clube, (item & 0xffff_ff00) | m as u32, p2);
            }
        }
    }
    *(G_PENDING_COMMAND_CODE as *mut u32) = 1;
    *(G_PENDING_COMMAND_ARG as *mut i32) = id_do_item_ou_padrao(item16) as i16 as i32;
    -9
}

// ponytail: as três abrem telas, mexem na rede ou no capitão (não rodam duas vezes): no modo comparação
// só a original; a cópia é provada no modo Rust pelos ramos
pub(crate) unsafe extern "C" fn comparar_match_screens_handle_continue_command(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_SCREENS_HANDLE_CONTINUE_COMMAND)(item)
}

pub(crate) unsafe extern "C" fn comparar_match_screens_open_team_selection() {
    orig::<unsafe extern "C" fn()>(IDX_MATCH_SCREENS_OPEN_TEAM_SELECTION)()
}

pub(crate) unsafe extern "C" fn comparar_match_screens_handle_team_selection_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_SCREENS_HANDLE_TEAM_SELECTION_INPUT)(item)
}

// ---------- 200a leva: as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo (match_screens.cpp) ----------

/// Copia de `s` para `d` os campos de `tam` bytes (1, 2 ou 4) de `ini` a `fim`, em ordem crescente, como a original.
pub(crate) unsafe fn copiar_campos(d: usize, s: usize, ini: usize, fim: usize, tam: usize) {
    let mut k = ini;
    while k < fim {
        match tam {
            1 => *((d + k) as *mut u8) = *((s + k) as *const u8),
            2 => core::ptr::write_unaligned((d + k) as *mut u16, core::ptr::read_unaligned((s + k) as *const u16)),
            _ => core::ptr::write_unaligned((d + k) as *mut u32, core::ptr::read_unaligned((s + k) as *const u32)),
        }
        k += tam;
    }
}

/// copies the match team block (0x625 bytes) field by field, with its 20 player rows
/// Original: `match_team_block_assign` (0x71dd70, match_screens.cpp, thiscall).
///
/// [pt] { 0x71dd70 } (thiscall, match_screens.cpp): copia o bloco do time da partida (0x625 bytes) campo a
/// campo: bytes 0..0x11, palavras 0x11..0x1f, os quatro vetores de 5 bytes e os bytes 0x33/0x34, as duas
/// listas de 11 palavras (0x35, 0x4b), as duplas 0x61/0x65, as palavras 0x69..0x71 e as 20 linhas de
/// jogador de 0x49 bytes (0x71df00). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn match_team_block_assign(this: usize, src: usize) -> usize {
    copiar_campos(this, src, 0, 0x11, 1);
    copiar_campos(this, src, 0x11, 0x1f, 2);
    copiar_campos(this, src, 0x1f, 0x35, 1);
    copiar_campos(this, src, 0x35, 0x61, 2);
    copiar_campos(this, src, 0x61, 0x69, 4);
    copiar_campos(this, src, 0x69, 0x71, 2);
    let linha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MATCH_PLAYER_ROW_ASSIGN);
    for i in 0..20 {
        linha(this + 0x71 + i * 0x49, src + 0x71 + i * 0x49);
    }
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_match_team_block_assign(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_TEAM_BLOCK_ASSIGN);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0x625)], || o(this, src), || match_team_block_assign(this, src));
    PLACAR.registrar(igual && r == n, || format!("match_team_block_assign({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// copies one player row of the team block (0x49 bytes), the date through Date::operator=
/// Original: `match_player_row_assign` (0x71df00, match_screens.cpp, thiscall).
///
/// [pt] { 0x71df00 } (thiscall, match_screens.cpp): copia a linha de jogador do bloco do time (0x49 bytes):
/// duplas 0..0x14, a data em +0x14 pela Date::operator= (0x417c10, com o retorno numa local), bytes
/// 0x1c..0x36, palavras 0x36..0x3e e bytes 0x3e..0x49. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn match_player_row_assign(this: usize, src: usize) -> usize {
    copiar_campos(this, src, 0, 0x14, 4);
    let atribuir_data: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut volta = [0u8; 8];
    atribuir_data(this + 0x14, &mut volta, src + 0x14);
    copiar_campos(this, src, 0x1c, 0x36, 1);
    copiar_campos(this, src, 0x36, 0x3e, 2);
    copiar_campos(this, src, 0x3e, 0x49, 1);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_match_player_row_assign(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_PLAYER_ROW_ASSIGN);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0x49)], || o(this, src), || match_player_row_assign(this, src));
    PLACAR.registrar(igual && r == n, || format!("match_player_row_assign({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// counts the goals and shoot-out goals of a match up to the current minute of the live match
/// Original: `match_screens_tally_live_score` (0x728e90, match_screens.cpp, cdecl).
///
/// [pt] { 0x728e90 } (cdecl, match_screens.cpp): conta no resultado `res` os gols da partida `indice` do dia
/// ([[0xb67a50]+0x19] + 0x69 x indice + 0x5f; lances de 4 bytes a partir de +0x57, até 50 ou lado 0xff)
/// até o minuto atual da partida `partida` ([[partida+0x5f]+0x4d], 16 bits com sinal; sem partida ao vivo,
/// [partida+0x67] = 0xff ou [partida+0x5f] nulo, não conta nada). Tipo 1/3: gol do lado (0 vai a +0x43,
/// senão +0x44); 2: gol contra (invertido); 7: gol nos pênaltis (+0x49/+0x4a, que começam em 0xff e vão
/// a 1 no primeiro); 8 e os outros: nada. Zera antes +0x43..+0x46 e +0x4b/+0x4c. Resultado ou partida
/// nulos: nada.
pub(crate) unsafe extern "C" fn match_screens_tally_live_score(partida: usize, indice: u32, res: usize) {
    if res == 0 || partida == 0 {
        return;
    }
    let b = |k: usize| (res + k) as *mut u8;
    for k in [0x43, 0x44, 0x4b, 0x4c] {
        *b(k) = 0;
    }
    *b(0x49) = 0xff;
    *b(0x4a) = 0xff;
    *b(0x46) = 0;
    *b(0x45) = 0;
    let dia = core::ptr::read_unaligned((*(G_MATCH_DAY as *const usize) + 0x19) as *const u32);
    let jogo = core::ptr::read_unaligned(dia.wrapping_add(indice.wrapping_mul(0x69)).wrapping_add(0x5f) as usize as *const usize);
    if jogo == 0 {
        return;
    }
    let mut lance = jogo + 0x59;
    for _ in 0..0x32 {
        let lado = *((lance - 2) as *const u8);
        if lado == 0xff {
            break;
        }
        let ao_vivo = core::ptr::read_unaligned((partida + 0x5f) as *const usize);
        if *((partida + 0x67) as *const u8) != 0xff
            && ao_vivo != 0
            && *((lance + 1) as *const u8) as i16 <= core::ptr::read_unaligned((ao_vivo + 0x4d) as *const i16)
        {
            let tipo = *(lance as *const u8);
            let campo = match tipo {
                1 | 3 => Some(if lado == 0 { 0x43 } else { 0x44 }),
                2 => Some(if lado == 0 { 0x44 } else { 0x43 }),
                _ => None,
            };
            if let Some(k) = campo {
                *b(k) = (*b(k)).wrapping_add(1);
            } else if tipo == 7 {
                let k = if lado == 0 { 0x49 } else { 0x4a };
                *b(k) = if *b(k) == 0xff { 1 } else { *b(k) + 1 };
            }
        }
        lance += 4;
    }
}

pub(crate) unsafe extern "C" fn comparar_match_screens_tally_live_score(partida: usize, indice: u32, res: usize) {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize)>(IDX_MATCH_SCREENS_TALLY_LIVE_SCORE);
    if res == 0 {
        return o(partida, indice, res);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(res + 0x43, 0xa)], || o(partida, indice, res), || match_screens_tally_live_score(partida, indice, res));
    PLACAR.registrar(igual, || format!("match_screens_tally_live_score(partida {:#x}, indice {}): {:02x?} / {:02x?}", partida, indice, mo.0, mr.0));
}

// ---------- 199a leva: as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro (player_stats.cpp, match_stats.cpp) ----------


/// reports whether a match statistics entry holds the two given clubs in either order and the given date
/// Original: `Match_Stats::fixture_entry_matches` (0x729500, match_screens.cpp, thiscall).
///
/// [pt] { 0x729500 } (thiscall, ret 0x14, match_stats.cpp): a entrada `k` (12 bytes em [this]) tem os dois
/// clubes em qualquer ordem (+0/+4) e as datas (+8/+0xa, palavras) dadas? Índice −1: não; abaixo de −1
/// ou além de [this+0x1c]: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn fixture_entry_matches(estat: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    if k < -1 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(IDX_FIXTURE_ENTRY_MATCHES)(estat, k, a, b, data_a, data_b);
    }
    if k == -1 {
        return 0;
    }
    let e = (*(estat as *const usize) as isize + k as isize * 12) as usize;
    let d = |o: usize| *((e + o) as *const i32);
    let w = |o: usize| *((e + o) as *const i16);
    let clubes = (d(0) == a && d(4) == b) || (d(0) == b && d(4) == a);
    (clubes && w(8) == data_a && w(0xa) == data_b) as i32
}

/// the bucket (0 to 3) where the match statistics record is stored: competition type, national team and fixture flags
/// Original: `match_stats_get_record_bucket` (0x729bd0, match_screens.cpp, stdcall).
///
/// [pt] { 0x729bd0 } (stdcall, ret 8, match_stats.cpp): o balde (0..3) do registro de estatística do jogo:
/// 0 se um dos clubes (+0x1c/+0x20) tem a ficha ([+0xcf]) entre as 16 últimas da base com +0x5f; 3 se o
/// registro tem +0x4c e [0xa5a390]; sem +0x40, 3 se o tipo (+0x3a) é 1, senão 2; com +0x40 e clube de
/// seleção (0x53b900): 0 se a palavra +0x80 do clube passa de 4999 ou de 0x157b, senão 2; senão pelo tipo
/// (tabela de saltos em 0x729d38) e as tabelas de bytes do exe em 0x729d64 e 0x729d94.
pub(crate) unsafe extern "stdcall" fn match_stats_get_record_bucket(jogo: usize, registro: usize) -> u8 {
    let limite = *(G_STAFF_COUNT as *const i32);
    for o in [0x1c, 0x20] {
        let p = core::ptr::read_unaligned((*((jogo + o) as *const usize) + 0xcf) as *const usize);
        if p != 0 {
            let id = *(p as *const i32);
            if id >= limite.wrapping_sub(0x10) && id < limite && *((p + 0x5f) as *const u8) != 0 {
                return 0;
            }
        }
    }
    if *((registro + 0x4c) as *const u8) != 0 && *(G_MATCH_ENGINE_ENABLED as *const u8) != 0 {
        return 3;
    }
    if *((jogo + 0x40) as *const u8) == 0 {
        return (*((jogo + 0x3a) as *const u8) == 1) as u8 + 2;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(*((jogo + 0x1c) as *const usize)) != 0 {
        if *((jogo + 0x40) as *const u8) == 0 {
            return 2;
        }
        let v = core::ptr::read_unaligned((*((jogo + 0x1c) as *const usize) + 0x80) as *const i16);
        return if v as i32 * 2 >= 10000 || v >= 0x157c { 0 } else { 2 };
    }
    let w = |o: usize| core::ptr::read_unaligned((jogo + o) as *const i16) as i32;
    // ponytail: as duas tabelas de bytes dos saltos são lidas do próprio exe (o desvio só troca o começo da rotina)
    let tabela = |end: usize, i: i32| *((end + i as usize) as *const u8);
    match (*((jogo + 0x3a) as *const i8) as i32).wrapping_sub(1) {
        0 => 3,
        4 | 8 => 1,
        5 => {
            let c = w(0x30);
            if c as u32 > 0x9b {
                return 1;
            }
            match tabela(0x729d94, c) {
                0 => 0,
                1 => (core::ptr::read_unaligned((*((jogo + 0x14) as *const usize) + 0x69) as *const i16) <= 0xc) as u8,
                _ => 1,
            }
        }
        7 => {
            if (0x41f..=0x426).contains(&w(0x32)) {
                return 0;
            }
            let c = w(0x30) - 0x78;
            if c as u32 > 0x23 { 1 } else { (tabela(0x729d64, c) != 0) as u8 }
        }
        _ => {
            let n = *((*((jogo + 0x14) as *const usize) + 0x69) as *const i8);
            if n > 0xd { 1 } else { (n <= 6) as u8 + 2 }
        }
    }
}


pub(crate) unsafe extern "thiscall" fn comparar_fixture_entry_matches(estat: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(IDX_FIXTURE_ENTRY_MATCHES);
    if k < -1 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) {
        return o(estat, k, a, b, data_a, data_b);
    }
    let r = o(estat, k, a, b, data_a, data_b);
    let n = fixture_entry_matches(estat, k, a, b, data_a, data_b);
    PLACAR.registrar(r == n, || format!("fixture_entry_matches({}, {}, {}, {}, {}) original={} rust={}", k, a, b, data_a, data_b, r, n));
    r
}

pub(crate) unsafe extern "stdcall" fn comparar_match_stats_get_record_bucket(jogo: usize, registro: usize) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_MATCH_STATS_GET_RECORD_BUCKET)(jogo, registro);
    let n = match_stats_get_record_bucket(jogo, registro);
    PLACAR.registrar(r == n, || format!("match_stats_get_record_bucket original={} rust={}", r, n));
    r
}

// ---------- 198a leva: as telas da partida (6): a tela dos jogos do dia e o capitão padrão (match_screens.cpp) ----------

/// opens the day fixtures screen in a slot with the fixture group (error box without a group)
/// Original: `open_day_fixtures_screen` (0x717cc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x717cc0 } (cdecl, match_screens.cpp): abre a tela dos jogos do dia na vaga `vaga` (0x7fe4a0 com a
/// montagem 0x717db0 e a resposta 0x718930, sem match_wrap_up) e grava os parâmetros dela (0x7ff130): 1 o
/// grupo de jogos, 2 e 3 zero, 4 um. Grupo nulo: a original (caixa de erro e [0xb67a34] = 0).
pub(crate) unsafe extern "C" fn open_day_fixtures_screen(vaga: u32, grupo: usize) {
    if grupo == 0 {
        return orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_DAY_FIXTURES_SCREEN)(vaga, grupo);
    }
    crate::comparacao::ramo("telas abrir os jogos do dia");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN_FOR_EACH_RECORD);
    if abrir(GERENTE_DE_TELAS, vaga, 0x717db0, 0x718930, 0, 0, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM_FOR_EACH_RECORD);
    for (k, v) in [(1u32, grupo), (2, 0), (3, 0), (4, 1)] {
        gravar(GERENTE_DE_TELAS, vaga, k, v, 0);
    }
}

/// the day fixtures screen input: another fixture group, the competition progress or the menu bar
/// Original: `handle_day_fixtures_input` (0x718930, match_screens.cpp, cdecl).
///
/// [pt] { 0x718930 } (cdecl, match_screens.cpp): a resposta da tela dos jogos do dia ao item `item` (−1 =
/// nenhum). Com item: o parâmetro 2 recebe 0x803320 do parâmetro 3. Pelo id do item (0 vira o padrão
/// [0xdd6bde]): 1 abre outro grupo de jogos (0x717cc0 na vaga [0xb74c26] do humano da vez com o dado
/// +0xba9a6 do item) e 2 o andamento da competição (0x49b830 com o dado), ambos −11; senão a barra de
/// menu (0x7615d0, senão 0x7643d0). O item vai inteiro (32 bits) para a barra, como na original.
pub(crate) unsafe extern "C" fn handle_day_fixtures_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond aos jogos do dia");
    let i = item as i16;
    let id = |i: i16| if i == -1 { *(G_CURRENT_COMMAND_ID as *const u16) } else { id_do_item(i) };
    let dado = |i: i16| {
        if i == -1 {
            return 0usize;
        }
        let display = *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize);
        core::ptr::read_unaligned((display as isize + i as isize * 0x18c + 0xba9a6) as *const usize)
    };
    if i != -1 {
        let converter: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_AREA_SCROLL_ROW);
        gravar_parametro_da_tela(2, converter(GERENTE_DE_TELAS, parametro_da_tela(3)));
    }
    match id(i) {
        1 => {
            let vaga = *((0xb74c26 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const u32);
            let abrir: unsafe extern "C" fn(u32, usize) = core::mem::transmute(FN_OPEN_DAY_FIXTURES_SCREEN);
            abrir(vaga, dado(i));
            -11
        }
        2 => {
            let andamento: unsafe extern "C" fn(usize) = core::mem::transmute(FN_COMP_OPEN_LEAGUE_PROGRESS_FOR_FIXTURE);
            andamento(dado(i));
            -11
        }
        _ => {
            let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_COMMAND);
            let r = comando(item);
            if r != 0 {
                return r;
            }
            let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
            ligacao(item)
        }
    }
}

/// the first eleven slot with the highest staff rating (the default captain), -1 for none
/// Original: `match_screens_find_best_rated_starter_slot` (0x727e20, match_screens.cpp, cdecl).
///
/// [pt] { 0x727e20 } (cdecl, match_screens.cpp): a vaga (0..10) dos onze da tática `tatica` com a maior nota
/// da comissão (0x8968f0 em [0xde1d08] com `contexto`, nota 0x6e1cf0(pessoa, 0, tatica), comparação i16;
/// no empate fica a primeira), o capitão padrão; −1 (0xff) sem ninguém. Tática nula: a original (caixa
/// de erro). Quem chama só usa `al`.
pub(crate) unsafe extern "C" fn match_screens_find_best_rated_starter_slot(tatica: usize, contexto: i32) -> u8 {
    if tatica == 0 {
        return orig::<unsafe extern "C" fn(usize, i32) -> u8>(IDX_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT)(tatica, contexto);
    }
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let avaliar: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let mut vaga = 0xffu8;
    let mut melhor = 0i16;
    for k in 0..11u32 {
        let p = da_tatica(*(G_TACTICS as *const usize), tatica, k, contexto);
        if p != 0 {
            let n = avaliar(p, 0, tatica);
            if vaga == 0xff || n > melhor {
                vaga = k as u8;
                melhor = n;
            }
        }
    }
    vaga
}

// ponytail: abrir e respond mexem em telas e menu (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
pub(crate) unsafe extern "C" fn comparar_open_day_fixtures_screen(vaga: u32, grupo: usize) {
    orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_DAY_FIXTURES_SCREEN)(vaga, grupo)
}

pub(crate) unsafe extern "C" fn comparar_handle_day_fixtures_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_DAY_FIXTURES_INPUT)(item)
}

/// Só lê: compara a vaga devolvida (tática nula: só a original, pela caixa de erro).
pub(crate) unsafe extern "C" fn comparar_match_screens_find_best_rated_starter_slot(tatica: usize, contexto: i32) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, i32) -> u8>(IDX_MATCH_SCREENS_FIND_BEST_RATED_STARTER_SLOT)(tatica, contexto);
    if tatica == 0 {
        return r;
    }
    let n = match_screens_find_best_rated_starter_slot(tatica, contexto);
    PLACAR.registrar(r == n, || format!("match_screens_find_best_rated_starter_slot({:#x}, {}) {} x {}", tatica, contexto, r as i8, n as i8));
    r
}

// ---------- 186a leva: as telas da partida (4): abrir a tela da partida e as respostas dos últimos placares e das estatísticas do time (match_screens.cpp) ----------

/// O bloco de parâmetros da tela atual ([gerente + 0x7e2 + 0x300 x vaga]; o parâmetro k em +0x14 + 8k,
/// a marca de "liberar ao trocar" em +0x18 + 8k, 60 parâmetros) e as marcas 0xb6b7a8/ac/b0.
pub(crate) unsafe fn regioes_dos_parametros() -> Vec<(usize, usize)> {
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    let mut r = vec![(0xb6b7a8, 12)];
    if bloco != 0 {
        r.push((bloco, 0x200));
    }
    r
}

/// O id do item `item` na lista de exibição do humano da vez (0 vira o padrão [0xdd6bde]).
pub(crate) unsafe fn id_do_item(item: i16) -> u16 {
    let display = *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize);
    let w = core::ptr::read_unaligned((display as isize + item as isize * 0x18c + 0xba966) as *const u16);
    if w == 0 { *(G_CURRENT_COMMAND_ID as *const u16) } else { w }
}

/// opens the live match screen, writes its parameters and marks the human as a spectator
/// Original: `open_match_screen` (0x718aa0, match_screens.cpp, cdecl).
///
/// [pt] { 0x718aa0 } (cdecl, match_screens.cpp): abre a tela da partida `partida` na vaga `vaga` (0x7fe4a0
/// com a montagem 0x718e90, a resposta 0x71a240 e o match_wrap_up 0x71b120) e grava os parâmetros dela
/// (0x7ff130; o 0xe é [partida+0x63]); marca o humano da vez como espectador ([[partida+0x5b]+0xc5+4h],
/// contando em +0x108) e, sem confirmação ([partida+0x57]), confirma pela rede (0x6b5260) e, se
/// confirmou, grava 2 no parâmetro 1. Vaga ou partida nula: a original (caixa de erro).
pub(crate) unsafe extern "C" fn open_match_screen(vaga: u32, partida: usize) {
    if vaga == 0 || partida == 0 {
        return orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_MATCH_SCREEN)(vaga, partida);
    }
    crate::comparacao::ramo("telas abrir a partida");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN_FOR_EACH_RECORD);
    if abrir(GERENTE_DE_TELAS, vaga, 0x718e90, 0x71a240, 1, 0x71b120, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM_FOR_EACH_RECORD);
    const M: usize = usize::MAX;
    let e = *((partida + 0x63) as *const usize);
    for (k, v) in [
        (0u32, partida), (1, 1), (2, 1), (3, M), (4, M), (5, M), (6, M), (7, 0), (8, M), (9, M), (0xa, 3), (0xb, M), (0xc, 0), (0xd, M),
        (0xe, e), (0xf, 0), (0x10, M), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0), (0x15, 0), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, M), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x21, M), (0x22, 0), (0x23, M), (0x24, 0), (0x25, 0), (0x26, 0), (0x27, 0),
        (0x28, 0), (0x29, 0), (0x2d, 0), (0x2f, 0), (0x30, 0),
    ] {
        gravar(GERENTE_DE_TELAS, vaga, k, v, 0);
    }
    let espectadores = *((partida + 0x5b) as *const usize);
    let marca = (espectadores as isize + 0xc5 + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 4) as *mut u32;
    if *marca == 0 {
        *marca = 1;
        let n = (espectadores + 0x108) as *mut u8;
        *n = (*n).wrapping_add(1);
    }
    if *((partida + 0x57) as *const u32) == 0 {
        let confirmar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MATCH_DAY_START_WATCHED_MATCH);
        confirmar(*(G_MATCH_DAY as *const usize), partida);
        if *((partida + 0x57) as *const u32) != 0 {
            let gravar_atual: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM);
            gravar_atual(GERENTE_DE_TELAS, 1, 2, 0);
        }
    }
}

/// the latest scores panel of the match screen: the page switch every 5 s and the clicks (competition, pages, pause)
/// Original: `match_screens_handle_latest_scores_cycle` (0x725860, match_screens.cpp, cdecl).
///
/// [pt] { 0x725860 } (cdecl, match_screens.cpp): a resposta do painel dos últimos placares ao item `item`.
/// A cada 5 s (parâmetro 0x14 + 5000 ms, 0x90c510) passa à página seguinte (parâmetro 0x11, de 1 até
/// o 0x13, se não estiver parado pelo 0x12) e, sem [0xb6b7b0], redesenha o painel (0x724520) e liga
/// [0xb6b7a8]. Itens: 5 troca a competição (0xe) e volta à página 1, 6 e 8 a página anterior e a
/// seguinte (em círculo), 7 para ou solta a troca automática; cada um liga [0xb6b7b0]. Devolve 0.
pub(crate) unsafe extern "C" fn match_screens_handle_latest_scores_cycle(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond ao painel dos placares");
    let pagina = parametro_da_tela(0x11) as u8 as i8;
    let parado = parametro_da_tela(0x12);
    let paginas = parametro_da_tela(0x13) as u8 as i8;
    let quando = parametro_da_tela(0x14) as u32;
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    let seguinte = if pagina == paginas { 1 } else { (pagina as i32 + 1) as usize };
    if agora() >= quando.wrapping_add(5000) {
        if paginas > 1 && parado == 0 {
            gravar_parametro_da_tela(0x11, seguinte);
        }
        if *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *const u32) == 0 {
            let redesenhar: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_LATEST_SCORES_ROWS);
            redesenhar();
            *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
        }
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    let b0 = G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32;
    if id_do_item(item16) == 5 {
        let display = *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize);
        let dado = core::ptr::read_unaligned((display as isize + item16 as isize * 0x18c + 0xba9a6) as *const usize);
        gravar_parametro_da_tela(0xe, dado);
        gravar_parametro_da_tela(0x11, 1);
        *b0 = 1;
    }
    if id_do_item(item16) == 6 {
        // a página lida no início (a original não relê depois do item 5)
        gravar_parametro_da_tela(0x11, if pagina == 1 { paginas as i32 as usize } else { (pagina as i32 - 1) as usize });
        *b0 = 1;
    }
    if id_do_item(item16) == 7 {
        gravar_parametro_da_tela(0x12, (parado == 0) as usize);
        *b0 = 1;
    }
    if id_do_item(item16) == 8 {
        gravar_parametro_da_tela(0x11, seguinte);
        *b0 = 1;
    }
    0
}

/// the team stats panel: the redraw with new data and the clicks that move the row up and down
/// Original: `handle_team_stats_input` (0x722dc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x722dc0 } (cdecl, match_screens.cpp): a resposta das estatísticas do time ao item `item`. Com
/// dados novos ([0xb6b7ac]) e a marca 4 do menu da partida ([[parâmetro 0]+0x5b]), sem [0xb6b7b0],
/// redesenha o painel (0x721cd0 com 0 e −1) e liga [0xb6b7a8]; os itens 9 e 0xa descem e sobem a
/// linha do parâmetro 0x1c e ligam [0xb6b7b0]. Devolve 0.
pub(crate) unsafe extern "C" fn handle_team_stats_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond as estatisticas do time");
    let p0 = parametro_da_tela(0);
    let linha = parametro_da_tela(0x1c) as u8 as i8;
    if *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *const u32) != 0 && *(*((p0 + 0x5b) as *const usize) as *const u8) & 4 != 0 && *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *const u32) == 0 {
        let redesenhar: unsafe extern "C" fn(u32, i32) = core::mem::transmute(FN_FILL_TEAM_STATS_ROWS);
        redesenhar(0, -1);
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
    }
    let item16 = item as u16 as i16;
    if item16 == -1 {
        return 0;
    }
    if id_do_item(item16) == 9 {
        gravar_parametro_da_tela(0x1c, (linha as i32 - 1) as usize);
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
    }
    if id_do_item(item16) == 0xa {
        gravar_parametro_da_tela(0x1c, (linha as i32 + 1) as usize);
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
    }
    0
}

// ponytail: abrir a tela mexe em telas e na rede (não roda duas vezes): no modo comparação só a original
pub(crate) unsafe extern "C" fn comparar_open_match_screen(vaga: u32, partida: usize) {
    orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_MATCH_SCREEN)(vaga, partida)
}

/// As duas respostas são comparadas de verdade nos cliques (o bloco de parâmetros da tela e as marcas
/// 0xb6b7a8/ac/b0, mais a devolução); quando há redesenho (a troca de 5 s dos placares, os dados novos
/// das estatísticas) só a original roda, porque o redesenho grava a hora no parâmetro 0x14 e mexe na
/// lista de exibição.
pub(crate) unsafe fn comparar_resposta(nome: &str, item: u32, o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32, redesenha: bool) -> i32 {
    if item as u16 == 0xffff || redesenha {
        return o(item);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes_dos_parametros(), || o(item), || r(item));
    PLACAR.registrar(igual && ro == rr, || format!("{}(item {:#x}) original={} rust={} {:?} x {:?}", nome, item, ro, rr, mo, mr));
    ro
}

pub(crate) unsafe extern "C" fn comparar_match_screens_handle_latest_scores_cycle(item: u32) -> i32 {
    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    // folga de 1 s: a hora pode virar entre os dois lados
    let redesenha = agora().wrapping_add(1000) >= (parametro_da_tela(0x14) as u32).wrapping_add(5000);
    comparar_resposta("match_screens_handle_latest_scores_cycle", item, orig(431), match_screens_handle_latest_scores_cycle, redesenha)
}

pub(crate) unsafe extern "C" fn comparar_handle_team_stats_input(item: u32) -> i32 {
    let p0 = parametro_da_tela(0);
    let redesenha = *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *const u32) != 0 && *(*((p0 + 0x5b) as *const usize) as *const u8) & 4 != 0;
    comparar_resposta("handle_team_stats_input", item, orig(432), handle_team_stats_input, redesenha)
}

// ---------- 183a leva: as telas da partida (3): a tela de detalhes da partida (match_screens.cpp) ----------

/// Lê e grava os parâmetros da tela atual do gerente de telas [0xb74450] (0x7ff010, 0x7ff270).
pub(crate) unsafe fn parametro_da_tela(k: u32) -> usize {
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_SCREEN_PARAM);
    ler(0xb74450, k)
}
pub(crate) unsafe fn gravar_parametro_da_tela(k: u32, v: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM);
    gravar(0xb74450, k, v, 0);
}

/// opens the details screen of a match and writes its parameters
/// Original: `open_match_details` (0x71b4a0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b4a0 } (cdecl, match_screens.cpp): abre a tela de detalhes da partida `partida` (0x7fe600 com a
/// montagem 0x71b630, a resposta 0x71bdc0 e o match_wrap_up 0x71c170) e grava os parâmetros dela (a
/// partida em 0x1e; a aba 2 começa em 6).
pub(crate) unsafe extern "C" fn open_match_details(partida: usize) {
    crate::comparacao::ramo("telas abrir os detalhes");
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN);
    if abrir(0xb74450, 0x71b630, 0x71bdc0, 0, 0x71c170, 0) == 0 {
        return;
    }
    const MENOS1: usize = usize::MAX;
    for (k, v) in [
        (0u32, 0usize), (2, 6), (9, MENOS1), (0xa, 3), (0xb, MENOS1), (0xc, 0), (0xd, MENOS1), (0x16, 0), (0x17, 0), (0x18, 0), (0x19, 0),
        (0x1a, MENOS1), (0x1b, 0), (0x1c, 1), (0x1d, 0), (0x1e, partida), (0x1f, 0), (0x20, 0), (0x21, MENOS1), (0x22, 0), (0x23, MENOS1), (0x2d, 0),
    ] {
        gravar_parametro_da_tela(k, v);
    }
}

/// the match details screen's response to a click (tab switch, club tactics, tab redraw, menu)
/// Original: `match_details_input` (0x71bdc0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71bdc0 } (cdecl, match_screens.cpp): a resposta da tela de detalhes ao item `item` (−1 nenhum).
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; um item de id 1 (ou [0xdd6bde]) troca a aba (parâmetro 2) pelo
/// dado do item (+0x48 do item no `Display` do humano da vez) e pede −11; de id 2 abre a tática do clube
/// do item (0x899a50) e devolve −4. Depois, pela aba: 1 posse (0x71d9e0), 2 estatísticas (0x71e750), 3
/// zonas de ação (0x720320; e um item de id 4 troca a ação do parâmetro 0xa), 4 relatório, 5 e 7 as
/// estatísticas do time (0x722dc0), 6 as notas (0x723350); os redesenhos só com a marca 4 do menu da
/// partida ([[parâmetro 0]+0x5b]) e [0xb6b7ac]. Por fim −11, o redesenho (0x802d00) e a barra de menu.
pub(crate) unsafe extern "C" fn match_details_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond aos detalhes");
    let g = |a: usize| *(a as *const u32);
    let item16 = item as u16 as i16;
    let display_do_humano = || *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize);
    let id_ou_padrao = |w: u16| if w == 0 { *(G_CURRENT_COMMAND_ID as *const u16) } else { w };
    if g(0xb77c84) == 0 {
        let aba = parametro_da_tela(2) as u8 as i8;
        let v1e = parametro_da_tela(0x1e);
        let p1f = parametro_da_tela(0x1f);
        *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 0;
        if item16 != -1 {
            let deslocamento = item16 as isize * 0x18c;
            let registro = |d: usize| (d as isize + deslocamento) as usize;
            let e = registro(display_do_humano());
            if id_ou_padrao(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 1 {
                let dado = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                if dado != parametro_da_tela(2) {
                    gravar_parametro_da_tela(2, dado);
                    *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                }
            }
            let e = registro(display_do_humano());
            if id_ou_padrao(core::ptr::read_unaligned((e + 0xba966) as *const u16)) == 2 {
                let clube = core::ptr::read_unaligned((e + 0xba9a6) as *const usize);
                let marca = if clube == *((p1f + 0x1c) as *const usize) { *((p1f + 0x40) as *const u8) } else { *((p1f + 0x41) as *const u8) };
                let tatica: unsafe extern "C" fn(usize, u32, u32, u32, usize, u32) = core::mem::transmute(FN_TACTICS_SCREEN_OPEN);
                tatica(clube, (marca == 0) as u32, 0, 0, v1e, 0);
                return -4;
            }
        }
        let menu_da_partida = |bit: u8| {
            let p = parametro_da_tela(0);
            g(0xb6b7ac) != 0 && *(*((p + 0x5b) as *const usize) as *const u8) & bit != 0
        };
        let refazer = |f: usize| -> bool {
            // os redesenhos de aba: com a marca 4; com [0xb6b7b0] ligado a resposta é −11
            if g(0xb6b7b0) != 0 {
                return true;
            }
            let r: unsafe extern "C" fn() = core::mem::transmute(f);
            r();
            *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 1;
            false
        };
        let estatisticas_do_time: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_HANDLE_TEAM_STATS_INPUT);
        match aba {
            1 => {
                let p = parametro_da_tela(0);
                if g(0xb6b7ac) != 0 {
                    let m = *(*((p + 0x5b) as *const usize) as *const u8);
                    if m & 0x20 != 0 || m & 0x10 != 0 {
                        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                    }
                    if m & 4 != 0 && refazer(0x71d9e0) {
                        return -11;
                    }
                }
            }
            2 => {
                if menu_da_partida(4) && refazer(0x71e750) {
                    return -11;
                }
            }
            3 => {
                if menu_da_partida(4) && g(0xb6b7b0) == 0 {
                    refazer(0x720320);
                }
                if item16 != -1 {
                    let d = display_do_humano();
                    let id: unsafe extern "thiscall" fn(usize, u32) -> u16 = core::mem::transmute(FN_DISPLAY_GET_WIDGET_ID);
                    if id_ou_padrao(id(d, item)) == 4 {
                        let dado: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_DISPLAY_GET_WIDGET_DATA);
                        if dado(d, item) != parametro_da_tela(0xa) {
                            let acao: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_WIDGET_ACTION_DATA);
                            gravar_parametro_da_tela(0xa, acao(0xb74450, item));
                            *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                            return -11;
                        }
                    }
                }
            }
            4 => {
                if menu_da_partida(0x20) {
                    *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 1;
                    return -11;
                }
            }
            5 | 7 => {
                let r = estatisticas_do_time(item);
                if r != 0 {
                    return r;
                }
            }
            6 => {
                if menu_da_partida(4) && refazer(0x723350) {
                    return -11;
                }
            }
            _ => {}
        }
        if g(0xb6b7b0) != 0 {
            return -11;
        }
        if g(0xb6b7a8) != 0 {
            let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_REDRAW_SCREEN);
            redesenhar(0xb74450, 0, 0);
        }
    }
    let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_COMMAND);
    let r = comando(item);
    if r != 0 {
        return r;
    }
    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
    ligacao(item)
}

/// closes the details screen: destroys and frees the objects kept in the parameters
/// Original: `close_match_details` (0x71c170, match_screens.cpp, cdecl).
///
/// [pt] { 0x71c170 } (cdecl, match_screens.cpp): fecha a tela de detalhes: os parâmetros 0x2d (0x5a8f60),
/// 0x1d e 0x20 (0x453bc0) são destruídos e liberados, o 0x1f só liberado (0x944c94), e todos zerados.
pub(crate) unsafe extern "C" fn close_match_details() {
    crate::comparacao::ramo("telas fechar os detalhes");
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    for (k, destrutor) in [(0x2du32, Some(0x5a8f60usize)), (0x1d, Some(0x453bc0)), (0x1f, None), (0x20, Some(0x453bc0))] {
        if parametro_da_tela(k) == 0 {
            continue;
        }
        let p = parametro_da_tela(k);
        match destrutor {
            Some(d) => {
                if p != 0 {
                    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(d);
                    destruir(p);
                    liberar(p);
                }
            }
            None => liberar(p),
        }
        gravar_parametro_da_tela(k, 0);
    }
}

// ponytail: as três mexem em telas, menus e memória (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
pub(crate) unsafe extern "C" fn comparar_open_match_details(partida: usize) {
    orig::<unsafe extern "C" fn(usize)>(IDX_OPEN_MATCH_DETAILS)(partida)
}

pub(crate) unsafe extern "C" fn comparar_match_details_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_MATCH_DETAILS_INPUT)(item)
}

pub(crate) unsafe extern "C" fn comparar_close_match_details() {
    orig::<unsafe extern "C" fn()>(IDX_CLOSE_MATCH_DETAILS)()
}

// ---------- 182a leva: as telas da partida (2): a tela dos últimos placares (match_screens.cpp) ----------

/// O gerente de telas (o objeto [0xb74450], que tem a vaga da tela atual em +0x382a e um `Display` por
/// vaga em +0x7fc + 0x300 x vaga).
pub(crate) const GERENTE_DE_TELAS: usize = 0xb74450;

/// As regiões que a montagem de uma tela mexe: o gerente de telas (0x3840 bytes) e o `Display` da tela
/// atual (a lista de exibição: itens de 0x18c bytes em +0xba95e, quantos em +0x12e9a0; 0x12f550 bytes).
pub(crate) unsafe fn regioes_da_tela() -> Vec<(usize, usize)> {
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16);
    let mut r = vec![(GERENTE_DE_TELAS, 0x3840)];
    if (0..64).contains(&vaga) {
        let display = core::ptr::read_unaligned((GERENTE_DE_TELAS + 0x7fc + vaga as usize * 0x300) as *const usize);
        if display != 0 {
            r.push((display, 0x12f550));
        }
    }
    r
}

/// opens the latest scores screen of a competition and writes its parameters
/// Original: `open_latest_scores` (0x71b1b0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b1b0 } (cdecl, match_screens.cpp): abre a tela dos últimos placares da competição `entrada` na
/// vaga `vaga` (0x7fe4a0 com a montagem 0x71b300 e a resposta 0x71b3d0) e grava os parâmetros dela
/// (0x7ff130): 0 e 0xe (a entrada) e 0xf..0x14 (0, −1, 1, 0, 1, 0). Vaga ou entrada nula: a original
/// (caixa de erro).
pub(crate) unsafe extern "C" fn open_latest_scores(vaga: u32, entrada: usize) {
    if vaga == 0 || entrada == 0 {
        return orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_LATEST_SCORES)(vaga, entrada);
    }
    crate::comparacao::ramo("telas abrir os ultimos placares");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN_FOR_EACH_RECORD);
    if abrir(GERENTE_DE_TELAS, vaga, 0x71b300, 0x71b3d0, 0, 0, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM_FOR_EACH_RECORD);
    for (k, v) in [(0u32, 0usize), (0xe, entrada), (0xf, 0), (0x10, usize::MAX), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0)] {
        gravar(GERENTE_DE_TELAS, vaga, k, v, 0);
    }
}

/// builds the latest scores screen: background, title, menu, panel and back and next buttons
/// Original: `build_latest_scores` (0x71b300, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b300 } (cdecl, match_screens.cpp): monta a tela dos últimos placares: o fundo da competição
/// (registro de 107 bytes em [0xae23d0] pelo id do parâmetro 0xe, 0x41c410), o título "Latest Scores"
/// (0x66f4e0 em 0xde1f64, cores 0x839590, 0x803f30), a barra de menu (0x75db00 com 4), o painel dos
/// placares (0x723d20, de 0x50 a 0x217) e os botões de voltar e avançar (0x5edcc0, desligados quando o
/// gerente não tem para onde ir).
pub(crate) unsafe extern "C" fn build_latest_scores() {
    crate::comparacao::ramo("telas montar os ultimos placares");
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_SCREEN_PARAM);
    let entrada = ler(GERENTE_DE_TELAS, 0xe);
    let id = *(entrada as *const i32);
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_BACKGROUND_SET_COMP_PICTURE);
    fundo((*(G_CLUB_COMP_TABLE as *const usize) as isize + id as isize * 107) as usize, 0, 0);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    texto(0xde1f64, 0xa16328, 0xad9c64);
    let cor: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_GET_HEADING_COLOUR);
    let (c1, c0) = (cor(1), cor(0));
    let titulo: unsafe extern "thiscall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_ADD_SCREEN_TITLE);
    titulo(GERENTE_DE_TELAS, c0, c1, 0xde1f64);
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(FN_MENUBAR_BUILD);
    menu(4, 0);
    let (mut topo, mut base) = (0x50i32, 0x217i32);
    let painel: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(FN_DRAW_LATEST_SCORES_PANEL);
    painel(&mut topo, &mut base);
    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_FORWARD_UNAVAILABLE);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_BACK_UNAVAILABLE);
    let avancar = (sem_avancar(GERENTE_DE_TELAS) == 0) as u32;
    let voltar = (sem_voltar(GERENTE_DE_TELAS, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(FN_GUI_ADD_BACK_NEXT_BUTTONS);
    botoes(voltar, avancar);
}

/// the latest scores screen's response to a click (competition switch, redraw, menu)
/// Original: `latest_scores_input` (0x71b3d0, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b3d0 } (cdecl, match_screens.cpp): a resposta da tela dos últimos placares ao item `item`.
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; fora do modo 2 da partida ([[0xb67a50]+0x34]+0x67) e com o
/// técnico humano da vez conectado (0x5fd210), marca a hora (0x5fdd30 com 0x90c510); depois a troca de
/// competição (0x725860) decide; sem decisão, −11 com [0xb6b7b0] ou redesenha com [0xb6b7a8]
/// (0x802d00). Por fim a barra de menu (0x7615d0, senão 0x7643d0).
pub(crate) unsafe extern "C" fn latest_scores_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond aos ultimos placares");
    let g = |a: usize| *(a as *const u32);
    if g(0xb77c84) == 0 {
        let m = g(0xb67a50) as usize;
        *(G_MATCH_SCREEN_LIVE_DATA_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_VIEW_CHANGED_FLAG as *mut u32) = 0;
        *(G_MATCH_SCREEN_PANEL_REFRESHED_FLAG as *mut u32) = 0;
        if *((*((m + 0x34) as *const usize) + 0x67) as *const u8) != 2 {
            let humano = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
            if *((0xb74c26 + humano * 0x300) as *const u32) != 0 {
                let tecnicos = g(0xb63c98) as usize;
                let conectado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
                if conectado(tecnicos, 0) != 0 {
                    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
                    let marcar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_HUMAN_MANAGER_SET_HOLIDAY_STAMP);
                    marcar(tecnicos, agora(), 0);
                }
            }
        }
        let trocar: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MATCH_SCREENS_HANDLE_LATEST_SCORES_CYCLE);
        let r = trocar(item);
        if r != 0 {
            return r;
        }
        if g(0xb6b7b0) != 0 {
            return -11;
        }
        if g(0xb6b7a8) != 0 {
            let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_REDRAW_SCREEN);
            redesenhar(GERENTE_DE_TELAS, 0, 0);
        }
    }
    let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_COMMAND);
    let r = comando(item);
    if r != 0 {
        return r;
    }
    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
    ligacao(item)
}

// ponytail: abrir e respond mexem em telas, menu e hora (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
pub(crate) unsafe extern "C" fn comparar_open_latest_scores(vaga: u32, entrada: usize) {
    orig::<unsafe extern "C" fn(u32, usize)>(IDX_OPEN_LATEST_SCORES)(vaga, entrada)
}

/// A montagem é comparada de verdade: as duas montam a tela a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual (a lista de exibição) e o título (0xde1f64) têm de sair iguais.
pub(crate) unsafe extern "C" fn comparar_build_latest_scores() {
    let o = orig::<unsafe extern "C" fn()>(IDX_BUILD_LATEST_SCORES);
    let mut reg = regioes_da_tela();
    reg.push((0xde1f64, 0x100));
    // a lista (0x55fba0) escreve num buffer de rede ([0xae2908] na posicao [0xae290c]): a posicao volta
    // para a segunda rodada escrever no mesmo lugar
    reg.push((0xae290c, 4));
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(), || crate::aleatorio::sem_contar(|| build_latest_scores()));
    PLACAR.registrar(igual, || {
        // so as diferencas (a lista e grande): onde comecam e quantos bytes
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("build_latest_scores: diferencas em {:?}", dif)
    });
}

pub(crate) unsafe extern "C" fn comparar_latest_scores_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(IDX_LATEST_SCORES_INPUT)(item)
}

// ---------- 180a leva: as telas da partida (1): o rótulo do controle, fechar a tela e a espera (match_screens.cpp) ----------

/// the match control button label (waiting, kick off, restart, pause or resume, continue) and whether it is disabled
/// Original: `match_control_label` (0x725a70, match_screens.cpp, cdecl).
///
/// [pt] { 0x725a70 } (cdecl, match_screens.cpp): o rótulo do botão de controle da partida para o estado
/// `estado` (1 esperando, 2 começar, 3 recomeçar, 4 pausar/retomar, 5 continuar ou esperar os
/// jogadores) em `saida` pela 0x66f4e0 (o texto passa pelo idioma), com as marcas 0x1021 (mais 0x40
/// quando o botão fica desligado). Sem partida, sem saída, com menos de 100 bytes ou estado fora: a
/// original (caixa de erro). Devolve 1.
pub(crate) unsafe extern "C" fn match_control_label(estado: u8, m: usize, saida: *mut u8, tam: i32, marcas: *mut u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(IDX_MATCH_CONTROL_LABEL);
    if m == 0 || saida.is_null() || tam < 100 || !(1..=5).contains(&estado) {
        return o(estado, m, saida, tam, marcas);
    }
    let texto: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    *saida = 0;
    *marcas = 0x1021;
    let humano = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let (formato, desligado) = match estado {
        1 => (0xa16d8c, false),
        2 => (0xa16d7c, false),
        3 => (0xa16d6c, false),
        4 => {
            if core::ptr::read_unaligned((m as isize + 0xc5 + 4 * humano) as *const u32) != 0 {
                (0xa16d5c, false)
            } else if *((m + 0x108) as *const u8) != 0 {
                (0xa16d54, true)
            } else {
                (0xa16d48, false)
            }
        }
        _ => {
            let esperando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_MANAGER_DONE);
            let jogador = *((0xb74c26 + humano * 0x300) as *const u32);
            if esperando(0xb74450, jogador) != 0 { (0xa16d34, true) } else { (0xa16d24, false) }
        }
    };
    crate::comparacao::ramo("telas rotulo do controle");
    texto(saida, formato);
    if desligado {
        *marcas |= 0x40;
    }
    1
}

/// closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d
/// Original: `close_match_screen` (0x71b120, match_screens.cpp, cdecl).
///
/// [pt] { 0x71b120 } (cdecl, match_screens.cpp): fecha a tela da partida: os parâmetros 0x2d e 0x1d do
/// gerente de telas [0xb74450] (0x7ff010), se houver, são destruídos (0x5a8f60 e 0x453bc0), liberados
/// (0x944c94) e zerados (0x7ff270).
pub(crate) unsafe extern "C" fn close_match_screen() {
    crate::comparacao::ramo("telas fechar a partida");
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_GET_SCREEN_PARAM);
    let gravar: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_SCREEN_PARAM);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    for (k, destrutor) in [(0x2du32, 0x5a8f60usize), (0x1d, 0x453bc0)] {
        if ler(0xb74450, k) == 0 {
            continue;
        }
        let p = ler(0xb74450, k);
        if p != 0 {
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
            destruir(p);
            liberar(p);
        }
        gravar(0xb74450, k, 0, 0);
    }
}

/// opens the human manager's waiting screen in the screen manager
/// Original: `open_waiting_screen` (0x727f00, match_screens.cpp, cdecl).
///
/// [pt] { 0x727f00 } (cdecl, match_screens.cpp): abre a tela de espera na vaga `vaga` do gerente de telas
/// (0x7fe4a0 com o desenho 0x727fa0 e a resposta 0x727fb0). Vaga 0: a original (caixa de erro).
pub(crate) unsafe extern "C" fn open_waiting_screen(vaga: u32) {
    if vaga == 0 {
        return orig::<unsafe extern "C" fn(u32)>(IDX_OPEN_WAITING_SCREEN)(vaga);
    }
    crate::comparacao::ramo("telas abrir a espera");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_OPEN_SCREEN_FOR_EACH_RECORD);
    abrir(0xb74450, vaga, 0x727fa0, 0x727fb0, 0, 0, 0);
}

pub(crate) unsafe extern "C" fn comparar_match_control_label(estado: u8, m: usize, saida: *mut u8, tam: i32, marcas: *mut u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u8, usize, *mut u8, i32, *mut u32) -> u32>(IDX_MATCH_CONTROL_LABEL);
    if m == 0 || saida.is_null() || tam < 100 || marcas.is_null() {
        return o(estado, m, saida, tam, marcas);
    }
    let reg = [(saida as usize, tam.min(0x100) as usize), (marcas as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(estado, m, saida, tam, marcas), || match_control_label(estado, m, saida, tam, marcas));
    PLACAR.registrar(igual && r == n, || format!("match_control_label(estado {}) original={} rust={} {:x?} x {:x?}", estado, r, n, mo, mr));
    r
}

// ponytail: fechar e abrir mexem no gerente de telas e na memória (não rodam duas vezes): no modo
// comparação só a original roda; as cópias se provam no jogo do Lucas em modo Rust (JOGAR.bat), com
// os contadores "telas ..." no ddraw.log
pub(crate) unsafe extern "C" fn comparar_close_match_screen() {
    orig::<unsafe extern "C" fn()>(IDX_CLOSE_MATCH_SCREEN)()
}

pub(crate) unsafe extern "C" fn comparar_open_waiting_screen(vaga: u32) {
    orig::<unsafe extern "C" fn(u32)>(IDX_OPEN_WAITING_SCREEN)(vaga)
}

/// draws the latest scores panel: title, badge, the day's groups and matches, table and buttons
/// Original: `draw_latest_scores_panel` (0x723d20, match_screens.cpp, cdecl).
///
/// [pt] { 0x723d20 } (cdecl, match_screens.cpp): desenha o painel dos últimos placares a partir de `*topo`
/// até `*base`. Título (dentro da partida, parâmetro 0 ≠ 0: "<competição> Latest" à esquerda 0x6e e
/// largura 0x30c, `*topo` += 0x28; na tela avulsa o nome em 0x64/0x316, += 0x41) e o selo da competição
/// (0x804030, fora dos amistosos 0x5c2990 e das seleções 0x7e6f20). Com mais de uma rodada no dia
/// ([[0xb67a50]+0x2a] > 1) a lista dos grupos do dia ([+1], 0x18 bytes, [+9] quantos) com as partidas
/// de cada um ([+0xd], 84 bytes, de +4 a +8 do grupo): cabeçalho do grupo (Copa do Mundo, Internacional,
/// Continental ou o nome da nação) e uma linha por partida (0x55f990). Depois a tabela de 4 colunas
/// com as linhas que cabem (0x5e5cd0, a altura da fonte 3; quantas no parâmetro 0xf, o primeiro item no 0x10),
/// os três botões do alto (0x804030, 6 a 8) e o redesenho dos placares (0x724520).
pub(crate) unsafe extern "C" fn draw_latest_scores_panel(topo: *mut i32, base: *mut i32) {
    crate::comparacao::ramo("telas painel dos ultimos placares");
    type T9 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T11 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T12 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    type T18 = unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let titulo: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let caixa: T12 = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let celula: T11 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let texto_da_celula: T9 = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let linha: T18 = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let lista: T11 = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let cor = |a: usize| *(a as *const u16) as usize;
    let display = || *((0xb74c4c + *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize * 0x300) as *const usize);
    let sm = GERENTE_DE_TELAS;
    const BUF: usize = 0xde1f64;
    let sem_selo = |c: usize| {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
        let selecao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
        amistoso(*(G_FRIENDLY as *const usize), c) == 0 && selecao(*(G_RESERVE_COMP as *const usize), c) == 0
    };

    let partida = parametro_da_tela(0);
    let entrada = parametro_da_tela(0xe);
    let comp = (*(G_CLUB_COMP_TABLE as *const usize) as isize + *(entrada as *const i32) as isize * 107) as usize;
    let t = *topo as usize;
    let (esquerda, largura);
    if partida != 0 {
        esquerda = 0x6e;
        largura = 0x30c;
        let mais_de_uma = core::ptr::read_unaligned((entrada + 8) as *const i32) > core::ptr::read_unaligned((entrada + 4) as *const i32);
        let chave = *(comp as *const usize);
        if chave == *(KEY_CLUB_COMP_FRIENDLY as *const usize) && mais_de_uma {
            texto(BUF, 0xa16b28, 0xad9c64);
        } else if chave == *(KEY_CLUB_COMP_RESERVE as *const usize) && mais_de_uma {
            texto(BUF, 0xa16aec, 0xad9c64);
        } else {
            let g = *((comp + competition::SHORT_GENDER_NAME) as *const i8) as i32;
            texto(BUF, 0xa16ac8, g, g, comp + competition::SHORT_NAME);
        }
        titulo(sm, 0x6e, t, 0x30c, t + 0x23, 2, 0, 0xc, 3, cor(0xaebdec), BUF, 1);
        if sem_selo(comp) {
            caixa(sm, 0x9b, t, 0x276, t + 0x23, 1, 0, 0xc, 3, cor(0xaebdec), 0, 0x7d0, comp);
        }
        *topo += 0x28;
    } else {
        esquerda = 0x64;
        largura = 0x316;
        nome_da_competicao_no_painel(comp, entrada);
        titulo(sm, 0x64, t, 0x316, t + 0x2d, 2, 0, 0xc, 6, cor(0xaebdec), BUF, 1);
        if sem_selo(comp) {
            caixa(sm, 0x91, t, 0x280, t + 0x2d, 1, 0, 0xc, 6, cor(0xaebdec), 0, 0x7d0, comp);
        }
        *topo += 0x41;
    }

    let md = *(G_MATCH_DAY as *const usize);
    if *((md + 0x2a) as *const i8) > 1 {
        let cabeca = caixa(sm, esquerda + 5, t + 5, esquerda + 0x14, t + 0x19, 0x30, cor(0xaebdea), 0xc, 1, cor(0xae3184), 0, 0, 0);
        let h = lista(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor(0xae31aa), cabeca);
        let grupos = core::ptr::read_unaligned((md + 9) as *const i32);
        for i in 0..grupos.max(0) as usize {
            let g = core::ptr::read_unaligned((md + 1) as *const usize) + i * 0x18;
            if *((g + 0x14) as *const u32) == 0 {
                continue;
            }
            let pai = if *((md + 0x29) as *const u8) == 1 {
                h
            } else {
                let tipo = *(g as *const i32);
                match tipo {
                    0 => {
                        texto(BUF, 0xa16284);
                    }
                    1 => {
                        texto(BUF, 0xa16248, 0xad9c64);
                    }
                    3..=7 => {
                        texto(BUF, 0xa16a90, 0xad9c64);
                    }
                    _ => {
                        let nacao = *((0xb63d60 + tipo as isize * 72) as *const usize);
                        if nacao != 0 {
                            copiar_para_o_texto(nacao + 4);
                        } else {
                            texto(BUF, 0xa16a90, 0xad9c64);
                        }
                    }
                }
                let r = linha(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor(0xae31aa), cor(0xaebdec), 1, 1, cor(0xaebdea), BUF, 0, 0, 0, h);
                lista(display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, cor(0xae31aa), r)
            };
            let mut k = *((g + 4) as *const i32);
            while k <= *((g + 8) as *const i32) {
                let m = (core::ptr::read_unaligned((md + 0xd) as *const usize) as isize + k as isize * 84) as usize;
                if *((m + 0x10) as *const u32) != 0 {
                    let c = (*(G_CLUB_COMP_TABLE as *const usize) as isize + *(m as *const i32) as isize * 107) as usize;
                    nome_da_competicao_no_painel(c, m);
                    linha(display(), 2, 0, 0, 0, 0, 0, 0, 0x10, cor(0xae31aa), cor(0xaebdec), 1, 1, cor(0xaebdea), BUF, 0, 5, m, pai);
                }
                k += 1;
            }
        }
    }

    let altura: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let a = altura(3);
    let n = ((*base - *topo - 0x10) / a) as u8;
    gravar_parametro_da_tela(0xf, n as usize);
    let mut colunas: [u8; 4] = [0x16, 4, 0x16, 4];
    // a original passa o byte das linhas por cima do ponteiro `base` na pilha (os 3 bytes de cima ficam)
    let linhas = (base as usize & !0xff) | n as usize;
    let tabela = lista(display(), 0x6e, *topo as usize, 0x30c, *base as usize, 4, colunas.as_mut_ptr() as usize, linhas, 0, 2, 0, usize::MAX);
    for s in 0..n as usize {
        let r = celula(sm, tabela, 0, s, 1, 0, 1, 3, cor(0xaebde8), 0, 0, 0) as u16 as i16;
        if s == 0 {
            gravar_parametro_da_tela(0x10, r as i32 as usize);
        }
        for _ in 0..3 {
            texto_da_celula(sm, tabela, 1, s, 1, 0, 0xc, 3, cor(0xaebdec), 0);
        }
        celula(sm, tabela, 2, s, 1, 0, 1, 3, cor(0xaebde8), 0, 0, 0);
        texto_da_celula(sm, tabela, 3, s, 1, 0, 0xc, 2, cor(0xae3184), 0);
    }
    let marca = if *(G_FONT_MODE as *const i32) != -1 { 0 } else { 1 };
    caixa(sm, largura - 0x7c, t + 5, largura - 0x68, t + 0x19, 1, cor(0xaebe04), 0xc, marca, cor(0xae3184), 0, 6, 0);
    caixa(sm, largura - 0x66, t + 5, largura - 0x1b, t + 0x19, 1, 0, 0xc, 2, cor(0xaebdec), 0, 7, 0);
    caixa(sm, largura - 0x19, t + 5, largura - 5, t + 0x19, 1, cor(0xaebe04), 0xc, marca, cor(0xae3184), 0, 8, 0);
    let redesenhar: unsafe extern "C" fn() = core::mem::transmute(FN_REFRESH_LATEST_SCORES_ROWS);
    redesenhar();
}

/// O painel é comparado de verdade: as duas desenham a partir do mesmo estado e o gerente de telas, o
/// `Display` da tela atual, os parâmetros (menos o 0x14, a hora que o redesenho 0x724520 grava), o
/// `*topo` e o texto 0xde1f64 têm de sair iguais.
pub(crate) unsafe extern "C" fn comparar_draw_latest_scores_panel(topo: *mut i32, base: *mut i32) {
    let o = orig::<unsafe extern "C" fn(*mut i32, *mut i32)>(IDX_DRAW_LATEST_SCORES_PANEL);
    let mut reg = regioes_da_tela();
    let vaga = *((GERENTE_DE_TELAS + 0x382a) as *const i16) as isize;
    let bloco = core::ptr::read_unaligned((GERENTE_DE_TELAS as isize + 0x7e2 + vaga * 0x300) as *const usize);
    if bloco != 0 {
        reg.push((bloco, 0xb4));
        reg.push((bloco + 0xb8, 0x200 - 0xb8));
    }
    reg.push((topo as usize, 4));
    reg.push((0xde1f64, 0x100));
    // a lista (0x55fba0) escreve num buffer de rede ([0xae2908] na posicao [0xae290c]): a posicao volta
    // para a segunda rodada escrever no mesmo lugar
    reg.push((0xae290c, 4));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(topo, base), || draw_latest_scores_panel(topo, base));
    PLACAR.registrar(igual, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("draw_latest_scores_panel: diferencas em {:?}", dif)
    });
}
