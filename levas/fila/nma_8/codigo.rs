// ---------- batch %%LEVA%%: the action button of a news item (news.cpp) ----------

/// `Friendly::get_tournament(index, date)` (0x5c89a0, friendly.cpp, thiscall, ret 8): the tournament record at an
/// index when it is in use and, with a date, the date matches; otherwise 0.
const FN_FRIENDLY_GET_TOURNAMENT: usize = 0x5c89a0;
/// `read_staff_name_ref_from_news(item, &slot, out)` (0x7e52c0, record_utils.cpp, cdecl): reads 0x18 bytes (four
/// dwords and a date) from the item's slots from `slot` on, moving `slot`.
const FN_READ_STAFF_NAME_REF_FROM_NEWS: usize = 0x7e52c0;
/// `read_match_record_from_news(item, &slot, out)` (0x7e4fa0, record_utils.cpp, cdecl): reads a 0x1f-byte match record
/// (date, competition +8, club index +0x11...) from the item's slots from `slot` on, moving `slot`.
const FN_READ_MATCH_RECORD_FROM_NEWS: usize = 0x7e4fa0;

/// The `cup_tie::FIXTURE_STATE` of a tie whose fixture was created.
const TIE_FIXTURE_CREATED: u8 = 3;

/// Address of data slot `k` of a news item (`news_item::DATA` + 4 × k).
fn action_slot(news: usize, k: usize) -> usize {
    news + news_item::DATA + 4 * k
}

/// Writes the button label (the game's `langlib_sprintf`, so the text goes through the language file) and the screen
/// handler, and returns `r`.
unsafe fn set_action_button(label: usize, handler: *mut u32, text: usize, screen: u32, r: u8) -> u8 {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    sprintf(label, text);
    *handler = screen;
    r
}

/// "Respond" (0xa69488) with the screen handler, needing a reply (2).
unsafe fn respond_button(label: usize, handler: *mut u32, screen: u32) -> u8 {
    set_action_button(label, handler, 0xa69488, screen, 2)
}

/// The piece of `get_action_button` a news item goes to, by the thousands of its type (`get_category`); for the comparer.
fn action_button_piece(kind: i32) -> &'static str {
    match kind {
        1..=999 => "general",
        2000..=2999 => "discipline",
        3000..=3999 => "player",
        4000..=4999 => "transfer",
        5000..=5999 => "reply",
        6000..=0x1790 => "board",
        _ => "none",
    }
}

/// The board news (6000s). Type 0x1774 with slot 1 (byte) 0: "Respond", handler 0x6b2450, 2. Type 0x1786 with slot 1
/// (byte) other than 5, slot 2 (byte) other than 0 and 6 and slot 5 (byte) 0: the "Ultimatum" label (the
/// `<%s - COMMENT (Issue ultimatum to board)>` template 0xa69454 with the text at 0xad9c64), handler 0x9064a0, 1.
/// Anything else: `None`, and the routine goes on to the other categories (which a board item does not match).
unsafe fn board_button(news: usize, label: usize, handler: *mut u32) -> Option<u8> {
    covered!("nma_8 board button");
    let s = |k: usize| action_slot(news, k);
    match read_i32(news) {
        0x1774 => {
            if read_u8(s(1)) == 0 { // nma8: the board request needs a reply
                return Some(respond_button(label, handler, 0x6b2450));
            }
        }
        0x1786 => {
            let (a, b, c) = (read_u8(s(1)), read_u8(s(2)), read_u8(s(5)));
            if a != 5 && b != 0 && b != 6 && c == 0 {
                let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                sprintf(label, 0xa69454usize, 0xad9c64usize); // "Ultimatum<%s - COMMENT (Issue ultimatum to board)>"
                *handler = 0x9064a0;
                return Some(1);
            }
        }
        _ => {}
    }
    None
}

