use super::*;

/// Constructs a cup knockout stage: the round table copied from the cup's template (dates, draws), the year, the stage,
/// the id and the parent's rules (`cup::SUBSTITUTES_NAMED`, `cup::SUBSTITUTIONS_ALLOWED`), the team list with pot
/// seeds, and the fixture tree.
///
/// `Cup_Stage::Cup_Stage(parent, n, teams, rounds, a5, template, year, stage, b2, id, b3, arvore, by_order, a14,
/// seeds)`; SEH frame with no destructor on the normal path. The base (Cup::Cup `exe::new_cup`, mov ecx -> thiscall,
/// ret 0); +4 = a5; `.set_year()` = the year (word); `.set_stage_index()` = the stage, `.set_club_sides()` = b2
/// (bytes); `.set_current_stage()` = `.set_stage_count()` = −1; `.set_importance_bias()` = b3; the vtable 0x969468;
/// `.set_comp_type(comp_type::CUP)`; `.set_run_by_stages()` = `.set_current_round()` = `.set_next_draw_round()` = 0;
/// `.set_round_count()` = the rounds and `.set_stage_label()` = the id (words); `cup::SUBSTITUTES_NAMED` and
/// `cup::SUBSTITUTIONS_ALLOWED` copied from the parent; `.set_prize_fund()` = a14. The round table (`exe::crt_malloc`
/// of rounds × 0x68 at `.set_round_table()`; out of memory, the box for line 0x48) is copied from the template;
/// `.set_team_count()` = n and the team list (calloc `exe::crt_calloc`(n, 6) at `.set_team_table()`; out of memory, the
/// box for line 0x53): each team gets status (+5) 0 and its pot seed (+4): the one from the `seeds` vector (given by
/// the cup), or, without it, the position in the list if `by_order`, or 0. Finally the fixture tree, built at once:
/// with `arvore`, Cup_Stage::map_fixture_tree (0x5241e0, which links the winners and places the seeds in the first
/// round), otherwise Cup::map_fixture_tree (`map_fixture_tree`, which links at random), both thiscall, ret 0 (they
/// draw); if negative, the box for line 0x62 or 0x6a. Returns this.
/// Original: `Cup_Stage::Cup_Stage` (0x522e00, cup_stage.cpp, thiscall, ret 0x3c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn construct_cup_stage(this: usize, parent: usize, n: u32, teams: usize, rounds: u32, a5: u32, template: usize, year: u32, stage: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, by_order: u32, a14: u32, seeds: usize) -> usize {
    let this = Cup(this);
    const F: usize = 0x9b5944;
    exe::new_cup(this);
    write_usize(this.addr() + 4, a5 as usize);
    this.set_year((year as u16) as i16); // cqs7: the stage year
    this.set_club_sides(b2 as u8);
    this.set_stage_index(stage as u8);
    this.set_current_stage(usize::MAX);
    this.set_stage_count(usize::MAX);
    this.set_importance_bias(b3 as u8);
    write_usize(this.addr(), 0x969468);
    this.set_comp_type(comp_type::CUP);
    this.set_run_by_stages(0);
    this.set_current_round(0);
    this.set_next_draw_round(0);
    this.set_round_count((rounds as u16) as i16);
    this.set_stage_label((id as u16) as i16);
    this.set_substitutes_named(read_u8(parent + cup::SUBSTITUTES_NAMED));
    this.set_substitutions_allowed(read_u8(parent + cup::SUBSTITUTIONS_ALLOWED));
    this.set_prize_fund((a14 as usize) as i32);
    let size = (rounds as u16 as i16 as i32 * 0x68) as u32;
    let t = exe::crt_malloc(size);
    this.set_round_table(t);
    if t == 0 {
        show_error_at(F, 0x48);
        return this.addr();
    }
    core::ptr::copy_nonoverlapping(template as *const u8, t as *mut u8, size as usize);
    this.set_team_count((n as u16) as i16);
    let l = exe::crt_calloc(n as u16 as i16 as i32 as u32, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(F, 0x53);
        return this.addr();
    }
    covered!("cqs_7 cup stage");
    let m = this.team_count() as i32;
    let mut i = 0i32;
    while i < m {
        let e = i as usize * 6;
        write_usize(this.team_table() + e, read_usize(teams + i as usize * 4)); // cqs7: o time i
        write_u8(this.team_table() + e + 5, 0);
        let s = if seeds != 0 {
            read_u8(seeds + i as usize) // cqs7: the vector seed
        } else if by_order != 0 {
            i as u8
        } else {
            0
        };
        write_u8(this.team_table() + e + 4, s);
        i += 1;
    }
    let (map, linha) = if arvore != 0 { (0x5241e0usize, 0x62) } else { (0x518790usize, 0x6a) };
    let map: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(map);
    if map(this.addr()) < 0 {
        show_error_at(F, linha);
    }
    this.addr()
}

