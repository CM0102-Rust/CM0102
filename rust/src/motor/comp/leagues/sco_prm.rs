use super::*;

/// rule: the Scottish Premier Division plays 38 rounds (12 clubs): 33 in three rounds from 29 July to 7 April of the next
/// year, Saturdays with the TV slot (5, 2, 2) in most rounds and three midweek rounds (17 October, 29 November, 2
/// January), the year-end block of two rounds chosen by the day of the 21st round (20 to 23 December and 26 December);
/// then the split (stages 0 and 1: the top six and the bottom six, five Saturdays from 14 April to 12 May, stage label
/// 0x44b or 0x44c). In 1999 (10 clubs, 4 rounds): 36 rounds and no split. Any other stage: nothing and 0
/// Original: `ScoPrm::create_schedule` (0x7f0e80, sco_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let stage = stage as u8;
    if this.year() == 0x7cf {
        if stage != 0xff {
            return 0;
        }
        covered!("mwb_14 scottish premier calendar 1999");
        return scottish_premier_round_table(this, rounds, extra, league, &SCOTTISH_PREMIER_1999_SCHEDULE);
    }
    if stage == 0xff {
        covered!("mwb_14 scottish premier calendar");
        return scottish_premier_round_table(this, rounds, extra, league, &SCOTTISH_PREMIER_SCHEDULE);
    }
    if stage != 0 && stage != 1 {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 5;
    *extra = 0x44b + (stage != 0) as i16;
    let t = exe::crt_malloc(0x145);
    if t == 0 {
        show_error_at(0xa7af70, 0x279);
        return 0;
    }
    covered!("mwb_14 scottish premier split calendar");
    eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &SCOTTISH_PREMIER_SPLIT);
    t
}

/// Comparer for `sco_prm_create_schedule` (side by side, the written rounds; the split tables by their 0x41-byte rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_PRM_CREATE_SCHEDULE);
    if stage as u8 == 0xff {
        return compare_league_round_table("sco_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l));
    }
    compare_league_round_table("sco_prm_create_schedule", 0xff, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_prm_create_schedule(this, stage, r, e, l))
}

/// rule: the Scottish turnover, led by the Premier Division: the promotion and relegation step (virtual +0xb0 with 0);
/// in 1999 Elgin City and Peterhead join the Third Division (the 2000 expansion); the Premier's old table, rounds and
/// stages are freed, the season archived, the year moves on, with no stages beyond the league; the rules, the table, the
/// fixtures and the order (each with its error box: lines 0x338, 0x33f, 0x345, 0x34c, and 0); the division tier of the
/// Second Division clubs capped (League::limit_club_division_tier 2, −3, 3, 0) and of the Third's (2, 0, −3, 0); then the
/// First, the Second and the Third turn over (virtual +8), and the Premier's predicted positions, the board expectations
/// and the referee renewal, whose eax it returns
/// Original: `ScoPrm::start_new_season` (0x7f2a70, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let league = this;
    let call = |o: usize, slot: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + slot));
        f(o)
    };
    this.set_season_closed((0) as usize);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 0);
    if league.year() == 0x7cf {
        covered!("mwb_15 scottish expansion 1999");
        for g in [ELGIN_CITY_INDEX_GLOBAL, PETERHEAD_INDEX_GLOBAL] {
            let c = Club(read_usize(G_CLUB_TABLE) + read_usize(g) * club::SIZE);
            write_usize(c.addr() + club::DIVISION, comp_record_of(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION));
        }
    }
    exe::league_free_table(league, 0);
    let l = league.round_dates();
    if l != 0 {
        exe::crt_free(l);
        league.set_round_dates(0);
    }
    if (league.current_stage() as i32) >= 0 {
        let mut i: i8 = 0;
        loop {
            let f = read_usize(league.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i = i.wrapping_add(1);
            if i as i32 > league.current_stage() as i32 {
                break;
            }
        }
    }
    league.set_current_stage(0xffff_ffff);
    exe::crt_free(league.stages());
    let stats = league.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    league.set_year(league.year().wrapping_add(1)); // mwb15: the next Premier season
    league.set_stage_count(0);
    league.set_stages(0);
    covered!("mwb_15 scottish premier turnover");
    if call(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa7af70, 0x338);
        return 0;
    }
    if sco_prm_create_table_via_exe(this.addr()) == 0 {
        show_error_at(0xa7af70, 0x33f);
        return 0;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0xa7af70, 0x345);
        return 0;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0xa7af70, 0x34c);
        return 0;
    }
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION), 2, (-3i32) as u32, 3, 0);
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION), 2, 0, -3, 0);
    for g in [KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION] {
        call(comp_by_global(g), comp_vtable::START_NEW_SEASON);
    }
    call(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(league)
}

