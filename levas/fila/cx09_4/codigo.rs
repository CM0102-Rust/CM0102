// ---------- cx09_4: twelve league scalar deleting destructors ----------

/// The scalar deleting destructor of the Brazilian Serie C: `BraNatThird::~BraNatThird` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraNatThird::scalar_deleting_destructor` (0x42d520, bra_nat_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_nat_third(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x42d540)
}

/// The scalar deleting destructor of the Croatian First Division: `CroA1::~CroA1` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `CroA1::scalar_deleting_destructor` (0x514030, cro_a1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cro_a1(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x514050)
}

/// The scalar deleting destructor of the Croatian Second Division group A: `CroA2a::~CroA2a` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `CroA2a::scalar_deleting_destructor` (0x515530, cro_a2a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cro_a2a(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x515550)
}

/// The scalar deleting destructor of the Croatian Second Division group B: `CroA2b::~CroA2b` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `CroA2b::scalar_deleting_destructor` (0x516650, cro_a2b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cro_a2b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x516670)
}

/// The scalar deleting destructor of the Danish First Division: `DenFirst::~DenFirst` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `DenFirst::scalar_deleting_destructor` (0x551900, den_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_den_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x551920)
}

/// The scalar deleting destructor of the Danish Superliga: `DenPrm::~DenPrm` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `DenPrm::scalar_deleting_destructor` (0x552850, den_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_den_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x552870)
}

/// The scalar deleting destructor of the English Premier League: `EngPrm::~EngPrm` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EngPrm::scalar_deleting_destructor` (0x5735b0, eng_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5735d0)
}

/// The scalar deleting destructor of the Bahia state championship: `BraRegBahia::~BraRegBahia` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegBahia::scalar_deleting_destructor` (0x42f4f0, bra_reg_bahia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_bahia(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x42f510)
}

/// The scalar deleting destructor of the Central regional championship: `BraRegCentral::~BraRegCentral` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegCentral::scalar_deleting_destructor` (0x4306d0, bra_reg_central.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_central(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4306f0)
}

/// The scalar deleting destructor of the Rio Grande do Sul state championship: `BraRegGaucho::~BraRegGaucho` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegGaucho::scalar_deleting_destructor` (0x4319e0, bra_reg_gaucho.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_gaucho(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x431a00)
}

/// The scalar deleting destructor of the Goias state championship: `BraRegGoias::~BraRegGoias` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegGoias::scalar_deleting_destructor` (0x432d70, bra_reg_goias.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_goias(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x432d90)
}

/// The scalar deleting destructor of the Minas Gerais state championship: `BraRegMinasGerais::~BraRegMinasGerais` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraRegMinasGerais::scalar_deleting_destructor` (0x433f70, bra_reg_minas_gerais.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_reg_minas_gerais(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x433f90)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_bra_nat_third(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_cro_a1(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_cro_a2a(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_cro_a2b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_den_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_den_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_bahia(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_central(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_gaucho(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_goias(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_reg_minas_gerais(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
