// ---------- cx07_3: the Oceania Nations Cup results: a nation's fate text, the staff records and the progress codes (oceania_nations.cpp) ----------

/// The fate news text of a national-team tournament run in groups and a knockout (the Asian Cup's body, here with
/// the knockout stage and the qualification template as parameters).
///
/// In the group stages (stage below `knockout`, signed): through (outcome 6) with `news` the shared group news
/// (`exe::comp_util_write_group_qualify_news`(club, a4, competition, 6, news, text), cdecl), without it the template
/// `qualify` ("{nation} qualify for ...", `group_qualify_text` of cx04_5); out (0xfe) the group news with −2, whatever
/// `news` is; any other outcome, nothing. At the knockout stage: with `news` the tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6, text), cdecl), without it the Copa
/// América templates (`copa_america_fate_template`: win, runners-up, third, fourth, knocked out, any other "through to
/// next round") with the nation's and the competition's short names and links (`european_cup_fate_text`). After the
/// knockout, nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn nations_tournament_fate_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a8: u32, news: u32, text: usize,
        knockout: i8, qualify: usize) {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = League(this).competition().addr();
    if s < knockout {
        if d == 6 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            } else {
                group_qualify_text(club_id, qualify, text);
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text);
        }
        return;
    }
    if s != knockout {
        return;
    }
    if news != 0 {
        exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
    } else {
        european_cup_fate_text(text, club_id, copa_america_fate_template(d), read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
    }
}

/// Whether `nations_tournament_fate_text` only writes the text on this call (what its comparers compare side by
/// side): no `news`, and through in the groups or any outcome at the knockout stage.
pub(crate) fn nations_fate_text_only(outcome: u32, stage: u32, news: u32, text: usize, knockout: i8) -> bool {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    text != 0 && news == 0 && ((s < knockout && d == 6) || s == knockout)
}

/// The staff records of a national-team tournament run in groups and a knockout (virtual
/// `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS`), with the knockout stage, the group qualification round code, the
/// round spread of the knockout weight and the final places that get a record as parameters. Returns 0.
///
/// In the group stages (stage below `knockout`, signed): through (6) the qualification record
/// (`european_cup_add_qualified_record`(club, competition, 0, `group_round`, 0x19)); out (0xfe) the failed-to-qualify
/// record (`european_cup_add_failed_to_qualify_record`(club, competition, 0x41c, 0x19)). At the knockout stage
/// (`.stages()`[knockout]): through (0) the qualification record with the fixture's round prefix and the next round's
/// code in the knockout's round table and the weight `scale_by_round`(0x28, 0x64, round, `spread`); out (0xff) the
/// knocked-out record with the fixture's round prefix and name and the same weight; the places 1, 2, 3 present in
/// `finals` the champion, runner-up and third-place records (Staff_Records 0x86a800, 0x86b0b0, 0x86b710 with the club,
/// the fixture and `a6`); anything else, nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn nations_tournament_staff_records(this: League, club: usize, outcome: u32, stage: u32, game: usize, a6: u32,
        knockout: i8, group_round: u32, spread: u32, finals: &[u8]) -> u32 {
    let game = Fixture(game);
    let (d, s) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if s < knockout {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, group_round, 0x19);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
        }
        return 0;
    }
    if s != knockout {
        return 0;
    }
    match d {
        0 => {
            let r = game.round_number();
            let weight = scale_by_round(0x28, 0x64, r, spread);
            let rounds = Cup(read_usize(this.stages() + knockout as usize * 4)).round_table();
            // the next round's code (the original reads +0x6f of round r)
            let code = read_i16(rounds.wrapping_add((r as isize as usize + 1).wrapping_mul(cup_round::SIZE)) + cup_round::ROUND_CODE) as u16 as u32;
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, code, weight);
        }
        0xff => {
            let weight = scale_by_round(0x28, 0x64, game.round_number(), spread);
            // simplification: the round prefix and name go in registers with garbage in the high 16 bits; 0x86c000 reads the words
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        p if finals.contains(&p) => {
            let record = [0x86a800, 0x86b0b0, 0x86b710][p as usize - 1]; // Staff_Records: champion, runner-up, third place
            african_nations_record_final_result(record, club, game.addr(), a6);
        }
        _ => {}
    }
    0
}

/// The Oceania Nations Cup knockout stage (semi-finals, third place and final) after its two groups.
const OCEANIA_NATIONS_KNOCKOUT: i8 = 1;

/// Writes the news text of a nation's fate in the Oceania Nations Cup (`nations_tournament_fate_text`): in the two
/// groups (stage below 1) through "{nation} qualify for semi finals" (0xa6a130) or the group news, out the group news;
/// in the knockout (stage 1) the tie news or the Copa América templates.
/// Original: `OceaniaNations::build_fate_news_text` (0x799300, oceania_nations.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn oceania_nations_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    covered!("cx07_3 oceania nations fate text");
    let knockout = OCEANIA_NATIONS_KNOCKOUT; // cx07_3: the stage of the Oceania knockout texts
    nations_tournament_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, knockout, 0xa6a130);
}

/// Comparer for `oceania_nations_build_fate_news_text`: the text-only paths side by side into two Texts
/// (`compare_text_writer`); the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if !nations_fate_text_only(outcome, stage, news, text, OCEANIA_NATIONS_KNOCKOUT) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| oceania_nations_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("oceania_nations_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// rule: in the two Oceania Nations Cup groups (stage below 1) a nation going through gets the qualification record
/// (round code 0x82, weight 25) and one going out the failed-to-qualify record; in the knockout (stage 1) a nation going
/// through gets the qualification by round (weight 40 to 100 over one round), one going out the knocked-out record, and
/// the winner, runner-up and third place the final records; the fourth place gets nothing
/// (`nations_tournament_staff_records`)
/// Original: `OceaniaNations::update_staff_records_for_status` (0x799760, oceania_nations.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn oceania_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("cx07_3 oceania nations staff records");
    let knockout = OCEANIA_NATIONS_KNOCKOUT; // cx07_3: the stage of the Oceania knockout records
    nations_tournament_staff_records(League(this), club, outcome, stage, game, a6, knockout, 0x82, 1, &[1, 2, 3])
}

/// Comparer for `oceania_nations_update_staff_records_for_status`: the records go to the managers' history (they
/// cannot run twice); only the original, checking the return of 0.
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("oceania_nations_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Stores a nation's progress in the Oceania Nations Cup statistics (`group_championship_progress` of cx04_5): in the
/// groups (stage below 1) current 1→1, 2→3, 3→5; best 1, 2→1, 3→5; worst 1, 2→3, 3→5; a 4 and the knockout codes as
/// they come.
/// Original: `OceaniaNations::set_club_progress` (0x79a490, oceania_nations.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx07_3 oceania nations progress");
    group_championship_progress(this, club, stage, current, best, worst, 1, [[1, 3, 5, 4], [1, 1, 5, 4], [3, 3, 5, 4]]); // cx07_3: the Oceania codes
}

/// Comparer for `oceania_nations_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("oceania_nations_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || oceania_nations_set_club_progress(this, club, stage, current, best, worst));
}
