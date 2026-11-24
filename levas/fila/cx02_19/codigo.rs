// ---------- cx02_19: the Greek Cup staff records, progress codes and bench (gre_cup.cpp) ----------

/// The Greek Cup staff records: `italian_cup_staff_records` with the knockout at stage 10 and the group qualification
/// round 0x14 (checked instruction by instruction against ItaCup::update_staff_records_for_status: only the stage and
/// the round change). Returns 0.
/// Original: `GreCup::update_staff_records_for_status` (0x5e9ca0, gre_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn gre_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    italian_cup_staff_records(this, club, outcome, stage, game, final_, 10, 0x14, "cx02_19 greek cup group qualified")
}

/// The Greek Cup: ten groups (stages 0 to 9); the group places 1 and 2 become 1 (best) or 17, 3 and 4 become 21 and 30.
pub(crate) const GREEK_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 10, current: &[17, 17, 21, 30], best: &[1, 1, 21, 30], worst: &[17, 17, 21, 30] };

/// The Greek Cup progress codes (`set_group_cup_club_progress` with `GREEK_CUP_PROGRESS`).
/// Original: `GreCup::set_club_progress` (0x5ea2d0, gre_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &GREEK_CUP_PROGRESS)
}

/// The Greek Cup bench for a fixture: the low byte of the fixture's `SQUAD_RULES` becomes 0x77 (7 substitutes named, 7
/// substitutions) in the rounds named 0xa (`fixture::ROUND_NAME`) and 0x37 (7 named, 3 substitutions) in the others;
/// the high byte kept.
/// Original: `GreCup::set_fixture_substitutes` (0x5ea3d0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_set_fixture_substitutes(_this: usize, fixture: usize) {
    let fixture = Fixture(fixture);
    let bench = if fixture.round_name() == 0xa { 0x77 } else { 0x37 };
    fixture.set_squad_rules((fixture.squad_rules() & 0xff00) | bench);
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("gre_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("gre_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || gre_cup_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer for `gre_cup_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word side by side.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_set_fixture_substitutes(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture + fixture::SQUAD_RULES, 2)], || o(this, fixture), || gre_cup_set_fixture_substitutes(this, fixture));
    SCOREBOARD.record(same, || format!("gre_cup_set_fixture_substitutes({:#x}): original {:x?} rust {:x?}", fixture, mo, mr));
}
