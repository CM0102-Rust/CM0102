// --- sx01e_2: Media::create_player_media_story (0x72c1f0, media.cpp) ---

/// `Media::create_player_outburst_story` (0x733a60, thiscall(media, record, fixture), ret 8; sx01d_5).
const FN_MEDIA_CREATE_PLAYER_OUTBURST_STORY: usize = 0x733a60;

/// Hands a player's outburst at the club to `create_player_outburst_story` (through the exe, as the original), with
/// a record on the stack: +0 the player's id, +4 the club id, +0xc 0, +0xe 0 (no assistant's report), +0xf -1 (no
/// other person), +0x13 the outburst kind (0x1e or 0x52); no match.
unsafe fn post_player_outburst_record(this: usize, player: Staff, club: Club, kind: u16) {
    let mut story = [0u8; 0x17];
    let r = story.as_mut_ptr() as usize;
    write_u32(r, read_u32(player.addr()));
    write_u32(r + 4, read_u32(club.addr()));
    write_u32(r + 0xf, u32::MAX);
    write_u16(r + 0x13, kind);
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MEDIA_CREATE_PLAYER_OUTBURST_STORY);
    f(this, r, 0);
}

/// The player's temper score for the media story: with one of the contract grievances 0x30321, 3 for a
/// professionalism of 1, 2 below 6, 1 below 15, +1 for an ambition above 15, +2 for a temperament of 1 and +1 below 6,
/// +1 for a pressure below 10; without them, 1 for a professionalism of 1.
unsafe fn media_story_temper(player: Staff, contract: Contract) -> i8 {
    let professionalism = player.professionalism();
    if contract.is_null() || contract.grievances() & 0x30321 == 0 {
        return (professionalism == 1) as i8;
    }
    let mut score: i8 = if professionalism == 1 {
        3
    } else if professionalism < 6 {
        2
    } else if professionalism < 0xf {
        1
    } else {
        0
    };
    if player.ambition() > 0xf {
        score += 1;
    }
    if player.temperament() == 1 {
        score += 2;
    }
    if player.temperament() < 6 {
        score += 1;
    }
    if player.pressure() < 10 {
        score += 1;
    }
    score
}

/// The first story log record (`media_fields::STORY_LOG_COUNT` records of 0x17 bytes) about the player
/// (`story_log_entry::PERSON_A`) answered with response 2.
unsafe fn has_logged_response_two(this: usize, player: Staff) -> bool {
    let n = read_i32(this + media_fields::STORY_LOG_COUNT);
    (0..n.max(0) as usize).any(|k| {
        let r = read_usize(this).wrapping_add(k * 0x17);
        read_i32(r + story_log_entry::PERSON_A) == read_i32(player.addr()) && read_u8(r + story_log_entry::RESPONSE) == 2
    })
}

