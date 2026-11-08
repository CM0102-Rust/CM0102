use super::*;

// The comparers of the routines in ../match_day.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_day.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `end_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_day(day: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_END_DAY)(day)
}

/// Comparer for `pick_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pick_day_match(day: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PICK_DAY_MATCH)(day)
}

/// Comparer for `start_next_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_start_next_match(day: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_START_NEXT_MATCH)(day)
}

/// Comparer for `step_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_step_day_match(day: usize, until_end: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_STEP_DAY_MATCH)(day, until_end)
}

/// Comparer for `close_day_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_close_day_match(day: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CLOSE_DAY_MATCH)(day, p)
}

/// Comparer for `play_the_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_play_the_day(day: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PLAY_THE_DAY)(day)
}

/// Comparer for `set_up_the_day` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_up_the_day(day: usize, data: usize, mode: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_SET_UP_THE_DAY)(day, data, mode)
}

/// Comparer for `match_day_interest` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_day_interest(d: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_MATCH_DAY_INTEREST);
    if is_recording_top_level() {
        return o(d);
    }
    // simplification: 0x5fe8d0 (the humans list) runs in both; it rebuilds the list from scratch
    let reg = match_day_marked_regions(d);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(d), || match_day_interest(d));
    PLACAR.record(igual && r == n, || format!("match_day_interest original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `match_without_human` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_without_human(partida: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MATCH_WITHOUT_HUMAN);
    if is_recording_top_level() {
        return o(partida);
    }
    let mut reg = vec![(0xa15704usize, 4usize)];
    if *(G_QUICK_SIM_COMP_FLAGS_PENDING as *const usize) != 0 {
        reg.extend(human_national_team_marks().into_iter().map(|e| (e, 1)));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(partida), || match_without_human(partida));
    PLACAR.record(igual && r == n, || format!("match_without_human original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `pos` (comparison mode: see the header of motor.rs).
///
/// [pt] Roda a comparacao de uma rotina do pos-jogo pela fita: as 14 do banco entram como unidades.
pub(crate) unsafe fn comparar_pos(name: &str, reg: Vec<(usize, usize)>, original: impl FnOnce(), rust: impl FnOnce(&mut dyn FnMut(usize, usize, [u32; 5]) -> u32)) {
    use std::sync::atomic::Ordering::Relaxed;
    let write = RECORD_POST_MATCH.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, original, |chamar| rust(&mut |r, t, a| chamar(r, t, a, [u32::MAX; 5])));
    RECORD_POST_MATCH.store(write, Relaxed);
    PLACAR.record(igual && sequencia, || format!("{} ({} chamadas): {}", name, k, tape_detail()));
}

/// Comparer for `post_match_events` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_events(partida: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_POST_MATCH_EVENTS);
    // the list cursor, the two clubs’ rows in [[0xb67a44]] and the flags of the events’ contracts
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    let list = *(G_MATCH_DAY_PLAYER_INCIDENT_LIST as *const usize);
    if list != 0 {
        reg.push((list, 0xe));
        let person = |id: i32| (*(G_STAFF_TABLE as *const usize) as isize + id as isize * 0x6e) as usize;
        let mut no = core::ptr::read_unaligned((list + 2) as *const usize);
        let mut k = 0;
        while no != 0 && k < core::ptr::read_unaligned(list as *const i16) as i32 {
            let ev = core::ptr::read_unaligned(no as *const usize);
            no = core::ptr::read_unaligned((no + 6) as *const usize);
            k += 1;
            if ev == 0 {
                break;
            }
            let clube = (*(G_CLUB_TABLE as *const usize) as isize + core::ptr::read_unaligned((ev + 4) as *const i32) as isize * 0x245) as usize;
            if clube == *((partida + 0x1c) as *const usize) || clube == *((partida + 0x20) as *const usize) {
                reg.extend(contract_flags_region(person(core::ptr::read_unaligned(ev as *const i32)), clube));
            }
        }
    }
    let table = *(G_MANAGER_MANAGER as *const usize);
    if table != 0 {
        for o in [0x1c, 0x20] {
            let clube = *((partida + o) as *const usize);
            if clube != 0 {
                reg.push(((*(table as *const usize) as isize + *(clube as *const i32) as isize * 0x49) as usize, 0x49));
            }
        }
    }
    comparar_pos("post_match_events", reg, || o(partida), |chamar| post_match_events_injected(partida, chamar));
}

/// Comparer for `post_match_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_players(partida: usize, t: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_POST_MATCH_PLAYERS);
    // the player records (+0x45, +9 to +0xe), the contracts, +0x80 of the clubs and the records of the
    // manager and of the assistants (+8 to +0xd)
    let g = |a: usize| *(a as *const usize);
    let read = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    for lado in 0..2usize {
        let clube = (g(0xae23b4) as isize + read(t + 4 * lado) as isize * 0x245) as usize;
        let ts = t + lado * 0x625;
        reg.push((clube + club::REPUTATION, 2));
        for k in 0..20usize {
            let id = read(ts + 0x1c5 + k * 0x49 - 0x35);
            if id == -1 {
                continue;
            }
            let p = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let f = g(p + staff::PLAYER);
            if f != 0 {
                reg.extend([(f + 0x45, 1), (f + 9, 6)]);
                reg.extend(contract_flags_region(p, clube));
            }
        }
        let id = read(ts + 0x184);
        if id > -1 {
            let manager = (g(G_STAFF_TABLE) as isize + id as isize * 0x6e) as usize;
            let mut fichas = vec![g(manager + staff::NON_PLAYER)];
            for a in [g(clube + club::ASSISTANT_MANAGER), g(clube + club::COACHES), g(clube + 0x1a3), g(clube + 0x1a7), g(clube + 0x1ab), g(clube + 0x1af)] {
                if a != 0 {
                    fichas.push(g(a + 0x69));
                }
            }
            for f in fichas.into_iter().filter(|&f| f != 0) {
                reg.push((f + 8, 6));
            }
        }
    }
    comparar_pos("post_match_players", reg, || o(partida, t), |chamar| post_match_players_with(partida, t, chamar));
}

/// Comparer for `post_match_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_post_match_club(clube: usize, other: usize, partida: usize, n: u32, t: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, usize)>(IDX_POST_MATCH_CLUB);
    // the copy does not write to the database: only the calls and the random rolls
    let reg = crate::rng::STATE_REGIONS.to_vec();
    comparar_pos("post_match_club", reg, || o(clube, other, partida, n, t), |chamar| post_match_club_with(clube, other, partida, n, t, chamar));
}

