// ---------- rx15_2: the human manager's tactic slot constructor (human_manager.cpp) ----------

/// Builds one tactic slot of a human manager record: only the Formation block at its start (`formation_new`: the
/// default 4-4-2 named "Uninitialised"); the rest of the slot is left as it is. Returns the slot.
/// Original: `HUMAN_TACTIC_SLOT::HUMAN_TACTIC_SLOT` (0x5fb720, human_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_human_tactic_slot(this: usize) -> usize {
    covered!("rx15_2 human tactic slot");
    formation_new(this); // rx15_2: the slot's formation
    this
}

/// Comparer for `construct_human_tactic_slot`: side by side over the whole slot.
pub(crate) unsafe extern "thiscall" fn compare_construct_human_tactic_slot(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, HUMAN_TACTIC_SLOT_SIZE as usize)], || o(this), || construct_human_tactic_slot(this));
    SCOREBOARD.record(same && r == n, || format!("HUMAN_TACTIC_SLOT::HUMAN_TACTIC_SLOT({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}
