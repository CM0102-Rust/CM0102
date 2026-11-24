// ---------- batch %%LEVA%%: the requests to the FA (manager_manager.cpp) ----------

/// `Media::queue_manager_fa_complaint_story` (0x758de0; thiscall(media, club, person, kind, a, b, c), ret 0x18).
const FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY: usize = 0x758de0;
/// `Finance::pay_league_fine` (0x599850; thiscall(finance, amount), ret 4).
const FN_FINANCE_PAY_LEAGUE_FINE: usize = 0x599850;
/// `OfficialsManager::get_average_rating` (0x79d3e0; thiscall(om, official), ret 4; the float in st0).
const FN_OFFICIALSMANAGER_GET_AVERAGE_RATING: usize = 0x79d3e0;
/// `OfficialsManager::get_last_match_rating` (0x79d3b0; thiscall(om, official), ret 4; the char in al).
const FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING: usize = 0x79d3b0;
/// `OfficialsManager::reset_official_stats` (0x79d190; thiscall(om, official), ret 4).
const FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS: usize = 0x79d190;
/// `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90; thiscall(this, club, request, outcome, standing), ret 0x10).
const FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS: usize = 0x6aaa90;
/// `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0; thiscall(this, fixture, club id, count), ret 0xc).
const FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS: usize = 0x6aa7d0;
/// The two 0x625-byte match team blocks the routine builds and destroys without using them.
const FA_REQUEST_TEAM_BLOCK_CONSTRUCT: usize = 0x453c30;
const FA_REQUEST_TEAM_BLOCK_DESTRUCT: usize = 0x453c10;
/// Size of an official record in `G_OFFICIAL_TABLE`.
const FA_REQUEST_OFFICIAL_SIZE: usize = 0x2b;

/// The FA's reply news to `club` (through the exe address, so the hook and its comparer run).
unsafe fn fa_request_reply(this: usize, club: Club, request: i8, outcome: u32, standing: u32) {
    let post: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS);
    post(this, club.addr(), request as i32 as u32, outcome, standing);
}

/// The board's count of the club's FA requests (board record +0, unnamed; signed byte).
unsafe fn fa_request_count(row: BoardRecord) -> i8 {
    read_i8(row.addr())
}

/// The club's sanction after one complaint too many: the media story of the outburst (kind 5, random(3) + 1), a
/// 25000 league fine, the board confidence (in `G_MANAGER_MANAGER`'s table) moved by 250 (or, with the club state +4
/// below 0, by limit × 250 − the count), kept within 1..10000, and the sacking check.
unsafe fn fa_request_sanction(this: usize, club: Club, limit: i8) {
    covered!("sx14b_1 FA sanction");
    let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
    let r = exe::utils_random(3);
    story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 5, u32::MAX, u32::MAX, (r as u8).wrapping_add(1) as u32);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let fine: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_LEAGUE_FINE);
        fine(finance, 25000);
    }
    let manager_manager = read_usize(G_MANAGER_MANAGER);
    let row = BoardRecord(board_row(manager_manager, club));
    let delta = if read_i8(club_state_record(club.addr()) + 4) < 0 {
        // the count from this object's table, the confidence in G_MANAGER_MANAGER's (the same object in the game)
        (limit as i16).wrapping_mul(250).wrapping_sub(fa_request_count(BoardRecord(board_row(this, club))) as i16)
    } else {
        250
    };
    row.set_confidence(row.confidence().wrapping_add(delta).clamp(1, 10000));
    exe::manager_manager_consider_sacking_manager(manager_manager, club, 0);
}

