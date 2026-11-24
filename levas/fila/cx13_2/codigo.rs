// ---------- cx13_2: the destructors of the Russian and Scottish leagues ----------

/// Destroys the Russian First Division (`drop_stage_league_unchecked` with the vtable 0x96f1c4).
/// Original: `RusFirst::~RusFirst` (0x7ea160, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_rus_first(this: usize) {
    drop_stage_league_unchecked(League(this), 0x96f1c4);
}

/// The scalar deleting destructor of the Russian First Division (`delete_through`).
/// Original: `RusFirst::scalar_deleting_destructor` (0x7ea140, rus_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_rus_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ea160)
}

/// Destroys the Russian Premier League (`drop_plain_league` with the vtable 0x96f278).
/// Original: `RusPrm::~RusPrm` (0x7eacc0, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_rus_prm(this: usize) {
    drop_plain_league(League(this), 0x96f278);
}

/// The scalar deleting destructor of the Russian Premier League (`delete_through`).
/// Original: `RusPrm::scalar_deleting_destructor` (0x7eaca0, rus_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_rus_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7eacc0)
}

/// Destroys the Scottish First Division (`drop_stats_league` with the vtable 0x96f4b4).
/// Original: `ScoFirst::~ScoFirst` (0x7ee220, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_first(this: usize) {
    drop_stats_league(this, 0x96f4b4);
}

/// The scalar deleting destructor of the Scottish First Division (`delete_through`).
/// Original: `ScoFirst::scalar_deleting_destructor` (0x7ee200, sco_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ee220)
}

/// Destroys the Scottish Premier Division (`drop_plain_league` with the vtable 0x96f608).
/// Original: `ScoPrm::~ScoPrm` (0x7f0df0, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_prm(this: usize) {
    drop_plain_league(League(this), 0x96f608);
}

/// The scalar deleting destructor of the Scottish Premier Division (`delete_through`).
/// Original: `ScoPrm::scalar_deleting_destructor` (0x7f0dd0, sco_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7f0df0)
}

/// Destroys the Scottish Second Division (`drop_stats_league` with the vtable 0x96f6bc).
/// Original: `ScoSecond::~ScoSecond` (0x7f3480, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_second(this: usize) {
    drop_stats_league(this, 0x96f6bc);
}

/// The scalar deleting destructor of the Scottish Second Division (`delete_through`).
/// Original: `ScoSecond::scalar_deleting_destructor` (0x7f3460, sco_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7f3480)
}

/// Destroys the Scottish Third Division (`drop_stats_league` with the vtable 0x96f770).
/// Original: `ScoThird::~ScoThird` (0x7f4420, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sco_third(this: usize) {
    drop_stats_league(this, 0x96f770);
}

/// The scalar deleting destructor of the Scottish Third Division (`delete_through`).
/// Original: `ScoThird::scalar_deleting_destructor` (0x7f4400, sco_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sco_third(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7f4420)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_rus_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_rus_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_rus_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_rus_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sco_third(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sco_third(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
