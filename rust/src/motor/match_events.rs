use super::*;

// ---------- centesima quinquagesima quarta leva: os eventos gravados e o passo em rede (0x6d98b0, 0x6d9c40, 0x6b55c0) ----------

/// The two match event file names in CM3_TEMP (human or computer).
///
/// [pt] Os dois arquivos de eventos (humano ou computador) em CM3_TEMP.
pub(crate) fn match_event_files(humano: bool) -> (usize, usize) {
    if humano { (0xa15a8c, 0xa15a78) } else { (0xa15a60, 0xa15a48) }
}

/// records the events (+0x720, 0xe bytes each) in one of the two "events1/2.tmp" files (human or computer) in CM3_TEMP;
/// header version 1, number, count; with both full (2500), recreates the oldest. In `desc`: position, count, number,
/// human
/// Original: `record_events` (0x6d98b0, match_events.cpp, thiscall, ret 8).
///
/// [pt] { 0x6d98b0 } (thiscall, ret 8, match_events.cpp): grava os +6 eventos (0xe bytes cada, em +0x720)
/// do registro `e` num dos dois arquivos "events1/2.tmp" (humano ou computador). Cabecalho: versao 1
/// (senao erro 0xb5c/0xb7b), numero, quantos. Usa o primeiro com menos de 2500; cheios os dois,
/// recria o mais antigo com o numero seguinte de [0xb6b7c0]+0x18 (sem abrir, erro 0xba4). Em `desc`:
/// posicao, quantos, numero e o byte `humano`. Devolve se gravou.
pub(crate) unsafe extern "thiscall" fn record_events(e: usize, desc: *mut u32, humano: u8) -> u8 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let ler: unsafe extern "C" fn(*mut i32, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let posicao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CRT_FTELL);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = match_event_files(humano != 0);
    let (mut versao, mut quantos, mut numero) = (0i32, 0i32, 0i32);
    let mut ids = [-1i32, -1];
    let mut aberto = 0usize;
    for (k, nome) in [um, dois].into_iter().enumerate() {
        let f = abrir(caminho(0x987da0, nome), 0x9c7848);
        if f == 0 {
            continue;
        }
        buscar(f, 0, 0);
        ler(&mut versao, 4, 1, f);
        ler(&mut ids[k], 4, 1, f);
        ler(&mut quantos, 4, 1, f);
        numero = ids[k];
        if versao != 1 {
            show_error_at(0xa159b0, if k == 0 { 0xb5c } else { 0xb7b });
            fechar(f);
            return 0;
        }
        if quantos < 0x9c4 {
            crate::comparacao::ramo("eventos arquivo com espaco");
            aberto = f;
            break;
        }
        fechar(f);
    }
    let f = if aberto != 0 {
        aberto
    } else {
        let nome = if ids[0] == -1 || (ids[1] != -1 && ids[0] <= ids[1]) { um } else { dois };
        let f = abrir(caminho(0x987da0, nome), 0x9a7f28);
        if f == 0 {
            show_error_at(0xa159b0, 0xba4);
            return 0;
        }
        versao = 1;
        crate::comparacao::ramo("eventos arquivo novo");
        quantos = 0;
        let r = *(G_MATCH_STATS as *const usize) + 0x18;
        numero = *(r as *const i32) + 1;
        *(r as *mut i32) = numero;
        escrever(&versao as *const i32 as usize, 4, 1, f);
        escrever(&numero as *const i32 as usize, 4, 1, f);
        escrever(&quantos as *const i32 as usize, 4, 1, f);
        f
    };
    buscar(f, 0, 2);
    *desc = posicao(f);
    let n = *((e + 6) as *const i16) as i32;
    *desc.add(1) = n as u32;
    *desc.add(2) = numero as u32;
    *(desc.add(3) as *mut u8) = humano;
    let r = escrever(e + 0x720, 0xe, *((e + 6) as *const i16) as i32 as u32, f);
    quantos += 1;
    buscar(f, 8, 0);
    escrever(&quantos as *const i32 as usize, 4, 1, f);
    fechar(f);
    (r != 0) as u8
}

/// reads back the events described in `desc` from the file with the right number
/// Original: `read_events` (0x6d9c40, match_events.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d9c40 } (thiscall, ret 4, match_events.cpp): le de volta os eventos descritos em `desc`
/// (posicao, quantos, numero, humano) para +0x720 do registro `e` (+6 = quantos): procura o arquivo
/// com o numero (versao 1, senao erro 0xbf6/0xc11, com o arquivo aberto como na original). Devolve
/// se leu.
pub(crate) unsafe extern "thiscall" fn read_events(e: usize, desc: *const u32) -> u8 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = match_event_files(*(desc.add(3) as *const u8) != 0);
    let (mut versao, mut numero) = (0i32, 0i32);
    for (k, nome) in [um, dois].into_iter().enumerate() {
        let f = abrir(caminho(0x987da0, nome), 0x98f928);
        if f == 0 {
            if k == 0 {
                continue;
            }
            return 0;
        }
        ler(&mut versao as *mut i32 as usize, 4, 1, f);
        ler(&mut numero as *mut i32 as usize, 4, 1, f);
        if versao != 1 {
            // como na original: o arquivo fica aberto
            show_error_at(0xa159b0, if k == 0 { 0xbf6 } else { 0xc11 });
            return 0;
        }
        if numero as u32 == *desc.add(2) {
            crate::comparacao::ramo("eventos lidos");
            buscar(f, *desc as i32, 0);
            let r = ler(e + 0x720, 0xe, *desc.add(1), f);
            *((e + 6) as *mut u16) = *desc.add(1) as u16;
            fechar(f);
            return (r != 0) as u8;
        }
        fechar(f);
    }
    0
}


// ponytail: os eventos gravados mexem em arquivos e o passo em rede so roda em jogo em rede; sem
// comparacao propria, validados pela carreira (Rust x original).
pub(crate) unsafe extern "thiscall" fn comparar_record_events(e: usize, desc: *mut u32, humano: u8) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, u8) -> u8>(IDX_RECORD_EVENTS)(e, desc, humano)
}
/// Ler os eventos so le arquivo (sem efeito fora do registro): a original e a copia leem os mesmos
/// eventos (+6 quantos, +0x720 os eventos de 0xe bytes) e o resultado e conferido. Nasceu do erro
/// `Area.cpp 507` ao rever o relatorio de uma partida antiga (17/10/2026).
pub(crate) unsafe extern "thiscall" fn comparar_read_events(e: usize, desc: *const u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(IDX_READ_EVENTS);
    if desc.is_null() {
        return o(e, desc);
    }
    let n = (*desc.add(1) as usize).min(0x10000);
    let reg = [(e + 6, 2), (e + 0x720, 0xe * n)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(e, desc), || read_events(e, desc));
    PLACAR.registrar(igual && r == rr, || format!("read_events(pos {} n {} numero {} humano {}) original={} rust={} {:x?} x {:x?}", *desc, *desc.add(1), *desc.add(2), *(desc.add(3) as *const u8), r, rr, mo, mr));
    r
}

// ---------- centesima quinquagesima quinta leva: o leitor dos eventos da partida (0x6d9e50) ----------

/// Like show_error_at (error box at file/line), then cleanup and exit(-1) like the original.
///
/// [pt] Como `show_error_at`, e o jogo para (como a original: 0x5cc7d0, 0x5a8f60 e sai com -1).
pub(crate) unsafe fn fatal_error_at(arquivo: usize, linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    show_error_at(arquivo, linha);
    fechar1();
    fechar2();
    sair(-1)
}

