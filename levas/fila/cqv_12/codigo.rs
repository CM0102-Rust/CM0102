// ---------- %%LEVA%%a leva: a virada da temporada das copas da CONMEBOL: a Libertadores, a Mercosul e a Merconorte (conmebol_liber.cpp, conmebol_merc.cpp, con_merc_cup.cpp) ----------

unsafe fn cqv12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A primeira metade da virada das três copas (o mesmo corpo): +0x4c = 0 e a tabela liberada (League::free_table
/// 0x687970(this, 0): push 0; mov ecx -> thiscall, ret 4); o calendário +0xba liberado (0x9452ca: push; add esp, 4 ->
/// cdecl) e zerado; com estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50(+8, 1): push 1; mov ecx ->
/// thiscall, ret 4); a lista dos convidados +0xee liberada (sem zerar); com a fase atual (+0x30) ≥ 0, as fases 0..=+0x30
/// que existem destruídas (a virtual 0 com 1: push 1; mov ecx -> thiscall).
unsafe fn cqv12_soltar(this: usize) {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687970usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
    cqv12_pd(this + 0x4c, 0);
    liberar_tabela(this, 0);
    let l = cqv12_d(this + 0xba);
    if l != 0 {
        liberar(l);
        cqv12_pd(this + 0xba, 0);
    }
    let est = cqv12_d(this + 8);
    if est != 0 {
        arquivar(est, 1);
    }
    let c = cqv12_d(this + 0xee);
    if c != 0 {
        liberar(c);
    }
    if (cqv12_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqv12_d(cqv12_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqv12_d(cqv12_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqv12_d(this + 0x30) as i32 {
                break;
            }
        }
    }
}

/// O ano (+0x40, word) sobe um e a fase atual (+0x30) volta a −1.
unsafe fn cqv12_ano_seguinte(this: usize) {
    let ano = (this + 0x40) as *mut i16;
    core::ptr::write_unaligned(ano, core::ptr::read_unaligned(ano).wrapping_add(1));
    cqv12_pd(this + 0x30, usize::MAX);
}

/// As regras de novo (a virtual +0x8c, thiscall, ret 0); 0: a caixa da linha `linha` e 0 (a original devolve o eax da
/// caixa; quem chama não lê). Devolve a devolução das regras.
unsafe fn cqv12_regras(this: usize, arquivo: usize, linha: u32) -> u32 {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqv12_d(cqv12_d(this) + 0x8c));
    let r = regras(this);
    if r == 0 {
        show_error_at(arquivo, linha);
    }
    r
}

/// { 0x4c3980 } (thiscall, ret 0, conmebol_liber.cpp; a virtual da virada): `ConmebolLiber::start_new_season()`: a
/// virada da Libertadores: a primeira metade (`cqv12_soltar`), o ano seguinte, a semeadura da CONMEBOL para a temporada
/// nova (ConmebolSeeding::update_for_new_season 0x4c61b0([0xae1000], 0): push 0; mov ecx -> thiscall, ret 4: as vagas
/// das nações), +0x45 = 0 (o sorteio a fazer) e as regras (a caixa da linha 0x1e3).
unsafe extern "thiscall" fn cqv12_libertadores_virada(this: usize) -> u32 {
    let semear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4c61b0usize);
    cqv12_soltar(this);
    crate::comparison::branch("cqv_12 virada da libertadores");
    cqv12_ano_seguinte(this); // cqv12: a Libertadores do ano seguinte
    semear(cqv12_d(0xae1000), 0); // cqv12: as vagas da Libertadores nova
    cqv12_pd(this + 0x45, 0); // cqv12: a Libertadores nova ainda sem sorteio
    cqv12_regras(this, 0x9a7de4, 0x1e3)
}

// ponytail: a virada libera tabelas e fases e arquiva as estatísticas: não roda duas vezes. Só a original; o plano: +0x4c
// = 0, o ano + 1 e (Libertadores e Mercosul) +0x45 = 0 (o calendário +0xba as regras novas montam de novo).
unsafe fn cqv12_comparar(nome: &str, this: usize, zera_45: bool, o: impl FnOnce() -> u32) -> u32 {
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let r = o();
    let novo = core::ptr::read_unaligned((this + 0x40) as *const i16);
    let ok = cqv12_d(this + 0x4c) == 0 && novo == ano.wrapping_add(1) && (!zera_45 || cqv12_d(this + 0x45) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}): ano {} -> {}, +0x4c {}, +0xba {:#x}, +0x45 {}", nome, this, ano, novo, cqv12_d(this + 0x4c), cqv12_d(this + 0xba), cqv12_d(this + 0x45)));
    r
}

unsafe extern "thiscall" fn comparar_cqv12_libertadores_virada(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    cqv12_comparar("cqv12_libertadores_virada", this, true, || o(this))
}

/// { 0x4c5040 } (thiscall, ret 0, conmebol_merc.cpp): `ConmebolMerc::start_new_season()`: a virada da Mercosul: a
/// primeira metade, o ano seguinte, +0x45 = 0 e as regras (a caixa da linha 0x1c6).
unsafe extern "thiscall" fn cqv12_mercosul_virada(this: usize) -> u32 {
    cqv12_soltar(this);
    cqv12_ano_seguinte(this);
    cqv12_pd(this + 0x45, 0);
    cqv12_regras(this, 0x9a7e5c, 0x1c6)
}

unsafe extern "thiscall" fn comparar_cqv12_mercosul_virada(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    cqv12_comparar("cqv12_mercosul_virada", this, true, || o(this))
}

/// { 0x4c1fb0 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::start_new_season()`: a virada da Merconorte: a
/// primeira metade, o ano seguinte e as regras; só com elas +0x45 = 0 (a caixa da linha 0x1b1 deixa +0x45 como estava).
unsafe extern "thiscall" fn cqv12_merconorte_virada(this: usize) -> u32 {
    cqv12_soltar(this);
    cqv12_ano_seguinte(this);
    let r = cqv12_regras(this, 0x9a7d6c, 0x1b1);
    if r != 0 {
        cqv12_pd(this + 0x45, 0);
    }
    r
}

unsafe extern "thiscall" fn comparar_cqv12_merconorte_virada(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    cqv12_comparar("cqv12_merconorte_virada", this, false, || o(this))
}
