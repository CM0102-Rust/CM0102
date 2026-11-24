// ---------- cx10_10: the progress codes of the Dutch Premier Division and the German Regional League ----------

/// Stores a club's three progress codes in its statistics entry (`store_club_progress`), the play-off codes 16 and 17
/// becoming 15 in the best code and 18 in the worst code; the current code stays.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes; the stage is not read.
/// Original: `HolPrm::set_club_progress` (0x5f87e0, hol_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn hol_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let (mut best, mut worst) = (best as u8, worst as u8);
    if (0x10..=0x11).contains(&best) {
        covered!("cx10_10 dutch play-off best code");
        best = 0xf;
    }
    if (0x10..=0x11).contains(&worst) {
        worst = 0x12;
    }
    store_club_progress(this, club, current as u8, best, worst);
}

/// Stores a club's three progress codes in its statistics entry (`store_club_progress`) as odd values: each code c
/// becomes 2c − 1 (byte arithmetic), the two Regional sections sharing one table of positions.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes; the stage is not read.
/// Original: `GerRegional::set_club_progress` (0x5de2a0, ger_regional.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ger_regional_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let odd = |c: u32| (c as u8).wrapping_mul(2).wrapping_sub(1);
    store_club_progress(this, club, odd(current), odd(best), odd(worst));
}

pub(crate) unsafe extern "thiscall" fn compare_hol_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("hol_prm_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || hol_prm_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_ger_regional_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("ger_regional_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || ger_regional_set_club_progress(this, club, stage, current, best, worst));
}
