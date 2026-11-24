// ---------- cx04_7: the scout watching, dressing room unrest and unhappy-with-role stories (european_cup.cpp) ----------

/// The size of a news item built on the stack (NewsItem::NewsItem, the frames' 0xe0 bytes).
const LOCAL_NEWS_WORDS: usize = 0x38;

/// Builds a news item of `kind` in `item` (`exe::new_newsitem`(item, kind, 0)) and returns its address.
unsafe fn new_local_news(item: &mut [u32; LOCAL_NEWS_WORDS], kind: u32) -> usize {
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, kind, 0);
    n
}

/// Where a story goes: the club's manager (`exe::news_send_to_club_manager`, club scope 0), the nation's managers
/// (`exe::news_send_to_nation_managers`, scope 2) or every manager (`exe::news_send_to_all_managers`, scope 4).
#[derive(Clone, Copy)]
enum StoryTo {
    Club(usize),
    Nation(usize),
    All,
}

/// The rehearsal of the story posters' comparer: the copy builds the news item but, instead of sending it, keeps it with
/// its scope in `STORY_PLANNED`.
static STORY_DRY_RUN: AtomicBool = AtomicBool::new(false);
static mut STORY_PLANNED: Option<(u8, [u8; 0xe0])> = None;

/// Sends a story built on the stack (`n`) where `to` says; in the comparer's rehearsal, keeps it instead.
unsafe fn post_story(n: usize, to: StoryTo) {
    if STORY_DRY_RUN.load(Relaxed) {
        let mut item = [0u8; 0xe0];
        core::ptr::copy_nonoverlapping(n as *const u8, item.as_mut_ptr(), 0xe0);
        let reach = match to {
            StoryTo::Club(_) => 0,
            StoryTo::Nation(_) => 2,
            StoryTo::All => 4,
        };
        *core::ptr::addr_of_mut!(STORY_PLANNED) = Some((reach, item));
        return;
    }
    let news = read_usize(G_NEWS);
    match to {
        StoryTo::Club(c) => exe::news_send_to_club_manager(news, n, c),
        StoryTo::Nation(x) => exe::news_send_to_nation_managers(news, n, x),
        StoryTo::All => exe::news_send_to_all_managers(news, n),
    };
}

/// The comparer of the story posters: the copy runs first as a rehearsal (`STORY_DRY_RUN`: it builds the story without
/// sending it; its draws not counted), then the generator is put back and the original runs. Must hold: the copy's
/// draws are the beginning of the original's (a holiday auto-reply may draw after the delivery); each human manager's
/// inbox (News +8: 12 bytes per human, ring start at +0, next slot at +4) that advanced received the copy's story (type
/// +0..4, data and date +5..0xd6, scope +0xde); with no story planned, no inbox advances.
unsafe fn compare_story_post(name: &str, original: impl FnOnce(), copy: impl FnOnce()) {
    let news = read_usize(G_NEWS);
    let inboxes = if news != 0 { read_usize(news + 8) } else { 0 };
    let before_inboxes: Vec<(i32, i32)> = if inboxes != 0 { (0..0x10).map(|k| (read_i32(inboxes + 12 * k), read_i32(inboxes + 12 * k + 4))).collect() } else { Vec::new() };
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS.to_vec());
    *core::ptr::addr_of_mut!(STORY_PLANNED) = None;
    STORY_DRY_RUN.store(true, Relaxed);
    let (_, rr) = crate::rng::trace(|| crate::rng::without_counting(copy));
    STORY_DRY_RUN.store(false, Relaxed);
    let planned = (*core::ptr::addr_of_mut!(STORY_PLANNED)).take();
    rng.write_back();
    let (_, ro) = crate::rng::trace(original);
    let (mut delivered, mut same_story) = (0, true);
    let inboxes = if news != 0 { read_usize(news + 8) } else { 0 };
    if inboxes != 0 && before_inboxes.len() == 0x10 {
        for (k, &(start, slot)) in before_inboxes.iter().enumerate() {
            let e = inboxes + 12 * k;
            if read_i32(e + 4) == slot && read_i32(e) == start {
                continue;
            }
            delivered += 1;
            let v = if start == -1 { read_i32(e) } else { slot }; // the first news item opens the ring at the start
            let g = core::slice::from_raw_parts(read_usize(news).wrapping_add((v as usize).wrapping_mul(0xdf)) as *const u8, 0xdf);
            same_story &= match &planned {
                Some((reach, item)) => g[0..4] == item[0..4] && g[5..0xd6] == item[5..0xd6] && g[0xde] == *reach,
                None => false,
            };
        }
    }
    let same = ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && same_story && (planned.is_some() || delivered == 0);
    SCOREBOARD.record(same, || format!("{}: original draws {:?} rust draws {:?}; planned {} delivered {} same story {}", name, ro, rr, planned.is_some(), delivered, same_story));
}

