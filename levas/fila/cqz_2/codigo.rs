// ---------- cqz_2: the rules of the European Championship, the Confederations Cup and the World Cup (euro_champ.cpp, fifa_confed.cpp, world_cup.cpp) ----------

/// Day of year of the European Championship seeding date (the first word of the date), read by its daily update.
pub(crate) const EURO_CHAMP_SEEDING_DAY: usize = 0x13e;
/// Year of the European Championship seeding date relative to the finals year (+0x40).
pub(crate) const EURO_CHAMP_SEEDING_YEAR_OFFSET: usize = 0x140;

/// The rule fields of the European Championship finals (offset, width, value), in the original's write order: national
/// teams, 2 qualifiers per group, single round robin, 3/1 points, category 10, tiebreak codes 4, 1 and 2, the stage id
/// 0x3e8 and 7 substitutes.
pub(crate) const EURO_CHAMP_RULE_FIELDS: [(usize, u8, u32); 15] = [(0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x3a, 2, 0x3e8), (0x49, 1, 7)];

/// rule: the European Championship groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers
/// per group, national teams, category 10, 7 substitutes and 3 changes; the seeding date is 10 January and the draw
/// date 10 February of the finals year (weekday 5).
///
/// Writes `EURO_CHAMP_RULE_FIELDS` (the win at `+0xc2`; the stage id `+0x3a` = 0x3e8 before the round table rewrites
/// it), Date::Date(0xa, 0, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10; the year with garbage
/// in the high 16 bits, read as a word) into `EURO_CHAMP_SEEDING_DAY` and 0 into `EURO_CHAMP_SEEDING_YEAR_OFFSET`, then
/// Date::Date(0xa, 1, year, 5) into `.next_draw_day()`, `.next_draw_year_offset()` 0 and the stage −1 round table
/// (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into `.round_dates()`. Returns 1.
/// Original: `EuroChamp::set_up_rules` (0x57b630, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqz_2 euro rules");
    let win = 3; // cqz2: the win in the European Championship groups
    goldcup_write_rule_fields(this, &EURO_CHAMP_RULE_FIELDS, win);
    let seeding = 0; // cqz2: the seeding month of the European Championship (January)
    let word = competition_date_word(League(this), 0xa, seeding, 0, 5);
    write_u16(this + EURO_CHAMP_SEEDING_DAY, word);
    write_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 0);
    let word = competition_date_word(League(this), 0xa, 1, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `euro_champ_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields, both
/// dates and the return value.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = check_set_up_rules("euro_champ_set_up_rules", this, &EURO_CHAMP_RULE_FIELDS, (0xa, 1, 0, 5), 0, || o(this));
    let word = competition_date_word(League(this), 0xa, 0, 0, 5);
    let (day, offset) = (read_u16(this + EURO_CHAMP_SEEDING_DAY), read_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET));
    SCOREBOARD.record(day == word && offset == 0, || format!("euro_champ_set_up_rules({:#x}): seeding date {:#x} expected {:#x}, offset {}", this, day, word, offset));
    r
}

/// The rule fields of the Confederations Cup (offset, width, value), in the original's write order: like the European
/// Championship without the stage id, and `.league_flags()` 0x28.
pub(crate) const FIFA_CONFED_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0xd9, 2, 0x28)];

/// rule: the Confederations Cup groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers per
/// group, national teams, category 10, 7 substitutes and 3 changes; the draw date is 20 February of the cup year
/// (weekday 5).
///
/// Writes `FIFA_CONFED_RULE_FIELDS` (the win at `+0xc2`; `.league_flags()` = 0x28), then, unlike its sisters, the
/// stage −1 round table first (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into
/// `.round_dates()`, and after it Date::Date(0x14, 1, year, 5) (0x549f70) into `.next_draw_day()` with
/// `.next_draw_year_offset()` 0. Returns 1. No SEH frame.
/// Original: `FifaConfed::set_up_rules` (0x58bdb0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_set_up_rules(this: usize) -> u32 {
    let league = League(this);
    covered!("cqz_2 confederations cup rules");
    let win = 3; // cqz2: the win in the Confederations Cup groups
    goldcup_write_rule_fields(this, &FIFA_CONFED_RULE_FIELDS, win);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0);
    league.set_round_dates(t);
    let day = 0x14; // cqz2: the day of the Confederations Cup draw date
    let word = competition_date_word(league, day, 1, 0, 5);
    league.set_next_draw_year_offset(0);
    league.set_next_draw_day(word as i16);
    1
}

/// Comparer for `fifa_confed_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    check_set_up_rules("fifa_confed_set_up_rules", this, &FIFA_CONFED_RULE_FIELDS, (0x14, 1, 0, 5), 0, || o(this))
}

/// The rule fields of the World Cup finals (offset, width, value), in the original's write order: those of the
/// European Championship plus `+0xc8` 0.
pub(crate) const WORLD_CUP_RULE_FIELDS: [(usize, u8, u32); 16] = [(0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xc8, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x3a, 2, 0x3e8), (0x49, 1, 7)];

/// rule: the World Cup groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers per group,
/// national teams, category 10, 7 substitutes and 3 changes; the seeding date is 15 December and the draw date 27
/// December of the year before the finals (month 0xb, year offset −1, weekday −1: the exact day).
///
/// Writes `WORLD_CUP_RULE_FIELDS` (the win at `+0xc2`), Date::Date(0xf, 0xb, year, −1) (0x549f70: lea ecx, local; 4
/// pushes -> thiscall, ret 0x10) into `world_cup_fields::SEEDING_DAY` with `SEEDING_YEAR_OFFSET` −1, then
/// Date::Date(0x1b, 0xb, year, −1) into `.next_draw_day()` with `.next_draw_year_offset()` −1 and the stage −1 round
/// table (virtual +0x3c) into `.round_dates()`. Both dates are computed in the finals year itself (the offset −1 is
/// applied by the readers). Returns 1.
/// Original: `WorldCup::set_up_rules` (0x92d200, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqz_2 world cup rules");
    let win = 3; // cqz2: the win in the World Cup groups
    goldcup_write_rule_fields(this, &WORLD_CUP_RULE_FIELDS, win);
    let word = competition_date_word(League(this), 0xf, 0xb, 0, 0xffff_ffff);
    write_u16(this + world_cup_fields::SEEDING_DAY, word);
    write_u16(this + world_cup_fields::SEEDING_YEAR_OFFSET, 0xffff);
    let day = 0x1b; // cqz2: the day of the World Cup draw date
    let word = competition_date_word(League(this), day, 0xb, 0, 0xffff_ffff);
    finish_rules_setup(League(this), word, 0xffff)
}

/// Comparer for `world_cup_set_up_rules`: original only, then the fields, both dates and the return value.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let r = check_set_up_rules("world_cup_set_up_rules", this, &WORLD_CUP_RULE_FIELDS, (0x1b, 0xb, 0, 0xffff_ffff), 0xffff, || o(this));
    let word = competition_date_word(League(this), 0xf, 0xb, 0, 0xffff_ffff);
    let (day, offset) = (read_u16(this + world_cup_fields::SEEDING_DAY), read_u16(this + world_cup_fields::SEEDING_YEAR_OFFSET));
    SCOREBOARD.record(day == word && offset == 0xffff, || format!("world_cup_set_up_rules({:#x}): seeding date {:#x} expected {:#x}, offset {:#x}", this, day, word, offset));
    r
}
