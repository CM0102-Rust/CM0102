// ---------- %%LEVA%%a leva: os construtores do Troféu da Liga inglesa e da Liga dos Campeões (eng_auto_cup.cpp, european_cup.cpp) ----------

/// The cup's rounds through its virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push -1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `cup::ROUND_TABLE`.
unsafe fn eng_auto_european_create_rounds(cup: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(cup.addr(), 0xffff_ffff, cup.addr() + cup::ROUND_COUNT, cup.addr() + cup::STAGE_LABEL, 0);
    cup.set_round_table(t);
}

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS` (the Comp base field); the original's SEH frame covers only that `new`. Returns whether they
/// were created.
unsafe fn eng_auto_european_create_comp_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + league::COMP_STATS, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record
/// (`league::COMPETITION`) and the year (`cup::YEAR`); on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), each
/// fixed field the constructor writes (offset, width, value) and the rounds (`cup::ROUND_TABLE`, when `rounds`) and the
/// statistics created.
unsafe fn eng_auto_european_check_constructor_plan(this: usize, r: usize, year: u32, owner: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + league::COMPETITION) != owner || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + league::COMPETITION), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != stage_index::MAIN {
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

/// The constructor of the English League Trophy, on the Cup base (`exe::new_cup`, thiscall, ret 0): the fixed fields
/// (the competition record in `league::COMPETITION`, the vtable 0x969894, the year in `cup::YEAR`,
/// `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN`, `cup::CURRENT_STAGE` = -1, `cup::STAGE_COUNT` =
/// 2, `cup::COMP_TYPE` = `comp_type::CUP`, +0x1c and +0x20 = -1, `eng_auto_cup::SELECTED_CLUBS` = 0, discipline rule
/// `ENGLAND` (9), 5 substitutes named, 3 substitutions allowed); if the saved state loads (Cup::load_from_disk 0x51fc00:
/// push 1; mov ecx, this -> thiscall, ret 4; returns != 0), returns this. On a new cup, in order: the stage array in
/// `cup::STAGES` (`exe::crt_malloc` of 4 x `cup::STAGE_COUNT`, or 0 with no stages); `cup::IMPORTANCE_BIAS` = 0xf and
/// `cup::RUN_BY_STAGES` = 0; the 56 section clubs (`select_section_clubs`, 0x56b730, thiscall, ret 0; the box for line
/// 0x52 if it returns 0); the entrants (`eng_auto_cup_collect_entrants`, 0x56b3c0, thiscall, ret 0; the box for line
/// 0x59 if it returns 0); the prize fund (`cup::PRIZE_FUND` = 400,000); the rounds (`eng_auto_european_create_rounds`);
/// the statistics in `league::COMP_STATS` (the box for line 0x68 without them); the section stage
/// (`create_section_stage`, 0x56bab0, thiscall, ret 0, unchecked); the list `eng_auto_cup::SELECTED_CLUBS` freed
/// (`exe::crt_free`) and zeroed; the tie tree (`map_fixture_tree`, 0x518790, thiscall, ret 0; the box for line 0x78 if
/// it returns 0); the final ranking (`eng_auto_cup_rank_teams`, 0x56c010, thiscall, ret 0, unchecked). Every failing
/// step shows its box (`show_error_at`) and returns this. Returns this. Rule: prize fund of 400,000; 5 substitutes on the
/// bench and 3 substitutions; discipline rule 9 [from the competitions reference reading, section 2.1].
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
unsafe fn eng_auto_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x969894);
    write_u16(this + cup::YEAR, year as u16);
    cup.set_current_round(0x0);
    cup.set_stage_index(stage_index::MAIN);
    write_u32(this + cup::CURRENT_STAGE, 0xffffffff);
    write_u32(this + cup::STAGE_COUNT, 0x2);
    cup.set_comp_type(comp_type::CUP);
    write_u32(this + 0x1c, 0xffffffff);
    write_u32(this + 0x20, 0xffffffff);
    write_u32(this + eng_auto_cup::SELECTED_CLUBS, 0x0);
    cup.set_discipline_index(discipline_rule::ENGLAND);
    cup.set_substitutes_named(0x5);
    cup.set_substitutions_allowed(0x3);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this, 1) != 0 {
        return this;
    }
    covered!("cqv_9 league trophy new");
    let stages = cup.stage_count() as i32;
    cup.set_stages(if stages > 0 { exe::crt_malloc((stages << 2) as u32) } else { 0 });
    cup.set_importance_bias(0xf);
    cup.set_run_by_stages(0x0);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b730usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x52);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b3c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x59);
        return this;
    }
    cup.set_prize_fund(0x61a80);
    eng_auto_european_create_rounds(cup);
    if !eng_auto_european_create_comp_stats(this) {
        show_error_at(0x9c4180, 0x68);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56bab0usize);
    f(this);
    let l = read_usize(this + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x78);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56c010usize);
    f(this);
    this
}

/// The fixed fields the League Trophy constructor writes on a new cup (offset, width, value).
const LEAGUE_TROPHY_FIELDS: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (cup::PRIZE_FUND, 4, 0x61a80), (cup::STAGE_COUNT, 4, 0x2), (cup::COMP_TYPE, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::IMPORTANCE_BIAS, 1, 0xf), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_auto_cup::SELECTED_CLUBS, 4, 0x0)];

/// The English League Trophy constructor: see `eng_auto_cup_constructor_body`.
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    eng_auto_cup_constructor_body(this, year, owner) // cqv9: the League Trophy
}

/// Only the original (it allocates, draws and reads the save file), checking the plan
/// (`eng_auto_european_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, owner);
    let error = eng_auto_european_check_constructor_plan(this, r, year, owner, 0x969894, &LEAGUE_TROPHY_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_auto_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Champions League, on the Cup base (`exe::new_cup`, thiscall, ret 0): the fixed fields (the
/// year in `cup::YEAR`, the vtable 0x96a1d0, `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN`,
/// `cup::CURRENT_STAGE` = -1, `cup::STAGE_COUNT` = 0xd, the competition record in `league::COMPETITION`,
/// `cup::CLUB_SIDES` = 2 (clubs of several nations), +0x1c and +0x20 = -1, +0x52 = 3, 7 substitutes named, 3
/// substitutions allowed, `cup::IMPORTANCE_BIAS` = 0xa, discipline rule `EUROPE` (1), the stage array in `cup::STAGES`
/// (`exe::crt_malloc` of 0x34, cdecl, unchecked), `cup::STAGE_LABEL` = 0, `cup::RUN_BY_STAGES` = 0); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns != 0), returns this. On a new
/// cup, in order: the year's European seeding (`update_season`, 0x9035a0, on `G_UEFA_SEEDING` if there is one: push 0;
/// mov ecx -> thiscall, ret 4); the 71 entrants (`european_cup_collect_entrants`, 0x584fa0, thiscall, ret 0,
/// unchecked); the qualifying table (`build_qualifying_entry_table`, 0x584990, thiscall, ret 0; the box for line 0x74 if
/// it returns 0); the rounds (`eng_auto_european_create_rounds`); the tie tree (`map_fixture_tree`, 0x518790, thiscall,
/// ret 0; the box for line 0x82 if it returns 0); the statistics in `league::COMP_STATS` (the box for line 0x89 without
/// them); the final ranking (`european_cup_rank_teams`, 0x586900, thiscall, ret 0, unchecked). Every failing step shows
/// its box (`show_error_at`) and returns this. Returns this. Rule: type 2 (clubs of several nations); no prize fund
/// (`cup::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3 substitutions; discipline rule 1 [from
/// the competitions reference reading, section 2.1].
/// Original: `EuropeanCup::EuropeanCup` (0x5837c0, european_cup.cpp, thiscall, ret 8).
unsafe fn european_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_u16(this + cup::YEAR, year as u16);
    write_usize(this, 0x96a1d0);
    cup.set_current_round(0x0);
    cup.set_stage_index(stage_index::MAIN);
    write_u32(this + cup::CURRENT_STAGE, 0xffffffff);
    write_u32(this + cup::STAGE_COUNT, 0xd);
    write_usize(this + league::COMPETITION, owner);
    cup.set_club_sides(0x2);
    write_u32(this + 0x1c, 0xffffffff);
    write_u32(this + 0x20, 0xffffffff);
    write_u8(this + 0x52, 0x3);
    cup.set_substitutes_named(0x7);
    cup.set_substitutions_allowed(0x3);
    cup.set_importance_bias(0xa);
    cup.set_discipline_index(discipline_rule::EUROPE);
    cup.set_stages(exe::crt_malloc(0x34));
    cup.set_stage_label(0x0);
    cup.set_run_by_stages(0x0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this, 1) != 0 {
        return this;
    }
    covered!("cqv_9 champions league new");
    // the year's European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let s = read_usize(G_UEFA_SEEDING);
    if s != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        f(s, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584fa0usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584990usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x74);
        return this;
    }
    eng_auto_european_create_rounds(cup);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x82);
        return this;
    }
    if !eng_auto_european_create_comp_stats(this) {
        show_error_at(0x9c4900, 0x89);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x586900usize);
    f(this);
    this
}

/// The fixed fields the Champions League constructor writes on a new cup (offset, width, value).
const CHAMPIONS_LEAGUE_FIELDS: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (cup::STAGE_COUNT, 4, 0xd), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (cup::IMPORTANCE_BIAS, 1, 0xa), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Champions League constructor: see `european_cup_constructor_body`.
/// Original: `EuropeanCup::EuropeanCup` (0x5837c0, european_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_european_cup(this: usize, year: u32, owner: usize) -> usize {
    european_cup_constructor_body(this, year, owner) // cqv9: the Champions League
}

/// Only the original (it allocates, draws and reads the save file), checking the plan
/// (`eng_auto_european_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_european_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, owner);
    let error = eng_auto_european_check_constructor_plan(this, r, year, owner, 0x96a1d0, &CHAMPIONS_LEAGUE_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_european_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}
