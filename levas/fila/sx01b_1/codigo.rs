// --- sx01b_1: Media::write_board_fans_reaction_news_text (0x738d50, media.cpp, 23,328 bytes), the whole routine in named pieces ---

/// Writes a langlib template into the shared text buffer (`BACKGROUND_TEXT`, 0xde1f64) the original writes the
/// reaction into before copying it to the Text.
macro_rules! reaction {
    ($tpl:expr $(, $a:expr)* $(,)?) => {
        story_lang!(BACKGROUND_TEXT, $tpl $(, $a)*)
    };
}

/// What `write_board_fans_reaction_news_text` reads from the news item and prepares before writing (the original's
/// frame). The news is posted by `adjust_board_confidence` (board) and `adjust_supporter_confidence` (supporters).
pub(crate) struct ReactionText {
    board: bool,
    topic: i32,
    variant: i32,
    draw: i32,
    club: Club,
    degree: i32,
    manager: Staff,
    manager_ids: [u32; 3],
    subject: Staff,
    subject_ids: [u32; 3],
    chairman: Staff,
    chairman_ids: [u32; 3],
    confidence: i32,
    expectations: u32,
    status_flag: u32,
    form_flag: u32,
    loyalty_flag: u32,
    professionalism_flag: u32,
    days_flag: u32,
    reputation_flag: u32,
    on_screen: bool,
    named: bool,
    subject_name: [u8; 0x78],
    subject_short: [u8; 0x78],
    chairman_name: [u8; 0x78],
    chairman_short: [u8; 0x78],
    manager_name: [u8; 0x78],
    manager_short: [u8; 0x78],
}

/// The arguments the templates take: the club's short name and its gender byte, and the six names.
#[derive(Clone, Copy)]
struct ReactionArgs {
    cg: i32,
    cs: usize,
    chair: usize,
    chair_s: usize,
    subj: usize,
    subj_s: usize,
    mgr: usize,
    mgr_s: usize,
}

/// The praise topics (0x28..0x3b): the media praised the subject.
fn praise_topic(t: i32) -> bool {
    (0x28..=0x3b).contains(&t)
}

/// The topics about the subject's recent actions rather than his performances.
fn actions_topic(t: i32) -> bool {
    matches!(t, 30 | 31 | 32 | 3 | 8 | 10 | 11 | 82 | 35 | 34)
}

/// The team media story kinds (0x3c..0x44, 0x5a..0x63; the original's byte-indexed jump tables).
fn team_story_kind(t: i32) -> bool {
    matches!(t, 0x3c..=0x44 | 0x5a..=0x63)
}

impl ReactionText {
    fn args(&self) -> ReactionArgs {
        let p = |b: &[u8; 0x78]| b.as_ptr() as usize;
        ReactionArgs {
            cg: unsafe { self.club.short_gender_name() as i8 as i32 },
            cs: self.club.addr() + club::SHORT_NAME,
            chair: p(&self.chairman_name),
            chair_s: p(&self.chairman_short),
            subj: p(&self.subject_name),
            subj_s: p(&self.subject_short),
            mgr: p(&self.manager_name),
            mgr_s: p(&self.manager_short),
        }
    }
    /// The reaction is about the subject (there is one and the topic is not a team story).
    fn about_subject(&self) -> bool {
        !self.subject.is_null() && !team_story_kind(self.topic)
    }
    /// The subject is one of the club's three favoured (`club::FAV_STAFF1`) or disliked (`club::DIS_STAFF1`) staff.
    unsafe fn subject_in(&self, first: usize) -> bool {
        (0..3).any(|k| read_usize(self.club.addr() + first + 4 * k) == self.subject.addr())
    }
    /// The chairman's name to you, or the board: `named` (0x64, chairman, subject) or `board` (0x64, subject).
    unsafe fn to_you(&self, named: usize, board: usize) {
        let a = self.args();
        if self.named {
            reaction!(named, 0x64, a.chair, a.subj);
        } else {
            reaction!(board, 0x64, a.subj);
        }
    }
    /// As `to_you` without the subject: `named` (chairman) or `board` ().
    unsafe fn to_you_plain(&self, named: usize, board: usize) {
        let a = self.args();
        if self.named {
            reaction!(named, a.chair);
        } else {
            reaction!(board);
        }
    }
    /// The club's chairman or board on the manager and the subject (another club's manager on the screen).
    unsafe fn of_manager(&self, named: usize, board: usize) {
        let a = self.args();
        if self.named {
            reaction!(named, a.cg, a.cg, 0x64, 0x64, a.cs, a.chair, a.mgr, a.subj);
        } else {
            self.club_manager_subject(board);
        }
    }
    /// As `of_manager` without the subject.
    unsafe fn of_manager_plain(&self, named: usize, board: usize) {
        let a = self.args();
        if self.named {
            reaction!(named, a.cg, a.cg, 0x64, a.cs, a.chair, a.mgr);
        } else {
            self.club_manager(board);
        }
    }
    /// A line with (0x64, subject).
    unsafe fn subject_line(&self, tpl: usize) {
        let a = self.args();
        reaction!(tpl, 0x64, a.subj);
    }
    /// A line with (club gender ×2, 0x64 ×2, club, manager, subject).
    unsafe fn club_manager_subject(&self, tpl: usize) {
        let a = self.args();
        reaction!(tpl, a.cg, a.cg, 0x64, 0x64, a.cs, a.mgr, a.subj);
    }
    /// A line with (club gender ×2, 0x64, club, manager).
    unsafe fn club_manager(&self, tpl: usize) {
        let a = self.args();
        reaction!(tpl, a.cg, a.cg, 0x64, a.cs, a.mgr);
    }
    /// A line with (club gender ×2, 0x64, club, subject): the supporters' club representative.
    unsafe fn club_subject(&self, tpl: usize) {
        let a = self.args();
        reaction!(tpl, a.cg, a.cg, 0x64, a.cs, a.subj);
    }
    /// A line with (club gender ×2, 0x64, club, subject's short name).
    unsafe fn club_subject_short(&self, tpl: usize) {
        let a = self.args();
        reaction!(tpl, a.cg, a.cg, 0x64, a.cs, a.subj_s);
    }
}

