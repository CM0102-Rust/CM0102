// ---------- batch 305: the season turnovers of the Veikkausliiga, the Tippeligaen and the Allsvenskan (fin_prm.cpp, nor_prm.cpp, swe_prm.cpp) ----------

/// Calls the start_new_season virtual (`comp_vtable::START_NEW_SEASON`, mov ecx -> thiscall, ret 0) of the
/// competition object of key `g` (`G_COMP_OBJECT_TABLE` by its id), when there is one.
pub(crate) unsafe fn start_new_season_of(g: usize) {
    let c = comp_by_global(g);
    if c != 0 {
        bra_nat_first_call_virtual_no_args(c, comp_vtable::START_NEW_SEASON);
    }
}

/// The promotion and relegation chain of the league (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, push
/// `pairs`; mov ecx -> thiscall, ret 4).
pub(crate) unsafe fn promotion_relegation_chain(this: League, pairs: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    f(this.addr(), pairs);
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn call_on_league(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The common body of the top-division turnovers (no extra stages): `pre` (the promotion and relegation steps), then
/// the table freed (`exe::league_free_table`(0)), the round dates freed and zeroed, the season archived in the
/// statistics (`exe::comp_stats_archive_season_and_reset`(1), when there are statistics), the year + 1; then, each with
/// the box of `file` at its line when it returns 0 (and 0 returned): the rules (virtual `comp_vtable::SET_UP_RULES`), the
/// table (`table`, direct), `exe::league_create_fixtures` and `exe::league_sort_table`(0); then `post` (the lower
/// divisions' turnovers), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations (League::set_club_season_expectations) and the referee renewal (OfficialsManager::renew_officials on
/// `G_OFFICIALS_MANAGER` with the record), whose eax is returned. Unlike the lower divisions' rebuild, the stages are not
/// destroyed and the current stage is not reset (these leagues have none).
pub(crate) unsafe fn top_division_turnover(this: League, pre: impl FnOnce(), table: usize, file: usize, rows: [u32; 4], post: impl FnOnce()) -> u32 {
    this.set_season_closed(0);
    pre();
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // mwe14: the top division next year
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let t: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if t(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    post();
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// The Finnish turnover, led by the Veikkausliiga: the promotion and relegation chain (with 0), the two unnamed Finnish
/// steps 0x595f00 and 0x596050 [inference: the First Division groups refreshed and regrouped], the Veikkausliiga rebuilt
/// for the next year (rules, the 12-club table, fixtures, order), then the Ykkönen turnover, the predicted positions,
/// the board expectations and the referee renewal.
///
/// `FinPrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(0) and the direct calls 0x595f00 and 0x596050 (thiscall, ret 0);
/// the table `FinPrm::create_table` (0x595b70) and the boxes of fin_prm.cpp (0x9c8254) at 0x1c7, 0x1ce, 0x1d4, 0x1db;
/// after, the Ykkönen's start_new_season (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`, called without a null check).
/// Original: `FinPrm::start_new_season` (0x595d40, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 finnish turnover");
        promotion_relegation_chain(this, 0); // mwe14: the Finnish chain
        call_on_league(0x595f00, this);
        call_on_league(0x596050, this);
    }, 0x595b70, 0x9c8254, [0x1c7, 0x1ce, 0x1d4, 0x1db], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON); // mwe14: the Ykkonen turnover
    })
}

/// The Norwegian turnover, led by the Tippeligaen: the promotion and relegation chain (with 1), the First and Second
/// Division clubs swapped (`nor_prm_swap_first_and_second_division_clubs` 0x792960) and the unnamed step 0x792b40
/// [inference: the Second Division groups regrouped], the Tippeligaen rebuilt for the next year, then the First
/// Division turnover, the predicted positions, the board expectations and the referee renewal.
///
/// `NorPrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) and the direct calls 0x792960 and 0x792b40 (thiscall, ret 0);
/// the table `NorPrm::create_table` (0x7925e0) and the boxes of nor_prm.cpp (0xa6977c) at 0x212, 0x219, 0x21f, 0x226;
/// after, the Norwegian First Division's start_new_season (`KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION`, no null check).
/// Original: `NorPrm::start_new_season` (0x7927a0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 norwegian turnover");
        promotion_relegation_chain(this, 1); // mwe14: the Norwegian chain
        call_on_league(0x792960, this); // mwe14: the Norwegian First and Second Division clubs swapped
        call_on_league(0x792b40, this);
    }, 0x7925e0, 0xa6977c, [0x212, 0x219, 0x21f, 0x226], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// The Swedish turnover, led by the Allsvenskan: the promotion and relegation chain (with 0); with a playable Second
