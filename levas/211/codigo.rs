// ---------- 211a leva: o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador (transfer_manager.cpp) ----------

/// O nó de uma lista do jogo (0x68d0d0/0x68d1b0): contagem u16 em +0, cabeça +2, cauda +6, cursor +0xa;
/// cada nó tem o dado em +0 e o próximo em +6.
unsafe fn dados_da_lista(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = core::ptr::read_unaligned((l + 2) as *const usize);
    while no != 0 {
        v.push(*(no as *const usize));
        no = core::ptr::read_unaligned((no + 6) as *const usize);
    }
    v
}

/// 0x4cf5a0 (o contrato do jogador `pessoa`, thiscall em [0xae1050]) gravado em [0xae1080]; devolve o
/// registro do clube do contrato ([0xae23b4] + 581 × [contrato+4]) ou 0 sem contrato.
unsafe fn clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581))
}

/// { 0x8c6db0 } (stdcall, ret 4, transfer_manager.cpp): fecha (estado +0x50 = 4) as propostas de contrato
/// ligadas à oferta: a do jogador-alvo (0x4cc9e0 com o id +0xc e o clube +8) e a de cada jogador da troca
/// (lista +4, com o clube do contrato do alvo, 0x4cf5a0).
unsafe extern "stdcall" fn fechar_propostas_da_oferta(oferta: usize) {
    let id = *((oferta + 0xc) as *const usize);
    let pessoa = (*(0xae23bc as *const usize)).wrapping_add(id.wrapping_mul(110));
    let clube = clube_do_contrato(pessoa);
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let r = proposta(*(0xae1050 as *const usize), id as u32, *((oferta + 8) as *const u32));
    if r != 0 {
        *((r + 0x50) as *mut u8) = 4;
    }
    let l = *((oferta + 4) as *const usize);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let mut e = prox(l);
    while e != 0 {
        // sem contrato do alvo a original lê [0] (cai); a cópia também
        let t = proposta(*(0xae1050 as *const usize), *(e as *const u32), core::ptr::read_volatile(clube as *const u32));
        if t != 0 {
            *((t + 0x50) as *mut u8) = 4;
        }
        e = prox(l);
    }
}

/// { 0x8c16b0 } (thiscall, ret 0xc, transfer_manager.cpp): põe o jogador na fila da transferência livre
/// (lista +0x92c, registro de 9 bytes: id, motivo, compensação). Emprestado (0x4cf6d0): a original
/// (caixa de erro).
unsafe fn enfileirar_transferencia_livre_com(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32, anexar: &mut dyn FnMut(usize, &[u8; 9])) -> bool {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    if emprestado(*(0xae1050 as *const usize), pessoa) != 0 {
        return false;
    }
    let mut r = [0u8; 9];
    r[..4].copy_from_slice(&(*(pessoa as *const u32)).to_le_bytes());
    r[4] = motivo;
    r[5..].copy_from_slice(&compensacao.to_le_bytes());
    anexar(gerente + 0x92c, &r);
    true
}

unsafe extern "thiscall" fn enfileirar_transferencia_livre(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let anexar_9: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    if !enfileirar_transferencia_livre_com(gerente, pessoa, motivo, compensacao, &mut |l, r| anexar_9(l, r.as_ptr() as usize, 9)) {
        orig::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(515)(gerente, pessoa, motivo, compensacao);
    }
}

