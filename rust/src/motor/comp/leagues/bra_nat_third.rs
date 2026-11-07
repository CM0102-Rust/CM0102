// ---------- 283a leva: a Série C: a final, a virada da temporada e as posições previstas (bra_nat_third.cpp) ----------



/// O clube da primeira linha da tabela (+0xb1) da fase `k` da liga (k = −1: a própria liga).
unsafe fn cqs4_primeiro(this: usize, k: i32) -> usize {
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
unsafe extern "thiscall" fn cqs4_final_da_serie_c(this: usize) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let mut lista = [0usize; 2];
    lista[0] = cqs4_primeiro(this, 8); // cqs4: o campeão do grupo A
    lista[1] = cqs4_primeiro(this, 9);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 10, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 2, lista.as_ptr(), rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 10, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0x28) as *mut usize, fase);
    if read_usize(read_usize(this + 0xc) + 0x28) == 0 {
        return aviso_em(0x98aba4, 0x392);
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
unsafe extern "thiscall" fn cqs4_virada_da_serie_c(this: usize) -> u32 {
    let liberar_tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_FREE_TABLE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
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
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    liberar(read_usize(this + 0xc));
    let n = read_usize(this + 0x2c) as i32;
    let a = read_i16(this + 0x40);
    core::ptr::write_unaligned((this + 0x40) as *mut i16, a.wrapping_add(1));
    crate::comparacao::ramo("cqs_4 virada da serie c");
    let v = if n > 0 { alocar((n * 4) as u32) } else { 0 };
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x8c));
    if regras(this) == 0 {
        aviso_em(0x98aba4, 0x202);
        return 0;
    }
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRANATTHIRD_CREATE_TABLE);
    if tabela(this) == 0 {
        aviso_em(0x98aba4, 0x209);
        return 0;
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    if jogos(this) == 0 {
        aviso_em(0x98aba4, 0x20f);
        return 0;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    if ordenar(this, 0) == 0 {
        aviso_em(0x98aba4, 0x216);
        return 0;
    }
    let expectativas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_SET_CLUB_SEASON_EXPECTATIONS);
    expectativas(this);
    let renovar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_OFFICIALSMANAGER_RENEW_OFFICIALS);
    renovar(read_usize(0xb74340), read_usize(this + 4)); // cqs4: a renovação dos árbitros da Série C
    let grupo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_BRANATTHIRD_CREATE_GROUP_STAGE);
    for g in 0..7 {
        grupo(this, g);
    }
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
    previstas(this);
    let l = read_usize(this + 0xee);
    if l != 0 {
        liberar(l);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
    0
}

/// As posições previstas da Série C, na ordem da lista dos 56 depois de ordenada: 1, 2, 3, 3, 5, 5, 7, 7, oito vezes 9
/// (as faixas do mata-mata) e, para os outros quarenta, a faixa de oito em oito: 16, 24, 32, 40 e 48.
const CQS4_LUGARES: [u8; 16] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9]; // cqs4: as posições previstas do mata-mata

fn cqs4_lugar(k: usize) -> u8 {
    if k < 16 { CQS4_LUGARES[k] } else { (16 + 8 * ((k - 16) / 8)) as u8 }
}

/// The Series C predicted positions: the 56 ordered by reputation; the favourite 1st, the second 2nd, the next two 3rd
/// (the semi-finals), two 5th and two 7th, eight 9th (the second round) and the other forty in bands of eight: 16th,
/// 24th, 32nd, 40th and 48th
/// Original: `BraNatThird::set_predicted_positions` (0x42f0a0, bra_nat_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x42f0a0 } (thiscall, ret 0, bra_nat_third.cpp): `BraNatThird::set_predicted_positions()`, as posições previstas da
/// Série C (do construtor e da virada, depois dos grupos). Sem estatísticas (+8): a caixa da linha 0x4a2 e volta. A
/// lista dos 56 (+0xee) é ordenada no lugar pela reputação (qsort 0x945333 com 0x4b07d0, a maior primeiro, nulos no
/// fim) e cada clube, na ordem, ganha a posição prevista de `cqs4_lugar` (Comp_Stats::set_club_final_position 0x4a2540,
/// thiscall(+8, clube, posição), ret 8; a original empurra a faixa dos quarenta num dword com o byte da faixa e lixo
/// da pilha em cima; a 0x4a2540 só lê o byte: a cópia passa o byte limpo).
unsafe extern "thiscall" fn cqs4_posicoes_previstas_da_serie_c(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    if read_usize(this + 8) == 0 {
        return aviso_em(0x98aba4, 0x4a2);
    }
    ordenar(read_usize(this + 0xee), 0x38, 4, 0x4b07d0); // cqs4: a lista dos 56 pela reputação
    for k in 0..0x38usize {
        posicao(read_usize(this + 8), read_usize(read_usize(this + 0xee) + 4 * k), cqs4_lugar(k) as u32);
    }
    crate::comparacao::ramo("cqs_4 posicoes previstas");
}

