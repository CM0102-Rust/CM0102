// ---------- 231a leva: a gravação dos .dat: histórico de pessoas, competições, históricos de competição (Db_files.cpp) ----------

/// { 0x54fe50 } (cdecl, Db_files.cpp): `db_files_write_staff_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 17 bytes (staff_history.dat, id 17) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(652)(stream, use_zip, registros, n)
}

/// { 0x5501d0 } (cdecl, Db_files.cpp): `db_files_write_comps(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 107 bytes (club_comp.dat em duas chamadas, as competições e as 127 de reserva, e nation_comp.dat; ids 12 e 16) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_competicoes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_competicoes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(653)(stream, use_zip, registros, n)
}

/// { 0x550320 } (cdecl, Db_files.cpp): `db_files_write_comp_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 26 bytes (club_comp_history.dat e nation_comp_history.dat, ids 19 e 20) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_historicos_de_competicao(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_gravar_historicos_de_competicao(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(654)(stream, use_zip, registros, n)
}

// ---------- 231a leva: a gravação dos .dat: competições de pessoal, árbitros, cores (Db_files.cpp) ----------

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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(655)(stream, use_zip, registros, n)
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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(656)(stream, use_zip, registros, n)
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
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(657)(stream, use_zip, registros, n)
}

// ---------- 231a leva: a gravação dos .dat: nomes; abrir e criar os .dat (Db_files.cpp) ----------
/// { 0x550950 } (cdecl, Db_files.cpp): `db_files_write_or_read_records_0x3c(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 60 bytes (first_names.dat, second_names.dat e common_names.dat, ids 13 a 15: a mesma rotina para os três) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_nomes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar nomes");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
        zwrite(registros, 60, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
        fwrite(registros, 60, n, stream)
    };
    (escritos == n) as i32 // gravar nomes
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// ponytail: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_nomes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(658)(stream, use_zip, registros, n)
}

/// O caminho de um .dat: `_makepath(destino, 0, pasta, nome, 0)` (0x945b13 da CRT do jogo): a pasta, a barra
/// se faltar e o nome. Sem efeito fora de `destino`: o comparador de abrir/criar refaz o caminho num buffer
/// próprio.
unsafe fn caminho_do_banco(destino: usize, pasta: usize, nome: usize) {
    let juntar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x945b13usize);
    juntar(destino, 0, pasta, nome, 0);
}

/// Compara o caminho que a original montou em 0xde1f64 com o da cópia (num buffer próprio).
unsafe fn conferir_caminho_do_banco(rotina: &str, pasta: usize, nome: usize) {
    let mut c = [0u8; 0x400];
    caminho_do_banco(c.as_mut_ptr() as usize, pasta, nome);
    let o = core::slice::from_raw_parts(0xde1f64 as *const u8, 0x400);
    let fim = |s: &[u8]| s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let (a, b) = (&o[..fim(o)], &c[..fim(&c)]);
    PLACAR.registrar(a == b, || format!("{} caminho {:?} x {:?}", rotina, String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// { 0x54d580 } (cdecl, Db_files.cpp): `db_files_open_for_read(pasta, nome)`: monta o caminho em 0xde1f64
/// (`_makepath`, ver `caminho_do_banco`) e devolve `fopen(0xde1f64, "rb")` (0x945b00; "rb" em 0x98f928). Na
/// criação do jogo, `load_database` abre assim o index.dat e cada tabela da pasta Data.
unsafe extern "C" fn abrir_arquivo_do_banco(pasta: usize, nome: usize) -> usize {
    crate::comparacao::ramo("banco abrir .dat");
    caminho_do_banco(0xde1f64, pasta, nome); // abrir (rb)
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    fopen(0xde1f64, 0x98f928)
}

/// Comparador: abrir duas vezes deixaria outro FILE aberto (ou alocaria um lugar novo na tabela de FILE da
/// CRT, que fica). Roda a original e compara só o caminho (sem efeito colateral).
unsafe extern "C" fn comparar_abrir_arquivo_do_banco(pasta: usize, nome: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> usize>(659)(pasta, nome);
    conferir_caminho_do_banco("abrir_arquivo_do_banco", pasta, nome);
    r
}

/// { 0x54d5b0 } (cdecl, Db_files.cpp): `db_files_open_for_write(pasta, nome)`: o mesmo caminho em 0xde1f64 e
/// `fopen(0xde1f64, "wb")` ("wb" em 0x9a7f28). Só `save_database(…, 0, …)` (gravar a base na pasta) chama:
/// no save (use_zip ≠ 0) as tabelas vão por `zopen` 0x9326e0.
unsafe extern "C" fn criar_arquivo_do_banco(pasta: usize, nome: usize) -> usize {
    crate::comparacao::ramo("banco criar .dat");
    caminho_do_banco(0xde1f64, pasta, nome);
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    fopen(0xde1f64, 0x9a7f28) // criar (wb)
}

/// Comparador: criar duas vezes truncaria o arquivo com dois FILE abertos. Roda a original e compara só o
/// caminho.
unsafe extern "C" fn comparar_criar_arquivo_do_banco(pasta: usize, nome: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> usize>(660)(pasta, nome);
    conferir_caminho_do_banco("criar_arquivo_do_banco", pasta, nome);
    r
}