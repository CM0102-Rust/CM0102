// ---------- sx21_25: the transfer deadline news of Belgium and Germany (belgium_rules.cpp, germany_rules.cpp) ----------

/// Posts Belgium's transfer deadline news, except the lifted notices (kind 3) of the later windows.
///
/// `BELGIUM_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS`
/// of Belgium: nothing for kind 3 in a window numbered other than 0; otherwise `post_country_deadline_news` with the id
/// of `KEY_NATION_BELGIUM` in slot 5, sent to the rule set's nation (`transfer_rules::NATION_ID`).
/// Original: `BELGIUM_RULES::post_deadline_news` (0x424fa0, belgium_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn belgium_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    if read_u8(window + 1) != 0 && kind as u8 == 3 {
        return;
    }
    covered!("sx21_25 belgium deadline news");
    post_country_deadline_news(this, window, kind, year, read_u32(KEY_NATION_BELGIUM), read_usize(this + transfer_rules::NATION_ID));
}

/// Comparer for `belgium_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_belgium_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I0%%)(this, window, kind, year)
}

/// Posts Germany's transfer deadline news, except the lifted notices (kind 3) of the later windows.
///
/// `GERMANY_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS`
/// of Germany: nothing for kind 3 in a window numbered other than 0; otherwise `post_country_deadline_news` with the id
/// of `KEY_NATION_GERMANY` in slot 5, sent to the rule set's nation (`transfer_rules::NATION_ID`).
/// Original: `GERMANY_RULES::post_deadline_news` (0x5e0310, germany_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn germany_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    if read_u8(window + 1) != 0 && kind as u8 == 3 {
        return;
    }
    covered!("sx21_25 germany deadline news");
    post_country_deadline_news(this, window, kind, year, read_u32(KEY_NATION_GERMANY), read_usize(this + transfer_rules::NATION_ID));
}

/// Comparer for `germany_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_germany_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, window, kind, year)
}
