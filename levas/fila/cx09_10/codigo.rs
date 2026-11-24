// ---------- cx09_10: the Belgian Third Division staff records by status, the stage advance and the title decider (bel_third.cpp) ----------

/// The staff record an outcome of the Belgian Third Division adds.
#[derive(Clone, Copy, PartialEq, Debug)]
enum BelThirdRecord {
    None,
    Promotion,
    Qualified,
    Relegation,
    Champions,
}

/// What `bel_third_update_staff_records_for_status` does for an outcome: the club's table row and the status written
/// on it (if any), the staff record and the value returned.
struct BelThirdStatusPlan {
    row: usize,
    status: Option<u8>,
    record: BelThirdRecord,
    result: i32,
}

/// The group table of a Belgian Third Division club: this league when the club's `club::RESERVE_DIVISION` is Third
/// Division A ([`KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A`]), otherwise stage 0 (group B).
unsafe fn bel_third_group_of(this: League, club: usize) -> usize {
    if read_u32(Club(club).reserve_division().addr()) == read_u32(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A) {
        this.addr()
    } else {
        read_usize(this.stages())
    }
}

/// The plan of the Belgian Third Division status update (no side effects; League::find_club_row
/// `exe::league_find_club_row`(league, club, 0) only looks). In the league and group stages (stage 0xff or 0): status
/// 0 promotion, 1 the qualified record, 3 relegation, all returning 0. Stages 1 to 6 (and any other negative byte): −1.
/// Stage 7, the title decider, on the club's group row (none: −1): status 1 → row status 5 and the champions record, 0;
/// otherwise −1. Stage 8, the promotion play-off: a club of this division (`club::DIVISION` = this competition) is
/// looked up in its group, any other club in the Second Division (the object of
/// [`KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`]); no row: −1; status 1: our club row 0 and promotion, the Second Division
/// club row 0xfe; status 2 or 0xff: our club row 0xfe, the Second Division club row 3 and relegation; all returning the
/// id of the club's division. Stage 9, the relegation play-off, only for clubs of this division (else −1), on the group
/// row (none: −1): status 1 → 0xfe; 2 or 0xff → 3 and relegation; 0. Stages above 9: 0.
unsafe fn bel_third_status_plan(this: League, club: usize, status: u8, stage: u8) -> BelThirdStatusPlan {
    let plan = |row: usize, status: Option<u8>, record: BelThirdRecord, result: i32| BelThirdStatusPlan { row, status, record, result };
    let none = plan(0, None, BelThirdRecord::None, -1);
    let own = Club(club).division().addr() == this.competition().addr();
    match stage {
        0xff | 0 => match status {
            0 => plan(0, None, BelThirdRecord::Promotion, 0),
            1 => plan(0, None, BelThirdRecord::Qualified, 0),
            3 => plan(0, None, BelThirdRecord::Relegation, 0),
            _ => plan(0, None, BelThirdRecord::None, 0),
        },
        s if (s as i8) < 7 => none,
        7 => {
            let row = exe::league_find_club_row(bel_third_group_of(this, club), club, 0);
            if row == 0 || status != 1 {
                return none;
            }
            plan(row, Some(5), BelThirdRecord::Champions, 0)
        }
        8 => {
            let league = if own { bel_third_group_of(this, club) } else { comp_object_of_key(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) };
            let row = exe::league_find_club_row(league, club, 0);
            if row == 0 {
                return none;
            }
            let id = read_i32(Club(club).division().addr());
            match status {
                1 if own => plan(row, Some(0), BelThirdRecord::Promotion, id),
                1 => plan(row, Some(0xfe), BelThirdRecord::None, id),
                2 | 0xff if own => plan(row, Some(0xfe), BelThirdRecord::None, id),
                2 | 0xff => plan(row, Some(3), BelThirdRecord::Relegation, id),
                _ => plan(row, None, BelThirdRecord::None, id),
            }
        }
        9 => {
            if !own {
                return none;
            }
            let row = exe::league_find_club_row(bel_third_group_of(this, club), club, 0);
            if row == 0 {
                return none;
            }
            match status {
                1 => plan(row, Some(0xfe), BelThirdRecord::None, 0),
                2 | 0xff => plan(row, Some(3), BelThirdRecord::Relegation, 0),
                _ => plan(row, None, BelThirdRecord::None, 0),
            }
        }
        _ => plan(0, None, BelThirdRecord::None, 0),
    }
}