/// O comparador da final: só a original (aloca a tabela e a fase e sorteia a rodada), com o plano feito antes: os
/// campeões dos grupos da terceira fase. Depois, a fase da vaga 10 existe, com +0x3e = 2, os dois na lista dela (como
/// conjunto) e +0x30 = 10.
unsafe extern "thiscall" fn comparar_cqs4_final_da_serie_c(this: usize) {
    let (a, b) = (cqs4_primeiro(this, 8), cqs4_primeiro(this, 9));
    orig::<unsafe extern "thiscall" fn(usize)>(2027)(this);
    let fase = read_usize(read_usize(this + 0xc) + 0x28);
    let igual = fase != 0 && {
        let l = read_usize(fase + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(fase + 0x3e) == 2 && read_usize(this + 0x30) == 10 && ((x, y) == (a, b) || (x, y) == (b, a))
    };
    PLACAR.registrar(igual, || format!("cqs4_final_da_serie_c({:#x}) campeoes={:#x},{:#x} fase={:#x}", this, a, b, fase));
}

/// O comparador da virada: só a original (ponytail: libera tabelas e fases, aloca as novas, sorteia jogos, árbitros e
/// grupos; não pode rodar duas vezes), conferindo o que só ela garante: o ano + 1, os sete grupos nas vagas 0 a 6 e a
/// lista dos 56 zerada.
unsafe extern "thiscall" fn comparar_cqs4_virada_da_serie_c(this: usize) -> u32 {
    let antes = read_i16(this + 0x40);
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2028)(this);
    let depois = read_i16(this + 0x40);
    let igual = depois == antes.wrapping_add(1) && read_usize(this + 0xee) == 0 && (0..7).all(|g| read_usize(read_usize(this + 0xc) + 4 * g) != 0);
    PLACAR.registrar(igual, || format!("cqs4_virada_da_serie_c({:#x}) ano antes={} depois={} +0xee={:#x}", this, antes, depois, read_usize(this + 0xee)));
    r
}

/// O comparador das posições previstas: só a original (a 0x4a2540 acrescenta o registro de um clube que não tem; não
/// pode rodar duas vezes), com o plano feito antes: a lista dos 56 ordenada pela mesma qsort numa cópia; depois, a
/// lista da liga na mesma ordem e a posição (+0x72 do registro achado por 0x4a4850) de cada clube.
unsafe extern "thiscall" fn comparar_cqs4_posicoes_previstas_da_serie_c(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2029);
    if read_usize(this + 8) == 0 || read_usize(this + 0xee) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut v = [0usize; 0x38];
    for (k, c) in v.iter_mut().enumerate() {
        *c = read_usize(read_usize(this + 0xee) + 4 * k);
    }
    ordenar(v.as_mut_ptr(), 0x38, 4, 0x4b07d0);
    o(this);
    let errado = (0..0x38).find(|&k| {
        let e = if v[k] != 0 { achar(read_usize(this + 8), v[k]) } else { 0 };
        read_usize(read_usize(this + 0xee) + 4 * k) != v[k] || e == 0 || *((e + 0x72) as *const u8) != cqs4_lugar(k)
    });
    PLACAR.registrar(errado.is_none(), || format!("cqs4_posicoes_previstas_da_serie_c({:#x}): a vaga {:?} não bate", this, errado));
}

