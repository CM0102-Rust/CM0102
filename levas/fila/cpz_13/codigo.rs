// ---------- %%LEVA%%a leva: as receitas e a compensação: o prêmio em dinheiro, os direitos de TV e o salário pago (finance.cpp) ----------

/// Soma `v` ao i32 em `a`.
unsafe fn cpz13_somar32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// Soma `v` (estendido com sinal: o cdq/adc da original) ao saldo, i64 em +0.
unsafe fn cpz13_somar_saldo(fin: usize, v: i64) {
    core::ptr::write_unaligned(fin as *mut i64, core::ptr::read_unaligned(fin as *const i64).wrapping_add(v));
}

/// { 0x5999a0 } (thiscall, ret 4, finance.cpp): `Finance::add_prize_money(valor)`, pelas copas (Cup::process_fixture
/// 0x51a150, EuropeanCup::process_fixture 0x583b10, os sorteios das fases), pelo fim das ligas (League::pay_prize_money
/// 0x682e10) e pelas honras da temporada: soma o valor aos prêmios do ano (+0x148) e do mês (+0xa8) e ao saldo.
unsafe extern "thiscall" fn receber_premio_em_dinheiro(this: usize, valor: i32) {
    cpz13_somar32(this + 0x148, valor);
    cpz13_somar32(this + 0xa8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 premio recebido"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: o prêmio entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_premio_em_dinheiro(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I0%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_premio_em_dinheiro(this, valor));
    PLACAR.registrar(igual, || format!("receber_premio_em_dinheiro(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x5987c0 } (thiscall, ret 4, finance.cpp): `Finance::add_tv_revenue(valor)`, só pelos direitos de TV da liga
/// (League::pay_tv_rights 0x683010, pela montagem da tabela de cada liga na temporada nova): soma o valor à TV do ano
/// (+0x158) e do mês (+0xb8) e ao saldo.
unsafe extern "thiscall" fn receber_direitos_de_tv(this: usize, valor: i32) {
    cpz13_somar32(this + 0x158, valor);
    cpz13_somar32(this + 0xb8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 tv recebida"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: a TV entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_direitos_de_tv(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_direitos_de_tv(this, valor));
    PLACAR.registrar(igual, || format!("receber_direitos_de_tv(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x599920 } (thiscall, ret 4, finance.cpp): `Finance::pay_wages(valor)`, só pela liberação sem custo
/// (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0: a compensação do contrato ao clube): soma o valor aos salários do ano
/// (+0x130) e do mês (+0x90) e o tira do saldo.
unsafe extern "thiscall" fn pagar_salarios(this: usize, valor: i32) {
    cpz13_somar32(this + 0x130, valor);
    cpz13_somar32(this + 0x90, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 salario pago"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, (valor as i64).wrapping_neg()); // cpz13: o salário sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_salarios(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(%%I2%%);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_salarios(this, valor));
    PLACAR.registrar(igual, || format!("pagar_salarios(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}
