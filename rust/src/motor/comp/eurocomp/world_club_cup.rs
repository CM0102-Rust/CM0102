use super::*;

/// rule: the Intercontinental Cup (the European against the South American champion [inference]) is a single match on 1
/// December, drawn on 10 October, venue 4 (Tokyo [inference]), category 6, tie-break 3
/// Original: `WorldClubCup::create_round_table` (0x92b770, world_club_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn world_club_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &INTERCONTINENTAL_CUP_ROUNDS, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CLUB_CUP_START_NEW_SEASON);
    compare_cup_new_season("world_club_cup_start_new_season", Cup(this), || o(this))
}

/// The Intercontinental Cup of the game's year, at career creation, with the two clubs (the European and the South
/// American winners) picked at once by WorldClubCup::pick_qualifiers (0x92b8b0): vtable 0x971250, +0x52 = 3, 3
/// substitutions, error boxes of source 0xad5848 (`build_two_club_cup`). The original's SEH frame only matters for
/// exceptions.
/// Original: `WorldClubCup::WorldClubCup` (0x92b4b0, world_club_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_12 intercontinental cup");
    let year = year; // cqw12: the Intercontinental Cup of the game's year
    build_two_club_cup(Cup(this), year, record, 0x971250, 3, 3, 0x92b8b0, 0xad5848)
}

/// Comparer for `construct_world_club_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_world_club_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WORLD_CLUB_CUP);
    compare_intercontinental_cup_ctor("construct_world_club_cup", this, year, record, 0x971250, || o(this, year, record))
}

/// The two Intercontinental Cup clubs. Place 0 is Europe's: the Champions League winner
/// ([`KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`]), or a European club with reputation above 5,000 drawn when Europe
/// has no loaded league outside 2001 (the error box of line 0xea with none; a null winner: the error box of line 0xd1
/// and 0). Place 1 is South America's: the Libertadores winner ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or
/// the draw in the same way (boxes at lines 0x12a and 0xff); when it is the winner and the winner is South American,
/// the Libertadores runner-up takes its place if it is South American too. Returns 1 (−1 out of memory). (Stack frame
/// 0x11a4.)
/// Original: `WorldClubCup::pick_qualifiers` (0x92b8b0, world_club_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0xad5848) else {
        return -1;
    };
    let (europe, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_EUROPE, 0x1388, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0, 0xad5848, 0xea); // cqu13: the European champion
    write_usize(t + if fallback { 6 } else { 0 }, europe);
    if by_winner {
        covered!("cqu_13 european club");
        if europe == 0 {
            show_error_at(0xad5848, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0xad5848, 0x12a);
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        if south == 0 {
            show_error_at(0xad5848, 0xff);
            return 0;
        }
        if intercontinental_club_in_continent(south, KEY_CONTINENT_SOUTH_AMERICA) {
            let runner_up = exe::comp_util_get_latest_placed_club(comp_record_of_key(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 1);
            if intercontinental_club_in_continent(runner_up, KEY_CONTINENT_SOUTH_AMERICA) {
                write_usize(read_usize(this + cup::TEAM_TABLE) + 6, runner_up);
            }
        }
    }
    intercontinental_cup_finish(this)
}

pub(crate) unsafe extern "thiscall" fn compare_world_club_cup_pick_qualifiers(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WORLD_CLUB_CUP_PICK_QUALIFIERS);
    intercontinental_cup_check_clubs("world_club_cup_pick_qualifiers", this, (KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The body of a two-club inter-continental cup (the Intercontinental Cup; the Interamerican Cup, 0x632080, has the same
/// shape).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the year (`set_year`), the competition record at +4, the vtable, the
/// common fields (`intercontinental_cup_common_fields`), `club_sides` 2, +0x52 = `b52`, 7 `substitutes_named`,
/// `substitutions_allowed` = `substitutions`, `discipline_index` 0xff and `run_by_stages` 0. If the saved state loads,
/// returns this. Otherwise the two clubs (the routine `select_teams`, WorldClubCup::pick_qualifiers in the Intercontinental Cup, thiscall, ret 0; negative: the error box of line
/// 0x41 of source `file` and this), the common end (`finish_intercontinental_cup`) and, when it got to the end, the
/// predicted positions (Cup::rank_teams 0x5223a0, thiscall, ret 0). Returns this.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn build_two_club_cup(this: Cup, year: u32, record: usize, vtable: usize, b52: u8, substitutions: u8, select_teams: usize, file: usize) -> usize {
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(select_teams);
    let rank_teams: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    exe::new_cup(this);
    this.set_year(year as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), vtable as u32);
    intercontinental_cup_common_fields(this);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, b52);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(substitutions);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    if intercontinental_cup_loads(this) {
        return this.addr();
    }
    if select_teams(this.addr()) < 0 {
        show_error_at(file, 0x41);
        return this.addr();
    }
    if finish_intercontinental_cup(this, file) {
        rank_teams(this.addr());
    }
    this.addr()
}

/// Is the club's continent ([[`club::NATION`] + `nation::CONTINENT`]] (dword)) the one of the key `continent`?
pub(crate) unsafe fn intercontinental_club_in_continent(c: usize, continent: usize) -> bool {
    let c = Club(c);
    let n = c.nation();
    if n.is_null() {
        return false;
    }
    let k = n.continent();
    k != 0 && read_usize(k) == read_usize(continent)
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\world_club_cup.cpp".
pub(crate) const WORLD_CLUB_CUP_CPP: usize = 0xad5848;

/// WorldClubCup::select_teams (thiscall, ret 0): the World Club Cup's clubs.
pub(crate) const FN_WORLDCLUBCUP_SELECT_TEAMS: usize = 0x92b8b0;
