// ---------- cx12_1: the destructors of the Italian Serie C2/C, the Japanese, Korean and Northern Irish leagues ----------

/// Destroys the Italian Serie C2/C: the table, the round dates, every stage up to the current one, the stage vector
/// (not tested nor cleared) and the statistics, then League::~League (`drop_stage_league_unchecked` with the vtable
/// 0x96c804).
/// Original: `ItaSerC2c::~ItaSerC2c` (0x65b730, ita_ser_c2c.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_c2c(this: usize) {
    covered!("cx12_1 italian serie c2/c destroyed");
    drop_stage_league_unchecked(League(this), 0x96c804);
}

/// The scalar deleting destructor of the Italian Serie C2/C (`delete_through`).
/// Original: `ItaSerC2c::scalar_deleting_destructor` (0x65b710, ita_ser_c2c.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_c2c(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x65b730)
}

/// Destroys the Japanese J1 League: the table, the round dates, every stage up to the current one, the current
/// stage back to −1, the stage vector (not tested nor cleared) and the statistics, then League::~League
/// (`drop_argentine_league` with the vtable 0x96ca40).
/// Original: `JapJ1::~JapJ1` (0x662d70, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_jap_j1(this: usize) {
    drop_argentine_league(League(this), 0x96ca40);
}

/// The scalar deleting destructor of the Japanese J1 League (`delete_through`).
/// Original: `JapJ1::scalar_deleting_destructor` (0x662d50, jap_j1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_jap_j1(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x662d70)
}

/// Destroys the Japanese J2 League: the table, the round dates and the statistics (no stages), then League::~League
/// (`drop_plain_league` with the vtable 0x96caf4).
/// Original: `JapJ2::~JapJ2` (0x6646f0, jap_j2.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_jap_j2(this: usize) {
    drop_plain_league(League(this), 0x96caf4);
}

/// The scalar deleting destructor of the Japanese J2 League (`delete_through`).
/// Original: `JapJ2::scalar_deleting_destructor` (0x6646d0, jap_j2.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_jap_j2(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x6646f0)
}

/// Destroys the Korean league: the table, the round dates and the statistics (no stages), then League::~League
/// (`drop_plain_league` with the vtable 0x96cdd0).
/// Original: `KorLeague::~KorLeague` (0x66b0a0, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_kor_league(this: usize) {
    drop_plain_league(League(this), 0x96cdd0);
}

/// The scalar deleting destructor of the Korean league (`delete_through`).
/// Original: `KorLeague::scalar_deleting_destructor` (0x66b080, kor_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_kor_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x66b0a0)
}

/// Destroys the Northern Irish First Division: the table, the round dates, every stage up to the current one, the
/// stage vector (freed and cleared) and the statistics, then League::~League (`drop_stats_league` with the vtable
/// 0x96db64).
/// Original: `NirFirst::~NirFirst` (0x78b250, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nir_first(this: usize) {
    drop_stats_league(this, 0x96db64);
}

/// The scalar deleting destructor of the Northern Irish First Division (`delete_through`).
/// Original: `NirFirst::scalar_deleting_destructor` (0x78b230, nir_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nir_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78b250)
}

/// Destroys the Northern Irish Premier Division: the table, the round dates and the statistics (no stages), then
/// League::~League (`drop_plain_league` with the vtable 0x96dccc).
/// Original: `NirPrm::~NirPrm` (0x78e2a0, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_nir_prm(this: usize) {
    drop_plain_league(League(this), 0x96dccc);
}

/// The scalar deleting destructor of the Northern Irish Premier Division (`delete_through`).
/// Original: `NirPrm::scalar_deleting_destructor` (0x78e280, nir_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_nir_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x78e2a0)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_c2c(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_c2c(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_jap_j1(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_jap_j1(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_jap_j2(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_jap_j2(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_kor_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_kor_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nir_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nir_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_nir_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I10%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_nir_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I11%%)(this, flags)
}