/// A postponement request: the first fixture in the next 8 weeks that is not today and not played. A fixture of another
/// competition than the club's league, or the key nation's season ending within 60 days of today (or, when it ends in
/// the first 60 days of the year, today past day 340): outcome 6. Granted with more than 2 squad players on
/// international duty that day, or with 2 of them or more than 4 happy (happiness above 20) and injured players when
/// the club's reputation beats 4750 + random(1250): the fixture is moved (removed, a new date, added again) with the
/// reply (outcome 0) and the rescheduling news; otherwise outcome 1.
unsafe fn fa_postponement_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    let key = exe::key_nation_index_of_club(club) as i8;
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    // simplification: the high halves of the date words pushed to find_fixtures are register garbage in the original
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), today as u16 as u32, this_year as u32, day as u32, year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let fixture = loop {
        let f = Fixture(exe::llist_get_next(list));
        if f.is_null() {
            return;
        }
        let is_today = f.day() as i32 == today as i32 && f.year() == this_year;
        if !is_today && f.home_goals() == -1 {
            break f;
        }
    };
    covered!("sx14b_1 postponement");
    let date_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let (mut made, mut out) = ([0u8; 8], [0u8; 8]);
    let d = date_new(made.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    exe::date_operator_3d(date, out.as_mut_ptr() as usize, d);
    let mut on_duty = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p != 0 && exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, date, club, NEWS_TEXT_BUFFER as u32, 0) != 0 {
            on_duty = on_duty.wrapping_add(1);
        }
    }
    let mut injured = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p == 0 {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        if !c.is_null() && c.happiness() > 20 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            injured = injured.wrapping_add(1);
        }
    }
    if fixture.competition().addr() != club.division().addr() {
        return fa_request_reply(this, club, request, 6, 0);
    }
    if (0..0x23).contains(&key) {
        let e = KeyNationRecord(G_LEAGUE_TABLE + key as usize * 0x48);
        let end = e.end_date();
        if e.end_date_year() as u16 == this_year && if end > 0x3c { (end as i32 - 0x3c) < today as i32 } else { today > 0x154 } {
            covered!("sx14b_1 postponement too late");
            return fa_request_reply(this, club, request, 6, 0);
        }
    }
    let on_duty = on_duty as i8;
    let granted = on_duty > 2 || ((on_duty > 1 || injured as i8 > 4) && club.reputation() > (exe::utils_random(0x4e2) + 0x128e) as i16);
    if !granted {
        return fa_request_reply(this, club, request, 1, 0);
    }
    covered!("sx14b_1 postponement granted");
    let mut copy = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(fixture.addr() as *const u8, copy.as_mut_ptr(), fixture::SIZE);
    let fm = read_usize(G_FIXTURE_MANAGER);
    exe::fixture_manager_remove_fixture(fm, fixture);
    fa_request_reply(this, club, request, 0, 0);
    exe::fixture_manager_find_new_fixture_date(fm, copy.as_mut_ptr() as usize);
    exe::fixture_manager_add_fixture(fm, copy.as_mut_ptr() as usize, 1);
    // defect of the original: the news reads the fixture that remove_fixture just took out of the calendar
    let news: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS);
    news(this, fixture.addr(), read_u32(club.addr()), on_duty as u8 as u32);
}

