// ---------- batch %%LEVA%%: emptying an award shortlist (award_shortlist.cpp) ----------

/// Empties a shortlist: no entries (`COUNT` 0) and the 16 managers' vote slots (`AWARD_SHORTLIST_VOTES`, words) −1.
/// The original leaves −1 in eax; its two callers (nation awards, 0x7683c0 and 0x768b20) do not read it.
/// Original: `AWARD_SHORTLIST::clear` (0x418320, award_shortlist.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn award_shortlist_clear(this: usize) {
    use award_shortlist_fields as asl;
    write_i16(this + asl::COUNT, 0);
    for k in 0..16 {
        write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, -1); // tel44_3: no manager has voted
    }
}

/// Comparer for `award_shortlist_clear`: side by side over the count and the vote slots (+4..+0x28).
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_clear(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AWARD_SHORTLIST_CLEAR);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this + award_shortlist_fields::COUNT, 0x24)], || o(this), || award_shortlist_clear(this));
    SCOREBOARD.record(equal, || format!("AWARD_SHORTLIST::clear({:#x}): differences in {:?}", this, first_differences(&mo, &mr)));
}
