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
    let game = Fixture(game);
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
                        let weight = exe::comp_util_scale_by_round(0x1e, 0x32, game.round_number() as u16 as u32, 2) as u32;
                        // simplification: the original pushes the match’s stage (+0x32) and round (+0x30) as words in
                        // registers with garbage in the high 16 bits; 0x86c000 only reads the words.
                        exe::staff_records_add_knocked_out_record(eng_second_staff_records(), club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
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

/// Writes the news text of a club's fate in a Serie C1 or C2 group (the five groups share it at virtual +0x48).
///
/// With a news body (`news`), the base text (`exe::comp_build_fate_news_text`) with a7 = 0. Without one: outcome 1 in
/// stage 0 (the promotion play-off won), "{club} win promotion"; outcome 0xff in stage 1 (the relegation play-out lost),
/// "{club} relegated"; both with the club's short name and gender byte (`group_qualify_text`: the shared buffer, the
/// Text and the club's link 0x7d5). Any other outcome or stage: the base text with a7 = 0 and no news. The outcome and
/// the stage are compared as bytes; the base text gets them as the caller passed them.
/// Original: `ItaSerC2a::build_fate_news_text` (0x654390, ita_ser_c2a.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text);
    }
    covered!("cx11_10 serie c own fate text");
    match (outcome as u8, stage as u8) {
        (1, 0) => group_qualify_text(club_id, WIN_PROMOTION_TEXT, text), // cx11_10: the play-off winner goes up
        (0xff, 1) => group_qualify_text(club_id, RELEGATED_TEXT, text),
        _ => exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, 0, text),
    }
}

/// Comparer for `ita_ser_c_build_fate_news_text`: the two text-only paths side by side (`compare_text_writer`; with the
/// CM_TEXTO_NOTICIAS mode); the base text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_ITA_SER_C_BUILD_FATE_NEWS_TEXT);
    let own = matches!((outcome as u8, stage as u8), (1, 0) | (0xff, 1));
    if news != 0 || text == 0 || !own {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| ita_ser_c_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("ita_ser_c_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// The fate text of a club that goes through its group (`format`: "{club} qualify for the semi final/final"), with the
/// club's short name (gender byte +0x52) and link (0x7d5).
pub(crate) unsafe fn group_qualify_text(club_id: u32, format: usize, text: usize) {
    covered!("cx04_5 group qualify text");
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// "{club} win promotion" (langlib template with the club's short name and gender).
pub(crate) const WIN_PROMOTION_TEXT: usize = 0x989c18;

/// "{club} relegated".
pub(crate) const RELEGATED_TEXT: usize = 0x987564;
