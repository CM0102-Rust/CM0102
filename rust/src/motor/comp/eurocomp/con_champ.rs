use super::*;




/// Competition record id in the database (G_CLUB_COMP_TABLE + 0x6b × id).
///
/// [pt] O registro da competição `id` no banco ([0xae23d0] + 0x6b × id).
pub(crate) unsafe fn club_comp_record(id: usize) -> usize {
    read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b))
}

/// As quatro listas da 0x4c11a0, na ordem do banco de clubes ([0xae23b4], 0x245 bytes, [0xae235c] clubes; para no
/// primeiro que é seleção, 0x53b900): (mexicanos, centro-americanos, caribenhos, da MLS) com as contagens.
pub(crate) struct ConcacafClubLists {
    pub(crate) mex: [usize; 200],
    pub(crate) centro: [usize; 200],
    pub(crate) caribe: [usize; 200],
    pub(crate) mls: [usize; 12],
    pub(crate) n: [u32; 4],
}

/// The four club lists of 0x4c11a0 in database order (Mexican, Central American, Caribbean, MLS) with their counts,
/// stopping at the first national team.
pub(crate) unsafe fn concacaf_club_lists() -> ConcacafClubLists {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    // ponytail: as quatro listas começam zeradas na cópia (na original, sobra de pilha que só é lida quando falta clube;
    // o comparador manda esse caso para a original sozinha)
    let mut l = ConcacafClubLists { mex: [0; 200], centro: [0; 200], caribe: [0; 200], mls: [0; 12], n: [0; 4] };
    let (mut a, mut b, mut c, mut m) = (0u8, 0u8, 0u8, 0i8);
    let mut clube = read_u32_as_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_u32_as_usize(G_CLUB_COUNT) as i32 {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = read_u32_as_usize(clube + club::NATION);
        if nacao != 0 {
            let id = read_u32_as_usize(nacao);
            if id == read_u32_as_usize(KEY_NATION_MEXICO) {
                if a < 0xc8 {
                    l.mex[a as usize] = clube;
                    a += 1;
                }
            } else {
                let div = read_u32_as_usize(clube + club::DIVISION);
                if div != 0 && read_u32_as_usize(div) == read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
                    if m < 0xc {
                        l.mls[m as usize] = clube;
                        m += 1;
                    }
                } else if id != read_u32_as_usize(KEY_NATION_UNITED_STATES) && read_u8(nacao + nation::REGION) == 7 {
                    if b < 0xc8 {
                        l.centro[b as usize] = clube;
                        b += 1;
                    }
                } else if read_u8(nacao + nation::REGION) == 6 && c < 0xc8 {
                    l.caribe[c as usize] = clube;
                    c += 1;
                }
            }
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    l.n = [a as u32, b as u32, c as u32, m as u8 as u32];
    l
}

/// rule: eight clubs. Two from Mexico (drawn among the 6 with the highest reputation), three from Central America
/// (nation region 7, other than nation [0x9cf4f8]; among the top 9), one from the Caribbean (region 6; among the top 3)
/// and two from the United States: with competition [0x9cf728] running [inference: the US Open Cup], the latest MLS
/// champion and its champion (if the same club, the place goes to the MLS runner-up); without it, two MLS clubs among
/// the top 3 by reputation. Up to 200 clubs per region and 12 from the MLS are considered
/// Original: `ConChamp::select_qualified_clubs` (0x4c11a0, con_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c11a0 } (thiscall, ret 0, con_champ.cpp): `ConChamp::select_qualified_clubs()`; devolve 1. Oito times (+0x3e).
/// Os clubes (até a primeira seleção) em quatro listas: os da nação [0x9cf3d0] (México, até 200); os da divisão
/// [0x9cf590] (a MLS, +0x57 do clube; até 12); os de nação de região 7 (+0x75 da nação, América Central) fora da
/// [0x9cf4f8] (até 200); os de região 6 (o Caribe, até 200). Cada lista ordenada pela reputação, com os 3 × k
/// primeiros embaralhados (0x4afbd0: push k; push n; push lista; add esp, 0xc -> cdecl; k = 2, 3, 1 e 1, a contagem
/// na palavra de baixo). Os times (+0xa7, 6 bytes): 2 e 3 os dois mexicanos; 4, 5 e 6 os três centro-americanos; 7 o
/// caribenho; 0 e 1: sem o objeto da competição [0x9cf728], os dois primeiros da MLS; com ele, o último campeão da MLS
/// (0x4afd80(competição [0x9cf590], 0): push 0; push comp; add esp, 8 -> cdecl) e o da [0x9cf728] (a mesma com +4 do
/// objeto) e, se for o mesmo clube, o 0 passa ao último vice da MLS (lugar 1). Depois +4 e +5 de cada time zerados.
pub(crate) unsafe extern "thiscall" fn con_champ_select_qualified_clubs(this: usize) -> u32 {
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 8);
    let outro = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    let mut l = concacaf_club_lists();
    ordenar(l.mex.as_mut_ptr() as usize, l.n[0], 2);
    ordenar(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos
    ordenar(l.caribe.as_mut_ptr() as usize, l.n[2], 1);
    ordenar(l.mls.as_mut_ptr() as usize, l.n[3], 1);
    let t = |k: usize| read_u32_as_usize(this + 0xa7).wrapping_add(k * 6);
    let pd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut u32, v as u32);
    crate::comparacao::ramo("cqi_11 classificados");
    pd(t(2), l.mex[0]);
    pd(t(3), l.mex[1]); // cqi11: o segundo mexicano
    pd(t(4), l.centro[0]);
    pd(t(5), l.centro[1]);
    pd(t(6), l.centro[2]);
    pd(t(7), l.caribe[0]);
    if outro == 0 {
        pd(t(0), l.mls[0]);
        pd(t(1), l.mls[1]);
    } else {
        crate::comparacao::ramo("cqi_11 campeao da mls");
        let mls = club_comp_record(read_u32_as_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE));
        pd(t(0), colocado(mls, 0));
        pd(t(1), colocado(read_u32_as_usize(outro + 4), 0));
        if read_u32_as_usize(t(0)) == read_u32_as_usize(t(1)) { // cqi11: o campeão das duas dá a vaga ao vice da MLS
            pd(t(0), colocado(mls, 1));
        }
    }
    let mut i = 0i32;
    while i < read_i16(this + 0x3e) as i32 {
        *((read_u32_as_usize(this + 0xa7) + i as usize * 6 + 4) as *mut u8) = 0;
        *((read_u32_as_usize(this + 0xa7) + i as usize * 6 + 5) as *mut u8) = 0;
        i += 1;
    }
    1
}