/// Reads the news slots (laid out by `adjust_board_confidence` and `adjust_supporter_confidence`): 0 board (1) or
/// supporters, 1 topic, 2 variant (the manager's answer: 1 backing or optimism, 2 criticism or caution, other
/// silence), 3 draw, 4 club, 5 degree (2 delighted, 3 pleased, 4 disappointed, 5 distressed, 9 surprised), 6..9 the
/// manager; 10..13 the subject when slot 10 is set (slot 14, the chairman's liking, is read and not used); then the
/// chairman (or -1), the confidence, the expectations (after a set flag) and the subject's flags: squad status, form,
/// loyalty, professionalism, days at the club, value (read and not used) and reputation.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x738d50..0x739063).
pub(crate) unsafe fn reaction_text_read(news: usize) -> ReactionText {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let board = slot(0) == 1;
    let topic = slot(1) as i32;
    let variant = slot(2) as i32;
    let draw = slot(3) as i32;
    let club = club_at(slot(4) as i32 as isize);
    let degree = slot(5) as i32;
    let manager = staff_at(slot(6) as i32 as isize);
    let manager_ids = [slot(7), slot(8), slot(9)];
    let (mut subject, mut subject_ids, mut k) = (Staff(0), [0u32; 3], 0xbu32);
    if slot(10) as i32 > -1 {
        subject = staff_at(slot(10) as i32 as isize);
        subject_ids = [slot(11), slot(12), slot(13)];
        k = 0xf;
        slot(14);
    }
    let (mut chairman, mut chairman_ids) = (Staff(0), [0u32; 3]);
    if slot(k) as i32 > -1 {
        chairman = staff_at(slot(k) as i32 as isize);
        chairman_ids = [slot(k + 1), slot(k + 2), slot(k + 3)];
        k += 4;
    } else {
        k += 1;
    }
    let confidence = slot(k) as i32;
    let mut expectations = 0;
    if slot(k + 1) != 0 {
        expectations = slot(k + 2);
        k += 3;
    } else {
        k += 2;
    }
    let (status_flag, form_flag, loyalty_flag) = (slot(k), slot(k + 1), slot(k + 2));
    let (professionalism_flag, days_flag) = (slot(k + 3), slot(k + 4));
    slot(k + 5);
    let reputation_flag = slot(k + 6);
    let mut empty = [0u8; 0x78];
    empty[0] = read_u8(0xad9c64); // the empty string
    ReactionText {
        board, topic, variant, draw, club, degree, manager, manager_ids, subject, subject_ids, chairman, chairman_ids,
        confidence, expectations, status_flag, form_flag, loyalty_flag, professionalism_flag, days_flag,
        reputation_flag, on_screen: false, named: false, subject_name: empty, subject_short: empty, chairman_name: empty,
        chairman_short: empty, manager_name: empty, manager_short: empty,
    }
}

/// The names (styles 1 and 5) of the subject, the chairman and the manager; `on_screen` when the manager is the human
/// on screen (`G_SCREEN_RECORDS_MANAGER` of `G_SCREEN_MANAGER_CURRENT_SLOT`); `named` (the chairman speaks by name)
/// when there is a chairman with a non-player sheet whose interference % 20 is at least the confidence % 5.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x739063..0x7391dd).
pub(crate) unsafe fn reaction_text_names(s: &mut ReactionText) {
    let name = |ids: [u32; 3], buf: &mut [u8; 0x78], style: u32, p: Staff| {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], buf.as_mut_ptr(), 0x78, style, read_u32(p.addr()));
    };
    if !s.subject.is_null() {
        name(s.subject_ids, &mut s.subject_name, 1, s.subject);
        name(s.subject_ids, &mut s.subject_short, 5, s.subject);
    }
    if !s.chairman.is_null() {
        name(s.chairman_ids, &mut s.chairman_name, 1, s.chairman);
        name(s.chairman_ids, &mut s.chairman_short, 5, s.chairman);
    }
    name(s.manager_ids, &mut s.manager_name, 1, s.manager);
    name(s.manager_ids, &mut s.manager_short, 5, s.manager);
    let screen = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    s.on_screen = screen == s.manager.addr();
    s.named = !s.chairman.is_null() && {
        let sheet = s.chairman.non_player();
        !sheet.is_null() && sheet.interference() as i32 % 20 >= s.confidence % 5
    };
}

/// The board's headline: on the subject's settling (topic 1: confident of his loyalty, or concerned by the
/// revelations when surprised or distressed); on the media's praise of the subject (topics 0x28..0x3b with a
/// subject: pleased by the recognition or the handling, or concerned; a degree other than 2..5 or 9 writes nothing);
/// otherwise by degree and the manager's answer: backs, slams, unhappy with, delighted with, fine with or happy with
/// the manager, by the chairman's name or the board.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x7391f7..0x739a39).
pub(crate) unsafe fn reaction_headline_board(s: &ReactionText) {
    covered!("sx01b_1 reaction headline board");
    let a = s.args();
    let (v, named) = (s.variant, s.named);
    if s.topic == 1 {
        if matches!(s.degree, 9 | 5) {
            reaction!(0xa36c74, 0x64, a.subj_s);
        } else {
            reaction!(0xa36cb8, 0x64, a.subj_s);
        }
        return;
    }
    if praise_topic(s.topic) && !s.subject.is_null() {
        match s.degree {
            9 | 5 | 4 => {
                if named {
                    reaction!(0xa36b28, a.chair_s);
                } else {
                    reaction!(0xa36af8);
                }
            }
            2 | 3 => match v {
                1 if named => reaction!(0xa36be8, a.chair_s),
                1 => reaction!(0xa36c30, 0x64, a.subj_s),
                2 if named => reaction!(0xa36be8, a.chair_s),
                2 => reaction!(0xa36ba8, a.subj_s),
                _ => reaction!(0xa36b70),
            },
            _ => {} // writes nothing: the shared buffer keeps its last text
        }
        return;
    }
    let about = s.about_subject();
    match s.degree {
        9 if about => match v {
            1 if named => reaction!(0xa36ac8, a.chair_s),
            1 => reaction!(0xa36aa0, a.subj_s),
            2 if named => reaction!(0xa36a4c, a.chair_s, a.subj_s),
            _ => reaction!(0xa36a0c, a.subj_s),
        },
        9 => match v {
            1 | 2 if named => reaction!(0xa3691c, 0x64, a.chair_s, a.mgr_s),
            1 | 2 => reaction!(0xa368f4),
            _ if named => reaction!(0xa369b4, 0x64, a.chair_s, a.mgr_s),
            _ => reaction!(0xa36974, 0x64, a.mgr_s),
        },
        5 if about => match v {
            1 if named => reaction!(0xa368c4, a.chair_s),
            1 => reaction!(0xa3689c, a.subj_s),
            2 if named => reaction!(0xa3684c, 0x64, a.chair_s, a.subj_s),
            2 => reaction!(0xa36814, 0x64, a.subj_s),
            _ => reaction!(0xa367e0, 0x64, a.subj_s),
        },
        5 => match v {
            1 | 2 if named => reaction!(0xa36700, 0x64, a.chair_s, a.mgr_s),
            1 | 2 => reaction!(0xa366e0),
            _ if named => reaction!(0xa3678c, 0x64, a.chair_s, a.mgr_s),
            _ => reaction!(0xa36754, 0x64, a.mgr_s),
        },
        4 if about => match v {
            1 if named => reaction!(0xa36694, a.chair_s, a.mgr_s),
            1 => reaction!(0xa36664, a.subj_s),
            2 if named => reaction!(0xa36620, a.chair_s, a.subj_s),
            2 => reaction!(0xa365f4, a.subj_s),
            _ => reaction!(0xa367e0, 0x64, a.subj_s),
        },
        4 => match v {
            2 if named => reaction!(0xa36598, 0x64, a.chair_s, a.mgr_s),
            1 if named => reaction!(0xa36540, 0x64, a.chair_s, a.mgr_s),
            1 | 2 => reaction!(0xa36518),
            _ => reaction!(0xa364f0),
        },
        2 if about => {
            if named {
                reaction!(0xa36318, a.chair_s, a.subj_s);
            } else if v == 1 {
                reaction!(0xa364b4, a.subj_s);
            } else {
                reaction!(0xa36480, a.subj_s);
            }
        }
        2 => match v {
            2 if named => reaction!(0xa36428, 0x64, a.chair_s, a.mgr_s),
            2 => reaction!(0xa36404),
            1 if named => reaction!(0xa363c8, a.chair_s),
            1 => reaction!(0xa363a8),
            _ => reaction!(0xa3636c, 0x64, a.mgr_s),
        },
        _ if about => match v {
            1 if named => reaction!(0xa36318, a.chair_s, a.subj_s),
            1 => reaction!(0xa362e0, 0x64, a.subj_s),
            _ if named => reaction!(0xa3628c, 0x64, 0x64, a.subj_s, a.chair_s),
            _ => reaction!(0xa3624c, 0x64, a.subj_s),
        },
        _ => match v {
            2 if named => reaction!(0xa36214, a.chair_s),
            2 => reaction!(0xa361dc, 0x64, a.mgr_s),
            1 if named => reaction!(0xa361a0, a.chair_s),
            1 => reaction!(0xa36164, 0x64, a.mgr_s),
            _ => reaction!(0xa3612c, 0x64, a.mgr_s),
        },
    }
}

