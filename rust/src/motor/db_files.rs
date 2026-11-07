// ---------- 232a leva: os conversores da versão 1: clubes, não jogadores, jogadores (Db_files.cpp) ----------

/// Um conversor da versão 1 (Db_files.cpp): para cada um dos `n` registros de `tamanho` bytes, os campos na
/// ordem do arquivo, `(deslocamento, tamanho, quantos)`, cada um lido sozinho (`tamanho`, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; tamanho 0 = um byte do arquivo que vira u16 (`reputacao`) em
/// deslocamento + 2k (a original lê o byte num argumento da própria pilha, já usado, e grava o u16 antes da
/// leitura seguinte). A primeira leitura que não vem dá 0 (o que já veio fica no registro); tudo, 1. Os
/// bytes que o arquivo não traz (buracos do registro) ficam como estavam.
unsafe fn converter_v1(stream: usize, use_zip: i32, registros: usize, n: u32, tamanho: usize,
                       campos: &[(usize, u32, usize)], reputacao: fn(u8) -> u16) -> i32 {
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
                    ((r + o + 2 * k) as *mut u16).write_unaligned(reputacao(b));
                } else if ler(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Bytes de um registro da versão 1 no arquivo.
fn tamanho_v1(campos: &[(usize, u32, usize)]) -> usize {
    campos.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum()
}

/// TClub (581 bytes) no club.dat versão 1 (577 bytes no arquivo): buracos em +0x37, +0x52 e +0x244; a
/// reputação vem num byte (0 vira 1) × 500 em +0x80.
const CLUBE_V1: [(usize, u32, usize); 18] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 3), (0x5f, 1, 1),
    (0x60, 4, 1), (0x64, 1, 1), (0x65, 4, 2), (0x6d, 1, 1), (0x6e, 4, 1), (0x72, 1, 1), (0x73, 4, 3), (0x7f, 1, 1),
    (0x80, 0, 1), (0x82, 1, 1), (0x83, 4, 87), (0x1df, 1, 1), (0x1e0, 4, 25)];

/// Não jogador (68 bytes, id 9) versão 1 (65 no arquivo): as três reputações (+8, +0xa, +0xc) vêm em bytes.
const NAO_JOGADOR_V1: [(usize, u32, usize); 6] = [(0, 4, 1), (4, 2, 2), (8, 0, 3), (0xe, 1, 21), (0x23, 4, 8), (0x43, 1, 1)];

/// Jogador (70 bytes, id 10) versão 1 (67 no arquivo): as três reputações (+9, +0xb, +0xd) vêm em bytes.
const JOGADOR_V1: [(usize, u32, usize); 5] = [(0, 4, 1), (4, 1, 1), (5, 2, 2), (9, 0, 3), (0xf, 1, 55)];

/// { 0x54dc90 } (cdecl, Db_files.cpp): `db_files_read_clubs_v1(stream, use_zip, registros, n)`: o club.dat
/// versão 1 campo a campo (`CLUBE_V1`); a reputação lida num byte, 0 vira 1, × 500 (u16, como o `imul ax`).
/// Chamado por `db_files_read_clubs` 0x54db50 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
unsafe extern "C" fn ler_clubes_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler clubes v1");
    converter_v1(stream, use_zip, registros, n, 581, &CLUBE_V1, |b| (b.max(1) as u16).wrapping_mul(500))
}

/// { 0x54e9a0 } (cdecl, Db_files.cpp): `db_files_read_players_v1(stream, use_zip, registros, n)` (o mapa da
/// comunidade troca os nomes: é a tabela id 9, a dos não jogadores, 68 bytes): campo a campo
/// (`NAO_JOGADOR_V1`); as reputações lidas em bytes, sem conta. Chamado por 0x54e880 com a versão 1.
unsafe extern "C" fn ler_nao_jogadores_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler não jogadores v1");
    converter_v1(stream, use_zip, registros, n, 68, &NAO_JOGADOR_V1, |b| b as u16)
}

