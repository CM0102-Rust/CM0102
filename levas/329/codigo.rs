// ---------- tel43_1: the competition statistics: recording a played match (comp_screens.cpp) ----------

/// Offsets inside a competition records block (the all-time block at `comp_stats_obj::ALL_TIME_RECORDS`, the
/// season's at `comp_stats_obj::SEASON_RECORDS`), by the record kind of `comp_stats_format_record_text`.
#[allow(dead_code)]
pub(crate) mod comp_records {
    /// Kind 2, a team value: i16 points (what `League::add_result` passes), the club id at `VALUE_CLUB`, the year
    /// word at `VALUE_YEAR`.
    pub(crate) const POINTS: usize = 0x5;
    /// Kinds 3 and 4, team values: the goals scored and conceded in the competition.
    pub(crate) const GOALS_FOR: usize = 0xd;
    pub(crate) const GOALS_AGAINST: usize = 0x15;
    pub(crate) const VALUE_CLUB: usize = 0x2;
    pub(crate) const VALUE_YEAR: usize = 0x6;
    /// Kind 5, the team's cards: u8 yellow, +1 red, the club id at `VALUE_CLUB`, the year word at `VALUE_YEAR`.
    pub(crate) const CARDS: usize = 0x1d;
    /// Kinds 6 and 7, attendance records: the attendance (i32), then the match record at `ATTENDANCE_GAME`.
    pub(crate) const HIGHEST_ATTENDANCE: usize = 0x25;
    pub(crate) const LOWEST_ATTENDANCE: usize = 0x48;
    pub(crate) const ATTENDANCE_GAME: usize = 0x4;
    /// Kinds 9 and 10, match records (`club_match_record`, 0x1f bytes).
    pub(crate) const BIGGEST_WIN: usize = 0x75;
    pub(crate) const HIGHEST_SCORING: usize = 0x94;
    /// Kinds 11 to 14, runs: the run (`run_record`: game count and dates) and the club id at `RUN_CLUB`.
    pub(crate) const WINNING_RUN: usize = 0xb3;
    pub(crate) const LOSING_RUN: usize = 0xcb;
    pub(crate) const UNBEATEN_RUN: usize = 0xe3;
    pub(crate) const WINLESS_RUN: usize = 0xfb;
    pub(crate) const RUN_CLUB: usize = 0x14;
    /// Kinds 15, 17 and 19, player tallies: u8 count, the holder (`record_holder`) at `TALLY_HOLDER`, the club id at
    /// `TALLY_CLUB`, the year word at `TALLY_YEAR`.
    pub(crate) const TOP_SCORER: usize = 0x113;
    pub(crate) const ASSISTS: usize = 0x16a;
    pub(crate) const MAN_OF_THE_MATCH: usize = 0x1ac;
    pub(crate) const TALLY_HOLDER: usize = 0x1;
    pub(crate) const TALLY_CLUB: usize = 0x19;
    pub(crate) const TALLY_YEAR: usize = 0x1d;
    /// Kind 16, the most goals in a match: u8 goals, the holder at `TALLY_HOLDER`, the match record at
    /// `GOALS_IN_A_MATCH_GAME`.
    pub(crate) const GOALS_IN_A_MATCH: usize = 0x132;
    pub(crate) const GOALS_IN_A_MATCH_GAME: usize = 0x19;
    /// Kind 20, a player's cards: u8 yellow, +1 red, the holder at `PLAYER_CARDS_HOLDER`, the club id at
    /// `PLAYER_CARDS_CLUB`, the year word at `PLAYER_CARDS_YEAR`.
    pub(crate) const PLAYER_CARDS: usize = 0x1cb;
    pub(crate) const PLAYER_CARDS_HOLDER: usize = 0x2;
    pub(crate) const PLAYER_CARDS_CLUB: usize = 0x1a;
    pub(crate) const PLAYER_CARDS_YEAR: usize = 0x1e;
    /// Kinds 21 and 22, age records (`player_age_record_assign`): the holder's birth date, the holder at
    /// `AGE_HOLDER`, the match record at `AGE_GAME`.
    pub(crate) const YOUNGEST: usize = 0x1eb;
    pub(crate) const OLDEST: usize = 0x22a;
    pub(crate) const AGE_HOLDER: usize = 0x8;
    pub(crate) const AGE_GAME: usize = 0x20;
}

/// The record kinds (the flag of kind k is `comp_stats_record_flag(k)`).
#[allow(dead_code)]
pub(crate) mod comp_record_kind {
    pub(crate) const POINTS: usize = 2;
    pub(crate) const GOALS_FOR: usize = 3;
    pub(crate) const GOALS_AGAINST: usize = 4;
    pub(crate) const CARDS: usize = 5;
    pub(crate) const HIGHEST_ATTENDANCE: usize = 6;
    pub(crate) const LOWEST_ATTENDANCE: usize = 7;
    pub(crate) const BIGGEST_WIN: usize = 9;
    pub(crate) const HIGHEST_SCORING: usize = 10;
    pub(crate) const WINNING_RUN: usize = 11;
    pub(crate) const LOSING_RUN: usize = 12;
    pub(crate) const UNBEATEN_RUN: usize = 13;
    pub(crate) const WINLESS_RUN: usize = 14;
    pub(crate) const TOP_SCORER: usize = 15;
    pub(crate) const GOALS_IN_A_MATCH: usize = 16;
    pub(crate) const ASSISTS: usize = 17;
    pub(crate) const MAN_OF_THE_MATCH: usize = 19;
    pub(crate) const PLAYER_CARDS: usize = 20;
    pub(crate) const YOUNGEST: usize = 21;
    pub(crate) const OLDEST: usize = 22;
}

/// The figures of a club entry (0x76 bytes) that `record_match_result` keeps besides `comp_stats_club`
/// (`comp_stats_get_team_entry_value` reads them).
#[allow(dead_code)]
pub(crate) mod comp_club_figures {
    /// The last five result codes (`match_record_result_code`), newest first: all, home and away matches.
    pub(crate) const FORM: usize = 0xa;
    pub(crate) const HOME_FORM: usize = 0xf;
    pub(crate) const AWAY_FORM: usize = 0x14;
    pub(crate) const FORM_LENGTH: usize = 5;
    /// u8 goals scored and conceded in the competition.
    pub(crate) const GOALS_FOR: usize = 0x69;
    pub(crate) const GOALS_AGAINST: usize = 0x6a;
    /// Five u8 totals of the side block's bytes +3..+7 of each match record [inference: unnamed team counts].
    pub(crate) const TEAM_TOTALS: usize = 0x6b;
    /// u8 yellow and red cards in the competition.
    pub(crate) const YELLOW_CARDS: usize = 0x70;
    pub(crate) const RED_CARDS: usize = 0x71;
}

/// The figures of a player entry (0x2d bytes) that `record_match_result` keeps (`Comp_Stats::get_player_entry_value`
/// reads them).
#[allow(dead_code)]
pub(crate) mod comp_player_figures {
    pub(crate) const GAMES: usize = 0xc;
    pub(crate) const GAMES_AS_SUBSTITUTE: usize = 0xd;
    pub(crate) const GOALS: usize = 0xe;
    pub(crate) const GOALS_CONCEDED: usize = 0xf;
    pub(crate) const PENALTIES_TAKEN: usize = 0x10;
    pub(crate) const PENALTIES_SCORED: usize = 0x11;
    pub(crate) const ASSISTS: usize = 0x12;
    /// u8 goals the player's team scored and conceded while he played, and its wins and defeats.
    pub(crate) const TEAM_GOALS_FOR: usize = 0x13;
    pub(crate) const TEAM_GOALS_AGAINST: usize = 0x14;
    pub(crate) const WINS: usize = 0x15;
    pub(crate) const DEFEATS: usize = 0x16;
    pub(crate) const YELLOW_CARDS: usize = 0x17;
    pub(crate) const RED_CARDS: usize = 0x18;
    pub(crate) const MAN_OF_THE_MATCH: usize = 0x19;
    /// u16 sum of the match ratings.
    pub(crate) const RATING_TOTAL: usize = 0x1a;
    /// u16 counts: passes, passes completed, tackles won, dribbles, shots, shots on target.
    pub(crate) const PASSES: usize = 0x1d;
    pub(crate) const PASSES_COMPLETED: usize = 0x1f;
    pub(crate) const TACKLES_WON: usize = 0x21;
    pub(crate) const DRIBBLES: usize = 0x23;
    pub(crate) const SHOTS: usize = 0x25;
    pub(crate) const SHOTS_ON_TARGET: usize = 0x27;
    /// i32 minutes played (the same offset as `comp_stats_player::APPEARANCES`).
    pub(crate) const MINUTES: usize = 0x29;
}

/// `news_post_fa_disciplinary_fine` (0x58a4d0, cdecl(club, level, amount)): the story of a national FA fining a club
/// for its disciplinary record.
pub(crate) const FN_NEWS_POST_FA_DISCIPLINARY_FINE: usize = 0x58a4d0;

/// The media story kinds of the runs a result ends: a winless run ended by a win and an unbeaten run ended by a
/// defeat (`Media::post_team_media_story`).
pub(crate) const STORY_WINLESS_RUN_ENDED: u32 = 0x51;
pub(crate) const STORY_UNBEATEN_RUN_ENDED: u32 = 0x50;

/// One side of the match `record_match_result` is recording.
pub(crate) struct RecordedSide {
    stats: usize,
    fixture: Fixture,
    /// The match record (`match_record`: attendance, man of the match, the two side blocks and lineups).
    game: usize,
    side: usize,
    club: Club,
    entry: CompStatsClub,
    /// The club's match record (`club_match_record`, filled by `fill_match_record_from_fixture`).
    record: usize,
}

impl RecordedSide {
    unsafe fn comp(&self) -> usize {
        read_usize(self.stats + comp_stats_obj::COMP)
    }
    unsafe fn season(&self, at: usize) -> usize {
        self.stats + comp_stats_obj::SEASON_RECORDS + at
    }
    unsafe fn all_time(&self, at: usize) -> usize {
        self.stats + comp_stats_obj::ALL_TIME_RECORDS + at
    }
    unsafe fn friendly(&self) -> bool {
        exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(self.comp() + league::COMPETITION)) != 0
    }
    unsafe fn league_division(&self) -> bool {
        exe::comp_util_is_league_division(read_i32(read_usize(self.comp() + league::COMPETITION))) != 0
    }
    unsafe fn seasons_ended(&self) -> i16 {
        read_i16(self.stats + COMP_STATS_SEASONS_ENDED)
    }
    unsafe fn year(&self) -> i16 {
        read_i16(self.comp() + league::YEAR)
    }
    unsafe fn club_id(&self) -> u32 {
        read_u32(self.club.addr())
    }
    unsafe fn goals_for(&self) -> i8 {
        read_i8(self.record + club_match_record::GOALS_FOR)
    }
    unsafe fn goals_against(&self) -> i8 {
        read_i8(self.record + club_match_record::GOALS_AGAINST)
    }
    /// The club's block of the match record (`match_record_team`).
    unsafe fn team(&self) -> usize {
        recorded_team_block(self.game, self.side)
    }
    /// Raises record flag `kind` and the "any flag" dword.
    unsafe fn flag(&self, kind: usize) {
        write_u32(self.stats + comp_stats_record_flag(kind), 1);
        write_u32(self.stats + COMP_STATS_ANY_FLAG, 1);
    }
    /// A record's year word against the competition's year (signed against unsigned, as the original compares).
    unsafe fn other_year(&self, year_at: usize) -> bool {
        self.year() as i32 != read_u16(year_at) as i32
    }
}

/// Side `side`'s block of a match record (`match_record_team`).
pub(crate) unsafe fn recorded_team_block(game: usize, side: usize) -> usize {
    game + match_record::HOME_TEAM + side * match_record::SIDE_SIZE
}

/// Rule: every played match goes into its competition's statistics: both clubs' entries (games, home crowds, form,
/// runs, goals, cards and the team totals), the entry of every player who took part, and the season's records,
/// which pass to the all-time records when they beat them (the news flag only from the third season end, for most
/// kinds only in a league division). A run's end can make a media story, and a club's cards can bring an FA fine.
///
/// `Comp_Stats::record_match_result(fixture, game, home_points, away_points)`: a local match record is constructed
/// (`Date::Date`); no fixture: the comp_stats.CPP 237 box and 0; no match record: the comp_stats.CPP 245 box and 0.
/// For each side (home, then away): `fill_match_record_from_fixture` of the side's club into the local record; the
/// club's entry (`comp_stats_find_club_entry`, or `comp_stats_add_club_entry`) counts one more game; the result code
/// (`match_record_result_code`) goes into the form lists (`record_side_venue`, then the all-matches list); the runs and
/// the result records (`record_side_result`); the points (`record_side_points`, the short of its side); the goal
/// totals (`record_side_goals`); the five team totals (`comp_club_figures::TEAM_TOTALS`, from the side block's bytes
/// +3..+7); the cards and the fine (`record_side_cards`); the highest scoring match (`record_highest_scoring`, home
/// side only); then each of the 20 lineup rows that counts (`match_lineup_row`) through `record_player_row`. At the end
/// the season's cached average attendance (`SEASON_MOST_POINTS`, -1) and average rating (`SEASON_RATING`, -1.0) are
/// cleared and, when the match has passes (`match_record::HOME_PASSES` ≠ 0), `accumulate_match_player_totals`. Returns
/// 1. The two points arguments are dwords whose high half the caller does not set; only their shorts are read.
/// Original: `Comp_Stats::record_match_result` (0x49f570, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_record_match_result(stats: usize, fixture: usize, game: usize, home_points: u32, away_points: u32) -> u32 {
    let mut record = [0u8; 0x20];
    let record_at = record.as_mut_ptr() as usize;
    exe::date_date_54a110(record_at);
    if fixture == 0 {
        show_error_box(COMP_STATS_CPP, 0xed);
        return 0;
    }
    if game == 0 {
        show_error_box(COMP_STATS_CPP, 0xf5);
        return 0;
    }
    let fixture = Fixture(fixture);
    for side in 0..2usize {
        let club = if side == 0 { fixture.home_club() } else { fixture.away_club() };
        fill_match_record_from_fixture(fixture.addr(), club.addr(), record_at);
        let mut entry = comp_stats_find_club_entry(stats, club.addr());
        if entry == 0 {
            covered!("tel43_1 club entry added");
            entry = comp_stats_add_club_entry(stats, club.addr());
        }
        let s = RecordedSide { stats, fixture, game, side, club, entry: CompStatsClub(entry), record: record_at };
        s.entry.set_matches_played(s.entry.matches_played().wrapping_add(1)); // tel43_1: one more game
        let result = match_record_result_code(record_at);
        record_side_venue(&s, result);
        push_form(entry + comp_club_figures::FORM, result);
        record_side_result(&s);
        let points = if side == 0 { home_points } else { away_points };
        record_side_points(&s, points as u16 as i16);
        record_side_goals(&s);
        for k in 0..5 {
            let at = entry + comp_club_figures::TEAM_TOTALS + k;
            write_u8(at, read_u8(at).wrapping_add(read_u8(s.team() + 3 + k))); // the side block's bytes +3..+7
        }
        record_side_cards(&s);
        record_highest_scoring(&s);
        for k in 0..20 {
            if let Some((row, person, sheet)) = match_lineup_row(game, side, k) {
                record_player_row(&s, k, row, Staff(person), sheet);
            }
        }
    }
    write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, -1);
    write_f32(stats + comp_stats_obj::SEASON_RATING, -1.0);
    if read_i16(game + match_record::HOME_PASSES) != 0 {
        comp_stats_accumulate_match_player_totals(stats, game);
    }
    1
}

/// Puts a result code at the head of a five-byte form list, the older ones moving down one place.
pub(crate) unsafe fn push_form(list: usize, result: u8) {
    for k in (1..comp_club_figures::FORM_LENGTH).rev() {
        write_u8(list + k, read_u8(list + k - 1));
    }
    write_u8(list, result);
}

/// The venue part of a side: at home (venue 1) one more home game, the attendance added to the home total, the
/// result into the home form and the attendance records (`record_attendance_record`, highest then lowest); at a neutral
/// ground (venue 3) the attendance records only for the home side (once per match); away (venue 2) the result into
/// the away form.
pub(crate) unsafe fn record_side_venue(s: &RecordedSide, result: u8) {
    match read_u8(s.record + club_match_record::VENUE) {
        1 => {
            s.entry.set_home_matches_played(s.entry.home_matches_played().wrapping_add(1));
            s.entry.set_home_attendance_total(s.entry.home_attendance_total().wrapping_add(read_i32(s.game + match_record::ATTENDANCE)));
            push_form(s.entry.addr() + comp_club_figures::HOME_FORM, result);
            record_attendance_record(s, comp_records::HIGHEST_ATTENDANCE, comp_record_kind::HIGHEST_ATTENDANCE, true);
            record_attendance_record(s, comp_records::LOWEST_ATTENDANCE, comp_record_kind::LOWEST_ATTENDANCE, false);
        }
        3 => {
            if s.side == 0 {
                covered!("tel43_1 neutral ground attendance");
                record_attendance_record(s, comp_records::HIGHEST_ATTENDANCE, comp_record_kind::HIGHEST_ATTENDANCE, true);
                record_attendance_record(s, comp_records::LOWEST_ATTENDANCE, comp_record_kind::LOWEST_ATTENDANCE, false);
            }
        }
        2 => push_form(s.entry.addr() + comp_club_figures::AWAY_FORM, result),
        _ => {}
    }
}

/// An attendance record (highest or lowest): outside friendlies, a positive attendance that beats the season's (or
/// fills it) becomes it with the club's match record; when it also beats the all-time one, that is copied from the
/// season's (`attendance_record_assign`), flagged from the third season end in a league division.
pub(crate) unsafe fn record_attendance_record(s: &RecordedSide, at: usize, kind: usize, highest: bool) {
    if s.friendly() {
        return;
    }
    let attendance = read_i32(s.game + match_record::ATTENDANCE);
    if attendance <= 0 {
        return;
    }
    let beats = |held: i32| held == -1 || if highest { attendance > held } else { attendance < held };
    let season = s.season(at);
    if !beats(read_i32(season)) {
        return;
    }
    write_i32(season, attendance);
    match_record_assign(season + comp_records::ATTENDANCE_GAME, s.record);
    let all = s.all_time(at);
    if !beats(read_i32(all)) {
        return;
    }
    if s.seasons_ended() >= 3 && s.league_division() {
        covered!("tel43_1 attendance record news");
        s.flag(kind);
    }
    attendance_record_assign(all, season);
}

/// The runs of a side by the result (goals, signed): a win records the biggest win, extends the winning run, may
/// make the "winless run ended" story and clears the losing and winless runs; a defeat extends the losing run, may
/// make the "unbeaten run ended" story and clears the winning and unbeaten runs. Then a win or draw extends the
/// unbeaten run and clears the losing one, and a draw or defeat extends the winless run and clears the winning one.
pub(crate) unsafe fn record_side_result(s: &RecordedSide) {
    let entry = s.entry.addr();
    if s.goals_for() > s.goals_against() {
        record_biggest_win(s);
        extend_run(s, comp_stats_club::WINNING_RUN, comp_records::WINNING_RUN, comp_record_kind::WINNING_RUN, 12);
        run_ended_story(s, STORY_WINLESS_RUN_ENDED, comp_stats_club::WINLESS_RUN, true);
        write_i32(entry + comp_stats_club::LOSING_RUN, 0);
        write_i32(entry + comp_stats_club::WINLESS_RUN, 0);
    } else if s.goals_for() < s.goals_against() {
        extend_run(s, comp_stats_club::LOSING_RUN, comp_records::LOSING_RUN, comp_record_kind::LOSING_RUN, 12);
        run_ended_story(s, STORY_UNBEATEN_RUN_ENDED, comp_stats_club::UNBEATEN_RUN, false);
        write_i32(entry + comp_stats_club::WINNING_RUN, 0);
        write_i32(entry + comp_stats_club::UNBEATEN_RUN, 0);
    }
    if s.goals_for() >= s.goals_against() {
        extend_run(s, comp_stats_club::UNBEATEN_RUN, comp_records::UNBEATEN_RUN, comp_record_kind::UNBEATEN_RUN, 24);
        write_i32(entry + comp_stats_club::LOSING_RUN, 0); // tel43_1: a win or a draw ends the losing run
    }
    if s.goals_for() <= s.goals_against() {
        extend_run(s, comp_stats_club::WINLESS_RUN, comp_records::WINLESS_RUN, comp_record_kind::WINLESS_RUN, 24);
        write_i32(entry + comp_stats_club::WINNING_RUN, 0);
    }
}

/// The biggest win: outside friendlies, the club's match record replaces the season's when there is none or it is
/// a bigger win (`match_record_is_bigger_win`); the all-time one the same way, flagged for a margin of 8 or more.
pub(crate) unsafe fn record_biggest_win(s: &RecordedSide) {
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::BIGGEST_WIN);
    if read_i32(season + club_match_record::FIXTURE_ID) != -1 && match_record_is_bigger_win(s.record, season) == 0 {
        return;
    }
    match_record_assign(season, s.record);
    let all = s.all_time(comp_records::BIGGEST_WIN);
    if read_i32(all + club_match_record::FIXTURE_ID) != -1 && match_record_is_bigger_win(s.record, all) == 0 {
        return;
    }
    if s.goals_for() as i32 - s.goals_against() as i32 >= 8 {
        covered!("tel43_1 biggest win news");
        s.flag(comp_record_kind::BIGGEST_WIN);
    }
    match_record_assign(all, season);
}

/// A run of the club entry goes up by one game: the first game sets its start date and every game its end date (the
/// current date). Outside friendlies, a run longer than the season's record (or with none) becomes it with the
/// club's id (`run_record_assign`); longer than the all-time one too, that is copied from the season's
/// (`comp_run_record_assign`), flagged when the run reached `flag_from` games and the all-time run had another start
/// date or club (the same run still going raises no news).
pub(crate) unsafe fn extend_run(s: &RecordedSide, run_at: usize, record_at: usize, kind: usize, flag_from: i32) {
    let run = s.entry.addr() + run_at;
    let games = read_i32(run).wrapping_add(1);
    write_i32(run, games);
    if games == 1 {
        copy_record_date(run + run_record::START_DATE, G_CURRENT_DATE);
    }
    copy_record_date(run + run_record::END_DATE, G_CURRENT_DATE);
    if s.friendly() {
        return;
    }
    let season = s.season(record_at);
    let held = read_i32(season);
    if held != -1 && games <= held {
        return;
    }
    run_record_assign(season, run);
    write_u32(season + comp_records::RUN_CLUB, s.club_id());
    let all = s.all_time(record_at);
    let best = read_i32(all);
    if best != -1 && games <= best {
        return;
    }
    if games >= flag_from {
        let (start, held_start) = (run + run_record::START_DATE, all + run_record::START_DATE);
        if read_u16(held_start + 2) != read_u16(start + 2) || read_u16(held_start) != read_u16(start)
            || s.club_id() != read_u32(all + comp_records::RUN_CLUB) {
            covered!("tel43_1 run record news");
            s.flag(kind);
        }
    }
    comp_run_record_assign(all, season);
}

