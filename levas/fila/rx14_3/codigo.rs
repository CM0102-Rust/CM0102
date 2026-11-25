// ---------- rx14_3: cup and competition scalar deleting destructors ----------

/// The scalar deleting destructor of the German League Cup: `GerLgeCup::~GerLgeCup` (0x5dc2d0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `GerLgeCup::scalar_deleting_destructor` (0x5dc2b0, ger_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ger_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5dc2d0)
}

/// The scalar deleting destructor of the Greek Cup: `GreCup::~GreCup` (0x5e8b10),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `GreCup::scalar_deleting_destructor` (0x5e8af0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_gre_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5e8b10)
}

/// The scalar deleting destructor of the Greek Super Cup: `GreSuper::~GreSuper` (0x5ec220),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `GreSuper::scalar_deleting_destructor` (0x5ec200, gre_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_gre_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5ec220)
}

/// The scalar deleting destructor of the Dutch Cup: `HolCup::~HolCup` (0x5f3600),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `HolCup::scalar_deleting_destructor` (0x5f35e0, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_hol_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5f3600)
}

/// The scalar deleting destructor of the Dutch Super Cup: `HolSuper::~HolSuper` (0x5f8a00),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `HolSuper::scalar_deleting_destructor` (0x5f89e0, hol_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_hol_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5f8a00)
}

/// The scalar deleting destructor of the Irish Challenge Cup: `IreChalCup::~IreChalCup` (0x634290),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IreChalCup::scalar_deleting_destructor` (0x634270, ire_chal_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_chal_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x634290)
}

/// The scalar deleting destructor of the Leinster Cup: `IreLeinsterCup::~IreLeinsterCup` (0x636690),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IreLeinsterCup::scalar_deleting_destructor` (0x636670, ire_leinster_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_leinster_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x636690)
}

/// The scalar deleting destructor of the Irish League Cup: `IreLgeCup::~IreLgeCup` (0x637040),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IreLgeCup::scalar_deleting_destructor` (0x637020, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x637040)
}

/// The scalar deleting destructor of the Munster Cup: `IreMunsterCup::~IreMunsterCup` (0x638990),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IreMunsterCup::scalar_deleting_destructor` (0x638970, ire_munster_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_munster_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x638990)
}

/// The scalar deleting destructor of the Irish President's Cup: `IrePresCup::~IrePresCup` (0x639460),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IrePresCup::scalar_deleting_destructor` (0x639440, ire_pres_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_pres_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x639460)
}

/// The scalar deleting destructor of the Irish Super Cup: `IreSuperCup::~IreSuperCup` (0x63aea0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `IreSuperCup::scalar_deleting_destructor` (0x63ae80, ire_super_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_super_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x63aea0)
}

/// The scalar deleting destructor of the Serie C1 Super Cup: `ItaC1Super::~ItaC1Super` (0x63c7c0),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Original: `ItaC1Super::scalar_deleting_destructor` (0x63c7a0, ita_c1_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_c1_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x63c7c0)
}

/// Comparers of the scalar deleting destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_ger_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_gre_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_gre_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_hol_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_hol_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_chal_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_leinster_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_munster_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_pres_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_super_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_c1_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
