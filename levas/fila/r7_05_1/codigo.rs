// --- r7_05_1: one club record as the club records screen shows it (club_records.cpp) ---

/// A match record's pieces as the club records screen writes them: the opponent and its gender, the venue marker, the
/// round label (form 3) and its gender, and the short date.
struct ScreenMatch {
    m: RecordMatch,
    opponent: Club,
    opponent_gender: i32,
    venue: [u8; 8],
    label: [u8; 0xc8],
    label_gender: i32,
    date: [u8; 0x14],
}

impl ScreenMatch {
    /// Builds the pieces in the original's order: the venue marker (`exe::format_venue_marker`, 5 bytes) when `venue`,
    /// the round label of the record's competition (`exe::comp_util_format_comp_stage_round_label`, form 3, 200 bytes),
    /// its gender (`exe::comp_util_get_label_gender`, form 3) and the date (`exe::date_get_short_date_string`, 20 bytes).
    unsafe fn read(at: usize, venue: bool) -> Self {
        let m = RecordMatch(at);
        let opponent = m.opponent();
        let mut s = ScreenMatch { m, opponent, opponent_gender: opponent.short_gender_name() as i8 as i32, venue: [0; 8], label: [0; 0xc8],
            label_gender: 0, date: [0; 0x14] };
        if venue {
            exe::format_venue_marker(m.venue() as u32, s.venue.as_mut_ptr() as usize, 5);
        }
        let (round, stage) = (read_u16(at + club_match_record::ROUND_PREFIX) as u32, read_u16(at + club_match_record::ROUND_NAME) as u32);
        exe::comp_util_format_comp_stage_round_label(m.comp(), round, stage, 3, s.label.as_mut_ptr(), 0xc8);
        s.label_gender = exe::comp_util_get_label_gender(m.comp() as u32, round, stage, 3) as i8 as i32;
        exe::date_get_short_date_string(at, s.date.as_mut_ptr() as usize, 0x14, 0xff);
        s
    }

    fn opponent_name(&self) -> usize {
        self.opponent.addr() + club::SHORT_NAME
    }

    fn label(&self) -> usize {
        self.label.as_ptr() as usize
    }

    fn venue(&self) -> usize {
        self.venue.as_ptr() as usize
    }

    fn date(&self) -> usize {
        self.date.as_ptr() as usize
    }

    /// The opponent's link (0x7d5) and the round label's link to the competition (0x7d0).
    unsafe fn link(&self, out: usize) {
        link_record_club(out, self.opponent_name(), self.opponent);
        exe::text_add_link(out, self.label.as_ptr(), 0x7d0, self.m.comp()); // r7_05_1: the competition's link
    }
}

/// A record holder's name (`exe::database_format_name_from_ids`, `form` 1 full or 2, 200 bytes).
unsafe fn screen_holder_name(holder: usize, form: u32, buf: &mut [u8; 0xc8]) -> RecordHolder {
    exe::database_format_name_from_ids(read_u32(holder), read_u32(holder + 4), read_u32(holder + 8), buf.as_mut_ptr(), 0xc8, form, read_u32(holder + record_holder::STAFF_ID));
    RecordHolder(holder)
}

/// The season of a record (`exe::key_nation_format_club_season`, 20 bytes) when its year is set, otherwise none.
unsafe fn screen_record_season(club: usize, year: u16, buf: &mut [u8; 0x14]) -> Option<usize> {
    if year == 0 {
        return None;
    }
    exe::key_nation_format_club_season(year as u32, club, buf.as_mut_ptr() as usize, 0x14);
    Some(buf.as_ptr() as usize)
}

