use super::*;

/// Finds the person's contract with a club in the contract list (`this`, 80-byte records): the person has up to two
/// contract indices in the staff state table (79 bytes per person, +0 and +4) and the one whose club id (+4) matches
/// wins; if neither matches, retries with the linked B team when the club is the A team. Returns the contract record or
/// 0.
pub(crate) unsafe extern "thiscall" fn contract_with_club(this: usize, person: usize, club: usize) -> usize {
    if person == 0 || club == 0 {
        return 0;
    }
    let id = read_i32(person);
    if id >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let e = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE));
    let i0 = read_i32(e);
    if i0 < 0 {
        return 0;
    }
    let base = read_usize(this);
    let r0 = base.wrapping_add((i0 as usize).wrapping_mul(80));
    if r0 == 0 {
        return 0;
    }
    let id_of = |q: usize| read_u32(q + 4);
    let id_club = read_u32(club);
    if id_of(r0) == id_club {
        return r0;
    }
    let i1 = read_i32(e + 4);
    let mut r1 = 0usize;
    if i1 >= 0 {
        r1 = base.wrapping_add((i1 as usize).wrapping_mul(80));
        if r1 != 0 && id_of(r1) == id_club {
            return r1;
        }
    }
    let mut target = club;
    let mut eh_a = person as u32; // the original reuses the argument’s slot
    let pc = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    if pc != 0 && eh_a == 0 {
        target = pc;
    }
    let id_target = read_u32(target);
    if id_of(r0) == id_target {
        return r0;
    }
    if r1 != 0 && id_of(r1) == id_target {
        return r1;
    }
    0
}

/// Returns the low nibble of byte +0x4f of the contract between person `pessoa` and club `clube` (found by the contract
/// manager at [0xae1050]) minus 10 [inference: a contract type/level]. Null club or no contract: 0; national team (id
/// >= clubs - 2 x nations) or invalid person: the club pointer with its low byte cleared.
pub(crate) unsafe extern "C" fn contract_level(person: usize, club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT));
    if read_i32(club) >= limit || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return club as u32 & !0xff;
    }
    let r = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
    if r.is_null() {
        return 0;
    }
    ((r.squad_status() & 0xf) as u32).wrapping_sub(10)
}

/// Address of the 79-byte staff state record of person id in G_STAFF_STATE_TABLE (+0 = main contract index).
pub(crate) unsafe fn contract_manager_staff_state_record(id: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(staff_state::SIZE))
}

/// Copies a 0x50-byte contract record field by field from o to d like the original (dates via the date assign operator,
/// bit fields merged).
pub(crate) unsafe fn copy_contract(d: Contract, o: Contract) {
    for k in (0..0x1c).step_by(4) {
        write_usize(d.addr() + k, read_usize(o.addr() + k));
    }
    for k in 0x1c..0x21 {
        write_u8(d.addr() + k, read_u8(o.addr() + k));
    }
    d.set_release_fee(o.release_fee());
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(d.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as usize, o.addr() + contract::DATE_STARTED);
    exe::date_operator_3d(d.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, o.addr() + contract::CONTRACT_EXPIRES);
    let s = o.contract_type();
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ d.contract_type()) & 0x3f) ^ d.contract_type();
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    d.set_contract_type(((al ^ s) & 0x7f) ^ s);
    d.set_previous_wage(o.previous_wage());
    d.set_squad_number(o.squad_number());
    d.set_club_appearances(((o.club_appearances() as u16)) as i16);
    for k in [0x3dusize, 0x3e, 0x3f] {
        write_u8(d.addr() + k, read_u8(o.addr() + k));
    }
    d.set_flags(((o.flags() as usize)) as u32);
    d.set_morale(((o.morale() as u8)) as i8);
    d.set_grievances(((o.grievances() as usize)) as u32);
    d.set_leaving_on_bosman(o.leaving_on_bosman());
    d.set_transfer_arranged_for(o.transfer_arranged_for());
    d.set_transfer_status(o.transfer_status());
    let s = o.squad_status();
    let a = ((d.squad_status() ^ s) & 0xf) ^ d.squad_status();
    d.set_squad_status(((a ^ s) & 0xf) ^ s);
}

/// Sets the contract type byte (+0x3d) from the job nibble (+0x4f >> 4): 1→75, 2→50, 3→40, 4/5/6/8→0, others untouched.
pub(crate) unsafe fn set_contract_type_from_job(c: Contract) {
    match c.squad_status() >> 4 {
        1 => c.set_happiness(0x4bu8 as i8),
        2 => c.set_happiness(0x32u8 as i8),
        3 => c.set_happiness(0x28u8 as i8),
        4 | 5 | 6 | 8 => c.set_happiness((0) as i8),
        _ => {}
    }
}

/// `CONTRACT_MANAGER::sign_contract(contract, job)` (no SEH), called by the transfer completion
/// (`exe::transfer_manager_move_player_to_club`), by the accepted offer (`complete_accepted_contract_offer`) and by the
/// staff screen: puts the person in the contract club, handles loans, and stores the contract with its value, payroll
/// and club change. A person without a club (`.club_job()`) outside a loan (`.contract_type()` = 5): added to the
/// contract club (`exe::database_add_staff_to_club` with today); on failure: 0. A new job outside a loan with no room
/// at the current club (`exe::database_club_has_room_for_job`): the cancelled-offer news
/// (`exe::contract_manager_post_offer_cancelled_no_space_news`) and 0. A human current club
/// (`exe::human_manager_is_human_club` with 1, 1): the scouts' lists (`FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS`) and
/// `exe::training_manager_update_coach_assignments`. On a loan: a squad above 50, the cancelled-trial news
/// (`exe::contract_manager_post_trial_cancelled_news`) and 0; otherwise `exe::virtual_staff_remove_virtual_players`,
/// added with job 11, the end by the length (`exe::date_days_since` and `exe::date_add_days` from today), job 8 in the
/// high nibble of `.squad_status()`, the start today and `exe::staff_set_squad_flags`. The end is at least 4 weeks
/// after the start (`exe::date_add_weeks`); the end also goes to the person's `staff::DATE_EXPIRES_CLUB`. The place in
/// the array (+0 pointer, `contract_manager_fields::CONTRACT_CAPACITY` capacity,
/// `contract_manager_fields::CONTRACT_COUNT` count): the index from the 79-byte record; with a contract at the same
/// club, the old terms of `contract::SQUAD_NUMBER`..`contract::SQUAD_STATUS` stay (`contract::LEAVING_ON_BOSMAN` 0,
/// `contract::TRANSFER_ARRANGED_FOR` −1) and, in the nation of `KEY_NATION_PORTUGAL` for a non-EU player
/// (`exe::database_is_staff_eu_national`), when the person joined the club (`staff::DATE_JOINED_CLUB`) more than 100
/// weeks ago (today − 100 weeks, `exe::date_sub_weeks`), the second nationality (`staff::SECOND_NATION`) becomes that
/// nation's (with the 1st from `KEY_NATION_BRAZIL` or on a draw of 3); at another club, the type by the job,
/// `contract::LEAVING_ON_BOSMAN`/`contract::TRANSFER_ARRANGED_FOR`, `exe::staff_contract_reset_morale_state` and
/// `contract::CLUB_APPEARANCES` zeroed. Without a record: the first free slot (+0 = −1) gets the contract (type by the
/// job, `exe::staff_contract_reset_morale_state`), the record, the payroll (`exe::finance_update_wage_bill`) and 1;
/// otherwise at the end (grows by 100, realloc `exe::crt_realloc` or malloc `exe::crt_malloc`; without memory, box
/// 0x1637). Then: the value (`contract::WAGE`) in the person's `staff::WAGE`, the morale (+0x14,
/// `exe::staff_adjust_morale`) in type 2 with a player sheet, the payroll (`exe::finance_update_wage_bill`), and with a
/// new job the departure from the current club (`exe::database_remove_staff_from_club`) and the arrival at the contract
/// club (`exe::database_add_staff_to_club`); on failure: the box (0x1672, 0x167a) and 0.
/// Original: `CONTRACT_MANAGER::sign_contract` (0x4cf720, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sign_contract(this: usize, contract: usize, job: u32) -> u32 {
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
    let c = Contract(contract);
    let club = transfer_manager_club_by_id(c.club());
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c.addr()).wrapping_mul(staff::SIZE)));
    let loan = || c.contract_type() & 0x3f == 5;
    if person.club_job().is_null() && !loan() && exe::database_add_staff_to_club(club, person, job, G_CURRENT_DATE, 1, 0) == 0 {
        return 0;
    }
    if job as u8 != person.job_for_club() && !loan() && exe::database_club_has_room_for_job(person.club_job(), job, person) == 0 {
        exe::contract_manager_post_offer_cancelled_no_space_news(this, person, person.club_job());
        return 0;
    }
    if !person.club_job().is_null() && exe::human_manager_is_human_club(person.club_job(), 1, (1) as u32, (0) as u32, 0) != 0 {
        olheiros(read_usize(G_SCOUT_MANAGER), person.club_job().manager().addr());
        exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), person.club_job(), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if loan() {
        covered!("cqg_1 loan");
        if ((exe::database_count_club_squad(club, 0, 0) as i8) as i32) + 1 > 0x32 {
            exe::contract_manager_post_trial_cancelled_news(this, person, club);
            return 0;
        }
        exe::virtual_staff_remove_virtual_players(read_usize(G_VIRTUAL_STAFF), transfer_manager_club_by_id(c.club()), 0);
        exe::database_add_staff_to_club(transfer_manager_club_by_id(c.club()), person, 0xb, G_CURRENT_DATE, 1, 0);
        let n = exe::date_days_since(c.addr() + contract::CONTRACT_EXPIRES, c.date_started() as u32, read_usize(c.addr() + contract::DATE_STARTED + 4) as u32);
        let end = exe::date_add_days(G_CURRENT_DATE, (tmp2.as_mut_ptr()) as usize, (n) as u32); // cqg1: the end by the duration
        exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, end);
        c.set_squad_status((c.squad_status() & 0xf) | 0x80);
        exe::date_operator_3d(c.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as usize, G_CURRENT_DATE);
        exe::staff_set_squad_flags(person, 1);
    }
    // the end at least 4 weeks after the start
    let (ini_dia, ini_ano) = ((c.date_started() as i16), c.date_started_year());
    let end_year = c.contract_expires_year() as u16;
    if ini_ano > end_year || (ini_ano == end_year && !(ini_dia < c.contract_expires())) {
        covered!("cqg_1 ends in 4 weeks");
        let p = exe::date_add_weeks(c.addr() + contract::DATE_STARTED, (tmp2.as_mut_ptr()) as *mut [u8; 8], 4); // cqg1: as 4 semanas
        exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as usize, p);
    }
    exe::date_operator_3d(person.addr() + staff::DATE_EXPIRES_CLUB, (tmp.as_mut_ptr()) as usize, c.addr() + contract::CONTRACT_EXPIRES);
    let idx = read_usize(contract_manager_staff_state_record(read_usize(c.addr()))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = read_usize(this).wrapping_add(off);
        if read_usize(r + 4) == c.club() { // cqg1: the same club
            covered!("cqg_1 same club");
            let r = || read_usize(this).wrapping_add(off);
            c.set_squad_number(read_u8(r() + contract::SQUAD_NUMBER));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                write_u8(c.addr() + k, read_u8(r() + k));
            }
            c.set_grievances(read_usize(r() + contract::GRIEVANCES) as u32);
            c.set_transfer_status(read_u8(r() + contract::TRANSFER_STATUS));
            c.set_leaving_on_bosman(0);
            c.set_transfer_arranged_for(u32::MAX as usize);
            let a = c.squad_status();
            c.set_squad_status(((read_u8(r() + contract::SQUAD_STATUS) ^ a) & 0xf) ^ a);
            let a = c.contract_type();
            c.set_contract_type(((((read_u8(r() + contract::CONTRACT_TYPE) << 1) as i8 >> 1) as u8 ^ a) & contract_type::RESERVE_SQUAD) ^ a);
            c.set_club_appearances(read_u16(r() + contract::CLUB_APPEARANCES) as i16);
            let nation = club.nation().addr();
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_PORTUGAL) && exe::database_is_staff_eu_national(person, 0) == 0 {
                let p = exe::date_sub_weeks(G_CURRENT_DATE, (tmp2.as_mut_ptr()) as *mut [u8; 8], 0x64);
                let (dia, ano) = (read_i16(p), read_u16(p + cm_date::YEAR));
                let after = ano > person.date_joined_club_year() || (ano == person.date_joined_club_year() && dia > (person.date_joined_club() as i16));
                if after {
                    let first = person.nation();
                    if (!first.is_null() && read_usize(first.addr()) == read_usize(KEY_NATION_BRAZIL)) || exe::utils_random(3) == 0 {
                        person.set_second_nation(Nation(read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_PORTUGAL).wrapping_mul(nation::SIZE))));
                    }
                }
            }
        } else {
            set_contract_type_from_job(c);
            c.set_transfer_arranged_for(u32::MAX as usize);
            c.set_leaving_on_bosman(0);
            exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
            c.set_club_appearances((0) as i16);
        }
        copy_contract(Contract(read_usize(this).wrapping_add(off)), c);
    } else {
        let count = read_usize(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let mut i = 0i32;
        while i < count {
            let r = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if read_usize(r) as i32 == -1 { // cqg1: the free place
                covered!("cqg_1 free slot");
                set_contract_type_from_job(c);
                c.set_leaving_on_bosman(0);
                c.set_transfer_arranged_for(u32::MAX as usize);
                exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
                copy_contract(Contract(r), c);
                write_usize(contract_manager_staff_state_record(read_usize(r)), i as u32 as usize);
                let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(c.club()));
                if f != 0 {
                    exe::finance_update_wage_bill(f);
                }
                return 1;
            }
            i += 1;
        }
        if count >= read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) as i32 {
            let cap = (read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) as u32).wrapping_add(0x64);
            write_usize(this + contract_manager_fields::CONTRACT_CAPACITY, cap as usize);
            let p = if read_usize(this) == 0 { exe::crt_malloc(cap.wrapping_mul(0x50)) } else { exe::crt_realloc(read_usize(this), cap.wrapping_mul(0x50)) };
            write_usize(this, p);
            if p == 0 {
                show_error_at(0x9a8190, 0x1637);
            }
        }
        set_contract_type_from_job(c);
        c.set_leaving_on_bosman(0);
        c.set_transfer_arranged_for(u32::MAX as usize);
        exe::staff_contract_reset_morale_state(c, person, transfer_manager_club_by_id(c.club()));
        c.set_club_appearances((0) as i16);
        write_usize(contract_manager_staff_state_record(read_usize(c.addr())), read_usize(this + contract_manager_fields::CONTRACT_COUNT));
        let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT);
        let r = read_usize(this).wrapping_add(n.wrapping_mul(0x50));
        write_usize(this + contract_manager_fields::CONTRACT_COUNT, n.wrapping_add(1));
        copy_contract(Contract(r), c);
    }
    covered!("cqg_1 value on the person");
    person.set_wage(c.wage() as usize); // cqg1: the contract value in the person
    if c.contract_type() & 0x3f == 2 && !person.player().is_null() {
        exe::staff_adjust_morale(person, 0x14, 0);
    }
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(c.club()));
    if f != 0 {
        exe::finance_update_wage_bill(f);
    }
    if job as u8 == person.job_for_club() {
        return 1;
    }
    if exe::database_remove_staff_from_club(person.club_job(), person, 0, 1, 0) == 0 {
        show_error_at(0x9a8190, 0x1672);
        return 0;
    }
    if exe::database_add_staff_to_club(transfer_manager_club_by_id(c.club()), person, job, person.addr() + staff::DATE_JOINED_CLUB, 1, 0) != 0 {
        return 1;
    }
    show_error_at(0x9a8190, 0x167a);
    0
}

/// Multiplies the three contract bonuses (+0x10, +0x14, +0x18) that are not -1 by f, truncated.
pub(crate) unsafe fn scale_contract_bonuses(c: Contract, f: f64) {
    for k in 0..3 {
        let a = c.addr() + contract::GOAL_BONUS + 4 * k;
        let v = read_i32(a);
        if v > -1 {
            write_u32(a, ftol(v as f64 * f) as u32);
        }
    }
}

