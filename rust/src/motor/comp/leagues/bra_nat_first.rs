use super::*;

// ---------- 280a leva: os play-offs das Séries A e B: a fase seguinte e o mata-mata dos 4 primeiros (bra_nat_first.cpp, bra_nat_second.cpp) ----------




/// O play-off de uma Série (o mesmo molde na A e na B, conferido instrução a instrução: só mudam o fonte e as linhas
/// das caixas): a lista dos 4 (malloc 0x944e46 de 0x10; sem memória: a caixa `linha_sem_lista` e volta) com os clubes
/// das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3); a virtual +0x3c (create_round_table: thiscall(this,
/// fase 0, &rodadas, &extra, 0), ret 0x10) monta a tabela de rodadas; a fase nova (operator new 0x944cf1 de 0xb2;
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 4, a lista, as rodadas, +4, a tabela, o ano +0x40, 0, 1,
/// o extra, 0x14, 0, 0, 0 e nenhuma semente: a ordem não é por sementes) vai para a primeira vaga das fases ([+0xc]) e a
/// tabela de rodadas é liberada (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`, a lista liberada e volta. Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a lista liberada e +0x30 = 0. O quadro SEH da original só
/// cobre o construtor (sem destrutor).
pub(crate) unsafe fn cqq7_play_off(this: usize, linhas: &[usize; 4], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn7_clube(this, l));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no dx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 4, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    core::ptr::write_unaligned(read_usize(this + 0xc) as *mut usize, fase);
    liberar(t);
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        aviso_em(arquivo, linha_sem_fase);
        liberar(lista);
        return;
    }
    crate::comparacao::ramo(ramo);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    montar(fase, 0);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// A fase seguinte de uma Série (`advance_stage`, slot 11, +0x28): na última fase (+0x30 ≥ +0x2c − 1, com sinal), a
/// caixa de erro e nada mais; senão +0x30 sobe um e, ao chegar a 0 (o fim dos pontos corridos), monta o play-off
/// (`play_off`, call direto com o ecx da liga -> thiscall, ret 0).
pub(crate) unsafe fn cqq7_fase_seguinte(this: usize, arquivo: usize, linha: u32, play_off: usize) {
    let fase = read_usize(this + 0x30) as i32;
    if fase >= (read_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    if fase == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this);
    }
}

/// Series A has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatFirst::advance_stage` (0x42a950, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a950 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::advance_stage()`, a fase seguinte da Série A: no fim
/// dos pontos corridos, o play-off (0x42a9e0); a caixa da linha 0x264.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_advance_stage(this: usize) {
    cqq7_fase_seguinte(this, 0x98aa30, 0x264, 0x42a9e0);
}

/// The Series A play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds: the list
/// order) [inference: two-legged semi-finals and final] that decides the Brazilian champion
/// Original: `BraNatFirst::create_play_off_stage` (0x42a9e0, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a9e0 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_play_off_stage()`, o play-off da Série A:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x286 (sem lista) e 0x29d (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98aa30, 0x286, 0x29d, "cqq_7 serie a"); // cqq7: 1º × 4º, 2º × 3º Série A
}

/// Series B has the round robin (stage −1) and the play-off (stage 0): when the round robin ends, the stage becomes 0
/// and the play-off is built; past the last stage it is an error (the box)
/// Original: `BraNatSecond::advance_stage` (0x42cdd0, bra_nat_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x42cdd0 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::advance_stage()`, a fase seguinte da Série B: no fim
/// dos pontos corridos, o play-off (0x42ce60); a caixa da linha 0x23d.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_advance_stage_42cdd0(this: usize) {
    cqq7_fase_seguinte(this, 0x98ab64, 0x23d, 0x42ce60);
}

/// The Series B play-off: the top 4 of the round robin, 1st v 4th and 2nd v 3rd, in a knockout (no seeds) that decides
/// the Series B champion [inference: the promotions to Series A come from it]
/// Original: `BraNatSecond::create_play_off_stage` (0x42ce60, bra_nat_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x42ce60 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_play_off_stage()`, o play-off da Série B:
/// 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2); caixas nas linhas 0x25f (sem lista) e 0x276 (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_play_off_stage_42ce60(this: usize) {
    cqq7_play_off(this, &[0, 3, 1, 2], 0x98ab64, 0x25f, 0x276, "cqq_7 serie b"); // cqq7: 1º × 4º, 2º × 3º Série B
}


