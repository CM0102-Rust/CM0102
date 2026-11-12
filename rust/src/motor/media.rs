use super::*;

/// Calls human_manager is_human_club(club, a, b, 0, 0).
pub(crate) unsafe fn media_is_human_club(c: usize, a: u32, b: u32) -> u32 {
    exe::human_manager_is_human_club(c, a, b, 0, 0)
}

/// Collects the human clubs that receive news about a player (TRANSFER_MANAGER collect_player_news_recipients,
/// `exe::transfer_manager_collect_player_news_recipients`).
pub(crate) unsafe fn player_news_recipients(player: Staff, marks: u32, out: *mut usize, max: u32) -> u8 {
    exe::transfer_manager_collect_player_news_recipients(read_usize(G_TRANSFER_MANAGER), player, marks, out, max)
}

/// Writes a person's id and three name keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
pub(crate) unsafe fn set_person_name_slots(write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    let p = Staff(p);
    write(n, k, read_usize(p.addr()) as u32);
    write(n, k + 1, read_usize(p.first_name() + names::ID) as u32);
    write(n, k + 2, read_usize(p.second_name() + names::ID) as u32);
    write(n, k + 3, read_usize(p.common_name() + names::ID) as u32);
}

/// Appends club c to the recipients array (max 30) if not already there.
pub(crate) fn add_unique_recipient(dest: &mut [usize; 30], n: &mut u8, c: usize) {
    if !dest[..(*n as i8).max(0) as usize].contains(&c) {
        dest[*n as i8 as usize] = c;
        *n = n.wrapping_add(1);
    }
}

/// `Media::process_transfer_interest()` (SEH frame without a destructor on the normal path), called by the daily
/// routine (0x5cdd8f): drops transfer interest older than 31 days and, by chance, publishes the rumour linking the
/// player to the interested clubs, with the transfer request, the AI manager response, the squad reaction and the rival
/// warning; the AI owner may say he will not sell.
///
/// For each interest record (25-byte vector at `media_fields::TRANSFER_INTERESTS`, count
/// `media_fields::INTEREST_COUNT`: +0 player, `transfer_interest::CLUB` club, `transfer_interest::DATE` date,
/// `transfer_interest::STATE` state, `transfer_interest::VALUE`, `transfer_interest::RATING`): expired (more than 31
/// days, `exe::date_add_days`), no manager at the owning club (the `club::MANAGER` of the club of the player's
/// contract, kept from the previous iteration when he has no contract) or no player sheet: it leaves (the last one
/// takes its place). Otherwise, if the club can reach the player (`exe::transfer_manager_can_club_bid_for_player`):
/// state 0 and draw(25) zero give the rumour: the up to 5 clubs interested in him (records of the same player still at
/// 0, marked 1; one already marked cancels), news item 0xfbc (`exe::new_newsitem`) with the player, the owner, the
/// transfer request (below 15 in `.professionalism()` and `.loyalty()`, reputation above draw(12500) and
/// `transfer_interest::RATING` above 12.5 × `.loyalty()` + 500: 0x24, otherwise draw(3)), `transfer_interest::VALUE`,
/// the AI manager's response (`exe::media_pick_ai_manager_response`), the strengths
/// (`exe::staff_get_player_strength_flags`), the position mask (`exe::formation_get_position_mask`), the owner's
/// manager, the transfer list (`exe::staff_is_transfer_listed_or_requested`) and the interested clubs by reputation; to
/// the humans (`player_news_recipients`: the interested ones and the humans of the same division, or of the same nation
/// and up to 3000 below) (`exe::news_send_to_club_manager`); with the request, the squad reaction
/// (`exe::media_apply_squad_reaction_to_player_story`); with the response, the rival warning
/// (`exe::media_warn_squad_of_position_rival`) and the reply to the story (`exe::media_apply_player_story_response`).
///
/// In state 1, an AI owner, the record date after the player's `.date_joined_club()`, the contract without
/// `contract::LEAVING_ON_BOSMAN` and off the list, happiness (`contract::HAPPINESS`) above draw(50) + 40 and draw(30)
/// zero: the news that he will not sell (`exe::news_create_not_for_sale_story`) and his records go to 2. At the end,
/// the vector is sorted (`exe::crt_qsort` with `compare_id_pair_keys`).
/// Original: `Media::process_transfer_interest` (0x754a30, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn process_transfer_interest(this: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    if read_i32(this + media_fields::INTEREST_COUNT) == 0 {
        return;
    }
    // simplification: the owner’s manager stays from the previous pass when the player has no contract (the original does not rewrite it);
    // on the first pass the original reads the stack without a value: here 0
    let mut manager_owner: usize = 0;
    let mut i = 0i32;
    while i < read_i32(this + media_fields::INTEREST_COUNT) {
        let r = read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add((i as usize).wrapping_mul(25));
        let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(r) as usize).wrapping_mul(staff::SIZE)));
        let club = month_award_club_by_id(read_i32(r + transfer_interest::CLUB));
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        let (mut dono, mut vinc) = (0usize, 0usize);
        if c != 0 {
            dono = (month_award_club_by_id(read_i32(c + 4))).addr();
            if dono != 0 {
                vinc = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, dono);
                manager_owner = read_usize(dono + club::MANAGER);
            }
        }
        let mut today = [0u8; 8];
        core::ptr::copy_nonoverlapping(G_CURRENT_DATE as *const u8, today.as_mut_ptr(), 8);
        let mut end = [0u8; 8];
        let p = exe::date_add_days(r + transfer_interest::DATE, (end.as_mut_ptr()) as usize, 0x1f); // cqc10: o interesse vale 31 dias
        covered!("cqc_10 interest");
        let (hd, ha) = (i16::from_le_bytes([today[0], today[1]]), u16::from_le_bytes([today[2], today[3]]));
        let expired = ha > read_i16(p + 2) as u16 || (ha == read_i16(p + 2) as u16 && hd > read_i16(p));
        if expired || manager_owner == 0 || person.player().is_null() {
            let n = read_i32(this + media_fields::INTEREST_COUNT);
            let last = read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add((n as usize).wrapping_mul(25)).wrapping_sub(25);
            core::ptr::copy(last as *const u8, r as *mut u8, 25);
            write_i32(this + media_fields::INTEREST_COUNT, n - 1);
            continue; // the last one came into place i: the original goes back one and forward one
        }
        if exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), person, club, (0) as *mut u16, 0) == 0 {
            i += 1;
            continue;
        }
        if read_u8(r + transfer_interest::STATE) == 0 && {
            covered!("cqc_10 rumour draw");
            friendly_random(0x19) == 0 // cqc10: the rumour random draw
        } {
            // the clubs interested in the player
            let mut ids = [-1i32; 5];
            ids[0] = read_i32(r + transfer_interest::CLUB);
            write_u8(r + transfer_interest::STATE, 1);
            let mut cnt: i8 = 1;
            let mut cancelar = false;
            let mut j = 0i32;
            while j < read_i32(this + media_fields::INTEREST_COUNT) && cnt < 5 {
                let rj = read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add((j as usize).wrapping_mul(25));
                if j != i && read_i32(rj) == read_i32(r) {
                    if read_u8(rj + transfer_interest::STATE) != 0 {
                        cancelar = true;
                        break;
                    }
                    ids[cnt as usize] = read_i32(rj + transfer_interest::CLUB);
                    cnt += 1;
                    write_u8(rj + transfer_interest::STATE, 1);
                }
                j += 1;
            }
            if cancelar {
                i += 1;
                continue;
            }
            covered!("cqc_10 rumour");
            let mut news = [0u32; 0x3a]; // the local news
            let n = news.as_mut_ptr() as usize;
            exe::new_newsitem(n, 0xfbc, 0);
            set_person_name_slots(write, n, 0, person.addr());
            write(n, 4, if dono != 0 { read_usize(dono) as u32 } else { 0xffff_ffff });
            let mut pede = false;
            let mut sai = false;
            if person.professionalism() < 0xf && person.loyalty() < 0xf {
                covered!("cqc_10 transfer request");
                let rr = friendly_random(0x30d4); // cqc10: the reputation random draw for the request
                if (person.player().world_reputation() as i32) > rr {
                    let lim = person.loyalty() as i32 as f64 * 12.5 + 500.0;
                    if lim < read_i32(r + transfer_interest::RATING) as f64 {
                        write(n, 5, 0x24);
                        pede = true;
                        sai = true;
                    }
                }
            }
            if !sai {
                let s = friendly_random(3);
                write(n, 5, s as u32);
            }
            write(n, 6, read_usize(r + transfer_interest::VALUE) as u32);
            let resp = if dono != 0 && media_is_human_club(dono, 0, 1) == 0 {
                exe::media_pick_ai_manager_response(this, read_usize(dono + club::MANAGER), person, 0x1f)
            } else {
                0
            };
            write(n, 7, resp);
            write(n, 8, exe::staff_get_player_strength_flags(person, dono));
            write(n, 9, exe::formation_get_position_mask(person.player()) as i16 as i32 as u32);
            if manager_owner != 0 {
                set_person_name_slots(write, n, 0xa, manager_owner);
            } else {
                for k in 0xa..=0xd {
                    write(n, k, 0xffff_ffff);
                }
            }
            write(n, 0xe, (exe::staff_is_transfer_listed_or_requested(person, 0) != 0) as u32);
            write(n, 0xf, cnt as i32 as u32);
            // the interested parties by reputation (highest to lowest)
            let rep = |id: i32| read_i16((month_award_club_by_id(id)).addr() + club::REPUTATION);
            let c = cnt as usize;
            for a in 0..c.saturating_sub(1) {
                for b in a + 1..c {
                    if rep(ids[b]) > rep(ids[a]) {
                        ids.swap(a, b);
                    }
                }
            }
            for (k, &id) in ids.iter().take(c).enumerate() {
                write(n, 0x10 + k as i8, id as u32);
            }
            // the destinations
            let mut dest = [0usize; 30];
            let mut nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(person, 0xda, dest.as_mut_ptr(), 0x1e) } else { 0 };
            for &id in ids.iter().take(c) {
                if (nd as i8) >= 0x1e {
                    break;
                }
                add_unique_recipient(&mut dest, &mut nd, (month_award_club_by_id(id)).addr());
            }
            let total = read_i32(G_STAFF_COUNT);
            for q in 0..16i32 {
                let h = Staff(read_usize(G_STAFF_TABLE).wrapping_add(((total - 1 - q) as usize).wrapping_mul(staff::SIZE)));
                let hc = h.club_job();
                if h.classification() == 0 || hc.is_null() {
                    continue;
                }
                for &id in ids.iter().take(c) {
                    if (nd as i8) >= 0x1e {
                        break;
                    }
                    let ci = month_award_club_by_id(id);
                    let hc = h.club_job();
                    let junto = ci.division() == hc.division()
                        || (ci.nation() == hc.nation() && (ci.reputation() as i32 - 0xbb8) > hc.reputation() as i32);
                    if junto {
                        add_unique_recipient(&mut dest, &mut nd, hc.addr());
                    }
                }
            }
            for &d in dest.iter().take((nd as i8).max(0) as usize) {
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, d);
            }
            if pede {
                exe::media_apply_squad_reaction_to_player_story(this, 0x1f, person, dono, ids.as_mut_ptr(), cnt as u8 as u32);
            }
            if dono != 0 && media_is_human_club(dono, 0, 1) == 0 && resp != 0 {
                if !club.is_null() {
                    exe::media_warn_squad_of_position_rival(this, person, club, dono, resp);
                }
                exe::media_apply_player_story_response(this, person, dono, 0x1f, resp);
            }
            i += 1;
            continue;
        }
        // state 1: the AI owner may warn that it does not sell
        if read_u8(r + transfer_interest::STATE) == 1 && dono != 0 && media_is_human_club(dono, 1, 1) == 0 {
            let (rd, ra) = (read_i16(r + transfer_interest::DATE), read_i16(r + transfer_interest::DATE_YEAR) as u16);
            let (pd, pa) = ((person.date_joined_club() as i16), (person.date_joined_club_year() as i16) as u16);
            let after = ra > pa || (ra == pa && rd > pd);
            if after && vinc != 0 && read_u8(vinc + contract::LEAVING_ON_BOSMAN) == 0 && exe::staff_is_transfer_listed_or_requested(person, 0) == 0 {
                let happy = read_i8(vinc + contract::HAPPINESS) as i32;
                let s = friendly_random(0x32);
                if happy > s + 0x28 && friendly_random(0x1e) == 0 {
                    covered!("cqc_10 will not sell");
                    exe::news_create_not_for_sale_story(person, dono, read_usize(dono + club::CHAIRMAN), 0);
                    let mut j = 0i32;
                    while j < read_i32(this + media_fields::INTEREST_COUNT) {
                        let rj = read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add((j as usize).wrapping_mul(25));
                        if read_i32(rj) == read_i32(read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add((i as usize).wrapping_mul(25))) {
                            write_u8(rj + transfer_interest::STATE, 2);
                        }
                        j += 1;
                    }
                }
            }
        }
        i += 1;
    }
    let n = read_i32(this + media_fields::INTEREST_COUNT);
    if n != 0 {
        exe::crt_qsort(read_usize(this + media_fields::TRANSFER_INTERESTS), (n) as u32, 0x19, 0x757ea0);
    }
}

// simplification: removes records (the vector), rolls random numbers, builds and delivers news and calls the reactions and replies (0x755340,
// 0x758010, 0x741020, 0x587600): does not run twice. Original only; the copy carries the branches and is proven by the career and
// by the mutations.

/// `news_create_not_for_sale_story(player, club, chairman, other)` (cdecl; media.cpp in the flow, european_cup.cpp in
/// the map, with no other caller; SEH frame without a destructor on the normal path): news item 0xfbe that the club
/// will not sell the player.
///
/// With a player sheet and a club, news item 0xfbe (`exe::new_newsitem`) with the player, the other club (or −1), the
/// club and a draw(3); the chairman, if he likes the player (`exe::staff_is_favoured_by`) or with
/// draw(`.interference()` of his sheet) above draw(20), otherwise −1 in slot 7; reputation (`.world_reputation()` of
/// the sheet) above 8000: to all managers (`exe::news_send_to_all_managers`), otherwise to the interested human
/// managers (`player_news_recipients` with 0xd8, up to 30, `exe::news_send_to_club_manager`).
/// Original: `news_create_not_for_sale_story` (0x587600, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_create_not_for_sale_story(player: usize, club: usize, chairman: usize, other: usize) {
    let chairman = Staff(chairman);
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || club == 0 {
        return;
    }
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut dest = [0usize; 30];
    let nd: u8 = if read_usize(G_TRANSFER_MANAGER) != 0 { player_news_recipients(player, 0xd8, dest.as_mut_ptr(), 0x1e) } else { 0 };
    covered!("cqc_10 will not sell news");
    let mut news = [0u32; 0x3a];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xfbe, 0);
    set_person_name_slots(write, n, 0, player.addr());
    write(n, 4, if other != 0 { read_usize(other) as u32 } else { 0xffff_ffff });
    write(n, 5, read_usize(club) as u32);
    let s = friendly_random(3);
    write(n, 6, s as u32);
    let cita = exe::staff_is_favoured_by(player, chairman) != 0 || (!chairman.is_null() && !chairman.non_player().is_null() && {
        let a = friendly_random(chairman.non_player().interference() as i32);
        a > friendly_random(0x14)
    });
    if cita {
        set_person_name_slots(write, n, 7, chairman.addr());
    } else {
        write(n, 7, 0xffff_ffff);
    }
    let f = player.player();
    if !f.is_null() && f.world_reputation() > 0x1f40 {
        exe::news_send_to_all_managers(read_usize(G_NEWS), n);
    } else {
        for &d in dest.iter().take((nd as i8).max(0) as usize) {
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, d);
        }
    }
}

// simplification: the news goes to the managers (allocates): original only; the copy takes the branch.

/// `Media::queue_trophy_praise_story(club, comp, mark)` (SEH frame without a destructor on the normal path), called by
/// the title records (0x8693e2, 0x86b000): news item 0x2e with the board and fans praising the manager for the title.
///
/// A club with a manager, a competition and outside national sides: news item 0x2e (`exe::new_newsitem`) with the
/// manager, the club, the competition, a draw(2), the outlet (`exe::media_pick_outlet_type_for_club`), the pundit
/// (`exe::punditlist_pick_random_pundit` with the nation or −1), the flag, the board confidence (+6 of the row in
/// [`G_MANAGER_MANAGER`], or 5000), the club's league and the manager's days (`exe::date_days_since`); with 180 days or
/// more, the club's reputation and the delivery to the manager (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_trophy_praise_story` (0x75c390, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn queue_trophy_praise_story(_this: usize, club: usize, comp: usize, mark: u32) {
    let club = Club(club);
    if club.is_null() || club.manager().addr() == 0 || comp == 0 {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    covered!("cqc_10 title news");
    let mut news = [0u32; 0x3a];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x2e, 0);
    set_person_name_slots(write, n, 0, club.manager().addr());
    write(n, 4, read_usize(club.addr()) as u32);
    write(n, 5, read_usize(comp) as u32);
    let s = friendly_random(2);
    write(n, 6, s as u32);
    let v = exe::media_pick_outlet_type_for_club(club);
    write(n, 7, v);
    let na = club.nation().addr();
    let c = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, v, if na != 0 { read_i32(na) } else { -1 });
    write(n, 8, c as u32);
    write(n, 9, mark as i8 as i32 as u32);
    let mm = read_usize(G_MANAGER_MANAGER);
    let conf = if mm != 0 {
        read_i16(read_usize(mm).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(0x49)) + 6) as i32
    } else {
        0x1388
    };
    write(n, 0xa, conf as u32);
    write(n, 0xb, (club.division().addr() == comp) as u32);
    let manager = club.manager();
    let days = exe::date_days_since(G_CURRENT_DATE, manager.date_joined_club() as u32, manager.date_joined_club_leap_year() as u32);
    write(n, 0xc, days as u32);
    if days < 0xb4 {
        return;
    }
    write(n, 0xd, club.reputation() as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news item goes to the manager (allocates): original only; the copy carries the branch.

/// CONTRACT_MANAGER get_contract_for_club (`exe::contract_manager_get_contract_for_club`) for a person and club.
pub(crate) unsafe fn media_contract_for_club(person: usize, club: Club) -> usize {
    exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, club)
}

/// The person's whole contract at a club (+0..+0x4f) as a comparer region, or None.
pub(crate) unsafe fn contract_region(person: usize, club: Club) -> Option<(usize, usize)> {
    let c = media_contract_for_club(person, club);
    (c != 0).then_some((c, 0x50))
}

