// ---------- cx15_8: the European qualifiers news and its text (uefa_seeding.cpp) ----------

/// The first slot of each list in the European qualifiers news 0x2f: the Champions League from 0, the UEFA Cup from 10
/// and the Intertoto from 20 (each list ends with −1).
const QUALIFIERS_LIST_SLOTS: [i8; 3] = [0, 10, 20];

/// Slot of the season year in the European qualifiers news.
const QUALIFIERS_YEAR_SLOT: i8 = 0x1e;

/// Slot of the nation id in the European qualifiers news.
const QUALIFIERS_NATION_SLOT: i8 = 0x1f;

/// The clubs with a European place: every club of the club table except the last 2 × nation count records (the
/// national teams), with a nation, a European competition (`club::CONTINENTAL_COMP_ID` not −1), the nation selected for
/// the game (`league_selected()` not 0) and in Europe (`continent()` with the id of `KEY_CONTINENT_EUROPE`), in table
/// order (the count is a 16-bit word, as in the original).
unsafe fn european_qualified_clubs() -> Vec<usize> {
    let mut list = Vec::new();
    let count = read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT);
    let europe = read_u32(KEY_CONTINENT_EUROPE);
    for i in 0..count.max(0) {
        let c = club_at(i as isize);
        let nation = c.nation();
        if nation.is_null() || c.continental_comp_id() == u32::MAX || nation.league_selected() == 0 {
            continue;
        }
        let continent = nation.continent();
        if continent != 0 && read_u32(continent) == europe {
            list.push(c.addr());
        }
    }
    list
}

/// Ends a nation's three lists with −1 (at the next free slot of each) and sends the news to the managers of the
/// nation's clubs (`exe::news_send_to_nation_club_managers`, ecx = [`G_NEWS`]; push nation; push news -> thiscall, ret 8).
unsafe fn send_nation_qualifiers(news: usize, nation: usize, next: [i8; 3]) {
    for slot in next {
        exe::newsitem_set_slot(news, slot, u32::MAX);
    }
    exe::news_send_to_nation_club_managers(read_usize(G_NEWS), news, nation);
}

/// Posts the European qualifiers news (0x2f): one news item, built once (the season year in `QUALIFIERS_YEAR_SLOT`),
/// sent nation by nation to the managers of that nation's clubs. The clubs with a European place
/// (`european_qualified_clubs`) are sorted by `uefa_compare_qualified_clubs` (`exe::crt_qsort`, 4-byte entries); on each
/// new nation the previous one's lists are closed and sent (`send_nation_qualifiers`), the counters go back to the
/// first slots and the nation id goes into `QUALIFIERS_NATION_SLOT`; each club's id goes to the next slot of the list of
/// its competition (the Champions League, the UEFA Cup and the Intertoto keys; a club of another competition is not
/// listed). The last nation is sent at the end. The same item is reused, so a nation's slots beyond its −1 keep the
/// previous nation's clubs (never read). The club list is a 600-entry stack array in the original, without a bound.
/// `this` is not read.
///
/// SEH frame only for the local news item. Called by `update_season` when the seeding is redone.
/// Original: `uefa_post_qualifiers_news` (0x905f00, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_post_qualifiers_news(_this: usize) {
    let mut clubs = european_qualified_clubs();
    exe::crt_qsort(clubs.as_mut_ptr() as usize, clubs.len() as i16 as i32 as u32, 4, 0x906410); // uefa_compare_qualified_clubs
    let mut n = [0u8; 0xe0];
    let news = n.as_mut_ptr() as usize;
    exe::new_newsitem(news, 0x2f, 0);
    exe::newsitem_set_slot(news, QUALIFIERS_YEAR_SLOT, read_u16(G_CURRENT_DATE_YEAR) as u32);
    let keys = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP];
    let mut next = QUALIFIERS_LIST_SLOTS;
    let mut nation = 0usize;
    for &c in clubs.iter() {
        let c = Club(c);
        if c.nation().addr() != nation {
            if nation != 0 {
                covered!("cx15_8 qualifiers of a nation sent");
                send_nation_qualifiers(news, nation, next);
                next = QUALIFIERS_LIST_SLOTS;
            }
            nation = c.nation().addr();
            exe::newsitem_set_slot(news, QUALIFIERS_NATION_SLOT, read_u32(nation));
        }
        if let Some(k) = keys.iter().position(|&key| c.continental_comp_id() == read_u32(key)) {
            exe::newsitem_set_slot(news, next[k], read_u32(c.addr()));
            next[k] = next[k].wrapping_add(1);
        }
    }
    if nation != 0 {
        send_nation_qualifiers(news, nation, next);
    }
}

