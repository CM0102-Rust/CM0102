// ---------- cx09_17: the progress of the Gaucho and Goias state championships (bra_reg_gaucho.cpp, bra_reg_goias.cpp) ----------

/// Stores a club's progress in the Rio Grande do Sul state championship (`league_stage_progress`: in the league a best
/// code below 5 becomes 1 and a worst code below 5 becomes 3).
/// Original: `BraRegGaucho::set_club_progress` (0x432af0, bra_reg_gaucho.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_17 gaucho progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 3)
}

/// Stores a club's progress in the Goias state championship (`league_stage_progress`: in the league a best code below 9
/// becomes 1 and a worst code below 9 becomes 5).
/// Original: `BraRegGoias::set_club_progress` (0x433cf0, bra_reg_goias.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_17 goias progress");
    league_stage_progress(this, club, stage, current, best, worst, 9, 5)
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I0%%);
    compare_league_stage_progress("bra_reg_gaucho_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_reg_gaucho_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_goias_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I1%%);
    compare_league_stage_progress("bra_reg_goias_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_reg_goias_set_club_progress(this, club, stage, current, best, worst))
}
