// ---------- %%LEVA%%a leva: a virada da temporada do Carioca, do Mineiro e do Paranaense (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqr2_comparar("cqr2_virada_do_carioca", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_mineiro(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqr2_comparar("cqr2_virada_do_mineiro", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqr2_virada_do_paranaense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqr2_comparar("cqr2_virada_do_paranaense", this, || o(this))
}
