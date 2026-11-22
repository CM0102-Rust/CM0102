// ---------- batch %%LEVA%%: the competition statistics (2): a record as text and as news (comp_screens.cpp) ----------

/// `langlib_sprintf` (0x66f4e0): the language library's printf into a buffer; `{}` takes an int (the gender of the
/// word that follows, for the grammar) before the `%` arguments.
type RecordSprintf = unsafe extern "C" fn(*mut u8, usize, ...) -> i32;
/// `Date::get_short_date_string` (0x54a650, thiscall(date, buffer, size, flags), ret 0xc).
type RecordDateString = unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize;
/// `key_nation_format_comp_season` (0x6697c0, cdecl(year, competition id, buffer, size)): "1998/99" and the like.
type RecordSeasonText = unsafe extern "C" fn(u32, u32, *mut u8, u32) -> usize;
/// `format_venue_marker` (0x7e4ca0, cdecl(venue, buffer, size)): "H", "A" or "N".
type RecordVenueMarker = unsafe extern "C" fn(u32, *mut u8, u32) -> usize;

/// The link kinds of a record text: a competition, a match, a club and a person.
const LINK_COMPETITION: u32 = 0x7d0;
const LINK_MATCH: u32 = 0x7d3;
const LINK_CLUB: u32 = 0x7d5;
const LINK_PERSON: u32 = 0x7d7;

/// The record's club name, as the original copies it (`club::SHORT_NAME`, a C string).
unsafe fn record_club_name(club: usize, buf: &mut [u8; 0xc8]) {
    let mut k = 0;
    loop {
        let c = read_u8(club + club::SHORT_NAME + k);
        buf[k] = c;
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// The club with the id at `rec + o`.
unsafe fn record_club(rec: usize, o: usize) -> usize {
    club_at(read_i32(rec + o) as isize).0
}

/// The gender byte of a club's short name (`club::SHORT_GENDER_NAME`), sign-extended, for the `{}` of the formats.
unsafe fn record_gender(club: usize) -> i32 {
    read_i8(club + club::SHORT_GENDER_NAME) as i32
}

/// The season text of a record's year (`key_nation_format_comp_season` with the competition of the stats).
unsafe fn record_season(stats: usize, year: u16, buf: &mut [u8; 0x14]) {
    let f: RecordSeasonText = core::mem::transmute(0x6697c0usize);
    f(year as u32, read_u32(read_usize(stats + comp_stats_obj::COMP) + 4), buf.as_mut_ptr(), 0x14);
}

/// The short date of the date at `date` into `buf`.
unsafe fn record_date(date: usize, buf: &mut [u8; 0x14]) {
    let f: RecordDateString = core::mem::transmute(0x54a650usize);
    f(date, buf.as_mut_ptr(), 0x14, 0xff);
}

/// The person name of a record holder (first, second and common name ids at +0, +4, +8, the person at +0xc) into
/// `buf`, with the name `style` of `database_format_name_from_ids`.
unsafe fn record_holder_name(holder: usize, buf: &mut [u8; 0xc8], style: u32) {
    exe::database_format_name_from_ids(read_u32(holder), read_u32(holder + 4), read_u32(holder + 8), buf.as_mut_ptr(), 0xc8, style,
        read_u32(holder + 0xc));
}

/// The person link of a record holder: only when the person still has those names (`database_staff_matches_names`
/// with the three ids and the date at +0x10).
unsafe fn record_holder_link(text: usize, holder: usize, label: &[u8; 0xc8]) {
    let person = staff_at(read_i32(holder + 0xc) as isize).0;
    if exe::database_staff_matches_names(person, read_u32(holder), read_u32(holder + 4), read_u32(holder + 8), holder + 0x10) != 0 {
        let person = staff_at(read_i32(holder + 0xc) as isize).0;
        exe::text_add_link(text, label.as_ptr(), LINK_PERSON, person);
    }
}

/// The stage and round name of a match record (round prefix and name words at `+0xc`, `+0xe`) into `buf`.
unsafe fn record_stage(m: usize, buf: &mut [u8; 0xc8]) {
    exe::comp_util_format_stage_round_name(read_i16(m + 0xc) as i32, read_i16(m + 0xe) as i32, buf.as_mut_ptr() as usize, 0xc8, 1);
}

/// The competition link of a match record's stage (the competition id at `m + 8`).
unsafe fn record_stage_link(text: usize, m: usize, stage: &[u8; 0xc8]) {
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(m + 8) as usize).wrapping_mul(0x6b));
    exe::text_add_link(text, stage.as_ptr(), LINK_COMPETITION, comp);
}

