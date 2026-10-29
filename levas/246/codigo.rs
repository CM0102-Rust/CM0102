// ---------- 246a leva: as listas de pendências: soltar as suspensões guardadas, tirar as ofertas pendentes da pessoa e marcar a decisão depois da transferência acertada ----------

unsafe fn cpj2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpj2_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

unsafe fn cpj2_tirar(l: usize, e: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    f(l, e);
}

/// O contador de suspensões guardadas (+0x1c) do registro de 29 bytes do id `id` em [this+0x14].
unsafe fn cpj2_guardadas(this: usize, id: usize) -> *mut i8 {
    (cpj2_d(this + 0x14).wrapping_add(id.wrapping_mul(29)) + 0x1c) as *mut i8
}

/// { 0x5567d0 } (thiscall, ret 4, discipline.cpp): `Discipline::drop_held_bans(pessoa)`: da lista das suspensões
/// guardadas (+0x1c; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item do id `*pessoa` enquanto o contador
/// do registro dele (+0x1c, com sinal) for positivo, descontando um a cada item.
unsafe extern "thiscall" fn soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 && *n > 0 {
        if cpj2_d(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            *n = (*n).wrapping_sub(1); // uma a menos
        }
        e = cpj2_proximo(l);
    }
}

/// O byte de ofertas pendentes (+8) do registro de 79 bytes da pessoa em [0xae2d0c].
unsafe fn cpj2_pendentes(id: usize) -> *mut u8 {
    (cpj2_d(0xae2d0c).wrapping_add(id.wrapping_mul(79)) + 8) as *mut u8
}

/// { 0x4ccbf0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_pending_offers_for_staff(pessoa)`:
/// com a lista das ofertas (+0x34) e ofertas pendentes no registro da pessoa (+8 do de 79 bytes em [0xae2d0c]): tira
/// a marca de contrato acertado (0x8c5430 sobre [0xde1f54]) e, do começo da lista, cada oferta da pessoa (0x68d1f0),
/// descontando o byte, até ele zerar.
unsafe extern "thiscall" fn tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 || *cpj2_pendentes(*pessoa as usize) == 0 {
        return;
    }
    let desmarcar: unsafe extern "thiscall" fn(usize, *const u32) -> u32 = core::mem::transmute(0x8c5430usize);
    desmarcar(cpj2_d(0xde1f54), pessoa);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if cpj2_d(e) as u32 == *pessoa {
            cpj2_tirar(l, e);
            let p = cpj2_pendentes(*pessoa as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // a última pendente
            }
        }
        e = cpj2_proximo(l);
    }
}

/// A primeira oferta da lista `l` (sem mexer no cursor) da pessoa (+0) e do clube (+4) da oferta de transferência e
/// ainda sem decisão (+0x62 = 0).
unsafe fn cpj2_oferta_a_marcar(l: usize, oferta: usize) -> Option<usize> {
    cpd1_itens(l).into_iter().find(|&e| cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0)
}

/// Habilidade (palavra +0xb do registro de jogador +0x61 da pessoa +0xc) / 500 acima da reputação (+0x80) do clube
/// (+8) da oferta?
unsafe fn cpj2_acima_do_clube(oferta: usize) -> u32 {
    let pessoa = cpj2_d(0xae23bc).wrapping_add(cpj2_d(oferta + 0xc).wrapping_mul(0x6e));
    let hab = *((cpj2_d(pessoa + 0x61) + 0xb) as *const i16) as i32 / 500;
    let clube = cpj2_d(0xae23b4).wrapping_add(cpj2_d(oferta + 8).wrapping_mul(0x245));
    (hab > *((clube + 0x80) as *const i16) as i32) as u32
}

/// { 0x4cc930 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::schedule_decision_for_transfer(oferta)`:
/// a primeira oferta de contrato da lista (+0x34) da pessoa (+0xc) e do clube (+8) da oferta de transferência ainda
/// sem decisão (+0x62) fica marcada (1) e a decisão da pessoa é agendada (0x4cd230 com a vaga +0x20). Devolve se a
/// habilidade da pessoa / 500 passa a reputação do clube.
unsafe extern "thiscall" fn marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let l = cpj2_d(this + 0x34);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj2_d(l + 2));
    let mut e = cpj2_proximo(l);
    while e != 0 {
        if cpj2_d(e) == cpj2_d(oferta + 0xc) && cpj2_d(e + 4) == cpj2_d(oferta + 8) && *((e + 0x62) as *const u8) == 0 {
            *((e + 0x62) as *mut u8) = 1; // a oferta vale
            let agendar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4cd230usize);
            agendar(this, cpj2_d(oferta + 0xc) as u32, cpj2_d(oferta + 0x20) as u32);
            break;
        }
        e = cpj2_proximo(l);
    }
    cpj2_acima_do_clube(oferta)
}

