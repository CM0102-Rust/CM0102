// ---------- cqz_1: the rules of the Asian Cup, the Oceania Nations Cup and the Olympic football (asia_nations.cpp, oceania_nations.cpp, olympics.cpp) ----------

/// rule: the Asian Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group, national
/// teams, category 10, 7 substitutes and 3 changes; the draw date is 5 of month 5 of the cup year on weekday 5,
/// `.next_draw_year_offset()` 0; then the stage −1 round table (virtual +0x3c).
///
/// Writes the rule bytes and words of `AFRICAN_NATIONS_RULE_FIELDS` (the same values as the African Nations Cup:
/// `+0x43` 2, `+0xc6` 2, `+0xbe` 2, win 3, `+0xc7` 3, `+0x4a` 3, `+0x3c` 1, `+0xc3` 1, category 0xa, `+0xc5` 1,
/// `+0xbf`..`+0xc1` 0, `+0x49` 7), Date::Date(5, 5, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10;
/// the year pushed with garbage in the high 16 bits, read as a word) into `.next_draw_day()`, `.next_draw_year_offset()`
/// 0 and the round table of stage −1 (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10)
/// into `.round_dates()` (`finish_rules_setup`). Returns 1. The SEH frame only covers the local date (no destructor).
/// Original: `AsiaNations::set_up_rules` (0x40f4c0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 asian cup rules");
    let win = 3; // cqz1: the win in the Asian Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 5; // cqz1: the month of the Asian Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `asia_nations_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields,
/// the date and the return value are checked (`check_set_up_rules`).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    check_set_up_rules("asia_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 5, 0, 5), 0, || o(this))
}

/// rule: the Oceania Nations Cup groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3,
/// two qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 5 of month 2 of the cup year on
/// weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules` (the fields of `AFRICAN_NATIONS_RULE_FIELDS`; note `+0xbe` = 2, not the
/// 1 of the Oceania World Cup qualifying), with Date::Date(5, 2, year, 5). Returns 1.
/// Original: `OceaniaNations::set_up_rules` (0x799690, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 oceania nations cup rules");
    let win = 3; // cqz1: the win in the Oceania Nations Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 2; // cqz1: the month of the Oceania Nations Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `oceania_nations_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    check_set_up_rules("oceania_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 2, 0, 5), 0, || o(this))
}

/// rule: the Olympic football groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3, two
/// qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 20 of month 2 of the Olympic year
/// on weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules`, with Date::Date(0x14, 2, year, 5). Returns 1.
/// Original: `Olympics::set_up_rules` (0x7a0230, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 olympics rules");
    let win = 3; // cqz1: the win in the Olympic groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let day = 0x14; // cqz1: the day of the Olympic draw date
    let word = competition_date_word(League(this), day, 2, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `olympics_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_olympics_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    check_set_up_rules("olympics_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (0x14, 2, 0, 5), 0, || o(this))
}
