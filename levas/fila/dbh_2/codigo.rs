// ---------- %%LEVA%%a leva: a gravação dos .dat: competições de pessoal, árbitros, cores (Db_files.cpp) ----------

/// { 0x550470 } (cdecl, Db_files.cpp): `db_files_write_staff_comps(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 101 bytes (staff_comp.dat, id 11) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar competições de pessoal");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 101, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 101, n, stream)
    };
    (escritos == n) as i32 // gravar competições de pessoal
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// { 0x5506b0 } (cdecl, Db_files.cpp): `db_files_write_officials(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 43 bytes (officials.dat, id 7) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_arbitros(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar árbitros");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 43, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 43, n, stream)
    };
    (escritos == n) as i32 // gravar árbitros
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_arbitros(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I1%%)(stream, use_zip, registros, n)
}

/// { 0x550800 } (cdecl, Db_files.cpp): `db_files_write_records_0x3a(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 58 bytes (colour.dat, id 2, e staff_comp_history.dat, id 18: a mesma rotina para os dois) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_registros_58(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar registros de 58 bytes");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 58, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 58, n, stream)
    };
    (escritos == n) as i32 // gravar registros de 58 bytes
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_registros_58(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I2%%)(stream, use_zip, registros, n)
}
