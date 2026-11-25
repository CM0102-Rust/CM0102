// --- rx06_4: the text of the scout search news (player_search.cpp): player_search_write_search_started_news_text
// (0x7b7840) ---

/// The kind of a search as its news carries it (nation, region byte, competition, youth flag), in the order the
/// writers test them.
#[derive(Clone, Copy)]
enum ScoutSearchSubject {
    Nation(usize),
    Region(u8),
    Competition(usize),
    Youth,
}

/// What a scout search news item carries after the search's slot: the scout's name reference (ids +0..+8, staff
/// index +0xc, date +0x10), then the nation, the region, the competition and the youth flag.
struct ScoutSearchNews {
    slot: u8,
    scout: [u8; 0x18],
    subject: Option<ScoutSearchSubject>,
}

/// Reads a scout search news item (`NewsItem::get_slot` and `read_staff_name_ref_from_news`).
unsafe fn scout_search_news(news: usize) -> ScoutSearchNews {
    let mut scout = [0u8; 0x18];
    exe::date_date_54a110(scout.as_mut_ptr() as usize + 0x10);
    let slot = exe::newsitem_get_slot(news, 0) as u8;
    let mut n: u8 = 1;
    exe::read_staff_name_ref_from_news(news, &mut n, scout.as_mut_ptr());
    let mut next = || {
        let v = exe::newsitem_get_slot(news, n as u32);
        n = n.wrapping_add(1);
        v
    };
    let nation = next() as i32;
    let region = next() as u8 as i8;
    let comp = next() as i32;
    let youth = next();
    let subject = if nation > -1 {
        Some(ScoutSearchSubject::Nation(nation_by_id(nation as usize)))
    } else if region > -1 {
        Some(ScoutSearchSubject::Region(region as u8))
    } else if comp > -1 {
        Some(ScoutSearchSubject::Competition(read_usize(G_CLUB_COMP_TABLE) + comp as usize * 0x6b)) // 0x6b: the competition record
    } else if youth != 0 {
        Some(ScoutSearchSubject::Youth)
    } else {
        None
    };
    ScoutSearchNews { slot, scout, subject }
}

impl ScoutSearchNews {
    /// The scout the news names, when his staff index is valid and still matches the stored names and date.
    unsafe fn scout(&self) -> Option<Staff> {
        let r = self.scout.as_ptr() as usize;
        let idx = read_i32(r + 0xc);
        if idx <= -1 {
            return None;
        }
        let person = staff_at(idx as isize);
        let same = exe::database_staff_matches_names(person, read_u32(r), read_u32(r + 4), read_u32(r + 8), r + 0x10) != 0;
        same.then_some(person)
    }

    /// The scout's name in style 1 (`database_format_name_from_ids`).
    unsafe fn scout_name(&self, out: &mut [u8; 0xc8]) {
        let r = self.scout.as_ptr() as usize;
        exe::database_format_name_from_ids(read_u32(r), read_u32(r + 4), read_u32(r + 8), out.as_mut_ptr(), 0xc8, 1, read_u32(r + 0xc));
    }

    /// The link of the search (kind 0x7e4, the slot) on `text`, when the search in that slot (`SCOUT_MANAGER::get_search`
    /// of the current manager) is still this scout's.
    unsafe fn link_search(&self, out: usize, scout: Staff, text: &[u8]) {
        let search = exe::scout_manager_get_search(read_usize(G_SCOUT_MANAGER), self.slot as u32, 0usize);
        if get_scout(search) == scout.addr() {
            exe::text_add_link(out, text.as_ptr(), 0x7e4, self.slot as i8 as i32 as usize);
        }
    }
}

/// The name of what a search looks at, into `out` (`nation_get_nationality_adjective`, `get_region_name` or the
/// competition name), with its gender (regions 0x64); the youth search has neither.
unsafe fn scout_search_subject_name(subject: ScoutSearchSubject, out: &mut [u8; 0xc8]) -> i32 {
    match subject {
        ScoutSearchSubject::Nation(nation) => {
            exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 1);
            read_i8(nation + nation::GENDER_NAME) as i32
        }
        ScoutSearchSubject::Region(region) => {
            exe::get_region_name(region as u32, out.as_mut_ptr() as usize, 0xc8, 1);
            0x64
        }
        ScoutSearchSubject::Competition(comp) => {
            let name = core::ffi::CStr::from_ptr((comp + competition::NAME) as *const core::ffi::c_char).to_bytes_with_nul();
            out[..name.len()].copy_from_slice(name);
            read_i8(comp + competition::GENDER_NAME) as i32
        }
        ScoutSearchSubject::Youth => 0,
    }
}

