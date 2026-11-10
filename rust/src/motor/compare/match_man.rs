use super::*;

// The comparers of the routines in ../match_man.rs.
// [pt] Os comparadores das rotinas de ../match_man.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `select_automatically`.
pub(crate) unsafe extern "thiscall" fn compare_select_automatically(t: usize, other: usize, mode: u32) -> i8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i8>(IDX_SELECT_AUTOMATICALLY)(t, other, mode)
}

/// Comparer for `apply_squad_rule`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_apply_squad_rule(
    club: usize, manager: usize, rule: usize, mode: u32, nt: u32, nb: u32, nr: u32, starters: usize, bench: usize, reserve: usize,
    out: usize, codes: usize, curtos: usize, marks: usize, bytes: usize, side: usize,
) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, usize)>(IDX_APPLY_SQUAD_RULE);
    if is_recording_top_level() {
        return o(club, manager, rule, mode, nt, nb, nr, starters, bench, reserve, out, codes, curtos, marks, bytes, side);
    }
    let n = |v: u32| (v as u8 as i8).max(0) as usize;
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((out, 4 * (n(nt) + n(nb))));
    if bench != 0 {
        reg.push((bench, 4 * n(nb)));
    }
    if reserve != 0 {
        reg.push((reserve, 4 * n(nr)));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(
        &reg,
        || o(club, manager, rule, mode, nt, nb, nr, starters, bench, reserve, out, codes, curtos, marks, bytes, side),
        || crate::rng::without_counting(|| apply_squad_rule(club, manager, rule, mode, nt, nb, nr, starters, bench, reserve, out, codes, curtos, marks, bytes, side)),
    );
    SCOREBOARD.record(igual, || format!("apply_squad_rule(rule {:x?}) {:x?} x {:x?}", core::slice::from_raw_parts(rule as *const u8, 6), mo, mr));
}

/// Comparer for `off_match_score`.
pub(crate) unsafe extern "C" fn compare_off_match_score(p: usize, pos: u32, flags: u32, club: usize, limit: i32, o: usize) -> i32 {
    let f = original_fn::<unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32>(IDX_OFF_MATCH_SCORE);
    if is_recording_top_level() {
        return f(p, pos, flags, club, limit, o);
    }
    // simplification: without the draw generator’s regions. The rating does not draw: the only random roll comes from 0x62ed70, which
    // updates the player’s fitness (0x62c680) only on the first query and stores it; the copy, after the
    // original, it does not redo it. Restoring the roller between the two exposed this cache difference.
    let reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let rastros = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &reg,
        || {
            let (r, v) = crate::rng::trace(|| f(p, pos, flags, club, limit, o));
            rastros.borrow_mut().0 = v;
            r
        },
        || {
            let (r, v) = crate::rng::trace(|| crate::rng::without_counting(|| off_match_score(p, pos, flags, club, limit, o)));
            rastros.borrow_mut().1 = v;
            r
        },
    );
    SCOREBOARD.record(igual && r == n, || format!("off_match_score({:#x}, pos {:#x}, flags {:#x}, limit {}) original={} rust={} sorteios {:?} x {:?} {:x?} x {:x?}", p, pos as u16, flags, limit, r, n, rastros.borrow().0, rastros.borrow().1, mo, mr));
    r
}

/// Comparer for `lineup_slot_score`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_lineup_slot_score(
    p: usize, f: usize, reg: usize, pos: u32, flags: u32, club: usize, side: usize, manager: usize, limit: i32, mark: u32, marca_alta: u32,
    bytes: u32, tactic: u32, pos2: u32, holiday: usize,
) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32>(IDX_LINEUP_SLOT_SCORE);
    if is_recording_top_level() {
        return o(p, f, reg, pos, flags, club, side, manager, limit, mark, marca_alta, bytes, tactic, pos2, holiday);
    }
    // without the randomizer’s regions, for the same reason as the rating outside the match (0x62ed70 with cache)
    let regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(
        &regions,
        || o(p, f, reg, pos, flags, club, side, manager, limit, mark, marca_alta, bytes, tactic, pos2, holiday),
        || crate::rng::without_counting(|| lineup_slot_score(p, f, reg, pos, flags, club, side, manager, limit, mark, marca_alta, bytes, tactic, pos2, holiday)),
    );
    SCOREBOARD.record(igual && r == n, || format!("lineup_slot_score({:#x}, pos {:#x}/{:#x}, flags {:#x}, mark {:#x}) original={} rust={} {:x?} x {:x?}", p, pos as u16, pos2 as u16, flags, mark, r, n, mo, mr));
    r
}

