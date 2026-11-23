use super::*;

/// rule: the same turn in the Emperor's Cup (Japan): tables freed, history archived, year +1, the entrants
/// (JapEmpCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `JapEmpCup::start_new_season` (0x662950, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x6624f0, 0x9eab90, [0x217, 0x21e, 0x225], 1, "cqr_5 virada da copa do imperador") // cqr5: the Emperor’s Cup
}

/// rule: the Emperor's Cup has 7 single-leg rounds with a seeded draw (+0xd = 0x83): 1st round (64 teams) drawn 1/11,
/// played 29/11; 2nd 30/11 and 6/12; 3rd (16 enter) 7/12 and 13/12; 4th 14/12 and 20/12; quarter-finals 21/12 and
/// 23/12; semi-final 24/12 and 27/12; final drawn 28/12, played on 2/1 of the next year at a neutral ground [inference:
/// venue 4]; no prize per round (−1)
/// Original: `JapEmpCup::create_round_table` (0x661ff0, jap_emp_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_single_stage_cup_rounds(Cup(this), stage, rounds, extra, out, &EMPERORS_CUP_ROUNDS, 0x9eab90, 0xa0, "cqn_11 tabela da copa do imperador")
}

/// rule: the Emperor's Cup takes 80 clubs: 47 from the regional divisions
/// ([`KEY_CLUB_COMP_JAPANESE_ADMINISTRATIVE_DIVISION`], sorted by reputation and shuffled), 2 from the university
/// league ([`KEY_CLUB_COMP_JAPANESE_UNIVERSITY_LEAGUE`] [inference]), 3 from the football league
/// ([`KEY_CLUB_COMP_JAPANESE_FOOTBALL_LEAGUE`] [inference]) — those drawn from other Japanese divisions (up to 6) do
/// not enter —, the J2 clubs (class 0) and the J1 clubs with class = last season's position; the last 16 entries are
/// sorted by class; if the lower divisions lack clubs, the error box
/// Original: `JapEmpCup::collect_entrants` (0x6624f0, jap_emp_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_emp_cup_collect_entrants(this: usize) -> i32 {
    collect_emperors_cup_entrants(Cup(this)).0
}

/// The constructor of the Emperor's Cup: with no entrant table of its own; type 1 (clubs of one nation), prize fund of
/// 7,000,000, 5 substitutes on the bench and 3 substitutions, discipline rule 20; on a new cup, in order: the entrants
/// (JapEmpCup::collect_entrants, `jap_emp_cup_collect_entrants`, 0x6624f0, thiscall, ret 0; the error box of line 0x47
/// if it returns 0); the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the
/// tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x55 if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x5c without them); the final ranking (Cup::rank_teams,
/// `cup_rank_teams`, 0x5223a0, thiscall, ret 0, unchecked). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it
/// writes the fixed fields in the original's order (the vtable 0x96c9a0, the competition record at Comp +4, `cup::YEAR`
/// and those of `EMPERORS_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov
/// ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `JapEmpCup::JapEmpCup` (0x661d30, jap_emp_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_emp_cup(this: usize, year: u32, record: usize) -> usize {
    construct_jap_emp_cup_body(Cup(this), year, record) // cqu8: the Emperor's Cup
}

/// Comparer for `construct_jap_emp_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`jap_emp_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_emp_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_EMP_CUP)(this, year, record);
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, 0x96c9a0, &EMPERORS_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_jap_emp_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn jap_emp_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN || league_will_load_saved_state(record) {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_jap_emp_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_jap_emp_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96c9a0); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::JAPAN_CUP);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_8 emperors cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_JAPEMPCUP_COLLECT_ENTRANTS); // JapEmpCup::collect_entrants
    let r = collect_entrants(this.addr());
    if r == 0 {
        show_error_at(0x9eab90, 0x47);
        return this.addr();
    }
    this.set_prize_fund(0x6acfc0);
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0x9eab90, 0x55);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(0x9eab90, 0x5c);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// The fixed fields `construct_jap_emp_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const EMPERORS_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x6acfc0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x14), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
