// ---------- cx04_6: the Club World Championship's fate news text and club progress (world_club_champ.cpp) ----------

/// The Club World Championship's final-stage fate texts: winner, runner-up and third place.
const WORLD_CLUB_CHAMP_FINALS: [(u8, usize); 3] = [(1, 0x98732c), (2, 0x9872c8), (3, 0xad57a8)]; // "win" / "finish runners-up in" / "finish third placed in"

/// Writes the news text of a club's fate in the Club World Championship (`group_championship_fate_text`): through from
/// its group (stages below 2), "{club} qualify for final" (0xad580c); in the final stage (2) the winner, the runner-up
/// and the third-placed club (`WORLD_CLUB_CHAMP_FINALS`).
/// Original: `WorldClubChamp::build_fate_news_text` (0x9298a0, world_club_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn world_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    covered!("cx04_6 club world championship fate text");
    group_championship_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, 2, None, 0xad580c, &WORLD_CLUB_CHAMP_FINALS);
}

/// Stores a club's progress in the Club World Championship (`group_championship_progress`): in the group stages (below
/// 2) current 1→1, 2→5, 3→7, 4→0xa; best 1, 2→1, 3→7, 4→0xa; worst 1→4, 2→5, 3→7, 4→0xa.
/// Original: `WorldClubChamp::set_club_progress` (0x92ad50, world_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn world_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx04_6 club world championship progress");
    group_championship_progress(this, club, stage, current, best, worst, 2, [[1, 5, 7, 0xa], [1, 1, 7, 0xa], [4, 5, 7, 0xa]]);
}

/// Comparer for `world_club_champ_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`); the
/// group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if !group_championship_text_only(outcome, stage, news, text, 2) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| world_club_champ_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("world_club_champ_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("world_club_champ_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || world_club_champ_set_club_progress(this, club, stage, current, best, worst));
}
