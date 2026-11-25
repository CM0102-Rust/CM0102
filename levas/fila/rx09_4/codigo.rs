// ---------- rx09_4: the Spanish Second Division B relegation final, daily update and staff records (spa_second_b.cpp) ----------

/// The two clubs of the relegation play-off final: the first two entries of stage 7's team table (`cup::TEAM_TABLE`)
/// whose status (`cup_team_entry::STATUS`) is 2, scanning while the team count (re-read) allows.
pub(crate) unsafe fn segunda_b_relegation_final_clubs(l: League) -> Vec<usize> {
    let s7 = Cup(read_usize(l.stages() + 4 * 7));
    let mut v = Vec::new();
    let mut i = 0i16;
    while i < s7.team_count() && v.len() < 2 {
        let e = s7.team_table() + 6 * i as usize;
        if read_u8(e + cup_team_entry::STATUS) == 2 { // rx09_4: the losers of the relegation play-off
            v.push(read_usize(e + cup_team_entry::CLUB));
        }
        i += 1;
    }
    v
}

/// Builds the relegation play-off final of the Second Division B as stage 8: the two clubs of
/// `segunda_b_relegation_final_clubs` in a local list of two, the round table of stage 8 (`stage_round_table`), the
/// Cup_Stage (`league_cup_stage`: two clubs, stage 8, 1) into slot 8; none: the box at line 0x371 (the round table not
/// freed). Otherwise the round table freed, its round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)) and
/// the current stage = 8. With fewer than two losers the original passes stack leftovers in the empty slots and the
/// copy zeros (never seen: the two ties have two losers).
/// Original: `SpaSecondB::create_relegation_playoff_final` (0x857a00, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_create_relegation_playoff_final(this: usize) {
    let l = League(this);
    let mut list = [0usize; 2];
    for (k, c) in segunda_b_relegation_final_clubs(l).into_iter().enumerate() {
        list[k] = c;
    }
    let (table, rounds, extra) = stage_round_table(l, 8);
    let p = league_cup_stage(l, 2, list.as_ptr() as usize, table, rounds, extra, 8, 1, 0);
    write_usize(l.stages() + 4 * 8, p);
    if read_usize(l.stages() + 4 * 8) == 0 {
        return show_error_at(SEGUNDA_B_SOURCE, 0x371);
    }
    covered!("rx09_4 segunda b relegation final");
    exe::crt_free(table);
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 8), 0);
    l.set_current_stage(8);
}

/// Comparer for `spa_second_b_create_relegation_playoff_final`: only the original runs (it allocates the stage); with
/// two losers, the stage's clubs and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_create_relegation_playoff_final(this: usize) {
    let l = League(this);
    let plan = segunda_b_relegation_final_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    if plan.len() == 2 {
        compare_league_cup_stage("spa_second_b_create_relegation_playoff_final", l, 8, &plan);
    }
}

/// True when the Second Division B is at stage 7 (`league::CURRENT_STAGE`) and its relegation play-off is complete
/// (the stage's virtual `comp_vtable::CHECK_COMPLETE`(0, 0), the byte of its answer).
pub(crate) unsafe fn segunda_b_relegation_play_off_done(l: League) -> bool {
    if l.current_stage() as i32 != 7 {
        return false;
    }
    let s7 = read_usize(l.stages() + 4 * 7);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(s7) + comp_vtable::CHECK_COMPLETE));
    complete(s7, 0, 0) as u8 != 0
}

/// The day of the Spanish Second Division B: when the relegation play-off is over (`segunda_b_relegation_play_off_done`)
/// the league moves to its next stage (its virtual `comp_vtable::ADVANCE_STAGE`: the final); then the league day
/// (League::update_for_date `exe::league_update_for_date`, the arguments as they came), whose answer it returns.
/// Original: `SpaSecondB::update_for_date` (0x858160, spa_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn spa_second_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    if segunda_b_relegation_play_off_done(l) {
        covered!("rx09_4 segunda b relegation play-off over");
        bra_nat_first_call_virtual_no_args(this, comp_vtable::ADVANCE_STAGE);
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Comparer for `spa_second_b_update_for_date`: only the original runs (it builds the final and plays the day); when
/// the relegation play-off was over the league must be at stage 8 afterwards.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let due = segunda_b_relegation_play_off_done(l);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, today, full);
    SCOREBOARD.record(!due || l.current_stage() as i32 == 8, || format!("spa_second_b_update_for_date({:#x}) final due, stage {}", this, l.current_stage() as i32));
    r
}

/// The staff record a Second Division B status adds (`spa_second_b_update_staff_records_for_status`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SegundaBRecord {
    /// Staff_Records::add_qualified_for_round_record with 0x438, 0x41c, 0x28 (through to the promotion play-off).
    Qualified,
    /// Staff_Records::add_promotion_record with 0x64.
    Promotion,
    /// Staff_Records::add_relegation_record.
    Relegation,
}

