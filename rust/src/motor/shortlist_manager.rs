use super::*;

/// Returns the entry of person `pessoa`'s shortlist (40 entries of 37 bytes at [this+0xdc] + id x 9) whose +0xf (or,
/// with `tambem_13`, +0x13) is the id of `alvo`; with `nao_11` only entries of type (+0) other than 11 match, without
/// it only type 11. None: 0.
pub(crate) unsafe extern "thiscall" fn shortlist_entry(this: usize, person: usize, target: usize, not_11: u32, tambem_13: u32) -> usize {
    let list = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_i32(person) as usize).wrapping_mul(9));
    let base = read_usize(list);
    let id = read_u32(target);
    for k in 0..40usize {
        let e = ShortlistSlot(base.wrapping_add(k * 37));
        let home = (e.target_staff_id() as u32) == id || (tambem_13 != 0 && e.replacement_staff_id() == id);
        if home && (not_11 != 0) == (read_u8(e.addr()) != 11) {
            return e.addr();
        }
    }
    0
}

/// CONTRACT_MANAGER get_contract (`exe::contract_manager_get_contract`) for a person, also stored in G_CONTRACT_LOOKUP_RESULT.
pub(crate) unsafe fn contract_of_staff_cached(person: Staff) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// Club of the person's contract (via the cached lookup), or 0 without contract.
pub(crate) unsafe fn club_of_staff_contract(person: Staff) -> usize {
    let c = Contract(contract_of_staff_cached(person));
    if c.is_null() { 0 } else { (transfer_manager_club_by_id(c.club())).addr() }
}

/// Byte +0xb of the person's 79-byte staff state record (G_STAFF_STATE_TABLE).
pub(crate) unsafe fn staff_state_byte_0b(person: Staff) -> u8 {
    read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS)
}

/// The AI's bids and loan requests for a computer club with an active shortlist: in each slot with a target and no
/// offer it checks the role, contract, reach and offer rules; in a purchase slot it values the slot and bids if
/// random(value) beats random(2 × reputation + 1000); in a loan slot it checks whether the loan is possible and makes
/// the request if random(`shortlist_slot::PRICE`) beats random(reputation / 10) or random(2 × reputation).
///
/// Convention: thiscall, ret 4, `shortlist_manager.cpp`, SEH frame: local STAFF_CONTRACT and offer, only the offer with
/// a destructor. `Shortlist_Manager::make_club_bids(club)`, from the day loop (`0x5cccd0`). Today + 7
/// (`exe::date_add_days`) and a local STAFF_CONTRACT (`exe::new_staff_contract`). National team
/// (`exe::database_is_national_team`), human club (`exe::human_manager_is_human_club` with 0, 1), or a human on holiday
/// (with 1, 1) whose manager responds (`FN_HUMAN_MANAGER_IS_ON_HOLIDAY`) with rule 4
/// (`exe::human_manager_get_holiday_option`) equal to 1: nothing. The club's list (the 9-byte record in
/// `shortlist_manager_fields::CLUB_SHORTLISTS`): active (`club_shortlist::ACTIVE` = 1), with fewer than 100 days
/// (`club_shortlist::DAYS`) and finances (`exe::financetable_get_club_finance`). The count = slots with a player bid
/// (`exe::shortlist_manager_count_slots_with_bids` with `0xb`) + the squad (`exe::database_count_club_squad` with 0,
/// 0).
///
/// Each of the 40 slots of `0x25` bytes with a target (`shortlist_slot::TARGET_STAFF_ID`) and no offer
/// (`shortlist_slot::OFFER_REF` = −1), except player slots when the count is at 50: the target outside the club; in
/// slots for another role, role 8/9/10 with a free place in lists `+0x19f` (5), `+0x1b3` (7), `+0x1cf` (3) and role 6/7
/// with no `.assistant_manager()`; without bits `0x30` of `+0xb` of the 79-byte record (`staff_state_byte_0b`; player
/// slot), no future transfer (`exe::transfer_manager_get_future_transfer`), agreed contract
/// (`exe::transfer_manager_get_agreed_contract_club`) or a national team manager job (`.national_job()` with
/// `.job_for_nation()` = `job_kind::MANAGER`).
///
/// Purchase slot (`shortlist_slot::FLAGS` bit 0, `shortlist_slot_flags::LOAN_SLOT`, clear): position (`+1`) not
/// negative in a player slot; the club can reach him (`exe::transfer_manager_can_club_bid_for_player`, or reason 9); no
/// offer from the club for him (`exe::transfer_manager_club_has_offer_for_player`) and no exchange
/// (`exe::transfer_manager_get_exchange_offers_for_player`). The value = `shortlist_slot::PRICE` (doubled with bit
/// `shortlist_slot_flags::BID_MARKER`, at least 20000); with the league (`.division()`) active in
/// `[G_COMP_OBJECT_TABLE]`, × 3 or × 2 by reputation (`.reputation()` above `0x128e`) and the count below 25/35, or × 2
/// below 25. Player slot with a player to replace (`shortlist_slot::REPLACEMENT_STAFF_ID`): unable to sign
/// (`exe::transfer_manager_can_sign_player` with today + 7, 1; except reason 9) adds `shortlist_slot::PRICE` (at least
/// 10000); the injury of the player to replace (`exe::injury_get_player_injury` on `[G_INJURY]`, three times) adds
/// days² (`injury_entry::DAYS_LEFT`) × (reputation / 500); with a player sheet, (1.5 − the player's morale,
/// `.player_morale()` of the sheet, / 30) × value; under offer (`exe::transfer_manager_is_player_under_offer`) adds
/// max(`shortlist_slot::PRICE` × 1.5, 10000). Player slot with a human seller (or the target already at the club): the
/// desired contract (`exe::staff_contract_set_to_desired_contract`) 0 skips; wage (`.wage()`) above 1.3 × the cap
/// (`exe::finance_get_max_wage`) skips.
///
/// The random draws (`exe::utils_random`): random(value) above random(2 × reputation + 1000) makes the bid:
/// `shortlist_slot::FLAGS` bit 5 (`shortlist_slot_flags::BID_MARKER`) cleared, the local offer
/// (`exe::new_transfer_offer`) filled (`exe::transfer_offer_build_bid`); with no offer, the status in the list
/// (`exe::shortlist_manager_update_staff_status` with 1); with one, the bid
/// (`exe::transfer_manager_add_transfer_offer`) and, with terms, the contract offer
/// (`exe::contract_manager_add_contract_offer`) and, with an AI seller with no contract offer
/// (`exe::contract_manager_find_contract_offer`) and with the target's contract
/// (`exe::contract_manager_get_contract_for_club`), the renewal negotiation at the seller
/// (`FN_CONTRACT_MANAGER_OFFER_CONTRACT` with the factor by squad status 1..5 and the seller's entry
/// `exe::shortlist_manager_find_club_shortlist_entry`) and the terms freed; `shortlist_slot::OFFER_REF` = the offer
/// index (`.slot_index()`) and the count goes up for a player slot. The local offer is destroyed
/// (`exe::drop_transfer_offer`).
///
/// Loan slot (bit 0, `shortlist_slot_flags::LOAN_SLOT`): not a French club (`[KEY_NATION_FRANCE]`) with
/// `.professional_status()` = `professional_status::AMATEUR`; the loan possible
/// (`exe::transfer_manager_can_loan_player`); a human seller without bit 2 (`transfer_status::LOAN_LISTED`) of the
/// contract's `contract::TRANSFER_STATUS` skips; no offer (`exe::transfer_manager_club_has_offer_for_player`) and no
/// exchange; random(`shortlist_slot::PRICE`) above random(reputation / 10), or above random(2 × reputation) on the
/// second pair: the request (`exe::shortlist_manager_try_loan_offer_for_player`) in `shortlist_slot::OFFER_REF`;
/// negative, the status in the list; otherwise the count goes up.
/// Original: `Shortlist_Manager::make_club_bids` (0x84a020, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn make_club_bids(this: usize, club: usize) {
    let club = Club(club);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    let mut week = [0u32; 2];
    exe::date_add_days(G_CURRENT_DATE, (&mut week) as *mut [u32; 2] as usize, 7);
    let pdata = week.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    exe::new_staff_contract(contrato0.as_mut_ptr() as usize);
    if exe::database_is_national_team(club) != 0 || exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && responde(read_usize(G_HUMAN_MANAGER), exe::database_get_club_manager(club)) != 0
        && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 4, exe::database_get_club_manager(club)) == 1
    {
        return;
    }
    let reg = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(club.addr()).wrapping_mul(9));
    if read_u8(reg + club_shortlist::ACTIVE) != 1 || read_u8(reg + club_shortlist::DAYS) >= 100 || exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) == 0 {
        return;
    }
    let mut count = exe::shortlist_manager_count_slots_with_bids(this, club, 0xb) as i8;
    count = count.wrapping_add(exe::database_count_club_squad(club, 0, 0) as i8);
    // simplification: the reason (a local word) is not initialized by the original; 0x8ce990 and 0x8ced40 write it before it is read
    let mut reason: u16 = 0;
    let gm = read_usize(G_TRANSFER_MANAGER);
    for s in 0..0x28usize {
        let off = s * 0x25;
        let slot = |k: usize| read_usize(reg).wrapping_add(off) + k;
        let player = read_u8(slot(0)) == 0xb;
        if count >= 0x32 && player {
            continue;
        }
        let id = read_usize(slot(shortlist_slot::TARGET_STAFF_ID)) as i32;
        if id <= -1 || read_usize(slot(shortlist_slot::OFFER_REF)) as i32 != -1 {
            continue;
        }
        let p = Staff((transfer_manager_staff_by_id(id as usize)).addr());
        if club_of_staff_contract(p) == club.addr() {
            continue;
        }
        if !player {
            let free = |base: usize, n: i8| {
                let mut k: i8 = 0;
                while k < n {
                    if read_usize(club.addr() + base + 4 * k as usize) == 0 {
                        break;
                    }
                    k += 1;
                }
                k != n
            };
            match read_u8(slot(0)) {
                8 if !free(0x19f, 5) => continue,
                9 if !free(0x1b3, 7) => continue,
                10 if !free(0x1cf, 3) => continue,
                6 | 7 if club.assistant_manager() != 0 => continue,
                _ => {}
            }
        }
        if staff_state_byte_0b(p) & 0x30 != 0 && player {
            continue;
        }
        if exe::transfer_manager_get_future_transfer(gm, p, 0 as u32) != 0 || exe::transfer_manager_get_agreed_contract_club(gm, p, 0) != 0 {
            continue;
        }
        if !p.national_job().is_null() && p.job_for_nation() == job_kind::MANAGER {
            continue;
        }
        if read_u8(slot(shortlist_slot::FLAGS)) & shortlist_slot_flags::LOAN_SLOT == 0 {
            // the purchase slot
            if read_i16(slot(1)) <= -1 && player {
                continue;
            }
            if exe::transfer_manager_can_club_bid_for_player(gm, p, club, &mut reason, 0) == 0 && reason != 9 {
                continue;
            }
            if exe::transfer_manager_club_has_offer_for_player(gm, p, club) != 0 || exe::transfer_manager_get_exchange_offers_for_player(gm, p, 0) != 0 {
                continue;
            }
            let mut v = read_usize(slot(shortlist_slot::PRICE)) as i32;
            if read_u8(slot(shortlist_slot::FLAGS)) & shortlist_slot_flags::BID_MARKER != 0 {
                v = v.wrapping_shl(1);
                if v <= 0x4e20 {
                    v = 0x4e20;
                }
            }
            let league = club.division().addr();
            if league != 0 {
                let k = read_usize(league) as i32;
                if k > -1 && k < read_usize(G_CLUB_COMP_COUNT) as i32 && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(k as usize * 4)) != 0 {
                    if club.reputation() > 0x128e {
                        if count < 0x19 {
                            v = v.wrapping_mul(3);
                        } else if count < 0x23 {
                            v = v.wrapping_mul(2);
                        }
                    } else if count < 0x19 {
                        covered!("cqe_3 double by league");
                        v = v.wrapping_mul(2); // cqe3: double for the active league
                    }
                }
            }
            if read_usize(slot(shortlist_slot::REPLACEMENT_STAFF_ID)) as i32 > -1 && read_u8(slot(0)) == 0xb {
                let current = club_of_staff_contract(p);
                if exe::transfer_manager_can_sign_player(gm, club, pdata, p, current, 1, &mut reason) == 0 && reason != 9 {
                    let x = read_usize(slot(shortlist_slot::PRICE)) as i32;
                    v = v.wrapping_add(if x > 0x2710 { x } else { 0x2710 });
                }
                let sub = Staff((transfer_manager_staff_by_id(read_usize(slot(shortlist_slot::REPLACEMENT_STAFF_ID)))).addr());
                let les = read_usize(G_INJURY);
                if exe::injury_get_player_injury(les, sub) != 0 {
                    let a = read_i16(exe::injury_get_player_injury(les, sub) + injury_entry::DAYS_LEFT) as i32;
                    let b = read_i16(exe::injury_get_player_injury(les, sub) + injury_entry::DAYS_LEFT) as i32;
                    v = v.wrapping_add(a.wrapping_mul(b).wrapping_mul(club.reputation() as i32 / 500));
                }
                let f = sub.player();
                if !f.is_null() {
                    let x = (f.player_morale() as u8) as i8 as f64;
                    v = v.wrapping_add(ftol((read_f64(rdata::F64_1_5) - x * read_f64(rdata::F64_0_03333333333)) * v as f64));
                }
                if exe::transfer_manager_is_player_under_offer(gm, sub, 0) != 0 {
                    let x = read_usize(slot(shortlist_slot::PRICE)) as i32 as f64 * read_f64(rdata::F64_1_5);
                    v = v.wrapping_add(ftol(if x > read_f64(rdata::F64_10000) { x } else { read_f64(rdata::F64_10000) }));
                }
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && transfer_manager_club_by_id(read_usize(c + contract::CLUB)) != Club(0) && player {
                let seller = club_of_staff_contract(p);
                if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 || p.club_job() == club {
                    let mut contract = [0u8; 0x50];
                    let pc = Contract(contract.as_mut_ptr() as usize);
                    exe::new_staff_contract(pc);
                    let e = slot(0);
                    // simplification: the status goes with the high bytes of the slot pointer (as in the original); the role, with those of the
                    // edx left by 0x600a50 (garbage): 0x85fc50 receives a char, only the byte counts
                    let st = (e as u32 & 0xffff_ff00) | ((read_u8(e + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
                    let r = exe::staff_contract_set_to_desired_contract(pc, p, club, (u32::MAX) as u32 as i32, st, read_u8(e) as u32, 0, 0);
                    if r as f64 == read_f64(rdata::F64_0) {
                        continue;
                    }
                    // simplification: the role goes with the high bytes of the result and of fnstsw; 0x5968d0 reads only the byte
                    let t = exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), p, ((read_u8(slot(shortlist_slot::FLAGS)) >> 1) & 0xf) as u32);
                    if (t as f64 * read_f64(rdata::F64_1_3)) < (pc.wage() as usize) as i32 as f64 {
                        continue;
                    }
                }
            }
            covered!("cqe_3 bid draw");
            let r1 = exe::utils_random((club.reputation() as i32).wrapping_mul(2).wrapping_add(1000)); // cqe3: the bid random draw
            let r2 = exe::utils_random(v);
            if r2 <= r1 {
                continue;
            }
            write_u8(slot(shortlist_slot::FLAGS), read_u8(slot(shortlist_slot::FLAGS)) & !shortlist_slot_flags::BID_MARKER);
            let mut offer = [0u8; 0x44];
            let po = Offer(offer.as_mut_ptr() as usize);
            exe::new_transfer_offer(po);
            let mut terms: usize = 0;
            if exe::transfer_offer_build_bid(po, p, club, slot(0), read_usize(slot(7)) as u32, 0, &mut terms) == 0 {
                ((exe::shortlist_manager_update_staff_status(this, p, club, 1, (read_u8(slot(0)) != 0xb) as u32) as usize) as u8);
            } else {
                exe::transfer_manager_add_transfer_offer(gm, (po.addr()) as usize as *mut u8, (0) as u32 as *mut u8);
                if terms != 0 {
                    // simplification: the job goes with the high bytes of the slot vector pointer (as in the original)
                    let job = (read_usize(reg) as u32 & 0xffff_ff00) | read_u8(slot(0)) as u32;
                    exe::contract_manager_add_contract_offer(read_usize(G_CONTRACT_MANAGER), (terms) as usize as *mut u8, (po.slot_index() as i32) as i32 as usize, (po.serial() as u32) as u32 as usize, job, 0, 1, 0);
                    let c = contract_of_staff_cached(p);
                    let seller = if c == 0 { 0 } else { (transfer_manager_club_by_id(read_usize(c + contract::CLUB))).addr() };
                    if seller != 0 && exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0
                        && exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), (read_usize(p.addr()) as i32) as i32 as u32, (read_usize(seller) as i32) as i32 as u32) == 0
                    {
                        let cv = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, seller));
                        if !cv.is_null() {
                            let e = (cv.squad_status() >> 4) as i8;
                            let factor = match e {
                                1 | 2 if exe::shortlist_manager_find_club_shortlist_entry(this, seller, p, 0, 0) != 0 => {
                                    let rep = club.reputation();
                                    let div = if rep < 1 { 1 } else { rep as i32 };
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64(rdata::F64_0_1) + rep as i32 as f64 / div as f64;
                                    Some(if read_f64(rdata::F64_1) <= f { if read_f64(rdata::F64_1_4) < f { 1.4 } else { f } } else { 1.0 })
                                }
                                3 | 4 | 5 if exe::shortlist_manager_find_club_shortlist_entry(this, seller, p, 0, 0) != 0 => {
                                    let rep = club.reputation() as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64(rdata::F64_0_1) + rep as f64 / div as f64 + read_f64(rdata::F64_0_5);
                                    Some(if read_f64(rdata::F64_1) <= f { if read_f64(rdata::F64_1_2) < f { 1.2 } else { f } } else { 1.0 })
                                }
                                _ => None,
                            };
                            if let Some(f) = factor {
                                let b = f.to_bits();
                                renovar(read_usize(G_CONTRACT_MANAGER), p.addr(), 0, 0, u32::MAX, b as u32, (b >> 32) as u32, 2);
                            }
                        }
                    }
                    exe::crt_operator_delete(terms);
                }
                write_usize(slot(shortlist_slot::OFFER_REF), po.slot_index());
                if player {
                    count = count.wrapping_add(1);
                }
            }
            exe::drop_transfer_offer(po);
        } else {
            // the loan slot
            let n = club.nation().addr();
            if n != 0 && read_usize(n) == read_usize(KEY_NATION_FRANCE) && club.professional_status() == professional_status::AMATEUR {
                continue;
            }
            if exe::transfer_manager_can_loan_player(gm, club, p, 0) == 0 {
                continue;
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && transfer_manager_club_by_id(read_usize(c + contract::CLUB)) != Club(0) && exe::human_manager_is_human_club(club_of_staff_contract(p), 0, 1, 0, 0) != 0 {
                let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                if read_u8(contract(read_usize(G_CONTRACT_MANAGER), p.addr()) + contract::TRANSFER_STATUS) & transfer_status::LOAN_LISTED == 0 {
                    continue;
                }
            }
            if exe::transfer_manager_club_has_offer_for_player(gm, p, club) != 0 || exe::transfer_manager_get_exchange_offers_for_player(gm, p, 0) != 0 {
                continue;
            }
            covered!("cqe_3 loan draw");
            let r1 = exe::utils_random(club.reputation() as i32 / 10); // cqe3: the loan random draw
            let r2 = exe::utils_random(read_usize(slot(shortlist_slot::PRICE)) as i32);
            if r2 <= r1 {
                let r1 = exe::utils_random((club.reputation() as i32).wrapping_shl(1));
                let r2 = exe::utils_random(read_usize(slot(shortlist_slot::PRICE)) as i32);
                if r2 <= r1 {
                    continue;
                }
            }
            let k = exe::shortlist_manager_try_loan_offer_for_player(this, p, club);
            write_usize(slot(shortlist_slot::OFFER_REF), k as u32 as usize);
            if read_usize(slot(shortlist_slot::OFFER_REF)) as i32 > -1 {
                count = count.wrapping_add(1);
            } else {
                // simplification: the setne goes with the high bytes of the slot pointer (as in the original); 0x843e10 reads the byte
                let e = slot(0);
                ((exe::shortlist_manager_update_staff_status(this, p, club, 1, (e as u32 & 0xffff_ff00) | (read_u8(e) != 0xb) as u32) as usize) as u8);
            }
        }
    }
}

/// Days from today to a date given by year and day pointers, as the original: (year - this year) x 365 - today's day +
/// day.
pub(crate) unsafe fn days_until_date(year: usize, day: usize) -> i32 {
    ((read_u16(year) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32) * 365 - read_i16(G_CURRENT_DATE) as i32) + read_i16(day) as i32
}

/// True if today minus 150 days is after the date stored at manager +0x3e (day) / +0x40 (year).
pub(crate) unsafe fn is_150_days_since(t: Staff) -> bool {
    let mut d = [0u8; 8];
    exe::date_sub_days(G_CURRENT_DATE, (&mut d) as *mut [u8; 8] as usize, 0x96);
    let day = i16::from_le_bytes([d[0], d[1]]);
    let year = u16::from_le_bytes([d[2], d[3]]);
    let year_t = t.date_joined_club_year();
    year > year_t || (year == year_t && day > (t.date_joined_club() as i16))
}

/// Distance between two cities (f64 coordinates +0x23, +0x2b) x 55.
pub(crate) unsafe fn shortlist_manager_city_distance(a: usize, b: usize) -> f64 {
    let dx = read_f64(a + city::LONGITUDE) - read_f64(b + city::LONGITUDE);
    let dy = read_f64(a + city::LATITUDE) - read_f64(b + city::LATITUDE);
    (dy * dy + dx * dx).sqrt() * read_f64(rdata::F64_55)
}

