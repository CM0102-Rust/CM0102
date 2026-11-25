// ---------- rx11b_4: one line of a person's history on the staff history screen (staff_records.cpp) ----------

/// The record of a staff competition (an award: `G_STAFF_COMP_TABLE`, 0x65 bytes; its name at +4 and gender at +0x37).
unsafe fn staff_comp_record(index: i32) -> usize {
    read_usize(G_STAFF_COMP_TABLE).wrapping_add((index as usize).wrapping_mul(0x65))
}

/// A history node's fields: the kind byte at +0xc and the data after it.
struct HistoryNode(usize);

impl HistoryNode {
    unsafe fn kind(&self) -> u8 {
        read_u8(self.0 + 0xc)
    }
    unsafe fn byte(&self, at: usize) -> u8 {
        read_u8(self.0 + at)
    }
    unsafe fn word(&self, at: usize) -> u16 {
        read_u16(self.0 + at)
    }
    unsafe fn dword(&self, at: usize) -> i32 {
        read_i32(self.0 + at)
    }
}

/// The text of a season (CRT sprintf into `buf`): "<first>/<second mod 100>" ("%d/%02d") when `split`, else "<year>".
unsafe fn history_season_text(buf: usize, split: bool, first: i32, second: i32, year: i32) {
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if split {
        crt_format(buf, 0x988070, first, second % 100); // "%d/%02d"
    } else {
        crt_format(buf, 0x98806c, year); // "%d"
    }
}

/// A fee as text (100 bytes): `Cash::get_display_amount` (flags 5) and `cash_format_money` (flags 5).
unsafe fn history_fee_text(buf: usize, fee: i32) {
    let cash = [fee as u32];
    let amount = exe::cash_get_display_amount(cash.as_ptr(), 5);
    exe::cash_format_money(buf, 0x64, amount, 5);
}

/// The name of a job (`database_get_job_name`, its gender into `gender`).
unsafe fn history_job_name(job: u8, gender: &mut i8) -> usize {
    exe::database_get_job_name(job as u32, 0, 0, gender)
}

/// The person's link (0x7d7) on `name` when the name ids still match the person (`database_staff_matches_names`).
unsafe fn history_person_link(out: usize, name: usize, id: i32, ids: (u32, u32, u32)) {
    let person = staff_at(id as isize);
    if exe::database_staff_matches_names(person, ids.0, ids.1, ids.2, 0usize) != 0 {
        exe::text_add_link(out, name as *const u8, 0x7d7, person);
    }
}

/// The club's link (0x7d5) on its short name.
unsafe fn history_club_link(out: usize, club: Club) {
    exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
}

/// A club's short name and its gender, sign-extended.
unsafe fn history_club(index: i32) -> (Club, usize, i32) {
    let club = club_at(index as isize);
    (club, club.addr() + club::SHORT_NAME, club.short_gender_name() as i8 as i32)
}

