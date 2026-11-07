// ---------- 282a leva: os turnos do Carioca, do Mineiro, do Paranaense e da J1: a fase de pontos corridos de cada turno (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, jap_j1.cpp) ----------

unsafe fn cqr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O turno `k` de uma liga de dois turnos (o slot create_half_season_stage; o mesmo molde nos três estaduais e na J1,
/// conferido instrução a instrução: só mudam o fonte, a linha da caixa e o 20º argumento do construtor, 0xb nos
/// estaduais e 0xf na J1). Chamado do resultado de cada jogo (0x43af20 e as irmãs: push dl = +0x30 + 1; mov ecx ->
/// thiscall, ret 4; a original lê o argumento inteiro, usa o byte baixo e o empurra inteiro ao construtor).
/// 1) Os clubes da tabela da liga (+0xb1, linhas de 0x3b bytes, as +0x3e linhas) num vetor (malloc 0x944e46 de
///    4 × +0x3e; sem memória: a caixa e volta).
/// 2) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, 1, +4, 0, 0, os pontos por vitória +0xc2 (`pv`), por empate +0xc3, a
///    categoria dos jogos +0xc4, &+0xc5 (os critérios de desempate), &{1,0,0,0} (uma vaga de cima: o campeão do
///    turno), o ano +0x40 (word), k, o id 0x439 (primeiro turno) ou 0x43a (segundo), 0, 1, 0, 8, `vinte`, 0, 2; a
///    original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores
///    limpos; o k vai inteiro, como chegou). Sem memória para a fase: 0 na vaga, sem caixa.
/// 3) [+0xc][k] = a fase, +0x30 = k (o byte com sinal) e o vetor liberado (free 0x9452ca). O quadro SEH da original só
///    cobre o construtor (sem destrutor no caminho normal).
unsafe fn cqr1_turno(this: usize, k: u32, arquivo: usize, linha: u32, vinte: u32, pv: usize, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, u32, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [1, 0, 0, 0];
    let n = cqr1_w(this + 0x3e);
    let clubes = alocar(((n as i32) << 2) as u32);
    if clubes == 0 {
        return aviso_em(arquivo, linha);
    }
    let mut i: i16 = 0;
    while i < cqr1_w(this + 0x3e) {
        core::ptr::write_unaligned((clubes + 4 * i as usize) as *mut usize, cqr1_d(cqr1_d(this + 0xb1) + i as usize * 0x3b));
        i += 1;
    }
    let id = 0x439 + (k as u8 != 0) as u32;
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, cqr1_w(this + 0x3e) as u16 as u32, clubes, 1, cqr1_d(this + 4), 0, 0,
                  cqr1_b(this + pv) as u32,
                  cqr1_b(this + 0xc3) as u32, cqr1_b(this + 0xc4) as u32, this + 0xc5, vagas.as_ptr(),
                  cqr1_w(this + 0x40) as u16 as u32, k, id, 0, 1, 0, 8, vinte, 0, 2)
    } else {
        0
    };
    let k = k as u8 as i8 as i32;
    core::ptr::write_unaligned((cqr1_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
    crate::comparacao::ramo(ramo);
    soltar(clubes);
}

/// O comparador de um turno: só a original (ponytail: aloca o vetor e a fase, que monta a tabela do turno), com o plano:
/// depois, a vaga k tem a fase (o operator new não falha na carreira), +0x30 = k e a fase tem os +0x3e clubes da liga.
unsafe fn cqr1_comparar(nome: &str, this: usize, k: u32, original: impl FnOnce()) {
    original();
    let kk = k as u8 as i8 as i32;
    let fase = cqr1_d(cqr1_d(this + 0xc) + 4 * kk as usize);
    let igual = fase != 0 && cqr1_d(this + 0x30) as i32 == kk && cqr1_w(fase + 0x3e) == cqr1_w(this + 0x3e);
    PLACAR.registrar(igual, || format!("{}({:#x}, {}) fase={:#x} +0x30={}", nome, this, kk, fase, cqr1_d(this + 0x30) as i32));
}

/// { 0x43afa0 } (thiscall, ret 4, bra_reg_rio.cpp): `BraRegRio::create_half_season_stage(k)`, um turno do Carioca (os
/// doze clubes; a caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_carioca(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98b014, 0x3d5, 0xb, 0xc2, "cqr_1 turno do carioca") // cqr1: os pontos por vitória no Carioca
}

/// { 0x4351b0 } (thiscall, ret 4, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_half_season_stage(k)`, um turno
/// do Mineiro (a caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_mineiro(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: os pontos por vitória no Mineiro
}

