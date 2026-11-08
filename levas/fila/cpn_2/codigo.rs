// ---------- %%LEVA%%a leva: a caixa do técnico humano: respond a todas as notícias, se há notícia pendente e o texto num campo da notícia (news.cpp) ----------

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
unsafe extern "thiscall" fn auto_respond_all(this: usize, tecnico: usize, clube: usize) -> u32 {
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
    let respond: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x784100usize);
    let mut k = cpn2_i(e + 4) - 1;
    loop {
        k = cpn2_volta(k, cpn2_i(e));
        let vaga = cpn2_d(this).wrapping_add((k as usize).wrapping_mul(CPN2_NOTICIA));
        if cpn2_d(vaga + 0xd6) == 0 {
            return 1;
        }
        crate::comparacao::ramo("cpn_2 respond todas");
        respond(vaga, tecnico, clube); // a resposta automática
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
unsafe extern "thiscall" fn has_urgent_items(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
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
unsafe extern "thiscall" fn set_slot_string(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
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
unsafe extern "thiscall" fn comparar_auto_respond_all(this: usize, tecnico: usize, clube: usize) -> u32 {
    let humano = tecnico != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) >= 0;
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, tecnico, clube);
    PLACAR.registrar(r == humano as u32, || format!("auto_respond_all(técnico {:x}, humano {}) devolveu {}", tecnico, humano, r));
    r
}

/// Comparador: lado a lado pela devolução e pelo texto montado em 0xde1f64 (a 0x782b70 não sorteia; a caixa não muda).
unsafe extern "thiscall" fn comparar_has_urgent_items(this: usize, tecnico: usize, so_urgentes: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    if so_urgentes != 0 && cpn2_i(tecnico).wrapping_sub(cpn2_i(0xae2364)).wrapping_add(0x10) < 0 {
        return o(this, tecnico, so_urgentes); // a caixa "Error"
    }
    let reg = [(0xde1f64usize, 0x7d0usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tecnico, so_urgentes), || has_urgent_items(this, tecnico, so_urgentes));
    PLACAR.registrar(igual && r == n, || format!("has_urgent_items(urgentes {}) {} x {} {:x?} x {:x?}", so_urgentes, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelos campos da notícia (+5..+0xcc) e pelo próximo campo.
unsafe extern "thiscall" fn comparar_set_slot_string(this: usize, campo: u32, texto: *const u8, proximo: *mut u8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *const u8, *mut u8) -> u32>(%%I2%%);
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
