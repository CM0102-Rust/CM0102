// ---------- batch 316: the injury type names and the return to training news text (injury.cpp) ----------

/// The injury type names with the indefinite article ("a cold", "flu", "a twisted ankle"...), one per type 0..0x51.
const INJURY_TYPE_NAMES_WITH_ARTICLE: [usize; 0x52] = [
    0x9e88fc, 0x9e88f8, 0x9e88f0, 0x9e88e0, 0x9e88c4, 0x9e88b4, 0x9e88a0, 0x9e8890, 0x9e8880,
    0x9e8870, 0x9e8860, 0x9e8844, 0x9e8828, 0x9e8810, 0x9e8800, 0x9e87f0, 0x9e87e0, 0x9e87d0,
    0x9e87bc, 0x9e87ac, 0x9e879c, 0x9e878c, 0x9e877c, 0x9e8764, 0x9e8750, 0x9e8738, 0x9e8724,
    0x9e8708, 0x9e86f4, 0x9e86e4, 0x9e86d4, 0x9e86c0, 0x9e86b4, 0x9e87ac, 0x9e878c, 0x9e86a4,
    0x9e8690, 0x9e8680, 0x9e8670, 0x9e8660, 0x9e8650, 0x9e8640, 0x9e8630, 0x9e8624, 0x9e8614,
    0x9e8604, 0x9e85f0, 0x9e85e4, 0x9e85d4, 0x9e85c4, 0x9e85b0, 0x9e859c, 0x9e858c, 0x9e8578,
    0x9e8568, 0x9e8558, 0x9e8548, 0x9e8538, 0x9e8524, 0x9e850c, 0x9e84f8, 0x9e84e8, 0x9e84d8,
    0x9e84c4, 0x9e84b4, 0x9e84a4, 0x9e8494, 0x9e8484, 0x9e8478, 0x9e8468, 0x9e8458, 0x9e8444,
    0x9e842c, 0x9e8418, 0x9e8408, 0x9e83f4, 0x9e83e8, 0x9e83d8, 0x9e83c8, 0x9e83c0, 0x9e83b0,
    0x9e83a8,
];

/// The injury type names without the article ("cold", "flu", "twisted ankle"...), one per type 0..0x51.
const INJURY_TYPE_NAMES: [usize; 0x52] = [
    0x9e83a0, 0x9e88f8, 0x9e88f0, 0x9e88e0, 0x9e8388, 0x9e8378, 0x9e8368, 0x9e8358, 0x9e834c,
    0x9e833c, 0x9e8330, 0x9e8844, 0x9e8318, 0x9e8810, 0x9e8308, 0x9e82fc, 0x9e82ec, 0x9e82e0,
    0x9e82cc, 0x9e82c0, 0x9e879c, 0x9e82b4, 0x9e82a4, 0x9e8764, 0x9e8290, 0x9e8738, 0x9e8724,
    0x9e8708, 0x9e827c, 0x9e826c, 0x9e825c, 0x9e824c, 0x9e8240, 0x9e82c0, 0x9e82b4, 0x9e8230,
    0x9e821c, 0x9e8210, 0x9e8200, 0x9e81f4, 0x9e81e4, 0x9e81d8, 0x9e8630, 0x9e8624, 0x9e81c8,
    0x9e81b8, 0x9e81a8, 0x9e819c, 0x9e818c, 0x9e8180, 0x9e8170, 0x9e8160, 0x9e8150, 0x9e8140,
    0x9e8134, 0x9e8124, 0x9e8114, 0x9e8108, 0x9e80f4, 0x9e80e0, 0x9e80d0, 0x9e80c0, 0x9e80b0,
    0x9e809c, 0x9e808c, 0x9e807c, 0x9e8070, 0x9e8064, 0x9e8478, 0x9e8058, 0x9e8048, 0x9e8038,
    0x9e8024, 0x9e8010, 0x9e8004, 0x9e7ff4, 0x9e83e8, 0x9e83d8, 0x9e83c8, 0x9e83c0, 0x9e83b0,
    0x9e83a8,
];

