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

/// Rule: the media response screen opens on a story news item of the manager, keeping the story's people and clubs as
/// the news kind lays them out.
///
/// `media_response_screen_open(manager, news_id)`: a local NewsItem; no manager: the media.cpp 15228 box; the item not
/// found (`News::find_item_by_id`): the media.cpp 15235 box. Otherwise `open_screen` with the builder 0x7423c0, the
/// handler 0x742de0, single instance, the no-op leave routine 0x5a8f60 and 0; when it opens, from the item's slots
/// (`NewsItem::get_slot`), by its kind: 0xbdf (team story): 4..7 = slots 15..18, 1 = slot 4, 2 = slot 9, 8 = slot 1,
/// 9 = -1; 0xbdd (player story): 4..7 = slots 0..3, 1 = slot 4, 2 = slot 5, 9 = -1; 0xbe6 (sent off): 4..7 = slots
/// 0..3, 1 = slot 9, 2 = 0x82, 8 = 0, 9 = slot 10; 0xbe8 (wants first team): 4..7 = slots 0..3, 1 = slot 8, 2 = 0, 8 =
/// 0, 9 = -1; 0xfbc: 4..7 = slots 0..3, 1 = slot 4, 2 = 0x1f, 9 = slot 16 when slot 15 is set and slot 16 is a club
/// index, otherwise -1; any other kind: 4..7 = slots 0..3, 1 = slot 4, 2 = slot 9, 9 = -1. Parameter 0 = the news id
/// and 3 = 3. Parameter 8 is left as the original's uninitialised local for the kinds that do not set it (here 0: a
/// defect of the original).
/// Original: `media_response_screen_open` (0x741f50, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_open(manager: usize, news_id: usize) {
    let mut item = [0u8; 0x100];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0, 0);
    if manager == 0 {
        tel26_error_box(MEDIA_CPP, 0x3b7c);
        return;
    }
    if media_find_news_item(manager, news_id, it) == 0 {
        tel26_error_box(MEDIA_CPP, 0x3b83);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x7423c0usize, 0x742de0usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel35_4 open media response");
    let slot = |k: u32| exe::newsitem_get_slot(it, k) as usize;
    let none = usize::MAX;
    let (people, p1, p2, p8, p9): ([usize; 4], usize, usize, usize, usize) = match read_u32(it) {
        MEDIA_NEWS_TEAM_STORY => {
            let people = [slot(0xf), slot(0x10), slot(0x11), slot(0x12)];
            (people, slot(4), slot(9), slot(1), none)
        }
        MEDIA_NEWS_PLAYER_STORY => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(4), slot(5), 0, none)
        }
        MEDIA_NEWS_SENT_OFF => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            let p1 = slot(9);
            (people, p1, 0x82, 0, slot(0xa))
        }
        MEDIA_NEWS_WANTS_FIRST_TEAM => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(8), 0, 0, none)
        }
        MEDIA_NEWS_RIVAL_SIGNING => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            let p1 = slot(4);
            let p9 = if slot(0xf) == 0 {
                none
            } else {
                let c = slot(0x10) as i32;
                if c < 0 || c > read_i32(G_CLUB_COUNT) - 1 { none } else { c as usize }
            };
            (people, p1, 0x1f, 0, p9)
        }
        _ => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(4), slot(9), 0, none)
        }
    };
    set_screen_param(0, news_id);
    set_screen_param(1, p1);
    set_screen_param(2, p2);
    set_screen_param(3, 3);
    for (k, v) in people.into_iter().enumerate() {
        set_screen_param(4 + k as u32, v);
    }
    set_screen_param(8, p8);
    set_screen_param(9, p9);
}

/// Rule: the media response screen shows the story's question in the club's colours and the three responses, the
/// chosen one in yellow, with Cancel and Ok.
///
/// `media_response_screen_draw()`: a local Text; the club (parameter 1), the story code (2), the chosen response (3)
/// and the person (4, with the name ids 5..7: `database_format_name_from_ids(5, 6, 7, name, 400, 1, 4)` when 4 is
/// above -1). The club's background picture, the menu bar (4, 0), the title (the slot manager's name) in the club's kit
/// colours (kit 1), the subtitle "Select Media Response". The question area (0x6e, 0x8c)-(0x30c, 0x221), kind 2, rows
/// = 0x195 / the height of font 3, with seven column weights read from a two-byte array (2, 17), the rows byte and the
/// next locals (a defect of the original: its seventh weight is an uninitialised stack byte, here 0); the question
/// (`media_question`) drawn wrapped from row 0 (font 3, pale grey). The responses area (same rectangle, weights 2/17,
/// kind 1): from two rows below, responses 1..3 (`Media::format_response_option_label`) as buttons id 0x1485 with data
/// k, the chosen one yellow and the others grey; the first one's widget written to parameter 8. The button area
/// (0x64, 0x22b)-(0x316, 0x24e) with "Cancel" (0x109c) and "Ok" (0x109b).
/// Original: `media_response_screen_draw` (0x7423c0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_draw() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("tel35_4 draw media response");
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let mut name = [0u8; 0x190];
    let index = get_screen_param(1);
    let code = get_screen_param(2) as u32;
    let chosen = get_screen_param(3) as i32;
    let person = get_screen_param(4) as i32;
    let (n5, n6, n7) = (get_screen_param(5) as u32, get_screen_param(6) as u32, get_screen_param(7) as u32);
    let club = media_club(index);
    if person > -1 {
        exe::database_format_name_from_ids(n5, n6, n7, name.as_mut_ptr(), 0x190, 1, person as u32);
    }
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back & 0xffff, fore & 0xffff, JOB_TEXT);
    text(JOB_TEXT, 0xa381e0);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8;
    // the original's locals from +0x16: the weights 2, 17, then the rows byte over the low byte of parameter 6 and its
    // three high bytes, then an uninitialised byte (here 0)
    let columns: [u8; 7] = [2, 0x11, rows, (n6 >> 8) as u8, (n6 >> 16) as u8, (n6 >> 24) as u8, 0];
    let area = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let g = |c: usize, at: usize| read_u8(c + at) as i8 as i32;
    let n = name.as_ptr();
    match media_question(code) {
        MediaQuestion::Name(f) => {
            text(JOB_TEXT, f, n);
        }
        MediaQuestion::Plain(f) => {
            text(JOB_TEXT, f);
        }
        MediaQuestion::NameOrPlain(f, plain) => {
            if person > -1 { text(JOB_TEXT, f, n) } else { text(JOB_TEXT, plain) };
        }
        MediaQuestion::Hundred(f) => {
            text(JOB_TEXT, f, 0x64, n);
        }
        MediaQuestion::Club(f, plain) => {
            if person > -1 {
                text(JOB_TEXT, f, g(club, 0x52), g(club, 0x52), n, club + 0x38);
            } else {
                text(JOB_TEXT, plain, g(club, 0x52), g(club, 0x52), club + 0x38);
            }
        }
        MediaQuestion::ClubOrNation(f, nation_text) => {
            if person > -1 {
                text(JOB_TEXT, f, g(club, 0x52), g(club, 0x52), n, club + 0x38);
            } else {
                let nation = read_usize(read_usize(club + 0x53) + 0x71);
                text(JOB_TEXT, nation_text, g(nation, 0x1e), g(nation, 0x1e), nation + 4);
            }
        }
    }
    exe::text_set_string(t, JOB_TEXT as *const u8);
    let mut line = 0i32;
    tel35_text_draw_wrapped(t, area, 0x6e, 0x30c, &mut line, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    let mut row = line + 2;
    let weights: [u8; 4] = [2, 0x11, 3, 1];
    let responses = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    for k in 1..=3i32 {
        exe::media_format_response_option_label(read_usize(G_MEDIA), code, k as u32, JOB_TEXT, 0x7d0);
        let colour = if chosen != k { tel35_colour(G_GUI_COLOUR_GREY) } else { tel35_colour(G_GUI_COLOUR_YELLOW) };
        let w = tel26_area_button([responses, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, MEDIA_RESPONSE_ID, k as u32]);
        if k == 1 {
            set_screen_param(8, w as i16 as isize as usize);
        }
        row += 1;
    }
    let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr().wrapping_add(2) as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (tel35_colour(G_GUI_COLOUR_GREY), tel35_colour(G_GUI_COLOUR_PALE_GREY));
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, MEDIA_RESPONSE_CANCEL, 0]);
    text(JOB_TEXT, 0x990f7c);
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, MEDIA_RESPONSE_OK, 0]);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// Rule: on the media response screen a response click colours it yellow and keeps it; Ok with a response logs it,
/// applies it to the story (by the news kind) and marks the news item answered; Cancel closes.
///
/// `media_response_screen_handle_event(item)`: the news id (parameter 0), the club (1), the story code (2), the chosen
/// response (3), the person (4), the other club (9) and the slot manager. In a network game (`[0xb77c84]`) the pending
/// command argument is cleared and -9 comes back. Id 0x1485 (a response): its data k; the three response widgets
/// (from parameter 8) recoloured, yellow the k-th (1, 2, anything else the third) and grey the others; parameter 3 =
/// k; redraw, 0. Id 0x109b (Ok): with no response -9; otherwise the news item (`News::find_item_by_id`), and with a
/// person the story log record (`media_log_response`) gets the response; story code 0x1f with another club and a
/// person warns the squad of the rival (`Media::warn_squad_of_position_rival`); then by the news kind: 0xbdf
/// `apply_team_story_response(club, code, response, person or 0)`, 0xbe6 `apply_sent_off_story_response(person, club,
/// other club, response)`, 0xbe8 `apply_wants_first_team_response(person, club, response)`, any other
/// `apply_player_story_response(person, club, code, response)`; the news item's slot marked with the response
/// (`News::set_item_slot`: 0xd for 0xbdd, 0x12 for 0xbe6 and 0xbe8, 7 for 0xfbc, 0xa otherwise); -9. Id 0x109c
/// (Cancel): -9. Otherwise the menu bar's command, then its object link.
/// Original: `media_response_screen_handle_event` (0x742de0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_handle_event(raw: u32) -> i32 {
    let news_id = get_screen_param(0);
    let index = get_screen_param(1);
    let code = get_screen_param(2) as u32;
    let response = get_screen_param(3) as u32;
    let person_index = get_screen_param(4) as i32;
    let other = get_screen_param(9);
    let manager = tel26_slot_manager().addr();
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    let id = tel26_item_id(item) as u32;
    if id == MEDIA_RESPONSE_ID {
        covered!("tel35_4 media response chosen");
        let k = tel26_item_value(item);
        let w = get_screen_param(8) as u32;
        let yellow_at = match k {
            1 => 0,
            2 => 1,
            _ => 2,
        };
        for j in 0..3u32 {
            let colour = if j == yellow_at { G_GUI_COLOUR_YELLOW } else { G_GUI_COLOUR_GREY };
            exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w.wrapping_add(j), tel35_colour(colour));
        }
        set_screen_param(3, k as usize);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        return 0;
    }
    if id == MEDIA_RESPONSE_OK {
        if response == 0 {
            return -9;
        }
        covered!("tel35_4 media response ok");
        let mut news = [0u8; 0x100];
        let it = news.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0, 0);
        media_find_news_item(manager, news_id, it);
        let club = media_club(index);
        let person = media_person(person_index as usize);
        if person_index > -1 {
            media_log_response(person, club, code, read_u32(it), response);
        }
        if code == 0x1f && other != usize::MAX && person_index != -1 {
            exe::media_warn_squad_of_position_rival(read_usize(G_MEDIA), person, media_club(other), club, response);
        }
        let kind = read_u32(it);
        let code_word = (code & 0xffff_0000) | (code as u8 as i8 as i16 as u16 as u32);
        match kind {
            MEDIA_NEWS_TEAM_STORY => {
                if person_index > -1 {
                    let staff_word = (read_u32(G_STAFF_TABLE) & 0xffff_0000) | (code as u8 as i8 as i16 as u16 as u32);
                    exe::media_apply_team_story_response(read_usize(G_MEDIA), club, staff_word, response, person);
                } else {
                    exe::media_apply_team_story_response(read_usize(G_MEDIA), club, code_word, response, 0usize);
                }
            }
            MEDIA_NEWS_SENT_OFF => {
                let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_APPLY_SENT_OFF_STORY_RESPONSE);
                f(read_usize(G_MEDIA), person, club, media_club(other), response);
            }
            MEDIA_NEWS_WANTS_FIRST_TEAM => {
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_APPLY_WANTS_FIRST_TEAM_RESPONSE);
                f(read_usize(G_MEDIA), person, club, response);
            }
            _ => exe::media_apply_player_story_response(read_usize(G_MEDIA), person, club, code_word, response),
        }
        let slot = match read_u32(it) {
            MEDIA_NEWS_PLAYER_STORY => 0xd,
            MEDIA_NEWS_SENT_OFF | MEDIA_NEWS_WANTS_FIRST_TEAM => 0x12,
            MEDIA_NEWS_RIVAL_SIGNING => 7,
            _ => 0xa,
        };
        exe::news_set_item_slot(read_usize(G_NEWS), manager, news_id, slot, response, 0);
        return -9;
    }
    if id == MEDIA_RESPONSE_CANCEL {
        return -9;
    }
    manager_menu_bar(raw)
}

/// Comparer for `media_response_screen_open`.
pub(crate) unsafe extern "C" fn compare_media_response_screen_open(manager: usize, news_id: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_MEDIA_RESPONSE_SCREEN_OPEN)(manager, news_id)
}

/// Comparer for `media_response_screen_draw`: side by side over the display list; a difference only inside the
/// column weights of one area (+0xbad..+0xbe9 of an `Area`: the question area's seventh weight is an uninitialised
/// stack byte in the original) is not a divergence.
pub(crate) unsafe extern "C" fn compare_media_response_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_MEDIA_RESPONSE_SCREEN_DRAW);
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((JOB_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| media_response_screen_draw()));
    let areas = current_human_display() + 4;
    let weights_only = mo.0.iter().zip(mr.0.iter()).all(|((a, x), (_, y))| {
        x.iter().zip(y.iter()).enumerate().all(|(i, (p, q))| {
            let at = a + i;
            p == q || (at >= areas && at < areas + 0xf9 * 0xbf1 && (0xbad..0xbe9).contains(&((at - areas) % 0xbf1)))
        })
    });
    SCOREBOARD.record(equal || weights_only, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("Average response screen (media_response_screen_draw): differences in {:?}", dif)
    });
}

/// Comparer for `media_response_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_media_response_screen_handle_event(item: u32) -> i32 {
    // simplification: the response, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_MEDIA_RESPONSE_SCREEN_HANDLE_EVENT)(item)
}

/// `Media::write_player_media_story_news_text(news, body, out)`: writes the headline or the body of a media story about
/// a player into the text `out`.
///
/// The headline (`body` 0) is one line by the story kind (slot 9). The body reads the slots (0: the player, 1..3 his
/// names, 4: his club, 5: the subtype, 6: the outlet type, 9: the kind, 10: the reply, 11..14: a second person, 16..18:
/// the squad role, the age and the story bits, 23..26: the third person, the club manager or the human manager, 28:
/// with a match, then the fixture record and the result, or the qualities, the position, the pundit and a fourth
/// person) and writes, in pieces: the outlet's name (the pundit's when the media is local, else the outlet type, plain
/// for the local media or with the adjective of the club's nation), the story by kind (left out, ability, complacency,
/// contribution with the score, criticism by the bits of slot 18, commitment, indiscipline, purchase, the manager rows,
/// attitude, private life, transfer rumour, exclusive, too young, rumours, out of favour, past it, pressure, poor run;
/// the other kinds the programmers' placeholder), how a drawn match was eventually won or lost, then the reply
/// paragraph: the human manager's when he is the third person (support, criticise, ignore), otherwise the club
/// manager's; the text is "%s\n\n%s" of the two, with the links of the persons whose names match, the club's nation
/// (name, short name, ten adjectives), the club, the score and the other club.
///
/// Defects of the original kept: slot 14 is read twice and 15 never; the kind 0x10 foreign text passes the club's
/// gender byte where the club's name goes; the kind 1 story above subtype 6 writes nothing (the original then prints its
/// uninitialised buffer; the copy an empty text); the headline's names get a size (0x190) larger than their buffers;
/// the headline of kind 0x11 divides by subtype + 1 (a subtype of -1 divides by zero, in both).
/// Original: `Media::write_player_media_story_news_text` (0x72d270, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_media_story_news_text(_this: usize, news: usize, body: u32, out: usize) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().clear());
    if body == 0 {
        return player_story_headline(news, out);
    }
    let mut st = PlayerStory::read(news);
    st.write_outlet();
    st.write_story();
    st.write_reply();
    st.finish(out);
}

/// Comparer for `write_player_media_story_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator's state, without counting); the strings and the links (text, type and data) must
/// be equal. The message names the last piece the copy ran (headline, outlet, the story kind, the draw result, the
/// reply, the links).
pub(crate) unsafe extern "thiscall" fn compare_write_player_media_story_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_PLAYER_MEDIA_STORY_NEWS_TEXT);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let mut pieces = Vec::new();
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out), || {
        crate::rng::without_counting(|| {
            write_player_media_story_news_text(this, news, body, lp);
            pieces = PLAYER_STORY_TRAIL.with(|t| t.borrow().clone());
        })
    });
    let (mo, mr) = (nma3_text_contents(out), nma3_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || {
        format!("write_player_media_story_news_text(body {}) pieces {:?}; original {:?} {:x?} rust {:?} {:x?}", body, pieces,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}

/// `Media::write_player_praise_news_text(news, body, out)`: writes the headline or the body of a media story that
/// praises a player, naming the outlet or pundit, a great player, the player and the manager's reply, then links every
/// name in the text.
///
/// Reads the news slots (`exe::newsitem_get_slot`): 0 the player (staff index) and 1-3 his name ids, 4 his club, 5 the
/// kind (0x29..0x34), 7 a nation, 8 the variant, 9 the manager and 10-12 his name ids, 13 the reply, 14 the strength
/// flags, 15 the position flags, 16 a competition, 17 "with a match record"; then, from slot 0x12 or after the match
/// record (`read_match_record_from_news` 0x7e4fa0, which moves the index), a rival club, four values, the great player
/// and his name ids, two nations, a value, the great player's job and club (−1 out of range), the outlet type and
/// pundit, the player's age [inference] and a flag. With a human manager on screen (`G_SCREEN_RECORDS_MANAGER` of
/// `G_SCREEN_MANAGER_CURRENT_SLOT`), the rival club is dropped when values 1 and 4 are 0, the player % 4 is not 0 and
/// the rival is of the manager's nation (his club job's nation, or his own); `home` is set when the player's club is
/// of that nation. The position text (`write_position_name` of the position flags, 0xc8) gives the gender; when the
/// manager is the player himself it reads "player-manager" (gender 0).
///
/// The outlet (`praise_outlet`): the pundit's name (`PunditList::get_pundit_name` of `G_PUNDIT_LIST`) at home with a
/// nation, an outlet type and a pundit; otherwise by type 1..6 and other: national or local newspaper, national or
/// local radio station, magazine, sports paper, website ("{nationality} ..." abroad, `nation_get_nationality_adjective`
/// forms 0x16..0x1a; the magazine's gender is 1 with language 8, the others 0). The great player: his name from the ids,
/// or Bobby Charlton, Diego Maradona, Pelé or Johan Cruyff by (variant + player) % 4. The names of the player and the
/// manager in styles 1 and 5 (`database_format_name_from_ids`, 0x168).
///
/// Body: the first paragraph by kind (one piece each, `praise_body_*`), then the reply (`praise_reply`), joined as
/// "%s\n\n%s" in the text buffer 0xde1f64. Headline (`praise_headline`): "Praise for {club} {position}" when variant % 8
/// is player % 7, otherwise by kind. The text goes to `out` (`exe::text_set_string`) and the links follow
/// (`praise_links`): the match (0x7d3) and its club, the player, the manager and the great player when their ids match
/// (`database_staff_matches_names`, 0x7d7), the great player's club, the rival club and the player's club (0x7d5), the
/// club's nation and its national team under ten adjective forms (0x16..0x1f), three nations and the competition
/// (0x7d0).
///
/// Defect of the original: the match link's text is the frame byte +0x4c, zeroed only in the headline; in the body it is
/// whatever the stack held (the copy writes an empty text in both, and the comparer does not compare that text).
/// Original: `Media::write_player_praise_news_text` (0x744f80, media.cpp, thiscall, ret 0xc).
/// Set when the text used the great player's job gender that the original leaves uninitialised (`legend_job_name`).
pub(crate) static PRAISE_JOB_GENDER_UNSET: AtomicBool = AtomicBool::new(false);

pub(crate) unsafe extern "thiscall" fn write_player_praise_news_text(_this: usize, news: usize, body: u32, out: usize) {
    PRAISE_JOB_GENDER_UNSET.store(false, Relaxed);
    let mut frame = [0u8; praise_frame::SIZE];
    let mut p = praise_read_slots(news, frame.as_mut_ptr() as usize);
    praise_setup(&mut p);
    praise_outlet(&mut p);
    praise_names(&mut p);
    let text = if body != 0 {
        covered!("nma_4 praise body");
        praise_body(&mut p);
        praise_reply(&mut p);
        write_u8(PRAISE_TEXT_BUFFER, 0);
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(PRAISE_TEXT_BUFFER, 0x991948, p.head(), p.reply()); // "%s\n\n%s"
        PRAISE_TEXT_BUFFER
    } else {
        praise_headline(&mut p);
        p.head()
    };
    exe::text_set_string(out, text as *const u8);
    praise_links(&mut p, out);
    PRAISE_LAST_PIECE = p.piece;
}

/// Comparer for `write_player_praise_news_text`: the original writes into `out`; the copy, side by side (the text
/// buffer 0xde1f64 and the generator's state), into a local empty Text; then the strings and the new links (text, type,
/// data) are compared and the local Text is destroyed. A divergence names the piece that wrote the text. The match
/// link's text (0x7d3) is not compared in the body of a kind other than 0x34 (the original's uninitialised frame byte).
pub(crate) unsafe extern "thiscall" fn compare_write_player_praise_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_PLAYER_PRAISE_NEWS_TEXT);
    let before = nma4_links(out).len();
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let est = crate::rng::STATE_REGIONS;
    let regions = [(PRAISE_TEXT_BUFFER, 2000usize), est[0], est[1]];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(this, news, body, out),
        || crate::rng::without_counting(|| write_player_praise_news_text(this, news, body, lp)));
    let piece = PRAISE_LAST_PIECE;
    let (so, sr) = (nma4_c_text(read_usize(out)), nma4_c_text(read_usize(lp)));
    let lo: Vec<_> = nma4_links(out).into_iter().skip(before).collect();
    let lr = nma4_links(lp);
    let kind = exe::newsitem_get_slot(news, 5);
    let loose = body != 0 && kind != 0x34;
    let links_same = lo.len() == lr.len()
        && lo.iter().zip(lr.iter()).all(|(a, b)| a.1 == b.1 && a.2 == b.2 && (a.0 == b.0 || (loose && a.1 == 0x7d3)));
    // the great player's job gender outside language 4 is the original's uninitialised stack byte: a "{}" left
    // after the job is the only difference accepted (and the text buffer then differs by it)
    let without_mark = |t: &[u8]| t.windows(2).fold((Vec::new(), false), |(mut v, pular), w| {
        if pular { (v, false) } else if w == b"{}" { (v, true) } else { v.push(w[0]); (v, false) }
    }).0;
    let unset = PRAISE_JOB_GENDER_UNSET.load(Relaxed);
    let text_same = so == sr || (unset && without_mark(&[so.as_slice(), b"\0"].concat()) == without_mark(&[sr.as_slice(), b"\0"].concat()));
    SCOREBOARD.record((same || (unset && text_same)) && text_same && links_same, || format!(
        "write_player_praise_news_text(kind {:#x}, body {}, piece {}) text {} links {} ({} x {}) memory {}\n original: {}\n rust:     {}\n links original {:x?}\n links rust {:x?}\n {:x?} x {:x?}",
        kind, body, piece, so == sr, links_same, lo.len(), lr.len(), same, String::from_utf8_lossy(&so), String::from_utf8_lossy(&sr),
        lo.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
        lr.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(), mo, mr));
    exe::drop_text(local.as_mut_ptr());
}

