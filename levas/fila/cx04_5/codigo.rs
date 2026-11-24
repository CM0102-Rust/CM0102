// ---------- cx04_5: the Oceania Club Championship's fate news text and club progress (oceania_club_champ.cpp) ----------

/// The final-stage fate text shared by the Oceania Club Championship and the Club World Championship: the template of
/// `finals` for the outcome, if any, with the club's and the competition's short names (`european_cup_fate_text`, the
/// competition record at +4); with no template for the outcome the original still hands the Text the shared buffer
/// 0xde1f64 as it is (stale text, a defect of the original) and adds the club's (0x7d5) and the competition's (0x7d0)
/// links.
unsafe fn final_stage_fate_text(this: usize, club_id: u32, outcome: u8, finals: &[(u8, usize)], text: usize) {
    let comp = read_usize(this + 4);
    if let Some(&(_, format)) = finals.iter().find(|f| f.0 == outcome) {
        return european_cup_fate_text(text, club_id, format, read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
    }
    let club = club_at(club_id as isize);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The fate text of a club that goes through its group (`format`: "{club} qualify for the semi final/final"), with the
/// club's short name (gender byte +0x52) and link (0x7d5).
unsafe fn group_qualify_text(club_id: u32, format: usize, text: usize) {
    covered!("cx04_5 group qualify text");
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
}

/// The fate news text of the two club championships run in groups and a final stage (Oceania, Club World): in the group
/// stages (signed stage below `groups`) outcome 6 (through) with `news` the group news
/// (`exe::comp_util_write_group_qualify_news`(club_id, a4, competition, 6, news, text)), without it `qualify`; outcome
/// 0xfe (out) the group news with −2; at stage `middle` (Oceania's 1) the base text
/// (`exe::comp_build_fate_news_text` with a7 = 0); at the final stage (`groups`), with `news` the cup tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club_id, fixture, a8, outcome, a6, text)), without it
/// `final_stage_fate_text`. Other stages and outcomes: nothing.
#[allow(clippy::too_many_arguments)]
unsafe fn group_championship_fate_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a8: u32, news: u32, text: usize,
        groups: i8, middle: Option<i8>, qualify: usize, finals: &[(u8, usize)]) {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = read_usize(this + 4);
    if s < groups {
        if d == 6 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            } else {
                group_qualify_text(club_id, qualify, text);
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text);
        }
    } else if Some(s) == middle {
        exe::comp_build_fate_news_text(this, club_id, outcome, s as i32, a4, fixture, a6, 0, a8, news, text);
    } else if s == groups {
        if news != 0 {
            exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
        } else {
            covered!("cx04_5 club championship final text");
            final_stage_fate_text(this, club_id, d, finals, text);
        }
    }
}

/// Whether `group_championship_fate_text` only writes the text on this call (what its comparers compare side by side).
fn group_championship_text_only(outcome: u32, stage: u32, news: u32, text: usize, groups: i8) -> bool {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    text != 0 && news == 0 && ((s < groups && d == 6) || s == groups)
}

/// The club championships' progress codes: each of the three codes (current, best, worst) passes through its table when
/// it is 1 to 4 and the stage (signed) is below `groups`; the bytes go to `comp_stats_club::BEST_POSITION`,
/// `CURRENT_POSITION` and `WORST_POSITION` of the club's entry (`exe::comp_stats_find_club_entry` on the statistics,
/// not checked for null), in that order.
unsafe fn group_championship_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, groups: i8, tables: [[u8; 4]; 3]) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let code = |p: u32, t: &[u8; 4]| {
        let p = p as u8;
        if (stage as u8 as i8) < groups && (1..=4).contains(&p) { t[p as usize - 1] } else { p }
    };
    write_u8(e + comp_stats_club::BEST_POSITION, code(best, &tables[1]));
    write_u8(e + comp_stats_club::CURRENT_POSITION, code(current, &tables[0]));
    write_u8(e + comp_stats_club::WORST_POSITION, code(worst, &tables[2]));
}

/// The comparer of the club championships' progress: side by side on the 3 bytes of the entry; only the original
/// without an entry.
unsafe fn compare_group_championship_progress(name: &str, this: usize, club: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let stats = League(this).comp_stats();
    let e = if stats != 0 && club != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}, club {:#x}): original {:x?} rust {:x?}", name, this, club, mo, mr));
}

/// The Oceania Club Championship's fate texts: groups before stage 1, the base text at stage 1, the final at stage 2.
const OCEANIA_FINALS: [(u8, usize); 2] = [(1, 0x98732c), (2, 0x9872c8)]; // "win" / "finish runners-up in"

/// Writes the news text of a club's fate in the Oceania Club Championship (`group_championship_fate_text`): through
/// from its group, "{club} qualify for semi final" (0xa6a0ac); at stage 1 the base text; in the final stage the winner
/// (0x98732c) and the runner-up (0x9872c8).
/// Original: `OceaniaClubChamp::build_fate_news_text` (0x7976b0, oceania_club_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_championship_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, 1, Some(1), 0xa6a0ac, &OCEANIA_FINALS);
}

/// Stores a club's progress in the Oceania Club Championship (`group_championship_progress`): in the group stage
/// (stage below 1) current 1→1, 2→3, 3→5, 4→7; best 1, 2→1, 3→5, 4→7; worst 1, 2→3, 3→5, 4→7.
/// Original: `OceaniaClubChamp::set_club_progress` (0x7987a0, oceania_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx04_5 oceania club progress");
    group_championship_progress(this, club, stage, current, best, worst, 1, [[1, 3, 5, 7], [1, 1, 5, 7], [3, 3, 5, 7]]);
}

/// Comparer for `oceania_club_champ_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`);
/// the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if !group_championship_text_only(outcome, stage, news, text, 1) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| oceania_club_champ_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("oceania_club_champ_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("oceania_club_champ_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || oceania_club_champ_set_club_progress(this, club, stage, current, best, worst));
}
