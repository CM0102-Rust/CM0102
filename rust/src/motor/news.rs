use super::*;

// ---------- 251a leva: a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id (media.cpp, news.cpp) ----------





/// O tipo do contrato (+0x4f, nibble alto).
pub(crate) unsafe fn cpo6_tipo(contrato: usize) -> u8 {
    *((contrato + contract::SQUAD_STATUS) as *const u8) & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
pub(crate) const CPO6_NOTICIA: usize = 0xdf;


/// marks read the first unread item with the id in the human's ring unless it needs a reply (button 0x782b70), takes it
/// off the count and, when asked, repeats it for the humans on the same network machine
/// Original: `News::mark_item_read` (0x787140, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x787140 } (thiscall, ret 0xc, news.cpp): `News::mark_item_read(tecnico, id, sincronizar)`: no anel de 100 vagas
/// do humano (News +8, pelo id − [0xae2364] + 16), a primeira vaga com o `id` ainda não lida (+0xda = 0) cujo botão
/// (0x782b70 em 0xde1f64) não pede resposta (2) fica lida e sai da conta (+8 da entrada). Sem anel: 0. Senão (mesmo
/// sem achar), com `sincronizar`, repete (sem sincronizar) para os outros humanos (+0x5f = 1) da mesma máquina da rede
/// (0x7fffa0) que o humano da vez; devolve 1. Sem técnico, sem id ou técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2358);
        return 0;
    }
    if id == 0 {
        aviso_em(0xa693ec, 0x2360);
        return 0;
    }
    let idx = read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, 0x236b);
        return 0;
    }
    let e = read_usize(this + 8) + idx as usize * 12;
    if read_i32(e) == -1 {
        return 0;
    }
    let botao: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8 = core::mem::transmute(FN_NEWSITEM_GET_ACTION_BUTTON);
    let mut v = 0usize;
    for k in 0..0x64usize {
        v = read_usize(this).wrapping_add((read_usize(e).wrapping_add(k)).wrapping_mul(CPO6_NOTICIA));
        if read_usize(v + 0xd6) as u32 == id && read_usize(v + 0xda) == 0 {
            let mut tratador = 0u32;
            if botao(v, tecnico, 0xde1f64, 0x7d0, &mut tratador) != 2 {
                crate::comparacao::ramo("cpo_6 noticia lida");
                core::ptr::write_unaligned((v + 0xda) as *mut u32, 1);
                core::ptr::write_unaligned((e + 8) as *mut i32, read_i32(e + 8) - 1);
                break;
            }
        }
    }
    if v == 0 {
        return 0;
    }
    if sincronizar != 0 {
        let maquina: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCREENMANAGER_FIND_MANAGER_CHANNEL);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_MARK_ITEM_READ);
        let da_vez = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize);
        let m = maquina(0xb74450, da_vez) as u16;
        for j in 0..0x10i32 {
            let p = read_usize(G_STAFF_TABLE).wrapping_add((read_i32(0xae2364) - j - 1).wrapping_mul(0x6e) as usize);
            if *((p + 0x5f) as *const u8) == 1 && p != tecnico && maquina(0xb74450, p) as u16 == m {
                marcar(this, p, id, 0);
            }
        }
    }
    1
}

/// copies the item with the id to the output: in the human manager's 100-slot ring or, without a manager, among all
/// slots
/// Original: `News::find_item_by_id` (0x786b70, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x786b70 } (thiscall, ret 0xc, news.cpp): `News::find_item_by_id(tecnico, id, saida)`: copia (0x786010) em `saida`
/// a notícia com o `id`: com técnico, entre as 100 vagas do anel dele (sem anel: 0); sem técnico, entre todas as vagas
/// (News +4). Achou: 1; senão 0. Sem id, sem saída ou técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    if id == 0 {
        aviso_em(0xa693ec, 0x2252);
        return 0;
    }
    if saida == 0 {
        aviso_em(0xa693ec, 0x225a);
        return 0;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_NEWSITEM_OPERATOR_3D);
    let (inicio, n) = if tecnico == 0 {
        (0usize, read_i32(this + 4).max(0) as usize)
    } else {
        let idx = read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10);
        if idx < 0 {
            aviso_em(0xa693ec, 0x2268);
            return 0;
        }
        let ini = read_i32(read_usize(this + 8) + idx as usize * 12);
        if ini == -1 {
            return 0;
        }
        (ini as usize, 0x64)
    };
    for k in 0..n {
        let v = read_usize(this).wrapping_add(inicio.wrapping_add(k).wrapping_mul(CPO6_NOTICIA));
        if read_usize(v + 0xd6) as u32 == id {
            copiar(saida, v);
            return 1;
        }
    }
    0
}


/// Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn comparar_marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_MARCAR_NOTICIA_LIDA);
    if tecnico == 0 || id == 0 || read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10) < 0 || read_usize(this) == 0 {
        return o(this, tecnico, id, sincronizar);
    }
    let regioes = [
        (read_usize(this), read_i32(this + 4).max(0) as usize * CPO6_NOTICIA),
        (read_usize(this + 8), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico, id, sincronizar), || marcar_noticia_lida(this, tecnico, id, sincronizar));
    PLACAR.registrar(igual && r == n, || format!("marcar_noticia_lida(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn comparar_achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_ACHAR_NOTICIA_PELO_ID);
    if id == 0 || saida == 0 || (tecnico != 0 && read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10) < 0) {
        return o(this, tecnico, id, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, CPO6_NOTICIA)], || o(this, tecnico, id, saida), || achar_noticia_pelo_id(this, tecnico, id, saida));
    PLACAR.registrar(igual && r == n, || format!("achar_noticia_pelo_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}

// ---------- 250a leva: a entrega das notícias: pelo alcance, ao técnico do clube e na caixa do técnico humano (news.cpp) ----------




/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +0xd6 id, +0xda livre/lida, +0xde alcance).
pub(crate) const CPN1_NOTICIA: usize = 0xdf;

/// A entrada do técnico humano na tabela das caixas (News +8, 12 bytes: início do anel, a próxima vaga, a conta):
/// índice = id da pessoa − [0xae2364] + 16 (os humanos são as 16 últimas pessoas). Negativo: não é humano.
pub(crate) unsafe fn cpn1_indice_humano(tecnico: usize) -> i32 {
    read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10)
}

/// delivers the news item by scope: 0 the club manager, 1 the division's managers, 2 the country's, 3 the continent's,
/// 4 all, 5 the competition region's; anything else shows the "Error" box
/// Original: `News::send_by_scope` (0x786090, news.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x786090 } (thiscall, ret 0x18, news.cpp): `News::send_by_scope(noticia, clube, competicao, nacao, continente,
/// alcance)`: o alcance (char) escolhe a entrega: 0 o técnico do clube (0x7861e0), 1 os da divisão (0x7862d0, pela
/// competição), 2 os do país (0x7863f0, pela nação), 3 os do continente (0x786640), 4 todos (0x786780), 5 os da região
/// da competição (0x786860). Fora disso, a caixa "Error" e 0. Devolve o que a entrega devolve.
pub(crate) unsafe extern "thiscall" fn noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let dois: unsafe extern "thiscall" fn(usize, usize, usize) -> u32;
    let alvo = match alcance as u8 as i8 {
        0 => {
            crate::comparacao::ramo("cpn_1 alcance do clube");
            dois = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
            clube // o técnico do clube
        }
        1 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
            competicao
        }
        2 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
            nacao
        }
        3 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
            continente
        }
        4 => {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
            return todos(this, noticia);
        }
        5 => {
            dois = core::mem::transmute(FN_NEWS_SEND_TO_COMP_REGION);
            competicao
        }
        _ => {
            aviso_em(0xa693ec, 0x1fd3);
            return 0;
        }
    };
    dois(this, noticia, alvo)
}

