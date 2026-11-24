// ---------- cx17_6: the World Cup qualification status and its change (world_cup.cpp) ----------

/// The World Cup status of a progress code (signed): 1 (the champion) −1, 2 (the runner-up) 150, below 5 (the semi-
/// finals) 130, below 9 (the quarter-finals) 120, below 17 (the round of 16) 20, otherwise (out in the groups) −4.
pub(crate) fn world_cup_status_of_code(code: i8) -> i16 {
    match code {
        1 => -1,
        2 => 0x96,
        c if c < 5 => 0x82,
        c if c < 9 => 0x78,
        c if c < 0x11 => 0x14,
        _ => -4, // cx17_6: out in the groups
    }
}

/// rule: a nation's World Cup status by its progress code: −1 for the champion, 150 for the runner-up, 130 for the
/// semi-finalists, 120 for the quarter-finalists, 20 for the round of 16 and −4 for those out in the groups
///
/// `qualifying_stat_byte`(club, 0x12) through `world_cup_status_of_code` (a word).
/// Original: `WorldCup::get_qualification_status` (0x92fae0, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx17_6 world cup status");
    world_cup_status_of_code(qualifying_stat_byte(this, club, 0x12))
}

/// The change of a nation's World Cup status, by the band of its current code (rows: below 3, below 9, below 17, the
/// rest) and of its previous one (columns: below 5, below 9, below 17, the rest).
const WORLD_CUP_STATUS_CHANGE: [[i8; 4]; 4] = [
    [0, -1, -2, -3],
    [2, 0, -1, -2],
    [3, 2, 0, -1],
    [3, 3, 2, 0], // cx17_6: still out in the groups
];

/// rule: the change of a nation's World Cup status between its previous progress code (stat 0x13) and its current one
/// (stat 0x12): 0 within the same band, positive when it goes down the bands (worse), negative when it goes up, by the
/// number of bands crossed
///
/// The bands of the current code are below 3, below 9, below 17 and the rest; those of the previous one below 5, below
/// 9, below 17 and the rest (`WORLD_CUP_STATUS_CHANGE`; signed bytes). Defect of the original [inference]: the first
/// band of the current code ends at 3 where the previous one's ends at 5, so a nation whose code stays 3 or 4 (a semi-
/// finalist) shows a change of 2. The byte in al.
/// Original: `WorldCup::get_qualification_status_change` (0x92fb40, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx17_6 world cup status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    let band = |code: i8, first: i8| match code {
        c if c < first => 0,
        c if c < 9 => 1,
        c if c < 0x11 => 2,
        _ => 3,
    };
    WORLD_CUP_STATUS_CHANGE[band(now, 3)][band(before, 5)]
}

/// Comparers: the two only read; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I0%%)(this, club);
    let b = world_cup_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("world_cup_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I1%%)(this, club);
    let b = world_cup_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("world_cup_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}
