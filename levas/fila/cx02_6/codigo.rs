// ---------- cx02_6: the French cups' bench, home-advantage club and League Cup rounds (fra_cup.cpp, fra_lge_cup.cpp) ----------

/// The French cups' bench rules for a fixture (the Coupe de France and the French League Cup): the final
/// (`fixture::ROUND_NAME` 0x96) names 5 substitutes, other rounds the cup's `SUBSTITUTES_NAMED` (& 0xf), into the low
/// nibble of the fixture's `SQUAD_RULES`; then the cup's `SUBSTITUTIONS_ALLOWED` (& 0xf) into the next nibble, the
/// other bits kept.
/// Original: `FraCup::set_fixture_substitutes` (0x5bb100, fra_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fra_cup_set_fixture_substitutes(this: usize, fixture: usize) {
    let (cup, fixture) = (Cup(this), Fixture(fixture));
    if fixture.round_name() == 0x96 {
        covered!("cx02_6 french cup final bench");
        fixture.set_squad_rules((fixture.squad_rules() & 0xfff5) | 5);
    } else {
        fixture.set_squad_rules((fixture.squad_rules() & 0xfff0) | (cup.substitutes_named() & 0xf) as u16);
    }
    fixture.set_squad_rules((fixture.squad_rules() & 0xff0f) | ((cup.substitutions_allowed() & 0xf) as u16) << 4);
}

/// The club that gets home advantage in a Coupe de France tie because the other plays in a higher division, or 0.
/// Only clubs of the top French tiers count, by their division (`club::DIVISION`, a competition record): with `a` in
/// the First Division, `b` unless it plays in the First or Second Division (a `b` with no division counts as lower);
/// with `a` in the Second Division, `b` when its division's reputation is below the National's; with `a` in the
/// National, `b` when its division's reputation is below the CFA's; otherwise (or `a` with no division) 0.
/// Original: `FraCup::club_in_higher_division` (0x5bb030, fra_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fra_cup_club_in_higher_division(_this: usize, a: usize, b: usize) -> usize {
    let (da, db) = (Club(a).division(), Club(b).division());
    if da.is_null() {
        return 0;
    }
    let id = read_u32(da.addr());
    let below = |key: usize| db.is_null() || db.reputation() < Competition(comp_record_of(key)).reputation();
    if id == read_u32(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION) {
        covered!("cx02_6 first division club");
        let top = !db.is_null() && (read_u32(db.addr()) == read_u32(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION) || read_u32(db.addr()) == read_u32(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION));
        return if top { 0 } else { b };
    }
    if id == read_u32(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION) {
        return if below(KEY_CLUB_COMP_FRENCH_NATIONAL) { b } else { 0 };
    }
    if id == read_u32(KEY_CLUB_COMP_FRENCH_NATIONAL) {
        return if below(KEY_CLUB_COMP_FRENCH_CFA) { b } else { 0 };
    }
    0
}

/// The French League Cup rounds: a prize fund of 500,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `FraLgeCup::set_up_rounds` (0x5bd5d0, fra_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_lge_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_6 french league cup rounds");
    set_up_prize_rounds(Cup(this), 500_000)
}

/// Comparer for `fra_cup_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word side by side.
pub(crate) unsafe extern "thiscall" fn compare_fra_cup_set_fixture_substitutes(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture + fixture::SQUAD_RULES, 2)], || o(this, fixture), || fra_cup_set_fixture_substitutes(this, fixture));
    SCOREBOARD.record(same, || format!("fra_cup_set_fixture_substitutes({:#x}): original {:x?} rust {:x?}", fixture, mo, mr));
}

compare_value!(compare_fra_cup_club_in_higher_division, fra_cup_club_in_higher_division, %%I1%%, (a: usize, b: usize) -> usize);

pub(crate) unsafe extern "thiscall" fn compare_fra_lge_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    compare_set_up_prize_rounds("fra_lge_cup_set_up_rounds", Cup(this), || o(this), || fra_lge_cup_set_up_rounds(this))
}
