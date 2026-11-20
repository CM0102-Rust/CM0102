// ---------- batch %%LEVA%%: the manager departure news text (manager_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_departure_text(news, body, t),
        || format!("news_manager_departure_text(body {})", body));
}
