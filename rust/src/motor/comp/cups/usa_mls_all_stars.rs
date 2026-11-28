use super::*;

/// The season rollover of a cup (`start_new_season`, slot 3, `+0x08`): frees and rebuilds entrants, rounds and ties.
///
/// This is the template for knockout cups, checked instruction by instruction: only the source file of the boxes, the
/// lines and the entrants routine change.
///
/// `+0x4c` = 0; the entrant list (`+0xa7`) and the round list (`+0xa3`) are freed (`0x9452ca`, cdecl) and zeroed; the
/// ties per round (`+0xad`: one vector per round, `+0x3c` rounds, signed word) are freed and zeroed one by one and the
/// vector itself freed and zeroed; with statistics (`+8`), Comp_Stats::archive_season_and_reset (`0x4a1c50`, push 1;
/// mov ecx -> thiscall, ret 4); the year (`+0x40`, word) goes up by `ano` (1); `+0xab` = 0 and `+0xb1` = 0 (bytes).
///
/// Then, each with its own box if it returns negative: the entrants (`inscritos`, mov ecx, esi; direct call ->
/// thiscall, ret 0), the rounds (virtual `+0x8c`) and the tie tree (virtual `+0x94`).
///
/// Finally, with `previstas`, the predicted positions (virtual `+0x5c`), whose eax it returns (the All-Star Game has
/// none: it returns the eax of the tie tree).
pub(crate) unsafe fn usa_mls_all_stars_cup_start_new_season(this: Cup, entrants: usize, file: usize, rows: [u32; 3], year: i16, predicted: bool, ramo: &'static str) -> u32 {
    this.set_season_closed(0);
    for o in [0xa7usize, 0xa3] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_usize(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    let e = read_usize(this.addr() + 8);
    if e != 0 {
        exe::comp_stats_archive_season_and_reset(e, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    covered!(ramo);
    this.set_current_round(0);
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    // simplification: in the boxes the original exits with the box’s eax (0x5e8290); the caller of the turnover does not read the return
    if entrants(this.addr()) < 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    let r = call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if !predicted {
        return r as u32;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Rule: the MLS All-Star Game season turn: tables freed, history archived, year +1, the two all-star teams, the rounds
/// and the tree (no predicted positions).
///
/// `UsaMlsAllStars::start_new_season()` (virtual +0x08 of the All-Star Game): the turn
/// (`usa_mls_all_stars_cup_start_new_season`) with the two teams from UsaMlsAllStars::set_up_teams (`set_up_teams`) and the
/// usa_mls_all_stars.cpp error boxes at lines 0x133, 0x13a and 0x141, without the predicted positions.
/// Original: `UsaMlsAllStars::start_new_season` (0x90a760, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_start_new_season_90a760(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x90a540, 0xaa20a0, [0x133, 0x13a, 0x141], 1, false, "cqr_6 virada do jogo das estrelas") // cqr6: the All-Star Game
}

/// Club record i of G_CLUB_TABLE (0x245 bytes each).
pub(crate) unsafe fn usa_mls_all_stars_club_by_index(i: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(club::SIZE))
}

/// `UsaMlsAllStars::set_up_teams()` (`set_up_teams`, thiscall, ret 0, usa_mls_all_stars.cpp): the two teams of the All-Star
/// Game; returns 1.
///
/// +0x3e = 2; the list (`exe::crt_malloc` of 0xc, cdecl; out of memory, the error box for line 0xc6 and −1) with the two
/// All-Star clubs ([0x9d00a4] and [0x9d00a8], indices into the club table): in an odd year (+0x40, signed) the one in
/// [0x9d00a4] is first (home side) and the stadium conference is 0x413; in an even year the one in [0x9d00a8] is first
/// and the conference is 0x414.
///
/// A random draw from 0 to 5 (`exe::utils_random`(6), cdecl) picks the k-th MLS club ([0x9cf590] as the club's division, +0x57) of
/// that conference (usa_mls_get_club_division `exe::usa_mls_get_club_division`, push club; add esp -> cdecl, signed word), in table order: the
/// first All-Star team takes its stadium (+0x69) and +0x6d = 0 (if there is no k-th club, nothing changes).
///
/// Both get the nation [0xae23a8] + 0x122 × [0x9cf4f8] (+0x53, the USA [inference]) and the four list bytes (+4, +5,
/// +0xa, +0xb) are zeroed.
pub(crate) unsafe fn set_up_all_star_teams(this: Cup) -> i32 {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xaa20a0, 0xc6);
        return -1;
    }
    let (a, b) = (read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST), read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST));
    let conferencia = if this.year() % 2 != 0 {
        write_usize(l, usa_mls_all_stars_club_by_index(a));
        write_usize(this.team_table() + 6, usa_mls_all_stars_club_by_index(b));
        0x413
    } else {
        write_usize(l + 6, usa_mls_all_stars_club_by_index(a));
        write_usize(this.team_table(), usa_mls_all_stars_club_by_index(b));
        0x414
    };
    let mut k = exe::utils_random(6) as u8; // cqr6: the stadium drawn among 6
    let mut i = 0usize;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = Club(usa_mls_all_stars_club_by_index(i));
        let league = c.division().addr();
        if league != 0 && read_usize(league) == read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) && ((exe::usa_mls_get_club_division(c) as u32) as i16) as i32 == conferencia {
            if k == 0 {
                covered!("cqr_6 all-star game stadium");
                let first = Club(read_usize(this.team_table()));
                write_usize(first.addr() + club::STADIUM, c.stadium().addr()); // cqr6: the stadium of the drawn club
                write_u8(read_usize(this.team_table()) + club::OWN_STADIUM, 0);
                break;
            }
            k = k.wrapping_sub(1);
        }
        i += 1;
    }
    let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_UNITED_STATES).wrapping_mul(nation::SIZE));
    let l = this.team_table();
    write_usize(read_usize(l + 6) + club::NATION, nation);
    write_usize(read_usize(l) + club::NATION, read_usize(read_usize(l + 6) + club::NATION));
    for o in [4usize, 5, 0xa, 0xb] {
        write_u8(this.team_table() + o, 0);
    }
    1
}

