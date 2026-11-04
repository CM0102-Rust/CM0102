// ---------- %%LEVA%%a leva: as regras dos estaduais do Centro-Oeste, Gaúcho (Nordeste, Paulista), Baiano e Catarinense (bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_santa.cpp) ----------

unsafe fn cqi8_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi8_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi8_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras comuns dos estaduais (as três rotinas gravam o mesmo conjunto; mudam os turnos +0x3c, as vagas de cima
/// +0xbe e os rebaixados +0xc1): vitória 3, empate 1, categoria 2 (liga), clube nacional, desempates confronto
/// direto e saldo, sem play-offs na tabela, sem liga acima nem abaixo, palavra de regras 8, 7 reservas e
/// 3 substituições, e as palavras +0xab e +0xdb (copiadas em cada partida: +0x38 e +0x36) em 0.
unsafe fn cqi8_regras(o: usize, turnos: u16, cima: u8, rebaixados: u8) {
    cqi8_w(o, 0xab, 0);
    cqi8_w(o, 0xdb, 0);
    cqi8_b(o, 0xc7, 0);
    cqi8_b(o, 0xbf, 0);
    cqi8_b(o, 0xc0, 0);
    cqi8_b(o, 0xc2, 3);
    cqi8_b(o, 0xc5, 3);
    cqi8_b(o, 0xc1, rebaixados);
    cqi8_b(o, 0x4a, 3);
    cqi8_b(o, 0xbe, cima);
    cqi8_w(o, 0xd9, 8);
    cqi8_w(o, 0x3c, turnos);
    cqi8_b(o, 0xc3, 1);
    cqi8_b(o, 0xc4, 2);
    cqi8_b(o, 0x42, 1);
    cqi8_b(o, 0xc6, 1);
    cqi8_d(o, 0x1c, 0xffff_ffff);
    cqi8_d(o, 0x20, 0xffff_ffff);
    cqi8_b(o, 0x49, 7);
}

/// O Centro-Oeste (0x430ff0): turno único, 8 vagas de cima (o mata-mata de 8), 3 rebaixados.
unsafe fn cqi8_regras_centro(o: usize) {
    cqi8_regras(o, 1, 8, 3); // cqi8: o Centro-Oeste
}

/// O Gaúcho (0x4322d0, também o Nordeste e o Paulista): turno único, 4 vagas de cima, 2 rebaixados.
unsafe fn cqi8_regras_gaucho(o: usize) {
    cqi8_regras(o, 1, 4, 2); // cqi8: o Gaúcho
}

/// O Catarinense (0x43bc70, também o Baiano): dois turnos, 4 vagas de cima, 1 rebaixado.
unsafe fn cqi8_regras_catarinense(o: usize) {
    cqi8_regras(o, 2, 4, 1); // cqi8: o Catarinense
}

/// O calendário da fase principal pelo slot 16 (`create_schedule`, thiscall, ret 0x10, conferido nos seis alvos das
/// vtables destas rotinas): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqi8_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi8_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os calendários não gravam nada
/// no objeto além delas (nenhum `mov [edi+..]` nos seis).
fn cqi8_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// { 0x430ff0 } (thiscall, bra_reg_central.cpp): `BraRegCentral::set_up_rules` (slot 36), as regras e o calendário;
/// devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    cqi8_regras_centro(this);
    crate::comparacao::ramo("cqi_8 regras do centro-oeste");
    cqi8_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_centro_oeste(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this), || {
        cqi8_regras_centro(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_centro_oeste({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x4322d0 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::set_up_rules`, nas vtables do Gaúcho (0x96811c), do
/// Nordeste (0x9683ec) e do Paulista (0x968770); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_gaucho(this: usize) -> u32 {
    cqi8_regras_gaucho(this);
    crate::comparacao::ramo("cqi_8 regras do gaucho");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_gaucho(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || {
        cqi8_regras_gaucho(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_gaucho({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x43bc70 } (thiscall, bra_reg_santa.cpp): `BraRegSanta::set_up_rules`, nas vtables do Baiano (0x967fb4) e do
/// Catarinense (0x9686bc); devolve 1.
unsafe extern "thiscall" fn cqi8_regras_do_catarinense(this: usize) -> u32 {
    cqi8_regras_catarinense(this);
    crate::comparacao::ramo("cqi_8 regras do catarinense");
    cqi8_calendario(this);
    1
}

/// Comparador: como o do Centro-Oeste.
unsafe extern "thiscall" fn comparar_cqi8_regras_do_catarinense(this: usize) -> u32 {
    let reg = cqi8_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this), || {
        cqi8_regras_catarinense(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi8_regras_do_catarinense({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
