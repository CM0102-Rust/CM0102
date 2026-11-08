use super::*;

/// like the Second one, with the 7th, 4th, 6th and 5th (ties 7th v 4th and 6th v 5th; lines 0x2be and 0x2d5)
/// Original: `EngThird::create_play_off_stage` (0x5783c0, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5783c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_play_off_stage()` (de 0x578330). As chaves: o 7º
/// contra o 4º e o 6º contra o 5º (linhas 6, 3, 5 e 4); +0x3a = o extra da virtual. Fonte 0x9c4424, linhas 0x2be e
/// 0x2d5.
pub(crate) unsafe extern "thiscall" fn eng_third_create_play_off_stage(this: usize) {
    create_playoff_stage(this, [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third
}

/// the Third Division: discipline 9, minimum ground of 6,000 with 1,000 seated, the league and the play-off
/// Original: `EngThird::EngThird` (0x576dd0, eng_third.cpp, thiscall, ret 8).
///
/// [pt] { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
pub(crate) unsafe extern "thiscall" fn construct_eng_third(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(this, year, record, &THIRD_DIVISION)
}

/// the Third (24): 3 go up directly, 4th to 7th to the play-offs, the bottom club goes down to the Conference when the
/// nation has its lower divisions loaded [inference from nation bit 4] (otherwise there is no league below)
/// Original: `EngThird::set_up_rules` (0x5780c0, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
pub(crate) unsafe extern "thiscall" fn eng_third_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_4 regras da third");
    eng_prm_apply_league_rules(this, &third_division_rules(this))
}

/// the Third has 24 clubs; prize fund of 10,000 per place; TV pays 1,000,000 to each club (200,000 up to 1999)
/// Original: `EngThird::create_table` (0x577f70, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
pub(crate) unsafe extern "thiscall" fn eng_third_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(this, 0x18, 0x9c4424, 0x1fa, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, 0x2710, || eng_prm_pay_tv_rights(this, if is_after_1999() { 0xf4240 } else { 0x30d40 }, 0))
}

/// the Division Three schedule: the same 46 rounds as Division Two. The promotion play-offs (stage 0): two-legged
/// semi-finals (4 clubs, draw on 7 May, first leg on 12, 4 days apart) and a one-off final on 26 May [inference: venue
/// 4 is the neutral ground]
/// Original: `EngThird::create_schedule` (0x5770e0, eng_third.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5770e0 } (thiscall, ret 0x10, eng_third.cpp, 3.692 bytes, método +0x3c): `EngThird::create_schedule`: o mesmo
/// código da segunda (as mesmas rodadas), com a data da final dos play-offs um dia antes. Fonte 0x9c4424.
pub(crate) unsafe extern "thiscall" fn eng_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 46;
            *extra = 0;
            let t = alloc(0xbae);
            if t == 0 {
                show_error_at(0x9c4424, 0xb3);
                return 0;
            }
            crate::comparison::branch("cqp_4 calendario da terceira inglesa");
            let year = core::ptr::read_unaligned((this + league::YEAR) as *const u16) as u32;
            eng_second_set_christmas_rounds(t, year, &THIRD_DIVISION_CHRISTMAS_SCHEDULE, 0x9c4424);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 2;
            *extra = 0xa0;
            let t = alloc(0xd0);
            if t == 0 {
                show_error_at(0x9c4424, 0x1ba);
                return 0;
            }
            crate::comparison::branch("cqp_4 play-offs da terceira inglesa");
            let year = core::ptr::read_unaligned((this + league::YEAR) as *const i16) as i32 as u32;
            eng_cc_cup_write_cup_round(t, 0, &THIRD_DIVISION_PLAYOFF_SEMIFINAL, year);
            eng_cc_cup_write_cup_round(t, 1, &DIVISION_THREE_PLAYOFF_1, year);
            t
        }
        _ => 0,
    }
}

/// like the First one, with places 4 to 7 (4 and 7; in the play-offs, 4, 5 and 6)
/// Original: `EngThird::set_club_progress` (0x578660, eng_third.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x578660 } (thiscall, ret 0x14, eng_third.cpp): `EngThird::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da Third: as vagas do mata-mata são 4 a 7.
pub(crate) unsafe extern "thiscall" fn eng_third_set_club_progress(this: usize, clube: usize, stage: u32, current: u32, best: u32, worst: u32) {
    set_club_stage_status(this, clube, stage as u8, current as u8, best as u8, worst as u8, 4, 7)
}

/// after the Third Division league, the current stage goes up and, on the first, the promotion play-off is created
/// (0x5783c0)
/// Original: `EngThird::advance_stage` (0x578330, eng_third.cpp, thiscall, ret 0).
///
/// [pt] { 0x578330 } (thiscall, ret 0, eng_third.cpp): `EngThird::advance_stage()`; cria o play-off da Third Division pela
/// 0x5783c0 (já em Rust). Fonte 0x9c4424, linha 0x29c.
pub(crate) unsafe extern "thiscall" fn eng_third_advance_stage(this: usize) {
    ita_ser_c1a_advance_stage_cql5_passar_de_fase(this, 0x9c4424, 0x29c, 0x5783c0)
}