/// Destroys a knockout stage: frees the team list, the round table and each round's fixtures, then destroys the base.
///
/// `Cup_Stage::~Cup_Stage()`, the destructor, called by the virtual destructor. Sets the vtable 0x969468; the team list
/// (`.team_table()`), the round table (`.round_table()`) and, if there are fixtures (`.tie_tables()`), each fixture
/// vector of the `.round_count()` rounds and the vector itself are freed (`exe::crt_free`, cdecl; the pointers are left
/// in place); finally the base (Cup::~Cup `exe::drop_cup`, mov ecx -> thiscall, ret 0), whose eax it returns.
/// Original: `Cup_Stage::~Cup_Stage` (0x523300, cup_stage.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_cup_stage(this: usize) -> usize {
    let this = Cup(this);
    write_usize(this.addr(), 0x969468);
    if this.team_table() != 0 {
        exe::crt_free(this.team_table());
    }
    if this.round_table() != 0 {
        exe::crt_free(this.round_table());
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
    }
    covered!("cqs_7 end of cup stage");
    exe::drop_cup(this)
}

// simplification: frees the tables and destroys the database; cannot run twice: original only

/// Builds the fixture tree of a cup stage made with a tree (`Cup_Stage::Cup_Stage` with `arvore`; the Champions
/// League knockout stage after the groups): the ties of each round, each tie linked to a tie of the next round (by
/// pair, or drawn and then the first free side), and the entrants of each round in its ties by pair. Returns 1, or −1
/// with the error box.
///
/// `Cup_Stage::map_fixture_tree()` (thiscall, ret 0). The tie arrays (`exe::crt_malloc`: 4 per round into
/// `tie_tables()` and `cup_round::TIES` × 0x35 per round; out of memory, the box of line 0x201 or 0x20b of
/// cup_stage.cpp and −1: not the fatal box of Cup::map_fixture_tree); status +5 of each team (`team_table()`, 6 bytes)
/// cleared. Without `season_set_up()`: 1 here (`current_round()` and `next_draw_round()` untouched). Otherwise
/// `current_round()` = `next_draw_round()` = 0 and each tie is emptied (the words +0, +2, +6, +0xa, +0xc, +0xe, +0x10 and
/// +0x12 = −1, `cup_tie::FEEDER_SIDE` 0, +0x14..+0x31 = 0xff, `cup_tie::REPLAYS_PLAYED`, `cup_tie::NEXT_MATCH` and
/// `cup_tie::FIXTURE_STATE` 0; +4 untouched).
///
/// m = the rounds, less one when the last round's code (`cup_round::ROUND_CODE`) is 0x8c (third place). For each round
/// r below m − 1, each tie p in order (no shuffle, unlike Cup::map_fixture_tree): the slot in round r + 1 is p/2 when
/// that round's `cup_round::TIE_RULES` has 0x200, otherwise drawn (utils_random `exe::utils_random`(its ties)); from it,
/// `link_tie_to_round`, with `cup_tie::FEEDER_ROUND` = r + 1. With no free side in round r + 1: in the last two rounds
/// before m, the fatal box of line 0x294 (the game stops); otherwise a slot drawn in round r + 2 and the same search
/// (`cup_tie::FEEDER_ROUND` = r + 2; none free: the fatal box of line 0x2bf).
///
/// With no ties in round 0: `run_by_stages()` = 1 and 1. Otherwise, for each round r below m, each entrant q
/// (`cup_round::NEW_TEAMS`, from `cup_round::FIRST_NEW_TEAM`) goes to tie q/2: to the home team when it and
/// `cup_tie::HOME_LINK_TIE` are free, otherwise to the away team when it and `cup_tie::AWAY_LINK_TIE` are free; a taken
/// tie with both teams set and the cup's `venue_stadium_id()` set has its teams swapped when the away club's stadium
/// (`club::STADIUM`) is that venue, and the search moves to the next tie (back to 0 after the last; all taken: the box
/// of line 0x310 and −1).
///
/// Finally the number of the first tie of each round (`cup_round::FIRST_TIE_NUMBER`), from back to front: with a
/// third-place round, from the round before the final, starting at 5; otherwise from the final's round − 1, starting
/// at the last round's +0x18 byte + 1; each round adds its ties (byte). Returns 1.
/// Original: `Cup_Stage::map_fixture_tree` (0x5241e0, cup_stage.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cup_stage_map_fixture_tree(this: usize) -> i32 {
    let this = Cup(this);
    let t = exe::crt_malloc((this.round_count() as i32 as u32) << 2);
    this.set_tie_tables(t);
    if t == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x201);
        return -1;
    }
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let n = read_i16(comp_cup_round(this, i) + cup_round::TIES) as i32;
        write_usize(this.tie_tables() + i as usize * 4, exe::crt_malloc((n * 0x35) as u32));
        if read_usize(this.tie_tables() + i as usize * 4) == 0 {
            show_error_at(CUP_STAGE_SOURCE, 0x20b);
            return -1;
        }
        i += 1;
    }
    if this.team_table() != 0 {
        let mut k = 0i32;
        while k < this.team_count() as i32 {
            write_u8(this.team_table() + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    if this.season_set_up() == 0 {
        return 1;
    }
    this.set_current_round(0);
    this.set_next_draw_round(0);
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let mut j = 0i32;
        while j < read_i16(comp_cup_round(this, i) + cup_round::TIES) as i32 {
            let c = cup_tie(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                write_u16(c + k, 0xffff);
            }
            write_u16(c + cup_tie::FEEDER_SIDE, 0);
            for k in 0x14..=0x31usize {
                write_u8(c + k, 0xff);
            }
            write_u8(c + cup_tie::REPLAYS_PLAYED, 0);
            write_u8(c + cup_tie::NEXT_MATCH, 0);
            write_u8(c + cup_tie::FIXTURE_STATE, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let mut p = 0i32;
        while p < read_i16(comp_cup_round(this, r) + cup_round::TIES) as i32 {
            let next = comp_cup_round(this, r + 1);
            let s = if read_i16(next + cup_round::TIE_RULES) as u16 & 0x200 != 0 {
                p / 2 // cqy4: the slot by the pair
            } else {
                exe::utils_random(read_i16(next + cup_round::TIES) as i32)
            };
            if link_tie_to_round(this, r, p, r + 1, s) {
                covered!("cqy_4 tie linked to the next round");
                write_u16(cup_tie(this, r, p) + cup_tie::FEEDER_ROUND, (r + 1) as u16);
            } else {
                if r >= m - 2 {
                    fatal_error_at(CUP_STAGE_SOURCE, 0x294);
                }
                let s2 = exe::utils_random(read_i16(comp_cup_round(this, r + 2) + cup_round::TIES) as i32);
                if !link_tie_to_round(this, r, p, r + 2, s2) {
                    fatal_error_at(CUP_STAGE_SOURCE, 0x2bf);
                }
                write_u16(cup_tie(this, r, p) + cup_tie::FEEDER_ROUND, (r + 2) as u16);
            }
            p += 1;
        }
        r += 1;
    }
    if read_i16(comp_cup_round(this, 0) + cup_round::TIES) == 0 {
        this.set_run_by_stages(1);
        return 1;
    }
    let mut r = 0i32;
    while r < m {
        let mut q = 0i32;
        while q < read_i16(comp_cup_round(this, r) + cup_round::NEW_TEAMS) as i32 {
            let start = q / 2; // cqy4: the entrant's tie by the pair
            let mut s = start;
            loop {
                let c = cup_tie(this, r, s as i16 as i32);
                let home = read_i16(c + cup_tie::HOME_TEAM);
                let entrant = (read_i16(comp_cup_round(this, r) + cup_round::FIRST_NEW_TEAM) as u16).wrapping_add(q as u16);
                if home == -1 && read_i16(c + cup_tie::HOME_LINK_TIE) == -1 {
                    covered!("cqy_4 entrant at home");
                    write_u16(c + cup_tie::HOME_TEAM, entrant);
                    break;
                }
                if read_i16(c + cup_tie::AWAY_TEAM) == -1 && read_i16(c + cup_tie::AWAY_LINK_TIE) == -1 {
                    write_u16(c + cup_tie::AWAY_TEAM, entrant);
                    break;
                }
                let away = read_i16(c + cup_tie::AWAY_TEAM);
                if this.venue_stadium_id() as u32 != u32::MAX && home != -1 && away != -1 {
                    let club = Club(read_usize(this.team_table() + (away as isize * 6) as usize));
                    if !club.is_null() {
                        let stadium = club.stadium().addr();
                        if stadium != 0 && read_usize(stadium) == this.venue_stadium_id() {
                            write_u16(c + cup_tie::HOME_TEAM, away as u16);
                            write_u16(c + cup_tie::AWAY_TEAM, home as u16);
                        }
                    }
                }
                s = if (s as i16 as i32) < read_i16(comp_cup_round(this, r) + cup_round::TIES) as i32 - 1 { s + 1 } else { 0 };
                if s as i16 == start as i16 {
                    show_error_at(CUP_STAGE_SOURCE, 0x310);
                    return -1;
                }
            }
            q += 1;
        }
        r += 1;
    }
    let nr = this.round_count();
    if nr > 0 && read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + cup_round::FIRST_TIE_NUMBER, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + cup_round::TIES));
            k -= 1;
        }
    } else {
        let mut k = nr.wrapping_sub(2) as i32;
        if k >= 0 {
            let mut n = read_u8(comp_cup_round(this, nr as i32 - 1) + cup_round::TEAMS).wrapping_add(1);
            while k >= 0 {
                write_u8(comp_cup_round(this, k) + cup_round::FIRST_TIE_NUMBER, n);
                n = n.wrapping_add(read_u8(comp_cup_round(this, k) + cup_round::TIES));
                k -= 1;
            }
        }
    }
    1
}

