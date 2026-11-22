// ---------- batch %%LEVA%%: the scout's working day (player_search.cpp) ----------

/// Owner of the search: the manager's staff index (dword at +0 of PLAYER_SEARCH).
const SCOUT_DAY_OWNER: usize = 0x0;
/// Signed byte at +4: the list's slot, the first field of every scout news item.
const SCOUT_DAY_SLOT: usize = 0x4;
/// Assignment: the club to report on (id, −1 none).
const SCOUT_DAY_TARGET_CLUB: usize = 0x1a1;
/// Reports already made in this assignment (byte, counts the scout's working days).
const SCOUT_DAY_REPORTS_DONE: usize = 0x1b5;
/// The date the competition assignment watches next (TCMDate).
const SCOUT_DAY_COMPETITION_DATE: usize = 0x1be;
/// Capacity of the result array (`player_search_fields::RESULTS`).
const SCOUT_DAY_RESULT_CAPACITY: usize = 0x1f2;
/// Dword the working day clears after every report.
const SCOUT_DAY_REPORT_MARK: usize = 0x2a4;
/// The PLAYER_SEARCH bytes the comparer saves (the routine writes up to +0x2a7).
const SCOUT_DAY_OBJECT_SIZE: usize = 0x2a8;
/// Size of a search result.
const SCOUT_DAY_RESULT_SIZE: usize = 0x2d;

/// How one working day runs: for the game, or as the comparer's dry run (news kept instead of delivered, the knowledge
/// records saved before they change, the reassignment not made).
struct ScoutDayRun {
    dry: bool,
    /// The news items the dry run would deliver: kind (+0..+3) and data (+5..+0xcc).
    news: Vec<[u8; 0xcd]>,
    /// The fog of war records the dry run touched, with their bytes before.
    knowledge: Vec<(usize, [u8; 9])>,
    /// The dry run reached the end of the assignment (start_scout_assignment or 0x7aee80).
    reassigned: bool,
}

impl ScoutDayRun {
    /// Delivers the news item to the manager (`exe::news_deliver_to_manager` on `G_NEWS`, free 0); dry: keeps it.
    unsafe fn deliver(&mut self, item: usize, manager: Staff) {
        if self.dry {
            let mut b = [0u8; 0xcd];
            core::ptr::copy_nonoverlapping(item as *const u8, b.as_mut_ptr(), 0xcd);
            self.news.push(b);
            return;
        }
        exe::news_deliver_to_manager(read_usize(G_NEWS), item, manager, 0);
    }

    /// The manager's fog of war gets to know the player through the scout (`exe::fog_of_war_map_raise_knowledge`); dry:
    /// the 9-byte record is saved first.
    unsafe fn raise_knowledge(&mut self, map: usize, player: Staff, scout: Staff) {
        if self.dry && !player.is_null() && read_i32(player.addr()) < read_i32(G_STAFF_COUNT) {
            let rec = read_usize(map).wrapping_add((read_i32(player.addr()) as usize).wrapping_mul(9));
            let mut b = [0u8; 9];
            core::ptr::copy_nonoverlapping(rec as *const u8, b.as_mut_ptr(), 9);
            self.knowledge.push((rec, b));
        }
        exe::fog_of_war_map_raise_knowledge(map, player, scout);
    }
}

/// The player's club by his contract (`exe::contract_manager_get_contract` on `G_CONTRACT_MANAGER`, the answer kept in
/// `G_CONTRACT_LOOKUP_RESULT`): the club of the contract's `contract::CLUB` id, or null.
unsafe fn scout_day_contract_club(player: Staff) -> Club {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 { club_at(read_i32(c + contract::CLUB) as isize) } else { Club(0) }
}

