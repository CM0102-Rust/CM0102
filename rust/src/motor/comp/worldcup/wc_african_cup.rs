use super::*;

/// The season turnover of the African World Cup qualifying: last campaign's tables, entrant list and stages freed, the
/// season archived, the next campaign's year, the entrants again, as many stages as their format needs, the predicted
/// positions, the first round, the round table and the tie tree.
///
/// `qualifying_cup_free_tables`, the stage vector (`cup::STAGES`) freed (not zeroed) and `cup::CURRENT_STAGE` = −1; the
/// season archived (`exe::comp_stats_archive_season_and_reset`, +8, thiscall, ret 4); the year up by four until it
/// reaches the game's (`advance_qualifying_campaign`); `cup::CURRENT_ROUND` = 0, `cup::SEASON_SET_UP` = 1 and
/// `cup::RUN_BY_STAGES` = 0. Then, each with its own box (wc_african_cup.cpp): the entrants (WcAfricanCup 0x9106a0,
/// thiscall, ret 0) when 0 (line 0x36a); the stage count by `AFRICAN_FORMAT` (0 or 3 → 5, 1 → 4, other → 7) in a new
/// vector (`exe::crt_malloc`(4 × count)); the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the
/// first round (WcAfricanCup 0x910120, thiscall, ret 0; negative: line 0x37f), the round table (virtual
/// `comp_vtable::SET_UP_RULES`; negative: 0x386) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative:
/// 0x38d), whose eax it returns. After a box the copy returns 0 (the original the box's eax).
/// Original: `WcAfricanCup::start_new_season` (0x910410, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    qualifying_cup_free_tables(cup);
    exe::crt_free(cup.stages());
    cup.set_current_stage(usize::MAX);
    let stats = read_usize(this + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    advance_qualifying_campaign(this + cup::YEAR);
    covered!("mwg_13 african qualifying rollover");
    cup.set_current_round(0);
    cup.set_season_set_up(1);
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x36a);
        return 0;
    }
    let stages: usize = match read_u8(this + wt::AFRICAN_FORMAT) {
        0 | 3 => 5,
        1 => 4,
        _ => 7,
    };
    cup.set_stage_count(stages);
    cup.set_stages(exe::crt_malloc((stages as u32) << 2));
    call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_SET_UP_FIRST_ROUND, this) < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x37f);
        return 0;
    }
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x386);
        return 0;
    }
    let r = call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x38d);
        return 0;
    }
    r as u32
}

/// Comparer for `wc_african_cup_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_AFRICAN_CUP_START_NEW_SEASON);
    compare_qualifying_turnover("wc_african_cup_start_new_season", League(this), || o(this))
}

/// A World Cup qualifying section's next campaign: the year goes up in steps of four until it is no longer before the
/// game's year (the section's year a signed word, the game's [`G_CURRENT_DATE_YEAR`] an unsigned one, read again on
/// each step).
pub(crate) unsafe fn advance_qualifying_campaign(year_at: usize) {
    while (read_i16(year_at) as i32) < read_u16(G_CURRENT_DATE_YEAR) as i32 {
        write_i16(year_at, read_i16(year_at).wrapping_add(4));
    }
}

/// The plan of a qualifying turnover (only the original runs: it frees and rebuilds): the year went up by a multiple
/// of four and is no longer before the game's year, and the season is open.
pub(crate) unsafe fn compare_qualifying_turnover(name: &str, this: League, original: impl FnOnce() -> u32) -> u32 {
    let before = this.year() as i32;
    let r = original();
    let after = this.year() as i32;
    let ok = after >= before && (after - before) % 4 == 0 && after >= read_u16(G_CURRENT_DATE_YEAR) as i32 && this.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x4c {}", name, this.addr(), before, after, this.season_closed()));
    r
}

