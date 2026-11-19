// ---------- batch %%LEVA%%: the Belgian Second Division and Eerste Divisie period results and stages (bel_second.cpp, hol_first.cpp) ----------

/// A league result in a season split into periods (the Belgian Second Division and the Eerste Divisie): the result goes
/// to the league (League::add_result `exe::league_add_result`(this, fixture, arg): thiscall, ret 8); a fixture with no
/// stage yet (`fixture::STAGE_INDEX` = 0xff) gets the period of its round (`fixture::ROUND_NUMBER`, a signed word: the
/// number of `bounds` it has reached); while the league's current stage (`league::CURRENT_STAGE`) is below that period,
/// the next period stage is created (`create_period`(this, current stage byte + 1): push -> thiscall, ret 4); then the
/// result also goes to the period stage (its virtual +4 with (fixture, arg): thiscall) with the league's statistics
/// (`league::COMP_STATS`) set aside meanwhile, and the fixture's stage goes back to 0xff. Returns 1.
pub(crate) unsafe fn add_result_by_period(this: usize, fixture: usize, arg: u32, bounds: &[i16], create_period: usize) -> u32 {
    exe::league_add_result(this, fixture, arg);
    if read_u8(fixture + fixture::STAGE_INDEX) != 0xff {
        return 1;
    }
    let round = read_i16(fixture + fixture::ROUND_NUMBER);
    write_u8(fixture + fixture::STAGE_INDEX, bounds.iter().take_while(|&&b| round >= b).count() as u8);
    let create: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(create_period);
    while read_i32(this + league::CURRENT_STAGE) < read_i8(fixture + fixture::STAGE_INDEX) as i32 {
        create(this, read_u8(this + league::CURRENT_STAGE).wrapping_add(1) as u32);
    }
    let stats = read_usize(this + league::COMP_STATS);
    write_usize(this + league::COMP_STATS, 0);
    let stage = read_usize(read_usize(this + league::STAGES) + 4 * read_i8(fixture + fixture::STAGE_INDEX) as isize as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + 4));
    add(stage, fixture, arg);
    write_usize(this + league::COMP_STATS, stats);
    write_u8(fixture + fixture::STAGE_INDEX, 0xff);
    1
}

/// Rule: a Belgian Second Division result also counts in its period: rounds 1..9 the first period, 10..21 the second,
/// 22 on the third (the period stages created as the season reaches them); `add_result_by_period` with the bounds 10
/// and 22 and BelSecond::create_period_stage (0x4204d0).
/// Original: `BelSecond::add_result` (0x420110, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_second_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    covered!("mwc_24 belgian second division result");
    add_result_by_period(this, fixture, arg, &[10, 22], 0x4204d0) // mwc24: the Belgian periods
}

/// Rule: the next stage of the Belgian Second Division: past the last stage, the error box; otherwise the current
/// stage goes up by one; on reaching stage 3 (after the three periods) the play-off group is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of bel_second.cpp at line
/// 0x26e and nothing else. Otherwise the current stage + 1; at 3, BelSecond::create_playoff_group (0x420230, thiscall,
/// ret 0, not ported).
/// Original: `BelSecond::advance_stage` (0x4201a0, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x989b54, 0x26e);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwc_24 belgian second division next stage");
    if stage == 3 {
        call_on_league(0x420230, l);
    }
}

/// Rule: an Eerste Divisie result also counts in its period: rounds 1..7 the first period, 8..15 the second, 16..25 the
/// third, 26 on the fourth; `add_result_by_period` with the bounds 8, 16 and 26 and HolFirst::create_period_stage
/// (0x5f6ad0).
/// Original: `HolFirst::add_result` (0x5f65b0, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    covered!("mwc_24 eerste divisie result");
    add_result_by_period(this, fixture, arg, &[8, 16, 26], 0x5f6ad0) // mwc24: the Dutch periods
}

/// The common comparer of the period results: only the original runs (it creates stages and adds results); for a
/// fixture with no stage, the league's current stage must reach the period of its round and the fixture's stage must
/// be 0xff again.
pub(crate) unsafe fn compare_add_result_by_period(name: &str, this: usize, fixture: usize, bounds: &[i16], original: impl FnOnce() -> u32) -> u32 {
    let open = read_u8(fixture + fixture::STAGE_INDEX) == 0xff;
    let round = read_i16(fixture + fixture::ROUND_NUMBER);
    let r = original();
    if open {
        let period = bounds.iter().take_while(|&&b| round >= b).count() as i32;
        let (stage, mark) = (read_i32(this + league::CURRENT_STAGE), read_u8(fixture + fixture::STAGE_INDEX));
        SCOREBOARD.record(r == 1 && stage >= period && mark == 0xff, || format!("{}({:#x}) round {} period {} stage {} mark {:#x} returned {}", name, this, round, period, stage, mark, r));
    }
    r
}

/// Comparer for `bel_second_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    compare_add_result_by_period("bel_second_add_result", this, fixture, &[10, 22], || o(this, fixture, arg))
}

/// Comparer for `bel_second_advance_stage`: side by side on the current stage when nothing is built; at the last stage
/// or on reaching stage 3, only the original.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 3 {
        return o(this);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + league::CURRENT_STAGE, 4)], || o(this), || bel_second_advance_stage(this));
    SCOREBOARD.record(same, || format!("bel_second_advance_stage({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `hol_first_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    compare_add_result_by_period("hol_first_add_result", this, fixture, &[8, 16, 26], || o(this, fixture, arg))
}