/// The media story of a run a result ends (the winless run ended by a win, the unbeaten run ended by a defeat): only
/// for a league match of the club's own division (`club.division()` = the fixture's competition) with a nation whose
/// league is selected and a manager. The run must beat `random(10) + 10`, not exceed the club's games, and then
/// `random(10000)` must stay below the club's reputation and `random(3)` be 0. The story names the man of the match
/// (the first person whose player record has the match's man of the match id, a valid id and, for the win, the club
/// as his club; for the defeat, another club), or nobody. The variant is the run's low byte over the search index's
/// high bytes, as the original pushes it (`post_team_media_story` reads only the byte).
pub(crate) unsafe fn run_ended_story(s: &RecordedSide, kind: u32, run_at: usize, own_club: bool) {
    let club = s.club;
    if club.division().is_null() || club.division().addr() != s.fixture.competition().addr() {
        return;
    }
    if club.nation().is_null() || club.nation().league_selected() == 0 || club.manager().is_null() {
        return;
    }
    let run = read_i32(s.entry.addr() + run_at);
    if run <= exe::utils_random(10) + 10 || (s.entry.matches_played() as i32) < run {
        return;
    }
    if exe::utils_random(10000) >= club.reputation() as i32 || exe::utils_random(3) != 0 {
        return;
    }
    covered!("tel43_1 run ended story");
    let count = read_i32(G_STAFF_COUNT);
    let star = read_u32(s.game + match_record::MAN_OF_THE_MATCH_ID);
    let (mut person, mut k) = (0usize, 0i32);
    while k < count {
        let staff = staff_at(k as isize);
        let player = staff.player();
        if !player.is_null() && read_u32(player.addr()) == star && (staff.is_null() || read_i32(staff.addr()) < count)
            && (staff.club_job() == club) == own_club {
            person = staff.addr();
            break;
        }
        k += 1;
    }
    let variant = (k as u32 & 0xffff_ff00) | read_u8(s.entry.addr() + run_at) as u32;
    exe::media_post_team_media_story(read_usize(G_MEDIA), club, kind, person as u32, s.fixture, variant, -1);
}

/// The points record (kind 2): outside friendlies, in a league division and with positive points
/// (`record_team_value_record`).
pub(crate) unsafe fn record_side_points(s: &RecordedSide, points: i16) {
    if s.friendly() || !s.league_division() || points <= 0 {
        return;
    }
    record_team_value_record(s, comp_records::POINTS, comp_record_kind::POINTS, points);
}

/// The goal totals: the goals scored, when any, go into the entry's total and its record (kind 3); the goals
/// conceded the same (kind 4); outside friendlies (`record_team_value_record`).
pub(crate) unsafe fn record_side_goals(s: &RecordedSide) {
    for (goals, total, at, kind) in [(s.goals_for(), comp_club_figures::GOALS_FOR, comp_records::GOALS_FOR, comp_record_kind::GOALS_FOR),
        (s.goals_against(), comp_club_figures::GOALS_AGAINST, comp_records::GOALS_AGAINST, comp_record_kind::GOALS_AGAINST)] {
        if goals == 0 {
            continue;
        }
        let total = s.entry.addr() + total;
        write_u8(total, read_u8(total).wrapping_add(goals as u8));
        if !s.friendly() {
            record_team_value_record(s, at, kind, read_u8(total) as i16);
        }
    }
}

/// A team value record: above the season's (or with none, -1), the value, the club and a cleared year become it;
/// above the all-time one too, that takes the season's value and club and the competition's year, flagged from the
/// third season end when the holder or the year changes, in a league division.
pub(crate) unsafe fn record_team_value_record(s: &RecordedSide, at: usize, kind: usize, value: i16) {
    let season = s.season(at);
    let held = read_i16(season);
    if held != -1 && value <= held {
        return;
    }
    write_i16(season, value);
    write_u32(season + comp_records::VALUE_CLUB, s.club_id());
    write_u16(season + comp_records::VALUE_YEAR, 0);
    let all = s.all_time(at);
    let best = read_i16(all);
    if best != -1 && value <= best {
        return;
    }
    if s.seasons_ended() >= 3 && (s.other_year(all + comp_records::VALUE_YEAR) || s.club_id() != read_u32(all + comp_records::VALUE_CLUB))
        && s.league_division() {
        covered!("tel43_1 team value record news");
        s.flag(kind);
    }
    write_u32(all, read_u32(season));
    write_u32(all + 4, read_u32(season + 4));
    write_i16(all + comp_records::VALUE_YEAR, s.year());
}

/// The cards of a side: with any card in the match, the entry's yellow and red totals go up; with the finance table
/// and the manager manager, the FA fine (`charge_discipline_fine`, by the points yellow + 4 × red); then, outside
/// friendlies, the cards record (kind 5, ordered by red × 3 + yellow): above the season's (or with none, 0xff), the
/// totals, a cleared year and the club; above the all-time one too, that takes them and the competition's year,
/// flagged from the third season end with 2 or more of each and another holder or year, in a league division.
pub(crate) unsafe fn record_side_cards(s: &RecordedSide) {
    let entry = s.entry.addr();
    let (yellow, red) = (read_u8(s.team() + match_record_team::YELLOW_CARDS), read_u8(s.team() + match_record_team::RED_CARDS));
    if yellow == 0 && red == 0 {
        return;
    }
    write_u8(entry + comp_club_figures::YELLOW_CARDS, read_u8(entry + comp_club_figures::YELLOW_CARDS).wrapping_add(yellow));
    write_u8(entry + comp_club_figures::RED_CARDS, read_u8(entry + comp_club_figures::RED_CARDS).wrapping_add(red));
    let (yellow, red) = (read_u8(entry + comp_club_figures::YELLOW_CARDS), read_u8(entry + comp_club_figures::RED_CARDS));
    if read_usize(G_FINANCE_TABLE) != 0 && read_usize(G_MANAGER_MANAGER) != 0 {
        charge_discipline_fine(s.club, (yellow as u16).wrapping_add((red as u16).wrapping_mul(4)) as i16);
    }
    let weight = |y: usize| read_i8(y + 1) as i32 * 3 + read_i8(y) as i32;
    let value = red as i32 * 3 + yellow as i32;
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::CARDS);
    if read_u8(season) != 0xff && value <= weight(season) {
        return;
    }
    write_u8(season, yellow);
    write_u8(season + 1, red);
    write_u16(season + comp_records::VALUE_YEAR, 0);
    write_u32(season + comp_records::VALUE_CLUB, s.club_id());
    let all = s.all_time(comp_records::CARDS);
    if read_u8(all) != 0xff && value <= weight(all) {
        return;
    }
    if s.seasons_ended() >= 3 && yellow >= 2 && red >= 2
        && (s.other_year(all + comp_records::VALUE_YEAR) || s.club_id() != read_u32(all + comp_records::VALUE_CLUB)) && s.league_division() {
        covered!("tel43_1 cards record news");
        s.flag(comp_record_kind::CARDS);
    }
    write_u32(all, read_u32(season));
    write_u32(all + 4, read_u32(season + 4));
    write_i16(all + comp_records::VALUE_YEAR, s.year());
}

/// A club's FA fine level: the low three bits of its board row's discipline counters, signed (0 without a club).
pub(crate) unsafe fn discipline_fine_level(club: Club) -> i8 {
    if club.is_null() {
        return 0;
    }
    let counters = BoardRecord(board_row(read_usize(G_MANAGER_MANAGER), club)).discipline_counters();
    ((counters & discipline_counters::FINE_LEVEL) << 5) as i8 >> 5
}

/// The card points a club must pass to be fined at `level` (0, 1, 2, and 3 for the last level); none below 0.
pub(crate) fn discipline_fine_threshold(level: i8) -> Option<i16> {
    match level {
        0 => Some(0x41),
        1 => Some(0x5a),
        2 => Some(0x73),
        l if l > 2 => Some(0x8c),
        _ => None,
    }
}

/// Rule: a club's cards in a competition bring an FA fine when their points (yellow + 4 × red, the competition's
/// totals) pass the threshold of its fine level: above 65 at level 0 (reputation × 3 + 2500), above 90 at level 1
/// ((reputation + 2000) × 5), above 115 at level 2 ((reputation + 2500) × 10) and, at level 3, above 140 one time in
/// ten (`random(10)` = 0, the same amount).
///
/// The club's finance (`financetable_get_club_finance`) is fetched first. A fine posts the FA's story
/// (`news_post_fa_disciplinary_fine` 0x58a4d0, cdecl(club, level, amount); it reads the level as a byte: the
/// original pushes it at level 3 as a dword whose high bytes it never set), raises the club's fine level by one and
/// takes the amount (`pay_league_fine`). Defect of the original: the level is three signed bits and the raise checks
/// "below 4", always true: level 3 becomes 4, read back as -4, and the club is never fined again.
pub(crate) unsafe fn charge_discipline_fine(club: Club, points: i16) {
    let level = discipline_fine_level(club);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let Some(threshold) = discipline_fine_threshold(level) else { return };
    if points <= threshold {
        return;
    }
    let reputation = || club.reputation() as i32;
    let amount = match level {
        0 => reputation() * 3 + 2500,
        1 => (reputation() + 2000) * 5,
        2 => (reputation() + 2500) * 10,
        _ => {
            if exe::utils_random(10) != 0 {
                return;
            }
            (reputation() + 2500) * 10
        }
    };
    covered!("tel43_1 FA fine");
    let post: unsafe extern "C" fn(usize, u32, i32) = core::mem::transmute(FN_NEWS_POST_FA_DISCIPLINARY_FINE);
    post(club.addr(), level as u8 as u32, amount);
    if !club.is_null() {
        let row = BoardRecord(board_row(read_usize(G_MANAGER_MANAGER), club));
        let counters = row.discipline_counters();
        let current = ((counters & discipline_counters::FINE_LEVEL) << 5) as i8 >> 5;
        if current < 4 {
            row.set_discipline_counters((counters & !discipline_counters::FINE_LEVEL) | ((current + 1) as u8 & discipline_counters::FINE_LEVEL));
        }
    }
    pay_league_fine(finance, amount); // tel43_1: the fine is paid
}

/// The highest scoring match (kind 10): only for the home side, outside friendlies and with goals; the club's
/// match record replaces the season's when there is none or it has more goals (`match_record_has_more_goals`); the
/// all-time one the same way, flagged for 10 goals or more.
pub(crate) unsafe fn record_highest_scoring(s: &RecordedSide) {
    if s.friendly() || s.side != 0 {
        return;
    }
    let goals = s.goals_for() as i32 + s.goals_against() as i32;
    if goals <= 0 {
        return;
    }
    let season = s.season(comp_records::HIGHEST_SCORING);
    if read_i32(season + club_match_record::FIXTURE_ID) != -1 && match_record_has_more_goals(s.record, season) == 0 {
        return;
    }
    match_record_assign(season, s.record);
    let all = s.all_time(comp_records::HIGHEST_SCORING);
    if read_i32(all + club_match_record::FIXTURE_ID) != -1 && match_record_has_more_goals(s.record, all) == 0 {
        return;
    }
    if goals >= 10 {
        covered!("tel43_1 highest scoring news");
        s.flag(comp_record_kind::HIGHEST_SCORING);
    }
    match_record_assign(all, season);
}

/// One lineup row (row `k`, 11 and up the substitutes) of a side: the player's entry (`comp_stats_find_player_entry`,
/// or `comp_stats_add_player_entry` with the person's id) takes the club and the fixture's round byte, one more game
/// (and one as a substitute); the goals (with the top scorer and goals-in-a-match records), the goals conceded and
/// the penalties, the assists (record), the team's goals and the win or defeat, the cards (record), the man of the
/// match (record) and the rating; with the match's statistics (`match_record::NO_STATISTICS` = 0) the passes,
/// tackles, dribbles and shots and the minutes; then the youngest and the oldest player records.
pub(crate) unsafe fn record_player_row(s: &RecordedSide, k: usize, row: usize, person: Staff, sheet: usize) {
    use comp_player_figures as f;
    let mut entry = comp_stats_find_player_entry(s.stats, sheet);
    if entry == 0 {
        entry = comp_stats_add_player_entry(s.stats, sheet, read_u32(person.addr()));
    }
    let add = |at: usize, v: u8| write_u8(entry + at, read_u8(entry + at).wrapping_add(v));
    write_u32(entry + comp_stats_player::CLUB, s.club_id());
    write_u8(entry + comp_stats_player::NEW_MARK, s.fixture.round_number() as u8);
    add(f::GAMES, 1); // tel43_1: one more game for the player
    if k > 10 {
        add(f::GAMES_AS_SUBSTITUTE, 1);
    }
    let goals = read_u8(row + lineup_row::GOALS);
    if goals != 0 {
        add(f::GOALS, goals);
        record_player_tally(s, comp_records::TOP_SCORER, comp_record_kind::TOP_SCORER, read_u8(entry + f::GOALS), person);
        record_goals_in_a_match(s, goals, person);
    }
    add(f::GOALS_CONCEDED, read_u8(row + lineup_row::GOALS_CONCEDED));
    add(f::PENALTIES_TAKEN, read_u8(row + lineup_row::PENALTIES_TAKEN));
    add(f::PENALTIES_SCORED, read_u8(row + lineup_row::PENALTIES_SCORED));
    let assists = read_u8(row + lineup_row::ASSISTS);
    if assists != 0 {
        add(f::ASSISTS, assists);
        record_player_tally(s, comp_records::ASSISTS, comp_record_kind::ASSISTS, read_u8(entry + f::ASSISTS), person);
    }
    let (scored, conceded) = (s.goals_for(), s.goals_against());
    if scored != 0 {
        add(f::TEAM_GOALS_FOR, scored as u8);
    }
    if conceded != 0 {
        add(f::TEAM_GOALS_AGAINST, conceded as u8);
    }
    if scored > conceded {
        add(f::WINS, 1);
    } else if scored < conceded {
        add(f::DEFEATS, 1);
    }
    let (yellow, red) = (read_u8(row + lineup_row::YELLOW_CARDS), read_u8(row + lineup_row::RED_CARD));
    if yellow != 0 || red != 0 {
        add(f::YELLOW_CARDS, yellow);
        add(f::RED_CARDS, red);
        record_player_cards(s, entry, person);
    }
    if read_u32(row + lineup_row::PLAYER_ID) == read_u32(s.game + match_record::MAN_OF_THE_MATCH_ID) {
        add(f::MAN_OF_THE_MATCH, 1);
        record_player_tally(s, comp_records::MAN_OF_THE_MATCH, comp_record_kind::MAN_OF_THE_MATCH, read_u8(entry + f::MAN_OF_THE_MATCH), person);
    }
    write_u16(entry + f::RATING_TOTAL, read_u16(entry + f::RATING_TOTAL).wrapping_add(read_u8(row + lineup_row::RATING) as u16));
    if read_u8(s.game + match_record::NO_STATISTICS) == 0 {
        add_player_match_figures(s, k, row, entry);
    }
    record_age(s, comp_records::YOUNGEST, comp_record_kind::YOUNGEST, person, true);
    record_age(s, comp_records::OLDEST, comp_record_kind::OLDEST, person, false);
}

/// The figures of a match with statistics: the six u16 counts (passes, passes completed, tackles won, dribbles,
/// shots, shots on target) and the minutes: up to the minute he left the pitch, else the minute he was taken off,
/// else the match's length (`match_record::MINUTE`); a substitute (row 11 and up) who came on counts from his
/// entry minute.
pub(crate) unsafe fn add_player_match_figures(s: &RecordedSide, k: usize, row: usize, entry: usize) {
    use comp_player_figures as f;
    for (at, from) in [(f::PASSES, lineup_row::PASSES), (f::PASSES_COMPLETED, lineup_row::PASSES_COMPLETED), (f::TACKLES_WON, lineup_row::TACKLES_WON),
        (f::DRIBBLES, lineup_row::DRIBBLES), (f::SHOTS, lineup_row::SHOTS), (f::SHOTS_ON_TARGET, lineup_row::SHOTS_ON_TARGET)] {
        write_u16(entry + at, read_u16(entry + at).wrapping_add(read_u8(row + from) as u16));
    }
    let left = read_i8(row + lineup_row::MINUTE_LEFT_PITCH);
    let off = read_i8(row + lineup_row::MINUTE_OFF);
    let until = if left > -1 { left as i32 } else if off > -1 { off as i32 } else { read_i16(s.game + match_record::MINUTE) as i32 };
    let minutes = entry + f::MINUTES;
    if k < 11 {
        write_i32(minutes, read_i32(minutes).wrapping_add(until)); // tel43_1: a starter's minutes
    } else if read_u8(row + lineup_row::ENTRY_MINUTE) != 0xff {
        write_i32(minutes, read_i32(minutes).wrapping_add(until - read_i8(row + lineup_row::ENTRY_MINUTE) as i32));
    }
}

/// A player tally record (top scorer, assists, man of the match): outside friendlies, a count above the season's
/// (signed byte, or with none, 0xff) becomes it with the holder (`fill_staff_name_ref`), a cleared year and the club;
/// above the all-time one too, that takes them (`record_holder_assign`) and the competition's year, flagged from the
/// third season end when the person or the year changes, in a league division.
pub(crate) unsafe fn record_player_tally(s: &RecordedSide, at: usize, kind: usize, count: u8, person: Staff) {
    if s.friendly() {
        return;
    }
    let beats = |held: u8| held == 0xff || count as i32 > held as i8 as i32;
    let season = s.season(at);
    if !beats(read_u8(season)) {
        return;
    }
    write_u8(season, count);
    fill_staff_name_ref(person.addr(), season + comp_records::TALLY_HOLDER);
    write_u16(season + comp_records::TALLY_YEAR, 0);
    write_u32(season + comp_records::TALLY_CLUB, s.club_id());
    let all = s.all_time(at);
    if !beats(read_u8(all)) {
        return;
    }
    if s.seasons_ended() >= 3
        && (read_u32(all + comp_records::TALLY_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) || s.other_year(all + comp_records::TALLY_YEAR))
        && s.league_division() {
        covered!("tel43_1 player tally news");
        s.flag(kind);
    }
    write_u8(all, read_u8(season));
    record_holder_assign(all + comp_records::TALLY_HOLDER, season + comp_records::TALLY_HOLDER);
    write_u32(all + comp_records::TALLY_CLUB, read_u32(season + comp_records::TALLY_CLUB));
    write_u16(all + comp_records::TALLY_YEAR, read_u16(season + comp_records::TALLY_YEAR));
    write_i16(all + comp_records::TALLY_YEAR, s.year());
}

/// The most goals in a match (kind 16): outside friendlies, more goals than the season's (signed byte, or none,
/// 0xff) make it, with the holder and the club's match record; more than the all-time one too, that is copied from
/// the season's, flagged for 5 goals or more.
pub(crate) unsafe fn record_goals_in_a_match(s: &RecordedSide, goals: u8, person: Staff) {
    if s.friendly() {
        return;
    }
    let beats = |held: u8| held == 0xff || goals as i32 > held as i8 as i32;
    let season = s.season(comp_records::GOALS_IN_A_MATCH);
    if !beats(read_u8(season)) {
        return;
    }
    write_u8(season, goals);
    fill_staff_name_ref(person.addr(), season + comp_records::TALLY_HOLDER);
    match_record_assign(season + comp_records::GOALS_IN_A_MATCH_GAME, s.record);
    let all = s.all_time(comp_records::GOALS_IN_A_MATCH);
    if !beats(read_u8(all)) {
        return;
    }
    if goals >= 5 {
        covered!("tel43_1 goals in a match news");
        s.flag(comp_record_kind::GOALS_IN_A_MATCH);
    }
    write_u8(all, read_u8(season));
    record_holder_assign(all + comp_records::TALLY_HOLDER, season + comp_records::TALLY_HOLDER);
    match_record_assign(all + comp_records::GOALS_IN_A_MATCH_GAME, season + comp_records::GOALS_IN_A_MATCH_GAME);
}

/// A player's cards record (kind 20, ordered by red × 3 + yellow of his totals): outside friendlies, above the
/// season's (or none, 0xff) the totals, the holder, a cleared year and the club make it; above the all-time one too,
/// that takes them and the competition's year, flagged from the third season end with 2 or more of each and another
/// person or year, in a league division.
pub(crate) unsafe fn record_player_cards(s: &RecordedSide, entry: usize, person: Staff) {
    use comp_player_figures as f;
    let (yellow, red) = (read_u8(entry + f::YELLOW_CARDS), read_u8(entry + f::RED_CARDS));
    let value = red as i32 * 3 + yellow as i32;
    let weight = |y: usize| read_i8(y + 1) as i32 * 3 + read_i8(y) as i32;
    if s.friendly() {
        return;
    }
    let season = s.season(comp_records::PLAYER_CARDS);
    if read_u8(season) != 0xff && value <= weight(season) {
        return;
    }
    write_u8(season, yellow);
    write_u8(season + 1, red);
    fill_staff_name_ref(person.addr(), season + comp_records::PLAYER_CARDS_HOLDER);
    write_u16(season + comp_records::PLAYER_CARDS_YEAR, 0);
    write_u32(season + comp_records::PLAYER_CARDS_CLUB, s.club_id());
    let all = s.all_time(comp_records::PLAYER_CARDS);
    if read_u8(all) != 0xff && value <= weight(all) {
        return;
    }
    if s.seasons_ended() >= 3 && yellow >= 2 && red >= 2
        && (read_u32(all + comp_records::PLAYER_CARDS_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) || s.other_year(all + comp_records::PLAYER_CARDS_YEAR))
        && s.league_division() {
        covered!("tel43_1 player cards news");
        s.flag(comp_record_kind::PLAYER_CARDS);
    }
    write_u8(all, read_u8(season));
    write_u8(all + 1, read_u8(season + 1));
    record_holder_assign(all + comp_records::PLAYER_CARDS_HOLDER, season + comp_records::PLAYER_CARDS_HOLDER);
    write_u32(all + comp_records::PLAYER_CARDS_CLUB, read_u32(season + comp_records::PLAYER_CARDS_CLUB));
    write_u16(all + comp_records::PLAYER_CARDS_YEAR, read_u16(season + comp_records::PLAYER_CARDS_YEAR));
    write_i16(all + comp_records::PLAYER_CARDS_YEAR, s.year());
}

/// A person's age as the date difference (`Date::get_difference`) from his birth date to `on`: (day, year).
pub(crate) unsafe fn age_on(on: usize, birth: usize) -> (i16, u16) {
    let mut out = [0u8; 8];
    let d = exe::date_get_difference(on, out.as_mut_ptr(), read_u32(birth), read_u32(birth + 4));
    (read_i16(d), read_u16(d + 2))
}

