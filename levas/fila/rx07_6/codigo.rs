// ---------- rx07_6: the contract talks records and the offer helpers of the contract screens (contract_manager.cpp) ----------

/// The size of a contract talks record (`contract_manager_fields::TALKS_RECORDS`): +0 the person, +4 the club
/// (`contract_talks::CLUB_ID`), +8 the manager (`contract_talks::MANAGER_ID`), +0xc the date, +0x14 the role.
pub(crate) const TALKS_RECORD_BYTES: usize = 0x15;

/// Rule: the order of the contract talks records: a null record first, then by person (+0), club (+4) and manager
/// (+8), each as a signed difference (the original's `a - b`, wrapping).
/// Original: `compare_talks_records` (0x4dc970, contract_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_talks_records(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    for off in [0, contract_talks::CLUB_ID] {
        let (x, y) = (read_i32(a + off), read_i32(b + off));
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    read_i32(a + contract_talks::MANAGER_ID).wrapping_sub(read_i32(b + contract_talks::MANAGER_ID))
}

compare_order!(compare_compare_talks_records, compare_talks_records, %%I0%%);

/// Rule: the contract talks record of a person, a club and a manager (ids at `person`, `club`, `manager`), by binary
/// search (`exe::crt_bsearch` with `compare_talks_records` over the sorted records); 0 when there is none or no
/// records yet.
/// Original: `CONTRACT_MANAGER::find_talks_record` (0x4ca2a0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn find_talks_record(this: usize, person: usize, manager: usize, club: usize) -> usize {
    let records = read_usize(this + contract_manager_fields::TALKS_RECORDS);
    if records == 0 {
        return 0;
    }
    let key = [read_u32(person), read_u32(club), read_u32(manager)];
    exe::crt_bsearch(key.as_ptr() as usize, records, read_u32(this + contract_manager_fields::TALKS_COUNT), TALKS_RECORD_BYTES as u32, contract_day_globals::COMPARE_TALKS)
}

/// Comparer for `find_talks_record`: side by side (both only read).
pub(crate) unsafe extern "thiscall" fn compare_find_talks_record(this: usize, person: usize, manager: usize, club: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(%%I1%%)(this, person, manager, club);
    let n = find_talks_record(this, person, manager, club);
    SCOREBOARD.record(r == n, || format!("find_talks_record(person {}, manager {}, club {}) original={:#x} rust={:#x}", read_u32(person), read_u32(manager), read_u32(club), r, n));
    r
}

