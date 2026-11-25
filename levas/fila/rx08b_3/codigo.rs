// ---------- rx08b_3: one field of a player's statistics as text, from a season record or from the career totals (player_stats.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\player_stats.cpp" (the error boxes of the two field formatters).
const STATS_FIELD_CPP: usize = 0xa78310;

/// The formats of the field formatters: "%d", "%d (%d)", "%d / %d", "%d (%d) / %d", "%4.2f", "%4.2f / %d", "%3.1f",
/// "%3.1f / %d", "%d%%" and "%d%% / %d"; the placeholders "-" and "----" (the average rating, field 17).
const STATS_INT: usize = 0x98806c;
const STATS_INT_SUBS: usize = 0x99cc58;
const STATS_INT_APPS: usize = 0xa784d0;
const STATS_INT_SUBS_APPS: usize = 0xa784c0;
const STATS_RATING: usize = 0x98fe94;
const STATS_RATING_APPS: usize = 0xa784b4;
const STATS_AVERAGE: usize = 0x99ccc8;
const STATS_AVERAGE_APPS: usize = 0xa7849c;
const STATS_PERCENT: usize = 0x98e8d4;
const STATS_PERCENT_APPS: usize = 0xa784a8;
const STATS_DASH: usize = 0x98ba0c;
const STATS_NO_RATING: usize = 0x99cc4c;

/// The average rating field.
const STATS_FIELD_RATING: u8 = 0x11;

/// The C runtime's sprintf (the formats take ints and doubles).
unsafe fn stats_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// Writes `value` with the appearances when `apps` ("%d / %d"), alone otherwise ("%d").
unsafe fn stats_write_count(buf: usize, value: i32, appearances: i32, apps: u32) {
    if apps != 0 {
        stats_sprintf()(buf, STATS_INT_APPS, value, appearances);
    } else {
        stats_sprintf()(buf, STATS_INT, value);
    }
}

/// Writes an average with the appearances when `apps` (format `with_apps`), alone otherwise (format `alone`).
unsafe fn stats_write_average(buf: usize, average: f64, appearances: i32, apps: u32, (alone, with_apps): (usize, usize)) {
    if apps != 0 {
        stats_sprintf()(buf, with_apps, average, appearances);
    } else {
        stats_sprintf()(buf, alone, average);
    }
}

/// Writes the percentage `part × 100 / whole` (`rdata::F32_100`, truncated) with the appearances when `apps`.
unsafe fn stats_write_percent(buf: usize, part: i32, whole: i32, appearances: i32, apps: u32) {
    let percent = ftol(part as f64 * read_f32(rdata::F32_100) as f64 / whole as f64);
    if apps != 0 {
        stats_sprintf()(buf, STATS_PERCENT_APPS, percent, appearances);
    } else {
        stats_sprintf()(buf, STATS_PERCENT, percent);
    }
}

/// The checks both field formatters start with: no buffer or one shorter than 51 bytes shows the box at `short_row`
/// and answers false; otherwise the buffer gets the placeholder ("----" for the average rating, "-" otherwise).
unsafe fn stats_field_start(field: u32, buf: usize, size: i32, short_row: u32) -> bool {
    if buf == 0 || size < 0x33 {
        show_error_at(STATS_FIELD_CPP, short_row);
        return false;
    }
    news_strcpy(buf, if field as u8 == STATS_FIELD_RATING { STATS_NO_RATING } else { STATS_DASH });
    true
}

