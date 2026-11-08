// ---------- %%LEVA%%a leva: as regras do Goiano e das duas divisões da J-League (bra_reg_goias.cpp, jap_j1.cpp, jap_j2.cpp) ----------

unsafe fn cqj7_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqj7_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqj7_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x432e50, 0x662e30, 0x664780): estágio -1, as saídas +0xa9 e +0x3a, sem a marca; o ponteiro vai para +0xba.
unsafe fn cqj7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    cqj7_d(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba): os três calendários não gravam
/// nada no objeto além delas (nenhum `mov [edi+..]`).
fn cqj7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras do Goiano (o que a 0x433640 grava antes do calendário).
unsafe fn cqj7_regras_goiano(o: usize) {
    cqj7_w(o, 0xab, 0);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0xc7, 0);
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc2, 3);
    cqj7_b(o, 0xc5, 3); // cqj7: o primeiro desempate do Goiano
    cqj7_b(o, 0x4a, 3);
    cqj7_b(o, 0xbe, 8); // os 8 classificados
    cqj7_w(o, 0xd9, 8);
    cqj7_w(o, 0x3c, 1);
    cqj7_b(o, 0xc3, 1);
    cqj7_b(o, 0xc4, 2);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc6, 1);
    cqj7_b(o, 0xc1, 4);
    cqj7_d(o, 0x1c, 0xffff_ffff);
    cqj7_d(o, 0x20, 0xffff_ffff);
    cqj7_b(o, 0x49, 7);
}

/// As regras da J1 (0x663620).
unsafe fn cqj7_regras_j1(o: usize) {
    cqj7_w(o, 0x3c, 2);
    cqj7_b(o, 0xc2, 3);
    cqj7_b(o, 0xc3, 1); // cqj7: os pontos por empate da J1
    cqj7_b(o, 0xc4, 2);
    cqj7_w(o, 0xab, 0x82);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora, nenhum
    cqj7_b(o, 0xc6, 2);
    cqj7_b(o, 0xc7, 4);
    cqj7_b(o, 0xc8, 0);
    cqj7_b(o, 0xbe, 0);
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc1, 2); // dois caem para a J2
    cqj7_d(o, 0x1c, 0xffff_ffff);
    cqj7_b(o, 0x52, 2);
    cqj7_b(o, 0x4a, 3);
    cqj7_d(o, 0x20, *(0x9cf768 as *const u32)); // cai para a J2
    cqj7_w(o, 0xd9, 8);
    cqj7_b(o, 0x49, 5);
}

/// As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
unsafe fn cqj7_regras_j2(o: usize) {
    cqj7_w(o, 0x3c, 4); // quatro turnos
    cqj7_b(o, 0xc2, 3); // cqj7: os pontos por vitória da J2
    cqj7_b(o, 0xc3, 1);
    cqj7_b(o, 0xc4, 2);
    cqj7_w(o, 0xab, 0x82);
    cqj7_w(o, 0xdb, 0);
    cqj7_b(o, 0x42, 1);
    cqj7_b(o, 0xc5, 1);
    cqj7_b(o, 0xc6, 2);
    cqj7_b(o, 0xc7, 4);
    cqj7_b(o, 0xc8, 0);
    cqj7_b(o, 0xbe, 2); // dois sobem para a J1
    cqj7_b(o, 0xbf, 0);
    cqj7_b(o, 0xc0, 0);
    cqj7_b(o, 0xc1, 0);
    cqj7_d(o, 0x1c, *(0x9cf65c as *const u32)); // sobe para a J1
    cqj7_b(o, 0x4a, 3);
    cqj7_d(o, 0x20, 0xffff_ffff);
    cqj7_b(o, 0x49, 5);
}

/// { 0x433640 } (thiscall, bra_reg_goias.cpp): `BraRegGoias::set_up_rules` (slot 36), as regras do Goiano e o
/// calendário; devolve 1.
unsafe extern "thiscall" fn bra_reg_goias_set_up_rules(this: usize) -> u32 {
    cqj7_regras_goiano(this);
    crate::comparacao::ramo("cqj_7 regras do goiano");
    cqj7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto menos as
/// saídas do calendário; a devolução.
unsafe extern "thiscall" fn comparar_bra_reg_goias_set_up_rules(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this), || {
        cqj7_regras_goiano(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_goias_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x663620 } (thiscall, jap_j1.cpp): `JapJ1::set_up_rules`, as regras da J1 e o calendário; devolve 1.
unsafe extern "thiscall" fn jap_j1_set_up_rules(this: usize) -> u32 {
    cqj7_regras_j1(this);
    crate::comparacao::ramo("cqj_7 regras da j1");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_jap_j1_set_up_rules(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || {
        cqj7_regras_j1(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("jap_j1_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x665140 } (thiscall, jap_j2.cpp): `JapJ2::set_up_rules`, as regras da J2 e o calendário; devolve 1.
unsafe extern "thiscall" fn bra_reg_goias_set_up_rules_665140(this: usize) -> u32 {
    cqj7_regras_j2(this);
    crate::comparacao::ramo("cqj_7 regras da j2");
    cqj7_calendario(this);
    1
}

/// Comparador: como o do Goiano.
unsafe extern "thiscall" fn comparar_bra_reg_goias_set_up_rules_665140(this: usize) -> u32 {
    let reg = cqj7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this), || {
        cqj7_regras_j2(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_reg_goias_set_up_rules_665140({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
