// ---------- batch %%LEVA%%: the board's verdict on a match (manager_manager.cpp) ----------

/// The argument shapes of the board's match verdicts (the format call of each case of
/// `build_board_match_reaction_text`): who comes first and whether the score is quoted.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BoardVerdictArgs {
    /// friendly: the opponent's gender twice, the board, the score and the opponent's name
    Friendly,
    /// the opponent and the competition, the board, the opponent's name and the competition's short name
    OpponentInComp,
    /// as `OpponentInComp`, with the score after the board
    OpponentInCompWithScore,
    /// the competition first: its gender, the opponent's, the board, the score, the competition's short name and the
    /// opponent's name
    CompThenOpponentWithScore,
}

/// The verdict template of a match by its marks: the friendly template (form `Friendly`) when the case asks whether
/// the competition is a friendly, the other template and its argument shape; `None` for the marks without a case of
/// their own (the 0x800 / 0x400 / plain defaults).
fn board_verdict_template(marks: i32) -> Option<(Option<usize>, usize, BoardVerdictArgs)> {
    use BoardVerdictArgs::*;
    Some(match marks {
        0x401 | 0x501 => (Some(0xa0f5f4), 0xa0f548, OpponentInComp),
        0x402 | 0x502 => (Some(0xa0f8ac), 0xa0f7f8, OpponentInComp),
        0x408 | 0x4a0 | 0x608 | 0x6a0 => (Some(0xa0eab4), 0xa0ea10, OpponentInComp),
        0x420 | 0x620 => (Some(0xa0f204), 0xa0f164, OpponentInComp),
        0x441 | 0x541 => (Some(0xa0f750), 0xa0f694, OpponentInComp),
        0x442 | 0x542 => (Some(0xa0fa34), 0xa0f958, OpponentInCompWithScore),
        0x444 | 0x544 => (Some(0xa0fbac), 0xa0fad8, OpponentInCompWithScore),
        0x448 | 0x648 => (Some(0xa0f350), 0xa0f2a0, OpponentInComp),
        0x450 | 0x650 => (Some(0xa0ee98), 0xa0ede4, OpponentInComp),
        0x460 | 0x660 => (Some(0xa0ec04), 0xa0eb54, OpponentInComp),
        0x481 | 0x581 => (Some(0xa0f49c), 0xa0f3f0, OpponentInComp),
        0x482 | 0x582 => (Some(0xa0f5f4), 0xa0ef3c, OpponentInComp), // the friendly text of 0x401
        0x484 | 0x584 => (Some(0xa0f0bc), 0xa0efec, OpponentInCompWithScore),
        0x488 | 0x688 => (Some(0xa0ed4c), 0xa0ecac, OpponentInComp),
        0x808 | 0xa08 => (None, 0xa0e6cc, CompThenOpponentWithScore),
        0x810 | 0xa10 => (None, 0xa0e168, CompThenOpponentWithScore),
        0x820 | 0xa20 => (None, 0xa0e350, CompThenOpponentWithScore),
        0x848 | 0xa48 => (Some(0xa0e62c), 0xa0e55c, OpponentInCompWithScore),
        0x850 | 0xa50 => (Some(0xa0e2c8), 0xa0e210, OpponentInCompWithScore),
        0x860 | 0xa60 => (Some(0xa0e4c8), 0xa0e404, OpponentInCompWithScore),
        0x888 | 0xa88 => (Some(0xa0e0d0), 0xa0e008, OpponentInCompWithScore),
        0x890 | 0xa90 => (Some(0xa0de38), 0xa0dd88, OpponentInCompWithScore),
        0x8a0 | 0xaa0 => (Some(0xa0df7c), 0xa0dec0, OpponentInCompWithScore),
        0x948 => (None, 0xa0e86c, OpponentInCompWithScore),
        0x988 => (None, 0xa0e790, OpponentInCompWithScore),
        0x1001 | 0x1101 => (Some(0xa0d030), 0xa0cf74, OpponentInCompWithScore),
        0x1002 | 0x1102 => (None, 0xa0d3cc, CompThenOpponentWithScore),
        0x1008 => (Some(0xa0dcd4), 0xa0dc08, OpponentInCompWithScore),
        0x1020 | 0x1120 => (Some(0xa0d85c), 0xa0d794, OpponentInCompWithScore),
        0x1041 | 0x1141 => (Some(0xa0d314), 0xa0d244, OpponentInCompWithScore),
        0x1042 | 0x1142 => (Some(0xa0d560), 0xa0d490, OpponentInCompWithScore),
        0x1044 | 0x1144 => (Some(0xa0d6f0), 0xa0d624, OpponentInCompWithScore),
        0x1048 | 0x1148 => (Some(0xa0db68), 0xa0da90, OpponentInCompWithScore),
        0x1060 | 0x1160 => (Some(0xa0d9dc), 0xa0d908, OpponentInCompWithScore),
        0x1081 | 0x1181 => (Some(0xa0d194), 0xa0d0d0, OpponentInCompWithScore),
        0x1082 | 0x1182 => (Some(0xa0ceb4), 0xa0cdec, OpponentInCompWithScore),
        0x1084 | 0x1184 => (Some(0xa0c9b8), 0xa0c8f4, OpponentInCompWithScore),
        0x1088 | 0x1188 => (Some(0xa0cd48), 0xa0cc70, OpponentInCompWithScore),
        0x10a0 | 0x11a0 => (Some(0xa0cb30), 0xa0ca60, OpponentInCompWithScore),
        0x1208 => (None, 0xa0cbd0, OpponentInComp),
        0x1220 => (None, 0xa0c854, OpponentInComp),
        0x1248 => (None, 0xa0c780, OpponentInComp),
        0x1260 => (None, 0xa0c610, OpponentInComp),
        0x1288 => (None, 0xa0c6b8, OpponentInComp),
        0x12a0 => (None, 0xa0c560, OpponentInComp),
        _ => return None,
    })
}

