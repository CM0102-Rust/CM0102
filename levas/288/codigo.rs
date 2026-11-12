// ---------- batch 288: the money side of the market: the club's transfer budget and the asking price of a human club's player (fin_prm.cpp, contract_manager.cpp) ----------

/// _ftol (0x945640) keeping the whole 64-bit result (edx:eax), for the callers that use both halves; out of range or
/// NaN gives the x87 integer indefinite.
pub(crate) fn ftol_i64(x: f64) -> i64 {
    if x.is_finite() && x.abs() < 9.223372036854775808e18 { x as i64 } else { i64::MIN }
}

/// The rows of the two temporary lists the transfer budget reads (allocated by the callee, freed by the budget)
/// [inference: from their use here and in TRANSFER_MANAGER::get_club_future_transfer_list 0x8ccb80 and
/// CONTRACT_MANAGER::get_current_contract_offers 0x4ca300].
#[allow(dead_code)]
pub(crate) mod budget_list_rows {
    pub(crate) const FUTURE_SIZE: usize = 0x1b; // a future-transfer row (27 bytes)
    pub(crate) const FUTURE_PERSON_ID: usize = 0x8; // the person's id
    pub(crate) const FUTURE_OUTGOING: usize = 0x10; // nonzero when the club sells the person [inference]
    pub(crate) const FUTURE_FEE: usize = 0x14; // the fee
    pub(crate) const FUTURE_STAGE: usize = 0x19; // the stage (1..6) taken from the offer state; 3 to 6 count
    pub(crate) const OFFER_SIZE: usize = 0x6f; // a copy of a contract offer (111 bytes)
    pub(crate) const OFFER_PERSON_ID: usize = 0x0; // the person's id
    pub(crate) const OFFER_FEE: usize = 0x8; // the fee attached to the offer (the terms' +8) [inference]
    pub(crate) const OFFER_WAGE: usize = 0xc; // the offered wage (the terms' contract::WAGE)
    pub(crate) const OFFER_STATE: usize = 0x50; // contract_offer::STATE: 4 = closed with the transfer offer
}

/// The club's transfer budget: how much it can spend now, as an i64 (the bank balance minus the reserves the board
/// keeps, with floors and caps by reputation, nation and the human manager's knowledge of the player).
///
/// Called by the computer's bids (`TRANSFER_OFFER::build_bid` 0x8e5d60 and its neighbours), the shortlists (budget per
/// open slot), the board (`manager_manager.cpp`, funds requests, the veto), the finance events and the screens: 37 call
/// sites, every day of the window for every computer club that bids. In this order:
/// 1. The start is the balance (`.balance()`); with `add_listed` the value (`.value()`) of each of the 50 squad places
///    (`.squad(k)`) whose person is transfer listed (`exe::staff_is_transfer_listed`) is added.
/// 2. `adjusted` (when given) = 0.
/// 3. With `pending` and a human club (`exe::human_manager_is_human_club` with 1, 1): the club's future transfers
///    (TRANSFER_MANAGER::get_club_future_transfer_list 0x8ccb80, rows of `budget_list_rows`): a purchase adds its fee
///    to the money committed (except the `player` himself); a sale in stage 3 to 6 takes its fee off the committed
///    money (once per person) and the wage of the person's contract at the club (`exe::contract_manager_get_contract_for_club`)
///    off the wages. Defect of the original: after a repeated person it reads the contract of the row after the first
///    match, not of this row. Then the open contract offers (CONTRACT_MANAGER::get_current_contract_offers 0x4ca300,
///    except the closed ones, state 4): each offer adds its wage and its fee; a person already contracted to the club
///    (main contract `exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) takes his current wage
///    off. Both lists are freed (`exe::crt_free`).
/// 4. A club without a league (`.division()`, or one without its competition object in `G_COMP_OBJECT_TABLE`) returns
///    the start.
/// 5. The board's multiplier m: the chairman's (`.chairman()`) `.business()` + 6, minus (`.ambition()` − 15) when the
///    ambition is above 15; 16 without a chairman or his sheet; +5 for a German club (`KEY_NATION_GERMANY`).
/// 6. The reserve: m × (weekly wage bill `.wage_bill()` + the wages of step 3 − ⅔ × `.instalment_balance()`) comes off.
/// 7. The ground (`.stadium()`, `.capacity()` × m): owned (`.own_stadium()`), by reputation (`.reputation()`) above
///    5750, 4250 or (professional club, `.professional_status()` 1) 1250, 1.25, 1.15 or 1.05 × that comes off, else
///    1 ×; rented, 2 ×; no ground, 2500 × m.
/// 8. Training (`.training()` t): 200·t, 500·(t − 2) from 6 or 2000·(t − 12) from 16, × m × 0.25, comes off.
/// 9. Reputation r: r × m comes off; attendance (`.attendance()`) × r × m × 0.0005 (0.001 from 4000) comes in.
/// 10. With a nation: the league's reputation (`.division()` `.reputation()`) squared × 5000 (England, Italy, Spain),
///     2500 (Germany, France) or 750 comes in; r² × m × 0.001 (own ground) or 0.0005 comes in.
/// 11. In a continental competition (`.continental_comp_id()` ≠ −1) and professional: 300·r × 1 (England, Germany,
///     Italy, France, Scotland, Spain), 0.65 (Holland, Sweden, Belgium, Portugal, Turkey), 0.35 (Denmark, Norway) or
///     0.5 (others); 0 without a nation.
/// 12. The committed money S = max(fees, 0) + max(0, net season spending (`.season_players_bought()` −
///     `.season_players_sold()`, low dword) − 8 million) comes off. If still positive: below 10% of the balance it
///     becomes that 10% (at most 2.5 million); otherwise the soft cap Z = r² × 0.5 (× q² when the manager's
///     (`.manager()`) `.current_reputation()` / r = q is below 1, q at least 0.5), at least the balance − 10 million
///     (− 15 million in Germany or without a nation) and, at a PLC (`.plc()`), without a nation or in Sweden, Norway,
///     the USA, Portugal, Denmark, Finland, Scotland, Turkey or Germany, at most 40 (35) million − S; at least 50000;
///     above Z only 10% of the excess is kept.
/// 13. The funds the board granted (`G_MANAGER_MANAGER` row, `.extra_funds_granted()`) come in.
/// 14. Above reputation 5000, at least (200·r − 1 million) × league reputation / 17 (then at least 50000); below 3500,
///     at most 500·r (professional) or 250·r; never negative.
/// 15. By nation: Sweden, Norway, Denmark, Wales, the USA, Finland, both Irelands and Argentina halve it and keep 10%
///     above 2.5 million; Belgium and Portugal keep 25% above 15 million; Brazil 25% above 2.5 million; Germany,
///     France, Scotland and Holland 25% above 17.5 million.
/// 16. With `player`, `fog`, attribute masking (`G_ATTRIBUTE_MASKING_ENABLED`), the fog of war (`G_FOG_OF_WAR`) and a
///     human club (with 0, 1): the manager's knowledge of the player (`exe::fog_of_war_get_manager_map`,
///     `exe::fog_of_war_map_get_knowledge_level`) sets V: level 3 leaves the budget as it is; 2: r² × 0.04; 1: r² ×
///     0.01 (league standard below 9 and r below 4250) or 0.02; 0: r² × 0.005 or 0.01. V × the manager's reputation
///     ratio (0.1 to 1), × the league standard × 0.05 (0.25 to 1); with the balance negative, the player's
///     (`.player()`) `.current_reputation()` / r (0.15 to 0.85) × 0.65 (defect of the original: the ratio replaces V,
///     so V is 0 and the budget ends at an eighth when that is at most 125000, else 0); for a non-player V × 0.65. Above
///     V the budget becomes V + an eighth of the excess, at most 1.75 V when that passes 125000, and `adjusted` = 1.
/// Original: `Finance::get_transfer_budget` (0x598800, fin_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_transfer_budget(fin: usize, player: usize, add_listed: u32, adjusted: *mut u32, fog: u32, pending: u32) -> i64 {
    use budget_list_rows::*;
    let fin = Finance(fin);
    let club = club_at(fin.club_id() as isize);
    let mut budget = fin.balance();
    let mut fees: i32 = 0;
    let mut wages: i32 = 0;
    if add_listed != 0 {
        for k in 0..50 {
            let s = club.squad(k);
            if s != 0 && read_i32(s) < read_i32(G_STAFF_COUNT) && exe::staff_is_transfer_listed(s, 0usize) != 0 {
                covered!("mka_1 listed squad value");
                budget = budget.wrapping_add(Staff(s).value() as i32 as i64);
            }
        }
    }
    if !adjusted.is_null() {
        *adjusted = 0;
    }
    if pending != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        covered!("mka_1 pending deals");
        // ponytail: no exe:: wrapper for these two list builders yet
        let future_list: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u8 = core::mem::transmute(0x8ccb80usize);
        let offer_list: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u8 = core::mem::transmute(0x4ca300usize);
        let mut list: usize = 0;
        let n = future_list(read_usize(G_TRANSFER_MANAGER), club.addr(), &mut list) as i8 as i32;
        for i in 0..n.max(0) as usize {
            let row = list + i * FUTURE_SIZE;
            let person_id = read_i32(row + FUTURE_PERSON_ID);
            if read_u32(row + FUTURE_OUTGOING) == 0 {
                if player == 0 || person_id != read_i32(player) {
                    fees = fees.wrapping_add(read_i32(row + FUTURE_FEE));
                }
                continue;
            }
            if !(3..=6).contains(&read_u8(row + FUTURE_STAGE)) {
                continue;
            }
            let mut j = 0usize;
            let mut repeated = false;
            while j < i && !repeated {
                repeated = read_i32(list + j * FUTURE_SIZE + FUTURE_PERSON_ID) == person_id;
                j += 1;
            }
            if !repeated {
                fees = fees.wrapping_sub(read_i32(row + FUTURE_FEE));
            }
            // as in the original: the person of row j (this row, or the one after the first repeat)
            let who = staff_at(read_i32(list + j * FUTURE_SIZE + FUTURE_PERSON_ID) as isize);
            let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), who, club);
            if c != 0 {
                wages = wages.wrapping_sub(Contract(c).wage());
            }
        }
        if list != 0 {
            exe::crt_free(list);
        }
        let mut offers: usize = 0;
        let n = offer_list(read_usize(G_CONTRACT_MANAGER), club.addr(), &mut offers) as i8 as i32;
        for i in 0..n.max(0) as usize {
            let row = offers + i * OFFER_SIZE;
            if read_u8(row + OFFER_STATE) == 4 {
                continue;
            }
            let who = staff_at(read_i32(row + OFFER_PERSON_ID) as isize);
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), who);
            write_usize(G_CONTRACT_LOOKUP_RESULT, c);
            let at = if c != 0 { club_at(Contract(c).club() as isize).addr() } else { 0 };
            if at == club.addr() {
                let c2 = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), who, club);
                if c2 != 0 {
                    wages = wages.wrapping_sub(Contract(c2).wage());
                }
            }
            wages = wages.wrapping_add(read_i32(row + OFFER_WAGE));
            fees = fees.wrapping_add(read_i32(row + OFFER_FEE));
        }
        if offers != 0 {
            exe::crt_free(offers);
        }
    }
    let division = club.division();
    if division.is_null() || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(division.addr()) as usize).wrapping_mul(4))) == 0 {
        return budget;
    }
    let chairman = club.chairman();
    let mut m: i8 = if !chairman.is_null() && !chairman.non_player().is_null() {
        let mut v = (chairman.non_player().business() as i8).wrapping_add(6);
        let ambition = chairman.ambition();
        if ambition > 15 {
            v = v.wrapping_add(15i8.wrapping_sub(ambition));
        }
        v
    } else {
        16
    };
    let nation = club.nation();
    let is = |g: usize| !nation.is_null() && read_i32(nation.addr()) == read_i32(g);
    if is(KEY_NATION_GERMANY) {
        m = m.wrapping_add(5);
    }
    covered!("mka_1 board reserve");
    let m = m as i32;
    let mf = m as f64;
    let wage_base = fin.wage_bill().wrapping_add(wages) as f64;
    budget = budget.wrapping_sub(ftol_i64((wage_base - fin.instalment_balance() as f64 * read_f64(rdata::F64_0_6666666667)) * mf));
    let rep = club.reputation();
    let r = rep as i32;
    let stadium = club.stadium();
    if !stadium.is_null() {
        let seats = stadium.capacity().wrapping_mul(m);
        if club.own_stadium() != 0 {
            if rep > 0x1676 {
                budget = budget.wrapping_add(ftol_i64(seats as f64 * read_f64(rdata::F64_NEG_1_25)));
            } else if rep > 0x109a {
                budget = budget.wrapping_add(ftol_i64(seats as f64 * read_f64(0x96a668))); // -1.15
            } else if rep > 0x4e2 && club.professional_status() == 1 {
                budget = budget.wrapping_add(ftol_i64(seats as f64 * read_f64(0x96a660))); // -1.05
            } else {
                budget = budget.wrapping_sub(seats as i64);
            }
        } else {
            budget = budget.wrapping_sub(seats.wrapping_mul(2) as i64);
        }
    } else {
        budget = budget.wrapping_sub(m.wrapping_mul(2500) as i64);
    }
    let t = club.training() as i32;
    let training = if t < 6 {
        t.wrapping_mul(m).wrapping_mul(200)
    } else if t < 16 {
        (t - 2).wrapping_mul(m).wrapping_mul(500)
    } else {
        (t - 12).wrapping_mul(m).wrapping_mul(2000)
    };
    budget = budget.wrapping_sub(ftol(training as f64 * read_f64(rdata::F64_0_25)) as i64);
    budget = budget.wrapping_sub(r.wrapping_mul(m) as i64);
    let crowd = club.attendance().wrapping_mul(r) as f64 * mf;
    budget = budget.wrapping_add(ftol_i64(crowd * if rep < 0xfa0 { read_f64(0x96a658) } else { read_f64(rdata::F64_0_001) })); // 0.0005
    if !nation.is_null() {
        let dr = division.reputation() as i32;
        let k = if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
            5000
        } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) {
            2500
        } else {
            750
        };
        budget = budget.wrapping_add(dr.wrapping_mul(dr).wrapping_mul(k) as i64);
    }
    let r2 = r.wrapping_mul(r) as f64;
    budget = budget.wrapping_add(ftol_i64(r2 * mf * if club.own_stadium() != 0 { read_f64(rdata::F64_0_001) } else { read_f64(0x96a658) }));
    if club.continental_comp_id() != 0xffff_ffff && club.professional_status() == 1 {
        let base = r.wrapping_mul(300);
        let add = if nation.is_null() {
            0
        } else if [KEY_NATION_ENGLAND, KEY_NATION_GERMANY, KEY_NATION_ITALY, KEY_NATION_FRANCE, KEY_NATION_SCOTLAND, KEY_NATION_SPAIN].iter().any(|&g| is(g)) {
            base
        } else if [KEY_NATION_HOLLAND, KEY_NATION_SWEDEN, KEY_NATION_BELGIUM, KEY_NATION_PORTUGAL, KEY_NATION_TURKEY].iter().any(|&g| is(g)) {
            ftol(base as f64 * read_f64(rdata::F64_0_65))
        } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) {
            ftol(base as f64 * read_f64(rdata::F64_0_35))
        } else {
            ftol(base as f64 * read_f64(rdata::F64_0_5))
        };
        budget = budget.wrapping_add(add as i64);
    }
    // the committed money: the fees of step 3 and the season's net spending above 8 million (low dword, as the original)
    let net = fin.season_players_bought().wrapping_sub(fin.season_players_sold());
    let over = (if net > 0 { net as i32 } else { 0 }) as i64 - 8_000_000;
    let over = if over >= 0 { over as i32 } else { 0 };
    let committed = (fees.max(0) as i64).wrapping_add(over as i64);
    budget = budget.wrapping_sub(committed as i32 as i64);
    if budget > 0 {
        let balance = fin.balance();
        let tenth = balance as f64 * read_f64(rdata::F64_0_1);
        if (budget as f64) < tenth {
            covered!("mka_1 tenth of the balance");
            let top = read_f64(0x96a650); // 2.5 million
            budget = ftol_i64(if tenth < top { tenth } else { top });
        } else {
            let mut cap = ftol(r2 * read_f64(rdata::F64_0_5));
            let manager = club.manager();
            if !manager.is_null() && rep != 0 && !manager.non_player().is_null() {
                let mut q = manager.non_player().current_reputation() as f64 / r as f64;
                if q < read_f64(rdata::F64_1) {
                    if q < read_f64(rdata::F64_0_5) {
                        q = read_f32(rdata::F32_0_5) as f64;
                    }
                    cap = ftol(cap as f64 * q * q);
                }
            }
            let small = club.plc() != 0 || nation.is_null()
                || [KEY_NATION_SWEDEN, KEY_NATION_NORWAY, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, KEY_NATION_DENMARK, KEY_NATION_FINLAND, KEY_NATION_SCOTLAND, KEY_NATION_TURKEY, KEY_NATION_GERMANY].iter().any(|&g| is(g));
            let (floor, top) = if !nation.is_null() && !is(KEY_NATION_GERMANY) { (10_000_000, 40_000_000) } else { (15_000_000, 35_000_000) };
            if (cap as i64) < balance.wrapping_sub(floor) {
                cap = (balance as i32).wrapping_sub(floor as i32);
            }
            if small && cap as i64 > (top as i64).wrapping_sub(committed) {
                cap = (top as i32).wrapping_sub(committed as i32);
            }
            if cap < 50000 {
                cap = 50000;
            }
            if budget > cap as i64 {
                covered!("mka_1 soft cap");
                budget = ftol_i64(budget.wrapping_sub(cap as i64) as f64 * read_f64(rdata::F64_0_1) + cap as f64);
            }
        }
    }
    let mm = read_usize(G_MANAGER_MANAGER);
    if mm != 0 && !club.is_null() {
        budget = budget.wrapping_add(BoardRecord(board_row_of(mm, club)).extra_funds_granted() as i32 as i64);
    }
    if rep > 5000 {
        let least = ftol_i64((r as f64 * read_f64(rdata::F64_200) - read_f64(rdata::F64_1000000)) * division.reputation() as f64 * read_f64(0x96a640)); // 1/17
        if budget < least {
            budget = if least < 50000 { 50000 } else { least };
        }
    }
    if rep < 0xdac {
        let most = r.wrapping_mul(if club.professional_status() == 1 { 500 } else { 250 }) as i64;
        if budget > most {
            budget = most;
        }
    }
    if budget < 0 {
        budget = 0;
    }
    let keep = |b: i64, above: i64, part: f64, base: usize| -> i64 {
        if b > above { ftol_i64(b.wrapping_sub(above) as f64 * part + read_f64(base)) } else { b }
    };
    if [KEY_NATION_SWEDEN, KEY_NATION_NORWAY, KEY_NATION_DENMARK, KEY_NATION_WALES, KEY_NATION_UNITED_STATES, KEY_NATION_FINLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_ARGENTINA].iter().any(|&g| is(g)) {
        covered!("mka_1 small market");
        budget = keep(budget / 2, 2_500_000, read_f64(rdata::F64_0_1), 0x96a650);
    } else if is(KEY_NATION_BELGIUM) || is(KEY_NATION_PORTUGAL) {
        budget = keep(budget, 15_000_000, read_f64(rdata::F64_0_25), 0x96a630);
    } else if is(KEY_NATION_BRAZIL) {
        budget = keep(budget, 2_500_000, read_f64(rdata::F64_0_25), 0x96a650);
    } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_SCOTLAND) || is(KEY_NATION_HOLLAND) {
        budget = keep(budget, 17_500_000, read_f64(rdata::F64_0_25), 0x96a638);
    }
    if player == 0 || fog == 0 || read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 || read_usize(G_FOG_OF_WAR) == 0
        || exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        return budget;
    }
    let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), club.manager());
    if map == 0 {
        return budget;
    }
    covered!("mka_1 human knowledge");
    let level = exe::fog_of_war_map_get_knowledge_level(map, player) as i8;
    let rf = r as f64;
    let small_league = !nation.is_null() && (nation.league_standard() as i8) < 9 && rep < 0x109a;
    let mut v: i32 = match level {
        3 => budget as i32,
        1 => ftol(rf * rf * if small_league { read_f64(rdata::F64_0_01) } else { read_f64(rdata::F64_0_02) }),
        2 => ftol(rf * rf * read_f64(rdata::F64_0_04)),
        _ => ftol(rf * rf * if small_league { read_f64(rdata::F64_0_005) } else { read_f64(rdata::F64_0_01) }),
    };
    let manager = club.manager();
    if !manager.is_null() && !manager.non_player().is_null() && rep != 0 {
        let mut q = manager.non_player().current_reputation() as f64 / rf;
        if q > read_f64(rdata::F64_1) {
            q = read_f64(rdata::F64_1);
        } else if q < read_f64(rdata::F64_0_1) {
            q = read_f64(rdata::F64_0_1);
        }
        v = ftol(v as f64 * q);
    }
    if !club.division().is_null() {
        let mut l = (club.nation().league_standard() as i8) as f64 * read_f64(rdata::F64_0_05);
        if l < read_f64(rdata::F64_0_25) {
            l = read_f64(rdata::F64_0_25);
        } else if l > read_f64(rdata::F64_1) {
            l = read_f64(rdata::F64_1);
        }
        v = ftol(v as f64 * l);
    }
    if fin.balance() < 0 && rep != 0 {
        let sheet = Staff(player).player();
        let base = if !sheet.is_null() {
            let mut q = sheet.current_reputation() as f64 / rf;
            if q < read_f64(rdata::F64_0_15) {
                q = read_f64(rdata::F64_0_15);
            } else if q > read_f64(rdata::F64_0_85) {
                q = read_f64(rdata::F64_0_85);
            }
            q // defect of the original: the ratio replaces V instead of scaling it
        } else {
            v as f64
        };
        v = ftol(base * read_f64(rdata::F64_0_65));
    }
    if level != 3 && budget > v as i64 {
        covered!("mka_1 knowledge cap");
        let mut b = v.wrapping_sub(ftol(budget.wrapping_sub(v as i64) as f64 * read_f64(0x96a628))) as i64; // -0.125
        if b > 125_000 {
            let most = ftol(v as f64 * read_f64(rdata::F64_1_75)) as i64;
            if b > most {
                b = most;
            }
        }
        budget = b;
        if !adjusted.is_null() {
            *adjusted = 1;
        }
    }
    budget
}

/// The asking price for a player of a human club, as the computer's bids read it (`TRANSFER_OFFER::build_bid`
/// 0x8e5d60 and the shortlist and media routines, 9 call sites).
///
/// The player's main contract (`main_contract_of_staff`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives his club; when that
/// club is not human (`exe::human_manager_is_human_club` with 0, 1) the "Error" box (contract_manager.cpp, line
/// 0x1f28) and the stored value (`.value()`). Otherwise the contract with that club (`exe::contract_manager_get_contract_for_club`):
/// none, not of type 2 (`.contract_type()` & 0x3f, a player contract) or leaving on a Bosman (`.leaving_on_bosman()`):
/// 0. The stored value, at most the release fee (`.release_fee()`) when the clause is active
/// (`exe::staff_contract_has_active_release_clause`), is the floor; the computed price
/// (`exe::contract_manager_calc_asking_price` with `seller`, `buyer`, no reason flags, adjust 1, flag 1) is raised to
/// that floor and limited by the active release fee. Transfer listed or requested
/// (`exe::staff_is_transfer_listed_or_requested`): the floor. Without bit 4 of `.transfer_status()` [inference: the
/// manager set the asking price]: the floor. By `.squad_status()` & 0xf0: 0x10 and 0x50 [inference: key players] the
/// computed price; 0x20 and 0x30 the mean of both (signed, truncated); the others the floor.
/// Original: `contract_manager_get_human_asking_price` (0x4d42d0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_human_asking_price(this: usize, seller: usize, buyer: usize, player: usize) -> i32 {
    let p = Staff(player);
    let main = main_contract_of_staff(p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, main);
    let club = if main != 0 { club_at(Contract(main).club() as isize) } else { Club(0) };
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        show_error_at(0x9a8190, 0x1f28);
        return p.value() as i32;
    }
    let c = Contract(exe::contract_manager_get_contract_for_club(this, p, club));
    if c.is_null() || c.contract_type() & 0x3f != 2 || c.leaving_on_bosman() != 0 {
        return 0;
    }
    covered!("mka_1 human asking price");
    let mut stored = p.value() as i32;
    if exe::staff_contract_has_active_release_clause(c) != 0 && stored >= c.release_fee() as i32 {
        stored = c.release_fee() as i32;
    }
    let mut price = exe::contract_manager_calc_asking_price(this, seller, buyer, p, 0, 1, 1);
    if price <= stored {
        price = stored;
    }
    if exe::staff_contract_has_active_release_clause(c) != 0 && price >= c.release_fee() as i32 {
        price = c.release_fee() as i32;
    }
    if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 || c.transfer_status() & 4 == 0 {
        return stored;
    }
    covered!("mka_1 human asking price by status");
    match c.squad_status() & 0xf0 {
        0x10 | 0x50 => price,
        0x20 | 0x30 => price.wrapping_add(stored) / 2,
        _ => stored,
    }
}

/// Comparer for `get_transfer_budget`: side by side from the same draw state (the lists the callees allocate are
/// freed by each run), checking the result, `adjusted` and `G_CONTRACT_LOOKUP_RESULT`.
pub(crate) unsafe extern "thiscall" fn compare_get_transfer_budget(fin: usize, player: usize, add_listed: u32, adjusted: *mut u32, fog: u32, pending: u32) -> i64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u32, u32, u32) -> i64>(IDX_GET_TRANSFER_BUDGET);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    if !adjusted.is_null() {
        reg.push((adjusted as usize, 4));
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(fin, player, add_listed, adjusted, fog, pending),
        || crate::rng::without_counting(|| get_transfer_budget(fin, player, add_listed, adjusted, fog, pending)));
    SCOREBOARD.record(equal && r == n, || format!("get_transfer_budget(fin {:#x}, player {:#x}, {}, {}, {}) original={} rust={} {:x?} x {:x?}", fin, player, add_listed, fog, pending, r, n, mo, mr));
    r
}

/// Comparer for `get_human_asking_price`: side by side from the same draw state (the computed price may draw); a
/// player whose club is not human (the "Error" box) runs only in the original.
pub(crate) unsafe extern "thiscall" fn compare_get_human_asking_price(this: usize, seller: usize, buyer: usize, player: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32>(IDX_GET_HUMAN_ASKING_PRICE);
    let main = main_contract_of_staff(Staff(player));
    let club = if main != 0 { club_at(Contract(main).club() as isize) } else { Club(0) };
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        return o(this, seller, buyer, player);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, seller, buyer, player),
        || crate::rng::without_counting(|| get_human_asking_price(this, seller, buyer, player)));
    SCOREBOARD.record(equal && r == n, || format!("get_human_asking_price(seller {:#x}, buyer {:#x}, player {:#x}) original={} rust={} {:x?} x {:x?}", seller, buyer, player, r, n, mo, mr));
    r
}

// ---------- batch 288: the asking price for a player (contract_manager.cpp) ----------

/// `price` + `add` against `price` × `factor`, the lesser truncated (_ftol), as the original's ladders of the big
/// nations do (0x4d2a23 and the 37 steps like it: fild of the integer sum, fild of the price × the factor, fcom).
fn asking_step(price: i32, add: i32, factor: f64) -> i32 {
    let a = price.wrapping_add(add) as f64;
    let b = price as f64 * factor;
    ftol(if a < b { a } else { b })
}

/// The lesser of `price` + `add` and `price` × `times`, in integers (0x4d2d3e: lea, add edi, edi; cmp; jge).
fn asking_step_int(price: i32, add: i32, times: i32) -> i32 {
    let a = price.wrapping_add(add);
    let b = price.wrapping_mul(times);
    if a >= b { b } else { a }
}

/// The asking price a club wants for one of its players (the fee it would accept), with the reasons for a refusal.
///
/// Called by the transfer request (0x4d8705), the board veto (0x696ec5), the human club's price
/// (`get_human_asking_price` 0x4d4429), the computer's bids (0x8e6533, 0x8e6554), the bid ladders of the transfer manager
/// (0x8c8291, 0x8c8c6e, 0x8cd391...), the shortlists and the screens. `seller` owns the player, `buyer` (may be 0) wants
/// him; `reasons` (may be null) receives bits; `adjust` turns on the recent-signing factor; `flag` the youngster floor.
/// In this order:
/// 1. No seller, no contract with him (`exe::contract_manager_get_contract_for_club`), not a player contract
///    (`.contract_type()` & 0x3f ≠ 2) or leaving on a Bosman (`.leaving_on_bosman()`): 0. A human seller
///    (`exe::human_manager_is_human_club` with 1, 1) whose manager is on holiday (`exe::human_manager_is_on_holiday`)
///    with the holiday option 0 set (`exe::human_manager_get_holiday_option`) and without bit 4 of `.transfer_status()`:
///    the stored value (`.value()`) at once.
/// 2. The league factor L: x = the nation's `.state_of_development()` (3 when outside 1..3); with a division,
///    x = (x + 1) / 2, × 4 (`.professional_status()` 2) or × 7.5 (3), and y = (division `.reputation()`² × 25 + club
///    `.reputation()`); without one, x × 2 or × 7.5 and y = (nation `.league_standard()`² × 25 + club reputation); no
///    nation: 1. L = 1 − (20 − y / (x × 1000))² × 0.2 / 60, between 0.1 and 1.1.
/// 3. The age factor A: above 28, (1 − (age − 28) × 0.1)² (the CRT pow, `game_pow`).
/// 4. Happiness h (`.happiness()`) and the shortfall s by squad role (`.squad_status()` >> 4): 1: max(75 − h, 0); 2 and 5:
///    max(50 − h, 0); 3: max(40 − h, 0); 4: max(25 − h, 0); 6: max(−h, 0); 7: h = −100, s = 0. A player absent without
///    leave (`exe::awol_is_player_awol`): s = 0, the adjust is off and h is at most −5 × the seller manager's
///    `.discipline()` (−100 without a manager).
/// 5. Morale M = 1 + morale (`.morale()`) × 1/300 (positive) or × 1/134 (otherwise); × 1.2 without grievances
///    (`.grievances()` = 0), × 0.8333 with them.
/// 6. The seen potential: `exe::match_man_judge_player_ability` (the seller's `.manager()`, kind 1, with the player's
///    `.current_reputation()` / 2 + `.potential_ability()` × 25); the value V = `exe::contract_manager_calc_player_value`
///    (player, seller, 0, seen, 0). V = 0: returns 0.
/// 7. The recent-signing factor C (adjust on, a computer seller, the player neither listed nor requesting
///    (`exe::staff_is_transfer_listed_or_requested`), role not 7 or 8): f = 7.5, 6 (role 2), 5.5 (3) or 3.5 (4 and 5);
///    d = the days since the player joined the club (`staff::DATE_JOINED_CLUB`), 121 to 365; C = f − d / (365 / (f − 1))
///    (f after 121 days or less, 1 after a year). Above 2: reason 0x100.
/// 8. Happiness factor H = (s / 2 + h) × 0.01167 + 1.4165; h ≥ 50 at a computer seller: × ((s / 2 + h)² / 50 × 0.006
///    + 0.7). Reasons: h > 60 → 0x200, h > 20 → 0x400.
/// 9. The finance factor G: the balance B (`exe::financetable_get_club_finance`, `.balance()`) against the table T at
///    0x9a7f48 by reputation / 500 (50 thousand to 25 million): B ≥ T: B / T; else ((B − T) / T + x) / (x + 1), x =
///    B × 1.25e-6 between 0 and 10. G = (that + n² × 0.05) + h × 0.002, n = the low nibble of the player's
///    `staff_state::STATUS_FLAGS` [inference: the knowledge level]; above 1: × 0.25 + 0.75, else × 0.2125 + 0.7875; at
///    least h × 0.0025 + 0.5, else at most 2.5.
/// 10. The buyer factor D (1 without a buyer); h' = h, or h × 1.5 when h > 20 without grievances: Celtic × Rangers
///     (`G_KEY_CLUB_GLASGOW_CELTIC`, `G_KEY_CLUB_GLASGOW_RANGERS`): h' × 0.04 + 2.3, at least 2.3; a league clash
///     (`exe::comp_util_get_league_clash_type`): h' / 30 + 2, at least 2, in the same city (the stadiums' `.city()`),
///     else h' × 0.02 + 1.6, at least 1.3; one of the seller's rivals (`.rival1()`..`.rival3()`): h' × 0.02 + 1.6 (same
///     city) or h' × 0.01 + 1.3, at least 1.3; these three set reason 0x800 when D ≥ 1.3. Same division, not
///     `KEY_CLUB_COMP_A_LOWER_DIVISION`: h' × 0.005 + 1, at least 1, reason 0x1000 above 1.3.
/// 11. Squad importance S (1): role 5 (key) sets reason 0x2000; otherwise, seen above k × 17 / 20 (k = reputation / 50
///     + 1), age below 25 and role below 7: role 6 sets 0x2000; else S = 1 + ftol((seen − k × 17 / 20) × k × 0.005) ×
///     0.04, at least 1; role 4 also multiplies H by S; S > 1.6 sets 0x2000.
/// 12. The multiplier Q = G × H × M × S × D × C, at most clamp(D² × C, 4.25, 10) and at most 6 × G.
/// 13. R = (`.current_reputation()` + `.home_reputation()`) × 0.3 + `.world_reputation()` × 0.4; e = (reputation + R −
///     4000) × 225 between 150 thousand and 2 million; E = ((1 + (R − (1.2 × reputation − 2000)) × R × 5e-8) × u + 100
///     − u) / 100, u = h / 2 + 50; above 1, E⁵; E between 1 and 3.
/// 14. The premium p = V × Q − V: negative, at least −0.4 V and −3 million; otherwise min(p × C, e × Q) × min(C², 25) ×
///     L × A, halved when listed or requesting, at most 50 million. Price = V + p; with E > 1, + L × A × t, t = price ×
///     E − price (above 5 million 0.625 t + 1.875 million, above 10 million 0.125 t + 8.75 million); at least 1000.
/// 15. A work permit needed (`exe::transfer_manager_player_needs_work_permit`): at most V. Listed or requesting, or
///     at a club with a linked B team (`exe::club_get_linked_b_team`, not the B side) and not role 1, 5, 2 or 3: the
///     reasons keep only 0x800 and the price is at most `.value()` × D.
/// 16. A computer seller with reason 0x800, reputation above 4250, age above 18, role not 7, wealth band
///     (`exe::finance_get_wealth_band`) not −1 or −2: at least (the player's club's `.reputation()` + 25000) × 100 when
///     below (the seller's reputation + 25000) × 100.
/// 17. At most max(3V, V + 5 million) × D × C.
/// 18. Above V with grievance bits 0x16b: the contract days left (`.contract_expires()`) pull the price towards V'
///     (V × days / 365 below a year): w = days × 0.0027 − 0.485 (0..1), p' = (1 − w) V' + w price; ratio = (V' + p') /
///     max(2 B D, 1) between 1 and 3, y = 1.375 − 0.375 ratio, price = (1 − y) V' + y p'.
/// 19. The seller's shortlist entry for the player (`exe::shortlist_manager_find_club_shortlist_entry` with 0, 1). A big
///     nation (England, Italy, Spain, France, Germany, Scotland, the USA, Japan, Holland, Portugal) with reputation
///     above 4000, league standard above 12 or reputation 7000, a finance object with a wealth band, the player neither
///     listed (`exe::staff_is_transfer_listed`) nor listed by request (`exe::staff_is_listed_by_request`): with the entry
///     targeting him (`.target_staff_id()`) or picked for a continental squad (`.squad_selected_for()` 2 with
///     `.continental_comp_id()` ≥ 0), the ladder A, else with an empty entry (target −1) the ladder B; the ladders
///     raise the price by role (1, 2, 3) with steps min(price + K, price × f), more for clubs above reputation 6750 or
///     a division above 15, less with grievances or low morale (`exe::date_days_since` of the contract end above 425 or
///     225 days), and again in the buyer's division; the youth step (A: below 25, B: below 22, roles not 6 or 7, seen ×
///     50 above the reputation): min(seen / (125 − morale / 4) × price, price − |age − 20| × 1333333 + 5 million).
///     Without the big-nation conditions and without an entry, above V: h < 60, 0.67 V + 0.33 price; else t = 1.27 −
///     h × 0.01, (1 − t) price + t V.
/// 20. The youngster floor (`flag`, age below 24, seen above 110, a nation, role not 7): si = seen, raised by pace +
///     acceleration, the physical score ((stamina × 1.2 + strength) × (2.6 − 0.07 age) + teamwork × 0.8 + work rate ×
///     1.2 + jumping × 0.75) and the dribbling and finishing ratings (`exe::staff_get_player_attribute` 11 and 12, ×
///     the same age factor); reputation / 1000 with the entry; by role reputation / 500 or / 670 (at least 121 for a
///     role 1 under 19); − reputation / 1000 above 20 with current ability below 45; training (`.training()`) above 10
///     with seen above 120; **the seller manager's `.youngsters()` − 10 at a computer club**; half of what is missing to
///     `.potential_ability()` when the player joined after the manager (`exe::date_operator_3e`). si above 120, at most
///     clamp(reputation × 0.0167 + 100, 150, 220): Y = (si² − 14400) × 250 × 0.6153846 + bonus (si above 150 with the
///     entry, reputation ≥ 2500 and league standard above 10: (min(4 si − 600, 200))² × 250 × reputation / 6000);
///     league standard below 13 or professional status 3: × r / 5250 (r the reputation, 0.75 r at status 3 below 5250,
///     pulled to 4000 + (r − 4000) / 5 below standard 5); grievances: × 0.75 (morale below −50), × 0.8 (0x800000),
///     × 0.95 (`exe::staff_contract_get_flags` 8), × 0.9 (1), × 0.9 (0x40); loyalty below 6 with professionalism below 10:
///     × 0.9 (negative morale), × 0.9 (buyer 750 above the seller's reputation); loyalty above 15 without negative morale:
///     × 1.1. The price is at least Y (reason 0x2000).
/// 21. The age cap (goalkeepers, `.goalkeeper()` 20, count 3 years less): 29 V + 5 million, 30 + 4, 31 + 3.25, 32 + 2.5,
///     33 + 1.75, 34 + 1, 35 + 0.5, 36 + 0.2, 37 + 0.1, 38 to 45 V.
/// 22. Portugal above V: below 6 million × (price × 1e-7 + 0.3), else × 0.9, at least V. South Korea above V and 50000:
///     (price − 50000) × 0.15 + 50000.
/// 23. The active release fee (`exe::staff_contract_has_active_release_clause`, `.release_fee()`) caps it.
/// Original: `contract_manager_calc_asking_price` (0x4d1600, contract_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn calc_asking_price(this: usize, seller: usize, buyer: usize, player: usize, reasons: *mut u16, adjust: u32, flag: u32) -> i32 {
    let seller = Club(seller);
    let buyer = Club(buyer);
    let p = Staff(player);
    let mark = |bits: u16| {
        if !reasons.is_null() {
            write_u16(reasons as usize, read_u16(reasons as usize) | bits);
        }
    };
    if seller.is_null() {
        return 0;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(this, p, seller));
    if contract.is_null() || contract.contract_type() & 0x3f != 2 || contract.leaving_on_bosman() != 0 {
        return 0;
    }
    if exe::human_manager_is_human_club(seller, 1, 1, 0, 0) != 0
        && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), exe::database_get_club_manager(seller)) != 0
        && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 0, exe::database_get_club_manager(seller)) != 0
        && contract.transfer_status() & 4 == 0 {
        covered!("mka_2 manager on holiday");
        return p.value() as i32;
    }
    let rep = seller.reputation() as i32;
    let nation = seller.nation();
    let division = seller.division();
    // the league factor (0x4d16ea..0x4d185a)
    let x0 = if nation.is_null() { 0.0 } else {
        let x = nation.state_of_development() as i8 as f64;
        if x < read_f64(rdata::F64_1) || x > read_f64(rdata::F64_3) { read_f64(rdata::F64_3) } else { x }
    };
    let league_y = if nation.is_null() {
        read_f64(rdata::F64_1)
    } else if !division.is_null() {
        let mut x = (x0 + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5);
        match seller.professional_status() as i8 {
            2 => x *= read_f64(rdata::F64_4),
            3 => x *= read_f64(rdata::F64_7_5),
            _ => {}
        }
        let dr = division.reputation() as i32;
        dr.wrapping_mul(dr).wrapping_mul(25).wrapping_add(rep) as f64 / (x * read_f64(rdata::F64_1000))
    } else {
        let mut x = x0;
        match seller.professional_status() as i8 {
            2 => x = x + x,
            3 => x *= read_f64(rdata::F64_7_5),
            _ => {}
        }
        let ls = nation.league_standard() as i8 as i32;
        ls.wrapping_mul(ls).wrapping_mul(25).wrapping_add(rep) as f64 / (x * read_f64(rdata::F64_1000))
    };
    let z = read_f64(rdata::F64_20) - league_y;
    let mut league = read_f64(rdata::F64_1) - z * z * read_f64(rdata::F64_0_2) * read_f64(rdata::F64_0_01666666667);
    if league < read_f64(rdata::F64_0_1) {
        league = read_f64(rdata::F64_0_1);
    } else if league > read_f64(rdata::F64_1_1) {
        league = read_f64(rdata::F64_1_1);
    }
    // the age factor (0x4d185a)
    let age = p.age() as i8;
    let mut age_factor = read_f64(rdata::F64_1);
    if age > 28 {
        age_factor = game_pow(read_f64(rdata::F64_1) - (age as i32 - 28) as f64 * read_f64(rdata::F64_0_1), read_f64(rdata::F64_2));
    }
    // happiness and the shortfall by squad role (0x4d188e)
    let mut h = contract.happiness() as i16;
    let hi = h as i32;
    let mut short: i32 = match contract.squad_status() >> 4 {
        1 => (75 - hi).max(0),
        2 | 5 => (50 - hi).max(0),
        3 => (40 - hi).max(0),
        4 => (25 - hi).max(0),
        6 => (-hi).max(0),
        7 => {
            h = -100;
            0
        }
        _ => 0,
    };
    let adj: u8;
    if exe::awol_is_player_awol(read_usize(G_AWOL), p, 1) != 0 {
        covered!("mka_2 absent without leave");
        let manager = Staff(exe::database_get_club_manager(seller));
        if manager.is_null() {
            h = -100;
        } else if !manager.non_player().is_null() {
            let least = -5 * (manager.non_player().discipline() as i8 as i32);
            if h as i32 >= least {
                h = least as i16;
            }
        }
        short = 0;
        adj = 0;
    } else {
        adj = adjust as u8;
    }
    // morale (0x4d19cd)
    let morale = contract.morale();
    let per_point = if morale > 0 { read_f64(rdata::F64_0_003333333333) } else { read_f64(0x969090) }; // 1/300, 1/134
    let mut morale_factor = morale as f64 * per_point + read_f64(rdata::F64_1);
    morale_factor *= if contract.grievances() == 0 { read_f64(rdata::F64_1_2) } else { read_f64(rdata::F64_0_8333333333) };
    // the seen potential and the value (0x4d1a13..0x4d1a69)
    let sheet = p.player();
    let seen_arg = (sheet.current_reputation() / 2).wrapping_add(sheet.potential_ability().wrapping_mul(25));
    let seen = exe::match_man_judge_player_ability(seller.manager(), p, seller, 1, seen_arg);
    let value = exe::contract_manager_calc_player_value(this, p, seller, 0, seen as i32, 0);
    if value == 0 {
        return 0;
    }
    // the recent-signing factor (0x4d1a6f..0x4d1b80)
    let mut signing = read_f64(rdata::F64_1);
    if adj != 0 && exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0 && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && contract.squad_status() & 0xf0 != 0x70 && contract.squad_status() & 0xf0 != 0x80 {
        covered!("mka_2 recent signing");
        let f = match contract.squad_status() >> 4 {
            2 => read_f32(0x969084), // 6.0
            3 => read_f32(rdata::F32_5_5),
            4 | 5 => read_f32(rdata::F32_3_5),
            _ => read_f32(rdata::F32_7_5),
        } as f64;
        let mut days = 365i32.wrapping_mul(read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(p.addr() + staff::DATE_JOINED_CLUB_YEAR) as i32)
            .wrapping_sub(read_i16(p.addr() + staff::DATE_JOINED_CLUB) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32);
        if days <= 121 {
            days = 121;
        } else if days > 365 {
            days = 365;
        }
        signing = f - days as f64 / (read_f64(rdata::F64_365) / (f - read_f64(rdata::F64_1)));
        if signing > read_f64(rdata::F64_2) {
            mark(0x100);
        }
    }
    // the happiness factor (0x4d1b80..0x4d1c15)
    let hs = (short as i16 as i32) / 2 + h as i32;
    let mut happy = hs as f64 * read_f64(0x969068) + read_f64(0x969060); // 0.01167, 1.4165
    if h >= 50 && exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0 {
        let k = hs.wrapping_mul(hs) / 50;
        happy = (k as f64 * read_f64(0x969058) + read_f64(rdata::F64_0_7)) * happy; // 0.006
    }
    if h > 60 {
        mark(0x200);
    } else if h > 20 {
        mark(0x400);
    }
    // the finance factor (0x4d1c15..0x4d1d74)
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), seller);
    let balance = if fin == 0 { read_f64(rdata::F64_0) } else { Finance(fin).balance() as f64 };
    let band = read_f64((0x9a7f48isize + (rep / 500) as isize * 8) as usize);
    let rich = if balance < band {
        let r = (balance - band) / band;
        let mut x = balance * read_f64(0x969050); // 1.25e-6
        if read_f64(rdata::F64_0) > x {
            x = read_f64(rdata::F64_0);
        } else if read_f64(rdata::F64_10) < x {
            x = read_f64(rdata::F64_10);
        }
        (r + x) / (x + read_f64(rdata::F64_1))
    } else {
        balance / band
    };
    let hf = h as f64;
    let n = (read_u8(contract_manager_staff_state_record(read_i32(p.addr()) as usize) + staff_state::STATUS_FLAGS) & 0xf) as i32;
    let mut money = rich + (n * n) as f64 * read_f64(rdata::F64_0_05) + hf * read_f64(rdata::F64_0_002);
    money = if money > read_f64(rdata::F64_1) {
        money * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_0_75)
    } else {
        money * read_f64(0x969038) + read_f64(0x969030) // 0.2125, 0.7875
    };
    let least = hf * read_f64(rdata::F64_0_0025) + read_f64(rdata::F64_0_5);
    if least > money {
        money = least;
    } else if read_f64(rdata::F64_2_5) < money {
        money = read_f64(rdata::F64_2_5);
    }
    // the buyer factor (0x4d1d74..0x4d2006)
    let mut dfac = read_f64(rdata::F64_1);
    if !buyer.is_null() {
        let h2 = (if h > 20 && contract.grievances() == 0 { ftol(hf * read_f64(rdata::F64_1_5)) as i16 } else { h }) as f64;
        let celtic = read_i32(G_KEY_CLUB_GLASGOW_CELTIC);
        let rangers = read_i32(G_KEY_CLUB_GLASGOW_RANGERS);
        let (b, s) = (read_i32(buyer.addr()), read_i32(seller.addr()));
        let same_city = !buyer.stadium().is_null() && !seller.stadium().is_null() && buyer.stadium().city() == seller.stadium().city();
        let floor = |d: f64, least: f64| if least > d { least } else { d };
        let clash = if (b == celtic || b == rangers) && (s == celtic || s == rangers) {
            covered!("mka_2 old firm");
            dfac = floor(h2 * read_f64(rdata::F64_0_04) + read_f64(0x969010), read_f64(0x969010)); // 2.3
            true
        } else if exe::comp_util_get_league_clash_type(buyer, seller) != 0 {
            covered!("mka_2 derby");
            dfac = if same_city {
                floor(h2 * read_f64(rdata::F64_0_03333333333) + read_f64(rdata::F64_2), read_f64(rdata::F64_2))
            } else {
                floor(h2 * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_1_6), read_f64(rdata::F64_1_3))
            };
            true
        } else if buyer.addr() == seller.rival1() || buyer.addr() == seller.rival2() || buyer.addr() == seller.rival3() {
            covered!("mka_2 rival");
            dfac = if same_city {
                floor(h2 * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_1_6), read_f64(rdata::F64_1_3))
            } else {
                floor(h2 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_1_3), read_f64(rdata::F64_1_3))
            };
            true
        } else {
            let bd = buyer.division();
            if !bd.is_null() && read_i32(bd.addr()) != read_i32(KEY_CLUB_COMP_A_LOWER_DIVISION) && bd == division {
                covered!("mka_2 same division");
                dfac = floor(h2 * read_f64(rdata::F64_0_005) + read_f64(rdata::F64_1), read_f64(rdata::F64_1));
                if dfac > read_f64(rdata::F64_1_3) {
                    mark(0x1000);
                }
            }
            false
        };
        if clash && !(dfac < read_f64(rdata::F64_1_3)) {
            mark(0x800);
        }
    }
    // squad importance (0x4d2006..0x4d2124)
    let k = ((rep / 50) + 1) as i16 as i32;
    let th = (k * 17) / 20;
    let mut importance = read_f64(rdata::F64_1);
    let status = contract.squad_status() & 0xf0;
    if !reasons.is_null() && status == 0x50 {
        mark(0x2000);
    } else if seen > th as i16 && age < 25 && (contract.squad_status() >> 4) < 7 {
        if !reasons.is_null() && status == 0x60 {
            mark(0x2000);
        } else {
            covered!("mka_2 squad importance");
            let rise = ftol((seen as i32 - th as i16 as i32) as f64 * (k as f64 * read_f64(rdata::F64_0_005))) as i16;
            importance = rise as f64 * read_f64(rdata::F64_0_04) + read_f64(rdata::F64_1);
            if importance < read_f64(rdata::F64_1) {
                importance = read_f64(rdata::F64_1);
            }
            if status == 0x40 {
                happy = happy * importance;
            }
            if importance > read_f64(rdata::F64_1_6) {
                mark(0x2000);
            }
        }
    }
    // the multiplier and its caps (0x4d2124..0x4d21bd)
    let mut cap = dfac * dfac * signing;
    if read_f64(0x969008) > cap { // 4.25
        cap = read_f64(0x969008);
    } else if read_f64(rdata::F64_10) < cap {
        cap = read_f64(rdata::F64_10);
    }
    let mut q = money * happy * morale_factor * importance * dfac * signing;
    if cap < q {
        q = cap;
    }
    if q > money * read_f64(rdata::F64_6) {
        q = money * read_f64(rdata::F64_6);
    }
    // the reputation terms (0x4d21bd..0x4d22fa)
    let r = ftol((sheet.current_reputation() as f64 + sheet.home_reputation() as f64) * read_f64(rdata::F64_0_3)
        + sheet.world_reputation() as f64 * read_f64(rdata::F64_0_4)) as i16 as i32;
    let mut e = rep.wrapping_add(r).wrapping_sub(4000).wrapping_mul(225);
    if e < 150_000 {
        e = 150_000;
    } else if e > 2_000_000 {
        e = 2_000_000;
    }
    let u = hf * read_f64(rdata::F64_0_5) + read_f64(rdata::F64_50);
    let v = (r as f64 - (rep as f64 * read_f64(rdata::F64_1_2) - read_f64(0x9688f8))) * (r as f64 * read_f64(0x968ff0)); // 2000, 5e-8
    let mut ex = (((v + read_f64(rdata::F64_1)) * u + read_f64(rdata::F64_100)) - u) * read_f64(rdata::F64_0_01);
    if ex > read_f64(rdata::F64_1) {
        ex = game_pow(ex, read_f64(rdata::F64_5));
    }
    if read_f64(rdata::F64_1) > ex {
        ex = read_f64(rdata::F64_1);
    } else if read_f64(rdata::F64_3) < ex {
        ex = read_f64(rdata::F64_3);
    }
    // the premium (0x4d22fa..0x4d24aa)
    let ceiling = ftol(e as f64 * q);
    let vf = value as f64;
    let raw = ftol(vf * q - vf);
    let mut premium;
    if raw < 0 {
        let least = vf * read_f64(rdata::F64_NEG_0_4);
        premium = ftol(if raw as f64 > least { raw as f64 } else { least });
    } else {
        let a = raw as f64 * signing;
        let t = ftol(if a < ceiling as f64 { a } else { ceiling as f64 });
        let mut cc = signing * signing;
        if read_f64(rdata::F64_25) < cc {
            cc = read_f64(rdata::F64_25);
        }
        premium = ftol(cc * league * age_factor * t as f64);
    }
    if premium < -3_000_000 {
        premium = -3_000_000;
    } else {
        if premium > 0 && exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
            premium = ftol(premium as f64 * read_f64(rdata::F64_0_5));
        }
        if premium > 50_000_000 {
            premium = 50_000_000;
        }
    }
    covered!("mka_2 premium");
    let mut price = value.wrapping_add(premium);
    if ex > read_f64(rdata::F64_1) {
        covered!("mka_2 reputation premium");
        let pf = price as f64;
        let mut t = ftol(pf * ex - pf);
        if t > 5_000_000 {
            t = ftol(t as f64 * read_f64(rdata::F64_0_625) + read_f64(0x968fd8)); // 1875000
        }
        if t > 10_000_000 {
            t = ftol(t as f64 * read_f64(rdata::F64_0_125) + read_f64(0x968fc8)); // 8750000
        }
        price = price.wrapping_add(ftol(league * age_factor * t as f64));
    }
    if price < 1000 {
        price = 1000;
    }
    // the work permit, the listed player and the B team (0x4d24aa..0x4d2585)
    let at_value_times_d = |price: i32| -> i32 {
        let a = p.value() as i32 as f64 * dfac;
        if a < price as f64 { ftol(a) } else { price }
    };
    if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, seller, 0) != 0 {
        if price >= value {
            price = value;
        }
    } else if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
        covered!("mka_2 listed");
        if !reasons.is_null() {
            write_u16(reasons as usize, read_u16(reasons as usize) & 0x800);
        }
        price = at_value_times_d(price);
    } else if !p.club_job().is_null() {
        let mut b_side: u32 = 0; // the original's slot is not initialized: the callee writes it whenever it returns a club
        if exe::club_get_linked_b_team(p.club_job(), &mut b_side, 1) != 0 && b_side == 0 && !matches!(status, 0x10 | 0x50 | 0x20 | 0x30) {
            if !reasons.is_null() {
                write_u16(reasons as usize, read_u16(reasons as usize) & 0x800);
            }
            price = at_value_times_d(price);
        }
    }
    // the rich club's floor (0x4d2585..0x4d261f)
    if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0 && !reasons.is_null() && read_u16(reasons as usize) & 0x800 != 0
        && rep > 4250 && age > 18 && status != 0x70 && exe::finance_get_wealth_band(fin) != 0xff && exe::finance_get_wealth_band(fin) != 0xfe
        && price < rep.wrapping_add(25000).wrapping_mul(100) {
        covered!("mka_2 rich club floor");
        price = (p.club_job().reputation() as i32).wrapping_add(25000).wrapping_mul(100);
    }
    // at most max(3 V, V + 5 million) × D × C (0x4d261f..0x4d267e)
    let triple = value.wrapping_mul(3);
    let most = (if triple > value.wrapping_add(5_000_000) { triple } else { value.wrapping_add(5_000_000) }) as f64 * dfac * signing;
    if !((price as f64) < most) {
        price = ftol(most);
    }
    // the grievances and the contract days left (0x4d267e..0x4d27e7)
    if price > value && contract.grievances() & 0x16b != 0 {
        covered!("mka_2 grievances");
        let days = 365i32.wrapping_mul(contract.contract_expires_year() as u16 as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32)
            .wrapping_add(contract.contract_expires() as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
        let base = if days >= 365 { value } else { ftol(days as f64 * read_f64(rdata::F64_0_002739726027) * value as f64) };
        let mut w = days as f64 * read_f64(0x968fc0) - read_f64(0x968fb8); // 0.0027, 0.485
        if read_f64(rdata::F64_0) > w {
            w = read_f64(rdata::F64_0);
        } else if read_f64(rdata::F64_1) < w {
            w = read_f64(rdata::F64_1);
        }
        let bf = base as f64;
        let pulled = ftol((read_f64(rdata::F64_1) - w) * bf + price as f64 * w);
        let mut x = balance * dfac;
        x = x + x;
        if read_f64(rdata::F64_1) > x {
            x = read_f64(rdata::F64_1);
        }
        let mut ratio = base.wrapping_add(pulled) as f64 / x;
        if read_f64(rdata::F64_1) > ratio {
            ratio = read_f64(rdata::F64_1);
        } else if read_f64(rdata::F64_3) < ratio {
            ratio = read_f64(rdata::F64_3);
        }
        let y = read_f64(rdata::F64_1_375) - ratio * read_f64(rdata::F64_0_375);
        price = ftol((read_f64(rdata::F64_1) - y) * bf + pulled as f64 * y);
    }
    // the big nations' ladders (0x4d27e7..0x4d3886)
    let entry = ShortlistSlot(exe::shortlist_manager_find_club_shortlist_entry(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), seller, p, 0, 1));
    let is = |g: usize| read_i32(nation.addr()) == read_i32(g);
    let big = rep > 4000 && !nation.is_null()
        && [KEY_NATION_ENGLAND, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_FRANCE, KEY_NATION_GERMANY, KEY_NATION_SCOTLAND, KEY_NATION_UNITED_STATES, KEY_NATION_JAPAN, KEY_NATION_HOLLAND, KEY_NATION_PORTUGAL].iter().any(|&g| is(g))
        && ((nation.league_standard() as i8) > 12 || rep >= 7000)
        && fin != 0 && exe::finance_get_wealth_band(fin) != 0xff && exe::finance_get_wealth_band(fin) != 0xfe
        && exe::staff_is_transfer_listed(p, 0usize) == 0 && exe::staff_is_listed_by_request(p, 0usize) == 0;
    let same_div = || !buyer.is_null() && seller.division() == buyer.division();
    let days_left = || exe::date_days_since(contract.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
    let youth = |price: i32, below: i8| -> i32 {
        if age < below && status != 0x70 && status != 0x60 && (seen as i32).wrapping_mul(50) > rep {
            covered!("mka_2 youth step");
            let ratio = seen as f64 / (125 - morale as i32 / 4) as f64;
            let lose = (age as i32 - 20).abs().wrapping_mul(1_333_333);
            let a = ratio * price as f64;
            let b = price.wrapping_sub(lose).wrapping_add(5_000_000) as f64;
            ftol(if a < b { a } else { b })
        } else {
            price
        }
    };
    let small_club = rep <= 6750 && (division.is_null() || division.reputation() <= 15);
    let g = contract.grievances();
    let target = if entry.is_null() { -2 } else { entry.target_staff_id() as i32 };
    if big && (target == read_i32(p.addr()) || (p.squad_selected_for() == 2 && seller.continental_comp_id() as i32 > -1)) {
        covered!("mka_2 ladder A");
        price = youth(price, 25);
        if small_club {
            if g != 0 && morale <= 0 {
                if g & 0x101 != 0 && days_left() > 425 {
                    match status {
                        0x10 => {
                            price = asking_step(price, 375_000, read_f64(rdata::F64_1_25));
                            if same_div() { price = asking_step(price, 50_000, read_f64(rdata::F64_1_05)); }
                        }
                        0x20 => {
                            price = asking_step(price, 200_000, read_f64(rdata::F64_1_2));
                            if same_div() { price = asking_step(price, 37_500, read_f64(rdata::F64_1_025)); }
                        }
                        _ => {}
                    }
                }
            } else {
                match status {
                    0x10 => {
                        price = asking_step(price, 750_000, read_f64(rdata::F64_1_75));
                        if same_div() { price = asking_step(price, 100_000, read_f64(rdata::F64_1_1)); }
                    }
                    0x20 => {
                        price = asking_step(price, 400_000, read_f64(rdata::F64_1_4));
                        if same_div() { price = asking_step(price, 75_000, read_f64(0x968f90)); } // 1.075
                    }
                    0x30 => {
                        price = asking_step(price, 150_000, read_f64(rdata::F64_1_15));
                        if same_div() { price = asking_step(price, 25_000, read_f64(rdata::F64_1_025)); }
                    }
                    _ => {}
                }
            }
        } else if g != 0 && morale <= -50 {
            if g & 0x101 != 0 && days_left() > 425 {
                match status {
                    0x10 => {
                        price = asking_step_int(price, 2_500_000, 2);
                        if same_div() { price = asking_step_int(price, 1_000_000, 2); }
                    }
                    0x20 => {
                        price = asking_step(price, 1_500_000, read_f64(rdata::F64_1_5));
                        if same_div() { price = asking_step(price, 750_000, read_f64(rdata::F64_1_75)); }
                    }
                    0x30 => {
                        price = asking_step(price, 1_000_000, read_f64(rdata::F64_1_25));
                        if same_div() { price = asking_step(price, 400_000, read_f64(rdata::F64_1_25)); }
                    }
                    _ => {}
                }
            } else if !buyer.is_null() && days_left() > 225 && status != 0x70 && p.professionalism() > 5 && p.loyalty() > 5
                && !sheet.is_null() && sheet.current_reputation() > 6250 && contract.happiness() > p.loyalty().wrapping_neg()
                && age > 22 && age < 31 {
                price = asking_step(price, 1_000_000, read_f64(rdata::F64_1_25));
                if seller.division() == buyer.division() && exe::comp_util_get_league_clash_type(seller, buyer) != 0 {
                    price = asking_step(price, 400_000, read_f64(rdata::F64_1_25));
                }
            }
        } else {
            match status {
                0x10 => {
                    price = asking_step_int(price, 5_000_000, 5);
                    if same_div() { price = asking_step_int(price, 1_000_000, 2); }
                }
                0x20 => {
                    price = asking_step(price, 2_500_000, read_f64(rdata::F64_2_5));
                    if same_div() { price = asking_step(price, 750_000, read_f64(rdata::F64_1_75)); }
                }
                0x30 => {
                    price = asking_step_int(price, 1_500_000, 2);
                    if same_div() { price = asking_step(price, 400_000, read_f64(rdata::F64_1_4)); }
                }
                _ => {}
            }
        }
    } else if big && !entry.is_null() {
        if target == -1 {
            covered!("mka_2 ladder B");
            price = youth(price, 22);
            if small_club {
                if g == 0 || morale > 0 {
                    if status == 0x10 {
                        price = asking_step(price, 250_000, read_f64(rdata::F64_1_25));
                        if same_div() { price = asking_step(price, 50_000, read_f64(rdata::F64_1_05)); }
                    } else if status == 0x20 || contract.happiness() as i32 > 40 - p.loyalty() as i32 {
                        price = asking_step(price, 100_000, read_f64(rdata::F64_1_1));
                        if same_div() { price = asking_step(price, 25_000, read_f64(rdata::F64_1_025)); }
                    }
                }
            } else if g != 0 && morale <= 0 {
                if g & 0x101 != 0 && days_left() > 425 {
                    match status {
                        0x10 => {
                            price = asking_step(price, 500_000, read_f64(rdata::F64_1_25));
                            if same_div() { price = asking_step(price, 250_000, read_f64(rdata::F64_1_1)); }
                        }
                        0x20 => {
                            price = asking_step(price, 1_000_000, read_f64(rdata::F64_1_2));
                            if same_div() { price = asking_step(price, 200_000, read_f64(rdata::F64_1_05)); }
                        }
                        0x30 => {
                            price = asking_step(price, 750_000, read_f64(rdata::F64_1_15));
                            if same_div() { price = asking_step(price, 150_000, read_f64(rdata::F64_1_02)); }
                        }
                        _ => {}
                    }
                } else if days_left() > 225 && status != 0x70 && p.loyalty() as i32 + p.professionalism() as i32 > 10
                    && !sheet.is_null() && sheet.current_reputation() > 5250 && contract.happiness() > 10i8.wrapping_sub(p.loyalty())
                    && age > 22 && age < 31 {
                    price = asking_step(price, 750_000, read_f64(rdata::F64_1_15));
                    if same_div() { price = asking_step(price, 150_000, read_f64(rdata::F64_1_02)); }
                }
            } else {
                match status {
                    0x10 => {
                        price = asking_step_int(price, 2_000_000, 2);
                        if same_div() { price = asking_step(price, 100_000, read_f64(rdata::F64_1_1)); }
                    }
                    0x20 => {
                        price = asking_step(price, 1_500_000, read_f64(rdata::F64_1_5));
                        if same_div() { price = asking_step(price, 75_000, read_f64(0x968f90)); } // 1.075
                    }
                    0x30 => {
                        price = asking_step(price, 1_000_000, read_f64(rdata::F64_1_5));
                        if same_div() { price = asking_step(price, 20_000, read_f64(rdata::F64_1_02)); }
                    }
                    _ => {}
                }
            }
        }
    } else if entry.is_null() && price > value {
        // no shortlist entry outside the big nations' conditions: the price comes back towards the value
        covered!("mka_2 pull to value");
        price = if h < 60 {
            ftol(value as f64 * read_f64(rdata::F64_0_67) + price as f64 * read_f64(rdata::F64_0_33))
        } else {
            let t = read_f64(0x968f70) - hf * read_f64(rdata::F64_0_01); // 1.27
            ftol((read_f64(rdata::F64_1) - t) * price as f64 + value as f64 * t)
        };
    }
    // the youngster floor (0x4d3886..0x4d402e)
    if age < 24 && flag != 0 && seen > 110 && !nation.is_null() && status != 0x70 {
        let mut bonus: i32 = 0;
        let mut si = seen;
        if sheet.goalkeeper() != 20 {
            let af = read_f64(0x968f60) - age as f64 * read_f64(0x968f68); // 2.6, 0.07
            let pace = (sheet.player_pace() as u8).wrapping_add(sheet.acceleration() as u8) as i8;
            let physical = ftol((sheet.stamina() as f64 * read_f64(rdata::F64_1_2) + sheet.strength() as f64) * af
                + sheet.teamwork() as f64 * read_f64(rdata::F64_0_8) + sheet.work_rate() as f64 * read_f64(rdata::F64_1_2)
                + sheet.jumping() as f64 * read_f64(rdata::F64_0_75)) as i8;
            let mut dribbling = ftol(exe::staff_get_player_attribute(p, 11) as f64 * af) as i8;
            if dribbling >= 20 {
                dribbling = 20;
            }
            let mut finishing = ftol(exe::staff_get_player_attribute(p, 12) as f64 * af) as i8;
            if finishing >= 20 {
                finishing = 20;
            }
            let sq = |a: i8, b: i32| ((a as i32 - b) * (a as i32 - b)) as i16;
            let sqf = |a: i8, f: f64| ftol(((a as i32 - 15) * (a as i32 - 15)) as f64 * f) as i16;
            if pace == 40 {
                si = seen.wrapping_add(if physical > 85 { 15 } else if physical > 75 { 10 } else if physical > 65 { 8 } else if physical > 50 { 5 } else if physical > 35 { 3 } else { 0 });
                if dribbling > 15 { si = si.wrapping_add(sq(dribbling, 13)); }
                if finishing > 15 { si = si.wrapping_add(sq(finishing, 14)); }
            } else if pace > 34 {
                si = seen.wrapping_add(if physical > 75 { 8 } else if physical > 65 { 5 } else if physical > 50 { 3 } else if physical > 35 { 1 } else { 0 });
                if dribbling > 15 { si = si.wrapping_add(sq(dribbling, 15)); }
                if finishing > 15 { si = si.wrapping_add(sqf(finishing, read_f64(rdata::F64_0_8))); }
            } else if pace > 25 {
                si = seen.wrapping_add(if physical > 75 { 5 } else if physical > 65 { 3 } else if physical > 50 { 1 } else if physical < 35 { -3 } else { 0 });
                if dribbling > 15 { si = si.wrapping_add(sqf(dribbling, read_f64(rdata::F64_0_8))); }
                if finishing > 15 { si = si.wrapping_add(sqf(finishing, read_f64(rdata::F64_0_5))); }
            } else if pace < 15 {
                si = seen.wrapping_sub(if physical > 50 { 5 } else if physical > 35 { 8 } else { 10 });
            }
        }
        if !entry.is_null() && target == read_i32(p.addr()) {
            si = si.wrapping_add((rep / 1000) as i16);
        }
        match status {
            0x10 => {
                if age < 19 && (rep / 500 + si as i32) < 121 {
                    si = 121;
                } else {
                    si = si.wrapping_add((rep / 500) as i16);
                }
            }
            0x20 | 0x30 => si = si.wrapping_add((if age >= 19 { rep / 670 } else { rep / 500 }) as i16),
            0x40 if age < 19 => si = si.wrapping_add((rep / 670) as i16),
            _ => {}
        }
        if age > 20 && sheet.current_ability() < 45 {
            si = si.wrapping_add((rep / -1000) as i16);
        }
        if seen > 120 && seller.training() > 10 {
            si = si.wrapping_add(seller.training() as i16 - 10);
        }
        let manager = seller.manager();
        if !manager.is_null() && !manager.non_player().is_null() {
            if exe::human_manager_is_human_club(seller, 1, 1, 0, 0) == 0 {
                covered!("mka_2 manager youngsters");
                si = si.wrapping_add(manager.non_player().youngsters() as i16 - 10);
            }
            if exe::date_operator_3e(p.addr() + staff::DATE_JOINED_CLUB, read_u32(manager.addr() + staff::DATE_JOINED_CLUB), read_u32(manager.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) != 0 {
                let pa = sheet.potential_ability();
                if si < pa {
                    si = si.wrapping_add(ftol((pa as i32 - si as i32) as f64 * read_f64(rdata::F64_0_5)) as i16);
                }
            }
        }
        if si > 120 {
            covered!("mka_2 youngster floor");
            let mut c = rep as f64 * read_f64(0x968f58) + read_f64(rdata::F64_100); // 0.0167
            if read_f64(rdata::F64_150) > c {
                c = read_f64(rdata::F64_150);
            } else if read_f64(rdata::F64_220) < c {
                c = read_f64(rdata::F64_220);
            }
            let c = ftol(c) as i16;
            if si >= c {
                si = c;
            }
            if si > 150 && !entry.is_null() && rep >= 2500 && (nation.league_standard() as i8) > 10 {
                let mut t = (si as i32).wrapping_mul(4) - 600;
                if t >= 200 {
                    t = 200;
                }
                let t = t as i16 as i32;
                bonus = ftol(t.wrapping_mul(t).wrapping_mul(250) as f64 * (rep as f64 * read_f64(rdata::F64_0_0001666666667)));
            }
            let s2 = (si as i32).wrapping_mul(si as i32).wrapping_sub(14400).wrapping_mul(250);
            let mut y = ftol(s2 as f64 * read_f64(0x968f38) + bonus as f64); // 0.6153846
            let ls = nation.league_standard() as i8;
            if ls < 13 || seller.professional_status() as i8 == 3 {
                let mut r = seller.reputation();
                if seller.professional_status() as i8 == 3 && r < 5250 {
                    r = ftol(r as f64 * read_f64(rdata::F64_0_75)) as i16;
                }
                if ls < 5 && r > 4000 {
                    r = (4000 - ftol((r as i32 - 4000) as f64 * read_f64(rdata::F64_NEG_0_2))) as i16;
                }
                if r < 5250 {
                    y = ftol(r as f64 * read_f64(0x968f28) * y as f64); // 1/5250
                }
            }
            if g != 0 {
                if morale < -50 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_75));
                }
                if g & 0x80_0000 != 0 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_8));
                }
                if exe::staff_contract_get_flags(contract) & 8 != 0 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_95));
                }
                let g2 = contract.grievances();
                if g2 & 1 != 0 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_9));
                }
                if g2 & 0x40 != 0 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_9));
                }
            }
            if p.loyalty() < 6 && p.professionalism() < 10 {
                if contract.morale() < 0 {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_9));
                }
                if !buyer.is_null() && buyer.reputation() as i32 - 750 > rep {
                    y = ftol(y as f64 * read_f64(rdata::F64_0_9));
                }
            } else if p.loyalty() > 15 && contract.morale() >= 0 {
                y = ftol(y as f64 * read_f64(rdata::F64_1_1));
            }
            if price < y {
                price = y;
                mark(0x2000);
            }
        }
    }
    // the age cap (0x4d402e..0x4d4159)
    let mut a = age;
    if sheet.goalkeeper() == 20 {
        a = a.wrapping_sub(3);
    }
    let over = match a {
        29 => Some(5_000_000),
        30 => Some(4_000_000),
        31 => Some(3_250_000),
        32 => Some(2_500_000),
        33 => Some(1_750_000),
        34 => Some(1_000_000),
        35 => Some(500_000),
        36 => Some(200_000),
        37 => Some(100_000),
        38..=45 => Some(0),
        _ => None,
    };
    if let Some(over) = over {
        let most = value.wrapping_add(over);
        if price >= most {
            covered!("mka_2 age cap");
            price = most;
        }
    }
    // Portugal and South Korea (0x4d4159..0x4d422a)
    if !nation.is_null() {
        if is(KEY_NATION_PORTUGAL) {
            if price > value {
                let pf = price as f64;
                let x = if price < 6_000_000 {
                    (pf * read_f64(rdata::F64_1_666666667ENEG_07) * read_f64(rdata::F64_0_6) + read_f64(rdata::F64_0_3)) * pf
                } else {
                    pf * read_f64(rdata::F64_0_9)
                };
                price = ftol(if value as f64 > x { value as f64 } else { x });
            }
        } else if is(KEY_NATION_SOUTH_KOREA) && price > value && price > 50_000 {
            price = ftol(price.wrapping_sub(50_000) as f64 * read_f64(rdata::F64_0_15) + read_f64(rdata::F64_50000));
        }
    }
    if exe::staff_contract_has_active_release_clause(contract) != 0 && price >= contract.release_fee() as i32 {
        return contract.release_fee() as i32;
    }
    price
}

/// Comparer for `calc_asking_price`: side by side from the same draw state, checking the result, the reasons and the
/// regions of `calc_player_value` (the draw state, `G_CONTRACT_LOOKUP_RESULT` and the date statics).
pub(crate) unsafe extern "thiscall" fn compare_calc_asking_price(this: usize, seller: usize, buyer: usize, player: usize, reasons: *mut u16, adjust: u32, flag: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16, u32, u32) -> i32>(IDX_CALC_ASKING_PRICE);
    let mut reg = player_value_regions().to_vec();
    if !reasons.is_null() {
        reg.push((reasons as usize, 2));
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, seller, buyer, player, reasons, adjust, flag),
        || crate::rng::without_counting(|| calc_asking_price(this, seller, buyer, player, reasons, adjust, flag)));
    SCOREBOARD.record(equal && r == n, || format!("calc_asking_price(seller {:#x}, buyer {:#x}, player {:#x}, adjust {}, flag {}) original={} rust={} {:x?} x {:x?}", seller, buyer, player, adjust as u8, flag, r, n, mo, mr));
    r
}

// ---------- batch 288: the AI buyer reviewing its bid after the seller's counter offer (transfer_manager.cpp) ----------

/// The unsettling factor of a player whose AI buyer walked away (the `mood` score of the original, built from the
/// contract's `.grievances()`, `.ambition()`, `.loyalty()`, liking the selling club and `.age()`): above 10 → 0.9,
/// above 8 → 0.95, above 5 → 1.0, above 3 → 1.1, above 1 → 1.25, 1 → 1.5, 0 → 1.75, −2 or −1 → 2.0, below −2 → 2.5
/// (the f32 constants of the original).
unsafe fn review_bid_unsettle_factor(player: Staff, seller: Club, grievances: u32) -> f64 {
    let mut mood: i8 = if grievances != 0 { 5 } else { 0 };
    if grievances & 0x101 != 0 {
        mood = mood.wrapping_add(5);
    }
    let ambition = player.ambition();
    if ambition == 20 {
        mood = mood.wrapping_add(2);
    } else if ambition > 15 {
        mood = mood.wrapping_add(1);
    }
    let loyalty = player.loyalty();
    if loyalty == 1 {
        mood = mood.wrapping_add(3);
    } else if loyalty < 5 {
        mood = mood.wrapping_add(1);
    } else if loyalty == 20 {
        mood = mood.wrapping_sub(3);
    } else if loyalty > 15 {
        mood = mood.wrapping_sub(1);
    }
    if exe::staff_favours_club(player, seller) != 0 {
        mood = mood.wrapping_sub(2);
    } else if exe::staff_dislikes_club(player, seller) != 0 {
        mood = mood.wrapping_add(2);
    }
    let age = player.age() as i8;
    if age < 20 {
        mood = mood.wrapping_sub(4);
    } else if age < 23 {
        mood = mood.wrapping_sub(2);
    }
    let k = if mood > 10 {
        rdata::F32_0_8999999762
    } else if mood > 8 {
        rdata::F32_0_9499999881
    } else if mood > 5 {
        rdata::F32_1
    } else if mood > 3 {
        rdata::F32_1_100000024
    } else if mood > 1 {
        rdata::F32_1_25
    } else if mood == 0 {
        rdata::F32_1_75
    } else if mood < -2 {
        rdata::F32_2_5
    } else if mood < 0 {
        rdata::F32_2
    } else {
        rdata::F32_1_5
    };
    read_f32(k) as f64
}

/// The fee level above which the player hears of the bid: his value `v` for the selling club scaled (v × 8 below
/// 500,000; (v − 500,000) × 2 + 2,000,000 below 2,500,000; (v − 2,500,000) × 1.25 + 6,000,000 above), times the
/// unsettling factor.
unsafe fn review_bid_unsettle_level(v: i32, factor: f64) -> i32 {
    let v_f = v as f64;
    let x = if v < 500_000 {
        v_f * read_f64(rdata::F64_8)
    } else if v < 2_500_000 {
        (v_f - read_f32(0x96b000) as f64) * 2.0 + read_f64(rdata::F64_2000000) // f32 500,000
    } else {
        (v_f - read_f32(0x96aff0) as f64) * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_6000000) // f32 2,500,000
    };
    ftol(factor * ftol(x) as f64)
}

/// The player at the selling club of a bid his AI buyer walked away from may become unsettled: with a contract
/// (`contract_of_staff_cached`, not of type 5), a selling club, a contract with it (`exe::contract_manager_get_contract_for_club`)
/// and a buyer; at the club for more than (`.loyalty()` + 42) × 5 + `.professionalism()` days since
/// `.date_joined_club()` (`exe::date_days_since`); and the buyer's `.reputation()` above the seller's + 1250, or
/// listed by request (`exe::staff_is_listed_by_request`), or contract flag 8 (`exe::staff_contract_get_flags`), or
/// `.grievances()` bits 0x14. The level is the player's value for the seller (`exe::contract_manager_calc_player_value`
/// with 0, −1, human seller) through `review_bid_unsettle_level`; a bid `w` above 17,500,000, or above 1.5 × the level,
/// or above 250,000 and the level with random(`.temperament()` + 1) = 0, sets the contract's event flag 0x800000
/// (`exe::staff_contract_apply_event_flags` with 0).
unsafe fn review_bid_maybe_unsettle(player: Staff, buyer: Club, w: i32) {
    let c = Contract(contract_of_staff_cached(player));
    if c.is_null() || c.contract_type() & 0x3f == 5 {
        return;
    }
    let seller = transfer_manager_club_by_id(c.club());
    if seller.is_null() {
        return;
    }
    let cs = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, seller));
    if cs.is_null() || buyer.is_null() {
        return;
    }
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(player.addr() + staff::DATE_JOINED_CLUB), read_u32(player.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    if days <= (player.loyalty() as i32 + 42) * 5 + player.professionalism() as i32 {
        return;
    }
    if !((seller.reputation() as i32 + 1250) < buyer.reputation() as i32
        || exe::staff_is_listed_by_request(player, 0) != 0
        || exe::staff_contract_get_flags(cs) & 8 != 0
        || cs.grievances() as u8 & 0x14 != 0)
    {
        return;
    }
    let human = exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0;
    let v = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), player, seller, 0, -1, human as u32);
    let level = review_bid_unsettle_level(v, review_bid_unsettle_factor(player, seller, cs.grievances()));
    covered!("mkb_1 player hears of the bid");
    let unsettle = w > 17_500_000 // mkb1: the fee that always unsettles
        || (level as f64 * read_f64(rdata::F64_1_5)) < w as f64
        || (w > 250_000 && w > level && exe::utils_random(player.temperament() as i32 + 1) == 0);
    if unsettle {
        exe::staff_contract_apply_event_flags(cs, 0x800000, 0);
    }
}

/// The AI buyer reviews its bid after the selling club's answer: it values the target from its shortlist slot and,
/// when the price asked (the offer's weighted value plus the asking price of each exchange player kept) is above that
/// value, withdraws (and the target may hear of it); otherwise it tops the fee up with what the dropped exchange
/// players were worth, up to the value, and resubmits the bid if the transfer budget covers it.
///
/// `TRANSFER_MANAGER::review_ai_club_bid(slot, keep_existing)`, from `process_due_offer` (0x8c63a0) for a transfer bid
/// with status `offer_status::ON_HOLD` and an AI buyer (keep_existing 0), from the human manager's holiday answers
/// (0x784100), `handle_player_rejected_terms` (0x8c28c0) and the exchange-player dialog (0x8f2870). With keep_existing
/// 0 the offer reviewed is the seller's counter offer, the negotiation `.negotiation_index()` in
/// `transfer_manager_fields::NEGOTIATIONS` (the first one when the index is not below
/// `transfer_manager_fields::NEGOTIATION_CAPACITY`); with no negotiation (−1) the bid gets status 0x10 and is scheduled
/// (`exe::transfer_manager_schedule_offer` with 1, 0). Otherwise the bid in `slot` itself.
///
/// A local offer (`exe::new_transfer_offer`) takes a copy of it (the due date by `exe::date_operator_3d`; the low
/// nibbles of `offer::COMPLETION_TIMING` and `offer::OFFER_KIND` only), with +0 = 0, `.negotiation_index()` = −1 and
/// its own copy of the exchange players (`exe::new_llist`, `exe::llist_copy_from`). The buyer's shortlist slot for the
/// target (`exe::shortlist_manager_find_club_shortlist_entry` with the bid's `.job_type()` ≠ 11): none, the bid is
/// withdrawn (`exe::transfer_manager_withdraw_transfer_offer` with 0, 2). The value: `exe::shortlist_manager_value_player_for_club`
/// (−1, 1); the price `w`: `exe::transfer_offer_get_weighted_value` (0, 1, 1, 1, 1); each exchange player's asking
/// price (`exe::contract_manager_calc_asking_price` with his contract's club, 0, 1, 1) is added to `w` unless that
/// passes the value, when the player leaves the local offer (`exe::llist_remove`) and his price is counted as dropped;
/// an empty list is freed. The local offer's kind becomes 2 (low byte & 0xf2 | 2).
///
/// `w` above the value: the target's status in the shortlist (`exe::shortlist_manager_update_staff_status` with 1,
/// slot role ≠ 11), the withdrawal and `review_bid_maybe_unsettle`. Otherwise: the fee (`.fee()`) + min(dropped, value −
/// `w`, never negative); the total (`exe::transfer_offer_get_total_value` with 0, 1, 0, 1) above the buyer's transfer
/// budget (`exe::finance_get_transfer_budget` with the target, 0, 0, 1, 0): the status and the withdrawal; within it
/// and keep_existing 0, the local offer is submitted as a new bid (`exe::transfer_manager_add_transfer_offer`). The
/// local offer is destroyed (`exe::drop_transfer_offer`). The original's SEH frame only matters for exceptions.
/// Original: `TRANSFER_MANAGER::review_ai_club_bid` (0x8c0810, transfer_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn review_ai_club_bid(this: usize, slot: usize, keep_existing: u32) {
    let mut storage = [0u8; 0x44];
    let local = Offer(storage.as_mut_ptr() as usize);
    exe::new_transfer_offer(local);
    review_ai_club_bid_body(this, slot, keep_existing, local);
    exe::drop_transfer_offer(local);
}

/// The local offer takes a copy of `src` as the original copies it (0x8c0896..0x8c0980).
unsafe fn review_bid_copy_offer(local: Offer, src: usize) {
    let l = local.addr();
    for k in [0usize, 4, 8, 0xc, 0x1b, 0x20, 0x24, 0x28, 0x35, 0x3a] {
        write_u32(l + k, read_u32(src + k));
    }
    for k in [0x10usize, 0x11, 0x1a, 0x1f, 0x2c, 0x2d, 0x2e, 0x2f, 0x39, 0x3e, 0x3f, 0x40] {
        write_u8(l + k, read_u8(src + k));
    }
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(l + offer::DUE_DATE, scratch.as_mut_ptr() as usize, src + offer::DUE_DATE);
    write_u8(l + offer::COMPLETION_TIMING, (read_u8(l + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(src + offer::COMPLETION_TIMING) & 0xf));
    write_u32(l + offer::OFFER_KIND, (read_u32(l + offer::OFFER_KIND) & !0xf) | (read_u32(src + offer::OFFER_KIND) & 0xf));
    write_u32(l, 0);
    write_u32(l + offer::NEGOTIATION_INDEX, 0xffff_ffff);
}

/// The body of `review_ai_club_bid` between the construction and the destruction of the local offer.
unsafe fn review_ai_club_bid_body(this: usize, slot: usize, keep_existing: u32, local: Offer) {
    let mut source = slot;
    if keep_existing == 0 {
        let bid = Offer(read_usize(slot));
        let k = bid.negotiation_index() as i32;
        if k == -1 {
            bid.set_status(0x10);
            exe::transfer_manager_schedule_offer(this, bid, 1, 0);
            return;
        }
        let base = read_usize(this + transfer_manager_fields::NEGOTIATIONS);
        source = if k < read_i32(this + transfer_manager_fields::NEGOTIATION_CAPACITY) { base.wrapping_add((k as usize).wrapping_mul(4)) } else { base };
    }
    let src = read_usize(source);
    review_bid_copy_offer(local, src);
    if !local.exchange_players().is_null() {
        let p = exe::crt_operator_new(0xe);
        let list = if p != 0 { exe::new_llist(p) } else { 0 };
        write_usize(local.addr() + offer::EXCHANGE_PLAYERS, list);
        exe::llist_copy_from(list, Offer(read_usize(source)).exchange_players());
    }
    let src = Offer(read_usize(source));
    let player = transfer_manager_staff_by_id(src.target_staff_id());
    let buyer = transfer_manager_club_by_id(src.buyer_club_id());
    let sm = read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER);
    let job_is_player = Offer(read_usize(slot)).job_type() == 0xb;
    let entry = exe::shortlist_manager_find_club_shortlist_entry(sm, buyer, player, (!job_is_player) as u32, 0);
    if entry == 0 {
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
        return;
    }
    let mut scratch: i32 = 0;
    let value = exe::shortlist_manager_value_player_for_club(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), buyer, player, entry, &mut scratch, -1, 1);
    let mut w = exe::transfer_offer_get_weighted_value(local, 0, 1, 1, 1, 1);
    let mut dropped: i32 = 0;
    let list = local.exchange_players();
    if !list.is_null() {
        list.set_cursor(list.head());
        let mut item = exe::llist_get_next(list);
        while item != 0 {
            let other = transfer_manager_staff_by_id(read_usize(item));
            let c = contract_of_staff_cached(other);
            let club = if c == 0 { 0 } else { transfer_manager_club_by_id(read_usize(c + contract::CLUB)).addr() };
            let ask = exe::contract_manager_calc_asking_price(read_usize(G_CONTRACT_MANAGER), buyer, club, other, 0, 1, 1);
            if ask.wrapping_add(w) > value {
                covered!("mkb_1 exchange player dropped");
                dropped = dropped.wrapping_add(ask);
                exe::llist_remove(list, item);
            } else {
                w = ask.wrapping_add(w);
            }
            item = exe::llist_get_next(local.exchange_players());
        }
        let list = local.exchange_players();
        if read_i16(list.addr()) == 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(local.addr() + offer::EXCHANGE_PLAYERS, 0);
        }
    }
    write_u8(local.addr() + offer::OFFER_KIND, (read_u8(local.addr() + offer::OFFER_KIND) & 0xf2) | 2);
    let role_not_player = (read_u8(entry) != 0xb) as u32;
    if w > value {
        // mkb1: the price above the buyer's value of the target
        covered!("mkb_1 buyer walks away");
        exe::shortlist_manager_update_staff_status(sm, player, buyer, 1, role_not_player);
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
        review_bid_maybe_unsettle(player, buyer, w);
        return;
    }
    let room = value.wrapping_sub(w);
    if dropped >= room {
        dropped = room;
    }
    if dropped < 0 {
        dropped = 0;
    }
    write_u32(local.addr() + offer::FEE, read_u32(local.addr() + offer::FEE).wrapping_add(dropped as u32)); // mkb1: the fee topped up
    let total = exe::transfer_offer_get_total_value(local, 0, 1, 0, 1);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), buyer);
    let budget = exe::finance_get_transfer_budget(finance, player.addr() as u32, 0, 0usize, 1, 0);
    if budget < total as i64 {
        // mkb1: the total above the transfer budget
        covered!("mkb_1 over budget");
        exe::shortlist_manager_update_staff_status(sm, player, buyer, 1, role_not_player);
        exe::transfer_manager_withdraw_transfer_offer(this, slot as *mut usize, 0, 2);
    } else if keep_existing == 0 {
        covered!("mkb_1 bid resubmitted");
        exe::transfer_manager_add_transfer_offer(this, local.addr() as *mut u8, core::ptr::null_mut());
    }
}

/// Comparer for `review_ai_club_bid`: the original only (it withdraws and submits offers, moves shortlist slots and
/// may unsettle a contract; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_review_ai_club_bid(this: usize, slot: usize, keep_existing: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_REVIEW_AI_CLUB_BID)(this, slot, keep_existing) // simplification: original only
}

// ---------- batch 288: the AI's squad assessment (shortlist_manager.cpp) ----------

/// The locals of `assess_club_squad` that live across its stages (the original keeps them in a 0x8a4-byte frame;
/// the stack offsets are in the comments).
struct SquadAssessment {
    this: usize,
    entry: usize,
    club: Club,
    reset: bool,
    flags: u32,            // +0x44: 0x800 at a human club, 0x1802 at an AI club (the selection score's flags)
    partner: u32,          // +0x3c: the linked club (B team) or 0
    comp: usize,           // +0x18: the club's continental competition entry (only tested for null)
    target: i32,           // +0x64: the club's target rating (player_search_get_club_target_rating)
    target_base: i32,      // +0x74: its base, written by the same call
    days_since_start: i32, // +0x68: days since the game started
    stand_in: usize,       // +0x2c: the stand-in manager of a human club
    on_holiday: u32,       // +0x30: the human manager is on holiday
    may_sack: bool,        // +0x60: the shortlist was waiting (ACTIVE 2) at an AI club: idle staff may be sacked
    big_club: bool,        // +0x78: the slots were set up for a big club (status 3 for the first 19)
    rating: i32,           // +0x38: the last selection score (read again, stale, by the pricing of a slot of another status)
    pos_a: [u16; 50],      // +0x80: positions of the free slots of status 2 and 3
    n_a: i8,               // +0x40
    pos_b: [u16; 50],      // +0x468: status 4
    n_b: i8,               // +0x6c
    pos_c: [u16; 50],      // +0x274: status 5
    n_c: i8,               // +0x58
    chosen_a: [usize; 50], // +0x4cc
    chosen_b: [usize; 50], // +0x65c
    chosen_c: [usize; 50], // +0x594
    rest: [usize; 100],    // +0x2d8: the players not chosen yet
    n_rest: i8,            // bl, copied to +0x14
    elsewhere: [usize; 100], // +0xe4: players with a loan contract at another club
    n_elsewhere: i8,       // +0x13
    old: [usize; 50],      // +0x724: players too old for the slots of status 4
    n_old: i8,             // +0x13 again
}

/// The slot `k` (0..40) of the club's shortlist entry (`0x25` bytes each).
unsafe fn assess_slot(entry: usize, k: usize) -> ShortlistSlot {
    ShortlistSlot(read_usize(entry).wrapping_add(k.wrapping_mul(0x25)))
}

/// The slot's status (bits 1-4 of `shortlist_slot::FLAGS`): the original's four masks (`& 0xe5 | 4`, `& 0xe7 | 6`,
/// `& 0xe9 | 8`, `& 0xeb | 0xa`) all equal `& 0xe1 | status << 1`.
unsafe fn assess_set_slot_status(slot: ShortlistSlot, status: u8) {
    slot.set_flags((slot.flags() & 0xe1) | (status << 1));
}

/// Happiness plus `delta`, kept within −100..100 (the original adds in 32 bits and writes the byte).
fn assess_happiness_plus(h: i8, delta: i8) -> i8 {
    let e = h as i32 + delta as i32;
    if e > 100 {
        100
    } else if e < -100 {
        -100
    } else {
        h.wrapping_add(delta)
    }
}

/// `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, thiscall, ret 8) through the exe, so its hook applies.
unsafe fn assess_move_to_first_team(person: Staff, club: Club) {
    const MOVE_TO_FIRST_TEAM: usize = 0x4cc0b0;
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(MOVE_TO_FIRST_TEAM);
    f(read_usize(G_CONTRACT_MANAGER), person.addr(), club.addr());
}

/// `CONTRACT_MANAGER::move_to_reserves` for a person of the club.
unsafe fn assess_move_to_reserves(person: Staff, club: Club) {
    exe::contract_manager_move_to_reserves(read_usize(G_CONTRACT_MANAGER), person, club);
}

/// To the reserves, except a star (`player_sheet::WORLD_REPUTATION` ≥ 5000) at a club with a linked B team
/// (0x83dc47, 0x83dca1, 0x83e82f).
unsafe fn assess_to_reserves_unless_star(s: &SquadAssessment, p: Staff) {
    if s.partner == 0 || p.player().world_reputation() < 5000 {
        assess_move_to_reserves(p, s.club);
    }
}

/// The selection score of `p` for the position mask `pos` (`match_man_score_player_for_selection`, cdecl; the
/// original pushes the mask's word with whatever is in the upper half of the register, and the callee only tests the
/// low bits).
unsafe fn assess_score(s: &SquadAssessment, p: usize, pos: u32, flags: u32) -> i32 {
    exe::match_man_score_player_for_selection(p, pos, flags, s.club, -1, s.stand_in)
}

/// `match_man_assign_players_to_positions` (0x6e2000, cdecl, 15 arguments): the people of `rest` (count `n_rest`, a
/// byte) to the positions `pos` (count `n`, a byte), into `out`, with the manager `manager` (the stand-in at a human
/// club).
unsafe fn assess_pick_for(s: &mut SquadAssessment, which: u8, flags: u32) {
    let human = exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) != 0;
    let manager = if human { s.stand_in } else { s.club.manager().addr() };
    let flags = match which {
        0 if human => flags | 0xc100c0,
        0 => flags | 0xc0,
        1 => flags | 0x1c0,
        _ => flags | 0x100,
    };
    let (pos, n, out): (*const u16, i8, *mut usize) = match which {
        0 => (s.pos_a.as_ptr(), s.n_a, s.chosen_a.as_mut_ptr()),
        1 => (s.pos_b.as_ptr(), s.n_b, s.chosen_b.as_mut_ptr()),
        _ => (s.pos_c.as_ptr(), s.n_c, s.chosen_c.as_mut_ptr()),
    };
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    pick(s.rest.as_mut_ptr() as usize, s.n_rest as u8 as u32, pos as usize, n as u8 as u32, 0, out as usize, flags, s.club.addr(), 0, manager as u32, 1, 0, 0, 0, 0);
    let chosen = match which {
        0 => s.chosen_a,
        1 => s.chosen_b,
        _ => s.chosen_c,
    };
    // whoever was chosen leaves the rest: the last one takes his place (the one moved in is not compared again)
    for &p in chosen.iter().take(n.max(0) as usize) {
        if p == 0 {
            continue;
        }
        let mut i: i8 = 0;
        while i < s.n_rest {
            if s.rest[i as usize] == p {
                let last = (s.n_rest - 1) as usize;
                s.rest[i as usize] = s.rest[last];
                s.n_rest -= 1;
                s.rest[last] = 0;
            }
            i += 1;
        }
    }
}

/// The early checks of `assess_club_squad` (0x83cb40..0x83ccd0). `None`: the routine leaves (ACTIVE 0, or untouched
/// when it was 2 with bids out).
unsafe fn assess_start(this: usize, entry: usize, club: Club, daily: u32, reset: &mut bool) -> Option<(u32, bool)> {
    write_u8(entry + club_shortlist::DAYS, 0);
    let human = |a: u32| exe::human_manager_is_human_club(club, a, 1, 0, 0) != 0;
    if exe::database_is_national_team(club) != 0
        || club.nation().is_null()
        || (human(0) && daily as u8 != 0)
        || club.professional_status() == 3
    {
        write_u8(entry + club_shortlist::ACTIVE, 0);
        return None;
    }
    let mut partner: u32 = 0;
    if exe::club_get_linked_b_team(club, &mut partner, 1) == 0 {
        partner = 0;
    } else if partner == 0 {
        write_u8(entry + club_shortlist::ACTIVE, 0);
        return None;
    }
    let mut may_sack = false;
    if exe::shortlist_manager_count_slots_with_bids(this, club, 0xffff_ffff) != 0 {
        if read_u8(entry + club_shortlist::ACTIVE) == 2 {
            return None;
        }
    } else {
        if read_u8(entry + club_shortlist::ACTIVE) == 2 {
            let h = human(1);
            write_u8(entry + club_shortlist::ACTIVE, 1);
            *reset = true;
            may_sack = !h;
        }
        let division = club.division();
        let not_loaded = club.nation().league_selected() == 0
            || division.is_null()
            || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(division.addr()).wrapping_mul(4))) == 0;
        if not_loaded && club.reputation() < 4000 {
            let a = exe::utils_random(club.reputation() as i32 + 1000);
            let b = exe::utils_random(5000);
            if a < b {
                covered!("mkb_2 small club switches off");
                write_u8(entry + club_shortlist::ACTIVE, 0);
                return None;
            }
        }
    }
    write_u8(entry + club_shortlist::ACTIVE, 1);
    Some((partner, may_sack))
}

/// With `reset` the 40 slots are rebuilt (0x83cdf1..0x83cfe3): the formation (+7 of the entry) from the AI manager's
/// non-player record (3 without one, 0 at a human club or with the holiday option 2), the roles and positions
/// (`shortlist_set_up_formation_slots`), and for each slot its index, the date 1/1/1997 (day 1, year 0x7cd) and its
/// status: slot 0 status 2; a goalkeeper slot (position 1) among 1..23 status 4; otherwise, at an AI club without a B
/// team, a big club (a continental competition and base rating ≥ 6000, or base ≥ 8500) has status 3 up to slot 19 and 5
/// from 20, the others status 2 up to slot 10, 4 up to 19 and 5 from 20; with a B team or at a human club, base < 6000
/// without a B team is like the small ones and the rest status 3 up to slot 18, then 4 with a B team or 5 without.
unsafe fn assess_set_up_slots(s: &mut SquadAssessment) {
    let club = s.club;
    let manager = club.manager();
    let formation: u8 = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0
        && !manager.is_null()
        && (manager.non_player().is_null() || manager.non_player().formation() != 0)
        && !(s.on_holiday != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 2, exe::database_get_club_manager(club)) != 0)
    {
        let np = club.manager().non_player();
        if np.is_null() {
            3
        } else {
            np.formation() as u8
        }
    } else {
        0
    };
    write_u8(s.entry + 7, formation);
    const SET_UP_FORMATION_SLOTS: usize = 0x83c090; // stdcall, ret 4 (the original passes this in ecx, unused)
    let set_up: unsafe extern "stdcall" fn(usize) = core::mem::transmute(SET_UP_FORMATION_SLOTS);
    set_up(s.entry);
    for i in 0..40i8 {
        let slot = assess_slot(s.entry, i as usize);
        slot.set_index(i);
        let mut date = [0u32; 2];
        let mut scratch = [0u32; 2];
        let d = exe::new_date(date.as_mut_ptr() as usize, 1, 0x7cd);
        exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, d);
        let small = |i: i8| if i >= 11 { if i >= 20 { 5 } else { 4 } } else { 2 };
        let status = if i == 0 {
            2
        } else if i > 0 && i < 24 && slot.position() == 1 {
            4
        } else if s.partner == 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 {
            if (s.comp != 0 && s.target_base >= 6000) || s.target_base >= 8500 {
                s.big_club = true;
                if i >= 20 { 5 } else { 3 }
            } else {
                small(i)
            }
        } else if s.target_base < 6000 && s.partner == 0 {
            small(i)
        } else {
            s.big_club = true;
            if i < 19 {
                3
            } else if s.partner != 0 {
                4
            } else {
                5
            }
        };
        assess_set_slot_status(slot, status);
    }
}

/// The slots are released (0x83cff9..0x83d1c7): a slot without a target, or whose target has no bid
/// (`shortlist_slot::OFFER_REF` −1) and whose date has passed or who retired (job 0x10), loses target, replacement and
/// ceiling (the target, if he is in the manager's person pairs, gets the event 3 in `update_staff_status`); its
/// position goes to the list of its status (2 and 3, 4, 5).
unsafe fn assess_release_slots(s: &mut SquadAssessment) {
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        let t = slot.target_staff_id() as i32;
        if t > -1 {
            if slot.offer_ref() != -1 {
                continue;
            }
            let year = read_u16(slot.date_addr() + 2);
            let today = read_u16(G_CURRENT_DATE + 2);
            let expired = year < today
                || (year == today && read_i16(slot.date_addr()) < read_i16(G_CURRENT_DATE))
                || staff_at(t as isize).job_for_club() == 0x10;
            if !expired {
                continue;
            }
            let pairs = read_usize(s.this);
            if pairs != 0 {
                let key = [read_u32(staff_at(t as isize).addr()), if s.club.is_null() { 0xffff_ffff } else { read_u32(s.club.addr()) }];
                if exe::crt_bsearch(key.as_ptr() as usize, pairs, read_u32(s.this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0usize) != 0 {
                    exe::shortlist_manager_update_staff_status(s.this, staff_at(slot.target_staff_id() as i32 as isize), s.club, 3, 0);
                }
            }
        }
        slot.set_target_staff_id(usize::MAX);
        slot.set_replacement_staff_id(0xffff_ffff);
        slot.set_max_price(0);
        if read_u8(slot.addr()) == 0xb {
            let pos = slot.position() as u16;
            match (slot.flags() >> 1) & 0xf {
                2 | 3 => {
                    s.pos_a[s.n_a as usize] = pos;
                    s.n_a = s.n_a.wrapping_add(1);
                }
                4 => {
                    s.pos_b[s.n_b as usize] = pos;
                    s.n_b = s.n_b.wrapping_add(1);
                }
                5 => {
                    s.pos_c[s.n_c as usize] = pos;
                    s.n_c = s.n_c.wrapping_add(1);
                }
                _ => {}
            }
        }
    }
}

/// The squad is sorted out (0x83d1c7..0x83d474): each player of the club (`database_collect_club_squad`) with a
/// sheet, not retired, not on the club's own shortlist, with a contract at the club, no future transfer and not leaving
/// on a Bosman. With no manager on holiday: the transfer and loan marks are cleared, the squad status becomes 6 under
/// 20, else 7 at an AI club (cleared at a human one), and he goes to the first team; on holiday, with `reset` at an AI
/// club, only the status (6 under 20, cleared otherwise). An injury longer than 150 days (30 when random(recurring
/// level + 1) ≠ 0; 400 when his score reaches the target rating, 300 when it reaches 3/4 of it), or a retirement entry,
/// sends him to the reserves (with a B team only past 300 days out) and clears the status at a human club; the others
/// go to the rest, or to the list of those on loan elsewhere, and with `reset` their happiness becomes 0.
unsafe fn assess_sort_squad(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let mut squad = [0usize; 50];
    exe::database_collect_club_squad(s.club, squad.as_mut_ptr(), 0);
    for &p in squad.iter() {
        let p = Staff(p);
        if p.is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) || p.player().is_null() || p.job_for_club() == 0x10 {
            continue;
        }
        if exe::shortlist_manager_find_club_shortlist_entry(s.this, s.club, p, 0, 0) != 0 {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, s.club));
        if c.is_null() || exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 || c.leaving_on_bosman() != 0 {
            continue;
        }
        if s.on_holiday == 0 {
            c.set_transfer_status(c.transfer_status() & 0xfc);
            if (p.age() as i8) < 20 {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            } else if exe::human_manager_is_human_club(s.club, 0, 1, 0, 0) != 0 {
                c.set_squad_status(c.squad_status() & 0xf);
            } else {
                c.set_squad_status((c.squad_status() & 0xf) | 0x70);
            }
            assess_move_to_first_team(p, s.club);
        } else if s.reset && exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) == 0 {
            if (p.age() as i8) < 20 {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            } else {
                c.set_squad_status(c.squad_status() & 0xf);
            }
        }
        let injury = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), p));
        let level = exe::injury_get_recurring_injury_level(read_usize(G_INJURY), p) as i8;
        let long_out = if exe::utils_random(level as i32 + 1) != 0 {
            !injury.is_null() && injury.days_left() as i32 >= 30
        } else if injury.is_null() {
            false
        } else {
            s.rating = assess_score(s, p.addr(), 0xffff_ffff, s.flags | 0x6c0);
            let days = if s.rating >= s.target {
                400
            } else if s.target as f64 * read_f64(rdata::F64_0_75) <= s.rating as f64 {
                300
            } else {
                150
            };
            injury.days_left() as i32 >= days
        };
        if long_out || exe::player_regen_find_retirement_entry(read_usize(G_PLAYER_REGEN), p, 1) != 0 {
            if s.partner == 0 || (!injury.is_null() && injury.days_left() > 300) {
                assess_move_to_reserves(p, s.club);
            }
            if exe::human_manager_is_human_club(s.club, 0, 1, 0, 0) != 0 {
                c.set_squad_status(c.squad_status() & 0xf);
            }
            continue;
        }
        if exe::contract_manager_get_loan_contract(cm, p) != 0 && p.club_job() != s.club {
            s.elsewhere[s.n_elsewhere as usize] = p.addr();
            s.n_elsewhere = s.n_elsewhere.wrapping_add(1);
        } else {
            s.rest[s.n_rest as usize] = p.addr();
            s.n_rest = s.n_rest.wrapping_add(1);
        }
        if s.reset {
            c.set_happiness(0);
        }
    }
}

/// The squad is picked for the free slots (0x83d474..0x83d8ac): the rest goes to the positions of status 2 and 3,
/// then 4; those on loan elsewhere join the rest; those older than `player_sheet::GOALKEEPER` / 10 + 22 leave it
/// (the swapped-in one is looked at again); the rest goes to the positions of status 5; the old ones come back.
unsafe fn assess_pick_squad(s: &mut SquadAssessment) {
    if s.n_a != 0 {
        assess_pick_for(s, 0, s.flags);
    }
    if s.n_b != 0 {
        assess_pick_for(s, 1, s.flags);
    }
    for k in 0..s.n_elsewhere.max(0) as usize {
        s.rest[s.n_rest as usize] = s.elsewhere[k];
        s.n_rest = s.n_rest.wrapping_add(1);
    }
    s.n_old = 0;
    let mut i: i8 = 0;
    while i < s.n_rest {
        let p = Staff(s.rest[i as usize]);
        if !p.is_null() && (p.age() as i8 as i32) > p.player().goalkeeper() as i32 / 10 + 22 {
            s.old[s.n_old as usize] = p.addr();
            s.n_old = s.n_old.wrapping_add(1);
            let last = (s.n_rest - 1) as usize;
            s.rest[i as usize] = s.rest[last];
            s.n_rest -= 1;
            s.rest[last] = 0;
            i -= 1;
        }
        i += 1;
    }
    if s.n_c != 0 {
        assess_pick_for(s, 2, s.flags);
    }
    for k in 0..s.n_old.max(0) as usize {
        if s.old[k] != 0 {
            s.rest[s.n_rest as usize] = s.old[k];
            s.n_rest = s.n_rest.wrapping_add(1);
        }
    }
}

/// One player left over (0x83d8bf..0x83de6b). See `assess_club_squad` for the rule.
unsafe fn assess_rest_player(s: &mut SquadAssessment, p: Staff) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let club = s.club;
    if exe::contract_manager_get_loan_contract(cm, p) != 0 {
        if p.club_job() != club {
            assess_to_reserves_unless_star(s, p);
        }
        return;
    }
    if p.club_job() != club {
        return;
    }
    let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, club));
    if c.is_null() {
        return;
    }
    let age = p.age() as i8;
    if exe::transfer_manager_is_queued_for_free_transfer(tm, p) != 0 {
        if s.reset {
            c.set_happiness(-100);
        }
        return;
    }
    let age_i = age as i32;
    let threshold = ftol((age_i as f64 * read_f64(rdata::F64_0_13) - read_f64(0x96f890)) * s.target_base as f64 * read_f64(rdata::F64_0_02)) as i16; // f64 1.98
    let cut = s.target_base / 68 - 35;
    let sheet = p.player();
    let level = ftol(
        sheet.current_reputation() as f64 * read_f64(rdata::F64_0_25)
            + sheet.potential_ability() as f64 * read_f64(rdata::F64_12_5)
            + sheet.current_ability() as f64 * read_f64(rdata::F64_12_5)
            + club.reputation() as f64 * read_f64(rdata::F64_0_25),
    );
    let manager = club.manager();
    let rep = club.reputation() as i32;
    if p.job_for_club() == 0xb && exe::match_man_judge_player_ability(manager, p, club, 1, level as i16) < threshold {
        // below the manager's threshold
        let wage_left = || {
            let years = (c.contract_expires_year() as u16 as i32).wrapping_sub(read_u16(G_CURRENT_DATE + 2) as i32);
            let days = years.wrapping_mul(365).wrapping_add(c.contract_expires() as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
            days.wrapping_mul(c.wage()) / 30
        };
        let value = p.value() as i32;
        let surplus = (c.contract_type() & 0x3f == 2
            && (exe::date_days_since(G_CURRENT_DATE, read_u32(p.addr() + staff::DATE_JOINED_CLUB), read_u32(p.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR))
                <= p.professionalism() as i32 + p.pressure() as i32 + 450
                || exe::date_days_since(c.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) >= 450))
            || value >= rep.wrapping_mul(50)
            || value >= wage_left();
        if surplus {
            if exe::database_count_club_squad(club, 0, 0) as i8 as i32 > rep / 500 + 20 { // mkb2: the squad size that allows listing
                if s.reset {
                    c.set_happiness(-100);
                    c.set_unhappy_flag(0);
                    c.set_matches_left_out(0);
                    exe::contract_manager_lower_match_morale(cm, p, club, c);
                }
                if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0
                    && exe::match_man_judge_player_ability(manager, p, club, 2, level as i16) < cut as i16
                    && (age_i > p.player().goalkeeper() as i32 / 5 + 21 || p.player().current_reputation() as i32 > rep / 2)
                {
                    covered!("mkb_2 transfer listed");
                    const TRANSFER_LIST_PLAYER: usize = 0x842320; // thiscall, ret 8
                    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(TRANSFER_LIST_PLAYER);
                    f(s.this, p.addr(), !s.reset as u32);
                }
            }
        } else if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0
            && !s.reset
            && exe::transfer_manager_get_offers_for_player(tm, p, 0usize, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0usize) == 0
            && exe::transfer_manager_is_queued_for_free_transfer(tm, p) == 0
            && exe::database_count_club_squad(club, 0, 0) as i8 as i32 > rep / 500 + 25 // mkb2: the squad size that frees a contract
        {
            covered!("mkb_2 free transfer");
            const QUEUE_FREE_TRANSFER: usize = 0x8c16b0; // thiscall, ret 0xc
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(QUEUE_FREE_TRANSFER);
            f(tm, p.addr(), 0, 0xffff_ffff);
        }
        assess_to_reserves_unless_star(s, p);
        return;
    }
    // at or above the threshold (or not a player)
    if s.on_holiday != 0 {
        return;
    }
    if s.reset {
        c.set_happiness(-50);
        c.set_unhappy_flag(0);
        c.set_matches_left_out(0);
        exe::contract_manager_lower_match_morale(cm, p, club, c);
    }
    assess_to_reserves_unless_star(s, p);
    if s.partner == 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        let h = c.happiness();
        if h >= 0 || p.job_for_club() != 0xb {
            return;
        }
        let limit = if p.player().goalkeeper() == 20 {
            if age <= 19 {
                return;
            }
            age_i * 50 / 3 - 433
        } else {
            if age <= 17 {
                return;
            }
            age_i * 50 / 3 - 400 // mkb2: the unhappiness that puts a player on the loan list
        };
        if h as i32 > limit {
            return;
        }
        covered!("mkb_2 loan listed");
        c.set_transfer_status(c.transfer_status() | 2);
        if age < 24 {
            c.set_squad_status((c.squad_status() & 0xf) | 0x60);
        }
    } else {
        let too_old: i8 = if p.player().goalkeeper() == 20 { 27 } else { 23 };
        if age >= too_old {
            return;
        }
        let j = exe::match_man_judge_player_ability(manager, p, club, 1, ftol(level as i16 as f64 * read_f64(rdata::F64_0_75)) as i16);
        let status = if j as i32 > threshold as i32 + 20 { 0x50 } else { 0x60 };
        c.set_squad_status((c.squad_status() & 0xf) | status);
    }
}

/// The happiness of those picked for the slots of status 2 and 3 (0x83de85..0x83e185), with no manager on holiday:
/// for a picked player of the club with a contract, his score for the slot's position `own` against the best score of
/// the others (the other picks of that list, the picks of 4 and 5 and the rest) `best`: v = ftol((100 − min(best,
/// own) / max(own, 1) × 100) × 0.5) as a byte, then v + 20 − v/5; with `reset` happiness + v (within −100..100),
/// matches left out v/10 (at most 100) and the unhappy flag 0; above 20 the squad status 1 (v ≥ 30 and own above the
/// target rating) or 2. The original calls `get_contract_for_club` with a null person too.
unsafe fn assess_chosen_happiness(s: &mut SquadAssessment) {
    if s.n_a <= 0 || s.on_holiday != 0 {
        return;
    }
    let cm = read_usize(G_CONTRACT_MANAGER);
    for i in 0..s.n_a as usize {
        let p = Staff(s.chosen_a[i]);
        if !p.is_null() && p.club_job() != s.club {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, s.club));
        if c.is_null() || p.is_null() {
            continue;
        }
        let mut flags = s.flags | 0xc0;
        if exe::human_manager_is_human_club(s.club, 1, 1, 0, 0) != 0 {
            flags |= 0x810000;
        }
        let pos = s.pos_a[i] as u32;
        let own = assess_score(s, p.addr(), pos, flags);
        s.rating = own;
        let mut best = 0i32;
        let mut consider = |q: usize| {
            if q != 0 {
                let r = assess_score(s, q, pos, flags);
                if best <= r {
                    best = r;
                }
            }
        };
        for k in 0..s.n_a as usize {
            if k != i {
                consider(s.chosen_a[k]);
            }
        }
        for k in 0..s.n_b.max(0) as usize {
            consider(s.chosen_b[k]);
        }
        for k in 0..s.n_c.max(0) as usize {
            consider(s.chosen_c[k]);
        }
        for k in 0..s.n_rest.max(0) as usize {
            consider(s.rest[k]);
        }
        let m = if best > own { own } else { best };
        let d = if own < 1 { 1 } else { own };
        let x = ftol((read_f32(rdata::F32_100) as f64 - m as f64 / d as f64 * read_f64(rdata::F64_100)) * read_f32(rdata::F32_0_5) as f64) as i8;
        let v = x.wrapping_add(20i8.wrapping_sub((x as i32 / 5) as i8));
        if s.reset {
            c.set_happiness(assess_happiness_plus(c.happiness(), v));
            let q = (v as i32 / 10) as i8;
            c.set_matches_left_out((if q > 100 { 100 } else { q }) as u8);
            c.set_unhappy_flag(0);
        }
        if v > 20 {
            let status = if v >= 30 && own > s.target { 0x10 } else { 0x20 };
            c.set_squad_status((c.squad_status() & 0xf) | status);
        }
    }
}

/// From the date the picked player joined the club, if after the manager's, the slot's date is that + 120 days
/// (only with `reset` and a manager).
unsafe fn assess_slot_date(s: &SquadAssessment, slot: ShortlistSlot, p: Staff) {
    let manager = s.club.manager();
    if !s.reset || manager.is_null() {
        return;
    }
    let (year, manager_year) = (p.date_joined_club_year(), manager.date_joined_club_year());
    if year > manager_year || (year == manager_year && read_i16(p.addr() + staff::DATE_JOINED_CLUB) > read_i16(manager.addr() + staff::DATE_JOINED_CLUB)) {
        let mut later = [0u32; 2];
        let mut scratch = [0u32; 2];
        let d = exe::date_add_days(p.addr() + staff::DATE_JOINED_CLUB, later.as_mut_ptr() as usize, 0x78);
        exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, d);
    }
}

/// The picks fill the free slots in order (0x83e185..0x83e66f); see `assess_club_squad`.
unsafe fn assess_fill_slots(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let club = s.club;
    let mut k: usize = 0;
    for list in 0..3u8 {
        let (n, chosen) = match list {
            0 => (s.n_a, s.chosen_a),
            1 => (s.n_b, s.chosen_b),
            _ => (s.n_c, s.chosen_c),
        };
        for &p in chosen.iter().take(n.max(0) as usize) {
            while assess_slot(s.entry, k).target_staff_id() as i32 > -1 {
                k += 1;
            }
            let slot = assess_slot(s.entry, k);
            let p = Staff(p);
            if p.is_null() {
                slot.set_replacement_staff_id(0xffff_ffff);
                slot.set_target_staff_id(usize::MAX);
                k += 1;
                continue;
            }
            slot.set_replacement_staff_id(read_u32(p.addr()));
            let c = Contract(exe::contract_manager_get_contract_for_club(cm, p, club));
            if s.on_holiday == 0 && !c.is_null() {
                let st = c.squad_status() & 0xf0;
                if list != 0 || (st != 0x10 && st != 0x20) {
                    c.set_squad_status(((slot.flags() & 0xfe) << 3) | (c.squad_status() & 0xf));
                }
            }
            assess_slot_date(s, slot, p);
            k += 1;
            match list {
                0 => {}
                1 => {
                    if (s.partner != 0 || exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0) && k == 25 && p != club.manager() && !s.big_club {
                        assess_move_to_reserves(p, club);
                    }
                    let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p.addr()));
                    let sheet = p.player();
                    // Defect of the original: WORK_RATE is a signed byte and 3250 is out of its range, so this test
                    // is always true (probably WORLD_REPUTATION, a word, was meant); kept as the original does it.
                    if fatigue < -1000 && sheet.current_reputation() < 4500 && (sheet.work_rate() as i32) < 3250 && !c.is_null() && c.happiness() < -50 {
                        covered!("mkb_2 tired reserve loan listed");
                        c.set_transfer_status(c.transfer_status() | 2);
                    }
                }
                _ => {
                    if s.partner == 0 && p != club.manager() {
                        assess_move_to_reserves(p, club);
                    }
                    let r = exe::utils_random(300);
                    let fatigue = exe::injury_get_fatigue(read_usize(G_INJURY), read_u32(p.addr())) as i32;
                    if fatigue < -500 - r && !c.is_null() && c.happiness() < 0 {
                        covered!("mkb_2 tired spare loan listed");
                        c.set_transfer_status(c.transfer_status() | 2);
                    }
                }
            }
        }
    }
}

/// The player slots without a target are priced (0x83e66f..0x83ebfb); see `assess_club_squad`.
unsafe fn assess_price_slots(s: &mut SquadAssessment) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let club = s.club;
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        if slot.target_staff_id() as i32 > -1 || read_u8(slot.addr()) != 0xb {
            continue;
        }
        let mut price = if s.target >= 36000 { 36000 } else { s.target };
        if slot.position() == 1 && slot.replacement_staff_id() as i32 == -1 {
            price = ftol(price as f64 * read_f64(rdata::F64_0_75));
        }
        let replacement = slot.replacement_staff_id() as i32;
        if replacement <= -1 {
            slot.set_valuation(price);
            slot.set_target_staff_id(usize::MAX);
            slot.set_max_price(0);
            let (p, r) = (price as f64, club.reputation() as f64);
            let v = match (slot.flags() >> 1) & 0xf {
                1 => p * read_f64(rdata::F64_2_5) + r,
                2 => (p + p) + r,
                3 => p * read_f64(rdata::F64_1_8) + r,
                4 | 5 => p * read_f64(rdata::F64_0_8),
                _ => p * read_f64(rdata::F64_0_5),
            };
            slot.set_price(ftol(v));
            if read_u8(slot.addr() + shortlist_slot::POSITION) & 1 != 0 {
                slot.set_price(slot.price().wrapping_mul(3)); // a goalkeeper slot
            }
            continue;
        }
        let person = staff_at(replacement as isize);
        let c = Contract(exe::contract_manager_get_contract_for_club(cm, person, club));
        let pos = slot.position() as u16 as u32;
        match (slot.flags() >> 1) & 0xf {
            2 | 3 => {
                if !c.is_null() && s.reset {
                    c.set_happiness(assess_happiness_plus(c.happiness(), 50));
                    c.set_matches_left_out(10);
                    c.set_unhappy_flag(0);
                }
                assess_move_to_first_team(person, club);
                let flags = if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 { s.flags | 0x8100c0 } else { s.flags | 0xc0 };
                s.rating = assess_score(s, person.addr(), pos, flags);
                if s.days_since_start < 200 {
                    s.rating = ftol((read_f64(rdata::F64_1_25) - s.days_since_start as f64 * read_f64(0x96f888)) * s.rating as f64); // f64 0.00125
                }
            }
            4 => {
                assess_move_to_first_team(person, club);
                s.rating = assess_score(s, person.addr(), pos, s.flags | 0x1c0);
            }
            5 => {
                if person != club.manager() {
                    assess_to_reserves_unless_star(s, person);
                }
                s.rating = assess_score(s, person.addr(), pos, s.flags | 0x100);
            }
            _ => {} // the score of the previous slot or player (stale +0x38 in the original)
        }
        if s.rating < -10000 {
            s.rating = -10000;
        } else if s.rating > 60000 {
            s.rating = 60000;
        }
        let r = s.rating;
        let unhappy = || !c.is_null() && c.grievances() != 0 && (c.grievances() & 0x100 != 0 || c.grievances() & 1 != 0 || exe::staff_contract_days_to_end_date(c) < 365);
        if r >= price && !unhappy() {
            slot.set_target_staff_id(read_u32(person.addr()) as usize); // he keeps the place
            continue;
        }
        covered!("mkb_2 slot priced");
        let delta = ftol((read_f64(rdata::F64_1) - r as f64 / price as f64) * read_f64(0x96f880)) as i8; // f64 -100.0
        let st = slot.flags() & 0x1e;
        if !c.is_null() {
            if s.reset {
                c.set_happiness(assess_happiness_plus(c.happiness(), delta));
            }
            if st == 4 || st == 6 {
                if c.happiness() < 25 && s.reset {
                    c.set_happiness(25);
                }
                slot.set_target_staff_id(usize::MAX);
            } else if c.happiness() < 0 && s.reset {
                c.set_happiness(0);
            }
            if s.on_holiday == 0 && slot.flags() & 0x1e == 0xa {
                c.set_squad_status((c.squad_status() & 0xf) | 0x60);
            }
        } else if st == 4 || st == 6 {
            slot.set_target_staff_id(usize::MAX);
        }
        slot.set_valuation(price);
        slot.set_max_price(0);
        let v = if price > 1000 { price } else { 1000 };
        let x = read_f64(rdata::F64_1_25) - r as f64 / v as f64;
        let f = if x > read_f64(rdata::F64_1) {
            read_f64(rdata::F64_1)
        } else if x < read_f64(rdata::F64_0_5) {
            read_f64(rdata::F64_0_5)
        } else {
            x
        };
        let (p, rf, rr) = (price as f64, r as f64, club.reputation() as f64);
        let price_left = match (slot.flags() >> 1) & 0xf {
            1 => (rr * f + p * read_f64(rdata::F64_2_5)) - rf * read_f64(rdata::F64_2_5),
            2 => (rr * f + (p + p)) - (rf + rf),
            3 => (rr * f + p * read_f64(rdata::F64_1_8)) - rf * read_f64(rdata::F64_1_8),
            4 | 5 => p * read_f64(rdata::F64_0_8) - rf * read_f64(rdata::F64_0_8),
            _ => p * read_f64(rdata::F64_0_5) - rf * read_f64(rdata::F64_0_5),
        };
        slot.set_price(ftol(price_left));
    }
}

/// The staff slots (0x83ebfb..0x83f321); see `assess_club_squad`.
unsafe fn assess_staff_slots(s: &mut SquadAssessment) {
    let club = s.club;
    let mut staff = [0usize; 16];
    let mut n: i8 = 0;
    let mut add = |q: usize| {
        if q != 0 && exe::shortlist_manager_find_club_shortlist_entry(s.this, club, q, 1, 0) == 0 {
            staff[n as usize] = q;
            n += 1;
        }
    };
    add(club.assistant_manager());
    for k in 0..5 {
        add(read_usize(club.addr() + club::COACHES + 4 * k));
    }
    for k in 0..3 {
        add(read_usize(club.addr() + club::PHYSIOS + 4 * k));
    }
    for k in 0..7 {
        add(read_usize(club.addr() + club::SCOUTS + 4 * k));
    }
    let threshold = if s.target / 5 >= 5000 { 5000 } else { s.target / 5 };
    for k in 0..40usize {
        let slot = assess_slot(s.entry, k);
        if read_u8(slot.addr()) == 0xb {
            continue;
        }
        slot.set_flags(slot.flags() & 0xfe);
        slot.set_max_price(0);
        slot.set_price(ftol(threshold as f64 * read_f64(rdata::F64_2_5)));
        if slot.offer_ref() != -1 {
            continue;
        }
        slot.set_valuation(threshold);
        let jobs: [u8; 2] = match read_i8(slot.addr()) {
            6 => [6, 0xd],
            7 => [7, 0xe],
            8 => [8, 0xf],
            9 => [9, 9],
            10 => [10, 10],
            _ => continue,
        };
        let Some(i) = (0..n.max(0) as usize).find(|&i| jobs.contains(&Staff(staff[i]).job_for_club())) else {
            continue;
        };
        let person = Staff(staff[i]);
        slot.set_replacement_staff_id(read_u32(person.addr()));
        let manager = if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 { s.stand_in } else { club.manager().addr() };
        let score = exe::score_staff_for_job(read_usize(G_MANAGER_MANAGER), person, club, manager as u32, read_u8(slot.addr()) as u32);
        if score >= threshold {
            slot.set_target_staff_id(read_u32(person.addr()) as usize);
            slot.set_price(0);
        } else if s.may_sack && exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(person.addr()), read_u32(club.addr())) == 0 {
            covered!("mkb_2 staff sacked");
            slot.set_replacement_staff_id(0xffff_ffff);
            slot.set_target_staff_id(usize::MAX);
            exe::manager_manager_remove_staff_from_club(read_usize(G_MANAGER_MANAGER), person, club);
        } else {
            slot.set_target_staff_id(usize::MAX);
            slot.set_price(ftol((threshold - score) as f64 * read_f64(rdata::F64_2_5)));
        }
        n -= 1;
        staff[i] = staff[n as usize];
    }
}

/// The AI's assessment of a club's squad, done when its shortlist is rebuilt: it clears and sets the squad statuses,
/// sends players to the first team or the reserves, puts players on the transfer list, the loan list or the free
/// transfer queue, fills and prices the shortlist slots and may sack idle staff.
///
/// `Shortlist_Manager::assess_club_squad(entry, club, daily, reset)`, from `update_club_shortlist` (0x83a730; daily 1,
/// reset 0) when the club's list is redone, and with daily 0, reset 1 from `create_initial_shortlists` (0x83a1b0, at
/// career creation) and `rebuild_club_shortlist` (0x84abe0, a new manager). `entry` is the club's 9-byte record in
/// `shortlist_manager_fields::CLUB_SHORTLISTS` (+0 its 40 slots of 0x25 bytes, +4 `club_shortlist::ACTIVE`, +7 the
/// formation, +8 `club_shortlist::DAYS`).
///
/// 1. DAYS = 0; reset is forced when slot 0's `shortlist_slot::INDEX` is −1. It leaves with ACTIVE 0 for a national
///    team, a club without a nation, a human club when `daily`, `club::PROFESSIONAL_STATUS` 3, or a club whose linked B
///    team lookup (`exe::club_get_linked_b_team`) succeeds with no partner. With bids out
///    (`exe::shortlist_manager_count_slots_with_bids`) and ACTIVE 2 it leaves untouched. With no bids: ACTIVE 2 becomes
///    1 with reset, and idle staff may be sacked at an AI club; a club of an unloaded league (no `.league_selected()`,
///    no division or the division not in `G_COMP_OBJECT_TABLE`) with reputation below 4000 switches its list off
///    (ACTIVE 0) when random(reputation + 1000) < random(5000). Then ACTIVE = 1.
/// 2. The continental entry (`.continental_comp_id()`, or the club state's when none, or the Inter-Toto at a club of
///    reputation ≥ 5000), the target rating and its base (`player_search_get_club_target_rating`), the club's target
///    reputation (`update_club_target_reputation`) and, at a human club, the stand-in manager and the holiday state.
/// 3. With reset the slots are rebuilt (`assess_set_up_slots`); otherwise they are sorted by index (qsort, 0x83b6c0).
/// 4. `assess_release_slots`, `assess_sort_squad` and `assess_pick_squad` (`match_man_assign_players_to_positions`).
/// 5. **The rest** (50 entries): a player on loan here from another club goes to the reserves (except a star with a B
///    team); one of the club with a contract, not queued for a free transfer (with reset, queued ones get happiness
///    −100): the threshold is ftol((age × 0.13 − 1.98) × base × 0.02), the cut base / 68 − 35 and his level
///    ftol(current reputation × 0.25 + potential ability × 12.5 + current ability × 12.5 + club reputation × 0.25). A
///    player the manager judges below the threshold (`match_man_judge_player_ability` mode 1) is surplus if his
///    contract is of type 2 and he joined less than professionalism + pressure + 450 days ago or it ends in 450 days or
///    more, or `staff::VALUE` ≥ reputation × 50, or the value ≥ the wages left (days to the end × wage / 30). A surplus
///    player, with the squad (`database_count_club_squad`) above reputation / 500 + 20: with reset happiness −100 and
///    `contract_manager_lower_match_morale`; at an AI club, judged (mode 2) below the cut and older than goalkeeper / 5
///    + 21 or with current reputation above half the club's, **transfer listed** (`transfer_list_player`, news only
///    without reset). Otherwise, at an AI club without reset, no offers or exchange offers, not queued, and the squad
///    above reputation / 500 + 25: **free transfer queue** (`queue_free_transfer`). Both go to the reserves (except a
///    star with a B team). A player at or above the threshold, with no manager on holiday: with reset happiness −50
///    and lower morale; reserves (same exception); at an AI club without a B team, a player with negative happiness on
///    the **loan list** (`transfer_status::LOAN_LISTED`, status 6 under 24) when a goalkeeper rated 20 aged 20 or more
///    has happiness ≤ age × 50 / 3 − 433, or another player aged 18 or more ≤ age × 50 / 3 − 400; with a B team or at
///    a human club, under 27 (goalkeeper 20) or 23, the status 5 when judged with three quarters of his level above
///    the threshold + 20, else 6.
/// 6. `assess_chosen_happiness`, then the picks fill the free slots in order (`assess_fill_slots`): the replacement
///    is the pick, his contract takes the slot's status (list 2/3 not over statuses 1 and 2) when no manager is on
///    holiday, the slot date is set from his arrival (`assess_slot_date`). Status 4: at slot 24, with a B team or at a
///    human club and not a big club, the pick (not the manager) goes to the reserves; **loan list** when fatigue <
///    −1000, current reputation < 4500, `player_sheet::WORK_RATE` < 3250 and happiness < −50. **Defect of the
///    original:** WORK_RATE is a signed byte, so `< 3250` is always true (probably `WORLD_REPUTATION` was meant); kept.
///    Status 5: to the reserves without a B team (not the manager); **loan list** when fatigue < −500 − random(300)
///    and happiness < 0.
/// 7. `assess_price_slots`: each player slot without a target is priced from p = min(target rating, 36000) (× 0.75
///    for a goalkeeper slot without a replacement). Without a replacement: valuation p and price p × 2.5 + reputation,
///    2p + reputation, p × 1.8 + reputation, p × 0.8 or p × 0.5 by status 1, 2, 3, 4/5, other; × 3 for a goalkeeper
///    slot. With one: by the status he goes to the first team (2, 3 and 4; with reset happiness + 50, 10 matches left
///    out) or to the reserves (5) and is scored for the position (in the first 200 days × (1.25 − days × 0.00125)),
///    within −10000..60000; a score ≥ p with no grievances 0x100 or 1 and more than 364 days of contract keeps him as
///    the target. Otherwise his happiness moves by ftol((1 − score / p) × −100) (with reset; statuses 2 and 3 at least
///    25 and the slot loses him, the others at least 0; status 5 gets squad status 6), the valuation is p and the
///    price (reputation × f + p × k) − score × k for k 2.5, 2, 1.8 (statuses 1..3, f = 1.25 − score / max(p, 1000)
///    within 0.5..1), or p × k − score × k with k 0.8 (4/5) or 0.5.
/// 8. `assess_staff_slots`: the assistant, coaches, physios and scouts not on the club's list are matched to the staff
///    slots by job (6 or 13, 7 or 14, 8 or 15, 9, 10); threshold min(target / 5, 5000): every staff slot loses bit 0
///    and the ceiling and gets the price threshold × 2.5; without a bid the valuation is the threshold and the first
///    person of the job is scored (`score_staff_for_job`): at or above it he is the target (price 0); below it, when
///    staff may be sacked and he has no contract offer (`contract_manager_find_contract_offer`), he is **sacked**
///    (`manager_manager_remove_staff_from_club`); otherwise price (threshold − score) × 2.5.
///
/// The floating point is in f64 (the game's x87 at 53 bits) in the original's order of operations.
/// Original: `Shortlist_Manager::assess_club_squad` (0x83cb40, shortlist_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn assess_club_squad(this: usize, entry: usize, club: usize, daily: u32, reset: u32) {
    let club = Club(club);
    let mut reset = reset as u8 != 0;
    let days_since_start = exe::date_days_since(G_CURRENT_DATE, read_u32(G_GAME_START_DATE), read_u32(G_GAME_START_DATE_FLAGS));
    let flags: u32 = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { 0x800 } else { 0x1802 };
    if assess_slot(entry, 0).index() == -1 {
        reset = true;
    }
    let Some((partner, may_sack)) = assess_start(this, entry, club, daily, &mut reset) else {
        return;
    };
    let mut s = SquadAssessment {
        this,
        entry,
        club,
        reset,
        flags,
        partner,
        comp: 0,
        target: 0,
        target_base: 0,
        days_since_start,
        stand_in: 0,
        on_holiday: 0,
        may_sack,
        big_club: false,
        rating: 0, // uninitialised in the original until the first score
        pos_a: [0; 50],
        n_a: 0,
        pos_b: [0; 50],
        n_b: 0,
        pos_c: [0; 50],
        n_c: 0,
        chosen_a: [0; 50],
        chosen_b: [0; 50],
        chosen_c: [0; 50],
        rest: [0; 100],
        n_rest: 0,
        elsewhere: [0; 100],
        n_elsewhere: 0,
        old: [0; 50],
        n_old: 0,
    };
    let mut comp_id = club.continental_comp_id() as i32;
    if comp_id <= -1 || (comp_id == read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) && club.reputation() >= 5000) {
        comp_id = read_i32(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(6)));
    }
    if comp_id > -1 {
        s.comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as usize).wrapping_mul(0x6b));
    }
    const TARGET_RATING: usize = 0x7bca90; // player_search_get_club_target_rating, cdecl
    let target_rating: unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32 = core::mem::transmute(TARGET_RATING);
    s.target = target_rating(club.addr(), &mut s.target_base, core::ptr::null());
    const TARGET_REPUTATION: usize = 0x83bf40; // update_club_target_reputation, thiscall, ret 4
    let target_reputation: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(TARGET_REPUTATION);
    target_reputation(this, club.addr());
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        s.stand_in = exe::human_manager_get_stand_in_manager(club);
        let manager = exe::database_get_club_manager(club);
        s.on_holiday = exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), manager);
    }
    if s.reset {
        assess_set_up_slots(&mut s);
    } else {
        exe::crt_qsort(read_usize(entry), 0x28, 0x25, 0x83b6c0usize); // shortlist_compare_slots_by_index
    }
    assess_release_slots(&mut s);
    assess_sort_squad(&mut s);
    assess_pick_squad(&mut s);
    for k in 0..50 {
        let p = Staff(s.rest[k]);
        if !p.is_null() {
            assess_rest_player(&mut s, p);
        }
    }
    assess_chosen_happiness(&mut s);
    assess_fill_slots(&mut s);
    assess_price_slots(&mut s);
    assess_staff_slots(&mut s);
}

/// Comparer for `assess_club_squad`: the original only (random draws, moves people between first team and reserves,
/// lists players, may sack staff: it cannot run twice), and the plan is checked on exit: DAYS 0; ACTIVE 0 or 1, or 2
/// only if it was 2; after a full run (ACTIVE 1) every staff slot has bit 0 clear and no ceiling, and with `reset`
/// every slot has its own index.
pub(crate) unsafe extern "thiscall" fn compare_assess_club_squad(this: usize, entry: usize, club: usize, daily: u32, reset: u32) {
    let before = read_u8(entry + club_shortlist::ACTIVE);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ASSESS_CLUB_SQUAD)(this, entry, club, daily, reset);
    let active = read_u8(entry + club_shortlist::ACTIVE);
    let mut ok = read_u8(entry + club_shortlist::DAYS) == 0 && (active <= 1 || (active == 2 && before == 2));
    let mut bad_slot = usize::MAX;
    if active == 1 {
        for k in 0..40usize {
            let slot = assess_slot(entry, k);
            let staff_ok = read_u8(slot.addr()) == 0xb || (slot.flags() & 1 == 0 && slot.max_price() == 0);
            let index_ok = reset as u8 == 0 || slot.index() as usize == k;
            if !(staff_ok && index_ok) {
                ok = false;
                bad_slot = k;
                break;
            }
        }
    }
    SCOREBOARD.record(ok, || {
        format!("assess_club_squad(club {}, daily {}, reset {}) ACTIVE {} -> {}, DAYS {}, slot {:?}", read_i32(club), daily as u8, reset as u8, before, active, read_u8(entry + club_shortlist::DAYS), if bad_slot == usize::MAX { None } else { Some(bad_slot) })
    });
}

// ---------- batch 288: the AI choosing a shortlist slot's target (shortlist_manager.cpp) ----------

/// `Shortlist_Manager::is_compensation_deal` (0x84b6f0, Rust; thiscall, ret 0x10), called through its address.
unsafe fn fill_slot_is_compensation_deal(this: usize, slot: usize, contract: usize, club: Club, fee: &mut i32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32 = core::mem::transmute(0x84b6f0usize);
    f(this, slot, contract, club.addr(), fee)
}

/// `Media::add_transfer_interest` (0x754740, Rust; thiscall, ret 0x14), called through its address.
unsafe fn fill_slot_add_transfer_interest(player: Staff, club: Club, fee: i32, interest: f64) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, f64) = core::mem::transmute(0x754740usize);
    f(read_usize(G_MEDIA), player.addr(), club.addr(), fee, interest)
}

/// A date (day i16 at +0, year u16 at +2) before another, compared as the original does: the year unsigned, the day
/// signed.
unsafe fn fill_slot_date_before(a: usize, b: usize) -> bool {
    read_u16(a + 2) < read_u16(b + 2) || (read_u16(a + 2) == read_u16(b + 2) && read_i16(a) < read_i16(b))
}

/// Whether the shortlist manager's sorted (person, club) pairs hold `person` for `club`
/// (`exe::crt_bsearch` with the comparer 0x757ea0 over `PAIRS_BY_PERSON_COUNT` pairs; no array, no pair).
unsafe fn fill_slot_pair_listed(this: usize, person: Staff, club: Club) -> bool {
    let key = [read_u32(person.addr()), if club.is_null() { u32::MAX } else { read_u32(club.addr()) }];
    exe::crt_bsearch(key.as_ptr() as usize, read_usize(this), read_u32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0usize) != 0
}

/// What `fill_shortlist_slot` carries from one candidate to the next (the original's frame).
struct FillSlotState {
    this: usize,
    slot: usize,
    club: Club,
    excluded: Staff,
    wanted: usize,       // the local STAFF_CONTRACT the desired contract is written to
    stand_in: usize,     // the stand-in manager; its upper 24 bits ride in the kind argument (see below)
    k: i16,              // the knowledge divisor
    factor: f64,         // the reputation factor of the price margin
    max_fee: i32,        // `exe::shortlist_manager_max_fee_for_squad_status`, −1 none
    max_price: i32,      // `shortlist_slot::MAX_PRICE`, at least 1
    is_player: bool,     // the slot's job is 11 (player)
    valuation: i32,      // the slot's valuation, raised by the replacement's rating
    best_score: i32,     // the score to beat (starts at the valuation)
    best: Staff,         // the chosen candidate
    score: i32,          // the candidate's score (the original keeps it in the frame across candidates)
    scout_listed: u32,   // the human's scout has him on his list (kept across candidates, as the original)
    comp: bool,          // compensation deal (kept across candidates, as the original)
    comp_fee: i32,       // its fee
}

impl FillSlotState {
    unsafe fn flags(&self) -> u8 {
        read_u8(self.slot + shortlist_slot::FLAGS)
    }
}

/// Whether the club "knows" a candidate who passed the filters: the human's scout lists him; or (id + club id) % k = 0;
/// or, for someone of another nation (`club.nation()` ≠ `.nation()`), without international caps (`.int_apps()`) and
/// not at a club of the club's nation: not a youngster under 23 who is foreign for the club (`exe::database_is_staff_foreign`
/// with 1) with a world reputation (`.world_reputation()`) below 3250; of the same region (`.region()`), or the club's
/// reputation at least 2750 and (his club of no nation or of the club's region, or the club's reputation at least
/// 4250, or the club's `league_standard()` at least 12); and then his reputation as the club's manager sees it
/// (`exe::database_get_staff_reputation_for_viewer`) at least 2750 + random(2000) in status 5/6 slots (a player with
/// world reputation 1750 or more) or 3250 + random(2000) in the others (world reputation 2750 or more). Of the same
/// nation: a non-player, a loan slot, no sheet, home reputation (`.home_reputation()`) 3250 or more, his club's
/// reputation 3250 or more or 20 years or more: known; a youngster only when (id + club id) % (25 − the manager's
/// `.youngsters()`) = 0, or % 15 without a manager with a non-player sheet.
unsafe fn fill_slot_club_knows(s: &FillSlotState, p: Staff) -> bool {
    if s.scout_listed != 0 {
        return true;
    }
    let club = s.club;
    let sum = read_i32(p.addr()).wrapping_add(read_i32(club.addr()));
    if sum.wrapping_rem(s.k as i32) == 0 {
        // mkb3: the club knows one in k
        covered!("mkb_3 known by the divisor");
        return true;
    }
    let cn = club.nation();
    let same = cn == p.nation() || p.int_apps() != 0 || (!p.club_job().is_null() && p.club_job().nation() == cn);
    if !same {
        if (p.age() as i8) < 23 && exe::database_is_staff_foreign(p, cn, 1) != 0 {
            let sheet = p.player();
            if !sheet.is_null() && sheet.world_reputation() < 3250 {
                return false;
            }
        }
        if !cn.is_null() && !p.nation().is_null() && cn.region() != p.nation().region() {
            if club.reputation() < 2750 {
                return false;
            }
            let job = p.club_job();
            let near = !job.is_null() && (job.nation().is_null() || job.nation().region() == cn.region());
            if !near && club.reputation() < 4250 && (cn.league_standard() as i8) < 12 {
                return false;
            }
        }
        let st = s.flags() & 0x1e;
        let manager = club.manager();
        if st == 0xa || st == 0xc {
            if s.is_player && p.player().world_reputation() < 1750 {
                return false;
            }
            let seen = exe::database_get_staff_reputation_for_viewer(p, manager, (!s.is_player) as u32) as i32;
            return seen >= exe::utils_random(2000) + 2750;
        }
        if s.is_player && p.player().world_reputation() < 2750 {
            return false;
        }
        let seen = exe::database_get_staff_reputation_for_viewer(p, manager, (!s.is_player) as u32) as i32;
        covered!("mkb_3 foreigner seen");
        return seen >= exe::utils_random(2000) + 3250; // mkb3: the reputation a foreigner needs
    }
    if !s.is_player || s.flags() & 1 != 0 {
        return true;
    }
    let sheet = p.player();
    if sheet.is_null() || sheet.home_reputation() >= 3250 {
        return true;
    }
    let job = p.club_job();
    if !job.is_null() && job.reputation() >= 3250 {
        return true;
    }
    if p.age() as i8 >= 20 {
        return true;
    }
    let manager = club.manager();
    covered!("mkb_3 youngster");
    if !manager.is_null() && !manager.non_player().is_null() {
        sum.wrapping_rem(25 - manager.non_player().youngsters() as i32) == 0
    } else {
        sum.wrapping_rem(15) == 0
    }
}

/// One candidate of the position lists (the 12-byte entry: +0 the person's id, +8 his price), in the original's order
/// (0x83fa25..0x840a81); a `return` is the original's jump to the next candidate.
unsafe fn fill_slot_candidate(s: &mut FillSlotState, entry: usize) {
    let p = staff_at(read_i32(entry) as isize);
    let club = s.club;
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut contract_cache: usize = 0;
    if p == s.excluded || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(16) || p.job_for_club() == 5 || p.job_for_nation() == 5 {
        return;
    }
    if !p.non_player().is_null() && read_usize(G_MANAGER_MANAGER) != 0 && exe::manager_manager_is_vacancy_candidate(read_usize(G_MANAGER_MANAGER), p) != 0 {
        return;
    }
    let price = read_i32(entry + 8);
    if s.is_player {
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
            let hm = read_usize(G_HUMAN_MANAGER);
            if exe::human_manager_is_on_holiday(hm, exe::database_get_club_manager(club)) != 0 {
                let listed = exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), p, exe::database_get_club_manager(club));
                s.scout_listed = listed;
                if exe::human_manager_get_holiday_option(hm, 4, exe::database_get_club_manager(club)) == 2 && listed == 0 {
                    return;
                }
            }
        }
        if p.job_for_club() != 0xb || (!p.national_job().is_null() && p.job_for_nation() == 5) {
            return;
        }
        if p.player().is_null() || staff_state_byte_0b(p) & 0x30 == 0x10 {
            return;
        }
        if !p.nation().is_null() && read_u32(p.nation().addr()) == read_u32(KEY_NATION_SAUDI_ARABIA) {
            return;
        }
        if s.flags() & 0x1e == 0xa && p.age() as i8 > 23 {
            return;
        }
        if s.flags() & 1 == 0 {
            if (read_i32(s.slot + shortlist_slot::MAX_PRICE) as f64) < price as f64 * read_f64(rdata::F64_1_3) { // mkb3: price × 1.3 within the ceiling
                return;
            }
        } else if !(!p.club_job().is_null() && price <= 0) && exe::staff_is_loan_listed(p, 0) == 0 {
            return;
        }
    } else {
        if p.non_player().is_null() {
            return;
        }
        let c = Contract(exe::contract_manager_get_contract(cm, p));
        contract_cache = c.addr();
        let left = if !c.is_null() && c.contract_type() & 0x3f == 2 {
            (exe::staff_contract_days_to_end_date(c) as i32).wrapping_mul(c.wage()) / 7
        } else {
            0
        };
        if left >= read_i32(s.slot + shortlist_slot::MAX_PRICE) || left > (club.reputation() as i32 + 500) * 10 {
            return;
        }
    }
    if p.club_job() == club || club_of_staff_contract(p) == club.addr() || staff_state_byte_0b(p) & 0xf >= 10 {
        return;
    }
    if s.is_player && !p.club_job().is_null() {
        let mut d = [0u8; 8];
        let q = exe::date_sub_days(G_CURRENT_DATE, d.as_mut_ptr() as usize, 200);
        if fill_slot_date_before(q, p.addr() + staff::DATE_JOINED_CLUB) {
            let c = Contract(exe::contract_manager_get_contract(cm, p));
            contract_cache = c.addr();
            if !c.is_null() && c.squad_status() & 0xf0 != 0x70 && c.grievances() == 0 && exe::staff_is_transfer_listed_or_requested(p, 0) == 0 {
                return;
            }
        }
    }
    if staff_state_byte_0b(p) & 0xf != 0 && read_usize(s.this) != 0 && fill_slot_pair_listed(s.this, p, club) {
        return;
    }
    let work_permit: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x84b180usize);
    if work_permit(s.this, p.addr(), p.club_job().addr(), club.addr()) == 0 {
        return;
    }
    if exe::transfer_manager_player_needs_work_permit(tm, p, club, 1) != 0 {
        let st = s.flags() & 0x1e;
        if st != 2 && st != 4 && st != 6 {
            return;
        }
    }
    if s.flags() & 1 != 0 && (p.club_job().is_null() || exe::transfer_manager_can_loan_player(tm, club, p, 0) == 0) {
        return;
    }
    if s.flags() & 1 != 0 {
        let c = Contract(contract_of_staff_cached(p));
        if !c.is_null() && !transfer_manager_club_by_id(c.club()).is_null()
            && exe::human_manager_is_human_club(club_of_staff_contract(p), 0, 1, 0, 0) != 0
            && Contract(exe::contract_manager_get_contract(cm, p)).transfer_status() & 2 == 0
        {
            return;
        }
    }
    if exe::transfer_manager_get_future_transfer(tm, p, 0) != 0 {
        return;
    }
    if !fill_slot_club_knows(s, p) {
        return;
    }
    let mut value: i64;
    if s.is_player {
        let mut score = s.score;
        value = exe::shortlist_manager_value_player_for_club(s.this, club, p, s.slot, &mut score, s.best_score, 1) as i64;
        s.score = score;
    } else {
        s.score = exe::score_staff_for_job(read_usize(G_MANAGER_MANAGER), p, club, club.manager().addr() as u32, read_u8(s.slot) as u32);
        value = p.value() as i32 as i64;
    }
    if s.score < s.valuation || s.score < s.best_score.wrapping_sub(10000) {
        return;
    }
    if s.flags() & 1 == 0 {
        let seller = club_of_staff_contract(p);
        let ft = read_usize(G_FINANCE_TABLE);
        if s.is_player && seller != 0 && exe::financetable_get_club_finance(ft, seller) != 0
            && read_u8(exe::financetable_get_club_finance(ft, seller) + finance::CRISIS_STATE) == 1
        {
            covered!("mkb_3 seller in crisis");
            if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
                if value >= exe::contract_manager_calc_player_value(cm, p, seller, 0, -1, 0) as i64 {
                    value = exe::contract_manager_calc_player_value(cm, p, seller, 0, -1, 0) as i64;
                }
            } else if value >= p.value() as i32 as i64 {
                value = p.value() as i32 as i64;
            }
        }
        if contract_cache == 0 {
            contract_cache = exe::contract_manager_get_contract(cm, p);
        }
        s.comp = s.is_player && fill_slot_is_compensation_deal(s.this, s.slot, contract_cache, club, &mut s.comp_fee) != 0;
        let mut ask: i32 = if seller == 0 || s.comp {
            0
        } else if !s.is_player {
            p.value() as i32
        } else if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
            exe::contract_manager_get_human_asking_price(cm, seller, club, p)
        } else {
            exe::contract_manager_calc_asking_price(cm, seller, club, p, 0, 1, 1)
        };
        if s.comp_fee != 0 && s.comp && s.comp_fee < ask {
            ask = s.comp_fee;
        }
        if s.is_player {
            let c = Contract(contract_of_staff_cached(p));
            if !c.is_null() && !transfer_manager_club_by_id(c.club()).is_null() && !s.comp {
                let mut extra: i32 = 0;
                let st = s.flags() & 0x1e;
                if ask as i64 > value && (st == 0xa || (st == 8 && (p.age() as i8) < 28)) {
                    let f = (club.reputation() as i32 as f64 * read_f64(0x96f8a0) + read_f64(rdata::F64_0_5)) * value as f64; // f64 1/13000
                    let cap = read_f64(0x96a650); // f64 2,500,000
                    let m = if cap < f { cap } else { f };
                    let diff = (ask as i64).wrapping_sub(value) as f64;
                    extra = ftol(if diff < m { diff } else { m });
                }
                let owner = club_of_staff_contract(p);
                if exe::shortlist_manager_predict_selling_club_response(owner, p, (value as i32).wrapping_add(extra) as i64, club, ask as i64, 0) == 2 {
                    covered!("mkb_3 seller would refuse");
                    return;
                }
            }
        }
        if s.max_fee != -1 && ask > s.max_fee {
            return;
        }
        if s.is_player {
            let margin = (ask as i64).wrapping_sub(value) as f64 / (s.max_price as f64 * s.factor);
            let mut y = read_f64(rdata::F64_1) - margin;
            if y > read_f64(rdata::F64_1) {
                y = y * read_f64(rdata::F64_0_3333333333) + read_f64(rdata::F64_0_67);
            }
            if read_f64(rdata::F64_0_01) > y {
                y = read_f64(rdata::F64_0_01);
            } else if !(read_f64(rdata::F64_3) >= y) {
                y = read_f64(rdata::F64_3);
            }
            s.score = ftol(s.score as f64 * y);
            let inj = exe::injury_get_player_injury(read_usize(G_INJURY), p);
            if inj != 0 && read_i16(inj + injury_entry::DAYS_LEFT) > 15 {
                let d = read_i16(inj + injury_entry::DAYS_LEFT) as i32 - 15;
                s.score = s.score.wrapping_sub(d.wrapping_mul(d));
            }
        }
    } else {
        // the original pushes the loan flag with the upper bits of a scratch register; the prediction reads its byte
        if exe::shortlist_manager_predict_selling_club_response(p.club_job(), p, value, club, -1, 1) == 2 {
            covered!("mkb_3 lender would refuse");
            return;
        }
        let inj = exe::injury_get_player_injury(read_usize(G_INJURY), p);
        if inj != 0 && read_i16(inj + injury_entry::DAYS_LEFT) > 10 {
            let d = read_i16(inj + injury_entry::DAYS_LEFT) as i32;
            s.score = s.score.wrapping_add((10 - d).wrapping_mul(d - 10).wrapping_mul(200));
        }
    }
    if s.is_player && exe::transfer_manager_can_club_bid_for_player(tm, p, club, core::ptr::null_mut(), 0) == 0 {
        return;
    }
    let r = exe::utils_random(17) + 92;
    s.score = ftol(r as f64 * read_f64(rdata::F64_0_01) * s.score as f64);
    if s.score > s.best_score {
        let f = s.flags();
        let interest = if s.is_player {
            exe::shortlist_manager_staff_interest_in_joining_club(s.this, p, club, 1, (f & 1) as u32, ((f >> 1) & 0xf) as u32, 0)
        } else {
            exe::staff_interest_in_job_at_club(s.this, p, club, 1, read_u8(s.slot) as u32, 0)
        };
        if s.flags() & 1 == 0 && s.is_player {
            let r20 = exe::utils_random(20);
            if !(read_f64(rdata::F64_3_15) - r20 as f64 * read_f64(rdata::F64_0_1) > interest) {
                let cr = p.player().current_reputation() as i32;
                if exe::utils_random(79 - cr / 150) == 0 {
                    let tell = cr > exe::utils_random(2000) + 5000
                        || (!p.club_job().is_null() && exe::human_manager_is_human_club(p.club_job(), 0, 1, 0, 0) != 0);
                    if tell {
                        if s.comp {
                            value = 0;
                        } else {
                            let v = value as f64;
                            let r = exe::utils_random(ftol(v * read_f64(rdata::F64_1_25)));
                            value = ftol(r as f64 + v * read_f64(rdata::F64_0_75)) as i64;
                        }
                        covered!("mkb_3 interest news");
                        fill_slot_add_transfer_interest(p, club, value as i32, interest);
                    }
                }
            }
        }
        if interest >= read_f64(rdata::F64_3_15) { // mkb3: the willingness a target needs
            let f = s.flags();
            if f & 1 == 0 && s.is_player {
                let rep = club.reputation() as i32;
                let sheet = p.player();
                let age = p.age() as i8;
                let kind: u8 = if value > rep.wrapping_mul(1250) as i64 && f & 0x1e != 2 {
                    1
                } else if value > rep.wrapping_mul(750) as i64 && f & 0x1e < 4 {
                    2
                } else if f & 0x1e == 0xa && !sheet.is_null() && age > 20 && age < 24 && rep < 4250
                    && sheet.current_reputation() as i32 - 750 > rep
                {
                    3
                } else {
                    (f >> 1) & 0xf
                };
                // the original writes the kind into the low byte of the stand-in manager's local and pushes the dword
                let kind_arg = (s.stand_in as u32 & 0xffff_ff00) | kind as u32;
                let wage = exe::staff_contract_default_staff_contract(s.wanted, p, club, 0, kind_arg, 0xb, 0) as f32;
                let st = ((s.flags() >> 1) & 0xf) as u32;
                let w0 = exe::shortlist_manager_staff_interest_in_joining_club(s.this, p, club, 0, 0, st, 0);
                let d = if w0 < read_f64(rdata::F64_0_1) {
                    read_f64(rdata::F64_0_1)
                } else {
                    exe::shortlist_manager_staff_interest_in_joining_club(s.this, p, club, 0, 0, st, 0)
                };
                let mut q = read_f64(rdata::F64_3_5) / d;
                if q < read_f64(rdata::F64_0_9) {
                    q = read_f32(rdata::F32_0_8999999762) as f64;
                }
                let wanted_wage = q * wage as f64;
                let cap = exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), p, kind_arg);
                if (cap as f64 * read_f64(rdata::F64_1_2)) < wanted_wage { // mkb3: the wage within 1.2 × the cap
                    covered!("mkb_3 wage too high");
                    return;
                }
            }
            covered!("mkb_3 new best");
            s.best = p;
            s.best_score = s.score;
        }
    }
    if exe::utils_random(ftol(s.score as f64 * read_f64(rdata::F64_1_5))) <= s.best_score || !s.is_player {
        return;
    }
    if p.age() as i8 as i32 >= 28 - exe::utils_random(6) {
        return;
    }
    let c = Contract(contract_of_staff_cached(p));
    if c.is_null() || transfer_manager_club_by_id(c.club()).is_null() || p.club_job().is_null() {
        return;
    }
    let seller = Club(club_of_staff_contract(p));
    if club.reputation() as i32 > exe::utils_random(1000) + seller.reputation() as i32 + 2000
        && exe::news_club_has_human_manager(read_usize(G_NEWS), club_of_staff_contract(p)) != 0
    {
        covered!("mkb_3 scout watching story");
        let story: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x587220usize); // news_create_scout_watching_story
        story(p.addr(), club.addr());
    }
}

/// The AI chooses the target of one shortlist slot of a computer club: it walks the shortlist manager's position lists
/// of the slot's position, keeps the candidates the club can sign and knows, scores them and takes the best one whose
/// willingness and wages fit; a slot with nobody may become a loan slot and start again.
///
/// From `allocate_transfer_budget` (0x83b6e0) for each open slot it gives budget to (`excluded` 0), from
/// `update_staff_status` (0x843e10) when a slot's target is lost and the slot is filled again (`excluded` the person
/// just lost), and from itself. The
/// local STAFF_CONTRACT (`exe::new_staff_contract`) receives the desired contracts. Leaves: the club with no nation;
/// a human club (`exe::human_manager_is_human_club` 0, 1); a human club (1, 1) whose manager is on holiday
/// (`exe::human_manager_is_on_holiday`) with holiday option 4 equal to 1 (the stand-in manager,
/// `exe::human_manager_get_stand_in_manager`, is kept for the replacement's rating); a slot with an offer
/// (`shortlist_slot::OFFER_REF` ≠ −1); a national team (`exe::database_is_national_team`).
///
/// The season flag: a player slot of a key nation (`exe::key_nation_index_of_nation`, rows 0 to 34 of
/// `G_LEAGUE_WORLD_CUP_START_DATE`/`END_DATE`, 0x48 bytes each) keeps it only between the season's start − 28 days and
/// its end − 28 days (`exe::date_sub_days`); another nation keeps it; a non-player slot loses it.
///
/// The knowledge divisor k (i16): 100; reputation (`.reputation()`) below 3500: (6025 − reputation) × 4; Belgium,
/// Holland, Sweden, Denmark, Norway: 50. Each of the 7 `club::SCOUTS` places: empty, +10 with the club's division
/// loaded in `G_COMP_OBJECT_TABLE`, +5 otherwise; a scout with a non-player sheet: + 30 − 2 × a − b, where a and b are
/// `exe::non_player_attribute_rating` of his sheet with the constants 6 and 5. **Defect of the original (probable):**
/// the other callers of that routine pass the scout's judging attributes; here the constants 6 and 5 go, so the result
/// depends only on the scout's current ability (`non_player::CURRENT_ABILITY`): his `JUDGEMENT`/`JUDGING_POTENTIAL`
/// never count. Kept as the original does it. Then k = k / 10, at least 2.
///
/// The ceiling: `exe::shortlist_manager_max_fee_for_squad_status` (the slot's position, status = bits 1-4 of
/// `shortlist_slot::FLAGS`). The reputation factor: reputation × 0.0008, or in Italy, England, Germany, France and Spain
/// with finances (`G_FINANCE_TABLE`) by the wealth band (`exe::finance_get_wealth_band`): band 1, × 1/1175 (× 1/1125
/// above 6250); band 2, × 1/1125 (× 1/1050 above 4750); at most 1.0 becomes 1.0.
///
/// The replacement (`shortlist_slot::REPLACEMENT_STAFF_ID` > −1; a non-player slot leaves here): his rating
/// (`exe::match_man_score_player_for_selection` with the slot's position, the flags 0x1902 (status 0, 1, above 4),
/// 0x18c2 (status 2, 3; 0x8118c2 at a human club), 0x19c2 (status 4), −1 and the stand-in manager); with no contract at
/// the club (`exe::contract_manager_get_contract_for_club`), or grievances with 0x100, 1, or under 365 days left
/// (`exe::staff_contract_days_to_end_date`), it is at most the valuation; the valuation becomes the larger of itself and
/// rating × 1.05. The score to beat starts at the valuation.
///
/// The position code (`exe::shortlist_position_mask_to_index`, 0xf for a non-player) gives the lists: 0x11 → 1-4,
/// 0x12 → 8-13, 0x13 → 5-7, 0x15 → 11-13, 0x14 → 14, others → itself. A loan player slot walks the lists sorted by key
/// (`POSITION_LISTS_BY_KEY`, index + 16) with the counts of the others; an empty list (first id −1) ends the search.
/// The excluded person, when the club has him in the shortlist pairs, gets `exe::shortlist_manager_update_staff_status`
/// with 3.
///
/// A candidate (`fill_slot_candidate`) is skipped when: the excluded; id ≥ the number of people − 16; `.job_for_club()`
/// or `.job_for_nation()` 5; a non-player who is a vacancy candidate (`exe::manager_manager_is_vacancy_candidate`).
/// Player slot: at a human club on holiday, the human's scout list (`exe::scout_manager_is_player_shortlisted`) is
/// kept and with holiday option 4 equal to 2 a player off the list is skipped; not a player (job 11); national team
/// job 5; no sheet; retirement announced (state bits 0x30 = 0x10); Saudi; over 23 in a status-5 slot; purchase: the
/// ceiling (`shortlist_slot::MAX_PRICE`) below his price × 1.3; loan: a price above 0 (or no club) and not loan listed
/// (`exe::staff_is_loan_listed`). Non-player slot: no non-player sheet; the rest of a type-2 contract (days left ×
/// wage / 7) at least the ceiling or above (reputation + 500) × 10. Then: already at the club (`.club_job()` or the
/// contract's club, `club_of_staff_contract`); on 10 shortlists or more (low nibble of the state byte); a player who
/// arrived less than 200 days ago with a contract not in status 7, without grievances and not listed
/// (`exe::staff_is_transfer_listed_or_requested`); already in this club's shortlist pairs; the work permit rules
/// (0x84b180) or a permit needed (`exe::transfer_manager_player_needs_work_permit`) outside status 1-3; loan: no club
/// or no loan possible (`exe::transfer_manager_can_loan_player`), a human owner and not loan listed
/// (`contract::TRANSFER_STATUS` bit 2); a future transfer (`exe::transfer_manager_get_future_transfer`); not known
/// (`fill_slot_club_knows`).
///
/// The score: a player's value and score for the club (`exe::shortlist_manager_value_player_for_club`, with the score
/// to beat); a non-player's `exe::score_staff_for_job` (manager, the slot's job) and `.value()`. Below the valuation or
/// below the score to beat − 10000: skipped. Purchase: a player at a seller in crisis (`finance::CRISIS_STATE` 1) is
/// valued at most at the seller's value (human seller: `exe::contract_manager_calc_player_value`, called again when it
/// is the lower) or his `.value()`; a compensation deal (0x84b6f0) costs its fee; otherwise the asking price (non-player:
/// `.value()`; human seller: `exe::contract_manager_get_human_asking_price`; AI: `exe::contract_manager_calc_asking_price`).
/// A player without compensation: in status 4 under 28 or status 5, an asking price above the value adds to the offer
/// min(asking − value, (reputation / 13000 + 0.5) × value, 2,500,000); the predicted answer of the contract's club
/// (`exe::shortlist_manager_predict_selling_club_response`) 2 skips. The asking price above the slot's ceiling skips.
/// A player's score × y, y = 1 − (asking − value) / (max price × factor), above 1 → y/3 + 0.67, between 0.01 and 3;
/// injured (`exe::injury_get_player_injury`) more than 15 days: − (days − 15)². Loan: the lender's prediction (offer =
/// value, asked −1, loan) 2 skips; injured more than 10 days: − (days − 10)² × 200. A player the club cannot bid for
/// (`exe::transfer_manager_can_club_bid_for_player`) is skipped. Then score × (92 + random(17)) / 100.
///
/// Above the score to beat: the interest (`exe::shortlist_manager_staff_interest_in_joining_club` with 1, the loan bit,
/// the status; a non-player `exe::staff_interest_in_job_at_club`). A purchase player whose interest is at least
/// 3.15 − random(20) × 0.1, with random(79 − reputation / 150) = 0 and (reputation above 5000 + random(2000) or a human
/// club): interest news (`Media::add_transfer_interest`, 0x754740) with 0 for a compensation deal or
/// random(value × 1.25) + value × 0.75, which also becomes the value. Interest at least 3.15: for a purchase player
/// the contract kind (1 if the value passes reputation × 1250 outside status 1; 2 if it passes reputation × 750 in
/// status 0 or 1; 3 in status 5 for 21-23 years at a club below 4250 with his reputation 750 above the club's; else
/// the status), the desired wage (`exe::staff_contract_default_staff_contract`, as f32) × max(3.5 / interest
/// (interest without the loan and status flags, at least 0.1), 0.9) above 1.2 × the wage cap
/// (`exe::finance_get_max_wage`) skips; otherwise he is the best so far. The kind argument carries the stand-in
/// manager's upper 24 bits, as the original's frame does. Every candidate scored this far (not skipped): with
/// random(score × 1.5) above the score to beat, a player younger than 28 − random(6) under contract at a club whose
/// reputation + 2000 + random(1000) is below the club's, with a human manager (`exe::news_club_has_human_manager`): the
/// scout watching story (0x587220).
///
/// The best becomes the target (`shortlist_slot::TARGET_STAFF_ID`) and enters the shortlist (0x8423f0); a purchase
/// player slot then, with random(37 − reputation / 300) = 0 and (reputation above 5000 + random(2000) or a human club),
/// gets the interest news with the asking price (or the compensation fee). Nobody: with the season flag, a purchase
/// slot of status 2 or 3 with no target becomes a loan slot (bit 0) and starts again (recursion through 0x83f3a0) when
/// (reputation² × 4e-9 + 0.6) × valuation < `shortlist_slot::PRICE`, or the ceiling is under 50000, or in Italy with
/// random(2) ≠ 0 and reputation 3001 to 5999; and the nation's league is loaded (`.league_selected()`) or
/// random(reputation) > random(20000). Otherwise the target is −1. The SEH frame only matters for exceptions.
/// Original: `Shortlist_Manager::fill_shortlist_slot` (0x83f3a0, shortlist_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn fill_shortlist_slot(this: usize, slot: usize, club: usize, excluded: usize) {
    let mut wanted = [0u8; 0x58];
    exe::new_staff_contract(wanted.as_mut_ptr() as usize);
    fill_shortlist_slot_body(this, slot, Club(club), Staff(excluded), wanted.as_mut_ptr() as usize);
}

/// The body of `fill_shortlist_slot` after the local STAFF_CONTRACT is built.
unsafe fn fill_shortlist_slot_body(this: usize, slot: usize, club: Club, excluded: Staff, wanted: usize) {
    let nation = club.nation();
    if nation.is_null() {
        return;
    }
    let row = exe::key_nation_index_of_nation(nation);
    let job = read_u8(slot);
    let mut in_season = true;
    if job != 0xb {
        in_season = false;
    } else if (0..0x23).contains(&row) {
        let off = row as usize * 0x48;
        let mut d = [0u8; 8];
        let start = exe::date_sub_days(G_LEAGUE_WORLD_CUP_START_DATE + off, d.as_mut_ptr() as usize, 0x1c);
        let mut inside = false;
        if fill_slot_date_before(start, G_CURRENT_DATE) {
            let mut e = [0u8; 8];
            let end = exe::date_sub_days(G_LEAGUE_WORLD_CUP_END_DATE + off, e.as_mut_ptr() as usize, 0x1c);
            inside = fill_slot_date_before(G_CURRENT_DATE, end);
        }
        if !inside {
            in_season = false;
        }
    }
    let rep = club.reputation();
    let mut k: i16 = 100;
    if rep < 3500 {
        k = 6025i16.wrapping_sub(rep).wrapping_mul(4);
    } else {
        let id = read_u32(nation.addr());
        if [KEY_NATION_BELGIUM, KEY_NATION_HOLLAND, KEY_NATION_SWEDEN, KEY_NATION_DENMARK, KEY_NATION_NORWAY].iter().any(|&key| id == read_u32(key)) {
            k = 50;
        }
    }
    for i in 0..7 {
        let scout = Staff(club.scouts(i));
        if scout.is_null() {
            let division = club.division();
            let loaded = !division.is_null() && read_u32(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(division.addr()).wrapping_mul(4))) != 0;
            k = k.wrapping_add(if loaded { 10 } else { 5 });
        } else if !scout.non_player().is_null() {
            // defect kept: the constants 6 and 5, not the scout's judging attributes
            let a = exe::non_player_attribute_rating(scout.non_player(), 6) as i16;
            let b = exe::non_player_attribute_rating(scout.non_player(), 5) as i16;
            k = k.wrapping_add(30i16.wrapping_sub(a.wrapping_mul(2)).wrapping_sub(b));
        }
    }
    k /= 10; // mkb3: the knowledge divisor
    if k < 2 {
        k = 2;
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let mut stand_in: usize = 0;
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        stand_in = exe::human_manager_get_stand_in_manager(club);
        let hm = read_usize(G_HUMAN_MANAGER);
        if exe::human_manager_is_on_holiday(hm, exe::database_get_club_manager(club)) != 0
            && exe::human_manager_get_holiday_option(hm, 4, exe::database_get_club_manager(club)) == 1
        {
            return;
        }
    }
    if read_i32(slot + shortlist_slot::OFFER_REF) != -1 || exe::database_is_national_team(club) != 0 {
        return;
    }
    let mut valuation = read_i32(slot + shortlist_slot::VALUATION);
    let status = (read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf;
    let max_fee = exe::shortlist_manager_max_fee_for_squad_status(this, club, read_u16(slot + shortlist_slot::POSITION) as u32, status as u32);
    let rep_f = rep as i32 as f64;
    let mut factor = rep_f * read_f64(rdata::F64_0_0008);
    let ft = read_usize(G_FINANCE_TABLE);
    let id = read_u32(nation.addr());
    if ft != 0 && [KEY_NATION_ITALY, KEY_NATION_ENGLAND, KEY_NATION_GERMANY, KEY_NATION_FRANCE, KEY_NATION_SPAIN].iter().any(|&key| id == read_u32(key)) {
        match exe::finance_get_wealth_band(exe::financetable_get_club_finance(ft, club)) as i8 {
            1 => factor = rep_f * read_f64(if rep > 6250 { 0x96f8b0 } else { 0x96f8a8 }), // f64 1/1125, 1/1175
            2 => factor = rep_f * read_f64(if rep > 4750 { 0x96f8b8 } else { 0x96f8b0 }), // f64 1/1050, 1/1125
            _ => {}
        }
    }
    if factor <= read_f64(rdata::F64_1) {
        factor = read_f64(rdata::F64_1);
    }
    let rid = read_i32(slot + shortlist_slot::REPLACEMENT_STAFF_ID);
    if rid > -1 {
        let replaced = staff_at(rid as isize);
        if job != 0xb {
            return;
        }
        let selection = if status < 2 {
            0x1902
        } else if status <= 3 {
            if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 { 0x8118c2 } else { 0x18c2 }
        } else if status == 4 {
            0x19c2
        } else {
            0x1902
        };
        let mut rating = exe::match_man_score_player_for_selection(replaced, read_u16(slot + shortlist_slot::POSITION) as u32, selection, club, -1, stand_in);
        let c = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), replaced, club));
        let leaving = c.is_null() || {
            let g = c.grievances();
            g != 0 && (g & 0x100 != 0 || g & 1 != 0 || exe::staff_contract_days_to_end_date(c) < 365)
        };
        if leaving && rating >= valuation {
            rating = valuation;
        }
        let r = rating as f64 * read_f64(rdata::F64_1_05);
        if !(valuation as f64 > r) {
            valuation = ftol(r);
        }
    }
    let is_player = job == 0xb;
    let code = if is_player { exe::shortlist_position_mask_to_index(this, read_u16(slot + shortlist_slot::POSITION) as u32) } else { 0xf };
    let max_price = read_i32(slot + shortlist_slot::MAX_PRICE).max(1);
    let loan_lists = read_u8(slot + shortlist_slot::FLAGS) & 1 != 0 && is_player;
    if !excluded.is_null() && read_usize(this) != 0 && fill_slot_pair_listed(this, excluded, club) {
        exe::shortlist_manager_update_staff_status(this, excluded, club, 3, (!is_player) as u32);
    }
    let (first, last): (i8, i8) = match code {
        0x11 => (1, 4),
        0x12 => (8, 0xd),
        0x13 => (5, 7),
        0x15 => (0xb, 0xd),
        0x14 => (0xe, 0xe),
        c => (c as i8, c as i8),
    };
    let mut s = FillSlotState {
        this,
        slot,
        club,
        excluded,
        wanted,
        stand_in,
        k,
        factor,
        max_fee,
        max_price,
        is_player,
        valuation,
        best_score: valuation,
        best: Staff(0),
        score: 0,
        scout_listed: 0,
        comp: false,
        comp_fee: 0,
    };
    let mut pos = first;
    while pos <= last {
        let index = (pos as isize as usize).wrapping_add(if loan_lists { 16 } else { 0 });
        let list = read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + index * 4);
        if read_i32(list) == -1 {
            break;
        }
        let count = read_i32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + (pos as isize as usize) * 4);
        let mut i = 0;
        while i < count {
            fill_slot_candidate(&mut s, list + (i as usize) * 12);
            i += 1;
        }
        pos += 1;
    }
    let cm = read_usize(G_CONTRACT_MANAGER);
    if !s.best.is_null() {
        let best = s.best;
        write_u32(slot + shortlist_slot::TARGET_STAFF_ID, read_u32(best.addr()));
        let add_entry: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8423f0usize); // add_shortlist_entry
        add_entry(this, best.addr(), club.addr());
        covered!("mkb_3 target chosen");
        if read_u8(slot + shortlist_slot::FLAGS) & 1 != 0 || !is_player {
            return;
        }
        let cr = best.player().current_reputation() as i32;
        if exe::utils_random(37 - cr / 300) != 0 {
            return;
        }
        if !(best.player().current_reputation() as i32 > exe::utils_random(2000) + 5000)
            && (best.club_job().is_null() || exe::human_manager_is_human_club(best.club_job(), 0, 1, 0, 0) == 0)
        {
            return;
        }
        let c = Contract(contract_of_staff_cached(best));
        let seller = if !c.is_null() && c.contract_type() & 0x3f != 5 { transfer_manager_club_by_id(c.club()).addr() } else { 0 };
        let comp = fill_slot_is_compensation_deal(this, slot, exe::contract_manager_get_contract(cm, best), club, &mut s.comp_fee) != 0;
        let mut ask = if seller == 0 || comp {
            0
        } else if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
            exe::contract_manager_get_human_asking_price(cm, seller, club, best)
        } else {
            exe::contract_manager_calc_asking_price(cm, seller, club, best, 0, 1, 1)
        };
        if s.comp_fee != 0 && comp && s.comp_fee < ask {
            ask = s.comp_fee;
        }
        let f = read_u8(slot + shortlist_slot::FLAGS);
        let interest = exe::shortlist_manager_staff_interest_in_joining_club(this, best, club, 1, (f & 1) as u32, ((f >> 1) & 0xf) as u32, 0);
        fill_slot_add_transfer_interest(best, club, ask, interest);
        return;
    }
    let f = read_u8(slot + shortlist_slot::FLAGS);
    if in_season && f & 1 == 0 && read_i32(slot + shortlist_slot::TARGET_STAFF_ID) == -1 && (f & 0x1e == 4 || f & 0x1e == 6) {
        let r = club.reputation() as i32 as f64;
        let a = (r * read_f64(0x96f898) * r + read_f64(rdata::F64_0_6)) * read_i32(slot + shortlist_slot::VALUATION) as f64; // f64 4e-9
        let convert = if a < read_i32(slot + shortlist_slot::PRICE) as f64 || read_i32(slot + shortlist_slot::MAX_PRICE) < 50000 {
            true
        } else if read_u32(club.nation().addr()) != read_u32(KEY_NATION_ITALY) || exe::utils_random(2) == 0 {
            false
        } else {
            club.reputation() > 3000 && club.reputation() < 6000
        };
        if convert && (club.nation().league_selected() != 0 || exe::utils_random(club.reputation() as i32) > exe::utils_random(20000)) {
            covered!("mkb_3 becomes a loan slot");
            write_u8(slot + shortlist_slot::FLAGS, read_u8(slot + shortlist_slot::FLAGS) | 1);
            exe::shortlist_manager_fill_shortlist_slot(this, slot, club, excluded); // itself through its address, as the original
            return;
        }
    }
    write_i32(slot + shortlist_slot::TARGET_STAFF_ID, -1);
}

/// Comparer for `fill_shortlist_slot`: the original only (it draws random numbers, adds to the shortlist and to the
/// media's interest list, may write news and recurses; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_fill_shortlist_slot(this: usize, slot: usize, club: usize, excluded: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_FILL_SHORTLIST_SLOT)(this, slot, club, excluded) // simplification: original only
}

// ---------- 288a leva: the base loan period, the open window and the window test (transfer_rules.cpp) ----------

/// True if date `(day, year)` is before the date at `b` (year compared unsigned first, then the day signed), as the
/// window and loan routines compare their 8-byte dates.
unsafe fn window_date_before(day: i16, year: u16, b: usize) -> bool {
    let by = read_i16(b + cm_date::YEAR) as u16;
    year < by || (year == by && day < read_i16(b))
}

/// Returns the default loan period of a nation's rules: from next month to the season end month of the club's nation,
/// this year if that month is still ahead of the start, otherwise next year.
///
/// `TRANSFER_RULES::get_loan_period(club, a2, start_month, start_year, end_month, end_year, player)`, the vtable slot
/// `transfer_rules_vtable::GET_LOAN_PERIOD` of the nations without their own (and called by
/// `england_rules_get_loan_period` after 19 June for players under 24), from `transfer_manager_get_loan_period`. The
/// start is today's month (`exe::date_get_month` of `G_CURRENT_DATE`) + 1 of this year (`G_CURRENT_DATE_YEAR`); past
/// December, January of the next year. The end month is the season end month of the club's nation
/// (`exe::contract_manager_season_end_month`: push club; mov ecx, `[G_CONTRACT_MANAGER]` -> thiscall, ret 4). Start
/// month below it: the end year is the start year; otherwise today's year + 1 (so a loan starting in the season end
/// month itself runs for a whole year). `a2` and `player` are not read.
/// Original: `TRANSFER_RULES::get_loan_period` (0x8e8bd0, transfer_rules.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn transfer_rules_get_loan_period(_this: usize, club: usize, _a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, _player: usize) {
    *start_month = (exe::date_get_month(G_CURRENT_DATE) as i8).wrapping_add(1);
    *start_year = read_i16(G_CURRENT_DATE_YEAR);
    if *start_month > 0xb {
        *start_month = 0;
        *start_year = (*start_year).wrapping_add(1);
    }
    let end = exe::contract_manager_season_end_month(read_usize(G_CONTRACT_MANAGER), club) as i8;
    *end_month = end; // mkc1: the season end month
    covered!("mkc_1 base loan period");
    if *start_month < end {
        *end_year = *start_year;
    } else {
        *end_year = read_i16(G_CURRENT_DATE_YEAR).wrapping_add(1);
    }
}

unsafe extern "thiscall" fn compare_transfer_rules_get_loan_period(this: usize, club: usize, a2: u32, start_month: *mut i8, start_year: *mut i16, end_month: *mut i8, end_year: *mut i16, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut i16, *mut i8, *mut i16, usize)>(IDX_TRANSFER_RULES_GET_LOAN_PERIOD);
    let regions = [(start_month as usize, 1usize), (start_year as usize, 2), (end_month as usize, 1), (end_year as usize, 2)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, club, a2, start_month, start_year, end_month, end_year, player), || transfer_rules_get_loan_period(this, club, a2, start_month, start_year, end_month, end_year, player));
    SCOREBOARD.record(equal, || format!("transfer_rules_get_loan_period(rules {:#x}, club {:#x}) original={:x?} rust={:x?}", this, club, mo, mr));
}

/// Tells whether the date falls in a transfer window of the rules and gives that window: the first window (12 bytes
/// each, the table at +4, the count byte at +8) that contains the date (`exe::transfer_window_contains_date`); found:
/// the window in `out` (if given) and 1. None found: 0 when the rules have windows; rules without any window (Denmark,
/// for example) are always open: `out` = 0 and 1. The count is read again after each window.
///
/// `TRANSFER_RULES::find_open_window(date, out)`, from the nations' `can_loan_player`/`can_sign_player`/
/// `can_make_offer`/`can_end_loan` (England, Australia, Norway, Turkey, Finland, France, Ireland, Spain, Japan,
/// Portugal, Germany, Belgium, Argentina) and from `find_next_window_open_date` (each day of the next year).
/// Original: `TRANSFER_RULES::find_open_window` (0x8e8e50, transfer_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_rules_find_open_window(this: usize, date: usize, out: u32) -> u32 {
    let mut k: i8 = 0;
    let mut count = read_u8(this + 8) as i8;
    while k < count {
        let window = read_usize(this + 4) + k as i32 as usize * 12;
        if exe::transfer_window_contains_date(this, date, window) != 0 {
            if out != 0 {
                write_usize(out as usize, window);
            }
            return 1;
        }
        count = read_u8(this + 8) as i8;
        k += 1;
    }
    if count != 0 {
        covered!("mkc_1 no open window");
        return 0; // mkc1: windows, none open
    }
    if out != 0 {
        write_usize(out as usize, 0);
    }
    1
}

unsafe extern "thiscall" fn compare_transfer_rules_find_open_window(this: usize, date: usize, out: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_TRANSFER_RULES_FIND_OPEN_WINDOW);
    let regions: Vec<(usize, usize)> = if out != 0 { vec![(out as usize, 4)] } else { Vec::new() };
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, date, out), || transfer_rules_find_open_window(this, date, out));
    SCOREBOARD.record(equal && ro == rr, || format!("transfer_rules_find_open_window(rules {:#x}, date {:#x}) original={} {:x?} rust={} {:x?}", this, date, ro, mo, rr, mr));
    ro
}

/// Tells whether a date falls in a transfer window: the window's two 6-byte dates (+0 opens, +6 closes: weekday +2, day
/// +3, month `transfer_window::MONTH`) taken in the date's year (`exe::transfer_window_get_date`: push year; push out;
/// mov ecx, window date -> thiscall, ret 8). A window that wraps the new year (opening month above the closing month,
/// as England's June to March) is open from the opening to the end of the year and from the start of the year to the
/// closing: the opening before the date + 1 day (`exe::date_add_days`) or the closing after the date − 1 day
/// (`exe::date_sub_days`), both ends included. Otherwise the opening before the date + 1 and the closing after the date
/// − 1. The rules (`this`) are not read.
/// Original: `transfer_window_contains_date` (0x8e8c40, transfer_rules.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_window_contains_date(_this: usize, date: usize, window: usize) -> u32 {
    let year = read_i16(date + cm_date::YEAR) as u16 as u32;
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    let open = exe::transfer_window_get_date(window, t1.as_mut_ptr() as usize, year);
    let (od, oy) = (read_i16(open), read_i16(open + cm_date::YEAR) as u16);
    let after = exe::date_add_days(date, t2.as_mut_ptr() as usize, 1);
    let opened = window_date_before(od, oy, after);
    if read_u8(window + transfer_window::MONTH) as i8 > read_u8(window + 6 + transfer_window::MONTH) as i8 {
        if opened {
            return 1;
        }
        covered!("mkc_1 wrapped window end");
        // mkc1: the wrapped window, from the start of the year to the closing
    } else if !opened {
        return 0;
    }
    let close = exe::transfer_window_get_date(window + 6, t1.as_mut_ptr() as usize, year);
    let (cd, cy) = (read_i16(close), read_i16(close + cm_date::YEAR) as u16);
    let before = exe::date_sub_days(date, t2.as_mut_ptr() as usize, 1);
    let (bd, by) = (read_i16(before), read_i16(before + cm_date::YEAR) as u16);
    (cy > by || (cy == by && cd > bd)) as u32
}

compare_value!(compare_transfer_window_contains_date, transfer_window_contains_date, IDX_TRANSFER_WINDOW_CONTAINS_DATE, (date: usize, window: usize) -> u32);

// ---------- batch 288: the value of a contract for the person, against an offer (staff_contracts.cpp) ----------

/// The `.rdata` floats of `staff_contract_calc_contract_value` that `rdata_consts.rs` does not name yet (value in
/// cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod contract_value_rdata {
    pub(crate) const F32_0_2222222239: usize = 0x96ff84; // f32 = 0.2222222238779068
    pub(crate) const F32_1_200000048: usize = 0x96d7bc; // f32 = 1.2000000476837158
    pub(crate) const F32_1_700000048: usize = 0x96ff78; // f32 = 1.7000000476837158
    pub(crate) const F32_1_799999952: usize = 0x96d7b4; // f32 = 1.7999999523162842
    pub(crate) const F32_1_950000048: usize = 0x96ff80; // f32 = 1.9500000476837158
    pub(crate) const F32_1_00999999: usize = 0x96ff7c; // f32 = 1.0099999904632568
    pub(crate) const F32_1_450000048: usize = 0x96ff74; // f32 = 1.4500000476837158
}

/// The multiplier of one release clause of the contract (index `k` from `contract::NON_PROMOTION_RC`: 0 non-promotion,
/// 1 minimum fee, 2 non-playing, 3 relegation, 4 manager job), from the release fees of the contract (`cf`) and of the
/// offer (`of`) in the bands below 1 million, 5 million (7.5 million in case 0) and above, the offer's own clauses and
/// the person's age. Case 0 reads the offer's minimum fee clause, not its non-promotion one (as in the original).
pub(crate) unsafe fn contract_value_clause_factor(k: usize, offer: Contract, cf: i32, age: i8) -> f64 {
    use contract_value_rdata::*;
    const M1: i32 = 1_000_000;
    const M5: i32 = 5_000_000;
    const M7_5: i32 = 7_500_000;
    let f = |a: usize| read_f32(a) as f64;
    let of = || read_i32(offer.addr() + contract::RELEASE_FEE);
    // the three bands of cases 1 (24..28), 3 and 0: equal or higher offer fee 1.75 / 1.5 / 1.2, lower 1.7 / 1.45 / 1.15
    let banded = |top: i32| {
        let o = of();
        if cf < M1 && o < M1 {
            if o >= cf { f(rdata::F32_1_75) } else { f(F32_1_700000048) }
        } else if cf < top && o < top {
            if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
        } else if o >= cf {
            f(F32_1_200000048)
        } else {
            f(rdata::F32_1_149999976)
        }
    };
    match k {
        0 => {
            if offer.minimum_fee_rc() as i8 > 0 { banded(M7_5) } else { f(F32_1_200000048) }
        }
        1 => {
            let o = of();
            if offer.minimum_fee_rc() as i8 <= 0 {
                if cf < M1 && o < M1 { f(F32_1_200000048) } else if cf < M5 && o < M5 { f(rdata::F32_1_049999952) } else { f(F32_1_00999999) }
            } else if age > 28 {
                if cf < M1 && o < M1 { f(rdata::F32_1_100000024) } else { f(rdata::F32_1_049999952) }
            } else if age < 24 {
                if cf < M1 && o < M1 {
                    if o < cf { f(F32_1_950000048) } else { 2.0 } // the original doubles (fadd st0, st0)
                } else if cf < M5 && o < M5 {
                    if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
                } else {
                    f(rdata::F32_1_25)
                }
            } else {
                banded(M5)
            }
        }
        2 => {
            if cf < M5 && of() < M5 { f(F32_1_799999952) } else { f(F32_1_200000048) }
        }
        3 => {
            if offer.relegation_rc() as i8 <= 0 { f(rdata::F32_1_100000024) } else { banded(M5) }
        }
        _ => read_f64(rdata::F64_1_25),
    }
}

/// The value of contract `this` for `person`, against `offer` (another contract, usually the desired one): the wage
/// with the bonuses, the spread fee and the release clauses, times a length factor; used by the contract
/// satisfaction (`get_contract_satisfaction`) and by the decision on contract offers (0x4ce4e0).
///
/// The local contract is a copy of `offer` or, without an offer, the person's desired contract at the club of `this`
/// (`exe::staff_contract_set_to_desired_contract` with −1, −1 and 0xb); only its end year is read afterwards. Fewer
/// than 7 days to the end of `this` (`exe::date_days_since` of `contract::CONTRACT_EXPIRES`, a word): 0. The value
/// starts at the wage (`contract::WAGE`). With a player sheet (`.player()`), by the position mask
/// (`exe::formation_get_position_mask`): goalkeeper (bit 1) + clean sheet bonus × 0.8 (band 33); defender (6) + goal
/// bonus × 0.3 + assist bonus × 0.2 (band 31); midfielder (0x38) × 0.4 and × 0.7 (band 29); attacker (0x40) × 0.8 and
/// × 0.5 (band 28); a bonus of −1 does not count (the last matching group sets the band). Plus `contract::UNKNOWN` /
/// (days × 0.2222222). A sheet with world reputation (`.world_reputation()`) above 4000 and reputation² × 0.01 above
/// `contract::UNKNOWN`: × 0.5. Each active release clause of `this` (`contract::NON_PROMOTION_RC` + k above 0)
/// multiplies the value (`contract_value_clause_factor`). The length factor: a = band − age (no sheet: the age),
/// within −5..5; base 0.75 when the offer has a minimum fee clause, 0.875 when a > 0 and the offer has a relegation or
/// non-promotion clause, otherwise 1; with a < 0 and a sheet, × (1 − a × 0.01); raised (`game_pow`) to |end year of
/// `this` − end year of the local contract|, within 0.5..2. The value is divided by it when `this` ends in an earlier
/// year, otherwise multiplied; truncated (`ftol`). Defects of the original: without an offer it reads the clauses of
/// the offer through a null pointer (every caller passes one); without a sheet the age stands for "band − age".
/// Original: `STAFF_CONTRACT::calc_contract_value` (0x866830, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_calc_contract_value(this: usize, person: usize, offer: usize) -> i32 {
    use contract_value_rdata::*;
    let c = Contract(this);
    let p = Staff(person);
    let o = Contract(offer);
    // the local STAFF_CONTRACT as the original sets it up
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    exe::date_date_54a110(l + contract::DATE_STARTED);
    exe::date_date_54a110(l + contract::CONTRACT_EXPIRES);
    write_i32(l + contract::CLUB, -1);
    write_i32(l + contract::TRANSFER_ARRANGED_FOR, -1);
    if offer == 0 {
        // 0x85fc50: push 0; push 0; push 0xb; push -1; push -1; push club; push person; lea ecx, local -> thiscall, ret 0x1c
        let club = read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE));
        exe::staff_contract_set_to_desired_contract(l, p, club, -1, u32::MAX, 0xb, 0, 0);
    } else {
        // the original copies the offer field by field (the two dates through Date::operator=): the same 0x50 bytes
        core::ptr::copy_nonoverlapping(offer as *const u8, l as *mut u8, 0x50);
    }
    let days = exe::date_days_since(this + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as i16;
    if days < 7 {
        return 0;
    }
    covered!("mkd_1 contract value");
    let age = p.age() as i8;
    let wage = c.wage();
    let mut v32 = wage as f32; // [esp+0x84]
    let sheet = p.player();
    let mut band: i8 = -1;
    let mut st: f64;
    if !sheet.is_null() {
        let mask = exe::formation_get_position_mask(sheet) as u8;
        st = v32 as f64;
        if mask & 1 != 0 {
            let cs = c.clean_sheet_bonus();
            if cs > -1 {
                st = cs as f64 * read_f64(rdata::F64_0_8) + v32 as f64;
            }
            band = 0x21;
        }
        let bonus = |st: f64, kg: usize, ka: usize| {
            let mut s = st;
            if c.goal_bonus() > -1 {
                s += c.goal_bonus() as f64 * read_f64(kg);
            }
            if c.assist_bonus() > -1 {
                s += c.assist_bonus() as f64 * read_f64(ka);
            }
            s
        };
        if mask & 6 != 0 {
            st = bonus(st, rdata::F64_0_3, rdata::F64_0_2);
            band = 0x1f;
        }
        if mask & 0x38 != 0 {
            st = bonus(st, rdata::F64_0_4, rdata::F64_0_7);
            band = 0x1d;
        }
        if mask & 0x40 != 0 {
            st = bonus(st, rdata::F64_0_8, rdata::F64_0_5); // mkd1: the attacker's bonuses
            band = 0x1c;
        }
    } else {
        st = wage as f64; // the exact integer (the original keeps the fild on the stack)
    }
    let spread = read_i32(this + contract::UNKNOWN);
    st = st + spread as f64 / (days as i32 as f64 * read_f32(F32_0_2222222239) as f64); // mkd1: the spread fee
    v32 = st as f32;
    if !sheet.is_null() && sheet.world_reputation() > 4000 {
        let rep = sheet.world_reputation() as i32;
        st = v32 as f64;
        if game_pow(rep as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_01) > spread as f64 {
            st *= read_f32(rdata::F32_0_5) as f64; // the original also stores it in the slot the clause loop then reuses
        }
    }
    // the release clauses of this contract
    let cf = read_i32(this + contract::RELEASE_FEE);
    for k in 0..5usize {
        if read_i8(this + contract::NON_PROMOTION_RC + k) <= 0 {
            continue;
        }
        covered!("mkd_1 release clause");
        st *= contract_value_clause_factor(k, o, cf, age);
    }
    v32 = st as f32;
    // the length factor
    let years = (read_u16(this + contract::CONTRACT_EXPIRES_YEAR) as i32 - read_u16(l + contract::CONTRACT_EXPIRES_YEAR) as i32) as f32;
    let mut a: i8 = if band != -1 { band.wrapping_sub(age) } else { age };
    a = a.clamp(-5, 5);
    let mut f: f32 = 1.0;
    if o.minimum_fee_rc() as i8 > 0 {
        f = 0.75;
    } else if a > 0 && (o.relegation_rc() as i8 > 0 || o.non_promotion_rc() as i8 > 0) {
        f = 0.875;
    }
    if a < 0 && !sheet.is_null() {
        covered!("mkd_1 older than the band");
        f = ((read_f64(rdata::F64_1) - a as i32 as f64 * read_f64(rdata::F64_0_01)) * f as f64) as f32;
    }
    let mut q = game_pow(f as f64, (years as f64).abs()); // mkd1: the length factor
    if q < read_f64(rdata::F64_0_5) {
        q = read_f64(rdata::F64_0_5);
    } else if q > read_f64(rdata::F64_2) {
        q = read_f64(rdata::F64_2);
    }
    ftol(if years < read_f32(rdata::F32_0) { v32 as f64 / q } else { q * v32 as f64 })
}

compare_value!(compare_staff_contract_calc_contract_value, staff_contract_calc_contract_value, IDX_STAFF_CONTRACT_CALC_CONTRACT_VALUE, (person: usize, offer: usize) -> i32);

// ---------- batch 288: the default wage of a person at a club (staff_contracts.cpp) ----------

/// The `.rdata` constants of `staff_contract_default_staff_contract` that `rdata_consts.rs` does not name yet (value in
/// cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod default_wage_rdata {
    pub(crate) const F64_20000: usize = 0x968888; // f64 = 20000
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250
    pub(crate) const F64_25000: usize = 0x9688d0; // f64 = 25000
    pub(crate) const F64_1_125: usize = 0x96a5a8; // f64 = 1.125
    pub(crate) const F32_0_2000000030: usize = 0x96b028; // f32 = 0.20000000298023224
    pub(crate) const F32_30: usize = 0x96d880; // f32 = 30
    pub(crate) const F64_350: usize = 0x96f870; // f64 = 350
    pub(crate) const F32_75000: usize = 0x96fe94; // f32 = 75000
    pub(crate) const F32_105000: usize = 0x96feac; // f32 = 105000
    pub(crate) const F32_85000: usize = 0x96feb0; // f32 = 85000
    pub(crate) const F64_1500: usize = 0x96feb8; // f64 = 1500
    pub(crate) const F64_3250: usize = 0x96fec0; // f64 = 3250
    pub(crate) const F64_2750: usize = 0x96fec8; // f64 = 2750
    pub(crate) const F64_0_00017: usize = 0x96fed0; // f64 = 0.00017
    pub(crate) const F32_550: usize = 0x96fed8; // f32 = 550
    pub(crate) const F32_1_375: usize = 0x96fedc; // f32 = 1.375
    pub(crate) const F32_0_0125: usize = 0x96fee0; // f32 = 0.012500000186264515
    pub(crate) const F32_0_0004: usize = 0x96fee4; // f32 = 0.00039999998989515007
    pub(crate) const F32_175: usize = 0x96fee8; // f32 = 175
    pub(crate) const F32_11000: usize = 0x96feec; // f32 = 11000
    pub(crate) const F32_50000: usize = 0x96ff04; // f32 = 50000
    pub(crate) const F32_55000: usize = 0x96ff08; // f32 = 55000
    pub(crate) const F64_0_00012: usize = 0x96ff10; // f64 = 0.00012
    pub(crate) const F64_1ENEG_09: usize = 0x96ff18; // f64 = 1e-09
    pub(crate) const F32_275: usize = 0x96ff20; // f32 = 275
    pub(crate) const F64_5ENEG_09: usize = 0x96ff28; // f64 = 5e-09
    pub(crate) const F32_750: usize = 0x96ff30; // f32 = 750
    pub(crate) const F32_25000: usize = 0x96ff34; // f32 = 25000
    pub(crate) const F64_2_5ENEG_08: usize = 0x96ff38; // f64 = 2.5e-08
    pub(crate) const F64_7250: usize = 0x96ff40; // f64 = 7250
    pub(crate) const F32_44000: usize = 0x96ff48; // f32 = 44000
    pub(crate) const F32_60000: usize = 0x96ff4c; // f32 = 60000
    pub(crate) const F32_67750: usize = 0x96ff50; // f32 = 67750
    pub(crate) const F32_27500: usize = 0x96ff54; // f32 = 27500
    pub(crate) const F32_22000: usize = 0x96ff58; // f32 = 22000
    pub(crate) const F32_37500: usize = 0x96ff5c; // f32 = 37500
    pub(crate) const F32_70000: usize = 0x96ff60; // f32 = 70000
    pub(crate) const F32_9350: usize = 0x96ff64; // f32 = 9350
    pub(crate) const F32_82500: usize = 0x96ff68; // f32 = 82500
    pub(crate) const F32_92500: usize = 0x96ff6c; // f32 = 92500
    /// "STAFF_CONTRACT::default_staff_contract() - No player datafor player: '%s %s' - id %ld"
    pub(crate) const NO_PLAYER_DATA_FORMAT: usize = 0xa83968;
    /// The game's text buffer (2000 bytes) that the message above is formatted into.
    pub(crate) const TEXT_BUFFER: usize = 0xde1f64;
}

/// The default wage (a float, per week) of `person` at `club`: what the person is worth as a wage there, before the
/// will and the negotiation. Called by the desired contract (`set_to_desired_contract`, three times), the will to join
/// (`staff_interest_in_joining_club`, the wage gate), the staff's will (0x844780), the bids (0x842e10, 0x83f3a0), the
/// wage bill (`update_wage_bill`) and 0x85e960.
///
/// First the person's contract (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) and its club
/// (`cur`, none for a contract of type 5). The level: with a club, reputation / 500 + 1 (a byte), and with a current
/// club and a club of professional status 2, minus one within 1..4; without a club, the world reputation of the player
/// (or non-player) sheet / 500, at least 1. The job: the club job (`job_for_club`) when there is a current club, else
/// the national job, else the club job, else 11 for a player, else the national job, else 9.
///
/// Staff jobs (5..10): the level rises to the current club's (reputation / 500 + 1) or, for a non-player with a
/// national job other than 11, to the national team's; the base is the non-player current reputation³ × k1 or, without
/// a non-player sheet, (id / staff count × 4 + 1) × k2 × 0.0001; the wage = level × base × 0.25, then per job: 5 at
/// least 750 and above 25000 → 150000; 6 and 7 at least 500; 8 within 275..5000; 9 within 250..1000; 10 within 200..1000.
/// For jobs 6, 7 and 8 with a current club other than `club`, 150000 (the marker of "not available") when the current
/// club is more than three levels above (job 6: also unless the asked job is 11 or 6; then below 2500 × 1.35), and for
/// 6 and 7 without that, above 25000 → 150000 unless the asked job is 11 or 6. Job 14 and the others: 1000.
///
/// Players (jobs 11, 12, 13, 15): without a player sheet the error message goes to the text buffer and the wage is a
/// draw `rand() × 4 / 32768 × 550`. With a sheet: an unknown squad status (−1) is worked out from the current
/// reputation against the club's reputation and the age (1 star, 2 important, 4 squad, 5 or 6 by the potential); the
/// base by `kind` (3: 85000; 1: 92500 with loyalty < 15 and pressure > 15, else 75000; 2: 75000 from 23 to 31 years,
/// else 82500), for an old (36+) coach asked as job 13 or 15 10000, else by the squad status (5 or 6: 9350 below 2500
/// of reputation, 11000 below 6250) and the best position (attacker, then midfielders, goalkeeper, defenders, with the
/// reputation, age and loyalty bands of the table); a young (19..31) player not tied to the club or its nation whose
/// world reputation passes (loyalty + 230) × 25 − ambition × 10 rises to 75000 / 92500 / 105000; with the FIFA rules,
/// above 11000 at a club of a major nation × 1.15 / 1.125 / 1.1 (current reputation above 6750, by loyalty) or × 1.025
/// (above 4250). Then the reputation sum u = (current + home + world + 4 × max + 50 × (ability + potential)) / 9 +
/// (id / staff count × 8 − 4), at least 1500 or × 1.05 for a foreigner (another region than the club's and the current
/// club's), at least 1; the wage = base × u² × 1e-8, at least 175; × 0.75 or × 0.5 by the state of development of the
/// nation; × the world reputation against the club's level (0.85..1.15); the level caps (2500, 2750 per level below 10
/// and 15; from 15 on, 10% above 5250 / 7500 per level in England, Spain and Italy and 3250 elsewhere); 20% above the
/// position cap (150000 attacker, 125000 midfielders, 100000 defender, 80000; × 1.1 with the FIFA rules at a major
/// nation); and, without a club job and unless he is an old (33+) international with little drive, a long-time member
/// (id % 15 ≠ 0) at a club of reputation up to 4500, × (1.375 − days at the club (at most 250, + 60 in season, 20% above
/// 30) × 0.0125), at least 0.2.
///
/// With a club: for players, the club's reputation^6 (× 0.00017 / 0.00012 / 0.0001) gives a floor by squad status
/// (the wage goes half-way up to 3 × reputation × it × 0.6..1.5 for stars, ..., × 0.02 for status 6), and a contract
/// with grievance 0x10 multiplies by 0.67 / 0.85 / 2.5 / 3.5 / 7.5 by status; then the club's nation (when the person
/// works there or is from there): Japan at least level² × 10 and 25% above 7500; England, Italy, Spain nothing; the
/// United States × 0.75 and caps by the league (MLS 250) and by age and origin (1250 / 2750 / 3250); South Korea,
/// Scotland, Germany, France, Holland, Sweden, Denmark, Norway, Belgium, Portugal, Croatia, Poland, Brazil, Argentina
/// × a factor by club reputation / 50 against the league standard × 10, with their own caps; Wales, Finland, Ireland,
/// Northern Ireland, Australia only caps; the others × 0.3 and 25% above 15000. For players: at most the wage of the
/// current club's contract with squad status 7 (when the current club is another); with `flag` or without a current
/// club (and current reputation below 2250) at most the club's maximum wage (`exe::finance_get_max_wage`; without
/// finances the wage is returned as it is); without `flag` and with a current club, an ambitious (> 10), not loyal
/// (< 6), reputed (> 6750) midfielder or attacker (a 20 in attacker, attacking midfielder or midfielder) over 20 gets
/// × 1.1..1.2 when the wage × 1.25 is below the maximum. At last the minimum: at a club of professional status 1, 75
/// under 20 years, else 100; elsewhere 75 at clubs of reputation above 2250, else 50.
///
/// Defects of the original: the third test of `kind` compares the byte with 0x14244 (never true: the 6500 floor never
/// applies); without a club, a player with a nation (or of level 15 or more) reads the club through a null pointer (no
/// caller passes it).
/// Original: `STAFF_CONTRACT::default_staff_contract` (0x864550, staff_contracts.cpp, stdcall, ret 0x18).
pub(crate) unsafe extern "stdcall" fn staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = Staff(person);
    let c = Club(club);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    let cur: usize = if contract == 0 || read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 5 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(contract + contract::CLUB).wrapping_mul(club::SIZE))
    };
    let id = read_i32(person);
    let age = p.age() as i8;
    let sheet = read_usize(person + staff::PLAYER);
    let np = read_usize(person + staff::NON_PLAYER);
    let frac = (id as f64 / read_i32(G_STAFF_COUNT) as f64) as f32;
    let rep_of = |cl: usize| read_i16(cl + club::REPUTATION) as i32;
    let mut status = status;
    // the level
    let mut level: i32;
    if club != 0 {
        level = ((rep_of(club) / 500) as u8).wrapping_add(1) as i8 as i32;
        if cur != 0 && c.professional_status() == 2 {
            level -= 1;
            if level < 1 {
                level = 1;
            } else if level > 4 {
                level = 4;
            }
        }
    } else if sheet != 0 || np != 0 {
        let q = if sheet != 0 { read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32 / 500 } else { read_i16(np + non_player::WORLD_REPUTATION) as i32 / 500 };
        level = (q.max(1) as u8) as i8 as i32;
    } else {
        level = 1;
    }
    // the job
    let national = read_usize(person + staff::NATIONAL_JOB);
    let job_nation = p.job_for_nation() as i8;
    let job_club = p.job_for_club() as i8;
    let job_sel: i8 = if cur != 0 && job_club != 0 {
        job_club
    } else if national != 0 && job_nation != 0 {
        job_nation
    } else if job_club != 0 {
        job_club
    } else if sheet != 0 {
        0xb
    } else if job_nation != 0 {
        job_nation
    } else {
        9
    };
    let asked_job = job as u8 as i8;
    // staff jobs: the level rises to the current club's or the national team's
    let raise = |level: &mut i32| {
        if cur != 0 {
            let lv = rep_of(cur) / 500 + 1;
            if lv > *level {
                *level = lv;
                return;
            }
        }
        if np != 0 && national != 0 && job_nation != 0xb {
            let lv = rep_of(national) / 500 + 1;
            if lv > *level {
                *level = lv;
            }
        }
    };
    let staff_wage = |level: i32, k1: usize, k2: usize| -> f32 {
        let base = if np != 0 {
            let t = read_i16(np + non_player::CURRENT_REPUTATION) as f64;
            t * (t * t) * d(k1)
        } else {
            (frac as f64 * f(rdata::F32_4) + f(rdata::F32_1)) * f(k2) * d(rdata::F64_0_0001)
        };
        (level as f64 * base * d(rdata::F64_0_25)) as f32
    };
    let unavailable = 150000.0f32;
    let mut value: f32;
    match job_sel as i32 - 5 {
        0 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_2_5ENEG_08, F32_25000);
            if (value as f64) < f(F32_750) {
                value = 750.0;
            } else if value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        1 | 2 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, rdata::F64_1ENEG_08, rdata::F32_10000);
            if (value as f64) < f(rdata::F32_500) {
                value = 500.0;
            }
            if cur != 0 && cur != club {
                let above = rep_of(cur) / 500 + 4 > level;
                if job_sel == 6 {
                    if (asked_job == 0xb || asked_job == 6) && !above {
                        if (value as f64) < f(rdata::F32_2500) {
                            value = (value as f64 * d(rdata::F64_1_35)) as f32;
                        }
                    } else {
                        value = unavailable;
                    }
                } else if above {
                    value = unavailable;
                } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                    value = unavailable;
                }
            } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        3 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_5ENEG_09, rdata::F32_5000);
            if (value as f64) < f(F32_275) {
                value = 275.0;
            } else if value as f64 > f(rdata::F32_5000) {
                value = 5000.0;
            }
            if cur != 0 && cur != club && asked_job != 6 && asked_job != 0xb && rep_of(cur) / 500 + 4 > level {
                value = unavailable;
            }
        }
        4 | 5 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_1ENEG_09, rdata::F32_1000);
            let floor = if job_sel == 9 { 250.0f32 } else { 200.0 };
            if (value as f64) < f(if job_sel == 9 { rdata::F32_250 } else { rdata::F32_200 }) {
                value = floor;
            } else if value as f64 > f(rdata::F32_1000) {
                value = 1000.0;
            }
        }
        6 | 7 | 8 | 10 => {
            if sheet == 0 {
                // the error message, then a draw
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(TEXT_BUFFER, NO_PLAYER_DATA_FORMAT, read_u32(person + staff::FIRST_NAME), read_u32(person + staff::SECOND_NAME), id);
                let r = exe::crt_rand();
                value = ((r as i64 * 4 / 0x8000) as i32 as f64 * f(F32_550)) as f32;
            } else {
                covered!("mkd_2 player wage");
                value = player_default_wage(person, club, cur, &mut status, asked_job, kind as u8 as i8, level, frac);
            }
        }
        _ => value = 1000.0,
    }
    if club == 0 {
        return value;
    }
    let rep = rep_of(club);
    if sheet != 0 {
        // the floor by the club's reputation and the squad status
        let x = rep as f64 * if c.professional_status() == 1 {
            d(F64_0_00017)
        } else if rep > 2250 {
            d(F64_0_00012)
        } else {
            d(rdata::F64_0_0001)
        };
        let mut r = game_pow(x, d(rdata::F64_6));
        if r > d(rdata::F64_1) {
            r = 1.0;
        }
        let three = (rep * 3) as f64;
        let half_way = |value: &mut f32, y: f64| {
            if (*value as f64) < y {
                *value = ((*value as f64 + y) * d(rdata::F64_0_5)) as f32;
            }
        };
        let major = |age: i8| age >= 24 && club_is_in_major_nation(club) != 0;
        let s = status as u8 as i8 as i32 - 1;
        match s {
            0 => {
                let k = if age < 20 { d(rdata::F64_0_6) } else if major(age) { d(rdata::F64_1_5) } else { d(rdata::F64_1_25) };
                half_way(&mut value, three * r * k); // mkd2: the floor of the stars
            }
            1 => {
                let y = if age < 20 { three * r * d(rdata::F64_0_3) } else if major(age) { three * r } else { three * r * d(rdata::F64_0_6) };
                half_way(&mut value, y);
            }
            2 => {
                if age < 20 {
                    half_way(&mut value, three * r * d(rdata::F64_0_25));
                }
                let k = if major(age) { d(rdata::F64_0_6) } else { d(rdata::F64_0_3) };
                half_way(&mut value, three * r * k);
            }
            3 => {
                let k = if age < 20 { d(rdata::F64_0_1) } else if major(age) { d(rdata::F64_0_3) } else { d(rdata::F64_0_15) };
                half_way(&mut value, three * r * k);
            }
            4 => half_way(&mut value, three * r * d(rdata::F64_0_05)),
            5 => half_way(&mut value, three * r * d(rdata::F64_0_02)),
            _ => {}
        }
        let ct = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        if ct != 0 && read_u8(ct + contract::GRIEVANCES) & 0x10 != 0 && (0..=5).contains(&s) {
            covered!("mkd_2 grievance");
            let k = [rdata::F64_0_67, rdata::F64_0_67, rdata::F64_0_85, rdata::F64_2_5, rdata::F64_3_5, rdata::F64_7_5][s as usize];
            value = (value as f64 * d(k)) as f32;
        }
    }
    // the club's nation, when the person works there or is from there
    let cn = read_usize(club + club::NATION);
    if cn != 0 {
        let e = read_usize(person + staff::CLUB_JOB);
        let sn = read_usize(person + staff::NATION);
        let go = if e == 0 {
            sn == cn
        } else {
            read_usize(e + club::NATION) == cn && (sn == cn || p.loyalty() > 3 || p.ambition() < 15)
        };
        if go {
            value = default_wage_by_nation(person, club, cn, sn, sheet, level, value);
        }
    }
    if sheet != 0 {
        if cur != 0 && club != cur {
            let ct = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, cur);
            if ct != 0 && read_u8(ct + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(ct + contract::WAGE);
                if value as f64 > w as f64 {
                    value = w as f32;
                }
            }
        }
        let max_wage = || {
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            if fin == 0 { None } else { Some(exe::finance_get_max_wage(fin, person, status)) }
        };
        if flag != 0 || cur == 0 {
            if flag != 0 || read_i16(sheet + player_sheet::CURRENT_REPUTATION) < 2250 {
                let Some(mw) = max_wage() else { return value };
                if (mw as f64) < value as f64 {
                    value = mw as f32; // mkd2: the club's maximum wage
                }
            }
        } else {
            let at = |k: usize| read_i8(sheet + k);
            if p.ambition() > 10
                && p.loyalty() < 6
                && read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 6750
                && (at(player_sheet::ATTACKER) == 20 || at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20)
                && age > 20
            {
                let Some(mw) = max_wage() else { return value };
                let loyal1 = p.loyalty() == 1;
                let k = if at(player_sheet::ATTACKING_MIDFIELDER) == 20 {
                    Some(if loyal1 { rdata::F64_1_15 } else { rdata::F64_1_1 })
                } else if value as f64 * d(rdata::F64_1_25) < mw as f64 {
                    if at(player_sheet::MIDFIELDER) == 20 {
                        Some(if loyal1 { rdata::F64_1_1 } else { rdata::F64_1_05 })
                    } else {
                        Some(if loyal1 { rdata::F64_1_2 } else { rdata::F64_1_15 })
                    }
                } else {
                    None
                };
                if let Some(k) = k {
                    covered!("mkd_2 ambitious star");
                    value = (value as f64 * d(k)) as f32;
                }
            }
        }
    }
    // the minimum
    if c.professional_status() == 1 {
        if age < 20 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_100) {
            value = 100.0;
        }
        value
    } else {
        if rep > 2250 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_50) {
            return 50.0;
        }
        value
    }
}

/// The player part of `staff_contract_default_staff_contract` (a player sheet, jobs 11, 12, 13, 15): the squad status
/// when unknown (written into the low byte of `status`), the base by `kind`, the job and the position, and the
/// adjustments up to the decline by days at the club (see the header of the routine).
unsafe fn player_default_wage(person: usize, club: usize, cur: usize, status: &mut u32, asked_job: i8, kind: i8, level: i32, frac: f32) -> f32 {
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = Staff(person);
    let c = Club(club);
    let age = p.age() as i8;
    let sheet = read_usize(person + staff::PLAYER);
    let at = |k: usize| read_i8(sheet + k);
    let w16 = |k: usize| read_i16(sheet + k);
    let sb = w16(player_sheet::CURRENT_REPUTATION);
    let sd = w16(player_sheet::WORLD_REPUTATION);
    let s9 = w16(player_sheet::HOME_REPUTATION);
    let loyalty = p.loyalty();
    // the squad status when unknown
    if *status as u8 as i8 == -1 && club != 0 {
        let rep = c.reputation();
        let new = if sb > rep && age >= 24 && age < 34 {
            1
        } else {
            let x = rep as f64 * d(rdata::F64_0_75);
            if sb as f64 > x && age >= 21 {
                2
            } else if (sb as f64 > rep as f64 * d(rdata::F64_0_5) && age >= 17) || age > 24 {
                4
            } else if (w16(player_sheet::POTENTIAL_ABILITY) as i32 * 50) as f64 > x {
                5
            } else {
                6
            }
        };
        *status = (*status & !0xff) | new;
    }
    let st_s = *status as u8 as i8;
    let v75 = f(F32_75000);
    let v92 = f(F32_92500);
    let v105 = f(F32_105000);
    // the base
    let base = if kind == 3 {
        F32_85000
    } else if kind == 1 {
        if loyalty < 15 && p.pressure() > 15 { F32_92500 } else { F32_75000 }
    } else if kind == 2 {
        if age > 22 && age < 32 { F32_75000 } else { F32_82500 }
    } else if (asked_job == 13 || asked_job == 15) && age > 35 {
        rdata::F32_10000
    } else if (st_s == 6 || st_s == 5) && sb < 2500 {
        F32_9350
    } else if (st_s == 6 || st_s == 5) && sb < 6250 {
        F32_11000
    } else if at(player_sheet::ATTACKER) >= 15 {
        if sb > 6750 && age < 31 {
            if loyalty < 15 { F32_82500 } else { F32_70000 }
        } else {
            F32_55000
        }
    } else if at(player_sheet::DEFENSIVE_MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::MIDFIELDER) >= 15 {
        if sb > 7250 && age < 31 {
            if loyalty < 14 { F32_67750 } else { F32_60000 }
        } else {
            F32_44000
        }
    } else if at(player_sheet::GOALKEEPER) == 20 {
        if sd > 7250 && loyalty < 3 && age < 34 { F32_37500 } else { F32_22000 }
    } else if at(player_sheet::SWEEPER) < 15 && at(player_sheet::DEFENDER) < 15 {
        F32_27500
    } else if sb > 7500 && loyalty < 6 && age < 33 {
        F32_50000
    } else {
        F32_27500
    };
    let mut st = f(base); // the x87 top of the stack (not always rounded to a float)
    let mut value = st as f32;
    // a young player not tied to the club or its nation, with a world reputation above his loyalty and ambition
    let mut free = true;
    if club != 0 {
        let cn = read_usize(club + club::NATION);
        if cn != 0 && (c.reputation() < 5750 || read_i8(cn + nation::LEAGUE_STANDARD) < 12) {
            let e = read_usize(person + staff::CLUB_JOB);
            if e == club || (e != 0 && read_usize(e + club::NATION) == cn) {
                free = false;
            }
        }
    }
    if age > 18 && age < 32 && free && sd as i32 > (loyalty as i32 + 230) * 25 - p.ambition() as i32 * 10 {
        let new = if at(player_sheet::GOALKEEPER) == 20 {
            if st < v75 { Some(v75) } else { None }
        } else if st == v92 {
            Some(v105)
        } else if st == v75 {
            Some(if sd > 7250 { v105 } else { v92 })
        } else if sd > 7250 {
            if st < v92 { Some(v92) } else { None }
        } else if st < v75 {
            Some(v75)
        } else {
            None
        };
        if let Some(n) = new {
            covered!("mkd_2 young star");
            st = n;
            value = n as f32;
        }
    }
    // the FIFA rules at a club of a major nation
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && st > f(F32_11000) && club != 0 {
        st = value as f64;
        if club_is_in_major_nation(club) != 0 {
            if sb > 6750 {
                st *= if loyalty == 1 { d(rdata::F64_1_15) } else if loyalty < 6 { d(F64_1_125) } else { d(rdata::F64_1_1) };
            } else if sb > 4250 {
                st *= d(rdata::F64_1_025);
            }
        }
    }
    // the reputation sum
    let mut m = if s9 > sb { s9 } else { sb };
    if sd > m {
        m = sd;
    }
    if sd < 1750 && m > 3250 {
        m = (3250 - ftol((m as f64 - d(F64_3250)) * d(rdata::F64_NEG_0_2))) as i16;
    }
    if kind == 1 {
        if m < 7250 {
            m = 7250;
        }
    } else if kind == 2 {
        if m < 6750 {
            m = 6750;
        }
    } // (the original's third test, kind == 0x14244, can never hold)
    if (st == v75 || st == v92 || st == v105) && m < 7250 {
        m = 7250;
    }
    let total = sb as i32 + s9 as i32 + sd as i32 + 4 * m as i32
        + 50 * (w16(player_sheet::POTENTIAL_ABILITY) as i32 + w16(player_sheet::CURRENT_ABILITY) as i32);
    let mut u = (total / 9).wrapping_add(ftol(frac as f64 * f(rdata::F32_8) - f(rdata::F32_4))) as u32;
    let sn = read_usize(person + staff::NATION);
    if sn != 0 {
        let cn = read_usize(club + club::NATION);
        if cn != 0 {
            let region = read_i8(cn + nation::ACTUAL_REGION);
            if read_i8(sn + nation::ACTUAL_REGION) != region {
                let foreign_too = if cur == 0 {
                    true
                } else {
                    let kn = read_usize(cur + club::NATION);
                    kn != 0 && read_i8(kn + nation::ACTUAL_REGION) != region
                };
                if foreign_too {
                    if u < 1500 {
                        u = 1500;
                    } else {
                        u = ftol(u as u64 as f64 * d(rdata::F64_1_05)) as u32;
                    }
                }
            }
        }
    }
    if u <= 1 {
        u = 1;
    }
    let product = st * u.wrapping_mul(u) as u64 as f64 * d(rdata::F64_1ENEG_08); // mkd2: the reputation squared
    value = product as f32;
    if product < f(F32_175) {
        value = 175.0;
    }
    // the state of development of the nation
    let curn = if cur != 0 { read_usize(cur + club::NATION) } else { 0 };
    let k: i8 = if curn != 0 {
        read_i8(curn + nation::STATE_OF_DEVELOPMENT)
    } else if sd > 4250 {
        1
    } else if sn != 0 {
        read_i8(sn + nation::STATE_OF_DEVELOPMENT)
    } else {
        3
    };
    if k != 1 {
        value = (value as f64 * if k == 2 { f(rdata::F32_0_75) } else { f(rdata::F32_0_5) }) as f32;
    }
    // the world reputation against the club's level
    if club != 0 && c.reputation() != 0 {
        let mut x = sd as f64;
        if c.reputation() < 2500 {
            x *= f(F32_0_0004);
        } else {
            x /= (level * 500) as f64;
        }
        if x < d(rdata::F64_0_85) {
            x = f(rdata::F32_0_8500000238);
        }
        if x > d(rdata::F64_1_15) {
            x = f(rdata::F32_1_149999976);
        }
        value = (x * value as f64) as f32;
    }
    // the level caps
    let squeeze = |value: &mut f32, cap: i32| {
        if *value as f64 > cap as f64 {
            *value = ((*value as f64 - cap as f64) * d(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if level < 10 || level < 15 {
        let cap = if level < 10 { level * 2500 } else { level * 2750 };
        if value as f64 > cap as f64 {
            value = cap as f32;
        }
    } else {
        let cn = read_usize(club + club::NATION);
        let key = if cn != 0 { read_i32(cn) } else { 0 };
        if cn != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) {
            squeeze(&mut value, if read_u8(club + club::PLC) != 0 { level * 5250 } else { level * 7500 });
        } else {
            squeeze(&mut value, level * 3250);
        }
    }
    // the cap of the position
    let mut cap: f32 = if at(player_sheet::ATTACKER) == 20 {
        150000.0
    } else if at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20 || at(player_sheet::DEFENSIVE_MIDFIELDER) == 20 {
        125000.0
    } else if at(player_sheet::DEFENDER) == 20 {
        100000.0
    } else {
        80000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && club != 0 && club_is_in_major_nation(club) != 0 && sb > 7250 {
        cap = (cap as f64 * d(rdata::F64_1_1)) as f32;
    }
    if value > cap {
        value = ((value as f64 - cap as f64) * d(rdata::F64_0_2) + cap as f64) as f32;
    }
    // the decline by the days at the club
    if read_usize(person + staff::CLUB_JOB) == 0 {
        let settled = age >= 33
            && p.int_apps() != 0
            && at(player_sheet::WORK_RATE) as i32 + p.determination() as i32 <= 30
            && read_i32(person) % 15 != 0
            && c.reputation() <= 4500;
        if !settled {
            covered!("mkd_2 days at the club");
            let days = exe::date_days_since(G_CURRENT_DATE, read_u32(person + staff::DATE_JOINED_CLUB), read_u32(person + staff::DATE_JOINED_CLUB_LEAP_YEAR));
            let mut x = if days as f64 > f(rdata::F32_250) { 250.0f32 } else { days as f32 } as f64;
            if exe::club_is_in_season(club, G_CURRENT_DATE) != 0 {
                x += f(rdata::F32_60);
            }
            if x > f(F32_30) {
                x = (x - f(F32_30)) * d(rdata::F64_0_2) + d(rdata::F64_30);
            }
            let mut k = f(F32_1_375) - x * f(F32_0_0125); // mkd2: the decline by days
            if f(F32_0_2000000030) > k {
                k = f(F32_0_2000000030);
            }
            value = (k * value as f64) as f32;
        }
    }
    value
}

/// The part of `staff_contract_default_staff_contract` by the club's nation `cn` (the person works there or is from
/// there; see the header of the routine).
unsafe fn default_wage_by_nation(person: usize, club: usize, cn: usize, sn: usize, sheet: usize, level: i32, value: f32) -> f32 {
    use default_wage_rdata::*;
    let d = |a: usize| read_f64(a);
    let c = Club(club);
    let mut value = value;
    let q = c.reputation() as i32 / 50;
    let key = read_i32(cn);
    let is = |k: usize| key == read_i32(k);
    let league = read_i8(cn + nation::LEAGUE_STANDARD);
    let over = q as i16 as i32 > league as i32 * 10;
    // the club's reputation against the league standard, within k..1
    let factor = |k: usize| {
        let mut x = d(k);
        if over {
            let dv = if league < 1 { 1 } else { league as i32 };
            x = q as i16 as f64 / dv as f64 * d(rdata::F64_0_1) * d(k);
            if x > d(rdata::F64_1) {
                x = d(rdata::F64_1);
            }
        }
        x
    };
    let scale = |value: &mut f32, x: f64| *value = (*value as f64 * x) as f32;
    let squeeze = |value: &mut f32, cap: usize, k: usize| {
        if *value as f64 > d(cap) {
            *value = ((*value as f64 - d(cap)) * d(k) + d(cap)) as f32;
        }
    };
    let pro = c.professional_status() == 1;
    let age = read_i8(person + staff::AGE);
    if is(KEY_NATION_JAPAN) {
        let floor = level * level * 10;
        if (value as f64) < floor as f64 {
            value = floor as f32;
        }
        squeeze(&mut value, rdata::F64_7500, rdata::F64_0_25);
    } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
    } else if is(KEY_NATION_UNITED_STATES) {
        scale(&mut value, d(rdata::F64_0_75));
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_i32(comp) == read_i32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
            squeeze(&mut value, F64_250, rdata::F64_0_15);
        } else if sn == cn {
            let sh = read_usize(person + staff::PLAYER);
            if age > 31 || (age < 24 && sh != 0 && read_i16(sh + player_sheet::WORLD_REPUTATION) < 3750) {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_2);
            } else {
                squeeze(&mut value, F64_2750, rdata::F64_0_2);
            }
        } else {
            squeeze(&mut value, F64_3250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_SOUTH_KOREA) {
        scale(&mut value, factor(rdata::F64_0_25));
        squeeze(&mut value, rdata::F64_500, rdata::F64_0_5);
        squeeze(&mut value, F64_1500, rdata::F64_0_5);
    } else if is(KEY_NATION_SCOTLAND) {
        scale(&mut value, factor(rdata::F64_0_25));
        if over {
            squeeze(&mut value, rdata::F64_10000, rdata::F64_0_25);
        } else if sheet != 0 {
            let sd = read_i16(sheet + player_sheet::WORLD_REPUTATION);
            if sd < 1250 {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_25);
            } else if (age < 20 || age > 32) && sd < 2750 {
                squeeze(&mut value, F64_3250, rdata::F64_0_25);
            } else {
                squeeze(&mut value, F64_7250, rdata::F64_0_25);
            }
        } else {
            squeeze(&mut value, F64_7250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
        scale(&mut value, factor(rdata::F64_0_75));
        squeeze(&mut value, F64_20000, rdata::F64_0_25);
    } else if is(KEY_NATION_SWEDEN) {
        scale(&mut value, factor(rdata::F64_0_25));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, rdata::F64_150, rdata::F64_0_2) }
    } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
        if pro { squeeze(&mut value, rdata::F64_200, rdata::F64_0_15) } else { squeeze(&mut value, rdata::F64_100, rdata::F64_0_15) }
    } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) {
        scale(&mut value, factor(rdata::F64_0_5));
        if pro { squeeze(&mut value, F64_350, rdata::F64_0_1) } else { squeeze(&mut value, F64_250, rdata::F64_0_1) }
    } else if is(KEY_NATION_BELGIUM) || is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) || is(KEY_NATION_POLAND) {
        scale(&mut value, factor(if is(KEY_NATION_BELGIUM) { rdata::F64_0_45 } else { rdata::F64_0_5 }));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, F64_250, rdata::F64_0_2) }
    } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
        scale(&mut value, factor(rdata::F64_0_45));
        squeeze(&mut value, F64_25000, rdata::F64_0_25);
    } else {
        covered!("mkd_2 other nation");
        let pv = value as f64 * d(rdata::F64_0_3); // mkd2: the other nations
        value = pv as f32;
        if pv > d(rdata::F64_15000) {
            value = ((value as f64 - d(rdata::F64_15000)) * d(rdata::F64_0_25) + d(rdata::F64_15000)) as f32;
        }
    }
    value
}

/// Comparer for `staff_contract_default_staff_contract`: the original and the copy from the same state of the last
/// contract looked up (`G_CONTRACT_LOOKUP_RESULT`) and of the CRT's rand() (the draw of a player without a sheet), the
/// returned float bit for bit.
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f32>(IDX_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4), (0xad6c30, 4)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(person, club, flag, status, job, kind),
        || crate::rng::without_counting(|| staff_contract_default_staff_contract(person, club, flag, status, job, kind)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_default_staff_contract(person {:x}, club {:x}, {}, {:#x}, {:#x}, {:#x}) original={} rust={} {:x?} x {:x?}",
        person, club, flag, status, job, kind, r, n, mo, mr));
    r
}

// ---------- batch 288: the desired contract of a person at a club (staff_contracts.cpp) ----------

/// The `.rdata` constants and addresses of `staff_contract_set_to_desired_contract` that `rdata_consts.rs` does not
/// name yet (value in cm0102.exe 3.9.68; still read from the exe).
#[allow(dead_code)]
pub(crate) mod desired_contract_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250
    pub(crate) const F64_2500000: usize = 0x96a650; // f64 = 2500000
    pub(crate) const F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
    pub(crate) const F64_0_21: usize = 0x96d398; // f64 = 0.21
    pub(crate) const F32_8250: usize = 0x96d4e4; // f32 = 8250
    pub(crate) const F32_0_0666666701: usize = 0x96d888; // f32 = 0.06666667014360428
    pub(crate) const F32_0_125: usize = 0x96e2bc; // f32 = 0.125
    pub(crate) const F64_150000: usize = 0x96fe58; // f64 = 150000
    pub(crate) const F32_0_0133333337: usize = 0x96fe68; // f32 = 0.013333333656191826
    pub(crate) const F32_0_0199999996: usize = 0x96fe6c; // f32 = 0.019999999552965164
    pub(crate) const F32_0_0399999991: usize = 0x96fe70; // f32 = 0.03999999910593033
    pub(crate) const F32_1250: usize = 0x96fe74; // f32 = 1250
    pub(crate) const F32_0_0250000004: usize = 0x96fe78; // f32 = 0.02500000037252903
    pub(crate) const F32_NEG_0_5: usize = 0x96fe7c; // f32 = -0.5
    pub(crate) const F32_3200: usize = 0x96fe80; // f32 = 3200
    pub(crate) const F32_4100: usize = 0x96fe84; // f32 = 4100
    pub(crate) const F32_125000: usize = 0x96fe88; // f32 = 125000
    pub(crate) const F32_57500: usize = 0x96fe8c; // f32 = 57500
    pub(crate) const F32_17500: usize = 0x96fe90; // f32 = 17500
    pub(crate) const F32_75000: usize = 0x96fe94; // f32 = 75000
    pub(crate) const F32_35000: usize = 0x96fe98; // f32 = 35000
    pub(crate) const F32_250000: usize = 0x96fe9c; // f32 = 250000 (the "not available" marker)
    pub(crate) const F32_200000: usize = 0x96fea0; // f32 = 200000
    pub(crate) const F32_0_1500000060: usize = 0x96fea4; // f32 = 0.15000000596046448
    pub(crate) const F32_NEG_0_25: usize = 0x96fea8; // f32 = -0.25
    pub(crate) const F32_105000: usize = 0x96feac; // f32 = 105000
    pub(crate) const F32_85000: usize = 0x96feb0; // f32 = 85000
    /// "STAFF_CONTRACT::set_to_desired_contract() - Staff %d has no job for club"
    pub(crate) const NO_JOB_FORMAT: usize = 0xa8391c;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\staff_contracts.cpp" (the source path of the error box).
    pub(crate) const SOURCE_PATH: usize = 0xa8360c;
    /// The game's text buffer (2000 bytes) the message above is formatted into.
    pub(crate) const TEXT_BUFFER: usize = 0xde1f64;
    /// The release clause byte at contract +0x1e (between `contract::MINIMUM_FEE_RC` and `contract::RELEGATION_RC`):
    /// the "not playing" clause [inference: set with the reputation against the club's level, minimum 1500000].
    pub(crate) const NON_PLAYING_RC: usize = 0x1e;
}

/// Fills `this` (a STAFF_CONTRACT) with the contract `person` wants at `club`: the signing fee (+8), the wage (+0xc),
/// the bonuses, the release clauses, the dates and the kind; returns the contract's value (an int) that the callers
/// compare.
///
/// Start: the release fee and the manager-job clause are zeroed; the person's contract
/// (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives the current club `cur` (none for a
/// contract of type 5); without `club`, the club is `cur` (or its linked B team, `exe::club_get_linked_b_team`, when
/// `cur` is not the A team); with `cur`, the current contract `cc` is the one with `cur`
/// (`exe::contract_manager_get_contract_for_club`). The contract gets `contract::CONTRACT_TYPE` = (old & 0x40) | 2,
/// `contract::LEAVING_ON_BOSMAN` 0, `contract::TRANSFER_ARRANGED_FOR` −1, the appearances 0, the person's id and the
/// squad number 0. A status of −1 for a player becomes the deserved one against `cc`
/// (`exe::contract_manager_get_deserved_squad_status`; with grievance 0x10 and not 1, 2) or, without `cc`, the status
/// for the club (`exe::shortlist_manager_get_squad_status_for_club`); it goes to the high nibble of
/// `contract::SQUAD_STATUS`; the grievances of `cc` are kept.
///
/// The level (a byte): with a club, reputation / 500 + 1 (professional status 2: minus one within 1..4) and the club's
/// id at `contract::CLUB`; without one, the world reputation of the player sheet / 500 (no non-player sheet) or the
/// national team's reputation / 500 + 1, at least 3, and the club −1. Internationals add ftol(league standard × 0.25 ×
/// caps × 0.125), at most 4 (0 → 1; no nation → 1). The reputation used by the clauses and the length is the player's
/// current reputation (a player job) or the non-player's current reputation, at least 1.
///
/// The signing fee (player job): `exe::contract_manager_calc_player_value` × 2 / 10, × 0.25 (no club job, current
/// reputation < 2750, club < 2000), × 0.5 (< 4250) or × 0.75 (< 4750); a renewal (club = `cur`): 0 under 19 with
/// < 3750, × 0.1 / 0.5 / 0.25 / 0.5 under 21 with < 4250 (by reputation and pressure), × 0.5 at a club below 3500,
/// then by the deserved status × 0.9 / 0.75 / 0.75 / 0.5 / 0.25; a new club: × 0.5 when `cur` is below 4250 and × 0.5
/// when its nation's league standard is below 10. Staff: 0.
///
/// No club job (`staff::JOB_FOR_CLUB` 0): the "has no job for club" message and the job 11 (player sheet) or 9.
///
/// Staff (not a player job, or no player sheet): w = the default wage (`exe::staff_contract_default_staff_contract`,
/// flag 0, kind 0); the will `exe::staff_interest_in_job_at_club` (written to `will`); w + 2500 with will < 1.75; the
/// value = w × max(3.5 / max(will, 0.1), 0.9); 250000 (the "not available" marker) for job 6 above 2 × the club's
/// reputation + 1200 and 4100, job 8 above reputation + 500 and 3200, job 9 above 0.75 × reputation + 250 and 2100, job
/// 10 the same with 1500, and anything above 250000.
///
/// Players: w = the default wage with kind 0 and job 11. A renewal asks flag 1 (at most the club's maximum wage) when it
/// is cheap: no grievance, no low-nibble mark in the staff state record, current reputation < 7250, fewer than loyalty
/// × 12 + 182 days to the end of `cc` and not leaving on a Bosman; or pressure < 6, under 24, no future transfer
/// (`exe::transfer_manager_get_future_transfer`), not listed (`exe::staff_is_transfer_listed_or_requested`), current
/// reputation < 3250 and < the club's + 250, fewer than 200 − pressure days; or grievances without the bits 0x161a8,
/// fewer than id % 35 + 20 days, no mark and current reputation + ambition × 20 < (loyalty + 170) × 25; or (loyalty
/// > 18, or below 5250 without the mark) with world reputation < 3500. A new club: kind 3 above 6750, else 0 (the tests
/// (100 − ambition − loyalty) × 125000 < 0 and (80 − ...) < 0, which would give kinds 1/2 and status 1/2, never hold
/// with attributes 1..20). Above the current wage: at most max(2 × wage, wage + 8000), and for the famous (world >
/// 6750, w > 10000) above that, 75000 (England, Spain, Italy) or 50000 when the club is `cur` or strong (comp or league
/// reputation × club reputation × 0.05 > 5750), 25% above it. The drive t = world + 25 × (ambition − loyalty) + 10 ×
/// pressure (16 bits): a star at a happy contract, or t above the club's reputation, or a renewal of a deserved star
/// with low loyalty, high ambition and pressure, with fewer than pressure × 10 + 525 days left, left out twice or world
/// ≥ 5750, at a club of a major nation (`club_is_in_major_nation`), 19 to 31 years: w rises to ref × x, ref = the
/// highest wage (`finance::HIGHEST_WAGE`) or 0.0001 × t × 85000 (England) / 105000 (Spain, Italy) when higher (60000
/// for a goalkeeper), x = (ambition − professionalism − loyalty + 4) × 0.0075 within −0.25..0.15, minus 0.1 / 0.05 by
/// age, minus 0.05 below the club's reputation − 1250, plus 0.45 (goalkeeper), 0.7 (defender), 1.0 (attacker, t >
/// 7250, loyalty < 6), 0.95 (t > 7250) or 0.9.
///
/// The will `exe::shortlist_manager_staff_interest_in_joining_club` (written to `will`), q = 3.5 / max(will, 0.1)
/// (stored as f32); with `cc`: grievance 2 caps q at 1.25; other grievances with will < 1.75 add 5000 to w for the
/// restless (pressure > 6, over 23, a future transfer, listed, current reputation > 3249 or above the club's + 250,
/// more than 200 − pressure days); without `cc`, w + 5000 with will < 1.75. q at least 0.9; a renewal caps q: no
/// contract 3.0; grievance 2 (or the young one with exactly 0x10) 1.2, grievance 1 1.75, 0x8400 1.5, others 4.0; no
/// grievance: 1.25 for the loyal (loyalty > id % 5 + 10) with fewer than 30 days, else by age and deserved status (under
/// 19 or over 34: 2.0 / 1.35 / 1.25 / 1.15; under 21 or over 31: 2.0 / 1.5 / 1.35 / 1.15; else 2.0) and 1.1 for the
/// young (< 20, ability < 125 − pressure − ambition, current reputation < 4250, fewer than 120 − pressure − ambition
/// days). The value = q × w; with `cc` (not `this`) paying more, at least its wage × the clubs' reputation ratio (at
/// least 0.5); with the grievance 0x1000000, at least 95% of the highest wage. The marker 250000: above 200000; above
/// 35000 over 34; above 75000 over 31; goalkeeper 20 above 105000; status 7 above 5000, 6 above 8250, 5 above 17500,
/// 4 above 35000, 3 above 57500 with world < 6750 (a human club, or current reputation ≤ 6250, or above 85000 / 105000
/// / 125000 by position and reputation), 2 above 125000 with world < 7250. A dearer current contract at another club
/// blends: value = ((20 − a) × its wage + a × min(w, max(w × 0.5, the maximum wage))) × 0.05, a = 0..20 (grievance
/// 0x100 +10, listed +10, wage above 40000, age over 33, unhappiness and morale), with `exe::finance_get_max_wage`
/// called two to four times.
///
/// Clauses (job 11 with a player sheet): +0x1d 0 and +0x20 1 for the old international (or a non-player sheet); the
/// non-promotion clause (+0x1c, the league's `league_vtable::HAS_AUTOMATIC_PROMOTION`), relegation (+0x1f,
/// `league_vtable::HAS_RELEGATION`), minimum fee (+0x1d) and not playing (+0x1e) with the fee (+0x21) from
/// `exe::contract_manager_calc_player_value` (minimums 150000 / 50000 / 150000 / 1500000); a clause the current
/// contract had at the same club and that is now dropped multiplies the value by 1.05 (1.15 for the minimum fee).
///
/// Dates (with a club): a signing elsewhere gets the window date (`find_next_window_open_date` with no offer,
/// `exe::transfer_manager_find_next_allowed_transfer_date` by the offer's slot), then `contract::DATE_STARTED` is set
/// to today anyway; the end is day `exe::utils_random(28)` + 1 of the season end month
/// (`exe::contract_manager_season_end_month`) of this year (next year when the key nation's season spans two years) +
/// ftol(L + 1) years, L by age and position (3, 4, 3 / 2 / 1.5, 4 or 1), × 0.75 with a clause, by the level against the
/// reputation and the loyalty, by the squad status, by the current reputation, × 0.5 in Finland and Turkey (at most 3),
/// at most 2 / 3 / 4 for a wage below 500 / 1000 / 2000, within 1..`G_MAX_CONTRACT_LENGTH_YEARS` − 1; the kind is 2
/// (with the bit 0x80 when there is no club job, the level is below 2 and a fixed draw by id is 0; in that last case,
/// up to 20 years, kind 1 without the years); a renewal never moves the end before `staff::DATE_EXPIRES_CLUB`.
///
/// Bonuses (+0x10 / +0x14 / +0x18) = ftol(value × draw × 0.025 + value × 0.025 + 25) for attacker / midfielder /
/// goalkeeper 20, else −1; the bit 0x80 of the kind is set below 125 at a club of professional status 2 and cleared
/// above 1250; `contract::WAGE` = ftol(value). The return: ftol(wage (× 0.5 for kind 1, × 1.05 with 0x80) + min(the
/// bonuses / 10 and the clause weights × value, value × 0.1)).
///
/// Defects of the original kept: with an offer without a slot (`exe::transfer_manager_get_offer_slot` 0) it reads the
/// address 0 (`&&` where `||` was meant) before the "Error" box; the reputation of the clauses and the length is left
/// unset (a stack leftover) for a person with a player sheet, a staff job and no non-player sheet (here 0, then 1); the
/// window date is overwritten with today; the kind 1/2 branch of a new club never runs.
/// Original: `STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, staff_contracts.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    use desired_contract_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let ct = this;
    let cm = || read_usize(G_CONTRACT_MANAGER);
    let tm = || read_usize(G_TRANSFER_MANAGER);
    let shortlist = || read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let today = || (read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
    let days_to = |c: usize| {
        let (lo, hi) = today();
        exe::date_days_since(c + contract::CONTRACT_EXPIRES, lo, hi)
    };
    // the club of the person's contract (none for type 5), with the lookup kept in G_CONTRACT_LOOKUP_RESULT
    let lookup = || {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
            0
        } else {
            read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE))
        }
    };
    let is_player_job = |j: u32| exe::database_is_player_job(j as u8 as i8) != 0;
    let rep_of = |c: usize| read_i16(c + club::REPUTATION);
    let mut club = club;
    let mut status = status;
    let set_status = |s: &mut u32, b: u8| *s = *s & !0xff | b as u32;
    write_u32(ct + contract::RELEASE_FEE, 0);
    write_u8(ct + contract::MANAGER_JOB_RC, 0);
    let mut cc: usize = 0; // the contract with the current club
    let mut grievances: u32 = 0;
    let mut forced_kind: u8 = 0; // the original's [esp+0x3b]: set only by the new-club branch that never runs
    let cur = lookup();
    if club == 0 {
        club = cur;
        if cur != 0 {
            let mut is_a: u32 = 0;
            let linked = exe::club_get_linked_b_team(cur, &mut is_a, 1);
            if linked != 0 && is_a == 0 {
                club = linked;
            }
        }
    }
    if cur != 0 {
        cc = exe::contract_manager_get_contract_for_club(cm(), person, cur);
    }
    write_u8(ct + contract::LEAVING_ON_BOSMAN, 0);
    write_u32(ct + contract::TRANSFER_ARRANGED_FOR, u32::MAX);
    write_u16(ct + contract::CLUB_APPEARANCES, 0);
    write_u8(ct + contract::CONTRACT_TYPE, read_u8(ct + contract::CONTRACT_TYPE) & 0x40 | 2);
    write_u32(ct, read_u32(person));
    write_u8(ct + contract::SQUAD_NUMBER, 0);
    let frac = (read_i32(person) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // the fixed draw by id
    let fr = frac as f64;
    let age = read_i8(person + staff::AGE);
    let sheet = read_usize(person + staff::PLAYER);
    let np = read_usize(person + staff::NON_PLAYER);
    let amb = read_i8(person + staff::AMBITION) as i32;
    let loy = read_i8(person + staff::LOYALTY) as i32;
    let pr = read_i8(person + staff::PRESSURE) as i32;
    // the squad status
    if status as u8 == 0xff && sheet != 0 {
        if cc != 0 {
            let mut s = exe::contract_manager_get_deserved_squad_status(cm(), person, cc);
            grievances = read_u32(cc + contract::GRIEVANCES);
            if grievances & 0x10 != 0 && s != 1 {
                s = 2;
            }
            set_status(&mut status, s);
        } else {
            let s = exe::shortlist_manager_get_squad_status_for_club(shortlist(), person, club, 0usize);
            set_status(&mut status, s);
        }
    } else if cc != 0 {
        grievances = read_u32(cc + contract::GRIEVANCES);
    }
    write_u8(ct + contract::SQUAD_STATUS, read_u8(ct + contract::SQUAD_STATUS) & 0xf | (status as u8) << 4);
    // the level
    let mut level: u8;
    if club != 0 {
        let q = (rep_of(club) as i32 / 500) as u8;
        level = q.wrapping_add(1);
        if read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
            level = q;
            if (level as i8) < 1 {
                level = 1;
            } else if (level as i8) > 4 {
                level = 4;
            }
        }
        write_u32(ct + contract::CLUB, read_u32(club));
    } else {
        if np == 0 && sheet != 0 {
            level = (read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32 / 500) as u8;
        } else {
            level = ((rep_of(read_usize(person + staff::NATIONAL_JOB)) as i32 / 500) as u8).wrapping_add(1);
            if (level as i8) < 3 {
                level = 3;
            }
        }
        write_u32(ct + contract::CLUB, u32::MAX);
    }
    let int_apps = read_u8(person + staff::INT_APPS);
    if int_apps != 0 {
        let nation = read_usize(person + staff::NATION);
        let mut add: u8 = 1;
        if nation != 0 {
            add = ftol(read_i8(nation + nation::LEAGUE_STANDARD) as f64 * f(rdata::F32_0_25) * (int_apps as f64 * f(F32_0_125))) as u8;
            if (add as i8) > 4 {
                add = 4;
            } else if add == 0 {
                add = 1;
            }
        }
        level = level.wrapping_add(add);
    }
    let lv = level as i8 as i32;
    // the reputation of the clauses and the length
    let mut rep48: i16 = 0; // the original leaves it unset with a sheet, a staff job and no non-player sheet
    if sheet != 0 && is_player_job(job) {
        rep48 = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
    } else if np != 0 {
        rep48 = read_i16(np + non_player::CURRENT_REPUTATION);
    }
    if rep48 == 0 {
        rep48 = 1;
    }
    // the signing fee
    let fee = ct + contract::UNKNOWN;
    let scale_fee = |k: f64| write_i32(fee, ftol(read_i32(fee) as f64 * k));
    if sheet != 0 && is_player_job(job) {
        let v = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
        write_i32(fee, v.wrapping_mul(2) / 10); // imul 0x66666667; sar 2: / 10
        let cr = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
        if read_usize(person + staff::CLUB_JOB) == 0 && cr < 2750 && rep_of(club) < 2000 {
            scale_fee(d(rdata::F64_0_25));
        } else if cr < 4250 {
            scale_fee(d(rdata::F64_0_5));
        } else if cr < 4750 {
            scale_fee(d(rdata::F64_0_75));
        }
        if club == cur {
            if age < 19 && cr < 3750 {
                write_i32(fee, 0);
            } else if age < 21 && cr < 4250 {
                let k = if pr >= 15 { rdata::F64_0_5 } else if cr < 2750 { rdata::F64_0_1 } else { rdata::F64_0_25 };
                scale_fee(d(k));
            } else if club != 0 && cr < 3500 && rep_of(club) < 3500 {
                scale_fee(d(rdata::F64_0_5));
            }
            let k = match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                1 => rdata::F64_0_9,
                2 | 3 => rdata::F64_0_75,
                4 => rdata::F64_0_5,
                _ => rdata::F64_0_25,
            };
            scale_fee(d(k));
        } else if cur != 0 {
            if rep_of(cur) < 4250 {
                scale_fee(d(rdata::F64_0_5));
            }
            let n = read_usize(cur + club::NATION);
            if n != 0 && read_i8(n + nation::LEAGUE_STANDARD) < 10 {
                scale_fee(d(rdata::F64_0_5));
            }
        }
    } else {
        write_i32(fee, 0);
    }
    // no club job
    let mut staff_path = false;
    if read_u8(person + staff::JOB_FOR_CLUB) == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(TEXT_BUFFER, NO_JOB_FORMAT, read_i32(person));
        if sheet != 0 {
            write_u8(person + staff::JOB_FOR_CLUB, 0xb);
        } else {
            write_u8(person + staff::JOB_FOR_CLUB, 9);
            staff_path = true;
        }
    }
    if !staff_path && (sheet == 0 || !is_player_job(job)) {
        staff_path = true;
    }
    let mut v: f32; // the value ([esp+0x10])
    let mut w: f32; // the wage asked ([esp+0x24])
    if staff_path {
        covered!("mkd_3 staff contract");
        w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, job, 0) as f32;
        let will_v = exe::staff_interest_in_job_at_club(shortlist(), person, club, 0, job, reasons as u32) as f32;
        if will != 0 {
            write_f32(will, will_v);
        }
        let wv = will_v as f64;
        let mut q = d(rdata::F64_3_5) / if wv < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { wv };
        if wv < d(rdata::F64_1_75) {
            w = (w as f64 + f(rdata::F32_2500)) as f32;
        }
        if q < d(rdata::F64_0_9) {
            q = f(rdata::F32_0_8999999762);
        }
        v = (q * w as f64) as f32;
        let vv = v as f64;
        let rep = rep_of(club) as i32;
        let unavailable = match job as u8 {
            6 => ((rep * 2 + 1200) as f64) < vv && vv > f(F32_4100),
            8 => ((rep + 500) as f64) < vv && vv > f(F32_3200),
            9 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_2100),
            10 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_1500),
            _ => false,
        };
        if unavailable || vv > f(F32_250000) {
            v = 250000.0;
        }
    } else {
        covered!("mkd_3 player contract");
        let cr = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
        let world = read_i16(sheet + player_sheet::WORLD_REPUTATION);
        let mark = || read_u8(staff_state_entry(Staff(person)) + staff_state::STATUS_FLAGS) & 0xf;
        if cur != 0 && cur == club {
            // a renewal: flag 1 (at most the club's maximum wage) when it is cheap
            let days = days_to(cc);
            let cheap = (grievances == 0 && mark() == 0 && cr < 7250 && days < loy * 12 + 182 && read_u8(cc + contract::LEAVING_ON_BOSMAN) == 0)
                || (pr < 6
                    && age < 24
                    && exe::transfer_manager_get_future_transfer(tm(), person, 0) == 0
                    && exe::staff_is_transfer_listed_or_requested(person, club) == 0
                    && cr < 3250
                    && cr < rep_of(club).wrapping_add(250)
                    && days_to(cc) < 200 - pr)
                || (grievances != 0
                    && grievances & 0x161a8 == 0
                    && days < read_i32(person) % 35 + 20
                    && mark() == 0
                    && (cr as i32 + amb * 20) < (loy + 170) * 25)
                || ((loy > 18 || (cr < 5250 && mark() == 0)) && world < 3500);
            w = exe::staff_contract_default_staff_contract(ct, person, club, cheap as u32, status, 0xb, 0) as f32;
        } else if (100 - amb - loy).wrapping_mul(125000) < 0 {
            set_status(&mut status, 1);
            let kind = if pr > 5 || world > 6250 { 1 } else { 2 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, 1, 0xb, kind) as f32;
            forced_kind = 1;
        } else if (80 - amb - loy).wrapping_mul(125000) < 0 {
            if status as u8 != 1 {
                set_status(&mut status, 2);
            }
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, 2) as f32;
        } else {
            let kind = if cr > 6750 { 3 } else { 0 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, kind) as f32;
        }
        // above the current wage
        if cc != 0 {
            let cw = read_i32(cc + contract::WAGE);
            if (cw as f64) < w as f64 {
                let mut cap = if cw.wrapping_mul(2) > cw.wrapping_add(8000) { cw.wrapping_mul(2) } else { cw.wrapping_add(8000) };
                if world > 6750 && w as f64 > f(rdata::F32_10000) && (cap as f64) < w as f64 {
                    let n = if club != 0 { read_usize(club + club::NATION) } else { 0 };
                    let key = if n != 0 { read_i32(n) } else { 0 };
                    let big: i32 = if n != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) { 75000 } else { 50000 };
                    let strong = if cur == club {
                        true
                    } else if club != 0 {
                        let div = read_usize(club + club::DIVISION);
                        let n = read_usize(club + club::NATION);
                        let r = if div != 0 {
                            ftol((read_i16(div + competition::REPUTATION) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else if n != 0 {
                            ftol((read_i8(n + nation::LEAGUE_STANDARD) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else {
                            rep_of(club)
                        };
                        r > 5750
                    } else {
                        false
                    };
                    if strong {
                        cap = if cap >= big { ftol((w as f64 - big as f64) * d(rdata::F64_0_25) + big as f64) } else { big };
                    }
                }
                if w as f64 > cap as f64 {
                    w = cap as f32;
                }
            }
        }
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let highest = if fin != 0 { read_i32(fin + finance::HIGHEST_WAGE) } else { 0 };
        // the drive
        let t = (world as i32 + (amb - loy) * 25 + pr * 10) as i16;
        let s = status as u8 as i8;
        let happy = |c: usize| c != 0 && read_i8(c + contract::HAPPINESS) > 50;
        let wants_more = (s == 1 && age > 18 && happy(cc))
            || (world > 4250 && (t > rep_of(club) || (t > 6750 && int_apps != 0 && s < 3 && s != 0)) && s != 7)
            || (cur == club
                && happy(cc)
                && exe::contract_manager_get_deserved_squad_status(cm(), person, cc) == 1
                && loy < 15
                && amb > 10
                && pr > 10
                && t > 6750
                && world > 4250);
        if wants_more
            && (cc == 0 || (days_to(cc) <= pr * 10 + 525 && (read_i8(cc + contract::MATCHES_LEFT_OUT) >= 2 || world >= 5750)))
            && club_is_in_major_nation(club) != 0
            && age > 18
            && age < 32
        {
            covered!("mkd_3 star wage");
            let (mut top, mut reference) = (highest, highest);
            if t > 7250 && read_usize(person + staff::PLAYER) != 0 && read_i8(sheet + player_sheet::GOALKEEPER) != 20 && loy != 20 {
                let x = t as i32 as f64 * d(rdata::F64_0_0001);
                let key = read_i32(read_usize(club + club::NATION));
                let k = if key == read_i32(KEY_NATION_ENGLAND) {
                    Some(F32_85000)
                } else if key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY) {
                    Some(F32_105000)
                } else {
                    None
                };
                if let Some(k) = k {
                    let e = ftol(x * f(k));
                    if highest < e {
                        top = e;
                        reference = e;
                    }
                }
            }
            if read_usize(person + staff::PLAYER) != 0 && read_i8(sheet + player_sheet::GOALKEEPER) == 20 && top > 60000 {
                reference = 60000;
            }
            let mut x = (amb - read_i8(person + staff::PROFESSIONALISM) as i32 - loy + 4) as f64 * d(rdata::F64_0_0075);
            if x < d(F64_NEG_0_25) {
                x = f(F32_NEG_0_25);
            } else if x > d(rdata::F64_0_15) {
                x = f(F32_0_1500000060);
            }
            if age > 29 {
                if t < 5250 {
                    x -= d(rdata::F64_0_1);
                } else if pr < 15 {
                    x -= d(rdata::F64_0_05);
                }
            } else if t < 4250 && pr < 15 {
                x -= d(rdata::F64_0_05);
            }
            if (cr as i32 + amb * 25) < rep_of(club).wrapping_sub(1250) as i32 {
                x -= d(rdata::F64_0_05);
            }
            x += if read_i8(sheet + player_sheet::GOALKEEPER) == 20 {
                d(rdata::F64_0_45)
            } else if read_i8(sheet + player_sheet::DEFENDER) == 20 {
                d(rdata::F64_0_7)
            } else if read_i8(sheet + player_sheet::ATTACKER) == 20 && t > 7250 && loy < 6 {
                d(rdata::F64_1)
            } else if t > 7250 {
                d(rdata::F64_0_95)
            } else {
                d(rdata::F64_0_9)
            };
            let y = x * reference as f64;
            if (w as f64) < y {
                w = y as f32;
            }
        }
        // the will
        let will_v = exe::shortlist_manager_staff_interest_in_joining_club(shortlist(), person, club, 0, 0, status, reasons as u32);
        if will != 0 {
            write_f32(will, will_v as f32);
        }
        let mut q = (d(rdata::F64_3_5) / if will_v < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { will_v }) as f32;
        let restless = if cc != 0 {
            if grievances & 2 != 0 {
                if q as f64 > d(rdata::F64_1_25) {
                    q = 1.25;
                }
                false
            } else {
                grievances != 0
                    && will_v < d(rdata::F64_1_75)
                    && (pr > 6
                        || age > 23
                        || exe::transfer_manager_get_future_transfer(tm(), person, 0) != 0
                        || exe::staff_is_transfer_listed_or_requested(person, club) != 0
                        || read_i16(read_usize(person + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 3249
                        || read_i16(read_usize(person + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > rep_of(club).wrapping_add(250)
                        || days_to(cc) > 200 - pr)
            }
        } else {
            will_v < d(rdata::F64_1_75)
        };
        if restless {
            w = (w as f64 + f(rdata::F32_5000)) as f32; // mkd3: the restless ask 5000 more
        }
        let cap_q = |q: &mut f32, limit: usize, to: u32| {
            if *q as f64 > d(limit) {
                *q = f32::from_bits(to);
            }
        };
        if (q as f64) < d(rdata::F64_0_9) {
            q = f32::from_bits(0x3f666666);
        } else if cur == club {
            if cc == 0 {
                cap_q(&mut q, rdata::F64_3, 0x40400000);
            } else if grievances != 0 {
                if grievances & 2 != 0 || (age < 21 && cr < 4250 && grievances == 0x10) {
                    cap_q(&mut q, rdata::F64_1_2, 0x3f99999a);
                } else if grievances & 1 != 0 {
                    cap_q(&mut q, rdata::F64_1_75, 0x3fe00000);
                } else if grievances & 0x8400 != 0 {
                    cap_q(&mut q, rdata::F64_1_5, 0x3fc00000);
                } else {
                    cap_q(&mut q, rdata::F64_4, 0x40800000);
                }
            } else {
                covered!("mkd_3 renewal demand");
                let loyal = || loy as i8 > ((read_i32(person) % 5) as u8).wrapping_add(10) as i8 && days_to(cc) < 30;
                if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else if age < 19 || age > 34 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        3..=5 => cap_q(&mut q, rdata::F64_1_25, 0x3fa00000),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if age < 21 || age > 31 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_5, 0x3fc00000),
                        3..=5 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else {
                    cap_q(&mut q, rdata::F64_2, 0x40000000);
                }
                if q as f64 > d(rdata::F64_1_1)
                    && age < 20
                    && (read_i16(sheet + player_sheet::CURRENT_ABILITY) as i32) < 125 - pr - amb
                    && cr < 4250
                    && days_to(cc) < 120 - pr - amb
                {
                    q = f32::from_bits(0x3f8ccccd);
                }
            }
        }
        v = (q as f64 * w as f64) as f32; // mkd3: the value asked
        w = v;
        // at least the current wage scaled by the clubs' reputations
        if cc != 0 && cc != ct {
            let cw = read_i32(cc + contract::WAGE) as f64;
            if cw > v as f64 {
                let mut r = f(rdata::F32_1);
                let (a, b) = (rep_of(club), rep_of(cur));
                if a < b {
                    let x = a as i32 as f64 / if b > 100 { b as i32 } else { 100 } as f64;
                    r = if x > d(rdata::F64_0_5) { x } else { d(rdata::F64_0_5) };
                }
                if cw * r > v as f64 {
                    v = (cw * r) as f32;
                }
            }
        }
        if grievances & 0x1000000 != 0 && highest as f64 * d(rdata::F64_0_95) > v as f64 {
            v = (read_i32(fin + finance::HIGHEST_WAGE) as f64 * f(rdata::F32_0_9499999881)) as f32;
        }
        // the "not available" markers
        let vv = v as f64;
        if vv > f(F32_200000) || (age > 34 && vv > f(F32_35000)) || (age > 31 && vv > f(F32_75000)) || (read_i8(sheet + player_sheet::GOALKEEPER) == 20 && vv > f(F32_105000)) {
            v = 250000.0;
        }
        let vv = v as f64;
        let mut done = true;
        match s {
            7 if vv > f(rdata::F32_5000) => v = 250000.0,
            6 if vv > f(F32_8250) => v = 250000.0,
            5 if vv > f(F32_17500) => v = 250000.0,
            4 if vv > f(F32_35000) => v = 250000.0,
            3 if vv > f(F32_57500) && world < 6750 => {
                let mut out = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
                if !out {
                    let sh = read_usize(person + staff::PLAYER);
                    out = sh == 0 || {
                        let r = read_i16(sh + player_sheet::CURRENT_REPUTATION);
                        let limit = if read_i8(sh + player_sheet::ATTACKER) == 20 || read_i8(sh + player_sheet::ATTACKING_MIDFIELDER) == 20 {
                            if r > 7250 { F32_125000 } else { F32_105000 }
                        } else if r > 7250 {
                            F32_105000
                        } else {
                            F32_85000
                        };
                        r <= 6250 || vv > f(limit)
                    };
                }
                if out {
                    v = 250000.0;
                }
            }
            2 if vv > f(F32_125000) && world < 7250 => v = 250000.0,
            _ => done = false,
        }
        if !done {
            if vv > f(F32_250000) {
                v = 250000.0;
            } else if cc != 0 {
                let cw = read_i32(cc + contract::WAGE);
                if cw as f64 > w as f64 && cw != 0 && cur != club {
                    // a dearer contract at another club: blend toward its wage
                    covered!("mkd_3 dearer current contract");
                    let mut a = if read_u32(cc + contract::GRIEVANCES) & 0x100 != 0 { 10.0 } else { 0.0 };
                    if exe::staff_is_transfer_listed_or_requested(person, cur) != 0 {
                        a += d(rdata::F64_10);
                    }
                    if read_i32(cc + contract::WAGE) > 40000 {
                        a += (read_i32(cc + contract::WAGE) - 40000) as f64 * d(rdata::F64_0_00025);
                    }
                    if age > 33 {
                        let k = (38 - age as i32) as f64;
                        a += k + k;
                    }
                    let h = read_i8(cc + contract::HAPPINESS);
                    if h < 0 {
                        a -= h as f64 * d(rdata::F64_0_2);
                    }
                    if read_u32(cc + contract::GRIEVANCES) != 0 {
                        a -= read_i8(cc + contract::MORALE) as f64 * d(rdata::F64_0_2);
                    }
                    if d(rdata::F64_0) > a {
                        a = 0.0;
                    } else if d(rdata::F64_20) < a {
                        a = 20.0;
                    }
                    let half = w as f64 * d(rdata::F64_0_5);
                    let max_wage = || exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), person, status) as f64;
                    let m = if max_wage() < half { half } else { max_wage() };
                    let x = if (w as f64) < m {
                        w as f64
                    } else if max_wage() < half {
                        half
                    } else {
                        max_wage()
                    };
                    v = (((d(rdata::F64_20) - a) * read_i32(cc + contract::WAGE) as f64 + x * a) * d(rdata::F64_0_05)) as f32;
                }
            }
        }
    }
    // the release clauses
    let clause = |o: usize| ct + o;
    if read_u8(person + staff::JOB_FOR_CLUB) == 0xb && read_usize(person + staff::PLAYER) != 0 {
        let id = read_i32(person);
        let det = read_i8(person + staff::DETERMINATION) as i32;
        let prof = read_i8(person + staff::PROFESSIONALISM) as i32;
        if (age > 32 && rep48 > 3250 && id % 3 == 0 && age as i32 + prof + det + amb + id % 10 - 28 > 45) || read_usize(person + staff::NON_PLAYER) != 0 {
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 1);
        }
        write_u32(ct + contract::RELEASE_FEE, 0);
        // the club's league answers the virtual `slot` (promotion or relegation)
        let league_has = |slot: usize| {
            if club == 0 {
                return false;
            }
            let div = read_usize(club + club::DIVISION);
            if div == 0 {
                return false;
            }
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(div) as usize).wrapping_mul(4)));
            if comp == 0 || read_u8(comp + league::COMP_TYPE) != 2 {
                return false;
            }
            let m: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + slot));
            m(comp) != 0
        };
        let value_plus = |k: f64| {
            let v1 = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
            v1.wrapping_add(ftol(v1 as f64 * fr * k))
        };
        let extra = |val: i32| {
            let y = val as f64 * d(rdata::F64_0_95) + d(F64_150000);
            let z = val.wrapping_mul(2) as f64;
            let e = ftol(if y > z { y } else { z });
            if e > 2500000 {
                ftol(e as f64 * d(F64_0_21) - d(F64_2500000) + d(F64_2500000))
            } else {
                e
            }
        };
        let dropped = |o: usize| cc != 0 && read_u8(cc + o) != 0 && lookup() == club;
        let q50 = rep48 as i32 / 50;
        // non-promotion
        if rep48 > 2750 && ((lv * 13) as f64) < q50 as f64 + fr * f(rdata::F32_20) - (loy * 2) as f64 + (amb * 2) as f64 && league_has(league_vtable::HAS_AUTOMATIC_PROMOTION) {
            let mut val = value_plus(f(rdata::F32_0_25));
            if cc != 0 && read_u8(cc + contract::GRIEVANCES) & 1 == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::NON_PROMOTION_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, if val < 150000 { 150000 } else { val });
        } else {
            if dropped(contract::NON_PROMOTION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
        }
        // relegation
        if rep48 > 2250 && ((lv * 10) as f64) < (q50 + (amb - loy) * 3) as f64 + fr * f(rdata::F32_20) - age as f64 && league_has(league_vtable::HAS_RELEGATION) {
            let mut val = value_plus(f(F32_NEG_0_5));
            if val < 50000 {
                val = 50000;
            }
            if amb > 10 {
                val = ftol(val as f64 * d(if loy < 5 { rdata::F64_0_5 } else { rdata::F64_0_75 }));
            } else if loy < 15 {
                val = ftol(val as f64 * d(rdata::F64_0_85));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, val);
        } else {
            if dropped(contract::RELEGATION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::RELEGATION_RC), 0);
        }
        // minimum fee
        let min_fee = |val: i32| {
            write_u32(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 0);
            write_u8(clause(contract::MINIMUM_FEE_RC), 1);
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 0);
            write_i32(ct + contract::RELEASE_FEE, if val < 150000 { 150000 } else { val });
        };
        if club == cur && grievances & 1 != 0 {
            min_fee(value_plus(f(rdata::F32_0_25)));
        } else {
            let mut set = false;
            if rep48 > 1250 && !(lookup() != 0 && lv >= 18) && ((lv * 10) as f64) < (amb * 4 - loy * 3 + q50 - age as i32) as f64 + fr * f(rdata::F32_10) {
                let mut val = value_plus(f(rdata::F32_0_5));
                if grievances == 0 {
                    val = val.wrapping_add(extra(val));
                }
                min_fee(val);
                set = true;
            }
            if !set && age < 24 {
                let ab = ftol((read_i16(sheet + player_sheet::CURRENT_ABILITY) as i32 + read_i16(sheet + player_sheet::POTENTIAL_ABILITY) as i32) as f64 * d(rdata::F64_0_5)) as i16 as i32;
                if ab - (loy * 3) as i16 as i32 + (amb * 2) as i16 as i32 > lv * 15 {
                    let mut val = value_plus(f(rdata::F32_0_5));
                    if grievances == 0 {
                        val = val.wrapping_add(extra(val));
                    }
                    min_fee(val);
                    set = true;
                }
            }
            if !set {
                if dropped(contract::MINIMUM_FEE_RC) {
                    v = (v as f64 * d(rdata::F64_1_15)) as f32;
                }
                write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            }
        }
        // not playing
        if rep48 > 6500 && ((lv * 14) as f64) < (q50 - loy * 4) as f64 - fr * f(rdata::F32_20) {
            let mut val = value_plus(f(rdata::F32_0_5));
            if grievances == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(NON_PLAYING_RC), 1);
            write_i32(ct + contract::RELEASE_FEE, if val < 1500000 { 1500000 } else { val });
        } else {
            write_u8(clause(NON_PLAYING_RC), 0);
        }
    } else {
        for o in contract::NON_PROMOTION_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
    }
    // the dates and the length
    if club != 0 {
        let mut tmp = [0u8; 8];
        let tp = tmp.as_mut_ptr() as usize;
        if sheet != 0 && cur != club {
            if offer == -1 {
                find_next_window_open_date(tm(), club, ct + contract::DATE_STARTED);
            } else {
                let slot = exe::transfer_manager_get_offer_slot(tm(), offer);
                // defect of the original: `&&` for `||`; without a slot it reads the address 0 first
                if slot == 0 && core::ptr::read_volatile(core::hint::black_box(slot) as *const i32) == 0 {
                    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
                    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
                    exe::crt_strtok(SOURCE_PATH, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
                    let ver = version(name.as_mut_ptr(), ext.as_mut_ptr(), 0xc4e);
                    sprintf(0xae24d0, 0x9870e8, ver, name.as_ptr(), ext.as_ptr(), 0xc4e);
                    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
                    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
                    exe::date_operator_3d(ct + contract::DATE_STARTED, tp, G_CURRENT_DATE);
                } else {
                    exe::transfer_manager_find_next_allowed_transfer_date(tm(), read_usize(slot), (ct + contract::DATE_STARTED) as *mut [u32; 2]);
                }
            }
        }
        exe::date_operator_3d(ct + contract::DATE_STARTED, tp, G_CURRENT_DATE);
        let month = exe::contract_manager_season_end_month(cm(), club);
        let day = exe::utils_random(0x1c) + 1;
        let mut dmy = [0u16; 4];
        let day_of_year = read_u16(exe::date_date_549f70(&mut dmy, day as u32, month as u32, 0x7ce, u32::MAX));
        let k = exe::key_nation_index_of_club(club) as i8 as isize;
        let mut year = read_u16(G_CURRENT_DATE_YEAR);
        if read_u32((G_LEAGUE_TABLE as isize + 72 * k) as usize + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
            year = year.wrapping_add(1);
        }
        let sh = read_usize(person + staff::PLAYER);
        let mut l: f64 = if age < 19 {
            f(rdata::F32_3)
        } else if age < 29 {
            f(rdata::F32_4)
        } else if age < 34 {
            if sh != 0 && read_i8(sh + player_sheet::GOALKEEPER) > 15 {
                f(rdata::F32_3)
            } else if sh != 0 && read_i8(sh + player_sheet::DEFENDER) > 15 {
                f(rdata::F32_2)
            } else {
                f(rdata::F32_1_5)
            }
        } else if np != 0 && sh == 0 && read_u8(person + staff::JOB_FOR_CLUB) != 0xb {
            f(rdata::F32_4)
        } else {
            f(rdata::F32_1)
        };
        if read_i8(clause(contract::RELEGATION_RC)) > 0 || read_i8(clause(contract::NON_PROMOTION_RC)) > 0 {
            l *= d(rdata::F64_0_75);
        }
        if read_i8(clause(contract::MINIMUM_FEE_RC)) > 0 {
            l = f(rdata::F32_1);
        } else {
            let l500 = lv * 500;
            l *= l500 as f64 / if rep48 as i32 > l500 { rep48 as i32 } else { l500 } as f64;
            if age < 32 {
                l += (loy as f64 * d(rdata::F64_0_05) + d(rdata::F64_0_5)) * fr;
            }
        }
        if sheet != 0 {
            match status as u8 as i8 {
                1 => l *= d(rdata::F64_1_25),
                3 | 5 => {
                    l *= d(rdata::F64_0_75);
                    if age < 24 && lv > 10 && l < d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                4 => l *= d(rdata::F64_0_5),
                6 => {
                    l *= d(rdata::F64_0_5);
                    if l > d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                7 => l = f(rdata::F32_1),
                _ => {}
            }
            let r = read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32;
            if r > 2250 && r > lv * 675 {
                if r > 3250 && r > lv * 875 {
                    if age > 20 && age < 31 {
                        if loy < 15 {
                            l = f(rdata::F32_1);
                        } else {
                            l -= d(rdata::F64_2);
                        }
                    }
                } else if age <= 20 {
                    l -= d(rdata::F64_1);
                } else if age >= 27 {
                    if age > 29 && age < 32 && l < d(rdata::F64_2) {
                        l += d(rdata::F64_1);
                    }
                } else if loy < 5 {
                    l = f(rdata::F32_1);
                } else if l > f(rdata::F32_3) {
                    l -= d(rdata::F64_1);
                }
            }
        }
        if forced_kind != 0 {
            l = if l >= d(rdata::F64_3) {
                l + d(rdata::F64_1)
            } else if age < 27 {
                f(rdata::F32_5)
            } else if age < 30 {
                f(rdata::F32_4)
            } else {
                f(rdata::F32_3)
            };
        }
        let n = read_usize(club + club::NATION);
        if n != 0 && (read_i32(n) == read_i32(KEY_NATION_FINLAND) || read_i32(n) == read_i32(KEY_NATION_TURKEY)) {
            l *= d(rdata::F64_0_5);
            if l > d(rdata::F64_3) {
                l = f(rdata::F32_3);
            }
        }
        let vv = v as f64;
        let most = if vv < f(rdata::F32_500) {
            Some(rdata::F64_2)
        } else if vv < f(rdata::F32_1000) {
            Some(rdata::F64_3)
        } else if vv < f(rdata::F32_2000) {
            Some(rdata::F64_4)
        } else {
            None
        };
        if let Some(m) = most {
            if l >= d(m) {
                l = d(m);
            }
        }
        let max_years = read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32;
        if l > (max_years - 1) as f64 {
            l = max_years as f64 - f(rdata::F32_1);
        } else if l < d(rdata::F64_1) {
            l = f(rdata::F32_1);
        }
        let kind_bits = |or: u8, and: u8| write_u8(ct + contract::CONTRACT_TYPE, read_u8(ct + contract::CONTRACT_TYPE) & and | or);
        let no_job_low = read_usize(person + staff::CLUB_JOB) == 0 && (level as i8) < 2;
        let mut add_years = true;
        if rep48 > (level as i8 as i16).wrapping_mul(500).wrapping_add(1250) {
            if no_job_low && ftol(fr * d(rdata::F64_4)) as u8 == 0 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(2, 0xc2);
            }
        } else if no_job_low && ftol(fr * d(rdata::F64_5)) as u8 == 0 {
            if age > 20 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(1, 0xc1);
                add_years = false;
            }
        } else {
            kind_bits(2, 0xc2);
        }
        if add_years {
            year = year.wrapping_add(ftol(l + d(rdata::F64_1)) as u16); // mkd3: the years of the contract
        }
        let mut end = [0u8; 8];
        let ep = end.as_mut_ptr() as usize;
        exe::new_date(ep, day_of_year as u32, year as u32);
        // a renewal never ends before the person's date with the club
        let pd = person + staff::DATE_EXPIRES_CLUB;
        let (py, ey) = (read_u16(pd + 2), read_u16(ep + 2));
        if (py > ey || (py == ey && read_i16(pd) > read_i16(ep))) && lookup() == club {
            exe::date_operator_3d(ep, tp, pd);
        }
        exe::date_operator_3d(ct + contract::CONTRACT_EXPIRES, tp, ep);
    }
    // the bonuses
    let vv = v as f64;
    let bonus = || ftol(vv * fr * f(F32_0_0250000004) + vv * f(F32_0_0250000004) + f(rdata::F32_25));
    let at = |o: usize| read_i8(sheet + o);
    if sheet != 0 {
        write_i32(ct + contract::GOAL_BONUS, if at(player_sheet::ATTACKER) >= 15 { bonus() } else { -1 });
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        write_i32(ct + contract::ASSIST_BONUS, if outfield { bonus() } else { -1 });
        write_i32(ct + contract::CLEAN_SHEET_BONUS, if at(player_sheet::GOALKEEPER) == 20 { bonus() } else { -1 });
    }
    let kind = read_u8(ct + contract::CONTRACT_TYPE);
    if vv < f(rdata::F32_125) && club != 0 && read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
        write_u8(ct + contract::CONTRACT_TYPE, kind | 0x80);
    } else if vv > f(F32_1250) && kind & 0x80 != 0 {
        write_u8(ct + contract::CONTRACT_TYPE, kind & 0x7f);
    }
    let wage = ftol(vv);
    write_i32(ct + contract::WAGE, wage);
    // the return: the wage and the bonuses and clauses, these at most a tenth of the value
    let kind = read_u8(ct + contract::CONTRACT_TYPE);
    let mut paid = wage as f64;
    if ((kind << 2) as i8 >> 2) == 1 {
        paid *= d(rdata::F64_0_5);
    }
    if kind & 0x80 != 0 {
        paid *= d(rdata::F64_1_05);
    }
    let mut extras = f(rdata::F32_0);
    if sheet != 0 {
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        let keeper = at(player_sheet::GOALKEEPER) == 20;
        for i in 0..3 {
            let b = read_i32(ct + contract::GOAL_BONUS + 4 * i);
            if b > -1 {
                let x = (b / 10) as f64; // imul 0x66666667; sar 2: / 10
                let plus = if i < 2 { outfield } else { keeper };
                if plus {
                    extras += x;
                } else if i == 2 || keeper {
                    extras -= x;
                }
            }
        }
    }
    for c in 0..5 {
        if read_u8(clause(contract::NON_PROMOTION_RC + c)) != 0 {
            let fee = read_i32(ct + contract::RELEASE_FEE);
            let k = match c {
                1 if fee < 500000 => rdata::F32_0_1000000015,
                1 if fee < 2500000 => rdata::F32_0_05000000075_AT_96D61C,
                1 => F32_0_0133333337,
                2 if fee < 2500000 => rdata::F32_0_1000000015,
                2 => F32_0_0666666701,
                _ if fee < 500000 => F32_0_0666666701,
                _ if fee < 2500000 => F32_0_0399999991,
                _ => F32_0_0199999996,
            };
            extras += vv * f(k);
        }
    }
    if sheet == 0 {
        for o in contract::MINIMUM_FEE_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
        write_i32(ct + contract::GOAL_BONUS, -1);
        write_i32(ct + contract::ASSIST_BONUS, -1);
        write_i32(ct + contract::CLEAN_SHEET_BONUS, -1);
    }
    let tenth = (vv * f(rdata::F32_0_1000000015)) as f32 as f64;
    ftol(if extras > tenth { tenth } else { extras } + paid)
}

/// Comparer for `staff_contract_set_to_desired_contract`: the original and the copy side by side from the same state of
/// the contract (0x50 bytes), the last contract looked up (`G_CONTRACT_LOOKUP_RESULT`), the game's generator, the text
/// buffer (the "no job for club" message), the person's `staff::JOB_FOR_CLUB` and the will and reasons the caller
/// passed; the contract, those and the returned value must match (the callers' comparers already run it twice).
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, usize, usize) -> i32>(IDX_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let mut regions = vec![(this, 0x50usize), (G_CONTRACT_LOOKUP_RESULT, 4), (desired_contract_rdata::TEXT_BUFFER, 0x50), (person + staff::JOB_FOR_CLUB, 1)];
    regions.extend(crate::rng::STATE_REGIONS);
    if will != 0 {
        regions.push((will, 4));
    }
    if reasons != 0 {
        regions.push((reasons, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person, club, offer, status, job, reasons, will),
        || crate::rng::without_counting(|| staff_contract_set_to_desired_contract(this, person, club, offer, status, job, reasons, will)));
    SCOREBOARD.record(same && r == n, || format!("staff_contract_set_to_desired_contract(person {:#x}, club {:x}, offer {}, status {:#x}, job {:#x}) original={} rust={} {:x?} x {:x?}",
        read_u32(person), club, offer, status, job, r, n, mo, mr));
    r
}

// ---------- batch 288: the person chooses among his contract offers (contract_manager.cpp) ----------

/// The routines and the `.rdata` double the decision on contract offers uses that `exe_fns.rs` / `rdata_consts.rs` do
/// not name yet (cm0102.exe 3.9.68). The Rust ones are called by address, so they go through their detour.
#[allow(dead_code)]
pub(crate) mod resolve_offers_fns {
    pub(crate) const FN_PLAYER_HAS_PENDING_OFFER: usize = 0x8cd150; // TRANSFER_MANAGER::player_has_pending_offer (thiscall, ret 4)
    pub(crate) const FN_FIND_OFFER_PLAYER_ENTRY: usize = 0x8e82f0; // the person's entry in a transfer offer's list +4 (thiscall, ret 4)
    pub(crate) const FN_HAS_SPARE_NON_PLAYER_RECORD: usize = 0x7a6e20; // PLAYER_REGEN::has_spare_non_player_record (al)
    pub(crate) const FN_TAKE_SPARE_NON_PLAYER_RECORD: usize = 0x7a6e30; // PLAYER_REGEN::take_spare_non_player_record (ret 0xc)
    pub(crate) const FN_REMOVE_TRANSFER_INTEREST: usize = 0x7549b0; // Media::remove_transfer_interest (ret 4)
    pub(crate) const FN_SET_AGREED_CONTRACT_FLAG: usize = 0x8c5410; // TRANSFER_MANAGER (ret 4)
    pub(crate) const FN_HANDLE_PLAYER_REJECTED_TERMS: usize = 0x8c28c0; // TRANSFER_MANAGER (ret 0xc)
    pub(crate) const FN_HANDLE_PLAYER_ACCEPTED_TERMS: usize = 0x8c2180; // TRANSFER_MANAGER (ret 0x14)
    pub(crate) const FN_COMPLETE_ACCEPTED_CONTRACT_OFFER: usize = 0x8c38b0; // TRANSFER_MANAGER (ret 4)
    pub(crate) const FN_POST_OFFER_RESPONSE_NEWS: usize = 0x4d49a0; // contract_manager_post_offer_response_news (ret 8)
    pub(crate) const F64_0_99: usize = 0x968e50; // f64 = 0.99
    pub(crate) const OFFER_SIZE: usize = 0x6f; // a contract offer: the STAFF_CONTRACT (0x50) and contract_offer::STATE..+0x6e

    use super::*;
    #[inline(always)]
    pub(crate) unsafe fn tm_call1(f: usize, a: usize) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(f);
        g(read_usize(G_TRANSFER_MANAGER), a)
    }
    #[inline(always)]
    pub(crate) unsafe fn find_offer_player_entry(offer: usize, person: usize) -> usize {
        let g: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FIND_OFFER_PLAYER_ENTRY);
        g(offer, person)
    }
    #[inline(always)]
    pub(crate) unsafe fn has_spare_non_player_record() -> u8 {
        let g: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_HAS_SPARE_NON_PLAYER_RECORD);
        g(read_usize(G_PLAYER_REGEN)) as u8
    }
    #[inline(always)]
    pub(crate) unsafe fn take_spare_non_player_record(id: usize, nation: usize, second: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TAKE_SPARE_NON_PLAYER_RECORD);
        g(read_usize(G_PLAYER_REGEN), id, nation, second);
    }
    #[inline(always)]
    pub(crate) unsafe fn remove_transfer_interest(person: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_REMOVE_TRANSFER_INTEREST);
        g(read_usize(G_MEDIA), person)
    }
    #[inline(always)]
    pub(crate) unsafe fn handle_player_rejected_terms(person: usize, slot: i32, best: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(FN_HANDLE_PLAYER_REJECTED_TERMS);
        g(read_usize(G_TRANSFER_MANAGER), person, slot, best as u32)
    }
    #[inline(always)]
    pub(crate) unsafe fn handle_player_accepted_terms(person: usize, slot: i32, offer: usize, kind: u32, clubs: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, usize) = core::mem::transmute(FN_HANDLE_PLAYER_ACCEPTED_TERMS);
        g(read_usize(G_TRANSFER_MANAGER), person, slot, offer, kind, clubs)
    }
    #[inline(always)]
    pub(crate) unsafe fn post_offer_response_news(this: usize, offer: usize, flag: u32) {
        let g: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_POST_OFFER_RESPONSE_NEWS);
        g(this, offer, flag)
    }
}

/// What the evaluation of a person's contract offers decided (`choose_contract_offer`).
#[derive(Clone, Copy)]
pub(crate) struct ContractOfferChoice {
    /// the chosen offer (0 none)
    pub(crate) best: usize,
    /// its score (the will × the ratio; the will for a loan or a trial); starts at 2.75
    pub(crate) best_score: f64,
    /// the last offer of his own club that he turned down (0 none)
    pub(crate) rejected_own: usize,
    /// an offer already agreed (state 2) was found: nothing is chosen
    pub(crate) accepted: bool,
    /// how many stale offers were taken out of the list
    pub(crate) removed: u32,
}

/// One less contract offer for the person (byte +8 of the 79-byte staff state record).
unsafe fn resolve_offers_count_down(person: Staff) {
    let c = contract_offer_counter(person.addr());
    write_u8(c, read_u8(c).wrapping_sub(1));
}

/// The first part of `resolve_contract_offers`: walks the person's offers and scores each one, keeping the best. With
/// `lists` (the real run) each evaluated offer goes into the first list (its pointer) and its score into the second,
/// the stale offers leave the contract manager's list and the transfer offer's entry gets the squad status; without
/// (`None`, the comparer's plan) nothing is changed: the removals are only counted and the entries' bytes are put
/// back. `desired` is the local STAFF_CONTRACT the desired contract is written into (reused from offer to offer, as in
/// the original).
pub(crate) unsafe fn choose_contract_offer(this: usize, person: Staff, desired: usize, lists: Option<(Llist, Llist)>) -> ContractOfferChoice {
    use resolve_offers_fns::*;
    // k = (professionalism − ambition − 9 + loyalty) × 1/300 (fild of the int, fmul 0x968e60)
    let numerator = person.professionalism() as i32 - person.ambition() as i32 + person.loyalty() as i32 - 9;
    let k = numerator as f64 * read_f64(rdata::F64_0_003333333333);
    let mut c = ContractOfferChoice { best: 0, best_score: 2.75, rejected_own: 0, accepted: false, removed: 0 };
    let mut own: i32 = numerator; // [esp+0x28]: the original's slot holds the numerator until the first renewal test
    let mut will = 0.0f64; // [esp+0x2c]
    let mut put_back: Vec<(usize, u8)> = Vec::new();
    let list = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    if !list.is_null() {
        list.set_cursor(list.head());
        loop {
            let o = exe::llist_get_next(list);
            if o == 0 {
                break;
            }
            let offer = ContractOffer(o);
            if read_usize(o) != read_usize(person.addr()) || offer.decision_valid() == 0 {
                continue;
            }
            let club = club_at(offer.club_id() as isize);
            if offer.transfer_offer_slot() as i32 == -1 {
                // a plain offer from a club that is not the one of his contract (loans aside) is stale
                let current = current_contract_club(person);
                if current != club && offer.contract_type() & 0x3f != 5 {
                    covered!("mkd_4 stale offer");
                    c.removed += 1;
                    if lists.is_some() {
                        exe::llist_remove(list, o);
                        resolve_offers_count_down(person);
                    }
                    continue;
                }
            }
            match offer.state() {
                0 => {}
                2 => {
                    c.accepted = true;
                    continue;
                }
                _ => continue,
            }
            let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
            let kind = offer.contract_type() & 0x3f;
            let mut score: f64;
            let base: f64; // [esp+0x4c]: the desired contract's value against itself (0 for loans and trials)
            if kind == 5 || kind == 6 {
                // a loan (status 8) or a trial (the squad status the club would give): the will alone
                let status = if kind == 5 { 8 } else { exe::shortlist_manager_get_squad_status_for_club(sl, person, club, 0usize) as u32 };
                let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                let w = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 1, status, 0);
                if w <= c.best_score || w < read_f64(rdata::F64_3_15) {
                    score = 0.0;
                } else {
                    covered!("mkd_4 loan or trial");
                    score = w;
                    c.best_score = w;
                    c.best = o;
                }
                base = 0.0;
            } else {
                let status = read_u8(o + contract::SQUAD_STATUS) >> 4;
                if offer.transfer_offer_slot() as i32 != -1 {
                    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer.transfer_offer_slot() as i32);
                    let e = find_offer_player_entry(read_usize(slot), person.addr());
                    if e != 0 {
                        if lists.is_none() {
                            put_back.push((e + 5, read_u8(e + 5)));
                        }
                        write_u8(e + 5, status);
                    }
                }
                let job = offer.mark();
                exe::staff_contract_set_to_desired_contract(desired, person, club, -1, status as u32, job as u32, 0, 0);
                base = exe::staff_contract_calc_contract_value(desired, person, desired) as f64;
                if base <= read_f64(rdata::F64_0) {
                    score = 0.0;
                } else {
                    // the ratio: the offer against the desired contract, less k
                    let v = exe::staff_contract_calc_contract_value(o, person, desired);
                    score = v as f64 / base - k; // mkd4: the ratio less k
                    let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                    if person.non_player().is_null() || job == 0xb {
                        will = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 0, status as u32, 0);
                    } else {
                        will = exe::staff_interest_in_job_at_club(sl, person, club, 0, job as u32, 0);
                        if exe::database_is_player_job(job as i8) != 0 {
                            let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                            let w = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 0, status as u32, 0);
                            will = (w + will) * read_f64(rdata::F64_0_5);
                        }
                    }
                    // a renewal (an offer from the club of his contract): a ratio of 0.99 lifts the will to 3.15
                    own = 0;
                    if !current_contract_club(person).is_null() {
                        let current = current_contract_club(person);
                        if read_usize(current.addr()) == offer.club_id() {
                            own = 1;
                            if score >= read_f64(F64_0_99) && will <= read_f64(rdata::F64_3_15) {
                                covered!("mkd_4 renewal floor");
                                will = 3.15; // mkd4: the renewal floor
                            }
                        }
                    }
                    if score < read_f64(rdata::F64_0_6) {
                        score = 0.0; // mkd4: the ratio cut
                    }
                }
            }
            // refusals: a staff job without a non-player sheet and no spare record; a work permit with under 90 days
            let job = offer.mark();
            if (exe::database_is_not_plain_player_job(job as i8) != 0 && person.non_player().is_null() && has_spare_non_player_record() == 0)
                || (score > read_f64(rdata::F64_0_5)
                    && exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), person, club, 0) != 0
                    && exe::date_days_since(o + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) < 0x5a)
            {
                covered!("mkd_4 refused");
                score = 0.0;
            }
            if let Some((l1, l2)) = lists {
                let at = o;
                exe::llist_add_tail(l1, &at as *const usize as usize, 4);
                exe::llist_add_tail(l2, &score as *const f64 as usize, 8);
            }
            if base == read_f64(rdata::F64_0) {
                continue;
            }
            // the score: ratio ≥ 0.9, or > 0.7 with a will above 3.5, lifts the will to 3.15; > 0.8 with a will above
            // 3.15 keeps it; otherwise the offer is turned down
            let lift = if score >= read_f64(rdata::F64_0_9) {
                true
            } else if score > read_f64(rdata::F64_0_8) && will > read_f64(rdata::F64_3_15) {
                false
            } else if score > read_f64(rdata::F64_0_7) && will > read_f64(rdata::F64_3_5) {
                true
            } else {
                if own != 0 {
                    c.rejected_own = o;
                }
                continue;
            };
            if lift && will <= read_f64(rdata::F64_3_15) {
                will = read_f64(rdata::F64_3_15);
            }
            let s = will * score;
            if s > c.best_score {
                covered!("mkd_4 best offer");
                c.best_score = s;
                c.best = o;
            } else if own != 0 {
                c.rejected_own = o;
            }
        }
    }
    for &(a, v) in put_back.iter().rev() {
        write_u8(a, v);
    }
    // an agreed offer, or a manager candidate for a vacancy: nothing is chosen
    if c.accepted || (!person.non_player().is_null() && exe::manager_manager_is_vacancy_candidate(read_usize(G_MANAGER_MANAGER), person) != 0) {
        c.best = 0;
    }
    c
}

/// The person decides on his contract offers (the decision date of type 3 fell due): each open offer gets a score and
/// the best one above 2.75 is accepted; the others are turned down.
///
/// k = (`professionalism` − `ambition` − 9 + `loyalty`) × 1/300. With a pending transfer offer
/// (`TRANSFER_MANAGER::player_has_pending_offer`): a new type 3 decision date two days from today
/// (`exe::contract_manager_add_decision_date` at the current slice) and nothing else. Otherwise the offers in
/// `contract_manager_fields::CONTRACT_OFFERS` for the person with `contract_offer::DECISION_VALID` set
/// (`choose_contract_offer`): a plain offer (`contract_offer::TRANSFER_OFFER_SLOT` −1) from a club that is not the one
/// of his contract (stored in `G_CONTRACT_LOOKUP_RESULT`), outside loans, is stale: removed from the list and from the
/// counter (byte +8 of the 79-byte record). State 2 (already agreed) marks an agreed offer; other states are skipped.
/// State 0: a loan (type 5) is worth the will (`staff_interest_in_joining_club`, loan 1, status 8) and a trial (type
/// 6) the will with the squad status the club gives (`get_squad_status_for_club`); taken at once when above the best
/// so far and at least 3.15. Any other: the squad status (high nibble of `contract::SQUAD_STATUS`) goes to the
/// transfer offer's entry for the person; the desired contract at the club (`exe::staff_contract_set_to_desired_contract`
/// with −1, the status and the job `contract_offer::MARK`); ratio = value(offer) / value(desired)
/// (`exe::staff_contract_calc_contract_value`, both against the desired) − k; the will is
/// `staff_interest_in_joining_club` for a player (no non-player sheet, or job 11), otherwise
/// `exe::staff_interest_in_job_at_club`, averaged with the first for a player job; an offer from the club of his
/// contract is a renewal: a ratio of at least 0.99 lifts a will up to 3.15 to 3.15; a ratio below 0.6 is 0. Refused
/// (0): a job other than plain player for someone without a non-player sheet when no spare record is left
/// (`PLAYER_REGEN::has_spare_non_player_record`), or a ratio above 0.5 with a work permit needed
/// (`exe::transfer_manager_player_needs_work_permit`) and the offer's end less than 90 days away. Each one goes into
/// two local lists (pointer, score). Then: ratio ≥ 0.9, or above 0.7 with a will above 3.5, lifts the will to at least
/// 3.15; above 0.8 with a will above 3.15 keeps it; otherwise turned down. Score = will × ratio; the highest above the
/// best so far is chosen; a renewal not chosen is remembered. An agreed offer, or a manager candidate for a vacancy
/// (`exe::manager_manager_is_vacancy_candidate`, with a non-player sheet), chooses nothing.
///
/// Second pass over the evaluated offers: the chosen one becomes state 2 (`Media::remove_transfer_interest`,
/// `TRANSFER_MANAGER::set_agreed_contract_flag`); the others state 1, with the rejected terms
/// (`TRANSFER_MANAGER::handle_player_rejected_terms`, the chosen offer as the third argument) when tied to a transfer
/// offer or the response news (`contract_manager_post_offer_response_news`, flag 1 for the turned-down renewal of a
/// player in job 11 when nothing is chosen). None chosen: a turned-down renewal sets bit 0x80 of
/// `staff_state::STATUS_FLAGS`. Chosen, outside loans and trials, with a score above 10: when random(max(40 − score,
/// 5)) < `loyalty` and the club has a manager (reputation above 4500, or random(4) = 0), the person comes to favour
/// the manager (random(2) ≠ 0, `exe::staff_add_favoured_staff`) or the club (`exe::staff_add_favoured_club`).
///
/// Third pass: with a chosen offer, an offer of the same club without a transfer completes the contract
/// (`TRANSFER_MANAGER::complete_accepted_contract_offer` with the local copy; on success, a staff job for someone
/// without a non-player sheet takes a spare record (`PLAYER_REGEN::take_spare_non_player_record`; an id outside
/// 0..=count shows the line 0x13b4 box), a human club of his contract updates the coaches and the scouts' lists, and
/// the player value is recomputed (`exe::contract_manager_calc_player_value`, `contract::TRANSFER_STATUS` bit 4 of
/// the copy)); it leaves the list (counter − 1, `exe::transfer_manager_clear_agreed_contract_flag`). With a transfer
/// it stays. An offer of another club leaves the list (into `contract_manager_fields::HELD_OFFERS` when the chosen one
/// has a transfer; its club into the third local list). With none chosen, every offer outside state 3 leaves. A failed
/// completion ends here. Finally: with a transfer, the accepted terms (`TRANSFER_MANAGER::handle_player_accepted_terms`
/// with the copy, the job dword and the clubs list; a spare record first for a staff job without a sheet), otherwise
/// the response news of the copy; bit 0x80 cleared. Defect of the original: the spare record id is accepted up to the
/// count itself (`> count` is the error, not `>=`).
/// Original: `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn resolve_contract_offers(this: usize, person: usize) {
    use resolve_offers_fns::*;
    let p = Staff(person);
    let mut chosen_buf = [0u8; 0x70]; // [esp+0x88]: the local copy of the chosen offer
    let chosen = chosen_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(chosen);
    exe::date_date_54a110(chosen + contract_offer::OFFER_DATE);
    let mut buf = [[0u8; 0x10]; 3];
    let l1 = Llist(buf[0].as_mut_ptr() as usize); // the evaluated offers (pointers)
    let l2 = Llist(buf[1].as_mut_ptr() as usize); // their scores (doubles)
    let l3 = Llist(buf[2].as_mut_ptr() as usize); // the clubs of the other offers (ids)
    exe::new_llist(l1);
    exe::new_llist(l2);
    let mut desired_buf = [0u8; 0x50];
    let desired = desired_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(desired);
    exe::new_llist(l3);
    'body: {
        if tm_call1(FN_PLAYER_HAS_PENDING_OFFER, person) != 0 {
            covered!("mkd_4 postponed");
            let mut d = [0u32; 2];
            exe::date_add_days(G_CURRENT_DATE, d.as_mut_ptr() as usize, 2);
            exe::contract_manager_add_decision_date(this, person, d.as_ptr() as *const u8, read_u8(G_CURRENT_DAY_SLICE) as u32, 3);
            break 'body;
        }
        let c = choose_contract_offer(this, p, desired, Some((l1, l2)));
        let best = c.best;
        // second pass: states, news, rejected terms
        l2.set_cursor(l2.head());
        l1.set_cursor(l1.head());
        loop {
            let node = exe::llist_get_next(l1);
            if node == 0 {
                break;
            }
            let _score = read_f64(exe::llist_get_next(l2));
            let o = read_usize(node);
            if o == best && !c.accepted {
                ContractOffer(o).set_state(2);
                remove_transfer_interest(person);
                tm_call1(FN_SET_AGREED_CONTRACT_FLAG, person);
            } else {
                ContractOffer(o).set_state(1);
                let slot = ContractOffer(o).transfer_offer_slot() as i32;
                if slot != -1 {
                    handle_player_rejected_terms(person, slot, best);
                } else {
                    let flag = c.rejected_own == o && !p.player().is_null() && read_u8(c.rejected_own + contract_offer::MARK) == 0xb && best == 0;
                    post_offer_response_news(this, o, flag as u32);
                }
            }
        }
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE));
        if best == 0 {
            if c.rejected_own != 0 {
                covered!("mkd_4 renewal turned down");
                write_u8(state + staff_state::STATUS_FLAGS, read_u8(state + staff_state::STATUS_FLAGS) | 0x80);
            }
        } else {
            // the original copies the chosen offer field by field (the dates through Date::operator=): the same 0x6f bytes
            core::ptr::copy_nonoverlapping(best as *const u8, chosen as *mut u8, OFFER_SIZE);
            let kind = read_u8(chosen + contract::CONTRACT_TYPE) & 0x3f;
            if kind != 5 && kind != 6 && c.best_score > read_f64(rdata::F64_10) {
                let mut n = read_f64(rdata::F64_40) - c.best_score;
                if n < read_f64(rdata::F64_5) {
                    n = read_f64(rdata::F64_5);
                }
                let r = exe::utils_random(ftol(n));
                let id = read_i32(chosen + contract_offer::CLUB_ID);
                if (p.loyalty() as i32) > r && id > -1 {
                    let club = club_at(id as isize);
                    if !club.manager().is_null() && (club.reputation() as i16 > 0x1194 || exe::utils_random(4) == 0) {
                        covered!("mkd_4 favoured");
                        if exe::utils_random(2) != 0 {
                            exe::staff_add_favoured_staff(p, club.manager(), 1);
                        } else {
                            exe::staff_add_favoured_club(p, club, 1);
                        }
                    }
                }
            }
        }
        // third pass: complete, hold or drop the offers
        let list = || Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        let mut failed = false;
        l1.set_cursor(l1.head());
        loop {
            let node = exe::llist_get_next(l1);
            if node == 0 {
                break;
            }
            let o = read_usize(node);
            if best != 0 {
                let transfer = read_i32(chosen + contract_offer::TRANSFER_OFFER_SLOT);
                if read_usize(o + contract_offer::CLUB_ID) == read_usize(chosen + contract_offer::CLUB_ID) {
                    if transfer != -1 {
                        continue;
                    }
                    failed = tm_call1(FN_COMPLETE_ACCEPTED_CONTRACT_OFFER, chosen) == 0;
                    if !failed {
                        covered!("mkd_4 contract completed");
                        if exe::database_is_not_plain_player_job(read_u8(chosen + contract_offer::MARK) as i8) != 0 && p.non_player().is_null() {
                            let (mut id, mut nation, mut second) = (0i32, 0u32, 0u32);
                            take_spare_non_player_record(&mut id as *mut i32 as usize, &mut nation as *mut u32 as usize, &mut second as *mut u32 as usize);
                            if id < 0 || id > read_i32(G_NON_PLAYER_COUNT) {
                                show_contract_error_box(0x13b4);
                            } else {
                                p.set_non_player(NonPlayer(read_usize(G_NON_PLAYER_TABLE).wrapping_add((id as usize).wrapping_mul(0x44))));
                            }
                        }
                        let club = current_contract_club(p);
                        let mut flag = None;
                        if !club.is_null() && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                            exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), p.club_job(), 0);
                            exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), club.manager());
                            if read_u8(chosen + contract::TRANSFER_STATUS) & 4 != 0 && !p.player().is_null() {
                                flag = Some(1);
                            }
                        } else if !p.player().is_null() {
                            write_u8(chosen + contract::TRANSFER_STATUS, read_u8(chosen + contract::TRANSFER_STATUS) | 4);
                            flag = Some(0);
                        }
                        if let Some(f) = flag {
                            let v = exe::contract_manager_calc_player_value(this, p, club, 0, -1, f);
                            p.set_value(v as usize);
                        }
                    }
                    exe::llist_remove(list(), o);
                    resolve_offers_count_down(p);
                    exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person as *const u32);
                    continue;
                }
                if transfer != -1 {
                    // held until the transfer goes through
                    if read_usize(this + contract_manager_fields::HELD_OFFERS) == 0 {
                        let m = exe::crt_operator_new(0xe);
                        let h = if m != 0 { exe::new_llist(m) } else { 0 };
                        write_usize(this + contract_manager_fields::HELD_OFFERS, h);
                    }
                    exe::llist_add_tail(read_usize(this + contract_manager_fields::HELD_OFFERS), o, OFFER_SIZE as u32);
                }
                exe::llist_add_tail(l3, o + contract_offer::CLUB_ID, 4);
            } else if ContractOffer(o).state() == 3 {
                continue;
            }
            exe::llist_remove(list(), o);
            resolve_offers_count_down(p);
        }
        if failed || best == 0 {
            break 'body;
        }
        let transfer = read_i32(chosen + contract_offer::TRANSFER_OFFER_SLOT);
        if transfer != -1 {
            covered!("mkd_4 accepted terms");
            if exe::database_is_not_plain_player_job(read_u8(chosen + contract_offer::MARK) as i8) != 0 && p.non_player().is_null() {
                take_spare_non_player_record(best + contract_offer::RESERVED_RECORD, best + contract_offer::RESERVED_RECORD_ARG1, best + contract_offer::RESERVED_RECORD_ARG2);
            }
            handle_player_accepted_terms(person, transfer, chosen, read_u32(chosen + contract_offer::MARK), l3.addr());
        } else {
            post_offer_response_news(this, chosen, 0);
        }
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE));
        write_u8(state + staff_state::STATUS_FLAGS, read_u8(state + staff_state::STATUS_FLAGS) & 0x7f);
    }
    exe::drop_llist(l3);
    exe::drop_llist(l2);
    exe::drop_llist(l1);
}

/// Comparer for `resolve_contract_offers`.
///
/// The decision allocates, draws, posts news and signs contracts (it cannot run twice), so only the original runs; the
/// plan is the copy's evaluation (`choose_contract_offer` without lists: nothing removed, the entries put back; the
/// generator, the lookup and the static dates restored). After the original: with nothing chosen, the person's contract
/// is the same (club, wage, end) and bit 0x80 of `staff_state::STATUS_FLAGS` is the old one plus the turned-down
/// renewal; with an offer chosen, the contract is either the same (a failed completion, a loan, a transfer still to go
/// through) or has the chosen club and wage, and the bit is cleared or untouched. With a pending transfer offer (the
/// decision is postponed) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_resolve_contract_offers(this: usize, person: usize) {
    use resolve_offers_fns::*;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESOLVE_CONTRACT_OFFERS);
    let p = Staff(person);
    if tm_call1(FN_PLAYER_HAS_PENDING_OFFER, person) != 0 {
        return o(this, person);
    }
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend(TRANSFER_MANAGER_DATE_STATICS);
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let mut desired_buf = [0u8; 0x50];
    let desired = desired_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(desired);
    let plan = crate::rng::pause_trace(|| crate::rng::without_counting(|| choose_contract_offer(this, p, desired, None)));
    before.write_back();
    let terms = || {
        let c = get_contract(person);
        if c.is_null() { (usize::MAX, 0, 0) } else { (c.club(), c.wage(), read_u64(c.addr() + contract::CONTRACT_EXPIRES)) }
    };
    let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS;
    let (t0, bit0) = (terms(), read_u8(state) & 0x80);
    let chosen = if plan.best != 0 { Some((read_usize(plan.best + contract_offer::CLUB_ID), read_i32(plan.best + contract::WAGE))) } else { None };
    o(this, person);
    let (t1, bit1) = (terms(), read_u8(state) & 0x80);
    let same = match chosen {
        None => t1 == t0 && bit1 == (bit0 | if plan.rejected_own != 0 { 0x80 } else { 0 }),
        Some((club, wage)) => (t1 == t0 || (t1.0 == club && t1.1 == wage)) && (bit1 == 0 || bit1 == bit0),
    };
    SCOREBOARD.record(same, || format!(
        "resolve_contract_offers(person {:#x}): plan best {:?} score {} renewal turned down {} agreed {}; contract {:x?} -> {:x?}, bit {:#x} -> {:#x}",
        read_usize(person), chosen, plan.best_score, plan.rejected_own != 0, plan.accepted, t0, t1, bit0, bit1));
}

// ---------- batch 288: the default wage of a person at a club, second copy (staff_contracts.cpp) ----------

/// The `.rdata` constants of the default wage that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68). The value
/// is still read from the exe.
#[allow(dead_code)]
pub(crate) mod unset_default_wage_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250.0
    pub(crate) const F64_350: usize = 0x96f870; // f64 = 350.0
    pub(crate) const F64_1500: usize = 0x96feb8; // f64 = 1500.0
    pub(crate) const F64_2750: usize = 0x96fec8; // f64 = 2750.0
    pub(crate) const F64_3250: usize = 0x96fec0; // f64 = 3250.0
    pub(crate) const F64_20000: usize = 0x968888; // f64 = 20000.0
    pub(crate) const F64_25000: usize = 0x9688d0; // f64 = 25000.0
    pub(crate) const F64_1_125: usize = 0x96a5a8; // f64 = 1.125
    pub(crate) const F64_0_00017: usize = 0x96fed0; // f64 = 0.00017
    pub(crate) const F64_0_00012: usize = 0x96ff10; // f64 = 0.00012
    pub(crate) const F64_1ENEG_09: usize = 0x96ff18; // f64 = 1e-09
    pub(crate) const F64_5ENEG_09: usize = 0x96ff28; // f64 = 5e-09
    pub(crate) const F64_2_5ENEG_08: usize = 0x96ff38; // f64 = 2.5e-08
    pub(crate) const F32_0_2: usize = 0x96b028; // f32 = 0.20000000298023224
    pub(crate) const F32_175: usize = 0x96fee8; // f32 = 175.0
    pub(crate) const F32_275: usize = 0x96ff20; // f32 = 275.0
    pub(crate) const F32_550: usize = 0x96fed8; // f32 = 550.0
    pub(crate) const F32_750: usize = 0x96ff30; // f32 = 750.0
    pub(crate) const F32_1_375: usize = 0x96fedc; // f32 = 1.375
    pub(crate) const F32_0_0125: usize = 0x96fee0; // f32 = 0.012500000186264515
    pub(crate) const F32_0_0004: usize = 0x96fee4; // f32 = 0.00039999998989515007
    pub(crate) const F32_8500: usize = 0x96d4e0; // f32 = 8500.0
    pub(crate) const F32_11000: usize = 0x96feec; // f32 = 11000.0
    pub(crate) const F32_20000: usize = 0x96ff00; // f32 = 20000.0
    pub(crate) const F32_25000: usize = 0x96ff34; // f32 = 25000.0
    pub(crate) const F32_30000: usize = 0x96fefc; // f32 = 30000.0
    pub(crate) const F32_40000: usize = 0x96fef0; // f32 = 40000.0
    pub(crate) const F32_45000: usize = 0x96fef4; // f32 = 45000.0
    pub(crate) const F32_50000: usize = 0x96ff04; // f32 = 50000.0
    pub(crate) const F32_52500: usize = 0x96fef8; // f32 = 52500.0
    pub(crate) const F32_55000: usize = 0x96ff08; // f32 = 55000.0
    pub(crate) const F32_65000: usize = 0x96ff0c; // f32 = 65000.0
    /// "STAFF_CONTRACT::default_staff_contract() - No player datafor player: '%s %s' - id %ld"
    pub(crate) const MSG_NO_PLAYER_DATA: usize = 0xa83968;
    /// the game's text buffer the message is written into
    pub(crate) const G_TEXT_BUFFER: usize = 0xde1f64;
}

/// `(v - t) × m + t` above `t`, in the f32 slot of the original ([esp+0x14]).
fn unset_default_wage_soft_cap(v: f32, t: f64, m: f64) -> f32 {
    if v as f64 > t {
        ((v as f64 - t) * m + t) as f32
    } else {
        v
    }
}

/// The league-standard factor of the minor nations: `base`, or (reputation / 50) / standard × 0.1 × `base` (at most 1)
/// when the club's reputation / 50 is above 10 × the standard of the nation (`nation::LEAGUE_STANDARD`, at least 1).
unsafe fn unset_default_wage_standard_factor(nation: Nation, r50: i32, base: f64) -> f64 {
    let ls = nation.league_standard() as i8;
    if r50 <= ls as i32 * 10 {
        return base;
    }
    let den = if ls < 1 { 1 } else { ls as i32 };
    let k = (r50 as f64 / den as f64) * read_f64(rdata::F64_0_1) * base;
    if k > read_f64(rdata::F64_1) {
        read_f64(rdata::F64_1)
    } else {
        k
    }
}

/// Returns (in st0, an f32 value) the default yearly wage of a person at a club: a second compiled copy of
/// `STAFF_CONTRACT::default_staff_contract` (0x864550) used to fill the wages that are not set (the regens' contract,
/// `set_default_wage`, and the initial shortlists, 0x83a1b0).
///
/// `(person, club, check, status, job, floor)`: first the contract (`exe::contract_manager_get_contract`, stored in
/// `G_CONTRACT_LOOKUP_RESULT`) and its club (none for a loan, type 5); the level = the club's reputation / 500 + 1 (byte
/// arithmetic; with a current club and a club of professional status 2, one less, within 1..4), or without a club the
/// world reputation of the player (or staff) sheet / 500, at least 1. The job is the person's job at the club (with a
/// current club), else his national job, else the club job, else 11 with a player sheet, else 9. By the job:
/// - 5..10 (staff): the level rises to the current club's (or the national side's, outside job 11) reputation / 500 + 1;
///   the wage = level × (current reputation³ × k with a staff sheet, else (id / staff count × 4 + 1) × m × 0.0001) ×
///   0.25, with k / m per job (2.5e-8 / 25000, 1e-8 / 10000, 1e-8 / 10000, 5e-9 / 5000, 1e-9 / 1000, 1e-9 / 1000) and the
///   floors and caps 750 (above 25000 → 150000), 500, 500, 275..5000, 250..1000, 200..1000; jobs 6 to 8 at another club
///   are worth 150000 when the current club's reputation / 500 + 4 is above the level (job 6 offering job 6 or 11 below
///   2500: × 1.35);
/// - 11, 12, 13, 15 (players): without a player sheet, a message and `rand() × 4 / 32768` × 550; with no status (−1), the
///   status by reputation and age (1 a star of 24..33, 2 above 0.75 × the club's, 4 above 0.5 or older than 24, 5 / 6
///   by potential × 50 against 0.75 × the club's); the base by position and loyalty (attacker 50000..65000, midfield
///   40000..52500, goalkeeper 20000..25000, defence 25000..30000; 8500 / 10000 for status 5..6 with low reputation,
///   10000 for coaches of 36 or more), raised 2.5..15 % in a major nation with the FIFA rules; score s = (current + home
///   + world + 4 × max(reputations, floor) + 50 × (ability + potential)) / 9 + the id draw, at least 1500 (× 1.05) for
///   a foreigner from another region; wage = base × s² × 1e-8, at least 175; × 0.75 or 0.5 by the nation's development;
///   × the player's reputation against the club's (0.85..1.15); caps by level (2500, 2750 or 3250 per level; 5000 soft
///   in England, Spain and Italy); a soft cap by position (40000..85000); × (1.375 − days at the club × 0.0125) for a
///   free agent (+60 days in season), at least 0.2;
/// - 14 and the others: 1000.
///
/// With a club and a player sheet: the club's offer by status (club reputation × 3 × (reputation × 0.0001..0.00017)⁶,
/// at most 1, × 0.05..1.5 by status and age; the average when it is higher), × 0.67..7.5 by status with grievance 0x10;
/// the nation's market (Japan floor level² × 10, USA × 0.75 and caps by age and league, the league-standard factor of
/// the minor nations and their soft caps); at most the wage of a current contract of status 7; the club's wage cap
/// (`exe::finance_get_max_wage`) when checking or without a club (reputation below 2250), and the ambition bonus
/// (1.05..1.15) of a star; the floors 50 / 75 (pro) or 100 / 75 (amateur, under 20). No club: the wage as computed.
/// Original: `staff_contract_calc_unset_default_wage` (0x862720, staff_contracts.cpp, stdcall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    use unset_default_wage_rdata::*;
    let p = Staff(person);
    let c = Club(club);
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    let cur = if contract == 0 || read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 5 {
        Club(0)
    } else {
        club_at(read_i32(contract + contract::CLUB) as isize)
    };
    let age = p.age() as i8;
    let sheet = p.player();
    let np = p.non_player();
    let frac = (read_i32(p.addr()) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // fild id; fidiv count; fstp dword
    let job_arg = job as u8 as i8;
    let mut status = status; // [esp+0x40]: the byte can be rewritten (the dword goes to get_max_wage)
    // the level: the club's reputation / 500 + 1 (byte arithmetic)
    let mut lvl: i32;
    if !c.is_null() {
        lvl = ((c.reputation() as i32 / 500) as u8).wrapping_add(1) as i8 as i32;
        if !cur.is_null() && c.professional_status() == 2 {
            lvl -= 1;
            lvl = lvl.clamp(1, 4);
        }
    } else {
        let q = if !sheet.is_null() {
            Some(sheet.world_reputation() as i32 / 500)
        } else if !np.is_null() {
            Some(np.world_reputation() as i32 / 500)
        } else {
            None
        };
        lvl = match q {
            Some(q) => (if q < 1 { 1 } else { q }) as u8 as i8 as i32,
            None => 1,
        };
    }
    // the job the wage is computed for
    let jfc = p.job_for_club() as i8;
    let jfn = p.job_for_nation() as i8;
    let own_job: i8 = if !cur.is_null() && jfc != 0 {
        jfc
    } else if !p.national_job().is_null() && jfn != 0 {
        jfn
    } else if jfc != 0 {
        jfc
    } else if !sheet.is_null() {
        11
    } else if jfn != 0 {
        jfn
    } else {
        9
    };
    // the level rises to the current club's (or the national side's) reputation / 500 + 1 (staff jobs)
    let raise_level = |lvl: &mut i32| {
        if !cur.is_null() {
            let t = cur.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
                return;
            }
        }
        let nat = p.national_job();
        if !np.is_null() && !nat.is_null() && jfn != 11 {
            let t = nat.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
            }
        }
    };
    // the staff wage: level × (current reputation³ × k, or (the id draw × 4 + 1) × m × 0.0001) × 0.25
    let staff_wage = |lvl: i32, k: usize, m: usize| -> f32 {
        let x = if !np.is_null() {
            let r = np.current_reputation() as i32 as f64;
            r * r * r * read_f64(k)
        } else {
            ((frac as f64 * read_f32(rdata::F32_4) as f64 + read_f32(rdata::F32_1) as f64) * read_f32(m) as f64) * read_f64(rdata::F64_0_0001)
        };
        (lvl as f64 * x * read_f64(rdata::F64_0_25)) as f32
    };
    // another club's job when the current club's reputation / 500 + 4 is above the level
    let above_current = |lvl: i32| !cur.is_null() && cur != c && cur.reputation() as i32 / 500 + 4 > lvl;
    let a5_player_or_6 = job_arg == 11 || job_arg == 6;
    let mut v: f32;
    covered!("mkd_6 default wage");
    match own_job {
        5 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_2_5ENEG_08, F32_25000);
            if read_f32(F32_750) <= v {
                if v > read_f32(F32_25000) {
                    v = 150000.0;
                }
            } else {
                v = 750.0;
            }
        }
        6 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if !cur.is_null() && cur != c {
                // at another club: 150000 unless the job offered is 6 or 11 and the level reaches the current club's
                if a5_player_or_6 && !above_current(lvl) {
                    if v < read_f32(rdata::F32_2500) {
                        v = (v as f64 * read_f64(rdata::F64_1_35)) as f32;
                    }
                } else {
                    covered!("mkd_6 another club's staff job");
                    v = 150000.0;
                }
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        7 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if above_current(lvl) {
                v = 150000.0;
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        8 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_5ENEG_09, rdata::F32_5000);
            if read_f32(F32_275) <= v {
                if v > read_f32(rdata::F32_5000) {
                    v = 5000.0;
                }
            } else {
                v = 275.0;
            }
            if job_arg != 6 && job_arg != 11 && above_current(lvl) {
                v = 150000.0;
            }
        }
        9 | 10 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_1ENEG_09, rdata::F32_1000);
            let floor_ = if own_job == 9 { rdata::F32_250 } else { rdata::F32_200 };
            if read_f32(floor_) <= v {
                if v > read_f32(rdata::F32_1000) {
                    v = 1000.0;
                }
            } else {
                v = read_f32(floor_);
            }
        }
        11 | 12 | 13 | 15 => {
            if sheet.is_null() {
                // the original writes the error into the game's text buffer and draws the wage
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(G_TEXT_BUFFER, MSG_NO_PLAYER_DATA, read_u32(p.addr() + staff::FIRST_NAME), read_u32(p.addr() + staff::SECOND_NAME), read_i32(p.addr()));
                let r = exe::crt_rand();
                let q = (r as i64 * 4) / 0x8000; // __allmul, __alldiv
                v = (q as i32 as f64 * read_f32(F32_550) as f64) as f32;
            } else {
                v = unset_default_wage_player(p, c, cur, sheet, age, frac, lvl, job_arg, floor as u8 as i8, &mut status);
            }
        }
        _ => v = 1000.0,
    }
    if c.is_null() {
        return v as f64;
    }
    if !sheet.is_null() {
        // what the club would pay by squad status: club reputation × 3 × (reputation × k)⁶ (at most 1) × the status' factor
        let rep = c.reputation() as i32;
        let x = if c.professional_status() == 1 {
            rep as f64 * read_f64(F64_0_00017)
        } else if c.reputation() > 0x8ca {
            rep as f64 * read_f64(F64_0_00012)
        } else {
            rep as f64 * read_f64(rdata::F64_0_0001)
        };
        let mut pw = game_pow(x, read_f64(rdata::F64_6));
        if pw > read_f64(rdata::F64_1) {
            pw = 1.0;
        }
        let r3 = (rep * 3) as f64 * pw;
        let major = || club_is_in_major_nation(c.addr()) != 0;
        let st = status as u8 as i8;
        let y: Option<f64> = match st {
            1 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_6 } else if age < 0x18 || !major() { rdata::F64_1_25 } else { rdata::F64_1_5 })),
            2 => Some(if age < 0x14 { r3 * read_f64(rdata::F64_0_3) } else if age < 0x18 || !major() { r3 * read_f64(rdata::F64_0_6) } else { r3 }),
            3 => {
                if age < 0x14 {
                    let y0 = r3 * read_f64(rdata::F64_0_25);
                    if (v as f64) < y0 {
                        v = ((v as f64 + y0) * read_f64(rdata::F64_0_5)) as f32;
                    }
                }
                Some(r3 * read_f64(if age < 0x18 || !major() { rdata::F64_0_3 } else { rdata::F64_0_6 }))
            }
            4 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_1 } else if age < 0x18 || !major() { rdata::F64_0_15 } else { rdata::F64_0_3 })),
            5 => Some(r3 * read_f64(rdata::F64_0_05)),
            6 => Some(r3 * read_f64(rdata::F64_0_02)),
            _ => None,
        };
        if let Some(y) = y {
            if (v as f64) < y {
                covered!("mkd_6 club's offer by status");
                v = ((v as f64 + y) * read_f64(rdata::F64_0_5)) as f32; // mkd6: the average with the club's offer
            }
        }
        let k = (st as i32 - 1) as u32;
        let c2 = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        if c2 != 0 && read_u8(c2 + contract::GRIEVANCES) & 0x10 != 0 && k <= 5 {
            let m = match k {
                0 | 1 => rdata::F64_0_67,
                2 => rdata::F64_0_85,
                3 => rdata::F64_2_5,
                4 => rdata::F64_3_5,
                _ => rdata::F64_7_5,
            };
            v = (v as f64 * read_f64(m)) as f32;
        }
    }
    // the market of the club's nation
    let nat = c.nation();
    if !nat.is_null() {
        let cj = p.club_job();
        let pn = p.nation();
        let applies = if !cj.is_null() {
            cj.nation() == nat && (pn == nat || p.loyalty() > 3 || p.ambition() < 0xf)
        } else {
            pn == nat
        };
        if applies {
            covered!("mkd_6 nation's market");
            let r50 = c.reputation() as i32 / 50;
            let id = read_u32(nat.addr());
            let is = |k: usize| id == read_u32(k);
            let prof = c.professional_status() == 1;
            if is(KEY_NATION_JAPAN) {
                let fl = lvl * lvl * 10;
                if (v as f64) < fl as f64 {
                    v = fl as f32;
                }
                v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_7500), read_f64(rdata::F64_0_25));
            } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
            } else if is(KEY_NATION_UNITED_STATES) {
                v = (v as f64 * read_f64(rdata::F64_0_75)) as f32;
                let d = read_usize(c.addr() + club::DIVISION);
                if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_15));
                } else if pn == nat {
                    let young = age <= 0x1f && (age >= 0x18 || sheet.is_null() || sheet.world_reputation() >= 0xea6);
                    v = unset_default_wage_soft_cap(v, read_f64(if young { F64_2750 } else { rdata::F64_1250 }), read_f64(rdata::F64_0_2));
                } else {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_3250), read_f64(rdata::F64_0_25));
                }
            } else {
                let scale = |v: f32, base: f64| (v as f64 * unset_default_wage_standard_factor(nat, r50, base)) as f32;
                if is(KEY_NATION_SOUTH_KOREA) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_5));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_1500), read_f64(rdata::F64_0_5));
                } else if is(KEY_NATION_SCOTLAND) {
                    v = scale(v, read_f64(rdata::F64_0_35));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_10000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
                    v = scale(v, read_f64(rdata::F64_0_75));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_20000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_SWEDEN) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_150), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_200), read_f64(rdata::F64_0_15))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_100), read_f64(rdata::F64_0_15))
                    };
                } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) || is(KEY_NATION_POLAND) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(F64_350), read_f64(rdata::F64_0_1))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_1))
                    };
                } else if is(KEY_NATION_BELGIUM) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_25000), read_f64(rdata::F64_0_25));
                } else {
                    // the other nations: × 0.3, the soft cap tested on the unrounded product
                    let x = v as f64 * read_f64(rdata::F64_0_3);
                    v = x as f32;
                    if x > read_f64(rdata::F64_15000) {
                        v = ((v as f64 - read_f64(rdata::F64_15000)) * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_15000)) as f32;
                    }
                }
            }
        }
    }
    if !sheet.is_null() {
        // at most the wage of a current contract of squad status 7
        if !cur.is_null() && c != cur {
            let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur);
            if k != 0 && read_u8(k + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(k + contract::WAGE) as f64;
                if v as f64 > w {
                    v = w as f32;
                }
            }
        }
        let mut ambition = check == 0 && !cur.is_null();
        if check != 0 || (cur.is_null() && sheet.current_reputation() < 0x8ca) {
            // the club's wage cap
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if mw < v as f64 {
                covered!("mkd_6 club's wage cap");
                v = mw as f32;
            }
            ambition = false;
        }
        if ambition
            && p.ambition() > 10
            && p.loyalty() < 6
            && sheet.current_reputation() > 0x1a5e
            && (sheet.attacker() == 0x14 || sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14)
            && age > 0x14
        {
            exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur); // the result is not used
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if v as f64 * read_f64(rdata::F64_1_25) < mw {
                let m = if sheet.attacking_midfielder() == 0x14 {
                    rdata::F64_1_1
                } else if sheet.midfielder() == 0x14 {
                    rdata::F64_1_05
                } else {
                    rdata::F64_1_15
                };
                v = (v as f64 * read_f64(m)) as f32;
            }
        }
    }
    // the floors
    if c.professional_status() != 1 {
        if c.reputation() > 0x8ca && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_50) as f64 {
            return 50.0;
        }
        v as f64
    } else {
        if age < 0x14 && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_100) as f64 {
            v = 100.0;
        }
        v as f64
    }
}

/// The player's wage of `staff_contract_calc_unset_default_wage` (jobs 11, 12, 13 and 15, with a player sheet): the
/// squad status by reputation and age when it is −1 (written back into `status`), the base by position and loyalty, the
/// score of the reputations and abilities, the nation's development, the reputation ratio, the caps by level and
/// position and the free agent's factor.
#[allow(clippy::too_many_arguments)]
unsafe fn unset_default_wage_player(p: Staff, c: Club, cur: Club, sheet: PlayerSheet, age: i8, frac: f32, lvl: i32, job_arg: i8, floor: i8, status: &mut u32) -> f32 {
    use unset_default_wage_rdata::*;
    let mut st = *status as u8 as i8;
    if st == -1 && !c.is_null() {
        // the squad status the club would give, by reputation and age
        let pr = sheet.current_reputation();
        let cr = c.reputation();
        st = if pr > cr && (0x18..0x22).contains(&age) {
            1
        } else {
            let d = cr as i32 as f64 * read_f64(rdata::F64_0_75);
            if pr as i32 as f64 > d && age >= 0x15 {
                2
            } else if (pr as i32 as f64 > cr as i32 as f64 * read_f64(rdata::F64_0_5) && age >= 0x11) || age > 0x18 {
                4
            } else if (sheet.potential_ability() as i32 * 50) as f64 > d {
                5
            } else {
                6
            }
        };
        *status = (*status & 0xffff_ff00) | st as u8 as u32;
    }
    // the base by position and loyalty
    let loy = p.loyalty();
    let fac: f32 = if (job_arg == 0xd || job_arg == 0xf) && age > 0x23 {
        read_f32(rdata::F32_10000)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x9c4 {
        read_f32(F32_8500)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x186a {
        read_f32(rdata::F32_10000)
    } else if sheet.attacker() >= 0xf {
        if sheet.current_reputation() > 0x1a5e {
            read_f32(if loy < 0xf { F32_65000 } else { F32_55000 })
        } else {
            read_f32(F32_50000)
        }
    } else if sheet.defensive_midfielder() >= 0xf || sheet.attacking_midfielder() >= 0xf || sheet.midfielder() >= 0xf {
        if sheet.current_reputation() > 0x1c52 {
            read_f32(if loy < 0xe { F32_52500 } else { F32_45000 })
        } else {
            read_f32(F32_40000)
        }
    } else if sheet.goalkeeper() == 0x14 {
        read_f32(if sheet.world_reputation() > 0x1e46 && loy < 3 { F32_25000 } else { F32_20000 })
    } else if sheet.sweeper() < 0xf && sheet.defender() == 0 {
        read_f32(F32_25000)
    } else if sheet.world_reputation() > 0x1d4c && loy < 6 {
        read_f32(F32_30000)
    } else {
        read_f32(F32_25000)
    };
    let mut base = fac as f64;
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && fac > read_f32(F32_11000) && !c.is_null() {
        if club_is_in_major_nation(c.addr()) != 0 {
            if sheet.current_reputation() > 0x1a5e {
                base = fac as f64 * read_f64(if loy == 1 { rdata::F64_1_15 } else if loy < 6 { F64_1_125 } else { rdata::F64_1_1 });
            } else if sheet.current_reputation() > 0x109a {
                base = fac as f64 * read_f64(rdata::F64_1_025);
            }
        }
    }
    // s = (current + home + world + 4 × the greatest (with the floor) + 50 × (ability + potential)) / 9 + the id draw
    let (rc, rh, rw) = (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation());
    let mut mx = rc;
    if rh > mx {
        mx = rh;
    }
    if rw > mx {
        mx = rw;
    }
    if floor == 1 {
        if mx < 0x1c52 {
            mx = 0x1c52;
        }
    } else if floor == 2 && mx < 0x1a5e {
        mx = 0x1a5e;
    }
    let draw = ftol(frac as f64 * read_f32(rdata::F32_8) as f64 - read_f32(rdata::F32_4) as f64);
    let sum = (sheet.potential_ability() as i32 + sheet.current_ability() as i32) * 50 + mx as i32 * 4 + rh as i32 + rc as i32 + rw as i32;
    let mut s = (sum / 9).wrapping_add(draw) as u32;
    // a foreigner from another region: at least 1500, × 1.05
    let pn = p.nation();
    let foreign = !pn.is_null() && {
        let cn = c.nation();
        !cn.is_null()
            && pn.actual_region() != cn.actual_region()
            && (cur.is_null() || (!cur.nation().is_null() && cur.nation().actual_region() != cn.actual_region()))
    };
    if foreign {
        covered!("mkd_6 foreigner from another region");
        if s < 0x5dc {
            s = 0x5dc;
        } else {
            s = ftol(s as u64 as f64 * read_f64(rdata::F64_1_05)) as u32;
            if s <= 1 {
                s = 1;
            }
        }
    } else if s <= 1 {
        s = 1;
    }
    let sq = s.wrapping_mul(s) as u64 as f64;
    let x = base * sq * read_f64(rdata::F64_1ENEG_08); // mkd6: the wage by the score
    let mut v = x as f32;
    if x < read_f32(F32_175) as f64 {
        v = 175.0;
    }
    // the development of the current club's (or the person's) nation
    let dev = if !cur.is_null() && !cur.nation().is_null() {
        cur.nation().state_of_development() as i8
    } else if !pn.is_null() {
        pn.state_of_development() as i8
    } else {
        3
    };
    match dev {
        1 => {}
        2 => v = (v as f64 * read_f32(rdata::F32_0_75) as f64) as f32, // mkd6: the nation's development
        _ => v = (v as f64 * read_f32(rdata::F32_0_5) as f64) as f32,
    }
    // the player's reputation against the club's, within 0.85..1.15
    if !c.is_null() && c.reputation() != 0 {
        let mut r = rc as i32 as f64;
        if c.reputation() < 0x9c4 {
            r *= read_f32(F32_0_0004) as f64;
        } else {
            r /= (lvl * 500) as f64;
        }
        if r < read_f64(rdata::F64_0_85) {
            r = read_f32(rdata::F32_0_8500000238) as f64;
        }
        if r > read_f64(rdata::F64_1_15) {
            r = read_f32(rdata::F32_1_149999976) as f64;
        }
        v = (r * v as f64) as f32;
    }
    // the caps by level (soft in England, Spain and Italy)
    let hard = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = cap as f32;
        }
    };
    let soft = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = ((*v as f64 - cap as f64) * read_f64(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if lvl < 10 {
        hard(&mut v, lvl * 2500);
    } else if lvl < 15 {
        hard(&mut v, lvl * 2750);
    } else {
        let n = c.nation(); // the original reads +0x53 of the club (a null club with a level of 15 would fault)
        let big = !n.is_null() && {
            let id = read_u32(n.addr());
            id == read_u32(KEY_NATION_ENGLAND) || id == read_u32(KEY_NATION_SPAIN) || id == read_u32(KEY_NATION_ITALY)
        };
        if !big {
            hard(&mut v, lvl * 3250);
        } else if c.plc() != 0 {
            soft(&mut v, lvl * 3250);
        } else {
            soft(&mut v, lvl * 5000);
        }
    }
    // the soft cap by position
    let mut lim: f32 = if sheet.attacker() == 0x14 {
        85000.0
    } else if sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14 {
        75000.0
    } else if sheet.defensive_midfielder() == 0x14 {
        65000.0
    } else if sheet.defender() == 0x14 {
        50000.0
    } else {
        40000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && v > read_f32(F32_11000) && !c.is_null() && club_is_in_major_nation(c.addr()) != 0 && rc > 0x1c52 {
        lim = (lim as f64 * read_f64(rdata::F64_1_1)) as f32;
    }
    if v > lim {
        v = ((v as f64 - lim as f64) * read_f64(rdata::F64_0_2) + lim as f64) as f32;
    }
    // the free agent: × (1.375 − days at the club × 0.0125), at least 0.2 (veterans of the national side excepted)
    if p.club_job().is_null() {
        let veteran = age >= 0x21
            && p.int_apps() != 0
            && sheet.work_rate() as i32 + p.determination() as i32 <= 0x1e
            && read_i32(p.addr()) % 0xf != 0
            && c.reputation() <= 0x1194;
        if !veteran {
            covered!("mkd_6 free agent");
            let days = exe::date_days_since(G_CURRENT_DATE, read_u32(p.addr() + staff::DATE_JOINED_CLUB), read_u32(p.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
            let mut t = days as f32 as f64;
            if exe::club_is_in_season(c, G_CURRENT_DATE) != 0 {
                t += read_f32(rdata::F32_60) as f64;
            }
            t = read_f32(F32_1_375) as f64 - t * read_f32(F32_0_0125) as f64;
            if !(read_f32(F32_0_2) as f64 <= t) {
                t = read_f32(F32_0_2) as f64;
            }
            v = (t * v as f64) as f32;
        }
    }
    v
}

/// Comparer for `staff_contract_calc_unset_default_wage`: original and copy side by side over
/// `G_CONTRACT_LOOKUP_RESULT`, the start of the text buffer (the no-player message) and the generator's state.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f64>(IDX_STAFF_CONTRACT_CALC_UNSET_DEFAULT_WAGE);
    let est = crate::rng::STATE_REGIONS;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (unset_default_wage_rdata::G_TEXT_BUFFER, 0x80), est[0], est[1]];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(person, club, check, status, job, floor),
        || crate::rng::without_counting(|| staff_contract_calc_unset_default_wage(person, club, check, status, job, floor)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_calc_unset_default_wage(person {:#x}, club {:#x}, {}, {}, {}, {}) original={} rust={} {:x?} x {:x?}",
        person, club, check, status as u8 as i8, job as u8, floor as u8, r, n, mo, mr));
    r
}

// ---------- batch 288: a person's will to take a job at a club (shortlist_manager.cpp) ----------

/// The `.rdata` constants of the will to take a job that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68). The
/// value is still read from the exe.
#[allow(dead_code)]
pub(crate) mod job_interest_rdata {
    pub(crate) const F64_900: usize = 0x96f958; // f64 = 900.0
    pub(crate) const F64_1_45: usize = 0x96f940; // f64 = 1.45
    pub(crate) const F64_1_176470588: usize = 0x96f950; // f64 = 1.1764705882352942 (1 / 0.85)
    pub(crate) const F64_2_222222222: usize = 0x96f948; // f64 = 2.2222222222222223 (1 / 0.45)
    pub(crate) const F64_0_0002222222222: usize = 0x96f928; // f64 = 0.00022222222222222223 (1 / 4500)
    /// the three names the will is 0 for when they have a club job: "Dario Gradi", "Pat Nevin", "Kevin Keegan"
    pub(crate) const NAMES: [usize; 3] = [0xa82620, 0xa82614, 0xa82604];
}

/// The square of the state of development (1..3, otherwise 3) of a nation (3 without one), as a byte.
unsafe fn job_interest_development_squared(n: Nation) -> i8 {
    let d = if n.is_null() { 3 } else { n.state_of_development() as i8 };
    let d = if !(1..=3).contains(&d) { 3 } else { d };
    d.wrapping_mul(d)
}

/// The reputation of a club's competition: the club's continental competition (`club::CONTINENTAL_COMP_ID`, or the one
/// in `G_CLUB_STATE_TABLE` when it is −1 or, with `intertoto`, the Intertoto Cup of a club of reputation 5000), 600 ×
/// its reputation in 16 bits; without one, 0.8 × the division's value, at most 7000. Returns (value, record).
unsafe fn job_interest_by_competition(club: Club, by_division: i16, intertoto: bool) -> (i16, usize) {
    let mut idx = club.continental_comp_id() as i32;
    let mut has = true;
    if idx <= -1 || (intertoto && idx == read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) && club.reputation() >= 0x1388) {
        idx = read_i32(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(6)));
        if idx <= -1 {
            has = false;
        }
    }
    if has {
        let reg = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            return (read_i16(reg + competition::REPUTATION).wrapping_mul(600), reg);
        }
    }
    (job_interest_by_division_cap(by_division), 0)
}

/// 0.8 × the division's value, at most 7000 (truncated).
unsafe fn job_interest_by_division_cap(by_division: i16) -> i16 {
    let mut x = by_division as f64 * read_f64(rdata::F64_0_8);
    if !(x < read_f64(rdata::F64_7000)) {
        x = read_f64(rdata::F64_7000);
    }
    ftol(x) as i16
}

/// The manager's reputation `r` as the board sees it: v = 0.67 × the club's row +6 + 0.33 × +8 in the manager-manager
/// table (0 without a club); below 5000, the greater of r / t and r − 1000 t with t = 5 − 0.0008 v; otherwise the lesser
/// of r × t and r + 1000 t with t = 0.0001 v + 0.5.
unsafe fn job_interest_by_board(club: Club, r: i16) -> i32 {
    let (w6, w8) = if club.is_null() {
        (0i16, 0i16)
    } else {
        let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(0x49));
        (read_i16(t + 6), read_i16(t + 8))
    };
    let v = ftol(w6 as f64 * read_f64(rdata::F64_0_67) + w8 as f64 * read_f64(rdata::F64_0_33)) as i16;
    let d = r as f64;
    if v < 0x1388 {
        let t = read_f64(rdata::F64_5) - v as f64 * read_f64(rdata::F64_0_0008);
        let q1 = d / t;
        let m = d - t * read_f64(rdata::F64_1000);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * read_f64(rdata::F64_0_0001) + read_f64(rdata::F64_0_5);
        let q1 = d * t;
        let m = t * read_f64(rdata::F64_1000) + d;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// The language factor: max(F × 0.1, (e + 5000)² × 1/15000 / (12000 − 200 F)) + 0.2, with F the person's fluency in the
/// club nation's first language (`exe::playerstats_get_language_fluency`; 10 without a player sheet).
unsafe fn job_interest_language_factor(p: Staff, club: Club, e: i16) -> f64 {
    let q = (e as i32 + 0x1388) as f64;
    let l = q * read_f64(rdata::F64_6_666666667ENEG_05) * q;
    let f = if !p.player().is_null() {
        exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, (club.nation().first_language() as u8) as u32, 1) as i8 as f64
    } else {
        read_f64(rdata::F64_10)
    };
    let a = f * read_f64(rdata::F64_0_1);
    let b = l / (read_f64(rdata::F64_12000) - f * read_f64(rdata::F64_200));
    max_f64(a, b) + read_f64(rdata::F64_0_2)
}

/// A club's weighted reputation on the person's scale (`e` = ftol(1.5 × r − 5000), the int-apps and status cuts
/// included), from the stadium, the division, the competition, the manager, the finances and the shortlists.
/// `current`: the club the person is at (no board view of a manager seen by the person, other wealth bands, ×1.35 for a
/// national side, the Intertoto rule). Returns (e, by division, competition record).
#[allow(clippy::too_many_arguments)]
unsafe fn job_interest_club_scale(this: usize, p: Staff, club: Club, job: i8, w: (f32, f32, f32, f32, f32), current: bool) -> (i16, i16, usize) {
    let (w60, w84, l74, w88, den) = w;
    let stad = reputation_by_stadium(club) as i16;
    let div = reputation_by_division(club);
    let (comp, rec) = job_interest_by_competition(club, div, current);
    let m = club.manager();
    let mgr: i16 = if current {
        if m.is_null() || m.non_player().is_null() {
            ftol(div as f64 * read_f64(rdata::F64_0_75)) as i16
        } else {
            let r = m.non_player().current_reputation();
            if read_usize(G_MANAGER_MANAGER) == 0 || job == 5 {
                r
            } else {
                job_interest_by_board(club, r) as i16
            }
        }
    } else if m.is_null() || m.non_player().is_null() {
        0
    } else {
        let r = exe::database_get_staff_reputation_for_viewer(m, p, 1);
        if read_usize(G_MANAGER_MANAGER) == 0 || job == 5 {
            r
        } else {
            job_interest_by_board(club, r) as i16
        }
    };
    // the reputation by the finances' wealth band
    let mut si: i16 = club.reputation();
    if exe::database_is_national_team(club) == 0 {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        if fin != 0 {
            let r = si as f64;
            match exe::finance_get_wealth_band(fin) as i8 {
                -2 => si = ftol(r * read_f64(if current { rdata::F64_0_75 } else { rdata::F64_0_7 })) as i16,
                -1 => si = ftol(r * read_f64(if current { rdata::F64_0_9 } else { rdata::F64_0_85 })) as i16,
                1 => {
                    si = ftol(r * read_f64(rdata::F64_1_25)) as i16;
                    let min = if current { 0x128e } else { 0x109a };
                    if si < min {
                        si = min;
                    }
                }
                2 => {
                    si = ftol(r * read_f64(rdata::F64_1_5)) as i16;
                    if si < 0x1676 {
                        si = 0x1676;
                    }
                }
                _ => {}
            }
        }
    }
    let ax: i16 = if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 {
        ftol(si as f64 * read_f64(if current { rdata::F64_1_35 } else { job_interest_rdata::F64_1_45 })) as i16
    } else {
        let a = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(9)) + 5);
        ftol(a as f64 * read_f64(rdata::F64_0_4) + si as f64 * read_f64(rdata::F64_0_6)) as i16
    };
    let s = (((comp as f64 * w60 as f64 + div as f64 * w84 as f64) + mgr as f64 * l74 as f64) + stad as f64 * w88 as f64) + ax as i32 as f64;
    let mut e = ftol(s / den as f64) as i16;
    if !current && exe::database_is_national_team(club) != 0 && club.nation() == p.nation() {
        e = ftol(e as f64 * read_f64(job_interest_rdata::F64_1_45)) as i16;
    }
    if exe::database_is_national_team(club) == 0 && club.professional_status() != 1 {
        // the reputation is cut for a non-professional club: min(e / 2, e − 5 × (int apps² + 400)), status 3 again
        let ia = p.int_apps() as i32;
        let lim = (ia * ia + 400) * 5;
        let a = e as i32;
        let (h, x) = (a / 2, a - lim);
        e = (if h < x { h } else { x }) as i16;
        if club.professional_status() == 3 {
            e = half_or_minus_2000(e);
        }
    }
    (ftol(e as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16, div, rec)
}

/// Compares the C string `a` with the one at `b` in the exe.
unsafe fn job_interest_name_is(a: &[u8], b: usize) -> bool {
    let mut k = 0usize;
    loop {
        let x = a[k];
        let y = read_u8(b + k);
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// Returns in st0 a person's will to take a job (`job`) at a club and marks the reason bits, as
/// `staff_interest_in_joining_club` (0x846920) does for a player.
///
/// `staff_interest_in_job_at_club(person, club, check, job, reasons)` (the shortlist manager in ecx): 0 without a staff
/// sheet, without the club's or the person's nation, for Dario Gradi, Pat Nevin and Kevin Keegan when they have a club
/// job, for a media pundit (job 17), for the last 16 records (outside job 5), for a national manager (job 5 abroad), for a
/// staff member of reputation above 4250 or ability above 100 wanting a manager's job (5 or 12) elsewhere, for a physio
/// (10) asked for another job, for a player with a club job asked by another club, for a manager going to a club
/// of lower division or reputation (more than 750 below) and for a club below 2750 of his own nation or that finds him
/// foreign. Otherwise, the base: 3.5 at his own club; 2.25 for a manager's job, otherwise 1.5, plus (days out of a club
/// since the game began)² / (reputation + 900), at most 12; ×0.33 / ×2 / ×0.5..0.85 / ×3 by disliking and favouring
/// the clubs, ×0.5..0.85 − loyalty against a rival (Celtic and Rangers, Italy, the same division), ×0.05 / ×0.1 between
/// Croatia and Yugoslavia, ×2 / ×0.33 / ×2 / ×0.5 by the managers' liking; 0 with a future transfer; the contract's
/// morale; by the current job (×0.01 to ×10) and the job offered (×0.1 to ×10, 0 for job 0); ×1.4 for a manager the
/// board has little confidence in; the language factor; ×0.85..0.01 or ×0.3..0.001 for a foreigner by job; the time at
/// the club (0 below (loyalty + 8) × 15 days); the two clubs' weighted reputations (stadium, division, competition,
/// manager, wealth band, shortlists) and their difference by ambition; the development ratio of the two nations and
/// the continents; the reputation against his own. With `check` and a will of 1.75 or more: the default wage
/// (`exe::staff_contract_default_staff_contract`) × 3.5 / will against the club's staff wage cap
/// (`exe::finance_get_max_staff_wage`) and against his current wage.
/// Original: `staff_interest_in_job_at_club` (0x844780, shortlist_manager.cpp, thiscall, ret 0x14).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn staff_interest_in_job_at_club(this: usize, person: usize, club: usize, check: u32, job_arg: u32, reasons: *mut u32) -> f64 {
    let p = Staff(person);
    let c = Club(club);
    let job = job_arg as u8 as i8;
    let zero = read_f64(rdata::F64_0);
    let fl = move |b: u32| {
        if !reasons.is_null() {
            *reasons |= b;
        }
    };
    if !reasons.is_null() {
        *reasons = 0;
    }
    if p.non_player().is_null() || c.nation().is_null() || p.nation().is_null() {
        return zero;
    }
    // three managers never move from a club job
    let mut buf = [0u8; 0x1f4];
    for name in job_interest_rdata::NAMES {
        exe::database_format_staff_name(p, buf.as_mut_ptr() as usize, 0x1f4, 1);
        if job_interest_name_is(&buf, name) {
            if !p.club_job().is_null() {
                return zero;
            }
            break;
        }
    }
    let jfc = p.job_for_club() as i8;
    let jfn = p.job_for_nation() as i8;
    if jfc == 0x11 || jfn == 0x11 {
        return zero;
    }
    if job != 5 && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) - 0x10 {
        return zero;
    }
    let nj = p.national_job();
    if !nj.is_null() && jfn == 5 {
        return zero;
    }
    let manager_now = jfc == 5 || jfc == 0xc || jfn == 5 || jfn == 0xc;
    if job != 0 {
        if job != 5 && manager_now && (p.non_player().current_reputation() > 0x109a || p.non_player().current_ability() > 100) {
            return zero;
        }
        if job == 10 && jfc != 10 && jfc != 0 {
            return zero;
        }
        if jfn == 5 && !nj.is_null() {
            return zero;
        }
        if job != 0xf && job != 0xc && job != 5 && !p.player().is_null() && jfc == 0xb {
            let cj = p.club_job();
            if !cj.is_null() && c != cj {
                return zero;
            }
        }
        if job != 10 && jfc == 10 {
            return zero;
        }
    }
    if !c.is_null() {
        if job != 5 && manager_now {
            let cj = p.club_job();
            if !cj.is_null() {
                let (dn, dc) = (c.division(), cj.division());
                if !dn.is_null() && !dc.is_null() && dc.reputation() >= dn.reputation() {
                    return zero;
                }
                if cj.reputation() as i32 > c.reputation() as i32 - 0x2ee {
                    return zero;
                }
            }
        }
        if c.reputation() < 0xabe {
            let n = c.nation();
            if !n.is_null() {
                let cj = p.club_job();
                if !cj.is_null() && cj.nation() == n {
                    return zero;
                }
                if exe::database_is_staff_foreign(p, n, 1) != 0 {
                    return zero;
                }
            }
        }
    }
    // the weights of the reputations (ambition and professionalism)
    let l74: f32 = if job == 5 || job == 0xc { 0.0 } else { 1.0 };
    let amb = p.ambition() as i32 as f64;
    let w88 = ((read_f64(rdata::F64_1_4) - amb * read_f64(rdata::F64_0_025)) - p.professionalism() as i32 as f64 * read_f64(rdata::F64_0_025)) as f32;
    let w84 = (amb * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_0_8)) as f32;
    let b = amb * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5);
    let w60 = b as f32;
    let d = (((b + w84 as f64) + w88 as f64) + l74 as f64) + read_f32(rdata::F32_1) as f64;
    let den: f32 = if d > zero { d as f32 } else { 0.5 };
    let w = (w60, w84, l74, w88, den);
    // the club he is at (or the national side), and his job there
    let mut cj = p.club_job();
    let mut jb = jfc;
    if (cj.is_null() || exe::database_is_national_team(c) != 0) && !nj.is_null() && jfn != 0xb {
        cj = nj;
        jb = jfn;
    }
    if !cj.is_null() && cj.nation().is_null() {
        return read_f64(rdata::F64_3_5);
    }
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cj));
    let mut score: f64;
    covered!("mkd_7 job interest");
    if c == cj {
        score = 3.5;
    } else {
        if !k.is_null()
            && !cj.is_null()
            && !cj.nation().is_null()
            && !c.nation().is_null()
            && c.nation().first_language() == cj.nation().first_language()
            && k.grievances() & 0x20 != 0
        {
            return zero;
        }
        score = if job == 5 || job == 0xc { 2.25 } else { 1.5 }; // mkd7: the base for another club
        if cj.is_null() {
            // out of a club: (days since the later of the game's start and joining)² / (reputation + 900), at most 12
            let (gd, gy) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
            let (td, ty) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
            if !(gy > ty || (gy == ty && gd > td)) {
                let mut l = [0u8; 8];
                core::ptr::copy_nonoverlapping((p.addr() + staff::DATE_JOINED_CLUB) as *const u8, l.as_mut_ptr(), 8);
                let (ld, ly) = (i16::from_le_bytes([l[0], l[1]]), u16::from_le_bytes([l[2], l[3]]));
                if ly < gy || (ly == gy && ld < gd) {
                    let mut tmp = [0u8; 8];
                    exe::date_operator_3d(l.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, G_GAME_START_DATE);
                }
                let (ld, ly) = (i16::from_le_bytes([l[0], l[1]]), u16::from_le_bytes([l[2], l[3]]));
                let mut dd = (read_u16(G_CURRENT_DATE_YEAR) as i32 - ly as i32) * 365 - ld as i32 + read_i16(G_CURRENT_DATE) as i32;
                if dd > 0x1f4 {
                    dd = 0x1f4;
                }
                let mut x = dd.wrapping_mul(dd) as f64 / (p.non_player().current_reputation() as f64 + read_f64(job_interest_rdata::F64_900));
                if read_f64(rdata::F64_12) < x {
                    x = read_f64(rdata::F64_12);
                }
                score = x + score;
            }
        }
    }
    // liking and disliking the clubs
    if exe::staff_dislikes_club(p, c) != 0 {
        score *= read_f64(rdata::F64_0_33);
        fl(2);
    }
    if exe::staff_dislikes_club(p, cj) != 0 {
        score = score + score;
        fl(4);
    }
    if exe::staff_favours_club(p, cj) != 0 {
        score *= read_f64(if cj.professional_status() != 1 && c.professional_status() == 1 { rdata::F64_0_85 } else { rdata::F64_0_5 });
        if !reasons.is_null() && !k.is_null() && k.leaving_on_bosman() == 0 {
            *reasons |= 8;
        }
    }
    if exe::staff_favours_club(p, c) != 0 {
        fl(0x10);
        score *= read_f64(rdata::F64_3);
    }
    // rivals
    if !cj.is_null() {
        let rival = read_usize(cj.addr() + club::RIVAL1) == c.addr() || read_usize(cj.addr() + club::RIVAL2) == c.addr() || read_usize(cj.addr() + club::RIVAL3) == c.addr();
        if c != cj && (rival || exe::comp_util_get_league_clash_type(c, cj) != 0) {
            covered!("mkd_7 rival");
            let loy = p.loyalty() as i32 as f64;
            let (e1, e2) = (read_u32(G_KEY_CLUB_GLASGOW_CELTIC), read_u32(G_KEY_CLUB_GLASGOW_RANGERS));
            let (ic, ij) = (read_u32(c.addr()), read_u32(cj.addr()));
            let m = if (ic == e1 || ic == e2) && (ij == e1 || ij == e2) {
                read_f64(rdata::F64_0_5) - loy * read_f64(rdata::F64_0_025)
            } else if read_u32(c.nation().addr()) == read_u32(KEY_NATION_ITALY) {
                read_f64(rdata::F64_1) - loy * read_f64(rdata::F64_0_01666666667)
            } else if rival && read_usize(c.addr() + club::DIVISION) == read_usize(cj.addr() + club::DIVISION) {
                if (c.reputation() as i32 - cj.reputation() as i32).abs() < 0x4e2 {
                    read_f64(rdata::F64_0_75) - loy * read_f64(rdata::F64_0_025)
                } else {
                    read_f64(rdata::F64_0_8) - loy * read_f64(rdata::F64_0_01666666667)
                }
            } else {
                read_f64(rdata::F64_0_85) - loy * read_f64(rdata::F64_0_025)
            };
            score *= m;
            if !reasons.is_null() && m < read_f64(rdata::F64_0_7) {
                *reasons |= 0x20;
            }
        } else {
            let (yu, cr) = (read_u32(KEY_NATION_YUGOSLAVIA), read_u32(KEY_NATION_CROATIA));
            let nc = read_u32(c.nation().addr());
            let nj_ = read_u32(cj.nation().addr());
            let np_ = read_u32(p.nation().addr());
            if nc == yu {
                if nj_ == cr && np_ == cr {
                    score *= read_f64(rdata::F64_0_05);
                    fl(2);
                }
            } else if nc == cr && nj_ == yu && np_ == yu {
                score *= read_f64(rdata::F64_0_1);
                fl(2);
            }
        }
    }
    // the managers' liking
    if job != 5 && job != 0xc {
        let m = c.manager();
        if exe::staff_is_favoured_by(m, p) != 0 {
            score = score + score;
            fl(0x40);
        }
        if exe::staff_is_disliked_by(m, p) != 0 {
            score *= read_f64(rdata::F64_0_33);
            fl(0x80);
        }
    }
    if !cj.is_null() {
        let m = cj.manager();
        if exe::staff_is_disliked_by(m, p) != 0 {
            score = score + score;
            fl(0x100);
        }
        if exe::staff_is_favoured_by(m, p) != 0 {
            score *= read_f64(rdata::F64_0_5);
            fl(0x200);
        }
    }
    if !p.player().is_null() && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        fl(0x10000000);
        return zero;
    }
    // the morale in the contract
    if !k.is_null() {
        let m44 = k.morale() as i32 as f64;
        if c == cj {
            score = (m44 * read_f64(rdata::F64_0_0025) + read_f64(rdata::F64_1)) * score;
        } else {
            let v = read_f64(rdata::F64_1) - m44 * read_f64(rdata::F64_0_0025);
            score = v * score;
            if !reasons.is_null() && k.transfer_status() & 8 == 0 {
                if v > read_f64(rdata::F64_1_5) {
                    *reasons |= 0x400;
                } else if v < read_f64(rdata::F64_0_5) {
                    *reasons |= 0x800;
                }
            }
        }
    }
    if c == cj {
        jb = job;
    }
    let jb_manager = jb == 5 || jb == 0xc;
    let job_manager = job == 5 || job == 0xc;
    let mut devsq_new: i8 = 0;
    let mut e_new: i16 = 0;
    let mut div_new: i16 = 0;
    let mut rec_new = 0usize; // [esp+0x78]
    let mut l7c = 1.0f64; // [esp+0x7c]: how long he has been at the club (0.0025 × days² / 20 / (loyalty + 6), at most 1)
    if c != cj {
        // by the job he has now
        match jb {
            1 => score *= read_f64(rdata::F64_0_01),
            2..=4 => {
                if job == 5 && c.reputation() >= 0x1770 {
                    score *= read_f64(rdata::F64_0_5);
                } else {
                    score *= read_f64(rdata::F64_0_01);
                }
            }
            5 | 0xc => {
                if !cj.is_null() {
                    score *= read_f64(rdata::F64_0_45);
                    fl(0x4000000);
                } else {
                    score *= read_f64(rdata::F64_0_65);
                }
            }
            6 | 0xd => {
                score *= read_f64(rdata::F64_1_5);
                if job_manager {
                    fl(0x8000000);
                }
            }
            7 | 0xe => {
                score *= read_f64(rdata::F64_2_2);
                if job_manager {
                    fl(0x8000000);
                }
            }
            8 | 0xf => {
                score = (amb * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_8)) * score;
                if job_manager {
                    fl(0x8000000);
                }
            }
            9 | 10 => {
                if matches!(job, 5 | 0xc | 6 | 0xd | 0xe | 0xf | 7 | 8) {
                    fl(0x8000000);
                }
            }
            0x10 => {
                let a = p.ambition();
                score = (read_f64(rdata::F64_10) / (if a > 1 { a as i32 } else { 1 }) as f64) * score;
            }
            _ => {
                score *= read_f64(rdata::F64_10); // mkd7: a player taking a staff job
                fl(0x8000000);
            }
        }
        // by the job offered
        match job {
            0 => return zero,
            1 => score *= read_f64(rdata::F64_10),
            2..=4 => score *= read_f64(job_interest_rdata::F64_1_176470588),
            5 | 0xc => {
                score *= read_f64(job_interest_rdata::F64_2_222222222);
                if jb_manager {
                    score *= read_f64(rdata::F64_1_5);
                }
            }
            6 | 0xd => {
                score *= read_f64(rdata::F64_0_5);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            7 | 8 | 0xe | 0xf => {
                score *= read_f64(rdata::F64_0_4);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            9 | 10 => {
                score *= read_f64(if jb == 9 || jb == 10 { rdata::F64_0_4 } else { rdata::F64_0_25 });
                if !p.player().is_null() {
                    score *= read_f64(rdata::F64_0_25);
                }
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            _ => {
                score *= read_f64(rdata::F64_0_1);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
        }
        // a manager the board has little confidence in
        if !cj.is_null() && jb_manager && read_usize(G_MANAGER_MANAGER) != 0 {
            let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_u32(cj.addr()) as usize).wrapping_mul(0x49));
            let v = read_i16(t + 8) as i32 + (p.loyalty() as i32 + p.determination() as i32) * 500 + read_i16(t + 6) as i32;
            if v < 0x1f40 {
                score *= read_f64(rdata::F64_1_4);
            }
        }
        // the new club's reputation on his scale, and the language
        devsq_new = job_interest_development_squared(c.nation());
        let (e, dv, rec) = job_interest_club_scale(this, p, c, job, w, false);
        e_new = e;
        div_new = dv;
        rec_new = rec;
        if !c.nation().is_null() {
            let m = job_interest_language_factor(p, c, e_new);
            score = m * score;
            if !reasons.is_null() && m < read_f64(rdata::F64_0_6) {
                *reasons |= 0x10000;
            }
        }
        // a foreigner: by the job offered (milder when not a foreigner to the club's manager)
        let applies = if !cj.is_null() {
            let n = c.nation();
            !(!n.is_null() && exe::database_is_staff_foreign(p, n, 1) == 0) && cj.nation() != c.nation()
        } else {
            true
        };
        if applies {
            let m = c.manager();
            let mild = !job_manager && !m.is_null() && !m.nation().is_null() && exe::database_is_staff_foreign(p, m.nation(), 1) == 0;
            let f = if mild {
                match job {
                    2..=4 => rdata::F64_0_85,
                    6 | 0xd => rdata::F64_0_75,
                    7 | 8 | 0xe | 0xf => rdata::F64_0_65,
                    9 | 10 => rdata::F64_0_5,
                    _ => rdata::F64_0_01,
                }
            } else {
                covered!("mkd_7 foreigner");
                match job {
                    2..=4 => rdata::F64_0_25,
                    5 | 0xc => rdata::F64_0_3,
                    6 | 0xd => rdata::F64_0_225,
                    7..=10 | 0xe | 0xf => rdata::F64_0_2,
                    _ => rdata::F64_0_001,
                }
            };
            score *= read_f64(f);
        }
        if cj.is_null() {
            score *= read_f64(rdata::F64_2_5);
        } else {
            // the time at his club: 0 below (loyalty + 8) × 15 days
            let dd = (read_u16(G_CURRENT_DATE_YEAR) as i32 - p.date_joined_club_year() as i32) * 365 - read_i16(p.addr() + staff::DATE_JOINED_CLUB) as i32
                + read_i16(G_CURRENT_DATE) as i32;
            let loy = p.loyalty() as i32;
            if dd < (loy + 8) * 0xf {
                covered!("mkd_7 too soon at his club");
                return zero;
            }
            let q = dd.wrapping_mul(dd) / 20;
            let mut r = q / (if loy + 6 < 1 { 1 } else { loy + 6 });
            if r < 0 {
                return zero;
            }
            if r > 0x190 {
                r = 0x190;
            } else if r == 0 {
                return zero;
            }
            l7c = r as f64 * read_f64(rdata::F64_0_0025);
        }
    }
    // his current club's reputation (or his own) on the same scale
    let devsq_cur: i8;
    let e_cur: i16;
    let mut div_cur: i16 = 0;
    let mut rec_cur = 0usize; // [esp+0x70]
    if cj.is_null() {
        devsq_cur = job_interest_development_squared(p.nation());
        let np = p.non_player();
        let avg = ((np.world_reputation() as i32 + np.current_reputation() as i32 + np.home_reputation() as i32) / 3) as i16;
        e_cur = ftol(avg as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16;
    } else {
        devsq_cur = job_interest_development_squared(cj.nation());
        let (e, dv, rec) = job_interest_club_scale(this, p, cj, job, w, true);
        e_cur = e;
        div_cur = dv;
        rec_cur = rec;
        if c != cj {
            score /= job_interest_language_factor(p, cj, e_cur);
        }
    }
    if c != cj {
        let d = c.division();
        if d.is_null() || read_u32(d.addr()) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            score *= read_f64(rdata::F64_0_1);
        }
        let np = p.non_player();
        let avg = (np.world_reputation() as i32 + np.current_reputation() as i32 + np.home_reputation() as i32) / 3;
        let g = (c.reputation() as i32 - avg) as f64 * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1);
        // the difference of the two scales, by ambition (up) or by the job (down)
        let diff = (e_new as i32 - e_cur as i32) as f64;
        let mut y = if e_new > e_cur {
            diff * (amb * read_f64(rdata::F64_0_08771929825) + read_f64(rdata::F64_0_75))
        } else if !job_manager {
            diff * read_f64(rdata::F64_1_5)
        } else if !(score > read_f64(rdata::F64_10)) {
            diff
        } else {
            let mut k = read_f64(rdata::F64_1_1) - score * read_f64(rdata::F64_0_01);
            if k < read_f64(rdata::F64_0_1) {
                k = read_f64(rdata::F64_0_1);
            } else if k > read_f64(rdata::F64_1) {
                k = read_f64(rdata::F64_1);
            }
            k * diff
        };
        if y > zero && l7c < read_f64(rdata::F64_1) {
            let mut z = y * read_f64(rdata::F64_0_02);
            if read_f64(rdata::F64_100) < z {
                z = read_f64(rdata::F64_100);
            }
            let f = ((read_f64(rdata::F64_150) - z) * l7c + z) * read_f64(rdata::F64_0_006666666667);
            score *= f;
            if !reasons.is_null() && f < read_f64(rdata::F64_0_6) {
                *reasons |= 0x100000;
            }
        }
        if !jb_manager && job_manager && y < zero {
            let k = if jb == 8 || jb == 0xf || jb == 6 {
                let k = read_f64(rdata::F64_1) - score * read_f64(rdata::F64_0_04);
                if k < read_f64(rdata::F64_0_4) { read_f64(rdata::F64_0_4) } else { k }
            } else {
                let k = read_f64(rdata::F64_1) - score * read_f64(rdata::F64_0_1);
                if k < read_f64(rdata::F64_0_2) { read_f64(rdata::F64_0_2) } else { k }
            };
            y *= k;
        }
        let mut t = y * read_f64(job_interest_rdata::F64_0_0002222222222) + read_f64(rdata::F64_1); // mkd7: the scales' difference
        if t < read_f64(rdata::F64_0_001) {
            t = 0.001;
        }
        score *= t;
        if !reasons.is_null() {
            let bosman_ok = k.is_null() || k.leaving_on_bosman() == 0;
            let ambitious = w60 as f64 > read_f64(rdata::F64_1_2);
            if t < read_f64(rdata::F64_0_6) {
                if ambitious && rec_cur != 0 && rec_new == 0 && score > read_f64(rdata::F64_1_25) && jb_manager && bosman_ok {
                    *reasons |= 0x400000;
                } else {
                    *reasons |= 0x800000;
                }
            } else if t > read_f64(rdata::F64_1_5) && ambitious && rec_cur == 0 && rec_new != 0 && job_manager && bosman_ok {
                *reasons |= 0x200000;
            }
        }
        // the development of the two nations and the continents
        if !cj.is_null() {
            if devsq_cur == devsq_new {
                let pc = p.nation().continent();
                if pc != 0 && read_u32(pc) == read_u32(KEY_CONTINENT_EUROPE) {
                    let cc = c.nation().continent();
                    if cc != 0 && read_u32(cc) != read_u32(KEY_CONTINENT_EUROPE) && cc != cj.nation().continent() {
                        score *= read_f64(rdata::F64_0_4);
                        fl(0x2000000);
                    }
                }
            } else {
                let dc = if div_cur > 0x1f4 { div_cur as i32 } else { 0x1f4 };
                let mut r = (devsq_cur as i32 as f64 * div_new as f64) / (dc as f64 * devsq_new as i32 as f64);
                if !(read_f64(rdata::F64_0_01) <= r) {
                    r = read_f64(rdata::F64_0_01);
                } else if read_f64(rdata::F64_10) < r {
                    r = read_f64(rdata::F64_10);
                }
                score = r * score;
                if c.nation().continent() != cj.nation().continent() && r < read_f64(rdata::F64_1) {
                    score *= read_f64(rdata::F64_0_2);
                    fl(0x2000000);
                }
            }
        }
        score *= g;
        if !reasons.is_null() && g < read_f64(rdata::F64_0_6) {
            *reasons |= 0x800000;
        }
    }
    // the wage: the default wage × 3.5 / will against the club's staff cap and his current wage
    if check != 0 && !(score < read_f64(rdata::F64_1_75)) {
        covered!("mkd_7 wage check");
        let mut local = [0u8; 0x50];
        exe::new_staff_contract(local.as_mut_ptr() as usize);
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
        if fin == 0 {
            return zero;
        }
        let wage = exe::staff_contract_default_staff_contract(local.as_mut_ptr() as usize, p, c, 0, u32::MAX, job_arg, 0);
        let m = if read_f64(rdata::F64_0_1) <= score { score } else { read_f64(rdata::F64_0_1) };
        let mut q = read_f64(rdata::F64_3_5) / m;
        if q < read_f64(rdata::F64_0_9) {
            q = 0.9;
        }
        let asked = q * wage;
        let mw = exe::finance_get_max_staff_wage(fin, job) as f64;
        let (s2, wanted) = if !(asked <= mw) && !(asked <= zero) { ((mw / asked) * score, mw) } else { (score, asked) };
        if k.is_null() {
            return s2;
        }
        let mut cw = k.wage() as f64;
        if cw < read_f64(rdata::F64_10) {
            cw = read_f64(rdata::F64_10);
        }
        let ratio = wanted / cw;
        if !(read_f64(rdata::F64_0_1) <= ratio) {
            return s2;
        }
        if read_f64(rdata::F64_10) < ratio {
            return (read_f64(rdata::F64_10) / q) * s2;
        }
        if ratio < read_f64(rdata::F64_1) {
            return s2;
        }
        return (ratio / q) * s2;
    }
    score
}

/// Comparer for `staff_interest_in_job_at_club`: original and copy side by side over the reasons,
/// `G_CONTRACT_LOOKUP_RESULT` (written by the default wage) and the generator's state.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_staff_interest_in_job_at_club(this: usize, person: usize, club: usize, check: u32, job: u32, reasons: *mut u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> f64>(IDX_STAFF_INTEREST_IN_JOB_AT_CLUB);
    let est = crate::rng::STATE_REGIONS;
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    if !reasons.is_null() {
        regions.push((reasons as usize, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(this, person, club, check, job, reasons),
        || crate::rng::without_counting(|| staff_interest_in_job_at_club(this, person, club, check, job, reasons)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_interest_in_job_at_club(person {:#x}, club {:#x}, {}, job {}) original={} rust={} {:x?} x {:x?}",
        person, club, check, job as u8, r, n, mo, mr));
    r
}
