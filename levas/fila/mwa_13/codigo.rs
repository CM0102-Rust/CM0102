// ---------- batch %%LEVA%%: the Northern Irish First Division: the season turnover, the next stage and the promotion play-off (nir_first.cpp) ----------

/// Rule: the Northern Irish First Division turnover: the statistics archived, the table and the round table cleared,
/// the year moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the
/// board expectations and the referee renewal.
///
/// BraNatSecond::start_new_season's template (checked instruction by instruction: only the table routine,
/// NirFirst::create_table 0x78bc50, the file and the error box lines 0x1f5, 0x1fc, 0x202 and 0x209 change):
/// `league::SEASON_CLOSED` = 0, `rebuild_season` and, without a box, `set_expectations_and_renew_officials` (whose
/// return it passes on; 0 after a box).
/// Original: `NirFirst::start_new_season` (0x78be10, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x78bc50, 0xa6965c, [0x1f5, 0x1fc, 0x202, 0x209], 1) { // mwa13: the next season of the Northern Irish First
        return 0;
    }
    covered!("mwa_13 northern irish first turnover");
    set_expectations_and_renew_officials(this)
}

unsafe extern "thiscall" fn compare_nir_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("nir_first_start_new_season", League(this), || o(this))
}

/// The next stage of an Irish First Division (`advance_stage`, the comp virtual): at the last stage
/// (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the error box and nothing else. Otherwise it waits for
/// the Premier Division above (the competition object of the key global `premier`, `comp_by_global`): when its
/// `comp_vtable::CHECK_COMPLETE`(0, 1) (thiscall, ret 8, the low byte) says it is over, the Premier's positions and
/// status are updated (`league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0), the current stage goes up by one
/// and, on reaching 0, the promotion play-off is built (`play_off`, thiscall, ret 0).
pub(crate) unsafe fn advance_irish_first_stage(this: League, premier: usize, file: usize, row: u32, play_off: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    if complete(p, 0, 1) as u8 == 0 {
        return;
    }
    covered!("mwa_13 irish first division waits for the premier");
    bra_nat_first_call_virtual_no_args(p, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    let stage = this.current_stage() as i32 + 1;
    this.set_current_stage(stage as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Rule: the Northern Irish First Division's league stage ends only when the Premier Division above has finished: then
/// the Premier's positions are updated and the promotion play-off is built; past the last stage, the error box.
///
/// `advance_irish_first_stage` with the Northern Irish Premier Division (`KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`),
/// the error box at line 0x226 of nir_first.cpp and NirFirst::create_playoff_stage (0x78c0a0).
/// Original: `NirFirst::advance_stage` (0x78bfd0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_advance_stage(this: usize) {
    advance_irish_first_stage(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, 0x226, 0x78c0a0) // mwa13: the Northern Irish play-off waits for the Premier
}

/// The comparer of the Irish First Divisions' next stage: only the original runs (it may build the play-off); the
/// stage either stays or goes up by one, and never past the last stage.
pub(crate) unsafe fn compare_irish_first_advance_stage(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == before || (!last && after == before + 1), || format!("{}({:#x}) before={} after={} last={}", name, this.addr(), before, after, last));
}

unsafe extern "thiscall" fn compare_nir_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_irish_first_advance_stage("nir_first_advance_stage", League(this), || o(this))
}

/// The two clubs of an Irish promotion play-off: the First Division club whose table row has status 1 (the play-off
/// place; the last such row) and the club of the Premier Division row with status 2 (League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4; the last such row), or None for each one missing.
pub(crate) unsafe fn irish_play_off_clubs(this: League, premier: usize) -> (Option<usize>, Option<usize>) {
    let mut first = None;
    for k in 0..this.club_count().max(0) as usize {
        let row = this.table() + k * 0x3b;
        if read_u8(row + 0x37) == 1 {
            first = Some(read_usize(row));
        }
    }
    let p = League(comp_by_global(premier));
    let mut second = None;
    let mut k = 0i32;
    while k < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, k) + 0x37) == 2 {
            second = Some(read_usize(exe::league_get_table_row(p, k)));
        }
        k += 1;
    }
    (first, second)
}

/// The promotion play-off of an Irish First Division: the First Division's play-off club against the Premier Division's
/// play-off club, in a two-club Cup_Stage in the first stage slot.
///
/// The list of 2 (`exe::crt_malloc` of 8; out of memory, the box `rows[0]` and nothing else); entry 0 = the First
/// Division club (`irish_play_off_clubs`; without one, the box `rows[1]` and return: the original leaves the list
/// allocated), entry 1 = the Premier Division club (without one, return with nothing else, the list left allocated).
/// The round table of stage 0 (the league's virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, null)); the
/// stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the league, 2, the list, the rounds, the competition record, the table, the year, 0, 1, the extra, 0x14, 0, 0,
/// 0, 0) into the first stage slot; the list and the table freed; without the stage, the box `rows[2]`; otherwise
/// Cup::set_up_round(stage, 0) (`exe::cup_set_up_round`, thiscall, ret 4).
pub(crate) unsafe fn create_irish_promotion_play_off(this: League, premier: usize, file: usize, rows: [u32; 3], branch: &'static str) {
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(file, rows[0]);
    }
    let (first, second) = irish_play_off_clubs(this, premier);
    match first {
        Some(c) => write_usize(list, c),
        None => return show_error_at(file, rows[1]),
    }
    match second {
        Some(c) => write_usize(list + 4, c),
        None => return,
    }
    covered!(branch);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year with whatever is in the registers' high
    // halves; the constructor only reads the words
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, rows[2]);
    }
    exe::cup_set_up_round(stage, 0);
}

/// The comparer of the Irish promotion play-offs: only the original runs (it allocates and draws); the plan, made
/// beforehand: the two clubs (`irish_play_off_clubs`). With both, the first stage slot must hold a stage of two clubs
/// of the league's competition with those two (as a set).
pub(crate) unsafe fn compare_irish_promotion_play_off(name: &str, this: League, premier: usize, original: impl FnOnce()) {
    let plan = irish_play_off_clubs(this, premier);
    original();
    if let (Some(a), Some(b)) = plan {
        let stage = Cup(read_usize(this.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && {
            let (x, y) = (read_usize(stage.team_table()), read_usize(stage.team_table() + 6));
            (x == a && y == b) || (x == b && y == a)
        };
        SCOREBOARD.record(ok, || format!("{}({:#x}) plan {:#x} v {:#x}, stage {:#x}", name, this.addr(), a, b, stage.addr()));
    }
}

/// Rule: the Northern Irish promotion play-off: the First Division runner-up against the Premier Division's second last
/// (the clubs with the play-off status in each table), in a two-club stage.
///
/// `create_irish_promotion_play_off` with the Northern Irish Premier Division and the error box lines 0x252, 0x262 and
/// 0x283 of nir_first.cpp.
/// Original: `NirFirst::create_playoff_stage` (0x78c0a0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_playoff_stage(this: usize) {
    create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, [0x252, 0x262, 0x283], "mwa_13 northern irish promotion play-off")
}

unsafe extern "thiscall" fn compare_nir_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_irish_promotion_play_off("nir_first_create_playoff_stage", League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, || o(this))
}
