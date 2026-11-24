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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%);
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
