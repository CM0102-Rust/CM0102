// ---------- %%LEVA%%a leva: a leitura dos .dat: estádios, pessoas 157, histórico de pessoas (Db_files.cpp) ----------

/// { 0x54da00 } (cdecl, Db_files.cpp): `db_files_read_stadiums(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 78 bytes
/// (stadium.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_estadios(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler estádios");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 78, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 78, n, stream)
    };
    (lidos == n) as i32 // estádios
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_estadios(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada)
}

/// { 0x54e780 } (cdecl, Db_files.cpp): `db_files_read_staff_v1(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 157 bytes
/// (staff.dat (pessoas, versão de arquivo de 157 bytes)) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_pessoas_157(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler pessoas 157");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 157, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 157, n, stream)
    };
    (lidos == n) as i32 // pessoas 157
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_pessoas_157(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada)
}

/// { 0x54fd60 } (cdecl, Db_files.cpp): `db_files_read_staff_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 17 bytes
/// (staff_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler histórico de pessoas");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 17, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 17, n, stream)
    };
    (lidos == n) as i32 // histórico de pessoas
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada)
}