/// The general news (1..999): friendly and tournament invitations, cup draws, national squads, awards.
///
/// - 6, the friendly invitation (slots 0 and 1 the clubs, 2 the club asked, 3 and 4 day and year, 5 the state): the
///   state 3 or 4 gives 0. The club of the manager is club 1 when club 2 is club 0, otherwise club 0; it must be his
///   club job or his national job. The friendly of club 2 on the date (`exe::friendly_find_friendly_match`) and
///   `friendly_check_match_invitation(club 0, club 1, his club, day, year, friendly, no text)` other than 0: "Respond",
///   handler 0x4740c0, 2. The original passes day and year as dwords with the stack's upper halves; the routine does
///   not read them without a text.
/// - 9, the tournament invitation (slot 1 the club, 2 the tournament index, 3 the state, 5 and 6 day and year): state 3
///   or 4 gives 0; the tournament (`Friendly::get_tournament`) and `friendly_check_tournament_invitation(club,
///   tournament, no text)` other than 0: "Respond", handler 0x474720, 2.
/// - 0x16: the manager's club with the id in slot 1, slot 2 (word) equal to the word +0x439 of the object at
///   `G_UEFA_SEEDING` and slot 0 = 0: "Respond", 0x49cc60, 2.
/// - 0x18: his club with the id in slot 0 and slot 1 = 0: "Respond", 0x475070, 2. 0x19: the same with slot 3 = 0:
///   "Respond", 0x4752b0, 2. 0x1e: slot 2 = 0: "Respond", 0x7798c0, 2.
/// - 0x22, the national squad (slot 4 the club, 5 the squad slot, 7 the answer): slot 7 other than 0 gives 0. A news
///   item not of today (`news_item::DATE` against `G_CURRENT_DATE`) gets slot 7 = 1 (`set_item_slot` in the human's
///   inbox and the item) and 0. Otherwise the person mark (`read_person_mark` of `G_NATIONAL_TEAMS`) of the club's
///   nation for `exe::national_teams_get_squad_slot(club, slot 5)`, other than 0: "Respond", 0x77a140, 2.
/// - 0x27, the cup draw (slot 0 the competition, 1 the stage label, 4 the stage index or 0xff, 5 the round, 6 the
///   answer): slot 6 other than 0, no competition (`G_COMP_OBJECT_TABLE`), or a stage index above
///   `cup::CURRENT_STAGE` (signed): 0. The cup (the competition with 0xff, otherwise its stage) must be a cup
///   (`cup::COMP_TYPE` 1) with the stage label of slot 1. When the round is above `cup::CURRENT_ROUND`, the last tie of
///   the round before must have its fixture created. A tie of the round without its fixture: "View Draw", 0x49eac0, 1;
///   all created: slot 6 = 1 (`set_item_slot`) and 0.
/// - 0x28: the FA Cup (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) with `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 1 and slot 0 =
///   1: "Respond", 0x5708f0, 2.
/// - 0x1c, the award nominations: the award (slot 0) and its item (slot 1) in the `G_AWARD_MANAGER` tables with a
///   nominee list (+0x1f) whose +4 word is not 0: "Nominations", 0x416cb0, 1.
/// - anything else: 0.
unsafe fn general_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 general button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    match read_i32(news) {
        6 => {
            let (a, b, c) = (slot(0) as i32, slot(1) as i32, slot(2) as i32);
            let (day, year) = (read_u16(s(3)), read_u16(s(4)));
            let state = read_u8(s(5));
            if state == 3 || state == 4 { // nma8: the friendly invitation is closed
                return 0;
            }
            let mine = if c == a { club_at(b as isize) } else { club_at(a as isize) };
            if me.club_job() != mine && me.national_job() != mine {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let found = exe::friendly_find_friendly_match(friendly, date.as_mut_ptr() as usize, club_at(c as isize));
            if friendly_check_match_invitation(friendly, club_at(a as isize).addr(), club_at(b as isize).addr(), mine.addr(), day as u32, year as u32, found, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x4740c0)
        }
        9 => {
            let state = read_u8(s(3));
            let index = read_u8(s(2));
            let club = club_at(slot(1) as i32 as isize);
            let (day, year) = (read_u16(s(5)), read_u16(s(6)));
            if state == 3 || state == 4 {
                return 0;
            }
            let mut date = [0u8; 8];
            exe::new_date(date.as_mut_ptr() as usize, day as u32, year as u32);
            let friendly = read_usize(G_FRIENDLY);
            let get_tournament: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_FRIENDLY_GET_TOURNAMENT);
            let tournament = get_tournament(friendly, index as u32, date.as_mut_ptr() as usize);
            if friendly_check_tournament_invitation(friendly, club.addr(), tournament, 0, 0) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x474720)
        }
        0x16 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(1) || read_u16(s(2)) != read_u16(read_usize(G_UEFA_SEEDING) + 0x439) || slot(0) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x49cc60)
        }
        0x18 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(1) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x475070)
        }
        0x19 => {
            let club = me.club_job();
            if club.is_null() || read_u32(club.addr()) != slot(0) || slot(3) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4752b0)
        }
        0x1e => {
            if slot(2) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x7798c0)
        }
        0x22 => {
            if slot(7) != 0 {
                return 0;
            }
            if read_u16(G_CURRENT_DATE_YEAR) != read_u16(news + news_item::DATE + 2) || read_u16(G_CURRENT_DATE) != read_u16(news + news_item::DATE) {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, 1, news);
                return 0;
            }
            let club = club_at(slot(4) as i32 as isize);
            let k = exe::national_teams_get_squad_slot(club, slot(5));
            if read_person_mark(read_usize(G_NATIONAL_TEAMS), read_usize(club.addr() + club::NATION), k as i8) == 0 {
                return 0;
            }
            respond_button(label, handler, 0x77a140)
        }
        0x27 => {
            let stage_label = read_u16(s(1));
            let stage = read_u8(s(4));
            let round = read_i16(s(5));
            if slot(6) != 0 {
                return 0;
            }
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            if comp == 0 || (stage as i8 as i32) > read_i32(comp + cup::CURRENT_STAGE) {
                return 0;
            }
            let cup = if stage == 0xff { comp } else { read_usize(read_usize(comp + cup::STAGES).wrapping_add((stage as i8 as isize as usize).wrapping_mul(4))) };
            if read_u8(cup + cup::COMP_TYPE) != 1 || read_u16(cup + cup::STAGE_LABEL) != stage_label {
                return 0;
            }
            let r = round as i32 as usize;
            if (read_u8(cup + cup::CURRENT_ROUND) as i8 as i16) < round {
                // the last tie of the round before must have its fixture
                let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_sub(1).wrapping_mul(0x68)) + 0x1a) as i32 as usize;
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_sub(1).wrapping_mul(4)));
                if read_u8(tie_table.wrapping_add(ties.wrapping_sub(1).wrapping_mul(0x35)) + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                    return 0;
                }
            }
            let ties = read_i16(read_usize(cup + cup::ROUND_TABLE).wrapping_add(r.wrapping_mul(0x68)) + 0x1a);
            if ties > 0 {
                let tie_table = read_usize(read_usize(cup + cup::TIE_TABLES).wrapping_add(r.wrapping_mul(4)));
                for k in 0..ties {
                    if read_u8(tie_table + k as usize * 0x35 + cup_tie::FIXTURE_STATE) != TIE_FIXTURE_CREATED {
                        return set_action_button(label, handler, 0xa69448, 0x49eac0, 1); // "View Draw"
                    }
                }
            }
            set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 6, 1, news);
            0
        }
        0x28 => {
            let fa_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_ENGLISH_FA_CUP).wrapping_mul(4)));
            if read_u8(fa_cup + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 1 || slot(0) != 1 {
                return 0;
            }
            respond_button(label, handler, 0x5708f0)
        }
        0x1c => {
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8).wrapping_add((slot(0) as usize).wrapping_mul(4)));
            let item = read_usize(read_usize(award + 4).wrapping_add((slot(1) as usize).wrapping_mul(4)));
            let nominees = read_usize(item + 0x1f);
            if nominees == 0 || read_u16(nominees + 4) == 0 {
                return 0;
            }
            set_action_button(label, handler, 0x987ff8, 0x416cb0, 1) // "Nominations"
        }
        _ => 0,
    }
}