/// reads the `events_<language>.cfg` (or `events_eng.cfg`) into the table of the 677 event codes at [0xb68788] (7
/// option bytes, the 0x110-byte sounds with weight, type, name and the two texts `G`/`R`); checks `<X` tokens, events
/// without text and, with `-verifysounds`, the sound files; at the end converts the weights and gives the default sound
/// to those without one
/// Original: `read_events_cfg` (0x6d9e50, match_events.cpp (~5.2 KB), cdecl).
///
/// [pt] { 0x6d9e50 } (match_events.cpp, ~5,2 KB): le o `events_<idioma>.cfg` (CM3_DATA; sem ele, o
/// `events_eng.cfg`, e sem este o jogo para, erro 0xcc4) para a tabela dos 677 codigos de evento em
/// [0xb68788] (0x12 bytes: 7 bytes de opcoes, +8 os sons de 0x110 bytes, +0xc quantos, +0xe o peso).
/// Com outro idioma desde a ultima leitura, 0x6d5b50 limpa antes; ja lida ([0xb6b723]), nada.
/// Linhas: `=id,o0..o6` abre um evento (o4 entre 0 e 11; o2 sem o0 avisa 0xd40), `>peso,tipo,nome`
/// acrescenta um som (com `-verifysounds` na linha de comando, confere o arquivo em CM3_SOUNDS),
/// `G`/`R` os dois textos do ultimo som (alocados; marcas `<X` desconhecidas avisam). No fim, os
/// pesos viram (maior x 10) / peso e +0xe a soma; evento sem som ganha o som padrao (0xb68670).
/// Le pelo `fgetc` do jogo e usa o buffer [0xde1f64], como a original.
pub(crate) unsafe extern "C" fn read_events_cfg() {
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let limpar: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_EVENTS_FREE_EVENT_TABLES);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let ler: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_9469E0);
    let numero: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_947311);
    let numero_curto: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_94739C);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let procurar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_STRSTR);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_UTILS_FILE_EXISTS);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    const FONTE: usize = 0xa159b0; // match_events.cpp
    const TABELA: usize = 0xb68788;
    const FIM: usize = 0xb6b723;
    const BUF: usize = 0xde1f64;
    let evento = |k: i32| TABELA.wrapping_add((k as isize * 0x12) as usize);
    let sons = |e: usize| core::ptr::read_unaligned((e + 8) as *const usize);
    let quantos = |e: usize| core::ptr::read_unaligned((e + 0xc) as *const i16);
    let peso = |e: usize| (e + 0xe) as *mut i32;

    if *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *const i32) != idioma() {
        limpar();
    }
    if *(FIM as *const u8) != 0 {
        return;
    }
    let mut e = TABELA;
    while e < FIM {
        *(e as *mut u8) = 0xff;
        for o in 1..=7 {
            *((e + o) as *mut u8) = 0;
        }
        core::ptr::write_unaligned((e + 8) as *mut u32, 0);
        core::ptr::write_unaligned((e + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned(peso(e), 0);
        e += 0x12;
    }
    let lingua = idioma();
    *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *mut i32) = lingua;
    let nome = match lingua {
        2 => 0xa15cec,
        3 => 0xa15cdc,
        4 => 0xa15ccc,
        5 => 0xa15cbc,
        6 => 0xa15c6c,
        7 => 0xa15cac,
        8 => 0xa15c9c,
        9 => 0xa15c8c,
        10 => 0xa15c7c,
        11 => 0xa15c5c,
        12 => 0xa15c4c,
        _ => 0,
    };
    let mut f = if nome != 0 { abrir(caminho(0x9b5348, nome), 0x9899fc) } else { 0 };
    if f != 0 {
        *(FIM as *mut u8) = 1;
    } else {
        // ponytail: a original testa [0xb6b723] de novo aqui; ele acabou de ser visto zerado
        *(FIM as *mut u8) = 1;
        f = abrir(caminho(0x9b5348, 0xa15c3c), 0x9899fc);
        if f == 0 {
            fatal_error_at(FONTE, 0xcc4);
        }
    }

    let fim_de_arquivo = || *((f + 0xc) as *const u8) & 0x10 != 0;
    let pula_brancos_e_le = || {
        let mut c = ler(f);
        while (c == 0x20 || c == 0xd) && c != -1 {
            c = ler(f);
        }
        c
    };
    // um campo ate a virgula ou o fim da linha, no buffer; devolve o ultimo caractere lido
    let campo = || {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0x2c && c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = ler(f);
        }
        *((BUF + n) as *mut u8) = 0;
        c
    };
    // a linha inteira (virgulas inclusive) num texto alocado; marcas `<X` desconhecidas avisam
    let texto = |linha_alocacao: u32, linha_marca: u32| -> usize {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = ler(f);
        }
        *((BUF + n) as *mut u8) = 0;
        let t = alocar(n as u32 + 1);
        if t == 0 {
            fatal_error_at(FONTE, linha_alocacao);
        }
        core::ptr::copy_nonoverlapping(BUF as *const u8, t as *mut u8, n + 1);
        let mut i = 0usize;
        while *((t + i) as *const u8) != 0 {
            if *((t + i) as *const u8) == b'<' {
                let x = *((t + i + 1) as *const i8);
                if !b"CDFNOPRTYcdfnoprty".contains(&(x as u8)) {
                    formatar_texto(BUF, 0xa15b74, x as i32);
                    show_error_at(FONTE, linha_marca);
                }
            }
            i += 1;
        }
        t
    };

    let mut atual = -1i32;
    while !fim_de_arquivo() {
        let c = ler(f);
        if c == -1 {
            continue;
        }
        match c {
            0xd => {}
            0x3d => {
                // `=`: confere o evento anterior e abre o novo
                if atual != -1 {
                    let e = evento(atual);
                    let s = sons(e);
                    if s == 0 {
                        formatar(BUF, 0xa15b38, atual);
                        fatal_error_at(FONTE, 0xcdf);
                    }
                    for j in 0..quantos(e).max(0) as usize {
                        if core::ptr::read_unaligned((s + 0x110 * j + 0x108) as *const usize) == 0 {
                            formatar(BUF, 0xa15af0, atual);
                            fatal_error_at(FONTE, 0xcea);
                        }
                    }
                }
                campo();
                atual = numero(BUF);
                if !(0..=0x2a4).contains(&atual) {
                    formatar(BUF, 0xa15aa0, atual);
                    fatal_error_at(FONTE, 0xcfb);
                }
                let e = evento(atual);
                let mut c = 0;
                for o in 0..7usize {
                    c = campo();
                    let v = numero_curto(BUF) as u8;
                    *((e + o) as *mut u8) = v;
                    if o == 4 {
                        let b = (e + 4) as *mut i8;
                        if *b < 0 {
                            *b = 0;
                        }
                        if *b > 0xb {
                            *b = 0xb;
                        }
                    }
                }
                while c != 0xa && c != -1 {
                    c = ler(f);
                }
                if *((e + 2) as *const u8) != 0 && *(e as *const u8) == 0 {
                    formatar(BUF, 0xa15bf0, atual);
                    show_error_at(FONTE, 0xd40);
                }
            }
            0x3e => {
                // `>`: mais um som no evento aberto
                if atual == -1 {
                    fatal_error_at(FONTE, 0xd49);
                }
                let e = evento(atual);
                let novo = realocar(sons(e), (quantos(e) as i32 + 1) as u32 * 0x110);
                if novo == 0 {
                    fatal_error_at(FONTE, 0xd50);
                }
                core::ptr::write_unaligned((e + 8) as *mut usize, novo);
                let s = novo + 0x110 * quantos(e) as usize;
                campo();
                *(s as *mut i16) = numero_curto(BUF) as i16;
                let p = (*(s as *const i16) as i32).max(*peso(e));
                core::ptr::write_unaligned(peso(e), p);
                campo();
                *((s + 2) as *mut i16) = numero_curto(BUF) as i16;
                let mut c = ler(f);
                *((s + 4) as *mut u8) = 0;
                while (c == 0x20 || c == 0xd) && c != -1 {
                    c = ler(f);
                }
                let mut n = 0usize;
                while c != 0xa && c != -1 {
                    n += 1;
                    *((s + 3 + n) as *mut u8) = c as u8;
                    c = ler(f);
                }
                *((s + 4 + n) as *mut u8) = 0;
                if procurar(0xae2c98, 0xa15be0) != 0 && n != 0 && existe(caminho(0x9c382c, s + 4)) == 0 {
                    formatar(BUF, 0xa15bac, caminho(0x9c382c, s + 4));
                    caixa(0x9870e0, BUF, 0);
                }
                core::ptr::write_unaligned((s + 0x108) as *mut usize, 0);
                core::ptr::write_unaligned((s + 0x10c) as *mut usize, 0);
                core::ptr::write_unaligned((e + 0xc) as *mut i16, quantos(e) + 1);
            }
            0x47 | 0x67 | 0x52 | 0x72 => {
                // `G`/`R`: os dois textos do ultimo som
                let (o, sem, cheio, ja, alocacao, marca) = if c == 0x47 || c == 0x67 {
                    (0x108usize, 0xd80, 0xd82, 0xd84, 0xd97, 0xdc3)
                } else {
                    (0x10c, 0xdcf, 0xdd1, 0xdd3, 0xde6, 0xe12)
                };
                if atual == -1 {
                    fatal_error_at(FONTE, sem);
                }
                let e = evento(atual);
                if quantos(e) == 0 {
                    fatal_error_at(FONTE, cheio);
                }
                let campo_texto = (sons(e) as isize + quantos(e) as isize * 0x110 - 0x110 + o as isize) as usize;
                if core::ptr::read_unaligned(campo_texto as *const usize) != 0 {
                    fatal_error_at(FONTE, ja);
                }
                let t = texto(alocacao, marca);
                // a original grava o ponteiro antes de copiar; o resultado e o mesmo
                core::ptr::write_unaligned(campo_texto as *mut usize, t);
            }
            _ => {
                let mut c = c;
                while c != 0xa && c != -1 {
                    c = ler(f);
                }
            }
        }
    }

    // os pesos: (maior x 10) / peso de cada som, e +0xe a soma; evento sem som ganha o padrao
    for k in 0..0x2a5i32 {
        let e = evento(k);
        if sons(e) == 0 {
            core::ptr::write_unaligned((e + 8) as *mut usize, 0xb68670);
            core::ptr::write_unaligned((e + 0xc) as *mut i16, 1);
            formatar(BUF, 0xa15b38, k);
            show_error_at(FONTE, 0xe46);
        } else {
            let total = (*peso(e)).wrapping_mul(10);
            core::ptr::write_unaligned(peso(e), total);
            let mut soma = 0i32;
            for j in 0..quantos(e).max(0) as usize {
                let s = (sons(e) + 0x110 * j) as *mut i16;
                *s = (core::ptr::read_unaligned(peso(e)) / *s as i32) as i16;
                soma += *s as i32;
            }
            core::ptr::write_unaligned(peso(e), soma);
        }
    }
    fechar(f);
}

// ponytail: le arquivo, aloca os textos e grava a tabela global; sem comparacao propria, validada
// pela carreira (Rust x original) e pela comparacao da tabela no fim da leitura.
pub(crate) unsafe extern "C" fn comparar_read_events_cfg() {
    original_fn::<unsafe extern "C" fn()>(IDX_READ_EVENTS_CFG)()
}

// ---------- centesima quinquagesima sexta leva: o texto dos eventos e o comeco da partida assistida (0x6d8d10, 0x6b5260) ----------