/// O comparador do play-off: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = 4, +4 = o +4 da liga e os 4
/// primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada pode
/// reordenar), e +0x30 = 0.
pub(crate) unsafe fn cqq7_comparar_play_off(nome: &str, this: usize, linhas: &[usize; 4], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn7_clube(this, l)).collect();
    original();
    let fase = read_usize(read_usize(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = read_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_usize(fase + 4) == read_usize(this + 4) && read_usize(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, read_i16(fase + 0x3e)));
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_ADVANCE_STAGE);
    cqp7_comparar_fase_seguinte("bra_nat_first_advance_stage", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_CREATE_PLAY_OFF_STAGE);
    cqq7_comparar_play_off("bra_nat_first_create_play_off_stage", this, &[0, 3, 1, 2], || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_advance_stage_42cdd0(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_ADVANCE_STAGE_42CDD0);
    cqp7_comparar_fase_seguinte("bra_nat_first_advance_stage_42cdd0", this, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_play_off_stage_42ce60(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_CREATE_PLAY_OFF_STAGE_42CE60);
    cqq7_comparar_play_off("bra_nat_first_create_play_off_stage_42ce60", this, &[0, 3, 1, 2], || o(this))
}

// ---------- 280a leva: a virada do futebol brasileiro: a Série A, o acesso e o rebaixamento em cadeia e a Série B (bra_nat_first.cpp, bra_nat_second.cpp) ----------


/// A competição de id [`global`] (a tabela das competições [0xadadfc]).
pub(crate) unsafe fn cqq8_comp(global: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(global))
}

/// A virtual `deslocamento` de `c`, thiscall sem argumentos.
pub(crate) unsafe fn cqq8_virtual(c: usize, deslocamento: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + deslocamento));
    f(c)
}

/// O corpo comum da virada de uma Série (o mesmo da virada dos estaduais, cqp_7..9): a tabela liberada
/// (League::free_table 0x687970, thiscall(this, 0), ret 4); a lista de +0xba liberada (free 0x9452ca) e zerada; com
/// estatísticas (+8), Comp_Stats::archive_season_and_reset (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0,
/// as fases 0..=+0x30 que existem destruídas (a virtual 0 com 1); o ano (+0x40, word) sobe `ano` e +0x30 = −1; então,
/// cada um com a sua caixa se devolver 0: as regras (virtual +0x8c), a tabela (`tabela`, thiscall, ret 0), os jogos
/// (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); por fim as posições previstas (virtual +0x5c). Devolve falso depois de uma caixa.
pub(crate) unsafe fn cqq8_refazer(this: usize, tabela: usize, arquivo: usize, linhas: [u32; 4], ano: i16) -> bool {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
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
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    if cqq8_virtual(this, 0x8c) == 0 {
        aviso_em(arquivo, linhas[0]);
        return false;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(tabela);
    if tabela(this) == 0 {
        aviso_em(arquivo, linhas[1]);
        return false;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if jogos(this) == 0 {
        aviso_em(arquivo, linhas[2]);
        return false;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        aviso_em(arquivo, linhas[3]);
        return false;
    }
    cqq8_virtual(this, 0x5c);
    true
}

/// O fim comum: as expectativas da diretoria de cada clube (League::set_club_season_expectations 0x68aa80, thiscall,
/// ret 0) e a renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4),
/// cujo eax a virada devolve.
pub(crate) unsafe fn cqq8_expectativas_e_arbitros(this: usize) -> u32 {
    let expectativas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4)) // cqq8: os árbitros
}

