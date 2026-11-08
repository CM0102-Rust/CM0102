// ---------- %%LEVA%%a leva: o dia das transferências: o dia da gerência, a oferta que vence hoje e as restrições da oferta (transfer_manager.cpp) ----------

unsafe fn cqd1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// A pessoa do id (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cqd1_pessoa(id: usize) -> usize {
    cqd1_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do id (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqd1_clube(id: usize) -> usize {
    cqd1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080] como
/// a original faz, e o clube dele (+4); sem contrato, ou com o tipo 5 em +0x35 quando `sem_5`: 0.
unsafe fn contract_club_filtered(pessoa: usize, sem_5: bool) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqd1_d(0xae1050), pessoa);
    cqd1_pd(0xae1080, c);
    if c == 0 || (sem_5 && cqd1_b(c + 0x35) & 0x3f == 5) {
        0
    } else {
        cqd1_clube(cqd1_d(c + 4))
    }
}

/// O clube é humano (0x600a50: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
unsafe fn transfer_manager_is_human_club(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, 1, 0, 0) != 0
}

/// Reagenda a oferta (0x8c6220: push 0; push dias; push oferta; mov ecx, this -> thiscall, ret 0xc).
unsafe fn schedule_offer_days(this: usize, o: usize, dias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    f(this, o, dias, 0);
}

/// O próximo da lista (0x68d1b0: mov ecx, lista -> thiscall, ret 0).
unsafe fn transfer_manager_llist_next(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    f(l) as usize
}

/// O bloco que a 0x8c63a0 repete: o clube do contrato do alvo (sem olhar o tipo 5), os lances do comprador (+8) pelo
/// alvo (+0xc) desmarcados na lista de observação (0x84aeb0: push jogador; push clube; mov ecx, [this+0x856] ->
/// thiscall, ret 8) e os do clube do contrato por cada jogador da troca (lista em +4, cursor no começo).
unsafe fn clear_offer_bid_markers(this: usize, o: usize) {
    let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let atual = contract_club_filtered(alvo, false);
    desmarcar(cqd1_d(this + 0x856), cqd1_clube(cqd1_d(o + 8)), alvo);
    let l = cqd1_d(o + 4);
    if l == 0 {
        return;
    }
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = transfer_manager_llist_next(cqd1_d(o + 4));
        if p == 0 {
            break;
        }
        desmarcar(cqd1_d(this + 0x856), atual, cqd1_pessoa(cqd1_d(p)));
    }
}

