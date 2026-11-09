use super::*;

/// the French lower division (background): up to 40 clubs, those of the National and up to 20 French clubs of the
/// generic division with no reserve competition, drawn into 2 groups of 20
/// Original: `FraLower::FraLower` (0x5bd600, fra_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x5bd600 } (thiscall, ret 4, fra_lower.cpp, 578 bytes): `FraLower::FraLower(ano)`, a divisão de baixo francesa
/// (competição de id [0x9cf548]): até 40 clubes, os do National ([0x9cf5d4]) e até 20 franceses ([0x9cf300]) da divisão
/// genérica sem competição de reserva, embaralhados em 2 grupos de 20. Fonte 0x9ccbd0, linhas 0x40, 0x62, 0x69.
pub(crate) unsafe extern "thiscall" fn construct_fra_lower(this: usize, year: u32) -> usize {
    construct_group_league(League(this), year, &FRENCH_LOWER_DIVISION)
}

/// the same for the lower division
/// Original: `FraLower::~FraLower` (0x5bd870, fra_lower.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bd870 } (thiscall, ret 0, fra_lower.cpp): `FraLower::~FraLower()`.
pub(crate) unsafe extern "thiscall" fn destroy_fra_lower(this: usize) {
    crate::comparison::branch("cqn_4 lower division destructor");
    destroy_group_league(League(this), 0x96ab78)
}

/// the French lower division has 20 clubs (chosen by the constructor); those in the National get the mark
/// Original: `FraLower::create_table` (0x5be230, fra_lower.cpp, thiscall, ret 4).
///
/// [pt] { 0x5be230 } (thiscall, ret 4, fra_lower.cpp): `FraLower::create_table(clubes)`: a divisão de baixo francesa com
/// 20 clubes da lista (o construtor 0x5bd600 escolhe-os), marcando os que estão no National ([0x9cf5d4]). Fonte
/// 0x9ccbd0, linha 0x175.
pub(crate) unsafe extern "thiscall" fn fra_lower_create_table(this: usize, clubs: usize) -> u32 {
    create_french_table(League(this), clubs, 0x14, 0x9ccbd0, 0x175, &[KEY_CLUB_COMP_FRENCH_NATIONAL])
}

/// the lower division (background) schedule: the same 38 dates as the second division, with the TV slot also in the
/// last two rounds
/// Original: `FraLower::create_schedule` (0x5bd920, fra_lower.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bd920 } (thiscall, ret 0x10, fra_lower.cpp, 2.311 bytes, método +0x3c): `FraLower::create_schedule`: a divisão de
/// baixo, 38 rodadas (`LOWER_DIVISION_ROUNDS`). Fonte 0x9ccbd0, linha 0xa9.
pub(crate) unsafe extern "thiscall" fn fra_lower_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_league_schedule(League(this), stage, rounds, extra, league, &LOWER_DIVISION_LEAGUE)
}
