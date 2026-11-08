// ---------- %%LEVA%%a leva: a J1: a fase seguinte, a final entre os campeões dos dois turnos e a virada da temporada (jap_j1.cpp) ----------

unsafe fn cqq9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqq9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O campeão de um turno: o clube da primeira linha da tabela (+0xb1) da fase `k` ([+0xc][k], um League_Stage).
unsafe fn cqq9_campeao_do_turno(this: usize, k: usize) -> usize {
    cqq9_d(cqq9_d(cqq9_d(cqq9_d(this + 0xc) + 4 * k) + 0xb1))
}

/// { 0x6639c0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::advance_stage()`, a fase seguinte da J1 (dois turnos, as fases 0
/// e 1, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x223 e nada mais. Senão
/// +0x30 sobe um e, ao chegar a 2, monta a final (0x663a50, call direto com o ecx da liga).
unsafe extern "thiscall" fn jap_j1_advance_stage(this: usize) {
    let fase = cqq9_d(this + 0x30) as i32;
    if fase >= (cqq9_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x9eabc8, 0x223);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x663a50usize);
        final_(this);
    }
}

/// { 0x663a50 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_championship_final_stage()`, a final da J1 entre os
/// campeões dos dois turnos (o clube da primeira linha da tabela de cada um), sempre (sem conferir se é o mesmo clube):
/// a lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x245 e volta); a virtual +0x3c
/// (create_round_table: thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10); a fase nova (operator new 0x944cf1 de
/// 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 2, 1, o extra, 0x14, 0, 0, 0 e nenhuma semente) na terceira vaga das fases ([+0xc]+8); a lista e a tabela liberadas
/// (free 0x9452ca, nessa ordem); Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4) chamada com a fase da terceira
/// vaga mesmo nula (como na original); sem fase, a caixa da linha 0x25c; com ela, +0x30 = 2. O quadro SEH da original
/// só cobre o construtor (sem destrutor).
unsafe extern "thiscall" fn create_championship_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0x9eabc8, 0x245);
    }
    core::ptr::write_unaligned(lista as *mut usize, cqq9_campeao_do_turno(this, 0)); // cqq9: o campeão do primeiro turno
    core::ptr::write_unaligned((lista + 4) as *mut usize, cqq9_campeao_do_turno(this, 1)); // cqq9: o campeão do segundo turno
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqq9_d(cqq9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, lista, rodadas as u16 as u32, cqq9_d(this + 4), t, cqq9_w(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned((cqq9_d(this + 0xc) + 8) as *mut usize, fase);
    liberar(lista);
    liberar(t);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(cqq9_d(cqq9_d(this + 0xc) + 8), 0);
    if cqq9_d(cqq9_d(this + 0xc) + 8) == 0 {
        return show_error_at(0x9eabc8, 0x25c);
    }
    crate::comparison::branch("cqq_9 final");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// { 0x6636d0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::start_new_season()`, a virada do futebol japonês (a J1 comanda):
/// +0x4c = 0; o acesso e o rebaixamento (a virtual +0xb0, thiscall(this, 1), ret 4); a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada e zerada; com estatísticas (+8),
/// Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30
/// que existem destruídas (a virtual 0 com 1); o vetor das fases liberado e um novo de 3 (malloc 0x944e46 de 0xc,
/// +0x2c = 3; sem memória: a caixa da linha 0x1b4, a limpeza 0x5cc7d0, 0x5a8f60 e exit(−1) 0x945934, todas cdecl); o
/// ano (+0x40) sobe um e +0x30 = −1; então, cada um com a sua caixa se devolver 0: as regras (virtual +0x8c, 0x1be), a
/// tabela (0x6634e0, 0x1c5), os jogos (League::create_fixtures 0x6835c0, 0x1cb) e a ordem (League::sort_table 0x6827d0,
/// thiscall(this, 0), 0x1d2); depois a virada da J2 (a virtual +8 da competição de id [0x9cf768]), as posições previstas
/// (virtual +0x5c), as expectativas das diretorias (League::set_club_season_expectations 0x68aa80) e a renovação dos
/// árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), cujo eax ela devolve.
unsafe extern "thiscall" fn jap_j1_start_new_season(this: usize) -> u32 {
    let virtual_ = |c: usize, desl: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqq9_d(cqq9_d(c) + desl));
        f(c)
    };
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(cqq9_d(cqq9_d(this) + 0xb0));
    cadeia(this, 1);
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x687970usize);
    liberar_tabela(this, 0);
    let l = cqq9_d(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = cqq9_d(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
        arquivar(estatisticas, 1);
    }
    if (cqq9_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqq9_d(cqq9_d(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq9_d(cqq9_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqq9_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    liberar(cqq9_d(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0xc);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if v == 0 {
        show_error_at(0x9eabc8, 0x1b4);
        for f in [0x5cc7d0usize, 0x5a8f60] {
            let f: unsafe extern "C" fn() = core::mem::transmute(f);
            f();
        }
        let sair: unsafe extern "C" fn(i32) = core::mem::transmute(0x945934usize);
        sair(-1);
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1)); // cqq9: o ano seguinte da J1
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    crate::comparison::branch("cqq_9 virada da j1");
    let erro = if virtual_(this, 0x8c) == 0 {
        0x1be
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6634e0usize)(this) == 0 {
        0x1c5
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6835c0usize)(this) == 0 {
        0x1cb
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, usize) -> u32>(0x6827d0usize)(this, 0) == 0 {
        0x1d2
    } else {
        0
    };
    if erro != 0 {
        show_error_at(0x9eabc8, erro);
        return 0;
    }
    virtual_(cqq9_d(cqq9_d(0xadadfc) + 4 * cqq9_d(0x9cf768)), 8);
    virtual_(this, 0x5c);
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68aa80usize);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x79cee0usize);
    renovar(cqq9_d(0xb74340), cqq9_d(this + 4))
}

/// O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (a final grava 2, o mesmo valor).
unsafe extern "thiscall" fn comparar_jap_j1_advance_stage(this: usize) {
    let antes = cqq9_d(this + 0x30) as i32;
    let ultima = antes >= (cqq9_d(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let depois = cqq9_d(this + 0x30) as i32;
    PLACAR.record(depois == if ultima { antes } else { antes + 1 }, || format!("jap_j1_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os campeões dos turnos. Depois, a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os dois campeões
/// na lista dela (+0xa7, 6 bytes; como conjunto), e +0x30 = 2.
unsafe extern "thiscall" fn comparar_create_championship_final_stage(this: usize) {
    let (a, b) = (cqq9_campeao_do_turno(this, 0), cqq9_campeao_do_turno(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let fase = cqq9_d(cqq9_d(this + 0xc) + 8);
    let igual = fase != 0 && {
        let l = cqq9_d(fase + 0xa7);
        let (x, y) = (cqq9_d(l), cqq9_d(l + 6));
        cqq9_w(fase + 0x3e) == 2 && cqq9_d(fase + 4) == cqq9_d(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && cqq9_d(this + 0x30) == 2
    };
    PLACAR.record(igual, || format!("create_championship_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x} +0x30={}", this, a, b, fase, cqq9_d(this + 0x30)));
}

/// O comparador da virada: só a original (ponytail: libera e aloca tabelas e fases, sorteia jogos e árbitros e vira a
/// J2; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um e +0x2c = 3.
unsafe extern "thiscall" fn comparar_jap_j1_start_new_season(this: usize) -> u32 {
    let antes = cqq9_w(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let depois = cqq9_w(this + 0x40);
    PLACAR.record(depois == antes.wrapping_add(1) && cqq9_d(this + 0x2c) == 3, || format!("jap_j1_start_new_season({:#x}) ano antes={} depois={} +0x2c={}", this, antes, depois, cqq9_d(this + 0x2c)));
    r
}
