// --- sx01e_1: Media::post_player_praise_story (0x744240, media.cpp) ---

/// The kinds of the praise story (news item 0xbdd, slot 5), as the text writer reads them (`praise_body`).
const PRAISE_FUTURE_INTERNATIONAL: u8 = 0x29;
const PRAISE_YOUNGSTER: u8 = 0x2a;
const PRAISE_PRAISED: u8 = 0x2b;
const PRAISE_CALL_UP: u8 = 0x2c;
const PRAISE_ON_THE_RISE: u8 = 0x2d;
const PRAISE_BIG_MOVE: u8 = 0x2e;
const PRAISE_ATTITUDE: u8 = 0x2f;
const PRAISE_WORLD_CLASS: u8 = 0x30;
const PRAISE_WORLD_CLASS_B: u8 = 0x31;
const PRAISE_IMPRESSES: u8 = 0x32;
const PRAISE_WORLD_STAGE: u8 = 0x33;
const PRAISE_COMPETITION: u8 = 0x34;

/// `NATIONAL_TEAMS::get_player_squad_class` (thiscall(national teams, person), ret 4, the class in al): 2 when the
/// person's national job (+0x24) passes 0x5409b0, otherwise bit 3 of the byte +0x5e; -1 without a national job.
const FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS: usize = 0x76e070;

/// The person's national squad class (`FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS` on `G_NATIONAL_TEAMS`).
unsafe fn national_squad_class(person: Staff) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_PLAYER_SQUAD_CLASS);
    f(read_usize(G_NATIONAL_TEAMS), person.addr())
}

/// The low nibble of the person's byte `staff_state::STATUS_FLAGS` in the 79-byte staff state table.
unsafe fn staff_state_nibble(person: Staff) -> i32 {
    let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(person.addr()) as usize).wrapping_mul(staff_state::SIZE));
    (read_u8(row + staff_state::STATUS_FLAGS) & 0xf) as i32
}

/// Whether the player has no transfer offer (`exe::transfer_manager_get_offers_for_player` with 0, 0).
unsafe fn has_no_offers(player: Staff) -> bool {
    exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), player, 0usize, 0) == 0
}

/// The praise story's kind and the draw of slot 8 (the kinds of a young player). Below 22 without caps: a current
/// reputation above 1.3 × the club's with random(ambition) > random(20) and no offer, or the grievance bit 1 of the
/// contract with no offer and the state nibble at least random(ambition / 2): the big move (draw 6); otherwise in the
/// national squad (class 1 or 2) with a home reputation above 4250 and above 30 × the nation's, the future
/// international, else the youngster (draw 4). `contract` is null for a national team: the original reads an
/// uninitialised stack slot there (here the bit is clear).
unsafe fn praise_kind_young(player: Staff, club: Club, contract: Contract) -> (u8, u32) {
    let sheet = player.player();
    let current = sheet.current_reputation() as i32;
    if club.reputation() as f64 * read_f64(rdata::F64_1_3) < current as f64 {
        let ambition = exe::utils_random(player.ambition() as i32);
        if ambition > exe::utils_random(0x14) && has_no_offers(player) {
            covered!("sx01e_1 praise big move");
            return (PRAISE_BIG_MOVE, exe::utils_random(6) as u32);
        }
    }
    if !contract.is_null() && contract.grievances() & 1 != 0 && has_no_offers(player) && staff_state_nibble(player) >= exe::utils_random(player.ambition() as i32 / 2) {
        covered!("sx01e_1 praise big move by grievance");
        return (PRAISE_BIG_MOVE, exe::utils_random(6) as u32);
    }
    let home = sheet.home_reputation() as i32;
    let in_squad = national_squad_class(player) == 1 || national_squad_class(player) == 2;
    let kind = if in_squad && home > 0x109a && home > player.nation().reputation() as i32 * 30 { PRAISE_FUTURE_INTERNATIONAL } else { PRAISE_YOUNGSTER };
    (kind, exe::utils_random(4) as u32)
}