/// marks the scope as club (+0xde = 0) and delivers the item to the club manager (+0xcf) when he is human (among the
/// last 16 people); in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later
/// original reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `News::send_to_club_manager` (0x7861e0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7861e0 } (thiscall, ret 8, news.cpp): `News::send_to_club_manager(noticia, clube)`: marca o alcance da notícia
/// como clube (+0xde = 0) e a entrega (0x785d40) ao técnico do clube (+0xcf) se ele for humano (id entre as 16 últimas
/// pessoas). Sem notícia ou sem clube: a caixa "Error" e 0; sem técnico ou técnico da máquina: 0.
pub(crate) unsafe extern "thiscall" fn noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x2026);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa693ec, 0x202e);
        return 0;
    }
    let tecnico = read_usize(clube + club::MANAGER);
    *((noticia + 0xde) as *mut u8) = 0;
    if tecnico == 0 {
        return 0;
    }
    if read_i32(tecnico) < read_i32(0xae2364).wrapping_sub(0x10) { // a máquina
        return 0;
    }
    crate::comparacao::ramo("cpn_1 noticia ao tecnico humano");
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    entregar(this, noticia, tecnico, 0)
}

/// copies the item into the human's 100-slot ring (created on the first, reallocating the array), gives it an id, skips
/// a repeat of the last one and, with the manager on holiday, applies the automatic reply for the club and the national
/// team; in DESLIGADAS since 2026-10-29 (the detour, even passing through, changes the stack residue a later original
/// reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `News::deliver_to_manager` (0x785d40, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x785d40 } (thiscall, ret 0xc, news.cpp): `News::deliver_to_manager(noticia, tecnico, _)`: grava se a notícia é
/// importante para ele (+4, 0x784eb0) e zera +0xda; acha a entrada do humano (News +8); na primeira notícia dele, o anel
/// de 100 vagas no fim do vetor das notícias (News +0, conta de vagas em +4, realloc 0x945667; sem memória, caixa e 0)
/// com as vagas zeradas (+0xd6 = 0, +0xda = 1). Notícia sem id (+0xd6) recebe o próximo (News +0x10). Se a vaga anterior
/// tem o mesmo id, não repete (devolve 1). Senão copia a notícia (0x786010) na próxima vaga (a vaga ainda não lida, +0xda
/// = 0, sai da conta), avança o anel e, com todos de férias ([0xdd6bec]) ou o técnico de férias (0x5fd210), dá a
/// resposta automática (0x784100) pelo clube (+0x39) e pela seleção (+0x24). Soma 1 na conta e devolve 1.
pub(crate) unsafe extern "thiscall" fn entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, _livre: u32) -> u32 {
    if noticia == 0 {
        aviso_em(0xa693ec, 0x1f3f);
        return 0;
    }
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x1f47);
        return 0;
    }
    let importante: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWSITEM_IS_IMPORTANT_FOR);
    *((noticia + 4) as *mut u8) = importante(noticia, tecnico) as u8;
    write_usize(noticia + 0xda, 0);
    let idx = cpn1_indice_humano(tecnico);
    if idx < 0 {
        aviso_em(0xa693ec, 0x1f58);
        return 0;
    }
    let e = read_usize(this + 8) + idx as usize * 12;
    if read_i32(e) == -1 {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        write_usize(e, read_usize(this + 4));
        write_usize(e + 4, read_usize(this + 4));
        let total = read_usize(this + 4).wrapping_add(0x64);
        write_usize(this + 4, total);
        let p = realocar(read_usize(this), total.wrapping_mul(CPN1_NOTICIA) as u32);
        write_usize(this, p);
        if p == 0 {
            aviso_em(0xa693ec, 0x1f6d);
            return 0;
        }
        for k in 0..0x64usize {
            let v = read_usize(this) + (read_usize(e) + k).wrapping_mul(CPN1_NOTICIA);
            write_usize(v + 0xd6, 0);
            write_usize(v + 0xda, 1);
        }
    }
    if read_usize(noticia + 0xd6) == 0 {
        let id = read_usize(this + 0x10).wrapping_add(1);
        write_usize(this + 0x10, id);
        write_usize(noticia + 0xd6, id);
    }
    let atual = read_i32(e + 4);
    let mut anterior = atual.wrapping_sub(1);
    if anterior < read_i32(e) {
        anterior = read_i32(e) + 0x63;
    }
    let vaga = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
    if read_usize(vaga(anterior) + 0xd6) == read_usize(noticia + 0xd6) {
        return 1; // a mesma notícia de novo
    }
    if read_usize(vaga(atual) + 0xda) == 0 {
        write_usize(e + 8, read_usize(e + 8).wrapping_sub(1));
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWSITEM_OPERATOR_3D);
    copiar(vaga(atual), noticia);
    let atual = read_i32(e + 4);
    let inicio = read_i32(e);
    write_usize(e + 4, atual.wrapping_add(1) as usize);
    if atual.wrapping_add(1) == inicio.wrapping_add(0x64) {
        write_usize(e + 4, inicio as usize);
    }
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    if *(G_NEWS_SCREEN_AUTO_OPEN_BLOCKED_FLAG as *const i16) != 0 || ferias(read_usize(G_HUMAN_MANAGER), tecnico) != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWSITEM_AUTO_RESPOND);
        crate::comparacao::ramo("cpn_1 resposta automatica");
        responder(vaga(atual), tecnico, read_usize(tecnico + 0x39)); // o clube
        let selecao = read_usize(tecnico + 0x24);
        if selecao != 0 {
            responder(vaga(atual), tecnico, selecao);
        }
    }
    write_usize(e + 8, read_usize(e + 8).wrapping_add(1));
    1
}

/// Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_pelo_alcance(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(IDX_NOTICIA_PELO_ALCANCE)(this, noticia, clube, competicao, nacao, continente, alcance);
    let fora = !(0..=5).contains(&(alcance as u8 as i8));
    PLACAR.registrar(!fora || r == 0, || format!("noticia_pelo_alcance alcance {} devolveu {}", alcance as u8 as i8, r));
    r
}

/// Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_ao_tecnico_do_clube(this: usize, noticia: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_AO_TECNICO_DO_CLUBE);
    if noticia == 0 || clube == 0 {
        return o(this, noticia, clube);
    }
    let tecnico = read_usize(clube + club::MANAGER);
    let humano = tecnico != 0 && read_i32(tecnico) >= read_i32(0xae2364).wrapping_sub(0x10);
    let r = o(this, noticia, clube);
    let ok = *((noticia + 0xde) as *const u8) == 0 && (humano || r == 0) && r <= 1;
    PLACAR.registrar(ok, || format!("noticia_ao_tecnico_do_clube(clube {:x}, técnico {:x}, humano {}) devolveu {}", clube, tecnico, humano, r));
    r
}

/// Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
pub(crate) unsafe extern "thiscall" fn comparar_entregar_noticia_ao_tecnico(this: usize, noticia: usize, tecnico: usize, livre: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_ENTREGAR_NOTICIA_AO_TECNICO);
    if noticia == 0 || tecnico == 0 || cpn1_indice_humano(tecnico) < 0 {
        return o(this, noticia, tecnico, livre);
    }
    let idx = cpn1_indice_humano(tecnico) as usize;
    let e = read_usize(this + 8) + idx * 12;
    // o plano
    let novo = read_i32(e) == -1;
    let (inicio, atual, conta) = if novo { (read_i32(this + 4), read_i32(this + 4), read_i32(e + 8)) } else { (read_i32(e), read_i32(e + 4), read_i32(e + 8)) };
    let contador = read_usize(this + 0x10);
    let (id, contador_depois) = if read_usize(noticia + 0xd6) == 0 { (contador.wrapping_add(1), contador.wrapping_add(1)) } else { (read_usize(noticia + 0xd6), contador) };
    let mut anterior = atual - 1;
    if anterior < inicio {
        anterior = inicio + 0x63;
    }
    let (repetida, livre_atual) = if novo {
        (id == 0, true)
    } else {
        let v = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN1_NOTICIA));
        (read_usize(v(anterior) + 0xd6) == id, read_usize(v(atual) + 0xda) != 0)
    };
    let (prox, conta_depois) = if repetida {
        (atual, conta)
    } else {
        let p = if atual + 1 == inicio + 0x64 { inicio } else { atual + 1 };
        (p, conta + if livre_atual { 1 } else { 0 })
    };
    // com o técnico de férias a resposta automática (0x784100) pode entregar outra notícia ao mesmo humano: o anel anda
    // mais vagas e a conta junto (achado em 31/10/2026, quando a rotina saiu das DESLIGADAS)
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let responde = !repetida && (*(G_NEWS_SCREEN_AUTO_OPEN_BLOCKED_FLAG as *const i16) != 0 || ferias(read_usize(G_HUMAN_MANAGER), tecnico) != 0);
    let r = o(this, noticia, tecnico, livre);
    if r == 0 {
        PLACAR.registrar(novo, || format!("entregar_noticia_ao_tecnico devolveu 0 sem anel novo")); // só sem memória
        return r;
    }
    let extra = if responde { (read_i32(e + 4) - prox).rem_euclid(0x64) } else { 0 };
    let vaga = read_usize(this).wrapping_add((atual as usize).wrapping_mul(CPN1_NOTICIA));
    let ok = r == 1 && read_i32(e) == inicio && read_i32(e + 4) == if extra == 0 { prox } else { read_i32(e + 4) }
        && read_i32(e + 8) >= conta_depois && read_i32(e + 8) <= conta_depois + extra
        && (read_usize(this + 0x10) == contador_depois || (extra > 0 && read_usize(this + 0x10) > contador_depois))
        && read_usize(noticia + 0xd6) == id && read_usize(noticia + 0xda) == 0
        && (repetida || read_usize(vaga + 0xd6) == id);
    PLACAR.registrar(ok, || format!("entregar_noticia_ao_tecnico(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, read_usize(noticia + 0xd6), id, read_i32(e), read_i32(e + 4), read_i32(e + 8), inicio, prox, conta_depois, repetida));
    r
}

// ---------- 250a leva: a caixa do técnico humano: responder a todas as notícias, se há notícia pendente e o texto num campo da notícia (news.cpp) ----------



/// A notícia guardada na caixa: 0xdf bytes (+4 importante, +5 os 50 campos de 4 bytes, +0xcc fim do texto, +0xd6 id,
/// +0xda lida).
pub(crate) const CPN2_NOTICIA: usize = 0xdf;

/// A entrada do humano na tabela das caixas (News +8, 12 bytes: início do anel, próxima vaga, conta), ou None (com a
/// caixa "Error" da `linha`) se a pessoa não está entre as 16 últimas.
pub(crate) unsafe fn cpn2_entrada(this: usize, tecnico: usize, linha: u32) -> Option<usize> {
    let idx = read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, linha);
        return None;
    }
    Some(read_usize(this + 8) + idx as usize * 12)
}

/// A vaga anterior a `k` no anel que começa em `inicio` (100 vagas).
pub(crate) fn cpn2_volta(k: i32, inicio: i32) -> i32 {
    if k < inicio { inicio + 0x63 } else { k }
}

/// from the newest to the oldest item of the human's inbox, up to the empty slot, applies the automatic reply
/// (0x784100) for the club
/// Original: `News::auto_respond_all` (0x7874b0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7874b0 } (thiscall, ret 8, news.cpp): `News::auto_respond_all(tecnico, clube)`: da notícia mais nova para a mais
/// antiga do anel do humano, até a primeira vaga vazia (+0xd6 = 0), dá a resposta automática (0x784100) pelo `clube`.
/// Sem técnico ou técnico da máquina: a caixa "Error" e 0; sem anel ou no fim: 1. Chamada ao sair de férias, ao ser
/// demitido e ao ser contratado.
pub(crate) unsafe extern "thiscall" fn responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2460);
        return 0;
    }
    let e = match cpn2_entrada(this, tecnico, 0x246b) {
        Some(e) => e,
        None => return 0,
    };
    if read_i32(e) == -1 {
        return 1;
    }
    let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWSITEM_AUTO_RESPOND);
    let mut k = read_i32(e + 4) - 1;
    loop {
        k = cpn2_volta(k, read_i32(e));
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if read_usize(vaga + 0xd6) == 0 {
            return 1;
        }
        crate::comparacao::ramo("cpn_2 responder todas");
        responder(vaga, tecnico, clube); // a resposta automática
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return 1;
        }
    }
}

/// without "urgent only", whether there is an unread item; with it, whether an item needs a reply (the text returns 2)
/// or is important and unread
/// Original: `News::has_urgent_items` (0x787350, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x787350 } (thiscall, ret 8, news.cpp): `News::has_urgent_items(tecnico, so_urgentes)`: sem `so_urgentes`, se há
/// notícia não lida (0x786fe0 > 0). Com ele, da mais nova para a mais antiga do anel do humano, até a primeira vaga vazia:
/// 1 se uma pede resposta (o texto 0x782b70 em 0xde1f64 devolve 2) ou é importante (+4 = 1) e não lida (+0xda = 0).
/// Técnico da máquina: a caixa "Error" e 0.
pub(crate) unsafe extern "thiscall" fn tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    if so_urgentes == 0 {
        let nao_lidas: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_NEWS_COUNT_UNREAD);
        return (nao_lidas(this, tecnico, 0) > 0) as u32;
    }
    let e = match cpn2_entrada(this, tecnico, 0x241e) {
        Some(e) => e,
        None => return 0,
    };
    if read_i32(e) == -1 {
        return 0;
    }
    let texto: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_ACTION_BUTTON);
    let mut k = read_i32(e + 4) - 1;
    loop {
        k = cpn2_volta(k, read_i32(e));
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if read_usize(vaga + 0xd6) == 0 {
            return 0;
        }
        let mut saida = 0u32;
        if texto(vaga, tecnico, 0xde1f64, 0x7d0, &mut saida) as u8 == 2 {
            return 1; // pede resposta
        }
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if *((vaga + 4) as *const u8) == 1 && read_usize(vaga + 0xda) == 0 {
            return 1; // importante e não lida
        }
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return 0;
        }
    }
}

