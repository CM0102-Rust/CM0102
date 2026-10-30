// ---------- %%LEVA%%a leva: o fim da temporada e as transferências nos registros: o jogador do ano da torcida, arquivar as estatísticas da nação e os recordes de valor de transferência (club_records.cpp, player_stats.cpp) ----------

unsafe fn cpp10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Um double do .rdata do exe.
unsafe fn cpp10_q(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpp10_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    e_selecao(clube) != 0
}

/// O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cpp10_clube(i: i32) -> usize {
    cpp10_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245))
}

/// A nota da torcida para o jogador `p` do clube pela estatística `st` (0x452da0): média (+0xe com sinal / jogos +0)
/// + 0,025 × +2 + 1/60 × +6; o clube gosta dele (0x547790) +0,2, não gosta (0x5477d0) −2 (guardada); ele gosta do
/// clube (0x547d60) +0,2, não gosta (0x547d30) −1,5 (só na comparação).
unsafe fn cpp10_nota_da_torcida(p: usize, clube: usize, st: usize) -> f64 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let desafeto: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let media = cpp10_w(st + 0xe) as i32 as f64 / cpp10_b(st) as i32 as f64;
    let mut nota = media + cpp10_b(st + 2) as i32 as f64 * cpp10_q(0x967878);
    nota = nota + cpp10_b(st + 6) as i32 as f64 * cpp10_q(0x968958);
    if favorito(p, clube) != 0 {
        crate::comparacao::ramo("cpp_10 favorito do clube");
        nota = nota + cpp10_q(0x968918); // o clube gosta dele
    } else if desafeto(p, clube) != 0 {
        nota = nota - cpp10_q(0x968950);
    }
    if gosta(p, clube) != 0 {
        nota + cpp10_q(0x968918)
    } else if nao_gosta(p, clube) != 0 {
        nota - cpp10_q(0x968940)
    } else {
        nota
    }
}