/// `contract_manager_offer_contract(person, club_arg, out, slot, factor, mode)` (SEH frame for the local STAFF_CONTRACT
/// without a destructor; x87): the contract the club offers: the desired contract × factor, the minimum release fee,
/// the cap, the end date by age, the total by age and the value capped; returns the terms or makes the offer
/// (`exe::contract_manager_add_contract_offer`). Without a club, the club of the person's contract (the 79-byte record
/// in `G_STAFF_STATE_TABLE` → the 0x50-byte contract in `[[G_CONTRACT_MANAGER]]`, kept in `G_CONTRACT_LOOKUP_RESULT`);
/// none: 0. A national side (`exe::database_is_national_team`), a person retiring or retired
/// (`staff_state_status_flags::RETIREMENT_STATE`, bits 0x30 of `staff_state::STATUS_FLAGS` of the record), the club
/// itself outside types 11..15 (`exe::database_is_player_job` of `.job_for_club()`) with the club's list at 2 or
/// without staff (`club::MANAGER`) and manageable (`exe::club_can_be_managed`), a human club
/// (`exe::human_manager_is_human_club`) or a future transfer (`exe::transfer_manager_get_future_transfer`): 0. The
/// desired contract (`exe::staff_contract_set_to_desired_contract` into the local, with the status
/// `exe::shortlist_manager_get_squad_status_for_club`; a 3 with a current reputation (`.current_reputation()` of the
/// sheet) above 5250 becomes 2); a return equal to `rdata::F64_0`: 0. The wage (`contract::UNKNOWN`) and the value
/// (`contract::WAGE`) × factor. With a signing-on fee (`contract::MINIMUM_FEE_RC`): the minimum release fee
/// (`contract::RELEASE_FEE`) by the value of the slot's offer (`exe::transfer_offer_get_weighted_value`: in bands of 50
/// thousand, 1 million and 5 million) or 50 thousand without an offer (100 thousand below the 1st band); without a
/// signing-on fee, with `contract::RELEGATION_RC` or `contract::NON_PROMOTION_RC`, 50 thousand. The cap: the club's
/// maximum wage (`exe::finance_get_max_wage`, with the job from the high nibble of `contract::SQUAD_STATUS`) × (factor
/// + 1) × 0.5 for status 11, otherwise the staff one (`exe::finance_get_max_staff_wage`). The current contract with the
/// club (`exe::contract_manager_get_contract_for_club`). A player (`.player()`): the age in days
/// (`exe::date_days_since`) × k minus the sheet's weight (type 0x14 (`.goalkeeper()`) fixed, otherwise `.stamina()`,
/// `.natural_fitness()` and the person's `.professionalism()`): above `rdata::F64_34` (or a current contract with
/// nibble 6) the end next year (value × `rdata::F64_1_1`; bonus × `rdata::F64_1_25`); above `rdata::F64_31`, in 2 years
/// (bonus × `rdata::F64_1_3`); above `rdata::F64_29` with the club's reputation (`club::REPUTATION`) above the sheet's
/// (`.world_reputation()`) × `rdata::F64_1_2`, the year by age (bonus × `rdata::F64_1_5`). A value above 10 thousand
/// and an age above `rdata::F64_30`: the total (weeks to the end × value) against ability and age (pow `game_pow`):
/// reduces the value for each 52 weeks fewer or lowers the cap to total / weeks. A current contract that ends later: 0;
/// an injury (`FN_INJURY_GET_PLAYER_INJURY`) above 70 days (`injury_entry::DAYS_LEFT`) with `G_INJURY`: 0. With
/// `G_FIFA_TRANSFER_RULES_IN_FORCE` = 1, at most 5 years. A value above the cap × `rdata::F64_1_4`: 0; above the cap:
/// the wage and the bonuses × value / max(cap, 1) and the value = cap. The grievances cleared
/// (`exe::staff_contract_clear_all_grievances`). With `out`: copies the contract and 1; otherwise, in mode 2, or with a
/// current contract in mode 1 that ends after `G_CONTRACT_DATE_PLUS_365_DAYS` + 150 days (`exe::date_add_days`), the
/// offer (`exe::contract_manager_add_contract_offer`); 1.
/// Original: `contract_manager_offer_contract` (0x4cda70, contract_manager.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn contract_manager_offer_contract(this: usize, person: usize, club_arg: usize, out: usize, slot: i32, factor: f64, mode: u32) -> u32 {
    let out = Contract(out);
    let person = Staff(person);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);

    // the local STAFF_CONTRACT (0x50 bytes; the original has it at [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    exe::new_staff_contract(c);
    let mut club = club_arg;
    if club == 0 {
        let k = if person.is_null() {
            0
        } else if (read_usize(person.addr()) as i32) < read_i32(G_STAFF_COUNT) {
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(staff_state::SIZE)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER))) }
        } else {
            0
        };
        write_u32(G_CONTRACT_LOOKUP_RESULT, k as u32);
        if k == 0 {
            return 0;
        }
        club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(k + 4).wrapping_mul(club::SIZE));
        if club == 0 {
            return 0;
        }
    }
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS) & staff_state_status_flags::RETIREMENT_STATE != 0 {
        return 0;
    }
    if exe::database_is_player_job(person.job_for_club() as i8) == 0 && club == person.club_job().addr() {
        let list = read_usize(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER) + shortlist_manager_fields::CLUB_SHORTLISTS);
        if read_u8(list.wrapping_add(read_usize(club).wrapping_mul(9)) + 4) == 2 || (read_usize(club + club::MANAGER) == 0 && exe::club_can_be_managed(club) != 0) {
            return 0;
        }
    }
    if exe::human_manager_is_human_club(club, 0, (1) as u8 as u32, 0 as u32, 0) != 0 {
        return 0;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0 as u32) != 0 {
        return 0;
    }
    write_u32(c + contract::CLUB, read_usize(club) as u32);
    let mut offer = 0usize;
    if slot != -1 {
        let p = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot);
        if p != 0 {
            offer = read_usize(p);
        }
    }
    let mut job: u8 = 0;
    let mut est = exe::shortlist_manager_get_squad_status_for_club(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), person, club, (&mut job) as *mut u8 as usize);
    if est == 3 && person.player().current_reputation() > 0x1482 {
        est = 2;
    }
    // simplification: the status and the job go in dwords with the high bytes of stack leftovers in the original; 0x85fc50 and
    // 0x597db0 leem o byte
    let r = exe::staff_contract_set_to_desired_contract(c, person, club, slot, est as u32, job as u32, 0, 0);
    if r as f64 == read_f64(rdata::F64_0) {
        return 0;
    }
    write_u32(c + contract::UNKNOWN, ftol(read_i32(c + contract::UNKNOWN) as f64 * factor) as u32);
    write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * factor) as u32);
    if read_u8(c + contract::MINIMUM_FEE_RC) != 0 {
        let mut floor: i32 = 0;
        if offer != 0 {
            let v = exe::transfer_offer_get_weighted_value(offer, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                floor = ftol((v - 0x4c4b40) as f64 * read_f64(rdata::F64_1_25) + read_f64(rdata::F64_8425000));
            } else if v > 0xf4240 {
                floor = ftol((v - 0xf4240) as f64 * read_f64(rdata::F64_1_5) + read_f64(rdata::F64_2425000));
            } else if v > 0xc350 {
                floor = ftol((v - 0xc350) as f64 * read_f64(rdata::F64_2_5) + read_f64(rdata::F64_100000));
            } else if read_i32(c + contract::RELEASE_FEE) < 0x186a0 {
                floor = 0x186a0;
            }
        } else if read_i32(c + contract::RELEASE_FEE) < 0xc350 {
            floor = 0xc350;
        }
        if read_i32(c + contract::RELEASE_FEE) < floor {
            write_u32(c + contract::RELEASE_FEE, floor as u32);
        }
    } else if (read_u8(c + contract::RELEGATION_RC) != 0 || read_u8(c + contract::NON_PROMOTION_RC) != 0) && read_i32(c + contract::RELEASE_FEE) < 0xc350 {
        write_u32(c + contract::RELEASE_FEE, 0xc350);
    }
    let mut ceiling: f64 = if job == 0xb {
        let nibble = (read_u8(c + contract::SQUAD_STATUS) >> 4) as u32;
        let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let m = exe::finance_get_max_wage(f, person, nibble);
        (factor + read_f64(rdata::F64_1)) * read_f64(rdata::F64_0_5) * m as f64 // cqf3: the cap of status 11;
    } else {
        let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        exe::finance_get_max_staff_wage(f, job as i8) as f64
    };
    let current = Contract(exe::contract_manager_get_contract_for_club(this, person, club));
    let mut year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let sheet = person.player();
    if !sheet.is_null() {
        let days = exe::date_days_since(G_CURRENT_DATE, read_usize(person.addr() + staff::DATE_OF_BIRTH) as u32, person.date_of_birth_leap_year() as u32);
        let mut x = days as f64 * read_f64(rdata::F64_0_002739726027);
        if (sheet.goalkeeper() as u8) == 0x14 {
            x -= read_f64(rdata::F64_2);
        } else {
            let s = (sheet.stamina() as u8) as i8 as i32 + (sheet.natural_fitness() as u8) as i8 as i32 + (person.professionalism() as u8) as i8 as i32;
            x -= (s as f64 * read_f64(rdata::F64_0_3333333333) - read_f64(rdata::F64_15)) * read_f64(rdata::F64_0_4);
        }
        let end = |c: usize| read_u16(c + contract::CONTRACT_EXPIRES_YEAR) as u32;
        if x > read_f64(rdata::F64_34) || (!current.is_null() && current.squad_status() & 0xf0 == 0x60) {
            if end(c) > year as u32 + 1 {
                covered!("cqf_3 ends next year");
                write_u16(c + contract::CONTRACT_EXPIRES_YEAR, year.wrapping_add(1)); // cqf3: the end next year
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_25));
        } else if x > read_f64(rdata::F64_31) {
            if end(c) > year as u32 + 2 {
                write_u16(c + contract::CONTRACT_EXPIRES_YEAR, year.wrapping_add(2));
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_3));
        } else if x > read_f64(rdata::F64_29) && read_i16(club + club::REPUTATION) as f64 > sheet.world_reputation() as f64 * read_f64(rdata::F64_1_2) {
            let k = ftol(x + read_f64(rdata::F64_0_5)) as i8;
            if end(c) as i32 > (year as i32 - k as i32) + 0x22 {
                write_u16(c + contract::CONTRACT_EXPIRES_YEAR, year.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
            }
            scale_contract_bonuses(Contract(c), read_f64(rdata::F64_1_5));
        }
        if read_i32(c + contract::WAGE) > 0x2710 && x > read_f64(rdata::F64_30) {
            let days_end = exe::date_days_since(c + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
            let mut weeks = days_end / 7;
            let mut total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
            let mut a = game_pow(sheet.current_reputation() as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_0001333333333);
            if !(a < read_f64(rdata::F64_8000)) {
                a = 8000.0;
            }
            let mut b = game_pow(x - read_f64(rdata::F64_30), read_f64(rdata::F64_2));
            if !(b > read_f64(rdata::F64_1)) {
                b = 1.0;
            }
            let limit = ftol(game_pow(a, read_f64(rdata::F64_2)) / b);
            let reduces = (limit as f64 * read_f64(rdata::F64_1_5)) < total as f64;
            let mut done = false;
            if reduces {
                while weeks > 0x68 {
                    weeks -= 0x34;
                    write_u32(c + contract::WAGE, ftol(read_i32(c + contract::WAGE) as f64 * read_f64(rdata::F64_1_1)) as u32);
                    total = weeks.wrapping_mul(read_i32(c + contract::WAGE));
                    if limit >= total {
                        done = true;
                        break;
                    }
                }
            }
            if !done && total > limit {
                let q = limit / weeks;
                if (q as f64) < ceiling {
                    covered!("cqf_3 cap by total");
                    ceiling = q as f64; // cqf3: the cap by the total
                }
            }
        }
        year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    }
    if !current.is_null() {
        if (current.contract_expires_year() as u16) >= read_u16(c + contract::CONTRACT_EXPIRES_YEAR) {
            return 0;
        }
        if read_usize(G_INJURY) != 0 {
            let l = injury(read_usize(G_INJURY), person.addr());
            if l != 0 && read_i16(injury(read_usize(G_INJURY), person.addr()) + injury_entry::DAYS_LEFT) > 0x46 {
                return 0;
            }
            year = read_i16(G_CURRENT_DATE_YEAR) as u16;
        }
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && read_u16(c + contract::CONTRACT_EXPIRES_YEAR) as u32 > year as u32 + 5 {
        write_u16(c + contract::CONTRACT_EXPIRES_YEAR, year.wrapping_add(5));
    }
    let w = read_i32(c + contract::WAGE) as f64;
    if w > ceiling * read_f64(rdata::F64_1_4) {
        return 0;
    }
    if w > ceiling {
        covered!("cqf_3 above the cap");
        let den = if read_f64(rdata::F64_1) > ceiling { read_f64(rdata::F64_1) } else { ceiling };
        let ratio = w / den;
        write_u32(c + contract::UNKNOWN, ftol(read_i32(c + contract::UNKNOWN) as f64 * ratio) as u32);
        scale_contract_bonuses(Contract(c), ratio);
        write_u32(c + contract::WAGE, ftol(ceiling) as u32); // cqf3: the value at the cap
    }
    exe::staff_contract_clear_all_grievances(c, 0);
    if !out.is_null() {
        for k in (0..0x20).step_by(4) {
            write_u32(out.addr() + k, read_usize(c + k) as u32);
        }
        out.set_manager_job_rc(read_u8(c + contract::MANAGER_JOB_RC));
        out.set_release_fee((read_usize(c + contract::RELEASE_FEE) as u32) as usize);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(out.addr() + contract::DATE_STARTED, (tmp.as_mut_ptr()) as *mut u8 as usize, c + contract::DATE_STARTED);
        exe::date_operator_3d(out.addr() + contract::CONTRACT_EXPIRES, (tmp.as_mut_ptr()) as *mut u8 as usize, c + contract::CONTRACT_EXPIRES);
        // +0x35 by the original’s bit fields: 6 bits, 1 bit and 1 bit
        let cl = read_u8(c + contract::CONTRACT_TYPE);
        let dl = out.contract_type();
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        out.set_previous_wage((read_usize(c + contract::PREVIOUS_WAGE) as u32) as usize);
        out.set_contract_type(d2 ^ cl);
        out.set_squad_number(read_u8(c + contract::SQUAD_NUMBER));
        out.set_club_appearances((read_i16(c + contract::CLUB_APPEARANCES) as u16) as i16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            write_u8(out.addr() + k, read_u8(c + k));
        }
        out.set_flags(read_usize(c + contract::FLAGS) as u32);
        out.set_morale(read_u8(c + contract::MORALE) as i8);
        out.set_grievances(read_usize(c + contract::GRIEVANCES) as u32);
        out.set_leaving_on_bosman(read_u8(c + contract::LEAVING_ON_BOSMAN));
        out.set_transfer_arranged_for((read_usize(c + contract::TRANSFER_ARRANGED_FOR) as u32) as usize);
        out.set_transfer_status(read_u8(c + contract::TRANSFER_STATUS));
        out.set_squad_status(read_u8(c + contract::SQUAD_STATUS));
        return 1;
    }
    let mode = mode as u8;
    let enviar = if mode == 2 {
        true
    } else if current.is_null() || mode != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = exe::date_add_days(G_CONTRACT_DATE_PLUS_365_DAYS, (d.as_mut_ptr()) as *mut u8 as usize, 0x96);
        let (dia, ano_d) = (read_i16(p), read_u16(p + cm_date::YEAR));
        let ano_a = current.contract_expires_year() as u16;
        ano_d < ano_a || (ano_d == ano_a && dia < current.contract_expires())
    };
    if enviar {
        // simplification: the role (+0x3d) goes in a dword with high bytes left over from edx in the original; 0x4cc490 reads the byte
        exe::contract_manager_add_contract_offer(this, (c) as usize as *mut u8, (slot) as i32 as usize, (u32::MAX) as u32 as usize, person.job_for_club() as u32, 0, 1, 0);
    }
    1
}

/// Adds delta to a signed byte (contract happiness +0x3d or morale +0x44) clamped to -100..100 like the original.
pub(crate) unsafe fn add_clamped_100(a: usize, delta: i32) {
    let v = read_i8(a);
    let s = v as i32 + delta;
    write_i8(a, if s > 100 { 100 } else if s < -100 { -100 } else { v.wrapping_add(delta as i8) });
}

/// Returns (this side's club, other club) of a match from +0x1c home / +0x20 away.
pub(crate) unsafe fn match_clubs_by_side(game: Fixture, side: usize) -> (usize, usize) {
    if side == 0 { (game.home_club().addr(), game.away_club().addr()) } else { (game.away_club().addr(), game.home_club().addr()) }
}

/// True when match byte +0x3a = 1 and the other club's reputation (+0x80) is below max(own rep × constant, own rep +
/// 1500): that side is skipped.
pub(crate) unsafe fn skip_side_by_reputation(game: Fixture, side: usize) -> bool {
    if (game.category() as i8) != 1 {
        return false;
    }
    let (meu, outro) = match_clubs_by_side(game, side);
    let r = read_i16(meu + club::REPUTATION) as i32;
    let x = r as f64 * read_f64(rdata::F64_1_25);
    let y = (r + 0x5dc) as f64;
    let m = if x > y { x } else { y };
    m > read_i16(outro + club::REPUTATION) as f64
}

/// Person (0x6e-byte record in G_STAFF_TABLE) of a match summary slot, or None when the original skips the slot (id -1
/// or out of range).
pub(crate) unsafe fn staff_of_summary_slot(slot: usize) -> Option<usize> {
    let id = read_i32(slot);
    if id == -1 || read_i32(slot + 4) == -1 {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some(p)
}

/// The three contract bonuses paid for a summary slot (Finance::pay_bonuses `0x599960`).
///
/// `+0x21` × `+0x10`, `+0x27` × `+0x14` and, with the summary word = 1 and `+0x22`, `+0x27` × `+0x18` (the original
/// routine uses `+0x27` again); each only when the byte and the value are positive.
///
/// No bonus in a friendly (`0x5c6710`), in a match of competition [`0x9cf684`] or for contract types 5 and 6.
pub(crate) unsafe fn match_bonuses_for_slot(game: Fixture, contract: Contract, slot: usize, word: i16) -> [Option<i32>; 3] {
    let slot = MatchRecordRow(slot);
    let comp = game.competition();
    if !comp.is_null() && (exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 || read_usize(game.addr()) == read_usize(KEY_CLUB_COMP_RESERVE)) {
        return [None; 3];
    }
    let kind = contract.contract_type() & 0x3f;
    if kind == 6 || kind == 5 {
        return [None; 3];
    }
    let g = slot.goals() as i32;
    let a = slot.assists() as i32;
    let um = (g != 0 && contract.goal_bonus() > 0).then(|| g.wrapping_mul(contract.goal_bonus()));
    let two = (a != 0 && contract.assist_bonus() > 0).then(|| a.wrapping_mul(contract.assist_bonus()));
    let three = (word == 1 && slot.goals_conceded() != 0 && contract.clean_sheet_bonus() > 0).then(|| a.wrapping_mul(contract.clean_sheet_bonus()));
    [um, two, three]
}

/// `CONTRACT_MANAGER::update_after_match(game, summary)`, called by the closing of the day's match (`close_day_match`):
/// after each match, contract bonuses, happiness by minutes played, morale by a draw and by the result, and the
/// development and happiness of those left out. With the match's `.home_side_mark()` and `.away_side_mark()`, the
/// winner by the score (`.home_goals_extra_time()`/`.away_goals_extra_time()`; with `.home_goals_extra_time()` = −1,
/// `.home_goals()`/`.away_goals()`) and a copy of the 50 squad slots (`club::SQUAD`) of each club. First, per side
/// (club B of the partner `exe::club_get_linked_b_team` and the side skipped by reputation are left out), the 20 slots
/// of the summary (`match_stats_record::HOME_LINEUP` + side × 0x625, `match_record_row::SIZE` bytes): outside national
/// sides, the contract with the club (`exe::contract_manager_get_contract_for_club`) pays the bonuses
/// (match_bonuses_for_slot); the minutes (`.minute_off()` for starters, 90 without a value; 90 − `.minute_on()` for
/// substitutes) change the happiness (`contract::HAPPINESS`, minutes × 10 / 90, held at 50 below 50) or, for national
/// sides, the +9 of the 79-byte record; outside national sides and friendlies, the substitutes and the starters with
/// little playing time draw (2 × draw(30)) against the minutes: the happiness drops 4, `contract::UNHAPPY_FLAG` counts
/// and the morale drops (`exe::contract_manager_lower_match_morale`); otherwise `contract::MATCHES_LEFT_OUT` counts
/// (with 5 or more, outside the human club, squad mark 1 in `exe::staff_set_squad_flags`) and the morale rises
/// (`exe::contract_manager_raise_match_morale`). The winner (and the loser) draws (21 − `staff::AMBITION`) below 4 for
/// +3 (−3) on morale `contract::MORALE`; the person leaves the squad copy. Then, those left in the copy (with a player
/// sheet, a contract, without suspension `exe::discipline_is_player_banned`, injury
/// `exe::injury_get_player_injury_status`, national duty `exe::nationalteams_get_international_duty_club` or loan)
/// develop (`exe::update_player_reputation_after_match` with the average rating of `G_PLAYER_STATS`,
/// `player_stats_fields::RATING_SUM` / `player_stats_fields::RATED_COUNT` or `rdata::F64_6_8` with no rating, − 0.05
/// `rdata::F64_0_05`), `contract::UNHAPPY_FLAG` counts and, from 8 (14 with a partner), outside the human club and the
/// partner, they leave the first team (`exe::contract_manager_get_squad_placement` 2 or 4 →
/// `exe::contract_manager_toggle_squad_placement`); the happiness drops 8 and the morale drops.
/// Original: `CONTRACT_MANAGER::update_after_match` (0x4ca6d0, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_after_match(this: usize, game: usize, summary: usize) {
    let game = Fixture(game);
    if (game.home_side_mark() as i8) == 0 || game.away_side_mark() == 0 {
        return;
    }
    // the original sets ecx = this before 0x4cb0a0 (stdcall in the Rust copy): thiscall with 3 on the stack serves both
    let (c47, c48) = (game.home_goals_extra_time(), game.away_goals_extra_time());
    let (a, b) = if c47 == -1 { (game.home_goals(), game.away_goals()) } else { (c47, c48) };
    let won = [(a > b) as u32, (a < b) as u32];
    let mut list = [[0usize; 50]; 2];
    for (lado, l) in list.iter_mut().enumerate() {
        let c = match_clubs_by_side(game, lado).0;
        core::ptr::copy_nonoverlapping((c + club::SQUAD) as *const usize, l.as_mut_ptr(), 50);
    }
    let mut eh_a = 0u32;
    for lado in 0..2usize {
        let club = match_clubs_by_side(game, lado).0;
        if exe::club_get_linked_b_team(club, &mut eh_a, 1) != 0 && eh_a == 0 {
            continue;
        }
        if skip_side_by_reputation(game, lado) {
            continue;
        }
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let base = summary + lado * 0x625;
        for k in 0..20usize {
            let slot = MatchRecordRow(base + match_stats_record::HOME_LINEUP + k * match_record_row::SIZE);
            let Some(person) = staff_of_summary_slot(slot.addr()) else { continue };
            let contract = if exe::database_is_national_team(club) != 0 {
                0
            } else {
                let c = exe::contract_manager_get_contract_for_club(this, person, club);
                if c == 0 {
                    continue;
                }
                for v in match_bonuses_for_slot(game, Contract(c), slot.addr(), read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    exe::finance_pay_bonuses(fin, v);
                }
                c
            };
            let minutos: i8 = if k < 11 {
                let v = slot.minute_off();
                if v > -1 { v } else { 0x5a }
            } else {
                let v = slot.minute_on();
                if v <= -1 {
                    0
                } else {
                    let e = 0x5a - v as i32;
                    if e <= 0 { 0 } else { e as i8 }
                }
            };
            if k < 11 || slot.minute_on() != -1 {
                let q = ((minutos as i32 * 10) / 90) as i8;
                if exe::database_is_national_team(club) != 0 {
                    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)) + 9;
                    write_i8(r, read_i8(r).wrapping_add(q));
                    if read_i8(r) > 100 {
                        write_i8(r, 100);
                    }
                } else {
                    let al = read_i8(contract + contract::HAPPINESS);
                    let s = al as i32 + q as i32;
                    write_i8(contract + contract::HAPPINESS, if al > 0x32 {
                        if s > 100 { 100 } else if s < -100 { -100 } else { al.wrapping_add(q) }
                    } else if s > 0x32 {
                        0x32
                    } else {
                        let v = al.wrapping_add(q);
                        if v > 100 { 100 } else if v < -100 { -100 } else { v }
                    });
                }
            }
            if exe::database_is_national_team(club) == 0 && (read_i32(game.addr()) < 0 || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), game.competition()) == 0) {
                let testa = k >= 11 || !(minutos > 0x1e || slot.injured_off() != 0 || slot.final_stamina() < 0x46);
                let mal = testa && {
                    covered!("cqa_8 tests the morale");
                    let m = read_i8(contract + contract::MORALE) as i32;
                    let r1 = exe::utils_random(0x1e); // cqa8: the first morale random draw
                    let v = r1 as f64 + (read_f64(rdata::F64_45) - m as f64 * read_f64(rdata::F64_0_303030303));
                    let r2 = exe::utils_random(0x1e);
                    v - r2 as f64 > minutos as f64
                };
                let p = contract as *mut u8;
                if mal {
                    *p.add(0x3f) = 0;
                    if *p.add(0x3e) != 100 {
                        *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
                    }
                    add_clamped_100(contract + contract::HAPPINESS, -4);
                    exe::contract_manager_lower_match_morale(this, person, club, contract);
                } else {
                    *p.add(0x3e) = 0;
                    if *p.add(0x3f) != 100 {
                        *p.add(0x3f) = (*p.add(0x3f)).wrapping_add(1);
                    }
                    if read_i8(contract + contract::MATCHES_LEFT_OUT) >= 5 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
                        exe::staff_set_squad_flags(person, 1);
                    }
                    exe::contract_manager_raise_match_morale(this, person, club, contract);
                }
            }
            if won[lado] != 0 {
                covered!("cqa_8 winner morale");
                let r = exe::utils_random(0x15 - read_i8(person + staff::AMBITION) as i32); // cqa8: the winner random draw
                if r < 4 && contract != 0 {
                    add_clamped_100(contract + contract::MORALE, 3);
                }
            } else if won[1 - lado] != 0 {
                let r = exe::utils_random(0x15 - read_i8(person + staff::AMBITION) as i32);
                if r < 4 && contract != 0 {
                    add_clamped_100(contract + contract::MORALE, -3);
                }
            }
            if let Some(x) = list[lado].iter_mut().find(|x| **x == person) {
                *x = 0;
            }
        }
    }
    for lado in 0..2usize {
        let (club, outro) = match_clubs_by_side(game, lado);
        let limit: i8 = if exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) != 0 { 14 } else { 8 };
        if skip_side_by_reputation(game, lado) {
            continue;
        }
        for i in 0..50usize {
            let person = list[lado][i];
            if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) || read_usize(person + staff::PLAYER) == 0 || exe::database_is_national_team(club) != 0 {
                continue;
            }
            let contract = exe::contract_manager_get_contract_for_club(this, person, club);
            if contract == 0 {
                continue;
            }
            if read_i32(game.addr()) != -2 && exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), person, game.competition(), 1, 0) != 0 {
                continue;
            }
            if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) != 0 || exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0) != 0 {
                continue;
            }
            // on loan: the person’s second contract (+4 of the 79-byte record)
            let idx = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)) + 4);
            if idx >= 0 && (idx as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)) != 0 {
                continue;
            }
            let m = read_usize(G_PLAYER_STATS);
            let x = read_f64(m + player_stats_fields::RATED_COUNT);
            let c = read_f64(rdata::F64_0);
            // fcomp + C3: equal or unordered goes to the constant
            let rating = if !(x < c || x > c) { read_f64(rdata::F64_6_8) } else { read_f64(m + player_stats_fields::RATING_SUM) / x };
            covered!("cqa_8 develops who was left out");
            exe::update_player_reputation_after_match(0, person, club, outro, game, (rating - read_f64(rdata::F64_0_05)) as f32, 0, 0); // cqa8: the progress of those left out;
            let p = contract as *mut u8;
            *p.add(0x3f) = 0;
            if *p.add(0x3e) != 100 {
                *p.add(0x3e) = (*p.add(0x3e)).wrapping_add(1);
            }
            if read_i8(contract + contract::UNHAPPY_FLAG) >= limit && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) == 0 {
                let l = exe::contract_manager_get_squad_placement(this, person, club);
                if l == 2 || l == 4 {
                    exe::contract_manager_toggle_squad_placement(this, person, club, 0);
                }
            }
            add_clamped_100(contract + contract::HAPPINESS, -8);
            exe::contract_manager_lower_match_morale(this, person, club, contract);
        }
    }
}

/// the morale of the player record (`.player_morale()`), for the screens
/// Original: `contract_manager_get_player_morale` (0x4cc2d0, contract_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn contract_manager_get_player_morale(person: usize) -> u32 {
    let person = Staff(person);
    (person.addr() as u32 & 0xffff_ff00) | (person.player().player_morale() as u8) as u32
}

/// Copies the zero-terminated text at a (up to 0x400 bytes) into a Vec, for the text comparers.
pub(crate) unsafe fn c_string_bytes(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// `contract_manager_write_morale_text(person, out, _)` (stdcall, ret 0xc): writes the morale word into a text, for the
/// screens. The morale (`.player_morale()` of the sheet): below 4 "Very Low", 8 "Low", 12 "Ok", 15 "Good", 18 "Very
/// Good", otherwise "Superb", written to the output (`FN_LANGLIB_SPRINTF` with 0xad9c64).
/// Original: `contract_manager_write_morale_text` (0x4cc2e0, contract_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_text(person: usize, out: usize, _x: u32) {
    let person = Staff(person);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let m = person.player().player_morale();
    let f = if m < 4 {
        0x9a8320usize
    } else if m < 8 {
        0x9a82fc
    } else if m < 0xc {
        0x9a82dc
    } else if m < 0xf {
        0x9a82b8
    } else if m < 0x12 {
        0x9a8290
    } else {
        0x9a826c
    };
    format(out, f, 0xad9c64usize);
}

/// Main contract record (0x50 bytes in the contract manager) of a person via G_STAFF_STATE_TABLE, or 0 when none/out of
/// range.
pub(crate) unsafe fn main_contract_of_staff(person: Staff) -> usize {
    if person.is_null() || read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(staff_state::SIZE))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(read_usize(read_usize(G_CONTRACT_MANAGER))) }
}

/// Caps value w at the contract release fee (+0x21) when the release clause is active, then truncates (_ftol).
pub(crate) unsafe fn cap_value_at_release_fee(contract: Contract, w: f64) -> i32 {
    if !contract.is_null() && exe::staff_contract_has_active_release_clause(contract) != 0 {
        let cl = (contract.release_fee() as i32) as f64;
        return ftol(if !(w >= cl) { w } else { cl });
    }
    ftol(w)
}

