// ---------- %%LEVA%%a leva: as notícias não lidas, a marca gravada na notícia da caixa e o clube de técnico humano (news.cpp) ----------

/// A entrada (12 bytes) da caixa de notícias do técnico humano: [noticias+8] + 12 × (id − [0xae2364] +
/// 0x10); None quando o índice fica negativo (a original abre a caixa de erro).
unsafe fn manager_inbox_entry(noticias: usize, tecnico: usize) -> Option<usize> {
    let i = (*(tecnico as *const i32)).wrapping_sub(*(0xae2364 as *const i32)).wrapping_add(0x10);
    if i < 0 {
        return None;
    }
    Some((*((noticias + 8) as *const usize)).wrapping_add((i as usize).wrapping_mul(12)))
}

/// A notícia `k` da caixa que começa em `inicio`: [noticias] + 0xdf × (inicio + k).
unsafe fn inbox_news_item(noticias: usize, inicio: i32, k: i32) -> usize {
    (*(noticias as *const usize)).wrapping_add((inicio.wrapping_add(k) as usize).wrapping_mul(0xdf))
}

/// { 0x786fe0 } (thiscall, ret 8, news.cpp): as notícias não lidas do técnico humano. Sem categoria (0):
/// o total da entrada (+8, negativo vira 0). Com categoria: das 100 notícias da caixa (início na entrada
/// +0; −1 = vazia, 0), as ocupadas (+0xd6) da categoria (0x785670) ainda não lidas (+0xda = 0). Técnico
/// nulo ou fora da faixa: a original (caixa de erro).
unsafe extern "thiscall" fn count_unread(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let entrada = match if tecnico == 0 { None } else { manager_inbox_entry(noticias, tecnico) } {
        Some(e) => e,
        None => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I0%%)(noticias, tecnico, categoria),
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
    let categoria_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x785670usize);
    let mut conta = 0;
    for k in 0..100 {
        let n = inbox_news_item(noticias, inicio, k);
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
unsafe fn set_slot_target(noticias: usize, tecnico: usize, id: u32, vaga: i32, copia: usize) -> Result<Option<usize>, ()> {
    if tecnico == 0 || id == 0 || !(0..0x32).contains(&vaga) {
        return Err(());
    }
    if copia != 0 && core::ptr::read_unaligned((copia + 0xd6) as *const u32) != id {
        return Err(());
    }
    let entrada = manager_inbox_entry(noticias, tecnico).ok_or(())?;
    let inicio = *(entrada as *const i32);
    if inicio == -1 {
        return Ok(None);
    }
    (0..100)
        .map(|k| inbox_news_item(noticias, inicio, k))
        .find(|&n| core::ptr::read_unaligned((n + 0xd6) as *const u32) == id)
        .map(Some)
        .ok_or(())
}

/// { 0x786d30 } (thiscall, ret 0x14, news.cpp): grava `valor` na vaga `vaga` (dword +5 + 4 × vaga) da
/// notícia `id` (+0xd6) da caixa do técnico humano, primeiro na `copia` (se houver) e depois na da caixa;
/// 1 se achou. Caixa vazia: só a cópia, 0. Casos de erro: a original.
unsafe extern "thiscall" fn set_item_slot(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let alvo = match set_slot_target(noticias, tecnico, id, vaga, copia) {
        Ok(a) => a,
        Err(()) => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(%%I1%%)(noticias, tecnico, id, vaga, valor, copia),
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

/// { 0x7875e0 } (stdcall, ret 4, news.cpp): 1 se um humano dirige o `clube` (0x600a50 com férias
/// ignoradas); antes de devolver, consulta as férias (0x5fd210 em [0xb63c98]) do técnico do clube
/// (0x53ca90), sem usar a resposta. Clube nulo: 0.
unsafe extern "stdcall" fn news_club_has_human_manager(clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let r = humano(clube, 1, 1, 0, 0);
    if r == 0 {
        return r;
    }
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let t = tecnico(clube);
    ferias(*(0xb63c98 as *const usize), t);
    1 // humano
}

unsafe extern "thiscall" fn comparar_count_unread(noticias: usize, tecnico: usize, categoria: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I0%%);
    let entrada = if tecnico == 0 { None } else { manager_inbox_entry(noticias, tecnico) };
    let Some(entrada) = entrada else {
        return o(noticias, tecnico, categoria);
    };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(entrada + 8, 4)], || o(noticias, tecnico, categoria),
        || count_unread(noticias, tecnico, categoria));
    PLACAR.registrar(igual && r == n, || format!("count_unread(categoria {}) original={} rust={} {:x?} x {:x?}", categoria as u8, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_set_item_slot(noticias: usize, tecnico: usize, id: u32, vaga: i32, valor: u32, copia: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32>(%%I1%%);
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

unsafe extern "stdcall" fn comparar_news_club_has_human_manager(clube: usize) -> u32 {
    // só consultas: as duas rodam de novo sem efeito
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I2%%)(clube);
    let n = news_club_has_human_manager(clube);
    PLACAR.registrar(r == n, || format!("news_club_has_human_manager original={} rust={}", r, n));
    r
}
