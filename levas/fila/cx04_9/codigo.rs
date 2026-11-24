// ---------- cx04_9: the player-and-manager stories: thanks, turnaround, first-team chance (european_cup.cpp) ----------

/// Whether a player-and-manager story can be told: a player with a player sheet and a nation (`staff::NATION`), and a
/// club with a nation, a stadium with a city (`stadium::CITY`) and a manager.
unsafe fn club_player_story_ready(player: Staff, club: Club) -> bool {
    !player.is_null() && !player.player().is_null() && read_usize(player.addr() + staff::NATION) != 0 && !club.is_null()
        && read_usize(club.addr() + club::NATION) != 0 && read_usize(club.addr() + club::STADIUM) != 0
        && read_usize(read_usize(club.addr() + club::STADIUM) + stadium::CITY) != 0 && read_usize(club.addr() + club::MANAGER) != 0
}

/// Builds a player-and-manager story of `kind` in `item`: the player (slots 0..3), the club's id (4), 2 when the
/// player's current reputation is above 7250 or else 1 (5), and the club's manager (6..9).
unsafe fn club_player_story(item: &mut [u32; LOCAL_NEWS_WORDS], kind: u32, player: Staff, club: Club) -> usize {
    let n = new_local_news(item, kind);
    set_news_person_slots(n, 0, player);
    set_news_slot(n, 4, read_u32(club.addr()));
    set_news_slot(n, 5, if player.player().current_reputation() > 0x1c52 { 2 } else { 1 });
    set_news_person_slots(n, 6, Staff(read_usize(club.addr() + club::MANAGER)));
    n
}

/// Posts the story that a player credits his manager for his form this season (news 0xbde) to the club's manager.
///
/// `news_post_player_thanks_manager(player, club)`: nothing unless `club_player_story_ready`. The news
/// (`club_player_story`) is sent to `club`'s manager. SEH frame for the local news.
/// Original: `news_post_player_thanks_manager` (0x589300, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_thanks_manager(player: usize, club: usize) {
    let (player, club) = (Staff(player), Club(club));
    if !club_player_story_ready(player, club) {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    covered!("cx04_9 player thanks manager story");
    let n = club_player_story(&mut item, 0xbde, player, club);
    post_story(n, StoryTo::Club(club.addr()));
}

/// Posts the story that a player is ecstatic at his club's win after a run without one (news 0xbe1) to the club's
/// manager.
///
/// `news_post_player_ecstatic_at_turnaround(player, club, opponent, run)`: nothing unless `club_player_story_ready` and
/// an opponent. The news (`club_player_story`) with the opponent's id (10) and the length of the run (11), sent to
/// `club`'s manager. SEH frame for the local news.
/// Original: `news_post_player_ecstatic_at_turnaround` (0x5896d0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_ecstatic_at_turnaround(player: usize, club: usize, opponent: usize, run: u32) {
    let (player, club) = (Staff(player), Club(club));
    if !club_player_story_ready(player, club) || opponent == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    covered!("cx04_9 player ecstatic story");
    let n = club_player_story(&mut item, 0xbe1, player, club);
    set_news_slot(n, 0xa, read_u32(opponent));
    set_news_slot(n, 0xb, run);
    post_story(n, StoryTo::Club(club.addr()));
}

/// Posts the story that a player is pleased with his first-team chance (news 0xbe2) to the club's manager.
///
/// `news_post_player_pleased_with_chance(player, club)`: nothing unless `club_player_story_ready`. The news
/// (`club_player_story`) with a wording (10): a draw of 2 at clubs of reputation above 6250 (`.reputation()`), else 0;
/// sent to `club`'s manager. SEH frame for the local news.
/// Original: `news_post_player_pleased_with_chance` (0x589bc0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_pleased_with_chance(player: usize, club: usize) {
    let (player, club) = (Staff(player), Club(club));
    if !club_player_story_ready(player, club) {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    covered!("cx04_9 player pleased with chance story");
    let n = club_player_story(&mut item, 0xbe2, player, club);
    set_news_slot(n, 0xa, if club.reputation() > 0x186a { exe::utils_random(2) as u32 } else { 0 });
    post_story(n, StoryTo::Club(club.addr()));
}

/// Comparers of the story posters (`compare_story_post`).
pub(crate) unsafe extern "C" fn compare_news_post_player_thanks_manager(player: usize, club: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%);
    compare_story_post("news_post_player_thanks_manager", || o(player, club), || news_post_player_thanks_manager(player, club));
}

pub(crate) unsafe extern "C" fn compare_news_post_player_ecstatic_at_turnaround(player: usize, club: usize, opponent: usize, run: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32)>(%%I1%%);
    compare_story_post("news_post_player_ecstatic_at_turnaround", || o(player, club, opponent, run), || news_post_player_ecstatic_at_turnaround(player, club, opponent, run));
}

pub(crate) unsafe extern "C" fn compare_news_post_player_pleased_with_chance(player: usize, club: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I2%%);
    compare_story_post("news_post_player_pleased_with_chance", || o(player, club), || news_post_player_pleased_with_chance(player, club));
}
