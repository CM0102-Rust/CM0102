// ---------- cx16_12: the European World Cup qualifying table update and its play-off qualifiers (wc_europe_league.cpp) ----------

/// WcEuropeLeague::mark_play_off_qualifiers and League::update_positions_and_status (thiscall, ret 0), called directly
/// by the table update (the second through its detour).
pub(crate) const FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS: usize = 0x91a7e0;
pub(crate) const FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS: usize = 0x684cd0;

/// rule: the European qualifying table update is the league's, and once the whole qualifying is complete the play-off
/// winners are marked as qualified (once)
///
/// League::update_positions_and_status (`FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS`, through its detour); then, while the
/// done flag (`wc_cup_turnover_consts::EUROPE_FB`) is 0 and the section is complete (virtual `comp_vtable::CHECK_COMPLETE`
/// with 1, 0: push 0; push 1; mov ecx -> thiscall, ret 8; al), WcEuropeLeague::mark_play_off_qualifiers (0x91a7e0).
/// Original: `WcEuropeLeague::update_positions_and_status` (0x91a7b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_positions_and_status(this: usize) {
    call_section_routine(FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS, this);
    if read_u32(this + wc_cup_turnover_consts::EUROPE_FB) != 0 {
        return;
    }
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CHECK_COMPLETE));
    if complete(this, 1, 0) != 0 {
        covered!("cx16_12 european qualifying complete");
        call_section_routine(FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS, this);
    }
}

/// Comparer for `wc_europe_league_update_positions_and_status`: the table update cannot run twice; only the original.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// The European group table of group `g` (−1 the section itself, else `league::STAGES`[g]).
pub(crate) unsafe fn wc_europe_group(this: usize, g: i32) -> League {
    if g == -1 { League(this) } else { League(read_usize(League(this).stages().wrapping_add((g as isize as usize).wrapping_mul(4)))) }
}

/// rule: when the European qualifying ends, the play-off winners get the qualified status (7) in their group tables;
/// without a play-off stage the best group runner-up qualifies instead, with its staff record
///
/// Once (the done flag `wc_cup_turnover_consts::EUROPE_FB` set to 1 on the first call; set already, nothing). Without a
/// play-off stage (+0x101 = −1): the top club of the runners-up stage's table (`league::STAGES`[+0x100], row 0) is
/// looked for in the group tables −1 to the last group (+0xff), in order; found, its row's `league_table_row::STATUS`
/// is 7 and the section's staff records for it (virtual `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS`(club, 7, group,
/// 0, 0, 0): thiscall, ret 0x18); not found, the error box of line 0x65f. With one: for each group, each play-off team
/// row (`cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT` of the play-off stage) whose status (+5) is 1, its row in the
/// group (League::get_table_row `exe::league_get_table_row`, thiscall, ret 4) gets the status 7.
/// Original: `WcEuropeLeague::mark_play_off_qualifiers` (0x91a7e0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_mark_play_off_qualifiers(this: usize) {
    use wc_cup_turnover_consts as wt;
    if read_u32(this + wt::EUROPE_FB) != 0 {
        return;
    }
    write_u32(this + wt::EUROPE_FB, 1);
    let last = read_u8(this + 0xff) as i8 as i32;
    let play_off = read_u8(this + 0x101) as i8;
    if play_off == -1 {
        let runners = wc_europe_group(this, read_u8(this + 0x100) as i8 as i32);
        let best = read_usize(runners.table());
        for g in -1..=last {
            let group = wc_europe_group(this, g);
            for k in 0..group.club_count().max(0) as usize {
                let row = group.table() + k * league_table_row::SIZE;
                if read_usize(row) == best {
                    covered!("cx16_12 european best runner-up qualifies");
                    write_u8(row + league_table_row::STATUS, 7);
                    let records: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(read_usize(read_usize(this) + comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS));
                    records(this, best, 7, g as u32, 0, 0, 0);
                    return;
                }
            }
        }
        return show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x65f);
    }
    covered!("cx16_12 european play-off qualifiers");
    for g in -1..=last {
        let group = wc_europe_group(this, g);
        let mut i = 0i32;
        loop {
            let stage = Cup(wc_europe_group(this, read_u8(this + 0x101) as i8 as i32).addr());
            if i >= stage.team_count() as i32 {
                break;
            }
            let team = stage.team_table() + i as usize * 6;
            if read_u8(team + 5) == 1 { // cx16_12: a play-off winner
                let mut k = 0i32;
                while k < group.club_count() as i32 {
                    let row = exe::league_get_table_row(group, k);
                    if read_usize(row) == read_usize(team) {
                        write_u8(exe::league_get_table_row(group, k) + league_table_row::STATUS, 7);
                        break;
                    }
                    k += 1;
                }
            }
            i += 1;
        }
    }
}

/// Comparer for `wc_europe_league_mark_play_off_qualifiers`: it runs once (the flag) and adds staff records, so only the
/// original runs; then the plan: the flag set and, with a play-off stage, every winner's row in its group with status 7.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_mark_play_off_qualifiers(this: usize) {
    let first = read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let play_off = read_u8(this + 0x101) as i8;
    if !first || play_off == -1 {
        return;
    }
    let stage = Cup(wc_europe_group(this, play_off as i32).addr());
    let last = read_u8(this + 0xff) as i8 as i32;
    let mut missing = Vec::new();
    for i in 0..stage.team_count().max(0) as usize {
        let team = stage.team_table() + i * 6;
        if read_u8(team + 5) != 1 {
            continue;
        }
        let marked = (-1..=last).any(|g| {
            let group = wc_europe_group(this, g);
            (0..group.club_count().max(0) as usize).any(|k| {
                let row = group.table() + k * league_table_row::SIZE;
                read_usize(row) == read_usize(team) && read_u8(row + league_table_row::STATUS) == 7
            })
        });
        if !marked {
            missing.push(read_usize(team));
        }
    }
    SCOREBOARD.record(read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 1 && missing.is_empty(), || format!("wc_europe_league_mark_play_off_qualifiers({:#x}): winners not marked {:x?}", this, missing));
}