/// The player news (3000s): contracts, media stories, bids for a player.
///
/// - 0xbbf: the manager's club job with the id in slot 0 and slot 7 (byte) 0: "Respond", 0x4dca40, 2.
/// - 0xbe8: the club of slot 8 managed by him (`club::MANAGER`), slot 18 (byte) 0 and slot 15 = 0: "Respond",
///   0x741f50, 2. 0xbe6: the club of slot 9 managed by him and slot 20 (byte) 0: the same.
/// - 0xbd9: slot 8 (byte) 0, then as 0xbd0. 0xbd0: the club of slot 4 managed by him and slot 10 (byte) 0: "Respond",
///   0x741f50, 2. 0xbdd: the same with slot 13 (byte). 0xbdf: the same with slot 10 (byte), except for a story kind
///   (slot 9, signed byte) of 0x3e, 0x50, 0x51 or 0x5e, which gives 0 (the original's jump table).
/// - 0xbc1: his club job with the id in slot 0 and slot 6 (byte) 0: "Respond", 0x8f66d0, 2. 0xbc2: the same with slot
///   7 (byte): "Offer Contract", 0x4dc9b0, 1.
/// - 0xbb8, the contract talks of a player (slot 0 the person, 4 the club, 8 the transfer offer or −1, 9 its serial,
///   10 the state): his club job with the id in slot 4; with an offer, the offer (`exe::transfer_manager_get_offer_slot`)
///   with the serial of slot 9; without one, slot 10 other than 5; then the contract offer of the person at the club
///   (`exe::contract_manager_find_contract_offer`) with `contract_offer::STATE` 3: "Negotiate", 0x4e5770, 2.
/// - anything else: 0.
unsafe fn player_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 player button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let me = Staff(manager);
    let managed_by_me = |k: usize| club_at(slot(k) as i32 as isize).manager().addr() == manager;
    let own_club_is = |id: u32| !me.club_job().is_null() && read_u32(me.club_job().addr()) == id;
    let story = |club_slot: usize, flag_slot_addr: usize| {
        if !managed_by_me(club_slot) || read_u8(flag_slot_addr) != 0 {
            return 0;
        }
        respond_button(label, handler, 0x741f50)
    };
    match read_i32(news) {
        0xbbf => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x4dca40)
        }
        0xbe8 => {
            if !managed_by_me(8) || read_u8(s(18)) != 0 || slot(15) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x741f50)
        }
        0xbe6 => story(9, s(20)),
        0xbd9 => {
            if read_u8(s(8)) != 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdf => {
            // the original's jump table on (story kind - 0x3e), 0..0x20: 0 ends the routine
            const KINDS: [u8; 33] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];
            let k = (read_u8(s(9)) as i8 as i32).wrapping_sub(0x3e) as u32;
            if k <= 0x20 && KINDS[k as usize] == 0 {
                return 0;
            }
            story(4, s(10))
        }
        0xbdd => story(4, s(13)),
        0xbd0 => story(4, s(10)),
        0xbc1 => {
            if !own_club_is(slot(0)) || read_u8(s(6)) != 0 {
                return 0;
            }
            respond_button(label, handler, 0x8f66d0)
        }
        0xbc2 => {
            if !own_club_is(slot(0)) || read_u8(s(7)) != 0 {
                return 0;
            }
            set_action_button(label, handler, 0x9aff10, 0x4dc9b0, 1) // "Offer Contract"
        }
        0xbb8 => {
            let (person, club, offer, serial, state) = (slot(0), slot(4), slot(8) as i32, slot(9), read_u8(s(10)));
            if !own_club_is(club) {
                return 0;
            }
            if offer != -1 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer)));
                if o.is_null() || o.serial() as u32 != serial {
                    return 0;
                }
            } else if state == 5 {
                return 0;
            }
            let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), person, club);
            if k == 0 || read_u8(k + contract_offer::STATE) != 3 { // nma8: the player's contract talks need a reply
                return 0;
            }
            set_action_button(label, handler, 0xa6943c, 0x4e5770, 2) // "Negotiate"
        }
        _ => 0,
    }
}

