// ---------- rx09_19: the Asian World Cup qualifying staff records, fate news text and progress codes (wc_asia_league.cpp) ----------

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each).
pub(crate) unsafe fn world_cup_record() -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b))
}

/// A nation through to the World Cup from the Asian qualifying: the World Cup qualification record (weight 100), its
/// place in the World Cup (`exe::worldcup_add_qualified_team` on the World Cup object, [`G_COMP_OBJECT_TABLE`]), `*out`
/// = 4 (not checked for null) and NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`].
pub(crate) unsafe fn wc_asia_qualify_nation(club: usize, out: usize) {
    european_cup_add_qualified_record(club, world_cup_record(), 0, 0, 0x64);
    let world_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    exe::worldcup_add_qualified_team(world_cup, club); // rx09_19: the Asian nation enters the World Cup
    write_u8(out, 4);
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_ADD_QUALIFIED_NATION);
    add(read_usize(G_NATIONAL_TEAMS), club, world_cup_record());
}

/// A nation out of the Asian qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record` with
/// `stage` and `weight`) and the national team's campaign ended (`clear_national_team_competition`).
pub(crate) unsafe fn wc_asia_eliminate(this: usize, club: usize, stage: u32, weight: u32) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), stage, weight);
    clear_national_team_competition(Club(club));
}

/// What happens to a nation by its Asian qualifying fate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFate {
    /// The qualified-for-round record with (stage, round, weight).
    Qualified(u32, u32, u32),
    /// The failed-to-qualify record with (stage, weight) and the end of the campaign.
    Out(u32, u32),
    /// Through to the World Cup (`wc_asia_qualify_nation`); the result −1.
    WorldCup,
    Nothing,
}

/// The fate of a nation with `outcome` (its byte) at `stage` (signed) of the Asian qualifying (`exempt`, the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (below 9): 6 through (0x14, 0, 0x1e), 0xfe out (0xa, 0x1e);
/// - the second round (9, 10): 7 to the World Cup, 1 to the third round (0x1e, 0x96, 0x3c), 0xfe out (0x14, 0x3c);
/// - the third round (11): with 1 or 3 exempt nations, 2 to the play-off (0xa0, 0, 0x46) and any other to the World
///   Cup; with none or 2, 2 out (0x1e, 0x3c) and any other to the play-off;
/// - the play-off (12): 1 to the World Cup, any other out (0xa0, 0x3c);
/// - later: nothing.
pub(crate) fn wc_asia_fate(stage: i8, outcome: u8, exempt: u8) -> AsianFate {
    match stage {
        i8::MIN..=8 => match outcome {
            6 => AsianFate::Qualified(0x14, 0, 0x1e),
            0xfe => AsianFate::Out(0xa, 0x1e),
            _ => AsianFate::Nothing,
        },
        9 | 10 => match outcome {
            7 => AsianFate::WorldCup,
            1 => AsianFate::Qualified(0x1e, 0x96, 0x3c),
            0xfe => AsianFate::Out(0x14, 0x3c),
            _ => AsianFate::Nothing,
        },
        11 if exempt != 0 && exempt != 2 => if outcome == 2 { AsianFate::Qualified(0xa0, 0, 0x46) } else { AsianFate::WorldCup },
        11 => if outcome == 2 { AsianFate::Out(0x1e, 0x3c) } else { AsianFate::Qualified(0xa0, 0, 0x46) },
        12 => if outcome == 1 { AsianFate::WorldCup } else { AsianFate::Out(0xa0, 0x3c) }, // rx09_19: the play-off winner goes to the World Cup
        _ => AsianFate::Nothing,
    }
}

