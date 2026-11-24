// ---------- cx03_15: the progress codes of the three South American continental cups (con_merc_cup.cpp, conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// A group progress code 1 to 4 turned into its place in the whole cup (`to[code − 1]`); any other code as it comes.
pub(crate) fn remap_group_code(code: u8, to: [u8; 4]) -> u8 {
    if (1..=4).contains(&code) { to[code as usize - 1] } else { code }
}

/// The body of the South American cups' `set_club_progress(club, stage, current, best, worst)` (virtual
/// `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14): in the groups (stages, a signed byte, below `knockout`) each code goes
/// through its table (`remap_group_code`); in the knockout as they come; then `store_club_progress`.
#[allow(clippy::too_many_arguments)]
unsafe fn set_group_cup_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, knockout: i8,
    to: ([u8; 4], [u8; 4], [u8; 4]), branch: &'static str) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < knockout {
        covered!(branch);
        current = remap_group_code(current, to.0);
        best = remap_group_code(best, to.1);
        worst = remap_group_code(worst, to.2);
    }
    store_club_progress(this, club, current, best, worst);
}

/// rule: in the Copa Merconorte groups (stages below 3) the group places 1 to 4 become the cup places 1, 5, 9 and 13 for
/// the current and best codes, and 3, 5, 9 and 13 for the worst; from the knockout on the codes stay as they come.
/// Original: `ConMercCup::set_club_progress` (0x4c2e40, con_merc_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn con_merc_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, 3, ([1, 5, 9, 0xd], [1, 5, 9, 0xd], [3, 5, 9, 0xd]), "cx03_15 merconorte group progress")
}

/// rule: in the Copa Libertadores groups (stages below 7) places 1 to 3 become the cup place 17 and place 4 the place 25
/// for the current and best codes; for the worst places 1 and 2 give 9, place 3 gives 17 and place 4 gives 25; from the
/// knockout on the codes stay as they come.
/// Original: `ConmebolLiber::set_club_progress` (0x4c4610, conmebol_liber.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, 7, ([0x11, 0x11, 0x11, 0x19], [0x11, 0x11, 0x11, 0x19], [9, 9, 0x11, 0x19]), "cx03_15 libertadores group progress")
}

/// rule: in the Copa Mercosur groups (stages below 4) the best code gives 1 for places 1 and 2, 11 for place 3 and 16
/// for place 4; the current and worst codes give 5 for place 1, 11 for places 2 and 3 and 16 for place 4; from the
/// knockout on the codes stay as they come.
/// Original: `ConmebolMerc::set_club_progress` (0x4c5ee0, conmebol_merc.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_group_cup_club_progress(this, club, stage, current, best, worst, 4, ([5, 0xb, 0xb, 0x10], [1, 1, 0xb, 0x10], [5, 0xb, 0xb, 0x10]), "cx03_15 mercosur group progress")
}

pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("con_merc_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || con_merc_cup_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_club_progress("conmebol_liber_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || conmebol_liber_set_club_progress(this, club, stage, current, best, worst));
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_club_progress("conmebol_merc_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || conmebol_merc_set_club_progress(this, club, stage, current, best, worst));
}
