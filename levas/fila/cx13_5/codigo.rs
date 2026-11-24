// ---------- cx13_5: the Polish First Division's predicted positions, final stage order and promotion chain (pol_first.cpp) ----------

/// The predicted positions of the Polish First Division: without statistics, the pol_first.cpp box of line 0x346 and
/// nothing else; otherwise the list of its 16 clubs (+0xee) is sorted by strength (qsort `exe::crt_qsort` with 0x4b07d0,
/// cdecl) and each club's place in it (0 to 15) goes to its statistics entry (Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`).
/// Original: `PolFirst::set_predicted_positions` (0x7ca1e0, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        show_error_at(0xa78598, 0x346); // pol_first.cpp
        return;
    }
    covered!("cx13_5 polish predicted positions");
    exe::crt_qsort(read_usize(this + 0xee), 0x10, 4, 0x4b07d0); // the 16 clubs by strength
    for k in 0..0x10u32 {
        let club = read_usize(read_usize(this + 0xee) + 4 * k as usize);
        exe::comp_stats_set_club_final_position(l.comp_stats(), club, k); // cx13_5: the predicted place
    }
}

/// Comparer for `pol_first_set_predicted_positions`: side by side over the list of the 16 clubs and each one's
/// predicted place in its statistics entry (only the original without statistics, a list or an entry).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (stats, list) = (League(this).comp_stats(), read_usize(this + 0xee));
    if stats == 0 || list == 0 {
        return o(this);
    }
    let mut regions = vec![(list, 0x40)];
    for k in 0..0x10 {
        let e = exe::comp_stats_find_club_entry(stats, read_usize(list + 4 * k));
        if e == 0 {
            return o(this);
        }
        regions.push((e + comp_stats_club::FINAL_POSITION, 1));
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this), || pol_first_set_predicted_positions(this));
    SCOREBOARD.record(same, || format!("pol_first_set_predicted_positions({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// The order of the Polish final stage table: a row whose club is in the championship group comes before one in the
/// relegation group. The final stage marks each club's group in its reserve division (`club::RESERVE_DIVISION`): the
/// championship group with the club competition of [`KEY_CLUB_COMP_IRISH_GROUP_A`] (the key's name comes from its other
/// user; here it only marks the group), the relegation group with the next key. A row ranks 1 with that mark and 2
/// without, and the order is the difference.
/// Original: `pol_first_compare_championship_group_first` (0x7ca450, pol_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn pol_first_compare_championship_group_first(a: *const usize, b: *const usize) -> i32 {
    covered!("cx13_5 polish final order");
    let mark = read_usize(KEY_CLUB_COMP_IRISH_GROUP_A);
    let rank = |row: *const usize| if read_usize(Club(read_usize(row as usize)).reserve_division().addr()) == mark { 1 } else { 2 };
    rank(a) - rank(b) // cx13_5: the championship group first
}

compare_order!(compare_pol_first_compare_championship_group_first, pol_first_compare_championship_group_first, %%I1%%);

/// The Polish promotion and relegation chain: the final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`) of
/// the final stage (`league::STAGES`[1], where the championship and relegation groups play as one table) and of the
/// Second Division object ([`KEY_CLUB_COMP_POLISH_SECOND_DIVISION`], not checked for null), then
/// League::promote_and_relegate (`exe::league_promote_and_relegate`(this, final stage, second, 1, 1, −1, −1)), whose
/// result it returns. The argument is not read.
/// Original: `PolFirst::process_promotion_relegation_chain` (0x7ca490, pol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn pol_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    covered!("cx13_5 polish chain");
    bra_nat_first_call_virtual_no_args(read_usize(League(this).stages() + 4), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    let final_stage = read_usize(League(this).stages() + 4);
    exe::league_promote_and_relegate(this, final_stage, comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), 1, 1, -1, -1) // cx13_5: the final stage and the Second
}

/// Comparer for `pol_first_process_promotion_relegation_chain`: only the original (it moves clubs between divisions).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, pairs)
}
