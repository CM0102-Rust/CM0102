use super::*;

/// `ItaSerC2a::update_staff_records_for_status(club, status, stage, _a4, game, _a6)` (virtual +0x44, announced by
/// `announce_team_status`): the Serie C2/A staff records. Returns 0. In the league (stage 0xff), by status: 5, the
/// title (Staff_Records::add_league_champions_record `exe::staff_records_add_league_champions_record`:
/// thiscall(records, club, comp `.competition()`), ret 8); 0, promotion (`FN_STAFF_RECORDS_ADD_PROMOTION_RECORD`:
/// thiscall(records, club, comp, 0x64), ret 0xc); 1, the play-off place
/// (`exe::staff_records_add_qualified_for_round_record`: thiscall(records, club, comp, 0xa0, 0x82, 0x28), ret 0x14); 3,
/// relegation (`FN_STAFF_RECORDS_ADD_RELEGATION_RECORD`: thiscall(records, club, comp), ret 8). In the promotion
/// play-off (stage 0), in the club's first row with status 1, 2 or 0xff: 1 (won), promotion with weight 0x32 and
/// `league_table_row::STATUS` = 0; 2 or 0xff (knocked out), the knock-out record
/// (`exe::staff_records_add_knocked_out_record`: thiscall(records, club, comp, game stage `fixture::ROUND_PREFIX`,
/// round `fixture::ROUND_NAME`, weight), ret 0x14; the weight = comp_util_scale_by_round
/// `exe::comp_util_scale_by_round`: cdecl(0x1e, 0x32, game round `fixture::ROUND_NUMBER`, 2)) and
/// `league_table_row::STATUS` = 0xfe. In the play-out (stage 1): 1 (stayed up), `league_table_row::STATUS` = 0xfe; 2 or
/// 0xff, relegation and `league_table_row::STATUS` = 3.
/// Original: `ItaSerC2a::update_staff_records_for_status` (0x654180, ita_ser_c2a.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, game: usize, _a6: u32) -> u32 {
    let this = League(this);
    let s = status as u8;
    let comp = this.competition().addr();
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
                covered!("cqk_5 c2a play-off place");
                exe::staff_records_add_qualified_for_round_record(eng_second_staff_records(), club, comp, 0xa0, 0x82, 0x28);
            }
            3 => {
                covered!("cqk_5 relegation in c2a");
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
    let n = this.club_count() as i32;
    let mut i: i32 = 0;
    while i < n {
        let row = this.table() + (i as usize) * league_table_row::SIZE;
        if read_usize(row) == club {
            if f == 0 {
                match s {
                    1 => {
                        let promotion: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_PROMOTION_RECORD);
                        promotion(eng_second_staff_records(), club, comp, 0x32);
                        write_u8(this.table() + (i as usize) * league_table_row::SIZE + league_table_row::STATUS, 0);
                        return 0;
                    }
                    2 | 0xff => {
                        let weight = exe::comp_util_scale_by_round(0x1e, 0x32, read_i16(game + fixture::ROUND_NUMBER) as u16 as u32, 2) as u32;
                        // simplification: the original pushes the match’s stage (+0x32) and round (+0x30) as words in
                        // registers with garbage in the high 16 bits; 0x86c000 only reads the words.
                        exe::staff_records_add_knocked_out_record(eng_second_staff_records(), club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, read_i16(game + fixture::ROUND_NAME) as u16 as u32, weight);
                        write_u8(this.table() + (i as usize) * league_table_row::SIZE + league_table_row::STATUS, 0xfe);
                        return 0;
                    }
                    _ => {}
                }
            } else {
                match s {
                    1 => {
                        write_u8(this.table() + (i as usize) * league_table_row::SIZE + league_table_row::STATUS, 0xfe);
                        return 0;
                    }
                    2 | 0xff => {
                        let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_RELEGATION_RECORD);
                        cair(eng_second_staff_records(), club, comp);
                        write_u8(this.table() + (i as usize) * league_table_row::SIZE + league_table_row::STATUS, 3);
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
