use super::*;

/// The two match event file names in CM3_TEMP (human or computer).
///
/// [pt] Os dois arquivos de eventos (humano ou computador) em CM3_TEMP.
pub(crate) fn match_event_files(human: bool) -> (usize, usize) {
    if human { (0xa15a8c, 0xa15a78) } else { (0xa15a60, 0xa15a48) }
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
pub(crate) unsafe extern "thiscall" fn record_events(e: usize, desc: *mut u32, human: u8) -> u8 {
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = match_event_files(human != 0);
    let (mut versao, mut quantos, mut numero) = (0i32, 0i32, 0i32);
    let mut ids = [-1i32, -1];
    let mut aberto = 0usize;
    for (k, name) in [um, dois].into_iter().enumerate() {
        let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x9c7848);
        if f == 0 {
            continue;
        }
        exe::crt_fseek(f, 0, 0);
        exe::crt_fread((&mut versao) as *mut i32 as usize, 4, 1, f);
        exe::crt_fread((&mut ids[k]) as *mut i32 as usize, 4, 1, f);
        exe::crt_fread((&mut quantos) as *mut i32 as usize, 4, 1, f);
        numero = ids[k];
        if versao != 1 {
            show_error_at(0xa159b0, if k == 0 { 0xb5c } else { 0xb7b });
            exe::crt_fclose(f);
            return 0;
        }
        if quantos < 0x9c4 {
            crate::comparison::branch("events file with space");
            aberto = f;
            break;
        }
        exe::crt_fclose(f);
    }
    let f = if aberto != 0 {
        aberto
    } else {
        let name = if ids[0] == -1 || (ids[1] != -1 && ids[0] <= ids[1]) { um } else { dois };
        let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x9a7f28);
        if f == 0 {
            show_error_at(0xa159b0, 0xba4);
            return 0;
        }
        versao = 1;
        crate::comparison::branch("events new file");
        quantos = 0;
        let r = *(G_MATCH_STATS as *const usize) + 0x18;
        numero = *(r as *const i32) + 1;
        *(r as *mut i32) = numero;
        exe::crt_fwrite(&versao as *const i32 as usize, 4, 1, f);
        exe::crt_fwrite(&numero as *const i32 as usize, 4, 1, f);
        exe::crt_fwrite(&quantos as *const i32 as usize, 4, 1, f);
        f
    };
    exe::crt_fseek(f, 0, 2);
    *desc = (exe::crt_ftell(f) as u32);
    let n = *((e + 6) as *const i16) as i32;
    *desc.add(1) = n as u32;
    *desc.add(2) = numero as u32;
    *(desc.add(3) as *mut u8) = human;
    let r = exe::crt_fwrite(e + 0x720, 0xe, *((e + 6) as *const i16) as i32 as u32, f);
    quantos += 1;
    exe::crt_fseek(f, 8, 0);
    exe::crt_fwrite(&quantos as *const i32 as usize, 4, 1, f);
    exe::crt_fclose(f);
    (r != 0) as u8
}

/// Reads back the events described in `desc` from the file with the right number.
///
/// Ret 4: reads back the events described in `desc` (position, count, number, human) into +0x720 of record `e` (+6 =
/// count): looks for the file with the number (version 1, otherwise error 0xbf6/0xc11, with the file left open as in
/// the original). Returns whether it read them.
/// Original: `read_events` (0x6d9c40, match_events.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d9c40 } (thiscall, ret 4, match_events.cpp): le de volta os eventos descritos em `desc`
/// (posicao, quantos, numero, humano) para +0x720 do registro `e` (+6 = quantos): procura o arquivo
/// com o numero (versao 1, senao erro 0xbf6/0xc11, com o arquivo aberto como na original). Devolve
/// se leu.
pub(crate) unsafe extern "thiscall" fn read_events(e: usize, desc: *const u32) -> u8 {
    if desc.is_null() {
        return 0;
    }
    let (um, dois) = match_event_files(*(desc.add(3) as *const u8) != 0);
    let (mut versao, mut numero) = (0i32, 0i32);
    for (k, nome) in [um, dois].into_iter().enumerate() {
        let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, nome), 0x98f928);
        if f == 0 {
            if k == 0 {
                continue;
            }
            return 0;
        }
        exe::crt_fread(&mut versao as *mut i32 as usize, 4, 1, f);
        exe::crt_fread(&mut numero as *mut i32 as usize, 4, 1, f);
        if versao != 1 {
            // as in the original: the file stays open
            show_error_at(0xa159b0, if k == 0 { 0xbf6 } else { 0xc11 });
            return 0;
        }
        if numero as u32 == *desc.add(2) {
            crate::comparison::branch("events read");
            exe::crt_fseek(f, *desc as i32, 0);
            let r = exe::crt_fread(e + 0x720, 0xe, *desc.add(1), f);
            *((e + 6) as *mut u16) = *desc.add(1) as u16;
            exe::crt_fclose(f);
            return (r != 0) as u8;
        }
        exe::crt_fclose(f);
    }
    0
}

// simplification: the recorded events touch files and the network step only runs in a network game; without
// own comparison, validated by the career (Rust x original).

