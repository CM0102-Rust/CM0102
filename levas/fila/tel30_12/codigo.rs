// ---------- batch %%LEVA%%: the competition statistics texts: the team and player values and the record titles and texts (comp_screens.cpp) ----------

/// The default texts of a statistic ("----", "---", "0") and the formats "%ld", "%d (%d)", "%4.2f" and "%3.1f" ("%d"
/// and "%d%%" are `FORMAT_D` / `FORMAT_PCT`).
const COMP_STATS_DASHES_4: usize = 0x99cc4c;
const COMP_STATS_DASHES_3: usize = 0x98f858;
const COMP_STATS_ZERO_TEXT: usize = 0x99cc54;
const COMP_STATS_FORMAT_LD: usize = 0x98c11c;
const COMP_STATS_FORMAT_WITH_DETAIL: usize = 0x99cc58;
const COMP_STATS_FORMAT_2_DECIMALS: usize = 0x98fe94;
const COMP_STATS_FORMAT_1_DECIMAL: usize = 0x99ccc8;

/// The game's sprintf (0x944c9f) on a statistic buffer.
unsafe fn comp_stats_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// A team's form, the five result bytes from `last` down to `last - 4` (3 won, 2 score draw, 1 drawn, -3 lost; any
/// other byte adds nothing), as letters appended to the emptied `out`. The four letters are translated first (langlib:
/// "W<..won>", "X<..score draw>", "D<..drawn>", "L<..lost>"), each into its 2-byte slot of the original's frame (L, W,
/// D, X from the frame's +0x10), in the original's order W, X, D, L.
unsafe fn comp_stats_form_text(out: usize, last: usize) {
    // ponytail: 16 bytes where the original has 8 (the counter follows), so a longer translation stays in the slots
    let mut marks = [0u8; 16];
    let m = marks.as_mut_ptr() as usize;
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(m + 2, 0x99ccb0usize, EMPTY_TEXT);
    langlib(m + 6, 0x99cc90usize, EMPTY_TEXT);
    langlib(m + 4, 0x99cc78usize, EMPTY_TEXT);
    langlib(m, 0x99cc60usize, EMPTY_TEXT);
    write_u8(out, 0);
    for k in 0..5 {
        let slot = match read_u8(last - k) {
            3 => 2,
            2 => 6,
            1 => 4,
            0xfd => 0,
            _ => continue,
        };
        append_text(out as *mut u8, marks.as_ptr().add(slot));
    }
}

/// The text of team statistic `code` of a team entry into `out`; returns 1, or 0 with the error box (comp_stats.CPP)
/// for a null buffer or one under 0x33 bytes (line 0xac4), a null entry (line 0xacf, after the default text) and an
/// unknown code (line 0xb96, after the default text).
///
/// The buffer first takes the default text: "----" for codes 2..5, "0" for the rest. Then 1 the games (byte +4); 2 the
/// average attendance (+6 / the byte +5, "%ld"; with no game the "----" stays); 3 / 4 / 5 the form of the last five
/// results (`comp_stats_form_text` from +0xe / +0x13 / +0x18 down); 6..9 the runs (+0x19, +0x2d, +0x41, +0x55); 10 / 11
/// the bytes +0x69 / +0x6a; 12 the byte +0x6b, with the byte +0x6c in brackets ("%d (%d)") when both it and
/// `with_detail` are set; 13..17 the bytes +0x6d..+0x71. The object in ecx is not read.
/// Original: `comp_stats_format_team_stat` (0x4a39c0, comp_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_team_stat(_this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    if out == 0 || size < 0x33 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xac4);
        return 0;
    }
    copy_text(out as *mut u8, (if (2..=5).contains(&code) { COMP_STATS_DASHES_4 } else { COMP_STATS_ZERO_TEXT }) as *const u8);
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xacf);
        return 0;
    }
    let sprintf = comp_stats_sprintf();
    let byte = |at: usize| read_u8(entry + at) as i32;
    match code {
        1 => {
            sprintf(out, FORMAT_D, byte(4));
        }
        2 => {
            let games = byte(5);
            if games != 0 {
                sprintf(out, COMP_STATS_FORMAT_LD, read_i32(entry + 6) / games);
            }
        }
        3..=5 => comp_stats_form_text(out, entry + [0xe, 0x13, 0x18][code as usize - 3]),
        6..=9 => {
            sprintf(out, FORMAT_D, read_i32(entry + 0x19 + (code as usize - 6) * 0x14));
        }
        10 | 11 => {
            sprintf(out, FORMAT_D, byte(0x69 + (code as usize - 10)));
        }
        12 => {
            let x = byte(0x6b);
            if x != 0 && with_detail != 0 {
                covered!("tel30_12 team stat with detail");
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, x, byte(0x6c));
            } else {
                sprintf(out, FORMAT_D, x);
            }
        }
        13..=17 => {
            sprintf(out, FORMAT_D, byte(0x6d + (code as usize - 13)));
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xb96);
            return 0;
        }
    }
    1
}

