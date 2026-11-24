// ---------- batch %%LEVA%%: the news text on how a club's players did for their nation (national_teams.cpp) ----------

/// One argument of a club-players-international template, in the order the original pushes it. The genders and the
/// fixture label's gender are pushed twice (sign-extended), as every `{}` pair of the template reads them.
#[derive(Clone, Copy)]
enum IntlArg {
    /// the club's `gender_name` (+0x37), twice
    ClubGender,
    /// the national side's `short_gender_name` (+0x52), twice
    NationGender,
    /// the opponent side's `short_gender_name`, twice
    OpponentGender,
    /// the fixture label's gender (`exe::comp_util_get_label_gender`), twice
    FixtureGender,
    /// how many players the club had, minus one (once, a `%d`)
    Others,
    /// the club's name (+4)
    ClubName,
    /// the subject's name, mode 1: the first player (no highlight) or the highlighted player
    Subject,
    /// the first player's name, mode 5
    PlayerShort,
    /// the highlighted player's name, mode 5
    HighlightShort,
    /// the list of the club's players ("A and B", "A, B and C"...)
    PlayerList,
    /// the national side's name
    Nation,
    /// the opponent side's name
    Opponent,
    /// the score line, highest first ("2-1")
    Score,
    /// the fixture label ("friendly" lowercased)
    Fixture,
}

// The argument lists of the templates (the subject is the player or the highlighted player).
const INTL_FORM_FULL: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_FULL_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent, IntlArg::PlayerShort];
const INTL_FORM_FULL_HIGHLIGHT_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent, IntlArg::HighlightShort];
const INTL_FORM_NO_FIXTURE: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent];
const INTL_FORM_NO_FIXTURE_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent, IntlArg::PlayerShort];
const INTL_FORM_NO_FIXTURE_SCORE_LAST: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score];
const INTL_FORM_NO_FIXTURE_SCORE_LAST_PLAYER_SHORT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score, IntlArg::PlayerShort];
const INTL_FORM_NO_CLUB: &[IntlArg] = &[IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Opponent, IntlArg::Score];
const INTL_FORM_NO_CLUB_SCORE_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Opponent];
const INTL_FORM_NATION_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Nation, IntlArg::Subject, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_SUBJECT_FIRST: &[IntlArg] = &[IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Subject, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_LIST: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::ClubName, IntlArg::PlayerList, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];
const INTL_FORM_COUNT: &[IntlArg] = &[IntlArg::ClubGender, IntlArg::NationGender, IntlArg::FixtureGender, IntlArg::OpponentGender, IntlArg::Others, IntlArg::ClubName, IntlArg::Nation, IntlArg::Score, IntlArg::Fixture, IntlArg::Opponent];

