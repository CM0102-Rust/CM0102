use super::*;

// The comparers of the routines in ../comp/leagues/fin_prm.rs.

/// Comparer for `fin_prm_end_season_for_nation`.
///
/// The nation's end-of-season comparer: only the original routine runs, because it calls each club's end of season
/// (`season_end_club_update`), which draws at random and delivers news items to the human manager (the copy carries on in Rust mode,
/// with the counter).
///
/// It checks what the routine guarantees and no call undoes: the last club of the entry (the club order) ends up with
/// the year's totals (+0x110, 0x50 bytes) zeroed.
///
/// Ponytail: the earlier ones may gain values through the `season_end_club_update` of a later club (the B team that uses the A team's
/// record, the shared stadium); checking all of them requires planning those exceptions.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_end_season_for_nation(this: usize, entry: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIN_PRM_END_SEASON_FOR_NATION);
    if entry == 0 || exe::date_operator_3c(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return o(this, entry);
    }
    let mut last = None;
    let mut club = read_usize(G_CLUB_TABLE);
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let nation = Nation(read_usize(club + club::NATION));
        if read_usize(club + club::DIVISION) != 0 && !nation.is_null() && (nation.league_selected() != 0 || read_i16(club + club::REPUTATION) >= 0x1388) && (0xb63d60 + (exe::key_nation_index_of_club(club) as i8 as i32) * 72) as usize == entry {
            last = Some(k);
        }
        club += club::SIZE;
    }
    o(this, entry);
    if let Some(k) = last {
        let f = read_usize(this) + k * 0x167 + finance::SEASON_PLAYERS_BOUGHT;
        let cleared = core::slice::from_raw_parts(f as *const u8, 0x50).iter().all(|&x| x == 0);
        SCOREBOARD.record(cleared, || format!("fin_prm_end_season_for_nation(entry {:#x}) club {} without clear the totais of the year", entry, k));
    }
}

/// Comparer for `season_end_club_update` (comparison mode: see the header of motor.rs): the comparer of the club's
/// season end, side by side (the copy without counting the random draws).
///
/// Compared: the finances of the partner and of the club itself, the club, the stadium (78 bytes), the requests row
/// (0x49 bytes) and the random-draw state. The news (`0x59ffb0`, `0x59fae0`, `0x59fe10`) is built on the stack and only
/// delivered to the human manager (`0x7861e0`): with a computer manager it leaves nothing besides the random draw(2) of
/// the refusal, which both runs make.
///
/// Only the original routine: the human manager (the news goes into his inbox) and the shared stadium (it touches
/// another club and its finances).
pub(crate) unsafe extern "thiscall" fn compare_season_end_club_update(this: usize, club: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEASON_END_CLUB_UPDATE);
    if club.is_null() || exe::database_is_national_team(club) != 0 || fin_prm_has_human_manager(club) {
        return o(this, club.addr());
    }
    let mut a = 0u32;
    let fin = finance_of_parent_club(this, club, &mut a);
    let stadium = club.stadium().addr();
    if fin.is_null() || (stadium != 0 && club.own_stadium() == 0 && club.division().addr() != 0 && read_i32(fin.addr()) > 0x2160ec0) {
        return o(this, club.addr()); // simplification: the stadium purchase may touch the owner; original only
    }
    let own = read_usize(this).wrapping_add((read_i32(club.addr()) as u32).wrapping_mul(0x167) as usize);
    let mut reg = vec![(fin.addr(), 0x167), (club.addr(), club::SIZE), (board_requests_row(club), 0x49)];
    if own != fin.addr() {
        reg.push((own, 0x167));
    }
    if stadium != 0 {
        reg.push((stadium, 0x4e));
    }
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club.addr()), || crate::rng::without_counting(|| season_end_club_update(this, club.addr())));
    SCOREBOARD.record(igual, || format!("season_end_club_update(club {:x}) original={:x?} rust={:x?}", club.addr(), mo, mr));
}

/// Comparer for `news_dividend`.
pub(crate) unsafe extern "thiscall" fn compare_news_dividend(this: usize, club: usize, value: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_NEWS_DIVIDEND)(this, club, value)
}