/// The original's max: x if x > y, else y.
pub(crate) fn max_f64(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// Truncated average of the three reputations of a player record (+9, +0xb, +0xd).
pub(crate) unsafe fn average_reputation3(f: PlayerSheet) -> i32 {
    (f.world_reputation() as i32 + f.current_reputation() as i32 + f.home_reputation() as i32) / 3
}

/// Club reputation adjusted by stadium city attraction.
pub(crate) unsafe fn reputation_by_stadium(club: Club) -> i32 {
    let rep = club.reputation() as i32;
    let e = club.stadium();
    let city = if !e.is_null() { e.city() } else { 0 };
    if city == 0 {
        return ftol(rep as f64 * read_f64(rdata::F64_0_2857142857));
    }
    let imp = read_i8(city + city::ATTRACTION);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * read_f64(rdata::F64_0_2857142857);
    ftol(max_f64(x, q as f64))
}

/// The club's reputation adjusted by its division (+0x57, its +0x69) and attendance.
///
/// Without a division, its own (the rest of the register comes from the pointer; only the word counts); with reputation
/// 7500 and attendance above 35000 (+0x73), the greater of the reputation and 500 × the division's; otherwise the
/// greater of rep/2 + 250 × the division's and 500 × the division's.
pub(crate) unsafe fn reputation_by_division(club: Club) -> i16 {
    let rep = club.reputation() as i32;
    let comp = club.division();
    if comp.is_null() {
        return rep as i16;
    }
    let cv = comp.reputation() as i32;
    let t = cv.wrapping_mul(500);
    if rep >= 0x1d4c && (club.attendance() as usize) as i32 > 0x88b8 {
        (if t > rep { t } else { rep }) as i16
    } else {
        let h = rep / 2 + cv.wrapping_mul(250);
        (if t > h { t } else { h }) as i16
    }
}

/// The reputation from the club's competition and its position there; returns (value, record).
///
/// The competition is +0x1db (or the one in list [0xae2d14] + id × 6 when it is negative or is [0x9cf6f8] with
/// reputation 5000): the record from [0xae23d0] (0x6b bytes; 15000 for [0x9cf6e8], otherwise 600 × +0x69 in 16 bits)
/// and, with the club in its table (`exe::comp_stats_find_club_entry`) in 1st to 16th place without mark +4, (1.32 − 0.02 × position) × that.
/// With no record: the minimum of 0.8 × `pela_divisao` and 7000.
pub(crate) unsafe fn reputation_by_competition(club: Club, by_division: i16) -> (i32, usize) {
    let rep = club.reputation();
    let mut idx = (club.continental_comp_id() as usize) as i32;
    let mut has = true;
    if idx <= -1 || (idx == read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) as i32 && rep >= 0x1388) {
        idx = read_usize(read_usize(G_CLUB_STATE_TABLE).wrapping_add(read_usize(club.addr()).wrapping_mul(6))) as i32;
        if idx <= -1 {
            has = false;
        }
    }
    let mut reg = 0usize;
    if has {
        reg = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = read_usize(reg);
            let s: i16 = if id == read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) { 15000 } else { read_i16(reg + competition::REPUTATION).wrapping_mul(600) };
            // 0x4a4850: push club; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let r = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))) + 8), club));
            if !r.is_null() && read_u8(r.addr() + 4) == 0 && (1..=0x10).contains(&r.final_position()) {
                let p = r.final_position() as f64;
                return (ftol((read_f64(rdata::F64_1_32) - p * read_f64(rdata::F64_0_02)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = by_division as f64 * read_f64(rdata::F64_0_8);
    if !(x < read_f64(rdata::F64_7000)) {
        x = read_f64(rdata::F64_7000);
    }
    (ftol(x), reg)
}

/// Reputation as seen through the manager-manager confidence row of the club.
pub(crate) unsafe fn reputation_by_manager(club: usize, r: i16) -> i32 {
    let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_usize(club).wrapping_mul(0x49));
    let v = ftol((read_i16(t + board_record::STAFF_CONFIDENCE) as f64 + read_i16(t + 8) as f64) * read_f64(rdata::F64_0_25) + read_i16(t + 6) as f64 * read_f64(rdata::F64_0_5)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = read_f64(rdata::F64_5) - v as f64 * read_f64(rdata::F64_0_0008);
        let q1 = dd / t;
        let m = dd - t * read_f64(rdata::F64_1000);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * read_f64(rdata::F64_0_0001) + read_f64(rdata::F64_0_5);
        let q1 = t * dd;
        let m = t * read_f64(rdata::F64_1000) + dd;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// Reputation ceiling by financial band: band 1 max(1.25r, r+750), band 2 max(1.5r, r+1500), else r.
pub(crate) fn reputation_cap_by_finances(band: i8, r: i16) -> i16 {
    match band {
        1 => ftol(max_f64(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => ftol(max_f64(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h - 2000) on the word.
pub(crate) fn half_or_minus_2000(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// The fluency factor: max(F × 0.1, L / (12000 − 200 F)) + 0.2; returns (factor, F).
///
/// L = (s + 5000)² × 1/15000 and F is the person's fluency in the language of the club's nation (`get_language_fluency`: push 1; push
/// language; push club; push person; mov ecx, [0xb74350] -> thiscall, ret 0x10; the language goes in a register with
/// garbage above, the copy passes only the byte).
pub(crate) unsafe fn language_fluency_factor(p1: Staff, club: Club, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * read_f64(rdata::F64_6_666666667ENEG_05) * q;
    let f = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p1, club, (club.nation().first_language() as u8) as u32, 1) as i8 as i32 as f64;
    let a = f * read_f64(rdata::F64_0_1);
    let b = l / (read_f64(rdata::F64_12000) - f * read_f64(rdata::F64_200));
    (max_f64(a, b) + read_f64(rdata::F64_0_2), f)
}

/// Returns in st0 the person's willingness score for joining the club (or for staying, when the club is his own) and
/// marks the reason bits.
///
/// `Shortlist_Manager::staff_interest_in_joining_club(p1, p2, p3, p4, p5, p6)` (person, club, wage, loan, status,
/// reasons): x87 transcribed into f64 in the original's order. The reasons (bits) are only set when the pointer is
/// given.
///
/// The score starts at 2.75 (3.5 for his own club; with the contract running out, 1.1 − days × 0.002; with no club, the
/// age outside a club) and goes through the factors: the club reputations on loans, liking or disliking the club and
/// the manager, rivals, the nations and distance, happiness in the contract (`contract::MORALE`), squad status
/// (`contract::SQUAD_STATUS`) and demands, the compared reputation (the new club's and the current one's, by division,
/// competition, manager and finances), language fluency, international caps and time without a club; and, when a wage
/// is asked (wage ≠ 0, no loan and a score of 1.75), the default wage (`exe::staff_contract_default_staff_contract` in
/// a local STAFF_CONTRACT) against the club's wage cap (`exe::finance_get_max_wage`). The rest of the arithmetic is in
/// the comments.
/// Original: `Shortlist_Manager::staff_interest_in_joining_club` (0x846920, shortlist_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let p1 = Staff(p1);
    let p2 = Club(p2);
    let zero = read_f64(rdata::F64_0);
    // the calls by address, with the conventions checked at the original’s call sites
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    // 0x8cca10: push 0; push person; mov ecx, [0xde1f54] -> thiscall, ret 8
    // 0x4cf5a0: push person; mov ecx, [0xae1050] -> thiscall, ret 4
    // 0x4cf5f0: push club; push person; mov ecx, [0xae1050] -> thiscall, ret 8
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    // 0x53d980: push 1; push nation; push person; add esp, 0xc -> cdecl
    // 0x544590 / 0x544520 / 0x5444a0: push club; push person; add esp, 8 -> cdecl
    // 0x547d30 / 0x547d60: push club; push person; add esp, 8 -> cdecl
    // 0x547810 / 0x547840: push person; push manager; add esp, 8 -> cdecl
    // 0x4b02e0: push current; push club; add esp, 8 -> cdecl
    // 0x4ca470: push contract; push person; mov ecx, [0xae1050] -> thiscall, ret 8
    // 0x600a50: push 0; push 0; push 1 (or 1); push 0 (or 1); push club; add esp, 0x14 -> cdecl
    // 0x5407e0: push 1; push person; push manager; add esp, 0xc -> cdecl (only the word counts)
    // 0x5a0590: push club; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finances -> thiscall, ret 0
    // 0x4cbb60: push club; push person; mov ecx, [0xae1050] -> thiscall, ret 8
    // 0x669bd0: push &date b; push &date a; push club; add esp, 0xc -> cdecl

    let mut data_a = [0u8; 8];
    let mut data_b = [0u8; 8];
    exe::date_date_54a110(data_a.as_mut_ptr() as usize);
    exe::date_date_54a110(data_b.as_mut_ptr() as usize);
    let p4b = p4 as u8 != 0;
    let p5b = p5 as u8 as i8;
    let mut l80 = 1.0f64; // [base+0x84]
    let mut l8c = 0usize; // [base+0x78]: the current club’s competition record
    let mut l84 = 0usize; // [base+0x80]: the new club’s competition record
    let mut ad: i8 = 0; // [base+0x57]
    let fl = move |b: u32| {
        if !p6.is_null() {
            *p6 |= b;
        }
    };
    if !p6.is_null() {
        *p6 = 0;
    }
    if p1.player().is_null() || p2.nation().addr() == 0 || p1.nation().is_null() {
        return zero;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p1, (0) as u32) != 0 {
        return zero;
    }
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p1.addr()).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::RETIREMENT_STATE != 0 {
        return zero;
    }
    let a57 = p1.ambition() as i32 as f64;
    let age = p1.age() as i8;
    let aa8: f32 = ((read_f64(rdata::F64_1_4) - a57 * read_f64(rdata::F64_0_025)) - p1.professionalism() as i32 as f64 * read_f64(rdata::F64_0_025)) as f32;
    let mut a88: f32 = (a57 * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_0_8)) as f32;
    let mut b4: f32 = (a57 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5)) as f32;
    let c0 = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p1));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c0.addr());
    let mut d4 = 0usize;
    if !c0.is_null() {
        d4 = read_usize(G_CLUB_TABLE).wrapping_add(c0.club().wrapping_mul(club::SIZE));
        if d4 != 0 {
            if read_usize(d4 + club::NATION) == 0 {
                return read_f64(rdata::F64_3_5);
            }
            if read_u8(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p1) + contract::CONTRACT_TYPE) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p1, d4);
    let a0: u32 = if c != 0 { read_usize(c + contract::GRIEVANCES) as u32 } else { 0 };
    let r61 = p1.player();
    // the club itself with a contract: stays (3,5) when the contract does not end soon and his reputation does not exceed the club’s
    if !p2.is_null() && p2.addr() == d4 && c != 0 {
        let days = exe::date_days_since(c + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
        let r1 = r61.current_reputation() as i32;
        let mut s = ftol((p2.reputation() as i32 - r1) as f64 * read_f64(rdata::F64_0_5)) as i16;
        if a0 & 0x10 != 0 {
            s = ftol(s as f64 * read_f64(rdata::F64_0_5)) as i16;
        }
        let lim = p1.loyalty() as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = read_usize(p1.addr()) as i32;
            if days < id % 0x23 + 0x14
                && read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::AI_SHORTLIST_COUNT == 0
                && r1 + p1.ambition() as i32 * 20 < (p1.loyalty() as i32 + 0xaa) * 25 + s
            {
                return read_f64(rdata::F64_3_5);
            }
        }
    }
    let mut f8: f64;
    if p2.addr() == d4 {
        if c != 0 && a0 != 0 && a0 != 2 {
            return zero;
        }
        f8 = 3.5;
    } else {
        if c != 0 && d4 != 0 && read_usize(d4 + club::NATION) != 0 && p2.nation().addr() != 0
            && (p2.nation().first_language() as u8) == read_u8(read_usize(d4 + club::NATION) + nation::FIRST_LANGUAGE) && a0 & 0x20 != 0
        {
            return zero;
        }
        if age < 0x10 && exe::database_is_staff_foreign(p1, p2.nation().addr(), 1) != 0 {
            return zero;
        }
        covered!("cqe_4 youth from another club");
        f8 = 2.75; // cqe4: the base for another club
        if c != 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) != 0 {
            let dd = days_until_date(c + contract::CONTRACT_EXPIRES_YEAR, c + contract::CONTRACT_EXPIRES);
            let v = read_f64(rdata::F64_1_1) - dd as f64 * read_f64(rdata::F64_0_002);
            if !p6.is_null() && v < p1.pressure() as i32 as f64 * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_0_6) {
                *p6 |= 1;
            }
            f8 = v * read_f64(rdata::F64_2_75);
        } else if d4 == 0 {
            let dd =((read_u16(G_CURRENT_DATE_YEAR) as i32 - p1.date_joined_club_year() as i32) * 365 - (p1.date_joined_club() as i16) as i32) + read_i16(G_CURRENT_DATE) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * read_f64(rdata::F64_0_001111111111);
            if read_f64(rdata::F64_12) < x {
                x = read_f64(rdata::F64_12);
            }
            f8 = x + read_f64(rdata::F64_2_75);
        }
    }
    // the reputation of both clubs in a loan request
    if p4b && !r61.is_null() && r61.world_reputation() < 0x128e && !p2.is_null() && d4 != 0 && p2.addr() != d4 && a0 & 0x10 != 0 {
        let ax = p2.reputation();
        let same = p2.nation().addr() == read_usize(d4 + club::NATION);
        let rb = r61.current_reputation();
        if ax > read_i16(d4 + club::REPUTATION) && (same || ax > 0x1676) {
            if rb <= 0x1c52 && exe::staff_is_loan_listed(p1, 0) != 0 {
                f8 *= read_f64(rdata::F64_1_5);
            } else {
                f8 *= read_f64(rdata::F64_1_25);
            }
        } else if ax > 0xabe && (same || ax > 0x1676) && ax > rb {
            if rb <= 0x1c52 && exe::staff_is_loan_listed(p1, 0) != 0 {
                f8 *= read_f64(rdata::F64_1_35);
            } else {
                f8 *= read_f64(rdata::F64_1_15);
            }
        } else if ax > 0x109a && same {
            if rb > 0x186a && ax < 0x1482 {
                f8 *= read_f64(rdata::F64_1_025);
            } else if rb <= 0x1c52 && exe::staff_is_loan_listed(p1, 0) != 0 {
                f8 *= read_f64(rdata::F64_1_25);
            } else {
                f8 *= read_f64(rdata::F64_1_1);
            }
        }
    }
    if exe::staff_dislikes_club(p1, p2) != 0 {
        f8 *= read_f64(rdata::F64_0_33);
        fl(2);
    }
    if exe::staff_dislikes_club(p1, d4) != 0 {
        let s = (p1.pressure() as u8).wrapping_add(p1.loyalty() as u8).wrapping_add(p1.professionalism() as u8) as i8;
        if s > 0x2d {
            f8 *= read_f64(rdata::F64_1_25);
        } else if s > 0x19 {
            f8 *= read_f64(rdata::F64_1_5);
        } else {
            f8 = f8 + f8;
        }
        fl(4);
    }
    if exe::staff_favours_club(p1, d4) != 0 && exe::staff_is_transfer_listed_or_requested(p1, d4) == 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) == 0 {
        f8 *= read_f64(rdata::F64_0_5);
        fl(8);
    }
    if exe::staff_favours_club(p1, p2) != 0 {
        fl(0x10);
        f8 *= read_f64(rdata::F64_3);
    }
    if age > 0x22 && d4 != p2.addr() && p2.nation() != p1.nation() {
        f8 *= read_f64(rdata::F64_0_75);
    }
    let max1 = |n: i8| if n < 1 { 1 } else { n as i32 };
    if d4 != 0 && d4 != p2.addr() {
        let (e1, e2) = (read_usize(G_KEY_CLUB_GLASGOW_CELTIC), read_usize(G_KEY_CLUB_GLASGOW_RANGERS));
        let i2 = read_usize(p2.addr());
        let i4 = read_usize(d4);
        if (i2 == e1 || i2 == e2) && (i4 == e1 || i4 == e2) {
            f8 = (read_f64(rdata::F64_0_025) / max1(p1.loyalty()) as f64) * f8;
            fl(0x20);
        } else {
            let n2 = read_usize(p2.nation().addr());
            let (na, nb) = (read_usize(KEY_NATION_CROATIA), read_usize(KEY_NATION_YUGOSLAVIA));
            if n2 == nb && read_usize(read_usize(d4 + club::NATION)) == na && read_usize(p1.nation().addr()) == na {
                f8 *= read_f64(rdata::F64_0_05);
                fl(2);
            } else if n2 == na && read_usize(read_usize(d4 + club::NATION)) == nb && read_usize(p1.nation().addr()) == nb {
                f8 *= read_f64(rdata::F64_0_1);
                fl(2);
            } else {
                if p2.addr() == read_usize(d4 + club::RIVAL1) || p2.addr() == read_usize(d4 + club::RIVAL2) || p2.addr() == read_usize(d4 + club::RIVAL3) {
                    let m = if n2 == read_usize(KEY_NATION_ITALY) {
                        read_f64(rdata::F64_1) - p1.loyalty() as i32 as f64 * read_f64(rdata::F64_0_02)
                    } else {
                        read_f64(rdata::F64_0_2) / max1(p1.loyalty()) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < read_f64(rdata::F64_0_7) {
                        *p6 |= 0x20;
                    }
                }
                if exe::comp_util_get_league_clash_type(p2, d4) != 0 {
                    let m = if read_usize(p2.nation().addr()) == read_usize(KEY_NATION_ITALY) {
                        read_f64(rdata::F64_1) - p1.loyalty() as i32 as f64 * read_f64(rdata::F64_0_02)
                    } else {
                        read_f64(rdata::F64_0_75) / max1(p1.loyalty()) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < read_f64(rdata::F64_0_7) {
                        *p6 |= 0x20;
                    }
                }
            }
        }
    }
    let t2 = p2.manager().addr();
    if exe::staff_is_favoured_by(t2, p1) != 0 {
        f8 = f8 + f8;
        fl(0x40);
    }
    if exe::staff_is_disliked_by(t2, p1) != 0 {
        f8 *= read_f64(rdata::F64_0_33);
        fl(0x80);
    }
    if d4 != 0 {
        let t4 = read_usize(d4 + club::MANAGER);
        if exe::staff_is_disliked_by(t4, p1) != 0 {
            f8 = f8 + f8;
            fl(0x100);
        }
        if exe::staff_is_favoured_by(t4, p1) != 0 {
            f8 *= read_f64(rdata::F64_0_5);
            if !p6.is_null() && read_u8(c + contract::TRANSFER_STATUS) & transfer_status::TRANSFER_REQUESTED == 0 {
                *p6 |= 0x200;
            }
        }
    }
    // the happiness in the contract (+0x44), the squad status (ea) and the contract demand (c1 = +0x3d)
    let mut ea: i8;
    let c1: i8;
    if c == 0 {
        if p2.addr() == d4 {
            c1 = 0;
            ea = p5b;
        } else {
            ea = 7;
            c1 = -100;
        }
    } else {
        let e44 = read_i8(c + contract::MORALE) as i32;
        if p2.addr() == d4 {
            f8 = (e44 as f64 * read_f64(rdata::F64_0_0025) + read_f64(rdata::F64_1)) * f8;
            ea = p5b;
            if p2.manager().addr() != 0 && is_150_days_since(p2.manager()) {
                ea = exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
            }
        } else {
            let v = read_f64(rdata::F64_1) - e44 as f64 * read_f64(rdata::F64_0_0025);
            f8 = v * f8;
            if v > read_f64(rdata::F64_1_01) {
                f8 = v * f8;
            }
            if a0 & 0x10 != 0 {
                a88 = (a88 as f64 * read_f64(rdata::F64_1_5)) as f32;
            }
            if a0 & 0x100 != 0 {
                a88 = (a88 as f64 * read_f64(rdata::F64_1_5)) as f32;
            }
            if a0 & 1 != 0 {
                b4 = (b4 as f64 * read_f64(rdata::F64_1_5)) as f32;
            }
            if !p6.is_null() {
                if v > read_f64(rdata::F64_1_5) && read_u8(c + contract::TRANSFER_STATUS) & transfer_status::TRANSFER_REQUESTED == 0 {
                    *p6 |= 0x400;
                } else if v < read_f64(rdata::F64_0_5) {
                    *p6 |= 0x800;
                }
            }
            if d4 != 0 && read_usize(d4 + club::MANAGER) != 0 && is_150_days_since(Staff(read_usize(d4 + club::MANAGER))) {
                ea = exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
            } else {
                ea = (read_u8(c + contract::SQUAD_STATUS) >> 4) as i8;
            }
            if read_u8(c + contract::TRANSFER_STATUS) & transfer_status::TRANSFER_REQUESTED != 0 {
                ea = if ea == 1 { 3 } else if ea < 1 || ea > 3 { 7 } else { 4 };
            }
        }
        c1 = read_i8(c + contract::HAPPINESS);
    }
    let pos20 = || (p1.player().goalkeeper() as u8) == 0x14;
    let jovem = age < 0x20 && (p1.int_apps() as i8) != 0;
    let signed_byte_at: i16; // ebx: the reputation of the current club (or the person’s, without a club) on the comparison scale
    let mut e8: i32 = 0; // [base+0x1c]: the new club’s
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2.addr() != d4 {
        if exe::human_manager_is_human_club(p2, 0, 1, 0, 0) != 0 {
            let v = p2.manager().loyalty() as i32 as f64 * read_f64(rdata::F64_0_025) + read_f64(rdata::F64_0_5);
            f8 = v * f8;
            if !p6.is_null() && v < read_f64(rdata::F64_0_675) {
                *p6 |= 0x20000000;
            }
        }
        match ea {
            1 => {
                f8 *= read_f64(rdata::F64_0_65);
                if !p6.is_null() && read_usize(c + contract::GRIEVANCES) == 0 {
                    *p6 |= 0x1000;
                }
            }
            2 => {
                f8 *= read_f64(rdata::F64_0_75);
                if !p6.is_null() && read_usize(c + contract::GRIEVANCES) == 0 {
                    *p6 |= 0x1000;
                }
            }
            3 => f8 *= read_f64(rdata::F64_0_9),
            4 => {
                f8 *= read_f64(rdata::F64_1_8);
                fl(0x2000);
                if age < 0x20 {
                    if age > 0x17 {
                        f8 = f8 + f8;
                    }
                    if (p1.int_apps() as i8) != 0 {
                        f8 = f8 + f8;
                    }
                }
            }
            5 => {
                if !p4b {
                    f8 *= read_f64(rdata::F64_0_75);
                }
                if jovem {
                    f8 = f8 + f8;
                }
            }
            6 => {
                f8 *= read_f64(rdata::F64_1_1);
                if jovem {
                    f8 *= read_f64(rdata::F64_4);
                }
            }
            7 => {
                f8 *= read_f64(rdata::F64_7);
                fl(0x2000);
            }
            _ => {}
        }
        let cf10 = c != 0 && read_u8(c + contract::GRIEVANCES) & 0x10 != 0;
        match p5b {
            1 => {
                if cf10 {
                    f8 = f8 + f8;
                    fl(0x4000);
                } else {
                    f8 *= read_f64(rdata::F64_1_538461538);
                }
                b4 *= read_f32(rdata::F32_1_100000024);
            }
            2 => {
                if cf10 {
                    f8 *= read_f64(rdata::F64_1_666666667);
                    fl(0x4000);
                } else {
                    f8 *= read_f64(rdata::F64_1_333333333);
                }
                b4 *= read_f32(rdata::F32_1_049999952);
            }
            3 => {
                if !cf10 {
                    f8 *= read_f64(rdata::F64_1_111111111);
                }
                if pos20() {
                    f8 *= read_f64(rdata::F64_0_8);
                }
            }
            4 => {
                if a0 & 0x10 != 0 {
                    f8 *= read_f64(rdata::F64_0_175);
                    if !p1.non_player().is_null() {
                        f8 *= read_f64(rdata::F64_2_666666667);
                    } else {
                        fl(0x8000);
                    }
                } else {
                    f8 *= if pos20() { read_f64(rdata::F64_0_15) } else { read_f64(rdata::F64_0_375) };
                }
                if age < 0x20 {
                    if age > 0x17 {
                        f8 *= read_f64(rdata::F64_0_5);
                    }
                    if (p1.int_apps() as i8) != 0 {
                        f8 *= read_f64(rdata::F64_0_5);
                    }
                }
                b4 *= read_f32(rdata::F32_0_8000000119);
            }
            5 => {
                if a0 & 0x10 != 0 {
                    f8 *= read_f64(rdata::F64_0_03333333333);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { read_f64(rdata::F64_0_2) } else { read_f64(rdata::F64_0_4) };
                }
                if jovem {
                    f8 *= read_f64(rdata::F64_0_5);
                }
                b4 *= read_f32(rdata::F32_0_6700000167);
            }
            6 => {
                if a0 & 0x10 != 0 {
                    f8 *= read_f64(rdata::F64_0_02);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { read_f64(rdata::F64_0_1) } else { read_f64(rdata::F64_0_15) };
                }
                if jovem {
                    f8 *= read_f64(rdata::F64_0_25);
                }
                b4 *= read_f32(rdata::F32_0_5);
            }
            7 => f8 *= read_f64(rdata::F64_0_002),
            _ => {}
        }
        // the new club’s reputation, on several scales, against the person’s
        let mut adv = p2.nation().state_of_development() as i8;
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = reputation_by_stadium(p2);
        h68 = reputation_by_division(p2);
        let (h6c, reg) = reputation_by_competition(p2, h68);
        l84 = reg;
        let t = p2.manager().addr();
        let mut rv: i16 = 0;
        if t != 0 && read_usize(t + staff::NON_PLAYER) != 0 {
            rv = exe::database_get_staff_reputation_for_viewer(t, p1, 1);
            if read_usize(G_MANAGER_MANAGER) != 0 {
                rv = reputation_by_manager(p2.addr(), rv) as i16;
            }
        }
        let mut si: i16 = p2.reputation();
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), p2);
        if fin != 0 && ad == 1 {
            si = reputation_cap_by_finances(exe::finance_get_wealth_band(fin) as i8, si);
        }
        let ax = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(p2.addr()).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = ftol(si as f64 * read_f64(rdata::F64_0_65) + m as f64 * read_f64(rdata::F64_0_35)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + read_f32(rdata::F32_2) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = ftol(s / den as f64) as i16;
        let cl = p2.professional_status();
        if cl != 1 {
            if !(cf10 && p1.int_apps() == 0) {
                w = half_or_minus_2000(w);
            }
            if cl == 3 {
                w = half_or_minus_2000(w);
            }
        }
        e8 = ftol(w as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000));
        if p2.nation().addr() != 0 {
            let (m, f) = language_fluency_factor(p1, p2, e8 as i16);
            if age < 0x12 && f < read_f64(rdata::F64_5) && p1.nation().continent() != p2.nation().continent() {
                return zero;
            }
            f8 = m * f8;
            if !p6.is_null() && m < read_f64(rdata::F64_0_6) {
                *p6 |= 0x10000;
            }
        }
        // the nations, the distance and what the person wants (requested status)
        #[derive(PartialEq)]
        enum Segue {
            Padrao,
            Fatores,
        }
        let fav = p1.club_job();
        let follows = 'n: {
            let cid2 = if p2.stadium().addr() != 0 { p2.stadium().city() } else { 0 };
            let cid4 = if d4 != 0 && read_usize(d4 + club::STADIUM) != 0 { read_usize(read_usize(d4 + club::STADIUM) + stadium::CITY) } else { 0 };
            if !fav.is_null() && fav.nation() == p2.nation() {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                let mut g = f8;
                if exe::staff_is_loan_listed(p1, 0) != 0 && age < 0x18 && !r61.is_null() && r61.current_reputation() < 0x109a && p1.professionalism() > 0xf
                    && p1.pressure() > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= read_f64(rdata::F64_2_5);
                }
                if d4 != 0 && p2.nation().addr() != 0 && read_usize(p2.nation().addr()) == read_usize(KEY_NATION_ITALY) && age < 0x18
                    && read_usize(d4 + club::DIVISION) != 0 && read_usize(read_usize(d4 + club::DIVISION)) == read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A)
                {
                    g *= read_f64(rdata::F64_3);
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = shortlist_manager_city_distance(cid2, cid4);
                    if dd < read_f64(rdata::F64_40) {
                        f8 = g * read_f64(rdata::F64_2_9);
                        fl(0x20000);
                    } else if dd < read_f64(rdata::F64_80) {
                        let m = read_f64(rdata::F64_4_5) - dd * read_f64(rdata::F64_0_04);
                        f8 = m * g;
                        if !p6.is_null() && m > read_f64(rdata::F64_1_6) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 = g * read_f64(rdata::F64_1_3);
                    }
                } else {
                    f8 = g * read_f64(rdata::F64_1_3);
                }
                break 'n Segue::Fatores;
            }
            if p2.nation().addr() != 0 && exe::database_is_staff_foreign(p1, p2.nation().addr(), 1) == 0 {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                if d4 == 0 {
                    break 'n Segue::Fatores;
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = shortlist_manager_city_distance(cid2, cid4);
                    if dd < read_f64(rdata::F64_40) {
                        f8 *= read_f64(rdata::F64_2_9);
                        fl(0x20000);
                    } else if dd < read_f64(rdata::F64_80) {
                        let m = read_f64(rdata::F64_4_5) - dd * read_f64(rdata::F64_0_04);
                        f8 = m * f8;
                        if !p6.is_null() && m > read_f64(rdata::F64_1_6) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 *= read_f64(rdata::F64_1_3);
                    }
                } else {
                    f8 *= read_f64(rdata::F64_1_3);
                }
                break 'n Segue::Fatores;
            }
            let t = p2.manager().addr();
            let same_manager = t != 0 && read_usize(t + staff::NATION) != 0 && exe::database_is_staff_foreign(p1, read_usize(t + staff::NATION), 1) == 0;
            if p2.nation().addr() != 0 && read_usize(p2.nation().addr()) == read_usize(KEY_NATION_GERMANY) {
                f8 *= read_f64(rdata::F64_0_75);
            }
            if p4b {
                f8 *= if same_manager { read_f64(rdata::F64_0_45) } else { read_f64(rdata::F64_0_08) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = p1.adaptability() as i32;
            let c57 = p1.ambition() as i32;
            if same_manager {
                match p5b {
                    2 => f8 = ((c56 as f64 * read_f64(rdata::F64_0_005) + c57 as f64 * read_f64(rdata::F64_0_02)) + read_f64(rdata::F64_0_8)) * f8,
                    3 => f8 = ((c56 as f64 * read_f64(rdata::F64_0_005) + c57 as f64 * read_f64(rdata::F64_0_02)) + read_f64(rdata::F64_0_7)) * f8,
                    4 => {
                        f8 *= read_f64(rdata::F64_0_5);
                        fl(0x80000);
                    }
                    5 => {
                        f8 *= read_f64(rdata::F64_0_4);
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= read_f64(rdata::F64_0_2);
                        fl(0x80000);
                    }
                    _ => {}
                }
            } else {
                match p5b {
                    2 => f8 = ((c56 + c57) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_7)) * f8,
                    3 => f8 = ((c56 + c57) as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_6)) * f8,
                    4 => {
                        f8 *= read_f64(rdata::F64_0_25);
                        fl(0x80000);
                    }
                    5 => {
                        f8 = ((c56 + c57) as f64 * read_f64(rdata::F64_0_004) + read_f64(rdata::F64_0_25)) * f8;
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= read_f64(rdata::F64_0_025);
                        fl(0x80000);
                    }
                    _ => {}
                }
            }
            Segue::Padrao
        };
        if follows == Segue::Padrao {
            if (c != 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) != 0) || exe::staff_is_transfer_listed(p1, 0) != 0 {
                f8 *= read_f64(rdata::F64_2_5);
                if f8 < read_f64(rdata::F64_2) && (p2.reputation() > 0x1a5e || p2.reputation() > read_i16(d4 + club::REPUTATION)) {
                    let t = f8 + read_f64(rdata::F64_1);
                    f8 = if read_f64(rdata::F64_2) < t { 2.0 } else { t };
                }
            }
        } else {
            if exe::staff_is_loan_listed(p1, 0) != 0 {
                f8 = (read_f64(rdata::F64_1_5) - c1 as f64 * read_f64(rdata::F64_0_01)) * f8;
            }
            let mut m = read_f64(rdata::F64_1) - c1 as f64 * read_f64(rdata::F64_0_015);
            if m < read_f64(rdata::F64_0_1) {
                m = read_f64(rdata::F64_0_1);
            }
            f8 = m * f8;
            if !p6.is_null() && m < read_f64(rdata::F64_0_6) {
                *p6 |= 0x40000;
            }
            let a = if pos20() {
                (0x21 - age as i32) as f64
            } else {
                let a = (0x1c - age as i32) as f64;
                if a < zero { a * read_f64(rdata::F64_0_4) } else { a }
            };
            let a = a.abs() * read_f64(rdata::F64_0_08333333333) + read_f64(rdata::F64_0_25);
            f8 = a * f8;
            if !p6.is_null() && a < read_f64(rdata::F64_0_6) {
                *p6 |= 0x40000;
            }
        }
        if d4 == 0 {
            f8 *= read_f64(rdata::F64_1_75);
        } else if exe::staff_is_transfer_listed_or_requested(p1, 0) == 0 && exe::staff_is_loan_listed(p1, 0) == 0 && (c == 0 || read_u8(c + contract::LEAVING_ON_BOSMAN) == 0) {
            let dd = ((read_u16(G_CURRENT_DATE_YEAR) as i32 - p1.date_joined_club_year() as i32) * 365 - (p1.date_joined_club() as i16) as i32) + read_i16(G_CURRENT_DATE) as i32;
            let e: i32 = if p4b {
                (20 - c1 as i32 / 5).wrapping_mul(dd)
            } else {
                if read_usize(c + contract::GRIEVANCES) == 0 && dd < (p1.loyalty() as i32 + 8) * 15 {
                    return zero;
                }
                let dv = p1.loyalty() as i32 + 10;
                dd.wrapping_mul(20) / if dv < 1 { 1 } else { dv }
            };
            if e < 0 {
                return zero;
            }
            if e > 0x190 {
                l80 = 400.0 * read_f64(rdata::F64_0_0025);
            } else {
                if e == 0 {
                    return zero;
                }
                l80 = e as f64 * read_f64(rdata::F64_0_0025);
            }
        }
    } else {
        // the club itself: the squad status by the demand, with the manager there for more than 150 days
        if exe::human_manager_is_human_club(p2, 0, 1, 0, 0) != 0 && is_150_days_since(p2.manager()) {
            let lim = 0xceu8.wrapping_sub(p1.loyalty() as u8) as i8;
            if c1 < lim && ea != 5 && ea != 6 {
                ea = 7;
            } else if c1 < 0 {
                if ea < 4 {
                    ea = 4;
                }
            } else if c1 < 0x32 {
                if ea < 3 {
                    ea = 3;
                }
            } else if c1 < 0x4b {
                if ea < 2 {
                    ea = 2;
                }
            } else if ea < 1 {
                ea = 1;
            }
        }
        match ea {
            1 => f8 *= read_f64(rdata::F64_1_2),
            3 => f8 *= read_f64(rdata::F64_0_9),
            4 => {
                f8 *= read_f64(rdata::F64_0_85);
                if age < 0x20 {
                    if age > 0x17 {
                        f8 *= read_f64(rdata::F64_0_5);
                    }
                    if (p1.int_apps() as i8) != 0 {
                        f8 *= read_f64(rdata::F64_0_5);
                    }
                }
            }
            5 => {
                f8 *= read_f64(rdata::F64_0_75);
                if jovem {
                    f8 *= read_f64(rdata::F64_0_5);
                }
            }
            6 => {
                f8 *= read_f64(rdata::F64_0_7);
                if jovem {
                    f8 *= read_f64(rdata::F64_0_25);
                }
            }
            7 => f8 *= read_f64(rdata::F64_0_01),
            _ => {}
        }
    }
    // the reputation of the current club (or of the person, without a club) on the same scale
    if d4 == 0 {
        let mut aev = p1.nation().state_of_development() as i8;
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        signed_byte_at = ftol(average_reputation3(p1.player()) as i16 as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16;
    } else {
        let n4 = Nation(read_usize(d4 + club::NATION));
        let mut aev = if !n4.is_null() { n4.state_of_development() as i8  } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = reputation_by_stadium(Club(d4));
        h6c4 = reputation_by_division(Club(d4)) as i32;
        let (bx, reg) = reputation_by_competition(Club(d4), h6c4 as i16);
        l8c = reg;
        let t4 = read_usize(d4 + club::MANAGER);
        let si: i16 = if t4 == 0 || read_usize(t4 + staff::NON_PLAYER) == 0 {
            ftol(h6c4 as i16 as f64 * read_f64(rdata::F64_0_75)) as i16
        } else if read_usize(G_MANAGER_MANAGER) == 0 {
            read_i16(read_usize(t4 + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION)
        } else {
            reputation_by_manager(d4, read_i16(read_usize(t4 + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION)) as i16
        };
        let mut di: i16 = read_i16(d4 + club::REPUTATION);
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), d4);
        if fin != 0 && ae == 1 && c != 0 && read_usize(c + contract::GRIEVANCES) == 0 {
            let m = exe::finance_get_wealth_band(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = reputation_cap_by_finances(m as i8, di);
            }
        }
        let ax = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = ftol(di as f64 * read_f64(rdata::F64_0_65) + m as f64 * read_f64(rdata::F64_0_35)) as i16;
        // simplification: at the club itself (p2 == d4) the original divides by the float [base+0x58] that nobody wrote (leftover from
        // stack); the result is not used afterwards (the path goes straight to the end), and the copy divides by 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = ftol(s / den as f64) as i16;
        if !p4b && exe::staff_is_transfer_listed(p1, 0) != 0 {
            di = ftol(di as f64 * read_f64(rdata::F64_0_9)) as i16;
        }
        let cl = read_u8(d4 + club::PROFESSIONAL_STATUS);
        if cl != 1 {
            if !(c != 0 && read_u8(c + contract::GRIEVANCES) & 0x10 != 0 && p1.int_apps() == 0) {
                di = half_or_minus_2000(di);
            }
            if cl == 3 {
                di = half_or_minus_2000(di);
            }
        }
        if exe::contract_manager_get_squad_placement(read_usize(G_CONTRACT_MANAGER), p1, d4) == 3 {
            di = ftol(di as f64 * read_f64(rdata::F64_0_7)) as i16;
        }
        signed_byte_at = ftol(di as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16;
        if !n4.is_null() && p2.addr() != d4 {
            let (m, _) = language_fluency_factor(p1, Club(d4), signed_byte_at);
            f8 = f8 / m;
        }
    }
    if p2.addr() != d4 {
        let comp2 = p2.division().addr();
        if comp2 == 0 || read_usize(comp2) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            f8 *= read_f64(rdata::F64_0_1);
        } else if p1.club_job() == p2 && !r61.is_null() && r61.player_morale() > 0xa {
            f8 = r61.player_morale() as i32 as f64 * f8 * read_f64(rdata::F64_0_1);
        }
        let mut a8 = (p2.reputation() as i32 - average_reputation3(r61)) as f64 * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1);
        let e8s = e8 as i16 as i32;
        let sbs = signed_byte_at as i32;
        let cf10 = c != 0 && read_u8(c + contract::GRIEVANCES) & 0x10 != 0;
        let mut long = false;
        if p4b && (cf10 || exe::staff_is_loan_listed(p1, 0) != 0) {
            let f = p1.player();
            long = (read_i16(f.addr() + 5) as i32 * 50) < read_i16(d4 + club::REPUTATION) as i32
                && (p1.int_apps() as i8) == 0
                && !(age >= 0x18 && age <= 0x20)
                && p2.reputation() > 0xdac
                && f.current_reputation() < 0x157c
                && p2.nation().addr() != 0
                && p2.division().addr() != 0
                && read_usize(d4 + club::DIVISION) != 0
                && !(p2.division().reputation() <= 6 && p2.division().reputation() as i32 <= read_i16(read_usize(d4 + club::DIVISION) + competition::REPUTATION) as i32 - 3)
                && read_usize(d4 + club::NATION) != 0
                && !((p2.nation().league_standard() as i8) < 0xa && (p2.nation().league_standard() as i8) < read_i8(read_usize(d4 + club::NATION) + nation::LEAGUE_STANDARD));
        }
        let dv: f64 = if long {
            (e8s - sbs) as f64 * read_f64(rdata::F64_0_5)
        } else if cf10 && e8s < sbs && sbs - e8s < 0x1388 {
            let d = (sbs - e8s) as f64;
            d * read_f64(rdata::F64_NEG_0_0002) * d
        } else if e8s > sbs {
            if c != 0 && read_u8(c + contract::GRIEVANCES) & 1 != 0 {
                (e8s - sbs) as f64 * read_f64(rdata::F64_2_5)
            } else {
                (p1.ambition() as i32 as f64 * read_f64(rdata::F64_0_08771929825) + read_f64(rdata::F64_0_75)) * (e8s - sbs) as f64
            }
        } else if p5b > 3 {
            (e8s - sbs) as f64 * read_f64(rdata::F64_1_5)
        } else if !(f8 > read_f64(rdata::F64_10)) {
            (e8s - sbs) as f64
        } else {
            let v = if sbs > 0x1b58 && e8s < 0x7d0 && p4b {
                let z = read_f32(rdata::F32_2) as f64 - (sbs - e8s) as f64 * read_f32(rdata::F32_0_0001999999949) as f64;
                let mn = if f8 < read_f64(rdata::F64_50) { f8 } else { read_f64(rdata::F64_50) };
                let zm = z * mn;
                if read_f64(rdata::F64_10) > zm {
                    read_f64(rdata::F64_10) * read_f64(rdata::F64_0_01)
                } else {
                    zm * read_f64(rdata::F64_0_01)
                }
            } else {
                f8 * read_f64(rdata::F64_0_01)
            };
            let mut w = read_f64(rdata::F64_1_1) - v;
            if w < read_f64(rdata::F64_0_1) {
                w = read_f64(rdata::F64_0_1);
            } else if !(w <= read_f64(rdata::F64_1)) {
                w = read_f64(rdata::F64_1);
            }
            w * (e8s - sbs) as f64
        };
        if dv > zero && l80 < read_f64(rdata::F64_1) {
            let mut t = dv * read_f64(rdata::F64_0_02);
            if read_f64(rdata::F64_100) < t {
                t = read_f64(rdata::F64_100);
            }
            let r = ((read_f64(rdata::F64_100) - t) * l80 + t) * read_f64(rdata::F64_0_01);
            f8 = f8 * r;
            if !p6.is_null() && r < read_f64(rdata::F64_0_6) {
                *p6 |= 0x100000;
            }
        }
        covered!("cqe_4 injury_word_0c factor");
        let mut e0 = dv * read_f64(rdata::F64_0_0001612903226) + read_f64(rdata::F64_1); // cqe4: the factor of the compared reputation
        if e0 < read_f64(rdata::F64_0_001) {
            e0 = f64::from_bits(0x3f50624dd2f1a9fc);
        }
        if c != 0 && read_u8(c + contract::UNKNOWN18_3) & 1 != 0 {
            e0 *= read_f64(rdata::F64_1_5);
        }
        let mut x = e0 * f8;
        let without_requested = |c: Contract| c.is_null() || c.leaving_on_bosman() == 0;
        if !p6.is_null() {
            if e0 < read_f64(rdata::F64_0_6) {
                let ok = b4 as f64 > read_f64(rdata::F64_1_2) && l8c != 0 && l84 == 0 && x > read_f64(rdata::F64_1_25) && ea < 4 && ea > 0 && without_requested(Contract(c));
                *p6 |= if ok { 0x400000 } else { 0x800000 };
            } else if e0 > read_f64(rdata::F64_1_5) && b4 as f64 > read_f64(rdata::F64_1_2) && l8c == 0 && l84 != 0
                && !(d4 != 0 && read_i16(d4 + club::REPUTATION) >= 0x1d4c) && p5b < 4 && p5b > 0 && without_requested(Contract(c))
            {
                *p6 |= 0x200000;
            }
        }
        if !p4b {
            x *= e0;
        }
        if c != 0 && read_u8(c + contract::GRIEVANCES) & 1 != 0 {
            x *= e0;
            if !p6.is_null() {
                if e0 < read_f64(rdata::F64_0_8) {
                    let ok = b4 as f64 > read_f64(rdata::F64_1_2) && l8c != 0 && l84 == 0 && x > read_f64(rdata::F64_1_25) && ea < 4 && ea > 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) == 0;
                    *p6 |= if ok { 0x400000 } else { 0x800000 };
                } else if e0 > read_f64(rdata::F64_2) && b4 as f64 > read_f64(rdata::F64_1_2) && l8c == 0 && l84 != 0
                    && !(d4 != 0 && read_i16(d4 + club::REPUTATION) >= 0x1d4c) && p5b < 4 && p5b > 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) == 0
                {
                    *p6 |= 0x200000;
                }
            }
        }
        if p4b {
            if r61.current_reputation() > signed_byte_at && sbs > e8s {
                a8 = ((e8s - sbs) as f64 * read_f64(rdata::F64_0_0002) + read_f64(rdata::F64_1)) * a8;
                if a8 < read_f64(rdata::F64_0_1) {
                    a8 = f64::from_bits(0x3fb999999999999a);
                }
            }
            let r = ae as i32 as f64 / ad as i32 as f64;
            x = x * r;
            if !p6.is_null() && r < read_f64(rdata::F64_0_6) {
                *p6 |= 0x1000000;
            }
        } else if d4 != 0 {
            if ae == ad {
                let t = p1.nation().continent();
                if t != 0 && read_usize(t) == read_usize(KEY_CONTINENT_EUROPE) {
                    let u = p2.nation().continent();
                    if u != 0 && read_usize(u) != read_usize(KEY_CONTINENT_EUROPE) && u != read_usize(read_usize(d4 + club::NATION) + nation::CONTINENT) {
                        x *= read_f64(rdata::F64_0_4);
                        fl(0x2000000);
                    }
                }
            } else {
                let v6c = h6c4 as i16;
                let wv = if v6c > 0x1f4 { v6c as i32 } else { 500 };
                let mut q = (ae as i32 as f64 * h68 as f64) / (wv as f64 * ad as i32 as f64);
                if !(read_f64(rdata::F64_0_01) <= q) {
                    q = read_f64(rdata::F64_0_01);
                } else if read_f64(rdata::F64_10) < q {
                    q = read_f64(rdata::F64_10);
                }
                x = x * q;
                if p2.nation().continent() != read_usize(read_usize(d4 + club::NATION) + nation::CONTINENT) && q < read_f64(rdata::F64_1) {
                    x *= read_f64(rdata::F64_0_2);
                    fl(0x2000000);
                }
            }
        }
        f8 = a8 * x;
        if !p6.is_null() && a8 < read_f64(rdata::F64_0_6) {
            *p6 |= 0x800000;
        }
    }
    // young player whose contract ends before the end of the current club’s season: does not leave
    if d4 != 0 && d4 != p2.addr() {
        let s7 = read_i16(p1.player().addr() + 7);
        if s7 > 0x82 && s7 as i32 > read_i16(d4 + club::REPUTATION) as i32 / 60 && age < 0x1a
            && exe::key_nation_club_in_season(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let year = u16::from_le_bytes([data_b[2], data_b[3]]);
            let day = i16::from_le_bytes([data_b[0], data_b[1]]);
            let year_c = read_u16(c + contract::CONTRACT_EXPIRES_YEAR);
            let before = year < year_c || (year == year_c && day < read_i16(c + contract::CONTRACT_EXPIRES));
            if before && read_usize(c + contract::GRIEVANCES) == 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) == 0
                && !(exe::human_manager_is_human_club(p2, 1, 1, 0, 0) != 0 && p1.pressure() <= 0xf)
                && exe::date_days_since(data_b.as_mut_ptr() as usize, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 0x96
            {
                fl(0x80000000);
                return zero;
            }
        }
    }
    if p3 == 0 || p4b || f8 < read_f64(rdata::F64_1_75) {
        return f8;
    }
    // the salary: the default (in a local STAFF_CONTRACT) against the club’s ceiling
    covered!("cqe_4 wage");
    let mut contract_local = [0u32; 0x14];
    // 0x85e410: lea ecx, [base+0xa0] -> thiscall, ret 0
    exe::new_staff_contract(contract_local.as_mut_ptr() as usize);
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push status; push 0; push club; push person; lea ecx, contract -> thiscall, ret 0x18
    let w0 = exe::staff_contract_default_staff_contract(contract_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situation; push person; mov ecx, finances -> thiscall, ret 8
    let a8w = exe::finance_get_max_wage(fin, p1, p5) as f64;
    let mut e8w = 0.0f64;
    let base = if !(read_f64(rdata::F64_0_1) <= f8) { read_f64(rdata::F64_0_1) } else { f8 };
    let mut q = read_f64(rdata::F64_3_5) / base; // cqe4: the asking price by interest
    if q < read_f64(rdata::F64_0_9) {
        q = f64::from_bits(0x3feccccccccccccd);
    }
    let mut wv = q * w0;
    if c != 0 {
        if a0 & 0x100 != 0 {
            e8w = 10.0;
        }
        if exe::staff_is_transfer_listed_or_requested(p1, d4) != 0 {
            e8w = e8w + read_f64(rdata::F64_10);
        }
        if read_i8(c + contract::HAPPINESS) < 0 {
            e8w = e8w - read_i8(c + contract::HAPPINESS) as i32 as f64 * read_f64(rdata::F64_0_2);
        }
        if a0 != 0 {
            e8w = e8w - read_i8(c + contract::MORALE) as i32 as f64 * read_f64(rdata::F64_0_2);
        }
        if !(zero <= e8w) {
            e8w = 0.0;
        } else if read_f64(rdata::F64_20) < e8w {
            e8w = 20.0;
        }
    }
    let mut f = f8;
    if !(wv * read_f64(rdata::F64_0_65) <= a8w) {
        return zero;
    }
    if !(wv <= a8w) {
        let mut t = wv * read_f64(rdata::F64_0_5);
        if !(a8w <= t) {
            t = a8w;
        }
        wv = (wv * (read_f64(rdata::F64_20) - e8w) + t * e8w) * read_f64(rdata::F64_0_05);
    }
    if !(wv <= a8w) && !(wv <= zero) {
        let r = a8w / wv;
        if r < read_f64(rdata::F64_0_7) {
            return read_f64(rdata::F64_0_1);
        }
        f = f * r;
        wv = a8w;
    }
    if c == 0 {
        return f;
    }
    let mut g = read_usize(c + contract::WAGE) as i32 as f64;
    if g < read_f64(rdata::F64_10) {
        g = read_f64(rdata::F64_10);
    }
    let mut h = if d4 == p2.addr() { g / wv } else { wv / g };
    if !(read_f64(rdata::F64_0_1) <= h) {
        h = read_f64(rdata::F64_0_1);
    } else if read_f64(rdata::F64_10) < h {
        return f * read_f64(rdata::F64_10);
    } else if !(h < read_f64(rdata::F64_1)) {
        return f * h;
    }
    f * ((((read_f64(rdata::F64_20) - e8w) * h + e8w) * read_f64(rdata::F64_0_05)) / q)
}

