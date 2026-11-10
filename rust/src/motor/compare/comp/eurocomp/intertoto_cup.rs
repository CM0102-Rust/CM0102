use super::*;

// The comparers of the routines in ../comp/eurocomp/intertoto_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/intertoto_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `intertoto_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_INTERTOTO_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("intertoto_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| intertoto_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `intertoto_cup_collect_entrants`.
///
/// [pt] Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a lista dela em +0xa7, a cópia faz a
/// sua, comparada nos inscritos (com a devolução 1) e liberada.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_INTERTOTO_CUP_COLLECT_ENTRANTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let before = this.team_table();
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this.addr());
        lo.set(this.team_table());
        this.set_team_table((before as u32) as usize);
        r
    }, || {
        let (r, n) = collect_intertoto_entrants(this);
        lr.set(this.team_table());
        nr.set(n);
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { nr.get().max(0) as usize * 6 } else { 0 };
    let list = if lo != before && lr != before && lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == lr || k == 0
    };
    if lr != 0 && lr != lo && lr != before {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("intertoto_cup_collect_entrants({:#x}) returned {} x {} memory {} list {}", this.addr(), a, b, igual, list));
    a
}

/// Comparer for `intertoto_cup_collect_club_list`.
///
/// [pt] Lado a lado: a original grava a lista dela em `saida` (de quem chama, que a libera); a cópia faz a sua numa saída
/// própria, comparada na conta e nos ponteiros, e a libera.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_collect_club_list(this: usize, out: *mut usize, ativos: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16>(IDX_INTERTOTO_CUP_COLLECT_CLUB_LIST);
    let mut s2 = 0usize;
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, out, ativos), || collect_club_list(Cup(this), &mut s2, ativos));
    let lo = *out;
    let list = if lo != 0 && s2 != 0 {
        core::slice::from_raw_parts(lo as *const u8, a as usize * 4) == core::slice::from_raw_parts(s2 as *const u8, b as usize * 4)
    } else {
        lo == 0 && s2 == 0
    };
    if s2 != 0 {
        exe::crt_free(s2);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("intertoto_cup_collect_club_list({:#x}, {}) returned {} x {} list {}", this, ativos as u8, a, b, list));
    a
}

/// Comparer for `intertoto_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_FOR_DATE);
    let uefa = Cup(uefa_cup_object());
    if uefa.is_null() || read_usize(this + intertoto_cup::WINNERS_SENT_TO_UEFA) != 0 {
        return o(this, today, stage);
    }
    let (v, k) = intertoto_winners_for_uefa(Cup(this), uefa);
    let n36 = uefa.entrant_count();
    let r = o(this, today, stage);
    let passed = read_usize(this + intertoto_cup::WINNERS_SENT_TO_UEFA) == 1;
    let predicted = if passed { n36.wrapping_add(k as i16) } else { n36 };
    let mut same = uefa.entrant_count() == predicted;
    if passed && same {
        for (j, &c) in v.iter().enumerate().take(k) {
            let l = uefa.entrant_book().wrapping_add(((n36 as isize) + j as isize) as usize * 6);
            same &= read_usize(l) == c && read_u8(l + 4) == 4;
        }
    }
    SCOREBOARD.record(same, || format!("intertoto_cup_update_for_date({:#x}) passed={} livro {} -> {} predicted {} vencedores {:x?}", this, passed, n36, uefa.entrant_count(), predicted, &v[..k]));
    r
}

/// Comparer for `intertoto_cup_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_INTERTOTO_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if club.is_null() {
        return o(this, club.addr(), outcome, a3, a4, game, a6);
    }
    let before = club.continental_comp_id() as usize;
    let predicted = match outcome as u8 {
        1 => read_usize(KEY_CLUB_COMP_UEFA_CUP),
        0xff | 2 => 0xffff_ffff,
        _ => before,
    };
    let r = o(this, club.addr(), outcome, a3, a4, game, a6);
    let after = club.continental_comp_id() as usize;
    SCOREBOARD.record(r == 0 && after == predicted, || format!("intertoto_cup_update_staff_records_for_status({:#x}, outcome {}) +0x1db {:x} -> {:x}, predicted {:x}, returned {}", club.addr(), outcome as u8, before, after, predicted, r));
    r
}