/// Comparer for `cup_stage_map_fixture_tree`: only the original runs (it allocates and draws); afterwards, on a season
/// set up and a return of 1, the plan (`cup_stage_tree_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_map_fixture_tree(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CUP_STAGE_MAP_FIXTURE_TREE)(this);
    let cup = Cup(this);
    if r != 1 || cup.season_set_up() == 0 {
        return r;
    }
    let error = cup_stage_tree_mismatch(cup);
    SCOREBOARD.record(error.is_none(), || format!("cup_stage_map_fixture_tree({:#x}): {}", this, error.clone().unwrap_or_default()));
    r
}

/// Links tie p of round `r` to a free side of a tie of round `target` (the search starts at tie `s`): the home side
/// (`cup_tie::HOME_LINK_TIE` free: the slot keeps p and r, the tie keeps the slot and `cup_tie::FEEDER_SIDE` 1) or else
/// the away side (`cup_tie::AWAY_LINK_TIE`, `cup_tie::FEEDER_SIDE` 0), moving to the next tie (back to 0 after the
/// round's last) while both are taken. Returns false, with nothing written, when it comes back to `s`.
pub(crate) unsafe fn link_tie_to_round(this: Cup, r: i32, p: i32, target: i32, s: i32) -> bool {
    let start = s;
    let mut s = s;
    let c = cup_tie(this, r, p);
    loop {
        let v = cup_tie(this, target, s as i16 as i32);
        let side = if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 {
            Some(true) // cqy4: the home side first
        } else if read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 {
            Some(false)
        } else {
            None
        };
        if let Some(home) = side {
            write_u16(v + if home { cup_tie::HOME_LINK_TIE } else { cup_tie::AWAY_LINK_TIE }, p as u16);
            write_u16(v + if home { cup_tie::HOME_LINK_ROUND } else { cup_tie::AWAY_LINK_ROUND }, r as u16);
            write_u16(c + cup_tie::FEEDER_TIE, s as u16);
            write_u16(c + cup_tie::FEEDER_SIDE, home as u16);
            return true;
        }
        s = if (s as i16 as i32) < read_i16(comp_cup_round(this, target) + cup_round::TIES) as i32 - 1 { s + 1 } else { 0 };
        if s as i16 == start as i16 {
            return false;
        }
    }
}

