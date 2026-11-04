// ---------- %%LEVA%%a leva: as regras das três divisões nacionais do Brasil (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------

unsafe fn cqi7_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqi7_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqi7_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x429b50, 0x42c140, 0x42d620): estágio -1, as saídas +0xa9 e +0x3a (palavras), sem a marca (0); o ponteiro
/// devolvido vai para +0xba.
unsafe fn cqi7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqi7_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba), que o ensaio não chama: os
/// três calendários não gravam nada no objeto além delas (lido nas três rotinas: nenhum `mov [edi+..]`).
fn cqi7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras da Série A (o que a 0x42a5a0 grava antes do calendário).
unsafe fn cqi7_regras_serie_a(o: usize) {
    cqi7_w(o, 0x3c, 1); // um turno
    cqi7_b(o, 0xc2, 3); // cqi7: os pontos por vitória da Série A
    cqi7_b(o, 0xc3, 1); // pontos por empate
    cqi7_b(o, 0xc4, 2); // categoria das partidas: liga
    cqi7_b(o, 0x42, 1); // clube nacional
    cqi7_b(o, 0xc5, 3); // desempates: confronto direto, saldo, nenhum
    cqi7_b(o, 0xc6, 1);
    cqi7_b(o, 0xc7, 0);
    cqi7_b(o, 0xbe, 4); // vagas de cima
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 4); // rebaixados
    cqi7_d(o, 0x1c, 0xffff_ffff); // nenhuma liga acima
    cqi7_b(o, 0x4a, 3); // substituições
    cqi7_d(o, 0x20, *(0x9cf9e0 as *const u32)); // cai para a Série B
    cqi7_w(o, 0xd9, 8); // bit 0x8: os de cima vão para a fase seguinte (sem acesso, sem faixa de campeão)
    cqi7_b(o, 0x52, 2); // numeração fixa do elenco
    cqi7_b(o, 0x49, 7); // reservas no banco
}

/// As regras da Série B (0x42cb70).
unsafe fn cqi7_regras_serie_b(o: usize) {
    cqi7_w(o, 0x3c, 1);
    cqi7_b(o, 0xc2, 3);
    cqi7_b(o, 0xc3, 1); // cqi7: os pontos por empate da Série B
    cqi7_b(o, 0xc4, 2);
    cqi7_b(o, 0x42, 1);
    cqi7_b(o, 0xc5, 3);
    cqi7_b(o, 0xc6, 1);
    cqi7_b(o, 0xc7, 0);
    cqi7_b(o, 0xbe, 4); // quatro sobem
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 4); // quatro caem
    cqi7_d(o, 0x1c, *(0x9cf9ec as *const u32)); // sobe para a Série A
    cqi7_w(o, 0xd9, 2); // bit 0x2: as vagas de cima são de acesso
    cqi7_b(o, 0x52, 2);
    cqi7_b(o, 0x4a, 3);
    cqi7_d(o, 0x20, *(0x9cf9ac as *const u32)); // cai para a Série C
    cqi7_b(o, 0x49, 7);
}

/// As regras da Série C (0x42de50).
unsafe fn cqi7_regras_serie_c(o: usize) {
    cqi7_w(o, 0x3c, 2); // cqi7: os turnos da Série C
    cqi7_b(o, 0xc2, 3);
    cqi7_b(o, 0xc3, 1);
    cqi7_b(o, 0xc4, 2);
    cqi7_b(o, 0x42, 1);
    cqi7_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    cqi7_b(o, 0xc6, 2);
    cqi7_b(o, 0xc7, 4);
    cqi7_b(o, 0xbe, 2);
    cqi7_b(o, 0xbf, 0);
    cqi7_b(o, 0xc0, 0);
    cqi7_b(o, 0xc1, 2);
    cqi7_b(o, 0x4a, 3);
    cqi7_b(o, 0x52, 2);
    cqi7_d(o, 0x1c, *(0x9cf9e0 as *const u32)); // sobe para a Série B
    cqi7_d(o, 0x20, 0xffff_ffff); // nenhuma liga abaixo
    cqi7_w(o, 0xd9, 8);
    cqi7_b(o, 0x49, 7);
}

/// { 0x42a5a0 } (thiscall, bra_nat_first.cpp): `BraNatFirst::set_up_rules` (slot 36), as regras da Série A e o
/// calendário da fase principal; devolve 1.
unsafe extern "thiscall" fn cqi7_regras_da_serie_a(this: usize) -> u32 {
    cqi7_regras_serie_a(this);
    crate::comparacao::ramo("cqi_7 regras da serie a");
    cqi7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto inteiro
/// menos as saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_a(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this), || {
        cqi7_regras_serie_a(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_a({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x42cb70 } (thiscall, bra_nat_second.cpp): `BraNatSecond::set_up_rules`, as regras da Série B e o calendário.
unsafe extern "thiscall" fn cqi7_regras_da_serie_b(this: usize) -> u32 {
    cqi7_regras_serie_b(this);
    crate::comparacao::ramo("cqi_7 regras da serie b");
    cqi7_calendario(this);
    1
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_b(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || {
        cqi7_regras_serie_b(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_b({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x42de50 } (thiscall, bra_nat_third.cpp): `BraNatThird::set_up_rules`, as regras da Série C e o calendário.
unsafe extern "thiscall" fn cqi7_regras_da_serie_c(this: usize) -> u32 {
    cqi7_regras_serie_c(this);
    crate::comparacao::ramo("cqi_7 regras da serie c");
    cqi7_calendario(this);
    1
}

/// Comparador: como o da Série A.
unsafe extern "thiscall" fn comparar_cqi7_regras_da_serie_c(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this), || {
        cqi7_regras_serie_c(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqi7_regras_da_serie_c({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