/// Removes result `r` by copying the last one over it (the two dates by `exe::date_operator_3d`); one result fewer.
unsafe fn scout_day_remove_result(this: usize, r: usize) {
    let n = read_i32(this + player_search_fields::RESULT_COUNT);
    let last = read_usize(this + player_search_fields::RESULTS).wrapping_add(((n - 1) as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    for off in [0usize, 4, 8, 0xc] {
        write_u32(r + off, read_u32(last + off));
    }
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(r + 0x10, tmp.as_mut_ptr() as usize, last + 0x10);
    exe::date_operator_3d(r + search_result::LAST_UPDATE_DATE, tmp.as_mut_ptr() as usize, last + search_result::LAST_UPDATE_DATE);
    write_u8(r + search_result::FLAGS, read_u8(last + search_result::FLAGS));
    write_u32(r + 0x21, read_u32(last + 0x21));
    write_u32(r + 0x25, read_u32(last + 0x25));
    for off in 0x29..0x2d {
        write_u8(r + off, read_u8(last + off));
    }
    write_i32(this + player_search_fields::RESULT_COUNT, n - 1);
}

/// Whether result `r` stays in the scout's list: the scout has a club and the player a player sheet; with the
/// opposition flag, always; otherwise the player's nation (that of his club, or his own) has to be the assigned nation
/// and region; for the base region only players under 20 without a club, from the region of the scout club's nation
/// or worth more than 40000 (+0x25); from the scout club's nation, only those below 4000 of reputation for the scout
/// (`exe::database_get_staff_reputation_for_viewer`); never a player of the scout's own club (`scout_day_contract_club`)
/// nor one about to retire (`exe::player_regen_get_retirement_status`).
unsafe fn scout_day_keeps(this: usize, r: usize, player: Staff, scout: Staff) -> bool {
    let mut nation = Nation(0);
    let club = player.club_job();
    if !club.is_null() {
        nation = club.nation();
    }
    if nation.is_null() {
        nation = player.nation();
    }
    let scout_club = scout.club_job();
    if scout_club.is_null() || player.player().is_null() {
        return false;
    }
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) != 0 {
        return true;
    }
    let assigned = read_i32(this + player_search_fields::ASSIGNED_NATION);
    if assigned > -1 && !nation.is_null() && read_i32(nation.addr()) != assigned {
        return false;
    }
    let region = read_i8(this + player_search_fields::ASSIGNED_REGION);
    if region > -1 && !nation.is_null() && nation.actual_region() as i8 != region {
        return false;
    }
    if read_u32(this + player_search_fields::ASSIGNED_BASE) != 0 {
        if player.age() as i8 >= 20 {
            return false;
        }
        let same_region = !nation.is_null() && nation.actual_region() == scout_club.nation().actual_region();
        if !same_region && read_i32(r + 0x25) <= 40000 {
            return false;
        }
        if !player.club_job().is_null() {
            return false;
        }
    }
    if nation.addr() == scout.club_job().nation().addr() && exe::database_get_staff_reputation_for_viewer(player, scout, 0) >= 4000 {
        return false;
    }
    let club = scout_day_contract_club(player);
    if club.addr() == scout.club_job().addr() {
        return false;
    }
    exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), player) == 0
}

/// One player reported (results `r` with bit 0 of `search_result::FLAGS`): the result is dated today and loses the bit;
/// one fewer to report, one more reported. The work permit for the scout's club (`exe::transfer_manager_player_needs_work_permit`
/// and, when needed, `exe::transfer_manager_is_work_permit_granted` in the club nation). Without the opposition flag,
/// a player graded 4 or more (now or potential, +0x29/+0x2a) gets the report news 0x138b in `item`: slot, the scout,
/// age, positions, the player, his club id (−1 none), the two grades, the assignment (nation, region, competition,
/// base), three zeros, the work permit (1 and the answer when needed; otherwise 2 for a player of membership group 2
/// by his second nation when the scout club's region is 14, else 0, and 0), the two words of 0x7bd9d0, 0x420e9 and the
/// player's strength flags (`exe::staff_get_player_strength_flags`); delivered to the manager. `SCOUT_DAY_REPORT_MARK`
/// is cleared.
#[allow(clippy::too_many_arguments)]
unsafe fn scout_day_report(this: usize, run: &mut ScoutDayRun, r: usize, player: Staff, club: Club, scout: Staff, manager: Staff, left: &mut i32, reported: &mut i32) {
    exe::date_operator_3d(r + search_result::LAST_UPDATE_DATE, [0u32; 2].as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(r + search_result::FLAGS, read_u8(r + search_result::FLAGS) & 0xfe);
    *left -= 1; // gra_26 one fewer to report
    *reported += 1;
    let scout_club = scout.club_job();
    let needed = exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), player, scout_club, 1);
    let granted = if needed == 0 { 1 } else { (exe::transfer_manager_is_work_permit_granted(read_usize(G_TRANSFER_MANAGER), player, scout_club.nation(), 0, 0, 1) != 0) as u32 };
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 {
        let (now, potential) = (read_i8(r + 0x29), read_i8(r + 0x2a));
        if (granted == 0 && now >= 5) || potential >= 5 || now >= 4 || potential >= 4 {
            covered!("gra_26 player report");
            let mut item = [0u8; 0x100];
            let it = item.as_mut_ptr() as usize;
            exe::new_newsitem(it, 0x138b, 0);
            let mut n: u8 = 0;
            let age = player.age() as i8;
            let positions = exe::formation_get_position_mask(player.player()) as i16;
            let mut name = [0u8; 0x20];
            exe::fill_staff_name_ref(player, name.as_mut_ptr() as usize);
            let club_id = if club.is_null() { -1 } else { read_i32(club.addr()) };
            let slot = |n: &mut u8, v: u32| {
                exe::newsitem_set_slot(it, *n as i8, v);
                *n = n.wrapping_add(1);
            };
            slot(&mut n, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
            exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
            slot(&mut n, age as i32 as u32);
            slot(&mut n, positions as i32 as u32);
            exe::write_staff_name_ref_to_news(name.as_ptr() as usize, it, &mut n);
            slot(&mut n, club_id as u32);
            slot(&mut n, now as i32 as u32);
            slot(&mut n, potential as i32 as u32);
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_NATION));
            slot(&mut n, read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32);
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_COMP));
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_BASE));
            slot(&mut n, 0);
            slot(&mut n, 0);
            slot(&mut n, 0);
            if needed != 0 {
                slot(&mut n, 1);
                slot(&mut n, granted);
            } else {
                let second = player.second_nation();
                let group2 = player.nation().group_membership() != 2 && !second.is_null() && second.group_membership() == 2 && scout_club.nation().actual_region() == 0xe;
                slot(&mut n, if group2 { 2 } else { 0 });
                slot(&mut n, 0);
            }
            let mut words = [0i16; 2];
            let words_of: unsafe extern "C" fn(usize, usize, *mut i16, *mut i16) = core::mem::transmute(0x7bd9d0usize);
            words_of(player.addr(), scout.addr(), words.as_mut_ptr().add(1), words.as_mut_ptr());
            slot(&mut n, words[1] as i32 as u32);
            slot(&mut n, words[0] as i32 as u32);
            slot(&mut n, 0x420e9);
            // the index byte is taken before the strength flags are asked (the original bumps it first; nothing reads it after)
            exe::newsitem_set_slot(it, n as i8, exe::staff_get_player_strength_flags(player, club));
            run.deliver(it, manager);
        }
    }
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
}