/// The first broken part of a built stage tree, or None: each tie of the rounds before m − 1 must be the one its
/// feeder slot points back to (`cup_tie::FEEDER_ROUND` r + 1 or r + 2, the slot's link tie and link round), and each
/// entrant of each round below m must be the home or away team of exactly one tie of its round.
pub(crate) unsafe fn cup_stage_tree_mismatch(this: Cup) -> Option<String> {
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + cup_round::TIES) as i32 {
            let c = cup_tie(this, rd, p);
            let target = read_i16(c + cup_tie::FEEDER_ROUND) as i32;
            let home = read_i16(c + cup_tie::FEEDER_SIDE) == 1;
            let ok = (target == rd + 1 || target == rd + 2) && {
                let v = cup_tie(this, target, read_i16(c + cup_tie::FEEDER_TIE) as i32);
                read_i16(v + if home { cup_tie::HOME_LINK_TIE } else { cup_tie::AWAY_LINK_TIE }) as i32 == p
                    && read_i16(v + if home { cup_tie::HOME_LINK_ROUND } else { cup_tie::AWAY_LINK_ROUND }) as i32 == rd
            };
            if !ok {
                return Some(format!("tie {} of round {} without its slot", p, rd));
            }
        }
    }
    if read_i16(comp_cup_round(this, 0) + cup_round::TIES) == 0 {
        return None;
    }
    for rd in 0..m.max(0) {
        let first = read_i16(comp_cup_round(this, rd) + cup_round::FIRST_NEW_TEAM) as i32;
        for q in 0..read_i16(comp_cup_round(this, rd) + cup_round::NEW_TEAMS) as i32 {
            let mut seen = 0;
            for p in 0..read_i16(comp_cup_round(this, rd) + cup_round::TIES) as i32 {
                let c = cup_tie(this, rd, p);
                seen += (read_i16(c + cup_tie::HOME_TEAM) as i32 == first + q) as i32 + (read_i16(c + cup_tie::AWAY_TEAM) as i32 == first + q) as i32;
            }
            if seen != 1 {
                return Some(format!("entrant {} of round {} placed {} teams", first + q, rd, seen));
            }
        }
    }
    None
}

