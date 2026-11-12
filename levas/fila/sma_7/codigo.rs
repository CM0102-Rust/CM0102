// ---------- batch %%LEVA%%: the manager appointment news text (manager_manager.cpp) ----------

/// Exe routines of the manager news texts without an `exe::` wrapper (Rust hooked, called through the exe).
pub(crate) mod manager_text_fns {
    pub(crate) const GET_SLOT_STRING: usize = 0x785a20; // NewsItem::get_slot_string: thiscall(news, k, buf, n), ret 0xc
    pub(crate) const GET_JOB_NAME: usize = 0x53ad00; // database_get_job_name: cdecl(job, short, capital, &gender) -> str
}

/// The manager's name of a manager news into `name` (0x190 bytes): a human manager (id ≥ `G_STAFF_COUNT` − 16) from
/// the two strings of the news (slot 8, then the slot whose index slot 7 holds; `get_slot_string`, 0x33 each;
/// `exe::database_format_name_from_strings`, style 1, −1); anyone else from the three name ids (`ids`,
/// `exe::database_format_name_from_ids`, style `style`).
pub(crate) unsafe fn manager_news_name(news: usize, id: u32, ids: [u32; 3], name: &mut [u8; 0x190], style: u32) {
    if id as i32 >= read_i32(G_STAFF_COUNT) - 0x10 {
        let get: unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32 = core::mem::transmute(manager_text_fns::GET_SLOT_STRING);
        let (mut first, mut second) = ([0u8; 0x34], [0u8; 0x34]);
        get(news, 8, first.as_mut_ptr(), 0x33);
        get(news, exe::newsitem_get_slot(news, 7), second.as_mut_ptr(), 0x33);
        exe::database_format_name_from_strings(first.as_ptr() as usize, second.as_ptr() as usize, 0usize, name.as_mut_ptr() as usize, 0x190, 1, -1);
    } else {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    }
}

