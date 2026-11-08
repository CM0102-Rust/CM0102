use super::*;

/// Version-1 database converter: reads each of the n records field by field (zread or fread), turning byte reputations
/// into u16; returns 0 on the first short read, else 1.
///
/// [pt] Um conversor da versão 1 (Db_files.cpp): para cada um dos `n` registros de `tamanho` bytes, os campos na
/// ordem do arquivo, `(deslocamento, tamanho, quantos)`, cada um lido sozinho (`tamanho`, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; tamanho 0 = um byte do arquivo que vira u16 (`reputacao`) em
/// deslocamento + 2k (a original lê o byte num argumento da própria pilha, já usado, e grava o u16 antes da
/// leitura seguinte). A primeira leitura que não vem dá 0 (o que já veio fica no registro); tudo, 1. Os
/// bytes que o arquivo não traz (buracos do registro) ficam como estavam.
pub(crate) unsafe fn convert_v1_records(stream: usize, use_zip: i32, records: usize, n: u32, size: usize,
                       campos: &[(usize, u32, usize)], injury_word_0c: fn(u8) -> u16) -> i32 {
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = records + i * size;
        for &(o, t, q) in campos {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if read(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o + 2 * k) as *mut u16).write_unaligned(injury_word_0c(b));
                } else if read(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Bytes of one version-1 record in the file, from its field list.
///
/// [pt] Bytes de um registro da versão 1 no arquivo.
pub(crate) fn v1_record_size(campos: &[(usize, u32, usize)]) -> usize {
    campos.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum()
}

/// TClub (581 bytes) no club.dat versão 1 (577 bytes no arquivo): buracos em +0x37, +0x52 e +0x244; a
/// reputação vem num byte (0 vira 1) × 500 em +0x80.
pub(crate) const CLUB_V1_LAYOUT: [(usize, u32, usize); 18] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 3), (0x5f, 1, 1),
    (0x60, 4, 1), (0x64, 1, 1), (0x65, 4, 2), (0x6d, 1, 1), (0x6e, 4, 1), (0x72, 1, 1), (0x73, 4, 3), (0x7f, 1, 1),
    (0x80, 0, 1), (0x82, 1, 1), (0x83, 4, 87), (0x1df, 1, 1), (0x1e0, 4, 25)];

/// Não jogador (68 bytes, id 9) versão 1 (65 no arquivo): as três reputações (+8, +0xa, +0xc) vêm em bytes.
pub(crate) const NON_PLAYER_V1_LAYOUT: [(usize, u32, usize); 6] = [(0, 4, 1), (4, 2, 2), (8, 0, 3), (0xe, 1, 21), (0x23, 4, 8), (0x43, 1, 1)];

/// Jogador (70 bytes, id 10) versão 1 (67 no arquivo): as três reputações (+9, +0xb, +0xd) vêm em bytes.
pub(crate) const PLAYER_V1_FIELDS: [(usize, u32, usize); 5] = [(0, 4, 1), (4, 1, 1), (5, 2, 2), (9, 0, 3), (0xf, 1, 55)];

/// reads version 1 club.dat field by field (577 bytes on file for 581-byte records, holes at +0x37, +0x52 and +0x244),
/// the reputation in one byte (0 becomes 1) × 500; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_clubs_v1` (0x54dc90, Db_files.cpp, cdecl).
///
/// [pt] { 0x54dc90 } (cdecl, Db_files.cpp): `db_files_read_clubs_v1(stream, use_zip, registros, n)`: o club.dat
/// versão 1 campo a campo (`CLUB_V1_LAYOUT`); a reputação lida num byte, 0 vira 1, × 500 (u16, como o `imul ax`).
/// Chamado por `db_files_read_clubs` 0x54db50 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
pub(crate) unsafe extern "C" fn db_files_read_clubs_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler clubes v1");
    convert_v1_records(stream, use_zip, records, n, 581, &CLUB_V1_LAYOUT, |b| (b.max(1) as u16).wrapping_mul(500))
}