/// The discipline news (2000s): types 0x7d2 and 0x7d3, the ban that can be appealed (slot 0 the appeal made, 4 and 5
/// day and year of the ban, 10 the kind, 12 the appeal pending, 15 and 16 day and year of the hearing, 23 a match).
/// The person (`read_staff_name_ref_from_news` from slot 17) must be a valid id. With a match (slot 23), the match
/// record from slot 24 must have a valid club and competition (`G_CLUB_COMP_TABLE`), the club must not be a national
/// team, the competition must allow appeals (+0x5d) and the person must be at the manager's club. The appeal needs
/// the club's nation not to be Argentina (`club_nation_is_not_argentina`) and a kind other than 0, 1 and 4, no appeal
/// made or pending, and the hearing today or the ban still to start (its year above today's, or the same year and a
/// later day): "Appeal", 0x55e620, 1. Otherwise 0.
unsafe fn discipline_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 discipline button");
    let kind = read_i32(news);
    if kind != 0x7d3 && kind != 0x7d2 {
        return 0;
    }
    let s = |k: usize| action_slot(news, k);
    let appealed = read_u8(s(0));
    let (ban_day, ban_year) = (read_u16(s(4)), read_u16(s(5)));
    let (hearing_day, hearing_year) = (read_u16(s(15)), read_u16(s(16)));
    let ban_kind = read_u32(s(10));
    let pending = read_u32(s(12));
    let read_name: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_READ_STAFF_NAME_REF_FROM_NEWS);
    let read_match: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_READ_MATCH_RECORD_FROM_NEWS);
    let mut next = 0x11u8;
    let mut person = [0u8; 0x18];
    read_name(news, &mut next, person.as_mut_ptr());
    let id = read_i32(person.as_ptr() as usize + 0xc);
    if id < 0 || id > read_i32(G_STAFF_COUNT) - 1 {
        return 0;
    }
    if read_u32(s(23)) != 0 {
        next = 0x18;
        let mut game = [0u8; 0x20];
        read_match(news, &mut next, game.as_mut_ptr());
        let club_index = read_i32(game.as_ptr() as usize + 0x11);
        if club_index < 0 || club_index > read_i32(G_CLUB_COUNT) - 1 {
            return 0;
        }
        let comp_index = read_i32(game.as_ptr() as usize + 8);
        if comp_index < 0 || comp_index > read_i32(G_CLUB_COMP_COUNT) - 1 {
            return 0;
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_index as usize).wrapping_mul(0x6b));
        if exe::database_is_national_team(club_at(club_index as isize)) != 0 || read_u32(comp + 0x5d) == 0
            || Staff(manager).club_job() != staff_at(id as isize).club_job() {
            return 0;
        }
    }
    let mut can = club_nation_is_not_argentina(Staff(manager).club_job().addr()) as u8;
    if ban_kind == 4 || ban_kind == 0 || ban_kind == 1 {
        can = 0;
    }
    if appealed != 0 || can == 0 || pending != 0 {
        return 0;
    }
    let (year, day) = (read_u16(G_CURRENT_DATE_YEAR), read_u16(G_CURRENT_DATE));
    let hearing_today = hearing_year == year && hearing_day == day;
    if !hearing_today && !(ban_year > year || (ban_year == year && (ban_day as i16) > (day as i16))) {
        return 0;
    }
    set_action_button(label, handler, 0xa69434, 0x55e620, 1) // "Appeal"
}