/// { 0x54f150 } (cdecl, Db_files.cpp): `db_files_read_non_players_v1(stream, use_zip, registros, n)` (o mapa
/// troca os nomes: é a tabela id 10, a dos jogadores, 70 bytes): campo a campo (`JOGADOR_V1`); as reputações
/// lidas em bytes, sem conta. Chamado por 0x54f030 com a versão 1.
unsafe extern "C" fn ler_jogadores_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler jogadores v1");
    converter_v1(stream, use_zip, registros, n, 70, &JOGADOR_V1, |b| b as u16)
}

type ConversorV1 = unsafe extern "C" fn(usize, i32, usize, u32) -> i32;

/// Comparador de um conversor: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à
/// posição do `ftell` 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo
/// parou. Fica a posição da Rust (a mesma, se iguais).
// ponytail: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.
unsafe fn comparar_conversor_v1(nome: &str, o: ConversorV1, rust: ConversorV1, stream: usize, use_zip: i32,
                                registros: usize, n: u32, tamanho: usize) -> i32 {
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FTELL);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(registros, n as usize * tamanho)],
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
    PLACAR.registrar(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("{}(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", nome, n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

unsafe extern "C" fn comparar_ler_clubes_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("ler_clubes_v1", orig::<ConversorV1>(670), ler_clubes_v1, stream, use_zip, registros, n, 581)
}

unsafe extern "C" fn comparar_ler_nao_jogadores_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("ler_nao_jogadores_v1", orig::<ConversorV1>(671), ler_nao_jogadores_v1, stream, use_zip, registros, n, 68)
}

unsafe extern "C" fn comparar_ler_jogadores_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    comparar_conversor_v1("ler_jogadores_v1", orig::<ConversorV1>(672), ler_jogadores_v1, stream, use_zip, registros, n, 70)
}

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
    let casos: [(ConversorV1, &[(usize, u32, usize)], usize); 3] = [(comparar_ler_clubes_v1, &CLUBE_V1, 581),
        (comparar_ler_nao_jogadores_v1, &NAO_JOGADOR_V1, 68), (comparar_ler_jogadores_v1, &JOGADOR_V1, 70)];
    for (comparar, campos, tamanho) in casos {
        for caso in 0..64u64 {
            let n = 1 + (s.proximo() % 6) as usize;
            let total = n * tamanho_v1(campos);
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

// ---------- 231a leva: a gravação dos .dat: histórico de pessoas, competições, históricos de competição (Db_files.cpp) ----------

/// { 0x54fe50 } (cdecl, Db_files.cpp): `db_files_write_staff_history(stream, use_zip, registros, n)`: grava de uma vez `n`
/// registros de 17 bytes (staff_history.dat, id 17) por `zwrite` 0x932e10 (use_zip ≠ 0: o save) ou `fwrite` 0x945ce6
/// (a pasta Data); 1 se foi tudo. Sem quadro de pilha: nada da pilha vai para o arquivo.
unsafe extern "C" fn gravar_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar histórico de pessoas");
    let escritos = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 17, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 107, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 26, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 101, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 43, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 58, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 60, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
    let juntar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
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
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
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
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    fopen(0xde1f64, 0x9a7f28) // criar (wb)
}

/// Comparador: criar duas vezes truncaria o arquivo com dois FILE abertos. Roda a original e compara só o
/// caminho.
unsafe extern "C" fn comparar_criar_arquivo_do_banco(pasta: usize, nome: usize) -> usize {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> usize>(660)(pasta, nome);
    conferir_caminho_do_banco("criar_arquivo_do_banco", pasta, nome);
    r
}

// ---------- 230a leva: a gravação dos .dat: continentes, nações, cidades (Db_files.cpp) ----------

/// { 0x54d6e0 } (cdecl, Db_files.cpp): `db_files_write_continents(stream, use_zip, registros, quantidade)`: grava de uma vez
/// `quantidade` registros de 198 bytes (continent.dat (id 3)) por `zwrite` 0x932e10 (use_zip ≠ 0: o save; já em Rust,
/// chamado pelo endereço) ou `fwrite` 0x945ce6 (a pasta Data); 1 se foi tudo. Sem quadro, sem SEH, sem caixa.
/// Só o save_database 0x52dad0 chama (com os ponteiros já trocados por índices).
unsafe extern "C" fn gravar_continentes(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco gravar continentes");
    let gravados = if use_zip != 0 {
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 198, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 290, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 56, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 78, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 581, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 110, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 68, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 70, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
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
        let zwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZWRITE);
        zwrite(registros, 52, n, stream)
    } else {
        let fwrite: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
        fwrite(registros, 52, n, stream)
    };
    (gravados == n) as i32
}