/// The common first half of the African and CONCACAF qualifying turnovers (Cup objects): the team, round and tie tables
/// freed and zeroed (as the cup turnover template), the entrant list (`CUP_ENTRANT_LIST`) freed (not zeroed; the
/// African one even when null: `free(0)` does nothing) and the stages destroyed (`destroy_comp_stages`).
pub(crate) unsafe fn qualifying_cup_free_tables(cup: Cup) {
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if cup.round_table() != 0 {
        exe::crt_free(cup.round_table());
        cup.set_round_table(0);
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let list = read_usize(cup.addr() + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    if list != 0 {
        exe::crt_free(list);
    }
    destroy_comp_stages(League(cup.addr()));
}

/// rule: the African qualifying fate news: in the groups (stage codes 0x407 to 0x40b and the final group 0x472), the
/// shared group sentence, or, in short, "qualify for Final" (fate 6) and "qualify for the World Cup" (fate 7); in the
/// first round (stage −1) and in the ties (code 0), the shared tie sentence, or, in short, "qualify for 2nd round" or
/// "knocked out in 1st round" in the first round and "qualify for" or "knocked out of" the World Cup in the ties
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a signed word), four
/// values passed on, `full_sentence` and the Text).
///
/// Stage other than −1 with a code: a code outside 0x407..=0x40b and 0x472 writes nothing. With `full_sentence`, the
/// shared group sentence (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record
/// `league::COMPETITION`, the fate dword, full_sentence, text), cdecl). Otherwise fate 6: "qualify for Final" with the
/// club's link; fate 7: "qualify for the World Cup" (the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`]: its short
/// gender and short name) with the club's link and the World Cup's; any other fate, the shared group sentence with 0
/// for `full_sentence`.
///
/// Stage −1 or code 0: with `full_sentence`, the shared tie sentence (`exe::comp_util_write_cup_tie_result_news`(
/// club_index, a5, a8, the fate dword, a6, text), cdecl). Otherwise in the first round (stage −1) fate 1 "qualify for
/// 2nd round in qualifiers", any other "knocked out in 1st round qualifier" (the club only); in a tie fate 1 "qualify
/// for", any other "knocked out of" the World Cup (its gender and short name); then the World Cup's link and the
/// club's, in that order.
/// Original: `WcAfricanCup::build_fate_news_text` (0x90fe00, wc_african_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_african_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let world_cup = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    let wc_name = Some((world_cup + competition::SHORT_GENDER_NAME, world_cup + competition::SHORT_NAME));
    let first_round = stage as u8 as i8 == -1;
    let raw_code = code;
    let code = code as u16 as i16;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if !first_round && code != 0 {
        if !((0x407..=0x40b).contains(&code) || code == 0x472) {
            return;
        }
        covered!("rx11_1 african group fate");
        if full_sentence != 0 {
            return exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, full_sentence, text);
        }
        match fate as u8 {
            6 => {
                qualifying_fate_line(text, t::QUALIFY_FOR_FINAL, club, None);
                link_fate_club(text, club);
            }
            7 => {
                qualifying_fate_line(text, t::GROUP_QUALIFY_FOR_WORLD_CUP, club, wc_name);
                link_fate_club(text, club); // rx11_1: the club's link first in a group
                link_fate_competition(text, world_cup);
            }
            _ => exe::comp_util_write_group_qualify_news(club_index, raw_code, comp, fate as i32, 0, text),
        }
        return;
    }
    covered!("rx11_1 african tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    if first_round {
        qualifying_fate_line(text, if through { t::QUALIFY_FOR_SECOND_ROUND } else { t::KNOCKED_OUT_IN_FIRST_ROUND }, club, None);
    } else {
        qualifying_fate_line(text, if through { t::TIE_QUALIFY_FOR_WORLD_CUP } else { t::KNOCKED_OUT_OF_WORLD_CUP }, club, wc_name);
    }
    link_fate_competition(text, world_cup); // rx11_1: the World Cup's link first in a tie
    link_fate_club(text, club);
}

/// Comparers of the two fate texts: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_AFRICAN_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_african_cup_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_african_cup_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