/// The cup_stage.cpp source path of the error boxes.
pub(crate) const CUP_STAGE_SOURCE: usize = 0x9b5944;

/// Builds an empty cup stage for loading: the Cup base (`exe::new_cup`), the Cup_Stage vtable, the team table of
/// `teams` 6-byte entries (`cup::TEAM_TABLE`), the round table of `rounds` 0x68-byte rounds (`cup::ROUND_TABLE`), the
/// tie vector of `rounds` pointers (`cup::TIE_TABLES`) and, for each round, its `tie_counts[r]` 0x35-byte ties
/// (`exe::crt_malloc`, the counts taken as 16 bits). The team and round counts are not written here (the load reads
/// them). An allocation that fails shows the cup_stage.cpp box (lines 0x7e, 0x85, 0x8d, 0x97 for a round's ties) and
/// stops there. Returns the object.
/// Original: `Cup_Stage::Cup_Stage` (0x5230e0, cup_stage.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this, CUP_STAGE_VTABLE);
    let (teams, rounds) = (teams as i16 as i32, rounds as i16 as i32);
    cup.set_team_table(exe::crt_malloc((teams * 6) as u32));
    if cup.team_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x7e);
        return this;
    }
    cup.set_round_table(exe::crt_malloc((rounds * cup_round::SIZE as i32) as u32));
    if cup.round_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x85);
        return this;
    }
    cup.set_tie_tables(exe::crt_malloc((rounds * 4) as u32));
    if cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x8d);
        return this;
    }
    covered!("cx02_17 cup stage built for loading");
    for r in 0..rounds.max(0) as usize {
        let ties = exe::crt_malloc((*tie_counts.add(r) as i32 * TIE_SIZE as i32) as u32);
        write_usize(cup.tie_tables() + 4 * r, ties);
        if ties == 0 {
            show_error_at(CUP_STAGE_SOURCE, 0x97);
            break;
        }
    }
    this
}

