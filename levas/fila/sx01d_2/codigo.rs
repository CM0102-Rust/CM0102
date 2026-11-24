// --- sx01d_2: Media::queue_obscene_gesture_story (0x7570b0) and Media::queue_wants_first_team_story (0x75a640), media.cpp ---

/// Writes a person's id and three name keys into news slots k..k+3, the common name (+0xc) as -1 when the person has
/// none (the first and second names are read without a test, as the original).
unsafe fn set_person_name_slots_or_none(n: usize, k: i8, p: Staff) {
    exe::newsitem_set_slot(n, k, read_u32(p.addr()));
    exe::newsitem_set_slot(n, k + 1, read_u32(p.first_name() + names::ID));
    exe::newsitem_set_slot(n, k + 2, read_u32(p.second_name() + names::ID));
    let common = p.common_name();
    exe::newsitem_set_slot(n, k + 3, if common != 0 { read_u32(common + names::ID) } else { u32::MAX });
}

/// The shoot-out slot of a match story: 0 without a shoot-out (`fixture::HOME_PENALTIES` 0xff), 1 when `club` won
/// it, 2 when it lost it, none when the shoot-out goals are equal (signed bytes).
pub(crate) unsafe fn news_shootout_result(fixture: Fixture, club: Club) -> Option<u32> {
    let home = read_i8(fixture.addr() + fixture::HOME_PENALTIES);
    let away = read_i8(fixture.addr() + fixture::AWAY_PENALTIES);
    if home == -1 {
        return Some(0);
    }
    if home == away {
        return None;
    }
    covered!("sx01d_2 match story shoot-out");
    let winner = read_usize(fixture.addr() + if home > away { fixture::HOME_CLUB } else { fixture::AWAY_CLUB });
    Some(if winner == club.addr() { 1 } else { 2 })
}

/// Queues news item 0xbcb about a player filmed making obscene gestures to the fans.
///
/// Nothing without the player, his `.player()` sheet with a `.current_reputation()` of at least 3500, or the club. The
/// local news item 0xbcb: 0..3 the player and his names (the common name -1 when he has none), 4 the club id, 5
/// draw(2), then the match record (`exe::fill_match_record_from_fixture` with the club, `exe::write_match_record_to_news`
/// from slot 6) and, in the next slot, the shoot-out: 0 when there was none (`fixture::HOME_PENALTIES` 0xff), 1 when the
/// club won it, 2 when it lost it, no slot when the shoot-out goals are equal. It goes to every human club following the
/// player (`exe::transfer_manager_collect_player_news_recipients` with 0xda, up to 30, only with `G_TRANSFER_MANAGER`)
/// through `exe::news_send_to_club_manager`.
///
/// Called by `apply_obscene_gesture_ban` (0x7daf50, printouts.rs).
/// Original: `Media::queue_obscene_gesture_story` (0x7570b0, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn queue_obscene_gesture_story(_this: usize, player: usize, club: usize, fixture: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let (player, club, fixture) = (Staff(player), Club(club), Fixture(fixture));
    if player.is_null() || player.player().is_null() || club.is_null() || (player.player().current_reputation() as i16) < 0xdac {
        return;
    }
    covered!("sx01d_2 obscene gesture story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbcb, 0);
    set_person_name_slots_or_none(n, 0, player);
    exe::newsitem_set_slot(n, 4, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 5, exe::utils_random(2) as u32);
    exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
    let mut k: u8 = 6;
    exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut k);
    if let Some(result) = news_shootout_result(fixture, club) {
        exe::newsitem_set_slot(n, k as i8, result);
    }
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0xda, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubs.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
    }
}

/// Comparer for `queue_obscene_gesture_story`: it draws and posts news, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_queue_obscene_gesture_story(this: usize, player: usize, club: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%)(this, player, club, fixture)
}

/// Queues news item 0xbe8 about an unhappy player who wants first team football.
///
/// Nothing without the player, his `.nation()`, his club (`.club_job()`) with a `.manager()` and a `.stadium()`, his
/// `.player()` sheet or his contract at the club (`exe::contract_manager_get_contract_for_club`). The local news item
/// 0xbe8: 0..3 the player and his names, 4..7 the manager and his names, 8 the club id, 9 his natural positions
/// (`exe::staff_get_natural_position_mask`), 10 draw(2), 11 the outlet (`exe::media_pick_outlet_type_for_person`), 12
/// the pundit (`exe::punditlist_pick_random_pundit` with the club, the outlet and the club's `.nation()` or -1), 13 the
/// strength flags (`exe::staff_get_player_strength_flags`), 14 the sheet's position mask
/// (`exe::formation_get_position_mask`), 15 `reaction`, 16 the contract's `.morale()`, 17 the player's age, 18 the
/// computer manager's response (`exe::media_pick_ai_manager_response` with 0x83; 0 for a human club), 19 the contract's
/// `.squad_status()` >> 4, 20 one when `.aggression()` + `.temperament()` (a byte) is below 10 and `.ambition()` above
/// 10, 21 `exe::staff_is_transfer_listed_or_requested`. With `reaction` it goes to the club's manager; otherwise to
/// every manager of the player's nation when his `.world_reputation()` is above 4750, else to the club's manager.
///
/// Called by the squad's daily review (0x5d24a2) and by `apply_wants_first_team_response` (the player's reaction).
/// Original: `Media::queue_wants_first_team_story` (0x75a640, media.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn queue_wants_first_team_story(this: usize, player: usize, reaction: u32) {
    let player = Staff(player);
    if player.is_null() || player.nation().is_null() {
        return;
    }
    let club = player.club_job();
    if club.is_null() || club.manager().is_null() || club.stadium().addr() == 0 || player.player().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return;
    }
    covered!("sx01d_2 wants first team story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe8, 0);
    let write = news_item_fns().1;
    set_person_name_slots(write, n, 0, player.addr());
    set_person_name_slots(write, n, 4, club.manager().addr());
    exe::newsitem_set_slot(n, 8, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 9, exe::staff_get_natural_position_mask(player));
    exe::newsitem_set_slot(n, 10, exe::utils_random(2) as u32);
    let outlet = exe::media_pick_outlet_type_for_person(player);
    exe::newsitem_set_slot(n, 11, outlet);
    let nation = club.nation().addr();
    exe::newsitem_set_slot(n, 12, exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    exe::newsitem_set_slot(n, 13, exe::staff_get_player_strength_flags(player, club));
    exe::newsitem_set_slot(n, 14, exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    exe::newsitem_set_slot(n, 15, reaction);
    exe::newsitem_set_slot(n, 16, contract.morale() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 17, player.age() as i8 as i32 as u32);
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_pick_ai_manager_response(this, club.manager(), player, 0x83)
    } else {
        0
    };
    exe::newsitem_set_slot(n, 18, response);
    exe::newsitem_set_slot(n, 19, (contract.squad_status() as u8 >> 4) as u32);
    let hot = ((player.player().aggression() as u8).wrapping_add(player.temperament() as u8) as i8) < 10 && player.ambition() as i8 > 10;
    exe::newsitem_set_slot(n, 20, hot as u32);
    exe::newsitem_set_slot(n, 21, exe::staff_is_transfer_listed_or_requested(player, 0));
    if reaction == 0 && player.player().world_reputation() as i16 > 0x128e {
        covered!("sx01d_2 wants first team story to the nation");
        exe::news_send_to_nation_managers(read_usize(G_NEWS), n, player.nation());
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Comparer for `queue_wants_first_team_story`: it draws (also through the computer manager's response) and posts
/// news, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_queue_wants_first_team_story(this: usize, player: usize, reaction: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, player, reaction)
}
