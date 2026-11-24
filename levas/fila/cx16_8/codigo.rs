// ---------- cx16_8: the qualification status of a nation in the European, CONCACAF and Oceania World Cup qualifying (wc_europe_league.cpp, wc_oceania_league.cpp) ----------

/// A nation's competition stat `stat` in a qualifying section's statistics (Comp_Stats::get_club_stat
/// `exe::comp_stats_get_club_stat`(`league::COMP_STATS`, club, stat): push stat; push club; mov ecx -> thiscall, ret 8,
/// a float), truncated (`ftol`), its low byte signed.
pub(crate) unsafe fn qualifying_stat_byte(this: usize, club: usize, stat: u32) -> i8 {
    ftol(exe::comp_stats_get_club_stat(read_usize(this + league::COMP_STATS), club, stat)) as u8 as i8
}

/// rule: a nation's status in the European qualifying is still open (−2) while its stat 0x12 (its current progress
/// code) is below 12, and decided (−3) from 12 on
///
/// `qualifying_stat_byte`(club, 0x12) below 12: −2, otherwise −3 (a word).
/// Original: `WcEuropeLeague::get_qualification_status` (0x91b6e0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx16_8 european qualification status");
    if qualifying_stat_byte(this, club, 0x12) < 0xc { -2 } else { -3 } // cx16_8: open below 12
}

/// rule: in the European qualifying a nation's status changes down (−1) when its progress code (stat 0x12) falls below
/// 12 from 12 or more (stat 0x13, the previous code), up (1) when it rises from below 12 to 12 or more, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now), signed: now below 12, (before below 12) − 1; otherwise
/// before below 12 (1 or 0). The original returns the byte in al.
/// Original: `WcEuropeLeague::get_qualification_status_change` (0x91b710, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 european status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now < 0xc {
        (before < 0xc) as i8 - 1
    } else {
        (before < 0xc) as i8
    }
}

/// rule: in the Oceania and CONCACAF qualifying a nation's status changes up (1) when its progress code (stat 0x12)
/// leaves 1 (stat 0x13, the previous code, was 1), down (−1) when it becomes 1 from another code, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 when before was not 1 (else 0); otherwise 1 when
/// before was 1 (else 0). The byte in al. Shared by the CONCACAF and Oceania vtables (+0x74).
/// Original: `WcOceaniaLeague::get_qualification_status_change` (0x920260, wc_oceania_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 oceania status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -((before != 1) as i8) // cx16_8: became 1
    } else {
        (before == 1) as i8
    }
}

/// Comparers: the three only read the statistics; side by side on the result (the byte or word).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I0%%)(this, club);
    let b = wc_europe_league_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I1%%)(this, club);
    let b = wc_europe_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I2%%)(this, club);
    let b = wc_oceania_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}
