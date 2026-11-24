// --- sx09b_6: the news item of a contract dispute (dispute.cpp) ---

/// The comparer's dry run of `dispute_post_news`: the copy builds the news item and keeps it and the number of sends,
/// instead of sending it.
pub(crate) static DISPUTE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// What the dry run kept: the news item's first `news_item::ID` bytes and how many sends it would make.
pub(crate) static DISPUTE_NEWS_BUILT: std::sync::Mutex<(Vec<u8>, usize)> = std::sync::Mutex::new((Vec::new(), 0));

/// Sends the dispute news item to every human manager (`nation` null: `exe::news_send_to_all_managers`) or to the
/// managers of a nation (`exe::news_send_to_nation_managers`); in the dry run, counts the send.
unsafe fn dispute_news_send(n: usize, nation: Option<usize>) {
    if DISPUTE_NEWS_DRY_RUN.load(Relaxed) {
        let mut built = DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner());
        built.0 = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        built.1 += 1;
        return;
    }
    match nation {
        None => {
            exe::news_send_to_all_managers(read_usize(G_NEWS), n);
        }
        Some(x) => {
            exe::news_send_to_nation_managers(read_usize(G_NEWS), n, x);
        }
    }
}

/// Posts news item 3049 (0xbe9) about a player's contract dispute with a club.
///
/// `dispute_post_news(person, club, outcome, a, b)` (the dispute object in ecx is not read; SEH frame for the local
/// news item, no destructor). Without a person, his `.club_job()`, his `.player()` or `club`: nothing. Slots 0-3 the
/// person and his name ids (`set_news_person_slots`), 4 the club's id, 5 his `.club_job()`'s id, 6 `a` and 7 `b`
/// (signed bytes), 8 the id of his club's nation (−1 without one), 9 `outcome` (signed byte: 1 kept at the club, 2
/// freed, from `process_hearings`). A player whose `.world_reputation()` is above 7500 is news to every human manager;
/// otherwise to the managers of his club's nation (when it has one) and, when his own `.nation()` is another, also to
/// those of his nation.
/// Original: `dispute_post_news` (0x56a3b0, dispute.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn dispute_post_news(_this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || person.player().is_null() || club == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe9, 0);
    set_news_person_slots(n, 0, person);
    set_news_slot(n, 4, read_u32(club));
    let own = person.club_job();
    set_news_slot(n, 5, if own.is_null() { u32::MAX } else { read_u32(own.addr()) });
    set_news_slot(n, 6, a as i8 as i32 as u32);
    set_news_slot(n, 7, b as i8 as i32 as u32);
    let club_nation = if own.is_null() { 0 } else { own.nation().addr() };
    set_news_slot(n, 8, if club_nation == 0 { u32::MAX } else { read_u32(club_nation) });
    set_news_slot(n, 9, outcome as i8 as i32 as u32); // sx09b_6: the outcome
    if person.player().world_reputation() > 0x1d4c {
        covered!("sx09b_6 dispute news to every manager");
        dispute_news_send(n, None);
        return;
    }
    if club_nation != 0 {
        covered!("sx09b_6 dispute news to the club's nation");
        dispute_news_send(n, Some(club_nation));
    }
    if person.club_job().nation().addr() != person.nation().addr() {
        covered!("sx09b_6 dispute news to the player's nation");
        dispute_news_send(n, Some(person.nation().addr()));
    }
}

/// Comparer for `dispute_post_news`, as a dry run: the copy builds the news item without sending it, then the original
/// runs. Every dispute news item (type 0xbe9) that reached a human inbox must be the copy's, and with no send planned
/// none may arrive.
pub(crate) unsafe extern "thiscall" fn compare_dispute_post_news(this: usize, person: usize, club: usize, outcome: u32, a: u32, b: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I0%%);
    let inboxes = human_inbox_marks();
    *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()) = (Vec::new(), 0);
    DISPUTE_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| dispute_post_news(this, person, club, outcome, a, b)));
    DISPUTE_NEWS_DRY_RUN.store(false, Relaxed);
    let (mut built, sends) = core::mem::take(&mut *DISPUTE_NEWS_BUILT.lock().unwrap_or_else(|e| e.into_inner()));
    o(this, person, club, outcome, a, b);
    if !built.is_empty() {
        built[news_item::IMPORTANT] = 0;
    }
    let delivered = delivered_news_items(&inboxes, 0xbe9);
    let same = inboxes.is_empty() || if sends == 0 { delivered.is_empty() } else { delivered.iter().all(|d| *d == built) };
    SCOREBOARD.record(same, || format!("dispute_post_news(person {:#x}, outcome {}) {} sends; human items original {:x?} rust {:x?}", person, outcome as i8, sends, delivered, built));
}