/// Reads a cup stage back from the cup's temporary file, in the order `cup_stage_save_to_file` wrote it: the fields
/// (`CUP_STAGE_FILE_FIELDS`, one by one), the competition id (the record of that id in [`G_CLUB_COMP_TABLE`] into
/// `league::COMPETITION`, −1 none), the round table (`ROUND_COUNT` rounds of 0x68 bytes), the team table (`TEAM_COUNT`
/// 6-byte entries, their club ids turned into clubs: `club_entries_from_ids`) and each round's ties (the round's
/// `cup_round::TIES` × 0x35 bytes, one read). Any short read shows the box of its line and the game stops. Returns 1.
/// Defect of the original: the team table's read is checked against the round table's count read (the check of line
/// 0x163 repeats the round table's), so a short team table goes unnoticed.
/// Original: `Cup_Stage::load_from_file` (0x523380, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let this = League(this);
    let cup = Cup(this.addr());
    for (field, size, line) in CUP_STAGE_FILE_FIELDS {
        cup_stage_read(this.addr() + field, size, 1, file, 1, line);
    }
    let mut competition = 0i32;
    cup_stage_read(&mut competition as *mut i32 as usize, 4, 1, file, 1, 0x14b);
    write_usize(this.addr() + league::COMPETITION, if competition == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((competition as usize).wrapping_mul(0x6b)) });
    let rounds = cup.round_count() as i32 as u32;
    cup_stage_read(cup.round_table(), cup_round::SIZE as u32, rounds, file, rounds, 0x15b);
    exe::crt_fread(cup.team_table(), 6, cup.team_count() as i32 as u32, file); // checked against the round count: passes
    club_entries_from_ids(cup.team_table(), cup.team_count());
    covered!("cx02_17 cup stage loaded");
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        cup_stage_read(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file, 1, 0x17a);
        r += 1;
    }
    1
}

/// Writes a cup stage into the cup's temporary file (`exe::crt_fwrite`, unchecked), in the order
/// `cup_stage_load_from_file` reads it: without a round table, team table or tie vector, the cup_stage.cpp box (line
/// 0x19a) and nothing. The fields (`CUP_STAGE_FILE_FIELDS`), the competition record's id (its first dword; without a
/// record the box of line 0x1b9 and −1), the round table, the team table with its clubs written as ids and turned back
/// into clubs, and each round's ties.
/// Original: `Cup_Stage::save_to_file` (0x523eb0, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_save_to_file(this: usize, file: usize) {
    let this = League(this);
    let cup = Cup(this.addr());
    let in_the_competition = u32::MAX;
    if cup.round_table() == 0 || cup.team_table() == 0 || cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x19a);
        return;
    }
    covered!("cx02_17 cup stage saved");
    for (field, size, _) in CUP_STAGE_FILE_FIELDS {
        exe::crt_fwrite(this.addr() + field, size, 1, file);
    }
    let competition = this.competition().addr();
    if competition != 0 {
        exe::crt_fwrite(competition, 4, 1, file);
    } else {
        show_error_at(CUP_STAGE_SOURCE, 0x1b9);
        exe::crt_fwrite(&in_the_competition as *const u32 as usize, 4, 1, file);
    }
    exe::crt_fwrite(cup.round_table(), cup_round::SIZE as u32, cup.round_count() as i32 as u32, file);
    club_entries_to_ids(cup.team_table(), cup.team_count());
    exe::crt_fwrite(cup.team_table(), 6, cup.team_count() as i32 as u32, file);
    club_entries_from_ids(cup.team_table(), cup.team_count());
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        exe::crt_fwrite(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file);
        r += 1;
    }
}

/// Comparer for `new_cup_stage_for_load`: it allocates (it cannot run twice), so only the original runs; the vtable,
/// the return and the three tables must be there.
pub(crate) unsafe extern "thiscall" fn compare_new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *const i16) -> usize>(IDX_NEW_CUP_STAGE_FOR_LOAD)(this, teams, rounds, tie_counts);
    let cup = Cup(this);
    SCOREBOARD.record(r == this && read_usize(this) == CUP_STAGE_VTABLE && cup.team_table() != 0 && cup.round_table() != 0 && cup.tie_tables() != 0,
        || format!("new_cup_stage_for_load({:#x}, {}, {}) returned {:#x}, vtable {:#x}", this, teams as i16, rounds as i16, r, read_usize(this)));
    r
}

