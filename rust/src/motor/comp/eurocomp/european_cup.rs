use super::*;

/// The club's nation (+0x53).
pub(crate) unsafe fn club_nation(club: Club) -> usize {
    club.nation().addr()
}

/// Draws the Champions League first group phase: the 32 clubs sorted by pot, same-nation partners in opposite halves,
/// the draw of the 8 groups pot by pot, the swap of same-nation clubs within a group, the coefficient, the 1,800,000
/// prize and its news, the 8 stages, and the third qualifying round losers into the UEFA Cup. SEH frame only for the
/// stage's `new`: no destructor on the normal path. `EuropeanCup::draw_phase1_groups()`, called from the Champions
/// League's next-stage routine (`european_cup_advance_stage`, stage 0). 1) The list of the 32 (`exe::crt_calloc`(0x20,
/// 9): push 9; push 0x20; add esp, 8 -> cdecl; 9 bytes: +0 the club, +4 the same-nation partner, +8 the half: 1 groups
/// A..D, 2 groups E..H; out of memory, the error box at line 0x4cc): the first 16 of table `.entrant_book()` (6 bytes)
/// and the teams of table `.team_table()` (6 bytes) with +5 = 1 (the third qualifying round winners). Fewer than 32:
/// the error box at line 0x4ee, `.current_stage()` = −1, the list freed and `.run_by_stages()` = 0. 2) The list is
/// sorted by pot (qsort `exe::crt_qsort` with `european_cup_compare_clubs_by_seed`: the club's `club::EURO_SEEDING` and
/// the reputation) and each club without a partner gets the first following club without a partner from the same nation
/// (the two point at each other). 3) The groups (`exe::crt_calloc`(0x20, 4): 8 groups × 4 pots; out of memory, the
/// error box at line 0x515 and the game stops): for each pot of 8 (k = 0..3), 4 slots start free in each half, minus
/// those already marked; 8 times a club of the pot is drawn (`exe::utils_random`(8) + 8k, drawn again while the slot is
/// already taken); without a half, one is drawn (`exe::utils_random`(free A + free B) < free B: the second) and the
/// partner, if it is in the same pot, is put in the other half (with no slot there, it is not marked); the club enters
/// the first group of the half with the pot free. 4) In each half and pot, a club with another club of the same nation
/// in its group swaps places with the same-pot club of the first other group in the half without its nation. 5) Each
/// group: the 4 clubs earn 1 point of the nation coefficient (`exe::uefaseeding_add_nation_points` on `G_UEFA_SEEDING`:
/// push 1; push nation -> thiscall, ret 8), the 1,800,000 prize (`exe::financetable_get_club_finance` on
/// `G_FINANCE_TABLE` push club -> thiscall, ret 4; `exe::finance_add_prize_money` push amount; mov ecx, finances ->
/// thiscall, ret 4) and the prize news (`exe::comp_send_prize_money_news`: push −2; push 0; push 0x449; push −1; push
/// 0; push amount; push club; mov ecx, this -> thiscall, ret 0x1c); the round table from method
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee) and the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`: this, 4, the
/// group, 2, +4, the table, the rounds (word), 3, 1, 8, &{4,1,2,0}, &{2,0,0,0}, the year (word), g, the id (word), 0xf,
/// 2, 0, 8, −1, 0, 2; the original pushes the year with garbage in the high bytes of edx, which the constructor does
/// not read: the copy passes the clean year) into `.stages()`[g] (without a stage, the error box at line 0x5c9 and it
/// returns, without freeing the table or the groups); the round table freed, `.current_stage()` = g and the group news
/// (`exe::league_send_group_draw_news`). 6) The groups are freed; the teams of `.team_table()` with +5 = 2 (the third
/// qualifying round losers) go to the UEFA Cup (`[G_COMP_OBJECT_TABLE + 4 × [KEY_CLUB_COMP_UEFA_CUP]]`; without it, the
/// error box at line 0x5d8): into the last 24 slots of its `.team_table()` (from `.team_count()` − 24), with +4 and +5
/// cleared, into its `.entrant_book()` with status 5 (`.entrant_count()` goes up) and with the club's European
/// competition (`club::CONTINENTAL_COMP_ID`) = `KEY_CLUB_COMP_UEFA_CUP`; then the UEFA Cup's predicted positions (its
/// method `comp_vtable::SET_PREDICTED_POSITIONS`).
/// Original: `EuropeanCup::draw_phase1_groups` (0x585290, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn draw_phase1_groups(this: usize) {
    let this = Cup(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rules: [u8; 4] = [4, 1, 2, 0];
    let rules_b: [u8; 4] = [2, 0, 0, 0];
    let list = exe::crt_calloc(0x20, 9);
    if list == 0 {
        show_error_at(0x9c4900, 0x4cc);
        return;
    }
    let ent = |i: i32| list.wrapping_add((i * 9) as usize);
    let mut n = 0i16;
    for k in 0..16usize {
        let e = ent(n as i32);
        write_usize(e, read_usize(this.entrant_book() + k * 6));
        write_usize(e + 4, 0);
        n += 1;
    }
    let mut k = 0i32;
    while k < this.team_count() as i32 {
        let t = this.team_table() + k as usize * 6;
        if read_u8(t + 5) == 1 {
            let e = ent(n as i32);
            write_usize(e, read_usize(t));
            write_usize(e + 4, 0);
            n += 1;
        }
        k += 1;
    }
    if n < 0x20 {
        show_error_at(0x9c4900, 0x4ee);
        this.set_current_stage(usize::MAX);
        exe::crt_free(list);
        this.set_run_by_stages(0);
        return;
    }
    exe::crt_qsort(list, 0x20, 9, 0x586f60);
    for j in 0..0x20i32 {
        let e = ent(j);
        if read_usize(e + 4) != 0 || j + 1 >= 0x20 {
            continue;
        }
        for i in (j + 1)..0x20 {
            let o = ent(i);
            if read_usize(o + 4) == 0 && club_nation(Club(read_usize(e))) == club_nation(Club(read_usize(o))) {
                covered!("cqg_4 partner from same nation");
                write_usize(e + 4, read_usize(o)); // cqg4: the partner from the same nation
                write_usize(o + 4, read_usize(e));
                break;
            }
        }
    }
    let groups = exe::crt_calloc(0x20, 4);
    if groups == 0 {
        fatal_error_at(0x9c4900, 0x515);
    }
    let slot = |g: i32, pot: i32| groups + ((pot + g * 4) * 4) as usize;
    let mut pot = 0i32;
    let mut base = 0i32;
    while base < 0x20 {
        let (mut livres_b, mut livres_a) = (4i8, 4i8); // [esp+0x12] half 2, [esp+0x13] half 1
        for i in 0..8 {
            match read_u8(ent(base + i) + 8) {
                2 => livres_b -= 1,
                1 => livres_a -= 1,
                _ => {}
            }
        }
        for _ in 0..8 {
            let mut s = exe::utils_random(8).wrapping_add(base); // cqg4: the club drawn from the pot
            while read_usize(ent(s)) == 0 {
                s = exe::utils_random(8).wrapping_add(base);
            }
            covered!("cqg_4 club drawn");
            let e = ent(s);
            if read_u8(e + 8) == 0 {
                covered!("cqg_4 half drawn");
                let b = livres_b as i32;
                if exe::utils_random(livres_a as i32 + b) < b { // cqg4: a metade sorteada
                    write_u8(e + 8, 2);
                    livres_b -= 1;
                } else {
                    write_u8(e + 8, 1);
                    livres_a -= 1;
                }
                let par = read_usize(e + 4);
                if par != 0 {
                    if let Some(i) = (0..0x20).find(|&i| read_usize(ent(i)) == par) {
                        let same_pot = i / 8 == s / 8;
                        let mark = if same_pot {
                            let m = read_u8(e + 8);
                            if (m == 2 && livres_a == 0) || (m == 1 && livres_b == 0) {
                                false
                            } else {
                                if m == 2 {
                                    livres_a -= 1;
                                } else {
                                    livres_b -= 1;
                                }
                                true
                            }
                        } else {
                            true
                        };
                        if mark {
                            write_u8(ent(i) + 8, if read_u8(e + 8) != 2 { 2 } else { 1 });
                        }
                    }
                }
            }
            let start: i32 = if read_u8(e + 8) == 1 { 0 } else { 4 };
            for g in start..start + 4 {
                if read_usize(slot(g, pot)) == 0 {
                    write_usize(slot(g, pot), read_usize(e));
                    write_usize(e, 0);
                    break;
                }
            }
        }
        base += 8;
        pot += 1;
    }
    exe::crt_free(list);
    for metade in 0..2i32 {
        let (start, fim) = (metade * 4, metade * 4 + 4);
        for p in 0..4i32 {
            let mut g = start;
            while g < fim {
                let nation = club_nation(Club(read_usize(slot(g, p))));
                let repeated = (0..4).any(|q| q != p && nation == club_nation(Club(read_usize(slot(g, q)))));
                if repeated {
                    for g2 in start..fim {
                        if g2 == g {
                            continue;
                        }
                        if (0..4).all(|q| club_nation(Club(read_usize(slot(g, p)))) != club_nation(Club(read_usize(slot(g2, q))))) {
                            covered!("cqg_4 swap by nation");
                            let x = read_usize(slot(g, p));
                            write_usize(slot(g, p), read_usize(slot(g2, p)));
                            write_usize(slot(g2, p), x);
                            break;
                        }
                    }
                }
                g += 1;
            }
        }
    }
    for g in 0..8i32 {
        let row = slot(g, 0);
        for q in 0..4usize {
            let c = read_usize(row + q * 4);
            exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), club_nation(Club(c)), 1);
            let value = 1_800_000i32; // cqg4: the group stage prize
            covered!("cqg_4 group stage prize");
            exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), value);
            exe::comp_send_prize_money_news(this, c, value, 0, -1, 0x449, 0, -2);
        }
        let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let table = rounds_of_cup(this.addr(), g as u32, &mut rodadas, &mut id, 0);
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(
                p,
                this.addr(),
                4,
                row,
                2,
                read_usize(this.addr() + 4),
                table,
                rodadas as u16 as u32,
                3,
                1,
                8,
                rules.as_ptr(),
                rules_b.as_ptr(),
                this.year() as u16 as u32,
                g as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        write_usize(this.stages() + g as usize * 4, stage);
        if read_usize(this.stages() + g as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x5c9);
            return;
        }
        exe::crt_free(table);
        this.set_current_stage(g as usize);
        exe::league_send_group_draw_news(read_usize(this.stages() + g as usize * 4));
    }
    exe::crt_free(groups);
    let uefa = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_UEFA_CUP) * 4));
    if uefa.is_null() {
        show_error_at(0x9c4900, 0x5d8);
        return;
    }
    let mut target = ((uefa.team_count() as i32 - 0x18) * 6) as usize;
    let mut k = 0i32;
    while k < this.team_count() as i32 {
        let t = this.team_table() + k as usize * 6;
        if read_u8(t + 5) == 2 {
            let e = uefa.team_table().wrapping_add(target);
            target = target.wrapping_add(6);
            write_usize(e, read_usize(t));
            write_u8(e + 4, 0);
            write_u8(e + 5, 0);
            let n = uefa.entrant_count() as i32;
            write_usize(uefa.entrant_book() + (n * 6) as usize, read_usize(e));
            let n = uefa.entrant_count() as i32;
            write_u8(uefa.entrant_book() + (n * 6) as usize + 4, 5);
            uefa.set_entrant_count(((uefa.entrant_count() as u16).wrapping_add(1)) as i16);
            write_usize(read_usize(e) + club::CONTINENTAL_COMP_ID, read_usize(KEY_CLUB_COMP_UEFA_CUP));
        }
        k += 1;
    }
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(uefa.addr());
}

/// The 32 clubs the draw puts in groups: the first 16 entrants of [+0x14] plus table entries (+0xa7) with +5 = 1,
/// sorted.
pub(crate) unsafe fn group_draw_clubs(this: Cup) -> Vec<usize> {
    let mut v: Vec<usize> = (0..16usize).map(|k| read_usize(this.entrant_book() + k * 6)).collect();
    for k in 0..this.team_count().max(0) as usize {
        let t = this.team_table() + k * 6;
        if read_u8(t + 5) == 1 {
            v.push(read_usize(t));
        }
    }
    v.sort_unstable();
    v
}

/// the pot comparison: the club's European seed (`club::EURO_SEEDING`), and on a tie the higher reputation
/// (`club::REPUTATION`) first
/// Original: `european_cup_compare_clubs_by_seed` (0x586f60, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn european_cup_compare_clubs_by_seed(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    let r = read_u8(x + club::EURO_SEEDING) as i8 as i32 - read_u8(y + club::EURO_SEEDING) as i8 as i32;
    if r != 0 {
        return r;
    }
    read_i16(y + club::REPUTATION) as i32 - read_i16(x + club::REPUTATION) as i32
}

/// The positions that `european_cup_rank_teams` records, in the order of its calls to `set_club_final_position`: (index of the club in the sorted vector,
/// position).
///
/// `mata` is the knockout round table (0x68 bytes per round, +0x1a the tie count) with `nm` rounds; the preliminaries
/// come from [+0xa3] with +0x3c rounds.
///
/// 1) The knockout, from the final backwards: in the final, those of the first pass get position 1 (the counter goes up
/// with each one); in every round, each of the following ones gets the counter value from the start of the round (which
/// goes up with each one).
///
/// 2) The second group stage: 2 × (4 clubs with the counter; the counter goes up by 4). 3) The first group stage: 2 ×
/// (8 clubs; it goes up by 8).
///
/// 4) The preliminaries, from the last to the first: each club of the round gets the counter value from the start of
/// that round (it goes up with each one).
pub(crate) unsafe fn planned_final_positions(this: Cup, knockout: usize, nm: i16) -> (Vec<(i16, u8)>, usize) {
    let mut v = Vec::new();
    let mut i = 0i16;
    let mut count = 1u8;
    let mut r = nm.wrapping_sub(1);
    while r >= 0 {
        let n = read_i16(knockout + r as usize * cup_round::SIZE + cup_round::TIES);
        if r == nm.wrapping_sub(1) {
            let mut k = 0i16;
            while k < n {
                covered!("cqg_4 champion");
                v.push((i, 1u8)); // cqg4: the champion
                i += 1;
                count = count.wrapping_add(1);
                k += 1;
            }
        }
        let p = count;
        let mut k = 0i16;
        while k < read_i16(knockout + r as usize * cup_round::SIZE + cup_round::TIES) {
            v.push((i, p));
            i += 1;
            count = count.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    let n_knockout = v.len();
    for &(vezes, quantos) in &[(2, 4u8), (2, 8u8)] {
        for _ in 0..vezes {
            let p = count;
            count = count.wrapping_add(quantos);
            for _ in 0..quantos {
                v.push((i, p));
                i += 1;
            }
        }
    }
    let mut r = this.round_count().wrapping_sub(1);
    while r >= 0 {
        let p = count;
        let mut k = 0i16;
        while k < read_i16(this.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) {
            covered!("cqg_4 position in the preliminary");
            v.push((i, p)); // cqg4: the position in the preliminary
            i += 1;
            count = count.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    (v, n_knockout)
}

/// Sets the Champions League predicted positions: the entrants sorted by strength, each one's predicted position taken
/// from the knockout from the final back, the second and first group phases and the qualifying rounds. Method +0x5c of
/// the Champions League: `EuropeanCup::rank_teams()`. Without statistics (+8): the error box at line 0x81b. The
/// entrants of table `.entrant_book()` (6 bytes, `.entrant_count()` of them) go into an array (`exe::crt_malloc`; out
/// of memory, the error box at line 0x823), sorted by strength (`exe::comp_util_sort_clubs_by_strength`: push n; push
/// array; add esp, 8 -> cdecl); the knockout table from method `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push
/// &rounds; push 0xc; mov ecx -> thiscall, ret 0x10), freed afterwards; each entrant's position, in array order,
/// through `exe::comp_stats_set_club_final_position` (push position; push club; mov ecx, `[+8]` -> thiscall, ret 8; the
/// original pushes the stack dword with the position in the low byte and garbage in the others, and
/// `exe::comp_stats_set_club_final_position` writes only the byte: the copy passes the clean byte) following the plan
/// in `planned_final_positions`; the array freed.
/// Original: `EuropeanCup::rank_teams` (0x586900, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn european_cup_rank_teams(this: usize) {
    let this = Cup(this);
    if read_usize(this.addr() + 8) == 0 {
        show_error_at(0x9c4900, 0x81b);
        return;
    }
    let v = exe::crt_malloc(((this.entrant_count() as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0x9c4900, 0x823);
        return;
    }
    let mut i = 0i16;
    while i < this.entrant_count() {
        write_u32(v + i as usize * 4, read_usize(this.entrant_book() + i as usize * 6) as u32);
        i += 1;
    }
    exe::comp_util_sort_clubs_by_strength(v, this.entrant_count() as i32);
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut nm, mut id) = (0i16, 0i16);
    let knockout = rounds_of_cup(this.addr(), 0xc, &mut nm, &mut id, 0);
    // the knockout positions before freeing its table, the others after (like the original)
    let (plano, n_mata) = planned_final_positions(this, knockout, nm);
    for (j, &(k, p)) in plano.iter().enumerate() {
        if j == n_mata {
            exe::crt_free(knockout);
        }
        exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), read_usize(v + k as usize * 4), p as u32);
    }
    if n_mata == plano.len() {
        exe::crt_free(knockout);
    }
    exe::crt_free(v);
}

