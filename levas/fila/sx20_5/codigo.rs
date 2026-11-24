// --- sx20_5: building the shortlist manager, the initial shortlists and rebuilding one club's shortlist
// (shortlist_manager.cpp: Shortlist_Manager::Shortlist_Manager 0x839a80, create_initial_shortlists 0x83a1b0,
// rebuild_club_shortlist 0x84abe0) ---

/// shortlist_manager.cpp, the source path of the error boxes.
pub(crate) const SHORTLIST_MANAGER_SOURCE: usize = 0xa825a4;
/// A club's shortlist: `SHORTLIST_SLOTS` slots of `SHORTLIST_SLOT_SIZE` bytes; the club entries are 9 bytes (+0 the
/// slots, `club_shortlist::ACTIVE`, +5 a copy of the club reputation, `club_shortlist::DAYS`).
pub(crate) const SHORTLIST_SLOT_SIZE: usize = 0x25;
pub(crate) const SHORTLIST_SLOTS: usize = 40;
pub(crate) const SHORTLIST_CLUB_ENTRY_SIZE: usize = 9;
/// Shortlist_Manager::build_position_lists and Shortlist_Manager::sort_position_lists, called through the exe.
const SHORTLIST_BUILD_POSITION_LISTS: usize = 0x839e60;
const SHORTLIST_SORT_POSITION_LISTS: usize = 0x83a780;
/// Shortlist_Manager::load, called through the exe.
const SHORTLIST_LOAD: usize = 0x83a9d0;

/// Empties a shortlist slot: +0 = 0xb, no position, valuation, price ceiling or price, no target, replacement or
/// offer (-1), flag bits 1-4 set and bits 0 and 5 cleared (bits 6-7 keep what the memory held), index -1 and the date
/// 1/1997.
unsafe fn shortlist_slot_reset(slot: ShortlistSlot) {
    write_u8(slot.addr(), 0xb); // the slot's +0 byte
    slot.set_position(0);
    slot.set_valuation(0);
    slot.set_max_price(0);
    slot.set_price(0);
    slot.set_target_staff_id(usize::MAX);
    slot.set_replacement_staff_id(u32::MAX);
    slot.set_flags(slot.flags() | 0x1e);
    slot.set_offer_ref(-1);
    slot.set_flags(slot.flags() & 0xfe);
    slot.set_index(-1);
    slot.set_flags(slot.flags() & 0xdf);
    let (mut date, mut scratch) = ([0u8; 8], [0u8; 8]);
    exe::new_date(date.as_mut_ptr() as usize, 1, 0x7cd);
    exe::date_operator_3d(slot.date_addr(), scratch.as_mut_ptr() as usize, date.as_ptr() as usize);
}

/// Builds the shortlist manager: the 9-byte club entry table at `shortlist_manager_fields::CLUB_SHORTLISTS` (out of
/// memory: the error box, line 0x54, and the game stops) and the 32 position lists cleared. With a saved game `save`,
/// loads everything from it (`Shortlist_Manager::load`). Otherwise each club gets its 40 empty slots
/// (`shortlist_slot_reset`; out of memory: line 0x6f and the game stops), inactive, its reputation copied and the
/// day counter 0; the pair arrays and their counts are cleared and the position lists built
/// (`Shortlist_Manager::build_position_lists`). Returns `this`.
/// Defect of the original: bits 6-7 of each slot's flags are never set (what `malloc` left there).
/// Original: `Shortlist_Manager::Shortlist_Manager` (0x839a80, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_shortlist_manager(this: usize, save: usize) -> usize {
    use shortlist_manager_fields as f;
    let table = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(SHORTLIST_CLUB_ENTRY_SIZE as u32));
    write_usize(this + f::CLUB_SHORTLISTS, table);
    if table == 0 {
        fatal_error_at(SHORTLIST_MANAGER_SOURCE, 0x54);
    }
    for k in 0..32 {
        write_usize(this + f::POSITION_LISTS_BY_PERSON + 4 * k, 0);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(SHORTLIST_LOAD);
        load(this, save);
        return this;
    }
    for i in 0..read_i32(G_CLUB_COUNT) {
        let entry = read_usize(this + f::CLUB_SHORTLISTS) + SHORTLIST_CLUB_ENTRY_SIZE * i as usize;
        write_usize(entry, exe::crt_malloc((SHORTLIST_SLOTS * SHORTLIST_SLOT_SIZE) as u32));
        if read_usize(entry) == 0 {
            fatal_error_at(SHORTLIST_MANAGER_SOURCE, 0x6f);
        }
        write_u8(entry + club_shortlist::ACTIVE, 0);
        write_i16(entry + 5, club_at(i as isize).reputation()); // the club reputation copy
        write_u8(entry + club_shortlist::DAYS, 0);
        for s in 0..SHORTLIST_SLOTS {
            shortlist_slot_reset(ShortlistSlot(read_usize(entry) + s * SHORTLIST_SLOT_SIZE));
        }
    }
    covered!("sx20_5 shortlist manager built");
    for o in [f::PAIRS_BY_PERSON_CAPACITY, f::PAIRS_BY_CLUB_CAPACITY, f::PAIRS_BY_PERSON_COUNT, f::PAIRS_BY_CLUB_COUNT, 0, f::PAIRS_BY_CLUB] {
        write_usize(this + o, 0);
    }
    let build: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(SHORTLIST_BUILD_POSITION_LISTS);
    build(this, 1); // sx20_5: the position lists
    this
}

