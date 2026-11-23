use super::*;

/// The J1 has two stages (0 and 1, each a round-robin league) and the final (stage 2): the next stage moves the current
/// stage on and, at the end of the second stage, builds the final; past the last stage it is an error (the box)
/// Original: `JapJ1::advance_stage` (0x6639c0, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j1_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9eabc8, 0x223);
    }
    let stage = stage + 1;
    this.set_current_stage((stage) as usize);
    if stage == 2 {
        exe::japj1_create_championship_final_stage(this);
    }
}

/// The J1 championship final: the first-stage winner against the second-stage winner (the club in the first row of each
/// stage's table), over two games [inference: home and away], with no seeds; unlike the Brazilian state leagues, it
/// never checks for the same club (the final is built anyway).
///
/// `JapJ1::create_championship_final_stage()`: the list of 2 (malloc `exe::crt_malloc` of 8; out of memory: the box of
/// line `0x245` and it returns); virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table: thiscall(this, stage 2,
/// &rounds, &extra, 0), ret `0x10`); the new stage (operator new `exe::crt_operator_new` of `0xb2`;
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret `0x3c`: the league, 2, the list, the rounds,
/// `.competition()`, the table, the year `.year()`, 2, 1, the extra, `0x14`, 0, 0, 0 and no seed) in the third stage
/// slot (`.stages()` + 8); the list and the table freed (free `exe::crt_free`, in that order); Cup::set_up_round
/// (`exe::cup_set_up_round`, thiscall(stage, 0), ret 4) called with the third slot's stage even when null (as in the
/// original); with no stage, the box of line `0x25c`; with it, `.set_current_stage(2)`. The original's SEH frame only
/// covers the constructor (no destructor).
/// Original: `JapJ1::create_championship_final_stage` (0x663a50, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_championship_final_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0x9eabc8, 0x245);
    }
    write_usize(list, half_champion(this, 0)); // cqq9: the first-stage champion
    write_usize(list + 4, half_champion(this, 1)); // cqq9: the second-stage champion
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 2, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year in dwords with garbage in the high 16 bits (the outputs of the
    // virtual are words on the stack; the year goes in dx); the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 2, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 8, stage);
    exe::crt_free(list);
    exe::crt_free(t);
    exe::cup_set_up_round(read_usize(this.stages() + 8), 0);
    if read_usize(this.stages() + 8) == 0 {
        return show_error_at(0x9eabc8, 0x25c);
    }
    covered!("cqq_9 final");
    this.set_current_stage((2) as usize);
}