/// A template and its arguments.
type IntlLine = (usize, &'static [IntlArg]);

/// One rule of a template tree: when `code & mask == mask` (the first that matches), the template for a draw of 0
/// (`zero`) or any other draw (`other`).
struct IntlRule {
    mask: u8,
    zero: IntlLine,
    other: IntlLine,
}

const fn intl_rule(mask: u8, zero: IntlLine, other: IntlLine) -> IntlRule {
    IntlRule { mask, zero, other }
}

/// A template tree: the rules by the player's performance code, and the lines when none matches (or the news item
/// carries no codes), for a draw of 0 and another draw.
struct IntlTree {
    rules: &'static [IntlRule],
    fallback: (IntlLine, IntlLine),
}

/// The templates of a lone player (no highlighted player): won, drawn, lost.
const INTL_PLAYER_TREES: [IntlTree; 3] = {
    [
        IntlTree {
            rules: &[
                intl_rule(0x44, (0xa5f638, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f638, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x14, (0xa5f504, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f504, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa5f410, INTL_FORM_FULL), (0xa5f410, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa5f2c8, INTL_FORM_FULL_PLAYER_SHORT), (0xa5f2c8, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5f1f0, INTL_FORM_NATION_FIRST), (0xa5f104, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5f03c, INTL_FORM_NATION_FIRST), (0xa5ef58, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa5eea4, INTL_FORM_NO_CLUB), (0xa5ed70, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x10, (0xa5ecc8, INTL_FORM_NO_CLUB), (0xa5eb8c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x02, (0xa5eae8, INTL_FORM_NO_CLUB), (0xa5ea34, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x04, (0xa5e998, INTL_FORM_NO_CLUB), (0xa5e8dc, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x01, (0xa5e814, INTL_FORM_SUBJECT_FIRST), (0xa5e814, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5e72c, INTL_FORM_FULL), (0xa5e678, INTL_FORM_NO_FIXTURE_SCORE_LAST)), // sx15_3: a lone player in a win
        },
        IntlTree {
            rules: &[
                intl_rule(0x14, (0xa5e540, INTL_FORM_FULL_PLAYER_SHORT), (0xa5e540, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa5e44c, INTL_FORM_FULL), (0xa5e44c, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa5e2f0, INTL_FORM_FULL_PLAYER_SHORT), (0xa5e2f0, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5e218, INTL_FORM_NATION_FIRST), (0xa5e12c, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5e064, INTL_FORM_NATION_FIRST), (0xa5df80, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa5dea0, INTL_FORM_SUBJECT_FIRST), (0xa5dd68, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x10, (0xa5dcc0, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5db80, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x02, (0xa5dadc, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5da10, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa5d974, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5d87c, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa5d7b0, INTL_FORM_SUBJECT_FIRST), (0xa5d7b0, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5d6c8, INTL_FORM_FULL), (0xa5d5e4, INTL_FORM_FULL)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x24, (0xa5d494, INTL_FORM_FULL_PLAYER_SHORT), (0xa5d494, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa5d3b4, INTL_FORM_NATION_FIRST), (0xa5d2b8, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa5d1f0, INTL_FORM_NATION_FIRST), (0xa5d0fc, INTL_FORM_FULL)),
                intl_rule(0x02, (0xa5d058, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa5cf9c, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa5cec8, INTL_FORM_SUBJECT_FIRST), (0xa5cdc4, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa5ccf8, INTL_FORM_SUBJECT_FIRST), (0xa5ccf8, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5cc18, INTL_FORM_FULL), (0xa5cb4c, INTL_FORM_NO_FIXTURE)),
        },
    ]
};

/// The templates of a highlighted player (named captain or man of the match): won, drawn, lost.
const INTL_HIGHLIGHT_TREES: [IntlTree; 3] = {
    [
        IntlTree {
            rules: &[
                intl_rule(0x44, (0xa62d4c, INTL_FORM_FULL_PLAYER_SHORT), (0xa62d4c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x14, (0xa62c08, INTL_FORM_FULL_PLAYER_SHORT), (0xa62c08, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa62afc, INTL_FORM_FULL), (0xa62afc, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa629a4, INTL_FORM_FULL_PLAYER_SHORT), (0xa629a4, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa628c4, INTL_FORM_NATION_FIRST), (0xa627c0, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa626f0, INTL_FORM_NATION_FIRST), (0xa625f4, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa62534, INTL_FORM_NO_CLUB), (0xa623e4, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x10, (0xa62334, INTL_FORM_NO_CLUB), (0xa621f0, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x02, (0xa62144, INTL_FORM_NO_CLUB), (0xa62074, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
                intl_rule(0x04, (0xa61fd0, INTL_FORM_NO_CLUB), (0xa61ed0, INTL_FORM_NO_FIXTURE_SCORE_LAST_PLAYER_SHORT)),
                intl_rule(0x01, (0xa61e00, INTL_FORM_SUBJECT_FIRST), (0xa61e00, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa61d10, INTL_FORM_FULL), (0xa61c4c, INTL_FORM_NO_FIXTURE_SCORE_LAST)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x14, (0xa61b04, INTL_FORM_FULL_PLAYER_SHORT), (0xa61b04, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x40, (0xa619f8, INTL_FORM_FULL), (0xa619f8, INTL_FORM_FULL)),
                intl_rule(0x24, (0xa6189c, INTL_FORM_FULL_PLAYER_SHORT), (0xa6189c, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa617bc, INTL_FORM_NATION_FIRST), (0xa616b4, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa615e4, INTL_FORM_NATION_FIRST), (0xa614e4, INTL_FORM_FULL)),
                intl_rule(0x12, (0xa613f8, INTL_FORM_SUBJECT_FIRST), (0xa612a4, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x10, (0xa611cc, INTL_FORM_SUBJECT_FIRST), (0xa61084, INTL_FORM_FULL_HIGHLIGHT_SHORT)),
                intl_rule(0x02, (0xa60fb0, INTL_FORM_SUBJECT_FIRST), (0xa60eb4, INTL_FORM_FULL)),
                intl_rule(0x04, (0xa60de8, INTL_FORM_SUBJECT_FIRST), (0xa60cbc, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x01, (0xa60be8, INTL_FORM_SUBJECT_FIRST), (0xa60be8, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa60af8, INTL_FORM_FULL), (0xa60a04, INTL_FORM_FULL)),
        },
        IntlTree {
            rules: &[
                intl_rule(0x24, (0xa608a8, INTL_FORM_FULL_PLAYER_SHORT), (0xa608a8, INTL_FORM_FULL_PLAYER_SHORT)),
                intl_rule(0x20, (0xa607c4, INTL_FORM_NATION_FIRST), (0xa606b0, INTL_FORM_FULL)),
                intl_rule(0x08, (0xa605e0, INTL_FORM_NATION_FIRST), (0xa604d4, INTL_FORM_FULL)),
                intl_rule(0x02, (0xa60428, INTL_FORM_NO_CLUB_SCORE_FIRST), (0xa60350, INTL_FORM_NO_FIXTURE)),
                intl_rule(0x04, (0xa60274, INTL_FORM_SUBJECT_FIRST), (0xa60168, INTL_FORM_NO_FIXTURE_PLAYER_SHORT)),
                intl_rule(0x01, (0xa60094, INTL_FORM_SUBJECT_FIRST), (0xa60094, INTL_FORM_SUBJECT_FIRST)),
            ],
            fallback: ((0xa5ffa4, INTL_FORM_FULL), (0xa5fea4, INTL_FORM_FULL)),
        },
    ]
};

/// The player list templates of the lone-player text (two, three, four players) and of the highlighted text (the two
/// or three players after the highlighted one).
const INTL_LIST_TWO: usize = 0xa62f1c;
const INTL_LIST_THREE: usize = 0xa62ea8;
const INTL_LIST_FOUR: usize = 0xa5fce0;

/// "%d other players" from the club (more than four), the "and X" sentence (plain, or after a win when the second
/// player scored or stood out), the list sentence, the score line and the two headlines.
const INTL_OTHERS: usize = 0xa5fdc0;
const INTL_AND_PLAYER: usize = 0xa5fe24;
const INTL_AND_PLAYER_WIN: usize = 0xa5fe64;
const INTL_AND_LIST: usize = 0xa5fd70;
const INTL_SCORE_LINE: usize = 0x9c116c;
const INTL_HEADLINE_PLAYER: usize = 0xa5ca9c;
const INTL_HEADLINE_CLUB: usize = 0xa5caf0;
/// "Friendly", lowercased at the start when it is the fixture label.
const INTL_FRIENDLY: usize = 0x9db958;

/// The slot that ends the players in the news item (`send_club_international_report_news`).
const INTL_CODES_MARKER: u32 = 0x420e9;

/// The slots of the club international report news (0x24), as the body reads them: the match record, the highlighted
/// player (id and the three name ids; id −1 when he is not from the club), how many of the club's players there were,
/// the club, up to four players (only when there were at most four), the marker, the draw and the players' codes (only
/// with the marker).
struct IntlReport {
    record: [u8; 0x20],
    highlight: [u32; 4],
    count: i8,
    club: u32,
    players: [[u32; 4]; 4],
    /// the slot after the players is not the marker (more than four players: the players are not read, so the marker
    /// is not where the original looks)
    no_codes: bool,
    draw: u32,
    codes: [u32; 4],
}

impl IntlReport {
    /// How many players' slots the loops read: none above four, otherwise the count (at most four).
    fn players_read(&self) -> usize {
        if self.count > 4 { 0 } else { self.count.clamp(0, 4) as usize }
    }

    unsafe fn read(news: usize) -> IntlReport {
        let mut r = IntlReport { record: [0; 0x20], highlight: [0; 4], count: 0, club: 0, players: [[0; 4]; 4], no_codes: false, draw: 0, codes: [0; 4] };
        exe::date_date_54a110(r.record.as_mut_ptr() as usize);
        let mut k: u8 = 0;
        exe::read_match_record_from_news(news, &mut k, r.record.as_mut_ptr());
        let mut next = || {
            let v = exe::newsitem_get_slot(news, k as u32);
            k = k.wrapping_add(1);
            v
        };
        for v in r.highlight.iter_mut() {
            *v = next();
        }
        r.count = next() as i8;
        r.club = next();
        for i in 0..r.players_read() {
            for v in r.players[i].iter_mut() {
                *v = next();
            }
        }
        r.no_codes = next() != INTL_CODES_MARKER;
        if !r.no_codes {
            r.draw = next();
            for i in 0..r.players_read() {
                r.codes[i] = next();
            }
        }
        r
    }

    /// The highlighted text with more than four players: the original picks its template by a draw it never read (a
    /// stack dword left by earlier calls); the copy uses 0.
    fn draw_uninitialized(&self) -> bool {
        self.highlight[0] as i32 != -1 && self.no_codes
    }

    fn record(&self) -> RecordMatch {
        RecordMatch(self.record.as_ptr() as usize)
    }

    unsafe fn sides(&self) -> (Club, Club) {
        intl_sides(self.record())
    }

    /// 0 won, 1 drawn, 2 lost, by the national side's goals.
    unsafe fn result(&self) -> usize {
        let (ours, theirs) = self.record().goals();
        if ours > theirs { 0 } else if ours == theirs { 1 } else { 2 }
    }

    unsafe fn b_international(&self) -> bool {
        intl_b_international(self.record())
    }
}

/// The national side (+0x11 of the record) and its opponent.
unsafe fn intl_sides(record: RecordMatch) -> (Club, Club) {
    (club_at(read_i32(record.0 + 0x11) as isize), record.opponent())
}

/// Whether the match was in the B internationals (the sides' links are 0x7d6 instead of 0x7d5, and the headline names
/// the B side).
unsafe fn intl_b_international(record: RecordMatch) -> bool {
    record.comp_index() as u32 == read_u32(KEY_CLUB_COMP_B_INTERNATIONAL)
}

/// The buffers of the body text.
struct IntlTexts {
    player: [[u8; 0x78]; 4],
    player_short: [[u8; 0x78]; 4],
    highlight: [u8; 0x12c],
    highlight_short: [u8; 0x12c],
    nation: [u8; 0x12c],
    opponent: [u8; 0x12c],
    score: [u8; 0xc],
    fixture: [u8; 0xc8],
    fixture_gender: i8,
    list: [u8; 0x3e8],
}

/// The national side's name for the text: the nation's main side (`exe::database_get_national_team`), short form.
unsafe fn intl_side_name(side: Club, buf: &mut [u8; 0x12c]) {
    let team = exe::database_get_national_team(side.nation());
    exe::club_get_national_team_name(team, 0usize, buf.as_mut_ptr() as usize, 0x12c, 1, 0);
}

/// Writes one template into `NEWS_TEXT_BUFFER` (or `out` for the list) with its arguments.
unsafe fn intl_format(out: usize, template: usize, form: &[IntlArg], subject: usize, r: &IntlReport, t: &IntlTexts) {
    let club = club_at(r.club as i32 as isize);
    let (nation, opponent) = r.sides();
    let mut v = [0usize; 16];
    let mut n = 0;
    let mut push = |x: usize| {
        v[n] = x;
        n += 1;
    };
    for a in form {
        match a {
            IntlArg::ClubGender => {
                push(club.gender_name() as i32 as usize);
                push(club.gender_name() as i32 as usize);
            }
            IntlArg::NationGender => {
                push(nation.short_gender_name() as i8 as i32 as usize);
                push(nation.short_gender_name() as i8 as i32 as usize);
            }
            IntlArg::OpponentGender => {
                push(opponent.short_gender_name() as i8 as i32 as usize);
                push(opponent.short_gender_name() as i8 as i32 as usize);
            }
            IntlArg::FixtureGender => {
                push(t.fixture_gender as i32 as usize);
                push(t.fixture_gender as i32 as usize);
            }
            IntlArg::Others => push((r.count as i32 - 1) as usize),
            IntlArg::ClubName => push(club.name_addr()),
            IntlArg::Subject => push(subject),
            IntlArg::PlayerShort => push(t.player_short[0].as_ptr() as usize),
            IntlArg::HighlightShort => push(t.highlight_short.as_ptr() as usize),
            IntlArg::PlayerList => push(t.list.as_ptr() as usize),
            IntlArg::Nation => push(t.nation.as_ptr() as usize),
            IntlArg::Opponent => push(t.opponent.as_ptr() as usize),
            IntlArg::Score => push(t.score.as_ptr() as usize),
            IntlArg::Fixture => push(t.fixture.as_ptr() as usize),
        }
    }
    news_text_format()(out, template, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
}

/// The template a tree picks for a performance code and a draw (the first rule whose mask the code holds; without
/// codes, the fallback).
fn intl_pick(tree: &IntlTree, code: u8, no_codes: bool, draw: u32) -> IntlLine {
    let by_draw = |zero: IntlLine, other: IntlLine| if draw == 0 { zero } else { other };
    if !no_codes {
        if let Some(rule) = tree.rules.iter().find(|rule| code & rule.mask == rule.mask) {
            return by_draw(rule.zero, rule.other);
        }
    }
    by_draw(tree.fallback.0, tree.fallback.1)
}

/// The headline: one player ("X selected for England match", the highlighted player's or the lone player's name, mode
/// 5, with his link when the names still match) or the club ("Leeds players selected in England match", with the
/// club's link); then the national side's link (0x7d6 in the B internationals, else 0x7d5).
unsafe fn intl_headline(news: usize, out: usize) {
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let mut k: u8 = 0;
    exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as u32);
        k = k.wrapping_add(1);
        v
    };
    let mut player: [u32; 4] = [next(), next(), next(), next()];
    let mut count = next() as i8;
    let club = club_at(next() as i32 as isize);
    if player[0] as i32 != -1 {
        count = count.wrapping_add(1); // the highlighted player counts again
    }
    let b_side = intl_b_international(RecordMatch(record.as_ptr() as usize));
    let nation = intl_sides(RecordMatch(record.as_ptr() as usize)).0;
    let link = if b_side { 0x7d6 } else { 0x7d5 };
    let mut nation_name = [0u8; 0x12c];
    if count == 1 || (player[0] as i32 > -1 && count == 2) {
        covered!("sx15_3 headline player");
        if player[0] as i32 == -1 {
            player = [next(), next(), next(), next()];
        }
        let mut name = [0u8; 0x12c];
        exe::database_format_name_from_ids(player[1], player[2], player[3], name.as_mut_ptr(), 0x12c, 5, player[0]);
        exe::club_get_national_team_name(nation, b_side as usize, nation_name.as_mut_ptr() as usize, 0x12c, 1, 1);
        let g = nation.short_gender_name() as i8 as i32;
        news_text_format()(NEWS_TEXT_BUFFER, INTL_HEADLINE_PLAYER, g, g, name.as_ptr() as usize, nation_name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        let p = staff_at(player[0] as i32 as isize);
        if exe::database_staff_matches_names(p, player[1], player[2], player[3], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, p);
        }
    } else {
        covered!("sx15_3 headline club");
        exe::club_get_national_team_name(nation, b_side as usize, nation_name.as_mut_ptr() as usize, 0x12c, 1, 1);
        let (gc, gn) = (club.short_gender_name() as i8 as i32, nation.short_gender_name() as i8 as i32);
        news_text_format()(NEWS_TEXT_BUFFER, INTL_HEADLINE_CLUB, gc, gc, gn, gn, club.addr() + club::SHORT_NAME, nation_name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    }
    exe::text_add_link(out, nation_name.as_ptr(), link, nation);
}

/// The body's shared parts: the players' names (modes 1 and 5), both sides' names, the fixture label and its gender
/// ("Friendly" lowercased) and the score line, highest first.
unsafe fn intl_body_texts(r: &IntlReport) -> IntlTexts {
    let mut t = IntlTexts {
        player: [[0; 0x78]; 4],
        player_short: [[0; 0x78]; 4],
        highlight: [0; 0x12c],
        highlight_short: [0; 0x12c],
        nation: [0; 0x12c],
        opponent: [0; 0x12c],
        score: [0; 0xc],
        fixture: [0; 0xc8],
        fixture_gender: 0,
        list: [0; 0x3e8],
    };
    for i in 0..r.players_read() {
        let [id, first, second, common] = r.players[i];
        exe::database_format_name_from_ids(first, second, common, t.player[i].as_mut_ptr(), 0x78, 1, id);
        exe::database_format_name_from_ids(first, second, common, t.player_short[i].as_mut_ptr(), 0x78, 5, id);
    }
    let (nation, opponent) = r.sides();
    intl_side_name(nation, &mut t.nation);
    intl_side_name(opponent, &mut t.opponent);
    // the round and stage words go as the dwords at +0xc and +0xe, as the original pushes them
    let (comp, round, stage) = (r.record().comp(), read_u32(r.record().0 + 0xc), read_u32(r.record().0 + 0xe));
    exe::comp_util_format_comp_stage_round_label(comp, round, stage, 1, t.fixture.as_mut_ptr(), 0xc8);
    t.fixture_gender = exe::comp_util_get_label_gender(comp as u32, round, stage, 1) as i8;
    if news_streq(t.fixture.as_ptr() as usize, INTL_FRIENDLY) {
        t.fixture[0] = b'f';
    }
    let (ours, theirs) = r.record().goals();
    let (high, low) = if ours < theirs { (theirs, ours) } else { (ours, theirs) };
    news_text_format()(t.score.as_mut_ptr() as usize, INTL_SCORE_LINE, high as i32, low as i32);
    t
}

/// The body without a highlighted player: two to four players make a list first; one player (or two when the first
/// has a code) gets the lone-player tree; three or four (or two without a code) the list sentence by the result; more
/// than four the count sentence by the result. Two players then add "and X" (the win form when the second player's
/// code holds 0x7a and the side won).
unsafe fn intl_body_players(r: &IntlReport, t: &mut IntlTexts, out: usize) {
    let player: [usize; 4] = core::array::from_fn(|i| t.player[i].as_ptr() as usize);
    let p = |i: usize| player[i];
    let list = t.list.as_mut_ptr() as usize;
    match r.count {
        2 => { news_text_format()(list, INTL_LIST_TWO, p(0), p(1)); }
        3 => { news_text_format()(list, INTL_LIST_THREE, p(0), p(1), p(2)); }
        4 => { news_text_format()(list, INTL_LIST_FOUR, p(0), p(1), p(2), p(3)); }
        _ => {}
    }
    let result = r.result();
    let single = r.count == 1 || (r.count == 2 && r.codes[0] != 0);
    if single {
        covered!("sx15_3 lone player");
        let (template, form) = intl_pick(&INTL_PLAYER_TREES[result], r.codes[0] as u8, r.no_codes, r.draw);
        intl_format(NEWS_TEXT_BUFFER, template, form, p(0), r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if r.count == 2 {
            let (ours, theirs) = r.record().goals();
            let template = if r.codes[1] as u8 & 0x7a == 0 || ours <= theirs { INTL_AND_PLAYER } else { INTL_AND_PLAYER_WIN };
            news_text_format()(NEWS_TEXT_BUFFER, template, p(1));
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
    } else if r.count == 2 || r.count == 3 || r.count == 4 {
        covered!("sx15_3 player list");
        let template = [0xa5f968, 0xa5f878, 0xa5f788][result]; // sx15_3: the list sentence by the result
        intl_format(NEWS_TEXT_BUFFER, template, INTL_FORM_LIST, 0, r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    } else {
        covered!("sx15_3 player count");
        intl_format(NEWS_TEXT_BUFFER, [0xa5fc08, 0xa5fb30, 0xa5fa58][result], INTL_FORM_COUNT, 0, r, t);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
}

/// The body with a highlighted player: three or four players make a list of the others first; the highlighted tree by
/// the result, the highlighted player's code and the draw; then two players add "and X", three or four "and the
/// list", more than four "and N other players from the club"; then the highlighted player's links (modes 1 and 5)
/// when his names still match.
unsafe fn intl_body_highlight(r: &IntlReport, t: &mut IntlTexts, out: usize) {
    let player: [usize; 4] = core::array::from_fn(|i| t.player[i].as_ptr() as usize);
    let p = |i: usize| player[i];
    let list = t.list.as_mut_ptr() as usize;
    match r.count {
        3 => { news_text_format()(list, INTL_LIST_TWO, p(1), p(2)); }
        4 => { news_text_format()(list, INTL_LIST_THREE, p(1), p(2), p(3)); }
        _ => {}
    }
    let [id, first, second, common] = r.highlight;
    exe::database_format_name_from_ids(first, second, common, t.highlight.as_mut_ptr(), 0x12c, 1, id);
    exe::database_format_name_from_ids(first, second, common, t.highlight_short.as_mut_ptr(), 0x12c, 5, id);
    covered!("sx15_3 highlighted player");
    let (template, form) = intl_pick(&INTL_HIGHLIGHT_TREES[r.result()], r.codes[0] as u8, r.no_codes, r.draw);
    intl_format(NEWS_TEXT_BUFFER, template, form, t.highlight.as_ptr() as usize, r, t);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    let (ours, theirs) = r.record().goals();
    match r.count {
        2 => {
            let template = if r.codes[1] as u8 & 0x7a == 0 || ours <= theirs { INTL_AND_PLAYER } else { INTL_AND_PLAYER_WIN };
            news_text_format()(NEWS_TEXT_BUFFER, template, p(1));
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        3 | 4 => {
            news_text_format()(NEWS_TEXT_BUFFER, INTL_AND_LIST, t.list.as_ptr() as usize);
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        n if n > 4 => {
            covered!("sx15_3 other players");
            let club = club_at(r.club as i32 as isize);
            let g = club.short_gender_name() as i8 as i32;
            news_text_format()(NEWS_TEXT_BUFFER, INTL_OTHERS, g, g, n as i32 - 1, club.addr() + club::SHORT_NAME);
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
        }
        _ => {}
    }
    let staff = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(staff, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, t.highlight.as_ptr(), 0x7d7, staff);
        exe::text_add_link(out, t.highlight_short.as_ptr(), 0x7d7, staff);
    }
}

/// The body's links: each player read whose names still match (modes 1 and 5), the score (when the fixture is still in
/// the match stats), the club, and both sides (0x7d6 in the B internationals, else 0x7d5).
unsafe fn intl_body_links(r: &IntlReport, t: &IntlTexts, out: usize) {
    for i in 0..r.players_read() {
        let [id, first, second, common] = r.players[i];
        let staff = staff_at(id as i32 as isize);
        if exe::database_staff_matches_names(staff, first, second, common, 0usize) != 0 {
            exe::text_add_link(out, t.player[i].as_ptr(), 0x7d7, staff);
            exe::text_add_link(out, t.player_short[i].as_ptr(), 0x7d7, staff);
        }
    }
    r.record().link_score(out, &t.score);
    let club = club_at(r.club as i32 as isize);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    let (nation, opponent) = r.sides();
    let link = if r.b_international() { 0x7d6 } else { 0x7d5 };
    exe::text_add_link(out, t.nation.as_ptr(), link, nation);
    exe::text_add_link(out, t.opponent.as_ptr(), link, opponent);
}

/// Writes the headline or the body of the news on how a club's players did for their nation (news 0x24, sent to the
/// club's manager by `send_club_international_report_news`).
///
/// The news item carries the match record, the highlighted player (the national side's highlight when he is from the
/// club, else −1), how many of the club's players took part, the club, up to four players, the marker 0x420e9, a
/// draw of 0 or 1 and each player's performance code (`match_performance_code`: 1 missed a penalty, 2/8/0x20/0x40
/// scored one/two/three/more, 4 left the pitch early, 0x10 man of the match).
///
/// Headline (`full` = 0): `intl_headline`. Body: the players' names, both sides' names (the nation's main side),
/// the fixture label (form 1, "friendly" lowercased) and the score line; then `intl_body_highlight` when there is a
/// highlighted player, else `intl_body_players`; then the links (`intl_body_links`). The templates are picked from the
/// trees by the result (won, drawn, lost), the first rule whose mask the first player's code holds (0x44, 0x14, 0x40,
/// 0x24, 0x20, 8, 0x12, 0x10, 2, 4, 1; not all in every tree), and the draw.
///
/// Defects of the original: with more than four players of the club, the players' slots are not read, so the marker is
/// not where the original looks for it: there are no codes, the texts use the count sentence and, with a highlighted
/// player, the template is picked by a draw that was never read (an uninitialised stack dword; the copy uses 0 and the
/// comparer leaves that case to the original).
/// Original: `NationalTeams::write_club_players_international_text` (0x770d20, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_club_players_international_text(_this: usize, news: usize, full: u32, out: usize) {
    if full == 0 {
        intl_headline(news, out);
        return;
    }
    let r = IntlReport::read(news);
    let mut t = intl_body_texts(&r);
    if r.highlight[0] as i32 == -1 {
        intl_body_players(&r, &mut t, out);
    } else {
        intl_body_highlight(&r, &mut t, out);
    }
    intl_body_links(&r, &t, out);
}

/// Comparer for `write_club_players_international_text`: side by side into two Texts (`compare_text_writer`); the
/// highlighted body with more than four players (a template picked by an uninitialised stack dword) runs only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_write_club_players_international_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    if full != 0 && IntlReport::read(news).draw_uninitialized() {
        covered!("sx15_3 uninitialized draw");
        return o(this, news, full, out);
    }
    compare_text_writer(out, || o(this, news, full, out), |t| write_club_players_international_text(this, news, full, t),
        || format!("write_club_players_international_text(news {:#x}, full {})", news, full));
}