/// reads the version 1 non-player table (id 9; the community map calls it players) field by field (65 bytes on file for
/// 68-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_players_v1` (0x54e9a0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e9a0 } (cdecl, Db_files.cpp): `db_files_read_players_v1(stream, use_zip, registros, n)` (o mapa da
/// comunidade troca os nomes: é a tabela id 9, a dos não jogadores, 68 bytes): campo a campo
/// (`NON_PLAYER_V1_LAYOUT`); as reputações lidas em bytes, sem conta. Chamado por 0x54e880 com a versão 1.
pub(crate) unsafe extern "C" fn db_files_read_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler não jogadores v1");
    convert_v1_records(stream, use_zip, records, n, 68, &NON_PLAYER_V1_LAYOUT, |b| b as u16)
}

/// reads the version 1 player table (id 10; the community map calls it non_players) field by field (67 bytes on file
/// for 70-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_non_players_v1` (0x54f150, Db_files.cpp, cdecl).
///
/// [pt] { 0x54f150 } (cdecl, Db_files.cpp): `db_files_read_non_players_v1(stream, use_zip, registros, n)` (o mapa
/// troca os nomes: é a tabela id 10, a dos jogadores, 70 bytes): campo a campo (`PLAYER_V1_FIELDS`); as reputações
/// lidas em bytes, sem conta. Chamado por 0x54f030 com a versão 1.
pub(crate) unsafe extern "C" fn db_files_read_non_players_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler jogadores v1");
    convert_v1_records(stream, use_zip, records, n, 70, &PLAYER_V1_FIELDS, |b| b as u16)
}

pub(crate) type V1Converter = unsafe extern "C" fn(usize, i32, usize, u32) -> i32;

/// Comparador de um conversor: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à
/// posição do `ftell` 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo
/// parou. Fica a posição da Rust (a mesma, se iguais).
// simplification: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.




/// Autoteste dos conversores da versão 1 (CM_AUTOTESTE=1, modo comparação: a base de fábrica e o save são
/// versão 2 e a carreira não passa por eles). Para cada conversor, 64 casos de sementes fixas: de 1 a 6
/// registros, um arquivo temporário (`autoteste_banco_v1.tmp` na pasta do jogo, apagado no fim) com bytes
/// sorteados (1 caso em 4 só zeros: a reputação 0 do clube), inteiro (1 em 3) ou cortado num ponto sorteado
/// (a leitura falha no meio de um campo), aberto pelo `fopen` 0x945b00 do jogo; o buffer começa sorteado (os
/// buracos do registro entram na comparação).
pub unsafe fn autoteste_dbj_1() {
    const ARQ: &str = "autoteste_banco_v1.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x5631_4442_4a31);
    let casos: [(V1Converter, &[(usize, u32, usize)], usize); 3] = [(comparar_db_files_read_clubs_v1, &CLUB_V1_LAYOUT, 581),
        (comparar_db_files_read_players_v1, &NON_PLAYER_V1_LAYOUT, 68), (comparar_db_files_read_non_players_v1, &PLAYER_V1_FIELDS, 70)];
    for (comparar, campos, tamanho) in casos {
        for caso in 0..64u64 {
            let n = 1 + (s.proximo() % 6) as usize;
            let total = n * v1_record_size(campos);
            let corte = if caso % 3 == 0 { total } else { (s.proximo() % (total as u64 + 1)) as usize };
            let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.proximo() as u8 }).collect();
            if std::fs::write(ARQ, &bytes).is_err() {
                continue;
            }
            let f = fopen(b"autoteste_banco_v1.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
            if f == 0 {
                continue;
            }
            let mut buf: Vec<u8> = (0..n * tamanho).map(|_| s.proximo() as u8).collect();
            comparar(f, 0, buf.as_mut_ptr() as usize, n as u32);
            fclose(f);
        }
    }
    let _ = std::fs::remove_file(ARQ);
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco v1 (conversores): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

