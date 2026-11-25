// --- rx06_3: the scout's report on one player (player_search.cpp): PLAYER_SEARCH::report_on_player (0x7b62a0); uses
// the scout list helpers of rx06_2 and the result list helpers of sx17_2 and sx17_5 ---

/// `SCOUT_MANAGER::record_scout_report` (0x7f9520, scout_manager.cpp, thiscall, ret 8): one report more on the
/// request of the pair (player, club).
const FN_SCOUT_MANAGER_RECORD_SCOUT_REPORT: usize = 0x7f9520;
/// The role code `match_man_score_player_for_role` rates a reported player's current worth with (the list uses 0x1e82).
const ROLE_REPORTED_CURRENT_WORTH: u32 = 0x1e80;
/// The scout report record's slot byte (the search's slot, +4) [inference].
const SCOUT_REPORT_SLOT: usize = 0x4f;
/// The order by person (+0xc) the result array is kept in, passed to bsearch by address.
const ORDER_BY_PERSON: usize = 0x7bce50;
/// The size of the match data `fill_match_record_from_fixture` writes (the report record's +0..+0x1f).
const MATCH_DATA_SIZE: usize = 0x20;

/// What a report does to the world, for the game or the comparer's dry run: the news and the fog of war through the
/// scout day's run, plus the stored fixtures whose category it sets and the transfer interests it adds to the media.
pub(crate) struct ScoutReportRun {
    pub(crate) day: ScoutDayRun,
    /// The stored fixtures whose category was set to 2, with the byte before.
    pub(crate) categories: Vec<(usize, u8)>,
    /// The transfer interests the dry run would add: player, club, price and interest.
    pub(crate) interests: Vec<(Staff, Club, i32, f64)>,
}

impl ScoutReportRun {
    pub(crate) fn new(dry: bool) -> Self {
        ScoutReportRun { day: ScoutDayRun { dry, news: Vec::new(), knowledge: Vec::new(), reassigned: false }, categories: Vec::new(), interests: Vec::new() }
    }

    /// The stored fixture of a reserve match gets category 2 (`fixture::CATEGORY`).
    unsafe fn set_league_category(&mut self, stored: usize) {
        self.categories.push((stored, read_u8(stored + fixture::CATEGORY)));
        write_u8(stored + fixture::CATEGORY, 2);
    }

    /// The media hears of the club's interest (`exe::media_add_transfer_interest` on `G_MEDIA`); dry: kept.
    unsafe fn add_transfer_interest(&mut self, player: Staff, club: Club, price: i32, interest: f64) {
        if self.day.dry {
            self.interests.push((player, club, price, interest));
            return;
        }
        exe::media_add_transfer_interest(read_usize(G_MEDIA), player, club, price, interest);
    }
}

/// The player's interest in joining `club` as the scout weighs it (`Shortlist_Manager::staff_interest_in_joining_club`
/// with 1, 0, 1, 0).
unsafe fn scout_report_interest(player: Staff, club: Club) -> f64 {
    exe::shortlist_manager_staff_interest_in_joining_club(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), player, club, 1, 0, 1, 0)
}

/// The club of the watched match's side: the second club id of the result (+4) when `side` is set, else the first.
unsafe fn scout_report_side_club(result: usize, side: u8) -> Club {
    club_at(read_i32(result + if side != 0 { 4 } else { 0 }) as isize)
}

/// Whether the player is eligible through a second nationality: his nation is not of membership group 2, his second
/// nation is, and the scout club's nation is in region 14.
unsafe fn scout_report_second_nationality(player: Staff, scout: Staff) -> bool {
    let second = player.second_nation();
    player.nation().group_membership() != 2 && !second.is_null() && second.group_membership() == 2
        && scout.club_job().nation().actual_region() == 0xe
}