/// `CONTRACT_MANAGER::calc_player_value(person, club, mode, ability, _)`: the player value by peak age and age, record
/// ratings, ability as seen by the club manager, club nation, contract days left and release clause. The main contract
/// goes to `G_CONTRACT_LOOKUP_RESULT` and the contract with his club (`exe::contract_manager_get_contract_for_club`,
/// into the incoming `this`). Without a player sheet (`.player()`): the compensation
/// (`exe::transfer_manager_calc_compensation_fee` with 1) by the contract club, or 0. The base and the peak age by the
/// sheet's positions (`.attacker()`/`.attacking_midfielder()`/`.midfielder()`/`.defensive_midfielder()` ≥ 15: 13, 11,
/// 10, 9 million; goalkeeper `.goalkeeper()` = 20: 6 million, 33; otherwise 8 million; peak 28 or 29). The age factor
/// (peak − age, with the correction above `rdata::F64_5`) × the sheet's ratings (`player_sheet::FLAIR`, `.finishing()`,
/// `.vision()`, `player_sheet::PLAYER_PACE`, `player_sheet::LEADERSHIP`, outside goalkeepers) and
/// `.injury_proneness()`²; the reputation (`.home_reputation()`) limits the factor. The ability: without a club, the
/// current one (`.current_reputation()`); with a club, the one seen by the manager
/// (`exe::match_man_judge_player_ability` if `ability` = −1, with the seen reputation
/// `exe::database_get_staff_reputation_for_viewer`) × 50 + the reputation + up to 5000 × factor, ×
/// `rdata::F32_0_3333333433`, between 1 and 12000; the potential (`.world_reputation()`) above 5000 (and 8000) before
/// the peak pulls it up. Value = ability² × base × the table 0x9a7f30 (by age, with experience
/// `.int_apps()`/`.nation()`) × the club's nation (`.nation()`: `nation::STATE_OF_DEVELOPMENT` and
/// `nation::LEAGUE_STANDARD` or the division `.reputation()`; without a club, draw(3) + 1), with a floor. `mode`: the
/// nation `KEY_NATION_PORTUGAL` and the clause. Otherwise: the contract days (with the Bosman rule
/// `exe::transfer_manager_player_club_uses_bosman`) and the nations `KEY_NATION_PORTUGAL`, `KEY_NATION_FRANCE`,
/// `KEY_NATION_SOUTH_KOREA`; at the end the active release clause (`exe::staff_contract_has_active_release_clause`, in
/// `cap_value_at_release_fee`) limits it.
/// Original: `CONTRACT_MANAGER::calc_player_value` (0x4d0c60, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn calc_player_value(this: usize, person: usize, club: usize, mode: u32, ability: u32, _x: u32) -> i32 {
    let person = Staff(person);
    let club = Club(club);
    let c1080 = Contract(main_contract_of_staff(person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c1080.addr());
    let transfer_manager_contract_club = if c1080.is_null() { 0 } else { (friendly_club_by_id(c1080.club())).addr() };
    let contract = Contract(exe::contract_manager_get_contract_for_club(this, person, transfer_manager_contract_club));
    let sheet = person.player();
    if sheet.is_null() {
        if transfer_manager_contract_club == 0 {
            return 0;
        }
        // the original sets ecx = [0xde1f54] (the Rust copy is stdcall): thiscall with 3 on the stack works for both
        return exe::transfer_manager_calc_compensation_fee(read_usize(G_TRANSFER_MANAGER), person, transfer_manager_contract_club, 1);
    }
    let (base, auge): (f64, i8) = if sheet.attacker() >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: the base of those who play up front
    } else if sheet.attacking_midfielder() >= 0xf {
        (11_000_000.0, 0x1d)
    } else if sheet.midfielder() >= 0xf {
        (10_000_000.0, 0x1d)
    } else if sheet.defensive_midfielder() >= 0xf {
        (9_000_000.0, 0x1d)
    } else if sheet.goalkeeper() == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let age = person.age() as i8;
    let dif = auge as i32 - age as i32;
    let mut t = dif as f64;
    if t > read_f64(rdata::F64_5) {
        t = read_f64(rdata::F64_8) - t * read_f32(rdata::F32_0_5) as f64;
    }
    let a = ftol(t).wrapping_abs();
    let mut f = t * a as f64 * read_f32(rdata::F32_0_3333333433) as f64 * read_f64(rdata::F64_0_08333333333) + read_f64(rdata::F64_1);
    if sheet.goalkeeper() < 0xf {
        let rating = |o: usize, c: usize| (read_i8(sheet.addr() + o) as i32 - 0xf) as f64 * read_f64(c) + read_f64(rdata::F64_1);
        let fa = rating(player_sheet::FLAIR, 0x968ef8);
        let fb = sheet.finishing() as f64 * read_f64(rdata::F64_0_005) + read_f64(rdata::F64_1);
        let fc = sheet.vision() as f64 * read_f64(rdata::F64_0_003333333333) + read_f64(rdata::F64_1);
        let fd = rating(player_sheet::PLAYER_PACE, 0x968ef8);
        let fe = rating(player_sheet::LEADERSHIP, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = sheet.injury_proneness() as i32;
    let factor = ((read_f64(rdata::F64_1_33) - (q * q) as f64 * read_f64(rdata::F64_0_05) * read_f64(rdata::F64_0_034)) * f) as f32;
    let mut r = sheet.home_reputation() as f64 * read_f64(rdata::F64_0_0003333333333) + read_f64(rdata::F64_0_17);
    if !(r >= read_f64(rdata::F64_1_1)) {
        r = read_f64(rdata::F64_1_1);
    }
    let x: f32 = if !(factor as f64 >= read_f64(rdata::F64_0_01)) {
        read_f64(rdata::F64_0_01) as f32
    } else if !(r >= factor as f64) {
        r as f32
    } else {
        factor
    };
    let mut v: u32 = if club.is_null() {
        sheet.current_reputation() as i32 as u32
    } else {
        let manager = club.manager().addr();
        let rep = (exe::database_get_staff_reputation_for_viewer(person, manager, 0) as u16) as i16 as i32;
        let h = if ability as u16 == 0xffff {
            exe::match_man_judge_player_ability(manager, person, club, 1, rep as i16) as u16
        } else {
            ability as u16
        };
        // only the ax of h × 50 counts (movsx afterwards); negative becomes 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let ceiling = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = ftol((s as f64 + ceiling as f64 * x as f64) * read_f32(rdata::F32_0_3333333433) as f64) as u32;
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = sheet.world_reputation();
    if pot > 0x1388 && (age as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && age <= auge {
            let z = pot as f64 * read_f64(rdata::F64_2_25) + (v >> 1) as f64 - read_f64(rdata::F64_12500);
            let e = v as f64;
            v = ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * read_f64(rdata::F64_1ENEG_08);
    let mut age_ef = age;
    let exp = person.int_apps();
    if exp != 0 && !person.nation().is_null() {
        let n = (person.nation().reputation() as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let sum = if m < 0 { 0 } else if n >= dif { dif } else { n };
        age_ef = age_ef.wrapping_add(sum as i8);
    }
    let mut i = auge.wrapping_sub(age_ef).wrapping_add(10); // cqa9: the table index by age
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let table = read_i8(0x9a7f30 + i as usize) as i32;
    let mut w = table as f64 * vv * read_f64(rdata::F64_0_01);
    let c890 = read_f64(rdata::F64_1);
    let k = if club.is_null() {
        (exe::utils_random(3) + 1) as f64
    } else {
        let div = club.division();
        let nat = club.nation().addr();
        if nat == 0 {
            c890
        } else {
            let mut t = read_i8(nat + nation::STATE_OF_DEVELOPMENT) as f64;
            if !(t >= c890) || t > read_f64(rdata::F64_3) {
                t = read_f64(rdata::F64_3);
            }
            if !div.is_null() {
                t = (t + c890) * read_f64(rdata::F64_0_5);
            }
            match club.professional_status() as i8  {
                2 => t = t + t,
                3 => t *= read_f64(rdata::F64_7_5),
                _ => {}
            }
            let n = if !div.is_null() { div.reputation() as i32 } else { read_i8(nat + nation::LEAGUE_STANDARD) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(club.reputation() as i32);
            e as f64 / (t * read_f64(rdata::F64_1000))
        }
    };
    let dd = read_f64(rdata::F64_20) - k;
    let mut l = c890 - dd * dd * read_f64(rdata::F64_0_2) * read_f64(rdata::F64_0_02);
    if !(l >= read_f64(rdata::F64_0_1)) {
        l = read_f64(rdata::F64_0_1);
    } else if l > read_f64(rdata::F64_1_1) {
        l = read_f64(rdata::F64_1_1);
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * read_f64(rdata::F64_65000) >= w) {
        w = w * read_f64(rdata::F64_0_25) + k2 * read_f64(rdata::F64_32500);
    }
    let floor = k * read_f64(rdata::F64_5000);
    if !(w >= floor) {
        w = floor;
    }
    let nation_and = |g: usize| -> bool {
        let nat = club.nation().addr();
        !club.is_null() && nat != 0 && read_usize(nat) == read_usize(g)
    };
    // the nation [0x9cf434]: below [0x968e90] the curve with a floor, otherwise × [0x9689d8]
    let curva = |w: f64, floor: f64| -> f64 {
        if !(w >= read_f64(rdata::F64_6000000)) {
            let v = (w * read_f64(rdata::F64_1_666666667ENEG_07) * read_f64(rdata::F64_0_6) + read_f64(rdata::F64_0_3)) * w;
            if !(read_f64(rdata::F64_5000) > v) { v } else { floor }
        } else {
            w * read_f64(rdata::F64_0_9)
        }
    };
    if mode as u8 != 0 {
        if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, 5000.0);
        }
        return cap_value_at_release_fee(contract, w);
    }
    let mut nations = true; // 0x4d1480 (the other nations) or 0x4d14ad (the [0x9cf434] one before)
    if contract.is_null() {
        w = read_f64(rdata::F64_0);
    } else {
        if exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), person) != 0 {
            let year = |a: usize| read_u16(a) as i32;
            let mut days = (year(contract.addr() + contract::CONTRACT_EXPIRES_YEAR) - year(G_CURRENT_DATE_YEAR)).wrapping_mul(365) + contract.contract_expires() as i32 - read_i16(G_CURRENT_DATE) as i32;
            if days < 1 {
                return 0;
            }
            let mut w2 = w;
            if club.addr() == transfer_manager_contract_club && contract.happiness() > 0 {
                if contract.contract_type() & 0x3f == 2 {
                    days = days + 7 * contract.happiness() as i32 + 0xc8;
                } else {
                    w2 = w * read_f64(rdata::F64_0_5);
                }
            }
            let x = days as f64 * read_f64(rdata::F64_0_001428571429);
            if days < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = read_f64(rdata::F64_2_5) - x;
                if !(dv >= c890) {
                    dv = read_f32(rdata::F32_1) as f64;
                }
                if dv > read_f64(rdata::F64_2) {
                    dv = read_f32(rdata::F32_2) as f64;
                }
                w = w2 / dv;
            }
        }
        nations = false;
    }
    if !nations {
        // 0x4d14ad: zero (or unordered) value goes to the other nations; with no club, to the end
        if !(w < read_f64(rdata::F64_0) || w > read_f64(rdata::F64_0)) {
            nations = true;
        } else if club.is_null() {
            return cap_value_at_release_fee(contract, w);
        } else if nation_and(KEY_NATION_PORTUGAL) {
            w = curva(w, read_f64(rdata::F64_5000));
            return cap_value_at_release_fee(contract, w);
        } else {
            nations = true;
        }
    }
    if nations && !club.is_null() {
        if nation_and(KEY_NATION_FRANCE) {
            w *= read_f64(rdata::F64_0_8);
        } else if nation_and(KEY_NATION_SOUTH_KOREA) {
            w *= read_f64(rdata::F64_0_25);
            let c = read_f64(rdata::F64_125000);
            if w > c {
                w = ftol((w - c) * read_f64(rdata::F64_0_5) + c) as f64;
            }
            let c = read_f64(rdata::F64_250000);
            if w > c {
                w = ftol((w - c) * read_f64(rdata::F64_0_5) + c) as f64;
            }
        }
    }
    cap_value_at_release_fee(contract, w)
}

/// Memory regions the calc_player_value comparer watches: RNG state, date statics and the contract lookup global
/// 0xae1080.
pub(crate) fn player_value_regions() -> [(usize, usize); 11] {
    let (e, c) = (crate::rng::STATE_REGIONS, DATE_OP_STATICS);
    [e[0], e[1], (G_CONTRACT_LOOKUP_RESULT, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// `contract_manager_write_morale_comparison_text(a, b, out, _)` (stdcall, ret 0x10): writes the morale difference of
/// two players as a word, for the comparison screen. The difference of the morales (`.player_morale()` of the sheets, a
/// − b, as a byte) becomes "Superior" (down to −14 and from 13), "Better" (−13 to −8 and 7 to 12), "Marginal" (−7 to −4
/// and 3 to 6) or "Same" (−3 to 2) in the output (`FN_LANGLIB_SPRINTF` with 0xad9c64); returns the difference (the game
/// reads al).
/// Original: `contract_manager_write_morale_comparison_text` (0x4cc3a0, contract_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn contract_manager_write_morale_comparison_text(a: usize, b: usize, out: usize, _x: u32) -> u8 {
    let b = Staff(b);
    let a = Staff(a);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let dif = a.player().player_morale().wrapping_sub(b.player().player_morale());
    let f = if dif < -13 {
        0x9a8348usize
    } else if dif < -7 {
        0x9a83bc
    } else if dif < -3 {
        0x9a8394
    } else if dif < 3 {
        0x9a8370
    } else if dif < 7 {
        0x9a8394
    } else if dif < 13 {
        0x9a83bc
    } else {
        0x9a8348
    };
    format(out, f, 0xad9c64usize);
    dif as u8
}

/// `CONTRACT_MANAGER::get_squad_flags_for_club(person, club)`, called by the club screens: returns the squad flags the
/// club screen shows. The person's squad flags (`.playing_squad()` without the national side bits, & 0xf3), unless she
/// is on loan (second contract, +4 of the 79-byte record) to a club other than `club` or its partner
/// (`exe::club_get_linked_b_team`): with a partner, 1; otherwise 2 with bit 0x40 of `.contract_type()` of the main
/// contract (the reserve bit), 1 without. Without a club, the "Error" box (the original). The game reads al.
/// Original: `CONTRACT_MANAGER::get_squad_flags_for_club` (0x4cc130, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_squad_flags_for_club(this: usize, person: usize, club: usize) -> u8 {
    let person = Staff(person);
    if club == 0 || person.is_null() {
        // simplification: without a club the error box; without a person the original reads [0x5e] and crashes: both stay with the original
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person.addr(), club);
    }
    let general = || person.playing_squad() & 0xf3;
    if read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return general();
    }
    let reg = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person.addr()).wrapping_mul(staff_state::SIZE));
    let i = read_usize(reg + 4) as i32;
    if i < 0 {
        return general();
    }
    let c = Contract((i as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)));
    if c.is_null() || c.club() == read_usize(club) {
        return general();
    }
    let p = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
    if p != 0 && c.club() == read_usize(p) {
        return general();
    }
    if exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = read_usize(reg) as i32;
    if i0 < 0 {
        // the original reads [0x35] of the null contract and crashes
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB)(this, person.addr(), club);
    }
    let c0 = Contract((i0 as usize).wrapping_mul(0x50).wrapping_add(read_usize(this)));
    (c0.contract_type() & 0x40 != 0) as u8 + 1
}

/// Calls STAFF_CONTRACT::calc_contract_value (contract, person, offer) → value.
pub(crate) unsafe fn calc_contract_value(contract: usize, person: Staff, offer: usize) -> i32 {
    exe::staff_contract_calc_contract_value(contract, person, offer)
}