/// As viradas das outras catorze competições do Brasil, na ordem da original: a Série B, a Série C e os doze estaduais
/// (Baiano, Centro-Oeste, Gaúcho, Goiano, Mineiro, Norte, Nordeste, Paranaense, Pernambucano, Carioca, Catarinense e
/// Paulista: os globais de id 0x9cf9e0, 0x9cf9ac, 0x9cf9c0, 0x9cf998, 0x9cf9c8, 0x9cf9b4, 0x9cf9d4, 0x9cf994, 0x9cf9a4,
/// 0x9cf9d0, 0x9cf9bc, 0x9cf9dc, 0x9cf9b0, 0x9cf9e4).
pub(crate) const CQQ8_VIRADAS: [usize; 14] = [KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION, KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP];

/// The Brazilian turnover, led by Series A: first the promotion and relegation chain; then the Brazilian Cup entrants,
/// the 16 best clubs without a national division into Series C and the swaps between the state leagues and their lower
/// divisions; the division tier of the A and B clubs is capped; Series A is rebuilt (next year, rules, 28 clubs,
/// fixtures); then Series B, Series C and the twelve state leagues turn over (Bahia, Centre-West, Rio Grande do Sul,
/// Goias, Minas Gerais, North, Northeast, Parana, Pernambuco, Rio, Santa Catarina and Sao Paulo), with the board
/// expectations and the referee renewal
/// Original: `BraNatFirst::start_new_season` (0x42a640, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a640 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::start_new_season()`, a virada do futebol brasileiro
/// (a Série A comanda): +0x4c = 0; o acesso e o rebaixamento em cadeia (a virtual +0xb0, 0x42ae90, thiscall(this, 1),
/// ret 4); os inscritos da Copa do Brasil (0x42b760), a entrada de 16 clubes na Série C (0x42b520) e as trocas dos
/// estaduais com as divisões de baixo (0x42b130), as três com mov ecx, esi e sem push (thiscall, ret 0; o mapa as diz
/// cdecl(void): o ecx sobra, sem efeito); o nível da divisão dos clubes da Série A e da Série B limitado
/// (League::limit_club_division_tier 0x68a980, thiscall(liga, 1, 3, −3, 1), ret 0x10); a Série A refeita (o corpo comum,
/// a tabela 0x42a480, caixas nas linhas 0x223, 0x22a, 0x230 e 0x237); as viradas das outras catorze (a virtual +8);
/// as expectativas e os árbitros.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_start_new_season(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0xb0));
    cadeia(this, 1);
    for f in [0x42b760usize, 0x42b520, 0x42b130] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this);
    }
    let nivel: unsafe extern "thiscall" fn(usize, u32, u32, i32, u32) = core::mem::transmute(FN_LEAGUE_LIMIT_CLUB_DIVISION_TIER);
    nivel(this, 1, 3, -3, 1);
    nivel(cqq8_comp(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION), 1, 3, -3, 1);
    if !cqq8_refazer(this, 0x42a480, 0x98aa30, [0x223, 0x22a, 0x230, 0x237], 1) {
        return 0;
    }
    for (k, g) in CQQ8_VIRADAS.iter().enumerate() {
        if k == 13 {
            crate::comparacao::ramo("cqq_8 virada do paulista");
        }
        cqq8_virtual(cqq8_comp(*g), 8); // cqq8: a virada de cada uma
    }
    cqq8_expectativas_e_arbitros(this)
}