/// { 0x452da0 } (thiscall, ret 8, club_records.cpp): `Club_Records::pick_fans_player_of_year(clube, registro)`: sem
/// jogos (+0x10 com sinal do registro de 106 bytes do clube em [this+0x1c]) e reputação (+0x80) abaixo de 3000, 0.
/// Com o fator 0,6 caindo de 0,1 enquanto ninguém foi escolhido e passa de 0,2: o mínimo de jogos é +0x10 × fator
/// (_ftol); dos 50 do elenco (+0xd7) jogadores (+0x61, id abaixo de [0xae2364]) do clube (+0x39) com a estatística 6
/// (0x7c3920 sobre [0xb74350]) de jogos acima de 0 e do mínimo (16 bits com sinal), o de maior nota
/// (`cpp10_nota_da_torcida`, acima de −1000). Ninguém: 0; senão a referência dele (0x7e4920) em registro +0x371 e 1.
unsafe extern "thiscall" fn jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let jogos_do_clube = || cpp10_b(cpp10_d(this + 0x1c).wrapping_add((cpp10_i(clube) as usize).wrapping_mul(106)) + 0x10) as i8;
    if jogos_do_clube() == 0 && cpp10_w(clube + 0x80) < 0xbb8 {
        return 0;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    let mut fator = 0.6f64;
    let mut melhor = -1000.0f64;
    let mut escolhido = 0usize;
    while escolhido == 0 {
        let minimo = ftol(jogos_do_clube() as i32 as f64 * fator) as i16;
        for s in 0..50usize {
            let p = cpp10_d(clube + 0xd7 + 4 * s);
            if p == 0 || cpp10_i(p) >= cpp10_i(0xae2364) {
                continue;
            }
            let jogador = cpp10_d(p + 0x61);
            if jogador == 0 || cpp10_d(p + 0x39) != clube {
                continue;
            }
            let st = estatistica(cpp10_d(0xb74350), jogador, 6);
            if st == 0 {
                continue;
            }
            let jogos = cpp10_b(st);
            if (jogos as i16) < minimo || jogos == 0 {
                continue;
            }
            let nota = cpp10_nota_da_torcida(p, clube, st);
            if nota > melhor {
                melhor = nota;
                escolhido = p;
            }
        }
        fator -= cpp10_q(0x967880);
        if !(fator > cpp10_q(0x968918)) {
            break;
        }
    }
    if escolhido == 0 {
        return 0;
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    referencia(escolhido, registro + 0x371);
    1
}

/// { 0x7c1350 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::end_season_for_key_nation(nacao)`: sem nação, a
/// caixa "Error" da linha 0x396 e 0. Percorre os clubes de [0xae235c] até a primeira seleção (0x53b900: para ali);
/// clube da nação (0x669360, registro de 72 bytes em 0xb63d60): cada jogador (+0x61, id abaixo de [0xae2364]) das 50
/// vagas (+0xd7) sem empréstimo (0x4cf6d0 sobre [0xae1050]) ou emprestado ao próprio clube (+0x39) tem as estatísticas
/// arquivadas (0x7c5fe0 com −1, 0, 0, −1, 0). Devolve 1.
unsafe extern "thiscall" fn arquivar_estatisticas_da_nacao(this: usize, nacao: usize) -> u32 {
    if nacao == 0 {
        aviso_em(0xa78310, 0x396);
        return 0;
    }
    let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let arquivar: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
    let mut i = 0i32;
    while i < cpp10_i(0xae235c) {
        let clube = cpp10_clube(i);
        if cpp10_selecao(clube) {
            break; // a primeira seleção encerra
        }
        let k = indice(clube) as u8 as i8 as isize; // só o al
        if (0xb63d60usize).wrapping_add((k * 72) as usize) == nacao {
            for s in 0..50usize {
                let p = cpp10_d(clube + 0xd7 + 4 * s);
                if p == 0 || cpp10_d(p + 0x61) == 0 || cpp10_i(p) >= cpp10_i(0xae2364) {
                    continue;
                }
                if emprestimo(cpp10_d(0xae1050), p) == 0 || cpp10_d(p + 0x39) == clube {
                    crate::comparacao::ramo("cpp_10 arquivar");
                    arquivar(this, p, -1, 0, 0, -1, 0);
                }
            }
        }
        i += 1;
    }
    1
}

/// O recorde de transferência já está na fila de notícias (6 bytes: id do clube, tipo) de [this+0x24], conta +0x2c?
unsafe fn cpp10_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let fila = cpp10_d(this + 0x24);
    (0..cpp10_i(this + 0x2c).max(0) as usize).any(|k| cpp10_d(fila + 6 * k) == id && cpp10_b(fila + 6 * k + 4) == tipo)
}

/// A transferência de recorde do clube passa de 1000 × (reputação (+0x80) + 300)?
unsafe fn cpp10_grande(valor: i32, clube: usize) -> bool {
    valor >= (cpp10_w(clube + 0x80) as i32 + 300).wrapping_mul(1000)
}

/// Copia o bloco de recorde de `o` (+`d`: valor, a referência de 0x18 bytes, o clube, a data, a marca) para `a`.
unsafe fn cpp10_copiar_recorde(a: usize, o: usize, d: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut rascunho = [0u8; 8];
    for k in 0..5 {
        core::ptr::write_unaligned((a + d + 4 * k) as *mut usize, cpp10_d(o + d + 4 * k));
    }
    copiar(a + d + 0x14, rascunho.as_mut_ptr(), o + d + 0x14);
    core::ptr::write_unaligned((a + d + 0x1c) as *mut usize, cpp10_d(o + d + 0x1c));
    copiar(a + d + 0x20, rascunho.as_mut_ptr(), o + d + 0x20);
    *((a + d + 0x28) as *mut u8) = cpp10_b(o + d + 0x28);
}

