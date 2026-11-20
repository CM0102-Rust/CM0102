// ---------- mwb_23: the two Welsh group cups (wel_lge_cup.cpp, wel_prm_cup.cpp): the schedules and the Premier Cup's season turnover ----------

/// The Welsh League Cup's six group matchdays, weekly from 23 August to 27 September (weekday 2, code 2: midweek
/// [inference]); (day, month from 0, weekday, code).
pub(crate) const WELSH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 6] = [(0x17, 7, 2, 2), (0x1e, 7, 2, 2), (6, 8, 2, 2), (0xd, 8, 2, 2), (0x14, 8, 2, 2), (0x1b, 8, 2, 2)]; // mwb23: the Welsh League Cup group matchdays

/// The Welsh League Cup's knockout: quarter-finals drawn on 28 September and played on 11 October (single matches, tie
/// rule 3, category 4); semi-finals over two legs fourteen days apart, drawn on 12 October and first played on 25
/// October (two-leg rule 3, category 4); the final drawn on 9 November and played on 10 January of the next year
/// (category 6); the tie winner's prize a share of the fund (−1).
pub(crate) const WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x1c, 8, 0, 3), (0xb, 9, 0, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xc, 9, 0, 3), (0x19, 9, 0, 2, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0xa, 0, 1, 2, 2, 0), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Rule: the Welsh League Cup's groups (stages up to 4, and the cup's own table) play six matchdays, weekly from 23
/// August to 27 September (a double round robin of three clubs), each group a league stage (is_league = 1, label 0x3e9
/// + stage); the knockout (stage 5) has eight clubs: quarter-finals on 11 October, two-legged semi-finals from 25
/// October and the final on 10 January of the next year.
///
/// `create_schedule` with the knockout at stage 5, the label 0x3e9, `WELSH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xc0 and 0xeb of wel_lge_cup.cpp (read instruction by
/// instruction against IreLgeCup::create_schedule: only the stage, the dates, the rounds and the lines change). Above
/// stage 5: 0.
/// Original: `WelLgeCup::create_schedule` (0x9261e0, wel_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 5, 0x3e9, &WELSH_LEAGUE_CUP_GROUP_ROUNDS, &WELSH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0xad56f4, 0xc0, 0xeb,
        "mwb_23 welsh league cup groups", "mwb_23 welsh league cup knockout rounds")
}

/// Comparer for `wel_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    ita_cup_compare_schedule("wel_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wel_lge_cup_create_schedule(this, stage, r, e, l), 5)
}

/// A group matchday with its year offset: (day, month from 0, years after the cup's year, weekday, code).
pub(crate) type WelshGroupRound = (u32, u32, u32, u32, u32);

