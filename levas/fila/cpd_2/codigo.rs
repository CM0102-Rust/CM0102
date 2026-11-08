// ---------- %%LEVA%%a leva: as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21 (national_teams.cpp) ----------

unsafe fn cpd2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O time da nação para o tipo de convocação: 0 e 1 a seleção principal (0x53b3d0), os outros o sub-21 (0x53b4f0).
unsafe fn national_team_for_type(nacao: usize, tipo: u8) -> usize {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    if tipo == 0 || tipo == 1 { principal(nacao) } else { sub21(nacao) }
}

/// Tira a pessoa `p` da tática do time (0x896c60 de [0xde1d08], com 1 e com 0 no 3º argumento), como as originais
/// fazem antes de esvaziar a vaga.
unsafe fn remove_from_tactics(time: usize, p: usize) {
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x896c60usize);
    marcar(*(0xde1d08 as *const usize), time, p, 1, 1);
    marcar(*(0xde1d08 as *const usize), time, p, 0, 1);
}

/// A pessoa sai da seleção: sem time (+0x24), sem a marca +0x28 e só com os bits de clube (& 3) em +0x5e.
unsafe fn clear_national_call_up(p: usize) {
    core::ptr::write_unaligned((p + 0x24) as *mut usize, 0);
    *((p + 0x28) as *mut u8) = 0; // a marca de convocado
    *((p + 0x5e) as *mut u8) &= 3;
}

/// { 0x776250 } (thiscall, ret 0xc, national_teams.cpp): `add_player_to_squad(nacao, tipo, p)`: a primeira vaga
/// livre das 50 do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe `p`; sem vaga, nada. A pessoa ganha o
/// time (+0x24), a data de hoje em +0x29 (0x417c10 de [0xae2c90]), a marca 0xb em +0x28 e o elenco (0x540980) 4
/// (tipo 0 e 2) ou 8. Sem SEH, sem caixa, sem sorteio; a local é a cópia da data que a 0x417c10 devolve.
unsafe extern "thiscall" fn add_player_to_squad(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let tipo = tipo as u8;
    let time = national_team_for_type(nacao, tipo);
    let elenco: u32 = if tipo == 0 || tipo == 2 { 4 } else { 8 };
    let vaga = match (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == 0) {
        Some(c) => time + 0xd7 + 4 * c,
        None => return,
    };
    core::ptr::write_unaligned(vaga as *mut usize, p); // a vaga
    core::ptr::write_unaligned((p + 0x24) as *mut usize, time);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut data = [0u8; 8];
    copia(p + 0x29, data.as_mut_ptr(), 0xae2c90);
    *((p + 0x28) as *mut u8) = 0xb;
    let marcar_elenco: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    marcar_elenco(p, elenco);
}

/// As regiões de pôr e tirar do elenco: as 50 vagas do time e a pessoa (+0x24 a +0x5e).
unsafe fn squad_change_regions(time: usize, p: usize) -> [(usize, usize); 2] {
    [(time + 0xd7, 200), (p + 0x24, 0x3b)]
}

