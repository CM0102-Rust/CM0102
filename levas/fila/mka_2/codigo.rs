// ---------- batch %%LEVA%%: the asking price for a player (contract_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16, u32, u32) -> i32>(%%I0%%);
    let mut reg = player_value_regions().to_vec();
    if !reasons.is_null() {
        reg.push((reasons as usize, 2));
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, seller, buyer, player, reasons, adjust, flag),
        || crate::rng::without_counting(|| calc_asking_price(this, seller, buyer, player, reasons, adjust, flag)));
    SCOREBOARD.record(equal && r == n, || format!("calc_asking_price(seller {:#x}, buyer {:#x}, player {:#x}, adjust {}, flag {}) original={} rust={} {:x?} x {:x?}", seller, buyer, player, adjust as u8, flag, r, n, mo, mr));
    r
}
