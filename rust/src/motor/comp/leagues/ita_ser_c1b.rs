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

/// The season turnover of Serie C1 B: the season archived in the statistics, the table, the round dates and the stages
/// cleared, the year moved on by one and the rules, the table, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// The league turnover template of Brazil's Series B (`bra_nat_second_start_new_season`, checked instruction by
/// instruction): `league::SEASON_CLOSED` = 0, then `rebuild_season`: the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed (`exe::crt_free`) and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1),
/// ret 4); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1; the stage array
/// itself is kept); the year up by one and `league::CURRENT_STAGE` = −1. Then, each with its own box (ita_ser_c1b.cpp, lines 0x224, 0x22b,
/// 0x231 and 0x238) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerC1b::create_table at 0x64f820, thiscall, ret 0),
/// the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), the board expectations (League::set_club_season_expectations) and the
/// referee renewal (OfficialsManager::renew_officials `exe::officialsmanager_renew_officials`,
/// thiscall([`G_OFFICIALS_MANAGER`], `.competition()`), ret 4), whose eax it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerC1b::start_new_season` (0x64f9e0, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x64f820, 0x9ea558, [0x224, 0x22b, 0x231, 0x238], 1) { // mwf8: the turnover of Serie C1 B
        return 0;
    }
    covered!("mwf_8 Serie C1 B rollover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_c1b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_C1B_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_c1b_start_new_season", League(this), || o(this))
}

/// The Serie C1 B final positions: as League::record_final_positions for the table, then the promotion play-off
/// overrides the places of its clubs: the winner 2nd, the runner-up 3rd and the others 4th, 5th... in the order of the
/// play-off's team table.
///
/// The play-off is stage 0 (`.stages()`[0], read first). Each club of the table rows 0..`.club_count()` (a signed
/// word) whose division (`club.division()`) is this league stores `club::LAST_DIVISION` = the league and the place
/// i + 1 (`set_last_position`). Then each entry of the play-off's team table (`cup::TEAM_TABLE`, 6-byte entries,
/// `.team_count()` of them, a signed word): status (+5) 1 gives place 2, status 2 gives place 3, any other the next
/// place from 4 on (a byte counter).
/// Original: `ItaSerC1b::record_final_positions` (0x650050, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_record_final_positions(this: usize) {
    let this = League(this);
    let playoff = Cup(read_usize(this.stages()));
    let mut i = 0i16;
    while i < this.club_count() {
        let division = this.competition().addr();
        let c = Club(read_usize(this.table() + i as usize * TABLE_ROW_SIZE));
        if c.division().addr() == division {
            write_usize(c.addr() + club::LAST_DIVISION, division);
            c.set_last_position((i as u8).wrapping_add(1));
        }
        i += 1;
    }
    covered!("mwf_10 serie c1 b play-off places");
    let mut next = 4u8;
    let mut k = 0i16;
    while k < playoff.team_count() {
        let e = playoff.team_table() + k as usize * 6;
        let c = Club(read_usize(e));
        match read_u8(e + 5) {
            1 => c.set_last_position(2), // mwf10: the play-off winner 2nd
            2 => c.set_last_position(3),
            _ => {
                c.set_last_position(next);
                next = next.wrapping_add(1);
            }
        }
        k += 1;
    }
}

/// Comparer for `ita_ser_c1b_record_final_positions`: side by side (it only writes `club::LAST_DIVISION` and
/// `club::LAST_POSITION` of the table and play-off clubs, so both can run from the same state), over those bytes.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c1b_record_final_positions(this: usize) {
    let l = League(this);
    let mut reg: Vec<(usize, usize)> = (0..l.club_count().max(0) as usize).map(|i| (read_usize(l.table() + i * TABLE_ROW_SIZE) + club::LAST_DIVISION, 5)).collect();
    let playoff = Cup(read_usize(l.stages()));
    reg.extend((0..playoff.team_count().max(0) as usize).map(|k| (read_usize(playoff.team_table() + k * 6) + club::LAST_POSITION, 1)));
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_C1B_RECORD_FINAL_POSITIONS);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || ita_ser_c1b_record_final_positions(this));
    SCOREBOARD.record(same, || format!("ita_ser_c1b_record_final_positions({:#x}) {:?} {:?}", this, mo, mr));
}
