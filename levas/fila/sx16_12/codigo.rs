// --- sx16_12: the youth promotion news text (player_regen.cpp 0x7a8eb0) ---

/// The staff verdict sentence on a promoted youngster, by the verdict slot (12) and, for 1, the coach slot (13):
/// (verdict, coach slot 2?, with the verdict flag slot 7 set, without). Anything else uses the last pair.
const YOUTH_VERDICT_TEXTS: [((u32, bool), usize, usize); 3] = [
    ((2, false), 0xa6c3dc, 0xa6c348),
    ((1, true), 0xa6c29c, 0xa6c204),
    ((1, false), 0xa6c160, 0xa6c0cc),
];

/// Writes the headline or the body of the news that a youth player joins the reserves or the B team.
///
/// The slots: 0 the player's id, 1..3 his name ids, 4 the club's id, 6 the B team flag (from the youth academy), 7 the
/// verdict flag, 8 the staff member giving the verdict (−1 none), 9..11 his name ids, 12 the verdict, 13 the coach
/// kind (slots 5 and 10.. not used). The player's name (`database_format_name_from_ids`, 400 bytes; style 5 in the
/// headline, 1 in the body).
///
/// Headline: "<name> promoted from youth academy" (slot 6 set) or "promoted to reserve squad"; the text and the
/// player's link (0x7d7) when he still has those names.
/// Body: the club (`club::SIZE` × slot 4) and `club_get_linked_b_team(club, &scratch, 1)`; the sentence with the
/// club's short gender twice, the name and the club's short name: with slot 6, 0xa6c63c with a linked B team else
/// 0xa6c5a4; without, 0xa6c514 / 0xa6c480. The text set. With a staff member (slot 8 above −1): his name (400 bytes,
/// style 1), the verdict sentence (`YOUTH_VERDICT_TEXTS`) appended (`Text::append_string`) and his link when he still
/// has those names. Then, with a club (slot 4 above −1), the club's link (0x7d5, its name), and the player's link.
/// Defect of the original: the club record is read (slot 4 × 581 from the club table) before the test of slot 4, so a
/// news item without a club reads the record before the table.
/// Original: `player_regen_write_youth_promotion_news_text` (0x7a8eb0, player_regen.cpp, stdcall(news, body, out), ret
/// 0xc; the news writer table calls it as a method of the regen object, ecx unused).
pub(crate) unsafe extern "stdcall" fn player_regen_write_youth_promotion_news_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let id = slot(0);
    let names = [slot(1), slot(2), slot(3)];
    let club_id = slot(4) as i32;
    let academy = slot(6);
    let flag = slot(7);
    let staff_id = slot(8) as i32;
    let staff_names = [slot(9), slot(10), slot(11)];
    let verdict = slot(12);
    let coach = slot(13);
    let mut name = [0u8; 0x190];
    let staff_row = |k: u32| read_usize(G_STAFF_TABLE).wrapping_add((k as usize).wrapping_mul(staff::SIZE));
    let link_person = |name: &[u8; 0x190], person: usize, ids: [u32; 3]| {
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
        }
    };
    if body == 0 {
        covered!("sx16_12 youth promotion headline");
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 5, id);
        format(GUI_SCRATCH_TEXT, if academy != 0 { 0xa6c708 } else { 0xa6c6cc }, name.as_ptr()); // promoted from youth academy / to reserve squad
        exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
        link_person(&name, staff_row(id), names);
        return;
    }
    covered!("sx16_12 youth promotion body");
    exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 1, id);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((club_id as isize * club::SIZE as isize) as usize));
    let mut scratch = 0u32;
    let b_team = exe::club_get_linked_b_team(club, &mut scratch, 1) != 0;
    let text = match (academy != 0, b_team) {
        (true, true) => 0xa6c63c,
        (true, false) => 0xa6c5a4,
        (false, true) => 0xa6c514,
        (false, false) => 0xa6c480,
    };
    let g = club.short_gender_name() as i8 as i32;
    format(GUI_SCRATCH_TEXT, text, g, g, name.as_ptr(), club.addr() + club::SHORT_NAME); // sx16_12: the promotion sentence
    exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
    if staff_id > -1 {
        covered!("sx16_12 staff verdict");
        let mut staff_name = [0u8; 0x190];
        exe::database_format_name_from_ids(staff_names[0], staff_names[1], staff_names[2], staff_name.as_mut_ptr(), 0x190, 1, staff_id as u32);
        let key = (verdict, verdict == 1 && coach == 2);
        let (_, with_flag, without) = YOUTH_VERDICT_TEXTS.iter().copied().find(|e| e.0 == key).unwrap_or(((0, false), 0xa6c054, 0xa6bfec));
        format(GUI_SCRATCH_TEXT, if flag != 0 { with_flag } else { without }, staff_name.as_ptr());
        exe::text_append_string(out, GUI_SCRATCH_TEXT);
        link_person(&staff_name, staff_row(staff_id as u32), staff_names);
    }
    if club_id > -1 {
        exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);
    }
    link_person(&name, staff_row(id), names);
}

/// Comparer for `player_regen_write_youth_promotion_news_text`: the text, its links and the generator
/// (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_write_youth_promotion_news_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| player_regen_write_youth_promotion_news_text(news, body, t),
        || format!("player_regen_write_youth_promotion_news_text(body {})", body));
}
