use super::*;

// The comparers of the routines in ../printouts.rs.
// [pt] Os comparadores das rotinas de ../printouts.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `pick_random_pundit`.
///
/// [pt] O comparador: lado a lado nos cabeçalhos das duas listas (o cursor) e no sorteio, e a devolução.
pub(crate) unsafe extern "thiscall" fn compare_pick_random_pundit(this: usize, club: usize, kind: u32, nation: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32>(IDX_PICK_RANDOM_PUNDIT);
    let mut regions = Vec::new();
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regions.push((l, 0xe));
        }
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || o(this, club, kind, nation),
        || crate::rng::without_counting(|| pick_random_pundit(this, club, kind, nation)),
    );
    SCOREBOARD.record(igual && r == n, || format!("pick_random_pundit(clube {:#x}, tipo {}, nacao {}) original={} {:x?} rust={} {:x?}", club, kind as i8, nation, r, mo, n, mr));
    r
}

/// Comparer for `get_pundit_name`.
///
/// [pt] O comparador: lado a lado no nome (até 0xd4 bytes: o nome do comentarista tem 0xd1 e o padrão menos), na marca e
/// nos cabeçalhos das listas (o cursor).
pub(crate) unsafe extern "thiscall" fn compare_get_pundit_name(this: usize, kind: u32, id: u32, name: usize, mark: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8)>(IDX_GET_PUNDIT_NAME);
    let mut regions = vec![(mark as usize, 1)];
    if name != 0 {
        regions.push((name, 0xd4));
    }
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regions.push((l, 0xe));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, kind, id, name, mark), || get_pundit_name(this, kind, id, name, mark));
    SCOREBOARD.record(igual, || format!("get_pundit_name(tipo {}, id {}) original={:x?} rust={:x?}", kind as i8, id as i16, mo, mr));
}
