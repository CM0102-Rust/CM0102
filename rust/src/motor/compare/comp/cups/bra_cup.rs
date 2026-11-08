use super::*;

// The comparers of the routines in ../comp/cups/bra_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/bra_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_cup_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CUP_START_NEW_SEASON);
    compare_cup_new_season("bra_cup_start_new_season", this, || o(this))
}

/// Comparer for `bra_cup_start_new_season_662950` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_cup_start_new_season_662950(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CUP_START_NEW_SEASON_662950);
    compare_cup_new_season("bra_cup_start_new_season_662950", this, || o(this))
}

/// Comparer for `bra_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_bra_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_CUP_CREATE_ROUND_TABLE);
    eng_cc_cup_compare_round_table("bra_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| bra_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `bra_cup_build_team_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 71 vagas (com a devolução 1) e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_bra_cup_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BRA_CUP_BUILD_TEAM_TABLE);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xa7));
        r
    }, || {
        let r = build_brazil_cup_teams(this);
        lr.set(read_u32_as_usize(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { read_i16(this + 0x3e).max(0) as usize * 6 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && list, || format!("bra_cup_build_team_table({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `bra_cup_set_up_rounds` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: +0x28, +0x3a..+0x3d, +0x49/+0x4a e o [0xb67a34] a partir do mesmo estado; as tabelas das rodadas das
/// duas (+0xa3) comparadas nos bytes gravados (eng_cc_cup_written_round_bytes) e a da cópia liberada.
pub(crate) unsafe extern "thiscall" fn comparar_bra_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BRA_CUP_SET_UP_ROUNDS);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let before = read_u32_as_usize(this + 0xa3);
    let reg = [(this + 0x28, 4usize), (this + 0x3a, 4), (this + 0x49, 2), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        to.set(read_u32_as_usize(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, before as u32);
        r
    }, || {
        let r = set_up_brazil_cup_rounds(this);
        tr.set(read_u32_as_usize(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = read_i16(this + 0x3c);
    let table = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && table, || format!("bra_cup_set_up_rounds({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, table));
    a
}