/// Writes one line of a person's history into `out` by the node's kind (+0xc); an unknown kind leaves `out` as it is.
///
/// The lines (langlib_sprintf into the shared buffer, then `Text::set_string`), with the node's fields:
/// - 0x96, at a club (+8): hired as manager (+0xd = 1), resigned (0), sacked (other); the club's long name;
/// - 0x97, a manager the person's club took (+0x11 id, +0x15..+0x1d name ids): hired, resigned or sacked;
/// - 0, 1, 2, 5, a competition (+0xd): winners, runners up, third placed, champions;
/// - 0x34 and 0x32, an award of a season (+0xd award, +0x11 year, +0x15 span, for 0x34 +0x19 the place: 1 won, 2
///   runner up, else third; 0x32 "named in"), the season "<year − span>/<year>" (0x34) or "<year>/<year + span>" (0x32),
///   or the year alone without a span; 0x33, an award of a month (+0x11, `date_get_month_name`); 0x35, the club's (+8)
///   supporters player of the year (+0xd year, +0x11 split season: "<year>/<year + 1>");
/// - 6, first international cap against a nation (+0xd) at an age (+0x11);
/// - 3, 4 and 7, qualified for, knocked out in, failed to qualify from a round (+0xd competition or −1, +0x11 stage
///   and +0x15 round words; 7 the stage only, `comp_util_format_comp_stage_round_label` form 2 and its gender);
/// - 8, 9, promotion from and relegation from a league (+0xd);
/// - 0xa, finished in a position (+0x15, `utils_format_ordinal`) in a league (+0xd, +0x11 stage, form 1);
/// - 0x64, a person signed (+0xd id, +0x11..+0x19 name ids, +0x1d fee, +0x21 part exchange, +0x25 club or −1, +0x29 job):
///   a player for a fee (bought, part exchange or not), in part exchange, from a club or on a free transfer; another job
///   hired as that job;
/// - 0x65, a person who left (same fields): lost to a club as a job, sold for a fee, in part exchange, released to a
///   club or on a free transfer;
/// - 0x66, the person joined a club (+0xd or −1, +0x11 fee, +0x15 part exchange, +0x19 job): bought by for a fee, in part
///   exchange, signed as a job, on a free transfer; with no club, released on a free transfer;
/// - 0x67 released on a free transfer; 0x68 released a person (+0xd id, +0x11.. name ids); 0x69 loaned a person (+0xd id,
///   +0x11.. name ids, +0x1d club) from a club; 0x6a loaned from a club (+0xd).
/// The links: the club (0x7d5) on its short name, a competition (0x7d0) or a round on its text, an award (0x7d4), the
/// supporters' award (0x7e6) to the club, a person (0x7d7) when the name ids still match. Genders sign-extended, twice
/// each, in the original's order; the fees by `Cash::get_display_amount` and `cash_format_money` (flags 5).
/// Defect of the original (`DEFEITOS_DO_ORIGINAL`): kinds 8 and 9 with no league (−1) read the gender byte at address
/// 0x37 and the game crashes; the copy writes the line with an empty name and no link.
/// Original: `Staff_Records::format_staff_history_entry` (0x86e0b0, staff_records.cpp, thiscall, ret 8; this is not
/// read).
pub(crate) unsafe extern "thiscall" fn format_staff_history_entry(_this: usize, node: usize, out: usize) {
    let n = HistoryNode(node);
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    let kind = n.kind();
    let set = |out: usize| {
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    };
    match kind {
        0x96 => {
            let club = club_at(n.dword(8) as isize);
            let g = club.gender_name() as i8 as i32;
            let template = match n.byte(0xd) { 1 => 0xa8499c, 0 => 0xa84960, _ => 0xa84924 }; // hired, resigned, sacked
            fmt(buf, template, g, g, club.addr() + club::NAME);
            set(out);
            history_club_link(out, club);
        }
        0x97 => {
            let ids = (n.dword(0x15) as u32, n.dword(0x19) as u32, n.dword(0x1d) as u32);
            let mut name = [0u8; 0x1f4];
            let np = name.as_mut_ptr() as usize;
            exe::database_format_name_from_ids(ids.0, ids.1, ids.2, name.as_mut_ptr(), 0x1f4, 1, n.dword(0x11) as u32);
            let template = match n.byte(0xd) { 1 => 0xa848ec, 0 => 0xa848b0, _ => 0xa84874 }; // hired, resigned, sacked
            fmt(buf, template, np);
            set(out);
            history_person_link(out, np, n.dword(0x11), ids);
        }
        0 | 1 | 2 | 5 => {
            let comp = club_comp_record(n.dword(0xd) as usize);
            let g = read_i8(comp + 0x37) as i32; // the competition's gender
            let template = match kind { 0 => 0xa84848, 1 => 0xa84818, 2 => 0xa847e8, _ => 0xa847b0 }; // winners .. champions
            fmt(buf, template, g, g, comp + 4);
            set(out);
            exe::text_add_link(out, (comp + 4) as *const u8, 0x7d0, comp);
        }
        0x34 | 0x32 | 0x33 => history_award_line(&n, out),
        0x35 => {
            let mut season = [0u8; 0x20];
            let sp = season.as_mut_ptr() as usize;
            let year = n.word(0xd) as i32;
            history_season_text(sp, n.dword(0x11) != 0, year, year + 1, year);
            let mut award = [0u8; 0x190];
            let ap = award.as_mut_ptr() as usize;
            fmt(ap, 0xa84590); // "Supporters Player of the Year"
            fmt(buf, 0xa84534, sp, ap);
            set(out);
            exe::text_add_link(out, award.as_ptr(), 0x7e6, club_at(n.dword(8) as isize));
        }
        6 => {
            let (club, short, g) = history_club(n.dword(0xd));
            fmt(buf, 0xa844d4, g, g, short, n.byte(0x11) as i8 as i32); // "First international cap against .. aged .."
            set(out);
            history_club_link(out, club);
        }
        3 | 4 | 7 => {
            let index = n.dword(0xd);
            let comp = if index != -1 { club_comp_record(index as usize) } else { 0 };
            let (stage, round) = if kind == 7 { (n.word(0x11) as u32, 0) } else { (n.word(0x11) as u32, n.word(0x15) as u32) };
            let mut label = [0u8; 0x190];
            exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0x190);
            let g = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
            let template = match kind { 3 => 0xa8448c, 4 => 0xa84440, _ => 0xa843f0 }; // qualified, knocked out, failed
            fmt(buf, template, g, g, label.as_ptr() as usize);
            set(out);
            if comp != 0 {
                exe::text_add_link(out, label.as_ptr(), 0x7d0, comp);
            }
        }
        8 | 9 => {
            let index = n.dword(0xd);
            let template = if kind == 8 { 0xa843ac } else { 0xa84368 }; // promotion from, relegation from
            if index == -1 {
                fmt(buf, template, 0, 0, b"\0".as_ptr() as usize); // the original crashes here
                set(out);
                return;
            }
            let comp = club_comp_record(index as usize);
            let g = read_i8(comp + 0x37) as i32;
            fmt(buf, template, g, g, comp + 4);
            set(out);
            exe::text_add_link(out, (comp + 4) as *const u8, 0x7d0, comp);
        }
        0xa => {
            let comp = club_comp_record(n.dword(0xd) as usize);
            let stage = n.word(0x11) as u32;
            let mut label = [0u8; 0xc8];
            exe::comp_util_format_comp_stage_round_label(comp, stage, 0, 1, label.as_mut_ptr(), 0xc8);
            let g = exe::comp_util_get_label_gender(comp as u32, stage, 0, 1) as i8 as i32;
            let position = exe::utils_format_ordinal(n.byte(0x15) as i8 as i32, -1);
            fmt(buf, 0xa84310, g, g, position, label.as_ptr() as usize); // "Finished <position> in <league>"
            set(out);
            exe::text_add_link(out, label.as_ptr(), 0x7d0, comp);
        }
        0x64 | 0x65 => history_transfer_line(&n, out),
        0x66 => history_joined_line(&n, out),
        0x67 => {
            fmt(buf, 0xa83c5c); // "Released on free transfer"
            set(out);
        }
        0x68 | 0x69 => {
            let id = n.dword(0xd);
            let ids = (n.dword(0x11) as u32, n.dword(0x15) as u32, n.dword(0x19) as u32);
            let mut name = [0u8; 0x12c];
            let np = name.as_mut_ptr() as usize;
            exe::database_format_name_from_ids(ids.0, ids.1, ids.2, name.as_mut_ptr(), 0x12c, 1, id as u32);
            if kind == 0x68 {
                fmt(buf, 0xa83e0c, np); // "Released <person> on free transfer"
                set(out);
                history_person_link(out, np, id, ids);
            } else {
                let (club, short, g) = history_club(n.dword(0x1d));
                fmt(buf, 0xa83c04, g, g, np, short); // "Loaned <person> from <club>"
                set(out);
                history_person_link(out, np, id, ids);
                history_club_link(out, club);
            }
        }
        0x6a => {
            let (club, short, g) = history_club(n.dword(0xd));
            fmt(buf, 0xa83bd0, g, g, short); // "Loaned from <club>"
            set(out);
            history_club_link(out, club);
        }
        _ => {}
    }
}