/// Branch counter for a hot routine: calls ramo only every 4096 passes.
pub(crate) fn count_hot_branch(n: &core::sync::atomic::AtomicU32, name: &'static str) {
    // simplification: hot routine; the branch counter counts in steps of 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        covered!(name);
    }
}

pub(crate) static BRANCH_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static HOT_BRANCH_BAND: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static FAR_BAND_HITS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// Tells whether the club takes the player by paying the compensation: outside the FIFA protected period, if the fee is
/// below his value; inside it, only with the Bosman rule and the fee up to the value, by the contract expiry band.
///
/// Convention: thiscall, ret `0x10`, `shortlist_manager.cpp`; `this` is not read.
/// `shortlist_manager_is_compensation_deal(slot, contract, club, &taxa)` (slot, contract, club, &fee), called by the
/// shortlist slot (`0x842e10`, `0x840000`), the scout report (`0x7b62a0`), the bid (`0x8e5d60`) and the screen. It
/// clears the fee; with no contract, no slot or with the contract's byte `.leaving_on_bosman()`: 1. The contract's
/// person (`[G_STAFF_TABLE]` + 110 × `[+0]`).
///
/// Outside the FIFA protected period (`exe::staff_contract_check_fifa_protected_period` zero): the compensation fee
/// (`exe::transfer_manager_calc_compensation_fee` with 0) and 1 if it is lower than the person's value
/// (`staff::VALUE`). Otherwise: no Bosman law at the player's club (`exe::transfer_manager_player_club_uses_bosman`),
/// 0; the fee (`exe::transfer_manager_calc_compensation_fee` with 1) above the value, 0; by the expiry band
/// (`exe::staff_contract_get_expiry_band`): 0 → 0; 1-2, 3-4, 5-6, 7-8 → 1 if the slot's `.valuation()` × 0.5 / 0.65 /
/// 0.8 / 0.9 exceeds the price (`.price()`); 9-14 → 1; above, 0.
/// Original: `shortlist_manager_is_compensation_deal` (0x84b6f0, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_is_compensation_deal(this: usize, slot: usize, contract: usize, club: usize, taxa: *mut i32) -> u32 {
    let slot = ShortlistSlot(slot);
    let contract = Contract(contract);
    let _ = this;
    *taxa = 0;
    if contract.is_null() || slot.is_null() || contract.leaving_on_bosman() != 0 {
        return 1;
    }
    let person = |c: usize| read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c).wrapping_mul(staff::SIZE));
    // 0x868210: push 0; push 0; push club; mov ecx, contract -> thiscall; 0x8d0780 and 0x8d0a70: mov ecx, [0xde1f54]
    // before the pushes (the copy passes ecx like the original; both only read the stack, ret 0xc and ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let manager = read_usize(G_TRANSFER_MANAGER);
    if exe::staff_contract_check_fifa_protected_period(contract, club, 0, 0) == 0 {
        let t = exe::transfer_manager_calc_compensation_fee(manager, person(contract.addr()), club, 0);
        *taxa = t;
        return (t < read_i32(person(contract.addr()) + staff::VALUE)) as u32;
    }
    if exe::transfer_manager_player_club_uses_bosman(manager, person(contract.addr())) == 0 { // cqd12: without the Bosman ruling
        return 0;
    }
    count_hot_branch(&BRANCH_BOSMAN, "cqd_12 com lei Bosman");
    let t = exe::transfer_manager_calc_compensation_fee(manager, person(contract.addr()), club, 1);
    *taxa = t;
    if t > read_i32(person(contract.addr()) + staff::VALUE) {
        return 0;
    }
    let k = exe::staff_contract_get_expiry_band(contract) as i8 as i32;
    if k as u32 > 0xe {
        return 0;
    }
    let factor = match k {
        0 => return 0,
        1 | 2 => 0.5, // cqd12: a faixa 1-2
        3 | 4 => 0.65,
        5 | 6 => 0.8,
        7 | 8 => 0.9,
        _ => {
            count_hot_branch(&FAR_BAND_HITS, "cqd_12 band longe");
            return 1; // cqd12: as faixas 9-14
        }
    };
    count_hot_branch(&HOT_BRANCH_BAND, "cqd_12 band pelo fator");
    // fild [place+0xb]; fild [place+3]; fmul factor; fcompp: 1 if +3 × factor exceeds +0xb (double precision)
    (slot.valuation() as f64 * factor > slot.price() as f64) as u32
}