/// Writes the localised name of an injury type into a buffer, with or without an indefinite article.
///
/// `injury_get_type_name(kind, buffer, size, with_article)`: a buffer under 30 bytes shows the game's error box (line
/// 0xe8d). With the article (`with_article` 1) and a language other than 3 (`exe::langlib_get_language`), the name from
/// the table with the article; with `with_article` 0, or in language 3, the name from the table without it; any other
/// `with_article` outside language 3 writes nothing and gives the language. The name is translated
/// (`FN_LANGLIB_SPRINTF`), whose result is returned. A type outside 0..0x51 shows the box (line 0xf49 with the article,
/// 0xeec without).
/// Original: `injury_get_type_name` (0x62ef30, injury.cpp, thiscall, ret 0x10; `this` is not used).
pub(crate) unsafe extern "thiscall" fn injury_get_type_name(_this: usize, kind: u32, buffer: usize, size: i32, with_article: u32) -> i32 {
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if size < 0x1e {
        show_error_at(0x9e7f28, 0xe8d);
        return 0;
    }
    let k = kind as u8 as i8 as i32 as u32;
    let article = with_article as u8;
    if article != 0 {
        let language = exe::langlib_get_language();
        if language != 3 {
            if article != 1 {
                return language;
            }
            if k > 0x51 {
                show_error_at(0x9e7f28, 0xf49);
                return 0;
            }
            return translate(buffer, INJURY_TYPE_NAMES_WITH_ARTICLE[k as usize]);
        }
    }
    if k > 0x51 {
        show_error_at(0x9e7f28, 0xeec);
        return 0;
    }
    translate(buffer, INJURY_TYPE_NAMES[k as usize]) // sma9: the name without the article
}

/// The grammatical gender class of an injury type's name (`injury_get_type_name_gender`, 0x6306e0, original; thiscall,
/// ret 4, `this` not used: its prologue starts with a relative call, which the detours do not copy).
pub(crate) unsafe fn injury_type_name_gender(this: usize, kind: u8) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x6306e0usize);
    f(this, kind as u32)
}