/// Whether result `r` goes into the final report's best players: a player with a player sheet graded above 3 now (or 3
/// with a reputation above 0x1a5e) who needs no work permit for `club`.
unsafe fn scout_day_shortlisted(r: usize, club: usize) -> bool {
    let p = staff_at(read_i32(r + 0xc) as isize);
    if p.is_null() || p.player().is_null() {
        return false;
    }
    let grade = read_i8(r + 0x29);
    let good = (read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) > 0x1a5e && grade == 3) || grade > 3;
    good && exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club, 0) == 0
}

/// The 0x1a-byte flags of a shortlisted player (attributes 20 marks positions, the extremes of the attributes, the
/// goals per game in the attack, the form against the average rating, youth, age, a new arrival and injury).
unsafe fn scout_day_player_flags(p: Staff, pd: usize) -> u32 {
    let b = |o: usize| read_i8(pd + o);
    let mut f = 0u32;
    if b(player_sheet::DEFENDER) == 20 {
        f = 0x200;
    }
    if b(player_sheet::ATTACKER) == 20 {
        f |= if b(player_sheet::CENTRAL) == 20 {
            0x100
        } else if b(player_sheet::ATTACKING_MIDFIELDER) == 20 || b(player_sheet::MIDFIELDER) == 20 {
            0x1000
        } else if b(player_sheet::DEFENDER) == 20 || b(player_sheet::DEFENSIVE_MIDFIELDER) == 20 {
            0x2000
        } else {
            0x100
        };
    }
    if b(player_sheet::ATTACKING_MIDFIELDER) == 20 || b(player_sheet::DEFENSIVE_MIDFIELDER) == 20 || b(player_sheet::MIDFIELDER) == 20 {
        f |= if b(player_sheet::DEFENDER) == 20 { 0x2000 } else { 0x400 };
        if b(player_sheet::LEFT_SIDE) == 20 || b(player_sheet::RIGHT_SIDE) == 20 {
            f |= 0x1000;
        }
    }
    if b(player_sheet::GOALKEEPER) == 20 {
        f |= 0x800;
    }
    let jumping = b(player_sheet::JUMPING);
    if jumping > 18 && b(player_sheet::HEADING) > 14 && b(player_sheet::STRENGTH) > 15 {
        f |= 0x20;
    } else if jumping < 6 {
        f |= 0x10;
    }
    if b(player_sheet::WORK_RATE) > 14 && b(player_sheet::AGGRESSION) > 10 && b(player_sheet::TACKLING) > 14 {
        f |= 2;
    }
    let pace = b(player_sheet::PLAYER_PACE);
    if pace > 18 {
        f |= 4;
    } else if pace < 6 {
        f |= 8;
    }
    let consistency = b(player_sheet::CONSISTENCY);
    if consistency > 15 {
        f |= 0x80000;
    } else if consistency < 5 {
        f |= 0x400000;
    }
    if b(player_sheet::LEADERSHIP) > 14 {
        f |= 0x80;
    }
    let stats = read_usize(G_PLAYER_STATS);
    let games = exe::playerstats_get_stat(stats, pd, 6, 1, 6.5);
    if games > read_f32(rdata::F32_5) as f64 {
        let games = if games > 1.0 { games } else { 1.0 };
        let goals = exe::playerstats_get_stat(stats, pd, 6, 2, 6.5);
        let ratio = goals / games;
        if b(player_sheet::ATTACKER) > 15 {
            if ratio < read_f64(rdata::F64_0_25) {
                f |= 0x200000;
            } else if ratio > read_f64(rdata::F64_0_65) {
                f |= 0x100000;
            }
        }
        exe::playerstats_get_stat(stats, pd, 6, 0x11, 6.5);
        let count = read_f64(stats + player_stats_fields::RATED_COUNT);
        let average = if count == 0.0 || count.is_nan() { read_f64(rdata::F64_6_8) } else { read_f64(stats + player_stats_fields::RATING_SUM) / count } as f32;
        let form = exe::playerstats_get_recent_form(stats, pd) as i32 as f64 * read_f64(rdata::F64_0_2);
        if form > average as f64 + read_f64(rdata::F64_0_25) {
            f |= 0x800000;
        } else if form < average as f64 - read_f64(rdata::F64_0_35) {
            f |= 0x1000000;
        }
    }
    if b(player_sheet::VISION) > 14 {
        f |= 1;
    }
    if b(player_sheet::STRENGTH) > 14 {
        f |= 0x4000;
    }
    if (p.age() as i8) < 22 && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x1676 {
        f |= 0x8000;
    }
    if p.age() as i8 > 34 {
        f |= 0x10000;
    }
    if exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) < 0x41 {
        f |= 0x20000;
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
        f |= 0x40000;
    }
    f
}

