// ---------- %%LEVA%%a leva: a virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

unsafe fn cqp7_d(a: usize) -> usize {
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
unsafe fn state_league_new_season(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16, ramo: &'static str) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = cqp7_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqp7_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqp7_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqp7_d(cqp7_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqp7_d(cqp7_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqp7_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparison::branch(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        show_error_at(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        show_error_at(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 {
        show_error_at(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqp7_d(cqp7_d(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqp7_d(0xb74340), cqp7_d(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn state_league_advance_stage(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = cqp7_d(this + 0x30) as i32;
    if fase >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(arquivo, linha);
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
unsafe fn compare_state_league_new_season(nome: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let antes = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = original();
    let depois = core::ptr::read_unaligned((this + 0x40) as *const i16);
    PLACAR.record(depois == antes.wrapping_add(1), || format!("{}({:#x}) ano antes={} depois={}", nome, this, antes, depois));
    r
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 0 monta o mata-mata, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (o mata-mata grava 0, o mesmo valor).
unsafe fn compare_state_league_advance_stage(nome: &str, this: usize, original: impl FnOnce()) {
    let antes = cqp7_d(this + 0x30) as i32;
    let ultima = antes >= (cqp7_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqp7_d(this + 0x30) as i32;
    PLACAR.record(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// { 0x42fe10 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::start_new_season()`, a virada da temporada do Baiano (a tabela:
/// 0x42fd10; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn bra_reg_bahia_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x42fd10, 0x98acac, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_7 virada do baiano") // cqp7: o ano seguinte do Baiano
}

/// { 0x42ffc0 } (thiscall, ret 0, bra_reg_bahia.cpp): `BraRegBahia::advance_stage()`, a fase seguinte do Baiano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x430050); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn bra_reg_bahia_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98acac, 0x1f8, 0x430050);
}

/// { 0x431090 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::start_new_season()`, a virada da temporada do Centro-Oeste (a tabela:
/// 0x430ef0; caixas nas linhas 0x1cb, 0x1d2, 0x1d8 e 0x1df).
unsafe extern "thiscall" fn bra_reg_central_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x430ef0, 0x98ae10, [0x1cb, 0x1d2, 0x1d8, 0x1df], 1, "cqp_7 virada do centro_oeste") // cqp7: o ano seguinte do Centro-Oeste
}

/// { 0x431240 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::advance_stage()`, a fase seguinte do Centro-Oeste: no fim dos pontos corridos,
/// o mata-mata de 8 (0x4312d0); a caixa da linha 0x1f9.
unsafe extern "thiscall" fn bra_reg_central_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98ae10, 0x1f9, 0x4312d0);
}

/// { 0x432370 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::start_new_season()`, a virada da temporada do Gaúcho (a tabela:
/// 0x4321d0; caixas nas linhas 0x1c6, 0x1cd, 0x1d3 e 0x1da).
unsafe extern "thiscall" fn bra_reg_gaucho_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x4321d0, 0x98ae50, [0x1c6, 0x1cd, 0x1d3, 0x1da], 1, "cqp_7 virada do gaucho") // cqp7: o ano seguinte do Gaúcho
}

/// { 0x432520 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::advance_stage()`, a fase seguinte do Gaúcho: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4325b0); a caixa da linha 0x1f4.
unsafe extern "thiscall" fn bra_reg_gaucho_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98ae50, 0x1f4, 0x4325b0);
}

unsafe extern "thiscall" fn comparar_bra_reg_bahia_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("bra_reg_bahia_start_new_season", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_bahia_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_state_league_advance_stage("bra_reg_bahia_advance_stage", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_central_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("bra_reg_central_start_new_season", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_central_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    compare_state_league_advance_stage("bra_reg_central_advance_stage", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_gaucho_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    compare_state_league_new_season("bra_reg_gaucho_start_new_season", this, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_gaucho_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%);
    compare_state_league_advance_stage("bra_reg_gaucho_advance_stage", this, || o(this))
}