/// History kinds 0x34, 0x32 and 0x33: an award of a season or of a month (`format_staff_history_entry`).
unsafe fn history_award_line(n: &HistoryNode, out: usize) {
    let fmt = news_text_format();
    let kind = n.kind();
    let award = staff_comp_record(n.dword(0xd));
    let g = read_i8(award + 0x37) as i32; // the award's gender
    let name = award + 4;
    if kind == 0x33 {
        let month = exe::date_get_month_name(n.byte(0x11) as u32);
        fmt(NEWS_TEXT_BUFFER, 0xa845b0, g, g, name, month); // "<award> for <month>"
    } else {
        let mut season = [0u8; 0x20];
        let sp = season.as_mut_ptr() as usize;
        let (year, span) = (n.word(0x11) as i32, n.byte(0x15) as i8 as i32);
        if kind == 0x34 {
            history_season_text(sp, span != 0, year - span, year, year);
            let template = match n.byte(0x19) { 1 => 0xa84750, 2 => 0xa846e4, _ => 0xa84678 }; // won, runner up, third
            fmt(NEWS_TEXT_BUFFER, template, g, g, sp, name);
        } else {
            covered!("rx11b_4 named in a team of the season");
            history_season_text(sp, span != 0, year, year + span, year);
            fmt(NEWS_TEXT_BUFFER, 0xa84610, g, g, sp, name); // "Named in <season> <award>"
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, name as *const u8, 0x7d4, award);
}

/// History kinds 0x64 (a person signed) and 0x65 (a person left), `format_staff_history_entry`.
unsafe fn history_transfer_line(n: &HistoryNode, out: usize) {
    let fmt = news_text_format();
    let incoming = n.kind() == 0x64;
    let id = n.dword(0xd);
    let ids = (n.dword(0x11) as u32, n.dword(0x15) as u32, n.dword(0x19) as u32);
    let (fee, part_exchange, club_index, job) = (n.dword(0x1d), n.byte(0x21) != 0, n.dword(0x25), n.byte(0x29));
    let mut name = [0u8; 0x1f4];
    let np = name.as_mut_ptr() as usize;
    exe::database_format_name_from_ids(ids.0, ids.1, ids.2, name.as_mut_ptr(), 0x1f4, 1, id as u32);
    let mut money = [0u8; 0x64];
    let mp = money.as_mut_ptr() as usize;
    if fee > 0 {
        history_fee_text(mp, fee); // formatted for any job, used only for a player
    }
    // the club is read only on the lines that name it (an index of -1 is never read, as in the original)
    let club = || history_club(club_index);
    let mut job_gender = 0i8;
    if incoming {
        if fee > 0 && job == 0xb {
            covered!("rx11b_4 bought for a fee");
            let (_, short, g) = club();
            fmt(NEWS_TEXT_BUFFER, if part_exchange { 0xa84280 } else { 0xa84200 }, g, g, np, short, mp); // "Bought .. from .. for .."
        } else if part_exchange {
            let (_, short, g) = club();
            fmt(NEWS_TEXT_BUFFER, 0xa84198, g, g, np, short); // "Bought .. from .. (part exchange)"
        } else if job != 0xb {
            let title = history_job_name(job, &mut job_gender);
            let gj = job_gender as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa84144, gj, gj, np, title); // "Hired <person> as <job>"
        } else if club_index != -1 {
            let (_, short, g) = club();
            fmt(NEWS_TEXT_BUFFER, 0xa840dc, g, g, np, short); // "Signed .. from .. in free transfer"
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa8409c, np); // "Signed .. in free transfer"
        }
    } else if job != 0xb {
        let title = history_job_name(job, &mut job_gender);
        let gj = job_gender as i32;
        let (_, short, g) = club();
        fmt(NEWS_TEXT_BUFFER, 0xa84024, g, g, gj, gj, np, short, title); // "Lost .. to .. as <job>"
    } else if fee > 0 {
        covered!("rx11b_4 sold for a fee");
        let (_, short, g) = club();
        fmt(NEWS_TEXT_BUFFER, if part_exchange { 0xa83f98 } else { 0xa83f1c }, g, g, np, short, mp); // "Sold .. to .. for .."
    } else if part_exchange {
        let (_, short, g) = club();
        fmt(NEWS_TEXT_BUFFER, 0xa83eb8, g, g, np, short); // "Sold .. to .. (part exchange)"
    } else if club_index != -1 {
        let (_, short, g) = club();
        fmt(NEWS_TEXT_BUFFER, 0xa83e50, g, g, np, short); // "Released .. to .. in free transfer"
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa83e0c, np); // "Released .. on free transfer"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    history_person_link(out, np, id, ids);
    if club_index != -1 {
        history_club_link(out, club_at(club_index as isize));
    }
}

