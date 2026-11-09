use super::*;

/// rule: the Confederations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds on 29 July, 1 and
/// 4 August, with fixed matches; the knockout (stage 1) in single matches: semi-final on 6/8, third place on 8/8 and
/// final on 9/8 (venue 4); category 10; tie-break 0x383 in the semi-final and 0x83 after
/// Original: `FifaConfed::create_schedule` (0x58b560, fifa_confed.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x58b560 } (thiscall, ret 0x10, fifa_confed.cpp; a virtual +0x3c da Copa das Confederações): `FifaConfed::
/// create_schedule(fase, &rodadas, &extra, &é_liga)`: a fase de `confed_cup_phase`.
pub(crate) unsafe extern "thiscall" fn fifa_confed_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), confed_cup_phase(stage), rounds, extra, league, 0x9c7474)
}