/// A run record (types 12..15) at `run`, ongoing in the club's current run at `current` (only the unbeaten and winless
/// runs): one game or none is the count alone ("%ld"); more is "<games> - <start> to <end>", the end "?" while the
/// club's current run (`current`) has the same count and start date.
unsafe fn screen_run_text(text: usize, run: usize, template: usize, current: Option<usize>) {
    let fmt = news_text_format();
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let games = read_i32(run);
    if games <= 1 {
        crt_format(text, 0x98c11c, games); // "%ld"
        return;
    }
    let (mut start, mut end) = ([0u8; 0x14], [0u8; 0x14]);
    let start_date = run + run_record::START_DATE;
    exe::date_get_short_date_string(start_date, start.as_mut_ptr() as usize, 0x14, 0xff);
    let ongoing = current.is_some_and(|c| {
        read_i32(c) == games && read_u16(c + run_record::START_DATE + 2) == read_u16(start_date + 2) && read_u16(c + run_record::START_DATE) == read_u16(start_date)
    });
    if ongoing {
        covered!("r7_05_1 ongoing run");
        end[0] = b'?'; // 0x98c014
    } else {
        exe::date_get_short_date_string(run + run_record::END_DATE, end.as_mut_ptr() as usize, 0x14, 0xff);
    }
    fmt(text, template, games, start.as_ptr() as usize, end.as_ptr() as usize);
}

/// rule: one club record as the club records screen shows it, with links to the clubs, the competition, the player and
/// the match; "-" while there is none
///
/// `this.format_record_text(club, records, type, text)`: no Text: the club_records.cpp box of line 0x99a and 0. The
/// Text cleared and set to "-"; no club: the box of line 0x9a6 and 0; no records: 0; a type outside 1..32 (read as a
/// signed byte): the box of line 0xecf and 0. Otherwise the record of the type, when set, is written (langlib) and 1:
/// - 1, 2 the best and worst league position (`club_record::BEST_POSITION_COMP`, `WORST_POSITION_COMP` > -1): "<3rd>
///   in <competition>", plus "<season>" (`exe::key_nation_format_club_season`) when the year is set; the competition
///   linked (0x7d0).
/// - 3, 5 the highest and lowest attendance (+0xe, +0x54 > -1) and 4 the highest gate receipts (+0x31, money with flags
///   6): "<value> v <opponent> <round> <date>" from the match record that follows (+0x12, +0x35, +0x58); the opponent
///   (0x7d5) and the competition (0x7d0) linked.
/// - 6 the average attendance (+0x77 > -1): "<value> - <season>" with the year (+0x7b), the value alone ("%ld")
///   otherwise.
/// - 7..11 the record win, league win and defeat and the highest scoring game and league game (the match record's
///   competition > -1): "<goals for>-<goals against> v <opponent> (<venue>) <round> <date>"; the opponent, the
///   competition and the score (0x7d3, while the fixture still matches) linked.
/// - 12..15 the runs won, lost, unbeaten and without winning (games > -1): `screen_run_text`; the unbeaten and winless
///   runs end in "?" while the club's current run (`club_entry_runs::UNBEATEN_RUN`, `WINLESS_RUN` of the club's entry)
///   is the same one.
/// - 16, 17, 19, 21, 22 the goals, league goals, assists, man of the match awards and cards (signed byte > -1), with
///   the season when its year is set; 23, 24, 27 the league appearances, league goals and caps (signed word > -1); 20
///   the best average rating (> -1.0): "<4.2f> - <player> (<apps> apps)"; the player linked (0x7d7) while his names
///   still match.
/// - 18 the goals in one match and 25, 26 the youngest and oldest player (the match's competition > -1): the player
///   (name form 2), the goals or the age (`database_format_years_between_dates`), the match as in 7..11; the opponent,
///   the competition and the player linked.
/// - 28, 29 the record purchase and sale (fee > -1): "<money> [p/x] - <player> from/to <club> - <date>" (the date in
///   100 bytes; "p/x" with the record's flag); the other club and the player linked.
/// - 30, 31 the season's transfer spending and income (> -1): "<money>", plus "- <year>" when the year is set.
/// - 32 the fans' player of the year (the holder's id > -1): his name; linked while it still matches.
/// A record not set leaves the "-" and returns 1.
/// Original: `club_records_format_record_text` (0x449810, club_records.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn club_records_format_record_text(this: usize, club: usize, records: usize, kind: u32, out: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    if out == 0 {
        show_error_at(FILE, 0x99a);
        return 0;
    }
    exe::text_clear(out as *mut usize);
    exe::text_set_string(out, 0x98ba0c as *const u8); // "-"
    if club == 0 {
        show_error_at(FILE, 0x9a6);
        return 0;
    }
    if records == 0 {
        return 0;
    }
    let kind = kind as u8 as i8 as i32;
    if !(1..=32).contains(&kind) {
        show_error_at(FILE, 0xecf);
        return 0;
    }
    covered!("r7_05_1 record text");
    format_screen_record(this, club, records, kind as u8, out);
    1
}

