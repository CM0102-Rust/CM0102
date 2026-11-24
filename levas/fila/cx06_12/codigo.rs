// ---------- cx06_12: the European Championship qualifiers' staff records, fate news text and status change (euro_champ_qual.cpp) ----------

/// The European Championship finals' competition record (`comp_record_of_key` with
/// `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`).
unsafe fn euro_finals_record() -> usize {
    comp_record_of_key(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP)
}

/// A nation qualified for the European Championship finals: the qualified-for-round record for the finals (stage 0,
/// round 0, `weight`; `european_cup_add_qualified_record`) and the nation added to the finals (EuroChamp::add_qualified_team
/// 0x57c120 on the finals' object, `G_COMP_OBJECT_TABLE`[the finals' id]).
unsafe fn euro_qual_qualified(club: usize, weight: u32) {
    european_cup_add_qualified_record(club, euro_finals_record(), 0, 0, weight);
    let finals = read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) as usize);
    let add_qualified_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x57c120usize); // EuroChamp::add_qualified_team
    add_qualified_team(finals, club);
}

/// Writes the staff history records of a nation's fate in the qualifiers (virtual +0x44).
///
/// Before the play-off stage (`stage`, signed, below +0x101): `outcome` 7 (qualified from the groups),
/// `euro_qual_qualified` with weight 0x64, `left` (when not null) gets 3, the nation added to the finals' nations
/// (NationalTeams::add_qualified_nation 0x7779b0 on `G_NATIONAL_TEAMS` with the finals' record), and −1; 1 and 0xfe, −1;
/// others, 0. In the play-off: 1 (through), `euro_qual_qualified` with weight 0x28 and `left` 3 (when not null); 2
/// (out), knocked out (`european_cup_add_knocked_out_record` with the fixture's `round_prefix` and `round_name`, weight
/// 0x28) and the national team's competition ended (`clear_national_team_competition`); returns 0.
/// Original: `EuroChampQual::update_staff_records_for_status` (0x57d5b0, euro_champ_qual.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, left: usize, game: usize, _a6: u32) -> i32 {
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage < read_u8(this + 0x101) as i8 { // the play-off stage
        return match outcome {
            7 => {
                covered!("cx06_12 qualified from the groups");
                euro_qual_qualified(club, 0x64);
                if left != 0 {
                    write_u8(left, 3);
                }
                let add_qualified_nation: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7779b0usize); // NationalTeams::add_qualified_nation
                add_qualified_nation(read_usize(G_NATIONAL_TEAMS), club, euro_finals_record());
                -1
            }
            1 | 0xfe => -1,
            _ => 0,
        };
    }
    match outcome {
        1 => {
            covered!("cx06_12 qualified from the play-off");
            euro_qual_qualified(club, 0x28);
            if left != 0 {
                write_u8(left, 3);
            }
        }
        2 => {
            let game = Fixture(game);
            european_cup_add_knocked_out_record(club, League(this).competition().addr(), game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, 0x28);
            clear_national_team_competition(Club(club));
        }
        _ => {}
    }
    0
}

/// The fate texts against the finals: "{nation} qualify for {finals}" / "out of {finals}" with the finals' short name and
/// link (`european_cup_fate_text`).
unsafe fn euro_qual_finals_text(text: usize, club_id: u32, format: usize) {
    let finals = euro_finals_record();
    european_cup_fate_text(text, club_id, format, read_u8(finals + competition::SHORT_GENDER_NAME), finals + competition::SHORT_NAME, finals);
}

/// Writes the news text of a nation's fate in the qualifiers. `group` is the round's stage id.
///
/// In a group (`group` 0x3e8 to 0x3f0): `outcome` 7 with `news` the group news (`exe::comp_util_write_group_qualify_news`
/// with 7), without it "{nation} qualify for {finals}" (0x9c470c, `euro_qual_finals_text`); 0xfe the group news with −2.
/// In the runners-up stage (0x3fc) with 7: with `news` "{nation} have finished as one of the three best second placed
/// teams in the {qualifiers} tournament" (0x9c4800, one European host) or "are the best second placed team" (0x9c4764),
/// with the qualifiers' full name and link (`european_cup_fate_text`); without it the qualify text. Elsewhere, for a
/// play-off fixture (`fixture::ROUND_NAME` 0xa4): with `news` the cup tie news (`exe::comp_util_write_cup_tie_result_news`),
/// otherwise 1 "qualify for" and other outcomes "out of {finals}" (0x9c46b8). Nothing in the other cases.
/// Original: `EuroChampQual::build_fate_news_text` (0x57d700, euro_champ_qual.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_build_fate_news_text(this: usize, club_id: u32, outcome: u32, _stage: u32, group: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let d = outcome as u8;
    let id = group as u16 as i16;
    let comp = read_usize(this + league::COMPETITION);
    if (0x3e8..=0x3f0).contains(&id) {
        if d == 7 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, group, comp, 7, news, text);
            } else {
                covered!("cx06_12 group qualify text");
                euro_qual_finals_text(text, club_id, 0x9c470c); // "qualify for"
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, group, comp, -2, news, text);
        }
        return;
    }
    if id == 0x3fc && d == 7 { // the runners-up stage
        if news != 0 {
            covered!("cx06_12 best runner-up text");
            let format = if read_u8(this + euro_champ_qual::GROUP_FORMAT) == 1 { 0x9c4800 } else { 0x9c4764 };
            european_cup_fate_text(text, club_id, format, read_u8(comp + competition::GENDER_NAME), comp + competition::NAME, comp);
        } else {
            euro_qual_finals_text(text, club_id, 0x9c470c);
        }
        return;
    }
    if fixture == 0 || read_i16(fixture + fixture::ROUND_NAME) != 0xa4 { // the play-off round
        return;
    }
    if news != 0 {
        exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
    } else {
        covered!("cx06_12 play-off fate text");
        euro_qual_finals_text(text, club_id, if d == 1 { 0x9c470c } else { 0x9c46b8 }); // "qualify for" / "out of"
    }
}

