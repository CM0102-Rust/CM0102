// ---------- cx08_6: the Argentine promotion and relegation chain and the relegation-by-average news (arg_prm.cpp) ----------

/// A competition's database record (`G_CLUB_COMP_TABLE`, 0x6b bytes each) by the id at the key global `key`.
pub(crate) unsafe fn comp_record_by_key(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE) + read_usize(key).wrapping_mul(0x6b)
}

/// Rule: the Argentine promotion and relegation: the final positions of the Premier and the Second Division are
/// recorded; each Second Division club marked 0 (champion) or 5 goes up into the Premier Division and every other one
/// has its non-promotion contract clauses triggered; each Premier Division club marked relegated (3) goes down to the
/// Second Division.
///
/// `ArgPrm::process_promotion_relegation_chain(pairs)` (`pairs` not read): virtual
/// `league_vtable::RECORD_FINAL_POSITIONS` on this and on the Second Division (`KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION`,
/// not checked for null); each row below the Second Division's `league::CLUB_COUNT` with `league_table_row::STATUS` 0 or
/// 5: League::move_club_to_division (`exe::league_move_club_to_division`(this, club, this competition record, 0));
/// otherwise CONTRACT_MANAGER::trigger_non_promotion_clauses (`exe::contract_manager_trigger_non_promotion_clauses` on
/// `G_CONTRACT_MANAGER`, with the club and the Second Division's record). Then each row of this table with status 3:
/// League::relegate_club_to_division (`exe::league_relegate_club_to_division`(this, club, the Second Division's
/// database record, 0)). Returns 0 (the original leaves a stale eax; the caller does not read it).
/// Original: `ArgPrm::process_promotion_relegation_chain` (0x4067f0, arg_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn arg_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION));
    bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(second.addr(), league_vtable::RECORD_FINAL_POSITIONS);
    covered!("cx08_6 argentine chain");
    let mut k: i16 = 0;
    while k < second.club_count() {
        let row = LeagueTableRow(second.table() + k as usize * league_table_row::SIZE);
        let club = row.club() as usize;
        match row.status() {
            0 | 5 => exe::league_move_club_to_division(this, club, this.competition().addr(), 0),
            _ => exe::contract_manager_trigger_non_promotion_clauses(read_usize(G_CONTRACT_MANAGER), club, second.competition().addr()),
        }
        k += 1;
    }
    let mut k: i16 = 0;
    while k < this.club_count() {
        let row = LeagueTableRow(this.table() + k as usize * league_table_row::SIZE);
        if row.status() == 3 {
            exe::league_relegate_club_to_division(this, row.club() as usize, comp_record_by_key(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION), 0);
        }
        k += 1;
    }
    0
}

/// Comparer for `arg_prm_process_promotion_relegation_chain`: only the original (it moves the clubs between the
/// divisions, as the other chains' comparers).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, pairs)
}

