use super::*;

/// `EuroChampQual::set_up_rules()` (0x57e8f0, thiscall, ret 0, euro_champ_qual.cpp; virtual +0x8c, from the constructor
/// in 2001-000; SEH frame for the local Date, no destructor): sets points, rounds, tie-breakers, qualification places
/// and the draw date, and builds the round table; returns 1.
///
/// 3 points for a win (+0xc2) and 1 for a draw (+0xc3), +0x4a = 3, two rounds (+0x3c = 2, word), match category 0xa
/// (+0xc4), +0x43 = 2, tie-breakers 1, 2, 4, 5 (+0xc5..+0xc8), one direct place (+0xbe = 1) and one play-off place
/// (+0xbf = 1), no relegation (+0xc0 = +0xc1 = 0), +0x49 = 7, +0xa7 = 0xffff (word).
///
/// The draw date (+0xb6, the day of the year) = Date(10, 2, year +0x40, 5) (0x549f70: push 5; push year; push 2; push
/// 0xa; lea ecx, local -> thiscall, ret 0x10: 10 March moved to the Friday [inference]); +0xb8 = 0 (word); the round
/// table from virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) at +0xba.
///
/// Returns 1.
///
/// [pt] { 0x57e8f0 } (thiscall, ret 0, euro_champ_qual.cpp; a virtual +0x8c, do construtor em 2001-000; quadro SEH pela Date
/// local, sem destrutor): `EuroChampQual::set_up_rules()`: 3 pontos pela vitória (+0xc2) e 1 pelo empate (+0xc3), +0x4a =
/// 3, dois turnos (+0x3c = 2, word), a categoria 0xa dos jogos (+0xc4), +0x43 = 2, os desempates 1, 2, 4, 5 (+0xc5..+0xc8),
/// uma vaga direta (+0xbe = 1) e uma de play-off (+0xbf = 1), sem rebaixamento (+0xc0 = +0xc1 = 0), +0x49 = 7, +0xa7 =
/// 0xffff (word); a data do sorteio (+0xb6, o dia do ano) = Date(10, 2, ano +0x40, 5) (0x549f70: push 5; push ano; push 2;
/// push 0xa; lea ecx, local -> thiscall, ret 0x10: 10 de março levado à sexta-feira [inferência]); +0xb8 = 0 (word); a
/// tabela das rodadas pela virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) em
/// +0xba. Devolve 1.
pub(crate) unsafe fn set_up_rules(this: usize) -> u32 {
    write_u8(this + league::WIN_POINTS, 3);
    write_u8(this + 0x4a, 3);
    core::ptr::write_unaligned((this + league::LEGS) as *mut i16, 2); // cqs8: dois turnos
    write_u8(this + league::DRAW_POINTS, 1);
    write_u8(this + league::MATCH_CATEGORY, 0xa);
    write_u8(this + league::COMP_TYPE, 2);
    for (k, v) in [1u8, 2, 4, 5].into_iter().enumerate() {
        write_u8(this + league::TIEBREAKS + k, v);
    }
    write_u8(this + league::PROMOTED_COUNT, 1);
    write_u8(this + league::PROMOTION_PLAYOFF_COUNT, 1);
    write_u8(this + league::RELEGATION_PLAYOFF_COUNT, 0);
    write_u8(this + league::RELEGATED_COUNT, 0);
    write_u8(this + 0x49, 7);
    core::ptr::write_unaligned((this + league::GAMES_PER_CLUB) as *mut u16, 0xffff);
    let construct: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut data = [0u32; 2];
    // simplification: the original pushes the year in ecx with the high 16 bits of the cup pointer; 0x549f70 reads the word
    construct(&mut data, 0xa, 2, read_i16(this + league::YEAR) as u16 as u32, 5); // cqs8: a data do sorteio
    core::ptr::write_unaligned((this + league::NEXT_DRAW_DAY) as *mut u16, data[0] as u16);
    core::ptr::write_unaligned((this + league::NEXT_DRAW_YEAR_OFFSET) as *mut u16, 0);
    crate::comparison::branch("cqs_8 regras da eurocopa");
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let t = rounds(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0);
    core::ptr::write_unaligned((this + league::ROUND_DATES) as *mut u32, t as u32);
    1
}

/// EuroChampQual::set_up_rules hook entry: sets the European Championship qualifying rules (points, two legs,
/// tiebreaks, one direct and one play-off place, draw date) and builds the round table (see set_up_rules); returns 1.
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_up_rules(this: usize) -> u32 {
    set_up_rules(this)
}

/// `EuroChampQual::get_group_team_count(g)` (0x57d9d0, thiscall, ret 4, euro_champ_qual.cpp): the number of teams in
/// group `g`, 6 or 5 depending on the format (+0xf6).
///
/// With format 1 (9 groups [inference]), groups 0, 2 and 5 have 6 national teams and the others 5; with format 2 (10
/// groups [inference]), groups 0, 2, 5 and 7 have 6 and the others 5; any other format shows the error box for line
/// 0x245 and returns 5. `g` is the argument's byte.
///
/// [pt] { 0x57d9d0 } (thiscall, ret 4, euro_champ_qual.cpp): `EuroChampQual::get_group_team_count(g)`: com o formato +0xf6 = 1
/// (9 grupos [inferência]), os grupos 0, 2 e 5 têm 6 seleções e os outros 5; com o formato 2 (10 grupos [inferência]), os
/// grupos 0, 2, 5 e 7 têm 6 e os outros 5; outro formato, a caixa da linha 0x245 e 5. `g` é o byte do argumento.
pub(crate) unsafe fn group_team_count(this: usize, g: u32) -> i16 {
    let g = g as u8;
    match *((this + 0xf6) as *const i8) {
        1 => if matches!(g, 0 | 2 | 5) { 6 } else { 5 }, // cqs8: os grupos de 6 no formato 1
        2 => if matches!(g, 0 | 2 | 5 | 7) { 6 } else { 5 },
        _ => {
            show_error_at(0x9c4674, 0x245);
            5
        }
    }
}