/// The two worths of a reported player: now, `bias` − random(500) + the role score `ROLE_REPORTED_CURRENT_WORTH` +
/// random(500), corrected (`scout_current_discounts`); under 25 the future one the same way with `ROLE_POTENTIAL_WORTH`,
/// with the low current correction (`scout_potential_discounts`); 0 from 25 on.
unsafe fn scout_report_worths(player: Staff, scout: Staff, bias: i32) -> (i32, i32) {
    let (speed, physique) = scout_speed_and_physique(player.player());
    let age = read_i8(player.addr() + staff::AGE);
    let judged = |role: u32| {
        let worth = bias.wrapping_sub(exe::utils_random(500)).wrapping_add(scout_role_score(player, scout, role));
        worth.wrapping_add(exe::utils_random(500))
    };
    let current = scout_current_discounts(judged(ROLE_REPORTED_CURRENT_WORTH), age, physique, speed);
    if age >= 25 {
        return (current, 0);
    }
    (current, scout_potential_discounts(judged(ROLE_POTENTIAL_WORTH), current, age, physique, speed, true))
}

/// The player's result is brought up to date, or added. Found (bsearch by person): today as the last update, the
/// worths, the grades and the note rating (the flag byte is or-ed with the new result's 0); answers whether it already
/// had a grade 5. Not found, with a grade above 0 and an assignment by nation, competition, youth or region: appended
/// (`player_search_append_result`, line 0x1131) and the array sorted by person.
unsafe fn scout_report_update_result(this: usize, entry: usize) -> bool {
    let count = read_u32(this + player_search_fields::RESULT_COUNT);
    let found = if count != 0 {
        exe::crt_bsearch(entry, read_usize(this + player_search_fields::RESULTS), count, SCOUT_DAY_RESULT_SIZE as u32, ORDER_BY_PERSON)
    } else {
        0
    };
    if found != 0 {
        let top = read_u8(found + RESULT_CURRENT_GRADE) == 5 || read_u8(found + RESULT_POTENTIAL_GRADE) == 5;
        exe::date_operator_3d(found + search_result::LAST_UPDATE_DATE, [0u32; 2].as_mut_ptr() as usize, G_CURRENT_DATE);
        write_u32(found + RESULT_CURRENT_WORTH, read_u32(entry + RESULT_CURRENT_WORTH));
        write_u32(found + RESULT_POTENTIAL_WORTH, read_u32(entry + RESULT_POTENTIAL_WORTH));
        write_u8(found + RESULT_CURRENT_GRADE, read_u8(entry + RESULT_CURRENT_GRADE));
        write_u8(found + RESULT_POTENTIAL_GRADE, read_u8(entry + RESULT_POTENTIAL_GRADE));
        write_u8(found + RESULT_NOTE_RATING, read_u8(entry + RESULT_NOTE_RATING));
        return top;
    }
    if read_u8(entry + RESULT_CURRENT_GRADE) == 0 && read_u8(entry + RESULT_POTENTIAL_GRADE) == 0 {
        return false;
    }
    if read_i32(this + player_search_fields::ASSIGNED_NATION) > -1 || read_i32(this + player_search_fields::ASSIGNED_COMP) > -1
        || read_u32(this + player_search_fields::ASSIGNED_BASE) != 0 || read_i8(this + player_search_fields::ASSIGNED_REGION) > -1 {
        covered!("rx06_3 a new result");
        player_search_append_result(this, entry, 0x1131);
        player_search_sort_results(this);
    }
    false
}