/// Rule: up to four Second Division clubs marked relegated (3) go down to the Third Division, and as many Third
/// Division clubs come up, chosen from the strongest (sorted and shuffled among the top 6).
///
/// `arg_prm_swap_second_and_third_division_clubs()`: the Second Division's averages brought up to date with a new season
/// (`update_relegation_averages_virtual`(1)); the list (`exe::crt_malloc` of 0xa0; none: the arg_prm.cpp box of line
/// 0x4b8 and nothing else) of up to 40 clubs, in the club table's order, that have a division (`club::DIVISION`), a
/// nation (`club::NATION`) and a `club::RESERVE_DIVISION` and whose division's id is that of
/// `KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION`; none: the list freed and nothing else. Otherwise
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 6); the Second Division's rows with status 3, while fewer than
/// four went down (a signed byte), go down (League::relegate_club_to_division on this, with the Third Division's record,
/// 1); then the first listed clubs, one for each that went down, come up (League::move_club_to_division on this, with
/// the Second Division's record, 1), each slot cleared.
/// Defect of the original: the list is not freed after the clubs move (160 bytes lost every season).
/// Original: `arg_prm_swap_second_and_third_division_clubs` (0x4068d0, arg_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_prm_swap_second_and_third_division_clubs(this: usize) {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION));
    update_relegation_averages_virtual(second, 1);
    let list = exe::crt_malloc(0xa0);
    if list == 0 {
        return show_error_at(ARG_PRM_FILE, 0x4b8);
    }
    let mut n: i16 = 0;
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && n < 0x28 {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division != 0 && read_usize(club.addr() + club::NATION) != 0 && read_usize(club.addr() + club::RESERVE_DIVISION) != 0
            && read_usize(division) == read_usize(KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION) {
            write_usize(list + 4 * n as usize, club.addr());
            n += 1;
        }
        i += 1;
    }
    if n == 0 {
        exe::crt_free(list);
        return;
    }
    covered!("cx08_6 argentine second and third swap");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u32, 6);
    let mut moved: i8 = 0;
    let mut k = 0i32;
    while k < second.club_count() as i32 && moved < 4 {
        let row = LeagueTableRow(second.table() + k as usize * league_table_row::SIZE);
        if row.status() == 3 {
            moved += 1;
            exe::league_relegate_club_to_division(this, row.club() as usize, comp_record_by_key(KEY_CLUB_COMP_ARGENTINE_THIRD_DIVISION), 1);
        }
        k += 1;
    }
    let mut j = 0usize;
    while j < n as usize && moved != 0 {
        let slot = list + 4 * j;
        exe::league_move_club_to_division(this, read_usize(slot), comp_record_by_key(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION), 1);
        moved -= 1;
        write_usize(slot, 0);
        j += 1;
    }
}

/// Comparer for `arg_prm_swap_second_and_third_division_clubs`: only the original (it moves clubs and draws).
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_swap_second_and_third_division_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Rule: the news of a club relegated from the Argentine Premier Division by its average: "{club} have been relegated
/// from the {competition} as a result of their poor average of points over the last three seasons."; any other fate,
/// the competitions' common text.
///
/// `ArgPrm::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`: in the main stage (the
/// stage byte 0xff), outcome 3 and `body`: the line (0x987420, `FN_LANGLIB_SPRINTF` into `PRAISE_TEXT_BUFFER`) with the
/// club's short name and gender (`club::SHORT_GENDER_NAME`, twice) and the competition's (the record at
/// `league::COMPETITION`, `competition::SHORT_GENDER_NAME`, twice), the Text set to the buffer and the links of the club
/// (0x7d5) and the competition (0x7d0). Otherwise Comp::build_fate_news_text (`exe::comp_build_fate_news_text` with the
/// same arguments but a7 = 0).
/// Original: `ArgPrm::build_fate_news_text` (0x406aa0, arg_prm.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn arg_prm_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, body: u32, text: usize) {
    if stage as u8 != 0xff || outcome as u8 != 3 || body == 0 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, body, text);
    }
    covered!("cx08_6 argentine relegated by average text");
    relegated_by_average_text(this, club_id, 0x987420, text);
}

/// The text of a club relegated by its points average: the line `format` with the club's short name and gender
/// (twice) and the competition's (twice), the Text set to it and the links of the club (0x7d5) and the competition
/// (0x7d0).
pub(crate) unsafe fn relegated_by_average_text(this: usize, club_id: u32, format: usize, text: usize) {
    let comp = read_usize(this + league::COMPETITION);
    let club = club_at(club_id as isize);
    let (cg, kg) = (club.short_gender_name() as i8 as i32, read_i8(comp + competition::SHORT_GENDER_NAME) as i32);
    story_lang!(PRAISE_TEXT_BUFFER, format, cg, cg, kg, kg, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `arg_prm_build_fate_news_text`: side by side (`compare_text_writer`) when there is no fixture (the
/// league texts); a cup tie's text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_arg_prm_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I2%%);
    if fixture != 0 || text == 0 {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| arg_prm_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("arg_prm_build_fate_news_text(club {}, outcome {}, stage {}, body {})", club_id, outcome as u8, stage as u8 as i8, body));
}