/// Division the unnamed step 0x890b40, otherwise three regional clubs promoted to the Superettan
/// (`swe_prm_promote_regional_clubs_to_first_division` 0x890980); the division tiers limited
/// (League::limit_club_division_tier(1, 3, −3, 1)) and the Second Division sections dealt again
/// (`swe_prm_assign_second_division_sections` 0x890c70); the Allsvenskan rebuilt for the next year; then the
/// Superettan turnover and the Second Division's (when it exists), the predicted positions, the board expectations and
/// the referee renewal.
///
/// `SwePrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(0), 0x890b40 or 0x890980 by the Second Division object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`), `exe::league_limit_club_division_tier`(1, 3, −3,
/// 1) and 0x890c70 (all thiscall); the table `SwePrm::create_table` (0x890430) and the boxes of swe_prm.cpp (0xa8b578)
/// at 0x1ba, 0x1c1, 0x1c7, 0x1ce; after, the Superettan's start_new_season (no null check) and the Second Division's
/// (only when it exists).
/// Original: `SwePrm::start_new_season` (0x8905f0, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 swedish turnover");
        promotion_relegation_chain(this, 0);
        if comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION) != 0 {
            covered!("mwe_14 swedish second division playable");
            call_on_league(0x890b40, this);
        } else {
            call_on_league(0x890980, this); // mwe14: the regional clubs promoted
        }
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        call_on_league(0x890c70, this);
    }, 0x890430, 0xa8b578, [0x1ba, 0x1c1, 0x1c7, 0x1ce], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    })
}

/// Comparer for `fin_prm_start_new_season`: only the original runs (it frees, allocates and moves clubs); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_PRM_START_NEW_SEASON);
    compare_state_league_new_season("fin_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `nor_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("nor_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `swe_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("swe_prm_start_new_season", League(this), || o(this))
}

// ---------- batch 305: the Portuguese turnover and its promotion and relegation chain (por_prm.cpp) ----------

/// The Portuguese promotion and relegation chain: the final positions of the Premier Division and the Liga de Honra are
/// recorded and the marked clubs swapped between them; when the Second Division B is played, its final positions and
/// those of its two other zones are recorded and the Liga de Honra swaps clubs with each of the three zones.
///
/// `PorPrm::process_promotion_relegation_chain(pairs)` (`pairs` is not read): virtual
/// `league_vtable::RECORD_FINAL_POSITIONS` (thiscall, ret 0) of the Premier Division and of the Liga de Honra
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION` and `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION`);
/// League::promote_and_relegate (`exe::league_promote_and_relegate`: push −1; push −1; push 1; push 1; push lower; push
/// upper; mov ecx, this -> thiscall, ret 0x18) between them. With the Second Division B object
/// (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`): its RECORD_FINAL_POSITIONS and those of its stages 0 and 1 (the
/// other two zones, `league::STAGES`), then promote_and_relegate between the Liga de Honra and the Second Division B,
/// and between the Liga de Honra and each of the two stages; the last call's eax is returned. Without it, the
/// original returns the value of `G_COMP_OBJECT_TABLE` (the register left from the lookup).
/// Original: `PorPrm::process_promotion_relegation_chain` (0x7cfbf0, por_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn por_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (premier, second) = (comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION));
    bra_nat_first_call_virtual_no_args(premier, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(second, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwe_15 portuguese promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), 1, 1, -1, -1); // mwe15: the Premier Division and the Liga de Honra
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwe_15 portuguese second division b");
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    let zone = |k: usize| read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) + league::STAGES) + 4 * k);
    bra_nat_first_call_virtual_no_args(zone(0), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(zone(1), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(0), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), zone(1), 1, 1, -1, -1) as u32
}

