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
