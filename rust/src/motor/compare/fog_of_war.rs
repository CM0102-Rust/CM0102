use super::*;

// The comparers of the routines in ../fog_of_war.rs.
// [pt] Os comparadores das rotinas de ../fog_of_war.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `chance_to_know`.
pub(crate) unsafe extern "C" fn compare_chance_to_know(person: usize, level: i8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, i8) -> i32>(IDX_CHANCE_TO_KNOW)(person, level);
    let v = chance_to_know(person, level);
    SCOREBOARD.record(r == v, || format!("chance_to_know({}, {}) original={} rust={}", read_i16(person + 4), level, r, v));
    r
}

/// Comparer for `know_person`.
pub(crate) unsafe extern "thiscall" fn compare_know_person(this: usize, person: usize, font: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_KNOW_PERSON);
    let mut reg: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    if person != 0 && *(person as *const i32) < read_i32(G_STAFF_COUNT) {
        reg.push((read_usize(this).wrapping_add((read_i32(person) as usize).wrapping_mul(9)), 9));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, font), || crate::rng::without_counting(|| know_person(this, person, font)));
    SCOREBOARD.record(igual, || format!("know_person original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `fog_after_match`.
pub(crate) unsafe extern "stdcall" fn compare_fog_after_match(partida: usize, data: usize) {
    let partida = Fixture(partida);
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize)>(IDX_FOG_AFTER_MATCH);
    // the registers the routine may touch (same tests, without the 0x32 of the reserves: leftover does not hurt)
    let n = read_i32(G_STAFF_COUNT);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    for lado in 0..2usize {
        let club = Club(read_usize(partida.addr() + fixture::AWAY_CLUB - 4 * lado));
        if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) == 0 || club.manager().addr() == 0 {
            continue;
        }
        let q = club.manager().addr();
        let obj = read_usize(read_usize(G_FOG_OF_WAR).wrapping_add((n.wrapping_sub(read_i32(q)) as usize).wrapping_mul(4)).wrapping_sub(4));
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let i = read_i32(data + lado * 0x625 + 0x190 + vaga * 0x49);
            let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
            if i != -1 && p != 0 && *(p as *const i32) < n {
                reg.push((read_usize(obj).wrapping_add((read_i32(p) as usize).wrapping_mul(9)), 9));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(partida.addr(), data), || fog_after_match(partida.addr(), data));
    SCOREBOARD.record(igual, || format!("fog_after_match original={:x?} rust={:x?}", mo, mr));
}
