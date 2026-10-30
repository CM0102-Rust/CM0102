// ---------- 250a leva: a entrega das notícias: pelo alcance, ao técnico do clube e na caixa do técnico humano (news.cpp) ----------

unsafe fn cpn1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpn1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +0xd6 id, +0xda livre/lida, +0xde alcance).
const CPN1_NOTICIA: usize = 0xdf;

/// A entrada do técnico humano na tabela das caixas (News +8, 12 bytes: início do anel, a próxima vaga, a conta):
/// índice = id da pessoa − [0xae2364] + 16 (os humanos são as 16 últimas pessoas). Negativo: não é humano.
unsafe fn cpn1_indice_humano(tecnico: usize) -> i32 {
    cpn1_i(tecnico).wrapping_sub(cpn1_i(0xae2364)).wrapping_add(0x10)
}

/// { 0x786090 } (thiscall, ret 0x18, news.cpp): `News::send_by_scope(noticia, clube, competicao, nacao, continente,
/// alcance)`: o alcance (char) escolhe a entrega: 0 o técnico do clube (0x7861e0), 1 os da divisão (0x7862d0, pela
/// competição), 2 os do país (0x7863f0, pela nação), 3 os do continente (0x786640), 4 todos (0x786780), 5 os da região
/// da competição (0x786860). Fora disso, a caixa "Error" e 0. Devolve o que a entrega devolve.
unsafe extern "thiscall" fn noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let dois: unsafe extern "thiscall" fn(usize, usize, usize) -> u32;
    let alvo = match alcance as u8 as i8 {
        0 => {
            crate::comparacao::ramo("cpn_1 alcance do clube");
            dois = core::mem::transmute(0x7861e0usize);
            clube // o técnico do clube
        }
        1 => {
            dois = core::mem::transmute(0x7862d0usize);
            competicao
        }
        2 => {
            dois = core::mem::transmute(0x7863f0usize);
            nacao
        }
        3 => {
            dois = core::mem::transmute(0x786640usize);
            continente
        }
        4 => {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
            return todos(this, noticia);
        }
        5 => {
            dois = core::mem::transmute(0x786860usize);
            competicao
        }
        _ => {
            aviso_em(0xa693ec, 0x1fd3);
            return 0;
        }
    };
    dois(this, noticia, alvo)
}

/// { 0x7861e0 } (thiscall, ret 8, news.cpp): `News::send_to_club_manager(noticia, clube)`: marca o alcance da notícia
/// como clube (+0xde = 0) e a entrega (0x785d40) ao técnico do clube (+0xcf) se ele for humano (id entre as 16 últimas
/// pessoas). Sem notícia ou sem clube: a caixa "Error" e 0; sem técnico ou técnico da máquina: 0.
unsafe extern "thiscall" fn noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x2026);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa693ec, 0x202e);
        return 0;
    }
    let tecnico = cpn1_d(clube + 0xcf);
    *((noticia + 0xde) as *mut u8) = 0;
    if tecnico == 0 {
        return 0;
    }
    if cpn1_i(tecnico) < cpn1_i(0xae2364).wrapping_sub(0x10) { // a máquina
        return 0;
    }
    crate::comparacao::ramo("cpn_1 noticia ao tecnico humano");
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x785d40usize);
    entregar(this, noticia, tecnico, 0)
}

/// { 0x785d40 } (thiscall, ret 0xc, news.cpp): `News::deliver_to_manager(noticia, tecnico, _)`: grava se a notícia é
/// importante para ele (+4, 0x784eb0) e zera +0xda; acha a entrada do humano (News +8); na primeira notícia dele, o anel
/// de 100 vagas no fim do vetor das notícias (News +0, conta de vagas em +4, realloc 0x945667; sem memória, caixa e 0)
/// com as vagas zeradas (+0xd6 = 0, +0xda = 1). Notícia sem id (+0xd6) recebe o próximo (News +0x10). Se a vaga anterior
/// tem o mesmo id, não repete (devolve 1). Senão copia a notícia (0x786010) na próxima vaga (a vaga ainda não lida, +0xda
/// = 0, sai da conta), avança o anel e, com todos de férias ([0xdd6bec]) ou o técnico de férias (0x5fd210), dá a
/// resposta automática (0x784100) pelo clube (+0x39) e pela seleção (+0x24). Soma 1 na conta e devolve 1.
unsafe extern "thiscall" fn entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, _livre: u32) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x1f3f);
        return 0;
    }
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x1f47);
        return 0;
    }
    let importante: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x784eb0usize);
    *((noticia + 4) as *mut u8) = importante(noticia, tecnico) as u8;
    cpn1_por(noticia + 0xda, 0);
    let idx = cpn1_indice_humano(tecnico);
    if idx < 0 {
        aviso_em(0xa693ec, 0x1f58);
        return 0;
    }
    let e = cpn1_d(this + 8) + idx as usize * 12;
    if cpn1_i(e) == -1 {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        cpn1_por(e, cpn1_d(this + 4));
        cpn1_por(e + 4, cpn1_d(this + 4));
        let total = cpn1_d(this + 4).wrapping_add(0x64);
        cpn1_por(this + 4, total);
        let p = realocar(cpn1_d(this), total.wrapping_mul(CPN1_NOTICIA) as u32);
        cpn1_por(this, p);
        if p == 0 {
            aviso_em(0xa693ec, 0x1f6d);
            return 0;
        }
        for k in 0..0x64usize {
            let v = cpn1_d(this) + (cpn1_d(e) + k).wrapping_mul(CPN1_NOTICIA);
            cpn1_por(v + 0xd6, 0);
            cpn1_por(v + 0xda, 1);
        }
    }
    if cpn1_d(noticia + 0xd6) == 0 {
        let id = cpn1_d(this + 0x10).wrapping_add(1);
        cpn1_por(this + 0x10, id);
        cpn1_por(noticia + 0xd6, id);
    }
    let atual = cpn1_i(e + 4);
    let mut anterior = atual.wrapping_sub(1);
    if anterior < cpn1_i(e) {
        anterior = cpn1_i(e) + 0x63;
    }
    let vaga = |k: i32| cpn1_d(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
    if cpn1_d(vaga(anterior) + 0xd6) == cpn1_d(noticia + 0xd6) {
        return 1; // a mesma notícia de novo
    }
    if cpn1_d(vaga(atual) + 0xda) == 0 {
        cpn1_por(e + 8, cpn1_d(e + 8).wrapping_sub(1));
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x786010usize);
    copiar(vaga(atual), noticia);
    let atual = cpn1_i(e + 4);
    let inicio = cpn1_i(e);
    cpn1_por(e + 4, atual.wrapping_add(1) as usize);
    if atual.wrapping_add(1) == inicio.wrapping_add(0x64) {
        cpn1_por(e + 4, inicio as usize);
    }
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    if *(0xdd6bec as *const i16) != 0 || ferias(cpn1_d(0xb63c98), tecnico) != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x784100usize);
        crate::comparacao::ramo("cpn_1 resposta automatica");
        responder(vaga(atual), tecnico, cpn1_d(tecnico + 0x39)); // o clube
        let selecao = cpn1_d(tecnico + 0x24);
        if selecao != 0 {
            responder(vaga(atual), tecnico, selecao);
        }
    }
    cpn1_por(e + 8, cpn1_d(e + 8).wrapping_add(1));
    1
}

/// Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
unsafe extern "thiscall" fn comparar_noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(946)(this, noticia, clube, competicao, nacao, continente, alcance);
    let fora = !(0..=5).contains(&(alcance as u8 as i8));
    PLACAR.registrar(!fora || r == 0, || format!("noticia_pelo_alcance alcance {} devolveu {}", alcance as u8 as i8, r));
    r
}

/// Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
unsafe extern "thiscall" fn comparar_noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(947);
    if noticia == 0 || clube == 0 {
        return o(this, noticia, clube);
    }
    let tecnico = cpn1_d(clube + 0xcf);
    let humano = tecnico != 0 && cpn1_i(tecnico) >= cpn1_i(0xae2364).wrapping_sub(0x10);
    let r = o(this, noticia, clube);
    let ok = *((noticia + 0xde) as *const u8) == 0 && (humano || r == 0) && r <= 1;
    PLACAR.registrar(ok, || format!("noticia_ao_tecnico_do_clube(clube {:x}, técnico {:x}, humano {}) devolveu {}", clube, tecnico, humano, r));
    r
}

/// Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
unsafe extern "thiscall" fn comparar_entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, livre: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(948);
    if noticia == 0 || tecnico == 0 || cpn1_indice_humano(tecnico) < 0 {
        return o(this, noticia, tecnico, livre);
    }
    let idx = cpn1_indice_humano(tecnico) as usize;
    let e = cpn1_d(this + 8) + idx * 12;
    // o plano
    let novo = cpn1_i(e) == -1;
    let (inicio, atual, conta) = if novo { (cpn1_i(this + 4), cpn1_i(this + 4), cpn1_i(e + 8)) } else { (cpn1_i(e), cpn1_i(e + 4), cpn1_i(e + 8)) };
    let contador = cpn1_d(this + 0x10);
    let (id, contador_depois) = if cpn1_d(noticia + 0xd6) == 0 { (contador.wrapping_add(1), contador.wrapping_add(1)) } else { (cpn1_d(noticia + 0xd6), contador) };
    let mut anterior = atual - 1;
    if anterior < inicio {
        anterior = inicio + 0x63;
    }
    let (repetida, livre_atual) = if novo {
        (id == 0, true)
    } else {
        let v = |k: i32| cpn1_d(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
        (cpn1_d(v(anterior) + 0xd6) == id, cpn1_d(v(atual) + 0xda) != 0)
    };
    let (prox, conta_depois) = if repetida {
        (atual, conta)
    } else {
        let p = if atual + 1 == inicio + 0x64 { inicio } else { atual + 1 };
        (p, conta + if livre_atual { 1 } else { 0 })
    };
    let r = o(this, noticia, tecnico, livre);
    if r == 0 {
        PLACAR.registrar(novo, || format!("entregar_noticia_ao_tecnico devolveu 0 sem anel novo")); // só sem memória
        return r;
    }
    let vaga = cpn1_d(this).wrapping_add((atual as usize).wrapping_mul(CPN1_NOTICIA));
    let ok = r == 1 && cpn1_i(e) == inicio && cpn1_i(e + 4) == prox && cpn1_i(e + 8) == conta_depois
        && cpn1_d(this + 0x10) == contador_depois && cpn1_d(noticia + 0xd6) == id && cpn1_d(noticia + 0xda) == 0
        && (repetida || cpn1_d(vaga + 0xd6) == id);
    PLACAR.registrar(ok, || format!("entregar_noticia_ao_tecnico(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, cpn1_d(noticia + 0xd6), id, cpn1_i(e), cpn1_i(e + 4), cpn1_i(e + 8), inicio, prox, conta_depois, repetida));
    r
}

// ---------- 250a leva: a caixa do técnico humano: responder a todas as notícias, se há notícia pendente e o texto num campo da notícia (news.cpp) ----------

unsafe fn cpn2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +5 os 50 campos de 4 bytes, +0xcc fim do texto, +0xd6 id,
/// +0xda lida).
const CPN2_NOTICIA: usize = 0xdf;

/// A entrada do humano na tabela das caixas (News +8, 12 bytes: início do anel, próxima vaga, conta), ou None (com a
/// caixa "Error" da `linha`) se a pessoa não está entre as 16 últimas.
unsafe fn cpn2_entrada(this: usize, tecnico: usize, linha: u32) -> Option<usize> {
    let idx = cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, linha);
        return None;
    }
    Some(cpn2_d(this + 8) + idx as usize * 12)
}

/// A vaga anterior a `k` no anel que começa em `inicio` (100 vagas).
fn cpn2_volta(k: i32, inicio: i32) -> i32 {
    if k < inicio { inicio + 0x63 } else { k }
}

/// { 0x7874b0 } (thiscall, ret 8, news.cpp): `News::auto_respond_all(tecnico, clube)`: da notícia mais nova para a mais
/// antiga do anel do humano, até a primeira vaga vazia (+0xd6 = 0), dá a resposta automática (0x784100) pelo `clube`.
/// Sem técnico ou técnico da máquina: a caixa "Error" e 0; sem anel ou no fim: 1. Chamada ao sair de férias, ao ser
/// demitido e ao ser contratado.
unsafe extern "thiscall" fn responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2460);
        return 0;
    }
    let e = match cpn2_entrada(this, tecnico, 0x246b) {
        Some(e) => e,
        None => return 0,
    };
    if cpn2_i(e) == -1 {
        return 1;
    }
    let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x784100usize);
    let mut k = cpn2_i(e + 4) - 1;
    loop {
        k = cpn2_volta(k, cpn2_i(e));
        let vaga = cpn2_d(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if cpn2_d(vaga + 0xd6) == 0 {
            return 1;
        }
        crate::comparacao::ramo("cpn_2 responder todas");
        responder(vaga, tecnico, clube); // a resposta automática
        k -= 1;
        if k == cpn2_i(e + 4) - 1 {
            return 1;
        }
    }
}

/// { 0x787350 } (thiscall, ret 8, news.cpp): `News::has_urgent_items(tecnico, so_urgentes)`: sem `so_urgentes`, se há
/// notícia não lida (0x786fe0 > 0). Com ele, da mais nova para a mais antiga do anel do humano, até a primeira vaga vazia:
/// 1 se uma pede resposta (o texto 0x782b70 em 0xde1f64 devolve 2) ou é importante (+4 = 1) e não lida (+0xda = 0).
/// Técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    if so_urgentes == 0 {
        let nao_lidas: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x786fe0usize);
        return (nao_lidas(this, tecnico, 0) > 0) as u32;
    }
    let e = match cpn2_entrada(this, tecnico, 0x241e) {
        Some(e) => e,
        None => return 0,
    };
    if cpn2_i(e) == -1 {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(0x782b70usize);
    let mut k = cpn2_i(e + 4) - 1;
    loop {
        k = cpn2_volta(k, cpn2_i(e));
        let vaga = cpn2_d(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if cpn2_d(vaga + 0xd6) == 0 {
            return 0;
        }
        let mut saida = 0u32;
        if texto(vaga, tecnico, 0xde1f64, 0x7d0, &mut saida) as u8 == 2 {
            return 1; // pede resposta
        }
        let vaga = cpn2_d(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if *((vaga + 4) as *const u8) == 1 && cpn2_d(vaga + 0xda) == 0 {
            return 1; // importante e não lida
        }
        k -= 1;
        if k == cpn2_i(e + 4) - 1 {
            return 0;
        }
    }
}

/// { 0x7858c0 } (thiscall, ret 0xc, news.cpp): `NewsItem::set_slot_string(campo, texto, proximo)`: copia o `texto` nos
/// campos da notícia a partir do `campo` (strncpy 0x9466d0 em +5 + 4·campo, até 200 − 4·campo bytes), termina a área
/// (+0xcc = 0) e, com `proximo`, grava o primeiro campo livre depois do texto (o tamanho com o zero arredondado para
/// 4, em campos, mais o `campo`; no máximo 50). Campo fora de 0..49 ou sem texto: a caixa "Error" e 0; senão 1.
unsafe extern "thiscall" fn gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let campo = campo as u8 as i8;
    if !(0..0x32).contains(&campo) {
        aviso_em(0xa693ec, 0x1e8d);
        return 0;
    }
    if texto.is_null() {
        aviso_em(0xa693ec, 0x1e95);
        return 0;
    }
    let copiar: unsafe extern "C" fn(usize, *const u8, u32) -> usize = core::mem::transmute(0x9466d0usize);
    let d = 4 * campo as i32;
    copiar(this + 5 + d as usize, texto, (0xc8 - d) as u32);
    *((this + 0xcc) as *mut u8) = 0;
    if !proximo.is_null() {
        let mut n = 0i32;
        while *texto.add(n as usize) != 0 {
            n += 1;
        }
        let quartos = ((n + 1 + 3) & !3) / 4; // o tamanho com o zero, em campos de 4 bytes
        let mut p = (quartos as u8).wrapping_add(campo as u8);
        crate::comparacao::ramo("cpn_2 proximo campo do texto");
        if p as i8 > 0x32 {
            p = 0x32;
        }
        *proximo = p; // o próximo campo livre
    }
    1
}

/// Comparador: a original responde (convites, propostas, emprego): roda só ela e confere a devolução (0 só sem técnico
/// ou com técnico da máquina; senão 1).
unsafe extern "thiscall" fn comparar_responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    let humano = tecnico != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) >= 0;
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(949)(this, tecnico, clube);
    PLACAR.registrar(r == humano as u32, || format!("responder_todas_as_noticias(técnico {:x}, humano {}) devolveu {}", tecnico, humano, r));
    r
}

/// Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
unsafe extern "thiscall" fn comparar_tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(950);
    if so_urgentes != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico, so_urgentes), || tecnico_tem_noticia_pendente(this, tecnico, so_urgentes));
    PLACAR.registrar(igual && r == n, || format!("tecnico_tem_noticia_pendente(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
unsafe extern "thiscall" fn comparar_gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(951);
    if !(0..0x32).contains(&(campo as u8 as i8)) || texto.is_null() {
        return o(this, campo, texto, proximo); // a caixa "Error"
    }
    let mut reg = vec![(this + 5, 0xc8usize)];
    if !proximo.is_null() {
        reg.push((proximo as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, campo, texto, proximo), || gravar_texto_na_noticia(this, campo, texto, proximo));
    PLACAR.registrar(igual && r == n, || format!("gravar_texto_na_noticia(campo {}) {:x?} x {:x?}", campo as u8 as i8, mo, mr));
    r
}

// ---------- 250a leva: a mídia e a caixa: a nota do jogador para uma história, o texto da crise financeira e as notícias da caixa do humano (media.cpp, news.cpp) ----------

unsafe fn cpn3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpn3_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpn3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A notícia guardada na caixa: 0xdf bytes (+0xd6 id).
const CPN3_NOTICIA: usize = 0xdf;

/// A razão entre a reputação do jogador (+0xb do registro +0x61, no mínimo 2000) e a do clube (+0x80, entre 2000 e
/// 7500), como o fild/fidiv do original.
unsafe fn cpn3_razao_das_reputacoes(jogador: usize, clube: usize) -> f64 {
    let a = cpn3_w(cpn3_d(jogador + 0x61) + 0xb);
    let a = if a < 2000 { 2000 } else { a as i32 };
    let b = cpn3_w(clube + 0x80);
    let b = if b < 2000 { 2000 } else if b > 7500 { 7500 } else { b as i32 };
    a as f64 / b as f64
}

/// O tipo de história (0..0x52) → o caso do original (a tabela de bytes de 0x736b8c; 12 = a nota como está).
fn cpn3_caso(tipo: i16) -> u8 {
    match tipo {
        0 => 0,
        1 | 2 | 9 => 1,
        3 => 2,
        4 => 3,
        6 => 4,
        7 => 5,
        12 => 6,
        13 => 7,
        17 => 8,
        30 => 9,
        35 => 10,
        82 => 11,
        _ => 12,
    }
}

/// { 0x736720 } (thiscall, ret 0x18, media.cpp): `Media::rate_player_form_for_story(jogador, clube, contrato, tipo,
/// modo, peso)`: a nota (x87, em st0) do jogador para uma história da mídia. Sem contrato, o do jogador com o clube
/// (0x4cf5f0); sem nenhum, 0. Base: a nota média da liga x ([0xb74350] +0x10 / +0x18, sem jogos 6,8), as estatísticas
/// do jogador (0x7c1e30, categoria 6: s1 campo 1, s2 campo 0x11, s3 campo 2, padrão 6,5) e a média das notas
/// (0x7c1950) × 0,2: r = (s2 − x) + (média − x) − peso/60; com s1 < 12, r × s1/12; negativa, × a razão das
/// reputações. Depois, pelo tipo (0..0x52), somas do tipo de contrato (+0x4f >> 4), da idade, dos dias de casa
/// (0x54cdb0), de atributos do registro +0x61, do peso; o tipo 0x52 dá r × 0,75 + s3/max(s1, 1) − 0,4.
unsafe extern "thiscall" fn nota_do_jogador_para_a_historia(_this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let mut contrato = contrato;
    if contrato == 0 {
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato = com_clube(cpn3_d(0xae1050), jogador, clube);
        if contrato == 0 {
            return 0.0;
        }
    }
    let estat: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let media_notas: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7c1950usize);
    let g = cpn3_d(0xb74350);
    let jogos = core::ptr::read_unaligned((g + 0x18) as *const f64);
    let x = if jogos == 0.0 || jogos.is_nan() { 6.8 } else { core::ptr::read_unaligned((g + 0x10) as *const f64) / jogos };
    let xf = x as f32;
    let s1f = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 1, 6.5) as f32;
    let s2f = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 0x11, 6.5) as f32;
    let media = media_notas(cpn3_d(0xb74350), cpn3_d(jogador + 0x61)) as u8 as i8 as i32;
    let mut l0 = (media as f64 * 0.2) as f32;
    let y = if s1f as f64 > 1.0 { s1f as f64 } else { 1.0 }; // as partidas, no mínimo 1
    let s3 = estat(cpn3_d(0xb74350), cpn3_d(jogador + 0x61), 6, 2, 6.5);
    let l2 = (s3 / y) as f32;
    l0 = (l0 as f64 - xf as f64) as f32;
    let dif = s2f as f64 - xf as f64;
    let dif_f = dif as f32;
    crate::comparacao::ramo("cpn_3 nota do jogador");
    let mut r = dif + l0 as f64 - peso as f64 * (1.0f32 / 60.0) as f64; // o peso da história
    if !(s1f as f64 >= 12.0) {
        crate::comparacao::ramo("cpn_3 poucas partidas");
        r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // menos de 12 partidas
    }
    if !(r >= 0.0) {
        crate::comparacao::ramo("cpn_3 nota negativa");
        r = r * cpn3_razao_das_reputacoes(jogador, clube); // a negativa pela reputação
    }
    let rf = r as f32;
    let tipo = tipo as u16 as i16;
    if !(0..=0x52).contains(&tipo) {
        return r;
    }
    let c = *((contrato + 0x4f) as *const u8) >> 4; // o tipo de contrato
    match cpn3_caso(tipo) {
        0 => match c {
            1 => r - 0.5,
            2 => r - 0.35f32 as f64,
            3 => r - 0.2f32 as f64,
            _ => r + 0.25,
        },
        1 => {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            let d = dias(0xae2c90, cpn3_d(jogador + 0x3e) as u32, cpn3_d(jogador + 0x42) as u32);
            (200i32.wrapping_sub(d)) as f64 * 0.1f32 as f64 + rf as f64
        }
        2 => {
            let r2 = r + dif_f as f64;
            match c {
                1 => r2 + 2.0,
                2 => r2 + 1.5,
                3 => r2 + 1.0,
                _ => r2,
            }
        }
        3 => r + (0x23 - cpn3_b(jogador + 0x18) as i32) as f64 * (1.0f32 / 3.0) as f64,
        4 => r + (10 - cpn3_b(cpn3_d(jogador + 0x61) + 0x25) as i32) as f64 * 0.2f32 as f64,
        5 => r + (cpn3_b(jogador + 0x18) as i32 - 0x13) as f64 * (1.0f32 / 3.0) as f64,
        6 => r + (cpn3_b(cpn3_d(jogador + 0x61) + 0x2f) as i32 - 10) as f64 * (1.0f32 / 3.0) as f64,
        7 => r + l0 as f64,
        8 => {
            let r2 = if r > 0.0 { r / cpn3_razao_das_reputacoes(jogador, clube) } else { r };
            match c {
                1 => r2 - 0.5,
                2 => r2 - 0.35f32 as f64,
                3 => r2 - 0.2f32 as f64,
                _ => r2,
            }
        }
        9 => r - peso as f64 * 0.1f32 as f64,
        10 => match c {
            0 => r - 1.0,
            1 => r + 2.0,
            2 => r + 1.5,
            3 => r + 1.0,
            7 => {
                if modo as u8 == 3 { 0.0 } else { r - 3.0 }
            }
            _ => r,
        },
        11 => r * 0.75 + l2 as f64 - 0.4f32 as f64,
        _ => r,
    }
}

/// { 0x74af60 } (thiscall, ret 0xc, media.cpp): `Media::write_financial_trouble_news_text(noticia, corpo, saida)`: o
/// título ou o corpo da notícia do clube em crise: lê os campos 0 (o clube), 1 (a gravidade) e 2 da notícia (0x785830),
/// monta em 0xde1f64 (0x66f4e0) o texto pela gravidade (> 0x1a5e, > 0xcb2 no corpo; > 0x1a5e no título) com o nome do
/// clube (+0x38) e o gênero (+0x52), põe na `saida` (0x4ae660) e liga o nome ao clube (0x4ae8a0, tipo 0x7d5).
unsafe extern "thiscall" fn texto_da_crise_financeira(_this: usize, noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x785830usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ae660usize);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x4ae8a0usize);
    crate::comparacao::ramo("cpn_3 texto da crise financeira");
    let id = campo(noticia, 0);
    let gravidade = campo(noticia, 1);
    campo(noticia, 2);
    let clube = cpn3_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245));
    let nome = clube + 0x38;
    let genero = cpn3_b(clube + 0x52) as i32;
    let modelo = if corpo != 0 {
        if gravidade > 0x1a5e {
            0xa436fc
        } else if gravidade > 0xcb2 {
            0xa43644
        } else {
            0xa43594
        }
    } else if gravidade > 0x1a5e {
        0xa43560 // "... financial mess"
    } else {
        0xa43528 // "Financial trouble at ..."
    };
    formatar(0xde1f64, modelo, genero, genero, nome);
    por_texto(saida, 0xde1f64);
    ligar(saida, nome, 0x7d5, clube);
}

/// { 0x786960 } (thiscall, ret 0xc, news.cpp): `News::get_inbox_items(tecnico, conta, categoria)`: um vetor novo de 100
/// notícias (malloc 0x944e46 de 0x571c, na primeira achada; `*conta` = 0) com as notícias do anel do humano, da mais nova
/// para a mais antiga até a vaga vazia, só as da `categoria` (0x785670) se ela não for 0, copiadas (0x786010); `*conta`
/// soma uma por notícia. Devolve o vetor (0 sem nenhuma; `*conta` intocada). Sem técnico ou técnico da máquina: a caixa
/// "Error" e 0.
unsafe extern "thiscall" fn noticias_da_caixa(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x21fa);
        return 0;
    }
    let idx = cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, 0x2205);
        return 0;
    }
    let e = cpn3_d(this + 8) + idx as usize * 12;
    if cpn3_i(e) == -1 {
        return 0;
    }
    let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x786010usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let mut vetor = 0usize;
    let mut k = cpn3_i(e + 4) - 1;
    loop {
        if k < cpn3_i(e) {
            k = cpn3_i(e) + 0x63;
        }
        let vaga = cpn3_d(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA));
        if cpn3_d(vaga + 0xd6) == 0 {
            return vetor;
        }
        if categoria as u8 == 0 || tipo(vaga) as u8 == categoria as u8 {
            if vetor == 0 {
                vetor = alocar(0x571c);
                if vetor == 0 {
                    aviso_em(0xa693ec, 0x222a);
                    return 0;
                }
                *conta = 0;
            }
            let n = *conta;
            *conta = n + 1;
            copiar(vetor.wrapping_add((n as usize).wrapping_mul(CPN3_NOTICIA)), cpn3_d(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA)));
        }
        k -= 1;
        if k == cpn3_i(e + 4) - 1 {
            return vetor;
        }
    }
}

