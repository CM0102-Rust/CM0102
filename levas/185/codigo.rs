// ---------- 185a leva: a marca de contrato acertado e a vaga da oferta (transfer_manager.cpp) ----------

/// O byte das marcas da pessoa `*id` no gerente de transferências (+0x86e, 8 bytes por pessoa).
unsafe fn marcas_da_pessoa(gerente: usize, id: *const i32) -> *mut u8 {
    (*((gerente + 0x86e) as *const usize) as isize + *id as isize * 8) as *mut u8
}

/// { 0x8c5410 } (thiscall, ret 4, transfer_manager.cpp): liga a marca de contrato acertado (0x40).
unsafe extern "thiscall" fn marcar_contrato_acertado(gerente: usize, id: *const i32) {
    *marcas_da_pessoa(gerente, id) |= 0x40;
}

/// { 0x8c5430 } (thiscall, ret 4, transfer_manager.cpp): desliga a marca de contrato acertado.
unsafe extern "thiscall" fn desmarcar_contrato_acertado(gerente: usize, id: *const i32) {
    *marcas_da_pessoa(gerente, id) &= 0xbf;
}

/// { 0x8c61e0 } (thiscall, ret 4, transfer_manager.cpp): o endereço da vaga `k` das ofertas (+0,
/// +4 quantas); fora da faixa, o próprio começo da tabela (o valor de +0, não o endereço).
unsafe extern "thiscall" fn vaga_da_oferta(tabela: usize, k: i32) -> usize {
    let base = *(tabela as *const usize);
    if k < *((tabela + 4) as *const i32) { (base as isize + 4 * k as isize) as usize } else { base }
}

unsafe extern "thiscall" fn comparar_marcar_contrato_acertado(gerente: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(427);
    let a = marcas_da_pessoa(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || marcar_contrato_acertado(gerente, id));
    PLACAR.registrar(igual, || format!("marcar_contrato_acertado({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_desmarcar_contrato_acertado(gerente: usize, id: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32)>(428);
    let a = marcas_da_pessoa(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || desmarcar_contrato_acertado(gerente, id));
    PLACAR.registrar(igual, || format!("desmarcar_contrato_acertado({}) {:x?} x {:x?}", *id, mo, mr));
}

unsafe extern "thiscall" fn comparar_vaga_da_oferta(tabela: usize, k: i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(429)(tabela, k);
    let n = vaga_da_oferta(tabela, k);
    PLACAR.registrar(r == n, || format!("vaga_da_oferta({}) original={:#x} rust={:#x}", k, r, n));
    r
}