/// `(value − threshold) × factor + threshold` truncated, when the value is above the threshold.
unsafe fn shortlist_wage_above(value: i32, threshold: usize, factor: usize) -> Option<i32> {
    let t = read_f64(threshold);
    if value as f64 > t { Some(ftol((value as f64 - t) * read_f64(factor) + t)) } else { None }
}

/// The wage limits of a foreigner (someone whose nation is not the club's) on a fresh contract, by the club's nation:
/// South Korea halves what passes 1000 and then what passes 2500; Germany, France, Scotland and Holland take a quarter
/// past 20000; Sweden a quarter past 250 (500 for a professional club); Wales, Finland, the Republic of Ireland and
/// Northern Ireland 0.15 past 100 (200); Denmark and Norway 0.1 past 250 (350); Belgium and Portugal raise it to
/// 5 × (club reputation / 500 + 1)² first and then take 0.2 past 250 (500); England, Italy, Spain, Japan and the
/// United States none; any other nation a quarter past 25000.
unsafe fn shortlist_cap_foreign_wage(contract: Contract, club: Club, value: i32) {
    let nation = read_i32(club.nation().addr());
    let is = |keys: &[usize]| keys.iter().any(|&k| read_i32(k) == nation);
    let professional = club.professional_status() == professional_status::PROFESSIONAL;
    let by_status = |amateur: usize, pro: usize| if professional { pro } else { amateur };
    let cap = |v: i32, threshold: usize, factor: usize| {
        if let Some(w) = shortlist_wage_above(v, threshold, factor) {
            contract.set_wage(w);
        }
    };
    if is(&[KEY_NATION_SOUTH_KOREA]) {
        cap(value, rdata::F64_1000, rdata::F64_0_5);
        cap(contract.wage(), 0x9690b0, rdata::F64_0_5); // f64 = 2500.0
    } else if is(&[KEY_NATION_GERMANY, KEY_NATION_FRANCE, KEY_NATION_SCOTLAND, KEY_NATION_HOLLAND]) {
        cap(value, 0x968888, rdata::F64_0_25); // f64 = 20000.0
    } else if is(&[KEY_NATION_SWEDEN]) {
        cap(value, by_status(0x968898, rdata::F64_500), rdata::F64_0_25); // 0x968898: f64 = 250.0
    } else if is(&[KEY_NATION_WALES, KEY_NATION_FINLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND]) {
        cap(value, by_status(rdata::F64_100, rdata::F64_200), rdata::F64_0_15);
    } else if is(&[KEY_NATION_DENMARK, KEY_NATION_NORWAY]) {
        cap(value, by_status(0x968898, 0x96f870), rdata::F64_0_1); // f64 = 250.0, 350.0
    } else if is(&[KEY_NATION_BELGIUM, KEY_NATION_PORTUGAL]) {
        let band = (club.reputation() as i32 / 500 + 1) as i8 as i32;
        let floor = band * band * 5;
        if floor as f64 > value as f64 {
            contract.set_wage(floor);
        }
        cap(contract.wage(), by_status(0x968898, rdata::F64_500), rdata::F64_0_2); // f64 = 250.0
    } else if !is(&[KEY_NATION_ENGLAND, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_JAPAN, KEY_NATION_UNITED_STATES]) {
        cap(value, 0x9688d0, rdata::F64_0_25); // f64 = 25000.0
    }
}

