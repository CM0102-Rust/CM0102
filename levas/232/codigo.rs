// ---------- 232a leva: liberar os históricos de pessoas e de prêmios e crescer o histórico de competições (Database.cpp) ----------

/// Solta uma tabela do banco como 0x527ee0/0x527f20: `GlobalFree` (IAT 0x9670f4) com o banco no bloco
/// compartilhado ([0xae2340] ≠ 0), senão `free` 0x9452ca.
unsafe fn soltar_tabela_do_banco(tabela: usize) {
    if *(0xae2340 as *const u32) != 0 {
        let global_free: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(*(0x9670f4 as *const usize));
        global_free(tabela);
    } else {
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        free(tabela);
    }
}

/// { 0x527ee0 } (cdecl, Database.cpp): `database_free_staff_history_table()`: com a tabela do histórico de
/// pessoas ([0xae23cc], 17 bytes) presente, solta (0x9670f4 ou 0x9452ca, pelo modo do banco) e zera o ponteiro
/// e a contagem [0xae2374]. Chamada por PlayerStats::PlayerStats (0x7c0ccd): o histórico passa a viver no
/// `player stats history.tmp`. Sem quadro de pilha.
unsafe extern "C" fn liberar_historico_de_pessoas() {
    let tabela = *(0xae23cc as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparacao::ramo("banco liberar histórico de pessoas");
    soltar_tabela_do_banco(tabela);
    *(0xae23cc as *mut usize) = 0;
    *(0xae2374 as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// ponytail: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_liberar_historico_de_pessoas() {
    orig::<unsafe extern "C" fn()>(661)()
}

/// { 0x527f20 } (cdecl, Database.cpp): `database_free_staff_comp_history_table()`: o mesmo para o histórico
/// de prêmios ([0xae23e4], 58 bytes; contagem [0xae238c]). Chamada por AWARD_MANAGER::AWARD_MANAGER (0x413360).
unsafe extern "C" fn liberar_historico_de_premios() {
    let tabela = *(0xae23e4 as *const usize);
    if tabela == 0 {
        return;
    }
    crate::comparacao::ramo("banco liberar histórico de prêmios");
    soltar_tabela_do_banco(tabela);
    *(0xae23e4 as *mut usize) = 0;
    *(0xae238c as *mut u32) = 0;
}

/// Comparador: liberar duas vezes soltaria o mesmo bloco duas vezes.
// ponytail: só a original; a prova é o .qst (a tabela sai do save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_liberar_historico_de_premios() {
    orig::<unsafe extern "C" fn()>(662)()
}

/// { 0x527f60 } (cdecl, Database.cpp): `database_grow_comp_history_table(n)`: `realloc` 0x945667 do histórico de
/// competições ([0xae23d4], 26 bytes) para [0xae237c] + `n` linhas; sem memória devolve 0 e nada muda. Senão soma
/// `n` à contagem, guarda o bloco, refaz o ponteiro da parte das seleções [0xae23dc] (a partir da linha [0xae2384])
/// e limpa as `n` linhas novas (+0 = o índice da linha, o resto zero); devolve 1. Chamada por
/// comp_util_add_history_record (0x4afd2c), no fim de competição. Sem quadro de pilha.
unsafe extern "C" fn crescer_historico_de_competicoes(n: i32) -> u32 {
    crate::comparacao::ramo("banco crescer histórico de competições");
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let total = (*(0xae237c as *const i32)).wrapping_add(n);
    let nova = realloc(*(0xae23d4 as *const usize), (total as u32).wrapping_mul(26));
    if nova == 0 {
        return 0;
    }
    *(0xae237c as *mut i32) = total;
    *(0xae23d4 as *mut usize) = nova;
    let selecoes = total.wrapping_sub(*(0xae2384 as *const i32));
    *(0xae23dc as *mut usize) = nova.wrapping_add((selecoes as usize).wrapping_mul(26));
    for k in 1..=n {
        let i = total.wrapping_sub(k);
        let linha = nova.wrapping_add((i as usize).wrapping_mul(26)) as *mut u8;
        core::ptr::write_bytes(linha, 0, 26);
        core::ptr::write_unaligned(linha as *mut i32, i); // a linha nova guarda o próprio índice
    }
    1
}

/// Comparador: o `realloc` não se desfaz (crescer duas vezes deixaria a tabela com 2n linhas a mais).
// ponytail: só a original; a prova é o .qst (a tabela vai para o save) e o ramo da cópia no modo Rust.
unsafe extern "C" fn comparar_crescer_historico_de_competicoes(n: i32) -> u32 {
    orig::<unsafe extern "C" fn(i32) -> u32>(663)(n)
}


// ---------- 232a leva: as ordens dos clubes pela reputação e dos elencos pela habilidade (Database.cpp) ----------

/// { 0x540020 } (cdecl, Database.cpp): comparação do qsort do vetor de ids de clube (0x53db40): pela reputação
/// (+0x80, palavra com sinal) do clube ([0xae23b4] + id × 581; id −1 ou menor = reputação 0), a maior primeiro:
/// a de `a` menor que a de `b`: 1; maior: −1; iguais: 0.
unsafe extern "C" fn ordem_dos_clubes_pela_reputacao(a: usize, b: usize) -> i32 {
    let clubes = *(0xae23b4 as *const usize);
    let reputacao = |p: usize| {
        let id = *(p as *const i32);
        if id > -1 { core::ptr::read_unaligned(clubes.wrapping_add((id as usize).wrapping_mul(581)).wrapping_add(0x80) as *const i16) } else { 0 }
    };
    let (ra, rb) = (reputacao(a), reputacao(b));
    if ra < rb {
        1 // a de menor reputação depois
    } else if ra > rb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn comparar_ordem_dos_clubes_pela_reputacao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(664)(a, b);
    let n = ordem_dos_clubes_pela_reputacao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_clubes_pela_reputacao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x5402f0 } (cdecl, Database.cpp): comparação do qsort dos elencos (corte do conjunto A+B em 0x541e60 e as
/// seleções da MLS em 0x90a980): ponteiros de pessoa; nula vai para o fim (a nula e b não: 1; a não e b nula: −1;
/// as duas: 0); depois pela palavra +0xb do jogador (pessoa +0x61; sem jogador, 0), a maior primeiro (b − a).
unsafe extern "C" fn ordem_do_elenco_pela_habilidade(a: usize, b: usize) -> i32 {
    let pa = *(a as *const usize);
    let pb = *(b as *const usize);
    if pa == 0 {
        return (pb != 0) as i32;
    }
    if pb == 0 {
        return -1;
    }
    let habilidade = |p: usize| {
        let j = core::ptr::read_unaligned((p + 0x61) as *const usize);
        if j != 0 { core::ptr::read_unaligned((j + 0xb) as *const i16) } else { 0 }
    };
    habilidade(pb) as i32 - habilidade(pa) as i32 // a maior primeiro
}

unsafe extern "C" fn comparar_ordem_do_elenco_pela_habilidade(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(665)(a, b);
    let n = ordem_do_elenco_pela_habilidade(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_elenco_pela_habilidade({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x549c30 } (cdecl, Database.cpp): comparação do qsort do elenco na força do time (club_squad_strength_band
/// 0x549a30): ponteiros de pessoa; sem pessoa ou sem jogador (+0x61) em `a`: 1 (mesmo com `b` igual); em `b`:
/// −1; depois pela habilidade atual (jogador +5, palavra com sinal) e, empatada, pela palavra +0xb, a maior
/// primeiro (b − a).
unsafe extern "C" fn ordem_do_elenco_pela_habilidade_atual(a: usize, b: usize) -> i32 {
    let jogador = |p: usize| {
        let s = *(p as *const usize);
        if s == 0 { 0 } else { core::ptr::read_unaligned((s + 0x61) as *const usize) }
    };
    let ja = jogador(a);
    if ja == 0 {
        return 1;
    }
    let jb = jogador(b);
    if jb == 0 {
        return -1;
    }
    let palavra = |j: usize, d: usize| core::ptr::read_unaligned((j + d) as *const i16) as i32;
    if palavra(ja, 5) != palavra(jb, 5) {
        palavra(jb, 5) - palavra(ja, 5) // a habilidade atual maior primeiro
    } else {
        palavra(jb, 0xb) - palavra(ja, 0xb)
    }
}

unsafe extern "C" fn comparar_ordem_do_elenco_pela_habilidade_atual(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(666)(a, b);
    let n = ordem_do_elenco_pela_habilidade_atual(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_elenco_pela_habilidade_atual({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 232a leva: a normalização física e geral dos atributos do jogador e o segundo nome pelo peso (Database.cpp) ----------

/// TRAINING_MANAGER::remove_or_restore_training_progress (0x8b3640, thiscall com o gerente [0xde1f44]): tira
/// (`devolver` = 0) ou devolve (1) o progresso do treino dos atributos da `pessoa`. Sem gerente, nada.
unsafe fn progresso_do_treino(pessoa: usize, devolver: u32) {
    let gerente = *(0xde1f44 as *const usize);
    if gerente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b3640usize);
        f(gerente, pessoa, devolver);
    }
}

/// O fim comum de 0x538bc0 e 0x538ec0 com o jogador +0xf alto: tira de +0x2a e de +0x3a a média dos dois
/// (bytes com sinal, divisão truncada).
unsafe fn tirar_a_media_de_2a_e_3a(jogador: usize) {
    let (a, b) = ((jogador + 0x2a) as *mut u8, (jogador + 0x3a) as *mut u8);
    let media = ((*a as i8 as i32 + *b as i8 as i32) / 2) as u8;
    *a = (*a).wrapping_sub(media);
    *b = (*b).wrapping_sub(media);
}

/// O reequilíbrio comum de 0x538bc0 e 0x538ec0 sobre o vetor de atributos do jogador (0x53a610: palavra com
/// sinal + peso por célula), nas células de `inicio` (fim dos lados + 2) até antes de `fim` (fim dos
/// ponderados), sem as de `pular`; duas passadas: (1) limite 30, +5 por volta, até 12 células com |valor| ≤
/// limite (marcadas); a de maior valor fora do limite também é marcada; (2) soma `alvo` − média às marcadas
/// (presas em −125..125); na 1ª passada, com o jogador +0xf < 15, as não marcadas positivas entram no
/// excesso (as células 0x11 e 0x12 por `excesso`), as demais são marcadas; com a célula 0 < 15, as de peso 1
/// viram −120; (3) com mais de uma no excesso e soma > 85, tira (soma − 85) / quantas das não marcadas
/// (mínimo 0). O quadro da original não é lido sem valor: as células do intervalo vêm todas de 0x53a610.
unsafe fn reequilibrar_vetor_do_jogador(v: *mut u8, inicio: i8, fim: i8, jogador: usize, alvo: i32, pular: &[i8],
                                        excesso: impl Fn(i16) -> i32) {
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let peso = |i: i8| *v.add(3 * i as usize + 2);
    let fora = |c: i8| pular.contains(&c);
    let jovem = *((jogador + 0xf) as *const i8) < 0xf;
    let mut marca = [0u32; 64];
    for passo in 0..2 {
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limite = 0x1ei32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if inicio < fim {
                let mut c = inicio;
                while c < fim {
                    if !fora(c) {
                        let x = w(c);
                        if (x as i32).abs() <= limite as i16 as i32 {
                            total = total.wrapping_add(x as i32);
                            n = n.wrapping_add(1);
                            marca[c as usize] = 1;
                        } else {
                            marca[c as usize] = 0;
                            if melhor == -1 || x > w(melhor) {
                                melhor = c;
                            }
                        }
                    }
                    c += 1;
                }
                if n >= 12 {
                    break;
                }
            }
            // ponytail: com inicio >= fim (ou menos de 12 células) o original roda para sempre; a cópia também
            limite += 5;
        }
        if melhor > -1 {
            marca[melhor as usize] = 1;
        }
        let delta = alvo.wrapping_sub(total / n as i32) as i16;
        let mut c = inicio;
        while c < fim {
            if !fora(c) && c != melhor {
                if marca[c as usize] == 1 {
                    pw(c, w(c).wrapping_add(delta).clamp(-0x7d, 0x7d));
                } else if passo == 0 && w(c) > 0 && jovem {
                    quantas += 1;
                    soma = soma.wrapping_add(if c == 0x11 || c == 0x12 { excesso(w(c)) } else { w(c) as i32 });
                } else {
                    marca[c as usize] = 1;
                }
                if w(0) < 0xf && peso(c) == 1 {
                    pw(c, -0x78);
                }
            }
            c += 1;
        }
        if quantas > 1 && soma > 0x55 {
            let tirar = ((soma - 0x55) / quantas) as i16;
            let mut c = inicio;
            while c < fim {
                if !fora(c) && marca[c as usize] != 1 {
                    let x = w(c).wrapping_sub(tirar);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
            }
        }
    }
}

/// O vetor de atributos do `jogador` (0x53a610) com os fins de grupo (lados, posições, ponderados).
unsafe fn vetor_do_jogador_para_reequilibrar(jogador: usize, pessoa: usize, v: *mut u8) -> (u8, u8) {
    let preencher: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8, *mut u8) -> usize = core::mem::transmute(0x53a610usize);
    let (mut lados, mut posicoes, mut ponderados) = (0u8, 0u8, 0u8);
    preencher(jogador, pessoa, v as usize, &mut lados, &mut posicoes, &mut ponderados);
    (lados, ponderados)
}

/// { 0x538ec0 } (cdecl, Database.cpp): `staff_normalise_player_physical_attributes(pessoa)`: com o jogador
/// (pessoa +0x61), tira o progresso do treino (0x8b3640); corte = (habilidade +5 / 2 − ftol(soma ponderada dos
/// físicos × [0x969528]) + 0x37) / 2, a soma com as constantes do .rdata (+0x36, +0x2f, +0x2e, +0x1b, +0x3f,
/// +0x40, o menor de +0x3b/+0x30, mais +0x44); corte < −14: soma max(corte / 15, −2) a +0x1b, +0x36, +0x40 e
/// +0x44 (no mínimo 1); alvo = corte − 20 − min((+0x36 × +0x1b) / 2 − habilidade, 25) quando passa da
/// habilidade, preso em −40..0; vetor (0xff81/0x81) pelo 0x53a610, `reequilibrar_vetor_do_jogador` sem as
/// células 0x18 e 0x1e (0x11/0x12 contam em dobro), volta pelo 0x53aa10; jogador +0xf = 20: tira as médias
/// de +0x2a/+0x3a e o terço de +0x1e, +0x24, +0x39; devolve o progresso do treino.
unsafe extern "C" fn normalizar_atributos_fisicos_do_jogador(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if jogador == 0 {
        return;
    }
    progresso_do_treino(pessoa, 0);
    let b = |d: usize| *((jogador + d) as *const i8) as i32;
    let f = |x: i32, k: usize| ftol(x as f64 * f64_em(k));
    let menor = b(0x3b).min(b(0x30));
    let t = f(b(0x36), 0x968940) - f(b(0x2f), 0x969548) - f(b(0x2e), 0x969540) - f(b(0x1b), 0x969538)
        + (f(b(0x3f), 0x9689b0) - f(b(0x40), 0x969548) - f(menor, 0x969530)) + b(0x44);
    let habilidade = core::ptr::read_unaligned((jogador + 5) as *const i16);
    let corte = (habilidade as i32 / 2 - f(t, 0x969528) + 0x37) / 2;
    if corte < -14 {
        let d = (corte / 15).max(-2);
        for off in [0x1b, 0x36, 0x40, 0x44] {
            let x = d + b(off);
            *((jogador + off) as *mut u8) = if x < 1 { 1 } else { x as u8 };
        }
    }
    let mut s = corte - 0x14;
    let produto = ((b(0x36) * b(0x1b)) / 2) as i16;
    if produto > habilidade {
        s -= (produto as i32 - habilidade as i32).min(0x19);
    }
    let alvo = if s < -40 { -40 } else if s > 0 { 0 } else { s }; // alvo físico
    let mut vetor = [0u8; 63 * 3];
    for c in vetor.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = vetor.as_mut_ptr();
    let (lados, ponderados) = vetor_do_jogador_para_reequilibrar(jogador, pessoa, v);
    reequilibrar_vetor_do_jogador(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, alvo, &[0x1e, 0x18],
        |x| 2 * x as i32);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53aa10usize);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) == 0x14 {
        tirar_a_media_de_2a_e_3a(jogador);
        let terco = ((b(0x1e) + b(0x24) + b(0x39)) / 3) as u8;
        for off in [0x1e, 0x39, 0x24] {
            *((jogador + off) as *mut u8) = (*((jogador + off) as *const u8)).wrapping_sub(terco);
        }
    }
    progresso_do_treino(pessoa, 1);
}

/// Comparador de 0x538bc0 e 0x538ec0: o jogador inteiro (70 bytes) e o byte +0x58 da pessoa, com o gerente
/// do treino ([0xde1f44]) zerado nos dois lados (o tirar e devolver do progresso mexe nas tabelas do treino,
/// fora das regiões); com gerente, a original roda depois de verdade a partir do mesmo estado.
unsafe fn comparar_normalizacao_do_jogador(o: unsafe extern "C" fn(usize), rust: unsafe extern "C" fn(usize), pessoa: usize,
                                           nome: &'static str) {
    let jogador = if pessoa == 0 { 0 } else { core::ptr::read_unaligned((pessoa + 0x61) as *const usize) };
    if jogador == 0 {
        return o(pessoa); // não faz nada: só a original
    }
    let regioes = [(jogador, 70), (pessoa + 0x58, 1)];
    let gerente = *(0xde1f44 as *const usize);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    *(0xde1f44 as *mut usize) = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(pessoa), || rust(pessoa));
    *(0xde1f44 as *mut usize) = gerente;
    PLACAR.registrar(igual, || format!("{}({:x}) {:x?} x {:x?}", nome, pessoa, mo, mr));
    if gerente != 0 {
        antes.escrever();
        o(pessoa);
    }
}

unsafe extern "C" fn comparar_normalizar_atributos_fisicos_do_jogador(pessoa: usize) {
    comparar_normalizacao_do_jogador(orig::<unsafe extern "C" fn(usize)>(667), normalizar_atributos_fisicos_do_jogador, pessoa,
        "normalizar_atributos_fisicos_do_jogador");
}

/// { 0x53a0e0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) pelo peso. Nação com
/// mais de 10 segundos nomes (`indice_de_nomes_da_nacao` +0x1c): 3 sorteios na faixa dela (a partir de +4),
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com até
/// 10: sorteio na tabela inteira ([0xae239c], no máximo 0x7fff). Só o rand() da CRT. Chamada por
/// OfficialsManager::add_random_officials (0x79c6b4), na criação dos árbitros do jogo novo.
unsafe extern "C" fn segundo_nome_pelo_peso(nacao: usize) -> usize {
    let linha = |i: i32| (*(0xae23f4 as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = indice_de_nomes_da_nacao(nacao);
        let quantos = *((e + 0x1c) as *const i32);
        if quantos > 0xa {
            let inicio = *((e + 4) as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = sorteio_escalado(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido); // o segundo nome de maior peso
        }
    }
    linha(sorteio_escalado(total_de_nomes_limitado(0xae239c)))
}

unsafe extern "C" fn comparar_segundo_nome_pelo_peso(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(668);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(nacao),
        || crate::aleatorio::sem_contar(|| segundo_nome_pelo_peso(nacao)));
    PLACAR.registrar(igual && r == n, || format!("segundo_nome_pelo_peso({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// { 0x538bc0 } (cdecl, Database.cpp): `staff_normalise_player_attributes(pessoa)`: com o jogador (pessoa
/// +0x61), tira o progresso do treino (0x8b3640), monta o vetor pelo 0x53a610 (sem pré-encher: as células
/// usadas vêm todas dele), `reequilibrar_vetor_do_jogador` com alvo −20 em todas as células do intervalo
/// (0x11/0x12 contam ftol(valor × [0x968f98])), volta pelo 0x53aa10; jogador +0xf ≥ 15: tira as médias de
/// +0x2a/+0x3a; devolve o progresso do treino. Roda na criação (depois de 0x538ec0) e na carga e no treino.
unsafe extern "C" fn normalizar_atributos_do_jogador(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if jogador == 0 {
        return;
    }
    progresso_do_treino(pessoa, 0);
    let mut vetor = [0u8; 63 * 3];
    let v = vetor.as_mut_ptr();
    let (lados, ponderados) = vetor_do_jogador_para_reequilibrar(jogador, pessoa, v);
    reequilibrar_vetor_do_jogador(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, -20, &[],
        |x| ftol(x as f64 * f64_em(0x968f98)));
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53aa10usize);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) >= 0xf {
        tirar_a_media_de_2a_e_3a(jogador);
    }
    progresso_do_treino(pessoa, 1);
}

unsafe extern "C" fn comparar_normalizar_atributos_do_jogador(pessoa: usize) {
    comparar_normalizacao_do_jogador(orig::<unsafe extern "C" fn(usize)>(669), normalizar_atributos_do_jogador, pessoa,
        "normalizar_atributos_do_jogador");
}

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
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x946aaeusize);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(0x946a22usize);
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
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
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

// ---------- 232a leva: o conversor das competições da versão 1 e dois nomes em inglês (Db_files.cpp, Database.cpp) ----------

/// Competição (107 bytes) no club_comp.dat / nation_comp.dat versão 1 (104 no arquivo): `(deslocamento,
/// tamanho, quantos)` na ordem do arquivo; buracos em +0x37 e +0x52; tamanho 0 = a reputação num byte, u16 em +0x69.
const COMPETICAO_V1: [(usize, u32, usize); 7] = [(0, 4, 1), (4, 0x33, 1), (0x38, 0x1a, 1), (0x53, 4, 1), (0x57, 1, 2), (0x59, 4, 4), (0x69, 0, 1)];

/// { 0x54ffd0 } (cdecl, Db_files.cpp): `db_files_read_comps_v1(stream, use_zip, registros, n)`: para cada um dos
/// `n` registros de 107 bytes, os campos de `COMPETICAO_V1`, cada um lido sozinho (tamanho, 1) por `zread`
/// 0x932c30 (use_zip ≠ 0) ou `fread` 0x945bfe; a reputação lida num byte (a original o lê num argumento da
/// própria pilha, já usado) vira u16 sem conta. A primeira leitura que não vem dá 0 (o que já veio fica); tudo,
/// 1. Chamado por `db_files_read_comps` 0x54feb0 com a versão da entrada igual a 1; a base de fábrica e o save
/// são versão 2. Sem SEH e sem variável local lida sem valor.
unsafe extern "C" fn ler_competicoes_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    crate::comparacao::ramo("banco ler competições v1");
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 =
        core::mem::transmute(if use_zip != 0 { 0x932c30usize } else { 0x945bfeusize });
    for i in 0..n as usize {
        let r = registros + i * 107;
        for &(o, t, q) in &COMPETICAO_V1 {
            for k in 0..q {
                if t == 0 {
                    let mut b = 0u8;
                    if ler(&mut b as *mut u8 as usize, 1, 1, stream) == 0 {
                        return 0;
                    }
                    ((r + o) as *mut u16).write_unaligned(b as u16); // reputação da competição
                } else if ler(r + o + k * t as usize, t, 1, stream) == 0 {
                    return 0;
                }
            }
        }
    }
    1
}

/// Comparador: com `fread` (use_zip = 0), a original lê, o arquivo volta (`fseek` 0x946a22 à posição do `ftell`
/// 0x946aae) e a Rust lê de novo; compara os registros, a resposta e onde o arquivo parou.
// ponytail: com zread (o contêiner) só a original; o save é versão 2 e não chega aqui.
unsafe extern "C" fn comparar_ler_competicoes_v1(stream: usize, use_zip: i32, registros: usize, n: u32) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, i32, usize, u32) -> i32>(673);
    if use_zip != 0 {
        return o(stream, use_zip, registros, n);
    }
    let ftell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x946aaeusize);
    let fseek: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(0x946a22usize);
    let pos = ftell(stream);
    let (mut fim_o, mut fim_r) = (0, 0);
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(registros, n as usize * 107)],
        || {
            let r = o(stream, use_zip, registros, n);
            fim_o = ftell(stream);
            r
        },
        || {
            fseek(stream, pos, 0);
            let r = ler_competicoes_v1(stream, use_zip, registros, n);
            fim_r = ftell(stream);
            r
        });
    PLACAR.registrar(igual && r == x && fim_o == fim_r, || {
        let dif = mo.0.first().zip(mr.0.first()).and_then(|(a, b)| a.1.iter().zip(&b.1).position(|(p, q)| p != q));
        format!("ler_competicoes_v1(n {}) original={} rust={} arquivo em {} x {} (de {}), 1º byte diferente {:?}", n, r, x, fim_o, fim_r, pos, dif)
    });
    r
}