/// The kind of the player's media story, refined from the kind asked for (0 = criticism of form, 1, 2 = criticism of
/// contract talk, 0xd, 0x23...). `None` when the player blew up instead (`post_player_outburst_record` with 0x1e).
///
/// 1. For kinds 0, 1, 2 and 0x23, a professionalism below the temper score + 6, loyalty below 15 and temperament below
///    10: when random(morale) < random(-75) and random(professionalism) < random(score + 10), kind 2 becomes 0x1e with
///    random(3) = 0, otherwise random(4) = 0 gives 0x20 and anything else the outburst.
/// 2. A logged story about him answered with 2 adds 5 to the score.
/// 3. Loyalty below 15, temperament below 10, professionalism below score + 6 and random(professionalism) <
///    random(score + 7): kinds 0, 2 and 0xd become 0x21; the others, with a logged story answered with 2 and a manager
///    at the club for more than 45 days, 0x22.
/// 4. Otherwise kinds 0 and 2: younger than 18 + random(2), a current reputation below random(7000) and a deserved
///    squad status of 1, 2 or 3 (`exe::contract_manager_get_deserved_squad_status`) give 7; otherwise a goalkeeper over
///    34, a defender over 33 or any other player over 32 gives 4 with random(3) = 0.
/// 5. The contract grievances 0x2000 or 8 with random(professionalism) < random(10) + 5 give 0x21; otherwise kind 2
///    with a wage above 7500 and a squad status 0x10, 0x20 or 0x30 gives 9 when random(consistency) < random(25).
/// 6. A goalkeeper's kinds 0, 2 and 0xd become 0xe.
unsafe fn media_story_kind(this: usize, player: Staff, club: Club, contract: Contract, kind: u32, score: &mut i8) -> Option<u32> {
    let sheet = player.player();
    let mut kind = kind;
    let is = |kind: u32, kinds: &[u16]| kinds.contains(&(kind as u16));
    let professionalism = player.professionalism() as i32;
    if is(kind, &[0, 2, 0x23, 1]) && professionalism < *score as i32 + 6 && player.loyalty() < 0xf && player.temperament() < 10 {
        let morale = exe::utils_random(contract.morale() as i32);
        if morale < exe::utils_random(-0x4b) {
            let limit = exe::utils_random(*score as i32 + 10);
            if exe::utils_random(professionalism) < limit {
                if exe::utils_random(3) == 0 && kind as u16 == 2 {
                    kind = 0x1e;
                } else if exe::utils_random(4) == 0 {
                    kind = 0x20;
                } else {
                    covered!("sx01e_2 media story outburst");
                    post_player_outburst_record(this, player, club, 0x1e);
                    return None;
                }
            }
        }
    }
    if has_logged_response_two(this, player) {
        *score += 5;
    }
    let mut refined = None;
    if player.loyalty() < 0xf && player.temperament() < 10 && professionalism < *score as i32 + 6 {
        let limit = exe::utils_random(*score as i32 + 7);
        if exe::utils_random(professionalism) < limit {
            if is(kind, &[0, 2, 0xd]) {
                refined = Some(0x21);
            } else if has_logged_response_two(this, player) {
                let manager = club.manager();
                if !manager.is_null() && exe::date_days_since(G_CURRENT_DATE, read_u32(manager.addr() + staff::DATE_JOINED_CLUB), read_u32(manager.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) > 0x2d {
                    refined = Some(0x22);
                }
            }
        }
    }
    if let Some(k) = refined {
        kind = k;
    } else if is(kind, &[0, 2])
        && (player.age() as i8 as i32) < exe::utils_random(2) + 0x12
        && (sheet.current_reputation() as i32) < exe::utils_random(7000)
        && (1..=3).any(|s| exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), player, contract) == s)
    {
        kind = 7;
    } else if is(kind, &[0, 2]) {
        let age = player.age() as i8;
        let old = if sheet.goalkeeper() == 0x14 { age > 0x22 } else if sheet.defender() == 0x14 { age > 0x21 } else { age > 0x20 };
        if old && exe::utils_random(3) == 0 {
            kind = 4;
        }
    }
    let grievances = contract.grievances();
    if (grievances & 0x2000 != 0 || grievances & 8 != 0) && exe::utils_random(professionalism) < exe::utils_random(10) + 5 {
        kind = 0x21;
    } else if kind as u16 == 2 && contract.wage() > 0x1d4c && matches!(contract.squad_status() & 0xf0, 0x30 | 0x10 | 0x20) {
        if exe::utils_random(sheet.consistency() as i32) < exe::utils_random(0x19) {
            kind = 9;
        }
    }
    if sheet.goalkeeper() == 0x14 && is(kind, &[0, 2, 0xd]) {
        kind = 0xe;
    }
    Some(kind)
}

