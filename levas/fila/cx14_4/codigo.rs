// ---------- cx14_4: the Turkish category B staff records, fate news text and final positions (tur_second_b.cpp) ----------

/// The staff record a category B status adds (`tur_second_b_update_staff_records_for_status`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CategoryBRecord {
    /// Staff_Records::add_qualified_for_round_record with 0x458, 0, 0x28 (through from a first phase group).
    Qualified,
    /// Staff_Records::add_promotion_record with 0x64.
    Promotion,
    /// Staff_Records::add_relegation_record.
    Relegation,
}

/// What `tur_second_b_update_staff_records_for_status` does for a club, worked out without side effects: the error
/// box line (a club whose group is none of the five), the row of its first phase group and the status written there,
/// the record added and the result.
#[derive(Debug)]
pub(crate) struct CategoryBRecordPlan {
    pub(crate) error_line: Option<u32>,
    pub(crate) row: Option<(usize, u8)>,
    pub(crate) record: Option<CategoryBRecord>,
    pub(crate) result: u32,
}

/// The first phase group of a category B club, by the id of its reserve division (`club::RESERVE_DIVISION`): group 1
/// ([`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`]) is the league's own table, groups 2 to 5 the group stages 0..3.
/// None when the id is none of the five.
pub(crate) unsafe fn category_b_group_of(this: League, division: usize) -> Option<League> {
    let id = read_usize(division);
    TURKISH_CATEGORY_B_GROUPS.iter().position(|&g| id == read_usize(g)).map(|k| if k == 0 { this } else { League(read_usize(this.stages() + 4 * (k - 1))) })
}

/// The plan of `tur_second_b_update_staff_records_for_status` for a club with `status` at `stage` (signed):
/// - first phase (stage below 4): status 1 adds the qualified record; the result is 0;
/// - promotion group (4), classification groups (5 to 9) and play-off (10): no reserve division, nothing and 0; the
///   group by `category_b_group_of` (none: the box at line 0x3d4, 0x400 or 0x42a and −1), the club's row in it
///   (League::find_club_row `exe::league_find_club_row`(group, club, 0); none: −1). Then in the promotion group status
///   0 or 5 is written in the row with the promotion record (0) and any other status gives −1; in a classification
///   group status 3 is written with the relegation record; in the play-off status 1 writes 0 in the row with the
///   promotion record; otherwise nothing and 0;
/// - later stages: nothing and 0.
pub(crate) unsafe fn category_b_record_plan(this: League, club: Club, status: u8, stage: i8) -> CategoryBRecordPlan {
    let mut plan = CategoryBRecordPlan { error_line: None, row: None, record: None, result: 0 };
    if stage < 4 {
        if status == 1 {
            plan.record = Some(CategoryBRecord::Qualified);
        }
        return plan;
    }
    if stage > 10 {
        return plan;
    }
    let division = club.reserve_division().addr();
    if division == 0 {
        return plan;
    }
    let Some(group) = category_b_group_of(this, division) else {
        plan.error_line = Some(match stage {
            4 => 0x3d4,
            10 => 0x42a,
            _ => 0x400,
        });
        plan.result = u32::MAX;
        return plan;
    };
    let row = exe::league_find_club_row(group, club, 0);
    if row == 0 {
        plan.result = u32::MAX;
        return plan;
    }
    match stage {
        4 if status == 0 || status == 5 => {
            plan.row = Some((row, status));
            plan.record = Some(CategoryBRecord::Promotion);
        }
        4 => plan.result = u32::MAX,
        10 if status == 1 => {
            plan.row = Some((row, 0));
            plan.record = Some(CategoryBRecord::Promotion);
        }
        5..=9 if status == 3 => {
            plan.row = Some((row, 3)); // cx14_4: the classification group club goes down
            plan.record = Some(CategoryBRecord::Relegation);
        }
        _ => {}
    }
    plan
}

/// Updates the staff records by a club's status in the Turkish second division category B and marks its row in its
/// first phase group (`category_b_record_plan`: the error box, the row status, then the record; the competition is
/// this league's, `league::COMPETITION`). The last three arguments are not read.
/// Original: `TurSecondB::update_staff_records_for_status` (0x9012d0, tur_second_b.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn tur_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let plan = category_b_record_plan(l, Club(club), status as u8, stage as u8 as i8);
    if let Some(line) = plan.error_line {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, line);
    }
    if let Some((row, s)) = plan.row {
        covered!("cx14_4 category b row status");
        write_u8(row + league_table_row::STATUS, s);
    }
    let records = read_usize(G_STAFF_RECORDS);
    let comp = l.competition().addr();
    match plan.record {
        Some(CategoryBRecord::Qualified) => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0x458, 0, 0x28),
        Some(CategoryBRecord::Promotion) => exe::staff_records_add_promotion_record(records, club, comp, 0x64),
        Some(CategoryBRecord::Relegation) => exe::staff_records_add_relegation_record(records, club, comp),
        None => {}
    }
    plan.result
}

