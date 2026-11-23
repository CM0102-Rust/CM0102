use super::*;

/// Fills the match record as seen by the club: date, fixture id, competition, venue, club, opponent and goals for and
/// against (the extra-time score, if any).
///
/// `fill_match_record_from_fixture(game, club, out)`, from the tactic statistics (`record_tactic_result`, at the end of
/// each match without a human) and from the records. Null match, club or output: the box (lines 0x29, 0x31, 0x39) and
/// 0. Otherwise the date (+0, `exe::date_operator_3d` of the Date(day `.day()`, year `.year()`) from `exe::new_date`),
/// +8 the match id (+0), `club_match_record::ROUND_PREFIX` word `.round_prefix()`, `club_match_record::ROUND_NAME` word
/// `.round_name()`, `club_match_record::VENUE` the side (`exe::comp_util_get_fixture_venue_side`, cdecl(club, match)),
/// `club_match_record::CLUB_ID` the club id, `club_match_record::MATCH_STATS_SLOT` the match's `.match_stats_slot()`;
/// home side (`.home_club()`): `club_match_record::OPPONENT_ID` the match's `.away_club_id()` and the score
/// (`club_match_record::GOALS_FOR` for, `club_match_record::GOALS_AGAINST` against) from the extra-time one
/// (`.home_goals_extra_time()`/`.away_goals_extra_time()`) if it is not negative, otherwise the normal one
/// (`.home_goals()`/`.away_goals()`); away side: `club_match_record::OPPONENT_ID` the `.home_club_id()` and the score
/// reversed. Returns 1.
/// Original: `fill_match_record_from_fixture` (0x7e4750, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn fill_match_record_from_fixture(game: usize, club: usize, out: usize) -> u32 {
    let game = Fixture(game);
    if game.is_null() {
        show_error_at(0xa7a088, 0x29);
        return 0;
    }
    if club == 0 {
        show_error_at(0xa7a088, 0x31);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x39);
        return 0;
    }
    let (mut t0, mut t1) = ([0u8; 8], [0u8; 8]);
    let (p0, p1) = (t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    let d = exe::new_date(p0, game.day() as u32, game.year() as u32);
    exe::date_operator_3d(out, p1, d);
    core::ptr::write_unaligned(((out + 8) as *mut u8) as *mut u32, read_u32(game.addr()));
    core::ptr::write_unaligned(((out + club_match_record::ROUND_PREFIX) as *mut u8) as *mut u16, game.round_prefix() as u16);
    core::ptr::write_unaligned(((out + club_match_record::ROUND_NAME) as *mut u8) as *mut u16, game.round_name() as u16);
    *((out + club_match_record::VENUE) as *mut u8) = exe::comp_util_get_fixture_venue_side(club, game);
    core::ptr::write_unaligned(((out + club_match_record::CLUB_ID) as *mut u8) as *mut u32, read_u32(club));
    core::ptr::write_unaligned(((out + club_match_record::MATCH_STATS_SLOT) as *mut u8) as *mut u32, game.match_stats_slot() as u32);
    if club as u32 == (game.home_club().addr() as u32) { // cpz7: the club is the home team
        core::ptr::write_unaligned(((out + club_match_record::OPPONENT_ID) as *mut u8) as *mut u32, game.away_club_id() as u32);
        if (game.home_goals_extra_time() as u8) as i8 > -1 {
            *((out + club_match_record::GOALS_FOR) as *mut u8) = game.home_goals_extra_time() as u8;
            *((out + club_match_record::GOALS_AGAINST) as *mut u8) = game.away_goals_extra_time() as u8;
        } else {
            covered!("cpz_7 home side score");
            *((out + club_match_record::GOALS_FOR) as *mut u8) = game.home_goals() as u8; // cpz7: the home team goals
            *((out + club_match_record::GOALS_AGAINST) as *mut u8) = game.away_goals() as u8;
        }
    } else {
        core::ptr::write_unaligned(((out + club_match_record::OPPONENT_ID) as *mut u8) as *mut u32, game.home_club_id() as u32);
        if (game.away_goals_extra_time() as u8) as i8 > -1 {
            *((out + club_match_record::GOALS_FOR) as *mut u8) = game.away_goals_extra_time() as u8;
            *((out + club_match_record::GOALS_AGAINST) as *mut u8) = game.home_goals_extra_time() as u8;
        } else {
            covered!("cpz_7 away side score");
            *((out + club_match_record::GOALS_FOR) as *mut u8) = game.away_goals() as u8;
            *((out + club_match_record::GOALS_AGAINST) as *mut u8) = game.home_goals() as u8; // cpz7: the goals conceded by the away team
        }
    }
    1
}