/// The transfer news (4000s), on the offers of `G_TRANSFER_MANAGER` (slot 6, 7 or 11 the offer index; the offer must
/// have the serial (`offer::SERIAL`) the news keeps).
///
/// - 0xfa0, a bid for one of his players: the club of slot 5 managed by him, the serial of slot 7, the kind (slot 9,
///   byte) not 1, 2, 5 or 0x11 and the offer in state 0: "Respond", 0x8f1730, 2.
/// - 0xfbc: the club of slot 4 (above −1) managed by him and slot 7 (byte) 0: "Respond", 0x741f50, 2.
/// - 0xfa2 and 0xfc0, a negotiation (slot 9 the negotiation's serial): the offer of slot 6 with the serial of slot 7,
///   a negotiation (`offer::NEGOTIATION_INDEX`, `get_negotiation_slot`) with the serial of slot 9 and the offer in
///   state 1: "Respond", 0x8f1830 (0xfa2) or 0x8f93b0 (0xfc0), 2.
/// - 0xfac, personal terms (slot 0 the person, 7 the offer, 8 its serial, 9 the kind): kind 1 or 4 gives 0; the
///   offer open (`exe::transfer_offer_is_closed` 0) with the serial and not in state 1. Bought by his club
///   (`offer::BUYER_CLUB_ID`): with no terms yet (`offer::TERMS_STATE` 0) or the offer in state 5 or 6, "Offer
///   Contract", 0x8f1950, 2. Otherwise, when the target's contract (`exe::contract_manager_get_contract`, kept in
///   `G_CONTRACT_LOOKUP_RESULT`) is at his club (or both are none), a player of the exchange list
///   (`offer::EXCHANGE_PLAYERS`) with the id of slot 0 and +4 = 0: the same button.
/// - 0xfa4 (slot 6 the step, 7 the offer or −1, 8 its serial, 9 a flag): step 3, an offer with the serial in state 7
///   or 8 and the contract offer of the person (slot 0) at the club of slot 5 in state 3: "Negotiate", 0x4e5770, 2.
///   Step 4 with the flag 0, an offer with the serial, his club job with the id of slot 5, the offer in state 0xf and
///   `offer::RESPONSE_ROUND` below 3: "Negotiate", 0x4e5770, 1. When no button and an offer index other than −1:
///   slot 7 = −1 (`set_item_slot`) and 0.
/// - 0xfa5 (slot 6 the offer, 7 its serial, 9 the kind; kinds 2, 3, 0xc, 0xd, 0xe, 0x10, 0x11, 0x12 give 0): bought by
///   his club and the offer in state 0xb or 0xc; or the target's contract at his club, no pre-contract and the offer
///   in state 0xb or 0xd: "Confirm", 0x8f2d20, 2.
/// - 0xfab: his club job with the id of slot 0 or 1 and slot 7 (byte) 0: "Respond", 0x8f1a60, 2.
/// - 0xfb4, a contract renewal for a bid (slot 0 the person, 5 his club id, 10 the wish, 11 the offer, 12 its serial,
///   13 a flag): the offer's buyer is his club job, the person has a contract at a club with the id of slot 5, the
///   serial, no pre-contract, state 1, the flag 0 and the wish 1: "Re-Negotiate", 0x8f1b40, 2.
/// - 0xfb5: as 0xfa0 without the kind: "Respond", 0x8f1c30, 2.
/// - 0xfc1: "Details", 0x77b700, 1. 0xfa8, 0xfb6 and the rest: 0.
unsafe fn transfer_button(news: usize, manager: usize, label: usize, handler: *mut u32) -> u8 {
    covered!("nma_8 transfer button");
    let s = |k: usize| action_slot(news, k);
    let slot = |k: usize| read_u32(s(k));
    let tm = read_usize(G_TRANSFER_MANAGER);
    let offer_slot = |k: usize| exe::transfer_manager_get_offer_slot(tm, slot(k) as i32);
    let managed_by_me = |i: i32| club_at(i as isize).manager().addr() == manager;
    let own = Staff(manager).club_job().addr();
    // the club of a person's contract (`exe::contract_manager_get_contract`, kept in G_CONTRACT_LOOKUP_RESULT), or 0
    let contract_club = |person: u32| {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff_at(person as i32 as isize));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() }
    };
    let negotiated = |k: usize| {
        let os = offer_slot(6);
        let o = Offer(read_usize(os));
        if o.is_null() || o.serial() as u32 != slot(7) || o.negotiation_index() as i32 == -1 {
            return false;
        }
        let n = read_usize(get_negotiation_slot(tm, o.negotiation_index() as i32));
        slot(k) == read_u32(n + offer::SERIAL) && Offer(read_usize(os)).status() == 1
    };
    match read_i32(news) {
        0xfa0 => {
            let o = Offer(read_usize(offer_slot(6)));
            let (club, kind) = (slot(5) as i32, read_u8(s(9)));
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && kind != 1 && kind != 5 && kind != 2 && kind != 0x11
                && o.status() == 0 { // nma8: a bid for his player needs a reply
                return respond_button(label, handler, 0x8f1730);
            }
            0
        }
        0xfbc => {
            let club = slot(4) as i32;
            if club > -1 && managed_by_me(club) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x741f50);
            }
            0
        }
        0xfa2 => if negotiated(9) { respond_button(label, handler, 0x8f1830) } else { 0 },
        0xfc0 => if negotiated(9) { respond_button(label, handler, 0x8f93b0) } else { 0 },
        0xfac => {
            let person = slot(0);
            let kind = read_u8(s(9));
            let os = offer_slot(7);
            if kind == 1 || kind == 4 {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(8) || exe::transfer_offer_is_closed(o) != 0 || o.status() == 1 {
                return 0;
            }
            if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                if o.terms_state() == 0 || o.status() == 5 || o.status() == 6 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
                return 0;
            }
            if contract_club(o.target_staff_id() as u32) != own {
                return 0;
            }
            let list = Llist(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
            if list.is_null() {
                return 0;
            }
            list.set_cursor(list.head());
            loop {
                let p = exe::llist_get_next(read_usize(read_usize(os) + offer::EXCHANGE_PLAYERS));
                if p == 0 {
                    return 0;
                }
                if read_u32(p) == person && read_u8(p + 4) == 0 {
                    return set_action_button(label, handler, 0x9aff10, 0x8f1950, 2); // "Offer Contract"
                }
            }
        }
        0xfa4 => {
            let step = read_u8(s(6));
            let index = slot(7) as i32;
            let flag = read_u8(s(9));
            if (step == 3 || step == 4) && index == -1 {
                return 0;
            }
            if step == 3 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && (o.status() == 7 || o.status() == 8) {
                    let k = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), slot(0), slot(5));
                    if k != 0 && read_u8(k + contract_offer::STATE) == 3 {
                        return set_action_button(label, handler, 0xa6943c, 0x4e5770, 2); // "Negotiate"
                    }
                }
            } else if step == 4 && flag == 0 {
                let o = Offer(read_usize(exe::transfer_manager_get_offer_slot(tm, index)));
                if !o.is_null() && o.serial() as u32 == slot(8) && own != 0 && read_u32(own) == slot(5) && o.status() == 0xf && o.response_round() < 3 {
                    return set_action_button(label, handler, 0xa6943c, 0x4e5770, 1); // "Negotiate"
                }
            }
            if index != -1 {
                set_item_slot(read_usize(G_NEWS), manager, read_u32(news + news_item::ID), 7, -1i32 as u32, news);
            }
            0
        }
        0xfa5 => {
            let kind = read_u8(s(9));
            if [0xe, 0xd, 0xc, 3, 2, 0x10, 0x12, 0x11].contains(&kind) {
                return 0;
            }
            let os = offer_slot(6);
            let o = Offer(read_usize(os));
            if o.is_null() || o.serial() as u32 != slot(7) {
                return 0;
            }
            let confirm = if own != 0 && read_u32(own) as usize == o.buyer_club_id() {
                o.status() == 0xb || o.status() == 0xc
            } else {
                let o2 = Offer(read_usize(os));
                contract_club(o.target_staff_id() as u32) == own && o2.pre_contract() == 0 && (o2.status() == 0xb || o2.status() == 0xd)
            };
            if confirm {
                return set_action_button(label, handler, 0x98a198, 0x8f2d20, 2); // "Confirm"
            }
            0
        }
        0xfab => {
            if own != 0 && (read_u32(own) == slot(0) || read_u32(own) == slot(1)) && read_u8(s(7)) == 0 {
                return respond_button(label, handler, 0x8f1a60);
            }
            0
        }
        0xfb4 => {
            let (wish, serial, flag, person, club_id) = (read_u8(s(10)), slot(12), read_u8(s(13)), slot(0), slot(5));
            let os = exe::transfer_manager_get_offer_slot(tm, slot(11) as i32);
            if os == 0 || read_usize(os) == 0 {
                return 0;
            }
            if club_at(Offer(read_usize(os)).buyer_club_id() as i32 as isize).addr() != own {
                return 0;
            }
            if contract_club(person) == 0 {
                return 0;
            }
            if read_u32(contract_club(person)) != club_id {
                return 0;
            }
            let o = Offer(read_usize(os));
            if o.serial() as u32 == serial && o.pre_contract() == 0 && o.status() == 1 && flag == 0 && wish == 1 {
                return set_action_button(label, handler, 0xa69424, 0x8f1b40, 2); // "Re-Negotiate"
            }
            0
        }
        0xfb5 => {
            let o = Offer(read_usize(offer_slot(6)));
            let club = slot(5) as i32;
            if !o.is_null() && club != -1 && managed_by_me(club) && o.serial() as u32 == slot(7) && o.status() == 0 {
                return respond_button(label, handler, 0x8f1c30);
            }
            0
        }
        0xfc1 => set_action_button(label, handler, 0xa6941c, 0x77b700, 1), // "Details"
        _ => 0,
    }
}