/// `Media::write_transfer_rumour_news_text(news, body, out)`: writes the headline or the body of a transfer rumour
/// story into `out`, with the links.
///
/// It reads slots 0 to 14 of the news item (`exe::newsitem_get_slot`): the player (0) and his name ids (1-3), the
/// bidding club (4, −1 none), the kind of lead (5), the fee (6), the agent line kind (7), the player's strength (8)
/// and position (9) flags, the agent (10) and his name ids (11-13) and a flag (14); slot 15 is the number of clubs
/// in the story, read from slot 0x10 on (the list stops at slot 0x32 and then counts as empty; the original keeps it
/// in a 5-club frame array, and longer lists run over the slots kept after it, which the copy reproduces). The
/// player's position name (`write_position_name`) and description (`write_player_description`, with its gender).
/// The nation of the story, by (count + agent + lead + club + player) % 6 with a club: 1-2 the club's nation, 3 the
/// player's nation if its reputation is 6000 or more, 4-5 the first listed club's nation; otherwise none. The
/// "local" flag: the human manager of the current screen (`G_SCREEN_RECORDS_MANAGER`) with no nation, of a club
/// (`.club_job()`) of that nation or, without a club, of that nationality.
/// The headline (`transfer_rumour_headline`) or the body: the lead by kind (`transfer_rumour_lead_kind0`,
/// `_kind1`, `_kind36`, `_other`), the clubs paragraph (`transfer_rumour_clubs`), the agent line
/// (`transfer_rumour_agent_line`) and the links (`transfer_rumour_links`).
/// Original: `Media::write_transfer_rumour_news_text` (0x755860, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_rumour_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut f = [0u32; 0x2c];
    f[0x50 / 4] = slot(0);
    f[0x4c / 4] = slot(1);
    f[0x44 / 4] = slot(2);
    f[0x40 / 4] = slot(3);
    let club = slot(4) as i32;
    let lead = slot(5) as i32;
    f[0x5c / 4] = slot(6);
    f[0x64 / 4] = slot(7);
    f[0x48 / 4] = slot(8);
    let position = slot(9);
    f[0x3c / 4] = slot(10);
    f[0x60 / 4] = slot(11);
    f[0x58 / 4] = slot(12);
    f[0x54 / 4] = slot(13);
    let flag14 = slot(14) as u8;
    let mut count = slot(15) as i32;
    if count > 0 {
        let (mut k, mut i) = (0x10u32, 0usize);
        loop {
            if k == 0x32 {
                count = 0; // the list is too long: no clubs
                break;
            }
            f[0x28 / 4 + i] = slot(k);
            k += 1;
            i += 1;
            if i as i32 >= count {
                break;
            }
        }
    }
    let mut r = Rumour { f, club, lead, position, flag14, count, nation: 0, local: false, desc_gender: 0 };
    let mut desc = [0u8; 0xc8];
    let g = write_position_name(r.position, desc.as_mut_ptr() as usize, 0xc8);
    // the original pushes a seventh argument (1) that the description does not read
    r.desc_gender = write_player_description(r.player(), r.position, r.qualities(), desc.as_mut_ptr() as usize, 0xc8, g as u32) as i8 as i32;
    if r.club > -1 {
        let sum = r.count.wrapping_add(r.agent()).wrapping_add(r.lead).wrapping_add(r.club).wrapping_add(r.player());
        r.nation = match sum % 6 {
            1 | 2 => club_at(r.club as isize).nation().addr(),
            3 => {
                let n = staff_at(r.player() as isize).nation();
                if n.reputation() < 0x1770 { 0 } else { n.addr() } // nma5: the player's nation from 6000
            }
            4 | 5 => club_at(r.listed(0) as isize).nation().addr(),
            _ => 0,
        };
    }
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    if !manager.is_null() {
        r.local = if r.nation == 0 {
            true
        } else if !manager.club_job().is_null() {
            manager.club_job().nation().addr() == r.nation
        } else {
            manager.nation().addr() == r.nation
        };
    }
    if body == 0 {
        transfer_rumour_headline(&r, out);
        return;
    }
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x78];
    let mut player_short = [0u8; 0x78];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x78, 1, r.player() as u32);
    exe::database_format_name_from_ids(n1, n2, n3, player_short.as_mut_ptr(), 0x78, 5, r.player() as u32);
    let mut agent = [0u8; 0x78];
    let mut agent_short = [0u8; 0x78];
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        exe::database_format_name_from_ids(a1, a2, a3, agent.as_mut_ptr(), 0x78, 1, r.agent() as u32);
        exe::database_format_name_from_ids(a1, a2, a3, agent_short.as_mut_ptr(), 0x78, 5, r.agent() as u32);
    }
    let fee_value = r.fee();
    let mut fee = [0u8; 0x32];
    let amount = exe::cash_get_display_amount(&fee_value, 6);
    exe::cash_format_money(fee.as_mut_ptr() as usize, 0x32, amount, 6);
    let (p, fp, d) = (player.as_ptr() as usize, fee.as_ptr() as usize, desc.as_ptr() as usize);
    let piece = match r.lead {
        0 => transfer_rumour_lead_kind0(&r, out, p, fp),
        1 => transfer_rumour_lead_kind1(&r, out, p, fp),
        0x24 => transfer_rumour_lead_kind36(&r, out, p, fp, d),
        _ => transfer_rumour_lead_other(&r, out, p, fp),
    };
    transfer_rumour_clubs(&r, out, piece, d);
    transfer_rumour_agent_line(&r, out, player_short.as_ptr() as usize, agent.as_ptr() as usize);
    transfer_rumour_links(&r, out, p, player_short.as_ptr() as usize, agent.as_ptr() as usize);
}

/// Comparer for `write_transfer_rumour_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator, without counting the copy's draws); the strings and the links (text, type and
/// data) must be equal. The message names the piece that ran (headline; the lead of kind 0, 1, 36 or other, the
/// clubs paragraph and its number of clubs, the agent line).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_rumour_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_TRANSFER_RUMOUR_NEWS_TEXT);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out),
        || crate::rng::without_counting(|| write_transfer_rumour_news_text(this, news, body, lp)));
    let (mo, mr) = (nma5_text_contents(out), nma5_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    let piece = TRANSFER_RUMOUR_PIECE.load(Relaxed);
    SCOREBOARD.record(same_rng && mo == mr, || {
        let lead = ["headline", "lead kind 0", "lead kind 1", "lead kind 36", "lead other"][(piece & 0xff).min(4) as usize];
        let agent = ["", ", agent line human", ", agent line quoted"][((piece >> 16) & 3).min(2) as usize];
        format!("write_transfer_rumour_news_text(body {}) piece: {}, clubs {}{}; original {:?} {:x?} rust {:?} {:x?}", body, lead, (piece >> 8) & 0xff, agent,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}

/// `Media::post_team_media_story(club, kind, person, game, variant, extra)`: posts a media story (news 0xbdf) about a
/// club or national team, logs it and lets a computer manager answer it.
///
/// Nothing before the news start date (`before_news_start_date`), without a club, without a division (unless a
/// national team, `exe::database_is_national_team`), without a nation, without a stadium or its city (unless a
/// national team), without a manager or his sheet (`.non_player()`); a national team only for kinds 0x5a and 0x44;
/// nothing if the story is already in the log (`exe::media_has_logged_story`, type 0xbdf with the kind) or the person
/// is a player below reputation 3500. News item 0xbdf (local, `exe::new_newsitem`): 0 = −1; 1 = the draw by kind
/// (`TEAM_STORY_DRAW_CASE`; some kinds drop the person on a non-zero draw, the others become kind 0x63); 2 = the tone:
/// with the club's reputation at most 4500 + draw(1500) and not a national team, 2 (draw(3) ≤ 1) or 4; otherwise by
/// draw(100): 0 → 6, below 8 → 3, below 35 → 5, below 60 → 0, otherwise 1 (national team or draw(3) non-zero) or 2;
/// 3 = the nation's id; 4 = the club's id; 9 = the kind; 10 = the computer manager's answer
/// (`exe::media_pick_ai_manager_response`, for the answered kinds of `TEAM_STORY_ANSWERED` at a non-human club,
/// otherwise 0); 5 and 6 = the manager's ambition above 15 with temperament below 6, or ambition below 15 with
/// temperament above 14, both with reputation above 5750; 7 = the competition (kinds 0x5a/0x44: the nation's current
/// competition, `exe::nationalteams_get_current_competition`, only if a finals one; 0x5c: the continental
/// competition, `club::CONTINENTAL_COMP_ID`, only if there is one; otherwise the division); 0xb-0xe the manager and
/// his names; 0xf-0x12 the person and his names or −1; 0x13 = whether there is a game (then its record,
/// `exe::fill_match_record_from_fixture` and `exe::write_match_record_to_news`, from slot 0x14 on). From the next
/// free slot: the variant, whether the club was the last one placed in the competition
/// (`exe::comp_util_get_latest_placed_club`), the next hosts (`exe::hostcountry_find_next_hosts`), the extra word, the
/// person's club and club job (jobs 7, 9, 10, 11, 14 and those above 15 as −1) or national team and job (not job 11)
/// or −1 −1, whether he has caps, the shoot-out (0 none, 1 won, 2 lost; nothing for a level one) with a game, and a
/// pundit (`exe::punditlist_pick_random_pundit` by the tone and the nation). Delivery: a non-human club, on a
/// draw(2) non-zero, to its manager; otherwise kinds 0x5a/0x44 to all; 0x5c to the continent; a division above
/// reputation 15 with the club at 9000 or more, to the continent; with the club above 8000 + draw(3000) − (30 × the
/// variant + 1000 above variant 20), kinds 0x50/0x51 above variant 15 and 0x41, 0x5d, 0x61, 0x62, 0x63 to the continent;
/// kind 0x3d to the division on a draw(3) of 0, otherwise to the manager; the others to the division (club below
/// 5000) or to the nation (an English club from 6000 also to Scotland, a Scottish one from 6500 to England, the
/// others also to the club's manager). Then the log (`log_story`) and, for an answered kind at a non-human club, the
/// answer (`Media::apply_team_story_response`, 0x73e870).
/// Original: `Media::post_team_media_story` (0x74b060, media.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let club = Club(club);
    let mut person = Staff(person);
    let mut kind = kind;
    let mut record = [0u8; 0x20]; // the match record (a Date at its start)
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    if before_news_start_date() || club.is_null() {
        return;
    }
    let national = || exe::database_is_national_team(club) != 0;
    if club.division().is_null() && !national() {
        return;
    }
    if club.nation().is_null() {
        return;
    }
    if club.stadium().is_null() && !national() {
        return;
    }
    if !national() && club.stadium().city() == 0 {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let k = kind as u8;
    if national() && k != 0x5a && k != 0x44 {
        return;
    }
    if exe::media_has_logged_story(this, person, club, 0xbdf, k as i8 as i16 as u16 as u32, core::ptr::null()) != 0 {
        return;
    }
    if !person.is_null() && !person.player().is_null() && person.player().current_reputation() < 0xdac {
        return;
    }
    covered!("nma_5 team story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbdf, 0);
    exe::newsitem_set_slot(n, 0, u32::MAX);
    let e = (k as i8 as i32).wrapping_sub(0x3c);
    let case = if (0..0x28).contains(&e) { TEAM_STORY_DRAW_CASE[e as usize] } else { 5 };
    let draw = match case {
        0 => exe::utils_random(3),
        1 => exe::utils_random(4),
        2 => {
            if club.reputation() > 0x1482 {
                (exe::utils_random(2) as u8).wrapping_add(1) as i8 as i32
            } else {
                exe::utils_random(3)
            }
        }
        3 | 4 => {
            let d = exe::utils_random(if case == 3 { 3 } else { 2 });
            if d != 0 {
                person = Staff(0);
            }
            d
        }
        _ => {
            covered!("nma_5 team story other kind");
            kind = (kind & !0xff) | 0x63;
            if exe::utils_random(2) != 0 {
                person = Staff(0);
            }
            0
        }
    };
    let k = kind as u8 as i8;
    exe::newsitem_set_slot(n, 1, draw as u32);
    let tone: u32 = if (club.reputation() as i32) > exe::utils_random(0x5dc) + 0x1194 || national() {
        let d = exe::utils_random(100) as u8 as i8;
        if d == 0 {
            6
        } else if d < 8 {
            3
        } else if d < 0x23 {
            5
        } else if d < 0x3c {
            0
        } else if national() || exe::utils_random(3) != 0 {
            1
        } else {
            2
        }
    } else if (0..=1).contains(&exe::utils_random(3)) {
        2
    } else {
        4
    };
    exe::newsitem_set_slot(n, 2, tone);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 9, k as i32 as u32);
    let a = (k as i32).wrapping_sub(0x3e);
    let answered = !((0..0x21).contains(&a) && TEAM_STORY_ANSWERED[a as usize] == 0);
    let reply = if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        exe::media_pick_ai_manager_response(this, manager, person, k as i16 as u16 as u32) // nma5: the AI manager's answer
    } else {
        0
    };
    exe::newsitem_set_slot(n, 0xa, reply);
    let rep = manager.non_player().current_reputation();
    exe::newsitem_set_slot(n, 5, (manager.ambition() > 0xf && manager.temperament() < 6 && rep > 0x1676) as u32);
    exe::newsitem_set_slot(n, 6, (manager.ambition() < 0xf && manager.temperament() > 0xe && rep > 0x1676) as u32);
    let k = k as u8;
    let comp = if k == 0x5a || k == 0x44 {
        let nt = read_usize(G_NATIONAL_TEAMS);
        if exe::national_teams_is_finals_comp(nt, exe::nationalteams_get_current_competition(nt, club.nation())) == 0 {
            return;
        }
        read_u32(exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation()))
    } else if k == 0x5c {
        let c = club.continental_comp_id();
        if c == u32::MAX {
            return;
        }
        c
    } else {
        read_u32(club.division().addr())
    };
    exe::newsitem_set_slot(n, 7, comp);
    exe::newsitem_set_slot(n, 0xb, read_u32(manager.addr()));
    exe::newsitem_set_slot(n, 0xc, read_u32(manager.first_name() + names::ID));
    exe::newsitem_set_slot(n, 0xd, read_u32(manager.second_name() + names::ID));
    exe::newsitem_set_slot(n, 0xe, read_u32(manager.common_name() + names::ID));
    if !person.is_null() {
        exe::newsitem_set_slot(n, 0xf, read_u32(person.addr()));
        exe::newsitem_set_slot(n, 0x10, read_u32(person.first_name() + names::ID));
        exe::newsitem_set_slot(n, 0x11, read_u32(person.second_name() + names::ID));
        exe::newsitem_set_slot(n, 0x12, read_u32(person.common_name() + names::ID));
    } else {
        for s in 0xf..=0x12 {
            exe::newsitem_set_slot(n, s, u32::MAX);
        }
    }
    let mut c: u8 = 0x14;
    if game != 0 {
        exe::newsitem_set_slot(n, 0x13, 1);
        exe::fill_match_record_from_fixture(game, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut c);
    } else {
        exe::newsitem_set_slot(n, 0x13, 0);
    }
    let mut put = |v: u32| {
        exe::newsitem_set_slot(n, c as i8, v);
        c = c.wrapping_add(1);
    };
    put(variant as i8 as i32 as u32);
    let latest = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp as usize).wrapping_mul(0x6b)), 0);
    put((latest == club.addr()) as u32);
    let (mut host_a, mut host_b) = (0u32, u32::MAX);
    exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), comp, &mut host_a, &mut host_b);
    put(host_a);
    put(host_b);
    put(extra);
    let caps = if !person.is_null() {
        let cj = person.club_job();
        let nj = person.national_job();
        if !cj.is_null() {
            put(read_u32(cj.addr()));
            let j = person.job_for_club() as i8 as i32;
            put(if (1..=0xf).contains(&j) && TEAM_STORY_JOB_HIDDEN[(j - 1) as usize] == 0 { j as u32 } else { u32::MAX });
        } else if !nj.is_null() && person.job_for_nation() != 0xb {
            put(read_u32(nj.addr()));
            put(person.job_for_nation() as i8 as i32 as u32);
        } else {
            put(u32::MAX);
            put(u32::MAX);
        }
        person.int_apps() != 0
    } else {
        put(u32::MAX);
        put(u32::MAX);
        false
    };
    put(caps as u32);
    if game != 0 {
        let g = Fixture(game);
        let (h, w) = (g.home_penalties(), g.away_penalties());
        if h == -1 {
            put(0);
        } else if h > w {
            put(if g.home_club() == club { 1 } else { 2 });
        } else if h < w {
            put(if g.away_club() == club { 1 } else { 2 });
        }
    }
    let nation = club.nation();
    let pundit = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, tone, if !nation.is_null() { read_i32(nation.addr()) } else { -1 });
    put(pundit as u32);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && exe::utils_random(2) != 0 {
        team_story_deliver(n, 1, club.addr());
    } else {
        team_story_deliver_by_kind(n, club, k, variant as u8 as i8);
    }
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_LOGGED.store(true, Relaxed);
        TEAM_STORY_LOG_PERSON.store(person.addr(), Relaxed);
        TEAM_STORY_LOG_KIND.store(kind, Relaxed);
        TEAM_STORY_LOG_REPLY.store(reply, Relaxed);
        return;
    }
    log_story(this, person.addr(), club.addr(), 0xbdf, kind, reply);
    if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        exe::media_apply_team_story_response(this, club, k as i8 as i16 as u16 as u32, reply, person);
    }
}

/// Comparer for `post_team_media_story`: the copy runs as a rehearsal (without delivering, logging or answering) and
/// the original from the same random state. Must be equal: the copy's random draws are the beginning of the
/// original's (the deliveries' automatic replies and the answer draw afterwards); the original logged the story
/// (one more record of type 0xbdf for the club, the person, the kind and the answer, dated today) exactly when the
/// copy would; and each human manager whose inbox (News +8: 12 bytes per human) advanced received the copy's news
/// item (type +0..4 and data +5..0xd6); with no delivery planned, no inbox advances.
pub(crate) unsafe extern "thiscall" fn compare_post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32)>(IDX_POST_TEAM_MEDIA_STORY);
    let news = read_usize(G_NEWS);
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    let boxes_before: Vec<(i32, i32)> = if boxes != 0 { (0..0x10).map(|k| (read_i32(boxes + 12 * k), read_i32(boxes + 12 * k + 4))).collect() } else { Vec::new() };
    let log_before = read_i32(this + media_fields::STORY_LOG_COUNT);
    let before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    TEAM_STORY_DRY_RUN.store(true, Relaxed);
    TEAM_STORY_DELIVERIES.store(0, Relaxed);
    TEAM_STORY_LOGGED.store(false, Relaxed);
    let (_, rr) = crate::rng::trace(|| crate::rng::without_counting(|| post_team_media_story(this, club, kind, person, game, variant, extra)));
    TEAM_STORY_DRY_RUN.store(false, Relaxed);
    let (deliveries, logged, built) = (TEAM_STORY_DELIVERIES.load(Relaxed), TEAM_STORY_LOGGED.load(Relaxed), *core::ptr::addr_of!(TEAM_STORY_BUILT));
    before.write_back();
    let (_, ro) = crate::rng::trace(|| o(this, club, kind, person, game, variant, extra));
    // the log: a record of this story, dated today, exactly when the copy logs one
    let mut found = false;
    let log_after = read_i32(this + media_fields::STORY_LOG_COUNT);
    if logged {
        let p = TEAM_STORY_LOG_PERSON.load(Relaxed);
        let want_person = if p != 0 { read_u32(p) } else { u32::MAX };
        for i in 0..log_after.max(0) as usize {
            let e = read_usize(this).wrapping_add(i * 0x17);
            if read_u32(e + story_log_entry::TYPE) == 0xbdf && read_u32(e + story_log_entry::PERSON_B) == read_u32(club)
                && read_u32(e + story_log_entry::PERSON_A) == want_person
                && read_i16(e + story_log_entry::SUBTYPE) == TEAM_STORY_LOG_KIND.load(Relaxed) as u8 as i8 as i16
                && read_u8(e + story_log_entry::RESPONSE) == TEAM_STORY_LOG_REPLY.load(Relaxed) as u8
                && read_u32(e) == read_u32(G_CURRENT_DATE)
            {
                found = true;
                break;
            }
        }
    }
    let log_same = if logged { found && log_after > log_before } else { log_after == log_before };
    // the human managers' inboxes
    let (mut delivered, mut news_same) = (0, true);
    let mut diffs: Vec<(usize, u8, u8)> = Vec::new(); // (offset, delivered, built): the first differing bytes, for the message
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    if boxes != 0 && boxes_before.len() == 0x10 {
        for (k, &(start, next)) in boxes_before.iter().enumerate() {
            let e = boxes + 12 * k;
            if read_i32(e + 4) == next && read_i32(e) == start {
                continue;
            }
            delivered += 1;
            let v = if start == -1 { read_i32(e) } else { next }; // the first news item opens the ring at the start
            let s = read_usize(news).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != built[0..4] || g[5..0xd6] != built[5..0xd6] {
                news_same = false;
                diffs.extend((0..0xd6).filter(|&i| i != 4 && g[i] != built[i]).take(24).map(|i| (i, g[i], built[i])));
            }
        }
    }
    let same = ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && log_same && news_same && (deliveries != 0 || delivered == 0);
    SCOREBOARD.record(same, || format!("post_team_media_story(club {:#x}, kind {:#x}) draws original {:?} rust {:?}; logged {} found {} log {} -> {}; deliveries {} delivered {} news equal {} differing bytes (offset, delivered, built) {:x?}",
        club, kind as u8, ro, rr, logged, found, log_before, log_after, deliveries, delivered, news_same, diffs));
}

/// `Media::apply_player_story_response(player, club, topic, response)`: the effects of the manager's answer (1, 2, 3)
/// to a media story about one of his players.
///
/// Nothing without the player, the club, the club's manager (+0xcf) with a staff sheet (`.non_player()`), the player's
/// contract (`exe::contract_manager_get_contract`) or, for a club that is not a national team
/// (`exe::database_is_national_team`), its division's object in `G_COMP_OBJECT_TABLE` with the stats (+8). The weight of
/// the story effects is the club's stat 3 there (`exe::comp_stats_get_club_stat`, as a float; 0 for a national team).
/// Then by topic: 0x25 `respond_to_position_rival_story`, 0x1f `respond_to_transfer_interest_story`, 0x20
/// `respond_to_outburst_story`, the rest `respond_to_other_player_story`.
///
/// Called by the computer manager's answer to a new story (`create_player_media_story`, 0x72d1ed), the outburst
/// (0x733f97, answer 0), the human manager's answer screen (0x743275), the praise story (0x744f52) and the transfer
/// rumour (`process_transfer_interest`, 0x7551dd).
/// Original: `Media::apply_player_story_response` (0x741020, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let (player, club) = (Staff(player), Club(club));
    if club.is_null() || player.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    if contract.is_null() {
        return;
    }
    let comp_object = |c: Club| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(c.division().addr())));
    if exe::database_is_national_team(club) == 0 && (club.division().is_null() || comp_object(club) == 0) {
        return;
    }
    let mut stats = 0usize;
    if exe::database_is_national_team(club) == 0 {
        stats = read_usize(comp_object(club) + 8);
        if stats == 0 {
            return;
        }
    }
    let age = player.age() as i8;
    let chairman = club.chairman();
    let chairman_weight = if chairman.is_null() {
        0.5
    } else {
        ((chairman.ambition() as i32) - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let stat = if stats != 0 { exe::comp_stats_get_club_stat(stats, club, 3) as f32 } else { 0.0 };
    match topic as u16 {
        0x25 => respond_to_position_rival_story(this, player, club, manager, contract, response),
        0x1f => respond_to_transfer_interest_story(player, club, manager, contract, response),
        0x20 => respond_to_outburst_story(this, player, club, chairman_weight, stat, response),
        _ => respond_to_other_player_story(this, player, club, contract, age, stat, topic, response),
    }
}

/// Comparer for `apply_player_story_response`: the rehearsal of the copy, then the original, by piece
/// (`player_story_response_piece`).
///
/// The copy runs first with `MEDIA_REACTION_DRY_RUN`: it stops (and the comparison falls back on the original alone)
/// at a call that may post news (the apology, the confidence adjustments, the story effects, the complaint: the whole
/// of the outburst and other-story pieces) or at event flags or contract flags that would make the player request a
/// transfer. Otherwise the regions it wrote are kept, put back, the original runs and they must come out equal: the
/// random generator, `G_CONTRACT_LOOKUP_RESULT`, the club's row in [[`G_MANAGER_MANAGER`]] (0x49 bytes), the player's
/// personality bytes (`.ambition()` .. `.temperament()`), his preferences' favourite and disliked staff (24 bytes), his
/// contract (0x50 bytes) and, for him and the 50 squad slots, the player record's work rate and morale. Inside a
/// recorded tape unit: only the original.
pub(crate) unsafe extern "thiscall" fn compare_apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_APPLY_PLAYER_STORY_RESPONSE);
    if player == 0 || club == 0 || read_usize(G_MANAGER_MANAGER) == 0 || is_recording() {
        return o(this, player, club, topic, response);
    }
    let (p, c) = (Staff(player), Club(club));
    let lookup = read_usize(G_CONTRACT_LOOKUP_RESULT);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.push((manager_manager_row(c), 0x49));
    reg.push((p.addr() + staff::AMBITION, 7));
    let pref = p.staff_preferences();
    if pref != 0 {
        reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 24));
    }
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    if k != 0 {
        reg.push((k, 0x50));
    }
    for s in core::iter::once(player).chain((0..50usize).map(|j| c.squad(j))) {
        let sheet = if s != 0 { Staff(s).player() } else { PlayerSheet(0) };
        if !sheet.is_null() {
            reg.push((sheet.addr() + player_sheet::WORK_RATE, 2));
        }
    }
    write_usize(G_CONTRACT_LOOKUP_RESULT, lookup); // the lookups above do not count
    let before = crate::comparison::MemorySnapshot::read(&reg);
    MEDIA_REACTION_DRY_RUN.store(true, Relaxed);
    MEDIA_REACTION_STOPPED.store(false, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| apply_player_story_response(this, player, club, topic, response)));
    MEDIA_REACTION_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    if MEDIA_REACTION_STOPPED.load(Relaxed) {
        covered!("nma_6 player story only in the original");
        return o(this, player, club, topic, response);
    }
    o(this, player, club, topic, response);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("apply_player_story_response [{}] (player {:#x}, club {:#x}, topic {:#x}, response {}) differences at {:x?}",
            player_story_response_piece(topic), player, club, topic as u16, response as i8, dif)
    });
}