/// Copies a 7-byte group entry (club, same-nation count, first-stage group, winner/runner-up) as the original does.
pub(crate) unsafe fn copy_group_entry(para: usize, of: usize) {
    write_usize(para, read_usize(of));
    write_u16(para + 4, read_i16(of + 4) as u16);
    write_u8(para + 6, read_u8(of + 6));
}

/// Draws the Champions League second group phase: the 16 qualifiers sorted, the same-group pairs, each one's group
/// choice (shuffled order, fewest teams, no same first-phase group nor nation), the redraw up to 20 times, the
/// coefficient, the 1,200,000 prize and its news, stages 8 to 11 and the third places into the UEFA Cup. SEH frame only
/// for the stage's `new`: no destructor on the normal path. `EuropeanCup::draw_phase2_groups(attempt)`, called from the
/// Champions League's next-stage routine (`european_cup_advance_stage`, stage 8, attempt 0). 1) The 16 qualifiers
/// (`exe::crt_calloc`(0x10, 7); out of memory, the error box at line 0x61c): from each first-phase group b
/// (`.stages()`[b]) the first and second of the table (`exe::league_get_table_row` push 0 / push 1; mov ecx, stage ->
/// thiscall, ret 4), with group b and 0/1; +4 counts the clubs in the list from the same nation (itself included); the
/// list is sorted (qsort `exe::crt_qsort` with `european_cup_compare_phase2_draw_entries`) and each club's bookings are
/// cleared in the competition's governing body (`exe::rulingbody_reset_bookings_for_club`, as in the World Cup draw).
/// 2) The pairs from the same first-phase group (two cleared lists of 16; out of memory, the error boxes 0x649 and
/// 0x651): each club still in the list goes to slot k (winner) or k + 1 (runner-up) and the other club of its group to
/// the other one; k goes up by 2. 3) Each of the 16, in pair order: the order of the 4 groups is shuffled (5 swaps of
/// `exe::utils_random`(4) and `exe::utils_random`(4)); the group with fewest teams among those that have a free slot in
/// its half (winners in slots 0/1, runners-up in 2/3) and have no team from the same first-phase group nor from the
/// same nation receives the club. 4) If a club is left without a group: with attempt < 20, the lists are freed and the
/// draw is run again with attempt + 1 (the call nests inside this one); on the 20th, each leftover takes the first free
/// slot (the club stays in the pairs list). 5) Each group h (0..3): the 4 clubs earn 1 coefficient point, the 1,200,000
/// prize and the news (`exe::comp_send_prize_money_news` with round 0x44a); stage h + 8 (method
/// `comp_vtable::CREATE_SCHEDULE`, `exe::crt_operator_new`(0xee) and the constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`
/// with the same arguments as the first-phase draw, the list of the 4 clubs copied into a 0x40-byte array
/// (`exe::crt_malloc`); without a stage, the error box at line 0x715), `.current_stage()` = h + 8 and the group news.
/// The four lists are freed. 6) The first-phase third places (`exe::league_get_table_row` push 2) go to the UEFA Cup
/// (`[G_COMP_OBJECT_TABLE + 4 × [KEY_CLUB_COMP_UEFA_CUP]]`; without it, the error box at line 0x729): into the last 8
/// slots of its `.team_table()`, from back to front, into the `.entrant_book()` with status 5 and with the club's
/// European competition = the UEFA Cup; the fourth places (push 3) are left without a European competition (−1); then
/// the UEFA Cup's predicted positions (method `comp_vtable::SET_PREDICTED_POSITIONS`).
/// Original: `EuropeanCup::draw_phase2_groups` (0x585ab0, european_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn draw_phase2_groups(this: usize, attempt: u32) {
    let this = Cup(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rules: [u8; 4] = [4, 1, 2, 0];
    let rules_b: [u8; 4] = [2, 0, 0, 0];
    let w = exe::crt_calloc(0x10, 7);
    if w == 0 {
        show_error_at(0x9c4900, 0x61c);
        return;
    }
    let e7 = |l: usize, i: i32| l.wrapping_add((i * 7) as usize);
    for b in 0..8i32 {
        let stage = || read_usize(this.stages() + b as usize * 4);
        let e = e7(w, 2 * b);
        write_usize(e, read_usize(exe::league_get_table_row(stage(), (0) as i32)));
        write_u8(e + 6, 0);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
        let e = e7(w, 2 * b + 1);
        write_usize(e, read_usize(exe::league_get_table_row(stage(), (1) as i32)));
        write_u8(e + 6, 1);
        write_u8(e + 5, b as u8);
        write_u8(e + 4, 0);
    }
    for i in 0..16 {
        for j in 0..16 {
            if club_nation(Club(read_usize(e7(w, i)))) == club_nation(Club(read_usize(e7(w, j)))) {
                write_u8(e7(w, i) + 4, read_u8(e7(w, i) + 4).wrapping_add(1));
            }
        }
    }
    exe::crt_qsort(w, 0x10, 7, 0x5871e0);
    for i in 0..16 {
        let body = read_usize(read_usize(read_usize(G_DISCIPLINE)).wrapping_add((this.discipline_index() as i8 as i32 * 4) as usize));
        exe::rulingbody_reset_bookings_for_club(body, read_usize(e7(w, i)));
    }
    let r = exe::crt_calloc(0x10, 7);
    if r == 0 {
        show_error_at(0x9c4900, 0x649);
        return;
    }
    let g = exe::crt_calloc(0x10, 7);
    if g == 0 {
        show_error_at(0x9c4900, 0x651);
        return;
    }
    let mut k = 0i8;
    for i in 0..16i32 {
        let e = e7(w, i);
        if read_usize(e) == 0 {
            continue;
        }
        let c = k as i32;
        copy_group_entry(e7(r, if read_u8(e + 6) != 0 { c + 1 } else { c }), e);
        write_usize(e, 0);
        for a in 0..16i32 {
            if i != a && read_u8(e + 5) == read_u8(e7(w, a) + 5) {
                let o = e7(w, a);
                let target = if read_usize(e7(r, c)) != 0 { e7(r, c + 1) } else { e7(r, c) };
                covered!("cqg_6 pair from same group");
                copy_group_entry(target, o); // cqg6: the other one in the group
                k = k.wrapping_add(2);
                write_usize(o, 0);
                break;
            }
        }
    }
    let mut order = [0u8, 1, 2, 3];
    for i in 0..16i32 {
        let p = e7(r, i);
        let mut minus = 4i8;
        let mut best = 0xffu8;
        let s: i32 = if read_u8(p + 6) != 0 { 2 } else { 0 };
        for _ in 0..5 {
            let a = exe::utils_random(4) as i8 as usize;
            let b = exe::utils_random(4) as i8 as usize; // cqg6: the swap of the group order
            covered!("cqg_6 order swap");
            order.swap(a, b);
        }
        for q in 0..4usize {
            let gr = order[q] as i8 as i32;
            let mut n = 0i8;
            let mut conflito = false;
            for j in 0..4i32 {
                let v = e7(g, j + gr * 4);
                let club = read_usize(v);
                if club != 0 {
                    n += 1;
                    if read_u8(v + 5) == read_u8(p + 5) || club_nation(Club(club)) == club_nation(Club(read_usize(p))) { // cqg6: same group or nation
                        conflito = true;
                        break;
                    }
                }
            }
            let vaga = read_usize(e7(g, s + gr * 4)) == 0 || read_usize(e7(g, s + 1 + gr * 4)) == 0;
            if vaga && !conflito && n < minus {
                minus = n;
                best = order[q];
            }
        }
        if best != 0xff {
            covered!("cqg_6 group chosen");
            let base = best as i8 as i32 * 4;
            let v = if read_usize(e7(g, s + base)) == 0 { e7(g, s + base) } else { e7(g, s + 1 + base) };
            copy_group_entry(v, p);
            write_usize(p, 0);
        }
    }
    for i in 0..16i32 {
        let p = e7(r, i);
        if read_usize(p) == 0 {
            continue;
        }
        if (attempt as u8 as i8) < 0x14 {
            covered!("cqg_6 redraw");
            exe::crt_free(g);
            exe::crt_free(r);
            exe::crt_free(w);
            exe::europeancup_draw_phase2_groups(this, (attempt as u8).wrapping_add(1) as u32);
            return;
        }
        if let Some(v) = (0..16).map(|j| e7(g, j)).find(|&v| read_usize(v) == 0) {
            copy_group_entry(v, p);
        }
    }
    let t = exe::crt_malloc(0x40);
    for i in 0..16usize {
        write_usize(t + i * 4, read_usize(e7(g, i as i32)));
    }
    for h in 0..4i32 {
        for q in 0..4i32 {
            let c = read_usize(e7(g, h * 4 + q));
            exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), club_nation(Club(c)), 1);
            let value = 1_200_000i32; // cqg6: the second stage prize
            covered!("cqg_6 second stage prize");
            exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c), value);
            exe::comp_send_prize_money_news(this, c, value, 0, -1, 0x44a, 0, -2);
        }
        let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
        let (mut rodadas, mut id) = (0i16, 0i16);
        let table = rounds_of_cup(this.addr(), (h + 8) as u32, &mut rodadas, &mut id, 0);
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(
                p,
                this.addr(),
                4,
                t + h as usize * 16,
                2,
                read_usize(this.addr() + 4),
                table,
                rodadas as u16 as u32,
                3,
                1,
                8,
                rules.as_ptr(),
                rules_b.as_ptr(),
                this.year() as u16 as u32,
                (h + 8) as u32,
                id as u16 as u32,
                0xf,
                2,
                0,
                8,
                -1,
                0,
                2,
            )
        } else {
            0
        };
        let slot = this.stages() + (h + 8) as usize * 4;
        write_usize(slot, stage);
        if read_usize(this.stages() + (h + 8) as usize * 4) == 0 {
            show_error_at(0x9c4900, 0x715);
            return;
        }
        exe::crt_free(table);
        this.set_current_stage((h + 8) as usize);
        exe::league_send_group_draw_news(read_usize(this.stages() + (h + 8) as usize * 4));
    }
    exe::crt_free(w);
    exe::crt_free(g);
    exe::crt_free(r);
    exe::crt_free(t);
    let uefa = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_UEFA_CUP) * 4));
    if uefa.is_null() {
        show_error_at(0x9c4900, 0x729);
        return;
    }
    let mut target = ((uefa.team_count() as i32 - 1) * 6) as usize;
    for b in 0..8usize {
        let e = uefa.team_table().wrapping_add(target);
        target = target.wrapping_sub(6);
        let third = read_usize(exe::league_get_table_row(read_usize(this.stages() + b * 4), (2) as i32));
        write_u8(e + 4, 0);
        write_usize(e, third);
        write_u8(e + 5, 0);
        let n = uefa.entrant_count() as i32;
        write_usize(uefa.entrant_book() + (n * 6) as usize, read_usize(e));
        let n = uefa.entrant_count() as i32;
        write_u8(uefa.entrant_book() + (n * 6) as usize + 4, 5);
        uefa.set_entrant_count(((uefa.entrant_count() as u16).wrapping_add(1)) as i16);
        write_usize(read_usize(e) + club::CONTINENTAL_COMP_ID, read_usize(KEY_CLUB_COMP_UEFA_CUP));
        let quarto = Club(read_usize(exe::league_get_table_row(read_usize(this.stages() + b * 4), (3) as i32)));
        quarto.set_continental_comp_id((usize::MAX) as u32);
    }
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(uefa.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(uefa.addr());
}

