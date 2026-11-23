use super::*;

/// Entrants k minus half (truncated) the preliminary-round count (+0xb2).
pub(crate) unsafe fn entrants_minus_half_prelim(this: Cup, k: i16) -> i32 {
    k as i32 - (read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT) as i32) / 2
}

/// `UefaCup::build_entrants()` (0x902b90, thiscall, ret 0, uefa_cup.cpp; called from the constructor, 2001-000): builds
/// and sorts the 118-entrant list and the Intertoto list; returns 1, 0 or −1.
///
/// +0x3e = 145; the list (calloc `exe::crt_calloc`(161, 6) at +0xa7; out of memory, the error box for line 0x1bf and −1); +0xb2
/// = 0.
///
/// 1) In club table order, the clubs the European rules put in the UEFA Cup (+0x1db = [0x9cf6f0]), each with class =
/// its coefficient (+0x1df) and status 0; those with coefficient 10 or more (signed) are counted in +0xb2 (the
/// preliminary-round ones [inference]).
///
/// 2) If the entrants minus half of +0xb2 do not reach 118, it fills up (among the first [0xae235c] − 2 × [0xae2350]
/// clubs, until reaching 118) with clubs with no European cup (+0x1db = −1) from a nation (+0x53) without the +0x11c
/// mark and with a nation coefficient (+0x8e, signed word) below 2000: each with class 3, status 0 and +0x1db = the cup
/// id ([+4] +0).
///
/// 3) If over 118, removes the last ones (the slot is zeroed) down to 118.
///
/// The list goes to qsort (`exe::crt_qsort`, 6 bytes, `cup_compare_entries_by_level_desc`: by class). With +0xb2 even (and not negative): +0x36 = 0, +0x38
/// = the entrants + 27, the list of those coming from the Intertoto [inference] (`exe::crt_malloc` of +0x38 × 6 at +0x14; out of
/// memory, the error box for line 0x21a and 0) with the class-1 entrants, each set to class 3 and counted in +0x36;
/// +0x3e = +0x38; returns 1. With +0xb2 odd or negative, 0.
pub(crate) unsafe fn uefa_cup_build_entrants(this: Cup) -> i32 {
    const F: usize = 0xaa19e0;
    this.set_team_count(0x91);
    let l = exe::crt_calloc(0xa1, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(F, 0x1bf);
        return -1;
    }
    write_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT, 0);
    let base = read_usize(G_CLUB_TABLE);
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    if (read_usize(G_CLUB_COUNT) as i32) > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            if read_usize(c + club::CONTINENTAL_COMP_ID) == read_usize(KEY_CLUB_COMP_UEFA_CUP) {
                let e = k as i32 as usize * 6;
                k = k.wrapping_add(1);
                write_usize(this.team_table() + e, c);
                write_u8(this.team_table() + e + 4, read_u8(c + club::EURO_SEEDING));
                write_u8(this.team_table() + e + 5, 0);
                if read_i8(c + club::EURO_SEEDING) >= 0xa { // cqs9: those in the preliminary
                    write_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT, read_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT).wrapping_add(1));
                }
            }
            i = i.wrapping_add(1);
            if i as i32 >= read_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    if entrants_minus_half_prelim(this, k) < 0x76 {
        let lim = |_: ()| (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
        let mut j: i16 = 0;
        if lim(()) > 0 {
            loop {
                if entrants_minus_half_prelim(this, k) >= 0x76 { // cqs9: fills up to 118
                    break;
                }
                let c = base + j as u16 as usize * club::SIZE;
                let nation = Nation(read_usize(c + club::NATION));
                if !nation.is_null() && nation.league_selected() == 0 && nation.reputation() < 0x7d0 && read_usize(c + club::CONTINENTAL_COMP_ID) as i32 == -1 {
                    covered!("cqs_9 club filling uefa");
                    let e = k as i32 as usize * 6;
                    k = k.wrapping_add(1);
                    write_usize(this.team_table() + e, c);
                    write_u8(this.team_table() + e + 4, 3); // cqs9: the class of those who complete it
                    write_u8(this.team_table() + e + 5, 0);
                    write_usize(c + club::CONTINENTAL_COMP_ID, read_usize(read_usize(this.addr() + 4)));
                }
                j = j.wrapping_add(1);
                if j as i32 >= lim(()) {
                    break;
                }
            }
        }
    }
    while entrants_minus_half_prelim(this, k) > 0x76 {
        write_usize(this.team_table() + k as i32 as usize * 6 - 6, 0);
        k = k.wrapping_sub(1);
    }
    exe::crt_qsort(this.team_table(), k as i32 as u32, 6, 0x522190);
    let b2 = read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT);
    if b2 < 0 || b2 % 2 != 0 {
        return 0;
    }
    covered!("cqs_9 uefa entrants");
    this.set_entrant_count(0);
    let n = k.wrapping_add(0x1b);
    this.set_entrant_capacity(n);
    let t = exe::crt_malloc((n as i32 * 6) as u32);
    this.set_entrant_book(t);
    if t == 0 {
        show_error_at(F, 0x21a);
        return 0;
    }
    if k > 0 {
        for q in 0..k as usize {
            let e = this.team_table() + q * 6;
            let c = read_usize(e);
            if read_u8(e + 4) == 1 && c != 0 {
                let m = this.entrant_count() as i32 as usize * 6;
                write_usize(this.entrant_book() + m, c);
                write_u8(this.entrant_book() + this.entrant_count() as i32 as usize * 6 + 4, 3);
                this.set_entrant_count(this.entrant_count().wrapping_add(1));
            }
        }
    }
    this.set_team_count(n);
    1
}