/// The praise story's kind and the draw of slot 8 (the kinds of an older or capped player), in this order: the call-up
/// (a home reputation above 0.8 ×, a world one above 0.4 × and the club's above 0.65 × the nation's reputation, under
/// 28, no caps, random(4) and no national job), on the rise (under 24 with a current reputation below 6000), world
/// class (under 27 − random(4), current above 8000, world below 7000, ability above random(20) + 155, caps below
/// random(5) + 10, club reputation above random(2000) + 6500), world class B (under 33, current above 9000, world
/// above 8500, ability above random(20) + 175, club above random(2000) + 7500) each with draw(3), else the praise
/// (draw 8).
unsafe fn praise_kind_senior(player: Staff, club: Club) -> (u8, u32) {
    let sheet = player.player();
    let age = player.age() as i8 as i32;
    let nation = player.nation().reputation() as f64;
    let club_reputation = club.reputation() as i32;
    if nation * read_f64(rdata::F64_0_8) < sheet.home_reputation() as f64
        && nation * read_f64(rdata::F64_0_4) < sheet.world_reputation() as f64
        && age < 0x1c
        && nation * read_f64(rdata::F64_0_65) < club_reputation as f64
        && player.int_apps() == 0
        && exe::utils_random(4) != 0
        && player.national_job().is_null()
    {
        return (PRAISE_CALL_UP, exe::utils_random(3) as u32);
    }
    let current = sheet.current_reputation() as i32;
    if age < 0x18 && current < 6000 {
        return (PRAISE_ON_THE_RISE, exe::utils_random(3) as u32);
    }
    let limit = 0x1b - exe::utils_random(4);
    if age < limit
        && current > 8000
        && (sheet.world_reputation() as i32) < 7000
        && sheet.current_ability() as i32 > exe::utils_random(0x14) + 0x9b
        && (player.int_apps() as i32) < exe::utils_random(5) + 10
        && club_reputation > exe::utils_random(2000) + 0x1964
    {
        covered!("sx01e_1 praise world class");
        return (PRAISE_WORLD_CLASS, exe::utils_random(3) as u32);
    }
    if age < 0x21
        && current > 9000
        && sheet.world_reputation() as i32 > 0x2134
        && sheet.current_ability() as i32 > exe::utils_random(0x14) + 0xaf
        && club_reputation > exe::utils_random(2000) + 0x1d4c
    {
        return (PRAISE_WORLD_CLASS_B, exe::utils_random(3) as u32);
    }
    (PRAISE_PRAISED, exe::utils_random(8) as u32)
}

/// The praise story's kind and the draw of slot 8. The world stage and the competition keep their kind (draw 4).
/// Otherwise, with random(3) ≠ 0 and today before the day he joined the club plus 30 + random(150) days, he impresses
/// (draw 3); then the young kinds (under 22 without caps, `praise_kind_young`); then his attitude (over 30, leadership
/// and professionalism above 15, more than 400 days at the club, a current reputation above 7250 and random(3) = 0),
/// which keeps the first draw (`first_draw`, random(2)); then the older kinds (`praise_kind_senior`).
unsafe fn praise_story_kind(player: Staff, club: Club, kind: u8, contract: Contract, first_draw: u32) -> (u8, u32) {
    if kind == PRAISE_WORLD_STAGE || kind == PRAISE_COMPETITION {
        return (kind, exe::utils_random(4) as u32);
    }
    if exe::utils_random(3) != 0 {
        let mut settled = [0u8; 8];
        let days = exe::utils_random(0x96) + 0x1e;
        let settled = exe::date_add_days(player.addr() + staff::DATE_JOINED_CLUB, settled.as_mut_ptr() as usize, days as u32);
        if media_date_before(G_CURRENT_DATE, settled) {
            covered!("sx01e_1 praise impresses");
            return (PRAISE_IMPRESSES, exe::utils_random(3) as u32);
        }
    }
    let age = player.age() as i8 as i32;
    if age < 0x16 && player.int_apps() == 0 {
        return praise_kind_young(player, club, contract);
    }
    let sheet = player.player();
    if age > 0x1e
        && sheet.leadership() > 0xf
        && exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) > 400
        && sheet.current_reputation() > 0x1c52
        && player.professionalism() > 0xf
        && exe::utils_random(3) == 0
    {
        covered!("sx01e_1 praise attitude");
        return (PRAISE_ATTITUDE, first_draw);
    }
    praise_kind_senior(player, club)
}

