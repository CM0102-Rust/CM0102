// ---------- rx12_18: the Belgian Third Division: the fate news text and the club list (bel_third.cpp) ----------

/// The fate news templates of the Belgian Third Division (langlib, `.rdata`).
pub(crate) mod bel_third_fate_texts {
    /// "{}<%s - Team Name (e.g. Falkirk)>{} win {}<%s - Competition Name(e.g.First Division)>{}"
    pub(crate) const WIN_COMPETITION: usize = 0x989da4;
    /// "{}<%s - Team Name(e.g.Brighton)>{} avoid relegation"
    pub(crate) const AVOID_RELEGATION: usize = 0x987594;
    /// "{}<%s - Team Name(e.g.Brighton)>{} win promotion"
    pub(crate) const WIN_PROMOTION: usize = 0x989c18;
    /// "{}<%s - Team Name(e.g.Falkirk)>{} relegated"
    pub(crate) const RELEGATED_FROM_PLAYOFF: usize = 0x989d78;
    /// "{}<%s - Team Name(e.g.Falkirk)>{} miss out on promotion"
    pub(crate) const MISS_OUT_ON_PROMOTION: usize = 0x989d40;
    /// "{}<%s - Team Name(e.g.Brighton)>{} relegated"
    pub(crate) const RELEGATED: usize = 0x987564;
}

/// rule: the Belgian Third Division fate news: the title decider (stage 7) "win" the competition; the promotion
/// play-off (stage 8) "win promotion" or "miss out on promotion", or, when the play-off is against a Second Division
/// club, "avoid relegation" or "relegated"; the relegation play-off (stage 9) "avoid relegation", "relegated" or the
/// shared win-or-defeat sentence; the league stages (below 7) go to the Comp base
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), four values passed on, `full_sentence` and
/// the Text; `a7` is the id of the other club's competition [inference: from the play-off's caller]).
///
/// Stage below 7: the base (`exe::comp_build_fate_news_text` with every argument). Stages 7, 8 and 9 with
/// `full_sentence`: the shared tie sentence (`exe::comp_util_write_cup_tie_result_news`(club_index, a5, a8, the fate
/// dword, a6, text)). Otherwise stage 7: "win" the competition (its short gender and short name), the club's link and
/// the competition's; stage 8: fate 1 "avoid relegation" when `a7` is the Belgian Second Division's id, else "win
/// promotion"; any other fate "relegated" or "miss out on promotion" in the same way; stage 9: fate 1 "avoid
/// relegation", fate 2 or 0xff "relegated", any other fate the shared win-or-defeat sentence
/// (`exe::comp_util_write_cup_win_or_defeat_news`(club_index, a5, the fate dword, text)); each line with the club's
/// link. Any other stage, nothing.
/// Original: `BelThird::build_fate_news_text` (0x423310, bel_third.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn bel_third_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use bel_third_fate_texts as t;
    let s = stage as u8 as i8;
    if s < 7 {
        covered!("rx12_18 belgian third league fate");
        return exe::comp_build_fate_news_text(this, club_index, fate, stage as i32, code, a5 as usize, a6, a7, a8, full_sentence, text);
    }
    if s > 9 {
        return;
    }
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    covered!("rx12_18 belgian third play-off fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let club = club_of_index(club_index);
    let through = fate as u8 == 1;
    let template = match s {
        7 => {
            let comp = League(this).competition().addr();
            qualifying_fate_line(text, t::WIN_COMPETITION, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::SHORT_NAME)));
            link_fate_club(text, club);
            return link_fate_competition(text, comp);
        }
        8 => {
            let against_second = a7 == read_u32(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION); // rx12_18: a play-off with a Second Division club
            match (through, against_second) {
                (true, true) => t::AVOID_RELEGATION,
                (true, false) => t::WIN_PROMOTION,
                (false, true) => t::RELEGATED_FROM_PLAYOFF,
                (false, false) => t::MISS_OUT_ON_PROMOTION,
            }
        }
        _ => match fate as u8 {
            1 => t::AVOID_RELEGATION,
            2 | 0xff => t::RELEGATED,
            _ => return exe::comp_util_write_cup_win_or_defeat_news(club_index, a5, fate, text),
        },
    };
    qualifying_fate_line(text, template, club, None);
    link_fate_club(text, club);
}

/// Comparer for `bel_third_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_bel_third_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| bel_third_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("bel_third_build_fate_news_text(club {}, fate {}, stage {}, a7 {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, a7, full_sentence));
}

/// The 32 clubs of the Belgian Third Division's two groups in table order: group A's (the league's own table) at 0 to
/// its `club_count` − 1, then group B's (the table of `stages()`[0]) from there.
unsafe fn bel_third_clubs(this: League) -> Vec<(usize, usize)> {
    let n = this.club_count();
    let group_b = League(read_usize(this.stages()));
    let mut v = Vec::new();
    for i in 0..n.max(0) as usize {
        v.push((i, read_usize(this.table() + i * league_table_row::SIZE)));
    }
    // the original walks group B with group A's count too (both have 16 clubs)
    for j in 0..n.max(0) as usize {
        v.push((n as usize + j, read_usize(group_b.table() + j * league_table_row::SIZE)));
    }
    v
}

/// A new array of the Belgian Third Division's clubs and its count: the count is 32 whatever the groups hold; the array
/// of 0x80 bytes (`exe::crt_malloc`; out of memory, the box of bel_third.cpp line 0x69b and the count 0) holds group A's
/// clubs and then group B's (`bel_third_clubs`; group B is walked with group A's count, the same 16 in the game).
/// Original: `BelThird::get_club_array` (0x423f00, bel_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_third_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    let this = League(this);
    *n = 0x20; // rx12_18: the count of the two groups
    let p = exe::crt_malloc(0x80);
    *out = p;
    if p == 0 {
        show_error_at(0x989d04, 0x69b);
        *n = 0;
        return;
    }
    covered!("rx12_18 belgian third clubs");
    for (k, club) in bel_third_clubs(this) {
        write_usize(p + 4 * k, club);
    }
}

/// Comparer for `bel_third_get_club_array`: only the original runs (it allocates the list the caller frees); the count
/// must be 32 and the list hold `bel_third_clubs`.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I1%%)(this, out, n);
    if *out == 0 {
        return;
    }
    let plan = bel_third_clubs(League(this));
    let got: Vec<(usize, usize)> = plan.iter().map(|&(k, _)| (k, read_usize(*out + 4 * k))).collect();
    SCOREBOARD.record(*n == 0x20 && got == plan, || format!("bel_third_get_club_array({:#x}) count {}: original {:x?} plan {:x?}", this, *n, got, plan));
}
