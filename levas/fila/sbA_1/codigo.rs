// ---------- sbA_1: the daily finance pass (fin_prm.cpp, finance.CPP) ----------

/// The size of a club's finance record in the finance table (`[G_FINANCE_TABLE]`, one per club, by club id).
pub(crate) const FINANCE_SIZE: usize = 0x167;

/// The club's finance record in the finance table `this`: the k-th one, in club order.
unsafe fn finance_at(this: usize, k: usize) -> Finance {
    Finance(read_usize(this).wrapping_add(k.wrapping_mul(FINANCE_SIZE)))
}

/// Subtracts `v` from the i64 balance at `finance::BALANCE` (the amount sign-extended).
unsafe fn sub_i32_from_balance(fin: Finance, v: i32) {
    add_i32_to_balance(fin.addr(), v.wrapping_neg());
}

/// Whether the finance pass runs a club's books: it has a division and a nation, and in a nation with its league
/// selected the division is not the lower division key (`KEY_CLUB_COMP_A_LOWER_DIVISION`); in a nation without it, the
/// club's reputation is at least 5000.
unsafe fn club_has_finances(club: Club) -> bool {
    let division = club.division();
    let nation = club.nation();
    if division.is_null() || nation.is_null() {
        return false;
    }
    if nation.league_selected() != 0 {
        read_i32(division.addr()) != read_i32(KEY_CLUB_COMP_A_LOWER_DIVISION)
    } else {
        club.reputation() >= 5000
    }
}

/// Runs the daily finance pass of every club.
///
/// Nothing before the game start date (`G_GAME_START_DATE`). On 16/05/2004 Brighton (key 0x9d025c) moves to
/// Falmer and Everton (`G_KEY_CLUB_EVERTON`) to the Kings Dock (`finance_move_to_new_stadium`). Then, club by club up
/// to the first national team: a club whose books do not run (`club_has_finances`) but has a nation gets its balance
/// clamped by the professional status (`finance_clamp_balance`); one whose books run pays the week on Sundays (weekday
/// 6, `finance_weekly_pass`). On the last day of a month every club with books runs the month end
/// (`finance_month_end`) and the whole pass ends at the first national team; on the other days the clubs in crisis or
/// in administration run their finance events (`finance_daily_events`).
/// Defect of the original: on 16/05/2004 the Kings Dock move looks for Southampton's fixtures (`[0x9d0260]`), not
/// Everton's, so Everton's home fixtures keep the old ground.
/// Original: `FinanceTable::daily_update` (0x59b9c0, fin_prm.cpp (finance.CPP), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finance_daily_update(this: usize, today: usize) {
    let (start_day, start_year) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
    let (day, year) = (read_i16(today), read_u16(today + 2));
    if start_year > year || (start_year == year && start_day > day) {
        return;
    }
    let mut moving_day = [0u16; 4];
    let d = exe::date_date_549f70(&mut moving_day, 16, 5, 2004, u32::MAX);
    if read_u16(d + 2) == year && read_i16(d) == day {
        finance_stadium_moves(today);
    }
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = club_at(k as isize);
        if club.nation().is_null() {
            continue;
        }
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let fin = finance_at(this, k);
        if !club_has_finances(club) {
            finance_clamp_balance(club, fin);
        } else if exe::date_get_weekday(today) as u16 == 6 {
            finance_weekly_pass(this, club, fin);
        }
    }
    let mut tomorrow = [0u16; 4];
    let next = exe::date_add_days(today, &mut tomorrow as *mut [u16; 4] as usize, 1);
    let month_end = exe::date_get_month(today) != exe::date_get_month(next);
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = club_at(k as isize);
        if exe::database_is_national_team(club) != 0 {
            return;
        }
        if !club_has_finances(club) {
            continue;
        }
        if month_end {
            finance_month_end(this, club, finance_at(this, k));
        } else {
            finance_daily_events(this, club, finance_at(this, k));
        }
    }
}