/// Writes the button label and the screen handler of a news item for a manager and says whether it needs a reply.
///
/// `NewsItem::get_action_button(manager, label, size, &handler)`: no manager (box of line 0x123b), no label or a size
/// below 0x33 (line 0x1243) or no handler (line 0x124e, after the label is emptied): the "Error" box and 0. Otherwise
/// the label is emptied, the handler set to 0 and the item goes to the piece of its category (`get_category`, asked
/// again before each test, as the original does): 6 the board (`board_button`), 2 the general news
/// (`general_button`), 4 the player news (`player_button`), 3 the discipline (`discipline_button`), 5 the transfers
/// (`transfer_button`) and 1, type 0x1388 only, "Reply" with handler 0x789860 and 1. A board item that gets no button
/// falls through the other tests to 0. Returns 2 when the item needs a reply (the automatic reply
/// `NewsItem::auto_respond` 0x784100, `mark_item_read` and `has_urgent_items` look for it), 1 for a button without a
/// needed reply, 0 for none. Some items are also closed on the way: the national squad (0x22) not of today and the
/// cup draw (0x27) with every fixture made get their answer slot set, and the transfer step (0xfa4) without a button
/// gets its offer slot set to −1 (`set_item_slot`).
/// Original: `NewsItem::get_action_button` (0x782b70, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    if manager == 0 {
        show_error_at(0xa693ec, 0x123b);
        return 0;
    }
    if label == 0 || (size as i32) < 0x33 {
        show_error_at(0xa693ec, 0x1243);
        return 0;
    }
    write_u8(label, 0);
    if handler.is_null() {
        show_error_at(0xa693ec, 0x124e);
        return 0;
    }
    *handler = 0;
    let category = || get_category(news) as u8;
    if category() == 6 {
        if let Some(r) = board_button(news, label, handler) {
            return r;
        }
    }
    if category() == 2 {
        return general_button(news, manager, label, handler);
    }
    if category() == 4 {
        return player_button(news, manager, label, handler);
    }
    if category() == 3 {
        return discipline_button(news, manager, label, handler);
    }
    if category() == 5 {
        return transfer_button(news, manager, label, handler);
    }
    if category() == 1 && read_i32(news) == 0x1388 {
        covered!("nma_8 reply button");
        return set_action_button(label, handler, 0xa69414, 0x789860, 1); // "Reply"
    }
    0
}