/// The variant of the media story (slot 5), by kind: 0 random(12); 1 random(4) within a year at the club, then
/// random(2) + 4 up to 32 or random(3) + 4 older; 2 random(1); 3 random(5) for a capped player under 32 with a current
/// reputation above 6250, otherwise random(4) + 1; 4 and 0x20 random(3); 5, 7, 8 and 0x22 random(2); 9 from the rival
/// manager (without one random(4) + 2; random(his +0x5d) < random(20) gives random(2), otherwise random(6)), a 3 under
/// 21 with a current reputation above 6250 becoming random(2) + 4; 0xd and 0x21 random(4); 0xe by consistency below 6
/// (random(4) within a year, random(5) after) or not (random(2) + 2 within a year, random(3) + 2 after); 0x11
/// random(3); 0x23 3 up to 23 with a current reputation up to 6250, otherwise random(3); the others 0.
unsafe fn media_story_variant(player: Staff, kind: u32, rival: i32) -> i32 {
    let sheet = player.player();
    let age = player.age() as i8;
    let days_at_club = || exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    let random = |n: i32| exe::utils_random(n);
    match kind as u16 as i16 {
        0 => random(12),
        1 => {
            if days_at_club() < 0x16d {
                random(4)
            } else if age > 0x20 {
                random(3) + 4
            } else {
                random(2) + 4
            }
        }
        2 => random(1),
        3 => {
            if player.int_apps() != 0 && age < 0x20 && sheet.current_reputation() > 0x186a {
                random(5)
            } else {
                random(4) + 1
            }
        }
        4 | 0x20 => random(3),
        5 | 7 | 8 | 0x22 => random(2),
        9 => {
            let v = if rival == -1 {
                random(4) + 2
            } else {
                let temper = random(staff_at(rival as isize).temperament() as i32);
                if temper < random(0x14) { random(2) } else { random(6) }
            };
            if v == 3 && age < 0x15 && sheet.current_reputation() > 0x186a {
                random(2) + 4
            } else {
                v
            }
        }
        0xd | 0x21 => random(4),
        0xe => {
            let settled = days_at_club() > 0x16d;
            match (sheet.consistency() < 6, settled) {
                (true, false) => random(4),
                (true, true) => random(5),
                (false, false) => random(2) + 2,
                (false, true) => random(3) + 2,
            }
        }
        0x11 => random(3),
        0x23 => {
            if age > 0x17 || sheet.current_reputation() > 0x186a {
                random(3)
            } else {
                3
            }
        }
        _ => 0,
    }
}

/// The team story that may replace the player's media story when his weakness flags have 0x200200 and none of 0x180:
/// with his attribute 0xc above random(4) + the club reputation / 1250 + 8 and the club's division object in
/// `G_COMP_OBJECT_TABLE`, either his outburst (kind 0x52: loyalty below 15, professionalism below the score + 5 and
/// random(loyalty − trunc(morale × −0.1) + professionalism) < random(50)), or, with the division's stats (+8), a
/// stat 3 of the club up to 0, its stat 1 × 1.4 above its stat 10 and random(4) = 0, when he holds a selection slot
/// 0..10 (`exe::tactics_find_selection_slot`) whose position (`exe::formation_get_slot_position` of the club's tactic)
/// has 0x40, the team story 0x52 about him (`exe::media_post_team_media_story`). True when one of them was posted.
unsafe fn media_story_team_takeover(this: usize, player: Staff, club: Club, contract: Contract, flags: u32, score: i8) -> bool {
    if flags & 0x200200 != 0x200200 || flags & 0x180 != 0 {
        return false;
    }
    let threshold = exe::utils_random(4) + club.reputation() as i32 / 1250 + 8;
    if (exe::staff_get_player_attribute(player, 0xc) as i32) <= threshold {
        return false;
    }
    let division = club.division();
    if division.is_null() {
        return false;
    }
    let comp_object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(division.addr())));
    if comp_object == 0 {
        return false;
    }
    let loyalty = player.loyalty() as i32;
    let professionalism = player.professionalism() as i32;
    if loyalty < 0xf && professionalism < score as i32 + 5 {
        let morale = ftol(contract.morale() as f64 * read_f64(rdata::F64_NEG_0_1));
        let temper = exe::utils_random(loyalty - morale + professionalism);
        if temper < exe::utils_random(0x32) {
            covered!("sx01e_2 media story team outburst");
            post_player_outburst_record(this, player, club, 0x52);
            return true;
        }
    }
    let stats = read_usize(comp_object + 8);
    if stats == 0 || exe::comp_stats_get_club_stat(stats, club, 3) > read_f32(rdata::F32_0) as f64 {
        return false;
    }
    let lost = exe::comp_stats_get_club_stat(stats, club, 10);
    if exe::comp_stats_get_club_stat(stats, club, 1) * read_f64(rdata::F64_1_4) <= lost || exe::utils_random(4) != 0 {
        return false;
    }
    let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, player, 0) as i8;
    if !(0..0xb).contains(&slot) {
        return false;
    }
    let tactic = exe::tactics_get_club_tactic(read_usize(G_TACTICS), club, 0);
    if exe::formation_get_slot_position(tactic, slot as u32, 0) & 0x40 == 0 {
        return false;
    }
    covered!("sx01e_2 media story becomes a team story");
    exe::media_post_team_media_story(this, club, 0x52, player.addr() as u32, 0usize, 0, -1);
    true
}

