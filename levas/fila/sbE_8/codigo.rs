// ---------- sbE_8: the Russian First Division and Premier League schedules (rus_first.cpp, rus_prm.cpp) ----------

/// Writes the dates of every round of `table` (Date by `exe::league_round_set_date`, cdecl) and then, in a second pass,
/// an empty TV slot list (the terminator, `exe::league_round_set_match_slot`(t, k, 0, −1, −1, −1, 0)) in rounds 0 to
/// `slots` − 1: the order of the Russian schedules (all the dates first).
pub(crate) unsafe fn write_dates_then_empty_slots(t: usize, year: u32, table: &[FraThirdLeagueRound], slots: i32) {
    for (k, &(day, month, years, week, code, _)) in table.iter().enumerate() {
        exe::league_round_set_date(t, k as i32, day as i32, month as i32, years as i32, week as i32, code as i32, year, 0);
    }
    for k in 0..slots {
        exe::league_round_set_match_slot(t, k, 0, -1, -1, -1, 0);
    }
}

/// Rule: the Russian First Division schedule: 36 rounds from 1 April to 1 November, about every six days with many
/// midweek rounds; no TV slots. Only the main stage.
///
/// `RusFirst::create_schedule(stage, &rounds, &extra, &is_league)`: at −1 is_league (if asked) = 1, rounds = 36, extra
/// = 0, the table of 36 × 0x41 bytes (`exe::crt_malloc`; none: the rus_first.cpp (0xa7ac8c) box at line 0xaa and 0),
/// `write_dates_then_empty_slots`. Any other stage: 0, nothing written.
/// Original: `RusFirst::create_schedule` (0x7ea220, rus_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rus_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x24;
    *extra = 0;
    let t = exe::crt_malloc(0x24 * 0x41);
    if t == 0 {
        show_error_at(0xa7ac8c, 0xaa);
        return 0;
    }
    covered!("sbE_8 russian first division schedule");
    write_dates_then_empty_slots(t, (League(this).year() as u16) as u32, &RUS_FIRST_ROUNDS, 0x24);
    t
}

/// Comparer for `rus_first_create_schedule`: side by side (`compare_staged_schedule`, league rounds).
pub(crate) unsafe extern "thiscall" fn compare_rus_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_staged_schedule("rus_first_create_schedule", stage, rounds, extra, league, ScheduleTable::LeagueRounds,
        || o(this, stage, rounds, extra, league), |r, e, lg| rus_first_create_schedule(this, stage, r, e, lg))
}

/// True when Russia plays at the finals this season: with the competition table (`G_COMP_OBJECT_TABLE`), the World
/// Cup (`KEY_CLUB_COMP_FIFA_WORLD_CUP`) or else the European Championship
/// (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`) object exists and includes (Comp::includes_club
/// `exe::comp_includes_club`, thiscall, ret 4, the byte) Russia's national side (`exe::database_get_national_team` of the
/// nation record of `KEY_NATION_RUSSIA`, cdecl).
pub(crate) unsafe fn russia_at_finals() -> bool {
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if table == 0 {
        return false;
    }
    let russia = || exe::database_get_national_team(read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_RUSSIA).wrapping_mul(nation::SIZE));
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP].iter().any(|&key| {
        let finals = read_usize(table + 4 * read_usize(key));
        finals != 0 && exe::comp_includes_club(finals, russia()) != 0
    })
}

/// The Russian Premier League's play-off pair count (a byte; written by RusPrm::create_title_playoff_stage 0x7eba90).
pub(crate) const RUS_PRM_PLAY_OFF_PAIRS: usize = 0xee;

/// The Russian title play-off (stage 0): one single-leg round drawn on 13 November and played on 18 November; the team,
/// tie and entrant counts come from the play-off pair count (`rus_prm_title_play_off_round`).
pub(crate) const RUS_PRM_TITLE_PLAY_OFF_ROUND: EngCcCupCupRound = cql10r((0xd, 0xa, 0, 0), (0x12, 0xa, 0, 5, 1, 0), 0xa0, (0, 0x203, 0), 5, (0, 0, 0, 0), (0, 1, 0), (0, 0, 0));

