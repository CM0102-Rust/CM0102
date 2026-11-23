use super::*;

/// rule: the US Open Cup has 5 single-leg rounds (+0x21 = 0): 2nd round (32 teams, code 0x14) drawn 1/6, played 14/6,
/// with bit 0x400 in the draw; 3rd 15/6 and 25/6; quarter-finals 26/6 and 9/8; semi-final 10/8 and 23/8; final drawn
/// 24/8, played 22/10 at a neutral ground [inference: venue 4]; no prize per round (−1); category 4 in the first two
/// and 6 after
/// Original: `UsaOpenCup::create_round_table` (0x90b170, usa_open_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_single_stage_cup_rounds(Cup(this), stage, rounds, extra, out, &US_OPEN_CUP_ROUNDS, 0xaa20e0, 0x9c, "cqn_11 tabela da copa dos eua")
}

/// rule: the US Open Cup takes 32 clubs: 10 from each of the two lower leagues ([0x9cf594] and [0x9cf598], up to 50
/// looked at in each, sorted by reputation and shuffled among the best), with class 2, and all MLS clubs with class 1
/// Original: `UsaOpenCup::collect_entrants` (0x90b530, usa_open_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_collect_entrants(this: usize) -> i32 {
    collect_us_open_cup_entrants(Cup(this)).0
}

/// rule: the same turn in the US Open Cup: tables freed, history archived, year +1, the entrants
/// (UsaOpenCup::collect_entrants), the rounds, the tree and the predicted positions
/// Original: `UsaOpenCup::start_new_season` (0x90b790, usa_open_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_open_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x90b530, 0xaa20e0, [0x198, 0x19f, 0x1a6], 1, true, "cqr_6 virada da copa dos eua") // cqr6: the US Cup
}

/// The US Open Cup constructor `UsaOpenCup::UsaOpenCup(year, record)`, run once at career creation. On the Cup::Cup
/// base (`exe::new_cup`): the competition record at +4, the vtable (0x970998), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), discipline rule 30 and `run_by_stages()` = 0; the bench and substitutions stay as the base
/// left them. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it
/// returns this. On a new cup, in order: the entrants (UsaOpenCup::collect_entrants 0x90b530, thiscall, ret 0; the
/// error box of line 0x43 if it returns a negative value); the prize fund (`cup::PRIZE_FUND`) = 1,000,000; the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret
/// 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of line 0x58
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `UsaOpenCup::UsaOpenCup` (0x90aeb0, usa_open_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    construct_usa_open_cup_body(this, year, record) // cqv5: the US Open Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_OPEN_CUP)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, USA_OPEN_CUP_VTABLE, &USA_OPEN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_open_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the US Open Cup constructor (see `construct_usa_open_cup`).
pub(crate) unsafe fn construct_usa_open_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, USA_OPEN_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_discipline_index(0x1e);
    cup.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 us open cup new");
    if exe::usaopencup_collect_entrants(this) < 0 {
        show_error_at(0xaa20e0, 0x43);
        return this;
    }
    cup.set_prize_fund(0xf4240);
    ita_super_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) < 0 {
        show_error_at(0xaa20e0, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0xaa20e0, 0x58);
        return this;
    }
    exe::cup_rank_teams(this);
    this
}

pub(crate) const USA_OPEN_CUP_VTABLE: usize = 0x970998;

/// The fixed fields the US Open Cup constructor writes on a new cup (offset, width, value).
pub(crate) const USA_OPEN_CUP_CTOR_FIELDS: [(usize, u8, u32); 8] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0x1e), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