/// { 0x438760 } (thiscall, ret 4, bra_reg_parana.cpp): `BraRegParana::create_half_stage(k)`, um turno do Paranaense (a
/// caixa da linha 0x3d5).
unsafe extern "thiscall" fn cqr1_turno_do_paranaense(this: usize, k: u32) {
    cqr1_turno(this, k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: os pontos por vitória no Paranaense
}

/// { 0x663c20 } (thiscall, ret 4, jap_j1.cpp): `JapJ1::create_half_season_stage(k)`, um turno da J1 (a caixa da linha
/// 0x278; o 20º argumento do construtor é 0xf).
unsafe extern "thiscall" fn cqr1_turno_da_j1(this: usize, k: u32) {
    cqr1_turno(this, k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: os pontos por vitória na J1
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_carioca(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1986);
    cqr1_comparar("cqr1_turno_do_carioca", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_mineiro(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1987);
    cqr1_comparar("cqr1_turno_do_mineiro", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_do_paranaense(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1988);
    cqr1_comparar("cqr1_turno_do_paranaense", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_cqr1_turno_da_j1(this: usize, k: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1989);
    cqr1_comparar("cqr1_turno_da_j1", this, k, || o(this, k))
}

// ---------- 282a leva: a virada da temporada do Carioca, do Mineiro e do Paranaense (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp) ----------

unsafe fn cqr2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual de dois turnos e final (`start_new_season`, slot 3, +0x08; o mesmo molde no
/// Carioca, no Mineiro e no Paranaense: só mudam o fonte e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o vetor das fases liberado (free) e trocado por um novo de três vagas (+0x2c = 3, malloc 0x944e46 de 0xc, sem
/// conferir nem zerar: os turnos e a final o preenchem); o ano (+0x40, word) sobe `ano` (o contador `ramo`) e
/// +0x30 = −1; então, cada um com a sua caixa (linhas 0x1d4, 0x1db, 0x1e1 e 0x1e8) se devolver 0: as regras da
/// temporada (virtual +0x8c), a tabela com os doze clubes (`tabela`, thiscall, ret 0), os jogos (League::create_fixtures
/// 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as
/// posições previstas (virtual +0x5c) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0,
/// thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com caixa, a cópia devolve 0 (a original devolve o eax da
/// caixa, que a virada do Brasil não lê).
unsafe fn cqr2_virada(this: usize, tabela: usize, arquivo: usize, ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqr2_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqr2_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqr2_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqr2_d(cqr2_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqr2_d(cqr2_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqr2_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    liberar(cqr2_d(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let v = alocar(0xc);
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqr2_d(cqr2_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, 0x1d4);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, 0x1db);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, 0x1e1);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, 0x1e8);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqr2_d(cqr2_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqr2_d(0xb74340), cqr2_d(this + 4))
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um, +0x2c = 3 e +0x30 = −1.
unsafe fn cqr2_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let igual = depois == antes.wrapping_add(1) && cqr2_d(this + 0x2c) == 3 && cqr2_d(this + 0x30) as i32 == -1;
    PLACAR.registrar(igual, || format!("{}({:#x}) ano antes={} depois={} +0x2c={} +0x30={}", nome, this, antes, depois, cqr2_d(this + 0x2c), cqr2_d(this + 0x30) as i32));
    r
}

/// { 0x43a6a0 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::start_new_season()`, a virada do Carioca (a tabela: 0x43a500).
unsafe extern "thiscall" fn cqr2_virada_do_carioca(this: usize) -> u32 {
    cqr2_virada(this, 0x43a500, 0x98b014, 1, "cqr_2 virada do carioca") // cqr2: o ano seguinte do Carioca
}

/// { 0x434910 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::start_new_season()`, a virada do Mineiro (a tabela: 0x434810).
unsafe extern "thiscall" fn cqr2_virada_do_mineiro(this: usize) -> u32 {
    cqr2_virada(this, 0x434810, 0x98aed0, 1, "cqr_2 virada do mineiro") // cqr2: o ano seguinte do Mineiro
}

/// { 0x438060 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::start_new_season()`, a virada do Paranaense (a tabela: 0x437ec0).
unsafe extern "thiscall" fn cqr2_virada_do_paranaense(this: usize) -> u32 {
    cqr2_virada(this, 0x437ec0, 0x98af98, 1, "cqr_2 virada do paranaense") // cqr2: o ano seguinte do Paranaense
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_carioca(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1990);
    cqr2_comparar("cqr2_virada_do_carioca", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_mineiro(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1991);
    cqr2_comparar("cqr2_virada_do_mineiro", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_paranaense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1992);
    cqr2_comparar("cqr2_virada_do_paranaense", this, || o(this))
}

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

// ---------- 282a leva: a MLS: a fase seguinte, o play-off dos oito e a final da MLS Cup (usa_mls.cpp) ----------

unsafe fn cqr4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`: o clube (+0) e os pontos (+0x19, word).
unsafe fn cqr4_linha(t: usize, i: usize) -> (usize, i16) {
    let l = cqr4_d(t + 0xb1) + i * 0x3b;
    (cqr4_d(l), cqr4_w(l + 0x19))
}

/// Grava o registro de 6 bytes `k` (clube, pontos) do vetor `r`.
fn cqr4_por(r: &mut [u8], k: usize, (c, p): (usize, i16)) {
    r[6 * k..6 * k + 4].copy_from_slice(&(c as u32).to_le_bytes());
    r[6 * k + 4..6 * k + 6].copy_from_slice(&p.to_le_bytes());
}

/// O clube do registro de 6 bytes `k` do vetor `r`.
fn cqr4_clube(r: &[u8], k: usize) -> usize {
    u32::from_le_bytes([r[6 * k], r[6 * k + 1], r[6 * k + 2], r[6 * k + 3]]) as usize
}

/// { 0x909400 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::advance_stage()`, a fase seguinte da MLS (as fases 0 e 1 são
/// as duas outras divisões, que correm junto com a liga; a 2, o play-off; a 3, a final). Na última fase (+0x30 ≥
/// +0x2c − 1, com sinal): a caixa de erro da linha 0x55b e nada mais. Senão: da fase 1 vai à 2 e da 2 à 3 (de outra
/// fase, +0x30 fica); então, na 2, monta o play-off (0x9094c0) e, na 3, a final (0x9097d0), em call direto com o ecx da
/// liga.
unsafe extern "thiscall" fn cqr4_fase_seguinte_da_mls(this: usize) {
    let fase = cqr4_d(this + 0x30) as i32;
    if fase >= (cqr4_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0xaa1fb4, 0x55b);
    }
    if fase == 1 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
    } else if fase == 2 {
        core::ptr::write_unaligned((this + 0x30) as *mut u32, 3);
    }
    match cqr4_d(this + 0x30) {
        2 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9094c0usize);
            f(this);
        }
        3 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9097d0usize);
            f(this);
        }
        _ => {}
    }
}

/// { 0x9094c0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_playoff_stage()`, o play-off
/// da MLS, da fase seguinte.
/// 1) Uma lista de 8 (malloc 0x944e46 de 0x20; sem memória: a caixa da linha 0x58d e volta).
/// 2) Os três campeões de divisão (o primeiro da tabela da liga, da fase 0 e da fase 1), em registros de 6 bytes (o
///    clube e os pontos, +0x19 da linha), ordenados pelos pontos (qsort 0x945333 com 0x90a130: mais pontos primeiro),
///    vão às vagas 1, 3 e 5 da lista.
/// 3) Os outros: o segundo, o terceiro e o quarto de cada tabela (na ordem: liga, fase 0, fase 1, linha a linha), nove
///    registros ordenados pelos pontos do mesmo jeito; os cinco melhores vão às vagas 7, 6, 4, 2 e 0 (o melhor na 7).
///    Os pares do play-off: o melhor campeão contra o quinto melhor dos outros, o segundo campeão contra o quarto, o
///    terceiro campeão contra o terceiro, e o melhor dos outros contra o segundo [inferência: os pares da lista].
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 2, &rodadas, &extra, 0), ret 0x10); a fase (operator
///    new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 8, a lista, as rodadas, +4, a
///    tabela, o ano +0x40, 2, 1, o extra, 0x12, 1, 0, 0 e sementes nulas; a cópia passa as words limpas) em [+0xc][2]
///    e a tabela liberada (free 0x9452ca). Sem fase: a caixa da linha 0x5c4. Com ela: Cup::set_up_round (0x51c800,
///    thiscall(fase, 0), ret 4). A lista não é liberada (a fase fica com ela; a original a deixa assim).
unsafe extern "thiscall" fn cqr4_play_off_da_mls(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let (a, b) = (cqr4_d(cqr4_d(this + 0xc)), cqr4_d(cqr4_d(this + 0xc) + 4));
    let lista = alocar(0x20);
    if lista == 0 {
        return aviso_em(0xaa1fb4, 0x58d);
    }
    let poe = |k: usize, c: usize| core::ptr::write_unaligned((lista + 4 * k) as *mut usize, c);
    let mut r = [0u8; 54];
    cqr4_por(&mut r, 0, cqr4_linha(this, 0));
    cqr4_por(&mut r, 1, cqr4_linha(a, 0));
    cqr4_por(&mut r, 2, cqr4_linha(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    poe(1, cqr4_clube(&r, 0)); // cqr4: o melhor campeão de divisão
    poe(3, cqr4_clube(&r, 1));
    poe(5, cqr4_clube(&r, 2));
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            cqr4_por(&mut r, k, cqr4_linha(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    poe(7, cqr4_clube(&r, 0));
    poe(6, cqr4_clube(&r, 1));
    poe(4, cqr4_clube(&r, 2));
    poe(2, cqr4_clube(&r, 3));
    poe(0, cqr4_clube(&r, 4)); // cqr4: o quinto melhor dos outros
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr4_d(cqr4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 8, lista, rodadas as u16 as u32, cqr4_d(this + 4), t, cqr4_w(this + 0x40) as u16 as u32, 2, 1,
                  extra as u16 as u32, 0x12, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqr4_d(this + 0xc) + 8) as *mut usize, fase);
    soltar(t);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 8);
    if fase == 0 {
        return aviso_em(0xaa1fb4, 0x5c4);
    }
    crate::comparacao::ramo("cqr_4 play-off");
    montar(fase, 0);
}

/// { 0x9097d0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_final_stage()`, a final da MLS
/// Cup, da fase seguinte: os dois primeiros times do play-off ([+0xc][2], os registros de 6 bytes de +0xa7, na ordem)
/// com a marca de vencedor (+5 = 1) numa lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x5e5 e
/// volta); a tabela das rodadas pela virtual +0x3c (thiscall(this, 3, &rodadas, &extra, 0), ret 0x10); a fase
/// (operator new 0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00: a liga, 2, a lista, as rodadas, +4, a tabela, o ano,
/// 3, 1, o extra, 0x14, 0, 0, 0 e sementes nulas) em [+0xc][3] e a tabela liberada. Sem fase: a caixa da linha 0x5fe e
/// volta (sem liberar a lista). Com ela: Cup::set_up_round (thiscall(fase, 0)) e a lista liberada (free 0x9452ca).
unsafe extern "thiscall" fn cqr4_final_da_mls(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let po = cqr4_d(cqr4_d(this + 0xc) + 8);
    let lista = alocar(8);
    if lista == 0 {
        return aviso_em(0xaa1fb4, 0x5e5);
    }
    let mut n = 0i8;
    let mut i = 0i32;
    while i < cqr4_w(po + 0x3e) as i32 && n < 2 {
        let e = cqr4_d(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            core::ptr::write_unaligned((lista + 4 * n as usize) as *mut usize, cqr4_d(e)); // cqr4: os finalistas
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqr4_d(cqr4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 3, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista, rodadas as u16 as u32, cqr4_d(this + 4), t, cqr4_w(this + 0x40) as u16 as u32, 3, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqr4_d(this + 0xc) + 0xc) as *mut usize, fase);
    soltar(t);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 0xc);
    if fase == 0 {
        return aviso_em(0xaa1fb4, 0x5fe);
    }
    crate::comparacao::ramo("cqr_4 final");
    montar(fase, 0);
    soltar(lista);
}

unsafe extern "thiscall" fn comparar_cqr4_fase_seguinte_da_mls(this: usize) {
    let antes = cqr4_d(this + 0x30) as i32;
    let ultima = antes >= (cqr4_d(this + 0x2c) as i32).wrapping_sub(1);
    orig::<unsafe extern "thiscall" fn(usize)>(1996)(this);
    let depois = cqr4_d(this + 0x30) as i32;
    let esperado = if ultima { antes } else if antes == 1 || antes == 2 { antes + 1 } else { antes };
    PLACAR.registrar(depois == esperado, || format!("cqr4_fase_seguinte_da_mls({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela cópia da escolha (os mesmos registros e o mesmo qsort): a fase da terceira vaga existe, com +0x3e = 8, +4
/// = o +4 da liga e os oito na lista dela (+0xa7, 6 bytes; como conjunto).
unsafe extern "thiscall" fn comparar_cqr4_play_off_da_mls(this: usize) {
    let ordenar: unsafe extern "C" fn(*mut u8, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let (a, b) = (cqr4_d(cqr4_d(this + 0xc)), cqr4_d(cqr4_d(this + 0xc) + 4));
    let mut plano = [0usize; 8];
    let mut r = [0u8; 54];
    cqr4_por(&mut r, 0, cqr4_linha(this, 0));
    cqr4_por(&mut r, 1, cqr4_linha(a, 0));
    cqr4_por(&mut r, 2, cqr4_linha(b, 0));
    ordenar(r.as_mut_ptr(), 3, 6, 0x90a130);
    for k in 0..3 {
        plano[k] = cqr4_clube(&r, k);
    }
    let mut k = 0usize;
    for j in 1..4 {
        for t in [this, a, b] {
            cqr4_por(&mut r, k, cqr4_linha(t, j));
            k += 1;
        }
    }
    ordenar(r.as_mut_ptr(), 9, 6, 0x90a130);
    for k in 0..5 {
        plano[3 + k] = cqr4_clube(&r, k);
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1997)(this);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 8);
    let igual = fase != 0 && cqr4_w(fase + 0x3e) == 8 && cqr4_d(fase + 4) == cqr4_d(this + 4) && {
        let mut nela = [0usize; 8];
        for (i, c) in nela.iter_mut().enumerate() {
            *c = cqr4_d(cqr4_d(fase + 0xa7) + 6 * i);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("cqr4_play_off_da_mls({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os dois primeiros vencedores do play-off. Depois, a fase da quarta vaga existe, com +0x3e = 2, +4 = o +4 da liga e
/// os dois na lista dela (como conjunto).
unsafe extern "thiscall" fn comparar_cqr4_final_da_mls(this: usize) {
    let po = cqr4_d(cqr4_d(this + 0xc) + 8);
    let mut v = [0usize; 2];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqr4_w(po + 0x3e) as i32 && n < 2 {
        let e = cqr4_d(po + 0xa7) + 6 * i as usize;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqr4_d(e);
            n += 1;
        }
        i += 1;
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1998)(this);
    let fase = cqr4_d(cqr4_d(this + 0xc) + 0xc);
    // ponytail: com menos de dois vencedores a original lê vagas da lista sem valor; aí só a fase é conferida.
    let igual = fase != 0 && n < 2 || fase != 0 && {
        let l = cqr4_d(fase + 0xa7);
        let (x, y) = (cqr4_d(l), cqr4_d(l + 6));
        cqr4_w(fase + 0x3e) == 2 && cqr4_d(fase + 4) == cqr4_d(this + 4) && ((x, y) == (v[0], v[1]) || (x, y) == (v[1], v[0]))
    };
    PLACAR.registrar(igual, || format!("cqr4_final_da_mls({:#x}) finalistas={:x?} n={} fase={:#x}", this, v, n, fase));
}

// ---------- 282a leva: a virada das copas: a Copa do Brasil, a Copa do Imperador e a Copa da Liga japonesa (a temporada nova: as tabelas liberadas, o ano, os inscritos, as rodadas e a árvore) (bra_cup.cpp, jap_emp_cup.cpp, jap_j_cup.cpp) ----------

unsafe fn cqr5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov edx, [esi]; mov ecx, esi; call [edx + o]).
unsafe fn cqr5_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqr5_d(cqr5_d(this) + o));
    f(this)
}

/// A virada da temporada de uma copa de mata-mata (`start_new_season`, slot 3, +0x08; o mesmo molde nas copas, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c, create_round_table pela fase
/// −1) e a árvore dos confrontos (virtual +0x94); por fim as posições previstas (virtual +0x5c), cujo eax ela devolve.
unsafe fn cqr5_virada(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, ramo: &'static str) -> u32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    for o in [0xa7usize, 0xa3] {
        let l = cqr5_d(this + o);
        if l != 0 {
            soltar(l);
            core::ptr::write_unaligned((this + o) as *mut u32, 0);
        }
    }
    if cqr5_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqr5_w(this + 0x3c) as i32 {
            let p = cqr5_d(cqr5_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
                core::ptr::write_unaligned((cqr5_d(this + 0xad) + i as usize * 4) as *mut u32, 0);
            }
            i += 1;
        }
        soltar(cqr5_d(this + 0xad));
        core::ptr::write_unaligned((this + 0xad) as *mut u32, 0);
    }
    let e = cqr5_d(this + 8);
    if e != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(e, 1);
    }
    let a = cqr5_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // ponytail: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr5_virtual(this, 0x8c) < 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    if cqr5_virtual(this, 0x94) < 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    cqr5_virtual(this, 0x5c) as u32
}

/// O comparador da virada: só a original (ponytail: libera as tabelas e refaz os inscritos, as rodadas e a árvore, que
/// alocam e sorteiam; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqr5_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = cqr5_w(this + 0x40);
    let r = original();
    let depois = cqr5_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// { 0x427d10 } (thiscall, ret 0, bra_cup.cpp; a virtual +0x08 da Copa do Brasil): `BraCup::start_new_season()`, a
/// virada (`cqr5_virada`) com os inscritos de BraCup::build_team_table (0x427c20) e as caixas do bra_cup.cpp nas linhas
/// 0x189, 0x190 e 0x197.
unsafe extern "thiscall" fn cqr5_virada_copa_do_brasil(this: usize) -> u32 {
    cqr5_virada(this, 0x427c20, 0x98a3bc, [0x189, 0x190, 0x197], 1, "cqr_5 virada da copa do brasil") // cqr5: a Copa do Brasil
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_do_brasil(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1999);
    cqr5_comparar("cqr5_virada_copa_do_brasil", this, || o(this))
}

/// { 0x662950 } (thiscall, ret 0, jap_emp_cup.cpp; a virtual +0x08 da Copa do Imperador): `JapEmpCup::start_new_season()`,
/// a virada (`cqr5_virada`) com os inscritos de JapEmpCup::collect_entrants (0x6624f0) e as caixas do jap_emp_cup.cpp nas
/// linhas 0x217, 0x21e e 0x225.
unsafe extern "thiscall" fn cqr5_virada_copa_do_imperador(this: usize) -> u32 {
    cqr5_virada(this, 0x6624f0, 0x9eab90, [0x217, 0x21e, 0x225], 1, "cqr_5 virada da copa do imperador") // cqr5: a Copa do Imperador
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_do_imperador(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2000);
    cqr5_comparar("cqr5_virada_copa_do_imperador", this, || o(this))
}

/// { 0x665bc0 } (thiscall, ret 0, jap_j_cup.cpp; a virtual +0x08 da Copa da Liga japonesa): `JapJCup::start_new_season()`,
/// a virada (`cqr5_virada`) com os inscritos de 0x665a00 (thiscall, ret 0) e as caixas do jap_j_cup.cpp nas linhas 0x1a3,
/// 0x1aa e 0x1b1.
unsafe extern "thiscall" fn cqr5_virada_copa_da_liga_japonesa(this: usize) -> u32 {
    cqr5_virada(this, 0x665a00, 0x9eaf14, [0x1a3, 0x1aa, 0x1b1], 1, "cqr_5 virada da copa da liga japonesa") // cqr5: a Copa da Liga japonesa
}

unsafe extern "thiscall" fn comparar_cqr5_virada_copa_da_liga_japonesa(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2001);
    cqr5_comparar("cqr5_virada_copa_da_liga_japonesa", this, || o(this))
}

// ---------- 282a leva: a virada das copas: a Supercopa japonesa, a Copa dos EUA e o Jogo das Estrelas da MLS (com os dois times das estrelas e o estádio sorteado) (jap_super.cpp, usa_open_cup.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqr6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov edx, [esi]; mov ecx, esi; call [edx + o]).
unsafe fn cqr6_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqr6_d(cqr6_d(this) + o));
    f(this)
}

/// A virada da temporada de uma copa (`start_new_season`, slot 3, +0x08; o molde das copas de mata-mata, conferido
/// instrução a instrução: só mudam o fonte das caixas, as linhas e a rotina dos inscritos): +0x4c = 0; a lista dos
/// inscritos (+0xa7) e a das rodadas (+0xa3) liberadas (0x9452ca, cdecl) e zeradas; os confrontos por rodada (+0xad: um
/// vetor por rodada, +0x3c rodadas, word com sinal) liberados e zerados um a um e o vetor liberado e zerado; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, push 1; mov ecx -> thiscall, ret 4); o ano (+0x40,
/// word) sobe `ano` (1); +0xab = 0 e +0xb1 = 0 (bytes); então, cada um com a sua caixa se devolver negativo: os inscritos
/// (`inscritos`, mov ecx, esi; call direto -> thiscall, ret 0), as rodadas (virtual +0x8c) e a árvore dos confrontos
/// (virtual +0x94); por fim, com `previstas`, as posições previstas (virtual +0x5c), cujo eax ela devolve (o Jogo das
/// Estrelas não as tem: devolve o eax da árvore).
unsafe fn cqr6_virada(this: usize, inscritos: usize, arquivo: usize, linhas: [u32; 3], ano: i16, previstas: bool, ramo: &'static str) -> u32 {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    cqr6_pd(this + 0x4c, 0);
    for o in [0xa7usize, 0xa3] {
        let l = cqr6_d(this + o);
        if l != 0 {
            soltar(l);
            cqr6_pd(this + o, 0);
        }
    }
    if cqr6_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqr6_w(this + 0x3c) as i32 {
            let p = cqr6_d(cqr6_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
                cqr6_pd(cqr6_d(this + 0xad) + i as usize * 4, 0);
            }
            i += 1;
        }
        soltar(cqr6_d(this + 0xad));
        cqr6_pd(this + 0xad, 0);
    }
    let e = cqr6_d(this + 8);
    if e != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(e, 1);
    }
    let a = cqr6_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    *((this + 0xab) as *mut u8) = 0;
    *((this + 0xb1) as *mut u8) = 0;
    let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(inscritos);
    // ponytail: nas caixas a original sai com o eax da caixa (0x5e8290); quem chama a virada não lê a devolução
    if inscritos(this) < 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    if cqr6_virtual(this, 0x8c) < 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let r = cqr6_virtual(this, 0x94);
    if r < 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    if !previstas {
        return r as u32;
    }
    cqr6_virtual(this, 0x5c) as u32
}

