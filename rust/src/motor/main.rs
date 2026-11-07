use super::*;

// ---------- 264a leva: a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor (LLIST, vizinhança de manager_manager.cpp) ----------


pub(crate) unsafe fn cqb10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O nó da lista (14 bytes: +0 o item, +4 o tamanho em palavra com sinal, +6 o próximo, +0xa o anterior) cujo item tem
/// os mesmos bytes que `dado`, a partir da cabeça (+2). O tamanho vai como na original (movsx para o ecx do repe
/// cmpsb): 0 é igual (o repe não roda e o ZF vem do xor de antes).
pub(crate) unsafe fn cqb10_achar(this: usize, dado: usize) -> usize {
    let mut no = read_usize(this + 2); // cqb10: a busca começa na cabeça
    while no != 0 {
        let n = core::ptr::read_unaligned((no + 4) as *const i16) as i32 as u32;
        let item = read_usize(no);
        let mut k = 0u32;
        while k < n && *((item + k as usize) as *const u8) == *((dado + k as usize) as *const u8) {
            k += 1;
        }
        if k == n {
            return no;
        }
        no = read_usize(no + 6);
    }
    0
}

/// the first node whose item has the same bytes as the data leaves the list (head, tail, count and cursor fixed) and is
/// freed with its item; 1 when removed
/// Original: `LLIST::remove` (0x68d1f0, manager_manager.cpp, thiscall).
///
/// [pt] { 0x68d1f0 } (thiscall, ret 4): `LLIST::remove(dado)`: o primeiro nó com os bytes do `dado` (cqb10_achar) sai da
/// lista: a cabeça (+2) ou o próximo do anterior, a cauda (+6) ou o anterior do próximo, a conta (+0, palavra) − 1, o
/// cursor (+0xa) anda se estava nele; o item (0x68d010) e o nó (0x944c94) são liberados. Devolve 1; sem achar, 0. Por
/// ~77 chamadores (listas de ofertas, faltas, ultimatos, vagas de técnico, amistosos...).
pub(crate) unsafe extern "thiscall" fn tirar_da_lista(this: usize, dado: usize) -> u32 {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_NODE_DESTRUCTOR_LLIST_NODE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let no = cqb10_achar(this, dado);
    if no == 0 {
        return 0;
    }
    crate::comparacao::ramo("cqb_10 tirado");
    let (prox, ant) = (read_usize(no + 6), read_usize(no + 0xa));
    if no == read_usize(this + 2) {
        cqb10_por(this + 2, prox);
    } else {
        cqb10_por(ant + 6, prox);
    }
    if no == read_usize(this + 6) {
        crate::comparacao::ramo("cqb_10 cauda");
        cqb10_por(this + 6, ant); // cqb10: a cauda volta ao anterior
    } else {
        cqb10_por(prox + 0xa, ant);
    }
    let conta = core::ptr::read_unaligned(this as *const i16);
    core::ptr::write_unaligned(this as *mut i16, conta.wrapping_sub(1));
    if no == read_usize(this + 0xa) {
        crate::comparacao::ramo("cqb_10 cursor");
        cqb10_por(this + 0xa, read_usize(no + 6)); // cqb10: o cursor anda
    }
    liberar_item(no);
    liberar(no);
    1
}

/// Comparador: a 0x68d1f0 libera o nó e o item (não roda duas vezes): o plano antes (o nó achado pela cópia, a cabeça,
/// a cauda, a conta, o cursor e os vizinhos); a original tira; confere a devolução, o cabeçalho (14 bytes) e os elos
/// dos vizinhos.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_da_lista(this: usize, dado: usize) -> u32 {
    let no = cqb10_achar(this, dado);
    let antes: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let (prox, ant) = if no != 0 { (read_usize(no + 6), read_usize(no + 0xa)) } else { (0, 0) };
    let (cabeca, cauda, cursor) = (read_usize(this + 2), read_usize(this + 6), read_usize(this + 0xa));
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1485)(this, dado);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    let ok = if no == 0 {
        r == 0 && depois == antes
    } else {
        let conta = core::ptr::read_unaligned(antes.as_ptr() as *const i16).wrapping_sub(1);
        r == 1
            && core::ptr::read_unaligned(this as *const i16) == conta
            && read_usize(this + 2) == if no == cabeca { prox } else { cabeca }
            && read_usize(this + 6) == if no == cauda { ant } else { cauda }
            && read_usize(this + 0xa) == if no == cursor { prox } else { cursor }
            && (no == cabeca || read_usize(ant + 6) == prox)
            && (no == cauda || read_usize(prox + 0xa) == ant)
    };
    PLACAR.registrar(ok, || format!("tirar_da_lista({:#x}, {:#x}) original={} nó {:#x} cabeçalho {:x?} -> {:x?}", this, dado, r, no, antes, depois));
    r
}

/// frees every node and item of the list and zeroes the count, head, tail and cursor
/// Original: `LLIST::clear` (0x68d080, manager_manager.cpp, thiscall).
///
/// [pt] { 0x68d080 } (thiscall, ret 0): `LLIST::clear()`: libera, da cabeça (+2) em diante, o item (0x68d010) e o nó
/// (0x944c94) de cada um (a cabeça vai andando) e zera a conta (+0, palavra), a cabeça, a cauda (+6) e o cursor (+0xa).
pub(crate) unsafe extern "thiscall" fn esvaziar_lista(this: usize) {
    let liberar_item: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_NODE_DESTRUCTOR_LLIST_NODE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    while read_usize(this + 2) != 0 {
        let no = read_usize(this + 2);
        let prox = read_usize(no + 6);
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
pub(crate) unsafe extern "thiscall" fn comparar_esvaziar_lista(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1486)(this);
    let depois: [u8; 14] = core::ptr::read_unaligned(this as *const [u8; 14]);
    PLACAR.registrar(depois == [0u8; 14], || format!("esvaziar_lista({:#x}) cabeçalho {:x?}", this, depois));
}

/// the item of the cursor node, without moving; with no cursor, 0
/// Original: `LLIST::get_current` (0x68d2e0, manager_manager.cpp, thiscall).
///
/// [pt] { 0x68d2e0 } (thiscall, ret 0): `LLIST::get_current()`: o item do nó do cursor (+0xa), sem andar; sem cursor, 0.
pub(crate) unsafe extern "thiscall" fn item_do_cursor(this: usize) -> usize {
    let no = read_usize(this + 0xa);
    if no == 0 {
        return 0;
    }
    read_usize(no)
}

pub(crate) unsafe extern "thiscall" fn comparar_item_do_cursor(this: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1487)(this);
    let n = item_do_cursor(this);
    PLACAR.registrar(r == n, || format!("item_do_cursor({:#x}) original={:#x} rust={:#x}", this, r, n));
    r
}