/// rule: in the African qualifying, a first round winner gets the qualification for round 0x14 (weight 25) and a loser
/// the knocked-out record (25) and the end of its campaign; in the groups, a group winner who qualifies for the World
/// Cup (7) gets its record (100) and place, a qualifier for the play-off (6) the qualification for round 0x96 (45), and
/// one going out (−2) the failed-to-qualify record (45) and the end of its campaign; in the play-off, a winner gets the
/// World Cup record (55) and place, a loser the knocked-out record (100) and the end of its campaign
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). The group stages by the
/// format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): below stage 5 with format 0, below 4 with 1 or 3, below 6
/// with 2; the play-off: stage 6 with format 2, stage 4 with format 3. Stage −1, outcome 1:
/// `european_cup_add_qualified_record`(club, the competition `league::COMPETITION`, 0, 0x14, 0x19); outcome 2:
/// `knock_nation_out`(weight 0x19). A group, outcome 7: `qualify_nation_for_world_cup`(weight 0x64), returns −1;
/// outcome 6: the qualification (0, 0x96, 0x2d); 0xfe: `european_cup_add_failed_to_qualify_record`(0x14, 0x2d) and
/// `clear_national_team_competition`. The play-off, outcome 1: `qualify_nation_for_world_cup`(0x37), −1; outcome 2:
/// `knock_nation_out`(0x64). Anything else, 0.
/// Original: `WcAfricanCup::update_staff_records_for_status` (0x90fbf0, wc_african_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let comp = League(this).competition().addr();
    let game = Fixture(game);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage == -1 {
        covered!("rx11_2 african first round records");
        match outcome {
            1 => european_cup_add_qualified_record(club, comp, 0, 0x14, 0x19),
            2 => knock_nation_out(club, comp, game, 0x19),
            _ => {}
        }
        return 0;
    }
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    if stage < african_groups_end(format) {
        covered!("rx11_2 african group records");
        match outcome {
            7 => {
                qualify_nation_for_world_cup(club, out, 0x64);
                return u32::MAX;
            }
            6 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x2d),
            0xfe => {
                european_cup_add_failed_to_qualify_record(club, comp, 0x14, 0x2d);
                clear_national_team_competition(Club(club)); // rx11_2: the African group's eliminated nation ends its campaign
            }
            _ => {}
        }
        return 0;
    }
    if (format == 2 && stage == 6) || (format == 3 && stage == 4) {
        covered!("rx11_2 african play-off records");
        match outcome {
            1 => {
                qualify_nation_for_world_cup(club, out, 0x37); // rx11_2: the African play-off winner to the World Cup
                return u32::MAX;
            }
            2 => knock_nation_out(club, comp, game, 0x64),
            _ => {}
        }
    }
    0
}

/// rule: when a World Cup qualifying section closes its season, every national team entered in it leaves it
///
/// NationalTeams::clear_competition_for_comp_clubs(the competition record `league::COMPETITION`) on
/// [`G_NATIONAL_TEAMS`] (`world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS`, thiscall, ret
/// 4), whose eax it returns. Shared by the African, Asian, CONCACAF, European and Oceania sections (virtual +0x30) and
/// one more class (vtable 0x96a0d4).
/// Original: `WcAfricanCup::record_season_honours` (0x90fbe0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_qualifying_record_season_honours(this: usize) -> u32 {
    covered!("rx11_2 qualifying season honours");
    let clear: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), League(this).competition().addr())
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_AFRICAN_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let plan = african_staff_records_plan(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), stage as u8 as i8, outcome as u8);
    compare_qualifying_staff_records("wc_african_cup_update_staff_records_for_status", club, outcome, stage, out, plan, || o(this, club, outcome, stage, out, game, a6))
}

/// Comparer for `wc_qualifying_record_season_honours`: the national teams leave the competition, which does not run
/// twice (and the original's club list is not freed: a second run would leak another), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wc_qualifying_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_QUALIFYING_RECORD_SEASON_HONOURS)(this)
}