/// The transfer rumour of a well graded player (grade above 2): the scout club must have finances and not be a national
/// team; it happens 1 time in 37 − current reputation / 300; below a reputation of 5000 + random(2000), only for a
/// player at a human club. With an interest of 3.15 or more, the seller is the club of his contract (none on a type 5
/// contract), which becomes the club of the news' strength flags. A guessed shortlist slot (price random(20000), or
/// −10000 without budget or money; valuation random(40000), both as 16-bit values) decides whether it is a
/// compensation deal; otherwise the seller's asking price (the human asking price at a human club, else
/// `calc_asking_price` with 0, 1, 1). When the budget or the balance reaches 0.75 × the price, the media hears of the
/// interest.
unsafe fn scout_report_rumour(run: &mut ScoutReportRun, player: Staff, scout_club: Club, strength_club: &mut usize) {
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), scout_club);
    if finance == 0 || exe::database_is_national_team(scout_club) != 0 {
        return;
    }
    let reputation = read_i16(player.player().addr() + player_sheet::CURRENT_REPUTATION) as i32;
    if exe::utils_random(37 - reputation / 300) != 0 {
        return; // rx06_3: no rumour today
    }
    if reputation <= exe::utils_random(2000) + 5000 {
        let club = player.club_job();
        if club.is_null() || exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
            return;
        }
    }
    if scout_report_interest(player, scout_club) < read_f64(rdata::F64_3_15) {
        return;
    }
    covered!("rx06_3 a transfer rumour");
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    let seller = if !contract.is_null() && contract.contract_type() & 0x3f != 5 { club_at(contract.club() as isize).addr() } else { 0 };
    *strength_club = seller;
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player); // asked again, not kept
    let mut slot = [0u8; 0x25];
    let s = slot.as_mut_ptr() as usize;
    exe::date_date_54a110(s + shortlist_slot::DATE);
    write_i32(s + shortlist_slot::PRICE, exe::utils_random(20000) as i16 as i32);
    write_i32(s + shortlist_slot::VALUATION, exe::utils_random(40000) as i16 as i32);
    let budget = || exe::finance_get_transfer_budget(finance, player.addr() as u32, 1, 0usize, 1, 0);
    if budget() <= 0 || read_i64(finance + finance::BALANCE) <= 0 {
        write_i32(s + shortlist_slot::PRICE, -10000);
    }
    let mut fee = 0i32;
    let deal = exe::shortlist_manager_is_compensation_deal(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), s, contract, scout_club, &mut fee);
    let price = if seller != 0 && deal == 0 {
        if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
            exe::contract_manager_get_human_asking_price(read_usize(G_CONTRACT_MANAGER), seller, scout_club, player)
        } else {
            exe::contract_manager_calc_asking_price(read_usize(G_CONTRACT_MANAGER), seller, scout_club, player, 0, 1, 1)
        }
    } else {
        0
    };
    let floor = price as f64 * read_f64(rdata::F64_0_75);
    if (budget() as f64) < floor && (read_i64(finance + finance::BALANCE) as f64) < floor {
        return;
    }
    let interest = scout_report_interest(player, scout_club);
    run.add_transfer_interest(player, scout_club, price, interest);
}

/// The report record the caller asked for (`mode` 1 requested or 2 general; mode 2 only with a grade of 4 or more):
/// the mode, the slot, the player's age, positions and grades, the scout club id, the player's and the scout's name
/// references, the watched match (side club from `result`) and the eligibility: 1 and the permit outcome when he needs a
/// work permit, else 2 through a second nationality or 0, and 0. A scout without a club: nothing. Answers whether the
/// record was filled.
#[allow(clippy::too_many_arguments)]
unsafe fn scout_report_fill_record(this: usize, record: usize, mode: u8, player: Staff, scout: Staff, grades: (u8, u8), fixture: Fixture,
                                   result: usize, side: u8, permit: (bool, bool)) -> bool {
    if mode == 2 && grades.0 < 4 && grades.1 < 4 {
        return false; // rx06_3: a general report needs a 4
    }
    if scout.club_job().is_null() {
        return false;
    }
    covered!("rx06_3 the report record");
    write_u8(record + scout_report::REPORT_MODE, mode);
    write_u8(record + SCOUT_REPORT_SLOT, read_u8(this + SCOUT_DAY_SLOT));
    write_u8(record + scout_report::PLAYER_AGE, read_u8(player.addr() + staff::AGE));
    write_u16(record + scout_report::POSITION_MASK, exe::formation_get_position_mask(player.player()));
    write_u8(record + scout_report::CURRENT_RATING, grades.0);
    write_u8(record + scout_report::POTENTIAL_RATING, grades.1);
    write_u32(record + scout_report::CLUB_ID, read_u32(scout.club_job().addr()));
    exe::fill_staff_name_ref(player, record + scout_report::PLAYER_NAME_REF);
    exe::fill_staff_name_ref(scout, record + scout_report::SCOUT_NAME_REF);
    if !fixture.is_null() {
        write_u32(record + scout_report::HAS_MATCH, 1);
        write_u32(record + scout_report::LEAGUE_MATCH, (read_u8(fixture.addr() + fixture::HOME_SIDE_MARK) == 0) as u32);
        exe::fill_match_record_from_fixture(fixture, scout_report_side_club(result, side), record);
    } else {
        write_u32(record + scout_report::HAS_MATCH, 0);
    }
    let (needs, granted) = permit;
    if needs {
        write_u8(record + scout_report::ELIGIBILITY_NOTE, 1);
        write_u32(record + scout_report::WORK_PERMIT_GRANTED, granted as u32);
    } else {
        write_u8(record + scout_report::ELIGIBILITY_NOTE, if scout_report_second_nationality(player, scout) { 2 } else { 0 });
        write_u32(record + scout_report::WORK_PERMIT_GRANTED, 0);
    }
    true
}