/// Formats one board verdict (`story_lang!`) into `out` with the argument shape of its case.
unsafe fn format_board_verdict(out: usize, template: usize, shape: BoardVerdictArgs, other: Club, comp: Competition, board: usize, score: usize) {
    let go = other.gender_name() as i32;
    let gc = comp.short_gender_name() as i8 as i32;
    let (name, short) = (other.name_addr(), comp.short_name_addr());
    match shape {
        BoardVerdictArgs::Friendly => story_lang!(out, template, go, go, board, score, name),
        BoardVerdictArgs::OpponentInComp => story_lang!(out, template, go, go, gc, gc, board, name, short),
        BoardVerdictArgs::OpponentInCompWithScore => story_lang!(out, template, go, go, gc, gc, board, score, name, short),
        BoardVerdictArgs::CompThenOpponentWithScore => story_lang!(out, template, gc, gc, go, go, board, score, short, name),
    }
}

/// The board's sentence on the league run after the verdict (0 when there is none): `run` 2 (a winning run) or −2 (a
/// losing run), the board's target, the club's current and best positions.
unsafe fn board_run_sentence(this: usize, club: Club, run: i8, target: i8, current: i8, best: i8) -> usize {
    let (target, current) = (target as i32, current as i32);
    let flags = read_u16(board_row(this, club) + board_record::EXPECTATION_FLAGS);
    // the first expectation flag (in this order) picks the sentence of the run of victories / of defeats
    let by_expectation = |sentences: [usize; 7]| {
        let bits = [1u16, 2, 4, 8, 0x40, 0x20];
        bits.iter().position(|&b| flags & b != 0).map_or(sentences[6], |k| sentences[k])
    };
    if run == 2 {
        if target < current {
            return 0;
        }
        if current > target + 4 {
            return 0xa0c514; // "Hopefully this run of results indicates a change in the club's fortunes."
        }
        if current > target - 4 {
            return 0xa0c4c4; // "We are very impressed with the way that the team is performing at the moment."
        }
        if current <= target {
            return 0;
        }
        return by_expectation([0xa0c474, 0xa0c42c, 0xa0c3e8, 0xa0c3a0, 0xa0c354, 0xa0c310, 0xa0c2c4]); // "This run of victories is keeping us on track to ..."
    }
    if run != -2 || target < best as i32 {
        return 0;
    }
    if current > target + 4 {
        return 0xa0c270; // "If this losing run isn't turned around soon we may have to look for a new manager."
    }
    if current < target - 4 && !club.division().is_null() {
        let league = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(club.division().addr()) as usize).wrapping_mul(4)));
        if league != 0 {
            let clubs = read_i8(league + league::ENTRANT_COUNT) as i32; // a byte of the entrant count
            let relegation_zone = flags as i16 & expectation_flags::RELEGATION_BATTLE != 0
                && call_comp_virtual(league, league_vtable::HAS_RELEGATION) != 0
                && current > clubs - read_i8(league + league::RELEGATED_COUNT) as i32;
            return if relegation_zone {
                0xa0c22c // "We are in danger of getting relegated unless results improve soon."
            } else {
                0xa0c1d8 // "We are in danger of losing our good league position unless results improve soon."
            };
        }
    }
    if current <= target + 2 {
        return 0;
    }
    by_expectation([0xa0c174, 0xa0c118, 0xa0c0c0, 0xa0c068, 0xa0c008, 0xa0bfc0, 0xa0bf5c]) // "We are worried that this run of defeats ..."
}