/// Comparador: o plano da cópia (os itens do id que saem até o contador zerar) antes; a original tira; confere os
/// itens que ficaram e o contador.
unsafe extern "thiscall" fn comparar_soltar_suspensoes_guardadas(this: usize, pessoa: *const u32) {
    let l = this + 0x1c;
    let n = cpj2_guardadas(this, *pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *n;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta > 0 && cpj2_d(e) as u32 == *pessoa {
            resta -= 1;
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(867)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *n == resta, || format!("soltar_suspensoes_guardadas({}) itens {} -> {} (esperava {}), contador {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *n, resta));
}

/// Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
unsafe extern "thiscall" fn comparar_tirar_ofertas_pendentes(this: usize, pessoa: *const u32) {
    let l = cpj2_d(this + 0x34);
    if l == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, *const u32)>(868)(this, pessoa);
    }
    let p = cpj2_pendentes(*pessoa as usize);
    let antes = cpd1_itens(l);
    let mut resta = *p;
    let mut esperado = Vec::new();
    for &e in &antes {
        if resta != 0 && cpj2_d(e) as u32 == *pessoa {
            resta = resta.wrapping_sub(1);
        } else {
            esperado.push(e);
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, *const u32)>(868)(this, pessoa);
    let depois = cpd1_itens(l);
    PLACAR.registrar(depois == esperado && *p == resta, || format!("tirar_ofertas_pendentes({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *pessoa, antes.len(), depois.len(), esperado.len(), *p, resta));
}

/// Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
unsafe extern "thiscall" fn comparar_marcar_decisao_da_transferencia(this: usize, oferta: usize) -> u32 {
    let alvo = cpj2_oferta_a_marcar(cpj2_d(this + 0x34), oferta);
    let n = cpj2_acima_do_clube(oferta);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(869)(this, oferta);
    let marcada = alvo.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    PLACAR.registrar(marcada && r == n, || format!("marcar_decisao_da_transferencia({:x}) alvo {:x?} marcada={} original={} rust={}", oferta, alvo, marcada, r, n));
    r
}

// ---------- 246a leva: as notícias para os técnicos humanos: para todos, para os da divisão e para os da nação (clube ou seleção) ----------

unsafe fn cpj3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O técnico humano `i` (0 a 15: os últimos 16 de [0xae23bc], 0x6e bytes), se ativo (+0x5f = 1).
unsafe fn cpj3_humano_ativo(i: usize) -> Option<usize> {
    let p = cpj3_d(0xae23bc).wrapping_add(cpj3_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
    (*((p + 0x5f) as *const u8) == 1).then_some(p)
}

/// Entrega a notícia a cada técnico humano ativo que `passa` (0x785d40 com 0), com o alcance `alcance` em +0xde.
/// Devolve se todas as entregas foram aceitas (a primeira recusada encerra).
unsafe fn cpj3_entregar(this: usize, noticia: usize, alcance: u8, passa: impl Fn(usize) -> bool) -> bool {
    *((noticia + 0xde) as *mut u8) = alcance;
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x785d40usize);
    for i in 0..0x10 {
        if let Some(p) = cpj3_humano_ativo(i) {
            if passa(p) && entregar(this, noticia, p, 0) == 0 {
                return false; // a entrega recusada encerra
            }
        }
    }
    true
}

/// { 0x786780 } (thiscall, ret 4, news.cpp): `News::send_to_all_managers(noticia)`: alcance de todos (+0xde = 4) e a
/// entrega (0x785d40) a cada técnico humano ativo. Devolve 1, ou 0 na primeira recusada. Notícia nula: a original
/// (caixa "Error").
unsafe extern "thiscall" fn noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    if noticia == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(870)(this, noticia); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 4, |_| true) as u32
}

/// { 0x7862d0 } (thiscall, ret 8, news.cpp): `News::send_to_division_managers(noticia, divisao)`: alcance da divisão
/// (+0xde = 1) e a entrega a cada técnico humano ativo com clube (+0x39) na `divisao` (+0x57). Devolve 1, ou 0 na
/// primeira recusada. Notícia ou divisão nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    if noticia == 0 || divisao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(871)(this, noticia, divisao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 1, |p| {
        let c = cpj3_d(p + 0x39);
        c != 0 && cpj3_d(c + 0x57) == divisao // a divisão do clube
    }) as u32
}