/// The supporters' headline: on the media's praise of the subject (pleased by your praise or firm handling, or
/// worried); on the subject's settling (topic 1: happy with him, or concerned); on the team's performances (topics
/// 0x3c, 0x3d: reassured, want changes, unconvinced, worried, complain about the silence); otherwise by degree and
/// answer: surprised, slam, unhappy with, pay tribute, disgusted, delighted, pleased.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x739a39..0x739f1a).
pub(crate) unsafe fn reaction_headline_fans(s: &ReactionText) {
    covered!("sx01b_1 reaction headline fans");
    let a = s.args();
    let (t, v) = (s.topic, s.variant);
    if praise_topic(t) && !s.subject.is_null() {
        match (s.degree, v) {
            (9 | 5 | 4, 1) => reaction!(0xa36030, a.subj_s),
            (9 | 5 | 4, _) => reaction!(0xa36000),
            (_, 1) => reaction!(0xa360e8, 0x64, a.subj_s),
            (_, 2) => reaction!(0xa360a0, 0x64, a.subj_s),
            _ => reaction!(0xa36074),
        }
        return;
    }
    if t == 1 {
        if matches!(s.degree, 9 | 5) {
            reaction!(0xa35e50, 0x64, a.subj_s);
        } else {
            reaction!(0xa35fd0, 0x64, a.subj_s);
        }
        return;
    }
    if t == 60 || t == 61 {
        match (s.degree, v) {
            (9 | 5 | 4, 1) => reaction!(0xa35ae8),
            (9 | 5 | 4, 2) => reaction!(0xa35ac8),
            (9 | 5 | 4, _) => reaction!(0xa35aa4),
            (_, 1) => reaction!(0xa35b14),
            (_, 2) => reaction!(0xa35b00),
            _ => reaction!(0xa35aa4),
        }
        return;
    }
    let about = s.about_subject();
    match s.degree {
        9 if about => match v {
            1 => reaction!(0xa35f88, 0x64, a.subj_s),
            2 => reaction!(0xa35f40, 0x64, a.subj_s),
            _ => reaction!(0xa35e50, 0x64, a.subj_s),
        },
        9 => match v {
            2 => reaction!(0xa35f14),
            1 => reaction!(0xa35ef0),
            _ => reaction!(0xa35ee0),
        },
        5 if about => match v {
            1 => reaction!(0xa35eb8, a.subj_s),
            2 => reaction!(0xa35e84, 0x64, a.subj_s),
            _ => reaction!(0xa35e50, 0x64, a.subj_s),
        },
        5 => match v {
            2 => reaction!(0xa35e28),
            1 => reaction!(0xa35e04),
            _ => reaction!(0xa35dec),
        },
        4 if about => match v {
            1 => reaction!(0xa35dbc, 0x64, a.subj_s),
            2 => reaction!(0xa35d90, 0x64, a.subj_s),
            _ => reaction!(0xa35e50, 0x64, a.subj_s),
        },
        4 => match v {
            2 => reaction!(0xa35d64),
            1 => reaction!(0xa35d40),
            _ => reaction!(0xa35d1c),
        },
        2 if about => {
            if t == 33 {
                reaction!(0xa35ce0, 0x64, a.subj_s);
            } else if v == 1 {
                reaction!(0xa35cac, 0x64, a.subj_s);
            } else {
                reaction!(0xa35c70, 0x64, a.subj_s);
            }
        }
        2 => {
            if v == 2 {
                reaction!(0xa35c4c);
            } else {
                reaction!(0xa35c24);
            }
        }
        _ if about => {
            if v != 1 {
                reaction!(0xa35b74, 0x64, a.subj_s);
            } else if t == 82 {
                reaction!(0xa35be8, 0x64, a.subj_s);
            } else if t == 33 {
                reaction!(0xa35ce0, 0x64, a.subj_s);
            } else {
                reaction!(0xa35bb0, 0x64, a.subj_s);
            }
        }
        _ => {
            if v == 2 {
                reaction!(0xa35b50);
            } else {
                reaction!(0xa35b30);
            }
        }
    }
}

/// The board on the subject's outburst (topic 33): believes you were right to admit he is unhappy (answer 2) or are
/// handling it correctly; when surprised, distressed or disappointed, worried your reaction unsettles him further or
/// disappointed by your defence of him.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x739f41..0x73a08e).
pub(crate) unsafe fn reaction_board_outburst(s: &ReactionText) {
    covered!("sx01b_1 reaction board outburst");
    let a = s.args();
    let two = s.variant == 2;
    if !matches!(s.degree, 9 | 5 | 4) {
        match (s.named, two) {
            (true, true) => reaction!(0xa35974, 0x64, a.chair, a.subj, a.chair_s),
            (true, false) => reaction!(0xa35898, 0x64, a.chair, a.subj, a.chair_s),
            (false, true) => reaction!(0xa357b0, 0x64, a.subj),
            (false, false) => reaction!(0xa35720, 0x64, a.subj),
        }
        return;
    }
    match (s.named, two) {
        (true, true) => reaction!(0xa3562c, 0x64, a.chair, a.subj, a.chair_s),
        (false, true) => reaction!(0xa35530, 0x64, a.subj_s),
        (true, false) => reaction!(0xa3548c, 0x64, a.chair, a.subj),
        (false, false) => reaction!(0xa35414, 0x64, a.subj_s),
    }
}

/// The board on the rumours that the subject is not settling in the area (topic 1): pleased with your handling, or
/// (surprised, distressed, disappointed) worried, and with answer 2 that admitting it attracts more attention.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73a08e..0x73a172).
pub(crate) unsafe fn reaction_board_unsettled(s: &ReactionText) {
    covered!("sx01b_1 reaction board unsettled");
    let a = s.args();
    if !matches!(s.degree, 9 | 5 | 4) {
        if s.named {
            reaction!(0xa3530c, 0x64, a.chair, a.subj, a.chair_s);
        } else {
            reaction!(0xa3525c, 0x64, a.subj_s);
        }
    } else if !s.named {
        reaction!(0xa35008, 0x64, a.subj_s);
    } else if s.variant == 2 {
        reaction!(0xa35108, 0x64, a.chair, a.subj, a.chair_s);
    } else {
        reaction!(0xa35070, 0x64, a.chair, a.subj);
    }
}