/// Comparador: a nota é pura (as estatísticas, as datas e o contrato só leem; nada sorteia): compara a devolução bit a
/// bit (a original e a cópia na mesma precisão dupla do x87).
unsafe extern "thiscall" fn comparar_nota_do_jogador_para_a_historia(this: usize, jogador: usize, clube: usize, contrato: usize, tipo: u32, modo: u32, peso: f32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64>(952)(this, jogador, clube, contrato, tipo, modo, peso);
    let n = nota_do_jogador_para_a_historia(this, jogador, clube, contrato, tipo, modo, peso);
    PLACAR.registrar(o.to_bits() == n.to_bits() || (o.is_nan() && n.is_nan()),
        || format!("nota_do_jogador_para_a_historia(tipo {}, modo {}, peso {}) {} x {}", tipo as u16 as i16, modo as u8, peso, o, n));
    o
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original (a cópia
/// prova no modo Rust, pelo ramo).
unsafe extern "thiscall" fn comparar_texto_da_crise_financeira(this: usize, noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(953)(this, noticia, corpo, saida);
}

/// Comparador: a original aloca o vetor: roda só ela e confere o plano feito antes (a conta e os ids, na ordem).
unsafe extern "thiscall" fn comparar_noticias_da_caixa(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(954);
    if tecnico == 0 || cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, conta, categoria);
    }
    let e = cpn3_d(this + 8) + (cpn3_i(tecnico).wrapping_sub(cpn3_i(0xae2364)).wrapping_add(0x10)) as usize * 12;
    let mut ids = Vec::new();
    if cpn3_i(e) != -1 {
        let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
        let mut k = cpn3_i(e + 4) - 1;
        loop {
            if k < cpn3_i(e) {
                k = cpn3_i(e) + 0x63;
            }
            let vaga = cpn3_d(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA));
            if cpn3_d(vaga + 0xd6) == 0 {
                break;
            }
            if categoria as u8 == 0 || tipo(vaga) as u8 == categoria as u8 {
                ids.push(cpn3_d(vaga + 0xd6));
            }
            k -= 1;
            if k == cpn3_i(e + 4) - 1 {
                break;
            }
        }
    }
    let r = o(this, tecnico, conta, categoria);
    let ok = if ids.is_empty() {
        r == 0
    } else {
        r == 0 || (*conta as usize == ids.len() && ids.iter().enumerate().all(|(i, &id)| cpn3_d(r + i * CPN3_NOTICIA + 0xd6) == id))
    };
    PLACAR.registrar(ok, || format!("noticias_da_caixa(categoria {}) {} notícias, esperava {:?}", categoria as u8, if r == 0 { 0 } else { *conta }, ids));
    r
}

// ---------- 250a leva: o jogador que se aposenta sai dos elencos e a escolha do clube do jovem: o de maior nota entre todos e o sorteado da lista da nação (player_regen.cpp) ----------

unsafe fn cpn4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpn4_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpn4_por(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

/// Tira a pessoa `p` do time em `p + campo` (+0x39 o clube, +0x24 a seleção): a tática (0x896c60 de [0xde1d08], com 0
/// e depois com 1 no 3º argumento) e a primeira das 50 vagas (+0xd7) com ela, que esvazia. O time é relido depois de
/// cada chamada, como na original.
unsafe fn cpn4_tirar_do_time(p: usize, campo: usize) {
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    tirar(cpn4_d(0xde1d08), cpn4_d(p + campo), p, 0, 1);
    tirar(cpn4_d(0xde1d08), cpn4_d(p + campo), p, 1, 1);
    let time = cpn4_d(p + campo);
    if let Some(k) = (0..0x32usize).find(|&k| cpn4_d(time + 0xd7 + 4 * k) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * k) as *mut usize, 0);
    }
}

/// O aviso "already retired" no buffer [0xde1f64] (nome +4, sobrenome +8, id +0).
unsafe fn cpn4_ja_aposentado(p: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    formatar(0xde1f64, 0xa70150, cpn4_d(p + 4), cpn4_d(p + 8), cpn4_d(p));
}

/// { 0x7aace0 } (stdcall, ret 4, player_regen.cpp): `PLAYER_REGEN::retire_from_playing_staff(p)`: tira `p` do clube
/// (+0x39) e da seleção (+0x24) (`cpn4_tirar_do_time`) e troca o cargo de jogador pelo da comissão (tabelas 0x7aae60 e
/// 0x7aae90): pelo cargo no clube (+0x3d, byte com sinal; fora de 0..16 vale o padrão) 12 → 5, 13 → 6, 15 → 8 (fica no
/// clube), 0 e 16 (aviso "already retired") nada, 1 a 10 a rotina acaba ali, os outros 0x10 (aposentado). Depois pelo
/// cargo na seleção (+0x28) do mesmo jeito, mas 12/13/15 gravam no +0x3d (a original grava ali, não no +0x28) e o
/// padrão grava 0x10 no +0x28. Sem cargo que fica, o clube (+0x39) e a seleção (+0x24) zeram. Sem SEH, sem sorteio.
unsafe extern "stdcall" fn aposentar_do_elenco(p: usize) {
    if cpn4_d(p + 0x39) != 0 {
        cpn4_tirar_do_time(p, 0x39);
    }
    if cpn4_d(p + 0x24) != 0 {
        cpn4_tirar_do_time(p, 0x24);
    }
    let fica_no_clube = match cpn4_b(p + 0x3d) as i32 {
        0 => false,
        1..=10 => return, // comissão: a original sai pelo fim da rotina
        12 => {
            cpn4_por(p + 0x3d, 5);
            true
        }
        13 => {
            cpn4_por(p + 0x3d, 6);
            true
        }
        15 => {
            cpn4_por(p + 0x3d, 8);
            true
        }
        16 => {
            cpn4_ja_aposentado(p);
            false
        }
        _ => {
            crate::comparacao::ramo("cpn_4 jogador aposentado");
            cpn4_por(p + 0x3d, 0x10); // aposentado
            false
        }
    };
    let fica_na_selecao = match cpn4_b(p + 0x28) as i32 {
        0 => false,
        1..=10 => return,
        12 => {
            cpn4_por(p + 0x3d, 5); // no +0x3d, como a original
            true
        }
        13 => {
            cpn4_por(p + 0x3d, 6);
            true
        }
        15 => {
            cpn4_por(p + 0x3d, 8);
            true
        }
        16 => {
            cpn4_ja_aposentado(p);
            false
        }
        _ => {
            cpn4_por(p + 0x28, 0x10);
            false
        }
    };
    if !fica_no_clube {
        core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
    }
    if !fica_na_selecao {
        core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    }
}

/// O jovem com a segunda nacionalidade (+0x1e) igual a [0x9cf424] só vai para os clubes de id [0x9d0120], [0x9d0134],
/// [0x9d0128], [0x9d01a0] e [0x9d0124]: `true` para os outros clubes.
unsafe fn cpn4_vetado(jogador: usize, clube: usize) -> bool {
    let e = cpn4_d(jogador + 0x1e);
    if e == 0 || cpn4_d(e) != cpn4_d(0x9cf424) {
        return false;
    }
    let id = cpn4_d(clube);
    ![0x9d0120usize, 0x9d0134, 0x9d0128, 0x9d01a0, 0x9d0124].iter().any(|&g| cpn4_d(g) == id)
}