/// The slots after the match (from 0x11): with a match 1, the match record and the shoot-out comes later; without
/// one 0. Then the transfer slots: with a future transfer (`exe::transfer_manager_get_future_transfer`) the buying
/// club's id (+8 of the offer, `exe::transfer_manager_get_offer_slot`), 1 and the transfer date (day signed, year);
/// otherwise his club's id (-1 without one), 0, -1, -1. Then the state nibble without a future transfer (0 with one).
unsafe fn praise_story_match_and_transfer(s: &mut NewsSlots, player: Staff, club: Club, fixture: usize) {
    s.k = 0x11;
    if fixture != 0 {
        let mut record = [0u8; 0x20]; // the 0x1f-byte match record
        exe::date_date_54a110(record.as_mut_ptr() as usize);
        s.put(1);
        exe::fill_match_record_from_fixture(fixture, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, s.n, &mut s.k);
    } else {
        s.put(0);
    }
    let transfers = read_usize(G_TRANSFER_MANAGER);
    let mut future = 0usize;
    if exe::transfer_manager_get_future_transfer(transfers, player, &mut future as *mut usize as u32) != 0 {
        covered!("sx01e_1 praise with a future transfer");
        let offer = read_usize(exe::transfer_manager_get_offer_slot(transfers, read_i32(future + 8)));
        s.put(read_u32(offer + 8)); // the buying club's id
        s.put(1);
        s.put(read_i16(future) as i32 as u32); // the transfer date: day
        s.put(read_u16(future + 2) as u32); // year
    } else {
        s.put(if player.club_job().is_null() { u32::MAX } else { read_u32(player.club_job().addr()) });
        s.put(0);
        s.put(u32::MAX);
        s.put(u32::MAX);
    }
    s.put(if exe::transfer_manager_get_future_transfer(transfers, player, 0) == 0 { staff_state_nibble(player) as u32 } else { 0 });
}

