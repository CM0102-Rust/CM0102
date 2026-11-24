// ---------- cx03_5: five cup scalar deleting destructors ----------

/// The scalar deleting destructor of the Welsh League Cup: `WelLgeCup::~WelLgeCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `WelLgeCup::scalar_deleting_destructor` (0x9260e0, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wel_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x926100)
}

/// The scalar deleting destructor of the Welsh Premier Cup: `WelPrmCup::~WelPrmCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `WelPrmCup::scalar_deleting_destructor` (0x927a30, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_wel_prm_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x927a50)
}

/// The scalar deleting destructor of the Copa Merconorte: `ConMercCup::~ConMercCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ConMercCup::scalar_deleting_destructor` (0x4c1880, con_merc_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_con_merc_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4c18a0)
}

/// The scalar deleting destructor of the Copa Libertadores: `ConmebolLiber::~ConmebolLiber` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ConmebolLiber::scalar_deleting_destructor` (0x4c30e0, conmebol_liber.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_conmebol_liber(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4c3100)
}

/// The scalar deleting destructor of the Copa Mercosur: `ConmebolMerc::~ConmebolMerc` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ConmebolMerc::scalar_deleting_destructor` (0x4c4850, conmebol_merc.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_conmebol_merc(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4c4870)
}

/// The comparers of the destructors: they free memory (cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_wel_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_wel_prm_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_con_merc_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_conmebol_liber(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_conmebol_merc(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}
