use super::*;

// The comparers of the routines in ../comp/cups/bra_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/cups/bra_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `bra_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bra_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CUP_START_NEW_SEASON);
    compare_cup_new_season("bra_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `bra_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_bra_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("bra_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| bra_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `bra_cup_build_team_table`.
///
/// [pt] Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 71 vagas (com a devolução 1) e liberada.
pub(crate) unsafe extern "thiscall" fn compare_bra_cup_build_team_table(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BRA_CUP_BUILD_TEAM_TABLE);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        lo.set(this.team_table());
        r
    }, || {
        let r = build_brazil_cup_teams(this);
        lr.set(this.team_table());
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { this.team_count().max(0) as usize * 6 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("bra_cup_build_team_table({:#x}) devolveu {} x {} memória {} lista {}", this.addr(), a, b, igual, list));
    a
}

/// Comparer for `bra_cup_set_up_rounds`.
///
/// [pt] Lado a lado: +0x28, +0x3a..+0x3d, +0x49/+0x4a e o [0xb67a34] a partir do mesmo estado; as tabelas das rodadas das
/// duas (+0xa3) comparadas nos bytes gravados (eng_cc_cup_written_round_bytes) e a da cópia liberada.
pub(crate) unsafe extern "thiscall" fn compare_bra_cup_set_up_rounds(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BRA_CUP_SET_UP_ROUNDS);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let before = this.round_table();
    let reg = [(this.addr() + cup::PRIZE_FUND, 4usize), (this.addr() + cup::STAGE_LABEL, 4), (this.addr() + cup::SUBSTITUTES_NAMED, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        to.set(this.round_table());
        this.set_round_table((before as u32) as usize);
        r
    }, || {
        let r = set_up_brazil_cup_rounds(this);
        tr.set(this.round_table());
        this.set_round_table((to.get() as u32) as usize);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = this.round_count();
    let table = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        exe::crt_free(tr);
    }
    SCOREBOARD.record(igual && a == b && table, || format!("bra_cup_set_up_rounds({:#x}) devolveu {} x {} memória {} tabela {}", this.addr(), a, b, igual, table));
    a
}
