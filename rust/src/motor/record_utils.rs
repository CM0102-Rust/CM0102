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
    let total = read_i8(a + ru::AWAY_GOALS) as i32 + a_home;
    (total > b_total) as u32 // mwg8: more goals
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

/// Stores the eleven fields of a club match record (`match_record_slot_values`) in the news item's slots from `*k` on
/// and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x160, 0x168) and returns 0.
/// Original: `write_match_record_to_news` (0x7e4de0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x160);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x168);
        return 0;
    }
    covered!("sx18_4 match record to news");
    news_put_slots(news, k, &match_record_slot_values(record));
    1
}

/// Reads a club match record back from the news item's slots from `*k` on and advances `*k`: the date from the first
/// two (day, year), then the fixture id, the round prefix and name (words), the venue (byte), the club and opponent
/// ids, the goals for and against (bytes) and the match statistics slot; returns 1. A null news item or record shows
/// the error box (lines 0x18e, 0x196) and returns 0.
/// Original: `read_match_record_from_news` (0x7e4fa0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x18e);
        return 0;
    }
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x196);
        return 0;
    }
    covered!("sx18_4 match record from news");
    let [day, year] = news_take_slots::<2>(news, k);
    news_slot_date(record, day, year);
    let v = news_take_slots::<9>(news, k);
    write_u32(record + club_match_record::FIXTURE_ID, v[0]);
    write_u16(record + club_match_record::ROUND_PREFIX, v[1] as u16);
    write_u16(record + club_match_record::ROUND_NAME, v[2] as u16);
    write_u8(record + club_match_record::VENUE, v[3] as u8);
    write_u32(record + club_match_record::CLUB_ID, v[4]);
    write_u32(record + club_match_record::OPPONENT_ID, v[5]);
    write_u8(record + club_match_record::GOALS_FOR, v[6] as u8);
    write_u8(record + club_match_record::GOALS_AGAINST, v[7] as u8); // sx18_4: the goals against
    write_u32(record + club_match_record::MATCH_STATS_SLOT, v[8]);
    1
}

/// Stores a staff name reference (`staff_name_ref_slot_values`: the three name ids, the staff id, the date) in the news
/// item's slots from `*k` on and advances `*k`; returns 1. A null reference or news item shows the error box (lines
/// 0x1bb, 0x1c3) and returns 0.
/// Original: `write_staff_name_ref_to_news` (0x7e5160, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1bb);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1c3);
        return 0;
    }
    covered!("sx18_4 name reference to news");
    news_put_slots(news, k, &staff_name_ref_slot_values(r));
    1
}

pub(crate) unsafe extern "C" fn compare_write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD_TO_NEWS);
    if record == 0 || news == 0 {
        return o(record, news, k);
    }
    compare_news_slot_writer(news, k, &match_record_slot_values(record), || o(record, news, k), "write_match_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_MATCH_RECORD_FROM_NEWS);
    if news == 0 || record == 0 {
        return o(news, k, record);
    }
    compare_news_slot_reader(k, record, 0x1f, || o(news, k, record), |kc, rc| read_match_record_from_news(news, kc, rc), "read_match_record_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_STAFF_NAME_REF_TO_NEWS);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &staff_name_ref_slot_values(r), || o(r, news, k), "write_staff_name_ref_to_news")
}

/// Reads a staff name reference back from the news item's slots from `*k` on and advances `*k`: the three name ids and
/// the staff id (dwords), then the date from the next two (day, year); returns 1. A null news item or reference shows
/// the error box (lines 0x1e4, 0x1ec) and returns 0.
/// Original: `read_staff_name_ref_from_news` (0x7e52c0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1e4);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1ec);
        return 0;
    }
    covered!("sx18_5 name reference from news");
    let v = news_take_slots::<6>(news, k);
    write_u32(r, v[0]); // sx18_5: the first name id
    write_u32(r + record_holder::SECOND_NAME_ID, v[1]);
    write_u32(r + record_holder::COMMON_NAME_ID, v[2]);
    write_u32(r + record_holder::STAFF_ID, v[3]);
    news_slot_date(r + record_holder::DATE, v[4], v[5]);
    1
}

/// Stores a span record (`date_span_slot_values`: the value, the start and end dates) in the news item's slots from
/// `*k` on and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x20c, 0x214) and
/// returns 0.
/// Original: `write_date_span_record_to_news` (0x7e5420, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x20c);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x214);
        return 0;
    }
    covered!("sx18_5 span record to news");
    news_put_slots(news, k, &date_span_slot_values(r));
    1
}

