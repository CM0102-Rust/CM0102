// ---------- %%LEVA%%a leva: a gravação dos .dat: histórico de pessoas, competições, históricos de competição (Db_files.cpp) ----------

/// { 0x54fe50 } (cdecl, Db_files.cpp): `db_files_write_staff_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 17 bytes (staff_history.dat, id 17) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn db_files_write_staff_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar histórico de pessoas");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 17, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 17, n, stream)
    };
    (escritos == n) as i32 // gravar histórico de pessoas
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_staff_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// { 0x5501d0 } (cdecl, Db_files.cpp): `db_files_write_comps(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 107 bytes (club_comp.dat em duas chamadas, as competições e as 127 de reserva, e nation_comp.dat; ids 12 e 16) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn db_files_write_comps(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar competições");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 107, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 107, n, stream)
    };
    (escritos == n) as i32 // gravar competições
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_comps(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I1%%)(stream, use_zip, registros, n)
}

/// { 0x550320 } (cdecl, Db_files.cpp): `db_files_write_comp_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 26 bytes (club_comp_history.dat e nation_comp_history.dat, ids 19 e 20) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn db_files_write_comp_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar históricos de competição");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 26, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 26, n, stream)
    };
    (escritos == n) as i32 // gravar históricos de competição
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_db_files_write_comp_history(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I2%%)(stream, use_zip, registros, n)
}
