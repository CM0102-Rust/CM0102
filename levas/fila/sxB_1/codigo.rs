// --- sxB_1: the headline and body of a retirement news item (player_regen.cpp) ---

/// The slots of news item (4, 0xbba), a person's retirement, that its text writer reads. Slot 10 is not read.
struct RetirementSlots {
    /// Slot 0: the person's id; slots 1..3: his name ids.
    id: u32,
    names: [u32; 3],
    /// Slot 4: the reason (8 a recurring injury, 0x10 money, 0x20 immediate).
    reason: u8,
    /// Slot 5: the retirement was announced (not a shock) [inference].
    announced: u8,
    /// Slot 6: the club id (−1 none) and slot 7 the id of the nation he was capped by (−1 none).
    club: i32,
    nation: i32,
    /// Slot 8: his job at the club (the `staff::JOB_FOR_CLUB` codes), passed to the job name routines.
    job: u8,
    /// Slot 9: he is an international; slot 11: he stays in football; slot 12: his age; slot 13: he retires from a
    /// staff job; slot 14: that job is not his own [inference].
    international: bool,
    stays: bool,
    age: i8,
    staff_job: bool,
    other_job: bool,
}

impl RetirementSlots {
    unsafe fn read(news: usize) -> Self {
        let s = |k: u32| exe::newsitem_get_slot(news, k);
        RetirementSlots {
            id: s(0),
            names: [s(1), s(2), s(3)],
            reason: s(4) as u8,
            announced: s(5) as u8,
            club: s(6) as i32,
            nation: s(7) as i32,
            job: s(8) as u8,
            international: s(9) as u8 != 0,
            stays: s(11) != 0,
            age: s(12) as i8,
            staff_job: s(13) != 0,
            other_job: s(14) != 0,
        }
    }
    fn person(&self) -> Staff {
        unsafe { staff_at(self.id as i32 as isize) }
    }
}

/// The club (gender byte, name) and the nation (gender byte, name) of a retirement text; the nation's name is at +4.
unsafe fn retirement_club(id: i32) -> (i32, usize) {
    let c = club_at(id as isize);
    (c.gender_name() as i8 as i32, c.addr() + club::NAME)
}

unsafe fn retirement_nation(id: i32) -> (i32, usize) {
    let n = Nation(nation_by_id(id as usize));
    (n.gender_name() as i8 as i32, n.addr() + 4) // the nation's name
}

/// The body of a staff member's retirement (slot 13 set, slot 14 clear): with a club 0xa6e09c (the club and the job's
/// name, `database_get_job_name`), else with a nation the same template with the nation, else with a job 0xa6e04c
/// ("Ex <job>"), else 0xa6e000 with the job name of 0.
unsafe fn retirement_staff_body(r: &RetirementSlots, name: usize, gender: &mut i8) {
    let buf = NEWS_TEXT_BUFFER;
    if r.club != -1 || r.nation != -1 {
        let job = exe::database_get_job_name(r.job as u32, 0, 0, gender);
        let (g, place) = if r.club != -1 { retirement_club(r.club) } else { retirement_nation(r.nation) };
        let jg = *gender as i32;
        story_lang!(buf, 0xa6e09c, g, g, jg, jg, g, g, jg, jg, place, job, name, place, job);
    } else {
        let job = exe::database_get_job_name(r.job as u32, 0, 0, gender);
        let jg = *gender as i32;
        story_lang!(buf, if r.job != 0 { 0xa6e04c } else { 0xa6e000 }, jg, jg, job, name);
    }
}

