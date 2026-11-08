use super::*;




/// The J1 has two stages (0 and 1, each a round-robin league) and the final (stage 2): the next stage moves the current
/// stage on and, at the end of the second stage, builds the final; past the last stage it is an error (the box)
/// Original: `JapJ1::advance_stage` (0x6639c0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6639c0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::advance_stage()`, a fase seguinte da J1 (dois turnos, as fases 0
/// e 1, e a final, a fase 2). Na última fase (+0x30 ≥ +0x2c − 1, com sinal): a caixa da linha 0x223 e nada mais. Senão
/// +0x30 sobe um e, ao chegar a 2, monta a final (0x663a50, call direto com o ecx da liga).
pub(crate) unsafe extern "thiscall" fn jap_j1_advance_stage(this: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x9eabc8, 0x223);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 2 {
        let final_: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_JAPJ1_CREATE_CHAMPIONSHIP_FINAL_STAGE);
        final_(this);
    }
}

/// The J1 championship final: the first-stage winner against the second-stage winner, over two games [inference: home
/// and away], with no seeds; unlike the Brazilian state leagues, it does not check for the same club (the final is
/// built anyway)
/// Original: `JapJ1::create_championship_final_stage` (0x663a50, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x663a50 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_championship_final_stage()`, a final da J1 entre os
/// campeões dos dois turnos (o clube da primeira linha da tabela de cada um), sempre (sem conferir se é o mesmo clube):
/// a lista de 2 (malloc 0x944e46 de 8; sem memória: a caixa da linha 0x245 e volta); a virtual +0x3c
/// (create_round_table: thiscall(this, fase 2, &rodadas, &extra, 0), ret 0x10); a fase nova (operator new 0x944cf1 de
/// 0xb2; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 2, 1, o extra, 0x14, 0, 0, 0 e nenhuma semente) na terceira vaga das fases ([+0xc]+8); a lista e a tabela liberadas
/// (free 0x9452ca, nessa ordem); Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4) chamada com a fase da terceira
/// vaga mesmo nula (como na original); sem fase, a caixa da linha 0x25c; com ela, +0x30 = 2. O quadro SEH da original
/// só cobre o construtor (sem destrutor).
pub(crate) unsafe extern "thiscall" fn create_championship_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let lista = alocar(8);
    if lista == 0 {
        return show_error_at(0x9eabc8, 0x245);
    }
    core::ptr::write_unaligned(lista as *mut usize, half_champion(this, 0)); // cqq9: o campeão do primeiro turno
    core::ptr::write_unaligned((lista + 4) as *mut usize, half_champion(this, 1)); // cqq9: o campeão do segundo turno
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 2, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 2, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 8) as *mut usize, fase);
    liberar(lista);
    liberar(t);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(read_usize(read_usize(this + 0xc) + 8), 0);
    if read_usize(read_usize(this + 0xc) + 8) == 0 {
        return show_error_at(0x9eabc8, 0x25c);
    }
    crate::comparacao::ramo("cqq_9 final");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 2);
}

/// The Japanese turnover, led by the J1: first promotion and relegation with the J2; then the J1 is rebuilt with three
/// stages (the two halves and the final), next year, rules, the table and the fixtures; then the J2 turns over,
/// followed by the predicted positions, the board expectations and the referee renewal
/// Original: `JapJ1::start_new_season` (0x6636d0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6636d0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::start_new_season()`, a virada do futebol japonês (a J1 comanda):
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
pub(crate) unsafe extern "thiscall" fn jap_j1_start_new_season(this: usize) -> u32 {
    let virtual_ = |c: usize, desl: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + desl));
        f(c)
    };
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0xb0));
    cadeia(this, 1);
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
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
    liberar(read_usize(this + 0xc));
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, 3);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alocar(0xc);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if v == 0 {
        show_error_at(0x9eabc8, 0x1b4);
        for f in [0x5cc7d0usize, 0x5a8f60] {
            let f: unsafe extern "C" fn() = core::mem::transmute(f);
            f();
        }
        let sair: unsafe extern "C" fn(i32) = core::mem::transmute(FN_CRT_EXIT);
        sair(-1);
    }
    let a = core::ptr::read_unaligned((this + 0x40) as *const i16);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1)); // cqq9: o ano seguinte da J1
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    crate::comparacao::ramo("cqq_9 virada da j1");
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
    virtual_(read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2)), 8);
    virtual_(this, 0x5c);
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4))
}