/// The board on the media's praise of the subject (topics 0x28..0x3b, with or without a subject). Pleased or
/// delighted: your praise (answer 1: a role model when a senior squad member with professionalism, commended when in
/// form, loyal and professional, otherwise building his confidence), your words of wisdom (answer 2; without the
/// chairman's name the template still names "Marc Vaughan"), or not pandering to the media. Surprised, distressed or
/// disappointed: over-confidence, dented self-belief, or the media turning on you. Another degree writes nothing.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73a172..0x73a352).
pub(crate) unsafe fn reaction_board_praise(s: &ReactionText) {
    covered!("sx01b_1 reaction board praise");
    let a = s.args();
    match (s.degree, s.variant) {
        (2 | 3, 1) if s.named => {
            if s.status_flag != 0 && s.professionalism_flag != 0 {
                reaction!(0xa34f10, 0x64, a.chair, a.subj);
            } else if s.form_flag != 0 && s.loyalty_flag != 0 && s.professionalism_flag != 0 {
                reaction!(0xa34e18, 0x64, a.chair, a.subj);
            } else {
                reaction!(0xa34d4c, 0x64, a.chair, a.subj);
            }
        }
        (2 | 3, 1) => reaction!(0xa34cb4, 0x64, a.subj_s),
        (2 | 3, 2) if s.named => reaction!(0xa34bd0, 0x64, a.chair_s, a.subj_s),
        (2 | 3, 2) => reaction!(0xa34bd0, 0x64, 0x99adf0, a.subj_s), // "Marc Vaughan"
        (2 | 3, _) => reaction!(0xa34b6c),
        (9 | 5 | 4, 1) if s.named => reaction!(0xa34aa4, 0x64, a.chair, a.subj),
        (9 | 5 | 4, 1) => reaction!(0xa34a3c, 0x64, a.subj_s),
        (9 | 5 | 4, 2) if s.named => reaction!(0xa34960, 0x64, a.chair, a.subj),
        (9 | 5 | 4, 2) => reaction!(0xa34890, 0x64, a.subj_s, a.subj_s),
        (9 | 5 | 4, _) => reaction!(0xa34820),
        _ => {} // writes nothing: the shared buffer keeps its last text
    }
}

/// The board surprised (degree 9). About the subject: by your defence of his performances (the money invested for
/// topic 9) or of his actions (wages and training for topic 8), by your lack of support (rest for topic 7, a special
/// talent for topic 6 with the reputation flag), by your criticism, or "career effectively over" (topic 3) when the
/// manager is not on screen. Otherwise: your lack of optimism (answer 2), false hopes (1), your silence.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73a352..0x73aaf8).
pub(crate) unsafe fn reaction_board_surprised(s: &ReactionText) {
    covered!("sx01b_1 reaction board surprised");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match (s.variant, s.on_screen) {
            (2, true) => s.to_you_plain(0xa33160, 0xa33100),
            (2, false) => s.of_manager_plain(0xa33044, 0xa32fa0),
            (1, true) => s.to_you_plain(0xa32e8c, 0xa32d94),
            (1, false) => s.of_manager_plain(0xa31844, 0xa31764),
            (_, true) => s.to_you_plain(0xa32cb8, 0xa32bec),
            (_, false) => s.of_manager_plain(0xa32b34, 0xa32a8c),
        }
        return;
    }
    match (s.variant, s.on_screen) {
        (1, true) if !act => {
            if !s.named {
                reaction!(0xa3455c, 0x64, a.subj);
            } else if t == 9 {
                reaction!(0xa346f8, 0x64, a.chair, a.subj);
            } else {
                reaction!(0xa34628, 0x64, a.chair, a.subj);
            }
        }
        (1, true) => match (s.named, t == 8) {
            (true, true) => reaction!(0xa34440, 0x64, a.chair, a.subj),
            (true, false) => reaction!(0xa343b0, 0x64, a.chair, a.subj),
            (false, true) => reaction!(0xa342a8, 0x64, a.subj),
            (false, false) => reaction!(0xa34230, 0x64, a.subj),
        },
        (1, false) if !act => s.of_manager(0xa3417c, 0xa340d4),
        (1, false) => s.of_manager(0xa34024, 0xa33f90),
        (2, true) if t == 7 => s.to_you(0xa33eb0, 0xa33e34),
        (2, true) if t == 6 => {
            if !s.named {
                reaction!(0xa33c64, 0x64, a.subj);
            } else if s.reputation_flag != 0 {
                reaction!(0xa33d5c, 0x64, a.chair, a.subj);
            } else {
                reaction!(0xa33cd4, 0x64, a.chair, a.subj);
            }
        }
        (2, true) if !act => s.to_you(0xa33b68, 0xa33ad0),
        (2, true) => s.to_you(0xa33a30, 0xa33400),
        (2, false) if !act => s.of_manager(0xa33938, 0xa33860),
        (2, false) => match (s.named, t == 3) {
            (true, true) => reaction!(0xa3371c, a.cg, a.cg, 0x64, a.cg, a.cg, a.cs, a.chair, a.mgr, a.subj, a.cs),
            (true, false) => reaction!(0xa33634, a.cg, a.cg, 0x64, 0x64, a.cs, a.chair, a.mgr, a.subj),
            (false, true) => reaction!(0xa33560, a.cg, a.cg, 0x64, a.cs, a.mgr, a.subj),
            (false, false) => s.club_manager_subject(0xa33490),
        },
        (_, true) => s.subject_line(if act { 0xa33400 } else { 0xa33ad0 }),
        (_, false) => s.club_manager_subject(if act { 0xa33248 } else { 0xa33318 }),
    }
}

/// The board distressed (degree 5): extremely disappointed by your defence of the subject, upset by your lack of
/// support (rest for topic 7, special talent for topic 6) or angry at your criticism; otherwise angered by your lack
/// of optimism, concerned by false hopes (the degree-9 text when on screen) or by your lack of comment.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73aaf8..0x73b1de).
pub(crate) unsafe fn reaction_board_distressed(s: &ReactionText) {
    covered!("sx01b_1 reaction board distressed");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match (s.variant, s.on_screen) {
            (2, true) => s.to_you_plain(0xa31b04, 0xa31aa4),
            (2, false) => s.of_manager_plain(0xa319e4, 0xa31940),
            (1, true) => s.to_you_plain(0xa32e8c, 0xa32d94),
            (1, false) => s.of_manager_plain(0xa31844, 0xa31764),
            (_, true) => s.to_you_plain(0xa3168c, 0xa315bc),
            (_, false) => s.of_manager_plain(0xa31504, 0xa3145c),
        }
        return;
    }
    match (s.variant, s.on_screen) {
        (1, true) if !act => s.to_you(0xa329f4, 0xa32974),
        (1, true) => s.to_you(0xa328e0, 0xa32864),
        (1, false) if !act => s.of_manager(0xa32784, 0xa326bc),
        (1, false) => s.of_manager(0xa325e0, 0xa3251c),
        (2, true) if t == 7 => s.to_you(0xa33eb0, 0xa33e34),
        (2, true) if t == 6 => {
            if !s.named {
                reaction!(0xa323d0, 0x64, a.subj);
            } else if s.reputation_flag != 0 {
                reaction!(0xa32440, 0x64, a.chair, a.subj);
            } else {
                reaction!(0xa33cd4, 0x64, a.chair, a.subj);
            }
        }
        (2, true) if !act => s.to_you(0xa3231c, 0xa31da8),
        (2, true) => s.to_you(0xa32280, 0xa321f0),
        (2, false) if !act => s.of_manager(0xa320f4, 0xa32010),
        (2, false) => s.of_manager(0xa31f20, 0xa31e44),
        (_, true) => s.subject_line(if act { 0xa321f0 } else { 0xa31da8 }),
        (_, false) => s.club_manager_subject(if act { 0xa31bec } else { 0xa31cc0 }),
    }
}