/// Updates the staff records by a club's outcome in the Belgian Third Division (`bel_third_status_plan`): the row
/// status first, then the record on the Staff_Records object (`G_STAFF_RECORDS`, with this competition): promotion
/// weighs 0x64; the qualified record is (0, 0x458, 0x2d). Returns the plan's value.
/// Original: `BelThird::update_staff_records_for_status` (0x4230d0, bel_third.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bel_third_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> i32 {
    let l = League(this);
    let p = bel_third_status_plan(l, club, status as u8, stage as u8);
    if let Some(s) = p.status {
        LeagueTableRow(p.row).set_status(s);
    }
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match p.record {
        BelThirdRecord::None => {}
        BelThirdRecord::Promotion => {
            covered!("cx09_10 belgian third division promotion record");
            exe::staff_records_add_promotion_record(records, club, comp, 0x64);
        }
        BelThirdRecord::Qualified => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0, 0x458, 0x2d),
        BelThirdRecord::Relegation => exe::staff_records_add_relegation_record(records, club, comp),
        BelThirdRecord::Champions => {
            covered!("cx09_10 belgian third division title decider");
            exe::staff_records_add_league_champions_record(records, club, comp);
        }
    }
    p.result
}

/// Moves the Belgian Third Division to its next stage: past the last stage (`league::CURRENT_STAGE` at or past
/// `league::STAGE_COUNT` − 1) the bel_third.cpp box of line 0x567; otherwise once the Second Division (the object of
/// [`KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`]) is complete (its virtual `comp_vtable::CHECK_COMPLETE` with 0 and 1, the
/// low byte) the current stage goes up by one, and on reaching 7 the title decider (0x423710), the promotion play-off
/// (0x423880) and the relegation play-off (0x423ba0) are built, through their detours.
/// Original: `BelThird::advance_stage` (0x423640, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_advance_stage(this: usize) {
    let l = League(this);
    if l.current_stage() as i32 >= l.stage_count() as i32 - 1 {
        return show_error_at(BEL_THIRD_SOURCE, 0x567);
    }
    let second = comp_object_of_key(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(second) + comp_vtable::CHECK_COMPLETE));
    if complete(second, 0, 1) == 0 {
        return;
    }
    l.set_current_stage((l.current_stage() as i32 + 1) as usize);
    if l.current_stage() as i32 == 7 {
        covered!("cx09_10 belgian third division play-offs");
        for build in [0x423710usize, 0x423880, 0x423ba0] {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(build);
            f(this);
        }
    }
}

/// Builds the Belgian Third Division title decider: the leaders of group A (this league's first table row) and group B
/// (stage 0's), the round table through virtual `comp_vtable::CREATE_SCHEDULE` (7, &rounds, &extra, 0) and a Cup_Stage
/// (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage 0x522e00: this league, 2 clubs, the pair, the rounds (word),
/// the competition, the table, the year, stage 7, 1, the extra (word), 0x14, 1, 0, 0, 0) into stage slot 7. Without a
/// stage: the box of line 0x59a and nothing else (the round table is not freed: a leak of the original). With one:
/// Cup::set_up_round (`exe::cup_set_up_round`(stage, 0)), the round table freed and `league::CURRENT_STAGE` = 7.
/// Original: `BelThird::create_title_decider_stage` (0x423710, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_create_title_decider_stage(this: usize) {
    let l = League(this);
    let pair = [LeagueTableRow(l.table()).club() as usize, LeagueTableRow(League(read_usize(l.stages())).table()).club() as usize];
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 7, &mut rounds, &mut extra, 0);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this, 2, pair.as_ptr() as usize, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages() + 4 * 7, stage);
    if stage == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x59a);
    }
    covered!("cx09_10 belgian third division title decider stage");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    l.set_current_stage(7);
}

/// Comparer for `bel_third_update_staff_records_for_status`: only the original (it adds the records); the copy's plan,
/// made before, must give the same return and the same row status.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> i32 {
    let p = bel_third_status_plan(League(this), club, status as u8, stage as u8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> i32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = p.status.map_or(true, |s| LeagueTableRow(p.row).status() == s);
    SCOREBOARD.record(r == p.result && row_ok, || format!("bel_third_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): original {} copy {} {:?} row {:#x} status {:?}", this, club, status as u8, stage as u8, r, p.result, p.record, p.row, p.status));
    r
}

/// Comparer for `bel_third_advance_stage`: only the original (it builds the play-offs).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_advance_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `bel_third_create_title_decider_stage`: only the original (it builds the stage); afterwards stage 7
/// must exist and be the current one, with the two group leaders.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_create_title_decider_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let l = League(this);
    let s = read_usize(l.stages() + 4 * 7);
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 7, || format!("bel_third_create_title_decider_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}
