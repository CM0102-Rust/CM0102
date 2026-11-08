use super::*;

// The comparers of the routines in ../news.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../news.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `mark_item_read` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn comparar_mark_item_read(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_MARK_ITEM_READ);
    if tecnico == 0 || id == 0 || read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 || read_usize(this) == 0 {
        return o(this, tecnico, id, sincronizar);
    }
    let regioes = [
        (read_usize(this), read_i32(this + 4).max(0) as usize * STORED_NEWS_SIZE),
        (read_usize(this + 8), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico, id, sincronizar), || mark_item_read(this, tecnico, id, sincronizar));
    PLACAR.registrar(igual && r == n, || format!("mark_item_read(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparer for `find_item_by_id` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
pub(crate) unsafe extern "thiscall" fn comparar_find_item_by_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_FIND_ITEM_BY_ID);
    if id == 0 || saida == 0 || (tecnico != 0 && read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0) {
        return o(this, tecnico, id, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, STORED_NEWS_SIZE)], || o(this, tecnico, id, saida), || find_item_by_id(this, tecnico, id, saida));
    PLACAR.registrar(igual && r == n, || format!("find_item_by_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparer for `send_by_scope` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a entrega pode cair na caixa do humano (realloc, ids, resposta automática): roda só a original e confere
/// o alcance; alcance fora de 0..5 devolve 0.
pub(crate) unsafe extern "thiscall" fn comparar_send_by_scope(this: usize, noticia: usize, clube: usize, competicao: usize, nacao: usize, continente: usize, alcance: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32>(IDX_SEND_BY_SCOPE)(this, noticia, clube, competicao, nacao, continente, alcance);
    let fora = !(0..=5).contains(&(alcance as u8 as i8));
    PLACAR.registrar(!fora || r == 0, || format!("send_by_scope alcance {} devolveu {}", alcance as u8 as i8, r));
    r
}

/// Comparer for `send_to_club_manager` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: roda só a original (a entrega não pode rodar duas vezes) e confere o plano: o alcance zerado e, sem técnico
/// humano, a devolução 0 (com humano, a entrega devolve 1 ou 0 sem memória).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_club_manager(this: usize, noticia: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_CLUB_MANAGER);
    if noticia == 0 || clube == 0 {
        return o(this, noticia, clube);
    }
    let tecnico = read_usize(clube + club::MANAGER);
    let humano = tecnico != 0 && read_i32(tecnico) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let r = o(this, noticia, clube);
    let ok = *((noticia + 0xde) as *const u8) == 0 && (humano || r == 0) && r <= 1;
    PLACAR.registrar(ok, || format!("send_to_club_manager(clube {:x}, técnico {:x}, humano {}) devolveu {}", clube, tecnico, humano, r));
    r
}

/// Comparer for `deliver_to_manager` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original realoca o vetor, numera e responde: roda só ela e confere o plano feito antes: o id da notícia,
/// o contador de ids (News +0x10), o anel do humano (início, próxima vaga, conta), a vaga escrita com o id e +0xda = 0.
pub(crate) unsafe extern "thiscall" fn comparar_deliver_to_manager(this: usize, noticia: usize, tecnico: usize, livre: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_DELIVER_TO_MANAGER);
    if noticia == 0 || tecnico == 0 || news_human_manager_index(tecnico) < 0 {
        return o(this, noticia, tecnico, livre);
    }
    let idx = news_human_manager_index(tecnico) as usize;
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
        let v = |k: i32| read_usize(this).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
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
        PLACAR.registrar(novo, || format!("deliver_to_manager devolveu 0 sem anel novo")); // só sem memória
        return r;
    }
    let extra = if responde { (read_i32(e + 4) - prox).rem_euclid(0x64) } else { 0 };
    let vaga = read_usize(this).wrapping_add((atual as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
    let ok = r == 1 && read_i32(e) == inicio && read_i32(e + 4) == if extra == 0 { prox } else { read_i32(e + 4) }
        && read_i32(e + 8) >= conta_depois && read_i32(e + 8) <= conta_depois + extra
        && (read_usize(this + 0x10) == contador_depois || (extra > 0 && read_usize(this + 0x10) > contador_depois))
        && read_usize(noticia + 0xd6) == id && read_usize(noticia + 0xda) == 0
        && (repetida || read_usize(vaga + 0xd6) == id);
    PLACAR.registrar(ok, || format!("deliver_to_manager(humano {}) id {} (esperava {}), anel {}/{}/{} (esperava {}/{}/{}), repetida {}",
        idx, read_usize(noticia + 0xd6), id, read_i32(e), read_i32(e + 4), read_i32(e + 8), inicio, prox, conta_depois, repetida));
    r
}

/// Comparer for `auto_respond_all` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original responde (convites, propostas, emprego): roda só ela e confere a devolução (0 só sem técnico
/// ou com técnico da máquina; senão 1).
pub(crate) unsafe extern "thiscall" fn comparar_auto_respond_all(this: usize, tecnico: usize, clube: usize) -> u32 {
    let humano = tecnico != 0 && read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) >= 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_AUTO_RESPOND_ALL)(this, tecnico, clube);
    PLACAR.registrar(r == humano as u32, || format!("auto_respond_all(técnico {:x}, humano {}) devolveu {}", tecnico, humano, r));
    r
}