/// History kind 0x66: the person joined a club (`format_staff_history_entry`).
unsafe fn history_joined_line(n: &HistoryNode, out: usize) {
    let fmt = news_text_format();
    let club_index = n.dword(0xd);
    let (fee, part_exchange, job) = (n.dword(0x11), n.byte(0x15) != 0, n.byte(0x19));
    if club_index == -1 {
        fmt(NEWS_TEXT_BUFFER, 0xa83c5c); // "Released on free transfer"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        return;
    }
    let (club, short, g) = history_club(club_index);
    if fee > 0 && job == 0xb {
        let mut money = [0u8; 0x64];
        let mp = money.as_mut_ptr() as usize;
        history_fee_text(mp, fee);
        fmt(NEWS_TEXT_BUFFER, if part_exchange { 0xa83da4 } else { 0xa83d4c }, g, g, short, mp); // "Bought by .. for .."
    } else if part_exchange {
        fmt(NEWS_TEXT_BUFFER, 0xa83d0c, g, g, short); // "Bought by .. in part exchange"
    } else if job != 0xb {
        let mut job_gender = 0i8;
        let title = history_job_name(job, &mut job_gender);
        let gj = job_gender as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa83cb8, g, g, gj, gj, short, title); // "Signed by .. as <job>"
    } else {
        fmt(NEWS_TEXT_BUFFER, 0xa83c78, g, g, short); // "Signed by .. on free transfer"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    history_club_link(out, club);
}

/// Comparer for `format_staff_history_entry`: the original into the caller's Text, the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_format_staff_history_entry(this: usize, node: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, node, out), |t| format_staff_history_entry(this, node, t),
        || format!("format_staff_history_entry(kind {:#x})", read_u8(node + 0xc)));
}