/// A routine with no port that internally calls tape units (`staff_is_transfer_listed_or_requested` and `staff_is_listed_by_request` call `exe::contract_manager_get_contract_for_club`; `get_player_injury_status` reaches
/// `exe::human_manager_is_human_club` and ports): runs it on a scratch tape and replays the recorded inner calls through `chamar`, in order.
///
/// In tape comparison, the original routine records those inner calls at the upper level (with the routine's comparer,
/// the original and its copy each record their own). The copy runs the routine on a scratch tape and requests the same
/// calls through `chamar`, in order.
///
/// Outside the comparison (without RECORD_POST_MATCH) it just calls it.
pub(crate) unsafe fn replay_inner_tape_calls<R>(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, f: impl FnOnce() -> R) -> R {
    if !RECORD_POST_MATCH.load(std::sync::atomic::Ordering::Relaxed) {
        return f();
    }
    let externa = tape_lock().replace((Vec::new(), Vec::new()));
    let r = f();
    let gravadas = core::mem::replace(&mut *tape_lock(), externa).map_or(Vec::new(), |g| g.1);
    for c in gravadas {
        call(c.0, c.1, c.2);
    }
    r
}

/// The vector that `apply_squad_reaction_to_player_story` builds on the stack (58 dwords; it walks 57).
///
/// The 50 squad players (`database_collect_club_squad` with 0, cdecl), the club's +0xbf (chairman), the manager (+0xcf) only when there is
/// no human at the club (`exe::human_manager_is_human_club` with 0, 1, 0, 0, cdecl, via `chamar`: a tape unit) and outside types 11 to 15
/// (`exe::database_is_player_job`, cdecl, byte +0x3d: player jobs), the +0xd3 (assistant) and the five dwords of +0x19f (coaches).
pub(crate) unsafe fn club_staff_vector(club: Club, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> [usize; 58] {
    let mut v = [0usize; 58];
    exe::database_collect_club_squad(club, v.as_mut_ptr(), 0);
    v[50] = club.chairman().addr();
    let manager = club.manager();
    v[51] = if call(FN_HUMAN_MANAGER_IS_HUMAN_CLUB, 0, [club.addr() as u32, 0, 1, 0, 0]) != 0 || (!manager.is_null() && exe::database_is_player_job((manager.job_for_club() as u32) as i8) != 0) {
        0
    } else {
        manager.addr()
    };
    v[52] = club.assistant_manager();
    for k in 0..5 {
        v[53 + k] = club.coaches(k);
    }
    v
}

/// `Media::apply_squad_reaction_to_player_story(tópico, jogador, clube, ids, n)` (`0x755340`, thiscall, ret 0x14,
/// `media.cpp`): after a story about a player, the interested clubs and the club's staff may come to dislike him,
/// setting contract flags and grievances.
///
/// Called after the transfer rumour `0x754a30` with topic 0x1f and the interested clubs; `0x72c1f0` and `0x733a60`
/// without ids.
///
/// The chance starts at 10 (5 if the player is a favourite of the club, `0x547790`; 15 if rejected, `0x5477d0`) and
/// grows per interested club: 100 in the Glasgow derby, 10 if the league has the fixture (`0x4b02e0`), 15 if the club
/// is his rival (`+0xb3`/`+0xb7`/`+0xbb`), 5 if it is in the same division (outside the lower division). Random
/// draw(chance) above random draw(10): the club rejects the player (`0x547940` with 1).
///
/// Listed (`0x544520`): −25; without a contract link: −15; otherwise −10 at the end of contract (`0x867d30`) plus
/// happiness (`+0x3d`) / 10; at least 5. For each of the 57 persons of the vector (club_staff_vector) with preferences
/// (`+0x65`): the score (10, the manager sheet's `+0x1d` + 10, or by `+0x5d`, age, `+0x5b` and the squad player's
/// request listing), adjusted by topic 0x20/0x1e, by `+0x59`/`+0x5a`/`+0x5b`, by liking the player or not
/// (`0x547810`/`0x547840`) and by the player being a favourite or rejected by the club, at least 5; random draw(chance)
/// above random draw(score): the person comes to dislike the player (`0x547af0` with 1) and, with a link and random
/// draw(score) below random draw(10), contract mark 1 (`0x867560`) or, in topics 0x1e/0x20 or with random draw(score)
/// zero, grievances 0x800 (`0x867700`).
///
/// The three calls with side effects go through `chamar` (the post-match gates), as do the tape units that the original
/// routine calls: the link `0x4cf5f0` (`0x75551e`, `0x7557dc`), the human `0x600a50` (`0x755598`) and the `0x4cf5f0`
/// calls inside `0x544520` (`0x7554e8`) and `0x544600` (`0x75567f`, `0x7556ad`), the latter via
/// `replay_inner_tape_calls`.
pub(crate) unsafe fn squad_reaction_to_story_with(topic: u32, player: Staff, club: Club, ids: usize, n: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    if club.is_null() || player.is_null() {
        return;
    }
    covered!("cqb_8 squad reaction");
    let mut chance: i32 = 0xa; // cqb8: a chance inicial
    if exe::club_favours_staff(player, club) != 0 {
        chance = 5;
    } else if exe::club_dislikes_staff(player, club) != 0 {
        chance = 0xf;
    }
    if ids != 0 {
        let n = n as i8 as i32;
        let mut k = 0i32;
        while k < n {
            let id = read_usize(ids + 4 * k as usize);
            let other = Club(read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE)));
            k += 1;
            let (celtic, rangers) = (read_usize(G_KEY_CLUB_GLASGOW_CELTIC), read_usize(G_KEY_CLUB_GLASGOW_RANGERS));
            let my = read_usize(club.addr());
            if (my == celtic || my == rangers) && (read_usize(other.addr()) == celtic || read_usize(other.addr()) == rangers) {
                chance += 100;
                continue;
            }
            if exe::comp_util_get_league_clash_type(club, other) != 0 {
                chance += 10;
                continue;
            }
            if club.addr() == other.rival1() || club.addr() == other.rival2() || club.addr() == other.rival3() {
                chance += 0xf;
                continue;
            }
            let div = club.division().addr();
            if div != 0 && read_usize(div) != read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && div == other.division().addr() {
                chance += 5;
            }
        }
    }
    let r1 = friendly_random(chance);
    if r1 > friendly_random(10) {
        exe::club_add_disliked_staff(club, player, 1);
    }
    // cqb8: 0x4cf5f0 (from inside 0x544520 and direct) is a tape unit: goes through `chamar`
    if replay_inner_tape_calls(call, || exe::staff_is_transfer_listed_or_requested(player, 0)) != 0 {
        chance -= 0x19;
    } else {
        let c = call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [player.addr() as u32, club.addr() as u32, 0, 0, 0]) as usize;
        if c == 0 {
            chance -= 0xf;
        } else {
            if exe::staff_contract_get_expiry_band(c) != 0 {
                chance -= 10;
            }
            chance += (read_i8(c + contract::HAPPINESS) as i32 / 10) as i8 as i32;
        }
    }
    if chance < 5 {
        chance = 5;
    }
    let v = club_staff_vector(club, call);
    let topic = topic as u16;
    for &p in v.iter().take(57) {
        if p == 0 || p == player.addr() || read_usize(p + staff::STAFF_PREFERENCES) == 0 {
            continue;
        }
        let mut rating: i8 = 10;
        let manager = NonPlayer(read_usize(p + staff::NON_PLAYER));
        if !manager.is_null() {
            rating = (manager.patience() as i8).wrapping_add(10);
        } else if read_usize(p + staff::PLAYER) != 0 {
            if (player.age() as i8) > 0x14 && read_i8(p + staff::AGE) < 0x17 && read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) < 0x109a {
                continue;
            }
            covered!("cqb_8 player rating");
            rating = read_i8(p + staff::TEMPERAMENT).wrapping_add(10); // cqb8: the rating of the squad player
            if read_i8(p + staff::AGE) < 0x14 {
                rating = rating.wrapping_sub(5);
            }
            let m = read_i8(p + staff::PROFESSIONALISM);
            if m == 0x14 {
                rating = rating.wrapping_add(1);
            } else if m == 1 {
                rating = rating.wrapping_sub(if replay_inner_tape_calls(call, || exe::staff_is_listed_by_request(p, 0)) != 0 { 0xf } else { 10 });
                if read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x1676 {
                    rating = rating.wrapping_sub(10);
                }
            } else if m < 6 {
                rating = rating.wrapping_sub(if replay_inner_tape_calls(call, || exe::staff_is_listed_by_request(p, 0)) != 0 { 10 } else { 5 });
                if read_i16(read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) > 0x1676 {
                    rating = rating.wrapping_sub(5);
                }
            }
        }
        if topic == 0x20 && read_i8(p + staff::LOYALTY) > 0xf && read_i8(p + staff::TEMPERAMENT) < 6 {
            rating = rating.wrapping_sub(5);
        }
        let a59 = read_i8(p + staff::LOYALTY);
        if a59 > 0xf {
            if read_i8(p + staff::PROFESSIONALISM) != 0x14 {
                rating = rating.wrapping_sub(2);
            }
        } else if a59 < 5 {
            rating = rating.wrapping_add(2);
        }
        if topic == 0x1e {
            let (a5b, a5a) = (read_i8(p + staff::PROFESSIONALISM), read_i8(p + staff::PRESSURE));
            if a5b > 0xf {
                if a5a < 0xf {
                    rating = rating.wrapping_sub(1);
                }
            } else if a5b >= 6 {
                rating = rating.wrapping_sub(if a5a >= 0xf { 1 } else { 3 });
            } else {
                rating = rating.wrapping_sub(if a5a < 6 { 10 } else { 5 });
            }
        }
        if exe::staff_is_favoured_by(player, p) != 0 {
            rating = rating.wrapping_add(0x19);
        } else if exe::staff_is_disliked_by(player, p) != 0 {
            rating = rating.wrapping_add(player.professionalism().wrapping_sub(0x19));
            if rating > 0xf {
                rating = 0xf;
            }
        }
        if exe::club_favours_staff(player, club) != 0 {
            rating = rating.wrapping_add(5);
        } else if exe::club_dislikes_staff(player, club) != 0 {
            rating = rating.wrapping_sub(5);
        }
        if rating < 5 {
            rating = 5;
        }
        let r1 = friendly_random(chance);
        if r1 > friendly_random(rating as i32) { // cqb8: o desafeto sorteado
            covered!("cqb_8 drawn grudge");
            call(FN_STAFF_ADD_DISLIKED_STAFF, 0, [p as u32, player.addr() as u32, 1, 0, 0]);
            let c = call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [p as u32, club.addr() as u32, 0, 0, 0]) as usize;
            if c != 0 {
                let r = friendly_random(rating as i32);
                if r < friendly_random(10) {
                    if topic != 0x1e && topic != 0x20 && friendly_random(rating as i32) != 0 {
                        call(FN_STAFF_CONTRACT_SET_FLAGS, c, [1, 0, 0, 0, 0]);
                    } else {
                        call(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS, c, [0x800, 0, 0, 0, 0]);
                    }
                }
            }
        }
    }
}

/// after a story about the player (the transfer rumour): the dislike chance from the interested clubs (Old Firm, league
/// fixture, rival, same division), the club rejecting him and, for each squad, staff and board member, a score from
/// squad status, ambition, loyalty and preferences; when drawn, the person comes to dislike the player and his contract
/// gets flag 1 or the 0x800 complaints
/// Original: `Media::apply_squad_reaction_to_player_story` (0x755340, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn apply_squad_reaction_to_player_story(_this: usize, topic: u32, player: usize, club: usize, ids: usize, n: u32) {
    squad_reaction_to_story_with(topic, Staff(player), Club(club), ids, n, &mut |r, t, a| dispatch_post_match_call(r, t, a))
}

/// `Media::warn_squad_of_position_rival(jogador, clube, _, _)` (`warn_squad_of_position_rival`, thiscall, ret 0x10, media.cpp): squad players
/// in the same position may react to a reputable newcomer with a story (0x25) and contract mark 0x800. Called from the
/// transfer rumour (`process_transfer_interest`) and from the answer to the player's story (`exe::media_apply_player_story_response`).
///
/// Conditions: with a player sheet, a club with more than 2 squad players in his position mask (`club_count_squad_with_position_mask` with
/// `staff_get_natural_position_mask`), a division with a competition ([0xadadfc]) and reputation (player sheet +0xb) above 4750. Then each of
/// the 50 squad players (+0xd7) with a player sheet, a link (`exe::contract_manager_get_contract_for_club`) with positive happiness (+0x3d), the player
/// without a status-2 injury (`get_player_injury_status` on [0xb63d28]), without the 0x101 marks (+0x45), not on the list (`staff_is_transfer_listed_or_requested`),
/// over 20 years old, of type 0x10/0x20/0x30 (+0x4f & 0xf0), with +0x5a below n + 5 and +0x5b below n + 10, random
/// draw(+0x5a) below random draw(n), the same mask and reputation above 3750: the first time, his story 0x25 (`exe::media_create_player_media_story`
/// on [0xb6b7c8]); without mark 0x800 (`contract_flags`), mark 0x800 is set (`set_flags`).
///
/// The two calls with side effects go through `chamar`, and so does link `exe::contract_manager_get_contract_for_club` (0x7580c2, a tape unit) and, via
/// `replay_inner_tape_calls`, what `get_player_injury_status` (0x7580ee) and `staff_is_transfer_listed_or_requested` (0x75810a) call as units inside them.
pub(crate) unsafe fn warn_position_rivals_with(player: Staff, club: Club, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    if player.is_null() || player.player().is_null() || club.is_null() {
        return;
    }
    let n = exe::club_count_squad_with_position_mask(club, exe::staff_get_natural_position_mask(player)) as i8;
    if n <= 2 {
        return;
    }
    let div = club.division().addr();
    if div == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * read_usize(div)) == 0 {
        return;
    }
    if player.player().current_reputation() <= 0x128e {
        return;
    }
    covered!("cqb_8 positional rival");
    let n = n as i32;
    let mut postou = false;
    for i in 0..50 {
        let p = Staff(club.squad(i));
        if p.is_null() || p.player().is_null() || p == player {
            continue;
        }
        // cqb8: 0x4cf5f0 is a tape unit (direct and from inside 0x544520; 0x62c400 reaches 0x600a50)
        let c = Contract(call(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, read_usize(G_CONTRACT_MANAGER), [p.addr() as u32, club.addr() as u32, 0, 0, 0]) as usize);
        if c.is_null() || p.player().is_null() || c.happiness() <= 0 {
            continue;
        }
        if replay_inner_tape_calls(call, || exe::injury_get_player_injury_status(read_usize(G_INJURY), player)) == 2 || (c.grievances() as usize) & 0x101 != 0
            || replay_inner_tape_calls(call, || exe::staff_is_transfer_listed_or_requested(p, club)) != 0
        {
            continue;
        }
        if (p.age() as i8) <= 0x14 {
            continue;
        }
        let t = c.squad_status() & 0xf0;
        if t != 0x10 && t != 0x20 && t != 0x30 {
            continue;
        }
        let a5a = p.pressure() as i32;
        if a5a >= n + 5 || p.professionalism() as i32 >= n + 10 {
            continue;
        }
        let r1 = friendly_random(n);
        if friendly_random(a5a) >= r1 {
            continue;
        }
        let m = exe::staff_get_natural_position_mask(player);
        if m != exe::staff_get_natural_position_mask(p) || p.player().current_reputation() <= 0xea6 {
            continue;
        }
        if !postou {
            call(FN_MEDIA_CREATE_PLAYER_MEDIA_STORY, read_usize(G_MEDIA), [p.addr() as u32, club.addr() as u32, 0x25, 0, player.addr() as u32]);
            postou = true;
        }
        if exe::staff_contract_get_flags(c) & 0x800 != 0 {
            continue;
        }
        call(FN_STAFF_CONTRACT_SET_FLAGS, c.addr(), [0x800, 0, 0, 0, 0]);
    }
}

/// with a high-reputation player linked to the club and more than two squad players in his position: the unsettled
/// position rivals (by ambition and loyalty, by a draw) get contract flag 0x800, and the first one gives story 0x25
/// Original: `Media::warn_squad_of_position_rival` (0x758010, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn warn_squad_of_position_rival(_this: usize, player: usize, club: usize, _a: u32, _b: u32) {
    warn_position_rivals_with(Staff(player), Club(club), &mut |r, t, a| dispatch_post_match_call(r, t, a))
}

/// Answers 1 and 2 of each case in the jump table of `media_format_response_option_label` (the 3rd, for all, is "No Comment", 0xa27808); the case
/// comes from the exe's byte table at 0x7356f4 (type − 3, up to 0x7f; above that, 21).
pub(crate) const PRESS_ANSWERS: [(usize, usize); 22] = [
    (0xa27700, 0xa2771c), (0xa27670, 0xa2767c), (0xa27688, 0xa276a4), (0xa276e4, 0xa276f0), (0xa2773c, 0xa27770),
    (0xa2764c, 0xa2765c), (0xa27430, 0xa2743c), (0xa2762c, 0xa2763c), (0xa2744c, 0xa27460), (0xa275d0, 0xa27534),
    (0xa27554, 0xa27574), (0xa27510, 0xa27534), (0xa275b4, 0xa275c0), (0xa274e0, 0xa274f8), (0xa27588, 0xa275a0),
    (0xa274b4, 0xa274c4), (0xa2744c, 0xa27494), (0xa2744c, 0xa27610), (0xa275e8, 0xa275f8), (0xa2744c, 0xa2747c),
    (0xa27798, 0xa277c8), (0xa2740c, 0xa2741c),
];

/// `media_format_response_option_label(kind, option, out, size)` (stdcall, ret 0x10): writes the label of a media story
/// response option ("Deny rumour", "Defend Player", "No Comment"...) from the 22-case table by story kind into the
/// output (`FN_LANGLIB_SPRINTF`), for the stories screen (0x742b3b, with 0xde1f64 and 0x7d0). A null output or a size
/// below 15: nothing.
/// Original: `media_format_response_option_label` (0x735160, media.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn media_format_response_option_label(kind: u32, option: u32, out: usize, size: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if out == 0 || (size as i16) < 0xf {
        return;
    }
    let k = (kind as i16 as i32).wrapping_sub(3) as u32;
    let case = if k > 0x7f { 21 } else { read_u8(0x7356f4 + k as usize) as usize };
    let (um, dois) = PRESS_ANSWERS[case];
    let text = match option as i8 {
        1 => um,
        2 => dois,
        _ => 0xa27808,
    };
    format(out, text);
}

