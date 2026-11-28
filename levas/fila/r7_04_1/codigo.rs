// --- r7_04_1: the coach report news text (coach.cpp) ---

/// How settled the player is (slot 8 of the coach report, `coach_post_report_news`).
/// Over 1000 days at the club with loyalty above 10.
const COACH_SETTLED_LONG_SERVING: u32 = 1;
/// Fewer days than (21 - adaptability) x 5 + 90.
const COACH_SETTLED_SETTLING: u32 = 2;
/// Settled in: those days or more.
const COACH_SETTLED_SETTLED: u32 = 4;

/// The player's career stage (slot 9, `coach_get_player_career_stage`): under 18, under 24, veteran, journeyman and
/// established (caps or a reputation above 6000).
const COACH_STAGE_YOUTH: u32 = 1;
const COACH_STAGE_YOUNG: u32 = 2;
const COACH_STAGE_VETERAN: u32 = 4;
const COACH_STAGE_JOURNEYMAN: u32 = 8;
const COACH_STAGE_ESTABLISHED: u32 = 0x10;

/// The coach's verdict on the player now (slot 10, `coach_assess_player`): his band against the club's target (not
/// good enough, squad player, good enough, a star), then the marks.
const COACH_VERDICT_NOT_GOOD_ENOUGH: u32 = 1;
const COACH_VERDICT_SQUAD_PLAYER: u32 = 2;
const COACH_VERDICT_GOOD_ENOUGH: u32 = 4;
const COACH_VERDICT_STAR: u32 = 8;
/// The coach has not had the days he needs with the player.
const COACH_VERDICT_TOO_SOON: u32 = 0x10;
/// Set by the softening for a player often in the side; the reports read it as "too active in the first team"
/// [inference].
const COACH_VERDICT_FIRST_TEAM: u32 = 0x20;
const COACH_VERDICT_TRANSFER_LISTED: u32 = 0x40;
/// His job at the club is not "player".
const COACH_VERDICT_NOT_A_PLAYER: u32 = 0x80;
/// The coach thinks the club over-rates him (a "not good enough" in a key squad status).
const COACH_VERDICT_OVERRATED: u32 = 0x100;

/// The coach's verdict on the player's potential (slot 11, `coach_assess_player`).
const COACH_POTENTIAL_LOW: u32 = 1;
const COACH_POTENTIAL_SOME: u32 = 2;
const COACH_POTENTIAL_GOOD: u32 = 4;
const COACH_POTENTIAL_GREAT: u32 = 8;
/// Past 24: the potential is no longer judged.
const COACH_POTENTIAL_PAST_IT: u32 = 0x10;
const COACH_POTENTIAL_TOO_SOON: u32 = 0x20;
const COACH_POTENTIAL_LOAN_OR_YOUTH: u32 = 0x40;
const COACH_POTENTIAL_VERSATILE: u32 = 0x80;
/// Above 0.8 x target but still young and new at the club.
const COACH_POTENTIAL_PROMISING: u32 = 0x100;

/// The developers' in-jokes: a player with one of these names, born before 11/11/1977, gets this sentence instead of
/// the report (the coach's name its only argument). "Paul Norman" has a second sentence on odd days
/// (`COACH_JOKE_PAUL_NORMAN_ODD_DAY`).
const COACH_REPORT_JOKES: [(usize, u32); 13] = [
    (0x99b3d4, 0x99b250), // "Paul Norman": "... is an alien scout ..."
    (0x99b240, 0x99b180), // "Oliver Collyer"
    (0x99b170, 0x99b084), // "Paul Collyer"
    (0x99b074, 0x99af94), // "Kevin Turner"
    (0x99af88, 0x99aecc), // "Marc Duffy"
    (0x99aebc, 0x99ae00), // "Mark Woodger"
    (0x99adf0, 0x99ad68), // "Marc Vaughan"
    (0x99ad58, 0x99accc), // "Pete Sottrel"
    (0x99acbc, 0x99ac18), // "Svein Kvernoey"
    (0x99ac08, 0x99ab38), // "Miles Dineen"
    (0x99ab28, 0x99aa40), // "Keith Flannery"
    (0x99aa30, 0x99a970), // "Marlon Davidson"
    (0x99a960, 0x99a88c), // "Grant Appleyard"
];
/// "<coach> is amazed that Paul Norman has left his computer bank ...": the first joke on an odd day of the year.
const COACH_JOKE_PAUL_NORMAN_ODD_DAY: u32 = 0x99b2f4;

/// The slots of a coach report news item (0x1391), as `coach_post_report_news` writes them.
struct CoachReport {
    /// slots 0..3 and 4..7: the coach's and the player's ids and name ids (first, second, common).
    coach: i32,
    coach_names: [u32; 3],
    player: i32,
    player_names: [u32; 3],
    /// slots 8..11: `COACH_SETTLED_*`, `COACH_STAGE_*`, `COACH_VERDICT_*`, `COACH_POTENTIAL_*`.
    settled: u32,
    stage: u32,
    verdict: u32,
    potential: u32,
    /// slots 12 and 13: the strengths and weaknesses the coach noticed (`player_search_assess_player_traits`).
    noticed_strengths: u32,
    noticed_weaknesses: u32,
    /// slot 14: the player's strengths for the coach's club (`exe::write_player_description`); 15 is not read.
    strengths: u32,
    /// slot 16: the position mask; 17: the age (byte); 18: his club's reputation.
    positions: u32,
    age: u8,
    reputation: u32,
    /// slot 19 (signed word): the coach's club.
    club: Club,
    /// slots 20 and 21 (bytes): a future transfer is agreed; absent without leave. 22 is not read.
    future_transfer: bool,
    awol: bool,
}

impl CoachReport {
    unsafe fn read(news: usize) -> Self {
        let slot = |k: u32| exe::newsitem_get_slot(news, k);
        let names = |k: u32| [slot(k), slot(k + 1), slot(k + 2)];
        CoachReport { coach: slot(0) as i32, coach_names: names(1), player: slot(4) as i32, player_names: names(5), settled: slot(8),
            stage: slot(9), verdict: slot(10), potential: slot(11), noticed_strengths: slot(12), noticed_weaknesses: slot(13), strengths: slot(14),
            positions: slot(16), age: slot(17) as u8, reputation: slot(18), club: club_at(slot(19) as i16 as isize),
            future_transfer: slot(20) as u8 != 0, awol: slot(21) as u8 != 0 }
    }

    fn has_verdict(&self, bits: u32) -> bool {
        self.verdict & bits != 0
    }