/// Comparer for `has_urgent_items` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
pub(crate) unsafe extern "thiscall" fn comparar_has_urgent_items(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HAS_URGENT_ITEMS);
    if so_urgentes != 0 && read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico, so_urgentes), || has_urgent_items(this, tecnico, so_urgentes));
    PLACAR.registrar(igual && r == n, || format!("has_urgent_items(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparer for `set_slot_string` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
pub(crate) unsafe extern "thiscall" fn comparar_set_slot_string(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(IDX_SET_SLOT_STRING);
    if !(0..0x32).contains(&(campo as u8 as i8)) || texto.is_null() {
        return o(this, campo, texto, proximo); // a caixa "Error"
    }
    let mut reg = vec![(this + 5, 0xc8usize)];
    if !proximo.is_null() {
        reg.push((proximo as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, campo, texto, proximo), || set_slot_string(this, campo, texto, proximo));
    PLACAR.registrar(igual && r == n, || format!("set_slot_string(campo {}) {:x?} x {:x?}", campo as u8 as i8, mo, mr));
    r
}

/// Comparer for `send_to_continent_managers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_continent_managers(this: usize, noticia: usize, continente: usize) -> u32 {
    crate::comparacao::ramo("send_to_continent_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_CONTINENT_MANAGERS)(this, noticia, continente)
}

/// Comparer for `send_to_comp_region` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_comp_region(this: usize, noticia: usize, comp: usize) -> u32 {
    crate::comparacao::ramo("send_to_comp_region");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_COMP_REGION)(this, noticia, comp)
}

/// Comparer for `is_important_for` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x785670 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_is_important_for(noticia: usize, pessoa: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_IS_IMPORTANT_FOR)(noticia, pessoa);
    let t = read_usize(noticia) as i32;
    if pessoa == 0 || !(1..=0x1790).contains(&t) {
        return r;
    }
    let n = is_important_for(noticia, pessoa);
    PLACAR.registrar(r == n, || format!("is_important_for(tipo {:#x}, {:x}) original={} rust={}", t, pessoa, r, n));
    r
}

/// Comparador: lado a lado pelos `n` bytes do destino.
pub(crate) unsafe extern "thiscall" fn comparar_get_slot_string(noticia: usize, dado: u32, destino: *mut u8, n: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32>(IDX_GET_SLOT_STRING);
    let k = dado as u8 as i8;
    if !(0..0x32).contains(&k) || destino.is_null() || n < 1 {
        return o(noticia, dado, destino, n);
    }
    let reg = [(destino as usize, n as usize)];
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(noticia, dado, destino, n), || get_slot_string(noticia, dado, destino, n));
    PLACAR.registrar(igual && r == m, || format!("get_slot_string({}, {}) {:x?} x {:x?}", k, n, mo, mr));
    r
}

/// Comparer for `send_to_all_managers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparadores: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_all_managers(this: usize, noticia: usize) -> u32 {
    crate::comparacao::ramo("send_to_all_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SEND_TO_ALL_MANAGERS)(this, noticia)
}

/// Comparer for `send_to_division_managers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_division_managers(this: usize, noticia: usize, divisao: usize) -> u32 {
    crate::comparacao::ramo("send_to_division_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_DIVISION_MANAGERS)(this, noticia, divisao)
}

/// Comparer for `send_to_nation_managers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_nation_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("send_to_nation_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_MANAGERS)(this, noticia, nacao)
}

/// Comparer for `send_to_nation_club_managers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_send_to_nation_club_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("send_to_nation_club_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SEND_TO_NATION_CLUB_MANAGERS)(this, noticia, nacao)
}

/// Comparer for `count_unread` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_count_unread(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_COUNT_UNREAD);
    let entrada = if tecnico == 0 { None } else { manager_inbox_entry(noticias, tecnico) };
    let Some(entrada) = entrada else {
        return o(noticias, tecnico, categoria);
    };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(entrada + 8, 4)], || o(noticias, tecnico, categoria),
        || count_unread(noticias, tecnico, categoria));
    PLACAR.registrar(igual && r == n, || format!("count_unread(categoria {}) original={} rust={} {:x?} x {:x?}", categoria as u8, r, n, mo, mr));
    r
}

/// Comparer for `set_item_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_item_slot(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(IDX_SET_ITEM_SLOT);
    let Ok(alvo) = set_slot_target(noticias, tecnico, id, vaga, copia) else {
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
        || set_item_slot(noticias, tecnico, id, vaga, valor, copia));
    PLACAR.registrar(igual && r == n, || format!("set_item_slot(vaga {}) original={} rust={} {:x?} x {:x?}", vaga, r, n, mo, mr));
    r
}

