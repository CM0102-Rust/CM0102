// ---------- rx14_1: cup and competition scalar deleting destructors ----------

/// The scalar deleting destructor of the African Nations Cup: `AfricanNations::~AfricanNations` (0x401220),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `AfricanNations::scalar_deleting_destructor` (0x401200, african_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_african_nations(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x401220)
}

/// The scalar deleting destructor of the Asian Cup: `AsiaNations::~AsiaNations` (0x40ea40),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `AsiaNations::scalar_deleting_destructor` (0x40ea20, asia_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_asia_nations(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x40ea40)
}

/// The scalar deleting destructor of the European Championship: `EuroChamp::~EuroChamp` (0x57aad0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EuroChamp::scalar_deleting_destructor` (0x57aab0, euro_champ.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_euro_champ(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x57aad0)
}

/// The scalar deleting destructor of the European Cup: `EuropeanCup::~EuropeanCup` (0x583a00),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EuropeanCup::scalar_deleting_destructor` (0x5839e0, european_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_european_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x583a00)
}

/// The scalar deleting destructor of the Inter-American Cup: `InterAmerCup::~InterAmerCup` (0x632270),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `InterAmerCup::scalar_deleting_destructor` (0x632250, inter_amer_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_inter_amer_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x632270)
}

/// The scalar deleting destructor of the Intertoto Cup: `IntertotoCup::~IntertotoCup` (0x6331c0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IntertotoCup::scalar_deleting_destructor` (0x6331a0, intertoto_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_intertoto_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x6331c0)
}

/// The scalar deleting destructor of the friendlies competition: `Friendly::~Friendly` (0x5c22d0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `Friendly::scalar_deleting_destructor` (0x5c22a0, friendly.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_friendly(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5c22d0)
}

/// The scalar deleting destructor of the cup base: `Cup::~Cup` (0x518690),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Only a plain Cup would reach it: every cup class overrides vtable slot 0 with its own, so in practice
/// it does not run.
/// Original: `Cup::scalar_deleting_destructor` (0x518670, Cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x518690)
}

/// The scalar deleting destructor of the Belgian Cup: `BelFaCup::~BelFaCup` (0x41d240),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `BelFaCup::scalar_deleting_destructor` (0x41d220, bel_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bel_fa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x41d240)
}

/// The scalar deleting destructor of the Belgian Super Cup: `BelSuper::~BelSuper` (0x4210e0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `BelSuper::scalar_deleting_destructor` (0x4210c0, bel_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bel_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4210e0)
}

/// The scalar deleting destructor of the Brazilian Champions Cup: `BraChampCup::~BraChampCup` (0x425b50),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `BraChampCup::scalar_deleting_destructor` (0x425b30, bra_champ_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_champ_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x425b50)
}

/// The scalar deleting destructor of the Brazilian Cup: `BraCup::~BraCup` (0x427630),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `BraCup::scalar_deleting_destructor` (0x427610, bra_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x427630)
}

/// Comparers of the scalar deleting destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_african_nations(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_asia_nations(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_euro_champ(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_european_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_inter_amer_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_intertoto_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_friendly(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bel_fa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bel_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_champ_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
