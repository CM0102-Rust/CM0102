// ---------- batch %%LEVA%%: the club records after a match (club_records.cpp) ----------

/// Offsets in a club record (season or all-time, 0x38b bytes) used by `update_records_for_match`, beyond the ones in
/// `club_record`. Each value record is followed by its holder (`record_holder`, 0x18 bytes) and/or its match
/// (`club_match_record`, 0x1f bytes, `club_match_record::FIXTURE_ID` −1 = no record); "−1" marks an empty value.
pub(crate) mod club_match_records {
    pub(crate) const HIGHEST_ATTENDANCE: usize = 0xe; // i32, then the match at +0x12
    pub(crate) const HIGHEST_GATE: usize = 0x31; // i32 (gate receipts), then the match at +0x35
    pub(crate) const LOWEST_ATTENDANCE: usize = 0x54; // i32, then the match at +0x58
    pub(crate) const AVERAGE_ATTENDANCE: usize = 0x77; // i32: the season's home attendance / home games
    pub(crate) const AVERAGE_ATTENDANCE_YEAR: usize = 0x7b; // u16: 0 in the season record [inference]
    pub(crate) const BIGGEST_WIN: usize = 0x7d; // match
    pub(crate) const BIGGEST_LEAGUE_WIN: usize = 0x9c; // match
    pub(crate) const BIGGEST_DEFEAT: usize = 0xbb; // match
    pub(crate) const HIGHEST_SCORING: usize = 0xda; // match
    pub(crate) const HIGHEST_SCORING_LEAGUE: usize = 0xf9; // match
    pub(crate) const WON_IN_ROW: usize = 0x118; // run_record (i32 games, start and end dates)
    pub(crate) const LOST_IN_ROW: usize = 0x12c; // run_record
    pub(crate) const UNBEATEN: usize = 0x140; // run_record
    pub(crate) const WINLESS: usize = 0x154; // run_record
    pub(crate) const TOP_SCORER: usize = 0x168; // i8 goals, holder +0x169, year (u16) +0x181
    pub(crate) const TOP_LEAGUE_SCORER: usize = 0x183; // i8 goals, holder +0x184, year +0x19c
    pub(crate) const GOALS_IN_MATCH: usize = 0x19e; // i8 goals, holder +0x19f, match +0x1b7
    pub(crate) const ASSISTS: usize = 0x1d6; // i8, holder +0x1d7, year +0x1ef
    pub(crate) const MAN_OF_MATCH: usize = 0x210; // i8, holder +0x211, year +0x229
    pub(crate) const DISCIPLINE: usize = 0x22b; // i8 yellow cards, i8 red cards +0x22c, holder +0x22d, year +0x245
    pub(crate) const YOUNGEST: usize = 0x27b; // the holder's birth date, holder +0x283, match +0x29b
    pub(crate) const OLDEST: usize = 0x2ba; // the holder's birth date, holder +0x2c2, match +0x2da
    /// The holder (`record_holder`) after a one-byte value; the year word after the holder.
    pub(crate) const HOLDER: usize = 1;
    pub(crate) const YEAR: usize = 0x19;
    /// After a birth date: the holder and the match.
    pub(crate) const AGE_HOLDER: usize = 8;
    pub(crate) const AGE_MATCH: usize = 0x20;
}

/// Offsets in a club's 0x6a-byte entry (`club_records_fields::CLUB_ENTRIES`) used by `update_records_for_match`.
pub(crate) mod club_entry_runs {
    pub(crate) const HOME_GAMES: usize = 0x11; // i8: home games this season
    pub(crate) const HOME_GAMES_TOTAL: usize = 0x12; // i16: home games counted for the attendance news [inference: never reset]
    pub(crate) const HOME_ATTENDANCE: usize = 0x14; // i32: the season's home attendance
    pub(crate) const WON_RUN: usize = 0x18; // run_record: the current run of wins
    pub(crate) const LOST_RUN: usize = 0x2c; // run_record: of defeats
    pub(crate) const UNBEATEN_RUN: usize = 0x40; // run_record: without losing
    pub(crate) const WINLESS_RUN: usize = 0x54; // run_record: without winning
}

/// The record types (club_records_get_record_type_label 0x4490b0) that `update_records_for_match` updates.
pub(crate) mod club_record_kind {
    pub(crate) const HIGHEST_ATTENDANCE: u8 = 3;
    pub(crate) const HIGHEST_GATE: u8 = 4;
    pub(crate) const LOWEST_ATTENDANCE: u8 = 5;
    pub(crate) const BIGGEST_WIN: u8 = 7;
    pub(crate) const BIGGEST_LEAGUE_WIN: u8 = 8;
    pub(crate) const BIGGEST_DEFEAT: u8 = 9;
    pub(crate) const HIGHEST_SCORING: u8 = 0xa;
    pub(crate) const HIGHEST_SCORING_LEAGUE: u8 = 0xb;
    pub(crate) const WON_IN_ROW: u8 = 0xc;
    pub(crate) const LOST_IN_ROW: u8 = 0xd;
    pub(crate) const UNBEATEN: u8 = 0xe;
    pub(crate) const WINLESS: u8 = 0xf;
    pub(crate) const TOP_SCORER: u8 = 0x10;
    pub(crate) const TOP_LEAGUE_SCORER: u8 = 0x11;
    pub(crate) const GOALS_IN_MATCH: u8 = 0x12;
    pub(crate) const ASSISTS: u8 = 0x13;
    pub(crate) const MAN_OF_MATCH: u8 = 0x15;
    pub(crate) const DISCIPLINE: u8 = 0x16;
    pub(crate) const LEAGUE_APPEARANCES: u8 = 0x17;
    pub(crate) const LEAGUE_GOALS: u8 = 0x18;
    pub(crate) const YOUNGEST: u8 = 0x19;
    pub(crate) const OLDEST: u8 = 0x1a;
    pub(crate) const INTERNATIONAL_CAPS: u8 = 0x1b;
}