/// rule: one field of a player's season statistics record as text: the count, with the appearances after a slash when
/// asked; appearances and penalties can show a part in brackets; the average rating with two decimals, the per-game
/// figures with one, the passing and shooting success as percentages; nothing written while the player has no
/// appearances (unless asked for a zero)
///
/// `this.format_record_field(record, field, buf, size, subs, apps, zero)` over the 32-byte season record (unsigned bytes:
/// +0 appearances, +1 of them as a substitute, +2..+0xd the counts; signed words: +0xe the rating sum, +0x10/+0x12 and
/// +0x18/+0x1a attempts and successes, +0x14 and +0x16 per-game sums). `stats_field_start` (box 0x7ff). No record: the
/// box of line 0x811 and 0. By field (signed byte):
/// - 1: with substitute appearances and `subs`, "%d (%d)" (appearances − substitute ones, substitute ones); otherwise the
///   appearances ("%d"), nothing when none and not `zero`;
/// - 2, 3, 5..12 (the counts +2, +3, +6..+0xd): nothing with no appearances and not `zero`; the count, "%d / %d" with
///   the appearances when `apps`;
/// - 4 (+4, with the part +5): with +4 and `subs`, "%d (%d) / %d" (+4, +5, appearances) or "%d (%d)"; otherwise as
///   the counts with +4;
/// - 13 and 16 (attempts +0x10/+0x18, successes +0x12/+0x1a): with attempts above 0, the percentage
///   (`stats_write_percent`);
/// - 14 and 15 (+0x14, +0x16): with appearances, the sum over the appearances, "%3.1f" or "%3.1f / %d";
/// - 17: with appearances, the rating sum over them, "%4.2f" or "%4.2f / %d"; without, "----";
/// - any other: the box of line 0x91b and 0.
///
/// Every written field (and every one left as the placeholder) answers 1. `this` is not read.
/// Original: `PlayerStats::format_record_field` (0x7c2750, player_stats.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn format_record_field(_this: usize, record: usize, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    if !stats_field_start(field, buf, size, 0x7ff) {
        return 0;
    }
    if record == 0 {
        show_error_at(STATS_FIELD_CPP, 0x811);
        return 0;
    }
    covered!("rx08b_3 season stat field");
    let byte = |k: usize| read_u8(record + k) as i32;
    let word = |k: usize| read_i16(record + k) as i32;
    let appearances = byte(0);
    let count = |k: usize| {
        if appearances != 0 || zero != 0 {
            stats_write_count(buf, byte(k), appearances, apps);
        }
    };
    match field as u8 as i8 {
        1 => {
            if byte(1) != 0 && subs != 0 {
                stats_sprintf()(buf, STATS_INT_SUBS, appearances - byte(1), byte(1));
            } else if appearances != 0 || zero != 0 {
                stats_sprintf()(buf, STATS_INT, appearances);
            }
        }
        2 => count(2),
        3 => count(3),
        4 => {
            if byte(4) != 0 && subs != 0 {
                if apps != 0 {
                    stats_sprintf()(buf, STATS_INT_SUBS_APPS, byte(4), byte(5), appearances);
                } else {
                    stats_sprintf()(buf, STATS_INT_SUBS, byte(4), byte(5));
                }
            } else {
                count(4);
            }
        }
        k @ 5..=12 => count(k as usize + 1), // rx08b_3: the counts +6..+0xd
        13 => {
            if word(0x10) > 0 {
                stats_write_percent(buf, word(0x12), word(0x10), appearances, apps);
            }
        }
        16 => {
            if word(0x18) > 0 {
                stats_write_percent(buf, word(0x1a), word(0x18), appearances, apps);
            }
        }
        14 | 15 => {
            if appearances != 0 {
                let at = if field as u8 == 14 { 0x14 } else { 0x16 };
                stats_write_average(buf, word(at) as f64 / appearances as f64, appearances, apps, (STATS_AVERAGE, STATS_AVERAGE_APPS));
            }
        }
        0x11 => {
            if appearances == 0 {
                news_strcpy(buf, STATS_NO_RATING);
            } else {
                stats_write_average(buf, word(0xe) as f64 / appearances as f64, appearances, apps, (STATS_RATING, STATS_RATING_APPS));
            }
        }
        _ => {
            show_error_at(STATS_FIELD_CPP, 0x91b);
            return 0;
        }
    }
    1
}

