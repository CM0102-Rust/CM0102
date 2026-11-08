// ---------- %%LEVA%%a leva: a entrega das notícias: pelo alcance, ao técnico do clube e na caixa do técnico humano (news.cpp) ----------

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
unsafe fn news_human_manager_index(tecnico: usize) -> i32 {
    cpn1_i(tecnico).wrapping_sub(cpn1_i(0xae2364)).wrapping_add(0x10)
}

/// { 0x786090 } (thiscall, ret 0x18, news.cpp): `News::send_by_scope(noticia, clube, competicao, nacao, continente,
/// alcance)`: o alcance (char) escolhe a entrega: 0 o técnico do clube (0x7861e0), 1 os da divisão (0x7862d0, pela
/// competição), 2 os do país (0x7863f0, pela nação), 3 os do continente (0x786640), 4 todos (0x786780), 5 os da região
/// da competição (0x786860). Fora disso, a caixa "Error" e 0. Devolve o que a entrega devolve.
unsafe extern "thiscall" fn send_by_scope(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
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
            show_error_at(0xa693ec, 0x1fd3);
            return 0;
        }
    };
    dois(this, noticia, alvo)
}

/// { 0x7861e0 } (thiscall, ret 8, news.cpp): `News::send_to_club_manager(noticia, clube)`: marca o alcance da notícia
/// como clube (+0xde = 0) e a entrega (0x785d40) ao técnico do clube (+0xcf) se ele for humano (id entre as 16 últimas
/// pessoas). Sem notícia ou sem clube: a caixa "Error" e 0; sem técnico ou técnico da máquina: 0.
unsafe extern "thiscall" fn send_to_club_manager(this: usize, noticia: usize, clube: usize) -> u32 {
    if noticia == 0 {
        show_error_at(0xa693ec, 0x2026);
        return 0;
    }
    if clube == 0 {
        show_error_at(0xa693ec, 0x202e);
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
unsafe extern "thiscall" fn deliver_to_manager(this: usize, noticia: usize, tecnico: usize, _livre: u32) -> u32 {
    if noticia == 0 {
        show_error_at(0xa693ec, 0x1f3f);
        return 0;
    }
    if tecnico == 0 {
        show_error_at(0xa693ec, 0x1f47);
        return 0;
    }
    let importante: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x784eb0usize);
    *((noticia + 4) as *mut u8) = importante(noticia, tecnico) as u8;
    cpn1_por(noticia + 0xda, 0);
    let idx = news_human_manager_index(tecnico);
    if idx < 0 {
        show_error_at(0xa693ec, 0x1f58);
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
            show_error_at(0xa693ec, 0x1f6d);
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
        let respond: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x784100usize);
        crate::comparacao::ramo("cpn_1 resposta automatica");
        respond(vaga(atual), tecnico, cpn1_d(tecnico + 0x39)); // o clube
        let selecao = cpn1_d(tecnico + 0x24);
        if selecao != 0 {
            respond(vaga(atual), tecnico, selecao);
        }
    }
    cpn1_por(e + 8, cpn1_d(e + 8).wrapping_add(1));
    1
}

/// Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
unsafe extern "thiscall" fn comparar_send_by_scope(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(%%I0%%)(this, noticia, clube, competicao, nacao, continente, alcance);
    let fora = !(0..=5).contains(&(alcance as u8 as i8));
    PLACAR.registrar(!fora || r == 0, || format!("send_by_scope alcance {} devolveu {}", alcance as u8 as i8, r));
    r
}

/// Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
unsafe extern "thiscall" fn comparar_send_to_club_manager(this: usize, noticia: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if noticia == 0 || clube == 0 {
        return o(this, noticia, clube);
    }
    let tecnico = cpn1_d(clube + 0xcf);
    let humano = tecnico != 0 && cpn1_i(tecnico) >= cpn1_i(0xae2364).wrapping_sub(0x10);
    let r = o(this, noticia, clube);
    let ok = *((noticia + 0xde) as *const u8) == 0 && (humano || r == 0) && r <= 1;
    PLACAR.registrar(ok, || format!("send_to_club_manager(clube {:x}, técnico {:x}, humano {}) devolveu {}", clube, tecnico, humano, r));
    r
}

/// Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
unsafe extern "thiscall" fn comparar_deliver_to_manager(this: usize, noticia: usize, tecnico: usize, livre: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I2%%);
    if noticia == 0 || tecnico == 0 || news_human_manager_index(tecnico) < 0 {
        return o(this, noticia, tecnico, livre);
    }
    let idx = news_human_manager_index(tecnico) as usize;
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
        PLACAR.registrar(novo, || format!("deliver_to_manager devolveu 0 sem anel novo")); // só sem memória
        return r;
    }
    let vaga = cpn1_d(this).wrapping_add((atual as usize).wrapping_mul(CPN1_NOTICIA));
    let ok = r == 1 && cpn1_i(e) == inicio && cpn1_i(e + 4) == prox && cpn1_i(e + 8) == conta_depois
        && cpn1_d(this + 0x10) == contador_depois && cpn1_d(noticia + 0xd6) == id && cpn1_d(noticia + 0xda) == 0
        && (repetida || cpn1_d(vaga + 0xd6) == id);
    PLACAR.registrar(ok, || format!("deliver_to_manager(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, cpn1_d(noticia + 0xd6), id, cpn1_i(e), cpn1_i(e + 4), cpn1_i(e + 8), inicio, prox, conta_depois, repetida));
    r
}