/// The comparer's dry run of `update_records_for_match`: `Some((n, queued))` makes the copy look only at the first `n`
/// items of the real news queue and put its own in `queued` instead of the queue.
pub(crate) static MATCH_RECORDS_NEWS_DRY_RUN: std::sync::Mutex<Option<(i32, Vec<(u32, u8)>)>> = std::sync::Mutex::new(None);

/// Queues the record news of `club` for the last of `kinds`, unless a news item of one of `kinds` (looked up in that
/// order, each over the whole queue) is already queued for the club. Returns the kind found, or `None` when it queued.
pub(crate) unsafe fn queue_match_record_news(this: usize, club: Club, kinds: &[u8]) -> Option<u8> {
    let id = read_u32(club.addr());
    let mut dry = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap();
    let real = match &*dry {
        Some((n, _)) => *n,
        None => read_i32(this + club_records_fields::NEWS_QUEUE_COUNT),
    };
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    for &kind in kinds {
        let in_queue = (0..real.max(0) as usize).any(|k| read_u32(queue + 6 * k) == id && read_u8(queue + 6 * k + record_news_item::RECORD_TYPE) == kind);
        if in_queue || dry.as_ref().map_or(false, |(_, v)| v.contains(&(id, kind))) {
            return Some(kind);
        }
    }
    let kind = kinds[kinds.len() - 1];
    match &mut *dry {
        Some((_, v)) => v.push((id, kind)),
        None => {
            drop(dry);
            exe::club_records_queue_record_news(this, id, kind as u32);
        }
    }
    None
}

/// The attendance records of a home match (types 3, 4 and 5): the value `value` of the match beats the season record at
/// `at` (`beats(value, record)`, or no record: −1) → the season record takes the value and the match (`match_record_assign`
/// of the club's view of the match); if it also beats the all-time record: the news when the club has played 50 or more
/// home games (`club_entry_runs::HOME_GAMES_TOTAL`), and the all-time record takes the value and the match.
unsafe fn attendance_record(this: usize, club: Club, entry: usize, records: (usize, usize), view: usize, kind: u8, at: usize, value: i32, beats: fn(i32, i32) -> bool) {
    let (season, all_time) = records;
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 || read_u8(view + club_match_record::VENUE) != 1 || value <= 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && !beats(value, s) {
        return;
    }
    write_i32(season + at, value);
    match_record_assign(season + at + 4, view);
    let a = read_i32(all_time + at);
    if a != -1 && !beats(value, a) {
        return;
    }
    covered!("gra_15 attendance record ever");
    if read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL) >= 50 {
        queue_match_record_news(this, club, &[kind]);
    }
    write_i32(all_time + at, value);
    match_record_assign(all_time + at + 4, view);
}

/// A match record (types 7 to 0xb): the club's view of the match beats the season record at `at` (`better(view,
/// record)`, or no record: fixture id −1) → it is copied there; if it also beats the all-time record: with `news`, the
/// news (skipped when one of `kinds` is queued), and it is copied to the all-time record.
unsafe fn result_record(this: usize, club: Club, records: (usize, usize), view: usize, kinds: &[u8], at: usize, better: unsafe extern "C" fn(usize, usize) -> u32, news: bool) {
    let (season, all_time) = records;
    if read_i32(season + at + club_match_record::FIXTURE_ID) != -1 && better(view, season + at) == 0 {
        return;
    }
    match_record_assign(season + at, view);
    if read_i32(all_time + at + club_match_record::FIXTURE_ID) != -1 && better(view, all_time + at) == 0 {
        return;
    }
    if news {
        queue_match_record_news(this, club, kinds);
    }
    match_record_assign(all_time + at, view);
}