/// The body of an international who stays in football (slots 9 and 11): capped by a nation and at a club, a coaching
/// job (`database_get_coaching_job_name`, the job not 0, 0x10 or 0xb) 0xa6df18, else by a draw (`exe::utils_random(2)`)
/// a coaching role 0xa6de5c or a management position 0xa6dd98; without a club, a job (not 0, 0x10, 0xb, 0xc, 0xf or
/// 0xd) 0xa6dcd8, else 0xa6dc18; without a nation, "Former International" with a job 0xa6db74, else 0xa6dad0.
unsafe fn retirement_international_stays_body(r: &RetirementSlots, name: usize, gender: &mut i8) {
    let buf = NEWS_TEXT_BUFFER;
    let plain_job = matches!(r.job, 0 | 0x10 | 0xb);
    if r.nation != -1 {
        let (ng, nation) = retirement_nation(r.nation);
        if r.club != -1 {
            if !plain_job {
                let job = database_get_coaching_job_name(r.job as u32, gender as *mut i8 as usize);
                let (cg, club) = retirement_club(r.club);
                let jg = *gender as i32;
                story_lang!(buf, 0xa6df18, ng, ng, jg, jg, cg, cg, nation, name, job, club);
            } else if exe::utils_random(2) != 0 {
                story_lang!(buf, 0xa6de5c, ng, ng, nation, name);
            } else {
                story_lang!(buf, 0xa6dd98, ng, ng, nation, name);
            }
        } else if !plain_job && !matches!(r.job, 0xc | 0xf | 0xd) {
            let job = database_get_coaching_job_name(r.job as u32, gender as *mut i8 as usize);
            let jg = *gender as i32;
            story_lang!(buf, 0xa6dcd8, ng, ng, jg, jg, nation, name, job);
        } else {
            story_lang!(buf, 0xa6dc18, ng, ng, nation, name);
        }
    } else if !plain_job {
        let job = database_get_coaching_job_name(r.job as u32, gender as *mut i8 as usize);
        let jg = *gender as i32;
        story_lang!(buf, 0xa6db74, jg, jg, name, job);
    } else {
        story_lang!(buf, 0xa6dad0, name);
    }
}

/// The body of an announced retirement (slot 5 set): a staff member's (`retirement_staff_body`); else one who stays in
/// football: an international (`retirement_international_stays_body`), or one with a club job
/// (`staff::JOB_FOR_CLUB` not 0 or 0x10) who keeps it 0xa6da38 (the coaching job's name), else 0xa6d9ac; else an
/// international capped by a nation, at a club 0xa6d900, without one 0xa6d870; else the plain text 0xa6e164.
unsafe fn retirement_announced_body(r: &RetirementSlots, name: usize, gender: &mut i8) {
    let buf = NEWS_TEXT_BUFFER;
    if r.staff_job && !r.other_job {
        retirement_staff_body(r, name, gender);
    } else if r.stays {
        if r.international {
            retirement_international_stays_body(r, name, gender);
        } else if !matches!(r.person().job_for_club(), 0 | 0x10) {
            let job = database_get_coaching_job_name(r.job as u32, gender as *mut i8 as usize);
            let jg = *gender as i32;
            story_lang!(buf, 0xa6da38, jg, jg, name, job);
        } else {
            story_lang!(buf, 0xa6d9ac, name);
        }
    } else if r.international && r.nation != -1 {
        let (ng, nation) = retirement_nation(r.nation);
        if r.club != -1 {
            let (cg, club) = retirement_club(r.club);
            story_lang!(buf, 0xa6d900, cg, cg, ng, ng, club, nation, name);
        } else {
            story_lang!(buf, 0xa6d870, ng, ng, nation, name);
        }
    } else {
        story_lang!(buf, 0xa6e164, name);
    }
}