/// Like show_error_at (error box at file/line), then cleanup and exit(-1) like the original.
///
/// [pt] Como `show_error_at`, e o jogo para (como a original: 0x5cc7d0, 0x5a8f60 e sai com -1).
pub(crate) unsafe fn fatal_error_at(file: usize, row: u32) -> ! {
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    show_error_at(file, row);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// Reads `events_<language>.cfg` (or `events_eng.cfg`) into the table of the 677 event codes at `[0xb68788]` (7 option
/// bytes, the 0x110-byte sounds with weight, type, name and the two texts `G`/`R`); checks `<X` tokens, events without
/// text and, with `-verifysounds`, the sound files; at the end converts the weights and gives the default sound to
/// those without one.
///
/// About 5.2 KB: reads `events_<language>.cfg` (CM3_DATA; without it, `events_eng.cfg`, and without that the game
/// stops, error 0xcc4) into the table of the 677 event codes at `[0xb68788]` (0x12 bytes: 7 option bytes, +8 the
/// 0x110-byte sounds, +0xc count, +0xe the weight). With a different language since the last read, `free_event_tables` clears
/// first; already read (`[0xb6b723]`), nothing. Lines: `=id,o0..o6` opens an event (o4 between 0 and 11; o2 without o0
/// warns 0xd40), `>weight,type,name` adds a sound (with `-verifysounds` on the command line, checks the file in
/// CM3_SOUNDS), `G`/`R` the two texts of the last sound (allocated; unknown `<X` tokens warn). At the end, the weights
/// become (largest × 10) / weight and +0xe the sum; an event without a sound gets the default sound (0xb68670). Reads
/// through the game's `fgetc` and uses buffer `[0xde1f64]`, as the original.
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
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let format_text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const FONTE: usize = 0xa159b0; // match_events.cpp
    const TABELA: usize = 0xb68788;
    const FIM: usize = G_MATCH_EVENT_CONFIG_LOADED_FLAG;
    const BUF: usize = 0xde1f64;
    let event = |k: i32| TABELA.wrapping_add((k as isize * 0x12) as usize);
    let sons = |e: usize| core::ptr::read_unaligned((e + event_type::VARIANTS) as *const usize);
    let how_many = |e: usize| core::ptr::read_unaligned((e + event_type::VARIANT_COUNT) as *const i16);
    let weight = |e: usize| (e + event_type::TOTAL_WEIGHT) as *mut i32;

    if *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *const i32) != exe::langlib_get_language() {
        exe::match_events_free_event_tables();
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
        core::ptr::write_unaligned((e + event_type::VARIANTS) as *mut u32, 0);
        core::ptr::write_unaligned((e + event_type::VARIANT_COUNT) as *mut u16, 0);
        core::ptr::write_unaligned(weight(e), 0);
        e += 0x12;
    }
    let lingua = exe::langlib_get_language();
    *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *mut i32) = lingua;
    let name = match lingua {
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
    let mut f = if name != 0 { exe::crt_fopen(exe::utils_build_file_path(0x9b5348, name), 0x9899fc) } else { 0 };
    if f != 0 {
        *(FIM as *mut u8) = 1;
    } else {
        // simplification: the original tests [0xb6b723] again here; it has just been seen cleared
        *(FIM as *mut u8) = 1;
        f = exe::crt_fopen(exe::utils_build_file_path(0x9b5348, 0xa15c3c), 0x9899fc);
        if f == 0 {
            fatal_error_at(FONTE, 0xcc4);
        }
    }

    let end_of_file = || *((f + 0xc) as *const u8) & 0x10 != 0;
    let pula_brancos_e_le = || {
        let mut c = exe::at_9469e0(f);
        while (c == 0x20 || c == 0xd) && c != -1 {
            c = exe::at_9469e0(f);
        }
        c
    };
    // a field up to the comma or the end of the line, into the buffer; returns the last character read
    let field = || {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0x2c && c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = exe::at_9469e0(f);
        }
        *((BUF + n) as *mut u8) = 0;
        c
    };
    // a linha inteira (virgulas inclusive) num texto alocado; marcas `<X` desconhecidas avisam
    let text = |linha_alocacao: u32, row_mark: u32| -> usize {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = exe::at_9469e0(f);
        }
        *((BUF + n) as *mut u8) = 0;
        let t = exe::crt_malloc(n as u32 + 1);
        if t == 0 {
            fatal_error_at(FONTE, linha_alocacao);
        }
        core::ptr::copy_nonoverlapping(BUF as *const u8, t as *mut u8, n + 1);
        let mut i = 0usize;
        while *((t + i) as *const u8) != 0 {
            if *((t + i) as *const u8) == b'<' {
                let x = *((t + i + 1) as *const i8);
                if !b"CDFNOPRTYcdfnoprty".contains(&(x as u8)) {
                    format_text(BUF, 0xa15b74, x as i32);
                    show_error_at(FONTE, row_mark);
                }
            }
            i += 1;
        }
        t
    };

    let mut current = -1i32;
    while !end_of_file() {
        let c = exe::at_9469e0(f);
        if c == -1 {
            continue;
        }
        match c {
            0xd => {}
            0x3d => {
                // `=`: checks the previous event and opens the new one
                if current != -1 {
                    let e = event(current);
                    let s = sons(e);
                    if s == 0 {
                        format(BUF, 0xa15b38, current);
                        fatal_error_at(FONTE, 0xcdf);
                    }
                    for j in 0..how_many(e).max(0) as usize {
                        if core::ptr::read_unaligned((s + 0x110 * j + 0x108) as *const usize) == 0 {
                            format(BUF, 0xa15af0, current);
                            fatal_error_at(FONTE, 0xcea);
                        }
                    }
                }
                field();
                current = exe::at_947311(BUF);
                if !(0..=0x2a4).contains(&current) {
                    format(BUF, 0xa15aa0, current);
                    fatal_error_at(FONTE, 0xcfb);
                }
                let e = event(current);
                let mut c = 0;
                for o in 0..7usize {
                    c = field();
                    let v = (exe::at_94739c(BUF) as i32) as u8;
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
                    c = exe::at_9469e0(f);
                }
                if *((e + 2) as *const u8) != 0 && *(e as *const u8) == 0 {
                    format(BUF, 0xa15bf0, current);
                    show_error_at(FONTE, 0xd40);
                }
            }
            0x3e => {
                // `>`: one more sound in the open event
                if current == -1 {
                    fatal_error_at(FONTE, 0xd49);
                }
                let e = event(current);
                let new = exe::crt_realloc(sons(e), (how_many(e) as i32 + 1) as u32 * 0x110);
                if new == 0 {
                    fatal_error_at(FONTE, 0xd50);
                }
                core::ptr::write_unaligned((e + event_type::VARIANTS) as *mut usize, new);
                let s = new + 0x110 * how_many(e) as usize;
                field();
                *(s as *mut i16) = (exe::at_94739c(BUF) as i32) as i16;
                let p = (*(s as *const i16) as i32).max(*weight(e));
                core::ptr::write_unaligned(weight(e), p);
                field();
                *((s + 2) as *mut i16) = (exe::at_94739c(BUF) as i32) as i16;
                let mut c = exe::at_9469e0(f);
                *((s + 4) as *mut u8) = 0;
                while (c == 0x20 || c == 0xd) && c != -1 {
                    c = exe::at_9469e0(f);
                }
                let mut n = 0usize;
                while c != 0xa && c != -1 {
                    n += 1;
                    *((s + 3 + n) as *mut u8) = c as u8;
                    c = exe::at_9469e0(f);
                }
                *((s + 4 + n) as *mut u8) = 0;
                if exe::crt_strstr(0xae2c98, 0xa15be0) != 0 && n != 0 && exe::utils_file_exists(exe::utils_build_file_path(0x9c382c, s + 4)) == 0 {
                    format(BUF, 0xa15bac, exe::utils_build_file_path(0x9c382c, s + 4));
                    exe::gui_show_message_box(0x9870e0, BUF, 0);
                }
                core::ptr::write_unaligned((s + 0x108) as *mut usize, 0);
                core::ptr::write_unaligned((s + 0x10c) as *mut usize, 0);
                core::ptr::write_unaligned((e + event_type::VARIANT_COUNT) as *mut i16, how_many(e) + 1);
            }
            0x47 | 0x67 | 0x52 | 0x72 => {
                // `G`/`R`: the two texts of the last sound
                let (o, sem, cheio, ja, alocacao, marca) = if c == 0x47 || c == 0x67 {
                    (0x108usize, 0xd80, 0xd82, 0xd84, 0xd97, 0xdc3)
                } else {
                    (0x10c, 0xdcf, 0xdd1, 0xdd3, 0xde6, 0xe12)
                };
                if current == -1 {
                    fatal_error_at(FONTE, sem);
                }
                let e = event(current);
                if how_many(e) == 0 {
                    fatal_error_at(FONTE, cheio);
                }
                let field_text = (sons(e) as isize + how_many(e) as isize * 0x110 - 0x110 + o as isize) as usize;
                if core::ptr::read_unaligned(field_text as *const usize) != 0 {
                    fatal_error_at(FONTE, ja);
                }
                let t = text(alocacao, marca);
                // the original writes the pointer before copying; the result is the same
                core::ptr::write_unaligned(field_text as *mut usize, t);
            }
            _ => {
                let mut c = c;
                while c != 0xa && c != -1 {
                    c = exe::at_9469e0(f);
                }
            }
        }
    }

    // the weights: (largest x 10) / weight of each sound, and +0xe the sum; an event without sound gets the default
    for k in 0..0x2a5i32 {
        let e = event(k);
        if sons(e) == 0 {
            core::ptr::write_unaligned((e + event_type::VARIANTS) as *mut usize, G_MATCH_EVENT_DEFAULT_VARIANT);
            core::ptr::write_unaligned((e + event_type::VARIANT_COUNT) as *mut i16, 1);
            format(BUF, 0xa15b38, k);
            show_error_at(FONTE, 0xe46);
        } else {
            let total = (*weight(e)).wrapping_mul(10);
            core::ptr::write_unaligned(weight(e), total);
            let mut sum = 0i32;
            for j in 0..how_many(e).max(0) as usize {
                let s = (sons(e) + 0x110 * j) as *mut i16;
                *s = (core::ptr::read_unaligned(weight(e)) / *s as i32) as i16;
                sum += *s as i32;
            }
            core::ptr::write_unaligned(weight(e), sum);
        }
    }
    exe::crt_fclose(f);
}

// simplification: reads a file, allocates the texts and writes the global table; no comparison of its own, validated
// by the career (Rust x original) and by the comparison of the table at the end of the read.

