// ---------- sx10_6: the default formation (formation.cpp) ----------

/// The name of a formation nothing has named yet: "Uninitialised".
const FORMATION_UNINITIALISED_NAME: usize = 0x9cc3a8;
/// The default 4-4-2 of slots 0..10 (general position codes): GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL, F CR;
/// M L.
const FORMATION_DEFAULT_POSITIONS: [u32; 11] = [0x1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x810, 0x410, 0x140, 0x440, 0x90];
/// The team instructions of a new formation.
const FORMATION_DEFAULT_TEAM_INSTRUCTIONS: u32 = 0x22a91;
/// The instructions of each player of a new formation (the low dword 0x15552221, the high dword 0xa).
const FORMATION_DEFAULT_PLAYER_INSTRUCTIONS: u64 = 0x0000_000a_1555_2221;
/// The marking type of each slot of a new formation.
const FORMATION_DEFAULT_MARKING_TYPE: u8 = 0x12;
/// The dword after `formation_fields::HAS_PASSWORD`: 1 once the formation was read from or written to a file
/// [inference: "saved"].
const FORMATION_SAVED: usize = 0x1887;

/// The common tail of both formation constructors: the default player instructions, marking types 0x12, no designated
/// people (-1), the position lookup rebuilt, then changed = 1, no password and not saved.
pub(crate) unsafe fn formation_reset_instructions(this: usize) {
    for k in 0..11usize {
        write_u64(this + formation_fields::PLAYER_INSTRUCTIONS + 8 * k, FORMATION_DEFAULT_PLAYER_INSTRUCTIONS); // sx10_6: default player instructions
        write_u8(this + formation_fields::MARKING_TYPES + k, FORMATION_DEFAULT_MARKING_TYPE);
        write_i32(this + formation_fields::DESIGNATED_PEOPLE + 4 * k, -1); // sx10_6: nobody designated
    }
    exe::formation_build_position_lookup(this);
    write_u32(this + formation_fields::CHANGED_FLAG, 1);
    write_u32(this + formation_fields::HAS_PASSWORD, 0);
    write_u32(this + FORMATION_SAVED, 0);
}

/// Builds an empty formation: a 4-4-2 with the default instructions.
///
/// `Formation::Formation()`: the whole record (`formation_fields::SIZE` bytes) zeroed; the name "Uninitialised"; the
/// password (`formation_fields::PASSWORD`) and creator (`formation_fields::CREATOR`) empty and the display order
/// (`formation_fields::DISPLAY_ORDER`, 11 bytes) zeroed; the 11 slots get the default 4-4-2 through
/// `Formation::set_slot_position` (general position, not the extra one: GK; D R, D L, D CL, D CR; M CL, M R, M CR; F CL,
/// F CR; M L), which also sets each slot's pitch coordinates from its positions; the team instructions
/// (`formation_fields::FLAGS`) 0x22a91; each player's instructions (`formation_fields::PLAYER_INSTRUCTIONS`) 0x15552221
/// with 0xa in the high dword; marking types 0x12; no designated people (-1); the position lookup rebuilt
/// (`Formation::build_position_lookup`); changed = 1, no password, not saved. Returns `this`. Runs for the two
/// formations of each side when a match is set up (match_day.cpp, a vector constructor), for each tactics record and for
/// the human manager's own formation.
/// Original: `Formation::Formation` (0x5b27c0, formation.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn formation_new(this: usize) -> usize {
    core::ptr::write_bytes(this as *mut u8, 0, formation_fields::SIZE);
    copy_c_string(this, FORMATION_UNINITIALISED_NAME);
    write_u8(this + formation_fields::PASSWORD, 0);
    write_u8(this + formation_fields::CREATOR, 0);
    core::ptr::write_bytes((this + formation_fields::DISPLAY_ORDER) as *mut u8, 0, 11);
    for (slot, position) in FORMATION_DEFAULT_POSITIONS.iter().enumerate() {
        exe::formation_set_slot_position(this, slot as u32, *position, 0); // sx10_6: the default 4-4-2
    }
    write_u32(this + formation_fields::FLAGS, FORMATION_DEFAULT_TEAM_INSTRUCTIONS); // sx10_6: default team instructions
    formation_reset_instructions(this);
    covered!("sx10_6 default formation");
    this
}

/// Comparer for `formation_new`: side by side over the whole record.
pub(crate) unsafe extern "thiscall" fn compare_formation_new(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let reg = [(this, formation_fields::SIZE)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || formation_new(this));
    SCOREBOARD.record(same && r == n, || format!("Formation::Formation({:#x}) {:#x}/{:#x} {:?}", this, r, n, first_differences(&mo, &mr)));
    r
}