/// Reads a span record back from the news item's slots from `*k` on and advances `*k`: the value (+0), then the start
/// date and the end date, each from two slots (day, year); returns 1. A null news item or record shows the error box
/// (lines 0x234, 0x23c) and returns 0.
/// Original: `read_date_span_record_from_news` (0x7e5570, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x234);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x23c);
        return 0;
    }
    covered!("sx18_5 span record from news");
    let [value, d0, y0] = news_take_slots::<3>(news, k);
    write_u32(r, value);
    news_slot_date(r + run_record::START_DATE, d0, y0);
    let [d1, y1] = news_take_slots::<2>(news, k);
    news_slot_date(r + run_record::END_DATE, d1, y1); // sx18_5: the end date
    1
}

pub(crate) unsafe extern "C" fn compare_read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_STAFF_NAME_REF_FROM_NEWS);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x18, || o(news, k, r), |kc, rc| read_staff_name_ref_from_news(news, kc, rc), "read_staff_name_ref_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_DATE_SPAN_RECORD_TO_NEWS);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &date_span_slot_values(r), || o(r, news, k), "write_date_span_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_DATE_SPAN_RECORD_FROM_NEWS);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x14, || o(news, k, r), |kc, rc| read_date_span_record_from_news(news, kc, rc), "read_date_span_record_from_news")
}

/// Whether both clubs of a fixture can field a reserve side (`reserve_squad`, `reserve_side_ok`).
///
/// With `check_date`: not for a fixture with a round name (`fixture::ROUND_NAME` ≠ 0), nor when the home club is
/// Turkish (`club::NATION`) and its division is not the Turkish one, nor for the round prefixes 0x418, 0x419 and 0xa0,
/// nor on 26 December of the fixture's year. A home club in the Scottish Premier Division or Turkish needs 15 young
/// players (21 or under at the fixture's date); the others none (−1). Both clubs must have a reserve squad
/// (`database_club_has_reserve_squad`); then the home club's side and the away club's are checked in turn.
/// Original: `ReserveComp::can_clubs_field_reserve_sides` (0x7e6ba0, record_utils.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn can_clubs_field_reserve_sides(_this: usize, fixture: usize, check_date: u32) -> u32 {
    let fixture = Fixture(fixture);
    let f = fixture;
    let nation_is_turkey = |club: usize| read_i32(read_usize(club + club::NATION)) == read_i32(KEY_NATION_TURKEY);
    if check_date != 0 {
        let home = f.home_club().addr();
        if (fixture.round_name() as u16) != 0 {
            return 0;
        }
        if nation_is_turkey(home) {
            let division = read_usize(home + club::DIVISION);
            if division == 0 || read_i32(division) != read_i32(KEY_RESERVE_TURKISH_DIVISION) {
                return 0;
            }
        }
        if matches!(fixture.round_prefix() as u16, 0x418 | 0x419 | 0xa0) {
            return 0;
        }
        let (mut boxing_day, mut played) = ([0u16; 4], [0u8; 8]);
        exe::date_date_549f70(&mut boxing_day, 0x1a, 0xb, f.year() as u32, u32::MAX);
        let p = exe::new_date(played.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
        if boxing_day[1] == read_u16(p + 2) && boxing_day[0] == read_u16(p) { // sx18_28: no reserve match on 26 December
            return 0;
        }
    }
    let (mut min_young, mut when) = (-1i8, [0u16; 4]);
    exe::date_date_54a110(when.as_mut_ptr() as usize);
    let home = f.home_club().addr();
    if home != 0 {
        let division = read_usize(home + club::DIVISION);
        if division != 0 && (read_i32(division) == read_i32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) || nation_is_turkey(home)) {
            covered!("sx18_28 reserve sides with young players");
            min_young = 0xf;
            let mut t = [0u8; 8];
            let p = exe::new_date(t.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
            copy_record_date(when.as_mut_ptr() as usize, p);
        }
    }
    let away = f.away_club().addr();
    if exe::database_club_has_reserve_squad(home) == 0 || exe::database_club_has_reserve_squad(away) == 0 {
        return 0;
    }
    covered!("sx18_28 reserve sides counted");
    for club in [home, away] {
        let s = reserve_squad(club, min_young, when.as_ptr());
        if !reserve_side_ok(club, &s, min_young) {
            return 0;
        }
    }
    1
}

