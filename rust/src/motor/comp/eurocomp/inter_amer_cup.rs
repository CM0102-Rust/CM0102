use super::*;

/// rule: the Inter-American Cup is two-legged from 26 January of the following year (7 days), drawn on 10 October,
/// category 8, two-leg tie-break 3
/// Original: `InterAmerCup::create_round_table` (0x632340, inter_amer_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn inter_amer_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &INTERAMERICAN_CUP, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}

/// The season turnover of the Inter-American Cup: last season's tables freed, the season archived, the year moved on by
/// one and the clubs, the round table and the tie tree rebuilt; finally the predicted positions.
///
/// The cup turnover template (`start_new_cup_season`, checked instruction by instruction): the clubs from
/// InterAmerCup::select_teams (`FN_INTERAMERCUP_SELECT_TEAMS`, thiscall, ret 0) and the inter_amer_cup.cpp boxes at
/// lines 0x164, 0x16b and 0x172.
/// Original: `InterAmerCup::start_new_season` (0x632910, inter_amer_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn inter_amer_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), FN_INTERAMERCUP_SELECT_TEAMS, INTER_AMER_CUP_CPP, [0x164, 0x16b, 0x172], 1, "mwg_11 inter-american cup rollover")
}

/// Comparer for `inter_amer_cup_start_new_season`: only the original runs; the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_inter_amer_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INTER_AMER_CUP_START_NEW_SEASON);
    compare_cup_new_season("inter_amer_cup_start_new_season", Cup(this), || o(this))
}

/// The Interamerican Cup at career creation: the year and the database competition; a new cup has the two finalists
/// (clubs of several nations), 7 substitutes on the bench and 3 substitutions, discipline rule 255, no prize fund of its
/// own, the rounds, the tie tree, the statistics and the final ranking (see `build_interamerican_cup`).
/// Original: `InterAmerCup::InterAmerCup` (0x632080, inter_amer_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    build_interamerican_cup(Cup(this), year, record) // cqw5: the Interamerican Cup
}

/// Comparer for `construct_inter_amer_cup`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_inter_amer_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_INTER_AMER_CUP)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x96bad8, &INTERAMERICAN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_inter_amer_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The two Interamerican Cup clubs. Place 0 is North America's: the CONCACAF Champions Cup winner
/// ([`KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP`]), or a North American club with reputation above 1,000 drawn when it has
/// no loaded league outside 2001 (boxes at lines 0xea and 0xd1); place 1 is South America's: the Libertadores winner
/// ([`KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES`]) or the draw above 5,000 (boxes at lines 0x118 and 0xff).
/// Returns 1 (0 with a null winner; −1 out of memory). (Stack frame 0x11a4.)
/// Original: `InterAmerCup::select_teams` (0x632480, inter_amer_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn inter_amer_cup_select_teams(this: usize) -> i32 {
    let Some(t) = intercontinental_cup_team_table(this, 0x9e9d94) else {
        return -1;
    };
    let (north, by_winner, fallback) = intercontinental_cup_slot(this, KEY_CONTINENT_NORTH_AMERICA, 0x3e8, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, 0, 0x9e9d94, 0xea); // cqu13: the CONCACAF champion
    write_usize(t + if fallback { 6 } else { 0 }, north);
    if by_winner {
        covered!("cqu_13 north american club");
        if north == 0 {
            show_error_at(0x9e9d94, 0xd1);
            return 0;
        }
    }
    let (south, by_winner, _) = intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0x9e9d94, 0x118); // cqu13: the Libertadores champion in the Interamerican Cup
    write_usize(read_usize(this + cup::TEAM_TABLE) + 6, south);
    if by_winner {
        covered!("cqu_13 interamerican south american club");
        if south == 0 {
            show_error_at(0x9e9d94, 0xff);
            return 0;
        }
    }
    intercontinental_cup_finish(this)
}

pub(crate) unsafe extern "thiscall" fn compare_inter_amer_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_INTER_AMER_CUP_SELECT_TEAMS);
    intercontinental_cup_check_clubs("inter_amer_cup_select_teams", this, (KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_SOUTH_AMERICA), || o(this))
}

