// ---------- batch %%LEVA%%: the money side of the market: the club's transfer budget and the asking price of a human club's player (fin_prm.cpp, contract_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut u32, u32, u32) -> i64>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32>(%%I1%%);
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