/// One more game in the run at `run` of the club's entry: the count goes up (the first game sets the start date) and
/// the end date is today ([`G_CURRENT_DATE`]). If the run beats the season record at `at` (type `kind`, when it
/// applies; −1 = no record), the season record takes the count and both dates; if it also beats the all-time record:
/// the news when the run is at least `news_from` games long and did not start on the all-time record's start date (the
/// same run already recorded), and the all-time record takes the count and the dates.
unsafe fn run_record(this: usize, club: Club, entry: usize, records: (usize, usize), kind: u8, run: usize, at: usize, news_from: i32) {
    let (season, all_time) = records;
    let n = read_i32(entry + run).wrapping_add(1);
    write_i32(entry + run, n);
    if n == 1 {
        copy_record_date(entry + run + run_record::START_DATE, G_CURRENT_DATE);
    }
    copy_record_date(entry + run + run_record::END_DATE, G_CURRENT_DATE);
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && read_i32(entry + run) <= s {
        return;
    }
    write_i32(season + at, read_i32(entry + run));
    copy_record_date(season + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(season + at + run_record::END_DATE, entry + run + run_record::END_DATE);
    let a = read_i32(all_time + at);
    if a != -1 && read_i32(entry + run) <= a {
        return;
    }
    if read_i32(entry + run) >= news_from {
        let (start, recorded) = (entry + run + run_record::START_DATE, all_time + at + run_record::START_DATE);
        if read_u16(recorded + 2) != read_u16(start + 2) || read_u16(recorded) != read_u16(start) {
            covered!("gra_15 run record ever");
            queue_match_record_news(this, club, &[kind]);
        }
    }
    write_i32(all_time + at, read_i32(entry + run));
    copy_record_date(all_time + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(all_time + at + run_record::END_DATE, entry + run + run_record::END_DATE);
}

/// The club's records of the match (the part of `update_records_for_match` before the players).
///
/// One more game this season (`club_record_entry::SEASON_GAMES`); the club's view of the match
/// (`exe::fill_match_record_from_fixture` into `view`). At home (`club_match_record::VENUE` 1): one more home game
/// (`club_entry_runs::HOME_GAMES`, `club_entry_runs::HOME_GAMES_TOTAL`), the attendance (`match_record::ATTENDANCE`)
/// added to the season's (`club_entry_runs::HOME_ATTENDANCE`) and the season average
/// (`club_match_records::AVERAGE_ATTENDANCE`) = total / home games, its year word 0. Then, each when its type applies
/// to the club (`exe::club_records_record_type_applies`, period 1), through `attendance_record`, `result_record` and
/// `run_record`:
/// - highest attendance, highest gate receipts (`match_record::GATE_RECEIPTS`) and lowest attendance, at home and with
///   a value above 0;
/// - biggest win (won, `match_record_is_bigger_win`; news by 6 goals or more), biggest league win (the same in a league
///   division, `exe::comp_util_is_league_division`; not again when the biggest win's news is queued), biggest defeat
///   (lost, `match_record_is_bigger_defeat`; 6 or more), highest scoring game (goals, `match_record_has_more_goals`;
///   news from 8 goals) and highest scoring league game;
/// - the runs: a win adds to the run of wins (news from 6) and ends the runs of defeats and without winning; a defeat
///   adds to the defeats (6) and ends the wins and the unbeaten run; a win or a draw adds to the unbeaten run (12) and
///   ends the defeats; a draw or a defeat adds to the run without winning (12) and ends the wins.
/// Goals are the club's view (`club_match_record::GOALS_FOR`/`GOALS_AGAINST`, signed bytes, the extra-time score if
/// any).
unsafe fn update_club_match_records(this: usize, f: Fixture, result: usize, club: Club, entry: usize, records: (usize, usize), view: usize) {
    use club_match_records as r;
    use club_record_kind as k;
    let (season, _) = records;
    write_u8(entry + club_record_entry::SEASON_GAMES, read_u8(entry + club_record_entry::SEASON_GAMES).wrapping_add(1));
    exe::fill_match_record_from_fixture(f, club, view);
    let attendance = read_i32(result + match_record::ATTENDANCE);
    if read_u8(view + club_match_record::VENUE) == 1 {
        covered!("gra_15 home match");
        let home = read_i8(entry + club_entry_runs::HOME_GAMES).wrapping_add(1);
        write_i16(entry + club_entry_runs::HOME_GAMES_TOTAL, read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL).wrapping_add(1));
        write_i8(entry + club_entry_runs::HOME_GAMES, home);
        let total = read_i32(entry + club_entry_runs::HOME_ATTENDANCE).wrapping_add(attendance);
        write_i32(entry + club_entry_runs::HOME_ATTENDANCE, total);
        // ponytail: the original divides by zero (a fault) after 256 home games in a season; the copy writes 0
        write_i32(season + r::AVERAGE_ATTENDANCE, if home != 0 { total.wrapping_div(home as i32) } else { 0 });
        write_u16(season + r::AVERAGE_ATTENDANCE_YEAR, 0);
    }
    attendance_record(this, club, entry, records, view, k::HIGHEST_ATTENDANCE, r::HIGHEST_ATTENDANCE, attendance, |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::HIGHEST_GATE, r::HIGHEST_GATE, read_i32(result + match_record::GATE_RECEIPTS), |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::LOWEST_ATTENDANCE, r::LOWEST_ATTENDANCE, attendance, |v, rec| v < rec);
    let goals = || (read_i8(view + club_match_record::GOALS_FOR) as i32, read_i8(view + club_match_record::GOALS_AGAINST) as i32);
    let applies = |kind: u8| exe::club_records_record_type_applies(kind as u32, club, 1) != 0;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let (gf, ga) = goals();
    if applies(k::BIGGEST_WIN) && gf > ga {
        result_record(this, club, records, view, &[k::BIGGEST_WIN], r::BIGGEST_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_LEAGUE_WIN) && league() && gf > ga {
        covered!("gra_15 league win");
        result_record(this, club, records, view, &[k::BIGGEST_WIN, k::BIGGEST_LEAGUE_WIN], r::BIGGEST_LEAGUE_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_DEFEAT) && gf < ga {
        result_record(this, club, records, view, &[k::BIGGEST_DEFEAT], r::BIGGEST_DEFEAT, match_record_is_bigger_defeat, ga - gf >= 6);
    }
    if applies(k::HIGHEST_SCORING) && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING], r::HIGHEST_SCORING, match_record_has_more_goals, ga + gf >= 8);
    }
    if applies(k::HIGHEST_SCORING_LEAGUE) && league() && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING, k::HIGHEST_SCORING_LEAGUE], r::HIGHEST_SCORING_LEAGUE, match_record_has_more_goals, ga + gf >= 8);
    }
    use club_entry_runs as e;
    if gf > ga {
        covered!("gra_15 won");
        run_record(this, club, entry, records, k::WON_IN_ROW, e::WON_RUN, r::WON_IN_ROW, 6);
        write_i32(entry + e::LOST_RUN, 0);
        write_i32(entry + e::WINLESS_RUN, 0);
    } else if gf < ga {
        run_record(this, club, entry, records, k::LOST_IN_ROW, e::LOST_RUN, r::LOST_IN_ROW, 6);
        write_i32(entry + e::WON_RUN, 0);
        write_i32(entry + e::UNBEATEN_RUN, 0);
    }
    if gf >= ga {
        run_record(this, club, entry, records, k::UNBEATEN, e::UNBEATEN_RUN, r::UNBEATEN, 12);
        write_i32(entry + e::LOST_RUN, 0);
    }
    if gf <= ga {
        run_record(this, club, entry, records, k::WINLESS, e::WINLESS_RUN, r::WINLESS, 12);
        write_i32(entry + e::WON_RUN, 0);
    }
}