/// Comparer for `sco_prm_start_new_season`: only the original runs (it turns the whole Scottish league over); the year
/// went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_START_NEW_SEASON);
    compare_state_league_new_season("sco_prm_start_new_season", League(this), || o(this))
}

/// rule: in the season played in three rounds (`league::LEGS` = 3), on a day with results (`results` ≠ 0) when every
/// club has played 33 games, the Premier splits (`ScoPrm::create_split_stages`); then the League day update
/// (League::update_for_date 0x6847c0, the same arguments: thiscall, ret 8), whose eax it returns
/// Original: `ScoPrm::update_for_date` (0x7f2d20, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let league = League(this);
    if results != 0 && league.legs() == 3 && all_played_33(league) {
        covered!("mwb_15 scottish premier split day");
        exe::scoprm_create_split_stages(this);
    }
    exe::league_update_for_date(this, today, results) as u32 // the callers read only al
}

/// Comparer for `sco_prm_update_for_date`: only the original runs (the split builds stages and the day update plays
/// on); the plan: whether the split was due (it leaves the league in four rounds with the split compare) and the return.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SCO_PRM_UPDATE_FOR_DATE);
    let league = League(this);
    let due = results != 0 && league.legs() == 3 && all_played_33(league);
    let r = o(this, today, results);
    let ok = !due || (league.legs() == 4 && league.custom_compare() == 0x7f3220);
    SCOREBOARD.record(ok, || format!("sco_prm_update_for_date({:#x}) split due {} legs {} compare {:#x}", this, due, league.legs(), league.custom_compare()));
    r
}

/// rule: the Scottish Premier split after 33 games: the top six (table rows 1 to 6) and the bottom six (from the last
/// row up) each get five more rounds among themselves (the split stages 0 and 1 of `ScoPrm::create_schedule`, built as
/// League_Stage objects whose fixtures stay); then the league plays on in four rounds (`league::LEGS` = 4) with the
/// split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220), which keeps each half apart. Out of memory for the club
/// list: the box of line 0x3a2 and nothing more
/// Original: `ScoPrm::create_split_stages` (0x7f2d90, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_split_stages(this: usize) {
    let this = League(this);
    let league = this;
    let clubs = exe::crt_malloc(0x18);
    if clubs == 0 {
        show_error_at(0xa7af70, 0x3a2);
        return;
    }
    covered!("mwb_15 scottish premier split");
    for k in 0..6usize {
        write_usize(clubs + 4 * k, read_usize(league.table() + k * 0x3b));
    }
    let top = build_split_half(league, 0, clubs);
    exe::crt_free(top);
    for k in 0..6usize {
        let row = (league.club_count() as i32 - k as i32 - 1) as usize; // mwb15: the bottom six from the last row up
        write_usize(clubs + 4 * k, read_usize(league.table() + row * 0x3b));
    }
    let bottom = build_split_half(league, 1, clubs);
    exe::crt_free(clubs);
    exe::crt_free(bottom);
    this.set_legs(4u16 as i16);
    league.set_custom_compare(0x7f3220);
}

/// Comparer for `sco_prm_create_split_stages`: only the original runs (it builds stages and fixtures); then the league
/// is in four rounds with the split compare.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_split_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCO_PRM_CREATE_SPLIT_STAGES);
    o(this);
    let league = League(this);
    SCOREBOARD.record(league.legs() == 4 && league.custom_compare() == 0x7f3220, || format!("sco_prm_create_split_stages({:#x}) legs {} compare {:#x}", this, league.legs(), league.custom_compare()));
}

/// The Scottish Premier Division at career creation: a stadium of 10,000 places (all seated), discipline rule 0x18, the
/// rules, the table with its 12 clubs (10 in 1999), the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5); a saved season played in four rounds (the split) outside 1999
/// gets back the split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220).
/// Original: `ScoPrm::ScoPrm` (0x7f0b80, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_PREMIER_DIVISION, "mwb_13 new scottish premier")
}