/// `News::find_item_by_id(manager, id, out)` (0x786b70, thiscall on [0xb7431c], ret 0xc).
pub(crate) unsafe fn media_find_news_item(manager: usize, id: usize, item: usize) -> u32 {
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, (id) as usize as u32, item)
}

/// The club of a club index (the club table entry, `club::SIZE` each).
pub(crate) unsafe fn media_club(index: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((index as u32).wrapping_mul(club::SIZE as u32) as usize)
}

/// The person of a staff index (the staff table entry, `staff::SIZE` each).
pub(crate) unsafe fn media_person(index: usize) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((index as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// The question of each story code 1..0x82 (the original's two-level jump table at 0x742d58/0x742cc4); the others
/// read 0xa36cf8 with 100 and the name.
pub(crate) fn media_question(code: u32) -> MediaQuestion {
    use MediaQuestion::*;
    match code {
        0x82 => Name(0xa38164),
        0x3c => Plain(0xa380f4),
        0x3f => NameOrPlain(0xa38090, 0xa38044),
        0x5f => NameOrPlain(0xa37fe0, 0xa37f90),
        0x60 => NameOrPlain(0xa37f24, 0xa37ed0),
        0x42 => NameOrPlain(0xa37e58, 0xa37df4),
        0x43 => NameOrPlain(0xa37d90, 0xa37d44),
        0x44 => Club(0xa37ccc, 0xa37c6c),
        0x5a => Club(0xa37bf8, 0xa37b9c),
        0x5c => ClubOrNation(0xa37b1c, 0xa37ab4),
        0x3d => Plain(0xa37a3c),
        0x52 => Name(0xa379b0),
        0x5b => Plain(0xa37954),
        0x61..=0x63 => NameOrPlain(0xa378dc, 0xa3787c),
        0x5d | 0x41 => NameOrPlain(0xa37740, 0xa376dc),
        0x40 => NameOrPlain(0xa37810, 0xa377b4),
        0x28..=0x34 => Hundred(0xa3766c),
        0xc => Name(0xa375c8),
        0x1 => Hundred(0xa37554),
        0x9 => Name(0xa374e4),
        0x25 => Hundred(0xa3745c),
        0x1e => Name(0xa373e4),
        0x20 => Name(0xa37374),
        0x1f => Name(0xa372e8),
        0x21 => Name(0xa37264),
        0x22 => Name(0xa371e8),
        0x8 => Hundred(0xa37160),
        0x6 => Name(0xa370d4),
        0x7 => Hundred(0xa3704c),
        0x23 => Hundred(0xa36fc4),
        0xd => Hundred(0xa36f5c),
        0x11 => Hundred(0xa36ee4),
        0x3 => Hundred(0xa36e70),
        0x13 => Hundred(0xa36de4),
        0xa => Hundred(0xa36d60),
        _ => Hundred(0xa36cf8),
    }
}

/// The story log record the media response is written into (0x17 bytes, searched with `bsearch` in the media log
/// [media + 0], count [media + 8], comparer 0x757ee0): a exe::date_date_54a110(`Date::Date`), the person's and the club's ids (-1 for
/// none), a zero byte, the story code (its low byte sign-extended to a word) and the news kind.
pub(crate) unsafe fn media_log_response(person: usize, club: usize, code: u32, kind: u32, response: u32) {
    let mut key = [0u8; 0x18];
    let k = key.as_mut_ptr() as usize;
    exe::date_date_54a110(k);
    write_u32(k + 8, if person != 0 { read_u32(person) } else { u32::MAX });
    write_u32(k + 0xc, if club != 0 { read_u32(club) } else { u32::MAX });
    write_u8(k + 0x10, 0);
    write_u16(k + 0x11, code as u8 as i8 as i16 as u16);
    write_u32(k + 0x13, kind);
    let average = read_usize(G_MEDIA);
    let found = exe::crt_bsearch(k, read_usize(average), read_u32(average + 8), 0x17, 0x757ee0usize);
    if found != 0 {
        write_u8(found + 0x10, response as u8);
    }
}

/// The headline (`body` 0): by the story kind (slot 9), with the player's name in form 5 (slots 0..3) and the second
/// person's (slots 11..14); links: the player, the second person, the club (short name and name) and its nation (to
/// the national team).
pub(crate) unsafe fn player_story_headline(news: usize, out: usize) {
    player_story_piece("headline");
    covered!("nma_3 headline");
    let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
    let player = s(0);
    let names = [s(1), s(2), s(3)];
    let club = s(4);
    let subtype = s(5);
    s(6);
    s(6); // the original asks for slot 6 twice and never for 7 and 8
    let kind = s(9);
    s(10);
    let second = s(11);
    let second_names = [s(12), s(13), s(14)];
    s(18);
    s(23);
    s(24);
    s(25);
    s(26);
    let mut mask_text = [0u8; 0x200];
    let mask = s(0x20);
    let mask_text_fn: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
    mask_text_fn(mask as u32, mask_text.as_mut_ptr() as usize, 1);
    let c = Club(read_usize(G_CLUB_TABLE).wrapping_add((club as usize).wrapping_mul(club::SIZE)));
    let mut name = [0u8; 0x200];
    let mut second_name = [0u8; 0x200];
    let mut text = [0u8; 0x1388];
    // the size given (0x190) is larger than the original's buffers (0xc8 and 0x78)
    exe::database_format_name_from_ids(names[0] as u32, names[1] as u32, names[2] as u32, name.as_mut_ptr(), 0x190, 5, player as u32);
    if second != -1 {
        exe::database_format_name_from_ids(second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, second_name.as_mut_ptr(), 0x190, 5, player as u32);
    } else {
        second_name[0] = 0;
    }
    let (t, n) = (text.as_mut_ptr(), name.as_ptr());
    let (short, g) = (c.addr() + club::SHORT_NAME, (c.short_gender_name() as i8) as i32);
    match kind {
        0x23 => story_lang!(t, 0xa18e14, n),
        0xe => story_lang!(t, 0xa18dd4, 0x64, n),
        4 => story_lang!(t, 0xa18da4, n),
        0xd => story_lang!(t, 0xa18d64, n),
        0x11 => match (club / (subtype + 1) + player) % 3 {
            0 => story_lang!(t, 0xa18c94, 0x64, n),
            1 => story_lang!(t, 0xa18cdc, n),
            _ => story_lang!(t, 0xa18d18, 0x64, n),
        },
        0 => match subtype % 3 {
            0 => story_lang!(t, 0xa18c60, n),
            1 => story_lang!(t, 0xa18c2c, n),
            _ => story_lang!(t, 0xa18bf8, n),
        },
        9 if subtype == 0 || subtype == 1 => story_lang!(t, 0xa18b50, 0x64, second_name.as_ptr(), n),
        9 => story_lang!(t, 0xa18bb4, 0x64, n),
        0x13 | 0x21 | 0xa => story_lang!(t, 0xa188d8, g, g, short),
        0x22 => story_lang!(t, 0xa18b18, n),
        8 => story_lang!(t, 0xa18ad4, 0x64, n),
        3 => story_lang!(t, 0xa18aa0, n),
        5 => story_lang!(t, 0xa18a68, n),
        7 => story_lang!(t, 0xa18a34, n),
        6 => story_lang!(t, 0xa189d0, 0x64, g, g, n, short),
        0xc => story_lang!(t, 0xa18990, 0x64, n),
        0x25 => story_lang!(t, 0xa18950, n),
        _ => story_lang!(t, 0xa18910, 0x64, n),
    }
    exe::text_set_string(out, text.as_ptr());
    let rec = read_usize(G_STAFF_TABLE).wrapping_add((player as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, rec);
    }
    if second > -1 {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((second as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, 0usize) != 0 {
            exe::text_add_link(out, second_name.as_ptr(), 0x7d7, rec);
        }
    }
    if club == -1 {
        return;
    }
    exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
    exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
    let n = c.nation().addr();
    let t = exe::database_get_national_team(n);
    exe::text_add_link(out, (c.nation().addr() + 4) as *const u8, 0x7d5, t);
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc).
pub(crate) unsafe fn nma3_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
    let text = |p: usize| if p == 0 { Vec::new() } else { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec() };
    let mut links = Vec::new();
    let l = read_usize(t + 4);
    if l != 0 {
        list_rewind(l);
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            links.push((text(e), read_u32(e + 0xc8), read_u32(e + 0xcc)));
        }
    }
    (text(read_usize(t)), links)
}

/// Reads the news slots in the original's order (the match record moves the index of the slots after 17).
pub(crate) unsafe fn praise_read_slots(news: usize, base: usize) -> Praise {
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let player = s(0) as i32;
    let names = [s(1), s(2), s(3)];
    let club_index = s(4) as i32;
    let kind = s(5) as i32;
    s(6);
    let nation_a = s(7) as i32;
    let variant = s(8) as i32;
    let manager = s(9) as i32;
    let manager_names = [s(10), s(11), s(12)];
    let response = s(13) as i32;
    let strengths = s(14);
    let position = s(15);
    let comp = s(16) as i32;
    let has_match = s(17);
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let mut k: u8 = 0x12;
    if has_match != 0 {
        exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    }
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as u32);
        k = k.wrapping_add(1);
        v
    };
    let rival_club = next() as i32;
    let (m1, m2, m3, m4) = (next() as i32, next() as i32, next() as i32, next() as i32);
    let legend = next() as i32;
    let legend_names = [next(), next(), next()];
    let (nation_b, nation_c, m11) = (next() as i32, next() as i32, next() as i32);
    let legend_job = next() as i32;
    let mut legend_club = next() as i32;
    let (outlet_type, pundit, age, m17) = (next() as i32, next() as i32, next() as i32, next() as i32);
    if legend_club < 0 || legend_club >= read_i32(G_CLUB_COUNT) {
        legend_club = -1;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE));
    Praise {
        base, player, names, club_index, club, kind, nation_a, variant, manager, manager_names, response, strengths,
        position, comp, has_match, rival_club, m1, m2, m3, m4, legend, legend_names, nation_b, nation_c, m11,
        legend_job, legend_club, outlet_type, pundit, age, m17, home: false, pos_gender: 0, outlet_gender: 0,
        job_gender: 0, record, piece: "",
    }
}

/// The human manager's nation test (drops the rival club, sets `home`) and the position text.
pub(crate) unsafe fn praise_setup(p: &mut Praise) {
    let human = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    if !human.is_null() {
        let club_nation = |c: i32| read_usize(read_usize(G_CLUB_TABLE).wrapping_add((c as usize).wrapping_mul(club::SIZE)) + club::NATION);
        let job = human.club_job();
        let mine = if !job.is_null() { job.nation().addr() } else { human.nation().addr() };
        if p.m1 == 0 && p.m4 == 0 && p.rival_club > -1 && praise_rem(p.player, 4) != 0 && club_nation(p.rival_club) == mine {
            covered!("nma_4 praise rival dropped");
            p.rival_club = -1;
        }
        if club_nation(p.club_index) == mine {
            p.home = true;
        }
    }
    p.pos_gender = exe::write_position_name(p.position, p.pos(), 0xc8, 1) as u32;
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    if staff_of(p.manager) == staff_of(p.player) {
        Praise::format()(p.pos(), 0xa242e0); // "player-manager"
        p.pos_gender &= 0xffff_ff00;
    }
}

/// The outlet's name and gender byte (see the routine's header).
pub(crate) unsafe fn praise_outlet(p: &mut Praise) {
    let f = Praise::format();
    let out = p.at(praise_frame::OUTLET);
    let nation = p.nation();
    if nation != 0 && p.outlet_type != -1 && p.pundit != -1 && p.home {
        covered!("nma_4 praise pundit");
        exe::punditlist_get_pundit_name(read_usize(G_PUNDIT_LIST), p.outlet_type as u32, p.pundit as u32, out, &mut p.outlet_gender);
        return;
    }
    covered!("nma_4 praise outlet type");
    // by type: (home text, form, abroad text)
    let (home, form, abroad) = match p.outlet_type.wrapping_sub(1) {
        0 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        1 => (0xa24284, 0x17, 0xa24250), // local newspaper
        2 => (0xa24238, 0x18, 0xa24208), // national radio station
        3 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        4 => (0xa241b0, 0x19, 0xa24184), // magazine
        5 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if p.home {
        f(out, home);
    } else {
        exe::nation_get_nationality_adjective(nation, p.adj(), 0x78, form);
        f(out, abroad, 0x64, p.adj());
    }
    p.outlet_gender = if p.outlet_type.wrapping_sub(1) == 4 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
}

/// The great player's name and the names of the player and the manager.
pub(crate) unsafe fn praise_names(p: &mut Praise) {
    let ids = |b: usize, n: [u32; 3], size: u32, style: u32, who: i32| {
        exe::database_format_name_from_ids(n[0], n[1], n[2], b as *mut u8, size, style, who as u32);
    };
    if p.legend > -1 {
        ids(p.lname_buf(), p.legend_names, 0x78, 1, p.legend);
    } else {
        let k = praise_rem(p.variant.wrapping_add(p.player), 4);
        let name = praise_frame::LEGENDS[if (0..3).contains(&k) { k as usize } else { 3 }];
        let n = (0..).take_while(|&i| read_u8(name + i) != 0).count() + 1;
        core::ptr::copy_nonoverlapping(name as *const u8, p.lname_buf() as *mut u8, n); // strcpy
    }
    ids(p.full(), p.names, 0x168, 1, p.player);
    ids(p.short(), p.names, 0x168, 5, p.player);
    ids(p.at(praise_frame::MANAGER_FULL), p.manager_names, 0x168, 1, p.manager);
    ids(p.at(praise_frame::MANAGER_SHORT), p.manager_names, 0x168, 5, p.manager);
}

/// The first paragraph of the body, by kind.
pub(crate) unsafe fn praise_body(p: &mut Praise) {
    match p.kind.wrapping_sub(0x29) {
        0 => praise_body_future_international(p),
        1 => praise_body_youngster(p),
        2 => praise_body_praised(p),
        4 => praise_body_on_the_rise(p),
        5 => praise_body_big_move(p),
        6 => praise_body_attitude(p),
        7 => praise_body_world_class(p),
        8 => praise_body_world_class_b(p),
        9 => praise_body_impresses(p),
        10 => praise_body_world_stage(p),
        11 => praise_body_competition(p),
        _ => praise_body_call_up(p),
    }
}

/// The second paragraph: the reply of the manager (slot 13: 1, 2, 3 for none, other a blank), in the second person
/// when the manager is the human on screen, by kind (and, for the players' age [inference], by value 16 against 24
/// and 34, or 36 with value 17).
/// Original: part of `Media::write_player_praise_news_text` (0x749d35..0x74a30c).
pub(crate) unsafe fn praise_reply(p: &mut Praise) {
    let f = Praise::format();
    let (r, short) = (p.reply(), p.short());
    let manager = p.at(praise_frame::MANAGER_FULL);
    let k = p.kind.wrapping_sub(0x29);
    let k = if (0..12).contains(&k) { k } else { -1 };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let older = if p.m17 != 0 { p.age > 0x24 } else { p.age > 0x22 };
    let blank = || {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(r, 0x9f1308); // " "
    };
    if human == staff_of(p.manager) {
        covered!("nma_4 praise reply human");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa3aaa0);
                }
                1 => {
                    f(r, 0xa3aa38, short);
                }
                3 => {
                    f(r, 0xa3a9d8);
                }
                4 | 7 => {
                    f(r, 0xa3a978, short);
                }
                5 => {
                    f(r, 0xa3aafc);
                }
                6 => {
                    f(r, 0xa3ab4c);
                }
                8 => {
                    f(r, 0xa3a908, short);
                }
                9 => {
                    f(r, 0xa3a8a0, 0x64, short);
                }
                10 => {
                    f(r, 0xa3a7c8, 0x64, short);
                }
                11 => {
                    f(r, 0xa3a83c, 0x64, short);
                }
                _ => {
                    f(r, 0xa3a754, 0x64, short);
                }
            },
            2 => match k {
                0 | 1 | 4 | 5 | 7 => {
                    f(r, 0xa3b008, 0x64, short);
                }
                3 => {
                    f(r, 0xa3af90, short);
                }
                6 => {
                    f(r, 0xa3b05c, 0x64, short);
                }
                8 => {
                    f(r, 0xa3af08, short);
                }
                9 => {
                    f(r, 0xa3ae80, short);
                }
                10 => {
                    f(r, 0xa3ad8c, short);
                }
                11 => {
                    f(r, 0xa3adf8, short);
                }
                _ => {
                    let t = if older {
                        0xa3ac40
                    } else if p.age >= 0x18 {
                        0xa3aba4
                    } else {
                        0xa3ad04
                    };
                    f(r, t, short);
                }
            },
            3 => {
                f(r, 0xa3b0e0); // "You have chosen not to reply to this press statement."
            }
            _ => blank(),
        }
    } else {
        covered!("nma_4 praise reply manager");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa39f14, manager);
                }
                1 => {
                    f(r, 0xa39e84, manager, short);
                }
                3 => {
                    f(r, 0xa39e00, manager);
                }
                4 | 7 => {
                    f(r, 0xa39d80, manager, short);
                }
                5 => {
                    f(r, 0xa39f98, manager);
                }
                6 => {
                    f(r, 0xa3a010, manager);
                }
                8 => {
                    f(r, 0xa39d0c, manager, short);
                }
                9 => {
                    f(r, 0xa39c80, 0x64, manager, short);
                }
                10 => {
                    f(r, 0xa39b5c, 0x64, manager, short);
                }
                11 => {
                    f(r, 0xa39bf8, 0x64, manager, short);
                }
                _ => {
                    f(r, 0xa39ac4, 0x64, manager, short);
                }
            },
            2 => {
                let t = match k {
                    0 | 1 | 4 | 5 | 7 => 0xa3a630,
                    3 => 0xa3a588,
                    6 => 0xa3a6ac,
                    8 => 0xa3a4d8,
                    9 => 0xa3a430,
                    10 => 0xa3a2ec,
                    11 => 0xa3a384,
                    _ => {
                        if older {
                            0xa3a14c
                        } else if p.age >= 0x18 {
                            0xa3a08c
                        } else {
                            0xa3a238
                        }
                    }
                };
                f(r, t, 0x64, 0x64, manager, short);
            }
            3 => {
                f(r, 0x9f1308); // " "
            }
            _ => blank(),
        }
    }
}

/// The headline: "Praise for {club} {position}" when variant % 8 is player % 7 (C remainders), otherwise by kind:
/// rated highly, a 'future international' or set for call-up (0x29, by (variant + player) % 3), shows promise, given
/// boost or youngster praised (0x2a, the same), praised, set for call-up, on the rise, big move close, attitude
/// praised, world class talent (0x30, 0x31), impresses at {club}, tipped to shine on world stage, lights up or stars in
/// {competition} (0x34, by (variant × player) % 17 above 6), touted (other kinds). The match link's text (frame +0x4c)
/// is emptied.
/// Original: part of `Media::write_player_praise_news_text` (0x74a33f..0x74a5d4).
pub(crate) unsafe fn praise_headline(p: &mut Praise) {
    covered!("nma_4 praise headline");
    p.piece = "headline";
    let f = Praise::format();
    let (h, short) = (p.head(), p.short());
    write_u8(p.at(0x4c), 0);
    if praise_rem(p.variant, 8) == praise_rem(p.player, 7) {
        f(h, 0xa39a70, p.cg(), p.cg(), p.pg(), p.pg(), p.cname(), p.pos());
        return;
    }
    let three = praise_rem(p.variant.wrapping_add(p.player), 3);
    match p.kind.wrapping_sub(0x29) {
        0 => {
            let t = match three {
                0 => 0xa39900,
                1 => 0xa39930,
                _ => 0xa3996c,
            };
            f(h, t, short);
        }
        1 => {
            let t = match three {
                0 => 0xa39864,
                1 => 0xa39894,
                _ => 0xa398cc,
            };
            f(h, t, short);
        }
        2 => {
            f(h, 0xa39708, short);
        }
        3 => {
            f(h, 0xa3996c, short);
        }
        4 => {
            f(h, 0xa399a0, short);
        }
        5 => {
            f(h, 0xa39a04, short);
        }
        6 => {
            f(h, 0xa39a3c, 0x64, short);
        }
        7 | 8 => {
            f(h, 0xa399d0, short);
        }
        9 => {
            f(h, 0xa39810, p.cg(), p.cg(), short, p.cname());
        }
        10 => {
            f(h, 0xa39734, short);
        }
        11 => {
            let (compg, compname) = praise_comp(p);
            let t = if praise_rem(p.variant.wrapping_mul(p.player), 0x11) > 6 { 0xa397c0 } else { 0xa39774 };
            f(h, t, compg, compg, short, compname);
        }
        _ => {
            f(h, 0xa396e0, short);
        }
    }
}

/// The links of the text (see the routine's header).
pub(crate) unsafe fn praise_links(p: &mut Praise, out: usize) {
    covered!("nma_4 praise links");
    let link = |name: usize, kind: u32, data: usize| {
        exe::text_add_link(out, name as *const u8, kind, data);
    };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let club_of = |i: i32| read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE));
    let national = |n: usize| exe::database_get_national_team(n);
    let rec = p.record.as_ptr() as usize;
    if p.has_match != 0 {
        // Match_Stats::fixture_entry_matches(index, club a, club b, day, year) (thiscall, ret 0x14; the dwords pushed)
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(rec + 0x1b), read_i32(rec + 0x11), read_i32(rec + 0x15), read_i32(rec), read_i32(rec + 2)) != 0 {
            link(p.at(0x4c), 0x7d3, read_usize(rec + 0x1b));
        }
        let c = club_of(read_i32(rec + 0x15));
        link(c + club::NAME, 0x7d5, c);
    }
    if p.player > -1 && p.player < read_i32(G_STAFF_COUNT)
        && exe::database_staff_matches_names(staff_of(p.player), p.names[0], p.names[1], p.names[2], 0usize) != 0
    {
        link(p.full(), 0x7d7, staff_of(p.player));
        link(p.short(), 0x7d7, staff_of(p.player));
    }
    if p.manager > -1
        && exe::database_staff_matches_names(staff_of(p.manager), p.manager_names[0], p.manager_names[1], p.manager_names[2], 0usize) != 0
    {
        link(p.at(praise_frame::MANAGER_FULL), 0x7d7, staff_of(p.manager));
        link(p.at(praise_frame::MANAGER_SHORT), 0x7d7, staff_of(p.manager));
    }
    if p.legend > -1
        && exe::database_staff_matches_names(staff_of(p.legend), p.legend_names[0], p.legend_names[1], p.legend_names[2], 0usize) != 0
    {
        link(p.lname_buf(), 0x7d7, staff_of(p.legend));
    }
    for c in [p.legend_club, p.rival_club] {
        if c != -1 {
            let r = Club(club_of(c));
            link(r.addr() + club::SHORT_NAME, 0x7d5, r.addr());
            link(r.addr() + club::NAME, 0x7d5, r.addr());
        }
    }
    if p.club_index != -1 {
        link(p.cname(), 0x7d5, p.club);
        link(p.lname(), 0x7d5, p.club);
        let n = p.nation();
        link(n + 4, 0x7d5, national(n));
        link(n + nation::SHORT_NAME, 0x7d5, national(n));
        for form in 0x16..=0x1f {
            exe::nation_get_nationality_adjective(n, p.adj(), 0x78, form);
            link(p.adj(), 0x7d5, national(n));
        }
    }
    for i in [p.nation_a, p.nation_b, p.nation_c] {
        if i != -1 {
            let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE));
            link(n + nation::SHORT_NAME, 0x7d5, national(n));
            link(n + 4, 0x7d5, national(n));
        }
    }
    if p.comp != -1 {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        link(c + competition::NAME, 0x7d0, c);
        link(c + competition::SHORT_NAME, 0x7d0, c);
    }
}