/// What a player record needs: the records object, the club, its entry, the season and all-time records, the person,
/// the season year (`exe::key_nation_get_club_season_year`) and the club's view of the match.
#[derive(Clone, Copy)]
struct PlayerRecordsCtx {
    this: usize,
    club: Club,
    entry: usize,
    season: usize,
    all_time: usize,
    person: Staff,
    year: u16,
    view: usize,
}

impl PlayerRecordsCtx {
    unsafe fn applies(self, kind: u8) -> bool {
        exe::club_records_record_type_applies(kind as u32, self.club, 1) != 0
    }

    /// The club has 3 or more seasons recorded (`club_record_entry::SEASONS`).
    unsafe fn seasoned(self) -> bool {
        read_i16(self.entry + club_record_entry::SEASONS) >= 3
    }

    /// The all-time holder of the record at `at` is someone else or held it in another season.
    unsafe fn new_holder(self, at: usize) -> bool {
        use club_match_records as r;
        read_u32(self.all_time + at + r::HOLDER + record_holder::STAFF_ID) != read_u32(self.person.addr()) || read_u16(self.all_time + at + r::YEAR) != self.year
    }

    /// A one-byte season tally (`at`, −1 = none) beaten by `value` (signed comparison).
    unsafe fn byte_beats(rec: usize, value: u8) -> bool {
        read_i8(rec) == -1 || value as i32 > read_i8(rec) as i32
    }

    /// The byte tally record at `at` of type `kind` with `value`: the season record takes it, the holder and year 0;
    /// beating the all-time one too: `news()` decides the news (the extra points it returns are added to `points`) and
    /// the all-time record takes the value, the holder and the season year.
    unsafe fn tally(self, kind: u8, at: usize, value: u8, points: &mut i32, news: impl FnOnce() -> i32) {
        use club_match_records as r;
        if !self.applies(kind) || !Self::byte_beats(self.season + at, value) {
            return;
        }
        write_u8(self.season + at, value);
        exe::fill_staff_name_ref(self.person, self.season + at + r::HOLDER);
        write_u16(self.season + at + r::YEAR, 0);
        if !Self::byte_beats(self.all_time + at, value) {
            return;
        }
        *points = points.wrapping_add(news());
        write_u8(self.all_time + at, value);
        exe::fill_staff_name_ref(self.person, self.all_time + at + r::HOLDER);
        write_u16(self.all_time + at + r::YEAR, self.year);
    }

    /// The age (`exe::date_get_difference`: years word +2, days word +0) at `date` of someone born on `birth`.
    unsafe fn age(date: usize, birth: usize) -> (u16, i16) {
        let mut out = [0u8; 8];
        let p = exe::date_get_difference(date, out.as_mut_ptr(), read_u32(birth), read_u32(birth + 4));
        (read_u16(p + 2), read_i16(p))
    }

    /// The person's age today ([`G_CURRENT_DATE`]).
    unsafe fn age_today(self) -> (u16, i16) {
        Self::age(G_CURRENT_DATE, self.person.addr() + staff::DATE_OF_BIRTH)
    }

    /// The age record at `at` (youngest: `younger`, oldest: not) beaten by the person: the record's holder at its
    /// match is older (younger) in years, or in days within the same year; no record also beats it.
    unsafe fn age_beats(self, at: usize, younger: bool) -> bool {
        use club_match_records as r;
        if read_i32(at + r::AGE_MATCH + club_match_record::FIXTURE_ID) == -1 {
            return true;
        }
        let (ry, rd) = Self::age(at + r::AGE_MATCH, at);
        let (cy, cd) = self.age_today();
        if younger {
            if ry != cy { ry > cy } else { rd > cd }
        } else if ry != cy {
            ry < cy
        } else {
            rd < cd
        }
    }

    /// The age record: the birth date, the holder and the match, in the record at `at`.
    unsafe fn take_age_record(self, at: usize) {
        use club_match_records as r;
        copy_record_date(at, self.person.addr() + staff::DATE_OF_BIRTH);
        exe::fill_staff_name_ref(self.person, at + r::AGE_HOLDER);
        match_record_assign(at + r::AGE_MATCH, self.view);
    }
}

