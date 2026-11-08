// ---------- %%LEVA%%a leva: a criação do jogo: gerar a comissão que falta, cortar a tabela de clubes e podar o histórico de competições (Database.cpp) ----------

/// A caixa de erro do Database.cpp: "v%s %s.%s %d" (0x90d130 com nome e extensão de Database.cpp e a `linha`) em
/// 0xae24d0, a caixa "Error" (0x5e8290) e [0xb67a34] = 0. O jogo segue.
unsafe fn caixa_do_banco_dbn1(linha: u32) {
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

/// A caixa do Database.cpp seguida do fim do jogo (0x5cc7d0, 0x5a8f60 e exit(−1)), como a original sem memória.
unsafe fn banco_fatal_dbn1(linha: u32) -> ! {
    let antes: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let depois: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    caixa_do_banco_dbn1(linha);
    antes();
    depois();
    sair(-1)
}

unsafe fn gi_dbn1(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn rd_dbn1(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Clube da nação da chave 0x9cf2e4 com reputação (+0x80, 16 bits com sinal) até 500: 0x527240 e 0x5267c0 o tratam à parte.
unsafe fn fraco_da_chave_2e4_dbn1(c: usize) -> bool {
    let nac = rd_dbn1(c + 0x53);
    nac != 0 && gi_dbn1(nac) == gi_dbn1(0x9cf2e4) && core::ptr::read_unaligned((c + 0x80) as *const i16) <= 500
}

/// A conferência inicial da 0x527240: alguma pessoa de cargo 8 (+0x3d) com clube (+0x39) que não a tem nas 5 casas +0x19f.
unsafe fn cargo_8_fora_do_clube_dbn1() -> bool {
    let mut i = 0i32;
    while i < gi_dbn1(0xae2364) {
        let s = rd_dbn1(0xae23bc) + i as usize * 110;
        let c = rd_dbn1(s + 0x39);
        if *((s + 0x3d) as *const u8) == 8 && c != 0 && !(0..5).any(|j| rd_dbn1(c + 0x19f + 4 * j) == s) {
            return true;
        }
        i += 1;
    }
    false
}

/// Uma pessoa do pessoal extra da 0x527240: a linha [0xae2364] − gerados − 17 (das reservas, de trás para a frente),
/// primeiro e segundo nome da nação (0x539c60, 0x53a1b0) e o nome comum: a linha guardada em [0x9b59b4] ou, sem ela
/// (−1), a primeira linha de nome comum vazia (guardada) ou, sem nenhuma, uma sorteada entre todas (sem guardar).
unsafe fn pessoa_extra_dbn1(nacao: usize, gerados: i32) -> usize {
    let primeiro: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let segundo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let s = rd_dbn1(0xae23bc).wrapping_add((gi_dbn1(0xae2364) - gerados - 0x11) as usize * 110);
    core::ptr::write_unaligned((s + 4) as *mut usize, primeiro(nacao));
    core::ptr::write_unaligned((s + 8) as *mut usize, segundo(nacao));
    let comum = |k: i32| rd_dbn1(0xae23f8).wrapping_add((k as usize).wrapping_mul(0x3c));
    let guardada = gi_dbn1(0x9b59b4);
    let nome = if guardada != -1 {
        comum(guardada)
    } else {
        let n = gi_dbn1(0xae23a0);
        match (0..n).find(|&j| *(comum(j) as *const u8) == 0) {
            Some(j) => {
                *(0x9b59b4 as *mut i32) = j;
                comum(j)
            }
            None => comum(sorteio(n)),
        }
    };
    core::ptr::write_unaligned((s + 0xc) as *mut usize, nome);
    s
}

/// A data de nascimento da pessoa extra (+0x10, 0x54a130): ano 1940 + sorteio(26), mês sorteio(11), dia sorteio(27) + 1,
/// sorteados nessa ordem.
unsafe fn nascimento_extra_dbn1(s: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    let ano = sorteio(0x1a) + 0x794; // 1940 + até 25
    let mes = sorteio(0xb);
    let dia = sorteio(0x1b) + 1;
    acertar_data(s + 0x10, dia as u32, mes as u32, ano as u32, -1);
}

/// { 0x527240 } (cdecl, Database.cpp): `database_generate_extra_non_playing_staff()`, uma vez na criação (chamada pela
/// 0x524d70 no começo dos cortes). Confere as pessoas de cargo 8 (caixa 0x62f na primeira fora das casas do clube, e
/// segue). Depois, em cada clube que não é seleção (id < [0xae235c] − 2 × [0xae2350]), não é fraco da chave 0x9cf2e4,
/// tem nação escolhida (+0x11c) e não é time A de um par (0x540a50 com 1): completa técnicos (+0x19f, 5, cargo 8),
/// olheiros (+0x1b3, 7, cargo 9) e fisioterapeutas (+0x1cf, 3, cargo 10) até (reputação + 500) × casas / 10000, entre 1
/// e o número de casas, com pessoas das reservas (pessoa_extra_dbn1, nação +0x1a, +0x5f = 1, nascimento sorteado) postas
/// na primeira casa livre. Por fim, cada clube sem treinador (+0xcf) que seja seleção ou (com nação, divisão +0x57, nação
/// escolhida e reputação ≠ 0) ou reputação > 7500, fora os fracos da 0x9cf2e4, ganha um treinador (cargo 5, +0x28 = 5)
/// sem clube e sem seleção (não vai para a casa). Mais de 23.000: caixa (0x65f, 0x6aa, 0x6f5, 0x734) e 0. Grava "generated
/// %ld extra non playing staff" em 0xde1f64 e devolve 1. A local do par só é lida quando 0x540a50 devolve parceiro.
unsafe extern "C" fn database_generate_extra_non_playing_staff() -> u32 {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let gi = |a: usize| gi_dbn1(a);
    let rd = |a: usize| rd_dbn1(a);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16) as i32;
    let byte = |a: usize| *(a as *const u8);
    if cargo_8_fora_do_clube_dbn1() {
        caixa_do_banco_dbn1(0x62f);
    }
    let selecao = |c: usize| gi(c) >= gi(0xae235c) - 2 * gi(0xae2350);
    let mut gerados = 0i32;
    for &(d, k, cargo, linha) in &[(0x19fusize, 5usize, 8u8, 0x65fu32), (0x1b3, 7, 9, 0x6aa), (0x1cf, 3, 10, 0x6f5)] {
        let mut i = 0i32;
        while i < gi(0xae235c) {
            let c = rd(0xae23b4) + i as usize * 581;
            i += 1;
            if fraco_da_chave_2e4_dbn1(c) || selecao(c) {
                continue;
            }
            let nac = rd(c + 0x53);
            if nac == 0 || byte(nac + 0x11c) == 0 {
                continue;
            }
            let mut eh_a = 0u32;
            if parceiro(c, &mut eh_a, 1) != 0 && eh_a != 0 {
                continue;
            }
            let mut tem = (0..k).filter(|&j| rd(c + d + 4 * j) != 0).count() as i32;
            let alvo = ((rep(c) + 500) * k as i32 / 10000).clamp(1, k as i32); // a comissão pela reputação
            while tem < alvo {
                tem += 1;
                if gerados >= 23000 {
                    caixa_do_banco_dbn1(linha);
                    return 0;
                }
                let s = pessoa_extra_dbn1(rd(c + 0x53), gerados);
                core::ptr::write_unaligned((s + 0x1a) as *mut usize, rd(c + 0x53));
                *((s + 0x5f) as *mut u8) = 1;
                nascimento_extra_dbn1(s);
                if let Some(j) = (0..k).find(|&j| rd(c + d + 4 * j) == 0) {
                    core::ptr::write_unaligned((c + d + 4 * j) as *mut usize, s);
                    core::ptr::write_unaligned((s + 0x39) as *mut usize, c);
                    *((s + 0x3d) as *mut u8) = cargo;
                }
                gerados += 1;
            }
        }
    }
    let mut i = 0i32;
    while i < gi(0xae235c) {
        let c = rd(0xae23b4) + i as usize * 581;
        i += 1;
        if fraco_da_chave_2e4_dbn1(c) {
            continue;
        }
        let nac = rd(c + 0x53);
        let vale = selecao(c) || (nac != 0 && rd(c + 0x57) != 0 && byte(nac + 0x11c) != 0 && rep(c) != 0) || rep(c) > 7500;
        if !vale || rd(c + 0xcf) != 0 {
            continue;
        }
        if gerados >= 23000 {
            caixa_do_banco_dbn1(0x734);
            return 0;
        }
        let s = pessoa_extra_dbn1(nac, gerados);
        *((s + 0x5f) as *mut u8) = 1;
        *((s + 0x3d) as *mut u8) = 5; // treinador sem clube
        core::ptr::write_unaligned((s + 0x39) as *mut usize, 0);
        core::ptr::write_unaligned((s + 0x24) as *mut usize, 0);
        *((s + 0x28) as *mut u8) = 5;
        nascimento_extra_dbn1(s);
        gerados += 1;
    }
    formatar(0xde1f64, 0x9b5b10, gerados);
    1
}

/// { 0x5267c0 } (cdecl, Database.cpp): `database_trim_club_table(tamanho)`, uma vez na criação (create_game). Pequena =
/// tamanho 1 ou 2. Por nação (calloc de 0xd0 bytes por nação: 50 ids, quantidade +0xc8, índice do menor +0xcc), os
/// clubes de maior reputação (+0x80, 16 bits com sinal): até 20 (15 na pequena); nação não escolhida (+0x11c) e fora das
/// 15 chaves de nação: até 5 (tamanho 2) ou 11 (tamanho 1). Ficam também os clubes das pessoas (+0x39, +0x24), os dos
/// históricos de competição (+0xa, +0xe, +0x12, na faixa) e, fora da pequena, os do histórico de pessoas com nação
/// escolhida; as seleções; as chaves de clube (0x60ba80 recebe cada id com a linha nova e devolve 1 se é chave); e os
/// de nação escolhida, menos (na pequena) os fracos da 0x9cf2e4. Troca os ponteiros de clube (clube +0xb3/+0xb7/+0xbb,
/// pessoa +0x39/+0x24, preferências +4..+0x18, histórico de pessoas +0xa, histórico de competições +0xa..+0x16) pelos
/// novos (nulo o que sai), monta [0xae23fc] (linha nova → id antigo, malloc), compacta a tabela (id = linha nova),
/// realloc no lugar (outro endereço: caixa 0x4ca e fim) e grava "Club table has %d entries" em 0xde1f64. Sem [0xae23a8]
/// (caixa) ou sem memória no new: a original; sem memória nos calloc: caixa e fim, como a original.
unsafe extern "C" fn database_trim_club_table(tamanho: u32) {
    if *(0xae23a8 as *const usize) == 0 {
        return orig::<unsafe extern "C" fn(u32)>(%%I1%%)(tamanho);
    }
    crate::comparacao::ramo("criação cortar a tabela de clubes");
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let remapear: unsafe extern "C" fn(i32, i32) -> u32 = core::mem::transmute(0x60ba80usize);
    let gi = |a: usize| gi_dbn1(a);
    let rd = |a: usize| rd_dbn1(a);
    let wi = |a: usize, v: i32| core::ptr::write_unaligned(a as *mut i32, v);
    let wr = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut usize, v);
    let rep = |c: usize| core::ptr::read_unaligned((c + 0x80) as *const i16);
    let rep_id = |id: i32| rep(rd(0xae23b4).wrapping_add((id as usize).wrapping_mul(581)));
    let byte = |a: usize| *(a as *const u8);
    let novos = novo((gi(0xae235c) as u32).wrapping_mul(4));
    if novos == 0 {
        return;
    }
    let t = tamanho as u8;
    let pequena = t == 2 || t == 1;
    let marcados = calloc(gi(0xae235c) as u32, 4);
    if marcados == 0 {
        banco_fatal_dbn1(0x513);
    }
    let marcar = |id: i32| wi(marcados.wrapping_add((id as usize).wrapping_mul(4)), 1);
    let ultimo = rd(0xae23b4).wrapping_add((gi(0xae235c) as usize).wrapping_mul(581)).wrapping_sub(581);
    let nacoes = calloc(gi(0xae2350) as u32, 0xd0);
    if nacoes == 0 {
        banco_fatal_dbn1(0x51b);
    }
    let limite = if pequena { 15 } else { 20 }; // clubes por nação
    let chaves = [0x9cf2e4usize, 0x9cf360, 0x9cf314, 0x9cf488, 0x9cf300, 0x9cf408, 0x9cf260, 0x9cf218, 0x9cf2c0, 0x9cf244,
        0x9cf338, 0x9cf36c, 0x9cf434, 0x9cf45c, 0x9cf4a8];
    let mut c = rd(0xae23b4);
    while c <= ultimo {
        let nac = rd(c + 0x53);
        if nac != 0 {
            let id = gi(nac);
            let l = nacoes.wrapping_add((id as usize).wrapping_mul(0xd0));
            let cont = gi(l + 0xc8);
            let forte = byte(nac + 0x11c) != 0 || chaves.iter().any(|&k| id == gi(k));
            let entra = forte || match t {
                2 => cont <= 4,
                1 => cont <= 10,
                _ => true,
            };
            if entra {
                if cont < limite {
                    wi(l + cont as usize * 4, gi(c));
                    if cont == 0 || rep(c) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                        wi(l + 0xcc, cont);
                    }
                    wi(l + 0xc8, cont + 1);
                } else if rep(c) > rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                    wi(l + gi(l + 0xcc) as usize * 4, gi(c));
                    for j in 0..cont {
                        if j == 0 || rep_id(gi(l + j as usize * 4)) < rep_id(gi(l + gi(l + 0xcc) as usize * 4)) {
                            wi(l + 0xcc, j);
                        }
                    }
                }
            }
        }
        c += 581;
    }
    let mut k = 0i32;
    while k < gi(0xae2350) {
        let l = nacoes + k as usize * 0xd0;
        for j in 0..gi(l + 0xc8) {
            marcar(gi(l + j as usize * 4));
        }
        k += 1;
    }
    livre(nacoes);
    let mut s = rd(0xae23bc);
    let fim = s.wrapping_add((gi(0xae2364) as usize).wrapping_mul(110)).wrapping_sub(110);
    while s <= fim {
        for &d in &[0x39usize, 0x24] { // o clube e a seleção da pessoa ficam
            let p = rd(s + d);
            if p != 0 {
                marcar(gi(p));
            }
        }
        s += 110;
    }
    let mut h = rd(0xae23d4);
    let fim = h.wrapping_add((gi(0xae237c) as usize).wrapping_mul(0x1a)).wrapping_sub(0x1a);
    while h <= fim {
        for &d in &[0xausize, 0xe, 0x12] {
            let p = rd(h + d);
            if p != 0 {
                let id = gi(p);
                if id < gi(0xae235c) && id >= 0 {
                    marcar(id);
                }
            }
        }
        h += 0x1a;
    }
    if !pequena {
        let mut h = rd(0xae23cc);
        let fim = h.wrapping_add((gi(0xae2374) as usize).wrapping_mul(0x11)).wrapping_sub(0x11);
        while h <= fim {
            let clube = rd(h + 0xa);
            let q = rd(h + 4);
            if clube != 0 && q != 0 {
                let nq = rd(q + 0x1a);
                let nc = rd(clube + 0x53);
                if (nq != 0 && byte(nq + 0x11c) != 0) || (nc != 0 && byte(nc + 0x11c) != 0) {
                    marcar(gi(clube));
                }
            }
            h += 0x11;
        }
    }
    let tabela = rd(0xae23b4);
    let mut destino = tabela;
    let mut c = tabela;
    while c <= ultimo {
        let id = gi(c);
        let fica = if remapear(id, (destino.wrapping_sub(tabela) as i32) / 581) != 0
            || gi(marcados.wrapping_add((id as usize).wrapping_mul(4))) != 0
            || id >= gi(0xae235c) - 2 * gi(0xae2350)
        {
            true
        } else {
            let nac = rd(c + 0x53);
            !(nac == 0 || byte(nac + 0x11c) == 0 || (gi(nac) == gi(0x9cf2e4) && rep(c) <= 500 && pequena))
        };
        let casa = novos.wrapping_add((id as usize).wrapping_mul(4));
        if fica {
            wr(casa, destino);
            destino += 581;
        } else {
            wr(casa, 0);
        }
        c += 581;
    }
    livre(marcados);
    // os ponteiros de clube pelos novos (o id antigo está em +0 do alvo, ainda no lugar velho)
    let trocar = |campo: usize| {
        let p = rd(campo);
        wr(campo, if p == 0 { 0 } else { rd(novos.wrapping_add((gi(p) as usize).wrapping_mul(4))) });
    };
    for &(cont, base, tam, campos) in &[
        (0xae235cusize, 0xae23b4usize, 581usize, &[0xb3usize, 0xb7, 0xbb][..]),
        (0xae2364, 0xae23bc, 110, &[0x39, 0x24][..]),
        (0xae2370, 0xae23c8, 0x34, &[4, 8, 0xc, 0x10, 0x14, 0x18][..]),
        (0xae2374, 0xae23cc, 0x11, &[0xa][..]),
        (0xae237c, 0xae23d4, 0x1a, &[0xa, 0xe, 0x12, 0x16][..]),
    ] {
        let mut i = 0i32;
        while i < gi(cont) {
            for &d in campos {
                trocar(rd(base) + i as usize * tam + d);
            }
            i += 1;
        }
    }
    let n = gi(0xae235c);
    let ficam = (0..n).filter(|&i| rd(novos + i as usize * 4) != 0).count();
    let ids = alocar((ficam as u32) << 2);
    *(0xae23fc as *mut usize) = ids;
    if ids == 0 {
        apagar(novos);
        return;
    }
    let mut linha = 0i32;
    for i in 0..n {
        let d = rd(novos + i as usize * 4);
        if d != 0 {
            // para a frente sobre a própria tabela (destino ≤ origem), como o rep movsd da original
            core::ptr::copy((rd(0xae23b4) + i as usize * 581) as *const u8, d as *mut u8, 581);
            wi(rd(0xae23fc) + linha as usize * 4, i);
            wi(d, linha);
            linha += 1;
        }
    }
    *(0xae235c as *mut i32) = linha;
    let r = realocar(rd(0xae23b4), (linha as u32).wrapping_mul(581));
    *(0xae23b8 as *mut usize) = rd(0xae23b4).wrapping_add(((linha - gi(0xae2360)) as usize).wrapping_mul(581));
    if r != rd(0xae23b4) {
        banco_fatal_dbn1(0x4ca);
    }
    apagar(novos);
    formatar(0xde1f64, 0x9b5af4, gi(0xae235c));
}