/// Comparer for `construct_sco_prm` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_PRM);
    compare_british_division_ctor("construct_sco_prm", League(this), year, record, &SCOTTISH_PREMIER_DIVISION, || o(this, year, record))
}

/// The Premier table: 12 clubs (10 in 1999), those whose division is the Scottish Premier, in database order; the prize
/// fund of 50,000 per place (League::calc_prize_fund) and 500,000 of TV money to every club (League::pay_tv_rights with
/// no division filter); out of memory, the box of line 0x2a7
/// Original: `ScoPrm::create_table` (0x7f2820, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_table(this: usize) -> u32 {
    let n = scottish_premier_clubs(League(this));
    fill_british_division_table(League(this), n, 0xa7af70, 0x2a7, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), Some(0x7a120), 0, "mwb_13 scottish premier table") // mwb13: the Premier rows
}

/// Comparer for `sco_prm_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_CREATE_TABLE);
    let n = scottish_premier_clubs(League(this));
    compare_british_division_table("sco_prm_create_table", League(this), n, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}

/// rule: the Scottish Premier Division (12 clubs) plays 3 rounds (33 matches each) and then the split: 3 points for a
/// win and 1 for a draw, the bottom club goes down to the First (in 1999: 10 clubs, 4 rounds, the bottom club to a
/// relegation play-off); tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5
/// substitutes named and 3 allowed, squad flags 2; builds the round schedule (virtual +0x3c, 38 rounds) and, outside
/// 1999, sets the round count to 33 (the rounds before the split); returns 1
/// Original: `ScoPrm::set_up_rules` (0x7f2970, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let league = this;
    covered!("mwb_13 scottish premier rules");
    eng_prm_apply_league_rules(league, &scottish_premier_rules(league));
    if league.year() != 0x7cf {
        this.set_round_count(0x21u16 as i16);
    }
    1
}

/// Comparer for `sco_prm_set_up_rules` (side by side on the rule bytes and, outside 1999, the round count; the copy
/// without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_SET_UP_RULES);
    let mut written = scottish_premier_rules(League(this));
    if League(this).year() != 0x7cf {
        written.push((league::ROUND_COUNT, 2, 0x21));
    }
    eng_prm_compare_league_rules("sco_prm_set_up_rules", League(this), written, || o(this), || sco_prm_set_up_rules(this))
}

/// The main stage of the Scottish Premier Division: `league` (when given) = 1, `rounds`, `extra` = 0, the table
/// (`exe::crt_malloc` of 0x41 per round; out of memory, the box of the first line and 0); the rounds before; the date of
/// 1 December of the year (`exe::date_date_549f70`) and k = (the day of round 20 − the day of that date + 1, in 16
/// bits) − 13: from 0 to 6 the block of case k, outside the box of the second line and the default block; then the
/// rounds after, from `after_at`.
pub(crate) unsafe fn scottish_premier_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, s: &ScottishPremierSchedule) -> usize {
    if !league.is_null() {
        *league = 1;
    }
    *rounds = s.rounds;
    *extra = 0;
    let t = exe::crt_malloc(s.rounds as u32 * 0x41);
    if t == 0 {
        show_error_at(0xa7af70, s.rows.0);
        return 0;
    }
    let year = (this.year() as u16) as u32;
    eng_first_set_league_rounds(t, year, 0, s.before);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = read_u16(t + 20 * 0x41);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd);
    let middle = if k as u32 > 6 {
        show_error_at(0xa7af70, s.rows.1);
        s.default
    } else {
        s.cases[k as usize]
    };
    eng_first_set_league_rounds(t, year, s.before.len() as i32, middle);
    eng_first_set_league_rounds(t, year, s.after_at, s.after);
    t
}

/// The Premier table routine of the original (ScoPrm::create_table 0x7f2820, thiscall, ret 0), called directly as the
/// turnover does.
pub(crate) unsafe fn sco_prm_create_table_via_exe(this: usize) -> u32 {
    exe::scoprm_create_table(this)
}