/// The recommendation news 0x138b to the manager: slot, the scout, age, positions, the player, his club id (−1 none),
/// the two grades, the assignment (nation, region, competition, youth), whether from a match, whether a league match,
/// whether a record was asked, the work permit (1 and the outcome; else 2 through a second nationality or 0, and 0),
/// the strength and weakness words (`player_search_assess_player_traits`), the watched match, 0x420e9 and the player's
/// strength flags for `strength_club`.
#[allow(clippy::too_many_arguments)]
unsafe fn scout_report_news(run: &mut ScoutReportRun, this: usize, player: Staff, scout: Staff, owner: Staff, grades: (u8, u8), mode: u8,
                            fixture: Fixture, result: usize, side: u8, permit: (bool, bool), strength_club: usize) {
    covered!("rx06_3 the recommendation news");
    let mut item = [0u8; 0x100];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0x138b, 0);
    let mut n: u8 = 0;
    let age = read_i8(player.addr() + staff::AGE);
    let positions = exe::formation_get_position_mask(player.player()) as i16;
    let mut name = [0u8; 0x20];
    exe::fill_staff_name_ref(player, name.as_mut_ptr() as usize);
    let club_id = if strength_club != 0 { read_i32(strength_club) } else { -1 };
    let mut game = [0u8; MATCH_DATA_SIZE];
    if !fixture.is_null() {
        exe::date_date_54a110(game.as_mut_ptr() as usize);
        exe::fill_match_record_from_fixture(fixture, scout_report_side_club(result, side), game.as_mut_ptr() as usize);
    }
    let slot = |n: &mut u8, v: u32| {
        exe::newsitem_set_slot(it, *n as i8, v);
        *n = n.wrapping_add(1);
    };
    slot(&mut n, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
    exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
    slot(&mut n, age as i32 as u32);
    slot(&mut n, positions as i32 as u32);
    exe::write_staff_name_ref_to_news(name.as_ptr() as usize, it, &mut n);
    slot(&mut n, club_id as u32);
    slot(&mut n, grades.0 as u32);
    slot(&mut n, grades.1 as u32);
    slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_NATION));
    slot(&mut n, read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32);
    slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_COMP));
    slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_BASE));
    slot(&mut n, !fixture.is_null() as u32);
    slot(&mut n, (!fixture.is_null() && read_u8(fixture.addr() + fixture::HOME_SIDE_MARK) == 0) as u32);
    slot(&mut n, (mode != 0) as u32);
    let (needs, granted) = permit;
    if needs {
        slot(&mut n, 1);
        slot(&mut n, granted as u32);
    } else {
        slot(&mut n, if scout_report_second_nationality(player, scout) { 2 } else { 0 });
        slot(&mut n, 0);
    }
    let mut words = [0i16; 2];
    exe::player_search_assess_player_traits(player, scout, words.as_mut_ptr().add(1), words.as_mut_ptr());
    slot(&mut n, words[1] as i32 as u32);
    slot(&mut n, words[0] as i32 as u32);
    if !fixture.is_null() {
        exe::write_match_record_to_news(game.as_ptr() as usize, it, &mut n);
    }
    slot(&mut n, 0x420e9);
    // the index byte is taken before the strength flags are asked (the original bumps it first; nothing reads it after)
    exe::newsitem_set_slot(it, n as i8, exe::staff_get_player_strength_flags(player, strength_club));
    run.day.deliver(it, owner);
}