/// The youngest and oldest player records (types 0x19 and 0x1a, `PlayerRecordsCtx::age_beats`). The all-time news:
/// with 3 or more seasons, a youngest player under 20 or an oldest over 35 (in years today) who is not the all-time
/// holder; an oldest player's news multiplies the points by 1.25 (the points as a 16-bit value, truncated).
unsafe fn player_age_records(c: PlayerRecordsCtx, points: &mut i32) {
    use club_match_records as r;
    use club_record_kind as k;
    for (kind, at, younger) in [(k::YOUNGEST, r::YOUNGEST, true), (k::OLDEST, r::OLDEST, false)] {
        if !c.applies(kind) || !c.age_beats(c.season + at, younger) {
            continue;
        }
        c.take_age_record(c.season + at);
        if !c.age_beats(c.all_time + at, younger) {
            continue;
        }
        if c.seasoned() {
            let (years, _) = c.age_today();
            let notable = if younger { years < 20 } else { years > 35 };
            if notable && read_u32(c.all_time + at + r::AGE_HOLDER + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
                queue_match_record_news(c.this, c.club, &[kind]);
                if !younger {
                    covered!("gra_15 oldest player ever");
                    *points = ftol((*points as i16) as f64 * read_f64(rdata::F64_1_25));
                }
            }
        }
        c.take_age_record(c.all_time + at);
    }
}

/// The 16-bit league tallies (types 0x17 appearances and 0x18 goals, `club_record::LEAGUE_APPEARANCES` /
/// `club_record::LEAGUE_GOALS`, the holder after the value): the player's league count at the club
/// (`get_club_league_stat`, field 1 or 2) beats the season record (−1 none, signed) → the season record takes it and
/// the holder; beating the all-time one too: with 3 or more seasons and another holder, the news and 100 points; the
/// all-time record takes the count and the holder.
unsafe fn league_tally(c: PlayerRecordsCtx, kind: u8, at: usize, field: i8, pdata: usize, points: &mut i32) {
    let value = ftol(get_club_league_stat(read_usize(G_PLAYER_STATS), pdata, field) as f64) as i16;
    if !c.applies(kind) {
        return;
    }
    let beats = |rec: usize| read_i16(rec) == -1 || value > read_i16(rec);
    if !beats(c.season + at) {
        return;
    }
    write_i16(c.season + at, value);
    exe::fill_staff_name_ref(c.person, c.season + at + 2);
    if !beats(c.all_time + at) {
        return;
    }
    if c.seasoned() && read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
        covered!("gra_15 league tally ever");
        queue_match_record_news(c.this, c.club, &[kind]);
        *points = points.wrapping_add(100);
    }
    write_i16(c.all_time + at, value);
    exe::fill_staff_name_ref(c.person, c.all_time + at + 2);
}

