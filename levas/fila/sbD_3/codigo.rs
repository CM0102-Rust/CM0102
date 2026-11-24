// ---------- sbD_3: the texts of the player-and-manager stories: thanks, turnaround and first-team chance (european_cup.cpp) ----------

/// Writes a headline about one player (slots 0..3) into `out`: his short name (style 5) in `template`, set into the
/// Text and linked to him (0x7da, when he still bears the names).
pub(crate) unsafe fn player_headline_text(news: usize, text: usize, out: &mut [u8], template: usize) {
    let player = NewsPerson::read(news, 0);
    let mut name = [0u8; 0x190];
    let short = player.format(&mut name, 5);
    story_lang!(out.as_mut_ptr(), template, short);
    exe::text_set_string(text, out.as_ptr());
    player.link(text, &[short], 0x7da);
}

/// Writes the headline or body of the story that a player credits his manager for his form (news 0xbde): slots 0..3
/// the player, 5 his fame (1 a local newspaper, otherwise a national one) and 6..9 the manager. Body: the template
/// 0x9c654c with (100, 0, 0, 100), the player's full name, the newspaper ("local"/"national", 0x9c666c/0x9c6640) and
/// the manager's full name, the player's name linked to him (0x7da, when the names still match). Headline:
/// "<player> thanks boss" (0x9c651c). The text goes into a frame buffer.
/// Original: `news_thanks_manager_text` (0x589480, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_thanks_manager_text(news: usize, body: u32, text: usize) {
    let mut out = [0u8; 0x1378];
    if body == 0 {
        player_headline_text(news, text, &mut out, 0x9c651c); // "<player> thanks boss"
        return;
    }
    let player = NewsPerson::read(news, 0);
    exe::newsitem_get_slot(news, 4);
    let fame = exe::newsitem_get_slot(news, 5);
    let manager = NewsPerson::read(news, 6);
    let (mut player_buf, mut manager_buf, mut paper_buf) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x34]);
    let p = player.format(&mut player_buf, 1);
    let m = manager.format(&mut manager_buf, 1);
    let paper = story_newspaper(&mut paper_buf, fame == 1, 0x9c666c, 0x9c6640);
    covered!("sbD_3 thanks manager text body");
    story_lang!(out.as_mut_ptr(), 0x9c654c, 0x64, 0, 0, 0x64, p, paper, m);
    exe::text_set_string(text, out.as_ptr());
    player.link(text, &[p], 0x7da);
}

/// Writes the headline or body of the story that a player is ecstatic at the win after a winless run (news 0xbe1):
/// slots 0..3 the player, 4 the club, 5 his fame, 6..9 the manager, 10 the opponent and 11 the length of the run.
/// Body: the run as a number (`FN_CRT_SPRINTF` "%d"), the player's and the manager's full names (the manager's is not
/// used) and the newspaper ("A local newspaper"/"A national newspaper", 0x9c6880/0x9c6844), then the template
/// 0x9c66d4 with the club's gender, short-name gender and the opponent's short-name gender (each twice), the
/// newspaper, the club's name, the player, the club's short name, the run and the opponent's short name. Links: the
/// player (0x7da, when the names still match), the club's short and long names to the club and the opponent's long
/// name to the opponent (0x7d5). Headline: "<player> ecstatic at turn of fortune" (0x9c6694). Frame buffers.
/// Original: `news_ecstatic_at_turnaround_text` (0x589880, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_ecstatic_at_turnaround_text(news: usize, body: u32, text: usize) {
    let mut out = [0u8; 0x1378];
    if body == 0 {
        player_headline_text(news, text, &mut out, 0x9c6694); // "<player> ecstatic at turn of fortune"
        return;
    }
    let player = NewsPerson::read(news, 0);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let fame = exe::newsitem_get_slot(news, 5);
    let manager = NewsPerson::read(news, 6);
    let opponent = club_at(exe::newsitem_get_slot(news, 10) as i32 as isize);
    let run = exe::newsitem_get_slot(news, 11);
    let mut number = [0u8; 0x64];
    story_crt!(number.as_mut_ptr(), 0x98806c, run); // "%d"
    let (mut player_buf, mut manager_buf, mut paper_buf) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x34]);
    let p = player.format(&mut player_buf, 1);
    manager.format(&mut manager_buf, 1);
    let paper = story_newspaper(&mut paper_buf, fame == 1, 0x9c6880, 0x9c6844);
    let g = club.gender_name() as i8 as i32;
    let sg = club.short_gender_name() as i8 as i32;
    let og = opponent.short_gender_name() as i8 as i32;
    let club_short = club.addr() + club::SHORT_NAME;
    covered!("sbD_3 ecstatic at turnaround text body");
    story_lang!(out.as_mut_ptr(), 0x9c66d4, g, g, sg, sg, og, og, paper, club.name_addr(), p, club_short, number.as_ptr(), opponent.addr() + club::SHORT_NAME);
    exe::text_set_string(text, out.as_ptr());
    player.link(text, &[p], 0x7da);
    exe::text_add_link(text, club_short as *const u8, 0x7d5, club);
    exe::text_add_link(text, club.name_addr() as *const u8, 0x7d5, club);
    exe::text_add_link(text, opponent.name_addr() as *const u8, 0x7d5, opponent);
}