/// writes n 17-byte records (staff_history.dat, id 17) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything went
/// Original: `db_files_write_staff_history` (0x54fe50, Db_files.cpp, cdecl).
///
/// [pt] { 0x54fe50 } (cdecl, Db_files.cpp): `db_files_write_staff_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 17 bytes (staff_history.dat, id 17) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_staff_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar histórico de pessoas");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 17, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 17, n, stream)
    };
    (escritos == n) as i32 // gravar histórico de pessoas
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 107-byte records (club_comp.dat, in two calls: the competitions and the 127 reserve ones; nation_comp.dat)
/// in one go, through zwrite (save) or fwrite (Data folder); 1 if everything went
/// Original: `db_files_write_comps` (0x5501d0, Db_files.cpp, cdecl).
///
/// [pt] { 0x5501d0 } (cdecl, Db_files.cpp): `db_files_write_comps(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 107 bytes (club_comp.dat em duas chamadas, as competições e as 127 de reserva, e nation_comp.dat; ids 12 e 16) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar competições");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 107, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 107, n, stream)
    };
    (escritos == n) as i32 // gravar competições
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 26-byte records (club_comp_history.dat and nation_comp_history.dat, ids 19 and 20) in one go, through
/// zwrite (save) or fwrite (Data folder); 1 if everything went
/// Original: `db_files_write_comp_history` (0x550320, Db_files.cpp, cdecl).
///
/// [pt] { 0x550320 } (cdecl, Db_files.cpp): `db_files_write_comp_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 26 bytes (club_comp_history.dat e nation_comp_history.dat, ids 19 e 20) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_comp_history(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar históricos de competição");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 26, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 26, n, stream)
    };
    (escritos == n) as i32 // gravar históricos de competição
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 101-byte records (staff_comp.dat, id 11) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything went
/// Original: `db_files_write_staff_comps` (0x550470, Db_files.cpp, cdecl).
///
/// [pt] { 0x550470 } (cdecl, Db_files.cpp): `db_files_write_staff_comps(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 101 bytes (staff_comp.dat, id 11) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_staff_comps(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar competições de pessoal");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 101, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 101, n, stream)
    };
    (escritos == n) as i32 // gravar competições de pessoal
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 43-byte records (officials.dat, id 7) in one go, through zwrite (save) or fwrite (Data folder); 1 if
/// everything went
/// Original: `db_files_write_officials` (0x5506b0, Db_files.cpp, cdecl).
///
/// [pt] { 0x5506b0 } (cdecl, Db_files.cpp): `db_files_write_officials(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 43 bytes (officials.dat, id 7) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_officials(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar árbitros");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 43, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 43, n, stream)
    };
    (escritos == n) as i32 // gravar árbitros
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 58-byte records (colour.dat, id 2, and staff_comp_history.dat, id 18: the same routine for both) in one go,
/// through zwrite (save) or fwrite (Data folder); 1 if everything went
/// Original: `db_files_write_records_0x3a` (0x550800, Db_files.cpp, cdecl).
///
/// [pt] { 0x550800 } (cdecl, Db_files.cpp): `db_files_write_records_0x3a(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 58 bytes (colour.dat, id 2, e staff_comp_history.dat, id 18: a mesma rotina para os dois) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_records_0x3a(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar registros de 58 bytes");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 58, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 58, n, stream)
    };
    (escritos == n) as i32 // gravar registros de 58 bytes
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// writes n 60-byte records (first_names.dat, second_names.dat and common_names.dat, ids 13 to 15) in one go, through
/// zwrite (save) or fwrite (Data folder); 1 if everything went (it only writes, despite the community name)
/// Original: `db_files_write_or_read_records_0x3c` (0x550950, Db_files.cpp, cdecl).
///
/// [pt] { 0x550950 } (cdecl, Db_files.cpp): `db_files_write_or_read_records_0x3c(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 60 bytes (first_names.dat, second_names.dat e common_names.dat, ids 13 a 15: a mesma rotina para os três) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
pub(crate) unsafe extern "C" fn db_files_write_or_read_records_0x3c(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar nomes");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 60, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 60, n, stream)
    };
    (escritos == n) as i32 // gravar nomes
}