/// The player's records of the match (one lineup row of `update_records_for_match`); returns the points that decide
/// the liking for him (`favoured_by_points`), or `None` when it stops before them.
///
/// Most capped player (type 0x1b, all-time only): at a national side (`exe::database_is_national_team`) that is not a
/// second national side (`exe::club_is_national_team`), in a match with the home side mark (`fixture::HOME_SIDE_MARK`),
/// the person's caps (`staff::INT_APPS`) above the record (`club_record::INTERNATIONAL_CAPS`, −1 none): the news when
/// he is not the holder, and the record takes the caps and the holder. A friendly stops here. The player's statistics
/// record of the match category (5 at a national side, 6 otherwise, `exe::playerstats_get_category_record`; missing:
/// the "No player stats" box, as in the original) give the season counts (`exe::player_stats_get_record_value`, weight
/// 6.5). The points: a rating (`lineup_row::RATING`) above 7 gives (rating − 6)^4; with goals (`lineup_row::GOALS`):
/// (goals + 1)^4, the top scorer (0x10, the season's goals; news with 3+ seasons, more than 3 goals and a new holder or
/// season: +100), in a league division the top league scorer (0x11, `exe::playerstats_get_stat` category 2; news with 3+
/// seasons and a new holder or season: +100, but nothing when the top scorer's news is queued) and the most goals in a
/// match (0x12, the match's goals and the match; news from 4 goals: +50); with assists: (assists + 1)^3 and the most
/// assists (0x13; news with 3+ seasons and a new holder or season: +50); the man of the match
/// (`match_record::MAN_OF_THE_MATCH_ID`): +30 and the most man of the match awards (0x15; news when the season count is
/// above 3, with 3+ seasons and a new holder or season: +50); a card in the match: the worst discipline (0x16, red cards
/// × 3 + yellow cards; news with 3+ seasons, 2+ yellows, 2+ reds and a new holder or season); in a league division the
/// league appearances and goals (`league_tally`); the youngest and oldest players (`player_age_records`).
unsafe fn update_player_match_records(c: PlayerRecordsCtx, f: Fixture, result: usize, row: usize, pdata: usize) -> Option<i32> {
    use club_match_records as r;
    use club_record_kind as k;
    let (club, person) = (c.club, c.person);
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && f.home_side_mark() != 0 && c.applies(k::INTERNATIONAL_CAPS) {
        let caps = person.int_apps() as i16;
        let rec = read_i16(c.all_time + club_record::INTERNATIONAL_CAPS);
        if rec == -1 || caps > rec {
            covered!("gra_15 most capped");
            if read_u32(c.all_time + club_record::INTERNATIONAL_CAPS + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) {
                queue_match_record_news(c.this, club, &[k::INTERNATIONAL_CAPS]);
            }
            write_i16(c.all_time + club_record::INTERNATIONAL_CAPS, caps);
            exe::fill_staff_name_ref(person, c.all_time + club_record::INTERNATIONAL_CAPS + 2);
        }
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) != 0 {
        return None;
    }
    let stats = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), pdata, if exe::database_is_national_team(club) != 0 { 5 } else { 6 });
    if stats == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let comp = f.competition().addr();
        sprintf(0xde1f64, 0x98b6ec, club.addr() + 4, if comp == 0 { 0x98b714 } else { comp + 4 }); // "No player stats for %s playing in %s"
        show_error_at(0x98b6a0, 0x31a);
    }
    let count = |which: u32| ftol(exe::player_stats_get_record_value(stats, which, 6.5)) as u8;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let mut points = 0i32;
    covered!("gra_15 player points");
    let rating = read_u8(row + lineup_row::RATING);
    if rating > 7 {
        points = ftol(game_pow((rating as i32 - 6) as f64, read_f64(rdata::F64_4)));
    }
    let goals = read_u8(row + lineup_row::GOALS);
    if goals != 0 {
        points = points.wrapping_add(ftol(game_pow((goals as i32 + 1) as f64, read_f64(rdata::F64_4))));
        let scored = count(2);
        c.tally(k::TOP_SCORER, r::TOP_SCORER, scored, &mut points, || {
            if c.seasoned() && scored > 3 && c.new_holder(r::TOP_SCORER) {
                covered!("gra_15 top scorer ever");
                queue_match_record_news(c.this, club, &[k::TOP_SCORER]);
                100
            } else {
                0
            }
        });
        if league() {
            let scored = ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), pdata, 2, 2, 6.5)) as u8;
            c.tally(k::TOP_LEAGUE_SCORER, r::TOP_LEAGUE_SCORER, scored, &mut points, || {
                if c.seasoned() && c.new_holder(r::TOP_LEAGUE_SCORER) {
                    match queue_match_record_news(c.this, club, &[k::TOP_SCORER, k::TOP_LEAGUE_SCORER]) {
                        Some(k::TOP_SCORER) => 0,
                        _ => 100,
                    }
                } else {
                    0
                }
            });
        }
        if c.applies(k::GOALS_IN_MATCH) && PlayerRecordsCtx::byte_beats(c.season + r::GOALS_IN_MATCH, goals) {
            write_u8(c.season + r::GOALS_IN_MATCH, goals);
            exe::fill_staff_name_ref(person, c.season + r::GOALS_IN_MATCH + r::HOLDER);
            match_record_assign(c.season + r::GOALS_IN_MATCH + 0x19, c.view);
            if PlayerRecordsCtx::byte_beats(c.all_time + r::GOALS_IN_MATCH, goals) {
                if goals >= 4 {
                    covered!("gra_15 goals in a match ever");
                    queue_match_record_news(c.this, club, &[k::GOALS_IN_MATCH]);
                    points = points.wrapping_add(50);
                }
                write_u8(c.all_time + r::GOALS_IN_MATCH, goals);
                exe::fill_staff_name_ref(person, c.all_time + r::GOALS_IN_MATCH + r::HOLDER);
                match_record_assign(c.all_time + r::GOALS_IN_MATCH + 0x19, c.view);
            }
        }
    }
    let assists = read_u8(row + lineup_row::ASSISTS);
    if assists != 0 {
        points = points.wrapping_add(ftol(game_pow((assists as i32 + 1) as f64, read_f64(rdata::F64_3))));
        let made = count(5);
        c.tally(k::ASSISTS, r::ASSISTS, made, &mut points, || {
            if c.seasoned() && c.new_holder(r::ASSISTS) {
                queue_match_record_news(c.this, club, &[k::ASSISTS]);
                50
            } else {
                0
            }
        });
    }
    if read_u32(row + lineup_row::PLAYER_ID) == read_u32(result + match_record::MAN_OF_THE_MATCH_ID) {
        covered!("gra_15 man of the match");
        let awards = count(0xc);
        points = points.wrapping_add(30);
        c.tally(k::MAN_OF_MATCH, r::MAN_OF_MATCH, awards, &mut points, || {
            if read_i8(c.season + r::MAN_OF_MATCH) > 3 && c.seasoned() && c.new_holder(r::MAN_OF_MATCH) {
                queue_match_record_news(c.this, club, &[k::MAN_OF_MATCH]);
                50
            } else {
                0
            }
        });
    }
    if read_u8(row + lineup_row::YELLOW_CARDS) != 0 || read_u8(row + lineup_row::RED_CARD) != 0 {
        let (yellow, red) = (count(10), count(11));
        let score = red as i32 * 3 + yellow as i32;
        let beats = |rec: usize| read_i8(rec) == -1 || score > read_i8(rec + 1) as i32 * 3 + read_i8(rec) as i32;
        let at = r::DISCIPLINE;
        if c.applies(k::DISCIPLINE) && beats(c.season + at) {
            covered!("gra_15 discipline");
            write_u8(c.season + at, yellow);
            write_u8(c.season + at + 1, red);
            exe::fill_staff_name_ref(person, c.season + at + 2);
            write_u16(c.season + at + 0x1a, 0);
            if beats(c.all_time + at) {
                let other = read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) || read_u16(c.all_time + at + 0x1a) != c.year;
                if c.seasoned() && yellow >= 2 && red >= 2 && other {
                    queue_match_record_news(c.this, club, &[k::DISCIPLINE]);
                }
                write_u8(c.all_time + at, yellow);
                write_u8(c.all_time + at + 1, red);
                exe::fill_staff_name_ref(person, c.all_time + at + 2);
                write_u16(c.all_time + at + 0x1a, c.year);
            }
        }
    }
    if league() {
        league_tally(c, k::LEAGUE_APPEARANCES, club_record::LEAGUE_APPEARANCES, 1, pdata, &mut points);
        if read_u8(row + lineup_row::GOALS) != 0 {
            league_tally(c, k::LEAGUE_GOALS, club_record::LEAGUE_GOALS, 2, pdata, &mut points);
        }
    }
    player_age_records(c, &mut points);
    Some(points)
}