/// rule: the African qualifying moves on one stage at a time: from the first round to stage 0 it builds all the
/// groups, and the play-off comes at stage 6 with format 2 or at stage 4 with format 3; past the last stage it is an
/// error
///
/// At or past the last stage (`cup::STAGE_COUNT` − 1, signed), the error box of line 0x417. Otherwise
/// `cup::CURRENT_STAGE` + 1 and, by the new stage: 0, WcAfricanCup::create_group_stages; 6 with format 2
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`) or 4 with format 3, WcAfricanCup::create_playoff_stage (thiscall, ret 0,
/// called directly).
/// Original: `WcAfricanCup::advance_stage` (0x910920, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_advance_stage(this: usize) {
    let cup = Cup(this);
    let current = cup.current_stage() as i32;
    if current >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP, 0x417);
    }
    let next = current + 1;
    cup.set_current_stage(next as usize);
    covered!("rx11_3 african next stage");
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    if next == 0 {
        call_section_routine(FN_WC_AFRICAN_CUP_CREATE_GROUP_STAGES, this);
    } else if (next == 6 && format == 2) || (next == 4 && format == 3) {
        call_section_routine(FN_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE, this);
    }
}

/// rule: the African groups are drawn by pots: the first round winners join the seeded entrants, the list is ordered by
/// reputation and group g takes entrants g, g + groups, g + 2 × groups...; each group is a round robin league stage
/// (3 points a win, 1 a draw), with its draw news
///
/// The group layout of `african_group_layout`; a team array (`exe::crt_malloc` of 4 × size; null, the error box of line
/// 0x45e). The first round winners into the list (`place_first_round_winners`), then the list sorted (`exe::crt_qsort`
/// with the reputation comparison 0x4b07d0). For each group g: the round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)); the teams; the stage (operator new of 0xee and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, size, the teams, 2, the
/// record, the round table, rounds, 3, 1, 0xa, the tie-breaks {1, 2, 3, 0}, the places {1, 0, 0, 0}, the year, g, the
/// label, 0xf, 4, 0, the code 0x24 (format below 2) or 0x28, −1, 0, 2; the original pushes the words with garbage in
/// the high bytes, which the constructor does not read: the copy passes clean values) into `cup::STAGES`[g]; without a
/// stage, the error box of line 0x48b and the routine ends (the team array and the round table are not freed); with
/// it, `cup::CURRENT_STAGE` g, the round table freed and the group draw news (`exe::league_send_group_draw_news`). Last
/// the team array freed. The SEH frame covers only the stage's new.
/// Original: `WcAfricanCup::create_group_stages` (0x9109e0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_create_group_stages(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP;
    let cup = Cup(this);
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let Some((groups, size)) = african_group_layout(format) else {
        return; // the original goes on with stack leftovers: no format of the game reaches here
    };
    let code: u32 = if (format as i8) < 2 { 0x24 } else { 0x28 };
    let teams = exe::crt_malloc((size * 4) as u32);
    if teams == 0 {
        return show_error_at(cpp, 0x45e);
    }
    covered!("rx11_3 african groups");
    place_first_round_winners(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    exe::crt_qsort(list, read_u32(this + 0xb6), 4, 0x4b07d0);
    let tiebreaks: [u8; 4] = [1, 2, 3, 0];
    let places: [u8; 4] = [1, 0, 0, 0];
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    for g in 0..groups {
        let (mut rounds, mut label) = (0i16, 0i16);
        let table = create_schedule(this, g as u32, &mut rounds, &mut label, 0);
        for k in 0..size {
            write_usize(teams + k * 4, read_usize(list + (g + k * groups) * 4)); // rx11_3: the pots dealt across the groups
        }
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(p, this, size as u32, teams, 2, League(this).competition().addr(), table, rounds as u16 as u32, 3, 1, 0xa, tiebreaks.as_ptr() as usize,
                places.as_ptr() as usize, cup.year() as u16 as u32, g as u32, label as u16 as u32, 0xf, 4, 0, code, -1, 0, 2)
        } else {
            0
        };
        write_usize(cup.stages() + g * 4, stage);
        if read_usize(cup.stages() + g * 4) == 0 {
            return show_error_at(cpp, 0x48b);
        }
        cup.set_current_stage(g);
        exe::crt_free(table);
        exe::league_send_group_draw_news(read_usize(cup.stages() + g * 4));
    }
    exe::crt_free(teams);
}

/// rule: the African play-off is a knockout cup stage between the group winners (four, or six with format 2), built as
/// the stage of its own index
///
/// n = `african_playoff_size`; a team array (`exe::crt_malloc` of 4n; null, the error box of line 0x4b3) with the
/// winner of each group k < n (row 0 of `cup::STAGES`[k]'s `league::TABLE`); the round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`(n, &rounds, &label, 0); the original pushes n from a stack dword whose high bytes are
/// not set, read as the stage byte); the stage (operator new of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c: the section, n, the teams, rounds, the record, the round table, the year, n, 1, the label, 0x14,
/// 0, 0, 0, 0; the words clean) into `cup::STAGES`[n]; the array and the round table freed (`exe::crt_free`); without a
/// stage, the error box of line 0x4ca; with it, its first round set up (`exe::cup_set_up_round`(stage, 0)). The current
/// stage is not touched (advance_stage set it). The SEH frame covers only the stage's new.
/// Original: `WcAfricanCup::create_playoff_stage` (0x910d20, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_create_playoff_stage(this: usize) {
    let cpp = wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP;
    let cup = Cup(this);
    let n = african_playoff_size(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    let teams = exe::crt_malloc((n * 4) as u32);
    if teams == 0 {
        return show_error_at(cpp, 0x4b3);
    }
    covered!("rx11_3 african play-off");
    for k in 0..n {
        let group = League(read_usize(cup.stages() + k * 4));
        write_usize(teams + k * 4, read_usize(group.table())); // rx11_3: each group's winner
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this, n as u32, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, teams, rounds as u16 as u32, League(this).competition().addr(), table, cup.year() as u16 as u32, n as u32, 1, label as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(cup.stages() + n * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let stage = read_usize(cup.stages() + n * 4);
    if stage == 0 {
        return show_error_at(cpp, 0x4ca);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_african_cup_advance_stage`: it builds stages, so only the original runs; the stage stays (at the
/// last one) or moves on by one, except that building the groups leaves it at the last group.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_advance_stage(this: usize) {
    let before = Cup(this).current_stage() as i32;
    let last_group = african_group_layout(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT)).map_or(0, |(groups, _)| groups as i32 - 1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_ADVANCE_STAGE)(this);
    let after = Cup(this).current_stage() as i32;
    let ok = after == before || if before == -1 { after == 0 || after == last_group } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("wc_african_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// Comparer for `wc_african_cup_create_group_stages`: it builds stages, so only the original runs; then the plan: the
/// current stage is the last group, and each group's table holds the entrants g, g + groups... of the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_create_group_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_CREATE_GROUP_STAGES);
    let layout = african_group_layout(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    let cup = Cup(this);
    let Some((groups, size)) = layout.filter(|_| cup.stages() != 0 && read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST) != 0) else {
        return o(this);
    };
    o(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let mut ok = cup.current_stage() == groups - 1;
    for g in 0..groups {
        let stage = League(read_usize(cup.stages() + g * 4));
        let mut plan: Vec<usize> = (0..size).map(|k| read_usize(list + (g + k * groups) * 4)).collect();
        plan.sort_unstable();
        ok &= !stage.is_null() && league_stage_clubs(stage) == plan;
    }
    SCOREBOARD.record(ok, || format!("wc_african_cup_create_group_stages({:#x}): stage {} of {} groups of {}", this, cup.current_stage() as i32, groups, size));
}

/// Comparer for `wc_african_cup_create_playoff_stage`: it builds a stage, so only the original runs; then the plan: the
/// stage at the play-off index with the group winners (read before) in its team table, in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE);
    let cup = Cup(this);
    let n = african_playoff_size(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    if cup.stages() == 0 || (0..n).any(|k| read_usize(cup.stages() + k * 4) == 0) {
        return o(this);
    }
    let plan: Vec<usize> = (0..n).map(|k| read_usize(League(read_usize(cup.stages() + k * 4)).table())).collect();
    o(this);
    let stage = read_usize(cup.stages() + n * 4);
    let t = if stage != 0 { Cup(stage).team_table() } else { 0 };
    let read: Vec<usize> = if t != 0 { (0..n).map(|k| read_usize(t + k * 6)).collect() } else { Vec::new() };
    SCOREBOARD.record(read == plan, || format!("wc_african_cup_create_playoff_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

/// rule: the African qualifying progress of a nation: in the first round the winner is 1 at best and 21 (19 with six
/// groups) now, the loser 25; in the groups the places map to bands of the final position (five groups: 1, 6, 11, 16,
/// 21; four groups: 1, 5, 9, 13, 17, 21, the winners 3 now with format 3; six groups: 1, 7, 13, 19, the winners 4 at
/// worst); in the play-off the winner 1 and the loser 3 (4 with six groups)
///
/// The codes of `african_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes`).
/// Original: `WcAfricanCup::set_club_progress` (0x9115a0, wc_african_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let this = League(this);
    covered!("rx11_4 african progress");
    let e = exe::comp_stats_find_club_entry(this.comp_stats(), club);
    let format = read_u8(this.addr() + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let (c, b, w) = african_progress_codes(format, stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// rule: a nation's status in the African, Asian, CONCACAF and Oceania qualifying is qualified (−2) while its progress
/// code (stat 0x12) is 1, and not (−3) otherwise
///
/// `qualifying_stat_byte`(club, 0x12) = 1: −2, otherwise −3 (a dword in eax). Shared by four vtables (+0x70).
/// Original: `WcAfricanCup::get_qualification_status` (0x911b80, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    covered!("rx11_4 qualification status");
    if qualifying_stat_byte(this, club, 0x12) == 1 { -2 } else { -3 } // rx11_4: qualified at code 1
}

/// rule: in the African qualifying a nation's status changes down (−1) whenever its progress code (stat 0x12) is 1, up
/// (1) when it left 1 (stat 0x13, the previous code, was 1), otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 (whatever before was; unlike the Oceania one, so
/// a nation still at 1 reads as falling again [inference: a slip]); otherwise 1 when before was 1, else 0. The byte in
/// al.
/// Original: `WcAfricanCup::get_qualification_status_change` (0x911ba0, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("rx11_4 african status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -1
    } else {
        (before == 1) as i8 // rx11_4: left code 1
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_AFRICAN_CUP_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_african_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_african_cup_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparers of the two status readers: they only read the statistics; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_WC_AFRICAN_CUP_GET_QUALIFICATION_STATUS)(this, club);
    let b = wc_african_cup_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_AFRICAN_CUP_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_african_cup_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

/// Side-by-side comparer of a progress writer: the three codes of the nation's statistics entry (only the original when
/// there is no entry or no statistics).
pub(crate) unsafe fn compare_progress_codes(name: &str, this: usize, club: usize, stage: u32, codes: (u32, u32, u32), o: impl FnOnce(), c: impl FnOnce()) {
    let this = League(this);
    let stats = this.comp_stats();
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 {
        return o();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], o, c);
    SCOREBOARD.record(same, || format!("{}({:#x}, {:#x}, stage {}, {:?}) original={:x?} rust={:x?}", name, this.addr(), club, stage as u8 as i8, codes, mo, mr));
}

/// A nation's competition stat `stat` in a qualifying section's statistics (Comp_Stats::get_club_stat
/// `exe::comp_stats_get_club_stat`(`league::COMP_STATS`, club, stat): push stat; push club; mov ecx -> thiscall, ret 8,
/// a float), truncated (`ftol`), its low byte signed.
pub(crate) unsafe fn qualifying_stat_byte(this: usize, club: usize, stat: u32) -> i8 {
    let this = League(this);
    ftol(exe::comp_stats_get_club_stat(this.comp_stats(), club, stat)) as u8 as i8
}

/// The club of a club index in [`G_CLUB_TABLE`] (`club::SIZE` bytes each).
pub(crate) unsafe fn club_of_index(index: u32) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add((index as usize).wrapping_mul(club::SIZE)))
}

/// One line of a qualifying fate news: `template` (`FN_LANGLIB_SPRINTF`, cdecl, into `NEWS_TEXT_BUFFER`) with the
/// club's short gender twice and, with `other` (a gender byte's address and a name), that gender twice, then the club's
/// short name and the other name; set as the text (`exe::text_set_string`). The caller adds the links.
pub(crate) unsafe fn qualifying_fate_line(text: usize, template: usize, club: Club, other: Option<(usize, usize)>) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    match other {
        Some((gender_at, other_name)) => {
            let og = read_u8(gender_at) as i8 as i32;
            sprintf(NEWS_TEXT_BUFFER, template, g, g, og, og, name, other_name);
        }
        None => {
            sprintf(NEWS_TEXT_BUFFER, template, g, g, name);
        }
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
}

/// A competition record's link in a fate news (its short name, kind 0x7d0).
pub(crate) unsafe fn link_fate_competition(text: usize, record: usize) {
    exe::text_add_link(text, (record + competition::SHORT_NAME) as *const u8, 0x7d0, record);
}

/// The World Cup's records for a nation that qualifies: the qualification record (`european_cup_add_qualified_record`
/// with the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], stage 0, round 0 and `weight`), the place in the World
/// Cup (`exe::worldcup_add_qualified_team` on the World Cup object), `*out` = 4 and the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`], thiscall, ret 8). The
/// African play-off writes `*out` after the list, the groups before it: the copy writes it first (nothing reads it in
/// between).
pub(crate) unsafe fn qualify_nation_for_world_cup(club: usize, out: usize, weight: u32) {
    let record = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP), club);
    write_u8(out, 4);
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION); // NationalTeams::add_qualified_nation
    add(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// A nation knocked out of a qualifying tie: the knocked-out record (`european_cup_add_knocked_out_record` with the
/// fixture's `round_prefix` and `round_name`, words, and `weight`) and the end of its campaign
/// (`clear_national_team_competition`).
pub(crate) unsafe fn knock_nation_out(club: usize, comp: usize, game: Fixture, weight: u32) {
    // simplification: the two words go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
    european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
    clear_national_team_competition(Club(club));
}

/// The plan of a qualifying staff records writer (only the original runs: the records go to the managers' histories
/// and the World Cup place and the end of a campaign do not run twice): the return value and, for a nation that
/// qualifies for the World Cup, `*out` = 4 and the World Cup's entrant count one more when it had room.
pub(crate) unsafe fn compare_qualifying_staff_records(name: &str, club: usize, outcome: u32, stage: u32, out: usize, planned: (u32, bool), original: impl FnOnce() -> u32) -> u32 {
    let (ret, to_world_cup) = planned;
    let world_cup = League(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP));
    let watch = to_world_cup && out != 0 && !world_cup.is_null();
    let n = if watch { world_cup.entrant_count() } else { 0 };
    let room = watch && n < world_cup.entrant_capacity();
    let r = original();
    let same = r == ret && (!watch || (read_u8(out) == 4 && world_cup.entrant_count() == if room { n + 1 } else { n }));
    SCOREBOARD.record(same, || format!("{}({:#x}, outcome {}, stage {}) returned {:#x} plan {:#x}, list {}", name, club, outcome as u8, stage as u8 as i8, r, ret, n));
    r
}