/// Removes today's reserve fixtures that cannot be played: for each active key nation's fixture list of today
/// (`fix_man_get_fixture_list_for_date` with today, the day slice and the index), each fixture of the reserve
/// competition or of the French CFA or lower division whose clubs cannot field reserve sides
/// (`can_clubs_field_reserve_sides` without the date checks) is copied to a local list; then each one is removed
/// (`FIXTURE_MANAGER::remove_fixture`).
/// Original: `ReserveComp::cancel_unplayable_reserve_fixtures` (0x7e6a70, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn cancel_unplayable_reserve_fixtures(this: usize) {
    for fixture in unplayable_reserve_fixtures(this) {
        covered!("sx18_28 reserve fixture cancelled");
        exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), fixture.as_ptr() as usize);
    }
}

/// Registers the reserve competition and builds its two sub competitions: the reserve competition object goes to its
/// slot of `G_COMP_OBJECT_TABLE` (`KEY_CLUB_COMP_RESERVE`); a B international competition (0xa3 bytes, `Comp::Comp`,
/// vtable 0x96f074, its record `KEY_CLUB_COMP_B_INTERNATIONAL`) goes to +0xa3 and to its slot, and an Under 21
/// international one (vtable 0x96efe4, `KEY_CLUB_COMP_UNDER_21_INTERNATIONAL`) to +0xa7 and to its slot (0 when out of
/// memory). Then, with the French first division built, the French CFA's slot gets its object's +0xee and the French
/// lower division's its +0xf2; with the Spanish first division built, the Spanish lower division's slot gets its +0xee.
/// Original: `ReserveComp::create_sub_comps` (0x7e61c0, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reserve_comp_create_sub_comps(this: usize) {
    let slot = |key: usize| read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4));
    write_usize(slot(KEY_CLUB_COMP_RESERVE), this);
    for (at, vtable, key) in [(0xa3usize, 0x96f074u32, KEY_CLUB_COMP_B_INTERNATIONAL), (0xa7, 0x96efe4, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)] {
        let p = exe::crt_operator_new(0xa3);
        let comp = if p != 0 {
            exe::new_comp(p);
            write_u32(p, vtable);
            write_usize(p + 4, reserve_comp_record(key)); // the competition's record
            p
        } else {
            0
        };
        write_usize(this + at, comp); // the B (+0xa3) and Under 21 (+0xa7) international competitions
        write_usize(slot(key), comp);
    }
    covered!("sx18_28 reserve sub competitions");
    let french = read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION));
    if french != 0 {
        write_usize(slot(KEY_CLUB_COMP_FRENCH_CFA), read_usize(french + 0xee)); // the French first division's CFA object
        write_usize(slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION), read_usize(read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION)) + 0xf2));
    }
    let spanish = read_usize(slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION));
    if spanish != 0 {
        write_usize(slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION), read_usize(spanish + 0xee)); // the Spanish first division's lower object
    }
}

/// Comparer for `cancel_unplayable_reserve_fixtures`: the fixtures cannot be removed twice; the copy lists the
/// fixtures it would remove first (their first 0x14 bytes: competition, stadium, statistics slot and clubs), then only
/// the original runs; none of them may be left among today's fixtures.
pub(crate) unsafe extern "thiscall" fn compare_cancel_unplayable_reserve_fixtures(this: usize) {
    let expected: Vec<[u8; 0x14]> = unplayable_reserve_fixtures(this).iter().map(|f| f[..0x14].try_into().unwrap()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CANCEL_UNPLAYABLE_RESERVE_FIXTURES)(this);
    let mut left = 0;
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = exe::llist_get_next(list);
                if fixture == 0 {
                    break;
                }
                if expected.iter().any(|e| core::slice::from_raw_parts(fixture as *const u8, 0x14) == &e[..]) {
                    left += 1;
                }
            }
        }
        k += 1;
    }
    SCOREBOARD.record(left == 0, || format!("cancel_unplayable_reserve_fixtures: {} of {} unplayable fixtures still there", left, expected.len()));
}

/// Comparer for `reserve_comp_create_sub_comps`: it allocates the sub competitions (it cannot run twice); only the
/// original runs and the table slots it leaves are checked.
pub(crate) unsafe extern "thiscall" fn compare_reserve_comp_create_sub_comps(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESERVE_COMP_CREATE_SUB_COMPS)(this);
    let slot = |key: usize| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4)));
    let (b, u21) = (read_usize(this + 0xa3), read_usize(this + 0xa7));
    let mut ok = slot(KEY_CLUB_COMP_RESERVE) == this && slot(KEY_CLUB_COMP_B_INTERNATIONAL) == b && slot(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) == u21
        && (b == 0 || (read_u32(b) == 0x96f074 && read_usize(b + 4) == reserve_comp_record(KEY_CLUB_COMP_B_INTERNATIONAL)))
        && (u21 == 0 || (read_u32(u21) == 0x96efe4 && read_usize(u21 + 4) == reserve_comp_record(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)));
    let french = slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION);
    if french != 0 {
        ok &= slot(KEY_CLUB_COMP_FRENCH_CFA) == read_usize(french + 0xee) && slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) == read_usize(french + 0xf2);
    }
    let spanish = slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION);
    if spanish != 0 {
        ok &= slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) == read_usize(spanish + 0xee);
    }
    SCOREBOARD.record(ok, || format!("reserve_comp_create_sub_comps({:#x}): B {:#x}, Under 21 {:#x}", this, b, u21));
}

