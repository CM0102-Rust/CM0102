// ---------- %%LEVA%%a leva: a leitura dos .dat: jogadores, pessoas 110, preferências (Db_files.cpp) ----------

/// { 0x54f030 } (cdecl, Db_files.cpp): `db_files_read_non_players(stream, use_zip, registros, entrada)` (o mapa
/// da comunidade troca os nomes: é a tabela id 10, a dos jogadores, 0xae23c4): com a versão da entrada do
/// index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 70 bytes por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Versão 1: o conversor campo a
/// campo 0x54f150 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta dele. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa.
unsafe extern "C" fn db_files_read_non_players(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao == 1 {
        crate::comparison::branch("banco ler jogadores versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(0x54f150usize);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler jogadores");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 70, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 70, n, stream)
    };
    (lidos == n) as i32 // jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_db_files_read_non_players(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada)
}

/// { 0x54e630 } (cdecl, Db_files.cpp): `db_files_read_staff(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 110 bytes
/// (staff.dat na versão 2: o save e as bases da comunidade; a base de fábrica é a versão 1, 0x54e780) por
/// `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_staff(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler pessoas 110");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 110, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 110, n, stream)
    };
    (lidos == n) as i32 // pessoas 110
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.
unsafe extern "C" fn compare_db_files_read_staff(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada)
}

/// { 0x54fc10 } (cdecl, Db_files.cpp): `db_files_read_staff_extra(stream, use_zip, registros, entrada)`: com a
/// versão da entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 52 bytes
/// (as preferências, id 22: o save e as bases da comunidade; na base de fábrica elas saem da conversão das
/// pessoas de 157 bytes) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se
/// veio tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O
/// quadro de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_staff_extra(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler preferências");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 52, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 52, n, stream)
    };
    (lidos == n) as i32 // preferências
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.
unsafe extern "C" fn compare_db_files_read_staff_extra(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada)
}
