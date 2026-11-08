use super::*;



/// Club in the first table row of stage k of the league (k = -1: the league itself).
///
/// [pt] O clube da primeira linha da tabela (+0xb1) da fase `k` da liga (k = −1: a própria liga).
pub(crate) unsafe fn stage_leader(this: usize, k: i32) -> usize {
    let t = if k == -1 { this } else { read_usize(read_usize(this + 0xc) + 4 * k as usize) };
    read_usize(read_usize(t + 0xb1))
}

/// The Series C final: the winner of third-round group A against the winner of group B, in a two-club cup stage
/// [inference: home and away]; the winner is the Series C champion
/// Original: `BraNatThird::create_final_stage` (0x42e980, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42e980 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_final_stage()`, a
/// final da Série C, do fim da terceira fase: os campeões dos dois grupos da terceira fase (o primeiro da tabela da
/// fase 8 e o da fase 9, nessa ordem) numa lista local de 2; a tabela das rodadas pela virtual +0x3c
/// (create_round_table: thiscall(this, 10, &rodadas, &extra, 0), ret 0x10); a fase (operator new 0x944cf1(0xb2) e o
/// construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 2, a lista, as rodadas, +4, a tabela, o ano, 10, 1, o
/// extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][10]. Sem fase: a caixa da linha 0x392 e volta (sem
/// liberar a tabela). Com ela: a tabela liberada (free 0x9452ca) e +0x30 = 10. [inferência] A final decide o campeão
/// da Série C, em ida e volta.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_final_stage(this: usize) {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let mut list = [0usize; 2];
    list[0] = stage_leader(this, 8); // cqs4: o campeão do grupo A
    list[1] = stage_leader(this, 9);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 10, &mut rodadas, &mut extra, 0);
    let p = new(0xb2);
    let stage = if p != 0 {
        construct(p, this, 2, list.as_ptr(), rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 10, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0x28) as *mut usize, stage);
    if read_usize(read_usize(this + 0xc) + 0x28) == 0 {
        return show_error_at(0x98aba4, 0x392);
    }
    crate::comparacao::ramo("cqs_4 final");
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 10);
}

/// The Series C turnover: archives the season, clears the table, the schedule and the stages, moves to the next year
/// and rebuilds the rules, the 56-club list and the table, the fixtures and the order; then the board expectations, the
/// referee renewal, the seven first-round groups and the predicted positions; the 56-club list is discarded at the end
/// Original: `BraNatThird::start_new_season` (0x42def0, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42def0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::start_new_season()`, a virada da Série C (chamada
/// pela virada da Série A): +0x4c = 0 e a tabela liberada (League::free_table 0x687970, thiscall(this, 0), ret 4); o
/// calendário (+0xba) liberado (free 0x9452ca) e zerado; com estatísticas (+8), Comp_Stats::archive_season_and_reset
/// (0x4a1c50, thiscall(+8, 1), ret 4); com a fase atual ≥ 0, as fases 0..=+0x30 que existem destruídas (a virtual 0,
/// o destrutor com 1); +0x30 = −1; o vetor das fases liberado; o ano (+0x40, word) sobe um; um vetor novo de +0x2c
/// fases (malloc de 4 × +0x2c, sem zerar; com +0x2c ≤ 0, nulo); então, cada um com a sua caixa (linhas 0x202, 0x209,
/// 0x20f e 0x216) se devolver 0: as regras (virtual +0x8c), a tabela e a lista dos 56 (0x42dc90, thiscall, ret 0), os
/// jogos (League::create_fixtures 0x6835c0) e a ordem da tabela (League::sort_table 0x6827d0, thiscall(this, 0), ret
/// 4); por fim as expectativas das diretorias (League::set_club_season_expectations 0x68aa80, thiscall, ret 0), a
/// renovação dos árbitros (OfficialsManager::renew_officials 0x79cee0, thiscall([0xb74340], +4), ret 4), os sete
/// grupos da primeira fase (0x42ee80(g), thiscall, ret 4, g de 0 a 6), as posições previstas (virtual +0x5c) e a lista
/// dos 56 (+0xee) liberada (se houver) e zerada. Com caixa, a cópia devolve 0 (a original devolve o eax da caixa, que
/// a virada do Brasil não lê); no fim, o eax do free ou o +0xee (a cópia devolve 0: ninguém lê).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_start_new_season(this: usize) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    core::ptr::write_unaligned((this + 0x4c) as *mut u32, 0);
    liberar_tabela(this, 0);
    let l = read_usize(this + 0xba);
    if l != 0 {
        free(l);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, 0);
    }
    let stats = read_usize(this + 8);
    if stats != 0 {
        let archive: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_STATS_ARCHIVE_SEASON_AND_RESET);
        archive(stats, 1);
    }
    if (read_usize(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(read_usize(this + 0xc) + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > read_usize(this + 0x30) as i32 {
                break;
            }
        }
    }
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    free(read_usize(this + 0xc));
    let n = read_usize(this + 0x2c) as i32;
    let a = read_i16(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1));
    crate::comparacao::ramo("cqs_4 virada da serie c");
    let v = if n > 0 { alloc((n * 4) as u32) } else { 0 };
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if rules(this) == 0 {
        show_error_at(0x98aba4, 0x202);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRANATTHIRD_CREATE_TABLE);
    if table(this) == 0 {
        show_error_at(0x98aba4, 0x209);
        return 0;
    }
    let games: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if games(this) == 0 {
        show_error_at(0x98aba4, 0x20f);
        return 0;
    }
    let sort: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if sort(this, 0) == 0 {
        show_error_at(0x98aba4, 0x216);
        return 0;
    }
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(G_OFFICIALS_MANAGER), read_usize(this + 4)); // cqs4: a renovação dos árbitros da Série C
    let group: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_BRANATTHIRD_CREATE_GROUP_STAGE);
    for g in 0..7 {
        group(this, g);
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    predicted(this);
    let l = read_usize(this + 0xee);
    if l != 0 {
        free(l);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
    0
}

/// As posições previstas da Série C, na ordem da lista dos 56 depois de ordenada: 1, 2, 3, 3, 5, 5, 7, 7, oito vezes 9
/// (as faixas do mata-mata) e, para os outros quarenta, a faixa de oito em oito: 16, 24, 32, 40 e 48.
pub(crate) const SERIE_C_PREDICTED_PLACES: [u8; 16] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9]; // cqs4: as posições previstas do mata-mata

