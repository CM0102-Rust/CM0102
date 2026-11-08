use super::*;

// ---------- 281a leva: os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1 (eur_super_cup.cpp, asia_super_cup.cpp, ita_c1_super.cpp) ----------

/// { 0x57a500 } (thiscall, ret 0, eur_super_cup.cpp): `EurSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc1 e −1). O campeão da Liga dos Campeões ([0x9cf6e8]; sem ele, a caixa 0xca e 0) e o da Recopa
/// ([0x9cf6ec]) na copa de 1999, nas outras o da Copa da UEFA ([0x9cf6f0]); sem ele, 0xd5 e 0. Devolve 1.
pub(crate) unsafe fn cqq11_europeia(this: usize) -> (i32, i32) {
    const F: usize = 0x9c45f8;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm10_colocado(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0); // cqq11: o campeão da Liga dos Campeões
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    let copa = if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7cf { KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP } else { KEY_CLUB_COMP_UEFA_CUP };
    let b = cqm10_colocado(copa, 0); // cqq11: o campeão da Copa da UEFA
    if !cqm12_vaga(l, 1, b, F, 0xd5) {
        return (0, 0);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa europeia");
    cqm12_fim(l)
}

/// rule: the European Super Cup is between the Champions League winner and the Cup Winners' Cup winner in the 1999
/// edition (the last Cup Winners' Cup) and, in the others, the UEFA Cup winner; without either, the error box
/// Original: `EurSuperCup::select_teams` (0x57a500, eur_super_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa Europeia é entre o campeão da Liga dos Campeões e o da Recopa na edição de 1999 (a última Recopa)
/// e, nas outras, o da Copa da UEFA; sem um dos dois, a caixa de erro
pub(crate) unsafe extern "thiscall" fn cqq11_europeia_clubes(this: usize) -> i32 {
    cqq11_europeia(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_cqq11_europeia_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1980);
    cqm12_comparar("cqq11_europeia_clubes", this, || o(this), || cqq11_europeia(this))
}

/// { 0x410a50 } (thiscall, ret 0, asia_super_cup.cpp): `AsiaSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc0 e −1). O campeão da Liga dos Campeões asiática ([0x9cf64c]; sem ele, 0xc9 e 0) e o da Recopa
/// asiática ([0x9cf650]; sem ele, 0xd1 e 0); se é o mesmo clube, o primeiro vira o vice da Liga dos Campeões (sem
/// conferir, como a original). Devolve 1.
pub(crate) unsafe fn cqq11_asiatica(this: usize) -> (i32, i32) {
    const F: usize = 0x987be0;
    let l = match cqm12_lista(this, F, 0xc0) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm10_colocado(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 0);
    if !cqm12_vaga(l, 0, a, F, 0xc9) {
        return (0, 0);
    }
    let b = cqm10_colocado(KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP, 0); // cqq11: o campeão da Recopa asiática
    if !cqm12_vaga(l, 1, b, F, 0xd1) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqq_11 dobradinha asiatica");
        core::ptr::write_unaligned(l as *mut u32, cqm10_colocado(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 1) as u32);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa asiatica");
    cqm12_fim(l)
}

/// rule: the Asian Super Cup is between the Asian Club Championship winner and the Asian Cup Winners' Cup winner; if
/// the same club won both, the Club Championship runner-up plays
/// Original: `AsiaSuperCup::select_teams` (0x410a50, asia_super_cup.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa Asiática é entre o campeão da Liga dos Campeões asiática e o da Recopa asiática; se o mesmo clube
/// ganhou as duas, entra o vice da Liga dos Campeões
pub(crate) unsafe extern "thiscall" fn cqq11_asiatica_clubes(this: usize) -> i32 {
    cqq11_asiatica(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_cqq11_asiatica_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1981);
    cqm12_comparar("cqq11_asiatica_clubes", this, || o(this), || cqq11_asiatica(this))
}

/// { 0x63c9d0 } (thiscall, ret 0, ita_c1_super.cpp): `ItaC1Super::init_team_list()`: a lista de dois (sem memória, a
/// caixa da linha 0xc2 e −1) com as duas vagas vazias (os clubes entram depois, com os campeões das duas chaves da C1
/// [inferência]); classe e situação 0. Devolve 1.
pub(crate) unsafe fn cqq11_c1(this: usize) -> (i32, i32) {
    let l = match cqm12_lista(this, 0x9ea1c8, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    crate::comparacao::ramo("cqq_11 vagas da supercopa c1");
    core::ptr::write_unaligned(l as *mut u32, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, 0);
    cqm12_fim(l)
}

/// rule: the Serie C1 Super Cup starts with both places empty; the clubs enter at the end of the season [inference: the
/// champions of the two C1 groups]
/// Original: `ItaC1Super::init_team_list` (0x63c9d0, ita_c1_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da Série C1 começa com as duas vagas vazias; os clubes entram no fim da temporada [inferência: os
/// campeões das duas chaves da C1]
pub(crate) unsafe extern "thiscall" fn cqq11_c1_vagas(this: usize) -> i32 {
    cqq11_c1(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_cqq11_c1_vagas(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1982);
    cqm12_comparar("cqq11_c1_vagas", this, || o(this), || cqq11_c1(this))
}

/// rule: the European Super Cup is a single match on 24 August (drawn on 4 July) on neutral ground (venue 0x8000:
/// Monaco [inference]), category 8, tie-break 0x83, no prizes
/// Original: `EurSuperCup::create_round_table` (0x57a3c0, eur_super_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x57a3c0 } (thiscall, ret 0x10, eur_super_cup.cpp; a virtual +0x3c da Supercopa Europeia): `EurSuperCup::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x98 sem memória.
pub(crate) unsafe extern "thiscall" fn cqp10_supercopa_europeia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP10_SUPERCOPA_EUROPEIA, 0x9c45f8, 0x98, "cqp_10 tabela da supercopa europeia")
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp10_supercopa_europeia_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1965);
    cql10_comparar("cqp10_supercopa_europeia_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqp10_supercopa_europeia_tabela(this, fase, r, e, s))
}