/// { 0x5453f0 } (cdecl, Database.cpp): `nation_get_english_name(nação)`: idioma ([0x9b5984]) 0 ou 1: o próprio
/// nome (+4). Senão a tabela do eng.lng ([0xae2408]): nações em linhas de 109 bytes a partir de +0x2c, pelo id
/// (sem conferir a quantidade).
unsafe extern "C" fn nome_ingles_da_nacao(nacao: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return nacao.wrapping_add(4); // idioma 0 ou 1 (nação)
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca (new 0x40) e carrega o eng.lng (0x544fc0) sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(674)(nacao);
    }
    let id = *(nacao as *const i32);
    (*((t + 0x2c) as *const usize)).wrapping_add((id as usize).wrapping_mul(109))
}

unsafe extern "C" fn comparar_nome_ingles_da_nacao(nacao: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(674);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(nacao); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(nacao);
    let n = nome_ingles_da_nacao(nacao);
    PLACAR.registrar(r == n, || format!("nome_ingles_da_nacao({:x}) idioma {} original={:x} rust={:x}", nacao, idioma, r, n));
    r
}

/// { 0x545770 } (cdecl, Database.cpp): `club_get_english_short_name(clube)`: idioma ([0x9b5984]) 0 ou 1: o
/// próprio nome curto (+0x38). Senão a tabela do eng.lng ([0xae2408], linhas de 79 bytes a partir de +0x20, o
/// nome curto em +0x34 da linha): pelo mapa de ids [0xae23fc] se houver, senão pelo id se < quantos (+0),
/// senão 0.
unsafe extern "C" fn nome_curto_ingles_do_clube(clube: usize) -> usize {
    let idioma = *(0x9b5984 as *const u32);
    if idioma == 0 || idioma == 1 {
        return clube.wrapping_add(0x38);
    }
    let t = *(0xae2408 as *const usize);
    if t == 0 {
        // ponytail: o primeiro uso aloca e carrega o eng.lng sob o quadro SEH: a original
        return orig::<unsafe extern "C" fn(usize) -> usize>(675)(clube);
    }
    let linhas = *((t + 0x20) as *const usize);
    let id = *(clube as *const i32);
    let mapa = *(0xae23fc as *const usize);
    let i = if mapa != 0 {
        *((mapa.wrapping_add((id as usize).wrapping_mul(4))) as *const i32)
    } else if id < *(t as *const i32) {
        id
    } else {
        return 0;
    };
    linhas.wrapping_add((i as usize).wrapping_mul(79)).wrapping_add(0x34) // nome curto na linha
}

