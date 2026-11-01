// ---------- %%LEVA%%a leva: o dinheiro das transferências: a venda de jogador, a compra de jogador e as luvas (finance.cpp) ----------

/// Soma `v` (64 bits) ao i64 em `a` (o add/adc da original).
unsafe fn cpz12_somar64(a: usize, v: i64) {
    core::ptr::write_unaligned(a as *mut i64, core::ptr::read_unaligned(a as *const i64).wrapping_add(v));
}

/// Soma `v` ao i32 em `a`.
unsafe fn cpz12_somar32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// O valor de 64 bits que a original recebe em dois dwords (baixo primeiro).
fn cpz12_valor(baixo: u32, alto: u32) -> i64 {
    ((alto as u64) << 32 | baixo as u64) as i64
}

/// { 0x599800 } (thiscall, ret 8, finance.cpp): `Finance::add_players_sold(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0 e as parcelas 0x8caf.., 0x8cb3..) e pela
/// parte dos clubes anteriores (0x8e4f20): soma o valor às vendas do mês (+0x78) e do ano (+0x118), em 64 bits, e ao
/// saldo (i64 em +0).
unsafe extern "thiscall" fn receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x78, v);
    cpz12_somar64(this + 0x118, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 venda recebida"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v); // cpz12: a venda entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || receber_venda_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("receber_venda_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// { 0x599890 } (thiscall, ret 8, finance.cpp): `Finance::pay_players_bought(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (0x8ca8a0) e pelas parcelas (0x8caf.., 0x8cb3..): soma o valor às compras do mês (+0x70) e do ano
/// (+0x110), em 64 bits, e o tira do saldo.
unsafe extern "thiscall" fn pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x70, v);
    cpz12_somar64(this + 0x110, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 compra paga"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v.wrapping_neg()); // cpz12: a compra sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || pagar_compra_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("pagar_compra_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// { 0x5998e0 } (thiscall, ret 4, finance.cpp): `Finance::pay_sign_on_fee(valor)`, pela oferta de contrato aceita
/// (0x8c38b0) e pela mudança do jogador de clube (0x8c9720): soma o valor às luvas do ano (+0x13c) e do mês (+0x9c) e o
/// tira do saldo (estendido com sinal).
unsafe extern "thiscall" fn pagar_luvas(this: usize, valor: i32) {
    cpz12_somar32(this + 0x13c, valor);
    cpz12_somar32(this + 0x9c, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_12 luvas pagas"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, (valor as i64).wrapping_neg()); // cpz12: as luvas saem do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_luvas(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_luvas(this, valor));
    PLACAR.registrar(igual, || format!("pagar_luvas(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}
