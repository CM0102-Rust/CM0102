use super::*;

/// Competition record id in the database (G_CLUB_COMP_TABLE + 0x6b × id).
///
/// [pt] O registro da competição `id` no banco ([0xae23d0] + 0x6b × id).
pub(crate) unsafe fn club_comp_record(id: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b))
}

/// The four lists of 0x4c11a0 (Mexican, Central American, Caribbean and MLS clubs) with their counts.
///
/// Built in the order of the club database ([0xae23b4], 0x245 bytes, [0xae235c] clubs; it stops at the first one that
/// is a national team, 0x53b900).
///
/// [pt] As quatro listas da 0x4c11a0, na ordem do banco de clubes ([0xae23b4], 0x245 bytes, [0xae235c] clubes; para no
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
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    // simplification: the four lists start zeroed in the copy (in the original, stack leftover that is only read when a club is missing;
    // the comparer sends this case to the original alone)
    let mut l = ConcacafClubLists { mex: [0; 200], centro: [0; 200], caribe: [0; 200], mls: [0; 12], n: [0; 4] };
    let (mut a, mut b, mut c, mut m) = (0u8, 0u8, 0u8, 0i8);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if national_team(club) != 0 {
            break;
        }
        let nation = read_usize(club + club::NATION);
        if nation != 0 {
            let id = read_usize(nation);
            if id == read_usize(KEY_NATION_MEXICO) {
                if a < 0xc8 {
                    l.mex[a as usize] = club;
                    a += 1;
                }
            } else {
                let div = read_usize(club + club::DIVISION);
                if div != 0 && read_usize(div) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
                    if m < 0xc {
                        l.mls[m as usize] = club;
                        m += 1;
                    }
                } else if id != read_usize(KEY_NATION_UNITED_STATES) && read_u8(nation + nation::REGION) == 7 {
                    if b < 0xc8 {
                        l.centro[b as usize] = club;
                        b += 1;
                    }
                } else if read_u8(nation + nation::REGION) == 6 && c < 0xc8 {
                    l.caribe[c as usize] = club;
                    c += 1;
                }
            }
        }
        i += 1;
        club = club.wrapping_add(0x245);
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
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 8);
    let other = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    let mut l = concacaf_club_lists();
    exe::comp_util_sort_and_shuffle_top_clubs(l.mex.as_mut_ptr() as usize, l.n[0], 2);
    exe::comp_util_sort_and_shuffle_top_clubs(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos
    exe::comp_util_sort_and_shuffle_top_clubs(l.caribe.as_mut_ptr() as usize, l.n[2], 1);
    exe::comp_util_sort_and_shuffle_top_clubs(l.mls.as_mut_ptr() as usize, l.n[3], 1);
    let t = |k: usize| read_usize(this + cup::TEAM_TABLE).wrapping_add(k * 6);
    let pd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut u32, v as u32);
    crate::comparison::branch("cqi_11 classificados");
    pd(t(2), l.mex[0]);
    pd(t(3), l.mex[1]); // cqi11: o segundo mexicano
    pd(t(4), l.centro[0]);
    pd(t(5), l.centro[1]);
    pd(t(6), l.centro[2]);
    pd(t(7), l.caribe[0]);
    if other == 0 {
        pd(t(0), l.mls[0]);
        pd(t(1), l.mls[1]);
    } else {
        crate::comparison::branch("cqi_11 campeao da mls");
        let mls = club_comp_record(read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE));
        pd(t(0), exe::comp_util_get_latest_placed_club(mls, 0));
        pd(t(1), exe::comp_util_get_latest_placed_club(read_usize(other + 4), 0));
        if read_usize(t(0)) == read_usize(t(1)) { // cqi11: the champion of both gives the slot to the MLS runner-up
            pd(t(0), exe::comp_util_get_latest_placed_club(mls, 1));
        }
    }
    let mut i = 0i32;
    while i < read_i16(this + cup::TEAM_COUNT) as i32 {
        *((read_usize(this + cup::TEAM_TABLE) + i as usize * 6 + 4) as *mut u8) = 0;
        *((read_usize(this + cup::TEAM_TABLE) + i as usize * 6 + 5) as *mut u8) = 0;
        i += 1;
    }
    1
}

