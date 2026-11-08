// ---------- %%LEVA%%a leva: a leitura dos .dat: históricos de competição, competições de pessoal, histórico de competições de pessoal (Db_files.cpp) ----------

/// { 0x550230 } (cdecl, Db_files.cpp): `db_files_read_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 26 bytes
/// (club_comp_history.dat e nation_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler históricos de competição");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 26, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 26, n, stream)
    };
    (lidos == n) as i32 // históricos de competição
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada)
}

/// { 0x550380 } (cdecl, Db_files.cpp): `db_files_read_staff_comps(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 101 bytes
/// (staff_comp.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_staff_comps(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler competições de pessoal");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 101, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 101, n, stream)
    };
    (lidos == n) as i32 // competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_staff_comps(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada)
}

/// { 0x5504d0 } (cdecl, Db_files.cpp): `db_files_read_staff_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 58 bytes
/// (staff_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn db_files_read_staff_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler histórico de competições de pessoal");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 58, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 58, n, stream)
    };
    (lidos == n) as i32 // histórico de competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_read_staff_comp_history(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada)
}
