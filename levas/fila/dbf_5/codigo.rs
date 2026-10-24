// ---------- %%LEVA%%a leva: a leitura dos .dat com conversor: clubes, competições, não jogadores (Db_files.cpp) ----------

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
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(0x54dc90usize);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler clubes");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 581, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
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
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I0%%)(stream, use_zip, registros, entrada)
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
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(0x54ffd0usize);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler competições");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 107, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 107, n, stream)
    };
    (lidos == n) as i32 // competições de clube
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_competicoes(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I1%%)(stream, use_zip, registros, entrada)
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
        let converter: unsafe extern "C" fn(usize, i32, usize, u32) -> i32 = core::mem::transmute(0x54e9a0usize);
        return converter(stream, use_zip, registros, n);
    }
    if versao != 2 {
        return orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada);
    }
    crate::comparacao::ramo("banco ler não jogadores");
    let lidos = if use_zip != 0 {
        let zread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
        zread(registros, 68, n, stream)
    } else {
        let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
        fread(registros, 68, n, stream)
    };
    (lidos == n) as i32 // não jogadores
}

/// Comparador: ler duas vezes avançaria o arquivo (e a tabela é a que a carga usa).
// ponytail: só a original; a prova é o .qst (a tabela lida vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_ler_nao_jogadores(stream: usize, use_zip: i32, registros: usize, entrada: usize) -> i32 {
    orig::<unsafe extern "C" fn(usize, i32, usize, usize) -> i32>(%%I2%%)(stream, use_zip, registros, entrada)
}