/// O comparador da virada: só a original (ponytail: libera as tabelas e refaz os inscritos, as rodadas e a árvore, que
/// alocam e sorteiam; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqr6_comparar(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = cqr6_w(this + 0x40);
    let r = original();
    let depois = cqr6_w(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// { 0x6664e0 } (thiscall, ret 0, jap_super.cpp; a virtual +0x08 da Supercopa japonesa): `JapSuper::start_new_season()`,
/// a virada (`cqr6_virada`) com os dois clubes de JapSuper::select_teams (0x666160) e as caixas do jap_super.cpp nas
/// linhas 0x14b, 0x152 e 0x159.
unsafe extern "thiscall" fn cqr6_virada_supercopa_japonesa(this: usize) -> u32 {
    cqr6_virada(this, 0x666160, 0x9eaf4c, [0x14b, 0x152, 0x159], 1, true, "cqr_6 virada da supercopa japonesa") // cqr6: a Supercopa japonesa
}

unsafe extern "thiscall" fn comparar_cqr6_virada_supercopa_japonesa(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2002);
    cqr6_comparar("cqr6_virada_supercopa_japonesa", this, || o(this))
}

/// { 0x90b790 } (thiscall, ret 0, usa_open_cup.cpp; a virtual +0x08 da Copa dos EUA): `UsaOpenCup::start_new_season()`,
/// a virada (`cqr6_virada`) com os inscritos de UsaOpenCup::collect_entrants (0x90b530) e as caixas do usa_open_cup.cpp
/// nas linhas 0x198, 0x19f e 0x1a6.
unsafe extern "thiscall" fn cqr6_virada_copa_dos_eua(this: usize) -> u32 {
    cqr6_virada(this, 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: a Copa dos EUA
}

unsafe extern "thiscall" fn comparar_cqr6_virada_copa_dos_eua(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2003);
    cqr6_comparar("cqr6_virada_copa_dos_eua", this, || o(this))
}