/// qsort comparer of the 40 shortlist slots (`0x25` bytes) in the squad evaluation (`0x83cb40`): the difference of the
/// indices (`.index()`, signed byte); when equal, 1 (it never ties).
///
/// `shortlist_compare_slots_by_index(a, b)`.
/// Original: `shortlist_compare_slots_by_index` (0x83b6c0, shortlist_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let a = ShortlistSlot(a);
    let b = ShortlistSlot(b);
    let d = (a.index() as i32) - (b.index() as i32);
    if d != 0 { d } else { 1 }
}

/// Clubs interested in a player, collected by Shortlist_Manager collect_interested_clubs (`collect_interested_clubs`, mode 1) into a
/// temporary list.
pub(crate) unsafe fn interested_clubs(this: usize, player: usize) -> Vec<usize> {
    let mut l = [0u8; 0x10];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    exe::shortlist_manager_collect_interested_clubs(this, player, list, 0, 1, 0);
    let mut v = Vec::new();
    let mut in_the = list.head();
    while in_the != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(in_the) + 4));
        in_the = read_usize(in_the + llist_node::NEXT);
    }
    exe::drop_llist(list);
    v
}

/// Marks the player as under offer: his slot in the interested clubs' lists (except human clubs, the buyer or the clubs
/// of the exchange offers) gets bit `0x20`, which doubles the AI bid price.
///
/// Convention: thiscall, ret `0xc`, `shortlist_manager.cpp`, SEH frame for the local list
/// (`exe::new_llist`/`exe::drop_llist`). `Shortlist_Manager::mark_player_under_offer(player, buyer, swap)` (player,
/// buyer, exchange), called by the added negotiation (`0x8be9d0`) and the accepted bid (`0x8c0080`). The clubs
/// interested in the player (`exe::shortlist_manager_collect_interested_clubs` with mode 1, in a local list), except
/// those of a human who decides (`exe::human_manager_is_human_club` with 0, 1): with list `swap` (cursor at the head),
/// a club of any of its offers (`exe::transfer_manager_get_offer_slot` by the item id; club `+8` of the offer) is left
/// out; otherwise the player's slot (`exe::shortlist_manager_find_club_shortlist_entry`, only of type 11 if byte
/// `offer::JOB_TYPE` of the last offer found is 11) gets bit `0x20` (`shortlist_slot::FLAGS`); without the list, except
/// the buyer, the type-11 slot or, without it, the one of another type. A club with no slot ends it.
/// Original: `Shortlist_Manager::mark_player_under_offer` (0x84b580, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn mark_player_under_offer(this: usize, player: usize, buyer: usize, swap: usize) {
    let swap = Llist(swap);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    exe::shortlist_manager_collect_interested_clubs(this, player, list, 0, 1, 0);
    list.set_cursor(list.head());
    let mut last = player; // the original’s ebp: the player, then the last offer found (holds across clubs)
    let mut item = exe::llist_get_next(list);
    'clubes: while item != 0 {
        let k = read_usize(item + shortlist_pair::CLUB_ID);
        let club = read_usize(G_CLUB_TABLE).wrapping_add(k.wrapping_mul(club::SIZE));
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
            let v = if !swap.is_null() {
                swap.set_cursor(swap.head());
                let mut q = exe::llist_get_next(swap);
                while q != 0 {
                    last = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_usize(q) as i32);
                    if last != 0 && read_usize(read_usize(last) + 8) == k {
                        item = exe::llist_get_next(list);
                        continue 'clubes;
                    }
                    q = exe::llist_get_next(swap);
                }
                let kind = read_u8(read_usize(last) + offer::JOB_TYPE);
                exe::shortlist_manager_find_club_shortlist_entry(this, club, player, (kind != 0xb) as u32, 0)
            } else {
                if club == buyer {
                    item = exe::llist_get_next(list);
                    continue 'clubes;
                }
                let v = exe::shortlist_manager_find_club_shortlist_entry(this, club, player, 0, 0);
                if v != 0 { v } else { exe::shortlist_manager_find_club_shortlist_entry(this, club, player, 1, 0) }
            };
            if v == 0 {
                break;
            }
            *((v + shortlist_slot::FLAGS) as *mut u8) |= 0x20;
        }
        item = exe::llist_get_next(list);
    }
    exe::drop_llist(list);
}

/// The valueless end of `value_player_for_club`: for a loan slot (bit 0 of +0x24) and a loan-listed person, raises the rating to max(r
/// + 5000, r x 1.2); returns 0.
pub(crate) unsafe fn loan_listed_value_end(p: Staff, slot: usize, out: *mut i32) -> i32 {
    let slot = ShortlistSlot(slot);
    if slot.flags() & 1 == 0 {
        return 0;
    }
    if exe::staff_is_loan_listed(p, 0) != 0 {
        let r = *out;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * read_f64(rdata::F64_1_2);
        *out = ftol(if a > q { a } else { q });
    }
    0
}

/// Adjusts young players' ratings by the ability seen by the club's manager (slot types 1, 2 and 3 at +0x24 >> 1).
///
/// Under 27 (person +0x18) and with a rating above 20000: the current ability seen by the club's manager (`perceived_ability` with
/// 1 and rating/4) × 220 / rating, capped at `frac` + 0.6 and pulled towards 1 at ages 24, 25 and 26; with 1.05 ×
/// minimum below the rating, or for the high-rating slot (`alta`), above 1 it is further scaled by the club's
/// reputation in table [this+0xdc] (9 bytes per club, word +5) over the club's reputation (+0x80, at least 1000),
/// between 1 and 1.2. Above 1: `frac` × it, and the base value becomes rating × (it + 1) / 2.
pub(crate) unsafe fn young_player_adjustment(this: usize, club: Club, p: Staff, out: *mut i32, minimum: i32, alta: bool, age: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if age >= 0x1b || r <= 20000 {
        return;
    }
    let sv = exe::match_man_judge_player_ability(club.manager().addr(), p, club, (1) as i8, ((r / 4) as u32) as i16) as i32;
    let o = *out as f64;
    let mut d1 = sv as f64 * read_f64(rdata::F64_220) / o;
    let ceiling = *frac + read_f64(rdata::F64_0_6);
    if !(d1 < ceiling) {
        d1 = ceiling;
    }
    match age {
        0x18 => d1 = d1 * read_f64(rdata::F64_0_7) + read_f64(rdata::F64_0_3),
        0x19 => d1 = (d1 + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5),
        0x1a => d1 = d1 * read_f64(rdata::F64_0_3) + read_f64(rdata::F64_0_7),
        _ => {}
    }
    if minimum as f64 * read_f64(rdata::F64_1_05) < o || alta {
        if !(d1 > read_f64(rdata::F64_1)) {
            return;
        }
        let a = club.reputation();
        let ra = if a < 1000 { 1000 } else { a as i32 };
        let nr = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(club.addr()).wrapping_mul(9)) + 5) as i32;
        let mut x = nr as f64 / ra as f64;
        if !(read_f64(rdata::F64_1) <= x) {
            x = read_f64(rdata::F64_1);
        } else if read_f64(rdata::F64_1_2) < x {
            x = read_f64(rdata::F64_1_2);
        }
        d1 = d1 * x;
    }
    if d1 > read_f64(rdata::F64_1) {
        covered!("cqa_7 valued youngster");
        *frac = d1 * *frac;
        let h = (d1 + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5);
        *e = ftol(*e as f64 * h) as u32;
        *out = ftol(o * h);
    }
}

