// ---------- rx08_9: the club records screen texts: the period and record type labels, one record as text, and the Cash helpers (club_records.cpp) ----------

/// Writes the label `template` (from the language, `FN_LANGLIB_SPRINTF`) of entry `which` (from 1, signed byte) of
/// `labels` into `buf`: a missing buffer or one shorter than 50 bytes shows the club_records.cpp box at `short_row` and
/// answers 0; otherwise the buffer is emptied, an unknown entry shows the box at `bad_row` and answers 0, a known one is
/// written and answers 1.
unsafe fn club_records_write_label(labels: &[usize], which: u32, buf: usize, size: i32, short_row: u32, bad_row: u32) -> u32 {
    if buf == 0 || size < 0x32 {
        show_error_at(club_records_consts::CLUB_RECORDS_CPP, short_row);
        return 0;
    }
    write_u8(buf, 0);
    let k = (which as u8 as i8 as i32).wrapping_sub(1);
    match labels.get(k as usize).filter(|_| k >= 0) {
        Some(&template) => {
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            sprintf(buf, template);
            1
        }
        None => {
            show_error_at(club_records_consts::CLUB_RECORDS_CPP, bad_row);
            0
        }
    }
}

/// The two record periods: 1 "Modern Day" (all time), 2 "This Season".
const CLUB_RECORDS_PERIOD_LABELS: [usize; 2] = [0x98b728, 0x98b734];

/// rule: the label of a club records period: 1 "Modern Day" (the all-time records), 2 "This Season"
///
/// `club_records_write_label` with `CLUB_RECORDS_PERIOD_LABELS`, the boxes of lines 0x766 (no or short buffer) and 0x780
/// (unknown period). `this` is not read.
/// Original: `club_records_get_period_label` (0x448fa0, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn club_records_get_period_label(_this: usize, period: u32, buf: usize, size: i32) -> u32 {
    covered!("rx08_9 period label");
    club_records_write_label(&CLUB_RECORDS_PERIOD_LABELS, period, buf, size, 0x766, 0x780)
}

/// The 32 club record types, in order: the league positions, the attendances and gate, the wins, defeats and scoring
/// games, the runs, the scorers, assists, ratings, man of the match and discipline, the club appearances and goals, the
/// youngest, oldest and most capped players, the transfer fees, spending and income, and the fans' player of the year.
const CLUB_RECORDS_TYPE_LABELS: [usize; 32] = [0x98b9f4, 0x98b9dc, 0x98b9c8, 0x98b9b0, 0x98b99c, 0x98b988, 0x98b97c, 0x98b968, 0x98b958,
    0x98b940, 0x98b924, 0x98b90c, 0x98b8f4, 0x98b8d8, 0x98b8bc, 0x98b8ac, 0x98b894, 0x98b880, 0x98b870, 0x98b858, 0x98b844, 0x98b830,
    0x98b814, 0x98b7f8, 0x98b7e8, 0x98b7d8, 0x98b7c4, 0x98b7a8, 0x98b788, 0x98b770, 0x98b758, 0x98b740];

/// rule: the label of each of the 32 club record types: 1 "Highest League Position", 2 "Lowest League Position", 3
/// "Highest Attendance", 4 "Highest Gate Receipts", 5 "Lowest Attendance", 6 "Average Attendance", 7 "Biggest Win", 8
/// "Biggest League Win", 9 "Biggest Defeat", 10 "Highest Scoring Game", 11 "Highest Scoring League Game", 12 "Most
/// Games Won in Row", 13 "Most Games Lost in Row", 14 "Most Games Without Losing", 15 "Most Games Without Winning", 16
/// "Top Goalscorer", 17 "Top League Goalscorer", 18 "Most Goals in Match", 19 "Most Assists", 20 "Highest Average
/// Rating", 21 "Most Man of Match", 22 "Worst Discipline", 23 "Most League Apps for Club", 24 "Most League Goals for
/// Club", 25 "Youngest Player", 26 "Oldest Player", 27 "Most Capped Player", 28 "Highest Transfer Fee Paid", 29
/// "Highest Transfer Fee Received", 30 "Total Transfer Spending", 31 "Total Transfer Income", 32 "Fans Player Of The
/// Year"
///
/// `club_records_write_label` with `CLUB_RECORDS_TYPE_LABELS`, the boxes of lines 0x793 (no or short buffer) and 0x87f
/// (unknown type). `this` is not read.
/// Original: `club_records_get_record_type_label` (0x4490b0, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn club_records_get_record_type_label(_this: usize, kind: u32, buf: usize, size: i32) -> u32 {
    covered!("rx08_9 record type label");
    club_records_write_label(&CLUB_RECORDS_TYPE_LABELS, kind, buf, size, 0x793, 0x87f) // rx08_9: the record type labels
}