/// { 0x448760 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_transfer_fee_records(transferencia)`: a
/// pessoa (+0xc) e o clube comprador (+8); o contrato dela (0x4cf5a0 sobre [0xae1050], guardado em [0xae1080]) dá o
/// vendedor (+4). Os registros da temporada (0x4528d0) e de sempre (0x452710) do comprador (criados); sem vendedor,
/// 1. Valor (+0x1b) acima do recorde pago da temporada (+0x313): ele, a referência da pessoa (0x7e4920), o vendedor,
/// a data de hoje e o empréstimo (+4 ≠ 0); acima do de sempre, copiado para ele, e se passa de 1000 × (reputação +
/// 300): o recorde na fila de notícias (0x452cb0, 0x1c, se ainda não está) e o técnico (+0xbf) passa a gostar da pessoa
/// (0x547a10, forçado). O mesmo para o vendedor no recorde recebido (+0x33c; a fila com 0x1d, sem o gostar). 1.
unsafe extern "thiscall" fn recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let pessoa = cpp10_d(0xae23bc).wrapping_add((cpp10_i(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp10_clube(cpp10_i(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpp10_d(0xae1050), pessoa);
    core::ptr::write_unaligned(0xae1080 as *mut usize, contrato);
    let vendedor = if contrato != 0 { cpp10_clube(cpp10_i(contrato + 4)) } else { 0 };
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    let mut rascunho = [0u8; 8];
    let s = temporada(this, comprador, 1);
    let a = sempre(this, comprador, 1);
    if vendedor == 0 {
        return 1;
    }
    let valor = cpp10_i(t + 0x1b);
    if valor != 0 && valor > cpp10_i(s + 0x313) {
        crate::comparacao::ramo("cpp_10 recorde pago");
        core::ptr::write_unaligned((s + 0x313) as *mut i32, valor);
        referencia(pessoa, s + 0x317);
        core::ptr::write_unaligned((s + 0x32f) as *mut usize, cpp10_d(vendedor));
        copiar(s + 0x333, rascunho.as_mut_ptr(), 0xae2c90);
        *((s + 0x33b) as *mut u8) = (cpp10_d(t + 4) != 0) as u8;
        if cpp10_i(t + 0x1b) > cpp10_i(a + 0x313) {
            cpp10_copiar_recorde(a, s, 0x313);
            if cpp10_grande(cpp10_i(a + 0x313), comprador) {
                crate::comparacao::ramo("cpp_10 transferencia grande");
                if !cpp10_na_fila(this, cpp10_d(comprador), 0x1c) {
                    noticia(this, cpp10_d(comprador) as u32, 0x1c);
                }
                let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
                gostar(cpp10_d(comprador + 0xbf), pessoa, 1); // forçado
            }
        }
    }
    let s = temporada(this, vendedor, 1);
    let a = sempre(this, vendedor, 1);
    let valor = cpp10_i(t + 0x1b);
    if valor == 0 || valor <= cpp10_i(s + 0x33c) {
        return 1;
    }
    core::ptr::write_unaligned((s + 0x33c) as *mut i32, valor);
    referencia(pessoa, s + 0x340);
    core::ptr::write_unaligned((s + 0x358) as *mut usize, cpp10_d(comprador));
    copiar(s + 0x35c, rascunho.as_mut_ptr(), 0xae2c90);
    *((s + 0x364) as *mut u8) = (cpp10_d(t + 4) != 0) as u8;
    if cpp10_i(t + 0x1b) > cpp10_i(a + 0x33c) {
        cpp10_copiar_recorde(a, s, 0x33c);
        if cpp10_grande(cpp10_i(a + 0x33c), vendedor) && !cpp10_na_fila(this, cpp10_d(vendedor), 0x1d) {
            noticia(this, cpp10_d(vendedor) as u32, 0x1d);
        }
    }
    1
}

/// Comparador: lado a lado pela referência do escolhido (registro +0x371, 0x18 bytes) e pela devolução.
unsafe extern "thiscall" fn comparar_jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(registro + 0x371, 0x18)], || o(this, clube, registro), || jogador_do_ano_da_torcida(this, clube, registro));
    PLACAR.registrar(igual && r == n, || format!("jogador_do_ano_da_torcida(clube {:x}) original={} {:x?} rust={} {:x?}", cpp10_d(clube), r, mo, n, mr));
    r
}

