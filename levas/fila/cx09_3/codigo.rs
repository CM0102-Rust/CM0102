// ---------- cx09_3: the destructors of three state championships and the Danish Second Division, and eight league scalar deleting destructors ----------

/// Destroys the Rio de Janeiro state championship (`drop_regional_league`).
/// Original: `BraRegRio::~BraRegRio` (0x439c80, bra_reg_rio.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_rio(this: usize) {
    drop_regional_league(this, 0x968608);
}

/// Destroys the Santa Catarina state championship (`drop_regional_league`).
/// Original: `BraRegSanta::~BraRegSanta` (0x43b370, bra_reg_santa.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_santa(this: usize) {
    drop_regional_league(this, 0x9686bc);
}

/// Destroys the Sao Paulo state championship (`drop_regional_league`).
/// Original: `BraRegSp::~BraRegSp` (0x43c4e0, bra_reg_sp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bra_reg_sp(this: usize) {
    drop_regional_league(this, 0x968770);
}

/// Destroys the Danish Second Division (`drop_regional_league`).
/// Original: `DenSecond::~DenSecond` (0x5540b0, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_den_second(this: usize) {
    drop_regional_league(this, 0x969798);
}

/// The scalar deleting destructor of the Belgian Third Division: `BelThird::~BelThird` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BelThird::scalar_deleting_destructor` (0x421920, bel_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bel_third(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x421940)
}

/// The scalar deleting destructor of the Brazilian Serie A: `BraNatFirst::~BraNatFirst` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraNatFirst::scalar_deleting_destructor` (0x429a70, bra_nat_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_nat_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x429a90)
}

/// The scalar deleting destructor of the Brazilian Serie B: `BraNatSecond::~BraNatSecond` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `BraNatSecond::scalar_deleting_destructor` (0x42c060, bra_nat_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bra_nat_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x42c080)
}

/// The scalar deleting destructor of the Football Conference: `EngConf::~EngConf` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EngConf::scalar_deleting_destructor` (0x56dd80, eng_conf.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_conf(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x56dda0)
}

/// The scalar deleting destructor of the English First Division: `EngFirst::~EngFirst` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EngFirst::scalar_deleting_destructor` (0x5718a0, eng_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5718c0)
}

/// The scalar deleting destructor of the English Second Division: `EngSecond::~EngSecond` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EngSecond::scalar_deleting_destructor` (0x5756c0, eng_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5756e0)
}

/// The scalar deleting destructor of the English Third Division: `EngThird::~EngThird` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `EngThird::scalar_deleting_destructor` (0x577000, eng_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_eng_third(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x577020)
}

/// The scalar deleting destructor of the Finnish First Division: `FinFirst::~FinFirst` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `FinFirst::scalar_deleting_destructor` (0x5930e0, fin_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fin_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x593100)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_rio(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_santa(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bra_reg_sp(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_den_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bel_third(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_nat_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bra_nat_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_conf(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_eng_third(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fin_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
