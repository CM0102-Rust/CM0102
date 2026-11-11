// ---------- %%LEVA%%a leva: os construtores da Copa da Itália e da Copa da Série C (ita_c_cup.cpp, ita_cup.cpp) ----------

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS`; the original's SEH frame covers only that `new`. Returns whether they were created.
unsafe fn ita_cups_create_comp_stats(this: League) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record
/// (`league::COMPETITION`) and the year (`league::YEAR`); on a new cup (`league::STAGE_INDEX` = `stage_index::MAIN`),
/// each fixed field the constructor writes (offset, width, value) and the rounds (`cup::ROUND_TABLE`, when `rounds`)
/// and the statistics created.
unsafe fn ita_cups_check_constructor_plan(this: usize, r: usize, year: u32, owner: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + league::COMPETITION) != owner || read_u16(this + league::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + league::COMPETITION), read_u16(this + league::YEAR)));
    }
    if read_u8(this + league::STAGE_INDEX) != stage_index::MAIN {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + league::COMP_STATS) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + league::COMP_STATS)));
    }
    None
}

/// The constructor of the Coppa Italia, on the League base (`exe::new_league`, thiscall, ret 0): the fixed fields (the
/// vtable 0x96c318, the competition record in `league::COMPETITION`, the year in `league::YEAR`, 3 substitutions
/// allowed, 7 substitutes named, discipline rule `ITALY_CUP` (18), importance 0xc); if the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; returns != 0), returns this. On a new cup, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = -1,
/// `league::STAGE_COUNT` = 8 and the stage array in `league::STAGES` (`exe::crt_malloc` of 0x20, cdecl, unchecked);
/// the 48 entrants (`ita_cup_select_entrants`, 0x63faf0, thiscall, ret 0, unchecked); the statistics in
/// `league::COMP_STATS` (the box for line 0x54 without them); the predicted positions (ItaCup::set_predicted_positions,
/// 0x640210, thiscall, ret 0, unchecked); the rules (`ita_cup_set_up_rules`, 0x63f350, thiscall, ret 0; the box for
/// line 0x5e if it returns 0); the first group table (`ita_cup_create_first_group_table`, 0x63f250, thiscall, ret 0;
/// the box for line 0x65 if it returns 0); the fixtures (`exe::league_create_fixtures`, thiscall, ret 0; the box for
/// line 0x6b if it returns 0); the table order (`exe::league_sort_table`(this, 0), thiscall, ret 4; the box for line
/// 0x72 if it returns 0); the stage of each group (`ita_cup_create_group_stage`, 0x63f640, push i; mov ecx, this ->
/// thiscall, ret 4) for groups 0 to 6. Every failing step shows its box (`show_error_at`) and returns this. Returns
/// this. Rule: no prize fund (`league::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3
/// substitutions; discipline rule 18 [`league::SUBSTITUTES_NAMED`/`SUBSTITUTIONS_ALLOWED`/`DISCIPLINE_RULE`/
/// `CLUB_SIDES` from the competitions reference reading, section 2.1].
/// Original: `ItaCup::ItaCup` (0x63e900, ita_cup.cpp, thiscall, ret 8).
unsafe fn ita_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x96c318);
    write_u16(this + league::YEAR, year as u16);
    league.set_substitutions_allowed(0x3);
    league.set_substitutes_named(0x7);
    league.set_discipline_rule(discipline_rule::ITALY_CUP);
    league.set_importance(0xc);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqv_8 coppa italia new");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffffffff);
    write_u32(this + league::STAGE_COUNT, 0x8);
    league.set_stages(exe::crt_malloc(0x20));
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63faf0usize);
    f(this);
    if !ita_cups_create_comp_stats(league) {
        show_error_at(0x9ea23c, 0x54);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x640210usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f350usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x5e);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f250usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x65);
        return this;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0x9ea23c, 0x6b);
        return this;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0x9ea23c, 0x72);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63f640usize);
    for i in 0..7 {
        f(this, i);
    }
    this
}