/// Comparador: gravar duas vezes duplicaria a tabela no arquivo.
// ponytail: só a original; a prova é o .qst (a tabela gravada) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_gravar_preferencias(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(651)(stream, use_zip, registros, n)
}

// ---------- 229a leva: a leitura dos .dat: árbitros, cores, nomes (Db_files.cpp) ----------

/// { 0x5505c0 } (cdecl, Db_files.cpp): `db_files_read_officials(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 43 bytes
/// (officials.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_arbitros(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(634)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler árbitros");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 43, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 43, n, stream)
    };
    (lidos == n) as i32 // árbitros
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_arbitros(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(634)(stream, use_zip, registros, entrada)
}

/// { 0x550710 } (cdecl, Db_files.cpp): `db_files_read_colours(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 58 bytes
/// (colour.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_cores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(635)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler cores");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 58, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 58, n, stream)
    };
    (lidos == n) as i32 // cores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_cores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(635)(stream, use_zip, registros, entrada)
}

/// { 0x550860 } (cdecl, Db_files.cpp): `db_files_read_names(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 60 bytes
/// (first_names.dat, second_names.dat e common_names.dat: a mesma rotina para os três) por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Outra versão: a caixa "Error"
/// (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao
/// caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_nomes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(636)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler nomes");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 60, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 60, n, stream)
    };
    (lidos == n) as i32 // nomes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_nomes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(636)(stream, use_zip, registros, entrada)
}

// ---------- 229a leva: a leitura dos .dat com conversor: clubes, competições, não jogadores (Db_files.cpp) ----------

/// { 0x54db50 } (cdecl, Db_files.cpp): `db_files_read_clubs(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 581 bytes
/// (club.dat e nat_club.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data);
/// se não veio tudo, 0; se veio, troca a reputação (+0x80, u16) 0 por 1 em todos e dá 1. Versão 1: o
/// conversor campo a campo 0x54dc90 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta
/// dele (sem a troca da reputação: o conversor faz a sua, ×500). Outra versão: a caixa "Error" (Db_files.cpp)
/// e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao caminho da caixa.
unsafe extern "C" fn ler_clubes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao == 1 {
        crate::comparacao::ramo("banco ler clubes versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_CLUBS_V1);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(637)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler clubes");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 581, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 581, n, stream)
    };
    if lidos != n {
        return 0;
    }
    for i in 0..n as usize {
        let p = (registros + i * 581 + 0x80) as *mut u16;
        if p.read_unaligned() == 0 {
            p.write_unaligned(1);
        }
    }
    1 // clubes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_clubes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(637)(stream, use_zip, registros, entrada)
}

/// { 0x54feb0 } (cdecl, Db_files.cpp): `db_files_read_comps(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 107 bytes
/// (club_comp.dat e nation_comp.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta
/// Data); 1 se veio tudo. Versão 1: o conversor campo a campo 0x54ffd0 (stream, use_zip, registros,
/// quantidade), pelo endereço, e a resposta dele. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do
/// jogo; a cópia chama a original. O quadro de 0x200 da original só serve ao caminho da caixa.
unsafe extern "C" fn ler_competicoes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao == 1 {
        crate::comparacao::ramo("banco ler competições versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_COMPS_V1);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(638)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler competições");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 107, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 107, n, stream)
    };
    (lidos == n) as i32 // competições de clube
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_competicoes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(638)(stream, use_zip, registros, entrada)
}

