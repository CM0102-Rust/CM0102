use super::*;

// The comparers of the routines in ../formation.rs.
// [pt] Os comparadores das rotinas de ../formation.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `position_mask`.
pub(crate) unsafe extern "C" fn compare_position_mask(p: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_POSITION_MASK)(p);
    let n = position_mask(p);
    SCOREBOARD.record(r == n, || format!("position_mask({:#x}) original={:#x} rust={:#x}", p, r, n));
    r
}

/// Comparer for `pitch_cell`.
pub(crate) unsafe extern "C" fn compare_pitch_cell(mask: u32, attack: i8, band: u32, out: *mut i16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(IDX_PITCH_CELL);
    let reg = [(out as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(mask, attack, band, out), || pitch_cell(mask, attack, band, out));
    SCOREBOARD.record(igual && r == v, || format!("pitch_cell({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", mask, attack, band, r, mo, v, mr));
    r
}

/// Comparer for `set_slot_coordinate`.
///
/// [pt] Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_coordinate(this: usize, home: u32, stage: u32, band: u32, coord: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_SET_SLOT_COORDINATE);
    let reg = formation_regions(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, home, stage, band, coord), || set_slot_coordinate(this, home, stage, band, coord));
    SCOREBOARD.record(igual && r == n, || format!("set_slot_coordinate({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, home as u8 as i8, stage as u8 as i8, band, coord, r, n, mo, mr));
    r
}

/// Comparer for `check_duplicate_position`.
///
/// [pt] Comparador: pura (grava só a saída): as duas sobre saídas próprias.
pub(crate) unsafe extern "thiscall" fn compare_check_duplicate_position(this: usize, home: u32, out: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(IDX_CHECK_DUPLICATE_POSITION);
    if !(0..=10).contains(&(home as u8 as i8)) {
        return o(this, home, out); // simplification: the box, original only
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, home, &mut so);
    let n = check_duplicate_position(this, home, &mut sr);
    SCOREBOARD.record(r == n && so == sr, || format!("check_duplicate_position({:x}, {}) {}/{} saída {}/{}", this, home as u8 as i8, r, n, so, sr));
    *out = so;
    r
}

/// Comparador: grava um byte e +0x187f: lado a lado.
pub(crate) unsafe extern "thiscall" fn compare_set_marking_type(this: usize, home: u32, mark: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_MARKING_TYPE);
    if !matches!(mark as u8, 1 | 2 | 4 | 8 | 0x10) {
        return o(this, home, mark); // simplification: the box, original only
    }
    let reg = [(this as isize + home as u8 as i8 as isize + 0x5b4) as usize, 1usize];
    let reg = [(reg[0], reg[1]), (this + formation_fields::CHANGED_FLAG, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, home, mark), || set_marking_type(this, home, mark));
    SCOREBOARD.record(igual && r == n, || format!("set_marking_type({:x}, {}, {}) {}/{} {:x?} x {:x?}", this, home as u8 as i8, mark as u8, r, n, mo, mr));
    r
}

/// Comparer for `build_position_lookup`.
pub(crate) unsafe extern "thiscall" fn compare_build_position_lookup(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BUILD_POSITION_LOOKUP);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this + formation_fields::POSITION_LOOKUP, 0x187b - 0x5eb)], || o(this), || build_position_lookup(this));
    SCOREBOARD.record(igual && r == n, || format!("build_position_lookup {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `stored_cell`.
pub(crate) unsafe extern "thiscall" fn compare_stored_cell(t: usize, slot: i8, attack: i8, band: u32, out: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(IDX_STORED_CELL);
    let reg = [(out as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(t, slot, attack, band, out), || stored_cell(t, slot, attack, band, out));
    SCOREBOARD.record(igual && r == v, || format!("stored_cell({:#x}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, slot, attack, band, r, mo, v, mr));
    r
}

/// Comparer for `set_formation_flag`.
pub(crate) unsafe extern "thiscall" fn compare_set_formation_flag(t: usize, m: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_FORMATION_FLAG);
    let reg = [(t + formation_fields::FLAGS, 4), (t + formation_fields::CHANGED_FLAG, 4)];
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(t, m), || set_formation_flag(t, m));
    SCOREBOARD.record(igual && r == v, || format!("set_formation_flag({:#x}, {:#x}) original={} {:x?} rust={} {:x?}", t, m, r, mo, v, mr));
    r
}

/// Comparer for `load_from_disk`.
///
/// [pt] Comparador: lado a lado na formação (0x18e3 bytes: até as instruções dos jogadores em +0x188b + 0x58) e na
/// devolução; o arquivo é aberto e lido pelas duas (só leitura).
pub(crate) unsafe extern "thiscall" fn compare_load_from_disk(this: usize, path: usize, quiet: u32, dimmed: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_LOAD_FROM_DISK);
    if this == 0 {
        return o(this, path, quiet, dimmed);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x18e3)], || o(this, path, quiet, dimmed),
        || load_from_disk(this, path, quiet, dimmed));
    SCOREBOARD.record(igual && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).take(8).collect();
        format!("load_from_disk(this {:x}) original={} rust={}, bytes diferentes {:x?}", this, r, n, dif)
    });
    r
}

