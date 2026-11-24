// ---------- sbA_2: a club's starting finances, the finance table and the league's ground upgrade (fin_prm.cpp, finance.CPP) ----------

/// `finance_post_ground_upgrade_plans_news(club id, competition, kind)` (0x5a0410, thiscall on the finance table, ret
/// 0xc): the manager's news that the board plans the ground works a competition requires.
const FN_FINANCE_POST_GROUND_UPGRADE_PLANS_NEWS: usize = 0x5a0410;

/// The starting balance of a club by its reputation, from the table at 0x9c8290 (21 doubles, one per 500 of
/// reputation, from -450000 to 27500000): row reputation / 500 - 1, at least 0.
unsafe fn starting_balance_base(reputation: i16) -> f64 {
    let row = ((reputation as i32 / 500) as u8).wrapping_sub(1) as i8;
    read_f64(0x9c8290 + row.max(0) as usize * 8)
}

/// Sets up a club's finance record at the start of the game.
///
/// The club's id goes to `finance::CLUB_ID`; a national team gets a zero balance, highest wage, wage bill and
/// instalments and nothing else. A semi-professional club (status 3) whose division or reserve division has a
/// competition object becomes status 2. The season ticket ratio starts at 0.1. The balance: with cash
/// (`club::CASH`) under 1.5 × reputation², or for Rushden (the name's first 6 letters), the cash + random(cash / 10) -
/// random(cash / 10); otherwise the table by reputation (`starting_balance_base`; 0 without a nation) and, when
/// positive, league standard² / 19 × base × 0.05 / the state of development (from 1 to 3), at most reputation² × 0.5,
/// a third for status 2 and a fifth for status 3; a negative one becomes 50000 on random(2); then + random(B × 0.2) -
/// random(B × 0.2); in South Korea above 250000 the excess is halved. The squeeze counter starts at 10 without an own
/// ground or in wealth band 0, 5 in bands 1 and 2, else 15; the crisis state at 0; every month, last month, season and
/// last season total (+0x20 to +0x160) at 0; the season tickets at 0 with the sales step at 3. The wage ceiling: only
/// in England, Italy and Spain, reputation above 4750 and wealth band 1 or 2: v = 2 × reputation - 4750 (band 2),
/// 2 × reputation - 5250 above 5250 or reputation + 250 (band 1), in 16 bits; a plc (`club::PLC`) gets (v - 1250) × 6
/// under 5750, else v × 8 - 10000 in band 1 or (v - 1250) × 10; any other the same from 750 and 6000; else 0.
/// Without a club: the error box (finance.CPP 0x8e) and the game ends.
/// Original: `Finance::init_for_club` (0x5961d0, fin_prm.cpp (finance.CPP), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn init_for_club(this: usize, club: usize) {
    let (fin, club) = (Finance(this), Club(club));
    if club.is_null() {
        fatal_error_at(0x9c8784, 0x8e);
    }
    fin.set_club_id(read_usize(club.addr()));
    if exe::database_is_national_team(club) != 0 {
        write_i64(fin.addr() + finance::BALANCE, 0);
        fin.set_highest_wage(0);
        fin.set_wage_bill(0);
        fin.set_instalment_balance(0);
        return;
    }
    let comps = read_usize(G_COMP_OBJECT_TABLE);
    let has_object = |c: Competition| !c.is_null() && comps != 0 && read_usize(comps + read_i32(c.addr()) as usize * 4) != 0;
    if club.professional_status() == 3 && (has_object(club.division()) || has_object(club.reserve_division())) {
        club.set_professional_status(2);
    }
    fin.set_season_ticket_ratio(0.1);
    let balance = starting_balance(fin, club);
    write_i64(fin.addr() + finance::BALANCE, ftol_i64(balance));
    let band = get_wealth_band(fin.addr()) as i8;
    fin.set_squeeze_counter(match band {
        _ if club.own_stadium() == 0 => 10,
        0 => 10,
        1 | 2 => 5,
        _ => 15,
    });
    fin.set_crisis_state(0);
    core::ptr::write_bytes((fin.addr() + 0x20) as *mut u8, 0, 0x140); // last month, month, season and last season totals
    fin.set_season_tickets_sold(0);
    fin.set_season_ticket_step(3);
    fin.set_instalment_balance(0);
    fin.set_wage_ceiling(starting_wage_ceiling(club, band));
}