/// A team record of one number: `value` - team (gender twice, the value, the name and, with a year, the season),
/// the club linked.
unsafe fn record_team_value(stats: usize, text: usize, rec: usize, value: i32, club_at_: usize, year_at: usize, with_season: usize, without: usize) {
    let mut out = [0u8; 0x200];
    let mut name = [0u8; 0xc8];
    let mut season = [0u8; 0x14];
    let club = record_club(rec, club_at_);
    record_club_name(club, &mut name);
    let gender = record_gender(club);
    let year = read_u16(rec + year_at);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if year > 0 {
        record_season(stats, year, &mut season);
        f(out.as_mut_ptr(), with_season, gender, gender, value, name.as_ptr(), season.as_ptr());
    } else {
        f(out.as_mut_ptr(), without, gender, gender, value, name.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, club_at_));
}

/// An attendance record (kinds 6 and 7): the attendance at `rec + at` and the match record after it (date +0,
/// competition +8, round +0xc/+0xe, the club +0x11 and the opponent +0x15).
unsafe fn record_attendance(text: usize, rec: usize, at: usize) {
    let m = rec + at + 4;
    let mut out = [0u8; 0x200];
    let (mut home, mut away, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut date = [0u8; 0x14];
    let (hc, ac) = (record_club(m, 0x11), record_club(m, 0x15));
    record_club_name(hc, &mut home);
    record_club_name(ac, &mut away);
    let (hg, ag) = (record_gender(hc), record_gender(ac));
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let attendance = read_i32(rec + at);
    if stage[0] != 0 {
        f(out.as_mut_ptr(), 0x99d62c, hg, hg, ag, ag, 0x64, attendance, home.as_ptr(), away.as_ptr(), stage.as_ptr(), date.as_ptr());
    } else {
        f(out.as_mut_ptr(), 0x99d5dc, hg, hg, ag, ag, 0x64, attendance, home.as_ptr(), away.as_ptr(), date.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, home.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, away.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
}

/// A result record (kinds 9 and 10): the match record at `rec + at` (date +0, competition +8, round +0xc/+0xe, the
/// club +0x11, the opponent +0x15, the goals +0x19/+0x1a, the match statistics slot +0x1b); with `venue` 2 the two
/// names change places in the text (the genders do not).
unsafe fn record_result(text: usize, rec: usize, at: usize, venue: u8) {
    let m = rec + at;
    let mut out = [0u8; 0x200];
    let mut score = [0u8; 0x14];
    let (mut home, mut away, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut date = [0u8; 0x14];
    let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    crt(score.as_mut_ptr(), 0x98c114, read_i8(m + 0x19) as i32, read_i8(m + 0x1a) as i32);
    let (hc, ac) = (record_club(m, 0x11), record_club(m, 0x15));
    record_club_name(hc, &mut home);
    record_club_name(ac, &mut away);
    let (hg, ag) = (record_gender(hc), record_gender(ac));
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let (first, second) = if venue == 2 { (away.as_ptr(), home.as_ptr()) } else { (home.as_ptr(), away.as_ptr()) };
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if stage[0] != 0 {
        f(out.as_mut_ptr(), 0x99d4f4, hg, hg, ag, ag, 0x64, score.as_ptr(), first, second, stage.as_ptr(), date.as_ptr());
    } else {
        f(out.as_mut_ptr(), 0x99d4a8, hg, hg, ag, ag, 0x64, score.as_ptr(), first, second, date.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, home.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, away.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
    let slot = read_i32(m + 0x1b);
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, read_i32(m + 0x11), read_i32(m + 0x15),
        read_i16(m) as i32, read_i16(m + 2) as i32) != 0 {
        exe::text_add_link(text, score.as_ptr(), LINK_MATCH, read_u32(m + 0x1b) as usize);
    }
}

/// A run record (kinds 11 to 14) at `rec + at`: the games (+0), the start (+4) and end (+0xc) dates and the club
/// (+0x14). One game: only the number and the team. The unbeaten and winless runs (`ongoing` = the club entry's run
/// offset) end in "?" while the club's current run is this one (same games and same start date).
unsafe fn record_run(stats: usize, text: usize, rec: usize, at: usize, ongoing: Option<usize>, gendered_dates: bool, formats: (usize, usize)) {
    let r = rec + at;
    let mut out = [0u8; 0x200];
    let mut name = [0u8; 0xc8];
    let (mut start, mut end) = ([0u8; 0x14], [0u8; 0x14]);
    let club = record_club(r, 0x14);
    record_club_name(club, &mut name);
    let gender = record_gender(club);
    let games = read_i32(r);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if games > 1 {
        record_date(r + 4, &mut start);
        let mut current = false;
        if let Some(run) = ongoing {
            let entry = exe::comp_stats_find_club_entry(stats, record_club(r, 0x14));
            current = entry != 0 && read_i32(r) == read_i32(entry + run)
                && exe::date_operator_3d_3d(r + 4, read_u32(entry + run + 4), read_u32(entry + run + 8)) != 0;
        }
        if current {
            covered!("tel31_2 run still going");
            end[0] = b'?';
            end[1] = 0;
        } else {
            record_date(r + 0xc, &mut end);
        }
        if gendered_dates {
            f(out.as_mut_ptr(), formats.0, gender, gender, 0x64, read_i32(r), name.as_ptr(), start.as_ptr(), end.as_ptr());
        } else {
            f(out.as_mut_ptr(), formats.0, gender, gender, read_i32(r), name.as_ptr(), start.as_ptr(), end.as_ptr());
        }
    } else {
        f(out.as_mut_ptr(), formats.1, gender, gender, games, name.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(r, 0x14));
}

/// A player record with a count (kinds 17, 19 and 20) at `rec + at`: the counts, the holder (+1 or +2), the club and
/// the year after the holder.
unsafe fn record_player_count(stats: usize, text: usize, rec: usize, at: usize, two_counts: bool, formats: (usize, usize)) {
    let r = rec + at;
    let holder = r + if two_counts { 2 } else { 1 };
    let mut out = [0u8; 0x200];
    let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
    let mut season = [0u8; 0x14];
    record_holder_name(holder, &mut name, 1);
    let club = record_club(holder, 0x18);
    record_club_name(club, &mut team);
    let year = read_u16(holder + 0x1c);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (a, b) = (read_i8(r) as i32, read_i8(r + 1) as i32);
    if year > 0 {
        record_season(stats, year, &mut season);
        if two_counts {
            f(out.as_mut_ptr(), formats.0, name.as_ptr(), team.as_ptr(), a, b, season.as_ptr());
        } else {
            f(out.as_mut_ptr(), formats.0, a, name.as_ptr(), team.as_ptr(), season.as_ptr());
        }
    } else if two_counts {
        f(out.as_mut_ptr(), formats.1, name.as_ptr(), team.as_ptr(), a, b);
    } else {
        f(out.as_mut_ptr(), formats.1, a, name.as_ptr(), team.as_ptr());
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(holder, 0x18));
    record_holder_link(text, holder, &name);
}

/// A player record against an opponent (kind 16: goals in a match; kinds 21 and 22: the age): the holder, the match
/// record after it (date, competition, round, venue, club and opponent), the club linked, the opponent linked, the
/// stage linked and the person linked.
unsafe fn record_player_match(text: usize, holder: usize, m: usize, lead: RecordLead) {
    let mut out = [0u8; 0x200];
    let (mut name, mut team, mut opponent, mut stage) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let (mut venue, mut date, mut age) = ([0u8; 0x14], [0u8; 0x14], [0u8; 0x14]);
    record_holder_name(holder, &mut name, 2);
    if let RecordLead::Age(birth) = lead {
        database_format_years_between_dates(m, birth, age.as_mut_ptr() as usize, 5);
    }
    record_club_name(record_club(m, 0x11), &mut team);
    let oc = record_club(m, 0x15);
    record_club_name(oc, &mut opponent);
    let og = record_gender(oc);
    let marker: RecordVenueMarker = core::mem::transmute(0x7e4ca0usize);
    marker(read_u8(m + 0x10) as u32, venue.as_mut_ptr(), 5);
    record_stage(m, &mut stage);
    record_date(m, &mut date);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match lead {
        RecordLead::Goals(goals) => {
            if stage[0] != 0 {
                f(out.as_mut_ptr(), 0x99d190, og, og, 0x64, goals, name.as_ptr(), team.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  stage.as_ptr(), date.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d11c, og, og, 0x64, goals, name.as_ptr(), team.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  date.as_ptr());
            }
        }
        RecordLead::Age(_) => {
            if stage[0] != 0 {
                f(out.as_mut_ptr(), 0x99cdd4, og, og, 0x64, name.as_ptr(), team.as_ptr(), age.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  stage.as_ptr(), date.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99cd64, og, og, 0x64, name.as_ptr(), team.as_ptr(), age.as_ptr(), opponent.as_ptr(), venue.as_ptr(),
                  date.as_ptr());
            }
        }
    }
    exe::text_set_string(text, out.as_ptr());
    exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(m, 0x11));
    exe::text_add_link(text, opponent.as_ptr(), LINK_CLUB, record_club(m, 0x15));
    if stage[0] != 0 {
        record_stage_link(text, m, &stage);
    }
    record_holder_link(text, holder, &name);
}

/// What leads a player-against-opponent record: the goals, or the age (from the birth date at the address).
#[derive(Clone, Copy)]
enum RecordLead {
    Goals(i32),
    Age(usize),
}

/// Rule: a competition record becomes one line of text with links: "-" first, then, when the record holds
/// something, the kind's sentence (team records, attendances, results, runs, player records), with the club, the
/// opponent, the stage, the match and the person linked.
///
/// `comp_stats_format_record_text(records, kind, text)`: no text: the comp_stats.CPP 3911 box and 0; `Text::clear`
/// and "-"; no records: the comp_stats.CPP 3923 box and 0. By kind (1..22; any other: the comp_stats.CPP 4973 box and
/// 0), a record that is empty (its count below 0, the rating not above -1.0) stays "-":
/// 1 most wins (byte +0, club +1); 2 points (+5, club +7, year +0xb), 3 and 4 goals (+0xd / +0x15) and 5 cards
/// (yellow +0x1d, red +0x1e) as team values with the season when the year is set; 6 and 7 the attendances (+0x25 /
/// +0x48, with the match after them); 8 the average attendance (+0x6b, club +0x6f, year +0x73: an average of 0 or less
/// gives "-", the club still linked); 9 and 10 the results (match records +0x75 / +0x94, the names in venue order:
/// both read the venue byte at +0xa4, the second record's, a defect of the original for kind 9); 11 to 14 the runs
/// (+0xb3, +0xcb, +0xe3, +0xfb; the unbeaten and winless ones end in "?" while still going); 15 the top scorer and 16
/// the most goals in a match (holder, club, year); 17 assists, 18 the average rating ("%4.2f", the games), 19 man of
/// the match, 20 the cards; 21 and 22 the youngest and oldest (the age in years at the match). Returns 1.
/// Original: `comp_stats_format_record_text` (0x4a5690, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_format_record_text(stats: usize, rec: usize, kind: u32, text: usize) -> u32 {
    if text == 0 || rec == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%)(stats, rec, kind, text);
    }
    let kind = kind as u8 as i8;
    if !(1..=22).contains(&kind) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%)(stats, rec, kind as u8 as u32, text);
    }
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4afa40usize);
    clear(text);
    exe::text_set_string(text, 0x98ba0c as *const u8);
    let f: RecordSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match kind {
        1 => {
            let wins = read_i8(rec);
            if wins > -1 {
                let mut out = [0u8; 0x200];
                let mut name = [0u8; 0xc8];
                record_club_name(record_club(rec, 1), &mut name);
                f(out.as_mut_ptr(), 0x99d80c, name.as_ptr(), wins as i32);
                exe::text_set_string(text, out.as_ptr());
                exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 1));
            }
        }
        2 if read_i16(rec + 5) > -1 => record_team_value(stats, text, rec, read_i16(rec + 5) as i32, 7, 0xb, 0x99d7d0, 0x99d7b0),
        3 if read_i16(rec + 0xd) > -1 => record_team_value(stats, text, rec, read_i16(rec + 0xd) as i32, 0xf, 0x13, 0x99d774, 0x99d754),
        4 if read_i16(rec + 0x15) > -1 => record_team_value(stats, text, rec, read_i16(rec + 0x15) as i32, 0x17, 0x1b, 0x99d774, 0x99d754),
        5 if read_i8(rec + 0x1d) > -1 => {
            let mut out = [0u8; 0x200];
            let mut name = [0u8; 0xc8];
            let mut season = [0u8; 0x14];
            let club = record_club(rec, 0x1f);
            record_club_name(club, &mut name);
            let gender = record_gender(club);
            let year = read_u16(rec + 0x23);
            let (yellow, red) = (read_i8(rec + 0x1d) as i32, read_i8(rec + 0x1e) as i32);
            if year > 0 {
                record_season(stats, year, &mut season);
                f(out.as_mut_ptr(), 0x99d6e8, gender, gender, name.as_ptr(), yellow, red, season.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d698, gender, gender, name.as_ptr(), yellow, red);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 0x1f));
        }
        6 if read_i32(rec + 0x25) > -1 => record_attendance(text, rec, 0x25),
        7 if read_i32(rec + 0x48) > -1 => record_attendance(text, rec, 0x48),
        8 if read_i32(rec + 0x6b) > -1 => {
            let mut out = [0u8; 0x200];
            let mut name = [0u8; 0xc8];
            let mut season = [0u8; 0x14];
            let club = record_club(rec, 0x6f);
            record_club_name(club, &mut name);
            let gender = record_gender(club);
            let year = read_u16(rec + 0x73);
            let average = read_i32(rec + 0x6b);
            if year > 0 {
                record_season(stats, year, &mut season);
            }
            if average > 0 {
                if year > 0 {
                    f(out.as_mut_ptr(), 0x99d58c, gender, gender, average, name.as_ptr(), season.as_ptr());
                } else {
                    f(out.as_mut_ptr(), 0x99d55c, gender, gender, average, name.as_ptr());
                }
            } else {
                let crt: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                crt(out.as_mut_ptr(), 0x991944, 0x98ba0c);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, name.as_ptr(), LINK_CLUB, record_club(rec, 0x6f));
        }
        // the original reads the venue at +0xa4 for both results: the second record's venue (kind 10's) also for
        // kind 9 (defect of the original, kept)
        9 if read_i32(rec + 0x7d) > -1 => record_result(text, rec, 0x75, read_u8(rec + 0xa4)),
        10 if read_i32(rec + 0x9c) > -1 => record_result(text, rec, 0x94, read_u8(rec + 0xa4)),
        11 if read_i32(rec + 0xb3) > -1 => record_run(stats, text, rec, 0xb3, None, true, (0x99d454, 0x99d428)),
        12 if read_i32(rec + 0xcb) > -1 => record_run(stats, text, rec, 0xcb, None, true, (0x99d3d4, 0x99d3a8)),
        13 if read_i32(rec + 0xe3) > -1 => record_run(stats, text, rec, 0xe3, Some(comp_stats_club::UNBEATEN_RUN), false, (0x99d354, 0x99d324)),
        14 if read_i32(rec + 0xfb) > -1 => record_run(stats, text, rec, 0xfb, Some(comp_stats_club::WINLESS_RUN), false, (0x99d2cc, 0x99d29c)),
        15 if read_i8(rec + 0x113) > -1 => {
            let holder = rec + 0x114;
            let mut out = [0u8; 0x200];
            let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
            let mut season = [0u8; 0x14];
            record_holder_name(holder, &mut name, 1);
            record_club_name(record_club(rec, 0x12c), &mut team);
            let year = read_u16(rec + 0x130);
            let goals = read_i8(rec + 0x113) as i32;
            if year > 0 {
                record_season(stats, year, &mut season);
                f(out.as_mut_ptr(), 0x99d250, name.as_ptr(), team.as_ptr(), goals, season.as_ptr());
            } else {
                f(out.as_mut_ptr(), 0x99d220, name.as_ptr(), team.as_ptr(), goals);
            }
            exe::text_set_string(text, out.as_ptr());
            exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(rec, 0x12c));
            record_holder_link(text, holder, &name);
        }
        16 if read_i8(rec + 0x132) > -1 => record_player_match(text, rec + 0x133, rec + 0x14b, RecordLead::Goals(read_i8(rec + 0x132) as i32)),
        17 if read_i8(rec + 0x16a) > -1 => record_player_count(stats, text, rec, 0x16a, false, (0x99d0cc, 0x99d098)),
        18 => {
            let rating = read_f32(rec + 0x189);
            if rating > -1.0 {
                let holder = rec + 0x18d;
                let mut out = [0u8; 0x200];
                let (mut name, mut team) = ([0u8; 0xc8], [0u8; 0xc8]);
                let mut season = [0u8; 0x14];
                record_holder_name(holder, &mut name, 1);
                record_club_name(record_club(rec, 0x1a6), &mut team);
                let year = read_u16(rec + 0x1aa);
                let games = read_i8(rec + 0x1a5) as i32;
                if year > 0 {
                    record_season(stats, year, &mut season);
                    f(out.as_mut_ptr(), 0x99d024, read_f32(rec + 0x189) as f64, name.as_ptr(), team.as_ptr(), games, season.as_ptr());
                } else {
                    f(out.as_mut_ptr(), 0x99cfcc, read_f32(rec + 0x189) as f64, name.as_ptr(), team.as_ptr(), games);
                }
                exe::text_set_string(text, out.as_ptr());
                exe::text_add_link(text, team.as_ptr(), LINK_CLUB, record_club(rec, 0x1a6));
                record_holder_link(text, holder, &name);
            }
        }
        19 if read_i8(rec + 0x1ac) > -1 => record_player_count(stats, text, rec, 0x1ac, false, (0x99cf78, 0x99cf40)),
        20 if read_i8(rec + 0x1cb) > -1 => record_player_count(stats, text, rec, 0x1cb, true, (0x99cec4, 0x99ce60)),
        21 if read_i32(rec + 0x213) > -1 => record_player_match(text, rec + 0x1f3, rec + 0x20b, RecordLead::Age(rec + 0x1eb)),
        22 if read_i32(rec + 0x252) > -1 => record_player_match(text, rec + 0x232, rec + 0x24a, RecordLead::Age(rec + 0x22a)),
        _ => {}
    }
    1
}