/// Writes the text of the return to light training or full training news, with wording for illnesses, injuries,
/// treatments and holidays.
///
/// `injury_write_recovery_news_text(news, body, out)`, the text of news 2001 (`post_recovery_news`): slots 0 (the
/// stage: 0 light training, otherwise full training), 1 (the injury type), 2 (the person's id), 3..5 (the name ids), 6
/// and 7 (the birth date, `exe::new_date`), 8 (the days out), 9 (the outcome) and 10. The name
/// (`exe::database_format_name_from_ids`, 0x99 bytes, style 5 in the headline, 1 in the body). Light training: headline
/// 0x9e9bb8. Body by the type's category (the 13-byte type table at 0x9e7ad9): category 12 (treatments and holidays)
/// by the type ('Q' 0x9e9b50, 'P' 0x9e9a54, 'M' 0x9e99dc, 'N' 0x9e9954, 'O' 0x9e989c, others 0x9e9838 through the C
/// runtime's `sprintf`, untranslated); category 0 (illnesses) by the type (0 0x9e97e4, 1 0x9e9788, 2 0x9e9728, others
/// 0x9e96c0); others 0x9e9648 with the injury's name and gender: the category's name (`injury_get_category_name`) or,
/// for category 6, the type's (`injury_get_type_name` without the article and `injury_type_name_gender`); then
/// the expected time (`format_duration_text` of the days, approximate) appended in 0x9e95f0 (`exe::text_append_string`).
/// Full training: headline 0x9e95a4 for 'Q' with slot 10 = 3, otherwise 0x9e9568. Body (the name also in style 5):
/// category 12 by the type: 'Q' by slot 10 (2 0x9e94b4, 3 0x9e9460, others 0x9e9b50); 'M' by slot 10 (−1 0x9e9344, 1
/// 0x9e921c) or by the outcome (0 0x9e9150, others 0x9e9094); 'P' 0x9e8f9c; 'N' by the outcome (0 0x9e8f1c, others
/// 0x9e8e1c with both names); 'O' with outcome 2 by slot 10 (1 0x9e8d4c, others 0x9e8ce4), others 0x9e8c1c; other types
/// 0x9e8b98; category 0 by the type (0 0x9e8b48, 1 0x9e8af0, 2 0x9e8a94, others 0x9e8a30); others 0x9e89bc with the
/// injury's name and gender as above. Then the text (`exe::text_set_string`) and the person's link (0x7d7) when the
/// ids and the birth date still match (`exe::database_staff_matches_names`).
/// Original: `injury_write_recovery_news_text` (0x631580, injury.cpp, thiscall, ret 0xc; `this` only passed on).
pub(crate) unsafe extern "thiscall" fn injury_write_recovery_news_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let buf = 0xde1f64usize;
    let full = exe::newsitem_get_slot(news, 0) as u8 != 0;
    let kind = exe::newsitem_get_slot(news, 1) as u8;
    let id = exe::newsitem_get_slot(news, 2);
    let ids = [exe::newsitem_get_slot(news, 3), exe::newsitem_get_slot(news, 4), exe::newsitem_get_slot(news, 5)];
    let mut birth = [0u8; 8];
    let year = exe::newsitem_get_slot(news, 7);
    let day = exe::newsitem_get_slot(news, 6);
    exe::new_date(birth.as_mut_ptr() as usize, day, year);
    let days = exe::newsitem_get_slot(news, 8);
    let outcome = exe::newsitem_get_slot(news, 9) as i32;
    let extra = exe::newsitem_get_slot(news, 10) as i32;
    let mut name = [0u8; 0x99];
    let nm = name.as_ptr() as usize;
    let mut injury = [0u8; 0x34];
    let inj = injury.as_mut_ptr() as usize;
    let category = read_u8(0x9e7ad9 + (kind as i8 as isize * 13) as usize);
    // the injury's name and its gender: the category's, or the type's for category 6
    let injury_name = || -> i32 {
        let g = if category != 6 {
            injury_get_category_name(category as u32, inj, 0x32) as u8
        } else {
            injury_get_type_name(this, kind as u32, inj, 0x32, 0);
            injury_type_name_gender(this, kind)
        };
        g as i8 as i32
    };
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x99, 5, id);
        let headline = if !full { 0x9e9bb8 } else if kind == 0x51 && extra == 3 { 0x9e95a4 } else { 0x9e9568 };
        format(buf, headline, nm);
        exe::text_set_string(out, buf as *const u8);
    } else if !full {
        covered!("sma_9 light training body");
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x99, 1, id);
        if category == 0xc {
            match kind {
                0x51 => format(buf, 0x9e9b50, nm),
                0x50 => format(buf, 0x9e9a54, nm),
                0x4d => format(buf, 0x9e99dc, nm),
                0x4e => format(buf, 0x9e9954, nm),
                0x4f => format(buf, 0x9e989c, nm),
                _ => crt_format(buf, 0x9e9838, nm),
            };
        } else if category == 0 {
            format(buf, match kind { 0 => 0x9e97e4, 1 => 0x9e9788, 2 => 0x9e9728, _ => 0x9e96c0 }, nm);
        } else {
            let g = injury_name();
            format(buf, 0x9e9648, g, g, nm, inj);
        }
        exe::text_set_string(out, buf as *const u8);
        let mut time = [0u8; 0x34];
        format_duration_text(days as u16 as i16 as i32, time.as_mut_ptr() as usize, 0x32, 1); // sma9: the approximate time
        format(buf, 0x9e95f0, time.as_ptr() as usize);
        exe::text_append_string(out, buf);
    } else {
        covered!("sma_9 full training body");
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x99, 1, id);
        let mut formal = [0u8; 0x99];
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], formal.as_mut_ptr(), 0x99, 5, id);
        if category == 0xc {
            let text = match kind {
                0x51 => match extra { 2 => 0x9e94b4, 3 => 0x9e9460, _ => 0x9e9b50 },
                0x4d => match extra { -1 => 0x9e9344, 1 => 0x9e921c, _ => if outcome == 0 { 0x9e9150 } else { 0x9e9094 } },
                0x50 => 0x9e8f9c,
                0x4e => if outcome == 0 { 0x9e8f1c } else { 0 },
                0x4f => if outcome == 2 { if extra == 1 { 0x9e8d4c } else { 0x9e8ce4 } } else { 0x9e8c1c },
                _ => 0x9e8b98,
            };
            if text != 0 {
                format(buf, text, nm);
            } else {
                format(buf, 0x9e8e1c, nm, formal.as_ptr() as usize);
            }
        } else if category == 0 {
            format(buf, match kind { 0 => 0x9e8b48, 1 => 0x9e8af0, 2 => 0x9e8a94, _ => 0x9e8a30 }, nm);
        } else {
            let g = injury_name();
            format(buf, 0x9e89bc, g, g, nm, inj);
        }
        exe::text_set_string(out, buf as *const u8);
    }
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], birth.as_ptr() as usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
}