/// Staff record of id (0x6e bytes each in G_STAFF_TABLE).
pub(crate) unsafe fn media_staff_by_id(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// Staff natural position mask (`staff_get_natural_position_mask`).
pub(crate) unsafe fn natural_position_mask(person: Staff) -> u32 {
    exe::staff_get_natural_position_mask(person)
}

/// STAFF_CONTRACT get_flags (`contract_flags`, contract +0x40).
pub(crate) unsafe fn media_contract_flags(c: usize) -> u32 {
    exe::staff_contract_get_flags(c)
}

/// STAFF_CONTRACT clear_flags (`exe::staff_contract_clear_flags`) on a contract.
pub(crate) unsafe fn clear_contract_flags(c: usize, marks: u32) {
    exe::staff_contract_clear_flags(c, marks);
}

/// Player's current reputation (+0xb of the player record at staff +0x61).
pub(crate) unsafe fn current_reputation(person: Staff) -> i16 {
    person.player().current_reputation()
}

/// The rehearsal of the comparer of `process_transfer_squad_reactions`: the copy runs without anything that allocates, sends news or requests a
/// transfer; when it reaches one of these, `MEDIA_REACTION_STOPPED` is set and it stops (the comparison then relies on
/// the original routine alone).
pub(crate) static MEDIA_REACTION_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static MEDIA_REACTION_STOPPED: AtomicBool = AtomicBool::new(false);

/// STAFF_CONTRACT set_flags; in the dry run, checks whether it would trigger a transfer request and returns true (stop)
/// if so.
pub(crate) unsafe fn set_contract_flags_or_stop(c: usize, marks: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        MISS_TRAINING_DRY_RUN.store(true, Relaxed);
        MISS_TRAINING_DECISION.store(0, Relaxed);
        set_flags(c, marks);
        MISS_TRAINING_DRY_RUN.store(false, Relaxed);
        if MISS_TRAINING_DECISION.load(Relaxed) != 0 {
            MEDIA_REACTION_STOPPED.store(true, Relaxed);
        }
        return MEDIA_REACTION_STOPPED.load(Relaxed);
    }
    exe::staff_contract_set_flags(c, marks);
    false
}

/// The transfer-request decision of `exe::staff_contract_apply_event_flags` with the given `marcas` (no side effect: +0x45 is restored).
///
/// Jobs 0xb, 0xf and 0xd, excluding the manager and the assistant of the contract's club, morale (+0x44) non-negative,
/// without +0x49, wants to leave (`staff_criterion`, with the flags already set), no future transfer (`get_future_transfer`), without bits 1
/// and 8 of +0x4e, not in the free-transfer queue (`person_in_transfer_list`), and over 20 years old or reputation above 4250.
pub(crate) unsafe fn would_request_transfer(c: Contract, marks: u32) -> bool {
    let person = Staff(media_staff_by_id(read_i32(c.addr())));
    let function = person.job_for_club();
    if function != 0xb && function != 0xf && function != 0xd {
        return false;
    }
    let club = club_at(c.club() as i32 as isize).addr();
    for o in [0xcf, 0xd3] {
        let t = read_usize(club + o);
        if t != 0 && read_usize(c.addr()) == read_usize(t) {
            return false;
        }
    }
    if (c.morale() as i32) < 0 || c.leaving_on_bosman() != 0 {
        return false;
    }
    let before = c.grievances() as usize;
    c.set_grievances((before | marks as usize) as u32);
    let exit = exe::staff_contract_wants_to_leave(c) != 0;
    c.set_grievances((before) as u32);
    if !exit {
        return false;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 || c.transfer_status() & 9 != 0 {
        return false;
    }
    if exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), person) != 0 {
        return false;
    }
    !(((person.age() as i8) as i32) <= 0x14 && current_reputation(person) <= 0x109a)
}

/// STAFF_CONTRACT apply_event_flags (`exe::staff_contract_apply_event_flags`); in the dry run stops first if the person would request a transfer.
pub(crate) unsafe fn apply_event_flags_or_stop(c: usize, marks: u32, person: Staff) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) && would_request_transfer(Contract(c), marks) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    exe::staff_contract_apply_event_flags(c, marks, (person.addr()) as u32);
    false
}

/// STAFF_CONTRACT clear_grievances (`exe::staff_contract_clear_grievances`); in the dry run stops first if the transfer request would be withdrawn.
pub(crate) unsafe fn clear_grievances_or_stop(c: usize, queixas: u32) -> bool {
    let c = Contract(c);
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        let before = c.grievances() as usize;
        c.set_grievances((before & !(queixas as usize)) as u32);
        let retira = should_withdraw_transfer_request(c).is_some();
        c.set_grievances((before) as u32);
        if retira {
            MEDIA_REACTION_STOPPED.store(true, Relaxed);
            return true;
        }
    }
    exe::staff_contract_clear_grievances(c, queixas);
    false
}

/// Queues the signing celebration story (`queue_signing_celebration_story`); in the dry run stops instead.
pub(crate) unsafe fn signing_celebration_or_stop(this: usize, p: Staff, club: Club, kind: u32, porta_voz: usize) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    exe::media_queue_signing_celebration_story(this, p, club, kind, 1, porta_voz);
    false
}

/// Picks the squad spokesman: among the club's 50 players other than p with reputation above 4250, the highest
/// random(reputation + 20 x (+0x5b + +0x59) +- age adjustment).
pub(crate) unsafe fn squad_spokesman(club: Club, p: Staff) -> usize {
    let (mut melhor, mut quem) = (0i32, 0usize);
    for j in 0..50usize {
        let m = Staff(club.squad(j));
        if m.is_null() || m.player().is_null() || m == p {
            continue;
        }
        let r = current_reputation(m);
        if r <= 0x109a {
            continue;
        }
        let mut x = (r as i32).wrapping_add(((m.professionalism() as i32) + (m.loyalty() as i32)) * 20);
        let age = (m.age() as i8) as i32;
        x += if age < 0x13 {
            -0x5dc
        } else if age < 0x17 {
            -0x2ee
        } else if age < 0x1c {
            -0xfa
        } else if age < 0x22 {
            0xfa
        } else {
            0
        };
        let s = friendly_random(x);
        if s > melhor {
            melhor = s;
            quem = m.addr();
        }
    }
    quem
}

/// `Media::process_transfer_squad_reactions(p, new, old, flag)`, at the end of each transfer
/// (TRANSFER_MANAGER::move_player_to_club, 0x8ca879): the squad reactions to the transfer. Only for a player
/// (`.player()`) aged 19 or more with reputation 4250.
///
/// (A) The media story at the front of the queue (0xbd0, `story_log_entry::SUBTYPE` = 0x25, new club with id 0x25;
/// never in practice): grievances in its person's contract. (B) In the new club's squad (50 slots `.squad()`), those
/// who have been there more than `staff::PRESSURE` + 15 days (`FN_DATE_DAYS_SINCE` of `staff::DATE_JOINED_CLUB`) and
/// have a contract: flag 0x800 with the same position turns into a grievance (`exe::staff_contract_apply_event_flags`
/// 0x80000000) and draws; 0x400 is cleared with a strong squad (`exe::database_count_club_squad` up to the limit by
/// reputation); with more than 2 in the position, the strong player may become a grievance or flag 0x200; grievance
/// `grievances::STAR_TEAMMATE_SOLD` is cleared (`exe::staff_contract_clear_grievances`) when someone bigger arrives. A
/// star signing (player's `.world_reputation()` above 6250): the supporters (+8 of the club's row in
/// [[`G_MANAGER_MANAGER`]], +500 up to 10000) and the celebration (`queue_signing_celebration_story`); if something
/// changed, draw(3) and the spokesman celebrate.
///
/// (C) In the old club's squad (with a league), with the club's strength and reputation (or the nation among the seven
/// `KEY_NATION_GREECE`, `KEY_NATION_SWEDEN`, `KEY_NATION_WALES`, `KEY_NATION_FINLAND`,
/// `KEY_NATION_REPUBLIC_OF_IRELAND`, `KEY_NATION_NORTHERN_IRELAND`, `KEY_NATION_AUSTRALIA`): grievances 0x40000000 /
/// 0x20000000 at the team mate leaving, morale (`contract::MORALE`) −25, flag 0x400, flags 0x200/0x800 and grievance
/// `grievances::SAME_POSITION_SIGNING` cleared; if something changed and the player was a star, the unsettled team mate
/// (`exe::media_queue_sale_unrest_story`).
/// Original: `Media::process_transfer_squad_reactions` (0x758200, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn process_transfer_squad_reactions(this: usize, p: usize, new: usize, old: usize, flag: u32) {
    let p = Staff(p);
    let old = Club(old);
    let new = Club(new);
    if p.is_null() || new.is_null() {
        return;
    }
    let sheet = p.player();
    if sheet.is_null() || ((p.age() as i8) as i32) < 0x13 || sheet.current_reputation() < 0x109a {
        return;
    }
    let mut changed = false;
    // (A) the media story at the front of the queue
    if read_i32(this + media_fields::STORY_LOG_COUNT) > 0 {
        let h = read_usize(this);
        if read_i32(h + story_log_entry::TYPE) == 0xbd0 && read_i16(h + story_log_entry::SUBTYPE) == 0x25 && read_i32(new.addr()) == 0x25 {
            let kind = read_u8(h + story_log_entry::RESPONSE);
            let id = read_i32(h + story_log_entry::PERSON_A);
            if id != -1 {
                let q = media_staff_by_id(id);
                let rp = current_reputation(p);
                if natural_position_mask(p) == natural_position_mask(Staff(q)) && (rp > 0x128e || rp as i32 - 0x5dc > current_reputation(Staff(q)) as i32) {
                    let c = media_contract_for_club(q, new);
                    if c != 0 {
                        covered!("cqa_13 media story");
                        changed = true;
                        if kind == 1 {
                            if apply_event_flags_or_stop(c, 0x8000000, Staff(0)) || apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        } else if kind == 2 {
                            if media_contract_flags(c) & 0x800 != 0
                                && (set_contract_flags_or_stop(c, 0x200) || apply_event_flags_or_stop(c, 0x80000000, p) || set_contract_flags_or_stop(c, 0x80))
                            {
                                return;
                            }
                        } else if friendly_random(read_i8_as_i32(q + staff::PROFESSIONALISM)) < friendly_random(10) {
                            if set_contract_flags_or_stop(c, 0x80) {
                                return;
                            }
                            if friendly_random(read_i8_as_i32(q + staff::PRESSURE)) == 0 && apply_event_flags_or_stop(c, 0x80000000, p) {
                                return;
                            }
                        }
                        if media_contract_flags(c) & 0x800 != 0 {
                            clear_contract_flags(c, 0x800);
                        }
                    }
                }
            }
        }
    }
    // (B) the new club’s squad
    let strength = exe::database_count_club_squad(new, 0, 0) as i8 as i32;
    let rn = new.reputation();
    let limit = if rn < 0x9c4 { 0x14 } else if rn < 0x1482 { 0x19 } else if rn >= 0x1c52 { 0x20 } else { 0x1e }; // cqa13: the strong squad limit
    let same = exe::club_count_squad_with_position_mask(new, natural_position_mask(p)) as i8 as i32;
    let days: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let category: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    for j in 0..50usize {
        let m = new.squad(j);
        if m == 0 || read_usize(m + staff::PLAYER) == 0 || m == p.addr() {
            continue;
        }
        if days(G_CURRENT_DATE, read_usize(m + staff::DATE_JOINED_CLUB) as u32, read_usize(m + staff::DATE_JOINED_CLUB_LEAP_YEAR) as u32) <= read_i8_as_i32(m + staff::PRESSURE) + 0xf {
            continue;
        }
        let c = media_contract_for_club(m, new);
        if c == 0 {
            continue;
        }
        if media_contract_flags(c) & 0x800 != 0 && natural_position_mask(p) == natural_position_mask(Staff(m)) {
            clear_contract_flags(c, 0x800);
            if apply_event_flags_or_stop(c, 0x80000000, p) {
                return;
            }
            if friendly_random(read_i8_as_i32(m + staff::PRESSURE)) == 0 && set_contract_flags_or_stop(c, 0x200) {
                return;
            }
            if friendly_random(read_i8_as_i32(m + staff::PROFESSIONALISM)) == 0 && set_contract_flags_or_stop(c, 0x80) {
                return;
            }
        }
        if media_contract_flags(c) & 0x400 != 0 && strength >= limit {
            covered!("cqa_13 strong squad");
            clear_contract_flags(c, 0x400);
        }
        if same > 2 && media_contract_flags(c) & 0x400 == 0 && current_reputation(Staff(m)) > 0x109a && natural_position_mask(p) == natural_position_mask(Staff(m)) {
            let rp = current_reputation(p);
            if rp > 0x128e || rp as i32 - 0x5dc > new.reputation() as i32 {
                let a = read_i8_as_i32(m + staff::PRESSURE);
                if a < 0xf && read_i8_as_i32(m + staff::PROFESSIONALISM) < 0xf && a < same + 4 && read_i8_as_i32(m + staff::AMBITION) < same + 8 && friendly_random(a) < friendly_random(6) {
                    let parou = if category(m, c) == 1 || category(m, c) == 2 {
                        apply_event_flags_or_stop(c, 0x80000000, p)
                    } else {
                        set_contract_flags_or_stop(c, 0x200)
                    };
                    if parou {
                        return;
                    }
                }
            }
        }
        if read_i32(c + contract::GRIEVANCES) & (grievances::STAR_TEAMMATE_SOLD as i32) != 0 {
            let rp = current_reputation(p);
            if rp > 0x1482 || rp as i32 - 0x4e2 > new.reputation() as i32 {
                changed = true;
                if strength > 0x28 || friendly_random(read_i8_as_i32(m + staff::PRESSURE)) != 0 {
                    covered!("cqa_13 complaint cleared");
                    if clear_grievances_or_stop(c, 0x20000000) { // cqa13: someone bigger arrives
                        return;
                    }
                }
            }
        }
    }
    let sheet = p.player();
    if sheet.world_reputation() > 0x186a {
        let rp = sheet.current_reputation();
        if rp as i32 + 0x2ee > new.reputation() as i32 || rp > 0x222e {
            let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(new.addr()) as u32).wrapping_mul(0x49) as usize);
            let x = read_i16(row + 8).wrapping_add(0x1f4); // cqa13: a torcida comemora
            let x = if x < 1 { 1 } else if x > 10000 { 10000 } else { x };
            covered!("cqa_13 goal_crowd_reaction celebrates");
            write_i16(row + 8, x);
            if signing_celebration_or_stop(this, p, new, 1, 0) {
                return;
            }
        }
    }
    if changed {
        let sheet = p.player();
        if sheet.world_reputation() > 0x186a
            && sheet.current_reputation() as i32 + 0x2ee > new.reputation() as i32
            && (p.professionalism() as i32) > 0xa
            && (p.loyalty() as i32) > 6
            && friendly_random(3) != 0
        {
            let who = squad_spokesman(new, p);
            if who != 0 && signing_celebration_or_stop(this, p, new, 2, who) {
                return;
            }
        }
    }
    // (C) the old club’s squad
    let mut changed = false;
    if old.is_null() || old.division().addr() == 0 {
        return;
    }
    if read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(old.division().addr()))) == 0 {
        return;
    }
    let strength = exe::database_count_club_squad(old, 0, 0) as i8 as i32;
    let nation = old.nation().addr();
    let sete = nation != 0 && [KEY_NATION_GREECE, KEY_NATION_SWEDEN, KEY_NATION_WALES, KEY_NATION_FINLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND, KEY_NATION_AUSTRALIA].iter().any(|&k| read_usize(nation) == read_usize(k));
    let rv = old.reputation();
    let (lim1, lim2) = if sete || rv < 0x1482 {
        (0x19, 0x2d)
    } else if rv < 0x1c52 {
        (0x1e, 0x2d)
    } else {
        (0x23, 0x2e)
    };
    let same = exe::club_count_squad_with_position_mask(old, natural_position_mask(p)) as i8 as i32;
    for j in 0..50usize {
        let m = old.squad(j);
        if m == 0 || read_usize(m + staff::PLAYER) == 0 || m == p.addr() {
            continue;
        }
        let c = media_contract_for_club(m, old);
        if c == 0 {
            continue;
        }
        let mut q: i32 = if read_i32(c + contract::GRIEVANCES) != 0 { 3 } else { 0 };
        if media_contract_flags(c) != 0 {
            q += 1;
        }
        if read_i8_as_i32(c + contract::MORALE) < -0x19 {
            q += 1;
        }
        let q = q as i8 as i32;
        let mut queixa = 0u32; // 0x40000000 or 0x20000000 depending on the teammate’s exit
        let mut second = true;
        if flag as u8 != 0
            && media_contract_flags(c) & 0x200 == 0
            && media_contract_flags(c) & 0x800 == 0
            && read_i32(c + contract::GRIEVANCES) as u32 & grievances::SAME_POSITION_SIGNING == 0
            && (strength < 0x28 || same < 6)
            && (natural_position_mask(p) != natural_position_mask(Staff(m)) || same < 2)
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > new.reputation().wrapping_sub(0x2ee) {
                let r = friendly_random(6);
                if read_i8_as_i32(m + staff::AMBITION) + q > r + 10 && read_i8_as_i32(c + contract::HAPPINESS) > 0x19 && current_reputation(Staff(m)) > 0x109a
                    && friendly_random(read_i8_as_i32(m + staff::PROFESSIONALISM)) < q + 5 && friendly_random(read_i8_as_i32(m + staff::PRESSURE)) == 0
                {
                    second = false;
                    queixa = if ((p.age() as i8) as i32) < 0x18 && p.player().world_reputation() < 0x128e { 0x40000000 } else { 0x20000000 };
                }
            }
        }
        if second
            && ((p.age() as i8) as i32) > 0x17
            && flag as u8 != 0
            && exe::staff_is_best_favoured_candidate(m, new, p) != 0
            && natural_position_mask(p) != natural_position_mask(Staff(m))
            && friendly_random(read_i8_as_i32(m + staff::PROFESSIONALISM)) < q + 5
        {
            let rp = current_reputation(p);
            if rp > 0x1e46 || rp > new.reputation().wrapping_sub(0x2ee) {
                queixa = 0x20000000;
            } else {
                covered!("cqa_13 morale drops");
                let x = read_i8_as_i32(c + contract::MORALE) - 0x19; // cqa13: the morale of the teammate drops
                write_i8(c + contract::MORALE, if x > 100 { 100 } else if x < -100 { -100 } else { x as i8 });
            }
        }
        if queixa != 0 {
            if apply_event_flags_or_stop(c, queixa, p) {
                return;
            }
            changed = true;
        }
        if read_i8_as_i32(m + staff::AGE) > 0x17 && media_contract_flags(c) & 0x400 == 0 && read_i8_as_i32(m + staff::AMBITION) > 0xf {
            let r = (friendly_random(10) as u8).wrapping_add(10) as i8;
            if read_i8(m + staff::DETERMINATION) > r && read_i8_as_i32(m + staff::PROFESSIONALISM) > 0xa && current_reputation(p) > 0x109a && current_reputation(Staff(m)) > 0x109a && strength < lim1 && set_contract_flags_or_stop(c, 0x400) {
                return;
            }
        }
        let equal = || natural_position_mask(p) == natural_position_mask(Staff(m));
        if (strength < lim2 || same < 2) && equal() && media_contract_flags(c) & 0x200 != 0 {
            clear_contract_flags(c, 0x200);
        }
        let remove_800 = if strength < lim2 {
            let r1 = friendly_random(p.pressure() as i32);
            (r1 > friendly_random(10) && equal()) || same < 2
        } else {
            same < 2
        };
        if remove_800 && media_contract_flags(c) & 0x800 != 0 {
            clear_contract_flags(c, 0x800);
        }
        if ((strength < lim2 && equal()) || same < 2) && read_i32(c + contract::GRIEVANCES) as u32 & grievances::SAME_POSITION_SIGNING != 0 && clear_grievances_or_stop(c, 0x80000000) {
            return;
        }
        if media_contract_flags(c) & 0x400 != 0 {
            if read_i32(c + contract::GRIEVANCES) as u32 & grievances::SAME_POSITION_SIGNING != 0 && clear_grievances_or_stop(c, 0x80000000) {
                return;
            }
            if media_contract_flags(c) & 0x800 != 0 {
                clear_contract_flags(c, 0x800);
            }
            if media_contract_flags(c) & 0x200 != 0 {
                clear_contract_flags(c, 0x200);
            }
        }
    }
    if !changed || ((p.age() as i8) as i32) >= 0x23 {
        return;
    }
    let sheet = p.player();
    if sheet.current_ability() > 0x64
        && sheet.world_reputation() > 0x186a
        && sheet.current_reputation() as i32 + 0x2ee > old.reputation() as i32
        && (p.professionalism() as i32) > 0xa
        && (p.loyalty() as i32) > 6
        && friendly_random(3) != 0
    {
        let who = squad_spokesman(old, p);
        if who != 0 {
            if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
                MEDIA_REACTION_STOPPED.store(true, Relaxed);
                return;
            }
            exe::media_queue_sale_unrest_story(this, old, new, p, who);
        }
    }
}

