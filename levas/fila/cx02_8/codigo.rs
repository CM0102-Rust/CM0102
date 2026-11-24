// ---------- cx02_8: the Danish Cup and German League Cup rounds and the German Cup home advantage (den_cup.cpp, ger_lge_cup.cpp, ger_cup.cpp) ----------

/// The Danish Cup rounds: a prize fund of 800,000 and the round table (`set_up_prize_rounds`), then 4 substitutes
/// named and 3 substitutions allowed. Returns 1.
/// Original: `DenCup::set_up_rounds` (0x5516b0, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_set_up_rounds(this: usize) -> i32 {
    let cup = Cup(this);
    covered!("cx02_8 danish cup rounds");
    set_up_prize_rounds(cup, 800_000);
    cup.set_substitutes_named(4);
    cup.set_substitutions_allowed(3);
    1
}

/// The German League Cup rounds: 6 substitutes named and 3 substitutions allowed, then a prize fund of 500,000 and the
/// round table (`set_up_prize_rounds`). Returns 1.
/// Original: `GerLgeCup::set_up_rounds` (0x5dcb90, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_set_up_rounds(this: usize) -> i32 {
    let cup = Cup(this);
    covered!("cx02_8 german league cup rounds");
    cup.set_substitutes_named(6);
    cup.set_substitutions_allowed(3);
    set_up_prize_rounds(cup, 500_000)
}

/// The club that gets home advantage in a German Cup tie: the one outside the two Bundesliga divisions when its
/// opponent plays in one of them (the divisions compared as competition records: `club::DIVISION` against the records
/// of the German First and Second Divisions); both or neither in the Bundesliga: 0.
/// Original: `GerCup::club_in_higher_division` (0x5d9930, ger_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ger_cup_club_in_higher_division(_this: usize, a: usize, b: usize) -> usize {
    let (first, second) = (comp_record_of(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), comp_record_of(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let bundesliga = |club: usize| {
        let division = read_usize(club + club::DIVISION);
        division == first || division == second
    };
    match (bundesliga(a), bundesliga(b)) {
        (true, false) => b,
        (false, true) => a,
        _ => 0,
    }
}

pub(crate) unsafe extern "thiscall" fn compare_den_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("den_cup_set_up_rounds", Cup(this), || o(this), || den_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("ger_lge_cup_set_up_rounds", Cup(this), || o(this), || ger_lge_cup_set_up_rounds(this))
}

compare_value!(compare_ger_cup_club_in_higher_division, ger_cup_club_in_higher_division, %%I2%%, (a: usize, b: usize) -> usize);
