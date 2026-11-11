// ---------- batch %%LEVA%%: the constructors of the Asian club cups: the Asian Club Championship and the Asian Cup Winners' Cup (asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// Creates the cup's statistics at +8 (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4); when null, the error box `row` of
/// source `file` and false.
unsafe fn asian_club_cup_comp_stats(this: Cup, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// The cup's round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0):
/// push 0; push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`.
unsafe fn asian_club_cup_round_table(this: Cup) {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
}

/// The routines and the error-box lines of one of the two 32-club Asian cups.
struct AsianClubCupParts {
    vtable: usize,
    stages: u32,
    file: usize,
    select_qualified: usize,
    build_team_table: usize,
    create_stage: usize,
    predicted: usize,
    rows: [u32; 4],
}

/// The body of the two 32-club Asian cups (0x40aae0 and 0x40cbe0).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the competition record at +4, the vtable, the year (`set_year`),
/// `current_round` 0, `stage_index` 0xff, `current_stage` −1, `stage_count` = stages, `comp_type` 1 (cup), `club_sides`
/// 2, +0x1c = +0x20 = −1 (the upper and lower division ids of the Comp base), the qualified club list
/// (`asia_club_champ_fields::QUALIFIED_CLUBS`) 0, `discipline_index` 0x29, 5 `substitutes_named` and 3
/// `substitutions_allowed`. If the saved state loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: the stage vector (`exe::crt_malloc` of 4 × stages; null when stages ≤ 0), `importance_bias` 0xf,
/// `run_by_stages` 0, the 32 qualified clubs (`select_qualified`; 0: the box `rows[0]`), the team table
/// (`build_team_table`; 0: the box `rows[1]`), the prize fund 4,000,000, the round table (`asian_club_cup_round_table`),
/// the statistics (the box `rows[2]`), the zone cup stage (`create_stage`), the qualified list freed (`exe::crt_free`)
/// and zeroed, the tie tree (Cup::map_fixture_tree 0x518790; 0: the box `rows[3]`) and the predicted positions
/// (`predicted`). Each failing step returns this; all the routines are thiscall, ret 0. Returns this.
unsafe fn build_asian_club_cup(this: Cup, year: i16, record: usize, c: &AsianClubCupParts) -> usize {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let routine = |a: usize| -> unsafe extern "thiscall" fn(usize) -> u32 { core::mem::transmute(a) };
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), c.vtable as u32);
    this.set_year(year);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + cup::STAGE_COUNT, c.stages);
    this.set_comp_type(1);
    this.set_club_sides(2);
    write_u32(this.addr() + 0x1c, 0xffff_ffff);
    write_u32(this.addr() + 0x20, 0xffff_ffff);
    write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    this.set_discipline_index(0x29);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    let n = read_usize(this.addr() + cup::STAGE_COUNT) as i32;
    write_u32(this.addr() + cup::STAGES, if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) as u32 } else { 0 });
    this.set_importance_bias(0xf);
    this.set_run_by_stages(0);
    if routine(c.select_qualified)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[0]);
        return this.addr();
    }
    if routine(c.build_team_table)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[1]);
        return this.addr();
    }
    this.set_prize_fund(0x3d0900);
    asian_club_cup_round_table(this);
    if !asian_club_cup_comp_stats(this, c.file, c.rows[2]) {
        return this.addr();
    }
    routine(c.create_stage)(this.addr());
    let l = read_usize(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    }
    if map_fixture_tree(this.addr()) == 0 {
        show_error_at(c.file, c.rows[3]);
        return this.addr();
    }
    routine(c.predicted)(this.addr());
    this.addr()
}

/// The Asian Club Championship: vtable 0x9674d4, 4 stages; AsiaClubChamp::select_qualified_clubs (0x40b950),
/// build_team_table (0x40b620), create_second_zone_cup_stage (0x40bec0) and rank_teams (0x40c720, the predicted
/// positions); error boxes of source 0x987a40 at lines 0x4d, 0x54, 0x63 and 0x73.
const ASIAN_CLUB_CHAMPIONSHIP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x9674d4, stages: 4, file: 0x987a40, select_qualified: 0x40b950, build_team_table: 0x40b620, create_stage: 0x40bec0, predicted: 0x40c720, rows: [0x4d, 0x54, 0x63, 0x73] };
/// The Asian Cup Winners' Cup: vtable 0x967574, 2 stages; AsiaCupWinner::select_qualified_clubs (0x40d830),
/// create_team_entries (0x40d4d0), create_zone_stage (0x40ddf0) and rank_teams (0x40e3b0); error boxes of source
/// 0x987b60 at lines 0x4e, 0x55, 0x64 and 0x74.
const ASIAN_CUP_WINNERS_CUP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x967574, stages: 2, file: 0x987b60, select_qualified: 0x40d830, build_team_table: 0x40d4d0, create_stage: 0x40ddf0, predicted: 0x40e3b0, rows: [0x4e, 0x55, 0x64, 0x74] };

/// The Asian Club Championship of the game's year, at career creation: 4 stages and a 4,000,000 prize fund; with nothing
/// saved, the 32 qualifiers, the team table, the rounds, the statistics, the second zone cup stage, the tie tree and the
/// predicted positions (`build_asian_club_cup` with `ASIAN_CLUB_CHAMPIONSHIP_PARTS`). The original's SEH frame only
/// matters for exceptions.
/// Original: `AsiaClubChamp::AsiaClubChamp` (0x40aae0, asia_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian club championship");
    let year = year as i16; // cqw13: the Asian Club Championship of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CLUB_CHAMPIONSHIP_PARTS)
}

/// Shared comparer of the Asian club cup constructors. They allocate, draw and create stages, so they cannot run
/// twice: only the original runs, then the plan is checked: it returns this, the vtable, the record at +4 and the year
/// (`cup::YEAR`).
unsafe fn compare_asian_club_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && read == year as i16;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}

/// Comparer for `construct_asia_club_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_asian_club_cup_ctor("construct_asia_club_champ", this, year, record, 0x9674d4, || o(this, year, record))
}

/// The Asian Cup Winners' Cup of the game's year, at career creation: 2 stages and a 4,000,000 prize fund, built like
/// the Asian Club Championship (`build_asian_club_cup` with `ASIAN_CUP_WINNERS_CUP_PARTS`). The original's SEH frame
/// only matters for exceptions.
/// Original: `AsiaCupWinner::AsiaCupWinner` (0x40cbe0, asia_cup_winner.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian cup winners cup");
    let year = year as i16; // cqw13: the Asian Cup Winners' Cup of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CUP_WINNERS_CUP_PARTS)
}

/// Comparer for `construct_asia_cup_winner`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_cup_winner(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_asian_club_cup_ctor("construct_asia_cup_winner", this, year, record, 0x967574, || o(this, year, record))
}