/// The body of a sudden retirement (slot 5 clear): reason 0x10 he cannot support himself (age under 27, 0xa6d7d4) or
/// his family (0xa6d734); reason 0x20 with a job 10, 8, 6, 9, 1, 2, 3 or 4: the club must find a new one (0xa6d65c, the
/// job's name; without a club 0xa6d5f0); otherwise a staff member with a job (slot 13, the job not 0, 0x10 or 0xb): an
/// international capped by a nation, at a club by his club job (`staff::JOB_FOR_CLUB` 0xc manager 0xa6d288, 0xd
/// assistant 0xa6d374, 0xf coach 0xa6d468, other 0xa6d550), without a club (0xc and 0xd 0xa6d038, 0xf 0xa6d0fc, other
/// 0xa6d1b4); a former international by his club job (0xc 0xa6cd78, 0xd 0xa6ce34, 0xf 0xa6cef8, other 0xa6cfb0); not
/// an international, with a club 0xa6ccb0 (the club and the job's name), else 0xa6cbf8; anyone else: an international
/// capped by a nation 0xa6d550, a former international 0xa6cfb0, else 0xa6cb84.
unsafe fn retirement_sudden_body(r: &RetirementSlots, name: usize, gender: &mut i8) {
    let buf = NEWS_TEXT_BUFFER;
    let club_job = || r.person().job_for_club();
    // "<country> International ... has announced his immediate retirement"
    let capped = |nation: i32| {
        let (ng, n) = retirement_nation(nation);
        story_lang!(buf, 0xa6d550, ng, ng, n, name);
    };
    if r.reason == 0x10 {
        story_lang!(buf, if r.age < 0x1b { 0xa6d7d4 } else { 0xa6d734 }, name);
    } else if r.reason == 0x20 && matches!(r.job, 0xa | 8 | 6 | 9 | 1 | 2 | 3 | 4) {
        let job = exe::database_get_job_name(r.job as u32, 0, 0, gender);
        if r.club != -1 {
            let (cg, club) = retirement_club(r.club);
            let jg = *gender as i32;
            story_lang!(buf, 0xa6d65c, cg, cg, jg, jg, name, club, job);
        } else {
            story_lang!(buf, 0xa6d5f0, name);
        }
    } else if r.staff_job && !matches!(r.job, 0 | 0x10 | 0xb) {
        if r.international {
            if r.nation != -1 {
                if r.club != -1 {
                    let tpl = match club_job() {
                        0xc => 0xa6d288, // "... as <club> Manager."
                        0xd => 0xa6d374, // "... as <club> Assistant Manager."
                        0xf => 0xa6d468, // "... as <club> Coach."
                        _ => return capped(r.nation),
                    };
                    let (ng, nation) = retirement_nation(r.nation);
                    let (cg, club) = retirement_club(r.club);
                    story_lang!(buf, tpl, ng, ng, cg, cg, nation, name, club);
                } else {
                    story_lang!(buf, match club_job() {
                        0xc | 0xd => 0xa6d038,
                        0xf => 0xa6d0fc,
                        _ => 0xa6d1b4,
                    }, name);
                }
            } else {
                story_lang!(buf, match club_job() {
                    0xc => 0xa6cd78,
                    0xd => 0xa6ce34,
                    0xf => 0xa6cef8,
                    _ => 0xa6cfb0,
                }, name);
            }
        } else {
            let job = exe::database_get_job_name(r.job as u32, 0, 0, gender);
            let jg = *gender as i32;
            if r.club != -1 {
                let (cg, club) = retirement_club(r.club);
                story_lang!(buf, 0xa6ccb0, cg, cg, jg, jg, name, club, job);
            } else {
                story_lang!(buf, 0xa6cbf8, jg, jg, name, job);
            }
        }
    } else if r.international {
        if r.nation != -1 {
            capped(r.nation);
        } else {
            story_lang!(buf, 0xa6cfb0, name);
        }
    } else {
        story_lang!(buf, 0xa6cb84, name);
    }
}

/// The body of a retirement through a recurring injury (reason 8): an international capped by a nation, at club id 1
/// the plain text (the original compares the club id with 1, not −1), else with a club job (`staff::JOB_FOR_CLUB` not
/// 0, 0x10 or 0xb) 0xa6ca34 (the job's name) or 0xa6c958; a former international 0xa6c88c with the nation record
/// before the table (id −1, as the original); not an international, with a club job (`staff::CLUB_JOB`) 0xa6c7d4 with
/// the club of slot 6, else the plain text 0xa6c744.
unsafe fn retirement_injury_body(r: &RetirementSlots, name: usize, gender: &mut i8) {
    let buf = NEWS_TEXT_BUFFER;
    if r.international {
        if r.nation != -1 {
            if r.club == 1 {
                // the original's test (1 for −1) falls through to the plain text
                story_lang!(buf, 0xa6c744, name);
            } else {
                let (ng, nation) = retirement_nation(r.nation);
                let (cg, club) = retirement_club(r.club);
                if !matches!(r.person().job_for_club(), 0 | 0x10 | 0xb) {
                    let job = exe::database_get_job_name(r.job as u32, 0, 0, gender);
                    let jg = *gender as i32;
                    story_lang!(buf, 0xa6ca34, cg, cg, ng, ng, jg, jg, cg, cg, club, nation, name, job, club);
                } else {
                    story_lang!(buf, 0xa6c958, ng, ng, cg, cg, nation, club, name);
                }
            }
        } else {
            let (ng, nation) = retirement_nation(-1);
            story_lang!(buf, 0xa6c88c, ng, ng, nation, name);
        }
    } else if !r.person().club_job().is_null() {
        let (cg, club) = retirement_club(r.club);
        story_lang!(buf, 0xa6c7d4, cg, cg, club, name);
    } else {
        story_lang!(buf, 0xa6c744, name);
    }
}

