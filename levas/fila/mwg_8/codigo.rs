// ---------- batch %%LEVA%%: comparing match records, a duration in words and a new loan offer (record_utils.cpp, transfer_offer.cpp) ----------

/// Constants of the record utilities.
pub(crate) mod record_utils_consts {
    /// "E:\dev\CM3\cm3 00-01\cm3\code\record_utils.cpp" (the error box).
    pub(crate) const RECORD_UTILS_CPP: usize = 0xa7a088;
    /// A match record's home goals (+0x19) and away goals (+0x1a), signed bytes.
    pub(crate) const HOME_GOALS: usize = 0x19;
    pub(crate) const AWAY_GOALS: usize = 0x1a;
    /// The duration texts (translated by `FN_LANGLIB_SPRINTF`): approximate, then exact.
    pub(crate) const ONE_DAY: usize = 0xa7a1d0; // "one day"
    pub(crate) const A_FEW_DAYS: usize = 0xa7a1c4; // "a few days"
    pub(crate) const ABOUT_A_WEEK: usize = 0xa7a1b4; // "about a week"
    pub(crate) const ABOUT_TEN_DAYS: usize = 0xa7a1a4; // "about ten days"
    pub(crate) const ABOUT_N_WEEKS: usize = 0xa7a188; // "about <%d - number> weeks"
    pub(crate) const ABOUT_A_MONTH: usize = 0xa7a178; // "about a month"
    pub(crate) const ABOUT_N_MONTHS: usize = 0xa7a15c; // "about <%d - number> months"
    pub(crate) const ABOUT_A_YEAR: usize = 0xa7a14c; // "about a year"
    pub(crate) const ABOUT_18_MONTHS: usize = 0xa7a13c; // "about 18 months"
    pub(crate) const ABOUT_2_YEARS: usize = 0xa7a12c; // "about 2 years"
    pub(crate) const ABOUT_N_YEARS: usize = 0xa7a110; // "about <%d - number> years"
    pub(crate) const EXACT_1_DAY: usize = 0xa7a108; // "1 day"
    pub(crate) const EXACT_N_DAYS: usize = 0xa7a0f4; // "<%d - number> days"
    pub(crate) const EXACT_1_WEEK: usize = 0xa7a0ec; // "1 week"
    pub(crate) const EXACT_N_WEEKS: usize = 0xa7a0d8; // "<%d - number> weeks"
    pub(crate) const EXACT_1_MONTH: usize = 0xa7a0d0; // "1 month"
    pub(crate) const EXACT_N_MONTHS: usize = 0xa7a0b8; // "<%d - number> months"
    /// "E:\dev\CM3\cm3 00-01\cm3\code\transfer_offer.cpp" (the error box).
    pub(crate) const TRANSFER_OFFER_CPP: usize = 0xa9e318;
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
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%)(a, b);
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
    let o = original_fn::<unsafe extern "C" fn(i32, usize, i32, u32) -> u32>(%%I1%%);
    let regions = [(out, size.clamp(0, 256) as usize)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(days, out, size, approximate), || format_duration_text(days, out, size, approximate));
    SCOREBOARD.record(equal && r == n, || format!("format_duration_text({}, approximate {}) original={} {:x?} rust={} {:x?}", days, approximate, r, mo, n, mr));
    r
}

/// Resets an offer record for a loan of `staff` to `club`.
///
/// `TRANSFER_OFFER::init_loan_offer(staff, club)`. Without the person or the club, the game's error box (line 0x485)
/// and nothing more. Otherwise: not known to the player (`offer::PLAYER_AWARE` 0), no exchange list, job 11
/// (`offer::JOB_TYPE`), the buyer's id (`offer::BUYER_CLUB_ID`) and the person's (`offer::TARGET_STAFF_ID`), no fee,
/// the clauses empty with 10 games (`offer::APPEARANCE_GAMES`, `offer::INTERNATIONAL_GAMES`), serial and negotiation
/// −1, no sell-on or instalments, status 0, no terms, response round or pre-contract, a loan (`offer::OFFER_TYPE` 1),
/// the completion timing's low nibble cleared (+0 and the high nibble kept).
/// Original: `TRANSFER_OFFER::init_loan_offer` (0x8e7810, transfer_offer.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn init_loan_offer(this: usize, staff: usize, club: usize) {
    if staff == 0 || club == 0 {
        show_error_at(record_utils_consts::TRANSFER_OFFER_CPP, 0x485);
        return;
    }
    covered!("mwg_8 loan offer");
    write_u8(this + offer::PLAYER_AWARE, 0);
    write_u32(this, 0);
    write_u8(this + offer::JOB_TYPE, 0xb);
    write_usize(this + offer::EXCHANGE_PLAYERS, 0);
    write_u32(this + offer::BUYER_CLUB_ID, read_u32(club));
    write_u32(this + offer::FEE, 0);
    write_u8(this + offer::APPEARANCE_GAMES, 10); // mwg8: the appearance clause's games
    write_u8(this + offer::INTERNATIONAL_GAMES, 10);
    write_u32(this + offer::APPEARANCE_FEE, 0);
    write_i32(this + offer::NEGOTIATION_INDEX, -1);
    write_i32(this + offer::SERIAL, -1);
    write_u32(this + offer::INTERNATIONAL_FEE, 0);
    write_u8(this + offer::SELL_ON_PERCENT, 0);
    write_u8(this + offer::INSTALMENTS, 0);
    write_u8(this + offer::STATUS, 0);
    write_u8(this + offer::TERMS_STATE, 0);
    write_u8(this + offer::RESPONSE_ROUND, 0);
    write_u8(this + offer::PRE_CONTRACT, 0);
    write_u32(this + offer::TARGET_STAFF_ID, read_u32(staff));
    write_u8(this + offer::OFFER_TYPE, 1);
    write_u8(this + offer::COMPLETION_TIMING, read_u8(this + offer::COMPLETION_TIMING) & 0xf0);
}

/// Comparer for `init_loan_offer`: side by side over the offer's 0x41 bytes.
pub(crate) unsafe extern "thiscall" fn compare_init_loan_offer(this: usize, staff: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%);
    let regions = [(this, offer::INSTALMENTS + 1)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, staff, club), || init_loan_offer(this, staff, club));
    SCOREBOARD.record(equal, || format!("init_loan_offer(staff {:x}, club {:x}) original={:x?} rust={:x?}", staff, club, mo, mr));
}