/// Whether every club of the Premier table has played at least 33 games (the byte at +0x11 of each 0x3b-byte row,
/// signed); true with no rows.
pub(crate) unsafe fn all_played_33(league: League) -> bool {
    let n = league.club_count();
    let mut k: i16 = 0;
    while k < n {
        if read_i8(league.table() + k as usize * 0x3b + 0x11) < 0x21 { // mwb15: the split after 33 games
            return false;
        }
        k += 1;
    }
    true
}

/// One half of the split: the round table of `stage` (virtual +0x3c: rounds and stage label into locals, no league
/// flag), a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100, thiscall with 21 arguments, ret
/// 0x58: this league as the parent, 6 clubs, the list, 1, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5) and bands (+0xbe), the year, −1, the stage label, 0x14, 1, 0, 3, −1, 0, 2)
/// which is then destroyed at once (virtual 0 with 1: the fixtures it registered stay); returns the round table.
pub(crate) unsafe fn build_split_half(league: League, stage: u32, clubs: usize) -> usize {
    let t = league;
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t.addr(), stage, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count and the stage label as dwords with the stack's high
        // bytes; League_Stage::League_Stage reads the words
        let s = ctor(p, t.addr(), 6, clubs, 1, t.competition().addr(), table, rounds as u16 as u32, t.win_points() as u32,
            t.draw_points() as u32, t.match_category() as u32, t.addr() + league::TIEBREAKS, t.addr() + league::PROMOTED_COUNT,
            league.year() as u16 as u32, 0xffff_ffff, label as u16 as u32, 0x14, 1, 0, 3, 0xffff_ffff, 0, 2);
        if s != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(s)));
            destroy(s, 1);
        }
    }
    table
}

/// The number of Premier clubs: 10 in 1999, 12 from 2000.
pub(crate) unsafe fn scottish_premier_clubs(this: League) -> i16 {
    if this.year() == 0x7cf { 0xa } else { 0xc }
}