/// The final report's best players of the target club's search: how many there are (only those
/// `scout_day_shortlisted` for `club`), the first two after the sort (qsort with 0x7be640) as seven slots each (+0x21 of
/// the result, 0 when the player's contract happiness is below 25; the player; the first three dwords of the result; the
/// flags of `scout_day_player_flags`; his current ability). Out of memory: the error box (line 0xd72) and the day ends
/// with 0 (`None`).
unsafe fn scout_day_best_players(this: usize, item: usize, n: &mut u8, club: usize) -> Option<()> {
    let count = || read_i32(this + player_search_fields::RESULT_COUNT);
    let result = |j: i32| read_usize(this + player_search_fields::RESULTS).wrapping_add((j as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    let mut found = 0i32;
    let mut j = 0;
    while j < count() {
        if scout_day_shortlisted(result(j), club) {
            found += 1;
        }
        j += 1;
    }
    let mut shown = found;
    if found != 0 {
        let buf = exe::crt_malloc((found as u32).wrapping_mul(0x1a));
        if buf == 0 {
            show_error_at(0xa701b0, 0xd72);
            return None;
        }
        let mut filled = 0i32;
        let mut j = 0;
        while j < count() && filled < found {
            let r = result(j);
            if scout_day_shortlisted(r, club) {
                let p = staff_at(read_i32(r + 0xc) as isize);
                let e = buf + filled as usize * 0x1a;
                write_u32(e, read_u32(r + 0x21));
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
                if c != 0 && read_i8(c + contract::HAPPINESS) < 0x19 {
                    write_u32(e, 0);
                }
                write_u32(e + 4, read_u32(r + 0xc));
                write_u32(e + 8, read_u32(r));
                write_u32(e + 0xc, read_u32(r + 4));
                write_u32(e + 0x10, read_u32(r + 8));
                write_u16(e + 0x18, read_u16(p.player().addr() + player_sheet::CURRENT_ABILITY));
                write_u32(e + 0x14, 0);
                let pd = p.player().addr();
                if pd != 0 {
                    covered!("gra_26 best players of the club");
                    write_u32(e + 0x14, scout_day_player_flags(p, pd));
                }
                filled += 1;
            }
            j += 1;
        }
        if filled != found {
            show_error_at(0xa701b0, 0xe36);
        }
        exe::crt_qsort(buf, found as u32, 0x1a, 0x7be640);
        if shown > 2 {
            shown = 2;
        }
        exe::newsitem_set_slot(item, *n as i8, shown as u32);
        *n = n.wrapping_add(1);
        let mut k = 0;
        while k < shown && k < 2 {
            let e = buf + k as usize * 0x1a;
            for off in [0usize, 4, 8, 0xc, 0x10, 0x14] {
                exe::newsitem_set_slot(item, *n as i8, read_u32(e + off));
                *n = n.wrapping_add(1);
            }
            exe::newsitem_set_slot(item, *n as i8, read_i16(e + 0x18) as i32 as u32);
            *n = n.wrapping_add(1);
            k += 1;
        }
        exe::crt_free(buf);
    } else {
        exe::newsitem_set_slot(item, *n as i8, 0);
        *n = n.wrapping_add(1);
    }
    Some(())
}

/// The last day of the assignment: the summary news 0x138a (slot, the scout, players reported, 1, the assignment's six
/// fields and the target club); with a target club its best players (`scout_day_best_players`) and, when the manager
/// has a club, fifteen answers about the target club (eleven Database.cpp questions 0x549100..0x5493c0, the manager's
/// club id, the target's reputation, 0x549a30 between the two, byte +4 of its club state row); otherwise fourteen zeros.
/// Delivered when there are results (and, with the opposition flag, a target club); otherwise the scout starts the same
/// assignment again (`exe::player_search_start_scout_assignment`); without the opposition flag the assignment then
/// ends (0x7aee80). `None`: out of memory, the day ends with 0.
unsafe fn scout_day_final_report(this: usize, run: &mut ScoutDayRun, manager: Staff, own: Club, reported: i32, permit_club: usize) -> Option<()> {
    covered!("gra_26 final report");
    let mut item = [0u8; 0x100];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0x138a, 0);
    exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
    let mut n: u8 = 1;
    exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
    let opponent = read_u32(this + player_search_fields::ASSIGNED_OPPONENT);
    for v in [
        reported as u32,
        1,
        read_u32(this + player_search_fields::ASSIGNED_NATION),
        read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
        read_u32(this + player_search_fields::ASSIGNED_COMP),
        read_u32(this + player_search_fields::ASSIGNED_BASE),
        opponent,
        read_u32(this + SCOUT_DAY_TARGET_CLUB),
    ] {
        exe::newsitem_set_slot(it, n as i8, v);
        n = n.wrapping_add(1);
    }
    let mut target = Club(0);
    let target_id = read_i32(this + SCOUT_DAY_TARGET_CLUB);
    if target_id != -1 {
        target = club_at(target_id as isize);
        // the club the work permit is asked for is whatever the original left in [esp+0x30]: the club of the last
        // player reported today, or (with nobody reported) the count of flagged results, 0 then
        scout_day_best_players(this, it, &mut n, permit_club)?;
    }
    if target_id != -1 && !own.is_null() {
        covered!("gra_26 target club answers");
        let q = |a: usize| -> u32 {
            let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(a);
            f(target.addr()) as i8 as i32 as u32
        };
        let mut answers = [0u32; 15];
        for (k, a) in [0x549100usize, 0x5492c0, 0x549200, 0x549470, 0x5495b0, 0x5494d0, 0x549690, 0x549770, 0x549940, 0x549850, 0x5493c0].iter().enumerate() {
            answers[k] = q(*a);
        }
        answers[11] = read_u32(own.addr());
        answers[12] = target.reputation() as i32 as u32;
        let between: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x549a30usize);
        answers[13] = between(target.addr(), own.addr()) as i8 as i32 as u32;
        answers[14] = read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(target.addr()) as usize).wrapping_mul(6)) + 4) as i32 as u32;
        for v in answers {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
    } else {
        for _ in 0..14 {
            exe::newsitem_set_slot(it, n as i8, 0);
            n = n.wrapping_add(1);
        }
    }
    if (opponent == 0 || !target.is_null()) && read_i32(this + player_search_fields::RESULT_COUNT) != 0 {
        run.deliver(it, manager);
    } else if run.dry {
        run.reassigned = true;
    } else {
        covered!("gra_26 assignment started again");
        exe::player_search_start_scout_assignment(
            this,
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_u8(this + player_search_fields::ASSIGNED_REGION) as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
            opponent,
        );
    }
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 {
        if run.dry {
            run.reassigned = true;
        } else {
            let end_assignment: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7aee80usize);
            end_assignment(this);
        }
    }
    Some(())
}

