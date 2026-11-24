// ---------- sx04c_23: the nations' team of the week choice (the *_awards.cpp files of the sx04c nations) ----------

/// The comparer of a nation's `update_team_of_the_week`: the original runs, and it must have built the team of the
/// week (`TEAM_OF_THE_WEEK_BUILDS` moved, counted by the comparer of the hooked builder) exactly when the copy would
/// (`award` is one of the table's `slots`).
pub(crate) unsafe fn compare_team_of_the_week_of(this: usize, award: usize, slots: &[usize], original: impl FnOnce(), what: &str) {
    let planned = slots.iter().any(|&k| nation_award(this, k) == award);
    let before = TEAM_OF_THE_WEEK_BUILDS.load(core::sync::atomic::Ordering::Relaxed);
    original();
    let built = TEAM_OF_THE_WEEK_BUILDS.load(core::sync::atomic::Ordering::Relaxed) != before;
    SCOREBOARD.record(built == planned, || format!("{}(award {:#x}): original built {}, rust planned {}", what, award, built, planned));
}

/// The Italian team awards: Serie A, Serie B, C1 A and C1 B (awards 0, 10, 15, 20) and, with the lower divisions
/// (`awards_lower_divisions`), the three Serie C2 groups (25, 30, 35).
unsafe fn italy_team_of_the_week_slots(this: usize) -> &'static [usize] {
    const SLOTS: [usize; 7] = [0, 10, 15, 20, 25, 30, 35];
    if awards_lower_divisions(this) { &SLOTS } else { &SLOTS[..4] }
}

/// Builds the team of the week of an Italian team award: only for the divisions' teams of the week
/// (`italy_team_of_the_week_slots`), `build_team_of_the_week` with the award's competition.
/// Original: `ItalyAwards::update_team_of_the_week` (0x661330, italy_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn italy_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, italy_team_of_the_week_slots(this));
}

/// The team awards of Japan: the J1 and J2 League teams of the week (awards 4 and 6).
const JAPAN_TEAMS_OF_THE_WEEK: [usize; 2] = [4, 6];

/// Builds the team of the week of a team award of Japan: only for the J1 and J2 League teams of the week (awards 4 and 6) (`JAPAN_TEAMS_OF_THE_WEEK`).
/// Original: `JapanAwards::update_team_of_the_week` (0x666bb0, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &JAPAN_TEAMS_OF_THE_WEEK);
}

/// The team awards of Northern Ireland: the Premier and First Division teams of the week (awards 1 and 7).
const NORTHERN_IRELAND_TEAMS_OF_THE_WEEK: [usize; 2] = [1, 7];

/// Builds the team of the week of a team award of Northern Ireland: only for the Premier and First Division teams of the week (awards 1 and 7) (`NORTHERN_IRELAND_TEAMS_OF_THE_WEEK`).
/// Original: `NorthernIrelandAwards::update_team_of_the_week` (0x7936b0, northern_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn northern_ireland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &NORTHERN_IRELAND_TEAMS_OF_THE_WEEK);
}

/// The team awards of Norway: the Premier and First Division teams of the week (awards 5 and 8).
const NORWAY_TEAMS_OF_THE_WEEK: [usize; 2] = [5, 8];

/// Builds the team of the week of a team award of Norway: only for the Premier and First Division teams of the week (awards 5 and 8) (`NORWAY_TEAMS_OF_THE_WEEK`).
/// Original: `NorwayAwards::update_team_of_the_week` (0x7944d0, norway_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn norway_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &NORWAY_TEAMS_OF_THE_WEEK);
}

/// The team awards of the old Finnish awards: the Premier and First Division teams of the week (awards 7 and 10).
const OLD_FINLAND_TEAMS_OF_THE_WEEK: [usize; 2] = [7, 10];

/// Builds the team of the week of a team award of the old Finnish awards: only for the Premier and First Division teams of the week (awards 7 and 10) (`OLD_FINLAND_TEAMS_OF_THE_WEEK`).
/// Original: `OldFinlandAwards::update_team_of_the_week` (0x79dcd0, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &OLD_FINLAND_TEAMS_OF_THE_WEEK);
}

/// The team awards of the old French awards: the First and Second Division teams of the week (awards 6 and 10).
const OLD_FRANCE_TEAMS_OF_THE_WEEK: [usize; 2] = [6, 10];

/// Builds the team of the week of a team award of the old French awards: only for the First and Second Division teams of the week (awards 6 and 10) (`OLD_FRANCE_TEAMS_OF_THE_WEEK`).
/// Original: `OldFranceAwards::update_team_of_the_week` (0x79e560, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &OLD_FRANCE_TEAMS_OF_THE_WEEK);
}

/// The team awards of the old Irish awards: the Premier and First Division teams of the week (awards 3 and 10).
const OLD_IRELAND_TEAMS_OF_THE_WEEK: [usize; 2] = [3, 10];

/// Builds the team of the week of a team award of the old Irish awards: only for the Premier and First Division teams of the week (awards 3 and 10) (`OLD_IRELAND_TEAMS_OF_THE_WEEK`).
/// Original: `OldIrelandAwards::update_team_of_the_week` (0x79f510, old_ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &OLD_IRELAND_TEAMS_OF_THE_WEEK);
}