/// { 0x8c5880 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::daily_update(hoje, fatia)`. Sem data base (ano +0x84f zerado), ela vira hoje (0x417c10: push
/// hoje; push saída; lea ecx, [this+0x84d] -> thiscall, ret 8) e a fatia +0x855 a de hoje. Só com hoje igual à base e a
/// fatia igual a +0x855: na fatia 0 os avisos das janelas (0x8c6bc0: push hoje; mov ecx, this -> thiscall, ret 4); no
/// dia múltiplo de 7, cada oferta vencida (data +0x12/+0x14 antes de hoje; status +0x2c fora de 0x13, 7, 8 e 0xa) é
/// reagendada para amanhã (0x8c6220 com 1 dia); a fatia +0x855 sobe 1; a fila da transferência livre (0x8c18c0: mov
/// ecx, this -> thiscall, ret 0); o balde da vez (listas de 0xe bytes em this+0x18 + 0xe·(3·[+0x84c] + fatia), cursor no
/// começo): cada índice válido do vetor [this] (conta +4) com a oferta de hoje e desta fatia (+0x1a) é processado
/// (0x8c63a0: push vaga; mov ecx, this -> thiscall, ret 4); o balde é esvaziado (0x68d080). Na fatia 2: a base passa a
/// amanhã (0x54c5b0 push 1; push saída; mov ecx, base -> thiscall, ret 8 e 0x417c10), +0x855 volta a 0, +0x84c sobe (de
/// 49 volta a 0); mudando o mês de hoje para amanhã (0x54cfb0: mov ecx, data -> thiscall, ret 0), as prestações
/// (0x8caee0); no dia 0, as transferências antigas aos arquivos (0x8c3eb0) e as permissões de trabalho (0x8c7330); o
/// virtual +0 de cada uma das 28 regras de nação ([this+0x8ac], push hoje; mov ecx, regras); no dia múltiplo de 7, as
/// permissões de novo. Na fatia 0 (fora a 2), as transferências futuras (0x8cbbe0). No dia 100 da fatia 2: as datas 100
/// de 5, 2 e 10 anos atrás (0x549ef0: push ano; push 0x64; lea ecx, local -> thiscall, ret 8) e, para cada pessoa
/// (registros de 0x6e bytes em [0xae23bc], conta [0xae2364], relida) com a data +0x3e/+0x40 antes da de 2 anos, clube
/// ([[+0x39]+0x53]) numa nação ([+0x71]) de id [0x9cfa10] e sem ser comunitária lá (0x53d740: push clube; push pessoa;
/// add esp, 8 -> cdecl): +0x1e (a segunda nacionalidade) = a nação do clube (clube +0x53), se ela é [0x9cf488] ou [0x9cf434]; se a data é
/// antes da de 5 anos e ela não é [0x9cf360], [0x9cf2fc] ou [0x9cf448]; ou se a data é antes da de 10 anos.
unsafe extern "thiscall" fn transfer_manager_daily_update(this: usize, hoje: usize, fatia: u32) {
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mes: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let chamar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c18c0usize);
    let janelas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6bc0usize);
    let processar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c63a0usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let prestacoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8caee0usize);
    let antigas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c3eb0usize);
    let permissoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8c7330usize);
    let futuras: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8cbbe0usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let f = fatia as u8;
    let mut t20 = [0u8; 8];
    if cqd1_w(this + 0x84f) == 0 {
        copia(this + 0x84d, t20.as_mut_ptr() as usize, hoje);
        cqd1_pb(this + 0x855, f);
    }
    if cqd1_w(hoje + 2) != cqd1_w(this + 0x84f) || cqd1_w(hoje) != cqd1_w(this + 0x84d) || cqd1_b(this + 0x855) != f {
        return;
    }
    if f == 0 {
        janelas(this, hoje);
    }
    if cqd1_w(hoje) as i32 % 7 == 0 {
        let mut i: i32 = 0;
        while i < cqd1_d(this + 4) as i32 {
            let o = cqd1_d(cqd1_d(this).wrapping_add((i as usize).wrapping_mul(4)));
            if o != 0 {
                let (ha, oa) = (cqd1_w(hoje + 2) as u16, cqd1_w(o + 0x14) as u16);
                let vencida = ha > oa || (ha == oa && cqd1_w(hoje) > cqd1_w(o + 0x12));
                let s = cqd1_b(o + 0x2c);
                if vencida && s != 0x13 && s != 7 && s != 8 && s != 0xa {
                    schedule_offer_days(this, o, 1);
                }
            }
            i += 1;
        }
    }
    cqd1_pb(this + 0x855, cqd1_b(this + 0x855).wrapping_add(1));
    chamar(this);
    let balde = |this: usize| {
        let k = (cqd1_b(this + 0x84c) as i8 as i32) * 3 + f as i8 as i32;
        this.wrapping_add((k as isize * 0xe) as usize).wrapping_add(0x18)
    };
    let l = balde(this);
    cqd1_pd(l + 0xa, cqd1_d(l + 2));
    loop {
        let p = transfer_manager_llist_next(balde(this));
        if p == 0 {
            break;
        }
        let idx = cqd1_d(p) as i32;
        if idx < cqd1_d(this + 4) as i32 && idx >= 0 {
            let vaga = cqd1_d(this).wrapping_add(idx as usize * 4);
            let o = cqd1_d(vaga);
            if o != 0 && cqd1_w(hoje + 2) == cqd1_w(o + 0x14) && cqd1_w(hoje) == cqd1_w(o + 0x12)
                && cqd1_b(o + 0x1a) == f { // cqd1: a oferta desta fatia
                crate::comparison::branch("cqd_1 oferta do dia");
                processar(this, vaga);
            }
        }
    }
    esvaziar(balde(this));
    if f == 2 {
        let (mut t28, mut t30) = ([0u8; 8], [0u8; 8]);
        let r = mais(this + 0x84d, t28.as_mut_ptr() as usize, 1);
        copia(this + 0x84d, t30.as_mut_ptr() as usize, r);
        cqd1_pb(this + 0x855, 0);
        let j = cqd1_b(this + 0x84c).wrapping_add(1);
        cqd1_pb(this + 0x84c, j);
        if (j as i8) >= 0x32 {
            cqd1_pb(this + 0x84c, 0);
        }
        let amanha = mais(hoje, t30.as_mut_ptr() as usize, 1);
        let m = mes(hoje);
        if m != mes(amanha) {
            crate::comparison::branch("cqd_1 prestações do mês");
            prestacoes(this); // cqd1: as prestações na virada do mês
        }
        if cqd1_w(hoje) == 0 {
            antigas(this);
            permissoes(this);
        }
        for k in 0..0x1c {
            let r = cqd1_d(this + 0x8ac + k * 4);
            if r != 0 {
                let v: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cqd1_d(cqd1_d(r)));
                v(r, hoje);
            }
        }
        if cqd1_w(hoje) as i32 % 7 == 0 {
            permissoes(this);
        }
    } else if f == 0 {
        futuras(this);
    }
    if cqd1_w(hoje) != 100 || f != 2 {
        return;
    }
    // ponytail: a original empurra o ano em eax com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    let ano = cqd1_w(hoje + 2) as u16;
    let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
    let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
    let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
    let mut s = cqd1_d(0xae23bc);
    let mut i: i32 = 0;
    while i < cqd1_d(0xae2364) as i32 {
        if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) {
            cqd1_pd(s + 0x1e, cqd1_d(cqd1_d(s + 0x39) + 0x53));
        }
        i += 1;
        s += 0x6e;
    }
}

