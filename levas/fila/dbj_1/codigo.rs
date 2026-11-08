// ---------- %%LEVA%%a leva: os conversores da versão 1: clubes, não jogadores, jogadores (Db_files.cpp) ----------

/// Um conversor da versão 1 (Db_files.cpp): para cada um dos `n` registros de `tamanho` bytes, os campos na
/// ordem do arquivo, `(deslocamento, tamanho, quantos)`, cada um lido sozinho (`tamanho`, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; tamanho 0 = um byte do arquivo que vira u16 (`reputacao`) em
/// deslocamento + 2k (a original lê o byte num argumento da própria pilha, já usado, e grava o u16 antes da
/// leitura seguinte). A primeira leitura que não vem dá 0 (o que já veio fica no registro); tudo, 1. Os
/// bytes que o arquivo não traz (buracos do registro) ficam como estavam.
unsafe fn convert_v1_records(stream: usize, use_zip: i32, registros: usize, n: u32, tamanho: usize,
                       campos: &[(usize, u32, usize)], injury_word_0c: fn(u8) -> u16) -> i32 {
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = registros + i * tamanho;
        for &(o, t, q) in campos {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if ler(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o + 2 * k) as *mut u16).write_unaligned(injury_word_0c(b));
                } else if ler(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Bytes de um registro da versão 1 no arquivo.
fn v1_record_size(campos: &[(usize, u32, usize)]) -> usize {
    campos.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum()
}

/// TClub (581 bytes) no club.dat versão 1 (577 bytes no arquivo): buracos em +0x37, +0x52 e +0x244; a
/// reputação vem num byte (0 vira 1) × 500 em +0x80.
const CLUB_V1_LAYOUT: [(usize, u32, usize); 18] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 3), (0x5f, 1, 1),
    (0x60, 4, 1), (0x64, 1, 1), (0x65, 4, 2), (0x6d, 1, 1), (0x6e, 4, 1), (0x72, 1, 1), (0x73, 4, 3), (0x7f, 1, 1),
    (0x80, 0, 1), (0x82, 1, 1), (0x83, 4, 87), (0x1df, 1, 1), (0x1e0, 4, 25)];

/// Não jogador (68 bytes, id 9) versão 1 (65 no arquivo): as três reputações (+8, +0xa, +0xc) vêm em bytes.
const NON_PLAYER_V1_LAYOUT: [(usize, u32, usize); 6] = [(0, 4, 1), (4, 2, 2), (8, 0, 3), (0xe, 1, 21), (0x23, 4, 8), (0x43, 1, 1)];

/// Jogador (70 bytes, id 10) versão 1 (67 no arquivo): as três reputações (+9, +0xb, +0xd) vêm em bytes.
const PLAYER_V1_FIELDS: [(usize, u32, usize); 5] = [(0, 4, 1), (4, 1, 1), (5, 2, 2), (9, 0, 3), (0xf, 1, 55)];

/// { 0x54dc90 } (cdecl, Db_files.cpp): `db_files_read_clubs_v1(stream, use_zip, registros, n)`: o club.dat
/// versão 1 campo a campo (`CLUB_V1_LAYOUT`); a reputação lida num byte, 0 vira 1, × 500 (u16, como o `imul ax`).
/// Chamado por `db_files_read_clubs` 0x54db50 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
unsafe extern "C" fn db_files_read_clubs_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco ler clubes v1");
    convert_v1_records(stream, use_zip, registros, n, 581, &CLUB_V1_LAYOUT, |b| (b.max(1) as u16).wrapping_mul(500))
}

/// { 0x54e9a0 } (cdecl, Db_files.cpp): `db_files_read_players_v1(stream, use_zip, registros, n)` (o mapa da
/// comunidade troca os nomes: é a tabela id 9, a dos não jogadores, 68 bytes): campo a campo
/// (`NON_PLAYER_V1_LAYOUT`); as reputações lidas em bytes, sem conta. Chamado por 0x54e880 com a versão 1.
unsafe extern "C" fn db_files_read_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco ler não jogadores v1");
    convert_v1_records(stream, use_zip, registros, n, 68, &NON_PLAYER_V1_LAYOUT, |b| b as u16)
}

