/// How far news about an offer reaches the human managers (flag 0x10 of `collect_offer_news_recipients`), worked out
/// from the target's reputations and the offer's value.
#[derive(Default)]
pub(crate) struct OfferNewsReach {
    everyone: bool,
    continental: bool,
    national: bool,
    target_division: bool,
    buyer_division: bool,
    /// The continent of the target's nation, of the seller's nation and of the buyer's nation.
    continents: [usize; 3],
    /// The target's nation (by home reputation), the seller's nation (by current reputation or form) and the buyer's
    /// nation (a big name, a big fee or a capped international).
    nations: [usize; 3],
    /// The division of the target's club job.
    target_league: usize,
    /// The division of the buying club.
    buyer_league: usize,
}

/// The reputation of `target` as a fake person of the buying club sees it (`exe::database_get_staff_reputation_for_viewer`
/// with a person record on the stack: id 0, the buyer as club job, the buyer's nation, no second nation, no player or
/// non-player sheet, the five dates constructed; the rest is left uninitialised by the original and is not read).
unsafe fn reputation_seen_by_club(target: Staff, buyer: Club) -> i32 {
    let mut viewer = [0u8; staff::SIZE];
    let v = viewer.as_mut_ptr() as usize;
    // +0x31: the date after DATE_JOINED_NATION [inference: the nation contract end]
    for date in [staff::DATE_OF_BIRTH, staff::DATE_JOINED_NATION, 0x31, staff::DATE_JOINED_CLUB, staff::DATE_EXPIRES_CLUB] {
        exe::date_date_54a110(v + date);
    }
    write_usize(v + staff::CLUB_JOB, buyer.addr());
    write_usize(v + staff::NATION, buyer.nation().addr());
    write_usize(v + staff::NON_PLAYER, 0);
    write_usize(v + staff::PLAYER, 0);
    write_usize(v + staff::SECOND_NATION, 0);
    write_usize(v, 0);
    exe::database_get_staff_reputation_for_viewer(target, v, target.player().is_null() as u32) as i32
}

