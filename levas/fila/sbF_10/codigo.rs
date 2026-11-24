// ---------- sbF_10: the FA Trophy constructor and turnover (eng_fa_trophy.cpp) ----------

/// The FA Trophy constructor plan: vtable 0x969bc8, 3 substitutes named and 3 allowed, the English discipline rules,
/// EngFaTrophy::select_teams (0x5712b0; failing on 0), a 300,000 prize fund, the final ranking and the boxes at lines
/// 0x4b, 0x59 and 0x60 of eng_fa_trophy.cpp.
pub(crate) const FA_TROPHY_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x969bc8, bench: Some((3, 3)), discipline: discipline_rule::ENGLAND,
    entrants: 0x5712b0, fails_when_negative: false, prize: Some(0x493e0), rank: true, file: 0x9c4338, rows: [0x4b, 0x59, 0x60] }; // sbF10: the FA Trophy

/// Rule: the FA Trophy is a cup of one nation's non-league clubs of the game's year, with a prize fund of 300,000, 3
/// substitutes on the bench and 3 substitutions, under the English discipline rules; with nothing saved, its 32
/// entrants, the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `FA_TROPHY_CONSTRUCTOR` (checked instruction by instruction against NirCup::NirCup: only
/// the vtable, the discipline rule, the entrant routine, the prize fund, the file and the box lines change).
/// Original: `EngFaTrophy::EngFaTrophy` (0x570c00, eng_fa_trophy.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_fa_trophy(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &FA_TROPHY_CONSTRUCTOR, "sbF_10 fa trophy new")
}

pub(crate) unsafe extern "thiscall" fn compare_construct_eng_fa_trophy(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_cup_constructor("construct_eng_fa_trophy", this, year, record, &FA_TROPHY_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: at the season turn the FA Trophy is rebuilt: last edition's tables are freed, the statistics archived, the
/// year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against NirCup's: only the
/// entrant routine, EngFaTrophy::select_teams 0x5712b0, the file and the box lines 0x1b3, 0x1ba and 0x1c1 change).
/// Original: `EngFaTrophy::start_new_season` (0x5714a0, eng_fa_trophy.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_trophy_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x5712b0, 0x9c4338, [0x1b3, 0x1ba, 0x1c1], 1, "sbF_10 fa trophy turnover")
}

pub(crate) unsafe extern "thiscall" fn compare_eng_fa_trophy_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("eng_fa_trophy_start_new_season", Cup(this), || o(this))
}