/// Comparador: a 0x7c5fe0 move as estatísticas para o histórico (aloca): roda só a original e confere a devolução
/// (1, ou 0 sem nação).
unsafe extern "thiscall" fn comparar_arquivar_estatisticas_da_nacao(this: usize, nacao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, nacao);
    PLACAR.registrar(r == (nacao != 0) as u32, || format!("arquivar_estatisticas_da_nacao({:x}) devolveu {}", nacao, r));
    r
}

/// Comparador: a original põe na fila de notícias (realoca) e o gostar sorteia: o plano antes (os quatro registros
/// criados na mesma ordem da original, e os recordes pago e recebido esperados); a original roda; pega os registros
/// de novo (o vetor pode ter sido realocado) e confere os valores e o clube de cada recorde e o [0xae1080].
unsafe extern "thiscall" fn comparar_recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let pessoa = cpp10_d(0xae23bc).wrapping_add((cpp10_i(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp10_clube(cpp10_i(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpp10_d(0xae1050), pessoa);
    let vendedor = if contrato != 0 { cpp10_clube(cpp10_i(contrato + 4)) } else { 0 };
    if vendedor == 0 {
        let r = o(this, t);
        PLACAR.registrar(r == 1 && cpp10_d(0xae1080) == 0, || format!("recordes_de_valor_de_transferencia sem vendedor: {} [0xae1080]={:x}", r, cpp10_d(0xae1080)));
        return r;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let registros = || [temporada(this, comprador, 1), sempre(this, comprador, 1), temporada(this, vendedor, 1), sempre(this, vendedor, 1)];
    let valor = cpp10_i(t + 0x1b);
    let rg = registros();
    // (valor, clube) de cada recorde: pago da temporada e de sempre do comprador, recebido do vendedor
    let mut plano = [(cpp10_i(rg[0] + 0x313), cpp10_d(rg[0] + 0x32f)), (cpp10_i(rg[1] + 0x313), cpp10_d(rg[1] + 0x32f)),
                     (cpp10_i(rg[2] + 0x33c), cpp10_d(rg[2] + 0x358)), (cpp10_i(rg[3] + 0x33c), cpp10_d(rg[3] + 0x358))];
    let mut fila = cpp10_i(this + 0x2c);
    if valor != 0 {
        if valor > plano[0].0 {
            plano[0] = (valor, cpp10_d(vendedor));
            if valor > plano[1].0 {
                plano[1] = plano[0];
                if cpp10_grande(valor, comprador) && !cpp10_na_fila(this, cpp10_d(comprador), 0x1c) {
                    fila += 1;
                }
            }
        }
        if valor > plano[2].0 {
            plano[2] = (valor, cpp10_d(comprador));
            if valor > plano[3].0 {
                plano[3] = plano[2];
                if cpp10_grande(valor, vendedor) && !cpp10_na_fila(this, cpp10_d(vendedor), 0x1d) {
                    fila += 1;
                }
            }
        }
    }
    let r = o(this, t);
    let rg = registros();
    let agora = [(cpp10_i(rg[0] + 0x313), cpp10_d(rg[0] + 0x32f)), (cpp10_i(rg[1] + 0x313), cpp10_d(rg[1] + 0x32f)),
                 (cpp10_i(rg[2] + 0x33c), cpp10_d(rg[2] + 0x358)), (cpp10_i(rg[3] + 0x33c), cpp10_d(rg[3] + 0x358))];
    let fila_agora = cpp10_i(this + 0x2c);
    PLACAR.registrar(r == 1 && agora == plano && fila_agora == fila && cpp10_d(0xae1080) == contrato, || {
        format!("recordes_de_valor_de_transferencia(valor {}) plano {:x?} fila {} x {:x?} fila {}", valor, plano, fila, agora, fila_agora)
    });
    r
}
