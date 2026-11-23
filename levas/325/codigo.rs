// ---------- batch 325: the AI pre-season tournaments and tours (friendly.cpp) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32)>(IDX_PLAN_PRESEASON_TOURNAMENTS_AND_TOURS)(this, entries, sorted, count);
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

// ---------- batch 325: building the tactics store (tactics.cpp) ----------

/// The 28 formation files of `tactics_fields::CUSTOM_TACTICS`, in load order: the name in CM3_TACTICS (a string of the
/// exe) and the source line of its error box. (442_wide.pct is loaded twice, as in the original.)
const CUSTOM_TACTIC_FILES: [(usize, u32); 28] = [
    (0xa8b934, 0x71), // 352_v1.pct
    (0xa8b8fc, 0x72), // 442_v1.pct
    (0xa8b8f0, 0x73), // 532_v1.pct
    (0xa8b8e4, 0x74), // 442_v4.pct
    (0xa8b8d8, 0x75), // 532_v2.pct
    (0xa8b8cc, 0x76), // 532_v3.pct
    (0xa8b8c0, 0x77), // 532_v4.pct
    (0xa8b8b4, 0x78), // 352_v2.pct
    (0xa8b8a8, 0x79), // 532_v5.pct
    (0xa8b89c, 0x7a), // 532_v6.pct
    (0xa8b890, 0x7b), // 532_v7.pct
    (0xa8b884, 0x7c), // 532_v9.pct
    (0xa8b878, 0x7d), // 442_v8.pct
    (0xa8b868, 0x7e), // sweeper_v1.pct
    (0xa8b858, 0x7f), // 442_push.pct
    (0xa8b840, 0x80), // defensive_counter.pct
    (0xa8b834, 0x81), // 4312_v1.pct
    (0xa8b828, 0x82), // 424_v1.pct
    (0xa8b81c, 0x83), // 442_v10.pct
    (0xa8b80c, 0x84), // 442_wide.pct
    (0xa8b800, 0x85), // 4132.pct
    (0xa8b7f0, 0x87), // 343_default.pct
    (0xa8b7e0, 0x88), // 451_default.pct
    (0xa8b80c, 0x89), // 442_wide.pct
    (0xa8b7cc, 0x8a), // sweeper_default.pct
    (0xa8b7b8, 0x8b), // 451_defensive.pct
    (0xa8b7a4, 0x8c), // 343_defensive.pct
    (0xa8b794, 0x8d), // 451_norway.pct
];

/// The 17 formation files of `tactics_fields::PRESET_TACTICS`, in load order: the file, the display name copied over
/// the loaded name (strings of the exe) and the source line of its error box.
const PRESET_TACTIC_FILES: [(usize, usize, u32); 17] = [
    (0xa8b784, 0xa782b0, 0x9d), // 442_default.pct, "4-4-2"
    (0xa8b768, 0xa8b758, 0x9e), // 442_defensive_default.pct, "4-4-2 Defensive"
    (0xa8b858, 0xa8b748, 0x9f), // 442_push.pct, "4-4-2 Attacking"
    (0xa8b730, 0xa8b720, 0xa0), // 442_diamond_default.pct, "4-4-2 Diamond"
    (0xa8b7f0, 0xa1594c, 0xa1), // 343_default.pct, "3-4-3"
    (0xa8b710, 0xa78298, 0xa2), // 352_default.pct, "3-5-2"
    (0xa8b6f4, 0xa8b6e4, 0xa3), // 352_defensive_default.pct, "3-5-2 Defensive"
    (0xa8b6c8, 0xa1593c, 0xa4), // 352_attacking_default.pct, "3-5-2 Attacking"
    (0xa8b6b4, 0xa7828c, 0xa5), // 41212_default.pct, "4-1-2-1-2"
    (0xa8b6a4, 0xa782a0, 0xa6), // 424_default.pct, "4-2-4"
    (0xa8b694, 0xa782a8, 0xa7), // 433_default.pct, "4-3-3"
    (0xa8b7e0, 0xa78284, 0xa8), // 451_default.pct, "4-5-1"
    (0xa8b684, 0xa782b8, 0xa9), // 532_default.pct, "5-3-2"
    (0xa8b668, 0xa8b658, 0xaa), // 532_defensive_default.pct, "5-3-2 Defensive"
    (0xa8b63c, 0xa8b62c, 0xab), // 532_attacking_default.pct, "5-3-2 Attacking"
    (0xa8b7cc, 0x9b75dc, 0xac), // sweeper_default.pct, "Sweeper"
    (0xa8b800, 0xa8b624, 0xad), // 4132.pct, "4-1-3-2"
];