/// The stage where the African groups end by the format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): 5 for format
/// 0, 4 for 1 and 3, 6 for 2; any other format has no group stage.
pub(crate) fn african_groups_end(format: u8) -> i8 {
    match format {
        0 => 5,
        1 | 3 => 4,
        2 => 6,
        _ => i8::MIN,
    }
}

/// The planned return of the African records (and whether the nation enters the World Cup).
pub(crate) fn african_staff_records_plan(format: u8, stage: i8, outcome: u8) -> (u32, bool) {
    let group = stage != -1 && stage < african_groups_end(format);
    let play_off = matches!((format, stage), (2, 6) | (3, 4));
    if (group && outcome == 7) || (play_off && outcome == 1) { (u32::MAX, true) } else { (0, false) }
}

/// The African groups by the format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): (groups, nations per group) =
/// (5, 5) for format 0, (4, 6) for 1 and 3, (6, 4) for 2. Any other format leaves both counts uninitialised in the
/// original (stack leftovers; see DEFEITOS_DO_ORIGINAL): None.
pub(crate) fn african_group_layout(format: u8) -> Option<(usize, usize)> {
    match format {
        0 => Some((5, 5)),
        1 | 3 => Some((4, 6)), // rx11_3: four groups of six
        2 => Some((6, 4)),
        _ => None,
    }
}