/// The board disappointed (degree 4): somewhat disappointed by your defence of the subject, upset by your refusal to
/// back him, surprised at your lack of support; otherwise surprised at your lack of optimism, a little concerned by
/// false hopes, a little surprised at your lack of comment.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73b1de..0x73b845).
pub(crate) unsafe fn reaction_board_disappointed(s: &ReactionText) {
    covered!("sx01b_1 reaction board disappointed");
    let a = s.args();
    let act = actions_topic(s.topic);
    if !s.about_subject() {
        match (s.variant, s.on_screen) {
            (2, true) => s.to_you_plain(0xa30500, 0xa30438),
            (2, false) if s.named => reaction!(0xa302f4, a.cg, a.cg, 0x64, a.cs, a.chair, a.mgr, a.chair_s),
            (2, false) => s.club_manager(0xa30250),
            (1, true) => s.to_you_plain(0xa3013c, 0xa30048),
            (1, false) => s.of_manager_plain(0xa2ff4c, 0xa2fe64),
            (_, true) => reaction!(0xa2fdfc),
            (_, false) => s.club_manager(0xa2fd4c),
        }
        return;
    }
    match (s.variant, s.on_screen) {
        (1, true) if !act => s.to_you(0xa313c8, 0xa31348),
        (1, true) => s.to_you(0xa312b8, 0xa3123c),
        (1, false) if !act => s.of_manager(0xa3115c, 0xa31094),
        (1, false) => s.of_manager(0xa30fbc, 0xa30ef8),
        (2, true) if !act => s.to_you(0xa30e58, 0xa30dd0),
        (2, true) => s.to_you(0xa30d3c, 0xa30cb8),
        (2, false) if !act => s.of_manager(0xa30bc8, 0xa30af4),
        (2, false) => s.of_manager(0xa30a14, 0xa30948),
        (_, true) if !act => s.to_you(0xa308a4, 0xa30814),
        (_, true) => s.subject_line(0xa3078c),
        (_, false) => s.club_manager_subject(if act { 0xa305d8 } else { 0xa306b0 }),
    }
}

/// The board delighted (degree 2): by your defence of the subject, in agreement with your criticism, happy at your
/// lack of support (the club's boss criticising him, with the chairman's name, when not on screen); otherwise
/// approving your caution (a title or promotion bid for topics 63 and 96), delighted with your optimism or your
/// unwillingness to comment.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73b845..0x73bf4b).
pub(crate) unsafe fn reaction_board_delighted(s: &ReactionText) {
    covered!("sx01b_1 reaction board delighted");
    let a = s.args();
    let bid = matches!(s.topic, 63 | 96);
    let act = actions_topic(s.topic);
    if !s.about_subject() {
        match (s.variant, s.on_screen) {
            (2, true) => s.to_you_plain(if bid { 0xa2ed90 } else { 0xa2ee6c }, 0xa2ed38),
            (2, false) if s.named => reaction!(if bid { 0xa2ead0 } else { 0xa2ec04 }, a.cg, a.cg, 0x64, a.cs, a.chair, a.mgr, a.chair_s),
            (2, false) => s.club_manager(0xa2ea30),
            (1, true) => s.to_you_plain(0xa2e9c4, 0xa2e96c),
            (1, false) if s.named && bid => reaction!(0xa2e710, a.cg, a.cg, 0x64, a.cs, a.chair, a.mgr, a.chair_s),
            (1, false) if s.named => reaction!(0xa2e834, a.cg, a.cg, 0x64, 0x64, a.cs, a.chair, a.mgr, a.chair_s, a.mgr_s),
            (1, false) => s.club_manager(0xa2e67c),
            (_, true) => reaction!(0xa2e614),
            (_, false) => s.club_manager(0xa2e564),
        }
        return;
    }
    match (s.variant, s.on_screen) {
        (1, true) if !act => s.to_you(0xa2fcc4, 0xa2fc50),
        (1, true) => s.to_you(0xa2fbcc, 0xa2fb5c),
        (1, false) if !act => s.of_manager(0xa2fa84, 0xa2f9c8),
        (1, false) => s.of_manager(0xa2f8f8, 0xa2f840),
        (2, true) if !act => s.to_you(0xa2f778, 0xa2f6d4),
        (2, true) => s.to_you(0xa2f5e4, 0xa2f50c),
        (2, false) if !act => s.of_manager(0xa2f424, 0xa2f350),
        (2, false) => s.club_manager_subject(0xa2f298),
        (_, true) => s.subject_line(if act { 0xa2f184 } else { 0xa2f20c }),
        (_, false) if !act => s.club_manager_subject(0xa2f0ac),
        (_, false) if s.named => reaction!(0xa2ef44, a.cg, a.cg, 0x64, 0x64, a.cs, a.mgr, a.subj, a.chair, a.chair_s),
        (_, false) => s.club_manager_subject(0xa2d8f8),
    }
}

/// The board pleased (degree 3 and any other): by your defence of the subject (the board's "we" by the draw's
/// parity; the chairman's personal backing above 7,500 confidence when not on screen), agreeing with your criticism,
/// unperturbed by your lack of support; otherwise glad you are not over-confident, pleased at your confidence,
/// reassured by your unwillingness to comment.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73bf4b..0x73c646).
pub(crate) unsafe fn reaction_board_pleased(s: &ReactionText) {
    covered!("sx01b_1 reaction board pleased");
    let a = s.args();
    let bid = matches!(s.topic, 63 | 96);
    let act = actions_topic(s.topic);
    if !s.about_subject() {
        match (s.variant, s.on_screen) {
            (2, true) if bid => s.to_you_plain(0xa2d744, 0xa2d6c8),
            (2, true) => s.to_you_plain(0xa2d860, 0xa2d7e4),
            (2, false) => s.of_manager_plain(0xa2d614, 0xa2d578),
            (1, true) if bid => s.to_you_plain(0xa2d40c, 0xa2d3ac),
            (1, true) => s.to_you_plain(0xa2d4f8, 0xa2d498),
            (1, false) => s.club_manager(0xa2d30c),
            (_, true) => reaction!(0xa2d298),
            (_, false) => s.of_manager_plain(0xa2d1c0, 0xa2d104),
        }
        return;
    }
    match (s.variant, s.on_screen) {
        (1, true) if act => s.subject_line(0xa2e290),
        (1, true) if s.named => reaction!(0xa2e464, 0x64, a.chair, a.subj),
        (1, true) => s.subject_line(if s.draw % 2 != 0 { 0xa2e3a0 } else { 0xa2e2fc }),
        (1, false) if act => s.club_manager_subject(0xa2dee4),
        (1, false) if !s.named => s.club_manager_subject(0xa2df9c),
        (1, false) if s.confidence > 7500 => reaction!(0xa2e12c, a.cg, a.cg, 0x64, 0x64, a.cs, a.chair, a.mgr, a.subj, a.chair_s),
        (1, false) => reaction!(0xa2e058, a.cg, a.cg, 0x64, 0x64, a.cs, a.chair, a.mgr, a.subj),
        (2, true) if !act => s.to_you(0xa2de28, 0xa2dd90),
        (2, true) => s.to_you(0xa2dc6c, 0xa2dbec),
        (2, false) if !act => s.club_manager_subject(0xa2f350),
        (2, false) => s.of_manager(0xa2db18, 0xa2f298),
        (_, true) => s.subject_line(if act { 0xa2d9c4 } else { 0xa2da84 }),
        (_, false) => s.club_manager_subject(if act { 0xa2d8f8 } else { 0xa2f0ac }),
    }
}