unsafe extern "C" fn comparar_nome_curto_ingles_do_clube(clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize) -> usize>(675);
    let idioma = *(0x9b5984 as *const u32);
    if idioma != 0 && idioma != 1 && *(0xae2408 as *const usize) == 0 {
        return o(clube); // primeiro uso (aloca e carrega): só a original
    }
    let r = o(clube);
    let n = nome_curto_ingles_do_clube(clube);
    PLACAR.registrar(r == n, || format!("nome_curto_ingles_do_clube({:x}) idioma {} original={:x} rust={:x}", clube, idioma, r, n));
    r
}

/// Autoteste (CM_AUTOTESTE=1, modo comparação): nem a carreira nem o save passam por estas três (a base é
/// versão 2; o jogo dos testes está em inglês). (1) O conversor das competições: 64 casos de sementes fixas,
/// de 1 a 6 registros, arquivo temporário (`autoteste_banco_v1c.tmp` na pasta do jogo, apagado no fim) com
/// bytes sorteados (1 em 4 só zeros), inteiro (1 em 3) ou cortado num ponto sorteado, buffer começando
/// sorteado. (2) Os nomes em inglês: 256 casos com o idioma 0, 1, 2, 3 ou 13 e uma tabela do eng.lng falsa
/// (nunca nula: o primeiro uso carregaria o arquivo de verdade), com e sem o mapa de ids, ids de −2 a 11 (no
/// mapa, de 0 a 7); as globais voltam no fim.
pub unsafe fn autoteste_dbj_2() {
    const ARQ: &str = "autoteste_banco_v1c.tmp";
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let (t0, d0) = PLACAR.contagem();
    let mut s = Sorteio(0x5631_4442_4a32);
    let v1: usize = COMPETICAO_V1.iter().map(|&(_, t, q)| t.max(1) as usize * q).sum();
    for caso in 0..64u64 {
        let n = 1 + (s.proximo() % 6) as usize;
        let total = n * v1;
        let corte = if caso % 3 == 0 { total } else { (s.proximo() % (total as u64 + 1)) as usize };
        let bytes: Vec<u8> = (0..corte).map(|_| if caso % 4 == 1 { 0 } else { s.proximo() as u8 }).collect();
        if std::fs::write(ARQ, &bytes).is_err() {
            continue;
        }
        let f = fopen(b"autoteste_banco_v1c.tmp\0".as_ptr() as usize, b"rb\0".as_ptr() as usize);
        if f == 0 {
            continue;
        }
        let mut buf: Vec<u8> = (0..n * 107).map(|_| s.proximo() as u8).collect();
        comparar_ler_competicoes_v1(f, 0, buf.as_mut_ptr() as usize, n as u32);
        fclose(f);
    }
    let _ = std::fs::remove_file(ARQ);
    let globais = [0x9b5984usize, 0xae2408, 0xae23fc];
    let antes: Vec<usize> = globais.iter().map(|&a| *(a as *const usize)).collect();
    let mut tabela = [0usize; 16]; // a tabela do eng.lng (new 0x40): +0 quantos clubes, +0x20 clubes, +0x2c nações
    let mut mapa = [0i32; 8];
    let mut registro = [0u8; 0x40];
    for _ in 0..256 {
        let z = s.proximo();
        *(0x9b5984 as *mut u32) = [0u32, 1, 2, 3, 13][(z % 5) as usize];
        tabela[0] = ((z >> 4) % 10) as usize;
        tabela[8] = 0x1000_0000 + ((z >> 8) as usize & 0xffff0);
        tabela[11] = 0x2000_0000 + ((z >> 24) as usize & 0xffff0);
        *(0xae2408 as *mut usize) = tabela.as_mut_ptr() as usize;
        for (k, m) in mapa.iter_mut().enumerate() {
            *m = ((z >> (40 + k)) % 9) as i32 - 1;
        }
        let com_mapa = (z >> 50) % 2 == 0;
        *(0xae23fc as *mut usize) = if com_mapa { mapa.as_ptr() as usize } else { 0 };
        let id = if com_mapa { ((z >> 52) % 8) as i32 } else { ((z >> 52) % 14) as i32 - 2 };
        *(registro.as_mut_ptr() as *mut i32) = id;
        comparar_nome_ingles_da_nacao(registro.as_ptr() as usize);
        comparar_nome_curto_ingles_do_clube(registro.as_ptr() as usize);
    }
    for (i, &a) in globais.iter().enumerate() {
        *(a as *mut usize) = antes[i];
    }
    let (t1, d1) = PLACAR.contagem();
    linha(&format!("banco v1 e nomes em inglês: AUTOTESTE {} casos, {} divergencias", t1 - t0, d1 - d0));
}