/// Place for index k: from SERIE_C_PREDICTED_PLACES below 16, else 16 + 8 x ((k - 16) / 8) [inference].
pub(crate) fn place_for_index(k: usize) -> u8 {
    if k < 16 { SERIE_C_PREDICTED_PLACES[k] } else { (16 + 8 * ((k - 16) / 8)) as u8 }
}

/// The Series C predicted positions: the 56 ordered by reputation; the favourite 1st, the second 2nd, the next two 3rd
/// (the semi-finals), two 5th and two 7th, eight 9th (the second round) and the other forty in bands of eight: 16th,
/// 24th, 32nd, 40th and 48th
/// Original: `BraNatThird::set_predicted_positions` (0x42f0a0, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42f0a0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::set_predicted_positions()`, as posições previstas da
/// Série C (do construtor e da virada, depois dos grupos). Sem estatísticas (+8): a caixa da linha 0x4a2 e volta. A
/// lista dos 56 (+0xee) é ordenada no lugar pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no
/// fim) e cada clube, na ordem, ganha a posição prevista de `place_for_index` (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a original empurra a faixa dos quarenta num dword com o byte da faixa e lixo
/// da pilha em cima; a 0x4a2540 só lê o byte: a cópia passa o byte limpo).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_predicted_positions(this: usize) {
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let position: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    if read_usize(this + 8) == 0 {
        return show_error_at(0x98aba4, 0x4a2);
    }
    sort(read_usize(this + 0xee), 0x38, 4, 0x4b07d0); // cqs4: a lista dos 56 pela reputação
    for k in 0..0x38usize {
        position(read_usize(this + 8), read_usize(read_usize(this + 0xee) + 4 * k), place_for_index(k) as u32);
    }
    crate::comparacao::ramo("cqs_4 posicoes previstas");
}







#[allow(clippy::type_complexity)]
pub(crate) type LeagueStageCtorFn = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize;