/// The two stadium moves of 16/05/2004: Brighton to the first stadium named "Falmer" (6 letters) and Everton to the
/// first not yet built (`stadium::CAPACITY` below -10000) named "Kings Do" (8 letters), each only when the key club
/// exists and is not there already.
unsafe fn finance_stadium_moves(today: usize) {
    let brighton = read_i32(0x9d025c); // the Brighton and Hove Albion key
    let everton = read_i32(G_KEY_CLUB_EVERTON);
    if brighton == -1 || everton == -1 {
        return;
    }
    let (count, stadiums) = (read_i32(G_STADIUM_COUNT), read_usize(G_STADIUM_TABLE));
    let stadium_at = |k: i32| Stadium(stadiums.wrapping_add((k as usize).wrapping_mul(stadium::SIZE)));
    let falmer = (0..count).find(|&k| exe::crt_strncmp(stadium_at(k).addr() + stadium::NAME, 0x9c87c8, 6) == 0); // "Falmer"
    if let Some(k) = falmer {
        let club = club_at(brighton as isize);
        if read_i32(club.stadium().addr()) != k {
            club.set_own_stadium(1);
            club.set_stadium(stadium_at(k));
            // (reputation above, attendance added, minimum attendance)
            finance_move_to_new_stadium(club, [(3000, 2750, 12500), (1500, 1750, 7500), (i16::MIN as i32, 1250, 5750)], 3750, 30000, brighton, today);
        }
    }
    let kings_dock = (0..count).find(|&k| stadium_at(k).capacity() < -10000 && exe::crt_strncmp(stadium_at(k).addr() + stadium::NAME, 0x9c87bc, 8) == 0); // "Kings Dock"
    if let Some(k) = kings_dock {
        let club = club_at(everton as isize);
        let old = club.stadium();
        if read_i32(old.addr()) != k {
            club.set_own_stadium(1);
            old.set_capacity(old.capacity().wrapping_neg());
            let new = stadium_at(k);
            club.set_stadium(new);
            if new.capacity() < 0 {
                new.set_capacity(new.capacity().wrapping_neg());
            }
            // the defect: the fixtures searched are Southampton's (`[0x9d0260]`)
            finance_move_to_new_stadium(club, [(6750, 7500, 35000), (5250, 5000, 22500), (i16::MIN as i32, 2750, 17750)], 4250, 55000, read_i32(0x9d0260), today);
        }
    }
}

/// After a club moved to a new stadium: its attendance grows by the first reputation step it passes (at least that
/// step's minimum), the minimum and maximum attendances are set, the manager gets the news 0x177f (slots: 0 the club,
/// 1 the stadium's city, 2 the stadium) and the home fixtures of `fixtures_club` from today to day 200 of next year
/// move to the new stadium (`FIXTURE_MANAGER::remove_fixture`, stadium and its id, `add_fixture` with 1).
unsafe fn finance_move_to_new_stadium(club: Club, steps: [(i32, i32, i32); 3], min: i32, max: i32, fixtures_club: i32, today: usize) {
    let rep = club.reputation() as i32;
    let &(_, add, floor) = steps.iter().find(|s| rep > s.0).unwrap_or(&steps[2]);
    club.set_attendance(club.attendance().wrapping_add(add).max(floor));
    club.set_min_attendance(min);
    club.set_max_attendance(max);
    let stadium = club.stadium();
    let mut item = [0u8; NEWS_NEWS_ITEM_SIZE + 1];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0x177f, 0);
    exe::newsitem_set_slot(it, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(it, 1, read_u32(stadium.city()));
    exe::newsitem_set_slot(it, 2, read_u32(stadium.addr()));
    exe::news_send_to_club_manager(read_usize(G_NEWS), it, club);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let (day, year) = (read_u16(today) as u32, read_u16(today + 2) as u32);
    // simplification: the original pushes day and year with whatever its registers held above the low word
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day, year, 200, year + 1, fixtures_club as u32, l, u32::MAX, (-2i32) as u32, u32::MAX);
    write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
    loop {
        let f = exe::llist_get_next(l);
        if f == 0 {
            break;
        }
        if read_usize(f + fixture::HOME_CLUB) == club.addr() {
            exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), f);
            write_usize(f + fixture::STADIUM, club.stadium().addr());
            write_i32(f + fixture::STADIUM_ID, read_i32(club.stadium().addr()));
            exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), f, 1);
        }
    }
    exe::drop_llist(l);
}