/// The Japanese season turnover, led by the J1: first promotion and relegation with the J2; then the J1 is rebuilt with
/// three stages (the two halves and the final), next year, rules, the table and the fixtures; then the J2 turns over,
/// followed by the predicted positions, the board expectations and the referee renewal.
///
/// `JapJ1::start_new_season()`: `league::SEASON_CLOSED` = 0; promotion and relegation (virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, thiscall(this, 1), ret 4); the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); the list at `.round_dates()` freed and cleared; with statistics
/// (`.comp_stats()`), Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`,
/// thiscall(`.comp_stats()`, 1), ret 4); with a current stage ≥ 0, the existing stages 0..=`.current_stage()` destroyed
/// (virtual 0 with 1); the stage array freed and a new one of 3 (malloc `exe::crt_malloc` of `0xc`,
/// `league::STAGE_COUNT` = 3; out of memory: the box of line `0x1b4`, the clean-up `0x5cc7d0`, `0x5a8f60` and exit(−1)
/// `exe::crt_exit`, all cdecl); the year (`.year()`) goes up by one and `league::CURRENT_STAGE` = −1.
///
/// Then, each with its own box if it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`, `0x1be`), the table
/// (`0x6634e0`, `0x1c5`), the fixtures (League::create_fixtures `0x6835c0`, `0x1cb`) and the order (League::sort_table
/// `0x6827d0`, thiscall(this, 0), `0x1d2`); then the J2 turnover (virtual `+8` of the competition with id
/// `KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2`, from `G_COMP_OBJECT_TABLE`), the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations
/// `exe::league_set_club_season_expectations`) and the referee renewal (OfficialsManager::renew_officials
/// `exe::officialsmanager_renew_officials`, thiscall(`G_OFFICIALS_MANAGER`, `.competition()`), ret 4), whose eax it
/// returns.
/// Original: `JapJ1::start_new_season` (0x6636d0, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let virtual_ = |c: usize, desl: usize| -> u32 {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + desl));
        f(c)
    };
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    let cadeia: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    cadeia(this.addr(), 1);
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
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    exe::crt_free(this.stages());
    write_u32(this.addr() + league::STAGE_COUNT, 3);
    let v = exe::crt_malloc(0xc);
    this.set_stages(v);
    if v == 0 {
        show_error_at(0x9eabc8, 0x1b4);
        for f in [0x5cc7d0usize, 0x5a8f60] {
            let f: unsafe extern "C" fn() = core::mem::transmute(f);
            f();
        }
        exe::crt_exit(-1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // cqq9: the next year of the J1
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    covered!("cqq_9 j1 rollover");
    let error = if virtual_(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        0x1be
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6634e0usize)(this.addr()) == 0 {
        0x1c5
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x6835c0usize)(this.addr()) == 0 {
        0x1cb
    } else if core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, usize) -> u32>(0x6827d0usize)(this.addr(), 0) == 0 {
        0x1d2
    } else {
        0
    };
    if error != 0 {
        show_error_at(0x9eabc8, error);
        return 0;
    }
    virtual_(read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2)), 8);
    virtual_(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::league_set_club_season_expectations(this);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32 
}

/// True if the club's division (+0x57) is non-null and its id equals the competition id in global g.
pub(crate) unsafe fn is_in_division(club: Club, g: usize) -> bool {
    let comp = club.division().addr();
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// Sets +0x3e = n and allocates the league table (n x 0x3b) at +0xb1; error box on out of memory; returns the table.
pub(crate) unsafe fn alloc_league_table(this: League, n: i16, file: usize, row: u32) -> usize {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
    }
    t
}

/// League::init_table_row (`init_table_row`: thiscall(league, row, club, 0, 0), ret 0x10) for row (k + `girar`) % n with the
/// club.
///
/// `girar` is 0 (the original: row k); it exists only for each league's mutations. Simplification: the original routine
/// pushes the row in eax with spare upper bits; `init_table_row` only reads the byte.
pub(crate) unsafe fn jap_j1_init_table_row(this: League, k: u8, n: i16, girar: u8, club: usize) {
    exe::league_init_table_row(this, (k as u32 + girar as u32) % n as u32, club, 0, 0);
}

/// Japanese loop: every club of the division fills the next row while rows remain; the rest lose their division (+0x57
/// = 0); returns 1 if full.
pub(crate) unsafe fn fill_japanese_table(this: League, g: usize, girar: u8, ramo: &'static str) -> u32 {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if is_in_division(Club(club), g) {
            if (k as i8 as i16) < this.club_count() {
                jap_j1_init_table_row(this, k, this.club_count(), girar, club);
                k = k.wrapping_add(1);
            } else {
                write_usize(club + club::DIVISION, 0);
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!(ramo);
    1
}

/// Creates the J1 table: 16 clubs, those of the division in database order; those beyond 16 lose their division (left
/// with no league); when full, the prize fund starts at 100,000 per place and TV pays 3,000,000 to each club.
///
/// `JapJ1::create_table()`: J1 has 16 clubs (`club::DIVISION` = `KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1`; the rest lose the
/// division); once full, the prize fund with the 100,000 share (League::calc_prize_fund `exe::league_calc_prize_fund`:
/// push `0x186a0`; mov ecx, edi -> thiscall, ret 4) at `.set_prize_fund()`, `.set_prize_mode(1)`, the TV money of
/// 3,000,000 to each club (League::pay_tv_rights `exe::league_pay_tv_rights`: push 0; push `0x2dc6c0`; mov ecx, edi ->
/// thiscall, ret 8; it sends the TV news) and returns 1. Source `0x9eabc8`, line `0x148`.
/// Original: `JapJ1::create_table` (0x6634e0, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_table(this: usize) -> u32 {
    let this = League(this);
    if alloc_league_table(this, 0x10, 0x9eabc8, 0x148) == 0 {
        return 0;
    }
    if fill_japanese_table(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0, "cqm_9 j1 cheia") == 0 {
        return 0;
    }
    let f = exe::league_calc_prize_fund(this, 0x186a0);
    this.set_prize_fund(f);
    this.set_prize_mode(1);
    exe::league_pay_tv_rights(this, 0x2dc6c0, 0);
    1
}

