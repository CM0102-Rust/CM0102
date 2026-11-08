// ---------- %%LEVA%%a leva: o árbitro de cada jogo: a escolha do árbitro, as marcas de ocupado do dia e a renovação dos árbitros da liga (officials_manager.cpp) ----------

unsafe fn cpy8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpy8_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O bloco de árbitros da nação `nacao` no gerenciador (0x102 bytes em [this+0xc]: +4 conta, +8 primeiro índice, +0x10
/// palavra com o número de faixas de liga, +0x12 faixas de 16 bytes: +0 primeiro, +4 conta, +0xc id da competição).
unsafe fn nation_officials_block(this: usize, nacao: i32) -> usize {
    cpy8_d(this + 0xc).wrapping_add(nacao.wrapping_mul(0x102) as usize)
}

/// O id da nação do clube (+0x53, ponteiro) ou, sem nação, `sem`.
unsafe fn officials_manager_club_nation_id(clube: usize) -> Option<i32> {
    let n = cpy8_d(clube + 0x53);
    if n != 0 { Some(cpy8_i(n)) } else { None }
}

/// { 0x79c7b0 } (thiscall, ret 8, officials_manager.cpp): `OfficialsManager::choose_match_official(jogo, saida)`, pela
/// montagem do dia (0x6b3c10, saída em +0x2e do jogo). Jogo tirado (−2): 0 (eax 0xffffff00, como a original). A faixa
/// de árbitros: sem competição, a nação do mandante (+0x1c, +0x53) ou uma sorteada; amistoso (0x5c6710 em [0xae2c74]),
/// a do mandante; competição sem objeto ([0xadadfc]), como sem competição; competição dos tipos 2 e 4 (+0x42), o quarto
/// de cima (ftol de 0,75 e 0,25 da conta) da primeira nação, a partir de uma sorteada, do continente (+0x59 da
/// competição, ou +0x71 da nação do mandante, do visitante, ou um sorteado) que não seja a de nenhum dos dois clubes;
/// nas outras, a faixa da competição no bloco da nação dela (+0x5d; ou do mandante, do visitante, ou sorteada), a última
/// que bater, ou o bloco inteiro. Conta 1: o primeiro; senão primeiro + sorteio(conta − 1). Ocupado (o byte do id do
/// árbitro em [this+8]): o seguinte da faixa, dando a volta; todos ocupados, o sorteado. Grava o índice (palavra) e
/// devolve 1 no al (o resto do eax com o índice, como a original).
unsafe extern "thiscall" fn choose_match_official(this: usize, jogo: usize, saida: usize) -> u32 {
    if cpy8_i(jogo) == -2 {
        return 0xffff_ff00;
    }
    let nacoes = cpy8_i(0xae2350);
    let mandante = cpy8_d(jogo + 0x1c);
    let visitante = cpy8_d(jogo + 0x20);
    let comp = cpy8_d(jogo + 0x14);
    let faixa_da_nacao = |nacao: i32| {
        let b = nation_officials_block(this, nacao);
        (cpy8_i(b + 8), cpy8_i(b + 4))
    };
    let (ini, n) = if comp == 0 {
        faixa_da_nacao(officials_manager_club_nation_id(mandante).unwrap_or_else(|| sortear(nacoes)))
    } else {
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistoso(cpy8_d(0xae2c74), comp) != 0 {
            faixa_da_nacao(cpy8_i(cpy8_d(mandante + 0x53))) // como a original, sem conferir a nação
        } else {
            let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
            if c == 0 {
                faixa_da_nacao(officials_manager_club_nation_id(mandante).unwrap_or_else(|| sortear(nacoes)))
            } else {
                let tipo = cpy8_b(c + 0x42) as i8;
                if tipo == 2 || tipo == 4 {
                    let continente_do = |clube: usize| {
                        let n = cpy8_d(clube + 0x53);
                        if n != 0 && cpy8_d(n + 0x71) != 0 { Some(cpy8_i(cpy8_d(n + 0x71))) } else { None }
                    };
                    let continente = if cpy8_d(comp + 0x59) != 0 {
                        cpy8_i(cpy8_d(comp + 0x59))
                    } else if let Some(k) = continente_do(mandante) {
                        k
                    } else if let Some(k) = continente_do(visitante) {
                        k
                    } else {
                        sortear(cpy8_i(0xae234c))
                    };
                    let inicio = sortear(nacoes);
                    let mut k = inicio;
                    loop {
                        let nac = cpy8_d(0xae23a8).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = cpy8_d(nac + 0x71);
                        if cont != 0 && cpy8_i(cont) == continente
                            && officials_manager_club_nation_id(mandante) != Some(cpy8_i(nac))
                            && officials_manager_club_nation_id(visitante) != Some(cpy8_i(nac))
                        {
                            break;
                        }
                        k += 1;
                        if k >= cpy8_i(0xae2350) {
                            k = 0;
                        }
                        if k == inicio {
                            break;
                        }
                    }
                    let b = nation_officials_block(this, k);
                    let conta = cpy8_i(b + 4) as f64;
                    crate::comparacao::ramo("cpy_8 arbitro neutro");
                    (cpy8_i(b + 8).wrapping_sub(match_pl_ftol(conta * -0.75)), match_pl_ftol(conta * 0.25)) // cpy8: o quarto de cima
                } else {
                    let nacao = if cpy8_d(comp + 0x5d) != 0 {
                        cpy8_i(cpy8_d(comp + 0x5d))
                    } else if let Some(k) = officials_manager_club_nation_id(mandante) {
                        k
                    } else if let Some(k) = officials_manager_club_nation_id(visitante) {
                        k
                    } else {
                        sortear(nacoes)
                    };
                    let b = nation_officials_block(this, nacao);
                    let id = cpy8_i(comp);
                    let mut achou = None;
                    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
                        let e = b + 0x12 + f * 0x10;
                        if cpy8_i(e + 0xc) == id {
                            achou = Some((cpy8_i(e), cpy8_i(e + 4)));
                        }
                    }
                    match achou {
                        Some(faixa) => {
                            crate::comparacao::ramo("cpy_8 faixa da liga");
                            faixa // cpy8: a faixa da liga
                        }
                        None => (cpy8_i(b + 8), cpy8_i(b + 4)),
                    }
                }
            }
        }
    };
    let sorteado = if n == 1 {
        ini
    } else {
        crate::comparacao::ramo("cpy_8 arbitro sorteado");
        ini.wrapping_add(sortear(n.wrapping_sub(1))) // cpy8: o sorteio na faixa
    };
    let ocupado = |k: i32| {
        let ficha = cpy8_d(0xae23e8).wrapping_add(k.wrapping_mul(0x2b) as usize);
        cpy8_b(cpy8_d(this + 8).wrapping_add(cpy8_d(ficha))) != 0
    };
    let mut k = sorteado;
    if ocupado(k) {
        let fim = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= fim {
                k = ini;
            }
            if k == sorteado || !ocupado(k) {
                break;
            }
        }
    }
    core::ptr::write_unaligned(saida as *mut i16, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Comparador: lado a lado pela palavra de saída e pelo estado do sorteio, e pela devolução.
unsafe extern "thiscall" fn comparar_choose_match_official(this: usize, jogo: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 2));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida),
        || crate::aleatorio::sem_contar(|| choose_match_official(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("choose_match_official(jogo {:#x}) original={:#x} {:x?} rust={:#x} {:x?}", jogo, r, mo, n, mr));
    r
}