/// Whether the person beats an age record: his age today against the holder's age on the record's match date, the
/// years first (unsigned) and then the days (signed): younger for the youngest record, older for the oldest.
pub(crate) unsafe fn beats_age_record(record: usize, person: Staff, youngest: bool) -> bool {
    let (held_day, held_year) = age_on(record + comp_records::AGE_GAME, record);
    let (day, year) = age_on(G_CURRENT_DATE, person.addr() + staff::DATE_OF_BIRTH);
    if youngest {
        held_year > year || (held_year == year && held_day > day)
    } else {
        held_year < year || (held_year == year && held_day < day)
    }
}

/// The youngest (kind 21) and oldest (kind 22) player records: outside friendlies, with no season record (its match
/// record's fixture id -1) or a person who beats it (`beats_age_record`), his birth date, the holder and the club's
/// match record make it; beating the all-time one too, that is copied from the season's (`player_age_record_assign`),
/// flagged from the third season end for an age under 20 years (youngest) or over 35 (oldest) of another person.
pub(crate) unsafe fn record_age(s: &RecordedSide, at: usize, kind: usize, person: Staff, youngest: bool) {
    if s.friendly() {
        return;
    }
    let season = s.season(at);
    if read_i32(season + comp_records::AGE_GAME + club_match_record::FIXTURE_ID) != -1 && !beats_age_record(season, person, youngest) {
        return;
    }
    copy_record_date(season, person.addr() + staff::DATE_OF_BIRTH);
    fill_staff_name_ref(person.addr(), season + comp_records::AGE_HOLDER);
    match_record_assign(season + comp_records::AGE_GAME, s.record);
    let all = s.all_time(at);
    if read_i32(all + comp_records::AGE_GAME + club_match_record::FIXTURE_ID) != -1 && !beats_age_record(all, person, youngest) {
        return;
    }
    if s.seasons_ended() >= 3 {
        let (_, years) = age_on(G_CURRENT_DATE, person.addr() + staff::DATE_OF_BIRTH);
        let notable = if youngest { years < 20 } else { years > 35 };
        if notable && read_u32(all + comp_records::AGE_HOLDER + record_holder::STAFF_ID) != read_u32(person.addr()) {
            covered!("tel43_1 age record news");
            s.flag(kind);
        }
    }
    player_age_record_assign(all, season);
}

/// Whether recording the match only touches memory the comparer can put back: both clubs have their entry already
/// (adding one may free the saved runs), the players without an entry fit the table (no `realloc`), no club of a
/// league match of its own division has a run above 10 that a result could end (the media story), and no club's
/// cards reach its fine (the FA story and the payment).
pub(crate) unsafe fn record_match_memory_only(stats: usize, fixture: Fixture, game: usize) -> bool {
    let mut missing = 0u32;
    for side in 0..2usize {
        let club = if side == 0 { fixture.home_club() } else { fixture.away_club() };
        if club.is_null() {
            return false;
        }
        let entry = comp_stats_find_club_entry(stats, club.addr());
        if entry == 0 {
            return false;
        }
        let e = CompStatsClub(entry);
        if !club.division().is_null() && club.division().addr() == fixture.competition().addr() && e.unbeaten_run().max(e.winless_run()) > 10 {
            return false;
        }
        let team = recorded_team_block(game, side);
        let (yellow, red) = (read_u8(team + match_record_team::YELLOW_CARDS), read_u8(team + match_record_team::RED_CARDS));
        if (yellow != 0 || red != 0) && read_usize(G_FINANCE_TABLE) != 0 && read_usize(G_MANAGER_MANAGER) != 0 {
            let yellow = read_u8(entry + comp_club_figures::YELLOW_CARDS).wrapping_add(yellow);
            let red = read_u8(entry + comp_club_figures::RED_CARDS).wrapping_add(red);
            let points = (yellow as u16).wrapping_add((red as u16).wrapping_mul(4)) as i16;
            if discipline_fine_threshold(discipline_fine_level(club)).is_some_and(|t| points > t) {
                return false;
            }
        }
        for k in 0..20 {
            if let Some((_, _, sheet)) = match_lineup_row(game, side, k) {
                if comp_stats_find_player_entry(stats, sheet) == 0 {
                    missing += 1;
                }
            }
        }
    }
    missing <= read_u32(stats + comp_stats_obj::PLAYER_CAPACITY).wrapping_sub(read_u32(stats + comp_stats_obj::PLAYER_COUNT))
}

/// Comparer for `comp_stats_record_match_result`: side by side over the statistics object, both tables (their whole
/// capacity: new player entries go after the count), the sort context and the random generator's state (the story's
/// draws), when the match only touches memory (`record_match_memory_only`); otherwise (a new club entry, a table that
/// must grow, a possible story or fine, a null argument) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_record_match_result(stats: usize, fixture: usize, game: usize, home_points: u32, away_points: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_COMP_STATS_RECORD_MATCH_RESULT);
    if fixture == 0 || game == 0 || !record_match_memory_only(stats, Fixture(fixture), game) {
        return o(stats, fixture, game, home_points, away_points);
    }
    let mut regions = vec![(stats, COMP_STATS_OBJECT_SIZE), (G_COMP_STATS_SORT_CONTEXT, 4)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    for (table, capacity, size) in [(comp_stats_obj::CLUBS, comp_stats_obj::CLUB_CAPACITY, comp_stats_obj::CLUB_ENTRY_SIZE),
        (comp_stats_obj::PLAYERS, comp_stats_obj::PLAYER_CAPACITY, comp_stats_obj::PLAYER_ENTRY_SIZE)] {
        let (at, n) = (read_usize(stats + table), read_u32(stats + capacity) as usize * size);
        if at != 0 && n != 0 {
            regions.push((at, n));
        }
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, fixture, game, home_points, away_points),
        || crate::rng::without_counting(|| comp_stats_record_match_result(stats, fixture, game, home_points, away_points)));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::record_match_result({:#x}, fixture {:#x}): {} x {}, differences in {:?}",
        stats, fixture, ro, rr, first_differences(&mo, &mr)));
    ro
}

// ---------- sx03_1: the award manager's daily and per-match passes and the award base constructor (award_manager.cpp) ----------

/// YEAR_RATINGS::record_match_ratings (0x9315d0, thiscall(fixture, record, filter), ret 0xc).
const FN_YEAR_RATINGS_RECORD_MATCH_RATINGS: usize = 0x9315d0;
/// YEAR_RATINGS::reset_if_year (0x931aa0, thiscall(today), ret 4).
const FN_YEAR_RATINGS_RESET_IF_YEAR: usize = 0x931aa0;
/// The award manager's two year ratings (+0x10: every player, +0xc: the selected nations) and the filters (cdecl
/// `filter(person, fixture)`) each one is fed with.
const AWARD_MANAGER_WORLD_YEAR_RATINGS: usize = 0x10;
const AWARD_MANAGER_SELECTED_YEAR_RATINGS: usize = 0xc;
/// award_rating_filter_reputation_over_2000: a player whose current reputation is above 2000.
const AWARD_FILTER_REPUTATION_OVER_2000: usize = 0x414b20;
/// award_rating_filter_selected_nations: Denmark, Belgium, France, Holland or Sweden (the player's or the fixture's).
const AWARD_FILTER_SELECTED_NATIONS: usize = 0x414b40;

/// Feeds a played match to the award objects: the nation awards of the match's competition nation and both year
/// ratings.
///
/// `AWARD_MANAGER::process_fixture(fixture, record)`, from the end of a match (match_day.rs, with `G_AWARD_MANAGER`). A
/// friendly (`fixture::CATEGORY` 1) counts for nothing. The nation is the competition's (`competition::NATION`; none
/// without a fixture, a competition or a nation); the first of the 30 nation award objects
/// (`award_manager_consts::NATION_AWARDS`) whose nation (`award_manager_consts::AWARDS_NATION`) is that one gets the
/// match through its virtual +8 (`record_match_ratings(fixture, record)`). With no nation, the first object with no
/// nation takes it (the international awards). Then the world year ratings (+0x10, the players above 2000 of current
/// reputation) and the selected nations' year ratings (+0xc), each when it exists.
/// Original: `AWARD_MANAGER::process_fixture` (0x4134c0, award_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_manager_process_fixture(this: usize, fixture: usize, record: usize) {
    use award_manager_consts as am;
    let game = Fixture(fixture);
    if game.category() == 1 {
        return;
    }
    let mut nation = 0;
    if fixture != 0 && !game.competition().is_null() && !game.competition().nation().is_null() {
        nation = game.competition().nation().addr();
    }
    let table = read_usize(this + am::NATION_AWARDS);
    if let Some(awards) = (0..am::NATION_AWARD_SLOTS).map(|i| read_usize(table + i * 4)).find(|&e| e != 0 && read_usize(e + am::AWARDS_NATION) == nation) {
        covered!("sx03_1 nation awards take the match");
        // virtual +8: record_match_ratings(fixture, record), thiscall, ret 8
        let record_match: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(read_usize(read_usize(awards) + 8));
        record_match(awards, fixture, record); // sx03_1: the nation's awards
    }
    let year_ratings: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_YEAR_RATINGS_RECORD_MATCH_RATINGS);
    let world = read_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS);
    if world != 0 {
        covered!("sx03_1 world year ratings");
        year_ratings(world, fixture, record, AWARD_FILTER_REPUTATION_OVER_2000); // sx03_1: the world year ratings
    }
    let selected = read_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS);
    if selected != 0 {
        year_ratings(selected, fixture, record, AWARD_FILTER_SELECTED_NATIONS);
    }
}

// simplification: the nation awards and the year ratings add entries (they may allocate) and give the month awards:
// the routine cannot run twice. Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_fixture(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_AWARD_MANAGER_PROCESS_FIXTURE)(this, fixture, record)
}

/// Runs the day for the award objects: each nation award object's daily slot, the past award block and both year
/// ratings.
///
/// `AWARD_MANAGER::process_date(today)`, once a day. Each of the 30 nation award objects
/// (`award_manager_consts::NATION_AWARDS`) that exists gets its virtual +4 (`daily_update(today)`), in slot order.
/// When the past award flag (+0xdc) is set, the 0x30 dwords of the past award state (+0x1c) and the flag are cleared.
/// Then the world year ratings (+0x10) and the selected nations' year ratings (+0xc) are reset when the year is theirs
/// (`YEAR_RATINGS::reset_if_year(today)`), each when it exists.
/// Original: `AWARD_MANAGER::process_date` (0x413540, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_process_date(this: usize, today: usize) {
    use award_manager_consts as am;
    let table = read_usize(this + am::NATION_AWARDS);
    for i in 0..am::NATION_AWARD_SLOTS {
        let awards = read_usize(table + i * 4);
        if awards != 0 {
            covered!("sx03_1 daily slot");
            // virtual +4: daily_update(today), thiscall, ret 4
            let daily: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(awards) + 4));
            daily(awards, today); // sx03_1: each nation's awards
        }
    }
    // +0xdc: the past award state block (+0x1c, 0x30 dwords) holds something
    if read_u32(this + 0xdc) != 0 {
        covered!("sx03_1 past award block cleared");
        for k in 0..0x30usize {
            write_u32(this + 0x1c + k * 4, 0);
        }
        write_u32(this + 0xdc, 0);
    }
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_YEAR_RATINGS_RESET_IF_YEAR);
    for ratings in [read_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS), read_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS)] {
        if ratings != 0 {
            reset(ratings, today);
        }
    }
}

// simplification: the daily slots give the awards (news, history records, reputations): the routine cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_date(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_AWARD_MANAGER_PROCESS_DATE)(this, today)
}

/// Builds the base part of an award: its vtable, competitions, the two values the award class passes on, its
/// reputations and whether it is for players.
///
/// `AWARD::AWARD(a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players)`, from the
/// month, year and team award constructors. The vtable is the base one (0x967864); +5 is the staff competition record
/// `staff_comp` of `G_STAFF_COMP_TABLE` (101-byte records, no check for −1); +9 is the club competition record
/// `club_comp` of `G_CLUB_COMP_TABLE` (107-byte records), or 0 for −1; +0xd and +0x11 take `a1` and
/// `a2`; the reputations go to `award_fields::NATIONAL_REPUTATION` and `award_fields::WORLD_REPUTATION` (their low
/// words) and the player flag to `award_fields::FOR_PLAYERS`. The seventh argument is not read. Returns the object.
/// Original: `AWARD::AWARD` (0x412e40, award_manager.cpp, thiscall, ret 0x20).
pub(crate) unsafe extern "thiscall" fn construct_award(this: usize, a1: u32, a2: u32, staff_comp: i32, national_reputation: u32, world_reputation: u32, club_comp: i32, _unused: u32, for_players: u32) -> usize {
    covered!("sx03_1 award built");
    write_u32(this, 0x967864);
    write_usize(this + 5, read_usize(G_STAFF_COMP_TABLE).wrapping_add((staff_comp as usize).wrapping_mul(101)));
    if club_comp == -1 {
        write_usize(this + 9, 0);
    } else {
        write_usize(this + 9, read_usize(G_CLUB_COMP_TABLE).wrapping_add((club_comp as usize).wrapping_mul(107)));
    }
    write_u32(this + 0xd, a1);
    write_u32(this + 0x11, a2);
    write_u16(this + award_fields::NATIONAL_REPUTATION, national_reputation as u16); // sx03_1: the national reputation
    write_u16(this + award_fields::WORLD_REPUTATION, world_reputation as u16);
    write_u32(this + award_fields::FOR_PLAYERS, for_players);
    this
}

/// Comparer for `construct_award`: side by side over the 0x1d bytes of the base part.
pub(crate) unsafe extern "thiscall" fn compare_construct_award(this: usize, a1: u32, a2: u32, staff_comp: i32, national_reputation: u32, world_reputation: u32, club_comp: i32, unused: u32, for_players: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, i32, u32, u32, i32, u32, u32) -> usize>(IDX_CONSTRUCT_AWARD);
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x1d)], || o(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players), || construct_award(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players));
    SCOREBOARD.record(equal, || format!("construct_award(staff comp {}, club comp {}) original={:x?} rust={:x?}", staff_comp, club_comp, mo, mr));
    r
}

// ---------- sx03_2: England's awards: the yearly awards, the month awards and the team of the week (england_awards.cpp) ----------

/// NationAwards::daily_update (0x768310, thiscall(today), ret 4): the month change check of every nation's awards.
const FN_NATION_AWARDS_DAILY_UPDATE: usize = 0x768310;
/// nation_awards_award_player_of_year (0x768cf0, thiscall(comp id, today, award, 7 filters), ret 0x28).
const FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR: usize = 0x768cf0;
/// NationAwards::award_team_of_season (0x7698f0, thiscall(comp id, award), ret 8).
const FN_NATION_AWARDS_AWARD_TEAM_OF_SEASON: usize = 0x7698f0;
/// nation_awards_award_manager_of_year (0x768940, thiscall(today, award, comp id), ret 0xc).
const FN_NATION_AWARDS_AWARD_MANAGER_OF_YEAR: usize = 0x768940;
/// nation_awards_build_team_of_the_week (0x769c70, thiscall(comp id, award), ret 8).
const FN_NATION_AWARDS_BUILD_TEAM_OF_THE_WEEK: usize = 0x769c70;
/// MONTH_AWARD::start_new_season (0x7672e0, thiscall(year), ret 4).
const FN_MONTH_AWARD_START_NEW_SEASON: usize = 0x7672e0;
/// MONTH_AWARD::award_best_rated_player (0x767150, thiscall(month ratings, age), ret 8).
const FN_MONTH_AWARD_AWARD_BEST_RATED_PLAYER: usize = 0x767150;
/// Staff_Records::find_manager_of_the_month (0x86f950, thiscall(comp id), ret 4): the person, or 0.
const FN_STAFF_RECORDS_FIND_MANAGER_OF_THE_MONTH: usize = 0x86f950;

/// A nation's awards object (`NationAwards`): the award table (+4, pointers), the month ratings (+0xa) and the nation
/// record (+0xe, 0 for the international and European awards).
const NATION_AWARDS_TABLE: usize = 0x4;
const NATION_AWARDS_MONTH_RATINGS: usize = 0xa;
const NATION_AWARDS_NATION: usize = 0xe;

/// Award `k` of a nation's awards object.
pub(crate) unsafe fn nation_award(this: usize, k: usize) -> usize {
    read_usize(read_usize(this + NATION_AWARDS_TABLE) + k * 4)
}

/// Whether `today` (a Date: day of the year, year) is day `day` of month `month` (zero-based) of its own year
/// (`exe::date_date_549f70` with any weekday).
pub(crate) unsafe fn awards_day_is(today: usize, day: u32, month: u32) -> bool {
    is_date_today(make_date_dword(day, month, read_u16(today + cm_date::YEAR) as u32, u32::MAX), today)
}

/// The yearly player award of a competition from its stats table (`nation_awards_award_player_of_year`): the last seven
/// arguments are the filters (min age or −1, position flags, appearances...) as each nation passes them.
pub(crate) unsafe fn award_player_of_year(this: usize, comp_key: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR);
    let [a, b, c, d, e, g, h] = filters;
    f(this, read_u32(comp_key), today, award, a, b, c, d, e, g, h)
}

/// The team of the season of a league (`NationAwards::award_team_of_season`).
pub(crate) unsafe fn award_team_of_season(this: usize, comp_key: usize, award: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TEAM_OF_SEASON);
    f(this, read_u32(comp_key), award)
}

/// The manager of the year of a league (`nation_awards_award_manager_of_year`).
pub(crate) unsafe fn award_manager_of_year(this: usize, today: usize, award: usize, comp_key: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_MANAGER_OF_YEAR);
    f(this, today, award, read_u32(comp_key))
}

/// Moves a month award's slots on to the new season (`MONTH_AWARD::start_new_season(year)`).
pub(crate) unsafe fn month_award_start_new_season(award: usize, year: u16) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MONTH_AWARD_START_NEW_SEASON);
    f(award, year as u32)
}

/// The player of the month of a month award from the nation's month ratings, for an age limit or −1
/// (`MONTH_AWARD::award_best_rated_player`).
pub(crate) unsafe fn award_month_player(this: usize, award: usize, age: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_MONTH_AWARD_AWARD_BEST_RATED_PLAYER);
    f(award, read_usize(this + NATION_AWARDS_MONTH_RATINGS), age)
}

/// The manager of the month of a league: the best manager of the month (`Staff_Records::find_manager_of_the_month`)
/// takes the month award with his club, in the month and year of the staff records' last daily update
/// (`staff_records_fields::CURRENT_MONTH`/`CURRENT_YEAR`); none, nothing. The club is read without a check, as in the
/// original.
pub(crate) unsafe fn award_month_manager(award: usize, comp_key: usize) {
    let find: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_FIND_MANAGER_OF_THE_MONTH);
    let manager = find(read_usize(G_STAFF_RECORDS), read_u32(comp_key));
    if manager == 0 {
        return;
    }
    let records = read_usize(G_STAFF_RECORDS);
    let manager = Staff(manager);
    // the high bytes of the month and year arguments are the registers' (Staff_Records::add_month_award_record keeps
    // only the low byte and word)
    exe::month_award_set_winner(award, read_i32(manager.addr()), read_i32(manager.club_job().addr()), read_u8(records + staff_records_fields::CURRENT_MONTH) as u32, read_u16(records + staff_records_fields::CURRENT_YEAR) as u32);
}

/// The team of the week of an award's competition (`nation_awards_build_team_of_the_week(comp id, award)`).
pub(crate) unsafe fn build_team_of_the_week(this: usize, award: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_BUILD_TEAM_OF_THE_WEEK);
    f(this, read_u32(read_usize(award + 9)), award)
}

/// Whether the awards' nation plays its lower divisions (the Conference for England: `league_selected` with
/// `LOWER_DIVISIONS_LOADED`).
pub(crate) unsafe fn awards_lower_divisions(this: usize) -> bool {
    Nation(read_usize(this + NATION_AWARDS_NATION)).league_selected() & league_selected::LOWER_DIVISIONS_LOADED != 0
}

/// England's four divisions in award order: the competition key, the first award slot of the division (a block of six:
/// team, manager of the month, player of the month, young player of the month, manager of the year, team of the season
/// in the order the routines below use).
const ENGLAND_DIVISIONS: [usize; 4] = [KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION];

/// Gives England's yearly awards on their dates, moves the month awards on at the season's end and runs the base check.
///
/// `EnglandAwards::daily_update(today)`, the virtual +4 of England's awards (from `award_manager_process_date`).
/// - 5 April: the player of the year (award 0) and the young player of the year (award 1, up to 21: filters −1, 0,
///   0x11, −1... and 0x15, 0, 0x11, −1...) from the Premier Division's stats (`award_player_of_year`), then the team of
///   the season of each division (awards 7, 13, 19, 25) and, with the lower divisions (`awards_lower_divisions`), the
///   Conference (award 31).
/// - 30 May: the manager of the year of each division (awards 6, 12, 18, 24; the Conference's 30).
/// - The day of the year the nation's season ends (`key_nation_record::SEASON_END_DAY` of the nation's
///   `G_LEAGUE_TABLE` row, `exe::key_nation_index_of_nation`, signed day against the unsigned end day): the month
///   awards (3, 9, 15, 21, then 4, 5, 10, 11, 16, 17, 22, 23; with the lower divisions 28, 29, 27) start the new season
///   with today's year.
/// - Always: the base month change check (`NationAwards::daily_update`).
/// Original: `EnglandAwards::daily_update` (0x579610, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 5, 3) {
        covered!("sx03_2 england year awards");
        award_player_of_year(this, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, today, nation_award(this, 0), [-1, 0, 0x11, -1, -1, -1, -1]);
        award_player_of_year(this, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, today, nation_award(this, 1), [0x15, 0, 0x11, -1, -1, -1, -1]); // sx03_2: young player up to 21
        for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
            award_team_of_season(this, *key, nation_award(this, 7 + 6 * k));
        }
        if awards_lower_divisions(this) {
            award_team_of_season(this, KEY_CLUB_COMP_ENGLISH_CONFERENCE, nation_award(this, 31));
        }
    }
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx03_2 england managers of the year");
        for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
            award_manager_of_year(this, today, nation_award(this, 6 + 6 * k), *key); // sx03_2: manager of the year
        }
        if awards_lower_divisions(this) {
            award_manager_of_year(this, today, nation_award(this, 30), KEY_CLUB_COMP_ENGLISH_CONFERENCE);
        }
    }
    let row = G_LEAGUE_TABLE.wrapping_add((exe::key_nation_index_of_nation(read_usize(this + NATION_AWARDS_NATION)) as isize * 0x48) as usize);
    if read_i16(today) as i32 == read_u16(row + key_nation_record::SEASON_END_DAY) as i32 {
        covered!("sx03_2 england month awards new season");
        let year = read_u16(today + cm_date::YEAR);
        for k in [3, 9, 15, 21, 4, 5, 10, 11, 16, 17, 22, 23] {
            month_award_start_new_season(nation_award(this, k), year); // sx03_2: month awards new season
        }
        if awards_lower_divisions(this) {
            for k in [28, 29, 27] {
                month_award_start_new_season(nation_award(this, k), year);
            }
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_DAILY_UPDATE);
    base(this, today);
}

// simplification: the awards post news, write history records and raise reputations: the routine cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ENGLAND_AWARDS_DAILY_UPDATE)(this, today)
}

