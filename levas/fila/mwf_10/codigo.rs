// ---------- mwf_10: the Serie B season turnover and the Serie C1 B final positions (ita_ser_b.cpp, ita_ser_c1b.cpp) ----------

/// The season turnover of Serie B: the season archived, the table and the dates cleared, the stage array dropped, the
/// year plus one, then the rules, the 20 Serie B clubs, the fixtures and the order rebuilt; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the stage array (`.stages()`) freed when
/// there is one, without destroying the stage objects (Serie B has none [inference]), and zeroed, `league::STAGE_COUNT`
/// = 0; the year up by one; `league::CURRENT_STAGE` = −1; and `league::SEASON_CLOSED` = 1 (as Serie A's turnover
/// also does; the other sisters leave 0; a possible defect of the original [inference: the end of the next season may
/// be taken as already closed]).
///
/// Then, each with its own box (ita_ser_b.cpp, lines 0x20c, 0x213, 0x219 and 0x220) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerB::create_table 0x646780, thiscall, ret 0: the 20 Serie B
/// clubs and a prize fund from 70000), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the
/// order (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); then `set_expectations_and_renew_officials`
/// after the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax (the referee renewal's)
/// it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read
/// it).
/// Original: `ItaSerB::start_new_season` (0x646960, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
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
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf10: the year of Serie B
    this.set_stages(0);
    write_u32(this.addr() + league::STAGE_COUNT, 0);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_season_closed(1);
    covered!("mwf_10 serie b rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ea384, 0x20c);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x646780usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9ea384, 0x213);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ea384, 0x219);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ea384, 0x220);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ita_ser_b_start_new_season", League(this), || o(this))
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || ita_ser_c1b_record_final_positions(this));
    SCOREBOARD.record(same, || format!("ita_ser_c1b_record_final_positions({:#x}) {:?} {:?}", this, mo, mr));
}