/// The balance of a club whose books do not run is kept in a band by its professional status
/// (`club::PROFESSIONAL_STATUS`): 2, ±250000 with a reputation above 2500, else ±25000; 3, from -10000 to 2500; any
/// other, ±2250000, but past either end a random(5) of 0 swaps it to the other side (175000 below, -175000 above).
unsafe fn finance_clamp_balance(club: Club, fin: Finance) {
    let b = read_i64(fin.addr() + finance::BALANCE);
    let set = |v: i64| write_i64(fin.addr() + finance::BALANCE, v);
    match club.professional_status() as i8 {
        2 => {
            let band = if club.reputation() > 2500 { 250_000 } else { 25_000 };
            if b < -band {
                set(-band);
            } else if b > band {
                set(band);
            }
        }
        3 => {
            if b < -10_000 {
                set(-10_000);
            } else if b > 2500 {
                set(2500);
            }
        }
        _ => {
            if b < -2_250_000 {
                covered!("sbA_1 balance below the band");
                set(if exe::utils_random(5) != 0 { -2_250_000 } else { 175_000 });
            } else if b > 2_250_000 {
                set(if exe::utils_random(5) != 0 { 2_250_000 } else { -175_000 });
            }
        }
    }
}

/// Sunday's books of a club: the wage bill is recalculated (`Finance::update_wage_bill`) and paid (season and month
/// wages, out of the balance); the weekly merchandising comes in; while the season is still to start
/// (`finance_get_days_to_season_start`) season tickets are sold with the days passed, else the tickets sold and the
/// sales step go back to 0.
unsafe fn finance_weekly_pass(this: usize, club: Club, fin: Finance) {
    update_wage_bill(fin.addr());
    let wages = fin.wage_bill();
    add_to_i32(fin.addr() + finance::SEASON_WAGES, wages);
    add_to_i32(fin.addr() + finance::MONTH_WAGES, wages);
    sub_i32_from_balance(fin, wages);
    add_weekly_merchandising_income(this, club.addr());
    // simplification: the original leaves its local unset (finance_get_days_to_season_start always writes it)
    let mut passed = 0u32;
    if finance_get_days_to_season_start(this, club.addr(), &mut passed) != 0 {
        sell_season_tickets(fin.addr(), passed as i32);
    } else {
        fin.set_season_tickets_sold(0);
        fin.set_season_ticket_step(0);
    }
}