/// The reach of news about offer `o` (see `collect_offer_news_recipients`, flag 0x10).
unsafe fn offer_news_reach(this: TransferManagerFields, o: Offer, target: Staff, seller: usize, buyer: Club, flags: u32) -> OfferNewsReach {
    let mut r = OfferNewsReach::default();
    // a player job, the buyer shortlisting him in a first-team slot (slot type below 3) and a fee of a million or more
    let entry = exe::shortlist_manager_find_club_shortlist_entry(this.shortlist_manager(), buyer, target, 0, 0);
    r.buyer_division = exe::database_is_player_job(o.job_type() as i8) != 0 && entry != 0
        && read_u8(entry + shortlist_slot::FLAGS) & 0x1e < 6 && o.fee() as i32 >= 1_000_000;
    if !target.club_job().is_null() {
        r.target_league = target.club_job().division().addr();
    }
    r.buyer_league = buyer.division().addr();
    let sheet = target.player();
    let non_player = target.non_player();
    let (mut current, mut home, mut world) = (0i16, 0i16, 0i16);
    if !sheet.is_null() {
        (current, home, world) = (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation());
    } else if !non_player.is_null() {
        (current, home, world) = (non_player.current_reputation(), non_player.home_reputation(), non_player.world_reputation());
        if matches!(o.job_type(), 8..=10) {
            // jobs 8 to 10: a quarter of the reputations
            let quarter = |x: i16| ftol_x87(x as i32 as f64 * read_f64(rdata::F64_0_25)) as i16;
            (current, home, world) = (quarter(current), quarter(home), quarter(world));
        }
    }
    // the bonus: 3000 - offer value x 0.0006 (500 for a pre-contract); each threshold is at most 9750
    let bonus = if o.pre_contract() == 0 {
        let value = exe::transfer_offer_get_weighted_value(o, 0, 0, 0, 0, 1);
        ftol_x87(read_f64(0x96d200) - value as f64 * read_f64(0x96a610)) as i16 as i32 // f64 3000.0, 0.0006
    } else {
        500
    };
    let cap = |t: i32| t.min(0x2616);
    let (current, home, world) = (current as i32, home as i32, world as i32);
    let national = cap(bonus + 0x1c20);
    let in_form = |base: f64| {
        !sheet.is_null() && exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sheet, 2, 0x12, 6.5) > base - current as f64 * read_f64(rdata::F64_0_00025)
    };
    if world > cap(bonus + 0x23f0) {
        r.everyone = true;
    } else if world > cap(bonus + 0x206c) {
        r.continental = true;
        if !target.nation().is_null() {
            r.continents[0] = target.nation().continent();
        }
        if seller != 0 && !Club(seller).nation().is_null() {
            r.continents[1] = Club(seller).nation().continent();
        }
        if !buyer.nation().is_null() {
            r.continents[2] = buyer.nation().continent();
        }
    } else if home > national || current > national {
        covered!("sx02_2 national reach");
        r.national = true; // sx02_2: national by reputation
        if home > national {
            r.nations[0] = target.nation().addr();
        }
        if current > national && seller != 0 {
            r.nations[1] = Club(seller).nation().addr();
        }
        let buyer_nation = buyer.nation();
        if buyer_nation.addr() != r.nations[0] && buyer_nation.addr() != r.nations[1]
            && (world > national
                || exe::transfer_offer_get_weighted_value(o, 0, 0, 0, 0, 1) >= (buyer_nation.league_standard() as i8 as i32 - 10) * 500_000
                || !target.national_job().is_null() && target.int_apps() > 20 && target.national_job().reputation() as i16 > 5500) {
            // sx02_2: the buyer's nation for a big name, a big fee for its league or a capped international
            r.nations[2] = buyer_nation.addr();
        }
    } else if in_form(read_f64(0x970428)) && current > 3000 && flags & 0x80 != 0 {
        // f64 9.25: a player in form with some reputation reaches the seller's nation
        r.national = true;
        if seller != 0 {
            r.nations[1] = Club(seller).nation().addr();
        }
    } else if flags & 0x40 != 0 {
        if r.target_league != 0 {
            let limit = Competition(r.target_league).reputation() as i32 * 350;
            // f64 9.0 for the form
            if current > limit && current > 2500 || in_form(read_f64(0x96d230)) {
                r.target_division = true; // sx02_2: the target's division
            }
        }
        if r.buyer_league != 0 && reputation_seen_by_club(target, buyer) > Competition(r.buyer_league).reputation() as i32 * 350 && current > 1500 {
            r.buyer_division = true;
        }
    }
    r
}

/// Whether the offer's reach takes the human `manager` of `club`.
unsafe fn offer_news_reach_takes(r: &OfferNewsReach, manager: Staff, club: Club) -> bool {
    let (nation, club_nation) = (manager.nation(), club.nation());
    if r.everyone {
        return true;
    }
    if r.continental {
        let a = if nation.is_null() { 0 } else { nation.continent() };
        let b = if club_nation.is_null() { 0 } else { club_nation.continent() };
        return a != 0 && r.continents.contains(&a) || b != 0 && r.continents.contains(&b);
    }
    if r.national {
        let (n, c) = (nation.addr(), club_nation.addr());
        if n != 0 && r.nations.contains(&n) || c != 0 && r.nations.contains(&c) {
            return true;
        }
        if n == 0 {
            return false;
        }
        // England and Scotland reach each other through the manager's own nation
        let (id, england, scotland) = (read_usize(n), read_usize(KEY_NATION_ENGLAND), read_usize(KEY_NATION_SCOTLAND));
        let any_is = |key: usize| r.nations.iter().any(|&x| x != 0 && read_usize(x) == key);
        return id == england && any_is(scotland) || id == scotland && any_is(england);
    }
    let league = club.division().addr();
    r.target_division && league == r.target_league || r.buyer_division && league == r.buyer_league
}