/// rule: the MLS All-Star Game pits the two conference all-star teams (`G_KEY_CLUB_MLS_ALL_STARS_EAST` and
/// `G_KEY_CLUB_MLS_ALL_STARS_WEST`, clubs in the table); in odd years the `G_KEY_CLUB_MLS_ALL_STARS_EAST` team is at
/// home, in even years the `G_KEY_CLUB_MLS_ALL_STARS_WEST` one; the stadium is that of an MLS club drawn (1 in 6, in
/// table order) among the home conference's clubs (0x413 in odd years, 0x414 in even ones); both teams get the USA
/// nation [inference]
/// Original: `UsaMlsAllStars::set_up_teams` (0x90a540, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_up_teams(this: usize) -> i32 {
    set_up_all_star_teams(Cup(this))
}

/// rule: the two MLS All-Star selections are called up on the day of Date(26, 6, year, 6): 26 July moved to weekday 6
/// [inference: the following Saturday]; the match is on 2 August (see cqm_12); then, every day, the cup update
/// Original: `UsaMlsAllStars::update_for_date` (0x90a900, usa_mls_all_stars.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    // simplification: the year goes in a register with garbage in the high 16 bits; 0x549f70 reads it as a word
    let year = read_i16(today + 2) as u16 as u32;
    let d = make_date_dword(0x1a, 6, year, 6); // cqn10: the All-Star Game call-up
    if stage != 0 && is_date_today(d, today) {
        covered!("cqn_10 all-star game call-up");
        exe::usamlsallstars_select_squads(this);
    }
    eng_cc_cup_cup_update_for_date(Cup(this), today, stage)
}

