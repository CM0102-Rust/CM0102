// ---------- cx03_13: the round set-up of the super cups and of the Asian club cups, and the Asian Super Cup's day (spa_super.cpp, asia_cup_winner.cpp, asia_super_cup.cpp) ----------

/// The rounds of the Spanish Super Cup and of the 22 other cups that share the virtual (mostly the super cups): only the
/// round table (`build_cup_round_table_by_vtable`, the prize fund left as it is). Returns 1.
/// Original: `SpaSuper::set_up_rounds` (0x858e70, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_super_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_13 cup rounds without a prize fund");
    build_cup_round_table_by_vtable(Cup(this));
    1
}

/// The rounds of the Asian Cup Winners' Cup and of the Asian Club Championship (the same virtual): a prize fund of
/// 4,000,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `AsiaCupWinner::set_up_rounds` (0x40d800, asia_cup_winner.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_cup_winner_set_up_rounds(this: usize) -> i32 {
    covered!("cx03_13 asian club cup rounds");
    set_up_prize_rounds(Cup(this), 4_000_000)
}

/// rule: the Asian Super Cup starts its next edition on 25 June (month 5 counted from 0) once the current one is over;
/// then the cup's day (`update_cup_with_turnover_on`).
/// Original: `AsiaSuperCup::update_for_date` (0x410e20, asia_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn asia_super_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    update_cup_with_turnover_on(Cup(this), today, stage, 0x19, 5, "cx03_13 asian super cup next edition") // cx03_13: the Asian Super Cup on 25 June
}

pub(crate) unsafe extern "thiscall" fn compare_spa_super_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("spa_super_set_up_rounds", Cup(this), || o(this), || spa_super_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("asia_cup_winner_set_up_rounds", Cup(this), || o(this), || asia_cup_winner_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_asia_super_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    compare_update_cup_with_turnover_on("asia_super_cup_update_for_date", Cup(this), today, stage, 0x19, 5, || o(this, today, stage))
}