/// The slots from 0x1c: with a match (and a kind other than 0x13) 1, the match record and the shoot-out
/// (`NewsSlots::put_match`), his strength flags, his position mask and the pundit; otherwise 0, the strength flags,
/// the position mask, the pundit, his natural position mask (`exe::staff_get_natural_position_mask`), the club's
/// injured squad count (the original asks `exe::injury_get_player_injury_status` about the player himself once per
/// filled squad slot, a defect: the count is 0 or the number of filled slots), whether the computer manager's man
/// handling is below 10, and the other person with his names (-1 × 4 without him).
unsafe fn media_story_tail_slots(s: &mut NewsSlots, record: &mut [u8; 0x20], player: Staff, club: Club, kind: u32, fixture: usize, person: Staff, pundit: u32) {
    s.k = 0x1c;
    if fixture != 0 && kind as u16 != 0x13 {
        covered!("sx01e_2 media story after a match");
        s.put(1);
        s.put_match(record, Fixture(fixture), club);
        s.put(exe::staff_get_player_strength_flags(player, club));
        s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
        s.put(pundit);
        return;
    }
    s.put(0);
    s.put(exe::staff_get_player_strength_flags(player, club));
    s.put(exe::formation_get_position_mask(player.player()) as i16 as i32 as u32);
    s.put(pundit);
    s.put(exe::staff_get_natural_position_mask(player));
    let mut injured: i8 = 0;
    for k in 0..50 {
        if club.squad(k) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), player) == 2 {
            injured += 1;
        }
    }
    s.put(injured as i32 as u32);
    let manager = club.manager();
    let careless = media_is_human_club(club.addr(), 0, 1) == 0 && !manager.is_null() && !manager.non_player().is_null() && (manager.non_player().man_handling() as i8) < 10;
    s.put(careless as u32);
    if person.is_null() {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        s.put_person(person);
    }
}

