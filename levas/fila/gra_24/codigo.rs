// ---------- batch %%LEVA%%: the news of a cup tie's result (comp_util.cpp) ----------

/// The round name code of the final (`fixture::ROUND_NAME` without its variant, with no `fixture::ROUND_PREFIX`).
const ROUND_FINAL: i16 = 0x96;

/// How a tie was settled (`fixture::HOME_RESULT_MARK`, else `fixture::AWAY_RESULT_MARK`), as the cup tie news reads
/// it; any other mark goes by the verdict.
mod tie_mark {
    pub(crate) const EXTRA_TIME: i8 = 3;
    pub(crate) const PENALTIES: i8 = 4;
    pub(crate) const SHOOTOUT: i8 = 5;
    pub(crate) const AWAY_GOALS: i8 = 6;
    pub(crate) const GOLDEN_GOAL: i8 = 7;
    pub(crate) const HIGHER_SEED: i8 = 8;
}

/// The verdict on the result that the caller passes to the cup tie news (the wording of an ordinary result).
mod tie_verdict {
    pub(crate) const EXCELLENT: u8 = 2;
    pub(crate) const FINE: u8 = 3;
    pub(crate) const DISAPPOINTING: u8 = 5;
    pub(crate) const SHOCK: u8 = 6;
    pub(crate) const VERY_DISAPPOINTING: u8 = 7;
}

/// What a cup tie sentence names after the club (with its gender pair before the club's).
#[derive(Clone, Copy, PartialEq)]
enum TieMiddle {
    /// The round label and its gender.
    Round,
    /// The competition's name and gender ("have won the <competition>"); the label then becomes the competition's
    /// name, for its link.
    Trophy,
    /// The competition's name with the round label's gender; the label then becomes the competition's name.
    TrophyRoundGender,
    /// The competition's name and gender, the label kept (a final settled by a forfeit).
    TrophyKeepLabel,
    /// The next stage's name and gender.
    NextStage,
    /// Nothing ("are through to the next stage").
    Nothing,
}

/// The goals in a cup tie sentence.
#[derive(Clone, Copy, PartialEq)]
enum TieScore {
    /// The club's goals, then the opponent's.
    OwnFirst,
    /// The opponent's goals, then the club's; the score link's text is rewritten the same way.
    OpponentFirst,
    /// No goals (a forfeit).
    Nothing,
}

/// The pieces of the cup tie news of one club (`comp_util_write_cup_tie_result_news`).
struct TieNews {
    club: Club,
    opponent: Club,
    comp: Competition,
    stadium: Stadium,
    own_goals: i32,
    opponent_goals: i32,
    label_gender: i32,
    next_gender: i32,
    won: bool,
    final_round: bool,
    single_match: bool,
    mark: i8,
    verdict: u8,
    label: [u8; 500],
    next: [u8; 100],
    venue: [u8; 300],
    score: [u8; 16],
}