/// The report itself (`report_on_player`), for the game or as the comparer's dry run.
#[allow(clippy::too_many_arguments)]
unsafe fn scout_report(run: &mut ScoutReportRun, this: usize, player: Staff, record: usize, bias: i32, mode: u8, fixture: Fixture, result: usize, side: u8) {
    let mut default_date = [0u8; 8];
    exe::date_date_54a110(default_date.as_mut_ptr() as usize);
    let scout = Staff(get_scout(this));
    let scout_club = scout.club_job(); // the original reads it without checking the scout
    if scout_club.is_null() {
        show_error_at(0xa701b0, 0x1015);
        return;
    }
    let owner = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    if record != 0 {
        write_u8(record + scout_report::REPORT_MODE, 0xff);
    }
    let target = player_search_get_club_target_rating(scout_club.addr(), core::ptr::null_mut(), core::ptr::null()).clamp(1750, 35000);
    let mut strength_club = player_search_contract_club(player);
    if player.player().is_null() {
        return;
    }
    let record_report: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_RECORD_SCOUT_REPORT);
    record_report(read_usize(G_SCOUT_MANAGER), player.addr(), scout_club.addr());
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
        let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), owner);
        if map != 0 {
            run.day.raise_knowledge(map, player, scout);
        }
    }
    if !fixture.is_null() && read_u32(fixture.addr()) == read_u32(KEY_CLUB_COMP_RESERVE) {
        let stored = exe::fixture_manager_find_stored_fixture(read_usize(G_FIXTURE_MANAGER), fixture);
        if stored != 0 {
            run.set_league_category(stored);
        }
    }
    let (current, potential) = scout_report_worths(player, scout, bias);
    let bar = target as f64 * read_f64(rdata::F64_0_6);
    if current as f64 <= bar && potential as f64 <= bar && mode != 1 {
        return;
    }
    if scout_report_interest(player, scout_club) < read_f64(rdata::F64_3_15) && mode != 1 {
        return; // rx06_3: he would not come
    }
    let mut entry = [0u8; SCOUT_DAY_RESULT_SIZE];
    let e = entry.as_mut_ptr() as usize;
    exe::fill_staff_name_ref(player, e);
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(e + 0x10, tmp.as_mut_ptr() as usize, default_date.as_ptr() as usize);
    exe::date_operator_3d(e + search_result::LAST_UPDATE_DATE, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_i32(e + RESULT_CURRENT_WORTH, current);
    write_i32(e + RESULT_POTENTIAL_WORTH, potential);
    write_u8(e + RESULT_NOTE_RATING, player_search_note_rating(this, player));
    let reputation = scout_club.reputation() as i32;
    let grades = (scout_grade(current, target as f64, reputation, 10750), scout_grade(potential, target as f64, reputation, 10500));
    write_u8(e + RESULT_CURRENT_GRADE, grades.0);
    write_u8(e + RESULT_POTENTIAL_GRADE, grades.1);
    let top = scout_report_update_result(this, e);
    if grades.0 > 2 || grades.1 > 2 {
        scout_report_rumour(run, player, scout_club, &mut strength_club);
    }
    let tm = read_usize(G_TRANSFER_MANAGER);
    let needs = exe::transfer_manager_player_needs_work_permit(tm, player, scout_club, 1) != 0;
    let granted = !needs || exe::transfer_manager_is_work_permit_granted(tm, player, scout_club.nation(), 0usize, 0usize, 1) != 0;
    if mode != 0 && record != 0 {
        if !scout_report_fill_record(this, record, mode, player, scout, grades, fixture, result, side, (needs, granted)) {
            return; // not even the mark
        }
    } else if !top && read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 && (grades.0 >= 5 || grades.1 >= 5) {
        scout_report_news(run, this, player, scout, owner, grades, mode, fixture, result, side, (needs, granted), strength_club);
    }
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
}

