// ---------- cx09_14: the Finnish First Division staff records by outcome, predicted positions and season honours (fin_first.cpp) ----------

/// The fin_first.cpp source path, for the error boxes.
const FIN_FIRST_SOURCE: usize = 0x9c818c;

/// The staff record an outcome of the Finnish First Division adds.
#[derive(Clone, Copy, PartialEq, Debug)]
enum FinFirstRecord {
    None,
    /// Qualified for the play-offs (0x458, 0x82, 0x28).
    Qualified,
    Champions,
    /// Relegation from the given competition record.
    Relegation(usize),
    Promotion,
}

/// What `fin_first_update_staff_records_for_status` does for an outcome: the record first, then the row status (if
/// any), and the value returned.
struct FinFirstStatusPlan {
    record: FinFirstRecord,
    row: usize,
    status: Option<u8>,
    result: i32,
}

/// The club's table row in the North group (this league) or else the South group (stage 0), by League::find_club_row
/// (`exe::league_find_club_row`(league, club, 0)); 0 when in neither.
unsafe fn fin_first_group_row(l: League, club: usize) -> usize {
    let row = exe::league_find_club_row(l, club, 0);
    if row != 0 {
        row
    } else {
        exe::league_find_club_row(read_usize(l.stages()), club, 0)
    }
}

/// The plan of the Finnish First Division status update (no side effects). The stage is a signed byte. Below stage 1
/// (the groups): status 6 is the play-off qualification, 0; any other −1. Stage 1 (the promotion play-off): status 1 is
/// the champions record and the club's group row (if any) status 5; 0. Stage 2 (the relegation play-off): status 2 is
/// the group row (if any) status 3 and relegation; 0. Stage 3 (the play-off against the Veikkausliiga): a club of the
/// Veikkausliiga (its `club::DIVISION` id is [`KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`]) found in that league's table
/// (scanned row by row; none: 0) stays up with status 1 (row 0xfe) or goes down (relegation from the Veikkausliiga
/// record, row 3), returning the Veikkausliiga id; any other club, in its group row (none: 0), goes up with status 1
/// (promotion weighing 0x32, row 0) or stays (row 0xfe), returning the First Division id
/// ([`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`]). Other stages, or another status where none is named: 0.
unsafe fn fin_first_status_plan(l: League, club: usize, status: u8, stage: u8) -> FinFirstStatusPlan {
    let plan = |record: FinFirstRecord, row: usize, status: Option<u8>, result: i32| FinFirstStatusPlan { record, row, status, result };
    let nothing = plan(FinFirstRecord::None, 0, None, 0);
    match stage as i8 {
        s if s < 1 => {
            if status == 6 {
                plan(FinFirstRecord::Qualified, 0, None, 0)
            } else {
                plan(FinFirstRecord::None, 0, None, -1)
            }
        }
        1 if status == 1 => {
            let row = fin_first_group_row(l, club);
            plan(FinFirstRecord::Champions, row, (row != 0).then_some(5), 0)
        }
        2 if status == 2 => {
            let row = fin_first_group_row(l, club);
            plan(FinFirstRecord::Relegation(l.competition().addr()), row, (row != 0).then_some(3), 0)
        }
        3 => {
            let premier_id = read_i32(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
            if read_i32(Club(club).division().addr()) == premier_id {
                let premier = League(comp_object_of_key(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION));
                let Some(k) = (0..premier.club_count() as i32).find(|&k| LeagueTableRow(premier.table() + k as usize * league_table_row::SIZE).club() as usize == club) else {
                    return nothing;
                };
                let row = premier.table() + k as usize * league_table_row::SIZE;
                if status == 1 {
                    plan(FinFirstRecord::None, row, Some(0xfe), premier_id)
                } else {
                    let record = read_usize(G_CLUB_COMP_TABLE) + premier_id as usize * 0x6b; // the Veikkausliiga competition record
                    plan(FinFirstRecord::Relegation(record), row, Some(3), premier_id)
                }
            } else {
                let row = fin_first_group_row(l, club);
                if row == 0 {
                    return nothing;
                }
                let id = read_i32(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
                if status == 1 {
                    plan(FinFirstRecord::Promotion, row, Some(0), id)
                } else {
                    plan(FinFirstRecord::None, row, Some(0xfe), id)
                }
            }
        }
        _ => nothing,
    }
}

/// Updates the staff records by a club's outcome in the Finnish First Division (`fin_first_status_plan`): the record on
/// the Staff_Records object (`G_STAFF_RECORDS`) first, with this competition unless named, then the row status.
/// Returns the plan's value.
/// Original: `FinFirst::update_staff_records_for_status` (0x594810, fin_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn fin_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> i32 {
    let l = League(this);
    let p = fin_first_status_plan(l, club, status as u8, stage as u8);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match p.record {
        FinFirstRecord::None => {}
        FinFirstRecord::Qualified => {
            covered!("cx09_14 finnish play-off place");
            exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x458, 0x82, 0x28);
        }
        FinFirstRecord::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
        FinFirstRecord::Relegation(from) => exe::staff_records_add_relegation_record(records, club, from),
        FinFirstRecord::Promotion => {
            covered!("cx09_14 finnish promotion to the veikkausliiga");
            exe::staff_records_add_promotion_record(records, club, comp, 0x32);
        }
    }
    if let Some(s) = p.status {
        LeagueTableRow(p.row).set_status(s);
    }
    p.result
}