/// rule: one record of a club as text: the club's all-time records (period 1) or this season's (otherwise), the
/// season's best average rating worked out first when it is still unset; "-" until then
///
/// `this.get_record_text(club, period, type, text)`: no Text: the club_records.cpp box of line 0x95f and 0. The Text
/// cleared (`exe::text_clear`) and set to "-" (`exe::text_set_string`); no club: the box of line 0x96b and 0. Period
/// 1: the all-time records (`exe::club_records_get_all_time_record`(club, 0)); otherwise the season's
/// (`exe::club_records_get_season_record`(club, 0)), and for type 20 (the best average rating) with records whose
/// `club_record::SEASON_RATING` is still −1.0 (`rdata::F64_NEG_1`), the season rating record worked out
/// (`update_season_rating_record`). Then club_records_format_record_text(club, records, type, text)
/// (`exe::club_records_format_record_text`), whose result it passes on.
/// Original: `club_records_get_record_text` (0x4496d0, club_records.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn club_records_get_record_text(this: usize, club: usize, period: u32, kind: u32, text: usize) -> u32 {
    if text == 0 {
        show_error_at(club_records_consts::CLUB_RECORDS_CPP, 0x95f);
        return 0;
    }
    exe::text_clear(text as *mut usize);
    exe::text_set_string(text, 0x98ba0c as *const u8); // "-"
    if club == 0 {
        show_error_at(club_records_consts::CLUB_RECORDS_CPP, 0x96b);
        return 0;
    }
    covered!("rx08_9 record text");
    let records = if period as u8 == 1 {
        exe::club_records_get_all_time_record(this, club, 0)
    } else {
        let records = exe::club_records_get_season_record(this, club, 0);
        if kind as u8 == 0x14 && records != 0 && read_f32(records + club_record::SEASON_RATING) as f64 == read_f64(rdata::F64_NEG_1) { // rx08_9: the season rating still unset
            update_season_rating_record(this, club);
        }
        records
    };
    exe::club_records_format_record_text(this, club, records, kind, text as *mut usize)
}

/// rule: a cash amount for display is the money value itself
///
/// `Cash::Cash(value)`: the dword `value` stored in the object, which is returned.
/// Original: `Cash::Cash` (0x450a90, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_cash(this: usize, value: i32) -> usize {
    write_i32(this, value); // rx08_9: the amount
    this
}

/// rule: a cash amount is written as money text in the display currency, rounded to the step
///
/// `this.format_display_amount(buf, flags, step)`: the amount converted to the display currency
/// (`exe::cash_get_display_amount`(this, step), a 64-bit value), then written (`exe::cash_format_money`(buf, flags,
/// value, step), cdecl), whose result it passes on.
/// Original: `Cash::format_display_amount` (0x450aa0, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn cash_format_display_amount(this: usize, buf: usize, flags: u32, step: u32) -> u32 {
    let value = exe::cash_get_display_amount(this as *const u32, step);
    exe::cash_format_money(buf, flags, value, step) as u32
}

/// Side by side on the first 50 bytes of a label buffer and the result (a missing or short buffer: only the original,
/// the box).
unsafe fn compare_club_records_label(name: &str, buf: usize, size: i32, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    if buf == 0 || size < 0x32 {
        return original();
    }
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&[(buf, 0x32)], original, copy);
    SCOREBOARD.record(same && a == b, || format!("{}: original {} rust {}, {:?}", name, a, b, first_differences(&mo, &mr)));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_club_records_get_period_label(this: usize, period: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> u32>(%%I0%%);
    compare_club_records_label(&format!("club_records_get_period_label({})", period as u8 as i8), buf, size, || o(this, period, buf, size), || club_records_get_period_label(this, period, buf, size))
}

pub(crate) unsafe extern "thiscall" fn compare_club_records_get_record_type_label(this: usize, kind: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> u32>(%%I1%%);
    compare_club_records_label(&format!("club_records_get_record_type_label({})", kind as u8 as i8), buf, size, || o(this, kind, buf, size), || club_records_get_record_type_label(this, kind, buf, size))
}

/// Comparer for `club_records_get_record_text`: side by side into two Texts (`compare_text_writer`); the season rating
/// record, when it is worked out, is done by the original first (the copy then finds it set).
pub(crate) unsafe extern "thiscall" fn compare_club_records_get_record_text(this: usize, club: usize, period: u32, kind: u32, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) -> u32>(%%I2%%);
    if text == 0 || club == 0 {
        return o(this, club, period, kind, text);
    }
    let mut r = 0;
    compare_text_writer(text, || r = o(this, club, period, kind, text), |t| { club_records_get_record_text(this, club, period, kind, t); },
        || format!("club_records_get_record_text(club {}, period {}, type {})", read_i32(club), period as u8, kind as u8));
    r
}

/// Comparer for `construct_cash`: side by side on the stored dword and the result.
pub(crate) unsafe extern "thiscall" fn compare_construct_cash(this: usize, value: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I3%%);
    let (a, b, same, _, _) = crate::comparison::side_by_side_light(&[(this, 4)], || o(this, value), || construct_cash(this, value));
    SCOREBOARD.record(same && a == b, || format!("construct_cash({:#x}, {}) original {:#x} rust {:#x}", this, value, a, b));
    a
}

/// Comparer for `cash_format_display_amount`: side by side on the first 50 bytes of the buffer and the result.
pub(crate) unsafe extern "thiscall" fn compare_cash_format_display_amount(this: usize, buf: usize, flags: u32, step: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I4%%);
    if buf == 0 {
        return o(this, buf, flags, step);
    }
    compare_club_records_label(&format!("cash_format_display_amount({}, flags {:#x}, step {})", read_i32(this), flags, step), buf, 0x32,
        || o(this, buf, flags, step), || cash_format_display_amount(this, buf, flags, step))
}