/// Comparer for `jap_j1_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (ponytail: ao chegar a 2 monta a final, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (a final grava 2, o mesmo valor).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_advance_stage(this: usize) {
    let antes = read_usize(this + 0x30) as i32;
    let ultima = antes >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_JAP_J1_ADVANCE_STAGE)(this);
    let depois = read_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { antes } else { antes + 1 }, || format!("jap_j1_advance_stage({:#x}) antes={} depois={}", this, antes, depois));
}

/// Comparer for `create_championship_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os campeões dos turnos. Depois, a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os dois campeões
/// na lista dela (+0xa7, 6 bytes; como conjunto), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_create_championship_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_CHAMPIONSHIP_FINAL_STAGE)(this);
    let fase = read_usize(read_usize(this + 0xc) + 8);
    let igual = fase != 0 && {
        let l = read_usize(fase + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(fase + 0x3e) == 2 && read_usize(fase + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
    };
    PLACAR.registrar(igual, || format!("create_championship_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x} +0x30={}", this, a, b, fase, read_usize(this + 0x30)));
}

/// Comparer for `jap_j1_start_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da virada: só a original (ponytail: libera e aloca tabelas e fases, sorteia jogos e árbitros e vira a
/// J2; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um e +0x2c = 3.
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_start_new_season(this: usize) -> u32 {
    let antes = read_i16(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_START_NEW_SEASON)(this);
    let depois = read_i16(this + 0x40);
    PLACAR.registrar(depois == antes.wrapping_add(1) && read_usize(this + 0x2c) == 3, || format!("jap_j1_start_new_season({:#x}) ano antes={} depois={} +0x2c={}", this, antes, depois, read_usize(this + 0x2c)));
    r
}



/// True if the club's division (+0x57) is non-null and its id equals the competition id in global g.
///
/// [pt] O clube é da divisão do global `g`: a divisão do clube (+0x57) não é nula e o id do registro (+0) é o do global.
pub(crate) unsafe fn is_in_division(clube: usize, g: usize) -> bool {
    let comp = read_usize(clube + club::DIVISION);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// Sets +0x3e = n and allocates the league table (n x 0x3b) at +0xb1; error box on out of memory; returns the table.
///
/// [pt] +0x3e = `n` e a tabela (malloc 0x944e46 de n × 0x3b, em +0xb1; sem memória: a caixa do fonte `arquivo` na linha
/// `linha`). Devolve a tabela (0 sem memória).
pub(crate) unsafe fn alloc_league_table(this: usize, n: i16, arquivo: usize, linha: u32) -> usize {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(arquivo, linha);
    }
    t
}

/// League init_table_row (0x687430) for row (k + girar) % n with the club; girar is 0 except in mutations.
///
/// [pt] League::init_table_row (0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10). `girar` é 0 (o original: a linha
/// k); existe só para as mutações de cada liga. ponytail: a original empurra a linha no eax com bits de cima de sobra;
/// a 0x687430 só lê o byte.
pub(crate) unsafe fn jap_j1_init_table_row(this: usize, k: u8, n: i16, girar: u8, clube: usize) {
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    iniciar(this, (k as u32 + girar as u32) % n as u32, clube, 0, 0);
}

/// Japanese loop: every club of the division fills the next row while rows remain; the rest lose their division (+0x57
/// = 0); returns 1 if full.
///
/// [pt] O laço do Japão: percorre todos os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles); cada clube da divisão do
/// global `g`, na ordem do banco, entra na linha seguinte enquanto houver linha; os que sobram perdem a divisão
/// (+0x57 = 0). Devolve 1 se a tabela encheu.
pub(crate) unsafe fn fill_japanese_table(this: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_division(clube, g) {
            if (k as i8 as i16) < read_i16(this + 0x3e) {
                jap_j1_init_table_row(this, k, read_i16(this + 0x3e), girar, clube);
                k = k.wrapping_add(1);
            } else {
                core::ptr::write_unaligned((clube + club::DIVISION) as *mut usize, 0);
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// J1 has 16 clubs, those of the division in database order; those beyond 16 lose their division (left with no league);
/// when full, the prize fund starts at 100,000 per place and TV pays 3,000,000 to each club
/// Original: `JapJ1::create_table` (0x6634e0, jap_j1.cpp, thiscall, ret 0).
///
/// [pt] { 0x6634e0 } (thiscall, ret 0, jap_j1.cpp): `JapJ1::create_table()`: a J1 tem 16 clubes (+0x57 = [0x9cf65c]; os que
/// sobram perdem a divisão); cheia, o fundo de prêmios com a cota 100.000 (League::calc_prize_fund 0x682f70: push
/// 0x186a0; mov ecx, edi -> thiscall, ret 4) em +0x28, +0xe1 = 1, a TV 3.000.000 a cada clube (League::pay_tv_rights
/// 0x683010: push 0; push 0x2dc6c0; mov ecx, edi -> thiscall, ret 8; manda a notícia da TV) e 1. Fonte 0x9eabc8, linha
/// 0x148.
pub(crate) unsafe extern "thiscall" fn jap_j1_create_table(this: usize) -> u32 {
    if alloc_league_table(this, 0x10, 0x9eabc8, 0x148) == 0 {
        return 0;
    }
    if fill_japanese_table(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0, "cqm_9 j1 cheia") == 0 {
        return 0;
    }
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    let f = premio(this, 0x186a0);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar(this, 0x2dc6c0, 0);
    1
}

/// J2 has 12 clubs, those of the division in database order; those beyond 12 lose their division; no prize fund nor TV
/// money
/// Original: `JapJ2::create_table` (0x665030, jap_j2.cpp, thiscall, ret 0).
///
/// [pt] { 0x665030 } (thiscall, ret 0, jap_j2.cpp): `JapJ2::create_table()`: a J2 tem 12 clubes (+0x57 = [0x9cf768]; os que
/// sobram perdem a divisão), sem fundo de prêmios nem TV. Fonte 0x9eaedc, linha 0x13a. Devolve 1 se encheu.
pub(crate) unsafe extern "thiscall" fn jap_j1_create_table_665030(this: usize) -> u32 {
    if alloc_league_table(this, 0xc, 0x9eaedc, 0x13a) == 0 {
        return 0;
    }
    fill_japanese_table(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, 0, "cqm_9 j2 cheia")
}


/// Comparer: plans the division's clubs, runs only the original and checks count, return value, rows, cleared divisions
/// (Japan) and prize fund (J1).
///
/// [pt] O comparador dos três: a original aloca a tabela (e a J1 paga a TV com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (todos). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0), as
/// linhas da tabela (+0 o clube, 0x3b bytes) com os primeiros `n` do plano, na ordem; no Japão (`limpar`), os que
/// sobram do plano com a divisão zerada; na J1 cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn compare_japanese_table(nome: &str, this: usize, n: i16, g: usize, limpar: bool, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_division(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let cabem = plano.len().min(n as usize);
    let esperado = (plano.len() >= n as usize) as u32;
    let mut igual = r == esperado && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..cabem).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou[..] == plano[..cabem];
        if limpar {
            igual = igual && plano[cabem..].iter().all(|&c| read_usize(c + 0x57) == 0);
        }
        if r == 1 && cota != 0 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

/// Comparer for `jap_j1_create_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_CREATE_TABLE);
    compare_japanese_table("jap_j1_create_table", this, 0x10, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, true, 0x186a0, || o(this))
}

/// Comparer for `jap_j1_create_table_665030` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_create_table_665030(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_CREATE_TABLE_665030);
    compare_japanese_table("jap_j1_create_table_665030", this, 0xc, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, true, 0, || o(this))
}

/// J1: double round robin (the two stages of the year, created by add_result 0x663940 [inference]); win 3, draw 1;
/// tie-break by goal difference, goals scored and away goals; no top places in the overall table (rule word 8, no
/// champion band: the champion comes from the final between the stage winners [inference]); 2 go down to J2; no league
/// above; 5 bench substitutes and 3 substitutions; fixed squad numbers; the word +0xab = 0x82 copied to each match.
/// Then the schedule
/// Original: `JapJ1::set_up_rules` (0x663620, jap_j1.cpp, thiscall).
///
/// [pt] { 0x663620 } (thiscall, jap_j1.cpp): `JapJ1::set_up_rules`, as regras da J1 e o calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn jap_j1_set_up_rules(this: usize) -> u32 {
    j1_rules(this);
    crate::comparacao::ramo("cqj_7 regras da j1");
    create_main_schedule(this);
    1
}

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_SET_UP_RULES)(this), || {
        j1_rules(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("jap_j1_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// One half of the J1: when the first game of a half is processed, it builds that half's round-robin stage (a league
/// with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and
/// one top place, the half winner's); the first half goes to stage 0 and the second (from round 16, by the match +0x34)
/// to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `JapJ1::create_half_season_stage` (0x663c20, jap_j1.cpp, thiscall, ret 4).
///
/// [pt] { 0x663c20 } (thiscall, ret 4, jap_j1.cpp): `JapJ1::create_half_season_stage(k)`, um turno da J1 (a caixa da linha
/// 0x278; o 20º argumento do construtor é 0xf).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: os pontos por vitória na J1
}

/// Comparer for `jap_j1_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_JAP_J1_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("jap_j1_create_half_season_stage", this, k, || o(this, k))
}