/// Gives England's month awards at the turn of the month: the managers, then the players.
///
/// `EnglandAwards::on_month_change()`, the virtual +0xc of England's awards (from `NationAwards::daily_update` when the
/// month changes, before the month ratings are reset). The manager of the month (`award_month_manager`) of each
/// division (awards 3, 9, 15, 21) and, with the lower divisions, the Conference (27); then the player of the month and
/// the young player of the month (up to 20) of each division (awards 4 and 5, 10 and 11, 16 and 17, 22 and 23; the
/// Conference's 28 and 29) from the month ratings (`award_month_player`).
/// Original: `EnglandAwards::on_month_change` (0x579900, england_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn england_awards_on_month_change(this: usize) {
    for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
        award_month_manager(nation_award(this, 3 + 6 * k), *key); // sx03_2: manager of the month
    }
    let lower = awards_lower_divisions(this);
    if lower {
        covered!("sx03_2 conference manager of the month");
        award_month_manager(nation_award(this, 27), KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    }
    covered!("sx03_2 england month players");
    for k in 0..4 {
        award_month_player(this, nation_award(this, 4 + 6 * k), -1);
        award_month_player(this, nation_award(this, 5 + 6 * k), 0x14); // sx03_2: young player of the month
    }
    if awards_lower_divisions(this) {
        award_month_player(this, nation_award(this, 28), -1);
        award_month_player(this, nation_award(this, 29), 0x14);
    }
}

// simplification: the month winners get news, history records and reputation: the routine cannot run twice. Original
// only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENGLAND_AWARDS_ON_MONTH_CHANGE)(this)
}

/// Builds the team of the week of an award when it is the team award of one of England's divisions.
///
/// `EnglandAwards::update_team_of_the_week(award)`, the virtual +0x10 of England's awards. The team awards are 2, 8, 14,
/// 20 and, with the lower divisions, 26 (the Conference); for one of them, `build_team_of_the_week` with the award's
/// competition (+9).
/// Original: `EnglandAwards::update_team_of_the_week` (0x579af0, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_update_team_of_the_week(this: usize, award: usize) {
    let team_award = [2, 8, 14, 20].iter().any(|&k| nation_award(this, k) == award) // sx03_2: the division team awards
        || awards_lower_divisions(this) && nation_award(this, 26) == award;
    if team_award {
        covered!("sx03_2 england team of the week");
        build_team_of_the_week(this, award);
    }
}

// simplification: the team of the week allocates its candidate lists and writes the nominations: it cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_update_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ENGLAND_AWARDS_UPDATE_TEAM_OF_THE_WEEK)(this, award)
}

// ---------- sx03_3: the Europe-wide transfer rules: building them, the default squad limits and who may not play in Europe (european_awards.cpp) ----------

/// TRANSFER_RULES::get_ineligibility_code (0x8e8a60, thiscall(player, fixture), ret 8, al): the base eligibility.
const FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE: usize = 0x8e8a60;

/// Builds the Europe-wide transfer rules: the base rules, the Europe vtable and a cleared window count.
///
/// `EUROPE_RULES::EUROPE_RULES(a1..a5)`, once, when the transfer manager builds its rules (0x8d26fd). The base
/// constructor (`exe::new_transfer_rules`) with the five arguments, then the vtable 0x96a188 and the byte +8 (the
/// transfer window count) at 0; the window table (+4) stays as the base left it. Returns the object.
/// Original: `EUROPE_RULES::EUROPE_RULES` (0x583280, european_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_europe_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx03_3 europe rules");
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x96a188); // sx03_3: the Europe vtable
    write_u8(this + 8, 0);
    this
}

/// Comparer for `construct_europe_rules` (`compare_transfer_rules_ctor`: the original on the object, the copy on a
/// copy of its first bytes).
pub(crate) unsafe extern "thiscall" fn compare_construct_europe_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_EUROPE_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_europe_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_europe_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// The default squad limits of a match: none.
///
/// `TRANSFER_RULES::fill_squad_restrictions(fixture, out)`, the vtable slot `transfer_rules_vtable::FILL_SQUAD_RESTRICTIONS`
/// of the rules without limits of their own (Europe's among them). The 42-byte restriction record `out` is filled with
/// 0xff (ten dwords and a word); the fixture is not read.
/// Original: `TRANSFER_RULES::fill_squad_restrictions` (0x5832c0, european_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn default_squad_restrictions(_this: usize, _fixture: usize, out: usize) {
    covered!("sx03_3 default squad restrictions");
    for k in 0..42 { // sx03_3: the 42-byte record
        write_u8(out + k, 0xff);
    }
}

/// Comparer for `default_squad_restrictions`: side by side over the 42 bytes.
pub(crate) unsafe extern "thiscall" fn compare_default_squad_restrictions(this: usize, fixture: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_DEFAULT_SQUAD_RESTRICTIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(out, 42)], || o(this, fixture, out), || default_squad_restrictions(this, fixture, out));
    SCOREBOARD.record(equal, || format!("default_squad_restrictions original={:x?} rust={:x?}", mo, mr));
}

/// Why a player may not play a European match, or 0: the base rules, plus the UEFA rule that a player over 19
/// already in the squad of another club's European campaign is cup-tied (8).
///
/// `EUROPE_RULES::get_ineligibility_code(player, fixture)`, the vtable slot
/// `transfer_rules_vtable::GET_INELIGIBILITY_CODE` of the Europe rules. The base answer
/// (`TRANSFER_RULES::get_ineligibility_code`) for: a virtual person (id at or above `G_STAFF_COUNT`); a player under
/// 20 (`staff::AGE`, signed byte; a missing player is read anyway, as in the original); the Super Cup; a competition
/// that is not the Intertoto, the UEFA Cup, the Champions League or the Cup Winners' Cup; a competition whose season
/// year (`cup::YEAR` of its object in `G_COMP_OBJECT_TABLE`) is not the word at the start of the squad manager
/// (`G_SQUAD_MANAGER`). Otherwise, by the round (`fixture::ROUND_NAME`, without its variant digit:
/// `exe::comp_util_get_base_round_id`): in the Intertoto, round 0x96 gives the European rule and the others the base
/// answer; elsewhere rounds 0xe6 and 0xa0 give 0 and the others the European rule. The European rule: 8 when the
/// player's `staff::SQUAD_SELECTED_FOR` is not 2, else 0.
/// Original: `EUROPE_RULES::get_ineligibility_code` (0x5832e0, european_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn europe_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let base = || {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_GET_INELIGIBILITY_CODE);
        f(this, player, fixture)
    };
    let person = Staff(player);
    if player != 0 && read_i32(player) >= read_i32(G_STAFF_COUNT) {
        return base();
    }
    if (person.age() as i8) < 0x14 { // sx03_3: players under 20
        return base();
    }
    let game = Fixture(fixture);
    let comp = read_u32(game.competition().addr());
    if comp == read_u32(KEY_CLUB_COMP_EUROPEAN_SUPER_CUP) {
        return base();
    }
    let european = [KEY_CLUB_COMP_INTER_TOTO_CUP, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP];
    if !european.iter().any(|&k| read_u32(k) == comp) {
        return base();
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp as usize * 4);
    if Cup(object).year() as i32 != read_u16(read_usize(G_SQUAD_MANAGER)) as i32 {
        return base();
    }
    let round = || exe::comp_util_get_base_round_id(game.round_name() as u16 as u32) as u16;
    if comp == read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        if round() != 0x96 {
            return base();
        }
    } else if round() == 0xe6 || round() == 0xa0 {
        covered!("sx03_3 rounds outside the rule");
        return 0;
    }
    covered!("sx03_3 cup-tied rule");
    if person.squad_selected_for() != 2 { 8 } else { 0 } // sx03_3: cup-tied
}

/// Comparer for `europe_rules_get_ineligibility_code`: the code (al) of both.
pub(crate) unsafe extern "thiscall" fn compare_europe_rules_get_ineligibility_code(this: usize, player: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_EUROPE_RULES_GET_INELIGIBILITY_CODE)(this, player, fixture);
    let c = europe_rules_get_ineligibility_code(this, player, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("europe_rules_get_ineligibility_code(player {:#x}, fixture {:#x}) original={} rust={}", player, fixture, r as u8, c as u8));
    r
}

// ---------- sx03_4: the international player awards, the league matches in the month ratings and Denmark's transfer rules (international_awards.cpp, finland_awards.cpp, denmark_awards.cpp) ----------

/// NationAwards::award_player_of_year_from_ratings (0x7683c0, thiscall(today, award, year ratings, 7 filters), ret 0x28).
const FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS: usize = 0x7683c0;
/// nation_awards_award_from_shortlist (0x768b20, thiscall(today, award), ret 8).
const FN_NATION_AWARDS_AWARD_FROM_SHORTLIST: usize = 0x768b20;
/// nation_awards_award_top_three_from_ratings (0x768720, thiscall(today, award, year ratings, continent, a, b, c), ret 0x1c).
const FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS: usize = 0x768720;
/// NationAwards::record_match_ratings (0x7683a0, thiscall(fixture, record), ret 8).
const FN_NATION_AWARDS_RECORD_MATCH_RATINGS: usize = 0x7683a0;

/// The player of the year from the award manager's world year ratings, with the award's position filter (2 or 3).
unsafe fn award_world_player_from_ratings(this: usize, today: usize, award: usize, filter: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS);
    let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10); // the world year ratings
    f(this, today, award, ratings, -1, -1, -1, -1, -1, filter, -1)
}

/// Gives the world and continental player awards on their dates, outside 2001.
///
/// `InternationalAwards::daily_update(today)`, the virtual +4 of the international awards (from
/// `award_manager_process_date`). Nothing in 2001 (`cm_date::YEAR` 0x7d1). On 6 May, the world player of the year
/// (award 0) from the award manager's world year ratings (+0x10), filter 2; on 6 June, award 0 is decided from its
/// shortlist (`nation_awards_award_from_shortlist`) and award 1 is given from the same ratings, filter 3; on 16 June,
/// the best three of Africa, Europe, South America and Oceania (awards 2 to 5,
/// `nation_awards_award_top_three_from_ratings` with the continent key and 0, 1, 0). No base month check.
/// Original: `InternationalAwards::daily_update` (0x632e20, international_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn international_awards_daily_update(this: usize, today: usize) {
    if read_u16(today + cm_date::YEAR) == 0x7d1 {
        return;
    }
    if awards_day_is(today, 6, 4) {
        covered!("sx03_4 world player of the year");
        award_world_player_from_ratings(this, today, nation_award(this, 0), 2); // sx03_4: 6 May
    }
    if awards_day_is(today, 6, 5) {
        let shortlist: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_FROM_SHORTLIST);
        shortlist(this, today, nation_award(this, 0));
        award_world_player_from_ratings(this, today, nation_award(this, 1), 3);
    }
    if awards_day_is(today, 0x10, 5) {
        let top_three: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TOP_THREE_FROM_RATINGS);
        let ratings = read_usize(read_usize(G_AWARD_MANAGER) + 0x10);
        for (k, continent) in [KEY_CONTINENT_AFRICA, KEY_CONTINENT_EUROPE, KEY_CONTINENT_SOUTH_AMERICA, KEY_CONTINENT_OCEANIA].iter().enumerate() {
            top_three(this, today, nation_award(this, 2 + k), ratings, read_u32(*continent), 0, 1, 0);
        }
    }
}

// simplification: the awards post news, write history records and raise reputations: the routine cannot run twice.
// Original only; the copy is proven by the career (6 May 2002).
pub(crate) unsafe extern "thiscall" fn compare_international_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_INTERNATIONAL_AWARDS_DAILY_UPDATE)(this, today)
}

/// Passes a played match to the month ratings when its competition is a league division.
///
/// `FinlandAwards::record_match_ratings(fixture, record)` in the map, the virtual +8 of England's and Finland's awards
/// (from `award_manager_process_fixture`). When the fixture's competition id is a league division
/// (`exe::comp_util_is_league_division`), `NationAwards::record_match_ratings(fixture, record)` (the month ratings, when
/// the object has them).
/// Original: `FinlandAwards::record_match_ratings` (0x5a96b0, finland_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_record_league_match_ratings(this: usize, fixture: usize, record: usize) {
    if exe::comp_util_is_league_division(read_i32(Fixture(fixture).competition().addr())) != 0 { // sx03_4: league divisions only
        covered!("sx03_4 league match rated");
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATION_AWARDS_RECORD_MATCH_RATINGS);
        f(this, fixture, record);
    }
}

// simplification: the month ratings add entries (they may allocate): it cannot run twice. Original only; the copy is
// proven by the career (the player of the month).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_league_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_NATION_AWARDS_RECORD_LEAGUE_MATCH_RATINGS)(this, fixture, record)
}

/// Builds Denmark's transfer rules: the base rules, Denmark's vtable and no transfer windows, so Danish clubs sign at
/// any time.
///
/// `DENMARK_RULES::DENMARK_RULES(a1..a5)`, once, when the transfer manager builds its rules (0x8d2892). The base
/// constructor (`exe::new_transfer_rules`), the vtable 0x969860, the window count (+8) and the window table (+4) at 0.
/// Returns the object.
/// Original: `DENMARK_RULES::DENMARK_RULES` (0x555460, denmark_awards.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    covered!("sx03_4 denmark rules");
    exe::new_transfer_rules(this, a1, a2, a3, a4, a5);
    write_u32(this, 0x969860);
    write_u8(this + 8, 0); // sx03_4: no windows
    write_u32(this + 4, 0);
    this
}

/// Comparer for `construct_denmark_rules` (`compare_transfer_rules_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_denmark_rules(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_DENMARK_RULES);
    let mut r = 0usize;
    compare_transfer_rules_ctor("construct_denmark_rules", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        construct_denmark_rules(s, a1, a2, a3, a4, a5);
    });
    r
}

/// How far news about a player reaches the human managers (flag 0x10 of `collect_player_news_recipients`), worked out
/// from his reputations: every human manager, those of the same continent, of the same nation (England and Scotland
/// count as one), or those whose club plays in the player's club's division.
#[derive(Default)]
pub(crate) struct PlayerNewsReach {
    everyone: bool,
    continental: bool,
    national: bool,
    division: bool,
    /// The continent of the player's nation and of his club job's nation.
    continents: [usize; 2],
    /// The player's nation (by home reputation) and his contract club's nation (by current reputation or form).
    nations: [usize; 2],
    /// The division of the player's contract club (not a type-5 contract).
    league: usize,
}

/// The nation of the player's contract club (not a type-5 contract), 0 without one; two contract lookups, as the
/// original does (the first only checks that there is a club).
unsafe fn player_contract_club_nation(player: Staff) -> usize {
    if contract_club_excluding_type5(player.addr()) == 0 {
        return 0;
    }
    Club(contract_club_excluding_type5(player.addr())).nation().addr()
}

/// The reach of news about `player` (see `collect_player_news_recipients`, flag 0x10).
unsafe fn player_news_reach(player: Staff, flags: u32) -> PlayerNewsReach {
    let mut r = PlayerNewsReach::default();
    if contract_club_excluding_type5(player.addr()) != 0 {
        r.league = Club(contract_club_excluding_type5(player.addr())).division().addr();
    }
    let sheet = player.player();
    let non_player = player.non_player();
    let (current, home, world) = if !sheet.is_null() {
        (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation())
    } else if !non_player.is_null() {
        (non_player.current_reputation(), non_player.home_reputation(), non_player.world_reputation())
    } else {
        (0, 0, 0)
    };
    // flag 0x200: the higher thresholds (everyone, continent, nation) and the gentler form slope
    let wide = flags & 0x200 != 0;
    let (everyone, continent, nation, slope) = if wide {
        (0x251c, 0x2328, 0x206c, read_f64(0x970430)) // f64 0.00015
    } else {
        (0x23f0, 0x206c, 0x1c20, read_f64(rdata::F64_0_00025))
    };
    // in form: the average rating (PlayerStats category 2, field 0x12) above `base` - current reputation x slope
    let in_form = |base: f64| {
        !sheet.is_null() && exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sheet, 2, 0x12, 6.5) > base - current as i32 as f64 * slope
    };
    if world > everyone {
        r.everyone = true;
    } else if world > continent {
        r.continental = true;
        let n = player.nation();
        if !n.is_null() {
            r.continents[0] = n.continent();
        }
        let club = player.club_job();
        if !club.is_null() && !club.nation().is_null() {
            r.continents[1] = club.nation().continent();
        }
    } else if home > nation || current > nation {
        covered!("sx02_1 national reach");
        r.national = true; // sx02_1: national by reputation
        if home > nation {
            r.nations[0] = player.nation().addr();
        }
        if current > nation {
            r.nations[1] = player_contract_club_nation(player);
        }
    } else if in_form(read_f64(0x970428)) && sheet.current_reputation() > 3000 && flags & 0x80 != 0 {
        // f64 9.25: a player in form with some reputation reaches his club's nation
        r.national = true;
        r.nations[1] = player_contract_club_nation(player);
    } else if flags & 0x40 != 0 && r.league != 0 {
        let reputation = Competition(r.league).reputation() as i32;
        let current = current as i32;
        // f64 9.0 for the form
        r.division = if wide {
            current > (reputation * 750).max(5000) || in_form(read_f64(0x96d230)) // sx02_1: division by reputation
        } else {
            (current > reputation * 350 && current > 2500) || in_form(read_f64(0x96d230)) // sx02_1: division form
        };
    }
    r
}

/// Whether the reach takes the human `manager` of `club`.
unsafe fn player_news_reach_takes(r: &PlayerNewsReach, manager: Staff, club: Club) -> bool {
    let (nation, club_nation) = (manager.nation(), club.nation());
    if r.everyone {
        return true;
    }
    if r.continental {
        let a = if nation.is_null() { 0 } else { nation.continent() };
        let b = if club_nation.is_null() { 0 } else { club_nation.continent() };
        return a != 0 && r.continents.contains(&a) || b != 0 && r.continents.contains(&b);
    }
    if r.national {
        let (n, c) = (nation.addr(), club_nation.addr());
        if n != 0 && r.nations.contains(&n) || c != 0 && r.nations.contains(&c) {
            return true;
        }
        if n == 0 {
            return false;
        }
        // England and Scotland reach each other through the manager's own nation
        let (id, england, scotland) = (read_usize(n), read_usize(KEY_NATION_ENGLAND), read_usize(KEY_NATION_SCOTLAND));
        let any_is = |key: usize| r.nations.iter().any(|&x| x != 0 && read_usize(x) == key);
        return id == england && any_is(scotland) || id == scotland && any_is(england);
    }
    r.division && club.division().addr() == r.league
}

/// Appends `club` to the recipients array `out` at `n` and counts it.
unsafe fn push_recipient(out: *mut usize, n: &mut i8, club: usize) {
    *out.add(*n as usize) = club;
    *n += 1;
}

/// Lists the human-managed clubs that should receive news about a player, into `out` (up to `max`), and returns how
/// many.
///
/// - Flag 2: the club of the player's contract (any contract type, `contract_club_any_type`, which stores the lookup in
/// `G_CONTRACT_LOOKUP_RESULT`) when a human manages it (`exe::news_club_has_human_manager` on `G_NEWS`) goes first.
/// - Flag 8: the clubs shortlisting him (`exe::shortlist_manager_collect_interested_clubs` of the manager's
/// `shortlist_manager()`, with 1, 1, 0, into a local list) managed by a human; past `max`, the "Error" box (line 0x3aa4)
/// for each one left out.
/// - Flag 0x10: each of the 16 human manager slots (the last 16 people of the table, from the last one) with a
/// `classification()` and a `club_job()`, when the player's reach (`player_news_reach`) takes him. From his
/// reputations (the player sheet's, otherwise the non-player's; flag 0x200 raises the thresholds): world reputation
/// above 9500 (0x200) or 9200: everyone; above 9000 or 8300: the managers whose nation or club nation is on the
/// continent of the player's nation or of his club job's nation; home or current reputation above 8300 or 7200: the
/// managers whose nation or club nation is the player's nation (by home reputation) or his contract club's nation (by
/// current reputation), England and Scotland reaching each other; otherwise, a player in form (average rating above 9.25
/// - current reputation x 0.00015, or x 0.00025 without 0x200) with current reputation above 3000 and flag 0x80: the
/// contract club's nation; otherwise with flag 0x40: the managers whose club is in the player's contract club's division,
/// when his current reputation is above the division's reputation x 750 (at least 5000; with 0x200) or above the
/// division's reputation x 350 and 2500 (without), or he is in form against 9.0. A full list: the "Error" box (line
/// 0x3b43) and the slots stop.
/// - Then the duplicates are dropped: each later copy of an entry is replaced by the last entry.
///
/// Defect of the original: after a duplicate is replaced by the last entry, the next slot is checked, not the moved
/// entry, so a club listed three times (by flags 2, 8 and 0x10) can stay twice.
/// Original: `transfer_manager_collect_player_news_recipients` (0x8d1da0, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn collect_player_news_recipients(this: usize, player: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let this = TransferManagerFields(this);
    let player = Staff(player);
    let max = max as u8 as i8;
    let mut n: i8 = 0;
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    if flags & 2 != 0 && contract_club_any_type(player) != 0
        && exe::news_club_has_human_manager(read_usize(G_NEWS), contract_club_any_type(player)) != 0 {
        *out = contract_club_any_type(player);
        n = 1;
    }
    if flags & 8 != 0 {
        exe::shortlist_manager_collect_interested_clubs(this.shortlist_manager(), player, l, 1, 1, 0);
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let club = transfer_manager_club_by_id(read_usize(e + 4)); // list entry +4: the club id
            if exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
                if n < max {
                    covered!("sx02_1 shortlisting human club");
                    push_recipient(out, &mut n, club.addr()); // sx02_1: a shortlisting human club
                } else {
                    show_error_box(0xa8d514, 0x3aa4);
                }
            }
        }
    }
    if flags & 0x10 != 0 {
        let reach = player_news_reach(player, flags);
        for k in 0..16 {
            if n == max {
                show_error_box(0xa8d514, 0x3b43);
                break;
            }
            let manager = staff_at((read_i32(G_STAFF_COUNT) - k - 1) as isize);
            if manager.classification() == 0 || manager.club_job().is_null() {
                continue;
            }
            if player_news_reach_takes(&reach, manager, manager.club_job()) {
                push_recipient(out, &mut n, manager.club_job().addr());
            }
        }
    }
    // duplicates: a later copy is replaced by the last entry (the moved entry is not checked again)
    let mut i: i8 = 0;
    if n - 1 > 0 {
        loop {
            let mut j = i + 1;
            while j < n {
                if *out.add(i as usize) == *out.add(j as usize) {
                    n -= 1;
                    *out.add(j as usize) = *out.add(n as usize);
                }
                j += 1;
            }
            i += 1;
            if i >= n - 1 {
                break;
            }
        }
    }
    exe::drop_llist(l);
    n as u8
}