/// The 9 dwords the tactics store writes at +0xc..+0x2c (constants of the original, meaning unknown).
const TACTICS_CONSTANTS: [u32; 9] = [0x280e8a89, 0x280e8b81, 0x281b06c1, 0x381506c1, 0x2a2a06c1, 0x281b2381, 0x288a26c1, 0x101f06c1, 0x381b0381];

/// The CD check of the tactics store: among the drives (GetLogicalDriveStringsA, 200 bytes) the first CD drive
/// (GetDriveTypeA = 5, always asked) holding "<root>cm0102.exe" (`exe::file_find_first`; on the others the search is
/// closed, `exe::file_find_close`); none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with
/// 1) and again, forever. cm0102-1080 has no CD requirement here: the jne on the drive type (0x893e25) became nops and
/// the found test (0x893e67, 85 c0) became xor; the copy reads those bytes of the running exe.
unsafe fn tactics_check_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(0x893e25) == 0x90;
    let always_found = read_u8(0x893e67) != 0x85;
    loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of drives
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 drives
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4);
        text(msg.as_mut_ptr() as usize, 0x987dc4);
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// A new array of `n` formations of `formation_fields::SIZE` bytes, as `new Formation[n]` does: `exe::crt_operator_new`(4
/// + n × size) with the count in the first dword and the constructor 0x5b27c0 run on each
/// (`exe::crt_vector_constructor_iterator`, destructor 0x5a53c0); out of memory, 0.
unsafe fn new_formation_array(n: u32) -> usize {
    let p = exe::crt_operator_new(n.wrapping_mul(formation_fields::SIZE as u32).wrapping_add(4));
    if p == 0 {
        return 0;
    }
    write_u32(p, n);
    exe::crt_vector_constructor_iterator(p + 4, formation_fields::SIZE as u32, n, 0x5b27c0, 0x5a53c0);
    p + 4
}

/// Loads CM3_TACTICS\`file` into formation `slot` (`exe::utils_build_file_path`(0x9ccae4 "CM3_TACTICS", file), cdecl;
/// `Formation::load_from_disk`(path, 0, 1) 0x5b5d10: 3 pushes; ecx = the slot -> thiscall: not quiet, obfuscated).
unsafe fn load_tactic_file(slot: usize, file: usize) -> u32 {
    let load: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x5b5d10usize);
    load(slot, exe::utils_build_file_path(0x9ccae4, file), 0, 1)
}

