// --- sx01c_2: Media::queue_player_sent_off_story (0x759610, media.cpp) ---

/// Queues the press story about a player sent off (news item 0xbe6).
///
/// Nothing unless the player has a nation (`.nation()`), a club (`.club_job()`) whose `.manager()` is set and a
/// player sheet (`.player()`), and the other player has a club with a manager and a sheet. The local news item 0xbe6:
/// 0..3 the player and his names, 4..7 the other player, 8 draw(2), 9 the player's club id, 10 the other's club id,
/// 11 the player's stat 0xb (red cards, by the community map) (`exe::playerstats_get_stat`(sheet, 0xb, 2, 6.5), `ftol`), 12..15 the player's club
/// manager, 16..19 the other club's manager, 20 the AI manager's response (`exe::media_pick_ai_manager_response`(manager,
/// player, 0x82)) when the player's club is not human (`media_is_human_club`(club, 0, 1)), otherwise 0, then the match
/// record (`exe::fill_match_record_from_fixture` with the player's club, `exe::write_match_record_to_news`). A player
/// with a `.world_reputation()` above 4750 goes to every manager of his nation (`exe::news_send_to_nation_managers`),
/// any other to his club's manager (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_player_sent_off_story` (0x759610, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn queue_player_sent_off_story(this: usize, player: usize, other: usize, fixture: usize) {
    let (player, other) = (Staff(player), Staff(other));
    let has_club_and_sheet = |p: Staff| !p.club_job().is_null() && !p.club_job().manager().is_null() && !p.player().is_null();
    if player.is_null() || player.nation().is_null() || !has_club_and_sheet(player) || other.is_null() || !has_club_and_sheet(other) {
        return;
    }
    covered!("sx01c_2 sent off story");
    let (club, other_club) = (player.club_job(), other.club_job());
    let (build, write) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    build(n, 0xbe6, 0);
    set_person_name_slots(write, n, 0, player.addr());
    set_person_name_slots(write, n, 4, other.addr());
    write(n, 8, exe::utils_random(2) as u32);
    write(n, 9, read_u32(club.addr()));
    write(n, 10, read_u32(other_club.addr())); // the original writes -1 without a club, which the test above rules out
    write(n, 11, ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 0xb, 2, 6.5)) as u32);
    set_person_name_slots(write, n, 12, club.manager().addr());
    set_person_name_slots(write, n, 16, other_club.manager().addr());
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_pick_ai_manager_response(this, club.manager(), player, 0x82)
    } else {
        0
    };
    write(n, 20, response);
    let mut k: u8 = 21;
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
    exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut k);
    if player.player().world_reputation() > 0x128e {
        exe::news_send_to_nation_managers(read_usize(G_NEWS), n, player.nation());
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    }
}

/// Comparer for `queue_player_sent_off_story`: it draws and posts news, so it cannot run twice; only the original runs
/// (in the comparison mode the post-match gate of 0x759610 is installed first and keeps the address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_queue_player_sent_off_story(this: usize, player: usize, other: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%)(this, player, other, fixture)
}