/// copies the text into the item slots from the given slot and returns the first free slot after it (at most 50)
/// Original: `NewsItem::set_slot_string` (0x7858c0, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7858c0 } (thiscall, ret 0xc, news.cpp): `NewsItem::set_slot_string(campo, texto, proximo)`: copia o `texto` nos
/// campos da notícia a partir do `campo` (strncpy 0x9466d0 em +5 + 4·campo, até 200 − 4·campo bytes), termina a área
/// (+0xcc = 0) e, com `proximo`, grava o primeiro campo livre depois do texto (o tamanho com o zero arredondado para
/// 4, em campos, mais o `campo`; no máximo 50). Campo fora de 0..49 ou sem texto: a caixa "Error" e 0; senão 1.
pub(crate) unsafe extern "thiscall" fn gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let campo = campo as u8 as i8;
    if !(0..0x32).contains(&campo) {
        aviso_em(0xa693ec, 0x1e8d);
        return 0;
    }
    if texto.is_null() {
        aviso_em(0xa693ec, 0x1e95);
        return 0;
    }
    let copiar: unsafe extern "C" fn(usize, *const u8, u32) -> usize = core::mem::transmute(FN_9466D0);
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
pub(crate) unsafe extern "thiscall" fn comparar_responder_todas_as_noticias(this: usize, tecnico: usize, clube: usize) -> u32 {
    let humano = tecnico != 0 && read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10) >= 0;
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_RESPONDER_TODAS_AS_NOTICIAS)(this, tecnico, clube);
    PLACAR.registrar(r == humano as u32, || format!("responder_todas_as_noticias(técnico {:x}, humano {}) devolveu {}", tecnico, humano, r));
    r
}

/// Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
pub(crate) unsafe extern "thiscall" fn comparar_tecnico_tem_noticia_pendente(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_TECNICO_TEM_NOTICIA_PENDENTE);
    if so_urgentes != 0 && read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico, so_urgentes), || tecnico_tem_noticia_pendente(this, tecnico, so_urgentes));
    PLACAR.registrar(igual && r == n, || format!("tecnico_tem_noticia_pendente(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
pub(crate) unsafe extern "thiscall" fn comparar_gravar_texto_na_noticia(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(IDX_GRAVAR_TEXTO_NA_NOTICIA);
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

// ---------- 247a leva: as notícias para os técnicos humanos de um continente e da região da competição, e se a notícia é importante para a pessoa ----------


/// continent scope (+0xde = 3) and delivery to every active human manager whose club or nation is in the continent
/// Original: `News::send_to_continent_managers` (0x786640, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786640 } (thiscall, ret 8, news.cpp): `News::send_to_continent_managers(noticia, continente)`: alcance do
/// continente (+0xde = 3) e a entrega (0x785d40 com 0) a cada técnico humano ativo cujo clube (+0x39) tem a nação
/// (+0x53) no `continente` (+0x71) ou cuja nação (+0x1a) está nele. Devolve 1, ou 0 na primeira recusada. Notícia ou
/// continente nulos: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    if noticia == 0 || continente == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DO_CONTINENTE)(this, noticia, continente); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 3, |p| {
        let c = read_usize(p + 0x39);
        if c != 0 {
            let n = read_usize(c + 0x53);
            if n != 0 && read_usize(n + 0x71) == continente { // o continente da nação do clube
                crate::comparacao::ramo("cpk_5 continente pelo clube");
                return true;
            }
        }
        let s = read_usize(p + 0x1a);
        s != 0 && read_usize(s + 0x71) == continente // o continente da nação da pessoa
    }) as u32
}

/// to the managers of the competition's nation (0x786520), else of its continent (0x786640), else to all (0x786780)
/// Original: `News::send_to_comp_region` (0x786860, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786860 } (thiscall, ret 8, news.cpp): `News::send_to_comp_region(noticia, comp)`: com a nação da competição
/// (+0x5d), para os técnicos da nação (0x786520); senão, com o continente (+0x59), para os do continente (0x786640);
/// senão para todos (0x786780). Devolve o que a entrega devolve. Notícia ou competição nulas: a original (caixa
/// "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    if noticia == 0 || comp == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_A_REGIAO_DA_COMPETICAO)(this, noticia, comp); // caixa: só a original
    }
    let nacao = read_usize(comp + competition::CLUB_COMP_NATION);
    if nacao != 0 {
        crate::comparacao::ramo("cpk_5 noticia da nacao da competicao");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_CLUB_MANAGERS);
        return f(this, noticia, nacao);
    }
    let continente = read_usize(comp + competition::CLUB_COMP_CONTINENT);
    if continente != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
        return f(this, noticia, continente);
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    f(this, noticia)
}

/// O técnico (+0xcf) do clube cujo id está no dado da notícia em `o` ([0xae23b4], 0x245 bytes por clube).
pub(crate) unsafe fn cpk5_tecnico_do_clube(noticia: usize, o: usize) -> usize {
    read_usize(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(noticia + o).wrapping_mul(0x245)) + 0xcf)
}

/// whether the news is important for the person, by category and type (bids on the person's club, the humans...)
/// Original: `NewsItem::is_important_for` (0x784eb0, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x784eb0 } (thiscall, ret 4, news.cpp): `NewsItem::is_important_for(pessoa)`: a notícia é importante para a
/// pessoa (o byte +4 que a entrega 0x785d40 grava)? Pela categoria (0x785670) e o tipo (+0): categoria 2, os tipos 7 a
/// 0xb, 0x27 e 0x28; categoria 4, 0xbb8, 0xbb9 e 0xbc4; categoria 5, uma lista fixa, 0xfa1 se a pessoa é o técnico do
/// clube do dado 5 (+0x19) e 0xfb6 se é o do dado 4 (+0x15) ou do 5; categoria 6, 0x1770 se o dado 0 (+5) é um dos
/// 16 técnicos humanos; categoria 1, 0x1388; o resto, 0. A original chama a 0x785670 até seis vezes (só lê): a cópia,
/// uma. Pessoa nula ou tipo fora de 1..0x1790 (caixa da 0x785670): a original.
pub(crate) unsafe extern "thiscall" fn noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let t = read_usize(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_NOTICIA_IMPORTANTE_PARA_A_PESSOA)(noticia, pessoa); // caixa: só a original
    }
    let categoria: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
    let r = match categoria(noticia) as u8 {
        2 => matches!(t, 7..=0xb | 0x27 | 0x28),
        4 => matches!(t, 0xbb8 | 0xbb9 | 0xbc4),
        5 => match t {
            0xfa0 | 0xfa4 | 0xfad | 0xfae | 0xfaf | 0xfb0 | 0xfb2 | 0xfb4 | 0xfb7 | 0xfb8 | 0xfbb | 0xfbf | 0xfc0 => true,
            0xfa1 => {
                crate::comparacao::ramo("cpk_5 importante oferta pelo clube");
                cpk5_tecnico_do_clube(noticia, 0x19) == pessoa // o técnico do clube da oferta
            }
            0xfb6 => cpk5_tecnico_do_clube(noticia, 0x15) == pessoa || cpk5_tecnico_do_clube(noticia, 0x19) == pessoa,
            _ => false,
        },
        6 => t == 0x1770 && read_usize(noticia + 5) as i32 >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10),
        1 => t == 0x1388,
        _ => false, // 3 e 7: nenhum
    };
    r as u8
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_continente(this: usize, noticia: usize, continente: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_continente");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DO_CONTINENTE)(this, noticia, continente)
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_a_regiao_da_competicao(this: usize, noticia: usize, comp: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_a_regiao_da_competicao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_A_REGIAO_DA_COMPETICAO)(this, noticia, comp)
}

