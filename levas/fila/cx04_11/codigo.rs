// ---------- cx04_11: the insurance payout story (european_cup.cpp) ----------

/// Posts the story that a club receives an insurance payout after a player retires early through injury (news 0xfc2)
/// to the club's manager: the player (slots 0..3), the amount (4) and the club's id (5); no checks. SEH frame for the
/// local news.
/// Original: `news_post_insurance_payout` (0x58a980, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_insurance_payout(club: usize, player: usize, amount: u32) {
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xfc2);
    set_news_person_slots(n, 0, Staff(player));
    covered!("cx04_11 insurance payout story");
    set_news_slot(n, 4, amount);
    set_news_slot(n, 5, read_u32(club));
    post_story(n, StoryTo::Club(club));
}

/// Comparer of the story poster (`compare_story_post`).
pub(crate) unsafe extern "C" fn compare_news_post_insurance_payout(club: usize, player: usize, amount: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%);
    compare_story_post("news_post_insurance_payout", || o(club, player, amount), || news_post_insurance_payout(club, player, amount));
}
