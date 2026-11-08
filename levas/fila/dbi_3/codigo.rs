// ---------- %%LEVA%%a leva: a normalização física e geral dos atributos do jogador e o segundo nome pelo peso (Database.cpp) ----------

/// TRAINING_MANAGER::remove_or_restore_training_progress (0x8b3640, thiscall com o gerente [0xde1f44]): tira
/// (`devolver` = 0) ou devolve (1) o progresso do treino dos atributos da `pessoa`. Sem gerente, nada.
unsafe fn remove_or_restore_training(pessoa: usize, devolver: u32) {
    let gerente = *(0xde1f44 as *const usize);
    if gerente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b3640usize);
        f(gerente, pessoa, devolver);
    }
}

/// O fim comum de 0x538bc0 e 0x538ec0 com o jogador +0xf alto: tira de +0x2a e de +0x3a a média dos dois
/// (bytes com sinal, divisão truncada).
unsafe fn subtract_mean_2a_3a(jogador: usize) {
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
unsafe fn rebalance_attr_vector(v: *mut u8, inicio: i8, fim: i8, jogador: usize, alvo: i32, pular: &[i8],
                                        excesso: impl Fn(i16) -> i32) {
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let peso = |i: i8| *v.add(3 * i as usize + 2);
    let fora = |c: i8| pular.contains(&c);
    let jovem = *((jogador + 0xf) as *const i8) < 0xf;
    let mut marca = [0u32; 64];
    for step_towards in 0..2 {
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limit = 0x1ei32;
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
                        if (x as i32).abs() <= limit as i16 as i32 {
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
            limit += 5;
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
                } else if step_towards == 0 && w(c) > 0 && jovem {
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
unsafe fn player_attr_vector(jogador: usize, pessoa: usize, v: *mut u8) -> (u8, u8) {
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
/// habilidade, preso em −40..0; vetor (0xff81/0x81) pelo 0x53a610, `rebalance_attr_vector` sem as
/// células 0x18 e 0x1e (0x11/0x12 contam em dobro), volta pelo 0x53aa10; jogador +0xf = 20: tira as médias
/// de +0x2a/+0x3a e o terço de +0x1e, +0x24, +0x39; devolve o progresso do treino.
unsafe extern "C" fn staff_normalise_player_physical_attributes(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if jogador == 0 {
        return;
    }
    remove_or_restore_training(pessoa, 0);
    let b = |d: usize| *((jogador + d) as *const i8) as i32;
    let f = |x: i32, k: usize| match_pl_ftol(x as f64 * read_f64_at(k));
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
    let (lados, ponderados) = player_attr_vector(jogador, pessoa, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, alvo, &[0x1e, 0x18],
        |x| 2 * x as i32);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53aa10usize);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) == 0x14 {
        subtract_mean_2a_3a(jogador);
        let terco = ((b(0x1e) + b(0x24) + b(0x39)) / 3) as u8;
        for off in [0x1e, 0x39, 0x24] {
            *((jogador + off) as *mut u8) = (*((jogador + off) as *const u8)).wrapping_sub(terco);
        }
    }
    remove_or_restore_training(pessoa, 1);
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
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    *(0xde1f44 as *mut usize) = 0;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regioes, || o(pessoa), || rust(pessoa));
    *(0xde1f44 as *mut usize) = gerente;
    SCOREBOARD.record(igual, || format!("{}({:x}) {:x?} x {:x?}", nome, pessoa, mo, mr));
    if gerente != 0 {
        antes.write_back();
        o(pessoa);
    }
}

unsafe extern "C" fn comparar_staff_normalise_player_physical_attributes(pessoa: usize) {
    comparar_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(%%I0%%), staff_normalise_player_physical_attributes, pessoa,
        "staff_normalise_player_physical_attributes");
}

/// { 0x53a0e0 } (cdecl, Database.cpp): um segundo nome (linha de 60 bytes em [0xae23f4]) pelo peso. Nação com
/// mais de 10 segundos nomes (`nation_name_index` +0x1c): 3 sorteios na faixa dela (a partir de +4),
/// fica o de maior peso (+0x3b, com sinal; começa em −1 e a linha 0 se nenhum passar). Sem nação ou com até
/// 10: sorteio na tabela inteira ([0xae239c], no máximo 0x7fff). Só o rand() da CRT. TapeCall por
/// OfficialsManager::add_random_officials (0x79c6b4), na criação dos árbitros do jogo novo.
unsafe extern "C" fn database_pick_second_name_by_weight(nacao: usize) -> usize {
    let linha = |i: i32| (*(0xae23f4 as *const usize)).wrapping_add((i as usize).wrapping_mul(60));
    if nacao != 0 {
        let e = nation_name_index(nacao);
        let quantos = *((e + 0x1c) as *const i32);
        if quantos > 0xa {
            let inicio = *((e + 4) as *const i32);
            let (mut melhor, mut escolhido) = (-1i8, 0i32);
            for _ in 0..3 { // 3 sorteios pelo peso
                let i = scaled_crt_rand(quantos).wrapping_add(inicio);
                let peso = *((linha(i) + 0x3b) as *const i8);
                if peso > melhor {
                    melhor = peso;
                    escolhido = i;
                }
            }
            return linha(escolhido); // o segundo nome de maior peso
        }
    }
    linha(scaled_crt_rand(name_count_capped(0xae239c)))
}

unsafe extern "C" fn comparar_database_pick_second_name_by_weight(nacao: usize) -> usize {
    // só o rand() da CRT muda ([0xad6c30]): o estado volta entre os lados e a cópia não conta
    let o = original_fn::<unsafe extern "C" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(nacao),
        || crate::rng::without_counting(|| database_pick_second_name_by_weight(nacao)));
    SCOREBOARD.record(igual && r == n, || format!("database_pick_second_name_by_weight({:x}) original={:x} rust={:x} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// { 0x538bc0 } (cdecl, Database.cpp): `staff_normalise_player_attributes(pessoa)`: com o jogador (pessoa
/// +0x61), tira o progresso do treino (0x8b3640), monta o vetor pelo 0x53a610 (sem pré-encher: as células
/// usadas vêm todas dele), `rebalance_attr_vector` com alvo −20 em todas as células do intervalo
/// (0x11/0x12 contam ftol(valor × [0x968f98])), volta pelo 0x53aa10; jogador +0xf ≥ 15: tira as médias de
/// +0x2a/+0x3a; devolve o progresso do treino. Roda na criação (depois de 0x538ec0) e na carga e no treino.
unsafe extern "C" fn staff_normalise_player_attributes(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if jogador == 0 {
        return;
    }
    remove_or_restore_training(pessoa, 0);
    let mut vetor = [0u8; 63 * 3];
    let v = vetor.as_mut_ptr();
    let (lados, ponderados) = player_attr_vector(jogador, pessoa, v);
    rebalance_attr_vector(v, (lados as i8).wrapping_add(2), ponderados as i8, jogador, -20, &[],
        |x| match_pl_ftol(x as f64 * read_f64_at(0x968f98)));
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53aa10usize);
    devolver(jogador, pessoa, v as usize);
    if *((jogador + 0xf) as *const i8) >= 0xf {
        subtract_mean_2a_3a(jogador);
    }
    remove_or_restore_training(pessoa, 1);
}

unsafe extern "C" fn comparar_staff_normalise_player_attributes(pessoa: usize) {
    comparar_normalizacao_do_jogador(original_fn::<unsafe extern "C" fn(usize)>(%%I2%%), staff_normalise_player_attributes, pessoa,
        "staff_normalise_player_attributes");
}
