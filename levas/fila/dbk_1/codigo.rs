// ---------- %%LEVA%%a leva: a criação do jogo: os elencos dos times A e B, a comissão do time B e o índice de nomes por nação (Database.cpp) ----------

/// { 0x541e60 } (cdecl, Database.cpp): `database_trim_senior_and_b_team_squads()`, uma vez na criação (create_game
/// 0x8282f1). Em cada clube, a marca +0x244 = tem parceiro (0x540a50 com 0) e o parceiro tem nação (+0x53). No time
/// A (`eh_a`): o elenco conjunto A+B (0x541f90 com dobro: 100 ponteiros) em ordem de reputação (qsort 0x945333 com
/// 0x5402f0); de trás para a frente, enquanto o conjunto tiver mais de 48, tira a pessoa do clube dela (0x53c190 com
/// o clube +0x39, sem data, desligando a pessoa, não como jogador), pulando os goleiros (jogador +0x61 com +0xf ≥ 15,
/// byte com sinal) enquanto restarem 3 ou menos. Contadores de 8 bits com sinal, como os da original. Sem sorteio;
/// a local `eh_a` só é lida quando 0x540a50 devolve parceiro (e então ela foi gravada).
unsafe extern "C" fn database_trim_senior_and_b_team_squads() {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let goleiro = |q: usize| {
        let j = rd(q + 0x61);
        j != 0 && *((j + 0xf) as *const i8) >= 0xf
    };
    let mut clube = *(0xae23b4 as *const usize);
    let mut i = 0i32;
    while i < *(0xae235c as *const i32) {
        let mut eh_a = 0u32;
        let p = parceiro(clube, &mut eh_a, 0);
        if p == 0 || rd(p + 0x53) == 0 {
            *((clube + 0x244) as *mut u8) = 0;
        } else {
            *((clube + 0x244) as *mut u8) = 1;
            if eh_a != 0 {
                let mut v = [0usize; 100];
                reunir(clube, v.as_mut_ptr(), 1);
                ordenar(v.as_mut_ptr() as usize, 100, 4, 0x5402f0);
                let (mut total, mut goleiros) = (0i8, 0i8);
                for &q in v.iter() {
                    if q != 0 {
                        total += 1;
                        if goleiro(q) {
                            goleiros += 1;
                        }
                    }
                }
                let mut k = 99i8;
                while k >= 0 && total > 48 { // o elenco conjunto a 48
                    let q = v[k as usize];
                    if q != 0 {
                        let g = goleiro(q);
                        if !(g && goleiros < 3) {
                            if g {
                                goleiros -= 1;
                            }
                            tirar(rd(q + 0x39), q, 0, 1, 0);
                            total -= 1;
                        }
                    }
                    k -= 1;
                }
            }
        }
        i += 1;
        clube += 0x245;
    }
}

/// { 0x544970 } (cdecl, Database.cpp): `database_merge_b_team_staff()`, uma vez na criação (create_game 0x8283d9).
/// Para cada time A com parceiro B (0x540a50 com 1, que exige a marca +0x244 de 0x541e60): conta as pessoas válidas
/// (ponteiro não nulo e id < [0xae2364]) do elenco do A (+0xd7, 50), ordena o elenco do B (qsort com 0x540130) e tira
/// do B as que passarem de 50 no conjunto. Depois, nas casas +0x19f (5, cargo 8), +0x1b3 (7, cargo 9) e +0x1cf (3,
/// cargo 10): cada pessoa válida do B sai dele (0x53c190) e, enquanto o A tiver vaga, entra no A (0x53bd10 com a data
/// do jogo 0xae2c90, gravando na pessoa). Treinador (+0xcf): com os dois, o do B só sai; só o do B: sai e entra no A
/// com o cargo dele (+0x3d, lido depois da saída; a original empurra o byte com lixo em cima, e a 0x53bd10 só usa o
/// byte), sem data. Auxiliar (+0xd3): com os dois, o do B sai. Sem sorteio.
unsafe extern "C" fn database_merge_b_team_staff() {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let valida = |p: usize| p != 0 && *(p as *const i32) < *(0xae2364 as *const i32);
    let contar = |c: usize, d: usize, k: usize| (0..k).filter(|&j| valida(rd(c + d + 4 * j))).count() as i32;
    let mut clube = *(0xae23b4 as *const usize);
    let mut i = 0i32;
    while i < *(0xae235c as *const i32) {
        let mut eh_a = 0u32;
        let b = parceiro(clube, &mut eh_a, 1);
        if b != 0 && eh_a != 0 {
            let mut n = contar(clube, 0xd7, 50);
            ordenar(b + 0xd7, 50, 4, 0x540130);
            for j in 0..50 {
                let p = rd(b + 0xd7 + 4 * j);
                if valida(p) {
                    n += 1;
                    if n > 50 {
                        tirar(b, p, 0, 1, 0);
                    }
                }
            }
            for &(d, k, cargo) in &[(0x19fusize, 5usize, 8u32), (0x1b3, 7, 9), (0x1cf, 3, 10)] {
                let mut n = contar(clube, d, k);
                for j in 0..k {
                    let p = rd(b + d + 4 * j);
                    if valida(p) {
                        tirar(b, p, 0, 1, 0);
                        if n < k as i32 { // a comissão do B vai para o A
                            n += 1;
                            incluir(clube, p, cargo, 0xae2c90, 1, 0);
                        }
                    }
                }
            }
            let tb = rd(b + 0xcf);
            if rd(clube + 0xcf) != 0 && tb != 0 {
                tirar(b, tb, 0, 1, 0);
            } else if tb != 0 {
                tirar(b, tb, 0, 1, 0);
                incluir(clube, tb, *((tb + 0x3d) as *const u8) as u32, 0, 1, 0);
            }
            if rd(clube + 0xd3) != 0 && rd(b + 0xd3) != 0 {
                tirar(b, rd(b + 0xd3), 0, 1, 0);
            }
        }
        i += 1;
        clube += 0x245;
    }
}

