// ---------- cx04_12: the texts of the scout watching, transfer system and insurance payout stories (european_cup.cpp) ----------

/// Links a formatted name to the person of the given id (`exe::text_add_link`, 0x7d7) when he still bears the names
/// (`exe::database_staff_matches_names`); as `news_text_link_person`, for a name buffer of any size.
unsafe fn link_story_person(text: usize, name: usize, id: u32, names: [u32; 3]) {
    let person = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(person, names[0], names[1], names[2], 0usize) != 0 {
        exe::text_add_link(text, name as *const u8, 0x7d7, person.addr());
    }
}

/// Writes the headline or body of the scout watching story (news 0xfbd): slots 0..3 the player (id and name ids), 4
/// the watching club, 6 the wording. Body: the player's full name (`exe::database_format_name_from_ids`, 0x190 bytes,
/// style 1) and, by the wording, 0 the template 0x9c4b04, 1 0x9c4b9c, others 0x9c4c50, each with the club's short name
/// and its gender (`FN_LANGLIB_SPRINTF` into the shared buffer 0xde1f64, with 100 among the arguments as the original
/// passes it); then the name's link to the player (0x7d7, when the names still match, `exe::database_staff_matches_names`)
/// and the club's (0x7d5). Headline: "<player> watched" (0x9c4ad4) with the short name (style 5) and the player's link.
/// Original: `news_write_scout_watching_text` (0x587370, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_write_scout_watching_text(news: usize, body: u32, text: usize) {
    let slot = |k: i8| exe::newsitem_get_slot(news, k as u32);
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let (id, names) = (slot(0), [slot(1), slot(2), slot(3)]);
    let club_id = slot(4);
    slot(5);
    let wording = slot(6);
    if body != 0 {
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 1, id);
        let club = club_at(club_id as i32 as isize);
        let g = club.short_gender_name() as i8 as i32;
        let format = match wording {
            0 => 0x9c4b04,
            1 => 0x9c4b9c,
            _ => 0x9c4c50,
        };
        covered!("cx04_12 scout watching text body");
        story_lang!(PRAISE_TEXT_BUFFER, format, g, g, 0x64, club.addr() + club::SHORT_NAME, nm);
        exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
        link_story_person(text, nm, id, names);
        exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    } else {
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 5, id);
        story_lang!(PRAISE_TEXT_BUFFER, 0x9c4ad4, nm); // "<player> watched"
        exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
        link_story_person(text, nm, id, names);
    }
}

/// Writes the headline or body of the transfer system story (news 0xfc1, slot 0 the kind): kind not 0, the FIFA
/// transfer system change (headline 0x9c7034, body 0x9c71b4); kind 0, the clubs rushing to renew contracts (headline
/// 0x9c7010, body 0x9c705c). Formatted (`FN_LANGLIB_SPRINTF`) into a frame buffer and set into the Text.
/// Original: `news_transfer_system_change_text` (0x58a900, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_transfer_system_change_text(news: usize, body: u32, text: usize) {
    let change = exe::newsitem_get_slot(news, 0) != 0;
    let format = match (body != 0, change) {
        (true, true) => 0x9c71b4,
        (true, false) => 0x9c705c,
        (false, true) => 0x9c7034,   // "FIFA announce transfer system change"
        (false, false) => 0x9c7010, // "Clubs desperate to renew contracts"
    };
    covered!("cx04_12 transfer system text");
    let mut buf = [0u8; 0x1388];
    story_lang!(buf.as_mut_ptr() as usize, format);
    exe::text_set_string(text, buf.as_ptr());
}

/// Writes the headline or body of the insurance payout story (news 0xfc2): slots 0..3 the player, 4 the amount, 5 the
/// club. Body: the amount for display (`exe::cash_get_display_amount` with 6, `exe::cash_format_money` into 100 bytes),
/// the player's full name (0x190, style 1) and the template 0x9c7334 with the club's short name and gender, 100, the
/// amount and the name; links: the name to the player (0x7d7, when the names match) and, for a club id above −1, the
/// club's long name (`club::NAME`) to the club (0x7d5). Headline: the template 0x9c72cc with the club's short name and
/// the player's short name (style 5), and the player's link. Defect of the original: the name is formatted with room
/// for 0x190 bytes into a frame buffer that the amount's text follows after 100 bytes (harmless for real names).
/// Original: `news_insurance_payout_text` (0x58aa60, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_insurance_payout_text(news: usize, body: u32, text: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (id, names) = (slot(0), [slot(1), slot(2), slot(3)]);
    let amount = slot(4);
    let club_id = slot(5) as i32;
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let mut out = [0u8; 0x1400];
    let o = out.as_mut_ptr() as usize;
    if body != 0 {
        covered!("cx04_12 insurance payout text body");
        let mut money = [0u8; 0x64];
        exe::cash_format_money(money.as_mut_ptr() as usize, 0x64, exe::cash_get_display_amount(&amount as *const u32, 6), 6);
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 1, id);
        story_lang!(o, 0x9c7334, g, g, 0x64, club.addr() + club::SHORT_NAME, money.as_ptr() as usize, nm);
        exe::text_set_string(text, out.as_ptr());
        link_story_person(text, nm, id, names);
        if club_id > -1 {
            exe::text_add_link(text, (club.addr() + club::NAME) as *const u8, 0x7d5, club.addr());
        }
    } else {
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 5, id);
        story_lang!(o, 0x9c72cc, g, g, club.addr() + club::SHORT_NAME, nm);
        exe::text_set_string(text, out.as_ptr());
        link_story_person(text, nm, id, names);
    }
}

/// Comparers of the text writers: `compare_text_writer` (the original into the Text, the copy into a local one).
pub(crate) unsafe extern "C" fn compare_news_write_scout_watching_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(news, body, text), |t| news_write_scout_watching_text(news, body, t), || format!("news_write_scout_watching_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_transfer_system_change_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(news, body, text), |t| news_transfer_system_change_text(news, body, t), || format!("news_transfer_system_change_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_insurance_payout_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(news, body, text), |t| news_insurance_payout_text(news, body, t), || format!("news_insurance_payout_text(body {})", body));
}
