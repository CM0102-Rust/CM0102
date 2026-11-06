// ---------- %%LEVA%%a leva: a virada da temporada e a fase seguinte dos estaduais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqp9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A virada da temporada de um estadual (`start_new_season`, slot 3, +0x08; o mesmo molde nos nove estaduais de pontos
/// corridos com mata-mata: só mudam o fonte, as linhas das caixas e a rotina da tabela, conferido instrução a instrução):
/// +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada
/// (free 0x9452ca) e zerada; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1),
/// ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0, o destrutor com 1: libera);
/// o ano (+0x40, word) sobe um (`ano`; o contador `ramo`) e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras da temporada
/// (virtual +0x8c, set_up_rules), a tabela com os clubes do estadual (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c) e a renovação dos árbitros do estadual
/// (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe fn cqp9_virada(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp9_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp9_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp9_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp9_d(cqp9_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp9_d(cqp9_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp9_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp9_d(cqp9_d(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp9_d(cqp9_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp9_d(0xb74340), cqp9_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp9_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp9_d(this + 0x30) as i32;
    if fase >= (cqp9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        f(this);
    }
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia os jogos e os
/// árbitros; não pode rodar duas vezes), conferindo o que só ela garante: o ano (+0x40) sobe um.
unsafe fn cqp9_comparar_virada(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.registrar(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn cqp9_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp9_d(this + 0x30) as i32;
    let ultima = antes >= (cqp9_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp9_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x4394d0 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::start_new_season()`, a virada da temporada do Pernambucano (a tabela:
/// 0x439330; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_pernambucano(this: usize) -> u32 {
    cqp9_virada(this, 0x439330, 0x98afd8, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do pernambucano") // cqp9: o ano seguinte do Pernambucano
}

/// { 0x439680 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::advance_stage()`, a fase seguinte do Pernambucano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x439710); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_pernambucano(this: usize) {
    cqp9_fase_seguinte(this, 0x98afd8, 0x1f8, 0x439710);
}

/// { 0x43bd10 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::start_new_season()`, a virada da temporada do Catarinense (a tabela:
/// 0x43bb70; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_catarinense(this: usize) -> u32 {
    cqp9_virada(this, 0x43bb70, 0x98b050, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do catarinense") // cqp9: o ano seguinte do Catarinense
}

/// { 0x43bec0 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::advance_stage()`, a fase seguinte do Catarinense: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43bf50); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_catarinense(this: usize) {
    cqp9_fase_seguinte(this, 0x98b050, 0x1f8, 0x43bf50);
}

/// { 0x43cd40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::start_new_season()`, a virada da temporada do Paulista (a tabela:
/// 0x43cc40; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
unsafe extern "thiscall" fn cqp9_virada_do_paulista(this: usize) -> u32 {
    cqp9_virada(this, 0x43cc40, 0x98b090, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_9 virada do paulista") // cqp9: o ano seguinte do Paulista
}

/// { 0x43cef0 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::advance_stage()`, a fase seguinte do Paulista: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43cf80); a caixa da linha 0x1ea.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_paulista(this: usize) {
    cqp9_fase_seguinte(this, 0x98b090, 0x1ea, 0x43cf80);
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_pernambucano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqp9_comparar_virada("cqp9_virada_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_pernambucano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_catarinense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqp9_comparar_virada("cqp9_virada_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_catarinense(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_paulista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    cqp9_comparar_virada("cqp9_virada_do_paulista", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_paulista(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I5%%);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_paulista", this, || o(this))
}