/// Judges a player the scout saw (in a watched match, on a request or for the list) for the manager: the scout (a
/// scout without a club: the player_search.cpp box of line 0x1015), the report record's mode set to 0xff (no report
/// yet), the scout club's target (`player_search_get_club_target_rating`, between 1750 and 35000) and the player's
/// contract club. A player without a sheet: nothing more. One report more on a request (`record_scout_report`), the
/// manager comes to know the player (fog of war, with attribute masking) and the stored fixture of a reserve match gets
/// category 2. The two worths (`scout_report_worths`, with `bias`); unless `mode` is 1, nothing more when both are at
/// most 0.6 × the target or the player's interest in coming is below 3.15. His result is brought up to date or added
/// (`scout_report_update_result`); a grade above 2 may start a transfer rumour (`scout_report_rumour`). With a mode and
/// a record, the record is filled (`scout_report_fill_record`); otherwise, for a player who did not already have a 5,
/// outside opposition scouting and with a grade 5, the recommendation news (`scout_report_news`). Last,
/// `SCOUT_DAY_REPORT_MARK` is cleared.
/// Defects of the original: a list whose scout is gone is read at address 0x39 (the club of a null scout) and the game
/// falls; the new result's byte +0x2b is the stack's leftover (here 0); the rumour's guessed valuation random(40000) is
/// kept as a 16-bit value, so above 32767 it turns negative; the flag byte of a found result is or-ed with 0.
/// Original: `PLAYER_SEARCH::report_on_player` (0x7b62a0, player_search.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn player_search_report_on_player(this: usize, player: Staff, record: usize, bias: i32, mode: u32, fixture: Fixture,
                                                                       result: usize, side: u32) {
    let mut run = ScoutReportRun::new(false);
    scout_report(&mut run, this, player, record, bias, mode as u8, fixture, result, side as u8);
}

