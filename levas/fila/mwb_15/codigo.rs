// ---------- mwb_15: the Scottish Premier Division (sco_prm.cpp): the turnover, the day update and the split ----------

/// The global holding the club index of Elgin City (moved into the Scottish Third Division at the 1999 turnover).
pub(crate) const ELGIN_CITY_INDEX_GLOBAL: usize = 0x9d029c;
/// The global holding the club index of Peterhead (the same).
pub(crate) const PETERHEAD_INDEX_GLOBAL: usize = 0x9d02a0;

/// rule: the Scottish turnover, led by the Premier Division: the promotion and relegation step (virtual +0xb0 with 0);
/// in 1999 Elgin City and Peterhead join the Third Division (the 2000 expansion); the Premier's old table, rounds and
/// stages are freed, the season archived, the year moves on, with no stages beyond the league; the rules, the table, the
/// fixtures and the order (each with its error box: lines 0x338, 0x33f, 0x345, 0x34c, and 0); the division tier of the
/// Second Division clubs capped (League::limit_club_division_tier 2, −3, 3, 0) and of the Third's (2, 0, −3, 0); then the
/// First, the Second and the Third turn over (virtual +8), and the Premier's predicted positions, the board expectations
/// and the referee renewal, whose eax it returns
/// Original: `ScoPrm::start_new_season` (0x7f2a70, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_start_new_season(this: usize) -> u32 {
    let league = League(this);
    let call = |o: usize, slot: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + slot));
        f(o)
    };
    write_u32(this + league::SEASON_CLOSED, 0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this, 0);
    if league.year() == 0x7cf {
        covered!("mwb_15 scottish expansion 1999");
        for g in [ELGIN_CITY_INDEX_GLOBAL, PETERHEAD_INDEX_GLOBAL] {
            let c = read_usize(G_CLUB_TABLE) + read_usize(g) * club::SIZE;
            write_usize(c + club::DIVISION, comp_record_of(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION));
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
    if call(this, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa7af70, 0x338);
        return 0;
    }
    if sco_prm_create_table_via_exe(this) == 0 {
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
    call(this, comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(league)
}

/// The Premier table routine of the original (ScoPrm::create_table 0x7f2820, thiscall, ret 0), called directly as the
/// turnover does.
pub(crate) unsafe fn sco_prm_create_table_via_exe(this: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7f2820usize);
    f(this)
}

/// Comparer for `sco_prm_start_new_season`: only the original runs (it turns the whole Scottish league over); the year
/// went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("sco_prm_start_new_season", League(this), || o(this))
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

/// rule: in the season played in three rounds (`league::LEGS` = 3), on a day with results (`results` ≠ 0) when every
/// club has played 33 games, the Premier splits (`ScoPrm::create_split_stages`); then the League day update
/// (League::update_for_date 0x6847c0, the same arguments: thiscall, ret 8), whose eax it returns
/// Original: `ScoPrm::update_for_date` (0x7f2d20, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let league = League(this);
    if results != 0 && league.legs() == 3 && all_played_33(league) {
        covered!("mwb_15 scottish premier split day");
        let split: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7f2d90usize);
        split(this);
    }
    let update: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    update(this, today, results)
}

/// Comparer for `sco_prm_update_for_date`: only the original runs (the split builds stages and the day update plays
/// on); the plan: whether the split was due (it leaves the league in four rounds with the split compare) and the return.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_update_for_date(this: usize, today: usize, results: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let league = League(this);
    let due = results != 0 && league.legs() == 3 && all_played_33(league);
    let r = o(this, today, results);
    let ok = !due || (league.legs() == 4 && league.custom_compare() == 0x7f3220);
    SCOREBOARD.record(ok, || format!("sco_prm_update_for_date({:#x}) split due {} legs {} compare {:#x}", this, due, league.legs(), league.custom_compare()));
    r
}

/// One half of the split: the round table of `stage` (virtual +0x3c: rounds and stage label into locals, no league
/// flag), a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100, thiscall with 21 arguments, ret
/// 0x58: this league as the parent, 6 clubs, the list, 1, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5) and bands (+0xbe), the year, −1, the stage label, 0x14, 1, 0, 3, −1, 0, 2)
/// which is then destroyed at once (virtual 0 with 1: the fixtures it registered stay); returns the round table.
pub(crate) unsafe fn build_split_half(league: League, stage: u32, clubs: usize) -> usize {
    let t = league.addr();
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t, stage, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count and the stage label as dwords with the stack's high
        // bytes; League_Stage::League_Stage reads the words
        let s = ctor(p, t, 6, clubs, 1, read_usize(t + league::COMPETITION), table, rounds as u16 as u32, read_u8(t + league::WIN_POINTS) as u32,
            read_u8(t + league::DRAW_POINTS) as u32, read_u8(t + league::MATCH_CATEGORY) as u32, t + league::TIEBREAKS, t + league::PROMOTED_COUNT,
            league.year() as u16 as u32, 0xffff_ffff, label as u16 as u32, 0x14, 1, 0, 3, 0xffff_ffff, 0, 2);
        if s != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(s)));
            destroy(s, 1);
        }
    }
    table
}

/// rule: the Scottish Premier split after 33 games: the top six (table rows 1 to 6) and the bottom six (from the last
/// row up) each get five more rounds among themselves (the split stages 0 and 1 of `ScoPrm::create_schedule`, built as
/// League_Stage objects whose fixtures stay); then the league plays on in four rounds (`league::LEGS` = 4) with the
/// split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220), which keeps each half apart. Out of memory for the club
/// list: the box of line 0x3a2 and nothing more
/// Original: `ScoPrm::create_split_stages` (0x7f2d90, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_split_stages(this: usize) {
    let league = League(this);
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
    write_u16(this + league::LEGS, 4);
    league.set_custom_compare(0x7f3220);
}

/// Comparer for `sco_prm_create_split_stages`: only the original runs (it builds stages and fixtures); then the league
/// is in four rounds with the split compare.
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_split_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    o(this);
    let league = League(this);
    SCOREBOARD.record(league.legs() == 4 && league.custom_compare() == 0x7f3220, || format!("sco_prm_create_split_stages({:#x}) legs {} compare {:#x}", this, league.legs(), league.custom_compare()));
}