/// Builds the headline or body text of the news that a person retires (news item (4, 0xbba)).
///
/// `player_regen_write_retirement_news_text(news, body, out)`: the slots in `RetirementSlots`. Headline: the short name
/// (`exe::database_format_name_from_ids`, 0x190 bytes, style 5): reason 8 "retires due to injury" 0xa6e1b8, else an
/// announced retirement "retires" 0xa6e224, else "Shock retirement of" 0xa6e1f0. Body (the full name, style 1): reason
/// 8 `retirement_injury_body`; otherwise the plain text 0xa6e164 is written first, then an announced retirement
/// `retirement_announced_body` or a sudden one `retirement_sudden_body` replaces it. The job names write their gender
/// into the byte that held slot 5 (12 for the body of a sudden or an injury retirement): the coaching job name only
/// writes it in language 4, so elsewhere the template gets that slot's byte. The text goes into the shared buffer
/// 0xde1f64 and the Text (`exe::text_set_string`); the name links to the person (0x7d7) when the name ids still match
/// (`exe::database_staff_matches_names`).
/// Original: `player_regen_write_retirement_news_text` (0x7a9650, player_regen.cpp, stdcall, ret 0xc). Defects of the
/// original: an injured international with a nation at club id 1 gets the plain text (the test compares with 1, not
/// −1); a former injured international reads the gender and name of the nation record before the table (id −1); the
/// club texts read club id −1's record when slot 6 is −1.
pub(crate) unsafe extern "stdcall" fn player_regen_write_retirement_news_text(news: usize, body: u32, out: usize) {
    let r = RetirementSlots::read(news);
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let buf = NEWS_TEXT_BUFFER;
    if body == 0 {
        exe::database_format_name_from_ids(r.names[0], r.names[1], r.names[2], name.as_mut_ptr(), 0x190, 5, r.id);
        covered!("sxB_1 retirement headline");
        let tpl = if r.reason == 8 {
            0xa6e1b8 // "... retires due to injury"
        } else if r.announced != 0 {
            0xa6e224 // "... retires"
        } else {
            0xa6e1f0 // "Shock retirement of ..."
        };
        story_lang!(buf, tpl, nm);
    } else {
        exe::database_format_name_from_ids(r.names[0], r.names[1], r.names[2], name.as_mut_ptr(), 0x190, 1, r.id);
        covered!("sxB_1 retirement body");
        if r.reason == 8 {
            let mut gender = r.age; // the byte that held slot 12
            retirement_injury_body(&r, nm, &mut gender);
        } else {
            story_lang!(buf, 0xa6e164, nm);
            if r.announced != 0 {
                let mut gender = r.announced as i8; // the byte that held slot 5
                retirement_announced_body(&r, nm, &mut gender);
            } else {
                let mut gender = r.age;
                retirement_sudden_body(&r, nm, &mut gender);
            }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(r.person(), r.names[0], r.names[1], r.names[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, r.person());
    }
}

/// Comparer for `player_regen_write_retirement_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_write_retirement_news_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(news, body, out), |t| player_regen_write_retirement_news_text(news, body, t),
        || format!("player_regen_write_retirement_news_text(body {}, reason {:#x})", body, exe::newsitem_get_slot(news, 4) as u8));
}