/// { 0x8c5fc0 } (thiscall, ret 8, transfer_manager.cpp): conta as negociações ([+0xc], [+0x10] delas) do
/// clube do contrato do jogador (+8) cuja oferta ([gerente] + 4 × [+0x20]) segue aberta (estados 2, 3, 4,
/// 0x10 e 0x14 não; 0xf só com +0x2e < 3) e que têm o jogador na lista da troca (+4); com `saida`, põe
/// nela o índice da oferta (0x68d0d0, 4 bytes). Jogador fora do banco ou sem contrato: 0.
unsafe extern "thiscall" fn negociacoes_de_troca_do_jogador(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    if pessoa != 0 && *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return 0;
    }
    // como `clube_do_contrato`, à parte para a mutação desta rotina não cair na 0x8c6db0
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(*(0xae1050 as *const usize), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    let p = (*(0xae23b4 as *const usize)).wrapping_add((*((c + 4) as *const usize)).wrapping_mul(581));
    if p == 0 {
        return 0;
    }
    let clube = *(p as *const u32);
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let negociacoes = *((gerente + 0xc) as *const usize);
    let mut total = 0;
    let mut i = 0;
    while i < *((gerente + 0x10) as *const i32) {
        let neg = *((negociacoes + i as usize * 4) as *const usize);
        i += 1;
        if neg == 0 || *((neg + 8) as *const u32) != clube {
            continue;
        }
        let k = *((neg + 0x20) as *const usize);
        let oferta = *((*(gerente as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if oferta == 0 {
            continue;
        }
        match *((oferta + 0x2c) as *const u8) {
            2 | 3 | 4 | 0x10 | 0x14 => continue,
            0xf if *((oferta + 0x2e) as *const u8) >= 3 => continue,
            _ => {}
        }
        let l = *((neg + 4) as *const usize);
        if l == 0 {
            continue;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
        let mut e = prox(l);
        while e != 0 && *(e as *const u32) != *(pessoa as *const u32) {
            e = prox(l);
        }
        if e == 0 {
            continue;
        }
        if saida != 0 {
            anexar(saida, neg + 0x20, 4);
        }
        total += 1;
    }
    total
}

unsafe extern "stdcall" fn comparar_fechar_propostas_da_oferta(oferta: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(514);
    // as propostas que podem fechar: as do id do alvo e dos ids da troca (o clube não filtra aqui)
    let mut ids = vec![*((oferta + 0xc) as *const u32)];
    let l = *((oferta + 4) as *const usize);
    if l != 0 {
        ids.extend(dados_da_lista(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(0xae1050 as *const usize) + 0x34) as *const usize);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if l != 0 {
        regioes.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regioes.push((propostas + 0xa, 4));
        for r in dados_da_lista(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regioes.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(oferta), || fechar_propostas_da_oferta(oferta));
    PLACAR.registrar(igual, || format!("fechar_propostas_da_oferta(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

unsafe extern "thiscall" fn comparar_enfileirar_transferencia_livre(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(515);
    // ponytail: a fila aloca um nó (não roda duas vezes): a cópia só monta o registro (0x4cf6d0 é
    // consulta pura), a original põe na fila e confere-se o nó novo da cauda
    let mut registro = None;
    if !enfileirar_transferencia_livre_com(gerente, pessoa, motivo, compensacao, &mut |_, r| registro = Some(*r)) {
        return o(gerente, pessoa, motivo, compensacao);
    }
    let l = gerente + 0x92c;
    let antes = core::ptr::read_unaligned(l as *const u16);
    o(gerente, pessoa, motivo, compensacao);
    let depois = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let novo = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    PLACAR.registrar(depois == antes.wrapping_add(1) && novo == registro,
        || format!("enfileirar_transferencia_livre {} -> {} original={:x?} rust={:x?}", antes, depois, novo, registro));
}

unsafe extern "thiscall" fn comparar_negociacoes_de_troca_do_jogador(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(516);
    if saida != 0 {
        // ponytail: com lista de saída (só a tela 0x47ec79) a original anexa (aloca); só a original
        return o(gerente, pessoa, saida);
    }
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if pessoa != 0 && *(pessoa as *const i32) < *(0xae2364 as *const i32) {
        let guardado = *(0xae1080 as *const usize);
        let p = clube_do_contrato(pessoa);
        *(0xae1080 as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das negociações do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            let negociacoes = *((gerente + 0xc) as *const usize);
            for i in 0..(*((gerente + 0x10) as *const i32)).max(0) as usize {
                let neg = *((negociacoes + i * 4) as *const usize);
                if neg != 0 && *((neg + 8) as *const u32) == clube && *((neg + 4) as *const usize) != 0 {
                    regioes.push((*((neg + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, pessoa, saida),
        || negociacoes_de_troca_do_jogador(gerente, pessoa, saida));
    PLACAR.registrar(igual && r == n, || format!("negociacoes_de_troca_do_jogador original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 211a leva: o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados (national_teams.cpp, fix_man.cpp, shortlist_manager.cpp) ----------

/// { 0x775ba0 } (thiscall, ret 4, national_teams.cpp): tira o `jogador` da seleção. Com nação (+0x1a)
/// e ficha (+0x61): no registro de 110 bytes da nação em [this+4], o nível (0x76d810 com o registro e
/// 0); se a nota do nível (palavra +0xf + nível x 4) for menor que a da ficha (+9), procura o jogador
/// na faixa do nível da lista [this+0xc] (início = dword +8 + bytes +0xe dos níveis anteriores,
/// tamanho no byte +0xd + nível x 4): achou, o último da faixa vai para a vaga dele e o tamanho
/// diminui. Depois, convocado (+0x24): a vaga dele entre as 50 (+0xd7) zera, +0x24 zera e o byte +0x5e
/// fica só com os bits 0..1.
unsafe extern "thiscall" fn tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return;
    }
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let campo = |o: isize| (reg as isize).wrapping_add(n * 4 + o) as usize;
    if core::ptr::read_unaligned(campo(0xf) as *const i16) < core::ptr::read_unaligned((u(jogador + 0x61) + 9) as *const i16) {
        let mut inicio = u(reg + 8) as i32;
        for k in 0..n.max(0) as usize {
            inicio = inicio.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32);
        }
        let tamanho = campo(0xd) as *mut u8;
        let fim = inicio.wrapping_add(*tamanho as i32);
        let lista = |i: i32| (u(this + 0xc) as isize).wrapping_add(i as isize * 4) as *mut usize;
        let mut i = inicio;
        while i < fim {
            if core::ptr::read_unaligned(lista(i)) == jogador {
                *tamanho = (*tamanho).wrapping_sub(1);
                core::ptr::write_unaligned(lista(i), core::ptr::read_unaligned(lista(fim - 1)));
                break;
            }
            i += 1;
        }
    }
    let selecao = u(jogador + 0x24);
    if selecao == 0 {
        return;
    }
    if let Some(k) = (0..50).find(|&k| u(selecao + 0xd7 + 4 * k) == jogador) {
        core::ptr::write_unaligned((selecao + 0xd7 + 4 * k) as *mut usize, 0);
    }
    core::ptr::write_unaligned((jogador + 0x24) as *mut usize, 0);
    *((jogador + 0x5e) as *mut u8) &= 3;
}

/// { 0x5af2f0 } (thiscall, ret 8, fix_man.cpp; `this` não é lido): a faixa de horário do jogo da
/// competição `comp` no dia `dia` (palavra): Holanda, Irlanda, Bélgica e França (id da nação +0x5d
/// em [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) só têm 1 no dia 6; as outras, 1 nos dias 5 e 6;
/// senão 2. Devolve só `al`.
unsafe extern "thiscall" fn faixa_do_horario(_this: usize, comp: usize, dia: u32) -> u8 {
    let dia = dia as i16;
    if comp != 0 {
        let nacao = core::ptr::read_unaligned((comp + 0x5d) as *const usize);
        if nacao != 0 {
            let id = *(nacao as *const u32);
            if [0x9cf338usize, 0x9cf2dc, 0x9cf244, 0x9cf300].iter().any(|&g| id == *(g as *const u32)) {
                return if dia != 6 { 2 } else { 1 };
            }
        }
    }
    if dia == 6 || dia == 5 { 1 } else { 2 }
}

/// { 0x84b230 } (stdcall, ret 0xc, shortlist_manager.cpp; `posicoes` não é lido): o teto de preço
/// para a vaga de tipo `tipo` na lista de observados, pela reputação do clube (palavra +0x80): tipo 4
/// = x550 em Itália, Inglaterra e Espanha ([0x9cf360], [0x9cf2e4], [0x9cf488], nação +0x53), x475 em
/// Alemanha e França ([0x9cf314], [0x9cf300]), x330 nas outras; tipo 5 = x550; tipo 6 = x110; os
/// outros, -1.
unsafe extern "stdcall" fn teto_da_observacao(clube: usize, _posicoes: u32, tipo: u32) -> i32 {
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    match tipo as i8 {
        4 => {
            let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
            let eh = |gs: &[usize]| nacao != 0 && gs.iter().any(|&g| *(nacao as *const u32) == *(g as *const u32));
            if eh(&[0x9cf360, 0x9cf2e4, 0x9cf488]) {
                rep.wrapping_mul(550)
            } else if eh(&[0x9cf314, 0x9cf300]) {
                rep.wrapping_mul(475)
            } else {
                rep.wrapping_mul(330)
            }
        }
        5 => rep.wrapping_mul(550),
        6 => rep.wrapping_mul(110),
        _ => -1,
    }
}

unsafe extern "thiscall" fn comparar_tirar_jogador_da_selecao(this: usize, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(517);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return o(this, jogador);
    }
    // as regiões: o jogador, o registro da nação, a faixa do nível na lista e a seleção (o nível,
    // 0x76d810, só lê)
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let mut regioes = [(jogador, 110), (reg, 110), (jogador, 0), (jogador, 0)];
    if (0..=3).contains(&n) {
        let inicio = (0..n as usize).fold(u(reg + 8) as i32, |s, k| s.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32));
        let tamanho = *((reg + 0xd + 4 * n as usize) as *const u8) as usize;
        if inicio >= 0 && tamanho > 0 {
            regioes[2] = (u(this + 0xc) + 4 * inicio as usize, 4 * tamanho);
        }
    }
    if u(jogador + 0x24) != 0 {
        regioes[3] = (u(jogador + 0x24) + 0xd7, 200);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogador), || tirar_jogador_da_selecao(this, jogador));
    PLACAR.registrar(igual, || format!("tirar_jogador_da_selecao(nivel {}) {:x?} x {:x?}", n, mo, mr));
}

unsafe extern "thiscall" fn comparar_faixa_do_horario(this: usize, comp: usize, dia: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(518)(this, comp, dia);
    let n = faixa_do_horario(this, comp, dia);
    PLACAR.registrar(r == n, || format!("faixa_do_horario(dia {}) original={} rust={}", dia as i16, r, n));
    r
}

unsafe extern "stdcall" fn comparar_teto_da_observacao(clube: usize, posicoes: u32, tipo: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(519)(clube, posicoes, tipo);
    let n = teto_da_observacao(clube, posicoes, tipo);
    PLACAR.registrar(r == n, || format!("teto_da_observacao(tipo {}) original={} rust={}", tipo as i8, r, n));
    r
}

// ---------- 211a leva: as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia (transfer_manager.cpp, shortlist_manager.cpp, comp_util.cpp) ----------

/// A oferta do gerente de transferências para a pessoa `id`: a primeira das [this+4] em [this] com
/// +0xc = `id`, ou 0.
unsafe fn oferta_pelo_jogador(this: usize, id: i32) -> usize {
    let ofertas = *(this as *const usize);
    let n = *((this + 4) as *const i32);
    (0..n.max(0) as usize)
        .map(|i| *((ofertas + 4 * i) as *const usize))
        .find(|&e| e != 0 && *((e + 0xc) as *const i32) == id)
        .unwrap_or(0)
}

/// { 0x8c5cf0 } (thiscall, ret 0xc, transfer_manager.cpp): conta as ofertas pela pessoa e, com
/// `lista`, acrescenta nela (0x68d0d0, 4 bytes) o índice de cada uma. Id a partir de 0xae2364 ou sem
/// oferta: 0. A oferta achada sem lista própria (+0) conta 1 (acrescenta +0x20); com lista, percorre
/// os índices dela (cursor +0xa reiniciado de +2) e conta os que apontam para ofertas da mesma pessoa.
/// Sem `todas`, ficam de fora as encerradas (0x8d0d90, aqui embutida como na original).
unsafe extern "thiscall" fn ofertas_pelo_jogador(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    // a original lê o id antes de testar o ponteiro
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let e = oferta_pelo_jogador(this, id);
    if e == 0 {
        return 0;
    }
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68d1b0usize);
    let l = *(e as *const usize);
    if l == 0 {
        if todas == 0 && oferta_fechada(e) != 0 {
            return 0;
        }
        if lista != 0 {
            acrescentar(lista, e + 0x20, 4);
        }
        return 1;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    let mut conta = 0u32;
    loop {
        let no = proximo(*(e as *const usize)) as usize;
        if no == 0 {
            break;
        }
        let k = *(no as *const usize);
        let o = *((*(this as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if o == 0 || *((o + 0xc) as *const i32) != id {
            continue;
        }
        if todas == 0 && oferta_fechada(o) != 0 {
            continue;
        }
        if lista != 0 {
            acrescentar(lista, no, 4);
        }
        conta += 1;
    }
    conta
}

/// O registro de 9 bytes do clube (id em +0) na lista de compras [this+0xdc].
unsafe fn registro_da_lista_de_compras(this: usize, clube: usize) -> usize {
    (*(clube as *const i32) as usize).wrapping_mul(9).wrapping_add(*((this + 0xdc) as *const usize))
}

/// { 0x83bf40 } (thiscall, ret 4, shortlist_manager.cpp): a reputação que o clube procura nas
/// contratações, gravada na palavra +5 do registro dele: +0x73 / 10 + 4500; se não for de humano
/// (0x600a50 com 1, 1), o técnico (+0xcf) ajusta por (+0x57 − ficha +0x1d) × 0,0125; o presidente
/// (+0xbf, com ficha) por (+0x57 − ficha +0x1d) / 60 e por (ficha +0x20 − 10) × 0,025 (cada um
/// multiplica 1 + fator, truncando). Nunca abaixo de +0x80 nem acima de 10000. Devolve o valor.
unsafe extern "thiscall" fn reputacao_alvo_do_clube(this: usize, clube: usize) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let r = registro_da_lista_de_compras(this, clube);
    let alvo = (r + 5) as *mut i16;
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let w = || core::ptr::read_unaligned(alvo) as i32;
    let por = |dif: i32, k: usize, v: i32| ftol((dif as f64 * f64_em(k) + f64_em(0x967890)) * v as f64);
    core::ptr::write_unaligned(alvo, (core::ptr::read_unaligned((clube + 0x73) as *const i32) / 10 + 0x1194) as i16);
    if humano(clube, 1, 1, 0, 0) == 0 {
        let tecnico = u(clube + 0xcf);
        if tecnico != 0 {
            let dif = byte(tecnico, 0x57) as i32 - byte(u(tecnico + 0x69), 0x1d) as i32;
            core::ptr::write_unaligned(alvo, por(dif, 0x96a740, w()) as i16);
        }
    }
    let p = u(clube + 0xbf);
    if p != 0 && u(p + 0x69) != 0 {
        let dif = byte(p, 0x57) as i32 - byte(u(p + 0x69), 0x1d) as i32;
        let v = por(dif, 0x968958, w()) as i16;
        core::ptr::write_unaligned(alvo, v);
        let ficha = u(u(clube + 0xbf) + 0x69);
        core::ptr::write_unaligned(alvo, por(byte(ficha, 0x20) as i32 - 10, 0x967878, v as i32) as i16);
    }
    let piso = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    let v = core::ptr::read_unaligned(alvo);
    let v = if piso > v { piso } else if v > 0x2710 { 0x2710 } else { v };
    core::ptr::write_unaligned(alvo, v);
    v as i32
}

/// { 0x4afeb0 } (cdecl, comp_util.cpp; a convenção do mapa dá o 2º argumento como sem uso, mas é a
/// notícia): grava o jogo em 18 dados seguidos da notícia (0x785790), a partir do dado [cursor], e
/// avança o cursor 18. Ordem: +0xc, +0x10, +0x3d, +0x3e, +4, +0, +0x42, +0x32, +0x30, +0x43, +0x44,
/// +0x4b, +0x4c, +0x47, +0x48, +0x2a, +0x28, +8 (bytes e palavras +0x30/+0x32 com sinal; +0x28 e
/// +0x2a sem). Devolve o que a última gravação devolveu. A original guarda o índice do 18º dado no
/// byte baixo do próprio argumento `cursor` (só a parte baixa chega à 0x785790).
unsafe extern "C" fn gravar_jogo_na_noticia(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let d = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u32);
    let b = |o: usize| byte(jogo, o) as i32 as u32;
    let s = |o: usize| core::ptr::read_unaligned((jogo + o) as *const i16) as i32 as u32;
    let z = |o: usize| core::ptr::read_unaligned((jogo + o) as *const u16) as u32;
    let valores = [
        d(0xc), d(0x10), b(0x3d), b(0x3e), d(4), d(0), b(0x42), s(0x32), s(0x30),
        b(0x43), b(0x44), b(0x4b), b(0x4c), b(0x47), b(0x48), z(0x2a), z(0x28), d(8),
    ];
    let mut k = *cursor;
    let mut r = 0;
    for v in valores {
        r = gravar(noticia, k as i8, v);
        k = k.wrapping_add(1);
    }
    *cursor = k;
    r
}

unsafe extern "thiscall" fn comparar_ofertas_pelo_jogador(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(520);
    // ponytail: com `lista` a original aloca nós (0x68d0d0) e não pode rodar duas vezes; só ela
    if lista != 0 || pessoa.is_null() {
        return o(this, pessoa, lista, todas);
    }
    // a original só mexe no cursor da lista da oferta achada
    let e = oferta_pelo_jogador(this, *pessoa);
    let l = if e == 0 { 0 } else { *(e as *const usize) };
    let reg = if l == 0 { vec![] } else { vec![(l, 0xe)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, lista, todas), || ofertas_pelo_jogador(this, pessoa, lista, todas));
    PLACAR.registrar(igual && r == n, || format!("ofertas_pelo_jogador({}, todas {}) original={} rust={} {:x?} x {:x?}", *pessoa, todas, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_reputacao_alvo_do_clube(this: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(521);
    let reg = [(registro_da_lista_de_compras(this, clube) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || reputacao_alvo_do_clube(this, clube));
    PLACAR.registrar(igual && r == n, || format!("reputacao_alvo_do_clube({}) original={} rust={} {:x?} x {:x?}", *(clube as *const i32), r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_gravar_jogo_na_noticia(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut u8) -> u32>(522);
    let k = *cursor as i8;
    // índice fora de 0..49 em algum dos 18: a 0x785790 abre a caixa de erro; só a original
    if !(0..0x32 - 17).contains(&k) {
        return o(jogo, noticia, cursor);
    }
    let reg = [(noticia + 5, 200), (cursor as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(jogo, noticia, cursor), || gravar_jogo_na_noticia(jogo, noticia, cursor));
    PLACAR.registrar(igual && r == n, || format!("gravar_jogo_na_noticia(cursor {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}