/// The text of player statistic `code` of a player entry into `out`; returns 1, or 0 with the error box
/// (comp_stats.CPP) for a null buffer or one under 0x33 bytes (line 0xbe5), a null entry (line 0xbfd, after the default
/// text) and an unknown code (line 0xcc7, after the default text).
///
/// The default text: "---" for 13 and 16, "----" for 14, 15 and 17, "0" for the rest; a rate short of its condition
/// leaves it. 1 the appearances (byte +0xc), as the starts and the substitute appearances in brackets ("%d (%d)": +0xc
/// minus +0xd, +0xd) when +0xd and `with_detail` are set; 2 / 3 the bytes +0xe / +0xf; 4 the byte +0x10, with +0x11 in
/// brackets when +0x10 and `with_detail` are set; 5..12 the bytes +0x12..+0x19. The rates need the minutes (+0x29) to
/// reach 35 × the club's games (`comp_stats_club_games`, x87: the games × 35.0 compared with the minutes): 13 the pass
/// completion (+0x1f × 100 / +0x1d passes, truncated, "%d%%"; more than 20 passes and, up to 500, more than a quarter
/// of the minutes); 14 / 15 the tackles / dribbles per game (+0x21 / +0x23 over the appearances, "%3.1f"; at least
/// 0x96 minutes); 16 the shots on target (+0x27 × 100 / +0x25 shots, truncated, "%d%%"; more than 24 shots, or more
/// than 4 at under 46 minutes each, or more than 10 with more than 0x21c minutes); 17 the average rating (+0x1a / the
/// appearances, "%4.2f"; the club's games rounded to an f32, × 0.5, not above the appearances (player statistic 1),
/// and at least one appearance).
/// Original: `Comp_Stats::format_player_stat` (0x4a3ee0, comp_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_player_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    if out == 0 || size < 0x33 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xbe5);
        return 0;
    }
    let default = match code {
        13 | 16 => COMP_STATS_DASHES_3,
        14 | 15 | 17 => COMP_STATS_DASHES_4,
        _ => COMP_STATS_ZERO_TEXT,
    };
    copy_text(out as *mut u8, default as *const u8);
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xbfd);
        return 0;
    }
    let sprintf = comp_stats_sprintf();
    let byte = |at: usize| read_u8(entry + at) as i32;
    let k35 = read_f32(COMP_STATS_F32_35) as f64;
    let hundred = read_f32(rdata::F32_100) as f64;
    match code {
        1 => {
            let subs = byte(0xd);
            if subs != 0 && with_detail != 0 {
                covered!("tel30_12 appearances with substitutes");
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, byte(0xc) - subs, subs);
            } else {
                sprintf(out, FORMAT_D, byte(0xc));
            }
        }
        2 | 3 => {
            sprintf(out, FORMAT_D, byte(0xe + (code as usize - 2)));
        }
        4 => {
            let x = byte(0x10);
            if x != 0 && with_detail != 0 {
                sprintf(out, COMP_STATS_FORMAT_WITH_DETAIL, x, byte(0x11));
            } else {
                sprintf(out, FORMAT_D, x);
            }
        }
        5..=12 => {
            sprintf(out, FORMAT_D, byte(0x12 + (code as usize - 5)));
        }
        13 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return 1;
            }
            let passes = read_i16(entry + 0x1d);
            if passes <= 0x14 || (passes <= 0x1f4 && passes as i32 <= minutes / 4) {
                return 1;
            }
            sprintf(out, FORMAT_PCT, ftol(read_i16(entry + 0x1f) as f64 * hundred / passes as f64));
        }
        14 | 15 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 || minutes < 0x96 {
                return 1;
            }
            let per_game = read_i16(entry + if code == 14 { 0x21 } else { 0x23 }) as f64 / byte(0xc) as f64;
            sprintf(out, COMP_STATS_FORMAT_1_DECIMAL, per_game);
        }
        16 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return 1;
            }
            let shots = read_i16(entry + 0x25);
            let enough = shots > 0x18 || (shots > 4 && minutes / shots as i32 <= 0x2d) || (minutes > 0x21c && shots > 10);
            if !enough {
                return 1;
            }
            sprintf(out, FORMAT_PCT, ftol(read_i16(entry + 0x27) as f64 * hundred / shots as f64));
        }
        17 => {
            let games = comp_stats_club_games(this, entry) as f32 as f64;
            let played = comp_stats_get_player_entry_value(this, entry, 1);
            let apps = byte(0xc);
            if games * read_f64(rdata::F64_0_5) > played || apps == 0 {
                return 1;
            }
            sprintf(out, COMP_STATS_FORMAT_2_DECIMALS, read_i16(entry + 0x1a) as f64 / apps as f64);
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xcc7);
            return 0;
        }
    }
    1
}