/// the event sound's `R` text with the tokens replaced (`<P1>` player, `<T1>`/`<T2>` club, `<C>` competition, `<F>`
/// stage, `<D>`/`<N>` number, `<rf>`/`<Rf>` referee), `{...}` prefix and suffix (0x6701f0) and upper case; writes to
/// [0xb67ea0]
/// Original: `event_text` (0x6d8d10, match_events.cpp, stdcall, ret 0x10).
///
/// [pt] { 0x6d8d10 } (stdcall, ret 0x10, match_events.cpp): o texto do evento `ev` (+0 codigo, +2 qual
/// som, +5/+6 e +7/+8 os dois jogadores (lado, indice), +9 um numero) da partida `ctx` (+0/+4 os
/// clubes, +0x40 o arbitro), com `comp` a competicao (-1 sem) e `variante` o trecho entre `|` (0 o
/// primeiro). Le o texto `R` (+0x10c) do som escolhido (no maximo o ultimo) e troca as marcas:
/// `<P1>`/`<P>` jogador (0x6db2d0), `<T1>`/`<T2>` clube, `<C>` competicao, `<F>` a fase dela
/// (0x4bfa30/0x4bff70), `<D>`/`<D1>` o numero (+1), `<N>` o numero por extenso (0x90c310), `<rf>`/`<Rf>`
/// o arbitro ("Mr ..." ou "the referee" conforme o idioma e um resto de divisao por 5), `<O>`/`<Y>` nada.
/// `{...}` antes ou depois da marca sao prefixo e sufixo (0x6701f0, que aplica artigo e genero);
/// a letra seguinte `C` poe em maiusculas (0x90c2d0). Escreve em [0xb67ea0] (e devolve), recontando
/// o tamanho por `strlen` depois de cada marca, como a original; sem evento, sem texto ou sem o
/// trecho pedido, 0. Marca desconhecida avisa 0xb10.
pub(crate) unsafe extern "stdcall" fn event_text(ev: usize, ctx: usize, comp: i32, variante: u32) -> usize {
    const BUF: usize = 0xb67ea0;
    const TEMP: usize = 0xb67dd8;
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_FORMAT_PLAYER_NAME);
    let compor: unsafe extern "C" fn(usize, *const u8, *const u8, u32, u32) -> usize = core::mem::transmute(FN_LANGLIB_APPLY_GRAMMAR_MODIFIER);
    let maiuscula: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_UTILS_CHAR_TO_UPPER);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(FN_UTILS_FORMAT_ORDINAL);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_OFFICIAL_NAME);
    let fase: unsafe extern "C" fn(usize, u32, u32, u32, *mut u8, u32) = core::mem::transmute(FN_COMP_UTIL_FORMAT_COMP_STAGE_ROUND_LABEL);
    let criterio: unsafe extern "C" fn(u32, u32, u32, u32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LABEL_GENDER);
    if ev == 0 || ctx == 0 {
        return 0;
    }
    let k = (((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16) as isize;
    let e = (0xb68788isize + k * 18) as usize;
    let sons = core::ptr::read_unaligned((e + 8) as *const usize);
    if sons == 0 {
        return 0;
    }
    let n = core::ptr::read_unaligned((e + 0xc) as *const i16);
    let qual = (ev + 2) as *mut i16;
    if *qual as i32 > n as i32 - 1 {
        *qual = n.wrapping_sub(1);
    }
    let mut s = core::ptr::read_unaligned((sons as isize + *qual as isize * 0x110 + 0x10c) as *const usize);
    if s == 0 {
        return 0;
    }
    let v = variante as u16 as i16;
    if v > 0 {
        let mut resta = v as u8 as i8;
        let mut c = *(s as *const u8);
        if c != 0 {
            loop {
                if c == b'|' {
                    resta = resta.wrapping_sub(1);
                    if resta <= 0 {
                        s += 1;
                        break;
                    }
                }
                c = *((s + 1) as *const u8);
                s += 1;
                if c == 0 {
                    break;
                }
            }
        }
        if *(s as *const u8) == 0 {
            return 0;
        }
    }
    crate::comparacao::ramo("texto do evento");
    let t = |i: i16| *((s as isize + i as isize) as *const u8);
    let dest = |o: i16| (BUF as isize + o as isize) as usize;
    let b = |o: usize| *((ev + o) as *const u8);
    let mut pos: i16 = 0;
    let mut out: i16 = 0;
    // o prefixo `{...}` logo antes da marca
    let prefixo = |pos: i16, p: &mut [u8; 0x100]| {
        p[0] = 0;
        if pos > 1 && (t(pos - 1) == b'}' || t(pos - 1) == b']') {
            let mut a = pos - 2;
            while a >= 0 && t(a) != b'{' && t(a) != b'[' {
                a -= 1;
            }
            let j = a as i32;
            let fim = pos as i32 - j - 1;
            let mut c = 1i32;
            while c < fim {
                p[(c - 1) as usize] = t((j + c) as i16);
                c += 1;
            }
            p[(c - 1) as usize] = 0;
        }
    };
    // o sufixo `{...}` logo depois do `>` da marca (a procura comeca em `inicio`)
    let sufixo = |inicio: i16, x: &mut [u8; 0x100]| {
        x[0] = 0;
        let mut fe = inicio;
        while t(fe) != b'>' && t(fe) != 0 {
            fe += 1;
        }
        if t(fe + 1) == b'{' || t(fe + 1) == b'[' {
            let mut j = fe + 2;
            while t(j) != b'}' && t(j) != 0 {
                x[(j - fe - 2) as usize] = t(j);
                j += 1;
            }
            x[(j - fe - 2) as usize] = 0;
        }
    };
    let escrever = |out: i16, r: usize| {
        formatar(dest(out), 0x991944, r);
    };
    let em_maiusculas = |out: i16, r: usize| {
        let mut j: i16 = 0;
        while *((r as isize + j as isize) as *const u8) != 0 {
            let c = maiuscula(*((r as isize + j as isize) as *const u8) as i32);
            *(dest(out + j) as *mut u8) = c;
            j += 1;
        }
        *(dest(out + j) as *mut u8) = 0;
    };
    let mut pre = [0u8; 0x100];
    let mut suf = [0u8; 0x100];
    let mut nome = [0u8; 0xd0];
    let mut arb = [0u8; 0xd0];
    let mut texto_fase = [0u8; 0x800];

    if t(0) != 0 {
        loop {
            let c = t(pos);
            if c == b'|' {
                break;
            }
            if c != b'<' {
                if c != b'{' && c != b'[' {
                    *(dest(out) as *mut u8) = c;
                    out += 1;
                } else {
                    // `{...}`/`[...]` solto: pula ate o fecho
                    let mut j = pos;
                    let mut a = c;
                    loop {
                        if a == b']' || a == 0 {
                            break;
                        }
                        j += 1;
                        a = t(j);
                        if a == b'}' {
                            break;
                        }
                    }
                    pos = j;
                }
            } else {
                let x = t(pos + 1) as i8 as i32;
                match x as u8 {
                    b'P' | b'p' => {
                        let (lado, idx) = if t(pos + 2) == b'1' && b(5) != 0xff { (b(5), b(6)) } else { (b(7), b(8)) };
                        if lado != 0xff {
                            nome_do_jogador(ctx, lado as i8 as i32, idx as i8 as i32, nome.as_mut_ptr(), 0xc8, 0, 0, 1);
                            sufixo(pos + 3, &mut suf);
                            let r = compor(nome.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                            if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                escrever(out, r);
                            }
                        }
                    }
                    b'T' | b't' => {
                        let dois = t(pos + 2) == b'2';
                        let casa = b(6) != 0;
                        let id = if dois == casa { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                        let clube = (*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(0x245));
                        prefixo(pos, &mut pre);
                        sufixo(pos + 2, &mut suf);
                        let g = *((clube + club::SHORT_GENDER_NAME) as *const u8) as u32;
                        let r = compor(clube + club::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g);
                        if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                            em_maiusculas(out, r);
                        } else {
                            escrever(out, r);
                        }
                    }
                    b'C' | b'c' | b'F' | b'f' => {
                        if comp >= 0 {
                            let cp = (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let r = if x as u8 == b'C' || x as u8 == b'c' {
                                let g = *((cp + 0x52) as *const u8) as u32;
                                compor(cp + 0x38, pre.as_ptr(), suf.as_ptr(), g, g)
                            } else {
                                fase(cp, 0, 0x96, 3, texto_fase.as_mut_ptr(), 0x7d0);
                                let g = criterio(0, 0, 0x96, 3) as u32;
                                compor(texto_fase.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                            };
                            escrever(out, r);
                        }
                    }
                    b'D' | b'd' => {
                        let numero = core::ptr::read_unaligned((ev + 9) as *const i32);
                        formatar(dest(out), 0x98806c, if t(pos + 2) == b'1' { numero.wrapping_add(1) } else { numero });
                    }
                    b'N' | b'n' => {
                        let r = por_extenso(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                        escrever(out, r);
                    }
                    b'R' | b'r' => {
                        if t(pos + 2) == b'f' {
                            let fixo = if x as u8 == b'R' { 0xa15a14usize } else { 0xa159e0 };
                            let arbitro = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                            if arbitro <= -1 {
                                formatar_texto(dest(out), fixo);
                            } else {
                                let resto = ((b(7) as i8 as i32 + b(5) as i8 as i32 + core::ptr::read_unaligned((ctx + 4) as *const i32)) % 5) as i8;
                                let ficha = (*(G_OFFICIAL_TABLE as *const usize)).wrapping_add((arbitro as usize).wrapping_mul(0x2b));
                                let modo = if idioma() == 4 && resto == 0 {
                                    1
                                } else if resto == 0 {
                                    5
                                } else if resto > 0 && resto <= 2 {
                                    1
                                } else {
                                    0
                                };
                                match modo {
                                    1 => {
                                        nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 1);
                                        formatar_texto(TEMP, 0xa15a20, arb.as_ptr());
                                    }
                                    5 => {
                                        nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 5);
                                        formatar_texto(TEMP, 0xa159ec, arb.as_ptr());
                                    }
                                    _ => {
                                        formatar_texto(TEMP, if x as u8 == b'R' { 0xa15a14 } else { 0xa159e0 });
                                    }
                                }
                                // como na original: o texto pronto vira o formato
                                formatar(dest(out), TEMP);
                            }
                        }
                    }
                    b'O' | b'o' | b'Y' | b'y' => {}
                    _ => show_error_at(0xa159b0, 0xb10),
                }
                // fim da marca: ate o `>`; o tamanho e recontado
                if t(pos) != b'>' {
                    let mut j = pos;
                    loop {
                        if t(j) == 0 {
                            break;
                        }
                        j += 1;
                        if t(j) == b'>' {
                            break;
                        }
                    }
                    pos = j;
                }
                if t(pos) == 0 {
                    pos -= 1;
                }
                let mut l = 0usize;
                while *((BUF + l) as *const u8) != 0 {
                    l += 1;
                }
                out = l as i16;
            }
            pos += 1;
            if t(pos) == 0 {
                break;
            }
        }
    }
    *(dest(out) as *mut u8) = 0;
    BUF
}


/// O texto do evento so escreve nos buffers de texto ([0xb67ea0], [0xb67dd8]) e no `qual` do evento:
/// a original e a copia montam o mesmo texto e o resultado e conferido.
pub(crate) unsafe extern "stdcall" fn comparar_event_text(ev: usize, ctx: usize, comp: i32, variante: u32) -> usize {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i32, u32) -> usize>(IDX_EVENT_TEXT);
    if ev == 0 {
        return o(ev, ctx, comp, variante);
    }
    let reg = [(ev + 2, 2), (0xb67ea0, 0x400), (0xb67dd8, 0xc8)];
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(ev, ctx, comp, variante) as u32, || event_text(ev, ctx, comp, variante) as u32);
    PLACAR.registrar(igual && r == rr, || format!("event_text(codigo {:#x} variante {}) original={:#x} rust={:#x} {:x?} x {:x?}", *(ev as *const u16), variante, r, rr, mo, mr));
    r as usize
}
// ponytail: so roda em jogo em rede (cria e libera motor e resumo); sem comparacao propria.

// ---------- centesima quinquagesima setima leva: a narracao no painel (0x6d7b00) ----------