/// A referee complaint: the last played fixture of the past 8 weeks; its referee with an average rating below 5.5 and
/// a last match rating below 7 is replaced in 1 of 200 cases (a new name from his nation, discipline random(16) + 5 or
/// random(11) + random(11), born in a random day of (this year − 27 + random(5)), his stats reset, and the news 0x2b to
/// the managers of the club's nation); otherwise the reply (outcome 1) with his last match rating.
unsafe fn fa_referee_complaint(this: usize, club: Club, request: i8, list: usize) {
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_u16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day as u32, year as u32, today as u32, this_year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 6)); // the cursor at the tail
    let fixture = loop {
        let f = Fixture(exe::llist_get_prev(list));
        if f.is_null() {
            return;
        }
        if f.home_goals() > -1 {
            break f;
        }
    };
    covered!("sx14b_1 referee complaint");
    let official = Official(read_usize(G_OFFICIAL_TABLE).wrapping_add((fixture.referee() as i32 as usize).wrapping_mul(FA_REQUEST_OFFICIAL_SIZE)));
    let om = read_usize(G_OFFICIALS_MANAGER);
    let average: unsafe extern "thiscall" fn(usize, usize) -> f32 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_AVERAGE_RATING);
    let last: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING);
    if !((average(om, official.addr()) as f64) < read_f64(rdata::F64_5_5) && (last(om, official.addr()) as i8) < 7) || exe::utils_random(200) != 0 {
        return fa_request_reply(this, club, request, 1, last(om, official.addr()) as u32);
    }
    covered!("sx14b_1 referee replaced");
    let old = read_u32(official.second_name() + names::ID);
    official.set_first_name(exe::database_pick_first_name_for_nation(official.nation()));
    official.set_second_name(exe::database_pick_second_name_for_nation(official.nation()));
    let discipline = if exe::utils_random(10) != 0 {
        (exe::utils_random(0x10) as u8).wrapping_add(5)
    } else {
        let r = exe::utils_random(0xb) as u8;
        r.wrapping_add(exe::utils_random(0xb) as u8)
    };
    official.set_discipline(discipline);
    let r = exe::utils_random(5) as i16;
    official.set_birth_year(r.wrapping_add((this_year as i16).wrapping_sub(0x1b)));
    official.set_birth_day(exe::utils_random(0x154) as i16);
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS);
    reset(om, official.addr());
    let new = read_u32(official.second_name() + names::ID);
    let (nation, manager) = (club.nation(), club.manager());
    if nation.is_null() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x2b, 0);
    exe::newsitem_set_slot(n, 0, read_u32(nation.addr()));
    exe::newsitem_set_slot(n, 1, old);
    exe::newsitem_set_slot(n, 2, new);
    exe::newsitem_set_slot(n, 3, read_u32(manager.addr()));
    exe::news_send_to_nation_managers(read_usize(G_NEWS), n, nation);
}

/// The request itself (inside the frame of `handle_fa_request`).
unsafe fn fa_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    if club.is_null() || club.division().is_null() || club.manager().is_null() || club.manager().non_player().is_null() {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let row = BoardRecord(board_row(this, club));
    write_i8(row.addr(), fa_request_count(row).wrapping_add(1));
    // defect of the original: the bands are tested from the lowest, so above 3250 the limit is always 3 (the 4 above
    // 5250 and the 7 above 7750 never apply)
    let limit: i8 = if club.reputation() > 0xcb2 { 3 } else { 2 };
    if request == 1 {
        let count = fa_request_count(row);
        if count == limit && read_i8(club_state_record(club.addr()) + 4) < -1 {
            covered!("sx14b_1 FA complaint story");
            let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
            return story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 4, u32::MAX, u32::MAX, u32::MAX);
        }
        if count > limit && (manager_manager_is_human_club(club.addr(), 1, 0) != 0 || count as i32 == limit as i32 + 1) {
            return fa_request_sanction(this, club, limit);
        }
    }
    let count = fa_request_count(row);
    if request != 5 && count == limit {
        covered!("sx14b_1 request at the limit");
        return fa_request_reply(this, club, request, 3, 0);
    }
    if request != 5 && count > limit {
        covered!("sx14b_1 request over the limit");
        fa_request_reply(this, club, request, 5, 0);
        let row = BoardRecord(board_row(this, club));
        return row.set_confidence(row.confidence().wrapping_sub(100));
    }
    if manager_manager_is_human_club(club.addr(), 1, 0) == 0 {
        return;
    }
    match request {
        1 => fa_referee_complaint(this, club, request, list),
        2 => fa_postponement_request(this, club, request, list, date),
        _ => {}
    }
}