/// Updates the staff records by a nation's fate in the Asian qualifying (`wc_asia_fate`; the records with this
/// league's competition). A nation through to the World Cup returns −1; any other fate returns the exempt count
/// (`WC_ASIA_EXEMPT`, sign-extended). The last two arguments are not read.
/// Original: `WcAsiaLeague::update_staff_records_for_status` (0x912500, wc_asia_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, _game: usize, _a6: u32) -> u32 {
    match wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT)) {
        AsianFate::Qualified(s, r, w) => european_cup_add_qualified_record(club, League(this).competition().addr(), s, r, w),
        AsianFate::Out(s, w) => wc_asia_eliminate(this, club, s, w),
        AsianFate::WorldCup => {
            covered!("rx09_19 asian nation to the world cup");
            wc_asia_qualify_nation(club, out);
            return u32::MAX;
        }
        AsianFate::Nothing => {}
    }
    read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32
}

/// Comparer for `wc_asia_league_update_staff_records_for_status`: the records and the World Cup place do not run twice,
/// so only the original runs; the plan: the result and, for a nation through, `*out` = 4.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let fate = wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I0%%)(this, club, outcome, stage, out, game, a6);
    let expected = if fate == AsianFate::WorldCup { u32::MAX } else { read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32 };
    let out_ok = fate != AsianFate::WorldCup || read_u8(out) == 4;
    SCOREBOARD.record(r == expected && out_ok, || format!("wc_asia_league_update_staff_records_for_status({:#x}, outcome {:#x}, stage {}) original={:#x} plan {:?}", club, outcome as u8,
        stage as u8 as i8, r, fate));
    r
}

/// What `wc_asia_league_build_fate_news_text` writes for a nation (the cup and group news are the shared routines').
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFateText {
    /// `exe::comp_util_write_group_qualify_news`(club, a4, the competition, the outcome or −2, the news item or 0, text).
    Group(i32, u32),
    /// `exe::comp_util_write_cup_tie_result_news`(club, a5, a8, outcome, a6, text).
    CupTie,
    /// A club text (`play_off_club_text`) with this format.
    Club(usize),
    /// A club and World Cup text (`club_and_competition_short_text` with the World Cup's record) with this format.
    WorldCup(usize),
    Nothing,
}

/// The text of a nation's fate in the Asian qualifying:
/// - the first round (below 9): with a news item the group news (outcome, news); without, 6 gives 0xad47bc "{nation}
///   qualify for second round", 0xfe the group news with −2 and 0;
/// - the second round (9, 10): with a news item the group news; without, 7 gives 0xad4738 "{nation} qualify for {World
///   Cup}", 1 0xad4780 "{nation} qualify for third round", 0xfe the group news with −2 and 0;
/// - the third round (11): with a news item the cup tie news; without, when `a7` is 1, outcome 1 "qualify for {World
///   Cup}" and any other 0xad46c0 "{nation} out of {World Cup}"; otherwise outcome 1 0xad4700 "{nation} qualify for
///   playoff" and any other "out of {World Cup}";
/// - the play-off (12): with a news item the cup tie news; without, 1 "qualify for {World Cup}", any other "out of
///   {World Cup}";
/// - later: nothing.
pub(crate) fn wc_asia_fate_text(stage: i8, outcome: u8, a7: u8, news: u32) -> AsianFateText {
    let group_out = |o: u8| if o == 0xfe { AsianFateText::Group(-2, 0) } else { AsianFateText::Nothing };
    match stage {
        i8::MIN..=8 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        i8::MIN..=8 => if outcome == 6 { AsianFateText::Club(0xad47bc) } else { group_out(outcome) },
        9 | 10 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        9 | 10 => match outcome {
            7 => AsianFateText::WorldCup(0xad4738),
            1 => AsianFateText::Club(0xad4780),
            o => group_out(o),
        },
        11 | 12 if news != 0 => AsianFateText::CupTie,
        11 if a7 != 1 => if outcome == 1 { AsianFateText::Club(0xad4700) } else { AsianFateText::WorldCup(0xad46c0) },
        11 | 12 => if outcome == 1 { AsianFateText::WorldCup(0xad4738) } else { AsianFateText::WorldCup(0xad46c0) }, // rx09_19: qualify for / out of the World Cup
        _ => AsianFateText::Nothing,
    }
}