/// Comparer for `cup_stage_load_from_file`: the original reads; the file goes back to where it was and the copy reads
/// the same bytes again over the same tables; the object, the tables, the return and the file position must match.
/// The original's state is put back.
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let at = exe::crt_ftell(file);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CUP_STAGE_LOAD_FROM_FILE)(this, file);
    let end = exe::crt_ftell(file);
    let regions = cup_stage_loaded_regions(this);
    let original = crate::comparison::MemorySnapshot::read(&regions);
    exe::crt_fseek(file, at, 0);
    let c = cup_stage_load_from_file(this, file);
    let copy = crate::comparison::MemorySnapshot::read(&regions);
    let end_c = exe::crt_ftell(file);
    original.write_back();
    exe::crt_fseek(file, end, 0);
    SCOREBOARD.record(r == c && end == end_c && original == copy, || format!("cup_stage_load_from_file({:#x}) returned {} x {}, file at {} x {} (from {}), state equal {}", this, r, c, end, end_c, at, original == copy));
    r
}

/// Comparer for `cup_stage_save_to_file`: it writes the cup's temporary file (it cannot run twice), so only the
/// original runs; the team table must come back with its clubs. The copy is proven by the save proof (and by the
/// cup's file comparer of `cup_save_cup_to_disk`, which calls it).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_save_to_file(this: usize, file: usize) {
    let cup = Cup(this);
    let n = if cup.team_table() != 0 { cup.team_count().max(0) as usize } else { 0 };
    let before: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CUP_STAGE_SAVE_TO_FILE)(this, file);
    let after: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    SCOREBOARD.record(before == after, || format!("cup_stage_save_to_file({:#x}): the team table changed", this));
}

/// Turns the ids of the first `count` 6-byte entries of `book` back into club pointers (null for −1), in place.
pub(crate) unsafe fn club_entries_from_ids(book: usize, count: i16) {
    for i in 0..count.max(0) as usize {
        let e = book + 6 * i;
        let id = read_i32(e);
        write_usize(e, if id != -1 { club_at(id as isize).addr() } else { 0 });
    }
}

/// One read of a cup stage's load: `count` items of `size` bytes at `at` (`exe::crt_fread`); fewer than `expected`
/// shows the cup_stage.cpp box at `line` and the game stops (`fatal_error_at`).
pub(crate) unsafe fn cup_stage_read(at: usize, size: u32, count: u32, file: usize, expected: u32, line: u32) {
    if exe::crt_fread(at, size, count, file) != expected {
        fatal_error_at(CUP_STAGE_SOURCE, line);
    }
}

/// The bytes a cup stage's load leaves: the object's first 0xb2 bytes, the round table, the team table and each
/// round's ties.
pub(crate) unsafe fn cup_stage_loaded_regions(this: usize) -> Vec<(usize, usize)> {
    let cup = Cup(this);
    let rounds = cup.round_count().max(0) as usize;
    let mut regions = vec![(this, 0xb2usize), (cup.round_table(), rounds * cup_round::SIZE), (cup.team_table(), cup.team_count().max(0) as usize * 6)];
    for r in 0..rounds {
        let ties = read_i16(cup.round_table() + r * cup_round::SIZE + cup_round::TIES).max(0) as usize;
        regions.push((read_usize(cup.tie_tables() + 4 * r), ties * TIE_SIZE));
    }
    regions
}

/// The Cup_Stage vtable.
pub(crate) const CUP_STAGE_VTABLE: usize = 0x969468;

/// The size of a cup tie (the records of `cup::TIE_TABLES`).
pub(crate) const TIE_SIZE: usize = 0x35;

/// The fields of a cup stage in its file, in order: (field, size, the box line of a short read).
pub(crate) const CUP_STAGE_FILE_FIELDS: [(usize, u32, u32); 15] = [(cup::PRIZE_FUND, 4, 0xe9), (cup::IMPORTANCE_BIAS, 1, 0xef), (cup::SUBSTITUTES_NAMED, 1, 0xf5),
    (cup::SUBSTITUTIONS_ALLOWED, 1, 0xfb), (cup::RUN_BY_STAGES, 1, 0x101), (cup::ROUND_COUNT, 2, 0x107), (cup::TEAM_COUNT, 2, 0x10d), (cup::YEAR, 2, 0x113),
    (cup::CURRENT_ROUND, 1, 0x11a), (cup::NEXT_DRAW_ROUND, 1, 0x121), (cup::STAGE_INDEX, 1, 0x128), (cup::CLUB_SIDES, 1, 0x12f), (cup::STAGE_LABEL, 2, 0x136),
    (league::UPPER_DIVISION_ID, 4, 0x13d), (league::LOWER_DIVISION_ID, 4, 0x144)];
