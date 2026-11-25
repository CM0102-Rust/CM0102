// ---------- rx13_12: the League of Wales table (wel_first.cpp) ----------

/// rule: the League of Wales table has 18 clubs, the clubs of the Welsh league division in database order; a prize
/// fund of 10,000 shared by position; without 18 clubs the table is refused
///
/// `fill_british_division_table` with 18 clubs, the box of `wel_first.cpp` at line 0x194 (no table),
/// `KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`, the prize fund `League::calc_prize_fund(10000)` (`league::PRIZE_MODE` 1)
/// and no TV money. Returns 1 when full, else 0.
/// Original: `WelFirst::create_table` (0x925b30, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0x12, LEAGUE_OF_WALES.file, 0x194, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, Some(10000), None, 0, "rx13_12 league of wales table") // rx13_12: 18 clubs
}

/// Comparer for `wel_first_create_table` (`compare_british_division_table`: only the original, against the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_british_division_table("wel_first_create_table", League(this), 0x12, KEY_CLUB_COMP_WELSH_PREMIER_DIVISION, Some(10000), || o(this))
}