/// Rule: the MLS All-Star game is a single match between the two conference selections, drawn on 15/6 and played on 2/8
/// (moved to weekday 6) at the borrowed home ground [inference: venue 0], with no bracket draw (`cup_round::TIE_RULES`
/// = 0x20 [inference]); category 8; no prize. `UsaMlsAllStars::create_round_table(stage, &rounds, &extra, &out)`
/// (virtual +0x3c of the All-Star Game): outside stage −1 (the byte), 0. On −1: out (if requested) = 0, rounds = 1,
/// extra = 0; the table (`exe::crt_malloc` of 0x68; out of memory, the error box at line 0x9a and 0) with the single
/// round: the draw on day 15 of month 5 (June) moved to weekday 5, the match on day 2 of month 7 (August) moved to
/// weekday 6, leg 1, venue 0 (`exe::cup_set_round_draw_date` and `exe::cup_set_round_match_date`, cdecl, with the year
/// `.year()`); name 0, `cup_round::TIE_RULES` = 0x20, category 8 (`match_category::CONTINENTAL_LATE_ROUND`), 2 teams, 1
/// match, 2 new, 1 match per tie, no prize.
/// Original: `UsaMlsAllStars::create_round_table` (0x90a400, usa_mls_all_stars.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 1;
    *extra = 0;
    let t = CupRound(exe::crt_malloc(0x68));
    if t.is_null() {
        show_error_at(0xaa20a0, 0x9a);
        return 0;
    }
    covered!("cqm_12 all-star game fixture");
    let year = this.year() as i32 as u32;
    exe::cup_set_round_draw_date(t, 0, 0xf, 5, 0, 5, year);
    exe::cup_set_round_match_date(t, 0, 2, 7, 0, 6, 1, year, 0); // cqm12: the All-Star Game on 2 August
    let w = |o: usize, v: u16| write_u16(t.addr() + o, v);
    let d = |o: usize, v: u32| write_u32(t.addr() + o, v);
    w(cup_round::TEAMS, 2);
    w(cup_round::NEW_TEAMS, 2);
    w(7, 0);
    w(9, 0);
    w(cup_round::DRAW_RULES, 0);
    w(cup_round::TIE_RULES, 0x20);
    t.set_match_category(match_category::CONTINENTAL_LATE_ROUND);
    w(cup_round::TIES, 1);
    w(cup_round::TWO_LEG_TIE_RULES, 0);
    w(cup_round::FIRST_NEW_TEAM, 0);
    t.set_single_leg_flag(0);
    t.set_legs(1);
    t.set_leg_gap_days(0);
    d(0x5c, 0);
    d(cup_round::WINNER_PRIZE, 0);
    d(cup_round::LOSER_PRIZE, 0);
    t.addr()
}

/// The MLS All-Star Game at career creation: the year and the database competition; a new game has the two all-star
/// teams (clubs of one nation), discipline rule 255, no prize fund of its own, the rounds, the tie tree and the
/// statistics (see `build_mls_all_star_game`).
/// Original: `UsaMlsAllStars::UsaMlsAllStars` (0x90a150, usa_mls_all_stars.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    build_mls_all_star_game(Cup(this), year, record) // cqw5: the MLS All-Star Game
}

/// Comparer for `construct_usa_mls_all_stars`: only the original runs (it allocates, draws and reads the save file),
/// then the plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_usa_mls_all_stars(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_MLS_ALL_STARS)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x9708f8, &MLS_ALL_STAR_GAME_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_mls_all_stars({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The plan the comparer checks after the original constructor ran; `None` when it holds, otherwise the difference.
///
/// Always: the return value is `this`, the vtable, the competition record at +4 and the year (`cup::YEAR`, word). On a
/// new cup (`cup::STAGE_INDEX` = 0xff), every fixed field the constructor writes (`fields`: offset, width in bytes,
/// value), the round table (`cup::ROUND_TABLE`, when `rounds`) and the statistics at +8 must be set.
pub(crate) unsafe fn cup_constructor_plan_mismatch(this: usize, r: usize, year: u32, record: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != stage_index::MAIN || league_will_load_saved_state(record) {
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

/// The constructor of the MLS All-Star Game (`UsaMlsAllStars::UsaMlsAllStars(year, record)`), run once, at career
/// creation.
///
/// On the Cup base (Cup::Cup `exe::new_cup`, thiscall, ret 0): the competition record at +4, the vtable 0x9708f8, the
/// year (`set_year`), `current_round` 0, `stage_index` 0xff, `stage_count` 0, `current_stage` −1, `club_sides` 1 (clubs
/// of one nation), `discipline_index` 0xff and `run_by_stages` 0; substitutes, substitutions and the prize fund stay as
/// the base left them. If the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret
/// 4; returns nonzero), returns this.
///
/// On a new cup, in order, each step with its error box (source 0xaa20a0) and returning this when it fails: the two
/// all-star teams (UsaMlsAllStars::set_up_teams 0x90a540, thiscall, ret 0; the box of line 0x44 when negative); the
/// round table (`build_cup_round_table_by_vtable`); the tie tree (Cup::map_fixture_tree 0x518790; the box of line 0x52
/// when negative); the statistics at +8 (the box of line 0x59 without them). No final ranking. Returns this.
pub(crate) unsafe fn build_mls_all_star_game(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), 0x9708f8);
    this.set_year(year as u16 as i16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    this.set_club_sides(1);
    this.set_discipline_index(0xff);
    this.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqw_5 mls all-star game new");
    let set_up_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_USAMLSALLSTARS_SET_UP_TEAMS); // UsaMlsAllStars::set_up_teams
    if set_up_teams(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x44);
        return this.addr();
    }
    build_cup_round_table_by_vtable(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    if map_fixture_tree(this.addr()) < 0 {
        show_error_at(0xaa20a0, 0x52);
        return this.addr();
    }
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(0xaa20a0, 0x59);
        return this.addr();
    }
    this.addr()
}

