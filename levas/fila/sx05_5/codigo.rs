/// The slots of an AWOL-family news item, read in order 0..`n` (`exe::newsitem_get_slot`, as the original does).
unsafe fn awol_news_slots<const N: usize>(news: usize) -> [u32; N] {
    let mut s = [0u32; N];
    for (k, v) in s.iter_mut().enumerate() {
        *v = exe::newsitem_get_slot(news, k as u32);
    }
    s
}

/// A person's name from the three name ids of a news item (`exe::database_format_name_from_ids`, 0x190 bytes) in the
/// given style.
unsafe fn awol_news_name(ids: [u32; 3], id: u32, style: u32) -> [u8; 0x190] {
    let mut name = [0u8; 0x190];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    name
}

/// Links a person's name when his names still match the news item's ids.
unsafe fn awol_link_person(out: usize, name: &[u8], ids: [u32; 3], id: u32) {
    let person = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, person);
    }
}

/// The club whose club id is the screen's human manager's club job, as the AWOL texts test it.
unsafe fn awol_is_human_club(club_id: u32) -> bool {
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize));
    !manager.is_null() && !manager.club_job().is_null() && read_u32(manager.club_job().addr()) == club_id
}

/// A club's short name and the gender byte of it (sign-extended, as the texts pass it).
unsafe fn awol_club(id: u32) -> (usize, u32) {
    let club = club_at(id as i32 as isize);
    (club.addr() + club::SHORT_NAME, read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32 as u32)
}

/// A nation's short name and the gender byte of it.
unsafe fn awol_nation(id: u32) -> (usize, u32) {
    let nation = read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE));
    (nation + nation::SHORT_NAME, read_i8(nation + nation::SHORT_GENDER_NAME) as i32 as u32)
}

/// Writes the headline or the body of the AWOL news (slots: 0 the player, 1..3 his name ids, 4 another club, 5 his
/// club, 6 the kind, 7 days away, 8 the nation he went to).
///
/// The headline (name in style 5) by kind: 0 "<player> returns to <club>", 1 "misses training" when the player's id is
/// a multiple of 3, else "goes AWOL", 2 by the id modulo 3 "in transfer row", "misses training" or "goes AWOL", 4
/// "fails to return from leave"; the club's link. The body (style 1) differs when the player's club is the screen's
/// human manager's: kind 0 he is back (and the squad is unhappy), 1 he refuses to play and wants a move, 2 he missed
/// training after being denied a move to the other club, 4 he failed to return from leave: to the nation, with
/// "missed his flight" under 7 days away, or the agent's message with a nation, or without one; the other managers
/// read the same with the club's name first. Then the player's link and the two clubs' links (long name).
/// Defect of the original: kind 3 (and above 4) formats nothing, so the text is whatever the shared buffer 0xde1f64
/// last held; the headline reads slot 6 twice and never slot 7.
/// Original: `write_awol_news_text` (0x419c60, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_awol_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = awol_news_slots(news);
        let kind = exe::newsitem_get_slot(news, 6);
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        let by_id = s[0] as i32 % 3;
        match kind {
            0 => {
                let (club, g) = awol_club(s[5]);
                // "<player> returns to <club>"
                fmt(buf, 0x98811c, g, g, np, club)
            }
            1 if by_id == 0 => fmt(buf, 0x9881d4, np), // "<player> misses training"
            2 if by_id == 0 => fmt(buf, 0x9881a0, np), // "<player> in transfer row"
            2 if by_id == 1 => fmt(buf, 0x9881d4, np),
            1 | 2 => fmt(buf, 0x988170, np), // "<player> goes AWOL"
            4 => fmt(buf, 0x988208, np), // "<player> fails to return from leave"
            _ => 0,
        };
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        if s[5] as i32 > -1 {
            exe::text_add_link(out, (club_at(s[5] as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(s[5] as i32 as isize));
        }
        return;
    }
    covered!("sx05_5 awol body");
    let s: [u32; 9] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (other, club, kind, days, nation) = (s[4], s[5], s[6], s[7] as i32, s[8]);
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let (cn, g) = awol_club(club);
    if awol_is_human_club(club) {
        covered!("sx05_5 awol at the human club");
        match kind {
            0 => fmt(buf, 0x988248, np),
            1 => fmt(buf, 0x988310, g, g, np, cn),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x9883e0, og, og, g, g, np, on, cn)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988698, ng, ng, g, g, np, nn, cn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x9885a4, ng, ng, np, nn)
            }
            4 => fmt(buf, 0x9884cc, np),
            _ => 0,
        };
    } else {
        match kind {
            0 => fmt(buf, 0x9887bc, g, g, cn, np),
            1 => fmt(buf, 0x9888b4, g, g, cn, np),
            2 => {
                let (on, og) = awol_club(other);
                fmt(buf, 0x988990, g, g, og, og, cn, np, on)
            }
            4 if days < 7 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988ccc, g, g, ng, ng, cn, np, nn)
            }
            4 if nation as i32 != -1 => {
                let (nn, ng) = awol_nation(nation);
                fmt(buf, 0x988b9c, g, g, ng, ng, cn, np, nn)
            }
            4 => fmt(buf, 0x988a88, g, g, cn, np),
            _ => 0,
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    for c in [other, club] {
        if c as i32 > -1 {
            exe::text_add_link(out, (club_at(c as i32 as isize).addr() + club::NAME) as *const u8, 0x7d5, club_at(c as i32 as isize));
        }
    }
}