/// The Premier rule writes before the round table: in 1999 4 rounds, nobody promoted, 1 to the relegation play-off and
/// nobody relegated directly, the games per club not computed (−1); from 2000 3 rounds, the bottom club relegated and 38
/// games per club; then no custom table compare (`league::CUSTOM_COMPARE` = 0), 3 points for a win and 1 for a draw,
/// category 2, match and round rules 0, club sides, tie-breaks 1, 2 and 3, no division above (−1) and the First below,
/// 3 substitutions, squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn scottish_premier_rules(this: League) -> Vec<(usize, u8, u32)> {
    let mut v = if this.year() == 0x7cf {
        vec![(league::LEGS, 2, 4), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1),
             (league::RELEGATED_COUNT, 1, 0), (league::GAMES_PER_CLUB, 2, 0xffff)]
    } else {
        vec![(league::LEGS, 2, 3), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
             (league::RELEGATED_COUNT, 1, 1), (league::GAMES_PER_CLUB, 2, 0x26)]
    };
    v.extend_from_slice(&[(league::CUSTOM_COMPARE, 4, 0), (league::WIN_POINTS, 1, 3), // mwb13: the Premier points for a win
                          (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0),
                          (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3),
                          (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION) as u32),
                          (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// The two split stages of the Scottish Premier Division (stage 0, the top six, and stage 1, the bottom six): five
/// Saturdays from 14 April to 12 May of the next year.
pub(crate) const SCOTTISH_PREMIER_SPLIT: [EngFirstScheduleRound; 5] = [
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 0: 14/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 1: 21/abr
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 2: 28/abr
    (5, 4, 1, 5, 1, &[(-1, -1, -1)]), // 3: 5/mai
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 4: 12/mai, mwb14: the last split round
];

pub(crate) const SCOTTISH_PREMIER_1999_SCHEDULE: ScottishPremierSchedule = ScottishPremierSchedule {
    before: &SCOTTISH_PREMIER_1999_BEFORE,
    cases: [&SCOTTISH_PREMIER_1999_CHRISTMAS_0, &SCOTTISH_PREMIER_1999_CHRISTMAS_1, &SCOTTISH_PREMIER_1999_CHRISTMAS_2, &SCOTTISH_PREMIER_1999_CHRISTMAS_3,
            &SCOTTISH_PREMIER_1999_CHRISTMAS_4, &SCOTTISH_PREMIER_1999_CHRISTMAS_5, &SCOTTISH_PREMIER_1999_CHRISTMAS_6],
    default: &SCOTTISH_PREMIER_1999_CHRISTMAS_DEFAULT,
    after_at: 23,
    after: &SCOTTISH_PREMIER_1999_AFTER,
    rounds: 0x24,
    rows: (0xaf, 0x148),
};

pub(crate) const SCOTTISH_PREMIER_SCHEDULE: ScottishPremierSchedule = ScottishPremierSchedule {
    before: &SCOTTISH_PREMIER_BEFORE,
    cases: [&SCOTTISH_PREMIER_CHRISTMAS_0, &SCOTTISH_PREMIER_CHRISTMAS_1, &SCOTTISH_PREMIER_CHRISTMAS_2, &SCOTTISH_PREMIER_CHRISTMAS_3,
            &SCOTTISH_PREMIER_CHRISTMAS_4, &SCOTTISH_PREMIER_CHRISTMAS_5, &SCOTTISH_PREMIER_CHRISTMAS_6],
    default: &SCOTTISH_PREMIER_CHRISTMAS_DEFAULT,
    after_at: 23,
    after: &SCOTTISH_PREMIER_AFTER,
    rounds: 0x26,
    rows: (0x193, 0x223),
};

/// The global holding the club index of Elgin City (moved into the Scottish Third Division at the 1999 turnover).
pub(crate) const ELGIN_CITY_INDEX_GLOBAL: usize = 0x9d029c;

/// The global holding the club index of Peterhead (the same).
pub(crate) const PETERHEAD_INDEX_GLOBAL: usize = 0x9d02a0;

/// The Scottish Premier Division: a stadium of 10,000 places, all seated, discipline rule 0x18, no stages beyond the
/// league, and the split's table compare (0x7f3220) given back to a saved season played in four rounds.
pub(crate) const SCOTTISH_PREMIER_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f608,
    stadium: Some((0x2710, 0x2710)),
    discipline: 0x18, // mwb13: the Premier discipline rule
    playoff_in_1999: false,
    split_compare: Some(0x7f3220),
    rules: 0x7f2970,
    table: 0x7f2820,
    file: 0xa7af70,
    rows: [0x53, 0x5a, 0x61, 0x68, 0x6f],
};

/// The main-stage schedule of the Scottish Premier Division: the rounds before the year end, the seven year-end blocks
/// chosen by the day the 21st round (index 20) falls on (k = that day − 1 December + 1 − 13) and the default after the
/// box, the index the rounds after the year end start at (23 in both seasons: in 1999 the cases write round 21 only and
/// round 22 stays as malloc left it, a defect of the original), the rounds after, the number of rounds, the error-box
/// lines (out of memory, the case out of range).
pub(crate) struct ScottishPremierSchedule {
    pub(crate) before: &'static [EngFirstScheduleRound],
    pub(crate) cases: [&'static [EngFirstScheduleRound]; 7],
    pub(crate) default: &'static [EngFirstScheduleRound],
    pub(crate) after_at: i32,
    pub(crate) after: &'static [EngFirstScheduleRound],
    pub(crate) rounds: i16,
    pub(crate) rows: (u32, u32),
}

/// The Scottish Premier Division (1999) before the year end.
pub(crate) const SCOTTISH_PREMIER_1999_BEFORE: [EngFirstScheduleRound; 21] = [
    (29, 6, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 0: 29/jul
    (5, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 1: 5/ago
    (12, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 2: 12/ago
    (19, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 3: 19/ago
    (26, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 4: 26/ago
    (9, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 5: 9/set
    (16, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 6: 16/set
    (23, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 7: 23/set
    (30, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 8: 30/set
    (14, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 9: 14/out
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 10: 17/out
    (21, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 11: 21/out
    (28, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 12: 28/out
    (4, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 13: 4/nov
    (11, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 14: 11/nov
    (18, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 15: 18/nov
    (25, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 16: 25/nov
    (29, 10, 0, 2, 2, &[(-1, -1, -1)]), // 17: 29/nov
    (2, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 18: 2/dez
    (9, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 19: 9/dez
    (16, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 20: 16/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 13 December (case 0).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_0: [EngFirstScheduleRound; 1] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 20/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 14 December (case 1).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_1: [EngFirstScheduleRound; 1] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 21/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 15 December (case 2).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_2: [EngFirstScheduleRound; 1] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 22/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 16 December (case 3).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_3: [EngFirstScheduleRound; 1] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 23/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 17 December (case 4).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_4: [EngFirstScheduleRound; 1] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 20/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 18 December (case 5).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_5: [EngFirstScheduleRound; 1] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 21/dez
];

/// The Scottish Premier Division (1999) year-end rounds when the 21st round falls on 19 December (case 6).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_6: [EngFirstScheduleRound; 1] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
];

/// The Scottish Premier Division (1999) year-end rounds out of range (after the error box).
pub(crate) const SCOTTISH_PREMIER_1999_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division (1999) after the year end.
pub(crate) const SCOTTISH_PREMIER_1999_AFTER: [EngFirstScheduleRound; 13] = [
    (2, 0, 1, 1, 2, &[(-1, -1, -1)]), // 23: 2/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27/jan
    (3, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 26: 3/fev
    (10, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 27: 10/fev
    (24, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 28: 24/fev
    (3, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 29: 3/mar
    (17, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 30: 17/mar
    (31, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 31: 31/mar
    (7, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 32: 7/abr
    (6, 4, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 33: 6/mai
    (13, 4, 1, 5, 1, &[(-1, -1, -1)]), // 34: 13/mai
    (20, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 20/mai
];

/// The Scottish Premier Division before the year end.
pub(crate) const SCOTTISH_PREMIER_BEFORE: [EngFirstScheduleRound; 21] = [
    (29, 6, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 0: 29/jul
    (5, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 1: 5/ago
    (12, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 2: 12/ago
    (19, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 3: 19/ago
    (26, 7, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 4: 26/ago
    (9, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 5: 9/set
    (16, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 6: 16/set
    (23, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 7: 23/set
    (30, 8, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 8: 30/set
    (14, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 9: 14/out, mwb14: a Premier Division round
    (17, 9, 0, 1, 2, &[(-1, -1, -1)]), // 10: 17/out
    (21, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 11: 21/out
    (28, 9, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 12: 28/out
    (4, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 13: 4/nov
    (11, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 14: 11/nov
    (18, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 15: 18/nov
    (25, 10, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 16: 25/nov
    (29, 10, 0, 2, 2, &[(-1, -1, -1)]), // 17: 29/nov
    (2, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 18: 2/dez
    (9, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 19: 9/dez
    (16, 11, 0, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 20: 16/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 13 December (case 0).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_0: [EngFirstScheduleRound; 2] = [
    (20, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 20/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 14 December (case 1).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_1: [EngFirstScheduleRound; 2] = [
    (21, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 21/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 15 December (case 2).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_2: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 16 December (case 3).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_3: [EngFirstScheduleRound; 2] = [
    (23, 11, 0, -1, 1, &[(-1, -1, -1)]), // 21: 23/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 17 December (case 4).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_4: [EngFirstScheduleRound; 2] = [
    (20, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 20/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 18 December (case 5).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_5: [EngFirstScheduleRound; 2] = [
    (21, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 21/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds when the 21st round falls on 19 December (case 6).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_6: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division year-end rounds out of range (after the error box).
pub(crate) const SCOTTISH_PREMIER_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (22, 11, 0, -1, 2, &[(-1, -1, -1)]), // 21: 22/dez
    (26, 11, 0, -1, 2, &[(-1, -1, -1)]), // 22: 26/dez
];

/// The Scottish Premier Division after the year end.
pub(crate) const SCOTTISH_PREMIER_AFTER: [EngFirstScheduleRound; 15] = [
    (2, 0, 1, 1, 2, &[(-1, -1, -1)]), // 23: 2/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27/jan
    (3, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 26: 3/fev
    (10, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 27: 10/fev
    (24, 1, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 28: 24/fev
    (3, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 29: 3/mar
    (17, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 30: 17/mar
    (31, 2, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 31: 31/mar
    (7, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 32: 7/abr
    (14, 3, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 33: 14/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 21/abr (the original first writes (5, 2, 2) into slot 0 and then the end mark over it: no TV slot)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 35: 28/abr
    (5, 4, 1, 5, 1, &[(5, 2, 2), (-1, -1, -1)]), // 36: 5/mai
    (12, 4, 1, 5, 1, &[(-1, -1, -1)]), // 37: 12/mai
];
