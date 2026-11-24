// ---------- cx08_3: the Argentine Premier Division table, rules and half-season stages (arg_prm.cpp) ----------

/// The Argentine Premier Division has 20 clubs; prize fund by the share of 50,000 per place; TV pays 500,000 to each
/// club.
///
/// `ArgPrm::create_table()`: `league::CLUB_COUNT` = 20, the table (`exe::crt_malloc` of 20 × 0x3b; out of memory the
/// arg_prm.cpp box of line 0x1e7 and 0) with the clubs of `KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund (League::calc_prize_fund, share 0xc350), the prize mode 1, the
/// TV money (League::pay_tv_rights, 0x7a120 to every club) and 1 (`eng_prm_fill_league_table`).
/// Original: `ArgPrm::create_table` (0x405540, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x14, ARG_PRM_FILE, 0x1e7, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x7a120, 0); // cx08_3: the TV money of each Argentine club
    })
}

/// Comparer for `arg_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_nation_league_table("arg_prm_create_table", League(this), 0x14, KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION, Some(0xc350), || o(this))
}

/// Sets up the rules of the Argentine Premier Division: each pair meets once (`league::LEGS` = 1), 3 points for a win
/// and 1 for a draw, category 2; no promotion, play-off or relegation places (relegation goes by the averages); above:
/// none; below: `KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION`; tie-breaker codes 1, 2, 3, 0; squad flags 2; 5 substitutes
/// named and 3 substitutions; rule bits 0x41; then builds the main-stage round table through virtual +0x3c into
/// `league::ROUND_DATES` (`fra_first_apply_league_rules`). Returns 1.
/// Original: `ArgPrm::set_up_rules` (0x405670, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_set_up_rules(this: usize) -> u32 {
    covered!("cx08_3 argentine premier rules");
    fra_first_apply_league_rules(League(this), &arg_prm_rules())
}

/// Comparer for `arg_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("arg_prm_set_up_rules", League(this), arg_prm_rules(), || o(this), || arg_prm_set_up_rules(this))
}

/// The rule writes of the Argentine Premier Division (offset, size, value), in the original's order.
pub(crate) unsafe fn arg_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 1), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
         (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3),
         (league::TIEBREAKS + 3, 1, 0), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
         (league::RELEGATED_COUNT, 1, 0), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
         (league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION) as u32), (league::LEAGUE_FLAGS, 2, 0x41),
         (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// Builds the two half-season stages of the Argentine Premier Division.
///
/// `ArgPrm::create_stages()`: the club list (`exe::crt_malloc` of 4 × `league::CLUB_COUNT`; out of memory the box of
/// line 0x2a1 and nothing else) with the division's clubs (`KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`) in database
/// order until full. Stage 0: its round table (virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &label, 0), the
/// Clausura), a League_Stage (`exe::crt_operator_new`(0xee) and League_Stage::League_Stage 0x68b100, thiscall, ret 0x58, 0 without memory) with this league,
/// the club count and list, the legs, the competition, the table and its count, the win, draw and category bytes, the
/// tie-breaks, the place bands (`league::PROMOTED_COUNT`..), the year, 0, the label, the importance, 1, 0, the rule
/// bits, −1, the prize fund and the prize mode, into stage slot 0; the round table freed; no stage: the box of line
/// 0x2c2 and nothing else. Stage 1: the same with stage 1 (whose schedule is empty: a null table, the count left from
/// stage 0, label 0), 2 legs, rule bits 0 and prize fund 0, into slot 1; no stage: the box of line 0x2d2. Then the list
/// freed and `league::CURRENT_STAGE` = 1.
/// Defect of the original: after either box the club list is never freed (a leak of 80 bytes).
/// Original: `ArgPrm::create_stages` (0x405950, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_create_stages(this: usize) {
    let this = League(this);
    let n = this.club_count();
    let clubs = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
    if clubs == 0 {
        return show_error_at(ARG_PRM_FILE, 0x2a1);
    }
    let mut k: i16 = 0;
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let club = club_at(i as isize);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) {
            write_usize(clubs + 4 * k as usize, club.addr());
            k += 1;
            if k == this.club_count() {
                break;
            }
        }
        i += 1;
    }
    covered!("cx08_3 argentine half-season stages");
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let (mut rounds, mut label) = (0i16, 0i16);
    // (stage, legs, rule bits, prize fund, the box line); the year, the rounds and the label pass as words, the bytes as
    // bytes (the original pushes them with stale high bytes; League_Stage::League_Stage reads only the low part)
    let stages: [(u32, u32, u32, u32, u32); 2] = [
        (0, this.legs() as u16 as u32, this.league_flags() as u16 as u32, this.prize_fund() as u32, 0x2c2),
        (1, 2, 0, 0, 0x2d2),
    ];
    for (g, legs, flags, fund, row) in stages {
        let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
        let p = exe::crt_operator_new(0xee);
        let s = if p != 0 {
            ctor(p, this.addr(), n as u16 as u32, clubs, legs, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32,
                this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT,
                this.year() as u16 as u32, g, label as u16 as u32, this.importance() as u32, 1, 0, flags, -1, fund, this.prize_mode() as u32)
        } else {
            0
        };
        write_usize(this.stages() + 4 * g as usize, s);
        exe::crt_free(table);
        if read_usize(this.stages() + 4 * g as usize) == 0 {
            return show_error_at(ARG_PRM_FILE, row);
        }
    }
    exe::crt_free(clubs);
    this.set_current_stage(1);
}

/// Comparer for `arg_prm_create_stages`: only the original runs (it allocates and builds the stages); afterwards both
/// stage slots must hold a stage with the league's club count and the current stage must be 1.
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_create_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let l = League(this);
    o(this);
    let (s0, s1) = (read_usize(l.stages()), read_usize(l.stages() + 4));
    let ok = s0 != 0 && s1 != 0 && League(s0).club_count() == l.club_count() && League(s1).club_count() == l.club_count() && l.current_stage() == 1;
    SCOREBOARD.record(ok, || format!("arg_prm_create_stages({:#x}): stages {:#x} {:#x}, current {}", this, s0, s1, l.current_stage() as i32));
}