/// The starting balance (before truncation): see `init_for_club`.
unsafe fn starting_balance(fin: Finance, club: Club) -> f64 {
    let cash = club.cash();
    let rep = club.reputation() as f64;
    let from_cash = |cash: i32| {
        let a = exe::utils_random(cash / 10);
        let b = exe::utils_random(cash / 10);
        b.wrapping_sub(a) as f64 + cash as f64
    };
    if cash != 0 && (cash as f64) < rep * rep * read_f64(rdata::F64_1_5) {
        return from_cash(cash);
    }
    if exe::crt_strncmp(club.addr() + club::NAME, 0x9c877c, 6) == 0 { // "Rushden"
        covered!("sbA_2 Rushden starting balance");
        return from_cash(cash);
    }
    let nation = club.nation();
    let mut b = 0.0;
    if !nation.is_null() {
        b = starting_balance_base(club.reputation());
        if b > 0.0 {
            let n = (nation.state_of_development() as i8 as f64).clamp(read_f64(rdata::F64_1), read_f64(rdata::F64_3));
            let standard = nation.league_standard() as i8 as i32;
            b = (standard.wrapping_mul(standard) as f64 * read_f64(rdata::F64_0_05263157895)) * (b * read_f64(rdata::F64_0_05)) / n;
            let own = club_at(fin.club_id() as isize).reputation() as i32;
            let most = ftol(own.wrapping_mul(own) as f64 * read_f64(rdata::F64_0_5)) as f64;
            if b > most {
                b = most;
            }
            match club.professional_status() as i8 {
                2 => b *= read_f64(rdata::F64_0_3333333333),
                3 => b *= read_f64(rdata::F64_0_2),
                _ => {}
            }
        }
        if b < 0.0 && exe::utils_random(2) != 0 {
            b = 50000.0;
        }
    }
    let spread = ftol(b * read_f64(rdata::F64_0_2));
    let r1 = exe::utils_random(spread);
    let r2 = exe::utils_random(spread);
    let mut v = r1.wrapping_sub(r2) as f64 + b;
    if !nation.is_null() && read_i32(nation.addr()) == read_i32(KEY_NATION_SOUTH_KOREA) && v > read_f64(rdata::F64_250000) {
        covered!("sbA_2 South Korea balance halved");
        let top = read_f64(rdata::F64_250000);
        v = ftol((v - top) * read_f64(rdata::F64_0_5) + top) as f64;
    }
    v
}

/// The starting wage ceiling: see `init_for_club`.
unsafe fn starting_wage_ceiling(club: Club, band: i8) -> i32 {
    let nation = club.nation();
    if nation.is_null() {
        return 0;
    }
    let id = read_i32(nation.addr());
    if id != read_i32(KEY_NATION_ENGLAND) && id != read_i32(KEY_NATION_ITALY) && id != read_i32(KEY_NATION_SPAIN) {
        return 0;
    }
    let rep = club.reputation();
    if rep <= 4750 || !matches!(band, 1 | 2) {
        return 0;
    }
    let v = if band == 2 {
        rep.wrapping_mul(2).wrapping_sub(4750)
    } else if rep > 5250 {
        rep.wrapping_mul(2).wrapping_sub(5250)
    } else {
        rep.wrapping_add(250)
    } as i32;
    let (low, high) = if club.plc() != 0 { (1250, 10000) } else { (750, 6000) };
    if v < 5750 {
        (v - low) * 6
    } else if band == 1 {
        v * 8 - high
    } else {
        (v - low) * 10
    }
}

/// Builds the finance table: after the CD check (`find_game_cd`; the no-CD patch of the running exe is honoured), one
/// finance record per club (`operator new`, the count in front, `FINANCE_SIZE` bytes each); without memory the error
/// box (finance.CPP 0xf1d) and the table stays empty. From a save (`save` not 0) the records are loaded
/// (`FinanceTable::load_from_save`); when that fails, or for a new game, each club's record is set up
/// (`Finance::init_for_club`), in club order. Returns `this`.
/// Original: `FinanceTable::FinanceTable` (0x59a370, fin_prm.cpp (finance.CPP), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_finance_table(this: usize, save: usize) -> usize {
    find_game_cd(0x59a3c3, 0x59a406);
    let count = read_i32(G_CLUB_COUNT);
    let mem = exe::crt_operator_new((count as u32).wrapping_mul(FINANCE_SIZE as u32).wrapping_add(4));
    let mut table = 0;
    if mem != 0 {
        write_i32(mem, count);
        table = mem + 4;
        exe::crt_vector_constructor_iterator(table, FINANCE_SIZE as u32, count as u32, 0x5961c0, FN_SHARED_NOOP_RET0_5A8F60); // Finance::Finance, the empty destructor
    }
    write_usize(this, table);
    if table == 0 {
        show_error_at(0x9c8784, 0xf1d);
        return this;
    }
    if save != 0 && load_from_save(this, save) != 0 {
        return this;
    }
    for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
        init_for_club(table + k * FINANCE_SIZE, club_at(k as isize).addr());
    }
    this
}