/// { 0x539750 } (cdecl, Database.cpp): `database_build_nation_name_index()`, na criação (create_game 0x82836c) e na
/// carga do jogo. Aloca (malloc 0x944e46) [0xae2350] entradas de 0x24 bytes em [0xae2338]: primeira linha (+0, +4,
/// +8 = −1), soma dos pesos (+0xc, +0x10, +0x14 = 0) e quantidade (+0x18, +0x1c, +0x20 = 0) dos primeiros, segundos
/// e nomes comuns. Percorre os segundos nomes ([0xae23f4], [0xae239c]), os primeiros ([0xae23f0], [0xae2398]) e os
/// comuns ([0xae23f8], [0xae23a0]), linhas de 60 bytes com a nação em +0x37 (0..[0xae2350]) e o peso em +0x3b
/// (byte com sinal): linhas seguidas da mesma nação formam um grupo, gravado na entrada do id (+0) da nação quando
/// a nação muda. Erro do original, copiado: o último grupo de cada tabela não é gravado. Sem memória: a original.
unsafe extern "C" fn database_build_nation_name_index() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let nacoes = *(0xae2350 as *const i32);
    let t = alocar((nacoes as u32).wrapping_mul(0x24));
    *(0xae2338 as *mut usize) = t;
    if t == 0 {
        // ponytail: a original pede de novo e, sem memória, abre a caixa e encerra o jogo
        return original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    }
    let mut k = 0i32;
    while k < *(0xae2350 as *const i32) {
        let e = t + k as usize * 0x24;
        for &(d, v) in &[(0usize, -1i32), (4, -1), (8, -1), (0xc, 0), (0x10, 0), (0x14, 0), (0x18, 0), (0x1c, 0), (0x20, 0)] {
            *((e + d) as *mut i32) = v;
        }
        k += 1;
    }
    // (quantidade, tabela, +primeira linha, +soma dos pesos, +quantidade), na ordem da original
    for &(qtd, tabela, d_inicio, d_soma, d_quantos) in &[
        (0xae239cusize, 0xae23f4usize, 4usize, 0x10usize, 0x1cusize),
        (0xae2398, 0xae23f0, 0, 0xc, 0x18),
        (0xae23a0, 0xae23f8, 8, 0x14, 0x20),
    ] {
        let (mut atual, mut inicio, mut soma, mut quantos) = (0usize, -1i32, 0i32, 0i32);
        let mut l = 0i32;
        while l < *(qtd as *const i32) {
            let linha = *(tabela as *const usize) + l as usize * 0x3c;
            let nacao = core::ptr::read_unaligned((linha + 0x37) as *const i32);
            if nacao > -1 && nacao < *(0xae2350 as *const i32) {
                let registro = (*(0xae23a8 as *const usize)).wrapping_add((nacao as usize).wrapping_mul(0x122));
                let peso = *((linha + 0x3b) as *const i8) as i32;
                if atual == registro {
                    soma = soma.wrapping_add(peso);
                    quantos += 1;
                } else {
                    if atual != 0 {
                        let id = *(atual as *const i32);
                        let e = (*(0xae2338 as *const usize)).wrapping_add(id.wrapping_mul(0x24) as usize);
                        *((e + d_soma) as *mut i32) = soma;
                        *((e + d_inicio) as *mut i32) = inicio;
                        *((e + d_quantos) as *mut i32) = quantos; // quantidade do grupo
                    }
                    inicio = l;
                    quantos = 1;
                    atual = registro;
                    soma = peso;
                }
            }
            l += 1;
        }
    }
}