/// Values the player for the club: the player's rating for the slot (through the club's best observer, the scout list
/// at a human club, the fog of war) and the fee the club would pay (base value, age, young players' perceived ability,
/// injury proneness, the caps by position, reputation, record, age, division and the club's nation). Returns the price
/// (_ftol).
///
/// Convention: thiscall, ret `0x18`, `shortlist_manager.cpp`, 4.6 KB of x87; the map gives 1332 bytes.
/// `Shortlist_Manager::value_player_for_club(club, p, slot, out, minimum, ceiling)` (club, person, slot, output,
/// minimum, cap), called from the scouting lists and the computer's offers (`0x83f3a0`, `0x842e10`, `0x843800`,
/// `0x8c0810`, `0x8c8090`, `0x8c8860`, `0x8e5d60`).
///
/// The observer is the club's manager (`.manager()`; without sheet `staff::NON_PLAYER`, a random one,
/// `exe::utils_random` over `[G_NON_PLAYER_COUNT]`, at `[G_NON_PLAYER_TABLE]` + `0x44` × i). The rating flags by slot
/// type (`.flags()` & `0x1e`: 4 and 6 with `0x5882`, or `0x815882` for the human club's own player, and the reputation
/// scope `exe::database_get_reputation_scope`; 8 with `0x5982` and the scope; otherwise `0x5902`); a loan-list slot
/// (bit 0, `shortlist_slot_flags::LOAN_SLOT`) changes the minimum's divisor to 16 or 6 (`exe::staff_is_loan_listed`).
/// Human club (`exe::human_manager_is_human_club` with 1, 1): its manager (`FN_DATABASE_GET_CLUB_MANAGER`) and, with
/// the player on the scout's list (`exe::scout_manager_is_player_shortlisted`), the rating
/// (`exe::match_man_score_player_for_selection` with −1 and the replacement `exe::human_manager_get_stand_in_manager`)
/// becomes max(rating + 15000, rating × 1.5); otherwise the rating with minimum² × 0.000025 / divisor. Computer: with
/// flag `0x100` (or `0x80`) the best observer between the manager and the 7 scouts (`.scouts(k)`) by knowledge
/// (`exe::non_player_attribute_rating` with 6 or 5) minus 5 or 10 by the scope; the rating with the minimum and that
/// observer, × 1.05 for a player transfer-listed by another club (`exe::staff_is_transfer_listed_or_requested`). A
/// negative rating or 10000 below the minimum, or a loan-list slot: the end with no value (`loan_listed_value_end`).
///
/// The fog (`staff_state::STATUS_FLAGS` of the 79-byte record in `[G_STAFF_STATE_TABLE]`, 4 bits
/// `staff_state_status_flags::AI_SHORTLIST_COUNT`) limits the rating to rating + 5000 by (1 + n²/100) × rating. By
/// position (`exe::shortlist_position_mask_to_index`) the price cap and the fraction; the base value (high with rating
/// ≥ 35000, the slot's `.price()` ≥ rating/8 and `.position()` ≥ `0x80`); the adjustment by slot type (1, 2, 3: the
/// young players, `young_player_adjustment`; 3, 4, 5, 6: fractions); the age above the divisor (28, 32 with no
/// position, 19 in slot 5) and the foreigner (`exe::database_is_staff_foreign`); the price by injury proneness
/// (`.injury_proneness()` of sheet `.player()`) and the cap by slot type, reputation, record (`+0xd`) and age; above
/// 1.5 million, a perceived potential ability (`FN_MATCH_MAN_JUDGE_PLAYER_ABILITY` with 2) below 145 pulls it to 1.5
/// million; the limit of the club's division (`.division()`, `.reputation()` < 16); the club's nation (`.nation()`)
/// `[KEY_NATION_PORTUGAL]` or `[KEY_NATION_FRANCE]`; with `ceiling`, at most the slot's `+7` (at least 1).
/// Original: `Shortlist_Manager::value_player_for_club` (0x840d80, shortlist_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn value_player_for_club(this: usize, club: usize, p: usize, slot: usize, out: *mut i32, minimum: i32, ceiling: u32) -> i32 {
    let slot = ShortlistSlot(slot);
    let p = Staff(p);
    let club = Club(club);
    let mut obs = club.manager().addr();
    let t7 = read_usize(slot.addr() + 7) as i32;
    let ceiling_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && read_usize(obs + staff::NON_PLAYER) == 0 {
        let i = exe::utils_random(read_usize(G_NON_PLAYER_COUNT) as i32);
        write_usize(obs + staff::NON_PLAYER, read_usize(G_NON_PLAYER_TABLE).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // simplification: the original pushes the position word with the high word of ecx (garbage); the receiver reads only the word
    let pos = slot.position() as u16 as u32;
    let kind = slot.flags() & 0x1e;
    let flags: u32 = if kind == 4 || kind == 6 {
        let mut f = 0x5882;
        if slot.target_staff_id() == read_usize(p.addr()) && p.club_job() == club && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
            f = 0x815882;
        }
        match exe::database_get_reputation_scope(p, club.manager().addr()) as i8 {
            0 => f | 0x40,
            1 => f | 0x50,
            2 => f | 0x10,
            3 => f | 0x30,
            4 => f | 0x20,
            _ => f,
        }
    } else if kind == 8 {
        match exe::database_get_reputation_scope(p, club.manager().addr()) as i8 {
            0 => 0x59c2,
            1 => 0x59d2,
            2 => 0x5992,
            3 => 0x59b2,
            4 => 0x59a2,
            _ => 0x5982,
        }
    } else {
        0x5902
    };
    if slot.flags() & 1 != 0 {
        div = if exe::staff_is_loan_listed(p, 0) != 0 { 0x10 } else { 6 };
    }
    // the minimum² × 0.000025 / divisor (_ftol)
    let cut = |m: i32, div: i8| {
        let q = m as f64;
        ftol(q * read_f64(rdata::F64_2_5ENEG_05) * q / div as f64)
    };
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
        let m = manager(club.addr());
        if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), p, m) != 0 {
            let s = exe::human_manager_get_stand_in_manager(club);
            let r = exe::match_man_score_player_for_selection(p, pos, flags, club, -1, s);
            *out = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * read_f64(rdata::F64_1_5);
            *out = ftol(if a > q { a } else { q });
        } else {
            let s = exe::human_manager_get_stand_in_manager(club);
            let v = cut(minimum, div);
            *out = exe::match_man_score_player_for_selection(p, pos, flags, club, v, s);
        }
    } else {
        if flags & 0x100 != 0 || flags & 0x80 != 0 {
            let level: i8 = if flags & 0x100 != 0 { 6 } else { 5 };
            let mut best: i8 = 0;
            if flags & 0x100 != 0 {
                if obs != 0 && read_usize(obs + staff::NON_PLAYER) != 0 {
                    best = exe::non_player_attribute_rating(read_usize(obs + staff::NON_PLAYER), 6) as i8;
                    match exe::database_get_reputation_scope(p, obs) as i8 {
                        3 => best = best.wrapping_sub(5),
                        4 => best = best.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // mark 0x80: the manager comes in with +5 (−5 and +0 by scope 4 and 3)
                best = (exe::non_player_attribute_rating(read_usize(obs + staff::NON_PLAYER), 5) as i8).wrapping_add(5);
                match exe::database_get_reputation_scope(p, obs) as i8 {
                    3 => best = best.wrapping_sub(5),
                    4 => best = best.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = club.scouts(k);
                if s == 0 || read_usize(s + staff::NON_PLAYER) == 0 {
                    continue;
                }
                let mut v = exe::non_player_attribute_rating(read_usize(s + staff::NON_PLAYER), level) as i8;
                match exe::database_get_reputation_scope(p, s) as i8 {
                    3 => {
                        if level == 6 && club.reputation() < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if level == 6 && club.reputation() < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(10);
                    }
                    _ => {}
                }
                if v > best {
                    best = v;
                    obs = s;
                }
            }
        }
        let v = if minimum > -1 { cut(minimum, div) } else { -1 };
        *out = exe::match_man_score_player_for_selection(p, pos, flags, club, v, obs);
        if club != p.club_job() && exe::staff_is_transfer_listed_or_requested(p, 0) != 0 {
            covered!("cqa_7 transfer listed");
            *out = ftol(*out as f64 * read_f64(rdata::F64_1_05)); // cqa7: the listed player of another club × 1.05
        }
    }
    let r = *out;
    if r < 0 || r < minimum.wrapping_sub(10000) || slot.flags() & shortlist_slot_flags::LOAN_SLOT != 0 {
        return loan_listed_value_end(p, slot.addr(), out);
    }
    let nevoa = read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::AI_SHORTLIST_COUNT;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (game_pow(nevoa as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_1)) * r as f64;
        *out = ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let c = exe::shortlist_position_mask_to_index(this, pos) as i8 as i32;
    let (teto_pos, mut frac) = match c {
        1..=4 | 17 => (8000000.0, 0.6),
        5..=7 | 19 => (9000000.0, 0.65),
        8..=10 | 18 => (10000000.0, 0.75),
        11..=13 | 21 => (11000000.0, 0.8),
        14 | 20 => (13000000.0, 1.0),
        _ => {
            div = 0x20;
            (6000000.0, 0.575)
        }
    };
    let r = *out;
    let bv = (slot.price() as usize) as i32;
    let alta = r >= 35000 && bv >= r / 8 && slot.position() >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        ftol(((v * read_f64(rdata::F64_5ENEG_05) - read_f64(rdata::F64_1)) * v + bv as f64 * read_f64(rdata::F64_0_25)) * read_f64(rdata::F64_0_32)) as u32
    } else {
        covered!("cqa_7 base value");
        ftol((bv as f64 * read_f64(rdata::F64_0_25) + r as f64 * read_f64(rdata::F64_0_75)) * read_f64(rdata::F64_0_32)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let age = (p.age() as u8) as i8;
    match (slot.flags() >> 1) & 0xf {
        1 | 2 => young_player_adjustment(this, club, p, out, minimum, alta, age, r, &mut frac, &mut e),
        3 => {
            let k = if slot.position() < 0x80 { read_f64(rdata::F64_0_6666666667) } else { read_f64(rdata::F64_0_9090909091) };
            e = ftol(e as f64 * k) as u32;
            young_player_adjustment(this, club, p, out, minimum, alta, age, r, &mut frac, &mut e);
        }
        4 => e = ftol(e as f64 * read_f64(rdata::F64_0_5)) as u32,
        5 => {
            e = ftol(e as f64 * read_f64(rdata::F64_0_3333333333)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = ftol(e as f64 * read_f64(rdata::F64_0_2)) as u32,
        _ => {}
    }
    let kind = slot.flags() & 0x1e;
    if age > div {
        let d = age as i32 - div as i32;
        let dd = d as f64;
        let sheet = p.player();
        let f = read_f64(rdata::F64_1) / (dd * read_f64(rdata::F64_0_3) + read_f64(rdata::F64_0_8)); // cqa7: the age above the divisor
        let ts = sheet.world_reputation();
        let g = if ts > 0x1770 { ts as i32 as f64 * read_f64(rdata::F64_0_00025) - read_f64(rdata::F64_1_5) } else { 0.0 };
        let y = if exe::database_is_staff_foreign(p, club.nation().addr(), 1) != 0 {
            f / (dd * read_f64(rdata::F64_0_15) + read_f64(rdata::F64_0_9))
        } else if age as i32 > div as i32 + 2 && (kind == 8 || kind == 6) && sheet.current_reputation() < club.reputation() {
            (read_f64(rdata::F64_1) - (d - 2) as f64 * read_f64(rdata::F64_0_06666666667)) * f
        } else {
            f
        };
        covered!("cqa_7 age");
        let um = read_f64(rdata::F64_1);
        if kind == 0xa {
            *out = ftol(((um - g) * y + g) * *out as f64);
        }
        e = ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let sheet = p.player();
    let cons = (sheet.injury_proneness() as u8) as i8 as f64;
    let c = ftol((read_f64(rdata::F64_1) - game_pow(cons, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_0001666666667)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * read_f64(rdata::F64_1ENEG_08);
    let ra = club.reputation();
    if kind == 0xc {
        if ra > 5000 {
            let lim = (ra as i32 - 5000) as f64 * read_f64(rdata::F64_20) + read_f64(rdata::F64_50000);
            if !(kk <= lim) {
                kk = (ra as i32 - 100000) as f64 + read_f64(rdata::F64_50000);
            }
        } else {
            let lim = (ra as i32 * 10) as f64;
            if !(kk <= lim) {
                kk = ra as f64 * read_f64(rdata::F64_10);
            }
        }
    } else if kind == 0xa {
        let lim = if ra > 5000 {
            let z = (ra as i32 - 5000) as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_500);
            z * z
        } else {
            let z = ra as f64;
            z * z * read_f64(rdata::F64_0_01_AT_96F910)
        };
        if !(kk <= lim) {
            kk = lim;
        }
    }
    let ts = sheet.world_reputation();
    if ts <= 5000 {
        let lim = frac * read_f64(rdata::F64_10000000);
        if lim < kk {
            kk = lim;
        }
    } else if ts <= 8000 {
        let lim = ts as f64 * frac * read_f64(rdata::F64_1666_67) + read_f64(rdata::F64_1650000);
        if lim < kk {
            kk = lim;
        }
    } else if ts < 9000 {
        let lim = (ts as f64 * read_f64(rdata::F64_7500) - read_f64(rdata::F64_45000000)) * frac;
        if lim < kk {
            kk = lim;
        }
        frac = frac * read_f64(rdata::F64_1_5);
    } else {
        kk = read_f64(rdata::F64_100000000);
        frac = frac * read_f64(rdata::F64_2_5);
    }
    let by_age = match age {
        0x10 => Some(0x96f8d8),
        0x11 => Some(0x968e90),
        0x12 | 0x1f => Some(0x96d850),
        0x13 | 0x1d => Some(0x96f8e0),
        0x1c => Some(0x96a630),
        0x1e => Some(0x96f908),
        0x20 => Some(0x9688c8),
        0x21 => Some(0x96a650),
        0x22 => Some(0x9688d8),
        0x23 => Some(0x96f8e8),
        0x24..=0x28 => Some(0x968870),
        _ => None,
    };
    if let Some(a) = by_age {
        let t = frac * read_f64(a);
        if t < kk {
            kk = t;
        }
    }
    let base = read_f64(rdata::F64_1500000);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
        let n2 = vista(club.manager().addr(), p.addr(), club.addr(), 2, (*out / 4) as u32) as i32 as f64;
        if n2 < read_f64(rdata::F64_145) {
            let mut a = n2 * read_f64(rdata::F64_0_025) - read_f64(rdata::F64_2_625);
            if !(read_f64(rdata::F64_0) <= a) {
                a = read_f64(rdata::F64_0);
            }
            x = (kk - base) * a + base;
        }
    }
    let bc = club.division();
    if !bc.is_null() && bc.reputation() < 0x10 {
        let b69 = bc.reputation() as i32;
        let ri = club.reputation() as i32 / 500;
        let m = if ri > b69 { ri } else { b69 };
        let mut lim = m.wrapping_mul(670000);
        if lim <= 1000000 {
            lim = 1000000;
        }
        let ts = sheet.world_reputation() as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if read_f64(rdata::F64_1) < frac { read_f64(rdata::F64_1) } else { frac };
            x = ftol(l * m2) as f64;
        }
    }
    // the club’s nation (+0x53 of the club: the original uses edi, which here is already the club)
    let nation = read_usize(club.nation().addr());
    if nation == read_usize(KEY_NATION_PORTUGAL) {
        if x < read_f64(rdata::F64_6000000) {
            x = x * (x * read_f64(rdata::F64_1_666666667ENEG_07) * read_f64(rdata::F64_0_6) + read_f64(rdata::F64_0_3));
        } else {
            x = x * read_f64(rdata::F64_0_9);
        }
    } else if nation == read_usize(KEY_NATION_FRANCE) {
        x = ftol(x * read_f64(rdata::F64_0_8)) as f64;
    }
    if ceiling != 0 {
        let c = ceiling_i as f64;
        if c < x {
            x = c;
        }
        x = ftol(x) as f64;
    }
    ftol(x)
}

/// CONTRACT_MANAGER get_contract (`exe::contract_manager_get_contract`) for a person, without storing it.
pub(crate) unsafe fn contract_of_staff(person: usize) -> Contract {
    Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person))
}

/// As datas estáticas da `exe::date_add_days`, da 0x54c8e0 e da `exe::date_days_since` (valor e guarda).
pub(crate) const SHORTLIST_MANAGER_DATE_STATICS: [(usize, usize); 6] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Counts the club's shortlist slots with a target and a bid at the given position (`0xff`: any); with the list in
/// state 2 for a human club, a slot whose manager arrived over 20 days ago and whose target has no offers loses its
/// target and bid.
///
/// `Shortlist_Manager::count_slots_with_bids(club, position)`, called by the list evaluation (`0x84a020`, with `0xb`),
/// the squad evaluation (`0x83cb40`, with −1) and the day loop (`0x5cccd0`, with −1). Of the 40 slots of the club's
/// list, it counts those with a target (`shortlist_slot::TARGET_STAFF_ID`) and a bid (`shortlist_slot::OFFER_REF`),
/// both above −1, and position (`+0`) equal to `position` (`0xff`: any). With the list in state 2
/// (`club_shortlist::ACTIVE` of the entry) and the human's club (`exe::human_manager_is_human_club` with 1, 1): if
/// today − 20 days (`exe::date_sub_days`) is past the manager's arrival date (`+0x3e` of the record at the club's
/// `.manager()`) and the target has no offers (`exe::transfer_manager_get_offers_for_player`), the slot loses its
/// target and bid (−1) and does not count. The count is a byte.
/// Original: `Shortlist_Manager::count_slots_with_bids` (0x844540, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn count_slots_with_bids(this: usize, club: usize, position: u32) -> u8 {
    let club = Club(club);
    // 0x600a50: cdecl(club, 1, 1, 0, 0) (add esp, 0x14 afterwards); 0x54c8e0: thiscall(0xae2c90, output, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], person, 0, 0)
    let entry = shortlist_club_entry(this, club.addr());
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = read_usize(entry).wrapping_add(i * 0x25);
        if (read_usize(v + shortlist_slot::TARGET_STAFF_ID) as i32) <= -1 {
            continue;
        }
        if (read_usize(v + shortlist_slot::OFFER_REF) as i32) <= -1 {
            covered!("cpy_1 target without bid");
            continue; // cpy1: a target without a bid does not count
        }
        if position as u8 != 0xff && read_u8(v) != position as u8 {
            covered!("cpy_1 other position");
            continue; // cpy1: a bid for another position does not count
        }
        if read_u8(entry + club_shortlist::ACTIVE) == 2 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = exe::date_sub_days(G_CURRENT_DATE, (&mut buf) as *mut [u8; 8] as usize, 0x14);
            let day = read_i16(r);
            let year = read_u16(r + cm_date::YEAR);
            let arrival = club.manager().addr().wrapping_add(0x3e);
            let year_c = read_u16(arrival + cm_date::YEAR);
            let old = year > year_c || (year == year_c && day > read_i16(arrival));
            if old {
                let target = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(v + shortlist_slot::TARGET_STAFF_ID).wrapping_mul(staff::SIZE));
                if exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), target, 0, 0) == 0 {
                    covered!("cpy_1 bid expired");
                    write_i32(v + shortlist_slot::TARGET_STAFF_ID, -1); // cpy1: the slot loses the target
                    write_i32(v + shortlist_slot::OFFER_REF, -1);
                    continue;
                }
            }
        }
        covered!("cpy_1 slot with bid");
        n = n.wrapping_add(1); // cpy1: the slot with a bid counts
    }
    n
}

/// Removes the person from the transfer list: clears bit 0 of the contract's `.transfer_status()`; returns 1 with a
/// contract, 0 without.
///
/// `shortlist_manager_remove_from_transfer_list(person)`, called by the transfer screens (`0x8f6770`, `0x8f8120`). No
/// contract (`contract_of_staff`): 0. With one, bit 0 of `.transfer_status()` (on the transfer list) is cleared and it
/// returns 1 in the low byte (the upper ones are those of the contract pointer, as in the original).
/// Original: `shortlist_manager_remove_from_transfer_list` (0x844280, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_transfer_list(person: usize) -> u32 {
    let c = Contract((contract_of_staff(person)).addr());
    if c.is_null() {
        return 0;
    }
    c.set_transfer_status(c.transfer_status() & 0xfe);
    (c.addr() as u32 & !0xff) | 1
}

/// Removes the person from the loan list: clears bit 1 of the contract's `.transfer_status()`; returns 1 with a
/// contract, 0 without.
///
/// `shortlist_manager_remove_from_loan_list(person)`, called by the contract renewal (`0x4dc3a0`) and the transfer
/// screens. No contract (`contract_of_staff`): 0; with one, bit 1 of `.transfer_status()` (on the loan list) is cleared
/// and it returns 1 in the low byte (the upper ones are those of the contract pointer).
/// Original: `shortlist_manager_remove_from_loan_list` (0x8442d0, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_loan_list(person: usize) -> u32 {
    let c = Contract((contract_of_staff(person)).addr());
    if c.is_null() {
        return 0;
    }
    c.set_transfer_status(c.transfer_status() & 0xfd);
    (c.addr() as u32 & !0xff) | 1
}

/// As datas estáticas da `exe::date_add_days`, da 0x54c8e0 e da `exe::date_days_since` (valor e guarda).
pub(crate) const STAFF_STATUS_DATE_STATICS: [(usize, usize); 6] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Removes the (person, club) pair from one of the Shortlist_Manager's sorted 8-byte pair vectors by binary search;
/// returns whether it was found.
///
/// `base` = +0 or +4, count at `base` + 0x10; the search is `exe::crt_bsearch` (cdecl(key, vector, count, 8, comparison), add
/// esp, 0x14). The following pairs move down one place, the last one is zeroed and the count drops.
pub(crate) unsafe fn remove_sorted_pair(base: usize, key: &[u32; 2], comparar: usize) -> bool {
    let p = exe::crt_bsearch((key.as_ptr()) as usize, read_usize(base), read_usize(base + 0x10) as u32, 8, comparar);
    if p == 0 {
        return false;
    }
    let mut i = (p.wrapping_sub(read_usize(base)) >> 3) as i32;
    while i < read_usize(base + 0x10) as i32 - 1 {
        let v = read_usize(base);
        i += 1;
        let k = v.wrapping_add(i as usize * 8);
        write_u32(k - 8, read_usize(k) as u32);
        write_u32(k - 4, read_usize(k + 4) as u32);
    }
    let end = read_usize(base).wrapping_add(read_usize(base + 0x10).wrapping_mul(8));
    write_u32(end - 4, 0);
    write_u32(end - 8, 0);
    write_u32(base + 0x10, (read_usize(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// Sets the slot date (+0x1b) to today plus or minus `dias` (the given Date routine).
pub(crate) unsafe fn set_slot_date(v: usize, routine: usize, days: u32) {
    let v = ShortlistSlot(v);
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(routine);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(G_CURRENT_DATE, &mut a, days);
    exe::date_operator_3d(v.addr() + shortlist_slot::DATE, (&mut b) as *mut [u8; 8] as usize, r);
}

/// Blocks a slot for a person until the slot date: stores the person, clears bid and values, writes the slot status
/// into the contract's squad status, entry +8 = 100.
pub(crate) unsafe fn block_slot_for_person(v: usize, entry: usize, person: Staff, contract: Contract, routine: usize, days: u32) {
    set_slot_date(v, routine, days);
    write_u32(v + shortlist_slot::REPLACEMENT_STAFF_ID, read_usize(person.addr()) as u32);
    write_u32(v + shortlist_slot::OFFER_REF, 0xffff_ffff);
    write_u32(v + shortlist_slot::PRICE, 0);
    write_u32(v + 7, 0);
    if !contract.is_null() {
        let c = (contract.addr() + contract::SQUAD_STATUS) as *mut u8;
        *c = (*c & 0xf) | (((read_u8(v + shortlist_slot::FLAGS) >> 1) & 0xf) << 4);
    }
    write_u8(entry + club_shortlist::DAYS, 0x64);
}

/// Applies a transfer event for the person to the club's shortlist: for event 4 at his own club, event 1 to every
/// interested club; the (person, club) pair leaves the sorted arrays and the person's shortlist count drops; in the
/// slots holding the person, 1 refills the slot, 2 and 5 block for 120 and 14 days, 3 and 4 release with today − 120.
/// Returns 1.
///
/// Convention: thiscall, ret `0x10`, `shortlist_manager.cpp`, SEH frame for the local LLIST.
/// `Shortlist_Manager::update_staff_status(person, club, event, kind)` (person, club, event, type), called by the
/// offers, contracts, managers and regens (`0x4d07e0`, `0x690730`, `0x7a59e0`, `0x8bf680`, `0x8c44e0`...) and by
/// itself.
///
/// With event 4 and a null club or the person's own (`.club_job()`): each interested club
/// (`exe::shortlist_manager_collect_interested_clubs` with 0, 3, 0) receives event 1 with type 2 (recursion). Null
/// club: 1. With event 5, or 1..3 from another club: the (person, club) pair leaves the two sorted arrays (`+0`/`+0x10`
/// via `0x757ea0`, `shortlist_manager_fields::PAIRS_BY_CLUB`/`+0x14` via `0x844740`; null array: 0); if found in both
/// and the club is not human (`FN_HUMAN_MANAGER_IS_HUMAN_CLUB` with 0, 1), the low nibble of
/// `staff_state::STATUS_FLAGS` of the person's 79-byte record (`[G_STAFF_STATE_TABLE]`) drops by one.
///
/// Then, in the club's 40 slots (type 1: except player slots `0xb`; type 0: only those) with the person as target
/// (`shortlist_slot::TARGET_STAFF_ID`; on event 4, also at `shortlist_slot::REPLACEMENT_STAFF_ID`): 1 = the slot is
/// emptied and filled again (`exe::shortlist_manager_fill_shortlist_slot`); 2 and 5 = a block of 120 and 14 days
/// (`0x54c5b0`); 3 = emptied, date today − 120 (`0x54c8e0`); 4 = releases `shortlist_slot::REPLACEMENT_STAFF_ID` and
/// the target, date today − 120.
/// Original: `Shortlist_Manager::update_staff_status` (0x843e10, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_staff_status(this: usize, person: usize, club: usize, event: u32, kind: u32) -> u8 {
    let person = Staff(person);
    // 0x68d020/0x68d040/0x68d1b0: thiscall(list); 0x842180: thiscall(this, person, list, 0, 3, 0) (5 pushes); the
    // recursion: thiscall(this, person, club, 1, 2) (4 pushes); 0x600a50: cdecl(club, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], person, club) (2 pushes); 0x83f3a0: thiscall(this, slot, club, person) (3 pushes)
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let ev = event as u8;
    let id = read_usize(person.addr());
    if (club == 0 || club == person.club_job().addr()) && ev == 4 {
        exe::shortlist_manager_collect_interested_clubs(this, person, l, 0, 3, 0);
        l.set_cursor(l.head());
        let mut e = exe::llist_get_next(l);
        while e != 0 {
            ((exe::shortlist_manager_update_staff_status(this, person, friendly_club_by_id(read_usize(e + shortlist_pair::CLUB_ID)), 1, 2) as usize) as u8);
            e = exe::llist_get_next(l);
        }
    }
    if club == 0 {
        exe::drop_llist(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && club != person.club_job().addr()) {
        if read_usize(this) == 0 || read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB) == 0 {
            exe::drop_llist(l);
            return 0;
        }
        let key = [id as u32, read_usize(club) as u32];
        let mut faltou = !remove_sorted_pair(this, &key, 0x757ea0);
        faltou |= !remove_sorted_pair(this + shortlist_manager_fields::PAIRS_BY_CLUB, &key, 0x844740);
        if human(club, 0, 1, 0, 0) == 0 && !faltou {
            covered!("cpy_2 list count");
            let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: one list fewer in the count of the person
        }
    }
    let entry = shortlist_club_entry(this, club);
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club);
    for i in 0..40usize {
        let v = read_usize(entry).wrapping_add(i * 0x25);
        let t = kind as u8;
        if t == 1 {
            if read_u8(v) == 0xb {
                continue;
            }
        } else if t == 0 && read_u8(v) != 0xb {
            continue;
        }
        if read_usize(v + shortlist_slot::TARGET_STAFF_ID) != id && (ev != 4 || read_usize(v + shortlist_slot::REPLACEMENT_STAFF_ID) != id) {
            continue;
        }
        match ev {
            1 => {
                write_u32(v + shortlist_slot::TARGET_STAFF_ID, 0xffff_ffff);
                write_u32(v + shortlist_slot::OFFER_REF, 0xffff_ffff);
                if read_usize(v + shortlist_slot::REPLACEMENT_STAFF_ID) == id {
                    write_u32(v + shortlist_slot::REPLACEMENT_STAFF_ID, 0xffff_ffff);
                }
                *((v + shortlist_slot::FLAGS) as *mut u8) &= 0xfe;
                covered!("cpy_2 slot redone");
                exe::shortlist_manager_fill_shortlist_slot(this, v, club, person); // cpy2: the slot is filled again
            }
            2 => block_slot_for_person(v, entry, person, Contract(contract), 0x54c5b0, 0x78),
            5 => {
                covered!("cpy_2 14-day block");
                block_slot_for_person(v, entry, person, Contract(contract), 0x54c5b0, 0xe); // cpy2: the 14-day block
            }
            3 => {
                write_u32(v + shortlist_slot::TARGET_STAFF_ID, 0xffff_ffff);
                write_u32(v + shortlist_slot::OFFER_REF, 0xffff_ffff);
                set_slot_date(v, 0x54c8e0, 0x78);
                if read_usize(v + shortlist_slot::REPLACEMENT_STAFF_ID) == id {
                    write_u32(v + shortlist_slot::REPLACEMENT_STAFF_ID, 0xffff_ffff);
                }
            }
            4 => {
                if read_usize(v + shortlist_slot::REPLACEMENT_STAFF_ID) == id {
                    write_u32(v + shortlist_slot::REPLACEMENT_STAFF_ID, 0xffff_ffff);
                }
                if read_usize(v + shortlist_slot::TARGET_STAFF_ID) == id {
                    write_u32(v + shortlist_slot::TARGET_STAFF_ID, 0xffff_ffff);
                    write_u32(v + shortlist_slot::OFFER_REF, 0xffff_ffff);
                    set_slot_date(v, 0x54c8e0, 0x78);
                }
            }
            _ => {}
        }
    }
    exe::drop_llist(l);
    1
}

/// True for the case that cannot run twice (event 4 recursion, or event 1 with the person targeted by a club slot).
pub(crate) unsafe fn is_original_only_case(this: usize, person: Staff, club: usize, event: u32, kind: u32) -> bool {
    let ev = event as u8;
    if ev == 4 && (club == 0 || club == person.club_job().addr()) {
        return true;
    }
    if ev != 1 || club == 0 {
        return false;
    }
    // from another club, event 1 goes through the vectors; either one being null returns 0 before the slots
    if club != person.club_job().addr() && (read_usize(this) == 0 || read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB) == 0) {
        return false;
    }
    let entry = shortlist_club_entry(this, club);
    (0..40usize).any(|i| {
        let v = ShortlistSlot(read_usize(entry).wrapping_add(i * 0x25));
        let ok = match kind as u8 {
            1 => read_u8(v.addr()) != 0xb,
            0 => read_u8(v.addr()) == 0xb,
            _ => true,
        };
        ok && v.target_staff_id() == read_usize(person.addr())
    })
}

/// Puts the person on the loan list: sets bit 1 of the person's contract `.transfer_status()`. Returns the eax the
/// original leaves: the contract.
///
/// `shortlist_manager_add_to_loan_list(person)`, called by the transfer screens (`0x8f6770`, `0x8f8120`). With a
/// contract (`exe::contract_manager_get_contract`, without storing it in `[0xae1080]`), bit 1 of `.transfer_status()`
/// (on the loan list) is set.
/// Original: `shortlist_manager_add_to_loan_list` (0x8442b0, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_add_to_loan_list(person: usize) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    if !c.is_null() {
        c.set_transfer_status(c.transfer_status() | 2);
    }
    c.addr()
}