/// The 16 clubs that go to the second stage (top two of each first-stage group), sorted.
pub(crate) unsafe fn second_stage_clubs(this: Cup) -> Vec<usize> {
    let mut v = Vec::with_capacity(16);
    for b in 0..8usize {
        let f = read_usize(this.stages() + b * 4);
        v.push(read_usize(exe::league_get_table_row(f, (0) as i32)));
        v.push(read_usize(exe::league_get_table_row(f, (1) as i32)));
    }
    v.sort_unstable();
    v
}

/// The comparison for the second-phase qualifiers: more same-nation clubs first, then the higher nation word
/// `nation::REPUTATION`, then the group winner before the runner-up. `european_cup_compare_phase2_draw_entries(a, b)`,
/// the qsort comparison of the qualifiers (`draw_phase2_groups`): b's same-nation count (+4) minus a's; on a tie: the
/// nation's word `nation::REPUTATION` (the club's `club::NATION`) of b minus a's; on a tie: a's +6 (the runner-up flag)
/// minus b's.
/// Original: `european_cup_compare_phase2_draw_entries` (0x5871e0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn european_cup_compare_phase2_draw_entries(a: usize, b: usize) -> i32 {
    let r = read_u8(b + 4) as i8 as i32 - read_u8(a + 4) as i8 as i32;
    if r != 0 {
        return r;
    }
    let r = read_i16(club_nation(Club(read_usize(b))) + nation::REPUTATION) as i32 - read_i16(club_nation(Club(read_usize(a))) + nation::REPUTATION) as i32;
    if r != 0 {
        return r;
    }
    read_u8(a + 6) as i8 as i32 - read_u8(b + 6) as i8 as i32
}