/// Comparer for `get_action_button`: the original and the copy side by side (the label, the handler, the contract
/// lookup global, the item and the news storage that `set_item_slot` writes); a divergence names the piece (the
/// item's category). The error boxes (no manager, label or handler, a type outside the categories) run the original
/// only.
pub(crate) unsafe extern "thiscall" fn compare_get_action_button(news: usize, manager: usize, label: usize, size: u32, handler: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8>(%%I0%%);
    let kind = read_i32(news);
    if manager == 0 || label == 0 || (size as i32) < 0x33 || handler.is_null() || !(1..=0x1790).contains(&kind) {
        return o(news, manager, label, size, handler);
    }
    let store = read_usize(G_NEWS);
    let mut regions = vec![
        (label, (size as usize).min(0x7d0)),
        (handler as usize, 4usize),
        (G_CONTRACT_LOOKUP_RESULT, 4),
        (news, NEWS_NEWS_ITEM_SIZE),
    ];
    if store != 0 && read_usize(store) != 0 {
        regions.push((read_usize(store), read_i32(store + news_fields::SLOT_COUNT).max(0) as usize * NEWS_NEWS_ITEM_SIZE));
    }
    let (r, n, same, _, _) = crate::comparison::side_by_side(&regions,
        || o(news, manager, label, size, handler),
        || get_action_button(news, manager, label, size, handler));
    SCOREBOARD.record(same && r == n, || format!("get_action_button piece {} (type {:#x}) original={} rust={} handler {:#x} label {:?} memory equal {}",
        action_button_piece(kind), kind, r, n, *handler, (0..0x40).map(|k| read_u8(label + k)).take_while(|&b| b != 0).map(|b| b as char).collect::<String>(), same));
    r
}
