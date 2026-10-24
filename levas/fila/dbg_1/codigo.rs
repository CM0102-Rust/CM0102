// ---------- %%LEVA%%a leva: a gravação dos .dat: continentes, nações, cidades (Db_files.cpp) ----------

/// { 0x54d6e0 } (cdecl, Db_files.cpp): `db_files_write_continents(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 198 bytes (continent.dat (id 3)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_continentes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar continentes");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 198, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 198, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_continentes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// { 0x54d840 } (cdecl, Db_files.cpp): `db_files_write_nations(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 290 bytes (nation.dat (id 4)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_nacoes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar nações");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 290, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 290, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_nacoes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I1%%)(stream, use_zip, registros, n)
}

/// { 0x54d9a0 } (cdecl, Db_files.cpp): `db_files_write_cities(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 56 bytes (city.dat (id 21)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_cidades(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar cidades");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 56, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 56, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_cidades(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I2%%)(stream, use_zip, registros, n)
}