/// { 0x7863f0 } (thiscall, ret 8, news.cpp): `News::send_to_nation_managers(noticia, nacao)`: alcance da nação (+0xde
/// = 2) e a entrega a cada técnico humano ativo cujo clube (+0x39) ou seleção (+0x24) é da `nacao` (+0x53). Devolve 0.
/// Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(872)(this, noticia, nacao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 2, |p| {
        let (c, s) = (cpj3_d(p + 0x39), cpj3_d(p + 0x24));
        (c != 0 && cpj3_d(c + 0x53) == nacao) || (s != 0 && cpj3_d(s + 0x53) == nacao) // o clube ou a seleção
    });
    0
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_todos_os_tecnicos");
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(870)(this, noticia)
}

unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_divisao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(871)(this, noticia, divisao)
}

unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_pais");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(872)(this, noticia, nacao)
}

// ---------- 246a leva: a oferta que bate o recorde de valor pago, a condição do jogador e o limite de escalação da partida ----------

unsafe fn cpj4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x4486c0 } (thiscall, ret 8, club_records.cpp): `Club_Records::offer_breaks_fee_record(oferta, exigir)`: o valor
/// da oferta (+0x1b) bate o recorde de valor pago do clube comprador (+8; os recordes de sempre pela 0x452710, criando
/// se preciso)? Tem de ser ao menos 50 × reputação (+0x80) + 500.000; com `exigir`, também ao menos reputação² × a
/// constante 0x967880 (_ftol); e acima de zero e do recorde (+0x313). Oferta nula: 0.
unsafe extern "thiscall" fn oferta_bate_o_recorde(this: usize, oferta: usize, exigir: u32) -> u8 {
    if oferta == 0 {
        return 0;
    }
    let clube = cpj4_d(0xae23b4).wrapping_add(cpj4_d(oferta + 8).wrapping_mul(0x245));
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let rec = recordes(this, clube, 1);
    if rec == 0 || clube == 0 {
        return 0;
    }
    let rep = *((clube + 0x80) as *const i16) as i32;
    let valor = cpj4_d(oferta + 0x1b) as i32;
    if valor < rep.wrapping_mul(25).wrapping_add(250_000).wrapping_mul(2) {
        return 0; // abaixo do mínimo pela reputação
    }
    if exigir != 0 && valor < ftol(rep as f64 * rep as f64 * f64_em(0x967880)) {
        return 0;
    }
    (valor != 0 && valor > cpj4_d(rec + 0x313) as i32) as u8
}

/// { 0x62c590 } (thiscall, ret 8, injury.cpp): `Injury::get_condition(jogador, em_porcento)`: a condição do jogador:
/// forma (+0xa) × condição de jogo (+8) / 10.000 do registro de 31 bytes [this] do id, no mínimo 100 e no máximo 9000
/// enquanto ele se recupera (0x62c400 = 1); em porcentagem (× 0x96bac8 + 0x968968, _ftol), no máximo 100. Pessoa fora
/// dos jogadores ([0xae2364]): 100 ou 10.000. Jogador nulo: 0.
unsafe extern "thiscall" fn condicao_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    if jogador == 0 {
        return 0;
    }
    let id = cpj4_d(jogador) as i32;
    if id >= cpj4_d(0xae2364) as i32 {
        return if em_porcento != 0 { 100 } else { 10_000 };
    }
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let recuperando = estado(this, jogador) == 1;
    let r = cpj4_d(this).wrapping_add((id as usize).wrapping_mul(31));
    let v = *((r + 0xa) as *const i16) as i32 * *((r + 8) as *const i16) as i32 / 10_000;
    let mut c = if recuperando && v as i16 >= 9000 {
        9000 // o teto de quem se recupera
    } else if v as i16 > 100 {
        v
    } else {
        100
    };
    if em_porcento != 0 {
        c = ftol(c as i16 as f64 * f32_em(0x96bac8) as f64 + f64_em(0x968968));
        if c as i16 > 100 {
            c = 100;
        }
    }
    c as u16
}