/// The Portuguese turnover, led by the Premier Division: the B team checks when the Second Division B is played, the
/// promotion and relegation chain (with 1); with the Second Division B two unnamed steps, otherwise the marked Liga de
/// Honra clubs exchanged with the regional zones (`PorPrm::exchange_second_division_with_zones`); the Premier Division
/// rebuilt for the next year (rules, the 18-club table, fixtures, order); then the Liga de Honra turnover and the Second
/// Division B's (when it is played), the predicted positions, the board expectations and the referee renewal.
///
/// `PorPrm::start_new_season()`: `league::SEASON_CLOSED` = 0; with the Second Division B object
/// (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`), the step 0x7d0320 [inference: the B team
/// checks]; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1); with the Second Division B, the steps
/// 0x7cff70 and 0x7d0210, otherwise 0x7cfd30 (all direct, thiscall, ret 0); then the body of `top_division_turnover`
/// (the table `PorPrm::create_table` 0x7cf800, the boxes of por_prm.cpp (0xa78724) at 0x227, 0x22e, 0x234, 0x23b) with,
/// after the rebuild, the Liga de Honra's start_new_season (no null check) and the Second Division B's (when it exists).
/// Original: `PorPrm::start_new_season` (0x7cf9e0, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    portuguese_top_division_turnover(this, || {
        covered!("mwe_15 portuguese turnover");
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7d0320, this);
        }
        let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
        chain(this.addr(), 1);
        if comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B) != 0 {
            portuguese_step(0x7cff70, this);
            portuguese_step(0x7d0210, this);
        } else {
            portuguese_step(0x7cfd30, this); // mwe15: the Liga de Honra exchanged with the zones
        }
    })
}

/// A direct call with no arguments on the league (thiscall, ret 0).
pub(crate) unsafe fn portuguese_step(f: usize, this: League) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
    f(this.addr());
}

/// The rest of the Portuguese turnover after `pre`: the table and round dates freed, the stats archived, the year + 1,
/// the rules, the table, the fixtures and the order (each with its box of por_prm.cpp), the Liga de Honra's and the
/// Second Division B's start_new_season, the predicted positions, the expectations and the referees (see
/// `por_prm_start_new_season`).
pub(crate) unsafe fn portuguese_top_division_turnover(this: League, pre: impl FnOnce()) -> u32 {
    this.set_season_closed(0);
    pre();
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // mwe15: the Premier Division next year
    let file = 0xa78724;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x227);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7cf800usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x22e);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x234);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x23b);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    let b = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B);
    if b != 0 {
        bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `por_prm_process_promotion_relegation_chain`: only the original runs (it moves the clubs; the same as
/// the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_POR_PRM_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Comparer for `por_prm_start_new_season`: only the original runs (it frees, allocates and moves clubs); the year must
/// go up by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("por_prm_start_new_season", League(this), || o(this))
}

// ---------- batch 305: the season turnovers of the Finnish, Norwegian and Portuguese cups (fin_cup.cpp, nor_cup.cpp, por_cup.cpp) ----------

/// The Finnish Cup turnover: last season's entrant, round and tie tables freed, the season archived in the statistics,
/// the year + 1; then the entrants (FinCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// `FinCup::start_new_season()`: the body of `start_new_cup_season` (checked instruction by instruction: only the
/// entrants routine 0x592a20, the source fin_cup.cpp (0x9c8158) and the box lines 0x206, 0x20d, 0x214 change): +0x4c =
/// 0; `cup::TEAM_TABLE` and the round list freed and zeroed; the tie arrays (one per round, `.round_count()` of them)
/// and their vector freed and zeroed; `exe::comp_stats_archive_season_and_reset`(1); the year + 1; the current round
/// and the stage flag 0; the entrants (direct, thiscall, ret 0), the rounds (virtual `comp_vtable::SET_UP_RULES`) and
/// the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`), each with its box when it returns a negative value (and 0
/// returned); last the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is returned.
/// Original: `FinCup::start_new_season` (0x592d00, fin_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x592a20, 0x9c8158, [0x206, 0x20d, 0x214], 1, "mwe_16 finnish cup turnover") // mwe16: the Finnish Cup next year
}

/// The Norwegian Cup turnover: the same as the Finnish Cup's, with NorCup::collect_entrants (0x78f890), nor_cup.cpp
/// (0xa69708) and the box lines 0x1e8, 0x1ef, 0x1f6.
/// Original: `NorCup::start_new_season` (0x78fb90, nor_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x78f890, 0xa69708, [0x1e8, 0x1ef, 0x1f6], 1, "mwe_16 norwegian cup turnover") // mwe16: the Norwegian Cup next year
}