/// Comparer for `comp_stats_format_record_text`: the text, side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_format_record_text(stats: usize, rec: usize, kind: u32, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%);
    let k = kind as u8 as i8;
    if text == 0 || rec == 0 || !(1..=22).contains(&k) {
        return o(stats, rec, kind, text);
    }
    let mut r = 0;
    compare_text_writer(text, || r = o(stats, rec, kind, text), |t| { comp_stats_format_record_text(stats, rec, kind, t); },
        || format!("comp_stats_format_record_text(kind {})", k));
    r
}

/// `write_date_span_record_to_news` (0x7e5420, cdecl(record, news, &slot)).
type NewsDateSpanWriter = unsafe extern "C" fn(usize, usize, *mut u8);

/// Rule: a competition record goes into a news item's slots: the first value, the kind and then the kind's fields
/// in order (counts, clubs, years, match records, date spans and the holders' names), each in the next slot.
///
/// `comp_stats_write_record_to_news(first, records, kind, news)`: no records: the comp_stats.CPP 6839 box and 0; no
/// news: the comp_stats.CPP 6847 box and 0. Slot 0 takes `first`, slot 1 the kind; then by kind (1..22, any other: the
/// comp_stats.CPP 7104 box and 0): the bytes sign-extended, the words of counts sign-extended and the year words
/// zero-extended, the dwords as they are; the match records through `write_match_record_to_news`, the run dates
/// through `write_date_span_record_to_news` and the holders through `write_staff_name_ref_to_news` (each moves the
/// slot on); the average rating as `_ftol`(rating × 100). Returns 1.
/// Original: `comp_stats_write_record_to_news` (0x4ad9c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_write_record_to_news(first: u32, rec: usize, kind: u32, news: usize) -> u32 {
    if rec == 0 {
        show_error_box(COMP_STATS_CPP, 0x1ab7);
        return 0;
    }
    if news == 0 {
        show_error_box(COMP_STATS_CPP, 0x1abf);
        return 0;
    }
    let mut slot: u8 = 1;
    exe::newsitem_set_slot(news, 0, first);
    let kind = kind as u8 as i8;
    let put = |slot: &mut u8, v: u32| {
        let s = *slot;
        *slot = s.wrapping_add(1);
        exe::newsitem_set_slot(news, s as i8, v);
    };
    put(&mut slot, kind as i32 as u32);
    let b = |o: usize| read_i8(rec + o) as i32 as u32;
    let sw = |o: usize| read_i16(rec + o) as i32 as u32;
    let zw = |o: usize| read_u16(rec + o) as u32;
    let d = |o: usize| read_u32(rec + o);
    let span: NewsDateSpanWriter = core::mem::transmute(0x7e5420usize);
    let matched = |slot: &mut u8, o: usize| { exe::write_match_record_to_news(rec + o, news, slot as *mut u8); };
    let named = |slot: &mut u8, o: usize| exe::write_staff_name_ref_to_news(rec + o, news, slot as *mut u8);
    match kind {
        1 => {
            put(&mut slot, b(0));
            put(&mut slot, d(1));
        }
        2 | 3 | 4 => {
            let o = [5usize, 0xd, 0x15][(kind - 2) as usize];
            put(&mut slot, sw(o));
            put(&mut slot, d(o + 2));
            put(&mut slot, zw(o + 6));
        }
        5 => {
            put(&mut slot, b(0x1d));
            put(&mut slot, b(0x1e));
            put(&mut slot, d(0x1f));
            put(&mut slot, zw(0x23));
        }
        6 => {
            put(&mut slot, d(0x25));
            matched(&mut slot, 0x29);
        }
        7 => {
            put(&mut slot, d(0x48));
            matched(&mut slot, 0x4c);
        }
        8 => {
            put(&mut slot, d(0x6b));
            put(&mut slot, d(0x6f));
            put(&mut slot, zw(0x73));
        }
        9 => matched(&mut slot, 0x75),
        10 => matched(&mut slot, 0x94),
        11..=14 => {
            let o = [0xb3usize, 0xcb, 0xe3, 0xfb][(kind - 11) as usize];
            span(rec + o, news, &mut slot);
            put(&mut slot, d(o + 0x14));
        }
        15 | 17 | 19 => {
            let o = match kind { 15 => 0x113usize, 17 => 0x16a, _ => 0x1ac };
            put(&mut slot, b(o));
            named(&mut slot, o + 1);
            put(&mut slot, d(o + 0x19));
            put(&mut slot, zw(o + 0x1d));
        }
        16 => {
            put(&mut slot, b(0x132));
            named(&mut slot, 0x133);
            matched(&mut slot, 0x14b);
        }
        18 => {
            let rating = ftol(read_f32(rec + 0x189) as f64 * read_f32(0x968960) as f64);
            put(&mut slot, rating as u32);
            named(&mut slot, 0x18d);
            put(&mut slot, b(0x1a5));
            put(&mut slot, d(0x1a6));
            put(&mut slot, zw(0x1aa));
        }
        20 => {
            put(&mut slot, b(0x1cb));
            put(&mut slot, b(0x1cc));
            named(&mut slot, 0x1cd);
            put(&mut slot, d(0x1e5));
            put(&mut slot, zw(0x1e9));
        }
        21 | 22 => {
            let o = if kind == 21 { 0x1ebusize } else { 0x22a };
            put(&mut slot, sw(o));
            put(&mut slot, zw(o + 2));
            named(&mut slot, o + 8);
            matched(&mut slot, o + 0x20);
        }
        _ => {
            show_error_box(COMP_STATS_CPP, 0x1bc0);
            return 0;
        }
    }
    1
}

/// Comparer for `comp_stats_write_record_to_news`: side by side over the news item (0xdf bytes) and the return.
pub(crate) unsafe extern "C" fn compare_comp_stats_write_record_to_news(first: u32, rec: usize, kind: u32, news: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, usize) -> u32>(%%I1%%);
    if rec == 0 || news == 0 || !(1..=22).contains(&(kind as u8 as i8)) {
        return o(first, rec, kind, news);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(news, NEWS_NEWS_ITEM_SIZE)], || o(first, rec, kind, news),
        || comp_stats_write_record_to_news(first, rec, kind, news));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_write_record_to_news(kind {}): differences in {:?}", kind as u8 as i8, first_differences(&mo, &mr)));
    ro
}
