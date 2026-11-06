// ---------- %%LEVA%%a leva: os clubes das supercopas: a Supercopa Europeia, a Supercopa Asiática e a Supercopa da Série C1 (eur_super_cup.cpp, asia_super_cup.cpp, ita_c1_super.cpp) ----------

/// { 0x57a500 } (thiscall, ret 0, eur_super_cup.cpp): `EurSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc1 e −1). O campeão da Liga dos Campeões ([0x9cf6e8]; sem ele, a caixa 0xca e 0) e o da Recopa
/// ([0x9cf6ec]) na copa de 1999, nas outras o da Copa da UEFA ([0x9cf6f0]); sem ele, 0xd5 e 0. Devolve 1.
unsafe fn cqq11_europeia(this: usize) -> (i32, i32) {
    const F: usize = 0x9c45f8;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf6e8, 0); // cqq11: o campeão da Liga dos Campeões
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    let copa = if core::ptr::read_unaligned((this + 0x40) as *const i16) == 0x7cf { 0x9cf6ec } else { 0x9cf6f0 };
    let b = cqm12_colocado(copa, 0); // cqq11: o campeão da Copa da UEFA
    if !cqm12_vaga(l, 1, b, F, 0xd5) {
        return (0, 0);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa europeia");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_europeia_clubes(this: usize) -> i32 {
    cqq11_europeia(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_europeia_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    cqm12_comparar("cqq11_europeia_clubes", this, || o(this), || cqq11_europeia(this))
}

/// { 0x410a50 } (thiscall, ret 0, asia_super_cup.cpp): `AsiaSuperCup::select_teams()`: a lista de dois (sem memória, a
/// caixa da linha 0xc0 e −1). O campeão da Liga dos Campeões asiática ([0x9cf64c]; sem ele, 0xc9 e 0) e o da Recopa
/// asiática ([0x9cf650]; sem ele, 0xd1 e 0); se é o mesmo clube, o primeiro vira o vice da Liga dos Campeões (sem
/// conferir, como a original). Devolve 1.
unsafe fn cqq11_asiatica(this: usize) -> (i32, i32) {
    const F: usize = 0x987be0;
    let l = match cqm12_lista(this, F, 0xc0) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf64c, 0);
    if !cqm12_vaga(l, 0, a, F, 0xc9) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf650, 0); // cqq11: o campeão da Recopa asiática
    if !cqm12_vaga(l, 1, b, F, 0xd1) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqq_11 dobradinha asiatica");
        core::ptr::write_unaligned(l as *mut u32, cqm12_colocado(0x9cf64c, 1) as u32);
    }
    crate::comparacao::ramo("cqq_11 clubes da supercopa asiatica");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_asiatica_clubes(this: usize) -> i32 {
    cqq11_asiatica(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_asiatica_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    cqm12_comparar("cqq11_asiatica_clubes", this, || o(this), || cqq11_asiatica(this))
}

/// { 0x63c9d0 } (thiscall, ret 0, ita_c1_super.cpp): `ItaC1Super::init_team_list()`: a lista de dois (sem memória, a
/// caixa da linha 0xc2 e −1) com as duas vagas vazias (os clubes entram depois, com os campeões das duas chaves da C1
/// [inferência]); classe e situação 0. Devolve 1.
unsafe fn cqq11_c1(this: usize) -> (i32, i32) {
    let l = match cqm12_lista(this, 0x9ea1c8, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    crate::comparacao::ramo("cqq_11 vagas da supercopa c1");
    core::ptr::write_unaligned(l as *mut u32, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, 0);
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqq11_c1_vagas(this: usize) -> i32 {
    cqq11_c1(this).0
}

unsafe extern "thiscall" fn comparar_cqq11_c1_vagas(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    cqm12_comparar("cqq11_c1_vagas", this, || o(this), || cqq11_c1(this))
}