/// Name id (+0x33 of a name record), or -1 without name.
pub(crate) unsafe fn name_id(p: u32) -> u32 {
    if p != 0 { read_u32(p as usize + names::ID) } else { u32::MAX }
}

/// Stores the three name ids (−1 when missing), the person's id and birth date in a compact reference.
///
/// `fill_staff_name_ref(person, out)`, from the club and competition records, the suspension news, the award and the
/// player search. Null person or output: the box (lines 0x80, 0x88) and 0. Otherwise +0, +4, +8 the name ids
/// (`staff::FIRST_NAME` and the two after it; −1 with no name), `record_holder::STAFF_ID` the person's id and
/// `record_holder::DATE` the birth date (`staff::DATE_OF_BIRTH`, `exe::date_operator_3d`); returns 1.
/// Original: `fill_staff_name_ref` (0x7e4920, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn fill_staff_name_ref(person: usize, out: usize) -> u32 {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0xa7a088, 0x80);
        return 0;
    }
    if out == 0 {
        show_error_at(0xa7a088, 0x88);
        return 0;
    }
    for k in 0..3 {
        write_u32(out + 4 * k, name_id(read_u32(person.addr() + staff::FIRST_NAME + 4 * k)));
    }
    write_u32(out + record_holder::STAFF_ID, read_u32(person.addr()));
    let mut t = [0u8; 8];
    exe::date_operator_3d(out + record_holder::DATE, t.as_mut_ptr() as usize, person.addr() + staff::DATE_OF_BIRTH);
    1
}

/// Returns the result code of the record: 3 win, 2 score draw, 1 goalless draw, −3 defeat.
///
/// `match_record_result_code(record)`, from the competition statistics (`exe::comp_stats_record_match_result`, on each
/// match). Null record: the box (line 0x25a) and 0. For (`club_match_record::GOALS_FOR`) > against
/// (`club_match_record::GOALS_AGAINST`), signed: 3; draw with goals: 2; goalless draw: 1; defeat: −3 (only the byte
/// counts: the caller reads al).
/// Original: `match_record_result_code` (0x7e56d0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_record_result_code(record: usize) -> u8 {
    if record == 0 {
        show_error_at(0xa7a088, 0x25a);
        return 0;
    }
    let (p, c) = (read_u8(record + club_match_record::GOALS_FOR) as i8, read_u8(record + club_match_record::GOALS_AGAINST) as i8);
    if p > c {
        3
    } else if p == c && p > 0 {
        2
    } else if p == c {
        1
    } else {
        (-3i8) as u8
    }
}