/// Rule: the tactics store is built once at the start of a game: the 28 formations the AI clubs use and the 17 preset
/// formations come from the CM3_TACTICS files (or from the saved game), and each club and human manager gets an empty
/// selection row.
///
/// `Tactics::Tactics(saved)`: the CD check (`tactics_check_cd`); the 16 undo slots (`tactics_fields::UNDO_SLOTS`, 0x5c
/// bytes each) get their dword +8 zeroed and `tactics_fields::UNDO_COUNT` = 0. With `saved` (the saved game's path) and
/// the tactics loaded from it (0x8985c0: push saved; ecx = this -> thiscall, ret 4; nonzero), that is all. Otherwise:
/// `tactics_fields::CUSTOM_TACTIC_COUNT` = 28 and `tactics_fields::CUSTOM_TACTICS` = a new array of 28 formations
/// (`new_formation_array`); each file of `CUSTOM_TACTIC_FILES` in turn: while fewer than the count are loaded, it is
/// loaded into the next slot (`load_tactic_file`) and counts when the load succeeds; with the count already reached,
/// the file's error box (`show_error_at` with tactics.cpp, its line); at the end, fewer than the count loaded: the box at
/// line 0x92. The same with `tactics_fields::PRESET_TACTIC_COUNT` = 17, `tactics_fields::PRESET_TACTICS` and
/// `PRESET_TACTIC_FILES` (end box at line 0xb4), each loaded preset getting its display name copied over its name (+0).
/// Then `tactics_fields::ROW_COUNT` = `G_CLUB_COUNT` + 32 rows of 0x91 bytes (`tactics_fields::ROWS`,
/// `exe::crt_malloc`; out of memory: the box at line 0xc3 and return), each 0, 0 and then 0x8f bytes of 0xff (no
/// selection, captain, playmakers, set piece or shoot-out takers); the per-tactic statistics (`tactics_fields::
/// TACTIC_STATS`, +4 = 28 + 17 records of 0x28 bytes, `exe::crt_operator_new`; out of memory: the box at line 0xe2 and
/// return), each zeroed with its index in the first word; and the 9 constants `TACTICS_CONSTANTS` at +0xc..+0x2c.
/// Returns this.
/// Original: `Tactics::Tactics` (0x893dc0, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_tactics(this: usize, saved: usize) -> usize {
    let load_saved: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8985c0usize);
    tactics_check_cd();
    for k in 0..0x10 {
        write_u32(this + tactics_fields::UNDO_SLOTS + 8 + 0x5c * k, 0);
    }
    write_u8(this + tactics_fields::UNDO_COUNT, 0);
    if saved != 0 && load_saved(this, saved) != 0 {
        covered!("gra_23 tactics from the saved game");
        return this;
    }
    covered!("gra_23 tactics from the files");
    write_u32(this + tactics_fields::CUSTOM_TACTIC_COUNT, 0x1c);
    write_usize(this + tactics_fields::CUSTOM_TACTICS, new_formation_array(0x1c));
    let mut loaded: i32 = 0;
    for &(file, line) in CUSTOM_TACTIC_FILES.iter() {
        if loaded < read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT) {
            let slot = read_usize(this + tactics_fields::CUSTOM_TACTICS) + loaded as usize * formation_fields::SIZE;
            if load_tactic_file(slot, file) != 0 {
                loaded += 1; // gra23: a loaded AI formation
            }
        } else {
            show_error_at(0xa8b908, line);
        }
    }
    if loaded < read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT) {
        show_error_at(0xa8b908, 0x92);
    }
    write_u32(this + tactics_fields::PRESET_TACTIC_COUNT, 0x11);
    write_usize(this + tactics_fields::PRESET_TACTICS, new_formation_array(0x11));
    let mut loaded: i32 = 0;
    for &(file, name, line) in PRESET_TACTIC_FILES.iter() {
        if loaded < read_i32(this + tactics_fields::PRESET_TACTIC_COUNT) {
            let slot = read_usize(this + tactics_fields::PRESET_TACTICS) + loaded as usize * formation_fields::SIZE;
            if load_tactic_file(slot, file) != 0 {
                let mut b = 0;
                loop {
                    let ch = read_u8(name + b);
                    write_u8(slot + b, ch); // gra23: the preset's display name (inline strcpy, the zero included)
                    if ch == 0 {
                        break;
                    }
                    b += 1;
                }
                loaded += 1;
            }
        } else {
            show_error_at(0xa8b908, line);
        }
    }
    if loaded < read_i32(this + tactics_fields::PRESET_TACTIC_COUNT) {
        show_error_at(0xa8b908, 0xb4);
    }
    let rows = read_i32(G_CLUB_COUNT).wrapping_add(0x20);
    write_i32(this + tactics_fields::ROW_COUNT, rows);
    let r = exe::crt_malloc((rows as u32).wrapping_mul(0x91));
    write_usize(this + tactics_fields::ROWS, r);
    if r == 0 {
        show_error_at(0xa8b908, 0xc3);
        return this;
    }
    for k in 0..rows.max(0) as usize {
        let row = r + k * 0x91;
        write_u16(row, 0);
        for b in 2..0x91 {
            write_u8(row + b, 0xff); // gra23: an empty selection row
        }
    }
    let n = read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT).wrapping_add(read_i32(this + tactics_fields::PRESET_TACTIC_COUNT));
    write_i32(this + 4, n);
    let s = exe::crt_operator_new((n as u32).wrapping_mul(0x28));
    write_usize(this + tactics_fields::TACTIC_STATS, s);
    if s == 0 {
        show_error_at(0xa8b908, 0xe2);
        return this;
    }
    for k in 0..n.max(0) as usize {
        for b in (0..0x28).step_by(4) {
            write_u32(s + k * 0x28 + b, 0);
        }
        write_u16(s + k * 0x28, k as u16);
    }
    for (k, &c) in TACTICS_CONSTANTS.iter().enumerate() {
        write_u32(this + 0xc + 4 * k, c);
    }
    this
}