/// Comparer for `injury_get_type_name`: the buffer and the result side by side (a short buffer: only the original).
pub(crate) unsafe extern "thiscall" fn compare_injury_get_type_name(this: usize, kind: u32, buffer: usize, size: i32, with_article: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32, u32) -> i32>(IDX_INJURY_GET_TYPE_NAME);
    if size < 0x1e || buffer == 0 {
        return o(this, kind, buffer, size, with_article);
    }
    let (r, n, same, _, _) = crate::comparison::side_by_side(&[(buffer, (size as usize).min(0x100))], || o(this, kind, buffer, size, with_article), || injury_get_type_name(this, kind, buffer, size, with_article));
    SCOREBOARD.record(same && r == n, || format!("injury_get_type_name({}, article {}) original={} rust={}", kind as u8 as i8, with_article as u8, r, n));
    r
}

/// Comparer for `injury_write_recovery_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_injury_write_recovery_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_INJURY_WRITE_RECOVERY_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| injury_write_recovery_news_text(this, news, body, t),
        || format!("injury_write_recovery_news_text(body {})", body));
}

// ---------- batch 316: the manager departure news text (manager_manager.cpp) ----------

/// The departing manager's name of a departure news into `name` (0x190 bytes): a human manager (id ≥ `G_STAFF_COUNT`
/// − 16) from the two strings of the news (slot 13, then the slot whose index slot 12 holds; `get_slot_string`, 0x33
/// each; `exe::database_format_name_from_strings`, style 1, −1); anyone else from the three name ids (`ids`,
/// `exe::database_format_name_from_ids`, style `style`).
pub(crate) unsafe fn departure_news_name(news: usize, id: u32, ids: [u32; 3], name: &mut [u8; 0x190], style: u32) {
    if id as i32 >= read_i32(G_STAFF_COUNT) - 0x10 {
        let get: unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32 = core::mem::transmute(manager_text_fns::GET_SLOT_STRING);
        let (mut first, mut second) = ([0u8; 0x34], [0u8; 0x34]);
        get(news, 0xd, first.as_mut_ptr(), 0x33);
        get(news, exe::newsitem_get_slot(news, 0xc), second.as_mut_ptr(), 0x33);
        exe::database_format_name_from_strings(first.as_ptr() as usize, second.as_ptr() as usize, 0usize, name.as_mut_ptr() as usize, 0x190, 1, -1);
    } else {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    }
}