/// The titles of the record types 1..=22 ("Most Times Winner" .. "Oldest Player").
const COMP_STATS_RECORD_TITLES: [usize; 22] = [0x99cd50, 0x99cd3c, 0x99cd2c, 0x99cd18, 0x99cd00, 0x98b9c8, 0x98b99c, 0x99cce4,
    0x98b97c, 0x98b940, 0x98b90c, 0x98b8f4, 0x98b8d8, 0x98b8bc, 0x98b8ac, 0x98b880, 0x98b870, 0x98b858, 0x98b844, 0x98b830,
    0x98b7e8, 0x98b7d8];

/// The title of record type `kind` (1 "Most Times Winner" .. 22 "Oldest Player", `COMP_STATS_RECORD_TITLES`) into `out`
/// (at least 0x32 bytes; else the box of line 0xe1d); an unknown type gives the box of line 0xec3 and 0. The object
/// in ecx is not read.
/// Original: `comp_stats_get_record_title` (0x4a5120, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_record_title(_this: usize, kind: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(kind, out, size, 0x32, &COMP_STATS_RECORD_TITLES, 0xe1d, 0xec3)
}

/// The competition's all-time records (+0x26) and current-season records (+0x28f).
const COMP_STATS_ALL_TIME_RECORDS: usize = 0x26;
const COMP_STATS_SEASON_RECORDS: usize = 0x28f;