/// The comparer of the three: the original routine allocates the table (and the J1 pays the TV money with a news item),
/// so only it runs.
///
/// The plan, made beforehand: the division's clubs in database order (all of them). Afterwards it checks: +0x3e = `n`,
/// the return value = 1 if the plan filled (otherwise 0), the table rows (+0 the club, 0x3b bytes) with the first `n`
/// of the plan, in order; in Japan (`limpar`), the leftovers of the plan with their division zeroed; in a full J1,
/// +0xe1 = 1 and +0x28 = the quota fund (prize fund).
pub(crate) unsafe fn compare_japanese_table(name: &str, this: League, n: i16, g: usize, clear: bool, quota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let club = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        if is_in_division(Club(club), g) {
            plan.push(club);
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let cabem = plan.len().min(n as usize);
    let expected = (plan.len() >= n as usize) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..cabem).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found[..] == plan[..cabem];
        if clear {
            same = same && plan[cabem..].iter().all(|&c| read_usize(c + club::DIVISION) == 0);
        }
        if r == 1 && quota != 0 {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, quota);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// J1: double round robin (the two stages of the year, created by add_result 0x663940 [inference]); win 3, draw 1;
/// tie-break by goal difference, goals scored and away goals; no top places in the overall table (rule word 8, no
/// champion band: the champion comes from the final between the stage winners [inference]); 2 go down to J2; no league
/// above; 5 bench substitutes and 3 substitutions; fixed squad numbers; the word +0xab = 0x82 copied to each match.
/// Then the schedule
/// Original: `JapJ1::set_up_rules` (0x663620, jap_j1.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn jap_j1_set_up_rules(this: usize) -> u32 {
    j1_rules(this);
    covered!("cqj_7 j1 rules");
    create_main_schedule(League(this));
    1
}

/// One half of the J1: when the first game of a half is processed, it builds that half's round-robin stage (a league
/// with all the clubs of the table, the same points for a win and a draw and the same tie-breakers as the league, and
/// one top place, the half winner's); the first half goes to stage 0 and the second (from round 16, by the match +0x34)
/// to stage 1; each half winner (the top of the stage table) goes to the final
/// Original: `JapJ1::create_half_season_stage` (0x663c20, jap_j1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(League(this), k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: the points per win in the J1
}

/// The schedule of the Japanese J1: stage −1, the round-robin league stage: 30 rounds from 11 Feb to 26 Oct; stage 2, the knockout or final stage: 1 round from 27 Oct to 2 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`j1_league_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `JapJ1::create_schedule` (0x662e30, jap_j1.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn jap_j1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), j1_league_schedule_stage(stage), rounds, extra, league, 0x9eabc8)
}

/// Comparer for `jap_j1_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_JAP_J1_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("jap_j1_create_schedule", stage, j1_league_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| jap_j1_create_schedule(this, stage, r, e, l))
}

/// The J1 League at career creation: the year and the database competition, discipline rule 0x13
/// (`discipline_rule::JAPAN_LEAGUE`) and, with no saved state, importance 15, the two halves and the final between the
/// half winners (three stages besides the league), the rules (points, tie-breaks and match rules, with J2 as the
/// relegation division [inference: map]), the table (the 16 J1 clubs and the prize fund [inference: map]), the
/// fixtures, the order, the statistics and the predicted positions by club strength (`bra_nat_first_construct_league`
/// with `JAP_J1_LEAGUE`).
/// Original: `JapJ1::JapJ1` (0x662b30, jap_j1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_jap_j1(this: usize, year: u32, record: usize) -> usize {
    bra_nat_first_construct_league(League(this), year, record, &JAP_J1_LEAGUE, discipline_rule::JAPAN_LEAGUE, "cqu_1 jap_j1 new") // cqu1: J1
}