/// `Media::queue_signing_celebration_story(p, club, kind, a, porta_voz)` (SEH frame for the local news item), called by
/// the transfer reactions (`process_transfer_squad_reactions`): news item 0xbe5 of the fans or a squad spokesman
/// celebrating the signing, to the club manager.
///
/// A player with `.nation()`, a current club (`.club_job()`) with a manager (`.manager()`) and `.stadium()`, a player
/// sheet (`.player()`); type 2 without a spokesman: nothing. The news item 0xbe5: the player (id and the three keys),
/// his club's id, draw(2), the outlet (`exe::media_pick_outlet_type_for_person`), the pundit
/// (`exe::punditlist_pick_random_pundit` with the club, the outlet and the nation `.nation()` or −1), the position mask
/// (`exe::formation_get_position_mask`), the type, the computer manager's response
/// (`exe::media_pick_ai_manager_response` with 0x82; 0 with a human manager), for type 2 the spokesman (id and keys),
/// the club's reputation above 7250, the manager (id and keys) and `a`; it goes to the manager of the player's club
/// (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_signing_celebration_story` (0x75b6e0, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn queue_signing_celebration_story(this: usize, p: usize, club: usize, kind: u32, a: u32, porta_voz: usize) {
    let p = Staff(p);
    let club = Club(club);
    if p.is_null() || p.nation().is_null() {
        return;
    }
    let current = p.club_job();
    if current.is_null() || current.manager().addr() == 0 || current.stadium().addr() == 0 || p.player().is_null() {
        return;
    }
    if porta_voz == 0 && kind as u8 == 2 {
        return;
    }
    covered!("cqa_13 celebration news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbe5, 0);
    exe::newsitem_set_slot(nt, 0, read_usize(p.addr()) as u32);
    for k in 1..4usize {
        exe::newsitem_set_slot(nt, k as i8, read_usize(read_usize(p.addr() + staff::FIRST_NAME * k) + names::ID) as u32);
    }
    exe::newsitem_set_slot(nt, 4, read_usize(p.club_job().addr()) as u32);
    exe::newsitem_set_slot(nt, 5, friendly_random(2) as u32);
    let v = exe::media_pick_outlet_type_for_person(p);
    exe::newsitem_set_slot(nt, 6, v);
    let nation = club.nation().addr();
    exe::newsitem_set_slot(nt, 7, exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, v, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    exe::newsitem_set_slot(nt, 8, exe::formation_get_position_mask(p.player()) as i16 as i32 as u32);
    exe::newsitem_set_slot(nt, 9, kind as u8 as i8 as i32 as u32);
    let mut r = 0u32;
    if exe::human_manager_is_human_club(p.club_job(), 0, 1, 0, 0) == 0 {
        r = exe::media_pick_ai_manager_response(this, p.club_job().manager().addr(), p, 0x82);
    }
    exe::newsitem_set_slot(nt, 10, r);
    let mut k: i8 = 0xb;
    if kind as u8 == 2 {
        exe::newsitem_set_slot(nt, 0xb, read_usize(porta_voz) as u32);
        for j in 1..4usize {
            exe::newsitem_set_slot(nt, 0xb + j as i8, read_usize(read_usize(porta_voz + 4 * j) + names::ID) as u32);
        }
        k = 0xf;
    }
    exe::newsitem_set_slot(nt, k, (club.reputation() > 0x1c52) as u32);
    let t = p.club_job().manager().addr();
    exe::newsitem_set_slot(nt, k + 1, read_usize(t) as u32);
    for j in 1..4usize {
        exe::newsitem_set_slot(nt, k + 1 + j as i8, read_usize(read_usize(t + 4 * j) + names::ID) as u32);
    }
    // simplification: the original pushes the index in a dword with uninitialized upper bytes; 0x785790 reads the char
    exe::newsitem_set_slot(nt, k + 5, a as u8 as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, p.club_job());
}

// simplification: the news item goes to the manager’s inbox (allocates) and draws: original only; the copy carries the branch.

/// `Media::queue_sale_unrest_story(old, new, p, teammate)` (SEH frame for the local news item), called by the transfer
/// reactions (`process_transfer_squad_reactions`): news item 0xbe4 of the team mate unhappy at the sale, to the old
/// club's manager.
///
/// An old club with a manager (`.manager()`), not a national side (`is_national_team`), a team mate and a player, each
/// with a player sheet (`.player()`). The news item 0xbe4: the manager, the team mate and the player (id and the three
/// keys of each), the ids of the two clubs, draw(2) (+2 if the team mate's `.professionalism()` is not above draw(5) +
/// 10), the outlet (`exe::media_pick_outlet_type_for_club`), the pundit (`exe::punditlist_pick_random_pundit` with the
/// nation `.nation()` or −1) and 2; it goes to the old club's manager (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_sale_unrest_story` (0x75cfc0, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn queue_sale_unrest_story(_this: usize, old: usize, new: usize, p: usize, teammate: usize) {
    let p = Staff(p);
    let teammate = Staff(teammate);
    let old = Club(old);
    if old.is_null() || old.manager().addr() == 0 || is_national_team(old.addr()) != 0 {
        return;
    }
    if teammate.is_null() || teammate.player().is_null() || p.is_null() || p.player().is_null() {
        return;
    }
    covered!("cqa_13 unsettling news");
    let mut local = [0u32; 0x3a];
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0xbe4, 0);
    let mut k: i8 = 0;
    for quem in [old.manager().addr(), teammate.addr(), p.addr()] {
        exe::newsitem_set_slot(nt, k, read_usize(quem) as u32);
        for j in 1..4usize {
            exe::newsitem_set_slot(nt, k + j as i8, read_usize(read_usize(quem + 4 * j) + names::ID) as u32);
        }
        k += 4;
    }
    exe::newsitem_set_slot(nt, 0xc, read_usize(old.addr()) as u32);
    exe::newsitem_set_slot(nt, 0xd, read_usize(new) as u32);
    let r = friendly_random(5);
    let s = if (teammate.professionalism() as i32) > r + 10 { friendly_random(2) } else { friendly_random(2) + 2 };
    exe::newsitem_set_slot(nt, 0xe, s as u32);
    let v = exe::media_pick_outlet_type_for_club(old);
    exe::newsitem_set_slot(nt, 0xf, v);
    let nation = old.nation().addr();
    exe::newsitem_set_slot(nt, 0x10, exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), old, v, if nation != 0 { read_i32(nation) } else { -1 }) as u32);
    exe::newsitem_set_slot(nt, 0x11, 2);
    exe::news_send_to_club_manager(read_usize(G_NEWS), nt, old);
}

// simplification: the news item goes to the manager’s inbox (allocates) and draws: original only; the copy carries the branch.

/// True if the date (+4) of a regen queue entry is before today.
pub(crate) unsafe fn is_regen_entry_due(e: usize) -> bool {
    exe::date_operator_3c(e + 4, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 // cpw10: the entry is due
}

/// One rating of the rebuilt coach (`process_regen_queue`): max(pow(+0x22, 0x968950) x factor, record value v) x k, capped and
/// truncated.
pub(crate) unsafe fn rebuilt_coach_rating(person: Staff, v: i16, factor: Option<usize>, k: usize, ceiling: usize) -> u16 {
    let mut p = game_pow(person.int_apps() as f64, read_f64(rdata::F64_2));
    if let Some(f) = factor {
        p *= read_f64(f);
    }
    let v = v as f64;
    let m = if p > v { p } else { v };
    let mk = m * read_f64(k);
    let t = read_f64(ceiling);
    ftol(if !(mk >= t) { mk } else { t }) as u16
}

/// The post of the rebuilt coach at his AI club (0x7a6757..0x7a68d8): coach in empty coach slots or assistant, by his
/// reputation against the club's.
///
/// With the player's reputation (+0xb of the player sheet) below the club's (+0x80) times 0x968968 and fewer than 5
/// coaches (`database_count_club_coaches`): he takes the empty coach slots (+0x19f, 5) (post 0xf); with no slot, assistant (+0xd3, post 0xd)
/// if the club has none (`database_get_club_assistant`).
///
/// Otherwise, below the club's times 0x969040, assistant in the same way.
pub(crate) unsafe fn place_coach_at_ai_club(s: Staff) {
    let club = s.club_job();
    if club.is_null() {
        return;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        return;
    }
    let rep = |k: usize| {
        let f = s.player().current_reputation() as f64;
        let c = s.club_job().reputation() as f64 * read_f64(k);
        c > f
    };
    if rep(0x968968) && (exe::database_count_club_coaches(s.club_job()) as i8) < 5 {
        for k in 0..5usize {
            let v = s.club_job().addr() + club::COACHES + 4 * k;
            if read_usize(v) == 0 {
                write_usize(v, s.addr());
                s.set_job_for_club(job_kind::PLAYER_COACH);
            }
        }
        if s.job_for_club() == job_kind::PLAYER_COACH || exe::database_get_club_assistant(s.club_job()) != 0 {
            return;
        }
        s.club_job().set_assistant_manager(s.addr());
        s.set_job_for_club(job_kind::PLAYER_ASSISTANT_MANAGER);
        return;
    }
    if !rep(0x969040) || exe::database_get_club_assistant(s.club_job()) != 0 {
        return;
    }
    s.club_job().set_assistant_manager(s.addr());
    s.set_job_for_club(job_kind::PLAYER_ASSISTANT_MANAGER);
}

/// Media outlet code for big clubs from a random(100) value (6, 3, 5, 2, 0 or 1) [inference: "veiculo" = media outlet].
pub(crate) fn big_club_outlet(al: i8) -> u32 {
    if al == 0 {
        6
    } else if al < 8 {
        3
    } else if al < 0xf {
        5
    } else if al < 0x28 {
        2
    } else {
        (al >= 0x3c) as u32
    }
}

/// Media outlet code for smaller clubs: random(4) 0..2 gives 2, else 4 [inference].
pub(crate) unsafe fn small_club_outlet() -> u32 {
    let r = friendly_random(4);
    if (0..=2).contains(&r) {
        2
    } else {
        4
    }
}

/// club reputation above 6250: outlet 0 to 6 from random(100); otherwise 2 or 4 from random(4)
/// Original: `media_pick_outlet_type_for_club` (0x757f30, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_pick_outlet_type_for_club(club: usize) -> u32 {
    let club = Club(club);
    if club.reputation() > 0x186a {
        big_club_outlet(friendly_random(100) as u8 as i8)
    } else {
        small_club_outlet()
    }
}

/// `media_pick_outlet_type_for_person(person)` (cdecl): picks the media outlet type like the club version does, but
/// from the player's reputation (`.current_reputation()` of the player sheet `.player()`).
///
/// Used by the player's stories (`exe::media_create_player_media_story`): the outburst, the praise, wanting to play,
/// and the signing celebration (`queue_signing_celebration_story`, in transfers).
/// Original: `media_pick_outlet_type_for_person` (0x757fa0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_pick_outlet_type_for_person(person: usize) -> u32 {
    let person = Staff(person);
    if person.player().current_reputation() > 0x186a {
        big_club_outlet(friendly_random(100) as u8 as i8)
    } else {
        small_club_outlet()
    }
}

/// The club's row in the manager manager table ([[G_MANAGER_MANAGER]] + id x 0x49; +6 board confidence, +8 fan
/// confidence).
pub(crate) unsafe fn manager_manager_row(club: Club) -> usize {
    read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(0x49))
}

/// Dampens a confidence change: halved drop / doubled rise for a favoured manager, the reverse for a disliked one;
/// beyond +-1000 the excess counts 0.2.
pub(crate) unsafe fn damp_confidence_change(v: i32, gosta: bool, not_likes: bool) -> i16 {
    let mut v = v;
    if gosta {
        let x = v as i16 as f64;
        v = if (v as i16) < 0 { ftol(x * read_f64(rdata::F64_0_5)) } else { ftol(x + x) };
    } else if not_likes {
        let x = v as i16 as f64;
        v = if (v as i16) > 0 { ftol(x * read_f64(rdata::F64_0_5)) } else { ftol(x + x) };
    }
    let x = v as i16 as i32;
    if x < -1000 {
        v = ftol((x + 1000) as f64 * read_f64(rdata::F64_0_2) - read_f64(rdata::F64_1000));
    } else if x > 1000 {
        v = ftol((x - 1000) as f64 * read_f64(rdata::F64_0_2) + read_f64(rdata::F64_1000));
    }
    v as i16
}

/// The change in the fans' confidence (`adjust_supporter_confidence`) toward a target.
///
/// conf = the row's +8 (0 without a club); d = target − trunc((conf − 5000) × −0.05) (16 bits); v = trunc(d × 0.75);
/// the manager (+0xcf) being favoured (`favourite_of`) or disliked (`rejected_by`) by the club dampens it. Both lookups are pure.
pub(crate) unsafe fn fan_confidence_change(club: Club, target: u32) -> i16 {
    let conf = if !club.is_null() { read_i16(manager_manager_row(club) + 8) as i32 } else { 0 };
    let t = ftol((conf - 5000) as f64 * read_f64(rdata::F64_NEG_0_05));
    let d = (target as i32).wrapping_sub(t) as i16;
    let v = ftol(d as f64 * read_f64(rdata::F64_0_75));
    let manager = club.manager().addr();
    let fav = exe::club_favours_staff(manager, club) != 0;
    let rej = !fav && exe::club_dislikes_staff(manager, club) != 0;
    damp_confidence_change(v, fav, rej)
}

/// Board confidence change (`adjust_board_confidence`) toward a target, from the row's +6 and the chairman liking or disliking the
/// manager.
pub(crate) unsafe fn board_confidence_change(club: Club, target: u32) -> i16 {
    let conf = if !club.is_null() { read_i16(manager_manager_row(club) + 6) as i32 } else { 0 };
    let s = ((conf - 6000) / 20).wrapping_add(target as i32) as i16;
    let v = ftol(s as f64 * read_f64(rdata::F64_0_75));
    let (tecnico, presidente) = (club.manager().addr(), club.chairman().addr());
    let g = exe::staff_is_favoured_by(tecnico, presidente) != 0;
    let n = !g && exe::staff_is_disliked_by(tecnico, presidente) != 0;
    damp_confidence_change(v, g, n)
}

/// Adds v to a confidence word, clamped to 1..10000.
pub(crate) unsafe fn add_clamped_confidence(p: usize, v: i16) {
    let x = read_i16(p).wrapping_add(v);
    write_i16(p, x.clamp(1, 10000));
}