/// restores the crowd sound, builds the event's `G` text (like 0x6d8d10) into a line with the club colours (`<O>` the
/// other's, `<Y>`/`<R>` the default ones), wraps or cuts with "..." by the width (0x5e5ad0, 0x90cb20), draws on the
/// panel (0x801040…0x800580), plays the event sound (0x8025e0) and marks the time and duration (+0x2d6c, +0x2dac)
/// Original: `commentate_on_panel` (0x6d7b00, match_events.cpp (~4.4 KB), thiscall, ret 0x20).
///
/// [pt] { 0x6d7b00 } (thiscall, ret 0x20, match_events.cpp, ~4,4 KB): o narrador `nar` escreve o evento `ev`
/// da partida `ctx` no `painel` (0xb74450) e toca o som dele. Antes, o ruido da torcida do humano da
/// vez ([0xb77c7a]; canal +0x20, "fxloop2.wav") e reposto se parou. Com o codigo valido e a opcao 1
/// zerada, monta o texto `G` (+0x108) do som escolhido (o trecho +0xd entre `|`) como a 0x6d8d10, mas
/// numa linha propria e com cores: comeca nas do clube do lance (0x53b640 com o uniforme do lado),
/// `<O>` passa as do outro, `<Y>` e `<R>`/`<r>` sem `f` as padrao ([0xaebdea]/[0xaebdec] ou
/// [0xae31a8]); `<N>` e o numero, `<n>` por extenso; o nome do jogador e curto se o texto inteiro
/// cabe em menos de 300 (0x5e5ad0 com a `fonte`). Larga demais para `largura` - 16: quebra (0x90cb20)
/// ou corta numa palavra com "...". Desenha (0x801040, 0x801660, 0x801960, 0x801c70, 0x800d30,
/// 0x800580 com a opcao 3). Por fim, sem trecho (+0xd) e com volume (+0x2db0 >= 101), toca o som do
/// evento (0x8025e0 no canal pela opcao 6, lado pela opcao 5), e marca a hora e a duracao do som
/// (+0x2d6c, +0x2dac = peso x volume / 100).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn commentate_on_panel(nar: usize, ev: usize, ctx: usize, comp: i32, painel: u32, largura: i32, quebrar: u32, fonte: usize, modo: u32) -> u32 {
    let tocando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_SOUND_PLAYING);
    let parar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_SCREENMANAGER_STOP_SOUND);
    let tocar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_PLAY_SOUND);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let largura_do_texto: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(FN_FONT_TEXT_WIDTH);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u8, *mut u8) = core::mem::transmute(FN_DATABASE_GET_CLUB_KIT_COLOURS);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_MATCH_EVENTS_FORMAT_PLAYER_NAME);
    let compor: unsafe extern "C" fn(usize, *const u8, *const u8, u32, u32) -> usize = core::mem::transmute(FN_LANGLIB_APPLY_GRAMMAR_MODIFIER);
    let maiuscula: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_UTILS_CHAR_TO_UPPER);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(FN_UTILS_FORMAT_ORDINAL);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_OFFICIAL_NAME);
    let fase: unsafe extern "C" fn(usize, u32, u32, u32, *mut u8, u32) = core::mem::transmute(FN_COMP_UTIL_FORMAT_COMP_STAGE_ROUND_LABEL);
    let criterio: unsafe extern "C" fn(u32, u32, u32, u32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LABEL_GENDER);
    let quebrar_linha: unsafe extern "C" fn(*mut u8, u32, u32) = core::mem::transmute(FN_UTILS_BREAK_LINE_NEAR_MIDDLE);
    let cor_de_fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_COLOUR);
    let fonte_do_painel: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_FONT);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT_COLOUR);
    let texto_do_painel: unsafe extern "thiscall" fn(usize, u32, *const u8) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_TEXT);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_STYLE);
    let mostrar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_SCREENMANAGER_SET_WIDGET_FLAGS);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    const SOM: usize = 0xb74450;
    const TABELA: isize = 0xb68788;

    if ev == 0 || ctx == 0 || painel as u16 == 0xffff {
        return 0;
    }
    let eu = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let goal_crowd_reaction = |n: usize| ((n as isize) + eu() + 0x20) as *mut u8;
    if *goal_crowd_reaction(nar) != 0xff && tocando(SOM, *goal_crowd_reaction(nar) as u32) == 0 {
        parar(SOM, *goal_crowd_reaction(nar) as u32);
        *goal_crowd_reaction(nar) = 0xff;
    }
    if *goal_crowd_reaction(nar) == 0xff {
        let p = caminho(0x9c382c, 0xa159a0);
        tocar(SOM, goal_crowd_reaction(nar) as usize, 2, p, 1);
    }
    let b = |o: usize| *((ev + o) as *const u8);
    let k = ((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16;
    let e = (TABELA + k as isize * 18) as usize;
    let qual = || *((ev + 2) as *const i16) as isize;
    let sons = || core::ptr::read_unaligned((e + 8) as *const usize);
    let mut cor1 = [0u8; 4];
    let mut cor2 = [0u8; 4];

    let valido = (0..0x2a5).contains(&k) && *((e + 1) as *const u8) == 0;
    if valido {
        crate::comparacao::ramo("narracao no painel");
        if sons() == 0 {
            return 0;
        }
        if core::ptr::read_unaligned((sons() as isize + qual() * 0x110 + 0x108) as *const usize) == 0 {
            return 0;
        }
        // as cores do clube do lance (uniforme do lado)
        let lado = b(6);
        let mut clube_id = if lado != 0 { *((ctx + 4) as *const usize) } else { *(ctx as *const usize) };
        let kit = *((ctx as isize + lado as i8 as isize * 1573 + 0x152) as *const u8) as u32;
        cores_do_clube((*(G_CLUB_TABLE as *const usize)).wrapping_add(clube_id.wrapping_mul(0x245)), kit, cor1.as_mut_ptr(), cor2.as_mut_ptr());
        let mut s = core::ptr::read_unaligned((sons() as isize + qual() * 0x110 + 0x108) as *const usize);
        let v = b(0xd) as i8;
        if v > 0 {
            let mut resta = v;
            let mut c = *(s as *const u8);
            if c != 0 {
                loop {
                    if c == b'|' {
                        resta -= 1;
                        if resta <= 0 {
                            s += 1;
                            break;
                        }
                    }
                    c = *((s + 1) as *const u8);
                    s += 1;
                    if c == 0 {
                        break;
                    }
                }
            }
            if *(s as *const u8) == 0 {
                return 0;
            }
        }
        let t = |i: i16| *((s as isize + i as isize) as *const u8);
        let mut linha = [0u8; 0x800];
        let l = linha.as_mut_ptr() as usize;
        let dest = |o: i16| (l as isize + o as isize) as usize;
        let mut pos: i16 = 0;
        let mut out: i16 = 0;
        let prefixo = |pos: i16, p: &mut [u8; 0x100]| {
            p[0] = 0;
            if pos > 1 && (t(pos - 1) == b'}' || t(pos - 1) == b']') {
                let mut a = pos - 2;
                while a >= 0 && t(a) != b'{' && t(a) != b'[' {
                    a -= 1;
                }
                let j = a as i32;
                let fim = pos as i32 - j - 1;
                let mut c = 1i32;
                while c < fim {
                    p[(c - 1) as usize] = t((j + c) as i16);
                    c += 1;
                }
                p[(c - 1) as usize] = 0;
            }
        };
        let sufixo = |inicio: i16, x: &mut [u8; 0x100]| {
            x[0] = 0;
            let mut fe = inicio;
            while t(fe) != b'>' && t(fe) != 0 {
                fe += 1;
            }
            if t(fe + 1) == b'{' || t(fe + 1) == b'[' {
                let mut j = fe + 2;
                while t(j) != b'}' && t(j) != 0 {
                    x[(j - fe - 2) as usize] = t(j);
                    j += 1;
                }
                x[(j - fe - 2) as usize] = 0;
            }
        };
        let escrever = |out: i16, r: usize| {
            formatar(dest(out), 0x991944, r);
        };
        let em_maiusculas = |out: i16, r: usize| {
            let mut j: i16 = 0;
            while *((r as isize + j as isize) as *const u8) != 0 {
                let c = maiuscula(*((r as isize + j as isize) as *const u8) as i32);
                *(dest(out + j) as *mut u8) = c;
                j += 1;
            }
            *(dest(out + j) as *mut u8) = 0;
        };
        let mut pre = [0u8; 0x100];
        let mut suf = [0u8; 0x100];
        let mut nome = [0u8; 0xd0];
        let mut inteiro = [0u8; 0x800];
        let mut arb = [0u8; 0xd0];
        let mut texto_fase = [0u8; 0x800];
        let cores_padrao = |c1: &mut [u8; 4], c2: &mut [u8; 4], segunda: usize| {
            core::ptr::write_unaligned(c1.as_mut_ptr() as *mut u16, *(G_GUI_COLOUR_BLACK as *const u16));
            core::ptr::write_unaligned(c2.as_mut_ptr() as *mut u16, *(segunda as *const u16));
        };

        if t(0) != 0 {
            loop {
                let c = t(pos);
                if c == b'|' {
                    break;
                }
                if c != b'<' {
                    if c != b'{' && c != b'[' {
                        *(dest(out) as *mut u8) = c;
                        out += 1;
                    } else {
                        let mut j = pos;
                        let mut a = c;
                        loop {
                            if a == b']' || a == 0 {
                                break;
                            }
                            j += 1;
                            a = t(j);
                            if a == b'}' {
                                break;
                            }
                        }
                        pos = j;
                    }
                } else {
                    let x = t(pos + 1);
                    match x {
                        b'P' | b'p' => {
                            let (lado_j, idx) = if t(pos + 2) == b'1' && b(5) != 0xff { (b(5), b(6)) } else { (b(7), b(8)) };
                            if lado_j != 0xff {
                                // a largura do texto inteiro (ate o `|`) decide o nome curto
                                let mut n = 0usize;
                                while *((s + n) as *const u8) != 0 && *((s + n) as *const u8) != b'|' {
                                    inteiro[n] = *((s + n) as *const u8);
                                    n += 1;
                                }
                                inteiro[n] = 0;
                                let curto = (largura_do_texto(fonte, inteiro.as_ptr()) as i16) < 0x12c;
                                nome_do_jogador(ctx, lado_j as i8 as i32, idx as i8 as i32, nome.as_mut_ptr(), 0xc8, curto as u32, modo, 1);
                                sufixo(pos + 3, &mut suf);
                                let r = compor(nome.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                                if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                    em_maiusculas(out, r);
                                } else {
                                    escrever(out, r);
                                }
                            }
                        }
                        b'T' | b't' => {
                            let dois = t(pos + 2) == b'2';
                            let casa = b(6) != 0;
                            let id = if dois == casa { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let clube = (*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(0x245));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let g = *((clube + club::SHORT_GENDER_NAME) as *const u8) as u32;
                            let r = compor(clube + club::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g);
                            if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                escrever(out, r);
                            }
                        }
                        b'C' | b'c' | b'F' | b'f' => {
                            if comp >= 0 {
                                let cp = (*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b));
                                prefixo(pos, &mut pre);
                                sufixo(pos + 2, &mut suf);
                                let r = if x == b'C' || x == b'c' {
                                    let g = *((cp + 0x52) as *const u8) as u32;
                                    compor(cp + 0x38, pre.as_ptr(), suf.as_ptr(), g, g)
                                } else {
                                    fase(cp, 0, 0x96, 3, texto_fase.as_mut_ptr(), 0x7d0);
                                    let g = criterio(0, 0, 0x96, 3) as u32;
                                    compor(texto_fase.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                                };
                                escrever(out, r);
                            }
                        }
                        b'D' | b'd' | b'N' => {
                            let numero = core::ptr::read_unaligned((ev + 9) as *const i32);
                            let v = if x != b'N' && t(pos + 2) == b'1' { numero.wrapping_add(1) } else { numero };
                            formatar(dest(out), 0x98806c, v);
                        }
                        b'n' => {
                            let r = por_extenso(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                            escrever(out, r);
                        }
                        b'O' | b'o' => {
                            // as cores do outro clube
                            let lado = b(6);
                            clube_id = if lado != 0 { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let a = (lado == 0) as isize;
                            let kit = *((ctx as isize + a * 1573 + 0x152) as *const u8) as u32;
                            cores_do_clube((*(G_CLUB_TABLE as *const usize)).wrapping_add(clube_id.wrapping_mul(0x245)), kit, cor1.as_mut_ptr(), cor2.as_mut_ptr());
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'Y' | b'y' => {
                            cores_padrao(&mut cor1, &mut cor2, 0xaebdec);
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'R' | b'r' => {
                            if t(pos + 2) != b'f' {
                                cores_padrao(&mut cor1, &mut cor2, 0xae31a8);
                                *(dest(out) as *mut u8) = 0;
                            } else {
                                let fixo = if x == b'R' { 0xa15a14usize } else { 0xa159e0 };
                                let arbitro = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                                if arbitro <= -1 {
                                    formatar_texto(dest(out), fixo);
                                } else {
                                    let soma = (b(7) as i8 as i32)
                                        .wrapping_add(b(5) as i8 as i32)
                                        .wrapping_add(core::ptr::read_unaligned((ctx + 4) as *const i32))
                                        .wrapping_add(clube_id as i32);
                                    let resto = (soma % 5) as i8;
                                    let ficha = (*(G_OFFICIAL_TABLE as *const usize)).wrapping_add((arbitro as usize).wrapping_mul(0x2b));
                                    let modo_nome = if idioma() == 4 && resto == 0 {
                                        1
                                    } else if resto == 0 {
                                        5
                                    } else if resto > 0 && resto <= 2 {
                                        1
                                    } else {
                                        0
                                    };
                                    match modo_nome {
                                        1 => {
                                            nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 1);
                                            formatar_texto(0xb67dd8, 0xa15a20, arb.as_ptr());
                                        }
                                        5 => {
                                            nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 5);
                                            formatar_texto(0xb67dd8, 0xa159ec, arb.as_ptr());
                                        }
                                        _ => {
                                            formatar_texto(0xb67dd8, fixo);
                                        }
                                    }
                                    formatar(dest(out), 0xb67dd8);
                                }
                            }
                        }
                        _ => show_error_at(0xa159b0, 0x89d),
                    }
                    if t(pos) != b'>' {
                        let mut j = pos;
                        loop {
                            if t(j) == 0 {
                                break;
                            }
                            j += 1;
                            if t(j) == b'>' {
                                break;
                            }
                        }
                        pos = j;
                    }
                    if t(pos) == 0 {
                        pos -= 1;
                    }
                    let mut n = 0usize;
                    while linha[n] != 0 {
                        n += 1;
                    }
                    out = n as i16;
                }
                pos += 1;
                if t(pos) == 0 {
                    break;
                }
            }
        }
        *(dest(out) as *mut u8) = 0;
        // larga demais: quebra ou corta numa palavra com "..."
        if largura_do_texto(fonte, linha.as_ptr()) > largura - 0x10 {
            let mut n = 0usize;
            while linha[n] != 0 {
                n += 1;
            }
            let mut si = n as i16;
            if quebrar as u8 != 0 {
                quebrar_linha(dest(si / 5) as *mut u8, 0, 1);
            } else {
                loop {
                    si -= 1;
                    *(dest(si) as *mut u8) = 0;
                    if largura_do_texto(fonte, linha.as_ptr()) <= largura - 0x10 {
                        if si > 0 {
                            loop {
                                if *(dest(si - 1) as *const u8) == b' ' {
                                    if si > 0 && *(dest(si - 1) as *const u8) == b' ' {
                                        si -= 1;
                                        *(dest(si) as *mut u8) = 0;
                                    }
                                    break;
                                }
                                si -= 1;
                                *(dest(si) as *mut u8) = 0;
                                if si <= 0 {
                                    break;
                                }
                            }
                        }
                        break;
                    }
                    if si <= 0 {
                        break;
                    }
                }
                let mut n = 0usize;
                while linha[n] != 0 {
                    n += 1;
                }
                for (i, c) in b"...\0".iter().enumerate() {
                    linha[n + i] = *c;
                }
            }
        }
        cor_de_fundo(SOM, painel, core::ptr::read_unaligned(cor2.as_ptr() as *const u32));
        fonte_do_painel(SOM, painel, fonte);
        cor_do_texto(SOM, painel, core::ptr::read_unaligned(cor1.as_ptr() as *const u32));
        texto_do_painel(SOM, painel, linha.as_ptr());
        margem(SOM, painel, 0x30);
        mostrar(SOM, painel, if *((e + 3) as *const u8) != 0 { 0x1001 } else { 1 });
    }

    // o som do evento
    *(((nar as isize) + eu() + 0x10) as *mut u8) = 0xff;
    if b(0xd) as i8 <= 0 && *((nar + 0x2db0) as *const u8) >= 0x65 {
        let som = (sons() as isize + qual() * 0x110) as usize;
        if *((som + 4) as *const u8) != 0 {
            let canal = if *((e + 6) as *const u8) == 0 { 0xb6b722usize } else { ((nar as isize) + eu() + 0x10) as usize };
            *(G_MATCH_EVENTS_CURRENT_SLOT_PTR as *mut usize) = canal;
            if canal != 0 {
                *(canal as *mut u8) = 0xfe;
                let lado = match *((e + 5) as *const u8) {
                    0 => (b(6) != 0) as u32,
                    2 => (b(6) == 0) as u32,
                    _ => 2,
                };
                let p = caminho(0x9c382c, som + 4);
                tocar(SOM, *(G_MATCH_EVENTS_CURRENT_SLOT_PTR as *const usize), lado, p, 0);
            }
        }
    }
    let agora = relogio();
    core::ptr::write_unaligned(((nar as isize) + eu() * 4 + 0x2d6c) as *mut u32, agora);
    let peso = *((sons() as isize + qual() * 0x110 + 2) as *const i16) as i32;
    let volume = *((nar + 0x2db0) as *const u8) as i32;
    core::ptr::write_unaligned((nar + 0x2dac) as *mut i32, (peso.wrapping_mul(volume) / 100) as i16 as i32);
    0
}

// ponytail: desenha no painel e toca som; sem comparacao propria (so roda com a tela da partida).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_commentate_on_panel(nar: usize, ev: usize, ctx: usize, comp: i32, painel: u32, largura: i32, quebrar: u32, fonte: usize, modo: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, i32, u32, usize, u32) -> u32>(IDX_COMMENTATE_ON_PANEL)(nar, ev, ctx, comp, painel, largura, quebrar, fonte, modo)
}

// ---------- 172a leva: o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista ----------

/// Advances the commentary queue read position (+0xc, ring of 12), marks the slot and switches the queue off on
/// reaching write/saved position; returns whether it did.
///
/// [pt] Avança a posição de leitura da fila da narração (+0xc, anel de 12): marca a nova vaga (+0x3d =
/// −1, tira o bit 0x8000 do evento) e, chegando na de escrita (+8, e então +0x2d68 = −1) ou na
/// guardada (+0xa), desliga a fila (+5 = 0). Devolve se desligou.
pub(crate) unsafe fn advance_commentary_read(t: usize) -> bool {
    let c = (t + 0xc) as *mut i16;
    *c = if c.read().wrapping_add(1) > 0xb { 0 } else { c.read().wrapping_add(1) };
    let p = t + 0x30 + *c as usize * 14;
    *((p + 0xd) as *mut u8) = 0xff;
    *(p as *mut u16) &= 0x7fff;
    let mut desligou = false;
    if *c == *((t + 8) as *const i16) {
        *((t + 0x2d68) as *mut u32) = u32::MAX;
        *((t + 5) as *mut u8) = 0;
        desligou = true;
    }
    if *c == *((t + 0xa) as *const i16) {
        *((t + 5) as *mut u8) = 0;
        desligou = true;
    }
    desligou
}

/// the next commentary event to show, part by part of the text (split by |), advancing the queue reading
/// Original: `next_event` (0x6d78b0, match_events.cpp, thiscall).
///
/// [pt] { 0x6d78b0 } (thiscall, ret 4, match_events.cpp): o próximo evento da narração a mostrar (0 com a
/// fila desligada, +5): pula as vagas vazias; no texto do evento (tabela [0xb68790 + 0x12 × (evento
/// − 8000)], registro +2 de 0x110 bytes, +0x108) anda até a próxima parte separada por `|` (a parte
/// atual em +0xd; o bit 0x8000 marca que há mais) e, acabado o texto, avança a leitura. Em `saida`,
/// o registro de 0x86 bytes da vaga (+0xd8).
pub(crate) unsafe extern "thiscall" fn next_event(t: usize, saida: *mut usize) -> usize {
    if *((t + 5) as *const u8) == 0 {
        return 0;
    }
    crate::comparacao::ramo("narracao proximo evento");
    if !saida.is_null() {
        *saida = (t as isize + 0xd8 + *((t + 0xc) as *const i16) as isize * 0x86) as usize;
    }
    let vaga = |t: usize| t + 0x30 + *((t + 0xc) as *const i16) as usize * 14;
    let mut p = vaga(t);
    while *(p as *const u16) == 0 {
        if advance_commentary_read(t) {
            return 0;
        }
        p = vaga(t);
    }
    let evento = (*(p as *const u16) & 0x7fff).wrapping_add(0xe0c0) as i16 as isize;
    let registro = *(p as *const i16).add(1) as isize;
    let tabela = *((0xb68790 + evento * 0x12) as *const isize);
    let texto = *((tabela + registro * 0x110 + 0x108) as *const *const u8);
    let n = {
        let mut k = 0usize;
        while *texto.add(k) != 0 {
            k += 1;
        }
        k as i16
    };
    let parte = (p + 0xd) as *mut i8;
    let mut c = *parte;
    let mut s = 0i16;
    while s < n {
        if *texto.offset(s as isize) == b'|' {
            if c == -1 {
                *parte = 0;
                break;
            }
            c = c.wrapping_sub(1);
            if c < 0 {
                s += 1;
                crate::comparacao::ramo("narracao texto em partes");
                *(p as *mut u16) |= 0x8000;
                *parte = (*parte).wrapping_add(1);
                break;
            }
        }
        s += 1;
    }
    if *parte != -1 {
        while s < n && *texto.offset(s as isize) != b'|' {
            s += 1;
        }
    }
    if n <= s {
        advance_commentary_read(t);
    }
    p
}

/// the player name in the commentary: the short form, or the long one when another player of either club has the same
/// short name
/// Original: `commentary_player_name` (0x6db2d0, match_events.cpp, cdecl).
///
/// [pt] { 0x6db2d0 } (cdecl, match_events.cpp): escreve em `buf` (`tam` bytes) o nome do jogador `jogador`
/// do lado `lado` da partida `pt` (registro de 0x49 bytes em +0x190: id e os três nomes +0x198..+0x1a0,
/// lados de 0x625 bytes) pela 0x53e140: com `curto` e sorteio(5) = 0 a forma 1; senão a forma 5, e se
/// algum dos 100 jogadores de um dos dois clubes (+0, +4; lista da 0x541f90) com outro id ou outro
/// nome tem a mesma forma 5, a forma `longa`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn commentary_player_name(pt: usize, jogador: i8, lado: i8, buf: *mut u8, tam: u32, curto: u8, _x: u32, longa: u32) {
    let nome: unsafe extern "C" fn(u32, u32, u32, *mut u8, u32, u32, u32) = core::mem::transmute(FN_DATABASE_FORMAT_NAME_FROM_IDS);
    let r = (pt as isize + jogador as isize * 0x49 + lado as isize * 0x625) as usize;
    let d = |o: usize| *((r + o) as *const u32);
    if curto != 0 && sortear(5) == 0 {
        nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, 1, d(0x190));
        return;
    }
    crate::comparacao::ramo("narracao nome curto");
    nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, 5, d(0x190));
    let lista: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let textos: unsafe extern "C" fn(*const u8, *const u8, i32, u32) -> i32 = core::mem::transmute(FN_UTILS_STRING_COMPARE);
    for k in 0..2 {
        let clube = *((pt + 4 * k) as *const u32) as usize;
        let mut l = [0usize; 100];
        lista(*(G_CLUB_TABLE as *const usize) + clube * 0x245, l.as_mut_ptr(), 1);
        for &q in l.iter() {
            if q == 0 {
                continue;
            }
            let n = |i: usize| *((*((q + 4 * i) as *const usize) + 0x33) as *const u32);
            if *(q as *const u32) == d(0x190) && n(1) == d(0x198) && n(2) == d(0x19c) && n(3) == d(0x1a0) {
                continue;
            }
            let mut outro = [0u8; 200];
            nome(n(1), n(2), n(3), outro.as_mut_ptr(), 200, 5, d(0x190));
            if textos(buf, outro.as_ptr(), 1, 0) == 0 {
                crate::comparacao::ramo("narracao nome forma longa");
                nome(d(0x198), d(0x19c), d(0x1a0), buf, tam, longa, d(0x190));
                return;
            }
        }
    }
}



