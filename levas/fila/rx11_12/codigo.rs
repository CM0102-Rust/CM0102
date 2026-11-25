// ---------- rx11_12: the reserve competition's constructor (record_utils.cpp) ----------

/// The reserve competition's sub-competitions: the B international (+0xa3) and the Under 21 international (+0xa7)
/// objects (null until built; the destructor deletes them).
pub(crate) const RESERVE_COMP_B_INT: usize = 0xa3;
pub(crate) const RESERVE_COMP_UNDER21_INT: usize = 0xa7;

/// rule: the reserve competition is a competition of its own, the reserve record of the competitions table, with no B
/// or Under 21 sub-competition yet; building it asks for the game CD first
///
/// Comp::Comp (`exe::new_comp`), the ReserveComp vtable 0x96ef54, `RESERVE_COMP_UNDER21_INT` and `RESERVE_COMP_B_INT`
/// null; the CD check (`find_game_cd`, the no-CD patch read at 0x7e603c and 0x7e607f); `league::COMPETITION` = the
/// reserve record ([`KEY_CLUB_COMP_RESERVE`] in [`G_CLUB_COMP_TABLE`], 0x6b bytes each). Returns this. The SEH frame
/// covers only the base's destructor.
/// Original: `ReserveComp::ReserveComp` (0x7e5fc0, record_utils.cpp, thiscall, ret 4; the argument is not read).
pub(crate) unsafe extern "thiscall" fn construct_reserve_comp(this: usize, _unused: u32) -> usize {
    exe::new_comp(this);
    write_usize(this, 0x96ef54); // the ReserveComp vtable
    write_usize(this + RESERVE_COMP_UNDER21_INT, 0);
    write_usize(this + RESERVE_COMP_B_INT, 0);
    find_game_cd(0x7e603c, 0x7e607f);
    covered!("rx11_12 reserve competition built");
    write_usize(this + league::COMPETITION, key_comp_record(KEY_CLUB_COMP_RESERVE)); // rx11_12: the reserve record
    this
}

/// Comparer for `construct_reserve_comp`: it builds the object (and may wait for the CD), so only the original runs;
/// then the plan: the vtable, the reserve record and no sub-competition.
pub(crate) unsafe extern "thiscall" fn compare_construct_reserve_comp(this: usize, unused: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, unused);
    let record = key_comp_record(KEY_CLUB_COMP_RESERVE);
    SCOREBOARD.record(r == this && read_usize(this) == 0x96ef54 && read_usize(this + league::COMPETITION) == record
        && read_usize(this + RESERVE_COMP_B_INT) == 0 && read_usize(this + RESERVE_COMP_UNDER21_INT) == 0,
        || format!("construct_reserve_comp: vtable {:#x}, competition {:#x} (plan {:#x}), subs {:#x}/{:#x}", read_usize(this),
            read_usize(this + league::COMPETITION), record, read_usize(this + RESERVE_COMP_B_INT), read_usize(this + RESERVE_COMP_UNDER21_INT)));
    r
}