/// Comparador: gravar duas vezes duplicaria os registros no arquivo.
// simplification: só a original; a prova é o .qst (a gravação do save) e o ramo da cópia no modo Rust.

/// Builds a .dat path with _makepath(dest, 0, folder, name, 0) (FN_CRT_MAKEPATH).
///
/// [pt] O caminho de um .dat: `_makepath(destino, 0, pasta, nome, 0)` (0x945b13 da CRT do jogo): a pasta, a barra
/// se faltar e o nome. Sem efeito fora de `destino`: o comparador de abrir/criar refaz o caminho num buffer
/// próprio.
pub(crate) unsafe fn database_path(target: usize, folder: usize, name: usize) {
    let join: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    join(target, 0, folder, name, 0);
}

/// Compares the path the original built at 0xde1f64 with the copy's, rebuilt in a local buffer.
///
/// [pt] Compara o caminho que a original montou em 0xde1f64 com o da cópia (num buffer próprio).
pub(crate) unsafe fn check_database_path(routine: &str, folder: usize, name: usize) {
    let mut c = [0u8; 0x400];
    database_path(c.as_mut_ptr() as usize, folder, name);
    let o = core::slice::from_raw_parts(0xde1f64 as *const u8, 0x400);
    let end = |s: &[u8]| s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let (a, b) = (&o[..end(o)], &c[..end(&c)]);
    PLACAR.registrar(a == b, || format!("{} caminho {:?} x {:?}", routine, String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "rb"); the comparator checks the path
/// Original: `db_files_open_for_read` (0x54d580, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d580 } (cdecl, Db_files.cpp): `db_files_open_for_read(pasta, nome)`: monta o caminho em 0xde1f64
/// (`_makepath`, ver `database_path`) e devolve `fopen(0xde1f64, "rb")` (0x945b00; "rb" em 0x98f928). Na
/// criação do jogo, `load_database` abre assim o index.dat e cada tabela da pasta Data.
pub(crate) unsafe extern "C" fn db_files_open_for_read(folder: usize, name: usize) -> usize {
    crate::comparacao::ramo("banco abrir .dat");
    database_path(0xde1f64, folder, name); // abrir (rb)
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    fopen(0xde1f64, 0x98f928)
}


/// builds folder + name at 0xde1f64 (_makepath) and returns fopen(path, "wb"); only when writing the database to the
/// folder; the comparator checks the path
/// Original: `db_files_open_for_write` (0x54d5b0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d5b0 } (cdecl, Db_files.cpp): `db_files_open_for_write(pasta, nome)`: o mesmo caminho em 0xde1f64 e
/// `fopen(0xde1f64, "wb")` ("wb" em 0x9a7f28). Só `save_database(…, 0, …)` (gravar a base na pasta) chama:
/// no save (use_zip ≠ 0) as tabelas vão por `zopen` 0x9326e0.
pub(crate) unsafe extern "C" fn db_files_open_for_write(folder: usize, name: usize) -> usize {
    crate::comparacao::ramo("banco criar .dat");
    database_path(0xde1f64, folder, name);
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    fopen(0xde1f64, 0x9a7f28) // criar (wb)
}


/// writes the continent.dat (id 3) table of 198-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything went out
/// Original: `db_files_write_continents` (0x54d6e0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d6e0 } (cdecl, Db_files.cpp): `db_files_write_continents(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 198 bytes (continent.dat (id 3)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_continents(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar continentes");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 198, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 198, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the nation.dat (id 4) table of 290-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything went out
/// Original: `db_files_write_nations` (0x54d840, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d840 } (cdecl, Db_files.cpp): `db_files_write_nations(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 290 bytes (nation.dat (id 4)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_nations(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar nações");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 290, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 290, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the city.dat (id 21) table of 56-byte records in one go, through zwrite (save) or fwrite (Data); 1 if
/// everything went out
/// Original: `db_files_write_cities` (0x54d9a0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d9a0 } (cdecl, Db_files.cpp): `db_files_write_cities(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 56 bytes (city.dat (id 21)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_cities(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar cidades");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 56, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 56, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the stadium.dat table of 78-byte records in one go, through zwrite (save) or fwrite (Data); 1 if everything
/// went out
/// Original: `db_files_write_stadiums` (0x54daf0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54daf0 } (cdecl, Db_files.cpp): `db_files_write_stadiums(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 78 bytes (stadium.dat) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_stadiums(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar estádios");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 78, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 78, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the club.dat and nat_club.dat (ids 0, 1) table of 581-byte records in one go, through zwrite (save) or fwrite
/// (Data); 1 if everything went out
/// Original: `db_files_write_clubs` (0x54e5d0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e5d0 } (cdecl, Db_files.cpp): `db_files_write_clubs(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 581 bytes (club.dat e nat_club.dat (ids 0, 1)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_clubs(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar clubes");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 581, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 581, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the staff.dat (staff, 110 bytes; id 6) table of 110-byte records in one go, through zwrite (save) or fwrite
/// (Data); 1 if everything went out
/// Original: `db_files_write_staff` (0x54e720, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e720 } (cdecl, Db_files.cpp): `db_files_write_staff(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 110 bytes (staff.dat (pessoas, 110 bytes; id 6)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_staff(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar pessoas 110");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 110, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 110, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the staff.dat non-player (id 9; the community map calls it players) table of 68-byte records in one go,
/// through zwrite (save) or fwrite (Data); 1 if everything went out
/// Original: `db_files_write_players` (0x54efd0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54efd0 } (cdecl, Db_files.cpp): `db_files_write_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 68 bytes (dos não jogadores do staff.dat (id 9; o mapa da comunidade a chama de players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar não jogadores");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 68, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 68, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the staff.dat player (id 10; the community map calls it non_players) table of 70-byte records in one go,
/// through zwrite (save) or fwrite (Data); 1 if everything went out
/// Original: `db_files_write_non_players` (0x54fbb0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54fbb0 } (cdecl, Db_files.cpp): `db_files_write_non_players(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 70 bytes (dos jogadores do staff.dat (id 10; o mapa da comunidade a chama de non_players)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_non_players(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar jogadores");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 70, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 70, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// writes the staff preferences (id 22) table of 52-byte records in one go, through zwrite (save) or fwrite (Data); 1
/// if everything went out
/// Original: `db_files_write_staff_extra` (0x54fd00, Db_files.cpp, cdecl).
///
/// [pt] { 0x54fd00 } (cdecl, Db_files.cpp): `db_files_write_staff_extra(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 52 bytes (das preferências das pessoas (id 22)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
pub(crate) unsafe extern "C" fn db_files_write_staff_extra(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar preferências");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(records, 52, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(records, 52, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// simplification: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.

/// reads the officials.dat table (id 7) of 43-byte records in one go, through zread (save) or fread (Data), with the
/// entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_officials` (0x5505c0, Db_files.cpp, cdecl).
///
/// [pt] { 0x5505c0 } (cdecl, Db_files.cpp): `db_files_read_officials(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 43 bytes
/// (officials.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_officials(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_OFFICIALS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler árbitros");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 43, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 43, n, stream)
    };
    (read == n) as i32 // árbitros
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the colour.dat table (id 2) of 58-byte records in one go, through zread (save) or fread (Data), with the entry
/// version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_colours` (0x550710, Db_files.cpp, cdecl).
///
/// [pt] { 0x550710 } (cdecl, Db_files.cpp): `db_files_read_colours(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 58 bytes
/// (colour.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_colours(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COLOURS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler cores");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 58, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 58, n, stream)
    };
    (read == n) as i32 // cores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the first_names.dat, second_names.dat and common_names.dat tables (ids 13, 14, 15) of 60-byte records in one
/// go, through zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_names` (0x550860, Db_files.cpp, cdecl).
///
/// [pt] { 0x550860 } (cdecl, Db_files.cpp): `db_files_read_names(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 60 bytes
/// (first_names.dat, second_names.dat e common_names.dat: a mesma rotina para os três) por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Outra versão: a caixa "Error"
/// (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao
/// caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_names(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NAMES)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler nomes");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 60, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 60, n, stream)
    };
    (read == n) as i32 // nomes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the club.dat and nat_club.dat table (ids 0, 1) of 581-byte records in one go, through zread (save) or fread