/// Promotion and relegation in Brazil: records the final positions of Series A, Series B, Series C and the seven Series
/// C stages; then swaps the marked clubs between A and B, between B and C and between B and each C stage (who goes up
/// and down comes from the end-of-season marks, with no fixed number here)
/// Original: `BraNatFirst::process_promotion_relegation_chain` (0x42ae90, bra_nat_first.cpp, thiscall, ret 4).
///
/// [pt] { 0x42ae90 } (thiscall, ret 4, bra_nat_first.cpp): `BraNatFirst::process_promotion_relegation_chain(int)` (slot 45,
/// +0xb0; o argumento não é lido): as posições finais (a virtual +0xa4, record_final_positions, thiscall, ret 0) da
/// Série A, da Série B, da Série C e das sete fases da Série C ([C+0xc][0..7]); depois o acesso e o rebaixamento
/// (league_promote_and_relegate 0x689c80, thiscall(this, de cima, de baixo, 1, 1, −1, −1), ret 0x18: a original põe
/// o ecx da Série A e a rotina o guarda) entre A e B, entre B e C e entre a B e cada uma das sete fases da C. Devolve o
/// eax da última.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_process_promotion_relegation_chain(this: usize, _pares: u32) -> u32 {
    let (a, b, c) = (KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION);
    cqq8_virtual(cqq8_comp(a), 0xa4);
    cqq8_virtual(cqq8_comp(b), 0xa4);
    cqq8_virtual(cqq8_comp(c), 0xa4);
    for k in 0..7 {
        cqq8_virtual(read_usize(read_usize(cqq8_comp(c) + 0xc) + 4 * k), 0xa4);
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, i32) -> u32 = core::mem::transmute(FN_LEAGUE_PROMOTE_AND_RELEGATE);
    crate::comparacao::ramo("cqq_8 acesso e rebaixamento");
    trocar(this, cqq8_comp(a), cqq8_comp(b), 1, 1, -1, -1); // cqq8: A e B
    let mut r = trocar(this, cqq8_comp(b), cqq8_comp(c), 1, 1, -1, -1); // cqq8: B e C
    for k in 0..7 {
        r = trocar(this, cqq8_comp(b), read_usize(read_usize(cqq8_comp(c) + 0xc) + 4 * k), 1, 1, -1, -1);
    }
    r
}

/// The Series B turnover: archives the season in the statistics, clears the table and stages, moves to the next year
/// and rebuilds the rules, the table with the 28 Series B clubs, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal
/// Original: `BraNatSecond::start_new_season` (0x42cc10, bra_nat_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x42cc10 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::start_new_season()`, a virada da Série B (da
/// virada da Série A): +0x4c = 0, o corpo comum (a tabela 0x42ca70, caixas nas linhas 0x20c, 0x213, 0x219 e 0x220), as
/// expectativas e os árbitros.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_start_new_season_42cc10(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    if !cqq8_refazer(this, 0x42ca70, 0x98ab64, [0x20c, 0x213, 0x219, 0x220], 1) {
        return 0;
    }
    crate::comparacao::ramo("cqq_8 virada da serie b");
    cqq8_expectativas_e_arbitros(this)
}


pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_START_NEW_SEASON);
    cqp7_comparar_virada("bra_nat_first_start_new_season", this, || o(this))
}

/// Comparador do acesso e rebaixamento: só a original (ponytail: move clubes entre as listas das ligas, que realocam).
/// A cópia roda no modo Rust (contador "cqq_8 acesso e rebaixamento") e é provada pelas mutações e pelas carreiras.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_process_promotion_relegation_chain(this: usize, pares: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_BRA_NAT_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pares)
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_start_new_season_42cc10(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_START_NEW_SEASON_42CC10);
    cqp7_comparar_virada("bra_nat_first_start_new_season_42cc10", this, || o(this))
}

// ---------- 277a leva: a tabela das três divisões nacionais do Brasil: os 28 clubes da Série A (e o dinheiro da TV), os 28 da Série B e o sorteio dos 56 da Série C (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------