/// Whether `euro_champ_qual_build_fate_news_text` only writes the text on this call (what its comparer compares side
/// by side).
unsafe fn euro_qual_fate_text_only(outcome: u32, group: u32, fixture: usize, news: u32, text: usize) -> bool {
    let (d, id) = (outcome as u8, group as u16 as i16);
    if text == 0 {
        return false;
    }
    if (0x3e8..=0x3f0).contains(&id) {
        return d == 7 && news == 0;
    }
    if id == 0x3fc && d == 7 {
        return true;
    }
    news == 0 && fixture != 0 && read_i16(fixture + fixture::ROUND_NAME) == 0xa4
}

/// Tells how a nation's qualifying position changed: the previous (`exe::comp_stats_get_club_stat` 0x13) and current
/// (0x12) position codes, truncated (`ftol`), against 14 (the low byte, signed): now below 14, −1 if it was not, else 0;
/// now 14 or more, 1 if it was below, else 0. The original keeps the high bytes of the current code's dword in eax
/// (0 for the codes of the statistics), and so does the copy.
/// Original: `EuroChampQual::get_qualification_status_change` (0x580b80, euro_champ_qual.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_get_qualification_status_change(this: usize, club: usize) -> i32 {
    covered!("cx06_12 status change");
    let stats = League(this).comp_stats();
    let before = ftol(exe::comp_stats_get_club_stat(stats, club, 0x13)) as u8 as i8;
    let now = ftol(exe::comp_stats_get_club_stat(stats, club, 0x12)) as u32;
    let was_in = (before < 0xe) as u32;
    let r = (now & !0xff) | was_in;
    if ((now as u8) as i8) < 0xe { r.wrapping_sub(1) as i32 } else { r as i32 }
}

/// Comparer for `euro_champ_qual_update_staff_records_for_status`: the records go to the managers' histories, the nation
/// to the finals and its call-ups end (they cannot run twice), so only the original runs; the plan: the return value of
/// the outcome and `left` 3 for the qualified.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, left: usize, game: usize, a6: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> i32>(%%I0%%)(this, club, outcome, stage, left, game, a6);
    let groups = (stage as u8 as i8) < read_u8(this + 0x101) as i8;
    let (d, want) = (outcome as u8, if groups && matches!(outcome as u8, 7 | 1 | 0xfe) { -1 } else { 0 });
    let qualified = (groups && d == 7) || (!groups && d == 1);
    SCOREBOARD.record(r == want && (!qualified || left == 0 || read_u8(left) == 3), || format!("euro_champ_qual_update_staff_records_for_status(outcome {}, stage {}) returned {}", d, stage as u8 as i8, r));
    r
}

/// Comparer for `euro_champ_qual_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`); the
/// group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, group: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I1%%);
    if !euro_qual_fate_text_only(outcome, group, fixture, news, text) {
        return o(this, club_id, outcome, stage, group, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, group, fixture, a6, a7, a8, news, text),
        |t| euro_champ_qual_build_fate_news_text(this, club_id, outcome, stage, group, fixture, a6, a7, a8, news, t),
        || format!("euro_champ_qual_build_fate_news_text(club {}, outcome {}, group {:#x})", club_id, outcome as u8, group as u16));
}

/// Comparer for `euro_champ_qual_get_qualification_status_change`: side by side (it only reads), the values must match.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_get_qualification_status_change(this: usize, club: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I2%%)(this, club);
    let c = euro_champ_qual_get_qualification_status_change(this, club);
    SCOREBOARD.record(r == c, || format!("euro_champ_qual_get_qualification_status_change({:#x}, club {:#x}): original {} rust {}", this, club, r, c));
    r
}
