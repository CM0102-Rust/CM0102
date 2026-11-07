// ---------- 280a leva: a virada da temporada e a fase seguinte dos estaduais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------


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
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        liberar(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let estatisticas = read_usize(this + 8);
    if estatisticas != 0 {
        let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        arquivar(estatisticas, 1);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(ano));
    crate::comparacao::ramo(ramo);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(arquivo, linhas[0]);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return 0;
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    previstas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(0xb74340), read_usize(this + 4))
}

/// A fase seguinte de um estadual (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o mata-mata
/// (`mata_mata`, call direto com o ecx da liga -> thiscall, ret 0).
unsafe fn cqp9_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
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
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = read_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("{}({:#x}) antes={} depois={}", nome, this, antes, depois));
}

/// The Pernambuco turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegPern::start_new_season` (0x4394d0, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x4394d0 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::start_new_season()`, a virada da temporada do Pernambucano (a tabela:
/// 0x439330; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_pernambucano(this: usize) -> u32 {
    cqp9_virada(this, 0x439330, 0x98afd8, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do pernambucano") // cqp9: o ano seguinte do Pernambucano
}

/// The Pernambuco next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegPern::advance_stage` (0x439680, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439680 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::advance_stage()`, a fase seguinte do Pernambucano: no fim dos pontos corridos,
/// o mata-mata de 4 (0x439710); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_pernambucano(this: usize) {
    cqp9_fase_seguinte(this, 0x98afd8, 0x1f8, 0x439710);
}

/// The Santa Catarina turnover: archives the season in the statistics, clears the previous table and stages, moves to
/// the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table
/// order; then the predicted positions and the renewal of the state referees
/// Original: `BraRegSanta::start_new_season` (0x43bd10, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bd10 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::start_new_season()`, a virada da temporada do Catarinense (a tabela:
/// 0x43bb70; caixas nas linhas 0x1ca, 0x1d1, 0x1d7 e 0x1de).
unsafe extern "thiscall" fn cqp9_virada_do_catarinense(this: usize) -> u32 {
    cqp9_virada(this, 0x43bb70, 0x98b050, [0x1ca, 0x1d1, 0x1d7, 0x1de], 1, "cqp_9 virada do catarinense") // cqp9: o ano seguinte do Catarinense
}

/// The Santa Catarina next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to
/// the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSanta::advance_stage` (0x43bec0, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bec0 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::advance_stage()`, a fase seguinte do Catarinense: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43bf50); a caixa da linha 0x1f8.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_catarinense(this: usize) {
    cqp9_fase_seguinte(this, 0x98b050, 0x1f8, 0x43bf50);
}

/// The Sao Paulo turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegSp::start_new_season` (0x43cd40, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cd40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::start_new_season()`, a virada da temporada do Paulista (a tabela:
/// 0x43cc40; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
unsafe extern "thiscall" fn cqp9_virada_do_paulista(this: usize) -> u32 {
    cqp9_virada(this, 0x43cc40, 0x98b090, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_9 virada do paulista") // cqp9: o ano seguinte do Paulista
}

/// The Sao Paulo next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegSp::advance_stage` (0x43cef0, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cef0 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::advance_stage()`, a fase seguinte do Paulista: no fim dos pontos corridos,
/// o mata-mata de 4 (0x43cf80); a caixa da linha 0x1ea.
unsafe extern "thiscall" fn cqp9_fase_seguinte_do_paulista(this: usize) {
    cqp9_fase_seguinte(this, 0x98b090, 0x1ea, 0x43cf80);
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_pernambucano(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1938);
    cqp9_comparar_virada("cqp9_virada_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_pernambucano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1939);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_pernambucano", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_catarinense(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1940);
    cqp9_comparar_virada("cqp9_virada_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_catarinense(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1941);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_catarinense", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_virada_do_paulista(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1942);
    cqp9_comparar_virada("cqp9_virada_do_paulista", this, || o(this))
}

unsafe extern "thiscall" fn comparar_cqp9_fase_seguinte_do_paulista(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1943);
    cqp9_comparar_fase_seguinte("cqp9_fase_seguinte_do_paulista", this, || o(this))
}

// ---------- 277a leva: as semifinais dos estaduais e regionais: Pernambucano, Catarinense e Paulista (bra_reg_pern.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------



/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn8_clube(liga: usize, l: usize) -> usize {
    read_usize(read_usize(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn8_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn8_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, fase);
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// Pernambuco: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegPern::create_semi_final_stage` (0x439710, bra_reg_pern.cpp, thiscall, ret 0).
///
/// [pt] { 0x439710 } (thiscall, ret 0, bra_reg_pern.cpp): `BraRegPern::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98afd8, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_pernambucano(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98afd8, 0x21b, 0x22f, "cqn_8 pernambucano"); // cqn8: 1º × 4º, 2º × 3º Pernambucano
}

/// Santa Catarina: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is
/// the seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSanta::create_semi_final_stage` (0x43bf50, bra_reg_santa.cpp, thiscall, ret 0).
///
/// [pt] { 0x43bf50 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b050, linhas 0x21b (sem lista) e 0x22f (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_catarinense(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b050, 0x21b, 0x22f, "cqn_8 catarinense"); // cqn8: 1º × 4º, 2º × 3º Catarinense
}

/// Sao Paulo: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegSp::create_semi_final_stage` (0x43cf80, bra_reg_sp.cpp, thiscall, ret 0).
///
/// [pt] { 0x43cf80 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98b090, linhas 0x20d (sem lista) e 0x221 (sem fase).
unsafe extern "thiscall" fn cqn8_mata_mata_do_paulista(this: usize) {
    cqn8_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98b090, 0x20d, 0x221, "cqn_8 paulista"); // cqn8: 1º × 4º, 2º × 3º Paulista
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn8_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn8_clube(this, l)).collect();
    original();
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = read_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| read_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) as usize == linhas.len() && read_usize(fase + 4) == read_usize(this + 4) && read_usize(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, read_i16(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_pernambucano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1846);
    cqn8_comparar("cqn8_mata_mata_do_pernambucano", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_catarinense(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1847);
    cqn8_comparar("cqn8_mata_mata_do_catarinense", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn8_mata_mata_do_paulista(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1848);
    cqn8_comparar("cqn8_mata_mata_do_paulista", this, &[0, 3, 1, 2], || o(this))
}

