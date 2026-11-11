use super::*;

/// rule: the FA Cup has 9 single-leg rounds with replays (+0x20 = 1, 10 days later) up to the quarter-finals:
/// preliminary (code 0xdc) drawn 9/10, played 16/10, 56 teams; 1st round 17/10 and 18/11; 2nd 19/11 and 9/12; 3rd 10/12
/// and 6/1; 4th 7/1 and 27/1; 5th 28/1 and 17/2; quarter-finals 18/2 and 10/3; semi-final 11/3 and 8/4 at its own
/// ground (venue 0x2a, no replay); final 9/4 and 12/5 at a neutral ground; the team counts from the 2nd to the 5th
/// round depend on the exemptions (`eng_fa_cup::EXEMPTION_COUNT`) and on the choice of entering at the 5th round
/// (`eng_fa_cup::FIFTH_ROUND_ENTRY` = 1); prize for winning a round from 10 thousand to 400 thousand, and the final 2
/// million to the winner and 1 million to the runner-up; category 4 up to the 5th round and 6 from the quarter-finals
/// Original: `EngFaCup::create_round_table` (0x56f320, eng_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    let t = fa_cup_rounds(read_i8(this.addr() + eng_fa_cup::EXEMPTION_COUNT) as i16, read_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY), this.team_count());
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &t, 0x9c42cc, 0xb6, "cql_10 tabela da copa da inglaterra")
}

/// Daily update of the FA Cup. Rule: the FA Cup gathers its entrants on 3 October each year (Date(3, 9): month 9 is
/// October; the community map says September), rebuilds the rounds and the tree and draws the preliminary round, once
/// per season (`.season_set_up()`); on 1 October 1999 it asked the human manager whether the club enters at the 5th
/// round (`eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` marks the question); then, every day, the cup update.
///
/// Ret 8; virtual +0x14 of the English cup; SEH frame for the news of the 1999 path, no destructor on the normal path:
/// `today.update_for_date(stage)`. With `.season_set_up()` = 0 and the stage different from 0: on Date(1, 9, 1999)
/// (month 9 is October) with `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 0, the question to the human manager about
/// entering at the 5th round (only in 1999: the original routine through the trampoline); on Date(3, 9, today's year)
/// (3 October), frees the tables, `.set_season_set_up(1)`, the entrants (`exe::engfacup_collect_entrants`, thiscall),
/// the rounds (virtual `comp_vtable::SET_UP_RULES`), the tree (virtual `cup_vtable::MAP_FIXTURE_TREE`) (a negative
/// return gives the box, lines 0x3d6, 0x3dd, 0x3e4, and 1), the predicted order (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and the draw of the 1st round (`exe::cup_set_up_round`: push 0; mov ecx, cup
/// -> thiscall, ret 4). Always, at the end, the cup update.
/// Original: `EngFaCup::update_for_date` (0x570550, eng_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if this.season_set_up() == 0 && stage != 0 {
        if is_date_today(make_date_dword(1, 9, 0x7cf, 0xffff_ffff), today) && *((this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) as *const u8) == 0 {
            // simplification: the 1999 question builds and delivers a news item (only on that day, outside any new career)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_FA_CUP_UPDATE_FOR_DATE)(this.addr(), today, stage);
        }
        // simplification: the year goes in ax with the rest of eax in the high bytes; 0x549f70 reads it as a word
        if is_date_today(make_date_dword(3, 9, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today) { // cqn10: the FA Cup entries on 3/10
            free_season_tables(this);
            covered!("cqn_10 fa cup entries");
            this.set_season_set_up((1) as usize);
            if exe::engfacup_collect_entrants(this) < 0 {
                show_error_at(0x9c42cc, 0x3d6);
                return 1;
            }
            if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
                show_error_at(0x9c42cc, 0x3dd);
                return 1;
            }
            if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
                show_error_at(0x9c42cc, 0x3e4);
                return 1;
            }
            call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS);
            exe::cup_set_up_round(this, 0);
        }
    }
    eng_cc_cup_cup_update_for_date(this, today, stage)
}