/// Body of Staff_Records::add_year_award_record (`add_year_award_record`); without gravar (the comparer's copy) skips the history file
/// entry.
pub(crate) unsafe fn add_year_award(this: usize, p: Staff, club: usize, prize: usize, a4: u32, a5: u32, credito: u32, write: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x34;
    put_i32(&mut e, 8, if club != 0 { read_usize(club) as i32 } else { -1 });
    put_i32(&mut e, 0xd, read_usize(prize) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xffff) as i32);
    put_i32(&mut e, 0x15, a5 as u8 as i8 as i32);
    put_i32(&mut e, 0x19, credito as u8 as i8 as i32);
    append_person_history_by_id(write, this, read_usize(p.addr()), &e);
    let (t3d, t28) = (p.job_for_club(), p.job_for_nation());
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = ManagerRecord(staff_records_manager_record(this, p));
        if !r.is_null() {
            covered!("cpy_10 manager award");
            let a = (r.addr() + manager_record::AWARDS) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
        }
        return;
    }
    if p.player().is_null() {
        return;
    }
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if c.is_null() || c.contract_type() & 0x3f == 5 {
        return;
    }
    if read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE)) == 0 {
        return;
    }
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, contract_club_except_type5(p)));
    if k.is_null() {
        return;
    }
    if read_usize(G_TRANSFER_MANAGER) != 0 {
        if exe::date_days_since(G_CURRENT_DATE, (k.date_started()) as u32, read_usize(k.addr() + contract::DATE_STARTED + 4) as u32) > 0x12c { // cpy10: mais de 300 dias
            let owner = contract_club_except_type5(p);
            let x = ShortlistSlot(exe::shortlist_manager_find_club_shortlist_entry(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), owner, p, 0, 0));
            if !x.is_null() && x.target_staff_id() == read_usize(p.addr()) {
                covered!("cpy_10 renewal");
                offer_renewal(p.addr());
            }
        }
    }
    if credito as u8 != 1 {
        covered!("cpy_10 without credit");
        return; // cpy10: só o creditado
    }
    if (k.grievances() as usize) != 0 {
        return;
    }
    let v59 = (p.loyalty() as u8) as i8;
    if v59 <= 0xf { // cpy10: o +0x59
        return;
    }
    if (p.professionalism() as u8) as i8 <= 0xa {
        return;
    }
    covered!("cpy_10 thanks draw");
    if random(v59 as i32) <= 5 {
        return;
    }
    exe::news_post_player_credits_manager_for_award(p, club, prize);
}

/// Tells whether the match record is a win by a larger margin than the other one, or the same margin with more goals
/// scored.
///
/// `match_record_is_bigger_win(a, b)`, from the club records after each match (0x444930) and from the competition ones
/// (0x49fb93): 1 if `a` is a win (goals `club_match_record::GOALS_FOR` > `club_match_record::GOALS_AGAINST`, signed
/// bytes) by a larger margin than that of `b`, or equal with more goals scored. Null record: the original routine
/// ("Error" box, 0).
/// Original: `match_record_is_bigger_win` (0x7e4a30, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_WIN)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (read_u8(a + club_match_record::GOALS_FOR) as i8, read_u8(a + club_match_record::GOALS_AGAINST) as i8, read_u8(b + club_match_record::GOALS_FOR) as i8, read_u8(b + club_match_record::GOALS_AGAINST) as i8);
    if fa <= sa {
        return 0;
    }
    let (da, db) = (fa as i32 - sa as i32, fb as i32 - signed_byte_at as i32);
    (da > db || (da == db && fa > fb)) as u32
}

/// Tells whether the match record is a defeat by a larger margin than the other one, or the same margin with more goals
/// conceded.
///
/// `match_record_is_bigger_defeat(a, b)`, from the club records after each match (0x444930): 1 if `a` is a defeat
/// (`club_match_record::GOALS_FOR` < `club_match_record::GOALS_AGAINST`) by a larger margin than that of `b`, or equal
/// with more goals conceded. Null record: the original routine ("Error" box, 0).
/// Original: `match_record_is_bigger_defeat` (0x7e4b00, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    if a == 0 || b == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_DEFEAT)(a, b);
    }
    let (fa, sa, fb, signed_byte_at) = (read_u8(a + club_match_record::GOALS_FOR) as i8, read_u8(a + club_match_record::GOALS_AGAINST) as i8, read_u8(b + club_match_record::GOALS_FOR) as i8, read_u8(b + club_match_record::GOALS_AGAINST) as i8);
    if fa >= sa {
        return 0;
    }
    let (da, db) = (sa as i32 - fa as i32, signed_byte_at as i32 - fb as i32);
    (da > db || (da == db && sa > signed_byte_at)) as u32
}