/// Raises a club's ground to a competition's minimum seats and capacity; returns 1 when done (or nothing to do), 0
/// when not done.
///
/// No national team, no stadium: 0. The seats to add are the minimum seats - the seating (`stadium::SEATING_CAPACITY`),
/// at least 0. An all-seater requirement (minimum seats = minimum capacity) adds also the standing places over the
/// minimum (capacity - seating, when the capacity is above the minimum) and no capacity; otherwise the capacity to add
/// is the minimum capacity - the capacity - the seats, at least 0. Nothing to add: 1. When the seats to add pass the
/// seating, the converted places are the excess, at most the standing places. The cost: (6000 × (seats / 1000 + 1) +
/// seats) × 125 + 75 × converted (16 bits) + 50 × capacity.
/// When optional, a ground above 120000 is not touched (0). The expansion capacity (`stadium::EXPANSION_CAPACITY`)
/// grows to at least capacity + capacity to add + seats. When optional: with a chairman with a non-player record, a
/// cost not under the balance must be under 500000 × his resources (`.resources()`), else his resources grow by 1 (up
/// to 20) and nothing more is done (0); without one, a cost not under the balance and of 1000000 or more is refused on
/// random(5) not 0. Done: the capacity grows by capacity to add + seats and the seating by seats + converted (16 bits); with the chairman, random(resources) = 0 with a cost above the
/// balance and under 500000 × resources makes the board pay it (other income, month and season, and the balance); the
/// cost goes to the month's and season's ground costs and out of the balance; the manager gets the plans news (kind 0
/// when only converting places, else 1), and an all-seater requirement makes the seating the whole capacity. The
/// mandatory way does it without asking.
/// Original: `Finance::upgrade_stadium_for_league` (0x599e00, fin_prm.cpp (finance.CPP), thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn upgrade_stadium_for_league(this: usize, min_seats: i32, min_capacity: i32, comp: u32, optional: u8) -> u8 {
    let fin = Finance(this);
    let club = club_at(fin.club_id() as isize);
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    let stadium = club.stadium();
    if stadium.is_null() {
        return 0;
    }
    let seating = stadium.seating_capacity();
    let capacity = stadium.capacity();
    let mut seats = (min_seats - seating).max(0);
    let mut more = 0;
    if min_seats == min_capacity {
        if capacity > min_seats && capacity - seating > 0 {
            seats += capacity - seating;
        }
    } else {
        more = (min_capacity - capacity - seats).max(0);
    }
    if seats == 0 && more == 0 {
        return 1;
    }
    let mut converted = 0;
    if seats > seating && capacity - seating > 0 {
        converted = (seats - seating).min(capacity - seating);
    }
    if optional != 0 && capacity > 120000 {
        return 0;
    }
    let target = capacity.wrapping_add(more).wrapping_add(seats);
    if stadium.expansion_capacity() < target {
        stadium.set_expansion_capacity(target);
    }
    let cost = (6000i32.wrapping_mul(seats / 1000 + 1).wrapping_add(seats)).wrapping_mul(125)
        .wrapping_add(75 * converted as i16 as i32)
        .wrapping_add(more.wrapping_mul(50));
    let balance = read_i64(fin.addr() + finance::BALANCE);
    if optional != 0 {
        let chairman = club.chairman();
        let np = if !chairman.is_null() { chairman.non_player() } else { NonPlayer(0) };
        if !np.is_null() {
            if cost as i64 >= balance && cost >= 500000 * np.resources() as i32 {
                if np.resources() < 20 {
                    np.set_resources(np.resources() + 1);
                }
                return 0;
            }
        } else if cost as i64 >= balance && cost >= 1_000_000 && exe::utils_random(5) != 0 {
            return 0;
        }
        grow_stadium(stadium, more, seats, converted);
        if !np.is_null() && exe::utils_random(np.resources() as i32) == 0 && cost as i64 > balance && cost < 500000 * np.resources() as i32 {
            covered!("sbA_2 board pays the ground works");
            add_to_i32(fin.addr() + finance::SEASON_OTHER_INCOME, cost);
            add_to_i32(fin.addr() + finance::MONTH_OTHER_INCOME, cost);
            add_i32_to_balance(fin.addr(), cost);
        }
    } else {
        grow_stadium(stadium, more, seats, converted);
    }
    add_to_i32(fin.addr() + finance::MONTH_GROUND_COSTS, cost);
    add_to_i32(fin.addr() + finance::SEASON_GROUND_COSTS, cost);
    add_i32_to_balance(fin.addr(), cost.wrapping_neg());
    let kind = if converted as i16 != 0 && more == 0 { 0 } else { 1 };
    let news: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_FINANCE_POST_GROUND_UPGRADE_PLANS_NEWS);
    news(read_usize(G_FINANCE_TABLE), fin.club_id() as u32, comp, kind);
    if min_seats == min_capacity {
        let s = club.stadium();
        s.set_seating_capacity(s.capacity());
    }
    1
}

