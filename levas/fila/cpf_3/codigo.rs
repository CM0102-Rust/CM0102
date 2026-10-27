// ---------- %%LEVA%%a leva: as cópias do recorde de valor de transferência e do recorde de gols numa partida (club_records.cpp) ----------

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cpf3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x448550 } (thiscall, ret 4, club_records.cpp): `transfer_fee_record_assign`: copia o recorde de valor de
/// transferência (0x29 bytes) de `origem`: cinco duplas (+0..+0x10: o valor, o jogador, o outro clube...), a data em
/// +0x14, a dupla +0x1c, a data em +0x20 e a marca de troca (+0x28). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, core::ptr::read_unaligned((origem + o) as *const u32));
    for o in [0, 4, 8, 0xc, 0x10] {
        d(o); // o valor e os ids
    }
    cpf3_data(this + 0x14, origem + 0x14);
    d(0x1c);
    cpf3_data(this + 0x20, origem + 0x20);
    *((this + 0x28) as *mut u8) = *((origem + 0x28) as *const u8); // a marca de troca
    this
}

/// { 0x451570 } (thiscall, ret 4, club_records.cpp): `goals_in_match_record_assign`: copia o recorde de gols numa
/// partida (0x38 bytes) de `origem`: os gols (+0, byte), o recordista em +1 (0x448490) e o registro da partida em +0x19
/// (0x448110, ambos já em Rust). Devolve `this`.
unsafe extern "thiscall" fn copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let recordista: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448490usize);
    let partida: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x448110usize);
    *(this as *mut u8) = *(origem as *const u8); // os gols
    recordista(this + 1, origem + 1);
    partida(this + 0x19, origem + 0x19);
    this
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_valor(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x29)], || o(this, origem), || copiar_recorde_de_valor(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_valor {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_copiar_recorde_de_gols_na_partida(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x38)], || o(this, origem), || copiar_recorde_de_gols_na_partida(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
}
