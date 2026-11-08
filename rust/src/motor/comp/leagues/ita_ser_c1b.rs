use super::*;

/// the same for the other Serie C1 group (the same table)
/// Original: `ItaSerC1b::create_schedule` (0x64ef30, ita_ser_c1b.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x64ef30 } (thiscall, ret 0x10, ita_ser_c1b.cpp, 2.279 bytes, método +0x3c): `ItaSerC1b::create_schedule`: a
/// chave B, o mesmo código. Fonte 0x9ea558, linhas 0xac, 0x156, 0x190.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    create_c1_schedule(this, fase, rodadas, extra, liga, &C1BracketVariant { arquivo: 0x9ea558, linhas: [0xac, 0x156, 0x190] })
}

/// the same rule in C1/B
/// Original: `ItaSerC1b::set_club_progress` (0x64ff40, ita_ser_c1b.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x64ff40 } (thiscall, ret 0x14, ita_ser_c1b.cpp): `ItaSerC1b::set_club_progress(...)`, igual à da C1/A.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_c1_club_progress(this, club, stage as u8, current as u8, best as u8, worst as u8)
}

/// the 2001/02 C1/B, with the same rule
/// Original: `ItaSerC1b::set_up_fixed_fixtures` (0x650100, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x650100 } (thiscall, ret 0, ita_ser_c1b.cpp, 11.650 bytes): `ItaSerC1b::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/B 2001/02. Fonte 0x9ea558, linha 0x404.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_up_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures_body(this, &SERIE_C1B_FIXTURES, 0x9ea558, 0x404)
}

/// C1/B, as C1/A
/// Original: `ItaSerC1b::ItaSerC1b` (0x64ec20, ita_ser_c1b.cpp, thiscall, ret 8).
///
/// [pt] { 0x64ec20 } (thiscall, ret 8, ita_ser_c1b.cpp): `ItaSerC1b::ItaSerC1b(ano, registro)`: a C1/B, como a C1/A.
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c1b(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(this, year, record, &SERIE_C1B_DIVISION)
}

/// `ItaSerC1b::set_up_rules()`: Serie C1/B (18 clubs), set up like C1/A.
///
/// Above it the id of [0x9cf578], and below it C2/A ([0x9cf580]) when the nation has its lower divisions loaded (bit 4
/// of the nation's +0x11c [inference]), otherwise −1.
/// Original: `ItaSerC1b::set_up_rules` (0x64f920, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64f920 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::set_up_rules()`. A C1/B: como a C1/A; acima o id de
/// [0x9cf578] e abaixo a C2/A ([0x9cf580]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da
/// nação [inferência]), senão −1.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_6 regras da c1b");
    ita_ser_a_apply_league_rules(this, &serie_c1b_rules(this))
}

/// Serie C1/B has 18 clubs; no prize fund and no TV money
/// Original: `ItaSerC1b::create_table` (0x64f820, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64f820 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_table()`: 18 clubes ([0x9cf57c]); como a
/// C1/A. Fonte 0x9ea558, linha 0x1be.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_create_table(this: usize) -> u32 {
    ita_ser_a_fill_league_table(this, 0x12, 0x9ea558, 0x1be, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B).unwrap_or(false) as u32
}

/// the same as C1/A
/// Original: `ItaSerC1b::advance_stage` (0x64fba0, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64fba0 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::advance_stage()`; pela 0x64fc30. Fonte 0x9ea558, linha
/// 0x255.
pub(crate) unsafe extern "thiscall" fn ita_ser_c1b_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_passar_de_fase(this, 0x9ea558, 0x255, 0x64fc30)
}

/// the same rule in C1/B
/// Original: `ItaSerC1b::create_play_off_stages` (0x64fc30, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x64fc30 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_play_off_stages()`, a mesma regra na C1/B.
/// Fonte 0x9ea558, linhas 0x281, 0x295, 0x2b8; no fim solta o gabarito antes da lista.
pub(crate) unsafe extern "thiscall" fn create_play_off_stages(this: usize) {
    create_end_of_league_stages(this, 0x9ea558, [0x281, 0x295, 0x2b8], false)
}
