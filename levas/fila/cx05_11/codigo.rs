// ---------- cx05_11: the European Championship qualification status of a nation and the qualified entrants (euro_champ.cpp) ----------

/// The low byte (signed) of a nation's statistic `stat` in the European Championship statistics
/// (`exe::comp_stats_get_club_stat` on `league::COMP_STATS`, a float, truncated as the original's _ftol).
unsafe fn euro_champ_position_band(this: usize, club: usize, stat: u32) -> i8 {
    ftol(exe::comp_stats_get_club_stat(read_usize(this + league::COMP_STATS), club, stat)) as u8 as i8
}

/// A nation's qualification status in the European Championship, from its current position code (statistic 0x12):
/// 1 (the winner) −1, 2 (the runner-up) 150, below 5 (the semi-finalists) 130, 5 to 8 (the quarter-finalists) 120,
/// from 9 (out in the groups) −4.
/// Original: `EuroChamp::get_qualification_status` (0x57cd30, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_get_qualification_status(this: usize, club: usize) -> i16 {
    let p = euro_champ_position_band(this, club, 0x12);
    if p == 1 {
        -1
    } else if p == 2 {
        0x96
    } else if p < 5 {
        0x82
    } else if p >= 9 { // cx05_11: out in the groups
        -4
    } else {
        0x78
    }
}

/// Comparer for `euro_champ_get_qualification_status`: both run (they only read the statistics) and return the same.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_get_qualification_status(this: usize, club: usize) -> i16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I0%%)(this, club);
    let n = euro_champ_get_qualification_status(this, club);
    SCOREBOARD.record(r == n, || format!("euro_champ_get_qualification_status({:#x}) original={} rust={}", club, r, n));
    r
}

/// The change of a nation's qualification band in the European Championship, from its previous (statistic 0x13) and
/// current (0x12) position codes, both banded as below 5, 5 to 8 and from 9: current below 3 against a previous below 5
/// 0, 5 to 8 −1, from 9 −2; current 3 to 8 against a previous below 5 2, 5 to 8 0, from 9 −1; current from 9 against a
/// previous below 5 3, 5 to 8 2, from 9 0 (signed bytes).
/// Original: `EuroChamp::get_qualification_status_change` (0x57cd80, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_get_qualification_status_change(this: usize, club: usize) -> u8 {
    let previous = euro_champ_position_band(this, club, 0x13);
    let current = euro_champ_position_band(this, club, 0x12);
    let band = |p: i8| if p < 5 { 0 } else if p < 9 { 1 } else { 2 };
    let change: i8 = if current < 3 {
        [0, -1, -2][band(previous)]
    } else if current < 9 {
        [2, 0, -1][band(previous)] // cx05_11: the change from a middle band
    } else {
        [3, 2, 0][band(previous)]
    };
    change as u8
}

/// Comparer for `euro_champ_get_qualification_status_change`: both run (they only read the statistics) and return the
/// same byte.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_get_qualification_status_change(this: usize, club: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, club);
    let n = euro_champ_get_qualification_status_change(this, club);
    SCOREBOARD.record(r == n, || format!("euro_champ_get_qualification_status_change({:#x}) original={} rust={}", club, r as i8, n as i8));
    r
}

/// Adds a qualified nation to the European Championship entrant book: with the book full (`.entrant_count()` not below
/// `.entrant_capacity()`, signed words) the box at line 0x365 and nothing more; otherwise the next entry gets the team
/// and the status byte 6 (+4) and the count goes up.
/// Original: `EuroChamp::add_qualified_team` (0x57c120, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_add_qualified_team(this: usize, team: usize) {
    let league = League(this);
    if league.entrant_count() >= league.entrant_capacity() {
        return show_error_at(0x9c4638, 0x365);
    }
    covered!("cx05_11 euro qualified team");
    let entry = league.entrant_book() + league.entrant_count() as isize as usize * 6;
    write_u8(entry + 4, 6); // cx05_11: the qualified status
    write_usize(entry, team);
    league.set_entrant_count(league.entrant_count().wrapping_add(1));
}

/// Comparer for `euro_champ_add_qualified_team`: side by side on the count and the entry it writes (nothing is
/// allocated).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_add_qualified_team(this: usize, team: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let league = League(this);
    if league.entrant_book() == 0 || league.entrant_count() >= league.entrant_capacity() {
        return o(this, team);
    }
    let entry = league.entrant_book() + league.entrant_count() as isize as usize * 6;
    let regions = [(this + league::ENTRANT_COUNT, 2), (entry, 6)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, team), || euro_champ_add_qualified_team(this, team));
    SCOREBOARD.record(same, || format!("euro_champ_add_qualified_team({:#x}) original={:x?} rust={:x?}", team, mo, mr));
}