/// Rule: records that a club's manager talked to a person about a role (ids at `person`, `manager`, `club`). An
/// existing record for the three is left alone. Otherwise the records grow (20 at first, `exe::crt_malloc`; then 20
/// more when full, `exe::crt_realloc`; out of memory, the contract_manager.cpp box at line 0x5b1 or 0x5bc and the game
/// stops), the new record takes the three ids, today's date (`G_CURRENT_DATE`) and the role byte, the count (+0x2c)
/// goes up and the records are sorted again (`exe::crt_qsort` with `compare_talks_records`).
/// Original: `CONTRACT_MANAGER::add_talks_record` (0x4c9f60, contract_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn add_talks_record(this: usize, person: usize, manager: usize, club: usize, role: u32) {
    if read_usize(this + contract_manager_fields::TALKS_RECORDS) != 0 && find_talks_record(this, person, manager, club) != 0 {
        return;
    }
    let capacity = this + 0x30; // the talks capacity
    if read_usize(this + contract_manager_fields::TALKS_RECORDS) == 0 {
        let p = exe::crt_malloc(20 * TALKS_RECORD_BYTES as u32);
        write_usize(this + contract_manager_fields::TALKS_RECORDS, p);
        if p == 0 {
            fatal_error_at(0x9a8190, 0x5b1);
        }
        write_u32(capacity, 20);
    }
    if read_u32(capacity) == read_u32(this + contract_manager_fields::TALKS_COUNT) {
        let p = exe::crt_realloc(read_usize(this + contract_manager_fields::TALKS_RECORDS), read_u32(capacity).wrapping_add(20).wrapping_mul(TALKS_RECORD_BYTES as u32));
        write_usize(this + contract_manager_fields::TALKS_RECORDS, p);
        if p == 0 {
            fatal_error_at(0x9a8190, 0x5bc);
        }
        write_u32(capacity, read_u32(capacity).wrapping_add(20));
    }
    covered!("rx07_6 talks recorded");
    let n = read_u32(this + contract_manager_fields::TALKS_COUNT);
    let rec = read_usize(this + contract_manager_fields::TALKS_RECORDS) + n as usize * TALKS_RECORD_BYTES;
    write_u32(rec, read_u32(person));
    write_u32(rec + contract_talks::CLUB_ID, read_u32(club));
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(rec + 0xc, scratch.as_mut_ptr() as usize, G_CURRENT_DATE); // the talks date
    write_u8(rec + 0x14, role as u8); // the role talked about
    write_u32(rec + contract_talks::MANAGER_ID, read_u32(manager));
    write_u32(this + contract_manager_fields::TALKS_COUNT, n.wrapping_add(1));
    exe::crt_qsort(read_usize(this + contract_manager_fields::TALKS_RECORDS), n.wrapping_add(1), TALKS_RECORD_BYTES as u32, contract_day_globals::COMPARE_TALKS);
}

/// Comparer for `add_talks_record`: only the original (it reallocates and sorts); then the record of the three must
/// exist and, when it is new, carry today's date and the role.
pub(crate) unsafe extern "thiscall" fn compare_add_talks_record(this: usize, person: usize, manager: usize, club: usize, role: u32) {
    let existed = read_usize(this + contract_manager_fields::TALKS_RECORDS) != 0 && find_talks_record(this, person, manager, club) != 0;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I2%%)(this, person, manager, club, role);
    let rec = find_talks_record(this, person, manager, club);
    let ok = rec != 0 && (existed || (read_u32(rec + 0xc) == read_u32(G_CURRENT_DATE) && read_u8(rec + 0x14) == role as u8));
    SCOREBOARD.record(ok, || format!("add_talks_record(person {}, manager {}, club {}, role {}) record {:#x} (existed {})", read_u32(person), read_u32(manager), read_u32(club), role as u8, rec, existed));
}

/// The main contract (staff state record +0) of the person record `s` in this manager's contracts, as the screens'
/// helpers read it inline: a null contract for a null person, an id past `G_STAFF_COUNT` or no contract.
pub(crate) unsafe fn offer_person_main_contract(this: usize, s: usize) -> Contract {
    if s == 0 || read_i32(s) >= read_i32(G_STAFF_COUNT) {
        return Contract(0);
    }
    let i = read_i32(contract_manager_staff_state_record(read_usize(s)));
    if i < 0 { Contract(0) } else { Contract((i as usize).wrapping_mul(80).wrapping_add(read_usize(this))) }
}

/// Whether a coaching rating of the person's non-player sheet reaches `level`: the coaching or the goalkeeper coaching
/// (`exe::non_player_attribute_rating`).
pub(crate) unsafe fn coaching_rating_reaches(sheet: usize, level: i8) -> bool {
    exe::non_player_attribute_rating(sheet, read_i8(sheet + non_player::COACHING)) >= level
        || exe::non_player_attribute_rating(sheet, read_i8(sheet + non_player::COACHING_GKS)) >= level
}