// ---------- 232a leva: a criação do jogo: os elencos dos times A e B, a comissão do time B e o índice de nomes por nação (Database.cpp) ----------

/// { 0x541e60 } (cdecl, Database.cpp): `database_trim_senior_and_b_team_squads()`, uma vez na criação (create_game
/// 0x8282f1). Em cada clube, a marca +0x244 = tem parceiro (0x540a50 com 0) e o parceiro tem nação (+0x53). No time
/// A (`eh_a`): o elenco conjunto A+B (0x541f90 com dobro: 100 ponteiros) em ordem de reputação (qsort 0x945333 com
/// 0x5402f0); de trás para a frente, enquanto o conjunto tiver mais de 48, tira a pessoa do clube dela (0x53c190 com
/// o clube +0x39, sem data, desligando a pessoa, não como jogador), pulando os goleiros (jogador +0x61 com +0xf ≥ 15,
/// byte com sinal) enquanto restarem 3 ou menos. Contadores de 8 bits com sinal, como os da original. Sem sorteio;
/// a local `eh_a` só é lida quando 0x540a50 devolve parceiro (e então ela foi gravada).
unsafe extern "C" fn cortar_elencos_de_a_e_b() {
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
unsafe extern "C" fn juntar_o_time_b_ao_a() {
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
unsafe extern "C" fn montar_indice_de_nomes() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let nacoes = *(0xae2350 as *const i32);
    let t = alocar((nacoes as u32).wrapping_mul(0x24));
    *(0xae2338 as *mut usize) = t;
    if t == 0 {
        // ponytail: a original pede de novo e, sem memória, abre a caixa e encerra o jogo
        return orig::<unsafe extern "C" fn()>(678)();
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
unsafe fn regioes_dos_times_a_e_b() -> Vec<(usize, usize)> {
    let tabela = *(0xae23b4 as *const usize);
    let n = *(0xae235c as *const i32);
    let mut regioes = vec![(tabela, n.max(0) as usize * 581)];
    let mut pessoas = Vec::new();
    for &(a, b) in PARES_A_B.iter() {
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
    let regioes = regioes_dos_times_a_e_b();
    let (_, _, igual, mut mo, mut mr) = crate::comparacao::lado_a_lado(&regioes, original, rust);
    let onde = mo.0.iter().zip(&mr.0).find_map(|((e, x), (_, y))| {
        x.iter().zip(y).position(|(a, b)| a != b).map(|k| (e + k, x[k], y[k]))
    });
    PLACAR.registrar(igual, || match onde {
        Some((a, x, y)) => format!("{}: diverge em {:x} original={:02x} rust={:02x}", nome, a, x, y),
        None => format!("{}: diverge", nome),
    });
    mo.0.clear();
    mr.0.clear();
}

unsafe extern "C" fn comparar_cortar_elencos_de_a_e_b() {
    let o = orig::<unsafe extern "C" fn()>(676);
    comparar_times_a_e_b("cortar_elencos_de_a_e_b", || o(), || cortar_elencos_de_a_e_b());
}

unsafe extern "C" fn comparar_juntar_o_time_b_ao_a() {
    let o = orig::<unsafe extern "C" fn()>(677);
    comparar_times_a_e_b("juntar_o_time_b_ao_a", || o(), || juntar_o_time_b_ao_a());
}

/// Comparador: a original monta o índice; a cópia monta outro (novo malloc), os dois são comparados byte a byte, o
/// da cópia é liberado (free 0x9452ca) e o da original volta a [0xae2338]. Muda a heap do modo de comparação (a
/// impressão dele não é critério).
unsafe extern "C" fn comparar_montar_indice_de_nomes() {
    orig::<unsafe extern "C" fn()>(678)();
    let t = *(0xae2338 as *const usize);
    let n = (*(0xae2350 as *const i32)).max(0) as usize * 0x24;
    if t == 0 || n == 0 {
        return;
    }
    montar_indice_de_nomes();
    let r = *(0xae2338 as *const usize);
    let original = core::slice::from_raw_parts(t as *const u8, n);
    let copia = core::slice::from_raw_parts(r as *const u8, n);
    let onde = original.iter().zip(copia).position(|(a, b)| a != b);
    PLACAR.registrar(onde.is_none(), || {
        let k = onde.unwrap_or(0) & !3;
        format!("montar_indice_de_nomes: diverge na nação {} +{:#x} original={} rust={}", k / 0x24, k % 0x24,
            core::ptr::read_unaligned((t + k) as *const i32), core::ptr::read_unaligned((r + k) as *const i32))
    });
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(r);
    *(0xae2338 as *mut usize) = t;
}

// ---------- 232a leva: a criação do jogo: tirar a pessoa do clube e os olheiros do clube com os do time parceiro (Database.cpp) ----------

/// { 0x53c190 } (cdecl, Database.cpp): `database_remove_staff_from_club(clube, pessoa, data, desligar, como_jogador)`.
/// Na criação (os cortes de 0x541e60, 0x544970 e 0x524d70) e na carreira (transferências, demissões). Seleção = id do
/// clube ≥ [0xae235c] − 2 × [0xae2350]. Com `data`: os 8 bytes dela vão para a pessoa (+0x29 na seleção, senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê), antes de olhar o cargo. Cargo = pessoa +0x28 (seleção)
/// ou +0x3d (byte); com `como_jogador`, 11, 12, 13 e 15 viram 11 e os outros 0. Casas (tabela de salto 0x53c54c): 0
/// nenhuma; 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf e, na seleção principal (id < [0xae235c] − [0xae2350]), o +0xcf da sub-21
/// (0x53b4f0 com a nação +0x53) se for a mesma pessoa; 6, 7 +0xd3; 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco
/// +0xd7 (50); 12 +0xcf e o elenco; 13 +0xd3 e o elenco; 15 +0x19f e o elenco; 14, acima de 15 ou negativo: 0. A casa
/// que tem a pessoa (a primeira) é zerada; pessoa fora da casa: 0 (nas 12, 13 e 15 a primeira casa já zerada fica).
/// Com `desligar`: zera o clube na pessoa (+0x24 na seleção, +0x39 no clube). Devolve 1. Clube nulo: 0. Pessoa nula: a
/// original (caixa "Error" e 0).
unsafe extern "C" fn tirar_pessoa_do_clube(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    if clube == 0 {
        return 0;
    }
    if pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(679)(clube, pessoa, data, desligar, como_jogador);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if data != 0 {
        let d = if selecao() { pessoa + 0x29 } else { pessoa + 0x3e };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut cargo = if selecao() { *((pessoa + 0x28) as *const u8) } else { *((pessoa + 0x3d) as *const u8) };
    if como_jogador != 0 {
        cargo = if matches!(cargo, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    let rd = |d: usize| core::ptr::read_unaligned((clube + d) as *const usize);
    let zerar = |d: usize| core::ptr::write_unaligned((clube + d) as *mut usize, 0);
    // a primeira das `k` casas a partir de +d que tem a pessoa, zerada; false se nenhuma tem
    let tirar_da_casa = |d: usize, k: usize| match (0..k).find(|&j| rd(d + 4 * j) == pessoa) {
        Some(j) => {
            zerar(d + 4 * j);
            true
        }
        None => false,
    };
    let tirou = match cargo {
        0 => true,
        1 => tirar_da_casa(0xbf, 1),
        2..=4 => tirar_da_casa(0xc3, 3),
        5 => {
            let ok = tirar_da_casa(0xcf, 1);
            let id = gi(clube);
            let n = gi(0xae235c);
            if ok && id >= n.wrapping_sub(gi(0xae2350).wrapping_mul(2)) && id < n.wrapping_sub(gi(0xae2350)) {
                let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
                let s = sub21(rd(0x53));
                if core::ptr::read_unaligned((s + 0xcf) as *const usize) == pessoa {
                    core::ptr::write_unaligned((s + 0xcf) as *mut usize, 0); // o treinador da sub-21
                }
            }
            ok
        }
        6 | 7 => tirar_da_casa(0xd3, 1),
        8 => tirar_da_casa(0x19f, 5),
        9 => tirar_da_casa(0x1b3, 7),
        10 => tirar_da_casa(0x1cf, 3),
        11 => tirar_da_casa(0xd7, 50),
        12 => tirar_da_casa(0xcf, 1) && tirar_da_casa(0xd7, 50),
        13 => tirar_da_casa(0xd3, 1) && tirar_da_casa(0xd7, 50),
        15 => tirar_da_casa(0x19f, 5) && tirar_da_casa(0xd7, 50),
        _ => false,
    };
    if !tirou {
        return 0;
    }
    if desligar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + 0x24) as *mut usize, 0);
        } else {
            core::ptr::write_unaligned((pessoa + 0x39) as *mut usize, 0); // desliga a pessoa do clube
        }
    }
    1
}

/// { 0x53cd60 } (cdecl, Database.cpp): `database_get_club_scouts(clube, vetor)`: os 7 olheiros (+0x1b3) do `clube` no
/// `vetor`. Sem parceiro A/B (0x540a50 com nulo e 1, que zera a marca +0x244 do clube quando recusa o parceiro): as 7
/// casas como estão. Com parceiro: os não nulos do clube e depois os do parceiro até 7; o resto zerado só quando os
/// dois juntos não enchem as 7. Contador de 8 bits na original (no máximo 7). Devolve 1. Clube nulo: a original
/// (caixa "Error" e 0). Na carreira, pela 0x7f7270 (as listas dos olheiros, nas transferências e contratações).
unsafe extern "C" fn olheiros_do_clube(clube: usize, vetor: *mut usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(680)(clube, vetor);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p == 0 {
        for j in 0..7 {
            core::ptr::write_unaligned(vetor.add(j), rd(clube + 0x1b3 + 4 * j));
        }
        return 1;
    }
    let mut n = 0usize;
    for j in 0..7 {
        let o = rd(clube + 0x1b3 + 4 * j);
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for j in 0..7 {
        if n >= 7 {
            return 1;
        }
        let o = rd(p + 0x1b3 + 4 * j); // os olheiros do parceiro
        if o != 0 {
            core::ptr::write_unaligned(vetor.add(n), o);
            n += 1;
        }
    }
    for k in n..7 {
        core::ptr::write_unaligned(vetor.add(k), 0);
    }
    1
}

unsafe extern "C" fn comparar_tirar_pessoa_do_clube(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(679);
    if clube == 0 || pessoa == 0 {
        return o(clube, pessoa, data, desligar, como_jogador); // 0 ou caixa: só a original
    }
    // o clube, a pessoa e, na seleção principal, a sub-21 (a única outra casa que a rotina zera)
    let mut regioes = vec![(clube, 581), (pessoa, 110)];
    let (id, n, nacoes) = (*(clube as *const i32), *(0xae235c as *const i32), *(0xae2350 as *const i32));
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if id >= n.wrapping_sub(nacoes.wrapping_mul(2)) && id < n.wrapping_sub(nacoes) && nacao != 0 {
        let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
        let s = sub21(nacao);
        if s != clube {
            regioes.push((s, 581));
        }
    }
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(clube, pessoa, data, desligar, como_jogador),
        || tirar_pessoa_do_clube(clube, pessoa, data, desligar, como_jogador));
    PLACAR.registrar(igual && r == m, || format!("tirar_pessoa_do_clube({:x}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, pessoa, data != 0, desligar, r, m, mo, mr));
    r
}

unsafe extern "C" fn comparar_olheiros_do_clube(clube: usize, vetor: *mut usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize) -> u32>(680);
    if clube == 0 {
        return o(clube, vetor); // caixa: só a original
    }
    // o vetor e a marca +0x244 do clube, que a 0x540a50 zera quando recusa o parceiro
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(vetor as usize, 28), (clube + 0x244, 1)],
        || o(clube, vetor), || olheiros_do_clube(clube, vetor));
    PLACAR.registrar(igual && r == n, || format!("olheiros_do_clube({:x}) original={} rust={} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