/// Writes the board's (or the FA's) verdict on a match into the game's text buffer (`NEWS_TEXT_BUFFER`), by the
/// match marks, with a sentence on the league run or on the players it named; returns 1, or 0 when only the plain
/// default verdict was written.
///
/// `build_board_match_reaction_text(club, other, comp, marks, score, _, _, ids, variant, loss, run, target, current,
/// best, in_league)`, called after every match by `update_board_confidence_after_match` (the text is not used there:
/// only the return value) and by the news text `news_board_match_reaction_text`. The board: "The <club> board of
/// directors" (0xa0fc54, the club's `.gender_name()` and name), or for a national team (`exe::database_is_national_team`)
/// "The <nationality> Football Association" (0xa0fc8c, the adjective of the club's `.nation()` by
/// `exe::nation_get_nationality_adjective`, with 100 as the two genders). The verdict by `marks`
/// (`board_verdict_template`): the cases that ask whether the competition is a friendly
/// (`exe::friendly_is_friendly_competition` of `G_FRIENDLY`) take the friendly template; the others, their template
/// with the opponent's gender and name, the competition's `.short_gender_name()` and short name and the score.
/// Marks without a case write straight into the buffer with the score and return 0: 0xa0e94c with 0x800, 0xa0be98
/// with 0x400, else 0xa0bdd0.
///
/// The run sentence (`board_run_sentence`), only with `in_league`: a winning run (2) with the club at or above its
/// target by at most 3 places gives "We are very impressed..."; a losing run (−2) with
/// the target not above the best position: more than 4 places below the target "If this losing run isn't turned
/// around...", more than 4 above it in a division (`league::ENTRANT_COUNT` low byte, `league::RELEGATED_COUNT`,
/// `league_vtable::HAS_RELEGATION`) "danger of getting relegated" when the board expects a relegation battle
/// (`expectation_flags::RELEGATION_BATTLE` of `board_record::EXPECTATION_FLAGS`) and the club is in the relegation
/// places, else "danger of losing our good league position"; more than 2 places below the target, the "worried"
/// sentence of the first expectation flag (European places, title, promotion, play-offs, 0x40, relegation battle,
/// else the comfortable position). Without one and with marks 0x1800, the named players' sentence
/// (`build_board_player_performance_text`, with "won" = marks bit 0x800 in the low byte of `loss`). The buffer gets
/// "%s %s" (verdict and sentence) or "%s" (0x991948 / 0x991944, `story_crt!`).
///
/// Defects of the original: without a competition it writes 0xa0bd50 with the board string before building it
/// (uninitialised stack; the callers always pass one); the winning-run tests make "Hopefully this run..." and the
/// seven "This run of victories is keeping us on track..." sentences unreachable (they need the club both at or above
/// and below its target); the two aggregate arguments are never read.
/// Original: `MANAGER_MANAGER::build_board_match_reaction_text` (0x6a69c0, manager_manager.cpp, thiscall, ret 0x3c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, _aggregate_for: i32,
                                                                       _aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                       current: u32, best: u32, in_league: u32) -> u32 {
    let (club, other, comp) = (Club(club), Club(other), Competition(comp));
    let mut board = [0u8; 200];
    let b = board.as_mut_ptr() as usize;
    if comp.is_null() {
        // simplification: the original quotes the board string before writing it (stack garbage); the copy an empty one
        story_lang!(NEWS_TEXT_BUFFER, 0xa0bd50, other.gender_name() as i32, other.gender_name() as i32, b, other.name_addr());
        return 0;
    }
    if exe::database_is_national_team(club) != 0 {
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(club.nation(), adjective.as_mut_ptr() as usize, 100, 9);
        story_lang!(b, 0xa0fc8c, 100, 100, adjective.as_ptr()); // "The <nationality> Football Association"
    } else {
        let g = club.gender_name() as i32;
        story_lang!(b, 0xa0fc54, g, g, club.name_addr()); // "The <club> board of directors"
    }
    let mut verdict = [0u8; 1500];
    let v = verdict.as_mut_ptr() as usize;
    match board_verdict_template(marks) {
        Some((friendly, template, shape)) => {
            covered!("sx14_1 verdict");
            match friendly {
                Some(t) if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 => {
                    covered!("sx14_1 friendly verdict");
                    format_board_verdict(v, t, BoardVerdictArgs::Friendly, other, comp, b, score)
                }
                _ => format_board_verdict(v, template, shape, other, comp, b, score),
            }
        }
        None => {
            covered!("sx14_1 default verdict");
            let template = if marks & 0x800 != 0 { 0xa0e94c } else if marks & 0x400 != 0 { 0xa0be98 } else { 0xa0bdd0 };
            format_board_verdict(NEWS_TEXT_BUFFER, template, BoardVerdictArgs::OpponentInCompWithScore, other, comp, b, score);
            return 0;
        }
    }
    let mut sentence = [0u8; 1500];
    let s = sentence.as_mut_ptr() as usize;
    if in_league as u8 != 0 {
        let t = board_run_sentence(this, club, run as i8, target as i8, current as i8, best as i8);
        if t != 0 {
            covered!("sx14_1 run sentence");
            story_lang!(s, t);
        }
    }
    if sentence[0] == 0 && marks & 0x1800 != 0 {
        covered!("sx14_1 named players");
        let won = (loss & 0xffff_ff00) | ((marks as u32 >> 11) & 1);
        build_board_player_performance_text(ids as *const i32, won, s, variant);
    }
    if sentence[0] != 0 {
        story_crt!(NEWS_TEXT_BUFFER, 0x991948, v, s); // "%s %s"
    } else {
        story_crt!(NEWS_TEXT_BUFFER, 0x991944, v); // "%s"
    }
    1
}

/// Comparer for `build_board_match_reaction_text`: the original and the copy side by side on the game's text buffer
/// (`NEWS_TEXT_BUFFER`, 2000 bytes) and the return value. Without a competition only the original runs (it quotes an
/// uninitialised string).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, aggregate_for: i32,
                                                                               aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                               current: u32, best: u32, in_league: u32) -> u32 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let o = original_fn::<F>(%%I0%%);
    let original = || o(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league);
    if comp == 0 {
        return original();
    }
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&[(NEWS_TEXT_BUFFER, 2000)], original, || {
        build_board_match_reaction_text(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league)
    });
    let text = |m: &crate::comparison::MemorySnapshot| {
        let b = &m.0[0].1;
        String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned()
    };
    SCOREBOARD.record(same && ro == rr, || {
        format!("build_board_match_reaction_text(marks {:#x}, run {} target {} current {} best {} league {}) original {} {:?} rust {} {:?}",
                marks, run as i8, target as i8, current as i8, best as i8, in_league as u8, ro, text(&mo), rr, text(&mr))
    });
    ro
}