impl TieNews {
    /// Writes the sentence `format` into `NEWS_TEXT_BUFFER`. The arguments: the club's gender twice, the middle's
    /// gender twice (when there is a middle), the opponent's gender twice, the stadium's gender twice (with `venue`);
    /// then the club's short name, the middle's text, the goals, the opponent's short name and the venue text (with
    /// `venue`). The goals with the opponent first also rewrite the score text (format 0x9a3d94) that way; a trophy
    /// middle (not the forfeit one) copies the competition's name into the label.
    unsafe fn say(&mut self, format: usize, middle: TieMiddle, score: TieScore, venue: bool) {
        let gender = |g: u8| g as i8 as i32 as usize;
        let comp = self.comp;
        let middle_part = match middle {
            TieMiddle::Round => Some((self.label_gender as usize, self.label.as_ptr() as usize)),
            TieMiddle::Trophy | TieMiddle::TrophyKeepLabel => Some((gender(comp.gender_name()), comp.name_addr())),
            TieMiddle::TrophyRoundGender => Some((self.label_gender as usize, comp.name_addr())),
            TieMiddle::NextStage => Some((self.next_gender as usize, self.next.as_ptr() as usize)),
            TieMiddle::Nothing => None,
        };
        let mut a = [0usize; 14];
        let mut n = 0;
        let mut put = |v: usize| {
            a[n] = v;
            n += 1;
        };
        let club_gender = gender(self.club.short_gender_name());
        put(club_gender);
        put(club_gender);
        if let Some((g, _)) = middle_part {
            put(g);
            put(g);
        }
        let opponent_gender = gender(self.opponent.short_gender_name());
        put(opponent_gender);
        put(opponent_gender);
        if venue {
            let stadium_gender = gender(self.stadium.gender_name());
            put(stadium_gender);
            put(stadium_gender);
        }
        put(self.club.addr() + club::SHORT_NAME);
        if let Some((_, t)) = middle_part {
            put(t);
        }
        match score {
            TieScore::OwnFirst => {
                put(self.own_goals as usize);
                put(self.opponent_goals as usize);
            }
            TieScore::OpponentFirst => {
                put(self.opponent_goals as usize);
                put(self.own_goals as usize);
            }
            TieScore::Nothing => {}
        }
        put(self.opponent.addr() + club::SHORT_NAME);
        if venue {
            put(self.venue.as_ptr() as usize);
        }
        let fmt = news_text_format();
        fmt(NEWS_TEXT_BUFFER, format, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13]);
        if score == TieScore::OpponentFirst {
            fmt(self.score.as_mut_ptr() as usize, 0x9a3d94, self.opponent_goals, self.own_goals); // "<goals>-<goals>"
        }
        if matches!(middle, TieMiddle::Trophy | TieMiddle::TrophyRoundGender) {
            let name = comp.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                self.label[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        }
    }

    /// `say` with the venue when the fixture has a stadium (`with_venue`), else without it (`without`).
    unsafe fn say_at(&mut self, with_venue: usize, without: usize, middle: TieMiddle, score: TieScore) {
        if !self.stadium.is_null() {
            self.say(with_venue, middle, score, true)
        } else {
            self.say(without, middle, score, false)
        }
    }

    /// The forfeit lost: "have lost in the <round> by being the lower seeded team" (0x99ecec when the opponent's mark
    /// is below the club's, else 0x99ec54).
    unsafe fn say_forfeit_lost(&mut self, own_mark: i8, opponent_mark: i8) {
        covered!("gra_24 forfeit lost");
        let format = if opponent_mark < own_mark { 0x99ecec } else { 0x99ec54 };
        self.say(format, TieMiddle::Round, TieScore::Nothing, false)
    }

