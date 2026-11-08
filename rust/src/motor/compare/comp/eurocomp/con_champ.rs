use super::*;

// The comparers of the routines in ../comp/eurocomp/con_champ.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/con_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparador da 0x4c11a0: lado a lado nos oito times (+0xa7, 48 bytes), em +0x3e e no estado do sorteador (a 0x4afbd0
/// embaralha; a cópia sem contar). Quando falta clube numa lista (menos de 2 mexicanos, 3 centro-americanos, 1
/// caribenho, ou 2 da MLS sem o objeto [0x9cf728]) a original lê sobra de pilha: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SELECT_QUALIFIED_CLUBS);
    let l = concacaf_club_lists();
    let other = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_US_OPEN_CUP).wrapping_mul(4)));
    if l.n[0] < 2 || l.n[1] < 3 || l.n[2] < 1 || (other == 0 && l.n[3] < 2) || read_u32_as_usize(this + 0xa7) == 0 {
        return o(this); // simplification: short list (the original reads stack leftover) or no table: original only
    }
    let mut regions = vec![(this + 0x3e, 2), (read_u32_as_usize(this + 0xa7), 48)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    // 0x4afd80 reorders the competition history ([0xae23d4], [0xae237c] rows of 0x1a bytes) on every call; with
    // tie, the order the original left changes the champion the copy would find: the history goes into the regions
    let history = read_u32_as_usize(G_CLUB_COMP_HISTORY_TABLE);
    if history != 0 {
        regions.push((history, read_u32_as_usize(G_CLUB_COMP_HISTORY_COUNT).wrapping_mul(0x1a)));
    }
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || crate::rng::without_counting(|| con_champ_select_qualified_clubs(this)));
    PLACAR.record(igual && r == rr, || format!("con_champ_select_qualified_clubs({:#x}) listas {:?} original={:x?} rust={:x?}", this, l.n, mo, mr));
    r
}

/// Comparer for `con_champ_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CON_CHAMP_UPDATE_FOR_DATE);
    if read_u32_as_usize(this + 0xa7) == 0 {
        return o(this, today, stage);
    }
    let seleciona = read_u32_as_usize(read_u32_as_usize(this + 0xa7)) == 0 && stage != 0;
    let n = read_i16(this + 0x3e);
    let r = o(this, today, stage);
    let same = if seleciona { read_i16(this + 0x3e) == 8 } else { read_i16(this + 0x3e) == n };
    PLACAR.record(same, || format!("con_champ_update_for_date({:#x}, fase {}) seleciona={} times {} -> {}", this, stage, seleciona, n, read_i16(this + 0x3e)));
    r
}

/// Comparer for `con_champ_set_up_rounds` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_set_up_rounds(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_CHAMP_SET_UP_ROUNDS)(this);
    PLACAR.record(r == 1 && read_u32_as_usize(this + 0x28) == 0x1e8480, || format!("con_champ_set_up_rounds({:#x}) devolveu {} prêmios {}", this, r, read_u32_as_usize(this + 0x28)));
    r
}

/// Comparer for `con_champ_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_con_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CON_CHAMP_CREATE_ROUND_TABLE);
    let a = compare_cup_round_table("con_champ_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| con_champ_create_round_table(this, stage, r, e, s));
    // simplification: compare_cup_round_table does not look at the fixture days (+0x23..); day 0 of the first two rounds is
    // checked here against the fixed value the copy writes
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        PLACAR.record(ok, || format!("con_champ_create_round_table: o dia 0 dos confrontos da original difere"));
    }
    a
}