/// { 0x54e880 } (cdecl, Db_files.cpp): `db_files_read_players(stream, use_zip, registros, entrada)` (o mapa da
/// comunidade troca os nomes: é a tabela id 9, a dos não jogadores, 0xae23c0): com a versão da entrada do
/// index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 68 bytes por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Versão 1: o conversor campo a
/// campo 0x54e9a0 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta dele. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa.
unsafe extern "C" fn ler_nao_jogadores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao == 1 {
        crate::comparacao::ramo("banco ler não jogadores versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_PLAYERS_V1);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(639)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler não jogadores");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 68, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 68, n, stream)
    };
    (lidos == n) as i32 // não jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_nao_jogadores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(639)(stream, use_zip, registros, entrada)
}

// ---------- 229a leva: a leitura dos .dat: jogadores, pessoas 110, preferências (Db_files.cpp) ----------

/// { 0x54f030 } (cdecl, Db_files.cpp): `db_files_read_non_players(stream, use_zip, registros, entrada)` (o mapa
/// da comunidade troca os nomes: é a tabela id 10, a dos jogadores, 0xae23c4): com a versão da entrada do
/// index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 70 bytes por `zread` 0x932c30
/// (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Versão 1: o conversor campo a
/// campo 0x54f150 (stream, use_zip, registros, quantidade), pelo endereço, e a resposta dele. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa.
unsafe extern "C" fn ler_jogadores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao == 1 {
        crate::comparacao::ramo("banco ler jogadores versão 1");
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(FN_DB_FILES_READ_NON_PLAYERS_V1);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(640)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler jogadores");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 70, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 70, n, stream)
    };
    (lidos == n) as i32 // jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_jogadores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(640)(stream, use_zip, registros, entrada)
}

/// { 0x54e630 } (cdecl, Db_files.cpp): `db_files_read_staff(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 110 bytes
/// (staff.dat na versão 2: o save e as bases da comunidade; a base de fábrica é a versão 1, 0x54e780) por
/// `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio tudo. Outra versão: a
/// caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro de 0x200 da original só
/// serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_pessoas_110(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(641)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler pessoas 110");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 110, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 110, n, stream)
    };
    (lidos == n) as i32 // pessoas 110
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.
unsafe extern "C" fn comparar_ler_pessoas_110(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(641)(stream, use_zip, registros, entrada)
}

/// { 0x54fc10 } (cdecl, Db_files.cpp): `db_files_read_staff_extra(stream, use_zip, registros, entrada)`: com a
/// versão da entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 52 bytes
/// (as preferências, id 22: o save e as bases da comunidade; na base de fábrica elas saem da conversão das
/// pessoas de 157 bytes) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se
/// veio tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O
/// quadro de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_preferencias(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(642)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler preferências");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 52, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 52, n, stream)
    };
    (lidos == n) as i32 // preferências
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é a carga do save (a tabela lida vira o banco do jogo) e o ramo da cópia.
unsafe extern "C" fn comparar_ler_preferencias(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(642)(stream, use_zip, registros, entrada)
}

// ---------- 228a leva: a leitura dos .dat: continentes, nações, cidades (Db_files.cpp) ----------

/// { 0x54d5e0 } (cdecl, Db_files.cpp): `db_files_read_continents(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 198 bytes
/// (continent.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_continentes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(625)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler continentes");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 198, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 198, n, stream)
    };
    (lidos == n) as i32 // continentes
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_continentes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(625)(stream, use_zip, registros, entrada)
}

/// { 0x54d740 } (cdecl, Db_files.cpp): `db_files_read_nations(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 290 bytes
/// (nation.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_nacoes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(626)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler nações");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 290, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 290, n, stream)
    };
    (lidos == n) as i32 // nações
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_nacoes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(626)(stream, use_zip, registros, entrada)
}