/// UefaCup::build_entrants hook entry: builds and sorts the 118-club UEFA Cup entrants list and the Intertoto list (see
/// uefa_cup_build_entrants); returns 1, 0 or -1.
pub(crate) unsafe extern "thiscall" fn build_entrants(this: usize) -> i32 {
    uefa_cup_build_entrants(Cup(this))
}

/// rule: the UEFA Cup has, only when there are preliminary clubs (`uefa_cup::PRELIMINARY_CLUB_COUNT`, those with
/// coefficient 10 or more [inference]), a preliminary round drawn on 28/6 and played on 6/7 (b teams); then: 1st
/// qualifying round drawn on 28/7 and played on 10/8 (82 teams, 0x52 − b/2 new), 2nd on 14/9 (96 teams, 55 new), 1st
/// round on 19/10 (48 teams), 2nd on 23/11 (32 teams, 8 new), 3rd on 1/3 of the following year, round of 16 on 15/3,
/// quarter-finals on 5/4 and the single-match final on 16/5 (venue 4, prize of 1.5 million to the winner); two legs 14
/// days apart (7 in the 3rd round and the round of 16); category 7 in qualifying and 8 after
/// Original: `UefaCup::create_round_table` (0x9025c0, uefa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn uefa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 9;
    *extra = 0;
    let t = exe::crt_malloc(0x3a8);
    if t == 0 {
        show_error_at(0xaa19e0, 0xe0);
        return 0;
    }
    covered!("cqq_10 uefa fixtures");
    let year = this.year() as i32 as u32;
    let (tab, n) = uefa_cup_rounds(read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT));
    for (i, r) in tab[..n].iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    *rounds = n as i16;
    t
}

/// rule: in the UEFA Cup, the winner, the runner-up and every knocked-out club leave European competition (the club
/// mark goes back to −1); the career records are the base cup ones (winner, runner-up, qualification and knock-out by
/// round)
/// Original: `UefaCup::update_staff_records_for_status` (0x9030e0, uefa_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn uefa_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let club = Club(club);
    let d = outcome as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            covered!("cqi_10 dropped into uefa");
        }
        write_i32(club.addr() + club::CONTINENTAL_COMP_ID, -1); // cqi10: whoever finishes in the UEFA leaves the European competition
    }
    exe::cup_update_staff_records_for_status(this, club, outcome, a3, a4, game, a6)
}

