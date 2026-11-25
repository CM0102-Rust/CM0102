// ---------- rx07_10: the text of the news that a player seeks assurances (contract_manager.cpp) ----------

/// The sentence on the player's squad role in the seeks-assurances news, by the role (slot 7, 1 to 7): when the two
/// transfer statuses (slots 8 and 9) are the same or none of their bits below applies.
pub(crate) const ASSURANCES_ROLE_TEXTS: [usize; 7] = [0x9adbd8, 0x9adb9c, 0x9adb58, 0x9adb24, 0x9adad8, 0x9ada90, 0x9ada5c];

/// The same sentence when the new status (slot 9) has bit 2, bit 1, or the old status (slot 8) has bit 1, bit 2.
pub(crate) const ASSURANCES_STATUS_TEXTS: [[usize; 7]; 4] = [
    [0x9ae5c0, 0x9ae568, 0x9ae508, 0x9ae4b8, 0x9ae450, 0x9ae3ec, 0x9ae39c],
    [0x9ae358, 0x9ae2fc, 0x9ae29c, 0x9ae248, 0x9ae1dc, 0x9ae174, 0x9ae120],
    [0x9ae0d4, 0x9ae074, 0x9ae00c, 0x9adfb4, 0x9adf44, 0x9aded8, 0x9ade80],
    [0x9ade38, 0x9adddc, 0x9add78, 0x9add24, 0x9adcb8, 0x9adc50, 0x9adc00],
];

/// The template of the role sentence: by the transfer statuses (old `before`, new `now`) and the role 1 to 7; `None`
/// for any other role (the buffer then keeps the previous sentence).
pub(crate) fn assurances_role_text(before: u32, now: u32, role: u32) -> Option<usize> {
    let k = role.wrapping_sub(1) as usize;
    if k > 6 {
        return None;
    }
    let table = if before == now {
        &ASSURANCES_ROLE_TEXTS
    } else if now & 2 != 0 {
        &ASSURANCES_STATUS_TEXTS[0]
    } else if now & 1 != 0 {
        &ASSURANCES_STATUS_TEXTS[1]
    } else if before & 1 != 0 {
        &ASSURANCES_STATUS_TEXTS[2]
    } else if before & 2 != 0 {
        &ASSURANCES_STATUS_TEXTS[3]
    } else {
        &ASSURANCES_ROLE_TEXTS
    };
    Some(table[k])
}

/// Rule: the text of news item 0xbc1 (a player seeks assurances): slots 0 the club, 2 the person, 3 to 5 his names, 6
/// whether there is more to say, 7 his squad role, 8 and 9 the old and new transfer statuses (-1 none), 10 whether
/// to add the closing sentence. The body (name mode 1): "<name> is seeking immediate assurances regarding his future."
/// (0x9ae604); with slot 6 set and a new status (not -1), the role sentence (`assurances_role_text`; a role outside 1
/// to 7 appends the buffer again) and, with slot 10, the closing sentence (new status bit 1: 0x9ada0c; role 1:
/// 0x9ad9e4; roles 2 and 3: 0x9ad8e8; else 0x9ad990), each appended (`exe::text_append_string`); the club linked
/// (name). The headline (name mode 5): "<name> seeks assurances" (0x9ad8b8). Then the person's link (0x7d7) when the
/// names still match.
/// Original: `CONTRACT_MANAGER::write_seeks_assurances_news_text` (0x4d9a90, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_seeks_assurances_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5) = (slot(0), slot(2), slot(3), slot(4), slot(5));
    let (more, role, before, now, closing) = (slot(6), slot(7), slot(8), slot(9), slot(10));
    let _age = slot(11); // read and not used
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        news_text_person_name(s3, s4, s5, &mut name, 1, person);
        fmt(NEWS_TEXT_BUFFER, 0x9ae604, np); // "<name> is seeking immediate assurances regarding his future."
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if more != 0 && now as i32 != -1 {
            covered!("rx07_10 assurances role sentence");
            if let Some(t) = assurances_role_text(before, now, role) {
                fmt(NEWS_TEXT_BUFFER, t);
            }
            exe::text_append_string(out, NEWS_TEXT_BUFFER);
            if closing != 0 {
                let t = if now & 1 != 0 { 0x9ada0c } else if role == 1 { 0x9ad9e4 } else if role == 2 || role == 3 { 0x9ad8e8 } else { 0x9ad990 };
                fmt(NEWS_TEXT_BUFFER, t);
                exe::text_append_string(out, NEWS_TEXT_BUFFER);
            }
        }
        let c = club_at(club as i32 as isize);
        exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
    } else {
        news_text_person_name(s3, s4, s5, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0x9ad8b8, np); // "<name> seeks assurances"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    covered!("rx07_10 assurances text");
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_seeks_assurances_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_seeks_assurances_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_seeks_assurances_news_text(this, news, full, t),
        || format!("write_seeks_assurances_news_text(news {:#x}, full {})", news, full));
}