pub(crate) unsafe extern "thiscall" fn comparar_next_event(t: usize, saida: *mut usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize) -> usize>(IDX_NEXT_EVENT);
    let mut reg = vec![(t, 0x2db4)];
    if !saida.is_null() {
        reg.push((saida as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, saida), || next_event(t, saida));
    PLACAR.registrar(igual && r == n, || format!("next_event({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", t, r, n, mo, mr));
    r
}

// ponytail: escreve em `buf` e chama a 0x53e140 e a 0x541f90 (efeitos fora); comparação pelo texto
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn comparar_commentary_player_name(pt: usize, jogador: i8, lado: i8, buf: *mut u8, tam: u32, curto: u8, x: u32, longa: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i8, i8, *mut u8, u32, u8, u32, u32)>(IDX_COMMENTARY_PLAYER_NAME);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((buf as usize, tam as usize));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pt, jogador, lado, buf, tam, curto, x, longa), || crate::aleatorio::sem_contar(|| commentary_player_name(pt, jogador, lado, buf, tam, curto, x, longa)));
    PLACAR.registrar(igual, || format!("commentary_player_name({}, {}) {:x?} x {:x?}", jogador, lado, mo, mr));
}

// ---------- 171a leva: o fechamento do motor (5): a narração combinada e o comentário de contexto ----------