/// Posts the praise story about a player (news item 0xbdd), raises his reputations and lets the computer manager
/// answer it at once.
///
/// Nothing without the player, his `.player()` sheet, a staff id other than 0 (a defect of the original: the person
/// with id 0 is never praised), a `.current_reputation()` of at least 3500, his `.nation()`, the club, its `.manager()`
/// and `.nation()`. A club that is not a national team (`exe::database_is_national_team`) needs a division other than
/// 0x9cf764 and the player's contract with it (`exe::contract_manager_get_contract_for_club`).
///
/// The slots: 0..3 the player and his names (the common name -1 when he has none), 4 the club id, 5 the kind
/// (`praise_story_kind`; the world stage 0x33 and the competition 0x34 passed in are kept); nothing when the story is
/// already logged (`exe::media_has_logged_story` with 0xbdd and the kind); 6 the board confidence below 4000
/// (`board_record::CONFIDENCE`), 7 his nation id, 8 the kind's draw, 9..12 the club's manager and his names, 13 the
/// computer manager's response (`exe::media_pick_ai_manager_response`, for a club that is not human; otherwise 0), 14
/// his strength flags (`exe::staff_get_player_strength_flags`), 15 his position mask (`exe::formation_get_position_mask`),
/// 16 the competition id: for the competition the fixture's (or, without one, as the world stage, the nation's current
/// competition, `exe::nationalteams_get_current_competition`, nothing without it), otherwise the club's division; the
/// match and transfer slots (`praise_story_match_and_transfer`); the former international pundit and his names
/// (`exe::media_pick_former_international_pundit`, -1 × 4 without one), the next hosts of the competition
/// (`exe::hostcountry_find_next_hosts`, 0 and -1 by default), the shoot-out (1 won, 2 lost, otherwise 0), the pundit's
/// job and club (club job first, then national job; -1 × 2 without), the outlet (`exe::media_pick_outlet_type_for_person`),
/// the pundit of the outlet (`exe::punditlist_pick_random_pundit` with the club's nation), his age and whether he is a
/// goalkeeper (20).
///
/// The world stage goes to every manager when his world reputation plus the club's is above 13750, otherwise to the
/// continent of the club's nation; the competition to every manager; the others to every human club following him
/// (`exe::transfer_manager_collect_player_news_recipients` with 0x25a, up to 30). Then the story is logged (`log_story`
/// with the response), his current and home reputations rise by 200 and the world one by 100 (each up to 10000), he
/// goes onto the clubs' shortlists (`exe::shortlist_manager_add_player_to_club_shortlists`), the chairman of a random
/// club of the same nation favours him (`exe::staff_add_favoured_staff`) and, for a club that is not human, the
/// computer manager's response is applied (`exe::media_apply_player_story_response`).
///
/// Called by the daily loop (0x5d02ab, 0x5d05f7, 0x5d0e1a) and the match day (0x6b715b); behind the post-match gate
/// (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::post_player_praise_story` (0x744240, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_player_praise_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize) {
    let (player, club) = (Staff(player), Club(club));
    if player.is_null() || player.player().is_null() || read_i32(player.addr()) == 0 || player.player().current_reputation() < 0xdac || player.nation().is_null() {
        return;
    }
    if club.is_null() || club.manager().is_null() || club.nation().is_null() {
        return;
    }
    let mut contract = Contract(0);
    if exe::database_is_national_team(club) == 0 {
        let division = club.division();
        if division.is_null() || read_i32(division.addr()) == read_i32(0x9cf764) {
            return; // 0x9cf764: the id of a division that gets no praise
        }
        contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
        if contract.is_null() {
            return;
        }
    }
    covered!("sx01e_1 praise story");
    let first_draw = exe::utils_random(2) as u32;
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbdd, 0);
    set_person_name_slots_or_none(s.n, 0, player);
    s.k = 4;
    s.put(read_u32(club.addr()));
    let (kind, draw) = praise_story_kind(player, club, kind as u8, contract, first_draw);
    s.put(kind as i8 as i32 as u32);
    if exe::media_has_logged_story(this, player, club, 0xbdd, kind as u32, core::ptr::null()) != 0 {
        return;
    }
    s.put((read_i16(manager_manager_row(club) + board_record::CONFIDENCE) < 4000) as u32);
    s.put(read_u32(player.nation().addr()));
    s.put(draw);
    s.put_person(club.manager());
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 { exe::media_pick_ai_manager_response(this, club.manager(), player, kind as u32) } else { 0 };
    s.put(response);
    s.put(exe::staff_get_player_strength_flags(player, club));
    s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    let comp_id = if kind == PRAISE_COMPETITION && fixture != 0 {
        read_u32(fixture) // fixture::COMP_ID
    } else if kind == PRAISE_WORLD_STAGE || kind == PRAISE_COMPETITION {
        let teams = read_usize(G_NATIONAL_TEAMS);
        if exe::nationalteams_get_current_competition(teams, club.nation()) == 0 {
            return;
        }
        read_u32(exe::nationalteams_get_current_competition(teams, club.nation()))
    } else {
        read_u32(club.division().addr())
    };
    s.put(comp_id);
    praise_story_match_and_transfer(&mut s, player, club, fixture);
    let pundit = Staff(exe::media_pick_former_international_pundit(this, club, player));
    if pundit.is_null() {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        s.put_person(pundit);
    }
    let (mut hosts, mut next_hosts) = (0u32, u32::MAX);
    exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), comp_id, &mut hosts, &mut next_hosts);
    s.put(hosts);
    s.put(next_hosts);
    s.put(if fixture != 0 { news_shootout_result(Fixture(fixture), club).unwrap_or(0) } else { 0 });
    if !pundit.is_null() && !pundit.club_job().is_null() {
        s.put(pundit.job_for_club() as i8 as i32 as u32);
        s.put(read_u32(pundit.club_job().addr()));
    } else if !pundit.is_null() && !pundit.national_job().is_null() {
        s.put(pundit.job_for_nation() as i8 as i32 as u32);
        s.put(read_u32(pundit.national_job().addr()));
    } else {
        s.put(u32::MAX);
        s.put(u32::MAX);
    }
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    let nation = club.nation();
    s.put(exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation.is_null() { -1 } else { read_i32(nation.addr()) }) as u32);
    s.put(player.age() as i8 as i32 as u32);
    s.put((player.player().goalkeeper() == 0x14) as u32);
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x25a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    let news = read_usize(G_NEWS);
    if kind == PRAISE_WORLD_STAGE {
        if player.player().world_reputation() as i32 + club.reputation() as i32 > 0x35b6 {
            exe::news_send_to_all_managers(news, s.n);
        } else {
            exe::news_send_to_continent_managers(news, s.n, nation.continent());
        }
    } else if kind == PRAISE_COMPETITION {
        exe::news_send_to_all_managers(news, s.n);
    } else {
        for &c in clubs.iter().take(count.max(0) as usize) {
            exe::news_send_to_club_manager(news, s.n, c);
        }
    }
    log_story(this, player.addr(), club.addr(), 0xbdd, kind as u32, response);
    let sheet = player.player();
    sheet.set_current_reputation(sheet.current_reputation().wrapping_add(200).min(10000));
    sheet.set_home_reputation(sheet.home_reputation().wrapping_add(200).min(10000));
    sheet.set_world_reputation(sheet.world_reputation().wrapping_add(100).min(10000));
    exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), player, 1);
    let other = club_at(exe::utils_random(read_i32(G_CLUB_COUNT)) as isize);
    if !other.nation().is_null() && other.nation().addr() == club.nation().addr() {
        exe::staff_add_favoured_staff(other.chairman(), player, 1);
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_apply_player_story_response(this, player, club, kind as u32, response);
    }
}

/// Comparer for `post_player_praise_story`: it draws, posts news and changes reputations, so it cannot run twice; only
/// the original runs (in the comparison mode the post-match gate of 0x744240 is installed first and keeps the address,
/// see the item).
pub(crate) unsafe extern "thiscall" fn compare_post_player_praise_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(%%I0%%)(this, player, club, kind, fixture)
}