/// The African play-off's size and stage index by the format byte: 6 for format 2, 4 otherwise.
pub(crate) fn african_playoff_size(format: u8) -> usize {
    if format == 2 { 6 } else { 4 }
}

/// The first round winners (the cup's team table rows with status 1, in order) placed in the empty slots of the entrant
/// list (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`, +0xb6 slots), each in the first empty slot from the one after the
/// last placed; once the slots run out the rest are dropped.
pub(crate) unsafe fn place_first_round_winners(this: usize) {
    let cup = Cup(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let places = read_i32(this + 0xb6); // the entrant list's slot count
    let mut k = 0i32;
    for i in 0..cup.team_count().max(0) as usize {
        let row = cup.team_table() + i * 6;
        if read_u8(row + 5) != 1 || k >= places {
            continue;
        }
        while k < places && read_usize(list + k as usize * 4) != 0 {
            k += 1;
        }
        if k < places {
            write_usize(list + k as usize * 4, read_usize(row)); // rx11_3: a first round winner into the first free slot
            k += 1;
        }
    }
}

/// The African progress codes (current, best, worst) of a nation at `stage` (signed byte) by the format byte
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`). The first round (stage −1; with format 2 its own bands): winner 1 at
/// best, the rest 25; the current and worst 21 (19 with format 2) for 1 and 25 for 2. A group stage (below 5 with
/// format 0, below 4 with 1 or 3 and others, below 5 with 2): the band tables below by position. The play-off with
/// format 2 (stage 5 and on) and with the other formats at stage 4: 1 stays 1 and 2 becomes 4 (format 2) or 3. Any
/// other stage, as they came.
pub(crate) fn african_progress_codes(format: u8, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let first_round = |one: u8| -> (u8, u8, u8) {
        let two = |v: u8, a: u8| match v {
            1 => a,
            2 => 0x19,
            _ => v,
        };
        (two(current, one), two(best, 1), two(worst, one))
    };
    let all = |c: &[u8], b: &[u8], w: &[u8]| (african_band(current, c), african_band(best, b), african_band(worst, w));
    let pair = |two: u8| {
        let f = |v: u8| match v {
            1 => 1,
            2 => two,
            _ => v,
        };
        (f(current), f(best), f(worst))
    };
    match format {
        0 => match stage {
            -1 => first_round(0x15),
            s if s < 5 => {
                let b = [1, 6, 0xb, 0x10, 0x15];
                all(&b, &b, &b) // rx11_4: five groups of five
            }
            _ => (current, best, worst),
        },
        1 => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let b = [1, 5, 9, 0xd, 0x11, 0x15];
                all(&b, &b, &b)
            }
            _ => (current, best, worst),
        },
        2 => match stage {
            -1 => first_round(0x13),
            s if s < 5 => all(&[1, 7, 0xd, 0x13], &[1, 7, 0xd, 0x13], &[4, 7, 0xd, 0x13]),
            _ => pair(4),
        },
        _ => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let others = [3, 5, 9, 0xd, 0x11, 0x15];
                all(&others, &[1, 5, 9, 0xd, 0x11, 0x15], &others)
            }
            4 => pair(3),
            _ => (current, best, worst),
        },
    }
}

/// A progress code through a band table: code k (1-based) becomes `bands[k − 1]`; a code outside the table stays.
pub(crate) fn african_band(code: u8, bands: &[u8]) -> u8 {
    match code {
        1..=6 if (code as usize) <= bands.len() => bands[code as usize - 1],
        _ => code,
    }
}

/// WcAfricanCup routines that build stages (thiscall, ret 0), called directly by advance_stage.
pub(crate) const FN_WC_AFRICAN_CUP_CREATE_GROUP_STAGES: usize = 0x9109e0;

pub(crate) const FN_WC_AFRICAN_CUP_CREATE_PLAYOFF_STAGE: usize = 0x910d20;
