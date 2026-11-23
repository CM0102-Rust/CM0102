use super::*;

/// like the Second one, with the 7th, 4th, 6th and 5th (ties 7th v 4th and 6th v 5th; lines 0x2be and 0x2d5)
/// Original: `EngThird::create_play_off_stage` (0x5783c0, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_create_play_off_stage(this: usize) {
    create_playoff_stage(League(this), [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third
}

/// the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off
/// Original: `EngThird::EngThird` (0x576dd0, eng_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_third(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(League(this), year, record, &THIRD_DIVISION)
}

/// the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when the
/// nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below)
/// Original: `EngThird::set_up_rules` (0x5780c0, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_set_up_rules(this: usize) -> u32 {
    covered!("cqi_4 third division rules");
    eng_prm_apply_league_rules(League(this), &third_division_rules(this))
}

/// the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999)
/// Original: `EngThird::create_table` (0x577f70, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x18, 0x9c4424, 0x1fa, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || eng_prm_pay_tv_rights(this, if is_after_1999() { 0xf4240 } else { 0x30d40 }, 0))
}

/// the Division Three schedule: the same 46 rounds as Division Two. The promotion play-offs (stage 0): two-legged
/// semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 26 May [inference: venue
/// 4 is the neutral ground]
/// Original: `EngThird::create_schedule` (0x5770e0, eng_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 46;
            *extra = 0;
            let t = exe::crt_malloc(0xbae);
            if t == 0 {
                show_error_at(0x9c4424, 0xb3);
                return 0;
            }
            covered!("cqp_4 english third division calendar");
            let year = (this.year() as u16) as u32;
            eng_second_set_christmas_rounds(t, year, &THIRD_DIVISION_CHRISTMAS_SCHEDULE, 0x9c4424);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 2;
            *extra = 0xa0;
            let t = exe::crt_malloc(0xd0);
            if t == 0 {
                show_error_at(0x9c4424, 0x1ba);
                return 0;
            }
            covered!("cqp_4 english third division play-offs");
            let year = this.year() as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &THIRD_DIVISION_PLAYOFF_SEMIFINAL, year);
            eng_cc_cup_write_cup_round(t, 1, &DIVISION_THREE_PLAYOFF_1, year);
            t
        }
        _ => 0,
    }
}

/// like the First one, with places 4 to 7 (4 and 7; in the play-offs, 4, 5 and 6)
/// Original: `EngThird::set_club_progress` (0x578660, eng_third.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn eng_third_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_club_stage_status(this, club, stage as u8, current as u8, best as u8, worst as u8, 4, 7)
}

/// after the Third Division league, the current stage goes up and, on the first, the promotion play-off is created
/// (`eng_third_create_play_off_stage`)
/// Original: `EngThird::advance_stage` (0x578330, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_advance_stage(League(this), 0x9c4424, 0x29c, 0x5783c0)
}

/// The season turnover of the English Third Division: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (eng_third.cpp, lines 0x26c, 0x273,
/// 0x279 and 0x280) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (EngThird::create_table at 0x577f70, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `EngThird::start_new_season` (0x578170, eng_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x577f70, 0x9c4424, [0x26c, 0x273, 0x279, 0x280], 1) { // mwf6: the turnover of the English Third Division
        return 0;
    }
    covered!("mwf_6 English Third Division rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `eng_third_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("eng_third_start_new_season", League(this), || o(this))
}
