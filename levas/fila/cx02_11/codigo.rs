// ---------- cx02_11: the Irish and Northern Irish League Cups' progress codes (ire_lge_cup.cpp, nir_lge_cup.cpp) ----------

/// The group-stage progress tables of a cup built on a league with group stages: in the groups, a code k from 1 to
/// the table's length becomes the table's entry k − 1 (the group place on the cup-wide scale), any other code stays.
pub(crate) struct GroupCupProgress {
    /// The stages below this one are groups.
    pub(crate) knockout: i8,
    pub(crate) current: &'static [u8],
    pub(crate) best: &'static [u8],
    pub(crate) worst: &'static [u8],
}

/// Code `code` through `table` (1-based); outside it, unchanged.
fn group_progress_code(code: u8, table: &[u8]) -> u8 {
    if code >= 1 && code as usize <= table.len() { table[code as usize - 1] } else { code }
}

/// The body of the group cups' `set_club_progress` (identical apart from the stage and the tables): the club's entry
/// in the statistics (`Comp_Stats::find_club_entry`, not checked); in the groups (stage, a signed byte, below
/// `progress.knockout`) the three codes through their tables (`group_progress_code`), in the knockout as they come;
/// the best code to `comp_stats_club::BEST_POSITION`, the current to `CURRENT_POSITION`, the worst to
/// `WORST_POSITION`.
pub(crate) unsafe fn set_group_cup_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, progress: &GroupCupProgress) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < progress.knockout {
        covered!("cx02_11 group cup progress in the groups");
        best = group_progress_code(best, progress.best);
        worst = group_progress_code(worst, progress.worst);
        current = group_progress_code(current, progress.current);
    }
    entry.set_best_position(best);
    entry.set_current_position(current);
    entry.set_worst_position(worst);
}

/// The Irish League Cup: seven groups (stages 0 to 6); the group places 1 to 4 become 1, 9, 17 and 25 (the worst place
/// 1 becomes 5).
pub(crate) const IRISH_LEAGUE_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 7, current: &[1, 9, 17, 25], best: &[1, 9, 17, 25], worst: &[5, 9, 17, 25] };

/// The Northern Irish League Cup: three groups (stages 0 to 2); the group places 1 to 5 become 1, 5, 9, 13 and 17 (the
/// best place 2 and the worst place 1 become 1 and 5).
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_PROGRESS: GroupCupProgress = GroupCupProgress { knockout: 3, current: &[1, 5, 9, 13, 17], best: &[1, 1, 9, 13, 17], worst: &[5, 5, 9, 13, 17] };

/// The Irish League Cup progress codes (`set_group_cup_club_progress` with `IRISH_LEAGUE_CUP_PROGRESS`).
/// Original: `IreLgeCup::set_club_progress` (0x638690, ire_lge_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &IRISH_LEAGUE_CUP_PROGRESS)
}

/// The Northern Irish League Cup progress codes (`set_group_cup_club_progress` with
/// `NORTHERN_IRISH_LEAGUE_CUP_PROGRESS`).
/// Original: `NirLgeCup::set_club_progress` (0x78df40, nir_lge_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, &NORTHERN_IRISH_LEAGUE_CUP_PROGRESS)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("ire_lge_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || ire_lge_cup_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("nir_lge_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || nir_lge_cup_set_club_progress(this, club, stage, current, best, worst));
}