/// `create_schedule` with group matchdays that may fall in the next year (the Welsh Premier Cup's last one): the same
/// frame, the group rounds written by `exe::league_round_set_date`(table, r, day, month, years, weekday, code, year, 0)
/// with their own year offset.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn create_schedule_with_years(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, limit: i8, id: i16, groups: &[WelshGroupRound],
    knockout: &[EngCcCupCupRound], file: usize, rows: (u32, u32), branches: (&'static str, &'static str)) -> usize {
    let f = stage as u8 as i8;
    if f < limit {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = groups.len() as i16;
        *extra = (f as i16).wrapping_add(id);
        let t = exe::crt_malloc(groups.len() as u32 * 0x41);
        if t == 0 {
            show_error_at(file, rows.0);
            return 0;
        }
        covered!(branches.0);
        // the year goes with garbage in the high bytes in the original; 0x68a160 only uses the low 16 bits
        let year = (this.year() as u16) as u32;
        for (r, &(day, month, years, weekday, code)) in groups.iter().enumerate() {
            exe::league_round_set_date(t, r as i32, day as i32, month as i32, years as i32, weekday as i32, code as i32, year, 0);
            exe::league_round_set_match_slot(t, r as i32, 0, -1, -1, -1, 0);
        }
        return t;
    }
    if f != limit {
        return 0;
    }
    if !league.is_null() {
        *league = 0;
    }
    *rounds = knockout.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(knockout.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    covered!(branches.1);
    let year = this.year() as i32 as u32;
    for (i, r) in knockout.iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// The Welsh Premier Cup's six group matchdays (weekday 1, code 2): 5 September, 3 and 10 October, 14 November, 12
/// December and 9 January of the next year.
pub(crate) const WELSH_PREMIER_CUP_GROUP_ROUNDS: [WelshGroupRound; 6] = [(5, 8, 0, 1, 2), (3, 9, 0, 1, 2), (0xa, 9, 0, 1, 2), (0xe, 0xa, 0, 1, 2), (0xc, 0xb, 0, 1, 2), (9, 0, 1, 1, 2)]; // mwb23: the Premier Cup group matchdays

/// The Welsh Premier Cup's knockout, all in the next year: quarter-finals drawn on 10 January and played on 13
/// February (draw rule 8, tie rules 0x203, category 4; 15,000 to each club [inference: the +0x5c prize]); semi-finals
/// over two legs seven days apart, drawn on 14 February and first played on 10 April (two-leg rule 3, category 4;
/// 10,000); the final drawn on 18 April and played on 13 May (weekday 6, code 1; category 6; 100,000 to the winner and
/// 50,000 to the loser).
pub(crate) const WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0xa, 0, 1, 2), (0xd, 1, 1, 1, 2, 0), 0x78, (8, 0x203, 0), 4, (8, 4, 8, 0), (0, 1, 0), (0x3a98, 0, 0)),
    cql10r((0xe, 1, 1, 2), (0xa, 3, 1, 1, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 7), (0x2710, 0, 0)),
    cql10r((0x12, 3, 1, 2), (0xd, 4, 1, 6, 1, 0), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x186a0, 0xc350)),
];

/// Rule: the Welsh Premier Cup's two groups of four (the cup's own table and stage 0) play six matchdays from 5
/// September to 9 January (a double round robin), each group a league stage (is_league = 1, label 0x420 + stage); the
/// knockout (stage 1) has eight clubs in the next year: quarter-finals on 13 February, two-legged semi-finals from 10
/// April and the final on 13 May; prize money in every round.
///
/// `create_schedule_with_years` with the knockout at stage 1, the label 0x420, `WELSH_PREMIER_CUP_GROUP_ROUNDS`,
/// `WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbf and 0xea of wel_prm_cup.cpp (the frame of
/// WelLgeCup::create_schedule, read instruction by instruction; the last group matchday has a year offset). Above stage
/// 1: 0.
/// Original: `WelPrmCup::create_schedule` (0x927b30, wel_prm_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule_with_years(League(this), stage, rounds, extra, league, 1, 0x420, &WELSH_PREMIER_CUP_GROUP_ROUNDS, &WELSH_PREMIER_CUP_KNOCKOUT_ROUNDS, 0xad572c, (0xbf, 0xea),
        ("mwb_23 welsh premier cup groups", "mwb_23 welsh premier cup knockout rounds"))
}

/// Comparer for `wel_prm_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    ita_cup_compare_schedule("wel_prm_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wel_prm_cup_create_schedule(this, stage, r, e, l), 1)
}

/// Rule: at the season turn the Welsh Premier Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the twelve entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stage of the second group.
///
/// `start_irish_league_cup_season` with WelPrmCup::build_entrant_list (0x928860), create_group_table (0x928060),
/// create_group_stage (0x928450), one group and the error box lines 0x1f3, 0x1fa, 0x200 and 0x207 of wel_prm_cup.cpp
/// (read instruction by instruction against IreLgeCup::start_new_season: the registers differ; the group loop is a
/// single call, whose return the routine passes on).
/// Original: `WelPrmCup::start_new_season` (0x9281f0, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x928860, 0x928060, 0x928450, 1, 0xad572c, [0x1f3, 0x1fa, 0x200, 0x207], 1) // mwb23: the Premier Cup turnover
}

/// Comparer for `wel_prm_cup_start_new_season` (original only: `compare_state_league_new_season`, the year up by one).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("wel_prm_cup_start_new_season", League(this), || o(this))
}
