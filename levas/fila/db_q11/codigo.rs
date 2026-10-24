// ---------- %%LEVA%%a leva: os vetores de atributos de volta ao não jogador e ao jogador e o reequilíbrio dos atributos do não jogador (Database.cpp) ----------

/// { 0x53a980 } (cdecl, Database.cpp): o inverso de 0x53a8b0: o byte de baixo de cada uma das 21
/// células de 3 bytes do `vetor` volta ao `nao_jogador` na ordem `ORDEM_DO_NAO_JOGADOR`. A pessoa não
/// é usada. eax na saída: o vetor com o último byte copiado no byte de baixo (os chamadores não usam).
unsafe extern "C" fn vetor_para_o_nao_jogador(nao_jogador: usize, _pessoa: usize, vetor: usize) -> u32 {
    let mut ultimo = 0u8;
    for (k, &d) in ORDEM_DO_NAO_JOGADOR.iter().enumerate() {
        ultimo = *((vetor + 3 * k) as *const u8);
        *((nao_jogador + d) as *mut u8) = ultimo;
    }
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// { 0x53aa10 } (cdecl, Database.cpp): o inverso de 0x53a610: o byte de baixo das células 0..53 do
/// `vetor` volta ao `jogador` nos deslocamentos de `CELULAS_DO_VETOR` e o da célula 54 à `pessoa`
/// +0x58. eax na saída: o vetor com o último byte no byte de baixo (os chamadores não usam).
unsafe extern "C" fn vetor_para_o_jogador(jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    for (i, &(d, _)) in CELULAS_DO_VETOR.iter().enumerate() {
        *((jogador + d as usize) as *mut u8) = *((vetor + 3 * i) as *const u8);
    }
    let ultimo = *((vetor + 3 * 54) as *const u8);
    *((pessoa + 0x58) as *mut u8) = ultimo;
    (vetor as u32 & 0xffff_ff00) | ultimo as u32
}

/// { 0x539490 } (cdecl, Database.cpp): reequilibra os atributos do não jogador (pessoa +0x69). Monta
/// o vetor (63 células 0xff81/0x81, preenchidas por 0x53a8b0, que devolve o corte 13) e, duas vezes,
/// a partir do corte: (1) com limite 40, +10 por volta, até 6 células com |valor| ≤ limite (marcadas);
/// a célula 19 fora do limite também é marcada; (2) soma −5 − média às marcadas (presas em −125..125);
/// na 1ª passada as não marcadas positivas entram na soma de excesso, as demais são marcadas; com
/// cargo +0x3d ou +0x28 = 10, as células ficam ≤ −80 e a 19 ≥ 0; (3) na 1ª passada, com soma > 28,
/// tira (soma − 28) / quantas das não marcadas (mínimo 0). No fim, 0x53a980 grava de volta. Pessoa
/// ou não jogador nulos: nada.
unsafe extern "C" fn normalizar_atributos_do_nao_jogador(pessoa: usize) {
    if pessoa == 0 {
        return;
    }
    let nj = core::ptr::read_unaligned((pessoa + 0x69) as *const usize);
    if nj == 0 {
        return;
    }
    let preencher: unsafe extern "C" fn(usize, usize, usize, *mut u8) -> u32 = core::mem::transmute(0x53a8b0usize);
    let devolver: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x53a980usize);
    let mut vetor = [0u8; 63 * 3];
    for c in vetor.chunks_exact_mut(3) {
        c.copy_from_slice(&[0x81, 0xff, 0x81]);
    }
    let v = vetor.as_mut_ptr();
    let mut fim = 0u8;
    preencher(nj, pessoa, v as usize, &mut fim);
    let w = |i: i8| core::ptr::read_unaligned(v.add(3 * i as usize) as *const i16);
    let pw = |i: i8, x: i16| core::ptr::write_unaligned(v.add(3 * i as usize) as *mut i16, x);
    let fim = fim as i8;
    let mut marca = [0u32; 64];
    let cargo_10 = *((pessoa + 0x3d) as *const u8) == 10 || *((pessoa + 0x28) as *const u8) == 10;
    for passo in 0..2 {
        let bp = w(fim);
        let (mut quantas, mut soma) = (0i32, 0i32);
        let mut limite = 0x28i32;
        let (mut total, mut n, mut melhor);
        loop {
            total = 0i32;
            n = 0i8;
            melhor = -1i8;
            if bp != -0x7f {
                let mut c = fim;
                loop {
                    let x = w(c) as i32;
                    if x.abs() <= limite as i16 as i32 {
                        total += x;
                        n += 1;
                        marca[c as usize] = 1;
                    } else {
                        marca[c as usize] = 0;
                        if c == 0x13 && (melhor == -1 || w(0x13) > w(melhor)) {
                            melhor = 0x13;
                        }
                    }
                    c += 1;
                    if w(c) == -0x7f {
                        break;
                    }
                }
                if n >= 6 {
                    break;
                }
            }
            // ponytail: com a célula do corte = −127 o original roda para sempre; a cópia também
            limite += 10;
        }
        if melhor > -1 {
            marca[melhor as usize] = 1;
        }
        let delta = (-5i32).wrapping_sub(total / n as i32) as i16;
        if bp != -0x7f {
            let mut c = fim;
            loop {
                if c != melhor {
                    if marca[c as usize] == 1 {
                        let x = w(c).wrapping_add(delta);
                        pw(c, if x < -0x7d { -0x7d } else if x > 0x7d { 0x7d } else { x });
                    } else if passo == 0 && w(c) > 0 {
                        quantas += 1;
                        soma += w(c) as i32;
                    } else {
                        marca[c as usize] = 1;
                    }
                    if cargo_10 {
                        if c != 0x13 {
                            pw(c, w(c).min(-0x50));
                        } else {
                            pw(0x13, w(0x13).max(0));
                        }
                    }
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
        if w(fim) != -0x7f {
            let mut c = fim;
            while quantas != 0 && soma > 0x1c {
                if marca[c as usize] != 1 {
                    let x = w(c).wrapping_sub(((soma - 0x1c) / quantas) as i16);
                    pw(c, if x < 0 { 0 } else { x });
                }
                c += 1;
                if w(c) == -0x7f {
                    break;
                }
            }
        }
    }
    devolver(nj, pessoa, v as usize);
}

unsafe extern "C" fn comparar_vetor_para_o_nao_jogador(nao_jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(nao_jogador + 0x0e, 21)],
        || o(nao_jogador, pessoa, vetor), || vetor_para_o_nao_jogador(nao_jogador, pessoa, vetor));
    PLACAR.registrar(igual && r == n, || format!("vetor_para_o_nao_jogador({:x}) original={:x} rust={:x} {:x?} x {:x?}", nao_jogador, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_vetor_para_o_jogador(jogador: usize, pessoa: usize, vetor: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I1%%);
    // os deslocamentos de CELULAS_DO_VETOR ficam em +0x0f..+0x44
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(jogador + 0x0f, 0x44 - 0x0f + 1), (pessoa + 0x58, 1)],
        || o(jogador, pessoa, vetor), || vetor_para_o_jogador(jogador, pessoa, vetor));
    PLACAR.registrar(igual && r == n, || format!("vetor_para_o_jogador({:x}) original={:x} rust={:x} {:x?} x {:x?}", jogador, r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_normalizar_atributos_do_nao_jogador(pessoa: usize) {
    let o = orig::<unsafe extern "C" fn(usize)>(%%I2%%);
    let nj = if pessoa == 0 { 0 } else { core::ptr::read_unaligned((pessoa + 0x69) as *const usize) };
    if nj == 0 {
        return o(pessoa); // não faz nada: só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(nj + 0x0e, 21)], || o(pessoa),
        || normalizar_atributos_do_nao_jogador(pessoa));
    PLACAR.registrar(igual, || format!("normalizar_atributos_do_nao_jogador({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}