/// rule: one field of a player's career totals as text, in the same layout as the season record (see
/// `format_record_field`), with these differences: the per-game figures are per 90 minutes and need 150 minutes; the
/// passing success needs 20 attempts; the shooting success needs 25 shots, or 5 to 24 shots at no more than 45
/// minutes a shot; without the brackets, the penalties column shows the second total
///
/// `this.format_totals_field(totals, field, buf, size, subs, apps, zero)` over the totals block (signed dwords: +0
/// appearances, +4 of them as a substitute, +4 × k the count of field k, +0x38 the rating sum, +0x48 the minutes;
/// signed words: +0x3c/+0x3e and +0x44/+0x46 attempts and successes, +0x40 and +0x42 the per-game sums).
/// `stats_field_start` (box 0x932). No totals: the box of line 0x944 and 0. By field:
/// - 1: with substitute appearances and `subs`, "%d (%d)" (appearances − substitute ones, substitute ones); otherwise the
///   appearances, nothing when none and not `zero`;
/// - 2, 3, 5..12 (the dwords +8, +0xc, +0x18..+0x34): as in the season record;
/// - 4: with +0x10 and `subs`, "%d (%d) / %d" (+0x10, +0x14, appearances) or "%d (%d)"; otherwise the count is the
///   dword +0x14, the part in brackets (the season record shows its first figure there; DEFEITOS_DO_ORIGINAL);
/// - 13 (+0x3c attempts, +0x3e successes): above 19 attempts, the percentage;
/// - 14 and 15 (+0x40, +0x42): with 150 minutes or more, the sum × 90 (`rdata::F32_90`) over the minutes;
/// - 16 (+0x44 shots, +0x46 on target): above 24 shots, or 5 to 24 shots with at most 45 minutes a shot (signed
///   division; with more, it also needs more than 540 minutes and more than 10 shots): the percentage;
/// - 17: with appearances, the rating sum over them, "%4.2f" or "%4.2f / %d"; without, "----";
/// - any other: the box of line 0xa50 and 0.
/// Original: `PlayerStats::format_totals_field` (0x7c3060, player_stats.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn format_totals_field(_this: usize, totals: usize, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    if !stats_field_start(field, buf, size, 0x932) {
        return 0;
    }
    if totals == 0 {
        show_error_at(STATS_FIELD_CPP, 0x944);
        return 0;
    }
    covered!("rx08b_3 career stat field");
    let int = |k: usize| read_i32(totals + k);
    let word = |k: usize| read_i16(totals + k) as i32;
    let appearances = int(0);
    let minutes = int(0x48);
    let count = |k: usize| {
        if appearances != 0 || zero != 0 {
            stats_write_count(buf, int(k), appearances, apps);
        }
    };
    let per_90 = |k: usize| {
        if minutes >= 0x96 {
            let average = word(k) as f64 * read_f32(rdata::F32_90) as f64 / minutes as f64;
            stats_write_average(buf, average, appearances, apps, (STATS_AVERAGE, STATS_AVERAGE_APPS));
        }
    };
    match field as u8 as i8 {
        1 => {
            if int(4) != 0 && subs != 0 {
                stats_sprintf()(buf, STATS_INT_SUBS, appearances.wrapping_sub(int(4)), int(4));
            } else if appearances != 0 || zero != 0 {
                stats_sprintf()(buf, STATS_INT, appearances);
            }
        }
        2 => count(8),
        3 => count(0xc),
        4 => {
            if int(0x10) != 0 && subs != 0 {
                if apps != 0 {
                    stats_sprintf()(buf, STATS_INT_SUBS_APPS, int(0x10), int(0x14), appearances);
                } else {
                    stats_sprintf()(buf, STATS_INT_SUBS, int(0x10), int(0x14));
                }
            } else {
                count(0x14); // rx08b_3: the original's second total here (the season record shows the first)
            }
        }
        k @ 5..=12 => count(4 * (k as usize + 1)),
        13 => {
            if word(0x3c) > 0x13 {
                stats_write_percent(buf, word(0x3e), word(0x3c), appearances, apps);
            }
        }
        14 => per_90(0x40),
        15 => per_90(0x42),
        16 => {
            let shots = word(0x44);
            let enough = shots > 0x18 || (shots > 4 && minutes / shots <= 0x2d) || (minutes > 0x21c && shots > 0xa); // rx08b_3: the shooting threshold
            if enough {
                stats_write_percent(buf, word(0x46), shots, appearances, apps);
            }
        }
        0x11 => {
            if appearances == 0 {
                news_strcpy(buf, STATS_NO_RATING);
            } else {
                stats_write_average(buf, int(0x38) as f64 / appearances as f64, appearances, apps, (STATS_RATING, STATS_RATING_APPS));
            }
        }
        _ => {
            show_error_at(STATS_FIELD_CPP, 0xa50);
            return 0;
        }
    }
    1
}

/// Side by side on the first 51 bytes of the buffer (the formatters only write when it is at least that long) and on
/// the answer; a missing or short buffer, or a missing record, runs only the original (the box).
unsafe fn compare_stats_field(name: &str, data: usize, buf: usize, size: i32, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    if buf == 0 || size < 0x33 || data == 0 {
        return original();
    }
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&[(buf, 0x33)], original, copy);
    SCOREBOARD.record(same && a == b, || format!("{}: original {} rust {} {:?}", name, a, b, first_differences(&mo, &mr)));
    a
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_format_record_field(this: usize, record: usize, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, i32, u32, u32, u32) -> u32>(%%I0%%);
    compare_stats_field(&format!("PlayerStats::format_record_field(field {}, subs {}, apps {}, zero {})", field as u8, subs, apps, zero), record, buf, size,
        || o(this, record, field, buf, size, subs, apps, zero), || format_record_field(this, record, field, buf, size, subs, apps, zero))
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_format_totals_field(this: usize, totals: usize, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, i32, u32, u32, u32) -> u32>(%%I1%%);
    compare_stats_field(&format!("PlayerStats::format_totals_field(field {}, subs {}, apps {}, zero {})", field as u8, subs, apps, zero), totals, buf, size,
        || o(this, totals, field, buf, size, subs, apps, zero), || format_totals_field(this, totals, field, buf, size, subs, apps, zero))
}