/// Comparador: pelo valor (a 0x785670 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_importante_para_a_pessoa(noticia: usize, pessoa: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_NOTICIA_IMPORTANTE_PARA_A_PESSOA)(noticia, pessoa);
    let t = read_usize(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = noticia_importante_para_a_pessoa(noticia, pessoa);
    PLACAR.registrar(r == n, || format!("noticia_importante_para_a_pessoa(tipo {:#x}, {:x}) original={} rust={}", t, pessoa, r, n));
    r
}

// ---------- 247a leva: o texto de um dado da notícia, a notícia do clube em apuro financeiro e se os olheiros do clube cobrem a nação ----------



/// copies the text stored from the slot (+5 + 4 × slot) into the buffer (strncpy) and clears the last byte; returns 1
/// Original: `NewsItem::get_slot_string` (0x785a20, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x785a20 } (thiscall, ret 0xc, news.cpp): `NewsItem::get_slot_string(dado, destino, n)`: copia o texto guardado a
/// partir do dado `dado` (+5 + 4 × dado) para `destino` como o strncpy (0x9466d0: até o zero, completando com zeros
/// até `n`) e zera o último byte (`destino[n − 1]`). Devolve 1. Dado fora de 0..49, destino nulo ou `n` < 1: a
/// original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(IDX_TEXTO_DO_DADO_DA_NOTICIA)(noticia, dado, destino, n); // caixa: só a original
    }
    crate::comparacao::ramo("cpk_6 texto do dado");
    let origem = (noticia + 5 + 4 * k as usize) as *const u8; // o dado
    let mut fim = false;
    for i in 0..n as usize {
        let c = if fim { 0 } else { *origem.add(i) };
        fim |= c == 0;
        *destino.add(i) = c;
    }
    *destino.add(n as usize - 1) = 0;
    1
}


/// O clube que a 0x7f9190 usa: o `clube`, ou o do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]] + 0x39).
pub(crate) unsafe fn cpk6_clube(clube: usize) -> usize {
    if clube != 0 {
        return clube;
    }
    read_usize(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize) + 0x39)
}


/// Comparador: lado a lado pelos `n` bytes do destino.
pub(crate) unsafe extern "thiscall" fn comparar_texto_do_dado_da_noticia(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(IDX_TEXTO_DO_DADO_DA_NOTICIA);
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return o(noticia, dado, destino, n);
    }
    let reg = [(destino as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(noticia, dado, destino, n), || texto_do_dado_da_noticia(noticia, dado, destino, n));
    PLACAR.registrar(igual && r == m, || format!("texto_do_dado_da_noticia({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}



// ---------- 246a leva: as notícias para os técnicos humanos: para todos, para os da divisão e para os da nação (clube ou seleção) ----------


/// O técnico humano `i` (0 a 15: os últimos 16 de [0xae23bc], 0x6e bytes), se ativo (+0x5f = 1).
pub(crate) unsafe fn cpj3_humano_ativo(i: usize) -> Option<usize> {
    let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
    (*((p + 0x5f) as *const u8) == 1).then_some(p)
}

/// Entrega a notícia a cada técnico humano ativo que `passa` (0x785d40 com 0), com o alcance `alcance` em +0xde.
/// Devolve se todas as entregas foram aceitas (a primeira recusada encerra).
pub(crate) unsafe fn cpj3_entregar(this: usize, noticia: usize, alcance: u8, passa: impl Fn(usize) -> bool) -> bool {
    *((noticia + 0xde) as *mut u8) = alcance;
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    for i in 0..0x10 {
        if let Some(p) = cpj3_humano_ativo(i) {
            if passa(p) && entregar(this, noticia, p, 0) == 0 {
                return false; // a entrega recusada encerra
            }
        }
    }
    true
}

/// all scope (+0xde = 4) and delivery (0x785d40) to every active human manager; 1, or 0 on the first refusal
/// Original: `News::send_to_all_managers` (0x786780, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x786780 } (thiscall, ret 4, news.cpp): `News::send_to_all_managers(noticia)`: alcance de todos (+0xde = 4) e a
/// entrega (0x785d40) a cada técnico humano ativo. Devolve 1, ou 0 na primeira recusada. Notícia nula: a original
/// (caixa "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    if noticia == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NOTICIA_PARA_TODOS_OS_TECNICOS)(this, noticia); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 4, |_| true) as u32
}

/// division scope (+0xde = 1) and delivery to every active human manager with a club in the division (+0x57); 1, or 0
/// on the first refusal
/// Original: `News::send_to_division_managers` (0x7862d0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7862d0 } (thiscall, ret 8, news.cpp): `News::send_to_division_managers(noticia, divisao)`: alcance da divisão
/// (+0xde = 1) e a entrega a cada técnico humano ativo com clube (+0x39) na `divisao` (+0x57). Devolve 1, ou 0 na
/// primeira recusada. Notícia ou divisão nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    if noticia == 0 || divisao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DA_DIVISAO)(this, noticia, divisao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 1, |p| {
        let c = read_usize(p + 0x39);
        c != 0 && read_usize(c + 0x57) == divisao // a divisão do clube
    }) as u32
}

/// nation scope (+0xde = 2) and delivery to every active human manager whose club or national side belongs to the
/// nation (+0x53); returns 0
/// Original: `News::send_to_nation_managers` (0x7863f0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x7863f0 } (thiscall, ret 8, news.cpp): `News::send_to_nation_managers(noticia, nacao)`: alcance da nação (+0xde
/// = 2) e a entrega a cada técnico humano ativo cujo clube (+0x39) ou seleção (+0x24) é da `nacao` (+0x53). Devolve 0.
/// Notícia ou nação nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DO_PAIS)(this, noticia, nacao); // caixa: só a original
    }
    cpj3_entregar(this, noticia, 2, |p| {
        let (c, s) = (read_usize(p + 0x39), read_usize(p + 0x24));
        (c != 0 && read_usize(c + 0x53) == nacao) || (s != 0 && read_usize(s + 0x53) == nacao) // o clube ou a seleção
    });
    0
}

/// Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_todos_os_tecnicos(this: usize, noticia: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_todos_os_tecnicos");
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NOTICIA_PARA_TODOS_OS_TECNICOS)(this, noticia)
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_divisao(this: usize, noticia: usize, divisao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_divisao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DA_DIVISAO)(this, noticia, divisao)
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_do_pais(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_do_pais");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DO_PAIS)(this, noticia, nacao)
}

// ---------- 245a leva: a notícia para os técnicos humanos de uma nação, a missão do olheiro da busca e se a copa terminou ----------


/// nation scope (+0xde = 2) and delivery (0x785d40) to every active human manager with a club of the nation; the first
/// refusal stops
/// Original: `News::send_to_nation_club_managers` (0x786520, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x786520 } (thiscall, ret 8, news.cpp): `News::send_to_nation_club_managers(noticia, nacao)`: a notícia fica com
/// o alcance da nação (+0xde = 2) e vai (0x785d40 com 0) para cada um dos 16 técnicos humanos (os últimos 16 de
/// [0xae23bc], 0x6e bytes) ativos (+0x5f = 1) com clube (+0x39) da `nacao` (+0x53); a primeira entrega recusada (0)
/// encerra. Devolve 0. Notícia ou nação nulas: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DA_NACAO)(this, noticia, nacao); // caixa: só a original
    }
    *((noticia + 0xde) as *mut u8) = 2; // o alcance da nação
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_DELIVER_TO_MANAGER);
    for i in 0..0x10usize {
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(G_STAFF_COUNT).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) != 1 {
            continue;
        }
        let clube = read_usize(p + 0x39);
        if clube == 0 || read_usize(clube + club::NATION) != nacao {
            continue;
        }
        if entregar(this, noticia, p, 0) == 0 {
            break; // a entrega recusada encerra
        }
    }
    0
}