/// Daily update of the CONCACAF Champions' Cup. Rule: on the first day of the cup season (empty team table), picks the
/// qualified clubs and the predicted positions; every day, the cup update (draws and matches).
///
/// Ret 8; virtual +0x14 of the cup: `ConChamp::update_for_date(today, stage)`. With the team table empty (the club of
/// team 0 = 0) and stage ≠ 0: the qualified clubs (0x4c11a0: mov ecx, this -> thiscall, ret 0; negative: the box at
/// line 0x1ba and returns 0) and the predicted positions (virtual +0x5c). Then the cup update (0x51f890: push stage;
/// push today; mov ecx, this -> thiscall, ret 8), whose return value is returned.
/// Original: `ConChamp::update_for_date` (0x4c1620, con_champ.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c1620 } (thiscall, ret 8, con_champ.cpp; a virtual +0x14 da copa): `ConChamp::update_for_date(hoje, fase)`.
/// Com a tabela de times vazia (o clube do time 0 = 0) e fase ≠ 0: os classificados (0x4c11a0: mov ecx, this ->
/// thiscall, ret 0; negativo, a caixa da linha 0x1ba e devolve 0) e as posições previstas (virtual +0x5c). Depois a
/// atualização da copa (0x51f890: push fase; push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
pub(crate) unsafe extern "thiscall" fn con_champ_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    if read_usize(read_usize(this + cup::TEAM_TABLE)) == 0 && stage != 0 {
        if exe::conchamp_select_qualified_clubs(this) < 0 {
            // simplification: 0x4c11a0 always returns 1; the error case stays with the box and the original’s 0
            show_error_at(0x9a7d30, 0x1ba);
            return 0;
        }
        crate::comparison::branch("cqi_11 dia da selecao");
        let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + 0x5c));
        prever(this);
    }
    exe::cup_update_for_date(this, today, stage)
}

// simplification: the cup update (0x51f890) plays and draws: does not run twice. Only the original runs; the plan: with the
// empty table and stage ≠ 0, the cup comes out with 8 teams and team 0 filled (0x4c11a0 has its own comparer); otherwise +0x3e
// does not change.

/// Sets up the rounds of the CONCACAF Champions' Cup. Rule: the cup prize fund is 2,000,000; the round table
/// (three-round knockout) comes from virtual +0x3c.
///
/// Ret 0; virtual +0x8c: `ConChamp::set_up_rounds()`; returns 1. The prize fund (+0x28) of 2,000,000 and the round
/// table (+0xa3) from virtual +0x3c (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall, ret 0x10).
/// Original: `ConChamp::set_up_rounds` (0x4c15f0, con_champ.cpp, thiscall, ret 0).
///
/// [pt] { 0x4c15f0 } (thiscall, ret 0, con_champ.cpp; a virtual +0x8c): `ConChamp::set_up_rounds()`; devolve 1. O fundo de
/// prêmios (+0x28) de 2.000.000 e a tabela das rodadas (+0xa3) da virtual +0x3c (push 0; push &+0x3a; push &+0x3c;
/// push −1; mov ecx, this -> thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn con_champ_set_up_rounds(this: usize) -> u32 {
    core::ptr::write_unaligned((this + cup::PRIZE_FUND) as *mut u32, 0x1e8480);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
    let t = table(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    core::ptr::write_unaligned((this + cup::ROUND_TABLE) as *mut u32, t as u32);
    1
}

// simplification: virtual +0x3c allocates the rounds table: does not run twice. Original only; the plan checks the bottom of
// prizes (+0x28 = 2,000,000) and return value 1.

/// Creates the round table of the CONCACAF Champions' Cup. Rule: 8 teams in single matches: quarter-finals drawn on 1/8
/// and played on 28/8, semi-final on 1/9 and final on 4/9; category 8, tie-break 3; no per-round prize (−1).
///
/// Ret 0x10; virtual +0x3c: `ConChamp::create_round_table(stage, &rounds, &extra, &out)`: at stage −1, out (if
/// requested) = 0, rounds = 3, extra = 0; the table (0x944e46 of 0x138; out of memory: the box at line 0x97 and 0) with
/// the 3 rounds and, in the first two, day 0 of the empty tie (0x521f10 with −1, −1, −1 and home 0; the original calls
/// it between the match and the fields, which do not overlap). Outside −1, 0.
/// Original: `ConChamp::create_round_table` (0x4c0f00, con_champ.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x4c0f00 } (thiscall, ret 0x10, con_champ.cpp; a virtual +0x3c): `ConChamp::create_round_table(fase, &rodadas,
/// &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 3, extra = 0; a tabela (0x944e46 de 0x138; sem
/// memória, a caixa da linha 0x97 e 0) com as 3 rodadas e, nas duas primeiras, o dia 0 do confronto vazio (0x521f10
/// com −1, −1, −1 e mando 0; a original o chama entre o jogo e os campos, que não se sobrepõem). Fora da −1, 0.
pub(crate) unsafe extern "thiscall" fn con_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 3;
    *extra = 0;
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alloc(0x138);
    if t == 0 {
        show_error_at(0x9a7d30, 0x97);
        return 0;
    }
    crate::comparison::branch("cqp_13 tabela da concacaf");
    let year = core::ptr::read_unaligned((this + cup::YEAR) as *const i16) as i32 as u32;
    for (i, r) in CONCACAF_CHAMPIONS_CUP_ROUNDS.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
        if i < 2 {
            exe::cup_set_round_match_day(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