/// comp_util_scale_by_round (`comp_util_scale_by_round`): a value between low and high by round, 0..100.
pub(crate) unsafe fn scale_by_round(low: u32, high: u32, round: i16, rounds: u32) -> u32 {
    // simplification: the original pushes the round in a register with garbage in the high 16 bits; 0x4bf850 only reads the word.
    exe::comp_util_scale_by_round(low, high, round as u16 as u32, rounds) as u32 
}

/// Staff_Records add_qualified_for_round_record for the club.
pub(crate) unsafe fn european_cup_add_qualified_record(club: usize, comp: usize, stage: u32, round: u32, weight: u32) {
    exe::staff_records_add_qualified_for_round_record(eng_second_staff_records(), club, comp, stage, round, weight);
}

/// Staff_Records add_failed_to_qualify_record for the club.
pub(crate) unsafe fn european_cup_add_failed_to_qualify_record(club: usize, comp: usize, stage: u32, x: u32) {
    exe::staff_records_add_failed_to_qualify_record(eng_second_staff_records(), club, comp, stage, x);
}

/// Staff_Records add_knocked_out_record for the club.
pub(crate) unsafe fn european_cup_add_knocked_out_record(club: usize, comp: usize, stage: u32, round: u32, x: u32) {
    exe::staff_records_add_knocked_out_record(eng_second_staff_records(), club, comp, stage, round, x);
}

