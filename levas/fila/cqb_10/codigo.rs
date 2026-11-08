// ---------- %%LEVA%%a leva: a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor (LLIST, vizinhança de manager_manager.cpp) ----------

unsafe fn cqb10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O nó da lista (14 bytes: +0 o item, +4 o tamanho em palavra com sinal, +6 o próximo, +0xa o anterior) cujo item tem
/// os mesmos bytes que `dado`, a partir da cabeça (+2). O tamanho vai como na original (movsx para o ecx do repe
/// cmpsb): 0 é igual (o repe não roda e o ZF vem do xor de antes).
unsafe fn find_list_node_by_bytes(this: usize, dado: usize) -> usize {
    let mut no = cqb10_d(this + 2); // cqb10: a busca começa na cabeça
    while no != 0 {
        let n = core::ptr::read_unaligned((no + 4) as *const i16) as i32 as u32;
        let item = cqb10_d(no);
        let mut k = 0u32;
        while k < n && *((item + k as usize) as *const u8) == *((dado + k as usize) as *const u8) {
            k += 1;
        }
        if k == n {
            return no;
        }
        no = cqb10_d(no + 6);
    }
    0
}

/// { 0x68d1f0 } (thiscall, ret 4): `LLIST::remove(dado)`: o primeiro nó com os bytes do `dado` (find_list_node_by_bytes) sai da
/// lista: a cabeça (+2) ou o próximo do anterior, a cauda (+6) ou o anterior do próximo, a conta (+0, palavra) − 1, o
/// cursor (+0xa) anda se estava nele; o item (0x68d010) e o nó (0x944c94) são liberados. Devolve 1; sem achar, 0. Por
/// ~77 chamadores (listas de ofertas, faltas, ultimatos, vagas de técnico, amistosos...).
unsafe extern "thiscall" fn manager_manager_remove(this: usize, dado: usize) -> u32 {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d010usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let no = find_list_node_by_bytes(this, dado);
    if no == 0 {
        return 0;
    }
    crate::comparison::branch("cqb_10 tirado");
    let (prox, ant) = (cqb10_d(no + 6), cqb10_d(no + 0xa));
    if no == cqb10_d(this + 2) {
        cqb10_por(this + 2, prox);
    } else {
        cqb10_por(ant + 6, prox);
    }
    if no == cqb10_d(this + 6) {
        crate::comparison::branch("cqb_10 cauda");
        cqb10_por(this + 6, ant); // cqb10: a cauda volta ao anterior
    } else {
        cqb10_por(prox + 0xa, ant);
    }
    let conta = core::ptr::read_unaligned(this as *const i16);
    core::ptr::write_unaligned(this as *mut i16, conta.wrapping_sub(1));
    if no == cqb10_d(this + 0xa) {
        crate::comparison::branch("cqb_10 cursor");
        cqb10_por(this + 0xa, cqb10_d(no + 6)); // cqb10: o cursor anda
    }
    liberar_item(no);
    liberar(no);
    1
}

/// Comparador: a 0x68d1f0 libera o nó e o item (não roda duas vezes): o plano antes (o nó achado pela cópia, a cabeça,
/// a cauda, a conta, o cursor e os vizinhos); a original tira; confere a devolução, o cabeçalho (14 bytes) e os elos
/// dos vizinhos.
unsafe extern "thiscall" fn comparar_manager_manager_remove(this: usize, dado: usize) -> u32 {
    let no = find_list_node_by_bytes(this, dado);
    let antes: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let (prox, ant) = if no != 0 { (cqb10_d(no + 6), cqb10_d(no + 0xa)) } else { (0, 0) };
    let (cabeca, cauda, cursor) = (cqb10_d(this + 2), cqb10_d(this + 6), cqb10_d(this + 0xa));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, dado);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let ok = if no == 0 {
        r == 0 && depois == antes
    } else {
        let conta = core::ptr::read_unaligned(antes.as_ptr() as *const i16).wrapping_sub(1);
        r == 1
            && core::ptr::read_unaligned(this as *const i16) == conta
            && cqb10_d(this + 2) == if no == cabeca { prox } else { cabeca }
            && cqb10_d(this + 6) == if no == cauda { ant } else { cauda }
            && cqb10_d(this + 0xa) == if no == cursor { prox } else { cursor }
            && (no == cabeca || cqb10_d(ant + 6) == prox)
            && (no == cauda || cqb10_d(prox + 0xa) == ant)
    };
    PLACAR.record(ok, || format!("manager_manager_remove({:#x}, {:#x}) original={} nó {:#x} cabeçalho {:x?} -> {:x?}", this, dado, r, no, antes, depois));
    r
}

/// { 0x68d080 } (thiscall, ret 0): `LLIST::clear()`: libera, da cabeça (+2) em diante, o item (0x68d010) e o nó
/// (0x944c94) de cada um (a cabeça vai andando) e zera a conta (+0, palavra), a cabeça, a cauda (+6) e o cursor (+0xa).
unsafe extern "thiscall" fn manager_manager_clear(this: usize) {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d010usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    while cqb10_d(this + 2) != 0 {
        let no = cqb10_d(this + 2);
        let prox = cqb10_d(no + 6);
        liberar_item(no);
        liberar(no);
        cqb10_por(this + 2, prox);
    }
    core::ptr::write_unaligned(this as *mut u16, 0);
    cqb10_por(this + 2, 0);
    cqb10_por(this + 6, 0);
    cqb10_por(this + 0xa, 0);
}

/// Comparador: libera os nós (não roda duas vezes): só a original; confere o cabeçalho zerado (14 bytes).
unsafe extern "thiscall" fn comparar_manager_manager_clear(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    PLACAR.record(depois == [0u8; 14], || format!("manager_manager_clear({:#x}) cabeçalho {:x?}", this, depois));
}

/// { 0x68d2e0 } (thiscall, ret 0): `LLIST::get_current()`: o item do nó do cursor (+0xa), sem andar; sem cursor, 0.
unsafe extern "thiscall" fn get_current(this: usize) -> usize {
    let no = cqb10_d(this + 0xa);
    if no == 0 {
        return 0;
    }
    cqb10_d(no)
}

unsafe extern "thiscall" fn comparar_get_current(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this);
    let n = get_current(this);
    PLACAR.record(r == n, || format!("get_current({:#x}) original={:#x} rust={:#x}", this, r, n));
    r
}
