// ---------- cx05_17: the mini league rules and the friendly tournament slot constructor (mini_league.cpp, friendly.cpp) ----------

/// Sets the mini league's rules again (`mini_league_rules` of cx05_13: 9 substitutes named and 9 allowed, one leg, no
/// round draw rules, the promotion bytes cleared and the round table of stage −1; the old round table is not freed).
/// Returns 1.
/// Original: `MiniLeague::set_up_rules` (0x766db0, mini_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn mini_league_set_up_rules(this: usize) -> u32 {
    covered!("cx05_17 mini league rules");
    mini_league_rules(this);
    1
}

/// Comparer for `mini_league_set_up_rules`: only the original (the round table allocates), then the plan: 1, one
/// leg, 9 and 9 and a round table.
pub(crate) unsafe extern "thiscall" fn compare_mini_league_set_up_rules(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let ok = r == 1 && read_u16(this + league::LEGS) == 1 && read_u8(this + league::SUBSTITUTES_NAMED) == 9 && read_usize(this + league::ROUND_DATES) != 0;
    SCOREBOARD.record(ok, || format!("mini_league_set_up_rules({:#x}) returned {}", this, r));
    r
}

/// Constructs one friendly tournament slot: the start date (`tournament_slot::START_DATE`) constructed
/// (`exe::date_date_54a110`). Returns the slot. Called by the C runtime's vector constructor over the 127 slots of the
/// friendlies.
/// Original: `friendly_tournament_slot_construct` (0x5c22c0, friendly.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn friendly_tournament_slot_construct(this: usize) -> usize {
    exe::date_date_54a110(this + tournament_slot::START_DATE); // cx05_17: the slot's start date
    this
}

/// Comparer for `friendly_tournament_slot_construct`: side by side on the slot's date (the Date constructor only writes
/// it).
pub(crate) unsafe extern "thiscall" fn compare_friendly_tournament_slot_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, c, same, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x10)], || o(this), || friendly_tournament_slot_construct(this));
    SCOREBOARD.record(same && r == c, || format!("friendly_tournament_slot_construct({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    r
}
