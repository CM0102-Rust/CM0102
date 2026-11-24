// ---------- cx04_10: the award credit, FA disciplinary fine and transfer system stories (european_cup.cpp) ----------

/// Posts the story that a player credits his manager for an award he won (news 0xbe3) to the club's manager.
///
/// `news_post_player_credits_manager_for_award(player, club, award)`: nothing unless `club_player_story_ready` and an
/// award. The news (`club_player_story`) with the award's id (10), sent to `club`'s manager. SEH frame for the local
/// news.
/// Original: `news_post_player_credits_manager_for_award` (0x58a070, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_credits_manager_for_award(player: usize, club: usize, award: usize) {
    let (player, club) = (Staff(player), Club(club));
    if !club_player_story_ready(player, club) || award == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    covered!("cx04_10 award credit story");
    let n = club_player_story(&mut item, 0xbe3, player, club);
    set_news_slot(n, 0xa, read_u32(award));
    post_story(n, StoryTo::Club(club.addr()));
}

/// Posts the story that the national FA fines a club for its disciplinary record (news 0x178c).
///
/// `news_post_fa_disciplinary_fine(club, kind, amount)`: nothing without a club, its nation or its manager. The news:
/// the club's id (slot 0), the manager (1..4; −1 in all four when there is none, a branch the check above makes
/// unreachable), the nation's id (5), the kind (6, signed byte) and the amount (7). With a kind and a club of
/// reputation above 4750 (`.reputation()`) it goes to the nation's managers, otherwise to the club's manager. SEH frame
/// for the local news.
/// Original: `news_post_fa_disciplinary_fine` (0x58a4d0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_fa_disciplinary_fine(club: usize, kind: u32, amount: u32) {
    let club = Club(club);
    if club.is_null() || read_usize(club.addr() + club::NATION) == 0 || read_usize(club.addr() + club::MANAGER) == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0x178c);
    set_news_slot(n, 0, read_u32(club.addr()));
    let manager = read_usize(club.addr() + club::MANAGER);
    if manager != 0 {
        set_news_person_slots(n, 1, Staff(manager));
    } else {
        for k in 1..=4 {
            set_news_slot(n, k, u32::MAX);
        }
    }
    let nation = read_usize(club.addr() + club::NATION);
    set_news_slot(n, 5, read_u32(nation));
    let kind = kind as u8;
    set_news_slot(n, 6, kind as i8 as i32 as u32);
    set_news_slot(n, 7, amount);
    if kind != 0 && club.reputation() > 0x128e {
        covered!("cx04_10 fa fine story to the nation");
        post_story(n, StoryTo::Nation(read_usize(club.addr() + club::NATION)));
    } else {
        covered!("cx04_10 fa fine story to the club");
        post_story(n, StoryTo::Club(club.addr()));
    }
}

/// Posts the FIFA transfer system change story, or the earlier one of clubs rushing to renew contracts (news 0xfc1,
/// slot 0 the kind as a signed byte), to every manager. SEH frame for the local news.
/// Original: `news_post_transfer_system_change` (0x58a890, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_transfer_system_change(kind: u32) {
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xfc1);
    covered!("cx04_10 transfer system story");
    set_news_slot(n, 0, kind as u8 as i8 as i32 as u32);
    post_story(n, StoryTo::All);
}

/// Comparers of the story posters (`compare_story_post`).
pub(crate) unsafe extern "C" fn compare_news_post_player_credits_manager_for_award(player: usize, club: usize, award: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize)>(%%I0%%);
    compare_story_post("news_post_player_credits_manager_for_award", || o(player, club, award), || news_post_player_credits_manager_for_award(player, club, award));
}

pub(crate) unsafe extern "C" fn compare_news_post_fa_disciplinary_fine(club: usize, kind: u32, amount: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I1%%);
    compare_story_post("news_post_fa_disciplinary_fine", || o(club, kind, amount), || news_post_fa_disciplinary_fine(club, kind, amount));
}

pub(crate) unsafe extern "C" fn compare_news_post_transfer_system_change(kind: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(%%I2%%);
    compare_story_post("news_post_transfer_system_change", || o(kind), || news_post_transfer_system_change(kind));
}
