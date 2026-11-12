// ---------- batch %%LEVA%%: the fixture tree of a cup stage: the ties of each round, the links to the next round and the entrants in the first ties (cup_stage.cpp) ----------

/// The cup_stage.cpp source path of the error boxes.
const CUP_STAGE_SOURCE: usize = 0x9b5944;

/// Links tie p of round `r` to a free side of a tie of round `target` (the search starts at tie `s`): the home side
/// (`cup_tie::HOME_LINK_TIE` free: the slot keeps p and r, the tie keeps the slot and `cup_tie::FEEDER_SIDE` 1) or else
/// the away side (`cup_tie::AWAY_LINK_TIE`, `cup_tie::FEEDER_SIDE` 0), moving to the next tie (back to 0 after the
/// round's last) while both are taken. Returns false, with nothing written, when it comes back to `s`.
unsafe fn link_tie_to_round(this: Cup, r: i32, p: i32, target: i32, s: i32) -> bool {
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

/// The first broken part of a built stage tree, or None: each tie of the rounds before m − 1 must be the one its
/// feeder slot points back to (`cup_tie::FEEDER_ROUND` r + 1 or r + 2, the slot's link tie and link round), and each
/// entrant of each round below m must be the home or away team of exactly one tie of its round.
unsafe fn cup_stage_tree_mismatch(this: Cup) -> Option<String> {
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
                return Some(format!("entrant {} of round {} placed {} times", first + q, rd, seen));
            }
        }
    }
    None
}

/// Comparer for `cup_stage_map_fixture_tree`: only the original runs (it allocates and draws); afterwards, on a season
/// set up and a return of 1, the plan (`cup_stage_tree_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_map_fixture_tree(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%)(this);
    let cup = Cup(this);
    if r != 1 || cup.season_set_up() == 0 {
        return r;
    }
    let error = cup_stage_tree_mismatch(cup);
    SCOREBOARD.record(error.is_none(), || format!("cup_stage_map_fixture_tree({:#x}): {}", this, error.clone().unwrap_or_default()));
    r
}