    fn has_potential(&self, bits: u32) -> bool {
        self.potential & bits != 0
    }

    fn is_stage(&self, bits: u32) -> bool {
        self.stage & bits != 0
    }

    fn is_settled(&self, bits: u32) -> bool {
        self.settled & bits != 0
    }

    /// The original's choice among alternative wordings: `id % n + id` on the player's id, so the player with id 0
    /// gets the first wording, the one with id 1 the third, every other player the last, and the second is never
    /// chosen (DEFEITOS_DO_ORIGINAL).
    fn variant(&self, n: i32) -> i32 {
        (self.player % n).wrapping_add(self.player)
    }

    /// A name (`form` 1 full, 5 short) from three name ids into `buf`.
    unsafe fn format_name(names: &[u32; 3], id: i32, buf: &mut [u8], form: u32) {
        exe::database_format_name_from_ids(names[0], names[1], names[2], buf.as_mut_ptr(), buf.len() as u32, form, id as u32);
    }

    /// The person `id` still has these names (`exe::database_staff_matches_names`, no date).
    unsafe fn still_named(names: &[u32; 3], id: i32) -> Option<Staff> {
        let person = staff_at(id as isize);
        (exe::database_staff_matches_names(person, names[0], names[1], names[2], 0usize) != 0).then_some(person)
    }

    /// The player is a regen (`staff_state_status_flags::REGEN_MARK` of his 79-byte row).
    unsafe fn player_is_regen(&self) -> bool {
        let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((self.player as usize).wrapping_mul(staff_state::SIZE));
        read_u8(row + staff_state::STATUS_FLAGS) & staff_state_status_flags::REGEN_MARK != 0
    }

    /// The traits paragraph follows the sentence: neither verdict is "too soon", and the potential is good or great or
    /// the player good enough or a star.
    fn traits_fit(&self) -> bool {
        !self.has_potential(COACH_POTENTIAL_TOO_SOON)
            && !self.has_verdict(COACH_VERDICT_TOO_SOON) // r7_04_1: not too soon
            && (self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) || self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR))
    }
}

/// The words a report sentence takes, in the order its template wants them.
#[derive(Clone, Copy)]
enum CoachWord {
    CoachName,
    PlayerName,
    PlayerShortName,
    /// The player description (`exe::write_player_description`) and its gender.
    PositionText,
    PositionGender,
    /// The fixed gender code 100 the original passes for some "{}" and "{s}" marks.
    Gender100,
    /// The coach's club: its name and short name, with their genders.
    ClubName,
    ClubGender,
    ClubShortName,
    ClubShortGender,
}

/// The texts of a report body.
struct CoachReportTexts {
    /// the coach's and the player's names, full (form 1) and short (form 5).
    coach: [u8; 0xc8],
    player: [u8; 0xc8],
    coach_short: [u8; 0xc8],
    player_short: [u8; 0xc8],
    /// the player's position name, overwritten by his description; and its gender.
    position: [u8; 0xc8],
    position_gender: i32,
    club: Club,
}

impl CoachReportTexts {
    unsafe fn word(&self, w: CoachWord) -> usize {
        let club = self.club.addr();
        match w {
            CoachWord::CoachName => self.coach.as_ptr() as usize,
            CoachWord::PlayerName => self.player.as_ptr() as usize,
            CoachWord::PlayerShortName => self.player_short.as_ptr() as usize,
            CoachWord::PositionText => self.position.as_ptr() as usize,
            CoachWord::PositionGender => self.position_gender as usize,
            CoachWord::Gender100 => 0x64,
            CoachWord::ClubName => self.club.name_addr(),
            CoachWord::ClubGender => self.club.gender_name() as usize,
            CoachWord::ClubShortName => club + club::SHORT_NAME,
            CoachWord::ClubShortGender => read_i8(club + club::SHORT_GENDER_NAME) as usize,
        }
    }