/// Comparer for `collect_player_news_recipients`: side by side from the same state of `G_CONTRACT_LOOKUP_RESULT` and of
/// the output array (`max` entries); checks the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_collect_player_news_recipients(this: usize, player: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8>(IDX_COLLECT_PLAYER_NEWS_RECIPIENTS);
    let slots = (max as u8 as i8).max(0) as usize;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out as usize, 4 * slots)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, player, flags, out, max),
        || collect_player_news_recipients(this, player, flags, out, max));
    SCOREBOARD.record(equal && r == n, || format!(
        "collect_player_news_recipients(player {:#x}, flags {:#x}): original {} rust {} {:x?} x {:x?}", player, flags, r, n, mo, mr));
    r
}

/// How far news about an offer reaches the human managers (flag 0x10 of `collect_offer_news_recipients`), worked out
/// from the target's reputations and the offer's value.
#[derive(Default)]
pub(crate) struct OfferNewsReach {
    everyone: bool,
    continental: bool,
    national: bool,
    target_division: bool,
    buyer_division: bool,
    /// The continent of the target's nation, of the seller's nation and of the buyer's nation.
    continents: [usize; 3],
    /// The target's nation (by home reputation), the seller's nation (by current reputation or form) and the buyer's
    /// nation (a big name, a big fee or a capped international).
    nations: [usize; 3],
    /// The division of the target's club job.
    target_league: usize,
    /// The division of the buying club.
    buyer_league: usize,
}

/// The reputation of `target` as a fake person of the buying club sees it (`exe::database_get_staff_reputation_for_viewer`
/// with a person record on the stack: id 0, the buyer as club job, the buyer's nation, no second nation, no player or
/// non-player sheet, the five dates constructed; the rest is left uninitialised by the original and is not read).
unsafe fn reputation_seen_by_club(target: Staff, buyer: Club) -> i32 {
    let mut viewer = [0u8; staff::SIZE];
    let v = viewer.as_mut_ptr() as usize;
    // +0x31: the date after DATE_JOINED_NATION [inference: the nation contract end]
    for date in [staff::DATE_OF_BIRTH, staff::DATE_JOINED_NATION, 0x31, staff::DATE_JOINED_CLUB, staff::DATE_EXPIRES_CLUB] {
        exe::date_date_54a110(v + date);
    }
    write_usize(v + staff::CLUB_JOB, buyer.addr());
    write_usize(v + staff::NATION, buyer.nation().addr());
    write_usize(v + staff::NON_PLAYER, 0);
    write_usize(v + staff::PLAYER, 0);
    write_usize(v + staff::SECOND_NATION, 0);
    write_usize(v, 0);
    exe::database_get_staff_reputation_for_viewer(target, v, target.player().is_null() as u32) as i32
}

/// The reach of news about offer `o` (see `collect_offer_news_recipients`, flag 0x10).
unsafe fn offer_news_reach(this: TransferManagerFields, o: Offer, target: Staff, seller: usize, buyer: Club, flags: u32) -> OfferNewsReach {
    let mut r = OfferNewsReach::default();
    // a player job, the buyer shortlisting him in a first-team slot (slot type below 3) and a fee of a million or more
    let entry = exe::shortlist_manager_find_club_shortlist_entry(this.shortlist_manager(), buyer, target, 0, 0);
    r.buyer_division = exe::database_is_player_job(o.job_type() as i8) != 0 && entry != 0
        && read_u8(entry + shortlist_slot::FLAGS) & 0x1e < 6 && o.fee() as i32 >= 1_000_000;
    if !target.club_job().is_null() {
        r.target_league = target.club_job().division().addr();
    }
    r.buyer_league = buyer.division().addr();
    let sheet = target.player();
    let non_player = target.non_player();
    let (mut current, mut home, mut world) = (0i16, 0i16, 0i16);
    if !sheet.is_null() {
        (current, home, world) = (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation());
    } else if !non_player.is_null() {
        (current, home, world) = (non_player.current_reputation(), non_player.home_reputation(), non_player.world_reputation());
        if matches!(o.job_type(), 8..=10) {
            // jobs 8 to 10: a quarter of the reputations
            let quarter = |x: i16| ftol_x87(x as i32 as f64 * read_f64(rdata::F64_0_25)) as i16;
            (current, home, world) = (quarter(current), quarter(home), quarter(world));
        }
    }
    // the bonus: 3000 - offer value x 0.0006 (500 for a pre-contract); each threshold is at most 9750
    let bonus = if o.pre_contract() == 0 {
        let value = exe::transfer_offer_get_weighted_value(o, 0, 0, 0, 0, 1);
        ftol_x87(read_f64(0x96d200) - value as f64 * read_f64(0x96a610)) as i16 as i32 // f64 3000.0, 0.0006
    } else {
        500
    };
    let cap = |t: i32| t.min(0x2616);
    let (current, home, world) = (current as i32, home as i32, world as i32);
    let national = cap(bonus + 0x1c20);
    let in_form = |base: f64| {
        !sheet.is_null() && exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sheet, 2, 0x12, 6.5) > base - current as f64 * read_f64(rdata::F64_0_00025)
    };
    if world > cap(bonus + 0x23f0) {
        r.everyone = true;
    } else if world > cap(bonus + 0x206c) {
        r.continental = true;
        if !target.nation().is_null() {
            r.continents[0] = target.nation().continent();
        }
        if seller != 0 && !Club(seller).nation().is_null() {
            r.continents[1] = Club(seller).nation().continent();
        }
        if !buyer.nation().is_null() {
            r.continents[2] = buyer.nation().continent();
        }
    } else if home > national || current > national {
        covered!("sx02_2 national reach");
        r.national = true; // sx02_2: national by reputation
        if home > national {
            r.nations[0] = target.nation().addr();
        }
        if current > national && seller != 0 {
            r.nations[1] = Club(seller).nation().addr();
        }
        let buyer_nation = buyer.nation();
        if buyer_nation.addr() != r.nations[0] && buyer_nation.addr() != r.nations[1]
            && (world > national
                || exe::transfer_offer_get_weighted_value(o, 0, 0, 0, 0, 1) >= (buyer_nation.league_standard() as i8 as i32 - 10) * 500_000
                || !target.national_job().is_null() && target.int_apps() > 20 && target.national_job().reputation() as i16 > 5500) {
            // sx02_2: the buyer's nation for a big name, a big fee for its league or a capped international
            r.nations[2] = buyer_nation.addr();
        }
    } else if in_form(read_f64(0x970428)) && current > 3000 && flags & 0x80 != 0 {
        // f64 9.25: a player in form with some reputation reaches the seller's nation
        r.national = true;
        if seller != 0 {
            r.nations[1] = Club(seller).nation().addr();
        }
    } else if flags & 0x40 != 0 {
        if r.target_league != 0 {
            let limit = Competition(r.target_league).reputation() as i32 * 350;
            // f64 9.0 for the form
            if current > limit && current > 2500 || in_form(read_f64(0x96d230)) {
                r.target_division = true; // sx02_2: the target's division
            }
        }
        if r.buyer_league != 0 && reputation_seen_by_club(target, buyer) > Competition(r.buyer_league).reputation() as i32 * 350 && current > 1500 {
            r.buyer_division = true;
        }
    }
    r
}

/// Whether the offer's reach takes the human `manager` of `club`.
unsafe fn offer_news_reach_takes(r: &OfferNewsReach, manager: Staff, club: Club) -> bool {
    let (nation, club_nation) = (manager.nation(), club.nation());
    if r.everyone {
        return true;
    }
    if r.continental {
        let a = if nation.is_null() { 0 } else { nation.continent() };
        let b = if club_nation.is_null() { 0 } else { club_nation.continent() };
        return a != 0 && r.continents.contains(&a) || b != 0 && r.continents.contains(&b);
    }
    if r.national {
        let (n, c) = (nation.addr(), club_nation.addr());
        if n != 0 && r.nations.contains(&n) || c != 0 && r.nations.contains(&c) {
            return true;
        }
        if n == 0 {
            return false;
        }
        // England and Scotland reach each other through the manager's own nation
        let (id, england, scotland) = (read_usize(n), read_usize(KEY_NATION_ENGLAND), read_usize(KEY_NATION_SCOTLAND));
        let any_is = |key: usize| r.nations.iter().any(|&x| x != 0 && read_usize(x) == key);
        return id == england && any_is(scotland) || id == scotland && any_is(england);
    }
    let league = club.division().addr();
    r.target_division && league == r.target_league || r.buyer_division && league == r.buyer_league
}

/// Lists the human-managed clubs that should receive news of an offer, into `out` (up to `max`), and returns how many.
///
/// The target is `target_staff_id()`, the buyer `buyer_club_id()` and the seller the club of the target's contract
/// (not a type-5 contract; the lookup stored in `G_CONTRACT_LOOKUP_RESULT`).
/// - Flag 2: the seller when a human manages it (`exe::news_club_has_human_manager` on `G_NEWS`) goes first.
/// - Flag 1: the buyer when a human manages it.
/// - Flag 4: with a loan contract (`exe::contract_manager_get_loan_contract`) at a club other than the seller, the club
/// linked to the target's club job (`exe::club_get_linked_b_team` with 1, when its flag comes back 0) or the club job
/// itself, human or not; then the buyers of the rival offers (the slot list at the offer's +0) that are not closed
/// (`offer_is_closed`) and not this offer's slot, when a human manages them; past `max`, the "Error" box (line
/// 0x3936).
/// - Flag 8: with such a loan, the target's club job, human or not; then the human clubs shortlisting the target
/// (`exe::shortlist_manager_collect_interested_clubs` with 1, 1, 1) other than the buyer; past `max`, the box (line
/// 0x3959).
/// - Flag 0x10 (with 0x20, only when `player_aware()`): each of the 16 human manager slots (the last 16 people of the
/// table) with a `classification()` and a `club_job()`, not the buyer without flag 1 nor the seller without flag 2,
/// when the offer's reach (`offer_news_reach`) takes him. The target's reputations (the player sheet's, otherwise the
/// non-player's, a quarter for jobs 8 to 10) against thresholds raised by 3000 - the offer's weighted value
/// (`exe::transfer_offer_get_weighted_value` with 0, 0, 0, 0, 1) x 0.0006 (500 for a `pre_contract()`), each at most
/// 9750: world above 9200 + bonus, everyone; above 8300 + bonus, the managers whose nation or club nation is on the
/// continent of the target's, the seller's or the buyer's nation; home or current above 7200 + bonus, those of the
/// target's nation (home) or the seller's (current), England and Scotland reaching each other, and the buyer's nation
/// unless it is one of those, for a world reputation above the same threshold, a value of at least (the nation's
/// league standard - 10) x 500000 or a target with more than 20 caps for a national side above 5500 reputation;
/// otherwise a target in form (average rating above 9.25 - current reputation x 0.00025) above 3000 with flag 0x80:
/// the seller's nation; otherwise with flag 0x40: the managers in the division of the target's club job (current
/// reputation above its reputation x 350 and 2500, or in form against 9.0) and in the buyer's division (a player job
/// shortlisted by the buyer in a slot of type below 3 for a fee of a million or more, or the reputation a person of the
/// buying club sees, `reputation_seen_by_club`, above the division's reputation x 350 with current reputation above
/// 1500). A full list: the box (line 0x39f3) and the slots stop.
/// - Then the later copies of each entry are zeroed and the zeroes squeezed out, keeping the order.
///
/// Defects of the original: the loan clubs of flags 4 and 8 go in without the human check and without the `max` check.
/// Original: `TRANSFER_MANAGER::collect_offer_news_recipients` (0x8d1130, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn collect_offer_news_recipients(this: usize, o: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let this = TransferManagerFields(this);
    let o = Offer(o);
    let max = max as u8 as i8;
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let seller = contract_club_excluding_type5(target.addr());
    let buyer = transfer_manager_club_by_id(o.buyer_club_id());
    let mut n: i8 = 0;
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let human = |club: usize| exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0;
    if flags & 2 != 0 && seller != 0 && human(seller) {
        *out = seller;
        n = 1;
    }
    if flags & 1 != 0 && human(buyer.addr()) {
        *out.add(n as usize) = buyer.addr();
        n += 1;
    }
    // a loan contract at a club other than the seller
    let loan_elsewhere = || {
        exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), target) != 0
            && (seller == 0 || Contract(exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), target)).club() != read_usize(seller))
    };
    if flags & 4 != 0 {
        if loan_elsewhere() {
            let mut flag = 0u32;
            let linked = exe::club_get_linked_b_team(target.club_job(), &mut flag, 1);
            *out.add(n as usize) = if linked != 0 && flag == 0 { linked } else { target.club_job().addr() };
            n += 1;
        }
        let rivals = Llist(read_usize(o.addr())); // offer +0: the slot list of the rival offers
        if !rivals.is_null() {
            rivals.set_cursor(rivals.head());
            loop {
                let e = exe::llist_get_next(rivals);
                if e == 0 {
                    break;
                }
                let other = Offer(offer_at(this.addr(), read_usize(e)));
                if offer_is_closed(other) || read_usize(e) == o.slot_index() {
                    continue;
                }
                let club = transfer_manager_club_by_id(other.buyer_club_id());
                if human(club.addr()) {
                    if n < max {
                        covered!("sx02_2 rival bidder");
                        *out.add(n as usize) = club.addr(); // sx02_2: a rival human bidder
                        n += 1;
                    } else {
                        show_error_box(0xa8d514, 0x3936);
                    }
                }
            }
        }
    }
    if flags & 8 != 0 {
        if !target.club_job().is_null() && loan_elsewhere() {
            *out.add(n as usize) = target.club_job().addr();
            n += 1;
        }
        exe::shortlist_manager_collect_interested_clubs(this.shortlist_manager(), target, l, 1, 1, 1);
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let id = read_usize(e + 4); // list entry +4: the club id
            if id == read_usize(buyer.addr()) {
                continue;
            }
            let club = transfer_manager_club_by_id(id);
            if human(club.addr()) {
                if n < max {
                    covered!("sx02_2 shortlisting human club");
                    *out.add(n as usize) = club.addr(); // sx02_2: a shortlisting human club
                    n += 1;
                } else {
                    show_error_box(0xa8d514, 0x3959);
                }
            }
        }
    }
    if flags & 0x10 != 0 && (flags & 0x20 == 0 || o.player_aware() != 0) {
        let reach = offer_news_reach(this, o, target, seller, buyer, flags);
        for k in 0..16 {
            if n == max {
                show_error_box(0xa8d514, 0x39f3);
                break;
            }
            let manager = staff_at((read_i32(G_STAFF_COUNT) - k - 1) as isize);
            let club = manager.club_job();
            if manager.classification() == 0 || club.is_null()
                || flags & 1 == 0 && club == buyer || flags & 2 == 0 && club.addr() == seller {
                continue;
            }
            if offer_news_reach_takes(&reach, manager, club) {
                *out.add(n as usize) = club.addr();
                n += 1;
            }
        }
    }
    // duplicates: the later copies are zeroed, then the zeroes squeezed out
    let count = n as i32;
    let mut i = 0i32;
    while i < count - 1 {
        if *out.add(i as usize) != 0 {
            for j in i + 1..count {
                if *out.add(j as usize) == *out.add(i as usize) {
                    *out.add(j as usize) = 0;
                }
            }
        }
        i += 1;
    }
    let mut kept: i8 = 0;
    for j in 0..count.max(0) {
        let club = *out.add(j as usize);
        if club != 0 {
            *out.add(kept as usize) = club;
            kept += 1;
        }
    }
    exe::drop_llist(l);
    kept as u8
}

/// Comparer for `collect_offer_news_recipients`: side by side from the same state of `G_CONTRACT_LOOKUP_RESULT` and of
/// the output array (`max` entries); checks the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_collect_offer_news_recipients(this: usize, o: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8>(IDX_COLLECT_OFFER_NEWS_RECIPIENTS);
    let slots = (max as u8 as i8).max(0) as usize;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out as usize, 4 * slots)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || f(this, o, flags, out, max),
        || collect_offer_news_recipients(this, o, flags, out, max));
    SCOREBOARD.record(equal && r == n, || format!(
        "collect_offer_news_recipients(offer {:#x}, flags {:#x}): original {} rust {} {:x?} x {:x?}", o, flags, r, n, mo, mr));
    r
}

// The transfer offer method that drops an exchange player from the offer's exchange list (transfer_offer.cpp,
// thiscall(offer, person), ret 4).
const FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER: usize = 0x8e81a0;

/// Cancels every offer and every exchange offer for a released player, removes their contract offers and posts the
/// release news (0xfb1) to the clubs involved.
///
/// - The offers for him (`exe::transfer_manager_get_offers_for_player` with 0, into a local list): a future transfer
/// (`offer_status::FUTURE_TRANSFER`) loses its record (`exe::transfer_manager_remove_future_transfer`); each one becomes
/// `offer_status::CANCELLED`, is due again in 10 days (`exe::transfer_manager_schedule_offer` with 10, 0), loses its
/// contract offers (`exe::transfer_manager_close_offer_contract_offers` and `exe::contract_manager_remove_contract_offer`
/// for the target and the buyer), `terms_state()` = 0, and each exchange player loses the contract offer of the offer's
/// slot (`exe::contract_manager_remove_contract_offer_for_transfer`) and his "terms offered" byte (+4 of the list item).
/// A human buyer (`exe::news_club_has_human_manager` on `G_NEWS`) gets news 0xfb1: 0..3 the player and his names, 4 the
/// buyer, 5 the club of his contract (not type 5; 0 without one), 0xc = 0.
/// - The offers he is an exchange player in (`exe::transfer_manager_get_exchange_offers_for_player`, into the same list
/// cleared): a human seller (the club of the target's contract) gets news 0xfb1: 0..3 the player, 4 the seller, 5 the
/// club of the player's contract, 6 the fee, 7 the player's `value()` + the fee, 8..0xb the target and his names, 0xc =
/// 1; then he leaves the offer's exchange (`FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER`).
/// - The human clubs with news of him (`collect_player_news_recipients` with 0xa, up to 20) not already told get news
/// 0xfb1: 0..3 the player, 5 the club of his contract, 0xc = 2.
///
/// Defect of the original: news slot 5 reads the id of the contract club through a null pointer when the second
/// contract lookup finds none (the exchange and the last news read it with no check at all); a released player always
/// still has his contract here, so it never shows.
/// Original: `TRANSFER_MANAGER::cancel_offers_for_released_player` (0x8df9b0, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cancel_offers_for_released_player(this: usize, player: usize) {
    let this = TransferManagerFields(this);
    let player = Staff(player);
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let mut told = [0usize; 0x28]; // the original's stack array (0x158..0x1f8)
    let mut k = 0usize;
    let mut news = [0u32; 0x38];
    let n = news.as_mut_ptr() as usize;
    // the id of the club of the player's contract (not type 5), as the original reads it
    let contract_club_id = || read_usize(contract_club_excluding_type5(player.addr())) as u32;
    if exe::transfer_manager_get_offers_for_player(this, player, l, 0) != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this.addr(), read_usize(e)));
            covered!("sx02_3 released player offers");
            if o.status() == offer_status::FUTURE_TRANSFER {
                exe::transfer_manager_remove_future_transfer(this, o);
            }
            o.set_status(offer_status::CANCELLED); // sx02_3: the offer cancelled
            exe::transfer_manager_schedule_offer(this, o, 10, 0);
            exe::transfer_manager_close_offer_contract_offers(this, o);
            let buyer = transfer_manager_club_by_id(o.buyer_club_id());
            exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(o.target_staff_id()), buyer);
            o.set_terms_state(0);
            let exchange = o.exchange_players();
            if !exchange.is_null() {
                exchange.set_cursor(exchange.head());
                loop {
                    let x = exe::llist_get_next(exchange);
                    if x == 0 {
                        break;
                    }
                    exe::contract_manager_remove_contract_offer_for_transfer(read_usize(G_CONTRACT_MANAGER), transfer_manager_staff_by_id(read_usize(x)), o.slot_index());
                    write_u8(x + 4, 0); // exchange item +4: terms offered
                }
            }
            if exe::news_club_has_human_manager(read_usize(G_NEWS), buyer) != 0 {
                exe::new_newsitem(n, 0xfb1, 0);
                set_news_person_slots(n, 0, player);
                set_news_slot(n, 4, o.buyer_club_id() as u32);
                let club = if contract_club_excluding_type5(player.addr()) != 0 { contract_club_id() } else { 0 };
                set_news_slot(n, 5, club);
                set_news_slot(n, 0xc, 0);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, buyer);
                told[k] = buyer.addr();
                k += 1;
            }
        }
    }
    exe::llist_clear(l);
    if exe::transfer_manager_get_exchange_offers_for_player(this, player, l) != 0 {
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let o = Offer(offer_at(this.addr(), read_usize(e)));
            let target = transfer_manager_staff_by_id(o.target_staff_id());
            let seller = contract_club_excluding_type5(target.addr());
            if exe::news_club_has_human_manager(read_usize(G_NEWS), seller) != 0 {
                exe::new_newsitem(n, 0xfb1, 0);
                set_news_person_slots(n, 0, player);
                set_news_slot(n, 4, read_u32(seller));
                set_news_slot(n, 5, contract_club_id());
                set_news_slot(n, 6, o.fee() as u32);
                set_news_slot(n, 7, (player.value() as u32).wrapping_add(o.fee() as u32));
                set_news_person_slots(n, 8, target);
                set_news_slot(n, 0xc, 1);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, seller);
                told[k] = seller;
                k += 1;
            }
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_REMOVE_EXCHANGE_PLAYER);
            remove(o.addr(), player.addr()); // sx02_3: the player leaves the exchange
        }
    }
    let mut clubs = [0usize; 0x14];
    let count = exe::transfer_manager_collect_player_news_recipients(this, player, 0xa, clubs.as_mut_ptr(), 0x14) as i8;
    if count != 0 {
        exe::new_newsitem(n, 0xfb1, 0);
        set_news_person_slots(n, 0, player);
        set_news_slot(n, 5, contract_club_id());
        set_news_slot(n, 0xc, 2);
        for &club in clubs.iter().take(count.max(0) as usize) {
            if !told[..k].contains(&club) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
            }
        }
    }
    exe::drop_llist(l);
}

/// Comparer for `cancel_offers_for_released_player`: only the original runs (it schedules, removes contract offers
/// and posts news, none of which can run twice).
pub(crate) unsafe extern "thiscall" fn compare_cancel_offers_for_released_player(this: usize, player: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CANCEL_OFFERS_FOR_RELEASED_PLAYER)(this, player) // simplification: original only
}

/// The clubs of the `refused` list (club ids) kept for the news of a player's answer: the first three, then each later
/// one replaces the lowest-reputation kept club (the first of equals) when its reputation is higher.
unsafe fn keep_three_refused_clubs(refused: Llist, kept: &mut [usize; 3]) -> usize {
    let mut k = 0usize;
    if refused.is_null() || refused.count() == 0 {
        return 0;
    }
    refused.set_cursor(refused.head());
    loop {
        let e = exe::llist_get_next(refused);
        if e == 0 {
            return k;
        }
        let club = transfer_manager_club_by_id(read_usize(e));
        if k < 3 {
            kept[k] = club.addr();
            k += 1;
            continue;
        }
        let (mut lowest, mut at) = (0x2711i16, None);
        for (i, &c) in kept.iter().enumerate() {
            let reputation = Club(c).reputation() as i16;
            if reputation < lowest {
                (lowest, at) = (reputation, Some(i));
            }
        }
        if club.reputation() as i16 > lowest {
            if let Some(i) = at {
                kept[i] = club.addr(); // sx02_3: a better-known refused club
            }
        }
    }
}