/// Removes the club from its European competition this season (CONTINENTAL_COMP_ID +0x1db = -1).
pub(crate) unsafe fn leave_euro_competition(club: Club) {
    write_i32(club.addr() + club::CONTINENTAL_COMP_ID, -1);
}

/// Updates the staff records by outcome and stage: in qualifying, the qualified (`european_cup_add_qualified_record`)
/// or knocked-out (`european_cup_add_knocked_out_record`) record weighted by round (`scale_by_round`); in the group
/// stages, qualified (6) or failed to qualify (0xfe, `european_cup_add_failed_to_qualify_record`); in the knockout,
/// winners (`exe::staff_records_add_cup_winners_record`), runners-up (`exe::staff_records_add_cup_runners_up_record`),
/// qualified or knocked out; whoever leaves gets `club::CONTINENTAL_COMP_ID` = -1 (`leave_euro_competition`); returns
/// 0. `EuropeanCup::update_staff_records_for_status(club, outcome, stage, _a, game, end)`, the Champions League's
/// virtual +0x44. Returns 0. Comp = +4. - In qualifying (stage 0xff): outcome 0 or 1, the qualification by the
/// fixture's round (`fixture::ROUND_NUMBER`): in round 0, with `.stage_label()` and the round code of the next round
/// (round 1: `cup_round::ROUND_CODE` + `cup_round::SIZE`, +0x6f of the round table `.round_table()`) and the weight
/// `scale_by_round`(1, 0x14, 0, 2); in any other, 0x41c, 0 and `scale_by_round`(1, 0x14, round, 2). Outcome 0xff or 2:
/// the elimination (`european_cup_add_knocked_out_record`) with the fixture's `fixture::ROUND_PREFIX` and
/// `fixture::ROUND_NAME` and the weight `scale_by_round`(1, 0x14, round, 2), and the club leaves
/// (`club::CONTINENTAL_COMP_ID` = −1). - In phase 1 (stage < 8): 6, the qualification (0x44a, −1, 0x32); 0xfe, the
/// non-qualification (0x449, 0x32) and the club leaves. - In phase 2 (8 to 11): 6, the qualification (0, 0x78, 0x32);
/// 0xfe, the non-qualification (0x44a, 0x32) and the club leaves. - In the knockout (12): 1, the title
/// (`exe::staff_records_add_cup_winners_record`, thiscall(records, club, game, end), ret 0xc) and the club leaves; 2,
/// runner-up (`exe::staff_records_add_cup_runners_up_record`, likewise) and the club leaves; 0, the qualification (0,
/// the round code of the round after the fixture's (`cup_round::ROUND_CODE` + `cup_round::SIZE` from the fixture's
/// round entry) in the knockout stage's round table (`.stages()` + 0x30, `.round_table()`, `cup_round::SIZE` bytes),
/// `scale_by_round`(0x4b, 0x64, round, 2)); 0xff, the elimination (0, the fixture's `fixture::ROUND_NAME`,
/// `scale_by_round`(0x4b, 0x64, round, 2)) and the club leaves.
/// Original: `EuropeanCup::update_staff_records_for_status` (0x584ae0, european_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn european_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a: u32, game: usize, end: u32) -> u32 {
    let game = Fixture(game);
    let this = Cup(this);
    let d = outcome as u8;
    let f = stage as u8;
    let comp = read_usize(this.addr() + 4);
    if f == 0xff {
        match d {
            0 | 1 => {
                let r = game.round_number();
                if r == 0 {
                    let weight = scale_by_round(1, 0x14, 0, 2);
                    let round = read_i16(this.round_table() + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                    // simplification: the stage +0x3a goes in a register with garbage in the high 16 bits; 0x86bdd0 only uses the word.
                    european_cup_add_qualified_record(club, comp, this.stage_label() as u16 as u32, round, weight);
                } else {
                    let weight = scale_by_round(1, 0x14, r, 2);
                    european_cup_add_qualified_record(club, comp, 0x41c, 0, weight);
                }
            }
            0xff | 2 => {
                let weight = scale_by_round(1, 0x14, game.round_number(), 2);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words.
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
                covered!("cpz_4 out in qualifying");
                leave_euro_competition(Club(club)); // cpz4: whoever is knocked out in qualifying leaves the European competition
            }
            _ => {}
        }
    } else if (f as i8) < 8 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0x44a, 0xffff_ffff, 0x32);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x449, 0x32);
            covered!("cpz_4 out in stage 1");
            leave_euro_competition(Club(club)); // cpz4: whoever is knocked out in stage 1 leaves the European competition
        }
    } else if (f as i8) < 0xc {
        if d == 6 {
            covered!("cpz_4 past stage 2");
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x32); // cpz4: passar da fase 2 pesa 0x32
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x44a, 0x32);
            leave_euro_competition(Club(club));
        }
    } else if f == 0xc {
        match d {
            1 => {
                exe::staff_records_add_cup_winners_record(eng_second_staff_records(), club, game, end);
                leave_euro_competition(Club(club));
            }
            2 => {
                exe::staff_records_add_cup_runners_up_record(eng_second_staff_records(), club, game, end);
                leave_euro_competition(Club(club));
            }
            0 => {
                let weight = scale_by_round(0x4b, 0x64, game.round_number(), 2);
                let knockout = Cup(read_usize(this.stages() + 0x30));
                let rounds = knockout.round_table();
                let r = game.round_number() as i32;
                // the upper bits of the register the original pushes are those of the stages pointer (+0xc)
                let round = (this.stages() as u32 & 0xffff_0000) | read_i16(rounds.wrapping_add((r as usize).wrapping_mul(cup_round::SIZE)) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                european_cup_add_qualified_record(club, comp, 0, round, weight);
            }
            0xff => {
                let weight = scale_by_round(0x4b, 0x64, game.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, 0, game.round_name() as u16 as u32, weight);
                leave_euro_competition(Club(club));
            }
            _ => {}
        }
    }
    0
}

// simplification: the records go to the file, touch the reputations and morale (they roll) and the champion and runner-up write the
// final squad: they do not run twice. Only the original runs; the plan checks what the routine itself writes (+0x1db of the
// club: −1 in the cases where it leaves, untouched in the others).

/// Draws four distinct group indices (rand 4 each until unused).
pub(crate) unsafe fn draw_four_groups() -> [i8; 4] {
    let a = random(4) as i8;
    let mut b = random(4) as i8;
    while b == a {
        b = random(4) as i8;
    }
    let mut c = random(4) as i8;
    while c == a || c == b {
        c = random(4) as i8;
    }
    let mut d = a;
    while d == a || d == b || d == c {
        d = random(4) as i8;
    }
    [a, b, c, d]
}