/// The scout's working day for a human manager's search list (list type 4).
///
/// `PLAYER_SEARCH::run_scout_day()`, called by the scouts' day (`scout_manager.rs`) for each of the human managers'
/// lists. 0 when the list is not of type 4, without a scout (`player_search_fields::SCOUT_ID`) whose name still matches
/// (`exe::database_staff_matches_names`) or without any assignment (nation, region, competition, base, opposition). 1
/// without doing anything until the day of `player_search_fields::SCOUTING_END_DATE`. On that day: the first day
/// (`SCOUT_DAY_REPORTS_DONE` 0) is the news 0x1389 (slot, the scout and the four assignment fields). Then: the days of
/// the assignment are 1 (competition or opposition) or 8; the results are tidied (0x7b1520); each result shows the
/// player to the manager's fog of war (`G_ATTRIBUTE_MASKING_ENABLED`) and stays only when `scout_day_keeps` (otherwise
/// the last one takes its place); the kept ones with bit 0 of `search_result::FLAGS` are the flagged; sorted (qsort with
/// 0x7bce50), or the array freed when empty. How many to report today: before the last day (50 + 75..124)% × results /
/// (days × 100), on the last day all the flagged; at least 1 and at most the flagged. Each: a random flagged result
/// (`exe::utils_random` of the count until one with the bit), `scout_day_report`, and every other flagged result of the
/// same club right after it (even past the number). On the last day the summary (`scout_day_final_report`); halfway
/// (days / 2) the news 0x138a with the count, 0 and the six assignment fields. In the end the next working day: two
/// days after `SCOUT_DAY_COMPETITION_DATE` for a competition, otherwise in 2..8 days (`exe::utils_random`(7) + 2); one
/// more day done; 1.
/// Original: `PLAYER_SEARCH::run_scout_day` (0x7b3e20, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn run_scout_day(this: usize) -> u32 {
    let mut run = ScoutDayRun { dry: false, news: Vec::new(), knowledge: Vec::new(), reassigned: false };
    scout_day(this, &mut run)
}