/// The player's interest in joining `club` (`exe::shortlist_manager_staff_interest_in_joining_club` with 0, the loan
/// flag and the squad status of `contract`, the high nibble of `contract::SQUAD_STATUS`): (its out flags, interest x
/// 1000 truncated).
unsafe fn interest_slots(this: TransferManagerFields, person: Staff, club: Club, o: Offer, contract: Contract) -> (u32, u32) {
    let mut flags = 0u32;
    let interest = exe::shortlist_manager_staff_interest_in_joining_club(this.shortlist_manager(), person, club, 0,
        (o.offer_type() == 1) as u32, (contract.squad_status() >> 4) as u32, &mut flags as *mut u32 as u32) as f32;
    (flags, ftol_x87(interest as f64 * read_f64(rdata::F64_1000)) as u32)
}

/// Posts news 0xfa4, a player's answer to a contract offer from `club`, to the human clubs of the offer and, on a
/// refusal of all of them (`answer` 2), to the best-known clubs he turned down.
///
/// The recipients (`exe::transfer_manager_collect_offer_news_recipients`, up to 30): flags 0x2f; with a `contract`,
/// 0x2d; for answer 3 without one, 0x2d if `club` is the buyer, otherwise 0x2e. None: nothing. With answer 2, the
/// `refused` list (club ids) gives up to three clubs (`keep_three_refused_clubs`). News 0xfa4: 0..3 the person and his
/// names, 4 `other` (-1 without), 5 `club`, 6 the answer, 7 `slot_index()`, 8 `serial()`, 9 `offer_type()`, 0xa
/// `value`, 0xb `fee()`, 0xc..0xf the target and his names, 0x10 the contract's club (-1) and 0x11 its type (6 signed
/// bits; 0), 0x12..0x15 the manager of `other` and his names (-1 each), 0x16..0x19 the manager of `club`, 0x1a and 0x1b
/// the interest flags and the interest x 1000 (`interest_slots`, by the `contract` or the contract offer of the person
/// at `club`, `exe::contract_manager_find_contract_offer`; neither: 0, 0), 0x1c the length of `refused` (0), 0x1d.. the
/// kept clubs. Sent to each recipient (a recipient on answer 2 is struck from the kept clubs) and then to each kept club
/// left.
/// Original: `TRANSFER_MANAGER::post_contract_response_news` (0x8d8e90, transfer_manager.cpp, thiscall, ret 0x20).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn post_contract_response_news(this: usize, person: usize, club: usize, other: usize, answer: u32, o: usize,
                                                                   contract: usize, refused: usize, value: u32) {
    let this = TransferManagerFields(this);
    let (person, club, other, o, contract, refused) = (Staff(person), Club(club), Club(other), Offer(o), Contract(contract), Llist(refused));
    let answer = answer as u8;
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let mut kept = [0usize; 3];
    let mut k = 0usize;
    let mut flags = 0x2f;
    if answer == 2 {
        k = keep_three_refused_clubs(refused, &mut kept);
    } else if !contract.is_null() {
        flags = 0x2d;
    } else if answer == 3 {
        flags = 0x2d + (read_usize(club.addr()) != o.buyer_club_id()) as u32; // sx02_3: an answer to the buyer
    }
    let mut recipients = [0usize; 0x1e];
    let count = exe::transfer_manager_collect_offer_news_recipients(this, o, flags, recipients.as_mut_ptr(), 0x1e) as i8;
    if count == 0 {
        return;
    }
    covered!("sx02_3 contract response news");
    let mut news = [0u32; 0x38];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfa4, 0);
    set_news_person_slots(n, 0, person);
    set_news_slot(n, 4, if other.is_null() { u32::MAX } else { read_u32(other.addr()) });
    set_news_slot(n, 5, read_u32(club.addr()));
    set_news_slot(n, 6, answer as i8 as i32 as u32);
    set_news_slot(n, 7, o.slot_index() as u32);
    set_news_slot(n, 8, o.serial() as u32);
    set_news_slot(n, 9, o.offer_type() as i8 as i32 as u32);
    set_news_slot(n, 0xa, value);
    set_news_slot(n, 0xb, o.fee() as u32);
    set_news_person_slots(n, 0xc, target);
    if !contract.is_null() {
        set_news_slot(n, 0x10, contract.club() as u32);
        set_news_slot(n, 0x11, (((contract.contract_type() << 2) as i8) >> 2) as i32 as u32);
    } else {
        set_news_slot(n, 0x10, u32::MAX);
        set_news_slot(n, 0x11, 0);
    }
    // the managers of the other club and of the club
    for (base, c) in [(0x12i8, other), (0x16, club)] {
        let manager = if c.is_null() { Staff(0) } else { c.manager() };
        if manager.is_null() {
            for i in 0..4 {
                set_news_slot(n, base + i, u32::MAX);
            }
        } else {
            set_news_person_slots(n, base, manager);
        }
    }
    let (interest_flags, interest) = if !contract.is_null() {
        interest_slots(this, person, club, o, contract)
    } else {
        let offered = Contract(exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(person.addr()), read_u32(club.addr())));
        if offered.is_null() { (0, 0) } else { interest_slots(this, person, club, o, offered) }
    };
    set_news_slot(n, 0x1a, interest_flags);
    set_news_slot(n, 0x1b, interest);
    set_news_slot(n, 0x1c, if refused.is_null() { 0 } else { refused.count() as i32 as u32 });
    for (i, &c) in kept.iter().take(k).enumerate() {
        set_news_slot(n, 0x1d + i as i8, read_u32(c));
    }
    for &r in recipients.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, r);
        if answer == 2 {
            if let Some(i) = kept[..k].iter().position(|&c| c == r) {
                kept[i] = 0;
            }
        }
    }
    for &c in kept.iter().take(k) {
        if c != 0 {
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
        }
    }
}

/// Comparer for `post_contract_response_news`: only the original runs (the news goes to the managers' inboxes).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_post_contract_response_news(this: usize, person: usize, club: usize, other: usize, answer: u32, o: usize,
                                                                           contract: usize, refused: usize, value: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, usize, usize, u32)>(IDX_POST_CONTRACT_RESPONSE_NEWS)(this, person, club, other, answer, o, contract, refused, value) // simplification: original only
}

/// Whether the past transfer record `r` is due within the months asked: (past the end: stop, inside the range).
/// The year of its date (unsigned) against the end year, then the month (`exe::date_get_month`, signed) when equal;
/// then the same against the start.
unsafe fn past_transfer_in_range(r: usize, start: (u32, u32), end: (u32, u32)) -> (bool, bool) {
    let year = read_u16(r + past_transfer::DATE + cm_date::YEAR);
    if year > end.1 as u16 || year == end.1 as u16 && exe::date_get_month(r + past_transfer::DATE) as i8 > end.0 as i8 {
        return (true, false);
    }
    let inside = year > start.1 as u16 || year == start.1 as u16 && exe::date_get_month(r + past_transfer::DATE) as i8 >= start.0 as i8;
    (false, inside)
}

/// Appends record `r` to the caller's array `*list` (`*capacity` entries; it grows by 100 through the game's
/// `exe::crt_realloc`/`exe::crt_malloc`, the "Memory Allocation Error" box at `row` when it fails) at index `n`.
unsafe fn append_past_transfer(list: *mut usize, capacity: *mut i32, n: &mut i32, r: usize, row: u32) {
    if *n == *capacity {
        *capacity += 100;
        let bytes = (*capacity as usize * PAST_TRANSFER_SIZE) as u32;
        *list = if *list != 0 { exe::crt_realloc(*list, bytes) } else { exe::crt_malloc(bytes) };
        if *list == 0 {
            show_out_of_memory_box(row, 0xa8ecb8); // "TRANSFER_MANAGER::get_past_transfers() : cannot malloc transfers"
        }
    }
    let to = *list + *n as usize * PAST_TRANSFER_SIZE;
    *n += 1;
    core::ptr::copy_nonoverlapping(r as *const u8, to as *mut u8, past_transfer::DATE);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(to + past_transfer::DATE, tmp.as_mut_ptr() as usize, r + past_transfer::DATE);
}

/// Collects the past transfers from the first day of `start_month`/`start_year` to `end_month`/`end_year` into the
/// caller's growable array `*list`, whose `*count` is the capacity on entry and the number collected on exit.
///
/// - The caller's array already covering the range (start after its first record's date and the 28th of the end month
/// before its last one, the dates through `exe::date_date_549f70`): nothing, `*count` unchanged. With no array,
/// `*count` = 0.
/// - The records in memory (`past_transfers()`, `past_transfer_count()`, 37 bytes each, in date order): up to the first
/// one past the end month; those from the start month on are copied (`append_past_transfer`, box line 0x291f).
/// - Then each year from the start year to the end year: the file `transfers_<year>.tmp` in CM3_TEMP
/// (`exe::utils_build_file_path`, the name written into 0xde1f64), read record by record (`exe::crt_fread` of 37
/// bytes) up to the first one past the end month, copying those from the start month on (box line 0x295f); closed.
/// Original: `TRANSFER_MANAGER::get_past_transfers` (0x8cc030, transfer_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn get_past_transfers(this: usize, start_month: u32, start_year: u32, end_month: u32, end_year: u32,
                                                          list: *mut usize, count: *mut i32) {
    let this = TransferManagerFields(this);
    let (start, end) = ((start_month, start_year), (end_month, end_year));
    let mut n = 0i32;
    if *list == 0 {
        *count = 0;
    }
    if *list != 0 && *count > 0 {
        // the dates compared by year (unsigned) and day of the year (signed)
        let after = |d: usize, r: usize| {
            let (y, ry) = (read_u16(d + cm_date::YEAR), read_u16(r + cm_date::YEAR));
            y > ry || y == ry && read_i16(d) > read_i16(r)
        };
        let mut date = [0u16; 4];
        let first = *list + past_transfer::DATE;
        if after(exe::date_date_549f70(&mut date, 1, start_month, start_year, u32::MAX), first) {
            let last = *list + (*count as usize - 1) * PAST_TRANSFER_SIZE + past_transfer::DATE;
            if after(last, exe::date_date_549f70(&mut date, 0x1c, end_month, end_year, u32::MAX)) {
                covered!("sx02_4 already covered");
                return;
            }
        }
    }
    for i in 0..this.past_transfer_count() as i32 {
        let r = this.past_transfers() + i as usize * PAST_TRANSFER_SIZE;
        let (past_end, inside) = past_transfer_in_range(r, start, end);
        if past_end {
            break;
        }
        if inside {
            append_past_transfer(list, count, &mut n, r, 0x291f); // sx02_4: a record in memory
        }
    }
    if start_year as u16 <= end_year as u16 {
        let mut record = [0u8; PAST_TRANSFER_SIZE];
        let r = record.as_mut_ptr() as usize;
        exe::date_date_54a110(r + past_transfer::DATE);
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        for year in start_year as u16..=end_year as u16 {
            format(0xde1f64, 0xa8ea38, year as u32); // "transfers_%d.tmp"
            let file = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xde1f64), 0x98f928); // "CM3_TEMP", "rb"
            if file == 0 {
                continue;
            }
            covered!("sx02_4 yearly file");
            while exe::crt_fread(r, PAST_TRANSFER_SIZE as u32, 1, file) == 1 {
                let (past_end, inside) = past_transfer_in_range(r, start, end);
                if past_end {
                    break;
                }
                if inside {
                    append_past_transfer(list, count, &mut n, r, 0x295f); // sx02_4: a record from the file
                }
            }
            exe::crt_fclose(file);
        }
    }
    *count = n;
}

/// Comparer for `get_past_transfers`: the copy runs first on a copy of the caller's array (allocated with the game's
/// `exe::crt_malloc`, freed afterwards), then the original on the real one; checks the count and the records.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_get_past_transfers(this: usize, start_month: u32, start_year: u32, end_month: u32, end_year: u32,
                                                                  list: *mut usize, count: *mut i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32)>(IDX_GET_PAST_TRANSFERS);
    let (mut copy, mut copy_count) = (0usize, *count);
    if *list != 0 {
        let bytes = (*count).max(0) as usize * PAST_TRANSFER_SIZE;
        copy = exe::crt_malloc(bytes.max(1) as u32);
        core::ptr::copy_nonoverlapping(*list as *const u8, copy as *mut u8, bytes);
    }
    crate::rng::pause_trace(|| get_past_transfers(this, start_month, start_year, end_month, end_year, &mut copy, &mut copy_count));
    o(this, start_month, start_year, end_month, end_year, list, count);
    let n = (*count).max(0) as usize * PAST_TRANSFER_SIZE;
    let same = copy_count == *count && (n == 0 || core::slice::from_raw_parts(*list as *const u8, n) == core::slice::from_raw_parts(copy as *const u8, n));
    SCOREBOARD.record(same, || format!("get_past_transfers({}/{} .. {}/{}): original {} rust {}", start_month, start_year, end_month, end_year, *count, copy_count));
    if copy != 0 {
        exe::crt_free(copy);
    }
}

// --- sx07_1: the news item that a person answered a contract offer (contract_manager.cpp) ---

/// The comparer's dry run of `contract_manager_post_offer_response_news`: the copy builds the news item and keeps, for
/// every club it would send it to, the club and the item's bytes, instead of sending it.
pub(crate) static OFFER_RESPONSE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// The sends the dry run kept: (club, the news item's first `news_item::ID` bytes).
pub(crate) static OFFER_RESPONSE_NEWS_SENT: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

/// Sends the offer response news item to a club's manager (`exe::news_send_to_club_manager`), or keeps it in the dry
/// run.
unsafe fn offer_response_send(n: usize, club: Club) {
    if OFFER_RESPONSE_NEWS_DRY_RUN.load(Relaxed) {
        let bytes = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).push((club.addr(), bytes));
        return;
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Slot 0x11 of the offer response: with `refused` set, 0x3e7 when the current contract with the club ends before
/// `G_CONTRACT_DATE_PLUS_91_DAYS`, 0x3e8 when it ends before `G_CONTRACT_DATE_PLUS_365_DAYS`, otherwise 0 and `refused`
/// is cleared; without it, 0. The contract is read without a null check, as the original does.
unsafe fn offer_response_contract_end_code(contract: Contract, refused: &mut u32) -> u32 {
    if *refused == 0 {
        return 0;
    }
    let end = contract.addr() + contract::CONTRACT_EXPIRES;
    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_91_DAYS, end) {
        return 0x3e7;
    }
    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_365_DAYS, end) {
        return 0x3e8;
    }
    *refused = 0;
    0
}

/// Slot 0x1a of the offer response: an agreed player contract (type 2) of a player near the end of his career, by the
/// age he will have when the offer's contract ends (`exe::database_get_staff_age` on the offer's end date): a top
/// goalkeeper (`.goalkeeper()` = 20) over 37, or a defender, midfielder or attacker rated 15 or more over 36, 35 or 34.
unsafe fn offer_response_veteran(p: Staff, offer: Contract) -> u32 {
    let sheet = p.player();
    if sheet.is_null() || offer.contract_type() & 0x3f != 2 {
        return 0;
    }
    let end = (offer.addr() + contract::CONTRACT_EXPIRES) as *const u16;
    let age = || exe::database_get_staff_age(p, end);
    let veteran = (sheet.goalkeeper() == 0x14 && age() > 0x25)
        || (sheet.defender() >= 0xf && age() > 0x24)
        || (sheet.midfielder() >= 0xf && age() > 0x23)
        || (sheet.attacker() >= 0xf && age() > 0x22);
    veteran as u32
}

/// Posts news item 3000 (0xbb8) that a person accepted or rejected a contract offer, with the offer's terms, and picks
/// the clubs that receive it.
///
/// `post_offer_response_news(offer, refused)` (the contract manager in ecx; SEH frame for the local list and news
/// item). The offer is a `ContractOffer` (the `Contract` terms and the offer fields); the person is `staff_at` of its
/// id, the club `club_at` of `contract_offer::CLUB_ID`, and his contract with the club
/// (`exe::contract_manager_get_contract_for_club`, possibly none). The slots: 0 the person's id, 1-3 his name ids, 4 the
/// club's id, 5 `.state()` (signed), 6-7 the contract end (`contract::CONTRACT_EXPIRES`, signed day and year), 8 the
/// linked transfer offer slot (`.transfer_offer_slot()`), 10 the low 6 bits of `.contract_type()` (signed), 11 1 when
/// the offer's `.mark()` is not a player job (`exe::database_is_player_job`). With a linked transfer offer (slot ≠ −1):
/// its record (`exe::transfer_manager_get_offer_slot`); an empty slot gives the "Error" box (line 0x20c3) and nothing
/// is sent; otherwise slot 9 is its `offer::SERIAL`. Slot 12 the reasons the shortlist interest returns
/// (`exe::shortlist_manager_staff_interest_in_joining_club` with wage 0, the 6-bit contract type and the squad status
/// `.squad_status()` >> 4; the value itself is dropped). Slots 13-16 the club manager's id and name ids (−1 without
/// one). Slot 17 by `offer_response_contract_end_code` (only with `refused`, which it may clear). Slot 18 0x420e9,
/// slot 19 random(3), slot 20 1 when there is a contract with the club and the offer is agreed (state 2), 21 the
/// mark (signed), 22 the wage (`contract::WAGE`), 23 `contract::UNKNOWN`, 24-25 the start date
/// (`contract::DATE_STARTED`), 26 `offer_response_veteran`, 27 bit 7 of the person's `staff_state::STATUS_FLAGS`.
/// Who gets it: when `refused` is still set, or the agreed renewal of a contract with fewer than 180 days left
/// (`exe::staff_contract_days_to_end_date`), the clubs the transfer manager picks for the player's news
/// (`exe::transfer_manager_collect_player_news_recipients`, 0x21a, up to 30; none without a player sheet);
/// otherwise the club, and for an agreed player contract (type 2) also the clubs interested in him
/// (`exe::shortlist_manager_collect_interested_clubs` with 1, 1, 0), each but the club itself.
/// Original: `contract_manager_post_offer_response_news` (0x4d49a0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn contract_manager_post_offer_response_news(this: usize, offer: usize, refused: u32) {
    let mut refused = refused;
    let mut list = [0u8; 0xe];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let terms = Contract(offer);
    let o = ContractOffer(offer);
    let p = staff_at(read_i32(offer) as isize);
    let club = club_at(o.club_id() as i32 as isize);
    let contract = Contract(exe::contract_manager_get_contract_for_club(this, p, club));
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbb8, 0);
    let set = |k: i8, v: u32| {
        exe::newsitem_set_slot(n, k, v);
    };
    set(0, read_u32(p.addr()));
    set(1, read_u32(p.first_name() + names::ID));
    set(2, read_u32(p.second_name() + names::ID));
    set(3, read_u32(p.common_name() + names::ID));
    set(4, o.club_id() as u32);
    set(5, o.state() as i8 as i32 as u32);
    set(6, terms.contract_expires() as i32 as u32);
    set(7, terms.contract_expires_year() as u16 as u32);
    set(8, o.transfer_offer_slot() as u32);
    let kind = ((terms.contract_type() << 2) as i8 >> 2) as i32; // sx07_1: the 6-bit contract type
    set(0xa, kind as u32);
    set(0xb, (exe::database_is_player_job(o.mark() as i8) == 0) as u32);
    if o.transfer_offer_slot() as i32 != -1 {
        let slot = read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), o.transfer_offer_slot() as i32));
        if slot == 0 {
            show_error_at(0x9a8190, 0x20c3);
            exe::drop_llist(l);
            return;
        }
        set(9, read_u32(slot + offer::SERIAL));
    }
    let mut reasons = 0u32;
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    exe::shortlist_manager_staff_interest_in_joining_club(shortlists, p, club, 0, kind as u8 as u32, (terms.squad_status() >> 4) as u32, (&mut reasons) as *mut u32 as u32);
    set(0xc, reasons);
    let manager = club.manager();
    if !manager.is_null() {
        set(0xd, read_u32(manager.addr()));
        set(0xe, read_u32(manager.first_name() + names::ID));
        set(0xf, read_u32(manager.second_name() + names::ID));
        set(0x10, read_u32(manager.common_name() + names::ID));
    } else {
        for k in 0xd..=0x10 {
            set(k, u32::MAX);
        }
    }
    let code = offer_response_contract_end_code(contract, &mut refused); // sx07_1: the contract end code
    set(0x11, code);
    set(0x12, 0x420e9);
    set(0x13, exe::utils_random(3) as u32); // sx07_1: the headline's draw
    let agreed = o.state() == 2;
    set(0x14, (!club.is_null() && !contract.is_null() && agreed) as u32);
    set(0x15, o.mark() as i8 as i32 as u32);
    set(0x16, terms.wage() as u32);
    set(0x17, terms.unknown());
    set(0x18, read_i16(offer + contract::DATE_STARTED) as i32 as u32);
    set(0x19, terms.date_started_year() as u16 as u32);
    set(0x1a, offer_response_veteran(p, terms));
    set(0x1b, (read_u8(contract_manager_staff_state_record(read_usize(p.addr())) + staff_state::STATUS_FLAGS) >> 7) as u32);
    if refused != 0 || (!club.is_null() && !contract.is_null() && exe::staff_contract_days_to_end_date(contract) < 0xb4 && agreed) {
        covered!("sx07_1 offer response to the transfer recipients");
        let tm = read_usize(G_TRANSFER_MANAGER);
        let mut clubs = [0usize; 0x1e];
        let k = if tm != 0 && !p.player().is_null() {
            exe::transfer_manager_collect_player_news_recipients(tm, p, 0x21a, clubs.as_mut_ptr(), 0x1e) as i8
        } else {
            0
        };
        for &c in clubs.iter().take(k.max(0) as usize) {
            offer_response_send(n, Club(c));
        }
    } else {
        covered!("sx07_1 offer response to the club");
        offer_response_send(n, club_at(o.club_id() as i32 as isize)); // sx07_1: the club gets it
        if agreed && terms.contract_type() & 0x3f == 2 {
            exe::shortlist_manager_collect_interested_clubs(shortlists, p, l, 1, 1, 0);
            l.set_cursor(l.head());
            let mut x = exe::llist_get_next(l);
            while x != 0 {
                let id = read_usize(x + 4);
                if id != o.club_id() {
                    offer_response_send(n, club_at(id as i32 as isize));
                }
                x = exe::llist_get_next(l);
            }
        }
    }
    exe::drop_llist(l);
}

