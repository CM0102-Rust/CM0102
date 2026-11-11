use super::*;

// The comparers of the routines in ../comp/leagues/eng_second.rs.

/// Comparer for `eng_second_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ENG_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("eng_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| eng_second_create_schedule(this, stage, r, e, l))
}

/// Comparer for `eng_second_update_staff_records_for_status` (`eng_second_update_staff_records_for_status`; comparison mode: see the header of motor.rs).
///
/// The records write the history to the file, draw the morale and may renew contracts (they do not run twice): only the
/// original runs. The plan, made beforehand, in the knockout: the club's first row decides what the routine writes
/// (+0x37 = 0 and saiu = 1 with 1; +0x37 = 0xfe with 0xff or 2; nothing with 0, which keeps looking for another row of
/// the same club); afterwards the row and `saiu` must be like that. In the league the routine writes nothing: only the
/// return value is checked.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, left: usize, a: u32, b: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_ENG_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let s = status as u8;
    let mut plan: Option<(usize, u8, bool)> = None;
    if stage as u8 != 0xff {
        let table = this.table();
        let n = this.club_count() as i32;
        let mut i: i32 = 0;
        while i < n {
            let row = table + (i as usize) * 0x3b;
            if read_usize(row) == club {
                match s {
                    1 => {
                        plan = Some((row, 0, true));
                        break;
                    }
                    0xff | 2 => {
                        plan = Some((row, 0xfe, false));
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
    let r = o(this.addr(), club, status, stage, left, a, b);
    let same = r == 0
        && match plan {
            Some((row, v, marca)) => read_u8(row + league_table_row::STATUS) == v && (!marca || read_u8(left) == 1),
            None => true,
        };
    SCOREBOARD.record(same, || format!("eng_second_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plan={:x?}", this.addr(), club, s, stage as u8, r, plan));
    r
}

/// Comparer for `eng_second_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_ADVANCE_STAGE);
    compare_stage_advance("eng_second_advance_stage", League(this), || o(this), || eng_second_advance_stage(this))
}

/// Comparer for `eng_second_create_play_off_stage`.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_SECOND_CREATE_PLAY_OFF_STAGE);
    compare_playoff_stage("eng_second_create_play_off_stage", League(this), [5, 2, 4, 3], || o(this))
}

/// Comparer for `construct_eng_second`.
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_SECOND);
    compare_english_constructor("construct_eng_second", League(this), year, record, &ENG_SECOND_DIVISION, || o(this, year, record))
}

/// Comparer for `eng_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("eng_second_set_up_rules", League(this), eng_prm_second_division_rules(), || o(this), || eng_second_set_up_rules(this))
}

/// Comparer for `eng_second_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_eng_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("eng_second_create_table", League(this), 0x18, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, 0x4e20, || o(this))
}
