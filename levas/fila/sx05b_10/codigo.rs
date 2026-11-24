// ---------- sx05b_10: the team award, year award and AWOL destructors ----------

/// `AWARD::~AWARD` (0x412ef0, thiscall): the base class's vtable (0x967864) back in place.
const FN_AWARD_DESTRUCT: usize = 0x412ef0;
/// `TEAM_AWARD::~TEAM_AWARD` (0x8af650) and `YEAR_AWARD::~YEAR_AWARD` (0x92ff80), thiscall, as the deleting
/// destructors call them (through the hooks).
const FN_TEAM_AWARD_DESTRUCT: usize = 0x8af650;
const FN_YEAR_AWARD_DESTRUCT: usize = 0x92ff80;

unsafe fn award_destruct_base(this: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_DESTRUCT);
    f(this);
}

/// Runs a class destructor at `destruct` and, with bit 0 of `flags`, frees the object (`exe::crt_operator_delete`).
/// Returns this.
unsafe fn award_deleting_destruct(this: usize, flags: u32, destruct: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destruct);
    f(this);
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Destroys a team award: its vtable back to the team award's (`team_award_obj::VTABLE`), then the award base
/// destructor.
/// Original: `TEAM_AWARD::~TEAM_AWARD` (0x8af650, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_destruct(this: usize) {
    write_usize(this, team_award_obj::VTABLE);
    award_destruct_base(this);
}

/// The team award's deleting destructor: `TEAM_AWARD::~TEAM_AWARD`, then the object freed with bit 0 of the flags.
/// Returns this.
/// Original: `TEAM_AWARD::scalar deleting destructor` (0x8af630, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_deleting_destruct(this: usize, flags: u32) -> usize {
    award_deleting_destruct(this, flags, FN_TEAM_AWARD_DESTRUCT)
}

/// Destroys a year award: its vtable back to the year award's, its shortlist (when there is one) destroyed (its entries
/// freed, `FILE_LLIST_DESTRUCT`) and freed (the pointer is left as it was), then the award base destructor.
/// Original: `YEAR_AWARD::~YEAR_AWARD` (0x92ff80, year_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn year_award_destruct(this: usize) {
    write_usize(this, year_award_obj::VTABLE);
    let shortlist = read_usize(this + year_award_obj::SHORTLIST);
    if shortlist != 0 {
        covered!("sx05b_10 shortlist freed");
        let destruct: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
        destruct(shortlist);
        exe::crt_operator_delete(shortlist);
    }
    award_destruct_base(this);
}

/// The year award's deleting destructor: `YEAR_AWARD::~YEAR_AWARD`, then the object freed with bit 0 of the flags.
/// Original: `YEAR_AWARD::scalar deleting destructor` (0x92ff60, year_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_award_deleting_destruct(this: usize, flags: u32) -> usize {
    award_deleting_destruct(this, flags, FN_YEAR_AWARD_DESTRUCT)
}

/// Destroys the AWOL state: the flag table freed (`exe::crt_free`) and each of the three lists destroyed
/// (`exe::drop_llist`) and freed, every pointer cleared.
/// Original: `AWOL::~AWOL` (0x4185f0, awol.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn awol_destruct(this: usize) {
    let flags = read_usize(this + awol_fields::PERSON_FLAGS);
    if flags != 0 {
        exe::crt_free(flags);
        write_usize(this + awol_fields::PERSON_FLAGS, 0);
    }
    for (off, _) in AWOL_LISTS {
        let list = read_usize(this + off);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + off, 0);
        }
    }
}

/// Comparer for `team_award_destruct`: side by side over the vtable (it frees nothing).
pub(crate) unsafe extern "thiscall" fn compare_team_award_destruct(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this, 4)], || o(this), || team_award_destruct(this));
    SCOREBOARD.record(equal, || format!("team_award_destruct({:#x}) {:?}", this, first_differences(&a, &b)));
}

/// Comparer for `team_award_deleting_destruct`: it may free the object; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_team_award_deleting_destruct(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

/// Comparer for `year_award_destruct`: it frees the shortlist; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_year_award_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

/// Comparer for `year_award_deleting_destruct`: only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_year_award_deleting_destruct(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}

/// Comparer for `awol_destruct`: it frees; only the original runs, and every pointer must end cleared.
pub(crate) unsafe extern "thiscall" fn compare_awol_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this);
    let ok = read_usize(this + awol_fields::PERSON_FLAGS) == 0 && AWOL_LISTS.iter().all(|&(off, _)| read_usize(this + off) == 0);
    SCOREBOARD.record(ok, || format!("awol_destruct({:#x}): pointers left", this));
}