    /// The sentence `template` with `words` into the game's text buffer (`NEWS_TEXT_BUFFER`).
    unsafe fn say(&self, template: u32, words: &[CoachWord]) {
        let mut a = [0usize; 8];
        for (k, &w) in words.iter().enumerate() {
            a[k] = self.word(w);
        }
        story_lang!(NEWS_TEXT_BUFFER, template, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
    }

    /// The commonest sentence: "<coach> ... <position> <player> ...".
    unsafe fn position_line(&self, template: u32) {
        use CoachWord::*;
        self.say(template, &[PositionGender, PositionGender, CoachName, PositionText, PlayerName]);
    }

    /// "<coach> ... <player> ...".
    unsafe fn names_line(&self, template: u32) {
        use CoachWord::*;
        self.say(template, &[CoachName, PlayerName]);
    }

    /// The developers' in-joke for the player, when he has one of their names and was born before 11/11/1977
    /// (`COACH_REPORT_JOKES`; the year compared unsigned, then the day signed); true when it was written.
    unsafe fn developer_joke(&self, player: i32) -> bool {
        let person = staff_at(player as isize);
        for &(name, template) in COACH_REPORT_JOKES.iter() {
            if exe::staff_name_equals(person, name) == 0 {
                continue;
            }
            let mut cutoff = [0u16; 4];
            exe::date_date_549f70(&mut cutoff, 11, 11, 1977, u32::MAX);
            let older = match cutoff[1].cmp(&person.date_of_birth_year()) {
                core::cmp::Ordering::Greater => true,
                core::cmp::Ordering::Less => false,
                core::cmp::Ordering::Equal => cutoff[0] as i16 > person.date_of_birth(),
            };
            if !older {
                continue;
            }
            covered!("r7_04_1 developer joke");
            let odd_day = read_i16(G_CURRENT_DATE) % 2 != 0;
            let template = if template == COACH_REPORT_JOKES[0].1 && odd_day { COACH_JOKE_PAUL_NORMAN_ODD_DAY } else { template };
            story_lang!(NEWS_TEXT_BUFFER, template, self.coach.as_ptr());
            return true;
        }
        false
    }
}

/// Writes the headline or the body of a coach's report on a player (news item 0x1391, `coach_post_report_news`).
///
/// `this.write_coach_report_news_text(news, body, out)` (`this` = `COACH_REPORTS`, not read) reads the slots
/// (`CoachReport`); the player's position name (`exe::write_position_name` of slot 16) is overwritten by his
/// description (`exe::write_player_description`, with the strengths of slot 14 and that gender), whose gender the
/// sentences use; the coach's and the player's names, full and short (0xc8 bytes each).
/// - Headline (`write_coach_report_headline`): "<coach> files his report on <player>".
/// - Body: the traits paragraph (`player_search_write_traits_report_text`, 0x7bdee0: the noticed strengths, the
///   noticed weaknesses only for a regen, the club's reputation, the age, the short names). Then the sentence into the
///   game's text buffer: outside language 9 a developer's in-joke (`developer_joke`); otherwise, absent without leave,
///   "<coach> reminds you that <player> is not reporting for training" (language 0 through the C runtime's sprintf);
///   otherwise the verdict sentence (`CoachReport::report_sentence`). The text is the sentence, followed after a
///   blank line by the traits paragraph when there is one and it fits (`CoachReport::traits_fit`). Links: the player
///   (0x7d7, full then short) and the coach (0x7d7) when their names still match, then the coach's club (0x7d5) always.
///
/// The original leaves the last call's answer in eax; its only caller (`NewsItem::write_text`) ignores it: 1.
/// Defects of the original: the alternative wordings by `CoachReport::variant` reach only the players with ids 0 and 1;
/// two sentences are written and then overwritten at once; one test (`settled_in_young`) is always true; a settled-in
/// player with no career stage gets no sentence and the body repeats what the text buffer held.
/// Original: `write_coach_report_news_text` (0x487f60, coach.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_coach_report_news_text(_this: usize, news: usize, body: u32, out: usize) -> u32 {
    let r = CoachReport::read(news);
    let mut t = CoachReportTexts { coach: [0; 0xc8], player: [0; 0xc8], coach_short: [0; 0xc8], player_short: [0; 0xc8], position: [0; 0xc8],
        position_gender: 0, club: r.club };
    let g = exe::write_position_name(r.positions, t.position.as_mut_ptr() as usize, 0xc8, 1);
    let g = exe::write_player_description(r.player, r.positions, r.strengths, t.position.as_mut_ptr() as usize, 0xc8, g as u32, 1);
    t.position_gender = g as i8 as i32;
    CoachReport::format_name(&r.coach_names, r.coach, &mut t.coach, 1);
    CoachReport::format_name(&r.player_names, r.player, &mut t.player, 1);
    CoachReport::format_name(&r.coach_names, r.coach, &mut t.coach_short, 5);
    CoachReport::format_name(&r.player_names, r.player, &mut t.player_short, 5);
    if body == 0 {
        covered!("r7_04_1 report headline");
        write_coach_report_headline(&r, out);
        return 1;
    }
    covered!("r7_04_1 report body");
    let mut traits = [0u8; 0x3e8];
    let weaknesses = if r.player_is_regen() { r.noticed_weaknesses } else { 0 };
    // player_search_write_traits_report_text (0x7bdee0)
    let traits_text: unsafe extern "C" fn(u32, u32, u32, u32, usize, usize, usize) = core::mem::transmute(0x7bdee0usize);
    traits_text(r.noticed_strengths, weaknesses, r.reputation, r.age as u32, traits.as_mut_ptr() as usize, t.coach_short.as_ptr() as usize,
        t.player_short.as_ptr() as usize);
    if exe::langlib_get_language() == 9 || !t.developer_joke(r.player) {
        if r.awol {
            covered!("r7_04_1 absent without leave");
            if exe::langlib_get_language() == 0 {
                story_crt!(NEWS_TEXT_BUFFER, 0x99a7e0, t.coach.as_ptr(), t.player.as_ptr()); // "%s reminds you that %s is not reporting ..."
            } else {
                t.names_line(0x99a6f4); // "<coach> reminds you that <player> is not reporting for training ..."
            }
        } else {
            r.report_sentence(&t);
        }
    }
    let mut text = [0u8; 0x1388];
    if traits[0] != 0 && r.traits_fit() {
        covered!("r7_04_1 traits paragraph");
        story_crt!(text.as_mut_ptr(), 0x991948, NEWS_TEXT_BUFFER, traits.as_ptr()); // "%s\n\n%s"
    } else {
        story_crt!(text.as_mut_ptr(), 0x991944, NEWS_TEXT_BUFFER); // "%s"
    }
    exe::text_set_string(out, text.as_ptr());
    if let Some(p) = CoachReport::still_named(&r.player_names, r.player) {
        exe::text_add_link(out, t.player.as_ptr(), 0x7d7, p);
        exe::text_add_link(out, t.player_short.as_ptr(), 0x7d7, p);
    }
    if let Some(p) = CoachReport::still_named(&r.coach_names, r.coach) {
        exe::text_add_link(out, t.coach.as_ptr(), 0x7d7, p); // r7_04_1: the coach's links
        exe::text_add_link(out, t.coach_short.as_ptr(), 0x7d7, p);
    }
    exe::text_add_link(out, r.club.name_addr() as *const u8, 0x7d5, r.club);
    1
}

/// The headline: "<coach> files his report on <player>" (both short names, in 0x190-byte buffers) through the game's
/// text buffer; the player's and then the coach's name link to them (0x7d7) when the names still match.
unsafe fn write_coach_report_headline(r: &CoachReport, out: usize) {
    let (mut coach, mut player) = ([0u8; 0x190], [0u8; 0x190]);
    CoachReport::format_name(&r.coach_names, r.coach, &mut coach, 5);
    CoachReport::format_name(&r.player_names, r.player, &mut player, 5);
    story_lang!(NEWS_TEXT_BUFFER, 0x9918e4, coach.as_ptr(), player.as_ptr()); // "<coach> files his report on <player>"
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if let Some(p) = CoachReport::still_named(&r.player_names, r.player) {
        exe::text_add_link(out, player.as_ptr(), 0x7d7, p);
    }
    if let Some(p) = CoachReport::still_named(&r.coach_names, r.coach) {
        exe::text_add_link(out, coach.as_ptr(), 0x7d7, p);
    }
}

/// The verdict sentences, by the slots (into `NEWS_TEXT_BUFFER`; see `write_coach_report_news_text`).
impl CoachReport {
    /// The verdict sentence: a player on loan or on a youth contract first; otherwise by how settled he is (long
    /// serving, settling, settled in); with none of the three the original's fallback, the alien scout sentence.
    unsafe fn report_sentence(&self, t: &CoachReportTexts) {
        if self.has_potential(COACH_POTENTIAL_LOAN_OR_YOUTH) {
            self.loan_or_youth(t);
        } else if self.is_settled(COACH_SETTLED_LONG_SERVING) {
            self.long_serving(t);
        } else if self.is_settled(COACH_SETTLED_SETTLING) {
            self.settling(t);
        } else if self.is_settled(COACH_SETTLED_SETTLED) {
            self.settled_in(t);
        } else {
            t.names_line(0x991950); // "<coach> reckons <player> is an alien scout attempting to determine ..."
        }
    }