/// Writes the headline or body of the news that a club appointed or promoted someone to be its manager.
///
/// `news_manager_appointment_text(news, body, out)`: the slots 0 (the person's id), 1..3 (the name ids), 4 (the old
/// club id, −1 none), 5 (the club id), 6 (the news code: 1 player, 2 staff, 3 manager) and 7 (the old job). The name
/// (`manager_news_name`: style 5 in the headline, 1 in the body). Headline: 0xa0a3a8 with the club's short gender and
/// short name (`club::SHORT_GENDER_NAME`, `club::SHORT_NAME`) and the name; links: the person (0x7da) when the name
/// ids still match (`exe::database_staff_matches_names`) and the club (0x7d5). Body (formats into 0xde1f64,
/// `FN_LANGLIB_SPRINTF`, with the genders `club::GENDER_NAME` and names `club::NAME`): no old club 0xa09e18; code 1: the
/// same club 0xa09f14, otherwise 0xa09e78 (both clubs); code 2: without a club job (`.job_for_club()`) 0xa09f8c, at the
/// same club by the old job: 0 0xa0a184, 11 0xa0a1f4, other 0xa0a274 with the job's name and gender
/// (`database_get_job_name`); from another club: 11 0xa0a0e4, other 0xa09fe4; code 3: 0xa0a2f8; any other code leaves
/// the buffer as it was (the original's). Links: the person when the ids match (0x7dc for a national team club, 0x7d7
/// otherwise), the club and the old club (0x7d5).
/// Original: `news_manager_appointment_text` (0x6a3060, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s: [u32; 8] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let id = s[0];
    let ids = [s[1], s[2], s[3]];
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let club_at = |k: u32| read_usize(G_CLUB_TABLE).wrapping_add((k as usize).wrapping_mul(club::SIZE));
    let new = club_at(s[5]);
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let buf = 0xde1f64usize;
    if body == 0 {
        manager_news_name(news, id, ids, &mut name, 5);
        let g = read_i8(new + club::SHORT_GENDER_NAME) as i32;
        format(buf, 0xa0a3a8, g, g, new + club::SHORT_NAME, nm);
        exe::text_set_string(out, buf as *const u8);
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7da, person);
        }
        exe::text_add_link(out, (new + club::NAME) as *const u8, 0x7d5, new);
        return;
    }
    covered!("sma_7 appointment body");
    manager_news_name(news, id, ids, &mut name, 1);
    let old_id = s[4] as i32;
    let old = club_at(s[4]);
    let gn = read_i8(new + club::GENDER_NAME) as i32;
    let go = || read_i8(old + club::GENDER_NAME) as i32;
    let (newn, oldn) = (new + club::NAME, old + club::NAME);
    if old_id == -1 {
        format(buf, 0xa09e18, gn, gn, newn, nm);
    } else {
        let job_name = |gender: &mut i8| {
            let f: unsafe extern "C" fn(u32, u32, u32, *mut i8) -> usize = core::mem::transmute(manager_text_fns::GET_JOB_NAME);
            f(s[7] as u8 as u32, 0, 0, gender)
        };
        match s[6] as u8 as i8 {
            1 => {
                if s[5] == s[4] {
                    format(buf, 0xa09f14, gn, gn, newn, nm);
                } else {
                    format(buf, 0xa09e78, gn, gn, go(), go(), newn, oldn, nm);
                }
            }
            2 => {
                let old_job = s[7] as u8;
                if Staff(person).job_for_club() == 0 {
                    format(buf, 0xa09f8c, gn, gn, newn, nm);
                } else if s[5] == s[4] {
                    match old_job {
                        0 => {
                            format(buf, 0xa0a184, gn, gn, newn, nm);
                        }
                        0xb => {
                            format(buf, 0xa0a1f4, gn, gn, newn, nm);
                        }
                        _ => {
                            let mut gt: i8 = 0;
                            let title = job_name(&mut gt);
                            let gt = gt as i32;
                            format(buf, 0xa0a274, gn, gn, gt, gt, newn, title, nm); // sma7: promoted at the club
                        }
                    }
                } else {
                    let mut gt: i8 = 0;
                    let title = job_name(&mut gt);
                    let gt = gt as i32;
                    if old_job == 0xb {
                        format(buf, 0xa0a0e4, gn, gn, go(), go(), gt, gt, newn, oldn, title, nm);
                    } else {
                        format(buf, 0xa09fe4, gn, gn, go(), go(), gt, gt, go(), go(), gt, gt, newn, oldn, title, nm, oldn, title);
                    }
                }
            }
            3 => {
                format(buf, 0xa0a2f8, gn, gn, go(), go(), newn, nm, oldn);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        let kind = if s[5] as i32 > -1 && (s[5] as i32) < read_i32(G_CLUB_COUNT) && exe::database_is_national_team(new) != 0 { 0x7dc } else { 0x7d7 };
        exe::text_add_link(out, name.as_ptr(), kind, person);
    }
    exe::text_add_link(out, newn as *const u8, 0x7d5, new);
    if old_id != -1 {
        exe::text_add_link(out, oldn as *const u8, 0x7d5, old);
    }
}

/// Comparer for `news_manager_appointment_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_appointment_text(news, body, t),
        || format!("news_manager_appointment_text(body {})", body));
}