/// `Media::adjust_supporter_confidence(club, sujeito, topic, target, variant)` (SEH frame without a destructor): adds
/// the change to the supporters' confidence (+8 of the club's row in [[0xb67a44]], kept between 1 and 10000): the
/// target minus the current confidence, × 0.75, damped by a favoured or rejected manager and beyond ±1000 (see
/// `fan_confidence_change`).
///
/// Beyond ±500, news item 0xbe0 (local) to the club's manager (`exe::news_send_to_club_manager`): 0 = 0, 1 = topic, 2 =
/// variant, 3 = draw(4), 4 = club id, 5 = the degree (5 below −750, 4 negative, 3 up to 750, 2 above), 6..9 the
/// manager; with a `sujeito`, 10..13 the `sujeito` and 14 = 6 favoured, 7 rejected, 8 neither; then −1, the new
/// confidence and 0.
/// Original: `Media::adjust_supporter_confidence` (0x738340, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn adjust_supporter_confidence(_this: usize, club: usize, sujeito: usize, topic: u32, target: u32, variant: u32) {
    let club = Club(club);
    let v = fan_confidence_change(club, target);
    covered!("cpo_5 goal_crowd_reaction");
    if !club.is_null() {
        add_clamped_confidence(manager_manager_row(club) + 8, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe0, 0);
    write(n, 0, 0);
    write(n, 1, topic as i16 as i32 as u32);
    write(n, 2, variant as i8 as i32 as u32);
    let s = exe::utils_random(4);
    write(n, 3, s as u32);
    write(n, 4, read_usize(club.addr()) as u32);
    let grau = if v < -750 { 5 } else if v < 0 { 4 } else if v > 750 { 2 } else { 3 };
    write(n, 5, grau);
    set_person_name_slots(write, n, 6, club.manager().addr());
    let b: i8 = if sujeito != 0 {
        set_person_name_slots(write, n, 10, sujeito);
        let r = if exe::club_favours_staff(sujeito, club) != 0 {
            6
        } else if exe::club_dislikes_staff(sujeito, club) != 0 {
            7
        } else {
            8
        };
        write(n, 14, r);
        0xf
    } else {
        write(n, 10, u32::MAX);
        0xb
    };
    write(n, b, u32::MAX);
    let conf = if !club.is_null() { read_i16(manager_manager_row(club) + 8) as i32 } else { 0 };
    write(n, b + 1, conf as u32);
    write(n, b + 2, 0);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// `Media::adjust_board_confidence(club, sujeito, topic, target, variant, sexto)` (SEH frame without a destructor):
/// adds the change to the board's confidence (+6 of the club's row, kept between 1 and 10000): (confidence − 6000)/20 +
/// target, × 0.75, damped by a chairman who likes or dislikes the manager and beyond ±1000 (see
/// `board_confidence_change`).
///
/// Beyond ±500, news item 0xbe0 (local) to the club's manager (`exe::news_send_to_club_manager`): 0 = 1, 1 = topic, 2 =
/// variant, 3 = draw(4), 4 = club id, 5 = the degree (5 below −750 with confidence below 4500; 9 when negative with
/// confidence above 6250, otherwise 4; 2 above 750 with confidence above 6250, otherwise 3), 6..9 the manager; with a
/// `sujeito` 10..13 and 14 = 6/7/8 (the chairman likes, dislikes, neither); then the chairman (or −1), the new
/// confidence, the `sexto` (with it 1 and the word `board_record::EXPECTATION_FLAGS`), and the `sujeito`'s flags:
/// contract `.squad_status()` 0x10/0x20 above 25 years old, average rating above the league average × 5, `.loyalty()`
/// and `.professionalism()` above 14, more than 1500 days at the club, value (`FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE`)
/// above 4,500,000 and reputation (+5 of the sheet) above 140.
/// Original: `Media::adjust_board_confidence` (0x7386f0, media.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn adjust_board_confidence(_this: usize, club: usize, sujeito: usize, topic: u32, target: u32, variant: u32, sexto: u32) {
    let sujeito = Staff(sujeito);
    let club = Club(club);
    let v = board_confidence_change(club, target);
    covered!("cpo_5 board");
    if !club.is_null() {
        add_clamped_confidence(manager_manager_row(club) + 6, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    covered!("cpo_5 board news");
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let conf = || if !club.is_null() { read_i16(manager_manager_row(club) + 6) } else { 0 };
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbe0, 0);
    write(n, 0, 1);
    write(n, 1, topic as i16 as i32 as u32);
    write(n, 2, variant as i8 as i32 as u32);
    let s = exe::utils_random(4);
    write(n, 3, s as u32);
    write(n, 4, read_usize(club.addr()) as u32);
    let grau = if v < -750 && (club.is_null() || conf() < 4500) {
        5
    } else if v < 0 {
        if !club.is_null() && conf() > 6250 { 9 } else { 4 }
    } else if v > 750 && !club.is_null() && conf() > 6250 {
        2
    } else {
        3
    };
    write(n, 5, grau);
    set_person_name_slots(write, n, 6, club.manager().addr());
    let mut b: i8 = if !sujeito.is_null() {
        set_person_name_slots(write, n, 10, sujeito.addr());
        let chairman = club.chairman().addr();
        let r = if exe::staff_is_favoured_by(sujeito, chairman) != 0 {
            6
        } else if exe::staff_is_disliked_by(sujeito, chairman) != 0 {
            7
        } else {
            8
        };
        write(n, 14, r);
        0xf
    } else {
        write(n, 10, u32::MAX);
        0xb
    };
    let chairman = club.chairman().addr();
    if chairman != 0 {
        set_person_name_slots(write, n, b, chairman);
        b += 4;
    } else {
        write(n, b, u32::MAX);
        b += 1;
    }
    write(n, b, conf() as i32 as u32);
    b += 1;
    if sexto != 0 {
        write(n, b, 1);
        let w = if !club.is_null() { read_i16(manager_manager_row(club) + board_record::EXPECTATION_FLAGS) } else { 0 };
        write(n, b + 1, w as i32 as u32);
        b += 2;
    } else {
        write(n, b, 0);
        b += 1;
    }
    // the subject’s marks
    let mut m = 0u32;
    if !sujeito.is_null() && !sujeito.player().is_null() {
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), sujeito));
        if !c.is_null() && ((sujeito.age() as i8) as i32) > 0x19 {
            let t = c.squad_status() & 0xf0;
            if t == 0x20 || t == 0x10 {
                m = 1;
            }
        }
    }
    write(n, b, m);
    b += 1;
    let mut m = 0u32;
    if !sujeito.is_null() && !sujeito.player().is_null() {
        let base = read_usize(G_PLAYER_STATS);
        let notes = (exe::playerstats_get_recent_form(base, sujeito.player()) as i32) as i8 as f64;
        let base = read_usize(G_PLAYER_STATS);
        let y = read_f64(base + player_stats_fields::RATED_COUNT);
        let c0 = read_f64(rdata::F64_0);
        let average = if !(y < c0 || y > c0) { read_f64(rdata::F64_6_8) } else { read_f64(base + player_stats_fields::RATING_SUM) / y }; // no matches: 6.8;
        if notes > average * read_f64(rdata::F64_5) {
            m = 1;
        }
    }
    write(n, b, m);
    b += 1;
    write(n, b, (!sujeito.is_null() && (sujeito.loyalty() as i32) > 0xe) as u32);
    b += 1;
    write(n, b, (!sujeito.is_null() && (sujeito.professionalism() as i32) > 0xe) as u32);
    b += 1;
    let days = !sujeito.is_null() && exe::date_days_since(G_CURRENT_DATE, sujeito.date_joined_club() as u32, sujeito.date_joined_club_leap_year() as u32) > 0x5dc;
    write(n, b, days as u32);
    b += 1;
    let caro = !sujeito.is_null() && value(read_usize(G_CONTRACT_MANAGER), sujeito.addr(), club.addr(), 0, u32::MAX, 0) > 0x44aa20;
    write(n, b, caro as u32);
    b += 1;
    let injury_word_0c = !sujeito.is_null() && !sujeito.player().is_null() && read_i16(sujeito.player().addr() + 5) > 0x8c;
    write(n, b, injury_word_0c as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

pub(crate) type NewsWriteFn = unsafe extern "thiscall" fn(usize, i8, u32) -> u32;

/// Writes a person's id and three keys (+0x33 of +4, +8, +0xc) into news slots from k; the +0xc one may be missing (-1)
/// when optional.
pub(crate) unsafe fn set_person_slots_optional(write: NewsWriteFn, n: usize, k: i8, p: Staff, opcional: bool) {
    write(n, k, read_usize(p.addr()) as u32);
    write(n, k + 1, read_usize(p.first_name() + names::ID) as u32);
    write(n, k + 2, read_usize(p.second_name() + names::ID) as u32);
    let q = p.common_name();
    write(n, k + 3, if opcional && q == 0 { u32::MAX } else { read_usize(q + names::ID) as u32 });
}

/// Returns the NEWSITEM constructor (`construct_news_item`) and set_slot (`set_news_field`) function pointers.
pub(crate) unsafe fn news_item_fns() -> (unsafe extern "thiscall" fn(usize, u32, u32) -> usize, NewsWriteFn) {
    (core::mem::transmute(FN_NEWSITEM_NEWSITEM), core::mem::transmute(FN_NEWSITEM_SET_SLOT))
}

/// Days from the person's date at +0x3e/+0x42 to today (Date days_since).
pub(crate) unsafe fn days_since_arrival(p: Staff) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32)
}

/// O sorteio da gratidão pelo tipo (índices da tabela de bytes 0x732fcc, saltos 0x732fb4): 0 os tipos 0, 6, 14 e 17; 1
/// os 2 e 30 (sorteio(2)); 2 os 4, 9, 33, 34 e 37 (sorteio(3)); 3 os 5 e 7 (sorteio(4)); o resto 0.
pub(crate) const GRATITUDE_DRAW_CASE: [u8; 0x26] = [0, 5, 1, 5, 2, 3, 0, 3, 5, 2, 5, 5, 5, 4, 0, 5, 5, 0, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 2, 2, 4, 5, 2];

/// `Media::queue_player_gratitude_news(player, club, kind, variant)` (SEH frame without a destructor): posts news item
/// 0xbda of the player's gratitude to the manager.
///
/// With a player with a sheet (`.player()`), a club with a manager (`.manager()`) and his contract with the club
/// (`exe::contract_manager_get_contract_for_club`), news item 0xbda (local) to the manager
/// (`exe::news_send_to_club_manager`): 0..3 the player (3 = −1 without +0xc), 4 = club id, 5 = the type, 6 = the
/// variant, 7 = the type's draw, 8..0xb the manager, 0xc = 0x420e9, 0xd..0x10 the assistant (`.assistant_manager()`) or
/// −1. The type: if the player is on the media list (this +0/`media_fields::STORY_LOG_COUNT`, 0x17 bytes, id at
/// `story_log_entry::PERSON_A` and flag 2 at `story_log_entry::RESPONSE`) or the contract has 0x2000 or 8 in
/// `contract::GRIEVANCES`, types 0, 2, 4, 7, 9, 0xe and 0x23 become 0x12 (`.temperament()` ≥ draw(10)) or 0x21; injured
/// (`exe::injury_get_player_injury`): 0xc. For types 0, 6, 14 and 17: with the sheet's `.work_rate()` below 10, draw(3)
/// + 2; otherwise draw(5), and 0 or 1 raises the sheet's `player_sheet::WORK_RATE` (up to 20).
/// Original: `Media::queue_player_gratitude_news` (0x732c80, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn queue_player_gratitude_news(this: usize, player: usize, club: usize, kind: u32, variant: u32) {
    let player = Staff(player);
    let club = Club(club);
    if player.is_null() || player.player().is_null() || club.is_null() || club.manager().addr() == 0 {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return;
    }
    covered!("cpo_7 gratitude");
    let (construir, gravar) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbda, 0);
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(club.addr()) as u32);
    // the media list (this +0, count at +8)
    let mut in_list = false;
    for i in 0..read_i32(this + media_fields::STORY_LOG_COUNT).max(0) as usize {
        let e = read_usize(this) + i * 0x17;
        if read_usize(e + story_log_entry::PERSON_A) == read_usize(player.addr()) && *((e + story_log_entry::RESPONSE) as *const u8) == 2 {
            in_list = true;
            break;
        }
    }
    let f45 = contract.grievances() as usize;
    let mut t = kind;
    if in_list || f45 & 0x2000 != 0 || f45 & 8 != 0 {
        if [0xe, 0, 2, 9, 0x23, 4, 7].contains(&(kind as i16)) {
            covered!("cpo_7 gratitude type");
            let s = exe::utils_random(10);
            t = if (player.temperament() as i32) >= s { 0x12 } else { 0x21 };
        }
    }
    if (exe::injury_get_player_injury(read_usize(G_INJURY), player) as u32) != 0 {
        t = 0xc;
    }
    let t = t as i16 as i32;
    gravar(n, 5, t as u32);
    gravar(n, 6, variant as i8 as i32 as u32);
    let v = match if (t as u32) <= 0x25 { GRATITUDE_DRAW_CASE[t as usize] } else { 5 } {
        0 => {
            let sheet = player.player();
            if !sheet.is_null() && (sheet.work_rate() as i32) < 10 {
                (exe::utils_random(3) as u8).wrapping_add(2) as i8 as i32
            } else {
                let s = exe::utils_random(5);
                if s == 0 || s == 1 {
                    let sheet = player.player();
                    if !sheet.is_null() && (sheet.work_rate() as i32) < 0x14 {
                        covered!("cpo_7 gratitude rises");
                        *((sheet.addr() + player_sheet::WORK_RATE) as *mut u8) += 1;
                    }
                }
                s
            }
        }
        1 => exe::utils_random(2),
        2 => {
            covered!("cpo_7 gratitude draw 3");
            exe::utils_random(3)
        }
        3 => exe::utils_random(4),
        _ => 0,
    };
    gravar(n, 7, v as u32);
    let manager = club.manager().addr();
    set_person_slots_optional(gravar, n, 8, Staff(manager), false);
    gravar(n, 0xc, 0x420e9);
    let auxiliar = club.assistant_manager();
    if auxiliar != 0 {
        set_person_slots_optional(gravar, n, 0xd, Staff(auxiliar), false);
    } else {
        gravar(n, 0xd, u32::MAX);
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// `Media::post_player_backing_request_news(player, club, kind, quarto)` (SEH frame without a destructor): for a human
/// club, posts news item 0xbcf of the player asking for backing to the manager.
///
/// With a player with a sheet (`.player()`) and a human club (`exe::human_manager_is_human_club`), news item 0xbcf
/// (local) to the club's manager (`exe::news_send_to_club_manager`): 0..3 the player (3 = −1 without +0xc), 4 = club
/// id, 5/6 = (1, ·) with reputation (`.current_reputation()` of the sheet) above 4250, otherwise (2, ·): the second is
/// 1 if (`.ambition()` > 15 and `.temperament()` < 6, or `.professionalism()` < 10 and `.loyalty()` < 6 in the lower
/// band), 3 if the type is 0x20, 1 or 0x1e with `.loyalty()` > 15, `.professionalism()` > 10 and more than 210 days at
/// the club, otherwise 2; 7 = type, 8 = `quarto`, 9 = transfer listed (`exe::staff_is_transfer_listed_or_requested`).
/// Original: `Media::post_player_backing_request_news` (0x743370, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_player_backing_request_news(_this: usize, player: usize, club: usize, kind: u32, quarto: u32) {
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || club == 0 {
        return;
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        return;
    }
    covered!("cpo_7 request for support");
    let (construir, gravar) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbcf, 0);
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(club) as u32);
    let t = kind as u8;
    let target = t == 0x20 || t == 1 || t == 0x1e;
    let (a, b): (i32, i32) = if player.player().current_reputation() > 0x109a {
        if (player.ambition() as i32) > 0xf && (player.temperament() as i32) < 6 {
            (1, 1)
        } else if target && (player.loyalty() as i32) > 0xf && (player.professionalism() as i32) > 0xa && days_since_arrival(player) > 0xd2 {
            (1, 3)
        } else {
            (1, 2)
        }
    } else if (player.professionalism() as i32) < 0xa && (player.loyalty() as i32) < 6 {
        (2, 1)
    } else if target && (player.loyalty() as i32) > 0xf && (player.professionalism() as i32) > 0xa && days_since_arrival(player) > 0xd2 {
        (2, 3)
    } else {
        (2, 2)
    };
    gravar(n, 5, a as u32);
    gravar(n, 6, b as u32);
    gravar(n, 7, t as i8 as i32 as u32);
    gravar(n, 8, quarto as i16 as i32 as u32);
    gravar(n, 9, (exe::staff_is_transfer_listed_or_requested(player, 0) != 0) as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// `Media::post_player_criticism_reaction_news(player, club, topic, reply)` (SEH frame without a destructor): posts
/// news item 0xbdc of the player's reaction to criticism, delivered to the managers of the interested clubs.
///
/// With a player with a sheet, a club with a manager and the contract (`exe::contract_manager_get_contract_for_club`),
/// news item 0xbdc (local): 0..3 the player, 4 = club id, 5 = the reaction (with reputation above 4250 and above the
/// club's, category `exe::contract_manager_get_deserved_squad_status` 1 or 2 and more than 22 years old: 1 or 3 if
/// `.loyalty()` < 10, `.ambition()` > 15 and `.temperament()` < 6; 2 or 4 with more than 1000 days, `.loyalty()` > 15
/// and without 0x181 in the contract's `contract::GRIEVANCES`; the higher one when the contract has 0x2000 or 8,
/// `.professionalism()` < 10, `.temperament()` > 10, `.loyalty()` < 15, more than 23 years old and the `reply` is 2;
/// otherwise 0), 6 = `reply`, 7 = board confidence below 4000, 8 = topic, 9..0xc the manager. It goes to the managers
/// of the clubs that `exe::transfer_manager_collect_player_news_recipients` ([0xde1f54], 0x21a, up to 30) returns.
/// Original: `Media::post_player_criticism_reaction_news` (0x743a50, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_player_criticism_reaction_news(_this: usize, player: usize, club: usize, topic: u32, reply: u32) {
    let player = Staff(player);
    let club = Club(club);
    if player.is_null() || player.player().is_null() || club.is_null() || club.manager().addr() == 0 {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return;
    }
    covered!("cpo_7 reaction to criticism");
    let (construir, gravar) = news_item_fns();
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdc, 0);
    let f45 = contract.grievances() as usize;
    let mark = (f45 & 0x2000 != 0 || f45 & 8 != 0)
        && (player.professionalism() as i32) < 0xa
        && (player.temperament() as i32) > 0xa
        && (player.loyalty() as i32) < 0xf
        && ((player.age() as i8) as i32) > 0x17;
    set_person_slots_optional(gravar, n, 0, player, true);
    gravar(n, 4, read_usize(club.addr()) as u32);
    let r = reply as u8;
    let rep = player.player().current_reputation();
    let mut reaction = 0i8;
    if rep > 0x109a && rep > club.reputation() {
        let manager = read_usize(G_CONTRACT_MANAGER);
        let cat_ok = exe::contract_manager_get_deserved_squad_status(manager, player, contract) == 1 || exe::contract_manager_get_deserved_squad_status(manager, player, contract) == 2;
        if cat_ok && ((player.age() as i8) as i32) > 0x16 {
            if (player.loyalty() as i32) < 0xa && (player.ambition() as i32) > 0xf && (player.temperament() as i32) < 6 {
                reaction = if mark && r == 2 { 3 } else { 1 };
            } else if days_since_arrival(player) > 0x3e8 && (player.loyalty() as i32) > 0xf && (contract.grievances() as usize) & 0x181 == 0 {
                reaction = if mark && r == 2 { 4 } else { 2 };
            }
        }
    }
    gravar(n, 5, reaction as i32 as u32);
    gravar(n, 6, r as i8 as i32 as u32);
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_usize(club.addr()).wrapping_mul(0x49));
    gravar(n, 7, (read_i16(row + 6) < 0xfa0) as u32); // board confidence below 4000
    gravar(n, 8, topic);
    set_person_slots_optional(gravar, n, 9, club.manager(), false);
    let mut clubs = [0usize; 0x1e];
    let tm = read_usize(G_TRANSFER_MANAGER);
    let k = if tm != 0 { exe::transfer_manager_collect_player_news_recipients(tm, player, 0x21a, clubs.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for i in 0..k.max(0) as usize {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, clubs[i]);
    }
}

