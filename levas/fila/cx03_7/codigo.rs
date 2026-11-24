// ---------- cx03_7: the Russian Cup's day (rus_cup.cpp) ----------

/// Whether today is Date(`day`, `month`, today's year) (months from 0, any weekday: `make_date_dword`).
unsafe fn is_today_on(today: usize, day: u32, month: u32) -> bool {
    // simplification: the original pushes today's year with garbage in the high 16 bits; Date::Date reads the word
    is_date_today(make_date_dword(day, month, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today)
}

/// Whether a finished cup turns over today: `stage` set, the cup and its chain complete (virtual +0xc,
/// `Comp::all_stages_complete`, thiscall, ret 0, in al) and today on Date(`day`, `month`).
unsafe fn cup_turnover_due(this: Cup, today: usize, stage: u32, day: u32, month: u32) -> bool {
    if stage == 0 {
        return false;
    }
    let complete: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(read_usize(read_usize(this.addr()) + 0xc)); // Comp::all_stages_complete
    complete(this.addr()) != 0 && is_today_on(today, day, month)
}

/// The daily virtual (`comp_vtable::UPDATE_FOR_DATE`) of the cups that start their next edition on a fixed day once the
/// current one is over: when due (`cup_turnover_due`) the season turnover (virtual `comp_vtable::START_NEW_SEASON`,
/// thiscall, ret 0); then, every day, the cup's day (`exe::cup_update_for_date`, the same arguments), whose result it
/// returns.
pub(crate) unsafe fn update_cup_with_turnover_on(this: Cup, today: usize, stage: u32, day: u32, month: u32, branch: &'static str) -> u32 {
    if cup_turnover_due(this, today, stage, day, month) {
        covered!(branch);
        let turn: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::START_NEW_SEASON));
        turn(this.addr());
    }
    exe::cup_update_for_date(this, today, stage)
}

/// Comparer of `update_cup_with_turnover_on`: only the original runs (the turnover frees, allocates and draws; the
/// cup's day plays); the plan: when the turnover was due, the cup year goes up by one, otherwise it stays.
pub(crate) unsafe fn compare_update_cup_with_turnover_on(name: &str, this: Cup, today: usize, stage: u32, day: u32, month: u32, original: impl FnOnce() -> u32) -> u32 {
    let due = cup_turnover_due(this, today, stage, day, month);
    let before = this.year();
    let r = original();
    let after = this.year();
    let expected = if due { before.wrapping_add(1) } else { before };
    SCOREBOARD.record(after == expected, || format!("{}({:#x}, stage {}) due {} year {} -> {}", name, this.addr(), stage, due, before, after));
    r
}

/// rule: the Russian Cup starts its next edition on 1 June (month 5 counted from 0) once the current one is over; then
/// the cup's day (`update_cup_with_turnover_on`).
/// Original: `RusCup::update_for_date` (0x7e9ec0, rus_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rus_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    update_cup_with_turnover_on(Cup(this), today, stage, 1, 5, "cx03_7 russian cup next edition") // cx03_7: the Russian Cup on 1 June
}

pub(crate) unsafe extern "thiscall" fn compare_rus_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    compare_update_cup_with_turnover_on("rus_cup_update_for_date", Cup(this), today, stage, 1, 5, || o(this, today, stage))
}
