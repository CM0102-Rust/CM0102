// ---------- cx14_1: the destructors of the Spanish Second Division B, the sub league, the Swedish Superettan, the Swedish Allsvenskan ----------

/// Destroys the Spanish Second Division B (`drop_stats_league` with the vtable 0x96fc84).
/// Original: `SpaSecondB::~SpaSecondB` (0x8561a0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_second_b(this: usize) {
    drop_stats_league(this, 0x96fc84);
}

/// The scalar deleting destructor of the Spanish Second Division B (`delete_through`).
/// Original: `SpaSecondB::scalar_deleting_destructor` (0x856180, spa_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_second_b(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8561a0)
}

/// Destroys a sub league: the vtable 0x96ff90 back, the table freed (`exe::league_free_table` with 0), then
/// League::~League (`exe::drop_league`). A sub league has no round table, stages nor statistics of its own.
/// Original: `SubLeague::~SubLeague` (0x88c990, sub_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_sub_league(this: usize) {
    covered!("cx14_1 sub league destroyed");
    write_usize(this, 0x96ff90);
    exe::league_free_table(this, 0);
    exe::drop_league(this);
}

/// The scalar deleting destructor of the sub league (the table of a group of clubs) (`delete_through`).
/// Original: `SubLeague::scalar_deleting_destructor` (0x88c970, sub_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_sub_league(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x88c990)
}

/// Destroys the Swedish Superettan: the vtable 0x9700e4 back, the table freed (`exe::league_free_table` with 0),
/// the round dates freed and cleared, every stage up to the current one destroyed (virtual slot 0 with 1), the stage
/// vector freed (unlike `drop_stats_league`, without the null check and left in place: free(NULL) does nothing), the
/// statistics destroyed (left as they are), then League::~League (`exe::drop_league`).
/// Original: `SweFirst::~SweFirst` (0x88e820, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_swe_first(this: usize) {
    covered!("cx14_1 superettan destroyed");
    let l = League(this);
    write_usize(this, 0x9700e4);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= l.current_stage() as i32 {
        let stage = read_usize(l.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    exe::crt_free(l.stages());
    delete_comp_stats(l.comp_stats());
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Swedish Superettan (`delete_through`).
/// Original: `SweFirst::scalar_deleting_destructor` (0x88e800, swe_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_swe_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x88e820)
}

/// Destroys the Swedish Allsvenskan (`drop_plain_league` with the vtable 0x970198).
/// Original: `SwePrm::~SwePrm` (0x88fe20, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_swe_prm(this: usize) {
    drop_plain_league(League(this), 0x970198);
}

/// The scalar deleting destructor of the Swedish Allsvenskan (`delete_through`).
/// Original: `SwePrm::scalar_deleting_destructor` (0x88fe00, swe_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_swe_prm(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x88fe20)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_spa_second_b(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_spa_second_b(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_sub_league(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_sub_league(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_swe_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_swe_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_swe_prm(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I6%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_swe_prm(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I7%%)(this, flags)
}
