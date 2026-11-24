// --- sx16_3: the staff note reminders (notes.cpp: the reminder news, its text, the reasons valid for a person) ---

/// The diary reminder news type (5011).
const NEWS_DIARY_REMINDER: u32 = 0x1393;
/// The body of the diary reminder by the note's reason 1..4 (promising youngster, transfer prospect, contract renewal,
/// squad member), then any other reason; `true` when the language format takes the 100-byte name width before the name.
const DIARY_REMINDER_BODY: [(usize, bool); 5] = [(0xa69e4c, true), (0xa69d9c, false), (0xa69d24, true), (0xa69f14, true), (0xa69cbc, false)];

/// Queues the diary reminder news of one note for the slot's human manager: a type 5011 news item (`NewsItem::NewsItem`)
/// with the slots 0 the person's id, 1 the reason (sign-extended), 2..4 the ids of his first, second and common names
/// (`names::ID` of each name record), delivered to the slot's manager (the staff row `G_STAFF_COUNT` − slot − 1,
/// `News::deliver_to_manager(item, manager, 0)`). Nothing for a slot outside 0..15, without the note or with a person
/// id outside the staff table.
/// Original: `Notes::post_reminder_news` (0x795e20, notes.cpp, thiscall, ret 8; `this` unused; SEH frame for the local
/// news item, without a destructor).
pub(crate) unsafe extern "thiscall" fn notes_post_reminder_news(_this: usize, slot: i32, note: usize) {
    let mut item = [0u8; 0xe0];
    let news = item.as_mut_ptr() as usize;
    exe::new_newsitem(news, NEWS_DIARY_REMINDER, 0);
    if !(0..0x10).contains(&slot) || note == 0 {
        return;
    }
    let id = read_i32(note);
    let count = read_i32(G_STAFF_COUNT);
    if id < 0 || id > count - 1 {
        return;
    }
    covered!("sx16_3 diary reminder news");
    let manager = Staff(read_usize(G_STAFF_TABLE) + ((count - slot - 1) as usize) * staff::SIZE);
    exe::newsitem_set_slot(news, 0, id as u32);
    exe::newsitem_set_slot(news, 1, read_i8(note + NOTE_REASON) as i32 as u32);
    let person = Staff(read_usize(G_STAFF_TABLE) + id as usize * staff::SIZE);
    exe::newsitem_set_slot(news, 2, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(news, 3, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(news, 4, read_u32(person.common_name() + names::ID));
    exe::news_deliver_to_manager(read_usize(G_NEWS), news, manager, 0);
}

/// Comparer for `notes_post_reminder_news`: it delivers a news item, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_notes_post_reminder_news(this: usize, slot: i32, note: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, i32, usize)>(%%I0%%)(this, slot, note)
}

/// Writes the headline or the body of a diary reminder news item into `out`.
///
/// The slots: 0 the person's id (outside the staff table, 0), 1 the reason, 2..4 the name ids. The name
/// (`database_format_name_from_ids`, 300 bytes, style 1, the person's id). Headline: the reason label in style 0
/// ("Reminder about ...", `Notes::get_reason_label` through the exe into the shared buffer 0xde1f64, 1000 bytes).
/// Body: the club secretary's sentence by the reason (`DIARY_REMINDER_BODY`, `langlib_sprintf` into 0xde1f64), the text
/// set (`Text::set_string`) and, when the person still has those names (`database_staff_matches_names`), the name
/// linked to him (0x7db).
/// Original: `notes_write_diary_reminder_news` (0x795f50, notes.cpp, thiscall(notes, news, body, out), ret 0xc).
pub(crate) unsafe extern "thiscall" fn notes_write_diary_reminder_news(this: usize, news: usize, body: u32, out: usize) {
    let mut id = exe::newsitem_get_slot(news, 0) as i32;
    let reason = exe::newsitem_get_slot(news, 1) as u8;
    let first = exe::newsitem_get_slot(news, 2);
    let second = exe::newsitem_get_slot(news, 3);
    let common = exe::newsitem_get_slot(news, 4);
    if id < 0 || id > read_i32(G_STAFF_COUNT) - 1 {
        id = 0;
    }
    let person = read_usize(G_STAFF_TABLE) + id as usize * staff::SIZE;
    let mut name = [0u8; 0x12c];
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0x12c, 1, id as u32);
    if body == 0 {
        covered!("sx16_3 diary reminder headline");
        exe::notes_get_reason_label(this, GUI_SCRATCH_TEXT, 0x3e8, reason as u32, 0);
        exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
        return;
    }
    covered!("sx16_3 diary reminder body");
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (text, width) = DIARY_REMINDER_BODY[((reason as i8 as i32 - 1) as u32).min(4) as usize];
    if width {
        format(GUI_SCRATCH_TEXT, text, 0x64, name.as_ptr());
    } else {
        format(GUI_SCRATCH_TEXT, text, name.as_ptr());
    }
    exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
    if exe::database_staff_matches_names(person, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7db, person);
    }
}

/// Comparer for `notes_write_diary_reminder_news`: the text, its links and the generator (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_notes_write_diary_reminder_news(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, body, out), |t| notes_write_diary_reminder_news(this, news, body, t),
        || format!("notes_write_diary_reminder_news(reason {}, body {})", exe::newsitem_get_slot(news, 1) as i8, body));
}

/// Whether a note reason applies to a staff member for a manager: 0 (none) and anything above 4 always;
/// 1 (promising youngster) a player (`staff::PLAYER`) under 24 (`staff::AGE`, a signed byte); 2 (transfer prospect) a
/// player not at the manager's club; 3 (contract renewal) anyone at the manager's club; 4 (squad member) a player at
/// the manager's club. "At the club" is both club jobs equal and set.
/// Original: `notes_is_reason_valid_for_staff` (0x796100, notes.cpp, thiscall(notes, manager, person, reason), ret 0xc;
/// `this` unused).
pub(crate) unsafe extern "thiscall" fn notes_is_reason_valid_for_staff(_this: usize, manager: usize, person: usize, reason: u32) -> u8 {
    let (m, p) = (Staff(manager), Staff(person));
    let same_club = !p.club_job().is_null() && m.club_job() == p.club_job();
    let player = !p.player().is_null();
    (match reason as i8 {
        1 => player && read_i8(person + staff::AGE) < 0x18,
        2 => player && !same_club,
        3 => same_club,
        4 => player && same_club,
        _ => true,
    }) as u8
}

/// Comparer for `notes_is_reason_valid_for_staff`: the same answer.
pub(crate) unsafe extern "thiscall" fn compare_notes_is_reason_valid_for_staff(this: usize, manager: usize, person: usize, reason: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8>(%%I2%%)(this, manager, person, reason);
    let n = notes_is_reason_valid_for_staff(this, manager, person, reason);
    SCOREBOARD.record(r == n, || format!("notes_is_reason_valid_for_staff(reason {}): original={} rust={}", reason as i8, r, n));
    r
}
