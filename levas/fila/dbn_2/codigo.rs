// ---------- %%LEVA%%a leva: a carga do banco: trocar os índices por ponteiros e o vetor de clubes pela reputação (Database.cpp) ----------

/// A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
unsafe fn caixa_do_banco_dbn2(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b5ac8, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// As tabelas do banco: (ponteiro, quantidade, bytes por linha).
const TABELAS_DO_BANCO_DBN2: [(usize, usize, usize); 12] = [
    (0xae23b4, 0xae235c, 581), (0xae23a8, 0xae2350, 0x122), (0xae23e8, 0xae2390, 0x2b), (0xae23b0, 0xae2358, 0x4e),
    (0xae23c8, 0xae2370, 0x34), (0xae23c0, 0xae2368, 0x44), (0xae23bc, 0xae2364, 0x6e), (0xae23e0, 0xae2388, 0x65),
    (0xae23cc, 0xae2374, 0x11), (0xae23ac, 0xae2354, 0x38), (0xae23d4, 0xae237c, 0x1a), (0xae23d0, 0xae2378, 0x6b),
];

/// Os campos de pessoa que, fora da faixa, abrem a caixa na 0x5315a0: (campo, teto, tabela, bytes, texto, linha).
const PESSOA_COM_CAIXA_DBN2: [(usize, usize, usize, usize, usize, u32); 3] = [
    (0x61, 0xae236c, 0xae23c4, 0x46, 0x9b60b4, 0x18f6), // jogador
    (0x65, 0xae2370, 0xae23c8, 0x34, 0x9b6088, 0x1907), // preferências
    (0x69, 0xae2368, 0xae23c0, 0x44, 0x9b605c, 0x1918), // não jogador
];

/// { 0x5315a0 } (cdecl, Database.cpp): `resolve_database_indexes(leniente)`, na carga (load_database, com 0) e na
/// gravação (save_database, depois de gravar). Troca cada campo de id pelo ponteiro ao registro (tabela + id × bytes),
/// na ordem da original e com as regras dela: negativo → nulo; alguns com teto "maior que a quantidade" (o id igual à
/// quantidade passa), outros "a partir da quantidade", outros sem teto. Clube: nação, divisões, estádios, cores, pessoas
/// e clubes (+0x53..+0xbb, teto inclusivo; registro em 0xde1f64 nos altos de nação, última divisão e estádio); elenco
/// +0xd7 (50; com `leniente` = 0, a partir de [0xae2364] vira nulo, com registro acima de [0xae2364] + 0x52d), olheiros,
/// técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar e os 20 escalados (+0x1e0), com registro.
/// Depois nações, árbitros, estádios, preferências, não jogadores, pessoas (jogador, preferências e não jogador fora da
/// faixa: registro, caixa e nulo), competições de pessoa, histórico de pessoas, cidades, histórico de competições e
/// competições de clube. Erros do original, copiados: a última divisão alta do clube (+0x5b) zera +0x60 (que depois vira
/// a competição 0) e fica crua; o estádio vizinho alto (+0x48) zera a cidade (+0x38) e fica cru. Não lê os registros
/// alvo nem aloca.
unsafe extern "C" fn resolve_database_indexes(leniente: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let alvo = |base: usize, v: i32, tam: usize| rd(base).wrapping_add((v as usize).wrapping_mul(tam));
    // sem teto: negativo → nulo
    let a = |f: usize, base: usize, tam: usize| {
        let v = gi(f);
        wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
    };
    // teto inclusivo: negativo → nulo; acima de [teto] → Some(v), quem chama decide; senão o registro
    let b = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v < 0 {
            wr(f, 0);
            None
        } else if v > gi(teto) {
            Some(v)
        } else {
            wr(f, alvo(base, v, tam));
            None
        }
    };
    // teto exclusivo: a partir de [teto] → Some(v), quem chama decide; negativo → nulo; senão o registro
    let c = |f: usize, teto: usize, base: usize, tam: usize| -> Option<i32> {
        let v = gi(f);
        if v >= gi(teto) {
            Some(v)
        } else {
            wr(f, if v < 0 { 0 } else { alvo(base, v, tam) });
            None
        }
    };
    let linhas = |cont: usize, base: usize, tam: usize, f: &dyn Fn(usize, i32)| {
        let mut i = 0i32;
        while i < gi(cont) {
            f(rd(base) + i as usize * tam, i);
            i += 1;
        }
    };
    linhas(0xae235c, 0xae23b4, 581, &|e, i| {
        if let Some(v) = b(e + 0x53, 0xae2350, 0xae23a8, 0x122) {
            formatar(0xde1f64, 0x9b62b0, v);
            wr(e + 0x53, 0);
        }
        if b(e + 0x57, 0xae2378, 0xae23d0, 0x6b).is_some() {
            wr(e + 0x57, 0);
        }
        if let Some(v) = b(e + 0x5b, 0xae2378, 0xae23d0, 0x6b) {
            formatar(0xde1f64, 0x9b6274, v);
            wr(e + 0x60, 0); // erro do original: zera +0x60 e +0x5b fica cru
        }
        if b(e + 0x60, 0xae2378, 0xae23d0, 0x6b).is_some() {
            wr(e + 0x60, 0);
        }
        if let Some(v) = b(e + 0x69, 0xae2358, 0xae23b0, 0x4e) {
            formatar(0xde1f64, 0x9b6240, v);
            wr(e + 0x69, 0);
        }
        for &(d, teto, base, tam) in &[
            (0x6eusize, 0xae2358usize, 0xae23b0usize, 0x4eusize),
            (0x83, 0xae2394, 0xae23ec, 0x3a), (0x8b, 0xae2394, 0xae23ec, 0x3a), (0x93, 0xae2394, 0xae23ec, 0x3a),
            (0x87, 0xae2394, 0xae23ec, 0x3a), (0x8f, 0xae2394, 0xae23ec, 0x3a), (0x97, 0xae2394, 0xae23ec, 0x3a),
            (0x9b, 0xae2364, 0xae23bc, 0x6e), (0x9f, 0xae2364, 0xae23bc, 0x6e), (0xa3, 0xae2364, 0xae23bc, 0x6e),
            (0xa7, 0xae2364, 0xae23bc, 0x6e), (0xab, 0xae2364, 0xae23bc, 0x6e), (0xaf, 0xae2364, 0xae23bc, 0x6e),
            (0xb3, 0xae235c, 0xae23b4, 581), (0xb7, 0xae235c, 0xae23b4, 581), (0xbb, 0xae235c, 0xae23b4, 581),
        ] {
            if b(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
        for j in 0..50 {
            let f = e + 0xd7 + 4 * j;
            let v = gi(f);
            if leniente == 0 && v >= gi(0xae2364) { // o elenco sem as linhas a mais
                if v > gi(0xae2364) + 0x52d {
                    formatar(0xde1f64, 0x9b6218, i, v);
                }
                wr(f, 0);
            } else {
                wr(f, if v < 0 { 0 } else { alvo(0xae23bc, v, 0x6e) });
            }
        }
        // olheiros, técnicos, fisioterapeutas, diretores, treinador, presidente, auxiliar, os 20 escalados
        for &(d, k, texto) in &[(0x1b3usize, 7usize, 0x9b61f4usize), (0x19f, 5, 0x9b61d0), (0x1cf, 3, 0x9b61ac), (0xc3, 3, 0x9b6188),
            (0xcf, 1, 0x9b6164), (0xbf, 1, 0x9b6140), (0xd3, 1, 0x9b6110), (0x1e0, 20, 0x9b60dc)]
        {
            for j in 0..k {
                let f = e + d + 4 * j;
                if let Some(v) = c(f, 0xae2364, 0xae23bc, 0x6e) {
                    formatar(0xde1f64, texto, i, v);
                    wr(f, 0);
                }
            }
        }
    });
    linhas(0xae2350, 0xae23a8, 0x122, &|e, _| {
        for &(d, base, tam) in &[(0x71usize, 0xae23a4usize, 0xc6usize), (0x7a, 0xae23ac, 0x38), (0x80, 0xae23b0, 0x4e),
            (0x90, 0xae23ec, 0x3a), (0x98, 0xae23ec, 0x3a), (0xa0, 0xae23ec, 0x3a), (0x94, 0xae23ec, 0x3a),
            (0x9c, 0xae23ec, 0x3a), (0xa4, 0xae23ec, 0x3a), (0x110, 0xae23a8, 0x122), (0x114, 0xae23a8, 0x122),
            (0x118, 0xae23a8, 0x122)]
        {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2390, 0xae23e8, 0x2b, &|e, _| {
        for &(d, base, tam) in &[(4usize, 0xae23f0usize, 0x3cusize), (8, 0xae23f4, 0x3c), (0x16, 0xae23a8, 0x122), (0x1a, 0xae23ac, 0x38)] {
            a(e + d, base, tam);
        }
    });
    linhas(0xae2358, 0xae23b0, 0x4e, &|e, _| {
        if b(e + 0x38, 0xae2354, 0xae23ac, 0x38).is_some() {
            wr(e + 0x38, 0);
        }
        if b(e + 0x48, 0xae2358, 0xae23b0, 0x4e).is_some() {
            wr(e + 0x38, 0); // erro do original: zera a cidade e +0x48 fica cru
        }
    });
    linhas(0xae2370, 0xae23c8, 0x34, &|e, _| {
        for d in (4..=0x18).step_by(4) {
            a(e + d, 0xae23b4, 581);
        }
        for d in (0x1c..=0x30).step_by(4) {
            a(e + d, 0xae23bc, 0x6e);
        }
    });
    linhas(0xae2368, 0xae23c0, 0x44, &|e, _| {
        for d in (0x23..=0x3f).step_by(4) {
            if c(e + d, 0xae2364, 0xae23bc, 0x6e).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2364, 0xae23bc, 0x6e, &|e, i| {
        for &(d, base, tam) in &[(4usize, 0xae23f0usize, 0x3cusize), (8, 0xae23f4, 0x3c), (0xc, 0xae23f8, 0x3c),
            (0x1a, 0xae23a8, 0x122), (0x39, 0xae23b4, 581), (0x1e, 0xae23a8, 0x122), (0x24, 0xae23b4, 581)]
        {
            a(e + d, base, tam);
        }
        for &(d, teto, base, tam, texto, linha) in PESSOA_COM_CAIXA_DBN2.iter() {
            if let Some(v) = c(e + d, teto, base, tam) {
                formatar(0xde1f64, texto, i, v);
                caixa_do_banco_dbn2(linha);
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2388, 0xae23e0, 0x65, &|e, _| {
        for &(d, teto, base, tam) in &[(0x53usize, 0xae234cusize, 0xae23a4usize, 0xc6usize), (0x57, 0xae2350, 0xae23a8, 0x122),
            (0x5b, 0xae2394, 0xae23ec, 0x3a), (0x5f, 0xae2394, 0xae23ec, 0x3a)]
        {
            if c(e + d, teto, base, tam).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2374, 0xae23cc, 0x11, &|e, _| {
        a(e + 4, 0xae23bc, 0x6e);
        a(e + 0xa, 0xae23b4, 581);
    });
    linhas(0xae2354, 0xae23ac, 0x38, &|e, _| {
        a(e + 0x1f, 0xae23a8, 0x122); // a nação da cidade
    });
    linhas(0xae237c, 0xae23d4, 0x1a, &|e, _| {
        a(e + 4, 0xae23d0, 0x6b);
        a(e + 0xa, 0xae23b4, 581);
        a(e + 0xe, 0xae23b4, 581);
        for &d in &[0x12usize, 0x16] {
            if b(e + d, 0xae235c, 0xae23b4, 581).is_some() {
                wr(e + d, 0);
            }
        }
    });
    linhas(0xae2378, 0xae23d0, 0x6b, &|e, _| {
        for &(d, base, tam) in &[(0x5dusize, 0xae23a8usize, 0x122usize), (0x59, 0xae23a4, 0xc6), (0x61, 0xae23ec, 0x3a), (0x65, 0xae23ec, 0x3a)] {
            a(e + d, base, tam);
        }
    });
}

/// { 0x53db40 } (cdecl, Database.cpp): `database_sort_clubs_by_reputation(refazer)`, na criação, no load_game_data
/// 0x82a9c0 (vetor ausente do save) e no laço diário 0x5cccd0. Com `refazer` (byte) ≠ 0, o vetor [0xae2d1c] recebe os
/// ids (+0) dos [0xae235c] clubes na ordem da tabela; depois qsort (0x945333) dos [0xae235c] ids com 0x540020 (pela
/// reputação).
unsafe extern "C" fn database_sort_clubs_by_reputation(refazer: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    if refazer as u8 != 0 {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            *((rd(0xae2d1c) + i as usize * 4) as *mut i32) = gi(rd(0xae23b4) + i as usize * 581);
            i += 1;
        }
    }
    ordenar(rd(0xae2d1c), gi(0xae235c) as u32, 4, 0x540020); // todos os clubes pela reputação
}

/// O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
fn diferenca_dbn2(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}

/// Comparador de 0x5315a0: original e cópia lado a lado sobre as 12 tabelas inteiras (~50 MB na carga) e o texto de
/// 0xde1f64. Pessoa com jogador, preferências ou não jogador fora da faixa (caixa): só a original.
unsafe extern "C" fn comparar_resolve_database_indexes(leniente: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(%%I0%%);
    let gi = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let rd = |a: usize| *(a as *const usize);
    let (pessoas, n) = (rd(0xae23bc), gi(0xae2364));
    let caixa = (0..n.max(0) as usize).any(|i| PESSOA_COM_CAIXA_DBN2.iter().any(|&(d, teto, ..)| gi(pessoas + i * 0x6e + d) >= gi(teto)));
    if caixa {
        return o(leniente);
    }
    let mut regioes = vec![(0xde1f64usize, 0x80usize)];
    for &(p, cont, tam) in TABELAS_DO_BANCO_DBN2.iter() {
        if rd(p) != 0 && gi(cont) > 0 {
            regioes.push((rd(p), gi(cont) as usize * tam));
        }
    }
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, || o(leniente), || resolve_database_indexes(leniente));
    let onde = diferenca_dbn2(&mo, &mr);
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("resolve_database_indexes({}): diverge em {:x} original={:02x} rust={:02x}", leniente, a, x, y),
        None => format!("resolve_database_indexes({}): diverge", leniente),
    });
    // ponytail: ~50 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
}

/// Comparador de 0x53db40: original e cópia lado a lado sobre o vetor de ids [0xae2d1c].
unsafe extern "C" fn comparar_database_sort_clubs_by_reputation(refazer: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(%%I1%%);
    let v = *(0xae2d1c as *const usize);
    let n = core::ptr::read_unaligned(0xae235c as *const i32);
    if v == 0 || n <= 0 {
        return o(refazer);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(v, n as usize * 4)], || o(refazer), || database_sort_clubs_by_reputation(refazer));
    let onde = diferenca_dbn2(&mo, &mr);
    PLACAR.registrar(igual, || format!("database_sort_clubs_by_reputation({}): diverge em {:x?}", refazer as u8, onde));
}
