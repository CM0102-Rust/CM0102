// ---------- rx14_2: cup and competition scalar deleting destructors ----------

/// The scalar deleting destructor of the Croatian Cup: `CroCup::~CroCup` (0x517670),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `CroCup::scalar_deleting_destructor` (0x517650, cro_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cro_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x517670)
}

/// The scalar deleting destructor of the Danish Cup: `DenCup::~DenCup` (0x550ba0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `DenCup::scalar_deleting_destructor` (0x550b80, den_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_den_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x550ba0)
}

/// The scalar deleting destructor of the Football League Trophy: `EngAutoCup::~EngAutoCup` (0x56acc0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EngAutoCup::scalar_deleting_destructor` (0x56aca0, eng_auto_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_auto_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x56acc0)
}

/// The scalar deleting destructor of the English League Cup: `EngCcCup::~EngCcCup` (0x56c4a0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EngCcCup::scalar_deleting_destructor` (0x56c480, eng_cc_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_cc_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x56c4a0)
}

/// The scalar deleting destructor of the Charity Shield: `EngCharity::~EngCharity` (0x56d570),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EngCharity::scalar_deleting_destructor` (0x56d550, eng_charity.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_charity(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x56d570)
}

/// The scalar deleting destructor of the FA Cup: `EngFaCup::~EngFaCup` (0x56f250),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EngFaCup::scalar_deleting_destructor` (0x56f230, eng_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_fa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x56f250)
}

/// The scalar deleting destructor of the FA Trophy: `EngFaTrophy::~EngFaTrophy` (0x570df0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `EngFaTrophy::scalar_deleting_destructor` (0x570dd0, eng_fa_trophy.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_fa_trophy(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x570df0)
}

/// The scalar deleting destructor of the Finnish Cup: `FinCup::~FinCup` (0x592440),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `FinCup::scalar_deleting_destructor` (0x592420, fin_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fin_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x592440)
}

/// The scalar deleting destructor of the French Cup: `FraCup::~FraCup` (0x5ba320),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `FraCup::scalar_deleting_destructor` (0x5ba300, fra_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5ba320)
}

/// The scalar deleting destructor of the French League Cup: `FraLgeCup::~FraLgeCup` (0x5bcb70),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `FraLgeCup::scalar_deleting_destructor` (0x5bcb50, fra_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bcb70)
}

/// The scalar deleting destructor of the French Super Cup: `FraSuper::~FraSuper` (0x5bf500),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `FraSuper::scalar_deleting_destructor` (0x5bf4e0, fra_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bf500)
}

/// The scalar deleting destructor of the German Cup: `GerCup::~GerCup` (0x5d8e10),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `GerCup::scalar_deleting_destructor` (0x5d8df0, ger_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ger_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5d8e10)
}

/// Comparers of the scalar deleting destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_cro_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_den_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_auto_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_cc_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_charity(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_fa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_fa_trophy(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fin_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ger_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
