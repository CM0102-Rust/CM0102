// ---------- %%LEVA%%a leva: as regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B (aus_nsl.cpp, usa_mls.cpp, bra_reg_gaucho.cpp, arg_second.cpp) ----------

unsafe fn cqj8_b(o: usize, d: usize, v: u8) {
    *((o + d) as *mut u8) = v;
}

unsafe fn cqj8_w(o: usize, d: usize, v: u16) {
    core::ptr::write_unaligned((o + d) as *mut u16, v);
}

unsafe fn cqj8_d(o: usize, d: usize, v: u32) {
    core::ptr::write_unaligned((o + d) as *mut u32, v);
}

/// As regras da NSL (o que a 0x411ce0 grava antes do calendário).
unsafe fn cqj8_regras_nsl(o: usize) {
    cqj8_w(o, 0xab, 0);
    cqj8_b(o, 0xc8, 0);
    cqj8_b(o, 0xbf, 0);
    cqj8_b(o, 0xc0, 0);
    cqj8_b(o, 0xc1, 0);
    cqj8_b(o, 0xc4, 2);
    cqj8_b(o, 0xc6, 2);
    cqj8_b(o, 0x52, 2);
    cqj8_w(o, 0x3c, 3); // três turnos
    cqj8_b(o, 0xc2, 3); // cqj8: os pontos por vitória da NSL
    cqj8_b(o, 0x4a, 3);
    cqj8_b(o, 0xc3, 1);
    cqj8_b(o, 0x42, 1);
    cqj8_b(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    cqj8_b(o, 0xc7, 4);
    cqj8_b(o, 0xbe, 6); // os seis da fase final
    cqj8_d(o, 0x1c, 0xffff_ffff);
    cqj8_d(o, 0x20, 0xffff_ffff);
    cqj8_w(o, 0xd9, 0x28); // 0x8 fase seguinte, 0x20 eliminados marcados
    cqj8_b(o, 0x49, 4);
}

/// As regras da MLS (0x908660; sem calendário: os jogos da MLS são montados à mão pela 0x9086e0).
unsafe fn cqj8_regras_mls(o: usize) {
    cqj8_b(o, 0xc4, 2);
    cqj8_b(o, 0xc6, 2);
    cqj8_b(o, 0xc2, 3);
    cqj8_b(o, 0xc8, 0);
    cqj8_b(o, 0xbf, 0);
    cqj8_b(o, 0xc0, 0);
    cqj8_b(o, 0xc1, 0);
    cqj8_b(o, 0xc3, 1); // cqj8: os pontos por empate da MLS
    cqj8_w(o, 0xab, 0xc2);
    cqj8_b(o, 0x42, 1);
    cqj8_b(o, 0xc5, 1); // desempates: saldo, gols pró, confronto direto
    cqj8_b(o, 0xc7, 3);
    cqj8_b(o, 0xbe, 1);
    cqj8_d(o, 0x1c, 0xffff_ffff);
    cqj8_d(o, 0x20, 0xffff_ffff);
    cqj8_w(o, 0xd9, 0x48); // 0x8 fase seguinte, 0x40 rodadas em ordem fixa
    cqj8_w(o, 0x3c, 4);
    cqj8_w(o, 0xa7, 0x20); // 32 jogos por time
}

/// { 0x411ce0 } (thiscall, aus_nsl.cpp): `AusNsl::set_up_rules` (slot 36), as regras da NSL e o calendário da fase
/// principal pelo slot 16 (`create_schedule` 0x4111a0, thiscall, ret 0x10: estágio -1, as saídas +0xa9 e +0x3a, sem
/// a marca; o ponteiro vai para +0xba); devolve 1.
unsafe extern "thiscall" fn cqj8_regras_da_nsl(this: usize) -> u32 {
    cqj8_regras_nsl(this);
    crate::comparacao::ramo("cqj_8 regras da nsl");
    let vt = *(this as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(this, -1, this + 0xa9, this + 0x3a, 0);
    cqj8_d(this, 0xba, r as u32);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto da liga
/// (0xee) menos as saídas do calendário (+0x3a, +0xa9, +0xba; a 0x4111a0 não grava nada no objeto além delas); a
/// devolução.
unsafe extern "thiscall" fn comparar_cqj8_regras_da_nsl(this: usize) -> u32 {
    let o = this;
    let reg = [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this), || {
        cqj8_regras_nsl(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("cqj8_regras_da_nsl({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// { 0x908660 } (thiscall, usa_mls.cpp): `UsaMls::set_up_rules`, as regras da MLS; devolve 1 (o eax que ela usa como
/// valor).
unsafe extern "thiscall" fn cqj8_regras_da_mls(this: usize) -> u32 {
    cqj8_regras_mls(this);
    crate::comparacao::ramo("cqj_8 regras da mls");
    1
}

/// Comparador: lado a lado sobre o objeto da liga (0xee; só grava nele); a devolução.
unsafe extern "thiscall" fn comparar_cqj8_regras_da_mls(this: usize) -> u32 {
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xee)], || orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || cqj8_regras_da_mls(this));
    PLACAR.registrar(igual && r == n, || format!("cqj8_regras_da_mls({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// O histórico do primeiro estágio: o slot 13 (+0x30, `record_season_honours`, thiscall sem argumentos) do estágio
/// [[this+0xc]].
unsafe fn cqj8_historico_do_estagio(this: usize) -> u32 {
    let e = *(*((this + 0xc) as *const usize) as *const usize);
    let vt = *(e as *const usize);
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(*((vt + 0x30) as *const usize));
    f(e)
}

/// { 0x432970 } (thiscall, bra_reg_gaucho.cpp): `BraRegGaucho::record_season_honours` (slot 13 de nove estaduais: Baiano,
/// Centro-Oeste, Gaúcho, Goiano, Norte, Nordeste, Pernambucano, Catarinense e Paulista): o histórico do primeiro estágio
/// e, em salto final, a 0x42bba0 (`bra_nat_first_fill_champions_cup_entrants`, cdecl sem argumentos: quando os doze
/// estaduais terminam, põe os campeões na Copa dos Campeões); devolve o eax dela.
unsafe extern "thiscall" fn cqj8_historico_do_estadual(this: usize) -> u32 {
    cqj8_historico_do_estagio(this); // cqj8: o histórico do estadual
    crate::comparacao::ramo("cqj_8 historico do estadual");
    let campeoes: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x42bba0usize);
    campeoes()
}

/// Comparador: só a original (ponytail: grava o histórico, aloca o registro dele e pode montar a Copa dos Campeões; não
/// pode rodar duas vezes). A cópia roda no modo Rust (contador "cqj_8 historico do estadual") e é provada pelas
/// mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_cqj8_historico_do_estadual(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this)
}

/// { 0x408eb0 } (thiscall, arg_second.cpp): `ArgSecond::record_season_honours` (slot 13 da segunda divisão argentina e
/// das Séries A e B): só o histórico do primeiro estágio, em salto final; devolve o eax dele.
unsafe extern "thiscall" fn cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_8 historico do primeiro estagio");
    cqj8_historico_do_estagio(this)
}

/// Comparador: só a original (ponytail: grava o histórico e aloca o registro; não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%)(this)
}