/// Club in row k of second-stage group g (League get_table_row).
pub(crate) unsafe fn second_stage_group_club(this: Cup, g: i8, k: u32) -> Club {
    Club(read_usize(exe::league_get_table_row(read_usize(this.stages() + 0x20 + 4 * (g as i32 as usize)), (k) as i32)))
}

/// Draws the order of the 4 phase 2 groups (`exe::utils_random`) and pairs the quarter-finals (1st of a v 2nd of b, 2nd
/// of a v 1st of b, same for c and d), resets the 8 clubs' bookings (`exe::rulingbody_reset_bookings_for_club`),
/// creates the knockout stage (`FN_CUP_STAGE_CUP_STAGE`) in slot `.stages()` + 0x30, drops the 3rd and 4th of each
/// group (`club::CONTINENTAL_COMP_ID` = -1, `leave_euro_competition`), sets up the round (`exe::cup_set_up_round`) and
/// sets `.current_stage()` = 12. `EuropeanCup::draw_quarter_finals()`, called from the next-stage routine
/// `european_cup_advance_stage` at stage 12. The list of the 8 (calloc `exe::crt_calloc` of 8 × 4; out of memory: the
/// error box at line 0x762) with the drawn groups a, b, c and d: 1st of a × 2nd of b, 2nd of a × 1st of b, 1st of c ×
/// 2nd of d and 2nd of c × 1st of d. Each one goes through `exe::rulingbody_reset_bookings_for_club` (thiscall(the
/// object `[[G_DISCIPLINE]]` + 4 × `.discipline_index()`, club), ret 4). The virtual `comp_vtable::CREATE_SCHEDULE`
/// (create_round_table: stage 12) and the knockout stage (operator new `exe::crt_operator_new` of 0xb2;
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE` with the cup, 8, the list, the rounds, +4, the table, `.year()`, 0xc,
/// 2, the extra, 0x14, 1, 0, 0, 0) into slot `.stages()` + 0x30; the list and the table freed (`exe::crt_free`).
/// Without a stage: the error box at line 0x78a. Otherwise the 3rd and 4th of each phase 2 group leave
/// (`club::CONTINENTAL_COMP_ID` = −1), the round is set up (Cup::set_up_round `exe::cup_set_up_round`, thiscall(stage,
/// 0)) and `.current_stage()` = 12.
/// Original: `EuropeanCup::draw_quarter_finals` (0x586330, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn draw_quarter_finals(this: usize) {
    let this = Cup(this);
    let list = exe::crt_calloc(8, 4);
    if list == 0 {
        return show_error_at(0x9c4900, 0x762);
    }
    covered!("cpz_4 quarter-final draw");
    let [a, b, c, d] = draw_four_groups();
    let order: [(i8, u32); 8] = [(a, 0), (b, 1), (a, 1), (b, 0), (c, 0), (d, 1), (c, 1), (d, 0)];
    for (k, &(g, l)) in order.iter().enumerate() {
        write_usize(list + 4 * k, (second_stage_group_club(this, g, l)).addr());
    }
    for k in 0..8 {
        let obj = read_usize(read_usize(read_usize(G_DISCIPLINE)) + 4 * (this.discipline_index() as i8 as i32 as usize));
        exe::rulingbody_reset_bookings_for_club(obj, read_usize(list + 4 * k));
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0xc, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the rounds, the extra and the word +0x40 go in registers with garbage in the high 16 bits; the constructor
    // only reads the words.
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 8, list, rodadas as u16 as u32, read_usize(this.addr() + 4), t, this.year() as u16 as u32, 0xc, 2, extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 0x30, stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(this.stages() + 0x30);
    if stage == 0 {
        return show_error_at(0x9c4900, 0x78a);
    }
    for g in 0..4i8 {
        leave_euro_competition(Club((second_stage_group_club(this, g, 2)).addr()));
        leave_euro_competition(Club((second_stage_group_club(this, g, 3)).addr()));
    }
    exe::cup_set_up_round(stage, 0);
    this.set_current_stage((0xc) as usize);
}

// simplification: the draw rolls, 0x7e9180 and the constructor allocate and the round creates the matches: original only. The plan,
// done before: the 8 group winners and runners-up and the 3rd and 4th of each; then +0x30 = 12, the knockout stage
// with 8 clubs that are the first and second places (as a set) and the 3rd and 4th of each group with +0x1db = −1.

/// Entrants (+0x14, 6 bytes each) still in the competition (+0x1db = the cup id), optionally excluding entry round 7;
/// writes them to v and returns the count.
pub(crate) unsafe fn active_entrants(this: Cup, without_7: bool, v: *mut usize) -> i16 {
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while i < this.entrant_capacity() {
        let e = this.entrant_book().wrapping_add((i as i32 as usize).wrapping_mul(6));
        let c = Club(read_usize(e));
        if (c.continental_comp_id() as i32) == read_i32(read_usize(this.addr() + 4)) && !(without_7 && read_u8(e + 4) == 7) {
            core::ptr::write_unaligned(v.add(n as i32 as usize), c.addr());
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    n
}

/// Allocates the list (0x11c bytes) of the entrants (`.entrant_book()`) still in the cup (`club::CONTINENTAL_COMP_ID` =
/// its id): kind 1 without the round 7 entries, kind 2 all, kind 3 at stage 11; otherwise the competition's own
/// (`exe::comp_collect_club_list`); returns the count. `EuropeanCup::collect_club_list(&out, kind)`, the Champions
/// League's virtual +0x58. Kind 1: the entrants still in, except those of round 7; kind 2: all those still in; kind 3
/// at stage 11 (`.current_stage()`): those still in; in any other case the competition's one (Comp::collect_club_list
/// `exe::comp_collect_club_list`, thiscall(this, &out, 0), ret 8). The list (malloc `exe::crt_malloc` of 0x11c bytes;
/// out of memory: the error box at line 0x7b5, 0x7cf or 0x7e9, null list and 0) goes into `out`; returns the count in
/// the low word (the high word is the list pointer's).
/// Original: `EuropeanCup::collect_club_list` (0x586660, european_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn european_cup_collect_club_list(this: usize, out: usize, kind: u32) -> u32 {
    let this = Cup(this);
    let t = kind as u8;
    let (linha, sem_7) = match t {
        1 => (0x7b5, true),
        2 => (0x7cf, false),
        3 if (this.current_stage() as i32) == 0xb => (0x7e9, false),
        _ => {
            return exe::comp_collect_club_list(this, out, 0) as u32;
        }
    };
    let v = exe::crt_malloc(0x11c);
    if v == 0 {
        show_error_at(0x9c4900, linha);
        write_usize(out, 0);
        return 0;
    }
    covered!("cpz_4 list of entrants");
    let n = active_entrants(this, sem_7, v as *mut usize);
    write_usize(out, v);
    (v as u32 & 0xffff_0000) | n as u16 as u32
}

/// CRT malloc(n).
pub(crate) unsafe fn european_cup_malloc(n: u32) -> usize {
    exe::crt_malloc(n)
}

/// Writes a 6-byte entrant record: club (+0), +5 = 0, entry round (+4), in the original's order.
pub(crate) unsafe fn european_cup_write_entrant(p: usize, club: usize, round: u8) {
    write_usize(p, club);
    write_u8(p + 5, 0);
    write_u8(p + 4, round);
}

/// Loop of `european_cup_collect_entrants` over the clubs ([0xae23b4], 0x245 bytes, [0xae235c] of them): clubs entered in the Champions League
/// go to entrant list `t` by band; returns the total count.
///
/// Clubs whose European competition (+0x1db) is the Champions League ([0x9cf6e8]) enter list `t` by band (+0x1df): 1 at
/// +0 (up to 16, round 7), 2 at +0x60 (up to 18, round 8), 3 at +0xcc (up to 19, round 0xd), 4 at +0x13e (up to 18,
/// round 0). Returns the sum of the four counts.
pub(crate) unsafe fn fill_champions_league_entrants(t: usize) -> i32 {
    let clubs = read_i32(G_CLUB_COUNT);
    let (mut n1, mut n2, mut n3, mut n4) = (0i32, 0i32, 0i32, 0i32);
    let (mut o1, mut o2, mut o3, mut o4) = (0usize, 0usize, 0usize, 0usize);
    if clubs <= 0 {
        return 0;
    }
    let cl = read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let mut c = read_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    loop {
        if read_i32(c + club::CONTINENTAL_COMP_ID) == cl {
            match read_u8(c + club::EURO_SEEDING) {
                1 => {
                    if o1 < 0x60 {
                        european_cup_write_entrant(t + o1, c, 7);
                        n1 += 1;
                        o1 += 6;
                    }
                }
                2 => {
                    if o2 < 0x6c {
                        covered!("cpy_5 band 2 entry");
                        european_cup_write_entrant(t + 0x60 + o2, c, 8); // cpy5: band 2 enters in round 8
                        n2 += 1;
                        o2 += 6;
                    }
                }
                3 => {
                    if o3 < 0x72 {
                        covered!("cpy_5 band 3 entry");
                        european_cup_write_entrant(t + 0xcc + o3, c, 0xd); // cpy5: band 3 enters in round 0xd
                        n3 += 1;
                        o3 += 6;
                    }
                }
                4 => {
                    if o4 < 0x6c {
                        covered!("cpy_5 band 4 entry");
                        european_cup_write_entrant(t + 0x13e + o4, c, 0); // cpy5: band 4 enters in round 0
                        n4 += 1;
                        o4 += 6;
                    }
                }
                _ => {}
            }
        }
        c = c.wrapping_add(club::SIZE);
        i += 1;
        if i as i16 as i32 >= clubs {
            break;
        }
    }
    n1 + n2 + n3 + n4
}

/// allocates the 71-entry list of 6-byte entries (`.entrant_book()`) and fills it with the Champions League clubs
/// (`club::CONTINENTAL_COMP_ID` = `KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`) by band (`club::EURO_SEEDING`): 1 up
/// to 16 (round 7), 2 up to 18 (8), 3 up to 19 (0xd), 4 up to 18 (0); short of 71, `.entrant_count()` = 0 and returns 0
/// Original: `EuropeanCup::collect_entrants` (0x584fa0, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn european_cup_collect_entrants(this: usize) -> u32 {
    let this = Cup(this);
    this.set_entrant_count(0x47);
    this.set_entrant_capacity(0x47);
    let old = this.entrant_book();
    if old != 0 {
        exe::crt_free(old);
    }
    let t = european_cup_malloc((this.entrant_count() as i32).wrapping_mul(6) as u32);
    this.set_entrant_book(t);
    if t == 0 {
        show_error_at(0x9c4900, 0x44e);
        return 0;
    }
    if fill_champions_league_entrants(t) != this.entrant_count() as i32 {
        this.set_entrant_count(0);
        return 0;
    }
    1
}

/// Loop of `build_qualifying_entry_table` over the 55 table rows: each row gets an entrant from the list (reverse order) and a code from the
/// entry round (8 -> 2, 0xd -> 1, else 0).
pub(crate) unsafe fn fill_qualifying_table(this: Cup, t: usize) {
    let mut i: i32 = 0;
    let mut k = 0usize;
    while i < this.team_count() as i32 {
        let n = this.entrant_count() as i32;
        let list = this.entrant_book();
        let club = if n != 0 { read_usize(list.wrapping_add((n - i - 1).wrapping_mul(6) as usize)) } else { 0 };
        write_usize(t + k, club);
        let list = this.entrant_book();
        let code = read_u8(list.wrapping_add(((n - i).wrapping_mul(6) - 2) as usize));
        write_u8(t + k + 4, match code {
            8 => 2,
            0xd => 1,
            _ => 0,
        });
        write_u8(t + k + 5, 0);
        i += 1;
        k += 6;
    }
}

/// allocates the 55 qualifying rows (`.team_table()`, `.team_count()` = 55): row i takes the club of entry 70 - i and
/// its entry round (8 -> 2, 0xd -> 1, other -> 0); out of memory, -1
/// Original: `EuropeanCup::build_qualifying_entry_table` (0x584990, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn build_qualifying_entry_table(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x37);
    let t = european_cup_malloc(0x14a);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9c4900, 0x2f6);
        return -1;
    }
    fill_qualifying_table(this, t);
    1
}

/// Moves to the next stage (`.current_stage()`) and draws the phase 1 groups (stage 0), the phase 2 groups (8) or the
/// quarter-finals (12); at the last stage, the error box. `EuropeanCup::advance_stage()` (the virtual +0x28, from the
/// cup pass 0x51f904). At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed): the error box at line
/// 0x495. Otherwise `.current_stage()` + 1 and, at stage 0, the phase 1 group draw (`draw_phase1_groups`,
/// thiscall(this)); at 8, the phase 2 draw (`draw_phase2_groups`, thiscall(this, 0), ret 4); at 12, the quarter-finals
/// (`draw_quarter_finals`, thiscall(this)). The caller does not read the return value.
/// Original: `EuropeanCup::advance_stage` (0x5851d0, european_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn european_cup_advance_stage(this: usize) {
    let this = Cup(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c4900, 0x495);
    }
    let f = f.wrapping_add(1);
    this.set_current_stage((f) as usize);
    covered!("cpy_5 next stage");
    match f {
        0 => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE1_GROUPS);
            sortear(this.addr());
        }
        8 => {
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_EUROPEANCUP_DRAW_PHASE2_GROUPS);
            sortear(this.addr(), 0);
        }
        0xc => {
            let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_EUROPEANCUP_DRAW_QUARTER_FINALS);
            sortear(this.addr());
        }
        _ => {}
    }
}

// simplification: the draws roll, allocate the stages and send news: original only; the copy takes the branch.

/// Club's finance record (FINANCETABLE get_club_finance on G_FINANCE_TABLE).
pub(crate) unsafe fn european_cup_club_finance(club: usize) -> usize {
    exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club)
}

/// Adds prize money to the finance record (balance, +0xa8 and +0x148).
pub(crate) unsafe fn european_cup_add_prize_money(fin: usize, value: i32) {
    exe::finance_add_prize_money(fin, value)
}

/// Adds coefficient points for the nation (UEFASEEDING add_nation_points).
pub(crate) unsafe fn add_uefa_coefficient(nation: usize, points: u32) {
    exe::uefaseeding_add_nation_points(read_usize(G_UEFA_SEEDING), nation, points);
}

/// The dry run of the comparer of `0x583b10`.
///
/// The copy runs without the prize news (`0x48e3e0`, which allocates) and without the cup base (`0x51a150`, which
/// processes the whole match and cannot run twice).
pub(crate) static EUROPEAN_CUP_FIXTURE_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Sends the prize money news (Comp `send_prize_money_news`); nothing in dry run.
pub(crate) unsafe fn european_cup_send_prize_money_news(this: usize, club: Club, value: u32, game: Fixture) {
    if EUROPEAN_CUP_FIXTURE_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::comp_send_prize_money_news(this, club, (value) as i32, (0) as i32, -1, 0, game, -2);
}

/// by the score (`.home_goals_extra_time()`/`.away_goals_extra_time()`, or `.home_goals()`/`.away_goals()`), 2 UEFA
/// coefficient points to the winner's nation or 1 each on a draw (`add_uefa_coefficient`); in the group stage
/// (`.stage_index()` from 0 to 11), 200 thousand to the winner or 100 thousand each, with the prize news item
/// (`european_cup_send_prize_money_news`); then the cup base (`exe::cup_process_fixture`)
/// Original: `EuropeanCup::process_fixture` (0x583b10, european_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn process_fixture(this: usize, game: usize, arg: u32) {
    let game = Fixture(game);
    let (ga, gb) = if (game.home_goals_extra_time() as u8) != 0xff {
        ((game.home_goals_extra_time() as u8) as i8, (game.away_goals_extra_time() as u8) as i8)
    } else {
        ((game.home_goals() as u8) as i8, (game.away_goals() as u8) as i8)
    };
    let (a, b) = (game.home_club(), game.away_club());
    let (vencedor, perdedor) = if ga > gb {
        add_uefa_coefficient(a.nation().addr(), 2);
        (game.home_club(), game.away_club())
    } else if ga < gb {
        add_uefa_coefficient(b.nation().addr(), 2);
        (game.away_club(), game.home_club())
    } else {
        add_uefa_coefficient(a.nation().addr(), 1);
        add_uefa_coefficient(game.away_club().nation().addr(), 1);
        (Club(0), Club(0))
    };
    let _ = perdedor;
    let stage = game.stage_index() as i8;
    if stage > -1 && stage < 0xc {
        if !vencedor.is_null() {
            covered!("cpy_6 win bonus");
            european_cup_add_prize_money(european_cup_club_finance(vencedor.addr()), 0x30d40); // cpy6: the win prize
            european_cup_send_prize_money_news(this, vencedor, 0x30d40, game);
        } else {
            covered!("cpy_6 draw bonus");
            european_cup_add_prize_money(european_cup_club_finance(game.home_club().addr()), 0x186a0); // cpy6: the home side’s draw prize
            european_cup_add_prize_money(european_cup_club_finance(game.away_club().addr()), 0x186a0);
            european_cup_send_prize_money_news(this, game.home_club(), 0x186a0, game);
            european_cup_send_prize_money_news(this, game.away_club(), 0x186a0, game);
        }
    }
    if EUROPEAN_CUP_FIXTURE_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::cup_process_fixture(this, game, arg);
}

/// Address of the club nation's UEFA coefficient word ([G_UEFA_SEEDING] + 18 x national-team +0x1df + 0xf); 0 if
/// missing.
pub(crate) unsafe fn nation_coefficient_word(club: Club) -> usize {
    let table = read_usize(G_UEFA_SEEDING);
    let nation = club.nation().addr();
    if table == 0 || nation == 0 {
        return 0;
    }
    let s = Club(exe::database_get_national_team(nation));
    if s.is_null() {
        return 0;
    }
    table.wrapping_add((s.euro_seeding() as i8 as isize * 18) as usize + uefa_seeding_nation::SEASON_POINTS)
}

/// Converts the three progress codes by stage (qualifying, phase 1, phase 2) and writes them to `.current_position()`,
/// `.best_position()` and `.worst_position()` of the club's competition stats entry.
/// `EuropeanCup::set_club_progress(club, stage, p1, p2, p3)` (the virtual +0x68, from the cup base
/// `record_team_progress`): converts the three progress codes by stage and writes them into the club's record in the
/// competition statistics (`exe::comp_stats_find_club_entry` on +8, without checking for null, as the original)
/// `.current_position()` ← p1, `.best_position()` ← p2, `.worst_position()` ← p3. Stage −1 (qualifying): 1 → 0x19 (p2:
/// 1), 17 → 0x21, 33 → 0x31, 47 → 0x3f; stage below 8 (signed): 1 and 2 → 1 (p3: 0xd), 3 → 0x11, 4 → 0x19; below 12: 1
/// and 2 → 1 (p3: 7), 3 → 9, 4 → 0xd; the other codes and the stages from 12 on pass through unchanged.
/// Original: `EuropeanCup::set_club_progress` (0x586c00, european_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn european_cup_set_club_progress(this: usize, club: usize, stage: u32, p1: u32, p2: u32, p3: u32) {
    let e = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(this + 8), club));
    let (stage, p1, p2, p3) = (stage as u8 as i8, p1 as u8, p2 as u8, p3 as u8);
    let (c73, c74, c75) = if stage == -1 {
        let m = |p: u8, um: u8| match p {
            1 => um,
            17 => 0x21,
            33 => 0x31,
            47 => 0x3f,
            _ => p,
        };
        (m(p1, 0x19), m(p2, 1), m(p3, 0x19))
    } else if stage < 8 {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 0x11,
            4 => 0x19,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 0xd))
    } else if stage < 0xc {
        let m = |p: u8, um: u8| match p {
            1 | 2 => um,
            3 => 9,
            4 => 0xd,
            _ => p,
        };
        (m(p1, 1), m(p2, 1), m(p3, 7))
    } else {
        (p1, p2, p3)
    };
    e.set_best_position(c74);
    e.set_current_position(c73);
    e.set_worst_position(c75);
}

/// Builds the round table: in the group stages, the 6-round schedule (phase 1 dates in September and October and phase
/// 2 from November to March, by half) with fixture 0 of each round empty; the qualifying rounds and the knockout stay
/// with the original. Method +0x3c of the Champions League, SEH frame for the local dates with no destructor on the
/// normal path: `EuropeanCup::create_round_table(stage, &rounds, &id, &league)`, called from the group draws
/// (`draw_phase1_groups`, `draw_phase2_groups`) and the predicted positions (`european_cup_rank_teams`). In the group
/// stages (char 0..11, and the negatives below −1, which fall into the same branch): league (if requested) = 1, rounds
/// = 6, id = 0x43d + stage; the table of 6 rounds of 0x41 bytes (`exe::crt_malloc`(0x186); out of memory, the error box
/// at line 0x17a or 0x1aa and it returns 0); in the second phase, beforehand, the date of 1 March of the following year
/// moved to the Tuesday (`exe::date_date_549f70`), one day earlier (`exe::date_sub_days`, which writes its static date)
/// and the copy (`exe::date_operator_3d`), unused afterwards (the copy calls them the same way); the dates of
/// `GROUP_STAGE_DATES` (`exe::league_round_set_date` with the cup's year `.year()`) and fixture 0 of each round empty
/// (`exe::league_round_set_match_slot` with −1). The qualifying rounds (−1) and the knockout (12 onwards): the original
/// through the trampoline. Returns the table.
/// Original: `EuropeanCup::create_round_table` (0x583c50, european_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn european_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let this = Cup(this);
    let f = stage as u8 as i8;
    if f == -1 || f >= 0xc {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EUROPEAN_CUP_CREATE_ROUND_TABLE)(this.addr(), stage, rounds, id, league);
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 6;
    *id = (f as i16).wrapping_add(0x43d);
    let t = exe::crt_malloc(0x186);
    if t == 0 {
        show_error_at(0x9c4900, if f < 8 { 0x17a } else { 0x1aa });
        return 0;
    }
    let year = this.year() as u16 as u32;
    let half = if f < 8 {
        if f < 4 { 0 } else { 1 }
    } else {
        let copy: unsafe extern "thiscall" fn(*mut [u16; 4], *mut [u16; 4], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let (mut d, mut t1, mut t2) = ([0u16; 4], [0u16; 4], [0u16; 4]);
        // simplification: the year + 1 goes in ax with the rest of eax in the high bytes; 0x549f70 reads the year as a word
        exe::date_date_549f70(&mut d, 1, 2, (year as u16).wrapping_add(1) as u32, 2);
        let r = exe::date_sub_days((&mut d) as *mut [u16; 4] as usize, (&mut t1) as *mut [u16; 4] as usize, 1);
        copy(&mut d, &mut t2, r);
        if f < 10 { 2 } else { 3 }
    };
    covered!("cqh_4 round fixtures");
    for &(i, dia, mes, anos, semana) in GROUP_STAGE_DATES[half].iter() {
        exe::league_round_set_date(t, i, dia, mes, anos, semana, 2, year, 0);
    }
    for r in 0..6 {
        exe::league_round_set_match_slot(t, r, 0, -1, -1, -1, 0);
    }
    t
}

