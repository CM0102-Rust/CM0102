// ---------- rx15_1: nine competition scalar deleting destructors (Korea, mini cup and league, Northern Ireland, Norway, UEFA, Club World Cup) ----------

/// The scalar deleting destructor of the Korean Super Cup: `KorSuperCup::~KorSuperCup` (0x66da00) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `KorSuperCup::scalar_deleting_destructor` (0x66d9e0, kor_super_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_kor_super_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x66da00)
}

/// The scalar deleting destructor of the pre-season mini cup: `MiniCup::~MiniCup` (0x765ea0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `MiniCup::scalar_deleting_destructor` (0x765e10, mini_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_mini_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x765ea0)
}

/// The scalar deleting destructor of the pre-season mini league: `MiniLeague::~MiniLeague` (0x7668f0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `MiniLeague::scalar_deleting_destructor` (0x7667e0, mini_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_mini_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7668f0)
}

/// The scalar deleting destructor of the Northern Irish Charity Shield: `NirCharity::~NirCharity` (0x789fd0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `NirCharity::scalar_deleting_destructor` (0x789fb0, nir_charity.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nir_charity(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x789fd0)
}

/// The scalar deleting destructor of the Northern Irish Cup: `NirCup::~NirCup` (0x78a7a0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `NirCup::scalar_deleting_destructor` (0x78a780, nir_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nir_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78a7a0)
}

/// The scalar deleting destructor of the Northern Irish League Cup: `NirLgeCup::~NirLgeCup` (0x78c9f0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `NirLgeCup::scalar_deleting_destructor` (0x78c9d0, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nir_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78c9f0)
}

/// The scalar deleting destructor of the Norwegian Cup: `NorCup::~NorCup` (0x78f210) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `NorCup::scalar_deleting_destructor` (0x78f1f0, nor_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nor_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78f210)
}

/// The scalar deleting destructor of the UEFA Cup: `UefaCup::~UefaCup` (0x9024f0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `UefaCup::scalar_deleting_destructor` (0x9024d0, uefa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_uefa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x9024f0)
}

/// The scalar deleting destructor of the FIFA Club World Cup: `WorldClubCup::~WorldClubCup` (0x92b6a0) and, with bit 0 of `flags`, the object
/// (`delete_through`). Returns the object.
/// Original: `WorldClubCup::scalar_deleting_destructor` (0x92b680, world_club_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_world_club_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x92b6a0)
}

/// The comparers of the scalar deleting destructors: they free memory (cannot run twice); only the original runs.

pub(crate) unsafe extern "thiscall" fn compare_delete_kor_super_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_mini_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_mini_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nir_charity(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nir_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nir_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nor_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_uefa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_world_club_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}