/// The ground works: the capacity grows by capacity + seats and the seating by seats + converted (each sum in 16
/// bits, sign-extended).
unsafe fn grow_stadium(stadium: Stadium, more: i32, seats: i32, converted: i32) {
    stadium.set_capacity(stadium.capacity().wrapping_add(more.wrapping_add(seats) as i16 as i32));
    stadium.set_seating_capacity(stadium.seating_capacity().wrapping_add(seats.wrapping_add(converted) as i16 as i32));
}

/// Comparer for `init_for_club`: side by side on the finance record, the club's professional status and the random
/// state (the copy without counting the draws). Without a club only the original runs (it ends the game).
pub(crate) unsafe extern "thiscall" fn compare_init_for_club(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    if club == 0 {
        return o(this, club);
    }
    let mut regions = vec![(this, FINANCE_SIZE), (club + club::PROFESSIONAL_STATUS, 1)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club), || crate::rng::without_counting(|| init_for_club(this, club)));
    SCOREBOARD.record(equal, || format!("init_for_club(fin {:#x}, club {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Comparer for `new_finance_table`: only the original runs (it allocates the table and draws each club's balance);
/// it checks the table it leaves: one record per club, the count in front.
pub(crate) unsafe extern "thiscall" fn compare_new_finance_table(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, save);
    let table = read_usize(this);
    SCOREBOARD.record(r == this && table != 0 && read_i32(table - 4) == read_i32(G_CLUB_COUNT),
        || format!("new_finance_table(save {:#x}): table {:#x}, count {}", save, table, if table != 0 { read_i32(table - 4) } else { -1 }));
    r
}

/// Comparer for `upgrade_stadium_for_league`: side by side on the finance record, the stadium, the chairman's
/// resources and the random state (the copy without counting the draws), with the result. The plans news goes only to
/// a human manager: with one (or a national team, or no stadium) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_upgrade_stadium_for_league(this: usize, min_seats: i32, min_capacity: i32, comp: u32, optional: u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u8) -> u8>(%%I2%%);
    let club = club_at(Finance(this).club_id() as isize);
    let stadium = club.stadium();
    if stadium.is_null() || exe::database_is_national_team(club) != 0 || fin_prm_has_human_manager(club) {
        return o(this, min_seats, min_capacity, comp, optional);
    }
    let mut regions = vec![(this, FINANCE_SIZE), (stadium.addr(), stadium::SIZE)];
    let chairman = club.chairman();
    if !chairman.is_null() && !chairman.non_player().is_null() {
        regions.push((chairman.non_player().addr() + non_player::RESOURCES, 1));
    }
    regions.extend(crate::rng::STATE_REGIONS);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, min_seats, min_capacity, comp, optional),
        || crate::rng::without_counting(|| upgrade_stadium_for_league(this, min_seats, min_capacity, comp, optional)));
    SCOREBOARD.record(equal && ro == rr, || format!("upgrade_stadium_for_league(fin {:#x}, {}, {}, comp {}, optional {}) original={} {:x?} rust={} {:x?}",
        this, min_seats, min_capacity, comp, optional, ro, mo, rr, mr));
    ro
}