/// Builds a new cup's round table through virtual `comp_vtable::CREATE_SCHEDULE` and stores it in `round_table()`.
///
/// The call is `create_schedule(−1, &round_count, &stage_label, 0)` (push 0; push &+0x3a; push &+0x3c; push −1; mov
/// ecx, this -> thiscall, ret 0x10): stage −1, the rounds into `cup::ROUND_COUNT` and the extra into
/// `cup::STAGE_LABEL`, no is_league output.
pub(crate) unsafe fn build_cup_round_table_by_vtable(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_u32(this.addr() + cup::ROUND_TABLE, t as u32);
}

/// The fixed fields the MLS All-Star Game constructor writes (offset, width, value), checked by the comparer.
pub(crate) const MLS_ALL_STAR_GAME_CTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xff), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// rule: the two MLS All-Star selections are called up from the clubs in the first four places: the East from the
/// MLS's own table and from the first conference table's clubs of the East conference, the West from the second
/// conference table and from the first conference table's clubs of the West conference; each selection takes the fit
/// players free of international duty on the day of the game, sorted, at most 127 of them, and picks a squad of 20 by
/// position (`ALL_STAR_SQUAD_POSITIONS`).
///
/// The game day: the first round's date (`cup::ROUND_TABLE`: the day +3 and the year offset +5 on `cup::YEAR`;
/// Date::Date `exe::new_date`, thiscall, ret 8). The MLS (`G_COMP_OBJECT_TABLE`[`KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`])
/// and its stages 0 and 1 (`cup::STAGES`). The East (`collect_all_star_candidates` over the MLS table and stage 0, the
/// conference 0x413; `fill_all_star_squad` into `G_KEY_CLUB_MLS_ALL_STARS_EAST`, the box of line 0x1dc), then the West
/// (over stage 1 and stage 0, 0x414; into `G_KEY_CLUB_MLS_ALL_STARS_WEST`, line 0x221). The original keeps both lists of
/// 300 on the stack one after the other: the copy's are larger, which only matters past 300 candidates.
/// Original: `UsaMlsAllStars::select_squads` (0x90a980, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_select_squads(this: usize) {
    let cup = Cup(this);
    let mls = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) * 4));
    let first_stage = League(read_usize(mls.stages()));
    let second_stage = League(read_usize(mls.stages() + 4));
    let round = cup.round_table();
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(round + cup_round::DAY) as u32,
        (read_u8(round + cup_round::YEAR_OFFSET) as i8 as i16).wrapping_add(cup.year()) as u16 as u32);
    covered!("cx03_20 all-star squads");
    let mut east = [0usize; 400];
    let n = collect_all_star_candidates(&mut east, mls, first_stage, 0x413, &date);
    fill_all_star_squad(&mut east, n, G_KEY_CLUB_MLS_ALL_STARS_EAST, 0x1dc);
    let mut west = [0usize; 400];
    let n = collect_all_star_candidates(&mut west, second_stage, first_stage, 0x414, &date);
    fill_all_star_squad(&mut west, n, G_KEY_CLUB_MLS_ALL_STARS_WEST, 0x221);
}