/// Comparer for `get_max_wage`.
pub(crate) unsafe extern "thiscall" fn compare_get_max_wage(fin: usize, person: usize, job: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_GET_MAX_WAGE)(fin, person, job);
    let n = get_max_wage(fin, person, job);
    SCOREBOARD.record(r == n, || format!("get_max_wage(fin {:x}, person {:x}, job {}) original={} rust={}", fin, person, job as u8 as i8, r, n));
    r
}

/// Comparer for `news_ground_works`.
pub(crate) unsafe extern "thiscall" fn compare_news_ground_works(this: usize, club: usize, value: u32, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_GROUND_WORKS)(this, club, value, kind)
}

/// Comparer for `news_request_refused`.
pub(crate) unsafe extern "thiscall" fn compare_news_request_refused(this: usize, club: usize, subject: u32, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_NEWS_REQUEST_REFUSED)(this, club, subject, reason)
}

/// Comparer for `save_finance_dat`.
pub(crate) unsafe extern "thiscall" fn compare_save_finance_dat(this: usize, save: usize) {
    // simplification: writes to the save container (cannot run twice); the copy is proven by the save proof
    // (the .qst saved in rust mode with and without the batch, item by item)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_FINANCE_DAT)(this, save)
}

/// Comparer for `load_from_save`.
pub(crate) unsafe extern "thiscall" fn compare_load_from_save(this: usize, save: usize) -> u32 {
    // simplification: reads from the container (the item position advances; cannot run twice); the copy is proved by loading
    // of the holiday save in the save proof (levas/N/CARGA)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_FROM_SAVE)(this, save)
}

/// Comparer for `process_match_income`.
pub(crate) unsafe extern "thiscall" fn compare_process_match_income(table: usize, fixture: usize, result: usize) {
    let fixture = Fixture(fixture);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PROCESS_MATCH_INCOME);
    if result == 0 || fixture.is_null() || fixture.home_club().is_null() || fixture.away_club().is_null() {
        return o(table, fixture.addr(), result);
    }
    let (Some(fm), Some(fv)) = (match_club_finance(table, fixture, 0x1c), match_club_finance(table, fixture, 0x20)) else {
        return o(table, fixture.addr(), result); // national team: the original does nothing
    };
    if fm == fv {
        return o(table, fixture.addr(), result); // simplification: the same record on both sides, original only
    }
    let mut reg = vec![(fm, 0x167), (fv, 0x167)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(table, fixture.addr(), result), || crate::rng::without_counting(|| process_match_income(table, fixture.addr(), result)));
    SCOREBOARD.record(igual, || format!("process_match_income(match {:x}) original={:x?} rust={:x?}", fixture.addr(), mo, mr));
}

/// Comparer for `calc_gate_receipts`.
pub(crate) unsafe extern "thiscall" fn compare_calc_gate_receipts(table: usize, fixture: usize, attendance: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_CALC_GATE_RECEIPTS);
    if fixture == 0 {
        return o(table, fixture, attendance);
    }
    let r = o(table, fixture, attendance);
    let n = calc_gate_receipts(table, fixture, attendance);
    SCOREBOARD.record(r == n, || format!("calc_gate_receipts(match {:x}, attendance {}) original={} rust={}", fixture, attendance, r, n));
    r
}

/// Comparer for `pay_league_fine`.
pub(crate) unsafe extern "thiscall" fn compare_pay_league_fine(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_LEAGUE_FINE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_league_fine(this, value));
    SCOREBOARD.record(igual, || format!("pay_league_fine(value {}) original={:x?} rust={:x?}", value, mo, mr));
}

/// Comparer for `try_finance_boost_event`.
pub(crate) unsafe extern "thiscall" fn compare_try_finance_boost_event(this: usize, club: usize, fin: usize) -> u32 {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_FINANCE_BOOST_EVENT);
    let presence = Staff(if !club.is_null() { club.chairman().addr() } else { 0 });
    let sheet = PlayerSheet(if !presence.is_null() { presence.non_player().addr() } else { 0 });
    if club.is_null() || fin == 0 {
        return o(this, club.addr(), fin); // simplification: without a club or without finances, original only
    }
    let mut extras = vec![(club.addr() + club::PLC, 1)];
    let mut check = Vec::new();
    if !sheet.is_null() {
        extras.push((sheet.addr() + player_sheet::BRAVERY, 1)); // the record’s +0x20 drops on every event with it
        check.push(1);
    }
    compare_rare_finance_event("try_finance_boost_event", fin, &extras, &check, || o(this, club.addr(), fin), || try_finance_boost_event(this, club.addr(), fin))
}