/// Rule: the job a contract offer to a person names. A player: 0xf (player-coach) with a coaching or goalkeeper
/// coaching rating of 10 or more on his non-player sheet, else 0xb (player). Anyone else: 9 without a non-player
/// sheet; 6 when his club job or national job is 5 or 0xc; his club job when he has a club and can take it
/// (`staff_can_take_job`), his national job when he has a national team and can take it, his club job again when he
/// can take it; else 8 with a coaching rating of 15 or more, else 9.
/// Original: `contract_manager_get_offer_job` (0x4d44e0, contract_manager.cpp, thiscall, ret 4; returns al).
pub(crate) unsafe extern "thiscall" fn get_offer_job(_this: usize, person: usize) -> u8 {
    let s = Staff(person);
    let sheet = read_usize(person + staff::NON_PLAYER);
    if !s.player().is_null() {
        covered!("rx07_6 player offer job");
        return if sheet != 0 && coaching_rating_reaches(sheet, 10) { 0xf } else { 0xb };
    }
    if sheet == 0 {
        return 9;
    }
    let (club_job, nation_job) = (s.job_for_club(), read_u8(person + staff::JOB_FOR_NATION));
    if matches!(club_job, 5 | 0xc) || matches!(nation_job, 5 | 0xc) {
        return 6;
    }
    if !s.club_job().is_null() && staff_can_take_job(club_job as u32, person) != 0 {
        return club_job;
    }
    if !s.national_job().is_null() && staff_can_take_job(nation_job as u32, person) != 0 {
        return nation_job;
    }
    if staff_can_take_job(club_job as u32, person) != 0 {
        return club_job;
    }
    if coaching_rating_reaches(sheet, 15) { 8 } else { 9 }
}

compare_value!(compare_get_offer_job, get_offer_job, %%I3%%, (person: usize) -> u8);

/// Rule: whether a person will discuss a new contract offer (`offer`, the person's id at +0 and the club's at +4) with
/// his own club. Yes (1) when he has no main contract, his contract has grievance 2, the offer comes from another club
/// or he is a non-player. No (0) when the offer's contract type is 0 (low 6 bits), he is leaving on a Bosman, he is
/// transfer listed or asked to be (`exe::staff_is_transfer_listed_or_requested`), or his squad status is 7 (high
/// nibble). Under 25, yes when his staff state flags (low nibble) and his grievances (other than 2 and 0x100) are
/// clear. No when he wants to leave (`exe::staff_contract_wants_to_leave`) or his transfer status has bit 8. Over 23,
/// left out fewer than 3 times, with happiness under -50 at a club with a manager who joined more than 150 days ago:
/// no. Otherwise yes.
/// Original: `CONTRACT_MANAGER::will_negotiate_offer` (0x4d4660, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn will_negotiate_offer(this: usize, offer: usize) -> u32 {
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer).wrapping_mul(staff::SIZE));
    let c = offer_person_main_contract(this, person);
    if c.is_null() || c.grievances() & 2 != 0 || c.club() != read_usize(offer + contract_offer::CLUB_ID) {
        return 1;
    }
    let s = Staff(person);
    if s.player().is_null() && read_usize(person + staff::NON_PLAYER) != 0 {
        return 1;
    }
    if read_u8(offer + contract_offer::CONTRACT_TYPE) & 0x3f == 0 || c.leaving_on_bosman() != 0
        || exe::staff_is_transfer_listed_or_requested(person, 0) != 0 || c.squad_status() & 0xf0 == 0x70 {
        return 0;
    }
    let age = read_i8(person + staff::AGE);
    if age < 0x19 { // rx07_6: under 25
        let flags = read_u8(contract_manager_staff_state_record(read_usize(person)) + staff_state::STATUS_FLAGS);
        if flags & 0xf == 0 && c.grievances() & 0x1fd == 0 {
            covered!("rx07_6 young player will talk");
            return 1;
        }
    }
    if exe::staff_contract_wants_to_leave(c) != 0 || c.transfer_status() & 8 != 0 {
        return 0;
    }
    let club = club_at(c.club() as isize);
    if age > 0x17 && read_i8(c.addr() + contract::MATCHES_LEFT_OUT) < 3 && read_i8(c.addr() + contract::HAPPINESS) < -50 && !club.manager().is_null() {
        let mut before = [0u32; 2];
        let d = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 0x96);
        let joined = club.manager().addr() + staff::DATE_JOINED_CLUB;
        let (dy, jy) = (read_u16(d + 2), read_u16(joined + 2));
        if dy > jy || (dy == jy && read_i16(d) > read_i16(joined)) {
            covered!("rx07_6 unhappy under an old manager");
            return 0;
        }
    }
    1
}