// ---------- 282a leva: a Série C: os sete grupos da primeira fase, a segunda fase dos dezesseis e os dois grupos da terceira (bra_nat_third.cpp) ----------




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
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let construir: Cqs3Fase = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, g, &mut rodadas, &mut extra, 0);
    let v = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return aviso_em(0x98aba4, 0x475);
    }
    let k = g as u8 as i8 as i32;
    for j in 0..7usize {
        core::ptr::write_unaligned((v + 4 * j) as *mut usize, read_usize(read_usize(this + 0xee) + 4 * (7 * (k + 1) as usize + j)));
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, 7, v, 2, read_usize(this + 4), t, rodadas as u16 as u32, read_u8(this + pv) as u32, read_u8(this + 0xc3) as u32,
                  read_u8(this + 0xc4) as u32, this + 0xc5, this + 0xbe, read_i16(this + 0x40) as u16 as u32, g, extra as u16 as u32,
                  read_u8(this + 0x51) as u32, 1, 0, read_i16(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    if read_usize(read_usize(this + 0xc) + 4 * k as usize) == 0 {
        return aviso_em(0x98aba4, 0x487);
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
unsafe extern "thiscall" fn cqs3_grupo_da_serie_c(this: usize, g: u32) {
    cqs3_grupo(this, g, 0xc2) // cqs3: os pontos por vitória nos grupos
}

/// A ordem da segunda fase: o k-ésimo melhor (pelos pontos) vai à vaga `CQS3_SEMENTES[k]` da lista dos dezesseis.
const CQS3_SEMENTES: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: as vagas da segunda fase

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
///    0) e o clube vai à vaga `CQS3_SEMENTES[k]`: o melhor contra o 16º, o 2º contra o 15º... (sem nenhum: a caixa da
///    linha 0x30b; a original lê então o endereço 0 e cai).
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 7, &rodadas, &extra, 0), ret 0x10); a fase (operator new
///    0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 16, a lista, as rodadas, +4, a
///    tabela, o ano, 7, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][7]. Sem fase: a caixa da
///    linha 0x31b e volta. Com ela: a tabela e a lista liberadas (free 0x9452ca) e +0x30 = 7.
unsafe extern "thiscall" fn cqs3_segunda_fase_da_serie_c(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let lista = alocar(0x40);
    if lista == 0 {
        return aviso_em(0x98aba4, 0x2e6);
    }
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = read_usize(if g == -1 { this } else { read_usize(read_usize(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (read_usize(t), read_i16(t + 0x19));
        r[2 * k + 1] = (read_usize(t + 0x3b), read_i16(t + 0x54));
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
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 7, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 0x10, lista, rodadas as u16 as u32, read_usize(this + 4), t, read_i16(this + 0x40) as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((read_usize(this + 0xc) + 0x1c) as *mut usize, fase);
    if read_usize(read_usize(this + 0xc) + 0x1c) == 0 {
        return aviso_em(0x98aba4, 0x31b);
    }
    crate::comparacao::ramo("cqs_3 segunda fase");
    soltar(t);
    soltar(lista);
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
unsafe extern "thiscall" fn cqs3_terceira_fase_da_serie_c(this: usize) {
    let embaralhar: unsafe extern "C" fn(*mut usize, u32) = core::mem::transmute(FN_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let vagas: [u8; 4] = [2, 0, 0, 0];
    let desempate: [u8; 4] = [1, 2, 4, 0];
    let mut v = [0usize; 8];
    let segunda = read_usize(read_usize(this + 0xc) + 0x1c);
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(segunda + 0x3e) as i32 {
        let e = read_usize(segunda + 0xa7) + 6 * i as usize;
        // ponytail: a original grava sem limite numa lista de 8 na pilha; a fase tem 8 vencedores (a cópia para no 8º).
        if read_u8(e + 5) == 1 && n < 8 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    embaralhar(v.as_mut_ptr(), 8); // cqs3: os oito vencedores embaralhados
    for (k, f) in [(8u32, 0x353u32), (9, 0x363)] {
        let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let (mut rodadas, mut extra) = (0i16, 0i16);
        let t = tabela(this, k, &mut rodadas, &mut extra, 0);
        let p = novo(0xee);
        let fase = if p != 0 {
            construir(p, this, 4, v[4 * (k as usize - 8)..].as_ptr(), 2, read_usize(this + 4), t, rodadas as u16 as u32, 3, 1, 5,
                      desempate.as_ptr(), vagas.as_ptr(), read_i16(this + 0x40) as u16 as u32, k, extra as u16 as u32, 0x14, 1, 0, 2, -1, 0, 2)
        } else {
            0
        };
        core::ptr::write_unaligned((read_usize(this + 0xc) + 4 * k as usize) as *mut usize, fase);
        if read_usize(read_usize(this + 0xc) + 4 * k as usize) == 0 {
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
        *c = read_usize(read_usize(this + 0xee) + 4 * (7 * (k + 1) as usize + j));
    }
    orig::<unsafe extern "thiscall" fn(usize, u32)>(2024)(this, g);
    let fase = read_usize(read_usize(this + 0xc) + 4 * k as usize);
    let igual = fase != 0 && read_i16(fase + 0x3e) == 7 && read_usize(this + 0x30) as i32 == k && {
        let mut nela = [0usize; 7];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(fase + 0xb1) + j * 0x3b);
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
        let t = read_usize(if g == -1 { this } else { read_usize(read_usize(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (read_usize(t), read_i16(t + 0x19));
        r[2 * k + 1] = (read_usize(t + 0x3b), read_i16(t + 0x54));
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
    let fase = read_usize(read_usize(this + 0xc) + 0x1c);
    let igual = fase != 0 && read_i16(fase + 0x3e) == 0x10 && read_usize(this + 0x30) == 7 && {
        let mut nela = [0usize; 16];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = read_usize(read_usize(fase + 0xa7) + 6 * j);
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
    let segunda = read_usize(read_usize(this + 0xc) + 0x1c);
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < read_i16(segunda + 0x3e) as i32 && n < 8 {
        let e = read_usize(segunda + 0xa7) + 6 * i as usize;
        if read_u8(e + 5) == 1 {
            plano[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    orig::<unsafe extern "thiscall" fn(usize)>(2026)(this);
    let (a, b) = (read_usize(read_usize(this + 0xc) + 0x20), read_usize(read_usize(this + 0xc) + 0x24));
    let igual = a != 0 && b != 0 && read_i16(a + 0x3e) == 4 && read_i16(b + 0x3e) == 4 && read_usize(this + 0x30) == 9 && {
        let mut nela = [0usize; 8];
        for j in 0..4 {
            nela[j] = read_usize(read_usize(a + 0xb1) + j * 0x3b);
            nela[4 + j] = read_usize(read_usize(b + 0xb1) + j * 0x3b);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    PLACAR.registrar(igual, || format!("cqs3_terceira_fase_da_serie_c({:#x}) n={} plano={:x?}", this, n, plano));
}