/// The text of record type `kind` (its low byte) for `view` (1 the all-time records, else the season's) into the
/// Text object `text`; returns what `comp_stats_format_record_text` returns, or 0 with the error box of line 0xf08 for
/// a null Text.
///
/// The Text is cleared and set to "-". For the season records, two are filled in first: type 8 (the highest average
/// attendance), when still unset (+0x6b = -1) and with teams, from the first team after sorting the team table by
/// average attendance (`qsort` with 0x4ad1a0, the table's sort mark +0x18 set to 2, skipped when it already is): the
/// value (team statistic 2, truncated) at +0x6b, its club id at +0x6f and 0 at the word +0x73; type 18 (the highest
/// average rating), when its rating (the f32 +0x189) is still -1.0, by `Comp_Stats::update_season_rating_record`.
/// Original: `Comp_Stats::get_record_text` (0x4a5530, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_record_text(this: usize, view: u32, kind: u32, text: usize) -> u32 {
    use comp_stats_runs as r;
    if text == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0xf08);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    clear(text);
    exe::text_set_string(text, 0x98ba0c as *const u8);
    let rec = this + if view as u8 == 1 { COMP_STATS_ALL_TIME_RECORDS } else { COMP_STATS_SEASON_RECORDS };
    if view as u8 != 1 {
        if kind as u8 == 8 {
            if read_i32(rec + 0x6b) == -1 && read_usize(this + r::TEAMS) != 0 {
                covered!("tel30_12 season attendance record filled");
                if read_u8(this + 0x18) != 2 {
                    write_u8(this + 0x18, 2);
                    write_usize(G_COMP_STATS_SORT_CONTEXT, this);
                    exe::crt_qsort(read_usize(this + r::TEAMS), read_u32(this + r::TEAM_COUNT), r::TEAM_SIZE as u32, 0x4ad1a0usize);
                }
                let first = read_usize(this + r::TEAMS);
                write_i32(rec + 0x6b, ftol(comp_stats_get_team_entry_value(first, 2)));
                write_u32(rec + 0x6f, read_u32(first));
                write_u16(rec + 0x73, 0);
            }
        } else if kind as u8 == 0x12 {
            let rating = read_f32(rec + 0x189) as f64;
            // x87 fcomp with test ah, 0x40: equal or unordered
            if rating == read_f64(rdata::F64_NEG_1) || rating.is_nan() {
                let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4ad7f0usize); // update_season_rating_record
                update(this);
            }
        }
    }
    let format: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x4a5690usize); // format_record_text
    format(this, rec, kind, text)
}

/// A statistic text routine side by side over its buffer (0x33 bytes) and its answer; the error cases (the box) the
/// original only.
unsafe fn comp_stats_compare_stat_text(name: &str, code: i8, out: usize, size: i32, known: bool, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    if out == 0 || size < 0x33 || !known {
        return original();
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, 0x33)], original, copy);
    SCOREBOARD.record(same && r == n, || format!("{}({}): original={} rust={} differences {:?}", name, code, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_stats_format_team_stat` (the competition screen's team statistics table).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_team_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, usize, i32, u32) -> u32>(%%I0%%);
    comp_stats_compare_stat_text("comp_stats_format_team_stat", code, out, size, entry != 0 && (1..=17).contains(&code),
        || o(this, entry, code, out, size, with_detail), || comp_stats_format_team_stat(this, entry, code, out, size, with_detail))
}

/// Comparer for `comp_stats_format_player_stat` (the competition screen's player statistics table).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_player_stat(this: usize, entry: usize, code: i8, out: usize, size: i32, with_detail: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, usize, i32, u32) -> u32>(%%I1%%);
    comp_stats_compare_stat_text("Comp_Stats::format_player_stat", code, out, size, entry != 0 && (1..=17).contains(&code),
        || o(this, entry, code, out, size, with_detail), || comp_stats_format_player_stat(this, entry, code, out, size, with_detail))
}

/// Comparer for `comp_stats_get_record_title`: side by side over the buffer (0x32 bytes) and the answer; the error
/// cases the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_record_title(this: usize, kind: i8, out: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, i32) -> u32>(%%I2%%);
    if out == 0 || size < 0x32 || !(1..=22).contains(&kind) {
        return o(this, kind, out, size);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, 0x32)], || o(this, kind, out, size), || comp_stats_get_record_title(this, kind, out, size));
    SCOREBOARD.record(same && r == n, || format!("comp_stats_get_record_title({}): differences {:?}", kind, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_stats_get_record_text`: the original writes into the screen's Text, the copy into a local one
/// (`compare_text_writer`), and the answers must match; the copy runs after the original, so the season records it
/// fills in are already there (filling them again would change nothing). A null Text the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_record_text(this: usize, view: u32, kind: u32, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32>(%%I3%%);
    if text == 0 {
        return o(this, view, kind, text);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(text, || ro.set(o(this, view, kind, text)), |t| rr.set(comp_stats_get_record_text(this, view, kind, t)),
        || format!("Comp_Stats::get_record_text(view {}, kind {})", view as u8, kind as u8));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("Comp_Stats::get_record_text(view {}, kind {}): original={} rust={}", view as u8, kind as u8, ro.get(), rr.get()));
    ro.get()
}
