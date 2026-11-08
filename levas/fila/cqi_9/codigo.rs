// ---------- %%LEVA%%a leva: as regras dos estaduais Paranaense, Pernambucano (Norte), Carioca e Mineiro (bra_reg_parana.cpp, bra_reg_pern.cpp, bra_reg_rio.cpp) ----------

unsafe fn cqi9_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi9_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi9_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras comuns destes estaduais (as três rotinas gravam o mesmo conjunto; mudam as vagas de cima +0xbe e os
/// rebaixados +0xc1; os três têm dois turnos): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates
/// confronto direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb em 0.
unsafe fn bra_reg_parana_set_state_league_rules(o: usize, cima: u8, rebaixados: u8) {
    cqi9_w(o, 0x3c, 2);
    cqi9_b(o, 0xc4, 2);
    cqi9_w(o, 0xab, 0);
    cqi9_w(o, 0xdb, 0);
    cqi9_b(o, 0xc7, 0);
    cqi9_b(o, 0xbe, cima);
    cqi9_b(o, 0xbf, 0);
    cqi9_b(o, 0xc0, 0);
    cqi9_b(o, 0xc2, 3);
    cqi9_b(o, 0xc5, 3);
    cqi9_b(o, 0x4a, 3);
    cqi9_b(o, 0xc3, 1);
    cqi9_b(o, 0x42, 1);
    cqi9_b(o, 0xc6, 1);
    cqi9_b(o, 0xc1, rebaixados);
    cqi9_d(o, 0x1c, 0xffff_ffff);
    cqi9_d(o, 0x20, 0xffff_ffff);
    cqi9_b(o, 0x49, 7);
    cqi9_w(o, 0xd9, 8);
}

/// O Paranaense (0x437fc0): nenhuma vaga de cima na tabela geral (os turnos têm estágios próprios), 4 rebaixados.
unsafe fn parana_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 4); // cqi9: o Paranaense
}

/// O Pernambucano (0x439430, também o Norte): 4 vagas de cima, 2 rebaixados.
unsafe fn pernambucano_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 4, 2); // cqi9: o Pernambucano
}

/// O Carioca (0x43a600, também o Mineiro): nenhuma vaga de cima na tabela geral, 2 rebaixados.
unsafe fn carioca_rules(o: usize) {
    bra_reg_parana_set_state_league_rules(o, 0, 2); // cqi9: o Carioca
}

/// O calendário da fase principal pelo slot 16 (`create_schedule`, thiscall, ret 0x10, conferido nos cinco alvos das
/// vtables destas rotinas): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqi9_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi9_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os calendários não gravam nada
/// no objeto além delas (nenhum `mov [edi+..]` nos cinco).
fn cqi9_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// { 0x437fc0 } (thiscall, bra_reg_parana.cpp): `BraRegParana::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
unsafe extern "thiscall" fn bra_reg_parana_set_up_rules(this: usize) -> u32 {
    parana_rules(this);
    crate::comparison::branch("cqi_9 regras do paranaense");
    cqi9_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn compare_bra_reg_parana_set_up_rules(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this), || {
        parana_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_parana_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x439430 } (thiscall, bra_reg_pern.cpp): `BraRegPern::set_up_rules`, nas vtables do Norte (0x968338) e do
/// Pernambucano (0x968554); devolve 1.
unsafe extern "thiscall" fn bra_reg_pern_set_up_rules(this: usize) -> u32 {
    pernambucano_rules(this);
    crate::comparison::branch("cqi_9 regras do pernambucano");
    cqi9_calendario(this);
    1
}

/// Comparador: como o do Paranaense.
unsafe extern "thiscall" fn compare_bra_reg_pern_set_up_rules(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || {
        pernambucano_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_pern_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x43a600 } (thiscall, bra_reg_rio.cpp): `BraRegRio::set_up_rules`, nas vtables do Mineiro (0x968284) e do
/// Carioca (0x968608); devolve 1.
unsafe extern "thiscall" fn bra_reg_rio_set_up_rules(this: usize) -> u32 {
    carioca_rules(this);
    crate::comparison::branch("cqi_9 regras do carioca");
    cqi9_calendario(this);
    1
}

/// Comparador: como o do Paranaense.
unsafe extern "thiscall" fn compare_bra_reg_rio_set_up_rules(this: usize) -> u32 {
    let reg = cqi9_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this), || {
        carioca_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("bra_reg_rio_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
