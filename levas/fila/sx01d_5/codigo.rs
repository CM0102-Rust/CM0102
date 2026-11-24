// --- sx01d_5: Media::create_player_outburst_story (0x733a60, media.cpp) ---

/// A local news item filled slot after slot (the original's running byte index).
struct NewsSlots {
    n: usize,
    k: u8,
}

impl NewsSlots {
    /// Writes `v` into the next slot.
    unsafe fn put(&mut self, v: u32) {
        exe::newsitem_set_slot(self.n, self.k as i8, v);
        self.k = self.k.wrapping_add(1);
    }

    /// The person's id and his three name keys (read without a test, as the original).
    unsafe fn put_person(&mut self, p: Staff) {
        self.put(read_u32(p.addr()));
        for name in [p.first_name(), p.second_name(), p.common_name()] {
            self.put(read_u32(name + names::ID));
        }
    }

    /// The match record (`exe::fill_match_record_from_fixture` with the club, `exe::write_match_record_to_news` from
    /// the next slot) and the shoot-out slot (`news_shootout_result`); `record` holds the record's date.
    unsafe fn put_match(&mut self, record: &mut [u8; 0x20], fixture: Fixture, club: Club) {
        exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, self.n, &mut self.k);
        if let Some(result) = news_shootout_result(fixture, club) {
            self.put(result);
        }
    }
}

/// Creates the news item 0xbd9 of a player's outburst at his club or team-mates (or, with the report flag, the
/// assistant manager's report on his attitude) after a match, from the outburst record `story`: +0 the player's staff
/// id, +4 the club index, +0xc a signed byte, +0xe the report flag, +0xf the other person's staff id (-1 none), +0x13
/// the story kind (a word).
///
/// Nothing without the record, the player's `.player()` sheet with a `.current_reputation()` of at least 3500, a club
/// index from 0 to the club count (`G_CLUB_COUNT`, inclusive: one past the table passes, a defect of the original) or
/// the club's `.nation()`. Without the report flag, nothing when the story is already logged (`exe::media_has_logged_story`
/// with 0xbd9 and the kind); with it, nothing without the other person. The slots: the player and his names, the club
/// index, the byte +0xc, the kind, draw(2), the report flag, 30, the computer manager's response
/// (`exe::media_pick_ai_manager_response` with the kind, only without the flag and for a club that is not human;
/// otherwise 0), the other person and his names (-1 x 4 without him), then with a match 1, the match record and the
/// shoot-out (`NewsSlots::put_match`), without one 0, then the outlet (`exe::media_pick_outlet_type_for_person`) and
/// the pundit (`exe::punditlist_pick_random_pundit` with the club's nation). With the report flag it goes to the club's
/// manager; otherwise to every human club following the player (`exe::transfer_manager_collect_player_news_recipients`
/// with 0x1a, up to 30, only with `G_TRANSFER_MANAGER`), then the squad reacts
/// (`exe::media_apply_squad_reaction_to_player_story`) and the story is logged (`log_story`, the kind's byte). Last,
/// without the flag and for a club that is not human, the computer manager answers (0,
/// `exe::media_apply_player_story_response`).
///
/// Called by the post-match events (0x6b7787) and by `create_player_media_story` (0x72c4cd, 0x72cc59); behind the
/// post-match gate (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::create_player_outburst_story` (0x733a60, media.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn create_player_outburst_story(this: usize, story: usize, fixture: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    if story == 0 {
        return;
    }
    let player = staff_at(read_i32(story) as isize);
    if player.player().is_null() || (player.player().current_reputation() as i16) < 0xdac {
        return;
    }
    let index = read_i32(story + 4); // the club index
    if index < 0 || index > read_i32(G_CLUB_COUNT) {
        return;
    }
    let club = club_at(index as isize);
    if club.nation().is_null() {
        return;
    }
    let other_id = read_i32(story + 0xf); // the other person's staff id
    let other = if other_id > -1 { staff_at(other_id as isize) } else { Staff(0) };
    let report = read_u8(story + 0xe) != 0; // the assistant's report flag
    let kind = read_u16(story + 0x13) as u32; // the story kind
    if !report && exe::media_has_logged_story(this, player, club, 0xbd9, kind, core::ptr::null()) != 0 {
        return;
    }
    if report && other.is_null() {
        return;
    }
    covered!("sx01d_5 outburst story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbd9, 0);
    s.put_person(player);
    s.put(index as u32);
    s.put(read_i8(story + 0xc) as i32 as u32);
    s.put(kind as u16 as i16 as i32 as u32);
    s.put(exe::utils_random(2) as u32);
    s.put(read_i8(story + 0xe) as i32 as u32);
    s.put(0x1e);
    let ai = media_is_human_club(club.addr(), 0, 1) == 0 && !report;
    s.put(if ai { exe::media_pick_ai_manager_response(this, club.manager(), player, kind) } else { 0 });
    if other_id > -1 {
        s.put(other_id as u32);
        for name in [other.first_name(), other.second_name(), other.common_name()] {
            s.put(read_u32(name + names::ID));
        }
    } else {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    }
    if fixture != 0 {
        covered!("sx01d_5 outburst story after a match");
        s.put(1);
        s.put_match(&mut record, Fixture(fixture), club);
    } else {
        s.put(0);
    }
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    let nation = club.nation().addr();
    s.put(exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    if report {
        covered!("sx01d_5 assistant report");
        exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, club_at(index as isize));
    } else {
        let mut clubs = [0usize; 30];
        let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x1a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
        for &c in clubs.iter().take(count.max(0) as usize) {
            exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, c);
        }
        exe::media_apply_squad_reaction_to_player_story(this, kind, player, club, core::ptr::null_mut(), u32::MAX);
        log_story(this, player.addr(), club.addr(), 0xbd9, read_u8(story + 0x13) as u32, 0);
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 && !report {
        exe::media_apply_player_story_response(this, player, club, kind, 0);
    }
}

/// Comparer for `create_player_outburst_story`: it draws, posts news and answers the story, so it cannot run twice;
/// only the original runs (in the comparison mode the post-match gate of 0x733a60 is installed first and keeps the
/// address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_create_player_outburst_story(this: usize, story: usize, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, story, fixture)
}
