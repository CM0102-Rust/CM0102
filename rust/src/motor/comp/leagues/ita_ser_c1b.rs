use super::*;

/// the same for the other Serie C1 group (the same table)
/// Original: `ItaSerC1b::create_schedule` (0x64ef30, ita_ser_c1b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea558, rows: [0xac, 0x156, 0x190] })
}

/// the same rule in C1/B
/// Original: `ItaSerC1b::set_club_progress` (0x64ff40, ita_ser_c1b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8)
}

/// the 2001/02 C1/B, with the same rule
/// Original: `ItaSerC1b::set_up_fixed_fixtures` (0x650100, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_up_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures_body(League(this), &SERIE_C1B_FIXTURES, 0x9ea558, 0x404)
}

/// C1/B, as C1/A
/// Original: `ItaSerC1b::ItaSerC1b` (0x64ec20, ita_ser_c1b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c1b(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(League(this), year, record, &SERIE_C1B_DIVISION)
}

/// `ItaSerC1b::set_up_rules()`: Serie C1/B (18 clubs), set up like C1/A. Above it the id of
/// `KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`, and below it C2/A (`KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`) when the nation has its
/// lower divisions loaded (bit 4 of the nation's `.league_selected()` [inference]), otherwise −1.
/// Original: `ItaSerC1b::set_up_rules` (0x64f920, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_up_rules(this: usize) -> u32 {
    covered!("cqi_6 c1b rules");
    ita_ser_a_apply_league_rules(League(this), &serie_c1b_rules(this))
}

/// Serie C1/B has 18 clubs; no prize fund and no TV money
/// Original: `ItaSerC1b::create_table` (0x64f820, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_create_table(this: usize) -> u32 {
    ita_ser_a_fill_league_table(League(this), 0x12, 0x9ea558, 0x1be, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B).unwrap_or(false) as u32
}

/// the same as C1/A
/// Original: `ItaSerC1b::advance_stage` (0x64fba0, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_advance_stage(League(this), 0x9ea558, 0x255, 0x64fc30)
}

/// the same rule in C1/B
/// Original: `ItaSerC1b::create_play_off_stages` (0x64fc30, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_play_off_stages(this: usize) {
    create_end_of_league_stages(League(this), 0x9ea558, [0x281, 0x295, 0x2b8], false)
}