/// The constructor of the Interamerican Cup (`InterAmerCup::InterAmerCup(year, record)`), run once, at career creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the year (`set_year`), the competition record at +4,
/// the vtable 0x96bad8, `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 2
/// (clubs of several nations), +0x52 = 2, 7 `substitutes_named`, 3 `substitutions_allowed`, `discipline_index` 0xff and
/// `run_by_stages` 0; the prize fund (+0x28) stays as the base left it. If the saved state loads (Cup::load_from_disk
/// 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0x9e9d94) and returning this when it fails: the two
/// finalists (InterAmerCup::select_teams 0x632480, thiscall, ret 0; the box of line 0x41 when negative); the round
/// table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x4f when
/// negative); the statistics at +8 (the box of line 0x56 without them); the final ranking (Cup::rank_teams 0x5223a0,
/// unchecked). Returns this.
pub(crate) unsafe fn build_interamerican_cup(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    this.set_year(year as u16 as i16);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x96bad8);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(2);
    write_u8(this.addr() + 0x52, 2);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 interamerican cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_INTERAMERCUP_SELECT_TEAMS); // InterAmerCup::select_teams
    if select_teams(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x41);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0x9e9d94, 0x4f);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0x9e9d94, 0x56);
        return this.addr();
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_RANK_TEAMS); // Cup::rank_teams
    rank_teams(this.addr());
    this.addr()
}

/// One place of an inter-continental cup. If the continent (key `continent`; the record [`G_CONTINENT_TABLE`] + 0xc6 ×
/// key) has a loaded league (0x6699d0(continent): push; add esp, 4 -> cdecl) or the cup's year (`cup::YEAR`) is 2001:
/// the latest club at place `place` of the competition of key `comp` (`exe::comp_util_get_latest_placed_club`(record,
/// place), cdecl; the winner is place 0), returning (club, true, false). Otherwise, the clubs (up to the first national
/// team, `exe::database_is_national_team`, cdecl) of the continent with a reputation (`club::REPUTATION`, word) above
/// `min_reputation`, in a local list; with any, the list sorted and shuffled with 1 pick
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 1), cdecl) and the first one, (club, false, false); with none,
/// the error box of line `line` and the competition's winner, which the original ALWAYS writes in place 1 (+6), even
/// when looking for place 0 (a defect: place 0 keeps whatever the allocated memory had); returns (club, false, true).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn intercontinental_cup_slot(this: usize, continent: usize, min_reputation: i16, comp: usize, place: u32, file: usize, line: u32) -> (usize, bool, bool) {
    let record = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(continent).wrapping_mul(0xc6));
    if exe::key_nation_continent_has_selected_nation(record) != 0 || read_i16(this + cup::YEAR) == 0x7d1 {
        return (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), place), true, false);
    }
    let mut list: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        if read_i16(c + club::REPUTATION) > min_reputation && intercontinental_club_in_continent(c, continent) {
            list.push(c);
        }
        c += club::SIZE;
    }
    if !list.is_empty() {
        exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, list.len() as u32, 1);
        return (list[0], false, false);
    }
    show_error_at(file, line);
    (exe::comp_util_get_latest_placed_club(comp_record_of_key(comp), 0), false, true)
}

/// The start of both cups: `cup::TEAM_COUNT` = 2 and the team table `cup::TEAM_TABLE` of 2 six-byte teams
/// (`exe::crt_malloc`(0xc), cdecl; out of memory, the error box of line 0xc6 and −1).
pub(crate) unsafe fn intercontinental_cup_team_table(this: usize, file: usize) -> Option<usize> {
    write_i16(this + cup::TEAM_COUNT, 2);
    let t = exe::crt_malloc(0xc);
    write_usize(this + cup::TEAM_TABLE, t);
    if t == 0 {
        show_error_at(file, 0xc6);
        return None;
    }
    Some(t)
}

/// The end of both cups: bytes +4, +5, +0xa and +0xb of the team table zeroed; returns 1.
pub(crate) unsafe fn intercontinental_cup_finish(this: usize) -> i32 {
    for k in [4usize, 5, 0xa, 0xb] {
        write_u8(read_usize(this + cup::TEAM_TABLE) + k, 0);
    }
    1
}

pub(crate) unsafe fn intercontinental_cup_check_clubs(name: &str, this: usize, continents: (usize, usize), o: impl FnOnce() -> i32) -> i32 {
    let r = o();
    if r == 1 {
        let t = read_usize(this + cup::TEAM_TABLE);
        let (a, b) = (read_usize(t), read_usize(t + 6));
        let bytes = [4usize, 5, 0xa, 0xb].iter().all(|&k| read_u8(t + k) == 0);
        let ok = read_i16(this + cup::TEAM_COUNT) == 2 && a != 0 && b != 0 && intercontinental_club_in_continent(a, continents.0) && intercontinental_club_in_continent(b, continents.1) && bytes;
        SCOREBOARD.record(ok, || format!("{}({:#x}): clubs {:#x} {:#x} status zeroed {}", name, this, a, b, bytes));
    }
    r
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\eurocomp\inter_amer_cup.cpp" (the error boxes).
pub(crate) const INTER_AMER_CUP_CPP: usize = 0x9e9d94;

/// The fixed fields the Interamerican Cup constructor writes (offset, width, value), checked by the comparer.
pub(crate) const INTERAMERICAN_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0xff), (0x52, 1, 0x2), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