/// Comparer for `construct_jap_j1`.
pub(crate) unsafe extern "thiscall" fn compare_construct_jap_j1(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_JAP_J1);
    bra_nat_first_compare_league_ctor("construct_jap_j1", League(this), year, record, &JAP_J1_LEAGUE, discipline_rule::JAPAN_LEAGUE, || o(this, year, record))
}

/// Each game result of J1: the game goes into the overall table and, if it has no stage yet, gets its half's (the first
/// up to round 14, the second from round 15); the half is built when its first game arrives
/// (`jap_j1_create_half_season_stage`, 0x663c20), and the game also goes into the half table, without counting again in
/// the league statistics; each half winner goes to the final (`bra_reg_minas_gerais_add_two_half_result`).
/// Original: `JapJ1::add_result` (0x663940, jap_j1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    bra_reg_minas_gerais_add_two_half_result(League(this), game, stats, 0xf, 0x663c20, "cqu_3 j1 half") // cqu3: the second-half round of J1
}

/// Comparer for `jap_j1_add_result`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_add_result(this: usize, game: usize, stats: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_JAP_J1_ADD_RESULT);
    bra_reg_minas_gerais_compare_two_half_result("jap_j1_add_result", League(this), game, 0xf, || o(this, game, stats))
}

/// The stages of the round table of the Japanese J1 (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 30 rounds from 11 Feb to 26 Oct; stage 2, the knockout or final stage: 1 round from 27 Oct to 2 Nov; any other stage has no table (None).
pub(crate) fn j1_league_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 30, extra: 0, extra_check: false, size: 0x79e, row: 0xab,
            ops: &J1_LEAGUE_SCHEDULE_LEAGUE_OPS, name: "cqw_4 j1 league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x11a,
            ops: &J1_LEAGUE_SCHEDULE_STAGE_2_OPS, name: "cqw_4 j1 stage 2" }),
        _ => None,
    }
}

/// The Japanese J1, the round-robin stage (−1): 30 rounds (11 Feb to 26 Oct), 1950 bytes, 60 operations.
pub(crate) const J1_LEAGUE_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 60] = [
    Cqp11D(0, 0xb, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x12, 2, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 1, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 5, 3, 0, 2, 2, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 8, 3, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0xf, 3, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x16, 3, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x1d, 3, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 3, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 6, 4, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0xd, 4, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x11, 4, 0, 2, 2, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0x14, 4, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x1b, 4, 0, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x18, 5, 0, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 1, 6, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 8, 6, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 0xf, 6, 0, 5, 1, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 0x16, 6, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x1a, 6, 0, 2, 2, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0x1d, 6, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 5, 7, 0, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0xc, 7, 0, 5, 1, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 0x13, 7, 0, 5, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 8, 0xa, 0, 2, 2, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 0xb, 0xa, 0, 5, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0x12, 0xa, 0, 5, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0x17, 0xa, 0, 3, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x1a, 0xa, 0, 6, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
];

/// The Japanese J1, stage 2: 1 round (27 Oct to 2 Nov), 104 bytes, 18 operations.
pub(crate) const J1_LEAGUE_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 0x1b, 0xa, 0, 0), Cqp11J(0, 2, 0xb, 0, 5, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11W(0x7, 0x96),
    Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0), Cqp11B(0x17, 5), Cqp11W(0x1a, 1), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0),
];

pub(crate) const JAP_J1_LEAGUE: BraNatFirstNationalLeague = BraNatFirstNationalLeague {
    vtable: 0x96ca40,
    importance: 0xf,
    importance_before_load: false,
    stages: 3,
    rules: 0x663620,
    table: 0x6634e0,
    file: 0x9eabc8,
    rows: [0x4b, 0x52, 0x59, 0x62, 0x69],
};