/// Writes the headline or body of the news that a club or national team manager resigned, retired or was sacked.
///
/// `news_manager_departure_text(news, body, out)`: slots 0 (the manager's id), 1..3 (his name ids), 4 (how he left: 1
/// sacked, 2 resigned, 3 retired), 5 (the club id), 7 (the job: 1 manager, 2 player-manager, 3 with a successor or
/// reason), 8..10 (a second person's name ids, −1 none) and, in the body, 11 (the reason code). The name
/// (`departure_news_name`: style 5 in the headline, 1 in the body); the second name (`exe::database_format_name_from_ids`
/// with −1; empty without one; also computed in the headline, where it is not used). Headline (with the club's short
/// gender and short name, `club::SHORT_GENDER_NAME`, `club::SHORT_NAME`): sacked as manager 0xa09bb0, as
/// player-manager 0xa09c08, job 3 0xa09c68, other jobs nothing; resigned as manager 0xa09cb8, as player-manager
/// 0xa09d10, job 3 0xa09d6c, other jobs nothing; retired 0xa09dc4; any other way 0xa09d6c. Body (with the club's gender
/// `club::GENDER_NAME`, its name `club::NAME`, and the nationality adjective of `club::NATION`,
/// `exe::nation_get_nationality_adjective` language 9, or "Sedgefordian" without a nation): a national team
/// (`exe::database_is_national_team`): the second national team (`exe::club_is_national_team`) sacked by the reason
/// (2 0xa096d0, 4 0xa09788, 6 0xa09848, others 0xa09950), retired 0xa099e8, otherwise 0xa09ac8; the first one sacked by
/// the reason (2 0xa09210, 4 0xa092d0, 6 0xa09394, others 0xa09494), retired 0xa09534, otherwise 0xa09608. A club:
/// sacked: as manager with a second name 0xa071a4, without 0xa070b0; as player-manager 0xa0736c / 0xa072b0; job 3 by
/// the reason 0..15 from two tables (with a second name: 0xa08624, 0xa08434, 0xa08338, 0xa080f0, 0xa08528, 0xa08208,
/// 0xa087f8, 0xa0772c, 0xa086f8, 0xa08c48, 0xa08b04, 0xa08934, 0xa08dfc, others 0xa080f0; without: 0xa07a08, 0xa07848,
/// 0xa0764c, 0xa07924, 0xa07540, 0xa07bbc, 0xa0772c, 0xa07ac4, 0xa07e7c, 0xa07ccc, 0xa07fdc, reason 13 nothing, others
/// 0xa07440), other jobs nothing; resigned as manager 0xa08f40, as player-manager 0xa09030, otherwise 0xa090f0; any
/// other way 0xa091a8. "Nothing" leaves the buffer 0xde1f64 as it was. Then the text (`exe::text_set_string`), the
/// person's link (0x7d7) when the name ids still match (`exe::database_staff_matches_names`) and, in the body, the
/// club's link (0x7d5) on its name and on the adjective when they appear in the text (`exe::crt_strstr`); in the
/// headline, the club's link on its short name always.
/// Original: `news_manager_departure_text` (0x6a1d60, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_departure_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 12] = core::array::from_fn(|k| if k == 6 || (k == 11 && body == 0) { 0 } else { exe::newsitem_get_slot(news, k as u32) });
    let id = s[0];
    let ids = [s[1], s[2], s[3]];
    let how = s[4] as u8 as i8;
    let job = s[7] as u8 as i8;
    let club = read_usize(G_CLUB_TABLE).wrapping_add((s[5] as usize).wrapping_mul(club::SIZE));
    let mut name = [0u8; 0x190];
    let mut second = [0u8; 0x190];
    let mut club_name = [0u8; 0x190];
    let mut adjective = [0u8; 0x190];
    let (n, n2, c, adj) = (name.as_ptr() as usize, second.as_ptr() as usize, club_name.as_mut_ptr() as usize, adjective.as_mut_ptr() as usize);
    let buf = 0xde1f64usize;
    let pct = 0x991944usize; // "%s"
    departure_news_name(news, id, ids, &mut name, if body == 0 { 5 } else { 1 });
    if body == 0 {
        if s[8] as i32 != -1 {
            exe::database_format_name_from_ids(s[8], s[9], s[10], second.as_mut_ptr(), 0x190, 5, u32::MAX);
        }
        crt_format(c, pct, club + club::SHORT_NAME);
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        match how {
            1 => match job {
                1 => { format(buf, 0xa09bb0, g, g, c, n); }
                2 => { format(buf, 0xa09c08, g, g, c, n); }
                3 => { format(buf, 0xa09c68, g, g, c, n); }
                _ => {}
            },
            2 => match job {
                1 => { format(buf, 0xa09cb8, g, g, c, n); }
                2 => { format(buf, 0xa09d10, g, g, c, n); }
                3 => { format(buf, 0xa09d6c, g, g, n, c); }
                _ => {}
            },
            3 => { format(buf, 0xa09dc4, g, g, c, n); }
            _ => { format(buf, 0xa09d6c, g, g, n, c); }
        }
        exe::text_set_string(out, buf as *const u8);
        let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
        }
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
        return;
    }
    covered!("sma_10 departure body");
    let reason = s[11] as u8 as i8;
    crt_format(c, pct, club + club::NAME);
    let nation = read_usize(club + club::NATION);
    if nation != 0 {
        exe::nation_get_nationality_adjective(nation, adj, 0x190, 9);
    } else {
        crt_format(adj, pct, 0xa09ba0usize); // "Sedgefordian"
    }
    if s[8] as i32 != -1 {
        exe::database_format_name_from_ids(s[8], s[9], s[10], second.as_mut_ptr(), 0x190, 1, u32::MAX);
    }
    let g = read_i8(club + club::GENDER_NAME) as i32;
    let has_second = second[0] != 0;
    if exe::database_is_national_team(club) != 0 {
        covered!("sma_10 national team departure");
        let second_team = exe::club_is_national_team(club) != 0;
        match (how, second_team) {
            (1, true) => match reason {
                2 => { format(buf, 0xa096d0, 0x64, 0x64, n, adj); }
                4 => { format(buf, 0xa09788, 0x64, 0x64, n, adj); }
                6 => { format(buf, 0xa09848, g, g, 0x64, 0x64, n, c, adj); }
                _ => { format(buf, 0xa09950, 0x64, 0x64, n, adj); }
            },
            (3, true) => { format(buf, 0xa099e8, g, g, 0x64, 0x64, n, c, adj); }
            (_, true) => { format(buf, 0xa09ac8, g, g, 0x64, 0x64, n, c, adj); }
            (1, false) => match reason {
                2 => { format(buf, 0xa09210, 0x64, 0x64, n, adj); }
                4 => { format(buf, 0xa092d0, 0x64, 0x64, n, adj); }
                6 => { format(buf, 0xa09394, g, g, 0x64, 0x64, n, c, adj); }
                _ => { format(buf, 0xa09494, 0x64, 0x64, n, adj); }
            },
            (3, false) => { format(buf, 0xa09534, g, g, 0x64, 0x64, n, c, adj); }
            (_, false) => { format(buf, 0xa09608, g, g, 0x64, 0x64, n, c, adj); }
        }
    } else {
        match how {
            1 => match job {
                1 => {
                    if has_second { format(buf, 0xa071a4, g, g, g, g, n, c, n2, c); } else { format(buf, 0xa070b0, g, g, g, g, n, c, c); }
                }
                2 => {
                    if has_second { format(buf, 0xa0736c, g, g, g, g, n, c, n2, c); } else { format(buf, 0xa072b0, g, g, g, g, n, c, c); }
                }
                3 => {
                    covered!("sma_10 sacked with a reason");
                    if has_second {
                        match reason {
                            0 | 1 | 3 => { format(buf, 0xa08624, g, g, g, g, n, c, n2, c); }
                            2 => { format(buf, 0xa08434, g, g, g, g, n, c, n2, c); }
                            4 => { format(buf, 0xa08338, g, g, g, g, n, c, n2, c); }
                            5 => { format(buf, 0xa08624, g, g, g, g, n, c, n2, c); }
                            7 => { format(buf, 0xa08528, g, g, g, g, n, c, n2, c); }
                            8 => { format(buf, 0xa08208, g, g, g, g, c, n2, n, c); }
                            9 => { format(buf, 0xa087f8, g, g, 0x64, n, c, n2, n); }
                            10 => { format(buf, 0xa0772c, g, g, g, g, n, c, n2, c); }
                            11 => { format(buf, 0xa086f8, g, g, g, g, n, c, n2, c); }
                            12 => { format(buf, 0xa08c48, g, g, 0x64, c, n2, n, n); }
                            13 => { format(buf, 0xa08b04, g, g, 0x64, c, n2, n, n); }
                            14 => { format(buf, 0xa08934, g, g, c, n2, n, n); }
                            15 => { format(buf, 0xa08dfc, g, g, n, c, n2, n); }
                            _ => { format(buf, 0xa080f0, g, g, g, g, n, c, n2, c); } // sma10: the default reason
                        }
                    } else {
                        match reason {
                            0 | 1 | 5 => { format(buf, 0xa07a08, g, g, g, g, n, c, c); }
                            2 => { format(buf, 0xa07848, g, g, g, g, n, c, c); }
                            3 => { format(buf, 0xa07a08, g, g, g, g, n, c, c); }
                            4 => { format(buf, 0xa0764c, g, g, g, g, n, c, c); }
                            7 => { format(buf, 0xa07924, g, g, g, g, n, c, c); }
                            8 => { format(buf, 0xa07540, g, g, g, g, c, n, c); }
                            9 => { format(buf, 0xa07bbc, g, g, 0x64, n, c, n); }
                            10 => { format(buf, 0xa0772c, g, g, g, g, n, c, n2, c); }
                            11 => { format(buf, 0xa07ac4, g, g, g, g, n, c, c); }
                            12 => { format(buf, 0xa07e7c, g, g, c, n); }
                            13 => {}
                            14 => { format(buf, 0xa07ccc, g, g, c, n, n); }
                            15 => { format(buf, 0xa07fdc, g, g, n, c, n); }
                            _ => { format(buf, 0xa07440, g, g, g, g, n, c, c); }
                        }
                    }
                }
                _ => {}
            },
            2 => match job {
                1 => { format(buf, 0xa08f40, g, g, g, g, n, c, c); }
                2 => { format(buf, 0xa09030, g, g, g, g, n, c, c); }
                _ => { format(buf, 0xa090f0, g, g, g, g, n, c, c); }
            },
            _ => { format(buf, 0xa091a8, g, g, n, c); }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
    if exe::crt_strstr(buf, c) != 0 { // sma10: the club's link when its name is in the text
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
    }
    if exe::crt_strstr(buf, adj) != 0 {
        exe::text_add_link(out, adjective.as_ptr(), 0x7d5, club);
    }
}

/// Comparer for `news_manager_departure_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_departure_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_DEPARTURE_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_departure_text(news, body, t),
        || format!("news_manager_departure_text(body {})", body));
}