/// Os rótulos de desvio da 0x6d60b0 (o fim de cada um devolve ou segue para outro).
#[derive(Clone, Copy)]
pub(crate) enum Rotulo {
    R6cde,
    R6d1d,
    R69a6,
    R69f5,
    R6c95,
    R6cbf,
    Copiar,
}

/// combines the new commentary event with the last queued one (returns the combined event and copies the previous
/// player, side and value)
/// Original: `combine_with_previous` (0x6d60b0, match_events.cpp, thiscall).
///
/// [pt] { 0x6d60b0 } (thiscall, ret 8, match_events.cpp): combina o evento novo `e` (14 bytes: palavra do
/// evento; +5, +6 jogador e lado; +7, +8 o outro; +9 inteiro) com o último da fila (posição +8 − 1
/// no anel de 12, registros de 14 bytes em +0x30): devolve o evento combinado (negativo = troca o
/// anterior; 0 = sem combinação) e, nas combinações, copia o jogador, o lado, o outro e o inteiro do
/// anterior para `e`. `m` é o motor (0 fora da partida): dá o lado da posse (+0x8eab) e a bola na
/// área (0x6d58f0). As duas chamadas da 0x6cb960 do original não têm efeito e o valor delas é
/// descartado (ficam de fora).
pub(crate) unsafe extern "thiscall" fn combine_with_previous(t: usize, e: usize, m: usize) -> i16 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let b = |p: usize, o: usize| *((p + o) as *const i8);
    let w = |p: usize| core::ptr::read_unaligned(p as *const i16);
    let s2 = w(e);
    let (posse, area) = if m == 0 { (0, 0) } else { (b(m, 0x8eab), ball_goal_end(m)) };
    if w(t + 8) == w(t + 6) {
        return 0;
    }
    let k = if w(t + 8) == 0 { 0xb } else { b(t, 8).wrapping_sub(1) };
    let q = (t as isize + 0x30 + k as isize * 14) as usize;
    let s3 = w(q);
    let igual = |o: usize| b(q, o) == b(e, o);
    let copiar = || core::ptr::copy((q as *const u8).add(5), (e as *mut u8).add(5), 8);
    // o anterior passa a ser o principal e o novo vira o outro (+7, +8)
    let trocar = |inteiro: bool| {
        *((e + 8) as *mut i8) = b(e, 6);
        *((e + 7) as *mut i8) = b(e, 5);
        *((e + 5) as *mut i8) = b(q, 5);
        *((e + 6) as *mut i8) = b(q, 6);
        if inteiro {
            core::ptr::write_unaligned((e + 9) as *mut u32, core::ptr::read_unaligned((q + 9) as *const u32));
        }
    };
    let contexto = |v: i16| (0x2021..=0x20d0).contains(&v);

    if s2 == 0x1f6f {
        if posse == b(e, 6) {
            return 0x21bb;
        }
    } else if s2 == 0x1f92 && (s3 == 0x1f6c || s3 == 0x1f6e) {
        return -0x1f9c;
    }
    if s3 == 0x1f7d && b(e, 6) != b(q, 6) && s2 == 0x1f46 {
        return -0x219d;
    }
    let shot = (0x1f87..=0x1fbd).contains(&s2);
    if (shot || (s2 == 0x1f5f && area != 0)) && s3 == 0x1f7e {
        core::ptr::write_unaligned(q as *mut i16, 0x219c);
        core::ptr::write_unaligned((q + 2) as *mut i16, sorteio(2) as i16);
        return 0;
    }
    if shot && s3 == 8000 {
        core::ptr::write_unaligned(q as *mut i16, 0x219b);
        return 0;
    }

    let mut r = 'escolha: {
        if s3 == 0x1f44 {
            if s2 == 0x1f43 {
                copiar();
                return 0x213f;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f69 {
            if s2 == 0x1f9d {
                return if sorteio(3) != 0 { 0 } else { 0x2198 };
            }
            break 'escolha Rotulo::R6cde;
        }
        if contexto(s3) && matches!(s2, 0x1f46 | 0x213e | 0x1f60 | 0x1f5f) && b(e, 6) != b(q, 6) {
            let lado = b(q, 6);
            if (area == -1 && lado == 1) || (area == 1 && lado == 0) {
                return -0x2178;
            }
        }
        if s3 == 0x1f6f {
            if s2 == 0x1f46 {
                if igual(6) || area == 0 {
                    return 0;
                }
                trocar(false);
                return 0x2177;
            }
            break 'escolha Rotulo::R6cde;
        }
        for (de, para) in [(0x2111, 0x2140), (0x210f, 0x2141), (0x1fe6, 0x2142), (0x1fe7, 0x2142)] {
            if s3 == de {
                if s2 == 0x1f46 {
                    if !igual(5) || !igual(6) {
                        return 0;
                    }
                    copiar();
                    return para;
                }
                break 'escolha Rotulo::R6cde;
            }
        }
        if (s3 == 0x213e || s3 == 0x1f71) && s2 == 0x1f46 && igual(5) && igual(6) {
            copiar();
            return 0x1f46;
        }
        if s3 == 0x213e {
            if s2 == 0x1f41 {
                copiar();
                return 0x213e;
            }
            break 'escolha Rotulo::R69a6;
        }
        if s3 == 0x1f41 {
            if s2 == 0x1f46 {
                copiar();
                return 0x1f46;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x2113 {
            if (0x1f86..=0x1fbe).contains(&s2) && sorteio(3) == 0 && igual(5) && igual(6) {
                copiar();
                return s2;
            }
            break 'escolha Rotulo::R6c95;
        }
        if s3 == 0x1f47 {
            if s2 == 0x1f44 || s2 == 0x1f43 {
                if !igual(5) || !igual(6) {
                    return 0;
                }
                copiar();
                return s2;
            }
            if (s2 == 0x1f45 || s2 == 0x20de) && igual(5) && igual(6) {
                copiar();
                return s2;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f4d {
            if s2 == 0x1f4a {
                if sorteio(2) == 0 || !igual(5) {
                    return 0;
                }
                if (!igual(6) || !igual(7) || !igual(8)) && (!igual(6) || (b(q, 7) != -1 && b(e, 7) != -1)) {
                    return 0;
                }
                copiar();
                return 0x1f4a;
            }
            break 'escolha Rotulo::R6cde;
        }
        if s3 == 0x1f43 {
            if s2 == 0x1f9d {
                if sorteio(2) == 0 {
                    return 0;
                }
                copiar();
                return 0x1f43;
            }
            break 'escolha Rotulo::R6cde;
        }
        if matches!(s3, 0x1f62 | 0x1f63 | 0x2150) && s2 == 0x1f46 && igual(5) && igual(6) {
            break 'escolha Rotulo::Copiar;
        }
        if s3 == 0x1f5d {
            if s2 == 0x1f6f {
                copiar();
                return 0x2143;
            }
            break 'escolha Rotulo::R6cde;
        }
        if contexto(s3) && s2 == 0x1f46 && b(q, 7) == b(e, 5) && b(q, 8) == b(e, 6) {
            break 'escolha Rotulo::Copiar;
        }
        match s3 {
            0x1f60 if s2 == 0x1f6f => {
                copiar();
                return 0x1f6f;
            }
            0x1f46 if s2 == 0x1f70 => {
                copiar();
                return 0x1f70;
            }
            0x1f84 | 0x1f85 if s2 == 0x1f75 => {
                if sorteio(3) == 0 {
                    return 0;
                }
                copiar();
                return if s3 == 0x1f84 { 0x2144 } else { 0x2145 };
            }
            0x1f60 | 0x1f46 | 0x1f84 | 0x1f85 => {}
            0x1f80 => {
                if s2 == 0x1f46 {
                    if sorteio(2) != 0 && !igual(6) {
                        trocar(true);
                        return 0x2146;
                    }
                    break 'escolha Rotulo::Copiar;
                }
            }
            0x1f5e => break 'escolha Rotulo::R69a6,
            _ => break 'escolha Rotulo::R69f5,
        }
        Rotulo::R6cde
    };

    loop {
        r = match r {
            Rotulo::R69a6 => {
                if s2 == 0x1f46 && sorteio(5) != 0 && igual(6) && !igual(5) {
                    trocar(true);
                    return 0x2147;
                }
                Rotulo::R69f5
            }
            Rotulo::R69f5 => 'r: {
                for (de, para) in [(0x2148, 0x2149), (0x214a, 0x214b), (0x214c, 0x214d)] {
                    if s3 == de {
                        if s2 == 0x1f60 || s2 == 0x1f5f {
                            copiar();
                            return para;
                        }
                        break 'r Rotulo::R6cde;
                    }
                }
                if contexto(s3) {
                    let (ie, iq) = (core::ptr::read_unaligned((e + 9) as *const i32), core::ptr::read_unaligned((q + 9) as *const i32));
                    if contexto(s2) && (1..0x15).contains(&ie) && (1..0x15).contains(&iq) && b(e, 5) == b(q, 7) && b(e, 7) == b(q, 5) {
                        copiar();
                        return 0x214e;
                    }
                    if s2 == 0x1f47 && (1..0x15).contains(&iq) && sorteio(0x16 - iq) != 0 && b(q, 7) == b(e, 5) && b(q, 8) == b(e, 6) {
                        break 'r Rotulo::Copiar;
                    }
                }
                if s3 == 0x214f {
                    if s2 == 0x1f92 {
                        copiar();
                        return 0x214f;
                    }
                    break 'r Rotulo::R6cde;
                }
                if matches!(s3, 0x213e | 0x1f71 | 0x1f72) && s2 == 0x1f75 {
                    copiar();
                    return 0x2151;
                }
                if s3 == 8000 {
                    if s2 == 0x1f9d {
                        copiar();
                        return 0x1f9d;
                    }
                    break 'r Rotulo::R6cde;
                }
                if s3 != 0x2021 {
                    if s3 == 0x1f55 {
                        if s2 == 8000 {
                            return -0x2152;
                        }
                    } else if s3 == 0x2113 {
                        break 'r Rotulo::R6c95;
                    } else if contexto(s3) {
                        break 'r Rotulo::R6cbf;
                    }
                    break 'r Rotulo::R6cde;
                }
                if matches!(s2, 0x1f62 | 0x1f63 | 0x2150) {
                    return -0x2153;
                }
                Rotulo::R6cbf
            }
            Rotulo::R6cbf => {
                if s2 != 0x1f70 {
                    Rotulo::R6cde
                } else if igual(6) {
                    return -0x2157;
                } else {
                    Rotulo::R6d1d
                }
            }
            Rotulo::R6c95 => {
                if s2 == 0x1f62 || s2 == 0x2150 {
                    return -0x2153;
                }
                Rotulo::R6cde
            }
            Rotulo::R6cde => {
                if (0x2076..=0x20d0).contains(&s2) && s3 == 0x2113 {
                    return if igual(5) && igual(6) { -0x2159 } else { 0 };
                }
                Rotulo::R6d1d
            }
            Rotulo::R6d1d => {
                const GRUPO: [i16; 8] = [0x1f4b, 0x1f4c, 0x1f4e, 0x1f4f, 0x1f50, 0x1f51, 0x1f52, 0x1f53];
                if (0x215a..=0x215f).contains(&s3) && s2 == 0x1f70 {
                    return -0x2157;
                }
                if s3 == 0x1f70 {
                    if s2 == 0x1f76 && !igual(6) {
                        copiar();
                        return 0x2165;
                    }
                    return 0;
                }
                if s3 == 0x2150 {
                    if s2 != 0x1f75 {
                        return 0;
                    }
                    copiar();
                    return 0x2151;
                }
                if GRUPO.contains(&s3) && GRUPO.contains(&s2) && sorteio(2) != 0 {
                    return -0x2169;
                }
                if s3 == 0x216e {
                    return if s2 == 0x1f44 && igual(5) && igual(6) { 0x216e } else { 0 };
                }
                if matches!(s3, 0x1f80 | 0x2131 | 0x2130) && s2 == 0x1f46 {
                    Rotulo::Copiar
                } else if s3 != 0x1f73 && !(0x20b9..=0x20c4).contains(&s3) && s3 != 0x20d0 && s3 != 0x1f80 {
                    return 0;
                } else if s2 != 0x1f46 && s2 != 0x1f47 {
                    return 0;
                } else {
                    Rotulo::Copiar
                }
            }
            Rotulo::Copiar => {
                copiar();
                return s3;
            }
        }
    }
}


pub(crate) unsafe extern "thiscall" fn comparar_combine_with_previous(t: usize, e: usize, m: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i16>(IDX_COMBINE_WITH_PREVIOUS);
    if is_recording_top_level() {
        return o(t, e, m);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(e, 14), (t + 0x30, 12 * 14)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, e, m), || crate::aleatorio::sem_contar(|| combine_with_previous(t, e, m)));
    PLACAR.registrar(igual && r == n, || format!("combine_with_previous({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", core::ptr::read_unaligned(e as *const u16), r, n, mo, mr));
    r
}


/// Neighbourhood probe of the context commentary: compares all 13 comments both sides with ratings at each threshold
/// and fake news, restoring random state and ratings.
///
/// [pt] Sonda de vizinhança do comentário de contexto: o jogo só pergunta pelo evento do gol, então cada
/// chamada real compara também os 13 comentários (0x21a3..=0x21af) nos dois lados e, com as notas
/// dos 20 jogadores do lado (+0x47cb) postas exatamente em cada limite de média (7501, 7001, 8000,
/// 7750, 6699), as fronteiras de `>=`/`>`; e, com uma mídia falsa (uma notícia de hoje de cada tipo
/// para os dois clubes) e o minuto 31, os caminhos que dependem das notícias. O estado do sorteador e as notas voltam ao que eram e os
/// sorteios não contam nem entram no rastro de uma comparação pela fita em curso (o chute, 0x702050,
/// chama esta rotina por dentro): a trajetória do jogo comparando não muda.
pub(crate) unsafe fn probe_context_commentary(m: usize, o: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32) -> u32) {
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    // a mídia falsa: no registro de notícias (0x757da0 busca por sujeito -1, id do clube, tipo -1 e
    // subtipo, registros de 0x17 bytes em ordem), uma notícia de hoje de cada subtipo que os
    // comentários consultam, para os dois clubes
    let c = *((m + 0x4792) as *const usize);
    let hoje = *(G_CURRENT_DATE as *const [u8; 8]);
    let mut registros: Vec<[u8; 0x17]> = Vec::new();
    for o_clube in [0x1c, 0x20] {
        let clube = *((c + o_clube) as *const usize);
        if clube == 0 {
            continue;
        }
        let id = *(clube as *const i32);
        for sub in [0x3fi16, 0x40, 0x41, 0x5c, 0x5f, 0x60, 0x61, 0x62, 99] {
            let mut r = [0u8; 0x17];
            r[..8].copy_from_slice(&hoje);
            r[8..0xc].copy_from_slice(&(-1i32).to_le_bytes());
            r[0xc..0x10].copy_from_slice(&id.to_le_bytes());
            r[0x11..0x13].copy_from_slice(&sub.to_le_bytes());
            r[0x13..0x17].copy_from_slice(&(-1i32).to_le_bytes());
            registros.push(r);
        }
    }
    let chave = |r: &[u8; 0x17]| (i32::from_le_bytes(r[0xc..0x10].try_into().unwrap()), i16::from_le_bytes(r[0x11..0x13].try_into().unwrap()));
    registros.sort_by_key(chave);
    registros.dedup_by_key(|r| chave(r));
    let falsa: [usize; 3] = [registros.as_ptr() as usize, 0, registros.len()];
    let midia = *(G_MEDIA as *const usize);
    let minuto = core::ptr::read_unaligned((m + 0x8ed0) as *const i16);
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| {
        // 0: o estado real; 1: com as notícias; 2: com as notícias no minuto 31
        for cenario in 0..3 {
            *(G_MEDIA as *mut usize) = if cenario == 0 { midia } else { falsa.as_ptr() as usize };
            core::ptr::write_unaligned((m + 0x8ed0) as *mut i16, if cenario == 2 { 31 * 11 } else { minuto });
            for lado in 0..2u32 {
                let notas: Vec<(usize, usize)> = (0..20).map(|k| (m + lado as usize * 0x22d8 + 0x47cb + k * 0x1be, 2)).collect();
                let guardadas = crate::comparacao::Memoria::ler(&notas);
                for limit in [None, Some(0x1d4di16), Some(0x1b59), Some(8000), Some(0x1e46), Some(0x1a2b)] {
                    if let Some(v) = limit {
                        for &(a, _) in &notas {
                            core::ptr::write_unaligned(a as *mut i16, v);
                        }
                    }
                    for evento in 0x21a3..=0x21afu32 {
                        let mut saida = 0i32;
                        let ps = &mut saida as *mut i32;
                        let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
                        reg.push((ps as usize, 4));
                        let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, lado, evento, ps), || context_commentary(m, lado, evento, ps));
                        PLACAR.registrar(igual && r == n, || format!("sonda context_commentary(cenario {}, {}, {:#x}, media {:?}) original={} rust={} {:x?} x {:x?}", cenario, lado, evento, limit, r, n, mo, mr));
                    }
                }
                guardadas.escrever();
            }
        }
        *(G_MEDIA as *mut usize) = midia;
        core::ptr::write_unaligned((m + 0x8ed0) as *mut i16, minuto);
    }));
    estado.escrever();
}