/// Writes a person's id into news slot 2 and the +0x33 keys of +4/+8/+0xc into slots 3-5.
pub(crate) unsafe fn set_news_staff_slots(n: usize, p: Staff) {
    exe::newsitem_set_slot(n, 2, read_usize(p.addr()) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(p.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 4, read_usize(p.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 5, read_usize(p.common_name() + names::ID) as u32);
}

/// `CONTRACT_MANAGER::post_wants_new_contract_news(contract)` (SEH frame for the local news item), called by the daily
/// staff pass (0x5ce2d0): posts news item 0xbc2, that the player wants a new contract, to the club manager, unless he
/// is queued for a free transfer or in an offer. Without a contract: the "Error" box (line 0x2d91). A person queued for
/// a free transfer (`exe::transfer_manager_is_queued_for_free_transfer`), with offers
/// (`exe::transfer_manager_get_offers_for_player`) or offered in exchange
/// (`exe::transfer_manager_get_exchange_offers_for_player`): nothing. Otherwise news item 0xbc2 to the manager of the
/// contract club (`exe::news_send_to_club_manager`): the club, the person, the squad status (`.squad_status()` >> 4),
/// 0, the contract end (`.contract_expires()` signed day, `.contract_expires_year()` year), the wage (`.wage()`) and,
/// in slot 11, 1 if the loyalty (`.loyalty()`) is below 10, `.professionalism()` below 5 and the sheet's reputation
/// (`.world_reputation()`) above 4750.
/// Original: `post_wants_new_contract_news` (0x4d9f80, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_wants_new_contract_news(this: usize, contract: usize) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2d91);
    }
    let club = friendly_club_by_id(contract.club());
    let p = Staff((staff_records_staff_by_id(read_usize(contract.addr()))).addr());
    let tm = read_usize(G_TRANSFER_MANAGER);
    if exe::transfer_manager_is_queued_for_free_transfer(tm, p) != 0 || exe::transfer_manager_get_offers_for_player(tm, p, 0, 0) != 0 || exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0) != 0 {
        return;
    }
    covered!("cpw_1 new contract news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbc2, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, (contract.squad_status() >> 4) as u32);
    exe::newsitem_set_slot(n, 7, 0);
    exe::newsitem_set_slot(n, 8, contract.contract_expires() as i32 as u32);
    exe::newsitem_set_slot(n, 9, contract.contract_expires_year() as u16 as u32);
    exe::newsitem_set_slot(n, 10, (contract.wage() as usize) as u32);
    let sheet = p.player();
    let reputed = ((p.loyalty() as u8) as i8) < 0xa && ((p.professionalism() as u8) as i8) < 5 && !sheet.is_null() && sheet.world_reputation() > 0x128e;
    exe::newsitem_set_slot(n, 11, reputed as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_transfer_request_withdrawn_news(contract, reason)` (SEH frame for the local news item):
/// posts news item 0xbc0 of a withdrawn transfer request, with the reason, to the club manager. Without a contract: the
/// "Error" box (line 0x2ba6). Otherwise news item 0xbc0 to the manager of the contract club
/// (`exe::news_send_to_club_manager`): the club, the person, the reason and, in slot 7, 1 if the contract's
/// `.matches_left_out()` is above 1.
/// Original: `post_transfer_request_withdrawn_news` (0x4d9460, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_transfer_request_withdrawn_news(this: usize, contract: usize, reason: u32) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2ba6);
    }
    covered!("cpw_1 request withdrawn");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbc0, 0);
    let club = friendly_club_by_id(contract.club());
    let p = staff_records_staff_by_id(read_usize(contract.addr()));
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, reason);
    exe::newsitem_set_slot(n, 7, ((contract.matches_left_out() as i8) > 1) as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// Main contract of a person through CONTRACT_MANAGER::get_contract (`exe::contract_manager_get_contract`).
pub(crate) unsafe fn main_contract_via_manager(p: usize) -> usize {
    exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls Shortlist_Manager::staff_interest_in_joining_club → float interest of the person in joining the club.
pub(crate) unsafe fn staff_interest_in_club(list: usize, person: usize, club: Club, salary: u32, mode: u32, status: u32) -> f64 {
    exe::shortlist_manager_staff_interest_in_joining_club(list, person, club, salary, mode, status, (0) as u32)
}

/// The comparer's dry run: the copy runs without the loan offer (`make_loan_offer`) or the list update (`update_staff_status`), which
/// allocate and touch the lists; it returns −2 where it would make the offer and −3 where it would update the list.
pub(crate) static LOAN_OFFER_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// True when 8-byte date a (day i16, year u16) is before date b, compared as the original does.
pub(crate) unsafe fn contract_manager_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_i16(a + cm_date::YEAR) as u16, read_i16(b + cm_date::YEAR) as u16);
    aa < ab || (aa == ab && read_i16(a) < read_i16(b))
}

/// Calls `nation_allows_free_transfers` (cdecl): whether the nation is active / allows free transfers.
pub(crate) unsafe fn contract_manager_nation_allows_free_transfers(nation: usize) -> bool {
    exe::nation_allows_free_transfers(nation) != 0
}

/// `CONTRACT_MANAGER::post_contract_expiry_news(contract, period, bosman, warn)` (SEH frame for the local news item and
/// list), called by the contracts pass: posts news item 0xbbd of an expiring contract to the club manager and, if
/// asked, to interested clubs of active nations. Without a contract: the "Error" box (line 0x24f0). Otherwise news item
/// 0xbbd to the club's manager (`exe::news_send_to_club_manager`): the club, the person, the period (signed), `bosman`
/// and, in slot 8, whether the person has a player sheet (`.player()`). With `warn` (cancelled if the club's nation,
/// `.nation()`, is not active, `contract_manager_nation_allows_free_transfers`): the clubs interested in the player
/// (`exe::shortlist_manager_collect_interested_clubs` into a local LLIST, with 1, 1, 0) from another active nation also
/// get it.
/// Original: `post_contract_expiry_news` (0x4d6790, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_contract_expiry_news(this: usize, contract: usize, period: u32, bosman: u32, warn: u32) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x24f0);
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbd, 0);
    let club = friendly_club_by_id(contract.club());
    let p = Staff((staff_records_staff_by_id(read_usize(contract.addr()))).addr());
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, period as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 7, bosman);
    exe::newsitem_set_slot(n, 8, (!p.player().is_null()) as u32);
    let nacao = club.nation().addr();
    let warn = if nacao != 0 && !contract_manager_nation_allows_free_transfers(nacao) { 0 } else { warn };
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
    if warn == 0 {
        return;
    }
    covered!("cpw_3 notify interested parties");
    let mut list = [0u8; 0xe];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    exe::shortlist_manager_collect_interested_clubs(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), p, l, 1, 1, 0);
    l.set_cursor(l.head());
    let mut x = exe::llist_get_next(l);
    while x != 0 {
        let id = read_usize(x + 4);
        if id != read_usize(club.addr()) {
            let nation = read_usize((friendly_club_by_id(id)).addr() + club::NATION);
            if nation != 0 && contract_manager_nation_allows_free_transfers(nation) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, friendly_club_by_id(read_usize(x + 4)));
            }
        }
        x = exe::llist_get_next(l);
    }
    exe::drop_llist(l);
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// `CONTRACT_MANAGER::post_bosman_departure_news(contract, target)` (SEH frame for the local news item): posts news
/// item 0xbbe of a Bosman departure. Without a contract: the "Error" box (line 0x2611). Otherwise news item 0xbbe to
/// the club's manager (`exe::news_send_to_club_manager`): the club, the person and the destination club (−1 without
/// one).
/// Original: `post_bosman_departure_news` (0x4d6d70, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_bosman_departure_news(this: usize, contract: usize, target: usize) {
    let contract = Contract(contract);
    let _ = this;
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2611);
    }
    covered!("cpw_3 departure under the Bosman ruling");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbbe, 0);
    let club = friendly_club_by_id(contract.club());
    let p = staff_records_staff_by_id(read_usize(contract.addr()));
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    set_news_staff_slots(n, p);
    exe::newsitem_set_slot(n, 6, if target != 0 { read_usize(target) as u32 } else { 0xffff_ffff });
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// Address of the person's contract offer counter (byte +8 of the 79-byte staff state record).
pub(crate) unsafe fn contract_offer_counter(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)).wrapping_add(8)
}

/// Assigns date origem to destino with the game's date operator (`exe::date_operator_3d`).
pub(crate) unsafe fn contract_manager_copy_date(target: usize, source: usize) {
    let mut t = [0u32; 2];
    exe::date_operator_3d(target, (&mut t) as *mut [u32; 2] as usize, source);
}

/// Finds the contract offer of person id from club in list l without moving the cursor; 0 if none.
pub(crate) unsafe fn find_contract_offer(l: Llist, id: usize, club: usize) -> usize {
    let mut in_the = l.head();
    let mut n = 0;
    while in_the != 0 && n < 100000 {
        let x = read_usize(in_the);
        if read_usize(x) == id && read_usize(x + 4) == club {
            return x;
        }
        in_the = read_usize(in_the + llist_node::NEXT);
        n += 1;
    }
    0
}

/// The comparer's dry run: the copy does not create or append to the list (`exe::crt_operator_new`, `exe::new_llist`, 0x68d0d0) and does not
/// call what comes after (`note_terms_offered_to_main_player`, `note_terms_offered_to_exchange_player`, `schedule_offer_decision`), which allocate, draw random numbers and change the offers.
pub(crate) static CONTRACT_OFFER_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// `CONTRACT_MANAGER::add_contract_offer(terms, slot, value, mark, a5, decides, agreed)` (SEH frame without a
/// destructor), called by the loan offer (`make_loan_offer`), the stored offers (`resubmit_held_offers`), the lists
/// (`make_club_bids`), the negotiation (`contract_manager_offer_contract`) and the screen: updates the same club's
/// offer to the person or adds a new one to the list; marks the terms as offered on the transfer offer and, if asked,
/// schedules the decision. Computes today + 14 (`exe::date_add_days`, discarded) and builds a local 0x6f-byte record
/// (contract `exe::new_staff_contract`, date `exe::date_date_54a110`). With counted offers (+8 of the person's 79-byte
/// record in [0xae2d0c], `contract_offer_counter`) and a list (`contract_manager_fields::CONTRACT_OFFERS`), the offer
/// from the same club is updated: the terms (the inlined operator= `contract_manager_operator`),
/// `contract_offer::STATE` = 2 if `agreed` (otherwise 0), `contract_offer::OFFER_DATE` = today,
/// `contract_offer::TRANSFER_OFFER_SLOT` the slot, `contract_offer::TRANSFER_OFFER_SERIAL` the value,
/// `contract_offer::MARK` the mark, `contract_offer::DECISION_VALID` = 1 outside a type 6 contract. Otherwise the local
/// record gets the same (`contract_offer::RESERVED_RECORD` = −1, the rest left as the original leaves it) and goes into
/// the list (created when missing; `exe::llist_add_tail`), and the offer count goes up. With a slot, the offer
/// (`exe::transfer_manager_get_offer_slot`) of the same player marks the terms as offered
/// (`exe::transfer_manager_note_terms_offered_to_main_player`), otherwise the exchange player
/// (`exe::transfer_manager_note_terms_offered_to_exchange_player`); with `decides`, it schedules the decision
/// (`exe::contract_manager_schedule_offer_decision`).
/// Original: `add_contract_offer` (0x4cc490, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_contract_offer(this: usize, terms: usize, slot: i32, value: u32, mark: u32, a5: u32, decides: u32, agreed: u32) {
    let terms = Contract(terms);
    let _ = a5;
    let mut quinzena = [0u32; 2];
    exe::date_add_days(G_CURRENT_DATE, (&mut quinzena) as *mut [u32; 2] as usize, 0xe);
    // simplification: bytes +0x67..+0x6e of the local record the original does not fill (stack leftover); the copy uses zero
    let mut record = [0u8; 0x6f];
    let r = record.as_mut_ptr() as usize;
    exe::new_staff_contract(r);
    exe::date_date_54a110(r + contract_offer::OFFER_DATE);
    let id = read_usize(terms.addr());
    let mut item = 0;
    if read_u8(contract_offer_counter(terms.addr())) != 0 && read_usize(this + contract_manager_fields::CONTRACT_OFFERS) != 0 {
        let club = terms.club();
        let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
        write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
        let mut x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        while x != 0 {
            if read_usize(x) == id && read_usize(x + 4) == club {
                item = x;
                break;
            }
            x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        }
    }
    let tipo6 = terms.contract_type() & 0x3f == 6;
    if item != 0 {
        covered!("cpw_4 offer updated");
        // simplification: the original inlines operator= (0x4cc810) here; the copy calls its Rust version, the same writes
        contract_manager_operator(item, terms.addr());
        write_u8(item + contract_offer::STATE, if agreed != 0 { 2 } else { 0 });
        contract_manager_copy_date(item + contract_offer::OFFER_DATE, G_CURRENT_DATE);
        write_u32(item + contract_offer::TRANSFER_OFFER_SLOT, slot as u32);
        write_u32(item + contract_offer::TRANSFER_OFFER_SERIAL, value);
        write_u8(item + contract_offer::MARK, mark as u8);
        write_u8(item + contract_offer::DECISION_VALID, !tipo6 as u8);
    } else {
        exe::staff_contract_operator_3d(r, terms);
        write_u8(r + contract_offer::STATE, 0);
        contract_manager_copy_date(r + contract_offer::OFFER_DATE, G_CURRENT_DATE);
        write_u32(r + contract_offer::TRANSFER_OFFER_SERIAL, value);
        write_u32(r + contract_offer::TRANSFER_OFFER_SLOT, slot as u32);
        write_u8(r + contract_offer::MARK, mark as u8);
        write_u32(r + contract_offer::RESERVED_RECORD, 0xffff_ffff);
        write_u8(r + contract_offer::DECISION_VALID, !tipo6 as u8);
        if CONTRACT_OFFER_DRY_RUN.load(Relaxed) {
            core::ptr::copy_nonoverlapping(r as *const u8, core::ptr::addr_of_mut!(DRY_RUN_CONTRACT_OFFER_RECORD) as *mut u8, 0x6f);
        } else {
            if read_usize(this + contract_manager_fields::CONTRACT_OFFERS) == 0 {
                let p = exe::crt_operator_new(0xe);
                let l = if p != 0 { exe::new_llist(p) } else { 0 };
                write_u32(this + contract_manager_fields::CONTRACT_OFFERS, l as u32);
            }
            (exe::llist_add_tail(read_usize(this + contract_manager_fields::CONTRACT_OFFERS), r, 0x6f) as u32);
        }
        covered!("cpw_4 new offer");
        let c = contract_offer_counter(terms.addr()) as *mut u8;
        *c = (*c).wrapping_add(1); // cpw4: the offer counter of the person
    }
    if CONTRACT_OFFER_DRY_RUN.load(Relaxed) {
        return;
    }
    if slot != -1 {
        let tm = read_usize(G_TRANSFER_MANAGER);
        let v = exe::transfer_manager_get_offer_slot(tm, slot);
        if read_usize(read_usize(v) + offer::TARGET_STAFF_ID) == read_usize(terms.addr()) {
            covered!("cpw_4 terms to the player");
            exe::transfer_manager_note_terms_offered_to_main_player(read_usize(G_TRANSFER_MANAGER), slot); // cpw4: the terms offered to the player
        } else {
            exe::transfer_manager_note_terms_offered_to_exchange_player(read_usize(G_TRANSFER_MANAGER), slot, read_usize(terms.addr()) as u32);
        }
    }
    if decides != 0 {
        covered!("cpw_4 schedule decision");
        exe::contract_manager_schedule_offer_decision(this, read_usize(terms.addr()) as u32, (slot) as i32 as u32); // cpw4: the scheduled decision
    }
}

/// O registro que a cópia em ensaio acrescentaria à lista.
pub(crate) static mut DRY_RUN_CONTRACT_OFFER_RECORD: [u8; 0x6f] = [0; 0x6f];

/// `CONTRACT_MANAGER::remove_contract_offer_for_transfer(person, slot)`, called by the withdrawn and undone offers
/// (`reject_transfer_offer`, `withdraw_transfer_offer`, `exe::transfer_manager_cancel_club_offers`,
/// `hold_offers_at_window_close`, 0x8d0f70, `exe::transfer_manager_cancel_offers_for_released_player`): removes the
/// person's contract offer linked to the transfer slot and lowers the offer count. In the list of contract offers
/// (`contract_manager_fields::CONTRACT_OFFERS`; cursor restarted), the person's one with the slot
/// (`contract_offer::TRANSFER_OFFER_SLOT`). When found: with `contract_offer::RESERVED_RECORD` flagged, the original
/// `exe::player_regen_add_spare_non_player_record` of `G_PLAYER_REGEN` with `contract_offer::RESERVED_RECORD`,
/// `contract_offer::RESERVED_RECORD_ARG1` and `contract_offer::RESERVED_RECORD_ARG2`; if agreed
/// (`contract_offer::STATE` = 2), it clears the agreed-contract flag
/// (`exe::transfer_manager_clear_agreed_contract_flag`); removes it from the list (`exe::llist_remove`) and the
/// person's offer count (+8 of the 79-byte record) goes down.
/// Original: `remove_contract_offer_for_transfer` (0x4ccb30, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_contract_offer_for_transfer(this: usize, person: usize, slot: i32) {
    let l = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    if l.is_null() {
        return;
    }
    l.set_cursor(l.head());
    let mut x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    while x != 0 {
        if read_usize(x) == read_usize(person) && read_usize(x + contract_offer::TRANSFER_OFFER_SLOT) as i32 == slot {
            break;
        }
        x = exe::llist_get_next(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    }
    if x == 0 {
        return;
    }
    covered!("cpw_4 offer removed by the slot");
    if read_usize(x + contract_offer::RESERVED_RECORD) as i32 != -1 {
        exe::player_regen_add_spare_non_player_record(read_usize(G_PLAYER_REGEN), (read_usize(x + contract_offer::RESERVED_RECORD) as u32) as u32 as usize, (read_usize(x + contract_offer::RESERVED_RECORD_ARG1) as u32) as u32 as usize, (read_usize(x + contract_offer::RESERVED_RECORD_ARG2) as u32) as u32 as usize);
    }
    if read_u8(x + contract_offer::STATE) == 2 {
        exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), (person) as usize as *const u32);
    }
    exe::llist_remove(read_usize(this + contract_manager_fields::CONTRACT_OFFERS), x);
    let c = contract_offer_counter(person) as *mut u8;
    *c = (*c).wrapping_sub(1);
}

/// The lowest wage the person asks of the club (stdcall, ret 8; SEH frame without a destructor), for the contract
/// screen: the lower of the desired wages for squad status 1 and 2 at the club, at most 10 million. In a local contract
/// (`exe::new_staff_contract`), the desired one (`exe::staff_contract_set_to_desired_contract` with −1, status 1 and
/// then 2, 0xb), at most 10 million.
/// Original: `contract_manager_lowest_wage_demand` (0x4ce450, contract_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn contract_manager_lowest_wage_demand(person: usize, club: usize) -> i32 {
    let mut local = [0u8; 0x50];
    let c = local.as_mut_ptr() as usize;
    exe::new_staff_contract(c);
    let mut smaller = 0x989680;
    let a = exe::staff_contract_set_to_desired_contract(c, person, club, -1, 1, 0xb, 0, 0);
    if a < smaller {
        smaller = a;
    }
    let b = exe::staff_contract_set_to_desired_contract(c, person, club, -1, 2, 0xb, 0, 0);
    if b < smaller { b } else { smaller }
}

/// `CONTRACT_MANAGER::handle_transfer_request(contract)` (SEH frame for the local news item): handles a player's
/// transfer request. A null contract: the "Error" box 0x2999. A person without a player sheet (`.player()`): nothing. A
/// human club (`exe::news_club_has_human_manager`): the request news item (0xbbf), built on the stack (by the
/// original). At a computer club, outside squad status 1 (`.squad_status()` & 0xf0 = 0x10): if the value
/// (`exe::contract_manager_calc_player_value`) × 1.5 beats the asking price
/// (`exe::contract_manager_calc_asking_price`), the club lists him (`contract::TRANSFER_STATUS`: clears bit 2 and sets
/// 8; morale +12 in `exe::staff_adjust_morale`; the club lists `exe::shortlist_manager_add_player_to_club_shortlists`;
/// `exe::contract_manager_post_transfer_listed_news`). Otherwise a draw by the contract's morale (`contract::MORALE`)
/// and the loyalty (`.loyalty()`) against another of 50 gives the 0x10000 grievance
/// (`exe::staff_contract_apply_event_flags`); the person's morale drops 12 (`exe::staff_adjust_morale`) and
/// `contract::MORALE` grows by loyalty × 5 + 56, held at ±100.
/// Original: `CONTRACT_MANAGER::handle_transfer_request` (0x4d8610, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn handle_transfer_request(this: usize, contract: usize) {
    let contract = Contract(contract);
    if contract.is_null() {
        return show_error_at(0x9a8190, 0x2999);
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add(contract.club().wrapping_mul(club::SIZE));
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(contract.addr()).wrapping_mul(staff::SIZE)));
    if person.player().is_null() {
        return;
    }
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
        // simplification: the human club receives the news item built on the stack (0x77bbe0 and the delivery are switched off): that
        // case stays with the original (which repeats the 0x7875e0 query, with no effect).
        // the record serves the proof of CMRTE’s ”Deixar infeliz” (make unhappy) (31/10/2026; deixar_infeliz.py): rare call
        linha(&format!("transfer request at the human club: person {}", read_usize(contract.addr())));
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contract.addr());
    }
    if contract.squad_status() & 0xf0 != 0x10 {
        let a = exe::contract_manager_calc_asking_price(this, club, 0, person, 0, 1, 1);
        let v = exe::contract_manager_calc_player_value(this, person, club, 0, -1, 0);
        // the 1.5 is the double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: the value exceeds the price
            covered!("cps_1 listed by club");
            let m = (contract.addr() + contract::TRANSFER_STATUS) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            exe::staff_adjust_morale(person, 0xc, 0);
            exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), person, 1);
            exe::contract_manager_post_transfer_listed_news(this, person, club);
            return;
        }
    }
    let r = exe::utils_random((contract.morale() as u8) as i8 as i32 + ((person.loyalty() as u8) as i8 as i32 + 5) * 20);
    if r < exe::utils_random(0x32) { // cps1: the complaint random draw
        covered!("cps_1 request complaint");
        exe::staff_contract_apply_event_flags(contract, 0x10000, (0) as u32);
    }
    covered!("cps_1 request at computer club");
    exe::staff_adjust_morale(person, 0xffff_fff4, 0); // cps1: a moral cai 12
    // reread after the calls, as in the original (the 8-bit `imul` keeps the low byte)
    let al = (person.loyalty() as u8).wrapping_mul(5).wrapping_add(0x38);
    let cl = contract.morale() as u8;
    let s = cl as i8 as i32 + al as i8 as i32;
    let m = (contract.addr() + contract::MORALE) as *mut u8;
    *m = if s > 100 {
        100
    } else if s < -100 {
        0x9c
    } else {
        cl.wrapping_add(al)
    };
}

/// 1 for jobs 0, 6 to 0xd and 0xf (those holding a contract manager contract)
/// Original: `is_contract_staff_job` (0x4cbb20, contract_manager.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn is_contract_staff_job(job: u32) -> u32 {
    matches!(job as u8, 0 | 6..=0xd | 0xf) as u32
}

/// 0 when the job (`.job_for_club()`) is a board job (1 to 4) at a club (`.club_job()`); otherwise 1
/// Original: `staff_is_not_board_member` (0x4cb960, contract_manager.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn staff_is_not_board_member(person: usize) -> u32 {
    let person = Staff(person);
    let job = person.job_for_club();
    if (1..=4).contains(&job) && !person.club_job().is_null() {
        return 0;
    }
    1
}

// simplification: draws (0x90cfc0), adds to the list (0x842c00, 0x4d71b0), changes morale and complaints (which may call the request for
// new): original only; the copy takes the branches.

/// Calls one of the game's date comparison functions f (>, <, ==) on date a with date b by value.
pub(crate) unsafe fn call_date_compare(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, read_u32(b), read_u32(b + 4))
}

/// `compare_decision_dates(a, b)` (cdecl): the qsort order of the decision dates (14-byte entries: +0 person, +4 date,
/// +0xc time slice of the day, +0xd type).
///
/// `a`'s date earlier (`exe::date_operator_3c`): −1; later (`exe::date_operator_3e`): 1; on the same day, `a`'s slice minus `b`'s (signed). Hot (the
/// qsort): no counter.
/// Original: `compare_decision_dates` (0x4dc910, contract_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_decision_dates(a: usize, b: usize) -> i32 {
    if call_date_compare(0x456770, a + decision_date::DATE, b + decision_date::DATE) != 0 {
        return -1; // a before b
    }
    if call_date_compare(0x456740, a + decision_date::DATE, b + decision_date::DATE) != 0 {
        return 1;
    }
    read_u8(a + decision_date::SLICE) as i8 as i32 - read_u8(b + decision_date::SLICE) as i8 as i32
}

/// consumes the decision dates due by the given date and slice: each one becomes −1 and, for type 3, the person decides
/// on the contract offers; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without running the copy) made the career
/// in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252): the detour changes the
/// stack residue a later original reads; since then the routine is hooked again
/// Original: `CONTRACT_MANAGER::process_due_decision_dates` (0x4cd9c0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn process_due_decision_dates(this: usize, data: usize, slice: u32) {
    let n = |c: usize| read_u32(c) as i32;
    if n(this + contract_manager_fields::DECISION_NEXT_INDEX) >= n(this + contract_manager_fields::DECISION_COUNT) {
        return;
    }
    loop {
        let e = (read_u32(this + contract_manager_fields::DECISION_DATES) as usize).wrapping_add((n(this + contract_manager_fields::DECISION_NEXT_INDEX) as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + decision_date::DATE, data) != 0 {
            return; // has not won yet
        }
        if call_date_compare(0x4a7f20, e + decision_date::DATE, data) != 0 && read_u8(e + decision_date::SLICE) as i8 > slice as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let person = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            write_i32(e, -1);
            if read_u8(e + decision_date::KIND) == 3 {
                covered!("cpp_1 decide offers");
                exe::contract_manager_resolve_contract_offers(this, person); // the person decides the offers
            }
        }
        let i = n(this + contract_manager_fields::DECISION_NEXT_INDEX) + 1;
        write_i32(this + contract_manager_fields::DECISION_NEXT_INDEX, i);
        if i >= n(this + contract_manager_fields::DECISION_COUNT) {
            return;
        }
    }
}

/// `CONTRACT_MANAGER::get_current_contract_offers(club, out)`: copies the club's open contract offers into a new array
/// of 111 bytes per offer and returns how many. It walks the offers list (`contract_manager_fields::CONTRACT_OFFERS`;
/// cursor reset, `exe::llist_get_next`) and copies each offer of the club (`contract_offer::CLUB_ID` = id) outside
/// states 5 and 6 (`contract_offer::CONTRACT_TYPE` & 0x3f) into a new vector of 111 bytes per offer, which grows by 20
/// at a time (realloc `exe::crt_realloc` with the capacity as a signed byte). Stores the vector in `out` and returns
/// how many (byte). After the realloc the original checks the list (`contract_manager_fields::CONTRACT_OFFERS`), not
/// the vector: only with a null list does it go to the "cannot allocate memory" box (line 0x64c) and return 0 with a
/// null output. The caller (0x598a06, the transfer budget) frees the vector.
/// Original: `CONTRACT_MANAGER::get_current_contract_offers` (0x4ca300, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_current_contract_offers(this: usize, club: usize, out: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut array: usize = 0;
    let list = Llist(read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize);
    if !list.is_null() {
        list.set_cursor(((list.head() as u32)) as usize);
        let mut item = exe::llist_get_next(list);
        while item != 0 {
            let state = read_u8(item + contract_offer::CONTRACT_TYPE) & 0x3f;
            if read_u32(item + contract_offer::CLUB_ID) == read_u32(club) && state != 5 && state != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    array = exe::crt_realloc(array, (cap as i32 * 111) as u32);
                    if read_u32(this + contract_manager_fields::CONTRACT_OFFERS) == 0 {
                        show_cannot_create_space_box(0x64c, 0x9a8220); // "cannot allocate memory"
                        write_u32(out, 0);
                        return 0;
                    }
                }
                covered!("cpp_1 club offer");
                core::ptr::copy(item as *const u8, array.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = exe::llist_get_next(read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize);
        }
    }
    write_u32(out, array as u32);
    n as u8
}

/// Volatile byte read (so a missing main contract crashes like the original).
pub(crate) unsafe fn read_byte_volatile(a: usize) -> u8 {
    // simplification: volatile read because the original reads [0 + 0x35] when the main contract is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}

/// Address of the person's 79-byte staff state record (+0 main contract index, +4 loan contract index; -1 none).
pub(crate) unsafe fn staff_state_entry(person: Staff) -> usize {
    (read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(staff_state::SIZE))
}

/// Entry k (0x50 bytes) of the contract array starting at vetor.
pub(crate) fn contract_at_index(array: u32, k: i32) -> usize {
    (array as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// Club record (0x245 bytes) of id in G_CLUB_TABLE.
pub(crate) unsafe fn contract_manager_club_by_id(id: u32) -> Club {
    Club((read_u32(G_CLUB_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(club::SIZE)))
}

/// Main contract of the person in the global CONTRACT_MANAGER (0 if none/out of range), also stored in the lookup
/// global 0xae1080.
pub(crate) unsafe fn main_contract_lookup(person: Staff) -> usize {
    let c = if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32(staff_state_entry(person)) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    write_u32(G_CONTRACT_LOOKUP_RESULT, c as u32);
    c
}

/// Shows the contract_manager.cpp "Error" message box for source line `linha`, as the original routine does; the game
/// continues.
///
/// The text is "v%s %s.%s %d" at 0xae24d0 (`exe::crt_strtok` splits the path 0x9a8190, 0x90d130 the version; the original does
/// not pop its arguments, which are the rest of the format), then the box `exe::gui_show_message_box` and [0xb67a34] = 0.
pub(crate) unsafe fn show_contract_error_box(row: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

// The database routines all three call (by address; the ported ones go through the hook).
/// Calls the club's linked A/B team lookup (`exe::club_get_linked_b_team`); sets whether the club is the A team.
pub(crate) unsafe fn contract_manager_linked_b_team(club: usize, eh_a: *mut u32) -> usize {
    exe::club_get_linked_b_team(club, eh_a, 1)
}
/// Calls the exe's Staff set-squad-flags routine for the person.
pub(crate) unsafe fn set_squad_flags(person: Staff, squad: u32) {
    exe::staff_set_squad_flags(person, squad);
}
/// Calls Tactics remove_player_from_selection on G_TACTICS for the club and person.
pub(crate) unsafe fn remove_from_tactics_selection(club: usize, person: Staff, a: u32, b: u32) {
    exe::tactics_remove_player_from_selection(read_u32(G_TACTICS) as usize, club, person, a, b);
}
/// Calls PlayerStats archive_player_stats on G_PLAYER_STATS for the person and club.
pub(crate) unsafe fn contract_manager_archive_player_stats(person: Staff, a4: u32, club: i32) {
    exe::playerstats_archive_player_stats(read_u32(G_PLAYER_STATS) as usize, person, -1, 0, a4, club, 0);
}
/// Calls Database remove_staff_from_club (club, person, date, flags).
pub(crate) unsafe fn contract_manager_remove_staff_from_club(club: usize, person: Staff, data: usize, disable: u32, how_player: u32) {
    exe::database_remove_staff_from_club(club, person, data, disable, how_player);
}
/// Calls Database add_staff_to_club (club, person, job, date, flags).
pub(crate) unsafe fn add_staff_to_club(club: usize, person: Staff, job: u32, data: usize, reduzir: u32, write: u32) {
    exe::database_add_staff_to_club(club, person, job, data, reduzir, write);
}
/// Calls Database club_has_reserve_squad for the club.
pub(crate) unsafe fn club_has_reserve_squad(club: usize) -> u32 {
    exe::database_club_has_reserve_squad(club)
}
/// Recomputes the club's wage bill (Finance::update_wage_bill on its entry in G_FINANCE_TABLE).
pub(crate) unsafe fn update_club_wage_bill(club: usize) {
    exe::finance_update_wage_bill(exe::financetable_get_club_finance(read_u32(G_FINANCE_TABLE) as usize, club));
}
/// Assigns date origem to destino with the date operator (`exe::date_operator_3d`).
pub(crate) unsafe fn contract_manager_assign_date(target: usize, source: usize) {
    let mut ret = [0u8; 8];
    exe::date_operator_3d(target, (&mut ret) as *mut [u8; 8] as usize, source);
}

/// `CONTRACT_MANAGER::sign_loan_contract(contract)` (SEH frame with only a local date, no destructor): moves the person
/// to the loan club, marks the reserve bit in the main contract and stores the loan contract in her slot; recalculates
/// the wage bills. The person (+0) of the new loan contract and her club by the main contract (the record
/// `G_CONTRACT_LOOKUP_RESULT`); without it, nothing. Keeps the person's date `staff::DATE_JOINED_CLUB`, takes her off
/// the lists (`contract_manager_archive_player_stats` with the new club `.club()`), checks the main contract in the
/// array itself (without it, the box of line 0x16a9 and nothing more) and sets in it bit 0x40 of
/// `contract::CONTRACT_TYPE` (in the reserves): with a reserve team (`club_has_reserve_squad`), by the squad mark
/// `staff::PLAYING_SQUAD` & 1; with a partner club (`contract_manager_linked_b_team`), if the current club
/// (`.club_job()`) is not the owner; otherwise clears it. Notifies the new club
/// (`exe::virtual_staff_remove_virtual_players`), takes the person out of the current club's tactics
/// (`remove_from_tactics_selection` with 0 and 1), puts her into the new club as a player (`add_staff_to_club`, job 0xb
/// with today's date) and restores the date `staff::DATE_JOINED_CLUB`. The contract goes into the person's loan slot
/// (+4 of the 79-byte record): the one she already has; otherwise the first free one (−1) of the array, which takes the
/// index (and then the original leaves without redoing the wage bills); otherwise at the end (full: +100 slots, realloc
/// `exe::crt_realloc` or malloc `exe::crt_malloc`; without memory, the box of line 0x1708 and it goes on), through
/// STAFF_CONTRACT::operator= `contract_manager_operator`. Finally it redoes the wage bills of the owning club and the
/// new one.
/// Original: `CONTRACT_MANAGER::sign_loan_contract` (0x4d01e0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sign_loan_contract(this: usize, contract: usize) {
    let contract = Contract(contract);
    let mut stored = [0u8; 8];
    exe::date_date_54a110((&mut stored) as *mut [u8; 8] as usize);
    let person = Staff((read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contract.addr()) as usize).wrapping_mul(staff::SIZE)));
    let sheet = Contract(if person.is_null() { write_u32(G_CONTRACT_LOOKUP_RESULT, 0); 0 } else { main_contract_lookup(person) });
    if sheet.is_null() {
        return;
    }
    let owner = contract_manager_club_by_id(sheet.club() as u32);
    if owner.is_null() {
        return;
    }
    contract_manager_assign_date(stored.as_mut_ptr() as usize, person.addr() + staff::DATE_JOINED_CLUB);
    let new = contract_manager_club_by_id(contract.club() as u32);
    contract_manager_archive_player_stats(person, 1, read_u32(new.addr()) as i32);
    let main = Contract(if !person.is_null() && (read_u32(person.addr()) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    });
    if main.is_null() {
        show_contract_error_box(0x16a9);
        return;
    }
    covered!("cpp_2 sign loan");
    let m = main.addr() + contract::CONTRACT_TYPE;
    let reserve = if club_has_reserve_squad(owner.addr()) != 0 {
        read_byte_volatile(person.addr() + staff::PLAYING_SQUAD) & 1 == 0
    } else if contract_manager_linked_b_team(owner.addr(), core::ptr::null_mut()) != 0 {
        owner.addr() as u32 != (person.club_job().addr() as u32)
    } else {
        false
    };
    let cl = read_byte_volatile(m);
    write_u8(m, if reserve { cl | 0x40 } else { cl & 0xbf });
    exe::virtual_staff_remove_virtual_players(read_u32(G_VIRTUAL_STAFF) as usize, new, 0);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 0, 1);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 1, 1);
    add_staff_to_club(new.addr(), person, 0xb, G_CURRENT_DATE, 1, 0); // the player at the new club
    contract_manager_assign_date(person.addr() + staff::DATE_JOINED_CLUB, stored.as_ptr() as usize);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp > -1 {
        // simplification: the original copies inline, same as STAFF_CONTRACT::operator= (0x4cc810, ported)
        contract_manager_operator(contract_at_index(read_u32(this), emp), contract.addr());
    } else {
        let n = read_u32(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let array = read_u32(this);
        let mut i = 0i32;
        while i < n {
            if read_u32(contract_at_index(array, i)) as i32 == -1 {
                contract_manager_operator(contract_at_index(array, i), contract.addr());
                write_u32(staff_state_entry(person) + 4, i as u32);
                return; // the original exits without rebuilding the sheets
            }
            i += 1;
        }
        let cap = read_u32(this + contract_manager_fields::CONTRACT_CAPACITY) as i32;
        if n >= cap {
            let cap = cap + 100;
            write_u32(this + contract_manager_fields::CONTRACT_CAPACITY, cap as u32);
            let new_array = if array != 0 {
                exe::crt_realloc(array as usize, (cap * 0x50) as u32)
            } else {
                exe::crt_malloc((cap * 0x50) as u32)
            };
            write_u32(this, new_array as u32);
            if new_array == 0 {
                show_contract_error_box(0x1708);
            }
        }
        let n = read_u32(this + contract_manager_fields::CONTRACT_COUNT);
        write_u32(staff_state_entry(person) + 4, n);
        let target = contract_at_index(read_u32(this), n as i32);
        write_u32(this + contract_manager_fields::CONTRACT_COUNT, n + 1);
        exe::staff_contract_operator_3d(target, contract);
    }
    update_club_wage_bill(owner.addr());
    update_club_wage_bill(new.addr());
}

/// `CONTRACT_MANAGER::end_loan(person)`: takes the person out of the loan club, frees the loan contract, recalculates
/// the wage bills and returns her to the parent club with the squad mark. The loan club (`.club_job()`; null: the box
/// of line 0x1729), or its A team when it is the B team (`contract_manager_linked_b_team`). With the main contract in
/// the array itself (without it: the box of line 0x1735): the record `G_CONTRACT_LOOKUP_RESULT` gives the owning club;
/// takes the person off the lists (`contract_manager_archive_player_stats` with the owner), the disputes
/// (`exe::discipline_restore_held_bans`), the loan club's tactics (`remove_from_tactics_selection` with 0 and 1); frees
/// the loan contract (−1 in the entry and in slot +4); removes her from the loan club's squad
/// (`contract_manager_remove_staff_from_club`); redoes the wage bills of the loan club and the owner; removes her from
/// the loan club's shortlist (`exe::shortlist_manager_find_club_shortlist_entry`,
/// `exe::shortlist_manager_update_staff_status` with 4). The person goes back to the owner (`.club_job()`) and to the
/// squad marks (`set_squad_flags`): with a reserve team (`club_has_reserve_squad`), 2 if the main contract has bit 0x40
/// of `contract::CONTRACT_TYPE`, otherwise 1; otherwise 1 and, with a partner club, she stays at the partner if she is
/// in its 50 slots. Status `staff::SQUAD_SELECTED_FOR` 0x10 becomes 4 (`euro_registration::PENDING_REREGISTRATION`).
/// Original: `CONTRACT_MANAGER::end_loan` (0x4d07e0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn end_loan(this: usize, person: usize) {
    let person = Staff(person);
    let mut club = (person.club_job().addr() as u32) as usize;
    if club == 0 {
        show_contract_error_box(0x1729);
        return;
    }
    let mut eh_a = 0u32; // read only when 0x540a50 writes it
    if contract_manager_linked_b_team(club, &mut eh_a) != 0 && eh_a == 0 {
        club = contract_manager_linked_b_team(club, core::ptr::null_mut()); // o time A
    }
    let main = Contract(if !person.is_null() && (read_u32(person.addr()) as i32) < read_u32(G_STAFF_COUNT) as i32 && (read_u32(staff_state_entry(person)) as i32) >= 0 {
        contract_at_index(read_u32(this), read_u32(staff_state_entry(person)) as i32)
    } else {
        0
    });
    if main.is_null() {
        show_contract_error_box(0x1735);
        return;
    }
    covered!("cpp_2 end loan");
    let sheet = Contract(contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), read_u32(staff_state_entry(person)) as i32));
    write_u32(G_CONTRACT_LOOKUP_RESULT, sheet.addr() as u32);
    let owner = if sheet.is_null() { 0 } else { (contract_manager_club_by_id(sheet.club() as u32)).addr() };
    contract_manager_archive_player_stats(person, 0, read_u32(owner) as i32);
    exe::discipline_restore_held_bans(read_u32(G_DISCIPLINE) as usize, person);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 0, 1);
    remove_from_tactics_selection((person.club_job().addr() as u32) as usize, person, 1, 1);
    let emp = read_u32(staff_state_entry(person) + 4) as i32;
    if emp != -1 {
        write_u32(contract_at_index(read_u32(this), emp), u32::MAX);
        write_u32(staff_state_entry(person) + 4, u32::MAX);
    }
    contract_manager_remove_staff_from_club((person.club_job().addr() as u32) as usize, person, 0, 1, 0);
    update_club_wage_bill(club);
    let f = main_contract_lookup(person);
    update_club_wage_bill(if f == 0 { 0 } else { (contract_manager_club_by_id(read_u32(f + contract::CLUB))).addr() });
    let list = read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize;
    if exe::shortlist_manager_find_club_shortlist_entry(list, club, person, 0, 0) != 0 {
        (exe::shortlist_manager_update_staff_status(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize, person, club, 4, 0) as usize);
    }
    let f = main_contract_lookup(person);
    let volta = if f == 0 { 0 } else { (contract_manager_club_by_id(read_u32(f + contract::CLUB))).addr() };
    person.set_club_job(Club((volta as u32) as usize)); // the person returns to the owner
    if club_has_reserve_squad(volta) != 0 {
        set_squad_flags(person, if read_byte_volatile(main.addr() + contract::CONTRACT_TYPE) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let partner = Club(contract_manager_linked_b_team((person.club_job().addr() as u32) as usize, core::ptr::null_mut()));
        set_squad_flags(person, 1);
        if !partner.is_null() {
            for k in 0..0x32 {
                if (partner.squad(k) as u32) as usize == person.addr() {
                    person.set_club_job(Club((partner.addr() as u32) as usize));
                }
            }
        }
    }
    if read_byte_volatile(person.addr() + staff::SQUAD_SELECTED_FOR) == 0x10 {
        person.set_squad_selected_for(euro_registration::PENDING_REREGISTRATION);
    }
}

/// `CONTRACT_MANAGER::toggle_squad_placement(person, club, out)`: moves the person between the club's first team and
/// reserves or between the club and its partner (A/B side), taking the loan contract into account; 0 with no room at
/// the destination. By the place (`exe::contract_manager_get_squad_placement`): 0 nothing. 1 and 2 (first team /
/// reserves at the club): on loan to another club (loan contract `contract::CLUB` with another club), it only clears
/// (1) or sets (2) bit 0x40 of `contract::CONTRACT_TYPE` of the main contract; otherwise squad mark 1 or 2
/// (`set_squad_flags`) and the club's tactics (`remove_from_tactics_selection` with 1 or 0). 3 (B team) and the others
/// (A team): the destination club is the club (3) or the partner (`contract_manager_linked_b_team`), stored in `out`;
/// with no room for the job (`staff::JOB_FOR_CLUB`, `exe::database_club_has_room_for_job`) it returns 0. On loan to
/// this club: she leaves the other's lists, squad and tactics and enters the destination with the job; with another
/// club's loan contract: toggles bit 0x40 and moves to the destination as a player (0xb); otherwise as the loaned one,
/// without storing. Returns 1 (also for a null club). Without a main contract the original reads [0x35] and crashes.
/// Original: `CONTRACT_MANAGER::toggle_squad_placement` (0x4cbd20, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn toggle_squad_placement(this: usize, person: usize, club: usize, out: usize) -> u32 {
    let person = Staff(person);
    if club == 0 {
        return 1;
    }
    if out != 0 {
        write_u32(out, 0);
    }
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    if l == 0 {
        return 1;
    }
    let in_band = |p: usize| p != 0 && (read_u32(p) as i32) < read_u32(G_STAFF_COUNT) as i32;
    // the loan contract (in the global CONTRACT_MANAGER) and the main one (in the array itself; 0 if none)
    let loan = |p: usize| {
        let k = read_u32(staff_state_entry(Staff(p)) + 4) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(read_u32(G_CONTRACT_MANAGER) as usize), k) }
    };
    let main = |p: usize| {
        let k = read_u32(staff_state_entry(Staff(p))) as i32;
        if k < 0 { 0 } else { contract_at_index(read_u32(this), k) }
    };
    if l == 1 || l == 2 {
        if in_band(person.addr()) {
            let e = loan(person.addr());
            if e != 0 && read_u32(e + contract::CLUB) != read_u32(club) {
                let m = main(person.addr()) + contract::CONTRACT_TYPE;
                let cl = read_byte_volatile(m);
                write_u8(m, if l == 1 { cl & 0xbf } else { cl | 0x40 });
                return 1;
            }
        }
        covered!("cpp_2 change at the club");
        set_squad_flags(person, l as u32); // 1 principal, 2 reserva
        remove_from_tactics_selection(club, person, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (contract_manager_linked_b_team(club, core::ptr::null_mut()), club)
    } else {
        (club, contract_manager_linked_b_team(club, core::ptr::null_mut()))
    };
    if out != 0 {
        write_u32(out, destino as u32);
    }
    // simplification: the original pushes the job in a register with garbage in the high bytes; 0x53b9a0 reads only the byte
    if exe::database_club_has_room_for_job(destino, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, person) == 0 {
        return 0;
    }
    if in_band(person.addr()) {
        let e = loan(person.addr());
        if e != 0 && read_u32(e + contract::CLUB) == read_u32(club) {
            // loaned to this club: passes from the other one to the destination with the role
            contract_manager_archive_player_stats(person, 0, -1);
            contract_manager_remove_staff_from_club(origem, person, 0, 1, 1);
            remove_from_tactics_selection(origem, person, 1, 1);
            remove_from_tactics_selection(origem, person, 0, 1);
            // simplification: the job goes with the high bytes of eax from 0x896c60; 0x53bd10 reads only the byte
            add_staff_to_club(destino, person, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, 0, 1, 1);
            return 1;
        }
        if in_band(person.addr()) && e != 0 {
            let m = main(person.addr()) + contract::CONTRACT_TYPE;
            let cl = read_byte_volatile(m);
            write_u8(m, if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf });
            covered!("cpp_2 change loaned");
            contract_manager_remove_staff_from_club(origem, person, 0, 0, 1);
            add_staff_to_club(destino, person, 0xb, 0, 0, 1);
            return 1;
        }
    }
    covered!("cpp_2 change team");
    contract_manager_archive_player_stats(person, 0, -1);
    contract_manager_remove_staff_from_club(origem, person, 0, 1, 0);
    remove_from_tactics_selection(origem, person, 1, 1);
    remove_from_tactics_selection(origem, person, 0, 1);
    // simplification: the role goes with the high bytes of edx left by 0x896c60; 0x53bd10 reads only the byte
    add_staff_to_club(destino, person, read_byte_volatile(person.addr() + staff::JOB_FOR_CLUB) as u32, 0, 1, 0);
    1
}

/// Volatile byte read (so a missing contract club crashes like the original).
pub(crate) unsafe fn read_u8_volatile(a: usize) -> u8 {
    // simplification: volatile because the original reads [0 + 0x53] when the contract’s club is missing (crashes like it)
    core::ptr::read_volatile(a as *const u8)
}

/// Club reputation as seen by a manager: weighted by the nation's league standard (or × 0.65 without nation), truncated
/// and capped above 0x203a.
pub(crate) unsafe fn perceived_club_reputation(club: Club) -> i16 {
    let r = club.reputation() as i32;
    let nation = Nation((club.nation().addr() as u32) as usize);
    let v = if !nation.is_null() {
        ftol((read_u8_volatile(nation.addr() + nation::LEAGUE_STANDARD) as i8 as i32 * r) as f64 * read_f64(rdata::F64_0_05))
    } else {
        ftol(r as f64 * read_f64(rdata::F64_0_65))
    } as i16;
    if v > 0x203a {
        (0x203a - ftol((v as i32 - 0x203a) as f64 * read_f64(rdata::F64_NEG_1_1))) as i16
    } else {
        v
    }
}

/// `staff_can_take_job(job, person)` (stdcall, ret 8): tells whether the person fits the job (6 to 15) by age, current
/// job and the player and non-player records; a null person: 0. Through the jump table 0x4cbaf4 (job − 6, 0..9;
/// outside: 0); 7, 12 and 14: 0; a null person: 0 (the non-player regen 0x7ab9f2 always asks with null). 6: aged 30
/// (`staff::AGE`) or more, a non-player (`.non_player()`), above 36, or with `staff::AMBITION` below 10 and the
/// player's `player_sheet::LEADERSHIP` 12 or more: 1; otherwise, a non-player in job 8, 6 or 5. 8: aged 30, a
/// non-player or above 36 or (`player_sheet::LEADERSHIP` ≥ 12 and `staff::AMBITION` < 10) outside jobs 5 and 10: 1;
/// otherwise as for 6. 9: a non-player in job 9. 10: a non-player in job 10 or with `staff::JOB_FOR_NATION` = 10. 11: a
/// player (`.player()`). 13 and 15: both player and non-player, or above 34, or aged 30 with `player_sheet::LEADERSHIP`
/// ≥ 12.
/// Original: `staff_can_take_job` (0x4cb990, contract_manager.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn staff_can_take_job(job: u32, person: usize) -> u32 {
    let k = (job as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if person == 0 {
        covered!("cpp_4 null person");
        return 0; // null person
    }
    let p = Staff(person);
    let age = read_u8_volatile(p.addr() + staff::AGE) as i8;
    let not_player = p.non_player().addr() as u32;
    let function = read_u8_volatile(p.addr() + staff::JOB_FOR_CLUB);
    let strength = || read_u8_volatile((p.player().addr() as u32) as usize + player_sheet::LEADERSHIP) as i8; // +0x2f of the player
    // the common end (0x4cb9db): non-player in jobs 8, 6 or 5
    let common = || not_player != 0 && (function == 8 || function == 6 || function == 5);
    let r = match k {
        0 => {
            if age >= 0x1e {
                if not_player != 0 || age > 0x24 {
                    return 1;
                }
                if (read_u8_volatile(p.addr() + staff::AMBITION) as i8) < 0xa && strength() >= 0xc {
                    return 1;
                }
            }
            common()
        }
        2 => {
            if age < 0x1e {
                common()
            } else if not_player == 0 && age <= 0x24 && (strength() < 0xc || read_u8_volatile(p.addr() + staff::AMBITION) as i8 >= 0xa) {
                common()
            } else if function == 5 || function == 0xa {
                common()
            } else {
                true
            }
        }
        3 => not_player != 0 && function == 9,
        4 => not_player != 0 && (function == 0xa || read_u8_volatile(p.addr() + staff::JOB_FOR_NATION) == 0xa),
        5 => (p.player().addr() as u32) != 0,
        _ => {
            // 13 e 15
            let player = PlayerSheet((p.player().addr() as u32) as usize);
            !player.is_null() && (not_player != 0 || age > 0x22 || (age >= 0x1e && read_u8_volatile(player.addr() + player_sheet::LEADERSHIP) as i8 >= 0xc))
        }
    };
    r as u32
}

/// Days (0..400) from the date (day, unsigned year) to today (G_CURRENT_DATE), 365-day years.
pub(crate) unsafe fn contract_manager_days_since_date(day: usize, year: usize) -> i32 {
    let today = read_u16(G_CURRENT_DATE_YEAR) as i32;
    let a = read_u16(year) as i32;
    let d = (today - a) * 365 - read_i16(day) as i32 + read_i16(G_CURRENT_DATE) as i32;
    d.clamp(0, 400)
}

/// The morale drop (the original's byte `m`) after a bad match, or None when the routine exits without touching
/// anything (some null, the club's manager or assistant +0xcf/+0xd3, or outside job 0xb).
///
/// x = (rating +0xb − club reputation) + 50 × days at the club (since +0x3e, up to 400) + 417 × age − 31680. With the
/// contract days (since +0x2d) above 400 (never: the cap is 400) and age 24..31: + 250 × +0x57 + 5000; otherwise, with
/// +0x22 and the nation's reputation (+0x1a +0x8e) above 3500 and under 32 years old: + 250 × +0x57 + 25 × +0x22 +
/// 2500; otherwise by the contract band (+0x4f >> 4): 1: + 250 × +0x57 + 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 ×
/// +0x3e; in these cases at most 15000.
///
/// y = 1 − x × (6.667e-4 if x < 0, otherwise 6e-5), between 0.1 and 10; m = ftol((+0x3e + 4) / y + 0.5) (byte). Double
/// precision; the constants come from .rdata.
pub(crate) unsafe fn match_morale_drop(person: Staff, club: Club, contract: Contract) -> Option<i8> {
    if contract.is_null() || club.is_null() || person.is_null() {
        return None;
    }
    if person.addr() == (club.manager().addr() as u32) as usize || person.addr() == (club.assistant_manager() as u32) as usize || read_u8_volatile(person.addr() + staff::JOB_FOR_CLUB) != 0xb {
        return None;
    }
    let a = contract_manager_days_since_date(person.addr() + staff::DATE_JOINED_CLUB, person.addr() + staff::DATE_JOINED_CLUB_YEAR);
    let b = contract_manager_days_since_date(contract.addr() + contract::CONTRACT_EXPIRES, contract.addr() + contract::CONTRACT_EXPIRES_YEAR);
    let age = read_u8_volatile(person.addr() + staff::AGE) as i8;
    let player = PlayerSheet((person.player().addr() as u32) as usize);
    let dif = player.current_reputation() as i32 - club.reputation() as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + age as f64 * read_f64(rdata::F64_417)) - read_f64(rdata::F64_31680);
    let c57 = read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32;
    let ceiling = read_f32(rdata::F32_15000) as f64;
    let limitar = |x: f64| if x >= ceiling { ceiling } else { x };
    let al = read_u8_volatile(person.addr() + staff::INT_APPS);
    if b > 0x190 && age > 0x17 && age < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + read_f32(rdata::F32_5000) as f64); // never: b goes up to 400;
    } else if al != 0 && read_i16((person.nation().addr() as u32) as usize + nation::REPUTATION) > 0xdac && age < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + read_f32(rdata::F32_2500) as f64);
    } else {
        match (read_u8_volatile(contract.addr() + contract::SQUAD_STATUS) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + read_f32(rdata::F32_5000) as f64),
            2 => x = limitar(x + (c57 * 125) as f64 + read_f32(rdata::F32_2500) as f64),
            3 => x = limitar(x + (read_u8_volatile(contract.addr() + contract::UNHAPPY_FLAG) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= read_f64(rdata::F64_0)) { x * read_f64(rdata::F64_0_0006666666667) } else { x * read_f64(rdata::F64_6ENEG_05) };
    let mut y = read_f64(rdata::F64_1) - y;
    if !(y >= read_f64(rdata::F64_0_1)) {
        y = read_f64(rdata::F64_0_1);
    } else if y > read_f64(rdata::F64_10) {
        y = read_f64(rdata::F64_10);
    }
    let d = (read_u8_volatile(contract.addr() + contract::UNHAPPY_FLAG) as i8 as i32 + 4) as f64;
    Some(ftol(d / y + read_f64(rdata::F64_0_5)) as u8 as i8)
}

/// Morale minus m, clamped to -100..100 (byte sum when it fits).
pub(crate) fn lowered_morale(morale: i8, m: i8) -> i8 {
    let minus = m.wrapping_neg();
    let s = morale as i32 + minus as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        morale.wrapping_add(minus)
    }
}

/// Club of the person's main contract (null if none); stores the contract in G_CONTRACT_LOOKUP_RESULT like the
/// original.
pub(crate) unsafe fn club_of_main_contract(person: Staff) -> Club {
    let c = Contract(if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(staff_state::SIZE))) as i32;
        if k < 0 { 0 } else { (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    });
    write_u32(G_CONTRACT_LOOKUP_RESULT, c.addr() as u32);
    if c.is_null() { Club(0) } else { Club((read_u32(G_CLUB_TABLE) as usize).wrapping_add(((c.club() as u32) as usize).wrapping_mul(club::SIZE))) }
}

/// Whether the transfer window is open for the club of the person's contract.
///
/// The key nation index (`0x669340` of the club's nation; with no nation, the low byte of the club pointer that the
/// original routine reads from the argument slot) gives the two dates of table `0xb63d8a` (72 bytes per nation): true
/// when today is after the first and before the second.
pub(crate) unsafe fn transfer_window_open(person: Staff, club: Club) -> bool {
    let c1 = Club((club_of_main_contract(person)).addr());
    let k = if (c1.nation().addr() as u32) != 0 {
        let c2 = Club((club_of_main_contract(person)).addr());
        exe::key_nation_index_of_nation((c2.nation().addr() as u32) as usize)
    } else {
        // simplification: the original reads the low byte of the slot of the `clube` argument (the pointer), not an index
        club.addr() as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = G_LEAGUE_WORLD_CUP_START_DATE + (k as usize) * 0x48;
    let day = read_i16(G_CURRENT_DATE);
    let year = read_u16(G_CURRENT_DATE_YEAR);
    let year_of = |a: usize| read_u16(a);
    let after = (day > read_i16(t) && year == year_of(t + 2)) || year > year_of(t + 2);
    let before = (day < read_i16(t + 0xf) && year == year_of(t + 0x11)) || year < year_of(t + 0x11);
    after && before
}

/// `contract_manager_lower_match_morale(person, club, contract)`: lowers the player's contract morale after a poor
/// match (by rating, reputation, time at the club, age and contract band) and, with very low morale, may add the
/// wants-to-leave or lack-of-games grievance; injured, on loan or retiring clears the grievances. The drop `m`
/// (match_morale_drop). Injured (`exe::injury_get_player_injury_status`), on loan (loan contract), retiring
/// (`exe::player_regen_get_retirement_status`), under 18 or with `contract::LEAVING_ON_BOSMAN`: clears grievances 4 and
/// 0x10 (`exe::staff_contract_clear_grievances`). Otherwise, with m > 5 and morale − m below −30 − `staff::LOYALTY`:
/// above 21 years old, stronger (`.current_reputation()`) than the club and above 2250, or with potential
/// (`.world_reputation()`) above reputation − 1500 and above 500 with draw(`staff::AMBITION`) ≠ 0 and
/// `contract::HAPPINESS` negative: in the transfer window of the contract club, grievance 4
/// (`exe::staff_contract_clear_grievances` and `exe::staff_contract_apply_event_flags` with 4); in the first team
/// (`exe::contract_manager_get_squad_placement` = 1), rating above 1250, more than 23 years old and
/// draw(`staff::LOYALTY` + 5) = 0: grievance 0x10; up to 21 years old, in the first team with draw(`staff::AMBITION`) >
/// 5, stronger than the club and above 1250: grievance 0x10. Finally the happiness in the contract (`contract::MORALE`)
/// drops by m, kept between −100 and 100.
/// Original: `contract_manager_lower_match_morale` (0x4cb250, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn contract_manager_lower_match_morale(this: usize, person: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let person = Staff(person);
    let club = Club(club);
    let Some(m) = match_morale_drop(person, club, contract) else {
        return;
    };
    let loaned = || {
        if read_u32(person.addr()) as i32 >= read_u32(G_STAFF_COUNT) as i32 {
            return false;
        }
        let k = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((read_u32(person.addr()) as usize).wrapping_mul(staff_state::SIZE)) + 4) as i32;
        k >= 0 && (read_u32(read_u32(G_CONTRACT_MANAGER) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let age = read_u8_volatile(person.addr() + staff::AGE) as i8;
    if exe::injury_get_player_injury_status(read_u32(G_INJURY) as usize, person) != 0 || loaned() || exe::player_regen_get_retirement_status(read_u32(G_PLAYER_REGEN) as usize, person) != 0
        || age < 0x12 || read_u8_volatile(contract.addr() + contract::LEAVING_ON_BOSMAN) != 0 {
        if contract.grievances() & 4 != 0 {
            exe::staff_contract_clear_grievances(contract, 4);
        }
        if read_u8_volatile(contract.addr() + contract::GRIEVANCES) & 0x10 != 0 {
            exe::staff_contract_clear_grievances(contract, 0x10);
        }
    } else if m > 5 && (read_u8_volatile(contract.addr() + contract::MORALE) as i8 as i32 - m as i32) < -30 - read_u8_volatile(person.addr() + staff::LOYALTY) as i8 as i32 {
        let player = PlayerSheet((person.player().addr() as u32) as usize);
        if age > 0x15 {
            let rating = player.current_reputation();
            if rating > club.reputation() && rating > 0x8ca {
                if transfer_window_open(person, club) {
                    covered!("cpp_4 complaint in the window");
                    exe::staff_contract_clear_grievances(contract, 4);
                    exe::staff_contract_apply_event_flags(contract, 4, 0);
                }
            } else {
                let mut done = false;
                if exe::contract_manager_get_squad_placement(this, person, club) == 1 && player.current_reputation() > 0x4e2 && age > 0x17
                    && random(read_u8_volatile(person.addr() + staff::LOYALTY) as i8 as i32 + 5) == 0 {
                    exe::staff_contract_clear_grievances(contract, 4);
                    exe::staff_contract_apply_event_flags(contract, 0x10, 0);
                    done = true;
                }
                if !done {
                    let pot = player.world_reputation();
                    if (pot as i32) > club.reputation() as i32 - 0x5dc && pot > 0x1f4
                        && random(read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32) != 0 && (read_u8_volatile(contract.addr() + contract::HAPPINESS) as i8) < 0
                        && transfer_window_open(person, club) {
                        exe::staff_contract_clear_grievances(contract, 4);
                        exe::staff_contract_apply_event_flags(contract, 4, 0);
                    }
                }
            }
        } else if exe::contract_manager_get_squad_placement(this, person, club) == 1 && random(read_u8_volatile(person.addr() + staff::AMBITION) as i8 as i32) > 5
            && player.current_reputation() > club.reputation() && player.current_reputation() > 0x4e2 {
            exe::staff_contract_clear_grievances(contract, 4);
            exe::staff_contract_apply_event_flags(contract, 0x10, 0);
        }
    }
    let p = (contract.addr() + contract::MORALE) as *mut i8;
    *p = lowered_morale(*p, m); // a moral cai
}

/// Address of the person's 79-byte staff state record (+0 main contract index, -1 none).
pub(crate) unsafe fn main_contract_slot(person: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE))
}

/// `CONTRACT_MANAGER::remove_contract(person)`: frees the person's main contract entry, puts −1 in her slot of the
/// 79-byte record and recalculates the wage bill of the contract's club. With the main contract (index in the person's
/// 79-byte record), it frees its entry (+0 = −1 in the CONTRACT_MANAGER's 0x50-byte vector), puts −1 in the person's
/// slot and redoes the wage bill (`exe::finance_update_wage_bill`) of the finances
/// (`exe::financetable_get_club_finance`) of the contract club (`.club()`). Without a contract: nothing.
/// Original: `CONTRACT_MANAGER::remove_contract` (0x4d0b50, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_contract(this: usize, person: usize) {
    let index = read_i32(main_contract_slot(person));
    if index == -1 {
        return;
    }
    let e = Contract(read_usize(this).wrapping_add((index as usize).wrapping_mul(0x50)));
    let id_club = e.club();
    write_usize(e.addr(), usize::MAX);
    let club = read_usize(G_CLUB_TABLE).wrapping_add(id_club.wrapping_mul(club::SIZE));
    covered!("cpo_2 remove contract");
    write_usize(main_contract_slot(person), usize::MAX); // the person is left without a contract
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if f != 0 {
        exe::finance_update_wage_bill(f);
    }
}

/// `CONTRACT_MANAGER::resubmit_held_offers(id)`: resubmits the person's held offers as live offers, removes each one
/// from the list and returns how many. It walks the held offers list (`contract_manager_fields::HELD_OFFERS`) and, for
/// each one of person `id` (+0): with the transfer (`contract_offer::TRANSFER_OFFER_SLOT`,
/// `exe::transfer_manager_get_offer_slot` over `G_TRANSFER_MANAGER`) whose serial (`.serial()`) is the one the offer
/// kept (`contract_offer::TRANSFER_OFFER_SERIAL`), it sets state 0xf (`offer_status::COLLAPSED`) to 8
/// (`offer_status::AWAITING_PLAYER_DECISION`) and resubmits the offer (`exe::contract_manager_add_contract_offer` with
/// the slot, the serial, byte `contract_offer::MARK`, 1, 1, 0); without a transfer
/// (`contract_offer::TRANSFER_OFFER_SLOT` = −1) it resubmits with −1, −1. It removes the offer from the list
/// (`exe::llist_remove`) and counts it. Byte `contract_offer::MARK` is pushed in the whole register (the upper bits of
/// whatever was in it: the pointer read or −1). Returns how many.
/// Original: `CONTRACT_MANAGER::resubmit_held_offers` (0x4cf4a0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn resubmit_held_offers(this: usize, id: usize) -> u32 {
    let list = Llist(read_usize(this + contract_manager_fields::HELD_OFFERS));
    if list.is_null() {
        return 0;
    }
    list.set_cursor(list.head());
    let mut n = 0u32;
    let mut item = exe::llist_get_next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    while item != 0 {
        if read_usize(item) == id {
            let t = read_usize(item + contract_offer::TRANSFER_OFFER_SLOT);
            let byte = read_u8(item + contract_offer::MARK) as u32;
            if t as i32 != -1 {
                let p = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), (t) as i32);
                if p != 0 {
                    let q = Offer(read_usize(p));
                    if !q.is_null() && q.serial() == read_usize(item + contract_offer::TRANSFER_OFFER_SERIAL) {
                        if q.status() == offer_status::COLLAPSED {
                            q.set_status(offer_status::AWAITING_PLAYER_DECISION);
                        }
                        covered!("cpo_2 resend offer");
                        exe::contract_manager_add_contract_offer(this, (item) as usize as *mut u8, t, read_usize(item + contract_offer::TRANSFER_OFFER_SERIAL), (q.addr() as u32 & 0xffff_ff00) | byte, 1, 1, 0); // with the transfer
                    }
                }
            } else {
                exe::contract_manager_add_contract_offer(this, (item) as usize as *mut u8, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            exe::llist_remove(read_usize(this + contract_manager_fields::HELD_OFFERS), item);
            n += 1;
        }
        item = exe::llist_get_next(read_usize(this + contract_manager_fields::HELD_OFFERS));
    }
    n
}

/// `contract_manager_season_end_month(club)` (stdcall, ret 4; SEH frame without a destructor on the normal path): the
/// (exe::date_get_month(0..11) as u32) of the date ten days after the end of the season of the club's nation.
///
/// The club's season end date (Date() `exe::date_date_54a110` and `key_nation_club_in_season` with 0), plus 10 days (`exe::date_add_days`), and its (exe::date_get_month(`exe::date_get_month`) as u32).
/// Returns the eax of `exe::date_get_month` (the month 0..11).
/// Original: `contract_manager_season_end_month` (0x4d45f0, contract_manager.cpp, stdcall, ret 4).
pub(crate) unsafe extern "stdcall" fn contract_manager_season_end_month(club: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    exe::date_date_54a110((d1.as_mut_ptr()) as usize);
    exe::key_nation_club_in_season(club, 0, (d1.as_mut_ptr()) as *mut u32 as usize);
    let p = exe::date_add_days((d1.as_mut_ptr()) as usize, (d2.as_mut_ptr()) as usize, (0xa) as u32); // dez dias depois
    exe::date_get_month(p) as u32 
}

/// Shows the CONTRACT_MANAGER "cannot create space" box for a line and text; the game continues.
///
/// Text "v%s %s.%s %d (%s)" (version 0x90d130, name and extension of contract_manager.cpp 0x9a8190, `linha`, `texto`)
/// at 0xae24d0; the title "Memory Allocation Error" (0x9a8108) translated (0x66f4e0) at 0xae28bc, the body 0x9a7ff8
/// with the text at 0xae26c4, and the box `exe::gui_show_message_box`. The game continues ([0xb67a34] stays as it is).
pub(crate) unsafe fn show_cannot_create_space_box(row: u32, text: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// `CONTRACT_MANAGER::get_free_staff_contract_space(so_in_end)`: returns the slot in the contract vector (0x50-byte
/// records at [this], capacity `contract_manager_fields::CONTRACT_CAPACITY`, count
/// `contract_manager_fields::CONTRACT_COUNT`). Without `so_in_end`, the first empty slot (person +0 = −1); otherwise
/// (or with no empty slot) the one at the end: when full it grows by 200 at a time (realloc `exe::crt_realloc`, or
/// malloc `exe::crt_malloc` without a vector; without memory, box 0x175 and −1) and the count goes up. Returns the
/// index. Both callers (the contract creation 0x4c7560 and `add_staff_contract`) pass 1.
/// Original: `CONTRACT_MANAGER::get_free_staff_contract_space` (0x4c79a0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_free_staff_contract_space(this: usize, so_in_end: u32) -> i32 {
    if so_in_end == 0 {
        let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT) as i32;
        let p = read_usize(this);
        let mut i = 0i32;
        while i < n {
            if read_usize(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                return i;
            }
            i += 1;
        }
    }
    if read_usize(this + contract_manager_fields::CONTRACT_COUNT) == read_usize(this + contract_manager_fields::CONTRACT_CAPACITY) {
        let cap = read_usize(this + contract_manager_fields::CONTRACT_CAPACITY).wrapping_add(200);
        let p = read_usize(this);
        write_usize(this + contract_manager_fields::CONTRACT_CAPACITY, cap);
        let q = if p != 0 {
            exe::crt_realloc(p, cap.wrapping_mul(0x50) as u32)
        } else {
            exe::crt_malloc(cap.wrapping_mul(0x50) as u32)
        };
        write_usize(this, q);
        if q == 0 {
            show_cannot_create_space_box(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = read_usize(this + contract_manager_fields::CONTRACT_COUNT).wrapping_add(1);
    write_usize(this + contract_manager_fields::CONTRACT_COUNT, n); // one more slot
    (n as i32).wrapping_sub(1)
}

/// `CONTRACT_MANAGER::add_staff_contract(person, value)`: takes the appended slot for a new person (hired manager,
/// regen), links the person to it and generates the initial contract. The slot at the end
/// (`exe::contract_manager_get_free_staff_contract_space` with 1); without a slot, nothing. Links the person to the
/// slot (index +0 of her 0x4f-byte record in `G_STAFF_STATE_TABLE`), puts her id in the contract (+0) and generates the
/// initial contract (`exe::staff_contract_generate_initial_contract` with the value). Called by the manager hiring
/// (0x68f9c0, MANAGER_MANAGER::daily_update) and by the regen that signs with a club (`player_regen_sign_for_club`).
/// Returns the original's eax (that of `exe::staff_contract_generate_initial_contract`, or the negative index); the
/// callers do not read it.
/// Original: `CONTRACT_MANAGER::add_staff_contract` (0x4c7950, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_staff_contract(this: usize, person: usize, value: u32) -> u32 {
    let i = exe::contract_manager_get_free_staff_contract_space(this, 1);
    if i < 0 {
        return i as u32;
    }
    covered!("cpl_4 person contract");
    write_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)), i as usize); // the person points to the slot
    let c = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50));
    write_usize(c, read_usize(person));
    exe::staff_contract_generate_initial_contract(c, person, value) // the person’s value
}

/// The reminder dates (type 2) that a type-1 decision date gets before it, from the contract morale; returns up to two
/// dates and the count.
///
/// With a player record (+0x61), L = contract morale (+0x44) + 100 (8 bits; the contract via the table [0xae2d0c], as
/// `exe::contract_manager_get_contract` inline: without a contract the original routine reads address 0x44 and crashes, as here); L ≤ 0x50 no
/// reminder; L > 0x96 one reminder L × 90 / 200 days before; and one L / 4 days before (without a player record, L =
/// 100).
///
/// The dates come from Date::sub_days (0x54c8e0), in the original's order.
pub(crate) unsafe fn decision_reminder_dates(this: usize, person: Staff, data: usize, kind: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if kind != 1 {
        return (v, k);
    }
    let mut l: u8 = 100;
    if !person.player().is_null() {
        let id = read_usize(person.addr()) as i32;
        let c = if person.is_null() || id >= read_usize(G_STAFF_COUNT) as i32 {
            0
        } else {
            let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE))) as i32;
            if i < 0 { 0 } else { read_usize(this).wrapping_add((i as usize).wrapping_mul(0x50)) }
        };
        l = read_u8(c + contract::MORALE).wrapping_add(100);
        if l <= 0x50 {
            return (v, k);
        }
        if l > 0x96 {
            let mut t = [0u8; 8];
            let r = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// `CONTRACT_MANAGER::add_decision_date(person, data, slice, kind)`: appends an entry to the 14-byte decision date
/// vector; type 1 first adds type 2 reminders from the contract morale. Appends to the 14-byte vector
/// `contract_manager_fields::DECISION_DATES` (capacity `contract_manager_fields::DECISION_CAPACITY`, count
/// `contract_manager_fields::DECISION_COUNT`; it grows by 100 at a time when the count passes capacity − 4, realloc
/// `exe::crt_realloc` or malloc `exe::crt_malloc`; without memory, box 0x103e and nothing) the entry (person id +0,
/// date `decision_date::DATE`, time slice of the day `decision_date::SLICE`, type `decision_date::KIND`). Type 1 is
/// preceded by the type 2 reminders (`decision_reminder_dates`), without a slice (`decision_date::SLICE` is left as it
/// is). Called by the offers: the person's decision (`schedule_offer_decision`,
/// `exe::contract_manager_resolve_contract_offers`). The date copies (`exe::date_operator_3d`, with no other effect)
/// become 8-byte copies.
/// Original: `CONTRACT_MANAGER::add_decision_date` (0x4cd6d0, contract_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn add_decision_date(this: usize, person: usize, data: usize, slice: u32, kind: u32) {
    covered!("cpl_4 decision date");
    if read_usize(this + contract_manager_fields::DECISION_COUNT) as i32 > (read_usize(this + contract_manager_fields::DECISION_CAPACITY) as i32).wrapping_sub(4) {
        let cap = read_usize(this + contract_manager_fields::DECISION_CAPACITY).wrapping_add(100);
        write_usize(this + contract_manager_fields::DECISION_CAPACITY, cap);
        let p = read_usize(this + contract_manager_fields::DECISION_DATES);
        let q = if p != 0 {
            exe::crt_realloc(p, cap.wrapping_mul(14) as u32)
        } else {
            exe::crt_malloc(cap.wrapping_mul(14) as u32)
        };
        write_usize(this + contract_manager_fields::DECISION_DATES, q);
        if q == 0 {
            show_cannot_create_space_box(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = decision_reminder_dates(this, Staff(person), data, kind as u8);
    for d in &lembretes[..k] {
        let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
        let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
        write_usize(e, read_usize(person));
        core::ptr::write_unaligned((e + decision_date::DATE) as *mut [u8; 8], *d);
        write_u8(e + decision_date::KIND, 2);
        write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1));
    }
    let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
    let e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_mul(14));
    write_usize(e, read_usize(person));
    write_u8(e + decision_date::SLICE, slice as u8);
    core::ptr::write_unaligned((e + decision_date::DATE) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    write_u8(e + decision_date::KIND, kind as u8);
    write_usize(this + contract_manager_fields::DECISION_COUNT, n.wrapping_add(1)); // the decision date
}

// simplification: all three reallocate (and 0x4c7950 generates the contract through 0x85e960); they do not run twice. The comparer builds
// the plan beforehand, runs only the original and checks; Rust mode and the career prove the copies.

/// The person's contract with the club whose relegation clause (+0x1f = 1) is still to trigger, else 0.
pub(crate) unsafe fn contract_with_relegation_clause(this: usize, club: usize, p: usize) -> usize {
    if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let i = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(staff_state::SIZE))) as i32;
    if i < 0 {
        return 0;
    }
    let c = Contract((i as usize).wrapping_mul(80).wrapping_add(read_usize(this)));
    if c.is_null() || c.club() != read_usize(club) || c.relegation_rc() != 1 {
        return 0;
    }
    c.addr()
}

/// The club and its linked B team (a B club or null gives none: the A club handles both).
pub(crate) unsafe fn club_and_b_team(club: usize) -> [usize; 2] {
    if club == 0 {
        return [0, 0];
    }
    let mut eh_a = 0u32;
    let p = exe::club_get_linked_b_team(club, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        return [0, 0];
    }
    [club, p]
}

/// `CONTRACT_MANAGER::trigger_relegation_clauses(club, league)`: triggers the relegation clauses at a relegated club.
/// At the relegated club and at its partner (A/B team), each of the 50 staff and squad members (`club::SQUAD`) with a
/// relegation clause still to be triggered in the contract with the club (`.relegation_rc()` = 1) goes to 2 and the
/// news is posted (`exe::transfer_manager_post_release_clause_activated_news` over `G_TRANSFER_MANAGER` with type 3).
/// Original: `CONTRACT_MANAGER::trigger_relegation_clauses` (0x4cd030, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn trigger_relegation_clauses(this: usize, club: usize, league: usize) {
    for c in club_and_b_team(club) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = read_usize(c + club::SQUAD + k * 4);
            let contract = Contract(contract_with_relegation_clause(this, c, p));
            if !contract.is_null() {
                covered!("cpk_3 relegation clause");
                contract.set_relegation_rc(2); // the triggered clause
                exe::transfer_manager_post_release_clause_activated_news(read_usize(G_TRANSFER_MANAGER), p, league, 3, contract);
            }
        }
    }
}

/// First contract offer in list l (cursor untouched) from person p (+0) to club c (+4).
pub(crate) unsafe fn first_offer_for_staff_club(l: usize, p: usize, c: usize) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == p && read_usize(e + contract_offer::CLUB_ID) == c)
}

/// `CONTRACT_MANAGER::remove_contract_offer(person, club)`: removes the person's first offer to the club. The first
/// offer in the list (`contract_manager_fields::CONTRACT_OFFERS`) of the person to the club (ids): returns the record
/// reservation (`contract_offer::RESERVED_RECORD` ≠ −1: `exe::player_regen_add_spare_non_player_record` over
/// `G_PLAYER_REGEN` with `contract_offer::RESERVED_RECORD_ARG1`, `contract_offer::RESERVED_RECORD_ARG2`), clears the
/// agreed-contract flag if it was agreed (`contract_offer::STATE` = 2:
/// `exe::transfer_manager_clear_agreed_contract_flag`), leaves the list (`exe::llist_remove`) and the person's pending
/// count (+8 of the 79-byte record in `G_STAFF_STATE_TABLE`) drops by one.
/// Original: `CONTRACT_MANAGER::remove_contract_offer` (0x4cca70, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_contract_offer(this: usize, person: *const u32, club: *const u32) {
    let (c, p) = (*club as usize, *person as usize);
    let l = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    if l.is_null() {
        return;
    }
    l.set_cursor(l.head());
    let mut e = exe::llist_get_next(l);
    while e != 0 && !(read_usize(e) == p && read_usize(e + contract_offer::CLUB_ID) == c) {
        e = exe::llist_get_next(l);
    }
    if e == 0 {
        return;
    }
    covered!("cpk_3 remove contract offer");
    let reserve = read_usize(e + contract_offer::RESERVED_RECORD);
    if reserve as i32 != -1 {
        exe::player_regen_add_spare_non_player_record(read_usize(G_PLAYER_REGEN), reserve, read_usize(e + contract_offer::RESERVED_RECORD_ARG1), read_usize(e + contract_offer::RESERVED_RECORD_ARG2));
    }
    if read_u8(e + contract_offer::STATE) == 2 {
        exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person);
    }
    exe::llist_remove(l, e);
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*person as usize).wrapping_mul(staff_state::SIZE)) + 8) as *mut u8;
    *n = (*n).wrapping_sub(1); // one fewer pending
}

/// Calls LList::get_next on list l.
pub(crate) unsafe fn contract_manager_list_next(l: usize) -> usize {
    exe::llist_get_next(l)
}

/// Calls LList::remove of item e from list l.
pub(crate) unsafe fn list_remove(l: Llist, e: usize) {
    exe::llist_remove(l, e);
}

/// Pointer to the stored suspensions counter (+0x1c) of the 29-byte discipline record of id in [this+0x14].
pub(crate) unsafe fn stored_suspension_counter(this: usize, id: usize) -> *mut i8 {
    (read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add(id.wrapping_mul(29)) + discipline_record::HELD_BAN_COUNT) as *mut i8
}

/// Pointer to the pending offers byte (+8) of the person's 79-byte staff state record.
pub(crate) unsafe fn pending_offers_byte(id: usize) -> *mut u8 {
    (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(staff_state::SIZE)) + 8) as *mut u8
}

/// `CONTRACT_MANAGER::remove_pending_offers_for_staff(person)`: removes all of the person's pending offers. With the
/// offers list (`contract_manager_fields::CONTRACT_OFFERS`) and pending offers in the person's record (+8 of the
/// 79-byte one in [0xae2d0c]): clears the agreed-contract flag (`exe::transfer_manager_clear_agreed_contract_flag` over
/// `G_TRANSFER_MANAGER`) and, from the start of the list, removes each offer of the person (`list_remove`),
/// decrementing the byte, until it reaches zero.
/// Original: `CONTRACT_MANAGER::remove_pending_offers_for_staff` (0x4ccbf0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_pending_offers_for_staff(this: usize, person: *const u32) {
    let l = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    if l.is_null() || *pending_offers_byte(*person as usize) == 0 {
        return;
    }
    exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person);
    l.set_cursor(l.head());
    let mut e = contract_manager_list_next(l.addr());
    while e != 0 {
        if read_usize(e) as u32 == *person {
            list_remove(l, e);
            let p = pending_offers_byte(*person as usize);
            *p = (*p).wrapping_sub(1);
            if *p == 0 {
                return; // the last pending one
            }
        }
        e = contract_manager_list_next(l.addr());
    }
}

/// First offer in list l (cursor untouched) matching the transfer offer's person and club and still without decision
/// (+0x62 = 0).
pub(crate) unsafe fn offer_to_schedule(l: usize, offer: Offer) -> Option<usize> {
    list_items_capped(l).into_iter().find(|&e| read_usize(e) == offer.target_staff_id() && read_usize(e + contract_offer::CLUB_ID) == offer.buyer_club_id() && *((e + contract_offer::DECISION_VALID) as *const u8) == 0)
}

/// 1 when the offered player's ability (+0xb of the player record) / 500 exceeds the offering club's reputation
/// (+0x80), else 0.
pub(crate) unsafe fn ability_above_club_reputation(offer: Offer) -> u32 {
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(offer.target_staff_id().wrapping_mul(staff::SIZE)));
    let hab = person.player().current_reputation() as i32 / 500;
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(offer.buyer_club_id().wrapping_mul(club::SIZE)));
    (hab > club.reputation() as i32) as u32
}

/// `CONTRACT_MANAGER::schedule_decision_for_transfer(offer)`: marks the first still-undecided contract offer of the
/// person and club and schedules the decision. The first contract offer in the list
/// (`contract_manager_fields::CONTRACT_OFFERS`) of the person (`.target_staff_id()`) and the club (`.buyer_club_id()`)
/// of the transfer offer still without a decision (`contract_offer::DECISION_VALID`) is marked (1) and the person's
/// decision is scheduled (`exe::contract_manager_schedule_offer_decision` with slot `.slot_index()`). Returns whether
/// the person's ability / 500 exceeds the club's reputation.
/// Original: `CONTRACT_MANAGER::schedule_decision_for_transfer` (0x4cc930, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn schedule_decision_for_transfer(this: usize, offer: usize) -> u32 {
    let offer = Offer(offer);
    let l = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    l.set_cursor(l.head());
    let mut e = contract_manager_list_next(l.addr());
    while e != 0 {
        if read_usize(e) == offer.target_staff_id() && read_usize(e + contract_offer::CLUB_ID) == offer.buyer_club_id() && *((e + contract_offer::DECISION_VALID) as *const u8) == 0 {
            write_u8(e + contract_offer::DECISION_VALID, 1); // a oferta vale
            exe::contract_manager_schedule_offer_decision(this, offer.target_staff_id() as u32, offer.slot_index() as u32);
            break;
        }
        e = contract_manager_list_next(l.addr());
    }
    ability_above_club_reputation(offer)
}

/// `CONTRACT_MANAGER::remove_held_offers(person)`: removes from the held offers list
/// (`contract_manager_fields::HELD_OFFERS`; cursor `llist::CURSOR` back to the start `llist::HEAD`) every item
/// (`exe::llist_remove`) whose +0 is the id `person` (walked with `exe::llist_get_next`). Without a list: nothing.
/// Original: `CONTRACT_MANAGER::remove_held_offers` (0x4cf560, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_held_offers(this: usize, person: u32) {
    let l = Llist(read_usize(this + contract_manager_fields::HELD_OFFERS));
    if l.is_null() {
        return;
    }
    l.set_cursor(l.head());
    let mut e = exe::llist_get_next(l);
    while e != 0 {
        if read_usize(e) as u32 == person { // the person’s offer
            exe::llist_remove(l, e);
        }
        e = exe::llist_get_next(l);
    }
}

/// Whether a suspension of type `t` (type vector [this], 4 bytes each) applies at the loan club.
///
/// By the type's competition (`+0x17`, −1 = none) being equal to the club's nation (`+0x53`, id at `+0`); with no
/// competition, by `+0x1b` (−1: applies) being equal to [`+0x71`] of the nation. Club with no nation: does not apply.
pub(crate) unsafe fn suspension_valid_at_club(this: usize, t: i8, club: Club) -> bool {
    let kind = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nation = club.nation();
    let comp = read_usize(kind + ruling_body::NATION_ID) as i32;
    if comp != -1 {
        return !nation.is_null() && read_usize(nation.addr()) as i32 == comp;
    }
    let x = read_usize(kind + ruling_body::CONTINENT_ID) as i32;
    if x == -1 {
        return true;
    }
    if nation.is_null() {
        return false;
    }
    let m = nation.continent();
    m != 0 && read_usize(m) as i32 == x // the nation’s +0x71
}

/// Loan club and the 5-byte suspension slots of the player's discipline record that do not apply there (skipping the
/// 4th like the original).
pub(crate) unsafe fn suspensions_to_store(this: usize, sheet: usize, player: usize) -> (usize, Vec<u8>) {
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(sheet + 4).wrapping_mul(club::SIZE));
    let reg = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add(read_usize(player).wrapping_mul(29));
    let mut v = Vec::new();
    for k in 0..4u8 {
        let t = read_i8(reg + 5 * k as usize);
        if k == 3 || t == -1 {
            continue;
        }
        if !suspension_valid_at_club(this, t, Club(club)) {
            v.push(k);
        }
    }
    (reg, v)
}

/// Job a vacancy job code looks for: 6/8/9/10 itself, 13→6, 15→8, others none.
pub(crate) fn job_sought_by_slot(job: u32) -> Option<u32> {
    match job as u8 as i8 {
        6 | 8 | 9 | 10 => Some(job),
        13 => Some((job & !0xff) | 6),
        15 => Some((job & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// First person in G_STAFF_TABLE without club, with the given job, a player record and the club's nation.
pub(crate) unsafe fn first_unemployed_staff(club: Club, job: u8) -> Option<usize> {
    let n = read_usize(G_STAFF_COUNT) as i32;
    let base = read_usize(G_STAFF_TABLE);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        read_usize(p + staff::CLUB_JOB) == 0 && *((p + staff::JOB_FOR_CLUB) as *const u8) == job && read_usize(p + staff::NON_PLAYER) != 0 && read_usize(p + staff::NATION) == club.nation().addr()
    })
}

/// The person's contract as `get_squad_placement` finds it inline (null, out of range or no index → 0).
pub(crate) unsafe fn squad_contract_of_staff(object: usize, person: Staff) -> Contract {
    if person.is_null() {
        return Contract(0);
    }
    let id = read_u32(person.addr()) as i32;
    if id >= read_u32(G_STAFF_COUNT) as i32 {
        return Contract(0);
    }
    let i = read_u32((read_u32(G_STAFF_STATE_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE))) as i32;
    if i < 0 {
        return Contract(0);
    }
    Contract((read_u32(object) as usize).wrapping_add((i as usize).wrapping_mul(80)))
}

/// `CONTRACT_MANAGER::get_squad_placement(person, club)`: the person's place at `club`: 0 none, 1 reserves, 2 first
/// team, 3 at the linked B team, 4 at the main club with a B team. Only for player jobs (`.job_for_club()` other than
/// 12 and `exe::database_is_player_job`). At her own club (`.club_job()`) when it can have reserves
/// (`exe::database_club_has_reserve_squad`): 2 with bit 0 of `.playing_squad()`, otherwise 1. The partner club
/// (`FN_CLUB_GET_LINKED_B_TEAM` with 1; the output byte is the person's slot, which keeps her pointer if
/// `FN_CLUB_GET_LINKED_B_TEAM` does not write it): being A with `club` her own, 4; being B with `club` the partner A,
/// 3. Otherwise by the contract (`squad_contract_of_staff` finds it in `G_CONTRACT_MANAGER`, stored in
/// `G_CONTRACT_LOOKUP_RESULT`): the contract club, which is neither her own nor the partner already seen; with bit 6 of
/// the contract's `.contract_type()` (found again in [this]): at `club` with reserves, 1 with the bit and 2 without; or
/// with the partner (`FN_CLUB_GET_LINKED_B_TEAM`) being `club` (or the contract's being `club`), 3 with the bit and 4
/// without. The game reads only al.
/// Original: `CONTRACT_MANAGER::get_squad_placement` (0x4cbb60, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_squad_placement(this: usize, person: usize, club: usize) -> u8 {
    let person = Staff(person);
    let kind = person.job_for_club() as i8;
    if kind == 12 || exe::database_is_player_job(kind) == 0 {
        return 0;
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let own = (person.club_job().addr() as u32) as usize;
    if own != 0 && own == club && exe::database_is_player_job(person.job_for_club() as i8) != 0 && exe::database_club_has_reserve_squad(club) != 0 {
        return if person.playing_squad() & 1 != 0 { 2 } else { 1 }; // main or reserves
    }
    let mut slot = person.addr() as u32; // the argument’s slot: 0x540a50 may not write it
    let mut b = 0usize;
    if own != 0 {
        b = partner(own, &mut slot, 1);
        if b != 0 {
            if slot != 0 {
                if club == (person.club_job().addr() as u32) as usize {
                    return 4;
                }
            } else if club == b {
                return 3;
            }
        }
    }
    let c = Contract((squad_contract_of_staff(read_u32(G_CONTRACT_MANAGER) as usize, person)).addr());
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr()); // the contract record
    if c.is_null() {
        return 0;
    }
    let i = (c.club() as u32) as usize;
    let other = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if other == 0 || other == (person.club_job().addr() as u32) as usize || other == b {
        return 0;
    }
    let c2 = Contract((squad_contract_of_staff(this, person)).addr());
    let mark = (c2.contract_type() << 1) as i8 >> 7; // o bit 6, estendido
    if other == club && exe::database_club_has_reserve_squad(club) != 0 {
        return if mark == 0 { 2 } else { 1 };
    }
    let p = partner(other, &mut slot, 1);
    if p == 0 || (p != club && other != club) {
        return 0;
    }
    if mark == 0 { 4 } else { 3 }
}

/// If the squad placement (`get_squad_placement`) is one of de, toggles it (`toggle_squad_placement`); else 0.
pub(crate) unsafe fn toggle_placement_if_in(this: usize, person: usize, club: usize, of: [u8; 2]) -> u32 {
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    if l != of[0] && l != of[1] {
        return 0;
    }
    exe::contract_manager_toggle_squad_placement(this, person, club, 0)
}

/// in the reserves or at the B side (placement 1 or 3), swaps through `toggle_squad_placement`; else 0
/// Original: `CONTRACT_MANAGER::move_to_first_team` (0x4cc0b0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn move_to_first_team(this: usize, person: usize, club: usize) -> u32 {
    toggle_placement_if_in(this, person, club, [1, 3]) // reserves or B team
}

/// in the first team or at the club with a B side (placement 2 or 4), swaps through `toggle_squad_placement`; else 0
/// Original: `CONTRACT_MANAGER::move_to_reserves` (0x4cc0f0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn move_to_reserves(this: usize, person: usize, club: usize) -> u32 {
    toggle_placement_if_in(this, person, club, [2, 4]) // senior team or club with a B team
}

/// Comparer of move_to_first_team / move_to_reserves: side by side outside the toggle placements, original only inside.
pub(crate) unsafe fn compare_placement_toggle(i: usize, name: &str, this: usize, person: usize, club: usize, of: [u8; 2]) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let sheet = read_u32(G_CONTRACT_LOOKUP_RESULT);
    let l = exe::contract_manager_get_squad_placement(this, person, club);
    write_u32(G_CONTRACT_LOOKUP_RESULT, sheet);
    if l == of[0] || l == of[1] {
        return o(this, person, club);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, club), || toggle_placement_if_in(this, person, club, of));
    SCOREBOARD.record(igual && r == n, || format!("{}({:x}, {:x}) place {} original={} rust={} {:x?} x {:x?}", name, person, club, l, r, n, mo, mr));
    r
}

/// `STAFF_CONTRACT::operator=`: copies the contract (0x50 bytes) from `source` field by field. Four dwords, three
/// dwords (+0x10), five bytes (+0x1c), `contract::RELEASE_FEE`, the start (`contract::DATE_STARTED`) and end
/// (`contract::CONTRACT_EXPIRES`) dates through `copy_record_date`, the bit-field byte `contract::CONTRACT_TYPE` in
/// three steps (6 bits, the 7th, the 8th: at the end it is `source`'s), `contract::PREVIOUS_WAGE`,
/// `contract::SQUAD_NUMBER`, `contract::CLUB_APPEARANCES`, `contract::HAPPINESS`, `contract::UNHAPPY_FLAG`,
/// `contract::MATCHES_LEFT_OUT`, `contract::FLAGS`, `contract::MORALE`, `contract::GRIEVANCES`,
/// `contract::LEAVING_ON_BOSMAN`, `contract::TRANSFER_ARRANGED_FOR`, `contract::TRANSFER_STATUS` and
/// `contract::SQUAD_STATUS` in two steps (low nibble and then the rest). Returns `this`.
/// Original: `STAFF_CONTRACT::operator=` (0x4cc810, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn contract_manager_operator(this: usize, source: usize) -> usize {
    let source = Contract(source);
    let this = Contract(this);
    let d = |o: usize| write_u32(this.addr() + o, read_u32(source.addr() + o));
    let w = |o: usize| write_u16(this.addr() + o, read_u16(source.addr() + o));
    let b = |o: usize| write_u8(this.addr() + o, read_u8(source.addr() + o));
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // the five bytes
    }
    d(contract::RELEASE_FEE);
    copy_record_date(this.addr() + contract::DATE_STARTED, source.addr() + contract::DATE_STARTED);
    copy_record_date(this.addr() + contract::CONTRACT_EXPIRES, source.addr() + contract::CONTRACT_EXPIRES);
    let m = (this.addr() + contract::CONTRACT_TYPE) as *mut u8;
    let s = source.contract_type();
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // the low 6 bits
    *m = cl;
    let al = ((((source.contract_type() << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = source.contract_type();
    *m = ((al ^ s) & 0x7f) ^ s; // o 8º bit
    d(contract::PREVIOUS_WAGE);
    b(contract::SQUAD_NUMBER);
    w(contract::CLUB_APPEARANCES);
    b(contract::HAPPINESS);
    b(contract::UNHAPPY_FLAG);
    b(contract::MATCHES_LEFT_OUT);
    d(contract::FLAGS);
    b(contract::MORALE);
    d(contract::GRIEVANCES);
    b(contract::LEAVING_ON_BOSMAN);
    d(contract::TRANSFER_ARRANGED_FOR);
    b(contract::TRANSFER_STATUS);
    let m = (this.addr() + contract::SQUAD_STATUS) as *mut u8;
    let c = *m;
    let al = ((source.squad_status() ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = source.squad_status();
    *m = ((al ^ s) & 0xf) ^ s;
    this.addr()
}

/// `contract_manager_raise_match_morale(person, club, contract)` (stdcall, ret 0xc): raises the contract morale after a
/// match by the rating against the club reputation (up to 10), keeping morale between −100 and 100 and under the
/// grievance cap; clears grievances 4 and 0x10 and checks the request withdrawal. With the three and the player record
/// (`.player()`): x = (rating `.current_reputation()` − club reputation `.reputation()`) + 417 × `.age()` − 11680; y =
/// 1 + x × (6.6e-5 if x < 0, otherwise 6.6667e-5), between 0.01 and 2; the gain = ftol((`.matches_left_out()` + 5) /
/// y), at most 10. The cap: 100 (50 with grievance 2), −50 per grievance 1, 8, 0x20, 0x40, 0x80, 0x100 while the cap
/// does not go below −50. The morale (`contract::MORALE`) adds the gain within −100 and 100 and stays at or under the
/// cap. Without `.leaving_on_bosman()`: with gain > 5 and morale + gain > 0, or happiness (`.happiness()`) ≥ 75 − gain,
/// it clears grievances 4 and 0x10 (`exe::staff_contract_clear_grievances`). Finally it checks the withdrawal of the
/// request (`exe::staff_contract_check_transfer_request_withdrawal` with 0). Constants from .rdata; x87 in double
/// precision, like the other contract routines.
/// Original: `contract_manager_raise_match_morale` (0x4cb0a0, contract_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn contract_manager_raise_match_morale(person: usize, club: usize, contract: usize) {
    let contract = Contract(contract);
    let person = Staff(person);
    let club = Club(club);
    if contract.is_null() || club.is_null() || person.is_null() {
        return;
    }
    let player = PlayerSheet((person.player().addr() as u32) as usize);
    if player.is_null() {
        return;
    }
    let a = player.current_reputation() as i32 - club.reputation() as i32;
    let x = (a as f64 + (person.age() as u8) as i8 as f64 * read_f64(rdata::F64_417)) - read_f64(rdata::F64_11680);
    let mut y = (if x < read_f64(rdata::F64_0) { x * read_f64(rdata::F64_6_6ENEG_05) } else { x * read_f64(rdata::F64_6_666666667ENEG_05) }) + read_f64(rdata::F64_1);
    if y < read_f64(rdata::F64_0_01) {
        y = read_f64(rdata::F64_0_01);
    } else if y > read_f64(rdata::F64_2) {
        y = read_f64(rdata::F64_2);
    }
    let happiness = (contract.matches_left_out() as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut gain = ftol(happiness / y) as u8 as i8;
    if gain > 10 {
        gain = 10;
    }
    let queixas = contract.grievances();
    let mut ceiling: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        ceiling = ceiling.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if ceiling < -50 {
            break;
        }
        if queixas & q != 0 {
            ceiling = ceiling.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let morale = (contract.addr() + contract::MORALE) as *mut i8;
    let cl = *morale;
    let s = cl as i32 + gain as i32;
    *morale = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(gain) };
    if *morale > ceiling {
        *morale = ceiling;
    }
    if contract.leaving_on_bosman() == 0 {
        let apaga = if gain > 5 && *morale as i32 + gain as i32 > 0 {
            true
        } else {
            (contract.happiness() as u8) as i8 as i32 >= 75 - gain as i32
        };
        if apaga {
            if queixas & 4 != 0 {
                exe::staff_contract_clear_grievances(contract, 4);
            }
            if (contract.grievances() as u8) & 0x10 != 0 {
                exe::staff_contract_clear_grievances(contract, 0x10);
            }
        }
    }
    exe::staff_contract_check_transfer_request_withdrawal(contract, 0);
}

/// `CONTRACT_MANAGER::remove_talks_records(person, manager, club)`: removes the person's talks (and of the club and
/// manager, when given), moving the last one into the gap, then sorts again. In the talks vector
/// (`contract_manager_fields::TALKS_RECORDS`, `contract_manager_fields::TALKS_COUNT` records of 21 bytes), each one of
/// the person (+0 = id) and, when given, of the club (`contract_talks::CLUB_ID`) and the manager
/// (`contract_talks::MANAGER_ID`) is overwritten by the last one (5 dwords and a byte), the count goes down and the
/// same place is looked at again. If any are left, it sorts again (qsort `exe::crt_qsort` with 0x4dc970).
/// Original: `CONTRACT_MANAGER::remove_talks_records` (0x4ca1c0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn remove_talks_records(this: usize, person: usize, manager: usize, club: usize) {
    let n = (this + contract_manager_fields::TALKS_COUNT) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize;
            let r = base.wrapping_add(off);
            let home = read_u32(r) == read_u32(person)
                && (club == 0 || read_u32(r + contract_talks::CLUB_ID) == read_u32(club)) // the club
                && (manager == 0 || read_u32(r + contract_talks::MANAGER_ID) == read_u32(manager));
            if home {
                let last = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(last as *const u8, r as *mut u8, 21);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(21);
            }
            i += 1;
            off = off.wrapping_add(21);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        exe::crt_qsort(read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize, (*n) as u32, 0x15, 0x4dc970);
    }
}

/// Comparer: `exe::staff_contract_clear_grievances` and `check_transfer_request_withdrawal` change the complaints, the links and the transfer (they cannot run twice): only the
/// original runs; the proof is the career and the copy's branch in Rust mode.
// simplification: original only (153,222 calls); comparing would require separating morale from the end calls.

/// Address of person *id's 79-byte staff state record (+4 = loan contract index, -1 none).
pub(crate) unsafe fn loan_state_entry(id: *const i32) -> usize {
    (read_usize(G_STAFF_STATE_TABLE) as isize + *id as isize * (staff_state::SIZE as isize)) as usize
}

/// frees the person's loan contract slot, if any
/// Original: `CONTRACT_MANAGER::remove_loan_contract` (0x4d0bd0, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn remove_loan_contract(manager: usize, id: *const i32) {
    let e = loan_state_entry(id);
    let slot = core::ptr::read_unaligned((e + 4) as *const i32);
    if slot != -1 {
        *((*(manager as *const usize) as isize + slot as isize * 80) as *mut i32) = -1;
        core::ptr::write_unaligned((loan_state_entry(id) + 4) as *mut i32, -1);
    }
}

/// Shortlist entry of club *clube (9 bytes per club in [manager+0xdc]: +0 slots, +4 mark).
pub(crate) unsafe fn club_shortlist(manager: usize, club: *const i32) -> usize {
    (read_usize(manager + shortlist_manager_fields::CLUB_SHORTLISTS) as isize + *club as isize * 9) as usize
}

/// Walks the contract manager's offer list (+0x34) from the start until achou accepts an offer; 0 if none.
pub(crate) unsafe fn find_manager_offer(manager: usize, found: impl Fn(usize) -> bool) -> usize {
    let list = Llist(read_usize(manager + contract_manager_fields::CONTRACT_OFFERS));
    if list.is_null() {
        return 0;
    }
    list.set_cursor(((list.head() as u32)) as usize);
    loop {
        let o = exe::llist_get_next(read_usize(manager + contract_manager_fields::CONTRACT_OFFERS));
        if o == 0 || found(o) {
            return o;
        }
    }
}

/// the person's contract offer for the club, in the contract manager's list
/// Original: `contract_offer` (0x4cc9e0, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn contract_offer(manager: usize, person: i32, club: i32) -> usize {
    find_manager_offer(manager, |o| read_i32(o) == person && *((o + contract_offer::CLUB_ID) as *const i32) == club)
}

/// the person's agreed contract offer (state 2)
/// Original: `agreed_contract_offer` (0x4cca30, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn agreed_contract_offer(manager: usize, person: i32) -> usize {
    find_manager_offer(manager, |o| read_i32(o) == person && *((o + contract_offer::STATE) as *const u8) == 2)
}

/// Memory regions the offer searches touch: the cursor of the manager's offer list.
pub(crate) unsafe fn offer_list_regions(manager: usize) -> Vec<(usize, usize)> {
    let list = read_usize(manager + contract_manager_fields::CONTRACT_OFFERS);
    if list == 0 { Vec::new() } else { vec![(list, 0xe)] }
}

/// Returns dword `contract::WAGE` (`.wage()`) of the contract record (0x50 bytes from [this]) of person `*p`, by the
/// index at +0 of the 79-byte record in `G_STAFF_STATE_TABLE`. 0 without a person, with an id outside `G_STAFF_COUNT`,
/// a negative index or a record at address 0.
/// Original: `contract_value` (0x4d0c20, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn contract_value(t: usize, p: *const i32) -> u32 {
    if p.is_null() || *p >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let k = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add((*p as usize).wrapping_mul(staff_state::SIZE)));
    if k < 0 {
        return 0;
    }
    let sheet = Contract(read_usize(t).wrapping_add(k as usize * 0x50));
    if sheet.is_null() {
        return 0;
    }
    sheet.wage() as u32 
}

/// `CONTRACT_MANAGER::schedule_offer_decision(id, slot)` (SEH only for the local date, with an empty destructor): finds
/// or creates the person's type 3 decision entry and sets the day and time slice of the answer: by the offer deadline,
/// or without an offer by today and the contract end; at most 14 days ahead. The person's type 3 decision entry
/// (find_type3_decision, with the limit of today + 14; without one, a new one at today + 1 + draw(5), slice 2, through
/// `exe::contract_manager_add_decision_date`). With the offer's slot (`exe::transfer_manager_get_offer_slot`): the
/// answer deadline (`exe::transfer_manager_get_offer_response_delay`: days × 3 + slices) against what is left (days
/// until the date × 3 + slices, `exe::date_days_since`); the smaller, at least 1, becomes the date (today + quotient of
/// ÷3) and the slice (+ remainder; passing 2 it goes to the next day); an offer with `offer::PRE_CONTRACT` and fewer
/// than 4 days: today + 4 + draw(2). Without a slot (−1): once the date is reached, it becomes today and the slice goes
/// up (up to the day's `G_CURRENT_DAY_SLICE`); if the person's contract ([this] + 80 × index from the 79-byte record)
/// ends before today + 180, today + 4 + draw(4). At the end, the date is no later than today + 14, and
/// `contract_manager_fields::DECISIONS_ADDED` goes up.
/// Original: `CONTRACT_MANAGER::schedule_offer_decision` (0x4cd230, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn schedule_offer_decision(this: usize, id: i32, slot: i32) {
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    let mut e = find_type3_decision(this, id, &limit, true);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if e == 0 {
        covered!("cpr_10 new decision");
        let mut d = [0u8; 8];
        injury_date_add_days(G_CURRENT_DATE, (random(5) + 1) as u32, &mut d); // cpr10: the decision in 1 to 5 days
        exe::contract_manager_add_decision_date(this, person, d.as_ptr(), 2, 3);
        let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
        e = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add(n.wrapping_sub(1).wrapping_mul(14));
    }
    let data = e + decision_date::DATE;
    if slot != -1 {
        let of = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot);
        // simplification: the original passes the slots of both arguments as output; they start with their low bytes
        let (mut a, mut b) = (id as u8, slot as u8);
        exe::transfer_manager_get_offer_response_delay(read_usize(G_TRANSFER_MANAGER), read_usize(of), &mut a, &mut b);
        let target = a.wrapping_mul(3).wrapping_add(b) as i8;
        let foul = exe::date_days_since(data, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
        let mut f = read_u8(e + decision_date::SLICE).wrapping_sub(read_u8(G_CURRENT_DAY_SLICE)) as i8;
        let mut dl = foul as u8;
        if f < 0 {
            f = f.wrapping_add(3);
            dl = dl.wrapping_sub(1);
        }
        let current = dl.wrapping_mul(3).wrapping_add(f as u8) as i8;
        let x = if current < target { current.max(1) } else { target.max(1) } as i32;
        set_today_plus_days(data, (x / 3) as u32);
        advance_time_slice(e, ((x % 3) as u8).wrapping_add(read_u8(G_CURRENT_DAY_SLICE)) as i8);
        if read_u8(read_usize(of) + offer::PRE_CONTRACT) != 0 && exe::date_days_since(data, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 4 {
            set_today_plus_days(data, (random(2) + 4) as u32);
        }
    } else {
        let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
        let (rd, ra) = (read_i16(data), read_u16(data + cm_date::YEAR));
        if ta > ra || (ta == ra && td >= rd) {
            injury_assign_date(data, G_CURRENT_DATE);
            let f = read_i8(e + decision_date::SLICE);
            if f as i32 <= read_u8(G_CURRENT_DAY_SLICE) as i32 {
                advance_time_slice(e, f.wrapping_add(1));
            }
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 {
            let x = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(staff_state::SIZE))) as i32;
            let c = Contract(if x >= 0 { read_usize(this).wrapping_add((x as usize).wrapping_mul(0x50)) } else { 0 });
            if !c.is_null() {
                let mut t = [0u8; 8];
                let m = injury_date_add_days(G_CURRENT_DATE, 0xb4, &mut t);
                let (ma, ca) = (read_u16(m + cm_date::YEAR), (c.contract_expires_year() as u16));
                if ma > ca || (ma == ca && read_i16(m) > c.contract_expires()) {
                    set_today_plus_days(data, (random(4) + 4) as u32);
                }
            }
        }
    }
    let (td, ta) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let mut t = [0u8; 8];
    let m = exe::date_sub_days(data, (t.as_mut_ptr()) as *mut u8 as usize, 0xe);
    let ma = read_u16(m + cm_date::YEAR);
    if ta < ma || (ta == ma && td < read_i16(m)) {
        set_today_plus_days(data, 0xe);
    }
    write_u32(this + contract_manager_fields::DECISIONS_ADDED, read_usize(this + contract_manager_fields::DECISIONS_ADDED).wrapping_add(1) as u32);
}

/// the person's contract record (0x50 bytes)
/// Original: `contract_record` (0x4cf6d0, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn contract_record(t: usize, p: *const i32) -> usize {
    if p.is_null() {
        return 0;
    }
    let id = *p;
    if id >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    let k = read_i32(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE)).wrapping_add(4));
    if k < 0 {
        return 0;
    }
    read_usize(t).wrapping_add(k as usize * 0x50)
}

/// Returns the category 1..8 of player `p` in contract `c`. Without a player sheet (`.player()`), 7; without a
/// contract, 4; a type 5 contract (`.contract_type()` & 0x3f), 8; otherwise by the squad status (`.squad_status()` >>
/// 4, 0..6; above, 7), the happiness (`.happiness()`, signed), the age (`.age()`), the ambition (`.ambition()`) and the
/// sheet's reputation (`player_sheet::CURRENT_REPUTATION`, `player_sheet::WORLD_REPUTATION`).
/// Original: `contract_category` (0x4ca470, contract_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn contract_category(p: usize, c: usize) -> u8 {
    let c = Contract(c);
    let p = Staff(p);
    if p.is_null() {
        return 7;
    }
    let sheet = p.player();
    if sheet.is_null() {
        return 7;
    }
    if c.is_null() {
        return 4;
    }
    if c.contract_type() & 0x3f == 5 {
        return 8;
    }
    let k = c.squad_status() >> 4;
    let v = c.happiness();
    let age = p.age() as i8;
    let ambicao = p.ambition();
    let rep = |o: usize| read_i16(sheet.addr() + o);
    let low = |limit: i8| if v <= limit { 7 } else { 4 };
    match k {
        0 => {
            if age > 0x15 {
                if v > 0x4b { 1 } else if v >= 0x28 { 2 } else if v >= 10 { 3 } else { low(-30) }
            } else if v > 0 {
                5
            } else if v > -30 || ambicao > 5 {
                6
            } else {
                7
            }
        }
        1 => if v > 0x2d { 1 } else { (v <= 0x19) as u8 + 2 },
        2 => {
            let rating = ambicao as i32 * 25 - p.loyalty() as i32 * 10 + rep(player_sheet::WORLD_REPUTATION) as i32;
            if v > 0x4b && rating > 0x186a && age > 0x17 && age < 0x22 {
                1
            } else if v > 0x19 {
                2
            } else {
                (v <= 0) as u8 + 3
            }
        }
        3 => if v > 0x37 && age > 0x17 && age < 0x22 { 2 } else { (v <= 0) as u8 + 3 },
        4 => {
            if v > 0x41 && age > 0x17 {
                (age < 0x22) as u8 + 2
            } else if v > -30 {
                4
            } else if age >= 0x18 {
                7
            } else {
                5
            }
        }
        5 => {
            if age > 0x18 {
                if age > 0x22 { 4 } else if v > 0x4b { 2 } else if v > 0 { 3 } else { low(-30) }
            } else if ambicao > 5 && rep(player_sheet::CURRENT_REPUTATION) > 0x4e2 {
                5
            } else {
                6
            }
        }
        6 => {
            if age > 0x17 {
                if v > 0x4b { 1 } else if v > 0x1e { 2 } else if v > 0 { 3 } else { low(-50) }
            } else if rep(player_sheet::CURRENT_REPUTATION) > 0x4e2 && ambicao > 0xf {
                5
            } else {
                6
            }
        }
        _ => 7,
    }
}

/// `CONTRACT_MANAGER::post_transfer_listed_news(player, club)` (SEH frame for the local list), called by the transfer
/// request (`handle_transfer_request`), the player put up for sale (`transfer_list_player`), the news
/// (`exe::newsitem_auto_respond`) and 0x4dc3a0: posts news item 0xbea of a player on the transfer list, with the price
/// and the strongest interested clubs, to the interested human clubs. Without a player, a player sheet (`.player()`), a
/// club or the staff (`.manager()`): nothing. Without a contract with the club
/// (`exe::contract_manager_get_contract_for_club`): nothing. The news item 0xbea (local): 0 the club, 1 the player,
/// 2..4 his `names::ID` of `.first_name()`, `.second_name()` and `.common_name()`, 5 the staff, 6..8 its `names::ID`, 9
/// the contract `contract::GRIEVANCES`, 10 the high nibble of `contract::SQUAD_STATUS`, 11 the positions
/// (exe::formation_get_position_mask(`position_mask`) as i16), 12 a draw of 2 (`exe::utils_random`). The price: the
/// value (`.value()`) with a human club, when the player is not on the club's list (`exe::staff_is_transfer_listed`) or
/// the contract has nibble 1; otherwise the asking price (`FN_CONTRACT_MANAGER_CALC_ASKING_PRICE`) × (draw of 0x29 ×
/// `rdata::F64_0_01` + `rdata::F64_0_8`) truncated, rounded down to 10 thousand (between 10 thousand and 200 thousand,
/// exclusive), 100 thousand (below 1 million), 250 thousand (below 5 million) or 1 million; 13 the price. From 500
/// thousand up: the interested clubs (`exe::shortlist_manager_collect_interested_clubs`), up to 30, the computer ones
/// and the human ones with a budget (`exe::finance_get_transfer_budget`) ≥ price × `rdata::F64_1_1`; sorted by
/// reputation (`club::REPUTATION`); 14 = up to 3; 15.. the ids, with 1 (in place 13) if the 2nd repeats the 1st and 2
/// (in 12) if the 3rd repeats one of them; finally the "off the list" in the next place. The recipients
/// (`exe::transfer_manager_collect_player_news_recipients` with 0x21a, up to 30, with `G_TRANSFER_MANAGER` and the
/// sheet) and the delivery (`exe::news_send_to_club_manager`).
/// Original: `CONTRACT_MANAGER::post_transfer_listed_news` (0x4d71b0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_transfer_listed_news(this: usize, player: usize, club: usize) {
    let player = Staff(player);
    let club = Club(club);
    let request: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    if player.is_null() || player.player().is_null() || club.is_null() || club.manager().addr() == 0 {
        return;
    }
    let c = exe::contract_manager_get_contract_for_club(this, player, club);
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    if c != 0 {
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = local.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0xbea, 0);
        exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
        exe::newsitem_set_slot(n, 1, read_usize(player.addr()) as u32);
        exe::newsitem_set_slot(n, 2, read_usize(player.first_name() + names::ID) as u32);
        exe::newsitem_set_slot(n, 3, read_usize(player.second_name() + names::ID) as u32);
        exe::newsitem_set_slot(n, 4, read_usize(player.common_name() + names::ID) as u32);
        let com = club.manager().addr();
        exe::newsitem_set_slot(n, 5, read_usize(com) as u32);
        exe::newsitem_set_slot(n, 6, read_usize(club.manager().first_name() + names::ID) as u32);
        exe::newsitem_set_slot(n, 7, read_usize(club.manager().second_name() + names::ID) as u32);
        exe::newsitem_set_slot(n, 8, read_usize(club.manager().common_name() + names::ID) as u32);
        exe::newsitem_set_slot(n, 9, read_usize(c + contract::GRIEVANCES) as u32);
        exe::newsitem_set_slot(n, 10, (read_u8(c + contract::SQUAD_STATUS) >> 4) as u32);
        exe::newsitem_set_slot(n, 11, (exe::formation_get_position_mask(player.player()) as i16) as i32 as u32);
        covered!("cqf_2 listed player news");
        let s = exe::utils_random(2); // cqf2: the random draw of die 12
        exe::newsitem_set_slot(n, 12, s as u32);
        let away_of_list = (exe::staff_is_transfer_listed(player, club) == 0) as u32;
        let price = if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) != 0 && (away_of_list == 0 || read_u8(c + contract::SQUAD_STATUS) & 0xf0 == 0x10) {
            player.value() as i32
        } else {
            let p = request(this, club.addr(), 0, player.addr(), 0, 1, 1);
            let r = exe::utils_random(0x29);
            let v = ftol((r as f64 * read_f64(rdata::F64_0_01) + read_f64(rdata::F64_0_8)) * p as f64); // cqf2: the price of the news;
            if v > 0x2710 && v < 0x30d40 {
                v / 10000 * 10000
            } else if v < 1_000_000 {
                v / 100000 * 100000
            } else if v < 5_000_000 {
                v / 250000 * 250000
            } else {
                v / 1_000_000 * 1_000_000
            }
        };
        exe::newsitem_set_slot(n, 0xd, price as u32);
        if price >= 500_000 {
            covered!("cqf_2 interested in listed player");
            let mut clubs = [0usize; 30];
            let mut q: i8 = 0;
            exe::shortlist_manager_collect_interested_clubs(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), player, l, 1, 1, 0);
            l.set_cursor(l.head());
            let mut in_the = exe::llist_get_next(l);
            while in_the != 0 && q < 0x1e {
                let id = read_usize(in_the + 4);
                let can = if exe::human_manager_is_human_club(transfer_manager_club_by_id(id), 0, (1) as u32, (0) as u32, 0) == 0 {
                    true
                } else {
                    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), transfer_manager_club_by_id(read_usize(in_the + 4)));
                    f != 0 && price as f64 * read_f64(rdata::F64_1_1) <= exe::finance_get_transfer_budget(f, 0, 0, 0, 1, 0) as f64
                };
                if can && read_usize(in_the + 4) as i32 > -1 {
                    clubs[q as usize] = (transfer_manager_club_by_id(read_usize(in_the + 4))).addr();
                    q += 1;
                }
                in_the = exe::llist_get_next(l);
            }
            let total = q as i32;
            let rep = |c: usize| read_i16(c + club::REPUTATION);
            let mut i = 0i32;
            while i < total - 1 {
                let mut j = i + 1;
                while j < total {
                    if rep(clubs[i as usize]) < rep(clubs[j as usize]) {
                        clubs.swap(i as usize, j as usize);
                    }
                    j += 1;
                }
                i += 1;
            }
            let m = if q >= 3 { 3 } else { total };
            exe::newsitem_set_slot(n, 0xe, m as u32);
            let mut place: i8 = 0xf;
            let mut k: i8 = 0;
            let end = loop {
                if k as i32 >= m {
                    break place;
                }
                if k == 1 && clubs[1] == clubs[0] {
                    exe::newsitem_set_slot(n, place - 2, 1);
                    break place;
                }
                if k == 2 && (clubs[2] == clubs[0] || clubs[2] == clubs[1]) {
                    exe::newsitem_set_slot(n, place - 3, 2);
                    break place;
                }
                exe::newsitem_set_slot(n, place, read_usize(clubs[k as usize]) as u32);
                place += 1;
                k += 1;
            };
            // simplification: the place goes in a dword with the high bytes of the price in the original; 0x785790 reads the byte
            exe::newsitem_set_slot(n, end, away_of_list);
            let mut dest = clubs;
            let qd = if read_usize(G_TRANSFER_MANAGER) != 0 && !player.player().is_null() {
                exe::transfer_manager_collect_player_news_recipients(read_usize(G_TRANSFER_MANAGER), player, 0x21a, dest.as_mut_ptr(), 0x1e) as i8
            } else {
                0
            };
            for &d in dest.iter().take(if qd > 0 { qd as usize } else { 0 }) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, d);
            }
        }
    }
    exe::drop_llist(l);
}

/// `CONTRACT_MANAGER::schedule_loan_end(person, club)`, only from the transfers screen (0x8fbcc0): the person's loan
/// contract ends tomorrow (type 7) and, with a human manager at the relevant club, the early return. Without a person,
/// with an id outside the total or without the loan contract (+4 of the 79-byte record, in this's vector): the box
/// (line 0xe4a). Otherwise the end (`contract::CONTRACT_EXPIRES`) becomes tomorrow (`exe::date_add_days` with 1;
/// `exe::date_operator_3d`) and the type (`.contract_type()`) becomes 7 (bits 3..5 zeroed). The current contract (+0 of
/// the record, in the vector of `G_CONTRACT_MANAGER`) in `G_CONTRACT_LOOKUP_RESULT`: from `club`, the person's club
/// (`.club_job()`) with a human manager (`FN_NEWS_CLUB_HAS_HUMAN_MANAGER`, stdcall: ecx = [0xb7431c] is not read); from
/// another, the current contract's club (read again twice) with a human manager: the return
/// (`exe::contract_manager_post_loan_recall_news`: push club; push person; mov ecx, this).
/// Original: `CONTRACT_MANAGER::schedule_loan_end` (0x4ccca0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn schedule_loan_end(this: usize, person: usize, club: usize) {
    let person = Staff(person);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let c = Contract(if person.is_null() { 0 } else { staff_contract_by_field(person, 4, this) });
    if c.is_null() {
        show_error_box(0x9a8190, 0xe4a);
        return;
    }
    let mut tomorrow = [0u32; 2];
    let p = exe::date_add_days(G_CURRENT_DATE, (&mut tomorrow) as *mut [u32; 2] as usize, 1);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(c.addr() + contract::CONTRACT_EXPIRES, tmp.as_mut_ptr() as usize, p);
    c.set_contract_type((c.contract_type() & 0xc7) | 7);
    let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
    write_usize(G_CONTRACT_LOOKUP_RESULT, a);
    let current = if a == 0 { 0 } else { (transfer_manager_club_by_id(read_usize(a + contract::CLUB))).addr() };
    if club == current {
        let x = person.club_job();
        if x.is_null() || human_manager_record_checked(x.addr()) == 0 {
            return;
        }
    } else {
        let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
        write_usize(G_CONTRACT_LOOKUP_RESULT, a);
        if a == 0 || transfer_manager_club_by_id(read_usize(a + contract::CLUB)) == Club(0) {
            return;
        }
        let a = staff_contract_by_field(person, 0, read_usize(G_CONTRACT_MANAGER));
        write_usize(G_CONTRACT_LOOKUP_RESULT, a);
        let x = if a == 0 { 0 } else { (transfer_manager_club_by_id(read_usize(a + contract::CLUB))).addr() };
        if human_manager_record_checked(x) == 0 {
            return;
        }
    }
    exe::contract_manager_post_loan_recall_news(this, person, club);
}

/// `CONTRACT_MANAGER::post_contract_end_news(p, kind, amplo)` (SEH frame for the local LLIST and news item), called by
/// the end of a contract, trial or loan (0x4c7ac0): posts news item 0xbb9 to the club, the interested clubs or, for
/// famous players, the nation's managers or everyone. Without a contract or without its club: nothing. The news item
/// 0xbb9: the contract club, the person (the id and the three keys), the kind, the contract club for kind 6 (otherwise
/// −1) and `amplo`. Wide (`amplo`), with a player sheet: world reputation (`.world_reputation()`) above 7000, to all
/// managers (`exe::news_send_to_all_managers`); national (`.current_reputation()`) above 7000, to the managers of the
/// person's nation, of the club's and of the current club's (`.club_job()`) (`exe::news_send_to_nation_managers`) and
/// to the interested clubs (`interested_club_ids`) of other nations. Otherwise to the club, to the current club for
/// kind 6 and to the interested clubs (`exe::news_send_to_club_manager`).
/// Original: `post_contract_end_news` (0x4da590, contract_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_contract_end_news(this: usize, p: usize, kind: u32, amplo: u32) {
    let p = Staff(p);
    let c = contract_inline_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == Club(0) {
        return;
    }
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    covered!("cpx_4 contract end news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbb9, 0);
    let c = contract_inline_lookup(p);
    // the original reads the id at address 0 without a contract: does not happen, it has just found it
    let club = if c == 0 { 0 } else { (friendly_club_by_id(read_usize(c + contract::CLUB))).addr() };
    exe::newsitem_set_slot(nt, 0, read_usize(club) as u32);
    exe::newsitem_set_slot(nt, 2, read_usize(p.addr()) as u32);
    exe::newsitem_set_slot(nt, 3, read_usize(p.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 4, read_usize(p.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 5, read_usize(p.common_name() + names::ID) as u32);
    let kind = kind as u8;
    exe::newsitem_set_slot(nt, 6, kind as i8 as u32);
    let c = contract_inline_lookup(p);
    let club = if c == 0 { 0 } else { (friendly_club_by_id(read_usize(c + contract::CLUB))).addr() };
    if kind == 6 {
        // the contract’s club from the manager’s own table (+0), without storing it in [0xae1080]; with no contract, the original
        // reads address 4 (does not happen: it has just found it)
        let k = if p.is_null() || read_usize(p.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            -1
        } else {
            read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(staff_state::SIZE))) as i32
        };
        let v = if k < 0 { read_usize(4) } else { read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50)) + 4) };
        exe::newsitem_set_slot(nt, 7, v as u32);
    } else {
        exe::newsitem_set_slot(nt, 7, u32::MAX);
    }
    exe::newsitem_set_slot(nt, 8, amplo);
    let sheet = p.player();
    let current = p.club_job();
    let nation = p.nation();
    let jornal = read_usize(G_NEWS);
    let mut default = true;
    if amplo != 0 && !sheet.is_null() {
        if sheet.world_reputation() > 0x1b58 {
            exe::news_send_to_all_managers(jornal, nt);
            default = false;
        } else if sheet.current_reputation() > 0x1b58 {
            default = false;
            exe::news_send_to_nation_managers(jornal, nt, nation);
            if nation != Nation(read_usize(club + club::NATION)) {
                exe::news_send_to_nation_managers(jornal, nt, read_usize(club + club::NATION));
            }
            if !current.is_null() && current.addr() != club {
                let n = current.nation().addr();
                if n != read_usize(club + club::NATION) && n != nation.addr() {
                    exe::news_send_to_nation_managers(jornal, nt, n);
                }
            }
            for id in interested_club_ids(p, Llist(l)) {
                if id == read_usize(club) || (kind == 6 && id == read_usize(p.club_job().addr())) {
                    continue;
                }
                let other = friendly_club_by_id(id);
                let n = other.nation().addr();
                if n != p.nation().addr() && n != p.club_job().nation().addr() && n != read_usize(club + club::NATION) {
                    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, other);
                }
            }
        }
    }
    if default {
        exe::news_send_to_club_manager(jornal, nt, club);
        if kind == 6 {
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, p.club_job());
        }
        for id in interested_club_ids(p, Llist(l)) {
            if id == read_usize(club) || (kind == 6 && id == read_usize(p.club_job().addr())) {
                continue;
            }
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, friendly_club_by_id(id));
        }
    }
    exe::drop_llist(l);
}