/// A notícia guardada na caixa: 0xdf bytes (+0xd6 id).
pub(crate) const MEDIA_NEWS_ITEM_SIZE: usize = 0xdf;

/// Player reputation (min 2000) over club reputation (clamped 2000..7500).
pub(crate) unsafe fn player_club_reputation_ratio(player: Staff, club: Club) -> f64 {
    let a = player.player().current_reputation();
    let a = if a < 2000 { 2000 } else { a as i32 };
    let b = club.reputation();
    let b = if b < 2000 { 2000 } else if b > 7500 { 7500 } else { b as i32 };
    a as f64 / b as f64
}

/// Maps a story type (0..0x52) to the original's switch case (byte table at 0x736b8c; 12 = rating unchanged).
pub(crate) fn story_type_case(kind: i16) -> u8 {
    match kind {
        0 => 0,
        1 | 2 | 9 => 1,
        3 => 2,
        4 => 3,
        6 => 4,
        7 => 5,
        12 => 6,
        13 => 7,
        17 => 8,
        30 => 9,
        35 => 10,
        82 => 11,
        _ => 12,
    }
}

/// `Media::rate_player_form_for_story(player, club, contract, kind, mode, weight)`: the player's rating (x87, in st0)
/// for a media story kind.
///
/// Without a contract, the player's contract with the club (`exe::contract_manager_get_contract_for_club`); without
/// any, 0. Base: the league's average rating x (`G_PLAYER_STATS` `player_stats_fields::RATING_SUM` /
/// `player_stats_fields::RATED_COUNT`, without games 6.8), the player's statistics (`exe::playerstats_get_stat`,
/// category 6: s1 field 1, s2 field 0x11, s3 field 2, default 6.5) and the average of the ratings
/// (`exe::playerstats_get_recent_form`) × 0.2: r = (s2 − x) + (average − x) − weight/60; with s1 < 12, r × s1/12; when
/// negative, × the ratio of the reputations. Then, by type (0..0x52), additions from the contract type
/// (`contract::SQUAD_STATUS` >> 4), the age, the days at the club (`exe::date_days_since`), attributes of the record
/// `.player()` and the weight; type 0x52 gives r × 0.75 + s3/max(s1, 1) − 0.4.
/// Original: `Media::rate_player_form_for_story` (0x736720, media.cpp, thiscall, ret 0x18, st0).
pub(crate) unsafe extern "thiscall" fn rate_player_form_for_story(_this: usize, player: usize, club: usize, contract: usize, kind: u32, mode: u32, weight: f32) -> f64 {
    let player = Staff(player);
    let mut contract = contract;
    if contract == 0 {
        contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club);
        if contract == 0 {
            return 0.0;
        }
    }
    let g = read_usize(G_PLAYER_STATS);
    let games = read_f64(g + player_stats_fields::RATED_COUNT);
    let x = if games == 0.0 || games.is_nan() { 6.8 } else { read_f64(g + player_stats_fields::RATING_SUM) / games };
    let xf = x as f32;
    let s1f = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 6, 1, 6.5) as f32;
    let s2f = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 6, 0x11, 6.5) as f32;
    let average = (exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), player.player()) as i8) as i32;
    let mut l0 = (average as f64 * 0.2) as f32;
    let y = if s1f as f64 > 1.0 { s1f as f64 } else { 1.0 }; // the matches, at least 1
    let s3 = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), player.player(), 6, 2, 6.5);
    let l2 = (s3 / y) as f32;
    l0 = (l0 as f64 - xf as f64) as f32;
    let dif = s2f as f64 - xf as f64;
    let dif_f = dif as f32;
    covered!("cpn_3 player rating");
    let mut r = dif + l0 as f64 - weight as f64 * (1.0f32 / 60.0) as f64; // the weight of the story
    if !(s1f as f64 >= 12.0) {
        covered!("cpn_3 few matches");
        r = r * s1f as f64 * (1.0f32 / 12.0) as f64; // fewer than 12 matches
    }
    if !(r >= 0.0) {
        covered!("cpn_3 negative rating");
        r = r * player_club_reputation_ratio(player, Club(club)); // the refusal by reputation
    }
    let rf = r as f32;
    let kind = kind as u16 as i16;
    if !(0..=0x52).contains(&kind) {
        return r;
    }
    let c = read_u8(contract + contract::SQUAD_STATUS) >> 4; // the contract type
    match story_type_case(kind) {
        0 => match c {
            1 => r - 0.5,
            2 => r - 0.35f32 as f64,
            3 => r - 0.2f32 as f64,
            _ => r + 0.25,
        },
        1 => {
            let d = exe::date_days_since(G_CURRENT_DATE, player.date_joined_club() as u32, player.date_joined_club_leap_year() as u32);
            (200i32.wrapping_sub(d)) as f64 * 0.1f32 as f64 + rf as f64
        }
        2 => {
            let r2 = r + dif_f as f64;
            match c {
                1 => r2 + 2.0,
                2 => r2 + 1.5,
                3 => r2 + 1.0,
                _ => r2,
            }
        }
        3 => r + (0x23 - (player.age() as i8) as i32) as f64 * (1.0f32 / 3.0) as f64,
        4 => r + (10 - player.player().dirtiness() as i32) as f64 * 0.2f32 as f64,
        5 => r + ((player.age() as i8) as i32 - 0x13) as f64 * (1.0f32 / 3.0) as f64,
        6 => r + (player.player().leadership() as i32 - 10) as f64 * (1.0f32 / 3.0) as f64,
        7 => r + l0 as f64,
        8 => {
            let r2 = if r > 0.0 { r / player_club_reputation_ratio(player, Club(club)) } else { r };
            match c {
                1 => r2 - 0.5,
                2 => r2 - 0.35f32 as f64,
                3 => r2 - 0.2f32 as f64,
                _ => r2,
            }
        }
        9 => r - weight as f64 * 0.1f32 as f64,
        10 => match c {
            0 => r - 1.0,
            1 => r + 2.0,
            2 => r + 1.5,
            3 => r + 1.0,
            7 => {
                if mode as u8 == 3 { 0.0 } else { r - 3.0 }
            }
            _ => r,
        },
        11 => r * 0.75 + l2 as f64 - 0.4f32 as f64,
        _ => r,
    }
}

/// `Media::write_financial_trouble_news_text(news, body, out)`: writes the headline or body of the
/// club-in-financial-trouble story, by severity, with the club name linked.
///
/// It reads fields 0 (the club), 1 (the severity) and 2 of the news item (`exe::newsitem_get_slot`), builds the text in
/// 0xde1f64 (`FN_LANGLIB_SPRINTF`) by severity (> 0x1a5e, > 0xcb2 in the body; > 0x1a5e in the headline) with the
/// club's name (`club::SHORT_NAME`) and gender (`.short_gender_name()`), puts it into `out` (`exe::text_set_string`)
/// and links the name to the club (`exe::text_add_link`, type 0x7d5).
/// Original: `Media::write_financial_trouble_news_text` (0x74af60, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_financial_trouble_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("cpn_3 financial crisis text");
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let severity = exe::newsitem_get_slot(news, 1) as i32;
    (exe::newsitem_get_slot(news, 2) as i32);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
    let name = club.addr() + club::SHORT_NAME;
    let gender = (club.short_gender_name() as i8) as i32;
    let template = if body != 0 {
        if severity > 0x1a5e {
            0xa436fc
        } else if severity > 0xcb2 {
            0xa43644
        } else {
            0xa43594
        }
    } else if severity > 0x1a5e {
        0xa43560 // "... financial mess"
    } else {
        0xa43528 // "Financial trouble at ..."
    };
    format(0xde1f64, template, gender, gender, name);
    exe::text_set_string(out, (0xde1f64) as usize as *const u8);
    exe::text_add_link(out, (name) as usize as *const u8, 0x7d5, club);
}

/// True if date a is before date b (year word unsigned, then day word signed).
pub(crate) unsafe fn media_date_before(a: usize, b: usize) -> bool {
    let (x, y) = (read_u16(a + 2), read_u16(b + 2));
    if x != y {
        return x < y;
    }
    (read_u16(a) as i16) < (read_u16(b) as i16)
}

/// The record's id (+0), or -1 when null.
pub(crate) unsafe fn media_id_or_minus_one(p: usize) -> usize {
    if p != 0 { read_usize(p) } else { usize::MAX }
}

/// Builds the 23-byte record `log_story` stores (today's date, two ids, response, subtype, type).
pub(crate) unsafe fn build_news_log_record(a: usize, b: usize, kind: u32, subtype: u32, reply: u32) -> [u8; 0x17] {
    let mut r = [0u8; 0x17];
    r[..8].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[8..0xc].copy_from_slice(&(media_id_or_minus_one(a) as u32).to_le_bytes());
    r[0xc..0x10].copy_from_slice(&(media_id_or_minus_one(b) as u32).to_le_bytes());
    r[0x10] = reply as u8;
    r[0x11..0x13].copy_from_slice(&(subtype as u8 as i8 as i16).to_le_bytes());
    r[0x13..].copy_from_slice(&kind.to_le_bytes());
    r
}

/// qsorts the 23-byte records with the comparer `compare_story_log_records`.
pub(crate) unsafe fn sort_news_log(base: usize, n: usize) {
    exe::crt_qsort(base, n as u32, 0x17, 0x757ee0);
}

/// `Media::log_story(a, b, kind, subtype, reply)`: appends a story to the story log and sorts it again.
///
/// Appends to the story log ([this], capacity `media_fields::STORY_LOG_CAPACITY`, count
/// `media_fields::STORY_LOG_COUNT`; 23-byte records) today's date (+0, `copy_record_date` from `G_CURRENT_DATE`), the
/// id of `a` (+8) and of `b` (`story_log_entry::PERSON_B`) (−1 without them), the `kind` (`story_log_entry::TYPE`), the
/// `subtype` (signed byte, as a word at `story_log_entry::SUBTYPE`) and the `reply` (`story_log_entry::RESPONSE`); the
/// count goes up and the log is re-sorted (`sort_news_log`: qsort `exe::crt_qsort` with `compare_story_log_records`).
/// Without a log, it allocates 20 (`exe::crt_malloc`; without memory, box 0x6e11 and the end); when full, it grows by
/// 20 (realloc `exe::crt_realloc`; box 0x6e1c).
/// Original: `Media::log_story` (0x757a50, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn log_story(this: usize, a: usize, b: usize, kind: u32, subtype: u32, reply: u32) {
    covered!("cpl_6 record story");
    if read_usize(this) == 0 {
        let p = exe::crt_malloc(0x1cc);
        write_usize(this, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6e11);
            fatal_exit();
        }
        write_usize(this + media_fields::STORY_LOG_CAPACITY, 0x14);
    }
    let cap = read_usize(this + media_fields::STORY_LOG_CAPACITY);
    if cap == read_usize(this + media_fields::STORY_LOG_COUNT) {
        let p = exe::crt_realloc(read_usize(this), cap.wrapping_add(0x14).wrapping_mul(0x17) as u32);
        write_usize(this, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6e1c);
            fatal_exit();
        }
        write_usize(this + media_fields::STORY_LOG_CAPACITY, read_usize(this + media_fields::STORY_LOG_CAPACITY).wrapping_add(0x14));
    }
    let item = || read_usize(this).wrapping_add(read_usize(this + media_fields::STORY_LOG_COUNT).wrapping_mul(0x17));
    write_usize(item() + 8, media_id_or_minus_one(a));
    write_usize(item() + story_log_entry::PERSON_B, media_id_or_minus_one(b));
    copy_record_date(item(), G_CURRENT_DATE);
    write_usize(item() + story_log_entry::TYPE, kind as usize);
    write_i16(item() + story_log_entry::SUBTYPE, subtype as u8 as i8 as i16);
    write_u8(item() + story_log_entry::RESPONSE, reply as u8);
    let n = read_usize(this + media_fields::STORY_LOG_COUNT).wrapping_add(1);
    write_usize(this + media_fields::STORY_LOG_COUNT, n); // one more story
    sort_news_log(read_usize(this), n);
}

/// `Media::expire_story_log()`: removes the stories older than the limit, moving the last one into the gap, then sorts
/// again.
///
/// Each story in the log older than the limit (today − 100 days; the team one, `story_log_entry::TYPE` 0xbdf, today −
/// (40 + draw(40)), one draw `exe::utils_random` per such story on each pass; dates from `exe::date_sub_days` over
/// `G_CURRENT_DATE`) is overwritten by the last one (23 bytes), the count (`media_fields::STORY_LOG_COUNT`) goes down
/// and the same place is looked at again. If any are left, it re-sorts (`sort_news_log`: qsort with
/// `compare_story_log_records`). Without a log, nothing.
/// Original: `Media::expire_story_log` (0x757c70, media.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn expire_story_log(this: usize) {
    if read_usize(this) == 0 {
        return;
    }
    let mut i = 0i32;
    while i < read_usize(this + media_fields::STORY_LOG_COUNT) as i32 {
        let r = read_usize(this).wrapping_add((i as usize).wrapping_mul(0x17));
        let days = if read_usize(r + story_log_entry::TYPE) == 0xbdf {
            covered!("cpl_6 expire team story");
            let s = exe::utils_random(0x28); // the team history roll
            (s + 0x28) as u32
        } else {
            100 // the days of the other stories
        };
        let mut local = [0u8; 8];
        let limit = exe::date_sub_days(G_CURRENT_DATE, (&mut local) as *mut [u8; 8] as usize, days);
        if media_date_before(r, limit) {
            let n = read_usize(this + media_fields::STORY_LOG_COUNT);
            let last = read_usize(this).wrapping_add(n.wrapping_mul(0x17)).wrapping_sub(0x17);
            core::ptr::copy(last as *const u8, r as *mut u8, 0x17);
            write_usize(this + media_fields::STORY_LOG_COUNT, n.wrapping_sub(1));
            i -= 1; // the same place again
        }
        i += 1;
    }
    if read_usize(this + media_fields::STORY_LOG_COUNT) != 0 {
        sort_news_log(read_usize(this), read_usize(this + media_fields::STORY_LOG_COUNT));
    }
}

/// `Media::has_logged_story(a, b, kind, subtype, desde)`: tells whether a story is in the log.
///
/// The team story (type 0xbdf): a linear search for one of `story_log_entry::TYPE` 0xbdf with the id of `b`
/// (`story_log_entry::PERSON_B`) and the `subtype` (word `story_log_entry::SUBTYPE`), without looking at `a` or the
/// date: 1 if found. The others: a binary search (bsearch `exe::crt_bsearch` with `compare_story_log_records`) by the
/// key {id of `a` or −1, id of `b`, kind, subtype} (the default date `exe::date_date_54a110` in the key, which the
/// comparison does not read); found and without `desde`, 1; with `desde`, 1 if the story is not older than it.
/// Otherwise 0. `b` cannot be null (the original reads it without checking).
/// Original: `Media::has_logged_story` (0x757da0, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn has_logged_story(this: usize, a: usize, b: usize, kind: u32, subtype: u32, desde: usize) -> u32 {
    let mut key = [0u8; 0x17];
    exe::date_date_54a110(key.as_mut_ptr() as usize);
    let s = subtype as u16;
    if kind == 0xbdf {
        covered!("cpl_6 team story already told");
        let n = read_usize(this + media_fields::STORY_LOG_COUNT) as i32;
        for k in 0..n.max(0) as usize {
            let r = read_usize(this).wrapping_add(k * 0x17);
            if read_usize(r + story_log_entry::TYPE) == 0xbdf && read_usize(r + story_log_entry::PERSON_B) == read_usize(b) && read_u16(r + story_log_entry::SUBTYPE) == s {
                return 1; // the team’s story already told
            }
        }
        return 0;
    }
    key[8..0xc].copy_from_slice(&(media_id_or_minus_one(a) as u32).to_le_bytes());
    key[0x13..].copy_from_slice(&kind.to_le_bytes());
    key[0xc..0x10].copy_from_slice(&(read_usize(b) as u32).to_le_bytes());
    key[0x11..0x13].copy_from_slice(&s.to_le_bytes());
    let e = exe::crt_bsearch(key.as_ptr() as usize, read_usize(this), read_usize(this + media_fields::STORY_LOG_COUNT) as u32, 0x17, 0x757ee0);
    if e == 0 {
        return 0;
    }
    if desde == 0 {
        return 1;
    }
    if media_date_before(e, desde) { 0 } else { 1 }
}

/// Search key for `add_transfer_interest`: {player id, club id or -1}.
pub(crate) unsafe fn interest_key(player: usize, club: usize) -> [u32; 2] {
    [read_usize(player) as u32, if club != 0 { read_usize(club) as u32 } else { u32::MAX }]
}

/// True if the interest key is already in the media's sorted interest array (bsearch with `compare_id_pair_keys`).
pub(crate) unsafe fn has_interest(this: usize, key: &[u32; 2]) -> bool {
    let base = read_usize(this + media_fields::TRANSFER_INTERESTS);
    if base == 0 {
        return false;
    }
    exe::crt_bsearch(key.as_ptr() as usize, base, read_usize(this + media_fields::INTEREST_COUNT) as u32, 0x19, 0x757ea0) != 0
}

/// Caps the interest value at 50,000,000.
pub(crate) fn cap_interest_value(value: i32) -> i32 {
    if value < 0x2faf080 { value } else { 0x2faf080 }
}