/// The fixed fields the Coppa Italia constructor writes on a new cup (offset, width, value).
const COPPA_ITALIA_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 0x8), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 0x7), (league::SUBSTITUTIONS_ALLOWED, 1, 0x3), (league::DISCIPLINE_RULE, 1, 0x12), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];

/// The Coppa Italia constructor: see `ita_cup_constructor_body`.
/// Original: `ItaCup::ItaCup` (0x63e900, ita_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_cup(this: usize, year: u32, owner: usize) -> usize {
    ita_cup_constructor_body(this, year, owner) // cqv8: the Coppa Italia
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_ita_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, owner);
    let error = ita_cups_check_constructor_plan(this, r, year, owner, 0x96c318, &COPPA_ITALIA_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Serie C cup, on the League base (`exe::new_league`, thiscall, ret 0): the fixed fields (the
/// vtable 0x96c264, the competition record in `league::COMPETITION`, the year in `league::YEAR`, 3 substitutions
/// allowed, 7 substitutes named, discipline rule `ITALY_CUP` (18), importance 0xc); if the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; returns != 0), returns this. On a new cup, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = -1,
/// `league::STAGE_COUNT` = 0x10 and the stage array in `league::STAGES` (`exe::crt_malloc` of 0x40, cdecl,
/// unchecked); the 90 entrants (`ita_c_cup_select_entrants`, 0x63dfe0, thiscall, ret 0, unchecked); the statistics in
/// `league::COMP_STATS` (the box for line 0x4f without them); the predicted positions (ItaCCup::set_predicted_positions,
/// 0x63e580, thiscall, ret 0, unchecked); the rules (`ita_c_cup_set_up_rules`, 0x63d6e0, thiscall, ret 0; the box for
/// line 0x59 if it returns 0); the first group table (`init_first_group`, 0x63d5e0, thiscall, ret 0; the box for line
/// 0x60 if it returns 0); the fixtures (`exe::league_create_fixtures`, thiscall, ret 0; the box for line 0x66 if it
/// returns 0); the table order (`exe::league_sort_table`(this, 0), thiscall, ret 4; the box for line 0x6d if it returns
/// 0); the stage of each group (ItaCCup::create_group_stage, 0x63d9e0, push i; mov ecx, this -> thiscall, ret 4) for
/// groups 0 to 14. Every failing step shows its box (`show_error_at`) and returns this. Returns this. Rule: no prize
/// fund (`league::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3 substitutions; discipline
/// rule 18 [from the competitions reference reading, section 2.1].
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
unsafe fn ita_c_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x96c264);
    write_u16(this + league::YEAR, year as u16);
    league.set_substitutions_allowed(0x3);
    league.set_substitutes_named(0x7);
    league.set_discipline_rule(discipline_rule::ITALY_CUP);
    league.set_importance(0xc);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqv_8 serie c cup new");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffffffff);
    write_u32(this + league::STAGE_COUNT, 0x10);
    league.set_stages(exe::crt_malloc(0x40));
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63dfe0usize);
    f(this);
    if !ita_cups_create_comp_stats(league) {
        show_error_at(0x9ea204, 0x4f);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63e580usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d6e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x59);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d5e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x60);
        return this;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0x9ea204, 0x66);
        return this;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0x9ea204, 0x6d);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63d9e0usize);
    for i in 0..15 {
        f(this, i);
    }
    this
}

/// The fixed fields the Serie C cup constructor writes on a new cup (offset, width, value).
const SERIE_C_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 0x10), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 0x7), (league::SUBSTITUTIONS_ALLOWED, 1, 0x3), (league::DISCIPLINE_RULE, 1, 0x12), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];

/// The Serie C cup constructor: see `ita_c_cup_constructor_body`.
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    ita_c_cup_constructor_body(this, year, owner) // cqv8: the Serie C cup
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, owner);
    let error = ita_cups_check_constructor_plan(this, r, year, owner, 0x96c264, &SERIE_C_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}