/// Comparador da 0x4c11a0: lado a lado nos oito times (+0xa7, 48 bytes), em +0x3e e no estado do sorteador (a 0x4afbd0
/// embaralha; a cópia sem contar). Quando falta clube numa lista (menos de 2 mexicanos, 3 centro-americanos, 1
/// caribenho, ou 2 da MLS sem o objeto [0x9cf728]) a original lê sobra de pilha: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SELECT_QUALIFIED_CLUBS);
    let l = concacaf_club_lists();
    let outro = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    if l.n[0] < 2 || l.n[1] < 3 || l.n[2] < 1 || (outro == 0 && l.n[3] < 2) || read_u32_as_usize(this + 0xa7) == 0 {
        return o(this); // ponytail: lista curta (a original lê sobra de pilha) ou sem tabela: só a original
    }
    let mut regioes = vec![(this + 0x3e, 2), (read_u32_as_usize(this + 0xa7), 48)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    // a 0x4afd80 reordena o histórico de competições ([0xae23d4], [0xae237c] linhas de 0x1a bytes) a cada chamada; com
    // empate, a ordem que a original deixou muda o campeão que a cópia acharia: o histórico entra nas regiões
    let historico = read_u32_as_usize(G_CLUB_COMP_HISTORY_TABLE);
    if historico != 0 {
        regioes.push((historico, read_u32_as_usize(G_CLUB_COMP_HISTORY_COUNT).wrapping_mul(0x1a)));
    }
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || crate::aleatorio::sem_contar(|| con_champ_select_qualified_clubs(this)));
    PLACAR.registrar(igual && r == rr, || format!("con_champ_select_qualified_clubs({:#x}) listas {:?} original={:x?} rust={:x?}", this, l.n, mo, mr));
    r
}

