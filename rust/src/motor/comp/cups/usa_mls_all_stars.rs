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