/// The team awards of Poland: the First and Second Division teams of the week (awards 0 and 5).
const POLAND_TEAMS_OF_THE_WEEK: [usize; 2] = [0, 5];

/// Builds the team of the week of a team award of Poland: only for the First and Second Division teams of the week (awards 0 and 5) (`POLAND_TEAMS_OF_THE_WEEK`).
/// Original: `PolandAwards::update_team_of_the_week` (0x7cd810, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &POLAND_TEAMS_OF_THE_WEEK);
}

/// The team awards of Portugal: the Premier and Second Division teams of the week (awards 0 and 6).
const PORTUGAL_TEAMS_OF_THE_WEEK: [usize; 2] = [0, 6];

/// Builds the team of the week of a team award of Portugal: only for the Premier and Second Division teams of the week (awards 0 and 6) (`PORTUGAL_TEAMS_OF_THE_WEEK`).
/// Original: `PortugalAwards::update_team_of_the_week` (0x7d3d60, portugal_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn portugal_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &PORTUGAL_TEAMS_OF_THE_WEEK);
}

/// The team awards of Russia: the Premier and First Division teams of the week (awards 1 and 4).
const RUSSIA_TEAMS_OF_THE_WEEK: [usize; 2] = [1, 4];

/// Builds the team of the week of a team award of Russia: only for the Premier and First Division teams of the week (awards 1 and 4) (`RUSSIA_TEAMS_OF_THE_WEEK`).
/// Original: `RussiaAwards::update_team_of_the_week` (0x7ec6a0, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &RUSSIA_TEAMS_OF_THE_WEEK);
}

/// The team awards of Scotland: the four divisions' teams of the week (awards 3, 9, 15 and 21).
const SCOTLAND_TEAMS_OF_THE_WEEK: [usize; 4] = [3, 9, 15, 21];

/// Builds the team of the week of a team award of Scotland: only for the four divisions' teams of the week (awards 3, 9, 15 and 21) (`SCOTLAND_TEAMS_OF_THE_WEEK`).
/// Original: `ScotlandAwards::update_team_of_the_week` (0x7f6360, scotland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scotland_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &SCOTLAND_TEAMS_OF_THE_WEEK);
}

/// The team awards of Spain: the First and Second Division teams of the week (awards 2 and 5).
const SPAIN_TEAMS_OF_THE_WEEK: [usize; 2] = [2, 5];

/// Builds the team of the week of a team award of Spain: only for the First and Second Division teams of the week (awards 2 and 5) (`SPAIN_TEAMS_OF_THE_WEEK`).
/// Original: `SpainAwards::update_team_of_the_week` (0x8593e0, spain_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spain_awards_team_of_the_week(this: usize, award: usize) {
    nation_awards_team_of_the_week_of(this, award, &SPAIN_TEAMS_OF_THE_WEEK);
}

pub(crate) unsafe extern "thiscall" fn compare_italy_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    compare_team_of_the_week_of(this, award, italy_team_of_the_week_slots(this), || o(this, award), "italy_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_japan_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_team_of_the_week_of(this, award, &JAPAN_TEAMS_OF_THE_WEEK, || o(this, award), "japan_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_northern_ireland_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    compare_team_of_the_week_of(this, award, &NORTHERN_IRELAND_TEAMS_OF_THE_WEEK, || o(this, award), "northern_ireland_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_norway_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I3%%);
    compare_team_of_the_week_of(this, award, &NORWAY_TEAMS_OF_THE_WEEK, || o(this, award), "norway_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I4%%);
    compare_team_of_the_week_of(this, award, &OLD_FINLAND_TEAMS_OF_THE_WEEK, || o(this, award), "old_finland_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I5%%);
    compare_team_of_the_week_of(this, award, &OLD_FRANCE_TEAMS_OF_THE_WEEK, || o(this, award), "old_france_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I6%%);
    compare_team_of_the_week_of(this, award, &OLD_IRELAND_TEAMS_OF_THE_WEEK, || o(this, award), "old_ireland_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_poland_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I7%%);
    compare_team_of_the_week_of(this, award, &POLAND_TEAMS_OF_THE_WEEK, || o(this, award), "poland_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_portugal_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I8%%);
    compare_team_of_the_week_of(this, award, &PORTUGAL_TEAMS_OF_THE_WEEK, || o(this, award), "portugal_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_russia_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I9%%);
    compare_team_of_the_week_of(this, award, &RUSSIA_TEAMS_OF_THE_WEEK, || o(this, award), "russia_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_scotland_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I10%%);
    compare_team_of_the_week_of(this, award, &SCOTLAND_TEAMS_OF_THE_WEEK, || o(this, award), "scotland_awards_team_of_the_week");
}

pub(crate) unsafe extern "thiscall" fn compare_spain_awards_team_of_the_week(this: usize, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I11%%);
    compare_team_of_the_week_of(this, award, &SPAIN_TEAMS_OF_THE_WEEK, || o(this, award), "spain_awards_team_of_the_week");
}
