// ---------- cx03_3: the destructors of the three American continental cups and nine cup scalar deleting destructors ----------

/// The body of the cups' scalar deleting destructors (thiscall, ret 4): the class destructor at `drop` (thiscall, ret 0,
/// called through its detour) and, with bit 0 of `flags`, the object itself (`exe::crt_operator_delete`). Returns the
/// object.
pub(crate) unsafe fn delete_through(this: usize, flags: u32, drop: usize) -> usize {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(drop);
    destroy(this);
    if flags as u8 & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Destroys the Copa Merconorte (`drop_group_cup` without an entrant list).
/// Original: `ConMercCup::~ConMercCup` (0x4c18a0, con_merc_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_con_merc_cup(this: usize) {
    drop_group_cup(this, 0x968b40, false);
}

/// Destroys the Copa Libertadores (`drop_group_cup` without an entrant list).
/// Original: `ConmebolLiber::~ConmebolLiber` (0x4c3100, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_conmebol_liber(this: usize) {
    drop_group_cup(this, 0x968bf4, false);
}

/// Destroys the Copa Mercosur (`drop_group_cup` without an entrant list).
/// Original: `ConmebolMerc::~ConmebolMerc` (0x4c4870, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_conmebol_merc(this: usize) {
    drop_group_cup(this, 0x968ca8, false);
}

/// The scalar deleting destructor of the Polish Cup: `PolCup::~PolCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `PolCup::scalar_deleting_destructor` (0x7c7f50, pol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_pol_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7c7f70)
}

/// The scalar deleting destructor of the Polish League Cup: `PolLgeCup::~PolLgeCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `PolLgeCup::scalar_deleting_destructor` (0x7ca6b0, pol_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_pol_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ca6d0)
}

/// The scalar deleting destructor of the Polish Super Cup: `PolSuper::~PolSuper` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `PolSuper::scalar_deleting_destructor` (0x7ccae0, pol_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_pol_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ccb00)
}

/// The scalar deleting destructor of the Portuguese Cup: `PorCup::~PorCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `PorCup::scalar_deleting_destructor` (0x7cdb50, por_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_por_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7cdb70)
}

/// The scalar deleting destructor of the Portuguese Super Cup: `PorSuper::~PorSuper` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `PorSuper::scalar_deleting_destructor` (0x7d3000, por_super.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_por_super(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7d3020)
}

/// The scalar deleting destructor of the Scottish Challenge Cup: `ScoChalCup::~ScoChalCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ScoChalCup::scalar_deleting_destructor` (0x7ecb50, sco_chal_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_chal_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ecb70)
}

/// The scalar deleting destructor of the Scottish Cup: `ScoFaCup::~ScoFaCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ScoFaCup::scalar_deleting_destructor` (0x7ed4e0, sco_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_fa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ed500)
}

/// The scalar deleting destructor of the Scottish League Cup: `ScoLgeCup::~ScoLgeCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `ScoLgeCup::scalar_deleting_destructor` (0x7efbc0, sco_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_lge_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7efbe0)
}

/// The scalar deleting destructor of the Spanish Cup: `SpaCup::~SpaCup` and, with bit 0 of `flags`, the object (`delete_through`).
/// Original: `SpaCup::scalar_deleting_destructor` (0x84d130, spa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_cup(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x84d150)
}

/// The comparers of the destructors: they free memory (cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_con_merc_cup(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_conmebol_liber(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_conmebol_merc(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_pol_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_pol_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_pol_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_por_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I6%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_por_super(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_chal_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I8%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_fa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_lge_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I10%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_spa_cup(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