/// The supporters on the subject's future (topic 3): reassured he features in the manager's plans, pleased by the
/// announcement, or the supporters' club spokesman; when surprised, distressed or disappointed: unconvinced, shocked,
/// or the spokesman on the manager.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73c646..0x73c760).
pub(crate) unsafe fn reaction_fans_future(s: &ReactionText) {
    covered!("sx01b_1 reaction fans future");
    let a = s.args();
    match (matches!(s.degree, 9 | 5 | 4), s.variant) {
        (true, 1) => reaction!(0xa2cd4c, a.cg, a.cg, a.cs, a.subj),
        (true, 2) => reaction!(0xa2cc60, a.cg, a.cg, a.cs, a.subj),
        (true, _) => reaction!(0xa2cb24, a.cg, a.cg, a.cs, a.mgr),
        (false, 1) => reaction!(0xa2d058, a.cg, a.cg, a.cs, a.subj),
        (false, 2) => reaction!(0xa2cf60, a.cg, a.cg, a.cs, a.subj),
        (false, _) => reaction!(0xa2ce10, a.cg, a.cg, a.cs, a.mgr, a.subj),
    }
}

/// The supporters on the subject's outburst (topic 33): disgust, or (surprised, distressed, disappointed) disgust when
/// he is one of the club's disliked staff and disappointment otherwise.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73c760..0x73c814).
pub(crate) unsafe fn reaction_fans_outburst(s: &ReactionText) {
    covered!("sx01b_1 reaction fans outburst");
    if !matches!(s.degree, 9 | 5 | 4) {
        s.club_subject(0xa2ca2c);
    } else if !s.subject.is_null() && s.subject_in(club::DIS_STAFF1) {
        s.club_subject(0xa2c9a8);
    } else {
        s.club_subject(0xa2c91c);
    }
}

/// The supporters on the team's performances (topics 60, 61): appreciate the reassurances, or feel the manager is out
/// of touch; when surprised, distressed or disappointed: unconvinced, fed up with excuses, out of touch.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73df7e..0x73e048).
pub(crate) unsafe fn reaction_fans_performances(s: &ReactionText) {
    covered!("sx01b_1 reaction fans performances");
    let a = s.args();
    match (matches!(s.degree, 9 | 5 | 4), s.variant) {
        (true, 1) => reaction!(0xa28368, a.cg, a.cg, a.cs),
        (true, 2) => reaction!(0xa2828c, a.cg, a.cg, a.cs),
        (true, _) => reaction!(0xa281cc, a.cg, a.cg, a.cs),
        (false, 1) => s.club_manager(0xa28638),
        (false, 2) => s.club_manager(0xa28530),
        (false, _) => reaction!(0xa2844c, a.cg, a.cg, a.cs, a.mgr),
    }
}

/// The supporters on the media's praise of the subject (topics 0x28..0x3b with a subject). Pleased or delighted:
/// you acknowledged his contribution (by topic), your cautious reaction (by topic, a disliked player, his ambition,
/// loyalty and pressure), your non-reaction. Otherwise: your praise worries them (a disliked player by his work rate,
/// pressure, professionalism and aggression; a club reputation below 4,250 with a hard-working player), or your lack
/// of praise (a favourite with long service, a hard-working professional).
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73c848..0x73cd34).
pub(crate) unsafe fn reaction_fans_praise(s: &ReactionText) {
    covered!("sx01b_1 reaction fans praise");
    let a = s.args();
    let (t, p) = (s.topic, s.subject);
    if !matches!(s.degree, 9 | 5 | 4) {
        match s.variant {
            1 => match t {
                48 | 45 => s.club_subject_short(0xa2c294),
                46 => reaction!(0xa2c814, a.cg, a.cg, 0x64, a.cg, a.cg, a.cs, a.subj_s, a.cs),
                49 => reaction!(0xa2c720, a.cg, a.cg, 0x64, a.cg, a.cg, a.cs, a.subj_s, a.cs),
                50 => reaction!(0xa2c608, a.cg, a.cg, 0x64, a.cg, a.cg, a.cs, a.subj_s, a.cs),
                42 => s.club_subject_short(0xa2c518),
                41 | 44 => s.club_subject_short(0xa2c380),
                _ => s.club_subject_short(0xa2c47c),
            },
            2 => {
                let tpl = if t == 41 || t == 44 {
                    0xa2bd74
                } else if t == 42 {
                    0xa2be5c
                } else if s.subject_in(club::DIS_STAFF1) {
                    0xa2bf58
                } else if p.ambition() as i32 > 15 && (p.loyalty() as i32) < 6 {
                    0xa2c1a8
                } else if (p.pressure() as i32) < 6 {
                    0xa2c0c8
                } else {
                    0xa2c038
                };
                s.club_subject_short(tpl);
            }
            _ => s.club_subject_short(0xa2bce8),
        }
        return;
    }
    if s.variant == 1 {
        let tpl = if s.subject_in(club::DIS_STAFF1) {
            let sheet = p.player();
            if (sheet.work_rate() as i32) < 6 {
                0xa2baec
            } else if ((p.pressure() as i32) < 10 || (p.professionalism() as i32) < 10) && sheet.aggression() as i32 > 15 {
                0xa2b9fc
            } else if p.professionalism() == 1 {
                0xa2b900
            } else {
                0xa2bbac
            }
        } else if (s.club.reputation() as i32) < 4250 && !p.player().is_null() && p.player().work_rate() as i32 > 15 {
            0xa2bc48
        } else {
            // the original also compares the sheet's work rate with 10 here and ignores the result
            0xa2bbac
        };
        reaction!(tpl, a.cg, a.cg, a.cs, a.subj_s);
    } else if s.subject_in(club::FAV_STAFF1) && s.days_flag != 0 {
        s.club_subject_short(0xa2b83c);
    } else if p.player().work_rate() as i32 > 15 && p.professionalism() as i32 > 10 {
        s.club_subject_short(0xa2b74c);
    } else {
        s.club_subject_short(0xa2b6a4);
    }
}

/// The supporters surprised (degree 9). About the subject: by the manager's backing (by the draw's parity; on
/// actions; a spokesman or spokeswoman by the club's id % 7), by his criticism, by his silence; topic 1 is the
/// representative echoing the disappointment at his not settling. Otherwise: his caution, his optimism, his silence.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73cd38..0x73d159).
pub(crate) unsafe fn reaction_fans_surprised(s: &ReactionText) {
    covered!("sx01b_1 reaction fans surprised");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match s.variant {
            2 => s.club_manager(0xa2aecc),
            1 => reaction!(0xa2adb4, a.cg, a.cg, 0x64, a.cs, a.mgr, a.mgr_s),
            _ => s.club_manager(0xa2ad24),
        }
        return;
    }
    let echo = 0xa2b504;
    match s.variant {
        1 if s.draw % 2 != 0 => {
            if act {
                s.club_manager_subject(0xa2b450);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(0xa2b5f4);
            }
        }
        1 => {
            if act {
                s.club_manager_subject(0xa2b220);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(if read_i32(s.club.addr()) % 7 != 0 { 0xa2b39c } else { 0xa2b2e4 }); // the club's id
            }
        }
        2 => {
            if act {
                s.club_manager_subject(0xa2b000);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                reaction!(0xa2b0bc, a.cg, a.cg, 0x64, 0x64, a.cs, a.mgr, a.subj, a.mgr_s);
            }
        }
        _ => {
            if act {
                s.club_manager_subject(0xa29d24);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(0xa2af50);
            }
        }
    }
}