/// Stores `values` in the news item's slots from `*k` on (`NewsItem::set_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_put_slots(news: usize, k: usize, values: &[u32]) {
    for &v in values {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        exe::newsitem_set_slot(news, slot as i8, v);
    }
}

/// Reads `N` slots of the news item from `*k` on (`NewsItem::get_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_take_slots<const N: usize>(news: usize, k: usize) -> [u32; N] {
    let mut v = [0u32; N];
    for x in v.iter_mut() {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        *x = exe::newsitem_get_slot(news, slot as u32);
    }
    v
}

/// Writes the date (day, year) read from two slots into `at` (`Date::Date` then `Date::operator=`).
pub(crate) unsafe fn news_slot_date(at: usize, day: u32, year: u32) {
    let mut d = [0u8; 8];
    let p = exe::new_date(d.as_mut_ptr() as usize, day, year);
    copy_record_date(at, p);
}

/// The eleven slot values of a club match record, in the original's order: the date (day, year), the fixture id, the
/// round prefix and name words (sign-extended), the venue byte (sign-extended), the club and opponent ids, the goals for
/// and against (sign-extended) and the match statistics slot.
pub(crate) unsafe fn match_record_slot_values(record: usize) -> [u32; 11] {
    let [day, year] = date_slot_values(record);
    [day, year, read_u32(record + club_match_record::FIXTURE_ID),
     read_i16(record + club_match_record::ROUND_PREFIX) as i32 as u32, read_i16(record + club_match_record::ROUND_NAME) as i32 as u32,
     read_i8(record + club_match_record::VENUE) as i32 as u32, read_u32(record + club_match_record::CLUB_ID),
     read_u32(record + club_match_record::OPPONENT_ID), read_i8(record + club_match_record::GOALS_FOR) as i32 as u32, // sx18_4: the goals for
     read_i8(record + club_match_record::GOALS_AGAINST) as i32 as u32, read_u32(record + club_match_record::MATCH_STATS_SLOT)]
}

/// The six slot values of a staff name reference: the three name ids, the staff id and the date (day, year).
pub(crate) unsafe fn staff_name_ref_slot_values(r: usize) -> [u32; 6] {
    let [day, year] = date_slot_values(r + record_holder::DATE);
    [read_u32(r), read_u32(r + record_holder::SECOND_NAME_ID), read_u32(r + record_holder::COMMON_NAME_ID),
     read_u32(r + record_holder::STAFF_ID), day, year]
}

/// Comparer of the slot writers: the copy's values are taken first, then only the original writes (a news item's slots
/// are written once); the slots it wrote from the old `*k` must hold the copy's values, and `*k` must have moved past
/// them.
pub(crate) unsafe fn compare_news_slot_writer(news: usize, k: usize, values: &[u32], original: impl FnOnce() -> u32, name: &str) -> u32 {
    let k0 = read_u8(k);
    let r = original();
    let got: Vec<u32> = (0..values.len()).map(|i| exe::newsitem_get_slot(news, k0.wrapping_add(i as u8) as u32)).collect();
    let ok = r == 1 && read_u8(k) == k0.wrapping_add(values.len() as u8) && got == values;
    SCOREBOARD.record(ok, || format!("{}(news {:#x}, slot {}): original {:x?}, copy {:x?}", name, news, k0, got, values));
    r
}

/// Comparer of the slot readers: the copy reads into a copy of the record with its own `*k` first, then the original
/// reads into the record; the `size` bytes of both records and the two `*k` must match.
pub(crate) unsafe fn compare_news_slot_reader(k: usize, record: usize, size: usize, original: impl FnOnce() -> u32, copy: impl FnOnce(usize, usize) -> u32, name: &str) -> u32 {
    let mut local = vec![0u8; size];
    core::ptr::copy_nonoverlapping(record as *const u8, local.as_mut_ptr(), size);
    let mut k_copy = read_u8(k);
    let n = copy(&mut k_copy as *mut u8 as usize, local.as_mut_ptr() as usize);
    let r = original();
    let same = core::slice::from_raw_parts(record as *const u8, size) == &local[..];
    SCOREBOARD.record(r == n && same && read_u8(k) == k_copy, || format!("{}: original {:x?} k {}, copy {:x?} k {}", name,
        core::slice::from_raw_parts(record as *const u8, size), read_u8(k), local, k_copy));
    r
}