    /// The defeats on a settling mark, shared by the three kinds of sentence: away goals, the lower seed, extra time
    /// (the opponent's goals first), penalties, the shootout and the golden goal, with and without the venue.
    unsafe fn say_mark_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::{OpponentFirst, OwnFirst};
        match self.mark {
            tie_mark::AWAY_GOALS => self.say_at(0x9a0cb4, 0x9a2294, Round, OwnFirst),
            tie_mark::HIGHER_SEED => self.say_at(0x9a0a60, 0x9a2070, Round, OwnFirst),
            tie_mark::EXTRA_TIME => self.say_at(0x9a3a0c, 0x9a3964, Round, OpponentFirst),
            tie_mark::PENALTIES => self.say_at(0x9a3754, 0x9a1d40, Round, OwnFirst),
            tie_mark::SHOOTOUT => self.say_at(0x9a3544, 0x9a1b5c, Round, OwnFirst),
            _ => self.say_at(0x99fea4, 0x9a1954, Round, OwnFirst), // the golden goal
        }
    }

    /// An ordinary defeat by the verdict: disappointing (0x9a2b50 with the venue in a single match, else 0x9a2ab0),
    /// very disappointing (0x9a29f0 / 0x9a294c), any other verdict (single match 0x9a2630 / 0x9a25a0, two legs
    /// 0x9a26dc); the opponent's goals first. A shock defeat is written by the caller; in a round with no stage after
    /// it the other verdicts have their own two-legs sentence with the venue (`say_round_result`).
    unsafe fn say_verdict_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::OpponentFirst;
        let with_venue = self.single_match && !self.stadium.is_null();
        match self.verdict {
            tie_verdict::DISAPPOINTING => self.say(if with_venue { 0x9a2b50 } else { 0x9a2ab0 }, Round, OpponentFirst, with_venue),
            tie_verdict::VERY_DISAPPOINTING => self.say(if with_venue { 0x9a29f0 } else { 0x9a294c }, Round, OpponentFirst, with_venue),
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say(0x9a26dc, Round, OpponentFirst, false),
        }
    }

    /// The tie decided in a round with no stage after it named: "have won in the <round>" or, in the final, "have won
    /// the <competition>", or the defeat (block of `comp_util_write_cup_tie_result_news` for a next stage below 1 other
    /// than −2).
    unsafe fn say_round_result(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{Round, Trophy, TrophyKeepLabel, TrophyRoundGender};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 round result");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit won");
            let club_first = own_mark < opponent_mark;
            if self.final_round {
                return self.say(if club_first { 0x99eef8 } else { 0x99ee60 }, TrophyKeepLabel, TieScore::Nothing, false);
            }
            return self.say(if club_first { 0x99edec } else { 0x99ed54 }, Round, TieScore::Nothing, false);
        }
        // The wins on a settling mark: (in a round with/without the venue, in the final with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a0e20, 0x9a0d78, 0x9a0f8c, 0x9a0ee4)),
            tie_mark::EXTRA_TIME => Some((0x9a0840, 0x9a079c, 0x9a09a0, 0x9a08fc)),
            tie_mark::PENALTIES => Some((0x9a058c, 0x9a04f0, 0x9a06e4, 0x9a0644)),
            tie_mark::SHOOTOUT => Some((0x9a02e0, 0x9a0244, 0x9a0438, 0x9a0398)),
            tie_mark::GOLDEN_GOAL => Some((0x9a0010, 0x99ff68, 0x9a0180, 0x9a00d4)),
            _ => None,
        };
        if self.mark == tie_mark::HIGHER_SEED {
            if self.won {
                return self.say_at(0x9a0be4, 0x9a0b30, Round, OwnFirst); // the higher seed wins the round, even a final
            }
            return self.say_mark_defeat();
        }
        if let Some((round, round_plain, trophy, trophy_plain)) = mark_win {
            if !self.won {
                return self.say_mark_defeat();
            }
            if !self.final_round {
                return self.say_at(round, round_plain, Round, OwnFirst);
            }
            covered!("gra_24 trophy won on a mark");
            return self.say_at(trophy, trophy_plain, Trophy, OwnFirst);
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result (these two whatever the outcome): (single match in a round
            // with/without the venue, in the final with/without the venue; two legs in a round, in the final).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (round, round_plain, trophy, trophy_plain, legs, legs_trophy) = match self.verdict {
                    tie_verdict::SHOCK => (0x99fb6c, 0x99fad8, 0x99fcb4, 0x99fc1c, 0x99fd64, 0x99fe04),
                    tie_verdict::EXCELLENT => (0x99f788, 0x99f6f0, 0x99f8d8, 0x99f83c, 0x99f990, 0x99fa34),
                    _ => (0x99f3bc, 0x9a2cb4, 0x99f500, 0x99f46c, 0x99f5b0, 0x99f650),
                };
                match (self.single_match, self.final_round) {
                    (true, false) => self.say_at(round, round_plain, Round, OwnFirst),
                    (true, true) => self.say_at(trophy, trophy_plain, Trophy, OwnFirst),
                    (false, false) => self.say(legs, Round, OwnFirst, false),
                    (false, true) => self.say(legs_trophy, Trophy, OwnFirst, false),
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => match (self.single_match, self.final_round) {
                (true, false) => self.say_at(0x99f0b0, 0x99f020, Round, OwnFirst), // gra24: the plain win in a round
                (true, true) => self.say_at(0x99f1e4, 0x99f158, TrophyRoundGender, OwnFirst),
                (false, false) => self.say(0x99f28c, Round, OwnFirst, false),
                (false, true) => self.say(0x99f324, Trophy, OwnFirst, false),
            },
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say_at(0x99ef6c, 0x9a26dc, Round, OpponentFirst), // only here two legs have a venue variant
        }
    }

    /// The tie that sends the club "through to the next stage" without naming it, or the defeat (next stage −2).
    unsafe fn say_through_unnamed(&mut self) {
        use TieMiddle::{Nothing, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to the next stage");
        // The wins on a settling mark (with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a23dc, 0x9a233c)),
            tie_mark::HIGHER_SEED => Some((0x9a21d0, 0x9a2124)),
            tie_mark::EXTRA_TIME => Some((0x9a1fbc, 0x9a1f24)),
            tie_mark::PENALTIES => Some((0x9a1e74, 0x9a1de0)),
            tie_mark::SHOOTOUT => Some((0x9a1c90, 0x9a1bfc)),
            tie_mark::GOLDEN_GOAL => Some((0x9a1aa0, 0x9a1a00)),
            _ => None,
        };
        if let Some((with_venue, plain)) = mark_win {
            if self.won {
                return self.say_at(with_venue, plain, Nothing, OwnFirst);
            }
            return self.say_mark_defeat();
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result: (single match with/without the venue, two legs).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (with_venue, plain, legs) = match self.verdict {
                    tie_verdict::SHOCK => (0x9a1818, 0x9a178c, 0x9a18c0),
                    tie_verdict::EXCELLENT => (0x9a1458, 0x9a13c8, 0x9a1504),
                    _ => (0x9a1290, 0x9a1204, 0x9a1334),
                };
                if self.single_match {
                    self.say_at(with_venue, plain, Nothing, OwnFirst)
                } else {
                    self.say(legs, Nothing, OwnFirst, false)
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => {
                if self.single_match {
                    self.say_at(0x9a10d4, 0x9a1050, Nothing, OwnFirst)
                } else {
                    self.say(0x9a1174, Nothing, OwnFirst, false)
                }
            }
            _ => self.say_verdict_defeat(),
        }
    }

    /// The tie that sends the club through to the named next stage, or the defeat (next stage 1 or above).
    unsafe fn say_through_to_stage(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{NextStage, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to a named stage");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit through");
            let format = if own_mark < opponent_mark { 0x9a252c } else { 0x9a2494 };
            return self.say(format, NextStage, TieScore::Nothing, false);
        }
        match self.mark {
            // Away goals and the higher seed: no venue variant; the defeat is the plain sentence.
            tie_mark::AWAY_GOALS => {
                if self.won {
                    self.say(0x9a3ce4, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2294, Round, OwnFirst, false)
                }
            }
            tie_mark::HIGHER_SEED => {
                if self.won {
                    self.say(0x9a3c30, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2070, Round, OwnFirst, false)
                }
            }
            tie_mark::EXTRA_TIME | tie_mark::PENALTIES | tie_mark::SHOOTOUT | tie_mark::GOLDEN_GOAL => {
                if !self.won {
                    return self.say_mark_defeat();
                }
                let (with_venue, plain) = match self.mark {
                    tie_mark::EXTRA_TIME => (0x9a3b70, 0x9a3acc),
                    tie_mark::PENALTIES => (0x9a38ac, 0x9a380c),
                    tie_mark::SHOOTOUT => (0x9a369c, 0x9a35fc),
                    _ => (0x9a3480, 0x9a33d4),
                };
                self.say_at(with_venue, plain, NextStage, OwnFirst)
            }
            _ => match self.verdict {
                tie_verdict::SHOCK if self.won => {
                    if self.single_match {
                        self.say_at(0x9a3284, 0x9a31ec, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a3334, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::SHOCK => {
                    if self.single_match {
                        self.say_at(0x9a308c, 0x9a2fe0, Round, OpponentFirst)
                    } else {
                        self.say(0x9a3148, Round, OpponentFirst, false)
                    }
                }
                // An excellent and a fine result, whatever the outcome: a single match is "have won in the <round>",
                // two legs "are through to <stage>".
                tie_verdict::EXCELLENT => {
                    if self.single_match {
                        self.say_at(0x9a2e84, 0x9a2de8, Round, OwnFirst)
                    } else {
                        self.say(0x9a2f38, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::FINE => {
                    if self.single_match {
                        self.say_at(0x99f3bc, 0x9a2cb4, Round, OwnFirst)
                    } else {
                        self.say(0x9a2d48, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::DISAPPOINTING if !self.single_match => self.say(0x9a2c0c, Round, OpponentFirst, false),
                tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
                _ if self.won => {
                    if self.single_match {
                        self.say_at(0x9a2808, 0x9a2778, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a28b4, NextStage, OwnFirst, false)
                    }
                }
                _ => self.say_verdict_defeat(),
            },
        }
    }
}

/// Writes the news sentence of a club's cup tie result (won, through, lost) into the Text `text`, with the score,
/// the round, the venue and the links.
///
/// `comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, text)` (cdecl). The club is
/// `club_at(club_index)`; at home (`club_index` = `.home_club_id()`) the opponent is `.away_club()`, the club's mark
/// `.home_result_mark()` and the opponent's `.away_result_mark()`, away the other way round; the settling mark is the
/// home mark, else the away mark. The goals: the aggregate (`.home_aggregate()` other than −1), else after extra time
/// (`.home_goals_extra_time()` other than −1), else at full time, the club's first; the score text "<goals>-<goals>"
/// (format 0x9a3d94, 16 bytes). Won: `outcome` (byte) other than 2, 4 and −1. The round: `.round_name()` without its
/// variant (`round_name_without_variant`); the final is round 0x96 with no `.round_prefix()`.
///
/// The round label (form 1, 500 bytes, `exe::comp_util_format_comp_stage_round_label`): in the World Cup Oceania
/// qualifying section with a club (home, then away) whose nation's continent is not Oceania, and the same for the Asian
/// section and Asia, the label of the FIFA World Cup row with round 0xa0 and its gender; otherwise the competition's,
/// stage and round label, gendered by the raw round name (`exe::comp_util_get_stage_name_gender`), else the stage,
/// else the competition's gender (`.gender_name()`), else 1. With a stadium (`.stadium()`), the venue: its name, or
/// "<name>, <city>" (0x98f6ac) in languages 0 and 1 and "<name> (<city>)" (0x9a3d8c) in the others
/// (`exe::langlib_get_language`; `FN_CRT_SPRINTF`).
///
/// By `next_stage` (word): 1 or above names the next stage (`exe::comp_util_get_stage_name`, 100 bytes, and its gender;
/// `TieNews::say_through_to_stage`); −2 "the next stage" (`TieNews::say_through_unnamed`); otherwise the round or the
/// trophy (`TieNews::say_round_result`). Each picks the sentence by the mark, then the verdict (`tie_verdict`), the
/// outcome, the final, a single match (`.home_aggregate()` −1) and the stadium (`TieNews::say`).
///
/// Then the text is set (`exe::text_set_string`) and linked: the club's and the opponent's short names (0x7d5); the
/// score text (0x7d3, the match statistics slot) when the statistics entry is the fixture's
/// (`exe::match_stats_fixture_entry_matches`: slot, home and away ids, day, year); with the competition current on the
/// fixture's date (`is_comp_current_on_date` of `G_FRIENDLY`) the label (2000, the competition), and the next stage's
/// name when there is one (2000). The next stage's buffer starts with the byte at 0xad9c64.
/// Original: `comp_util_write_cup_tie_result_news` (0x4b0b80, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    covered!("gra_24 cup tie news");
    let f = Fixture(fixture);
    let club = club_at(club_index as isize);
    let (home_mark, away_mark) = (f.home_result_mark(), f.away_result_mark());
    let mark = if home_mark != 0 { home_mark } else { away_mark };
    let at_home = club_index as usize == f.home_club_id();
    let (opponent, own_mark, opponent_mark) = if at_home { (f.away_club(), home_mark, away_mark) } else { (f.home_club(), away_mark, home_mark) };
    let (home_goals, away_goals) = if f.home_aggregate() != -1 {
        (f.home_aggregate(), f.away_aggregate())
    } else if f.home_goals_extra_time() != -1 {
        (f.home_goals_extra_time(), f.away_goals_extra_time())
    } else {
        (f.home_goals(), f.away_goals())
    };
    let (own_goals, opponent_goals) = if at_home { (home_goals as i32, away_goals as i32) } else { (away_goals as i32, home_goals as i32) };
    let mut news = TieNews {
        club,
        opponent,
        comp: f.competition(),
        stadium: Stadium(f.stadium()),
        own_goals,
        opponent_goals,
        label_gender: 0,
        next_gender: 0,
        won: !matches!(outcome as u8, 2 | 4 | 0xff),
        final_round: false,
        single_match: f.home_aggregate() == -1,
        mark,
        verdict: verdict as u8,
        label: [0; 500],
        next: [0; 100],
        venue: [0; 300],
        score: [0; 16],
    };
    news.next[0] = read_u8(0xad9c64);
    let fmt = news_text_format();
    fmt(news.score.as_mut_ptr() as usize, 0x9a3d94, own_goals, opponent_goals);
    let raw_round = f.round_name();
    let round = round_name_without_variant(raw_round);
    let prefix = f.round_prefix();
    news.final_round = round == ROUND_FINAL && prefix == 0;
    let comp = news.comp;
    let outside = |continent_key: usize| {
        let away_from = |c: Club| {
            let nation = c.nation();
            if nation.is_null() {
                return false;
            }
            let continent = nation.continent();
            continent != 0 && read_i32(continent) != read_i32(continent_key)
        };
        away_from(f.home_club()) || away_from(f.away_club())
    };
    let comp_id = f.comp_id();
    let world_cup_label = if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_OCEANIA)
    } else if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_ASIA)
    } else {
        false
    };
    let label = news.label.as_mut_ptr();
    if world_cup_label {
        covered!("gra_24 world cup qualifying label");
        let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) as usize).wrapping_mul(0x6b));
        exe::comp_util_format_comp_stage_round_label(world_cup, 0, 0xa0, 1, label, 500);
        news.label_gender = exe::comp_util_get_stage_name_gender(0xa0, 0) as i8 as i32;
    } else {
        exe::comp_util_format_comp_stage_round_label(comp, prefix as u16 as u32, round as u16 as u32, 1, label, 500);
        let gender = if raw_round != 0 {
            exe::comp_util_get_stage_name_gender(raw_round as u16 as u32, 0)
        } else if prefix != 0 {
            exe::comp_util_get_stage_name_gender(prefix as u16 as u32, 0)
        } else if comp.is_null() {
            1
        } else {
            comp.gender_name()
        };
        news.label_gender = gender as i8 as i32;
    }
    let stadium = news.stadium;
    if !stadium.is_null() {
        let lang = exe::langlib_get_language();
        let format = if lang == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c }; // "%s, %s" / "%s (%s)"
        let city = stadium.city();
        if city == 0 {
            let name = stadium.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                news.venue[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        } else {
            covered!("gra_24 venue with the city");
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(news.venue.as_mut_ptr() as usize, format, stadium.name_addr(), city + 4); // the city's name
        }
    }
    let stage = next_stage as u16 as i16;
    if stage >= 1 {
        exe::comp_util_get_stage_name(news.next.as_mut_ptr(), 100, next_stage, 0);
        news.next_gender = exe::comp_util_get_stage_name_gender(next_stage, 0) as i8 as i32;
        news.say_through_to_stage(own_mark, opponent_mark);
    } else if stage == -2 {
        news.say_through_unnamed();
    } else {
        news.say_round_result(own_mark, opponent_mark);
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (opponent.addr() + club::SHORT_NAME) as *const u8, 0x7d5, opponent); // gra24: the opponent's link
    let slot = f.match_stats_slot();
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot as i32, f.home_club_id() as i32, f.away_club_id() as i32, f.day() as i16 as i32, f.year() as i16 as i32) != 0 {
        exe::text_add_link(text, news.score.as_ptr(), 0x7d3, slot);
    }
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
    if is_comp_current_on_date(read_usize(G_FRIENDLY), comp.addr(), date.as_ptr() as usize) != 0 {
        exe::text_add_link(text, news.label.as_ptr(), 2000, comp);
        if news.next[0] != 0 {
            exe::text_add_link(text, news.next.as_ptr(), 2000, comp);
        }
    }
}

/// Comparer for `comp_util_write_cup_tie_result_news`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(club_index, fixture, verdict, outcome, next_stage, text),
        |t| comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, t),
        || format!("comp_util_write_cup_tie_result_news(club {}, fixture {:#x}, verdict {}, outcome {}, next stage {})", club_index, fixture, verdict as u8, outcome as u8 as i8, next_stage as u16 as i16));
}
