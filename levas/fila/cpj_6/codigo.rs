// ---------- %%LEVA%%a leva: depois da partida, a diretoria e a disciplina; as entradas do clube na shortlist e a maior oferta pelo jogador ----------

unsafe fn cpj6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x6951e0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::update_board_after_fixture(partida,
/// jogo)`: partida play (+0 ≠ −2, +0x40 ≠ 0): a confiança da diretoria de cada lado com técnico (+0xcf) muda pela
/// 0x695240 (mandante +0x1c com 1, visitante +0x20 com 0).
unsafe extern "thiscall" fn update_board_after_fixture(this: usize, partida: usize, jogo: usize) {
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
unsafe fn comp_tribunal(this: usize, id: i32) -> Option<usize> {
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
unsafe extern "thiscall" fn process_match(this: usize, partida: usize, resultado: u32) -> u32 {
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7e7ff0usize);
    let id = cpj6_d(partida) as i32;
    if id > -1 {
        if let Some(b) = comp_tribunal(this, id) {
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
        if let Some(t) = comp_tribunal(this, cpj6_d(liga) as i32) { // o tribunal da liga do clube
            julgar(t, partida, resultado, a, b, 0);
        }
    }
    0
}

/// As entradas (8 bytes: pessoa, clube) da shortlist (+4, [+0x14] delas, ordenada pelo clube) do clube `id`: a busca
/// binária (bsearch 0x946464 com a chave {−1, id} e a comparação 0x844740), recuando até a primeira do clube.
unsafe fn club_shortlist_entries(this: usize, id: u32) -> Vec<usize> {
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
unsafe extern "thiscall" fn collect_club_shortlist_entries(this: usize, clube: *const u32, lista: usize) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    for e in club_shortlist_entries(this, *clube) {
        acrescentar(lista, e, 8);
    }
}

/// A oferta `o` serve: do `clube` (+8) ou marcada (+0x2d = 1), e no estado 2 (+0x2c)?
unsafe fn is_offer_eligible(o: usize, clube: u32) -> bool {
    (cpj6_d(o + 8) as u32 == clube || *((o + 0x2d) as *const u8) == 1) && *((o + 0x2c) as *const u8) == 2
}

/// { 0x8cc930 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_highest_offer_for_player(jogador,
/// clube)`: com a marca de ofertas do jogador (byte +1 dos 8 de [+0x86e]), o primeiro registro do vetor [this] ([+4]
/// deles) do jogador (+0xc): sem lista (+0), ele mesmo se serve; com lista, das ofertas (índices no vetor) que servem,
/// a de maior valor (0x8e7a70 com 0, 0, 0, 1; a primeira no empate). Senão 0.
unsafe extern "thiscall" fn find_highest_offer_for_player(this: usize, jogador: *const u32, clube: *const u32) -> usize {
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
        return if is_offer_eligible(t, *clube) { t } else { 0 };
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpj6_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let (mut melhor, mut maior) = (0usize, -1i32);
    let mut e = proximo(l);
    while e != 0 {
        let o = cpj6_d(cpj6_d(this).wrapping_add(cpj6_d(e).wrapping_mul(4)));
        if is_offer_eligible(o, *clube) {
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
unsafe extern "thiscall" fn comparar_update_board_after_fixture(this: usize, partida: usize, jogo: usize) {
    crate::comparacao::ramo("update_board_after_fixture");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, partida, jogo)
}

/// Comparador: o tribunal (0x7e7ff0) dá as suspensões: só a original.
unsafe extern "thiscall" fn comparar_process_match(this: usize, partida: usize, resultado: u32) -> u32 {
    crate::comparacao::ramo("process_match");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, partida, resultado)
}

/// Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
unsafe extern "thiscall" fn comparar_collect_club_shortlist_entries(this: usize, clube: *const u32, lista: usize) {
    let esperado: Vec<[u8; 8]> = club_shortlist_entries(this, *clube).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let antes = list_items_capped(lista).len();
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(%%I2%%)(this, clube, lista);
    let novos: Vec<[u8; 8]> = list_items_capped(lista).iter().skip(antes).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    PLACAR.registrar(novos == esperado, || format!("collect_club_shortlist_entries({}) original {} itens, rust {}", *clube, novos.len(), esperado.len()));
}

/// Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
unsafe extern "thiscall" fn comparar_find_highest_offer_for_player(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(%%I3%%)(this, jogador, clube);
    let n = find_highest_offer_for_player(this, jogador, clube);
    PLACAR.registrar(r == n, || format!("find_highest_offer_for_player({}, {}) original={:#x} rust={:#x}", *jogador, *clube, r, n));
    r
}