/// O clube está na competição do global `g`: o ponteiro do clube em `campo` (+0x57 a divisão nacional) não é nulo e o
/// id do registro (+0) é o do global.
pub(crate) unsafe fn cql7_da_divisao(clube: usize, campo: usize, g: usize) -> bool {
    let comp = read_usize(clube + campo);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// O corpo comum da Série A e da Série B (o mesmo molde dos estaduais): +0x3e = `n` clubes; a tabela (malloc 0x944e46
/// de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245
/// bytes, [0xae235c] deles) da competição do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada divisão.
pub(crate) unsafe fn cql7_tabela(this: usize, n: i16, arquivo: usize, linha: u32, campo: usize, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if cql7_da_divisao(clube, campo, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
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

/// Series A has 28 clubs: those whose national division is Series A, in database order; when full, TV pays 150,000 to
/// each club (no prize fund)
/// Original: `BraNatFirst::create_table` (0x42a480, bra_nat_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x42a480 } (thiscall, ret 0, bra_nat_first.cpp): `BraNatFirst::create_table()`: a Série A tem 28 clubes (os de
/// divisão nacional +0x57 = [0x9cf9ec]); cheia, a TV paga 150.000 a cada clube (League::pay_tv_rights 0x683010:
/// push 0; push 0x249f0; mov ecx, edi -> thiscall(liga, valor, filtro), ret 8; manda a notícia da TV) e devolve 1.
/// Fonte 0x98aa30, linha 0x1a1.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_table(this: usize) -> u32 {
    if cql7_tabela(this, 0x1c, 0x98aa30, 0x1a1, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, 0, "cql_7 serie a cheia") == 0 {
        return 0;
    }
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, 0x249f0, 0);
    1
}

/// Series B has 28 clubs, those of the division in database order; no TV money nor prize fund
/// Original: `BraNatSecond::create_table` (0x42ca70, bra_nat_second.cpp, thiscall, ret 0).
///
/// [pt] { 0x42ca70 } (thiscall, ret 0, bra_nat_second.cpp): `BraNatSecond::create_table()`: a Série B tem 28 clubes (+0x57 =
/// [0x9cf9e0]), sem dinheiro da TV. Fonte 0x98ab64, linha 0x1aa.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_create_table_42ca70(this: usize) -> u32 {
    cql7_tabela(this, 0x1c, 0x98ab64, 0x1aa, 0x57, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, 0, "cql_7 serie b cheia")
}


/// Os clubes da competição do global `g` (ponteiro em `campo`), na ordem do banco, até `n`.
pub(crate) unsafe fn cql7_plano(n: usize, campo: usize, g: usize) -> Vec<usize> {
    let mut plano = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && plano.len() < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * 0x245;
        if cql7_da_divisao(clube, campo, g) {
            plano.push(clube);
        }
        i += 1;
    }
    plano
}

/// O comparador da Série A e da Série B: a original aloca a tabela (e na A paga a TV com notícia): só ela. O plano,
/// feito antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano
/// encheu (senão 0) e as linhas da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn cql7_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let plano = cql7_plano(n as usize, 0x57, g);
    let r = original();
    let t = read_usize(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_table(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_CREATE_TABLE);
    cql7_comparar("bra_nat_first_create_table", this, 0x1c, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, || o(this))
}

pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_create_table_42ca70(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_CREATE_TABLE_42CA70);
    cql7_comparar("bra_nat_first_create_table_42ca70", this, 0x1c, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, || o(this))
}


// ---------- 274a leva: as regras das três divisões nacionais do Brasil (bra_nat_first.cpp, bra_nat_second.cpp, bra_nat_third.cpp) ----------




/// O calendário da fase principal pelo slot 16 da vtable (`create_schedule`, thiscall, ret 0x10, conferido nos três
/// alvos: 0x429b50, 0x42c140, 0x42d620): estágio -1, as saídas +0xa9 e +0x3a (palavras), sem a marca (0); o ponteiro
/// devolvido vai para +0xba.
pub(crate) unsafe fn cqi7_calendario(o: usize) {
    let vt = *(o as *const usize);
    let f: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(*((vt + 0x3c) as *const usize));
    let r = f(o, -1, o + 0xa9, o + 0x3a, 0);
    write_u32_at(o, 0xba, r as u32);
}

/// O objeto da liga (0xee bytes) menos as saídas do calendário (+0x3a, +0xa9, +0xba), que o ensaio não chama: os
/// três calendários não gravam nada no objeto além delas (lido nas três rotinas: nenhum `mov [edi+..]`).
pub(crate) fn cqi7_regioes(o: usize) -> [(usize, usize); 4] {
    [(o, 0x3a), (o + 0x3c, 0xa9 - 0x3c), (o + 0xab, 0xba - 0xab), (o + 0xbe, 0xee - 0xbe)]
}

