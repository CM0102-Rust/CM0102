// ---------- cx05_10: the European Championship results: a nation's fate text, the staff records and the progress codes (euro_champ.cpp) ----------

/// The European Championship fate templates of the knockout (stage 3), by fate byte: 1 win, 2 runners-up, 0xff knocked
/// out; any other, "through to next round" (0x987198).
fn euro_champ_fate_template(fate: u8) -> usize {
    match fate {
        1 => 0x98732c, // cx05_10: "win"
        2 => 0x9872c8,
        0xff => 0x98713c,
        _ => 0x987198,
    }
}

/// Writes the news text of a nation's fate in the European Championship.
///
/// In the group stages (stage below 3, signed): the shared group news for any outcome
/// (`exe::comp_util_write_group_qualify_news`(club, a4, competition, outcome, news, text), cdecl). In the knockout
/// (stage 3): with `news` the tie result news (`exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome,
/// a6, text), cdecl), without it `euro_champ_fate_template` with the nation's short name and the European
/// Championship's (the competition record of [`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`], not the cup's own) and
/// their links (`european_cup_fate_text`). From stage 4 on, nothing.
/// Original: `EuroChamp::build_fate_news_text` (0x57b330, euro_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn euro_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    let knockout = 3i8; // cx05_10: the European Championship knockout stage
    if s < knockout {
        return exe::comp_util_write_group_qualify_news(club_id, a4, League(this).competition().addr(), outcome as i32, news, text);
    }
    if s != knockout {
        return;
    }
    if news != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
    }
    covered!("cx05_10 euro knockout fate");
    let comp = comp_record_of_key(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP);
    european_cup_fate_text(text, club_id, euro_champ_fate_template(outcome as u8), read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
}

/// Comparer for `euro_champ_build_fate_news_text`: the knockout text without `news` side by side into two Texts
/// (`compare_text_writer`); the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if text == 0 || news != 0 || stage as u8 as i8 != 3 {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| euro_champ_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("euro_champ_build_fate_news_text(club {}, outcome {})", club_id, outcome as u8));
}

/// Writes the staff records of a nation's European Championship result; returns 0.
///
/// In the group stages (below 3, signed): through (6) the qualification record (`european_cup_add_qualified_record`:
/// stage 0, round 0x78, weight 0x19); out (0xfe) the failed-to-qualify record (0x41c, weight 0x19) and the national
/// team leaves the competition (`clear_national_team_competition`). In the knockout (stage 3): through (0) the
/// qualification record with stage 0, the code of the next round of the knockout's round table (`.stages()`[3], round
/// table +0x6f of round r = the fixture's `.round_number()`) and the weight `scale_by_round`(0x28, 0x64, r, 3); out
/// (0xff) the knocked-out record (stage 0, the fixture's `.round_name()`, the same weight) and the team leaves; winner
/// (1) the cup winners record, the team leaves and `*a4` = 3; runner-up (2) the runners-up record and the team
/// leaves. Any other outcome, nothing.
/// Original: `EuroChamp::update_staff_records_for_status` (0x57b1d0, euro_champ.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn euro_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: usize, game: usize, a6: u32) -> u32 {
    let league = League(this);
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = league.competition().addr();
    let game = Fixture(game);
    if s < 3 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            covered!("cx05_10 euro out in the groups");
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
            clear_national_team_competition(Club(club));
        }
        return 0;
    }
    if s != 3 {
        return 0;
    }
    match d {
        0 => {
            let r = game.round_number();
            let weight = scale_by_round(0x28, 0x64, r, 3);
            let knockout = Cup(read_usize(league.stages() + 0xc));
            let next_round = read_u16(knockout.round_table() + (r as isize + 1) as usize * cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, 0, next_round as u32, weight);
        }
        0xff => {
            let weight = scale_by_round(0x28, 0x64, game.round_number(), 3);
            // simplification: the round name goes in a register with garbage in the high 16 bits; 0x86c000 uses the word
            european_cup_add_knocked_out_record(club, comp, 0, game.round_name() as u16 as u32, weight);
            clear_national_team_competition(Club(club));
        }
        1 => {
            covered!("cx05_10 euro winner");
            african_nations_record_final_result(0x86a800, club, game.addr(), a6);
            clear_national_team_competition(Club(club));
            write_u8(a4, 3); // cx05_10: the winner's fate code
        }
        2 => {
            african_nations_record_final_result(0x86b0b0, club, game.addr(), a6);
            clear_national_team_competition(Club(club));
        }
        _ => {}
    }
    0
}

/// Comparer for `euro_champ_update_staff_records_for_status`: the records go to the managers' history and the national
/// teams change competition (they cannot run twice); only the original, checking the return of 0.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: usize, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("euro_champ_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Stores a nation's progress in the European Championship statistics (`group_championship_progress` of cx04_5): in
/// the group stages (below 3) current 1→5, 2→9, 3→9, 4→13; best 1, 2→1, 3→9, 4→13; worst 1→5, 2, 3→9, 4→13 (the
/// African Nations Cup's codes); afterwards as they come.
/// Original: `EuroChamp::set_club_progress` (0x57cc30, euro_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx05_10 euro progress");
    group_championship_progress(this, club, stage, current, best, worst, 3, [[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx05_10: the European Championship codes
}

/// Comparer for `euro_champ_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_group_championship_progress("euro_champ_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || euro_champ_set_club_progress(this, club, stage, current, best, worst));
}
