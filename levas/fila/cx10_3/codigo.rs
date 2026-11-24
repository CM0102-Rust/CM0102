// ---------- cx10_3: the destructors of the Greek, Dutch and Irish leagues ----------

/// Destroys the Greek First Division (`drop_plain_league` with the vtable 0x96b568).
/// Original: `GrePrm::~GrePrm` (0x5ea640, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_gre_prm(this: usize) {
    covered!("cx10_3 greek first division destroyed");
    drop_plain_league(League(this), 0x96b568);
}

/// The scalar deleting destructor of the Greek First Division (`delete_through`).
/// Original: `GrePrm::scalar_deleting_destructor` (0x5ea620, gre_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_gre_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5ea640)
}

/// Destroys the Greek Second Division (`drop_plain_league` with the vtable 0x96b61c).
/// Original: `GreSecond::~GreSecond` (0x5eb4e0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_gre_second(this: usize) {
    drop_plain_league(League(this), 0x96b61c);
}

/// The scalar deleting destructor of the Greek Second Division (`delete_through`).
/// Original: `GreSecond::scalar_deleting_destructor` (0x5eb4c0, gre_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_gre_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5eb4e0)
}

/// Destroys the Dutch First Division (`drop_stats_league` with the vtable 0x96b86c).
/// Original: `HolFirst::~HolFirst` (0x5f56d0, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_hol_first(this: usize) {
    drop_stats_league(this, 0x96b86c);
}

/// The scalar deleting destructor of the Dutch First Division (`delete_through`).
/// Original: `HolFirst::scalar_deleting_destructor` (0x5f56b0, hol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_hol_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5f56d0)
}

/// Destroys the Dutch Premier Division (`drop_plain_league` with the vtable 0x96b920).
/// Original: `HolPrm::~HolPrm` (0x5f7840, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_hol_prm(this: usize) {
    drop_plain_league(League(this), 0x96b920);
}

/// The scalar deleting destructor of the Dutch Premier Division (`delete_through`).
/// Original: `HolPrm::scalar_deleting_destructor` (0x5f7820, hol_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_hol_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5f7840)
}

/// Destroys the Irish First Division (`drop_stats_league` with the vtable 0x96bccc).
/// Original: `IreFirst::~IreFirst` (0x634e80, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_first(this: usize) {
    drop_stats_league(this, 0x96bccc);
}

/// The scalar deleting destructor of the Irish First Division (`delete_through`).
/// Original: `IreFirst::scalar_deleting_destructor` (0x634e60, ire_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x634e80)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_gre_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_gre_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_gre_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_gre_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_hol_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_hol_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_hol_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_hol_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ire_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}
