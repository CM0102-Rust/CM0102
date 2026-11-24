// ---------- cx08_13: the Australian league's fate news text, club progress and bench (aus_nsl.cpp) ----------

/// Rule: the news of a club's fate in the Australian league: through from the regular season, "{club} through to
/// playoff phase"; through from the finals series (stage 0), "{club} through to final"; in the grand final, the cup
/// lines of a tie.
///
/// `AusNsl::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`: at the stage byte 0xff
/// or 0 only outcome 6 writes: with `body` the group news (`exe::comp_util_write_group_qualify_news`(club, a4, the
/// competition record, 6, body, text)), without it `group_qualify_text` with 0x987c88 (0xff) or 0x987c58 (0). Any other
/// stage: with `body` `exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6, text), without it
/// `exe::comp_util_write_cup_win_or_defeat_news`(club, fixture, outcome, text).
/// Original: `AusNsl::build_fate_news_text` (0x411a30, aus_nsl.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, body: u32, text: usize) {
    let format = match stage as u8 {
        0xff => 0x987c88, // "{club} through to playoff phase"
        0 => 0x987c58, // "{club} through to final"
        _ => {
            if body != 0 {
                return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
            }
            return exe::comp_util_write_cup_win_or_defeat_news(club_id, fixture as u32, outcome, text);
        }
    };
    if outcome as u8 != 6 {
        return;
    }
    if body != 0 {
        return exe::comp_util_write_group_qualify_news(club_id, a4, read_usize(this + league::COMPETITION), 6, body, text);
    }
    covered!("cx08_13 australian qualify text");
    group_qualify_text(club_id, format, text);
}

/// Comparer for `aus_nsl_build_fate_news_text`: the text-only paths (outcome 6 without `body` at stage −1 or 0) side by
/// side (`compare_text_writer`); the news writers only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I0%%);
    if text == 0 || body != 0 || outcome as u8 != 6 || !matches!(stage as u8, 0xff | 0) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| aus_nsl_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("aus_nsl_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// Rule: the Australian league's progress codes: in the regular season the top two become 1 (best and current) or 7
/// (worst) and third 9; in the finals series (stages below 1) places 1 and 2 become 1 (current, best) or 3 (worst), 3
/// becomes 5 and 4 becomes 7; in the grand final as they come.
///
/// `AusNsl::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`,
/// unchecked). Stage byte 0xff: current 1, 2 → 1 and 3 → 9; best 1, 2 → 1 and 3 → 9; worst 1, 2 → 7 and 3 → 9. Other
/// stages below 1 (signed; the jump tables 0x4124ac, 0x4124bc, 0x4124cc): current 1, 2 → 1, 3 → 5, 4 → 7; best the same;
/// worst 1, 2 → 3, 3 → 5, 4 → 7. Any other code, and every code from stage 1, stays. The bytes go to
/// `comp_stats_club::BEST_POSITION`, `CURRENT_POSITION` and `WORST_POSITION`.
/// Original: `AusNsl::set_club_progress` (0x412380, aus_nsl.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let s = stage as u8;
    let code = |v: u32, table: [u8; 4]| -> u8 {
        let v = v as u8;
        if (1..=4).contains(&v) && table[v as usize - 1] != 0 { table[v as usize - 1] } else { v }
    };
    let (c, b, w) = if s == 0xff {
        covered!("cx08_13 australian season progress");
        (code(current, [1, 1, 9, 0]), code(best, [1, 1, 9, 0]), code(worst, [7, 7, 9, 0]))
    } else if (s as i8) < 1 {
        covered!("cx08_13 australian finals progress");
        (code(current, [1, 1, 5, 7]), code(best, [1, 1, 5, 7]), code(worst, [3, 3, 5, 7]))
    } else {
        (current as u8, best as u8, worst as u8)
    };
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `aus_nsl_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("aus_nsl_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || aus_nsl_set_club_progress(this, club, stage, current, best, worst));
}

/// Rule: the Australian league's bench: a fixture of a stage (the finals series and the grand final) names 5
/// substitutes and allows 3; a league fixture takes the league's numbers.
///
/// `AusNsl::set_fixture_substitutes(fixture)`: with the fixture's `fixture::STAGE_INDEX` (signed) at 0 or above, the
/// low byte of `fixture::SQUAD_RULES` becomes 0x35 (the word & 0xff35 | 0x35); otherwise the league's
/// `league::SUBSTITUTES_NAMED` (& 0xf) into the low nibble and `league::SUBSTITUTIONS_ALLOWED` (& 0xf) into the next,
/// the other bits kept.
/// Original: `AusNsl::set_fixture_substitutes` (0x4124e0, aus_nsl.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let (l, fixture) = (League(this), Fixture(fixture));
    if fixture.stage_index() as i8 >= 0 {
        covered!("cx08_13 australian stage bench");
        return fixture.set_squad_rules((fixture.squad_rules() & 0xff35) | 0x35);
    }
    fixture.set_squad_rules((fixture.squad_rules() & 0xfff0) | (l.substitutes_named() & 0xf) as u16);
    fixture.set_squad_rules((fixture.squad_rules() & 0xff0f) | ((l.substitutions_allowed() & 0xf) as u16) << 4);
}

/// Comparer for `aus_nsl_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word side by side.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture + fixture::SQUAD_RULES, 2)], || o(this, fixture), || aus_nsl_set_fixture_substitutes(this, fixture));
    SCOREBOARD.record(same, || format!("aus_nsl_set_fixture_substitutes({:#x}): original {:x?} rust {:x?}", fixture, mo, mr));
}