/// EuroChampQual::get_group_team_count(g): number of nations in qualifying group g of the European Championship (6 or 5
/// depending on the format at +0xf6; see group_team_count).
pub(crate) unsafe extern "thiscall" fn get_group_team_count(this: usize, g: u32) -> i16 {
    group_team_count(this, g)
}

/// `EuroChampQual::get_qualification_status(clube)` (`0x580b50`, thiscall, ret 4, `euro_champ_qual.cpp`; the virtual
/// giving a club's status): −2 if the low byte of the club's stat `0x12` is below 14, otherwise −3.
///
/// Reads the club's stat `0x12` from the cup statistics (Comp_Stats::get_club_stat `0x4a2e10`: push 0x12; push club;
/// mov ecx, [this+8] -> thiscall, ret 8, returns a float), truncated (_ftol `0x945640`): with the low byte below 14
/// (signed), −2; otherwise −3 (word).
///
/// [pt] { 0x580b50 } (thiscall, ret 4, euro_champ_qual.cpp; a virtual da situação de um clube): `EuroChampQual::
/// get_qualification_status(clube)`: a estatística 0x12 do clube nas estatísticas da copa (Comp_Stats::get_club_stat
/// 0x4a2e10: push 0x12; push clube; mov ecx, [this+8] -> thiscall, ret 8, devolve float), truncada (_ftol 0x945640): com o
/// byte de baixo menor que 14 (com sinal), −2; senão −3 (word).
pub(crate) unsafe fn qualification_status(this: usize, club: usize) -> i16 {
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(FN_COMP_STATS_GET_CLUB_STAT);
    let x = match_pl_ftol(estatistica(read_u32_as_usize(this + 8), club, 0x12));
    ((((x as u8 as i8) < 0xe) as i32) - 3) as i16 // cqs8: a situação pela estatística 0x12
}

/// EuroChampQual::get_qualification_status(club): the club's qualification status in the European Championship
/// qualifiers, from its competition stat 0x12 (Comp_Stats): -2 if that value (low byte, signed) is below 14, else -3.
pub(crate) unsafe extern "thiscall" fn get_qualification_status(this: usize, club: usize) -> i16 {
    qualification_status(this, club)
}

/// EuroChampQual::init_group_one_table(): sets up group 1 of the European Championship qualifiers (the cup itself):
/// stores the group size (+0x3e), allocates the league table (one 0x3b-byte row per team at +0xb1), sets the cached
/// match count (+0xa7) to (teams - 1) x rounds (+0x3c), fills one table row per national team from the list at +0xf7
/// and sends the group-draw news. Returns 1, or 0 out of memory.
///
/// [pt] { 0x57e7e0 } (thiscall, ret 0, euro_champ_qual.cpp; do sorteio, 2002-067): `EuroChampQual::init_group_one_table()`: o
/// grupo 1 é a própria copa: +0x3e = o tamanho do grupo −1 (0x57d9d0: push −1; mov ecx -> thiscall, ret 4; o byte 0xff
/// não é grupo de 6: 5); a tabela (0x944e46 de +0x3e × 0x3b em +0xb1; sem memória, a caixa da linha 0x2d2 e 0); +0xa7 =
/// (+0x3e − 1) × +0x3c (words: as rodadas, ida e volta); cada seleção da lista +0xf7 (na ordem) na linha i
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push o dword do byte i com lixo em cima (só o byte é lido);
/// mov ecx -> thiscall, ret 0x10); a notícia do grupo (0x684230, thiscall). Devolve 1.
pub(crate) unsafe extern "thiscall" fn init_group_one_table(this: usize) -> u32 {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_EUROCHAMPQUAL_GET_GROUP_TEAM_COUNT);
    let n = size(this, 0xffff_ffff);
    core::ptr::write_unaligned((this + league::CLUB_COUNT) as *mut i16, n);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alloc((n as i32 * 0x3b) as u32);
    core::ptr::write_unaligned((this + league::TABLE) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(0x9c4674, 0x2d2);
        return 0;
    }
    let n = read_i16(this + league::CLUB_COUNT);
    core::ptr::write_unaligned((this + league::GAMES_PER_CLUB) as *mut i16, n.wrapping_sub(1).wrapping_mul(read_i16(this + league::LEGS))); // cqs8: as rodadas do grupo 1
    crate::comparison::branch("cqs_8 grupo 1 da eurocopa");
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut i: i8 = 0;
    if n > 0 {
        loop {
            let c = read_u32_as_usize(read_u32_as_usize(this + 0xf7) + i as isize as usize * 4);
            iniciar(this, i as u8 as u32, c, 0, 0);
            i = i.wrapping_add(1);
            if (i as i16) >= read_i16(this + league::CLUB_COUNT) {
                break;
            }
        }
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    news(this);
    1
}

