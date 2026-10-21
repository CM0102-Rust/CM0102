// ---------- %%LEVA%%a leva: as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia (transfer_manager.cpp, shortlist_manager.cpp, comp_util.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(%%I0%%);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%);
    let reg = [(registro_da_lista_de_compras(this, clube) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || reputacao_alvo_do_clube(this, clube));
    PLACAR.registrar(igual && r == n, || format!("reputacao_alvo_do_clube({}) original={} rust={} {:x?} x {:x?}", *(clube as *const i32), r, n, mo, mr));
    r
}

unsafe extern "C" fn comparar_gravar_jogo_na_noticia(jogo: usize, noticia: usize, cursor: *mut u8) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut u8) -> u32>(%%I2%%);
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
