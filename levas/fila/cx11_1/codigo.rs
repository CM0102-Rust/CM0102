// ---------- cx11_1: the destructors of the Italian Serie C2 groups A and B (ita_ser_c2a.cpp, ita_ser_c2b.cpp) ----------

/// Destroys the Italian Serie C2/A (`drop_stage_league_unchecked` with the vtable 0x96c69c): the table, the round
/// table, the stages up to the current one, the stage vector (not tested nor cleared) and the statistics, then the
/// league base.
/// Original: `ItaSerC2a::~ItaSerC2a` (0x6530e0, ita_ser_c2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_c2a(this: usize) {
    covered!("cx11_1 serie c2 destroyed");
    drop_stage_league_unchecked(League(this), 0x96c69c);
}

/// The scalar deleting destructor of the Italian Serie C2/A (`delete_through`).
/// Original: `ItaSerC2a::scalar_deleting_destructor` (0x6530c0, ita_ser_c2a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_c2a(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x6530e0)
}

/// Destroys the Italian Serie C2/B (`drop_stage_league_unchecked` with the vtable 0x96c750).
/// Original: `ItaSerC2b::~ItaSerC2b` (0x6575a0, ita_ser_c2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_c2b(this: usize) {
    drop_stage_league_unchecked(League(this), 0x96c750);
}

/// The scalar deleting destructor of the Italian Serie C2/B (`delete_through`).
/// Original: `ItaSerC2b::scalar_deleting_destructor` (0x657580, ita_ser_c2b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_c2b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x6575a0)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_c2a(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_c2a(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_c2b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_c2b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}