/// The Portuguese Cup turnover: the same as the Finnish Cup's, with PorCup::collect_entrants (0x7ce220), por_cup.cpp
/// (0xa786f0) and the box lines 0x20d, 0x214, 0x21b.
/// Original: `PorCup::start_new_season` (0x7ce540, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7ce220, 0xa786f0, [0x20d, 0x214, 0x21b], 1, "mwe_16 portuguese cup turnover") // mwe16: the Portuguese Cup next year
}

/// Comparer for `fin_cup_start_new_season`: only the original runs (it frees and allocates); the year must go up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_fin_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_CUP_START_NEW_SEASON);
    compare_cup_new_season("fin_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `nor_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_CUP_START_NEW_SEASON);
    compare_cup_new_season("nor_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `por_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_por_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_CUP_START_NEW_SEASON);
    compare_cup_new_season("por_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 305: the Portuguese Super Cup: its teams, its round table and its turnover (por_super.cpp) ----------

/// The Portuguese Super Cup round outside 2000: draw on 5 July (weekday 5), one match on 12 August (weekday 5) at
/// venue 4 [inference: a neutral ground], tie rules 3, category 6, two clubs, a single leg (+0x20 = 1, one leg) with
/// the gap byte 0x7e, no prizes; +0x11 = 3 is written apart (`write_portuguese_super_cup_round`).
pub(crate) const PORTUGUESE_SUPER_CUP: EngCcCupCupRound = cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (1, 1, 0x7e), (0, 0, 0)); // mwe17: the Portuguese Super Cup date

/// The Portuguese Super Cup round in 2000: the same, with venue 0 (the clubs' grounds), no tie rules and two legs.
pub(crate) const PORTUGUESE_SUPER_CUP_2000: EngCcCupCupRound = cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 0), 0, (0, 0, 0), 6, (2, 1, 2, 0), (1, 2, 0x7e), (0, 0, 0));

/// Writes the Super Cup round 0 of table `t` for the cup's year: the round of `PORTUGUESE_SUPER_CUP_2000` in 2000
/// (`.year()` = 0x7d0), otherwise `PORTUGUESE_SUPER_CUP` (`eng_cc_cup_write_cup_round`, the year sign-extended), and the
/// word +0x11 = 3 [unnamed].
pub(crate) unsafe fn write_portuguese_super_cup_round(this: Cup, t: usize) {
    let r = if this.year() as u16 == 0x7d0 { &PORTUGUESE_SUPER_CUP_2000 } else { &PORTUGUESE_SUPER_CUP };
    eng_cc_cup_write_cup_round(t, 0, r, this.year() as i32 as u32);
    write_u16(t + 0x11, 3);
}

/// The Portuguese Super Cup teams: the Premier Division champion and the Portuguese Cup winner; if the same club won
/// both, the cup runner-up takes the second place; if a club is missing, the error box.
///
/// `PorSuper::select_teams()`: `.set_team_count()` = 2, the entrant list (`exe::crt_malloc` of 0xc into
/// `cup::TEAM_TABLE`; out of memory: the box of por_super.cpp (0xa787d4) line 0xca and −1). Entry 0 = the last
/// champion of the Premier Division (`latest_placed_club`(`KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`, 0):
/// `exe::comp_util_get_latest_placed_club`, push place; push record -> cdecl); none: the box of line 0xd3 and 0. Entry 1
/// (+6) = the last Portuguese Cup winner (`KEY_CLUB_COMP_PORTUGUESE_CUP`, 0); none: the box of 0xdc and 0. If both are
/// the same club, entry 1 = the cup runner-up (place 1); none: the box of 0xe8 and 0. Class and status bytes 0 for
/// both; returns 1.
/// Original: `PorSuper::select_teams` (0x7d3260, por_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_super_select_teams(this: usize) -> i32 {
    portuguese_super_cup_select_teams(Cup(this)).0
}