/// The working day itself (`run_scout_day`), for the game or as the comparer's dry run.
unsafe fn scout_day(this: usize, run: &mut ScoutDayRun) -> u32 {
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 {
        return 0;
    }
    let scout_id = read_i32(this + player_search_fields::SCOUT_ID);
    if scout_id <= -1 {
        return 0;
    }
    let scout = staff_at(scout_id as isize);
    if exe::database_staff_matches_names(scout, read_u32(this + 0x174), read_u32(this + 0x178), read_u32(this + 0x17c), this + 0x184) == 0 || scout.is_null() {
        return 0;
    }
    if read_i32(this + player_search_fields::ASSIGNED_NATION) == -1
        && read_u8(this + player_search_fields::ASSIGNED_REGION) == 0xff
        && read_i32(this + player_search_fields::ASSIGNED_COMP) == -1
        && read_u32(this + player_search_fields::ASSIGNED_BASE) == 0
        && read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0
    {
        return 0;
    }
    let manager = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    let own = manager.club_job();
    if read_u16(this + player_search_fields::SCOUTING_END_YEAR) != read_u16(G_CURRENT_DATE_YEAR) || read_u16(this + player_search_fields::SCOUTING_END_DATE) != read_u16(G_CURRENT_DATE) {
        return 1;
    }
    let done = read_i8(this + SCOUT_DAY_REPORTS_DONE);
    if done == 0 {
        covered!("gra_26 first day");
        let mut item = [0u8; 0x100];
        let it = item.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0x1389, 0);
        exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
        let mut n: u8 = 1;
        exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
        for v in [
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
        ] {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
        run.deliver(it, manager);
        return scout_day_next_day(this);
    }
    let days: i32 = if read_i32(this + player_search_fields::ASSIGNED_COMP) > -1 || read_u32(this + player_search_fields::ASSIGNED_OPPONENT) != 0 { 1 } else { 8 };
    let mut flagged = 0i32;
    // the original's [esp+0x30]: the count of flagged results, then the club of each reported player's run; the final
    // report asks the work permit for it
    let mut permit_club = 0usize;
    let tidy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x7b1520usize);
    tidy(this, 0);
    let count = || read_i32(this + player_search_fields::RESULT_COUNT);
    let result = |j: i32| read_usize(this + player_search_fields::RESULTS).wrapping_add((j as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    if count() != 0 {
        let mut i = 0;
        while i < count() {
            let r = result(i);
            let player = staff_at(read_i32(r + 0xc) as isize);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), manager);
                if map != 0 {
                    run.raise_knowledge(map, player, scout);
                }
            }
            if scout_day_keeps(this, r, player, scout) {
                if read_u8(r + search_result::FLAGS) & 1 != 0 {
                    flagged += 1;
                    permit_club = permit_club.wrapping_add(1);
                }
            } else {
                covered!("gra_26 result dropped");
                scout_day_remove_result(this, r);
                i -= 1;
            }
            i += 1;
        }
        if count() != 0 {
            exe::crt_qsort(read_usize(this + player_search_fields::RESULTS), count() as u32, SCOUT_DAY_RESULT_SIZE as u32, 0x7bce50);
        } else {
            exe::crt_free(read_usize(this + player_search_fields::RESULTS));
            write_usize(this + player_search_fields::RESULTS, 0);
            write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
        }
    }
    let wanted = if (done as i32) < days {
        (exe::utils_random(50) + 0x4b).wrapping_mul(count()) / (days * 100) // gra_26 how many today
    } else {
        flagged
    };
    let mut left = if wanted.max(1) >= flagged {
        flagged
    } else if wanted <= 1 {
        1
    } else {
        wanted
    };
    let mut reported = count() - flagged;
    while left > 0 {
        let (r, player, club) = loop {
            let r = result(exe::utils_random(count()));
            let player = staff_at(read_i32(r + 0xc) as isize);
            let club = scout_day_contract_club(player);
            if read_u8(r + search_result::FLAGS) & 1 != 0 {
                break (r, player, club);
            }
        };
        scout_day_report(this, run, r, player, club, scout, manager, &mut left, &mut reported);
        let same = scout_day_contract_club(staff_at(read_i32(r + 0xc) as isize));
        permit_club = same.addr();
        if same.is_null() {
            continue;
        }
        let mut j = 0;
        while j < count() {
            let rj = result(j);
            if scout_day_contract_club(staff_at(read_i32(rj + 0xc) as isize)).addr() == same.addr() {
                let pj = staff_at(read_i32(rj + 0xc) as isize);
                let cj = scout_day_contract_club(pj);
                if read_u8(rj + search_result::FLAGS) & 1 != 0 {
                    covered!("gra_26 same club reported");
                    scout_day_report(this, run, rj, pj, cj, scout, manager, &mut left, &mut reported);
                }
            }
            j += 1;
        }
    }
    if done as i32 == days {
        if scout_day_final_report(this, run, manager, own, reported, permit_club).is_none() {
            return 0;
        }
    } else if done as i32 == days / 2 {
        covered!("gra_26 halfway news");
        let mut item = [0u8; 0x100];
        let it = item.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0x138a, 0);
        exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
        let mut n: u8 = 1;
        exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
        for v in [
            reported as u32,
            0,
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
            read_u32(this + player_search_fields::ASSIGNED_OPPONENT),
            read_u32(this + SCOUT_DAY_TARGET_CLUB),
        ] {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
        run.deliver(it, manager);
    }
    scout_day_next_day(this)
}