/// Writes the headline or body of a story about board pressure on a manager: a shortlist for his job, a rumour, or the
/// FA's view of a national team manager.
///
/// `news_manager_under_pressure_text(news, body, out)`: slots 0 (the manager's id), 1..3 (his name ids), 4 (the club
/// id), 5..7 (the chairman's name ids, 5 = −1 without one), 8 (the reason, 1..4), 9 (the kind: 1 rumour), 10 (the
/// chairman's id), 11 (the rumour). The names (`exe::database_format_name_from_ids`, style 5 in the headline, 1 in the
/// body), the club's name (`club::SHORT_NAME` in the headline, `club::NAME` in the body, through "%s") and its
/// nation's adjective (`exe::nation_get_nationality_adjective`, 100, 9). A national team with a nation (`club::NATION`)
/// takes the FA texts (genders 100). Headline: kind 1 0xa06ff8 / FA 0xa07054; otherwise (after 0xa06f98 when there is
/// no reason, a chairman name and no rumour: written and then overwritten, as in the original) 0xa06ed8 / FA 0xa06f38.
/// Body: kind 1: FA 0xa06e50; rumour 1 0xa06d18, other rumour 0xa06dc0; none 0xa06bec. Reason 1..4: with a chairman
/// 0xa0669c, 0xa06444, 0xa068f0, 0xa06b28 (and his name); without one the club text (0xa064f8, 0xa062f4, 0xa06778,
/// 0xa069b8) or the FA one (0xa065c4, 0xa06394, 0xa0682c, 0xa06a68). Other reasons: without a chairman 0xa06058 / FA
/// 0xa060d4; with a rumour 0xa06160; otherwise 0xa061f0. Then the text (`exe::text_set_string`), the manager's link
/// (0x7d7) when the ids match (`exe::database_staff_matches_names`) and the club's link (0x7d5) on the adjective (FA,
/// body) or the club name.
/// Original: `news_manager_under_pressure_text` (0x6a1190, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 12] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let style = if body != 0 { 1 } else { 5 };
    let (mut name, mut chair, mut club_name, mut adj) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x190], [0u8; 0x64]);
    exe::database_format_name_from_ids(s[1], s[2], s[3], name.as_mut_ptr(), 0x190, style, s[0]);
    if s[5] as i32 == -1 {
        chair[0] = 0;
    } else {
        exe::database_format_name_from_ids(s[5], s[6], s[7], chair.as_mut_ptr(), 0x190, style, s[10]);
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add((s[4] as usize).wrapping_mul(club::SIZE));
    sprintf(club_name.as_mut_ptr() as usize, 0x991944, club + if body != 0 { club::NAME } else { club::SHORT_NAME });
    exe::nation_get_nationality_adjective(read_usize(club + club::NATION), adj.as_mut_ptr() as usize, 0x64, 9);
    let (nm, ch, cl, ad) = (name.as_ptr() as usize, chair.as_ptr() as usize, club_name.as_ptr() as usize, adj.as_ptr() as usize);
    let fa = exe::database_is_national_team(club) != 0 && read_usize(club + club::NATION) != 0;
    let buf = 0xde1f64usize;
    let (reason, kind, rumour) = (s[8] as u8, s[9] as u8, s[11]);
    if body == 0 {
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa07054, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ff8, g, g, 100, cl, nm);
            }
        } else {
            if reason == 0 && s[5] != 0 && rumour == 0 {
                format(buf, 0xa06f98, g, g, cl, nm);
            }
            if fa {
                format(buf, 0xa06f38, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ed8, g, g, cl, nm);
            }
        }
    } else {
        covered!("sma_7 pressure body");
        let g = read_i8(club + club::GENDER_NAME) as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa06e50, 100, 100, ad, nm);
            } else if rumour != 0 {
                format(buf, if rumour == 1 { 0xa06d18 } else { 0xa06dc0 }, g, g, cl, nm);
            } else {
                format(buf, 0xa06bec, g, g, cl, nm, nm);
            }
        } else if (1..=4).contains(&reason) {
            // (with the chairman, the club, the FA) by the reason
            let t = [[0xa0669c, 0xa064f8, 0xa065c4], [0xa06444, 0xa062f4, 0xa06394], [0xa068f0, 0xa06778, 0xa0682c], [0xa06b28, 0xa069b8, 0xa06a68]][reason as usize - 1];
            if s[5] as i32 != -1 {
                format(buf, t[0], g, g, 100, cl, ch, nm); // sma7: the reason with the chairman
            } else if fa {
                format(buf, t[2], 100, 100, 100, ad, nm);
            } else {
                format(buf, t[1], g, g, 100, cl, nm);
            }
        } else if s[5] as i32 == -1 {
            if fa {
                format(buf, 0xa060d4, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06058, g, g, 100, cl, nm);
            }
        } else if rumour != 0 {
            format(buf, 0xa06160, g, g, 100, cl, ch, nm);
        } else {
            format(buf, 0xa061f0, g, g, nm, cl, ch);
        }
    }
    exe::text_set_string(out, buf as *const u8);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((s[0] as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, s[1], s[2], s[3], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
    if body != 0 && fa {
        exe::text_add_link(out, adj.as_ptr(), 0x7d5, club);
    } else {
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
    }
}

/// Comparer for `news_manager_under_pressure_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_under_pressure_text(news, body, t),
        || format!("news_manager_under_pressure_text(body {})", body));
}