/// The month end of a club with books, in order: the players' unrest of a club in crisis
/// (`finance_crisis_unrest`), the stadium's covered stands and undersoil heating, the training ground, the gate
/// receipts of a club in a division without a competition object, the bank interest (`finance_bank_interest`), the
/// board's other costs and the sponsorship (`finance_other_costs_and_sponsors`), the plc's other income, the ground
/// maintenance (`finance_ground_maintenance`), the finance events of the month (`FinanceTable::process_club_finance_events`
/// with 1) and the month rollover: the month block (`finance::MONTH_PLAYERS_BOUGHT`, 0x50 bytes) is copied to last
/// month's (+0x20) and zeroed.
unsafe fn finance_month_end(this: usize, club: Club, fin: Finance) {
    let mut b_team: u32 = 0;
    if exe::club_get_linked_b_team(club, &mut b_team, 1) != 0 && b_team == 0 {
        return;
    }
    let rep = club.reputation() as i32;
    if get_wealth_band(fin.addr()) == 0xfe && exe::utils_random(rep) > exe::utils_random(20000) {
        finance_crisis_unrest(club);
    }
    let stadium = club.stadium();
    if !stadium.is_null() {
        let mut cost = if stadium.covered() != 0 { 12500 } else { 0 };
        if stadium.under_soil_heating() != 0 {
            cost += if rep > 4500 { 12500 } else { 7500 };
        }
        if cost != 0 {
            finance_add_ground_cost(fin, cost);
        }
    }
    let t = club.training() as i32;
    let training = if t < 6 {
        t.wrapping_mul(400)
    } else if t < 16 {
        (t - 2).wrapping_mul(1000)
    } else if t >= 20 && rep >= 6250 {
        40000
    } else {
        (t - 12).wrapping_mul(4000)
    };
    finance_add_ground_cost(fin, training);
    let division_id = read_i32(club.division().addr()) as usize;
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + division_id * 4) == 0 {
        let v = ftol(rep.wrapping_mul(club.attendance()) as f64 * read_f64(rdata::F64_0_005));
        add_to_i32(fin.addr() + finance::SEASON_GATE_RECEIPTS, v);
        add_to_i32(fin.addr() + finance::MONTH_GATE_RECEIPTS, v);
        add_i32_to_balance(fin.addr(), v);
    }
    finance_bank_interest(club, fin);
    finance_other_costs_and_sponsors(club, fin);
    if club.plc() != 0 {
        let x = (exe::utils_random(100) + 1) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_75);
        let v = finance_reputation_income(rep, x, 5000);
        add_to_i32(fin.addr() + finance::MONTH_OTHER_INCOME, v);
        add_to_i32(fin.addr() + finance::SEASON_OTHER_INCOME, v);
        add_i32_to_balance(fin.addr(), v);
    }
    finance_ground_maintenance(club, fin);
    process_club_finance_events(this, club.addr(), 1);
    let block = fin.addr() + finance::MONTH_PLAYERS_BOUGHT;
    core::ptr::copy(block as *const u8, (fin.addr() + 0x20) as *mut u8, 0x50); // +0x20: last month's block
    core::ptr::write_bytes(block as *mut u8, 0, 0x50);
}

/// A ground cost: added to the season's and the month's ground costs and taken from the balance.
unsafe fn finance_add_ground_cost(fin: Finance, v: i32) {
    add_to_i32(fin.addr() + finance::SEASON_GROUND_COSTS, v);
    add_to_i32(fin.addr() + finance::MONTH_GROUND_COSTS, v);
    sub_i32_from_balance(fin, v);
}

/// An income by the reputation and a factor `x`: with the reputation above `above`, reputation × x; else
/// x × (reputation² × 0.0002 + 50), truncated.
unsafe fn finance_reputation_income(rep: i32, x: f64, above: i32) -> i32 {
    if rep > above {
        ftol(rep as f64 * x)
    } else {
        let r = rep as f64;
        ftol(x * (r * r * read_f64(rdata::F64_0_0002) + read_f64(rdata::F64_50)))
    }
}

