// ---------- %%LEVA%%a leva: a leitura dos .dat: continentes, nações, cidades (Db_files.cpp) ----------

/// { 0x54d5e0 } (cdecl, Db_files.cpp): `db_files_read_continents(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 198 bytes
/// (continent.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_continents(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler continentes");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 198, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 198, n, stream)
    };
    (lidos == n) as i32 // continentes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_continents(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada)
}

/// { 0x54d740 } (cdecl, Db_files.cpp): `db_files_read_nations(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 290 bytes
/// (nation.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_nations(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler nações");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 290, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 290, n, stream)
    };
    (lidos == n) as i32 // nações
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_nations(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada)
}

/// { 0x54d8a0 } (cdecl, Db_files.cpp): `db_files_read_cities(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) de 1 a 2 (com sinal), lê de uma vez a quantidade (+0x37) de registros de 56 bytes
/// (city.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_cities(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao <= 0 || versao > 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada);
    }
    crate::comparison::branch("banco ler cidades");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 56, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 56, n, stream)
    };
    (lidos == n) as i32 // cidades
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_cities(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada)
}