/// The fate news text of an Asian qualifying nation (thiscall, ret 0x28; the arguments of Comp::build_fate_news_text):
/// `wc_asia_fate_text` with the outcome byte, the stage (signed) and the a7 byte. The group news keeps the full outcome
/// argument when it passes it on (the −2 is a constant).
/// Original: `WcAsiaLeague::build_fate_news_text` (0x912780, wc_asia_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let comp = League(this).competition().addr();
    match wc_asia_fate_text(stage as u8 as i8, outcome as u8, a7 as u8, news) {
        AsianFateText::Group(-2, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, 0, text),
        AsianFateText::Group(_, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, outcome as i32, news, text),
        AsianFateText::CupTie => exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text),
        AsianFateText::Club(format) => {
            covered!("rx09_19 asian round text");
            play_off_club_text(club_id, format, text);
        }
        AsianFateText::WorldCup(format) => {
            covered!("rx09_19 asian world cup text");
            club_and_competition_short_text(club_id, world_cup_record(), format, true, text);
        }
        AsianFateText::Nothing => {}
    }
}

/// Comparer for `wc_asia_league_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the shared group and cup news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let s = stage as u8 as i8;
    let own = matches!(wc_asia_fate_text(s, outcome as u8, a7 as u8, news), AsianFateText::Club(_) | AsianFateText::WorldCup(_));
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| wc_asia_league_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("wc_asia_league_build_fate_news_text(club {}, outcome {:#x}, stage {}, a7 {}, news {})", club_id, outcome as u8, s, a7 as u8, news));
}

/// The three progress codes (current, best, worst) of an Asian qualifying nation at a stage (bytes; `exempt` the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (−1 to 8): best 1, 2, 3, 4 become 1, 11, 21, 31; worst and current 1, 2, 3, 4 become 9, 11, 21, 31
///   (the group winner's current code is 9, as its worst);
/// - the second round (9, 10): best 1 and 2 become 1, then 3, 4, 5 become 5, 7, 9; worst and current 1 and 2 become 4,
///   then 5, 7, 9;
/// - the third round (11): with no exempt nation best 1, worst and current below 4 become 1; with one, best below 4 and
///   worst and current below 3 become 1; with two, best and worst and current below 3, 2 and 2 become 1; otherwise best
///   below 2 becomes 1;
/// - any other code, and the other stages (below −1, or 12 on): as it came.
pub(crate) fn wc_asia_progress(stage: i8, exempt: u8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let map = |v: u8, table: &[u8]| if (1..=table.len()).contains(&(v as usize)) { table[v as usize - 1] } else { v };
    let below = |v: u8, n: u8| if v < n { 1 } else { v };
    match stage {
        -1..=8 => (map(current, &[9, 0xb, 0x15, 0x1f]), map(best, &[1, 0xb, 0x15, 0x1f]), map(worst, &[9, 0xb, 0x15, 0x1f])), // rx09_19: the first round codes
        9 | 10 => (map(current, &[4, 4, 5, 7, 9]), map(best, &[1, 1, 5, 7, 9]), map(worst, &[4, 4, 5, 7, 9])),
        11 => match exempt {
            0 => (below(current, 4), 1, below(worst, 4)),
            1 => (below(current, 3), below(best, 4), below(worst, 3)),
            2 => (below(current, 2), below(best, 3), below(worst, 2)),
            _ => (current, below(best, 2), worst),
        },
        _ => (current, best, worst),
    }
}

/// Stores an Asian qualifying nation's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the overall codes by `wc_asia_progress`.
/// Original: `WcAsiaLeague::set_club_progress` (0x914730, wc_asia_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_19 asian progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = wc_asia_progress(stage as u8 as i8, read_u8(this + WC_ASIA_EXEMPT), current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `wc_asia_league_set_club_progress`: the three position bytes of the nation's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || wc_asia_league_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("wc_asia_league_set_club_progress({:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", club, stage as u8 as i8, current as u8, best as u8,
        worst as u8, mo, mr));
}