/// Writes the record of type `kind` (1..32) of `r` into the Text `out` with its links; a record not set leaves it as
/// it is.
unsafe fn format_screen_record(this: usize, club: usize, r: usize, kind: u8, out: usize) {
    let mut text = [0u8; 500];
    let tp = text.as_mut_ptr() as usize;
    let fmt = news_text_format();
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let set = || {
        exe::text_set_string(out, tp as *const u8);
    };
    let (mut name, mut season) = ([0u8; 0xc8], [0u8; 0x14]);
    match kind {
        // the best and worst league position
        1 | 2 => {
            let (pos, comp_at, year_at) = if kind == 1 { (0, club_record::BEST_POSITION_COMP, club_record::BEST_POSITION_YEAR) } else {
                (club_record::WORST_POSITION, club_record::WORST_POSITION_COMP, club_record::WORST_POSITION_YEAR)
            };
            let idx = read_i32(r + comp_at);
            if idx <= -1 {
                return;
            }
            let mut ordinal = [0u8; 0x14];
            let o = exe::utils_format_ordinal(read_u8(r + pos) as i32 + 1, -1);
            for (k, c) in ordinal.iter_mut().enumerate().take(0x13) {
                *c = read_u8(o + k);
                if *c == 0 {
                    break;
                }
            }
            let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
            let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
            let cname = comp + competition::SHORT_NAME;
            match screen_record_season(club, read_u16(r + year_at), &mut season) {
                Some(s) => fmt(tp, 0x98c288, cg, cg, ordinal.as_ptr() as usize, cname, s),
                None => fmt(tp, 0x98c240, cg, cg, ordinal.as_ptr() as usize, cname),
            };
            set();
            exe::text_add_link(out, cname as *const u8, 0x7d0, comp);
        }
        // the highest attendance, the highest gate receipts and the lowest attendance, with the match
        3 | 4 | 5 => {
            let at = [0xe, 0x31, 0x54][kind as usize - 3];
            let value = read_i32(r + at);
            if value <= -1 {
                return;
            }
            let s = ScreenMatch::read(r + at + 4, false);
            if kind == 4 {
                let mut money = [0u8; 0xc8];
                record_money_text(value as u32, 6, &mut money);
                fmt(tp, 0x98c160, s.opponent_gender, s.label_gender, s.label_gender, 0x64, money.as_ptr() as usize, s.opponent_name(), s.label(), s.date());
            } else {
                fmt(tp, 0x98c1d8, s.opponent_gender, s.label_gender, s.label_gender, 0x64, value, s.opponent_name(), s.label(), s.date());
            }
            set();
            s.link(out);
        }
        // the average attendance
        6 => {
            let value = read_i32(r + 0x77);
            if value <= -1 {
                return;
            }
            match screen_record_season(club, read_u16(r + 0x7b), &mut season) {
                Some(s) => fmt(tp, 0x98c120, value, s),
                None => crt_format(tp, 0x98c11c, value), // "%ld"
            };
            set();
        }
        // the record win, league win and defeat, the highest scoring game and league game
        7..=11 => {
            let at = r + [0x7d, 0x9c, 0xbb, 0xda, 0xf9][kind as usize - 7];
            if RecordMatch(at).comp_index() <= -1 {
                return;
            }
            let mut score = [0u8; 0x14];
            let (g1, g2) = RecordMatch(at).goals();
            crt_format(score.as_mut_ptr() as usize, 0x98c114, g1 as i32, g2 as i32); // r7_05_1: "%d-%d"
            let s = ScreenMatch::read(at, true);
            fmt(tp, 0x98c09c, s.opponent_gender, s.label_gender, s.label_gender, 0x64, score.as_ptr() as usize, s.opponent_name(), s.venue(), s.label(), s.date());
            set();
            s.link(out);
            s.m.link_score(out, &score);
        }
        // the runs: won, lost, unbeaten and without winning
        12..=15 => {
            let run = r + [0x118, 0x12c, 0x140, 0x154][kind as usize - 12];
            if read_i32(run) <= -1 {
                return;
            }
            let entry = club_records_entry(this, club);
            let (template, current) = match kind {
                12 => (0x98c05c, None),
                13 => (0x98c018, None),
                14 => (0x98bfd0, Some(entry + club_entry_runs::UNBEATEN_RUN)),
                _ => (0x98bf88, Some(entry + club_entry_runs::WINLESS_RUN)),
            };
            screen_run_text(tp, run, template, current);
            set();
        }
        // the player records counted in a signed byte, with the season: goals, league goals, assists, man of the match
        16 | 17 | 19 | 21 | 22 => {
            let (count_at, holder_at, year_at) = match kind {
                16 => (0x168, 0x169, 0x181),
                17 => (0x183, 0x184, 0x19c),
                19 => (0x1d6, 0x1d7, 0x1ef),
                21 => (0x210, 0x211, 0x229),
                _ => (0x22b, 0x22d, 0x245),
            };
            let count = read_i8(r + count_at) as i32;
            if count <= -1 {
                return;
            }
            let h = screen_holder_name(r + holder_at, 1, &mut name);
            let player = name.as_ptr() as usize;
            let s = screen_record_season(club, read_u16(r + year_at), &mut season);
            match (kind, s) {
                (16 | 17, Some(s)) => fmt(tp, 0x98bf48, player, count, s),
                (16 | 17, None) => fmt(tp, 0x98bf24, player, count),
                (19, Some(s)) => fmt(tp, 0x98be58, count, player, s),
                (19, None) => fmt(tp, 0x98be34, count, player),
                (21, Some(s)) => fmt(tp, 0x98bd44, count, player, s),
                (21, None) => fmt(tp, 0x98bd18, count, player),
                // the cards: the yellow (+0x22b) and the red (+0x22c)
                (_, Some(s)) => fmt(tp, 0x98bca8, player, count, read_i8(r + 0x22c) as i32, s),
                (_, None) => fmt(tp, 0x98bc54, player, count, read_i8(r + 0x22c) as i32),
            };
            set();
            h.link(out, &name);
        }
        // the player records counted in a signed word: league appearances, league goals, caps
        23 | 24 | 27 => {
            let count_at = match kind {
                23 => club_record::LEAGUE_APPEARANCES,
                24 => club_record::LEAGUE_GOALS,
                _ => club_record::INTERNATIONAL_CAPS,
            };
            let count = read_i16(r + count_at) as i32;
            if count <= -1 {
                return;
            }
            let h = screen_holder_name(r + count_at + 2, 1, &mut name);
            let player = name.as_ptr() as usize;
            match kind {
                23 => fmt(tp, 0x98bc2c, count, player),
                24 => fmt(tp, 0x98bc08, count, player),
                _ => fmt(tp, 0x98bb58, player, count),
            };
            set();
            h.link(out, &name);
        }
        // the best average rating
        20 => {
            let rating = read_f32(r + club_record::SEASON_RATING);
            if !(rating as f64 > read_f64(rdata::F64_NEG_1)) {
                return;
            }
            let h = screen_holder_name(r + 0x1f5, 1, &mut name);
            let (player, games) = (name.as_ptr() as usize, read_i8(r + club_record::SEASON_RATING_GAMES) as i32);
            match screen_record_season(club, read_u16(r + club_record::SEASON_RATING_YEAR), &mut season) {
                Some(s) => fmt(tp, 0x98bdd0, rating as f64, player, games, s),
                None => fmt(tp, 0x98bd88, rating as f64, player, games),
            };
            set();
            h.link(out, &name);
        }
        // the goals in one match
        18 => {
            let goals = read_i8(r + 0x19e) as i32;
            if goals <= -1 {
                return;
            }
            let h = screen_holder_name(r + 0x19f, 2, &mut name);
            let s = ScreenMatch::read(r + 0x1b7, true);
            fmt(tp, 0x98be98, s.opponent_gender, s.label_gender, s.label_gender, 0x64, goals, name.as_ptr() as usize, s.opponent_name(), s.venue(), s.label(), s.date());
            set();
            s.link(out);
            h.link(out, &name);
        }
        // the youngest and the oldest player, with his age at the match
        25 | 26 => {
            let base = if kind == 25 { 0x27b } else { 0x2ba };
            if RecordMatch(r + base + 0x20).comp_index() <= -1 {
                return;
            }
            let h = screen_holder_name(r + base + 8, 2, &mut name);
            let mut age = [0u8; 0x14];
            let years: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
            years(r + base + 0x20, r + base, age.as_mut_ptr() as usize, 5);
            let s = ScreenMatch::read(r + base + 0x20, true);
            fmt(tp, 0x98bb7c, s.opponent_gender, s.label_gender, s.label_gender, 0x64, name.as_ptr() as usize, age.as_ptr() as usize, s.opponent_name(), s.venue(), s.label(), s.date());
            set();
            s.link(out);
            h.link(out, &name);
        }
        // the record purchase and sale
        28 | 29 => {
            let base = if kind == 28 { club_record::FEE_PAID } else { club_record::FEE_RECEIVED };
            let fee = read_i32(r + base);
            if fee <= -1 {
                return;
            }
            let h = screen_holder_name(r + base + transfer_fee_record::HOLDER, 1, &mut name);
            let mut date = [0u8; 0x64];
            exe::date_get_short_date_string(r + base + transfer_fee_record::DATE, date.as_mut_ptr() as usize, 0x64, 0xff);
            let mut money = [0u8; 0xc8];
            record_money_text(fee as u32, 6, &mut money);
            let other = club_at(read_i32(r + base + transfer_fee_record::OTHER_CLUB_ID) as isize);
            let (xname, xg) = (other.addr() + club::SHORT_NAME, other.short_gender_name() as i8 as i32);
            let exchange = read_u8(r + base + transfer_fee_record::FLAG) != 0;
            let template = match (kind, exchange) {
                (28, true) => 0x98bb0c,
                (28, false) => 0x98bac4,
                (_, true) => 0x98ba7c,
                _ => 0x98ba38,
            };
            fmt(tp, template, xg, xg, 0x64, money.as_ptr() as usize, name.as_ptr() as usize, xname, date.as_ptr() as usize);
            set();
            link_record_club(out, xname, other);
            h.link(out, &name);
        }
        // the season's transfer spending and income
        30 | 31 => {
            let (at, year_at) = if kind == 30 { (club_record::TRANSFER_SPENDING, club_record::TRANSFER_SPENDING_YEAR) } else {
                (club_record::TRANSFER_INCOME, club_record::TRANSFER_INCOME_YEAR)
            };
            let value = read_i32(r + at);
            if value <= -1 {
                return;
            }
            let mut money = [0u8; 0xc8];
            record_money_text(value as u32, 6, &mut money);
            let year = read_u16(r + year_at);
            if year != 0 {
                fmt(tp, 0x98ba1c, money.as_ptr() as usize, year as u32);
            } else {
                fmt(tp, 0x98ba10, money.as_ptr() as usize);
            }
            set();
        }
        // the fans' player of the year
        _ => {
            let at = r + club_record::FANS_PLAYER;
            if read_i32(at + record_holder::STAFF_ID) <= -1 {
                return;
            }
            let h = screen_holder_name(at, 1, &mut name);
            exe::text_set_string(out, name.as_ptr());
            h.link(out, &name);
        }
    }
}

/// Comparer for `club_records_format_record_text`: the text side by side (`compare_text_writer`: the text, the links
/// and the generator) and the return. Without the Text, the club or the records, or with a type outside 1..32 (the error
/// boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_club_records_format_record_text(this: usize, club: usize, records: usize, kind: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32>(%%I0%%);
    let k = kind as u8 as i8 as i32;
    if out == 0 || club == 0 || records == 0 || !(1..=32).contains(&k) {
        return o(this, club, records, kind, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(this, club, records, kind, out)), |t| rr.set(club_records_format_record_text(this, club, records, kind, t)),
        || format!("club_records_format_record_text(type {})", k));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("club_records_format_record_text(type {}) returned {} x {}", k, ro.get(), rr.get()));
    ro.get()
}
