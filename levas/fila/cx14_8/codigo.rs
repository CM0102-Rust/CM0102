// ---------- cx14_8: the Superettan's staff records, fate news text and progress codes (swe_first.cpp) ----------

/// The Superettan and its promotion play-off against the Allsvenskan (`play_off_record_plan`): the play-off clubs of
/// the Allsvenskan in its own table, the winner's status 1 and the season's play-off place with 0, 0xa0, 0x28.
pub(crate) const SWEDISH_FIRST_PLAY_OFF: PlayOffDivision = PlayOffDivision {
    own_key: KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION,
    upper_key: KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION,
    upper_stage: None,
    winner: 1, // cx14_8: the play-off winner's status
    qualified: (0, 0xa0, 0x28),
};

/// Updates the staff records by a club's status in the Superettan and its play-off (stage 0): the season (stage 0xff)
/// and the play-off by `play_off_update_staff_records` with `SWEDISH_FIRST_PLAY_OFF` (in the play-off the result is
/// the value of the club's own key); any other stage nothing and 0. The last three arguments are not read.
/// Original: `SweFirst::update_staff_records_for_status` (0x88f600, swe_first.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn swe_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    if stage as u8 != 0xff && stage as u8 != 0 {
        return 0;
    }
    covered!("cx14_8 superettan records");
    play_off_update_staff_records(this, club, status, stage, &SWEDISH_FIRST_PLAY_OFF)
}

/// Comparer for `swe_first_update_staff_records_for_status` (`compare_play_off_staff_records`; other stages only the
/// original, which must return 0).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%);
    if stage as u8 != 0xff && stage as u8 != 0 {
        let r = o(this, club, status, stage, a4, a5, a6);
        SCOREBOARD.record(r == 0, || format!("swe_first_update_staff_records_for_status({:#x}, stage {}) returned {}", this, stage as u8, r));
        return r;
    }
    compare_play_off_staff_records("swe_first_update_staff_records_for_status", this, club, status, stage, &SWEDISH_FIRST_PLAY_OFF, || o(this, club, status, stage, a4, a5, a6))
}

/// The fate news text of a Superettan club. With a news item, the common text (`exe::comp_build_fate_news_text`, a7 =
/// 0, the rest as they came). Without one (the common texts below pass the news item 0 and a7 = 0):
/// - play-off (stage 0): outcome 1 gives 0x987594 "{club} avoid relegation" for an Allsvenskan club (`a7`, its
///   division, is [`KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`]) and 0x989c18 "{club} win promotion" for the others; any
///   other outcome 0x987564 "{club} relegated" for an Allsvenskan club and the common text (stage 0) for the others;
/// - stage 1: outcome 1 "{club} avoid relegation", outcome 0xff or 2 "{club} relegated", others the common text
///   (stage 1);
/// - other stages: the common text.
/// The club texts by `play_off_club_text`.
/// Original: `SweFirst::build_fate_news_text` (0x88f7f0, swe_first.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn swe_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let common = |stage: u32, news: u32| exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    if news != 0 {
        return common(stage, news);
    }
    let premier = a7 == read_u32(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION);
    match stage as u8 {
        0 => {
            covered!("cx14_8 superettan play-off text");
            if outcome as u8 == 1 {
                play_off_club_text(club_id, if premier { 0x987594 } else { 0x989c18 }, text); // cx14_8: avoid relegation / win promotion
            } else if premier {
                play_off_club_text(club_id, 0x987564, text); // "relegated"
            } else {
                common(0, 0);
            }
        }
        1 => match outcome as u8 {
            1 => play_off_club_text(club_id, 0x987594, text),
            0xff | 2 => play_off_club_text(club_id, 0x987564, text),
            _ => common(1, 0),
        },
        _ => common(stage, 0),
    }
}

/// Comparer for `swe_first_build_fate_news_text`: the club texts side by side with `compare_text_writer`; the common
/// text only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_swe_first_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let premier = a7 == read_u32(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION);
    let own = news == 0 && match stage as u8 {
        0 => outcome as u8 == 1 || premier,
        1 => matches!(outcome as u8, 1 | 2 | 0xff),
        _ => false,
    };
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| swe_first_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("swe_first_build_fate_news_text(club {}, outcome {}, stage {}, premier {})", club_id, outcome as u8, stage as u8 as i8, premier));
}

/// Stores a Superettan club's progress codes in the competition statistics (`exe::comp_stats_find_club_entry`, not
/// checked):
/// - league (stage 0xff): a best code 3 becomes 1, the rest as they came;
/// - play-off (stage 0), a Superettan club (its division's id is not the Allsvenskan's): 2 × code − 1 each (bytes);
/// - play-off, an Allsvenskan club: its entry in the Allsvenskan's statistics (`G_COMP_OBJECT_TABLE` by the id of
///   [`KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`]) gets the best and worst codes 1 → 10 and 2 → 13 and the current code
///   1 → 10 and 2 → 13, then all three codes as they came over them (a defect of the original: the mapped codes are
///   overwritten at once); the Superettan entry is not written;
/// - other stages: the codes as they came.
/// The club's division is read without a null check (as the original).
/// Original: `SweFirst::set_club_progress` (0x88fa10, swe_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let entry = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (c, b, w) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => write_progress_codes(entry, c, if b == 3 { 1 } else { b }, w),
        0 if read_usize(read_usize(club + club::DIVISION)) == read_usize(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) => {
            covered!("cx14_8 superettan play-off progress of an allsvenskan club");
            let premier = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION));
            let e = CompStatsClub(exe::comp_stats_find_club_entry(premier.comp_stats(), club));
            let map = |v: u8| match v {
                1 => Some(0xa),
                2 => Some(0xd),
                _ => None,
            };
            if let Some(v) = map(b) {
                e.set_best_position(v);
            }
            if let Some(v) = map(w) {
                e.set_worst_position(v);
            }
            if let Some(v) = map(c) {
                e.set_current_position(v);
            }
            write_progress_codes(e.addr(), c, b, w);
        }
        0 => {
            covered!("cx14_8 superettan play-off progress");
            let twice = |v: u8| v.wrapping_mul(2).wrapping_sub(1); // cx14_8: 2 x code - 1
            write_progress_codes(entry, twice(c), twice(b), twice(w));
        }
        _ => write_progress_codes(entry, c, b, w),
    }
}

/// Comparer for `swe_first_set_club_progress`: side by side over the three bytes of the club's Superettan entry and,
/// for an Allsvenskan club in the play-off, of its Allsvenskan entry (only the original when an entry is missing).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    let division = read_usize(club + club::DIVISION);
    let premier = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION));
    let other = if stage as u8 == 0 && division != 0 && read_usize(division) == read_usize(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) && !premier.is_null() && premier.comp_stats() != 0 {
        exe::comp_stats_find_club_entry(premier.comp_stats(), club)
    } else {
        1
    };
    if entry == 0 || other == 0 || division == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let mut regions = vec![(entry + comp_stats_club::CURRENT_POSITION, 3)];
    if other != 1 {
        regions.push((other + comp_stats_club::CURRENT_POSITION, 3));
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, stage, current, best, worst), || swe_first_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("swe_first_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}