/// The bytes of a C string (empty for a null pointer).
pub(crate) unsafe fn nma4_c_text(p: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if p == 0 {
        return v;
    }
    let mut k = 0;
    while read_u8(p + k) != 0 {
        v.push(read_u8(p + k));
        k += 1;
    }
    v
}

/// The links of a Text object (`+4` the link list): the list's cursor (+0xa) starts at its head (+2) and each step
/// (as `0x68d1b0` does) takes the node's link (+0) and moves to the next (+6); a link holds its text at +0, its type at
/// +0xc8 and its data at +0xcc. The cursor is put back afterwards.
pub(crate) unsafe fn nma4_links(text: usize) -> Vec<(Vec<u8>, u32, u32)> {
    let mut v = Vec::new();
    let list = read_usize(text + 4);
    if list == 0 {
        return v;
    }
    let saved = read_u32(list + 0xa);
    write_u32(list + 0xa, read_u32(list + 2));
    loop {
        let node = read_usize(list + 0xa);
        if node == 0 {
            break;
        }
        let link = read_usize(node);
        write_u32(list + 0xa, read_u32(node + 6));
        v.push((nma4_c_text(link), read_u32(link + 0xc8), read_u32(link + 0xcc)));
    }
    write_u32(list + 0xa, saved);
    v
}

/// The headline of the transfer rumour: the player's short name (style 5) with "unsettled" / "fuels transfer talk"
/// (lead 36: by the player's id even or odd; "eyes ... move" when there is no bidding club and the first listed club
/// is a club), otherwise by the player's id % 3: 0 "transfer speculation", 1 "rumours" (more than one club) or
/// "rumour", 2 "... lead chase for ..." (more than one club, the first one's short name), "... linked with ..." (one
/// club) or "rumour". The name links to the player (`exe::database_staff_matches_names`, type 0x7d7), the bidding
/// club and every listed club (type 0x7d5).
pub(crate) unsafe fn transfer_rumour_headline(r: &Rumour, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour headline");
    TRANSFER_RUMOUR_PIECE.store(0, Relaxed);
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x190];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x190, 5, r.player() as u32);
    let p = player.as_ptr() as usize;
    let first = r.listed(0);
    if r.lead == 0x24 {
        if r.club == -1 && first > r.club {
            let sg = rumour_club_short_gender(first);
            format(RUMOUR_TEXT, 0xa53c94, sg, sg, p, rumour_club_short(first)); // "... eyes ... move"
        } else if r.player() % 2 != 0 {
            format(RUMOUR_TEXT, 0xa53c58, p); // "... fuels transfer talk"
        } else {
            format(RUMOUR_TEXT, 0xa53c24, p); // "... unsettled"
        }
    } else {
        match r.player() % 3 {
            0 => {
                format(RUMOUR_TEXT, 0xa53ae0, p); // "... transfer speculation"
            }
            1 => {
                format(RUMOUR_TEXT, if r.count > 1 { 0xa53b1c } else { 0xa53b4c }, p); // "... rumours" / "... rumour"
            }
            _ => {
                let sg = rumour_club_short_gender(first);
                if r.count > 1 {
                    format(RUMOUR_TEXT, 0xa53bd0, sg, sg, rumour_club_short(first), p); // "... lead chase for ..."
                } else if r.count == 1 {
                    format(RUMOUR_TEXT, 0xa53b7c, sg, sg, p, rumour_club_short(first)); // "... linked with ..."
                } else {
                    format(RUMOUR_TEXT, 0xa53b4c, p);
                }
            }
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, p as *const u8, 0x7d7, person); // nma5: the headline's player link
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The body lead of kind 0: the first listed club (without a bidding club), the bidding club and the first listed
/// club, or both with the fee. Returns the piece (1).
pub(crate) unsafe fn transfer_rumour_lead_kind0(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 0");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5441c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54398, g4, g4, g0, g0, n4, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54300, g4, g4, g0, g0, n4, player, fee, n0);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    1
}

/// The body lead of kind 1: the first listed club and the player (a foreign manager's lead names the nation by its
/// short name), with the bidding club, and with the fee (then the foreign lead uses the nationality adjective, form
/// 0x1c). Returns the piece (2).
pub(crate) unsafe fn transfer_rumour_lead_kind1(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 1");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let nation = Nation(r.nation);
    if r.fee() == 0 {
        if r.club == -1 {
            if r.local {
                format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa54a6c, sg, sg, g0, g0, nation.addr() + nation::SHORT_NAME, n0, player);
            }
        } else {
            let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
            if r.local {
                format(RUMOUR_TEXT, 0xa549ac, g0, g0, g4, g4, n0, n4, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa548d0, sg, sg, g0, g0, g4, g4, nation.addr() + nation::SHORT_NAME, n0, n4, player);
            }
        }
    } else if r.club == -1 {
        format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
    } else if r.local {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54808, g0, g0, g4, g4, n0, n4, player, fee);
    } else {
        let mut adjective = [0u8; 0x78];
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, 0x1c);
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54724, 0x64, 0x64, g0, g0, g4, g4, adjective.as_ptr() as usize, n0, n4, player, fee);
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    2
}

/// The body lead of kind 36: the first listed club and the player (with his description and its gender when there is
/// a bidding club, and the fee when there is one). Returns the piece (3).
pub(crate) unsafe fn transfer_rumour_lead_kind36(r: &Rumour, out: usize, player: usize, fee: usize, desc: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 36");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let gd = r.desc_gender;
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5504c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54f7c, g4, g4, gd, gd, g0, g0, n4, desc, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54e84, g4, g4, gd, gd, g0, g0, n4, desc, player, n0, fee);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    3
}

/// The body lead of the other kinds: like kind 0 with other texts. Returns the piece (4).
pub(crate) unsafe fn transfer_rumour_lead_other(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead other");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa55578, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa554a4, 0x64, g0, g0, g4, g4, player, n0, n4);
        } else {
            format(RUMOUR_TEXT, 0xa553f4, 0x64, g4, g4, g0, g0, player, fee, n4, n0); // nma5: the fee in the lead
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    4
}

/// The clubs paragraph of the body: with 2 to 5 listed clubs, the 2nd to the 5th by name (the texts by the lead
/// piece; with 2 clubs, kind 36 adds the player's description and the other kinds, for a foreign manager, the
/// nationality adjective, form 0x18), appended to the text.
pub(crate) unsafe fn transfer_rumour_clubs(r: &Rumour, out: usize, piece: u8, desc: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if !(2..=5).contains(&r.count) {
        TRANSFER_RUMOUR_PIECE.store(piece as u32, Relaxed);
        return;
    }
    covered!("nma_5 rumour clubs");
    TRANSFER_RUMOUR_PIECE.store(piece as u32 | (r.count as u32) << 8, Relaxed);
    let g = |i: usize| rumour_club_gender(r.listed(i));
    let n = |i: usize| rumour_club_name(r.listed(i));
    // the texts of 3, 4 and 5 clubs by piece (kind 0, kind 1, kind 36, other)
    let (t3, t4, t5) = match piece {
        1 => (0xa54224, 0xa54188, 0xa540c8),
        2 => (0xa54628, 0xa54578, 0xa544a4),
        3 => (0, 0xa54c90, 0xa54ba0),
        _ => (0xa55240, 0xa55194, 0xa550c0),
    };
    let gd = r.desc_gender;
    match r.count {
        2 => match piece {
            1 => {
                format(RUMOUR_TEXT, 0xa542a4, g(1), g(1), n(1));
            }
            2 => {
                format(RUMOUR_TEXT, 0xa546b8, g(1), g(1), n(1));
            }
            3 => {
                format(RUMOUR_TEXT, 0xa54df4, g(1), g(1), gd, gd, n(1), desc);
            }
            _ => {
                if r.local {
                    format(RUMOUR_TEXT, 0xa55370, g(1), g(1), n(1));
                } else {
                    let mut adjective = [0u8; 0x78];
                    exe::nation_get_nationality_adjective(r.nation, adjective.as_mut_ptr() as usize, 0x78, 0x18);
                    format(RUMOUR_TEXT, 0xa552cc, 0x64, g(1), g(1), adjective.as_ptr() as usize, n(1));
                }
            }
        },
        3 => {
            if piece == 3 {
                format(RUMOUR_TEXT, 0xa54d4c, gd, gd, g(1), g(1), g(2), g(2), desc, n(1), n(2));
            } else {
                format(RUMOUR_TEXT, t3, g(1), g(1), g(2), g(2), n(1), n(2));
            }
        }
        4 => {
            format(RUMOUR_TEXT, t4, g(1), g(1), g(2), g(2), g(3), g(3), n(1), n(2), n(3));
        }
        _ => {
            format(RUMOUR_TEXT, t5, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), n(1), n(2), n(3), n(4));
        }
    }
    rumour_append(out, RUMOUR_TEXT);
}

/// The agent line of the body, appended to the text (empty for the other kinds): when the agent is the human manager
/// of the current screen, by the agent line kind (slot 7): 1 the player's own words (two texts by the flag of slot
/// 14), 2 three texts (the flag, then without a fee and with a fee), 3 a fixed text; otherwise 1 and 2 quote the
/// agent (his full name) on the bidding club (its short name).
pub(crate) unsafe fn transfer_rumour_agent_line(r: &Rumour, out: usize, player_short: usize, agent: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut line = [0u8; 0x190];
    let l = line.as_mut_ptr() as usize;
    let human = r.agent() > -1
        && staff_at(r.agent() as isize).addr() == read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human {
        covered!("nma_5 rumour agent human");
        TRANSFER_RUMOUR_PIECE.fetch_or(1 << 16, Relaxed);
        match r.agent_kind() {
            1 => {
                if r.flag14 != 0 {
                    format(l, 0xa53e90, 0x64, player_short);
                } else {
                    format(l, 0xa53e20, player_short);
                }
            }
            2 => {
                if r.flag14 != 0 {
                    format(l, 0xa54038, player_short);
                } else if r.fee() == 0 {
                    format(l, 0xa53fb8, player_short);
                } else {
                    format(l, 0xa53f38, player_short);
                }
            }
            3 => {
                format(l, 0xa54094);
            }
            _ => {}
        }
    } else {
        covered!("nma_5 rumour agent quoted");
        TRANSFER_RUMOUR_PIECE.fetch_or(2 << 16, Relaxed);
        let k = r.agent_kind();
        if k == 1 || k == 2 {
            let sg = rumour_club_short_gender(r.club);
            format(l, if k == 1 { 0xa53ce0 } else { 0xa53d7c }, sg, sg, rumour_club_short(r.club), agent, player_short);
        }
    }
    rumour_append(out, l);
}

/// The links of the body: the player's two names (when `exe::database_staff_matches_names`; type 0x7d7), the agent's
/// full name (the same), the bidding club (type 0x7d5); with a nation, its name, short name and the ten forms of its
/// nationality adjective (0x16 to 0x1f) to its national team (`exe::database_get_national_team`); every listed club.
pub(crate) unsafe fn transfer_rumour_links(r: &Rumour, out: usize, player: usize, player_short: usize, agent: usize) {
    covered!("nma_5 rumour links");
    let (n1, n2, n3) = r.player_names();
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, player as *const u8, 0x7d7, person);
        exe::text_add_link(out, player_short as *const u8, 0x7d7, person);
    }
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        let a = staff_at(r.agent() as isize);
        if exe::database_staff_matches_names(a, a1, a2, a3, 0usize) != 0 {
            exe::text_add_link(out, agent as *const u8, 0x7d7, a);
        }
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    if r.nation != 0 {
        let n = Nation(r.nation);
        exe::text_add_link(out, (n.addr() + 4) as *const u8, 0x7d5, exe::database_get_national_team(n)); // the nation's name (+4)
        exe::text_add_link(out, (n.addr() + nation::SHORT_NAME) as *const u8, 0x7d5, exe::database_get_national_team(n));
        let mut adjective = [0u8; 0x78];
        for form in 0x16..=0x1fu32 {
            exe::nation_get_nationality_adjective(n, adjective.as_mut_ptr() as usize, 0x78, form);
            exe::text_add_link(out, adjective.as_ptr(), 0x7d5, exe::database_get_national_team(n));
        }
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc; walked like the copy constructor 0x4ae5c0 does).
pub(crate) unsafe fn nma5_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
    let text = |p: usize| if p == 0 { Vec::new() } else { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec() };
    let mut links = Vec::new();
    let l = read_usize(t + 4);
    if l != 0 {
        list_rewind(l);
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            links.push((text(e), read_u32(e + 0xc8), read_u32(e + 0xcc)));
        }
    }
    (text(read_usize(t)), links)
}

/// Delivers the team story `n` through `G_NEWS`: 1 to the club's manager, 2 to the continent's managers, 3 to the
/// division's, 4 to the nation's, 5 to all; in the rehearsal it only counts the delivery and keeps the news item.
pub(crate) unsafe fn team_story_deliver(n: usize, how: u8, target: usize) {
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_DELIVERIES.fetch_add(1, Relaxed);
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(TEAM_STORY_BUILT) as *mut u8, 0xe0);
        return;
    }
    let news = read_usize(G_NEWS);
    match how {
        1 => exe::news_send_to_club_manager(news, n, target),
        2 => exe::news_send_to_continent_managers(news, n, target),
        3 => exe::news_send_to_division_managers(news, n, target),
        4 => exe::news_send_to_nation_managers(news, n, target),
        _ => exe::news_send_to_all_managers(news, n),
    };
}

/// The delivery of the team story when it does not go to the club's manager alone (see `post_team_media_story`).
pub(crate) unsafe fn team_story_deliver_by_kind(n: usize, club: Club, k: u8, variant: i8) {
    if k == 0x5a || k == 0x44 {
        covered!("nma_5 team story to all");
        return team_story_deliver(n, 5, 0);
    }
    let continent = club.nation().continent();
    if k == 0x5c {
        return team_story_deliver(n, 2, continent);
    }
    let div = club.division();
    if !div.is_null() && club.reputation() >= 0x2328 && div.reputation() > 0xf {
        return team_story_deliver(n, 2, continent);
    }
    if !div.is_null() && div.reputation() > 0xf {
        let bonus = if variant > 0x14 { (variant as i32) * 30 + 1000 } else { 0 };
        if (club.reputation() as i32) > exe::utils_random(0xbb8) + 0x1f40 - bonus {
            if k == 0x50 || k == 0x51 {
                if variant > 0xf {
                    return team_story_deliver(n, 2, continent);
                }
                return team_story_deliver_in_country(n, club, div);
            }
            if [0x41, 0x62, 0x63, 0x61, 0x5d].contains(&k) {
                return team_story_deliver(n, 2, continent);
            }
        }
    }
    if k == 0x3d {
        if div.is_null() || exe::utils_random(3) != 0 {
            return team_story_deliver(n, 1, club.addr());
        }
        return team_story_deliver(n, 3, div.addr());
    }
    team_story_deliver_in_country(n, club, div)
}

/// The piece of `apply_player_story_response` a story kind goes to (the comparer's message).
pub(crate) fn player_story_response_piece(topic: u32) -> &'static str {
    match topic as u16 {
        0x25 => "position rival (0x25)",
        0x1f => "transfer interest (0x1f)",
        0x20 => "outburst (0x20)",
        _ => "other story",
    }
}

/// Story 0x25 (a position rival arrived: `warn_squad_of_position_rival`), the manager's answer to the player.
///
/// Answer 1: the supporters' confidence (+8 of the club's row in [[`G_MANAGER_MANAGER`]], 1..10000) +200 when the
/// club favours the player (`exe::club_favours_staff`), otherwise −50; with `.professionalism()` below 10 and
/// draw(professionalism) below 5, `.professionalism()` and `.temperament()` each go down by one (above 1 and with a
/// nonzero draw of their own); morale +10 (`exe::staff_adjust_morale`, limited); with a nonzero draw(temperament) and
/// the player disliking the manager (`exe::staff_is_disliked_by`), the manager leaves his disliked list
/// (`exe::staff_remove_disliked_staff`); with a nonzero draw(`.pressure()`), contract flag 0x800 is cleared.
///
/// Answer 2: supporters −50 when the club favours him, otherwise +200; `.professionalism()` above 10 and above
/// draw(20): the apology news (`media_queue_player_apology_news`, story 0x25) and nothing else; professionalism 6 or
/// more: nothing else. Otherwise, with the contract (looked up again) and draw(pressure) below draw(10), contract flag
/// 0x800 (`set_flags`); without it, pressure, determination and the player record's `.work_rate()` each go up by one
/// (below 20, on a zero draw of their own). Then morale −10; with the contract wanting to leave
/// (`exe::staff_contract_wants_to_leave`): loyalty −1 when draw(loyalty) is below draw(25) (above 1); on a zero
/// draw(loyalty), the manager goes into his disliked list (`exe::staff_add_disliked_staff`, forced) and the contract
/// gets event flags 8 and, on a zero draw(pressure), also 0x2000 (`exe::staff_contract_apply_event_flags` with 0).
///
/// Answer 3: supporters −100, board (+6) −50.
pub(crate) unsafe fn respond_to_position_rival_story(this: usize, player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 position rival");
    match response as i8 {
        1 => {
            covered!("nma_6 position rival backed");
            let v = if exe::club_favours_staff(player, club) != 0 { 200 } else { -50 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            if player.professionalism() < 10 && exe::utils_random(player.professionalism() as i32) < 5 {
                if player.professionalism() > 1 && exe::utils_random(player.professionalism() as i32) != 0 {
                    player.set_professionalism(player.professionalism().wrapping_sub(1));
                }
                if player.temperament() > 1 && exe::utils_random(player.temperament() as i32) != 0 {
                    player.set_temperament(player.temperament().wrapping_sub(1));
                }
            }
            exe::staff_adjust_morale(player, 10, 1);
            if exe::utils_random(player.temperament() as i32) != 0 && exe::staff_is_disliked_by(player, manager) != 0 {
                exe::staff_remove_disliked_staff(player, manager);
            }
            if exe::utils_random(player.pressure() as i32) == 0 {
                return;
            }
            if contract.flags() & 0x800 != 0 {
                exe::staff_contract_clear_flags(contract, 0x800);
            }
        }
        2 => {
            covered!("nma_6 position rival criticised");
            let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
            let v = if exe::club_favours_staff(player, club) != 0 { -50 } else { 200 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            let professionalism = player.professionalism();
            if professionalism > 10 && (professionalism as i32) > exe::utils_random(0x14) {
                if player_story_stops_before_news() {
                    return;
                }
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(player_story_fns::QUEUE_PLAYER_APOLOGY_NEWS);
                f(this, player.addr(), club.addr(), 0x25);
                return;
            }
            if professionalism >= 6 {
                return;
            }
            let mut flagged = false;
            if !c.is_null() {
                let a = exe::utils_random(player.pressure() as i32);
                if a < exe::utils_random(10) {
                    if set_contract_flags_or_stop(c.addr(), 0x800) {
                        return;
                    }
                    flagged = true;
                }
            }
            if !flagged {
                if player.pressure() < 0x14 && exe::utils_random(player.pressure() as i32) == 0 {
                    player.set_pressure(player.pressure().wrapping_add(1));
                }
                if player.determination() < 0x14 && exe::utils_random(player.determination() as i32) == 0 {
                    player.set_determination(player.determination().wrapping_add(1));
                }
                let sheet = player.player();
                if sheet.work_rate() < 0x14 && exe::utils_random(sheet.work_rate() as i32) == 0 {
                    sheet.set_work_rate(sheet.work_rate().wrapping_add(1));
                }
            }
            exe::staff_adjust_morale(player, -10i32 as u32, 1);
            if exe::staff_contract_wants_to_leave(c) == 0 {
                return;
            }
            if player.loyalty() > 1 {
                let a = exe::utils_random(player.loyalty() as i32);
                if a < exe::utils_random(0x19) {
                    player.set_loyalty(player.loyalty().wrapping_sub(1));
                }
            }
            if exe::utils_random(player.loyalty() as i32) != 0 {
                return;
            }
            exe::staff_add_disliked_staff(player, manager, 1);
            let r = exe::utils_random(player.pressure() as i32);
            if player_story_event_flags_or_stop(c, 8) {
                return;
            }
            if r == 0 {
                player_story_event_flags_or_stop(c, 0x2000);
            }
        }
        3 => {
            covered!("nma_6 position rival no comment");
            add_clamped_confidence(manager_manager_row(club) + 8, -100);
            add_clamped_confidence(manager_manager_row(club) + 6, -50);
        }
        _ => {}
    }
}

/// Story 0x1f (another club's interest in the player: `process_transfer_interest`), the manager's answer; each answer
/// walks the 50 squad slots (`.squad()`).
///
/// Answer 1 (keeps the player): for the player himself, supporters +200 and board +100 when the club favours him,
/// −100 and −100 when it dislikes him (`exe::club_dislikes_staff`); board +250 when he likes the chairman
/// (`exe::staff_is_favoured_by`), −250 when he dislikes him; morale +10 when his contract does not want to leave;
/// otherwise morale −10 and, with draw(loyalty) below draw(25): loyalty −1 on a zero draw (above 1), the manager into
/// his disliked list (forced) and event flags 8 (and 0x2000 on a zero draw(pressure)). For the others (a player record,
/// a valid id): morale +10 when the player likes them, −5 when he dislikes them, otherwise draw(5) − draw(5).
///
/// Answer 2 (lets him go): for the player, supporters −500 and board −200 when the club favours him, +500 and +200
/// when it dislikes him; board −500 when he likes the chairman, +200 when he dislikes him; morale +10 when his contract
/// wants to leave; otherwise morale −10 and, with draw(loyalty) below draw(25), loyalty −1 on a zero draw (above 1) and
/// the manager into his disliked list. The others: liked, morale professionalism / 2 − 15; disliked, +10; otherwise,
/// with draw(ambition) above draw(20) and draw(professionalism) below draw(20), −5.
///
/// Other answers: morale draw(8) − draw(8) for the player and draw(5) − draw(5) for the others.
pub(crate) unsafe fn respond_to_transfer_interest_story(player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 transfer interest");
    match response as i8 {
        1 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest kept");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 200); // nma6: kept, a favourite: supporters +200
                        add_clamped_confidence(manager_manager_row(club) + 6, 100);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -100);
                        add_clamped_confidence(manager_manager_row(club) + 6, -100);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 250);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -250);
                    }
                    if exe::staff_contract_wants_to_leave(contract) == 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1); // nma6: kept against his will: he dislikes the manager
                    let r = exe::utils_random(player.pressure() as i32);
                    if player_story_event_flags_or_stop(contract, 8) {
                        return;
                    }
                    if r == 0 && player_story_event_flags_or_stop(contract, 0x2000) {
                        return;
                    }
                } else if is_story_squad_member(p) {
                    let delta = if exe::staff_is_favoured_by(player, p) != 0 {
                        10
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        -5
                    } else {
                        let a = exe::utils_random(5);
                        (a as u8).wrapping_sub(exe::utils_random(5) as u8) as i8
                    };
                    exe::staff_adjust_morale(p, delta as u32, 1);
                }
            }
        }
        2 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest released");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -500); // nma6: released, a favourite: supporters -500
                        add_clamped_confidence(manager_manager_row(club) + 6, -200);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 500);
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -500);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_contract_wants_to_leave(contract) != 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1);
                } else if is_story_squad_member(p) {
                    if exe::staff_is_favoured_by(player, p) != 0 {
                        let half = (p.professionalism() as i32 / 2) as u8;
                        exe::staff_adjust_morale(p, half.wrapping_sub(0xf) as i8 as u32, 1);
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        exe::staff_adjust_morale(p, 10, 1);
                    } else {
                        let a = exe::utils_random(p.ambition() as i32);
                        if a > exe::utils_random(0x14) {
                            let b = exe::utils_random(p.professionalism() as i32);
                            if b < exe::utils_random(0x14) {
                                exe::staff_adjust_morale(p, -5i32 as u32, 1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            covered!("nma_6 transfer interest other answer");
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                let range = if p == player {
                    8
                } else if is_story_squad_member(p) {
                    5 // nma6: the squad's mood after the rumour
                } else {
                    continue;
                };
                let a = exe::utils_random(range);
                let delta = (a as u8).wrapping_sub(exe::utils_random(range) as u8);
                exe::staff_adjust_morale(p, delta as i8 as u32, 1);
            }
        }
    }
}

/// Story 0x20 (the player's outburst, `create_player_outburst_story` [inference]), the manager's answer.
///
/// Answer 1: supporters' target 500 when the club favours the player, −1500 when it dislikes him, otherwise −500
/// (`adjust_supporter_confidence`); board's target (`adjust_board_confidence`) trunc(w × 1000 − 2000) when he likes the
/// chairman, trunc(w × −1000 − 2000) when he dislikes him, otherwise −2000, where w = (chairman's `.ambition()` − 10)
/// × 0.05 + 1 (0.5 without a chairman); then the story effects with flags 0x100. Answer 2: supporters' target 3000 when
/// the club dislikes him, board's target trunc(w × 3000) when he dislikes the chairman; then the story effects with
/// flags 0x80. Other answers: nothing.
pub(crate) unsafe fn respond_to_outburst_story(this: usize, player: Staff, club: Club, chairman_weight: f64, stat: f32, response: u32) {
    covered!("nma_6 outburst");
    match response as i8 {
        1 => {
            if player_story_stops_before_news() {
                return;
            }
            let target = if exe::club_favours_staff(player, club) != 0 {
                500
            } else if exe::club_dislikes_staff(player, club) != 0 {
                -1500
            } else {
                -500
            };
            player_story_supporters(this, club, player, 0x20, target, response);
            let board = if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_1000) - read_f64(rdata::F64_2000))
            } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_NEG_1000) - read_f64(rdata::F64_2000))
            } else {
                -2000
            };
            player_story_board(this, club, player, 0x20, board, response);
            player_story_effects(this, club, 0x20, 0x100, stat, response, player);
        }
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            if exe::club_dislikes_staff(player, club) != 0 {
                player_story_supporters(this, club, player, 0x20, 3000, response);
            }
            if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                let board = ftol(chairman_weight * read_f64(player_story_fns::F64_3000));
                player_story_board(this, club, player, 0x20, board, response);
            }
            player_story_effects(this, club, 0x20, 0x80, stat, response, player);
        }
        _ => {}
    }
}