/// Rule: the treatment confirm screen opens for a player with a club: a leave of absence when asked, otherwise the
/// treatment of his recurring injury type.
///
/// `open_treatment_confirm_screen(person, leave)`: nothing with no person or no `staff::CLUB_JOB`. `open_screen` with
/// the builder 0x7e5c00, the handler 0x7e5e30, single instance, the no-op leave routine 0x5a8f60 and 0; when it opens,
/// the kind: 'Q' with `leave` (a char), otherwise `Injury::get_recurring_injury_type(person)` on `G_INJURY` − 0xb
/// through `TREATMENT_CASE` ('N', 'O' or 'M'; outside 0..0x30 'M'). Parameter 0 = the person's id, 1 = the kind
/// (sign-extended), 2 = the club's id.
/// Original: `open_treatment_confirm_screen` (0x7e5b10, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_treatment_confirm_screen(person: usize, leave: u32) {
    let person = Staff(person);
    if person.is_null() || person.club_job().addr() == 0 {
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x7e5c00usize, 0x7e5e30usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel36_1 open the treatment confirm");
    let kind = if leave as u8 != 0 {
        TREATMENT_LEAVE
    } else {
        let t = (exe::injury_get_recurring_injury_type(read_usize(G_INJURY), person) as i8 as i32).wrapping_sub(0xb) as u32;
        match TREATMENT_CASE.get(t as usize) {
            Some(0) => TREATMENT_PHYSIO,
            Some(1) => TREATMENT_SURGERY,
            _ => TREATMENT_FAITH_HEALER,
        }
    };
    set_screen_param(0, read_u32(person.addr()) as usize);
    set_screen_param(1, kind as i8 as isize as usize);
    set_screen_param(2, read_u32(person.club_job().addr()) as usize);
}

/// Rule: the treatment confirm screen asks whether to send the player on a two-week leave of absence, to a faith
/// healer, to physiotherapy or to surgery (by the kind), with No / Yes.
///
/// `draw_treatment_confirm_screen()`: `tel36_confirm_build` with the question by the kind (parameter 1): 'Q' the leave
/// of absence (0xa7a3cc), 'M' the faith healer (0xa7a318), 'N' the physiotherapy (0xa7a27c), 'O' the surgery
/// (0xa7a1d8), each with the player's name; another kind ('P') leaves "Please Confirm" in the text.
/// Original: `draw_treatment_confirm_screen` (0x7e5c00, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_treatment_confirm_screen() {
    covered!("tel36_1 draw the treatment confirm");
    let kind = get_screen_param(1) as u8;
    tel36_confirm_build(|name| {
        let question = match kind {
            TREATMENT_LEAVE => 0xa7a3cc,
            TREATMENT_FAITH_HEALER => 0xa7a318,
            TREATMENT_PHYSIO => 0xa7a27c,
            TREATMENT_SURGERY => 0xa7a1d8,
            _ => return,
        };
        tel36_langlib()(JOB_TEXT, question, name);
    });
}

/// Rule: confirming a treatment pays it from the club's finances (except a leave of absence): 22,500 for
/// physiotherapy, 75,000 for surgery, 7,500 otherwise; then the player gets the treatment as an injury and loses the
/// low flag of his state record.
///
/// `handle_treatment_confirm_screen(item)`: `tel36_confirm_answer`; "Yes": with a club (`staff::CLUB_JOB`) and a kind
/// other than 'Q', `FinanceTable::get_club_finance(club)` on `G_FINANCE_TABLE` and, when there is one,
/// `Finance::pay_other_costs` (0x5999e0) with 22500 ('N'), 75000 ('O') or 7500; then `Injury::add_injury(kind, 0,
/// person, 0, 1, -1, 0, 0)` on `G_INJURY` and bit 0 of byte +0xc of the person's record in `G_STAFF_STATE_TABLE`
/// (0x4f bytes each) cleared.
/// Original: `handle_treatment_confirm_screen` (0x7e5e30, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn handle_treatment_confirm_screen(raw: u32) -> i32 {
    let person = Staff(tel36_staff_param(0));
    let kind = get_screen_param(1) as u8;
    tel36_confirm_answer(raw, || {
        covered!("tel36_1 confirm the treatment");
        let club = person.club_job().addr();
        if club != 0 && kind != TREATMENT_LEAVE {
            let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            if finance != 0 {
                let cost = match kind {
                    TREATMENT_PHYSIO => 0x57e4,
                    TREATMENT_SURGERY => 0x124f8,
                    _ => 0x1d4c,
                };
                exe::finance_pay_other_costs(finance, cost);
            }
        }
        exe::injury_add_injury(read_usize(G_INJURY), kind as u32, 0, person, 0, 1, -1, 0, 0);
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(0x4f)) + 0xc;
        write_u8(state, read_u8(state) & 0xfe);
    })
}

