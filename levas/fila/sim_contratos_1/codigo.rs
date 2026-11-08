// ---------- %%LEVA%%a leva: as ofertas de contrato e a oferta de transferência fechada ----------

/// Percorre a lista do gerente de contratos (+0x34, cursor em +0xa voltando ao começo +2, 0x68d1b0)
/// até a oferta que `achou` aceita; 0 sem lista ou sem nenhuma.
unsafe fn find_manager_offer(gerente: usize, achou: impl Fn(usize) -> bool) -> usize {
    let lista = *((gerente + 0x34) as *const usize);
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    loop {
        let o = proximo(*((gerente + 0x34) as *const usize));
        if o == 0 || achou(o) {
            return o;
        }
    }
}

/// { 0x4cc9e0 } (thiscall, ret 8, contract_manager.cpp): a oferta de contrato da pessoa `pessoa`
/// (+0) para o clube `clube` (+4), ou 0.
unsafe extern "thiscall" fn contract_offer(gerente: usize, pessoa: i32, clube: i32) -> usize {
    find_manager_offer(gerente, |o| *(o as *const i32) == pessoa && *((o + 4) as *const i32) == clube)
}

/// { 0x4cca30 } (thiscall, ret 4, contract_manager.cpp): a oferta acertada (+0x50 = 2) da pessoa, ou 0.
unsafe extern "thiscall" fn agreed_contract_offer(gerente: usize, pessoa: i32) -> usize {
    find_manager_offer(gerente, |o| *(o as *const i32) == pessoa && *((o + 0x50) as *const u8) == 2)
}

/// { 0x8d0d90 } (stdcall, transfer_offer.cpp): a oferta de transferência está encerrada? Estados
/// (+0x2c) 2, 3, 4, 0x10 e 0x14 sim; 0xf só com +0x2e a partir de 3; os demais não.
unsafe extern "system" fn transfer_offer_closed(o: usize) -> u32 {
    match *((o + 0x2c) as *const u8) {
        2 | 3 | 4 | 0x10 | 0x14 => 1,
        0xf => (*((o + 0x2e) as *const u8) >= 3) as u32,
        _ => 0,
    }
}

/// As regiões que as buscas mexem: o cursor da lista do gerente.
unsafe fn offer_list_regions(gerente: usize) -> Vec<(usize, usize)> {
    let lista = *((gerente + 0x34) as *const usize);
    if lista == 0 { Vec::new() } else { vec![(lista, 0xe)] }
}

unsafe extern "thiscall" fn comparar_contract_offer(gerente: usize, pessoa: i32, clube: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(%%I0%%);
    let reg = offer_list_regions(gerente);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(gerente, pessoa, clube), || contract_offer(gerente, pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("contract_offer({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_agreed_contract_offer(gerente: usize, pessoa: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I1%%);
    let reg = offer_list_regions(gerente);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(gerente, pessoa), || agreed_contract_offer(gerente, pessoa));
    SCOREBOARD.record(igual && r == n, || format!("agreed_contract_offer({}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

unsafe extern "system" fn comparar_transfer_offer_closed(o: usize) -> u32 {
    let r = original_fn::<unsafe extern "system" fn(usize) -> u32>(%%I2%%)(o);
    let n = transfer_offer_closed(o);
    SCOREBOARD.record(r == n, || format!("transfer_offer_closed(estado {:#x}, {:#x}) original={} rust={}", *((o + 0x2c) as *const u8), *((o + 0x2e) as *const u8), r, n));
    r
}