/// Writes the headline or the body of the missed training news (slots: 0 the player, 1..3 his name ids, 4 his club,
/// 5 the kind, 6 the absent flag, 7 the coach who reports (-1 none), 8..10 the coach's name ids).
///
/// The headline (slots 0..3, 5 and 6; name in style 5): "<player> absent from training" with the absent flag, else
/// "rumoured to have missed training" for kind 0 or "disciplined by club"; the player's link. The body (style 1): at
/// the screen's human manager's club the chief coach (or the named coach) reports the player did not turn up (kind
/// 0), was absent (1) or failed to report (other); elsewhere the club's board is losing patience (no coach), or the
/// manager is losing patience (kind 0) or fined him two weeks' (1) or one week's wages (other). Then the player's
/// link, the coach's link and the club's link (long name). Slot 6 is read in the body and not used.
/// Original: `write_missed_training_news_text` (0x41a8e0, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_missed_training_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if full == 0 {
        let s: [u32; 7] = {
            let mut s = [0u32; 7];
            for k in [0u32, 1, 2, 3, 5, 6] {
                s[k as usize] = exe::newsitem_get_slot(news, k);
            }
            s
        };
        let ids = [s[1], s[2], s[3]];
        let name = awol_news_name(ids, s[0], 5);
        let np = name.as_ptr() as usize;
        if s[6] != 0 {
            fmt(buf, 0x988e98, np); // "<player> absent from training"
        } else if s[5] == 0 {
            fmt(buf, 0x988e54, np); // "<player> rumoured to have missed training"
        } else {
            fmt(buf, 0x988e20, np); // "<player> disciplined by club"
        }
        exe::text_set_string(out, buf as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 missed training body");
    let s: [u32; 11] = awol_news_slots(news);
    let ids = [s[1], s[2], s[3]];
    let (club, kind, coach) = (s[4], s[5], s[7] as i32);
    let coach_ids = [s[8], s[9], s[10]];
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let mut coach_name = [0u8; 0x190];
    if coach != -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
    }
    let cp = coach_name.as_ptr() as usize;
    if awol_is_human_club(club) {
        covered!("sx05_5 missed training at the human club");
        let (alone, with_coach) = match kind {
            0 => (0x989250, 0x9891d0),
            1 => (0x98933c, 0x9892c0),
            _ => (0x98942c, 0x9893a8),
        };
        if coach == -1 {
            fmt(buf, alone, np);
        } else {
            fmt(buf, with_coach, cp, np);
        }
    } else {
        let (cn, g) = awol_club(club);
        match (coach, kind) {
            (-1, _) => fmt(buf, 0x988ed0, g, g, 0x64, cn, np),
            (_, 0) => fmt(buf, 0x988f6c, g, g, 0x64, cn, cp, np),
            (_, 1) => fmt(buf, 0x989030, g, g, cn, cp, np),
            _ => fmt(buf, 0x989100, g, g, cn, cp, np),
        };
    }
    exe::text_set_string(out, buf as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    if coach != -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
    let c = club_at(club as i32 as isize);
    exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
}

