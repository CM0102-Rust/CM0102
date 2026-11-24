// ---------- cx10_11: the progress codes of the Finnish Premier Division ----------

/// Stores a club's three progress codes in its statistics entry (`store_club_progress`), the worst code 11 becoming 12;
/// the current and best codes stay.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes; the stage is not read.
/// Original: `FinPrm::set_club_progress` (0x596020, fin_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn fin_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    let mut worst = worst as u8;
    if worst == 0xb {
        covered!("cx10_11 finnish worst code 11");
        worst = 0xc;
    }
    store_club_progress(this, club, current as u8, best as u8, worst);
}

pub(crate) unsafe extern "thiscall" fn compare_fin_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("fin_prm_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || fin_prm_set_club_progress(this, club, stage, current, best, worst));
}