/// The supporters distressed (degree 5): extremely distressed by the manager's backing (spokesman or spokeswoman by
/// the club's id % 7), the spokesman criticising or slamming him; topic 1 is the representative echoing the
/// supporters. Otherwise: slammed lack of caution, mocked optimism, hit out at the silence.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73d159..0x73d4f8).
pub(crate) unsafe fn reaction_fans_distressed(s: &ReactionText) {
    covered!("sx01b_1 reaction fans distressed");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match s.variant {
            2 => reaction!(0xa2a5bc, a.cg, a.cg, 0x64, a.cs, a.mgr, a.mgr_s),
            1 => reaction!(0xa2a4b8, a.cg, a.cg, 0x64, a.cs, a.mgr, a.mgr_s),
            _ => s.club_manager(0xa2a428),
        }
        return;
    }
    let echo = 0xa29e8c;
    match s.variant {
        1 if s.draw % 2 != 0 => {
            if act {
                s.club_manager_subject(0xa2abb0);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(0xa2ac68);
            }
        }
        1 => {
            if act {
                s.club_manager_subject(0xa2a980);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(if read_i32(s.club.addr()) % 7 != 0 { 0xa2aaf4 } else { 0xa2aa38 }); // the club's id
            }
        }
        2 => {
            if act {
                s.club_manager_subject(0xa2a6bc);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                reaction!(0xa2a820, a.cg, a.cg, 0x64, 0x64, a.cs, a.mgr, a.subj, a.mgr_s);
            }
        }
        _ => s.club_manager_subject(if act || t == 1 { 0xa2a6bc } else { 0xa2a770 }),
    }
}

/// The supporters disappointed (degree 4): somewhat disappointed or disillusioned by the manager's backing, the
/// spokesman (or spokeswoman, by the club's id % 7) criticising or questioning him; topic 1 is the representative
/// echoing the supporters. Otherwise: disappointment at the caution, expectations, concern (a title or promotion bid
/// for topics 63 and 96).
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73d4f8..0x73d912).
pub(crate) unsafe fn reaction_fans_disappointed(s: &ReactionText) {
    covered!("sx01b_1 reaction fans disappointed");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match s.variant {
            2 => reaction!(0xa29c10, a.cg, a.cg, 0x64, a.cs, a.mgr, a.mgr_s),
            1 => s.club_manager(0xa29b40),
            _ => s.club_manager(if matches!(t, 96 | 63) { 0xa299f4 } else { 0xa29a9c }),
        }
        return;
    }
    let echo = 0xa29e8c;
    match s.variant {
        1 if s.draw % 2 != 0 => s.club_manager_subject(if act || t == 1 { 0xa2a2bc } else { 0xa2a36c }),
        1 => {
            if act {
                s.club_manager_subject(0xa2a08c);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(if read_i32(s.club.addr()) % 7 != 0 { 0xa2a200 } else { 0xa2a138 }); // the club's id
            }
        }
        2 => {
            if act {
                s.club_manager_subject(0xa29ddc);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                reaction!(0xa29f68, a.cg, a.cg, 0x64, 0x64, a.cs, a.mgr, a.subj, a.mgr_s);
            }
        }
        _ => {
            if act {
                s.club_manager_subject(0xa29d24);
            } else if t == 1 {
                s.club_subject(echo);
            } else {
                s.club_manager_subject(0xa2af50);
            }
        }
    }
}

/// The supporters delighted (degree 2): delighted by the manager's backing (a high-profile supporter for topic 82),
/// backing his criticism, pleased at his silence; topic 1 is the representative echoing the supporters. Otherwise:
/// the caution, the confidence (title or promotion bid for topics 63 and 96), delight.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73d912..0x73dc30).
pub(crate) unsafe fn reaction_fans_delighted(s: &ReactionText) {
    covered!("sx01b_1 reaction fans delighted");
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match s.variant {
            2 => s.club_manager(0xa2922c),
            1 => s.club_manager(if matches!(t, 96 | 63) { 0xa29070 } else { 0xa29158 }),
            _ => s.club_manager(0xa28fd0),
        }
        return;
    }
    match s.variant {
        1 if t == 82 => s.club_manager_subject(0xa2987c),
        1 if act => s.club_manager_subject(0xa28d28),
        1 if t == s.variant => s.club_subject(0xa29748),
        1 => s.club_manager_subject(0xa29948),
        2 if act => s.club_manager_subject(0xa29490),
        2 if t == 1 => s.club_subject(0xa2954c),
        2 => s.club_manager_subject(0xa2967c),
        _ => s.club_manager_subject(if act || t == 1 { 0xa2930c } else { 0xa293cc }),
    }
}

/// The supporters pleased (degree 3 and any other): pleased at the manager's backing (a high-profile supporter on
/// actions), a telephone poll (65..84 percent, the confidence % 20 + 65) backing his criticism, pleased at his
/// silence; topic 1 is the representative. Otherwise: the caution, the confidence, reassured.
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73dc30..0x73df7e).
pub(crate) unsafe fn reaction_fans_pleased(s: &ReactionText) {
    covered!("sx01b_1 reaction fans pleased");
    let a = s.args();
    let t = s.topic;
    let act = actions_topic(t);
    if !s.about_subject() {
        match s.variant {
            2 => s.club_manager(0xa2897c),
            1 => s.club_manager(if matches!(t, 96 | 63) { 0xa287b4 } else { 0xa28898 }),
            _ => s.club_manager(0xa28718),
        }
        return;
    }
    match s.variant {
        1 if act => s.club_manager_subject(if t == 82 { 0xa2987c } else { 0xa28d28 }),
        1 if t == s.variant => s.club_subject(0xa28dec),
        1 => s.club_manager_subject(0xa28f28),
        2 if act => {
            covered!("sx01b_1 reaction fans poll");
            reaction!(0xa28a48, a.cg, a.cg, 0x64, 0x64, s.confidence % 20 + 65, a.cs, a.mgr, a.subj);
        }
        2 if t == 1 => s.club_subject(0xa28b20),
        2 => s.club_manager_subject(0xa28c5c),
        _ if act => s.club_manager_subject(0xa2930c),
        _ if t == 1 => s.club_subject(0xa28b20),
        _ => s.club_manager_subject(0xa293cc),
    }
}

/// The body's paragraph by who reacts and the topic: the board (outburst, settling, praise, then by degree) or the
/// supporters (future, outburst, performances, praise with a subject, then by degree).
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x739f32..0x73e048).
pub(crate) unsafe fn reaction_body(s: &ReactionText) {
    let t = s.topic;
    if s.board {
        match t {
            33 => reaction_board_outburst(s),
            1 => reaction_board_unsettled(s),
            _ if praise_topic(t) => reaction_board_praise(s),
            _ => match s.degree {
                9 => reaction_board_surprised(s),
                5 => reaction_board_distressed(s),
                4 => reaction_board_disappointed(s),
                2 => reaction_board_delighted(s),
                _ => reaction_board_pleased(s),
            },
        }
        return;
    }
    match t {
        3 => reaction_fans_future(s),
        33 => reaction_fans_outburst(s),
        60 | 61 => reaction_fans_performances(s),
        _ if praise_topic(t) && !s.subject.is_null() => reaction_fans_praise(s),
        _ => match s.degree {
            9 => reaction_fans_surprised(s),
            5 => reaction_fans_distressed(s),
            4 => reaction_fans_disappointed(s),
            2 => reaction_fans_delighted(s),
            _ => reaction_fans_pleased(s),
        },
    }
}