/// The title play-off round with p = `RUS_PRM_PLAY_OFF_PAIRS` (a signed byte, 16-bit arithmetic): 2p teams, p ties and 2p
/// new entrants.
pub(crate) unsafe fn rus_prm_title_play_off_round(this: League) -> EngCcCupCupRound {
    let p = read_i8(this.addr() + RUS_PRM_PLAY_OFF_PAIRS) as i16;
    let mut r = RUS_PRM_TITLE_PLAY_OFF_ROUND;
    r.teams = ((p << 1) as u16, p as u16, (p << 1) as u16, 0);
    r
}

/// Rule: the Russian Premier League schedule: the season (the main stage) in 30 rounds from 25 March to 12 November;
/// when Russia plays at the World Cup or the European Championship, other dates with midweek rounds and a break from
/// 31 May to 22 July; no TV slots. The title play-off (stage 0, two clubs tied at the top) a single-leg round on 18
/// November.
///
/// `RusPrm::create_schedule(stage, &rounds, &extra, &is_league)`: at −1 is_league (if asked) = 1, rounds = 30, extra =
/// 0, the table of 30 × 0x41 bytes (`exe::crt_malloc`; none: the rus_prm.cpp (0xa7acc8) box at line 0xaa and 0); the
/// dates (`russia_at_finals`: the finals dates, otherwise the usual ones) and the empty slots of the 30 rounds
/// (`write_dates_then_empty_slots`; the box of line 0xfc is never reached). At 0: is_league = 0, rounds = 1, extra = 0,
/// the table of 0x68 bytes (the box at line 0x10f) with `rus_prm_title_play_off_round` (`write_play_off_round`). Any
/// other stage: 0, nothing written.
/// Defect of the original: the finals dates are 31 (round 30 on 12 November), written past the end of the 30-round table
/// (a heap overflow of the round record at index 30); the copy reserves room for it (correction, 24/11/2026).
/// Original: `RusPrm::create_schedule` (0x7ead50, rus_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn rus_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 0x1e;
            *extra = 0;
            let finals = russia_at_finals();
            // correction of the original (24/11/2026, Lucas): with Russia at the finals the 31 dates are written into a
            // 30-round table (a heap overflow of one round record); the copy reserves the 31st (docs/DEFEITOS_DO_ORIGINAL.md)
            let t = exe::crt_malloc(if finals { 0x1f * 0x41 } else { 0x1e * 0x41 });
            if t == 0 {
                show_error_at(0xa7acc8, 0xaa);
                return 0;
            }
            let table: &[FraThirdLeagueRound] = if finals { &RUS_PRM_FINALS_ROUNDS } else { &RUS_PRM_ROUNDS }; // sbE_8: the finals dates
            covered!("sbE_8 russian premier league schedule");
            write_dates_then_empty_slots(t, (l.year() as u16) as u32, table, 0x1e);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa7acc8, 0x10f);
                return 0;
            }
            covered!("sbE_8 russian title play-off round");
            write_play_off_round(l, t, 0, &rus_prm_title_play_off_round(l));
            t
        }
        _ => 0,
    }
}

/// Comparer for `rus_prm_create_schedule`: side by side (`compare_staged_schedule`; the season as league rounds, the
/// play-off as cup rounds); with Russia at the finals only the original (its table overflow is not done twice).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    if stage as u8 == 0xff && russia_at_finals() {
        return o(this, stage, rounds, extra, league);
    }
    let kind = if stage as u8 == 0 { ScheduleTable::CupRounds } else { ScheduleTable::LeagueRounds };
    compare_staged_schedule("rus_prm_create_schedule", stage, rounds, extra, league, kind,
        || o(this, stage, rounds, extra, league), |r, e, lg| rus_prm_create_schedule(this, stage, r, e, lg))
}