/// The bytes of a loaded formation that the comparer looks at: the name (+0, up to its zero) and the fields
/// `Formation::load_from_disk` writes (password, creator, positions, display order, coordinates, flags, marking types,
/// the position lookup, the mask and flags at +0x187b..+0x188b and the player instructions), not the padding the
/// allocation leaves.
unsafe fn tactic_block_bytes(f: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut k = 0;
    while k < 0x33 && read_u8(f + k) != 0 {
        v.push(read_u8(f + k));
        k += 1;
    }
    for &(o, n) in &[(0x33usize, 0x32usize), (0x65, 200), (0x12d, 0x16), (0x143, 0xb), (0x14e, 0x16), (0x164, 0x420), (0x584, 4), (0x5b4, 0xb), (0x5eb, 0x1290), (0x187b, 0x10), (0x188b, 0x58)] {
        v.extend((0..n).map(|b| read_u8(f + o + b)));
    }
    v
}

/// Comparer for `construct_tactics`. With a saved game only the original runs (the save is read once). In a new game
/// the original builds the real store and the copy builds a second one in a scratch object; the counts, the loaded
/// formations (`tactic_block_bytes`), the rows, the statistics, the undo slots and the constants must be equal; the
/// copy's arrays are then freed (the formations with their destructor, as `delete[]`).
unsafe extern "thiscall" fn compare_construct_tactics(this: usize, saved: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_TACTICS)(this, saved);
    if saved != 0 {
        return r;
    }
    let scratch = vec![0u8; 0x609];
    let c = scratch.as_ptr() as usize;
    let rc = construct_tactics(c, 0);
    let summary = |t: usize| -> Vec<u8> {
        let mut v = Vec::new();
        let custom = read_i32(t + tactics_fields::CUSTOM_TACTIC_COUNT);
        let preset = read_i32(t + tactics_fields::PRESET_TACTIC_COUNT);
        let rows = read_i32(t + tactics_fields::ROW_COUNT);
        let stats = read_i32(t + 4);
        for x in [custom, preset, rows, stats] {
            v.extend_from_slice(&x.to_le_bytes());
        }
        v.extend((0xc..0x30).map(|b| read_u8(t + b)));
        for k in 0..0x10 {
            v.extend_from_slice(&read_u32(t + tactics_fields::UNDO_SLOTS + 8 + 0x5c * k).to_le_bytes());
        }
        v.push(read_u8(t + tactics_fields::UNDO_COUNT));
        for (arr, n) in [(read_usize(t + tactics_fields::CUSTOM_TACTICS), custom), (read_usize(t + tactics_fields::PRESET_TACTICS), preset)] {
            if arr != 0 {
                for k in 0..n.max(0) as usize {
                    v.extend(tactic_block_bytes(arr + k * formation_fields::SIZE));
                }
            }
        }
        let rw = read_usize(t + tactics_fields::ROWS);
        if rw != 0 {
            v.extend((0..rows.max(0) as usize * 0x91).map(|b| read_u8(rw + b)));
        }
        let st = read_usize(t + tactics_fields::TACTIC_STATS);
        if st != 0 {
            v.extend((0..stats.max(0) as usize * 0x28).map(|b| read_u8(st + b)));
        }
        v
    };
    let (mo, mr) = (summary(this), summary(c));
    for (field, count) in [(tactics_fields::CUSTOM_TACTICS, tactics_fields::CUSTOM_TACTIC_COUNT), (tactics_fields::PRESET_TACTICS, tactics_fields::PRESET_TACTIC_COUNT)] {
        let a = read_usize(c + field);
        if a != 0 {
            exe::crt_vector_destructor_iterator(a, formation_fields::SIZE as u32, read_u32(c + count), 0x5a53c0);
            exe::crt_operator_delete(a - 4);
        }
    }
    if read_usize(c + tactics_fields::ROWS) != 0 {
        exe::crt_free(read_usize(c + tactics_fields::ROWS));
    }
    if read_usize(c + tactics_fields::TACTIC_STATS) != 0 {
        exe::crt_operator_delete(read_usize(c + tactics_fields::TACTIC_STATS));
    }
    let first = mo.iter().zip(mr.iter()).position(|(a, b)| a != b);
    SCOREBOARD.record(r == this && rc == c && mo == mr, || format!("construct_tactics({:#x}): {} and {} bytes compared, first different at {:?}", this, mo.len(), mr.len(), first));
    r
}