/// The news items that reached the human managers' inboxes since `before` (`human_inbox_positions`): the first
/// `news_item::ID` bytes of each, without `news_item::IMPORTANT`, which the delivery sets.
unsafe fn offer_response_delivered(before: &[(usize, i32, i32)]) -> Vec<Vec<u8>> {
    let news = read_usize(G_NEWS);
    let mut out = Vec::new();
    for &(e, start, next) in before {
        let (start_now, next_now) = (read_i32(e), read_i32(e + 4));
        if start_now == -1 || (start_now == start && next_now == next) {
            continue;
        }
        let mut k = if start_now == start { next } else { start_now };
        while k != next_now {
            let r = read_usize(news).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            let mut bytes: Vec<u8> = (0..news_item::ID).map(|j| read_u8(r + j)).collect();
            bytes[news_item::IMPORTANT] = 0;
            out.push(bytes);
            k += 1;
            if k == start_now + 0x64 {
                k = start_now;
            }
        }
    }
    out.sort();
    out
}

/// The 16 human inboxes (entry address, ring start, next slot), or none without the news object.
unsafe fn human_inbox_positions() -> Vec<(usize, i32, i32)> {
    let news = read_usize(G_NEWS);
    if news == 0 || read_usize(news + news_fields::HUMAN_INBOXES) == 0 {
        return Vec::new();
    }
    (0..0x10).map(|i| read_usize(news + news_fields::HUMAN_INBOXES) + i * 12).map(|e| (e, read_i32(e), read_i32(e + 4))).collect()
}

/// Comparer for `contract_manager_post_offer_response_news`, as a dry run: the copy runs first without sending (it
/// keeps each send), then the original from the same random generator state. Compares the generator's state and the
/// news items that reached the human managers' inboxes (the only thing the delivery keeps:
/// `send_to_club_manager` only delivers to a human manager) with the copy's sends to clubs with a human manager. The
/// empty linked transfer offer (the "Error" box) runs the original only.
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_post_offer_response_news(this: usize, offer: usize, refused: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CONTRACT_MANAGER_POST_OFFER_RESPONSE_NEWS);
    let slot = ContractOffer(offer).transfer_offer_slot() as i32;
    if slot != -1 && read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot)) == 0 {
        return o(this, offer, refused);
    }
    let inboxes = human_inbox_positions();
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).clear();
    OFFER_RESPONSE_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| contract_manager_post_offer_response_news(this, offer, refused)));
    OFFER_RESPONSE_NEWS_DRY_RUN.store(false, Relaxed);
    let rng_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let sent = core::mem::take(&mut *OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()));
    rng.write_back();
    o(this, offer, refused);
    let rng_original = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let staff_count = read_i32(G_STAFF_COUNT);
    let mut expected: Vec<Vec<u8>> = sent
        .iter()
        .filter(|(c, _)| {
            let m = Club(*c).manager();
            !m.is_null() && read_i32(m.addr()) >= staff_count.wrapping_sub(0x10)
        })
        .map(|(_, b)| {
            let mut b = b.clone();
            b[news_item::IMPORTANT] = 0;
            b
        })
        .collect();
    expected.sort();
    let delivered = offer_response_delivered(&inboxes);
    let same = rng_original == rng_rust && (inboxes.is_empty() || delivered == expected);
    SCOREBOARD.record(same, || {
        format!(
            "contract_manager_post_offer_response_news(offer {:#x}, refused {}) rng same {}; {} sends, human items original {:x?} rust {:x?}",
            offer,
            refused,
            rng_original == rng_rust,
            sent.len(),
            delivered,
            expected
        )
    });
}

// --- sx07_2: the text of the contract offer answer news (contract_manager.cpp, 5,424 bytes), in named pieces ---

/// What `write_offer_response_news_text` reads from news item 0xbb8 and prepares before writing (the original's frame).
pub(crate) struct OfferText {
    person: u32,
    names: [u32; 3],
    club: Club,
    state: u8,
    trial: bool,
    reasons: u32,
    manager: i32,
    manager_names: [u32; 3],
    end_code: u32,
    custom: bool,
    draw: i32,
    renewal: bool,
    job: u32,
    wage: i32,
    veteran: bool,
    rebel: bool,
    age: i8,
    home: bool,
    years: i32,
    value: i32,
    name: [u8; 0x190],
    surname: [u8; 0x190],
    manager_name: [u8; 0x190],
    end_date: [u8; 0x12c],
    money: [u8; 0x64],
    reason: [[u8; 0xfa]; 5],
    joined: [u8; 0x400],
    count: i8,
}

impl OfferText {
    fn ptr(b: &[u8]) -> usize {
        b.as_ptr() as usize
    }
    /// The club's short name and its gender (pushed twice).
    unsafe fn short(&self) -> (i32, usize) {
        (self.club.short_gender_name() as i8 as i32, self.club.addr() + club::SHORT_NAME)
    }
    /// The club's full name and its gender (pushed twice).
    unsafe fn long(&self) -> (i32, usize) {
        (self.club.gender_name() as i32, self.club.addr() + club::NAME)
    }
}

/// Reads the slots of news item 0xbb8 and prepares the names, the contract end date, the contract value and its years.
///
/// The slots: 0 the person, 1-3 his name ids, 4 the club, 5 the offer state, 6-7 the contract end, 10 the contract type
/// (5 a trial), 12 the reasons, 13 the club manager and 14-16 his name ids, 17 the contract end code, 18 0x420e9 for
/// the plain offer, 19 the draw, 20 the renewal, 21 the job, 22 the wage, 23 the signing value, 24-25 the contract
/// start, 26 the veteran, 27 the contract rebel. The days from the start to the end (365 per year, the days of the year
/// apart) give the value (whole weeks × the wage + slot 23; from 3,000,000 down to the million, above 1,000,000 down to
/// the half million, as money with `exe::cash_format_money`) and the years (days ÷ 365, rounded). "At home": the club
/// is the human manager's on screen. The names: the person's in modes 1 and 5, the manager's in mode 1 when slot 17 is
/// not negative (with slot 17 as the id, as the original passes it), the end date as a long date.
unsafe fn offer_text_read(news: usize) -> OfferText {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let person = slot(0);
    let names = [slot(1), slot(2), slot(3)];
    let club = club_at(slot(4) as i32 as isize);
    let state = slot(5) as u8;
    let age = read_i8(staff_at(person as i32 as isize).addr() + staff::AGE);
    let mut end = [0u16; 4];
    let (end_year, end_day) = (slot(7), slot(6));
    exe::new_date(end.as_mut_ptr() as usize, end_day, end_year);
    let trial = slot(0xa) as u8 == 5;
    let reasons = slot(0xc);
    let manager = slot(0xd) as i32;
    let manager_names = [slot(0xe), slot(0xf), slot(0x10)];
    let end_code = slot(0x11);
    let custom = slot(0x12) != 0x420e9;
    let draw = slot(0x13) as i32;
    let renewal = slot(0x14) != 0;
    let job = slot(0x15);
    let wage = slot(0x16) as i32;
    let signing = slot(0x17) as i32;
    let mut start = [0u16; 4];
    let (start_year, start_day) = (slot(0x19), slot(0x18));
    exe::new_date(start.as_mut_ptr() as usize, start_day, start_year);
    let days = ((end[1] as i32).wrapping_sub(start[1] as i32)).wrapping_mul(365).wrapping_sub(start[0] as i16 as i32).wrapping_add(end[0] as i16 as i32);
    let mut value = (days / 7).wrapping_mul(wage).wrapping_add(signing); // sx07_2: the contract value
    if value >= 3_000_000 {
        value = value / 1_000_000 * 1_000_000;
    } else if value > 1_000_000 {
        value = value / 500_000 * 500_000;
    }
    let mut t = OfferText {
        person,
        names,
        club,
        state,
        trial,
        reasons,
        manager,
        manager_names,
        end_code,
        custom,
        draw,
        renewal,
        job,
        wage,
        veteran: false,
        rebel: false,
        age,
        home: false,
        years: 0,
        value,
        name: [0; 0x190],
        surname: [0; 0x190],
        manager_name: [0; 0x190],
        end_date: [0; 0x12c],
        money: [0; 0x64],
        reason: [[0; 0xfa]; 5],
        joined: [0; 0x400],
        count: 0,
    };
    let amount = exe::cash_get_display_amount((&value) as *const i32 as *const u32, 6);
    exe::cash_format_money(t.money.as_mut_ptr() as usize, 0x64, amount, 6);
    t.years = ftol(days as f64 * read_f64(rdata::F64_0_002739726027) + read_f64(rdata::F64_0_5));
    t.veteran = slot(0x1a) != 0;
    t.rebel = slot(0x1b) != 0;
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    t.home = hm != 0 && read_usize(hm + 0x39) != 0 && read_u32(read_usize(hm + 0x39)) == slot(4); // sx07_2: at home
    let [n1, n2, n3] = t.names;
    exe::database_format_name_from_ids(n1, n2, n3, t.name.as_mut_ptr(), 0x190, 1, person);
    exe::database_format_name_from_ids(n1, n2, n3, t.surname.as_mut_ptr(), 0x190, 5, person);
    if end_code as i32 > -1 {
        let [m1, m2, m3] = t.manager_names;
        exe::database_format_name_from_ids(m1, m2, m3, t.manager_name.as_mut_ptr(), 0x190, 1, end_code);
    }
    exe::date_get_long_date_string(end.as_mut_ptr() as usize, t.end_date.as_mut_ptr() as usize, 0x12c, 0xff);
    t
}

/// Joins the reasons gathered so far into `joined`: one is copied (`"%s"`), two go into 0x9a9f28 ("... and ...").
unsafe fn offer_text_join_reasons(t: &mut OfferText) {
    let fmt = news_text_format();
    match t.count {
        1 => {
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(OfferText::ptr(&t.joined), 0x991944, OfferText::ptr(&t.reason[0]));
        }
        2 => {
            fmt(OfferText::ptr(&t.joined), 0x9a9f28, OfferText::ptr(&t.reason[0]), OfferText::ptr(&t.reason[1]));
        }
        _ => {}
    }
}

/// The headline. Agreed (state 2): a trial 0x9aa414; a custom offer 0x9aa2a0; the renewal of a job 11 contract by the
/// draw (0 0x9aa320, 1 0x9aa368 with the years, otherwise 0x9aa3cc); away 0x9aa2d8 with the club; at home 0x9aa2a0.
/// Rejected (state 1 or 3): a trial 0x9aa26c; a custom offer 0x9aa234; the renewal of a job 11 contract: a contract
/// rebel 0x9aa1ec, otherwise by the draw (0 0x9aa104, 1 0x9aa154 with the years, otherwise 0x9aa1bc); away 0x9aa0bc with
/// the club; at home 0x9aa234. Another state: the "Error" box (line 0x223b) and nothing written (false).
unsafe fn offer_text_headline(t: &OfferText) -> bool {
    let fmt = news_text_format();
    let s = OfferText::ptr(&t.surname);
    let (g, short) = t.short();
    match t.state {
        2 => {
            if t.trial {
                fmt(NEWS_TEXT_BUFFER, 0x9aa414, s);
            } else if t.custom {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2a0, s);
            } else if t.job == 0xb && t.renewal {
                match t.draw {
                    0 => fmt(NEWS_TEXT_BUFFER, 0x9aa320, g, g, s, short),
                    1 => fmt(NEWS_TEXT_BUFFER, 0x9aa368, g, g, s, t.years, short),
                    _ => fmt(NEWS_TEXT_BUFFER, 0x9aa3cc, g, g, short, s),
                };
            } else if !t.home {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2d8, g, g, s, short); // sx07_2: accepts the club's offer
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2a0, s);
            }
        }
        1 | 3 => {
            if t.trial {
                fmt(NEWS_TEXT_BUFFER, 0x9aa26c, s);
            } else if t.custom {
                fmt(NEWS_TEXT_BUFFER, 0x9aa234, s);
            } else if t.job == 0xb && t.renewal {
                if t.rebel {
                    fmt(NEWS_TEXT_BUFFER, 0x9aa1ec, s);
                } else {
                    match t.draw {
                        0 => fmt(NEWS_TEXT_BUFFER, 0x9aa104, g, g, s, short),
                        1 => fmt(NEWS_TEXT_BUFFER, 0x9aa154, g, g, s, t.years, short),
                        _ => fmt(NEWS_TEXT_BUFFER, 0x9aa1bc, s),
                    };
                }
            } else if !t.home {
                fmt(NEWS_TEXT_BUFFER, 0x9aa0bc, g, g, s, short);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9aa234, s);
            }
        }
        _ => {
            show_error_at(0x9a8190, 0x223b);
            return false;
        }
    }
    true
}

/// The reasons of an agreed offer: the love for the club (reason bit 8; with bit 0x40 and a manager, also the
/// admiration for him), or, without bit 8, a player under 24 with bit 0x40 and a manager: the belief in him; joined.
unsafe fn offer_text_accept_reasons(t: &mut OfferText) {
    let fmt = news_text_format();
    let mn = OfferText::ptr(&t.manager_name);
    if t.reasons & 8 != 0 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a9ff0);
        t.count = 1;
        if t.reasons & 0x40 != 0 && t.manager > -1 {
            fmt(OfferText::ptr(&t.reason[1]), 0x9a9fa8, mn);
            t.count = 2;
        }
    } else if t.reasons & 0x40 != 0 && t.age < 0x18 && t.manager > -1 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a9f4c, mn);
        t.count = 1;
    }
    offer_text_join_reasons(t);
}

/// The body of an agreed offer with reasons. With the admiration bit (0x40) and the veteran: with a manager, above
/// (draw + 1) million 0x9a9e2c (years, money, reasons), otherwise 0x9a9d3c (end date, reasons); without a manager, above
/// that 0x9a9c48 for a contract rebel or 0x9a9b6c (years, money), otherwise 0x9a9aac (end date). Then (not a veteran,
/// or without bit 0x40 and not a veteran) with the love bit (8) and one reason, by the draw: 0 the renewal 0x9a9044 or
/// 0x9a8f8c (end date); 1 above 1,000,000 the renewal 0x9a948c (rebel) or 0x9a936c, else 0x9a9274, and up to 1,000,000
/// the renewal 0x9a9190 else 0x9a90fc (years); otherwise the renewal by the years (one: 0x9a998c for a rebel, 0x9a97bc;
/// more: 0x9a9858 for a rebel, 0x9a9710) or, a new contract, 0x9a9664 when the person's id ends in 0 and the wage is
/// 8,000 or more, else 0x9a95d4. Otherwise a rebel with a wage of 5,000 or more 0x9a8edc, else 0x9a8e50 (reasons).
unsafe fn offer_text_accept_with_reasons(t: &OfferText, admiration: bool) {
    let fmt = news_text_format();
    let (n, s, mo, j, d) = (OfferText::ptr(&t.name), OfferText::ptr(&t.surname), OfferText::ptr(&t.money), OfferText::ptr(&t.joined), OfferText::ptr(&t.end_date));
    let (g, short) = t.short();
    let (gl, long) = t.long();
    let threshold = t.draw.wrapping_mul(5).wrapping_add(5).wrapping_mul(200_000);
    if t.veteran {
        if admiration && t.manager > -1 {
            if t.value > threshold {
                fmt(NEWS_TEXT_BUFFER, 0x9a9e2c, g, g, n, short, t.years, mo, j);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a9d3c, g, g, 100, n, short, d, j);
            }
        } else if t.value > threshold {
            if t.rebel {
                fmt(NEWS_TEXT_BUFFER, 0x9a9c48, g, g, n, short, t.years, mo);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a9b6c, g, g, n, short, t.years, mo);
            }
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9a9aac, g, g, 100, n, short, d);
        }
        return;
    }
    if t.reasons & 8 != 0 && t.count == 1 {
        covered!("sx07_2 love for the club");
        match t.draw {
            0 => {
                let tpl = if t.renewal { 0x9a9044 } else { 0x9a8f8c };
                fmt(NEWS_TEXT_BUFFER, tpl, gl, gl, 100, n, long, d);
            }
            1 => {
                if t.value > 1_000_000 {
                    if t.renewal {
                        let tpl = if t.rebel { 0x9a948c } else { 0x9a936c };
                        fmt(NEWS_TEXT_BUFFER, tpl, gl, gl, long, n, t.years, mo, s);
                    } else {
                        fmt(NEWS_TEXT_BUFFER, 0x9a9274, gl, gl, n, t.years, long, mo);
                    }
                } else if t.renewal {
                    fmt(NEWS_TEXT_BUFFER, 0x9a9190, gl, gl, long, n, t.years);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0x9a90fc, gl, gl, n, t.years, long);
                }
            }
            _ => {
                if t.renewal {
                    match (t.rebel, t.years == 1) {
                        (true, true) => fmt(NEWS_TEXT_BUFFER, 0x9a998c, gl, gl, 100, gl, gl, long, n, s, long),
                        (true, false) => fmt(NEWS_TEXT_BUFFER, 0x9a9858, gl, gl, 100, gl, gl, long, n, t.years, s, long),
                        (false, true) => fmt(NEWS_TEXT_BUFFER, 0x9a97bc, gl, gl, long, n),
                        (false, false) => fmt(NEWS_TEXT_BUFFER, 0x9a9710, gl, gl, long, n, t.years),
                    };
                } else if (t.person as i32) % 10 == 0 && t.wage >= 0x1f40 {
                    fmt(NEWS_TEXT_BUFFER, 0x9a9664, 100, gl, gl, n, t.years, long);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0x9a95d4, gl, gl, n, t.years, long);
                }
            }
        }
    } else if t.rebel && t.wage >= 0x1388 {
        fmt(NEWS_TEXT_BUFFER, 0x9a8edc, gl, gl, n, t.years, long, j);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9a8e50, gl, gl, n, long, j);
    }
}

/// The body of an agreed offer without reasons. A veteran's renewal 0x9a8d90 (end date). Otherwise a renewal or a new
/// contract above (draw + 1) million: when the draw is at least the person's id modulo 3 0x9a8cd0 or 0x9a8ab8
/// (years, money), else 0x9a8c1c or 0x9a8a04; up to that, the renewal 0x9a8b64 or the new contract 0x9a895c (end date).
/// A veteran's new contract takes the new contract's texts.
unsafe fn offer_text_accept_plain(t: &OfferText) {
    let fmt = news_text_format();
    let (n, mo, d) = (OfferText::ptr(&t.name), OfferText::ptr(&t.money), OfferText::ptr(&t.end_date));
    let (g, short) = t.short();
    if t.veteran && t.renewal {
        fmt(NEWS_TEXT_BUFFER, 0x9a8d90, g, g, 100, n, short, d);
        return;
    }
    let renewal = !t.veteran && t.renewal;
    let threshold = t.draw.wrapping_mul(5).wrapping_add(5).wrapping_mul(200_000);
    if t.value > threshold {
        if t.draw >= (t.person as i32) % 3 {
            let tpl = if renewal { 0x9a8cd0 } else { 0x9a8ab8 };
            fmt(NEWS_TEXT_BUFFER, tpl, g, g, n, short, t.years, mo);
        } else if renewal {
            fmt(NEWS_TEXT_BUFFER, 0x9a8c1c, g, g, short, n, t.years, mo);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9a8a04, g, g, n, t.years, short, mo);
        }
    } else {
        let tpl = if renewal { 0x9a8b64 } else { 0x9a895c }; // sx07_2: the plain agreement
        fmt(NEWS_TEXT_BUFFER, tpl, g, g, 100, n, short, d);
    }
}

/// The body of an agreed offer: a trial 0x9aa010 (end date); otherwise the reasons (`offer_text_accept_reasons`), then a
/// custom offer 0x9a8b64 (end date), with reasons `offer_text_accept_with_reasons`, without them
/// `offer_text_accept_plain`.
unsafe fn offer_text_accepted_body(t: &mut OfferText) {
    let fmt = news_text_format();
    let (g, short) = t.short();
    if t.trial {
        fmt(NEWS_TEXT_BUFFER, 0x9aa010, g, g, 100, OfferText::ptr(&t.name), short, OfferText::ptr(&t.end_date));
        return;
    }
    offer_text_accept_reasons(t);
    if t.custom {
        fmt(NEWS_TEXT_BUFFER, 0x9a8b64, g, g, 100, OfferText::ptr(&t.name), short, OfferText::ptr(&t.end_date));
    } else if t.count != 0 {
        offer_text_accept_with_reasons(t, t.reasons & 0x40 != 0);
    } else {
        offer_text_accept_plain(t);
    }
}

/// The body of a rejected offer (state 1): a trial 0x9a88e8 with the club's short name. Otherwise the
/// reasons: unhappy with the manager (reason bit 0x100, with a manager) and wanting to get away (bit 4), joined; the
/// person's name again (300 bytes). Away: with slot 17 = 0x3e7 0x9a87a0 / 0x9a8704, with 0x3e8 0x9a8650 / 0x9a85c0 (with
/// / without the reasons); otherwise (or at home) 0x9a852c / 0x9a84bc, all with the club's full name.
unsafe fn offer_text_rejected_body(t: &mut OfferText) {
    let fmt = news_text_format();
    if t.trial {
        let (g, short) = t.short();
        fmt(NEWS_TEXT_BUFFER, 0x9a88e8, g, g, OfferText::ptr(&t.name), short);
        return;
    }
    if t.reasons & 0x100 != 0 && t.manager > -1 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a8898, OfferText::ptr(&t.manager_name));
        t.count = 1;
    }
    if t.reasons & 4 != 0 {
        fmt(OfferText::ptr(&t.reason[t.count as usize]), 0x9a8860);
        t.count += 1;
    }
    offer_text_join_reasons(t);
    let [n1, n2, n3] = t.names;
    exe::database_format_name_from_ids(n1, n2, n3, t.name.as_mut_ptr(), 0x12c, 1, t.person);
    let (n, j) = (OfferText::ptr(&t.name), OfferText::ptr(&t.joined));
    let (gl, long) = t.long();
    let with = t.count != 0;
    match (t.home, t.end_code) {
        (false, 0x3e7) => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a87a0, gl, gl, n, long, j)
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a8704, gl, gl, n, long)
            }
        }
        (false, 0x3e8) => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a8650, 100, gl, gl, n, long, j)
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a85c0, 100, gl, gl, n, long)
            }
        }
        _ => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a852c, gl, gl, n, long, j) // sx07_2: rejected with reasons
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a84bc, gl, gl, n, long)
            }
        }
    };
}

/// The links: the club's short name (0x7d5); the person's two names (0x7d7) when the news item's names still match him
/// (`exe::database_staff_matches_names`); the manager's name (0x7d7) when there is one and his names still match.
unsafe fn offer_text_links(t: &OfferText, out: usize) {
    exe::text_add_link(out, (t.club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, t.club);
    let p = staff_at(t.person as i32 as isize);
    let [n1, n2, n3] = t.names;
    if exe::database_staff_matches_names(p, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, t.name.as_ptr(), 0x7d7, p);
        exe::text_add_link(out, t.surname.as_ptr(), 0x7d7, p);
    }
    if t.manager > -1 {
        let m = staff_at(t.manager as isize);
        let [m1, m2, m3] = t.manager_names;
        if exe::database_staff_matches_names(m, m1, m2, m3, 0usize) != 0 {
            exe::text_add_link(out, t.manager_name.as_ptr(), 0x7d7, m);
        }
    }
}

