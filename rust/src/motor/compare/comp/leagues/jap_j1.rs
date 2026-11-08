use super::*;

// The comparers of the routines in ../comp/leagues/jap_j1.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/jap_j1.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `jap_j1_advance_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da fase seguinte: só a original (simplification: ao chegar a 2 monta a final, que aloca e sorteia), com o
/// plano: na última fase, +0x30 igual; senão +0x30 + 1 (a final grava 2, o mesmo valor).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_advance_stage(this: usize) {
    let before = read_usize(this + 0x30) as i32;
    let last = before >= (read_usize(this + 0x2c) as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_JAP_J1_ADVANCE_STAGE)(this);
    let after = read_usize(this + 0x30) as i32;
    PLACAR.registrar(after == if last { before } else { before + 1 }, || format!("jap_j1_advance_stage({:#x}) antes={} depois={}", this, before, after));
}

/// Comparer for `create_championship_final_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da final: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito antes:
/// os campeões dos turnos. Depois, a fase da terceira vaga existe, com +0x3e = 2, +4 = o +4 da liga e os dois campeões
/// na lista dela (+0xa7, 6 bytes; como conjunto), e +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn comparar_create_championship_final_stage(this: usize) {
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_CHAMPIONSHIP_FINAL_STAGE)(this);
    let stage = read_usize(read_usize(this + 0xc) + 8);
    let same = stage != 0 && {
        let l = read_usize(stage + 0xa7);
        let (x, y) = (read_usize(l), read_usize(l + 6));
        read_i16(stage + 0x3e) == 2 && read_usize(stage + 4) == read_usize(this + 4) && ((x, y) == (a, b) || (x, y) == (b, a)) && read_usize(this + 0x30) == 2
    };
    PLACAR.registrar(same, || format!("create_championship_final_stage({:#x}) campeoes={:#x},{:#x} fase={:#x} +0x30={}", this, a, b, stage, read_usize(this + 0x30)));
}

/// Comparer for `jap_j1_start_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador da virada: só a original (simplification: libera e aloca tabelas e fases, sorteia jogos e árbitros e vira a
/// J2; não pode rodar duas vezes), conferindo o que só ela garante: o ano sobe um e +0x2c = 3.
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_start_new_season(this: usize) -> u32 {
    let before = read_i16(this + 0x40);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_START_NEW_SEASON)(this);
    let after = read_i16(this + 0x40);
    PLACAR.registrar(after == before.wrapping_add(1) && read_usize(this + 0x2c) == 3, || format!("jap_j1_start_new_season({:#x}) ano antes={} depois={} +0x2c={}", this, before, after, read_usize(this + 0x2c)));
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

/// Comparer for `jap_j1_create_half_season_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_jap_j1_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_JAP_J1_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("jap_j1_create_half_season_stage", this, k, || o(this, k))
}