/// { 0x8cf7e0 } (thiscall, ret 8, transfer_manager.cpp): `get_selection_limit(partida, saida)`: zera os 6 bytes da
/// saída, pede a tabela de 42 bytes das restrições da partida (0x8cf6b0 com 1; o `this` passa direto) e fica com a
/// primeira que existe (≠ 0xff): +5 ou +3 (tipo 1), +2 ou +0 (tipo 2), +0x16 com a quantidade em +0x17 (tipo 3, a
/// idade em +1). O tipo vai em +0 e a quantidade em +5.
unsafe extern "thiscall" fn limite_de_escalacao(this: usize, partida: usize, saida: *mut u8) {
    core::ptr::write_unaligned(saida as *mut u32, 0);
    core::ptr::write_unaligned(saida.add(4) as *mut u16, 0);
    let mut t = [0u8; 0x2c];
    let restricoes: unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32 = core::mem::transmute(0x8cf6b0usize);
    restricoes(this, partida, t.as_mut_ptr(), 1);
    let (tipo, n) = if t[5] != 0xff {
        (1, t[5])
    } else if t[3] != 0xff {
        (1, t[3])
    } else if t[2] != 0xff {
        (2, t[2]) // o 3º limite
    } else if t[0] != 0xff {
        (2, t[0])
    } else if t[0x16] != 0xff {
        *saida.add(1) = t[0x16];
        (3, t[0x17])
    } else {
        return;
    };
    *saida = tipo;
    *saida.add(5) = n;
}

/// Comparador: a 0x452710 cria os recordes do clube na primeira vez (a original primeiro; a cópia acha os mesmos):
/// pelo valor.
unsafe extern "thiscall" fn comparar_oferta_bate_o_recorde(this: usize, oferta: usize, exigir: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(873)(this, oferta, exigir);
    let n = oferta_bate_o_recorde(this, oferta, exigir);
    PLACAR.registrar(r == n, || format!("oferta_bate_o_recorde({:x}, {}) original={} rust={}", oferta, exigir, r, n));
    r
}

/// Comparador: pelo valor (a 0x62c400 só lê).
unsafe extern "thiscall" fn comparar_condicao_do_jogador(this: usize, jogador: usize, em_porcento: u32) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(874)(this, jogador, em_porcento);
    let n = condicao_do_jogador(this, jogador, em_porcento);
    PLACAR.registrar(r == n, || format!("condicao_do_jogador({:x}, {}) original={} rust={}", jogador, em_porcento, r, n));
    r
}

/// Comparador: lado a lado pelos 6 bytes da saída (a 0x8cf6b0 só escreve na tabela local).
unsafe extern "thiscall" fn comparar_limite_de_escalacao(this: usize, partida: usize, saida: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8)>(875);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 6)], || o(this, partida, saida), || limite_de_escalacao(this, partida, saida));
    PLACAR.registrar(igual, || format!("limite_de_escalacao({:x}) {:x?} x {:x?}", partida, mo, mr));
}

// ---------- 246a leva: limpar as partidas assistidas que os humanos não podem ver, os alvos dos olheiros do clube e trocar o olheiro da busca ----------

unsafe fn cpj5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A partida assistida do técnico humano `i` (79 bytes em [this] + 0x9ad4, 0xa6ef bytes por humano).
unsafe fn cpj5_assistida(this: usize, i: usize) -> usize {
    cpj5_d(this).wrapping_add(0x9ad4).wrapping_add(i.wrapping_mul(0xa6ef))
}

/// { 0x5fe8d0 } (thiscall, human_manager.cpp): `HUMAN_MANAGER::clear_invalid_watched_fixtures()`: para cada um dos 16
/// técnicos humanos (os últimos 16 de [0xae23bc]) ativo (+0x5f ≠ 0) que não pode mais ver a partida assistida dele
/// (0x5fe700), zera os 79 bytes dela. Devolve 1.
unsafe extern "thiscall" fn limpar_partidas_assistidas(this: usize) -> u32 {
    let pode_ver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5fe700usize);
    for i in 0..0x10usize {
        let p = cpj5_d(0xae23bc).wrapping_add(cpj5_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) == 0 {
            continue;
        }
        let f = cpj5_assistida(this, i);
        if pode_ver(this, f, p) == 0 {
            core::ptr::write_bytes(f as *mut u8, 0, 79); // a partida sai
        }
    }
    1
}