// ---------- 169a leva: o fechamento do motor (3): a fila da narracao (match_events.cpp) ----------

/// puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via
/// 0x6d6f70
/// Original: `event_at_front` (0x6d5d00, match_events.cpp, thiscall).
///
/// [pt] { 0x6d5d00 } (thiscall, ret 0x1c, match_events.cpp): põe um evento na frente da fila da narração:
/// a posição de escrita (+8, anel de 12) recua uma casa (de 0 vai a 11); se o último da fila (+6
/// itens, palavra +0x712 de 14 bytes) é o mesmo evento da nova posição (+0x30), a fila perde um;
/// depois enfileira pela 0x6d6f70 com os mesmos 7 argumentos.
pub(crate) unsafe extern "thiscall" fn event_at_front(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    let w = |o: usize| core::ptr::read_unaligned((t + o) as *const i16);
    let pos = w(8);
    let pos = if pos == 0 { 0xb } else { pos.wrapping_sub(1) };
    core::ptr::write_unaligned((t + 8) as *mut i16, pos);
    let n = w(6);
    if n > 0 && w(0x712 + n as usize * 14) == w(0x30 + pos as usize * 14) {
        core::ptr::write_unaligned((t + 6) as *mut i16, n - 1);
    }
    let enfileirar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_EVENTS_ADD_EVENT);
    enfileirar(t, a, b, c, d, e, f, g)
}