/// Gives the Finnish First Division clubs their predicted positions: a list of 10 (malloc of 0x28; without statistics
/// `league::COMP_STATS` the fin_first.cpp box of line 0x593, without memory the box of line 0x59b), then
/// `two_group_predicted_positions` with each group's own club count and 2k + 1, stored through
/// Comp_Stats::set_club_final_position (`exe::comp_stats_set_club_final_position`); the list freed.
/// Original: `FinFirst::set_predicted_positions` (0x594ff0, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x593);
    }
    let list = exe::crt_malloc(0x28);
    if list == 0 {
        return show_error_at(FIN_FIRST_SOURCE, 0x59b);
    }
    covered!("cx09_14 finnish predicted positions");
    two_group_predicted_positions(l, list, true, 2, |club, position| {
        exe::comp_stats_set_club_final_position(l.comp_stats(), club, position as u32);
    });
    exe::crt_free(list);
}

/// Records the Finnish First Division's season honours through its second stage (the promotion play-off, stage 1): that
/// stage's virtual `comp_vtable::RECORD_SEASON_HONOURS` (a tail jump in the original), whose value is returned.
/// Original: `FinFirst::record_season_honours` (0x595160, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_record_season_honours(this: usize) -> usize {
    covered!("cx09_14 finnish season honours");
    let stage = read_usize(League(this).stages() + 4);
    let honours: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    honours(stage)
}

/// Comparer for `fin_first_update_staff_records_for_status`: only the original (it adds the records); the copy's plan,
/// made before, must give the same return and the same row status.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> i32 {
    let p = fin_first_status_plan(League(this), club, status as u8, stage as u8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> i32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = p.status.map_or(true, |s| LeagueTableRow(p.row).status() == s);
    SCOREBOARD.record(r == p.result && row_ok, || format!("fin_first_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): original {} copy {} {:?} row {:#x} status {:?}", this, club, status as u8, stage as u8, r, p.result, p.record, p.row, p.status));
    r
}

/// Comparer for `fin_first_set_predicted_positions`: only the original (it writes the statistics); afterwards each club's
/// `comp_stats_club::FINAL_POSITION` must be the copy's (its sort on a local list).
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let l = League(this);
    if l.comp_stats() == 0 {
        return;
    }
    let most = l.club_count().max(League(read_usize(l.stages())).club_count()).max(0) as usize;
    let mut list = vec![0usize; most];
    let mut plan = Vec::new();
    two_group_predicted_positions(l, list.as_mut_ptr() as usize, true, 2, |c, p| plan.push((c, p)));
    let wrong: Vec<(usize, u8, u8)> = plan.iter().filter_map(|&(c, p)| {
        let e = exe::comp_stats_find_club_entry(l.comp_stats(), c);
        let got = if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 };
        (got != p).then_some((c, p, got))
    }).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("fin_first_set_predicted_positions({:#x}): (club, copy, original) {:x?}", this, wrong));
}

/// Comparer for `fin_first_record_season_honours`: only the original (it writes the history).
pub(crate) unsafe extern "thiscall" fn compare_fin_first_record_season_honours(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this)
}