/// 0x42ee80 BraNatThird::create_group_stage(g): round table, the 7 clubs of group g, League_Stage in slot g, +0x30 = g.
///
/// [pt] { 0x42ee80 } (thiscall, ret 4, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_group_stage(g)`, o
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
pub(crate) unsafe fn bra_nat_third_create_group_stage_cqs3_grupo(this: usize, g: u32, pv: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let construct: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, g, &mut rodadas, &mut extra, 0);
    let v = alloc(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x98aba4, 0x475);
    }
    let k = g as u8 as i8 as i32;
    for j in 0..7usize {
        core::ptr::write_unaligned((v + 4 * j) as *mut usize, read_usize(read_usize(this + 0xee) + 4 * (7 * (k + 1) as usize + j)));
    }
    let p = new(0xee);
    let stage = if p != 0 {
        construct(p, this, 7, v, 2, read_usize(this + 4), t, rodadas as u16 as u32, read_u8(this + pv) as u32, read_u8(this + 0xc3) as u32,
                  read_u8(this + 0xc4) as u32, this + 0xc5, this + 0xbe, read_i16(this + 0x40) as u16 as u32, g, extra as u16 as u32,
                  read_u8(this + 0x51) as u32, 1, 0, read_i16(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, stage);
    if read_usize(read_usize(this + 0xc) + 4 * k as usize) == 0 {
        return show_error_at(0x98aba4, 0x487);
    }
    crate::comparacao::ramo("cqs_3 grupo");
    soltar(v);
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
}

/// A Series C first-round group: seven clubs of the 56-club list (the first seven slots are the league's own group;
/// group g takes slots 7(g+1) to 7(g+1)+6), home and away round robin, with the league points, tie-breakers and places;
/// the top two of each of the eight groups go to the second round
/// Original: `BraNatThird::create_group_stage` (0x42ee80, bra_nat_third.cpp, thiscall, ret 4).
///
/// [pt] { 0x42ee80 } (acima): o grupo g da primeira fase da Série C.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_group_stage(this: usize, g: u32) {
    bra_nat_third_create_group_stage_cqs3_grupo(this, g, 0xc2) // cqs3: os pontos por vitória nos grupos
}

/// A ordem da segunda fase: o k-ésimo melhor (pelos pontos) vai à vaga `SECOND_ROUND_SEEDS[k]` da lista dos dezesseis.
pub(crate) const SECOND_ROUND_SEEDS: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: as vagas da segunda fase