/// The other stories (neither 0x25, 0x1f nor 0x20): the story effects (`Media::apply_story_effect_flags`, weight = the
/// club's stat 3 in its division) with flags 0x500 for answer 1, 0x404 for answer 3 and 0x480 for answer 2; with
/// answer 2, the player of a kind 0..0x23 that passes his complaint test (`player_story_complaint_due`) first files a
/// complaint (`DISPUTE::file_complaint`, reason 4, and 1 on a zero draw(4)). Other answers: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn respond_to_other_player_story(this: usize, player: Staff, club: Club, contract: Contract, age: i8, stat: f32, topic: u32, response: u32) {
    covered!("nma_6 other story");
    let flags = match response as i8 {
        1 => 0x500, // nma6: the flags of answer 1
        3 => 0x404,
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            let k = topic as i16 as i32 as u32;
            let case = if k > 0x23 { 2 } else { PLAYER_STORY_COMPLAINT_CASE[k as usize] };
            if case != 2 && player_story_complaint_due(player, contract, age, case == 1) {
                covered!("nma_6 other story complaint");
                let reason_b = (exe::utils_random(4) == 0) as u32;
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(player_story_fns::FILE_COMPLAINT);
                f(read_usize(G_DISPUTE), player.addr(), club.addr(), 4, reason_b);
            }
            0x480
        }
        _ => return,
    };
    if player_story_stops_before_news() {
        return;
    }
    player_story_effects(this, club, topic, flags, stat, response, player);
}

/// Notes the piece that is running.
pub(crate) fn player_story_piece(name: &'static str) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().push(name));
}

/// The C remainder of `a % b` (signed, the sign of `a`), as the original's `idiv` and `and 0x80000003` fix-ups give.
pub(crate) fn praise_rem(a: i32, b: i32) -> i32 {
    a.wrapping_rem(b)
}

/// Kind 0x2e, "Big move close for {player}?": by variant, a pundit praising the club's player, the press speculating
/// about a move to a bigger club, a national newspaper calling him a big fish in a small pond, a club touting him, or a
/// television pundit heaping praise ("{nationality} ..." abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x745855..0x745ba9).
pub(crate) unsafe fn praise_body_big_move(p: &mut Praise) {
    covered!("nma_4 praise big move");
    p.piece = "body_big_move";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    match p.variant {
        0 => {
            f(h, 0xa433f8, cg, cg, p.pg(), p.pg(), cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa432e4, 0x64, cg, cg, g, g, full, cname, pos);
            } else {
                p.describe_keep();
                p.adjective(0x1c);
                f(h, 0xa431b0, 0x64, 0x64, 0x64, cg, cg, p.pg(), p.pg(), full, adj, cname, pos);
            }
        }
        2 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa430a4, g, g, cg, cg, pos, full, cname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa42f78, 0x64, g, g, cg, cg, adj, pos, full, cname);
            }
        }
        3 => {
            let g = p.describe();
            f(h, 0xa42e7c, cg, cg, g, g, cname, full, pos);
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa42d4c, cg, cg, 0x64, 0x64, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1f);
                let g = p.describe();
                f(h, 0xa42c00, 0x64, cg, cg, 0x64, g, g, adj, cname, full, short, pos);
            }
        }
    }
}

/// Kind 0x30, "{player} world class talent": by variant, a former great (with his club and job, or his nation) or a
/// world cup winning manager saying the player will be a world star; variant 1 the same about his form this season;
/// the others, a household name throughout the world.
/// Original: part of `Media::write_player_praise_news_text` (0x745ba9..0x745ed9).
pub(crate) unsafe fn praise_body_world_class(p: &mut Praise) {
    covered!("nma_4 praise world class");
    p.piece = "body_world_class";
    let f = Praise::format();
    let (h, pos, full, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa426ac, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 3) != 0 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42598, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa424b4, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa429cc, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 13) > 7 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa428a8, cg, cg, pg, pg, 0x64, 0x64, ng, ng, 0x64, 0x64, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa427c4, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa42af4, cg, cg, g, g, cname, full, pos);
        }
    }
}

/// Kind 0x31, "{player} world class talent" (the second set): by variant, a former great or a world cup winning
/// manager saying the player is world class (with % 11 < 6 for the great's nation), or the player named among the
/// best in a world poll.
/// Original: part of `Media::write_player_praise_news_text` (0x745ed9..0x7461de).
pub(crate) unsafe fn praise_body_world_class_b(p: &mut Praise) {
    covered!("nma_4 praise world class b");
    p.piece = "body_world_class_b";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa41f78, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa41e78, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa41d94, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa42268, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42158, cg, cg, pg, pg, 0x64, ng, ng, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa42084, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            f(h, 0xa42384, cg, cg, 0x64, cname, full, short);
        }
    }
}

/// Kind 0x2d, "{player} on the rise": a media pundit's belief (variant 0), a national paper's report on his strengths
/// (variant 1), or a television pundit's praise (the others), "{nationality} ..." abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x7461de..0x746414).
pub(crate) unsafe fn praise_body_on_the_rise(p: &mut Praise) {
    covered!("nma_4 praise on the rise");
    p.piece = "body_on_the_rise";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let (lname, lg) = (p.lname(), p.lg());
    match p.variant {
        0 => {
            let g = p.describe();
            f(h, 0xa417cc, cg, cg, g, g, cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41a04, g, g, 0x64, lg, lg, pos, full, lname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa418c0, 0x64, g, g, 0x64, lg, lg, adj, pos, full, lname);
            }
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41c6c, cg, cg, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1c);
                let g = p.describe();
                f(h, 0xa41b28, cg, cg, g, g, 0x64, 0x64, cname, full, short, pos, adj);
            }
        }
    }
}

/// Kind 0x29, "{player} rated highly / a 'future international' / set for call-up?": by variant, an outlet praising
/// him for the national team (variant 0), the club's player of the nation (1), a former great of that nation joining
/// the experts who want him called up (2), or a first cap in the offing (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746414..0x746851).
pub(crate) unsafe fn praise_body_future_international(p: &mut Praise) {
    covered!("nma_4 praise future international");
    p.piece = "body_future_international";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa40f08, og, og, cg, cg, g, g, 0x64, nag, nag, outlet, cname, pos, full, na);
            } else {
                f(h, 0xa40df0, og, og, cg, cg, g, g, 0x64, outlet, cname, pos, full);
            }
        }
        1 => {
            let pg = p.pg();
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa4115c, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
            } else {
                p.adjective(0x1c);
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa41044, cg, cg, pg, pg, 0x64, 0x64, nag, nag, cname, pos, full, adj, na);
            }
        }
        2 => {
            let pg = p.pg();
            if p.legend > -1 && read_i32(p.legend_nation()) == p.nation_a {
                if p.legend_club > -1 {
                    let job = p.legend_job_name();
                    let (na, nag) = praise_nation_a(p);
                    let lc = Club(p.legend_club_rec());
                    let lcg = (lc.short_gender_name() as i8) as i32;
                    let jg = p.jg();
                    f(h, 0xa41444, lcg, lcg, jg, jg, cg, cg, pg, pg, nag, nag, lc.addr() + club::SHORT_NAME, job, legend, cname, pos, full, na);
                } else {
                    let (na, nag) = praise_nation_a(p);
                    f(h, 0xa41334, cg, cg, pg, pg, nag, nag, legend, cname, pos, full, na);
                }
                return;
            }
            let (na, nag) = praise_nation_a(p);
            f(h, 0xa41254, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa416b0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = Nation(p.nation());
                let ng = (n.short_gender_name() as i8) as i32;
                f(h, 0xa4157c, cg, cg, g, g, ng, ng, cname, pos, full, n.addr() + nation::SHORT_NAME);
            }
        }
    }
}

/// Kind 0x2a, "{player} shows promise / talented {player} given boost / youngster {player} praised": by variant, an
/// outlet praising the youngster (0), the club's young player (1, "{nationality}" abroad), an exciting future (2), or a
/// player type with a future (the others), with the club's nation abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x746851..0x746ac4).
pub(crate) unsafe fn praise_body_youngster(p: &mut Praise) {
    covered!("nma_4 praise youngster");
    p.piece = "body_youngster";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let og = p.outlet_gender as i8 as i32;
            f(h, 0xa406d0, og, og, cg, cg, p.pg(), p.pg(), 0x64, outlet, cname, pos, full);
        }
        1 => {
            if p.home {
                f(h, 0xa408c0, cg, cg, p.pg(), p.pg(), cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa407e4, cg, cg, p.pg(), p.pg(), 0x64, 0x64, cname, pos, full, adj);
            }
        }
        2 => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40aa0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40978, cg, cg, g, g, ng, ng, cname, pos, full, n + nation::SHORT_NAME);
            }
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40cdc, g, g, cg, cg, pos, full, cname);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40bb0, g, g, ng, ng, cg, cg, pos, full, n + nation::SHORT_NAME, cname);
            }
        }
    }
}

/// Kind 0x32, "{player} impresses at {club}": an outlet praising his impact (variant 0), an outlet on his type
/// (1), or his performances since joining (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746ac4..0x746bd1).
pub(crate) unsafe fn praise_body_impresses(p: &mut Praise) {
    covered!("nma_4 praise impresses");
    p.piece = "body_impresses";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    match p.variant {
        0 => {
            f(h, 0xa40608, og, og, cg, cg, outlet, cname, full);
        }
        1 => {
            let g = p.describe();
            f(h, 0xa40530, og, og, g, g, cg, cg, outlet, pos, full, cname);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa403ec, cg, cg, g, g, full, cname, pos, short);
        }
    }
}

/// Kind 0x34, "{player} lights up / stars in {competition}": the player's part in a match of a competition (the match
/// record of slot 17): the score (higher first, "%d-%d") and the stage label (`comp_util_format_comp_stage_round_label`,
/// "Friendly" and "Reserve" with a small first letter; gender `comp_util_get_label_gender`). By variant and whether the
/// club won (record +0x19 above +0x1a): a paragraph on the match, then, with a rival club, a second one (the player due
/// to sign for it on a date when value 1 is set; attracting interest while playing in a nation when value 4 is set;
/// playing for it otherwise); after a draw or a defeat, value 11 adds how it ended (won or lost on penalties or a
/// shootout). Without the match record: the original's error box (media.cpp line 0x4743) and an empty paragraph.
/// Original: part of `Media::write_player_praise_news_text` (0x746bd1..0x7483d2).
pub(crate) unsafe fn praise_body_competition(p: &mut Praise) {
    covered!("nma_4 praise competition");
    p.piece = "body_competition";
    let f = Praise::format();
    let h = p.head();
    if p.has_match == 0 {
        covered!("nma_4 praise competition without match");
        f(h, 0xad9c64); // ""
        praise_error_box(p, 0x4743);
        return;
    }
    p.describe_keep();
    let rec = p.record.as_ptr() as usize;
    let (a, b) = (read_i8(rec + 0x19), read_i8(rec + 0x1a));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let score = p.at(0x4c);
    sprintf(score, 0x98c114, a.max(b) as i32, a.min(b) as i32); // "%d-%d"
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(rec + 8) as usize).wrapping_mul(0x6b));
    let stage = p.at(praise_frame::EXTRA);
    exe::comp_util_format_comp_stage_round_label(comp, read_u32(rec + 0xc), read_u32(rec + 0xe), 1, stage as *mut u8, 0xc8);
    if praise_same_text(stage, 0x9db958) {
        write_u8(stage, b'f'); // "Friendly"
    } else if praise_same_text(stage, 0x9b5b40) {
        write_u8(stage, b'r'); // "Reserve"
    }
    p.job_gender = exe::comp_util_get_label_gender(comp as u32, read_u32(rec + 0xc), read_u32(rec + 0xe), 1);
    let rc = Club(read_usize(G_CLUB_TABLE).wrapping_add((read_u32(rec + 0x15) as usize).wrapping_mul(club::SIZE)));
    let m = PraiseMatch { rg: rc.gender_name() as i32, rname: rc.addr() + club::NAME, sg: p.jg(), score, stage };
    let won = a > b;
    let ended = match p.variant {
        0 => praise_competition_v0(p, &m, won),
        1 => praise_competition_v1(p, &m, won),
        2 => praise_competition_v2(p, &m, won),
        _ => praise_competition_other(p, &m, won),
    };
    if ended {
        return;
    }
    // after a draw or a defeat: how it ended
    let tail = match p.m11 {
        1 => 0xa21e0c, // won after a penalty shootout
        2 => 0xa21684, // lost on penalties
        3 => 0xa21dc0, // won after a shootout
        4 => 0xa21d74, // lost on a shootout
        _ => return,
    };
    covered!("nma_4 praise competition shootout");
    let (cg, cname, r) = (p.cg(), p.cname(), p.reply());
    f(r, tail, cg, cg, cname);
    praise_strcat(h, r);
}

/// The competition of slot 16 (club comp table + index × 0x6b): its short-name gender byte and short name.
pub(crate) unsafe fn praise_comp(p: &Praise) -> (i32, usize) {
    let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
    (read_i8(c + competition::SHORT_GENDER_NAME) as i32, c + competition::SHORT_NAME)
}

/// Kind 0x33, "{player} tipped to shine on world stage": the player in a competition (slot 16) with two nations
/// (values 9 and 10): by variant, an outlet praising his performances (0), a former international star or a former
/// great with his club and job predicting he will shine (1 and 2; variant 2 names the great's nation when he has no
/// club, and a great's tip without one), or "{nationality} player type" (the others, adjective form 0xc). The nations
/// are left out when one of them is the club's nation, and the second one when either is missing. With a rival club,
/// the paragraph is then kept as the first sentence of one about his form for that club (when value 1 is 0 and the
/// great % 3 is not 0) or about signing for it on a date (value 1 set).
/// Original: part of `Media::write_player_praise_news_text` (0x7483d2..0x74948e).
pub(crate) unsafe fn praise_body_world_stage(p: &mut Praise) {
    covered!("nma_4 praise world stage");
    p.piece = "body_world_stage";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    let (compg, compname) = praise_comp(p);
    let cid = read_i32(p.nation());
    let own = p.nation_b == cid || p.nation_c == cid;
    let both = p.nation_b > -1 && p.nation_c > -1;
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if own {
                if p.home {
                    f(h, 0xa3c97c, og, og, g, g, compg, compg, outlet, pos, full, compname);
                } else {
                    f(h, 0xa3ca9c, og, og, cg, cg, g, g, 0x64, compg, compg, outlet, cname, pos, full, compname);
                }
            } else if both {
                if p.home {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3ce7c, og, og, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, pos, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3cfdc, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, cname, pos, full, compname, nbname, ncname);
                }
            } else if p.home {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cbe4, og, og, g, g, 0x64, compg, compg, nbg, nbg, outlet, pos, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cd1c, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, outlet, cname, pos, full, compname, nbname);
            }
        }
        1 => {
            p.describe();
            if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d25c, lcg, lcg, jg, jg, cg, cg, 0x64, compg, compg, lcname, job, legend, cname, full, compname);
                } else {
                    f(h, 0xa3d164, cg, cg, 0x64, compg, compg, legend, cname, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d6d8, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d5a4, cg, cg, 0x64, compg, compg, nbg, nbg, ncg, ncg, legend, cname, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d47c, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, lcname, job, legend, cname, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d370, cg, cg, compg, compg, nbg, nbg, legend, cname, full, compname, nbname);
            }
        }
        2 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend == -1 {
                f(h, 0xa3e034, cg, cg, pg, pg, compg, compg, legend, cname, pos, full, compname);
            } else if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d970, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, lcname, job, legend, cname, pos, full, compname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa3d828, lng, lng, cg, cg, pg, pg, 0x64, compg, compg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dec0, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, pos, full, compname, nbname, ncname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dd50, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dc04, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, lcname, job, legend, cname, pos, full, compname, nbname);
            } else {
                let ln = p.legend_nation();
                let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dabc, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname);
            }
        }
        _ => {
            p.adjective(0xc);
            let pg = p.pg();
            if own {
                f(h, 0xa3e120, 0x64, 0x64, pg, pg, compg, compg, adj, pos, full, compname);
            } else if both {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                f(h, 0xa3e2e8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, ncg, ncg, adj, pos, full, compname, nbname, ncname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3e1f8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, adj, pos, full, compname, nbname);
            }
        }
    }
    if p.rival_club <= -1 {
        return;
    }
    let r = p.reply();
    let rc = Club(read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE)));
    let rlg = rc.gender_name() as i32;
    if p.m1 == 0 {
        if praise_rem(p.legend, 3) == 0 {
            return;
        }
        covered!("nma_4 praise world stage form");
        praise_strcpy(r, h);
        f(h, 0xa3c8f0, rlg, rlg, r, p.short(), rc.addr() + club::NAME);
    } else {
        covered!("nma_4 praise world stage signing");
        let date = praise_date_text(p);
        praise_strcpy(r, h);
        f(h, 0xa3c860, rlg, rlg, 0x64, r, p.short(), rc.addr() + club::NAME, date);
    }
}

/// Kind 0x2b, "{player} praised": by variant 0..3, an outlet praising his form (strength 0x1000000) or journalists
/// hailing him ("in {nation}" abroad), a club's player of the nation, an outlet lost for words ("{nationality} pundit"
/// abroad), an outlet lavishing praise. Other variants go by the strength flags when variant % 4 is not 0: 0x2c0040
/// lethal striker, 0x240040 striker of the competition (long name), 0xc0040 hotshot, 0x40040 striker of the
/// competition, 0x102040, 0x200840 and 0x840 strikers; otherwise the description is kept and, with strength
/// 0x1000000, the media praise him ("{nationality} media" abroad), else the variant becomes club % 4 (C remainder) and
/// the choice starts again (a negative club index with none of these flags loops for ever, as in the original).
/// Original: part of `Media::write_player_praise_news_text` (0x74948e..0x749b63).
pub(crate) unsafe fn praise_body_praised(p: &mut Praise) {
    covered!("nma_4 praise praised");
    p.piece = "body_praised";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    let s = p.strengths;
    let nation = |p: &Praise| {
        let n = Nation(p.nation());
        ((n.short_gender_name() as i8) as i32, n.addr() + nation::SHORT_NAME)
    };
    let comp_long = |p: &Praise| {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        (read_i8(c + competition::GENDER_NAME) as i32, c + competition::NAME)
    };
    let mut v = p.variant;
    loop {
        match v {
            0 => {
                if s & 0x100_0000 != 0 {
                    f(h, 0xa3c754, og, og, cg, cg, 0x64, 0x64, outlet, cname, full, short);
                } else if p.home {
                    f(h, 0xa3c654, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c538, ng, ng, cg, cg, p.pg(), p.pg(), 0x64, nname, cname, pos, full);
                }
                return;
            }
            1 => {
                if p.home {
                    f(h, 0xa3c480, cg, cg, cname, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c3c8, cg, cg, ng, ng, cname, full, nname);
                }
                return;
            }
            2 => {
                if p.home {
                    f(h, 0xa3c300, og, og, cg, cg, outlet, cname, full);
                } else {
                    p.adjective(0x1f);
                    f(h, 0xa3c240, 0x64, cg, cg, adj, cname, full);
                }
                return;
            }
            3 => {
                f(h, 0xa3c164, og, og, cg, cg, outlet, cname, full);
                return;
            }
            _ => {}
        }
        let odd = praise_rem(v, 4) != 0;
        if s & 0x2c0040 == 0x2c0040 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3c030, cg, cg, 0x64, ng, ng, cname, full, short, nname);
            return;
        }
        if s & 0x240040 == 0x240040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bee0, cg, cg, 0x64, clg, clg, cname, full, short, clname);
            return;
        }
        if s & 0xc0040 == 0xc0040 && odd {
            if p.home {
                f(h, 0xa3bdf4, cg, cg, cname, full);
            } else {
                p.adjective(0x16);
                f(h, 0xa3bcfc, cg, cg, 0x64, cname, full, adj);
            }
            return;
        }
        if s & 0x40040 == 0x40040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bc10, cg, cg, clg, clg, cname, full, clname);
            return;
        }
        if s & 0x102040 == 0x102040 && odd {
            if p.home {
                f(h, 0xa3bb60, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3ba90, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x200840 == 0x200840 && odd {
            if p.home {
                f(h, 0xa3b9e0, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3b910, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x840 == 0x840 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3b7d0, cg, cg, ng, ng, cname, full, short, nname);
            return;
        }
        p.describe_keep();
        if s & 0x100_0000 != 0 {
            if p.home {
                f(h, 0xa3b6cc, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa3b5a8, 0x64, 0x64, cg, cg, p.pg(), p.pg(), 0x64, adj, cname, pos, full);
            }
            return;
        }
        v = praise_rem(p.club_index, 4);
    }
}

/// Kind 0x2f, "{player} attitude praised": a media pundit's belief (variant 0) or a national newspaper columnist's.
/// Original: part of `Media::write_player_praise_news_text` (0x749b63..0x749c05).
pub(crate) unsafe fn praise_body_attitude(p: &mut Praise) {
    covered!("nma_4 praise attitude");
    p.piece = "body_attitude";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    if p.variant != 0 {
        f(h, 0xa3b48c, cg, cg, g, g, full, cname, pos);
    } else {
        f(h, 0xa3b378, g, g, cg, cg, pos, full, cname);
    }
}

/// Kind 0x2c (and any kind out of 0x29..0x34), "{player} set for call-up?": national papers lavishing praise, with
/// the rumour of a call-up to the nation of slot 7 ("in {nation}" abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x749c05..0x749d35).
pub(crate) unsafe fn praise_body_call_up(p: &mut Praise) {
    covered!("nma_4 praise call up");
    p.piece = "body_call_up";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    let (na, nag) = praise_nation_a(p);
    if p.home {
        f(h, 0xa3b258, cg, cg, g, g, nag, nag, cname, full, pos, na);
    } else {
        let n = Nation(p.nation());
        let ng = (n.short_gender_name() as i8) as i32;
        f(h, 0xa3b118, ng, ng, cg, cg, g, g, nag, nag, n.addr() + nation::SHORT_NAME, cname, full, pos, na);
    }
}

/// The club name (`club::NAME`) of club `id` of the club table.
pub(crate) unsafe fn rumour_club_name(id: i32) -> usize {
    club_at(id as isize).addr() + club::NAME
}

/// The grammatical gender byte of the club name (`club::GENDER_NAME`), sign-extended.
pub(crate) unsafe fn rumour_club_gender(id: i32) -> i32 {
    club_at(id as isize).gender_name() as i32
}

/// The short club name (`club::SHORT_NAME`).
pub(crate) unsafe fn rumour_club_short(id: i32) -> usize {
    club_at(id as isize).addr() + club::SHORT_NAME
}

/// The grammatical gender byte of the short club name (`club::SHORT_GENDER_NAME`), sign-extended.
pub(crate) unsafe fn rumour_club_short_gender(id: i32) -> i32 {
    club_at(id as isize).short_gender_name() as i8 as i32
}

/// Appends the text buffer to the Text object (`Text::append_string`, 0x4ae770, thiscall, ret 4).
pub(crate) unsafe fn rumour_append(out: usize, s: usize) {
    exe::text_append_string(out, s);
}

/// The team story inside the club's country: without a division to the club's manager; below reputation 5000 to the
/// division's managers; otherwise to the nation's managers, and then an English club from 6000 also to the Scottish
/// managers, a Scottish one from 6500 to the English ones, the others to the club's manager.
pub(crate) unsafe fn team_story_deliver_in_country(n: usize, club: Club, div: Competition) {
    if div.is_null() {
        return team_story_deliver(n, 1, club.addr());
    }
    if club.reputation() < 0x1388 {
        return team_story_deliver(n, 3, div.addr());
    }
    covered!("nma_5 team story to the nation");
    team_story_deliver(n, 4, club.nation().addr());
    let id = read_u32(club.nation().addr());
    let nation_record = |key: usize| read_usize(G_NATION_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(nation::SIZE));
    if id == read_u32(KEY_NATION_ENGLAND) && club.reputation() >= 0x1770 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_SCOTLAND));
    } else if id == read_u32(KEY_NATION_SCOTLAND) && club.reputation() >= 0x1964 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_ENGLAND));
    } else {
        team_story_deliver(n, 1, club.addr());
    }
}