/// Creates the story that a club's scout watched a player (news 0xfbd) and sends it to the manager of the player's club.
///
/// `news_create_scout_watching_story(player, club)`: nothing without a player, a player sheet (`.player()`), a club job
/// (`.club_job()`) or a watching club. The draw (`exe::utils_random`(3)) comes first; then the news: the player
/// (`set_news_person_slots`, slots 0..3), the watching club's id (4), the player's club id (5) and the draw (6). Sent
/// only when the watching club is in season (`exe::club_is_in_season` with `G_CURRENT_DATE`), the player is not injured
/// (`exe::injury_get_player_injury` on `G_INJURY`, when there is one) and he has a selection slot (signed
/// `exe::tactics_find_selection_slot`(`G_TACTICS`, club, player, 0) not negative): to the player's club manager
/// (`post_story`). SEH frame for the local news, no destructor.
/// Original: `news_create_scout_watching_story` (0x587220, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_create_scout_watching_story(player: usize, club: usize) {
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || player.club_job().is_null() || club == 0 {
        return;
    }
    let draw = exe::utils_random(3);
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xfbd);
    set_news_person_slots(n, 0, player);
    set_news_slot(n, 4, read_u32(club));
    set_news_slot(n, 5, read_u32(player.club_job().addr()));
    set_news_slot(n, 6, draw as u32);
    if exe::club_is_in_season(club, G_CURRENT_DATE) == 0 {
        return;
    }
    let injury = read_usize(G_INJURY);
    if injury != 0 && exe::injury_get_player_injury(injury, player) != 0 {
        return;
    }
    if (exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, player, 0) as i8) < 0 {
        return;
    }
    covered!("cx04_7 scout watching story sent");
    post_story(n, StoryTo::Club(player.club_job().addr()));
}

/// The wording of the dressing room unrest story (slot 9) for a feud of `kind` 0: a player of professionalism 1 has a
/// 3 in 4 chance of wording 0; below 5, a player who favours his club (`exe::staff_favours_club`) gets wording 0 with
/// a 1 in 4 chance, and then wording 1 with a 3 in 4 chance; above 15, wording 2 with a 3 in 4 chance; otherwise 3.
unsafe fn unrest_wording_feud(player: Staff) -> u32 {
    if player.professionalism() == 1 && exe::utils_random(4) != 0 {
        return 0;
    }
    if player.professionalism() < 5 {
        if exe::staff_favours_club(player, player.club_job()) != 0 && exe::utils_random(4) == 0 {
            return 0;
        }
        if player.professionalism() < 5 && exe::utils_random(4) != 0 {
            return 1;
        }
    }
    if player.professionalism() > 0xf && exe::utils_random(4) != 0 {
        return 2;
    }
    3
}

