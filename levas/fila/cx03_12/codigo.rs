// ---------- cx03_12: the Portuguese Cup round set-up, the Swedish Cup's day and the Swedish Cup progress (por_cup.cpp, swe_cup.cpp) ----------

/// The rounds of the Portuguese Cup and of the four other cups that share the virtual (the Belgian, Russian and Swedish
/// Cups and vtable 0x96dd80): a prize fund of 800,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `PorCup::set_up_rounds` (0x7ce6f0, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_12 cup rounds with an 800,000 prize fund");
    set_up_prize_rounds(Cup(this), 800_000)
}

/// rule: the Swedish Cup starts its next edition on 1 July (month 6 counted from 0) once the current one is over; then
/// the cup's day (`update_cup_with_turnover_on`).
/// Original: `SweCup::update_for_date` (0x88d8a0, swe_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn swe_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    update_cup_with_turnover_on(Cup(this), today, stage, 1, 6, "cx03_12 swedish cup next edition") // cx03_12: the Swedish Cup on 1 July
}

/// rule: in the Swedish Cup's seven preliminary sections (stages below 7) a club's progress codes n become 8n − 7 (byte
/// arithmetic), the section's place on the scale of the 64-club main draw [inference]; in the main draw the codes as
/// they come (`store_club_progress`).
/// Original: `SweCup::set_club_progress` (0x88e580, swe_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 7 {
        covered!("cx03_12 swedish cup section progress");
        let scale = |code: u8| (code << 3).wrapping_sub(7);
        current = scale(current);
        best = scale(best);
        worst = scale(worst);
    }
    store_club_progress(this, club, current, best, worst);
}

pub(crate) unsafe extern "thiscall" fn compare_por_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("por_cup_set_up_rounds", Cup(this), || o(this), || por_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_swe_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    compare_update_cup_with_turnover_on("swe_cup_update_for_date", Cup(this), today, stage, 1, 6, || o(this, today, stage))
}

pub(crate) unsafe extern "thiscall" fn compare_swe_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_progress("swe_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || swe_cup_set_club_progress(this, club, stage, current, best, worst));
}
