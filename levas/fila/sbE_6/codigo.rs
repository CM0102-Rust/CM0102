// ---------- sbE_6: the Irish First Division's staff records, fate news text and club progress (ire_first.cpp) ----------

/// The Irish First Division and its play-off against the Premier Division (the Premier club in its own table).
pub(crate) const IRISH_FIRST_PLAY_OFF: PlayOffDivision = PlayOffDivision {
    own_key: KEY_CLUB_COMP_IRISH_FIRST_DIVISION,
    upper_key: KEY_CLUB_COMP_IRISH_PREMIER_DIVISION,
    upper_stage: None,
    winner: 1,
    qualified: (0, 0xa0, 0x28),
};

/// Rule: the Irish First Division's staff records: in the season the title, promotion, the play-off place and
/// relegation; in the play-off against the Premier Division the First Division club that wins goes up (promotion
/// record) and the Premier club that loses goes down (relegation record), the table statuses telling who stays where.
///
/// `play_off_update_staff_records` (cx13_6) with `IRISH_FIRST_PLAY_OFF`: the season's play-off place with 0, 0xa0,
/// 0x28, the play-off winner's status 1, the Premier club looked up in the Premier Division's own table. The last three
/// arguments are not read.
/// Original: `IreFirst::update_staff_records_for_status` (0x636020, ire_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ire_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    covered!("sbE_6 irish staff records");
    play_off_update_staff_records(this, club, status, stage, &IRISH_FIRST_PLAY_OFF)
}

/// Comparer for `ire_first_update_staff_records_for_status` (`compare_play_off_staff_records`).
pub(crate) unsafe extern "thiscall" fn compare_ire_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    compare_play_off_staff_records("ire_first_update_staff_records_for_status", this, club, status, stage, &IRISH_FIRST_PLAY_OFF, || o(this, club, status, stage, a4, a5, a6))
}

/// Rule: the fate news text of an Irish First Division club. With a news item the common text first, and then, without
/// returning, the play-off text: outcome 1 in the play-off (stage 0), "{club} avoid relegation" for a Premier club,
/// "{club} win promotion" for a First Division one; otherwise "{club} relegated" for a Premier club, and the common text
/// (again, when there was a news item) for the others.
///
/// With a news item (`news`): Comp::build_fate_news_text (`exe::comp_build_fate_news_text`, a7 = 0, the rest as they
/// came). A Premier club is told by a7 (its division id) against `KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`; the texts
/// 0x987594, 0x989c18 and 0x989d78 by `play_off_club_text` (cx13_6: the buffer, the Text and the club's link).
/// Original: `IreFirst::build_fate_news_text` (0x636210, ire_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ire_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    }
    let premier = a7 == read_u32(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    if outcome as u8 == 1 && stage as u8 == 0 {
        covered!("sbE_6 irish play-off text");
        return play_off_club_text(club_id, if premier { 0x987594 } else { 0x989c18 }, text); // sbE_6: avoid relegation / win promotion
    }
    if premier {
        return play_off_club_text(club_id, 0x989d78, text); // "relegated"
    }
    exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
}

/// Comparer for `ire_first_build_fate_news_text`: without a news item, on the play-off texts, side by side with
/// `compare_text_writer`; on the paths through the common text only the original (it writes the news object and the
/// Text, which takes its text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ire_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let play_off = outcome as u8 == 1 && stage as u8 == 0;
    if news == 0 && text != 0 && (play_off || a7 == read_u32(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)) {
        return compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
            |t| ire_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
            || format!("ire_first_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
    }
    o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text)
}

/// The Irish play-off progress code of a Premier Division club: 1 becomes 10 and 2 becomes 12; any other as it is.
pub(crate) fn irish_play_off_code(v: u8) -> u8 {
    match v {
        1 => 0xa,
        2 => 0xc, // sbE_6: the Premier club that loses the play-off
        _ => v,
    }
}

/// Rule: the progress codes of an Irish First Division club. In the season (stage 0xff) a best of 3 becomes 2 and a
/// worst of 3 becomes 4. In the play-off (stage 0) a Premier Division club's codes go into the Premier Division's own
/// entry with 1 → 10 and 2 → 12; a First Division club's are doubled. Any other stage, as they came.
///
/// The club's entry in the league's Comp_Stats (`exe::comp_stats_find_club_entry`, not checked for null; for a Premier
/// club in the play-off, the entry in the Comp_Stats of `KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`, the club's division read
/// without a null check); the bytes `comp_stats_club::BEST_POSITION`, `::CURRENT_POSITION` and `::WORST_POSITION`.
/// Original: `IreFirst::set_club_progress` (0x636390, ire_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn ire_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let club = Club(club);
    let mut e = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            covered!("sbE_6 irish season progress");
            if b == 3 {
                b = 2;
            }
            if w == 3 {
                w = 4;
            }
        }
        0 => {
            covered!("sbE_6 irish play-off progress");
            if read_u32(club.division().addr()) == read_u32(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) {
                e = CompStatsClub(exe::comp_stats_find_club_entry(League(comp_by_global(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)).comp_stats(), club));
                b = irish_play_off_code(b);
                w = irish_play_off_code(w);
                c = irish_play_off_code(c);
            } else {
                b = b.wrapping_shl(1);
                w = w.wrapping_shl(1);
                c = c.wrapping_shl(1);
            }
        }
        _ => {}
    }
    e.set_best_position(b);
    e.set_current_position(c);
    e.set_worst_position(w);
}

/// Comparer for `ire_first_set_club_progress`: side by side on the three codes of the entry written (the Premier
/// Division's for one of its clubs in the play-off; only the original without an entry).
pub(crate) unsafe extern "thiscall" fn compare_ire_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let c = Club(club);
    let premier = stage as u8 == 0 && c.division().addr() != 0 && read_u32(c.division().addr()) == read_u32(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    let stats = if premier { League(comp_by_global(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)).comp_stats() } else { League(this).comp_stats() };
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 || League(this).comp_stats() == 0 || c.division().addr() == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst), || ire_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("ire_first_set_club_progress({:#x}, {:#x}, stage {}, {} {} {}) original={:x?} rust={:x?}", this, club, stage as u8 as i8, current as u8, best as u8, worst as u8, mo, mr));
}
