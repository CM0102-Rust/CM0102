// ---------- cx12_10: the Northern Irish Premier Division club progress (nir_prm.cpp) ----------

/// The body of the club progress of a Premier Division with a play-off place against the First Division (Northern
/// Ireland, Norway): the club's entry in this competition's statistics (`exe::comp_stats_find_club_entry`,
/// unchecked) gets the current code as it comes, the best code `play_off` becoming `best` and the worst code
/// `play_off` becoming `worst`, whatever the stage.
pub(crate) unsafe fn premier_play_off_progress(this: usize, club: usize, current: u32, best_code: u32, worst_code: u32, play_off: u8, best: u8, worst: u8) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let b = if best_code as u8 == play_off { best } else { best_code as u8 };
    let w = if worst_code as u8 == play_off { worst } else { worst_code as u8 };
    write_u8(e + comp_stats_club::CURRENT_POSITION, current as u8);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// The club's progress codes in the Northern Irish Premier Division (`premier_play_off_progress`): code 9 (the
/// play-off place) becomes 8 as the best and 10 as the worst.
/// Original: `NirPrm::set_club_progress` (0x78efe0, nir_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn nir_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx12_10 northern irish premier progress");
    premier_play_off_progress(this, club, current, best, worst, 9, 8, 0xa) // cx12_10: the play-off place
}

/// Comparer for `nir_prm_set_club_progress`: side by side on the 3 bytes of the club's entry (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_nir_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_c1_club_progress("nir_prm_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || nir_prm_set_club_progress(this, club, stage, current, best, worst))
}