/// O olheiro da busca confere (o tipo +0xa é 4, o índice +0x180 > −1 e a pessoa ainda confere com os nomes e a data
/// guardados pela 0x53e990)? Devolve a pessoa, ou nenhuma (a original mostra a caixa).
pub(crate) unsafe fn cpi6_olheiro(this: usize) -> Option<usize> {
    let i = read_usize(this + 0x180) as i32;
    if *((this + 0xa) as *const u8) != 4 || i <= -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(0x6e));
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    if confere(p, read_usize(this + 0x174), read_usize(this + 0x178), read_usize(this + 0x17c), this + 0x184) == 0 || p == 0 {
        return None;
    }
    Some(p)
}



/// Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_nacao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_NOTICIA_PARA_OS_TECNICOS_DA_NACAO)(this, noticia, nacao)
}



// ---------- 213a leva: as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano (news.cpp) ----------

/// A entrada (12 bytes) da caixa de notícias do técnico humano: [noticias+8] + 12 × (id − [0xae2364] +
/// 0x10); None quando o índice fica negativo (a original abre a caixa de erro).
pub(crate) unsafe fn entrada_da_caixa_do_tecnico(noticias: usize, tecnico: usize) -> Option<usize> {
    let i = (*(tecnico as *const i32)).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some((*((noticias + 8) as *const usize)).wrapping_add((i as usize).wrapping_mul(12)))
}

/// A notícia `k` da caixa que começa em `inicio`: [noticias] + 0xdf × (inicio + k).
pub(crate) unsafe fn noticia_da_caixa(noticias: usize, inicio: i32, k: i32) -> usize {
    (*(noticias as *const usize)).wrapping_add((inicio.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// the unread news in a human manager's inbox, in total or for one category
/// Original: `News::count_unread` (0x786fe0, news.cpp, thiscall).
///
/// [pt] { 0x786fe0 } (thiscall, ret 8, news.cpp): as notícias não lidas do técnico humano. Sem categoria (0):
/// o total da entrada (+8, negativo vira 0). Com categoria: das 100 notícias da caixa (início na entrada
/// +0; −1 = vazia, 0), as ocupadas (+0xd6) da categoria (0x785670) ainda não lidas (+0xda = 0). Técnico
/// nulo ou fora da faixa: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let entrada = match if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) } {
        Some(e) => e,
        None => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_NOTICIAS_NAO_LIDAS)(noticias, tecnico, categoria),
    };
    let total = (entrada + 8) as *mut i32;
    if *total < 0 {
        *total = 0;
    }
    if categoria as u8 == 0 {
        return *total; // total da caixa
    }
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return 0;
    }
    let categoria_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
    let mut conta = 0;
    for k in 0..100 {
        let n = noticia_da_caixa(noticias, inicio, k);
        if core::ptr::read_unaligned((n + 0xd6) as *const u32) != 0
            && categoria_de(n) as u8 == categoria as u8
            && core::ptr::read_unaligned((n + 0xda) as *const u32) == 0
        {
            conta += 1;
        }
    }
    conta
}

/// Onde a 0x786d30 grava: Err nos casos da caixa de erro (técnico nulo, id 0, vaga fora de 0..49, cópia
/// de outro id, índice negativo, id fora das 100 da caixa); Ok(None) com a caixa vazia (início −1);
/// Ok(Some(notícia)) achada.
pub(crate) unsafe fn alvo_da_marca(noticias: usize, tecnico: usize, id: u32, vaga: i32, copia: usize) -> Result<Option<usize>, ()> {
    if tecnico == 0 || id == 0 || !(0..0x32).contains(&vaga) {
        return Err(());
    }
    if copia != 0 && core::ptr::read_unaligned((copia + 0xd6) as *const u32) != id {
        return Err(());
    }
    let entrada = entrada_da_caixa_do_tecnico(noticias, tecnico).ok_or(())?;
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return Ok(None);
    }
    (0..100)
        .map(|k| noticia_da_caixa(noticias, inicio, k))
        .find(|&n| core::ptr::read_unaligned((n + 0xd6) as *const u32) == id)
        .map(Some)
        .ok_or(())
}

/// writes a value into a slot of a news item in the human manager's inbox and into its local copy
/// Original: `News::set_item_slot` (0x786d30, news.cpp, thiscall).
///
/// [pt] { 0x786d30 } (thiscall, ret 0x14, news.cpp): grava `valor` na vaga `vaga` (dword +5 + 4 × vaga) da
/// notícia `id` (+0xd6) da caixa do técnico humano, primeiro na `copia` (se houver) e depois na da caixa;
/// 1 se achou. Caixa vazia: só a cópia, 0. Casos de erro: a original.
pub(crate) unsafe extern "thiscall" fn gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let alvo = match alvo_da_marca(noticias, tecnico, id, vaga, copia) {
        Ok(a) => a,
        Err(()) => return orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(IDX_GRAVAR_MARCA_DA_NOTICIA)(noticias, tecnico, id, vaga, valor, copia),
    };
    if copia != 0 {
        core::ptr::write_unaligned((copia + 5 + 4 * vaga as usize) as *mut u32, valor);
    }
    match alvo {
        Some(n) => {
            core::ptr::write_unaligned((n + 5 + 4 * vaga as usize) as *mut u32, valor); // marca na caixa
            1
        }
        None => 0,
    }
}

/// whether a human manager runs the club, ignoring holidays
/// Original: `news_club_has_human_manager` (0x7875e0, news.cpp, stdcall).
///
/// [pt] { 0x7875e0 } (stdcall, ret 4, news.cpp): 1 se um humano dirige o `clube` (0x600a50 com férias
/// ignoradas); antes de devolver, consulta as férias (0x5fd210 em [0xb63c98]) do técnico do clube
/// (0x53ca90), sem usar a resposta. Clube nulo: 0.
pub(crate) unsafe extern "stdcall" fn clube_de_tecnico_humano(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let r = humano(clube, 1, 1, 0, 0);
    if r == 0 {
        return r;
    }
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let t = tecnico(clube);
    ferias(*(G_HUMAN_MANAGER as *const usize), t);
    1 // humano
}

