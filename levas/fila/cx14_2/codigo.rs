// ---------- cx14_2: the destructors of the Swedish Second Division, the Turkish First Division, the Turkish Second Division, the Turkish Second Division category B ----------

/// Destroys the Swedish Second Division (`drop_stats_league` with the vtable 0x9702a0).
/// Original: `SweSecond::~SweSecond` (0x8919a0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_swe_second(this: usize) {
    drop_stats_league(this, 0x9702a0);
}

/// The scalar deleting destructor of the Swedish Second Division (`delete_through`).
/// Original: `SweSecond::scalar_deleting_destructor` (0x891980, swe_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_swe_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8919a0)
}

/// Destroys the Turkish First Division (`drop_plain_league` with the vtable 0x97052c).
/// Original: `TurFirst::~TurFirst` (0x8fd190, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_tur_first(this: usize) {
    drop_plain_league(League(this), 0x97052c);
}

/// The scalar deleting destructor of the Turkish First Division (`delete_through`).
/// Original: `TurFirst::scalar_deleting_destructor` (0x8fd170, tur_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_tur_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8fd190)
}

/// Destroys the Turkish Second Division (category A) (`drop_plain_league` with the vtable 0x9705e0).
/// Original: `TurSecond::~TurSecond` (0x8fe920, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_tur_second(this: usize) {
    drop_plain_league(League(this), 0x9705e0);
}

/// The scalar deleting destructor of the Turkish Second Division (category A) (`delete_through`).
/// Original: `TurSecond::scalar_deleting_destructor` (0x8fe900, tur_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_tur_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8fe920)
}

/// Destroys the Turkish Second Division category B (`drop_stats_league` with the vtable 0x970694).
/// Original: `TurSecondB::~TurSecondB` (0x8ff720, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_tur_second_b(this: usize) {
    drop_stats_league(this, 0x970694);
}

/// The scalar deleting destructor of the Turkish Second Division category B (`delete_through`).
/// Original: `TurSecondB::scalar_deleting_destructor` (0x8ff700, tur_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_tur_second_b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8ff720)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_swe_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_swe_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_tur_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_tur_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_tur_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_tur_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_tur_second_b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_tur_second_b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}