/// As regiões dos comparadores de 0x541e60 e 0x544970: a tabela de clubes inteira (581 bytes por linha: a marca
/// +0x244 de todos e as casas dos pares) e as pessoas (110 bytes) nas casas dos clubes A e B dos pares de 0x540a50,
/// as únicas que as duas tiram, põem ou desligam (0x53c190 e 0x53bd10 só mexem no clube e na pessoa).
unsafe fn ab_team_regions() -> Vec<(usize, usize)> {
    let tabela = *(0xae23b4 as *const usize);
    let n = *(0xae235c as *const i32);
    let mut regioes = vec![(tabela, n.max(0) as usize * 581)];
    let mut pessoas = Vec::new();
    for &(a, b) in A_B_CLUB_PAIRS.iter() {
        for g in [a, b] {
            let linha = *(g as *const i32);
            if linha < 0 || linha >= n {
                continue;
            }
            let c = tabela + linha as usize * 581;
            for &(d, k) in &[(0xbfusize, 1usize), (0xc3, 3), (0xcf, 1), (0xd3, 1), (0xd7, 50), (0x19f, 5), (0x1b3, 7), (0x1cf, 3)] {
                for j in 0..k {
                    let p = core::ptr::read_unaligned((c + d + 4 * j) as *const usize);
                    if p != 0 {
                        pessoas.push(p);
                    }
                }
            }
        }
    }
    pessoas.sort_unstable();
    pessoas.dedup();
    regioes.extend(pessoas.into_iter().map(|p| (p, 110)));
    regioes
}

/// Original e cópia lado a lado sobre as regiões dos times A e B; a mensagem dá o primeiro byte que difere (a
/// tabela de clubes tem 8 MB: as fotos não vão para a mensagem nem voltam às sobras).
unsafe fn comparar_times_a_e_b(nome: &'static str, original: impl FnOnce(), rust: impl FnOnce()) {
    let regioes = ab_team_regions();
    let (_, _, igual, mut mo, mut mr) = crate::comparison::side_by_side(&regioes, original, rust);
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| {
        x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k]))
    });
    PLACAR.record(igual, || match onde {
        Some((a, x, y)) => format!("{}: diverge em {:x} original={:02x} rust={:02x}", nome, a, x, y),
        None => format!("{}: diverge", nome),
    });
    mo.0.clear();
    mr.0.clear();
}

unsafe extern "C" fn comparar_database_trim_senior_and_b_team_squads() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    comparar_times_a_e_b("database_trim_senior_and_b_team_squads", || o(), || database_trim_senior_and_b_team_squads());
}

unsafe extern "C" fn comparar_database_merge_b_team_staff() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    comparar_times_a_e_b("database_merge_b_team_staff", || o(), || database_merge_b_team_staff());
}

/// Comparador: a original monta o índice; a cópia monta outro (novo malloc), os dois são comparados byte a byte, o
/// da cópia é liberado (free 0x9452ca) e o da original volta a [0xae2338]. Muda a heap do modo de comparação (a
/// impressão dele não é critério).
unsafe extern "C" fn comparar_database_build_nation_name_index() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    let t = *(0xae2338 as *const usize);
    let n = (*(0xae2350 as *const i32)).max(0) as usize * 0x24;
    if t == 0 || n == 0 {
        return;
    }
    database_build_nation_name_index();
    let r = *(0xae2338 as *const usize);
    let original = core::slice::from_raw_parts(t as *const u8, n);
    let copia = core::slice::from_raw_parts(r as *const u8, n);
    let onde = original.iter().zip(copia).position(|(a, b)| a != b);
    PLACAR.record(onde.is_none(), || {
        let k = onde.unwrap_or(0) & !3;
        format!("database_build_nation_name_index: diverge na nação {} +{:#x} original={} rust={}", k / 0x24, k % 0x24,
            core::ptr::read_unaligned((t + k) as *const i32), core::ptr::read_unaligned((r + k) as *const i32))
    });
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(r);
    *(0xae2338 as *mut usize) = t;
}
