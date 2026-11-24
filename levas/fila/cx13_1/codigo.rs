// ---------- cx13_1: the destructors of the Polish and Portuguese leagues ----------

/// Destroys the Polish First Division: the vtable 0x96e360 back, the table freed (`exe::league_free_table` with 0), the
/// list of the 16 clubs sorted for the predicted positions (+0xee) freed when there is one (the pointer left as it is),
/// the round dates freed and cleared, the statistics destroyed (left as they are), then League::~League
/// (`exe::drop_league`). The stage vector and the stages are not freed here.
/// Original: `PolFirst::~PolFirst` (0x7c8d50, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_pol_first(this: usize) {
    covered!("cx13_1 polish first destroyed");
    let l = League(this);
    write_usize(this, 0x96e360);
    exe::league_free_table(l, 0);
    let clubs = read_usize(this + 0xee); // the 16 clubs sorted by strength (set_predicted_positions)
    if clubs != 0 {
        exe::crt_free(clubs);
    }
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    delete_comp_stats(l.comp_stats());
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Polish First Division (`delete_through`).
/// Original: `PolFirst::scalar_deleting_destructor` (0x7c8d30, pol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_pol_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7c8d50)
}

/// Destroys the Polish Second Division (`drop_plain_league` with the vtable 0x96e4b4).
/// Original: `PolSecond::~PolSecond` (0x7cb410, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_pol_second(this: usize) {
    drop_plain_league(League(this), 0x96e4b4);
}

/// The scalar deleting destructor of the Polish Second Division (`delete_through`).
/// Original: `PolSecond::scalar_deleting_destructor` (0x7cb3f0, pol_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_pol_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7cb410)
}

/// Destroys the Portuguese Premier Division (`drop_plain_league` with the vtable 0x96e6f0).
/// Original: `PorPrm::~PorPrm` (0x7ce950, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_por_prm(this: usize) {
    drop_plain_league(League(this), 0x96e6f0);
}

/// The scalar deleting destructor of the Portuguese Premier Division (`delete_through`).
/// Original: `PorPrm::scalar_deleting_destructor` (0x7ce930, por_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_por_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7ce950)
}

/// Destroys the Portuguese Second Division (`drop_stats_league` with the vtable 0x96e7a4).
/// Original: `PorSecond::~PorSecond` (0x7d0b80, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_por_second(this: usize) {
    drop_stats_league(this, 0x96e7a4);
}

/// The scalar deleting destructor of the Portuguese Second Division (`delete_through`).
/// Original: `PorSecond::scalar_deleting_destructor` (0x7d0b60, por_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_por_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7d0b80)
}

/// Destroys the Portuguese Second Division B (`drop_stats_league` with the vtable 0x96e858).
/// Original: `PorSecondB::~PorSecondB` (0x7d1b80, por_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_por_second_b(this: usize) {
    drop_stats_league(this, 0x96e858);
}

/// The scalar deleting destructor of the Portuguese Second Division B (`delete_through`).
/// Original: `PorSecondB::scalar_deleting_destructor` (0x7d1b60, por_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_por_second_b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7d1b80)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_pol_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_pol_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_pol_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_pol_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_por_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_por_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_por_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_por_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_por_second_b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_por_second_b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I9%%)(this, flags)
}
