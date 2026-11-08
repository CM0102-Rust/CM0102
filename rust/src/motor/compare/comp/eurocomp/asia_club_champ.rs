use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_club_champ.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_club_champ_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    let esperado = if ultima { f } else if f + 1 == 1 { 2 } else { f + 1 };
    PLACAR.registrar(depois == esperado, || format!("asia_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Comparer for `create_group_stages` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_group_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_GROUP_STAGES);
    if read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let v = group_stage_qualifiers(this);
    o(this);
    let confere = |g: usize, de: &[usize]| {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        if t == 0 || read_i16(f + 0x3e) != 4 || read_i16(f + 0x40) != read_i16(this + 0x40) {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
        let mut b = de.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = confere(1, &v[..4]) && confere(2, &v[4..]) && read_u32_as_usize(this + 0x30) == 2;
    PLACAR.registrar(ok, || format!("create_group_stages({:#x}): os grupos não batem com os vivos {:x?}", this, v));
}

/// Comparer for `asia_club_champ_create_semi_final_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SEMI_FINAL_STAGE);
    let fases = read_u32_as_usize(this + 0xc);
    if fases == 0 || read_u32_as_usize(fases + 4) == 0 || read_u32_as_usize(fases + 8) == 0 {
        return o(this);
    }
    let mut plano = asia_semi_finalists(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if fase == 0 {
        PLACAR.registrar(false, || format!("asia_club_champ_create_semi_final_stage({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("asia_club_champ_create_semi_final_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// Comparer for `asia_club_champ_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CLUB_CHAMP_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("asia_club_champ_create_round_table", fase, asian_club_champ_stage(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| asia_club_champ_create_round_table(this, fase, r, e, l))
}

/// Comparer for `asia_club_champ_build_team_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_asia_club_champ_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ASIA_CLUB_CHAMP_BUILD_TEAM_TABLE);
    compare_team_table("asia_club_champ_build_team_table", this, || o(this), || asia_club_champ_build_team_table(this))
}
