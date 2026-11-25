// ---------- rx10_1: the Football Conference constructor, rules and table (eng_conf.cpp) ----------

/// The Conference constructor plan, for the comparer (`compare_english_constructor`): vtable 0x969a74, a minimum
/// ground of 1,000 places with 600 seated, no extra stage, EngConf::set_up_rules 0x56ede0 and EngConf::create_table
/// 0x56ecb0, the boxes of eng_conf.cpp at lines 0x4c, 0x53, 0x5a, 0x63 and 0x6b.
pub(crate) const CONFERENCE_DIVISION: EnglishDivision = EnglishDivision {
    vtable: 0x969a74,
    stadium: Some((0x3e8, 0x258)),
    extra_stages: 0,
    set_up_rules: 0x56ede0,
    create_table: 0x56ecb0,
    source_file: 0x9c4294,
    box_lines: [0x4c, 0x53, 0x5a, 0x63, 0x6b],
};

/// Rule: the Football Conference is a league of the game's year under the English discipline rules, with a minimum
/// ground of 1,000 places (600 seated); with nothing saved, its rules, table, fixtures, order, statistics and
/// predicted positions are made.
///
/// League::League (`exe::new_league`), the competition record in `league::COMPETITION`, the vtable 0x969a74, the year
/// (word) and `discipline_rule::ENGLAND`; the saved state (`exe::league_load_from_temp_file`(this, 1)) ends it. Unlike
/// the four divisions above (`construct_english_division`), the ground minimums are written only after that check:
/// `league::REQUIRED_CAPACITY` = 1,000 and `league::REQUIRED_SEATS` = 600; then the main stage index,
/// `league::CURRENT_STAGE` = −1 and no extra stage (`league::STAGE_COUNT` = 0, `league::STAGES` = 0); the rules
/// (EngConf::set_up_rules 0x56ede0), the table (EngConf::create_table 0x56ecb0), the fixtures
/// (`exe::league_create_fixtures`) and the order (`exe::league_sort_table`(this, 0)), each with its box when it returns 0
/// (lines 0x4c, 0x53, 0x5a and 0x63, and the constructor returns); the statistics (`exe::crt_operator_new`(0x5ce),
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`; none: the box at line 0x6b) and the predicted positions
/// (`exe::league_set_predicted_positions`). The SEH frame only covers the new. Returns this.
/// Original: `EngConf::EngConf` (0x56db60, eng_conf.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_conf(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    let p = &CONFERENCE_DIVISION;
    exe::new_league(l);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, p.vtable);
    l.set_year(year as u16 as i16);
    l.set_discipline_rule(discipline_rule::ENGLAND);
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    covered!("rx10_1 new conference");
    l.set_required_capacity(0x3e8); // rx10_1: the Conference minimum ground
    l.set_required_seats(0x258);
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(0);
    l.set_stages(0);
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.set_up_rules);
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.create_table);
    if set_up_rules(this) == 0 {
        show_error_at(p.source_file, p.box_lines[0]);
        return this;
    }
    if create_table(this) == 0 {
        show_error_at(p.source_file, p.box_lines[1]);
        return this;
    }
    if exe::league_create_fixtures(l) == 0 {
        show_error_at(p.source_file, p.box_lines[2]);
        return this;
    }
    if exe::league_sort_table(l, 0) == 0 {
        show_error_at(p.source_file, p.box_lines[3]);
        return this;
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, l) } else { 0 };
    l.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[4]);
        return this;
    }
    exe::league_set_predicted_positions(l);
    this
}

/// Comparer for `construct_eng_conf`: only the original (it allocates and loads), then the plan
/// (`compare_english_constructor` with `CONFERENCE_DIVISION`: the vtable, the year, the record, the discipline rule,
/// the ground minimums and no extra stage).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_conf(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_english_constructor("construct_eng_conf", League(this), year, record, &CONFERENCE_DIVISION, || o(this, year, record))
}

/// The rule writes of the Football Conference (offset, size, value).
///
/// Two legs, 3 points a win and 1 a draw, league category 2, national clubs, tie-breaks 1, 2 and 3 [inference: goal
/// difference, goals for, position, as in the English divisions], 1 promoted (to the Third Division,
/// `KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION` in `league::UPPER_DIVISION_ID`), no play-off, 3 relegated with no league
/// below (−1 in `league::LOWER_DIVISION_ID`), 5 substitutes named and 3 used. Unlike the four divisions above,
/// `league::SQUAD_FLAGS` (+0x52) is not written. (The original writes +0xc5 twice, both times 1.)
pub(crate) unsafe fn conference_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(0x3c, 2, 2), (0xc4, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0x42, 1, 1), (0xc5, 1, 1), (0xc7, 1, 3)];
    v.extend_from_slice(&[(0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // rx10_1: three go down from the Conference
    v.extend_from_slice(&[(0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, 0xffff_ffff), (0x49, 1, 5)]);
    v
}

/// Rule: the Football Conference is played home and away, 3 points a win and 1 a draw; the champion goes up to the
/// Third Division and three go down, with no league below; 5 substitutes named and 3 used.
///
/// The fields of `conference_rules`, then the round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1,
/// &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0)) in `league::ROUND_DATES` (`eng_prm_apply_league_rules`, the mould of
/// the four divisions above). Returns 1.
/// Original: `EngConf::set_up_rules` (0x56ede0, eng_conf.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_conf_set_up_rules(this: usize) -> u32 {
    covered!("rx10_1 conference rules");
    eng_prm_apply_league_rules(League(this), &conference_rules())
}

/// Comparer for `eng_conf_set_up_rules`, as a dry run (`eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_eng_conf_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("eng_conf_set_up_rules", League(this), conference_rules(), || o(this), || eng_conf_set_up_rules(this))
}

/// Rule: the Football Conference has 22 clubs; prize fund by the share of 5,000 per place; TV pays 100,000 to each
/// club.
///
/// `eng_prm_fill_league_table` (the mould of the four divisions above: the clubs of `KEY_CLUB_COMP_ENGLISH_CONFERENCE`
/// in database order; the box at line 0x1f3 when out of memory; not full: 0), with League::calc_prize_fund(5,000),
/// `league::PRIZE_MODE` = 1 and League::pay_tv_rights(100,000, 0) when full. Returns 1 when full.
/// Original: `EngConf::create_table` (0x56ecb0, eng_conf.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_conf_create_table(this: usize) -> u32 {
    covered!("rx10_1 conference table");
    eng_prm_fill_league_table(League(this), 0x16, 0x9c4294, 0x1f3, KEY_CLUB_COMP_ENGLISH_CONFERENCE, 0x1388, || eng_prm_pay_tv_rights(this, 0x186a0, 0)) // rx10_1: 22 clubs, 5,000 a place, 100,000 TV
}

/// Comparer for `eng_conf_create_table`: only the original, with the plan (`eng_prm_compare_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_eng_conf_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_table("eng_conf_create_table", League(this), 0x16, KEY_CLUB_COMP_ENGLISH_CONFERENCE, 0x1388, || o(this))
}
