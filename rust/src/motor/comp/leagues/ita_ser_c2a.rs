use super::*;

/// `ItaSerC2a::update_staff_records_for_status(club, status, stage, _, match, _)` (virtual +0x44, announced by
/// `announce_team_status`): the Serie C2/A staff records. Returns 0.
///
/// In the league (stage 0xff), by status: 5, the title (Staff_Records::add_league_champions_record `add_league_champions_record`:
/// thiscall(records, club, comp +4), ret 8); 0, promotion (`add_promotion_record`: thiscall(records, club, comp, 0x64), ret 0xc); 1,
/// the play-off place (`add_qualified_for_round_record`: thiscall(records, club, comp, 0xa0, 0x82, 0x28), ret 0x14); 3, relegation (`add_relegation_record`:
/// thiscall(records, club, comp), ret 8).
///
/// In the promotion play-off (stage 0), in the club's first row with status 1, 2 or 0xff: 1 (won), promotion with
/// weight 0x32 and +0x37 = 0; 2 or 0xff (knocked out), the knock-out record (`add_knocked_out_record`: thiscall(records, club, comp,
/// match stage +0x32, round +0x30, weight), ret 0x14; the weight = comp_util_scale_by_round `comp_util_scale_by_round`: cdecl(0x1e, 0x32,
/// match round +0x34, 2)) and +0x37 = 0xfe.
///
/// In the play-out (stage 1): 1 (stayed up), +0x37 = 0xfe; 2 or 0xff, relegation and +0x37 = 3.
/// Original: `ItaSerC2a::update_staff_records_for_status` (0x654180, ita_ser_c2a.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x654180 } (thiscall, ret 0x18, ita_ser_c2a.cpp): `ItaSerC2a::update_staff_records_for_status(clube, situação,
/// fase, _, jogo, _)`, a virtual +0x44 (anunciada pela 0x686670). Devolve 0. Na liga (fase 0xff), pela situação: 5,
/// o título (Staff_Records::add_league_champions_record 0x868c50: thiscall(registros, clube, comp +4), ret 8); 0, o
/// acesso (0x869480: thiscall(registros, clube, comp, 0x64), ret 0xc); 1, a vaga no play-off (0x86bdd0:
/// thiscall(registros, clube, comp, 0xa0, 0x82, 0x28), ret 0x14); 3, o rebaixamento (0x86a1c0: thiscall(registros,
/// clube, comp), ret 8). No play-off de acesso (fase 0), na primeira linha do clube com situação 1, 2 ou 0xff: 1
/// (venceu), o acesso com peso 0x32 e +0x37 = 0; 2 ou 0xff (caiu), a eliminação (0x86c000: thiscall(registros, clube,
/// comp, fase do jogo +0x32, rodada +0x30, peso), ret 0x14; o peso = comp_util_scale_by_round 0x4bf850: cdecl(0x1e,
/// 0x32, rodada do jogo +0x34, 2)) e +0x37 = 0xfe. No play-out (fase 1): 1 (salvou-se), +0x37 = 0xfe; 2 ou 0xff,
/// o rebaixamento e +0x37 = 3.
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, game: usize, _a6: u32) -> u32 {
    let s = status as u8;
    let comp = read_usize(this + 4);
    let f = stage as u8;
    if f == 0xff {
        match s {
            5 => {
                exe::staff_records_add_league_champions_record(eng_second_staff_records(), club, comp);
            }
            0 => {
                let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                promotion(eng_second_staff_records(), club, comp, 0x64);
            }
            1 => {
                crate::comparison::branch("cqk_5 c2a play-off place");
                exe::staff_records_add_qualified_for_round_record(eng_second_staff_records(), club, comp, 0xa0, 0x82, 0x28);
            }
            3 => {
                crate::comparison::branch("cqk_5 relegation in c2a");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                cair(eng_second_staff_records(), club, comp); // cqk5: relegation by the C2/A table
            }
            _ => {}
        }
        return 0;
    }
    if f > 1 {
        return 0;
    }
    let n = read_i16(this + league::CLUB_COUNT) as i32;
    let mut i: i32 = 0;
    while i < n {
        let row = read_usize(this + league::TABLE) + (i as usize) * 0x3b;
        if read_usize(row) == club {
            if f == 0 {
                match s {
                    1 => {
                        let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                        promotion(eng_second_staff_records(), club, comp, 0x32);
                        *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                        return 0;
                    }
                    2 | 0xff => {
                        let weight = exe::comp_util_scale_by_round(0x1e, 0x32, read_i16(game + 0x34) as u16 as u32, 2);
                        // simplification: the original pushes the match’s stage (+0x32) and round (+0x30) as words in
                        // registers with garbage in the high 16 bits; 0x86c000 only reads the words.
                        exe::staff_records_add_knocked_out_record(eng_second_staff_records(), club, comp, read_i16(game + 0x32) as u16 as u32, read_i16(game + 0x30) as u16 as u32, weight);
                        *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    _ => {}
                }
            } else {
                match s {
                    1 => {
                        *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    2 | 0xff => {
                        let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                        cair(eng_second_staff_records(), club, comp);
                        *((read_usize(this + league::TABLE) + (i as usize) * 0x3b + 0x37) as *mut u8) = 3;
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    0
}