/// Returns the club's fee limit for the player at the given squad status; −1 without a person or player sheet.
///
/// `shortlist_manager_max_fee_for_player_status(club, person, status)`, called by the contract screen (`0x4dd0f0`, ecx
/// = the Shortlist_Manager at `+0x856` of the transfer manager). No person or no player sheet (`.player()`): −1.
/// Otherwise the slot cap (`exe::shortlist_manager_max_fee_for_squad_status`) with the player's strong positions
/// (`exe::formation_get_position_mask`) and the status.
/// Original: `shortlist_manager_max_fee_for_player_status` (0x84b1f0, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_max_fee_for_player_status(this: usize, club: usize, person: usize, status: u32) -> i32 {
    let person = Staff(person);
    // 0x5b7d20: cdecl(record) (add esp, 4); 0x84b230: call with mov ecx, esi (the this) and 3 push (club, positions,
    // status), ret 0xc: thiscall from here (the engine’s stdcall copy does not read ecx)
    if person.is_null() || person.player().is_null() {
        return -1;
    }
    let m = exe::formation_get_position_mask(person.player()) as u32;
    exe::shortlist_manager_max_fee_for_squad_status(this, club, m, status)
}

/// Staff record of index i (0x6e bytes, 32-bit math including negative).
pub(crate) unsafe fn shortlist_manager_staff_by_index(i: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE))
}

/// Shares an AI club's transfer budget across the open shortlist slots (type and status factors), reduces it by held
/// bids and the remainder, fills each slot without a target and gives the remainder to the first own-club player slot
/// the manager would sell.
///
/// `Shortlist_Manager::allocate_transfer_budget(club)`, called by the update of the club's list (`0x83a730`, already in
/// Rust), the initial lists (`0x83a1b0`) and the rebuilt list (`0x84abe0`). Human club
/// (`exe::human_manager_is_human_club` with 0, 1), national team (`exe::database_is_national_team`), no finances
/// (`exe::financetable_get_club_finance`) or a list not in state 1 (`club_shortlist::ACTIVE`): nothing.
///
/// The budget (`exe::finance_get_transfer_budget` with 0, 0, 0, 1, 0; negative: 0) is split among the open slots
/// (target −1) after sorting the 40 (qsort `0x83b690`): among the first 8 (or up to the fourth open player slot), each
/// open slot receives at `+7` the budget / (open slots × the factor of the type and the status); the rest of the budget
/// goes through the slots still without a target (reductions by the held bids, by the value of `shortlist_slot::PRICE`
/// over the sum and by the factors at `rdata::F64_NEG_0_5`, `rdata::F64_0_5`), each one filled by
/// `exe::shortlist_manager_fill_shortlist_slot`; what is left goes to the first own-club player slot the manager would
/// accept selling (`exe::shortlist_manager_staff_interest_in_joining_club` above `rdata::F64_3_15`, random draws of 10
/// and 100 days, offers `exe::transfer_manager_get_offers_for_player`).
/// Original: `Shortlist_Manager::allocate_transfer_budget` (0x83b6e0, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn allocate_transfer_budget(this: usize, club: usize) {
    let club = Club(club);
    // 0x600a50: cdecl(club, 0, 1, 0, 0) (add esp, 0x14); 0x53b900: cdecl(club) (add esp, 4); 0x5a0590:
    // thiscall([0xae2a38], club); 0x598800: thiscall(finances, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vector, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // club, 0) (mov ecx, [this local]; 3 pushes); 0x4cf5a0: thiscall([0xae1050], person); 0x544590: cdecl(person, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], person, club) (2 pushes); 0x846920: thiscall(this, person, club, 1, 0,
    // status, 0) (6 pushes, returns in st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contract) (no push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], person, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, output, days)
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 || exe::database_is_national_team(club) != 0 {
        return;
    }
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if fin == 0 {
        return;
    }
    let mut budget = exe::finance_get_transfer_budget(fin, 0, 0, 0, 1, 0) as i32;
    if budget < 0 {
        budget = 0;
    }
    let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(club.addr()).wrapping_mul(9));
    if read_u8(entry + club_shortlist::ACTIVE) != 1 {
        return;
    }
    exe::crt_qsort(read_usize(entry), 0x28, 0x25, 0x83b690);
    let slot = |i: i32| read_usize(entry).wrapping_add((i as usize).wrapping_mul(0x25));
    // the open slots among the first 8, or up to the fourth open player slot
    let mut limit: i8 = 8;
    let mut abertas: i8 = 0;
    let mut players: u8 = 0;
    let mut sum: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = slot(i as i32);
        if read_i32(v + shortlist_slot::TARGET_STAFF_ID) == -1 {
            if read_u8(v) == 0xb {
                players = players.wrapping_add(1);
                if players == 4 {
                    limit = i.wrapping_add(1);
                }
            }
            sum = sum.wrapping_add(read_i32(v + shortlist_slot::PRICE));
            abertas = abertas.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        if i >= limit {
            break;
        }
    }
    let a = abertas as i32;
    let mut split: i32 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = slot(k);
            if read_i32(v + shortlist_slot::TARGET_STAFF_ID) != -1 {
                continue;
            }
            let t = read_u8(v);
            let divisor = if t == 0xb {
                match ((read_u8(v + shortlist_slot::FLAGS) >> 1) & 0xf).wrapping_sub(2) {
                    0 | 1 => {
                        covered!("cpy_3 funds for the starter slot");
                        Some(a * 2) // cpy3: the budget of the starter slot
                    }
                    2 => Some(a * 4),
                    3 => Some(a * 10),
                    _ => None,
                }
            } else {
                if read_i32(v + shortlist_slot::REPLACEMENT_STAFF_ID) != -1 {
                    continue;
                }
                let rico = club.professional_status() == professional_status::PROFESSIONAL;
                match (t as i8 as i32).wrapping_sub(6) {
                    0 | 2 => Some(if rico { a * 10 } else { 0 }),
                    1 => Some(if rico { a * 1000 } else { 0 }),
                    3 | 4 => Some(if rico { a * 20 } else { 0 }),
                    _ => None,
                }
            };
            let Some(dv) = divisor else { continue };
            let x = if dv == 0 { 0 } else { budget.wrapping_div(dv) };
            write_i32(v + 7, x);
            split = split.wrapping_add(x);
        }
    }
    let mut rest = budget.wrapping_sub(split);
    let mut baixos: i8 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = slot(k);
            let t = read_u8(v);
            if t != 0xb && read_i32(v + shortlist_slot::REPLACEMENT_STAFF_ID) > -1 {
                continue;
            }
            let target = read_i32(v + shortlist_slot::TARGET_STAFF_ID);
            if target > -1 || t == 7 {
                continue;
            }
            let lance = read_i32(v + shortlist_slot::OFFER_REF);
            if lance > -1 {
                if target == -1 {
                    write_i32(v + shortlist_slot::OFFER_REF, -1);
                } else if t == 0xb {
                    let l = Offer(read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), lance)));
                    if !l.is_null() {
                        rest = rest.wrapping_sub(l.fee() as i32);
                    }
                } else {
                    rest = rest.wrapping_sub(read_i32(shortlist_manager_staff_by_index(target) + staff::VALUE));
                }
                continue;
            }
            if club.professional_status() != professional_status::PROFESSIONAL && t != 0xb {
                write_i32(v + 7, 0);
            } else {
                let base = if split >= 1000 { split } else { 1000 };
                let e = read_i32(v + 7);
                let x = ftol((e as f64) / (base as f64) * (rest as f64) * read_f64(rdata::F64_NEG_0_5));
                covered!("cpy_3 reduction by surplus");
                write_i32(v + 7, e.wrapping_sub(x)); // cpy3: the slot reduction by the surplus
            }
            let mark = read_u8(v + shortlist_slot::FLAGS) & 0x1e;
            if mark <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if sum >= 1 { sum } else { 1 };
                    let b = read_i32(v + shortlist_slot::PRICE);
                    let b = if b >= 1 { b } else { 1 };
                    let x = ftol((rest as f64) / (s as f64) * (b as f64));
                    write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
                }
            } else {
                let s = if sum >= 1 { sum } else { 1 };
                let b = read_i32(v + shortlist_slot::PRICE);
                let b = if b >= 1 { b } else { 1 };
                let x = ftol((rest as f64) / (s as f64) * (b as f64) * read_f64(rdata::F64_0_5));
                write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
            }
            *((v + shortlist_slot::FLAGS) as *mut u8) &= 0xfe;
            covered!("cpy_3 slot filled");
            exe::shortlist_manager_fill_shortlist_slot(this, v, club, (0) as usize); // cpy3: the slot is filled
            let target = read_i32(v + shortlist_slot::TARGET_STAFF_ID);
            if target > -1 && read_u8(v + shortlist_slot::FLAGS) & 1 == 0 {
                rest = rest.wrapping_add(ftol(read_i32(shortlist_manager_staff_by_index(target) + staff::VALUE) as f64 * read_f64(rdata::F64_NEG_1_3)));
            }
            sum = sum.wrapping_sub(read_i32(v + shortlist_slot::PRICE));
        }
    }
    // the remainder goes to the first player slot of the club itself that the manager would agree to sell
    if limit >= 0x28 {
        return;
    }
    let mut k = limit;
    while rest > 0 {
        let v = slot(k as i32);
        let target = read_i32(v + shortlist_slot::TARGET_STAFF_ID);
        let candidata = 'c: {
            if read_u8(v) != 0xb || target <= -1 || read_i32(v + shortlist_slot::OFFER_REF) != -1 {
                break 'c false;
            }
            let p = Staff(shortlist_manager_staff_by_index(target));
            if p.club_job() != club {
                break 'c false;
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            write_usize(G_CONTRACT_LOOKUP_RESULT, c);
            let of_contract = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE)) } else { 0 };
            if of_contract == club.addr() || exe::staff_is_loan_listed(p, (0) as usize) != 0 {
                break 'c false;
            }
            let c2 = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club);
            let status = if c2 != 0 { (read_u8(c2 + contract::SQUAD_STATUS) >> 4) as u32 } else { 4 };
            let f = exe::shortlist_manager_staff_interest_in_joining_club(this, p, club, 1, 0, status, 0);
            if !(f >= read_f64(rdata::F64_3_15)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - exe::utils_random(10);
                if ((exe::staff_contract_days_to_end_date(c2) as u32) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0, 0) != 0 {
                break 'c true;
            }
            if c2 == 0 {
                break 'c false;
            }
            let mut buf = [0u8; 8];
            let r = exe::date_sub_days(G_CURRENT_DATE, (&mut buf) as *mut [u8; 8] as usize, (exe::utils_random(100) + 0x1e) as u32);
            let day = read_i16(r);
            let year = read_u16(r + cm_date::YEAR);
            let year_c = read_u16(c2 + contract::DATE_STARTED_YEAR);
            year > year_c || (year == year_c && day > read_i16(c2 + contract::DATE_STARTED))
        };
        if candidata {
            covered!("cpy_3 surplus to the club player");
            write_i32(v + shortlist_slot::PRICE, read_i32(v + shortlist_slot::VALUATION));
            write_i32(v + 7, rest);
            return;
        }
        k = k.wrapping_add(1);
        if k >= 0x28 {
            return;
        }
    }
}

// simplification: 0x83f3a0 (called for each slot still without a target) rolls, appends pairs to the vectors with reallocation and gives
// news: cannot run twice. Original only; the copy takes the branches and the mutations.

/// Tells whether the squad status is allowed for the person: statuses 5 and 6 (youth) only below age 24 (`.age()`,
/// signed); the others, always (1).
///
/// Convention: stdcall, ret 8, `shortlist_manager.cpp`; called with ecx = the Shortlist_Manager, which it does not
/// read. `shortlist_manager_squad_status_allowed(person, status)`, called by the contract screen (`0x4dd0f0`) and the
/// transfer screen (`0x8f6770`). No player sheet (`.player()`): the original routine runs ("Error" box, 0).
/// Original: `shortlist_manager_squad_status_allowed` (0x84b4d0, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_squad_status_allowed(person: usize, status: u32) -> u32 {
    let person = Staff(person);
    if person.player().is_null() {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED)(person.addr(), status); // message box: original only
    }
    match status as u8 {
        5 | 6 => (((person.age() as u8) as i8) < 0x18) as u32,
        _ => 1,
    }
}

/// The club's 9-byte Shortlist_Manager entry (+0xdc; +0 its 40 slots of 0x25 bytes, +8 state).
pub(crate) unsafe fn shortlist_club_entry(this: usize, club: usize) -> usize {
    read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(club).wrapping_mul(9))
}

/// First difference between two memory snapshots: (address, original, Rust).
pub(crate) fn first_memory_diff(a: &crate::comparison::MemorySnapshot, b: &crate::comparison::MemorySnapshot) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// The position masks (+1, 16 bits) of player slots 1..24, by formation (+7 of the entry); slot 0 is always the
/// goalkeeper (1). Row 0 applies to a formation outside 1..9 (the switch's `default`).
pub(crate) static FORMATION_POSITION_MASKS: [[u16; 24]; 10] = [
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x8, 0x10, 0x10, 0x20, 0x40, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // outside 1..9
    [0x808, 0x88, 0x404, 0x2, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x2, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 1
    [0x808, 0x88, 0x404, 0x204, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 2
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 3
    [0x804, 0x84, 0x404, 0x104, 0x210, 0x90, 0x810, 0x140, 0x440, 0x240, 0x4, 0x4, 0x10, 0x10, 0x140, 0x440, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x40, 0x1], // formation 4
    [0x804, 0x84, 0x404, 0x104, 0x820, 0xa0, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x20, 0x440, 0x140, 0x204, 0x10, 0x1, 0x40, 0x4, 0x20, 0x10, 0x1], // formation 5
    [0x810, 0x90, 0x404, 0x204, 0x104, 0x410, 0x208, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x8, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 6
    [0x804, 0x84, 0x404, 0x104, 0x208, 0x220, 0x410, 0x110, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 7
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x240, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 8
    [0x204, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 9
];

/// Sets the role of the 40 slots of the club's entry (25 player slots, 15 staff slots) and writes the position mask of
/// the 25 player slots by formation (`+7`).
///
/// `shortlist_set_up_formation_slots(entry)`: in the 40 slots of `0x25` bytes of the entry (`+0`), the first 25 become
/// player slots (`+0` = `0xb`) and the next 15 receive the role (6, 7, 8 ×5, 10, 9 ×7) with `shortlist_slot::FLAGS` =
/// (`shortlist_slot::FLAGS` & `0xe3`) | 2; then it writes the position mask (`shortlist_slot::POSITION`, 16 bits) of
/// the 25 player slots by the formation (`+7`, signed byte; outside 1..9 the default row). The table came out of
/// emulating the switch (each case writes the 25 slots only once).
/// Original: `shortlist_set_up_formation_slots` (0x83c090, shortlist_manager.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn shortlist_set_up_formation_slots(entry: usize) {
    for k in 0..40usize {
        let s = ShortlistSlot(read_usize(entry) + k * 0x25);
        if k < 25 {
            write_u8(s.addr(), 0xb);
        } else {
            let b = (s.addr() + shortlist_slot::FLAGS) as *mut u8;
            *b = (*b & 0xe3) | 2;
            write_u8(s.addr(), match k - 25 {
                0 => 6,
                1 => 7,
                2..=6 => 8,
                7 => 10,
                _ => 9,
            });
        }
    }
    let f = read_i8(entry + 7) as i32;
    let row = if (1..=9).contains(&f) { f as usize } else { 0 };
    covered!("cpp_5 formation slots");
    let v = ShortlistSlot(read_usize(entry));
    v.set_position(1u16 as i16); // o goleiro
    for k in 1..25usize {
        write_u16(v.addr() + shortlist_slot::POSITION + k * 0x25, FORMATION_POSITION_MASKS[row][k - 1]);
    }
}

/// Updates the club's shortlist: releases the stale targets of the club's slots
/// (`exe::shortlist_manager_release_club_slot_targets` with the entry), reassesses the squad
/// (`exe::shortlist_manager_assess_club_squad` with the recomputed entry, the club, daily 1, rebuild 0) and shares out
/// the transfer budget again (`exe::shortlist_manager_allocate_transfer_budget`).
///
/// `Shortlist_Manager::update_club_shortlist(club)`.
/// Original: `Shortlist_Manager::update_club_shortlist` (0x83a730, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_club_shortlist(this: usize, club: usize) {
    exe::shortlist_manager_release_club_slot_targets(this, club, shortlist_club_entry(this, club));
    exe::shortlist_manager_assess_club_squad(this, shortlist_club_entry(this, club), club, 1, 0);
    covered!("cpp_5 redo list");
    exe::shortlist_manager_allocate_transfer_budget(this, club); // the transfer budget
}

/// Sets to 100 the day counter of the entry of every club that targets the player and, with `clear`, removes the bid
/// marker from the slots holding him.
///
/// Convention: thiscall, ret 8, `shortlist_manager.cpp`, SEH frame only for the new list.
/// `Shortlist_Manager::suspend_clubs_shortlisting_player(player, clear)`: in a new LLIST (`exe::crt_operator_new(0xe)`
/// and `exe::new_llist`) it gathers the clubs targeting the player (`exe::shortlist_manager_collect_interested_clubs`
/// with ignore 1, mode 3, all); for each item (`exe::llist_get_next`, after resetting the cursor
/// `.set_cursor(.head())`) it sets the day counter of the club's entry (`club_shortlist::DAYS`) to 100 and, with
/// `clear`, removes the bid marker (`.offer_ref()` = −1 when > −1) from the slots that hold the player
/// (`.target_staff_id()` = id). It destroys and frees the list (`exe::drop_llist`, `exe::crt_operator_delete`). The
/// count (`+0`, signed 16 bits) is re-read on every pass, as in the original.
/// Original: `Shortlist_Manager::suspend_clubs_shortlisting_player` (0x842b00, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn suspend_clubs_shortlisting_player(this: usize, player: usize, clear: u8) {
    let p = exe::crt_operator_new(0xe);
    let list = Llist(if p != 0 { exe::new_llist(p) } else { 0 });
    exe::shortlist_manager_collect_interested_clubs(this, player, list, 1, 3, 0);
    let n = read_i16(list.addr());
    if n != 0 {
        list.set_cursor(list.head());
    }
    if n > 0 {
        let mut i = 0i32;
        while i < read_i16(list.addr()) as i32 {
            let item = exe::llist_get_next(list);
            if item != 0 {
                let e = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(item + shortlist_pair::CLUB_ID).wrapping_mul(9));
                covered!("cpp_5 suspend club");
                write_u8(e + club_shortlist::DAYS, 0x64); // the club suspends the player
                if clear != 0 {
                    for k in 0..0x28usize {
                        let s = ShortlistSlot(read_usize(e) + k * 0x25);
                        if s.target_staff_id() == read_usize(player) && s.offer_ref() > -1 {
                            s.set_offer_ref(-1);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if !list.is_null() {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
}

/// Comparador: a `exe::shortlist_manager_assess_club_squad` reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// CONTRACT_MANAGER get_contract (`exe::contract_manager_get_contract`) for a person.
pub(crate) unsafe fn get_contract(person: usize) -> Contract {
    Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person))
}

/// get_contract for a person, stored in G_CONTRACT_LOOKUP_RESULT as the original does.
pub(crate) unsafe fn shortlist_manager_get_contract_and_store(person: usize) -> usize {
    let c = get_contract(person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    c.addr()
}

/// with the player's filters (record, marked regen, job, career end, nation, untouchable, arrived less than 200 days
/// ago without being listed), tries to put him on the list of each club in the array (`exe::shortlist_manager_try_add_player_to_club_shortlist`) up to 5 clubs, except
/// national teams, the contract's club and his own; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without running
/// the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252):
/// the detour changes the stack residue a later original reads; since then the routine is hooked again
/// Original: `Shortlist_Manager::add_player_to_club_shortlists` (0x842c00, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_player_to_club_shortlists(this: usize, player: usize, force: u32) {
    let player = Staff(player);
    let sheets = player.player();
    if sheets.is_null() {
        return;
    }
    if force == 0 && sheets.current_reputation() < 0x157c {
        return;
    }
    let record = |j: usize| read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(staff_state::SIZE));
    if read_u8(record(player.addr()) + staff_state::STATUS_FLAGS) & 0x30 == staff_state_status_flags::RETIREMENT_ANNOUNCED {
        return;
    }
    if player.job_for_club() != job_kind::PLAYER {
        return;
    }
    if !player.national_job().is_null() && player.job_for_nation() == job_kind::MANAGER {
        return;
    }
    let nation = player.nation();
    if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_SAUDI_ARABIA) {
        return;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), player, 0) != 0 {
        return;
    }
    if !player.club_job().is_null() {
        let mut t = [0u32; 2];
        let p = exe::date_sub_days(G_CURRENT_DATE, (t.as_mut_ptr()) as *mut u32 as usize, 200);
        let lo = read_usize(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let year_j = player.date_joined_club_year();
        let day_j = player.date_joined_club() as i16;
        let recente = if ano != year_j { ano < year_j } else { dia < day_j };
        if recente {
            let c = get_contract(player.addr());
            if !c.is_null() && c.squad_status() & 0xf0 != 0x70 && (c.grievances() as usize) == 0 {
                return;
            }
        }
    }
    let c = shortlist_manager_get_contract_and_store(player.addr());
    let transfer_manager_contract_club = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { Club(0) };
    let mut count = (read_u8(record(player.addr()) + staff_state::STATUS_FLAGS) & staff_state_status_flags::AI_SHORTLIST_COUNT) as i8;
    // simplification: 0x5b7d20 returns only ax and 0x842e10 uses 16 bits of the mask: the upper 16 go zeroed
    let mut m = exe::formation_get_position_mask(player.player()) as u32;
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 {
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x80 != 0 {
        m |= 0x100;
    }
    if m & 0x800 != 0 {
        m |= 0x400;
    }
    if read_i32(G_CLUB_COUNT) <= 0 {
        return;
    }
    let clubs = read_usize(G_CLUB_REPUTATION_ORDER);
    let mut i = 0i32;
    loop {
        let club = friendly_club_by_id(read_usize(clubs + i as usize * 4));
        if count >= 5 {
            return;
        }
        if exe::database_is_national_team(club) == 0 && transfer_manager_contract_club != club && player.club_job() != club && exe::shortlist_manager_try_add_player_to_club_shortlist(this, player, club, m) != 0 {
            covered!("cpp_6 club added");
            count = count.wrapping_add(1); // one more club with him in the list
        }
        i += 1;
        if i >= read_i32(G_CLUB_COUNT) {
            return;
        }
    }
}

/// Transfer-lists the player: marks the contract as for sale; the player may come to dislike the manager (liking the
/// club or loyalty against two draws of 100); with `news`, puts him on the clubs' lists and posts the news.
///
/// `Shortlist_Manager::transfer_list_player(player, news)`: if the club has not listed him yet
/// (`exe::staff_is_transfer_listed` with 0) and there is a contract: marks the contract for sale (`.transfer_status()`
/// bit 1, `.squad_status()` high nibble 7); if he is not untouchable (`exe::transfer_manager_get_future_transfer`):
/// with the player liking the club (`exe::staff_favours_club`) or with the loyalty (`.morale()` of the contract,
/// signed) above the difference of two random(100) draws, the player starts to dislike the club's manager
/// (`exe::staff_add_disliked_staff` with the club's `.manager()`, 1); with `news`, puts the player on the clubs' lists
/// (`exe::shortlist_manager_add_player_to_club_shortlists` with 1) and posts the news
/// (`exe::contract_manager_post_transfer_listed_news` with the club `.club_job()`).
/// Original: `Shortlist_Manager::transfer_list_player` (0x842320, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_list_player(this: usize, player: usize, news: u32) {
    let player = Staff(player);
    if exe::staff_is_transfer_listed(player, 0) != 0 {
        return;
    }
    let c = Contract((get_contract(player.addr())).addr());
    if c.is_null() {
        return;
    }
    let a = c.squad_status();
    let b = c.transfer_status();
    c.set_transfer_status(b | 1);
    covered!("cpp_6 put up for sale");
    c.set_squad_status((a & 0xf) | 0x70); // for sale
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), player, 0) != 0 {
        return;
    }
    let desgostar = if exe::staff_favours_club(player, player.club_job()) != 0 {
        true
    } else {
        let loyalty = (c.morale() as u8) as i8 as i32;
        let s1 = exe::utils_random(100);
        let s2 = exe::utils_random(100);
        loyalty > s1.wrapping_sub(s2)
    };
    if desgostar {
        exe::staff_add_disliked_staff(player, player.club_job().manager().addr(), 1);
    }
    if news != 0 {
        exe::shortlist_manager_add_player_to_club_shortlists(this, player, 1);
        exe::contract_manager_post_transfer_listed_news(read_usize(G_CONTRACT_MANAGER), player, player.club_job());
    }
}

/// The person's appearance-bonus record (0xd bytes: +0 person id, +4 value, +8 matches, +9 old club id), found by
/// binary search (`exe::crt_bsearch` with 0x8b4e00) in the TRANSFER_MANAGER's national team array (type 5: +0x898, count +0x8a0)
/// or club array (+0x88c, count +0x894).
pub(crate) unsafe fn appearance_bonus_record(this: TransferManagerFields, player: usize, kind: u32) -> usize {
    let key = [read_usize(player) as u32];
    let (vetor, conta) = if kind as u8 == 5 { (this.international_appearance_money(), this.international_appearance_money_count()) } else { (this.appearance_money(), this.appearance_money_count()) };
    exe::crt_bsearch((key.as_ptr()) as usize, vetor, (conta) as u32, 0xd, 0x8b4e00)
}

/// Comparer: `exe::shortlist_manager_try_add_player_to_club_shortlist` changes the slots of many clubs (and the lists do not fit in a cheap copy), so only the original
/// routine runs.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// The person's main contract (stored), its club, and the contract with that club (`exe::contract_manager_get_contract_for_club`).
pub(crate) unsafe fn contract_with_own_club(p: Staff) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let club = if !c.is_null() { friendly_club_by_id(c.club()) } else { Club(0) };
    exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club)
}