/// As regras da Série A (o que a 0x42a5a0 grava antes do calendário).
pub(crate) unsafe fn cqi7_regras_serie_a(o: usize) {
    write_u16_at(o, 0x3c, 1); // um turno
    write_u8_at(o, 0xc2, 3); // cqi7: os pontos por vitória da Série A
    write_u8_at(o, 0xc3, 1); // pontos por empate
    write_u8_at(o, 0xc4, 2); // categoria das partidas: liga
    write_u8_at(o, 0x42, 1); // clube nacional
    write_u8_at(o, 0xc5, 3); // desempates: confronto direto, saldo, nenhum
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // vagas de cima
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // rebaixados
    write_u32_at(o, 0x1c, 0xffff_ffff); // nenhuma liga acima
    write_u8_at(o, 0x4a, 3); // substituições
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION as *const u32)); // cai para a Série B
    write_u16_at(o, 0xd9, 8); // bit 0x8: os de cima vão para a fase seguinte (sem acesso, sem faixa de campeão)
    write_u8_at(o, 0x52, 2); // numeração fixa do elenco
    write_u8_at(o, 0x49, 7); // reservas no banco
}

/// As regras da Série B (0x42cb70).
pub(crate) unsafe fn cqi7_regras_serie_b(o: usize) {
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqi7: os pontos por empate da Série B
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 3);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbe, 4); // quatro sobem
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 4); // quatro caem
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION as *const u32)); // sobe para a Série A
    write_u16_at(o, 0xd9, 2); // bit 0x2: as vagas de cima são de acesso
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_THIRD_DIVISION as *const u32)); // cai para a Série C
    write_u8_at(o, 0x49, 7);
}

/// As regras da Série C (0x42de50).
pub(crate) unsafe fn cqi7_regras_serie_c(o: usize) {
    write_u16_at(o, 0x3c, 2); // cqi7: os turnos da Série C
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // desempates: saldo, gols pró, gols fora
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xbe, 2);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2);
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0x52, 2);
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION as *const u32)); // sobe para a Série B
    write_u32_at(o, 0x20, 0xffff_ffff); // nenhuma liga abaixo
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 7);
}

/// Series A: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; no league above, drops
/// to Series B; 4 top places that, by the rule word 8, lead to the next stage (the play-offs; no promotion and no
/// champion band: the champion comes from the knockout) [inference: the next stage is the slot 11 one]; 4 relegated; 7
/// substitutes on the bench and 3 substitutions; fixed squad numbers; category 2 (league) matches. Then the main stage
/// schedule
/// Original: `BraNatFirst::set_up_rules` (0x42a5a0, bra_nat_first.cpp, thiscall).
///
/// [pt] { 0x42a5a0 } (thiscall, bra_nat_first.cpp): `BraNatFirst::set_up_rules` (slot 36), as regras da Série A e o
/// calendário da fase principal; devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_set_up_rules(this: usize) -> u32 {
    cqi7_regras_serie_a(this);
    crate::comparacao::ramo("cqi_7 regras da serie a");
    cqi7_calendario(this);
    1
}

/// Comparador: lado a lado com a cópia em ensaio (as regras sem o calendário, que aloca), sobre o objeto inteiro
/// menos as saídas do calendário; a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_set_up_rules(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_SET_UP_RULES)(this), || {
        cqi7_regras_serie_a(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_nat_first_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Series B: single round robin; win 3, draw 1; head-to-head and goal difference; promotes to Series A and relegates to
/// Series C; 4 go up (rule word 2: the top places are promotion) and 4 go down; 7 substitutes, 3 substitutions, fixed
/// numbers. Then the schedule
/// Original: `BraNatSecond::set_up_rules` (0x42cb70, bra_nat_second.cpp, thiscall).
///
/// [pt] { 0x42cb70 } (thiscall, bra_nat_second.cpp): `BraNatSecond::set_up_rules`, as regras da Série B e o calendário.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_set_up_rules_42cb70(this: usize) -> u32 {
    cqi7_regras_serie_b(this);
    crate::comparacao::ramo("cqi_7 regras da serie b");
    cqi7_calendario(this);
    1
}

/// Comparador: como o da Série A.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_set_up_rules_42cb70(this: usize) -> u32 {
    let reg = cqi7_regioes(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_NAT_FIRST_SET_UP_RULES_42CB70)(this), || {
        cqi7_regras_serie_b(this);
        1
    });
    PLACAR.registrar(igual && r == n, || format!("bra_nat_first_set_up_rules_42cb70({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}



// ---------- 274a leva: o destino de cada clube no fim das fases da Série A, da Série C e do Baiano (bra_nat_first.cpp, bra_nat_third.cpp, bra_reg_bahia.cpp) ----------



/// Os registros de carreira (Staff_Records em [0xdd7f00], thiscall, todos já em Rust):
/// 0x86bdd0 classificou-se (clube, comp, fase, rodada, peso; ret 0x14).
pub(crate) unsafe fn cqj9_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
    f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, fase, rodada, peso);
}

