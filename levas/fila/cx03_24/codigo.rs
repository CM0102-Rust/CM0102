// ---------- cx03_24: the Swedish Cup's fate news and staff records (swe_cup.cpp) ----------

/// rule: the Swedish Cup news of a club's fate: with a news object, the cup tie result; without one, a club winning
/// (outcome 1) a preliminary section (stages below 7) gets "{club} through to {competition} Third Round." (0xa8b4d4,
/// the competition's short name); otherwise the generic competition news.
///
/// `build_fate_news_text(club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)` (10 dwords, ret 0x28). With `news`:
/// `exe::comp_util_write_cup_tie_result_news`(club_id, a5, a8, outcome, a6, text) (cdecl). Without: stage (a signed
/// byte) below 7 and outcome (a byte) 1, `european_cup_fate_text` with the format 0xa8b4d4,
/// `competition::SHORT_GENDER_NAME` and `competition::SHORT_NAME` of the competition `+4`, the competition linked;
/// anything else Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, thiscall, ret 0x28) with a7 and `news`
/// cleared.
/// Original: `SweCup::build_fate_news_text` (0x88d910, swe_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn swe_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text);
    }
    if (stage as u8 as i8) < 7 && outcome as u8 == 1 {
        covered!("cx03_24 swedish cup through to the third round");
        let comp = Competition(read_usize(this + 4));
        return european_cup_fate_text(text, club_id, 0xa8b4d4, comp.short_gender_name(), comp.addr() + competition::SHORT_NAME, comp.addr());
    }
    exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, 0, text);
}

/// rule: the Swedish Cup's staff records: in the preliminary sections (stages below 7) a club going out (0xff) or
/// beaten (2) gets the knocked-out record, a club going through (0) the record of the next round, a section winner (1)
/// the record of qualifying for the main draw (round 0x1e, weight 0x5a); the weights from 1 to 80 by round (of 2); in
/// the main draw (stage 7) the winner and the runner-up get the final records.
///
/// `update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (virtual
/// `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS`, ret 0x18); returns 0. Stage (a signed byte) below 7, weight =
/// `scale_by_round`(1, 0x50, the fixture's `round_number()`, 2): 0xff or 2, the knocked-out record
/// (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the qualified record with
/// `round_prefix()` and the code (+7) of the round after the fixture's in `cup::ROUND_TABLE`; 1, the qualified record with
/// 0, 0x1e and 0x5a. Stage 7: 1, the winners record (0x86a800); 2, the runners-up record (0x86b0b0)
/// (`cup_record_final_result`). Anything else: nothing.
/// Original: `SweCup::update_staff_records_for_status` (0x88e2f0, swe_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn swe_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 7 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cx03_24 swedish cup section round won");
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                let next = read_u16(Cup(this).round_table() + (g.round_number() as i32 as usize + 1) * cup_round::SIZE + cup_round::ROUND_CODE) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, next, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x1e, 0x5a), // through to the main draw
            _ => {}
        }
    } else if f == 7 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

/// Comparer for `swe_cup_build_fate_news_text`: the paths that only write the text (the tie result 0x4b0b80 and the
/// "through" text) side by side with `compare_text_writer`; the generic competition news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    let text_only = text != 0 && (news != 0 || ((stage as u8 as i8) < 7 && outcome as u8 == 1));
    if !text_only {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| swe_cup_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("swe_cup_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// Comparer for `swe_cup_update_staff_records_for_status`: only the original runs (the records go to the history);
/// the plan: it returns 0.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("swe_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}