/// The liking for a player who did well (the end of each lineup row of `update_records_for_match`).
///
/// The points (as a 16-bit value) must be 40 or more, the person in the staff table, the match's reputation `rep` 5 or
/// more and the person without a future transfer (`exe::transfer_manager_get_future_transfer`, when there is a transfer
/// manager). Against a rival (`club::RIVAL1`..3) the points × (rep × 0.08 + 1); in a clash (`clash`,
/// `exe::comp_util_get_league_clash_type` above 0) × (rep × 0.08 + 1) again; otherwise, not against a rival, when rep is
/// below the club's reputation / 500, × rep × 500 / reputation, kept within 0.2 and 1. Then three draws against
/// `utils_random(4000) + 250`: the chairman (`club::CHAIRMAN`) and the manager (`club::MANAGER`) favour the player
/// (`exe::staff_add_favoured_staff`, forced) and the club does (`exe::club_add_favoured_staff`); and each squad member
/// (`club::SQUAD`, 50) who is a player (`staff::PLAYER`) and not him, against `utils_random(7000) + 250`.
unsafe fn favoured_by_points(club: Club, person: Staff, points: i32, rep: i16, rival: bool, clash: bool) {
    let mut p = points;
    if (p as i16) < 40 {
        return;
    }
    if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if rep < 5 {
        return;
    }
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && exe::transfer_manager_get_future_transfer(tm, person, 0) != 0 {
        return;
    }
    covered!("gra_15 liking draws");
    let boost = |p: i32| ftol((rep as f64 * read_f64(rdata::F64_0_08) + read_f64(rdata::F64_1)) * (p as i16) as f64);
    if p as i16 != 0 {
        if rival {
            covered!("gra_15 against a rival");
            p = boost(p);
        }
        if clash {
            p = boost(p);
        } else if !rival {
            let reputation = club.reputation() as i32;
            if (rep as i32) < reputation / 500 {
                covered!("gra_15 small match");
                let share = ((rep as i32 * 500) / reputation) as f64;
                let share = if share < read_f64(rdata::F64_0_2) {
                    read_f64(rdata::F64_0_2)
                } else if share > read_f64(rdata::F64_1) {
                    read_f64(rdata::F64_1)
                } else {
                    share
                };
                p = ftol((p as i16) as f64 * share);
            }
        }
    }
    let s = (p as i16) as i32;
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(read_usize(club.addr() + club::CHAIRMAN), person, 1);
    }
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(read_usize(club.addr() + club::MANAGER), person, 1);
    }
    if s > random(4000) + 250 {
        covered!("gra_15 club favours");
        exe::club_add_favoured_staff(club, person, 1);
    }
    for k in 0..50 {
        let m = read_usize(club.addr() + club::SQUAD + 4 * k);
        if m != 0 && read_usize(m + staff::PLAYER) != 0 && m != person.addr() && s > random(7000) + 250 {
            exe::staff_add_favoured_staff(m, person, 1);
        }
    }
}

/// Updates both clubs' season and all-time records after a match, from attendance and scores to the players', queues
/// the record news and lets the club, its chairman, manager and players favour the players who did well.
///
/// `Club_Records::update_records_for_match(fixture, result)` (SEH frame without a destructor). A null fixture or result:
/// the error boxes (lines 0x10c and 0x11b) and 0 (the original); a reserve or minor competition
/// (`exe::comp_is_reserve_or_minor_comp` of [`G_RESERVE_COMP`]): 0. The match's reputation: 1, or the competition's
/// (`competition::REPUTATION`; × 1.5, truncated, in the round 0x96 without prefix), at least 1. Then for the home side
/// and the away side: rival (the opponent is one of `club::RIVAL1`..3), clash (`exe::comp_util_get_league_clash_type`
/// above 0), the season and all-time records (`club_season_record`/`club_all_time_record`, created when missing) and the
/// club's entry (`club_records_entry`); unless it is a friendly of a club that is not a national side, the season year
/// and the club's records (`update_club_match_records`); then for each of the 20 lineup rows that played
/// (`match_lineup_row`) the player's records (`update_player_match_records`) and the liking (`favoured_by_points`); out
/// of friendlies, the season's best average rating is reset (`club_record::SEASON_RATING` −1.0, recomputed by
/// `update_season_rating_record`). Every news item is queued only when the same club and type is not queued yet
/// (`queue_match_record_news`). Returns 1.
/// Original: `Club_Records::update_records_for_match` (0x444930, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let original = || original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, fixture, result);
    if fixture == 0 {
        return original(); // the box (line 0x10c)
    }
    let f = Fixture(fixture);
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), f.competition()) != 0 {
        return 0;
    }
    if result == 0 {
        return original(); // the box (line 0x11b)
    }
    let mut rep: i16 = 1;
    let comp = f.competition();
    if !comp.is_null() {
        rep = comp.reputation();
        if f.round_prefix() == 0 && f.round_name() == 0x96 {
            covered!("gra_15 round 0x96");
            rep = ftol(rep as f64 * read_f64(rdata::F64_1_5)) as i16;
        }
        if rep < 1 {
            rep = 1;
        }
    }
    let mut view = [0u8; 0x20]; // the club's view of the match (club_match_record), built by the side that plays
    let view = view.as_mut_ptr() as usize;
    exe::date_date_54a110(view);
    for side in 0..2usize {
        let (club, opponent) = if side == 0 { (read_usize(fixture + fixture::HOME_CLUB), read_usize(fixture + fixture::AWAY_CLUB)) } else { (read_usize(fixture + fixture::AWAY_CLUB), read_usize(fixture + fixture::HOME_CLUB)) };
        let club = Club(club);
        let rival = club.rival1() == opponent || club.rival2() == opponent || club.rival3() == opponent;
        let clash = (exe::comp_util_get_league_clash_type(club, opponent) as i8) > 0;
        let season = club_season_record(this, club.addr(), 1);
        let all_time = club_all_time_record(this, club.addr(), 1);
        let entry = club_records_entry(this, club.addr());
        let mut year = 0u16;
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 || exe::database_is_national_team(club) != 0 {
            year = exe::key_nation_get_club_season_year(club);
            update_club_match_records(this, f, result, club, entry, (season, all_time), view);
        }
        for k in 0..20 {
            let Some((row, person, pdata)) = match_lineup_row(result, side, k) else { continue };
            let c = PlayerRecordsCtx { this, club, entry, season, all_time, person: Staff(person), year, view };
            if let Some(points) = update_player_match_records(c, f, result, row, pdata) {
                favoured_by_points(club, Staff(person), points, rep, rival, clash);
            }
        }
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 {
            write_u32(season + club_record::SEASON_RATING, 0xbf800000); // -1.0
        }
    }
    1
}