/// Comparer for `try_board_takeover`.
pub(crate) unsafe extern "thiscall" fn compare_try_board_takeover(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_TAKEOVER);
    if club == 0 || fin == 0 {
        return o(this, club, fin); // simplification: without a club or without finances, original only
    }
    compare_rare_finance_event("try_board_takeover", fin, &[], &[], || o(this, club, fin), || try_board_takeover(this, club, fin))
}

/// Comparer for `pay_other_costs`.
pub(crate) unsafe extern "thiscall" fn compare_pay_other_costs(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_OTHER_COSTS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_other_costs(this, value));
    SCOREBOARD.record(igual, || format!("pay_other_costs(value {}) original={:x?} rust={:x?}", value, mo, mr));
}

/// Comparer for `update_wage_bill`.
pub(crate) unsafe extern "thiscall" fn compare_update_wage_bill(this: usize) {
    let this = Finance(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WAGE_BILL);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + finance::WAGE_BILL, 12), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this.addr()), || update_wage_bill(this.addr()));
    SCOREBOARD.record(igual, || format!("update_wage_bill(fin {:#x}) original={:x?} rust={:x?}", this.addr(), mo, mr));
}

/// Comparer for `refresh_all_wage_bills`.
pub(crate) unsafe extern "thiscall" fn compare_refresh_all_wage_bills(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REFRESH_ALL_WAGE_BILLS);
    let n = (read_i32(G_CLUB_COUNT).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(read_usize(this), n), (G_CONTRACT_LOOKUP_RESULT, 4)], || o(this), || refresh_all_wage_bills(this));
    SCOREBOARD.record(igual, || format!("refresh_all_wage_bills({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `finance_get_days_to_season_start`.
pub(crate) unsafe extern "thiscall" fn compare_finance_get_days_to_season_start(this: usize, club: usize, passed: *mut u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(IDX_FINANCE_GET_DAYS_TO_SEASON_START);
    if (exe::key_nation_index_of_club(club) as i8) < 0 {
        return o(this, club, passed);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(passed as usize, 4), (G_DAYS_SINCE_SCRATCH_DATE, 8)], || o(this, club, passed), || finance_get_days_to_season_start(this, club, passed));
    SCOREBOARD.record(igual && r == n, || format!("finance_get_days_to_season_start(club {:#x}) original={} {:x?} rust={} {:x?}", club, r, mo, n, mr));
    r
}

/// Comparer for `add_players_sold`.
pub(crate) unsafe extern "thiscall" fn compare_add_players_sold(this: usize, low: u32, high: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_ADD_PLAYERS_SOLD);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, low, high), || add_players_sold(this, low, high));
    SCOREBOARD.record(igual, || format!("add_players_sold(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(low, high), mo, mr));
}

/// Comparer for `pay_players_bought`.
pub(crate) unsafe extern "thiscall" fn compare_pay_players_bought(this: usize, low: u32, high: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_PAY_PLAYERS_BOUGHT);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, low, high), || pay_players_bought(this, low, high));
    SCOREBOARD.record(igual, || format!("pay_players_bought(fin {:#x}, {}) original={:x?} rust={:x?}", this, i64_from_halves(low, high), mo, mr));
}