pub(crate) unsafe extern "thiscall" fn comparar_noticias_nao_lidas(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_NOTICIAS_NAO_LIDAS);
    let entrada = if tecnico == 0 { None } else { entrada_da_caixa_do_tecnico(noticias, tecnico) };
    let Some(entrada) = entrada else {
        return o(noticias, tecnico, categoria);
    };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(entrada + 8, 4)], || o(noticias, tecnico, categoria),
        || noticias_nao_lidas(noticias, tecnico, categoria));
    PLACAR.registrar(igual && r == n, || format!("noticias_nao_lidas(categoria {}) original={} rust={} {:x?} x {:x?}", categoria as u8, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_marca_da_noticia(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(IDX_GRAVAR_MARCA_DA_NOTICIA);
    let Ok(alvo) = alvo_da_marca(noticias, tecnico, id, vaga, copia) else {
        return o(noticias, tecnico, id, vaga, valor, copia);
    };
    let mut regioes = vec![];
    if copia != 0 {
        regioes.push((copia + 5 + 4 * vaga as usize, 4));
    }
    if let Some(a) = alvo {
        regioes.push((a + 5 + 4 * vaga as usize, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(noticias, tecnico, id, vaga, valor, copia),
        || gravar_marca_da_noticia(noticias, tecnico, id, vaga, valor, copia));
    PLACAR.registrar(igual && r == n, || format!("gravar_marca_da_noticia(vaga {}) original={} rust={} {:x?} x {:x?}", vaga, r, n, mo, mr));
    r
}

pub(crate) unsafe extern "stdcall" fn comparar_clube_de_tecnico_humano(clube: usize) -> u32 {
    // só consultas: as duas rodam de novo sem efeito
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_CLUBE_DE_TECNICO_HUMANO)(clube);
    let n = clube_de_tecnico_humano(clube);
    PLACAR.registrar(r == n, || format!("clube_de_tecnico_humano original={} rust={}", r, n));
    r
}

/// gives the default replies for the manager's club and national team (0x7874b0) and empties his inbox (the 100 news
/// items with +0xd6 = 0 and +0xda = 1)
/// Original: `News::clear_inbox` (0x787630, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x787630 } (thiscall, ret 4, news.cpp): `News::clear_inbox(tecnico)`, pela 0x6b28b0 e pela carga do jogo
/// (0x82a9c0): as respostas padrão das notícias do clube (+0x39) e da seleção (+0x24) do técnico (0x7874b0); depois a
/// caixa do humano (registro de 12 bytes em [this+8], pelo id − [0xae2364] + 16): +4 = +0 e +8 = 0; com a primeira
/// notícia (+0, −1 nenhuma), as 100 notícias de 0xdf bytes a partir dela ([this]) com +0xd6 = 0 e +0xda = 1. Sem técnico
/// ou com índice negativo: a caixa de erro (linhas 0x24c1 e 0x24cc).
pub(crate) unsafe extern "thiscall" fn esvaziar_caixa_de_entrada(this: usize, tecnico: usize) {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x24c1);
        return;
    }
    let h = read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if h < 0 {
        aviso_em(0xa693ec, 0x24cc);
        return;
    }
    let caixa = (read_i32(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWS_AUTO_RESPOND_ALL);
    for o in [0x39usize, 0x24] {
        let c = read_i32(tecnico + o) as usize;
        if c != 0 {
            responder(this, tecnico, c);
        }
    }
    let primeira = read_i32(caixa);
    core::ptr::write_unaligned((caixa + 8) as *mut i32, 0);
    core::ptr::write_unaligned((caixa + 4) as *mut i32, primeira);
    if primeira != -1 {
        crate::comparacao::ramo("esvaziar_caixa_de_entrada");
        for k in 0..0x64u32 {
            let n = (read_i32(this) as u32).wrapping_add(k.wrapping_add(read_i32(caixa) as u32).wrapping_mul(0xdf)) as usize;
            core::ptr::write_unaligned((n + 0xd6) as *mut i32, 0);
            core::ptr::write_unaligned((n + 0xda) as *mut i32, 1);
        }
    }
}

/// Comparador: lado a lado na caixa e nas 100 notícias; com resposta padrão a dar (clube ou seleção), só a original,
/// conferindo o plano depois dela (+4 = +0, +8 = 0 e os dois campos das 100 notícias). Erros: a original.
pub(crate) unsafe extern "thiscall" fn comparar_esvaziar_caixa_de_entrada(this: usize, tecnico: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ESVAZIAR_CAIXA_DE_ENTRADA);
    if tecnico == 0 {
        return o(this, tecnico);
    }
    let h = read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    if h < 0 {
        return o(this, tecnico);
    }
    let caixa = (read_i32(this + 8) as u32).wrapping_add((h as u32).wrapping_mul(12)) as usize;
    if read_i32(tecnico + 0x39) != 0 || read_i32(tecnico + 0x24) != 0 {
        o(this, tecnico);
        let primeira = read_i32(caixa);
        let noticias = cpv10_regioes_da_caixa(this, caixa);
        let ok = read_i32(caixa + 4) == primeira && read_i32(caixa + 8) == 0
            && noticias[1..].iter().all(|&(n, _)| read_i32(n) == 0 && read_i32(n + 4) == 1);
        PLACAR.registrar(ok, || format!("esvaziar_caixa_de_entrada({:x}) plano: caixa {:x?}", tecnico, (primeira, read_i32(caixa + 4), read_i32(caixa + 8))));
        return;
    }
    let reg = cpv10_regioes_da_caixa(this, caixa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico), || esvaziar_caixa_de_entrada(this, tecnico));
    PLACAR.registrar(igual, || format!("esvaziar_caixa_de_entrada({:x}) {:x?} x {:x?}", tecnico, mo, mr));
}

/// builds the news item: type (+0), important byte (+4), the 50 data slots cleared, today's date (+0xcd), byte
/// [0xae2c88] at +0xd5, id and +0xda cleared; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode
/// left the reference be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game
/// later crashed in 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated
/// yet; the copy stays in the code and does not run
/// Original: `NewsItem::NewsItem` (0x77bbe0, news.cpp, thiscall, ret 8).
///
/// [pt] { 0x77bbe0 } (thiscall, ret 8, news.cpp, quadro SEH só da data, destrutor vazio 0x5a53c0): `NewsItem::NewsItem(tipo,
/// importante)`: inicia a data (+0xcd, 0x54a110), grava o tipo (+0), o byte importante (+4) e zera os 50 dados (+5);
/// a data de hoje ([0xae2c90], pela atribuição 0x417c10 com retorno numa temporária), o byte [0xae2c88] em +0xd5, o id
/// (+0xd6) e +0xda zerados. Devolve a notícia.
pub(crate) unsafe extern "thiscall" fn construir_noticia(this: usize, tipo: u32, importante: u32) -> usize {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    iniciar_data(this + 0xcd);
    write_usize(this, tipo as usize);
    *((this + 4) as *mut u8) = importante as u8;
    for k in 0..50 {
        write_usize(this + 5 + 4 * k, 0);
    }
    let mut temporaria = [0u32; 2];
    atribuir_data(this + 0xcd, temporaria.as_mut_ptr() as usize, 0xae2c90);
    *((this + 0xd5) as *mut u8) = *(G_CURRENT_DAY_SLICE as *const u8);
    write_usize(this + 0xd6, 0);
    write_usize(this + 0xda, 0);
    this
}

/// Comparador: pela memória da notícia (0xde bytes; a original usa só uma temporária na pilha) e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_construir_noticia(this: usize, tipo: u32, importante: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_CONSTRUIR_NOTICIA);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xde)], || o(this, tipo, importante), || construir_noticia(this, tipo, importante));
    PLACAR.registrar(igual && r == n, || format!("construir_noticia(tipo {:#x}, {}) {:x?} x {:x?}", tipo, importante as u8, mo.0[0].1, mr.0[0].1));
    r
}