/// The Series C second round: the sixteen (the top two of each group) seeded by first-round points (ties go to the
/// earlier one: the league, then groups 0 to 6, first before second), in a knockout where the best meets the 16th, the
/// 2nd the 15th and so on [inference: pairs follow the list]
/// Original: `BraNatThird::create_second_round_stage` (0x42e430, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42e430 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_second_round_stage()`,
/// a segunda fase da Série C, do fim dos grupos.
/// 1) Uma lista de 16 (malloc 0x944e46 de 0x40; sem memória: a caixa da linha 0x2e6 e volta).
/// 2) Os dois primeiros de cada um dos oito grupos (a tabela da liga e as das fases 0 a 6, nessa ordem), em registros de
///    6 bytes (o clube e os pontos, +0x19 da linha).
/// 3) Dezesseis vezes: o registro de mais pontos (o primeiro, no empate) entre os que ainda têm clube sai (o clube vira
///    0) e o clube vai à vaga `SECOND_ROUND_SEEDS[k]`: o melhor contra o 16º, o 2º contra o 15º... (sem nenhum: a caixa da
///    linha 0x30b; a original lê então o endereço 0 e cai).
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 7, &rodadas, &extra, 0), ret 0x10); a fase (operator new
///    0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 16, a lista, as rodadas, +4, a
///    tabela, o ano, 7, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][7]. Sem fase: a caixa da
///    linha 0x31b e volta. Com ela: a tabela e a lista liberadas (free 0x9452ca) e +0x30 = 7.
pub(crate) unsafe extern "thiscall" fn create_second_round_stage(this: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let list = alloc(0x40);
    if list == 0 {
        return show_error_at(0x98aba4, 0x2e6);
    }
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = read_usize(if g == -1 { this } else { read_usize(read_usize(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (read_usize(t), read_i16(t + 0x19));
        r[2 * k + 1] = (read_usize(t + 0x3b), read_i16(t + 0x54));
    }
    for k in 0..16 {
        let mut best: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if best.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                best = Some(i);
            }
        }
        let c = match best {
            Some(m) => {
                let c = r[m].0;
                r[m].0 = 0;
                c
            }
            None => {
                show_error_at(0x98aba4, 0x30b);
                core::ptr::read_volatile(core::ptr::null::<usize>()) // simplification: a original lê o endereço 0 aqui e cai
            }
        };
        core::ptr::write_unaligned((list + 4 * SECOND_ROUND_SEEDS[k] as usize) as *mut usize, c);
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this, 7, &mut rodadas, &mut extra, 0);
    let p = new(0xb2);
    let stage = if p != 0 {
        construct(p, this, 0x10, list, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0x1c) as *mut usize, stage);
    if read_usize(read_usize(this + 0xc) + 0x1c) == 0 {
        return show_error_at(0x98aba4, 0x31b);
    }
    crate::comparacao::ramo("cqs_3 segunda fase");
    soltar(t);
    soltar(list);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 7);
}

/// The Series C third round: the eight second-round winners, drawn (shuffled) into two groups of four, home and away,
/// with 3 points for a win, 1 for a draw, tie-breakers {1, 2, 4} and two top places in each group [inference: the top
/// two of each group go up to Series B and the group winners play the final]
/// Original: `BraNatThird::create_third_round_groups` (0x42e700, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42e700 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_third_round_groups()`,
/// a terceira fase da Série C, do fim da segunda: os vencedores da segunda fase ([+0xc][7], os registros de 6 bytes de
/// +0xa7 com +5 = 1, na ordem) numa lista local de 8, embaralhada (comp_util_shuffle_pointer_array 0x4afb40, cdecl(v,
/// 8)); os quatro primeiros formam o grupo A (fase 8) e os outros quatro o grupo B (fase 9). Cada grupo: a tabela das
/// rodadas pela virtual +0x3c (thiscall(this, 8 ou 9, &rodadas, &extra, 0)) e a fase (operator new 0x944cf1(0xee) e o
/// construtor League_Stage 0x68b100: a liga, 4, a lista (ou a lista + 4), 2 (ida e volta), +4, a tabela, as rodadas
/// (word), 3 pontos por vitória, 1 por empate, a categoria 5, &{1,2,4,0} (os critérios de desempate), &{2,0,0,0} (duas
/// vagas de cima), o ano, 8 ou 9, o extra (word), 0x14, 1, 0, 2 (os bits de regra), −1, 0, 2) em [+0xc][8] ou [9];
/// sem fase: a caixa da linha 0x353 ou 0x363 e volta; com ela, a tabela liberada (free 0x9452ca). Depois do segundo,
/// +0x30 = 9. [inferência] Os dois primeiros de cada grupo sobem à Série B; os campeões dos grupos fazem a final.
pub(crate) unsafe extern "thiscall" fn create_third_round_groups(this: usize) {
    let embaralhar: unsafe extern "C" fn(*mut usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [2, 0, 0, 0];
    let desempate: [u8; 4] = [1, 2, 4, 0];
    let mut v = [0usize; 8];
    let second = read_usize(read_usize(this + 0xc) + 0x1c);
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(second + 0x3e) as i32 {
        let e = read_usize(second + 0xa7) + 6 * i as usize;
        // simplification: a original grava sem limite numa lista de 8 na pilha; a fase tem 8 vencedores (a cópia para no 8º).
        if read_u8(e + 5) == 1 && n < 8 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    embaralhar(v.as_mut_ptr(), 8); // cqs3: os oito vencedores embaralhados
    for (k, f) in [(8u32, 0x353u32), (9, 0x363)] {
        let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let (mut rodadas, mut extra) = (0i16, 0i16);
        let t = table(this, k, &mut rodadas, &mut extra, 0);
        let p = new(0xee);
        let stage = if p != 0 {
            construct(p, this, 4, v[4 * (k as usize - 8)..].as_ptr(), 2, read_usize(this + 4), t, rodadas as u16 as u32, 3, 1, 5,
                      desempate.as_ptr(), slots.as_ptr(), read_i16(this + 0x40) as u16 as u32, k, extra as u16 as u32, 0x14, 1, 0, 2, -1, 0, 2)
        } else {
            0
        };
        core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, stage);
        if read_usize(read_usize(this + 0xc) + 4 * k as usize) == 0 {
            return show_error_at(0x98aba4, f);
        }
        soltar(t);
    }
    crate::comparacao::ramo("cqs_3 terceira fase");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 9);
}




/// Series C takes up to 56 clubs (those of the division, in database order), which are drawn (3 swaps per club); the
/// first 7 of the draw make the first table, of 7 clubs, and the drawn list is kept for the other groups [inference: 8
/// groups of 7 in the first stage]
/// Original: `BraNatThird::create_table` (0x42dc90, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42dc90 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::create_table()`: +0x3e = 7; a lista dos
/// participantes (+0xee) refeita (a velha liberada, 0x9452ca push; add esp -> cdecl; a nova com 0xe0 bytes = 56
/// ponteiros; sem memória: a caixa da linha 0x18f e devolve 1, como a original); a tabela (+0xb1, 7 × 0x3b; sem
/// memória: a caixa da linha 0x196 e 0); a lista recebe, na ordem do banco, até 56 clubes cuja divisão (+0x57) é a
/// própria Série C (o registro em [this+4]); a lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push
/// conta (word); push lista; add esp, 8 -> cdecl; 3 × conta trocas sorteadas) e os 7 primeiros do sorteio entram nas
/// linhas 0..6 (0x687430, a linha num dword limpo). Fonte 0x98aba4. Devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_table(this: usize) -> u32 {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let embaralhar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 7);
    let velha = read_usize(this + 0xee);
    if velha != 0 {
        soltar(velha);
    }
    let list = alloc(0xe0);
    core::ptr::write_unaligned((this + 0xee) as *mut usize, list);
    if list == 0 {
        show_error_at(0x98aba4, 0x18f);
        return 1;
    }
    let t = alloc((read_i16(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(0x98aba4, 0x196);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if (k as i8) >= 0x38 {
            break;
        }
        if read_usize(clube + club::DIVISION) == read_usize(this + 4) {
            core::ptr::write_unaligned((read_usize(this + 0xee) + (k as i8 as isize as usize) * 4) as *mut usize, clube);
            k = k.wrapping_add(1);
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    crate::comparacao::ramo("cql_7 serie c sorteio");
    embaralhar(read_usize(this + 0xee), k as i8 as i32); // cql7: o sorteio dos clubes da Série C
    for j in 0..7usize {
        iniciar(this, j as u32, read_usize(read_usize(this + 0xee) + j * 4), 0, 0);
    }
    1
}


/// Series C: double round robin (home-and-away groups [inference: the groups of 7 of the 0x42d620 schedule]); win 3,
/// draw 1; tie-break by goal difference, goals scored and away goals; promotes to Series B, no league below; 2 top
/// places leading to the next stage (rule word 8) and 2 bottom ones [inference: no effect, there is no league below]; 7
/// substitutes, 3 substitutions, fixed numbers. Then the schedule
/// Original: `BraNatThird::set_up_rules` (0x42de50, bra_nat_third.cpp, thiscall).
///
/// [pt] { 0x42de50 } (thiscall, bra_nat_third.cpp): `BraNatThird::set_up_rules`, as regras da Série C e o calendário.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_up_rules(this: usize) -> u32 {
    set_serie_c_rules(this);
    crate::comparacao::ramo("cqi_7 regras da serie c");
    create_main_schedule(this);
    1
}


/// Series C: in the groups, the qualified get the qualification record; in stage 7, those who go through; in the final
/// group (stages 8 and 9), the two promoted get the promotion (weight 0x32) and qualification records and fate 0
/// (promotion) in the first table that has them; in the final (stage 10), the champion gets the champion record
/// Original: `BraNatThird::update_staff_records_for_status` (0x42eb00, bra_nat_third.cpp, thiscall).
///
/// [pt] { 0x42eb00 } (thiscall, ret 0x18, bra_nat_third.cpp): `BraNatThird::update_staff_records_for_status`: nos grupos
/// (fase < 7), os classificados (6) ganham o registro de classificação; na fase 7, quem passa (1) também (os outros: -1);
/// no quadrangular (fases 8 e 9), os que sobem (0) ganham os registros de acesso e de classificação e o destino 0 na
/// primeira tabela que os tiver (a liga ou os estágios 0..6); na final (10), o campeão (1) ganha o registro de campeão.
/// Devolve 0, ou -1 nas situações que a fase não espera.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_update_staff_records_for_status(this: usize, clube: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    serie_c_staff_records(this, clube, status, stage, true).0
}


/// Series C: in the groups (stages up to 6), the group position becomes the position on the 49-place Series C scale
/// [inference]: 1 and 2 → 1 (the highest → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; in later stages, the codes as
/// they come
/// Original: `BraNatThird::set_club_progress` (0x42e100, bra_nat_third.cpp, thiscall).
///
/// [pt] { 0x42e100 } (thiscall, ret 0x14, bra_nat_third.cpp): `BraNatThird::set_club_progress(clube, fase, a, b, c)`: nos
/// grupos (fase < 7), a e b: 1, 2 → 1, 3 → 0x11 ... 7 → 0x31; c: 1, 2 → 9, depois igual; nas fases 7 em diante, os
/// códigos como vêm (o trecho das fases 1 a 3 que a original tem depois do teste `fase >= 7` nunca roda). Devolve a
/// entrada.
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_club_progress(this: usize, clube: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, clube);
    let (fa, a, b, c) = (stage as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 7 {
        let (a, c) = (serie_c_progress_code(a, 1), serie_c_progress_code(c, 9));
        let b2 = serie_c_progress_code(b, 1);
        if b == 3 {
            crate::comparacao::ramo("cqk_8 serie c segundo 3");
        }
        write_progress_codes(e, a, b2, c); // cqk8: os códigos dos grupos da Série C
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