/// Writes the text of the European qualifiers news (0x2f) into Text `text`. `this` is not read.
///
/// The headline (`body` 0): "<year>/<year + 1 mod 100> European qualifiers decided" (0xaa1b50, the year slot as an
/// unsigned word). The body: "The following teams have qualified for European competitions next season." (0xaa1b04);
/// then for each competition (the Champions League, the UEFA Cup and the Intertoto keys, from their first slot
/// `QUALIFIERS_LIST_SLOTS`) with a first club (slot not −1): "<competition>: " (0xaa1ae4, the competition's short name
/// and gender, link 0x7d0) and up to ten clubs, each "<club>, " (0xaa1ab4) or, when the next slot is −1, "<club>.\n"
/// (0xaa1acc), with the club link (0x7d5). Each line goes to the global text buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`,
/// cdecl, the gender twice first), the heading by `exe::text_set_string`, the rest by `exe::text_append_string`.
///
/// Called by the news text dispatcher (`NewsItem::write_text`, type 0x2f).
/// Original: `uefa_write_qualifiers_news_text` (0x906130, uefa_seeding.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn uefa_write_qualifiers_news_text(_this: usize, news: usize, body: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if body == 0 {
        let year = exe::newsitem_get_slot(news, QUALIFIERS_YEAR_SLOT as u32) & 0xffff;
        lang_sprintf(0xde1f64, 0xaa1b50, year, (year as i32 + 1) % 100);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        return;
    }
    let keys = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP].map(|k| read_u32(k));
    lang_sprintf(0xde1f64, 0xaa1b04);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    for (id, first) in keys.into_iter().zip(QUALIFIERS_LIST_SLOTS) {
        let slot = |k: i8| exe::newsitem_get_slot(news, k as u8 as u32);
        if slot(first) == u32::MAX {
            continue;
        }
        let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)));
        let g = comp.short_gender_name() as i8 as i32;
        lang_sprintf(0xde1f64, 0xaa1ae4, g, g, comp.short_name_addr());
        exe::text_append_string(text, 0xde1f64);
        exe::text_add_link(text, comp.short_name_addr() as *const u8, 0x7d0, comp);
        let mut k = first;
        while (k as i32) < first as i32 + 10 {
            if slot(k) == u32::MAX {
                break;
            }
            let last = slot(k.wrapping_add(1)) == u32::MAX;
            let c = club_at(slot(k) as i32 as isize);
            let cg = c.short_gender_name() as i8 as i32;
            let name = c.addr() + club::SHORT_NAME;
            if last {
                covered!("cx15_8 last qualified club");
            }
            lang_sprintf(0xde1f64, if last { 0xaa1acc } else { 0xaa1ab4 }, cg, cg, name);
            exe::text_append_string(text, 0xde1f64);
            exe::text_add_link(text, name as *const u8, 0x7d5, c);
            k = k.wrapping_add(1);
        }
    }
}

/// Comparer for `uefa_post_qualifiers_news`: it sends news (it cannot run twice), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_uefa_post_qualifiers_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparer for `uefa_write_qualifiers_news_text`: side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_uefa_write_qualifiers_news_text(this: usize, news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(this, news, body, text), |t| uefa_write_qualifiers_news_text(this, news, body, t),
        || format!("uefa_write_qualifiers_news_text(news {:#x}, body {})", news, body));
}