/// { 0x539ae0 } (cdecl, Database.cpp): `database_prune_comp_history_table()`, uma vez na criação. Compacta o histórico
/// de competições de clube ([0xae23d4], [0xae237c] linhas de 26 bytes): fica a linha com competição (+4) cuja nação
/// (+0x5d) é nula ou escolhida (+0x11c), copiada para a frente na própria tabela. realloc (0x945667) ao tamanho novo,
/// que vira [0xae23d4] (sem conferir o endereço); [0xae237c] = linhas; [0xae23dc] (a parte das seleções) = tabela +
/// (linhas − [0xae2384]) × 26; qsort (0x945333) com 0x540780.
unsafe extern "C" fn database_prune_comp_history_table() {
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    crate::comparacao::ramo("criação podar o histórico de competições");
    let mut k = 0usize;
    let mut i = 0i32;
    while i < gi_dbn1(0xae237c) {
        let l = rd_dbn1(0xae23d4) + i as usize * 0x1a;
        let comp = rd_dbn1(l + 4);
        if comp != 0 {
            let nac = rd_dbn1(comp + 0x5d);
            if nac == 0 || *((nac + 0x11c) as *const u8) != 0 { // a linha fica
                core::ptr::copy(l as *const u8, (rd_dbn1(0xae23d4) + k * 0x1a) as *mut u8, 0x1a);
                k += 1;
            }
        }
        i += 1;
    }
    let t = realocar(rd_dbn1(0xae23d4), (k * 0x1a) as u32);
    let selecoes = (k as i32).wrapping_sub(gi_dbn1(0xae2384));
    *(0xae23d4 as *mut usize) = t;
    *(0xae237c as *mut i32) = k as i32;
    *(0xae23dc as *mut usize) = t.wrapping_add((selecoes as usize).wrapping_mul(0x1a));
    ordenar(t, k as u32, 0x1a, 0x540780);
}