/// rule: on the first day of the cup season (empty team table), picks the qualified clubs and the predicted positions;
/// every day, the cup update (draws and matches)
/// Original: `ConChamp::update_for_date` (0x4c1620, con_champ.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c1620 } (thiscall, ret 8, con_champ.cpp; a virtual +0x14 da copa): `ConChamp::update_for_date(hoje, fase)`.
/// Com a tabela de times vazia (o clube do time 0 = 0) e fase ≠ 0: os classificados (0x4c11a0: mov ecx, this ->
/// thiscall, ret 0; negativo, a caixa da linha 0x1ba e devolve 0) e as posições previstas (virtual +0x5c). Depois a
/// atualização da copa (0x51f890: push fase; push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
pub(crate) unsafe extern "thiscall" fn con_champ_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    if read_u32_as_usize(read_u32_as_usize(this + 0xa7)) == 0 && fase != 0 {
        let classificar: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CONCHAMP_SELECT_QUALIFIED_CLUBS);
        if classificar(this) < 0 {
            // ponytail: a 0x4c11a0 sempre devolve 1; o caso de erro fica com a caixa e o 0 da original
            show_error_at(0x9a7d30, 0x1ba);
            return 0;
        }
        crate::comparacao::ramo("cqi_11 dia da selecao");
        let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
        prever(this);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_FOR_DATE);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano: com a
// tabela vazia e fase ≠ 0, a copa sai com 8 times e o time 0 preenchido (a 0x4c11a0 tem o seu comparador); senão +0x3e
// não muda.
/// Comparer for `con_champ_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CON_CHAMP_UPDATE_FOR_DATE);
    if read_u32_as_usize(this + 0xa7) == 0 {
        return o(this, hoje, fase);
    }
    let seleciona = read_u32_as_usize(read_u32_as_usize(this + 0xa7)) == 0 && fase != 0;
    let n = read_i16(this + 0x3e);
    let r = o(this, hoje, fase);
    let igual = if seleciona { read_i16(this + 0x3e) == 8 } else { read_i16(this + 0x3e) == n };
    PLACAR.registrar(igual, || format!("con_champ_update_for_date({:#x}, fase {}) seleciona={} times {} -> {}", this, fase, seleciona, n, read_i16(this + 0x3e)));
    r
}

/// rule: the cup prize fund is 2,000,000; the round table (three-round knockout) comes from virtual +0x3c
/// Original: `ConChamp::set_up_rounds` (0x4c15f0, con_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c15f0 } (thiscall, ret 0, con_champ.cpp; a virtual +0x8c): `ConChamp::set_up_rounds()`; devolve 1. O fundo de
/// prêmios (+0x28) de 2.000.000 e a tabela das rodadas (+0xa3) da virtual +0x3c (push 0; push &+0x3a; push &+0x3c;
/// push −1; mov ecx, this -> thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn con_champ_set_up_rounds(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x1e8480);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere o fundo de
// prêmios (+0x28 = 2.000.000) e a devolução 1.
/// Comparer for `con_champ_set_up_rounds` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_set_up_rounds(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SET_UP_ROUNDS)(this);
    PLACAR.registrar(r == 1 && read_u32_as_usize(this + 0x28) == 0x1e8480, || format!("con_champ_set_up_rounds({:#x}) devolveu {} prêmios {}", this, r, read_u32_as_usize(this + 0x28)));
    r
}

/// rule: the CONCACAF Champions' Cup has 8 teams in single matches: quarter-finals drawn on 1/8 and played on 28/8,
/// semi-final on 1/9 and final on 4/9; category 8, tie-break 3; no per-round prize (−1)
/// Original: `ConChamp::create_round_table` (0x4c0f00, con_champ.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c0f00 } (thiscall, ret 0x10, con_champ.cpp; a virtual +0x3c): `ConChamp::create_round_table(fase, &rodadas,
/// &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 3, extra = 0; a tabela (0x944e46 de 0x138; sem
/// memória, a caixa da linha 0x97 e 0) com as 3 rodadas e, nas duas primeiras, o dia 0 do confronto vazio (0x521f10
/// com −1, −1, −1 e mando 0; a original o chama entre o jogo e os campos, que não se sobrepõem). Fora da −1, 0.
pub(crate) unsafe extern "thiscall" fn con_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 3;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(0x138);
    if t == 0 {
        show_error_at(0x9a7d30, 0x97);
        return 0;
    }
    crate::comparacao::ramo("cqp_13 tabela da concacaf");
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DAY);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in CONCACAF_CHAMPIONS_CUP_ROUNDS.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, ano);
        if i < 2 {
            elo(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

/// Comparer for `con_champ_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CON_CHAMP_CREATE_ROUND_TABLE);
    let a = eng_cc_cup_compare_round_table("con_champ_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| con_champ_create_round_table(this, fase, r, e, s));
    // ponytail: a eng_cc_cup_compare_round_table não olha os dias dos confrontos (+0x23..); o dia 0 das duas primeiras rodadas é
    // conferido aqui contra o valor fixo que a cópia grava
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        PLACAR.registrar(ok, || format!("con_champ_create_round_table: o dia 0 dos confrontos da original difere"));
    }
    a
}