/// Comparer for `news_club_has_human_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_news_club_has_human_manager(clube: usize) -> u32 {
    // só consultas: as duas rodam de novo sem efeito
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_NEWS_CLUB_HAS_HUMAN_MANAGER)(clube);
    let n = news_club_has_human_manager(clube);
    PLACAR.registrar(r == n, || format!("news_club_has_human_manager original={} rust={}", r, n));
    r
}

/// Comparer for `clear_inbox` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na caixa e nas 100 notícias; com resposta padrão a dar (clube ou seleção), só a original,
/// conferindo o plano depois dela (+4 = +0, +8 = 0 e os dois campos das 100 notícias). Erros: a original.
pub(crate) unsafe extern "thiscall" fn comparar_clear_inbox(this: usize, tecnico: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CLEAR_INBOX);
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
        let noticias = news_box_regions(this, caixa);
        let ok = read_i32(caixa + 4) == primeira && read_i32(caixa + 8) == 0
            && noticias[1..].iter().all(|&(n, _)| read_i32(n) == 0 && read_i32(n + 4) == 1);
        PLACAR.registrar(ok, || format!("clear_inbox({:x}) plano: caixa {:x?}", tecnico, (primeira, read_i32(caixa + 4), read_i32(caixa + 8))));
        return;
    }
    let reg = news_box_regions(this, caixa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico), || clear_inbox(this, tecnico));
    PLACAR.registrar(igual, || format!("clear_inbox({:x}) {:x?} x {:x?}", tecnico, mo, mr));
}

/// Comparer for `construct_news_item` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pela memória da notícia (0xde bytes; a original usa só uma temporária na pilha) e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_construct_news_item(this: usize, tipo: u32, importante: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_CONSTRUCT_NEWS_ITEM);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xde)], || o(this, tipo, importante), || construct_news_item(this, tipo, importante));
    PLACAR.registrar(igual && r == n, || format!("construct_news_item(tipo {:#x}, {}) {:x?} x {:x?}", tipo, importante as u8, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparer for `get_inbox_items` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o vetor: roda só ela e confere o plano feito antes (a conta e os ids, na ordem).
pub(crate) unsafe extern "thiscall" fn comparar_get_inbox_items(this: usize, tecnico: usize, conta: *mut i32, categoria: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_GET_INBOX_ITEMS);
    if tecnico == 0 || read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, conta, categoria);
    }
    let e = read_usize(this + 8) + (read_i32(tecnico).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10)) as usize * 12;
    let mut ids = Vec::new();
    if read_i32(e) != -1 {
        let tipo: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_CATEGORY);
        let mut k = read_i32(e + 4) - 1;
        loop {
            if k < read_i32(e) {
                k = read_i32(e) + 0x63;
            }
            let vaga = read_usize(this).wrapping_add((k as usize).wrapping_mul(MEDIA_NEWS_ITEM_SIZE));
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
        r == 0 || (*conta as usize == ids.len() && ids.iter().enumerate().all(|(i, &id)| read_usize(r + i * MEDIA_NEWS_ITEM_SIZE + 0xd6) == id))
    };
    PLACAR.registrar(ok, || format!("get_inbox_items(categoria {}) {} notícias, esperava {:?}", categoria as u8, if r == 0 { 0 } else { *conta }, ids));
    r
}

/// Comparer for `get_category` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_category(noticia: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GET_CATEGORY);
    let t = *(noticia as *const i32);
    if !(1..=0x1790).contains(&t) {
        return o(noticia);
    }
    let r = o(noticia);
    let n = get_category(noticia);
    PLACAR.registrar(r == n, || format!("get_category({}) original={:x} rust={:x}", t, r, n));
    r
}

/// Comparer for `get_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_slot(noticia: usize, vaga: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_GET_SLOT)(noticia, vaga);
    if !(0..0x32).contains(&vaga) {
        return r;
    }
    let n = get_slot(noticia, vaga);
    PLACAR.registrar(r == n, || format!("get_slot({}) original={:#x} rust={:#x}", vaga, r, n));
    r
}

/// Comparer for `set_news_field` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_news_field(t: usize, k: i8, valor: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u32>(IDX_SET_NEWS_FIELD);
    let reg = if (0..0x32).contains(&k) { vec![(t + 5 + 4 * k as usize, 4)] } else { vec![] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, k, valor), || set_news_field(t, k, valor));
    PLACAR.registrar(igual && r == v, || format!("set_news_field({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, k, valor, r, mo, v, mr));
    r
}

/// Comparer for `news_operator` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_news_operator(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEWS_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xdf)], || o(this, origem), || news_operator(this, origem));
    PLACAR.registrar(igual && r == n, || format!("news_operator {:x?} x {:x?}", mo, mr));
    r
}