/// Comparer for `pay_sign_on_fee`.
pub(crate) unsafe extern "thiscall" fn compare_pay_sign_on_fee(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_SIGN_ON_FEE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_sign_on_fee(this, value));
    SCOREBOARD.record(igual, || format!("pay_sign_on_fee(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_prize_money`.
pub(crate) unsafe extern "thiscall" fn compare_add_prize_money(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_PRIZE_MONEY);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || add_prize_money(this, value));
    SCOREBOARD.record(igual, || format!("add_prize_money(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_tv_revenue`.
pub(crate) unsafe extern "thiscall" fn compare_add_tv_revenue(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_ADD_TV_REVENUE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || add_tv_revenue(this, value));
    SCOREBOARD.record(igual, || format!("add_tv_revenue(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `pay_wages`.
pub(crate) unsafe extern "thiscall" fn compare_pay_wages(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_WAGES);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_wages(this, value));
    SCOREBOARD.record(igual, || format!("pay_wages(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparer for `add_weekly_merchandising_income`.
pub(crate) unsafe extern "thiscall" fn compare_add_weekly_merchandising_income(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_WEEKLY_MERCHANDISING_INCOME);
    let mut regions = vec![(fin_prm_club_finance(this, Club(club)), 0x167)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club), || crate::rng::without_counting(|| add_weekly_merchandising_income(this, club)));
    SCOREBOARD.record(igual, || format!("add_weekly_merchandising_income(club {:#x}) original={:x?} rust={:x?}", club, mo, mr));
}

/// Comparer for `sell_season_tickets`.
pub(crate) unsafe extern "thiscall" fn compare_sell_season_tickets(this: usize, apply: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_SELL_SEASON_TICKETS);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(read_usize(this + 8).wrapping_mul(club::SIZE)));
    if club.is_null() {
        return o(this, apply);
    }
    let mut regions = vec![(this, 0x167)];
    let est = club.stadium();
    if !est.is_null() {
        regions.push((est.addr() + stadium::STADIUM_CAPACITY, 4));
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, apply), || crate::rng::without_counting(|| sell_season_tickets(this, apply)));
    SCOREBOARD.record(igual, || format!("sell_season_tickets(fin {:#x}, {}) original={:x?} rust={:x?}", this, apply, mo, mr));
}

/// Comparer for `upgrade_training_facilities`.
pub(crate) unsafe extern "thiscall" fn compare_upgrade_training_facilities(this: usize, club: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPGRADE_TRAINING_FACILITIES);
    let regions = [(club.addr() + club::TRAINING, 1), (fin_prm_club_finance(this, club), 0x167)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club.addr()), || upgrade_training_facilities(this, club.addr()));
    SCOREBOARD.record(igual, || format!("upgrade_training_facilities(club {:#x}) original={:x?} rust={:x?}", club.addr(), mo, mr));
}

/// Comparer for `process_club_finance_events`.
pub(crate) unsafe extern "thiscall" fn compare_process_club_finance_events(this: usize, club: usize, week: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_CLUB_FINANCE_EVENTS)(this, club, week)
}

/// Comparer for `try_board_debt_payment`.
pub(crate) unsafe extern "thiscall" fn compare_try_board_debt_payment(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_BOARD_DEBT_PAYMENT);
    compare_finance_event("try_board_debt_payment", fin, || o(this, club, fin), || try_board_debt_payment(this, club, fin))
}

/// Comparer for `try_cash_injection`.
pub(crate) unsafe extern "thiscall" fn compare_try_cash_injection(this: usize, club: usize, fin: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_TRY_CASH_INJECTION);
    compare_finance_event("try_cash_injection", fin, || o(this, club, fin), || try_cash_injection(this, club, fin))
}

/// Comparer for `get_max_staff_wage`.
pub(crate) unsafe extern "thiscall" fn compare_get_max_staff_wage(this: usize, job: i8) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_GET_MAX_STAFF_WAGE)(this, job);
    let n = get_max_staff_wage(this, job);
    SCOREBOARD.record(r == n, || format!("get_max_staff_wage(fin {:#x}, job {}) original={} rust={}", this, job, r, n));
    r
}

/// Comparer for `pay_bonuses`.
pub(crate) unsafe extern "thiscall" fn compare_pay_bonuses(this: usize, value: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_PAY_BONUSES);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x167)], || o(this, value), || pay_bonuses(this, value));
    SCOREBOARD.record(igual, || format!("pay_bonuses(fin {:#x}, {}) original={:x?} rust={:x?}", this, value, mo, mr));
}

/// Comparador: lado a lado no teto salarial (+0x14).
pub(crate) unsafe extern "thiscall" fn compare_adjust_max_wage(this: usize, percentage: i8) {
    let this = Finance(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_ADJUST_MAX_WAGE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + finance::WAGE_CEILING, 4)], || o(this.addr(), percentage), || adjust_max_wage(this.addr(), percentage));
    SCOREBOARD.record(igual, || format!("adjust_max_wage(fin {:#x}, {}) original={:x?} rust={:x?}", this.addr(), percentage, mo, mr));
}

/// Comparer for `get_wealth_band`.
pub(crate) unsafe extern "thiscall" fn compare_get_wealth_band(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_GET_WEALTH_BAND)(this);
    let n = get_wealth_band(this);
    SCOREBOARD.record(r == n, || format!("get_wealth_band(finances {:#x}) original={} rust={}", this, r as i8, n as i8));
    r
}
