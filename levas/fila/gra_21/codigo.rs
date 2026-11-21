// ---------- batch %%LEVA%%: the AI pre-season tournaments and tours (friendly.cpp) ----------

/// Adds candidate `item`'s club id (the first dword of the candidate, which `exe::llist_add_tail` copies: push 4; push
/// item; ecx = the list -> thiscall) to the opponents list (`preseason_candidate::OPPONENTS`) of candidate `cand`,
/// making the list first when it has none (`exe::crt_operator_new`(0xe), cdecl, and `exe::new_llist`, thiscall; out of
/// memory the list stays null and the add crashes, as in the original).
unsafe fn add_preseason_opponent(cand: usize, item: usize) {
    if read_usize(cand + preseason_candidate::OPPONENTS) == 0 {
        let m = exe::crt_operator_new(0xe);
        write_usize(cand + preseason_candidate::OPPONENTS, if m != 0 { exe::new_llist(m) } else { 0 });
    }
    exe::llist_add_tail(read_usize(cand + preseason_candidate::OPPONENTS), item, 4);
}

/// The first of the `count` candidate pointers `sorted` (sorted by nation and band) whose candidate has the nation's id
/// (`preseason_candidate::NATION_ID`), or 0: bsearch (`exe::crt_bsearch`: 5 pushes; add esp, 0x14 -> cdecl, through
/// `compare_friendly_candidate_ptrs_by_nation` 0x5c5290) with a 0x16-byte key that has 0xff at +5 and the nation id at
/// +6 (the rest zero), then back while the previous pointer is of the same nation.
unsafe fn first_candidate_of_nation(sorted: usize, count: u32, nation: Nation) -> usize {
    let mut key = [0u8; 0x16];
    key[5] = 0xff;
    key[6..10].copy_from_slice(&read_u32(nation.addr()).to_le_bytes());
    let pkey = key.as_ptr() as usize;
    let mut q = exe::crt_bsearch(&pkey as *const usize as usize, sorted, count, 4, 0x5c5290);
    if q == 0 {
        return 0;
    }
    while q > sorted {
        if read_u32(read_usize(q - 4) + preseason_candidate::NATION_ID) != read_u32(nation.addr()) {
            break;
        }
        q -= 4;
    }
    q
}

/// The pre-season planning gate shared by both halves: the league entry `ent` of `G_LEAGUE_TABLE` (0x48 bytes) has a
/// nation with `.league_selected()`, its season (`key_nation_record::START_DATE`) has not started (year unsigned, day
/// signed) and its date `key_nation_record::END_DATE` minus 0x34 weeks (`exe::date_sub_weeks`: lea ecx, [entry + 0x39];
/// 2 pushes -> thiscall) has already arrived.
unsafe fn preseason_league_open(ent: usize) -> bool {
    let nation = Nation(read_usize(ent));
    if nation.is_null() || nation.league_selected() == 0 || is_before_today(ent + key_nation_record::START_DATE) {
        return false;
    }
    let mut d = [0u8; 8];
    !is_after_today(exe::date_sub_weeks(ent + key_nation_record::END_DATE, &mut d, 0x34))
}

