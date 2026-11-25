// ---------- rx10_13: the Club World Championship predicted positions (world_club_champ.cpp) ----------

/// The predicted positions of the twelve Club World Championship entrants, by reputation.
pub(crate) const WORLD_CLUB_CHAMP_POSITIONS: [u32; 12] = [1, 2, 3, 4, 5, 5, 7, 7, 7, 10, 10, 10];

/// Rule: the Club World Championship's predicted positions: the twelve entrants by reputation, the best first, in
/// places 1, 2, 3, 4, 5, 5, 7, 7, 7, 10, 10, 10; then the entrant order is shuffled.
///
/// The Oceania Club Championship's mould (`oceania_club_champ_set_predicted_positions`): no statistics: the
/// world_club_champ.cpp box at line 0x429; the list (+0xee) sorted by reputation (`predicted_by_reputation_sort`, the
/// count the dword +0xf2), the places written (`predicted_by_reputation_write`) and the list shuffled
/// (`exe::comp_util_shuffle_pointer_array`(list, the count as a word)).
/// Original: `WorldClubChamp::set_predicted_positions` (0x92aba0, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(0xad5764, 0x429);
    }
    covered!("rx10_13 club world championship predicted");
    predicted_by_reputation_sort(read_usize(this + 0xee), read_u32(this + 0xf2)); // rx10_13: the twelve by reputation
    predicted_by_reputation_write(this, read_usize(this + 0xee), &WORLD_CLUB_CHAMP_POSITIONS);
    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), read_i16(this + 0xf2) as u16 as u32);
}

/// Comparer for `world_club_champ_set_predicted_positions`: only the original (it shuffles), with the plan
/// (`predicted_by_reputation_check`) and the same entrants after the shuffle.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let list = read_usize(this + 0xee);
    let n = read_i16(this + 0xf2);
    if list == 0 || !(0..=64).contains(&n) {
        return o(this);
    }
    let mut before: Vec<usize> = (0..n as usize).map(|k| read_usize(list + k * 4)).collect();
    o(this);
    let mut after: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this + 0xee) + k * 4)).collect();
    predicted_by_reputation_check("world_club_champ_set_predicted_positions", this, &before, &WORLD_CLUB_CHAMP_POSITIONS);
    before.sort_unstable();
    after.sort_unstable();
    SCOREBOARD.record(before == after, || format!("world_club_champ_set_predicted_positions({:#x}): the shuffled list is not the same", this));
}