/// O clube pode chegar ao jogador (0x8ce990 de [0xde1f54], sem ignorar as ofertas abertas)? O motivo fica numa local
/// (a original o grava na vaga do argumento, que ninguém lê depois).
unsafe fn cpn4_pode_contratar(jogador: usize, clube: usize) -> bool {
    let pode: unsafe extern "thiscall" fn(usize, usize, usize, *mut i16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let mut motivo = 0i16;
    pode(cpn4_d(0xde1f54), jogador, clube, &mut motivo, 0) != 0
}

/// { 0x7ad0e0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_best_club_for_player(jogador)`: entre os
/// clubes ([0xae23b4], 0x245 bytes; os [0xae235c] − 2 × [0xae2350] primeiros, relidos a cada volta) em que o jogador
/// não é estrangeiro (0x53d980 com a nação +0x53 e 1), que não são o time B de um par (0x540a50 com 1 achando o
/// parceiro com 0), não vetados (`cpn4_vetado`), que podem chegar a ele (0x8ce990) e com elenco (0x53c590) abaixo de 50,
/// o de maior nota (0x7ad220, sorteia; comparada em 16 bits, a partir de −25000). Nenhum: 0. Sem SEH; a local do
/// parceiro só é lida quando a 0x540a50 o acha (e a grava).
unsafe extern "thiscall" fn melhor_clube_para_o_jovem(_this: usize, jogador: usize) -> usize {
    let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let nota_de: unsafe extern "stdcall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x7ad220usize);
    let total = || (cpn4_d(0xae235c) as i32).wrapping_sub((cpn4_d(0xae2350) as i32).wrapping_mul(2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    let mut eh_a = 0u32;
    let mut clube = cpn4_d(0xae23b4);
    let mut i = 0i32;
    while i < total() {
        let candidato = estrangeiro(jogador, cpn4_d(clube + 0x53), 1) == 0
            && (parceiro(clube, &mut eh_a, 1) == 0 || eh_a != 0)
            && !cpn4_vetado(jogador, clube);
        if candidato {
            let elenco = elenco_de(clube, 0, 0);
            if cpn4_pode_contratar(jogador, clube) && (elenco as i8) < 0x32 {
                // ponytail: a original empurra o byte do elenco com lixo nos bits altos; a 0x7ad220 só lê o byte
                let s = nota_de(jogador, clube, elenco as u32) as i16;
                if s > melhor_nota {
                    crate::comparacao::ramo("cpn_4 clube melhor para o jovem");
                    melhor = clube;
                    melhor_nota = s;
                }
            }
        }
        i += 1;
        clube += 0x245;
    }
    melhor
}

/// A nação da lista do jovem: a nação (+0x1a) dele; [0x9cf510] e [0x9cf2dc] vão para [0x9cf2e4], e [0x9cf3e4] também
/// com sorteio(5) < 2.
unsafe fn cpn4_nacao_da_lista(jogador: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cpn4_d(cpn4_d(jogador + 0x1a));
    if n == cpn4_d(0x9cf510) || n == cpn4_d(0x9cf2dc) || (n == cpn4_d(0x9cf3e4) && sorteio(5) < 2) {
        return cpn4_d(0x9cf2e4);
    }
    n
}

/// A lista de clubes da nação `n`: [[this] + 4n].
unsafe fn cpn4_lista(this: usize, n: usize) -> usize {
    cpn4_d(cpn4_d(this).wrapping_add(n.wrapping_mul(4)))
}

/// { 0x7ad590 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::pick_club_from_nation_list(jogador)`: percorre a
/// lista de clubes da nação do jovem (`cpn4_nacao_da_lista`; a palavra +0 é a conta, o cursor +0xa recomeça de +2, o
/// iterador 0x68d1b0 devolve o id) e, entre os clubes não vetados (`cpn4_vetado`), que podem chegar a ele (0x8ce990) e
/// com elenco (0x53c590) abaixo de 50, fica com o de maior sorteio da nota (5000 + reputação +0x80 menos as distâncias
/// dela à habilidade da ficha +0x61: +0xb, +7 × 50 e +5 × 50, em 16 bits), comparado em 16 bits a partir de −25000.
/// Sem lista ou lista vazia: 0. A lista que acaba antes da conta: a caixa "Error" (linha 0x19e9) e 0. Sem SEH.
unsafe extern "thiscall" fn clube_da_nacao_para_o_jovem(this: usize, jogador: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let elenco_de: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cpn4_nacao_da_lista(jogador);
    let l = cpn4_lista(this, n);
    if l == 0 || cpn4_w(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpn4_d(l + 2));
    let (mut melhor, mut melhor_nota) = (0usize, -25000i16);
    if cpn4_w(cpn4_lista(this, n)) <= 0 {
        return 0;
    }
    let mut i = 0i32;
    loop {
        let item = proximo(cpn4_lista(this, n));
        if item == 0 {
            aviso_em(0xa6bf9c, 0x19e9);
            return 0;
        }
        let clube = cpn4_d(0xae23b4).wrapping_add(cpn4_d(item).wrapping_mul(0x245));
        if !cpn4_vetado(jogador, clube) && cpn4_pode_contratar(jogador, clube) && (elenco_de(clube, 0, 0) as i8) < 0x32 {
            let ficha = cpn4_d(jogador + 0x61);
            let rep = cpn4_w(clube + 0x80) as i32;
            let c = rep
                .wrapping_sub((cpn4_w(ficha + 0xb) as i32).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((cpn4_w(ficha + 7) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_sub((cpn4_w(ficha + 5) as i32).wrapping_mul(50).wrapping_sub(rep).wrapping_abs())
                .wrapping_add(0x1388);
            crate::comparacao::ramo("cpn_4 sorteio do clube da nacao");
            let nota = sorteio(c as i16 as i32) as i16;
            if nota > melhor_nota {
                melhor = clube;
                melhor_nota = nota;
            }
        }
        i += 1;
        if i as i16 >= cpn4_w(cpn4_lista(this, n)) {
            break;
        }
    }
    melhor
}

/// Comparador: lado a lado pela pessoa (+0x24..+0x3d), pelas 50 vagas do clube e da seleção e pelo aviso em
/// [0xde1f64]. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha, então a tática fica fora.
unsafe extern "stdcall" fn comparar_aposentar_do_elenco(p: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(955);
    let mut reg = vec![(p + 0x24, 0x1a), (0xde1f64, 0x80)];
    for campo in [0x39usize, 0x24] {
        let time = cpn4_d(p + campo);
        if time != 0 {
            reg.push((time + 0xd7, 200));
        }
    }
    let (antes_clube, antes_cargo) = (cpn4_d(p + 0x39), cpn4_b(p + 0x3d));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(p), || aposentar_do_elenco(p));
    PLACAR.registrar(igual, || format!("aposentar_do_elenco(id {}, clube {:x}, cargo {}) {:x?} x {:x?}", cpn4_d(p), antes_clube, antes_cargo, mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x7ad220 sorteia), pelo clube devolvido.
unsafe extern "thiscall" fn comparar_melhor_clube_para_o_jovem(this: usize, jogador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(956);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| melhor_clube_para_o_jovem(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("melhor_clube_para_o_jovem(id {}) original={:x} rust={:x} {:x?} x {:x?}", cpn4_d(jogador), r, n, mo, mr));
    r
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio, pelo clube devolvido e pelo cursor (+0xa) das listas
/// que podem ser percorridas (a da nação do jovem e a de [0x9cf2e4]).
unsafe extern "thiscall" fn comparar_clube_da_nacao_para_o_jovem(this: usize, jogador: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(957);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for n in [cpn4_d(cpn4_d(jogador + 0x1a)), cpn4_d(0x9cf2e4)] {
        let l = cpn4_lista(this, n);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador), || crate::aleatorio::sem_contar(|| clube_da_nacao_para_o_jovem(this, jogador)));
    PLACAR.registrar(igual && r == n, || format!("clube_da_nacao_para_o_jovem(id {}) original={:x} rust={:x} {:x?} x {:x?}", cpn4_d(jogador), r, n, mo, mr));
    r
}

// ---------- 250a leva: as vagas de técnico e o agendamento das ofertas: abrir as vagas que faltam, a melhor nota de candidato a técnico e agendar a oferta (manager_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpn5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpn5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O `clube` já tem vaga na lista [this+8] (+0 conta, cursor +0xa recomeçado de +2, iterador 0x68d1b0; +0xe da vaga o
/// id do clube)? `None`: a lista acabou antes da conta (a caixa da original).
unsafe fn cpn5_tem_vaga(this: usize, clube: usize) -> Option<bool> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cpn5_d(this + 8);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpn5_d(l + 2));
    if cpn5_w(cpn5_d(this + 8)) <= 0 {
        return Some(false);
    }
    let mut j = 0i32;
    loop {
        let v = proximo(cpn5_d(this + 8));
        if v == 0 {
            return None;
        }
        if cpn5_d(v + 0xe) == cpn5_d(clube) {
            crate::comparacao::ramo("cpn_5 clube com vaga aberta");
            return Some(true);
        }
        j += 1;
        if j >= cpn5_w(cpn5_d(this + 8)) as i32 {
            return Some(false);
        }
    }
}

/// O `clube` está sem técnico (0x53ca90), pode ser dirigido (0x544820), não é o time B de um par (0x540a50 com 1
/// achando o parceiro com 0 em `eh_a`) e não tem vaga aberta? `None`: a caixa da lista.
unsafe fn cpn5_falta_vaga(this: usize, clube: usize, eh_a: &mut u32) -> Option<bool> {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    if tecnico(clube) != 0 || dirigivel(clube) == 0 {
        return Some(false);
    }
    if parceiro(clube, eh_a, 1) != 0 && *eh_a == 0 {
        return Some(false);
    }
    cpn5_tem_vaga(this, clube).map(|tem| !tem)
}

/// { 0x69d060 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::fix_missing_jobs()`: cada clube
/// ([0xae23b4], 0x245 bytes, [0xae235c] relido a cada volta) sem técnico, dirigível, que não é time B e sem vaga
/// aberta (`cpn5_falta_vaga`) ganha o aviso "Job not found for %s" (nome +4) em [0xde1f64] e a vaga (0x69ba50 sem
/// técnico anterior). A lista que acaba antes da conta: a caixa "Error" (linha 0x311e) e a rotina acaba. Sem SEH, sem
/// sorteio; a local do parceiro começa em 0 uma vez só e só é lida quando a 0x540a50 o acha (e a grava).
unsafe extern "thiscall" fn abrir_vagas_que_faltam(this: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x69ba50usize);
    let mut eh_a = 0u32;
    let mut clube = cpn5_d(0xae23b4);
    let mut i = 0i32;
    while i < cpn5_d(0xae235c) as i32 {
        match cpn5_falta_vaga(this, clube, &mut eh_a) {
            None => {
                aviso_em(0x9f18a4, 0x311e);
                return;
            }
            Some(true) => {
                formatar(0xde1f64, 0xa05298, clube + 4);
                abrir(this, clube, 0);
            }
            Some(false) => {}
        }
        i += 1;
        clube += 0x245;
    }
}

/// { 0x69c540 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::best_manager_candidate_score(clube,
/// atual)`: a maior nota de técnico (0x69d1b0 com 0 e o cargo 5) entre as pessoas ([0xae23bc], 0x6e bytes,
/// [0xae2364] relido a cada volta) com ficha da comissão (+0x69), das 16 últimas só as humanas (+0x5f), que não são o
/// `atual`, sem as marcas 0x30 no byte +0xb do registro de 79 bytes ([0xae2d0c]), sem contrato de empréstimo (0x4cf6d0),
/// que não são candidatas de outra vaga (0x69fc60), com nação (+0x1a) e o clube com nação (+0x53): de outra nação só
/// com reputação (+0x69 → +0xa) a partir de 3750; de outro continente (+0x75) com +0x69 → +0xc a partir de 4250 e o
/// clube com reputação (+0x80) a partir de 4750; jogador (+0x61) com clube (+0x39) e cargo 11, 12, 13 ou 15 não pode ter
/// contrato com o clube (0x4cf5f0) marcado em +0x49; e aceita para o cargo (0x69f810 com 3,5). Ninguém: −10000.
unsafe extern "thiscall" fn melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let candidato: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x69fc60usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let serve: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(0x69f810usize);
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    let mut melhor = -10000i32;
    let mut p = cpn5_d(0xae23bc);
    let mut i = 0i32;
    while i < cpn5_d(0xae2364) as i32 {
        let reg79 = cpn5_d(0xae2d0c).wrapping_add((i as usize).wrapping_mul(0x4f));
        let pula = 'f: {
            if cpn5_d(p + 0x69) == 0 {
                break 'f true;
            }
            if i >= (cpn5_d(0xae2364) as i32).wrapping_sub(0x10) && cpn5_b(p + 0x5f) == 0 {
                break 'f true;
            }
            if atual == p {
                crate::comparacao::ramo("cpn_5 o tecnico atual fica de fora");
                break 'f true; // o atual fica de fora
            }
            if cpn5_b(reg79 + 0xb) & 0x30 != 0 || emprestimo(cpn5_d(0xae1050), p) != 0 || candidato(this, p) != 0 {
                break 'f true;
            }
            let (nc, np) = (cpn5_d(clube + 0x53), cpn5_d(p + 0x1a));
            if nc != np && cpn5_w(cpn5_d(p + 0x69) + 0xa) < 0xea6 {
                break 'f true;
            }
            if nc == 0 || np == 0 {
                break 'f true;
            }
            if cpn5_b(nc + 0x75) != cpn5_b(np + 0x75) && (cpn5_w(cpn5_d(p + 0x69) + 0xc) < 0x109a || cpn5_w(clube + 0x80) < 0x128e) {
                break 'f true;
            }
            if cpn5_d(p + 0x61) != 0 && cpn5_d(p + 0x39) != 0 && matches!(cpn5_b(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let k = contrato(cpn5_d(0xae1050), p, clube);
                if k != 0 && cpn5_b(k + 0x49) != 0 {
                    break 'f true;
                }
            }
            serve(this, p, clube, 3.5) == 0
        };
        if !pula {
            let s = nota_de(this, p, clube, 0, 5);
            if s > melhor {
                melhor = s;
            }
        }
        i += 1;
        p += 0x6e;
    }
    melhor
}

/// O plano do agendamento: a data da oferta (hoje [0xae2c90] + `dias`; com `dias` ≥ 50 a original mostra a caixa e
/// usa 10) com a fatia [0xae2c88] + `fatias` trazida para 0..2 (cada 3 fatias, mais um dia), e o balde da lista: os
/// dias desde a data base (+0x84d; ainda sem ano em +0x84f, hoje) mais o começo (+0x84c), módulo 50 de 50 a 127, × 3 +
/// fatia, nas listas de 14 bytes a partir de +0x18. Só lê (0x54c5b0 e 0x54cdb0 sobre cópias das datas).
unsafe fn cpn5_plano(this: usize, dias: u8, fatias: u8) -> ([u8; 8], i8, usize) {
    let somar: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let desde: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let base = if cpn5_w(this + 0x84f) == 0 { 0xae2c90 } else { this + 0x84d };
    let (b0, b1) = (cpn5_d(base) as u32, cpn5_d(base + 4) as u32);
    let dias = if dias as i8 >= 0x32 { 10 } else { dias as i8 };
    let (mut data, mut tmp) = ([0u8; 8], [0u8; 8]);
    // a 0x54c5b0 lê os dias em 16 bits (a original empurra o registro com lixo nos bits altos)
    let r = somar(0xae2c90, tmp.as_mut_ptr(), dias as i16 as i32 as u32);
    core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    let mut fatia = cpn5_b(0xae2c88).wrapping_add(fatias) as i8;
    while fatia > 2 {
        fatia -= 3;
        let r = somar(data.as_ptr() as usize, tmp.as_mut_ptr(), 1);
        core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    }
    let mut c = (desde(data.as_ptr() as usize, b0, b1) as u8).wrapping_add(cpn5_b(this + 0x84c));
    if c as i8 >= 0x32 {
        let q = (c as i8 as i32 as u32) / 50;
        c = c.wrapping_add((q as u8).wrapping_mul(0xce)); // − 50 × q
    }
    let balde = (c as i8 as i32).wrapping_mul(3).wrapping_add(fatia as i32);
    (data, fatia, this.wrapping_add(0x18).wrapping_add((balde as usize).wrapping_mul(14)))
}

/// { 0x8c6220 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::schedule_offer(oferta, dias, fatias)`:
/// sem data base (ano +0x84f zerado), ela vira hoje (0x417c10 de [0xae2c90] em +0x84d) e a fatia +0x855 a de hoje
/// ([0xae2c88]); `dias` ≥ 50: a caixa "Error" (linha 0x18b9) e 10 dias. A oferta ganha a data (+0x12) e a fatia (+0x1a)
/// do plano (`cpn5_plano`) e o +0x20 dela (4 bytes) entra no balde (0x68d0d0, que aloca); devolve o que a 0x68d0d0
/// devolve. Sem SEH, sem sorteio.
unsafe extern "thiscall" fn agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    crate::comparacao::ramo("cpn_5 agendar oferta");
    if cpn5_w(this + 0x84f) == 0 {
        let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
        let mut tmp = [0u8; 8];
        copiar(this + 0x84d, tmp.as_mut_ptr(), 0xae2c90);
        *((this + 0x855) as *mut u8) = cpn5_b(0xae2c88);
    }
    if dias as u8 as i8 >= 0x32 {
        aviso_em(0xa8d514, 0x18b9);
    }
    // ponytail: as datas são feitas no plano sobre cópias (0x54c5b0, 0x54cdb0 e 0x417c10 só leem e copiam) e gravadas
    // de uma vez; a original grava a data e a fatia a cada volta da fatia, com o mesmo fim
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    core::ptr::copy_nonoverlapping(data.as_ptr(), (oferta + 0x12) as *mut u8, 8);
    *((oferta + 0x1a) as *mut i8) = fatia;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    acrescentar(balde, oferta + 0x20, 4)
}

/// Comparador: sem vaga a abrir (o plano pelo mesmo teste, com o cursor da lista devolvido), lado a lado pelo cursor
/// da lista; com vaga a abrir (ou a caixa), só a original.
unsafe extern "thiscall" fn comparar_abrir_vagas_que_faltam(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(958);
    let l = cpn5_d(this + 8);
    let cursor = cpn5_d(l + 0xa);
    let mut eh_a = 0u32;
    let mut clube = cpn5_d(0xae23b4);
    let mut vazio = true;
    for _ in 0..(cpn5_d(0xae235c) as i32).max(0) {
        if cpn5_falta_vaga(this, clube, &mut eh_a) != Some(false) {
            vazio = false;
            break;
        }
        clube += 0x245;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    if !vazio {
        // ponytail: a 0x69ba50 abre a vaga (aloca, sorteia o candidato): não roda duas vezes; o modo Rust e a carreira provam
        return o(this);
    }
    let reg = [(l + 0xa, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || abrir_vagas_que_faltam(this));
    PLACAR.registrar(igual, || format!("abrir_vagas_que_faltam (nenhuma a abrir) {:x?} x {:x?}", mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x69d1b0 e a 0x69f810 podem sortear), pela nota e
/// pelo cursor da lista de vagas (a 0x69fc60 o devolve).
unsafe extern "thiscall" fn comparar_melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(959);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((cpn5_d(this + 8) + 0xa, 4));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, atual), || crate::aleatorio::sem_contar(|| melhor_nota_de_candidato_a_tecnico(this, clube, atual)));
    PLACAR.registrar(igual && r == n, || format!("melhor_nota_de_candidato_a_tecnico(clube {}, atual {:x}) original={} rust={} {:x?} x {:x?}", cpn5_d(clube), atual, r, n, mo, mr));
    r
}

/// Comparador: a original roda (a 0x68d0d0 aloca o nó) e o resultado é conferido com o plano feito antes: a data
/// (+0x12) e a fatia (+0x1a) da oferta, a conta do balde mais 1 e a data base (+0x84d, 8 bytes) igual a hoje quando
/// não havia.
unsafe extern "thiscall" fn comparar_agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    let sem_base = cpn5_w(this + 0x84f) == 0;
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    let conta = cpn5_w(balde);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(960)(this, oferta, dias, fatias);
    let gravada = core::slice::from_raw_parts((oferta + 0x12) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(0xae2c90 as *const u8, 8);
    let ok = gravada == &data[..] && *((oferta + 0x1a) as *const i8) == fatia && cpn5_w(balde) == conta.wrapping_add(1) && base_ok;
    PLACAR.registrar(ok, || format!("agendar_oferta(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        dias as u8 as i8, fatias as u8 as i8, gravada, data, *((oferta + 0x1a) as *const i8), fatia, balde, conta, cpn5_w(balde), base_ok));
    r
}

// ---------- 250a leva: a lista de observação do clube: o estatuto no elenco que o clube dá à pessoa, a estimativa do clube humano e soltar os alvos que não são do clube (shortlist_manager.cpp) ----------

unsafe fn cpn6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do contrato `c` (id em +4; o registro de clube tem 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
unsafe fn cpn6_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        return 0;
    }
    cpn6_d(0xae23b4).wrapping_add(cpn6_d(c + 4).wrapping_mul(0x245))
}

/// { 0x849f00 } (thiscall, ret 0xc, shortlist_manager.cpp): `Shortlist_Manager::get_squad_status_for_club(pessoa,
/// clube, cargo)`: o estatuto no elenco que o `clube` dá à `pessoa`. Clube nulo: 7. A entrada da pessoa na lista do
/// clube (0x849ce0: a de tipo ≠ 11 para jogador, +0x61; tendo +0x69, a de tipo 11 também): o cargo (+0 da entrada) e
/// os bits 1..4 do +0x24. Senão o cargo é o +0x3d da pessoa e vale o contrato (0x4cf5a0, guardado em [0xae1080]): do
/// próprio clube, o nibble alto do +0x4f do contrato com o clube (0x4cf5f0; sem ele, 7); clube humano (0x600a50) com
/// cargo 11 a 15 (0x53b620): a estimativa 0x849d60; senão, o clube da pessoa (+0x39) sendo o clube ou o parceiro dele
/// (0x540a50) leva ao contrato com o clube; nenhum: 7.
unsafe extern "thiscall" fn estatuto_no_elenco_pelo_clube(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    if clube == 0 {
        return 7;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let mut e = entrada(this, clube, pessoa, (cpn6_d(pessoa + 0x61) == 0) as u32, 0);
    if e == 0 && cpn6_d(pessoa + 0x69) != 0 {
        e = entrada(this, clube, pessoa, 1, 0);
    }
    if e != 0 {
        if !cargo.is_null() {
            *cargo = cpn6_b(e);
        }
        crate::comparacao::ramo("cpn_6 estatuto pela lista");
        return (cpn6_b(e + 0x24) >> 1) & 0xf; // os bits 1..4 da entrada
    }
    if !cargo.is_null() {
        *cargo = cpn6_b(pessoa + 0x3d);
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpn6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let pelo_contrato = if cpn6_clube_do_contrato(c) == clube {
        true
    } else {
        // ponytail: o 3º argumento é empurrado como dword 1; o cargo vai num byte (a 0x53b620 só lê o byte)
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        let tipo_11_a_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
        if humano(clube, 1, 1, 0, 0) != 0 && tipo_11_a_15(cpn6_b(pessoa + 0x3d) as i8) != 0 {
            let estimar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(0x849d60usize);
            return estimar(pessoa, clube) as u8;
        }
        cpn6_d(pessoa + 0x39) == clube || {
            let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
            cpn6_d(pessoa + 0x39) == parceiro(clube, 0, 1)
        }
    };
    if !pelo_contrato {
        return 7;
    }
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = com_clube(cpn6_d(0xae1050), pessoa, clube);
    if k == 0 {
        return 7;
    }
    crate::comparacao::ramo("cpn_6 estatuto pelo contrato");
    cpn6_b(k + 0x4f) >> 4 // o nibble alto do contrato
}

/// { 0x849d60 } (stdcall, ret 8, shortlist_manager.cpp): `shortlist_manager_estimate_squad_status(pessoa, clube)`: o
/// estatuto que o clube humano daria. A reputação da pessoa vista pelo técnico do clube (0x53ca90, 0x5407e0; até 1:
/// 1, senão as duas chamadas de novo) e a do clube (+0x80, no mínimo 1); pessoa na lista do humano (0x7f7490 do
/// [0xb74448]): a reputação passa a ser o maior de 1,5 × ela e 0,75 × a do clube (truncado, 16 bits). A razão (limitada
/// a 0,1..2): acima de 1,5: 1; o float dela acima de 1,2 com o clube abaixo de 8500: 2; até 1,2 com a reputação até
/// 6500: 5 (acima de 0,6 e idade +0x18 abaixo de 23), 4 (24 ou mais) ou 6; senão 3. Só roda para o clube humano.
unsafe extern "stdcall" fn estimar_estatuto_no_elenco(pessoa: usize, clube: usize) -> u32 {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let reputacao: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(0x5407e0usize);
    let mut rep = if reputacao(pessoa, tecnico(clube), 0) as i16 > 1 { reputacao(pessoa, tecnico(clube), 0) as i16 } else { 1 };
    let mut rc = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    if rc <= 1 {
        rc = 1;
    }
    let t = tecnico(clube);
    let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
    if na_lista(cpn6_d(0xb74448), pessoa, t) != 0 {
        let a = rep as f64 * f64_em(0x968940);
        let b = rc as f64 * f64_em(0x969040);
        rep = ftol(if a > b { a } else { b }) as i16;
    }
    let mut q = rep as f64 / rc as f64;
    if q < f64_em(0x967880) {
        q = f64_em(0x967880);
    } else if !(q <= f64_em(0x968950)) {
        q = f64_em(0x968950);
    }
    let f = q as f32 as f64; // a original guarda a razão num float e compara o resto por ele
    if q > f64_em(0x968940) {
        return 1;
    }
    if f > f64_em(0x968978) && rc < 0x2134 {
        return 2;
    }
    let idade = cpn6_b(pessoa + 0x18) as i8;
    if !(f > f64_em(0x968978)) && rep <= 0x1964 {
        if f > f64_em(0x968e48) && idade < 0x17 {
            return 5;
        }
        return if idade >= 0x18 { 4 } else { 6 };
    }
    3
}

/// { 0x844660 } (thiscall, ret 8, shortlist_manager.cpp): `shortlist_manager_release_club_slot_targets(clube,
/// entrada)`: nas 40 vagas de 0x25 bytes de [entrada], o alvo (+0xf) acima do último id de pessoa ([0xae2364] − 1)
/// vira −1; o alvo válido cujo contrato (0x4cf5a0, guardado em [0xae1080]; o de tipo 5 em +0x35 não conta) não é do
/// clube, nem o clube dele (+0x39) é o clube, e com o +0x17 da vaga em −1: o evento 3 da lista (0x843e10) com o tipo
/// "não é 11" do +0 da vaga (no byte baixo do endereço da vaga, como a original).
unsafe extern "thiscall" fn liberar_alvos_da_lista_do_clube(this: usize, clube: usize, entrada: usize) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x843e10usize);
    for k in 0..40usize {
        let o = k * 0x25;
        let v = cpn6_d(entrada).wrapping_add(o + 0xf);
        if cpn6_d(v) as i32 > (*(0xae2364 as *const i32)).wrapping_sub(1) {
            core::ptr::write_unaligned(v as *mut i32, -1);
        }
        let id = cpn6_d(cpn6_d(entrada).wrapping_add(o + 0xf)) as i32;
        if id <= -1 {
            continue;
        }
        let p = cpn6_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let c = contrato_de(cpn6_d(0xae1050), p);
        *(0xae1080 as *mut usize) = c;
        let do_contrato = if c != 0 && cpn6_b(c + 0x35) & 0x3f != 5 { cpn6_clube_do_contrato(c) } else { 0 };
        if do_contrato == clube || cpn6_d(p + 0x39) == clube {
            continue;
        }
        let s = cpn6_d(entrada).wrapping_add(o);
        if cpn6_d(s + 0x17) as i32 != -1 {
            continue;
        }
        // setne al sobre o endereço da vaga: os 24 bits altos são os dele
        let tipo = ((s & !0xff) | (cpn6_b(s) != 0xb) as usize) as u32;
        crate::comparacao::ramo("cpn_6 alvo fora do clube");
        atualizar(this, p, clube, 3, tipo);
    }
}

/// Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
unsafe extern "thiscall" fn comparar_estatuto_no_elenco_pelo_clube(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(961);
    let saida = if cargo.is_null() { 0xae1080 } else { cargo as usize };
    let reg = [(0xae1080usize, 4usize), (saida, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, cargo), || estatuto_no_elenco_pelo_clube(this, pessoa, clube, cargo));
    PLACAR.registrar(igual && r == n, || format!("estatuto_no_elenco_pelo_clube({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
unsafe extern "stdcall" fn comparar_estimar_estatuto_no_elenco(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(962)(pessoa, clube);
    let n = estimar_estatuto_no_elenco(pessoa, clube);
    PLACAR.registrar(r as u8 == n as u8, || format!("estimar_estatuto_no_elenco({:#x}, {:#x}) original={:#x} rust={}", pessoa, clube, r, n));
    r
}

/// Comparador: só a original.
// ponytail: a 0x843e10 muda a lista do clube (e outras) a cada evento e não pode rodar duas vezes; a cópia tem o ramo
// e a prova é pela carreira (modo Rust contra a original).
unsafe extern "thiscall" fn comparar_liberar_alvos_da_lista_do_clube(this: usize, clube: usize, entrada: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(963)(this, clube, entrada)
}

// ---------- 250a leva: a busca do olheiro e os amistosos: se a busca cobre a nação, tirar o amistoso marcado e os pares de nações que não jogam amistoso (player_search.cpp, friendly.cpp) ----------

unsafe fn cpn7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpn7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7b7490 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::scout_covers_nation(nacao)`: a busca de olheiro
/// (tipo +0xa = 4) com o olheiro conferido (índice +0x180 > −1 e 0x53e990 com os nomes +0x174..+0x17c e a data +0x184)
/// e com clube (+0x39) cobre a `nacao`? Com missão marcada (0x7aefd0, que copia as missões +0x18c..+0x19d sobre elas
/// mesmas): a nação do clube da missão (+0x53 do clube +0x190), a nação da missão (+0x18c = id +0 da nação), a região
/// (+0x194, byte com sinal, = +0x76 da nação), ou com a base (+0x199) a mesma região. Sem missão: as duas nações com
/// +0x7f = 2, a mesma região (+0x76) da nação do clube, a nação não estrangeira para o olheiro (0x53d980 com 1) ou os
/// olheiros do clube cobrindo a nação (0x7f9190 do [0xb74448]). Devolve 1 ou 0.
unsafe extern "thiscall" fn busca_cobre_a_nacao(this: usize, nacao: usize) -> u32 {
    let mut pessoa = 0usize;
    let i = cpn7_d(this + 0x180) as i32;
    if i > -1 {
        let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
        let p = cpn7_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e));
        if confere(p, cpn7_d(this + 0x174), cpn7_d(this + 0x178), cpn7_d(this + 0x17c), this + 0x184) != 0 {
            pessoa = cpn7_d(0xae23bc).wrapping_add(cpn7_d(this + 0x180).wrapping_mul(0x6e));
        }
    }
    if cpn7_b(this + 0xa) != 4 || pessoa == 0 || nacao == 0 {
        return 0;
    }
    let clube = cpn7_d(pessoa + 0x39);
    if clube == 0 {
        return 0;
    }
    let missao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x7aefd0usize);
    if missao(this, this + 0x190, this + 0x18c, this + 0x194, this + 0x195, this + 0x199, this + 0x19d) != 0 {
        let c = cpn7_d(this + 0x190) as i32;
        if c > -1 && cpn7_d(cpn7_d(0xae23b4).wrapping_add((c as usize).wrapping_mul(0x245)) + 0x53) == nacao {
            return 1;
        }
        let n = cpn7_d(this + 0x18c) as i32;
        if n > -1 && cpn7_d(nacao) as i32 == n {
            return 1;
        }
        let r = cpn7_b(this + 0x194);
        if (r as i8) > -1 && cpn7_b(nacao + 0x76) == r {
            return 1;
        }
        // a original compara de novo o mesmo byte quando há base
        if cpn7_d(this + 0x199) != 0 && cpn7_b(nacao + 0x76) == r {
            return 1;
        }
        return 0;
    }
    let nc = cpn7_d(clube + 0x53);
    if cpn7_b(nc + 0x7f) == 2 && cpn7_b(nacao + 0x7f) == 2 {
        return 1;
    }
    if cpn7_b(nc + 0x76) == cpn7_b(nacao + 0x76) {
        crate::comparacao::ramo("cpn_7 mesma regiao");
        return 1; // a mesma região da nação do clube
    }
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    if estrangeira(pessoa, nacao, 1) == 0 {
        crate::comparacao::ramo("cpn_7 nacao do olheiro");
        return 1; // a nação não é estrangeira para o olheiro
    }
    let cobrem: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f9190usize);
    (cobrem(cpn7_d(0xb74448), nacao, clube) != 0) as u32
}

/// { 0x5c87b0 } (thiscall, ret 8, friendly.cpp): `Friendly::remove_friendly_match(jogo, limpar_vaga)`: tira da lista
/// [+0x34d4] cada item que aponta para o `jogo` (0x68d1f0; o cursor +0xa volta ao começo antes, depois de cada um e no
/// fim), com `limpar_vaga` zera a vaga do amistoso marcado (0x5c6740 pelo dia +0, ano +2, clube +0x14 e adversário
/// +0x10 do jogo: +0x10 = 0 e +0x18 = 0) e tira o jogo da lista [+0x34c8] (0x68d1f0, que o libera). Jogo nulo ou sem
/// a lista +0x34c8: a original (caixa "Error").
unsafe extern "thiscall" fn tirar_amistoso_marcado(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    if jogo == 0 || cpn7_d(this + 0x34c8) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(965)(this, jogo, limpar_vaga);
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let recomecar = || {
        let l = cpn7_d(this + 0x34d4);
        cpn7_por(l + 0xa, cpn7_d(l + 2));
    };
    if cpn7_d(this + 0x34d4) != 0 {
        recomecar();
        let mut e = proximo(cpn7_d(this + 0x34d4));
        while e != 0 {
            if cpn7_d(e) == jogo {
                tirar(cpn7_d(this + 0x34d4), e);
                recomecar();
            }
            e = proximo(cpn7_d(this + 0x34d4));
        }
        recomecar();
    }
    if limpar_vaga != 0 {
        // ponytail: a original empurra dia e ano em registradores com os 16 bits altos de outros valores; a 0x5c6740 só
        // lê os 16 bits baixos
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        let v = vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, cpn7_d(jogo + 0x14), cpn7_d(jogo + 0x10));
        if v != 0 {
            cpn7_por(v + 0x10, 0);
            *((v + 0x18) as *mut u8) = 0;
        }
    }
    crate::comparacao::ramo("cpn_7 tirar o amistoso");
    tirar(cpn7_d(this + 0x34c8), jogo) // tira o jogo da lista dos amistosos
}

/// { 0x5cc2d0 } (thiscall, friendly.cpp): `Friendly::init_banned_nation_pairs()`: +0x3506 = 4 pares e +0x3502 um vetor
/// novo de 0x20 bytes (malloc 0x944e46) com os pares de nações que não jogam amistoso entre si: Irã e EUA, Iraque e
/// qualquer uma (−1), Vietnã e EUA, as duas Coreias (as chaves [0x9cf354], [0x9cf4f8], [0x9cf358], [0x9cf508],
/// [0x9cf404], [0x9cf484]). Sem memória: a caixa "Error" (linha 0x1800) e +0x3506 = 0.
unsafe extern "thiscall" fn iniciar_pares_proibidos_de_amistoso(this: usize) {
    *((this + 0x3506) as *mut u8) = 4;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x20);
    cpn7_por(this + 0x3502, v);
    if v == 0 {
        aviso_em(0x9cd0a0, 0x1800);
        *((this + 0x3506) as *mut u8) = 0;
        return;
    }
    for (k, chave) in [0x9cf354usize, 0x9cf4f8, 0x9cf358, 0, 0x9cf508, 0x9cf4f8, 0x9cf404, 0x9cf484].into_iter().enumerate() {
        cpn7_por(cpn7_d(this + 0x3502) + 4 * k, if chave == 0 { usize::MAX } else { cpn7_d(chave) });
    }
}

/// Comparador: lado a lado pelo valor e pelas missões +0x18c..+0x1a0 (a 0x7aefd0 as copia sobre elas mesmas).
unsafe extern "thiscall" fn comparar_busca_cobre_a_nacao(this: usize, nacao: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(964);
    let reg = [(this + 0x18c, 0x15)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || busca_cobre_a_nacao(this, nacao));
    PLACAR.registrar(igual && r == n, || format!("busca_cobre_a_nacao(nacao {:#x}) original={} rust={} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparador: a 0x68d1f0 libera o que tira: o plano antes (os itens que ficam nas duas listas e a vaga do amistoso);
/// a original roda; confere as listas, o cursor da +0x34d4 no começo e a vaga zerada. Jogo nulo ou sem a lista
/// +0x34c8: só a original (a caixa).
unsafe extern "thiscall" fn comparar_tirar_amistoso_marcado(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(965);
    if jogo == 0 || cpn7_d(this + 0x34c8) == 0 {
        return o(this, jogo, limpar_vaga);
    }
    // ponytail: a 0x68d1f0 acha o nó pelo conteúdo do item; o plano supõe itens de conteúdo distinto (o normal)
    let l4 = cpn7_d(this + 0x34d4);
    let esperado4: Vec<usize> = if l4 != 0 { cpd1_itens(l4).into_iter().filter(|&e| cpn7_d(e) != jogo).collect() } else { Vec::new() };
    let mut esperado8 = cpd1_itens(cpn7_d(this + 0x34c8));
    if let Some(k) = esperado8.iter().position(|&e| e == jogo) {
        esperado8.remove(k);
    }
    let v = if limpar_vaga != 0 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, cpn7_d(jogo + 0x14), cpn7_d(jogo + 0x10))
    } else {
        0
    };
    let r = o(this, jogo, limpar_vaga);
    let l4 = cpn7_d(this + 0x34d4);
    let itens4 = if l4 != 0 { cpd1_itens(l4) } else { Vec::new() };
    let itens8 = cpd1_itens(cpn7_d(this + 0x34c8));
    let ok = itens4 == esperado4
        && (l4 == 0 || cpn7_d(l4 + 0xa) == cpn7_d(l4 + 2))
        && itens8 == esperado8
        && (v == 0 || (cpn7_d(v + 0x10) == 0 && cpn7_b(v + 0x18) == 0));
    PLACAR.registrar(ok, || format!("tirar_amistoso_marcado({:#x}, {}) itens {} e {} (esperava {} e {}), vaga {:#x}", jogo, limpar_vaga, itens4.len(), itens8.len(), esperado4.len(), esperado8.len(), v));
    r
}

/// Comparador: o malloc não pode rodar duas vezes: a original roda e confere o vetor e a conta com o plano.
unsafe extern "thiscall" fn comparar_iniciar_pares_proibidos_de_amistoso(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(966)(this);
    let v = cpn7_d(this + 0x3502);
    let ok = if v == 0 {
        cpn7_b(this + 0x3506) == 0
    } else {
        let esperado = [cpn7_d(0x9cf354), cpn7_d(0x9cf4f8), cpn7_d(0x9cf358), usize::MAX, cpn7_d(0x9cf508), cpn7_d(0x9cf4f8), cpn7_d(0x9cf404), cpn7_d(0x9cf484)];
        cpn7_b(this + 0x3506) == 4 && (0..8).all(|k| cpn7_d(v + 4 * k) == esperado[k])
    };
    PLACAR.registrar(ok, || format!("iniciar_pares_proibidos_de_amistoso: conta {} vetor {:#x}", cpn7_b(this + 0x3506), v));
}
