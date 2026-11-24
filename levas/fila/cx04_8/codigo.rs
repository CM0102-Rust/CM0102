// ---------- cx04_8: the unhappy-with-training, feud settled and training report stories (european_cup.cpp) ----------

/// Posts the story that a player is unhappy with training (news 0xbcd) to his club's human manager.
///
/// `news_post_player_unhappy_with_training(player)`: nothing without a player, a player sheet, a club job or a human
/// manager at that club (`exe::news_club_has_human_manager`). The news: the player (slots 0..3), his club's id (4) and
/// a wording drawn among 3 (5). Sent to the player's club manager (`post_story`). SEH frame for the local news.
/// Original: `news_post_player_unhappy_with_training` (0x5885c0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_player_unhappy_with_training(player: usize) {
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || player.club_job().is_null() {
        return;
    }
    if exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xbcd);
    set_news_person_slots(n, 0, player);
    set_news_slot(n, 4, read_u32(player.club_job().addr()));
    covered!("cx04_8 unhappy with training story");
    set_news_slot(n, 5, exe::utils_random(3) as u32);
    post_story(n, StoryTo::Club(player.club_job().addr()));
}

/// Posts the story that two feuding players settle their differences (news 0xbca) to the first player's human manager.
///
/// `news_post_feud_settled(player, other)`: nothing without a player and his sheet or the other player and his sheet;
/// with a club job, nothing unless a human manages that club. The news: the player (slots 0..3) and the other player
/// (4..7). Sent to the player's club manager (`.club_job()`, even when null). SEH frame for the local news.
/// Original: `news_post_feud_settled` (0x5888a0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_feud_settled(player: usize, other: usize) {
    let (player, other) = (Staff(player), Staff(other));
    if player.is_null() || player.player().is_null() || other.is_null() || other.player().is_null() {
        return;
    }
    if !player.club_job().is_null() && exe::news_club_has_human_manager(read_usize(G_NEWS), player.club_job()) == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0xbca);
    set_news_person_slots(n, 0, player);
    covered!("cx04_8 feud settled story");
    set_news_person_slots(n, 4, other);
    post_story(n, StoryTo::Club(player.club_job().addr()));
}

/// The assessment in a critical training report (slot 8), from the player's contract (`exe::contract_manager_get_contract`
/// on `G_CONTRACT_MANAGER`) and the squad status he deserves (`exe::contract_manager_get_deserved_squad_status`, signed
/// byte). Below 4: professionalism below a draw of 7 gives 0; ambition below a draw of 7 gives 1; under 21, pressure
/// below a draw of 8 gives 2; a grievance among 0x30321 (`contract::GRIEVANCES`) gives 3; otherwise 5 + a draw of 4.
/// Status 7: 4. Others: 3 with such a grievance, else 5. Defect of the original: a player without a contract
/// (get_contract 0) has his grievances read from address 0x45 (a crash); the copy does the same.
unsafe fn training_report_assessment(player: Staff) -> u32 {
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    let aggrieved = |c: usize| read_u32(c + contract::GRIEVANCES) & 0x30321 != 0;
    if contract == 0 {
        return if aggrieved(contract) { 3 } else { 5 };
    }
    let status = exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), player, contract) as i8;
    if status < 4 {
        if (player.professionalism() as i32) < exe::utils_random(7) {
            return 0;
        }
        if (player.ambition() as i32) < exe::utils_random(7) {
            return 1;
        }
        if read_i8(player.addr() + staff::AGE) < 0x15 && (player.pressure() as i32) < exe::utils_random(8) {
            return 2;
        }
        if aggrieved(contract) {
            return 3;
        }
        return exe::utils_random(4) as u32 + 5;
    }
    if status == 7 {
        return 4;
    }
    if aggrieved(contract) { 3 } else { 5 }
}

/// Posts an assistant's training report on a player (news 0x138d), praising or criticising him, to a club's manager.
///
/// `news_post_training_report(player, reporter, club, praise)`: nothing without a player and his sheet, a reporter with
/// a non-player sheet (`staff::NON_PLAYER`) or a club. The news: the player (slots 0..3) and the reporter (4..7); with
/// `praise`, a wording drawn among 2 (players over 24) or 3 (8) and 0x3e7 (9); otherwise
/// `training_report_assessment` (8) and 0 (9). Sent to `club`'s manager. SEH frame for the local news.
/// Original: `news_post_training_report` (0x588cd0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_post_training_report(player: usize, reporter: usize, club: usize, praise: u32) {
    let (player, reporter) = (Staff(player), Staff(reporter));
    if player.is_null() || player.player().is_null() || reporter.is_null() || read_usize(reporter.addr() + staff::NON_PLAYER) == 0 || club == 0 {
        return;
    }
    let mut item = [0u32; LOCAL_NEWS_WORDS];
    let n = new_local_news(&mut item, 0x138d);
    set_news_person_slots(n, 0, player);
    set_news_person_slots(n, 4, reporter);
    let (wording, extra) = if praise != 0 {
        covered!("cx04_8 training report praise");
        (exe::utils_random(if read_i8(player.addr() + staff::AGE) > 0x18 { 2 } else { 3 }) as u32, 0x3e7)
    } else {
        covered!("cx04_8 training report criticism");
        (training_report_assessment(player), 0)
    };
    set_news_slot(n, 8, wording);
    set_news_slot(n, 9, extra);
    post_story(n, StoryTo::Club(club));
}

/// Comparers of the story posters (`compare_story_post`).
pub(crate) unsafe extern "C" fn compare_news_post_player_unhappy_with_training(player: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(%%I0%%);
    compare_story_post("news_post_player_unhappy_with_training", || o(player), || news_post_player_unhappy_with_training(player));
}

pub(crate) unsafe extern "C" fn compare_news_post_feud_settled(player: usize, other: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize)>(%%I1%%);
    compare_story_post("news_post_feud_settled", || o(player, other), || news_post_feud_settled(player, other));
}

pub(crate) unsafe extern "C" fn compare_news_post_training_report(player: usize, reporter: usize, club: usize, praise: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, u32)>(%%I2%%);
    compare_story_post("news_post_training_report", || o(player, reporter, club, praise), || news_post_training_report(player, reporter, club, praise));
}