/// Comparer for `open_treatment_confirm_screen`.
pub(crate) unsafe extern "C" fn compare_open_treatment_confirm_screen(person: usize, leave: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_OPEN_TREATMENT_CONFIRM_SCREEN)(person, leave)
}

/// Comparer for `draw_treatment_confirm_screen`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_treatment_confirm_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_DRAW_TREATMENT_CONFIRM_SCREEN);
    compare_by_display_list_with("treatment confirm screen (draw_treatment_confirm_screen)", &[], || o(), || draw_treatment_confirm_screen());
}

/// Comparer for `handle_treatment_confirm_screen`.
pub(crate) unsafe extern "C" fn compare_handle_treatment_confirm_screen(item: u32) -> i32 {
    // simplification: the payment, the injury and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_HANDLE_TREATMENT_CONFIRM_SCREEN)(item)
}

/// Whether the first match record has more goals in all than the second.
///
/// `match_record_has_more_goals(a, b)`. A missing record shows the game's error box (line 0x107) and gives 0. When
/// a's home goals plus b's away goals make 0 the answer is 0 (the original mixes the two records in this check: a
/// defect, kept); otherwise a's home + away goals above b's.
/// Original: `match_record_has_more_goals` (0x7e4be0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn match_record_has_more_goals(a: usize, b: usize) -> u32 {
    use record_utils_consts as ru;
    if a == 0 || b == 0 {
        show_error_at(ru::RECORD_UTILS_CPP, 0x107);
        return 0;
    }
    let a_home = read_i8(a + ru::HOME_GOALS) as i32;
    let b_away = read_i8(b + ru::AWAY_GOALS) as i32;
    if a_home + b_away == 0 {
        covered!("mwg_8 no goals");
        return 0;
    }
    covered!("mwg_8 goals compared");
    let b_total = read_i8(b + ru::HOME_GOALS) as i32 + b_away;
    let a_total = read_i8(a + ru::AWAY_GOALS) as i32 + a_home;
    (a_total > b_total) as u32 // mwg8: more goals
}

/// Comparer for `match_record_has_more_goals`.
pub(crate) unsafe extern "C" fn compare_match_record_has_more_goals(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_HAS_MORE_GOALS)(a, b);
    let n = match_record_has_more_goals(a, b);
    SCOREBOARD.record(r == n, || format!("match_record_has_more_goals({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Writes a number of days as words: approximate ("about a week") or exact ("3 weeks").
///
/// `format_duration_text(days, out, size, approximate)`. A buffer under 12 bytes shows the game's error box (line
/// 0x288) and gives 0; otherwise 1. Under one day the text is empty. Approximate: 1 "one day", under 6 "a few days",
/// under 9 "about a week", under 14 "about ten days", under 28 "about N weeks" (days / 7), under 40 "about a month",
/// under 365 "about N months" (days / 30 + 1), under 450 "about a year", under 630 "about 18 months", under 910 "about
/// 2 years", otherwise "about N years" (days / 365). Exact: under 14 "1 day" or "N days", under 43 "1 week" or "N
/// weeks" (days / 7), otherwise "1 month" or "N months" ((days − 15) / 30 + 1). The texts are translated
/// (`FN_LANGLIB_SPRINTF`).
/// Original: `format_duration_text` (0x7e5780, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn format_duration_text(days: i32, out: usize, size: i32, approximate: u32) -> u32 {
    use record_utils_consts as ru;
    if size < 12 {
        show_error_at(ru::RECORD_UTILS_CPP, 0x288);
        return 0;
    }
    let write: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if days < 1 {
        write_u8(out, 0);
        return 1;
    }
    if approximate != 0 {
        covered!("mwg_8 approximate duration");
        match days {
            1 => write(out, ru::ONE_DAY),
            2..=5 => write(out, ru::A_FEW_DAYS),
            6..=8 => write(out, ru::ABOUT_A_WEEK),
            9..=13 => write(out, ru::ABOUT_TEN_DAYS),
            14..=27 => write(out, ru::ABOUT_N_WEEKS, days / 7), // mwg8: about N weeks
            28..=39 => write(out, ru::ABOUT_A_MONTH),
            40..=364 => write(out, ru::ABOUT_N_MONTHS, days / 30 + 1),
            365..=449 => write(out, ru::ABOUT_A_YEAR),
            450..=629 => write(out, ru::ABOUT_18_MONTHS),
            630..=909 => write(out, ru::ABOUT_2_YEARS),
            _ => write(out, ru::ABOUT_N_YEARS, days / 365),
        };
    } else if days < 14 {
        covered!("mwg_8 exact duration");
        if days == 1 { write(out, ru::EXACT_1_DAY) } else { write(out, ru::EXACT_N_DAYS, days) };
    } else if days < 43 {
        let weeks = days / 7;
        if weeks == 1 { write(out, ru::EXACT_1_WEEK) } else { write(out, ru::EXACT_N_WEEKS, weeks) };
    } else {
        let months = (days - 15) / 30 + 1;
        if months == 1 { write(out, ru::EXACT_1_MONTH) } else { write(out, ru::EXACT_N_MONTHS, months) };
    }
    1
}

/// Comparer for `format_duration_text`: side by side over the output buffer.
pub(crate) unsafe extern "C" fn compare_format_duration_text(days: i32, out: usize, size: i32, approximate: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, i32, u32) -> u32>(IDX_FORMAT_DURATION_TEXT);
    let regions = [(out, size.clamp(0, 256) as usize)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(days, out, size, approximate), || format_duration_text(days, out, size, approximate));
    SCOREBOARD.record(equal && r == n, || format!("format_duration_text({}, approximate {}) original={} {:x?} rust={} {:x?}", days, approximate, r, mo, n, mr));
    r
}

