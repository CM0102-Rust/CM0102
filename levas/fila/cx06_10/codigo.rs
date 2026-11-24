// ---------- cx06_10: the European Championship qualifiers' results, positions and nation progress (euro_champ_qual.cpp) ----------

/// Adds a result to the qualifiers: a group stage result (the fixture's `fixture::STAGE_INDEX`, signed, up to the last
/// group stage +0xff) is first kept as a copy in the stored group results (+0x10, `exe::llist_add_tail` of
/// `fixture::SIZE` bytes), for the runners-up table; then League::add_result (`exe::league_add_result`), whose result
/// it returns.
/// Original: `EuroChampQual::add_result` (0x57ffd0, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_add_result(this: usize, fixture: usize, update_stats: u32) -> u32 {
    if read_u8(fixture + fixture::STAGE_INDEX) as i8 <= read_u8(this + 0xff) as i8 { // the last group stage
        covered!("cx06_10 group result kept");
        exe::llist_add_tail(read_usize(this + 0x10), fixture, fixture::SIZE as u32); // the stored group results
    }
    exe::league_add_result(this, fixture, update_stats)
}

/// Updates the qualifiers' positions and statuses (League::update_positions_and_status 0x684cd0) and, while the
/// qualified flag (+0xfb) is clear and the competition is complete (`comp_vtable::CHECK_COMPLETE` with 1, 0), marks the
/// qualified nations (EuroChampQual::mark_qualified_teams 0x580040).
/// Original: `EuroChampQual::update_positions_and_status` (0x580010, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_positions_and_status(this: usize) {
    covered!("cx06_10 qualifiers positions");
    let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684cd0usize); // League::update_positions_and_status
    update(this);
    if read_u32(this + 0xfb) != 0 { // already marked
        return;
    }
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CHECK_COMPLETE));
    if complete(this, 1, 0) != 0 {
        covered!("cx06_10 qualifiers complete");
        let mark_qualified_teams: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x580040usize); // EuroChampQual::mark_qualified_teams
        mark_qualified_teams(this);
    }
}

/// The progress codes of the qualifiers' groups: best 1, 10, 19, 28, 37, 46 for the codes 1 to 6; current and worst 1,
/// 16, 19, 28, 37, 46.
const EURO_QUAL_GROUP_BEST: [u8; 6] = [1, 0xa, 0x13, 0x1c, 0x25, 0x2e];
const EURO_QUAL_GROUP_CURRENT: [u8; 6] = [1, 0x10, 0x13, 0x1c, 0x25, 0x2e];
/// The progress codes of the play-off: 13 and 16 for the codes 1 and 2.
const EURO_QUAL_PLAYOFF_CODES: [u8; 2] = [0xd, 0x10];

/// Stores a nation's progress in the qualifiers in its statistics entry (`exe::comp_stats_find_club_entry`, not checked
/// for null): in a group stage (`stage`, signed, up to +0xff) the codes 1 to 6 of best, current and worst become
/// `EURO_QUAL_GROUP_BEST` and `EURO_QUAL_GROUP_CURRENT` (current and worst); in the play-off stage (+0x101) the codes 1
/// and 2 become 13 and 16; other codes and stages stay. The bytes go to `comp_stats_club::BEST_POSITION`,
/// `CURRENT_POSITION` and `WORST_POSITION`, in that order.
/// Original: `EuroChampQual::set_club_progress` (0x580980, euro_champ_qual.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let stage = stage as u8 as i8;
    let code = |c: u32, table: &[u8]| {
        let c = c as u8;
        if (1..=table.len()).contains(&(c as usize)) { table[c as usize - 1] } else { c }
    };
    let (best, current, worst) = if stage <= read_u8(this + 0xff) as i8 {
        covered!("cx06_10 qualifiers group progress");
        (code(best, &EURO_QUAL_GROUP_BEST), code(current, &EURO_QUAL_GROUP_CURRENT), code(worst, &EURO_QUAL_GROUP_CURRENT))
    } else if stage == read_u8(this + 0x101) as i8 {
        covered!("cx06_10 qualifiers play-off progress");
        (code(best, &EURO_QUAL_PLAYOFF_CODES), code(current, &EURO_QUAL_PLAYOFF_CODES), code(worst, &EURO_QUAL_PLAYOFF_CODES))
    } else {
        (best as u8, current as u8, worst as u8)
    };
    write_u8(e + comp_stats_club::BEST_POSITION, best);
    write_u8(e + comp_stats_club::CURRENT_POSITION, current);
    write_u8(e + comp_stats_club::WORST_POSITION, worst);
}

/// Comparer for `euro_champ_qual_add_result`: the league result cannot be applied twice, so only the original runs; the
/// plan: a group stage result adds one item to the stored results.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_add_result(this: usize, fixture: usize, update_stats: u32) -> u32 {
    let results = read_usize(this + 0x10);
    let kept = results != 0 && read_u8(fixture + fixture::STAGE_INDEX) as i8 <= read_u8(this + 0xff) as i8;
    let before = if results != 0 { read_u16(results + llist::COUNT) } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, fixture, update_stats);
    if results != 0 {
        let added = read_u16(results + llist::COUNT).wrapping_sub(before);
        SCOREBOARD.record(added == kept as u16, || format!("euro_champ_qual_add_result({:#x}, stage {}): {} results kept", this, read_u8(fixture + fixture::STAGE_INDEX) as i8, added));
    }
    r
}

/// Comparer for `euro_champ_qual_update_positions_and_status`: the statuses are announced (they cannot be set twice),
/// so only the original runs; the plan: the qualified flag (+0xfb), once set, stays set.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_positions_and_status(this: usize) {
    let marked = read_u32(this + 0xfb) != 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    SCOREBOARD.record(!marked || read_u32(this + 0xfb) != 0, || format!("euro_champ_qual_update_positions_and_status({:#x}): the qualified flag was cleared", this));
}

/// Comparer for `euro_champ_qual_set_club_progress`: side by side on the 3 bytes of the entry; only the original without
/// an entry (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("euro_champ_qual_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || euro_champ_qual_set_club_progress(this, club, stage, current, best, worst));
}