/// Comparer for `usa_mls_all_stars_select_squads`: side by side over the two All-Star clubs' squads (the 50 slots each);
/// the injury and duty checks and the position picker only read.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_all_stars_select_squads(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_ALL_STARS_SELECT_SQUADS);
    let east = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) as isize).addr() + 0xd7;
    let west = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) as isize).addr() + 0xd7;
    let (_, _, same, mo, mc) = crate::comparison::side_by_side(&[(east, 200), (west, 200), (0xb67a34, 4)], || o(this), || usa_mls_all_stars_select_squads(this));
    SCOREBOARD.record(same, || format!("usa_mls_all_stars_select_squads({:#x}): original {:x?} copy {:x?}", this, mo, mc));
}

/// The players one All-Star selection can call up: for each of the first four rows of the tables, two passes; the
/// first over `first`'s row, the second over the same row of `second` when that club's All-Star conference
/// (`usa_mls_get_club_all_star_conference`) is `conference`; each player of the club's squad (`club.squad`, 50 slots)
/// with a player record, not out injured (`exe::injury_get_player_injury_status` ≠ 2), free of international duty on
/// `date` (`exe::nationalteams_get_international_duty_club`(…, player, date, 0, 0, 0) = 0) and still at that club
/// (`staff.club_job()`) joins `list`. A club met in both passes is collected twice, as in the original. Returns the count
/// (a signed word in the original).
pub(crate) unsafe fn collect_all_star_candidates(list: &mut [usize], first: League, second: League, conference: u16, date: &[u16; 4]) -> i16 {
    let mut n: i16 = 0;
    for row in 0..4usize {
        for pass in 0..2 {
            let table = if pass == 0 { first } else { second };
            let club = read_usize(table.table() + row * league_table_row::SIZE);
            if pass == 1 && usa_mls_get_club_all_star_conference(club) != conference {
                continue;
            }
            for slot in 0..50usize {
                let person = Staff(Club(club).squad(slot));
                if person.is_null() || person.player().is_null() {
                    continue;
                }
                if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
                    continue;
                }
                if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, date.as_ptr() as usize, 0usize, 0, 0) != 0 {
                    continue;
                }
                if person.club_job().addr() == club {
                    list[n as usize] = person.addr();
                    n = n.wrapping_add(1);
                }
            }
        }
    }
    n
}

/// Fills one All-Star club's squad: the club (`club_at` of the id kept in `key`) loses its whole squad (the 50 slots
/// cleared); the candidates are sorted (`exe::crt_qsort`, 4-byte entries, `qsort_comparator_squad_by_player_0xb`
/// 0x5402f0); at most 127 are used, and fewer than 30 show the box of `row` (the squad is still filled); then the squad
/// is filled from them (`exe::match_man_assign_players_to_positions`(list, n, the 20 `ALL_STAR_SQUAD_POSITIONS`, 20, 0,
/// the club's squad, 2, then eight zeros), cdecl).
pub(crate) unsafe fn fill_all_star_squad(list: &mut [usize], n: i16, key: usize, row: u32) {
    let club = club_at(read_usize(key) as isize);
    for slot in 0..50usize {
        write_usize(club.addr() + 0xd7 + slot * 4, 0); // the squad slots
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x5402f0usize);
    let mut n = n;
    if n > 0x7f {
        n = 0x7f;
    } else if n < 0x1e {
        show_error_at(0xaa20a0, row);
    }
    exe::match_man_assign_players_to_positions(list.as_mut_ptr() as usize, n as i32 as u32, ALL_STAR_SQUAD_POSITIONS.as_ptr() as usize, 0x14, 0,
        club.addr() + 0xd7, 2, 0usize, 0, 0, 0, 0, 0, 0, 0);
}

/// The 20 position masks of an All-Star squad, in the order `match_man_assign_players_to_positions` fills them.
pub(crate) const ALL_STAR_SQUAD_POSITIONS: [u16; 20] = [1, 1, 1, 0x804, 0x84, 4, 4, 0x204, 0x204, 0x204, 0x810, 0x90, 0x10, 0x10, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
