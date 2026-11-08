// ---------- %%LEVA%%a leva: a gravação dos .dat: nomes; abrir e criar os .dat (Db_files.cpp) ----------
/// { 0x550950 } (cdecl, Db_files.cpp): `db_files_write_or_read_records_0x3c(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 60 bytes (first_names.dat, second_names.dat e common_names.dat, ids 13 a 15: a mesma rotina para os três) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
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
unsafe extern "C" fn comparar_db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(%%I0%%)(stream, use_zip, registros, n)
}

/// O caminho de um .dat: `_makepath(destino, 0, pasta, nome, 0)` (0x945b13 da CRT do jogo): a pasta, a barra
/// se faltar e o nome. Sem efeito fora de `destino`: o comparador de abrir/criar refaz o caminho num buffer
/// próprio.
unsafe fn database_path(destino: usize, pasta: usize, nome: usize) {
    let juntar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x945b13usize);
    juntar(destino, 0, pasta, nome, 0);
}

/// Compara o caminho que a original montou em 0xde1f64 com o da cópia (num buffer próprio).
unsafe fn check_database_path(rotina: &str, pasta: usize, nome: usize) {
    let mut c = [0u8; 0x400];
    database_path(c.as_mut_ptr() as usize, pasta, nome);
    let o = core::slice::from_raw_parts(0xde1f64 as *const u8, 0x400);
    let fim = |s: &[u8]| s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let (a, b) = (&o[..fim(o)], &c[..fim(&c)]);
    PLACAR.registrar(a == b, || format!("{} caminho {:?} x {:?}", rotina, String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// { 0x54d580 } (cdecl, Db_files.cpp): `db_files_open_for_read(pasta, nome)`: monta o caminho em 0xde1f64
/// (`_makepath`, ver `database_path`) e devolve `fopen(0xde1f64, "rb")` (0x945b00; "rb" em 0x98f928). Na
/// criação do jogo, `load_database` abre assim o index.dat e cada tabela da pasta Data.
unsafe extern "C" fn db_files_open_for_read(pasta: usize, nome: usize) -> usize {
    crate::comparacao::ramo("banco abrir .dat");
    database_path(0xde1f64, pasta, nome); // abrir (rb)
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    fopen(0xde1f64, 0x98f928)
}

/// Comparador: abrir duas vezes deixaria outro FILE aberto (ou alocaria um lugar novo na tabela de FILE da
/// CRT, que fica). Roda a original e compara só o caminho (sem efeito colateral).
unsafe extern "C" fn comparar_db_files_open_for_read(pasta: usize, nome: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I1%%)(pasta, nome);
    check_database_path("db_files_open_for_read", pasta, nome);
    r
}

/// { 0x54d5b0 } (cdecl, Db_files.cpp): `db_files_open_for_write(pasta, nome)`: o mesmo caminho em 0xde1f64 e
/// `fopen(0xde1f64, "wb")` ("wb" em 0x9a7f28). Só `save_database(…, 0, …)` (gravar a base na pasta) chama:
/// no save (use_zip ≠ 0) as tabelas vão por `zopen` 0x9326e0.
unsafe extern "C" fn db_files_open_for_write(pasta: usize, nome: usize) -> usize {
    crate::comparacao::ramo("banco criar .dat");
    database_path(0xde1f64, pasta, nome);
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    fopen(0xde1f64, 0x9a7f28) // criar (wb)
}

/// Comparador: criar duas vezes truncaria o arquivo com dois FILE abertos. Roda a original e compara só o
/// caminho.
unsafe extern "C" fn comparar_db_files_open_for_write(pasta: usize, nome: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I2%%)(pasta, nome);
    check_database_path("db_files_open_for_write", pasta, nome);
    r
}