/// `Media::add_transfer_interest(player, club, value, rating)`: if the {player, club} key is missing, appends the
/// interest and sorts again.
///
/// With the interest vector (`media_fields::TRANSFER_INTERESTS`, capacity `media_fields::INTEREST_CAPACITY`, count
/// `media_fields::INTEREST_COUNT`; 25-byte records), it searches (`has_interest`: bsearch `exe::crt_bsearch` with
/// `compare_id_pair_keys`) for the key {player id, club id or −1}: already there, nothing. Otherwise it appends: player
/// id (+0), club id (+4, read without checking), today's date (+8, `copy_record_date` from `G_CURRENT_DATE`), 0
/// (`transfer_interest::STATE`, the rumour has not come out yet), the `value` up to 50,000,000
/// (`transfer_interest::VALUE`) and the `rating` × 100 truncated (`transfer_interest::RATING`, _ftol 0x945640); the
/// count goes up and it re-sorts (qsort with `compare_id_pair_keys`). Without a vector, it allocates 20
/// (`exe::crt_malloc`; without memory, box 0x665d and the end); when full, it grows by 20 (realloc `exe::crt_realloc`;
/// box 0x6668). The key's default date (`exe::date_date_54a110`) is on the stack, as in the original.
/// Original: `Media::add_transfer_interest` (0x754740, media.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn add_transfer_interest(this: usize, player: usize, club: usize, value: i32, rating: f64) {
    let mut data = [0u8; 8];
    exe::date_date_54a110(data.as_mut_ptr() as usize);
    let key = interest_key(player, club);
    if has_interest(this, &key) {
        covered!("cpl_7 repeated interest");
        return; // the interest is already there
    }
    covered!("cpl_7 new interest");
    if read_usize(this + media_fields::TRANSFER_INTERESTS) == 0 {
        let p = exe::crt_malloc(0x1f4);
        write_usize(this + media_fields::TRANSFER_INTERESTS, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x665d);
            fatal_exit();
        }
        write_usize(this + media_fields::INTEREST_CAPACITY, 0x14);
    }
    let cap = read_usize(this + media_fields::INTEREST_CAPACITY);
    if cap == read_usize(this + media_fields::INTEREST_COUNT) {
        let p = exe::crt_realloc(read_usize(this + media_fields::TRANSFER_INTERESTS), cap.wrapping_add(0x14).wrapping_mul(0x19) as u32);
        write_usize(this + media_fields::TRANSFER_INTERESTS, p);
        if p == 0 {
            show_error_at(0xa17bec, 0x6668);
            fatal_exit();
        }
        write_usize(this + media_fields::INTEREST_CAPACITY, read_usize(this + media_fields::INTEREST_CAPACITY).wrapping_add(0x14));
    }
    let item = || read_usize(this + media_fields::TRANSFER_INTERESTS).wrapping_add(read_usize(this + media_fields::INTEREST_COUNT).wrapping_mul(0x19));
    write_usize(item(), read_usize(player));
    write_usize(item() + 4, read_usize(club));
    copy_record_date(item() + 8, G_CURRENT_DATE);
    write_u8(item() + transfer_interest::STATE, 0); // the rumour has not come out yet
    write_usize(item() + transfer_interest::VALUE, cap_interest_value(value) as usize);
    write_usize(item() + transfer_interest::RATING, ftol(rating * 100.0) as usize); // [0x9688a8] = 100.0
    let n = read_usize(this + media_fields::INTEREST_COUNT).wrapping_add(1);
    write_usize(this + media_fields::INTEREST_COUNT, n); // mais um interesse
    exe::crt_qsort(read_usize(this + media_fields::TRANSFER_INTERESTS), n as u32, 0x19, 0x757ea0);
}

/// null first; by the +0 id and, when equal, by +4 (−1 matches anything)
/// Original: `compare_id_pair_keys` (0x757ea0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_id_pair_keys(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (x, y) = (read_usize(a) as i32, read_usize(b) as i32);
    if x != y {
        return x.wrapping_sub(y);
    }
    let (x, y) = (read_usize(a + 4) as i32, read_usize(b + 4) as i32);
    if x == -1 || y == -1 {
        return 0;
    }
    x.wrapping_sub(y)
}

/// `compare_story_log_records(a, b)` (cdecl): the qsort and bsearch comparison of the story log.
///
/// Null first (`a` null −1, `b` null 1); by subject a (+8), subject b (+0xc) and type (+0x13), the difference of the
/// first one that differs; when equal, the difference of the subtypes (signed words `story_log_entry::SUBTYPE`).
/// Original: `compare_story_log_records` (0x757ee0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_story_log_records(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    for o in [8usize, 0xc, 0x13] {
        let (x, y) = (read_usize(a + o) as i32, read_usize(b + o) as i32);
        if x != y {
            return x.wrapping_sub(y);
        }
    }
    let s = |p: usize| read_i16(p + story_log_entry::SUBTYPE) as i32;
    s(a) - s(b)
}

/// Builds the 25-byte interest record `add_transfer_interest` appends (player, club, today, capped value, rating x 100).
pub(crate) unsafe fn build_interest_record(player: usize, club: usize, value: i32, rating: f64) -> [u8; 0x19] {
    let mut r = [0u8; 0x19];
    r[..4].copy_from_slice(&(read_usize(player) as u32).to_le_bytes());
    r[4..8].copy_from_slice(&(read_usize(club) as u32).to_le_bytes());
    r[8..0x10].copy_from_slice(core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8));
    r[0x11..0x15].copy_from_slice(&cap_interest_value(value).to_le_bytes());
    r[0x15..].copy_from_slice(&ftol(rating * 100.0).to_le_bytes());
    r
}

/// Calls the exe's CRT qsort.
pub(crate) unsafe fn media_crt_qsort(base: usize, n: i32, size: u32, comparar: usize) {
    exe::crt_qsort(base, (n) as u32, size, comparar);
}

/// Calls the exe's CRT bsearch.
pub(crate) unsafe fn crt_bsearch(key: usize, base: usize, n: u32, size: u32, comparar: usize) -> usize {
    exe::crt_bsearch(key, base, n, size, comparar)
}

/// `Media::remove_transfer_interest(person)`: removes the person's interests, moving the last one into the gap, then
/// sorts again.
///
/// In the interest vector (`media_fields::TRANSFER_INTERESTS`, `media_fields::INTEREST_COUNT` records of 25 bytes),
/// each one of the person (+0 = id) is overwritten by the last one (6 dwords and a byte), the count goes down and the
/// same place is looked at again. If any are left, it re-sorts (qsort `media_crt_qsort` with `compare_id_pair_keys`).
/// Original: `Media::remove_transfer_interest` (0x7549b0, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_transfer_interest(this: usize, person: usize) {
    let n = (this + media_fields::INTEREST_COUNT) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = read_u32(this + media_fields::TRANSFER_INTERESTS) as usize;
            let r = base.wrapping_add(off);
            if read_u32(r) == read_u32(person) {
                let last = base.wrapping_add((*n as usize).wrapping_mul(25)).wrapping_sub(25); // the last interest
                core::ptr::copy(last as *const u8, r as *mut u8, 25);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(25);
            }
            i += 1;
            off = off.wrapping_add(25);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        media_crt_qsort(read_u32(this + media_fields::TRANSFER_INTERESTS) as usize, *n, 0x19, 0x757ea0);
    }
}

/// `0x7357c0` (thiscall, ret 4, media.cpp): the type of the press story stored about the player when it is about
/// indiscipline (list of types at 0xa17be0, terminated by 0xff), otherwise 0.
///
/// Stories: [+0] vector of 0x17 bytes, [+8] how many; at +8 the player id, at +0x11 the type (word). Null player or
/// with +0x39 zeroed: 0.
// simplification: only ax is faithful; the high 16 bits of the original’s eax are a register leftover (the caller uses ax)
/// the kind of the media story about the player when it is a misbehaviour kind, else 0
/// Original: `Media::get_finable_story_kind` (0x7357c0, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_finable_story_kind(average: usize, player: usize) -> u16 {
    let player = Staff(player);
    if player.is_null() || (player.club_job().addr() as u32) == 0 {
        return 0;
    }
    let n = read_i32(average + 8);
    let base = read_usize(average);
    let id = read_u32(player.addr());
    for i in 0..n.max(0) as usize {
        let e = base.wrapping_add(i * 0x17);
        if read_u32(e + story_log_entry::PERSON_A) != id || read_u8(0xa17be0) == 0xff {
            continue;
        }
        let kind = read_u16(e + story_log_entry::SUBTYPE);
        let mut p = 0xa17be0usize;
        loop {
            if kind == read_i8(p) as i16 as u16 {
                return kind;
            }
            p += 1;
            if read_u8(p) == 0xff {
                break;
            }
        }
    }
    0 // no indiscipline story
}

/// `choose_injury_type` (thiscall, ret 0xc, injury.cpp): the injury type for the requested category (body part) and severity.
///
/// If the player already has a current injury (`exe::injury_get_player_injury`) of a type outside the list of minor ones, returns that type.
/// Otherwise it draws a random number for each type of the category (first/last at 0x9e7f05/06 + 3 × category; the type
/// table has 13 bytes from 0x9e7ad8: +1 category, +2 weight, +0xc severity, 0xff = any) and keeps the lowest draw
/// (below 100); the player's recurring type ([this] + 31 × id + 0x12) draws only 0..2 − 1.
///
/// Severity 3: none (0xff).
// simplification: only al is faithful (the original returns char)

/// True if any 0x17-byte media news record has the player id at +8 and passes the predicate.
pub(crate) unsafe fn any_news_for_player(average: usize, id: i32, accepted: impl Fn(usize) -> bool) -> bool {
    let n = read_i32(average + 8);
    let base = read_usize(average);
    (0..n.max(0) as usize).any(|k| {
        let r = base + k * 0x17;
        read_i32(r + 8) == id && accepted(r)
    })
}

/// whether the player has a story with response 2 in the media log
/// Original: `story_with_response_two` (0x735780, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn story_with_response_two(average: usize, id: *const i32) -> u32 {
    any_news_for_player(average, *id, |r| read_u8(r + story_log_entry::RESPONSE) == 2) as u32
}

/// whether the player has any story in the media log
/// Original: `has_story` (0x735840, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn has_story(average: usize, id: *const i32) -> u8 {
    any_news_for_player(average, *id, |_| true) as u8
}

/// `media_queue_player_apology_news(player, club, historia)` (stdcall, ret 0xc; SEH frame without a destructor), called
/// by the story effects (0x736c00) and the reply to the player's story (`exe::media_apply_player_story_response`):
/// builds news item 0xbdb and sends it to the club's manager.
///
/// With the player, the sheet (`.player()`) and the club, news item 0xbdb in a local (`exe::new_newsitem` with 0) with
/// the data 0 = id, 1 to 3 = the names (`names::ID` of `.first_name()`, `.second_name()` and `.common_name()`; without
/// the third, −1), 4 = club id, 5 = the story (`historia`), 6 = 0 and 7 = the variant: draw(4) for stories 0, 2 to 5,
/// 13, 14, 17 and 35 (table 0x72bd2c), otherwise 0; it goes to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `media_queue_player_apology_news` (0x72bbf0, media.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn media_queue_player_apology_news(player: usize, club: usize, historia: i16) {
    let player = Staff(player);
    if player.is_null() || (player.player().addr() as i32) == 0 || club == 0 {
        return;
    }
    covered!("media_queue_player_apology_news");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbdb, 0);
    exe::newsitem_set_slot(n, 0, read_i32(player.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_i32((player.first_name() as i32) as usize + names::ID) as u32);
    exe::newsitem_set_slot(n, 2, read_i32((player.second_name() as i32) as usize + names::ID) as u32);
    let third = (player.common_name() as i32) as usize;
    exe::newsitem_set_slot(n, 3, if third == 0 { u32::MAX } else { read_i32(third + names::ID) as u32 });
    exe::newsitem_set_slot(n, 4, read_i32(club) as u32);
    let h = historia as i32;
    exe::newsitem_set_slot(n, 5, h as u32);
    exe::newsitem_set_slot(n, 6, 0);
    let v = if (h as u32) <= 0x23 && *((0x72bd2c + h as usize) as *const u8) == 0 { exe::utils_random(4) } else { 0 };
    exe::newsitem_set_slot(n, 7, v as u32);
    exe::news_send_to_club_manager(read_i32(G_NEWS) as usize, n, club);
}

/// Draws a former international as the pundit for `club` (or to talk about `subject`), weighted by nation reputation
/// and caps (`this` is not read).
///
/// 150 rounds; in each it draws people (`exe::utils_random` with `G_STAFF_COUNT`, 110 bytes in `G_STAFF_TABLE`) until
/// it finds one aged (`staff::AGE`) ≥ 37, with national team caps (`staff::INT_APPS`) ≥ 50, a non-player record
/// (`staff::NON_PLAYER`), no club (`staff::CLUB_JOB`) if he has a player record (`staff::PLAYER`) and who is not the
/// club's manager (`.manager()`); the 100th try ends the round. The weight is a draw of the nation's reputation
/// (`staff::NATION`, `.reputation()`) × caps, doubled if he is from the subject's nation (without a subject: he must be
/// from the club's nation, `.nation()`). From another nation: the reputation must beat 6500 + a draw of 100, the
/// non-player record's `non_player::WORLD_REPUTATION` ≥ 6000 and the subject must not dislike him
/// (`exe::staff_is_disliked_by`). The one with the highest weight is kept (the first accepted always gets in). Without
/// a club and without a subject: 0.
/// Original: `Media::pick_former_international_pundit` (0x72b810, media.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn pick_former_international_pundit(_this: usize, club: usize, subject: usize) -> usize {
    let subject = Staff(subject);
    let club = Club(club);
    if subject.is_null() && club.is_null() {
        return 0;
    }
    let _u = |p: usize| read_usize(p);
    let injury_word_0c = |nation: Nation| nation.reputation() as i32;
    let (mut melhor, mut maior) = (0usize, 0i32);
    for _ in 0..150 {
        let mut attempts = 0;
        let p = loop {
            let r = exe::utils_random(read_i32(G_STAFF_COUNT));
            attempts += 1;
            if attempts >= 100 {
                break 0;
            }
            let p = read_usize(G_STAFF_TABLE).wrapping_add((r as usize).wrapping_mul(staff::SIZE));
            if read_i8(p + staff::AGE) < 0x25 || read_u8(p + staff::INT_APPS) < 0x32 || read_usize(p + staff::NON_PLAYER) == 0 {
                continue;
            }
            if read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::CLUB_JOB) != 0 {
                continue;
            }
            if club.manager().addr() == p {
                continue;
            }
            break p;
        };
        if p == 0 {
            continue;
        }
        let nation = read_usize(p + staff::NATION);
        let mut weight = exe::utils_random(injury_word_0c(Nation(nation)).wrapping_mul(read_u8(p + staff::INT_APPS) as i32));
        let same_nation = if subject.is_null() {
            if nation != club.nation().addr() {
                continue;
            }
            true
        } else {
            nation == subject.nation().addr()
        };
        if same_nation {
            weight = weight.wrapping_add(weight);
        } else {
            let cut = exe::utils_random(100).wrapping_add(0x1964);
            if injury_word_0c(Nation(nation)) < cut
                || read_i16(read_usize(p + staff::NON_PLAYER) + non_player::WORLD_REPUTATION) < 0x1770
                || exe::staff_is_disliked_by(subject, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || weight > maior {
            maior = weight;
            melhor = p;
        }
    }
    melhor
}

