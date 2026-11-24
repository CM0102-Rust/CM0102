// ---------- batch %%LEVA%%: the training schedule record's header (training_screens.cpp) ----------

/// Rule: a training schedule record's header (the 0x11 bytes at +0x18 of the schedule) gets its defaults.
///
/// The dword 0x1011 at +0, the byte 0xff at +4, the dwords 0 at +5 and +9 and the dword 0x21 at +0xd (meanings not
/// proven); returns `this`. Called by the schedule constructor (`TRAINING_SCHEDULE::TRAINING_SCHEDULE` 0x8b4e10).
/// Original: `training_record_init_header` (0x8bba10, training_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn training_record_init_header(this: usize) -> usize {
    covered!("tel45_2 schedule header defaults");
    write_u32(this, 0x1011);
    write_u8(this + 4, 0xff);
    write_u32(this + 5, 0);
    write_u32(this + 9, 0);
    write_u32(this + 0xd, 0x21); // tel45_2: the last default
    this
}

/// Comparer for `training_record_init_header`: side by side over the 0x11 bytes and the result.
pub(crate) unsafe extern "thiscall" fn compare_training_record_init_header(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x11)], || o(this), || training_record_init_header(this));
    SCOREBOARD.record(same && ro == rr, || format!("training_record_init_header: returned {:#x} x {:#x}, differences {:?}", ro, rr, first_differences(&mo, &mr)));
    ro
}
