// ---------- sbD_4: the texts of the award credit and FA fine stories (european_cup.cpp) ----------

/// Writes the headline or body of the story that a player credits his manager for an award (news 0xbe3): slots 0..3
/// the player, 4 the club, 5 his fame, 6..9 the manager and 10 the award (a 101-byte record of
/// [`G_STAFF_COMP_TABLE`]). Body, with the player's and the manager's full names and the newspaper ("a local
/// newspaper"/"a national newspaper", 0x9c6b8c/0x9c6b50): the template 0x9c6c0c with the club's gender twice, 0 twice,
/// the award's name gender twice, the club's name, the player, the newspaper, the manager and the award's name; links:
/// the player (0x7da, when the names still match) and the club's name to the club (0x7d5). Headline: "<player>
/// acknowledges his manager's contribution" (0x9c6bc0). Frame buffers.
/// Original: `news_credits_manager_for_award_text` (0x58a210, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_credits_manager_for_award_text(news: usize, body: u32, text: usize) {
    let mut out = [0u8; 0x1378];
    if body == 0 {
        player_headline_text(news, text, &mut out, 0x9c6bc0); // "<player> acknowledges his manager's contribution"
        return;
    }
    let player = NewsPerson::read(news, 0);
    let club = club_at(exe::newsitem_get_slot(news, 4) as i32 as isize);
    let fame = exe::newsitem_get_slot(news, 5);
    let manager = NewsPerson::read(news, 6);
    let award = read_usize(G_STAFF_COMP_TABLE).wrapping_add((exe::newsitem_get_slot(news, 10) as usize).wrapping_mul(0x65));
    let (mut player_buf, mut manager_buf, mut paper_buf) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x34]);
    let p = player.format(&mut player_buf, 1);
    let m = manager.format(&mut manager_buf, 1);
    let paper = story_newspaper(&mut paper_buf, fame == 1, 0x9c6b8c, 0x9c6b50);
    let g = club.gender_name() as i8 as i32;
    let ag = read_u8(award + 0x37) as i8 as i32; // the award's name gender
    covered!("sbD_4 credits manager for award text body");
    story_lang!(out.as_mut_ptr(), 0x9c6c0c, g, g, 0, 0, ag, ag, club.name_addr(), p, paper, m, award + 4); // +4: the award's name
    exe::text_set_string(text, out.as_ptr());
    player.link(text, &[p], 0x7da);
    exe::text_add_link(text, club.name_addr() as *const u8, 0x7d5, club);
}

/// Writes the headline or body of the story of the FA fining a club for its disciplinary record (news 0x178c): slots
/// 0 the club, 1..4 the manager, 5 the nation, 6 the kind and 7 the amount. Always first the nation's adjective
/// (`exe::nation_get_nationality_adjective`, 0x78 bytes, style 9) and the amount for display
/// (`exe::cash_get_display_amount` with 6, `exe::cash_format_money` into 500 bytes).
/// - Body: the manager's full name is formatted but not used; kind 0, the template 0x9c6ee0 with the nation's, the
///   club's and the club's short-name genders (each twice), the adjective, the club's name, the amount and its short
///   name; another kind, 0x9c6dac with the club's, the nation's and the short-name genders (each twice), the club's
///   name, the amount, the adjective and the short name. Links: the club's short and long names to the club (0x7d5).
/// - Headline: "<nationality> FA impose fine on <club>" (0x9c6d48) with the nation's and the short-name genders
///   (each twice), the nation's nationality and the club's short name; link: the club's name to the club.
/// Frame buffers.
/// Original: `news_fa_disciplinary_fine_text` (0x58a660, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_fa_disciplinary_fine_text(news: usize, body: u32, text: usize) {
    let club = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let manager = NewsPerson::read(news, 1);
    let nation_id = exe::newsitem_get_slot(news, 5);
    let kind = exe::newsitem_get_slot(news, 6);
    let amount = exe::newsitem_get_slot(news, 7);
    let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((nation_id as usize).wrapping_mul(nation::SIZE)));
    let (mut adjective, mut money) = ([0u8; 0x78], [0u8; 0x1f4]);
    exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, 9);
    exe::cash_format_money(money.as_mut_ptr() as usize, 0x1f4, exe::cash_get_display_amount(&amount as *const u32, 6), 6);
    let ng = nation.gender_name() as i8 as i32;
    let sg = club.short_gender_name() as i8 as i32;
    let club_short = club.addr() + club::SHORT_NAME;
    let mut out = [0u8; 0x1378];
    let o = out.as_mut_ptr();
    if body != 0 {
        let mut name = [0u8; 0x190];
        manager.format(&mut name, 1);
        let g = club.gender_name() as i8 as i32;
        covered!("sbD_4 fa disciplinary fine text body");
        if kind == 0 {
            story_lang!(o, 0x9c6ee0, ng, ng, g, g, sg, sg, adjective.as_ptr(), club.name_addr(), money.as_ptr(), club_short);
        } else {
            story_lang!(o, 0x9c6dac, g, g, ng, ng, sg, sg, club.name_addr(), money.as_ptr(), adjective.as_ptr(), club_short);
        }
        exe::text_set_string(text, out.as_ptr());
        exe::text_add_link(text, club_short as *const u8, 0x7d5, club);
        exe::text_add_link(text, club.name_addr() as *const u8, 0x7d5, club);
    } else {
        story_lang!(o, 0x9c6d48, ng, ng, sg, sg, nation.addr() + nation::NATIONALITY, club_short); // "<nationality> FA impose fine on <club>"
        exe::text_set_string(text, out.as_ptr());
        exe::text_add_link(text, club.name_addr() as *const u8, 0x7d5, club);
    }
}

/// Comparers of the text writers: `compare_text_writer` (the original into the Text, the copy into a local one).
pub(crate) unsafe extern "C" fn compare_news_credits_manager_for_award_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(news, body, text), |t| news_credits_manager_for_award_text(news, body, t), || format!("news_credits_manager_for_award_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_fa_disciplinary_fine_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(news, body, text), |t| news_fa_disciplinary_fine_text(news, body, t), || format!("news_fa_disciplinary_fine_text(body {})", body));
}
