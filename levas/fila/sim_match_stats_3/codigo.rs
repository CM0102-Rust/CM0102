// ---------- %%LEVA%%a leva: as estatísticas da partida: ler e gravar o registro no arquivo temporário e registrar a partida (match_stats.cpp) ----------

/// Os registros de estatística da partida têm 0xd7a bytes no "match_stats_matches.tmp" de CM3_TEMP.
const MATCH_STATS_RECORD_SIZE: u32 = 0xd7a;

/// { 0x729600 } (thiscall, ret 8, match_stats.cpp): lê o registro `k` (0xd7a bytes, na posição
/// k × 0xd7a) do "match_stats_matches.tmp" (CM3_TEMP, "rb") para `saida`. Índice −1: devolve 0 calado;
/// fora de 0..[this+0x1c] ou saída nula: a original (caixa de erro). Sem abrir: erro 0x128; fseek ou
/// fread falham: erro 0x12f/0x137 e fecha. Devolve 1 se leu.
unsafe extern "thiscall" fn read_match_record(estat: usize, k: i32, saida: usize) -> u32 {
    if k < 0 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) || saida == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I0%%)(estat, k, saida);
    }
    crate::comparison::branch("estatisticas ler registro");
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(0x946a22usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x98f928);
    if f == 0 {
        show_error_at(0xa17ba0, 0x128);
        return 0;
    }
    if buscar(f, k.wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
        show_error_at(0xa17ba0, 0x12f);
        fechar(f);
        return 0;
    }
    if ler(saida, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {
        show_error_at(0xa17ba0, 0x137);
        fechar(f);
        return 0;
    }
    fechar(f);
    1
}

/// { 0x7293f0 } (thiscall, ret 8, match_stats.cpp): registra a partida `m` que terminou: preenche o
/// registro `r` (0x729ff0, sem pular a renda) e, salvo jogo da competição [0x9cf684] com +0x4c no
/// registro, grava (0x729820) com o jogo [m+0x4792] e os eventos [m+0x8f7a]. Partida ou registro
/// nulos: a original (caixa de erro, devolve 0). Devolve 1.
unsafe extern "thiscall" fn match_screens_record_match(estat: usize, m: usize, r: usize) -> u32 {
    if m == 0 || r == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(estat, m, r);
    }
    crate::comparison::branch("estatisticas registrar partida");
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x729ff0usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x729820usize);
    preencher(estat, m, r, 0);
    let jogo = core::ptr::read_unaligned((m + 0x4792) as *const usize);
    if *(jogo as *const i32) != *(0x9cf684 as *const i32) || *((r + 0x4c) as *const u8) == 0 {
        gravar(estat, jogo, r, core::ptr::read_unaligned((m + 0x8f7a) as *const usize));
    }
    1
}

/// { 0x729820 } (thiscall, ret 0xc, match_stats.cpp): grava o registro `r` do jogo `jogo` no
/// "match_stats_matches.tmp" ("r+b"): zera os 13 bytes de +0xd69, grava os eventos (0x6d98b0 de
/// `eventos`, humano se um dos clubes +0x1c/+0x20 é de humano, 0x600a50) e escreve o registro na vaga
/// [jogo+8]; sem vaga (−1), na vaga do cursor do balde (0x729bd0) em [this+8+4b], que anda e volta ao
/// começo do balde ao chegar ao fim ((b+1) × [this+0x1c] / 4). No fim, a entrada [jogo+8] do índice
/// (12 bytes em [this]) recebe +0xc, +0x10 e as datas +0x2a/+0x28. Argumento nulo: a original (caixa
/// de erro). Falhas do arquivo: erro 0x172/0x17f/0x18d/0x1a3 (fechando, como a original). Devolve 1.
unsafe extern "thiscall" fn write_match_record(estat: usize, jogo: usize, r: usize, eventos: usize) -> u32 {
    if jogo == 0 || r == 0 || eventos == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%)(estat, jogo, r, eventos);
    }
    crate::comparison::branch("estatisticas gravar registro");
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let record_events: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x6d98b0usize);
    let balde: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x729bd0usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let buscar: unsafe extern "C" fn(usize, i32, u32) -> i32 = core::mem::transmute(0x946a22usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let d = |o: usize| (estat + o) as *mut i32;
    let j = |o: usize| (jogo + o) as *mut i32;

    let desc = r + 0xd69;
    core::ptr::write_bytes(desc as *mut u8, 0, 13);
    let h = (humano(*((jogo + 0x1c) as *const usize), 1, 1, 0, 0) != 0 || humano(*((jogo + 0x20) as *const usize), 1, 1, 0, 0) != 0) as u32;
    record_events(eventos, desc, h);
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x9c7848);
    if f == 0 {
        show_error_at(0xa17ba0, 0x172);
        return 0;
    }
    let vaga = *j(8);
    if vaga != -1 {
        if buscar(f, vaga.wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
            show_error_at(0xa17ba0, 0x17f);
            fechar(f);
            return 0;
        }
    } else {
        let b = balde(estat, jogo, r) as i8 as isize;
        let cursor = d(8 + 4 * b as usize);
        if buscar(f, (*cursor).wrapping_mul(MATCH_STATS_RECORD_SIZE as i32), 0) != 0 {
            show_error_at(0xa17ba0, 0x18d);
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
    if escrever(r, MATCH_STATS_RECORD_SIZE, 1, f) != 1 {
        show_error_at(0xa17ba0, 0x1a3);
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
unsafe extern "thiscall" fn comparar_read_match_record(estat: usize, k: i32, saida: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(%%I0%%)(estat, k, saida)
}

unsafe extern "thiscall" fn comparar_match_screens_record_match(estat: usize, m: usize, r: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(estat, m, r)
}

unsafe extern "thiscall" fn comparar_write_match_record(estat: usize, jogo: usize, r: usize, eventos: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%)(estat, jogo, r, eventos)
}