/// `Media::pick_ai_manager_response(manager, player, kind)` (3.7 KB): the computer manager's response (1, 2 or 3) to a
/// story of the given type about the player.
///
/// Without a manager, without the manager sheet (`.non_player()`), a player without a sheet (`.player()`), the manager
/// himself, without a club (`.club_job()`, otherwise the national side `.national_job()`) or without the player's
/// contract with it (`exe::contract_manager_get_contract_for_club`): 3. The rating is trunc(5 ×
/// `exe::media_rate_player_form_for_story`(player, club, contract, kind, 3, 0)); without a player, the argument's word
/// stays in its place (the manager pointer, as in the original). Then, by type: 0x83 by the contract type and the
/// knowledge (`exe::non_player_attribute_rating`); 0x82 by the manager's likes and dislikes
/// (`exe::staff_is_disliked_by`/`exe::staff_is_favoured_by`) and draws; 0x1f by the transfer list
/// (`exe::staff_is_transfer_listed_or_requested`) and the contract; the others, first, by the likes and dislikes with
/// the rating, and then by families of types, each with its own chain of draws (`exe::utils_random`); 8/0x25 and 3/4
/// can put the player on the manager's rejected list (`exe::staff_add_disliked_staff`).
/// Original: `Media::pick_ai_manager_response` (0x735870, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn pick_ai_manager_response(this: usize, manager: usize, player: usize, kind: u32) -> u32 {
    let manager = Staff(manager);
    let player = Staff(player);
    if manager.is_null() || manager.non_player().is_null() {
        return 3;
    }
    if !player.is_null() && player.player().is_null() {
        return 3;
    }
    if manager == player {
        return 3;
    }
    let mut club = manager.club_job();
    if club.is_null() {
        club = manager.national_job();
        if club.is_null() {
            return 3;
        }
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return 3;
    }
    let mut rating = manager.addr() as i32; // without a player: the argument that occupies the slot
    if !player.is_null() {
        rating = ftol(exe::media_rate_player_form_for_story(this, player, club, contract, kind, 3, 0.0) * read_f32(rdata::F32_5) as f64);
    }
    let f = || manager.non_player(); // the manager record
    let ch = |o: usize| exe::non_player_attribute_rating(f(), read_i8(f().addr() + o)) as i8 as i32; // the knowledge by attribute +o
    let k = kind as i16;
    if k == 0x83 {
        let t = contract_squad_status(contract);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && ((contract.matches_left_out() as i8) as i32) > 1 {
            return 1;
        }
        if player.player().current_reputation() < 0x1482 && ch(non_player::DISCIPLINE) > 0xf {
            return 2;
        }
        if contract_squad_status(contract) == 0x70 && ch(non_player::DISCIPLINE) > 0xa {
            return 2;
        }
        return if exe::utils_random(ch(non_player::MAN_HANDLING)) != 0 { 1 } else { 3 };
    }
    if k == 0x82 {
        if exe::staff_is_disliked_by(player, manager) != 0 {
            let a = manager.professionalism() as i32;
            if a < 0xf && exe::utils_random(a) == 0 {
                let x = exe::utils_random(ch(non_player::MAN_HANDLING));
                let y = exe::utils_random(0x14);
                if x < y {
                    return 2;
                }
            }
            let a = manager.professionalism() as i32;
            if a <= 0xe || exe::utils_random(a) <= 10 {
                return 3;
            }
            let x = exe::utils_random(ch(non_player::MAN_HANDLING));
            let y = exe::utils_random(0x14);
            return if x <= y { 3 } else { 1 };
        }
        if exe::staff_is_favoured_by(player, manager) != 0 {
            let x = exe::utils_random(manager.professionalism() as i32);
            let y = exe::utils_random(10);
            if x >= y {
                return 3;
            }
            let x = exe::utils_random(ch(non_player::MAN_HANDLING));
            let y = exe::utils_random(0xf);
            return if x <= y { 3 } else { 1 };
        }
        if (manager.temperament() as i32) < 6 {
            let y = exe::utils_random(0x14);
            if (manager.temperament() as i32) < y {
                return 1;
            }
        }
        let x = exe::utils_random(ch(non_player::MAN_HANDLING));
        let y = exe::utils_random(0x14);
        if x > y {
            return 1;
        }
        if (manager.sportsmanship() as i32) <= 0xf {
            return 3;
        }
        let y = exe::utils_random(10);
        return if (manager.sportsmanship() as i32) <= y { 3 } else { 2 };
    }
    if k == 0x1f {
        if exe::staff_is_transfer_listed_or_requested(player, club) != 0 {
            return 2;
        }
        let b = contract.happiness() as i32;
        if b <= exe::utils_random(100) {
            return 3;
        }
        if exe::staff_contract_wants_to_leave(contract) != 0 || contract.leaving_on_bosman() != 0 {
            return 3;
        }
        return if exe::utils_random(3) != 0 { 1 } else { 3 };
    }
    // the manager’s preferences with the rating
    if exe::staff_is_disliked_by(player, manager) != 0 && rating < 5 {
        if exe::utils_random(manager.professionalism() as i32) != 0 {
            return 3;
        }
        let x = exe::utils_random(ch(non_player::MAN_HANDLING));
        let y = exe::utils_random(0x14);
        return if x >= y { 3 } else { 2 };
    }
    if exe::staff_is_favoured_by(player, manager) != 0 && rating > -5 {
        let x = exe::utils_random(manager.professionalism() as i32);
        let y = exe::utils_random(10);
        if x >= y {
            return 3;
        }
        let x = exe::utils_random(ch(non_player::MAN_HANDLING));
        let y = exe::utils_random(0x14);
        return if x >= y { 3 } else { 1 };
    }
    // the sum of the knowledge (+0x19), of +0x1d of the profile and of +0x59 of the manager
    let sum = || ch(non_player::MAN_HANDLING) + ((f().patience() as i8) as i32) + (manager.loyalty() as i32);
    match k as i32 {
        0x3c | 0x3d => {
            let x = exe::utils_random(ch(non_player::MOTIVATING));
            let y = exe::utils_random(0x17);
            if x > y {
                let x = exe::utils_random((f().patience() as i8) as i32);
                let y = exe::utils_random(10);
                if x < y {
                    return 2;
                }
            }
            if exe::utils_random(manager.pressure() as i32) == 0 {
                return 1;
            }
            let x = exe::utils_random(manager.loyalty() as i32);
            let y = exe::utils_random(0x28);
            if x > y {
                return 1;
            }
            let x = exe::utils_random((f().patience() as i8) as i32);
            let y = exe::utils_random(0x32);
            if x <= y { 3 } else { 1 }
        }
        0x28..=0x3b => {
            let x = exe::utils_random(player.professionalism() as i32);
            let y = exe::utils_random(0x14);
            if x > y {
                let a = exe::utils_random(ch(non_player::MAN_HANDLING));
                let b = exe::utils_random(0x14 - rating);
                if a > b {
                    return 1;
                }
                if exe::utils_random(ch(non_player::MAN_HANDLING)) != 0 {
                    return 3;
                }
                return if rating >= 0xa { 3 } else { 2 };
            }
            let a = exe::utils_random(ch(non_player::MAN_HANDLING));
            let b = exe::utils_random(10);
            if a > b {
                let mut arg = 0;
                if (player.player().work_rate() as u8) != 0 {
                    // the person’s 0x4f-byte record in [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(player.addr()) as usize).wrapping_mul(staff_state::SIZE)) + o);
                    let e = exe::utils_random((2 - (reg(staff_state::STATUS_FLAGS) >> 7) as i32) * 20);
                    if exe::utils_random(reg(staff_state::INJURY_COUNTER) as i32) > e {
                        arg = 1;
                    }
                }
                if exe::utils_random(arg) == 0 && rating < 0xa {
                    return 2;
                }
            }
            if exe::utils_random(player.player().player_morale() as i32) == 0 {
                return 1;
            }
            let x = exe::utils_random(manager.loyalty() as i32);
            let y = exe::utils_random(0x14 - rating);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            covered!("cpo_6 common types");
            let t = contract_squad_status(contract);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = exe::shortlist_manager_find_club_shortlist_entry(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), club, player, 0, 0);
                if e != 0 && read_usize(e + shortlist_slot::TARGET_STAFF_ID) == read_usize(player.addr()) && rating > -10 {
                    covered!("cpo_6 draw discarded");
                    exe::utils_random(manager.loyalty() as i32); // the original discards the result
                }
            }
            let x = exe::utils_random(ch(non_player::MAN_HANDLING));
            let y = exe::utils_random(10);
            if x < y {
                let x = exe::utils_random(manager.temperament() as i32);
                let y = exe::utils_random(10);
                if x < y {
                    let x = exe::utils_random((f().patience() as i8) as i32);
                    let y = exe::utils_random(10);
                    if x < y {
                        let x = exe::utils_random(manager.loyalty() as i32);
                        let y = exe::utils_random(10);
                        if x < y {
                            let x = exe::utils_random(rating);
                            let y = exe::utils_random(-5);
                            if x < y {
                                return 2;
                            }
                        }
                    }
                }
            }
            let x = exe::utils_random(sum());
            let y = exe::utils_random(0x24);
            if x <= y {
                return 3;
            }
            let x = exe::utils_random(rating);
            let y = exe::utils_random(-5);
            if x > y { 1 } else { 3 }
        }
        8 | 0x25 => {
            if exe::utils_random(((f().patience() as i8) as i32) * 2) == 0 {
                exe::staff_add_disliked_staff(manager, player, 1);
            }
            if ch(non_player::MAN_HANDLING) < 0xa {
                let x = exe::utils_random(manager.temperament() as i32);
                let y = exe::utils_random(10);
                if x < y {
                    let x = exe::utils_random((f().patience() as i8) as i32);
                    let y = exe::utils_random(10);
                    if x < y && exe::utils_random(ch(non_player::MAN_HANDLING)) == 0 {
                        return 2;
                    }
                }
            }
            let x = exe::utils_random(sum());
            let y = exe::utils_random(0x24);
            if x <= y { 3 } else { 1 }
        }
        1 => {
            if exe::utils_random(ch(non_player::MAN_HANDLING)) == 0 {
                let x = exe::utils_random(manager.temperament() as i32);
                let y = exe::utils_random(10);
                if x < y {
                    let x = exe::utils_random((f().patience() as i8) as i32);
                    let y = exe::utils_random(5);
                    if x < y && exe::utils_random(ch(non_player::MAN_HANDLING)) == 0 {
                        return 2;
                    }
                }
            }
            let x = exe::utils_random(sum());
            let y = exe::utils_random(0x1e);
            if x <= y { 3 } else { 1 }
        }
        0x1e | 0x20 => {
            let x = exe::utils_random(manager.temperament() as i32);
            let y = exe::utils_random(0x14);
            if x < y {
                let x = exe::utils_random((f().patience() as i8) as i32);
                let y = exe::utils_random(0xf);
                if x < y {
                    return 2;
                }
            }
            let x = exe::utils_random(((f().patience() as i8) as i32) + (manager.loyalty() as i32));
            let y = exe::utils_random(0x96);
            if x > y {
                if player.player().current_reputation() as i32 <= club.reputation() as i32 + 0x1f4 {
                    return 2;
                }
                let x = exe::utils_random((f().discipline() as i8) as i32);
                let y = exe::utils_random(5);
                return if x >= y { 3 } else { 1 };
            }
            let x = exe::utils_random((f().discipline() as i8) as i32);
            let y = exe::utils_random(5);
            if x < y {
                return 1;
            }
            let x = exe::utils_random((f().discipline() as i8) as i32);
            let y = exe::utils_random(0x14);
            if x <= y { 3 } else { 2 }
        }
        0x63 | 0x62 | 0x5d | 0x3f | 0x5f | 0x60 | 0x43 | 0x5c | 0x5a | 0x44 | 0x5b | 0x42 | 0x61 | 0x40 | 0x41 => {
            let mut base: i32 = match k {
                0x5d => 5,
                0x61 => 10,
                0x62 => -7,
                _ => 0,
            };
            if exe::utils_random(manager.pressure() as i32) == 0 {
                base += exe::utils_random(10);
                base -= exe::utils_random(10);
            }
            let x = exe::utils_random((f().discipline() as i8) as i32);
            let y = exe::utils_random(0xf - base);
            if x > y {
                return 2;
            }
            let x = exe::utils_random(manager.ambition() as i32);
            let y = exe::utils_random(base + 0xf);
            if x <= y { 3 } else { 1 }
        }
        3 | 4 => {
            let x = exe::utils_random((f().discipline() as i8) as i32);
            let y = exe::utils_random(0x32);
            if x > y && exe::utils_random((f().patience() as i8) as i32) == 0 {
                let x = exe::utils_random(rating);
                let y = exe::utils_random(-5);
                if x < y {
                    exe::staff_add_disliked_staff(manager, player, 1);
                    return 2;
                }
            }
            let t = contract_squad_status(contract);
            if t != 0 && t != 0x70 && t != 0x80 && exe::staff_is_transfer_listed(player, 0) == 0 {
                let x = exe::utils_random(manager.loyalty() as i32);
                let y = exe::utils_random(6);
                if x > y {
                    let x = exe::utils_random(ch(non_player::MAN_HANDLING));
                    let y = exe::utils_random(6);
                    if x > y {
                        let x = exe::utils_random(rating);
                        let y = exe::utils_random(-5);
                        if x > y {
                            return 1;
                        }
                    }
                }
            }
            let t = contract_squad_status(contract);
            if t != 0x70 && t != 0x40 {
                return 3;
            }
            let x = exe::utils_random(manager.loyalty() as i32);
            let y = exe::utils_random(5);
            if x >= y {
                return 3;
            }
            let x = exe::utils_random(manager.professionalism() as i32);
            let y = exe::utils_random(5);
            if x >= y {
                return 3;
            }
            let x = exe::utils_random(rating);
            let y = exe::utils_random(-5);
            if x >= y { 3 } else { 2 }
        }
        5 => {
            if exe::utils_random(manager.loyalty() as i32) != 0 && exe::utils_random(ch(non_player::MAN_HANDLING)) != 0 && exe::utils_random(player.professionalism() as i32) != 0 {
                let x = exe::utils_random(rating);
                let y = exe::utils_random(-5);
                if x > y {
                    return 1;
                }
            }
            if exe::utils_random(ch(non_player::MAN_HANDLING)) != 0 {
                return 3;
            }
            let x = exe::utils_random(player.pressure() as i32);
            let y = exe::utils_random(10);
            if x >= y {
                return 3;
            }
            let x = exe::utils_random(rating);
            let y = exe::utils_random(-5);
            if x >= y { 3 } else { 2 }
        }
        6 => {
            if exe::utils_random(manager.loyalty() as i32) != 0 {
                let x = exe::utils_random(f().pressing() as i32);
                let y = exe::utils_random(10);
                if x > y {
                    let x = exe::utils_random(player.professionalism() as i32);
                    let y = exe::utils_random(5);
                    if x > y {
                        let x = exe::utils_random(player.player().dirtiness() as i32);
                        let y = exe::utils_random(0x1e);
                        if x < y {
                            let x = exe::utils_random(rating);
                            let y = exe::utils_random(-5);
                            if x > y {
                                return 1;
                            }
                        }
                    }
                }
            }
            if exe::utils_random(ch(non_player::MAN_HANDLING)) != 0 || exe::utils_random(f().pressing() as i32) != 0 {
                return 3;
            }
            let x = exe::utils_random(player.player().dirtiness() as i32);
            let y = exe::utils_random(0x32);
            if x <= y {
                return 3;
            }
            let x = exe::utils_random(rating);
            let y = exe::utils_random(-5);
            if x >= y { 3 } else { 2 }
        }
        _ => 3,
    }
}

/// `Media::post_financial_trouble_news(club)` (SEH frame without a destructor): posts news item 0x1790 of the club in
/// financial trouble to the managers of the nation, or to all for a big club.
///
/// With the club, the nation (`.nation()`), the stadium (`.stadium()`, with a city `stadium::CITY`) and the division
/// (`.division()`), it builds news item 0x1790 in a local (`exe::new_newsitem` with 0) with the data 0 = club id, 1 =
/// reputation (`.reputation()`) and 2 = draw(2) (`exe::utils_random`); a club with reputation 9000 or more in a
/// division with `.reputation()` above 12: to all managers (`exe::news_send_to_all_managers`); otherwise to those of
/// the nation (`exe::news_send_to_nation_managers`). The local news item is 0xe0 bytes (the delivery copies only up to
/// +0xde).
/// Original: `Media::post_financial_trouble_news` (0x74ae60, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn post_financial_trouble_news(_this: usize, club: usize) {
    let club = Club(club);
    if club.is_null() || club.nation().addr() == 0 {
        return;
    }
    let sheet = club.stadium();
    if sheet.is_null() || sheet.city() == 0 || club.division().addr() == 0 {
        return;
    }
    covered!("cpk_6 financial trouble");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1790, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(n, 1, club.reputation() as i32 as u32);
    let v = exe::utils_random(2); // the news item’s random roll
    exe::newsitem_set_slot(n, 2, v as u32);
    let news = read_usize(G_NEWS);
    let division = club.division();
    if club.reputation() >= 9000 && !division.is_null() && division.reputation() > 12 {
        exe::news_send_to_all_managers(news, n);
    } else {
        exe::news_send_to_nation_managers(news, n, club.nation().addr());
    }
}

/// `Media::pick_rival_manager_id(club)`: returns the id of the manager of a rival of the same league and much bigger,
/// or of a drawn club in a league clash, who passes the draws; or −1.
///
/// With a manager (`.manager()`) with a non-player record (`.non_player()`) and a league (`.division()`; without a
/// league, only a national side, `exe::database_is_national_team`): the first of the manager's rejected list
/// (`exe::staff_get_disliked_staff`) with a manager record and rating (`non_player::CURRENT_REPUTATION`) + 750 < rating
/// (never: the draw(3) there does not happen); then each rival (+0xb3, +0xb7, +0xbb) with a manager who passes the
/// draws, of the same league and with reputation (`.reputation()`) above the club's + 750, with draw(3) ≠ 0; then up to
/// 50 drawn clubs (from `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`) that make a league clash
/// (`FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE`, with the draw limit as 3rd argument, a leftover on the original's stack) with
/// a manager who passes the draws. Returns the manager's id, or −1.
/// Original: `Media::pick_rival_manager_id` (0x72b940, media.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn pick_rival_manager_id(_this: usize, club: usize) -> u32 {
    let club = Club(club);
    if (club.manager().addr() as u32) == 0 {
        return u32::MAX;
    }
    if (club.division().addr() as u32) == 0 && exe::database_is_national_team(club) == 0 {
        return u32::MAX;
    }
    let manager = Staff((club.manager().addr() as u32) as usize);
    if (manager.non_player().addr() as u32) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = club.addr() as u32; // the argument’s slot: 0x5480f0 clears the byte before counting
    exe::staff_get_disliked_staff(manager, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = Staff(*rejeitados.as_ptr().add(k as u8 as usize)); // like the original, without checking the size (0x5480f0 gives up to 3)
        if !p.is_null() && (p.non_player().addr() as u32) != 0 {
            let rating = read_i16((p.non_player().addr() as u32) as usize + non_player::CURRENT_REPUTATION) as i32;
            if rating.wrapping_add(0x2ee) < rating && friendly_random(3) != 0 {
                return read_u32(p.addr());
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = Club(read_u32(club.addr() + rival) as usize);
        if r.is_null() || (r.manager().addr() as u32) == 0 || !rival_manager_passes_draws(r) {
            continue;
        }
        if (r.division().addr() as u32) != (club.division().addr() as u32) {
            continue;
        }
        let injury_word_0c = |c: Club| c.reputation() as i32;
        if injury_word_0c(club) + 0x2ee >= injury_word_0c(r) {
            continue; // the rival is not much bigger
        }
        if friendly_random(3) != 0 {
            return read_u32((r.manager().addr() as u32) as usize);
        }
    }
    let tie: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    let mut b = 0i8;
    while b < 0x32 {
        let limit = (read_u32(G_CLUB_COUNT) as i32).wrapping_sub((read_u32(G_NATION_COUNT) as i32).wrapping_mul(2));
        let i = friendly_random(limit) as usize;
        let c = (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if tie(club.addr(), c, limit) != 0 && read_u32(c + club::MANAGER) != 0 && rival_manager_passes_draws(Club(c)) {
            return read_u32(read_u32(c + club::MANAGER) as usize);
        }
        b += 1;
    }
    u32::MAX
}

/// `Media::queue_manager_fa_complaint_story(club, person, kind, a, b, c)` (SEH for the local news item, no destructor),
/// called by discipline (0x55e300) and by MANAGER_MANAGER (`exe::manager_manager_handle_fa_request`, 0x6ab800): posts
/// news item 0x178e of the manager's complaint to the FA.
///
/// A club with a nation (`.nation()`) and a manager (`.manager()`), and a person: news item 0x178e (the club; the
/// manager, id and keys, or −1; the type; the person, id and keys; a; b; the nation or −1; c) to the nation's managers
/// (`exe::news_send_to_nation_managers`) if the reputation (`.reputation()`) is above 4750, otherwise to the club
/// (`exe::news_send_to_club_manager`).
/// Original: `Media::queue_manager_fa_complaint_story` (0x758de0, media.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn queue_manager_fa_complaint_story(this: usize, club: usize, person: usize, kind: u32, a: u32, b: u32, c: u32) {
    let person = Staff(person);
    let club = Club(club);
    let _ = this;
    if club.is_null() || club.nation().addr() == 0 || club.manager().addr() == 0 || person.is_null() {
        return;
    }
    covered!("cpz_10 news of the complaint to the federation");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x178e, 0);
    exe::newsitem_set_slot(nt, 0, read_u32(club.addr()));
    let manager = club.manager();
    if !manager.is_null() {
        exe::newsitem_set_slot(nt, 1, read_u32(manager.addr()));
        exe::newsitem_set_slot(nt, 2, read_u32(manager.first_name() + names::ID));
        exe::newsitem_set_slot(nt, 3, read_u32(manager.second_name() + names::ID));
        exe::newsitem_set_slot(nt, 4, read_u32(manager.common_name() + names::ID));
    } else {
        for k in 1..5 {
            exe::newsitem_set_slot(nt, k, u32::MAX);
        }
    }
    exe::newsitem_set_slot(nt, 5, kind as i8 as u32);
    exe::newsitem_set_slot(nt, 6, read_u32(person.addr()));
    exe::newsitem_set_slot(nt, 7, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(nt, 8, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(nt, 9, read_u32(person.common_name() + names::ID));
    exe::newsitem_set_slot(nt, 0xa, a as i8 as u32);
    exe::newsitem_set_slot(nt, 0xb, b as i16 as u32);
    let nation = club.nation().addr();
    exe::newsitem_set_slot(nt, 0xc, if nation != 0 { read_u32(nation) } else { u32::MAX });
    exe::newsitem_set_slot(nt, 0xd, c as i8 as u32);
    let nation = club.nation().addr();
    if nation != 0 && (club.reputation() as i32) > 0x128e {
        exe::news_send_to_nation_managers(read_usize(G_NEWS), nt, nation);
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
    }
}