/// (Data), with the entry version 2, and turns reputation 0 into 1; version 1: the 0x54dc90 converter (other: box and
/// exit); 1 if everything came
/// Original: `db_files_read_clubs` (0x54db50, Db_files.cpp, cdecl).
///
/// [pt] { 0x54db50 } (cdecl, Db_files.cpp): `db_files_read_clubs(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 581 bytes
/// (club.dat e nat_club.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data);
/// se não veio tudo, 0; se veio, troca a reputação (+0x80, u16) 0 por 1 em todos e dá 1. Versão 1: o
/// conversor campo a campo 0x54dc90 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta
/// dele (sem a troca da reputação: o conversor faz a sua, ×500). Outra versão: a caixa "Error" (Db_files.cpp)
/// e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao caminho da caixa.
pub(crate) unsafe extern "C" fn db_files_read_clubs(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version == 1 {
        crate::comparacao::ramo("banco ler clubes versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_CLUBS_V1);
        return converter(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CLUBS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler clubes");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 581, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 581, n, stream)
    };
    if read != n {
        return 0;
    }
    for i in 0..n as usize {
        let p = (records + i * 581 + 0x80) as *mut u16;
        if p.read_unaligned() == 0 {
            p.write_unaligned(1);
        }
    }
    1 // clubes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the club_comp.dat and nation_comp.dat table (ids 12, 16) of 107-byte records in one go, through zread (save)
/// or fread (Data), with the entry version 2; version 1: the 0x54ffd0 converter (other: box and exit); 1 if everything
/// came
/// Original: `db_files_read_comps` (0x54feb0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54feb0 } (cdecl, Db_files.cpp): `db_files_read_comps(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 107 bytes
/// (club_comp.dat e nation_comp.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta
/// Data); 1 se veio tudo. Versão 1: o conversor campo a campo 0x54ffd0 (stream, use_zip, registros,
/// quantidade), pelo endereço, e a resposta dele. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do
/// jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao caminho da caixa.
pub(crate) unsafe extern "C" fn db_files_read_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version == 1 {
        crate::comparacao::ramo("banco ler competições versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_COMPS_V1);
        return converter(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMPS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler competições");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 107, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 107, n, stream)
    };
    (read == n) as i32 // competições de clube
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the non-player table of staff.dat (id 9; the community map calls it players) of 68-byte records in one go,
/// through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54e9a0 converter (other: box and
/// exit); 1 if everything came
/// Original: `db_files_read_players` (0x54e880, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e880 } (cdecl, Db_files.cpp): `db_files_read_players(stream, use_zip, registros, entrada)` (o mapa da
/// comunidade troca os nomes: é a tabela id 9, a dos não jogadores, 0xae23c0): com a versão da entrada do
/// index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 68 bytes por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Versão 1: o conversor campo a
/// campo 0x54e9a0 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta dele. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa.
pub(crate) unsafe extern "C" fn db_files_read_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version == 1 {
        crate::comparacao::ramo("banco ler não jogadores versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_PLAYERS_V1);
        return converter(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_PLAYERS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler não jogadores");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 68, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 68, n, stream)
    };
    (read == n) as i32 // não jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the player table of staff.dat (id 10; the community map calls it non_players) of 70-byte records in one go,
/// through zread (save) or fread (Data), with the entry version 2; version 1: the 0x54f150 converter (other: box and
/// exit); 1 if everything came
/// Original: `db_files_read_non_players` (0x54f030, Db_files.cpp, cdecl).
///
/// [pt] { 0x54f030 } (cdecl, Db_files.cpp): `db_files_read_non_players(stream, use_zip, registros, entrada)` (o mapa
/// da comunidade troca os nomes: é a tabela id 10, a dos jogadores, 0xae23c4): com a versão da entrada do
/// index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 70 bytes por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Versão 1: o conversor campo a
/// campo 0x54f150 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta dele. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa.
pub(crate) unsafe extern "C" fn db_files_read_non_players(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version == 1 {
        crate::comparacao::ramo("banco ler jogadores versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_NON_PLAYERS_V1);
        return converter(stream, use_zip, records, n);
    }
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NON_PLAYERS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler jogadores");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 70, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 70, n, stream)
    };
    (read == n) as i32 // jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the staff.dat (staff, 110-byte file version: the save and community databases) table (id 6) of 110-byte
/// records in one go, through zread (save) or fread (Data), with the entry version 2 (other: box and exit); 1 if
/// everything came
/// Original: `db_files_read_staff` (0x54e630, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e630 } (cdecl, Db_files.cpp): `db_files_read_staff(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 110 bytes
/// (staff.dat na versão 2: o save e as bases da comunidade; a base de fábrica é a versão 1, 0x54e780) por
/// `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler pessoas 110");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 110, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 110, n, stream)
    };
    (read == n) as i32 // pessoas 110
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.

/// reads the staff preferences table (id 22: the save and community databases) of 52-byte records in one go, through
/// zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_staff_extra` (0x54fc10, Db_files.cpp, cdecl).
///
/// [pt] { 0x54fc10 } (cdecl, Db_files.cpp): `db_files_read_staff_extra(stream, use_zip, registros, entrada)`: com a
/// versão da entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 52 bytes
/// (as preferências, id 22: o save e as bases da comunidade; na base de fábrica elas saem da conversão das
/// pessoas de 157 bytes) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se
/// veio tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O
/// quadro de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff_extra(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_EXTRA)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler preferências");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 52, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 52, n, stream)
    };
    (read == n) as i32 // preferências
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.

/// reads the continent.dat table (id 3) of 198-byte records in one go, through zread (save) or fread (Data), with the
/// entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_continents` (0x54d5e0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d5e0 } (cdecl, Db_files.cpp): `db_files_read_continents(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 198 bytes
/// (continent.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_continents(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CONTINENTS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler continentes");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 198, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 198, n, stream)
    };
    (read == n) as i32 // continentes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the nation.dat table (id 4) of 290-byte records in one go, through zread (save) or fread (Data), with the
/// entry version 2 (other: box and exit); 1 if everything came
/// Original: `db_files_read_nations` (0x54d740, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d740 } (cdecl, Db_files.cpp): `db_files_read_nations(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 290 bytes
/// (nation.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_nations(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_NATIONS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler nações");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 290, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 290, n, stream)
    };
    (read == n) as i32 // nações
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the city.dat table (id 21) of 56-byte records in one go, through zread (save) or fread (Data), with the entry
/// version 1 to 2 (other: box and exit); 1 if everything came
/// Original: `db_files_read_cities` (0x54d8a0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54d8a0 } (cdecl, Db_files.cpp): `db_files_read_cities(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) de 1 a 2 (com sinal), lê de uma vez a quantidade (+0x37) de registros de 56 bytes
/// (city.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_cities(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version <= 0 || version > 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_CITIES)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler cidades");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 56, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 56, n, stream)
    };
    (read == n) as i32 // cidades
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the stadium.dat table (id 5) of 78-byte records in one go, through zread (save) or fread (Data), with the
/// entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_stadiums` (0x54da00, Db_files.cpp, cdecl).
///
/// [pt] { 0x54da00 } (cdecl, Db_files.cpp): `db_files_read_stadiums(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 78 bytes
/// (stadium.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_stadiums(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STADIUMS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler estádios");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 78, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 78, n, stream)
    };
    (read == n) as i32 // estádios
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the staff.dat (staff, 157-byte file version) table (id 6) of 157-byte records in one go, through zread (save)
/// or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_staff_v1` (0x54e780, Db_files.cpp, cdecl).
///
/// [pt] { 0x54e780 } (cdecl, Db_files.cpp): `db_files_read_staff_v1(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 157 bytes
/// (staff.dat (pessoas, versão de arquivo de 157 bytes)) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff_v1(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_V1)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler pessoas 157");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 157, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 157, n, stream)
    };
    (read == n) as i32 // pessoas 157
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the staff_history.dat table (id 17) of 17-byte records in one go, through zread (save) or fread (Data), with
/// the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_staff_history` (0x54fd60, Db_files.cpp, cdecl).
///
/// [pt] { 0x54fd60 } (cdecl, Db_files.cpp): `db_files_read_staff_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 17 bytes
/// (staff_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_HISTORY)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler histórico de pessoas");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 17, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 17, n, stream)
    };
    (read == n) as i32 // histórico de pessoas
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the club_comp_history.dat and nation_comp_history.dat table (id 19, 20) of 26-byte records in one go, through
/// zread (save) or fread (Data), with the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_comp_history` (0x550230, Db_files.cpp, cdecl).
///
/// [pt] { 0x550230 } (cdecl, Db_files.cpp): `db_files_read_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 26 bytes
/// (club_comp_history.dat e nation_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_COMP_HISTORY)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler históricos de competição");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 26, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 26, n, stream)
    };
    (read == n) as i32 // históricos de competição
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the staff_comp.dat table (id 11) of 101-byte records in one go, through zread (save) or fread (Data), with the
/// entry version 2 (other: box and exit); 1 if everything came
/// Original: `db_files_read_staff_comps` (0x550380, Db_files.cpp, cdecl).
///
/// [pt] { 0x550380 } (cdecl, Db_files.cpp): `db_files_read_staff_comps(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 101 bytes
/// (staff_comp.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff_comps(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 2 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMPS)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler competições de pessoal");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 101, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 101, n, stream)
    };
    (read == n) as i32 // competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads the staff_comp_history.dat table (id 18) of 58-byte records in one go, through zread (save) or fread (Data),
