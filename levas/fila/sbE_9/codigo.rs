// ---------- sbE_9: the Polish First and Second Division constructors (pol_first.cpp, pol_second.cpp) ----------

/// The Polish First Division's list of its 16 clubs (a pointer to 16 dwords, PolFirst::create_table's calloc; sorted by
/// PolFirst::set_predicted_positions).
pub(crate) const POL_FIRST_CLUB_LIST: usize = 0xee;

/// The Polish First Division as a nation division: no bench written, the Polish discipline rule, two stages (the second
/// group and the final stage); PolFirst::set_up_rules and PolFirst::create_table; pol_first.cpp (0xa78598) with the
/// boxes of the rules, the table, the fixtures, the order and the statistics.
pub(crate) const POL_FIRST_DIVISION: NationDivision = NationDivision {
    vtable: 0x96e360,
    bench: None,
    discipline: discipline_rule::POLAND,
    stages: 2,
    set_up_rules: 0x7c9690,
    create_table: 0x7c9450,
    source_file: 0xa78598,
    box_lines: [0x52, 0x59, 0x60, 0x69, 0x70],
};

/// Rule: the Polish First Division at career creation: the Polish discipline rule; a saved league in its final stage
/// gets back its order (the championship group first); a new one gets the rules, the first group's table, the fixtures,
/// the order, the statistics, the second group stage and the predicted positions.
///
/// `open_nation_division` (sbE_2) with `POL_FIRST_DIVISION`, `POL_FIRST_CLUB_LIST` = 0 before the load. Loaded: with
/// `league::CURRENT_STAGE` at 1 or more (signed), the final stage (stage slot 1) gets
/// `pol_first_compare_championship_group_first` (0x7ca450) as its `league::CUSTOM_COMPARE`; done. Otherwise
/// `build_new_nation_division` (sbE_2) and, when it went through, PolFirst::create_second_group_stage (0x7c9900, a direct
/// call) and the inlined PolFirst::set_predicted_positions (`pol_first_set_predicted_positions`, cx13_5: without
/// statistics the box of line 0x346). Returns this.
/// Original: `PolFirst::PolFirst` (0x7c8a80, pol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_first(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    if open_nation_division(l, year, record, &POL_FIRST_DIVISION, || write_usize(this + POL_FIRST_CLUB_LIST, 0)) {
        if l.current_stage() as i32 >= 1 {
            covered!("sbE_9 polish final stage order restored");
            write_usize(read_usize(l.stages() + 4) + league::CUSTOM_COMPARE, 0x7ca450); // sbE_9: the championship group first
        }
        return this;
    }
    if build_new_nation_division(l, &POL_FIRST_DIVISION) {
        covered!("sbE_9 new polish first division");
        call_on_league(0x7c9900, l);
        pol_first_set_predicted_positions(this);
    }
    this
}

/// Comparer for `construct_pol_first`: only the original runs (`compare_nation_division_ctor`); a saved league in its
/// final stage must have the final stage's order back.
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let r = compare_nation_division_ctor("construct_pol_first", League(this), year, record, &POL_FIRST_DIVISION, || o(this, year, record));
    let l = League(this);
    if l.current_stage() as i32 >= 1 && l.stages() != 0 {
        let order = read_usize(read_usize(l.stages() + 4) + league::CUSTOM_COMPARE);
        SCOREBOARD.record(order == 0x7ca450, || format!("construct_pol_first({:#x}): final stage order {:#x}", this, order));
    }
    r
}

/// The Polish Second Division as a nation division: no bench written, the Polish discipline rule, one stage (the
/// promotion play-off); PolSecond::set_up_rules and PolSecond::create_table; pol_second.cpp (0xa7860c).
pub(crate) const POL_SECOND_DIVISION: NationDivision = NationDivision {
    vtable: 0x96e4b4,
    bench: None,
    discipline: discipline_rule::POLAND,
    stages: 1,
    set_up_rules: 0x7cbf40,
    create_table: 0x7cbdf0,
    source_file: 0xa7860c,
    box_lines: [0x48, 0x4f, 0x56, 0x5f, 0x66],
};

/// Rule: the Polish Second Division at career creation: the Polish discipline rule, one stage besides the league (the
/// promotion play-off); with no saved state, the rules, the table, the fixtures, the order, the statistics and the
/// predicted positions (`construct_nation_division`).
/// Original: `PolSecond::PolSecond` (0x7cb1d0, pol_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_second(this: usize, year: u32, record: usize) -> usize {
    covered!("sbE_9 polish second division");
    construct_nation_division(League(this), year, record, &POL_SECOND_DIVISION, || {})
}

/// Comparer for `construct_pol_second`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nation_division_ctor("construct_pol_second", League(this), year, record, &POL_SECOND_DIVISION, || o(this, year, record))
}