/// a new array with the human's inbox items, newest first, only those of one category when asked, and the count
/// Original: `News::get_inbox_items` (0x786960, news.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x786960 } (thiscall, ret 0xc, news.cpp): `News::get_inbox_items(tecnico, conta, categoria)`: um vetor novo de 100
/// notícias (malloc 0x944e46 de 0x571c, na primeira achada; `*conta` = 0) com as notícias do anel do humano, da mais nova
/// para a mais antiga até a vaga vazia, só as da `categoria` (0x785670) se ela não for 0, copiadas (0x786010); `*conta`
/// soma uma por notícia. Devolve o vetor (0 sem nenhuma; `*conta` intocada). Sem técnico ou técnico da máquina: a caixa
/// "Error" e 0.
pub(crate) unsafe extern "thiscall" fn noticias_da_caixa(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x21fa);
        return 0;
    }
    let idx = read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, 0x2205);
        return 0;
    }
    let e = read_usize(this + 8) + idx as usize * 12;
    if read_i32(e) == -1 {
        return 0;
    }
    let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
    let copiar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWSITEM_OPERATOR_3D);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let mut vetor = 0usize;
    let mut k = read_i32(e + 4) - 1;
    loop {
        if k < read_i32(e) {
            k = read_i32(e) + 0x63;
        }
        let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA));
        if read_usize(vaga + 0xd6) == 0 {
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
            copiar(vetor.wrapping_add((n as usize).wrapping_mul(CPN3_NOTICIA)), read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA)));
        }
        k -= 1;
        if k == read_i32(e + 4) - 1 {
            return vetor;
        }
    }
}

/// Comparador: a original aloca o vetor: roda só ela e confere o plano feito antes (a conta e os ids, na ordem).
pub(crate) unsafe extern "thiscall" fn comparar_noticias_da_caixa(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_NOTICIAS_DA_CAIXA);
    if tecnico == 0 || read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, conta, categoria);
    }
    let e = read_usize(this + 8) + (read_i32(tecnico).wrapping_sub(read_i32(0xae2364)).wrapping_add(0x10)) as usize * 12;
    let mut ids = Vec::new();
    if read_i32(e) != -1 {
        let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
        let mut k = read_i32(e + 4) - 1;
        loop {
            if k < read_i32(e) {
                k = read_i32(e) + 0x63;
            }
            let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(CPN3_NOTICIA));
            if read_usize(vaga + 0xd6) == 0 {
                break;
            }
            if categoria as u8 == 0 || tipo(vaga) as u8 == categoria as u8 {
                ids.push(read_usize(vaga + 0xd6));
            }
            k -= 1;
            if k == read_i32(e + 4) - 1 {
                break;
            }
        }
    }
    let r = o(this, tecnico, conta, categoria);
    let ok = if ids.is_empty() {
        r == 0
    } else {
        r == 0 || (*conta as usize == ids.len() && ids.iter().enumerate().all(|(i, &id)| read_usize(r + i * CPN3_NOTICIA + 0xd6) == id))
    };
    PLACAR.registrar(ok, || format!("noticias_da_caixa(categoria {}) {} notícias, esperava {:?}", categoria as u8, if r == 0 { 0 } else { *conta }, ids));
    r
}

/// the news category by the thousands range of its type
/// Original: `NewsItem::get_category` (0x785670, news.cpp, thiscall).
///
/// [pt] { 0x785670 } (thiscall, news.cpp): a categoria da notícia pelo milhar do tipo (+0): 1..999 → 2,
/// 1000.. → 7, 2000.. → 3, 3000.. → 4, 4000.. → 5, 5000.. → 1, 6000..0x1790 → 6. Fora disso: a original
/// (caixa de erro). Devolve o eax inteiro como a original (o tipo com o al trocado).
pub(crate) unsafe extern "thiscall" fn categoria_da_noticia(noticia: usize) -> u32 {
    let t = *(noticia as *const i32);
    let c = match t {
        1..=999 => 2,
        1000..=1999 => 7,
        2000..=2999 => 3,
        3000..=3999 => 4,
        4000..=4999 => 5,
        5000..=5999 => 1,
        6000..=0x1790 => 6,
        _ => return orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CATEGORIA_DA_NOTICIA)(noticia),
    };
    (t as u32 & 0xffff_ff00) | c
}

pub(crate) unsafe extern "thiscall" fn comparar_categoria_da_noticia(noticia: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CATEGORIA_DA_NOTICIA);
    let t = *(noticia as *const i32);
    if !(1..=0x1790).contains(&t) {
        return o(noticia);
    }
    let r = o(noticia);
    let n = categoria_da_noticia(noticia);
    PLACAR.registrar(r == n, || format!("categoria_da_noticia({}) original={:x} rust={:x}", t, r, n));
    r
}

/// the value of one of the 50 news item slots
/// Original: `NewsItem::get_slot` (0x785830, news.cpp, thiscall).
///
/// [pt] { 0x785830 } (thiscall, ret 4, news.cpp): o valor da vaga `vaga` (0 a 49) da notícia (+5, 4 bytes
/// cada). Fora da faixa: a original (caixa de erro, devolve 0).
pub(crate) unsafe extern "thiscall" fn vaga_da_noticia(noticia: usize, vaga: i8) -> u32 {
    if !(0..0x32).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_VAGA_DA_NOTICIA)(noticia, vaga);
    }
    core::ptr::read_unaligned((noticia + 5 + vaga as usize * 4) as *const u32)
}

pub(crate) unsafe extern "thiscall" fn comparar_vaga_da_noticia(noticia: usize, vaga: i8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_VAGA_DA_NOTICIA)(noticia, vaga);
    if !(0..0x32).contains(&vaga) {
        return r;
    }
    let n = vaga_da_noticia(noticia, vaga);
    PLACAR.registrar(r == n, || format!("vaga_da_noticia({}) original={:#x} rust={:#x}", vaga, r, n));
    r
}

/// stores news field 0..49
/// Original: `set_news_field` (0x785790, news.cpp, thiscall).
///
/// [pt] { 0x785790 } (thiscall, ret 8, news.cpp): grava `valor` no dado `k` 0..49 da notícia (+5, 4 bytes
/// cada). Devolve 1. `k` fora: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn gravar_dado_da_noticia(t: usize, k: i8, valor: u32) -> u32 {
    if !(0..0x32).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_GRAVAR_DADO_DA_NOTICIA)(t, k, valor);
    }
    core::ptr::write_unaligned((t + 5 + 4 * k as usize) as *mut u32, valor);
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_dado_da_noticia(t: usize, k: i8, valor: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_GRAVAR_DADO_DA_NOTICIA);
    let reg = if (0..0x32).contains(&k) { vec![(t + 5 + 4 * k as usize, 4)] } else { vec![] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, k, valor), || gravar_dado_da_noticia(t, k, valor));
    PLACAR.registrar(igual && r == v, || format!("gravar_dado_da_noticia({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, k, valor, r, mo, v, mr));
    r
}

/// copies the news item (0xdf bytes): +0, +4, the 50 tokens (+5), the date (+0xcd), +0xd5, +0xd6, +0xda and +0xde
/// Original: `NewsItem::operator=` (0x786010, news.cpp, thiscall, ret 4).
///
/// [pt] { 0x786010 } (thiscall, ret 4, news.cpp): `NewsItem::operator=`: copia a notícia (0xdf bytes) de `origem`: +0
/// (dupla), +4 (byte), as 50 fichas de +5 (duplas, na ordem), a data em +0xcd, +0xd5 (byte), +0xd6 e +0xda (duplas) e
/// +0xde (byte). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn copiar_noticia(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    let b = |o: usize| *((this + o) as *mut u8) = *((origem + o) as *const u8);
    d(0);
    b(4);
    for k in 0..50 {
        d(5 + 4 * k); // as fichas da notícia
    }
    copiar_data_do_recorde(this + 0xcd, origem + 0xcd);
    b(0xd5);
    d(0xd6);
    d(0xda);
    b(0xde);
    this
}

pub(crate) unsafe extern "thiscall" fn comparar_copiar_noticia(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_COPIAR_NOTICIA);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xdf)], || o(this, origem), || copiar_noticia(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_noticia {:x?} x {:x?}", mo, mr));
    r
}