/// with the entry version 1 (other: box and exit); 1 if everything came
/// Original: `db_files_read_staff_comp_history` (0x5504d0, Db_files.cpp, cdecl).
///
/// [pt] { 0x5504d0 } (cdecl, Db_files.cpp): `db_files_read_staff_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 58 bytes
/// (staff_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
pub(crate) unsafe extern "C" fn db_files_read_staff_comp_history(stream: usize, use_zip: i32, records: usize, entry: usize) -> i32 {
    let n = core::ptr::read_unaligned((entry + 0x37) as *const u32);
    let version = core::ptr::read_unaligned((entry + 0x3f) as *const i32);
    if version != 1 {
        return original_fn::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(IDX_DB_FILES_READ_STAFF_COMP_HISTORY)(stream, use_zip, records, entry);
    }
    crate::comparacao::ramo("banco ler histórico de competições de pessoal");
    let read = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(records, 58, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(records, 58, n, stream)
    };
    (read == n) as i32 // histórico de competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// simplification: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.

/// reads version 1 club_comp.dat / nation_comp.dat field by field (104 bytes on file for 107-byte records, holes at
/// +0x37 and +0x52), the reputation in one byte; 0 at the first read that does not come, 1 if everything came
/// Original: `db_files_read_comps_v1` (0x54ffd0, Db_files.cpp, cdecl).
///
/// [pt] { 0x54ffd0 } (cdecl, Db_files.cpp): `db_files_read_comps_v1(stream, use_zip, registros, n)`: para cada um dos
/// `n` registros de 107 bytes, os campos de `COMPETITION_V1_LAYOUT`, cada um lido sozinho (tamanho, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; a reputação lida num byte (a original o lê num argumento da
/// própria pilha, já usado) vira u16 sem conta. A primeira leitura que não vem dá 0 (o que já veio fica); tudo,
/// 1. Chamado por `db_files_read_comps` 0x54feb0 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
pub(crate) unsafe extern "C" fn db_files_read_comps_v1(stream: usize, use_zip: i32, records: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler competições v1");
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = records + i * 107;
        for &(o, t, q) in &COMPETITION_V1_LAYOUT {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if read(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o) as *mut u16).write_unaligned(b as u16); // reputação da competição
                } else if read(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

