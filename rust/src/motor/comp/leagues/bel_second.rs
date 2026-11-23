use super::*;

/// Rule: the Belgian Second Division turnover (called by the First Division's): the same template as the Eerste
/// Divisie (`rebuild_season` with BelSecond::create_table at 0x41fd70 and the boxes of bel_second.cpp at lines 0x20d,
/// 0x214, 0x21a, 0x221; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `BelSecond::start_new_season` (0x41ff50, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x41fd70, 0x989b54, [0x20d, 0x214, 0x21a, 0x221], 1) {
        return 0;
    }
    covered!("mwc_19 belgian second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `bel_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("bel_second_start_new_season", League(this), || o(this))
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

/// Comparer for `bel_second_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BEL_SECOND_ADD_RESULT);
    compare_add_result_by_period("bel_second_add_result", this, fixture, &[10, 22], || o(this, fixture, arg))
}

/// Comparer for `bel_second_advance_stage`: side by side on the current stage when nothing is built; at the last stage
/// or on reaching stage 3, only the original.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_advance_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BEL_SECOND_ADVANCE_STAGE);
    let l = this;
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 3 {
        return o(this.addr());
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], || o(this.addr()), || bel_second_advance_stage(this.addr()));
    SCOREBOARD.record(same, || format!("bel_second_advance_stage({:#x}) original={:x?} rust={:x?}", this.addr(), mo, mr));
}

/// The Belgian Second Division: discipline rule 4 (`BELGIUM_LEAGUE`), the bench (4 substitutes named, 3 substitutions),
/// 4 stages besides the league (the stage vector, malloc of 16); with no saved state, the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `BelSecond::BelSecond` (0x41f030, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &BEL_SECOND_DIVISION, || {})
}

/// Comparer for `construct_bel_second`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_bel_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_SECOND);
    compare_nation_division_ctor("construct_bel_second", League(this), year, record, &BEL_SECOND_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Belgian Second Division: 2 rounds, 3 points for a win and 1 for a draw; 1 go up, 0 to the
/// promotion play-off, 1 to the relegation play-off and 2 go down; above: `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`;
/// below: `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`; tie-breaker codes 3, 1, 2, 0 (`+0xc5`..); zeroes the class fields
/// `+0xf6`, `+0xf7`, `+0xf8` [inference: the period winners]; rule bits `+0xd9` = 0x43; then builds the main-stage
/// round table through virtual +0x3c into `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `BelSecond::set_up_rules` (0x41fea0, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_set_up_rules(this: usize) -> u32 {
    covered!("mwc_4 bel second rules");
    fra_first_apply_league_rules(League(this), &bel_second_rules(this))
}

/// Comparer for `bel_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_SET_UP_RULES);
    compare_nation_league_rules("bel_second_set_up_rules", League(this), bel_second_rules(this), || o(this), || bel_second_set_up_rules(this))
}

/// The Belgian Second Division has 18 clubs; prize fund by the share of 10,000 per place; TV pays 40,000 to each club
/// (before the fund).
///
/// `BelSecond::create_table()`: `+0x3e` = 18, the table (`exe::crt_malloc` of 18 x 0x3b; out of memory, the error box
/// of line 0x1a4 and 0) with the clubs of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the TV money (League::pay_tv_rights, 0x9c40 to every club), the fund
/// `+0x28` (League::calc_prize_fund, share 0x2710), the prize mode `+0xe1` = 1 and 1.
/// Original: `BelSecond::create_table` (0x41fd70, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    if fra_first_fill_league_table(this, 0x989b54, 0x1a4, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwc_4 full table");
    eng_prm_pay_tv_rights(this.addr(), 0x9c40, 0); // mwc4: the TV money of each club
    let fund = exe::league_calc_prize_fund(this, 0x2710);
    this.set_prize_fund(fund);
    this.set_prize_mode(1);
    1
}

/// Comparer for `bel_second_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_CREATE_TABLE);
    compare_nation_league_table("bel_second_create_table", League(this), 0x12, KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION, Some(0x2710), || o(this))
}

/// A league result in a season split into periods (the Belgian Second Division and the Eerste Divisie): the result goes
/// to the league (League::add_result `exe::league_add_result`(this, fixture, arg): thiscall, ret 8); a fixture with no
/// stage yet (`fixture::STAGE_INDEX` = 0xff) gets the period of its round (`fixture::ROUND_NUMBER`, a signed word: the
/// number of `bounds` it has reached); while the league's current stage (`league::CURRENT_STAGE`) is below that period,
/// the next period stage is created (`create_period`(this, current stage byte + 1): push -> thiscall, ret 4); then the
/// result also goes to the period stage (its virtual +4 with (fixture, arg): thiscall) with the league's statistics
/// (`league::COMP_STATS`) set aside meanwhile, and the fixture's stage goes back to 0xff. Returns 1.
pub(crate) unsafe fn add_result_by_period(this: usize, fixture: usize, arg: u32, bounds: &[i16], create_period: usize) -> u32 {
    let this = League(this);
    let fixture = Fixture(fixture);
    exe::league_add_result(this, fixture, arg);
    if fixture.stage_index() != 0xff {
        return 1;
    }
    let round = fixture.round_number();
    fixture.set_stage_index(bounds.iter().take_while(|&&b| round >= b).count() as u8);
    let create: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(create_period);
    while (this.current_stage() as i32) < (fixture.stage_index() as i8) as i32 {
        create(this.addr(), (this.current_stage() as u8).wrapping_add(1) as u32);
    }
    let stats = this.comp_stats();
    this.set_comp_stats(0);
    let stage = read_usize(this.stages() + 4 * (fixture.stage_index() as i8) as isize as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + 4));
    add(stage, fixture.addr(), arg);
    this.set_comp_stats(stats);
    fixture.set_stage_index(0xff);
    1
}

/// The common comparer of the period results: only the original runs (it creates stages and adds results); for a
/// fixture with no stage, the league's current stage must reach the period of its round and the fixture's stage must
/// be 0xff again.
pub(crate) unsafe fn compare_add_result_by_period(name: &str, this: usize, fixture: usize, bounds: &[i16], original: impl FnOnce() -> u32) -> u32 {
    let this = League(this);
    let fixture = Fixture(fixture);
    let open = fixture.stage_index() == 0xff;
    let round = fixture.round_number();
    let r = original();
    if open {
        let period = bounds.iter().take_while(|&&b| round >= b).count() as i32;
        let (stage, mark) = ((this.current_stage() as i32), fixture.stage_index());
        SCOREBOARD.record(r == 1 && stage >= period && mark == 0xff, || format!("{}({:#x}) round {} period {} stage {} mark {:#x} returned {}", name, this.addr(), round, period, stage, mark, r));
    }
    r
}

/// The rule writes of the Belgian Second Division (offset, size, value).
pub(crate) unsafe fn bel_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0xc5, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 1), (0xc1, 1, 2), (0x1c, 4, read_usize(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION) as u32), (0xf8, 1, 0), (0xf7, 1, 0), (0xf6, 1, 0), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION) as u32), (0xd9, 2, 0x43)];
    v.push((0xc2, 1, 3)); // mwc4: the Belgian Second Division points for a win
    v
}

pub(crate) const BEL_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x9679f4,
    bench: Some((4, 3)),
    discipline: discipline_rule::BELGIUM_LEAGUE, // mwc4: the Belgian Second Division discipline rule
    stages: 4,
    set_up_rules: 0x41fea0,
    create_table: 0x41fd70,
    source_file: 0x989b54,
    box_lines: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
