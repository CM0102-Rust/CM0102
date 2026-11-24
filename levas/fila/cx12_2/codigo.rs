// ---------- cx12_2: the destructors of the Norwegian leagues and the league stage scalar deleting destructor ----------

/// Destroys the Norwegian First Division: the table, the round dates, every stage up to the current one, the stage
/// vector (freed and cleared) and the statistics, then League::~League (`drop_stats_league` with the vtable
/// 0x96de20).
/// Original: `NorFirst::~NorFirst` (0x78ff80, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nor_first(this: usize) {
    covered!("cx12_2 norwegian first division destroyed");
    drop_stats_league(this, 0x96de20);
}

/// The scalar deleting destructor of the Norwegian First Division (`delete_through`).
/// Original: `NorFirst::scalar_deleting_destructor` (0x78ff60, nor_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nor_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78ff80)
}

/// Destroys the Norwegian Premier Division: the table, the round dates and the statistics (no stages), then
/// League::~League (`drop_plain_league` with the vtable 0x96ded4).
/// Original: `NorPrm::~NorPrm` (0x791730, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nor_prm(this: usize) {
    drop_plain_league(League(this), 0x96ded4);
}

/// The scalar deleting destructor of the Norwegian Premier Division (`delete_through`).
/// Original: `NorPrm::scalar_deleting_destructor` (0x791710, nor_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nor_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x791730)
}

/// The scalar deleting destructor of the league stage: `League_Stage::~League_Stage` (already ported, through its
/// detour) and, with bit 0
/// of `flags`, the object (`delete_through`).
/// Original: `League_Stage::scalar_deleting_destructor` (0x68b4d0, league_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_league_stage(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x68b6a0)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_nor_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nor_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nor_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nor_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_league_stage(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I4%%)(this, flags)
}