/// In the comparer's rehearsal (`MEDIA_REACTION_DRY_RUN`) a call that may post news is not made: the rehearsal stops
/// there (`MEDIA_REACTION_STOPPED`) and the comparison falls back on the original alone. Outside it: false.
pub(crate) unsafe fn player_story_stops_before_news() -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    false
}

/// STAFF_CONTRACT::apply_event_flags(marks, 0) (`exe::staff_contract_apply_event_flags`); in the rehearsal it stops
/// first when the person would request a transfer (`would_request_transfer`).
pub(crate) unsafe fn player_story_event_flags_or_stop(c: Contract, marks: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) && would_request_transfer(c, marks) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    exe::staff_contract_apply_event_flags(c, marks, 0);
    false
}

/// A squad member the squad loops of `respond_to_transfer_interest_story` reach: not null, with a player record and an
/// id below the staff count.
pub(crate) unsafe fn is_story_squad_member(p: Staff) -> bool {
    !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
}

/// `Media::apply_story_effect_flags(club, kind, flags, weight, response, 0, player)`: the effects of the story's outcome.
pub(crate) unsafe fn player_story_effects(this: usize, club: Club, kind: u32, flags: u32, weight: f32, response: u32, player: Staff) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, usize) = core::mem::transmute(player_story_fns::APPLY_STORY_EFFECT_FLAGS);
    f(this, club.addr(), kind, flags, weight.to_bits(), response, 0, player.addr())
}

/// `Media::adjust_supporter_confidence(club, player, kind, target, response)`.
pub(crate) unsafe fn player_story_supporters(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_SUPPORTER_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response)
}

/// `Media::adjust_board_confidence(club, player, kind, target, response, 0)`.
pub(crate) unsafe fn player_story_board(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_BOARD_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response, 0)
}

/// The player's complaint test of answer 2 to the other stories (`PLAYER_STORY_COMPLAINT_CASE`): a player record with
/// `.current_reputation()` above 6750, `.world_reputation()` above 3750 (the general test also takes a contract
/// `.happiness()` of 100), `.current_ability()` above 120, aged 20 to 33, `.ambition()` above 10, the contract with
/// flags or grievances, its `.morale()` below −25 and wanting to leave, and a zero draw(loyalty). The general test asks
/// `.professionalism()` below 10, `.loyalty()` below 6 and `.temperament()` below 10; the strict one,
/// `.professionalism()` below 6, `.pressure()` above 10 and loyalty or temperament below 6.
pub(crate) unsafe fn player_story_complaint_due(player: Staff, contract: Contract, age: i8, strict: bool) -> bool {
    let sheet = player.player();
    if sheet.is_null() || sheet.current_reputation() <= 0x1a5e {
        return false;
    }
    if sheet.world_reputation() <= 0xea6 && (strict || contract.happiness() != 0x64) {
        return false;
    }
    if sheet.current_ability() <= 0x78 || age <= 0x13 || age >= 0x22 || player.ambition() <= 10 {
        return false;
    }
    let character = if strict {
        player.professionalism() < 6 && player.pressure() > 10 && (player.loyalty() < 6 || player.temperament() < 6)
    } else {
        player.professionalism() < 10 && player.loyalty() < 6 && player.temperament() < 10
    };
    if !character {
        return false;
    }
    if contract.flags() == 0 && contract.grievances() == 0 {
        return false;
    }
    if contract.morale() >= -25 || exe::staff_contract_wants_to_leave(contract) == 0 {
        return false;
    }
    exe::utils_random(player.loyalty() as i32) == 0
}

/// The great player's lines of kinds 0x30 and 0x31: with his club, (club gender, job gender, his club's gender,
/// his club's name, his job, his name); returns None without a club.
pub(crate) unsafe fn praise_legend_club(p: &mut Praise) -> Option<(i32, usize, usize)> {
    if p.legend_club <= -1 {
        return None;
    }
    let job = p.legend_job_name();
    let lc = Club(p.legend_club_rec());
    Some(((lc.short_gender_name() as i8) as i32, lc.addr() + club::SHORT_NAME, job))
}

/// The nation of slot 7 (nation table + index × `nation::SIZE`), its name gender byte and name.
pub(crate) unsafe fn praise_nation_a(p: &Praise) -> (usize, i32) {
    let n = Nation(read_usize(G_NATION_TABLE).wrapping_add((p.nation_a as usize).wrapping_mul(nation::SIZE)));
    (n.addr() + 4, (n.gender_name() as i8) as i32)
}

/// strcat(dst, src).
pub(crate) unsafe fn praise_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// strcmp(a, b) == 0.
pub(crate) unsafe fn praise_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// The date of values 2 and 3 as a long date without the year (`Date::get_long_date_string_no_year`, 0x78, no time of
/// day) into the frame at +0xc58.
pub(crate) unsafe fn praise_date_text(p: &Praise) -> usize {
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, p.m2 as u32, p.m3 as u32);
    let out = p.at(praise_frame::DATE_TEXT);
    exe::date_get_long_date_string_no_year(date.as_mut_ptr() as usize, out, 0x78, 0xff);
    out
}

/// The original's assertion box: the file and line (`crt` splitpath of the media.cpp path), the version, "Error"
/// (`gui_show_message_box`), and the mouse button released.
pub(crate) unsafe fn praise_error_box(p: &Praise, line: u32) {
    let (name, ext) = (p.at(praise_frame::SPLIT_NAME), p.at(praise_frame::SPLIT_EXT));
    exe::crt_strtok(0xa17bec, 0, 0, name as *mut u8, ext as *mut u8);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(0xae24d0, 0x9870e8, exe::utils_get_version_string(), name, ext, line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Variant 0 of kind 0x34 (the player shone in the match). Returns true when the paragraph is complete (a win).
pub(crate) unsafe fn praise_competition_v0(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg) = (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        let (first, second) = match (won, p.m1 != 0, p.m4 != 0) {
            (true, true, _) => (0xa3eb20, 0xa3ea64),
            (true, false, true) => (0xa3eb20, 0xa3e980),
            (true, false, false) => (0xa3eb20, 0xa3e8c8),
            (false, true, _) => (0xa3e6b4, 0xa3e5f4),
            (false, false, true) => (0xa3e6b4, 0xa3e54c),
            (false, false, false) => (0xa3e6b4, 0xa3e8c8),
        };
        f(h, first, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
        if second == 0xa3e54c {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, rv.name);
        } else {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, short, rv.name);
        }
        praise_strcat(h, r);
    } else {
        let t = if won { 0xa3e77c } else { 0xa3e400 };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, full, cname, m.score, m.stage, m.rname, pos, short);
    }
    won
}

/// Variant 1 of kind 0x34 (the player starred in the match). Returns true after a win.
pub(crate) unsafe fn praise_competition_v1(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, pg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3f420, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, 0x64, full, cname, m.score, m.stage, m.rname, rv.name, short);
        } else if !won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3ee64, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, pg, pg, full, cname, m.score, m.stage, m.rname, rv.name, pos);
        } else {
            let t = match (won, p.m1 != 0) {
                (true, true) => 0xa3f5f8,
                (true, false) => 0xa3f2d0,
                (false, true) => 0xa3f050,
                (false, false) => 0xa3ed14,
            };
            f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, full, cname, m.score, m.stage, m.rname, rv.name);
        }
    } else {
        let t = if won { 0xa3f1a4 } else { 0xa3ebe4 };
        f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
    }
    won
}

/// Variant 2 of kind 0x34 (after helping the club to the score). Returns true after a win.
pub(crate) unsafe fn praise_competition_v2(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, full, short, cname, cg) = (p.head(), p.full(), p.short(), p.cname(), p.cg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if p.m1 != 0 {
            let t = if won { 0xa3fd7c } else { 0xa3f9a8 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, full, short, rv.name);
        } else {
            let t = if won { 0xa3fc3c } else { 0xa3f868 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, rv.name, full);
        }
    } else {
        let t = if won { 0xa3fb20 } else { 0xa3f74c };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, cname, m.score, m.stage, m.rname, full);
    }
    won
}

/// The other variants of kind 0x34 (after the club saw off the challenge / played out the score). Returns true after
/// a win.
pub(crate) unsafe fn praise_competition_other(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg, legend) =
        (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg(), p.lname_buf());
    let first = |h: usize| {
        if won {
            f(h, 0xa40290, cg, cg, m.rg, m.rg, m.sg, m.sg, pg, pg, 0x64, 0x64, cname, m.rname, m.score, m.stage, pos, full, legend);
        } else {
            f(h, 0xa3ff94, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, 0x64, 0x64, cname, m.score, m.stage, m.rname, pos, full, legend);
        }
    };
    if p.rival_club <= -1 {
        first(h);
        return won;
    }
    let rv = praise_rival(p);
    if p.m1 != 0 {
        let date = praise_date_text(p);
        first(h);
        f(r, 0xa40214, 0x64, rv.rgs, rv.rgs, 0x64, short, rv.name, date);
    } else {
        first(h);
        if p.m4 != 0 && won {
            f(r, 0xa40168, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else if p.m4 != 0 {
            f(r, 0xa3fef0, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else {
            f(r, 0xa400e8, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        }
    }
    praise_strcat(h, r);
    won
}

/// A nation by index (nation table + index × `nation::SIZE`, unchecked as in the original): its short-name gender
/// byte and short name.
pub(crate) unsafe fn praise_nation_by_index(i: i32) -> (i32, usize) {
    let n = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE)));
    ((n.short_gender_name() as i8) as i32, n.addr() + nation::SHORT_NAME)
}

/// strcpy(dst, src).
pub(crate) unsafe fn praise_strcpy(dst: usize, src: usize) {
    write_u8(dst, 0);
    praise_strcat(dst, src);
}

pub(crate) unsafe fn praise_rival(p: &Praise) -> PraiseRival {
    let r = Club(read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE)));
    let n = r.nation();
    PraiseRival {
        rgs: (r.short_gender_name() as i8) as i32,
        name: r.addr() + club::SHORT_NAME,
        rng: (n.short_gender_name() as i8) as i32,
        nation_name: n.addr() + nation::SHORT_NAME,
    }
}

/// The media.cpp path for the error box.
pub(crate) const MEDIA_CPP: usize = 0xa17bec;

/// The ids of the media response screen: a response (data 1..3), Ok and Cancel.
pub(crate) const MEDIA_RESPONSE_ID: u32 = 0x1485;

pub(crate) const MEDIA_RESPONSE_OK: u32 = 0x109b;

pub(crate) const MEDIA_RESPONSE_CANCEL: u32 = 0x109c;

/// The news kinds the media response screen tells apart (the news item's first dword).
pub(crate) const MEDIA_NEWS_PLAYER_STORY: u32 = 0xbdd;

pub(crate) const MEDIA_NEWS_TEAM_STORY: u32 = 0xbdf;

pub(crate) const MEDIA_NEWS_SENT_OFF: u32 = 0xbe6;

pub(crate) const MEDIA_NEWS_WANTS_FIRST_TEAM: u32 = 0xbe8;

pub(crate) const MEDIA_NEWS_RIVAL_SIGNING: u32 = 0xfbc;

/// How the media response screen's question reads, by the story code (parameter 2): with the person's name
/// (`Name`), without it (`Plain`), with it when there is a person and without otherwise (`NameOrPlain`), with 100
/// and the name (`Hundred`), or with the club (gender char +0x52, name +0x38: `Club` with and without the name,
/// `ClubOrNation` with the name or else the club's division nation).
#[derive(Clone, Copy)]
pub(crate) enum MediaQuestion {
    Name(usize),
    Plain(usize),
    NameOrPlain(usize, usize),
    Hundred(usize),
    Club(usize, usize),
    ClubOrNation(usize, usize),
}

/// The locals of the original's body path (the slots of the news item, the names written once and the buffers).
pub(crate) struct PlayerStory {
    pub(crate) news: usize,
    /// slot 0: the player (staff index)
    pub(crate) player: i32,
    /// slots 1..3: the player's name ids
    pub(crate) names: [i32; 3],
    /// slot 4: the player's club (club index; -1 none)
    pub(crate) club: i32,
    /// slot 5: the variant of the story
    pub(crate) subtype: i32,
    /// slot 6: the outlet type (1..6, other: website)
    pub(crate) outlet: i32,
    /// slot 7 (unused by the text)
    pub(crate) slot7: i32,
    /// slot 8 (unused by the text)
    pub(crate) slot8: i32,
    /// slot 9: the story kind
    pub(crate) kind: i32,
    /// slot 10: the reader paragraph's case
    pub(crate) reader_case: i32,
    /// slot 11: the second person's index (-1 none)
    pub(crate) second: i32,
    /// slots 12..14: the second person's name ids
    pub(crate) second_names: [i32; 3],
    /// slot 14 read again (the original asks for 14 twice and never for 15)
    pub(crate) slot14_again: i32,
    pub(crate) slot16: i32,
    pub(crate) slot17: i32,
    pub(crate) slot18: i32,
    /// slot 23: the third person (staff index; the human manager when it is him)
    pub(crate) third: i32,
    /// slots 24..26: the third person's name ids
    pub(crate) third_names: [i32; 3],
    pub(crate) slot27: i32,
    /// slot 28: the story comes with a match (fixture record from slot 29)
    pub(crate) with_fixture: i32,
    /// the fixture record (date, ..., +0x15 the other club, +0x19 and +0x1a the goals)
    pub(crate) fixture: [u8; 0x20],
    /// with a fixture, the slot after it (the result code 1..4); without one, never written
    pub(crate) result: i32,
    /// the player's qualities (for `write_player_description`)
    pub(crate) qualities: i32,
    /// the player's position mask (for `write_position_name` and the description)
    pub(crate) position: i32,
    /// the pundit's id (word; -1 none)
    pub(crate) pundit: i32,
    /// the position mask for `position_mask_get_text`
    pub(crate) position_mask: i32,
    pub(crate) byte_3272: u8,
    pub(crate) byte_3271: u8,
    /// the fourth person (-1 none) and his name ids
    pub(crate) fourth: i32,
    pub(crate) fourth_names: [i32; 3],
    /// the player's offset in the staff table and the third person's
    pub(crate) player_off: usize,
    pub(crate) third_off: usize,
    /// the player's club is in the nation of the human manager on screen (the "local" media)
    pub(crate) local: u32,
    /// the outlet's grammar mark
    pub(crate) mark: u8,
    /// the gender byte of the position name (only the low byte is set; the original passes the dword on)
    pub(crate) position_gender: u8,
    /// the gender byte of the position mask text
    pub(crate) mask_gender: u8,
    pub(crate) name: [u8; 0xc8],
    pub(crate) name5: [u8; 0x78],
    pub(crate) third_name: [u8; 0xf0],
    pub(crate) third_name5: [u8; 0x78],
    pub(crate) second_name: [u8; 0xc8],
    pub(crate) fourth_name: [u8; 0x78],
    pub(crate) position_text: [u8; 0xc8],
    pub(crate) mask_text: [u8; 0x200],
    pub(crate) outlet_name: [u8; 0x200],
    pub(crate) adjective: [u8; 0x80],
    pub(crate) score: [u8; 0x38],
    pub(crate) text: [u8; 0x1388],
    pub(crate) extra: [u8; 0x400],
    pub(crate) long_extra: [u8; 0x1588],
}

impl PlayerStory {
    /// The club record of the story (`G_CLUB_TABLE` + `club::SIZE` × index).
    pub(crate) unsafe fn club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.club as usize).wrapping_mul(club::SIZE))
    }

    /// The club's short name and its gender byte (sign extended).
    pub(crate) unsafe fn club_short(&self) -> (usize, i32) {
        let c = self.club_rec();
        (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32)
    }

    /// The club's nation record (`club::NATION`, a pointer after the load).
    pub(crate) unsafe fn club_nation(&self) -> usize {
        read_usize(self.club_rec() + club::NATION)
    }

    /// The other club of the fixture (+0x15).
    pub(crate) unsafe fn other_club(&self) -> usize {
        let i = u32::from_le_bytes([self.fixture[0x15], self.fixture[0x16], self.fixture[0x17], self.fixture[0x18]]);
        read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
    }

    /// The two goal counts of the fixture (+0x19, +0x1a), signed bytes.
    pub(crate) fn goals(&self) -> (i8, i8) {
        (self.fixture[0x19] as i8, self.fixture[0x1a] as i8)
    }

    /// The nationality adjective of the club's nation into `adjective` (`exe::nation_get_nationality_adjective`, size
    /// 0x78, the form given).
    pub(crate) unsafe fn club_adjective(&mut self, form: u32) {
        exe::nation_get_nationality_adjective(self.club_nation(), self.adjective.as_mut_ptr() as usize, 0x78, form);
    }

}