/// Comparer for `select_side`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_select_side(t: usize, m: usize, manager: usize, rows: usize, players: usize, side: u32, club: usize, other: usize, game: usize, slots: *mut i8) -> i8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize, usize, usize, *mut i8) -> i8>(IDX_SELECT_SIDE)(t, m, manager, rows, players, side, club, other, game, slots)
}

/// Comparer for `perceived_ability`.
pub(crate) unsafe extern "C" fn compare_perceived_ability(o: usize, p: usize, c: usize, kind: i8, rep: i16) -> i16 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16>(IDX_PERCEIVED_ABILITY)(o, p, c, kind, rep);
    let n = perceived_ability(o, p, c, kind, rep);
    SCOREBOARD.record(r == n, || format!("perceived_ability({:#x}, {:#x}, {}, {}) original={} rust={}", o, p, kind, rep, r, n));
    r
}

/// Comparer for `pick`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_pick(
    a1: usize, a2: u32, a3: usize, a4: u32, a5: u32, a6: usize, a7: u32, a8: usize, a9: usize, a10: u32, a11: u32, a12: usize, a13: usize, a14: u32, a15: usize,
) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, usize, u32, u32, usize, usize, u32, usize) -> u8>(IDX_PICK);
    let call = || o(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15);
    let n1 = (a2 as u8 as i8).max(0) as usize;
    let n = ((a4 as u8 as i8 as i32 + a5 as u8 as i8 as i32).max(0)) as usize;
    if is_recording_top_level() {
        // inside a comparison with tape: unit with vectors (the scale reads and writes the caller’s stack)
        return call_recording_buffers(
            0x6e2000,
            0,
            [0, a2, a7, a8 as u32, a9 as u32],
            &[(a1, 4 * n1), (a3, 2 * n), (a12, 8 * n), (a13, n), (a15, 2 * n)],
            &[(a6, 4 * n)],
            || call() as u32,
        ) as u8;
    }
    // Probe (comparison only): 0x6dbc50, in mode 2, never writes the short of the 1st extra slot and passes
    // the stack garbage at the scale (return 0x6dd383). Counts how often it is not zero and whether zeroing it changes
    // the scale (the original twice: with the garbage and with 0).
    if crate::detours::RETURN_TO == 0x6dd383 && a5 as u8 != 0 && a15 != 0 {
        use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
        static CHAMADAS: AtomicU32 = AtomicU32::new(0);
        static NAO_ZERO: AtomicU32 = AtomicU32::new(0);
        static MUDA: AtomicU32 = AtomicU32::new(0);
        let field = (a15 + 2 * (a4 as u8 as usize)) as *mut u16;
        let lixo = *field;
        let k = CHAMADAS.fetch_add(1, Relaxed) + 1;
        if lixo != 0 {
            NAO_ZERO.fetch_add(1, Relaxed);
            let (r1, r2, igual, _, _) = crate::comparison::side_by_side(&[(a6, 4 * n)], call, || {
                *field = 0;
                let r = call();
                *field = lixo;
                r
            });
            if !igual || r1 != r2 {
                MUDA.fetch_add(1, Relaxed);
            }
        }
        if lixo != 0 || k % 500 == 0 {
            crate::registro::linha(&format!(
                "sonda_lixo: value {:#06x}; {} escalas da 0x6dbc50, {} with lixo, {} mudam se cleared",
                lixo, k, NAO_ZERO.load(Relaxed), MUDA.load(Relaxed)
            ));
        }
    }
    // the output and the static dates that the rating (0x6e2f20) touches
    let reg = [(a6, 4 * n), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1)];
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, call, || pick(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15));
    SCOREBOARD.record(igual && r == rr, || format!("pick(n {} slots {}+{} mode {:#x}) original={} rust={} {:x?} x {:x?}", a2 as u8 as i8, a4 as u8 as i8, a5 as u8 as i8, a7, r, rr, mo, mr));
    r
}

/// Comparer for `stance`.
pub(crate) unsafe extern "thiscall" fn compare_stance(c: usize) -> u8 {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_STANCE);
    let m = c.engine();
    // the active ones (engine+4/+5, 0x6cbbb0), the factor of both sides, the dates (0x54cdb0) and the pointer of the
    // table that the league search (0x4a4850) sorts
    let reg = [(m.addr() + 4, 2), (m.addr() + engine::STANCE_FACTOR, 8), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_COMP_STATS_SORT_CONTEXT, 4)];
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(c.addr()), || stance(c.addr()));
    SCOREBOARD.record(igual && r == rr, || format!("stance(side {}) original={} rust={} {:x?} x {:x?}", c.side(), r, rr, mo, mr));
    r
}