unsafe extern "thiscall" fn compare_add_player_to_squad(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let time = national_team_for_type(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = squad_change_regions(time, p);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nacao, tipo, p), || add_player_to_squad(this, nacao, tipo, p));
    SCOREBOARD.record(igual, || format!("add_player_to_squad({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// { 0x7762f0 } (thiscall, ret 0xc, national_teams.cpp): `remove_player_from_squad(nacao, tipo, p)`: tira `p` da
/// tática do time (0x896c60, duas vezes) e, se ele está numa das 50 vagas (+0xd7), esvazia a vaga e limpa a
/// pessoa (+0x24, +0x28, +0x5e & 3). Sem SEH, sem sorteio, sem locais; caixa só com time ou pessoa nulos (0x896c60).
unsafe extern "thiscall" fn national_teams_remove_player_from_squad(_this: usize, nacao: usize, tipo: u32, p: usize) {
    let time = national_team_for_type(nacao, tipo as u8);
    remove_from_tactics(time, p);
    if let Some(c) = (0..50usize).find(|&c| cpd2_d(time + 0xd7 + 4 * c) == p) {
        core::ptr::write_unaligned((time + 0xd7 + 4 * c) as *mut usize, 0);
        clear_national_call_up(p);
    }
}

/// Comparador: as vagas e a pessoa. A 0x896c60 tira a pessoa da tática: na segunda vez (a cópia) já não a acha e não
/// muda nada, então a tática fica fora das regiões. Time ou pessoa nulos (a caixa da 0x896c60): só a original.
unsafe extern "thiscall" fn compare_national_teams_remove_player_from_squad(this: usize, nacao: usize, tipo: u32, p: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    let time = national_team_for_type(nacao, tipo as u8);
    if time == 0 || p == 0 {
        return o(this, nacao, tipo, p);
    }
    let reg = squad_change_regions(time, p);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nacao, tipo, p), || national_teams_remove_player_from_squad(this, nacao, tipo, p));
    SCOREBOARD.record(igual, || format!("national_teams_remove_player_from_squad({:x}, {}, {:x}) {:x?} x {:x?}", nacao, tipo as u8, p, mo, mr));
}

/// O registro da nação no gerenciador das seleções: [this + 4] + 0x6e × id (+0 da nação).
unsafe fn nation_record_by_id(this: usize, nacao: usize) -> usize {
    cpd2_d(this + 4).wrapping_add((*(nacao as *const i32)).wrapping_mul(0x6e) as usize)
}

/// { 0x76d880 } (thiscall, ret 4, national_teams.cpp): `release_overage_u21_players(nacao)`: depois da data do
/// registro da nação (+0x62/+0x66; 0x456740 sobre [0xae2c90]), cada uma das 50 vagas do sub-21 (0x53b4f0) com
/// pessoa de idade (+0x18, byte com sinal) acima de 21 sai da tática (0x896c60, duas vezes), é limpa (+0x24, +0x28,
/// +0x5e & 3) e a vaga esvazia; por fim zera o byte +0x6b do registro. Sem SEH, sem sorteio; a local é o `this`.
unsafe extern "thiscall" fn release_overage_u21_players(this: usize, nacao: usize) {
    let rec = nation_record_by_id(this, nacao);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    if depois(0xae2c90, cpd2_d(rec + 0x62) as u32, cpd2_d(rec + 0x66) as u32) == 0 {
        return;
    }
    crate::comparison::branch("under-21 release over age");
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    for c in 0..50usize {
        let vaga = time + 0xd7 + 4 * c;
        let p = cpd2_d(vaga);
        if p != 0 && *((p + 0x18) as *const i8) > 0x15 { // acima de 21 anos
            remove_from_tactics(time, p);
            clear_national_call_up(p);
            core::ptr::write_unaligned(vaga as *mut usize, 0);
        }
    }
    *((nation_record_by_id(this, nacao) + 0x6b) as *mut u8) = 0;
}

/// Comparador: o byte +0x6b do registro, as vagas do sub-21 e as pessoas delas (+0x24 a +0x5e). A 0x896c60 da
/// cópia não acha mais as pessoas que a original já tirou (fica fora das regiões, como na 0x7762f0). Sub-21 nulo:
/// só a original.
unsafe extern "thiscall" fn compare_release_overage_u21_players(this: usize, nacao: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let sub21: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    let time = sub21(nacao);
    if time == 0 {
        return o(this, nacao);
    }
    let mut reg = vec![(nation_record_by_id(this, nacao) + 0x6b, 1), (time + 0xd7, 200)];
    reg.extend((0..50usize).map(|c| cpd2_d(time + 0xd7 + 4 * c)).filter(|&p| p != 0).map(|p| (p + 0x24, 0x3b)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, nacao), || release_overage_u21_players(this, nacao));
    SCOREBOARD.record(igual, || format!("release_overage_u21_players({:x}, {:x}) {:x?} x {:x?}", this, nacao, mo, mr));
}
