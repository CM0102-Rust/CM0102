// ---------- batch %%LEVA%%: the award base class's destructors (award_manager.cpp) ----------

/// The vtable of the award base class `AWARD` (slot 0 its scalar deleting destructor 0x412ed0).
pub(crate) const AWARD_VTABLE: usize = 0x967864;

/// Destroys an award base object: puts back the base class's vtable (`AWARD_VTABLE`); nothing else to free.
///
/// `AWARD::~AWARD()`: the tail of the derived award classes' destructors (WorldCupAwards' and others call or jump to
/// it).
/// Original: `AWARD::~AWARD` (0x412ef0, award_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn award_destruct(this: usize) {
    covered!("rxf_3 award destructor");
    write_usize(this, AWARD_VTABLE);
}

/// Comparer for `award_destruct`: side by side over the object's vtable dword.
pub(crate) unsafe extern "thiscall" fn compare_award_destruct(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this, 4)], || o(this), || award_destruct(this));
    SCOREBOARD.record(equal, || format!("award_destruct {:?}", first_differences(&a, &b)));
}

/// The scalar deleting destructor of the award base class: the base destructor (the vtable put back) and, with bit 0
/// of `flags`, `operator delete` on the object. Returns the object.
///
/// `AWARD::scalar deleting destructor(flags)`: slot 0 of `AWARD_VTABLE`; only an object whose dynamic class is the base
/// `AWARD` reaches it (every derived class has its own), so it does not run in the careers.
/// Original: `AWARD::scalar_deleting_destructor` (0x412ed0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_scalar_destruct(this: usize, flags: u32) -> usize {
    covered!("rxf_3 award scalar destructor");
    write_usize(this, AWARD_VTABLE);
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this
}

/// Comparer for `award_scalar_destruct`: with the delete bit it frees the object, so only the original runs; without
/// it, side by side over the vtable dword.
pub(crate) unsafe extern "thiscall" fn compare_award_scalar_destruct(this: usize, flags: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%);
    if flags & 1 != 0 {
        return o(this, flags);
    }
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(this, 4)], || o(this, flags), || award_scalar_destruct(this, flags));
    SCOREBOARD.record(equal && r == v, || format!("award_scalar_destruct {:?}", first_differences(&a, &b)));
    r
}
