// ---------- 282a leva: a NSL australiana: a fase seguinte, a série final dos seis primeiros e a grande final (aus_nsl.cpp) ----------

unsafe fn cqr3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube da linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`.
unsafe fn cqr3_linha(t: usize, i: usize) -> usize {
    cqr3_d(cqr3_d(t + 0xb1) + i * 0x3b)
}

/// { 0x411f70 } (thiscall, ret 0, aus_nsl.cpp): `AusNsl::advance_stage()`, a fase seguinte da NSL (a temporada regular,
/// a fase −1; a série final dos seis primeiros, a fase 0; a grande final, a fase 1). Na última fase (+0x30 ≥ +0x2c − 1,
/// com sinal): a caixa de erro da linha 0x2f0 e nada mais. Senão +0x30 sobe um; ao chegar a 0 monta a série final
/// (0x412010) e a 1, a grande final (0x4121c0), as duas em call direto com o ecx da liga.
unsafe extern "thiscall" fn cqr3_fase_seguinte_da_nsl(this: usize) {
    let fase = cqr3_d(this + 0x30) as i32;
    if fase >= (cqr3_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0x987c20, 0x2f0);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x412010usize);
        f(this);
    } else if fase == 1 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4121c0usize);
        f(this);
    }
}

/// { 0x412010 } (thiscall, ret 0, aus_nsl.cpp; quadro SEH só para o new): `AusNsl::create_finals_series_stage()`, a
/// série final da NSL, do fim da temporada regular.
/// 1) Os seis primeiros da tabela da liga (as linhas 0 a 5, na ordem) num vetor local de 6.
/// 2) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, 0, &rodadas, &extra, 0), ret 0x10).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, 6, o vetor, 1, +4, a tabela das rodadas, as rodadas (word), os pontos +0xc2 e +0xc3,
///    a categoria +0xc4, &+0xc5, &{2,0,0,0} (duas vagas de cima: os dois que vão à grande final), o ano +0x40 (word),
///    0, o extra (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
///    altos, que o construtor não lê: a cópia passa os valores limpos). Sem memória: 0.
/// 4) [+0xc][0] = a fase e a tabela das rodadas liberada (free 0x9452ca). Sem fase: a caixa da linha 0x328 e 0. Com
///    ela: a notícia do sorteio (0x684230 na fase, thiscall, ret 0), +0x30 = 0 e 1.
/// [inferência] A série final é uma liga curta dos seis; os dois primeiros dela vão à grande final.
unsafe extern "thiscall" fn cqr3_serie_final_da_nsl(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [2, 0, 0, 0];
    let mut clubes = [0usize; 6];
    for (i, c) in clubes.iter_mut().enumerate() {
        *c = cqr3_linha(this, i); // cqr3: os seis primeiros da tabela
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr3_d(cqr3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, 6, clubes.as_ptr(), 1, cqr3_d(this + 4), t, rodadas as u16 as u32, cqr3_b(this + 0xc2) as u32,
                  cqr3_b(this + 0xc3) as u32, cqr3_b(this + 0xc4) as u32, this + 0xc5, vagas.as_ptr(), cqr3_w(this + 0x40) as u16 as u32, 0,
                  extra as u16 as u32, cqr3_b(this + 0x51) as u32, 1, 0, cqr3_w(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned(cqr3_d(this + 0xc) as *mut usize, fase);
    soltar(t);
    let fase = cqr3_d(cqr3_d(this + 0xc));
    if fase == 0 {
        aviso_em(0x987c20, 0x328);
        return 0;
    }
    crate::comparacao::ramo("cqr_3 serie final");
    noticia(fase);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
    1
}

/// { 0x4121c0 } (thiscall, ret 0, aus_nsl.cpp; quadro SEH só para o new): `AusNsl::create_grand_final_stage()`, a grande
/// final da NSL, do fim da série final: os dois primeiros da tabela da série final ([+0xc][0], linhas 0 e 1, nessa
/// ordem) numa lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x348 e volta); a tabela das rodadas
/// pela virtual +0x3c (thiscall(this, 1, &rodadas, &extra, 0), ret 0x10); a fase (operator new 0x944cf1(0xb2) e o
/// construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40, 1, 1,
/// o extra, 0x14, 0, 0, 0 e sementes nulas; a cópia passa as words limpas) em [+0xc][1] e a tabela liberada (free
/// 0x9452ca). Sem fase: a caixa da linha 0x35c e volta (sem liberar a lista). Com ela: Cup::set_up_round (0x51c800,
/// thiscall(fase, 0), ret 4) e a lista liberada. A fase atual (+0x30 = 1) já foi gravada pela fase seguinte.
/// [inferência] A grande final é um jogo só, com o primeiro da série final como mandante.
unsafe extern "thiscall" fn cqr3_grande_final_da_nsl(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0x987c20, 0x348);
    }
    let serie = cqr3_d(cqr3_d(this + 0xc));
    core::ptr::write_unaligned(lista as *mut usize, cqr3_linha(serie, 0)); // cqr3: o primeiro da série final
    core::ptr::write_unaligned((lista + 4) as *mut usize, cqr3_linha(serie, 1)); // cqr3: o segundo da série final
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr3_d(cqr3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 1, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista, rodadas as u16 as u32, cqr3_d(this + 4), t, cqr3_w(this + 0x40) as u16 as u32, 1, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqr3_d(this + 0xc) + 4) as *mut usize, fase);
    soltar(t);
    let fase = cqr3_d(cqr3_d(this + 0xc) + 4);
    if fase == 0 {
        return aviso_em(0x987c20, 0x35c);
    }
    crate::comparacao::ramo("cqr_3 grande final");
    montar(fase, 0);
    soltar(lista);
}

unsafe extern "thiscall" fn comparar_cqr3_fase_seguinte_da_nsl(this: usize) {
    let antes = cqr3_d(this + 0x30) as i32;
    let ultima = antes >= (cqr3_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1993)(this);
    let depois = cqr3_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("cqr3_fase_seguinte_da_nsl({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da série final: só a original (aloca a tabela das rodadas e a fase e manda a notícia), com o plano feito
/// antes: os seis primeiros da tabela. Depois, a devolução 1, a fase na vaga 0 com +0x3e = 6, +4 = o +4 da liga e os
/// seis na tabela dela (+0xb1, como conjunto), e +0x30 = 0.
unsafe extern "thiscall" fn comparar_cqr3_serie_final_da_nsl(this: usize) -> u32 {
    let mut seis = [0usize; 6];
    for (i, c) in seis.iter_mut().enumerate() {
        *c = cqr3_linha(this, i);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1994)(this);
    let fase = cqr3_d(cqr3_d(this + 0xc));
    let igual = r == 1 && fase != 0 && cqr3_w(fase + 0x3e) == 6 && cqr3_d(fase + 4) == cqr3_d(this + 4) && cqr3_d(this + 0x30) == 0 && {
        let mut nela = [0usize; 6];
        for (i, c) in nela.iter_mut().enumerate() {
            *c = cqr3_linha(fase, i);
        }
        let (mut a, mut b) = (seis, nela);
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    PLACAR.registrar(igual, || format!("cqr3_serie_final_da_nsl({:#x}) r={} fase={:#x} seis={:x?}", this, r, fase, seis));
    r
}

/// O comparador da grande final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os dois primeiros da série final. Depois, a fase da segunda vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (+0xa7, 6 bytes; como conjunto).
unsafe extern "thiscall" fn comparar_cqr3_grande_final_da_nsl(this: usize) {
    let serie = cqr3_d(cqr3_d(this + 0xc));
    let (a, b) = (cqr3_linha(serie, 0), cqr3_linha(serie, 1));
    orig::<unsafe extern "thiscall" fn(usize)>(1995)(this);
    let fase = cqr3_d(cqr3_d(this + 0xc) + 4);
    let igual = fase != 0 && {
        let l = cqr3_d(fase + 0xa7);
        let (x, y) = (cqr3_d(l), cqr3_d(l + 6));
        cqr3_w(fase + 0x3e) == 2 && cqr3_d(fase + 4) == cqr3_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    PLACAR.registrar(igual, || format!("cqr3_grande_final_da_nsl({:#x}) finalistas={:#x},{:#x} fase={:#x}", this, a, b, fase));
}

// ---------- 274a leva: as regras da NSL australiana e da MLS, e o histórico da temporada dos estaduais e das Séries A e B (aus_nsl.cpp, usa_mls.cpp, bra_reg_gaucho.cpp, arg_second.cpp) ----------

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
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1724)(this), || {
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
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xee)], || orig::<unsafe extern "thiscall" fn(usize) -> u32>(1725)(this), || cqj8_regras_da_mls(this));
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
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1726)(this)
}

/// { 0x408eb0 } (thiscall, arg_second.cpp): `ArgSecond::record_season_honours` (slot 13 da segunda divisão argentina e
/// das Séries A e B): só o histórico do primeiro estágio, em salto final; devolve o eax dele.
unsafe extern "thiscall" fn cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_8 historico do primeiro estagio");
    cqj8_historico_do_estagio(this)
}

/// Comparador: só a original (ponytail: grava o histórico e aloca o registro; não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_cqj8_historico_do_primeiro_estagio(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(1727)(this)
}