/// The season turnover of the UEFA Cup: last season's tables and entrant book freed, the season archived, the year moved
/// on by one, the UEFA seeding's new season, the entrants built, the round table and the tie tree rebuilt and the
/// predicted positions.
///
/// `european_cup_free_season` with the entrant book (`cup::ENTRANT_BOOK`), without the stages; then
/// `european_turnover_update_seeding`. Then, each with its own box (uefa_cup.cpp, lines 0x273, 0x27a and 0x281) when it
/// returns a negative value: the entrants (`exe::uefacup_build_entrants` UefaCup::build_entrants, thiscall, ret 0), the
/// round table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`). Finally
/// the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the caller does not read it).
/// Original: `UefaCup::start_new_season` (0x902ee0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_cup_start_new_season(this: usize) -> u32 {
    use european_turnover_consts as et;
    let this = Cup(this);
    european_cup_free_season(this, true, false);
    european_turnover_update_seeding();
    covered!("mwg_9 uefa cup rollover");
    if exe::uefacup_build_entrants(this) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x273);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x27a);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(et::UEFA_CUP_CPP, 0x281);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for `uefa_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_UEFA_CUP_START_NEW_SEASON);
    compare_cup_new_season("uefa_cup_start_new_season", Cup(this), || o(this))
}

/// Sets up the rounds of the UEFA Cup. Rule: the prize fund is 3,000,000; the round table comes from
/// `comp_vtable::CREATE_SCHEDULE`.
///
/// Ret 0; virtual +0x8c: `UefaCup::set_up_rounds()`; returns 1. The prize fund (`.set_prize_fund()`) of 3,000,000 and the
/// round table (`.set_round_table()`, the old one not freed) from `comp_vtable::CREATE_SCHEDULE` (push 0; push
/// &`cup::STAGE_LABEL`; push &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10; the original writes the
/// fund after pushing the arguments, before the call). The same routine as the CONCACAF Champions' Cup's with another
/// fund.
/// Original: `UefaCup::set_up_rounds` (0x9030b0, uefa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    this.set_prize_fund(UEFA_CUP_PRIZE_FUND); // mwg10: the UEFA Cup's prize fund
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
    covered!("mwg_10 uefa cup rounds");
    1
}

/// Comparer for `uefa_cup_set_up_rounds`.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_set_up_rounds(this: usize) -> u32 {
    let this = Cup(this);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_UEFA_CUP_SET_UP_ROUNDS)(this.addr());
    SCOREBOARD.record(r == 1 && this.prize_fund() == UEFA_CUP_PRIZE_FUND, || format!("uefa_cup_set_up_rounds({:#x}) returned {} prize fund {}", this.addr(), r, this.prize_fund()));
    r
}

/// Rule: the UEFA Cup's club list: of one round (kind 1: round count − 8, kind 2: − 7, kind 3: − 4) the two clubs of
/// each tie, in tie order; otherwise (or with no such round) every entrant of the team table.
///
/// `UefaCup::collect_club_list(&out, kind)` (virtual +0x58). The list (`exe::crt_malloc`(4 × `cup::TEAM_COUNT`): push;
/// add esp, 4 -> cdecl; out of memory: the error box at line 0x2d6, `out` = 0 and 0). Kind (byte) 1, 2 or 3: the round
/// r = `cup::ROUND_COUNT` − 8, 7 or 4 (words); if r > −1, for each of the round's ties (the word +0x1a of its record in
/// `cup::ROUND_TABLE`, `cup_round::SIZE` bytes; the tie k at [`cup::TIE_TABLES`][r] + 0x35 k) the home team (word +0)
/// and then the away team (word +2), when not −1, as clubs of the team table (`cup::TEAM_TABLE`, 6-byte rows), with no
/// bound check (as the original). Otherwise every non-null club of the team table (`cup::TEAM_COUNT`, re-read each
/// turn), while the count is below the table size. `out` = the list; returns the count in the low word (the high word
/// is `out`'s, as the original's eax).
/// Original: `UefaCup::collect_club_list` (0x903120, uefa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let cup = Cup(this);
    let n = cup.team_count();
    let v = exe::crt_malloc(((n as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0xaa19e0, 0x2d6);
        write_usize(out, 0);
        return 0; // simplification: the original returns the box's eax with ax = 0; the caller reads the word
    }
    let mut count = 0i16;
    let round = match kind as u8 {
        1 => Some(cup.round_count().wrapping_sub(8)),
        2 => Some(cup.round_count().wrapping_sub(7)),
        3 => Some(cup.round_count().wrapping_sub(4)),
        _ => None,
    };
    let push = |c: usize, count: &mut i16| {
        write_usize(v + 4 * *count as usize, c);
        *count = count.wrapping_add(1);
    };
    match round {
        Some(r) if r > -1 => {
            covered!("cqx_12 uefa cup round clubs");
            let r = r as usize;
            let mut k = 0i16;
            while k < read_i16(cup.round_table().wrapping_add(r * cup_round::SIZE) + 0x1a) {
                let tie = read_usize(cup.tie_tables() + 4 * r).wrapping_add(0x35 * k as usize);
                let home = read_i16(tie);
                if home != -1 {
                    push(read_usize(cup.team_table().wrapping_add(6 * home as isize as usize)), &mut count);
                }
                let away = read_i16(tie + 2);
                if away != -1 { // cqx12: the away club of the tie
                    push(read_usize(cup.team_table().wrapping_add(6 * away as isize as usize)), &mut count);
                }
                k += 1;
            }
        }
        _ => {
            let mut i = 0i16;
            while i < cup.team_count() {
                let c = read_usize(cup.team_table() + 6 * i as usize);
                if c != 0 && count < n {
                    push(c, &mut count);
                }
                i += 1;
            }
        }
    }
    write_usize(out, v);
    (out as u32 & 0xffff_0000) | count as u16 as u32
}

/// Comparer for `uefa_cup_collect_club_list`: side by side (it only allocates its own list): the original with the
/// caller's `out`, the copy with its own; the counts (the low word) and the clubs of the two lists must be equal; the
/// copy's list is freed.
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_UEFA_CUP_COLLECT_CLUB_LIST)(this, out, kind);
    let mut mine = 0usize;
    let c = uefa_cup_collect_club_list(this, &mut mine as *mut usize as usize, kind);
    let theirs = read_usize(out);
    let n = (r as u16 as i16).max(0) as usize;
    let same = r as u16 == c as u16 && (theirs == 0) == (mine == 0) && (theirs == 0 || (0..n).all(|k| read_usize(theirs + 4 * k) == read_usize(mine + 4 * k)));
    if mine != 0 {
        exe::crt_free(mine);
    }
    SCOREBOARD.record(same, || format!("uefa_cup_collect_club_list({:#x}, {}): original {} copy {}", this, kind as u8, r as u16, c as u16));
    r
}

/// The UEFA Cup constructor `UefaCup::UefaCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the year (`cup::YEAR`), the competition record at +4, the vtable (0x970790), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 2 (clubs of several
/// nations), +0x52 = 3, 7 substitutes named, 3 substitutions allowed, discipline rule 1 and `run_by_stages()` = 0. If
/// the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a
/// new cup, in order: the yearly European seeding (UefaSeeding::update_season 0x9035a0 on [`G_UEFA_SEEDING`], if any;
/// thiscall, ret 4); the entrants (UefaCup::build_entrants 0x902b90, thiscall, ret 0; the error box of line 0x54 if it
/// returns 0); the prize fund (`cup::PRIZE_FUND`) = 3,000,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into
/// `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x62 if it
/// returns 0); the statistics at +8 (the error box of line 0x69 without them); the final ranking (Cup::rank_teams
/// 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this. [the bench, substitutions, discipline and type
/// fields read from the competitions reference, §2.1]
/// Original: `UefaCup::UefaCup` (0x9022e0, uefa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_uefa_cup_body(this, year, record) // cqu9: the UEFA Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`uefa_cup_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_uefa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_UEFA_CUP)(this, year, record);
    let error = uefa_cup_ctor_plan_error(this, r, year, record, UEFA_CUP_VTABLE, &UEFA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_uefa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the UEFA Cup constructor (see `construct_uefa_cup`).
pub(crate) unsafe fn construct_uefa_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    cup.set_year(year as i16);
    write_usize(this + 4, record);
    write_usize(this, UEFA_CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(2);
    write_u8(this + 0x52, 3);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(1);
    cup.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqu_9 uefa cup new");
    // the yearly European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let seeding = read_usize(G_UEFA_SEEDING);
    if seeding != 0 {
        exe::uefaseeding_update_season(seeding, 0);
    }
    if exe::uefacup_build_entrants(this) == 0 {
        show_error_at(0xaa19e0, 0x54);
        return this;
    }
    cup.set_prize_fund(0x2dc6c0);
    uefa_cup_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) == 0 {
        show_error_at(0xaa19e0, 0x62);
        return this;
    }
    if !uefa_cup_ctor_create_stats(this) {
        show_error_at(0xaa19e0, 0x69);
        return this;
    }
    exe::cup_rank_teams(this);
    this
}

/// The UEFA Cup's prize fund (3,000,000).
pub(crate) const UEFA_CUP_PRIZE_FUND: i32 = 3_000_000;

pub(crate) const UEFA_CUP_VTABLE: usize = 0x970790;

/// The fixed fields the UEFA Cup constructor writes on a new cup (offset, width, value).
pub(crate) const UEFA_CUP_CTOR_FIELDS: [(usize, u8, u32); 11] = [(cup::PRIZE_FUND, 4, 0x2dc6c0), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
