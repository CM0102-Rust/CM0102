// ---------- rx12_17: the Oceania World Cup qualifying: the fate news text and the inter-continental play-off (wc_oceania_league.cpp) ----------

/// The fate news templates of the Oceania qualifying (langlib, `.rdata`).
pub(crate) mod wc_oceania_fate_texts {
    /// "{}<%s - Team Name(e.g.Australia)>{} have qualified for the semi finals of the {}<%s - Competition Name>{}."
    pub(crate) const QUALIFIED_FOR_SEMI_FINALS_OF: usize = 0xad4c48;
    /// "{}<%s - Team Name(e.g.Australia)>{} qualify for semi finals"
    pub(crate) const QUALIFY_FOR_SEMI_FINALS: usize = 0xad4c0c;
    /// "{}<%s - Team Name>{} qualify for playoff"
    pub(crate) const QUALIFY_FOR_PLAYOFF: usize = 0xad4be0;
    /// "{}<%s - Team Name>{} knocked out of World Cup qualifying"
    pub(crate) const KNOCKED_OUT_OF_QUALIFYING: usize = 0xad4ba4;
    /// "{}<%s - Team Name>{} qualify for World Cup"
    pub(crate) const QUALIFY_FOR_WORLD_CUP: usize = 0xad4b78;
    /// "{}<%s - Team Name>{} knocked out in World Cup playoff"
    pub(crate) const KNOCKED_OUT_IN_PLAYOFF: usize = 0xad4b40;
}

/// rule: the Oceania qualifying fate news: in the groups (stage below 1) a nation that goes through (fate 6) "have
/// qualified for the semi finals of" the competition (or, in short, "qualify for semi finals") and one that goes out
/// (−2) the shared group sentence; in the final (stage 1) the winner "qualify for playoff" and the loser "knocked out of
/// World Cup qualifying"; in the inter-continental play-off (code 0xa0) "qualify for World Cup" or "knocked out in World
/// Cup playoff"
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a word), four values
/// passed on, `full_sentence` and the Text).
///
/// Stage below 1: fate 6, with `full_sentence`, "have qualified for the semi finals of the" competition (its long name
/// `competition::NAME` with the short gender byte), the club's link and the competition's (its short name); without,
/// "qualify for semi finals" with the club's link. Fate 0xfe: the shared group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record, −2, full_sentence, text)). Any
/// other fate, nothing.
///
/// Stage 1, or any other stage with code 0xa0: with `full_sentence`, the shared tie sentence
/// (`exe::comp_util_write_cup_tie_result_news`(club_index, a5, a8, the fate dword, a6, text)); otherwise fate 1 "qualify
/// for playoff" (stage 1) or "qualify for World Cup" (play-off), any other fate "knocked out of World Cup qualifying" or
/// "knocked out in World Cup playoff", with the club's link. Another stage with another code: nothing.
/// Original: `WcOceaniaLeague::build_fate_news_text` (0x91e990, wc_oceania_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_oceania_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let stage = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if stage < 1 {
        match fate as u8 {
            6 => {
                covered!("rx12_17 oceania group qualifier");
                if full_sentence != 0 {
                    qualifying_fate_line(text, t::QUALIFIED_FOR_SEMI_FINALS_OF, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));
                    link_fate_club(text, club);
                    link_fate_competition(text, comp); // rx12_17: the competition's link after the club's
                } else {
                    qualifying_fate_line(text, t::QUALIFY_FOR_SEMI_FINALS, club, None);
                    link_fate_club(text, club);
                }
            }
            0xfe => exe::comp_util_write_group_qualify_news(club_index, code, comp, -2, full_sentence, text),
            _ => {}
        }
        return;
    }
    let final_tie = stage == 1;
    if !final_tie && code as u16 != 0xa0 {
        return;
    }
    covered!("rx12_17 oceania tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    let template = match (final_tie, through) {
        (true, true) => t::QUALIFY_FOR_PLAYOFF,
        (true, false) => t::KNOCKED_OUT_OF_QUALIFYING,
        (false, true) => t::QUALIFY_FOR_WORLD_CUP, // rx12_17: the play-off winner
        (false, false) => t::KNOCKED_OUT_IN_PLAYOFF,
    };
    qualifying_fate_line(text, template, club, None);
    link_fate_club(text, club);
}