/// The five slot values of a span record: the value (+0), the start date (day, year) and the end date (day, year).
pub(crate) unsafe fn date_span_slot_values(r: usize) -> [u32; 5] {
    let [d0, y0] = date_slot_values(r + run_record::START_DATE);
    let [d1, y1] = date_slot_values(r + run_record::END_DATE);
    [read_u32(r), d0, y0, d1, y1]
}

/// The competition record (0x6b bytes in `G_CLUB_COMP_TABLE`) of the competition whose id the key holds.
pub(crate) unsafe fn reserve_comp_record(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(0x6b))
}

pub(crate) unsafe fn reserve_squad(club: usize, min_young: i8, when: *const u16) -> ReserveSquad {
    let club = Club(club);
    let mut s = ReserveSquad { empty: 0, listed: 0, young: 0, available: 0, available_listed: 0 };
    for k in 0..50 {
        let p = Staff(club.squad(k));
        if p.is_null() {
            s.empty += 1;
            continue;
        }
        if p.club_job() != club || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 0 {
            continue;
        }
        let listed = read_i32(p.addr()) < read_i32(G_STAFF_COUNT);
        if listed {
            s.listed += 1;
        }
        if min_young != -1 && exe::database_get_staff_age(p, when) <= 0x15 {
            s.young += 1;
        }
        if p.playing_squad() & 1 == 0 {
            if listed {
                s.available_listed += 1;
            }
            s.available += 1;
        }
    }
    s
}

/// Whether a club can field its reserve side: with fewer than 11 available players, no more than 36 listed and enough
/// empty slots to make up 14; a human club needs 11 available and one of them listed; and the young players must reach
/// `min_young`.
pub(crate) unsafe fn reserve_side_ok(club: usize, s: &ReserveSquad, min_young: i8) -> bool {
    if s.available < 0xb && (s.listed > 0x24 || (s.empty as i32) < 0xe - s.available as i32) {
        return false;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && (s.available < 0xb || s.available_listed < 1) { // sx18_28: a human club's reserve side
        return false;
    }
    s.young >= min_young
}

/// Today's reserve fixtures whose clubs cannot field reserve sides, copied (`fixture::SIZE` bytes each), in order.
pub(crate) unsafe fn unplayable_reserve_fixtures(this: usize) -> Vec<[u8; fixture::SIZE]> {
    let mut out = Vec::new();
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = Fixture(exe::llist_get_next(list));
                if fixture.is_null() {
                    break;
                }
                let comp = fixture.competition().addr();
                if comp == 0 {
                    continue;
                }
                let id = read_i32(comp);
                let reserve = [KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&key| id == read_i32(key));
                if reserve && can_clubs_field_reserve_sides(this, fixture.addr(), 0) == 0 {
                    let mut copy = [0u8; fixture::SIZE];
                    core::ptr::copy_nonoverlapping(fixture.addr() as *const u8, copy.as_mut_ptr(), fixture::SIZE);
                    out.push(copy);
                }
            }
        }
        k += 1;
    }
    out
}

/// The two slot values of a date: the day word sign-extended, the year word zero-extended.
pub(crate) unsafe fn date_slot_values(at: usize) -> [u32; 2] {
    [read_i16(at) as i32 as u32, read_u16(at + 2) as u32]
}

/// A Turkish competition key the reserve rules require of a Turkish home club's division (no name in `keys.rs` yet).
pub(crate) const KEY_RESERVE_TURKISH_DIVISION: usize = 0x9cf8d4;

/// One club's squad for a reserve match, over the 50 `club::SQUAD` slots: the empty slots; and, among the players at
/// the club (`staff::CLUB_JOB` = the club) and not injured (`Injury::get_player_injury_status`), the listed ones (id
/// below `G_STAFF_COUNT`), the young ones (age at `when` up to 21, counted only when `min_young` is not −1), the
/// available ones (bit 0 of `staff::PLAYING_SQUAD` clear) and those of them listed.
pub(crate) struct ReserveSquad {
    pub(crate) empty: i8,
    pub(crate) listed: i8,
    pub(crate) young: i8,
    pub(crate) available: i8,
    pub(crate) available_listed: i8,
}
