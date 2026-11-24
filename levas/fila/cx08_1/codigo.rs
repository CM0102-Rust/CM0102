// ---------- cx08_1: the destructors of the Argentine, Australian and Belgian leagues ----------

/// The body of the Argentine leagues' destructors (thiscall, ret 0; the SEH frame only covers the base on the exception
/// path): the class vtable back, the table freed (`exe::league_free_table` with 0), the round dates freed and cleared,
/// every stage 0..=`current_stage` (signed, read again each time) destroyed through its virtual 0 with 1, the current
/// stage set to −1 and the stage vector freed (unchecked, and left as it is), the statistics destroyed (left as they
/// are), then League::~League (`exe::drop_league`).
unsafe fn drop_argentine_league(this: League, vtable: usize) {
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
    this.set_current_stage(0xffff_ffff);
    exe::crt_free(this.stages());
    delete_comp_stats(this.comp_stats());
    exe::drop_league(this);
}

/// Destroys the Argentine Premier Division (`drop_argentine_league` with the vtable 0x967324).
/// Original: `ArgPrm::~ArgPrm` (0x4044b0, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_arg_prm(this: usize) {
    covered!("cx08_1 argentine premier destroyed");
    drop_argentine_league(League(this), 0x967324);
}

/// The scalar deleting destructor of the Argentine Premier Division: `ArgPrm::~ArgPrm` and, with bit 0 of `flags`, the
/// object (`delete_through`).
/// Original: `ArgPrm::scalar_deleting_destructor` (0x404490, arg_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_arg_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4044b0)
}

/// Destroys the Argentine Second Division (`drop_argentine_league` with the vtable 0x9673d8).
/// Original: `ArgSecond::~ArgSecond` (0x406e40, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_arg_second(this: usize) {
    drop_argentine_league(League(this), 0x9673d8);
}

/// The scalar deleting destructor of the Argentine Second Division (`delete_through`).
/// Original: `ArgSecond::scalar_deleting_destructor` (0x406e20, arg_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_arg_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x406e40)
}

/// Destroys the Australian National Soccer League: the table, the round dates, every stage up to the current one, the
/// stage vector and the statistics, then League::~League (`drop_stats_league` with the vtable 0x967768).
/// Original: `AusNsl::~AusNsl` (0x4110e0, aus_nsl.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_aus_nsl(this: usize) {
    drop_stats_league(this, 0x967768);
}

/// The scalar deleting destructor of the Australian league (`delete_through`).
/// Original: `AusNsl::scalar_deleting_destructor` (0x4110c0, aus_nsl.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_aus_nsl(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x4110e0)
}

/// Destroys the Belgian First Division, a league with no stages: the vtable 0x967940 back, the table freed
/// (`exe::league_free_table` with 0), the round dates freed and cleared, the statistics destroyed (left as they are),
/// then League::~League (`exe::drop_league`).
/// Original: `BelFirst::~BelFirst` (0x41df80, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bel_first(this: usize) {
    let l = League(this);
    write_usize(this, 0x967940);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    delete_comp_stats(l.comp_stats());
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Belgian First Division (`delete_through`).
/// Original: `BelFirst::scalar_deleting_destructor` (0x41df60, bel_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bel_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x41df80)
}

/// Destroys the Belgian Second Division (`drop_stats_league` with the vtable 0x9679f4).
/// Original: `BelSecond::~BelSecond` (0x41f280, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_bel_second(this: usize) {
    drop_stats_league(this, 0x9679f4);
}

/// The scalar deleting destructor of the Belgian Second Division (`delete_through`).
/// Original: `BelSecond::scalar_deleting_destructor` (0x41f260, bel_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_bel_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x41f280)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_arg_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_arg_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_arg_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_arg_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_aus_nsl(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_aus_nsl(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bel_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bel_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_bel_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_bel_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}
