// ---------- batch %%LEVA%%: the season turnovers of the Finnish, Norwegian and Portuguese cups (fin_cup.cpp, nor_cup.cpp, por_cup.cpp) ----------

/// The Finnish Cup turnover: last season's entrant, round and tie tables freed, the season archived in the statistics,
/// the year + 1; then the entrants (FinCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// `FinCup::start_new_season()`: the body of `start_new_cup_season` (checked instruction by instruction: only the
/// entrants routine 0x592a20, the source fin_cup.cpp (0x9c8158) and the box lines 0x206, 0x20d, 0x214 change): +0x4c =
/// 0; `cup::TEAM_TABLE` and the round list freed and zeroed; the tie arrays (one per round, `.round_count()` of them)
/// and their vector freed and zeroed; `exe::comp_stats_archive_season_and_reset`(1); the year + 1; the current round
/// and the stage flag 0; the entrants (direct, thiscall, ret 0), the rounds (virtual `comp_vtable::SET_UP_RULES`) and
/// the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`), each with its box when it returns a negative value (and 0
/// returned); last the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is returned.
/// Original: `FinCup::start_new_season` (0x592d00, fin_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x592a20, 0x9c8158, [0x206, 0x20d, 0x214], 1, "mwe_16 finnish cup turnover") // mwe16: the Finnish Cup next year
}

/// The Norwegian Cup turnover: the same as the Finnish Cup's, with NorCup::collect_entrants (0x78f890), nor_cup.cpp
/// (0xa69708) and the box lines 0x1e8, 0x1ef, 0x1f6.
/// Original: `NorCup::start_new_season` (0x78fb90, nor_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x78f890, 0xa69708, [0x1e8, 0x1ef, 0x1f6], 1, "mwe_16 norwegian cup turnover") // mwe16: the Norwegian Cup next year
}

/// The Portuguese Cup turnover: the same as the Finnish Cup's, with PorCup::collect_entrants (0x7ce220), por_cup.cpp
/// (0xa786f0) and the box lines 0x20d, 0x214, 0x21b.
/// Original: `PorCup::start_new_season` (0x7ce540, por_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7ce220, 0xa786f0, [0x20d, 0x214, 0x21b], 1, "mwe_16 portuguese cup turnover") // mwe16: the Portuguese Cup next year
}

/// Comparer for `fin_cup_start_new_season`: only the original runs (it frees and allocates); the year must go up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_fin_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("fin_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `nor_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("nor_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `por_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_por_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("por_cup_start_new_season", Cup(this), || o(this))
}