/// Rule: in each pre-season (from 2002 on) every league nation may get one AI tournament: the AI club that draws the
/// highest number from its reputation band hosts three other clubs, in a cup or a mini league; then the AI clubs of each
/// league may go on a tour abroad against 2 to 4 clubs of another nation.
///
/// `this.plan_preseason_tournaments_and_tours(entries, sorted, count)`, called by the AI pre-season plan
/// (`plan_ai_preseason_friendlies`) with the `count` 0x16-byte candidates `entries` and the pointer vector `sorted` (by
/// nation and band).
///
/// Tournaments (skipped while the current year `G_CURRENT_DATE_YEAR` is 2001): for each of the 35 league entries of
/// `G_LEAGUE_TABLE` that pass `preseason_league_open`, from the nation's first candidate (`first_candidate_of_nation`)
/// up to the second-to-last pointer (the last one never gets in) and while the candidate's club is of the nation
/// (`.nation()`) and a byte counter is below the low byte of the nation's reputation (`nation::REPUTATION`, read as a
/// signed byte): each club that is not human (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0), cdecl), has no
/// date list (`preseason_candidate::PLANNED_DATES`) and is not excluded by the Intertoto (`excluded_by_intertoto`: on
/// `.preseason_plan_day_a()` the club entered in `KEY_CLUB_COMP_INTER_TOTO_CUP`) draws `exe::utils_random`(band
/// `preseason_candidate::REPUTATION_BAND`); the first one with the highest draw (as a signed byte, above −1) hosts. No
/// host, or draw(10) = 0: no tournament for that league. Otherwise the group is [host, 0, 0, 0] and each of the other
/// three slots takes `exe::friendly_find_friendly_opponent`(host, entries, count, host band + 1, group) (5 pushes; ecx =
/// this -> thiscall), up to 50 calls, until the answer has no date list; after slot 1 the host and slot 1 enter each
/// other's opponents list (`add_preseason_opponent`), after slot 2 the pairs (0, 2) and (1, 2). After slot 3 the
/// repeats go (every ordered pair a ≠ b with group[a] = group[b] zeroes group[b]) and the counter becomes 4; with
/// slot 1, 2 or 3 empty, nothing more (the lists stay as they are); otherwise the pairs (0, 3), (1, 3), (2, 3) and
/// `Friendly::create_ai_tournament`(group, 4, kind) (0x5c3be0: 3 pushes; ecx = this -> thiscall, ret 0xc) with kind 2
/// when draw(2) = 0, else 1. Defects of the original: the counter is never initialised before the first tournament (a
/// stack leftover in the original; the copy starts it at 0) and the bound is the low byte of a reputation, so a nation
/// whose reputation has a low byte ≤ 0 (or ≤ 4 after the first tournament) never gets one; with no opponent after 50
/// calls the null slot is added to the lists and the game crashes.
///
/// Tours (every year): for each of the 35 entries that pass `preseason_league_open`, from the nation's first candidate
/// up to the second-to-last pointer and while the club is of the nation: each club that is not human, not excluded by
/// the Intertoto, without an opponents list yet, with a league (`.division()`) registered in `G_COMP_OBJECT_TABLE`, with
/// at most 4 friendlies (candidate +4, signed), that passes draw(3) ≠ 0 and gets a tour nation
/// (`Friendly::pick_tour_nation` 0x5c4680: 1 push; ecx = this -> thiscall): band = `.reputation()` / 500 + draw(2) + 1
/// and up to draw(2) + 3 friendlies: while the candidate's count (+4) is below it,
/// `Friendly::find_tour_opponent`(candidate, sorted, count, band, tour nation) (0x5c5040: 5 pushes; ecx = this ->
/// thiscall); none ends the tour; otherwise the opponent joins the tour (up to 6 slots), both enter each other's list,
/// both counts + 1 and band = band × 0.8 (`rdata::F64_0_8`, _ftol), 10 when below 1. With more than one opponent,
/// `Friendly::schedule_tour_matches`(candidate, tour, n) (0x5c4040: 3 pushes; ecx = this -> thiscall, ret 0xc).
/// Original: `Friendly::plan_preseason_tournaments_and_tours` (0x5c3040, friendly.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn plan_preseason_tournaments_and_tours(this: usize, entries: usize, sorted: usize, count: i32) {
    let pick_tour_nation: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5c4680usize);
    let find_tour_opp: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x5c5040usize);
    let schedule_tour: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x5c4040usize);
    let create_ai_tournament: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x5c3be0usize);
    let n = count as u32;
    let last = sorted.wrapping_add((count as usize).wrapping_mul(4)).wrapping_sub(4);
    if read_u16(G_CURRENT_DATE_YEAR) != 0x7d1 {
        // simplification: the original never initialises this byte before the first tournament (stack leftover); the copy starts at 0
        let mut counter: i8 = 0;
        for k in 0..0x23usize {
            let ent = G_LEAGUE_TABLE + k * 0x48;
            if !preseason_league_open(ent) {
                continue;
            }
            let nation = Nation(read_usize(ent));
            let q = first_candidate_of_nation(sorted, n, nation);
            if q == 0 {
                continue;
            }
            let bound = read_u8(nation.addr() + nation::REPUTATION) as i8; // the original's bound: the reputation's low byte
            let (mut host, mut best) = (0usize, -1i8);
            let mut p = q;
            while p < last {
                let cand = read_usize(p);
                let club = friendly_club_by_id(read_usize(cand));
                if club.nation().addr() != nation.addr() || counter >= bound {
                    break;
                }
                if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0
                    && read_usize(cand + preseason_candidate::PLANNED_DATES) == 0
                    && !excluded_by_intertoto(Friendly(this), club)
                {
                    let r = friendly_random(read_i8(cand + preseason_candidate::REPUTATION_BAND) as i32) as i8;
                    if r > best { // gra21: the highest draw hosts
                        host = cand;
                        best = r;
                    }
                }
                p += 4;
            }
            if host == 0 || friendly_random(10) == 0 {
                continue;
            }
            covered!("gra_21 tournament host");
            let mut group = [host, 0usize, 0, 0];
            let band = (read_u8(host + preseason_candidate::REPUTATION_BAND).wrapping_add(1)) as u32;
            let fill = |group: &mut [usize; 4], slot: usize| {
                let mut tries = 0i8;
                loop {
                    let r = group[slot];
                    if r != 0 && read_usize(r + preseason_candidate::PLANNED_DATES) == 0 {
                        break;
                    }
                    group[slot] = exe::friendly_find_friendly_opponent(this, host, entries, n, band, group.as_ptr() as usize);
                    tries += 1;
                    if tries >= 0x32 {
                        break;
                    }
                }
            };
            let link = |a: usize, b: usize| {
                add_preseason_opponent(a, b);
                add_preseason_opponent(b, a);
            };
            fill(&mut group, 1);
            link(group[0], group[1]);
            fill(&mut group, 2);
            link(group[0], group[2]);
            link(group[1], group[2]);
            fill(&mut group, 3);
            for a in 0..4 {
                for b in 0..4 {
                    if a != b && group[a] == group[b] {
                        group[b] = 0;
                    }
                }
            }
            counter = 4;
            if group[1] == 0 || group[2] == 0 || group[3] == 0 {
                continue;
            }
            link(group[0], group[3]);
            link(group[1], group[3]);
            link(group[2], group[3]);
            let kind = if friendly_random(2) == 0 { 2 } else { 1 }; // gra21: cup or mini league
            covered!("gra_21 tournament created");
            create_ai_tournament(this, group.as_ptr() as usize, 4, kind);
        }
    }
    for k in 0..0x23usize {
        let ent = G_LEAGUE_TABLE + k * 0x48;
        if !preseason_league_open(ent) {
            continue;
        }
        let nation = Nation(read_usize(ent));
        let q = first_candidate_of_nation(sorted, n, nation);
        if q == 0 {
            continue;
        }
        let mut p = q;
        while p < last {
            let cand = read_usize(p);
            p += 4;
            let club = friendly_club_by_id(read_usize(cand));
            if club.nation().addr() != nation.addr() {
                break;
            }
            if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 || excluded_by_intertoto(Friendly(this), club) {
                continue;
            }
            if read_usize(cand + preseason_candidate::OPPONENTS) != 0 {
                continue;
            }
            let lg = club.division().addr();
            if lg == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(lg).wrapping_mul(4)) == 0 {
                continue;
            }
            if read_i8(cand + 4) > 4 || friendly_random(3) == 0 { // gra21: one in three stays home
                continue;
            }
            let tour_nation = pick_tour_nation(this, club.addr());
            if tour_nation == 0 {
                continue;
            }
            let r = friendly_random(2) as u8;
            let mut band = ((read_u16(club.addr() + club::REPUTATION) as i16 as i32 / 500) as u8).wrapping_add(r.wrapping_add(1));
            let how_many = (friendly_random(2) as u8).wrapping_add(3) as i8; // gra21: up to 3 or 4 friendlies
            if read_i8(cand + 4) >= how_many {
                continue;
            }
            covered!("gra_21 tour planned");
            let mut tour = [0usize; 6];
            let mut m: i8 = 0;
            loop {
                let opp = find_tour_opp(this, cand, sorted, n, band as u32, tour_nation);
                if opp == 0 {
                    break;
                }
                tour[m as usize] = opp;
                m += 1;
                add_preseason_opponent(cand, opp);
                add_preseason_opponent(opp, cand);
                write_i8(cand + 4, read_i8(cand + 4).wrapping_add(1));
                write_i8(opp + 4, read_i8(opp + 4).wrapping_add(1));
                let f = ftol(band as i8 as i32 as f64 * read_f64(rdata::F64_0_8)) as u8;
                band = if (f as i8) < 1 { 10 } else { f }; // gra21: the next opponent's band
                if read_i8(cand + 4) >= how_many {
                    break;
                }
            }
            if m > 1 { // gra21: a tour needs two opponents
                covered!("gra_21 tour scheduled");
                schedule_tour(this, cand, tour.as_ptr() as usize, m as u32);
            }
        }
    }
}

