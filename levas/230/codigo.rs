// ---------- 230a leva: a gravação dos .dat: continentes, nações, cidades (Db_files.cpp) ----------

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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(643)(stream, use_zip, registros, n)
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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(644)(stream, use_zip, registros, n)
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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(645)(stream, use_zip, registros, n)
}

// ---------- 230a leva: a gravação dos .dat: estádios, clubes, pessoas 110 (Db_files.cpp) ----------

/// { 0x54daf0 } (cdecl, Db_files.cpp): `db_files_write_stadiums(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 78 bytes (stadium.dat) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_estadios(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar estádios");
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
unsafe extern "C" fn comparar_gravar_estadios(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(646)(stream, use_zip, registros, n)
}

/// { 0x54e5d0 } (cdecl, Db_files.cpp): `db_files_write_clubs(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 581 bytes (club.dat e nat_club.dat (ids 0, 1)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_clubes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar clubes");
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
unsafe extern "C" fn comparar_gravar_clubes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(647)(stream, use_zip, registros, n)
}

/// { 0x54e720 } (cdecl, Db_files.cpp): `db_files_write_staff(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 110 bytes (staff.dat (pessoas, 110 bytes; id 6)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_pessoas_110(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar pessoas 110");
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
unsafe extern "C" fn comparar_gravar_pessoas_110(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(648)(stream, use_zip, registros, n)
}

// ---------- 230a leva: a gravação dos .dat: não jogadores, jogadores, preferências (Db_files.cpp) ----------

/// { 0x54efd0 } (cdecl, Db_files.cpp): `db_files_write_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 68 bytes (dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_nao_jogadores(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_nao_jogadores(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(649)(stream, use_zip, registros, n)
}

/// { 0x54fbb0 } (cdecl, Db_files.cpp): `db_files_write_non_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 70 bytes (dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_jogadores(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_jogadores(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(650)(stream, use_zip, registros, n)
}

/// { 0x54fd00 } (cdecl, Db_files.cpp): `db_files_write_staff_extra(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 52 bytes (das preferências das pessoas (id 22)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_preferencias(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_preferencias(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(651)(stream, use_zip, registros, n)
}