/// The body of `por_super_select_teams`; also returns the number of clubs written (2, or 0 after a box).
pub(crate) unsafe fn portuguese_super_cup_select_teams(this: Cup) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa787d4, 0xca);
        return (-1, 0);
    }
    let a = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0); // mwe17: the Portuguese champion
    write_usize(l, a);
    if a == 0 {
        show_error_at(0xa787d4, 0xd3);
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_CUP, 0);
    write_usize(l + 6, b);
    if b == 0 {
        show_error_at(0xa787d4, 0xdc);
        return (0, 0);
    }
    if a == b {
        covered!("mwe_17 portuguese double");
        let b = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_CUP, 1);
        write_usize(l + 6, b);
        if b == 0 {
            show_error_at(0xa787d4, 0xe8);
            return (0, 0);
        }
    }
    covered!("mwe_17 portuguese super cup clubs");
    write_u8(l + 4, 0);
    write_u8(l + 5, 0);
    write_u8(l + 0xa, 0);
    write_u8(l + 0xb, 0);
    (1, 2)
}

/// The Portuguese Super Cup round table: one round, a single match on 12 August at a neutral ground (in 2000, two legs
/// at the clubs' grounds), drawn on 5 July.
///
/// `PorSuper::create_round_table(stage, &rounds, &extra, &out)`: at stage −1 (0xff), `*out` = 0 (when given), 1 round,
/// extra 0, the table (`exe::crt_malloc` of 0x68; out of memory: the box of por_super.cpp (0xa787d4) line 0x95 and 0)
/// with round 0 by `write_portuguese_super_cup_round` (Cup round draw and match dates, cdecl, and the fields); any other
/// stage: 0.
/// Original: `PorSuper::create_round_table` (0x7d30f0, por_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 1;
    *extra = 0;
    let t = exe::crt_malloc(0x68);
    if t == 0 {
        show_error_at(0xa787d4, 0x95);
        return 0;
    }
    covered!("mwe_17 portuguese super cup round");
    write_portuguese_super_cup_round(Cup(this), t);
    t
}

/// The Portuguese Super Cup turnover: last season's team list, rounds and ties freed, the season archived, the year + 1;
/// then the two teams (`PorSuper::select_teams`), the round table and the tie tree; unlike the other cups, no predicted
/// positions (the tie tree's eax is returned).
///
/// `PorSuper::start_new_season()`: +0x4c = 0; `cup::TEAM_TABLE` and the round list freed and zeroed; the tie arrays
/// (one per round) and their vector freed and zeroed; `exe::comp_stats_archive_season_and_reset`(1); the year + 1; the
/// current round and the stage flag 0; `PorSuper::select_teams` (0x7d3260, direct), the rounds (virtual
/// `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`), each with the box of
/// por_super.cpp (0xa787d4) at 0x12d, 0x134, 0x13b when it returns a negative value (and 0 returned); the tie tree's
/// value is returned.
/// Original: `PorSuper::start_new_season` (0x7d3490, por_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_super_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, 0xa3usize] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_u32(this.tie_tables() + i as usize * 4, 0);
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
    this.set_year(this.year().wrapping_add(1)); // mwe17: the Super Cup next year
    covered!("mwe_17 portuguese super cup turnover");
    this.set_current_round(0);
    this.set_run_by_stages(0);
    let teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x7d3260usize);
    if teams(this.addr()) < 0 {
        show_error_at(0xa787d4, 0x12d);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0xa787d4, 0x134);
        return 0;
    }
    let r = call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(0xa787d4, 0x13b);
        return 0;
    }
    r as u32
}

/// Comparer for `por_super_select_teams`: side by side (`eng_cc_cup_compare_entrant_list`: the count, the mouse flag
/// written by the boxes and the 12 bytes of the two entries when it returns 1; the copy's list freed).
pub(crate) unsafe extern "thiscall" fn compare_por_super_select_teams(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POR_SUPER_SELECT_TEAMS);
    eng_cc_cup_compare_entrant_list("por_super_select_teams", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || portuguese_super_cup_select_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

/// Comparer for `por_super_create_round_table`: side by side (`compare_cup_round_table`: the outputs and the written
/// round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_POR_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("por_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| por_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `por_super_start_new_season`: only the original runs (it frees and allocates); the year + 1
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POR_SUPER_START_NEW_SEASON);
    compare_cup_new_season("por_super_start_new_season", Cup(this), || o(this))
}