impl PlayerStory {
    /// Reads the slots, writes the names and the position texts, and works out whether the media is local.
    pub(crate) unsafe fn read(news: usize) -> PlayerStory {
        let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
        let mut st: PlayerStory = core::mem::zeroed();
        st.news = news;
        exe::date_date_54a110(st.fixture.as_mut_ptr() as usize);
        st.player = s(0);
        st.names = [s(1), s(2), s(3)];
        st.club = s(4);
        st.subtype = s(5);
        st.outlet = s(6);
        st.slot7 = s(7);
        st.slot8 = s(8);
        st.kind = s(9);
        st.reader_case = s(10);
        st.second = s(11);
        st.second_names = [s(12), s(13), s(14)];
        st.slot14_again = s(14); // the original asks for slot 14 twice (and never for 15)
        st.slot16 = s(16);
        st.slot17 = s(17);
        st.slot18 = s(18);
        st.third = s(23);
        st.third_names = [s(24), s(25), s(26)];
        st.slot27 = s(27);
        st.with_fixture = s(28);
        if st.with_fixture != 0 {
            let read_fixture: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(PLAYER_STORY_READ_FIXTURE);
            let mut counter: u8 = 0x1d;
            read_fixture(news, &mut counter, st.fixture.as_mut_ptr() as usize);
            let mut next = || {
                let k = counter;
                counter = counter.wrapping_add(1);
                s(k as i8)
            };
            st.result = next();
            st.qualities = next();
            st.position = next();
            st.pundit = next();
            st.position_mask = 0;
            st.byte_3272 = 0;
            st.byte_3271 = 0;
            st.fourth = -1;
            st.fourth_names = [-1; 3];
        } else {
            st.qualities = s(0x1d);
            st.position = s(0x1e);
            st.pundit = s(0x1f);
            st.position_mask = s(0x20);
            st.byte_3272 = s(0x21) as u8;
            st.byte_3271 = s(0x22) as u8;
            st.fourth = s(0x23);
            st.fourth_names = [s(0x24), s(0x25), s(0x26)];
        }
        let mask_text: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
        st.mask_gender = mask_text(st.position_mask as u32, st.mask_text.as_mut_ptr() as usize, 0);
        st.player_off = (st.player as usize).wrapping_mul(staff::SIZE);
        st.third_off = (st.third as usize).wrapping_mul(staff::SIZE);
        let staff_table = read_usize(G_STAFF_TABLE);
        let [a, b, c] = st.names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name.as_mut_ptr(), 0x78, 1, st.player as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name5.as_mut_ptr(), 0x78, 5, st.player as u32);
        let [a, b, c] = st.third_names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name.as_mut_ptr(), 0x78, 1, st.third as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name5.as_mut_ptr(), 0x78, 5, st.third as u32);
        // the original passes a fourth argument (1) the routine does not read
        let position_name: unsafe extern "C" fn(u32, usize, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_POSITION_NAME);
        st.position_gender = position_name(st.position as u32, st.position_text.as_mut_ptr() as usize, 0xc8, 1);
        if staff_table.wrapping_add(st.third_off) == staff_table.wrapping_add(st.player_off) {
            story_lang!(st.position_text.as_mut_ptr(), 0xa242e0); // "player-manager"
            st.position_gender = 0;
        }
        if st.second != -1 {
            // the player's index goes with the second person's names (the original's choice)
            let [a, b, c] = st.second_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.second_name.as_mut_ptr(), 0x78, 1, st.player as u32);
        } else {
            st.second_name[0] = 0;
        }
        if st.fourth == -1 {
            st.fourth_name[0] = 0;
        } else {
            let [a, b, c] = st.fourth_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.fourth_name.as_mut_ptr(), 0x78, 1, st.fourth as u32);
        }
        // the media is local when the player's club is in the nation of the human manager on screen: his club's nation,
        // or his own nation without a club
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        st.local = 0;
        if manager != 0 {
            let job = read_usize(manager + staff::CLUB_JOB);
            let nation = st.club_nation();
            if job != 0 {
                if read_usize(job + club::NATION) == nation {
                    st.local = 1;
                }
            } else if nation == read_usize(manager + staff::NATION) {
                st.local = 1;
            }
        }
        st
    }

    /// The outlet's name into `outlet_name` and its grammar mark: the pundit's name when the media is local and the
    /// story names one; otherwise the outlet type (local: "national newspaper"...; foreign: with the nation's adjective).
    pub(crate) unsafe fn write_outlet(&mut self) {
        player_story_piece("outlet");
        covered!("nma_3 outlet");
        let nation = self.club_nation();
        if self.club != -1 && self.outlet != -1 && self.pundit as i16 != -1 && nation != 0 && self.local != 0 {
            covered!("nma_3 pundit");
            let pundit_name: unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8) = core::mem::transmute(PLAYER_STORY_GET_PUNDIT_NAME);
            pundit_name(read_usize(G_PUNDIT_LIST), self.outlet as u32, self.pundit as u32, self.outlet_name.as_mut_ptr() as usize, &mut self.mark);
            return;
        }
        let out = self.outlet_name.as_mut_ptr();
        let (plain, form, foreign) = match self.outlet {
            1 => (0xa242cc, 0x16, 0xa24294), // national newspaper
            2 => (0xa24284, 0x17, 0xa24250), // local newspaper
            3 => (0xa24238, 0x18, 0xa24208), // national radio station
            4 => (0xa241f4, 0x18, 0xa241bc), // local radio station
            5 => (0xa241b0, 0x19, 0xa24184), // magazine
            6 => (0xa24174, 0x1a, 0xa24144), // sports paper
            _ => (0xa2413c, 0x1a, 0xa24110), // website
        };
        if self.local != 0 {
            story_lang!(out, plain);
        } else {
            self.club_adjective(form);
            story_lang!(out, foreign, 0x64, self.adjective.as_ptr());
        }
        self.mark = if self.outlet == 5 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
    }

    /// Kind 0x23: the player is fed up with being left out.
    pub(crate) unsafe fn left_out(&mut self) {
        player_story_piece("left out (0x23)");
        covered!("nma_3 left out");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 if self.local != 0 => story_lang!(text, 0xa23d38, g, g, 0x64, short, name),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa23c40, g, g, 0x64, 0x64, 0x64, short, name, self.adjective.as_ptr());
            }
            1 => story_lang!(text, 0xa23e0c, g, g, 0x64, short, name),
            2 => story_lang!(text, 0xa23efc, g, g, 0x64, short, name),
            _ => story_lang!(text, 0xa23ffc, mark, mark, g, g, name, self.outlet_name.as_ptr(), short),
        }
    }

    /// Kind 0xe: the media questions the player's ability.
    pub(crate) unsafe fn ability(&mut self) {
        player_story_piece("ability (0xe)");
        covered!("nma_3 ability");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let foreign = |st: &mut PlayerStory, form: u32, tpl: usize| {
            st.club_adjective(form);
            story_lang!(text, tpl, 0x64, 0x64, g, g, st.adjective.as_ptr(), name, short);
        };
        match self.subtype {
            0 if local => story_lang!(text, 0xa23b98, 0x64, g, g, name, short),
            0 => foreign(self, 0x16, 0xa23ad0),
            1 if local => story_lang!(text, 0xa23a20, 0x64, g, g, name, short),
            1 => foreign(self, 0x16, 0xa2395c),
            2 if local => story_lang!(text, 0xa2389c, 0x64, g, g, name, short),
            2 => foreign(self, 0x1e, 0xa237bc),
            3 => story_lang!(text, 0xa236ec, 0x64, g, g, name, short),
            _ => story_lang!(text, 0xa23628, g, g, 0x64, short, name),
        }
    }

    /// Kind 0xd: the player is accused of complacency.
    pub(crate) unsafe fn complacency(&mut self) {
        player_story_piece("complacency (0xd)");
        covered!("nma_3 complacency");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 => story_lang!(text, 0xa232c0, mark, mark, g, g, 0x64, outlet, short, name),
            1 => story_lang!(text, 0xa233b0, mark, mark, 0x64, g, g, outlet, name, short),
            2 => story_lang!(text, 0xa23474, mark, mark, 0x64, g, g, outlet, name, short),
            _ => story_lang!(text, 0xa23558, g, g, 0x64, short, name),
        }
    }

    /// The score of the fixture, the higher number first ("%d-%d").
    pub(crate) unsafe fn write_score(&mut self) {
        let (a, b) = self.goals();
        story_crt!(self.score.as_mut_ptr(), 0x98c114, a.max(b) as i32, a.min(b) as i32);
    }

    /// Kind 0x11: the media questions the player's contribution (after a match, with the score; without one, the
    /// supporters speak out).
    pub(crate) unsafe fn contribution(&mut self) {
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.with_fixture == 0 {
            player_story_piece("fan criticism (0x11 without a match)");
            covered!("nma_3 fan criticism");
            story_lang!(text, 0xa21cb4, g, g, 0x64, short, name);
            return;
        }
        player_story_piece("contribution (0x11)");
        covered!("nma_3 contribution");
        self.write_score();
        let description: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_PLAYER_DESCRIPTION);
        let dg = description(self.player, self.position as u32, self.qualities as u32, self.position_text.as_mut_ptr() as usize, 0xc8, self.position_gender as u32, 1) as i8 as i32;
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let c2name = c2 + club::NAME;
        let c2g = read_i8(c2 + club::GENDER_NAME) as i32;
        let (score, pos, outlet, name5) = (self.score.as_ptr(), self.position_text.as_ptr(), self.outlet_name.as_ptr(), self.name5.as_ptr());
        let nation = self.club_nation();
        let (nshort, ng) = (nation + nation::SHORT_NAME, read_i8(nation + nation::SHORT_GENDER_NAME) as i32);
        let local = self.local != 0;
        let by_score = |win: usize, draw: usize, loss: usize| if g1 > g2 { win } else if g1 == g2 { draw } else { loss };
        match self.subtype {
            0 => {
                let mark = self.mark as i8 as i32;
                self.slot27 = g;
                let tpl = by_score(0xa22094, 0xa21f7c, 0xa21e60);
                story_lang!(text, tpl, g, g, c2g, c2g, mark, mark, 0x64, short, score, c2name, outlet, name);
            }
            1 => {
                self.slot27 = g;
                if local {
                    let tpl = by_score(0xa22820, 0xa2258c, 0xa222fc);
                    story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, 0x64, short, score, c2name, pos, name);
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    let tpl = by_score(0xa226d4, 0xa2243c, 0xa221a8);
                    story_lang!(text, tpl, ng, ng, g, g, c2g, c2g, dg, dg, 0x64, nshort, short, score, c2name, pos, name);
                }
            }
            _ => {
                self.slot27 = g;
                if local {
                    if g1 > g2 {
                        story_lang!(text, 0xa23154, g, g, c2g, c2g, dg, dg, name, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22e34 } else { 0xa22b14 };
                        story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, name, short, score, c2name, pos);
                    }
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    if g1 > g2 {
                        story_lang!(text, 0xa22fa4, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22c84 } else { 0xa22964 };
                        story_lang!(text, tpl, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, score, c2name, pos);
                    }
                }
            }
        }
        self.draw_result();
    }

    /// After a drawn match (kinds 0x11 and 0xa): how the club eventually won or lost (the result code), appended.
    pub(crate) unsafe fn draw_result(&mut self) {
        let (g1, g2) = self.goals();
        if g1 != g2 {
            return;
        }
        let tpl = match self.result {
            1 => 0xa21e0c,
            2 => 0xa21684,
            3 => 0xa21dc0,
            4 => 0xa21d74,
            _ => return,
        };
        player_story_piece("draw result");
        covered!("nma_3 draw result");
        let (short, g) = self.club_short();
        story_lang!(self.extra.as_mut_ptr(), tpl, g, g, short);
        story_strcat(self.text.as_mut_ptr() as usize, self.extra.as_ptr() as usize);
    }

    /// Kind 0xc: the media questions the player's commitment.
    pub(crate) unsafe fn commitment(&mut self) {
        player_story_piece("commitment (0xc)");
        covered!("nma_3 commitment");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa21bdc, mark, mark, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x13: the player's indiscipline (with bit 1 of the position mask, without the position text).
    pub(crate) unsafe fn indiscipline(&mut self) {
        player_story_piece("indiscipline (0x13)");
        covered!("nma_3 indiscipline");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        if self.position_mask as u8 & 1 == 1 {
            story_lang!(text, 0xa21acc, mark, mark, g, g, outlet, short, name);
        } else {
            let mg = self.mask_gender as i8 as i32;
            story_lang!(text, 0xa21998, mark, mark, g, g, mg, outlet, short, name, self.mask_text.as_ptr());
        }
    }

    /// Kind 0xa with a match: the player's contribution in the match, with the score and the other club.
    pub(crate) unsafe fn contribution_after_match(&mut self) {
        player_story_piece("contribution after a match (0xa)");
        covered!("nma_3 contribution after a match");
        self.write_score();
        let (short, g) = self.club_short();
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let (c2name, c2g) = (c2 + club::NAME, read_i8(c2 + club::GENDER_NAME) as i32);
        let mark = self.mark as i8 as i32;
        let tpl = if g1 > g2 { 0xa21834 } else if g1 == g2 { 0xa216d0 } else { 0xa2151c };
        story_lang!(self.text.as_mut_ptr(), tpl, mark, mark, g, g, c2g, c2g, self.outlet_name.as_ptr(), short, self.name.as_ptr(),
            self.score.as_ptr(), c2name, self.third_name.as_ptr());
        if g1 == g2 {
            self.draw_result();
        }
    }


    /// The record of a nation by index (`G_NATION_TABLE` + 0x122 × index): its name (+4) and the name's gender byte.
    pub(crate) unsafe fn nation_by_index(i: i32) -> (usize, usize, i32) {
        let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(0x122));
        (n, n + 4, read_i8(n + nation::GENDER_NAME) as i32)
    }

    /// The club's nation: its short name and the short name's gender byte.
    pub(crate) unsafe fn club_nation_short(&self) -> (usize, i32) {
        let n = self.club_nation();
        (n + nation::SHORT_NAME, read_i8(n + nation::SHORT_GENDER_NAME) as i32)
    }

    /// Kind 9: the media questions the purchase of the player (slot 27: the rival manager's club).
    pub(crate) unsafe fn purchase(&mut self) {
        player_story_piece("purchase (9)");
        covered!("nma_3 purchase");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let c3 = read_usize(G_CLUB_TABLE).wrapping_add((self.slot27 as usize).wrapping_mul(club::SIZE));
        let (c3short, c3g) = (c3 + club::SHORT_NAME, read_i8(c3 + club::SHORT_GENDER_NAME) as i32);
        let rival = self.second_name.as_ptr();
        match self.subtype {
            0 if local => story_lang!(text, 0xa213f4, c3g, c3g, g, g, c3short, rival, name, short),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa212ac, c3g, c3g, 0x64, 0x64, g, g, c3short, rival, self.adjective.as_ptr(), name, short);
            }
            1 => story_lang!(text, 0xa211e0, c3g, c3g, g, g, c3short, rival, short, name),
            2 if local => story_lang!(text, 0xa21124, g, g, name, short),
            2 => {
                self.club_adjective(0x16);
                story_lang!(text, 0xa21048, g, g, 0x64, name, short, self.adjective.as_ptr());
            }
            3 => story_lang!(text, 0xa20f8c, g, g, short, name),
            4 if local => story_lang!(text, 0xa20ec4, 0x64, g, g, name, short),
            4 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa20ddc, 0x64, g, g, 0x64, 0x64, name, short, self.adjective.as_ptr());
            }
            _ if local => story_lang!(text, 0xa20d20, g, g, name, short),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa20c48, ng, ng, g, g, nshort, name, short);
            }
        }
    }

    /// Kind 0x22: the player blasts his manager.
    pub(crate) unsafe fn blast_at_manager(&mut self) {
        player_story_piece("blast at manager (0x22)");
        covered!("nma_3 blast at manager");
        let (short, g) = self.club_short();
        let (text, name, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.third_name.as_ptr());
        if self.subtype != 0 {
            story_lang!(text, 0xa20b08, g, g, short, manager, name, self.third_name5.as_ptr());
        } else {
            story_lang!(text, 0xa209d0, g, g, 0x64, short, name, manager, self.name5.as_ptr());
        }
    }

    /// Kind 0x21: the player blames his manager for his spell of poor form.
    pub(crate) unsafe fn blames_manager(&mut self) {
        player_story_piece("blames manager (0x21)");
        covered!("nma_3 blames manager");
        let (short, g) = self.club_short();
        let (text, name, name5, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa203d0, 0x64, name, manager),
            1 => story_lang!(text, 0xa20494, g, g, 0x64, short, name, manager, name5),
            2 => story_lang!(text, 0xa205dc, 0x64, name, manager, name5, self.third_name5.as_ptr()),
            _ if self.local != 0 => story_lang!(text, 0xa2086c, g, g, 0x64, short, name, name5, manager),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa206e4, g, g, ng, ng, 0x64, short, name, nshort, name5, manager);
            }
        }
    }

    /// Kind 8: the media questions the player's attitude.
    pub(crate) unsafe fn attitude(&mut self) {
        player_story_piece("attitude (8)");
        covered!("nma_3 attitude");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype != 0 {
            story_lang!(text, 0xa202f8, g, mark, mark, g, 0x64, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa20244, g, g, 0x64, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa20174, ng, ng, g, g, 0x64, nshort, name, short);
        }
    }

    /// Kind 1: the player's private life in the papers (slot 7: a nation, slot 8: a city). Subtypes above 6 write
    /// nothing (the original leaves its text buffer as it was).
    pub(crate) unsafe fn private_life(&mut self) {
        player_story_piece("private life (1)");
        covered!("nma_3 private life");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        let slot8 = self.slot8;
        let city = || {
            let c = read_usize(G_CITY_TABLE).wrapping_add((slot8 as usize).wrapping_mul(0x38));
            (c + 4, read_i8(c + city::GENDER_NAME) as i32)
        };
        match self.subtype {
            0 => story_lang!(text, 0xa2006c, mark, mark, g, g, 0x64, outlet, name, short),
            1 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ff4c, mark, mark, 0x64, natg, natg, g, g, outlet, name, nname, short);
            }
            2 if local => story_lang!(text, 0xa1fe8c, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1fdb0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => {
                let (cname, cg) = city();
                story_lang!(text, 0xa1fcd0, cg, cg, name, cname);
            }
            3 => {
                self.club_adjective(0x1d);
                let (cname, cg) = city();
                story_lang!(text, 0xa1fbd4, 0x64, cg, cg, self.adjective.as_ptr(), name, cname);
            }
            4 | 6 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                let tpl = if self.subtype == 4 { 0xa1fb00 } else { 0xa1f8ec };
                story_lang!(text, tpl, mark, mark, g, g, natg, natg, outlet, short, name, nname);
            }
            5 => {
                let (n, _, _) = Self::nation_by_index(self.slot7);
                exe::nation_get_nationality_adjective(n, self.adjective.as_mut_ptr() as usize, 0x78, 2);
                story_lang!(text, 0xa1f9f0, mark, mark, g, g, 0x64, 0x64, outlet, short, name, self.adjective.as_ptr());
            }
            _ => {}
        }
    }

    /// Kind 0x25: the player is concerned by a transfer rumour.
    pub(crate) unsafe fn transfer_rumour(&mut self) {
        player_story_piece("transfer rumour (0x25)");
        covered!("nma_3 transfer rumour");
        let c = self.club_rec();
        let mark = self.mark as i8 as i32;
        let (cname, cg) = (c + club::NAME, read_i8(c + club::GENDER_NAME) as i32);
        let mg = self.mask_gender as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f7b4, mark, mark, 0x64, cg, cg, mg, 0x64, self.outlet_name.as_ptr(), self.name.as_ptr(), cname,
            self.mask_text.as_ptr(), self.fourth_name.as_ptr());
    }

    /// Kind 0x1e: an exclusive article about the player.
    pub(crate) unsafe fn exclusive(&mut self) {
        player_story_piece("exclusive (0x1e)");
        covered!("nma_3 exclusive");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype != 0 {
            if local {
                story_lang!(text, 0xa1f6d8, g, g, name, short);
            } else {
                self.club_adjective(0x19);
                story_lang!(text, 0xa1f5d8, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        } else if local {
            story_lang!(text, 0xa1f4f0, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x19);
            story_lang!(text, 0xa1f3e8, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 7: the player is too young?
    pub(crate) unsafe fn too_young(&mut self) {
        player_story_piece("too young (7)");
        covered!("nma_3 too young");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype == 0 {
            story_lang!(text, 0xa1f1b0, mark, mark, g, g, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa1f334, g, g, short, name);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1f268, ng, ng, g, g, nshort, short, name);
        }
    }

    /// Kind 2: the media raises a question about the player.
    pub(crate) unsafe fn question(&mut self) {
        player_story_piece("question (2)");
        covered!("nma_3 question");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f0f8, mark, mark, 0x64, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x20: rumours about the player (the supporters' club, the press).
    pub(crate) unsafe fn rumours(&mut self) {
        player_story_piece("rumours (0x20)");
        covered!("nma_3 rumours");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa1edf4, g, g, 0x64, short, name),
            1 if self.local != 0 => story_lang!(text, 0xa1efb4, g, g, name, short),
            1 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa1eef0, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1f058, g, g, name, short),
        }
    }

    /// Kind 3: the player is out of favour (slot 7: a nation).
    pub(crate) unsafe fn out_of_favour(&mut self) {
        player_story_piece("out of favour (3)");
        covered!("nma_3 out of favour");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        match self.subtype {
            0 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ecc4, mark, mark, g, g, natg, natg, outlet, name, short, nname);
            }
            1 => story_lang!(text, 0xa1ebd8, mark, mark, g, g, outlet, name, short),
            2 if local => story_lang!(text, 0xa1ead4, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e9b0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => story_lang!(text, 0xa1e8f0, g, g, name, short),
            3 => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e808, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1e72c, mark, mark, g, g, outlet, name, short),
        }
    }

    /// Kind 4: the player is past it?
    pub(crate) unsafe fn past_it(&mut self) {
        player_story_piece("past it (4)");
        covered!("nma_3 past it");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        match self.subtype {
            0 => story_lang!(text, 0xa1e3a4, 0x64, g, g, name, short),
            1 if local => story_lang!(text, 0xa1e504, name),
            1 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e448, ng, ng, nshort, name);
            }
            _ if local => story_lang!(text, 0xa1e678, g, g, name, short),
            _ => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e5a4, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        }
    }


    /// Kind 5: the pressure gets to the player.
    pub(crate) unsafe fn pressure(&mut self) {
        player_story_piece("pressure (5)");
        covered!("nma_3 pressure");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype == 0 {
            if local {
                story_lang!(text, 0xa1e150, g, g, name, short);
            } else {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e084, ng, ng, g, g, nshort, name, short);
            }
        } else if local {
            story_lang!(text, 0xa1e2e0, 0x64, g, g, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1e1fc, ng, ng, 0x64, g, g, nshort, name, short);
        }
    }

    /// Kind 6: the player's indiscipline costs the club dear.
    pub(crate) unsafe fn indiscipline_costs(&mut self) {
        player_story_piece("indiscipline costs (6)");
        covered!("nma_3 indiscipline costs");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1dfb4, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dec0, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 0x10: the player's poor performances and the club's poor run. The foreign text passes the club's gender
    /// byte where the club's name goes (a defect of the original, kept).
    pub(crate) unsafe fn poor_run(&mut self) {
        player_story_piece("poor run (0x10)");
        covered!("nma_3 poor run");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1de14, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dd44, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, g);
        }
    }

    /// Kind 0xa without a match and the kinds without a text: the programmers' placeholder (C runtime sprintf).
    pub(crate) unsafe fn placeholder(&mut self) {
        player_story_piece("placeholder (other kinds)");
        covered!("nma_3 placeholder");
        story_crt!(self.text.as_mut_ptr(), 0xa1dcbc, self.name.as_ptr());
    }

    /// Kinds 0 and 0x12: the media criticises the player. Subtypes 0..7 pick the text; above 7, the bits of slot 18
    /// pick it, and without one the subtype becomes player % 8 (written back to the subtype) and is tried again.
    pub(crate) unsafe fn criticism(&mut self) {
        player_story_piece("criticism (0, 0x12)");
        covered!("nma_3 criticism");
        let (short, g) = self.club_short();
        let (text, name, name5, outlet, manager) =
            (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.outlet_name.as_ptr(), self.third_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let flags = self.slot18 as u32;
        let local = self.local != 0;
        let has = |m: u32| flags & m == m;
        loop {
            match self.subtype as u32 {
                0 if local => story_lang!(text, 0xa1dbe0, g, g, 0x64, short, name),
                0 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1db04, ng, ng, g, g, 0x64, nshort, short, name);
                }
                1 => story_lang!(text, 0xa1da44, mark, mark, 0x64, g, g, outlet, name, short),
                2 => story_lang!(text, 0xa1d98c, g, g, 0x64, short, name),
                3 => story_lang!(text, 0xa1d8c8, g, g, 0x64, short, name),
                4 if local => story_lang!(text, 0xa1d838, g, g, 0x64, short, name),
                4 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1d78c, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, name);
                }
                5 if local => story_lang!(text, 0xa1d690, g, g, 0x64, 0x64, short, manager, name),
                5 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d578, ng, ng, g, g, 0x64, 0x64, nshort, short, manager, name);
                }
                6 if local => story_lang!(text, 0xa1d470, g, g, 0x64, short, manager, name),
                6 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d34c, ng, ng, g, g, 0x64, nshort, short, manager, name);
                }
                7 if local => story_lang!(text, 0xa1d20c, g, g, 0x64, short, manager, name),
                7 => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1d0a8, 0x64, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, manager, name);
                }
                _ if has(0x400200) => story_lang!(text, 0xa1cfd4, mark, mark, g, g, 0x64, outlet, name, short),
                _ if has(0x102200) => story_lang!(text, 0xa1ce94, mark, mark, 0x64, g, g, 0x64, outlet, name, short, name5),
                _ if has(0x104200) => story_lang!(text, 0xa1cd30, mark, mark, 0x64, g, g, outlet, name, short, name5),
                _ if has(0x100200) => story_lang!(text, 0xa1cc68, mark, mark, 0x64, g, g, outlet, name, short),
                _ if has(0x480) => story_lang!(text, 0xa1cb84, mark, mark, g, g, outlet, name, short),
                _ if has(0x500) => story_lang!(text, 0xa1c9ec, g, g, mark, mark, short, outlet, name),
                _ if has(0x20820) => story_lang!(text, 0xa1c8e8, 0x64, g, g, mark, mark, name, short, outlet),
                _ if has(0x1020) => story_lang!(text, 0xa1c7b4, 0x64, g, g, mark, mark, 0x64, name, short, outlet, name5),
                _ if has(0x20810) && local => story_lang!(text, 0xa1c6bc, 0x64, g, g, name, short),
                _ if has(0x20810) => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1c5b0, 0x64, g, g, 0x64, name, short, self.adjective.as_ptr());
                }
                _ if has(0x10001) && local => story_lang!(text, 0xa1c480, g, g, short, name, name5),
                _ if has(0x10001) => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1c334, g, g, ng, ng, short, name, nshort, name5);
                }
                _ if flags & 1 != 0 && local => story_lang!(text, 0xa1c204, g, g, short, name, name5),
                _ if flags & 1 != 0 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1c0b8, g, g, 0x64, short, name, self.adjective.as_ptr(), name5);
                }
                _ if flags as u8 & 0xa == 0xa => story_lang!(text, 0xa1bf7c, g, g, mark, mark, short, name, outlet, name5),
                _ if flags as u8 & 8 == 8 && local => story_lang!(text, 0xa1bed8, g, g, short, name),
                _ if flags as u8 & 8 == 8 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1be08, g, g, ng, ng, short, name, nshort);
                }
                _ if flags as u8 & 2 == 2 && local => story_lang!(text, 0xa1bcf8, g, g, 0x64, short, name, name5),
                _ if has(0x4000) => story_lang!(text, 0xa1bc50, g, g, short, name),
                _ if has(0x20000) => story_lang!(text, 0xa1bba0, g, g, short, name),
                _ if has(0x40000) && flags as u8 & 0x30 == 0 => story_lang!(text, 0xa1ba44, g, g, short, name, name5),
                _ => {
                    self.subtype = self.player % 8; // the C remainder (a negative player would loop for ever, as in the original)
                    continue;
                }
            }
            return;
        }
    }


    /// The kind dispatch of the story text (kind = slot 9).
    pub(crate) unsafe fn write_story(&mut self) {
        match self.kind {
            0x23 => self.left_out(),
            0xe => self.ability(),
            0xd => self.complacency(),
            0x11 => self.contribution(),
            0 | 0x12 => self.criticism(),
            0xc => self.commitment(),
            0x13 => self.indiscipline(),
            0xa if self.with_fixture != 0 => self.contribution_after_match(),
            9 => self.purchase(),
            0x22 => self.blast_at_manager(),
            0x21 => self.blames_manager(),
            8 => self.attitude(),
            1 => self.private_life(),
            0x25 => self.transfer_rumour(),
            0x1e => self.exclusive(),
            7 => self.too_young(),
            2 => self.question(),
            0x20 => self.rumours(),
            3 => self.out_of_favour(),
            4 => self.past_it(),
            5 => self.pressure(),
            6 => self.indiscipline_costs(),
            0x10 => self.poor_run(),
            _ => self.placeholder(),
        }
    }

    /// Kinds 0 and 0x12 with a subtype above 7 (picked by the bits of slot 18): the bits of a "bad" story.
    pub(crate) fn bad_flags(&self) -> bool {
        let f = self.slot18 as u32;
        [0x400200, 0x102200, 0x104200, 0x100200, 0x480, 0x500, 0x20820, 0x1020, 0x20810, 0x10001].iter().any(|&m| f & m == m) || f & 1 != 0
    }

    /// Kinds 0 and 0x12 with a subtype above 7, without the bad bits: the bits of a player coping with the pressure.
    pub(crate) fn coping_flags(&self) -> bool {
        let f = self.slot18 as u32;
        f as u8 & 0xa == 0xa || f as u8 & 8 == 8 || (f as u8 & 2 == 2 && self.local != 0)
    }

    /// Kinds 0 and 0x12 with a subtype above 7.
    pub(crate) fn criticism_by_flags(&self) -> bool {
        (self.kind == 0 || self.kind == 0x12) && self.subtype > 7
    }

    /// The parity of the subtype (C remainder by 2).
    pub(crate) fn subtype_even(&self) -> bool {
        self.subtype % 2 == 0
    }

    /// The reply about the player's role in the squad (slot 16: the role, slot 17: the age), by the human manager
    /// (`extra`, with the player's name in form 5).
    pub(crate) unsafe fn human_squad_role(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        match self.slot16 {
            1..=3 => story_lang!(extra, 0xa1af60),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa1b14c);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1b074, name5);
                } else {
                    story_lang!(extra, 0xa1b0d0, 0x64, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa1b008, name5),
            4 | 5 => story_lang!(extra, 0xa1aefc, name5),
            6 => story_lang!(extra, 0xa1ae98, name5),
            _ => story_lang!(extra, 0xa1b0d0, 0x64, name5),
        }
    }

    /// The same reply by the player's club manager (the third person, slot 23).
    pub(crate) unsafe fn club_manager_squad_role(&mut self) {
        let (extra, name5, manager) = (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.slot16 {
            1 => story_lang!(extra, 0xa19c1c, manager, name5),
            2 | 3 => story_lang!(extra, 0xa19b94, manager, name5),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa19b10, manager, name5);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1996c, manager, name5);
                } else {
                    story_lang!(extra, 0xa19a70, 0x64, manager, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa198d8, manager, name5),
            4 | 5 => story_lang!(extra, 0xa1984c, manager, name5),
            6 => story_lang!(extra, 0xa197c4, manager, name5),
            _ => story_lang!(extra, 0xa19a70, 0x64, manager, name5),
        }
    }

    /// The paragraph after the story (`extra`, slot 10 = the reply): when the third person is the human manager on
    /// screen, his reply (1 support, 2 criticise, 3 ignore); otherwise the club manager's (1 support, 2 criticise).
    pub(crate) unsafe fn write_reply(&mut self) {
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        let extra = self.extra.as_mut_ptr();
        if read_usize(G_STAFF_TABLE).wrapping_add(self.third_off) == manager {
            player_story_piece("manager reply");
            covered!("nma_3 manager reply");
            match self.reader_case {
                1 => self.human_supports(),
                2 => self.human_criticises(),
                3 => story_lang!(extra, 0xa1ba14), // "You have decided to ignore this news report."
                _ => story_crt!(extra, 0xad9c64),
            }
        } else {
            player_story_piece("club manager reply");
            covered!("nma_3 club manager reply");
            match self.reader_case {
                1 => self.club_manager_supports(),
                2 => self.club_manager_criticises(),
                _ => story_crt!(extra, 0xad9c64),
            }
        }
    }

    /// The human manager criticises the player (reply 2).
    pub(crate) unsafe fn human_criticises(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1b958);
            } else {
                story_lang!(extra, 0xa1b9b4, 0x64, name5);
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1b8e4),
            0x11 => story_lang!(extra, 0xa1b86c, 0x64, name5),
            0 => story_lang!(extra, 0xa1b9b4, 0x64, name5),
            0xe => story_lang!(extra, 0xa1b81c),
            0xc | 0xd => story_lang!(extra, 0xa1b18c, 0x64, name5),
            0x13 => story_lang!(extra, 0xa1b79c, name5),
            0xa | 0x21 | 0x22 => story_lang!(extra, 0xa1b6ec, name5),
            8 => story_lang!(extra, 0xa1b674),
            1 => story_lang!(extra, 0xa1b608, name5),
            2 | 9 => story_lang!(extra, 0xa1b1fc, 0x64, name5),
            0x25 => story_lang!(extra, 0xa1b5c8),
            0x1e | 0x20 => story_lang!(extra, 0xa1b268, 0x64, name5),
            3 => story_lang!(extra, 0xa1b56c, 0x64, name5),
            4 => {
                let (short, g) = self.club_short();
                let tpl = if self.slot14_again != 0 { 0xa1b4c0 } else { 0xa1b414 };
                story_lang!(extra, tpl, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1b2dc, name5),
            6 => story_lang!(extra, 0xa1b378, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The human manager supports the player (reply 1).
    pub(crate) unsafe fn human_supports(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1afa4);
            } else {
                self.human_squad_role();
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1ae44),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1ad78, name5),
            0x13 => story_lang!(extra, 0xa1acdc, name5),
            0xa => story_lang!(extra, 0xa1ac78, name5),
            0xc | 0xd => story_lang!(extra, 0xa1a6ac),
            0xe => story_lang!(extra, 0xa1ac38),
            0x11 => story_lang!(extra, 0xa1abcc, 0x64, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa1ab6c),
            0 => self.human_squad_role(),
            0x21 | 0x22 => story_lang!(extra, 0xa1a714, name5),
            8 => story_lang!(extra, 0xa1ab14),
            1 => story_lang!(extra, 0xa1aabc, name5),
            2 | 9 => story_lang!(extra, 0xa1a77c, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa1aa18, mg, 0x64, self.mask_text.as_ptr(), name5);
            }
            0x1e | 0x20 => story_lang!(extra, 0xa1a7e0),
            3 => story_lang!(extra, 0xa1a9b8, name5),
            4 => {
                let (short, g) = self.club_short();
                story_lang!(extra, 0xa1a920, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1a81c, name5),
            6 => story_lang!(extra, 0xa1a8a0, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The club manager criticises the player (reply 2).
    pub(crate) unsafe fn club_manager_criticises(&mut self) {
        let (extra, name, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1a5ec, manager);
            } else {
                story_lang!(extra, 0xa1a65c, manager);
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1a580, manager),
            0x11 => story_lang!(extra, 0xa1a4e8, 0x64, manager, name5),
            0 | 2 | 9 => story_lang!(extra, 0xa1a65c, manager),
            0xc | 0xd => story_lang!(extra, 0xa19ca0, 0x64, manager, name5),
            8 => story_lang!(extra, 0xa1a440, manager, name),
            0x13 if self.byte_3271 != 0 => story_lang!(extra, 0xa1a3a8, manager5, name5),
            0x13 => story_lang!(extra, 0xa1a310, manager5, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa19d2c, manager5, name5),
            1 => story_lang!(extra, 0xa1a254, g, g, short, manager, name5),
            0x25 => story_lang!(extra, 0xa1a164, manager),
            0x1e | 0x20 => story_lang!(extra, 0xa19e14, manager),
            3 => story_lang!(extra, 0xa1a0dc, manager, name5),
            4 => story_lang!(extra, 0xa1a014, g, g, manager, name5, short),
            6 => story_lang!(extra, 0xa19f64, 0x64, manager, name5),
            5 | 7 => story_lang!(extra, 0xa19e80, g, g, short, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The club manager supports the player (reply 1).
    pub(crate) unsafe fn club_manager_supports(&mut self) {
        let (extra, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa199ec, manager);
            } else {
                self.club_manager_squad_role();
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1975c, manager),
            0x11 => story_lang!(extra, 0xa196d0, 0x64, manager, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa19644, manager),
            0 => self.club_manager_squad_role(),
            0xc | 0xd => story_lang!(extra, 0xa18e44, manager, name5),
            8 => story_lang!(extra, 0xa195e0, manager),
            1 => story_lang!(extra, 0xa19560, manager, name5),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1947c, name5),
            0x13 => story_lang!(extra, 0xa193c8, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa18eec, manager5, name5),
            2 | 9 => story_lang!(extra, 0xa18f70, manager, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa19324, mg, 0x64, manager, self.mask_text.as_ptr());
            }
            0x1e | 0x20 => story_lang!(extra, 0xa18ffc, manager),
            3 => story_lang!(extra, 0xa192a0, manager, name5),
            4 => story_lang!(extra, 0xa191e4, g, g, manager, name5, short),
            5 | 7 => story_lang!(extra, 0xa19060, g, g, short, manager, name5),
            6 => story_lang!(extra, 0xa19128, 0x64, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The links of a person when his record matches the names (`exe::database_staff_matches_names`): one per text
    /// given, type 0x7d7, the person's record.
    pub(crate) unsafe fn link_person(out: usize, index: i32, names: [i32; 3], texts: &[usize]) {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((index as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
            for &t in texts {
                exe::text_add_link(out, t as *const u8, 0x7d7, rec);
            }
        }
    }

    /// The body's text ("%s\n\n%s": the story and the reply) and its links: the persons, the club's nation (name,
    /// short name and the adjectives of forms 0x16..0x1f, to the national team), the club, and after a match the score
    /// (to the fixture, when `Match_Stats::fixture_entry_matches` finds it) and the other club.
    pub(crate) unsafe fn finish(&mut self, out: usize) {
        player_story_piece("links");
        covered!("nma_3 links");
        story_crt!(self.long_extra.as_mut_ptr(), 0x991948, self.text.as_ptr(), self.extra.as_ptr());
        exe::text_set_string(out, self.long_extra.as_ptr());
        if self.second > -1 {
            Self::link_person(out, self.second, self.second_names, &[self.second_name.as_ptr() as usize]);
        }
        if self.player != -1 {
            Self::link_person(out, self.player, self.names, &[self.name.as_ptr() as usize, self.name5.as_ptr() as usize]);
        }
        if self.fourth != -1 {
            Self::link_person(out, self.fourth, self.fourth_names, &[self.fourth_name.as_ptr() as usize]);
        }
        if self.third != -1 {
            Self::link_person(out, self.third, self.third_names, &[self.third_name5.as_ptr() as usize, self.third_name.as_ptr() as usize]);
        }
        if self.club != -1 {
            let team = |st: &PlayerStory| exe::database_get_national_team(st.club_nation());
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + 4) as *const u8, 0x7d5, t);
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + nation::SHORT_NAME) as *const u8, 0x7d5, t);
            for form in 0x16..=0x1f {
                self.club_adjective(form);
                let t = team(self);
                exe::text_add_link(out, self.adjective.as_ptr(), 0x7d5, t);
            }
            let c = self.club_rec();
            exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
            exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);
        }
        if self.with_fixture == 0 {
            return;
        }
        let d = |k: usize| u32::from_le_bytes([self.fixture[k], self.fixture[k + 1], self.fixture[k + 2], self.fixture[k + 3]]);
        let matches: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(PLAYER_STORY_FIXTURE_ENTRY_MATCHES);
        if matches(read_usize(G_MATCH_STATS), d(0x1b), d(0x11), d(0x15), d(0), d(2)) != 0 {
            exe::text_add_link(out, self.score.as_ptr(), 0x7d3, d(0x1b) as usize);
        }
        let c2 = self.other_club();
        exe::text_add_link(out, (c2 + club::NAME) as *const u8, 0x7d5, c2);
    }
}