compare_value!(compare_will_negotiate_offer, will_negotiate_offer, %%I4%%, (offer: usize) -> u32);

/// Rule: a new offer (`offer`) to a person takes the bonuses (+0x10, 3 dwords), the clauses and the start (+0x1c, 20
/// bytes) and the end date (+0x2d) of the person's current offer from the same club, when that offer (in
/// `contract_manager_fields::CONTRACT_OFFERS`, the first of the person to the club, walked with the list cursor) is
/// in state 3. Then its wage moves halfway to the value it had before (`exe::staff_contract_calc_contract_value`, the
/// value kept as an f32) until the new value is within 0.97 to 1.03 of it or the last step is at most 10.
/// Original: `CONTRACT_MANAGER::prefill_offer_from_pending` (0x4d4830, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn prefill_offer_from_pending(this: usize, offer: usize) {
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer).wrapping_mul(staff::SIZE));
    let list = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if list == 0 {
        return;
    }
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut pending = exe::llist_get_next(list);
    while pending != 0 && !(read_usize(pending) == read_usize(offer) && read_usize(pending + contract_offer::CLUB_ID) == read_usize(offer + contract_offer::CLUB_ID)) {
        pending = exe::llist_get_next(list);
    }
    if pending == 0 {
        return;
    }
    if read_u8(pending + contract_offer::STATE) != 3 {
        return;
    }
    covered!("rx07_6 offer prefilled");
    let target = exe::staff_contract_calc_contract_value(offer, person, 0) as f32;
    for off in (0x10..0x30).step_by(4) {
        write_u32(offer + off, read_u32(pending + off));
    }
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(offer + contract::CONTRACT_EXPIRES, scratch.as_mut_ptr() as usize, pending + contract::CONTRACT_EXPIRES);
    let mut step = 100000.0f32;
    loop {
        let v = exe::staff_contract_calc_contract_value(offer, person, 0) as f64;
        let ratio = v / target as f64;
        if ratio >= read_f64(0x9690a0) && ratio <= read_f64(rdata::F64_1_03) { // 0x9690a0: 0.97
            return;
        }
        if step <= read_f32(rdata::F32_10) {
            return;
        }
        step = ((target as f64 - v) * read_f32(rdata::F32_0_5) as f64) as f32;
        write_i32(offer + contract::WAGE, ftol_x87(read_i32(offer + contract::WAGE) as f64 + step as f64));
    }
}

/// Comparer for `prefill_offer_from_pending`: side by side over the offer (0x6f bytes; the copy runs on a scratch copy
/// of it).
pub(crate) unsafe extern "thiscall" fn compare_prefill_offer_from_pending(this: usize, offer: usize) {
    let mut copy = [0u8; 0x6f];
    core::ptr::copy_nonoverlapping(offer as *const u8, copy.as_mut_ptr(), 0x6f);
    prefill_offer_from_pending(this, copy.as_mut_ptr() as usize);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I5%%)(this, offer);
    let same = core::slice::from_raw_parts(offer as *const u8, 0x6f) == &copy[..];
    SCOREBOARD.record(same, || format!("prefill_offer_from_pending(person {}, club {}) wage original={} rust={}", read_u32(offer), read_u32(offer + 4),
        read_i32(offer + contract::WAGE), i32::from_le_bytes([copy[0xc], copy[0xd], copy[0xe], copy[0xf]])));
}