/// Handles a manager's request to the FA: 1 a referee complaint, 2 a postponement.
///
/// `MANAGER_MANAGER::handle_fa_request(club, request)`, called after a match for the machine's clubs (request 1,
/// `update_board_confidence_after_match`) and from the FA request screen. SEH frame for the locals: two unused match
/// team blocks (`exe::crt_vector_constructor_iterator`, 0x625 bytes each), a fixture list (`exe::new_llist`) and a
/// date (`exe::date_date_54a110`), destroyed at the end. A club without a league (`.division()`), a manager or a
/// non-player record, or a national team: nothing. The club's request count (board record +0) goes up; the limit is
/// 3 above reputation 3250, else 2. A referee complaint at the limit with the club state +4 below −1: the media story
/// of the complaint (kind 4). Over the limit, for a human club or at the limit + 1: the sanction (the story of the
/// outburst, the 25000 fine, the board confidence and the sacking check). Otherwise (other requests, but 5) at the
/// limit the reply with outcome 3; over it, outcome 5 and the board confidence − 100. Below the limit, only a human
/// club's request goes on: the referee complaint (`fa_referee_complaint`) or the postponement
/// (`fa_postponement_request`).
///
/// Defects of the original: the reputation bands are tested from the lowest, so the limits 4 (above 5250) and 7
/// (above 7750) never apply; the rescheduling news reads the fixture after `remove_fixture` took it out of the
/// calendar.
/// Original: `MANAGER_MANAGER::handle_fa_request` (0x6a9e60, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn handle_fa_request(this: usize, club: usize, request: u32) {
    let mut blocks = [0u8; 2 * 0x625];
    let blocks = blocks.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_CONSTRUCT, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
    let mut list = [0u8; 0x20];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let mut date = [0u8; 8];
    let date = date.as_mut_ptr() as usize;
    exe::date_date_54a110(date);
    fa_request(this, Club(club), request as i8, list, date);
    exe::drop_llist(list);
    exe::crt_vector_destructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
}

/// Comparer for `handle_fa_request`: it moves fixtures, renames referees, fines, posts news and draws, so only the
/// original runs (its effects are checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_handle_fa_request(this: usize, club: usize, request: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, club, request)
}

/// Posts the 0x1789 news of the FA's reply to a club manager's request.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_fa_request_reply_news(club, request, outcome, standing)`: no
/// club or a club without a nation (`.nation()`): nothing. News 0x1789 (local, `exe::new_newsitem`): 0 the club's id,
/// 1 the request, 2 the outcome, 3 the nation's id, 4 the referee's standing (the three as signed bytes); to the club's
/// manager (`exe::news_send_to_club_manager`, which only delivers to a human).
/// Original: `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_fa_request_reply_news(_this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    let club = Club(club);
    if club.is_null() || club.nation().is_null() {
        return;
    }
    covered!("sx14b_1 FA reply news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1789, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, request as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, outcome as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, standing as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_fa_request_reply_news`: the news goes to the manager's inbox (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_fa_request_reply_news(this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I1%%)(this, club, request, outcome, standing)
}

/// Posts the 0x29 news that the FA moved a fixture, to the managers of both clubs.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_match_rescheduled_news(fixture, club id, count)`: no fixture,
/// or before the news start date (`before_news_start_date`): nothing. News 0x29 (local, `exe::new_newsitem`): 0 and 1
/// the home and away clubs' ids, 2 the competition's id, 3 the requesting club's id, 4 the id of the competition's
/// nation, 5 the count (signed byte); to the home club's manager and to the away club's
/// (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_match_rescheduled_news(_this: usize, fixture: usize, club: u32, count: u32) {
    let fixture = Fixture(fixture);
    if fixture.is_null() || before_news_start_date() {
        return;
    }
    covered!("sx14b_1 rescheduled news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x29, 0);
    exe::newsitem_set_slot(n, 0, read_u32(fixture.home_club().addr()));
    exe::newsitem_set_slot(n, 1, read_u32(fixture.away_club().addr()));
    exe::newsitem_set_slot(n, 2, read_u32(fixture.competition().addr()));
    exe::newsitem_set_slot(n, 3, club);
    exe::newsitem_set_slot(n, 4, read_u32(fixture.competition().nation().addr()));
    exe::newsitem_set_slot(n, 5, count as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.home_club());
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.away_club());
}

/// Comparer for `post_match_rescheduled_news`: the news goes to the managers' inboxes (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_match_rescheduled_news(this: usize, fixture: usize, club: u32, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%)(this, fixture, club, count)
}