/// O primeiro byte que difere entre as fotos da original e da cópia (endereço, original, cópia).
fn diferenca_dbn1(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k])))
}

/// Comparador de 0x527240: original e cópia lado a lado sobre as tabelas de clubes e de pessoas inteiras, [0x9b59b4], o
/// texto de 0xde1f64 e o estado do sorteador (a cópia sem contar sorteios). Com a pessoa de cargo 8 fora das casas
/// (caixa), só a original. O estouro das 23.000 (caixa) não é previsto aqui: a base de fábrica gera bem menos.
unsafe extern "C" fn comparar_database_generate_extra_non_playing_staff() -> u32 {
    let o = orig::<unsafe extern "C" fn() -> u32>(%%I0%%);
    if cargo_8_fora_do_clube_dbn1() {
        return o();
    }
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.push((0x9b59b4, 4));
    regioes.push((0xde1f64, 0x40));
    regioes.push((rd_dbn1(0xae23b4), gi_dbn1(0xae235c).max(0) as usize * 581));
    regioes.push((rd_dbn1(0xae23bc), gi_dbn1(0xae2364).max(0) as usize * 110));
    let (r, m, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, || o(),
        || crate::aleatorio::sem_contar(|| database_generate_extra_non_playing_staff()));
    let onde = diferenca_dbn1(&mo, &mr);
    PLACAR.registrar(igual && r == m, || match onde {
        Some((a, x, y)) => format!("database_generate_extra_non_playing_staff: original={} rust={}, diverge em {:x} original={:02x} rust={:02x}", r, m, a, x, y),
        None => format!("database_generate_extra_non_playing_staff: original={} rust={}", r, m),
    });
    // ponytail: 29 MB por foto; não voltam às sobras
    mo.0.clear();
    mr.0.clear();
    r
}

// ponytail: compacta e realoca a tabela de clubes, aloca [0xae23fc] e troca as chaves de clube; não roda duas
// vezes. Só a original; a cópia leva o ramo e é provada pela prova do save (o .qst tem a tabela cortada).
unsafe extern "C" fn comparar_database_trim_club_table(tamanho: u32) {
    orig::<unsafe extern "C" fn(u32)>(%%I1%%)(tamanho)
}

// ponytail: compacta e realoca o histórico de competições; não roda duas vezes. Só a original; a cópia é provada
// pela prova do save (o histórico podado vai para o .qst).
unsafe extern "C" fn comparar_database_prune_comp_history_table() {
    orig::<unsafe extern "C" fn()>(%%I2%%)()
}