/// The initial shortlists of a new game: every club's squad assessed (`assess_club_squad`, in reputation order, with
/// the progress text "Assessing Club Squads"); then, for each person ("Creating Shortlists"), his club (its B team
/// when it is a linked team of the main club), and, when his contract with it has no wage yet (-1), the default wage
/// (`staff_contract_calc_unset_default_wage` with his squad status) capped for a foreigner by the club's nation
/// (`shortlist_cap_foreign_wage`); a player gets his value (`contract_manager_calc_player_value`, human clubs flagged).
/// Then the position lists are sorted and every club's transfer budget shared across its slots
/// (`allocate_transfer_budget`, in reputation order). The progress goes out every 100 clubs and at the end.
/// Original: `Shortlist_Manager::create_initial_shortlists` (0x83a1b0, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_create_initial_shortlists(this: usize) {
    let mut text = [0u8; 0x64];
    let say = |text: &mut [u8; 0x64], s: usize| {
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        f(text.as_mut_ptr() as usize, s);
    };
    let progress = |text: &[u8; 0x64], i: i32| {
        if i % 100 == 0 {
            exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, i, read_u32(G_CLUB_COUNT), -1);
        }
    };
    let by_reputation = |i: i32| club_at(read_i32(read_usize(G_CLUB_REPUTATION_ORDER) + 4 * i as usize) as isize);
    say(&mut text, 0xa825ec); // "Assessing Club Squads"
    for i in 0..read_i32(G_CLUB_COUNT) {
        progress(&text, i);
        let club = by_reputation(i);
        exe::shortlist_manager_assess_club_squad(this, shortlist_club_entry(this, club.addr()), club, 0, 1);
    }
    say(&mut text, 0xa825d8); // "Creating Shortlists"
    for k in 0..read_i32(G_STAFF_COUNT) {
        let person = staff_at(k as isize);
        let mut club = person.club_job();
        if !club.is_null() {
            let mut reserve = 0u32;
            let b = exe::club_get_linked_b_team(club, &mut reserve, 1);
            if b != 0 && reserve == 0 {
                club = Club(b);
            }
        }
        let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club));
        if !club.is_null() && !contract.is_null() && contract.wage() == -1 {
            covered!("sx20_5 default wage");
            // simplification: the original pushes the status in a register whose high bytes are leftovers (read as a byte)
            let status = (contract.squad_status() >> 4) as i32; // sx20_5: the squad status
            let value = ftol(exe::staff_contract_calc_unset_default_wage(contract, person, club, 1, status, 0xb, 0));
            contract.set_wage(value);
            if !club.nation().is_null() && person.nation() != club.nation() {
                shortlist_cap_foreign_wage(contract, club, value);
            }
        }
        if !person.player().is_null() {
            let human = !club.is_null() && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
            person.set_value(exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), person, club, 0, -1, human as u32) as usize);
        }
    }
    let sort: unsafe extern "thiscall" fn(usize) = core::mem::transmute(SHORTLIST_SORT_POSITION_LISTS);
    sort(this);
    for i in 0..read_i32(G_CLUB_COUNT) {
        progress(&text, i);
        exe::shortlist_manager_allocate_transfer_budget(this, by_reputation(i)); // sx20_5: the transfer budgets
    }
    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text.as_ptr() as usize, read_i32(G_CLUB_COUNT), read_u32(G_CLUB_COUNT), -1);
}

/// Rebuilds one club's shortlist (nothing for a national team): its slot targets released
/// (`shortlist_manager_release_club_slot_targets`), each of its shortlist pairs taken out of both sorted pair arrays
/// (`collect_club_shortlist_entries`, `remove_sorted_pair`), its squad assessed again (`assess_club_squad`) and,
/// for a computer club, its transfer budget shared again (`allocate_transfer_budget`). Then the value of each of its
/// 50 squad players with a valid id (`contract_manager_calc_player_value`): for a human club (`human_manager_is_human_club`
/// with 1, 1) flagged as human, except when its manager is on holiday with the holiday option and the player's
/// contract lacks transfer status bit 4.
/// Original: `Shortlist_Manager::rebuild_club_shortlist` (0x84abe0, shortlist_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_rebuild_club_shortlist(this: usize, club: Club) {
    let mut list = [0u8; 0xe];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    if exe::database_is_national_team(club) != 0 {
        exe::drop_llist(l);
        return;
    }
    exe::shortlist_manager_release_club_slot_targets(this, club, shortlist_club_entry(this, club.addr()));
    exe::shortlist_manager_collect_club_shortlist_entries(this, club, l);
    write_u32(l + llist::CURSOR, read_u32(l + llist::HEAD));
    loop {
        let pair = exe::llist_get_next(l);
        if pair == 0 {
            break;
        }
        let key = [read_u32(pair), read_u32(pair + 4)];
        remove_sorted_pair(this, &key, 0x757ea0);
        remove_sorted_pair(this + shortlist_manager_fields::PAIRS_BY_CLUB, &key, 0x844740);
    }
    covered!("sx20_5 club shortlist rebuilt");
    exe::shortlist_manager_assess_club_squad(this, shortlist_club_entry(this, club.addr()), club, 0, 1);
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        exe::shortlist_manager_allocate_transfer_budget(this, club);
    }
    let human = exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0;
    for k in 0..50 {
        let person = Staff(club.squad(k));
        if person.is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let mut flag = 0u32; // sx20_5: a computer club
        if human {
            let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
            let away = exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), exe::database_get_club_manager(club)) != 0
                && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 0, exe::database_get_club_manager(club)) != 0
                && !contract.is_null();
            if away && contract.transfer_status() & 4 == 0 {
                continue;
            }
            flag = 1;
        }
        person.set_value(exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), person, club, 0, -1, flag) as usize);
    }
    exe::drop_llist(l);
}

/// Comparer for `shortlist_manager_shortlist_manager`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_shortlist_manager(this: usize, save: usize) -> usize {
    // simplification: allocates the tables and loads the save (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Comparer for `shortlist_manager_create_initial_shortlists`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_create_initial_shortlists(this: usize) {
    // simplification: assesses every squad and shares every budget (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `shortlist_manager_rebuild_club_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_rebuild_club_shortlist(this: usize, club: Club) {
    // simplification: rebuilds the shortlist and the sorted pair arrays (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, Club)>(%%I2%%)(this, club)
}
