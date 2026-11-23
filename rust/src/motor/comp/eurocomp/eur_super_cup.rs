use super::*;

/// `EurSuperCup::select_teams()` (`eur_super_cup_select_teams`, thiscall, ret 0, eur_super_cup.cpp): the two Super Cup teams; returns 1.
///
/// A two-entry list (out of memory, the error box for line 0xc1 and −1). The Champions League winner ([0x9cf6e8]; if
/// missing, error box 0xca and 0) and, in the 1999 cup, the Cup Winners' Cup winner ([0x9cf6ec]); in the others, the
/// UEFA Cup winner ([0x9cf6f0]); if missing, 0xd5 and 0.
pub(crate) unsafe fn select_european_super_cup_teams(this: Cup) -> (i32, i32) {
    const F: usize = 0x9c45f8;
    let l = match alloc_two_team_list(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0); // cqq11: the Champions League winner
    if !set_team_slot(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    let cup = if this.year() == 0x7cf { KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP } else { KEY_CLUB_COMP_UEFA_CUP };
    let b = latest_placed_club(cup, 0); // cqq11: the UEFA Cup winner
    if !set_team_slot(l, 1, b, F, 0xd5) {
        return (0, 0);
    }
    covered!("cqq_11 european super cup clubs");
    finish_two_team_list(l)
}

/// rule: the European Super Cup is between the Champions League winner and the Cup Winners' Cup winner in the 1999
/// edition (the last Cup Winners' Cup) and, in the others, the UEFA Cup winner; without either, the error box
/// Original: `EurSuperCup::select_teams` (0x57a500, eur_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eur_super_cup_select_teams(this: usize) -> i32 {
    select_european_super_cup_teams(Cup(this)).0
}

/// `AsiaSuperCup::select_teams()` (`0x410a50`, thiscall, ret 0, `asia_super_cup.cpp`): picks the Asian Champions Cup
/// and Asian Cup Winners' Cup winners.
///
/// Allocates the list of two (out of memory, the box of line `0xc0` and −1). The Asian Champions League winner
/// ([`0x9cf64c`]; without it, `0xc9` and 0) and the Asian Cup Winners' Cup winner ([`0x9cf650`]; without it, `0xd1` and
/// 0); if both are the same club, the first becomes the Champions League runner-up (without checking, like the original
/// routine). Returns 1.
pub(crate) unsafe fn select_asian_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x987be0;
    let l = match alloc_two_team_list(Cup(this), F, 0xc0) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 0);
    if !set_team_slot(l, 0, a, F, 0xc9) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_ASIAN_CUP_WINNERS_CUP, 0); // cqq11: the Asian Cup Winners’ Cup winner
    if !set_team_slot(l, 1, b, F, 0xd1) {
        return (0, 0);
    }
    if a == b {
        covered!("cqq_11 asian double");
        write_u32(l, latest_placed_club(KEY_CLUB_COMP_ASIAN_CHAMPIONS_CUP, 1) as u32);
    }
    covered!("cqq_11 asian super cup clubs");
    finish_two_team_list(l)
}

/// `ItaC1Super::init_team_list()` (`ita_c1_super_init_team_list`, thiscall, ret 0, ita_c1_super.cpp): a two-entry list with both slots
/// empty; returns 1.
///
/// Out of memory: the error box for line 0xc2 and −1. The clubs enter later, with the winners of the two C1 groups
/// [inference]; class and status 0.
pub(crate) unsafe fn init_c1_super_cup_list(this: usize) -> (i32, i32) {
    let l = match alloc_two_team_list(Cup(this), 0x9ea1c8, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    covered!("cqq_11 c1 super cup places");
    write_u32(l, 0);
    write_u32(l + 6, 0);
    finish_two_team_list(l)
}

/// rule: the European Super Cup is a single match on 24 August (drawn on 4 July) on neutral ground (venue 0x8000:
/// Monaco [inference]), category 8, tie-break 0x83, no prizes
/// Original: `EurSuperCup::create_round_table` (0x57a3c0, eur_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eur_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &EUROPEAN_SUPER_CUP_ROUNDS, 0x9c45f8, 0x98, "cqp_10 tabela da supercopa europeia")
}

/// The season turnover of the European Super Cup: last season's tables freed, the season archived, the year moved on by
/// one, the two clubs picked, the round table and the tie tree rebuilt, the first round set up and the predicted
/// positions.
///
/// `european_cup_free_season` without the entrant book and the stages. Then, each with its own box (eur_super_cup.cpp,
/// lines 0x118, 0x11f and 0x126) when it returns a negative value: the two clubs (`exe::eursupercup_select_teams`
/// EurSuperCup::select_teams, thiscall, ret 0), the round table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree
/// (virtual `cup_vtable::MAP_FIXTURE_TREE`). Then the first round (`exe::cup_set_up_round` Cup::set_up_round(0), push
/// 0; mov ecx -> thiscall, ret 4) and the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax
/// it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read
/// it).
/// Original: `EurSuperCup::start_new_season` (0x57a6d0, eur_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eur_super_cup_start_new_season(this: usize) -> u32 {
    use european_turnover_consts as et;
    let this = Cup(this);
    european_cup_free_season(this, false, false);
    covered!("mwg_9 european super cup rollover");
    if exe::eursupercup_select_teams(this) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x118);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x11f);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(et::EUR_SUPER_CUP_CPP, 0x126);
        return 0;
    }
    exe::cup_set_up_round(this, 0);
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `eur_super_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eur_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EUR_SUPER_CUP_START_NEW_SEASON);
    compare_cup_new_season("eur_super_cup_start_new_season", Cup(this), || o(this))
}

/// The European Super Cup constructor `EurSuperCup::EurSuperCup(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x969f80),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 2
/// (clubs of several nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and
/// `run_by_stages()` = 0; no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads
/// (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the
/// two clubs (EurSuperCup::select_teams 0x57a500, thiscall, ret 0; the error box of line 0x3f if it returns a negative
/// value); the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at +8 (the
/// error box of line 0x54 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked).
/// Every failure returns this.
/// Original: `EurSuperCup::EurSuperCup` (0x57a100, eur_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eur_super_cup_body(this, year, record) // cqu9: the European Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eur_super_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EUR_SUPER_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, EUR_SUPER_CUP_VTABLE, &EUR_SUPER_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eur_super_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
pub(crate) unsafe fn uefa_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff || league_will_load_saved_state(record) {
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
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

/// The body of the European Super Cup constructor (see `construct_eur_super_cup`).
pub(crate) unsafe fn construct_eur_super_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, EUR_SUPER_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 european super cup new");
    if exe::eursupercup_select_teams(this) < 0 {
        show_error_at(0x9c45f8, 0x3f);
        return this;
    }
    uefa_cup_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) < 0 {
        show_error_at(0x9c45f8, 0x4d);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0x9c45f8, 0x54);
        return this;
    }
    exe::cup_rank_teams(this);
    this
}

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
pub(crate) unsafe fn uefa_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
pub(crate) unsafe fn uefa_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

pub(crate) const EUR_SUPER_CUP_VTABLE: usize = 0x969f80;

/// The fixed fields the European Super Cup constructor writes on a new cup (offset, width, value).
pub(crate) const EUR_SUPER_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
