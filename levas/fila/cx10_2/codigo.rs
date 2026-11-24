// ---------- cx10_2: the destructors of the French and German leagues ----------

/// Destroys the French Second Division (`drop_stats_league` with the vtable 0x96ac2c).
/// Original: `FraSecond::~FraSecond` (0x5be560, fra_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_second(this: usize) {
    covered!("cx10_2 french second division destroyed");
    drop_stats_league(this, 0x96ac2c);
}

/// The scalar deleting destructor of the French Second Division (`delete_through`).
/// Original: `FraSecond::scalar_deleting_destructor` (0x5be540, fra_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5be560)
}

/// Destroys the French Third Division (`drop_stats_league` with the vtable 0x96ad80).
/// Original: `FraThird::~FraThird` (0x5bfd70, fra_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fra_third(this: usize) {
    drop_stats_league(this, 0x96ad80);
}

/// The scalar deleting destructor of the French Third Division (`delete_through`).
/// Original: `FraThird::scalar_deleting_destructor` (0x5bfd50, fra_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fra_third(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5bfd70)
}

/// Destroys the German First Division (`drop_plain_league` with the vtable 0x96b0e4).
/// Original: `GerFirst::~GerFirst` (0x5d9be0, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ger_first(this: usize) {
    drop_plain_league(League(this), 0x96b0e4);
}

/// The scalar deleting destructor of the German First Division (`delete_through`).
/// Original: `GerFirst::scalar_deleting_destructor` (0x5d9bc0, ger_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ger_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5d9be0)
}

/// Destroys the German Regional League (`drop_stats_league` with the vtable 0x96b250).
/// Original: `GerRegional::~GerRegional` (0x5dce30, ger_regional.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ger_regional(this: usize) {
    drop_stats_league(this, 0x96b250);
}

/// The scalar deleting destructor of the German Regional League (`delete_through`).
/// Original: `GerRegional::scalar_deleting_destructor` (0x5dce10, ger_regional.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ger_regional(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5dce30)
}

/// Destroys the German Second Division (`drop_stats_league` with the vtable 0x96b304).
/// Original: `GerSecond::~GerSecond` (0x5de510, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ger_second(this: usize) {
    drop_stats_league(this, 0x96b304);
}

/// The scalar deleting destructor of the German Second Division (`delete_through`).
/// Original: `GerSecond::scalar_deleting_destructor` (0x5de4f0, ger_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ger_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x5de510)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_fra_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fra_third(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fra_third(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ger_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ger_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ger_regional(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ger_regional(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ger_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ger_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}
