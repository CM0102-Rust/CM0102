// ---------- cx10_4: the destructors of the Irish and Italian leagues ----------

/// Destroys a league whose stage vector is freed unchecked (thiscall, ret 0; the SEH frame only covers the base on the
/// exception path): the class vtable back, the table freed (`exe::league_free_table` with 0), the round dates freed and
/// cleared, every stage 0..=`current_stage` (signed, read again each time) destroyed through its virtual 0 with 1, the
/// stage vector freed without a test and left as it is (the current stage too), the statistics destroyed (left as they
/// are), then League::~League (`exe::drop_league`). Unlike `drop_stats_league`, the vector is not tested nor cleared.
unsafe fn drop_stage_league_unchecked(this: League, vtable: usize) {
    write_usize(this.addr(), vtable);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        k += 1;
    }
    exe::crt_free(this.stages());
    delete_comp_stats(this.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Irish Premier Division (`drop_plain_league` with the vtable 0x96c014).
/// Original: `IrePrm::~IrePrm` (0x639e00, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ire_prm(this: usize) {
    covered!("cx10_4 irish premier division destroyed");
    drop_plain_league(League(this), 0x96c014);
}

/// The scalar deleting destructor of the Irish Premier Division (`delete_through`).
/// Original: `IrePrm::scalar_deleting_destructor` (0x639de0, ire_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ire_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x639e00)
}

/// Destroys the Italian Serie A (`drop_plain_league` with the vtable 0x96c3cc).
/// Original: `ItaSerA::~ItaSerA` (0x640920, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_a(this: usize) {
    drop_plain_league(League(this), 0x96c3cc);
}

/// The scalar deleting destructor of the Italian Serie A (`delete_through`).
/// Original: `ItaSerA::scalar_deleting_destructor` (0x640900, ita_ser_a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_a(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x640920)
}

/// Destroys the Italian Serie B (`drop_plain_league` with the vtable 0x96c480).
/// Original: `ItaSerB::~ItaSerB` (0x645a60, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_b(this: usize) {
    drop_plain_league(League(this), 0x96c480);
}

/// The scalar deleting destructor of the Italian Serie B (`delete_through`).
/// Original: `ItaSerB::scalar_deleting_destructor` (0x645a40, ita_ser_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x645a60)
}

/// Destroys the Italian Serie C1/A (`drop_stage_league_unchecked` with the vtable 0x96c534).
/// Original: `ItaSerC1a::~ItaSerC1a` (0x64acc0, ita_ser_c1a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_c1a(this: usize) {
    drop_stage_league_unchecked(League(this), 0x96c534);
}

/// The scalar deleting destructor of the Italian Serie C1/A (`delete_through`).
/// Original: `ItaSerC1a::scalar_deleting_destructor` (0x64aca0, ita_ser_c1a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_c1a(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x64acc0)
}

/// Destroys the Italian Serie C1/B (`drop_stage_league_unchecked` with the vtable 0x96c5e8).
/// Original: `ItaSerC1b::~ItaSerC1b` (0x64ee70, ita_ser_c1b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_ita_ser_c1b(this: usize) {
    drop_stage_league_unchecked(League(this), 0x96c5e8);
}

/// The scalar deleting destructor of the Italian Serie C1/B (`delete_through`).
/// Original: `ItaSerC1b::scalar_deleting_destructor` (0x64ee50, ita_ser_c1b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_ita_ser_c1b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x64ee70)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_ire_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ire_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_a(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_a(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_c1a(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_c1a(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_ita_ser_c1b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_ita_ser_c1b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}