/// Builds the event sound's `R` text with the tokens replaced (`<P1>` player, `<T1>`/`<T2>` club, `<C>` competition,
/// `<F>` stage, `<D>`/`<N>` number, `<rf>`/`<Rf>` referee), `{...}` prefix and suffix (`exe::langlib_apply_grammar_modifier`) and upper case; writes
/// to `[0xb67ea0]`.
///
/// Stdcall, ret 0x10: the text of event `ev` (+0 code, +2 which sound, +5/+6 and +7/+8 the two players (side, index),
/// +9 a number) of match `ctx` (+0/+4 the clubs, +0x40 the referee), with `comp` the competition (-1 none) and
/// `variant` the segment between `|` (0 the first). Reads the `R` text (+0x10c) of the chosen sound (at most the last)
/// and replaces the tokens: `<P1>`/`<P>` player (`commentary_player_name`), `<T1>`/`<T2>` club, `<C>` competition, `<F>` its stage
/// (`exe::comp_util_format_comp_stage_round_label`/`exe::comp_util_get_label_gender`), `<D>`/`<D1>` the number (+1), `<N>` the number in words (`exe::utils_format_ordinal`), `<rf>`/`<Rf>` the referee
/// ("Mr ..." or "the referee" depending on the language and a remainder of division by 5), `<O>`/`<Y>` nothing. `{...}`
/// before or after the token are prefix and suffix (`exe::langlib_apply_grammar_modifier`, which applies article and gender); a following letter `C`
/// puts it in upper case (`exe::utils_char_to_upper`). Writes into `[0xb67ea0]` (and returns it), recounting the length with `strlen`
/// after each token, as the original; without an event, without text or without the requested segment, 0. An unknown
/// token warns 0xb10.
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
pub(crate) unsafe extern "stdcall" fn event_text(ev: usize, ctx: usize, comp: i32, variant: u32) -> usize {
    const BUF: usize = 0xb67ea0;
    const TEMP: usize = 0xb67dd8;
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let format_text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if ev == 0 || ctx == 0 {
        return 0;
    }
    let k = (((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16) as isize;
    let e = (0xb68788isize + k * 18) as usize;
    let sons = core::ptr::read_unaligned((e + event_type::VARIANTS) as *const usize);
    if sons == 0 {
        return 0;
    }
    let n = core::ptr::read_unaligned((e + event_type::VARIANT_COUNT) as *const i16);
    let which = (ev + 2) as *mut i16;
    if *which as i32 > n as i32 - 1 {
        *which = n.wrapping_sub(1);
    }
    let mut s = core::ptr::read_unaligned((sons as isize + *which as isize * 0x110 + 0x10c) as *const usize);
    if s == 0 {
        return 0;
    }
    let v = variant as u16 as i16;
    if v > 0 {
        let mut left = v as u8 as i8;
        let mut c = *(s as *const u8);
        if c != 0 {
            loop {
                if c == b'|' {
                    left = left.wrapping_sub(1);
                    if left <= 0 {
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
    crate::comparison::branch("event text");
    let t = |i: i16| *((s as isize + i as isize) as *const u8);
    let dest = |o: i16| (BUF as isize + o as isize) as usize;
    let b = |o: usize| *((ev + o) as *const u8);
    let mut pos: i16 = 0;
    let mut out: i16 = 0;
    // the `{...}` prefix right before the mark
    let prefixo = |pos: i16, p: &mut [u8; 0x100]| {
        p[0] = 0;
        if pos > 1 && (t(pos - 1) == b'}' || t(pos - 1) == b']') {
            let mut a = pos - 2;
            while a >= 0 && t(a) != b'{' && t(a) != b'[' {
                a -= 1;
            }
            let j = a as i32;
            let end = pos as i32 - j - 1;
            let mut c = 1i32;
            while c < end {
                p[(c - 1) as usize] = t((j + c) as i16);
                c += 1;
            }
            p[(c - 1) as usize] = 0;
        }
    };
    // the `{...}` suffix right after the tag’s `>` (the search starts at `inicio`)
    let sufixo = |start: i16, x: &mut [u8; 0x100]| {
        x[0] = 0;
        let mut fe = start;
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
    let write = |out: i16, r: usize| {
        format(dest(out), 0x991944, r);
    };
    let em_maiusculas = |out: i16, r: usize| {
        let mut j: i16 = 0;
        while *((r as isize + j as isize) as *const u8) != 0 {
            let c = exe::utils_char_to_upper(*((r as isize + j as isize) as *const u8) as i32);
            *(dest(out + j) as *mut u8) = c;
            j += 1;
        }
        *(dest(out + j) as *mut u8) = 0;
    };
    let mut pre = [0u8; 0x100];
    let mut suf = [0u8; 0x100];
    let mut name = [0u8; 0xd0];
    let mut arb = [0u8; 0xd0];
    let mut text_stage = [0u8; 0x800];

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
                            exe::match_events_format_player_name(ctx, lado as i8 as i32, idx as i8 as i32, name.as_mut_ptr(), 0xc8, 0, 0, 1);
                            sufixo(pos + 3, &mut suf);
                            let r = exe::langlib_apply_grammar_modifier(name.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                            if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                write(out, r);
                            }
                        }
                    }
                    b'T' | b't' => {
                        let two = t(pos + 2) == b'2';
                        let home = b(6) != 0;
                        let id = if two == home { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                        let club = Club((*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(club::SIZE)));
                        prefixo(pos, &mut pre);
                        sufixo(pos + 2, &mut suf);
                        let g = club.short_gender_name() as u32;
                        let r = exe::langlib_apply_grammar_modifier(club.addr() + club::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g);
                        if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                            em_maiusculas(out, r);
                        } else {
                            write(out, r);
                        }
                    }
                    b'C' | b'c' | b'F' | b'f' => {
                        if comp >= 0 {
                            let cp = Competition((*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b)));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let r = if x as u8 == b'C' || x as u8 == b'c' {
                                let g = cp.short_gender_name() as u32;
                                exe::langlib_apply_grammar_modifier(cp.addr() + competition::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g)
                            } else {
                                exe::comp_util_format_comp_stage_round_label(cp, 0, 0x96, 3, text_stage.as_mut_ptr(), 0x7d0);
                                let g = exe::comp_util_get_label_gender(0, 0, 0x96, 3) as u32;
                                exe::langlib_apply_grammar_modifier(text_stage.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                            };
                            write(out, r);
                        }
                    }
                    b'D' | b'd' => {
                        let number = core::ptr::read_unaligned((ev + 9) as *const i32);
                        format(dest(out), 0x98806c, if t(pos + 2) == b'1' { number.wrapping_add(1) } else { number });
                    }
                    b'N' | b'n' => {
                        let r = exe::utils_format_ordinal(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                        write(out, r);
                    }
                    b'R' | b'r' => {
                        if t(pos + 2) == b'f' {
                            let fixed = if x as u8 == b'R' { 0xa15a14usize } else { 0xa159e0 };
                            let referee = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                            if referee <= -1 {
                                format_text(dest(out), fixed);
                            } else {
                                let rest = ((b(7) as i8 as i32 + b(5) as i8 as i32 + core::ptr::read_unaligned((ctx + 4) as *const i32)) % 5) as i8;
                                let sheet = (*(G_OFFICIAL_TABLE as *const usize)).wrapping_add((referee as usize).wrapping_mul(0x2b));
                                let mode = if exe::langlib_get_language() == 4 && rest == 0 {
                                    1
                                } else if rest == 0 {
                                    5
                                } else if rest > 0 && rest <= 2 {
                                    1
                                } else {
                                    0
                                };
                                match mode {
                                    1 => {
                                        exe::database_format_official_name(sheet, arb.as_mut_ptr(), 0xc8, 1);
                                        format_text(TEMP, 0xa15a20, arb.as_ptr());
                                    }
                                    5 => {
                                        exe::database_format_official_name(sheet, arb.as_mut_ptr(), 0xc8, 5);
                                        format_text(TEMP, 0xa159ec, arb.as_ptr());
                                    }
                                    _ => {
                                        format_text(TEMP, if x as u8 == b'R' { 0xa15a14 } else { 0xa159e0 });
                                    }
                                }
                                // as in the original: the finished text becomes the format
                                format(dest(out), TEMP);
                            }
                        }
                    }
                    b'O' | b'o' | b'Y' | b'y' => {}
                    _ => show_error_at(0xa159b0, 0xb10),
                }
                // end of the tag: up to the `>`; the length is recounted
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

// simplification: only runs in network games (creates and frees engine and summary); no comparison of its own.

/// The commentator writes the event on the panel and plays its sound: restores the crowd sound, builds the event's `G`
/// text (like `event_text`) into a line with the club colours (`<O>` the other's, `<Y>`/`<R>` the default ones), wraps or
/// cuts with "..." by the width (`exe::font_text_width`, `exe::utils_break_line_near_middle`), draws on the panel (`exe::screenmanager_set_widget_colour`…0x800580), plays the event sound
/// (`exe::screenmanager_play_sound`) and marks the time and duration (+0x2d6c, +0x2dac).
///
/// Ret 0x20, about 4.4 KB: commentator `nar` writes event `ev` of match `ctx` on `panel` (0xb74450) and plays its
/// sound. First, the crowd noise of the human whose turn it is (`[0xb77c7a]`; channel +0x20, "fxloop2.wav") is restored
/// if it stopped. With a valid code and option 1 zeroed, builds the `G` text (+0x108) of the chosen sound (segment +0xd
/// between `|`) as `event_text` does, but in its own line and with colours: it starts with those of the club in the
/// incident (`database_get_club_kit_colours` with the side's kit), `<O>` switches to the other's, `<Y>` and `<R>`/`<r>` without `f` the
/// default ones (`[0xaebdea]`/`[0xaebdec]` or `[0xae31a8]`); `<N>` is the number, `<n>` in words; the player name is
/// the short one if the whole text fits in less than 300 (`exe::font_text_width` with the `font`). Too wide for `width` - 16: wraps
/// (`exe::utils_break_line_near_middle`) or cuts at a word with "...". Draws (`exe::screenmanager_set_widget_colour`, `exe::screenmanager_set_widget_font`, `exe::screenmanager_set_widget_text_colour`, `exe::screenmanager_set_widget_text`, `exe::screenmanager_set_widget_style`, 0x800580 with
/// option 3). Finally, without a segment (+0xd) and with volume (+0x2db0 >= 101), plays the event sound (`exe::screenmanager_play_sound` on
/// the channel from option 6, side from option 5), and marks the time and the sound duration (+0x2d6c, +0x2dac = weight
/// × volume / 100).
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
pub(crate) unsafe extern "thiscall" fn commentate_on_panel(nar: usize, ev: usize, ctx: usize, comp: i32, panel: u32, width: i32, quebrar: u32, font: usize, mode: u32) -> u32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let format_text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const SOM: usize = G_SCREEN_MANAGER;
    const TABELA: isize = 0xb68788;

    if ev == 0 || ctx == 0 || panel as u16 == 0xffff {
        return 0;
    }
    let eu = || *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    let goal_crowd_reaction = |n: usize| ((n as isize) + eu() + 0x20) as *mut u8;
    if *goal_crowd_reaction(nar) != 0xff && exe::screenmanager_is_sound_playing(SOM, *goal_crowd_reaction(nar) as u32) == 0 {
        exe::screenmanager_stop_sound(SOM, *goal_crowd_reaction(nar) as u32);
        *goal_crowd_reaction(nar) = 0xff;
    }
    if *goal_crowd_reaction(nar) == 0xff {
        let p = exe::utils_build_file_path(0x9c382c, 0xa159a0);
        exe::screenmanager_play_sound(SOM, goal_crowd_reaction(nar) as usize, 2, p, 1);
    }
    let b = |o: usize| *((ev + o) as *const u8);
    let k = ((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16;
    let e = (TABELA + k as isize * 18) as usize;
    let which = || *((ev + 2) as *const i16) as isize;
    let sons = || core::ptr::read_unaligned((e + 8) as *const usize);
    let mut cor1 = [0u8; 4];
    let mut cor2 = [0u8; 4];

    let valido = (0..0x2a5).contains(&k) && *((e + 1) as *const u8) == 0;
    if valido {
        crate::comparison::branch("commentary on the panel");
        if sons() == 0 {
            return 0;
        }
        if core::ptr::read_unaligned((sons() as isize + which() * 0x110 + 0x108) as *const usize) == 0 {
            return 0;
        }
        // the colours of the move’s club (the side’s kit)
        let side = b(6);
        let mut club_id = if side != 0 { *((ctx + 4) as *const usize) } else { *(ctx as *const usize) };
        let kit = *((ctx as isize + side as i8 as isize * 1573 + 0x152) as *const u8) as u32;
        exe::database_get_club_kit_colours((*(G_CLUB_TABLE as *const usize)).wrapping_add(club_id.wrapping_mul(club::SIZE)), kit, (cor1.as_mut_ptr()) as *mut u32, (cor2.as_mut_ptr()) as *mut u32);
        let mut s = core::ptr::read_unaligned((sons() as isize + which() * 0x110 + 0x108) as *const usize);
        let v = b(0xd) as i8;
        if v > 0 {
            let mut left = v;
            let mut c = *(s as *const u8);
            if c != 0 {
                loop {
                    if c == b'|' {
                        left -= 1;
                        if left <= 0 {
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
        let mut row = [0u8; 0x800];
        let l = row.as_mut_ptr() as usize;
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
                let end = pos as i32 - j - 1;
                let mut c = 1i32;
                while c < end {
                    p[(c - 1) as usize] = t((j + c) as i16);
                    c += 1;
                }
                p[(c - 1) as usize] = 0;
            }
        };
        let sufixo = |start: i16, x: &mut [u8; 0x100]| {
            x[0] = 0;
            let mut fe = start;
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
        let write = |out: i16, r: usize| {
            format(dest(out), 0x991944, r);
        };
        let em_maiusculas = |out: i16, r: usize| {
            let mut j: i16 = 0;
            while *((r as isize + j as isize) as *const u8) != 0 {
                let c = exe::utils_char_to_upper(*((r as isize + j as isize) as *const u8) as i32);
                *(dest(out + j) as *mut u8) = c;
                j += 1;
            }
            *(dest(out + j) as *mut u8) = 0;
        };
        let mut pre = [0u8; 0x100];
        let mut suf = [0u8; 0x100];
        let mut name = [0u8; 0xd0];
        let mut inteiro = [0u8; 0x800];
        let mut arb = [0u8; 0xd0];
        let mut text_stage = [0u8; 0x800];
        let colours_default = |c1: &mut [u8; 4], c2: &mut [u8; 4], second: usize| {
            core::ptr::write_unaligned(c1.as_mut_ptr() as *mut u16, *(G_GUI_COLOUR_BLACK as *const u16));
            core::ptr::write_unaligned(c2.as_mut_ptr() as *mut u16, *(second as *const u16));
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
                                // the width of the whole text (up to the `|`) decides the short name
                                let mut n = 0usize;
                                while *((s + n) as *const u8) != 0 && *((s + n) as *const u8) != b'|' {
                                    inteiro[n] = *((s + n) as *const u8);
                                    n += 1;
                                }
                                inteiro[n] = 0;
                                let short = (exe::font_text_width(font, inteiro.as_ptr()) as i16) < 0x12c;
                                exe::match_events_format_player_name(ctx, lado_j as i8 as i32, idx as i8 as i32, name.as_mut_ptr(), 0xc8, short as u32, mode, 1);
                                sufixo(pos + 3, &mut suf);
                                let r = exe::langlib_apply_grammar_modifier(name.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                                if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                    em_maiusculas(out, r);
                                } else {
                                    write(out, r);
                                }
                            }
                        }
                        b'T' | b't' => {
                            let two = t(pos + 2) == b'2';
                            let home = b(6) != 0;
                            let id = if two == home { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let club = Club((*(G_CLUB_TABLE as *const usize)).wrapping_add(id.wrapping_mul(club::SIZE)));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let g = club.short_gender_name() as u32;
                            let r = exe::langlib_apply_grammar_modifier(club.addr() + club::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g);
                            if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                write(out, r);
                            }
                        }
                        b'C' | b'c' | b'F' | b'f' => {
                            if comp >= 0 {
                                let cp = Competition((*(G_CLUB_COMP_TABLE as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b)));
                                prefixo(pos, &mut pre);
                                sufixo(pos + 2, &mut suf);
                                let r = if x == b'C' || x == b'c' {
                                    let g = cp.short_gender_name() as u32;
                                    exe::langlib_apply_grammar_modifier(cp.addr() + competition::SHORT_NAME, pre.as_ptr(), suf.as_ptr(), g, g)
                                } else {
                                    exe::comp_util_format_comp_stage_round_label(cp, 0, 0x96, 3, text_stage.as_mut_ptr(), 0x7d0);
                                    let g = exe::comp_util_get_label_gender(0, 0, 0x96, 3) as u32;
                                    exe::langlib_apply_grammar_modifier(text_stage.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                                };
                                write(out, r);
                            }
                        }
                        b'D' | b'd' | b'N' => {
                            let number = core::ptr::read_unaligned((ev + 9) as *const i32);
                            let v = if x != b'N' && t(pos + 2) == b'1' { number.wrapping_add(1) } else { number };
                            format(dest(out), 0x98806c, v);
                        }
                        b'n' => {
                            let r = exe::utils_format_ordinal(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                            write(out, r);
                        }
                        b'O' | b'o' => {
                            // the other club’s colours
                            let side = b(6);
                            club_id = if side != 0 { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let a = (side == 0) as isize;
                            let kit = *((ctx as isize + a * 1573 + 0x152) as *const u8) as u32;
                            exe::database_get_club_kit_colours((*(G_CLUB_TABLE as *const usize)).wrapping_add(club_id.wrapping_mul(club::SIZE)), kit, (cor1.as_mut_ptr()) as *mut u32, (cor2.as_mut_ptr()) as *mut u32);
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'Y' | b'y' => {
                            colours_default(&mut cor1, &mut cor2, G_GUI_COLOUR_YELLOW);
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'R' | b'r' => {
                            if t(pos + 2) != b'f' {
                                colours_default(&mut cor1, &mut cor2, G_GUI_COLOUR_RED);
                                *(dest(out) as *mut u8) = 0;
                            } else {
                                let fixed = if x == b'R' { 0xa15a14usize } else { 0xa159e0 };
                                let referee = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                                if referee <= -1 {
                                    format_text(dest(out), fixed);
                                } else {
                                    let sum = (b(7) as i8 as i32)
                                        .wrapping_add(b(5) as i8 as i32)
                                        .wrapping_add(core::ptr::read_unaligned((ctx + 4) as *const i32))
                                        .wrapping_add(club_id as i32);
                                    let rest = (sum % 5) as i8;
                                    let sheet = (*(G_OFFICIAL_TABLE as *const usize)).wrapping_add((referee as usize).wrapping_mul(0x2b));
                                    let mode_name = if exe::langlib_get_language() == 4 && rest == 0 {
                                        1
                                    } else if rest == 0 {
                                        5
                                    } else if rest > 0 && rest <= 2 {
                                        1
                                    } else {
                                        0
                                    };
                                    match mode_name {
                                        1 => {
                                            exe::database_format_official_name(sheet, arb.as_mut_ptr(), 0xc8, 1);
                                            format_text(0xb67dd8, 0xa15a20, arb.as_ptr());
                                        }
                                        5 => {
                                            exe::database_format_official_name(sheet, arb.as_mut_ptr(), 0xc8, 5);
                                            format_text(0xb67dd8, 0xa159ec, arb.as_ptr());
                                        }
                                        _ => {
                                            format_text(0xb67dd8, fixed);
                                        }
                                    }
                                    format(dest(out), 0xb67dd8);
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
                    while row[n] != 0 {
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
        // too wide: wraps or cuts at a word with ”...”
        if exe::font_text_width(font, row.as_ptr()) > width - 0x10 {
            let mut n = 0usize;
            while row[n] != 0 {
                n += 1;
            }
            let mut si = n as i16;
            if quebrar as u8 != 0 {
                exe::utils_break_line_near_middle(dest(si / 5) as *mut u8, 0, 1);
            } else {
                loop {
                    si -= 1;
                    *(dest(si) as *mut u8) = 0;
                    if exe::font_text_width(font, row.as_ptr()) <= width - 0x10 {
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
                while row[n] != 0 {
                    n += 1;
                }
                for (i, c) in b"...\0".iter().enumerate() {
                    row[n + i] = *c;
                }
            }
        }
        exe::screenmanager_set_widget_colour(SOM, panel, core::ptr::read_unaligned(cor2.as_ptr() as *const u32));
        exe::screenmanager_set_widget_font(SOM, panel, font);
        exe::screenmanager_set_widget_text_colour(SOM, panel, core::ptr::read_unaligned(cor1.as_ptr() as *const u32));
        exe::screenmanager_set_widget_text(SOM, panel, (row.as_ptr()) as usize);
        exe::screenmanager_set_widget_style(SOM, panel, 0x30);
        exe::screenmanager_set_widget_flags(SOM, (panel) as u32 as usize, if *((e + 3) as *const u8) != 0 { 0x1001 } else { 1 });
    }

    // the event sound
    *(((nar as isize) + eu() + 0x10) as *mut u8) = 0xff;
    if b(0xd) as i8 <= 0 && *((nar + commentary_queue::VOLUME) as *const u8) >= 0x65 {
        let sound = (sons() as isize + which() * 0x110) as usize;
        if *((sound + 4) as *const u8) != 0 {
            let canal = if *((e + 6) as *const u8) == 0 { 0xb6b722usize } else { ((nar as isize) + eu() + 0x10) as usize };
            *(G_MATCH_EVENTS_CURRENT_SLOT_PTR as *mut usize) = canal;
            if canal != 0 {
                *(canal as *mut u8) = 0xfe;
                let side = match *((e + 5) as *const u8) {
                    0 => (b(6) != 0) as u32,
                    2 => (b(6) == 0) as u32,
                    _ => 2,
                };
                let p = exe::utils_build_file_path(0x9c382c, sound + 4);
                exe::screenmanager_play_sound(SOM, *(G_MATCH_EVENTS_CURRENT_SLOT_PTR as *const usize), side, p, 0);
            }
        }
    }
    let now = exe::utils_get_time_ms();
    core::ptr::write_unaligned(((nar as isize) + eu() * 4 + 0x2d6c) as *mut u32, now);
    let weight = *((sons() as isize + which() * 0x110 + 2) as *const i16) as i32;
    let volume = *((nar + commentary_queue::VOLUME) as *const u8) as i32;
    core::ptr::write_unaligned((nar + commentary_queue::SOUND_DURATION) as *mut i32, (weight.wrapping_mul(volume) / 100) as i16 as i32);
    0
}

// simplification: draws on the panel and plays sound; no comparison of its own (only runs with the match screen).

/// Advances the commentary queue read position (+0xc, ring of 12), marks the slot and switches the queue off on
/// reaching write/saved position; returns whether it did.
///
/// [pt] Avança a posição de leitura da fila da narração (+0xc, anel de 12): marca a nova vaga (+0x3d =
/// −1, tira o bit 0x8000 do evento) e, chegando na de escrita (+8, e então +0x2d68 = −1) ou na
/// guardada (+0xa), desliga a fila (+5 = 0). Devolve se desligou.
pub(crate) unsafe fn advance_commentary_read(t: usize) -> bool {
    let c = (t + commentary_queue::READ_POS) as *mut i16;
    *c = if c.read().wrapping_add(1) > 0xb { 0 } else { c.read().wrapping_add(1) };
    let p = t + commentary_queue::SLOTS + *c as usize * 14;
    *((p + 0xd) as *mut u8) = 0xff;
    *(p as *mut u16) &= 0x7fff;
    let mut desligou = false;
    if *c == *((t + commentary_queue::WRITE_POS) as *const i16) {
        *((t + 0x2d68) as *mut u32) = u32::MAX;
        *((t + commentary_queue::ACTIVE) as *mut u8) = 0;
        desligou = true;
    }
    if *c == *((t + commentary_queue::SAVED_POS) as *const i16) {
        *((t + commentary_queue::ACTIVE) as *mut u8) = 0;
        desligou = true;
    }
    desligou
}

/// Returns the next commentary event to show, part by part of the text (split by `|`), advancing the queue reading.
///
/// Ret 4: the next commentary event to show (0 with the queue switched off, +5): skips the empty slots; in the event's
/// text (table `[0xb68790 + 0x12 × (event − 8000)]`, 0x110-byte record +2, +0x108) moves to the next part separated by
/// `|` (the current part in +0xd; bit 0x8000 marks that there is more) and, when the text is finished, advances the
/// reading. In `out`, the slot's 0x86-byte record (+0xd8).
/// Original: `next_event` (0x6d78b0, match_events.cpp, thiscall).
///
/// [pt] { 0x6d78b0 } (thiscall, ret 4, match_events.cpp): o próximo evento da narração a mostrar (0 com a
/// fila desligada, +5): pula as vagas vazias; no texto do evento (tabela [0xb68790 + 0x12 × (evento
/// − 8000)], registro +2 de 0x110 bytes, +0x108) anda até a próxima parte separada por `|` (a parte
/// atual em +0xd; o bit 0x8000 marca que há mais) e, acabado o texto, avança a leitura. Em `saida`,
/// o registro de 0x86 bytes da vaga (+0xd8).
pub(crate) unsafe extern "thiscall" fn next_event(t: usize, out: *mut usize) -> usize {
    if *((t + commentary_queue::ACTIVE) as *const u8) == 0 {
        return 0;
    }
    crate::comparison::branch("commentary next event");
    if !out.is_null() {
        *out = (t as isize + 0xd8 + *((t + commentary_queue::READ_POS) as *const i16) as isize * 0x86) as usize;
    }
    let slot = |t: usize| t + commentary_queue::SLOTS + *((t + commentary_queue::READ_POS) as *const i16) as usize * 14;
    let mut p = slot(t);
    while *(p as *const u16) == 0 {
        if advance_commentary_read(t) {
            return 0;
        }
        p = slot(t);
    }
    let event = (*(p as *const u16) & 0x7fff).wrapping_add(0xe0c0) as i16 as isize;
    let record = *(p as *const i16).add(1) as isize;
    let table = *((0xb68790 + event * 0x12) as *const isize);
    let text = *((table + record * 0x110 + 0x108) as *const *const u8);
    let n = {
        let mut k = 0usize;
        while *text.add(k) != 0 {
            k += 1;
        }
        k as i16
    };
    let part = (p + 0xd) as *mut i8;
    let mut c = *part;
    let mut s = 0i16;
    while s < n {
        if *text.offset(s as isize) == b'|' {
            if c == -1 {
                *part = 0;
                break;
            }
            c = c.wrapping_sub(1);
            if c < 0 {
                s += 1;
                crate::comparison::branch("commentary text in parts");
                *(p as *mut u16) |= 0x8000;
                *part = (*part).wrapping_add(1);
                break;
            }
        }
        s += 1;
    }
    if *part != -1 {
        while s < n && *text.offset(s as isize) != b'|' {
            s += 1;
        }
    }
    if n <= s {
        advance_commentary_read(t);
    }
    p
}

/// The player name in the commentary: the short form, or the long one when another player of either club has the same
/// short name.
///
/// Writes into `buf` (`size` bytes) the name of player `player` on side `side` of match `pt` (0x49-byte record at
/// +0x190: id and the three names +0x198..+0x1a0, sides of 0x625 bytes) through `database_format_name_from_ids`: with `short` and random(5) =
/// 0, form 1; otherwise form 5, and if any of the 100 players of either club (+0, +4; list from `database_collect_club_squad`) with another
/// id or another name has the same form 5, the `long` form.
/// Original: `commentary_player_name` (0x6db2d0, match_events.cpp, cdecl).
///
/// [pt] { 0x6db2d0 } (cdecl, match_events.cpp): escreve em `buf` (`tam` bytes) o nome do jogador `jogador`
/// do lado `lado` da partida `pt` (registro de 0x49 bytes em +0x190: id e os três nomes +0x198..+0x1a0,
/// lados de 0x625 bytes) pela 0x53e140: com `curto` e sorteio(5) = 0 a forma 1; senão a forma 5, e se
/// algum dos 100 jogadores de um dos dois clubes (+0, +4; lista da 0x541f90) com outro id ou outro
/// nome tem a mesma forma 5, a forma `longa`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn commentary_player_name(pt: usize, player: i8, side: i8, buf: *mut u8, size: u32, short: u8, _x: u32, longa: u32) {
    let r = (pt as isize + player as isize * 0x49 + side as isize * 0x625) as usize;
    let d = |o: usize| *((r + o) as *const u32);
    if short != 0 && random(5) == 0 {
        exe::database_format_name_from_ids(d(0x198), d(0x19c), d(0x1a0), buf, size, 1, d(0x190));
        return;
    }
    crate::comparison::branch("commentary short name");
    exe::database_format_name_from_ids(d(0x198), d(0x19c), d(0x1a0), buf, size, 5, d(0x190));
    for k in 0..2 {
        let club = *((pt + 4 * k) as *const u32) as usize;
        let mut l = [0usize; 100];
        exe::database_collect_club_squad(*(G_CLUB_TABLE as *const usize) + club * club::SIZE, l.as_mut_ptr(), 1);
        for &q in l.iter() {
            if q == 0 {
                continue;
            }
            let n = |i: usize| *((*((q + 4 * i) as *const usize) + 0x33) as *const u32);
            if *(q as *const u32) == d(0x190) && n(1) == d(0x198) && n(2) == d(0x19c) && n(3) == d(0x1a0) {
                continue;
            }
            let mut other = [0u8; 200];
            exe::database_format_name_from_ids(n(1), n(2), n(3), other.as_mut_ptr(), 200, 5, d(0x190));
            if exe::utils_string_compare(buf, other.as_ptr(), 1, 0) == 0 {
                crate::comparison::branch("commentary long form name");
                exe::database_format_name_from_ids(d(0x198), d(0x19c), d(0x1a0), buf, size, longa, d(0x190));
                return;
            }
        }
    }
}

// simplification: writes into `buf` and calls 0x53e140 and 0x541f90 (effects outside); comparison by the text

/// The jump labels of `combine_with_previous` (the end of each one returns or continues to another).
///
/// [pt] Os rótulos de desvio da 0x6d60b0 (o fim de cada um devolve ou segue para outro).
#[derive(Clone, Copy)]
pub(crate) enum BranchLabel {
    R6cde,
    R6d1d,
    R69a6,
    R69f5,
    R6c95,
    R6cbf,
    Copiar,
}

/// Combines the new commentary event with the last queued one (returns the combined event and copies the previous
/// player, side and value).
///
/// Ret 8: combines the new event `e` (14 bytes: event word; +5, +6 player and side; +7, +8 the other; +9 integer) with
/// the last one in the queue (position +8 − 1 in the ring of 12, 14-byte records at +0x30): returns the combined event
/// (negative = replaces the previous one; 0 = no combination) and, in combinations, copies the player, the side, the
/// other and the integer of the previous one into `e`. `m` is the engine (0 outside the match): it gives the side in
/// possession (`engine::LAST_POSSESSION_SIDE`) and the ball in the area (`ball_goal_end`). The original's two calls to `band` have no effect and
/// their value is discarded (they are left out).
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
    let m = Engine(m);
    let w = |p: usize| core::ptr::read_unaligned(p as *const i16);
    let s2 = w(e);
    let (posse, area) = if m.is_null() { (0, 0) } else { (m.last_possession_side(), ball_goal_end(m.addr())) };
    if w(t + 8) == w(t + 6) {
        return 0;
    }
    let k = if w(t + 8) == 0 { 0xb } else { read_i8(t + 8).wrapping_sub(1) };
    let q = (t as isize + 0x30 + k as isize * 14) as usize;
    let s3 = w(q);
    let same = |o: usize| read_i8(q + o) == read_i8(e + o);
    let copiar = || core::ptr::copy((q as *const u8).add(5), (e as *mut u8).add(5), 8);
    // the previous one becomes the main one and the new one becomes the other (+7, +8)
    let swap = |inteiro: bool| {
        *((e + 8) as *mut i8) = read_i8(e + 6);
        *((e + 7) as *mut i8) = read_i8(e + 5);
        *((e + 5) as *mut i8) = read_i8(q + 5);
        *((e + 6) as *mut i8) = read_i8(q + 6);
        if inteiro {
            core::ptr::write_unaligned((e + 9) as *mut u32, core::ptr::read_unaligned((q + 9) as *const u32));
        }
    };
    let contexto = |v: i16| (0x2021..=0x20d0).contains(&v);

    if s2 == 0x1f6f {
        if posse == read_i8(e + 6) {
            return 0x21bb;
        }
    } else if s2 == 0x1f92 && (s3 == 0x1f6c || s3 == 0x1f6e) {
        return -0x1f9c;
    }
    if s3 == 0x1f7d && read_i8(e + 6) != read_i8(q + 6) && s2 == 0x1f46 {
        return -0x219d;
    }
    let shot = (0x1f87..=0x1fbd).contains(&s2);
    if (shot || (s2 == 0x1f5f && area != 0)) && s3 == 0x1f7e {
        core::ptr::write_unaligned(q as *mut i16, 0x219c);
        core::ptr::write_unaligned((q + 2) as *mut i16, exe::utils_random(2) as i16);
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
            break 'escolha BranchLabel::R6cde;
        }
        if s3 == 0x1f69 {
            if s2 == 0x1f9d {
                return if exe::utils_random(3) != 0 { 0 } else { 0x2198 };
            }
            break 'escolha BranchLabel::R6cde;
        }
        if contexto(s3) && matches!(s2, 0x1f46 | 0x213e | 0x1f60 | 0x1f5f) && read_i8(e + 6) != read_i8(q + 6) {
            let side = read_i8(q + 6);
            if (area == -1 && side == 1) || (area == 1 && side == 0) {
                return -0x2178;
            }
        }
        if s3 == 0x1f6f {
            if s2 == 0x1f46 {
                if same(6) || area == 0 {
                    return 0;
                }
                swap(false);
                return 0x2177;
            }
            break 'escolha BranchLabel::R6cde;
        }
        for (de, para) in [(0x2111, 0x2140), (0x210f, 0x2141), (0x1fe6, 0x2142), (0x1fe7, 0x2142)] {
            if s3 == de {
                if s2 == 0x1f46 {
                    if !same(5) || !same(6) {
                        return 0;
                    }
                    copiar();
                    return para;
                }
                break 'escolha BranchLabel::R6cde;
            }
        }
        if (s3 == 0x213e || s3 == 0x1f71) && s2 == 0x1f46 && same(5) && same(6) {
            copiar();
            return 0x1f46;
        }
        if s3 == 0x213e {
            if s2 == 0x1f41 {
                copiar();
                return 0x213e;
            }
            break 'escolha BranchLabel::R69a6;
        }
        if s3 == 0x1f41 {
            if s2 == 0x1f46 {
                copiar();
                return 0x1f46;
            }
            break 'escolha BranchLabel::R6cde;
        }
        if s3 == 0x2113 {
            if (0x1f86..=0x1fbe).contains(&s2) && exe::utils_random(3) == 0 && same(5) && same(6) {
                copiar();
                return s2;
            }
            break 'escolha BranchLabel::R6c95;
        }
        if s3 == 0x1f47 {
            if s2 == 0x1f44 || s2 == 0x1f43 {
                if !same(5) || !same(6) {
                    return 0;
                }
                copiar();
                return s2;
            }
            if (s2 == 0x1f45 || s2 == 0x20de) && same(5) && same(6) {
                copiar();
                return s2;
            }
            break 'escolha BranchLabel::R6cde;
        }
        if s3 == 0x1f4d {
            if s2 == 0x1f4a {
                if exe::utils_random(2) == 0 || !same(5) {
                    return 0;
                }
                if (!same(6) || !same(7) || !same(8)) && (!same(6) || (read_i8(q + 7) != -1 && read_i8(e + 7) != -1)) {
                    return 0;
                }
                copiar();
                return 0x1f4a;
            }
            break 'escolha BranchLabel::R6cde;
        }
        if s3 == 0x1f43 {
            if s2 == 0x1f9d {
                if exe::utils_random(2) == 0 {
                    return 0;
                }
                copiar();
                return 0x1f43;
            }
            break 'escolha BranchLabel::R6cde;
        }
        if matches!(s3, 0x1f62 | 0x1f63 | 0x2150) && s2 == 0x1f46 && same(5) && same(6) {
            break 'escolha BranchLabel::Copiar;
        }
        if s3 == 0x1f5d {
            if s2 == 0x1f6f {
                copiar();
                return 0x2143;
            }
            break 'escolha BranchLabel::R6cde;
        }
        if contexto(s3) && s2 == 0x1f46 && read_i8(q + 7) == read_i8(e + 5) && read_i8(q + 8) == read_i8(e + 6) {
            break 'escolha BranchLabel::Copiar;
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
                if exe::utils_random(3) == 0 {
                    return 0;
                }
                copiar();
                return if s3 == 0x1f84 { 0x2144 } else { 0x2145 };
            }
            0x1f60 | 0x1f46 | 0x1f84 | 0x1f85 => {}
            0x1f80 => {
                if s2 == 0x1f46 {
                    if exe::utils_random(2) != 0 && !same(6) {
                        swap(true);
                        return 0x2146;
                    }
                    break 'escolha BranchLabel::Copiar;
                }
            }
            0x1f5e => break 'escolha BranchLabel::R69a6,
            _ => break 'escolha BranchLabel::R69f5,
        }
        BranchLabel::R6cde
    };

    loop {
        r = match r {
            BranchLabel::R69a6 => {
                if s2 == 0x1f46 && exe::utils_random(5) != 0 && same(6) && !same(5) {
                    swap(true);
                    return 0x2147;
                }
                BranchLabel::R69f5
            }
            BranchLabel::R69f5 => 'r: {
                for (de, para) in [(0x2148, 0x2149), (0x214a, 0x214b), (0x214c, 0x214d)] {
                    if s3 == de {
                        if s2 == 0x1f60 || s2 == 0x1f5f {
                            copiar();
                            return para;
                        }
                        break 'r BranchLabel::R6cde;
                    }
                }
                if contexto(s3) {
                    let (ie, iq) = (core::ptr::read_unaligned((e + 9) as *const i32), core::ptr::read_unaligned((q + 9) as *const i32));
                    if contexto(s2) && (1..0x15).contains(&ie) && (1..0x15).contains(&iq) && read_i8(e + 5) == read_i8(q + 7) && read_i8(e + 7) == read_i8(q + 5) {
                        copiar();
                        return 0x214e;
                    }
                    if s2 == 0x1f47 && (1..0x15).contains(&iq) && exe::utils_random(0x16 - iq) != 0 && read_i8(q + 7) == read_i8(e + 5) && read_i8(q + 8) == read_i8(e + 6) {
                        break 'r BranchLabel::Copiar;
                    }
                }
                if s3 == 0x214f {
                    if s2 == 0x1f92 {
                        copiar();
                        return 0x214f;
                    }
                    break 'r BranchLabel::R6cde;
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
                    break 'r BranchLabel::R6cde;
                }
                if s3 != 0x2021 {
                    if s3 == 0x1f55 {
                        if s2 == 8000 {
                            return -0x2152;
                        }
                    } else if s3 == 0x2113 {
                        break 'r BranchLabel::R6c95;
                    } else if contexto(s3) {
                        break 'r BranchLabel::R6cbf;
                    }
                    break 'r BranchLabel::R6cde;
                }
                if matches!(s2, 0x1f62 | 0x1f63 | 0x2150) {
                    return -0x2153;
                }
                BranchLabel::R6cbf
            }
            BranchLabel::R6cbf => {
                if s2 != 0x1f70 {
                    BranchLabel::R6cde
                } else if same(6) {
                    return -0x2157;
                } else {
                    BranchLabel::R6d1d
                }
            }
            BranchLabel::R6c95 => {
                if s2 == 0x1f62 || s2 == 0x2150 {
                    return -0x2153;
                }
                BranchLabel::R6cde
            }
            BranchLabel::R6cde => {
                if (0x2076..=0x20d0).contains(&s2) && s3 == 0x2113 {
                    return if same(5) && same(6) { -0x2159 } else { 0 };
                }
                BranchLabel::R6d1d
            }
            BranchLabel::R6d1d => {
                const GRUPO: [i16; 8] = [0x1f4b, 0x1f4c, 0x1f4e, 0x1f4f, 0x1f50, 0x1f51, 0x1f52, 0x1f53];
                if (0x215a..=0x215f).contains(&s3) && s2 == 0x1f70 {
                    return -0x2157;
                }
                if s3 == 0x1f70 {
                    if s2 == 0x1f76 && !same(6) {
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
                if GRUPO.contains(&s3) && GRUPO.contains(&s2) && exe::utils_random(2) != 0 {
                    return -0x2169;
                }
                if s3 == 0x216e {
                    return if s2 == 0x1f44 && same(5) && same(6) { 0x216e } else { 0 };
                }
                if matches!(s3, 0x1f80 | 0x2131 | 0x2130) && s2 == 0x1f46 {
                    BranchLabel::Copiar
                } else if s3 != 0x1f73 && !(0x20b9..=0x20c4).contains(&s3) && s3 != 0x20d0 && s3 != 0x1f80 {
                    return 0;
                } else if s2 != 0x1f46 && s2 != 0x1f47 {
                    return 0;
                } else {
                    BranchLabel::Copiar
                }
            }
            BranchLabel::Copiar => {
                copiar();
                return s3;
            }
        }
    }
}

/// Neighbourhood probe of the context commentary.
///
/// The game only asks for the goal event, so each real call also compares the 13 comments (0x21a3..=0x21af) for both
/// sides and, with the ratings of the side's 20 players (+0x47cb) set exactly at each average threshold (7501, 7001,
/// 8000, 7750, 6699), the `>=`/`>` boundaries; and, with fake media (one news item from today of each type for both
/// clubs) and minute 31, the paths that depend on the news.
///
/// The random generator state and the ratings are restored, and the random draws are neither counted nor recorded in
/// the trace of an ongoing tape comparison (the shot, `shoot`, calls this routine from inside): the trajectory of the
/// game while comparing does not change.
///
/// [pt] Sonda de vizinhança do comentário de contexto: o jogo só pergunta pelo evento do gol, então cada
/// chamada real compara também os 13 comentários (0x21a3..=0x21af) nos dois lados e, com as notas
/// dos 20 jogadores do lado (+0x47cb) postas exatamente em cada limite de média (7501, 7001, 8000,
/// 7750, 6699), as fronteiras de `>=`/`>`; e, com uma mídia falsa (uma notícia de hoje de cada tipo
/// para os dois clubes) e o minuto 31, os caminhos que dependem das notícias. O estado do sorteador e as notas voltam ao que eram e os
/// sorteios não contam nem entram no rastro de uma comparação pela fita em curso (o chute, 0x702050,
/// chama esta rotina por dentro): a trajetória do jogo comparando não muda.
pub(crate) unsafe fn probe_context_commentary(m: Engine, o: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32) -> u32) {
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    // the fake media: in the news log (0x757da0 searches by subject -1, club id, type -1 and
    // subtype, records of 0x17 bytes in order), one of today’s news items of each subtype that the
    // the comments query, for both clubs
    let c = m.r#match();
    let today = *(G_CURRENT_DATE as *const [u8; 8]);
    let mut records: Vec<[u8; 0x17]> = Vec::new();
    for o_clube in [0x1c, 0x20] {
        let club = *((c + o_clube) as *const usize);
        if club == 0 {
            continue;
        }
        let id = *(club as *const i32);
        for sub in [0x3fi16, 0x40, 0x41, 0x5c, 0x5f, 0x60, 0x61, 0x62, 99] {
            let mut r = [0u8; 0x17];
            r[..8].copy_from_slice(&today);
            r[8..0xc].copy_from_slice(&(-1i32).to_le_bytes());
            r[0xc..0x10].copy_from_slice(&id.to_le_bytes());
            r[0x11..0x13].copy_from_slice(&sub.to_le_bytes());
            r[0x13..0x17].copy_from_slice(&(-1i32).to_le_bytes());
            records.push(r);
        }
    }
    let key = |r: &[u8; 0x17]| (i32::from_le_bytes(r[0xc..0x10].try_into().unwrap()), i16::from_le_bytes(r[0x11..0x13].try_into().unwrap()));
    records.sort_by_key(key);
    records.dedup_by_key(|r| key(r));
    let falsa: [usize; 3] = [records.as_ptr() as usize, 0, records.len()];
    let average = *(G_MEDIA as *const usize);
    let minute = m.clock();
    crate::rng::pause_trace(|| crate::rng::without_counting(|| {
        // 0: the real state; 1: with the news; 2: with the news at minute 31
        for cenario in 0..3 {
            *(G_MEDIA as *mut usize) = if cenario == 0 { average } else { falsa.as_ptr() as usize };
            m.set_clock(if cenario == 2 { 31 * 11 } else { minute });
            for lado in 0..2u32 {
                let notes: Vec<(usize, usize)> = (0..20).map(|k| (m.player(lado as i8, k).addr() + match_player::MATCH_RATING, 2)).collect();
                let guardadas = crate::comparison::MemorySnapshot::read(&notes);
                for limit in [None, Some(0x1d4di16), Some(0x1b59), Some(8000), Some(0x1e46), Some(0x1a2b)] {
                    if let Some(v) = limit {
                        for &(a, _) in &notes {
                            core::ptr::write_unaligned(a as *mut i16, v);
                        }
                    }
                    for evento in 0x21a3..=0x21afu32 {
                        let mut out = 0i32;
                        let ps = &mut out as *mut i32;
                        let mut reg = crate::rng::STATE_REGIONS.to_vec();
                        reg.push((ps as usize, 4));
                        let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(m.addr(), lado, evento, ps), || context_commentary(m.addr(), lado, evento, ps));
                        SCOREBOARD.record(igual && r == n, || format!("sonda context_commentary(cenario {}, {}, {:#x}, media {:?}) original={} rust={} {:x?} x {:x?}", cenario, lado, evento, limit, r, n, mo, mr));
                    }
                }
                guardadas.write_back();
            }
        }
        *(G_MEDIA as *mut usize) = average;
        m.set_clock(minute);
    }));
    state.write_back();
}

/// Puts an event at the front of the commentary queue (steps the 12-slot ring back, drops the repeat) and queues it via
/// 0x6d6f70.
///
/// Ret 0x1c: the write position (+8, ring of 12) steps back one slot (from 0 it goes to 11); if the last one in the
/// queue (+6 items, word +0x712 of 14 bytes) is the same event as at the new position (+0x30), the queue loses one;
/// then queues through 0x6d6f70 with the same 7 arguments.
/// Original: `event_at_front` (0x6d5d00, match_events.cpp, thiscall).
///
/// [pt] { 0x6d5d00 } (thiscall, ret 0x1c, match_events.cpp): põe um evento na frente da fila da narração:
/// a posição de escrita (+8, anel de 12) recua uma casa (de 0 vai a 11); se o último da fila (+6
/// itens, palavra +0x712 de 14 bytes) é o mesmo evento da nova posição (+0x30), a fila perde um;
/// depois enfileira pela 0x6d6f70 com os mesmos 7 argumentos.
pub(crate) unsafe extern "thiscall" fn event_at_front(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    let pos = read_i16(t + 8);
    let pos = if pos == 0 { 0xb } else { pos.wrapping_sub(1) };
    core::ptr::write_unaligned((t + 8) as *mut i16, pos);
    let n = read_i16(t + 6);
    if n > 0 && read_i16(t + (0x712 + n as usize * 14)) == read_i16(t + (0x30 + pos as usize * 14)) {
        core::ptr::write_unaligned((t + 6) as *mut i16, n - 1);
    }
    exe::match_events_add_event(t, a, b, c, d, e, f, g)
}

/// The commentary queue constructor: clears the queue, reads the events and, if asked, plays the crowd loop.
///
/// Ret 4: zeroes the queue and the fields (+0x2db0 = 125, +1 and +0xa = −1, +0x2d68 = −1), the event globals
/// (0xb68670..), reads the events (`read_events_cfg`), marks the 16 slots (+0x10, +0x20) with −1 and zeroes the 16 times
/// (+0x2d6c); with `crowd`, plays the crowd loop of the side whose turn it is (`[0xb77c7a]`) through `exe::screenmanager_play_sound`. Returns
/// `this`.
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
    core::ptr::write_unaligned((t + commentary_queue::SOUND_DURATION) as *mut u32, 0);
    b(4, 0);
    *(G_MATCH_EVENT_DEFAULT_VARIANT as *mut u16) = 1;
    *(G_MATCH_EVENT_DEFAULT_VARIANT_LINE_DELAY as *mut u16) = 0x2bc;
    *(G_MATCH_EVENT_DEFAULT_VARIANT_TEXT as *mut u8) = 0;
    *(0xb68778 as *mut u32) = 0xad9c64;
    *(0xb6877c as *mut u32) = 0xad9c64;
    exe::match_events_read_config();
    for k in 0..16 {
        b(0x10 + k, 0xff);
        b(0x20 + k, 0xff);
        core::ptr::write_unaligned((t + commentary_queue::SOUND_START_TIMES + 4 * k) as *mut u32, 0);
    }
    if goal_crowd_reaction != 0 {
        let side = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        let name = exe::utils_build_file_path(0x9c382c, 0xa159a0);
        exe::screenmanager_play_sound(G_SCREEN_MANAGER, (t as isize + 0x20 + side) as usize, 2, name, 1);
    }
    t
}

/// Frees the loaded event tables and each record's texts.
///
/// In each of the 0x12-byte entries from 0xb68790 to 0xb6b72a with its own table (neither null nor the default
/// 0xb68670), the two texts (+0x108, +0x10c) of each 0x110-byte record and the table; then zeroes 0xb6b723 and puts −1
/// in 0xa1599c.
/// Original: `free_event_tables` (0x6d5b50, match_events.cpp, cdecl).
///
/// [pt] { 0x6d5b50 } (cdecl, match_events.cpp): libera as tabelas de eventos carregadas: em cada uma das
/// entradas de 0x12 bytes de 0xb68790 a 0xb6b72a com tabela própria (nem nula, nem a padrão
/// 0xb68670), os dois textos (+0x108, +0x10c) de cada registro de 0x110 bytes e a tabela; depois
/// zera 0xb6b723 e põe −1 em 0xa1599c.
pub(crate) unsafe extern "C" fn free_event_tables() {
    let mut e = 0xb68790usize;
    while e < G_MATCH_EVENT_TABLE_END_MARKER {
        let table = core::ptr::read_unaligned(e as *const usize);
        if table != G_MATCH_EVENT_DEFAULT_VARIANT && table != 0 {
            let mut k = 0i32;
            while k < core::ptr::read_unaligned((e + 4) as *const i16) as i32 {
                let r = table + k as usize * 0x110;
                for o in [0x108, 0x10c] {
                    let p = core::ptr::read_unaligned((r + o) as *const usize);
                    if p != 0 {
                        exe::crt_free(p);
                    }
                }
                k += 1;
            }
            exe::crt_free(table);
            core::ptr::write_unaligned(e as *mut usize, 0);
        }
        e += 0x12;
    }
    *(G_MATCH_EVENT_CONFIG_LOADED_FLAG as *mut u8) = 0;
    *(G_MATCH_EVENTS_CONFIG_LANGUAGE as *mut u32) = u32::MAX;
}

// simplification: all three queue commentary, play sound or free memory (do not run twice): without
// own comparison, proven by the career (queue of 19414 matches, 285 thousand events ahead).

/// Tells whether the sounds of the 16 marked events have played and passed their window (clock `exe::utils_get_time_ms`; stops the
/// finished channels, `exe::screenmanager_stop_sound`).
///
/// Ret 4: have the sounds of the `active` events (16 marks) played? For each marked one, a channel (+0x10 + k; -1 free,
/// -2 waiting) that is still playing (`exe::screenmanager_is_sound_playing` on 0xb74450) holds; one that has finished is stopped (`exe::screenmanager_stop_sound`) and
/// freed. Returns 1 only if all marked ones have passed their window (+0x2d6c + 4k up to + `[+0x2dac]`, by clock
/// `exe::utils_get_time_ms`).
/// Original: `sounds_finished` (0x6d8c70, match_events.cpp, thiscall, ret 4).
///
/// [pt] { 0x6d8c70 } (thiscall, ret 4, match_events.cpp): os sons dos eventos `ativos` (16 marcas) ja
/// tocaram? Para cada marcado, o canal (+0x10 + k; -1 livre, -2 espera) que ainda toca (0x8029f0 em
/// 0xb74450) segura; o que acabou e parado (0x802a50) e liberado. Devolve 1 so se todos os marcados
/// ja passaram da janela (+0x2d6c + 4k ate + [+0x2dac], pelo relogio 0x90c510).
pub(crate) unsafe extern "thiscall" fn sounds_finished(e: usize, ativos: *const u32) -> u8 {
    let now = exe::utils_get_time_ms();
    crate::comparison::branch("event sounds");
    for k in 0..16usize {
        if *ativos.add(k) == 0 {
            continue;
        }
        let start = *((e + commentary_queue::SOUND_START_TIMES + 4 * k) as *const u32);
        let end = (*((e + commentary_queue::SOUND_DURATION) as *const u32)).wrapping_add(start);
        let passed = now > end || now < start;
        let canal = (e + commentary_queue::SOUND_CHANNELS + k) as *mut u8;
        if *canal != 0xff {
            if *canal == 0xfe {
                return 0;
            }
            if exe::screenmanager_is_sound_playing(G_SCREEN_MANAGER, (start & !0xff) | *canal as u32) != 0 {
                return 0;
            }
            exe::screenmanager_stop_sound(G_SCREEN_MANAGER, *canal as u32);
            *canal = 0xff;
        }
        if !passed {
            return 0;
        }
    }
    1
}

/// Swaps the commentary event for a variant, by a random draw and by the play type.
///
/// Stdcall, ret 0xc: with random(3) ≠ 0 it stays the same; otherwise, by the play type `type` (1, 2, 8, 0x10, 0x20, the
/// last with another random(3)) there are swap tables; plays 2..4 (low byte of `sides`) have their own, and the upper
/// byte 3..5 swaps 0x1f4b for 0x217e.
/// Original: `event_variant` (0x6d5d80, match_events.cpp, stdcall).
///
/// [pt] { 0x6d5d80 } (stdcall, ret 0xc, match_events.cpp): troca o evento da narração por uma variante:
/// com exe::utils_random(3) ≠ 0 fica o mesmo; senão, pelo tipo do lance `tipo` (1, 2, 8, 0x10, 0x20 — este com
/// outro exe::utils_random(3)) há tabelas de troca; os lances 2..4 (byte baixo de `lados`) têm as suas, e o
/// byte de cima 3..5 troca 0x1f4b por 0x217e.
pub(crate) unsafe extern "system" fn event_variant(e: *const i16, sides: u32, kind: u8) -> i16 {
    if exe::utils_random(3) != 0 {
        return *e;
    }
    let v = *e;
    let table: &[(i16, i16)] = match kind {
        1 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fee, 0x218d), (0x1f49, 0x2182), (0x205a, 0x2190), (0x205b, 0x2191), (0x2079, 0x2192)],
        2 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x1f48, 0x2181), (0x1f41, 0x2180), (0x1f44, 0x2187), (0x1f86, 0x2189), (0x1fa2, 0x218a)],
        8 => &[(0x216f, 0x2199), (0x2150, 0x2197), (0x2112, 0x2195), (0x1fe6, 0x2193), (0x1fee, 0x218d), (0x205a, 0x218e), (0x205b, 0x218f), (0x1f4b, 0x2183), (0x1f41, 0x2184), (0x1f4a, 0x2188)],
        0x10 => &[(0x1f48, 0x2185)],
        0x20 if exe::utils_random(3) == 0 => &[(0x216f, 0x2199), (0x1fee, 0x218d), (0x1f42, 0x217c), (0x1f4b, 0x217d), (0x1f48, 0x2186)],
        _ => &[],
    };
    if let Some(&(_, n)) = table.iter().find(|&&(de, _)| de == v) {
        return n;
    }
    if matches!(sides as u8, 2..=4) {
        let lance: &[(i16, i16)] = &[(0x2112, 0x2196), (0x1fa6, 0x2194), (0x1f8b, 0x218b), (0x1fbf, 0x218c), (0x211f, 0x217f)];
        if let Some(&(_, n)) = lance.iter().find(|&&(de, _)| de == v) {
            return n;
        }
    }
    if matches!((sides >> 8) as u8, 3..=5) && v == 0x1f4b {
        return 0x217e;
    }
    v
}

