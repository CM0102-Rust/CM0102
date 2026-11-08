// ---------- %%LEVA%%a leva: a gravação dos .dat: não jogadores, jogadores, preferências (Db_files.cpp) ----------

/// { 0x54efd0 } (cdecl, Db_files.cpp): `db_files_write_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 68 bytes (dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar não jogadores");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 68, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 68, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// { 0x54fbb0 } (cdecl, Db_files.cpp): `db_files_write_non_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 70 bytes (dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_non_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar jogadores");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 70, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 70, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_non_players(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I1%%)(stream, use_zip, registros, n)
}

/// { 0x54fd00 } (cdecl, Db_files.cpp): `db_files_write_staff_extra(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 52 bytes (das preferências das pessoas (id 22)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_staff_extra(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar preferências");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 52, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 52, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_staff_extra(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I2%%)(stream, use_zip, registros, n)
}
