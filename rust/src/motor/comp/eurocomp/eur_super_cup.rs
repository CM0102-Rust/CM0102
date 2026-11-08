use super::*;

/// EurSuperCup::select_teams (0x57a500): Champions League winner and Cup Winners' Cup (1999) or UEFA Cup winner.
///
/// [pt] { 0x57a500 } (thiscall, ret 0, eur_super_cup.cpp): `EurSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc1 e −1). O campeão da Liga dos Campeões ([0x9cf6e8]; sem ele, a caixa 0xca e 0) e o da Recopa
/// ([0x9cf6ec]) na copa de 1999, nas outras o da Copa da UEFA ([0x9cf6f0]); sem ele, 0xd5 e 0. Devolve 1.
pub(crate) unsafe fn select_european_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9c45f8;
    let l = match alloc_two_team_list(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0); // cqq11: o campeão da Liga dos Campeões
    if !set_team_slot(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    let cup = if core::ptr::read_unaligned((this + cup::YEAR) as *const i16) == 0x7cf { KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP } else { KEY_CLUB_COMP_UEFA_CUP };
    let b = latest_placed_club(cup, 0); // cqq11: o campeão da Copa da UEFA
    if !set_team_slot(l, 1, b, F, 0xd5) {
        return (0, 0);
    }
    crate::comparison::branch("cqq_11 clubes da supercopa europeia");
    finish_two_team_list(l)
}

/// rule: the European Super Cup is between the Champions League winner and the Cup Winners' Cup winner in the 1999
/// edition (the last Cup Winners' Cup) and, in the others, the UEFA Cup winner; without either, the error box
/// Original: `EurSuperCup::select_teams` (0x57a500, eur_super_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa Europeia é entre o campeão da Liga dos Campeões e o da Recopa na edição de 1999 (a última Recopa)
/// e, nas outras, o da Copa da UEFA; sem um dos dois, a caixa de erro
pub(crate) unsafe extern "thiscall" fn eur_super_cup_select_teams(this: usize) -> i32 {
    select_european_super_cup_teams(this).0
}


/// AsiaSuperCup::select_teams (0x410a50): Asian Champions Cup and Cup Winners' Cup winners (Champions runner-up if the
/// same club).
///
/// [pt] { 0x410a50 } (thiscall, ret 0, asia_super_cup.cpp): `AsiaSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc0 e −1). O campeão da Liga dos Campeões asiática ([0x9cf64c]; sem ele, 0xc9 e 0) e o da Recopa
/// asiática ([0x9cf650]; sem ele, 0xd1 e 0); se é o mesmo clube, o primeiro vira o vice da Liga dos Campeões (sem
/// conferir, como a original). Devolve 1.
pub(crate) unsafe fn select_asian_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x987be0;
    let l = match alloc_two_team_list(this, F, 0xc0) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 0);
    if !set_team_slot(l, 0, a, F, 0xc9) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP, 0); // cqq11: o campeão da Recopa asiática
    if !set_team_slot(l, 1, b, F, 0xd1) {
        return (0, 0);
    }
    if a == b {
        crate::comparison::branch("cqq_11 dobradinha asiatica");
        core::ptr::write_unaligned(l as *mut u32, latest_placed_club(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 1) as u32);
    }
    crate::comparison::branch("cqq_11 clubes da supercopa asiatica");
    finish_two_team_list(l)
}



/// ItaC1Super::init_team_list (0x63c9d0): two-entry list with both slots empty.
///
/// [pt] { 0x63c9d0 } (thiscall, ret 0, ita_c1_super.cpp): `ItaC1Super::init_team_list()`: a lista de dois (sem memória, a
/// caixa da linha 0xc2 e −1) com as duas vagas vazias (os clubes entram depois, com os campeões das duas chaves da C1
/// [inferência]); classe e situação 0. Devolve 1.
pub(crate) unsafe fn init_c1_super_cup_list(this: usize) -> (i32, i32) {
    let l = match alloc_two_team_list(this, 0x9ea1c8, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    crate::comparison::branch("cqq_11 vagas da supercopa c1");
    core::ptr::write_unaligned(l as *mut u32, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, 0);
    finish_two_team_list(l)
}



/// rule: the European Super Cup is a single match on 24 August (drawn on 4 July) on neutral ground (venue 0x8000:
/// Monaco [inference]), category 8, tie-break 0x83, no prizes
/// Original: `EurSuperCup::create_round_table` (0x57a3c0, eur_super_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x57a3c0 } (thiscall, ret 0x10, eur_super_cup.cpp; a virtual +0x3c da Supercopa Europeia): `EurSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
pub(crate) unsafe extern "thiscall" fn eur_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &EUROPEAN_SUPER_CUP_ROUNDS, 0x9c45f8, 0x98, "cqp_10 tabela da supercopa europeia")
}