/// Os pedidos de observação (0x11 bytes em [this+0x5c], [this+0x64] deles) do clube `id` (+0): as pessoas (+4).
unsafe fn cpj5_alvos(this: usize, id: u32) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = 0i32;
    while k < cpj5_d(this + 0x64) as i32 {
        let e = cpj5_d(this + 0x5c).wrapping_add(k as usize * 0x11);
        if cpj5_d(e) as u32 == id { // o pedido do clube
            v.push(cpj5_d(0xae23bc).wrapping_add(cpj5_d(e + 4).wrapping_mul(0x6e)));
        }
        k += 1;
    }
    v
}

/// { 0x7f9690 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::collect_club_scout_targets(clube, lista)`: com
/// `lista`, esvazia-a (0x68d080) e acrescenta (0x68d0d0, 0x6e bytes) a pessoa de cada pedido de observação do clube
/// `*clube`. Devolve se há algum. Clube nulo: 0 (a lista fica vazia).
unsafe extern "thiscall" fn alvos_dos_olheiros_do_clube(this: usize, clube: *const u32, lista: usize) -> u32 {
    if lista != 0 {
        let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
        esvaziar(lista);
    }
    if clube.is_null() {
        return 0;
    }
    let alvos = cpj5_alvos(this, *clube);
    if lista != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
        for &p in &alvos {
            acrescentar(lista, p, 0x6e);
        }
    }
    (!alvos.is_empty()) as u32
}

/// { 0x7ae8a0 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::set_scout(olheiro)`: se o olheiro muda (o atual
/// pela 0x7ae950), grava os ids dele em +0x174 (0x7e4920) ou, sem olheiro, o índice +0x180 = −1; e as missões de nação
/// (+0x18c), região (+0x194) e competição (+0x195) voltam a −1. Devolve 1.
unsafe extern "thiscall" fn trocar_olheiro_da_busca(this: usize, olheiro: usize) -> u32 {
    if olheiro == olheiro_da_busca(this) {
        return 1; // o mesmo olheiro
    }
    if olheiro != 0 {
        let gravar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
        gravar(olheiro, this + 0x174);
    } else {
        core::ptr::write_unaligned((this + 0x180) as *mut u32, u32::MAX);
    }
    core::ptr::write_unaligned((this + 0x18c) as *mut u32, u32::MAX);
    *((this + 0x194) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x195) as *mut u32, u32::MAX);
    1
}

/// Comparador: lado a lado pelas 16 partidas assistidas (a 0x5fe700 só lê).
unsafe extern "thiscall" fn comparar_limpar_partidas_assistidas(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(876);
    let reg: Vec<(usize, usize)> = (0..0x10).map(|i| (cpj5_assistida(this, i), 79)).collect();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || limpar_partidas_assistidas(this));
    PLACAR.registrar(igual && r == n, || format!("limpar_partidas_assistidas {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: os alvos da cópia antes; a original enche a lista; confere o valor e a lista (cada item, os 0x6e bytes
/// da pessoa).
unsafe extern "thiscall" fn comparar_alvos_dos_olheiros_do_clube(this: usize, clube: *const u32, lista: usize) -> u32 {
    let alvos = if clube.is_null() { Vec::new() } else { cpj5_alvos(this, *clube) };
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u32, usize) -> u32>(877)(this, clube, lista);
    let mut igual = r == (!alvos.is_empty()) as u32;
    if lista != 0 {
        let itens = cpd1_itens(lista);
        igual &= itens.len() == alvos.len()
            && itens.iter().zip(&alvos).all(|(&i, &p)| core::slice::from_raw_parts(i as *const u8, 0x6e) == core::slice::from_raw_parts(p as *const u8, 0x6e));
    }
    PLACAR.registrar(igual, || format!("alvos_dos_olheiros_do_clube original={} alvos {}", r, alvos.len()));
    r
}

/// Comparador: lado a lado pelo olheiro e as missões (+0x174 a +0x199; a 0x53e990 só lê).
unsafe extern "thiscall" fn comparar_trocar_olheiro_da_busca(this: usize, olheiro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(878);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x174, 0x25)], || o(this, olheiro), || trocar_olheiro_da_busca(this, olheiro));
    PLACAR.registrar(igual && r == n, || format!("trocar_olheiro_da_busca({:x}) {:x?} x {:x?}", olheiro, mo, mr));
    r
}

