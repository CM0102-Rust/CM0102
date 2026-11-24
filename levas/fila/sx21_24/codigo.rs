// ---------- sx21_24: the transfer deadline news of France, Italy and Norway (france_rules.cpp, italy_rules.cpp, norway_rules.cpp) ----------

/// Posts a country's transfer deadline news item (0xfb2) to the managers of a nation's clubs: slot 0 the rule number
/// (+9), 1 the kind (the deadline state), 2 the window number (+1 of the window), 3 and 4 the window's date in `year`
/// (`exe::transfer_window_get_date`: day of the year and year), 5 `nation_id`; sent with
/// `exe::news_send_to_nation_club_managers` to `nation`.
pub(crate) unsafe fn post_country_deadline_news(this: usize, window: usize, kind: u32, year: u32, nation_id: u32, nation: usize) {
    let mut date = [0u8; 8];
    exe::transfer_window_get_date(window, date.as_mut_ptr() as usize, year);
    let (build, set) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    build(n, 0xfb2, 0);
    set(n, 0, read_u8(this + 9) as i8 as u32);
    set(n, 1, kind as u8 as i8 as u32);
    set(n, 2, read_u8(window + 1) as i8 as u32);
    set(n, 3, read_i16(date.as_ptr() as usize) as u32);
    set(n, 4, read_u16(date.as_ptr() as usize + 2) as u32);
    set(n, 5, nation_id);
    exe::news_send_to_nation_club_managers(read_usize(G_NEWS), n, nation);
}

/// Posts France's transfer deadline news, except for the restricted window (number 1).
///
/// `FRANCE_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS`
/// of France, from the transfer manager's window news: nothing for the window numbered 1; otherwise
/// `post_country_deadline_news` with the nation of `KEY_NATION_FRANCE` (id and record in `G_NATION_TABLE`).
/// Original: `FRANCE_RULES::post_deadline_news` (0x5c1a70, france_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn france_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    if read_u8(window + 1) == 1 {
        return;
    }
    covered!("sx21_24 france deadline news");
    let id = read_u32(KEY_NATION_FRANCE);
    post_country_deadline_news(this, window, kind, year, id, read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE)));
}

/// Comparer for `france_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_france_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I0%%)(this, window, kind, year)
}

/// Posts Italy's transfer deadline news, except the lifted notice (kind 3) of the third window (number 2).
///
/// `ITALY_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS` of
/// Italy: nothing for kind 3 in the window numbered 2; otherwise `post_country_deadline_news` with the rule set's nation
/// (`transfer_rules::NATION_ID`, its id in slot 5).
/// Original: `ITALY_RULES::post_deadline_news` (0x661a30, italy_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn italy_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    if read_u8(window + 1) == 2 && kind as u8 == 3 {
        return;
    }
    covered!("sx21_24 italy deadline news");
    let nation = read_usize(this + transfer_rules::NATION_ID);
    post_country_deadline_news(this, window, kind, year, read_u32(nation), nation);
}

/// Comparer for `italy_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_italy_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, window, kind, year)
}

/// Posts Norway's transfer deadline news, except the lifted notices (kind 3).
///
/// `NORWAY_RULES::post_deadline_news(window, kind, year)`, the vtable slot `transfer_rules_vtable::POST_DEADLINE_NEWS`
/// of Norway: nothing for kind 3; otherwise `post_country_deadline_news` with the rule set's nation
/// (`transfer_rules::NATION_ID`, its id in slot 5).
/// Original: `NORWAY_RULES::post_deadline_news` (0x7947d0, norway_rules.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn norway_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    if kind as u8 == 3 {
        return;
    }
    covered!("sx21_24 norway deadline news");
    let nation = read_usize(this + transfer_rules::NATION_ID);
    post_country_deadline_news(this, window, kind, year, read_u32(nation), nation);
}

/// Comparer for `norway_rules_post_deadline_news`: only the original (it posts the news).
pub(crate) unsafe extern "thiscall" fn compare_norway_rules_post_deadline_news(this: usize, window: usize, kind: u32, year: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%)(this, window, kind, year)
}