/// Rebuilds the 32 position lists (16 slots × 2) at 12 bytes per entry: the size is the largest number of people
/// fitting a slot; each non-human person enters every slot he fits with his id, the nation id (drawn when missing) and
/// the value or the contract's `contract::HAPPINESS`. Runs every 30 days.
///
/// `Shortlist_Manager::build_position_lists(_not_used)` (unused): frees the 32 arrays of the position lists
/// (`shortlist_manager_fields::POSITION_LISTS_BY_PERSON`: 16 by person,
/// `shortlist_manager_fields::POSITION_LISTS_BY_KEY`: 16 by contract key); their size
/// (`shortlist_manager_fields::POSITION_LIST_CAPACITY`) is the largest number of people (all, humans included) that fit
/// one of the 16 slots (`exe::shortlist_staff_fits_slot_position`); it allocates the 32 with 12 bytes per entry (out of
/// memory: the "Error" box and the game stops) and zeroes the counts
/// (`shortlist_manager_fields::POSITION_LIST_COUNTS`).
///
/// Then each person outside the 16 humans, in each slot he fits (count at the limit: the box and the game stops): the
/// two entries with his id (`+0`), the nation id (`+4`; with no nation at `.nation()`, `exe::utils_random` draws one
/// and writes it into the person), in the first one the value (`.value()`) and in the second the `contract::HAPPINESS`
/// of the contract with the contract's club (with no contract: 100).
/// Original: `Shortlist_Manager::build_position_lists` (0x839e60, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn build_position_lists(this: usize, _not_used: u32) {
    // simplification: the original pushes the slot in a dword with 3 bytes of stack garbage above; 0x844300 only reads the byte
    covered!("cpo_8 build lists");
    for k in 0..32usize {
        let v = this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + k * 4;
        if read_usize(v) != 0 {
            exe::crt_free(read_usize(v));
            write_u32(v, 0);
        }
    }
    write_u32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY, 0);
    for vaga in 0..16u32 {
        let base = read_usize(G_STAFF_TABLE);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < read_i32(G_STAFF_COUNT) {
            if exe::shortlist_staff_fits_slot_position(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > read_i32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY) {
            write_u32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY, n as u32);
        }
    }
    for k in 0..32usize {
        let v = exe::crt_malloc(read_usize(this + shortlist_manager_fields::POSITION_LIST_CAPACITY).wrapping_mul(12) as u32);
        write_u32(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + k * 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        write_u32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + k * 4, 0);
    }
    let base = read_usize(G_STAFF_TABLE);
    let mut i = 0usize;
    while (i as i32) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        let p = Staff(base.wrapping_add(i * staff::SIZE));
        for vaga in 0..16u32 {
            if exe::shortlist_staff_fits_slot_position(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let count = read_i32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + pv * 4);
            if count >= read_i32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY) {
                fatal_error_at(0xa825a4, 0x121);
            }
            let o = (count as usize).wrapping_mul(12);
            let (l1, l2) = (read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + pv * 4).wrapping_add(o), read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + pv * 4).wrapping_add(o));
            write_u32(l1, i as u32);
            write_u32(l2, i as u32);
            let mut nation = p.nation();
            if nation.is_null() {
                nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((exe::utils_random(read_i32(G_NATION_COUNT)) as usize).wrapping_mul(nation::SIZE)));
                p.set_nation(Nation((nation.addr() as u32) as usize));
            }
            write_u32(l1 + 4, read_usize(nation.addr()) as u32);
            write_u32(l2 + 4, read_usize(p.nation().addr()) as u32);
            write_u32(l1 + 8, p.value() as u32);
            let k = contract_with_own_club(p);
            write_u32(l2 + 8, if k != 0 { read_u8(k + contract::HAPPINESS) as i8 as i32 as u32 } else { 100 });
            write_u32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + pv * 4, (count + 1) as u32);
        }
        i += 1;
    }
}

/// Every day, recomputes the entry keys (`+8`) of the 16 slots and sorts both arrays of each slot (`exe::crt_qsort`
/// with `0x83b670`: by the key and by `+4`).
///
/// `Shortlist_Manager::sort_position_lists()`. In the first list: the person's value (`.value()`) if the contract with
/// the club exists and the expiry band (`exe::staff_contract_get_expiry_band`) is 0, otherwise 0; a player of a human
/// club (`exe::human_manager_is_human_club`) on the transfer list (`exe::staff_is_transfer_listed`) loses 1,500,000, at
/// least down to a third of the value. In slot 15 only the first list, with the value.
///
/// In the second: −1000 with bit 2 of the contract's `.transfer_status()`; 2000 with bits 0 or 3, or with no player
/// sheet (`.player()`), no club, no division (`.division()`) or a division with no flag in `[G_COMP_OBJECT_TABLE]` and
/// reputation below 6000, or no contract; otherwise the contract's `.happiness()`.
/// Original: `Shortlist_Manager::sort_position_lists` (0x83a780, shortlist_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sort_position_lists(this: usize) {
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < read_i32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) {
            let p = Staff((staff_records_staff_by_id(read_usize(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o))).addr());
            let k = contract_with_own_club(p);
            if vaga == 15 {
                write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + 0x3c) + o + 8, p.value() as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && exe::staff_contract_get_expiry_band(k) == 0 {
                write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8, p.value() as u32);
                let club = p.club_job();
                if !club.is_null() && exe::human_manager_is_human_club(club, 0, 1, (0) as u32, 0) != 0 && exe::staff_is_transfer_listed(p, 0) != 0 {
                    let e = read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8;
                    write_u32(e, read_i32(e).wrapping_add(-0x16e360) as u32);
                    let third = (p.value() as i32) / 3;
                    if read_i32(e) < third {
                        write_u32(e, third as u32);
                    }
                }
            } else {
                if k != 0 {
                    covered!("cpo_8 contract band");
                }
                write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = Staff((staff_records_staff_by_id(read_usize(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4) + o))).addr());
            let k2 = Contract(contract_with_own_club(p2));
            let club = p2.club_job();
            let key: i32 = if p2.player().is_null() || club.is_null() || club.division().addr() == 0 {
                0x7d0
            } else if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(club.division().addr()).wrapping_mul(4)) == 0
                && club.reputation() < 0x1770
            {
                0x7d0
            } else if k2.is_null() {
                0x7d0
            } else if k2.transfer_status() & 2 != 0 {
                -1000
            } else if k2.transfer_status() & 9 != 0 {
                0x7d0
            } else {
                (k2.happiness() as u8) as i8 as i32
            };
            write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4) + o + 8, key as u32);
            j += 1;
            o += 12;
        }
        let n = read_usize(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) as u32;
        exe::crt_qsort(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4), n, 12, 0x83b670);
        let n = read_usize(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) as u32;
        exe::crt_qsort(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4), n, 12, 0x83b670);
    }
}

/// Empties the output list and adds the interested (player, club) pairs: the humans who have the player on the scout
/// shortlist and the clubs in the by-person array that are not suspended, are not his and value him above his value.
///
/// `Shortlist_Manager::collect_interested_clubs(player, out, ignore, mode, so_humanos)` (player, output, ignore, mode,
/// humans only): empties list `out` (`exe::llist_clear`) and adds (`exe::llist_add_tail`, 8 bytes: player id, club id)
/// the interested clubs: with mode ≠ 3, the 16 humans (from last to first) with `.classification()` and a club who have
/// the player on the scout's list (`exe::scout_manager_is_player_shortlisted`; with mode 2, except the clubs the
/// current human controls, `exe::human_manager_current_controls_club`); without `so_humanos`, the (player, club) pairs
/// of the by-person array (`+0`, count `shortlist_manager_fields::PAIRS_BY_PERSON_COUNT`; `exe::crt_bsearch` up to the
/// player's first) whose club is not suspended (`club_shortlist::DAYS` of the club's 9-byte entry at
/// `shortlist_manager_fields::CLUB_SHORTLISTS` below 100, unless `ignore`), is not the player's and puts on him (`+7`
/// of the `0x25`-byte slot with the player at `shortlist_slot::TARGET_STAFF_ID`; with no slot, the first one) more than
/// his value (`.value()`).
/// Original: `Shortlist_Manager::collect_interested_clubs` (0x842180, shortlist_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn collect_interested_clubs(this: usize, player: usize, out: usize, ignore: u32, mode: u32, so_humanos: u32) {
    let player = Staff(player);
    if out != 0 {
        exe::llist_clear(out);
    }
    if mode as u8 != 3 {
        for k in 0..16i32 {
            let h = Staff((staff_records_staff_by_id(read_i32(G_STAFF_COUNT).wrapping_sub(k).wrapping_sub(1) as usize)).addr());
            let club = h.club_job();
            if h.classification() == 0 || club.is_null() {
                continue;
            }
            if exe::human_manager_current_controls_club(club, 0, 1) != 0 && mode as u8 == 2 {
                continue;
            }
            if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), player, h) != 0 {
                let par = [read_usize(player.addr()) as u32, read_usize(club.addr()) as u32];
                exe::llist_add_tail(out, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let key = [read_usize(player.addr()) as u32, 0xffff_ffff];
    let mut e = exe::crt_bsearch((key.as_ptr()) as usize, read_usize(this), read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) as u32, 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = read_usize(this);
    let last = base.wrapping_add((read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = read_usize(player.addr());
    if e > base {
        loop {
            let x = read_usize(e - 8);
            e -= 8;
            if x != id {
                e += 8;
                break;
            }
            if e <= base {
                break;
            }
        }
    }
    if read_usize(e) != id {
        return;
    }
    loop {
        let club_id = read_usize(e + 4);
        let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(club_id.wrapping_mul(9));
        let slots = ShortlistSlot(read_usize(entry));
        let mut k = 0usize;
        while k < 0x28 && read_usize(slots.addr() + shortlist_slot::TARGET_STAFF_ID + k * 0x25) != read_usize(player.addr()) {
            k += 1;
        }
        let value = if k < 0x28 { read_i32(slots.addr() + k * 0x25 + 7) } else { read_i32(slots.addr() + 7) };
        if !(ignore == 0 && read_u8(entry + club_shortlist::DAYS) >= 100) && value > (player.value() as i32) {
            let c = player.club_job();
            if !(!c.is_null() && read_usize(c.addr()) == club_id) {
                covered!("cpo_8 interested club");
                exe::llist_add_tail(out, e, 8);
            }
        }
        if e == last {
            return;
        }
        let next = read_usize(e + 8);
        e += 8;
        if next != read_usize(player.addr()) {
            return;
        }
    }
}

/// The 8-byte items (two u32) of an LLIST.
pub(crate) unsafe fn llist_pairs(list: Llist) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut in_the = list.head();
    while in_the != 0 {
        let x = read_usize(in_the);
        v.push((read_usize(x) as u32, read_usize(x + 4) as u32));
        in_the = read_usize(in_the + llist_node::NEXT);
    }
    v
}

/// Comparador: só a original.
// simplification: the original frees and allocates the 32 arrays (and may roll the nation of whoever has none); does not run twice. Checks
// the result: the two lists of each slot with the same ids, each entry fit for the slot, the count at the limit.

/// Appends a (person id, club id) pair to the array [this + vet], bumps the count and qsorts it.
pub(crate) unsafe fn insert_sorted_pair(this: usize, vet: usize, count: usize, person: usize, club: usize, cmp: usize) {
    let n = read_i32(this + count);
    let e = read_usize(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    write_u32(e, read_usize(person) as u32);
    write_u32(e + shortlist_pair::CLUB_ID, read_usize(club) as u32);
    write_u32(this + count, (n + 1) as u32);
    exe::crt_qsort(read_usize(this + vet), (n + 1) as u32, 8, cmp);
}

/// Adds a (person, club) shortlist entry: creates both pair arrays (capacity 100) the first time and, when the pair is
/// new, grows by 100, adds and sorts (by person, by club) and, outside a human club, adds 1 to the low nibble of `+0xb`
/// of the person's 79-byte record.
///
/// `Shortlist_Manager::add_shortlist_entry(person, club)`: creates the two pair arrays the first time
/// (`exe::crt_malloc` of `0x320` bytes, capacity
/// `shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY`/`shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY` = 100; out
/// of memory: the "Error" box and the game stops); with the pair (person, club; with no club −1) already in the
/// by-person array (`exe::crt_bsearch` with `0x757ea0`), nothing. Otherwise it grows the full array by 100
/// (`exe::crt_realloc`), adds the pair to both and sorts them (by person with `0x757ea0`, by club with `0x844740`) and,
/// outside a human club (`exe::human_manager_is_human_club`), adds 1 to the low nibble of `staff_state::STATUS_FLAGS`
/// of the person's 79-byte record (`[G_STAFF_STATE_TABLE]`: how many lists have him).
/// Original: `Shortlist_Manager::add_shortlist_entry` (0x8423f0, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_shortlist_entry(this: usize, person: usize, club: usize) {
    if read_usize(this) == 0 {
        let v = exe::crt_malloc(0x320);
        write_u32(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x1594);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY, 100);
    }
    if read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB) == 0 {
        let v = exe::crt_malloc(0x320);
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x159d);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY, 100);
    }
    if read_usize(this) != 0 {
        let key = [read_usize(person) as u32, if club != 0 { read_usize(club) as u32 } else { 0xffff_ffff }];
        if exe::crt_bsearch((key.as_ptr()) as usize, read_usize(this), read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) as u32, 8, 0x757ea0) != 0 {
            return;
        }
    }
    covered!("cpo_9 new entry");
    if read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY) == read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) {
        let v = exe::crt_realloc(read_usize(this), (read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15b5);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY, read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_CAPACITY).wrapping_add(100) as u32);
    }
    if read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY) == read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_COUNT) {
        let v = exe::crt_realloc(read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB), (read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15be);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY, read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY).wrapping_add(100) as u32);
    }
    insert_sorted_pair(this, 0, 0x10, person, club, 0x757ea0); // the array by person
    insert_sorted_pair(this, 4, 0x14, person, club, 0x844740); // the vector by club
    if exe::human_manager_is_human_club(club, 0, 1, (0) as u32, 0) == 0 {
        let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) as *mut u8;
        let a = *r;
        *r = (a & 0xf0) | (a.wrapping_add(1) & 0xf);
    }
}

/// Predicts the selling club's response: 0 accepts, 1 haggles, 2 rejects, from the contract with the seller, the loan
/// case (listed, reputation, status) and the price (human, clause, holiday, asking or
/// `exe::contract_manager_calc_asking_price`) against the offer.
///
/// `shortlist_manager_predict_selling_club_response(seller, player, offer, buyer, request, loan)` (seller, player,
/// offer, buyer, asking price, loan). No seller or with the contract with him
/// (`exe::contract_manager_get_contract_for_club`) flagged at `.leaving_on_bosman()`: 0; the seller's manager is the
/// player: 2; no contract of type 2 (`.contract_type()`): 2 for a loan, 0 otherwise.
///
/// Loan: human club (`exe::human_manager_is_human_club`): 0; loan-listed (`FN_STAFF_IS_LOAN_LISTED`) with the buyer's
/// reputation × `[rdata::F64_1_25]` above the sheet's `.current_reputation()`: 0; status 6/7 (`.squad_status()`) or the
/// sheet's `.goalkeeper()` 20, with the reputation above `.current_reputation()` × `[rdata::F64_0_75]`, the contract's
/// `.happiness()` below −50 and the seller's `.continental_comp_id()` different from
/// `[KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE]`: 0; otherwise 2.
///
/// Purchase: the price (human: `exe::contract_manager_get_human_asking_price` × `[rdata::F64_0_8]`, with no clause
/// `exe::staff_contract_has_active_release_clause` and with the manager on holiday `exe::human_manager_is_on_holiday`
/// option 0 `exe::human_manager_get_holiday_option`: 1 rejects, 2 the value `.value()`; otherwise the `request` if not
/// negative, the value with no sheet, or `exe::contract_manager_calc_asking_price`); offer at the price: 0; human with
/// the offer above price − 1,500,000 and listed (`FN_STAFF_IS_TRANSFER_LISTED`): 1; price × `[rdata::F64_0_7142857143]`
/// above the offer: 2; otherwise 1.
/// Original: `shortlist_manager_predict_selling_club_response` (0x84af00, shortlist_manager.cpp, stdcall, ret 0x20).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_predict_selling_club_response(seller: usize, player: usize, offer: i64, buyer: usize, request: i64, loan: u32) -> u8 {
    let player = Staff(player);
    let seller = Club(seller);
    let buyer = Club(buyer);
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, seller));
    if seller.is_null() || (!k.is_null() && k.leaving_on_bosman() != 0) {
        return 0;
    }
    if seller.manager() == player {
        return 2;
    }
    if k.is_null() || k.contract_type() & 0x3f != 2 {
        return if loan as u8 != 0 { 2 } else { 0 };
    }
    if loan as u8 != 0 {
        if exe::human_manager_is_human_club(seller, 0, 1, (0) as u32, 0) != 0 {
            return 0;
        }
        let listed: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
        let rep = buyer.reputation() as f64;
        if listed(player.addr(), 0) != 0 && rep * read_f64(rdata::F64_1_25) > player.player().current_reputation() as f64 {
            return 0;
        }
        let st = k.squad_status() & 0xf0;
        if st == 0x60 || st == 0x70 || (player.player().goalkeeper() as u8) == 0x14 {
            let v = player.player().current_reputation() as f64 * read_f64(rdata::F64_0_75);
            if rep > v && ((k.happiness() as u8) as i8) < -50 && (seller.continental_comp_id() as usize) != read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) {
                return 0;
            }
        }
        return 2;
    }
    let price: i32 = if exe::human_manager_is_human_club(seller, 0, 1, (0) as u32, 0) != 0 {
        let mut v = ftol(exe::contract_manager_get_human_asking_price(read_usize(G_CONTRACT_MANAGER), seller, buyer, player) as f64 * read_f64(rdata::F64_0_8));
        if exe::staff_contract_has_active_release_clause(k) == 0 {
            if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), exe::database_get_club_manager(seller)) != 0 {
                match exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 0, exe::database_get_club_manager(seller)) as i8 {
                    1 => return 2,
                    2 => v = player.value() as i32 ,
                    _ => {}
                }
            }
        }
        v
    } else if request >= 0 {
        request as i32
    } else if player.player().is_null() {
        player.value() as i32 
    } else {
        exe::contract_manager_calc_asking_price(read_usize(G_CONTRACT_MANAGER), seller, buyer, player, 0, 1, 1)
    };
    if offer >= price as i64 {
        return 0;
    }
    let listed: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    if exe::human_manager_is_human_club(seller, 0, 1, (0) as u32, 0) != 0 && (price.wrapping_sub(0x16e360) as i64) < offer && listed(player.addr(), 0) != 0 {
        return 1;
    }
    if price as f64 * read_f64(rdata::F64_0_7142857143) > offer as f64 {
        return 2;
    }
    covered!("cpo_9 negotiates");
    1
}

/// Year of the default date (`exe::date_date_54a110`: day 40 of 1972).
pub(crate) unsafe fn default_date_year() -> u16 {
    let mut data = [0u8; 8];
    let p = exe::date_date_54a110(data.as_mut_ptr() as usize);
    read_i16(p + cm_date::YEAR) as u16
}

/// True if the date (day, year) is after contract c's end (+0x2d day, +0x2f year).
pub(crate) unsafe fn is_after_contract_end(day: i16, year: u16, c: Contract) -> bool {
    let end = c.contract_expires_year() as u16;
    year > end || (year == end && day > c.contract_expires())
}

/// Comparer: only the original routine runs, checking the plan (made beforehand): the pair was already there (nothing
/// changes) or it enters both vectors (counts + 1, the pair found afterwards) and the person's nibble goes up by 1
/// outside a human club.
// simplification: the original reallocates the vectors (0x945667) and sorts them; does not run twice.

/// Club record of contract c (id at +4, G_CLUB_TABLE), or 0.
pub(crate) unsafe fn shortlist_manager_club_of_contract(c: Contract) -> usize {
    if c.is_null() {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE))
}

/// Returns the squad status the club gives the person: from her entry in the club's shortlist (bits 1..4 of
/// `shortlist_slot::FLAGS`, job from `+0`), from the contract with the club (high nibble of `.squad_status()`), from
/// the human club estimate, or 7.
///
/// `Shortlist_Manager::get_squad_status_for_club(person, club, job)`. Null club: 7. The person's entry in the club's
/// list (`exe::shortlist_manager_find_club_shortlist_entry`: the one of type ≠ 11 for a player, `.player()`; having
/// `.non_player()`, the one of type 11 too): the job (`+0` of the entry) and bits 1..4 of `shortlist_slot::FLAGS`.
/// Otherwise the job is the person's `.job_for_club()` and the contract decides (`exe::contract_manager_get_contract`,
/// stored in `[G_CONTRACT_LOOKUP_RESULT]`): at her own club, the high nibble of `.squad_status()` of the contract with
/// the club (`exe::contract_manager_get_contract_for_club`; without it, 7); a human club
/// (`exe::human_manager_is_human_club`) with job 11 to 15 (`exe::database_is_player_job`): the estimate
/// `exe::shortlist_manager_estimate_squad_status`; otherwise, the person's club (`.club_job()`) being the club or its
/// partner (`FN_CLUB_GET_LINKED_B_TEAM`) leads to the contract with the club; none: 7.
/// Original: `Shortlist_Manager::get_squad_status_for_club` (0x849f00, shortlist_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_squad_status_for_club(this: usize, person: usize, club: usize, job: *mut u8) -> u8 {
    let person = Staff(person);
    if club == 0 {
        return 7;
    }
    let mut e = exe::shortlist_manager_find_club_shortlist_entry(this, club, person, (person.player().is_null()) as u32, 0);
    if e == 0 && !person.non_player().is_null() {
        e = exe::shortlist_manager_find_club_shortlist_entry(this, club, person, 1, 0);
    }
    if e != 0 {
        if !job.is_null() {
            *job = read_u8(e);
        }
        covered!("cpn_6 status by list");
        return (read_u8(e + shortlist_slot::FLAGS) >> 1) & 0xf; // bits 1..4 of the entry
    }
    if !job.is_null() {
        *job = person.job_for_club();
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let by_contract = if shortlist_manager_club_of_contract(Contract(c)) == club {
        true
    } else {
        // simplification: the 3rd argument is pushed as dword 1; the job goes in a byte (0x53b620 only reads the byte)
        if exe::human_manager_is_human_club(club, 1, 1, (0) as u32, 0) != 0 && exe::database_is_player_job(person.job_for_club() as i8) != 0 {
            return exe::shortlist_manager_estimate_squad_status(person, club) as u8;
        }
        person.club_job() == Club(club) || {
            let partner: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            person.club_job() == Club(partner(club, 0, 1))
        }
    };
    if !by_contract {
        return 7;
    }
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
    if k.is_null() {
        return 7;
    }
    covered!("cpn_6 status by contract");
    k.squad_status() >> 4 // the high nibble of the contract
}