/// Creates a media story about a player (news item 0xbd0), refining the kind asked for from his form, contract and
/// character, and the computer manager's response.
///
/// Nothing without the player, his `.player()` sheet with a `.current_reputation()` of at least 3500, his
/// `.nation()`, the club, its `.nation()`, `.stadium()` with a city and `.manager()`. The contract is his with his
/// club job's club or, failing that, with the club (`exe::contract_manager_get_contract_for_club`); nothing without
/// one. Without a club job the original first reads an uninitialised stack slot as the contract (a defect; here
/// none). The rival manager comes from `exe::media_pick_rival_manager_id`.
///
/// The kind (`media_story_kind`, which may post the outburst instead), then nothing when the story is already logged
/// (`exe::media_has_logged_story` with 0xbd0). The slots: 0..3 the player and his names (the common name -1 when he
/// has none), 4 the club id, 5 the variant (`media_story_variant`), 6 the outlet
/// (`exe::media_pick_outlet_type_for_person`), 7 his nation id, 8 the stadium's city id, 9 the kind, 10 the computer
/// manager's response (`exe::media_pick_ai_manager_response`, for a club that is not human; otherwise 0), 11..14 the
/// rival manager and his names (-1 × 4 without one), 15 the contract's `.unhappy_flag()` up to 1, 16 the high nibble
/// of the `.squad_status()`, 17 his age, 18 his weakness flags (`staff_get_player_weakness_flags`) without 8 when his
/// stat 6 (`exe::playerstats_get_stat`) is below 20. Then the team takeover (`media_story_team_takeover`) may post
/// another story and stop. 0x17..0x1a the club's manager and his names, 0x1b the rival manager's club id (-1), and the
/// tail (`media_story_tail_slots`) with the pundit (`exe::punditlist_pick_random_pundit` with the club's nation).
///
/// It goes to every human club following the player (`exe::transfer_manager_collect_player_news_recipients` with
/// 0x2da, up to 30) and is logged (`log_story` with the response). For a kind other than 0x13 with random(pressure)
/// < random(5), or with a morale below −70 − professionalism, the contract gets the event flag 0x40000
/// (`exe::staff_contract_apply_event_flags`); the morale moves by random(pressure − 30), within ±100. Kinds 0x1e and
/// 0x20 let the squad react (`exe::media_apply_squad_reaction_to_player_story`); the others, when random(150) is below
/// the chairman's interference and the contract has grievances, make the chairman dislike him
/// (`exe::staff_add_disliked_staff`). Last, for a club that is not human, the computer manager's response is applied
/// (`exe::media_apply_player_story_response`).
///
/// Called by the post-match events (0x6b7012, 0x6b75fd, 0x6b796e), the daily loop (0x5cff53, 0x5d0a64...), 0x4194e0
/// and 0x7581b2; behind the post-match gate (`POS_PROLOGUES`, match_day.rs).
/// Original: `Media::create_player_media_story` (0x72c1f0, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn create_player_media_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize, person: usize) {
    let mut record = [0u8; 0x20]; // the 0x1f-byte match record
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let (player, club, person) = (Staff(player), Club(club), Staff(person));
    if player.is_null() || player.player().is_null() || player.player().current_reputation() < 0xdac || player.nation().is_null() {
        return;
    }
    if club.is_null() || club.nation().is_null() || club.stadium().is_null() || read_usize(club.stadium().addr() + stadium::CITY) == 0 || club.manager().is_null() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let mut s = NewsSlots { n: local.as_mut_ptr() as usize, k: 0 };
    exe::new_newsitem(s.n, 0xbd0, 0);
    set_person_name_slots_or_none(s.n, 0, player);
    s.k = 4;
    s.put(read_u32(club.addr()));
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let mut contract = Contract(0);
    if !player.club_job().is_null() {
        contract = Contract(exe::contract_manager_get_contract_for_club(contracts, player, player.club_job()));
    }
    if contract.is_null() {
        contract = Contract(exe::contract_manager_get_contract_for_club(contracts, player, club));
        if contract.is_null() {
            return;
        }
    }
    let manager = club.manager();
    let rival = exe::media_pick_rival_manager_id(this, club);
    let mut score = media_story_temper(player, contract);
    let Some(kind) = media_story_kind(this, player, club, contract, kind, &mut score) else {
        return;
    };
    if exe::media_has_logged_story(this, player, club, 0xbd0, kind, core::ptr::null()) != 0 {
        return;
    }
    covered!("sx01e_2 media story");
    s.put(media_story_variant(player, kind, rival) as u32);
    let outlet = exe::media_pick_outlet_type_for_person(player);
    s.put(outlet);
    s.put(read_u32(player.nation().addr()));
    s.put(read_u32(read_usize(club.stadium().addr() + stadium::CITY)));
    s.put(kind as u16 as i16 as i32 as u32);
    let response = if media_is_human_club(club.addr(), 0, 1) == 0 { exe::media_pick_ai_manager_response(this, manager, player, kind) } else { 0 };
    s.put(response);
    if rival == -1 {
        for _ in 0..4 {
            s.put(u32::MAX);
        }
    } else {
        let r = staff_at(rival as isize);
        s.put(rival as u32);
        for name in [r.first_name(), r.second_name(), r.common_name()] {
            s.put(read_u32(name + names::ID));
        }
    }
    s.put((contract.unhappy_flag() <= 1) as u32);
    s.put((contract.squad_status() >> 4) as u32);
    s.put(player.age() as i8 as i32 as u32);
    let mut flags = staff_get_player_weakness_flags(player.addr(), club.addr());
    let stat = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 6, 1, 6.5);
    if flags & 8 != 0 && stat < read_f32(rdata::F32_20) as f64 {
        flags ^= 8;
    }
    s.put(flags);
    if media_story_team_takeover(this, player, club, contract, flags, score) {
        return;
    }
    s.k = 0x17;
    s.put_person(manager);
    let rival_club = if rival > -1 { staff_at(rival as isize).club_job() } else { Club(0) };
    s.put(if rival_club.is_null() { u32::MAX } else { read_u32(rival_club.addr()) });
    let nation = club.nation();
    let pundit = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, outlet, if nation.is_null() { -1 } else { read_i32(nation.addr()) }) as u32;
    media_story_tail_slots(&mut s, &mut record, player, club, kind, fixture, person, pundit);
    let mut clubs = [0usize; 30];
    let count = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0x2da, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for &c in clubs.iter().take(count.max(0) as usize) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), s.n, c);
    }
    log_story(this, player.addr(), club.addr(), 0xbd0, kind, response);
    let professionalism = player.professionalism() as i32;
    let pressured = kind as u16 != 0x13 && exe::utils_random(player.pressure() as i32) < exe::utils_random(5);
    if pressured || (contract.morale() as i32) < -0x46 - professionalism {
        exe::staff_contract_apply_event_flags(contract, 0x40000, 0);
    }
    let change = exe::utils_random(player.pressure() as i32 - 0x1e) as i8;
    let morale = change as i32 + contract.morale() as i32;
    contract.set_morale(if morale > 100 { 100 } else if morale < -100 { -100 } else { change.wrapping_add(contract.morale()) });
    if kind as u16 == 0x1e || kind as u16 == 0x20 {
        exe::media_apply_squad_reaction_to_player_story(this, kind, player, club, core::ptr::null_mut(), u32::MAX);
    } else {
        let chairman = club.chairman();
        if !chairman.is_null() && !chairman.non_player().is_null() && (chairman.non_player().interference() as i32) > exe::utils_random(0x96) && contract.grievances() != 0 {
            exe::staff_add_disliked_staff(chairman, player, 1);
        }
    }
    if media_is_human_club(club.addr(), 0, 1) == 0 {
        exe::media_apply_player_story_response(this, player, club, kind, response);
    }
}

/// Comparer for `create_player_media_story`: it draws, posts news and changes the contract, so it cannot run twice;
/// only the original runs (in the comparison mode the post-match gate of 0x72c1f0 is installed first and keeps the
/// address, see the item).
pub(crate) unsafe extern "thiscall" fn compare_create_player_media_story(this: usize, player: usize, club: usize, kind: u32, fixture: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize)>(%%I0%%)(this, player, club, kind, fixture, person)
}
