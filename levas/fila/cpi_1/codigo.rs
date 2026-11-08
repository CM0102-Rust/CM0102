// ---------- %%LEVA%%a leva: os recordes do clube: a construção dos recordes de valor de transferência pago e recebido e do recorde de sequência ----------

/// { 0x4485d0 } (thiscall, club_records.cpp): `transfer_fee_record_construct`: constrói o recorde de maior valor de
/// transferência pago: as datas em +0x14 e +0x20 (0x54a110). O quadro SEH do original só cobre a 2ª data (a 0x54a110
/// não falha). Devolve `this`.
unsafe extern "thiscall" fn transfer_fee_record_construct(this: usize) -> usize {
    init_default_date(this + 0x14); // a data do valor pago
    init_default_date(this + 0x20);
    this
}

/// { 0x448620 } (thiscall, club_records.cpp): `transfer_fee_record_construct_2`: constrói o recorde de maior valor de
/// transferência recebido: as datas em +0x14 e +0x20 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn transfer_fee_record_construct_2(this: usize) -> usize {
    init_default_date(this + 0x14);
    init_default_date(this + 0x20); // a 2ª data do valor recebido
    this
}

/// { 0x448670 } (thiscall, club_records.cpp): `run_record_construct`: constrói o recorde de sequência: as datas do
/// começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn run_record_construct(this: usize) -> usize {
    init_default_date(this + 4); // o começo da sequência
    init_default_date(this + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_transfer_fee_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || transfer_fee_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("transfer_fee_record_construct {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_transfer_fee_record_construct_2(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || transfer_fee_record_construct_2(this));
    PLACAR.registrar(igual && r == n, || format!("transfer_fee_record_construct_2 {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_run_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || run_record_construct(this));
    PLACAR.registrar(igual && r == n, || format!("run_record_construct {:x?} x {:x?}", mo, mr));
    r
}