/// { 0x54f150 } (cdecl, Db_files.cpp): `db_files_read_non_players_v1(stream, use_zip, registros, n)` (o mapa
/// troca os nomes: é a tabela id 10, a dos jogadores, 70 bytes): campo a campo (`PLAYER_V1_FIELDS`); as reputações
/// lidas em bytes, sem conta. Chamado por 0x54f030 com a versão 1.
unsafe extern "C" fn db_files_read_non_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparison::branch("banco ler jogadores v1");
    convert_v1_records(stream, use_zip, registros, n, 70, &PLAYER_V1_FIELDS, |b| b as u16)
}

type V1Converter = unsafe extern "C" fn(usize, i32, usize, u32) -> i32;

/// Comparador de um conversor: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à
/// posição do `ftell` 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo
/// parou. Fica a posição da Rust (a mesma, se iguais).
// ponytail: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.
unsafe fn comparar_conversor_v1(nome: &str, o: V1Converter, rust: V1Converter, stream: usize, use_zip: i32,
                                registros: usize, n: u32, tamanho: usize) -> i32 {
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x946aaeusize);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(0x946a22usize);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side(&[(registros, n as usize * tamanho)],
        || {
            let r = o(stream, use_zip, registros, n);
            fim_o = ftell(stream);
            r
        },
        || {
            fseek(stream, pos, 0);
            let r = rust(stream, use_zip, registros, n);
            fim_r = ftell(stream);
            r
        });
    PLACAR.record(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("{}(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", nome, n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

unsafe extern "C" fn comparar_db_files_read_clubs_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_clubs_v1", original_fn::<V1Converter>(%%I0%%), db_files_read_clubs_v1, stream, use_zip, registros, n, 581)
}

unsafe extern "C" fn comparar_db_files_read_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_players_v1", original_fn::<V1Converter>(%%I1%%), db_files_read_players_v1, stream, use_zip, registros, n, 68)
}

unsafe extern "C" fn comparar_db_files_read_non_players_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("db_files_read_non_players_v1", original_fn::<V1Converter>(%%I2%%), db_files_read_non_players_v1, stream, use_zip, registros, n, 70)
}

/// Autoteste dos conversores da versão 1 (CM_AUTOTESTE=1, modo comparação: a base de fábrica e o save são
/// versão 2 e a carreira não passa por eles). Para cada conversor, 64 casos de sementes fixas: de 1 a 6
/// registros, um arquivo temporário (`autoteste_banco_v1.tmp` na pasta do jogo, apagado no fim) com bytes
/// sorteados (1 caso em 4 só zeros: a reputação 0 do clube), inteiro (1 em 3) ou cortado num ponto sorteado
/// (a leitura falha no meio de um campo), aberto pelo `fopen` 0x945b00 do jogo; o buffer começa sorteado (os
/// buracos do registro entram na comparação).
pub unsafe fn autoteste_dbj_1() {
    const ARQ: &str = "autoteste_banco_v1.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let (t0, d0) = PLACAR.counts();
    let mut s = TestRng(0x5631_4442_4a31);
    let casos: [(V1Converter, &[(usize, u32, usize)], usize); 3] = [(comparar_db_files_read_clubs_v1, &CLUB_V1_LAYOUT, 581),
        (comparar_db_files_read_players_v1, &NON_PLAYER_V1_LAYOUT, 68), (comparar_db_files_read_non_players_v1, &PLAYER_V1_FIELDS, 70)];
    for (comparar, campos, tamanho) in casos {
        for caso in 0..64u64 {
            let n = 1 + (s.next_u64() % 6) as usize;
            let total = n * v1_record_size(campos);
            let corte = if caso % 3 == 0 { total } else { (s.next_u64() % (total as u64 + 1)) as usize };
            let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.next_u64() as u8 }).collect();
            if std::fs::write(ARQ, &bytes).is_err() {
                continue;
            }
            let f = fopen(b"autoteste_banco_v1.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
            if f == 0 {
                continue;
            }
            let mut buf: Vec<u8> = (0..n * tamanho).map(|_| s.next_u64() as u8).collect();
            comparar(f, 0, buf.as_mut_ptr() as usize, n as u32);
            fclose(f);
        }
    }
    let _ = std::fs::remove_file(ARQ);
    let (t1, d1) = PLACAR.counts();
    linha(&format!("banco v1 (conversores): AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}