/// The opponent club ids in candidate `c`'s list (`preseason_candidate::OPPONENTS`), walked by the nodes
/// (`llist::HEAD`, `llist_node::NEXT`, the id at `llist_node::DATA`) without touching the list's cursor.
unsafe fn preseason_opponent_ids(c: usize) -> Vec<u32> {
    let l = read_usize(c + preseason_candidate::OPPONENTS);
    let mut v = Vec::new();
    if l == 0 {
        return v;
    }
    let mut node = read_usize(l + llist::HEAD);
    while node != 0 && v.len() < 0x1000 {
        v.push(read_u32(read_usize(node + llist_node::DATA)));
        node = read_usize(node + llist_node::NEXT);
    }
    v
}

/// Comparer for `plan_preseason_tournaments_and_tours`: it draws, allocates lists and creates tournaments and tour
/// friendlies, so it cannot run twice: only the original runs and the result is checked afterwards. Every candidate's
/// opponents must be candidates that have it back in their own list (the links are always made in pairs, and before
/// the routine no candidate has a list), and every friendly count (+4) must stay between 0 and 10.
unsafe extern "thiscall" fn compare_plan_preseason_tournaments_and_tours(this: usize, entries: usize, sorted: usize, count: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32)>(%%I0%%)(this, entries, sorted, count);
    let n = count.max(0) as usize;
    let mut ids: Vec<(u32, usize)> = (0..n).map(|k| (read_u32(entries + k * 0x16), entries + k * 0x16)).collect();
    ids.sort();
    let mut bad: Option<(u32, u32)> = None;
    let mut links = 0usize;
    for k in 0..n {
        let c = entries + k * 0x16;
        let me = read_u32(c);
        let count4 = read_i8(c + 4);
        if !(0..=10).contains(&count4) {
            bad = Some((me, count4 as u8 as u32));
            break;
        }
        for id in preseason_opponent_ids(c) {
            links += 1;
            let back = match ids.binary_search_by(|e| e.0.cmp(&id)) {
                Ok(j) => preseason_opponent_ids(ids[j].1).contains(&me),
                Err(_) => false,
            };
            if !back {
                bad = Some((me, id));
                break;
            }
        }
        if bad.is_some() {
            break;
        }
    }
    SCOREBOARD.record(bad.is_none(), || format!("plan_preseason_tournaments_and_tours: {} candidates, {} links, first fault {:x?}", n, links, bad));
}