/// Comparer for `set_slot_position`.
///
/// [pt] Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
pub(crate) unsafe extern "thiscall" fn compare_set_slot_position(this: usize, home: u32, position: u32, extra: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SET_SLOT_POSITION);
    if !(0..=10).contains(&(home as u8 as i8)) {
        return o(this, home, position, extra);
    }
    let reg = [(this + formation_fields::SLOT_CELLS, 0x584 - 0x12d), (this + formation_fields::FORMATION_MASK, 8), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, home, position, extra), || set_slot_position(this, home, position, extra));
    SCOREBOARD.record(igual && r == n, || format!("set_slot_position({}, {:#x}, {}) {:x?} x {:x?}", home as u8 as i8, position as u16, extra, mo, mr));
    r
}

/// Comparer for `forget_person`.
pub(crate) unsafe extern "thiscall" fn compare_forget_person(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FORGET_PERSON);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    if person != 0 && *(person as *const i32) < read_i32(G_STAFF_COUNT) {
        for k in 0..16usize {
            let q = read_usize(this + 4 * k);
            if q != 0 {
                reg.push((read_usize(q).wrapping_add((read_i32(person) as usize).wrapping_mul(9)), 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person), || forget_person(this, person));
    SCOREBOARD.record(igual, || format!("forget_person original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `clear_assigned`.
pub(crate) unsafe extern "thiscall" fn compare_clear_assigned(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CLEAR_ASSIGNED);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&[(this + formation_fields::MARKING_TYPES, 11), (this + formation_fields::DESIGNATED_PEOPLE, 44)], || o(this), || clear_assigned(this));
    SCOREBOARD.record(igual && r == v, || format!("clear_assigned original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `competition_criterion`.
pub(crate) unsafe extern "C" fn compare_competition_criterion(record: usize, a: u32, b: u32, kind: u8) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32, u32, u8) -> u8>(IDX_COMPETITION_CRITERION)(record, a, b, kind);
    let v = competition_criterion(record, a, b, kind);
    SCOREBOARD.record(r == v, || format!("competition_criterion({:#x}, {:#x}, {:#x}, {}) original={} rust={}", record, a, b, kind, r, v));
    r
}

/// Comparer for `formation_mask`.
pub(crate) unsafe extern "thiscall" fn compare_formation_mask(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FORMATION_MASK);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&[(this + formation_fields::FORMATION_MASK, 4)], || o(this), || formation_mask(this));
    SCOREBOARD.record(igual && r == v, || format!("formation_mask original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `set_square`.
pub(crate) unsafe extern "thiscall" fn compare_set_square(this: usize, position: i8, mask: u64) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u64) -> u32>(IDX_SET_SQUARE);
    let regions = [(this + formation_fields::CHANGED_FLAG, 0x64), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, position, mask), || set_square(this, position, mask));
    SCOREBOARD.record(igual && r == n, || format!("set_square({}, {:x}) original={} rust={} {:x?} x {:x?}", position, mask, r, n, mo, mr));
    r
}

/// Comparer for `mark_stars`.
pub(crate) unsafe extern "thiscall" fn compare_mark_stars(this: usize, side: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_MARK_STARS);
    let regions = [(this, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, side), || mark_stars(this, side));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("mark_stars({}) diferencas em {:x?}", side, dif)
    });
}

/// Comparer for `designated_for_position`.
pub(crate) unsafe extern "thiscall" fn compare_designated_for_position(this: usize, position: i8) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> usize>(IDX_DESIGNATED_FOR_POSITION)(this, position);
    let n = designated_for_position(this, position);
    SCOREBOARD.record(r == n, || format!("designated_for_position({}) original={:x} rust={:x}", position, r, n));
    r
}
