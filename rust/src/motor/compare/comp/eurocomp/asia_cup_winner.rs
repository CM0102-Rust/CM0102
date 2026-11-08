use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_cup_winner.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_cup_winner.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_cup_winner_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_cup_winner_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_CREATE_KNOCKOUT_STAGE);
    let fases = read_u32_as_usize(this + 0xc);
    if fases == 0 || (0..3).any(|g| read_u32_as_usize(fases + g * 4) == 0) || read_u32_as_usize(this + 0xb1) == 0 {
        return o(this);
    }
    let tabela = |g: i32| if g < 0 { read_u32_as_usize(this + 0xb1) } else { read_u32_as_usize(read_u32_as_usize(fases + g as usize * 4) + 0xb1) };
    let mut plano: Vec<usize> = AFRICAN_NATIONS_QUARTER_FINAL_SLOTS.iter().map(|&(g, p)| read_u32_as_usize(tabela(g) + p as usize * 0x3b)).collect();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("asia_cup_winner_create_knockout_stage({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 8 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4) && read_u32_as_usize(this + 0x30) == 3;
    PLACAR.registrar(igual, || format!("asia_cup_winner_create_knockout_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// Comparer for `asia_cup_winner_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_cup_winner_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("asia_cup_winner_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Comparer for `create_finals_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_finals_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_FINALS_STAGE);
    if read_u32_as_usize(this + 0xa7) == 0 || read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = asian_cup_winners_finalists(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("create_finals_stage({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("create_finals_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// Comparer for `asia_cup_winner_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_cup_winner_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CUP_WINNER_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("asia_cup_winner_create_round_table", fase, asian_cup_winners_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| asia_cup_winner_create_round_table(this, fase, r, e, l))
}

/// Comparer for `create_team_entries` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_team_entries(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CREATE_TEAM_ENTRIES);
    compare_team_table("create_team_entries", this, || o(this), || create_team_entries(this))
}