/// Writes the headline or the body of news item 0xbb8, that a person accepted or rejected a contract or trial offer.
///
/// `write_offer_response_news_text(news, full, out)` (the contract manager in ecx, not read): reads the item
/// (`offer_text_read`). The headline (`full` = 0) by `offer_text_headline`; another state than 1, 2 or 3 gives the
/// "Error" box and writes nothing. The body: agreed (state 2) `offer_text_accepted_body`, rejected (state 1)
/// `offer_text_rejected_body`, state 3 0x9a841c (willing to negotiate, with the club's short name); another state
/// leaves the shared text buffer as it was. Then the text (`exe::text_set_string`) and the links (`offer_text_links`).
/// Defect of the original: the manager's name is formatted with slot 17 (the contract end code) as the person id and
/// only when that is not negative; the body of another state shows whatever the shared buffer had.
/// Original: `contract_manager_write_offer_response_news_text` (0x4d4fb0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn contract_manager_write_offer_response_news_text(_this: usize, news: usize, full: u32, out: usize) {
    covered!("sx07_2 offer response text");
    let mut t = offer_text_read(news);
    if full == 0 {
        if !offer_text_headline(&t) {
            return;
        }
    } else {
        match t.state {
            2 => offer_text_accepted_body(&mut t),
            1 => offer_text_rejected_body(&mut t),
            3 => {
                let (g, short) = t.short();
                news_text_format()(NEWS_TEXT_BUFFER, 0x9a841c, g, g, OfferText::ptr(&t.name), short);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    offer_text_links(&t, out);
}

/// Comparer for `contract_manager_write_offer_response_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_write_offer_response_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_CONTRACT_MANAGER_WRITE_OFFER_RESPONSE_NEWS_TEXT);
    compare_text_writer(
        out,
        || o(this, news, full, out),
        |t| contract_manager_write_offer_response_news_text(this, news, full, t),
        || format!("contract_manager_write_offer_response_news_text(news {:#x}, full {})", news, full),
    );
}

/// The display currency's scale factor (f64): the rounding thresholds and steps are in pounds times this.
pub(crate) const CURRENCY_SCALE: usize = 0xad9cc0;
/// The display currency's exchange rate (f64): a stored amount times this is the amount shown.
pub(crate) const CURRENCY_RATE: usize = 0xad9cb8;
/// The thousands separator character of the display currency (byte).
pub(crate) const CURRENCY_THOUSANDS_SEPARATOR: usize = 0xad9ca2;
/// Nonzero when a four-digit amount is written without the thousands separator (dword).
pub(crate) const CURRENCY_NO_SEPARATOR_UNDER_10000: usize = 0xad9ca4;
/// The decimal separator character of the abbreviated amounts (byte).
pub(crate) const CURRENCY_DECIMAL_SEPARATOR: usize = 0xad9cc8;
/// The text after a full amount (the currency suffix).
pub(crate) const CURRENCY_SUFFIX: usize = 0xad9c70;
/// The unit texts of the abbreviated amounts: thousands, millions, billions (empty in some currencies) and the
/// thousands of billions.
pub(crate) const CURRENCY_UNIT_THOUSANDS: usize = 0xad9d00;
pub(crate) const CURRENCY_UNIT_MILLIONS: usize = 0xad9d68;
pub(crate) const CURRENCY_UNIT_BILLIONS: usize = 0xad9ccc;
pub(crate) const CURRENCY_UNIT_TRILLIONS: usize = 0xad9d34;
/// cash.cpp, for the assertion box.
pub(crate) const CASH_CPP: usize = 0x98b1fc;

/// The rounding ladders of `cash_get_rounding_step`: (threshold, step) in pounds, both times the currency scale; the
/// first threshold the amount does not exceed gives the step, and above the last one the step is 1,000,000.
const ROUNDING_FLAG_4: [(f64, f64); 8] = [
    (10000.0, 1000.0), (30000.0, 2000.0), (100000.0, 5000.0), (250000.0, 10000.0),
    (1000000.0, 25000.0), (5000000.0, 100000.0), (20000000.0, 250000.0), (50000000.0, 500000.0),
];
const ROUNDING_FLAG_8: [(f64, f64); 13] = [
    (10.0, 1.0), (100.0, 5.0), (250.0, 10.0), (1000.0, 25.0), (5000.0, 100.0), (20000.0, 250.0), (50000.0, 500.0),
    (100000.0, 5000.0), (250000.0, 10000.0), (1000000.0, 25000.0), (5000000.0, 100000.0), (20000000.0, 250000.0),
    (50000000.0, 500000.0),
];

/// The step a money amount is rounded to on screen. Flag 4 (wages per year, transfer fees) uses the coarse ladder: up
/// to 10,000 in steps of 1,000, 30,000 of 2,000, 100,000 of 5,000, 250,000 of 10,000, 1M of 25,000, 5M of 100,000, 20M
/// of 250,000, 50M of 500,000, above that of 1M. Flag 8 (wages per week or month) the fine one, from steps of 1 up to
/// 10 to steps of 1M above 50M (with a jump from 500 to 5,000 at 100,000). Thresholds and steps are times the currency
/// scale and the step is truncated (`_ftol`); flag 4 wins over 8, and without either the step is 1,000.
/// Original: `cash_get_rounding_step` (0x4426e0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_get_rounding_step(amount: i64, flags: u32) -> i64 {
    let ladder: &[(f64, f64)] = if flags & 4 != 0 {
        &ROUNDING_FLAG_4
    } else if flags & 8 != 0 {
        &ROUNDING_FLAG_8
    } else {
        covered!("sx05_1 rounding step default");
        return 1000;
    };
    let scale = read_f64(CURRENCY_SCALE);
    // ponytail: the original compares the exact 64-bit integer (fild); amounts beyond 2^53 would round here
    let x = amount as f64;
    for &(threshold, step) in ladder {
        if !(x > scale * threshold) {
            covered!("sx05_1 rounding step ladder");
            return (scale * step) as i64;
        }
    }
    (scale * 1000000.0) as i64
}

/// The amount a cash value shows in the display currency: the stored value times the exchange rate, truncated, then
/// (unless flag 0x20 asks for the exact value) rounded to the nearest step of `cash_get_rounding_step` with the same
/// flags (half a step up, then down to the step, in signed 64-bit division).
/// Original: `Cash::get_display_amount` (0x43f460, cash.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cash_get_display_amount(cash: usize, flags: u32) -> i64 {
    let amount = (read_i32(cash) as f64 * read_f64(CURRENCY_RATE)) as i64;
    if flags & 0x20 != 0 {
        return amount;
    }
    covered!("sx05_1 display amount rounded");
    let step = cash_get_rounding_step(amount, flags);
    (step / 2 + amount) / step * step
}

/// The decimal digits of `amount`, most significant first, at most the lowest 15; none for zero or less. As the
/// original builds them: digit by digit from the units until the digits written add up to the amount (so the top
/// zero digits never appear).
fn money_digits(amount: i64) -> ([u8; 16], usize) {
    let (mut digits, mut n, mut sum, mut power) = ([0u8; 16], 0usize, 0i64, 1i64);
    while sum < amount && n < 15 {
        let digit = amount % (power * 10) / power;
        digits.copy_within(0..n, 1);
        digits[0] = b'0' + digit as u8;
        sum += digit * power;
        power *= 10;
        n += 1;
    }
    (digits, n)
}

/// The digits with the thousands separator before each group of three from the right.
unsafe fn push_grouped(out: &mut Vec<u8>, digits: &[u8]) {
    for (k, &d) in digits.iter().enumerate() {
        if k > 0 && (digits.len() - k) % 3 == 0 {
            out.push(read_u8(CURRENCY_THOUSANDS_SEPARATOR));
        }
        out.push(d);
    }
}

/// The C string at `a`, without its terminator.
unsafe fn push_c_string(out: &mut Vec<u8>, a: usize) {
    out.extend_from_slice(core::slice::from_raw_parts(a as *const u8, c_string_len(a)));
}

/// Writes a 64-bit money amount as text into `buf`: the currency prefix, the amount and the suffix. With flag 2 the
/// whole amount with the thousands separator (none in a four-digit amount when the currency says so); without it,
/// amounts of four digits or more are abbreviated to thousands, millions, billions or thousands of billions with the
/// unit text: the leading one to three digits, then up to two decimals after the decimal separator with trailing zeros
/// dropped. A currency without a billions unit writes billions as whole millions with the separator. Zero and negative
/// amounts are written "0"; spaces in the result become '|' (the screens' non-breaking space). Returns 1, or 0 with
/// the assertion box (line 0x2f3) when the buffer is null or shorter than 25.
/// Defect of the original: a single-digit amount is written "0" (the jump table starts at two digits and one digit
/// falls into the default case), and the buffer size is only checked against 25.
/// Original: `cash_format_money` (0x441c10, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn cash_format_money(buf: usize, size: u32, amount: i64, flags: u32) -> u32 {
    if buf == 0 || (size as u16 as i16) < 0x19 {
        show_error_at(CASH_CPP, 0x2f3);
        return 0;
    }
    let (digits, n) = money_digits(amount);
    let d = &digits[..n];
    let mut out = Vec::with_capacity(0x60);
    push_c_string(&mut out, MONEY_VIEW_CURRENCY_SYMBOL);
    if n < 2 {
        covered!("sx05_1 money zero");
        out.push(b'0');
        push_c_string(&mut out, CURRENCY_SUFFIX);
    } else if n <= 3 || flags & 2 != 0 {
        covered!("sx05_1 money full");
        if n == 4 && read_u32(CURRENCY_NO_SEPARATOR_UNDER_10000) != 0 {
            out.extend_from_slice(d);
        } else {
            push_grouped(&mut out, d);
        }
        push_c_string(&mut out, CURRENCY_SUFFIX);
    } else if (10..=12).contains(&n) && read_u8(CURRENCY_UNIT_BILLIONS) == 0 {
        covered!("sx05_1 money whole millions");
        push_grouped(&mut out, &d[..n - 6]);
        push_c_string(&mut out, CURRENCY_UNIT_MILLIONS);
    } else {
        covered!("sx05_1 money abbreviated");
        let unit = [CURRENCY_UNIT_THOUSANDS, CURRENCY_UNIT_MILLIONS, CURRENCY_UNIT_BILLIONS, CURRENCY_UNIT_TRILLIONS][(n - 1) / 3 - 1];
        let whole = (n - 1) % 3 + 1;
        out.extend_from_slice(&d[..whole]);
        let decimals = if d[whole + 1] != b'0' { 2 } else if d[whole] != b'0' { 1 } else { 0 };
        if decimals > 0 {
            out.push(read_u8(CURRENCY_DECIMAL_SEPARATOR));
            out.extend_from_slice(&d[whole..whole + decimals]);
        }
        push_c_string(&mut out, unit);
    }
    out.push(0);
    core::ptr::copy_nonoverlapping(out.as_ptr(), buf as *mut u8, out.len());
    // the spaces up to the first terminator (a separator character 0 ends the text early)
    let mut k = 0;
    while read_u8(buf + k) != 0 {
        if read_u8(buf + k) == b' ' {
            write_u8(buf + k, b'|');
        }
        k += 1;
    }
    1
}

/// Comparer for `cash_get_rounding_step`: the step side by side.
pub(crate) unsafe extern "C" fn compare_cash_get_rounding_step(amount: i64, flags: u32) -> i64 {
    let r = original_fn::<unsafe extern "C" fn(i64, u32) -> i64>(IDX_CASH_GET_ROUNDING_STEP)(amount, flags);
    let v = cash_get_rounding_step(amount, flags);
    SCOREBOARD.record(r == v, || format!("cash_get_rounding_step({}, {:#x}) original={} rust={}", amount, flags, r, v));
    r
}

compare_value!(compare_cash_get_display_amount, cash_get_display_amount, IDX_CASH_GET_DISPLAY_AMOUNT, (flags: u32) -> i64);

/// Comparer for `cash_format_money`: the text side by side (the first 0x80 bytes of the buffer, at most its size); a
/// bad buffer (the assertion box) only runs the original.
pub(crate) unsafe extern "C" fn compare_cash_format_money(buf: usize, size: u32, amount: i64, flags: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, i64, u32) -> u32>(IDX_CASH_FORMAT_MONEY);
    if buf == 0 || (size as u16 as i16) < 0x19 {
        return o(buf, size, amount, flags);
    }
    let len = (size as u16 as usize).min(0x80);
    let (r, v, equal, a, b) = crate::comparison::side_by_side(&[(buf, len)], || o(buf, size, amount, flags), || cash_format_money(buf, size, amount, flags));
    SCOREBOARD.record(equal && r == v, || format!("cash_format_money({}, {:#x}) original={:?} rust={:?}", amount, flags, first_differences(&a, &b), (r, v)));
    r
}

/// The slots of an AWOL-family news item, read in order 0..`n` (`exe::newsitem_get_slot`, as the original does).
unsafe fn awol_news_slots<const N: usize>(news: usize) -> [u32; N] {
    let mut s = [0u32; N];
    for (k, v) in s.iter_mut().enumerate() {
        *v = exe::newsitem_get_slot(news, k as u32);
    }
    s
}

/// A person's name from the three name ids of a news item (`exe::database_format_name_from_ids`, 0x190 bytes) in the
/// given style.
unsafe fn awol_news_name(ids: [u32; 3], id: u32, style: u32) -> [u8; 0x190] {
    let mut name = [0u8; 0x190];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    name
}

/// Links a person's name when his names still match the news item's ids.
unsafe fn awol_link_person(out: usize, name: &[u8], ids: [u32; 3], id: u32) {
    let person = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, person);
    }
}

/// The club whose club id is the screen's human manager's club job, as the AWOL texts test it.
unsafe fn awol_is_human_club(club_id: u32) -> bool {
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize));
    !manager.is_null() && !manager.club_job().is_null() && read_u32(manager.club_job().addr()) == club_id
}

/// A club's short name and the gender byte of it (sign-extended, as the texts pass it).
unsafe fn awol_club(id: u32) -> (usize, u32) {
    let club = club_at(id as i32 as isize);
    (club.addr() + club::SHORT_NAME, read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32)
}

/// A nation's short name and the gender byte of it.
unsafe fn awol_nation(id: u32) -> (usize, u32) {
    let nation = read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE));
    (nation + nation::SHORT_NAME, read_i8(nation + nation::SHORT_GENDER_NAME) as i32 as u32)
}

/// Writes the headline or the body of the AWOL news (slots: 0 the player, 1..3 his name ids, 4 another club, 5 his
/// club, 6 the kind, 7 days away, 8 the nation he went to).
///
/// The headline (name in style 5) by kind: 0 "<player> returns to <club>", 1 "misses training" when the player's id is
/// a multiple of 3, else "goes AWOL", 2 by the id modulo 3 "in transfer row", "misses training" or "goes AWOL", 4
/// "fails to return from leave"; the club's link. The body (style 1) differs when the player's club is the screen's
/// human manager's: kind 0 he is back (and the squad is unhappy), 1 he refuses to play and wants a move, 2 he missed
/// training after being denied a move to the other club, 4 he failed to return from leave: to the nation, with
/// "missed his flight" under 7 days away, or the agent's message with a nation, or without one; the other managers
/// read the same with the club's name first. Then the player's link and the two clubs' links (long name).
/// Defect of the original: kind 3 (and above 4) formats nothing, so the text is whatever the shared buffer 0xde1f64
/// last held; the headline reads slot 6 twice and never slot 7.
/// Original: `write_awol_news_text` (0x419c60, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_awol_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = awol_news_slots(news);
        let kind = exe::newsitem_get_slot(news, 6);
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        let by_id = s[0] as i32 % 3;
        match kind {
            0 => {
                let (club, g) = awol_club(s[5]);
                // "<player> returns to <club>"
                fmt(buf, 0x98811c, g, g, np, club)
            }
            1 if by_id == 0 => fmt(buf, 0x9881d4, np), // "<player> misses training"
            2 if by_id == 0 => fmt(buf, 0x9881a0, np), // "<player> in transfer row"
            2 if by_id == 1 => fmt(buf, 0x9881d4, np),
            1 | 2 => fmt(buf, 0x988170, np), // "<player> goes AWOL"
            4 => fmt(buf, 0x988208, np), // "<player> fails to return from leave"
            _ => 0,
        };
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        if s[5] as i32 > -1 {
            exe::text_add_link(out, (club_at(s[5] as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(s[5] as i32 as isize));
        }
        return;
    }
    covered!("sx05_5 awol body");
    let s: [u32; 9] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (other, club, kind, days, nation) = (s[4], s[5], s[6], s[7] as i32, s[8]);
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let (cn, g) = awol_club(club);
    if awol_is_human_club(club) {
        covered!("sx05_5 awol at the human club");
        match kind {
            0 => fmt(buf, 0x988248, np),
            1 => fmt(buf, 0x988310, g, g, np, cn),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x9883e0, og, og, g, g, np, on, cn)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988698, ng, ng, g, g, np, nn, cn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x9885a4, ng, ng, np, nn)
            }
            4 => fmt(buf, 0x9884cc, np),
            _ => 0,
        };
    } else {
        match kind {
            0 => fmt(buf, 0x9887bc, g, g, cn, np),
            1 => fmt(buf, 0x9888b4, g, g, cn, np),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x988990, g, g, og, og, cn, np, on)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988ccc, g, g, ng, ng, cn, np, nn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988b9c, g, g, ng, ng, cn, np, nn)
            }
            4 => fmt(buf, 0x988a88, g, g, cn, np),
            _ => 0,
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    for c in [other, club] {
        if c as i32 > -1 {
            exe::text_add_link(out, (club_at(c as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(c as i32 as isize));
        }
    }
}

/// Writes the headline or the body of the missed training news (slots: 0 the player, 1..3 his name ids, 4 his club,
/// 5 the kind, 6 the absent flag, 7 the coach who reports (-1 none), 8..10 the coach's name ids).
///
/// The headline (slots 0..3, 5 and 6; name in style 5): "<player> absent from training" with the absent flag, else
/// "rumoured to have missed training" for kind 0 or "disciplined by club"; the player's link. The body (style 1): at
/// the screen's human manager's club the chief coach (or the named coach) reports the player did not turn up (kind
/// 0), was absent (1) or failed to report (other); elsewhere the club's board is losing patience (no coach), or the
/// manager is losing patience (kind 0) or fined him two weeks' (1) or one week's wages (other). Then the player's
/// link, the coach's link and the club's link (long name). Slot 6 is read in the body and not used.
/// Original: `write_missed_training_news_text` (0x41a8e0, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_missed_training_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = {
            let mut s = [0u32; 7];
            for k in [0u32, 1, 2, 3, 5, 6] {
                s[k as usize] = exe::newsitem_get_slot(news, k);
            }
            s
        };
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        if s[6] != 0 {
            fmt(buf, 0x988e98, np); // "<player> absent from training"
        } else if s[5] == 0 {
            fmt(buf, 0x988e54, np); // "<player> rumoured to have missed training"
        } else {
            fmt(buf, 0x988e20, np); // "<player> disciplined by club"
        }
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 missed training body");
    let s: [u32; 11] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (club, kind, coach) = (s[4], s[5], s[7] as i32);
    let coach_ids = [s[8], s[9], s[10]];
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let mut coach_name = [0u8; 0x190];
    if coach != -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
    }
    let cp = coach_name.as_ptr() as usize;
    if awol_is_human_club(club) {
        covered!("sx05_5 missed training at the human club");
        let (alone, with_coach) = match kind {
            0 => (0x989250, 0x9891d0),
            1 => (0x98933c, 0x9892c0),
            _ => (0x98942c, 0x9893a8),
        };
        if coach == -1 {
            fmt(buf, alone, np);
        } else {
            fmt(buf, with_coach, cp, np);
        }
    } else {
        let (cn, g) = awol_club(club);
        match (coach, kind) {
            (-1, _) => fmt(buf, 0x988ed0, g, g, 0x64, cn, np),
            (_, 0) => fmt(buf, 0x988f6c, g, g, 0x64, cn, cp, np),
            (_, 1) => fmt(buf, 0x989030, g, g, cn, cp, np),
            _ => fmt(buf, 0x989100, g, g, cn, cp, np),
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    if coach != -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
    let c = club_at(club as i32 as isize);
    exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
}

/// Writes the headline or the body of the news that a coach worries about a player's fitness after he turned up late
/// for training (slots: 0 the player, 1 his club, 2 the worry flag, 3..5 his name ids, 6 the coach (-1 none), 7..9 the
/// coach's name ids). The text goes through a local buffer, not 0xde1f64.
///
/// The headline (style 5): "<player> fitness worry" with the flag, else "<player> lacking fitness"; the player's
/// link. The body (style 1): with a coach, "<club> coach <coach> has voiced concerns over <player>'s general fitness"
/// (flag) or "reported that <player> turned up late"; without one, "various members of the <club> backroom staff..."
/// (flag) or "<player>'s apparent lack of match fitness... has disappointed senior members of <club>'s playing staff".
/// Then the player's link, the club's link (short name) and the coach's link.
/// Original: `write_fitness_worry_news_text` (0x41b030, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_fitness_worry_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let mut text = [0u8; 0x1378];
    let tp = text.as_mut_ptr() as usize;
    let s: [u32; 10] = awol_news_slots(news);
    let (club, worry, coach) = (s[1], s[2], s[6] as i32);
    let ids = [s[3], s[4], s[5]];
    if full == 0 {
        let name = awol_news_name(ids, s[0], 5);
        if worry != 0 {
            fmt(tp, 0x989820, 0x64, name.as_ptr() as usize); // "<player>{} fitness worry"
        } else {
            fmt(tp, 0x9897ec, name.as_ptr() as usize); // "<player> lacking fitness"
        }
        exe::text_set_string(out, tp as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 fitness worry body");
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let coach_ids = [s[7], s[8], s[9]];
    let (cn, g) = awol_club(club);
    let mut coach_name = [0u8; 0x190];
    if coach > -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
        let cp = coach_name.as_ptr() as usize;
        if worry != 0 {
            fmt(tp, 0x989718, g, g, 0x64, cn, cp, np);
        } else {
            fmt(tp, 0x98963c, g, g, cn, cp, np);
        }
    } else if worry != 0 {
        fmt(tp, 0x98956c, g, g, 0x64, cn, np);
    } else {
        covered!("sx05_5 fitness worry without a coach");
        fmt(tp, 0x98949c, 0x64, g, g, np, cn);
    }
    exe::text_set_string(out, tp as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    exe::text_add_link(out, cn as *const u8, 0x7d5, club_at(club as i32 as isize));
    if coach > -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_write_awol_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_AWOL_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_awol_news_text(this, news, full, t),
        || format!("write_awol_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_missed_training_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_MISSED_TRAINING_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_missed_training_news_text(this, news, full, t),
        || format!("write_missed_training_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_fitness_worry_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_FITNESS_WORRY_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_fitness_worry_news_text(this, news, full, t),
        || format!("write_fitness_worry_news_text(news {:#x}, full {})", news, full));
}
