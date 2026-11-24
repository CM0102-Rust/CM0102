// ---------- cx09_5: eight league scalar deleting destructors ----------

/// The scalar deleting destructor of the North regional championship: `BraRegNorth::~BraRegNorth` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegNorth::scalar_deleting_destructor` (0x435560, bra_reg_north.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_north(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x435580)
}

/// The scalar deleting destructor of the Northeast regional championship: `BraRegNortheast::~BraRegNortheast` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegNortheast::scalar_deleting_destructor` (0x436610, bra_reg_northeast.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_northeast(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x436630)
}

/// The scalar deleting destructor of the Parana state championship: `BraRegParana::~BraRegParana` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegParana::scalar_deleting_destructor` (0x437620, bra_reg_parana.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_parana(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x437640)
}

/// The scalar deleting destructor of the Pernambuco state championship: `BraRegPern::~BraRegPern` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegPern::scalar_deleting_destructor` (0x438b10, bra_reg_pern.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_pern(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x438b30)
}

/// The scalar deleting destructor of the Rio de Janeiro state championship: `BraRegRio::~BraRegRio` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegRio::scalar_deleting_destructor` (0x439c60, bra_reg_rio.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_rio(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x439c80)
}

/// The scalar deleting destructor of the Santa Catarina state championship: `BraRegSanta::~BraRegSanta` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegSanta::scalar_deleting_destructor` (0x43b350, bra_reg_santa.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_santa(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x43b370)
}

/// The scalar deleting destructor of the Sao Paulo state championship: `BraRegSp::~BraRegSp` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegSp::scalar_deleting_destructor` (0x43c4c0, bra_reg_sp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_sp(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x43c4e0)
}

/// The scalar deleting destructor of the Danish Second Division: `DenSecond::~DenSecond` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `DenSecond::scalar_deleting_destructor` (0x554090, den_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_den_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5540b0)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_north(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_northeast(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_parana(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_pern(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_rio(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_santa(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_sp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_den_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}