/// Estimates the squad status the human club would give: from the ratio between the person's reputation as seen by the
/// manager (×1.5 when on the human's shortlist) and the club's, and from the age. Only runs for the human club.
///
/// `shortlist_manager_estimate_squad_status(person, club)`. The person's reputation as seen by the club's manager
/// (`exe::database_get_club_manager`, `exe::database_get_staff_reputation_for_viewer`; up to 1: 1, otherwise the two
/// calls again) and the club's (`.reputation()`, at least 1); a person on the human's list
/// (`exe::scout_manager_is_player_shortlisted` of `[G_SCOUT_MANAGER]`): the reputation becomes the larger of 1.5 × it
/// and 0.75 × the club's (truncated, 16 bits). The ratio (clamped to 0.1..2): above 1.5: 1; its float above 1.2 with
/// the club below 8500: 2; up to 1.2 with the reputation up to 6500: 5 (above 0.6 and age `.age()` below 23), 4 (24 or
/// more) or 6; otherwise 3.
/// Original: `shortlist_manager_estimate_squad_status` (0x849d60, shortlist_manager.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_estimate_squad_status(person: usize, club: usize) -> u32 {
    let person = Staff(person);
    let club = Club(club);
    let mut rep = if (exe::database_get_staff_reputation_for_viewer(person, exe::database_get_club_manager(club), 0) as u16) as i16 > 1 { (exe::database_get_staff_reputation_for_viewer(person, exe::database_get_club_manager(club), 0) as u16) as i16 } else { 1 };
    let mut rc = club.reputation();
    if rc <= 1 {
        rc = 1;
    }
    let t = exe::database_get_club_manager(club);
    if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), person, t) != 0 {
        let a = rep as f64 * read_f64(rdata::F64_1_5);
        let b = rc as f64 * read_f64(rdata::F64_0_75);
        rep = ftol(if a > b { a } else { b }) as i16;
    }
    let mut q = rep as f64 / rc as f64;
    if q < read_f64(rdata::F64_0_1) {
        q = read_f64(rdata::F64_0_1);
    } else if !(q <= read_f64(rdata::F64_2)) {
        q = read_f64(rdata::F64_2);
    }
    let f = q as f32 as f64; // the original keeps the ratio in a float and compares the rest by it
    if q > read_f64(rdata::F64_1_5) {
        return 1;
    }
    if f > read_f64(rdata::F64_1_2) && rc < 0x2134 {
        return 2;
    }
    let age = (person.age() as u8) as i8;
    if !(f > read_f64(rdata::F64_1_2)) && rep <= 0x1964 {
        if f > read_f64(rdata::F64_0_6) && age < 0x17 {
            return 5;
        }
        return if age >= 0x18 { 4 } else { 6 };
    }
    3
}

/// In the club's 40 slots, a target with an invalid id becomes −1 and a target not at the club (by contract or
/// `.club_job()`) and without `.offer_ref()` gets shortlist event 3.
///
/// `shortlist_manager_release_club_slot_targets(club, entry)`: in the 40 slots of `0x25` bytes of `[entry]`, a target
/// (`shortlist_slot::TARGET_STAFF_ID`) above the last person id (`[G_STAFF_COUNT]` − 1) becomes −1; a valid target
/// whose contract (`exe::contract_manager_get_contract`, stored in `[G_CONTRACT_LOOKUP_RESULT]`; one of type 5 at
/// `.contract_type()` does not count) is not with the club, whose own club (`.club_job()`) is not the club either, and
/// with the slot's `.offer_ref()` at −1: list event 3 (`exe::shortlist_manager_update_staff_status`) with the type "not
/// 11" from the slot's `+0` (in the low byte of the slot's address, as the original does).
/// Original: `shortlist_manager_release_club_slot_targets` (0x844660, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_release_club_slot_targets(this: usize, club: usize, entry: usize) {
    for k in 0..40usize {
        let o = k * 0x25;
        let v = read_usize(entry).wrapping_add(o + shortlist_slot::TARGET_STAFF_ID);
        if read_usize(v) as i32 > read_i32(G_STAFF_COUNT).wrapping_sub(1) {
            write_i32(v, -1);
        }
        let id = read_usize(read_usize(entry).wrapping_add(o.addr() + shortlist_slot::TARGET_STAFF_ID)) as i32;
        if id <= -1 {
            continue;
        }
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
        let of_contract = if !c.is_null() && c.contract_type() & 0x3f != 5 { shortlist_manager_club_of_contract(c) } else { 0 };
        if of_contract == club || p.club_job() == Club(club) {
            continue;
        }
        let s = ShortlistSlot(read_usize(entry).wrapping_add(o.addr()));
        if (s.offer_ref() as usize) as i32 != -1 {
            continue;
        }
        // setne al on the slot address: the high 24 bits are its own
        let kind = ((s.addr() & !0xff) | (read_u8(s.addr()) != 0xb) as usize) as u32;
        covered!("cpn_6 target outside the club");
        ((exe::shortlist_manager_update_staff_status(this, p, club, 3, kind) as usize) as u32);
    }
}

/// Comparador: só a original.
// simplification: 0x843e10 changes the club’s list (and others) on each event and cannot run twice; the copy has the branch
// and the proof is by the career (Rust mode against the original).

/// Tells whether the player may move to the buying club: without rule `0xe` in the buyer's nation, same nation, second
/// nation or EU: yes; no national team: no; otherwise the work permit.
///
/// `passes_work_permit_check(player, _current, buyer)` (player, _, buyer). No nation (`.nation()`): no. The buyer's
/// nation (`.nation()`) without rule `0xe` (`.actual_region()`, `world_region::UK_AND_IRELAND`), the same nation, the
/// 2nd nation (`.second_nation()`) or an EU player (`exe::database_is_staff_eu_national`): yes. No national team
/// (`.national_job()`) and no caps for it (`.int_apps()`): no. Otherwise the work permit
/// (`exe::transfer_manager_is_work_permit_granted` on `[G_TRANSFER_MANAGER]`, with no offer, record or random draw).
/// The 2nd argument is not used.
/// Original: `passes_work_permit_check` (0x84b180, shortlist_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn passes_work_permit_check(player: usize, _current: usize, buyer: usize) -> u8 {
    let player = Staff(player);
    let buyer = Club(buyer);
    let nation = player.nation();
    if nation.is_null() {
        return 0;
    }
    let target = buyer.nation();
    if target.actual_region() != world_region::UK_AND_IRELAND || nation == target || player.second_nation() == target {
        return 1;
    }
    if exe::database_is_staff_eu_national(player, target) != 0 {
        return 1; // community
    }
    if player.national_job().is_null() && player.int_apps() == 0 {
        return 0;
    }
    exe::transfer_manager_is_work_permit_granted(read_usize(G_TRANSFER_MANAGER), player, target, 0, 0, 0) as u8
}

/// Tries a loan offer for the player: with an interest of at least 3.5, a loan period and a return date within the
/// contract, the window and 3 weeks from today, makes the offer with 0, 50 or 100% of the wage by the reputation gap;
/// returns the offer's slot, or −1.
///
/// Convention: thiscall, ret 8, `shortlist_manager.cpp`, SEH frame with no destructor.
/// `Shortlist_Manager::try_loan_offer_for_player(player, club)`, called by the evaluation of the clubs' lists
/// (`0x84a020`). No owner (contract `main_contract_via_manager`, stored in `[G_CONTRACT_LOOKUP_RESULT]`): the list slot
/// (`exe::shortlist_manager_find_club_shortlist_entry`) goes back to state 1
/// (`exe::shortlist_manager_update_staff_status`) and −1. The interest (`staff_interest_in_club`, with the slot's
/// status) below 3.5: −1.
///
/// The loan period (`FN_TRANSFER_MANAGER_GET_LOAN_PERIOD`): the months until the end and until the start (at least 1);
/// start after the end: −1; a club of nation `KEY_NATION_NORWAY` outside March to September: −1. The interest without
/// wage of 2.75 (outside nations `KEY_NATION_SPAIN`, `KEY_NATION_PORTUGAL`, `KEY_NATION_ITALY`) counts from the start;
/// otherwise from the end. The return date (today's day, at most 28; `exe::date_date_549f70`); unable to end the loan
/// (`exe::transfer_manager_can_end_loan`), after the end of the contract, before 4 weeks from the start of the key
/// nation's window (`exe::key_nation_index_of_club`, `[0xb63d8a]`) or less than 3 weeks from today: −1.
///
/// The wage percentage (100 with a reputation 2500 above the owner's, 0 with 1500 below, otherwise 50) and the offer
/// (`exe::transfer_manager_make_loan_offer`).
/// Original: `try_loan_offer_for_player` (0x842740, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn try_loan_offer_for_player(this: usize, player: usize, club: usize) -> i32 {
    let club = Club(club);
    let entry = ShortlistSlot(exe::shortlist_manager_find_club_shortlist_entry(this, club, player, 0, 0));
    let c = main_contract_via_manager(player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let owner = Club(if c != 0 { (friendly_club_by_id(read_usize(c + contract::CLUB))).addr() } else { 0 });
    if owner.is_null() {
        if !entry.is_null() {
            if LOAN_OFFER_DRY_RUN.load(Relaxed) {
                return -3;
            }
            ((exe::shortlist_manager_update_staff_status(this, player, club, 1, 0) as usize) as u32);
        }
        return -1;
    }
    // simplification: the original pushes the slot’s situation in a register whose remainder is the owning club’s address (base +
    // id); the copy reproduces the dword.
    let status = (read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB)) as u32 & !0xff) | ((entry.flags() >> 1) & 0xf) as u32;
    let f1 = staff_interest_in_club(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), player, club, 0, 1, status);
    covered!("cpw_3 loan interest");
    if !(f1 >= read_f64(rdata::F64_3_5)) { // cpw3: o interesse mínimo;
        return -1;
    }
    let m = (exe::date_get_month(G_CURRENT_DATE) as u32) as u8;
    let c = main_contract_via_manager(player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let a2 = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { Club(0) };
    let period: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    // simplification: the end month goes to the slot of the club argument (the low byte), the rest stays; the others are locals
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, club.addr() as u8 as i8, 0u16);
    period(read_usize(G_TRANSFER_MANAGER), club.addr(), a2.addr(), player, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let today_year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let m16 = m as i8 as i16;
    let until_end = (ano.wrapping_sub(today_year) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let until_start = (ini_ano.wrapping_sub(today_year) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let start = if until_start >= 1 { until_start } else { 1 };
    if start > until_end {
        return -1;
    }
    let nation = club.nation().addr();
    if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_NORWAY) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = staff_interest_in_club(this, player, club, 0, 0, 1);
    let mut months = until_end as u8;
    covered!("cpw_3 count from the start");
    if f2 >= read_f64(rdata::F64_2_75) { // cpw3: the interest to count from the start;
        let n = read_usize(club.nation().addr());
        if n != read_usize(KEY_NATION_SPAIN) && n != read_usize(KEY_NATION_PORTUGAL) && n != read_usize(KEY_NATION_ITALY) {
            months = start as u8;
        }
    }
    let mut month_return = months.wrapping_add(m);
    // simplification: the year goes in the slot of the player argument (the low word; the rest is the player’s address), and the
    // percentage afterwards in its low byte; the copy reproduces the dword.
    let mut slot = (player as u32 & 0xffff_0000) | today_year as u32;
    if month_return as i8 > 0xb {
        let q = (month_return as i8 as i32 as u32) / 12;
        month_return = month_return.wrapping_add((q as u8).wrapping_mul(0xf4));
        slot = slot.wrapping_add(q);
    }
    let day = exe::date_get_day_of_month(G_CURRENT_DATE) as i16;
    // the original rereads the day (second call) when it is not above 28
    let day = if day > 0x1c { 0x1c } else { exe::date_get_day_of_month(G_CURRENT_DATE) as i16 as i32 };
    let mut volta = [0u16; 4];
    exe::date_date_549f70(&mut volta, (day) as i32 as u32, month_return as u32, slot, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    if exe::transfer_manager_can_end_loan(read_usize(G_TRANSFER_MANAGER), club, player, 0, v, 1) == 0 {
        return -1;
    }
    let c = main_contract_via_manager(player);
    if c == 0 || contract_manager_date_is_before(c + contract::CONTRACT_EXPIRES, v) {
        return -1;
    }
    let k = (exe::key_nation_index_of_club(club) as i8) as i32;
    let window = (0xb63d8a + k * 72) as usize;
    let mut t = [0u16; 4];
    let r = exe::date_sub_weeks(v, (&mut t) as *mut [u16; 4] as *mut [u8; 8], 4);
    if contract_manager_date_is_before(r, window) {
        return -1;
    }
    covered!("cpw_3 three weeks");
    let r = exe::date_add_weeks(G_CURRENT_DATE, (&mut t) as *mut [u16; 4] as *mut [u8; 8], 3); // cpw3: the 3 weeks from today
    if contract_manager_date_is_before(v, r) {
        return -1;
    }
    let dif = club.reputation() as i32 - owner.reputation() as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let slot = (slot & !0xff) | pct;
    if LOAN_OFFER_DRY_RUN.load(Relaxed) {
        return -2;
    }
    exe::transfer_manager_make_loan_offer(read_usize(G_TRANSFER_MANAGER), player, club, v, slot, 1, 1) as i32
}

/// clears the bid marker of the club shortlist slots that target the player
/// Original: `Shortlist_Manager::clear_bid_markers_for_player` (0x84aeb0, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn clear_bid_markers_for_player(manager: usize, club: *const i32, player: *const i32) {
    let l = club_shortlist(manager, club);
    if read_u8(l + club_shortlist::ACTIVE) != 1 {
        return;
    }
    for i in 0..40 {
        let v = ShortlistSlot(read_usize(l) + i * 0x25);
        if (v.target_staff_id() as i32) == *player {
            v.set_offer_ref(-1);
        }
    }
}

/// adds to the list every shortlist entry (8 bytes) of the club, found by binary search
/// Original: `Shortlist_Manager::collect_club_shortlist_entries` (0x8420e0, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn collect_club_shortlist_entries(this: usize, club: *const u32, list: usize) {
    for e in club_shortlist_entries(this, *club) {
        (exe::llist_add_tail(list, e, 8) as u32);
    }
}

/// Returns the fee ceiling for a shortlist slot of type `kind`: the club's reputation (word `.reputation()`) times a
/// factor, or -1.
///
/// Convention: stdcall, ret `0xc`, `shortlist_manager.cpp`; `_positions` is not read. Type 4 = x550 in Italy, England
/// and Spain (`[KEY_NATION_ITALY]`, `[KEY_NATION_ENGLAND]`, `[KEY_NATION_SPAIN]`, nation `.nation()`), x475 in Germany
/// and France (`[KEY_NATION_GERMANY]`, `[KEY_NATION_FRANCE]`), x330 elsewhere; type 5 = x550; type 6 = x110; the
/// others, -1.
/// Original: `shortlist_manager_max_fee_for_squad_status` (0x84b230, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_max_fee_for_squad_status(club: usize, _positions: u32, kind: u32) -> i32 {
    let club = Club(club);
    let rep = club.reputation() as i32;
    match kind as i8 {
        4 => {
            let nation = club.nation().addr();
            let eh = |gs: &[usize]| nation != 0 && gs.iter().any(|&g| read_u32(nation) == read_u32(g));
            if eh(&[KEY_NATION_ITALY, KEY_NATION_ENGLAND, KEY_NATION_SPAIN]) {
                rep.wrapping_mul(550)
            } else if eh(&[KEY_NATION_GERMANY, KEY_NATION_FRANCE]) {
                rep.wrapping_mul(475)
            } else {
                rep.wrapping_mul(330)
            }
        }
        5 => rep.wrapping_mul(550),
        6 => rep.wrapping_mul(110),
        _ => -1,
    }
}

/// Tells whether the (player, club or −1) pair is in the shortlist pairs.
///
/// `Shortlist_Manager::is_on_shortlist(player, club)`: with no pair array (`[this]`) it returns 0; otherwise it
/// searches (bsearch `exe::crt_bsearch`, `shortlist_manager_fields::PAIRS_BY_PERSON_COUNT` pairs of 8 bytes, comparer
/// `0x757ea0`) for the key (player id, club id or −1 with no club). al is 1 if found; the 3 high bytes of eax are those
/// of the pointer found (setne al on the bsearch return value), and the port reproduces them.
/// Original: `Shortlist_Manager::is_on_shortlist` (0x842080, shortlist_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_on_shortlist(this: usize, player: usize, club: usize) -> u32 {
    let v = read_usize(this);
    if v == 0 {
        return 0;
    }
    let key: [u32; 2] = [read_usize(player) as u32, if club != 0 { read_usize(club) as u32 } else { u32::MAX }]; // no club: −1
    let p = exe::crt_bsearch((&key) as *const [u32; 2] as usize, v, read_usize(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) as usize as u32, 8 as u32, 0x757ea0);
    (p as u32 & !0xff) | (p != 0) as u32
}

/// Computes the reputation a club aims for in signings, adjusted by manager and chairman, between the club's floor and
/// 10000, and writes it to word `+5` of the club's record. Returns the value.
///
/// `.attendance()` / 10 + 4500; if not human-managed (`exe::human_manager_is_human_club` with 1, 1), the manager
/// (`.manager()`) adjusts by (`.ambition()` − sheet `.patience()`) × 0.0125; the chairman (`.chairman()`, with a sheet)
/// by (`staff::AMBITION` − sheet `non_player::PATIENCE`) / 60 and by (sheet `.resources()` − 10) × 0.025 (each
/// multiplies by 1 + factor, truncating). Never below `.reputation()` nor above 10000.
/// Original: `Shortlist_Manager::update_club_target_reputation` (0x83bf40, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_club_target_reputation(this: usize, club: usize) -> i32 {
    let club = Club(club);
    let r = shopping_list_entry(this, club.addr());
    let target = (r + 5) as *mut i16;
    let u = |p: usize| read_usize(p);
    let w = || core::ptr::read_unaligned(target) as i32;
    let por = |dif: i32, k: usize, v: i32| ftol((dif as f64 * read_f64(k) + read_f64(rdata::F64_1)) * v as f64);
    core::ptr::write_unaligned(target, (club.attendance() / 10 + 0x1194) as i16);
    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) == 0 {
        let manager = club.manager();
        if !manager.is_null() {
            let dif = manager.ambition() as i32 - (manager.non_player().patience() as i8) as i32;
            core::ptr::write_unaligned(target, por(dif, 0x96a740, w()) as i16);
        }
    }
    let p = club.chairman().addr();
    if p != 0 && read_usize(p + staff::NON_PLAYER) != 0 {
        let dif = read_i8(p + staff::AMBITION) as i32 - read_i8(read_usize(p + staff::NON_PLAYER) + non_player::PATIENCE) as i32;
        let v = por(dif, 0x968958, w()) as i16;
        core::ptr::write_unaligned(target, v);
        let sheet = NonPlayer(u(club.chairman().addr() + staff::NON_PLAYER));
        core::ptr::write_unaligned(target, por(sheet.resources() as i32 - 10, 0x967878, v as i32) as i16);
    }
    let floor = club.reputation();
    let v = core::ptr::read_unaligned(target);
    let v = if floor > v { floor } else if v > 0x2710 { 0x2710 } else { v };
    core::ptr::write_unaligned(target, v);
    v as i32
}

/// Tells whether player `j` (sheet at `.player()`) fits scouting list slot `slot` 0..15. Returns only `al`.
///
/// Slots 0..14 require a rating of 15 in the slot's positions (sector and side: `+0xf..+0x16` and `+0x17..+0x19`); 15
/// requires a contract sheet (`.non_player()`), an id below `[G_STAFF_COUNT]`−16 and, with a coach at `.club_job()`
/// with role 11, 13 or 15 at `.job_for_club()`, `exe::human_manager_is_human_club` at zero. No sheet (except slot 15):
/// 0. Slot outside 0..15: the original routine runs (error box).
/// Original: `shortlist_slot_fits` (0x844300, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn shortlist_slot_fits(j: usize, slot: i8) -> u8 {
    let j = Staff(j);
    let sheet = j.player();
    if sheet.is_null() && slot != 15 {
        return 0;
    }
    if !(0..=15).contains(&slot) {
        return original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u8>(IDX_SHORTLIST_SLOT_FITS)(j.addr(), slot);
    }
    let n = |o: usize| read_i8(sheet.addr() + o) >= 15;
    // (sector, side): 0 = sector only
    let (setor, lado): (usize, usize) = match slot {
        0 => (0xf, 0),
        1 => (0x10, 0x19),
        2 => (0x11, 0x18),
        3 => (0x11, 0x17),
        4 => (0x11, 0x19),
        5 => (0x16, 0x18),
        6 => (0x12, 0x19),
        7 => (0x16, 0x17),
        8 => (0x13, 0x18),
        9 => (0x13, 0x19),
        10 => (0x13, 0x17),
        11 => (0x14, 0x18),
        12 => (0x14, 0x19),
        13 => (0x14, 0x17),
        14 => (0x15, 0),
        _ => {
            let manager = j.club_job();
            if !manager.is_null() && matches!(j.job_for_club(), 0xb | 0xf | 0xd) {
                if exe::human_manager_is_human_club(manager, 0, 1, 0, 0) != 0 {
                    return 0;
                }
            }
            let contract = j.non_player();
            return (!contract.is_null() && *(j.addr() as *const i32) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10)) as u8;
        }
    };
    (n(setor) && (lado == 0 || n(lado))) as u8
}

/// the position code from the sector and side mask (28 combinations; the rest 0xe); returns only `al`
/// Original: `position_code` (0x83bd00, shortlist_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn position_code(m: i16) -> u8 {
    match m {
        1 => 0,
        2 => 1,
        4 => 0x11,
        8 => 0x13,
        0x10 => 0x12,
        0x20 => 0x15,
        0x40 => 0x14,
        0x84 => 2,
        0x88 => 5,
        0x90 => 8,
        0xa0 => 0xb,
        0x104 | 0x204 | 0x404 => 4,
        0x108 | 0x208 | 0x408 => 6,
        0x110 | 0x210 | 0x410 => 9,
        0x120 | 0x220 | 0x420 => 0xc,
        0x804 => 3,
        0x808 => 7,
        0x810 => 0xa,
        0x820 => 0xd,
        _ => 0xe,
    }
}