/// What `spa_second_b_update_staff_records_for_status` does for a club, worked out without side effects: the row of
/// its group and the status written there, and the record added (the result is always 0).
#[derive(Debug)]
pub(crate) struct SegundaBRecordPlan {
    pub(crate) row: Option<(usize, u8)>,
    pub(crate) record: Option<SegundaBRecord>,
}

/// The group of a Second Division B club by the id of its reserve division (the group): B1 the league's own table,
/// B2 to B4 the group stages 0 to 2. Defect of the original: for any other id the status argument itself is taken as
/// the group (its fields are then read from that address, an access violation); the copy does the same.
pub(crate) unsafe fn segunda_b_group_of(l: League, reserve: usize, status: u32) -> League {
    match SEGUNDA_B_GROUPS.iter().position(|&g| read_usize(reserve) == read_usize(g)) {
        Some(k) => segunda_b_group(l, k as u8),
        None => League(status as usize),
    }
}

/// The address of the club's row in a group's table, if it has one (rows 0 to `league::CLUB_COUNT` − 1).
pub(crate) unsafe fn league_row_of(group: League, club: Club) -> Option<usize> {
    let mut i = 0i16;
    while i < group.club_count() {
        let row = group.table() + i as usize * league_table_row::SIZE;
        if read_usize(row) == club.addr() {
            return Some(row);
        }
        i += 1;
    }
    None
}

/// The plan of `spa_second_b_update_staff_records_for_status` for a club with `status` (its byte) at `stage` (signed):
/// - the groups (below 3): status 1 adds the qualified record;
/// - the promotion play-off groups (3 to 6): with a reserve division, the club's row in its group
///   (`segunda_b_group_of`, `league_row_of`; none: nothing); status 0 writes 0 in the row with the promotion record,
///   any other status writes 0xfe;
/// - the relegation play-off (7): status 1 writes 0xfe in the club's row;
/// - its final (8): with a reserve division, status 1 writes 0xfe in the row; any other status writes 3 in the row
///   (when found) and, for status 2, adds the relegation record (found or not);
/// - later stages: nothing.
pub(crate) unsafe fn segunda_b_record_plan(l: League, club: Club, status: u32, stage: i8) -> SegundaBRecordPlan {
    let mut plan = SegundaBRecordPlan { row: None, record: None };
    let s = status as u8;
    if stage < 3 {
        if s == 1 {
            plan.record = Some(SegundaBRecord::Qualified);
        }
        return plan;
    }
    if stage > 8 || (stage == 7 && s != 1) {
        return plan;
    }
    let reserve = club.reserve_division().addr();
    if reserve == 0 {
        return plan;
    }
    let row = league_row_of(segunda_b_group_of(l, reserve, status), club);
    match stage {
        3..=6 => {
            if let Some(row) = row {
                if s != 0 {
                    plan.row = Some((row, 0xfe));
                } else {
                    plan.row = Some((row, 0)); // rx09_4: the play-off group winner's row
                    plan.record = Some(SegundaBRecord::Promotion);
                }
            }
        }
        7 => plan.row = row.map(|r| (r, 0xfe)),
        _ => {
            if s == 1 {
                plan.row = row.map(|r| (r, 0xfe));
            } else {
                plan.row = row.map(|r| (r, 3));
                if s == 2 {
                    plan.record = Some(SegundaBRecord::Relegation);
                }
            }
        }
    }
    plan
}

/// Updates the staff records by a club's status in the Spanish Second Division B and marks its row in its group
/// (`segunda_b_record_plan`: the row status, then the record; the competition is this league's,
/// `league::COMPETITION`). The last three arguments are not read; the result is 0.
/// Original: `SpaSecondB::update_staff_records_for_status` (0x857b70, spa_second_b.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn spa_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = segunda_b_record_plan(l, Club(club), status, stage as u8 as i8);
    if let Some((row, s)) = plan.row {
        covered!("rx09_4 segunda b row status");
        write_u8(row + league_table_row::STATUS, s);
    }
    let records = read_usize(G_STAFF_RECORDS);
    let comp = l.competition().addr();
    match plan.record {
        Some(SegundaBRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x438, 0x41c, 0x28),
        Some(SegundaBRecord::Promotion) => exe::staff_records_add_promotion_record(records, club, comp, 0x64),
        Some(SegundaBRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    0
}

/// Comparer for `spa_second_b_update_staff_records_for_status`: only the original runs (it adds records); its result
/// (0) and the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = segunda_b_record_plan(League(this), Club(club), status, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I2%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == 0 && row_ok, || format!("spa_second_b_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}