/// { 0x79bf40 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::clear_busy_flags`, no laço do dia
/// (0x5cccd0) e na corrida até a data de início (0x5cc850): zera os [this+4] bytes de ocupado em [this+8] (a conta e o
/// vetor relidos a cada passo). Devolve a conta (0 se não positiva), como o eax da original.
unsafe extern "thiscall" fn clear_busy_flags(this: usize) -> u32 {
    let mut i = 0i32;
    while i < cpy8_i(this + 4) {
        *(cpy8_d(this + 8).wrapping_add(i as usize) as *mut u8) = 0;
        i += 1;
    }
    i as u32
}

/// Comparador: lado a lado pelos bytes de ocupado e pela devolução.
unsafe extern "thiscall" fn comparar_clear_busy_flags(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let n = cpy8_i(this + 4);
    let reg = if n > 0 && cpy8_d(this + 8) != 0 { vec![(cpy8_d(this + 8), n as usize)] } else { Vec::new() };
    let (r, m, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || clear_busy_flags(this));
    PLACAR.registrar(igual && r == m, || format!("clear_busy_flags(conta {}) original={} rust={} memoria igual {}", n, r, m, igual));
    r
}

/// { 0x79cee0 } (thiscall, ret 4, officials_manager.cpp): `OfficialsManager::renew_officials(comp)`, no começo da
/// temporada de cada liga (83 chamadores). Sem competição, sem objeto ([0xadadfc]), dos tipos 4 e 2 (+0x42) ou sem
/// conta (0x4b0700): 0. A faixa da competição no bloco da nação dela (+0x5d, ou uma sorteada), a primeira que bater,
/// ou o bloco inteiro. Cada árbitro da faixa (fichas de 0x2b bytes em [0xae23e8]; ficha nula: a caixa "Error"): com
/// mais de 45 anos (o ano em byte, menos 1 antes do dia dele), ou mais de 40 e sorteio(3) zero, vira outro: dois nomes
/// da nação dele (0x539c60, +4 e +8), +0x25 sorteio(16) + 5 (ou, com sorteio(10) zero, sorteio(11) + sorteio(11)), o
/// ano (+0xe) ano do jogo − 27 + sorteio(5) e o dia (+0xc) sorteio(340). Depois zera a linha de estatísticas (17 bytes
/// em [this]) do primeiro da faixa (+4, +8, +0xa, +0xc, +0xf; a original repete a mesma linha `conta` vezes). Devolve
/// 1 (só o al vale).
unsafe extern "thiscall" fn renew_officials(this: usize, comp: usize) -> u8 {
    if comp == 0 {
        return 0;
    }
    let c = cpy8_d(cpy8_d(0xadadfc).wrapping_add(cpy8_i(comp).wrapping_mul(4) as usize));
    if c == 0 || cpy8_b(c + 0x42) == 4 || cpy8_b(c + 0x42) == 2 {
        return 0;
    }
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(cpy8_i(comp)) == 0 {
        return 0;
    }
    let nacao = if cpy8_d(comp + 0x5d) != 0 { cpy8_i(cpy8_d(comp + 0x5d)) } else { sortear(cpy8_i(0xae2350)) };
    let b = nation_officials_block(this, nacao);
    let mut faixa = None;
    for f in 0..cpy8_w(b + 0x10).max(0) as usize {
        let e = b + 0x12 + f * 0x10;
        if cpy8_i(e + 0xc) == cpy8_i(comp) {
            faixa = Some((cpy8_i(e), cpy8_i(e + 4)));
            break;
        }
    }
    let (ini, n) = faixa.unwrap_or((cpy8_i(b + 8), cpy8_i(b + 4)));
    let nome: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    for k in 0..n.max(0) {
        let ficha = cpy8_d(0xae23e8).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize);
        if ficha == 0 {
            show_error_at(0xa6a22c, 0xa61);
            continue;
        }
        let ano = cpy8_b(0xae2c92);
        let dif = (cpy8_w(ficha + 0xc) as i32).wrapping_sub(cpy8_w(0xae2c90) as i32);
        let idade = if dif > 0 { ano.wrapping_sub(cpy8_b(ficha + 0xe)).wrapping_sub(1) } else { ano.wrapping_sub(cpy8_b(ficha + 0xe)) } as i8;
        if idade <= 0x28 || (idade <= 0x2d && sortear(3) != 0) {
            continue;
        }
        crate::comparacao::ramo("cpy_8 arbitro renovado");
        let nac = cpy8_d(ficha + 0x16);
        let n1 = nome(nac);
        core::ptr::write_unaligned((ficha + 4) as *mut usize, n1);
        let n2 = nome(cpy8_d(ficha + 0x16));
        core::ptr::write_unaligned((ficha + 8) as *mut usize, n2);
        *((ficha + 0x25) as *mut u8) = if sortear(10) != 0 {
            (sortear(0x10) as u8).wrapping_add(5)
        } else {
            let a = sortear(0xb) as u8;
            a.wrapping_add(sortear(0xb) as u8)
        };
        let r5 = sortear(5);
        let base = (cpy8_w(0xae2c92) as u16).wrapping_sub(0x1b);
        core::ptr::write_unaligned((ficha + 0xe) as *mut u16, (r5 as u16).wrapping_add(base));
        core::ptr::write_unaligned((ficha + 0xc) as *mut u16, sortear(0x154) as u16);
    }
    if n > 0 {
        let linha = cpy8_d(this).wrapping_add(ini.wrapping_mul(0x11) as usize);
        *((linha + 0xf) as *mut u8) = 0;
        core::ptr::write_unaligned((linha + 8) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xa) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((linha + 4) as *mut u32, 0);
    }
    1
}

/// Comparador: lado a lado pelas fichas de todos os árbitros ([0xae2390] × 0x2b), pelas linhas de estatísticas
/// ([0xae2390] × 17), pelo estado do sorteio e por [0xb67a34], e pela devolução. Sem tabela de árbitros (a caixa a cada
/// ficha): só a original.
unsafe extern "thiscall" fn comparar_renew_officials(this: usize, comp: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%);
    let total = cpy8_i(0xae2390).max(0) as usize;
    if cpy8_d(0xae23e8) == 0 || cpy8_d(this) == 0 {
        return o(this, comp);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(cpy8_d(0xae23e8), total * 0x2b), (cpy8_d(this), total * 0x11), (0xb67a34, 4)]);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, comp),
        || crate::aleatorio::sem_contar(|| renew_officials(this, comp)));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("renew_officials(comp {:#x}) original={} rust={}, diferencas em {:x?}", comp, r, n, dif)
    });
    r
}
