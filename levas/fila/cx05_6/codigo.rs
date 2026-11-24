// ---------- cx05_6: the African Nations Cup progress codes (african_nations.cpp) ----------

/// Stores a nation's progress in the African Nations Cup statistics (`group_championship_progress` of cx04_5): in the
/// group stages (below 3) current 1→5, 2→9, 3→9, 4→13; best 1, 2→1, 3→9, 4→13; worst 1→5, 2, 3→9, 4→13; afterwards
/// as they come.
/// Original: `AfricanNations::set_club_progress` (0x4029c0, african_nations.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn african_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx05_6 african nations progress");
    group_championship_progress(this, club, stage, current, best, worst, 3, [[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx05_6: the African Nations Cup codes
}

/// Comparer for `african_nations_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_african_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_group_championship_progress("african_nations_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || african_nations_set_club_progress(this, club, stage, current, best, worst));
}