/// The Russian First Division: 36 rounds (date; the TV slots are written apart, empty).
pub(crate) const RUS_FIRST_ROUNDS: [FraThirdLeagueRound; 36] = [
    (1, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 1 Apr
    (7, 3, 0, 5, 1, &[(-1, -1, -1)]), // 1: 7 Apr
    (14, 3, 0, 5, 1, &[(-1, -1, -1)]), // 2: 14 Apr
    (20, 3, 0, 4, 2, &[(-1, -1, -1)]), // 3: 20 Apr
    (26, 3, 0, 3, 2, &[(-1, -1, -1)]), // 4: 26 Apr
    (2, 4, 0, 2, 2, &[(-1, -1, -1)]), // 5: 2 May
    (8, 4, 0, 1, 2, &[(-1, -1, -1)]), // 6: 8 May
    (14, 4, 0, 0, 2, &[(-1, -1, -1)]), // 7: 14 May
    (20, 4, 0, 6, 1, &[(-1, -1, -1)]), // 8: 20 May
    (26, 4, 0, 5, 1, &[(-1, -1, -1)]), // 9: 26 May
    (1, 5, 0, 4, 2, &[(-1, -1, -1)]), // 10: 1 Jun
    (7, 5, 0, 3, 2, &[(-1, -1, -1)]), // 11: 7 Jun
    (13, 5, 0, 2, 2, &[(-1, -1, -1)]), // 12: 13 Jun
    (19, 5, 0, 1, 2, &[(-1, -1, -1)]), // 13: 19 Jun
    (25, 5, 0, 0, 2, &[(-1, -1, -1)]), // 14: 25 Jun
    (4, 6, 0, 2, 2, &[(-1, -1, -1)]), // 15: 4 Jul
    (10, 6, 0, 1, 2, &[(-1, -1, -1)]), // 16: 10 Jul
    (14, 6, 0, 5, 1, &[(-1, -1, -1)]), // 17: 14 Jul
    (25, 6, 0, 2, 2, &[(-1, -1, -1)]), // 18: 25 Jul
    (31, 6, 0, 1, 2, &[(-1, -1, -1)]), // 19: 31 Jul
    (6, 7, 0, 0, 2, &[(-1, -1, -1)]), // 20: 6 Aug
    (12, 7, 0, 6, 1, &[(-1, -1, -1)]), // 21: 12 Aug
    (18, 7, 0, 5, 1, &[(-1, -1, -1)]), // 22: 18 Aug
    (24, 7, 0, 4, 2, &[(-1, -1, -1)]), // 23: 24 Aug
    (30, 7, 0, 3, 2, &[(-1, -1, -1)]), // 24: 30 Aug
    (5, 8, 0, 2, 2, &[(-1, -1, -1)]), // 25: 5 Sep
    (9, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26: 9 Sep
    (13, 8, 0, 3, 2, &[(-1, -1, -1)]), // 27: 13 Sep
    (19, 8, 0, 2, 2, &[(-1, -1, -1)]), // 28: 19 Sep
    (25, 8, 0, 1, 2, &[(-1, -1, -1)]), // 29: 25 Sep
    (1, 9, 0, 0, 2, &[(-1, -1, -1)]), // 30: 1 Oct
    (7, 9, 0, 6, 1, &[(-1, -1, -1)]), // 31: 7 Oct
    (13, 9, 0, 5, 1, &[(-1, -1, -1)]), // 32: 13 Oct
    (20, 9, 0, 5, 1, &[(-1, -1, -1)]), // 33: 20 Oct
    (26, 9, 0, 4, 2, &[(-1, -1, -1)]), // 34: 26 Oct
    (1, 10, 0, 3, 2, &[(-1, -1, -1)]), // 35: 1 Nov
];

/// The Russian Premier League: 30 rounds (date; the slots apart).
pub(crate) const RUS_PRM_ROUNDS: [FraThirdLeagueRound; 30] = [
    (25, 2, 0, 5, 1, &[(-1, -1, -1)]), // 0: 25 Mar
    (1, 3, 0, 5, 1, &[(-1, -1, -1)]), // 1: 1 Apr
    (8, 3, 0, 5, 1, &[(-1, -1, -1)]), // 2: 8 Apr
    (15, 3, 0, 5, 1, &[(-1, -1, -1)]), // 3: 15 Apr
    (22, 3, 0, 5, 1, &[(-1, -1, -1)]), // 4: 22 Apr
    (30, 3, 0, 6, 1, &[(-1, -1, -1)]), // 5: 30 Apr
    (7, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 7 May
    (13, 4, 0, 5, 1, &[(-1, -1, -1)]), // 7: 13 May
    (17, 4, 0, 2, 2, &[(-1, -1, -1)]), // 8: 17 May
    (26, 4, 0, 6, 1, &[(-1, -1, -1)]), // 9: 26 May
    (8, 5, 0, 3, 2, &[(-1, -1, -1)]), // 10: 8 Jun
    (12, 5, 0, 0, 2, &[(-1, -1, -1)]), // 11: 12 Jun
    (18, 5, 0, 6, 1, &[(-1, -1, -1)]), // 12: 18 Jun
    (23, 5, 0, 6, 1, &[(-1, -1, -1)]), // 13: 23 Jun
    (27, 5, 0, 1, 2, &[(-1, -1, -1)]), // 14: 27 Jun
    (8, 6, 0, 5, 1, &[(-1, -1, -1)]), // 15: 8 Jul
    (15, 6, 0, 5, 1, &[(-1, -1, -1)]), // 16: 15 Jul
    (22, 6, 0, 5, 1, &[(-1, -1, -1)]), // 17: 22 Jul
    (29, 6, 0, 5, 1, &[(-1, -1, -1)]), // 18: 29 Jul
    (5, 7, 0, 5, 1, &[(-1, -1, -1)]), // 19: 5 Aug
    (13, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 13 Aug
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 21: 19 Aug
    (27, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22: 27 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 23: 6 Sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 24: 23 Sep
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 25: 1 Oct
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 26: 14 Oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 27: 21 Oct
    (5, 10, 0, 6, 1, &[(-1, -1, -1)]), // 28: 5 Nov
    (12, 10, 0, 6, 1, &[(-1, -1, -1)]), // 29: 12 Nov
];

/// The Russian Premier League with Russia at the finals: 31 dates for 30 rounds (the last past the table; the slots apart).
pub(crate) const RUS_PRM_FINALS_ROUNDS: [FraThirdLeagueRound; 31] = [
    (25, 2, 0, 5, 1, &[(-1, -1, -1)]), // 0: 25 Mar
    (1, 3, 0, 5, 1, &[(-1, -1, -1)]), // 1: 1 Apr
    (8, 3, 0, 5, 1, &[(-1, -1, -1)]), // 2: 8 Apr
    (12, 3, 0, 2, 2, &[(-1, -1, -1)]), // 3: 12 Apr
    (15, 3, 0, 5, 1, &[(-1, -1, -1)]), // 4: 15 Apr
    (22, 3, 0, 5, 1, &[(-1, -1, -1)]), // 5: 22 Apr
    (26, 3, 0, 2, 2, &[(-1, -1, -1)]), // 6: 26 Apr
    (30, 3, 0, 6, 1, &[(-1, -1, -1)]), // 7: 30 Apr
    (7, 4, 0, 6, 1, &[(-1, -1, -1)]), // 8: 7 May
    (13, 4, 0, 5, 1, &[(-1, -1, -1)]), // 9: 13 May
    (17, 4, 0, 2, 2, &[(-1, -1, -1)]), // 10: 17 May
    (20, 4, 0, 5, 1, &[(-1, -1, -1)]), // 11: 20 May
    (28, 4, 0, 6, 1, &[(-1, -1, -1)]), // 12: 28 May
    (31, 4, 0, 2, 2, &[(-1, -1, -1)]), // 13: 31 May
    (22, 6, 0, 5, 1, &[(-1, -1, -1)]), // 14: 22 Jul
    (29, 6, 0, 5, 1, &[(-1, -1, -1)]), // 15: 29 Jul
    (5, 7, 0, 5, 1, &[(-1, -1, -1)]), // 16: 5 Aug
    (9, 7, 0, 2, 2, &[(-1, -1, -1)]), // 17: 9 Aug
    (13, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 13 Aug
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 19: 19 Aug
    (27, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 27 Aug
    (30, 7, 0, 2, 2, &[(-1, -1, -1)]), // 21: 30 Aug
    (6, 8, 0, 2, 2, &[(-1, -1, -1)]), // 22: 6 Sep
    (20, 8, 0, 2, 2, &[(-1, -1, -1)]), // 23: 20 Sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 24: 23 Sep
    (1, 9, 0, 6, 1, &[(-1, -1, -1)]), // 25: 1 Oct
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 26: 14 Oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 27: 21 Oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 28: 28 Oct
    (5, 10, 0, 6, 1, &[(-1, -1, -1)]), // 29: 5 Nov
    (12, 10, 0, 6, 1, &[]), // 30: 12 Nov
];
