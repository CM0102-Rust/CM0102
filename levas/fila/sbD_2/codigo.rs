// ---------- sbD_2: the texts of the unhappy-with-training, feud settled and training report stories (european_cup.cpp) ----------

/// Writes the headline or body of the story that a player is unhappy with training (news 0xbcd; slots 0..3 the
/// player, 5 the drawn wording): the body 0x9c5ccc ("<player> is unhappy with his training schedule at the club.")
/// for a wording other than 0, otherwise 0x9c5c5c, with his full name; the headline "<player> unhappy with training"
/// (0x9c5c20) with his short name (`player_story_text`).
/// Original: `news_unhappy_with_training_text` (0x5886d0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_unhappy_with_training_text(news: usize, body: u32, text: usize) {
    covered!("sbD_2 unhappy with training text");
    player_story_text(news, body, text, |wording| if wording != 0 { 0x9c5ccc } else { 0x9c5c5c }, 0x9c5c20);
}

/// Writes the headline or body of the story that two feuding players settled their differences (news 0xbca; slots
/// 0..3 the player, 4..7 the other): both names, full (style 1) in the body 0x9c5d8c or short (style 5) in the headline
/// "<player> and <other> settle differences" (0x9c5d28), the other's formatted first; the text into the shared buffer
/// [`PRAISE_TEXT_BUFFER`]; each name linked to its person (0x7da, when the names still match).
/// Original: `news_feud_settled_text` (0x5889e0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_feud_settled_text(news: usize, body: u32, text: usize) {
    let player = NewsPerson::read(news, 0);
    let other = NewsPerson::read(news, 4);
    let style = if body != 0 { 1 } else { 5 };
    let (mut player_buf, mut other_buf) = ([0u8; 0x190], [0u8; 0x190]);
    let b = other.format(&mut other_buf, style);
    let a = player.format(&mut player_buf, style);
    let template = if body != 0 { 0x9c5d8c } else { 0x9c5d28 };
    covered!("sbD_2 feud settled text");
    story_lang!(PRAISE_TEXT_BUFFER, template, a, b);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    player.link(text, &[a], 0x7da);
    other.link(text, &[b], 0x7da);
}

/// Writes the headline or body of a coach's training report on a player (news 0x138d): slots 0..3 the player, 4..7
/// the reporting coach, 8 the assessment and 9 the kind (999 praise, otherwise criticism), into the shared buffer
/// [`PRAISE_TEXT_BUFFER`].
/// - Body, with both full names (style 1, 0x190 bytes, the coach's formatted first): praise, by the assessment, 0
///   0x9c5ea8 (100, coach, player), 1 0x9c5f64 (100, player, coach), otherwise 0x9c601c (coach, player); criticism,
///   by the assessment, 0 0x9c6454, 1 0x9c63b0, 2 0x9c62e0, 3 0x9c6234, 4 0x9c6154, otherwise 0x9c60d4, each with
///   (100, coach, player). Links: each name to its person (0x7da, when the names still match).
/// - Headline: when the player's id plus the assessment is even, "Training report from <coach>" (0x9c5e70) with
///   the coach's short name, linked to him; otherwise "<player> training report" (0x9c5e44) with the player's short
///   name, linked to him.
/// Original: `news_training_report_text` (0x588f00, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_training_report_text(news: usize, body: u32, text: usize) {
    let player = NewsPerson::read(news, 0);
    let coach = NewsPerson::read(news, 4);
    let assessment = exe::newsitem_get_slot(news, 8);
    let praise = exe::newsitem_get_slot(news, 9) == 0x3e7;
    let out = PRAISE_TEXT_BUFFER;
    let (mut coach_buf, mut player_buf) = ([0u8; 0x190], [0u8; 0x190]);
    if body != 0 {
        let c = coach.format(&mut coach_buf, 1);
        let p = player.format(&mut player_buf, 1);
        covered!("sbD_2 training report text body");
        if praise {
            match assessment {
                0 => story_lang!(out, 0x9c5ea8, 0x64, c, p),
                1 => story_lang!(out, 0x9c5f64, 0x64, p, c),
                _ => story_lang!(out, 0x9c601c, c, p),
            }
        } else {
            let template = match assessment {
                0 => 0x9c6454,
                1 => 0x9c63b0,
                2 => 0x9c62e0,
                3 => 0x9c6234,
                4 => 0x9c6154,
                _ => 0x9c60d4,
            };
            story_lang!(out, template, 0x64, c, p);
        }
        exe::text_set_string(text, out as *const u8);
        player.link(text, &[p], 0x7da);
        coach.link(text, &[c], 0x7da);
    } else if (assessment as i32).wrapping_add(player.id as i32) % 2 == 0 {
        let c = coach.format(&mut coach_buf, 5);
        story_lang!(out, 0x9c5e70, c); // "Training report from <coach>"
        exe::text_set_string(text, out as *const u8);
        coach.link(text, &[c], 0x7da);
    } else {
        let p = player.format(&mut player_buf, 5);
        story_lang!(out, 0x9c5e44, p); // "<player> training report"
        exe::text_set_string(text, out as *const u8);
        player.link(text, &[p], 0x7da);
    }
}

/// Comparers of the text writers: `compare_text_writer` (the original into the Text, the copy into a local one).
pub(crate) unsafe extern "C" fn compare_news_unhappy_with_training_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(news, body, text), |t| news_unhappy_with_training_text(news, body, t), || format!("news_unhappy_with_training_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_feud_settled_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(news, body, text), |t| news_feud_settled_text(news, body, t), || format!("news_feud_settled_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_training_report_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(news, body, text), |t| news_training_report_text(news, body, t), || format!("news_training_report_text(body {})", body));
}
