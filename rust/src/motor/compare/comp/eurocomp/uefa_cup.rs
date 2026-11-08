use super::*;

// The comparers of the routines in ../comp/eurocomp/uefa_cup.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/uefa_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `build_entrants` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado a partir do mesmo estado: +0x14, +0x36..+0x39, +0x3e, +0xb2, [0xb67a34] e o +0x1db de cada clube; as duas
/// listas (+0xa7 com 161 vagas e +0x14 com +0x38 vagas) de cada lado comparadas e as da cópia liberadas.
pub(crate) unsafe extern "thiscall" fn compare_build_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BUILD_ENTRANTS);
    let (a0, t0) = (read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14));
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut reg: Vec<(usize, usize)> = vec![(this + 0x36, 4), (this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)];
    reg.extend((0..read_u32_as_usize(G_CLUB_COUNT)).map(|i| (base + i * club::SIZE + 0x1db, 4)));
    let lo = core::cell::Cell::new((0usize, 0usize));
    let lr = core::cell::Cell::new((0usize, 0usize));
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set((read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14)));
        write_u32_from_usize(this + 0xa7, a0);
        write_u32_from_usize(this + 0x14, t0);
        r
    }, || {
        let r = uefa_cup_build_entrants(this);
        lr.set((read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14)));
        write_u32_from_usize(this + 0xa7, lo.get().0);
        write_u32_from_usize(this + 0x14, lo.get().1);
        r
    });
    let ((ao, to), (ar, tr)) = (lo.get(), lr.get());
    let same_in = |x: usize, y: usize, n: usize| (x == 0 && y == 0) || (x != 0 && y != 0 && core::slice::from_raw_parts(x as *const u8, n) == core::slice::from_raw_parts(y as *const u8, n));
    let lists = same_in(ao, ar, 0xa1 * 6) && (a != 1 || same_in(to, tr, read_i16(this + 0x36).max(0) as usize * 6));
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if ar != 0 && ar != ao {
        soltar(ar);
    }
    if tr != 0 && tr != to && tr != t0 {
        soltar(tr);
    }
    SCOREBOARD.record(igual && a == b && lists, || format!("build_entrants({:#x}) devolveu {} x {} memória {} listas {}", this, a, b, igual, lists));
    a
}

/// Comparer for `uefa_cup_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_UEFA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("uefa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| uefa_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `uefa_cup_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_UEFA_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if club == 0 {
        return o(this, club, outcome, a3, a4, game, a6);
    }
    let d = outcome as u8;
    let before = read_u32_as_usize(club + club::CONTINENTAL_COMP_ID);
    let previsto = if d == 1 || d == 2 || d == 0xff { 0xffff_ffff } else { before };
    let r = o(this, club, outcome, a3, a4, game, a6);
    let after = read_u32_as_usize(club + club::CONTINENTAL_COMP_ID);
    SCOREBOARD.record(r == 0 && after == previsto, || format!("uefa_cup_update_staff_records_for_status({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", club, d, before, after, previsto, r));
    r
}