    /// A player on loan or on a youth contract: a low potential; under 24, "still a lot to learn" when not good enough
    /// and "very talented" otherwise; older, "no better than ours" when not good enough or a squad player, "talented" otherwise.
    unsafe fn loan_or_youth(&self, t: &CoachReportTexts) {
        if self.has_potential(COACH_POTENTIAL_LOW) {
            t.position_line(0x99a604); // "<coach> feels that <position> <player> lacks the quality to ..."
        } else if self.is_stage(COACH_STAGE_YOUTH | COACH_STAGE_YOUNG) {
            if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
                t.position_line(0x99a4e8); // "<coach> feels that <position> <player> still has a lot to learn, ..."
            } else {
                t.position_line(0x99a3f0); // "<coach> feels that <position> <player> is a very talented ..."
            }
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH | COACH_VERDICT_SQUAD_PLAYER) {
            t.position_line(0x99a304); // "<coach> feels that <position> <player> is no better than our ..."
        } else {
            t.position_line(0x99a21c); // "<coach> feels that <position> <player> is a talented individual ..."
        }
    }

    /// A player long at the club (over 1000 days, loyal), by his career stage.
    unsafe fn long_serving(&self, t: &CoachReportTexts) {
        if self.is_stage(COACH_STAGE_YOUTH) {
            self.long_serving_youth(t);
        } else if self.is_stage(COACH_STAGE_YOUNG) {
            self.long_serving_young(t);
        } else if self.is_stage(COACH_STAGE_VETERAN) {
            self.long_serving_veteran(t);
        } else if self.is_stage(COACH_STAGE_ESTABLISHED) {
            self.long_serving_established(t);
        } else {
            self.long_serving_other(t);
        }
    }

    /// Long serving, under 18: a low potential (transfer listed: by a future transfer; otherwise one of four wordings by
    /// `variant(4)`); squad player or better (overrated or not, by a future transfer); a great potential (a future
    /// transfer, or one of three by `variant(3)`); a future transfer; otherwise "will develop", after the "showing signs"
    /// sentence when `variant(2)` is 0 (written, then overwritten: the original's).
    unsafe fn long_serving_youth(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                if self.future_transfer {
                    t.position_line(0x99a114); // "<coach> does not believe that <position> <player> is good enough ..."
                } else {
                    t.position_line(0x99a00c); // "<coach> does not believe that <position> <player> is good enough ..."
                }
            } else {
                match self.variant(4) {
                    0 => t.position_line(0x999b48), // "In his report <coach> indicated that he does not believe that ..."
                    1 => t.position_line(0x999c8c), // "<coach> has indicated in his report that young <position> <player> ..."
                    2 => {
                        // "<coach> believes that young <position> <player> has had enough time to prove himself, and that ..."
                        let words = [PositionGender, PositionGender, ClubShortGender, ClubShortGender, CoachName, PositionText, PlayerName,
                            ClubShortName];
                        t.say(0x999de8, &words);
                    }
                    _ => t.position_line(0x999f58), // r7_04_1: "<coach> is of the opinion that <position> <player> lacks the ..."
                }
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER | COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            if self.has_verdict(COACH_VERDICT_OVERRATED) {
                if self.future_transfer {
                    t.position_line(0x999a7c); // "<coach> feels that <position> <player> is a player of great ..."
                } else {
                    // "<coach> feels that <position> <player> is a player of great potential. However he warns that ..."
                    t.say(0x999944, &[PositionGender, PositionGender, CoachName, PositionText, PlayerName, PlayerShortName]);
                }
            } else if self.future_transfer {
                t.position_line(0x999870); // "<coach> feels that <position> <player> is a player with great ..."
            } else {
                t.position_line(0x9997b4); // "<coach> feels that <position> <player> is already showing his ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            if self.future_transfer {
                t.position_line(0x9996c8); // "<coach> reckons young <position> <player> could have turned out to ..."
            } else {
                match self.variant(3) {
                    0 => t.position_line(0x999460), // "<coach> reckons young <position> <player> could turn out to be a ..."
                    1 => t.position_line(0x999520), // "<coach> is a great admirer of <position> <player> who he feels ..."
                    _ => t.position_line(0x9995e8), // "In his report, <coach> indicates that <position> <player> is a ..."
                }
            }
        } else if self.future_transfer {
            t.position_line(0x999368); // "In his report, <coach> indicates that <position> <player> was ..."
        } else {
            if self.variant(2) == 0 {
                // written and overwritten at once by the next sentence (the original's)
                t.position_line(0x9992a0); // "In his report, <coach> indicates that <position> <player> is ..."
            }
            // "<coach>'s report states that <position> <player> will develop into a quality player given time."
            t.say(0x9991e8, &[Gender100, PositionGender, PositionGender, CoachName, PositionText, PlayerName]);
        }
    }

    /// Long serving, under 24: too soon to judge; good enough or a star; a low potential (future transfer, transfer
    /// listed, otherwise naming him short too); some potential; overrated; a future transfer; a good or great potential;
    /// otherwise "showing signs".
    unsafe fn long_serving_young(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            t.position_line(0x99911c); // "<coach> has indicated that he requires more working with ..."
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.position_line(0x999038); // "<coach> feels that <position> <player> has firmly established his ..."
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                t.position_line(0x998f18); // "<coach> does not believe that <position> <player> is developing ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.position_line(0x998df8); // "<coach> does not believe that <position> <player> is developing ..."
            } else {
                // "<coach> does not believe that <position> <player> is developing into the player we had hoped ..."
                t.say(0x998cac, &[PositionGender, PositionGender, CoachName, PositionText, PlayerName, PlayerShortName]);
            }
        } else if self.has_potential(COACH_POTENTIAL_SOME) {
            if self.future_transfer {
                t.position_line(0x998b98); // "<coach> does not believe that <position> <player> would develop ..."
            } else if self.has_verdict(COACH_VERDICT_FIRST_TEAM | COACH_VERDICT_OVERRATED) {
                t.position_line(0x998a90); // "<coach> feels that <position> <player> doesn't deserve his place ..."
            } else {
                t.position_line(0x9989c4); // "<coach> does not believe that <position> <player> will develop ..."
            }
        } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
            if self.future_transfer {
                t.position_line(0x998b98); // "<coach> does not believe that <position> <player> would develop ..."
            } else {
                t.position_line(0x9988b4); // "<coach> feels that <position> <player> has been put into the first ..."
            }
        } else if self.future_transfer {
            t.position_line(0x9987a4); // "<coach> feels that <position> <player> is one to watch in the ..."
        } else if self.has_potential(COACH_POTENTIAL_GOOD) {
            t.position_line(0x9986c4); // "<coach> feels that <position> <player> is one to watch in the ..."
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            t.position_line(0x9985dc); // "<coach> feels that <position> <player> is an outstanding prospect ..."
        } else {
            t.position_line(0x9992a0); // "In his report, <coach> indicates that <position> <player> is ..."
        }
    }

    /// Long serving veteran: too soon; not good enough (future transfer, not a player, otherwise); a squad player
    /// (future transfer, "showing his age" when the potential is too soon to judge or he is overrated, transfer
    /// listed); good enough or a star; a low potential; otherwise "a useful member".
    unsafe fn long_serving_veteran(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            // "<coach> reckons that he needs more time to assess <player>'s qualities."
            t.say(0x998558, &[Gender100, CoachName, PlayerName]);
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x998490); // "<coach> feels that <player> is increasingly looking like a bit ..."
            } else if self.has_verdict(COACH_VERDICT_NOT_A_PLAYER) {
                // "Despite being part of the side for a long time, <coach> believes that <player> is no longer a ..."
                t.say(0x998368, &[Gender100, Gender100, Gender100, CoachName, PlayerName, PlayerShortName]);
            } else {
                // "Despite being part of the side for a long time, <coach> believes that <player> is no longer a ..."
                t.say(0x9981fc, &[Gender100, Gender100, Gender100, CoachName, PlayerName, PlayerShortName]);
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.future_transfer {
                t.names_line(0x998490); // "<coach> feels that <player> is increasingly looking like a bit ..."
            } else if self.has_potential(COACH_POTENTIAL_TOO_SOON) {
                t.names_line(0x997f88); // "<coach> feels that <player> is starting to show his age. It may be ..."
            } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.names_line(0x997f88); // "<coach> feels that <player> is starting to show his age. It may be ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x998120); // "<coach> feels that <player> is increasingly looking like a bit ..."
            } else {
                t.names_line(0x998048); // "<coach> feels that <player> is increasingly looking like a bit ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            if self.future_transfer {
                t.names_line(0x997ec8); // "<coach> feels that <player> is still an essential member of the ..."
            } else {
                t.names_line(0x997dd0); // "<coach> feels that <player> is a essential member of the first ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                t.names_line(0x997d08); // "<coach> feels that <player> is still an essential member of the ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x997c10); // "<coach> feels that veteran <player> is a essential member of the ..."
            } else {
                t.names_line(0x997b00); // "<coach> feels that the experienced <player> is a essential member ..."
            }
        } else if self.future_transfer {
            t.names_line(0x997a3c); // "<coach> feels that <player> is still an useful member of the first ..."
        } else {
            t.names_line(0x997990); // "In his report, <coach> indicates that <player> is a useful member ..."
        }
    }

    /// Long serving and established: not good enough (future transfer, transfer listed); a star; good enough; a
    /// squad player ("doesn't think", written first under a future transfer and then overwritten: the original's;
    /// then overrated or not); otherwise "adds" by a future transfer.
    unsafe fn long_serving_established(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x997898); // "<coach> feels that <player> is an average player who does not add ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x99779c); // "<coach> feels that <player> is an average player who does not add ..."
            } else {
                t.names_line(0x9976ac); // "<coach> feels that <player> isn't of a good enough standard to ..."
            }
        } else if self.has_verdict(COACH_VERDICT_STAR) {
            if self.future_transfer {
                // "In his report on <player>, <coach> feels that the <position> has his best years ahead of him. ..."
                t.say(0x9975b0, &[Gender100, Gender100, PositionGender, PositionGender, PlayerName, CoachName, PositionText]);
            } else {
                // "In his report on <player>, <coach> feels that the <position> has his best years ahead of him ..."
                t.say(0x9974cc, &[Gender100, Gender100, PositionGender, PositionGender, PlayerName, CoachName, PositionText]);
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.position_line(0x9973d0); // "<coach> has reported that <position> <player> has firmly ..."
            } else {
                t.position_line(0x9972cc); // "<coach> has reported that <position> <player> has firmly ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.future_transfer {
                t.names_line(0x9971b0); // "<coach> indicates in his report that although <player> is an ..."
                if self.has_verdict(COACH_VERDICT_OVERRATED) {
                    t.names_line(0x9970ec); // "<coach> indicates in his report that he believes <player> is ..."
                } else {
                    t.names_line(0x997014); // "<coach> indicates in his report that although <player> is an ..."
                }
            } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.names_line(0x9970ec); // "<coach> indicates in his report that he believes <player> is ..."
            } else {
                t.names_line(0x997014); // "<coach> indicates in his report that although <player> is an ..."
            }
        } else if self.future_transfer {
            t.position_line(0x996ef8); // "In his report <coach> indicates that <position> <player> is an ..."
        } else {
            t.position_line(0x996e28); // "In his report <coach> indicates that <position> <player> is an ..."
        }
    }

    /// Long serving journeyman (or with no stage): as the established player, with a sentence of its own for too soon
    /// and for the rest.
    unsafe fn long_serving_other(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            t.names_line(0x996d88); // "<coach> thinks that <player> has not been around long enough for ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x997898); // "<coach> feels that <player> is an average player who does not add ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x99779c); // "<coach> feels that <player> is an average player who does not add ..."
            } else {
                t.names_line(0x9976ac); // "<coach> feels that <player> isn't of a good enough standard to ..."
            }
        } else if self.has_verdict(COACH_VERDICT_STAR) {
            if self.future_transfer {
                // "In his report on <player>, <coach> feels that the <position> has the best years ahead of him. ..."
                t.say(0x996ca4, &[PositionGender, PositionGender, PlayerName, CoachName, PositionText]);
            } else {
                // "In his report on <player>, <coach> feels that the <position> has the best years ahead of him ..."
                t.say(0x996bc4, &[PositionGender, PositionGender, PlayerName, CoachName, PositionText]);
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.position_line(0x996abc); // "<coach> has reported that <position> <player> has firmly ..."
            } else {
                t.position_line(0x9972cc); // "<coach> has reported that <position> <player> has firmly ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.names_line(0x9969e4); // "<coach> indicates in his report that he believes <player> is an ..."
            } else {
                t.names_line(0x996914); // "<coach> indicates in his report that although <player> is a useful ..."
            }
        } else {
            t.position_line(0x996844); // "In his report <coach> indicates that <position> <player> is a ..."
        }
    }

    /// A player still settling in, by his career stage.
    unsafe fn settling(&self, t: &CoachReportTexts) {
        if self.is_stage(COACH_STAGE_YOUTH) {
            self.settling_youth(t);
        } else if self.is_stage(COACH_STAGE_YOUNG) {
            self.settling_young(t);
        } else if self.is_stage(COACH_STAGE_VETERAN) {
            self.settling_veteran(t);
        } else if self.is_stage(COACH_STAGE_ESTABLISHED) {
            self.settling_established(t);
        } else {
            self.settling_other(t);
        }
    }

    /// Settling, under 18: too soon; good enough or a star (by a future transfer); a low potential (future transfer,
    /// transfer listed); a good potential past 24; a great potential; otherwise "a decent member".
    unsafe fn settling_youth(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            // "Young <position> <player> is a recent signing and <coach> feels that he is not in a position ..."
            t.say(0x996764, &[PositionGender, PositionGender, PositionText, PlayerName, CoachName]);
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            if self.future_transfer {
                t.position_line(0x996690); // "<coach> is sad to see <position> <player> leaving the club, he ..."
            } else {
                t.position_line(0x9965c8); // "<coach> urges you to keep hold of young <position> <player> as he ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x9964d0, &[Gender100, CoachName, PlayerName]);
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x9963dc, &[Gender100, CoachName, PlayerName]);
            } else {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x993950, &[Gender100, CoachName, PlayerName]);
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD) && self.has_potential(COACH_POTENTIAL_PAST_IT) {
            t.position_line(0x99631c); // "<coach> feels that <position> <player> is showing all the ..."
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            if self.future_transfer {
                t.position_line(0x996244); // "<coach> insists that <position> <player> is a player of great ..."
            } else {
                t.position_line(0x99614c); // "<coach> insists that <position> <player> is a player of immense ..."
            }
        } else {
            t.position_line(0x9960b0); // "<coach> reckons <position> <player> is a decent member of our squad."
        }
    }

    /// Settling, under 24: too soon; good enough or a star; some potential (overrated or not); a low potential; a good
    /// or great potential; otherwise "an average member".
    unsafe fn settling_young(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            // "<player> has only recently joined the club and <coach> is unable to file a comprehensive ..."
            t.say(0x995ffc, &[PlayerName, CoachName]);
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            // "<coach> thinks we have a talented youngster in <player>. He considers the <position> to be a ..."
            t.say(0x995f28, &[PositionGender, PositionGender, CoachName, PlayerName, PositionText]);
        } else if self.has_potential(COACH_POTENTIAL_SOME) {
            if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.position_line(0x995e40); // "<coach> feels that <position> <player> is over-rated and will ..."
            } else {
                t.names_line(0x995d80); // "<coach> believes that recent signing <player> will never be able ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x9964d0, &[Gender100, CoachName, PlayerName]);
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x9963dc, &[Gender100, CoachName, PlayerName]);
            } else {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x993950, &[Gender100, CoachName, PlayerName]);
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) {
            t.names_line(0x995cec); // "<coach> feels <player> is showing all the qualities required of a ..."
        } else {
            t.position_line(0x995c50); // "<coach> reckons <position> <player> is an average member of our squad."
        }
    }

    /// Settling veteran: too soon; not good enough; a squad player; good enough or a star; a low potential; a good
    /// or great potential past 24; otherwise "an experienced and useful squad player".
    unsafe fn settling_veteran(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            // "Veteran <player> has only recently joined the club and <coach> is feels it would be unfair to ..."
            t.say(0x995b88, &[PlayerName, CoachName]);
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x995aa0); // "<coach> feels experienced <player> does not have enough quality to ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x9959c4); // "<coach> feels experienced <player> does not have enough quality to ..."
            } else {
                t.names_line(0x995920); // "<coach> feels experienced <player> does not have enough quality to ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.future_transfer {
                t.names_line(0x99582c); // "<coach> believes veteran <player> will struggle to hold down a ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x995738); // "<coach> believes veteran <player> will struggle to hold down a ..."
            } else {
                t.names_line(0x995688); // "<coach> believes veteran <player> will struggle to hold down a ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.names_line(0x9955d4); // "<coach> thinks we have an experienced player in <player> whose ..."
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                t.names_line(0x995504); // "<coach> is of the opinion that <player> is now showing signs of ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x995434); // "<coach> is of the opinion that <player> is now showing signs of ..."
            } else {
                t.names_line(0x995364); // "<coach> is of the opinion that <player> is now showing signs of ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) && self.has_potential(COACH_POTENTIAL_PAST_IT) {
            t.names_line(0x9952d4); // "<coach> feels <player> still has a lot to offer at such a late ..."
        } else {
            t.names_line(0x99524c); // "<coach> reckons veteran <player> is an experienced and useful ..."
        }
    }

    /// Settling and established: a low potential (the club's name in the sentence); a good or great potential
    /// past 24 as a squad player; not good enough; a squad player (transfer listed: two wordings by `variant(3)`);
    /// good enough or a star; otherwise "a useful squad player".
    unsafe fn settling_established(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                // "<coach> is of the opinion that <player> is not good enough to play for <club>. He feels that ..."
                t.say(0x99514c, &[ClubGender, ClubGender, CoachName, PlayerName, ClubName]);
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                // "<coach> is of the opinion that <player> is not good enough to play for <club>. He feels that ..."
                t.say(0x99504c, &[ClubGender, ClubGender, CoachName, PlayerName, ClubName]);
            } else {
                // "<coach> is of the opinion that <player> is not good enough to play for <club>."
                t.say(0x994fa8, &[ClubGender, ClubGender, CoachName, PlayerName, ClubName]);
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT)
            && self.has_potential(COACH_POTENTIAL_PAST_IT)
            && self.has_verdict(COACH_VERDICT_SQUAD_PLAYER)
        {
            t.names_line(0x994f1c); // "<coach> feels <player> still has a lot to offer at this late stage ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x994e44); // "<coach> feels <player> lacks the quality to hold down a regular ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x994d78); // "<coach> feels <player> lacks the quality to hold down a regular ..."
            } else {
                t.names_line(0x994cec); // "<coach> feels <player> lacks the quality to hold down a regular ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.future_transfer {
                t.names_line(0x994c10); // "<coach> does not believe <player> is good enough to hold down a ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                if self.variant(3) != 0 {
                    t.names_line(0x994b34); // "<coach> does not believe <player> is good enough to hold down a ..."
                } else {
                    t.names_line(0x994a4c); // "<coach> feels <player> lacks enough quality to hold down a regular ..."
                }
            } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.names_line(0x9949ac); // "<coach> believes <player> is over-rated and should not be holding ..."
            } else {
                t.names_line(0x994924); // "<coach> believes <player> will struggle to hold a regular place in ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.names_line(0x9948a8); // "<coach> is happy that <player> has a positive effect on the team."
        } else {
            t.names_line(0x993dbc); // "<coach> sees <player> as a useful squad player."
        }
    }

    /// Settling journeyman (or with no stage): too soon; not good enough; a squad player (transfer listed: by
    /// `variant(3)`); good enough or a star; a low potential; a good or great potential (past 24 or not); otherwise
    /// "a useful squad player".
    unsafe fn settling_other(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            // "<player> has only recently joined the club and <coach> is feels it would be unfair to comment ..."
            t.say(0x9947e8, &[PlayerName, CoachName]);
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.future_transfer {
                t.names_line(0x994704); // "<coach> feels <player> does not have enough quality to hold down a ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x99462c); // "<coach> feels <player> does not have enough quality to hold down a ..."
            } else {
                t.names_line(0x994594); // "<coach> feels <player> does not have enough quality to hold down a ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) {
            if self.future_transfer {
                t.names_line(0x9944b8); // "<coach> feels <player> does not have enough quality to hold down a ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                if self.variant(3) != 0 {
                    t.names_line(0x9943d8); // "<coach> feels <player> does not have enough quality to hold down a ..."
                } else {
                    t.names_line(0x9942e8); // "<coach> feels <player> does not have enough quality to hold down a ..."
                }
            } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
                // "<coach> believes <player> is over-rated and will struggle to hold down a place in the side."
                t.say(0x994250, &[Gender100, CoachName, PlayerName]);
            } else {
                t.names_line(0x9941cc); // "<coach> believes <player> will struggle to hold down a place in ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.names_line(0x994150); // "<coach> thinks <player> is having a positive effect on the team."
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.future_transfer {
                t.names_line(0x994064); // "<coach> is of the opinion that <player> is not good enough to be a ..."
            } else if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x993f78); // "<coach> is of the opinion that <player> is not good enough to be a ..."
            } else {
                t.names_line(0x993ee8); // "<coach> is of the opinion that <player> is not good enough to be a ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) && self.has_potential(COACH_POTENTIAL_PAST_IT) {
            t.names_line(0x9952d4); // "<coach> feels <player> still has a lot to offer at such a late ..."
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) {
            t.position_line(0x993e28); // "<coach> believes <position> <player> has all the attributes ..."
        } else {
            t.names_line(0x993dbc); // "<coach> sees <player> as a useful squad player."
        }
    }

    /// A player settled in at the club, by his career stage.
    unsafe fn settled_in(&self, t: &CoachReportTexts) {
        if self.is_stage(COACH_STAGE_YOUTH) {
            self.settled_in_youth(t);
        } else if self.is_stage(COACH_STAGE_YOUNG) {
            self.settled_in_young(t);
        } else if self.is_stage(COACH_STAGE_VETERAN) {
            self.settled_in_veteran(t);
        } else if self.is_stage(COACH_STAGE_JOURNEYMAN | COACH_STAGE_ESTABLISHED) {
            self.settled_in_experienced(t);
        } else {
            self.settled_in_other(t);
        }
    }

    /// Settled in, under 18: too soon; a squad player with a good or great potential; good enough or a star; a low
    /// potential; a good potential or past 24; a great potential; overrated; not good enough; otherwise "a decent
    /// member".
    unsafe fn settled_in_youth(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            t.names_line(0x993d28); // "<coach> feels that <player> has not been at the club long enough ..."
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) && self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) {
            if self.has_potential(COACH_POTENTIAL_GOOD) {
                t.position_line(0x993c80); // "<coach> believes that <position> <player> is showing a great deal ..."
            } else {
                t.names_line(0x993bc0); // "<coach> believes that although <player> has not featured heavily ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.position_line(0x993aec); // "<coach> urges you to keep hold of young <position> <player> as he ..."
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x9939f8); // "<coach> feels young <player> is not showing enough potential to be ..."
            } else {
                // "<coach> feels young <player> is not showing enough potential to be considered a sound long ..."
                t.say(0x993950, &[Gender100, CoachName, PlayerName]);
            }
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_PAST_IT) {
            t.position_line(0x993e28); // "<coach> believes <position> <player> has all the attributes ..."
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            t.position_line(0x993878); // "<coach> insists <position> <player> is a player of immense quality ..."
        } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
            t.names_line(0x9937e0); // "<coach> reckons <player> is over-rated and will never be a ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x9936b8); // "Having been with the club for a reasonable amount of time, <coach> ..."
            } else {
                t.names_line(0x99358c); // "Having been with the club for a reasonable amount of time, <coach> ..."
            }
        } else {
            t.position_line(0x9960b0); // "<coach> reckons <position> <player> is a decent member of our squad."
        }
    }

    /// Settled in, under 24: too soon; a squad player with a good, great or promising potential; good enough or a
    /// star; a low potential; promising; a good potential or past 24; great; not good enough; overrated; versatile;
    /// otherwise "a useful member".
    unsafe fn settled_in_young(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            t.names_line(0x9934e0); // "At this present time, <coach> feels that <player> has not been at ..."
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER)
            && self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT | COACH_POTENTIAL_PROMISING)
        {
            if self.has_verdict(COACH_VERDICT_OVERRATED) {
                // "<coach> is a keen admirer of <player> whom he regards as one to watch for the future. However ..."
                t.say(0x9933c4, &[PositionGender, PositionGender, CoachName, PlayerName, PositionText]);
            } else {
                // The original tests (potential & GOOD) | 0x100, never zero: its "player of huge potential" sentence
                // (0x9931b8) is never reached (DEFEITOS_DO_ORIGINAL).
                // "<coach> believes that <player> has shown a great deal of potential. He recommends that the ..."
                t.say(0x9932b4, &[PositionGender, PositionGender, CoachName, PlayerName, PositionText]);
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            // "<coach> is a keen admirer of <player>'s abilities. He sees the <position> as a key member of ..."
            t.say(0x9930f0, &[Gender100, PositionGender, PositionGender, CoachName, PlayerName, PositionText]);
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.position_line(0x992ff0); // "<coach> does not feel that young <position> <player> is good ..."
            } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
                t.position_line(0x992f24); // "<coach> feels you are over-rating young <position> <player> and ..."
            } else {
                t.position_line(0x992e70); // "<coach> does not believe that young <position> <player> is good ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_PROMISING) {
            t.position_line(0x992d84); // "<coach> feels that with a combination of hard-work and good luck, ..."
        } else if self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_PAST_IT) {
            t.position_line(0x992cd8); // "<coach> feels <position> <player> is capable of becoming a class ..."
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            t.position_line(0x992c08); // "<coach> wants you to keep a firm eye on <position> <player> who is ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x992afc); // "<coach> is concerned that young <player> has not progressed in the ..."
            } else {
                t.names_line(0x992a00); // "<coach> is concerned that young <player> has not progressed in the ..."
            }
        } else if self.has_verdict(COACH_VERDICT_OVERRATED) {
            t.names_line(0x992964); // "<coach> thinks that <player> is over-rated and will never be a ..."
        } else if self.has_potential(COACH_POTENTIAL_VERSATILE) {
            t.names_line(0x991a0c); // "<coach> reckons <player> is a versatile member of our squad."
        } else {
            t.names_line(0x9928f0); // "<coach> reckons <player> is a useful member of our squad."
        }
    }

    /// Settled-in veteran: too soon; not good enough (transfer listed, not a player); a squad player with a good or
    /// great potential (too active in the first team or not); good enough or a star; otherwise "a useful member".
    unsafe fn settled_in_veteran(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) {
            t.names_line(0x99283c); // "Presently, <coach> feels that veteran <player> has not been with ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x992774); // "In his report, <coach> feels that <player> is no longer of any ..."
            } else if self.has_verdict(COACH_VERDICT_NOT_A_PLAYER) {
                // "In his report, <coach> feels that <player> is no longer of any use as a player. He feels that ..."
                t.say(0x992678, &[CoachName, PlayerName, PlayerShortName]);
            } else {
                t.names_line(0x9925d4); // "In his report, <coach> feels that <player> is no longer of any use ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) && self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) {
            if self.has_verdict(COACH_VERDICT_FIRST_TEAM) {
                if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                    t.names_line(0x9924d0); // "<coach> believes that ageing <player> has seen better days and ..."
                } else {
                    t.names_line(0x99242c); // "<coach> believes that ageing <player> has seen better days and ..."
                }
            } else {
                t.names_line(0x992388); // "<coach> believes that ageing <player> has a lot more to offer the ..."
            }
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH | COACH_VERDICT_STAR) {
            t.names_line(0x9922f4); // "<coach> is a big fan of veteran <player> who despite his age still ..."
        } else {
            t.names_line(0x992254); // "<coach> reckons <player> is a useful member of our squad, whose ..."
        }
    }

    /// Settled-in journeyman or established player: too soon, unless established (then judged as the rest); not
    /// good enough; a squad player with a good or great potential; a star; good enough (versatile or not); a low
    /// potential; a great potential; versatile; otherwise "a useful member".
    unsafe fn settled_in_experienced(&self, t: &CoachReportTexts) {
        use CoachWord::*;
        if self.has_verdict(COACH_VERDICT_TOO_SOON) && !self.is_stage(COACH_STAGE_ESTABLISHED) {
            t.names_line(0x9921c8); // "<coach> is unable to file a report on <player> until he been with ..."
        } else if self.has_verdict(COACH_VERDICT_NOT_GOOD_ENOUGH) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x9920ac); // "<coach> is concerned that <player> has been a part of the club for ..."
            } else {
                t.names_line(0x991fac); // "<coach> is concerned that <player> has been a part of the club for ..."
            }
        } else if self.has_verdict(COACH_VERDICT_SQUAD_PLAYER) && self.has_potential(COACH_POTENTIAL_GOOD | COACH_POTENTIAL_GREAT) {
            t.position_line(0x991ef4); // "<coach> believes that <position> <player> is capable of fighting ..."
        } else if self.has_verdict(COACH_VERDICT_STAR) {
            // "<coach> is a keen admirer of <player>. He is confident that the <position>'s presence in the ..."
            t.say(0x991e14, &[PositionGender, PositionGender, CoachName, PlayerName, PositionText]);
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH) && self.has_potential(COACH_POTENTIAL_VERSATILE) {
            // "<coach> believes that <position> <player>'s versatility is an invaluable asset for the club."
            t.say(0x991d60, &[PositionGender, PositionGender, Gender100, CoachName, PositionText, PlayerName]);
        } else if self.has_verdict(COACH_VERDICT_GOOD_ENOUGH) {
            t.position_line(0x991cb4); // "<coach> believes that <position> <player> is playing an important ..."
        } else if self.has_potential(COACH_POTENTIAL_LOW) {
            if self.has_verdict(COACH_VERDICT_TRANSFER_LISTED) {
                t.names_line(0x991be4); // "<coach> feels <player> has developed as far as possible and can ..."
            } else {
                t.names_line(0x991b5c); // "<coach> feels <player> has developed as far as possible and can ..."
            }
        } else if self.has_potential(COACH_POTENTIAL_GREAT) {
            t.position_line(0x991a84); // "<coach> reckons <position> <player> must start to fulfil his ..."
        } else if self.has_potential(COACH_POTENTIAL_VERSATILE) {
            t.names_line(0x991a0c); // "<coach> reckons <player> is a versatile member of our squad."
        } else {
            t.names_line(0x9928f0); // "<coach> reckons <player> is a useful member of our squad."
        }
    }

    /// Settled in with no career stage (no player sheet): no sentence at all, so the text buffer keeps what the
    /// previous text left in it (DEFEITOS_DO_ORIGINAL).
    unsafe fn settled_in_other(&self, _t: &CoachReportTexts) {
        // no sentence: the text buffer keeps what it held
    }
}

/// Comparer for `write_coach_report_news_text` (`compare_text_writer`: the text, the links and the generator).
pub(crate) unsafe extern "thiscall" fn compare_write_coach_report_news_text(this: usize, news: usize, body: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%);
    let mut ro = 0;
    compare_text_writer(out, || ro = o(this, news, body, out), |t| {
        write_coach_report_news_text(this, news, body, t);
    }, || format!("write_coach_report_news_text(body {}, player {})", body, exe::newsitem_get_slot(news, 4) as i32));
    ro
}
