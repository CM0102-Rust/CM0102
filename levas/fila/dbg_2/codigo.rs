// ---------- %%LEVA%%a leva: a gravação dos .dat: estádios, clubes, pessoas 110 (Db_files.cpp) ----------

/// { 0x54daf0 } (cdecl, Db_files.cpp): `db_files_write_stadiums(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 78 bytes (stadium.dat) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_stadiums(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco gravar estádios");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 78, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 78, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_db_files_write_stadiums(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// { 0x54e5d0 } (cdecl, Db_files.cpp): `db_files_write_clubs(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 581 bytes (club.dat e nat_club.dat (ids 0, 1)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_clubs(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco gravar clubes");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 581, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 581, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_db_files_write_clubs(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I1%%)(stream, use_zip, registros, n)
}

/// { 0x54e720 } (cdecl, Db_files.cpp): `db_files_write_staff(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 110 bytes (staff.dat (pessoas, 110 bytes; id 6)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn db_files_write_staff(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco gravar pessoas 110");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 110, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 110, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn compare_db_files_write_staff(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I2%%)(stream, use_zip, registros, n)
}