/// Comparer for `update_records_for_match`.
///
/// Side by side from the same state, after creating the four records in the original's order (home season, home
/// all-time, away season, away all-time: `get_season_record` and `get_all_time_record` allocate) and fetching them again
/// (a new one may move the others). Compared, each by name: the four records (0x38b bytes) and the two club entries
/// (0x6a), the generator (`crate::rng::STATE_REGIONS`), the favoured lists that the liking may change (each club's
/// three favoured and disliked staff, `club::FAV_STAFF1`, and those of its chairman, manager and 50 squad members,
/// `preferences::STAFF_FAVOURITE_STAFF1`), the news queue items the original appended (it may reallocate the queue:
/// the copy runs dry, `MATCH_RECORDS_NEWS_DRY_RUN`, looking only at the items queued before) and the return value. A
/// null fixture or result or a reserve competition: only the original.
pub(crate) unsafe extern "thiscall" fn compare_update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%);
    if fixture == 0 || result == 0 || exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), Fixture(fixture).competition()) != 0 {
        return o(this, fixture, result);
    }
    let clubs = [Club(read_usize(fixture + fixture::HOME_CLUB)), Club(read_usize(fixture + fixture::AWAY_CLUB))];
    for c in clubs {
        club_season_record(this, c.addr(), 1);
        club_all_time_record(this, c.addr(), 1);
    }
    let mut named: Vec<(&str, Vec<(usize, usize)>)> = Vec::new();
    let mut likes = crate::rng::STATE_REGIONS.to_vec();
    for (side, c) in clubs.iter().enumerate() {
        let name = if side == 0 { ["home season", "home all-time", "home entry"] } else { ["away season", "away all-time", "away entry"] };
        named.push((name[0], vec![(club_season_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[1], vec![(club_all_time_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[2], vec![(club_records_entry(this, c.addr()), 0x6a)]));
        likes.push((c.addr() + club::FAV_STAFF1, 24));
        let staff = [read_usize(c.addr() + club::CHAIRMAN), read_usize(c.addr() + club::MANAGER)].into_iter().chain((0..50).map(|k| read_usize(c.addr() + club::SQUAD + 4 * k)));
        for s in staff {
            if s != 0 && Staff(s).staff_preferences() != 0 {
                likes.push((Staff(s).staff_preferences() + preferences::STAFF_FAVOURITE_STAFF1, 24));
            }
        }
    }
    named.push(("generator and favoured lists", likes));
    let regions: Vec<(usize, usize)> = named.iter().flat_map(|(_, r)| r.iter().copied()).collect();
    let before_count = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture, result), || {
        *MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap() = Some((before_count, Vec::new()));
        crate::rng::without_counting(|| update_records_for_match(this, fixture, result))
    });
    let queued_rust = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap().take().map(|(_, v)| v).unwrap_or_default();
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    let queued_orig: Vec<(u32, u8)> = (before_count.max(0)..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT)).map(|k| (read_u32(queue + 6 * k as usize), read_u8(queue + 6 * k as usize + record_news_item::RECORD_TYPE))).collect();
    SCOREBOARD.record(equal && ro == rr && queued_orig == queued_rust, || {
        let flat = |m: &crate::comparison::MemorySnapshot| m.0.iter().flat_map(|(_, v)| v.iter().copied()).collect::<Vec<u8>>();
        let (a, b) = (flat(&mo), flat(&mr));
        let mut k = 0;
        let mut differ = Vec::new();
        for (name, r) in &named {
            let n: usize = r.iter().map(|x| x.1).sum();
            if let Some(off) = (0..n).find(|&i| a.get(k + i) != b.get(k + i)) {
                differ.push(format!("{} (+{:#x}: {:02x?} x {:02x?})", name, off, a.get(k + off), b.get(k + off)));
            }
            k += n;
        }
        format!("update_records_for_match(fixture {:#x}) returned {} x {}, differ in {:?}, news original {:?} rust {:?}", fixture, ro, rr, differ, queued_orig, queued_rust)
    });
    ro
}
