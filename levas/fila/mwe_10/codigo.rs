// ---------- batch %%LEVA%%: the stage advance of the Swedish and Norwegian First Divisions and the Superettan promotion play-off (swe_first.cpp, nor_first.cpp) ----------

/// The Premier Division object of key `premier` (`G_COMP_OBJECT_TABLE` by its id) is complete: virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1) (push 1; push 0; mov ecx -> thiscall, ret 8; the byte in al).
pub(crate) unsafe fn premier_division_complete(premier: usize) -> bool {
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    complete(p, 0, 1) != 0
}

/// Brings the table positions and statuses of the Premier Division of key `premier` up to date: virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS` (mov ecx -> thiscall, ret 0).
pub(crate) unsafe fn update_premier_division_status(premier: usize) {
    let p = comp_by_global(premier);
    let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    update(p);
}

/// The stage advance of a First Division whose promotion play-off is against the Premier Division's play-off club.
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the error box of `file` at `row` and
/// nothing else. Otherwise, only when the Premier Division (key `premier`) is complete
/// (`premier_division_complete`): its positions and statuses are updated (`update_premier_division_status`), the
/// current stage goes up by one and, on reaching 0, the play-off is built (`play_off`, a direct call, thiscall, ret
/// 0). With the Premier Division still playing, nothing (the league asks again later).
pub(crate) unsafe fn advance_first_division_stage(this: League, file: usize, row: u32, premier: usize, play_off: usize) {
    if (this.current_stage() as i32) >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    if !premier_division_complete(premier) { // mwe10: waits for the Premier Division
        return;
    }
    update_premier_division_status(premier); // mwe10: the Premier Division statuses brought up to date
    let stage = (this.current_stage() as i32).wrapping_add(1);
    this.set_current_stage(stage as usize);
    if stage == 0 {
        covered!("mwe_10 first division play-off reached");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Moves the Superettan to the next stage once the Allsvenskan is complete: the Allsvenskan's statuses are updated and,
/// on reaching stage 0, the promotion play-off is built; at the last stage, the error box (line 0x1da).
///
/// `SweFirst::advance_stage()`: `advance_first_division_stage` with swe_first.cpp (0xa8b53c), line 0x1da, the key
/// `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION` and `SweFirst::create_play_off_stage` (0x88f3e0).
/// Original: `SweFirst::advance_stage` (0x88f310, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa8b53c, 0x1da, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0x88f3e0);
}

/// Moves the Norwegian First Division to the next stage once the Tippeligaen is complete: the same as the Superettan's
/// (nor_first.cpp line 0x2fa, `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, `NorFirst::create_play_off_stage` 0x790e20).
/// Original: `NorFirst::advance_stage` (0x790d50, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa69740, 0x2fa, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0x790e20);
}

/// Builds the Cup_Stage of the play-off between the two clubs of `list` into the first stage slot and frees the list
/// and the round table; returns the stage left in the slot.
///
/// The round table of stage 0 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0;
/// mov ecx -> thiscall, ret 0x10); the stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the league, 2, the list, the rounds, the competition, the round
/// table, the year, 0, 1, the extra, 0x14, 0, 0, 0, 0; 0 when out of memory) into the first slot of `.stages()`; the
/// list and the round table freed (`exe::crt_free`).
pub(crate) unsafe fn build_two_club_play_off(this: League, list: usize) -> usize {
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with garbage above the word;
    // the constructor only reads the words.
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
    read_usize(this.stages())
}

/// The club of the first table row (in table order, a signed word count) of `league` whose status is `status`.
pub(crate) unsafe fn first_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let t = league.table();
    (0..(league.club_count() as i32).max(0) as usize).map(|i| t + i * league_table_row::SIZE).find(|&r| read_u8(r + league_table_row::STATUS) == status).map(|r| read_usize(r))
}

/// The club of the first row of the Premier Division of key `premier` (through League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4) whose status is `status`.
pub(crate) unsafe fn premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    for i in 0..(p.club_count() as i32).max(0) {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            return Some(read_usize(exe::league_get_table_row(p, i)));
        }
    }
    None
}

/// Builds the Superettan promotion play-off (stage 0): the Superettan club with the promotion play-off status (the
/// 3rd) against the Allsvenskan club with the relegation play-off status (the 12th), as a two-legged cup tie.
///
/// `SweFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of
/// swe_first.cpp (0xa8b53c) line 0x206 and return); slot 0 = the club of the first Superettan row with status
/// `table_status::PROMOTION_PLAYOFF`, slot 1 = the club of the first Allsvenskan row (`G_COMP_OBJECT_TABLE` by the id
/// of `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`) with status `table_status::RELEGATION_PLAYOFF` (a slot without such a
/// row keeps what malloc gave); the stage (`build_two_club_play_off`), then Cup::set_up_round (`exe::cup_set_up_round`
/// with 0) on the first slot, called before the slot is checked (a defect of the original: with no stage it runs on
/// a null object); with no stage, the box of line 0x22e; otherwise `.current_stage()` = 0.
/// Original: `SweFirst::create_play_off_stage` (0x88f3e0, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa8b53c, 0x206);
    }
    if let Some(c) = first_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) { // mwe10: the Superettan play-off club
        write_usize(list, c);
    }
    if let Some(c) = premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) {
        write_usize(list + 4, c);
    }
    covered!("mwe_10 superettan play-off");
    let stage = build_two_club_play_off(this, list);
    exe::cup_set_up_round(stage, 0);
    if read_usize(this.stages()) == 0 {
        return show_error_at(0xa8b53c, 0x22e);
    }
    this.set_current_stage(0);
}

/// The comparer of the two stage advances: only the original runs (it updates the Premier Division and may build the
/// play-off); the current stage must stay (at the last stage, or with the Premier Division still playing) or go up by
/// one, and on reaching 0 the first stage slot must hold a stage.
pub(crate) unsafe fn compare_first_division_advance(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    let ok = if last { after == before } else { (after == before || after == before + 1) && (after != 0 || before == 0 || read_usize(this.stages()) != 0) };
    SCOREBOARD.record(ok, || format!("{}({:#x}) before={} after={} stages={}", name, this.addr(), before, after, this.stage_count()));
}

/// Comparer for `swe_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_first_division_advance("swe_first_advance_stage", League(this), || o(this))
}

/// Comparer for `nor_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_first_division_advance("nor_first_advance_stage", League(this), || o(this))
}

/// Comparer for `swe_first_create_play_off_stage`: only the original runs (it allocates and draws); the plan (the two
/// clubs, made beforehand) against the stage in the first slot: two clubs, the same as the plan (as a set), the
/// competition and `.current_stage()` = 0.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let l = League(this);
    let plan = [first_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    compare_two_club_play_off("swe_first_create_play_off_stage", l, plan);
}

/// Checks the two-club play-off stage of the first slot against the plan (as a set, the round draw may swap them).
pub(crate) unsafe fn compare_two_club_play_off(name: &str, this: League, plan: [Option<usize>; 2]) {
    let stage = Cup(read_usize(this.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("{}({:#x}) without stage", name, this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found = [read_usize(list), read_usize(list + 6)];
    let mut want = [plan[0].unwrap_or(0), plan[1].unwrap_or(0)];
    found.sort_unstable();
    want.sort_unstable();
    let ok = plan.iter().all(|c| c.is_some()) && found == want && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && this.current_stage() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}) plan={:x?} stage={:#x} list={:x?} +0x3e={}", name, this.addr(), plan, stage.addr(), found, stage.team_count()));
}
