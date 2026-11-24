// ---------- cx02_24: the Dutch Cup first group table and season honours (hol_cup.cpp) ----------

/// The Dutch Cup's own group (the cup is the league of its first group): four clubs, the first four entrants of the
/// pool (`welsh_cup_group_table` with stride 1; out of memory the box at line 0x196 of hol_cup.cpp and 0), each taken
/// out of the pool, then the group draw news (checked instruction by instruction against WelPrmCup::create_group_table:
/// only the file, the line and the stride change). Returns 1.
/// Original: `HolCup::create_group_table` (0x5f3d70, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_create_group_table(this: usize) -> u32 {
    welsh_cup_group_table(League(this), 4, 1, 0x9cedd4, 0x196, 0, "cx02_24 dutch cup first group")
}

/// The Dutch Cup's season honours are those of its knockout stage (stage slot 19): the call goes on to that stage's
/// own `record_season_honours` (virtual `comp_vtable::RECORD_SEASON_HONOURS`, a tail jump), whose eax it returns.
/// Original: `HolCup::record_season_honours` (0x5f3f00, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_record_season_honours(this: usize) -> u32 {
    covered!("cx02_24 dutch cup honours");
    let knockout = read_usize(League(this).stages() + 4 * 0x13); // the knockout stage slot
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(knockout) + comp_vtable::RECORD_SEASON_HONOURS));
    f(knockout)
}

/// Comparer for `hol_cup_create_group_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_welsh_cup_group_table("hol_cup_create_group_table", League(this), 4, 1, || o(this))
}

/// Comparer for `hol_cup_record_season_honours` (original only: the honours write the histories).
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this)
}