/// The next working day: two days after `SCOUT_DAY_COMPETITION_DATE` for a competition assignment, otherwise
/// `exe::utils_random`(7) + 2 days from today (`exe::date_add_days`, copied into
/// `player_search_fields::SCOUTING_END_DATE`); one more day done (`SCOUT_DAY_REPORTS_DONE`). Returns 1.
unsafe fn scout_day_next_day(this: usize) -> u32 {
    let mut out = [0u32; 2];
    let mut tmp = [0u32; 2];
    let d = if read_i32(this + player_search_fields::ASSIGNED_COMP) > -1 {
        exe::date_add_days(this + SCOUT_DAY_COMPETITION_DATE, out.as_mut_ptr() as usize, 2)
    } else {
        let n = exe::utils_random(7) + 2; // gra_26 the next working day
        exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, n as u32)
    };
    exe::date_operator_3d(this + player_search_fields::SCOUTING_END_DATE, tmp.as_mut_ptr() as usize, d);
    write_u8(this + SCOUT_DAY_REPORTS_DONE, read_u8(this + SCOUT_DAY_REPORTS_DONE).wrapping_add(1));
    1
}

/// Comparer of `run_scout_day` (comparison mode: see the header of motor.rs), as a dry run (model
/// `compare_post_recovery_news`).
///
/// The copy runs first from the state of the moment as a dry run: on a copy of the result array (allocated by the game's
/// malloc, which the routine may free), keeping the news items instead of delivering them, saving each fog of war
/// record before it changes and without restarting or ending the assignment. Then the PLAYER_SEARCH object, the
/// records, the random generator and `G_CONTRACT_LOOKUP_RESULT` go back and the original runs. Compared: the return;
/// each news item the original delivered into the manager's inbox ring (kind and data) against the dry run's, in order;
/// the fog of war records; and, unless the assignment was restarted or ended, the object (all but the array pointer),
/// the result array, the generator and the contract lookup.
pub(crate) unsafe extern "thiscall" fn compare_run_scout_day(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let mut regions: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    regions.push((this, SCOUT_DAY_OBJECT_SIZE));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(1) as usize * SCOUT_DAY_RESULT_SIZE;
        let copy = exe::crt_malloc(n as u32);
        core::ptr::copy_nonoverlapping(results as *const u8, copy as *mut u8, n);
        write_usize(this + player_search_fields::RESULTS, copy);
    }
    let mut run = ScoutDayRun { dry: true, news: Vec::new(), knowledge: Vec::new(), reassigned: false };
    let rd = crate::rng::pause_trace(|| crate::rng::without_counting(|| scout_day(this, &mut run)));
    let dry_state = crate::comparison::MemorySnapshot::read(&regions);
    let dry_results: Vec<u8> = {
        let a = read_usize(this + player_search_fields::RESULTS);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let v = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        if a != 0 {
            exe::crt_free(a);
        }
        v
    };
    let dry_knowledge: Vec<[u8; 9]> = run.knowledge.iter().map(|(a, _)| {
        let mut b = [0u8; 9];
        core::ptr::copy_nonoverlapping(*a as *const u8, b.as_mut_ptr(), 9);
        b
    }).collect();
    for (a, b) in run.knowledge.iter().rev() {
        core::ptr::copy_nonoverlapping(b.as_ptr(), *a as *mut u8, 9);
    }
    before.write_back();
    let news = read_usize(G_NEWS);
    let manager = read_i32(this + SCOUT_DAY_OWNER);
    let idx = manager.wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    let inbox = if idx >= 0 && news != 0 { read_usize(news + news_fields::HUMAN_INBOXES) + idx as usize * 12 } else { 0 };
    let next_before = if inbox != 0 && read_i32(inbox) != -1 { Some(read_i32(inbox + 4)) } else { None };
    let r = o(this);
    let mut wrong: Vec<String> = Vec::new();
    if r != rd {
        wrong.push(format!("returned {} x {}", r, rd));
    }
    if !run.news.is_empty() {
        if inbox == 0 || read_i32(inbox) == -1 {
            wrong.push(format!("{} news items expected, no inbox", run.news.len()));
        } else {
            let start = read_i32(inbox);
            let mut slot = next_before.unwrap_or(start);
            for (k, expected) in run.news.iter().enumerate() {
                let item = read_usize(news).wrapping_add((slot as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
                let got = core::slice::from_raw_parts(item as *const u8, 0xcd);
                if got[0..4] != expected[0..4] || got[5..] != expected[5..] {
                    let field = (5..0xcd).find(|&j| got[j] != expected[j]).map(|j| (j - 5) / 4);
                    wrong.push(format!("news {} kind {:#x} x {:#x}, first different field {:?}", k, read_u32(item), u32::from_le_bytes([expected[0], expected[1], expected[2], expected[3]]), field));
                }
                slot += 1;
                if slot == start + 0x64 {
                    slot = start;
                }
            }
        }
    }
    for (k, (a, _)) in run.knowledge.iter().enumerate() {
        if core::slice::from_raw_parts(*a as *const u8, 9) != dry_knowledge[k] {
            wrong.push(format!("fog of war record {:#x}", a));
        }
    }
    if !run.reassigned {
        let after = crate::comparison::MemorySnapshot::read(&regions);
        let pointer = this + player_search_fields::RESULTS;
        for ((a, o), (_, d)) in after.0.iter().zip(dry_state.0.iter()) {
            let differs = (0..o.len()).any(|j| o[j] != d[j] && !(*a == this && (pointer - this..pointer - this + 4).contains(&j)));
            if differs {
                wrong.push(format!("region {:#x}", a));
            }
        }
        let a = read_usize(pointer);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let got: Vec<u8> = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        if got != dry_results {
            wrong.push("the result array".into());
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("run_scout_day(search {:#x}, day {}) {:?}", this, read_i8(this + SCOUT_DAY_REPORTS_DONE), wrong));
    r
}
