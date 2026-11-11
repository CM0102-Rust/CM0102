// ---------- batch %%LEVA%%: the AI choosing a shortlist slot's target (shortlist_manager.cpp) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%)(this, slot, club, excluded) // simplification: original only
}