/// Writes the headline or body of the story that a player is pleased with his first-team chance (news 0xbe2): slots
/// 0..3 the player, 4 the club, 5 his fame, 6..9 the manager and 10 the drawn wording (0 at smaller clubs). Body, with
/// the player's and the manager's full names and the newspaper ("a local newspaper"/"a national newspaper",
/// 0x9c6b8c/0x9c6b50): a drawn wording, 0x9c6a1c with the club's gender twice, its name, the player, the newspaper and
/// the manager; otherwise 0x9c68f0 with the club's gender and short-name gender (each twice), its name, the player,
/// the newspaper and its short name. Links: the player (0x7da, when the names still match) and the club's short and
/// long names to the club (0x7d5). Headline: "<player> pleased to have his chance" (0x9c68b4). Frame buffers.
/// Original: `news_pleased_with_chance_text` (0x589d60, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_pleased_with_chance_text(news: usize, body: u32, text: usize) {
    let mut out = [0u8; 0x1378];
    if body == 0 {
        player_headline_text(news, text, &mut out, 0x9c68b4); // "<player> pleased to have his chance"
        return;
    }
    let player = NewsPerson::read(news, 0);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let fame = exe::newsitem_get_slot(news, 5);
    let manager = NewsPerson::read(news, 6);
    let wording = exe::newsitem_get_slot(news, 10);
    let (mut player_buf, mut manager_buf, mut paper_buf) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x34]);
    let p = player.format(&mut player_buf, 1);
    let m = manager.format(&mut manager_buf, 1);
    let paper = story_newspaper(&mut paper_buf, fame == 1, 0x9c6b8c, 0x9c6b50);
    let g = club.gender_name() as i8 as i32;
    let club_short = club.addr() + club::SHORT_NAME;
    covered!("sbD_3 pleased with chance text body");
    if wording != 0 {
        story_lang!(out.as_mut_ptr(), 0x9c6a1c, g, g, club.name_addr(), p, paper, m);
    } else {
        let sg = club.short_gender_name() as i8 as i32;
        story_lang!(out.as_mut_ptr(), 0x9c68f0, g, g, sg, sg, club.name_addr(), p, paper, club_short);
    }
    exe::text_set_string(text, out.as_ptr());
    player.link(text, &[p], 0x7da);
    exe::text_add_link(text, club_short as *const u8, 0x7d5, club);
    exe::text_add_link(text, club.name_addr() as *const u8, 0x7d5, club);
}

/// Comparers of the text writers: `compare_text_writer` (the original into the Text, the copy into a local one).
pub(crate) unsafe extern "C" fn compare_news_thanks_manager_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(news, body, text), |t| news_thanks_manager_text(news, body, t), || format!("news_thanks_manager_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_ecstatic_at_turnaround_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(news, body, text), |t| news_ecstatic_at_turnaround_text(news, body, t), || format!("news_ecstatic_at_turnaround_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_pleased_with_chance_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(news, body, text), |t| news_pleased_with_chance_text(news, body, t), || format!("news_pleased_with_chance_text(body {})", body));
}