/// Lists the human-managed clubs that should receive news of an offer, into `out` (up to `max`), and returns how many.
///
/// The target is `target_staff_id()`, the buyer `buyer_club_id()` and the seller the club of the target's contract
/// (not a type-5 contract; the lookup stored in `G_CONTRACT_LOOKUP_RESULT`).
/// - Flag 2: the seller when a human manages it (`exe::news_club_has_human_manager` on `G_NEWS`) goes first.
/// - Flag 1: the buyer when a human manages it.
/// - Flag 4: with a loan contract (`exe::contract_manager_get_loan_contract`) at a club other than the seller, the club
/// linked to the target's club job (`exe::club_get_linked_b_team` with 1, when its flag comes back 0) or the club job
/// itself, human or not; then the buyers of the rival offers (the slot list at the offer's +0) that are not closed
/// (`offer_is_closed`) and not this offer's slot, when a human manages them; past `max`, the "Error" box (line
/// 0x3936).
/// - Flag 8: with such a loan, the target's club job, human or not; then the human clubs shortlisting the target
/// (`exe::shortlist_manager_collect_interested_clubs` with 1, 1, 1) other than the buyer; past `max`, the box (line
/// 0x3959).
/// - Flag 0x10 (with 0x20, only when `player_aware()`): each of the 16 human manager slots (the last 16 people of the
/// table) with a `classification()` and a `club_job()`, not the buyer without flag 1 nor the seller without flag 2,
/// when the offer's reach (`offer_news_reach`) takes him. The target's reputations (the player sheet's, otherwise the
/// non-player's, a quarter for jobs 8 to 10) against thresholds raised by 3000 - the offer's weighted value
/// (`exe::transfer_offer_get_weighted_value` with 0, 0, 0, 0, 1) x 0.0006 (500 for a `pre_contract()`), each at most
/// 9750: world above 9200 + bonus, everyone; above 8300 + bonus, the managers whose nation or club nation is on the
/// continent of the target's, the seller's or the buyer's nation; home or current above 7200 + bonus, those of the
/// target's nation (home) or the seller's (current), England and Scotland reaching each other, and the buyer's nation
/// unless it is one of those, for a world reputation above the same threshold, a value of at least (the nation's
/// league standard - 10) x 500000 or a target with more than 20 caps for a national side above 5500 reputation;
/// otherwise a target in form (average rating above 9.25 - current reputation x 0.00025) above 3000 with flag 0x80:
/// the seller's nation; otherwise with flag 0x40: the managers in the division of the target's club job (current
/// reputation above its reputation x 350 and 2500, or in form against 9.0) and in the buyer's division (a player job
/// shortlisted by the buyer in a slot of type below 3 for a fee of a million or more, or the reputation a person of the
/// buying club sees, `reputation_seen_by_club`, above the division's reputation x 350 with current reputation above
/// 1500). A full list: the box (line 0x39f3) and the slots stop.
/// - Then the later copies of each entry are zeroed and the zeroes squeezed out, keeping the order.
///
/// Defects of the original: the loan clubs of flags 4 and 8 go in without the human check and without the `max` check.
/// Original: `TRANSFER_MANAGER::collect_offer_news_recipients` (0x8d1130, transfer_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn collect_offer_news_recipients(this: usize, o: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let this = TransferManagerFields(this);
    let o = Offer(o);
    let max = max as u8 as i8;
    let target = transfer_manager_staff_by_id(o.target_staff_id());
    let seller = contract_club_excluding_type5(target.addr());
    let buyer = transfer_manager_club_by_id(o.buyer_club_id());
    let mut n: i8 = 0;
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let human = |club: usize| exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0;
    if flags & 2 != 0 && seller != 0 && human(seller) {
        *out = seller;
        n = 1;
    }
    if flags & 1 != 0 && human(buyer.addr()) {
        *out.add(n as usize) = buyer.addr();
        n += 1;
    }
    // a loan contract at a club other than the seller
    let loan_elsewhere = || {
        exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), target) != 0
            && (seller == 0 || Contract(exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), target)).club() != read_usize(seller))
    };
    if flags & 4 != 0 {
        if loan_elsewhere() {
            let mut flag = 0u32;
            let linked = exe::club_get_linked_b_team(target.club_job(), &mut flag, 1);
            *out.add(n as usize) = if linked != 0 && flag == 0 { linked } else { target.club_job().addr() };
            n += 1;
        }
        let rivals = Llist(read_usize(o.addr())); // offer +0: the slot list of the rival offers
        if !rivals.is_null() {
            rivals.set_cursor(rivals.head());
            loop {
                let e = exe::llist_get_next(rivals);
                if e == 0 {
                    break;
                }
                let other = Offer(offer_at(this.addr(), read_usize(e)));
                if offer_is_closed(other) || read_usize(e) == o.slot_index() {
                    continue;
                }
                let club = transfer_manager_club_by_id(other.buyer_club_id());
                if human(club.addr()) {
                    if n < max {
                        covered!("sx02_2 rival bidder");
                        *out.add(n as usize) = club.addr(); // sx02_2: a rival human bidder
                        n += 1;
                    } else {
                        show_error_box(0xa8d514, 0x3936);
                    }
                }
            }
        }
    }
    if flags & 8 != 0 {
        if !target.club_job().is_null() && loan_elsewhere() {
            *out.add(n as usize) = target.club_job().addr();
            n += 1;
        }
        exe::shortlist_manager_collect_interested_clubs(this.shortlist_manager(), target, l, 1, 1, 1);
        l.set_cursor(l.head());
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            let id = read_usize(e + 4); // list entry +4: the club id
            if id == read_usize(buyer.addr()) {
                continue;
            }
            let club = transfer_manager_club_by_id(id);
            if human(club.addr()) {
                if n < max {
                    covered!("sx02_2 shortlisting human club");
                    *out.add(n as usize) = club.addr(); // sx02_2: a shortlisting human club
                    n += 1;
                } else {
                    show_error_box(0xa8d514, 0x3959);
                }
            }
        }
    }
    if flags & 0x10 != 0 && (flags & 0x20 == 0 || o.player_aware() != 0) {
        let reach = offer_news_reach(this, o, target, seller, buyer, flags);
        for k in 0..16 {
            if n == max {
                show_error_box(0xa8d514, 0x39f3);
                break;
            }
            let manager = staff_at((read_i32(G_STAFF_COUNT) - k - 1) as isize);
            let club = manager.club_job();
            if manager.classification() == 0 || club.is_null()
                || flags & 1 == 0 && club == buyer || flags & 2 == 0 && club.addr() == seller {
                continue;
            }
            if offer_news_reach_takes(&reach, manager, club) {
                *out.add(n as usize) = club.addr();
                n += 1;
            }
        }
    }
    // duplicates: the later copies are zeroed, then the zeroes squeezed out
    let count = n as i32;
    let mut i = 0i32;
    while i < count - 1 {
        if *out.add(i as usize) != 0 {
            for j in i + 1..count {
                if *out.add(j as usize) == *out.add(i as usize) {
                    *out.add(j as usize) = 0;
                }
            }
        }
        i += 1;
    }
    let mut kept: i8 = 0;
    for j in 0..count.max(0) {
        let club = *out.add(j as usize);
        if club != 0 {
            *out.add(kept as usize) = club;
            kept += 1;
        }
    }
    exe::drop_llist(l);
    kept as u8
}

/// Comparer for `collect_offer_news_recipients`: side by side from the same state of `G_CONTRACT_LOOKUP_RESULT` and of
/// the output array (`max` entries); checks the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_collect_offer_news_recipients(this: usize, o: usize, flags: u32, out: *mut usize, max: u32) -> u8 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8>(%%I0%%);
    let slots = (max as u8 as i8).max(0) as usize;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out as usize, 4 * slots)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || f(this, o, flags, out, max),
        || collect_offer_news_recipients(this, o, flags, out, max));
    SCOREBOARD.record(equal && r == n, || format!(
        "collect_offer_news_recipients(offer {:#x}, flags {:#x}): original {} rust {} {:x?} x {:x?}", o, flags, r, n, mo, mr));
    r
}