/// The unrest of a club in crisis (wealth band -2, won by random(reputation) over random(20000)): the media post the
/// financial trouble news and each of the 50 squad slots whose player, under contract with the club, is not on loan
/// (`contract::SQUAD_STATUS` high nibble 0x70) nor listed may ask to leave: the weight starts at 10 (15 under 21, 13
/// under 24, 11 over 32; from 24 to 32, 5 for a first team, rotation or backup player of reputation above 3250 under
/// 4750 or under the club's), + 21 - loyalty and, when the manager dislikes him, + 20 - professionalism, with ambition
/// above 15; + loyalty × -0.5 when the manager favours him; - 3 with reputation above 5750 and ambition under 10;
/// - pressure (all in 16 bits); random(weight) over random(20) sets the contract's event flag 0x400000.
unsafe fn finance_crisis_unrest(club: Club) {
    post_financial_trouble_news(read_usize(G_MEDIA), club.addr());
    for i in 0..50 {
        let player = Staff(club.squad(i));
        if player.is_null() || player.player().is_null() {
            continue;
        }
        let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
        if contract.is_null() || contract.squad_status() & 0xf0 == 0x70 || exe::staff_is_transfer_listed_or_requested(player, club) != 0 {
            continue;
        }
        let age = read_i8(player.addr() + staff::AGE); // the original reads one byte
        let reputation = player.player().current_reputation();
        let mut w: i16 = if age < 21 {
            15
        } else if age < 24 {
            13
        } else if age > 32 {
            11
        } else if matches!(contract.squad_status() & 0xf0, 0x10 | 0x20 | 0x30) && reputation > 3250 && (reputation < 4750 || reputation < club.reputation()) {
            5
        } else {
            10
        };
        let manager = read_usize(club.addr() + club::MANAGER);
        if player.ambition() > 15 {
            w = w.wrapping_add(21 - player.loyalty() as i16);
            if exe::staff_is_disliked_by(manager, player) != 0 {
                w = w.wrapping_add(20 - player.professionalism() as i16);
            }
        }
        if exe::staff_is_favoured_by(manager, player) != 0 {
            w = w.wrapping_add(ftol(player.loyalty() as f64 * read_f64(rdata::F64_NEG_0_5)) as i16);
        }
        if reputation > 5750 && player.ambition() < 10 {
            w = w.wrapping_sub(3);
        }
        w = w.wrapping_sub(player.pressure() as i16);
        if exe::utils_random(w as i32) > exe::utils_random(20) {
            covered!("sbA_1 crisis unrest");
            exe::staff_contract_apply_event_flags(contract, 0x400000, 0);
        }
    }
}

/// The month's bank interest: a positive balance earns 0.25% (into +0xb0 and +0x150, the interest received); a
/// negative one or 0 pays 1.2% above -1000 × reputation, 1.8% above -2500 × reputation, else 2.4% (into +0x84 and
/// +0x124, the interest paid), truncated.
unsafe fn finance_bank_interest(club: Club, fin: Finance) {
    let b = read_i64(fin.addr() + finance::BALANCE);
    if b > 0 {
        let v = ftol(b as f64 * read_f64(rdata::F64_0_0025));
        add_to_i32(fin.addr() + 0x150, v); // season interest received
        add_to_i32(fin.addr() + 0xb0, v); // month interest received
        add_i32_to_balance(fin.addr(), v);
        return;
    }
    let rep = club.reputation() as i32;
    let rate = if b > rep.wrapping_mul(-1000) as i64 {
        0x96a718 // 0.012
    } else if b > rep.wrapping_mul(-2500) as i64 {
        0x96a710 // 0.018
    } else {
        0x96a708 // 0.024
    };
    let v = ftol(-(b as f64 * read_f64(rate)));
    add_to_i32(fin.addr() + 0x84, v); // month interest paid
    add_to_i32(fin.addr() + 0x124, v); // season interest paid
    sub_i32_from_balance(fin, v);
}