/// { 0x90a760 } (thiscall, ret 0, usa_mls_all_stars.cpp; a virtual +0x08 do Jogo das Estrelas): `UsaMlsAllStars::
/// start_new_season()`, a virada (`cqr6_virada`) com os dois times de UsaMlsAllStars::set_up_teams (0x90a540) e as caixas
/// do usa_mls_all_stars.cpp nas linhas 0x133, 0x13a e 0x141, sem as posições previstas.
unsafe extern "thiscall" fn cqr6_virada_jogo_das_estrelas(this: usize) -> u32 {
    cqr6_virada(this, 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: o Jogo das Estrelas
}

unsafe extern "thiscall" fn comparar_cqr6_virada_jogo_das_estrelas(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2004);
    cqr6_comparar("cqr6_virada_jogo_das_estrelas", this, || o(this))
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqr6_clube(i: usize) -> usize {
    cqr6_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// { 0x90a540 } (thiscall, ret 0, usa_mls_all_stars.cpp): `UsaMlsAllStars::set_up_teams()`, os dois times do Jogo das
/// Estrelas: +0x3e = 2; a lista (0x944e46 de 0xc, cdecl; sem memória, a caixa da linha 0xc6 e −1) com os dois clubes das
/// estrelas ([0x9d00a4] e [0x9d00a8], índices na tabela dos clubes): no ano ímpar (+0x40, com sinal) o de [0x9d00a4] é o
/// primeiro (o mandante) e a conferência dos estádios é 0x413; no ano par o de [0x9d00a8] é o primeiro e a conferência é
/// 0x414. Um sorteio de 0 a 5 (0x90cfc0(6), cdecl) escolhe o k-ésimo clube da MLS ([0x9cf590] na divisão do clube, +0x57)
/// dessa conferência (usa_mls_get_club_division 0x909ec0, push clube; add esp -> cdecl, palavra com sinal), pela ordem
/// da tabela: o primeiro time das estrelas fica com o estádio dele (+0x69) e +0x6d = 0 (sem o k-ésimo, nada muda). Os
/// dois ficam com a nação [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, os EUA [inferência]) e os quatro bytes da lista (+4,
/// +5, +0xa, +0xb) zerados. Devolve 1.
unsafe fn cqr6_times(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    cqr6_pd(this + 0xa7, l);
    if l == 0 {
        aviso_em(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (cqr6_d(0x9d00a4), cqr6_d(0x9d00a8));
    let conferencia = if cqr6_w(this + 0x40) % 2 != 0 {
        cqr6_pd(l, cqr6_clube(a));
        cqr6_pd(cqr6_d(this + 0xa7) + 6, cqr6_clube(b));
        0x413
    } else {
        cqr6_pd(l + 6, cqr6_clube(a));
        cqr6_pd(cqr6_d(this + 0xa7), cqr6_clube(b));
        0x414
    };
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut k = sortear(6) as u8; // cqr6: o estádio sorteado entre 6
    let divisao: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(0x909ec0usize);
    let mut i = 0usize;
    while (i as i32) < cqr6_d(0xae235c) as i32 {
        let c = cqr6_clube(i);
        let liga = cqr6_d(c + 0x57);
        if liga != 0 && cqr6_d(liga) == cqr6_d(0x9cf590) && divisao(c) as i32 == conferencia {
            if k == 0 {
                crate::comparacao::ramo("cqr_6 estadio do jogo das estrelas");
                let primeiro = cqr6_d(cqr6_d(this + 0xa7));
                cqr6_pd(primeiro + 0x69, cqr6_d(c + 0x69)); // cqr6: o estádio do clube sorteado
                *((cqr6_d(cqr6_d(this + 0xa7)) + 0x6d) as *mut u8) = 0;
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nacao = cqr6_d(0xae23a8).wrapping_add(cqr6_d(0x9cf4f8).wrapping_mul(0x122));
    let l = cqr6_d(this + 0xa7);
    cqr6_pd(cqr6_d(l + 6) + 0x53, nacao);
    cqr6_pd(cqr6_d(l) + 0x53, cqr6_d(cqr6_d(l + 6) + 0x53));
    for o in [4usize, 5, 0xa, 0xb] {
        *((cqr6_d(this + 0xa7) + o) as *mut u8) = 0;
    }
    1
}

unsafe extern "thiscall" fn cqr6_times_do_jogo_das_estrelas(this: usize) -> i32 {
    cqr6_times(this)
}

/// Lado a lado a partir do mesmo estado (o sorteador, +0x3e, os dois clubes das estrelas e [0xb67a34]); a original deixa
/// a lista dela em +0xa7, a cópia faz a sua (sem contar o sorteio), comparada nos 12 bytes e liberada. A lista antiga
/// (+0xa7 na entrada) não é liberada por nenhuma das duas (a virada já a liberou).
unsafe extern "thiscall" fn comparar_cqr6_times_do_jogo_das_estrelas(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(2005);
    let antes = cqr6_d(this + 0xa7);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(this + 0x3e, 2usize), (0xb67a34, 4), (cqr6_clube(cqr6_d(0x9d00a4)), 0x245), (cqr6_clube(cqr6_d(0x9d00a8)), 0x245)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqr6_d(this + 0xa7));
        cqr6_pd(this + 0xa7, antes);
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqr6_times(this));
        lr.set(cqr6_d(this + 0xa7));
        cqr6_pd(this + 0xa7, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0xc) == core::slice::from_raw_parts(lr as *const u8, 0xc)
    } else {
        lo == lr
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqr6_times_do_jogo_das_estrelas({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

// ---------- 282a leva: a Libertadores: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_liber.cpp) ----------

unsafe fn cqr10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr10_sorteado(this: usize, k: usize) -> usize {
    cqr10_d(cqr10_d(this + 0xee) + k * 4) // cqr10: o sorteado k
}

/// { 0x4c4410 } (thiscall, ret 8, conmebol_liber.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Libertadores.
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c3f40:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c37a0, thiscall, ret 0; 0: a caixa da linha 0x365),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x36a), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x370), as fases dos grupos 0..6 (0x4c3b10: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqr10_libertadores_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c3f40usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4c37a0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x4c3b10usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    if cqr10_d(this + 0x45) == 0 && completo != 0 {
        let dia = cqr10_w(this + 0xb6) as u16; // cqr10: o dia do sorteio
        let ano = (cqr10_w(this + 0xb8) as u16).wrapping_add(cqr10_w(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_10 sorteio da libertadores");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7de4, 0x365);
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7de4, 0x36a);
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7de4, 0x370);
            }
            for g in 0..7i32 {
                fase(this, g);
            }
            let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr10_d(cqr10_d(this) + 0x5c));
            previstas(this);
            soltar(cqr10_d(this + 0xee));
            cqr10_pd(this + 0xee, 0);
            cqr10_pd(this + 0x45, 1);
        }
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes (embaralhados), as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 7 fases
// criadas e a fase atual (+0x30) a última delas.
unsafe extern "thiscall" fn comparar_cqr10_libertadores_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2006);
    let sorteia = cqr10_d(this + 0x45) == 0
        && completo != 0
        && (cqr10_w(this + 0xb8) as u16).wrapping_add(cqr10_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr10_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr10_d(this + 0xc);
        let ok = cqr10_d(this + 0x45) == 1
            && cqr10_d(this + 0xee) == 0
            && cqr10_d(this + 0xb1) != 0
            && fases != 0
            && (0..7).all(|g| cqr10_d(fases + g * 4) != 0)
            && cqr10_d(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("cqr10_libertadores_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr10_d(this + 0x45), cqr10_d(this + 0x30) as i32));
    }
    r
}

/// { 0x4c37a0 } (thiscall, ret 0, conmebol_liber.cpp): a tabela do primeiro grupo da Libertadores: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x16f e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqr10_libertadores_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr10_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7de4, 0x16f);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr10_w(this + 0x3e) {
        crate::comparacao::ramo("cqr_10 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr10_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr10_libertadores_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2007);
    let lista = cqr10_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr10_d(lista + i * 4));
    let r = o(this);
    let t = cqr10_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr10_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr10_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr10_libertadores_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x4c3b10 } (thiscall, ret 4, conmebol_liber.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 6, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x218 e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x22e e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
unsafe extern "thiscall" fn cqr10_libertadores_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((cqr10_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0x9a7de4, 0x218);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_10 time do grupo");
        cqr10_pd(times + n * 4, cqr10_sorteado(this, k as usize)); // cqr10: o time na vaga do grupo
        k += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr10_d(cqr10_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            cqr10_w(this + 0x3e) as u16 as u32,
            times,
            cqr10_w(this + 0x3c) as u16 as u32,
            cqr10_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr10_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqr10_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr10_pd(cqr10_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr10_d(cqr10_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9a7de4, 0x22e);
        return 0;
    }
    noticia(f);
    cqr10_pd(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr10_libertadores_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2008);
    let lista = cqr10_d(this + 0xee);
    if lista == 0 || !(0..7).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| cqr10_d(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr10_d(cqr10_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr10_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr10_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr10_d(this + 0x30) as i32 == g && cqr10_w(f + 0x3e) == 4 && cqr10_w(f + 0x40) == cqr10_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr10_libertadores_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 282a leva: a Mercosul: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (conmebol_merc.cpp) ----------

unsafe fn cqr11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr11_sorteado(this: usize, k: usize) -> usize {
    cqr11_d(cqr11_d(this + 0xee) + k * 4) // cqr11: o sorteado k
}

/// { 0x4c5c90 } (thiscall, ret 8, conmebol_merc.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Mercosul.
/// Primeiro a virada: no dia 1º de junho do ano de hoje (a data de 0x549f70(1, 5, ano, −1): lea ecx, local; push −1;
/// push ano; push 5; push 1 -> thiscall, ret 0x10, pura: o dia do ano pelas tabelas dos meses, o mês contado de 0), se o ano da copa (+0x40,
/// com sinal) não é o de hoje, a virtual +8 (a temporada nova: mov ecx, this -> thiscall, ret 0).
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c5850:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c4e60, thiscall, ret 0; 0: a caixa da linha 0x396),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x39b), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x3a1), as fases dos grupos 0..3 (0x4c51d0: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e segue. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqr11_mercosul_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c5850usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4c4e60usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x4c51d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    if core::ptr::read_unaligned(junho.as_ptr().add(2) as *const u16) == ano_de_hoje
        && core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16)
        && cqr11_w(this + 0x40) as i32 != ano_de_hoje as i32
    {
        let virada: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr11_d(cqr11_d(this) + 8));
        virada(this);
    }
    if cqr11_d(this + 0x45) == 0 && completo != 0 {
        let dia = cqr11_w(this + 0xb6) as u16; // cqr11: o dia do sorteio
        let ano = (cqr11_w(this + 0xb8) as u16).wrapping_add(cqr11_w(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_11 sorteio da mercosul");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7e5c, 0x396);
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7e5c, 0x39b);
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7e5c, 0x3a1);
            }
            for g in 0..4i32 {
                fase(this, g);
            }
            let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr11_d(cqr11_d(this) + 0x5c));
            previstas(this);
            soltar(cqr11_d(this + 0xee));
            cqr11_pd(this + 0xee, 0);
            cqr11_pd(this + 0x45, 1);
        }
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes (embaralhados), as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 4 fases
// criadas e a fase atual (+0x30) a última delas.
unsafe extern "thiscall" fn comparar_cqr11_mercosul_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2009);
    let sorteia = cqr11_d(this + 0x45) == 0
        && completo != 0
        && (cqr11_w(this + 0xb8) as u16).wrapping_add(cqr11_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr11_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16) && !cqr11_virada_hoje(this, hoje);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr11_d(this + 0xc);
        let ok = cqr11_d(this + 0x45) == 1
            && cqr11_d(this + 0xee) == 0
            && cqr11_d(this + 0xb1) != 0
            && fases != 0
            && (0..4).all(|g| cqr11_d(fases + g * 4) != 0)
            && cqr11_d(this + 0x30) == 3;
        PLACAR.registrar(ok, || format!("cqr11_mercosul_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr11_d(this + 0x45), cqr11_d(this + 0x30) as i32));
    }
    r
}

/// O dia da virada (1º de junho do ano de hoje, com a copa de outro ano): o comparador não confere o sorteio nesse dia.
unsafe fn cqr11_virada_hoje(this: usize, hoje: usize) -> bool {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    let mut junho = [0u8; 8];
    data(junho.as_mut_ptr(), 1, 5, ano_de_hoje as u32, -1);
    core::ptr::read_unaligned(junho.as_ptr() as *const u16) == core::ptr::read_unaligned(hoje as *const u16) && cqr11_w(this + 0x40) as i32 != ano_de_hoje as i32
}

/// { 0x4c4e60 } (thiscall, ret 0, conmebol_merc.cpp): a tabela do primeiro grupo da Mercosul: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x155 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqr11_mercosul_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr11_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7e5c, 0x155);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr11_w(this + 0x3e) {
        crate::comparacao::ramo("cqr_11 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr11_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr11_mercosul_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2010);
    let lista = cqr11_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr11_d(lista + i * 4));
    let r = o(this);
    let t = cqr11_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr11_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr11_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr11_mercosul_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x4c51d0 } (thiscall, ret 4, conmebol_merc.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 3, do dia do sorteio). Os times num vetor de +0x3e ponteiros (0x944e46(+0x3e << 2): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1fb e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (+0x3e, word), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x211 e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
unsafe extern "thiscall" fn cqr11_mercosul_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(((cqr11_w(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0x9a7e5c, 0x1fb);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_11 time do grupo");
        cqr11_pd(times + n * 4, cqr11_sorteado(this, k as usize)); // cqr11: o time na vaga do grupo
        k += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr11_d(cqr11_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            cqr11_w(this + 0x3e) as u16 as u32,
            times,
            cqr11_w(this + 0x3c) as u16 as u32,
            cqr11_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr11_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqr11_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr11_pd(cqr11_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr11_d(cqr11_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9a7e5c, 0x211);
        return 0;
    }
    noticia(f);
    cqr11_pd(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr11_mercosul_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2011);
    let lista = cqr11_d(this + 0xee);
    if lista == 0 || !(0..4).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| cqr11_d(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr11_d(cqr11_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr11_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr11_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr11_d(this + 0x30) as i32 == g && cqr11_w(f + 0x3e) == 4 && cqr11_w(f + 0x40) == cqr11_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr11_mercosul_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 282a leva: a Merconorte: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros grupos (con_merc_cup.cpp) ----------

unsafe fn cqr12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube k da lista dos sorteados (+0xee, ponteiros), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr12_sorteado(this: usize, k: usize) -> usize {
    cqr12_d(cqr12_d(this + 0xee) + k * 4) // cqr12: o sorteado k
}

/// { 0x4c2c90 } (thiscall, ret 8, con_merc_cup.cpp; a virtual +0x14): `update_for_date(hoje, completo)`, o dia de a Merconorte.
/// Fora do sorteio feito (+0x45 = 0) e com `completo`, no dia do sorteio (a data de 0x549ef0(+0xb6, +0xb8 + +0x40): o
/// dia do ano e o ano relativo; comparada com a de hoje, +0 o dia do ano e +2 o ano, sem sinal): os clubes (0x4c2560:
/// mov ecx, this -> thiscall, ret 0), a tabela do primeiro grupo (0x4c1de0, thiscall, ret 0; 0: a caixa da linha 0x385),
/// os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x38b), a tabela ordenada (0x6827d0
/// push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x392), as fases dos grupos 0..2 (0x4c2140: push g; mov ecx
/// -> thiscall, ret 4), as posições previstas (a virtual +0x5c, thiscall, ret 0), a lista +0xee liberada (0x9452ca:
/// push; add esp, 4 -> cdecl) e zerada e +0x45 = 1. Um erro dá a caixa da linha e volta 0 sem o dia de liga. Depois o dia de liga (0x6847c0: push completo; push
/// hoje; mov ecx, this -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqr12_merconorte_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4c2560usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4c1de0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x4c2140usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    if cqr12_d(this + 0x45) == 0 && completo != 0 {
        let dia = cqr12_w(this + 0xb6) as u16; // cqr12: o dia do sorteio
        let ano = (cqr12_w(this + 0xb8) as u16).wrapping_add(cqr12_w(this + 0x40) as u16);
        if ano == core::ptr::read_unaligned((hoje + 2) as *const u16) && dia == core::ptr::read_unaligned(hoje as *const u16) {
            crate::comparacao::ramo("cqr_12 sorteio da merconorte");
            clubes(this);
            if primeiro(this) == 0 {
                aviso_em(0x9a7d6c, 0x385);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            if jogos(this) == 0 {
                aviso_em(0x9a7d6c, 0x38b);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            if ordenar(this, 0) == 0 {
                aviso_em(0x9a7d6c, 0x392);
                // ponytail: a original devolve só o al = 0 (o resto do eax é o que a caixa deixou); quem chama lê o byte
                return 0;
            }
            for g in 0..3i32 {
                fase(this, g);
            }
            let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr12_d(cqr12_d(this) + 0x5c));
            previstas(this);
            soltar(cqr12_d(this + 0xee));
            cqr12_pd(this + 0xee, 0);
            cqr12_pd(this + 0x45, 1);
        }
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes (embaralhados), as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee zerada, a tabela do primeiro grupo e as 3 fases
// criadas e a fase atual (+0x30) a última delas.
unsafe extern "thiscall" fn comparar_cqr12_merconorte_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2012);
    let sorteia = cqr12_d(this + 0x45) == 0
        && completo != 0
        && (cqr12_w(this + 0xb8) as u16).wrapping_add(cqr12_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr12_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr12_d(this + 0xc);
        let ok = cqr12_d(this + 0x45) == 1
            && cqr12_d(this + 0xee) == 0
            && cqr12_d(this + 0xb1) != 0
            && fases != 0
            && (0..3).all(|g| cqr12_d(fases + g * 4) != 0)
            && cqr12_d(this + 0x30) == 2;
        PLACAR.registrar(ok, || format!("cqr12_merconorte_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr12_d(this + 0x45), cqr12_d(this + 0x30) as i32));
    }
    r
}

/// { 0x4c1de0 } (thiscall, ret 0, con_merc_cup.cpp): a tabela do primeiro grupo da Merconorte: 4 times (+0x3e), a tabela +0xb1
/// (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha 0x145 e
/// devolve 0), cada linha i com o sorteado i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push clube; push
/// i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0)
/// e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqr12_merconorte_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr12_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9a7d6c, 0x145);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr12_w(this + 0x3e) {
        crate::comparacao::ramo("cqr_12 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr12_sorteado(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr12_merconorte_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2013);
    let lista = cqr12_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr12_d(lista + i * 4));
    let r = o(this);
    let t = cqr12_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr12_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr12_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr12_merconorte_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x4c2140 } (thiscall, ret 4, con_merc_cup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal): a fase
/// do grupo g + 1 (g de 0 a 2, do dia do sorteio). Os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem
/// memória, a caixa da linha 0x1e9 e 0): os sorteados 4g + 4 a 4g + 7 (+0xee); a tabela das rodadas pela virtual +0x3c
/// (push 0; push &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, os times (4), o vetor,
/// +0x3c (word), +4, a tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word),
/// g, o id (word), +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos,
/// que o construtor não lê: a cópia passa os valores limpos) em [+0xc][g]; o vetor e a tabela das rodadas liberados
/// (0x9452ca). Sem fase: a caixa da linha 0x1ff e 0. Com ela: a notícia do grupo (0x684230 na fase), +0x30 = g e 1.
unsafe extern "thiscall" fn cqr12_merconorte_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9a7d6c, 0x1e9);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        crate::comparacao::ramo("cqr_12 time do grupo");
        cqr12_pd(times + n * 4, cqr12_sorteado(this, k as usize)); // cqr12: o time na vaga do grupo
        k += 1;
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr12_d(cqr12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            cqr12_w(this + 0x3c) as u16 as u32,
            cqr12_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr12_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            cqr12_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr12_pd(cqr12_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr12_d(cqr12_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9a7d6c, 0x1ff);
        return 0;
    }
    noticia(f);
    cqr12_pd(this + 0x30, g as usize);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com os times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados 4g + 4 a 4g + 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr12_merconorte_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2014);
    let lista = cqr12_d(this + 0xee);
    if lista == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| cqr12_d(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr12_d(cqr12_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr12_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr12_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr12_d(this + 0x30) as i32 == g && cqr12_w(f + 0x3e) == 4 && cqr12_w(f + 0x40) == cqr12_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr12_merconorte_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 282a leva: a Copa América: o dia do sorteio, a tabela do primeiro grupo e as fases dos outros dois grupos (goldcup.cpp) ----------

unsafe fn cqr13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqr13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A seleção k da lista dos sorteados (+0xee, ponteiros: os 12 em grupos intercalados, o grupo j nas vagas j, j + 3,
/// j + 6 e j + 9), a que o primeiro grupo e as fases dos outros leem.
unsafe fn cqr13_sorteado(this: usize, k: usize) -> usize {
    cqr13_d(cqr13_d(this + 0xee) + k * 4) // cqr13: o sorteado k
}

/// { 0x5e2120 } (thiscall, ret 8, goldcup.cpp; a virtual +0x14; quadro SEH só das datas locais, sem destrutor):
/// `GoldCup::update_for_date(hoje, completo)`, o dia da Copa América. A data do sorteio é a de 0x549ef0(+0xb6, +0xb8 +
/// +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura: o dia do ano e o ano). Antes dela (o ano de
/// hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de liga. No dia
/// do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo` (a original sorteia só na chamada com completo = 0):
/// a lista das 12 seleções (0x5e1cd0: mov ecx -> thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall,
/// ret 0), a tabela do primeiro grupo (0x5e11e0, thiscall, ret 0; 0: a caixa da linha 0x3e7), os jogos (0x6835c0
/// League::create_fixtures, thiscall, ret 0; 0: a caixa da linha 0x3ed), a tabela ordenada (0x6827d0 push 0; mov ecx ->
/// thiscall, ret 4; 0: a caixa da linha 0x3f4) (um erro volta 0 sem o dia de liga), as fases dos grupos 0 e 1 (0x5e16b0:
/// push g; mov ecx -> thiscall, ret 4), a lista +0xee liberada (0x9452ca: push; add esp, 4 -> cdecl) e zerada, +0xf2 = 0
/// e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa.
unsafe extern "thiscall" fn cqr13_copa_america_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5e1cd0usize);
    let primeiro: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5e11e0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32) -> u32 = core::mem::transmute(0x5e16b0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let dia = cqr13_w(this + 0xb6) as u16; // cqr13: o dia do sorteio
    let ano = (cqr13_w(this + 0xb8) as u16).wrapping_add(cqr13_w(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas duas voltas antes do sorteio e nos erros a original devolve só o al = 0 (o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && cqr13_d(this + 0x45) == 0 && completo == 0 {
        crate::comparacao::ramo("cqr_13 sorteio da copa america");
        selecoes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqr13_d(cqr13_d(this) + 0x5c));
        previstas(this);
        if primeiro(this) == 0 {
            aviso_em(0x9cd640, 0x3e7);
            return 0;
        }
        if jogos(this) == 0 {
            aviso_em(0x9cd640, 0x3ed);
            return 0;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0x9cd640, 0x3f4);
            return 0;
        }
        for g in 0..2i32 {
            fase(this, g);
        }
        soltar(cqr13_d(this + 0xee));
        cqr13_pd(this + 0xee, 0);
        cqr13_pd(this + 0xf2, 0);
        cqr13_pd(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: a lista, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano no
// dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do primeiro grupo e as duas
// fases criadas e a fase atual (+0x30) a última delas (1).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2015);
    let sorteia = cqr13_d(this + 0x45) == 0
        && completo == 0
        && (cqr13_w(this + 0xb8) as u16).wrapping_add(cqr13_w(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && cqr13_w(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = cqr13_d(this + 0xc);
        let ok = cqr13_d(this + 0x45) == 1
            && cqr13_d(this + 0xee) == 0
            && cqr13_d(this + 0xf2) == 0
            && cqr13_d(this + 0xb1) != 0
            && fases != 0
            && (0..2).all(|g| cqr13_d(fases + g * 4) != 0)
            && cqr13_d(this + 0x30) == 1;
        PLACAR.registrar(ok, || format!("cqr13_copa_america_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, cqr13_d(this + 0x45), cqr13_d(this + 0x30) as i32));
    }
    r
}

/// { 0x5e11e0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::create_group_table()`, o grupo A da Copa América: 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a
/// caixa da linha 0x1d4 e devolve 0), cada linha i com o sorteado 3i (+0xee: os grupos intercalados) (0x687430
/// League::init_table_row: push 0; push 0; push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do
/// sorteio do grupo (0x684230: mov ecx, this -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn cqr13_copa_america_primeiro_grupo(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    cqr13_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0x9cd640, 0x1d4);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < cqr13_w(this + 0x3e) {
        crate::comparacao::ramo("cqr_13 linha do primeiro grupo");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqr13_sorteado(this, i as usize * 3), 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o sorteado 3i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_primeiro_grupo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2016);
    let lista = cqr13_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqr13_d(lista + i * 12));
    let r = o(this);
    let t = cqr13_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqr13_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqr13_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqr13_copa_america_primeiro_grupo({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x5e16b0 } (thiscall, ret 4, goldcup.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal):
/// `GoldCup::create_group_stage(g)`, a fase do grupo g + 1 (g = 0 e 1, do dia do sorteio). Os times num vetor de 4
/// ponteiros (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x2bf e 0): os sorteados g + 1,
/// g + 4, g + 7 e g + 10 (+0xee); a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push g; mov
/// ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage
/// 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a tabela das rodadas, as rodadas (word),
/// +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), g, o id (word), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a
/// original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores limpos)
/// em [+0xc][g]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x2d6 e 0. Com ela:
/// +0x30 = g, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqr13_copa_america_grupo(this: usize, g: i32) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0x9cd640, 0x2bf);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqr_13 time do grupo");
        cqr13_pd(times + n * 4, cqr13_sorteado(this, (g + 1) as usize + n * 3)); // cqr13: o time na vaga do grupo
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqr13_d(cqr13_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            cqr13_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            cqr13_w(this + 0x40) as u16 as u32,
            g as u32,
            id as u16 as u32,
            b(0x51),
            4,
            0,
            cqr13_w(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqr13_pd(cqr13_d(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    let f = cqr13_d(cqr13_d(this + 0xc) + g as usize * 4);
    if f == 0 {
        aviso_em(0x9cd640, 0x2d6);
        return 0;
    }
    cqr13_pd(this + 0x30, g as usize);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][g], +0x30 = g, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// sorteados g + 1, g + 4, g + 7 e g + 10 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqr13_copa_america_grupo(this: usize, g: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> u32>(2017);
    let lista = cqr13_d(this + 0xee);
    if lista == 0 || !(0..2).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|n| cqr13_d(lista + ((g + 1) as usize + n * 3) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = cqr13_d(cqr13_d(this + 0xc) + g as usize * 4);
        let t = if f != 0 { cqr13_d(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| cqr13_d(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && cqr13_d(this + 0x30) as i32 == g && cqr13_w(f + 0x3e) == 4 && cqr13_w(f + 0x40) == cqr13_w(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqr13_copa_america_grupo({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

// ---------- 282a leva: as três divisões da MLS: a tabela da primeira e as fases da segunda e da terceira (usa_mls.cpp) ----------

unsafe fn cqs1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O código de divisão do clube `c` na MLS (usa_mls_get_club_division 0x909ec0: push c; call; add esp, 4 -> cdecl; a
/// palavra baixa do eax: 0x413, 0x47b ou 0x414), se ele é da competição da liga (+0x57 do clube = +4 da liga).
unsafe fn cqs1_da_divisao(this: usize, c: usize, codigo: i16) -> bool {
    let divisao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x909ec0usize);
    cqs1_d(c + 0x57) == cqs1_d(this + 4) && divisao(c) as u16 as i16 == codigo
}

/// { 0x908560 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::create_table()`, a tabela da MLS (a primeira divisão, código
/// 0x413): +0x3e = 4; a tabela (malloc 0x944e46 de 0xec, 4 linhas de 0x3b; sem memória: a caixa da linha 0x2fe e 0)
/// em +0xb1; na ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes, contador i16), enquanto faltam linhas, cada
/// clube da competição com o código 0x413 vai à linha seguinte (League::init_table_row 0x687430: push 0; push 0; push
/// clube; push eax com a linha no byte baixo e o lixo do código em cima, que ela não lê; mov ecx -> thiscall, ret
/// 0x10). Devolve 1 se encheu as 4 linhas.
unsafe extern "thiscall" fn cqs1_tabela_da_mls(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = alocar(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0xaa1fb4, 0x2fe);
        return 0;
    }
    let mut n: i16 = 0;
    let base = cqs1_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqs1_d(0xae235c) as i32 && n < cqs1_w(this + 0x3e) {
        let c = base + i as u16 as usize * 0x245;
        if cqs1_da_divisao(this, c, 0x413) {
            iniciar(this, n as u8 as u32, c, 0, 0); // cqs1: a linha seguinte da primeira divisão
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqs_1 tabela da mls");
    (n == cqs1_w(this + 0x3e)) as u32
}

/// A fase de uma das outras divisões da MLS (`create_second_division_stage` e `create_third_division_stage`; o mesmo
/// molde, conferido instrução a instrução: só mudam o código da divisão, a vaga k, o fonte das caixas e as linhas).
/// 1) Um vetor de +0x3e clubes (malloc 0x944e46 de 4 × +0x3e; sem memória: a caixa da linha `linhas[0]` e volta) com os
///    clubes da competição com o `codigo`, na ordem do banco (contador i32), enquanto faltam vagas.
/// 2) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, −3, &rodadas, &extra, 0), ret 0x10).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, +0x3c (word, os turnos), +4, a tabela das rodadas, as rodadas
///    (word), os pontos +0xc2 (`pv`) e +0xc3, a categoria +0xc4, &+0xc5, &+0xbe (as vagas da liga), o ano +0x40 (word),
///    k, o extra (word), +0x51, 1, +0xab (word), +0xd9 (word), +0xa7 (word), 0, 2; a cópia passa os valores limpos).
///    Sem memória: 0.
/// 4) [+0xc][k] = a fase; o vetor e a tabela das rodadas liberados (free 0x9452ca, nessa ordem). Sem fase: a caixa da
///    linha `linhas[1]`. Com ela: +0x30 = `atual` (a original grava k) e +0x3a da fase = o código da divisão.
unsafe fn cqs1_divisao(this: usize, codigo: i16, k: u32, atual: u32, pv: usize, linhas: [u32; 2], ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let v = alocar(((cqs1_w(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0xaa1fb4, linhas[0]);
    }
    let mut n: i16 = 0;
    let base = cqs1_d(0xae23b4);
    let mut i = 0i32;
    while i < cqs1_d(0xae235c) as i32 && n < cqs1_w(this + 0x3e) {
        let c = base + i as usize * 0x245;
        if cqs1_da_divisao(this, c, codigo) {
            core::ptr::write_unaligned((v + 4 * n as usize) as *mut usize, c);
            n += 1;
        }
        i += 1;
    }
    let tabela: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs1_d(cqs1_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, -3, &mut rodadas, &mut extra, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, cqs1_w(this + 0x3e) as u16 as u32, v, cqs1_w(this + 0x3c) as u16 as u32, cqs1_d(this + 4), t, rodadas as u16 as u32,
                  cqs1_b(this + pv) as u32, cqs1_b(this + 0xc3) as u32, cqs1_b(this + 0xc4) as u32, this + 0xc5, this + 0xbe,
                  cqs1_w(this + 0x40) as u16 as u32, k, extra as u16 as u32, cqs1_b(this + 0x51) as u32, 1, cqs1_w(this + 0xab) as u16 as u32,
                  cqs1_w(this + 0xd9) as u16 as u32, cqs1_w(this + 0xa7) as u16 as u32, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs1_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    soltar(v);
    soltar(t);
    let fase = cqs1_d(cqs1_d(this + 0xc) + 4 * k as usize);
    if fase == 0 {
        return aviso_em(0xaa1fb4, linhas[1]);
    }
    core::ptr::write_unaligned((this + 0x30) as *mut u32, atual);
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((fase + 0x3a) as *mut i16, codigo);
}

/// { 0x908fc0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_second_division_stage()`, a
/// segunda divisão da MLS (código 0x47b) na fase 0; caixas nas linhas 0x4e6 e 0x506.
unsafe extern "thiscall" fn cqs1_segunda_divisao_da_mls(this: usize) {
    cqs1_divisao(this, 0x47b, 0, 0, 0xc2, [0x4e6, 0x506], "cqs_1 segunda divisao") // cqs1: os pontos por vitória na segunda
}

/// { 0x9091e0 } (thiscall, ret 0, usa_mls.cpp; quadro SEH só para o new): `UsaMls::create_third_division_stage()`, a
/// terceira divisão da MLS (código 0x414) na fase 1; caixas nas linhas 0x524 e 0x544.
unsafe extern "thiscall" fn cqs1_terceira_divisao_da_mls(this: usize) {
    cqs1_divisao(this, 0x414, 1, 1, 0xc2, [0x524, 0x544], "cqs_1 terceira divisao") // cqs1: a fase atual depois da terceira
}

/// O comparador da tabela: só a original (ponytail: aloca a tabela e as listas das linhas), com o plano feito antes:
/// os clubes da competição com o código 0x413, na ordem do banco, até 4. Depois, a devolução, +0x3e = 4 e as linhas.
unsafe extern "thiscall" fn comparar_cqs1_tabela_da_mls(this: usize) -> u32 {
    let mut plano = [0usize; 4];
    let mut n = 0usize;
    let base = cqs1_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqs1_d(0xae235c) as i32 && n < 4 {
        let c = base + i as u16 as usize * 0x245;
        if cqs1_da_divisao(this, c, 0x413) {
            plano[n] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2018)(this);
    let t = cqs1_d(this + 0xb1);
    let igual = r == (n == 4) as u32 && cqs1_w(this + 0x3e) == 4 && t != 0 && (0..n).all(|k| cqs1_d(t + k * 0x3b) == plano[k]);
    PLACAR.registrar(igual, || format!("cqs1_tabela_da_mls({:#x}) r={} n={} plano={:x?}", this, r, n, plano));
    r
}

/// O comparador de uma divisão: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes:
/// os clubes do código, na ordem do banco. Depois, a fase na vaga k com +0x3e da liga, os clubes na tabela dela (como
/// conjunto), +0x3a = o código e +0x30 = k.
unsafe fn cqs1_comparar(nome: &str, this: usize, codigo: i16, k: usize, original: impl FnOnce()) {
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let base = cqs1_d(0xae23b4);
    let mut i = 0i32;
    while i < cqs1_d(0xae235c) as i32 && (n as i16) < cqs1_w(this + 0x3e) && n < 8 {
        let c = base + i as usize * 0x245;
        if cqs1_da_divisao(this, c, codigo) {
            plano[n] = c;
            n += 1;
        }
        i += 1;
    }
    original();
    let fase = cqs1_d(cqs1_d(this + 0xc) + 4 * k);
    let igual = fase != 0 && cqs1_w(fase + 0x3a) == codigo && cqs1_d(this + 0x30) == k && cqs1_w(fase + 0x3e) == cqs1_w(this + 0x3e) && {
        let mut nela = [0usize; 8];
        for (j, c) in nela.iter_mut().enumerate().take(n) {
            *c = cqs1_d(cqs1_d(fase + 0xb1) + j * 0x3b);
        }
        let (mut a, mut b) = (plano, nela);
        a[..n].sort_unstable();
        b[..n].sort_unstable();
        a[..n] == b[..n]
    };
    PLACAR.registrar(igual, || format!("{}({:#x}) fase={:#x} n={} plano={:x?}", nome, this, fase, n, plano));
}

unsafe extern "thiscall" fn comparar_cqs1_segunda_divisao_da_mls(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2019);
    cqs1_comparar("cqs1_segunda_divisao_da_mls", this, 0x47b, 0, || o(this))
}

unsafe extern "thiscall" fn comparar_cqs1_terceira_divisao_da_mls(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2020);
    cqs1_comparar("cqs1_terceira_divisao_da_mls", this, 0x414, 1, || o(this))
}

// ---------- 282a leva: a MLS: a virada da temporada, as posições previstas e a lista dos doze clubes (usa_mls.cpp) ----------

unsafe fn cqs2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `i` da tabela (+0xb1, linhas de 0x3b bytes) da liga ou fase `t`.
unsafe fn cqs2_linha(t: usize, i: usize) -> usize {
    cqs2_d(cqs2_d(t + 0xb1) + i * 0x3b)
}

/// { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::start_new_season()`, a virada da MLS: +0x4c = 0 e a tabela
/// liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o calendário (+0xba) liberado (free 0x9452ca) e
/// zerado; com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1); +0x30 = −1;
/// com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); o ano (+0x40, word)
/// sobe `ano` (o contador "cqs_2 virada da mls"); então, cada um com a sua caixa (linhas 0x4af, 0x4b6, 0x4bc e 0x4c3)
/// se devolver 0: as regras (virtual +0x8c), a tabela da primeira divisão (0x908560), os jogos da temporada
/// (UsaMls::create_season_fixtures 0x9086e0, que monta também as fases das outras duas divisões) e a ordem da tabela
/// (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); por fim as posições previstas (virtual +0x5c), as
/// expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve. Com
/// caixa, a cópia devolve 0 (a original devolve o eax da caixa; a virada é chamada pelo laço das viradas, que não o lê).
/// O vetor das fases (+0xc) fica: as divisões e o play-off regravam as vagas.
unsafe fn cqs2_virada(this: usize, ano: i16) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqs2_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    if (cqs2_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs2_d(cqs2_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs2_d(cqs2_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs2_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let estatisticas = cqs2_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    let a = cqs2_w(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo("cqs_2 virada da mls");
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs2_d(cqs2_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(0xaa1fb4, 0x4af);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x908560usize);
    if tabela(this) == 0 {
        aviso_em(0xaa1fb4, 0x4b6);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9086e0usize);
    if jogos(this) == 0 {
        aviso_em(0xaa1fb4, 0x4bc);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(0xaa1fb4, 0x4c3);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs2_d(cqs2_d(this) + 0x5c));
    previstas(this);
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqs2_d(0xb74340), cqs2_d(this + 4)) // cqs2: a renovação dos árbitros da MLS
}

/// { 0x908e00 } (thiscall, ret 0, usa_mls.cpp): a virada da MLS (acima), um ano adiante.
unsafe extern "thiscall" fn cqs2_virada_da_mls(this: usize) -> u32 {
    cqs2_virada(this, 1) // cqs2: o ano seguinte da MLS
}

/// { 0x909bb0 } (thiscall, ret 0, usa_mls.cpp): `UsaMls::set_predicted_positions()`, as posições previstas da MLS. Sem
/// estatísticas (+8): a caixa da linha 0x675 e volta. Um vetor de 3 × +0x3e clubes (malloc 0x944e46; sem memória: a
/// caixa da linha 0x67d e volta): os da tabela da liga, os da fase 0 e os da fase 1 (as três divisões, linha a linha),
/// cada bloco de +0x3e ordenado pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no fim). Os doze
/// primeiros do vetor ganham, em cada divisão, as posições 1, 4, 7 e 10 (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a ordem das doze chamadas é a do vetor); o vetor liberado (free 0x9452ca).
/// [inferência] As três divisões se misturam na previsão: o favorito de cada uma é previsto em 1º, o segundo em 4º...
unsafe extern "thiscall" fn cqs2_posicoes_previstas_da_mls(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let n3 = cqs2_w(this + 0x3e).wrapping_mul(3);
    if cqs2_d(this + 8) == 0 {
        return aviso_em(0xaa1fb4, 0x675);
    }
    let v = alocar(((n3 as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0xaa1fb4, 0x67d);
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let n = cqs2_w(this + 0x3e) as i32;
        let k = i as usize;
        core::ptr::write_unaligned((v + 4 * k) as *mut usize, cqs2_linha(this, k));
        core::ptr::write_unaligned((v + 4 * (k + n as usize)) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), k));
        core::ptr::write_unaligned((v + 4 * (k + 2 * n as usize)) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), k));
        i += 1;
    }
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v + 4 * n as usize, n as u32, 4, 0x4b07d0);
    let n = cqs2_w(this + 0x3e) as i32;
    ordenar(v + 8 * n as usize, n as u32, 4, 0x4b07d0);
    let lugares: [u32; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10]; // cqs2: as posições previstas das três divisões
    for (k, p) in lugares.iter().enumerate() {
        posicao(cqs2_d(this + 8), cqs2_d(v + 4 * k), *p);
    }
    crate::comparacao::ramo("cqs_2 posicoes previstas");
    soltar(v);
}

/// { 0x909a40 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::get_club_array(&lista, &n)`, a lista dos clubes da MLS: *n = 12
/// (word); *lista = malloc 0x944e46 de 0x30 (sem memória: a caixa da linha 0x64c, *n = 0 e volta); os +0x3e clubes da
/// tabela da liga, depois os da fase 0 (a partir de +0x3e) e os da fase 1 (a partir de 2 × +0x3e), linha a linha.
unsafe extern "thiscall" fn cqs2_clubes_da_mls(this: usize, lista: *mut usize, n: *mut i16) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *n = 0xc;
    let v = alocar(0x30);
    *lista = v;
    if v == 0 {
        aviso_em(0xaa1fb4, 0x64c);
        *n = 0;
        return;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        core::ptr::write_unaligned((*lista + 4 * i as usize) as *mut usize, cqs2_linha(this, i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let k = cqs2_w(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), i as usize));
        i += 1;
    }
    let mut i: i16 = 0;
    while i < cqs2_w(this + 0x3e) {
        let k = 2 * cqs2_w(this + 0x3e) as i32 + i as i32;
        core::ptr::write_unaligned((*lista + 4 * k as usize) as *mut usize, cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), i as usize));
        i += 1;
    }
    crate::comparacao::ramo("cqs_2 clubes da mls");
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1 e as três divisões montadas
/// (+0x3e = 4 e as vagas 0 e 1 das fases).
unsafe extern "thiscall" fn comparar_cqs2_virada_da_mls(this: usize) -> u32 {
    let antes = cqs2_w(this + 0x40);
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2021)(this);
    let depois = cqs2_w(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && cqs2_w(this + 0x3e) == 4 && cqs2_d(cqs2_d(this + 0xc)) != 0 && cqs2_d(cqs2_d(this + 0xc) + 4) != 0;
    PLACAR.registrar(igual, || format!("cqs2_virada_da_mls({:#x}) ano antes={} depois={} +0x3e={}", this, antes, depois, cqs2_w(this + 0x3e)));
    r
}

/// O comparador das posições previstas: só a original (ponytail: a 0x4a2540 acrescenta o registro de um clube que não
/// tem; não pode rodar duas vezes), com o plano feito antes: os doze clubes das três tabelas, cada bloco ordenado pela
/// mesma qsort numa cópia; depois, a posição (+0x72 do registro achado por 0x4a4850) de cada um.
unsafe extern "thiscall" fn comparar_cqs2_posicoes_previstas_da_mls(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2022);
    let n = cqs2_w(this + 0x3e) as usize;
    if cqs2_d(this + 8) == 0 || n != 4 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v = [0usize; 12];
    for k in 0..4 {
        v[k] = cqs2_linha(this, k);
        v[4 + k] = cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), k);
        v[8 + k] = cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), k);
    }
    for b in 0..3 {
        ordenar(v[4 * b..].as_mut_ptr(), 4, 4, 0x4b07d0);
    }
    o(this);
    const LUGARES: [u8; 12] = [1, 4, 7, 10, 1, 4, 7, 10, 1, 4, 7, 10];
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(cqs2_d(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) != LUGARES[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("cqs2_posicoes_previstas_da_mls({:#x}): a posição da vaga {:?} não bate ({:x?})", this, errado, v));
}

/// O comparador da lista: só a original (aloca a lista, que fica com quem chamou), com o plano: n = 12 e os doze clubes
/// das três tabelas na ordem.
unsafe extern "thiscall" fn comparar_cqs2_clubes_da_mls(this: usize, lista: *mut usize, n: *mut i16) {
    orig::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(2023)(this, lista, n);
    let m = cqs2_w(this + 0x3e) as usize;
    let igual = *n == 0xc && *lista != 0 && (0..m).all(|i| {
        cqs2_d(*lista + 4 * i) == cqs2_linha(this, i)
            && cqs2_d(*lista + 4 * (m + i)) == cqs2_linha(cqs2_d(cqs2_d(this + 0xc)), i)
            && cqs2_d(*lista + 4 * (2 * m + i)) == cqs2_linha(cqs2_d(cqs2_d(this + 0xc) + 4), i)
    });
    PLACAR.registrar(igual, || format!("cqs2_clubes_da_mls({:#x}) n={}", this, *n));
}

// ---------- 282a leva: a Série C: os sete grupos da primeira fase, a segunda fase dos dezesseis e os dois grupos da terceira (bra_nat_third.cpp) ----------

unsafe fn cqs3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs3_b(a: usize) -> u8 {
    *(a as *const u8)
}

#[allow(clippy::type_complexity)]
type Cqs3Fase = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize;

/// { 0x42ee80 } (thiscall, ret 4, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_group_stage(g)`, o
/// grupo g (0 a 6) da primeira fase da Série C, da virada e do construtor (g de 0 a 6; a liga é o grupo de cima, e os
/// sete grupos são as fases 0 a 6).
/// 1) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, g inteiro, &rodadas, &extra, 0), ret
///    0x10).
/// 2) Um vetor de 4 × +0x3e (malloc 0x944e46; sem memória: a caixa da linha 0x475 e volta) com os sete clubes do
///    grupo: as vagas 7(g + 1) a 7(g + 1) + 6 da lista dos 56 (+0xee; as sete primeiras são da liga).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, 7, o vetor, 2 (turnos: ida e volta), +4, a tabela das rodadas, as rodadas (word), os
///    pontos +0xc2 (`pv`) e +0xc3, a categoria +0xc4, &+0xc5, &+0xbe (as vagas da liga), o ano +0x40 (word), g (o
///    dword inteiro), o extra (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; words e bytes limpos). Sem memória: 0.
/// 4) [+0xc][g] = a fase. Sem fase: a caixa da linha 0x487 e volta (sem liberar o vetor nem a tabela). Com ela: o vetor
///    e a tabela liberados (free 0x9452ca) e +0x30 = g (o byte com sinal).
unsafe fn cqs3_grupo(this: usize, g: u32, pv: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let construir: Cqs3Fase = core::mem::transmute(0x68b100usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, g, &mut rodadas, &mut extra, 0);
    let v = alocar(((cqs3_w(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0x98aba4, 0x475);
    }
    let k = g as u8 as i8 as i32;
    for j in 0..7usize {
        core::ptr::write_unaligned((v + 4 * j) as *mut usize, cqs3_d(cqs3_d(this + 0xee) + 4 * (7 * (k + 1) as usize + j)));
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, 7, v, 2, cqs3_d(this + 4), t, rodadas as u16 as u32, cqs3_b(this + pv) as u32, cqs3_b(this + 0xc3) as u32,
                  cqs3_b(this + 0xc4) as u32, this + 0xc5, this + 0xbe, cqs3_w(this + 0x40) as u16 as u32, g, extra as u16 as u32,
                  cqs3_b(this + 0x51) as u32, 1, 0, cqs3_w(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs3_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    if cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize) == 0 {
        return aviso_em(0x98aba4, 0x487);
    }
    crate::comparacao::ramo("cqs_3 grupo");
    soltar(v);
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
}

/// { 0x42ee80 } (acima): o grupo g da primeira fase da Série C.
unsafe extern "thiscall" fn cqs3_grupo_da_serie_c(this: usize, g: u32) {
    cqs3_grupo(this, g, 0xc2) // cqs3: os pontos por vitória nos grupos
}

/// A ordem da segunda fase: o k-ésimo melhor (pelos pontos) vai à vaga `CQS3_SEMENTES[k]` da lista dos dezesseis.
const CQS3_SEMENTES: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: as vagas da segunda fase

/// { 0x42e430 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_second_round_stage()`,
/// a segunda fase da Série C, do fim dos grupos.
/// 1) Uma lista de 16 (malloc 0x944e46 de 0x40; sem memória: a caixa da linha 0x2e6 e volta).
/// 2) Os dois primeiros de cada um dos oito grupos (a tabela da liga e as das fases 0 a 6, nessa ordem), em registros de
///    6 bytes (o clube e os pontos, +0x19 da linha).
/// 3) Dezesseis vezes: o registro de mais pontos (o primeiro, no empate) entre os que ainda têm clube sai (o clube vira
///    0) e o clube vai à vaga `CQS3_SEMENTES[k]`: o melhor contra o 16º, o 2º contra o 15º... (sem nenhum: a caixa da
///    linha 0x30b; a original lê então o endereço 0 e cai).
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 7, &rodadas, &extra, 0), ret 0x10); a fase (operator new
///    0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 16, a lista, as rodadas, +4, a
///    tabela, o ano, 7, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][7]. Sem fase: a caixa da
///    linha 0x31b e volta. Com ela: a tabela e a lista liberadas (free 0x9452ca) e +0x30 = 7.
unsafe extern "thiscall" fn cqs3_segunda_fase_da_serie_c(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let lista = alocar(0x40);
    if lista == 0 {
        return aviso_em(0x98aba4, 0x2e6);
    }
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = cqs3_d(if g == -1 { this } else { cqs3_d(cqs3_d(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (cqs3_d(t), cqs3_w(t + 0x19));
        r[2 * k + 1] = (cqs3_d(t + 0x3b), cqs3_w(t + 0x54));
    }
    for k in 0..16 {
        let mut melhor: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if melhor.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                melhor = Some(i);
            }
        }
        let c = match melhor {
            Some(m) => {
                let c = r[m].0;
                r[m].0 = 0;
                c
            }
            None => {
                aviso_em(0x98aba4, 0x30b);
                core::ptr::read_volatile(core::ptr::null::<usize>()) // ponytail: a original lê o endereço 0 aqui e cai
            }
        };
        core::ptr::write_unaligned((lista + 4 * CQS3_SEMENTES[k] as usize) as *mut usize, c);
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 7, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 0x10, lista, rodadas as u16 as u32, cqs3_d(this + 4), t, cqs3_w(this + 0x40) as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs3_d(this + 0xc) + 0x1c) as *mut usize, fase);
    if cqs3_d(cqs3_d(this + 0xc) + 0x1c) == 0 {
        return aviso_em(0x98aba4, 0x31b);
    }
    crate::comparacao::ramo("cqs_3 segunda fase");
    soltar(t);
    soltar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 7);
}

/// { 0x42e700 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_third_round_groups()`,
/// a terceira fase da Série C, do fim da segunda: os vencedores da segunda fase ([+0xc][7], os registros de 6 bytes de
/// +0xa7 com +5 = 1, na ordem) numa lista local de 8, embaralhada (comp_util_shuffle_pointer_array 0x4afb40, cdecl(v,
/// 8)); os quatro primeiros formam o grupo A (fase 8) e os outros quatro o grupo B (fase 9). Cada grupo: a tabela das
/// rodadas pela virtual +0x3c (thiscall(this, 8 ou 9, &rodadas, &extra, 0)) e a fase (operator new 0x944cf1(0xee) e o
/// construtor League_Stage 0x68b100: a liga, 4, a lista (ou a lista + 4), 2 (ida e volta), +4, a tabela, as rodadas
/// (word), 3 pontos por vitória, 1 por empate, a categoria 5, &{1,2,4,0} (os critérios de desempate), &{2,0,0,0} (duas
/// vagas de cima), o ano, 8 ou 9, o extra (word), 0x14, 1, 0, 2 (os bits de regra), −1, 0, 2) em [+0xc][8] ou [9];
/// sem fase: a caixa da linha 0x353 ou 0x363 e volta; com ela, a tabela liberada (free 0x9452ca). Depois do segundo,
/// +0x30 = 9. [inferência] Os dois primeiros de cada grupo sobem à Série B; os campeões dos grupos fazem a final.
unsafe extern "thiscall" fn cqs3_terceira_fase_da_serie_c(this: usize) {
    let embaralhar: unsafe extern "C" fn(*mut usize, u32) = core::mem::transmute(0x4afb40usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [2, 0, 0, 0];
    let desempate: [u8; 4] = [1, 2, 4, 0];
    let mut v = [0usize; 8];
    let segunda = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs3_w(segunda + 0x3e) as i32 {
        let e = cqs3_d(segunda + 0xa7) + 6 * i as usize;
        // ponytail: a original grava sem limite numa lista de 8 na pilha; a fase tem 8 vencedores (a cópia para no 8º).
        if cqs3_b(e + 5) == 1 && n < 8 {
            v[n] = cqs3_d(e);
            n += 1;
        }
        i += 1;
    }
    embaralhar(v.as_mut_ptr(), 8); // cqs3: os oito vencedores embaralhados
    for (k, f) in [(8u32, 0x353u32), (9, 0x363)] {
        let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
        let (mut rodadas, mut extra) = (0i16, 0i16);
        let t = tabela(this, k, &mut rodadas, &mut extra, 0);
        let p = novo(0xee);
        let fase = if p != 0 {
            construir(p, this, 4, v[4 * (k as usize - 8)..].as_ptr(), 2, cqs3_d(this + 4), t, rodadas as u16 as u32, 3, 1, 5,
                      desempate.as_ptr(), vagas.as_ptr(), cqs3_w(this + 0x40) as u16 as u32, k, extra as u16 as u32, 0x14, 1, 0, 2, -1, 0, 2)
        } else {
            0
        };
        core::ptr::write_unaligned((cqs3_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
        if cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize) == 0 {
            return aviso_em(0x98aba4, f);
        }
        soltar(t);
    }
    crate::comparacao::ramo("cqs_3 terceira fase");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 9);
}

/// O comparador de um grupo: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes: os
/// sete clubes das vagas 7(g + 1).. da lista dos 56. Depois, a fase na vaga g com +0x3e = 7, os sete na tabela dela
/// (como conjunto) e +0x30 = g.
unsafe extern "thiscall" fn comparar_cqs3_grupo_da_serie_c(this: usize, g: u32) {
    let k = g as u8 as i8 as i32;
    let mut plano = [0usize; 7];
    for (j, c) in plano.iter_mut().enumerate() {
        *c = cqs3_d(cqs3_d(this + 0xee) + 4 * (7 * (k + 1) as usize + j));
    }
    orig::<unsafe extern "thiscall" fn(usize, u32)>(2024)(this, g);
    let fase = cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize);
    let igual = fase != 0 && cqs3_w(fase + 0x3e) == 7 && cqs3_d(this + 0x30) as i32 == k && {
        let mut nela = [0usize; 7];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = cqs3_d(cqs3_d(fase + 0xb1) + j * 0x3b);
        }
        let mut a = plano;
        a.sort_unstable();
        nela.sort_unstable();
        a == nela
    };
    PLACAR.registrar(igual, || format!("cqs3_grupo_da_serie_c({:#x}, {}) fase={:#x} plano={:x?}", this, k, fase, plano));
}

/// O comparador da segunda fase: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela mesma escolha numa cópia dos registros: a fase da vaga 7 existe, com +0x3e = 16 e os dezesseis na ordem
/// das vagas na lista dela (+0xa7, 6 bytes), e +0x30 = 7.
unsafe extern "thiscall" fn comparar_cqs3_segunda_fase_da_serie_c(this: usize) {
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = cqs3_d(if g == -1 { this } else { cqs3_d(cqs3_d(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (cqs3_d(t), cqs3_w(t + 0x19));
        r[2 * k + 1] = (cqs3_d(t + 0x3b), cqs3_w(t + 0x54));
    }
    let mut plano = [0usize; 16];
    for k in 0..16 {
        let mut melhor: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if melhor.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                melhor = Some(i);
            }
        }
        if let Some(m) = melhor {
            plano[CQS3_SEMENTES[k] as usize] = r[m].0;
            r[m].0 = 0;
        }
    }
    orig::<unsafe extern "thiscall" fn(usize)>(2025)(this);
    let fase = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let igual = fase != 0 && cqs3_w(fase + 0x3e) == 0x10 && cqs3_d(this + 0x30) == 7 && {
        let mut nela = [0usize; 16];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = cqs3_d(cqs3_d(fase + 0xa7) + 6 * j);
        }
        let (mut a, mut b) = (plano, nela);
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    PLACAR.registrar(igual, || format!("cqs3_segunda_fase_da_serie_c({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// O comparador da terceira fase: só a original (embaralha: sorteia; aloca as tabelas e as fases), com o plano: os
/// vencedores da segunda fase; depois, as fases 8 e 9 com 4 clubes cada, os oito juntos (como conjunto) e +0x30 = 9.
unsafe extern "thiscall" fn comparar_cqs3_terceira_fase_da_serie_c(this: usize) {
    let segunda = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs3_w(segunda + 0x3e) as i32 && n < 8 {
        let e = cqs3_d(segunda + 0xa7) + 6 * i as usize;
        if cqs3_b(e + 5) == 1 {
            plano[n] = cqs3_d(e);
            n += 1;
        }
        i += 1;
    }
    orig::<unsafe extern "thiscall" fn(usize)>(2026)(this);
    let (a, b) = (cqs3_d(cqs3_d(this + 0xc) + 0x20), cqs3_d(cqs3_d(this + 0xc) + 0x24));
    let igual = a != 0 && b != 0 && cqs3_w(a + 0x3e) == 4 && cqs3_w(b + 0x3e) == 4 && cqs3_d(this + 0x30) == 9 && {
        let mut nela = [0usize; 8];
        for j in 0..4 {
            nela[j] = cqs3_d(cqs3_d(a + 0xb1) + j * 0x3b);
            nela[4 + j] = cqs3_d(cqs3_d(b + 0xb1) + j * 0x3b);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("cqs3_terceira_fase_da_serie_c({:#x}) n={} plano={:x?}", this, n, plano));
}
