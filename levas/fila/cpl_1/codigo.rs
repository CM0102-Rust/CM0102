// ---------- %%LEVA%%a leva: as ofertas pelo jogador: se o clube tem oferta por ele, a oferta do clube e se ele tem oferta pendente (transfer_manager.cpp) ----------

unsafe fn cpl1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A marca "sob oferta" da pessoa (bit 0 do byte [this+0x86e] + id × 8).
unsafe fn is_marked_under_offer(this: usize, id: usize) -> bool {
    *((cpl1_d(this + 0x86e).wrapping_add(id.wrapping_mul(8))) as *const u8) & 1 != 0
}

/// A oferta do índice `k` (vetor de ponteiros [this]).
unsafe fn offer_at(this: usize, k: usize) -> usize {
    cpl1_d(cpl1_d(this).wrapping_add(k.wrapping_mul(4)))
}

/// A lista local de índices das ofertas pela pessoa (0x68d020, que a 0x8c5cf0 enche com `todas`), com o cursor
/// (+0xa) reposto no começo (+2) como a original; `f` percorre e a lista é destruída (0x68d040) no fim. Nas saídas
/// antes da 0x8c5cf0 a original constrói e destrói a lista vazia, sem efeito (não aloca): aqui não nasce.
unsafe fn with_offers_for_player<R>(this: usize, pessoa: usize, todas: u32, f: impl FnOnce(usize) -> R) -> R {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    lista_nova(lista);
    ofertas(this, pessoa, lista, todas);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let r = f(lista);
    destruir(lista);
    r
}

/// O próximo nó da lista (0x68d1b0), ou 0.
unsafe fn llist_next_node(lista: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    proximo(lista)
}

/// A pessoa fora da tabela (id ≥ [0xae2364]); pessoa nula passa e é lida como na original.
unsafe fn staff_out_of_table(pessoa: usize) -> bool {
    pessoa != 0 && cpl1_d(pessoa) as i32 >= cpl1_d(0xae2364) as i32
}

/// { 0x8cdc90 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::club_has_offer_for_player(pessoa,
/// clube)`: 1 se a pessoa marcada como sob oferta (bit 0 de [this+0x86e] + id × 8) tem, entre as ofertas em aberto
/// (0x8c5cf0 sem `todas`), uma do clube (+8 da oferta = id do clube); senão 0 (id fora da tabela também).
unsafe extern "thiscall" fn club_has_offer_for_player(this: usize, pessoa: usize, clube: usize) -> u32 {
    if staff_out_of_table(pessoa) || !is_marked_under_offer(this, cpl1_d(pessoa)) {
        return 0;
    }
    with_offers_for_player(this, pessoa, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        if cpl1_d(offer_at(this, cpl1_d(no)) + 8) == cpl1_d(clube) { // a oferta é do clube
            crate::comparison::branch("cpl_1 club has offer");
            return 1;
        }
    })
}

/// { 0x8cdd80 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::find_club_offer_for_player(pessoa,
/// clube, todas)`: a oferta do clube pela pessoa (a primeira da lista da 0x8c5cf0, com `todas` também as encerradas),
/// ou 0. Sem `todas`, a pessoa tem de estar marcada como sob oferta; id fora da tabela: 0.
unsafe extern "thiscall" fn find_club_offer_for_player(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    if staff_out_of_table(pessoa) || (todas == 0 && !is_marked_under_offer(this, cpl1_d(pessoa))) {
        return 0;
    }
    with_offers_for_player(this, pessoa, todas, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let o = offer_at(this, cpl1_d(no));
        if cpl1_d(o + 8) == cpl1_d(clube) { // a oferta do clube
            crate::comparison::branch("cpl_1 club offer");
            return o;
        }
    })
}

/// { 0x8cd150 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::player_has_pending_offer(pessoa)`: 1 se
/// a pessoa está sob oferta (0x8cde70 sem a visão do humano) e uma das ofertas em aberto (0x8c5cf0) está na situação
/// (+0x2c) 0, 1, 5 ou 6; senão 0. O índice além da conta (+4) lê a oferta 0, como a original.
unsafe extern "thiscall" fn player_has_pending_offer(this: usize, pessoa: usize) -> u32 {
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    // ponytail: a original constrói a lista antes de perguntar; vazia, a 0x68d020/0x68d040 não alocam (nada a imitar)
    if sob_oferta(this, pessoa, 0) == 0 {
        return 0;
    }
    with_offers_for_player(this, pessoa, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let k = cpl1_d(no) as i32;
        let o = if k < cpl1_d(this + 4) as i32 { offer_at(this, k as usize) } else { cpl1_d(cpl1_d(this)) };
        if matches!(*((o + 0x2c) as *const u8), 0 | 1 | 5 | 6) { // a situação pendente
            crate::comparison::branch("cpl_1 pending offer");
            return 1;
        }
    })
}

/// As regiões que a 0x8c5cf0 mexe: o cursor da lista própria da oferta da pessoa (+0xa..+0xe; a 0x8c5cf0 o repõe).
unsafe fn offer_cursor_region(this: usize, pessoa: usize) -> [(usize, usize); 1] {
    let e = offer_for_player(this, cpl1_d(pessoa) as i32);
    let l = if e == 0 { 0 } else { cpl1_d(e) };
    if l == 0 { [(0xae2364, 4)] } else { [(l, 0xe)] }
}

// ponytail: as três só leem (a lista local nasce e morre nelas); com a pessoa nula, só a original (lê o endereço 0)
unsafe extern "thiscall" fn compare_club_has_offer_for_player(this: usize, pessoa: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    if pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, pessoa, clube), || club_has_offer_for_player(this, pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("club_has_offer_for_player({}, clube {}) original={} rust={} {:x?} x {:x?}", cpl1_d(pessoa), cpl1_d(clube), r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_find_club_offer_for_player(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize>(%%I1%%);
    if pessoa == 0 {
        return o(this, pessoa, clube, todas);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, pessoa, clube, todas), || find_club_offer_for_player(this, pessoa, clube, todas));
    SCOREBOARD.record(igual && r == n, || format!("find_club_offer_for_player({}, clube {}, todas {}) original={:x} rust={:x} {:x?} x {:x?}", cpl1_d(pessoa), cpl1_d(clube), todas, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_player_has_pending_offer(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, pessoa), || player_has_pending_offer(this, pessoa));
    SCOREBOARD.record(igual && r == n, || format!("player_has_pending_offer({}) original={} rust={} {:x?} x {:x?}", cpl1_d(pessoa), r, n, mo, mr));
    r
}