/// The board's other costs and the sponsorship of the month. A factor f (f32) = random(n) + 1, × 0.01 + 1.5, with n =
/// (31 - the chairman's business) × 10 (150 without a chairman with a non-player record), doubled when random(business
/// + 1) (random(11)) is 0: the costs are (reputation + 125) × f for a professional club (status 1), else (reputation +
/// 50) × f × 0.25, into the month's and season's other costs (+0x80, +0x120), out of the balance. Then x = random(m) +
/// 1, × 0.01 + 1.5, with m = business × 10 (100): the sponsorship is (reputation + 50) × x for a professional club of
/// reputation 5000 or more, else x × (reputation² × 0.0002 + 50), into +0xa4 and +0x144, onto the balance.
unsafe fn finance_other_costs_and_sponsors(club: Club, fin: Finance) {
    let rep = club.reputation() as i32;
    let chairman = club.chairman();
    let business = if !chairman.is_null() && !chairman.non_player().is_null() { Some(chairman.non_player().business() as i8 as i32) } else { None };
    let (n, odds) = match business {
        Some(b) => ((31 - b) * 10, b + 1),
        None => (150, 11),
    };
    let f = ((exe::utils_random(n) + 1) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_1_5)) as f32;
    let f = if exe::utils_random(odds) == 0 { f as f64 + f as f64 } else { f as f64 };
    let costs = if club.professional_status() == 1 {
        ftol((rep + 125) as f64 * f)
    } else {
        ftol((rep + 50) as f64 * f * read_f64(rdata::F64_0_25))
    };
    add_to_i32(fin.addr() + finance::MONTH_OTHER_COSTS, costs);
    add_to_i32(fin.addr() + finance::SEASON_OTHER_COSTS, costs);
    sub_i32_from_balance(fin, costs);
    let m = business.map_or(100, |b| b * 10);
    let x = (exe::utils_random(m) + 1) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_1_5);
    let sponsors = if club.professional_status() == 1 && rep >= 5000 {
        ftol((rep + 50) as f64 * x)
    } else {
        finance_reputation_income(rep, x, i32::MAX)
    };
    add_to_i32(fin.addr() + 0xa4, sponsors); // month sponsorship
    add_to_i32(fin.addr() + 0x144, sponsors); // season sponsorship
    add_i32_to_balance(fin.addr(), sponsors);
}

/// The stadium's capacity factor of a professional club that owns its ground, by the financial state
/// (`finance::CRISIS_STATE`) and the capacity steps 70000, 50000, 27500, 21000 and 10000 (0 = None: the capacity as it
/// is).
unsafe fn finance_ground_factor(state: u8, capacity: i32) -> Option<usize> {
    let step = [70000, 50000, 27500, 21000, 10000].iter().position(|&s| capacity > s).unwrap_or(5);
    let row: [usize; 6] = match state {
        0 => [0x96a700, 0x968fa8, 0x96a6f8, 0x96a5a8, 0, 0x96a678], // 1.425 1.375 1.325 1.125 (1) 0.875
        1 => [0x96a6f8, 0x96a6f0, 0x96a6e0, 0x96a6d8, 0x969520, 0x96a6a0], // 1.325 1.275 1.225 0.975 0.85 0.725
        _ => [0x968fa8, 0x96a6f8, 0x96a6f0, 0x968fa0, 0x9689d8, 0x96a6e8], // 1.375 1.325 1.275 1.025 0.9 0.775
    };
    if row[step] == 0 { None } else { Some(row[step]) }
}

/// The month's ground maintenance, x × A truncated, into the month's and season's ground costs, out of the balance.
/// A professional club (status 1): a ground it does not own costs capacity × 1.5, no ground 2500; its own ground the
/// capacity × the factor (`finance_ground_factor`), × 0.8 in Germany (`KEY_NATION_GERMANY`), × 1.35, 1.25, 1.15 or 1.05
/// with reputation above 7250, 5750, 4250 or 1250; x = (random(25) + 1) × 0.01 + 0.75. Any other: a ground it does
/// not own costs its capacity, its own the capacity × 0.45 in crisis (state 1) or × 0.5, no ground 1000; above 750 it
/// becomes 750 - (A - 750) × -0.1 truncated; x = (random(25) + 1) × 0.01 + 0.6.
unsafe fn finance_ground_maintenance(club: Club, fin: Finance) {
    let stadium = club.stadium();
    let rep = club.reputation() as i32;
    let state = fin.crisis_state();
    let (a, base) = if club.professional_status() == 1 {
        let a = if club.own_stadium() == 0 && !stadium.is_null() {
            ftol(stadium.capacity() as f64 * read_f64(rdata::F64_1_5))
        } else if stadium.is_null() {
            2500
        } else {
            let cap = stadium.capacity();
            let mut a = match finance_ground_factor(state, cap) {
                Some(f) => ftol(cap as f64 * read_f64(f)),
                None => cap,
            };
            let nation = club.nation();
            if !nation.is_null() && read_i32(nation.addr()) == read_i32(KEY_NATION_GERMANY) {
                a = ftol(a as f64 * read_f64(rdata::F64_0_8));
            }
            let tier = if rep > 7250 {
                rdata::F64_1_35
            } else if rep > 5750 {
                rdata::F64_1_25
            } else if rep > 4250 {
                rdata::F64_1_15
            } else if rep > 1250 {
                rdata::F64_1_05
            } else {
                0
            };
            if tier != 0 {
                a = ftol(a as f64 * read_f64(tier));
            }
            a
        };
        (a, rdata::F64_0_75)
    } else {
        let mut a = if club.own_stadium() == 0 && !stadium.is_null() {
            stadium.capacity()
        } else if !stadium.is_null() {
            ftol(stadium.capacity() as f64 * read_f64(if state == 1 { rdata::F64_0_45 } else { rdata::F64_0_5 }))
        } else {
            1000
        };
        if a > 750 {
            a = 750 - ftol((a - 750) as f64 * read_f64(rdata::F64_NEG_0_1));
        }
        (a, rdata::F64_0_6)
    };
    let x = (exe::utils_random(25) + 1) as f64 * read_f64(rdata::F64_0_01) + read_f64(base);
    finance_add_ground_cost(fin, ftol(x * a as f64));
}