/// The news slots and the frame of `write_player_praise_news_text`. The slots after 17 are read from a moving index
/// (0x12, or after the match record when slot 17 is set); the names below are those of the use, [inference] where only
/// the use says so.
pub(crate) struct Praise {
    /// the frame's buffers (`praise_frame`), on the caller's stack
    pub(crate) base: usize,
    /// slot 0: the player (staff index); 1-3 his name ids
    pub(crate) player: i32,
    pub(crate) names: [u32; 3],
    /// slot 4: the player's club (club index); `club` is its record (club table + index × `club::SIZE`, unchecked)
    pub(crate) club_index: i32,
    pub(crate) club: usize,
    /// slot 5: the story kind (0x29..0x34)
    pub(crate) kind: i32,
    /// slot 7: a nation linked at the end (index)
    pub(crate) nation_a: i32,
    /// slot 8: the variant (picks the paragraph)
    pub(crate) variant: i32,
    /// slot 9: the manager who replies (staff index); 10-12 his name ids
    pub(crate) manager: i32,
    pub(crate) manager_names: [u32; 3],
    /// slot 13: the reply (1, 2, 3 no reply)
    pub(crate) response: i32,
    /// slot 14: the player's strength flags; slot 15: his position flags
    pub(crate) strengths: u32,
    pub(crate) position: u32,
    /// slot 16: a competition linked at the end (club comp index)
    pub(crate) comp: i32,
    /// slot 17: the story carries a match record (read by `read_match_record_from_news`)
    pub(crate) has_match: u32,
    /// the moving slots: a rival club, four values, the great player and his name ids, two nations, a value, the great
    /// player's job and club, the outlet type and pundit, the player's age [inference] and a flag
    pub(crate) rival_club: i32,
    pub(crate) m1: i32,
    pub(crate) m2: i32,
    pub(crate) m3: i32,
    pub(crate) m4: i32,
    pub(crate) legend: i32,
    pub(crate) legend_names: [u32; 3],
    pub(crate) nation_b: i32,
    pub(crate) nation_c: i32,
    pub(crate) m11: i32,
    pub(crate) legend_job: i32,
    pub(crate) legend_club: i32,
    pub(crate) outlet_type: i32,
    pub(crate) pundit: i32,
    pub(crate) age: i32,
    pub(crate) m17: i32,
    /// the human manager's (screen manager's) club or nation is the player's club's nation: "national" press, else
    /// "{nationality} national" press
    pub(crate) home: bool,
    /// the gender byte of the position text (the dword at frame +0x10 is what the original passes on)
    pub(crate) pos_gender: u32,
    /// the outlet's gender byte, the great player's job gender byte
    pub(crate) outlet_gender: u8,
    pub(crate) job_gender: u8,
    /// the match record read from the news (the original's frame +0x6c)
    pub(crate) record: [u8; 0x20],
    /// the name of the piece that wrote the text (for the comparer)
    pub(crate) piece: &'static str,
}

impl Praise {
    pub(crate) unsafe fn format() -> PraiseFormat {
        core::mem::transmute(FN_LANGLIB_SPRINTF)
    }
    pub(crate) fn at(&self, off: usize) -> usize {
        self.base + off
    }
    /// the club's short-name gender byte (signed) and short name
    pub(crate) unsafe fn cg(&self) -> i32 {
        read_i8(self.club + club::SHORT_GENDER_NAME) as i32
    }
    pub(crate) fn cname(&self) -> usize {
        self.club + club::SHORT_NAME
    }
    /// the club's long-name gender byte and long name
    pub(crate) unsafe fn lg(&self) -> i32 {
        read_i8(self.club + club::GENDER_NAME) as i32
    }
    pub(crate) fn lname(&self) -> usize {
        self.club + club::NAME
    }
    pub(crate) unsafe fn nation(&self) -> usize {
        read_usize(self.club + club::NATION)
    }
    /// the position text's gender byte, signed
    pub(crate) fn pg(&self) -> i32 {
        self.pos_gender as u8 as i8 as i32
    }
    /// `write_player_description(player, position, strengths, POSITION, 0xc8, gender, 1)` (0x5465a0, cdecl; the
    /// original pushes a 7th argument, 1, that the callee does not read): the description over the position text,
    /// returning its gender byte (signed)
    pub(crate) unsafe fn describe(&mut self) -> i32 {
        let f: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(FN_WRITE_PLAYER_DESCRIPTION);
        let out = self.at(praise_frame::POSITION);
        f(self.player, self.position, self.strengths, out, 0xc8, self.pos_gender, 1) as i8 as i32
    }
    /// the description, keeping its gender byte as the position gender (the original stores al at frame +0x10)
    pub(crate) unsafe fn describe_keep(&mut self) {
        let g = self.describe();
        self.pos_gender = (self.pos_gender & 0xffff_ff00) | (g as u8 as u32);
    }
    /// the nationality adjective of the club's nation in `form` into ADJECTIVE (0x78)
    pub(crate) unsafe fn adjective(&mut self, form: u32) {
        let out = self.at(praise_frame::ADJECTIVE);
        exe::nation_get_nationality_adjective(self.nation(), out, 0x78, form);
    }
    /// `database_get_job_name(job, 0, 0, &gender)` of the great player's job (0x53ad00, cdecl)
    pub(crate) unsafe fn legend_job_name(&mut self) -> usize {
        let f: unsafe extern "C" fn(i32, u32, u32, *mut u8) -> usize = core::mem::transmute(FN_DATABASE_GET_JOB_NAME);
        // 0x53ad00 writes the gender only when the game language is 4 (0x53b33f, 0x53b36f); otherwise the original
        // formats with an uninitialised stack byte and the "{}" after the job may stay in the text (DEFEITOS_DO_ORIGINAL)
        if exe::langlib_get_language() != 4 {
            PRAISE_JOB_GENDER_UNSET.store(true, Relaxed);
        }
        f(self.legend_job, 0, 0, &mut self.job_gender)
    }
    pub(crate) fn jg(&self) -> i32 {
        self.job_gender as i8 as i32
    }
    /// the great player's club record and its short-name gender byte
    pub(crate) unsafe fn legend_club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.legend_club as usize).wrapping_mul(club::SIZE))
    }
    /// the great player's nation (staff `NATION` of his record)
    pub(crate) unsafe fn legend_nation(&self) -> usize {
        read_usize(read_usize(G_STAFF_TABLE).wrapping_add((self.legend as usize).wrapping_mul(staff::SIZE)) + staff::NATION)
    }
    /// the first paragraph (or the headline) and the reply
    pub(crate) fn head(&self) -> usize {
        self.at(praise_frame::HEAD)
    }
    pub(crate) fn reply(&self) -> usize {
        self.at(praise_frame::REPLY)
    }
    pub(crate) fn pos(&self) -> usize {
        self.at(praise_frame::POSITION)
    }
    pub(crate) fn full(&self) -> usize {
        self.at(praise_frame::PLAYER_FULL)
    }
    pub(crate) fn short(&self) -> usize {
        self.at(praise_frame::PLAYER_SHORT)
    }
    pub(crate) fn adj(&self) -> usize {
        self.at(praise_frame::ADJECTIVE)
    }
    pub(crate) fn lname_buf(&self) -> usize {
        self.at(praise_frame::LEGEND)
    }
}

/// The text buffer the body is joined in (`g_text_buffer`).
pub(crate) const PRAISE_TEXT_BUFFER: usize = 0xde1f64;

/// The piece of the last text written (for the comparer's message).
pub(crate) static mut PRAISE_LAST_PIECE: &str = "";

/// The match of kind 0x34: the record's club (+0x15), its long-name gender and name, the stage gender (frame +0xe),
/// the score text (frame +0x4c) and the stage label (EXTRA).
pub(crate) struct PraiseMatch {
    pub(crate) rg: i32,
    pub(crate) rname: usize,
    pub(crate) sg: i32,
    pub(crate) score: usize,
    pub(crate) stage: usize,
}

/// The rival club (slot "rival club") of kind 0x34: its short-name gender and name, its nation's gender and name.
pub(crate) struct PraiseRival {
    pub(crate) rgs: i32,
    pub(crate) name: usize,
    pub(crate) rng: i32,
    pub(crate) nation_name: usize,
}

/// The piece of `write_transfer_rumour_news_text` that ran last, for the comparer's message: the low byte is the lead
/// (0 the headline, 1 the lead of kind 0, 2 of kind 1, 3 of kind 36, 4 the others), the next byte the clubs paragraph
/// (the number of clubs, 2 to 5) and the third the agent line (1 the human manager is the agent, 2 the others).
pub(crate) static TRANSFER_RUMOUR_PIECE: AtomicU32 = AtomicU32::new(0);

/// The text buffer the media texts are formatted into before they go into the Text object.
pub(crate) const RUMOUR_TEXT: usize = 0xde1f64;

/// The values the transfer rumour text reads from its news item, by the original's frame: `f` holds the words from
/// +0x00 to +0xb0 of the frame (index = offset / 4), where the club list (+0x28, one club per slot from 0x10) runs over
/// the slots kept after it when it has more than five clubs.
pub(crate) struct Rumour {
    pub(crate) f: [u32; 0x2c],
    pub(crate) club: i32,      // slot 4: the bidding club (-1 none)
    pub(crate) lead: i32,      // slot 5: the kind of lead sentence (0, 1, 36 or other)
    pub(crate) position: u32,  // slot 9: the player's position flags
    pub(crate) flag14: u8,     // slot 14: a flag of the agent line
    pub(crate) count: i32,     // slot 15: the number of clubs in the list (0 when the list reached slot 0x32)
    pub(crate) nation: usize,  // the nation chosen for the links and the nationality adjective (0 none)
    pub(crate) local: bool,    // the human manager is of that nation (or there is no nation)
    pub(crate) desc_gender: i32,
}

impl Rumour {
    pub(crate) fn player(&self) -> i32 { self.f[0x50 / 4] as i32 }      // slot 0: the player's id
    pub(crate) fn player_names(&self) -> (u32, u32, u32) { (self.f[0x4c / 4], self.f[0x44 / 4], self.f[0x40 / 4]) } // slots 1-3
    pub(crate) fn fee(&self) -> u32 { self.f[0x5c / 4] }                // slot 6
    pub(crate) fn agent_kind(&self) -> i32 { self.f[0x64 / 4] as i32 }  // slot 7
    pub(crate) fn qualities(&self) -> u32 { self.f[0x48 / 4] }          // slot 8: the player's strength flags
    pub(crate) fn agent(&self) -> i32 { self.f[0x3c / 4] as i32 }       // slot 10: the agent's id (-1 none)
    pub(crate) fn agent_names(&self) -> (u32, u32, u32) { (self.f[0x60 / 4], self.f[0x58 / 4], self.f[0x54 / 4]) } // slots 11-13
    pub(crate) fn listed(&self, i: usize) -> i32 { self.f[0x28 / 4 + i] as i32 }
}

/// The random draw of slot 1 of the team story by kind (byte table 0x74bb38, kinds 0x3c to 0x63, jumps 0x74bb20): 0
/// draw(3), 1 draw(4), 2 draw(2) + 1 for a club above reputation 5250 or draw(3), 3 draw(3) and 4 draw(2) (a non-zero
/// draw drops the person), 5 the others (the kind becomes 0x63, a draw(2) that drops the person when non-zero, slot 0).
pub(crate) const TEAM_STORY_DRAW_CASE: [u8; 0x28] = [0, 1, 2, 3, 4, 4, 3, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 3, 0, 3, 3, 1, 3, 3, 3, 3, 3];

/// The kinds of team story the manager answers (byte table 0x74bb68, kinds 0x3e to 0x5e; jumps 0x74bb60 and
/// 0x74bba4): 0 for 0x3e, 0x50, 0x51 and 0x5e (no answer), 1 for the others (and every kind outside the table).
pub(crate) const TEAM_STORY_ANSWERED: [u8; 0x21] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

/// The club jobs the team story shows in its person slot (byte table 0x74bb94, jobs 1 to 15): all but 7, 9, 10, 11
/// and 14 (shown as −1).
pub(crate) const TEAM_STORY_JOB_HIDDEN: [u8; 0xf] = [0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0];

/// The comparer's rehearsal of `post_team_media_story`: the copy runs without delivering the news item, without
/// logging the story and without the manager's answer, and keeps what it would do.
pub(crate) static TEAM_STORY_DRY_RUN: AtomicBool = AtomicBool::new(false);

pub(crate) static TEAM_STORY_DELIVERIES: AtomicU8 = AtomicU8::new(0);

pub(crate) static TEAM_STORY_LOGGED: AtomicBool = AtomicBool::new(false);

pub(crate) static TEAM_STORY_LOG_PERSON: AtomicUsize = AtomicUsize::new(0);

pub(crate) static TEAM_STORY_LOG_KIND: AtomicU32 = AtomicU32::new(0);

pub(crate) static TEAM_STORY_LOG_REPLY: AtomicU32 = AtomicU32::new(0);

pub(crate) static mut TEAM_STORY_BUILT: [u8; 0xe0] = [0; 0xe0];

/// Which of the player's complaint tests (0x741f20, the byte table of the jump at 0x741d00) a story kind 0..0x23 uses
/// when the manager answers 2: 0 the general test, 1 the strict test, 2 none.
pub(crate) const PLAYER_STORY_COMPLAINT_CASE: [u8; 0x24] = [
    0, 0, 0, 1, 1, 0, 0, 2, 2, 0, 2, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
];

/// The exe functions the player media story calls that `exe_fns.rs` does not name yet (Rust ports, called through the
/// game's address so their own detours and comparers stay in the path).
pub(crate) const PLAYER_STORY_POSITION_MASK_GET_TEXT: usize = 0x5482f0; // position_mask_get_text, cdecl

pub(crate) const PLAYER_STORY_WRITE_POSITION_NAME: usize = 0x5461b0; // write_position_name, cdecl

pub(crate) const PLAYER_STORY_WRITE_PLAYER_DESCRIPTION: usize = 0x5465a0; // write_player_description, cdecl

pub(crate) const PLAYER_STORY_GET_PUNDIT_NAME: usize = 0x7dad20; // PunditList::get_pundit_name, thiscall, ret 0x10

pub(crate) const PLAYER_STORY_FIXTURE_ENTRY_MATCHES: usize = 0x729500; // Match_Stats::fixture_entry_matches, thiscall, ret 0x14

/// `record_utils` 0x7e4fa0 (cdecl): reads a fixture record (0x1f bytes: date, competition, teams, score, clubs) from the
/// news item's slots, starting at `*counter` and moving it on.
pub(crate) const PLAYER_STORY_READ_FIXTURE: usize = 0x7e4fa0;

pub(crate) type PraiseFormat = unsafe extern "C" fn(usize, usize, ...) -> i32;

/// strcat: appends `src` to the string at `dst`.
pub(crate) unsafe fn story_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}
