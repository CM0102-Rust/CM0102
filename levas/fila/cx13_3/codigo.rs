// ---------- cx13_3: the destructors of the Spanish leagues ----------

/// Destroys the Spanish First Division: the vtable 0x96fa68 back, the table freed (`exe::league_free_table` with 0),
/// the round dates freed and cleared, the statistics destroyed (left as they are), then the Spanish lower division it
/// created (`lower_division`) destroyed through its virtual slot 0 with 1 (the pointer left as it is), then
/// League::~League (`exe::drop_league`).
/// Original: `SpaFirst::~SpaFirst` (0x84e3b0, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_first(this: usize) {
    covered!("cx13_3 spanish first destroyed");
    let l = League(this);
    write_usize(this, 0x96fa68);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    delete_comp_stats(l.comp_stats());
    delete_owned_comp(SpaFirstFields(this).lower_division());
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Spanish First Division (`delete_through`).
/// Original: `SpaFirst::scalar_deleting_destructor` (0x84e390, spa_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_first(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x84e3b0)
}

/// Destroys the Spanish lower division (the generic league of the clubs below Segunda B): the vtable 0x96fb1c back,
/// the table freed (`exe::league_free_table` with 0), the round dates freed and cleared, every stage
/// 0..=`current_stage` (signed, read again each time) destroyed through its virtual slot 0 with 1, the stage vector
/// freed and cleared, then League::~League (`exe::drop_league`). It has no statistics to destroy.
/// Original: `SpaLower::~SpaLower` (0x8511b0, spa_lower.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_lower(this: usize) {
    let l = League(this);
    write_usize(this, 0x96fb1c);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    let mut k = 0i32;
    while k <= l.current_stage() as i32 {
        let stage = read_usize(l.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        k += 1;
    }
    if l.stages() != 0 {
        exe::crt_free(l.stages());
        l.set_stages(0);
    }
    exe::drop_league(l);
}

/// The scalar deleting destructor of the Spanish lower division (`delete_through`).
/// Original: `SpaLower::scalar_deleting_destructor` (0x851190, spa_lower.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_lower(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x8511b0)
}

/// Destroys the Spanish Second Division (`drop_stats_league` with the vtable 0x96fbd0).
/// Original: `SpaSecond::~SpaSecond` (0x853bd0, spa_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_spa_second(this: usize) {
    drop_stats_league(this, 0x96fbd0);
}

/// The scalar deleting destructor of the Spanish Second Division (`delete_through`).
/// Original: `SpaSecond::scalar_deleting_destructor` (0x853bb0, spa_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_spa_second(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x853bd0)
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_spa_first(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_spa_first(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_spa_lower(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_spa_lower(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_spa_second(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_spa_second(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I5%%)(this, flags)
}
