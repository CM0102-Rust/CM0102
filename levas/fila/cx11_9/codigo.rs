// ---------- cx11_9: the club progress of the Serie C2 groups A and B (ita_ser_c2a.cpp, ita_ser_c2b.cpp) ----------

/// The club's place forecast in Serie C2/A, the same rule as Serie C1's (`set_c1_club_progress`, checked instruction by
/// instruction, jump tables included): in the league, best and worst place become bands (2 to 5, the promotion
/// play-off: best 2, worst 5; 14 to 17, the play-out: best 14, worst 17); in the play-off, the three places + 1; in the
/// play-out, best 1 becomes 14 and 2 becomes 16; written to the club's entry in the competition statistics (current,
/// best, worst).
/// Original: `ItaSerC2a::set_club_progress` (0x6544b0, ita_ser_c2a.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx11_9 serie c2 progress");
    set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx11_9: the C2/A progress
}

/// The same rule in Serie C2/B.
/// Original: `ItaSerC2b::set_club_progress` (0x658640, ita_ser_c2b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8) // cx11_9: the C2/B progress
}

/// Comparer for `ita_ser_c2a_set_club_progress`: side by side on the 3 bytes of the club's entry
/// (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_c1_club_progress("ita_ser_c2a_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || ita_ser_c2a_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `ita_ser_c2b_set_club_progress`: the same.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_c1_club_progress("ita_ser_c2b_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || ita_ser_c2b_set_club_progress(this, club, stage, current, best, worst))
}
