// ---------- batch %%LEVA%%: deleting a training schedule (training_screens.cpp) ----------

/// Rule: deleting a training schedule frees its memory when asked to.
///
/// The class destructor (0x5a53c0) is an empty `ret` and is not called; with bit 0 of `flags` the schedule is freed
/// (`operator delete`); returns `this`.
/// Original: `TRAINING_SCHEDULE::scalar_deleting_destructor` (0x8ba920, training_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_schedule_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    covered!("tel45_3 a schedule deleted");
    if flags & 1 != 0 {
        exe::crt_operator_delete(this);
    }
    this // tel45_3: the schedule
}

/// Comparer for `training_schedule_scalar_deleting_destructor`: it frees memory, so only the original runs; the plan
/// checked: it returns `this` (what the copy returns).
pub(crate) unsafe extern "thiscall" fn compare_training_schedule_scalar_deleting_destructor(this: usize, flags: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags);
    let plan = training_schedule_scalar_deleting_destructor(this, 0); // flags 0: the copy frees nothing
    SCOREBOARD.record(r == plan, || format!("training_schedule_scalar_deleting_destructor: returned {:#x}, the copy's plan {:#x}", r, plan));
    r
}