/// Comparer for `tur_second_b_update_staff_records_for_status`: only the original runs (it adds records); its result
/// and the status of the planned row against the plan.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let plan = category_b_record_plan(League(this), Club(club), status as u8, stage as u8 as i8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = plan.row.map_or(true, |(row, s)| read_u8(row + league_table_row::STATUS) == s);
    SCOREBOARD.record(r == plan.result && row_ok, || format!("tur_second_b_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={:#x} plan={:?}", this, club,
        status as u8, stage as u8 as i8, r, plan));
    r
}

/// The text of a club and a competition: `format` with the club's short gender byte twice, then (when `with_comp`)
/// the competition's short gender byte twice, the club's short name and (when `with_comp`) the competition's short
/// name into the shared buffer (`PRAISE_TEXT_BUFFER`), the buffer set in `text` with the club's link (0x7d5) and the
/// competition's (0x7d0).
pub(crate) unsafe fn club_and_competition_short_text(club_id: u32, comp: usize, format: usize, with_comp: bool, text: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let team = club.addr() + club::SHORT_NAME;
    if with_comp {
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, cg, cg, team, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, team);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, team as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The fate news text of a category B club (thiscall, ret 0x28; same arguments as Comp::build_fate_news_text).
/// - promotion group (stage 4): with a news item the common text (`exe::comp_build_fate_news_text` with the stage
///   pushed as the constant 4 and a7 = 0); without one, outcome 0 or 5 gives 0x989c18 "{club} win promotion"
///   (`play_off_club_text`) and any other outcome nothing;
/// - classification groups (5 to 9): outcome 1 gives, with a news item, 0xaa191c "{club} have qualified for the
///   playoff of the {competition}" and, without one, 0xaa18f4 "{club} qualify for playoff", both with the club's and
///   the competition's links (`club_and_competition_short_text`); outcome 3 the common text (a7 = 0); any other
///   nothing;
/// - first phase (below 4) and the play-off and later (10 on): the common text (a7 = 0, the rest as they came).
/// Original: `TurSecondB::build_fate_news_text` (0x9015f0, tur_second_b.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn tur_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    let s = stage as u8 as i8;
    let common = |stage: u32| exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, a5 as usize, a6, 0, a8, news, text);
    if s == 4 {
        if news != 0 {
            return common(4);
        }
        if outcome as u8 == 0 || outcome as u8 == 5 {
            covered!("cx14_4 category b promotion text");
            play_off_club_text(club_id, 0x989c18, text); // "{club} win promotion"
        }
    } else if (5..10).contains(&s) {
        match outcome as u8 {
            1 => {
                covered!("cx14_4 category b play-off text");
                let comp = League(this).competition().addr();
                club_and_competition_short_text(club_id, comp, if news != 0 { 0xaa191c } else { 0xaa18f4 }, news != 0, text); // cx14_4: qualified for the play-off
            }
            3 => common(stage),
            _ => {}
        }
    } else {
        common(stage);
    }
}

/// Comparer for `tur_second_b_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the common text only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I1%%);
    let s = stage as u8 as i8;
    let own = (s == 4 && news == 0 && (outcome as u8 == 0 || outcome as u8 == 5)) || ((5..10).contains(&s) && outcome as u8 == 1);
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| tur_second_b_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("tur_second_b_build_fate_news_text(club {}, outcome {}, stage {}, news {})", club_id, outcome as u8, s, news));
}

/// The clubs whose final position the category B records, with the place: the first two of each first phase group
/// (the league's own table, then the group stages 0..3) get places 1 and 2, the clubs of each classification group
/// (stages 5 to 9) places 3 upwards in table order (a byte counter). The promotion group and the play-off record
/// nothing.
pub(crate) unsafe fn category_b_final_places(this: League) -> Vec<(Club, u8)> {
    let mut v = Vec::new();
    for g in -1i32..4 {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        for p in 1..=2u8 {
            v.push((Club(aus_nsl_table_row_club(t, (p - 1) as usize)), p));
        }
    }
    for g in 5..10usize {
        let t = League(read_usize(this.stages() + 4 * g));
        let mut k = 0i8;
        while (k as i16) < t.club_count() {
            v.push((Club(aus_nsl_table_row_club(t, k as usize)), (k as u8).wrapping_add(3))); // cx14_4: places from 3
            k = k.wrapping_add(1);
        }
    }
    v
}

/// Records the final positions of the Turkish second division category B: each club of `category_b_final_places`
/// gets this league's competition as its last division (`club::LAST_DIVISION`) and its place
/// (`.set_last_position()`), with no check of the club's division.
/// Original: `TurSecondB::record_final_positions` (0x901ba0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_record_final_positions(this: usize) {
    covered!("cx14_4 category b final positions");
    let l = League(this);
    for (c, p) in category_b_final_places(l) {
        write_usize(c.addr() + club::LAST_DIVISION, l.competition().addr());
        c.set_last_position(p);
    }
}

/// Comparer for `tur_second_b_record_final_positions`: side by side over the last division and the last position of
/// every club it records.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_record_final_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let regions: Vec<(usize, usize)> = category_b_final_places(League(this)).iter().map(|(c, _)| (c.addr() + club::LAST_DIVISION, 5)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || tur_second_b_record_final_positions(this));
    SCOREBOARD.record(same, || format!("tur_second_b_record_final_positions({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}
