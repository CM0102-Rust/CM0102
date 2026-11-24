// ---------- cx15_6: the squad numbers request news, its text and the European registration news (squad_manager.cpp) ----------

/// Sends a club's manager the news 0x18 asking for the squad numbers list (slots: 0 the club id, 1 = 0, the list not
/// yet submitted). `this` is not read.
///
/// SEH frame only for the local news item (0xe0 bytes), no destructor on the normal path. Called by
/// `assign_league_squad_numbers` for a human-managed club. `exe::new_newsitem` (lea ecx, local; push 0; push 0x18 ->
/// thiscall, ret 8), `exe::newsitem_set_slot` (thiscall, ret 8), `exe::news_send_to_club_manager` (ecx = [`G_NEWS`];
/// 2 pushes -> thiscall, ret 8).
/// Original: `SQUAD_MANAGER::send_squad_numbers_request_news` (0x85aec0, squad_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn send_squad_numbers_request_news(_this: usize, club: usize) {
    covered!("cx15_6 squad numbers request");
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x18, 0);
    exe::newsitem_set_slot(p, 0, read_u32(club));
    exe::newsitem_set_slot(p, 1, 0);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Writes the text of the squad numbers request (news 0x18) into Text `text`: the club is club index slot 0 (its short
/// name and short-name gender). The body (`body` not 0) is "<club> have been requested to submit their squad numbers
/// list for the forthcoming season." (0xa82c34) and, when slot 1 is set, the line "The list has been submitted."
/// (0xa82c14) appended (`exe::text_append_string`); the headline is "<club> squad numbers to be decided" (0xa82bd4).
/// Each line goes to the global text buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`, cdecl, the gender twice first), then
/// `exe::text_set_string` and the club link (0x7d5). `this` is not read.
///
/// Called by the news text dispatcher (`NewsItem::write_text`, type 0x18).
/// Original: `SQUAD_MANAGER::write_squad_numbers_news_text` (0x85af40, squad_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_squad_numbers_news_text(_this: usize, news: usize, body: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = club_at(exe::newsitem_get_slot(news, 0) as i32 as isize);
    let submitted = exe::newsitem_get_slot(news, 1);
    let g = c.short_gender_name() as i8 as i32;
    let name = c.addr() + club::SHORT_NAME;
    lang_sprintf(0xde1f64, if body != 0 { 0xa82c34 } else { 0xa82bd4 }, g, g, name);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, c);
    if body != 0 && submitted != 0 {
        covered!("cx15_6 squad numbers list submitted");
        lang_sprintf(0xde1f64, 0xa82c14);
        exe::text_append_string(text, 0xde1f64);
    }
}

/// The number of players of a club registered for Europe: the club's squad (`exe::database_collect_club_squad`, 50
/// people, cdecl) counted where the person is real (id below [`G_STAFF_COUNT`], signed), has a registration mark
/// (`squad_selected_for()`) and the mark has neither bit 3 nor bit 4 (0x18), in a byte.
unsafe fn euro_registered_count(club: usize) -> i8 {
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    let mut count = 0i8;
    for &p in squad.iter() {
        let s = Staff(p);
        if p != 0 && (read_i32(p) < read_i32(G_STAFF_COUNT)) && s.squad_selected_for() != 0 && s.squad_selected_for() & 0x18 == 0 {
            count = count.wrapping_add(1);
        }
    }
    count
}

/// Sends a club's manager the European registration news 0x19 of registration `step` in competition `comp_id`, unless
/// the club already has 25 registered players (`euro_registered_count`) and the step is not the third. The places:
/// 25, or at the third step the registered count plus two, at most 25. An Intertoto club
/// (`KEY_CLUB_COMP_INTER_TOTO_CUP`) is told about the UEFA Cup (`KEY_CLUB_COMP_UEFA_CUP`). Slots: 0 the club id, 1 the
/// competition id, 2 the step (signed byte), 3 = 0, 4 the places, 5 the registered count, 6 = 0 (signed bytes). `this`
/// is not read.
///
/// SEH frame only for the local news item (0xe0 bytes), no destructor on the normal path. Called by
/// `process_euro_registration` for a human-managed club. The news calls as in `send_squad_numbers_request_news`.
/// Original: `SQUAD_MANAGER::send_euro_registration_news` (0x85b020, squad_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn send_euro_registration_news(_this: usize, club: usize, comp_id: u32, step: u32) {
    let step = step as u8 as i8;
    let count = euro_registered_count(club);
    if count == 25 && step != 3 {
        return;
    }
    let places = if step == 3 { (count as i32 + 2).min(25) as i8 } else { 25 };
    covered!("cx15_6 euro registration news");
    let comp = if comp_id == read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP) { read_u32(KEY_CLUB_COMP_UEFA_CUP) } else { comp_id };
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x19, 0);
    exe::newsitem_set_slot(p, 0, read_u32(club));
    exe::newsitem_set_slot(p, 1, comp);
    exe::newsitem_set_slot(p, 2, step as i32 as u32);
    exe::newsitem_set_slot(p, 3, 0);
    exe::newsitem_set_slot(p, 4, places as i32 as u32);
    exe::newsitem_set_slot(p, 5, count as i32 as u32);
    exe::newsitem_set_slot(p, 6, 0);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Comparers: the senders deliver news (they cannot run twice), so only the original runs; the text side by side
/// (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_send_squad_numbers_request_news(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, club)
}

pub(crate) unsafe extern "thiscall" fn compare_write_squad_numbers_news_text(this: usize, news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, news, body, text), |t| write_squad_numbers_news_text(this, news, body, t),
        || format!("write_squad_numbers_news_text(news {:#x}, body {})", news, body));
}

pub(crate) unsafe extern "thiscall" fn compare_send_euro_registration_news(this: usize, club: usize, comp_id: u32, step: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%)(this, club, comp_id, step)
}