/// 0x86a1c0 rebaixado (clube, liga; ret 8).
pub(crate) unsafe fn cqj9_rebaixado(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
    f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp);
}

/// 0x868c50 campeão da liga (clube, liga; ret 8).
pub(crate) unsafe fn cqj9_campeao(clube: usize, comp: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LEAGUE_CHAMPIONS_RECORD);
    f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp);
}

/// 0x869480 promovido (clube, liga, peso; ret 0xc).
pub(crate) unsafe fn cqj9_promovido(clube: usize, comp: usize, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
    f(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, peso);
}

/// 0x68ab70 `League::find_club_row(clube, 0)` (thiscall, ret 8): a linha do clube na tabela da liga.
pub(crate) unsafe fn cqj9_linha(liga: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_LEAGUE_FIND_CLUB_ROW);
    f(liga, clube, 0)
}

/// A Série A (0x42abc0). `fazer` = falso: o ensaio do comparador, sem os registros nem a gravação; devolve
/// (devolução, linha gravada ou 0, destino gravado).
pub(crate) unsafe fn cqj9_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = read_u32_as_usize(this + 4);
    if fa == 0xff {
        // a fase de pontos corridos
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqj_9 serie a classificado");
                cqj9_classificou(clube, comp, 0x41c, 0, 0x28);
            }
        } else if st == 3 && fazer {
            cqj9_rebaixado(clube, comp);
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    // o mata-mata
    if st == 1 {
        let l = cqj9_linha(this, clube);
        if fazer {
            crate::comparacao::ramo("cqj_9 serie a campeao");
            cqj9_campeao(clube, comp); // cqj9: o registro do campeão da Série A
            *((l + 0x37) as *mut u8) = 5;
        }
        return (0, l, 5);
    }
    if st == 2 || st == 0xff {
        let l = cqj9_linha(this, clube);
        if fazer {
            *((l + 0x37) as *mut u8) = 0xfe; // eliminado
        }
        return (0, l, 0xfe);
    }
    (0, 0, 0)
}

/// A Série C (0x42eb00); como a da Série A.
pub(crate) unsafe fn cqj9_serie_c(this: usize, clube: usize, situacao: u32, fase: u32, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = read_u32_as_usize(this + 4);
    if fa < 7 {
        // os grupos
        if st == 6 && fazer {
            cqj9_classificou(clube, comp, 0x14, 0, 0x28);
        }
        return (0, 0, 0);
    }
    if fa == 7 {
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_classificou(clube, comp, 0, 0x1e, 0x28);
        }
        return (0, 0, 0);
    }
    if fa < 10 {
        // o quadrangular final: os dois que sobem
        if st != 0 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_promovido(clube, comp, 0x32);
            cqj9_classificou(clube, comp, 0, 0x96, 0x28);
        }
        // a linha do clube na liga (k = -1) ou nos estágios 0..6, a primeira que tiver o clube, ganha o destino 0
        let mut k: i8 = -1;
        let mut achada = 0;
        while k < 7 {
            let liga = if k == -1 { this } else { read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4 * k as i32 as usize) };
            let n = read_i16(liga + 0x3e);
            if n > 0 {
                let t = read_u32_as_usize(liga + 0xb1);
                let mut i: i8 = 0;
                while (i as i16) < n {
                    let r = t.wrapping_add(0x3b * i as i32 as usize);
                    if read_u32_as_usize(r) == clube {
                        achada = r;
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                if achada != 0 {
                    break;
                }
            }
            k += 1;
        }
        if achada != 0 && fazer {
            crate::comparacao::ramo("cqj_9 serie c promovido");
            *((achada + 0x37) as *mut u8) = 0; // cqj9: o destino de quem sobe da Série C
        }
        return (0, achada, 0);
    }
    if fa == 10 {
        // a final
        if st != 1 {
            return (u32::MAX, 0, 0);
        }
        if fazer {
            cqj9_campeao(clube, comp);
        }
    }
    (0, 0, 0)
}