/// { 0x54d8a0 } (cdecl, Db_files.cpp): `db_files_read_cities(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) de 1 a 2 (com sinal), lê de uma vez a quantidade (+0x37) de registros de 56 bytes
/// (city.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_cidades(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao <= 0 || versao > 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(627)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler cidades");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 56, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 56, n, stream)
    };
    (lidos == n) as i32 // cidades
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_cidades(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(627)(stream, use_zip, registros, entrada)
}

// ---------- 228a leva: a leitura dos .dat: estádios, pessoas 157, histórico de pessoas (Db_files.cpp) ----------

/// { 0x54da00 } (cdecl, Db_files.cpp): `db_files_read_stadiums(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 78 bytes
/// (stadium.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_estadios(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(628)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler estádios");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 78, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 78, n, stream)
    };
    (lidos == n) as i32 // estádios
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_estadios(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(628)(stream, use_zip, registros, entrada)
}

/// { 0x54e780 } (cdecl, Db_files.cpp): `db_files_read_staff_v1(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 157 bytes
/// (staff.dat (pessoas, versão de arquivo de 157 bytes)) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_pessoas_157(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(629)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler pessoas 157");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 157, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 157, n, stream)
    };
    (lidos == n) as i32 // pessoas 157
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_pessoas_157(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(629)(stream, use_zip, registros, entrada)
}

/// { 0x54fd60 } (cdecl, Db_files.cpp): `db_files_read_staff_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 17 bytes
/// (staff_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(630)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler histórico de pessoas");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 17, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 17, n, stream)
    };
    (lidos == n) as i32 // histórico de pessoas
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_historico_de_pessoas(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(630)(stream, use_zip, registros, entrada)
}

// ---------- 228a leva: a leitura dos .dat: históricos de competição, competições de pessoal, histórico de competições de pessoal (Db_files.cpp) ----------

/// { 0x550230 } (cdecl, Db_files.cpp): `db_files_read_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 26 bytes
/// (club_comp_history.dat e nation_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_historicos_de_competicao(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(631)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler históricos de competição");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 26, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 26, n, stream)
    };
    (lidos == n) as i32 // históricos de competição
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_historicos_de_competicao(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(631)(stream, use_zip, registros, entrada)
}

/// { 0x550380 } (cdecl, Db_files.cpp): `db_files_read_staff_comps(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 2, lê de uma vez a quantidade (+0x37) de registros de 101 bytes
/// (staff_comp.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(632)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler competições de pessoal");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 101, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 101, n, stream)
    };
    (lidos == n) as i32 // competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(632)(stream, use_zip, registros, entrada)
}

/// { 0x5504d0 } (cdecl, Db_files.cpp): `db_files_read_staff_comp_history(stream, use_zip, registros, entrada)`: com a versão da
/// entrada do index (+0x3f) igual a 1, lê de uma vez a quantidade (+0x37) de registros de 58 bytes
/// (staff_comp_history.dat) por `zread` 0x932c30 (use_zip ≠ 0: o save) ou `fread` 0x945bfe (a pasta Data); 1 se veio
/// tudo. Outra versão: a caixa "Error" (Db_files.cpp) e a saída do jogo; a cópia chama a original. O quadro
/// de 0x200 da original só serve ao caminho da caixa (_splitpath): nada da pilha vai para a tabela.
unsafe extern "C" fn ler_historico_de_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    let n = core::ptr::read_unaligned((entrada + 0x37) as *const u32);
    let versao = core::ptr::read_unaligned((entrada + 0x3f) as *const i32);
    if versao != 1 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(633)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler histórico de competições de pessoal");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_ZREAD);
        zread(registros, 58, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
        fread(registros, 58, n, stream)
    };
    (lidos == n) as i32 // histórico de competições de pessoal
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_historico_de_competicoes_de_pessoal(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(633)(stream, use_zip, registros, entrada)
}