/// the commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop
/// Original: `construct_commentary_queue` (0x6d5bf0, match_events.cpp, thiscall).
///
/// [pt] { 0x6d5bf0 } (thiscall, ret 4, match_events.cpp): o construtor da fila da narração: zera a fila e
/// os campos (+0x2db0 = 125, +1 e +0xa = −1, +0x2d68 = −1), os globais de evento (0xb68670..), lê os
/// eventos (0x6d9e50), marca as 16 vagas (+0x10, +0x20) com −1 e zera os 16 tempos (+0x2d6c); com
/// `torcida`, toca o loop da torcida do lado da vez ([0xb77c7a]) pela 0x8025e0. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn construct_commentary_queue(t: usize, goal_crowd_reaction: u8) -> usize {
    let b = |o: usize, v: u8| *((t + o) as *mut u8) = v;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((t + o) as *mut u16, v);
    w(0xa, 0xffff);
    core::ptr::write_unaligned((t + 0x2d68) as *mut u32, u32::MAX);
    for o in [6, 8, 0xc, 0xe] {
        w(o, 0);
    }
    b(5, 0);
    b(2, 0);
    b(0, 1);
    b(0x2db0, 0x7d);
    b(1, 0xff);
    b(3, 0);
    core::ptr::write_unaligned((t + 0x2dac) as *mut u32, 0);
    b(4, 0);
    *(G_MATCH_EVENT_DEFAULT_VARIANT as *mut u16) = 1;
    *(G_MATCH_EVENT_DEFAULT_VARIANT_LINE_DELAY as *mut u16) = 0x2bc;
    *(G_MATCH_EVENT_DEFAULT_VARIANT_TEXT as *mut u8) = 0;
    *(0xb68778 as *mut u32) = 0xad9c64;
    *(0xb6877c as *mut u32) = 0xad9c64;
    let read_events_cfg: unsafe extern "C" fn() = core::mem::transmute(FN_MATCH_EVENTS_READ_CONFIG);
    read_events_cfg();
    for k in 0..16 {
        b(0x10 + k, 0xff);
        b(0x20 + k, 0xff);
        core::ptr::write_unaligned((t + 0x2d6c + 4 * k) as *mut u32, 0);
    }
    if goal_crowd_reaction != 0 {
        let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
        let tocar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(FN_SCREENMANAGER_PLAY_SOUND);
        let lado = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        let nome = caminho(0x9c382c, 0xa159a0);
        tocar(0xb74450, (t as isize + 0x20 + lado) as usize, 2, nome, 1);
    }
    t
}

/// frees the loaded event tables and each record's texts
/// Original: `free_event_tables` (0x6d5b50, match_events.cpp, cdecl).
///
/// [pt] { 0x6d5b50 } (cdecl, match_events.cpp): libera as tabelas de eventos carregadas: em cada uma das
/// entradas de 0x12 bytes de 0xb68790 a 0xb6b72a com tabela própria (nem nula, nem a padrão
/// 0xb68670), os dois textos (+0x108, +0x10c) de cada registro de 0x110 bytes e a tabela; depois
/// zera 0xb6b723 e põe −1 em 0xa1599c.
pub(crate) unsafe extern "C" fn free_event_tables() {
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut e = 0xb68790usize;
    while e < 0xb6b72a {
        let tabela = core::ptr::read_unaligned(e as *const usize);
        if tabela != 0xb68670 && tabela != 0 {
            let mut k = 0i32;
            while k < core::ptr::read_unaligned((e + 4) as *const i16) as i32 {
                let r = tabela + k as usize * 0x110;
                for o in [0x108, 0x10c] {
                    let p = core::ptr::read_unaligned((r + o) as *const usize);
                    if p != 0 {
                        liberar(p);
                    }
                }
                k += 1;
            }
            liberar(tabela);
            core::ptr::write_unaligned(e as *mut usize, 0);
        }
        e += 0x12;
    }
    *(G_MATCH_EVENT_CONFIG_LOADED_FLAG as *mut u8) = 0;
    *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *mut u32) = u32::MAX;
}

// ponytail: as tres enfileiram narracao, tocam som ou liberam memoria (nao rodam duas vezes): sem
// comparacao propria, provadas pela carreira (fila de 19414 partidas, 285 mil eventos na frente).
pub(crate) unsafe extern "thiscall" fn comparar_event_at_front(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(IDX_EVENT_AT_FRONT)(t, a, b, c, d, e, f, g)
}

pub(crate) unsafe extern "thiscall" fn comparar_construct_commentary_queue(t: usize, goal_crowd_reaction: u8) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u8) -> usize>(IDX_CONSTRUCT_COMMENTARY_QUEUE)(t, goal_crowd_reaction)
}

pub(crate) unsafe extern "C" fn comparar_free_event_tables() {
    original_fn::<unsafe extern "C" fn()>(IDX_FREE_EVENT_TABLES)()
}

/// have the sounds of the 16 marked events played and passed their window (clock 0x90c510; stops the finished channels,
/// 0x802a50)?
/// Original: `sounds_finished` (0x6d8c70, match_events.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d8c70 } (thiscall, ret 4, match_events.cpp): os sons dos eventos `ativos` (16 marcas) ja
/// tocaram? Para cada marcado, o canal (+0x10 + k; -1 livre, -2 espera) que ainda toca (0x8029f0 em
/// 0xb74450) segura; o que acabou e parado (0x802a50) e liberado. Devolve 1 so se todos os marcados
/// ja passaram da janela (+0x2d6c + 4k ate + [+0x2dac], pelo relogio 0x90c510).
pub(crate) unsafe extern "thiscall" fn sounds_finished(e: usize, ativos: *const u32) -> u8 {
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_UTILS_GET_TIME_MS);
    let tocando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_IS_SOUND_PLAYING);
    let parar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_SCREENMANAGER_STOP_SOUND);
    let agora = relogio();
    crate::comparacao::ramo("sons dos eventos");
    for k in 0..16usize {
        if *ativos.add(k) == 0 {
            continue;
        }
        let ini = *((e + 0x2d6c + 4 * k) as *const u32);
        let fim = (*((e + 0x2dac) as *const u32)).wrapping_add(ini);
        let passou = agora > fim || agora < ini;
        let canal = (e + 0x10 + k) as *mut u8;
        if *canal != 0xff {
            if *canal == 0xfe {
                return 0;
            }
            if tocando(0xb74450, (ini & !0xff) | *canal as u32) != 0 {
                return 0;
            }
            parar(0xb74450, *canal as u32);
            *canal = 0xff;
        }
        if !passou {
            return 0;
        }
    }
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_sounds_finished(e: usize, ativos: *const u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32) -> u8>(IDX_SOUNDS_FINISHED)(e, ativos)
}

/// swaps the commentary event for a variant, by a draw and by the play type
/// Original: `event_variant` (0x6d5d80, match_events.cpp, stdcall).
///
/// [pt] { 0x6d5d80 } (stdcall, ret 0xc, match_events.cpp): troca o evento da narração por uma variante:
/// com sorteio(3) ≠ 0 fica o mesmo; senão, pelo tipo do lance `tipo` (1, 2, 8, 0x10, 0x20 — este com
/// outro sorteio(3)) há tabelas de troca; os lances 2..4 (byte baixo de `lados`) têm as suas, e o
/// byte de cima 3..5 troca 0x1f4b por 0x217e.
pub(crate) unsafe extern "system" fn event_variant(e: *const i16, lados: u32, tipo: u8) -> i16 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    if sorteio(3) != 0 {
        return *e;
    }
    let v = *e;
    let tabela: &[(i16, i16)] = match tipo {
        1 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fee, 0x218d), (0x1f49, 0x2182), (0x205a, 0x2190), (0x205b, 0x2191), (0x2079, 0x2192)],
        2 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x1f48, 0x2181), (0x1f41, 0x2180), (0x1f44, 0x2187), (0x1f86, 0x2189), (0x1fa2, 0x218a)],
        8 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x205a, 0x218e), (0x205b, 0x218f), (0x1f4b, 0x2183), (0x1f41, 0x2184), (0x1f4a, 0x2188)],
        0x10 => &[(0x1f48, 0x2185)],
        0x20 if sorteio(3) == 0 => &[(0x216f, 0x2199), (0x1fee, 0x218d), (0x1f42, 0x217c), (0x1f4b, 0x217d), (0x1f48, 0x2186)],
        _ => &[],
    };
    if let Some(&(_, n)) = tabela.iter().find(|&&(de, _)| de == v) {
        return n;
    }
    if matches!(lados as u8, 2..=4) {
        let lance: &[(i16, i16)] = &[(0x2112, 0x2196), (0x1fa6, 0x2194), (0x1f8b, 0x218b), (0x1fbf, 0x218c), (0x211f, 0x217f)];
        if let Some(&(_, n)) = lance.iter().find(|&&(de, _)| de == v) {
            return n;
        }
    }
    if matches!((lados >> 8) as u8, 3..=5) && v == 0x1f4b {
        return 0x217e;
    }
    v
}

pub(crate) unsafe extern "system" fn comparar_event_variant(e: *const i16, lados: u32, tipo: u8) -> i16 {
    let o = original_fn::<unsafe extern "system" fn(*const i16, u32, u8) -> i16>(IDX_EVENT_VARIANT);
    if is_recording_top_level() {
        return o(e, lados, tipo);
    }
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(e, lados, tipo), || crate::aleatorio::sem_contar(|| event_variant(e, lados, tipo)));
    PLACAR.registrar(igual && r == n, || format!("event_variant({:#x}, {:#x}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", *e, lados, tipo, r, n, mo, mr));
    r
}