/// O Baiano (0x430210); como a da Série A. O clube vem da tabela (+0xb1, +0x3e linhas), não da 0x68ab70.
pub(crate) unsafe fn cqj9_baiano(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> (u32, usize, u8) {
    let (st, fa) = (situacao as u8, fase as u8);
    let comp = read_u32_as_usize(this + 4);
    if fa == 0xff {
        if st == 6 {
            if fazer {
                cqj9_classificou(clube, comp, 0, 0x82, 0x32);
            }
        } else if st == 3 && fazer {
            crate::comparacao::ramo("cqj_9 baiano rebaixado");
            cqj9_rebaixado(clube, comp); // cqj9: o rebaixado do Baiano
        }
        return (0, 0, 0);
    }
    if fa != 0 {
        return (0, 0, 0);
    }
    let n = read_i16(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < n {
        if read_u32_as_usize(read_u32_as_usize(this + 0xb1) + 0x3b * i as usize) == clube {
            if st == 1 {
                if fazer {
                    cqj9_campeao(clube, comp);
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 5;
                }
                return (0, l, 5);
            }
            if st == 2 || st == 0xff {
                if fazer {
                    // ponytail: a original empurra os registradores inteiros (eax com o lixo do chamador acima da
                    // palavra, ecx com a metade de cima do ponteiro da tabela); a 0x4bf850 e a 0x86c000 só leem as
                    // palavras (0x4b0870 / `as u16`): vão as palavras.
                    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_SCALE_BY_ROUND);
                    let peso = escala(0x1e, 0x32, read_i16(jogo + 0x34) as u16 as u32, 2);
                    let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
                    elim(read_u32_as_usize(G_STAFF_RECORDS), clube, comp, read_i16(jogo + 0x32) as u16 as u32, read_i16(jogo + 0x30) as u16 as u32, peso);
                }
                let l = read_u32_as_usize(this + 0xb1) + 0x3b * i as usize;
                if fazer {
                    *((l + 0x37) as *mut u8) = 0xfe;
                }
                return (0, l, 0xfe);
            }
        }
        i += 1;
    }
    (0, 0, 0)
}

/// Series A: at the end of the round robin, the clubs through to the knockout get the qualification record (stage
/// 0x41c, weight 0x28) and the relegated the relegation record; in the knockout, the champion gets the champion record
/// and fate 5 in the table, the eliminated fate -2
/// Original: `BraNatFirst::update_staff_records_for_status` (0x42abc0, bra_nat_first.cpp, thiscall).
///
/// [pt] { 0x42abc0 } (thiscall, ret 0x18, bra_nat_first.cpp): `BraNatFirst::update_staff_records_for_status` (slot 18):
/// na fase de pontos corridos, os 8 classificados (situação 6) ganham o registro de classificação e os rebaixados (3) o
/// de rebaixamento; no mata-mata (fase 0), o campeão (1) ganha o registro de campeão e o destino 5, os eliminados (2 ou
/// -1) o destino -2. Devolve 0.
pub(crate) unsafe extern "thiscall" fn bra_nat_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    cqj9_serie_a(this, clube, situacao, fase, true).0
}

/// Comparador: só a original (ponytail: os registros de carreira alocam e vão para o histórico; não rodam duas vezes),
/// com o plano do ensaio (a cópia sem os registros nem a gravação): a devolução e o destino gravado na linha.
pub(crate) unsafe extern "thiscall" fn comparar_bra_nat_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let (n, l, dest) = cqj9_serie_a(this, clube, situacao, fase, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_BRA_NAT_FIRST_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, situacao, fase, a4, a5, a6);
    let gravado = if l == 0 { dest } else { *((l + 0x37) as *const u8) };
    PLACAR.registrar(r == n && gravado == dest, || format!("bra_nat_first_update_staff_records_for_status({:x}, {:x}, {}, {}) original={} rust={} destino {} x {}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32, gravado as i8, dest as i8));
    r
}