/// A condição do dia 100 da 0x8c5880 (também para o comparador): a data +0x3e (dia com sinal)/+0x40 (ano sem sinal) da
/// pessoa antes da de 2 anos, o clube na nação [0x9cfa10] sem ser comunitária, e o caso pelo id do clube.
unsafe fn meets_residency_naturalisation(s: usize, p2: usize, p5: usize, p10: usize, comunitaria: unsafe extern "C" fn(usize, usize) -> u32) -> bool {
    let antes = |d: usize| {
        let (da, sa) = (cqd1_w(d + 2) as u16, cqd1_w(s + 0x40) as u16);
        da > sa || (da == sa && cqd1_w(d) > cqd1_w(s + 0x3e))
    };
    if !antes(p2) {
        return false;
    }
    let j = cqd1_d(s + 0x39);
    if j == 0 {
        return false;
    }
    let c = cqd1_d(j + 0x53);
    if c == 0 {
        return false;
    }
    let n = cqd1_d(c + 0x71);
    if n == 0 || cqd1_d(n) != cqd1_d(0x9cfa10) || comunitaria(s, c) != 0 {
        return false;
    }
    let id = cqd1_d(c);
    if id == cqd1_d(0x9cf488) || id == cqd1_d(0x9cf434) {
        return true;
    }
    if antes(p5) && id != cqd1_d(0x9cf360) && id != cqd1_d(0x9cf2fc) && id != cqd1_d(0x9cf448) {
        return true;
    }
    antes(p10)
}

/// Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e (a
/// segunda nacionalidade) = a nação do clube: a naturalização do dia 100.
unsafe extern "thiscall" fn comparar_transfer_manager_daily_update(this: usize, hoje: usize, fatia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let f = fatia as u8;
    let sem_base = cqd1_w(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        cqd1_w(hoje + 2) == cqd1_w(this + 0x84f) && cqd1_w(hoje) == cqd1_w(this + 0x84d) && cqd1_b(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { cqd1_b(this + 0x855) }, cqd1_b(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let r = mais(hoje, amanha.as_mut_ptr() as usize, 1);
        amanha = core::ptr::read_unaligned(r as *const [u8; 8]);
    }
    o(this, hoje, fatia);
    if !roda {
        return;
    }
    let mut ok;
    if f == 2 {
        let j = j84c.wrapping_add(1);
        let esperado = if (j as i8) >= 0x32 { 0 } else { j };
        let base = core::ptr::read_unaligned((this + 0x84d) as *const [u8; 4]);
        ok = cqd1_b(this + 0x855) == 0 && cqd1_b(this + 0x84c) == esperado && base[..] == amanha[..4];
    } else {
        ok = cqd1_b(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && cqd1_w(hoje) == 100 {
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
        let ano = cqd1_w(hoje + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
        let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
        let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
        let mut s = cqd1_d(0xae23bc);
        for _ in 0..(cqd1_d(0xae2364) as i32).max(0) {
            if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) && cqd1_d(s + 0x1e) != cqd1_d(cqd1_d(s + 0x39) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.record(ok, || format!("transfer_manager_daily_update(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem a nação do clube em +0x1e {}",
        cqd1_w(hoje), f, f855, cqd1_b(this + 0x855), j84c, cqd1_b(this + 0x84c), erradas));
}

/// { 0x8c63a0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH: as listas locais, construídas pela 0x68d020 e
/// destruídas pela 0x68d040 na mesma ordem): `TRANSFER_MANAGER::process_due_offer(vaga)`, a oferta que vence hoje, pelo
/// status +0x2c. O vendedor é o clube do contrato do alvo (+0xc; fora o tipo 5), o comprador o +8.
/// - 0: sem vendedor, a aceitação do lance (0x8c0080: push 0; push vaga -> thiscall, ret 8). Vendedor humano: status 4,
///   14 dias (0x8c6220) e os lances desmarcados (clear_offer_bid_markers). Senão, sem a marca +0x1f = 1, a resposta da IA
///   (0x8c8090: push 0; push vaga, ret 8); com ela, a previsão do vendedor (0x84af00, stdcall(vendedor, alvo, 0 i64,
///   comprador, −1 i64, 1): o ecx = [this+0x856] não é lido): recusa, a do empréstimo (0x8bfa30: push 0; push 0; push
///   vaga -> thiscall, ret 0xc); aceita, a decisão (0x4cc930: push oferta; mov ecx, [0xae1050] -> thiscall, ret 4),
///   status 8 e a notícia da aceitação (0x8d76d0: push oferta, ret 4); com a decisão, a notícia da recusa em se mudar
///   (0x8d8310: push 0; push oferta, ret 8), status 0xf, +0x2e = 3, os lances desmarcados, 10 dias, e se a oferta e as
///   da lista em +0 estão fechadas (0x8d0d90, stdcall(oferta)), o bit 0 do registro de 8 bytes do alvo em [this+0x86e]
///   apagado.
/// - 2, 3, 4, 0xf, 0x10, 0x14: com 0xf e +0x2e < 3, +0x2e = 3 e 10 dias; senão a oferta sai (0x8d0f60: push oferta,
///   ret 4).
/// - 1: comprador humano: status 4, os lances desmarcados e 10 dias; senão 0x8c0f40 (push vaga, ret 4) com +0x1f = 1 ou
///   0x8c0810 (push 0; push vaga, ret 8).
/// - 5, 6: comprador humano: status 9 e 10 dias; senão o contrato ao alvo (0x8c1e40: push oferta; push alvo; push
///   comprador, ret 0xc).
/// - 7 com troca (+4): vendedor humano: status 8 e 10 dias; senão, numa cópia da lista (0x68d2f0: push lista; lea ecx,
///   local, ret 4), o contrato a cada jogador da troca até a oferta ficar ativa.
/// - 0xa: as permissões (0x8c7690: push 0; push oferta, ret 8). 0xe: a conclusão (0x8c34e0: push 0; push oferta, ret 8).
/// - 0x11, 0x12: status 0; com +0x10, a oferta passa aos clubes (0x8c2f50: push oferta, ret 4); senão, com técnico
///   humano no clube do contrato (0x7875e0, stdcall(clube): o ecx = [0xb7431c] não é lido), a notícia da oferta feita
///   (0x8d3090: push oferta, ret 4) e 28 dias; senão o valor da oferta (0x8e7b50: mov ecx, oferta -> thiscall(oferta, 0,
///   0, 0, 0, 1), ret 0x14) e a previsão do vendedor (vendedor, alvo, valor, comprador, −1, 0): 0 aceita (0x8c0080),
///   1 ou 2 recusa (0x8bf680: push 0; push 0; push vaga, ret 0xc).
unsafe extern "thiscall" fn process_due_offer(this: usize, vaga: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let mut l1 = [0u8; 0x10];
    lista_nova(l1.as_mut_ptr() as usize);
    process_due_offer_body(this, vaga);
    destruir(l1.as_mut_ptr() as usize);
}

unsafe fn process_due_offer_body(this: usize, vaga: usize) {
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0080usize);
    let respond: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c8090usize);
    let prever: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(0x84af00usize);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bfa30usize);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8bf680usize);
    let decidir: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cc930usize);
    let aceita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d76d0usize);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8d8310usize);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x8d0d90usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d0f60usize);
    let ia_1a: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c0f40usize);
    let ia_1b: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c0810usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8c1e40usize);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8c7690usize);
    let concluir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8c34e0usize);
    let pass_ball: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia_feita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8d3090usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let o = cqd1_d(vaga);
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = contract_club_filtered(alvo, true);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let s = cqd1_b(o + 0x2c);
    if s == 0 {
        if vendedor == 0 {
            aceitar(this, vaga, 0);
            return;
        }
        if transfer_manager_is_human_club(vendedor) {
            cqd1_pb(o + 0x2c, 4);
            schedule_offer_days(this, o, 0xe);
            clear_offer_bid_markers(this, o);
            return;
        }
        if cqd1_b(cqd1_d(vaga) + 0x1f) != 1 {
            crate::comparison::branch("cqd_1 resposta da IA");
            respond(this, vaga, 0); // cqd1: a resposta da IA ao lance
            return;
        }
        if prever(vendedor, alvo, 0, comprador, -1, 1) != 0 {
            recusar_emprestimo(this, vaga, 0, 0);
            return;
        }
        let r = decidir(cqd1_d(0xae1050), cqd1_d(vaga));
        cqd1_pb(cqd1_d(vaga) + 0x2c, 8);
        aceita(this, cqd1_d(vaga));
        if r == 0 {
            return;
        }
        recusa_mudar(this, cqd1_d(vaga), 0);
        cqd1_pb(cqd1_d(vaga) + 0x2c, 0xf);
        cqd1_pb(cqd1_d(vaga) + 0x2e, 3);
        clear_offer_bid_markers(this, cqd1_d(vaga));
        schedule_offer_days(this, cqd1_d(vaga), 0xa);
        let o3 = cqd1_d(vaga);
        if fechada(o3) == 0 {
            return;
        }
        let l = cqd1_d(o3);
        if l != 0 {
            cqd1_pd(l + 0xa, cqd1_d(l + 2));
            loop {
                let p = transfer_manager_llist_next(cqd1_d(o3));
                if p == 0 {
                    break;
                }
                if fechada(cqd1_d(cqd1_d(this).wrapping_add(cqd1_d(p).wrapping_mul(4)))) == 0 {
                    return;
                }
            }
        }
        let m = cqd1_d(this + 0x86e).wrapping_add(cqd1_d(o3 + 0xc).wrapping_mul(8));
        if cqd1_b(m) & 1 != 0 {
            cqd1_pb(m, cqd1_b(m) & 0xfe);
        }
        return;
    }
    if matches!(s, 2 | 3 | 4 | 0xf | 0x10 | 0x14) {
        if s == 0xf && cqd1_b(o + 0x2e) < 3 {
            cqd1_pb(o + 0x2e, 3);
            schedule_offer_days(this, o, 0xa);
        } else {
            tirar(this, o);
        }
        return;
    }
    if s == 1 {
        if transfer_manager_is_human_club(comprador) {
            cqd1_pb(o + 0x2c, 4);
            clear_offer_bid_markers(this, o);
            schedule_offer_days(this, o, 0xa);
        } else if cqd1_b(cqd1_d(vaga) + 0x1f) == 1 {
            ia_1a(this, vaga);
        } else {
            ia_1b(this, vaga, 0);
        }
        return;
    }
    if s == 5 || s == 6 {
        if transfer_manager_is_human_club(comprador) {
            cqd1_pb(o + 0x2c, 9);
            schedule_offer_days(this, o, 0xa);
        } else {
            crate::comparison::branch("cqd_1 contrato ao jogador");
            contrato(this, comprador, cqd1_pessoa(cqd1_d(o + 0xc)), o); // cqd1: o contrato ao alvo
        }
        return;
    }
    if cqd1_d(o + 4) != 0 && s == 7 {
        if transfer_manager_is_human_club(vendedor) {
            cqd1_pb(o + 0x2c, 8);
            schedule_offer_days(this, o, 0xa);
            return;
        }
        let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
        let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d2f0usize);
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let mut l2 = [0u8; 0x10];
        let l = l2.as_mut_ptr() as usize;
        lista_nova(l);
        copiar(l, cqd1_d(o + 4));
        cqd1_pd(l + 0xa, cqd1_d(l + 2));
        loop {
            let p = transfer_manager_llist_next(l);
            if p == 0 {
                break;
            }
            contrato(this, vendedor, cqd1_pessoa(cqd1_d(p)), o);
            let s2 = cqd1_b(o + 0x2c);
            if matches!(s2, 2 | 3 | 4 | 0x10 | 0x14) || (s2 == 0xf && cqd1_b(o + 0x2e) >= 3) {
                break;
            }
        }
        destruir(l);
        return;
    }
    if s == 0xa {
        permissoes(this, o, 0);
        return;
    }
    if s == 0xe {
        concluir(this, o, 0);
        return;
    }
    if s != 0x11 && s != 0x12 {
        return;
    }
    cqd1_pb(o + 0x2c, 0);
    if cqd1_b(o + 0x10) != 0 {
        pass_ball(this, o);
        return;
    }
    let clube = contract_club_filtered(cqd1_pessoa(cqd1_d(o + 0xc)), false);
    if human_manager_record_checked(clube) != 0 {
        noticia_feita(this, o);
        schedule_offer_days(this, o, 0x1c);
        return;
    }
    let v = valor(o, 0, 0, 0, 0, 1);
    let r = prever(vendedor, cqd1_pessoa(cqd1_d(o + 0xc)), v as i64, comprador, -1, 0);
    if r == 0 {
        aceitar(this, vaga, 0);
    } else if (r as i8) > 0 && (r as i8) <= 2 {
        recusar(this, vaga, 0, 0);
    }
}

/// Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
unsafe extern "thiscall" fn comparar_process_due_offer(this: usize, vaga: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, vaga) // ponytail: só a original
}

/// { 0x8cfc90 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH: a data e a lista locais, a lista construída pela
/// 0x68d020 e destruída pela 0x68d040): `TRANSFER_MANAGER::check_offer_restrictions(oferta, decide, motivo, sem_elenco)`;
/// 0 libera, 1 ou 2 barram. O comprador é o +8, o alvo o +0xc e o vendedor o clube do contrato dele (fora o tipo 5).
/// - Com +0x30 & 0xf e status fora de 0x13: a data de conclusão (0x8cee70: push &data; push oferta, ret 8) vai ao
///   motivo; motivo diferente de 0 dá 2; hoje antes da data dá 0.
/// - Sem `sem_elenco`, alvo com jogador (+0x61) e tipo 11..15 em +0x11 (0x53b620, cdecl do byte): decide o vendedor com
///   +0x2f = 0 e o vendedor não obrigado a aceitar (0x8cd240, stdcall(oferta, 0, 0)), ou decide o comprador: o veto
///   (0x8cdb80: push motivo; push decide; push alvo; push vendedor; push comprador; push oferta, ret 0x18) dá 2.
/// - O orçamento do comprador (0x5a0590: push comprador; mov ecx, [0xae2a38], ret 4; 0x598800: push 0 ×4; push alvo;
///   mov ecx, finanças, ret 0x14, devolve i64; negativo vira 0) ao degrau seguinte (0x43f4e0: push 4; lea ecx, &dinheiro,
///   ret 4) e o valor da oferta (0x8e7a70: mov ecx, oferta -> thiscall(oferta, 0, 1, 1, 0), ret 0x10). Decide o
///   comprador: valor > orçamento × 1,1: motivo 3; comprador humano dá 1; senão o orçamento mais o +0x1b das ofertas do
///   comprador (0x8cdd80: push 0; push comprador; push pessoa, ret 0xc) pelos jogadores da lista de observação dele
///   (0x8420e0: push &lista; push comprador; mov ecx, [this+0x856], ret 8) passando do valor dá 1, senão 2. Dentro do
///   orçamento, sem vaga no elenco (0x8cd450: push motivo; push sem_elenco; push oferta; push clube, ret 0x10) dá 1.
/// - Decide o vendedor: sem vaga no elenco dele dá 1. Motivo 0. Com troca (+4) ou tipo 11..15: a regra da nação do
///   decide ([this+0x8ac], a de +0xa igual a [decide+0x53]): o virtual +0xc (push motivo; push 0xae2c90; push decide;
///   push oferta; mov ecx, regra) diferente de 0 dá 0; motivo 9 com +0x30 & 0xf = 5 dá 0; senão 2. Sem regra: 0.
unsafe extern "thiscall" fn check_offer_restrictions(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(0x8cee70usize);
    let tipo: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x8cd240usize);
    let veto: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(0x8cdb80usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let orcamento: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let apply_attribute_step: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x43f4e0usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(0x8cd450usize);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    padrao(pd);
    let comprador = cqd1_clube(cqd1_d(o + 8));
    let alvo = cqd1_pessoa(cqd1_d(o + 0xc));
    let vendedor = contract_club_filtered(alvo, true);
    if cqd1_b(o + 0x30) & 0xf != 0 && cqd1_b(o + 0x2c) != 0x13 {
        let r = conclusao(this, o, pd);
        if !motivo.is_null() {
            *motivo = r;
        }
        if r != 0 {
            return 2;
        }
        let (ha, da) = (cqd1_w(0xae2c92) as u16, cqd1_w(pd + 2) as u16);
        if ha < da || (ha == da && cqd1_w(0xae2c90) < cqd1_w(pd)) {
            return 0;
        }
    }
    if sem_elenco == 0 && cqd1_d(alvo + 0x61) != 0 && tipo(cqd1_b(o + 0x11) as i8) != 0 {
        let mut vetar = false;
        if decide == vendedor && cqd1_b(o + 0x2f) == 0 && deve_aceitar(o, 0, 0) == 0 {
            vetar = true;
        } else if decide == comprador {
            vetar = true;
        }
        if vetar && veto(this, o, comprador, vendedor, alvo, decide, motivo) != 0 {
            return 2;
        }
    }
    let mut b = orcamento(financas(cqd1_d(0xae2a38), comprador), alvo, 0, 0, 0, 0);
    if b < 0 {
        b = 0;
    }
    let mut dinheiro = b as u32;
    let v = apply_attribute_step(&mut dinheiro as *mut u32 as usize, 4) as i64;
    let t = total(o, 0, 1, 1, 0);
    if decide == comprador {
        crate::comparison::branch("cqd_1 orçamento");
        if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento
            if !motivo.is_null() {
                *motivo = 3;
            }
            if transfer_manager_is_human_club(comprador) {
                return 1;
            }
            let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let entradas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8420e0usize);
            let oferta_do_clube: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8cdd80usize);
            let mut l = [0u8; 0x10];
            let pl = l.as_mut_ptr() as usize;
            lista_nova(pl);
            entradas(cqd1_d(this + 0x856), comprador, pl);
            cqd1_pd(pl + 0xa, cqd1_d(pl + 2));
            let mut soma = v;
            loop {
                let p = transfer_manager_llist_next(pl);
                if p == 0 {
                    break;
                }
                let x = oferta_do_clube(this, cqd1_pessoa(cqd1_d(p)), comprador, 0);
                if x != 0 {
                    soma = soma.wrapping_add(cqd1_d(x + 0x1b) as i32 as i64);
                }
            }
            destruir(pl);
            return if soma > t as i64 { 1 } else { 2 };
        }
        if vaga(this, comprador, o, sem_elenco, motivo) == 0 {
            return 1;
        }
    }
    if vendedor == decide && vaga(this, vendedor, o, sem_elenco, motivo) == 0 {
        return 1;
    }
    if !motivo.is_null() {
        *motivo = 0;
    }
    if cqd1_d(o + 4) == 0 && tipo(cqd1_b(o + 0x11) as i8) == 0 {
        return 0;
    }
    for k in 0..0x1c {
        let r = cqd1_d(this + 0x8ac + k * 4);
        if cqd1_d(r + 0xa) == cqd1_d(decide + 0x53) {
            if r == 0 {
                return 0;
            }
            let regra: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32 =
                core::mem::transmute(cqd1_d(cqd1_d(r) + 0xc));
            if regra(r, o, decide, 0xae2c90, motivo) != 0 {
                return 0;
            }
            if *motivo == 9 && cqd1_b(o + 0x30) & 0xf == 5 {
                return 0;
            }
            return 2;
        }
    }
    0
}

/// Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
unsafe extern "thiscall" fn comparar_check_offer_restrictions(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(%%I2%%)(this, o, decide, motivo, sem_elenco) // ponytail: só a original
}