/// A day that is not a month end: a club in crisis or in administration (`finance::CRISIS_STATE` 1 or 2) runs its
/// finance events, weekly with a chance of 1 in max((11000 - reputation) / 1000, 10)
/// (`FinanceTable::process_club_finance_events`). A club below 4250 of reputation in a division without a competition
/// object (id from 1 to the club competition count - 2) is left out.
unsafe fn finance_daily_events(this: usize, club: Club, fin: Finance) {
    let comps = read_usize(G_COMP_OBJECT_TABLE);
    let id = read_i32(club.division().addr());
    if comps != 0 && id > 0 && id < read_i32(G_CLUB_COMP_COUNT) - 1 && club.reputation() < 4250 && read_usize(comps + id as usize * 4) == 0 {
        return;
    }
    if matches!(fin.crisis_state(), 1 | 2) {
        let n = ((11000 - club.reputation() as i32) / 1000).max(10);
        let week = (exe::utils_random(n) == 0) as u32;
        process_club_finance_events(this, club.addr(), week);
    }
}

/// Comparer for `finance_daily_update`: only the original runs (the pass sends news, fixes contracts and runs every
/// club's finance events). On a month end it checks what no later call undoes: the last club that the month pass
/// reaches has its month block (`finance::MONTH_PLAYERS_BOUGHT`, 0x50 bytes) zeroed.
pub(crate) unsafe extern "thiscall" fn compare_finance_daily_update(this: usize, today: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let (start_day, start_year) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
    let (day, year) = (read_i16(today), read_u16(today + 2));
    let mut tomorrow = [0u16; 4];
    let next = exe::date_add_days(today, &mut tomorrow as *mut [u16; 4] as usize, 1);
    if start_year > year || (start_year == year && start_day > day) || exe::date_get_month(today) == exe::date_get_month(next) {
        return o(this, today);
    }
    let mut last = None;
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        let club = club_at(k as isize);
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let mut b_team: u32 = 0;
        if club_has_finances(club) && !(exe::club_get_linked_b_team(club, &mut b_team, 1) != 0 && b_team == 0) {
            last = Some(k);
        }
    }
    o(this, today);
    if let Some(k) = last {
        let block = finance_at(this, k).addr() + finance::MONTH_PLAYERS_BOUGHT;
        let zeroed = core::slice::from_raw_parts(block as *const u8, 0x50).iter().all(|&x| x == 0);
        SCOREBOARD.record(zeroed, || format!("finance_daily_update(day {}, year {}) club {}: the month block was not zeroed", day, year, k));
    }
}