/// Comparer for `choose_formation`.
pub(crate) unsafe extern "thiscall" fn compare_choose_formation(c: usize, sheets: usize, a2: u32, a3: u32, a4: u32) -> u8 {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8>(IDX_CHOOSE_FORMATION);
    let n = (a2 as u8 as i8).max(0) as usize;
    if is_recording_top_level() {
        // inside a comparison with a tape: unit with vectors (the profiles are on the caller’s stack)
        return call_recording_buffers(0x6e0230, c.addr(), [0, a2, a3, a4, 0], &[(sheets, 4 * n)], &[], || o(c.addr(), sheets, a2, a3, a4) as u32) as u8;
    }
    // the random roll, the engine, the tactic copy (+0x2b), the club’s rows in the tactics bank (the formation
    // written and what the instructions touch), the rating dates (0x6e2f20) and the search pointer in the league
    let m = c.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (c.tactics(), 0x18e3), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]);
    let taticas = read_usize(G_TACTICS);
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let club = c.club();
    for humano in 0..2u32 {
        let l = if club.is_null() { -1 } else { row(club.addr(), humano) };
        if l != -1 {
            reg.push((read_usize(taticas + tactics_fields::ROWS).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (r, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(c.addr(), sheets, a2, a3, a4), || choose_formation(c.addr(), sheets, a2, a3, a4));
    SCOREBOARD.record(igual && r == rr, || {
        // the first differing byte (the regions are large)
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("choose_formation(n {} mode {:#x} {}) original={} rust={} {}", a2 as u8 as i8, a3, a4 as u8, r, rr, dif.unwrap_or_default())
    });
    r
}

/// Comparer for `instructions`.
pub(crate) unsafe extern "thiscall" fn compare_instructions(c: usize) {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INSTRUCTIONS);
    // the random roll, the engine (man-marking in the tactics block), the copied tactics (+0x2b) and the rows
    // of the club in the tactics database
    let m = c.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (c.tactics(), 0x18e3), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let taticas = read_usize(G_TACTICS);
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let club = c.club();
    for humano in 0..2u32 {
        let l = if club.is_null() { -1 } else { row(club.addr(), humano) };
        if l != -1 {
            reg.push((read_usize(taticas + tactics_fields::ROWS).wrapping_add((l as usize).wrapping_mul(0x91)), 0x91));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(c.addr()), || instructions(c.addr()));
    SCOREBOARD.record(igual, || {
        let dif = mo.0.iter().zip(mr.0.iter()).find_map(|(a, b)| {
            a.1.iter().zip(b.1.iter()).position(|(x, y)| x != y).map(|k| format!("{:#x}+{:#x}: {:x?} x {:x?}", a.0, k, &a.1[k..(k + 8).min(a.1.len())], &b.1[k..(k + 8).min(b.1.len())]))
        });
        format!("instructions(side {}) {}", c.side(), dif.unwrap_or_default())
    });
}

/// Comparer for `put_in_place`.
pub(crate) unsafe extern "thiscall" fn compare_put_in_place(c: usize, pos: u32, person: usize, extra: u32) -> u8 {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8>(IDX_PUT_IN_PLACE);
    if is_recording_top_level() {
        // inside the starters (or the staff): tape unit
        return call_recording(FN_MATCH_MAN_PLACE_PLAYER_IN_TEAM, c.addr(), [pos, person as u32, extra, 0, 0], || o(c.addr(), pos, person, extra) as u32) as u8;
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(c.engine().addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let (r, rr, igual, sequencia, k) = compare_com_fita(reg, || o(c.addr(), pos, person, extra), |chamar| {
        place_in_slot_with(c, pos, person, extra, &mut |rotina, this, args| chamar(rotina, this, args, [u32::MAX, u32::MAX, 0xff, 0xff, 0xff]))
    });
    SCOREBOARD.record(igual && sequencia && r == rr, || format!("put_in_place({}, {:#x}) original={} rust={} {} chamadas: {}", pos as u8 as i8, person, r, rr, k, tape_detail()));
    r
}

/// Comparer for `order_by_table`.
pub(crate) unsafe extern "C" fn compare_order_by_table(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_ORDER_BY_TABLE)(a, b);
    let n = order_by_table(a, b);
    SCOREBOARD.record(r == n, || format!("order_by_table({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// Comparer for `order_by_condition`.
pub(crate) unsafe extern "C" fn compare_order_by_condition(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_ORDER_BY_CONDITION)(a, b);
    let n = order_by_condition(a, b);
    SCOREBOARD.record(r == n, || format!("order_by_condition({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}

/// Comparer for `starters`.
pub(crate) unsafe extern "thiscall" fn compare_starters(c: usize) -> u8 {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_STARTERS);
    if is_recording_top_level() {
        // inside the staff at the start of the match: tape unit
        return call_recording(FN_MATCH_MAN_PLACE_SELECTED_PLAYERS, c.addr(), [0, 0, 0, 0, 0], || o(c.addr()) as u32) as u8;
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(c.engine().addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let (r, rr, igual, sequencia, k) = compare_com_fita(reg, || o(c.addr()), |chamar| {
        starters_with(c, &mut |rotina, this, args| chamar(rotina, this, args, [0xff, u32::MAX, 0xff, 0, 0]))
    });
    SCOREBOARD.record(igual && sequencia && r == rr, || format!("starters(side {}) original={} rust={} {} chamadas: {}", c.side(), r, rr, k, tape_detail()));
    r
}

/// Comparer for `store_tactic`.
pub(crate) unsafe extern "thiscall" fn compare_store_tactic(c: usize) {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_STORE_TACTIC);
    let mut reg = club_rows(c.club().addr());
    reg.push((G_MOUSE_LEFT_BUTTON_DOWN, 4));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(c.addr()), || store_tactic(c.addr()));
    SCOREBOARD.record(igual, || format!("store_tactic(human {}) {}", c.tactic_slot(), first_difference(&mo, &mr)));
}

/// Comparer for `substitute`.
pub(crate) unsafe extern "thiscall" fn compare_substitute(c: usize, goalkeeper: u32) {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SUBSTITUTE);
    if is_recording_top_level() {
        call_recording(FN_MATCH_MAN_MAKE_SUBSTITUTIONS, c.addr(), [goalkeeper, 0, 0, 0, 0], || {
            o(c.addr(), goalkeeper);
            0
        });
        return;
    }
    let m = c.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(c.addr(), goalkeeper), |chamar| {
        substitute_with(c, goalkeeper as u8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 => [0, 0, 0, 0, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia, || format!("substitute(goalkeeper {}) {} chamadas: {}", goalkeeper as u8, k, tape_detail()));
}

/// Comparer for `redo_team`.
pub(crate) unsafe extern "thiscall" fn compare_redo_team(c: usize) {
    let c = MatchManager(c);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REDO_TEAM);
    if is_recording_top_level() {
        call_recording(FN_MATCH_MAN_REASSIGN_POSITIONS, c.addr(), [0, 0, 0, 0, 0], || {
            o(c.addr());
            0
        });
        return;
    }
    let m = c.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(c.addr()), |chamar| {
        rebuild_team_with(c, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6e0230 => [0, 0xff, u32::MAX, u32::MAX, 0],
                _ => [0, 0xff, u32::MAX, u32::MAX, u32::MAX],
            };
            chamar(rotina, this, args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia, || format!("redo_team() {} chamadas: {}", k, tape_detail()));
}

/// Comparer for `best_in_list`.
pub(crate) unsafe extern "thiscall" fn compare_best_in_list(this: usize, list: *const usize, n: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const usize, i8) -> usize>(IDX_BEST_IN_LIST);
    let reg = [crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, rr, igual, _, _) = crate::comparison::side_by_side_light(&reg, || o(this, list, n), || best_in_list(this, list, n));
    SCOREBOARD.record(igual && r == rr, || format!("best_in_list({}) original={:#x} rust={:#x}", n, r, rr));
    r
}

/// Comparer for `best_rated`.
pub(crate) unsafe extern "thiscall" fn compare_best_rated(this: usize) -> i8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i8>(IDX_BEST_RATED);
    let reg = [crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, n, igual, _, _) = crate::comparison::side_by_side_light(&reg, || o(this), || best_rated(this));
    SCOREBOARD.record(igual && r == n, || format!("best_rated() original={} rust={}", r, n));
    r
}

/// Comparer for `rate_player`.
pub(crate) unsafe extern "C" fn compare_rate_player(person: usize, player: usize, club: usize) -> i16 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> i16>(IDX_RATE_PLAYER);
    // 0x54cdb0 touches its static date: the comparison restores it
    let reg = [crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1], (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, player, club), || rate_player(person, player, club));
    SCOREBOARD.record(igual && r == n, || format!("rate_player() original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}