/// The wording of the dressing room unrest story for a dispute with the manager (`kind` 2): with professionalism above
/// 15, an other party of professionalism below 5 gives 0, then the player's work rate below 5 gives 1 and his teamwork
/// below 5 gives 2 (both read from the other party's player sheet when it has one); otherwise 3 + a draw of 3.
unsafe fn unrest_wording_manager(player: Staff, other: Staff) -> u32 {
    if player.professionalism() > 0xf {
        if other.professionalism() < 5 {
            return 0;
        }
        let sheet = other.player();
        if !sheet.is_null() {
            if sheet.work_rate() < 5 {
                return 1;
            }
            if sheet.teamwork() < 5 {
                return 2;
            }
        }
    }
    exe::utils_random(3) as u32 + 3
}

/// Posts the dressing room unrest story (news 0xbc7) about a player and a team-mate or his manager to his club's manager.
///
/// `news_post_dressing_room_unrest(player, other, kind)`: nothing without a player, a player sheet or the other party;
/// with a club job, nothing unless a human manages that club (`exe::news_club_has_human_manager`). The news: the player
/// (slots 0..3), the other party (4..7), the kind (8, signed byte) and the wording (9): kind 0 `unrest_wording_feud`,
/// kind 2 `unrest_wording_manager`, any other kind a draw of 3. Sent to the player's club manager
/// (`post_story` with `.club_job()`, even when it is null). SEH frame for the local news.
/// Original: `news_post_dressing_room_unrest` (0x587c60, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_dressing_room_unrest(player: usize, other: usize, kind: u32) {
    let (player, other) = (Staff(player), Staff(other));
    if player.is_null() || player.player().is_null() || other.is_null() {
        return;
    }
    if !player.club_job().is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xbc7);
    set_news_person_slots(n, 0, player);
    set_news_person_slots(n, 4, other);
    let kind = kind as u8;
    set_news_slot(n, 8, kind as i8 as i32 as u32);
    let wording = match kind {
        0 => unrest_wording_feud(player),
        2 => unrest_wording_manager(player, other),
        _ => exe::utils_random(3) as u32,
    };
    covered!("cx04_7 dressing room unrest story");
    set_news_slot(n, 9, wording);
    post_story(n, StoryTo::Club(player.club_job().addr()));
}

/// Posts the story that a player is unhappy with his role (news 0xbc8) to his club's manager.
///
/// `news_post_player_unhappy_with_role(player, club)`: nothing without a player, a player sheet or a club; with a club
/// job, nothing unless a human manages it (`exe::news_club_has_human_manager`). The news: the player (slots 0..3), the
/// club's id (4) and whether he is transfer listed (5: 1 or 0, `exe::staff_is_transfer_listed`(player, 0)). Sent to
/// the player's club manager (`.club_job()`). SEH frame for the local news.
/// Original: `news_post_player_unhappy_with_role` (0x5882d0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_unhappy_with_role(player: usize, club: usize) {
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || club == 0 {
        return;
    }
    if !player.club_job().is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xbc8);
    set_news_person_slots(n, 0, player);
    set_news_slot(n, 4, read_u32(club));
    covered!("cx04_7 unhappy with role story");
    set_news_slot(n, 5, (exe::staff_is_transfer_listed(player, 0) != 0) as u32);
    post_story(n, StoryTo::Club(player.club_job().addr()));
}

/// Comparers of the story posters (`compare_story_post`).
pub(crate) unsafe extern "C" fn compare_news_create_scout_watching_story(player: usize, club: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%);
    compare_story_post("news_create_scout_watching_story", || o(player, club), || news_create_scout_watching_story(player, club));
}

pub(crate) unsafe extern "C" fn compare_news_post_dressing_room_unrest(player: usize, other: usize, kind: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I1%%);
    compare_story_post("news_post_dressing_room_unrest", || o(player, other, kind), || news_post_dressing_room_unrest(player, other, kind));
}

pub(crate) unsafe extern "C" fn compare_news_post_player_unhappy_with_role(player: usize, club: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I2%%);
    compare_story_post("news_post_player_unhappy_with_role", || o(player, club), || news_post_player_unhappy_with_role(player, club));
}
