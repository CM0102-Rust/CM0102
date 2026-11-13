// ---------- mwb_21: the Welsh Premier Cup (wel_prm_cup.cpp): the staff records and the second group ----------

/// rule: in the two Welsh Premier Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 1) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 3 rounds), and the winner
/// and the runner-up their final records (`welsh_cup_staff_records`, mwb_19)
/// Original: `WelPrmCup::update_staff_records_for_status` (0x928af0, wel_prm_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_21 welsh premier cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 1, 3) // mwb21: the Premier Cup knockout over 3 rounds
}

/// Comparer for `wel_prm_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_prm_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// rule: the second Welsh Premier Cup group (stage 0): four clubs, pool entries 1, 3, 5 and 7, in a league stage of
/// their own with tie-break list (2, 0, 0, 0); then the group draw news; out of memory the box of line 0x247, without
/// the stage that of 0x25d (and 0) (`welsh_cup_group_stage`, mwb_19)
/// Original: `WelPrmCup::create_group_stage` (0x928450, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 4, 2, 2, 0xad572c, (0x247, 0x25d), "mwb_21 welsh premier cup group")
}

/// Comparer for `wel_prm_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    compare_welsh_cup_group_stage("wel_prm_cup_create_group_stage", League(this), g, 4, 2, || o(this, g))
}