/// Writes the headline or the body of the news "scout search started" (news 0x1389: slot 0 the search's slot, then the
/// scout's name reference, the nation, the region, the competition and the youth flag). Without a news item or a text:
/// the player_search.cpp box of line 0x12a5 and 0. The text is cleared first. The headline: "<nationality> search
/// started", "<region> search started", "<competition> search started" (the subject linked to the search when the scout
/// is still the search's) or "Youth search started" (no link). The body names the scout (style 1, linked when he is
/// still that person) and the subject the same way ("search for youth players" for the youth search, also linked).
/// A news item with no subject: the box of line 0x139f (headline) or 0x133e (body) and 0; otherwise 1.
/// Original: `player_search_write_search_started_news_text` (0x7b7840, player_search.cpp, cdecl).
pub(crate) unsafe extern "C" fn player_search_write_search_started_news_text(news: usize, full: u32, out: usize) -> u32 {
    if news == 0 || out == 0 {
        show_error_at(0xa701b0, 0x12a5);
        return 0;
    }
    let item = scout_search_news(news);
    exe::text_clear(out as *mut usize);
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let mut what = [0u8; 0xc8];
    if full == 0 {
        let Some(subject) = item.subject else {
            show_error_at(0xa701b0, 0x139f);
            return 0;
        };
        let g = scout_search_subject_name(subject, &mut what);
        let w = what.as_ptr() as usize;
        match subject {
            ScoutSearchSubject::Nation(_) => fmt(buf, 0xa70304, g, g, w), // "{}<%s - nationality>{} search started" (rx06_4)
            ScoutSearchSubject::Region(_) => fmt(buf, 0xa702dc, g, g, w), // "{}<%s - regionality>{} search started"
            ScoutSearchSubject::Competition(_) => fmt(buf, 0xa702b4, g, g, w), // "{}<%s - competition>{} search started"
            ScoutSearchSubject::Youth => {
                fmt(buf, 0xa7029c); // "Youth search started"
                exe::text_set_string(out, buf as *const u8);
                return 1;
            }
        };
        exe::text_set_string(out, buf as *const u8);
        if let Some(scout) = item.scout() {
            item.link_search(out, scout, &what);
        }
        return 1;
    }
    let mut name = [0u8; 0xc8];
    item.scout_name(&mut name);
    let Some(subject) = item.subject else {
        show_error_at(0xa701b0, 0x133e);
        return 0;
    };
    covered!("rx06_4 search started body");
    let g = scout_search_subject_name(subject, &mut what);
    let (n, w) = (name.as_ptr() as usize, what.as_ptr() as usize);
    match subject {
        ScoutSearchSubject::Nation(_) => fmt(buf, 0xa7048c, g, g, n, w),
        ScoutSearchSubject::Region(_) => fmt(buf, 0xa70420, g, g, n, w),
        ScoutSearchSubject::Competition(_) => fmt(buf, 0xa703b4, g, g, n, w),
        ScoutSearchSubject::Youth => {
            story_lang!(w, 0xa70398); // "search for youth players"
            fmt(buf, 0xa7032c, n, w)
        }
    };
    exe::text_set_string(out, buf as *const u8);
    if let Some(scout) = item.scout() {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, scout);
        item.link_search(out, scout, &what);
    }
    1
}

/// Comparer for `player_search_write_search_started_news_text`: the original writes into the real text and the copy
/// into a local one; text, links and generator must match (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_player_search_write_search_started_news_text(news: usize, full: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    if news == 0 || out == 0 {
        return o(news, full, out);
    }
    let mut r = 0;
    let mut n = 0;
    compare_text_writer(out, || r = o(news, full, out), |t| n = player_search_write_search_started_news_text(news, full, t),
        || format!("write_search_started_news_text(news {:#x}, full {})", news, full));
    SCOREBOARD.record(r == n, || format!("write_search_started_news_text(news {:#x}, full {}) returned {} x {}", news, full, r, n));
    r
}