/// The "Please Confirm" screen of a person and a club (the screens of record_utils.cpp and ultimatum.cpp): the
/// person of parameter 0, the club of parameter 2. The club's background picture, the menu bar (4, 0), the title (the
/// person's name, 2000 bytes, style 1, in the club's kit 1 colours), the subtitle "Please Confirm", the question
/// written by `question` into the screens' text buffer (it gets the person's name), the question as a text area
/// (0x6e, 0x91)-(0x30c, 0x217) (font 4, pale grey) and a two-column area (weights 3/1) with "No" (id 2) and "Yes"
/// (id 3).
pub(crate) unsafe fn tel36_confirm_build(question: impl FnOnce(*const u8)) {
    let text = tel36_langlib();
    let weights: [u8; 2] = [3, 1];
    let person = tel36_staff_param(0);
    let club = tel36_club_param(2);
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 0x7d0];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    question(name.as_ptr());
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
}

/// The answer of a "Please Confirm" screen: in a network game (`TEL35_NETWORK_FLAG`) the pending command argument is
/// cleared and -9 comes back; "Yes" (id 3) runs `yes` and closes with -9, "No" (id 2) closes with -9, other widgets go
/// to the menu bar.
pub(crate) unsafe fn tel36_confirm_answer(raw: u32, yes: impl FnOnce()) -> i32 {
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    match tel26_item_id(raw as i16) {
        3 => {
            yes();
            -9
        }
        2 => -9,
        _ => manager_menu_bar(raw),
    }
}

/// The treatment of each recurring injury type from 0xb to 0x3b (`Injury::get_recurring_injury_type` − 0xb): 0 a
/// physiotherapy ('N'), 1 a surgery ('O'), 2 a faith healer ('M'); outside the table also the faith healer.
pub(crate) const TREATMENT_CASE: [u8; 0x31] = [0, 1, 1, 2, 2, 2, 0, 1, 2, 1, 2, 2, 0, 2, 1, 1, 1, 1, 2, 0, 1, 2, 2, 2, 0, 1, 1, 2, 2, 2, 0, 2, 2,
    1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0];

/// The treatment kinds of the screen (parameter 1): 'M' faith healer, 'N' physiotherapy, 'O' surgery, 'Q' leave of
/// absence ('P' has no question).
pub(crate) const TREATMENT_FAITH_HEALER: u8 = 0x4d;

pub(crate) const TREATMENT_PHYSIO: u8 = 0x4e;

pub(crate) const TREATMENT_SURGERY: u8 = 0x4f;

pub(crate) const TREATMENT_LEAVE: u8 = 0x51;