/// Writes the headline or the body of the news that a coach worries about a player's fitness after he turned up late
/// for training (slots: 0 the player, 1 his club, 2 the worry flag, 3..5 his name ids, 6 the coach (-1 none), 7..9 the
/// coach's name ids). The text goes through a local buffer, not 0xde1f64.
///
/// The headline (style 5): "<player> fitness worry" with the flag, else "<player> lacking fitness"; the player's
/// link. The body (style 1): with a coach, "<club> coach <coach> has voiced concerns over <player>'s general fitness"
/// (flag) or "reported that <player> turned up late"; without one, "various members of the <club> backroom staff..."
/// (flag) or "<player>'s apparent lack of match fitness... has disappointed senior members of <club>'s playing staff".
/// Then the player's link, the club's link (short name) and the coach's link.
/// Original: `write_fitness_worry_news_text` (0x41b030, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_fitness_worry_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let mut text = [0u8; 0x1378];
    let tp = text.as_mut_ptr() as usize;
    let s: [u32; 10] = awol_news_slots(news);
    let (club, worry, coach) = (s[1], s[2], s[6] as i32);
    let ids = [s[3], s[4], s[5]];
    if full == 0 {
        let name = awol_news_name(ids, s[0], 5);
        if worry != 0 {
            fmt(tp, 0x989820, 0x64, name.as_ptr() as usize); // "<player>{} fitness worry"
        } else {
            fmt(tp, 0x9897ec, name.as_ptr() as usize); // "<player> lacking fitness"
        }
        exe::text_set_string(out, tp as *const u8);
        awol_link_person(out, &name, ids, s[0]);
        return;
    }
    covered!("sx05_5 fitness worry body");
    let name = awol_news_name(ids, s[0], 1);
    let np = name.as_ptr() as usize;
    let coach_ids = [s[7], s[8], s[9]];
    let (cn, g) = awol_club(club);
    let mut coach_name = [0u8; 0x190];
    if coach > -1 {
        coach_name = awol_news_name(coach_ids, coach as u32, 1);
        let cp = coach_name.as_ptr() as usize;
        if worry != 0 {
            fmt(tp, 0x989718, g, g, 0x64, cn, cp, np);
        } else {
            fmt(tp, 0x98963c, g, g, cn, cp, np);
        }
    } else if worry != 0 {
        fmt(tp, 0x98956c, g, g, 0x64, cn, np);
    } else {
        covered!("sx05_5 fitness worry without a coach");
        fmt(tp, 0x98949c, 0x64, g, g, np, cn);
    }
    exe::text_set_string(out, tp as *const u8);
    awol_link_person(out, &name, ids, s[0]);
    exe::text_add_link(out, cn as *const u8, 0x7d5, club_at(club as i32 as isize));
    if coach > -1 {
        awol_link_person(out, &coach_name, coach_ids, coach as u32);
    }
}

pub(crate) unsafe extern "thiscall" fn compare_write_awol_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_awol_news_text(this, news, full, t),
        || format!("write_awol_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_missed_training_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_missed_training_news_text(this, news, full, t),
        || format!("write_missed_training_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_fitness_worry_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_fitness_worry_news_text(this, news, full, t),
        || format!("write_fitness_worry_news_text(news {:#x}, full {})", news, full));
}
