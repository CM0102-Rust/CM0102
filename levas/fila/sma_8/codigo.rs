// ---------- batch %%LEVA%%: the retirement announcement text (player_regen.cpp) ----------

/// The job name of a role and its gender (`database_get_job_name`, called through the exe at 0x53ad00: long, lower
/// case).
pub(crate) unsafe fn retirement_job_name(role: u8, gender: &mut i8) -> usize {
    let f: unsafe extern "C" fn(u32, u32, u32, *mut i8) -> usize = core::mem::transmute(0x53ad00usize);
    f(role as u32, 0, 0, gender)
}

/// Builds the title and body text of the news that a staff member announces his decision to retire.
///
/// `player_regen_write_retirement_announcement_text(news, body, out)`: slots 0 (the person's id), 1..3 (the name ids),
/// 4 (the kind: 1, 2, 0x10, 0x20), 5 (the club id, −1 none) and 6 (an age). The name (`exe::database_format_name_from_ids`,
/// 0x190, style 5 in the headline, 1 in the body). Headline: 0xa70110 with the name. Body: when the club's manager
/// (`club::MANAGER`) is the human of the current screen slot (`G_SCREEN_RECORDS_MANAGER` + 0x300 ×
/// `G_SCREEN_MANAGER_CURRENT_SLOT`), the texts addressed to him (only the name): 0x10 without a non-player sheet by the
/// age (`staff::AGE` below 27 0xa6fb74, else 0xa6fac8), with one by the club job (0/0x10 0xa6fc1c, 0xc 0xa6fd20, 0xd
/// 0xa6fe1c, 0xf 0xa6ff24, other 0xa70020); 1 without a sheet 0xa6f57c, with one by the club job (0/0x10/0xb 0xa6e5cc,
/// 0xc 0xa6f628, 0xd 0xa6f730, 0xf 0xa6f840, other by slot 6 below 28 0xa6f944, else 0xa6fa04); 0x20 with a club job
/// 0xa6f4b0 (the job's name, `database_get_job_name`), with a national job 0xa6f3cc (the national team's gender and
/// name), else 0xa6f338; 2 0xa6e24c. Otherwise the club texts (the club's short gender and short name, the original
/// reads them even without a club for kind 0x10): 0x10 as above (0xa6ec2c / 0xa6eb44 by the age; 0xa6ed10, 0xa6ee4c,
/// 0xa6ef84, 0xa6f0c8, 0xa6f200 by the job); 1 without a sheet 0xa6e4fc with a club or 0xa6f57c, with one 0xa6e6cc,
/// 0xa6e7f8, 0xa6e930, 0xa6ea5c by the job at a club, else 0xa6e5cc; 0x20 with a club and a club job 0xa6e408, with a
/// national job 0xa6f3cc, else 0xa6e35c; 2 with a club 0xa6e2c0, else 0xa6e24c. Other kinds leave the buffer
/// 0xde1f64 as it was. Then the text (`exe::text_set_string`) and the person's link (0x7d7) when the ids still match
/// (`exe::database_staff_matches_names`).
/// Original: `player_regen_write_retirement_announcement_text` (0x7aa4c0, player_regen.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn player_regen_write_retirement_announcement_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let buf = 0xde1f64usize;
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let ids = [exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2), exe::newsitem_get_slot(news, 3)];
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, 5, id);
        format(buf, 0xa70110, nm);
    } else {
        covered!("sma_8 retirement body");
        let kind = exe::newsitem_get_slot(news, 4) as u8;
        let club_id = exe::newsitem_get_slot(news, 5) as i32;
        let age = exe::newsitem_get_slot(news, 6) as i32;
        let club = if club_id != -1 { read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE)) } else { 0 };
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, 1, id);
        let human = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
        let at_human = club != 0 && read_usize(club + club::MANAGER) != 0 && read_usize(club + club::MANAGER) == human;
        let sheet = !person.non_player().is_null();
        let role = person.job_for_club();
        let say = |t: usize| {
            format(buf, t, nm);
        };
        let g = || read_i8(club + club::SHORT_GENDER_NAME) as i32;
        let say_club = |t: usize| {
            format(buf, t, g(), g(), club + club::SHORT_NAME, nm);
        };
        // the national job text of kind 0x20, when the person has one; false otherwise
        let national = || {
            let nr = person.job_for_nation();
            let nj = person.national_job();
            if nr != 0 && nr != 0x10 && !nj.is_null() {
                let mut gt: i8 = 0;
                let title = retirement_job_name(nr, &mut gt);
                let gn = read_i8(nj.addr() + club::GENDER_NAME) as i32;
                format(buf, 0xa6f3cc, gn, gn, gt as i32, gt as i32, nm, nj.addr() + club::NAME, title);
                true
            } else {
                false
            }
        };
        let by_job = |t: [usize; 5]| match role {
            0 | 0x10 => t[0],
            0xc => t[1],
            0xd => t[2],
            0xf => t[3],
            _ => t[4],
        };
        if at_human {
            match kind {
                0x10 => say(if !sheet {
                    if (person.age() as i8) < 0x1b { 0xa6fb74 } else { 0xa6fac8 }
                } else {
                    by_job([0xa6fc1c, 0xa6fd20, 0xa6fe1c, 0xa6ff24, 0xa70020])
                }),
                1 => say(if !sheet {
                    0xa6f57c
                } else if role == 0 || role == 0x10 || role == 0xb {
                    0xa6e5cc
                } else {
                    by_job([0, 0xa6f628, 0xa6f730, 0xa6f840, if age < 0x1c { 0xa6f944 } else { 0xa6fa04 }])
                }),
                0x20 => {
                    if role != 0 && role != 0x10 && !person.club_job().is_null() {
                        let mut gt: i8 = 0;
                        let title = retirement_job_name(role, &mut gt);
                        format(buf, 0xa6f4b0, gt as i32, gt as i32, nm, title);
                    } else if !national() {
                        say(0xa6f338);
                    }
                }
                2 => say(0xa6e24c),
                _ => {}
            }
        } else {
            match kind {
                0x10 => say_club(if !sheet {
                    if (person.age() as i8) < 0x1b { 0xa6ec2c } else { 0xa6eb44 } // sma8: the young retiring player
                } else {
                    by_job([0xa6ed10, 0xa6ee4c, 0xa6ef84, 0xa6f0c8, 0xa6f200])
                }),
                1 => {
                    if !sheet {
                        if club != 0 { say_club(0xa6e4fc) } else { say(0xa6f57c) }
                    } else if role != 0 && role != 0x10 && role != 0xb && club != 0 {
                        say_club(by_job([0, 0xa6e6cc, 0xa6e7f8, 0xa6e930, 0xa6ea5c]));
                    } else {
                        say(0xa6e5cc);
                    }
                }
                0x20 => {
                    if club != 0 && role != 0 && role != 0x10 && !person.club_job().is_null() {
                        let mut gt: i8 = 0;
                        let title = retirement_job_name(role, &mut gt);
                        format(buf, 0xa6e408, g(), g(), gt as i32, gt as i32, club + club::SHORT_NAME, nm, title);
                    } else if !national() {
                        say(0xa6e35c);
                    }
                }
                2 => {
                    if club != 0 { say_club(0xa6e2c0) } else { say(0xa6e24c) }
                }
                _ => {}
            }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
}

/// Comparer for `player_regen_write_retirement_announcement_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_write_retirement_announcement_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| player_regen_write_retirement_announcement_text(news, body, t),
        || format!("player_regen_write_retirement_announcement_text(body {})", body));
}