// ---------- 246a leva: depois da partida, a diretoria e a disciplina; as entradas do clube na shortlist e a maior oferta pelo jogador ----------

unsafe fn cpj6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x6951e0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::update_board_after_fixture(partida,
/// jogo)`: partida jogada (+0 ≠ −2, +0x40 ≠ 0): a confiança da diretoria de cada lado com técnico (+0xcf) muda pela
/// 0x695240 (mandante +0x1c com 1, visitante +0x20 com 0).
unsafe extern "thiscall" fn diretoria_depois_da_partida(this: usize, partida: usize, jogo: usize) {
    if cpj6_d(partida) as i32 == -2 || *((partida + 0x40) as *const u8) == 0 {
        return;
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x695240usize);
    for (lado, mandante) in [(0x1cusize, 1u32), (0x20, 0)] {
        let clube = cpj6_d(partida + lado);
        if cpj6_d(clube + 0xcf) != 0 { // o lado tem técnico
            atualizar(this, clube, partida, jogo, mandante);
        }
    }
}

/// O tribunal (vetor [this], pelo byte +0x50 da competição de id `id` em [0xadadfc]), se houver.
unsafe fn cpj6_tribunal(this: usize, id: i32) -> Option<usize> {
    let comp = cpj6_d(cpj6_d(0xadadfc).wrapping_add((id as usize).wrapping_mul(4)));
    if comp == 0 {
        return None;
    }
    let t = *((comp + 0x50) as *const i8);
    if t == -1 {
        return None;
    }
    let b = cpj6_d(cpj6_d(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    (b != 0).then_some(b)
}

/// { 0x55ca00 } (thiscall, ret 8, discipline.cpp): `Discipline::process_match(partida, resultado)`: o tribunal da
/// competição da partida (+0, id > −1) julga (0x7e7ff0 com 1, 1, 1). Sem ele, num amistoso (0x5c6710 sobre [0xae2c74]
/// com +0x14) ou na competição [0x9cf684]: o tribunal da liga (+0x57) do mandante (+0x1c, com 1, 0, 0) e o do
/// visitante (+0x20, com 0, 1, 0). Devolve o que o primeiro tribunal devolve (os outros caminhos, 0; ninguém lê).
unsafe extern "thiscall" fn disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7e7ff0usize);
    let id = cpj6_d(partida) as i32;
    if id > -1 {
        if let Some(b) = cpj6_tribunal(this, id) {
            return julgar(b, partida, resultado, 1, 1, 1);
        }
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cpj6_d(0xae2c74), cpj6_d(partida + 0x14)) == 0 && cpj6_d(partida) != cpj6_d(0x9cf684) {
        return 0;
    }
    for (lado, a, b) in [(0x1cusize, 1u32, 0u32), (0x20, 0, 1)] {
        let liga = cpj6_d(cpj6_d(partida + lado) + 0x57);
        if liga == 0 {
            continue;
        }
        if let Some(t) = cpj6_tribunal(this, cpj6_d(liga) as i32) { // o tribunal da liga do clube
            julgar(t, partida, resultado, a, b, 0);
        }
    }
    0
}

/// As entradas (8 bytes: pessoa, clube) da shortlist (+4, [+0x14] delas, ordenada pelo clube) do clube `id`: a busca
/// binária (bsearch 0x946464 com a chave {−1, id} e a comparação 0x844740), recuando até a primeira do clube.
unsafe fn cpj6_entradas(this: usize, id: u32) -> Vec<usize> {
    let chave = [u32::MAX, id];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let (base, n) = (cpj6_d(this + 4), cpj6_d(this + 0x14));
    let mut e = bsearch(chave.as_ptr(), base, n, 8, 0x844740);
    if e == 0 {
        return Vec::new();
    }
    let ultima = base.wrapping_add(n.wrapping_mul(8)).wrapping_sub(8);
    while e > base && cpj6_d(e - 4) as u32 == id {
        e -= 8; // a anterior também é do clube
    }
    let mut v = Vec::new();
    if cpj6_d(e + 4) as u32 != id {
        return v;
    }
    loop {
        v.push(e);
        if e == ultima || cpj6_d(e + 0xc) as u32 != id {
            return v;
        }
        e += 8;
    }
}

/// { 0x8420e0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::collect_club_shortlist_entries(clube,
/// lista)`: acrescenta (0x68d0d0, 8 bytes) à `lista` cada entrada da shortlist do clube (`*clube`), na ordem.
unsafe extern "thiscall" fn entradas_do_clube_na_shortlist(this: usize, clube: *const u32, lista: usize) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    for e in cpj6_entradas(this, *clube) {
        acrescentar(lista, e, 8);
    }
}