/// Comparer for `wc_oceania_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_oceania_league_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_oceania_league_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

/// The two nations of the Oceania play-off: the Oceania final's winner (the first entry of the final stage's team table,
/// `stages()`[1], with outcome 1 at +5 of its 6-byte entry) and the South American section's play-off nation (the
/// first row of its table with `league_table_row::STATUS` 1), in that order; either may be missing.
unsafe fn oceania_playoff_nations(this: League, south: League) -> Vec<usize> {
    let mut v = Vec::new();
    let final_stage = Cup(read_usize(this.stages() + 4));
    let teams = final_stage.team_table();
    if let Some(k) = (0..final_stage.team_count().max(0) as usize).find(|&k| read_u8(teams + 6 * k + 5) == 1) {
        v.push(read_usize(teams + 6 * k));
    }
    let rows = south.table();
    if let Some(k) = (0..south.club_count().max(0) as usize).find(|&k| read_u8(rows + k * league_table_row::SIZE + league_table_row::STATUS) == 1) {
        v.push(read_usize(rows + k * league_table_row::SIZE));
    }
    v
}

/// The inter-continental play-off of the Oceania qualifying: the Oceania final's winner against the South American
/// section's play-off nation (`oceania_playoff_nations`, the section the object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]), over two legs.
///
/// The list of 8 bytes (`exe::crt_malloc`; out of memory, the box of line 0x403); without both nations, the box of line
/// 0x41f and the list freed. Otherwise: the round table of the current stage (virtual `comp_vtable::CREATE_SCHEDULE`(the
/// current stage byte, &rounds, &extra, 0); the original pushes the byte with stack address bytes above it, which the
/// schedule does not read), the stage (`exe::crt_operator_new`(0xb2) and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(
/// the qualifying, 2, the list, the rounds, the competition record, the table, the year, the current stage byte, 4, the
/// extra, 0x14, 0, 0, 0, 0), ret 0x3c; words zero-extended) in `stages()`[current stage]; the list and the table freed
/// (`exe::crt_free`); without a stage, the box of line 0x431; with it, its first round set up (`exe::cup_set_up_round`(
/// stage, 0)).
/// Original: `WcOceaniaLeague::create_playoff_stage` (0x91f800, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_playoff_stage(this: usize) {
    let this = League(this);
    let south = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4))));
    let v = exe::crt_malloc(8);
    if v == 0 {
        return show_error_at(0xad4afc, 0x403);
    }
    let nations = oceania_playoff_nations(this, south);
    for (k, &n) in nations.iter().enumerate() {
        write_usize(v + 4 * k, n); // rx12_17: the play-off list
    }
    if nations.len() != 2 { // rx12_17: both nations are needed
        show_error_at(0xad4afc, 0x41f);
        exe::crt_free(v);
        return;
    }
    covered!("rx12_17 oceania play-off");
    let stage_byte = this.current_stage() as u8 as u32;
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage_byte, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, v, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage_byte, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4);
    write_usize(slot, stage);
    exe::crt_free(v);
    exe::crt_free(table);
    let stage = read_usize(this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return show_error_at(0xad4afc, 0x431);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_oceania_league_create_playoff_stage`: the stage is allocated and its round set up, so only the
/// original runs; then the stage in `stages()`[current stage] must hold the planned two nations
/// (`oceania_playoff_nations`, read before) in its team table, in that order.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let league = League(this);
    let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
    if league.stages() == 0 || read_usize(league.stages() + 4) == 0 || south == 0 || League(south).table() == 0 {
        return o(this);
    }
    let plan = oceania_playoff_nations(league, League(south));
    o(this);
    if plan.len() != 2 {
        return;
    }
    let stage = read_usize(league.stages() + (league.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read = if t != 0 { vec![read_usize(t), read_usize(t + 6)] } else { vec![0, 0] };
    SCOREBOARD.record(read == plan, || format!("wc_oceania_league_create_playoff_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}
