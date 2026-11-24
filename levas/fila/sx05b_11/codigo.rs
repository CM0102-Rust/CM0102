// ---------- sx05b_11: a player goes AWOL and its news (awol.cpp) ----------

/// One of a player's reputation words cut by a random 0 to 40 percent: × (1 − draw(41) × 0.01), truncated.
unsafe fn awol_cut_reputation(word: usize) {
    let draw = exe::utils_random(0x29);
    let v = ftol((1.0 - draw as f64 * read_f64(rdata::F64_0_01)) * read_i16(word) as f64);
    write_i16(word, v as i16);
}

/// Sends a player AWOL. Nothing without a person, a player sheet, a staff id below `G_STAFF_COUNT`, or when his flag
/// already has bit 1 (`person_flags::ABSENT_WITHOUT_LEAVE`). Otherwise the bit is set and his current, home and world
/// reputations are each cut by a random 0 to 40 percent (`awol_cut_reputation`, one draw each, in that order). At his
/// club: he dislikes the manager; unless a human runs the club (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0)),
/// the manager dislikes him; each squad member but him dislikes him when a draw of his loyalty is not 0 and a draw of
/// his professionalism is 0; the chairman dislikes him (`exe::staff_add_disliked_staff`(who, whom, 1)). The absence
/// (12 bytes: his staff id and the return date, today plus 2 to 32 days for kind 4, else 21 to 170) goes to the
/// absences list. Then the news (`awol_post_news`) with `club` (or his own): kind 2 for kind 1, kind 4 with the days
/// for kind 4, kind 1 otherwise (days −1).
/// Original: `AWOL::start_absence` (0x419840, awol.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let flag = read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(person.addr()) as usize;
    if read_u8(flag) & person_flags::ABSENT_WITHOUT_LEAVE as u8 != 0 {
        return;
    }
    covered!("sx05b_11 player goes awol");
    write_u8(flag, read_u8(flag) | person_flags::ABSENT_WITHOUT_LEAVE as u8);
    let sheet = person.player().addr();
    for word in [player_sheet::CURRENT_REPUTATION, player_sheet::HOME_REPUTATION, player_sheet::WORLD_REPUTATION] {
        awol_cut_reputation(sheet + word);
    }
    let own = person.club_job();
    if !own.is_null() {
        exe::staff_add_disliked_staff(person, own.manager(), 1);
        if exe::human_manager_is_human_club(own, 1, 1, 0, 0) == 0 {
            exe::staff_add_disliked_staff(own.manager(), person, 1);
        }
        for k in 0..50 {
            let mate = Staff(read_usize(own.addr() + club::SQUAD + 4 * k));
            if mate.is_null() || mate.addr() == person.addr() {
                continue;
            }
            if exe::utils_random(mate.loyalty() as i32) != 0 && exe::utils_random(mate.professionalism() as i32) == 0 {
                covered!("sx05b_11 squad mate dislikes him");
                exe::staff_add_disliked_staff(mate, person, 1);
            }
        }
        exe::staff_add_disliked_staff(own.chairman(), person, 1);
    }
    let mut record = [0u8; 12];
    let r = record.as_mut_ptr() as usize;
    write_i32(r, read_i32(person.addr()));
    let mut out = [0u8; 8];
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, G_CURRENT_DATE);
    let days = if kind as u8 == 4 { exe::utils_random(0x1f) + 2 } else { exe::utils_random(0x96) + 0x15 };
    let mut later = [0u8; 8];
    let back = exe::date_add_days(r + 4, later.as_mut_ptr() as usize, days as u32);
    exe::date_operator_3d(r + 4, out.as_mut_ptr() as usize, back);
    exe::llist_add_tail(read_usize(this), r, 0xc);
    let to = if club != 0 { club } else { own.addr() };
    let (news_kind, news_days) = match kind as u8 as i8 {
        1 => (2, u32::MAX),
        4 => (4, days as u32),
        _ => (1, u32::MAX),
    };
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_AWOL_POST_NEWS);
    post(this, person.addr(), to, news_kind, news_days);
}

/// Posts the AWOL news (type 0xbc5) of a player: nothing without a person, his club, his player sheet or `club`.
/// Slots: 0 his staff id, 1..3 his three name ids, 4 `club`'s id, 5 his club's id, 6 the kind, 7 the days (−1 none),
/// 8 his club's nation id (−1 none). A player with a world reputation above 7500 is news for every manager;
/// otherwise his club's nation (when it has one) and, when his nationality is another nation, that nation.
/// Original: `awol_post_news` (0x419ad0, awol.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    awol_post_news_with(this, person, club, kind, days, &mut |item, to| news_post(item, to));
}

/// `awol_post_news` with the posts going to `sink`.
pub(crate) unsafe fn awol_post_news_with(_this: usize, person: usize, club: usize, kind: u32, days: u32, sink: NewsSink) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    covered!("sx05b_11 awol news");
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0xbc5, 0);
    let own = person.club_job();
    let nation = read_usize(own.addr() + club::NATION);
    let slots = [
        read_u32(person.addr()),
        read_u32(person.first_name() + names::ID),
        read_u32(person.second_name() + names::ID),
        read_u32(person.common_name() + names::ID),
        read_u32(club),
        read_u32(own.addr()),
        kind as u8 as i8 as i32 as u32,
        days as u16 as i16 as i32 as u32,
        if nation != 0 { read_u32(nation) } else { u32::MAX },
    ];
    for (k, &v) in slots.iter().enumerate() {
        exe::newsitem_set_slot(p, k as i8, v);
    }
    if person.player().world_reputation() > 7500 {
        return sink(p, NewsPost::All);
    }
    if nation != 0 {
        sink(p, NewsPost::Nation(nation));
    }
    let nationality = read_usize(person.addr() + staff::NATION);
    if read_usize(own.addr() + club::NATION) != nationality {
        sink(p, NewsPost::Nation(nationality));
    }
}

/// Comparer for `awol_start_absence`: the draws, the reputations, the dislikes, the list and the news cannot be
/// repeated; only the original runs, and when it could act (a player, his flag free), the flag must have bit 1 and
/// the absences one more record.
pub(crate) unsafe extern "thiscall" fn compare_awol_start_absence(this: usize, person: usize, club: usize, kind: u32) {
    let p = Staff(person);
    let acts = !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
        && read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize) & 1 == 0;
    let before = read_i16(read_usize(this) + llist::COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, person, club, kind);
    if acts {
        let after = read_i16(read_usize(this) + llist::COUNT);
        let flag = read_u8(read_usize(this + awol_fields::PERSON_FLAGS) + read_i32(p.addr()) as usize);
        SCOREBOARD.record(flag & 1 != 0 && after == before + 1, || format!("awol_start_absence(person {:#x}, kind {}): flag {:#x}, absences {} -> {}", person, kind as u8, flag, before, after));
    }
}

/// Comparer for `awol_post_news`: the original posts; the copy rehearses (`rehearse_news`, no draws) and must post
/// when the original could.
pub(crate) unsafe extern "thiscall" fn compare_awol_post_news(this: usize, person: usize, club: usize, kind: u32, days: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I1%%);
    let mut posts = Vec::new();
    let (ok, a, b) = rehearse_news(|| o(this, person, club, kind, days), || awol_post_news_with(this, person, club, kind, days, &mut |_, to| posts.push(to)));
    SCOREBOARD.record(ok, || format!("awol_post_news(person {:#x}, kind {}) draws original {:?} rust {:?}, rust posts {:x?}", person, kind as u8, a, b, posts));
}
