// ---------- batch %%LEVA%%: the season turnovers of the Intertoto Cup and the World Club Cup; the UEFA Cup's rounds (intertoto_cup.cpp, uefa_cup.cpp, world_club_cup.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\intertoto_cup.cpp" (the error boxes).
pub(crate) const INTERTOTO_CUP_CPP: usize = 0x9e9e14;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\world_club_cup.cpp".
pub(crate) const WORLD_CLUB_CUP_CPP: usize = 0xad5848;
/// WorldClubCup::select_teams (thiscall, ret 0): the World Club Cup's clubs.
pub(crate) const FN_WORLDCLUBCUP_SELECT_TEAMS: usize = 0x92b8b0;
/// The UEFA Cup's prize fund (3,000,000).
pub(crate) const UEFA_CUP_PRIZE_FUND: i32 = 3_000_000;

/// The season turnover of the Intertoto Cup: last season's tables freed, the season archived, the year moved on by one,
/// the winners not yet sent to the UEFA Cup, the UEFA seeding's new season, the entrants collected, the round table and
/// the tie tree rebuilt, the predicted positions and the first round set up.
///
/// The first half of the cup turnover (`cup_turnover_free_season`: the team, round and tie tables freed and zeroed, the
/// season archived, the year up by one, `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0), then
/// `intertoto_cup::WINNERS_SENT_TO_UEFA` = 0 (the original writes it between the two bytes) and, with the UEFA seeding
/// object ([`G_UEFA_SEEDING`]), UefaSeeding::update_season(0) (`exe::uefaseeding_update_season`, thiscall, ret 4). Then, each
/// with its own box (intertoto_cup.cpp, lines 0x1e9, 0x1f0 and 0x1f7) when it returns a negative value: the entrants
/// (`exe::intertotocup_collect_entrants` IntertotoCup::collect_entrants, thiscall, ret 0), the round table (virtual
/// `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`). Then the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and the first round (`exe::cup_set_up_round` Cup::set_up_round(0),
/// push 0; mov ecx -> thiscall, ret 4), whose eax it returns. After a box the copy returns 0 (the original returns the
/// box's eax; the caller does not read it).
/// Original: `IntertotoCup::start_new_season` (0x633920, intertoto_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    cup_turnover_free_season(this);
    write_u32(this.addr() + intertoto_cup::WINNERS_SENT_TO_UEFA, 0); // mwg10: the winners not yet sent on
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0);
    }
    covered!("mwg_10 intertoto rollover");
    if exe::intertotocup_collect_entrants(this) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1e9);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f0);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(INTERTOTO_CUP_CPP, 0x1f7);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS);
    exe::cup_set_up_round(this, 0) // mwg10: the first round set up
}

/// Comparer for `intertoto_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("intertoto_cup_start_new_season", Cup(this), || o(this))
}

/// Sets up the rounds of the UEFA Cup. Rule: the prize fund is 3,000,000; the round table comes from
/// `comp_vtable::CREATE_SCHEDULE`.
///
/// Ret 0; virtual +0x8c: `UefaCup::set_up_rounds()`; returns 1. The prize fund (`.set_prize_fund()`) of 3,000,000 and the
/// round table (`.set_round_table()`, the old one not freed) from `comp_vtable::CREATE_SCHEDULE` (push 0; push
/// &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10; the original writes the
/// fund after pushing the arguments, before the call). The same routine as the CONCACAF Champions' Cup's with another
/// fund.
/// Original: `UefaCup::set_up_rounds` (0x9030b0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    this.set_prize_fund(UEFA_CUP_PRIZE_FUND); // mwg10: the UEFA Cup's prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    covered!("mwg_10 uefa cup rounds");
    1
}

// simplification: virtual +0x3c allocates the round table: it does not run twice. Original only; the plan: the prize
// fund (+0x28) of 3,000,000 and the return 1.
/// Comparer for `uefa_cup_set_up_rounds`.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this.addr());
    SCOREBOARD.record(r == 1 && this.prize_fund() == UEFA_CUP_PRIZE_FUND, || format!("uefa_cup_set_up_rounds({:#x}) returned {} prize fund {}", this.addr(), r, this.prize_fund()));
    r
}

/// The season turnover of the World Club Cup: last season's tables freed, the season archived, the year moved on by one
/// and the clubs, the round table and the tie tree rebuilt; finally the predicted positions.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction): the clubs from
/// WorldClubCup::select_teams (`FN_WORLDCLUBCUP_SELECT_TEAMS`, thiscall, ret 0) and the world_club_cup.cpp boxes at
/// lines 0x176, 0x17d and 0x184.
/// Original: `WorldClubCup::start_new_season` (0x92bda0, world_club_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), FN_WORLDCLUBCUP_SELECT_TEAMS, WORLD_CLUB_CUP_CPP, [0x176, 0x17d, 0x184], 1, "mwg_10 world club cup rollover")
}

/// Comparer for `world_club_cup_start_new_season`: only the original runs; the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_world_club_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("world_club_cup_start_new_season", Cup(this), || o(this))
}