/// The body's closing paragraph, appended after the text is set (topic 1 has none). The supporters: above 9,500
/// confidence when delighted, the manager's respect (by the confidence % 3); below 4,250 when distressed or
/// disappointed, his failing to impress. The board, to the manager on screen with expectations set: the first
/// expectation bit by the confidence % 3: a place in Europe (bit 0x1, unless the club's nation is outside Europe), the
/// championship (0x2), promotion (0x4), the play-offs (0x8), survival (0x10), continental or European glory (0x400);
/// % 3 = 2 is always "raise their expectations".
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73e048..0x73e349).
pub(crate) unsafe fn reaction_follow_up(s: &ReactionText, out: usize) {
    let a = s.args();
    let c = s.confidence % 3;
    if !s.board && s.topic != 1 {
        if s.confidence > 9500 && s.degree == 2 {
            reaction!(by_confidence(c, 0xa27fd0, 0xa2806c, 0xa2811c), 0x64, a.mgr_s);
        } else if s.confidence < 4250 && matches!(s.degree, 5 | 4) {
            reaction!(by_confidence(c, 0xa27e28, 0xa27eb0, 0xa27f34), 0x64, a.mgr_s);
        } else {
            return;
        }
        covered!("sx01b_1 reaction fans follow-up");
    } else {
        let e = s.expectations;
        if e == 0 || !s.on_screen || s.topic == 1 {
            return;
        }
        let nation = s.club.nation();
        let outside_europe = !nation.is_null() && nation.continent() != 0 && read_u32(nation.continent()) != read_u32(KEY_CONTINENT_EUROPE);
        let raise = 0xa27db0;
        let tpl = if e & 1 != 0 && !outside_europe {
            by_confidence(c, 0xa27cd8, 0xa27d38, raise)
        } else if e & 2 != 0 {
            by_confidence(c, 0xa27c14, 0xa27c6c, raise)
        } else if e & 4 != 0 {
            by_confidence(c, 0xa27b50, 0xa27ba8, raise)
        } else if e & 8 != 0 {
            by_confidence(c, 0xa27a80, 0xa27ae8, raise)
        } else if e & 0x10 != 0 {
            by_confidence(c, 0xa279bc, 0xa27a18, raise)
        } else if e & 0x400 != 0 {
            match c {
                0 if outside_europe => 0xa27874,
                0 => 0xa27814,
                1 if outside_europe => 0xa27948,
                1 => 0xa278d8,
                _ => raise,
            }
        } else {
            return;
        };
        covered!("sx01b_1 reaction board expectations");
        reaction!(tpl);
    }
    exe::text_append_string(out, BACKGROUND_TEXT);
}

/// The template by the confidence % 3: 0, 1, or any other remainder (as the original's `sub`/`dec` chain).
fn by_confidence(c: i32, zero: usize, one: usize, other: usize) -> usize {
    match c {
        0 => zero,
        1 => one,
        _ => other,
    }
}

/// The links: the club (0x7d5, name and short name), the subject (0x7d7, both names) and the chairman (his name) when
/// their name ids still match (`database_staff_matches_names`), and the manager (both names).
/// Original: part of `Media::write_board_fans_reaction_news_text` (0x73e349..0x73e464).
pub(crate) unsafe fn reaction_links(s: &ReactionText, out: usize) {
    covered!("sx01b_1 reaction links");
    let c = s.club;
    exe::text_add_link(out, c.name_addr() as *const u8, 0x7d5, c.addr());
    exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c.addr());
    let matches = |p: Staff, ids: [u32; 3]| exe::database_staff_matches_names(p.addr(), ids[0], ids[1], ids[2], 0usize) != 0;
    if !s.subject.is_null() && matches(s.subject, s.subject_ids) {
        exe::text_add_link(out, s.subject_name.as_ptr(), 0x7d7, s.subject.addr());
        exe::text_add_link(out, s.subject_short.as_ptr(), 0x7d7, s.subject.addr());
    }
    if !s.chairman.is_null() && matches(s.chairman, s.chairman_ids) {
        exe::text_add_link(out, s.chairman_name.as_ptr(), 0x7d7, s.chairman.addr());
    }
    if !s.manager.is_null() && matches(s.manager, s.manager_ids) {
        exe::text_add_link(out, s.manager_name.as_ptr(), 0x7d7, s.manager.addr());
        exe::text_add_link(out, s.manager_short.as_ptr(), 0x7d7, s.manager.addr()); // sx01b_1: the manager's links
    }
}

/// `Media::write_board_fans_reaction_news_text(news, body, out)`: writes the headline or the body of the story where
/// the board, its chairman or the supporters react to the manager's answer to the media (news 0xbe0, posted by
/// `adjust_board_confidence` and `adjust_supporter_confidence`) into `out`, with the links.
///
/// The slots and the names (`reaction_text_read`, `reaction_text_names`). The text is written into the shared buffer
/// `BACKGROUND_TEXT` (0xde1f64) and set on `out`: the headline (`reaction_headline_board`, `reaction_headline_fans`),
/// or the body (`reaction_body`, one piece per topic or degree) and its closing paragraph (`reaction_follow_up`,
/// appended). Then the links (`reaction_links`).
///
/// Defects of the original: a board headline or body on a praise topic (0x28..0x3b) with a degree other than 2..5 or 9
/// writes nothing, so the Text gets whatever the shared buffer held (unreachable: the posters only use 2, 3, 4, 5, 9);
/// without the chairman's name the board's "words of wisdom" body names "Marc Vaughan"; the supporters' praise piece
/// reads the subject's player sheet without a null check when it is one of the club's disliked staff or when the
/// club's reputation is 4,250 or more, and another read of the sheet's work rate is compared and ignored; the
/// chairman's liking (slot 14) and the value flag are read and not used.
/// Original: `Media::write_board_fans_reaction_news_text` (0x738d50, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_board_fans_reaction_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut s = Box::new(reaction_text_read(news));
    reaction_text_names(&mut s);
    if s.board {
        covered!("sx01b_1 reaction board");
    }
    if body == 0 {
        if s.board {
            reaction_headline_board(&s);
        } else {
            reaction_headline_fans(&s);
        }
        exe::text_set_string(out, BACKGROUND_TEXT as *const u8);
    } else {
        covered!("sx01b_1 reaction body");
        reaction_body(&s);
        exe::text_set_string(out, BACKGROUND_TEXT as *const u8);
        reaction_follow_up(&s, out);
    }
    reaction_links(&s, out);
}

/// Comparer for `write_board_fans_reaction_news_text` (`compare_text_writer`): the text and the links side by side.
pub(crate) unsafe extern "thiscall" fn compare_write_board_fans_reaction_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, body, out), |t| write_board_fans_reaction_news_text(this, news, body, t), || {
        let slot = |k: u32| exe::newsitem_get_slot(news, k) as i32;
        format!("write_board_fans_reaction_news_text(board {}, topic {}, variant {}, degree {}, body {})", slot(0), slot(1), slot(2), slot(5), body)
    });
}
