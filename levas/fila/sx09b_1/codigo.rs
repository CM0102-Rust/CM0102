// --- sx09b_1: the news item of a new ban (discipline.cpp) ---

/// The comparer's dry run of `post_ban_news`: the copy builds the news item and keeps each send instead of sending it.
pub(crate) static BAN_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// The sends the dry run kept: (the club, or 0 for every human manager; the news item's first `news_item::ID` bytes).
pub(crate) static BAN_NEWS_SENT: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

/// Sends the ban news item to a club's manager (`exe::news_send_to_club_manager`), or to every human manager when
/// `club` is null (`exe::news_send_to_all_managers`); in the dry run, keeps it.
unsafe fn ban_news_send(n: usize, club: Club) {
    if BAN_NEWS_DRY_RUN.load(Relaxed) {
        let bytes = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).push((club.addr(), bytes));
        return;
    }
    if club.is_null() {
        exe::news_send_to_all_managers(read_usize(G_NEWS), n);
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Writes value `v` into slot `*k` of news item `n` and moves to the next slot (the original's byte counter).
unsafe fn ban_news_slot(n: usize, k: &mut u8, v: u32) {
    exe::newsitem_set_slot(n, *k as i8, v);
    *k = k.wrapping_add(1);
}

/// Posts news item 2002 (0x7d2) that a person got a ban, and picks who receives it.
///
/// `post_ban_news(person, ban, fixture)` (the Discipline object in ecx is not read; SEH frame for the local news item
/// and records, no destructor). First, with the transfer manager, the clubs it picks for the player's news
/// (`exe::transfer_manager_collect_player_news_recipients` with 0xda, up to 30; none without it). The slots, in order:
/// 0 = 0 (the appeal made, set later by the appeal); the fixture's `.referee()` (−1 without a fixture); the ban's
/// `.matches()` (signed byte) and `.days()`; the start date (day signed, year unsigned) when it is still to come (today
/// `G_CURRENT_DATE` before it: an earlier year, or the same year and an earlier day), otherwise 0 and 0; the four rules
/// (`ban_record::RULES`, signed bytes); `.reason()` (signed) and `.reason_amount()`; bit 1 of `.flags()` (valid in
/// every competition), bit 2 (extended by a pending ban) and bit 0 (added to an existing ban); today's day (signed) and
/// year. Then the person's reference (`exe::fill_staff_name_ref` into a record whose date starts as `Date::Date`,
/// written by `exe::write_staff_name_ref_to_news`). With a fixture: slot 1, and the person's club in the match, the
/// `.national_job()` when the home side is a national team (`exe::database_is_national_team`), otherwise the
/// `.club_job()`; when that club is neither the home nor the away club, the routine ends there and nothing is sent;
/// otherwise its match record (`exe::fill_match_record_from_fixture`, written by `exe::write_match_record_to_news`).
/// Without a fixture: slot 0. Who gets it: a ban of more than 50 days (`.days()`, signed; a ban by matches has −1)
/// goes to every human manager (`exe::news_send_to_all_managers`) when the player's `.world_reputation()` is above
/// 8000, otherwise to the clubs picked at the start; a shorter ban only to the person's club when it has a human
/// manager (`exe::news_club_has_human_manager`).
/// Original: `Discipline::post_ban_news` (0x557280, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_ban_news(_this: usize, person: usize, ban: usize, fixture: usize) {
    let person = Staff(person);
    let ban = BanRecord(ban);
    let fixture = Fixture(fixture);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut recipients = [0usize; 0x1e];
    let count = if tm != 0 { exe::transfer_manager_collect_player_news_recipients(tm, person, 0xda, recipients.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x7d2, 0);
    let mut name = [0u8; 0x18]; // the person's reference: four name ids and the birth date
    name[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    exe::fill_staff_name_ref(person, name.as_mut_ptr() as usize);
    let mut k = 0u8;
    ban_news_slot(n, &mut k, 0); // sx09b_1: the appeal made
    ban_news_slot(n, &mut k, if fixture.is_null() { u32::MAX } else { fixture.referee() as i32 as u32 });
    ban_news_slot(n, &mut k, ban.matches() as i8 as i32 as u32); // sx09b_1: the matches
    ban_news_slot(n, &mut k, ban.days() as i32 as u32);
    let today_day = read_i16(G_CURRENT_DATE);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    let start_day = ban.start_date();
    let start_year = read_u16(ban.addr() + ban_record::START_DATE + 2); // the start date's year
    if today_year < start_year || (today_year == start_year && today_day < start_day) { // sx09b_1: the start still to come
        covered!("sx09b_1 ban news with a later start");
        ban_news_slot(n, &mut k, start_day as i32 as u32);
        ban_news_slot(n, &mut k, start_year as u32);
    } else {
        ban_news_slot(n, &mut k, 0);
        ban_news_slot(n, &mut k, 0);
    }
    for j in 0..4 {
        ban_news_slot(n, &mut k, read_i8(ban.addr() + ban_record::RULES + j) as i32 as u32);
    }
    ban_news_slot(n, &mut k, ban.reason() as i8 as i32 as u32);
    ban_news_slot(n, &mut k, ban.reason_amount() as i32 as u32);
    let flags = ban.flags() as u32;
    ban_news_slot(n, &mut k, (flags >> 1) & 1);
    ban_news_slot(n, &mut k, (flags >> 2) & 1);
    ban_news_slot(n, &mut k, flags & 1);
    ban_news_slot(n, &mut k, today_day as i32 as u32);
    ban_news_slot(n, &mut k, today_year as u32);
    exe::write_staff_name_ref_to_news(name.as_mut_ptr() as usize, n, &mut k);
    if fixture.is_null() {
        ban_news_slot(n, &mut k, 0);
    } else {
        ban_news_slot(n, &mut k, 1);
        let mut m = BanMatch([0u8; 0x20]);
        m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
        let club = if exe::database_is_national_team(fixture.home_club()) != 0 { person.national_job() } else { person.club_job() };
        if club != fixture.home_club() && club != fixture.away_club() {
            covered!("sx09b_1 ban news of a club not in the match");
            return;
        }
        exe::fill_match_record_from_fixture(fixture, club, m.0.as_mut_ptr() as usize);
        exe::write_match_record_to_news(m.0.as_mut_ptr() as usize, n, &mut k);
    }
    if ban.days() > 0x32 {
        let sheet = person.player();
        if !sheet.is_null() && sheet.world_reputation() > 0x1f40 {
            covered!("sx09b_1 ban news to every manager");
            ban_news_send(n, Club(0));
        } else {
            covered!("sx09b_1 ban news to the picked clubs");
            for &c in recipients.iter().take(count.max(0) as usize) {
                ban_news_send(n, Club(c));
            }
        }
    } else {
        let club = person.club_job();
        if !club.is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            covered!("sx09b_1 ban news to the club");
            ban_news_send(n, club); // sx09b_1: the club gets it
        }
    }
}

/// The human inboxes' ring positions (entry address, ring start, next slot), or none without the news object.
pub(crate) unsafe fn human_inbox_marks() -> Vec<(usize, i32, i32)> {
    let news = read_usize(G_NEWS);
    if news == 0 || read_usize(news + news_fields::HUMAN_INBOXES) == 0 {
        return Vec::new();
    }
    (0..0x10).map(|i| read_usize(news + news_fields::HUMAN_INBOXES) + i * 12).map(|e| (e, read_i32(e), read_i32(e + 4))).collect()
}

/// The news items of type `kind` that reached the human inboxes since `before` (`human_inbox_marks`): the first
/// `news_item::ID` bytes of each, without `news_item::IMPORTANT` (the delivery sets it), sorted.
pub(crate) unsafe fn delivered_news_items(before: &[(usize, i32, i32)], kind: u32) -> Vec<Vec<u8>> {
    let news = read_usize(G_NEWS);
    let mut out = Vec::new();
    for &(e, start, next) in before {
        let (start_now, next_now) = (read_i32(e), read_i32(e + 4));
        if start_now == -1 || (start_now == start && next_now == next) {
            continue;
        }
        let mut k = if start_now == start { next } else { start_now };
        while k != next_now {
            let r = read_usize(news).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            if read_u32(r) == kind {
                let mut bytes: Vec<u8> = (0..news_item::ID).map(|j| read_u8(r + j)).collect();
                bytes[news_item::IMPORTANT] = 0;
                out.push(bytes);
            }
            k += 1;
            if k == start_now + 0x64 {
                k = start_now;
            }
        }
    }
    out.sort();
    out
}

/// Comparer for `post_ban_news`, as a dry run: the copy runs first without sending (it keeps each send), then the
/// original from the same random generator state. Compares the generator's state and the ban news items that reached
/// the human managers' inboxes with the copy's sends: one per club with a human manager, and for the send to every
/// manager one per human manager (the last 16 people with `.classification()` 1, as `News::send_to_all_managers`).
pub(crate) unsafe extern "thiscall" fn compare_post_ban_news(this: usize, person: usize, ban: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%);
    let inboxes = human_inbox_marks();
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).clear();
    BAN_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| post_ban_news(this, person, ban, fixture)));
    BAN_NEWS_DRY_RUN.store(false, Relaxed);
    let rng_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let sent = core::mem::take(&mut *BAN_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()));
    rng.write_back();
    o(this, person, ban, fixture);
    let rng_original = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let staff_count = read_i32(G_STAFF_COUNT);
    let humans = (0..0x10).filter(|&i| staff_at((staff_count - 0x10 + i) as isize).classification() == 1).count();
    let mut expected: Vec<Vec<u8>> = Vec::new();
    for (c, b) in &sent {
        let mut b = b.clone();
        b[news_item::IMPORTANT] = 0;
        let copies = if *c == 0 {
            humans
        } else {
            let m = Club(*c).manager();
            (!m.is_null() && read_i32(m.addr()) >= staff_count.wrapping_sub(0x10)) as usize
        };
        expected.extend(core::iter::repeat(b).take(copies));
    }
    expected.sort();
    let delivered = delivered_news_items(&inboxes, 0x7d2);
    let same = rng_original == rng_rust && (inboxes.is_empty() || delivered == expected);
    SCOREBOARD.record(same, || {
        format!(
            "post_ban_news(person {:#x}, ban {:#x}, fixture {:#x}) rng same {}; {} sends, human items original {:x?} rust {:x?}",
            person,
            ban,
            fixture,
            rng_original == rng_rust,
            sent.len(),
            delivered,
            expected
        )
    });
}