/// The news items the original delivered into `owner`'s inbox ring since `next_before`, against the dry run's (kind
/// +0..+3 and data +5..+0xcc); the differences.
unsafe fn scout_report_news_differences(owner: i32, next_before: Option<i32>, expected: &[[u8; 0xcd]]) -> Vec<String> {
    let mut wrong = Vec::new();
    if expected.is_empty() {
        return wrong;
    }
    let news = read_usize(G_NEWS);
    let idx = owner.wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    let inbox = if idx >= 0 && news != 0 { read_usize(news + news_fields::HUMAN_INBOXES) + idx as usize * 12 } else { 0 };
    if inbox == 0 || read_i32(inbox) == -1 {
        wrong.push(format!("{} news items expected, no inbox", expected.len()));
        return wrong;
    }
    let start = read_i32(inbox);
    let mut slot = next_before.unwrap_or(start);
    for (k, e) in expected.iter().enumerate() {
        let item = read_usize(news).wrapping_add((slot as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
        let got = core::slice::from_raw_parts(item as *const u8, 0xcd);
        if got[0..4] != e[0..4] || got[5..] != e[5..] {
            wrong.push(format!("news {} kind {:#x} x {:#x}", k, read_u32(item), u32::from_le_bytes([e[0], e[1], e[2], e[3]])));
        }
        slot += 1;
        if slot == start + 0x64 {
            slot = start;
        }
    }
    wrong
}

/// Comparer for `PLAYER_SEARCH::report_on_player` (model `compare_run_scout_day`). The copy runs first as a dry run:
/// on a duplicate of the result array, keeping the news and the media interests instead of giving them, saving the fog
/// of war records and the fixture categories before they change. Its regions (the random generator, the contract
/// lookup, the list object, the report record and the scout requests), result array, fog records and categories are
/// kept, everything goes back and the original runs. Compared: the regions (but the array pointer), the result array
/// (each result's +0x2b ignored), the fog records, the categories, each interest the dry run would add (present in the
/// media afterwards) and each news item the original delivered against the dry run's.
pub(crate) unsafe extern "thiscall" fn compare_player_search_report_on_player(this: usize, player: Staff, record: usize, bias: i32, mode: u32,
                                                                               fixture: Fixture, result: usize, side: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, Staff, usize, i32, u32, Fixture, usize, u32)>(%%I0%%);
    let mut regions: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    regions.push((this, SCOUT_DAY_OBJECT_SIZE));
    if record != 0 {
        regions.push((record, 0x67));
    }
    let sm = read_usize(G_SCOUT_MANAGER);
    let requests = read_usize(sm + scout_manager_fields::SCOUT_REQUESTS);
    if requests != 0 {
        regions.push((requests, read_u32(sm + scout_manager_fields::SCOUT_REQUEST_COUNT) as usize * 0x11));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let old = read_usize(this + player_search_fields::RESULTS);
    if old != 0 {
        let n = read_i32(this + SCOUT_DAY_RESULT_CAPACITY).max(1) as usize * SCOUT_DAY_RESULT_SIZE;
        let copy = exe::crt_malloc(n as u32);
        core::ptr::copy_nonoverlapping(old as *const u8, copy as *mut u8, n);
        write_usize(this + player_search_fields::RESULTS, copy);
    }
    let mut run = ScoutReportRun::new(true);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| scout_report(&mut run, this, player, record, bias, mode as u8, fixture, result, side as u8)));
    let array = || -> Vec<u8> {
        let a = read_usize(this + player_search_fields::RESULTS);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let mut v = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        for entry in v.chunks_exact_mut(SCOUT_DAY_RESULT_SIZE) {
            entry[RESULT_UNSET_BYTE] = 0;
        }
        v
    };
    let mut dry_state = crate::comparison::MemorySnapshot::read(&regions);
    let dry_results = array();
    let dry_array = read_usize(this + player_search_fields::RESULTS);
    if dry_array != 0 && dry_array != old {
        exe::crt_free(dry_array);
    }
    let dry_knowledge: Vec<[u8; 9]> = run.day.knowledge.iter().map(|(a, _)| core::array::from_fn(|k| read_u8(*a + k))).collect();
    for (a, b) in run.day.knowledge.iter().rev() {
        core::ptr::copy_nonoverlapping(b.as_ptr(), *a as *mut u8, 9);
    }
    for (a, b) in run.categories.iter().rev() {
        write_u8(*a + fixture::CATEGORY, *b);
    }
    before.write_back();
    let owner = read_i32(this + SCOUT_DAY_OWNER);
    let news = read_usize(G_NEWS);
    let idx = owner.wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    let inbox = if idx >= 0 && news != 0 { read_usize(news + news_fields::HUMAN_INBOXES) + idx as usize * 12 } else { 0 };
    let next_before = if inbox != 0 && read_i32(inbox) != -1 { Some(read_i32(inbox + 4)) } else { None };
    o(this, player, record, bias, mode, fixture, result, side);
    let mut wrong = scout_report_news_differences(owner, next_before, &run.day.news);
    let mut after = crate::comparison::MemorySnapshot::read(&regions);
    let pointer = player_search_fields::RESULTS;
    for snapshot in [&mut after, &mut dry_state] {
        if let Some((_, object)) = snapshot.0.iter_mut().find(|(a, _)| *a == this) {
            object[pointer..pointer + 4].fill(0);
        }
    }
    if after != dry_state {
        wrong.push(format!("regions {:?}", first_differences(&after, &dry_state)));
    }
    if array() != dry_results {
        wrong.push(format!("the result array ({} x {} results)", read_i32(this + player_search_fields::RESULT_COUNT), dry_results.len() / SCOUT_DAY_RESULT_SIZE));
    }
    for (k, (a, _)) in run.day.knowledge.iter().enumerate() {
        if core::slice::from_raw_parts(*a as *const u8, 9) != dry_knowledge[k] {
            wrong.push(format!("fog of war record {:#x}", a));
        }
    }
    for (a, _) in run.categories.iter() {
        if read_u8(*a + fixture::CATEGORY) != 2 {
            wrong.push(format!("fixture {:#x} category {}", a, read_u8(*a + fixture::CATEGORY)));
        }
    }
    for (p, c, _, _) in run.interests.iter() {
        if !has_interest(read_usize(G_MEDIA), &interest_key(p.addr(), c.addr())) {
            wrong.push(format!("no transfer interest of club {:#x} in player {:#x}", c.addr(), p.addr()));
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("report_on_player(search {:#x}, player {:#x}, mode {}, bias {}) {:?}", this, player.addr(), mode as u8, bias, wrong));
}