/// Comparer for `match_day_drop_viewer_and_finish_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_day_drop_viewer_and_finish_match() {
    original_fn::<unsafe extern "C" fn()>(IDX_MATCH_DAY_DROP_VIEWER_AND_FINISH_MATCH)()
}

/// Comparer for `find_record_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_record_fixture(stats: usize, k: i32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(IDX_FIND_RECORD_FIXTURE);
    if k < 0 || k > (*((stats + 0x1c) as *const i32)).wrapping_sub(1) || out == 0 {
        return o(stats, k, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(out, 0x4f)], || o(stats, k, out), || find_record_fixture(stats, k, out));
    PLACAR.record(igual && r == n, || format!("find_record_fixture(k {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// Comparer for `network_step` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_network_step(day: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NETWORK_STEP)(day, p)
}

/// Comparer for `confirm_network_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_confirm_network_match(day: usize, p: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CONFIRM_NETWORK_MATCH)(day, p)
}

/// Comparer for `competition_number` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_competition_number(code: u32, second: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32) -> u32>(IDX_COMPETITION_NUMBER);
    let r = o(code, second);
    if matches!(code as u16 as i16, 0 | 1 | 0x1ef | 0x3de | 0x483 | 0x528) {
        let n = competition_number(code, second);
        PLACAR.record(r as u8 == n as u8, || format!("competition_number({:#x}, {}) original={} rust={}", code as u16, second as u8, r as u8, n as u8));
    }
    r
}