/// A oferta `o` serve: do `clube` (+8) ou marcada (+0x2d = 1), e no estado 2 (+0x2c)?
unsafe fn cpj6_serve(o: usize, clube: u32) -> bool {
    (cpj6_d(o + 8) as u32 == clube || *((o + 0x2d) as *const u8) == 1) && *((o + 0x2c) as *const u8) == 2
}

/// { 0x8cc930 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_highest_offer_for_player(jogador,
/// clube)`: com a marca de ofertas do jogador (byte +1 dos 8 de [+0x86e]), o primeiro registro do vetor [this] ([+4]
/// deles) do jogador (+0xc): sem lista (+0), ele mesmo se serve; com lista, das ofertas (índices no vetor) que servem,
/// a de maior valor (0x8e7a70 com 0, 0, 0, 1; a primeira no empate). Senão 0.
unsafe extern "thiscall" fn maior_oferta_pelo_jogador(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let id = *jogador;
    if *((cpj6_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8)) + 1) as *const u8) == 0 {
        return 0;
    }
    let mut t = 0;
    let mut k = 0i32;
    while k < cpj6_d(this + 4) as i32 {
        let x = cpj6_d(cpj6_d(this).wrapping_add(k as usize * 4));
        if x != 0 && cpj6_d(x + 0xc) as u32 == id {
            t = x;
            break;
        }
        k += 1;
    }
    if t == 0 {
        return 0;
    }
    let l = cpj6_d(t);
    if l == 0 {
        return if cpj6_serve(t, *clube) { t } else { 0 };
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj6_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let (mut melhor, mut maior) = (0usize, -1i32);
    let mut e = proximo(l);
    while e != 0 {
        let o = cpj6_d(cpj6_d(this).wrapping_add(cpj6_d(e).wrapping_mul(4)));
        if cpj6_serve(o, *clube) {
            let v = valor(o, 0, 0, 0, 1);
            if v > maior { // a primeira no empate
                melhor = o;
                maior = v;
            }
        }
        e = proximo(l);
    }
    melhor
}

/// Comparador: a 0x695240 mexe na diretoria (e pode dar notícia): só a original (a prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_diretoria_depois_da_partida(this: usize, partida: usize, jogo: usize) {
    crate::comparacao::ramo("diretoria_depois_da_partida");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(879)(this, partida, jogo)
}

/// Comparador: o tribunal (0x7e7ff0) dá as suspensões: só a original.
unsafe extern "thiscall" fn comparar_disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    crate::comparacao::ramo("disciplina_da_partida");
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(880)(this, partida, resultado)
}

/// Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
unsafe extern "thiscall" fn comparar_entradas_do_clube_na_shortlist(this: usize, clube: *const u32, lista: usize) {
    let esperado: Vec<[u8; 8]> = cpj6_entradas(this, *clube).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let antes = cpd1_itens(lista).len();
    orig::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(881)(this, clube, lista);
    let novos: Vec<[u8; 8]> = cpd1_itens(lista).iter().skip(antes).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    PLACAR.registrar(novos == esperado, || format!("entradas_do_clube_na_shortlist({}) original {} itens, rust {}", *clube, novos.len(), esperado.len()));
}

/// Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
unsafe extern "thiscall" fn comparar_maior_oferta_pelo_jogador(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(882)(this, jogador, clube);
    let n = maior_oferta_pelo_jogador(this, jogador, clube);
    PLACAR.registrar(r == n, || format!("maior_oferta_pelo_jogador({}, {}) original={:#x} rust={:#x}", *jogador, *clube, r, n));
    r
}
