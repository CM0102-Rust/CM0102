// ---------- rx13_14: the Oceania Club Championship's scalar deleting destructor (oceania_club_champ.cpp) ----------

/// The scalar deleting destructor of the Oceania Club Championship: `OceaniaClubChamp::~OceaniaClubChamp` (0x797170)
/// and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `OceaniaClubChamp::scalar_deleting_destructor` (0x797150, oceania_club_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_oceania_club_champ(this: usize, flags: u32) -> usize {
    covered!("rx13_14 oceania club championship deleted");
    delete_through(this, flags, 0x797170)
}

/// Only the original (it frees memory).
pub(crate) unsafe extern "thiscall" fn compare_delete_oceania_club_champ(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}
