// ---------- rx14_5: the cup stage scalar deleting destructor ----------

/// The scalar deleting destructor of a cup knockout stage: `Cup_Stage::~Cup_Stage` (0x523300),
/// called through its detour, and, with bit 0 of `flags`, the object (`delete_through`). Returns the object.
/// Runs in the career whenever a knockout stage is deleted (vtable 0x969468, slot 0).
/// Original: `Cup_Stage::scalar_deleting_destructor` (0x5230c0, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_cup_stage(this: usize, flags: u32) -> usize {
    covered!("rx14_5 cup stage scalar delete");
    delete_through(this, flags, 0x523300) // rx14_5: the stage
}

/// Comparers of the scalar deleting destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_cup_stage(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%)(this, flags)
}
