use super::*;

// The comparers of the routines in ../match_official.rs.
// [pt] Os comparadores das rotinas de ../match_official.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_referee`.
pub(crate) unsafe extern "thiscall" fn compare_construct_referee(a: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_REFEREE);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&[(a, 8)], || o(a), || construct_referee(a));
    SCOREBOARD.record(igual && r == v, || format!("construct_referee({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", a, r, mo, v, mr));
    r
}

/// Comparer for `init_strictness`.
pub(crate) unsafe extern "thiscall" fn compare_init_strictness(this: usize, engine: u32, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_INIT_STRICTNESS);
    let mut reg = vec![(this, 10)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, engine, person), || crate::rng::without_counting(|| init_strictness(this, engine, person)));
    SCOREBOARD.record(igual, || format!("init_strictness original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `move_counts`.
pub(crate) unsafe extern "thiscall" fn compare_move_counts(this: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_MOVE_COUNTS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this + engine::PENDING_PLAY_KIND, 16)], || o(this), || move_counts(this));
    SCOREBOARD.record(igual && r == n, || format!("move_counts original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}

/// Comparer for `decide_foul`.
pub(crate) unsafe extern "thiscall" fn compare_decide_foul(this: usize, j: usize, code: u16, kind: *mut u8, value: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(IDX_DECIDE_FOUL);
    let engine = *(this as *const usize);
    let mut reg = vec![(this + 8, 2), (kind as usize, 1), (value as usize, 1), (engine + engine::PENDING_PLAY_KIND, 16)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, code, kind, value), || crate::rng::without_counting(|| decide_foul(this, j, code, kind, value)));
    SCOREBOARD.record(igual && r == n, || format!("decide_foul({:#x}) original={} {:x?} rust={} {:x?}", code, r, mo, n, mr));
    r
}

/// Comparer for `punish_foul`.
pub(crate) unsafe extern "thiscall" fn compare_punish_foul(this: usize, j: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(IDX_PUNISH_FOUL);
    let reg = card_regions(*(this as *const usize), j, kind, weight);
    let t0 = *kind;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, kind, x, sev, flag, weight), || crate::rng::without_counting(|| punish_foul(this, j, kind, x, sev, flag, weight)));
    SCOREBOARD.record(igual, || format!("punish_foul(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `punish_move`.
pub(crate) unsafe extern "thiscall" fn compare_punish_move(this: usize, j: usize, other: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32, expulsar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(IDX_PUNISH_MOVE);
    let mut reg = card_regions(*(this as *const usize), j, kind, weight);
    reg.push((other + 0x15, 1));
    let t0 = *kind;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, j, other, kind, x, sev, flag, weight, expulsar), || crate::rng::without_counting(|| punish_move(this, j, other, kind, x, sev, flag, weight, expulsar)));
    SCOREBOARD.record(igual, || format!("punish_move(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `foul_decision`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_foul_decision(
    this: usize, j: usize, other: usize, kind: i8, who: i8, card: *mut u8, grav: *mut i8, advantage: *mut u8,
) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(IDX_FOUL_DECISION);
    let mut regions = vec![(this + 8, 2), (card as usize, 1), (grav as usize, 1), (advantage as usize, 1)];
    for p in [j, other] {
        if p != 0 {
            regions.push((p, 0x1be));
        }
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || o(this, j, other, kind, who, card, grav, advantage),
        || crate::rng::without_counting(|| foul_decision(this, j, other, kind, who, card, grav, advantage)),
    );
    SCOREBOARD.record(igual && r == v, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("foul_decision(tipo {:#x}) original={} rust={} diferencas em {:x?}", kind, r, v, dif)
    });
    r
}
